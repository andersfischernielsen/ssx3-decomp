//! `src/fe/debugfe.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::ai::rider::INPUT_BINDING_EVAL_BOOL;
pub(crate) use crate::game::be::beintnetwork::ldl;
pub(crate) use crate::game::be::beintnetwork::sdl;
pub(crate) use crate::game::bx::execman::oc821_feui_eval;
pub(crate) use crate::game::bx::execman::oc822_feui_eval;
pub(crate) use crate::game::bx::execman::oc823_feui_eval;
pub(crate) use crate::game::bx::execman::oc824_feui_eval;
use crate::game::dirtysock::tagsunk::gp_8;
use crate::game::dirtysock::tagsunk::LOBBY_INSTANCE_GP;
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
        0x0017_d0a8,
        oc817_debug_menu_start_multiplayer,
        "DebugMenu_StartMultiplayer",
    ),
    (0x0017_d298, lobby_menu_ctor, "LobbyMenu_ctor"),
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
    (0x0017_c5f8, debug_menu_render, "DebugMenu_Render"),
    (
        0x0017_d1b8,
        oc817_debug_menu_dispatch_item_action,
        "DebugMenu_DispatchItemAction",
    ),
    (0x0017_d3b0, lobby_menu_dtor, "LobbyMenu_dtor"),
    (0x0017_da30, lobby_menu_show_error, "LobbyMenu_ShowError"),
    (
        0x0017_db18,
        lobby_menu_update_state,
        "LobbyMenu_UpdateState",
    ),
    (
        0x0017_de70,
        lobby_menu_start_net_race,
        "LobbyMenu_StartNetRace",
    ),
    (0x0017_e5c8, lobby_menu_render, "LobbyMenu_Render"),
    (
        0x0017_f500,
        debug_menu_module_draw_cursor_trail,
        "DebugMenuModule_DrawCursorTrail",
    ),
    (0x0017_f7b8, debug_menu_module_ctor, "DebugMenuModule_ctor"),
    (0x0017_f838, debug_menu_module_dtor, "DebugMenuModule_dtor"),
    (0x0017_f868, debug_menu_module_init, "DebugMenuModule_Init"),
    (
        0x0017_fab0,
        debug_menu_module_try_exit,
        "DebugMenuModule_TryExit",
    ),
    (0x0017_fe80, feui_input_is_start, "FEUIInput_IsStart"),
    (0x0017_fea0, feui_input_is_select, "FEUIInput_IsSelect"),
    (0x0017_fee0, feui_input_is_previous, "FEUIInput_IsPrevious"),
    (0x0017_ff00, feui_input_is_option, "FEUIInput_IsOption"),
    (0x0017_ff20, feui_input_is_misc, "FEUIInput_IsMisc"),
    (0x0017_ff40, feui_input_is_next_held, "FEUIInput_IsNextHeld"),
    (
        0x0017_ff60,
        feui_input_is_previous_held,
        "FEUIInput_IsPreviousHeld",
    ),
    (
        0x0017_ff80,
        feui_input_is_option_held,
        "FEUIInput_IsOptionHeld",
    ),
    (0x0017_ffa0, feui_input_is_misc_held, "FEUIInput_IsMiscHeld"),
    (
        0x0017_ffc0,
        feui_input_is_next_released,
        "FEUIInput_IsNextReleased",
    ),
    (
        0x0017_ffe0,
        feui_input_is_previous_released,
        "FEUIInput_IsPreviousReleased",
    ),
    (
        0x0018_0000,
        feui_input_is_option_released,
        "FEUIInput_IsOptionReleased",
    ),
    (
        0x0018_0020,
        feui_input_is_misc_released,
        "FEUIInput_IsMiscReleased",
    ),
    (0x0018_0080, oc823_feui_is_left, "FEUIInput_IsLeft"),
    (0x0018_00a0, oc823_feui_is_right, "FEUIInput_IsRight"),
    (0x0018_00c0, oc823_feui_is_up_held, "FEUIInput_IsUpHeld"),
    (0x0018_00e0, oc823_feui_is_down_held, "FEUIInput_IsDownHeld"),
    (0x0018_0100, oc823_feui_is_left_held, "FEUIInput_IsLeftHeld"),
    (
        0x0018_0120,
        oc823_feui_is_right_held,
        "FEUIInput_IsRightHeld",
    ),
    (0x0018_0140, feui_input_is_r_shift1, "FEUIInput_IsRShift1"),
    (0x0018_0160, feui_input_is_r_shift2, "FEUIInput_IsRShift2"),
    (0x0018_0180, feui_input_is_l_shift1, "FEUIInput_IsLShift1"),
    (0x0018_01a0, feui_input_is_l_shift2, "FEUIInput_IsLShift2"),
    (
        0x0018_01c0,
        feui_input_is_r_shift1_held,
        "FEUIInput_IsRShift1Held",
    ),
    (
        0x0018_01e0,
        feui_input_is_r_shift2_held,
        "FEUIInput_IsRShift2Held",
    ),
    (
        0x0018_0200,
        feui_input_is_l_shift1_held,
        "FEUIInput_IsLShift1Held",
    ),
    (
        0x0018_0220,
        feui_input_is_l_shift2_held,
        "FEUIInput_IsLShift2Held",
    ),
    (0x0018_02a0, feui_input_get_cam_zoom, "FEUIInput_GetCamZoom"),
    (0x0018_02c0, feui_input_get_cam_pan, "FEUIInput_GetCamPan"),
    (
        0x0018_02e0,
        feui_input_get_cam_rotate,
        "FEUIInput_GetCamRotate",
    ),
    (
        0x0018_07d8,
        keyboard_screen_build_masked_text,
        "KeyboardScreen_BuildMaskedText",
    ),
    (
        0x0018_0840,
        keyboard_screen_type_key,
        "KeyboardScreen_TypeKey",
    ),
    (
        0x0017_d6f0,
        lobby_menu_on_lobby_event,
        "LobbyMenu_OnLobbyEvent",
    ),
    (
        0x001d_a540,
        lobby_menu_debug_printf_stub,
        "LobbyMenu_DebugPrintfStub",
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

    pub(crate) use crate::game::dirtysock::tagsunk::callee::C_BX_STRING_ASSIGN;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_BX_STRING_ASSIGN_CSTR;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_GET_ROOM_COUNT;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_GET_ROOM_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_IS_IN_ROOM;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_LOGIN;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_MOVE_TO_ROOM_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SELECT_PERSONA_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_DECLINE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MENU_SHOW_ERROR;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MENU_START_NET_RACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_POP_PAGE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_PUSH_PAGE;
    pub(crate) use crate::game::lib::libc::callee::STRCMP;
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

/// `void DebugMenu_Render(cDebugMenuScreen *self)` (0x17c5f8): draws the debug
/// front-end menu: the background image (unless the active image is negative),
/// then rows of text with the current row highlighted, the event, path, mode,
/// location, rider and controller names, audio and network info. A converted
/// labelled-block body: every register, cycle and call kept. It has no resume
/// entries, so no `__entries` fallback.
pub fn debug_menu_render(c: &mut Cpu) {
    guest(c, |c| {
        'b_17cfcc: {
            'b_17c65c: {
                c.cycles += 21;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x1d0));
                c.g[V0][0] = 0x8c10;
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.g[S6][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x190));
                c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x180));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x170));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x160));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x150));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x110), c.g[RA][0]);
                c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x1c8));
                c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x1b8));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                let t = (c.g[V1][0] as i64) < 0;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if t {
                    break 'b_17c65c;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0017c654);
                    Callee {
                        addr: 0x0017ae00,
                        f: ee::s02::DebugMenu_DrawBackground,
                    }
                    .call(c, 0x0017c654)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                    if t {
                        break 'b_17cfcc;
                    } else {
                        break 'b_17c65c;
                    }
                }
            }
            'b_17c95c: {
                'b_17c920: {
                    c.cycles += 158;
                    c.g[S2][0] = 0x4a0000;
                    c.g[AT][0] = 0x42700000;
                    c.mtc1(1, 20);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x12c0));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x80));
                    c.g[AT][0] = 0x41a00000;
                    c.mtc1(1, 22);
                    c.g[RA][0] = sx(0x0017c680);
                    c.g[S4][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0014dc80, 0x0017c680)?;
                    c.g[AT][0] = 0x44110000;
                    c.mtc1(1, 21);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0017c694);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x0017c694)?;
                    c.g[RA][0] = sx(0x0017c69c);
                    c.g[S5][0] = c.g[V0][0];
                    call_ptr(c, 0x0014dc80, 0x0017c69c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0017c6a8);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x0014dd58, 0x0017c6a8)?;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x104), c.g[V0][0]);
                    c.g[A1][0] = 0x1;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[RA][0] = sx(0x0017c6bc);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                    call_ptr(c, 0x0014dd58, 0x0017c6bc)?;
                    c.lwc1(4, (c.g[S4][0] as u32).wrapping_add(0x34));
                    c.lwc1(3, (c.g[S4][0] as u32).wrapping_add(0x14));
                    c.cvt_s_w(3, 3);
                    c.lwc1(5, (c.g[S4][0] as u32).wrapping_add(0x30));
                    c.g[S1][0] = 0x8000;
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5b1c));
                    c.g[S1][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 1);
                    c.g[FP][0] = c.g[V0][0];
                    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5b18));
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.swc1(5, (c.g[S4][0] as u32).wrapping_add(0x38));
                    c.g[V1][0] = c.g[SP][0];
                    c.swc1(4, (c.g[S4][0] as u32).wrapping_add(0x3c));
                    c.g[A1][0] = 0x460000;
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x1c));
                    c.mul_s(23, 3, 4);
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.mov_s(13, 22);
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x2c));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3140));
                    c.swc1(1, c.g[SP][0] as u32);
                    c.mov_s(12, 20);
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0u64);
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x24));
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc1c));
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.g[S0][0];
                    }
                    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, c.g[V1][0] as u32);
                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sdl(2, (c.g[S4][0] as u32).wrapping_add(0x47));
                    c.sdr(2, (c.g[S4][0] as u32).wrapping_add(0x40));
                    c.sdl(6, (c.g[S4][0] as u32).wrapping_add(0x4f));
                    c.sdr(6, (c.g[S4][0] as u32).wrapping_add(0x48));
                    c.g[RA][0] = sx(0x0017c76c);
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x00391cb0, 0x0017c76c)?;
                    c.g[RA][0] = sx(0x0017c774);
                    c.g[A0][0] = c.g[S5][0];
                    call_ptr(c, 0x00145378, 0x0017c774)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.mov_s(13, 22);
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x0017c788);
                    c.mov_s(12, 21);
                    call_ptr(c, 0x00392a60, 0x0017c788)?;
                    c.add_s(22, 23, 22);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc1c));
                    c.g[V1][0] = c.g[SP][0];
                    c.g[A1][0] = 0x460000;
                    c.mov_s(12, 20);
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    c.mov_s(13, 22);
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.g[S0][0];
                    }
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3130));
                    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, c.g[V1][0] as u32);
                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sdl(2, (c.g[S4][0] as u32).wrapping_add(0x47));
                    c.sdr(2, (c.g[S4][0] as u32).wrapping_add(0x40));
                    c.sdl(6, (c.g[S4][0] as u32).wrapping_add(0x4f));
                    c.sdr(6, (c.g[S4][0] as u32).wrapping_add(0x48));
                    c.g[RA][0] = sx(0x0017c7d4);
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x00391cb0, 0x0017c7d4)?;
                    c.g[RA][0] = sx(0x0017c7dc);
                    c.g[A0][0] = c.g[S5][0];
                    call_ptr(c, 0x00145398, 0x0017c7dc)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.mov_s(13, 22);
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x0017c7f0);
                    c.mov_s(12, 21);
                    call_ptr(c, 0x00392a60, 0x0017c7f0)?;
                    c.add_s(22, 22, 23);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc1c));
                    c.g[V1][0] = c.g[SP][0];
                    c.g[A1][0] = 0x460000;
                    c.mov_s(12, 20);
                    c.g[V0][0] = c.g[V0][0] ^ 0x2;
                    c.mov_s(13, 22);
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.g[S0][0];
                    }
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3120));
                    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, c.g[V1][0] as u32);
                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sdl(2, (c.g[S4][0] as u32).wrapping_add(0x47));
                    c.sdr(2, (c.g[S4][0] as u32).wrapping_add(0x40));
                    c.sdl(6, (c.g[S4][0] as u32).wrapping_add(0x4f));
                    c.sdr(6, (c.g[S4][0] as u32).wrapping_add(0x48));
                    c.g[RA][0] = sx(0x0017c83c);
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x00391cb0, 0x0017c83c)?;
                    c.g[RA][0] = sx(0x0017c844);
                    c.g[A0][0] = c.g[S5][0];
                    call_ptr(c, 0x00145340, 0x0017c844)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.mov_s(13, 22);
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x0017c858);
                    c.mov_s(12, 21);
                    call_ptr(c, 0x00392a60, 0x0017c858)?;
                    c.add_s(22, 22, 23);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc1c));
                    c.g[A1][0] = 0x460000;
                    c.mov_s(12, 20);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3110));
                    c.g[V0][0] = c.g[V0][0] ^ 0x3;
                    c.mov_s(13, 22);
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = c.g[SP][0];
                    }
                    c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x7));
                    c.ldr(2, c.g[S0][0] as u32);
                    c.ldl(3, (c.g[S0][0] as u32).wrapping_add(0xf));
                    c.ldr(3, (c.g[S0][0] as u32).wrapping_add(0x8));
                    c.sdl(2, (c.g[S4][0] as u32).wrapping_add(0x47));
                    c.sdr(2, (c.g[S4][0] as u32).wrapping_add(0x40));
                    c.sdl(3, (c.g[S4][0] as u32).wrapping_add(0x4f));
                    c.sdr(3, (c.g[S4][0] as u32).wrapping_add(0x48));
                    c.g[RA][0] = sx(0x0017c8a0);
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x00391cb0, 0x0017c8a0)?;
                    c.g[A0][0] = c.g[FP][0];
                    c.g[RA][0] = sx(0x0017c8ac);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x00145c38, 0x0017c8ac)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.mov_s(13, 22);
                    c.mov_s(12, 21);
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x0017c8c8);
                    c.add_s(22, 22, 23);
                    call_ptr(c, 0x001dca80, 0x0017c8c8)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f38));
                    let t = c.g[V0][0] == 0u64;
                    c.g[S0][0] = 0x460000;
                    if t {
                        break 'b_17c920;
                    } else {
                        c.cycles += 19;
                        c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x30f8));
                        c.g[RA][0] = sx(0x0017c8e4);
                        c.g[A0][0] = c.g[S5][0];
                        call_ptr(c, 0x00144bc0, 0x0017c8e4)?;
                        c.g[S7][0] = c.g[S3][0];
                        c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                        c.g[RA][0] = sx(0x0017c8f4);
                        c.g[A0][0] = c.g[S5][0];
                        call_ptr(c, 0x00144bc0, 0x0017c8f4)?;
                        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x44));
                        c.g[RA][0] = sx(0x0017c900);
                        c.g[A0][0] = c.g[S5][0];
                        call_ptr(c, 0x00144bc0, 0x0017c900)?;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A2][0] = c.g[S2][0];
                        c.g[A3][0] = c.g[S1][0];
                        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x34));
                        c.g[RA][0] = sx(0x0017c918);
                        c.g[A0][0] = c.g[S3][0];
                        call_ptr(c, 0x00417828, 0x0017c918)?;
                        let t = true;
                        c.g[S0][0] = 0x8000;
                        if t {
                            break 'b_17c95c;
                        } else {
                            break 'b_17c920;
                        }
                    }
                }
                c.cycles += 15;
                c.g[S0][0] = 0x4a0000;
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x12c8));
                c.g[RA][0] = sx(0x0017c934);
                c.g[A0][0] = c.g[S5][0];
                call_ptr(c, 0x00144bc0, 0x0017c934)?;
                c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x34));
                c.g[RA][0] = sx(0x0017c940);
                c.g[A0][0] = c.g[S5][0];
                call_ptr(c, 0x00144bc0, 0x0017c940)?;
                c.g[S7][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x34));
                c.g[RA][0] = sx(0x0017c958);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x00417828, 0x0017c958)?;
                c.g[S0][0] = 0x8000;
                break 'b_17c95c;
            }
            'b_17cc74: {
                'b_17cc6c: {
                    c.cycles += 194;
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[AT][0] = 0x42700000;
                    c.mtc1(1, 20);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc1c));
                    c.g[A1][0] = 0x4a0000;
                    c.g[AT][0] = 0x44110000;
                    c.mtc1(1, 21);
                    c.mov_s(13, 22);
                    c.g[V0][0] = c.g[V0][0] ^ 0x4;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x12d0));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = c.g[SP][0];
                    }
                    c.g[A0][0] = c.g[S4][0];
                    c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(8, c.g[V1][0] as u32);
                    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(2, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sdl(8, (c.g[S4][0] as u32).wrapping_add(0x47));
                    c.sdr(8, (c.g[S4][0] as u32).wrapping_add(0x40));
                    c.sdl(2, (c.g[S4][0] as u32).wrapping_add(0x4f));
                    c.sdr(2, (c.g[S4][0] as u32).wrapping_add(0x48));
                    c.mov_s(12, 20);
                    c.g[T0][0] = 0x4a0000;
                    c.g[S1][0] = 0x4a0000;
                    c.g[S2][0] = sx((c.g[T0][0] as u32).wrapping_add(0x12c0));
                    c.g[RA][0] = sx(0x0017c9c8);
                    c.g[S3][0] = c.g[S1][0];
                    call_ptr(c, 0x00391cb0, 0x0017c9c8)?;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x12f0));
                    c.g[A1][0] = c.g[S7][0];
                    c.mov_s(13, 22);
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x0017c9e0);
                    c.mov_s(12, 21);
                    call_ptr(c, 0x00392a60, 0x0017c9e0)?;
                    c.add_s(22, 22, 23);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc1c));
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = 0x460000;
                    c.mov_s(12, 20);
                    c.g[V0][0] = c.g[V0][0] ^ 0x5;
                    c.mov_s(13, 22);
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = c.g[SP][0];
                    }
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x30e8));
                    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, c.g[V1][0] as u32);
                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sdl(2, (c.g[S4][0] as u32).wrapping_add(0x47));
                    c.sdr(2, (c.g[S4][0] as u32).wrapping_add(0x40));
                    c.sdl(6, (c.g[S4][0] as u32).wrapping_add(0x4f));
                    c.sdr(6, (c.g[S4][0] as u32).wrapping_add(0x48));
                    c.g[RA][0] = sx(0x0017ca2c);
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x00391cb0, 0x0017ca2c)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                    c.g[RA][0] = sx(0x0017ca38);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014ee28, 0x0017ca38)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.mov_s(13, 22);
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x0017ca4c);
                    c.mov_s(12, 21);
                    call_ptr(c, 0x00392a60, 0x0017ca4c)?;
                    c.add_s(22, 22, 23);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc1c));
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = 0x460000;
                    c.mov_s(12, 20);
                    c.g[V0][0] = c.g[V0][0] ^ 0x6;
                    c.mov_s(13, 22);
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = c.g[SP][0];
                    }
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x30d8));
                    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, c.g[V1][0] as u32);
                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sdl(2, (c.g[S4][0] as u32).wrapping_add(0x47));
                    c.sdr(2, (c.g[S4][0] as u32).wrapping_add(0x40));
                    c.sdl(6, (c.g[S4][0] as u32).wrapping_add(0x4f));
                    c.sdr(6, (c.g[S4][0] as u32).wrapping_add(0x48));
                    c.g[RA][0] = sx(0x0017ca98);
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x00391cb0, 0x0017ca98)?;
                    c.g[A0][0] = c.g[FP][0];
                    c.g[RA][0] = sx(0x0017caa4);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x00147618, 0x0017caa4)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.mov_s(13, 22);
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S4][0];
                    c.mov_s(12, 21);
                    c.g[RA][0] = sx(0x0017cac0);
                    c.add_s(22, 22, 23);
                    call_ptr(c, 0x001dca80, 0x0017cac0)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc1c));
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = 0x4a0000;
                    c.mov_s(13, 22);
                    c.g[V0][0] = c.g[V0][0] ^ 0x7;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x12d8));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = c.g[SP][0];
                    }
                    c.g[A0][0] = c.g[S4][0];
                    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, c.g[V1][0] as u32);
                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sdl(2, (c.g[S4][0] as u32).wrapping_add(0x47));
                    c.sdr(2, (c.g[S4][0] as u32).wrapping_add(0x40));
                    c.sdl(6, (c.g[S4][0] as u32).wrapping_add(0x4f));
                    c.sdr(6, (c.g[S4][0] as u32).wrapping_add(0x48));
                    c.g[RA][0] = sx(0x0017cb08);
                    c.mov_s(12, 20);
                    call_ptr(c, 0x00391cb0, 0x0017cb08)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc24));
                    c.g[V1][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x1e10));
                    c.mov_s(13, 22);
                    c.g[A0][0] = c.g[S4][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.mov_s(12, 21);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.add_s(22, 22, 23);
                    c.g[RA][0] = sx(0x0017cb30);
                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                    call_ptr(c, 0x00392a60, 0x0017cb30)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc1c));
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = 0x460000;
                    c.mov_s(13, 22);
                    c.g[V0][0] = c.g[V0][0] ^ 0x8;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x30c8));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = c.g[SP][0];
                    }
                    c.g[A0][0] = c.g[S4][0];
                    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, c.g[V1][0] as u32);
                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sdl(2, (c.g[S4][0] as u32).wrapping_add(0x47));
                    c.sdr(2, (c.g[S4][0] as u32).wrapping_add(0x40));
                    c.sdl(6, (c.g[S4][0] as u32).wrapping_add(0x4f));
                    c.sdr(6, (c.g[S4][0] as u32).wrapping_add(0x48));
                    c.g[RA][0] = sx(0x0017cb78);
                    c.mov_s(12, 20);
                    call_ptr(c, 0x00391cb0, 0x0017cb78)?;
                    c.g[V0][0] = 0x530000;
                    c.mov_s(13, 22);
                    c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c04));
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x0017cb94);
                    c.mov_s(12, 21);
                    call_ptr(c, 0x001dca80, 0x0017cb94)?;
                    c.add_s(22, 22, 23);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc1c));
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[V0][0] = c.g[V0][0] ^ 0x9;
                    c.mov_s(13, 22);
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = c.g[SP][0];
                    }
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x30b0));
                    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, c.g[V1][0] as u32);
                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sdl(2, (c.g[S4][0] as u32).wrapping_add(0x47));
                    c.sdr(2, (c.g[S4][0] as u32).wrapping_add(0x40));
                    c.sdl(6, (c.g[S4][0] as u32).wrapping_add(0x4f));
                    c.sdr(6, (c.g[S4][0] as u32).wrapping_add(0x48));
                    c.g[RA][0] = sx(0x0017cbe0);
                    c.mov_s(12, 20);
                    call_ptr(c, 0x00391cb0, 0x0017cbe0)?;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc30));
                    c.g[A1][0] = c.g[S2][0];
                    c.mov_s(13, 22);
                    c.mov_s(12, 21);
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x0017cbfc);
                    c.add_s(22, 22, 23);
                    call_ptr(c, 0x001dca80, 0x0017cbfc)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc1c));
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[V0][0] = c.g[V0][0] ^ 0xa;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x12e8));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = c.g[SP][0];
                    }
                    c.mov_s(12, 20);
                    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, c.g[V1][0] as u32);
                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sdl(2, (c.g[S4][0] as u32).wrapping_add(0x47));
                    c.sdr(2, (c.g[S4][0] as u32).wrapping_add(0x40));
                    c.sdl(6, (c.g[S4][0] as u32).wrapping_add(0x4f));
                    c.sdr(6, (c.g[S4][0] as u32).wrapping_add(0x48));
                    c.g[RA][0] = sx(0x0017cc44);
                    c.mov_s(13, 22);
                    call_ptr(c, 0x00391cb0, 0x0017cc44)?;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0017cc5c);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0017cc5c)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        break 'b_17cc6c;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3098));
                        if t {
                            break 'b_17cc74;
                        } else {
                            break 'b_17cc6c;
                        }
                    }
                }
                c.cycles += 2;
                c.g[V0][0] = 0x460000;
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3088));
                break 'b_17cc74;
            }
            'b_17cd34: {
                c.cycles += 46;
                c.g[AT][0] = 0x44110000;
                c.mtc1(1, 12);
                c.g[A1][0] = c.g[S1][0];
                c.mov_s(13, 22);
                c.g[RA][0] = sx(0x0017cc8c);
                c.g[A0][0] = c.g[S4][0];
                call_ptr(c, 0x001dca80, 0x0017cc8c)?;
                c.add_s(22, 22, 23);
                c.g[V0][0] = 0x8000;
                c.g[AT][0] = 0x42700000;
                c.mtc1(1, 12);
                c.g[S1][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A1][0] = 0x460000;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc1c));
                c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.mov_s(13, 22);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3078));
                c.g[V1][0] = c.g[V1][0] ^ 0xb;
                c.g[A0][0] = c.g[S4][0];
                if c.g[V1][0] == 0 {
                    c.g[V0][0] = c.g[SP][0];
                }
                c.ldl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
                c.ldr(3, c.g[V0][0] as u32);
                c.ldl(6, (c.g[V0][0] as u32).wrapping_add(0xf));
                c.ldr(6, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.sdl(3, (c.g[S4][0] as u32).wrapping_add(0x47));
                c.sdr(3, (c.g[S4][0] as u32).wrapping_add(0x40));
                c.sdl(6, (c.g[S4][0] as u32).wrapping_add(0x4f));
                c.sdr(6, (c.g[S4][0] as u32).wrapping_add(0x48));
                c.g[RA][0] = sx(0x0017cce8);
                c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x12f0));
                call_ptr(c, 0x00391cb0, 0x0017cce8)?;
                c.g[RA][0] = sx(0x0017ccf0);
                call_ptr(c, 0x0028b180, 0x0017ccf0)?;
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc34));
                c.g[V1][0] = 0x14;
                c.g[AT][0] = 0x44110000;
                c.mtc1(1, 12);
                c.mov_s(13, 22);
                c.mult(0, 6, 6, 3);
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[S0][0];
                c.add_s(22, 22, 23);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x60a4));
                c.g[RA][0] = sx(0x0017cd20);
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                call_ptr(c, 0x001dca80, 0x0017cd20)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                let t = c.g[A0][0] == 0u64;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x100), 0u64);
                if t {
                    break 'b_17cd34;
                } else {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0017cd34);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                    call_ptr(c, 0x00256038, 0x0017cd34)?;
                    break 'b_17cd34;
                }
            }
            'b_17ce04: {
                'b_17cdd0: {
                    c.cycles += 25;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x100));
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.g[A3][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x102));
                    c.g[A1][0] = 0x460000;
                    c.g[T0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x101));
                    c.g[T1][0] = c.g[A2][0] & 0xff;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3068));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0017cd5c);
                    c.g[A2][0] = sx((c.g[A2][0] as u32) >> 24);
                    call_ptr(c, 0x00417828, 0x0017cd5c)?;
                    c.g[V0][0] = 0x460000;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3048));
                    c.ldl(4, (c.g[T0][0] as u32).wrapping_add(0x7));
                    c.ldr(4, c.g[T0][0] as u32);
                    c.ldl(5, (c.g[T0][0] as u32).wrapping_add(0xf));
                    c.ldr(5, (c.g[T0][0] as u32).wrapping_add(0x8));
                    c.g[A3][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
                    c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0xd7));
                    c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0xdf));
                    c.sdr(5, (c.g[SP][0] as u32).wrapping_add(0xd8));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[A3][0]);
                    let t = c.g[V1][0] == 0u64;
                    c.g[A2][0] = c.g[S0][0];
                    if t {
                        break 'b_17cdd0;
                    } else {
                        c.cycles += 14;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc1c));
                        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[V0][0] = c.g[V0][0] ^ 0xc;
                        if c.g[V0][0] == 0 {
                            c.g[V1][0] = c.g[SP][0];
                        }
                        c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.ldr(2, c.g[V1][0] as u32);
                        c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
                        c.ldr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.sdl(2, (c.g[S4][0] as u32).wrapping_add(0x47));
                        c.sdr(2, (c.g[S4][0] as u32).wrapping_add(0x40));
                        c.sdl(4, (c.g[S4][0] as u32).wrapping_add(0x4f));
                        c.sdr(4, (c.g[S4][0] as u32).wrapping_add(0x48));
                        let t = true;
                        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                        if t {
                            break 'b_17ce04;
                        } else {
                            break 'b_17cdd0;
                        }
                    }
                }
                c.cycles += 13;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc1c));
                c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S1][0] = c.g[V1][0];
                c.g[V0][0] = c.g[V0][0] ^ 0xc;
                if c.g[V0][0] == 0 {
                    c.g[V1][0] = c.g[SP][0];
                }
                c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.ldr(8, c.g[V1][0] as u32);
                c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0xf));
                c.ldr(2, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.sdl(8, (c.g[S4][0] as u32).wrapping_add(0x47));
                c.sdr(8, (c.g[S4][0] as u32).wrapping_add(0x40));
                c.sdl(2, (c.g[S4][0] as u32).wrapping_add(0x4f));
                c.sdr(2, (c.g[S4][0] as u32).wrapping_add(0x48));
                break 'b_17ce04;
            }
            'b_17cf60: {
                c.cycles += 66;
                c.g[AT][0] = 0x42700000;
                c.mtc1(1, 21);
                c.g[A1][0] = 0x460000;
                c.mov_s(13, 22);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3030));
                c.g[A0][0] = c.g[S4][0];
                c.g[RA][0] = sx(0x0017ce24);
                c.mov_s(12, 21);
                call_ptr(c, 0x001dc990, 0x0017ce24)?;
                c.g[AT][0] = 0x43a00000;
                c.mtc1(1, 20);
                c.g[S0][0] = 0x8000;
                c.g[AT][0] = 0x44110000;
                c.mtc1(1, 12);
                c.g[S0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x12f0));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc44));
                c.mov_s(13, 22);
                c.g[A0][0] = c.g[S4][0];
                c.add_s(22, 22, 23);
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[RA][0] = sx(0x0017ce60);
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                call_ptr(c, 0x001dca80, 0x0017ce60)?;
                c.g[AT][0] = 0x40a00000;
                c.mtc1(1, 0);
                c.g[V1][0] = c.g[SP][0];
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc1c));
                c.g[A1][0] = 0x4a0000;
                c.add_s(22, 22, 0);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1318));
                c.g[V0][0] = c.g[V0][0] ^ 0xd;
                c.g[A0][0] = c.g[S4][0];
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = c.g[S1][0];
                }
                c.mov_s(12, 20);
                c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.ldr(6, c.g[V1][0] as u32);
                c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0xf));
                c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.sdl(6, (c.g[S4][0] as u32).wrapping_add(0x47));
                c.sdr(6, (c.g[S4][0] as u32).wrapping_add(0x40));
                c.sdl(7, (c.g[S4][0] as u32).wrapping_add(0x4f));
                c.sdr(7, (c.g[S4][0] as u32).wrapping_add(0x48));
                c.g[RA][0] = sx(0x0017ceb4);
                c.mov_s(13, 22);
                call_ptr(c, 0x001dca40, 0x0017ceb4)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc1c));
                c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.add_s(13, 22, 23);
                c.g[A1][0] = 0x460000;
                c.g[V0][0] = c.g[V0][0] ^ 0xe;
                c.g[A0][0] = c.g[S4][0];
                if c.g[V0][0] == 0 {
                    c.g[V1][0] = c.g[SP][0];
                }
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3020));
                c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.ldr(6, c.g[V1][0] as u32);
                c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0xf));
                c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.sdl(6, (c.g[S4][0] as u32).wrapping_add(0x47));
                c.sdr(6, (c.g[S4][0] as u32).wrapping_add(0x40));
                c.sdl(7, (c.g[S4][0] as u32).wrapping_add(0x4f));
                c.sdr(7, (c.g[S4][0] as u32).wrapping_add(0x48));
                c.g[RA][0] = sx(0x0017cefc);
                c.mov_s(12, 20);
                call_ptr(c, 0x001dca40, 0x0017cefc)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                let t = c.g[V1][0] != 0u64;
                c.g[A1][0] = 0x460000;
                if t {
                    break 'b_17cf60;
                } else {
                    c.cycles += 21;
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5b14));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 1);
                    c.mov_s(12, 20);
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xf8));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3010));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xf4));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xfc), 0u64);
                    c.g[AT][0] = 0x43d20000;
                    c.mtc1(1, 13);
                    c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0xf7));
                    c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0xff));
                    c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0xf8));
                    c.sdl(3, (c.g[S4][0] as u32).wrapping_add(0x47));
                    c.sdr(3, (c.g[S4][0] as u32).wrapping_add(0x40));
                    c.sdl(6, (c.g[S4][0] as u32).wrapping_add(0x4f));
                    c.sdr(6, (c.g[S4][0] as u32).wrapping_add(0x48));
                    c.g[RA][0] = sx(0x0017cf60);
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x00392908, 0x0017cf60)?;
                    break 'b_17cf60;
                }
            }
            c.cycles += 27;
            c.g[AT][0] = 0x3f000000;
            c.mtc1(1, 0);
            c.g[A1][0] = 0x460000;
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 1);
            c.g[A0][0] = c.g[S4][0];
            c.mov_s(14, 0);
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xf4));
            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xf0));
            c.mov_s(12, 21);
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xf8));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2ff8));
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xfc));
            c.mov_s(15, 14);
            c.g[AT][0] = 0x43d20000;
            c.mtc1(1, 13);
            c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0xf7));
            c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
            c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0xff));
            c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0xf8));
            c.sdl(2, (c.g[S4][0] as u32).wrapping_add(0x47));
            c.sdr(2, (c.g[S4][0] as u32).wrapping_add(0x40));
            c.sdl(3, (c.g[S4][0] as u32).wrapping_add(0x4f));
            c.sdr(3, (c.g[S4][0] as u32).wrapping_add(0x48));
            c.g[RA][0] = sx(0x0017cfc8);
            c.g[A2][0] = 0x7;
            call_ptr(c, 0x001dc9d0, 0x0017cfc8)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1a0));
            break 'b_17cfcc;
        }
        c.cycles += 15;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x190));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x180));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x170));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x150));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x110));
        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x1c8));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x1c0));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x1b8));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x1b0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1d0));
        c.pc = t;
        return Ok(());
    });
}

/// `DebugMenu_DispatchItemAction(self, a1, item)`: the AI-rider count for a
/// debug-menu item. Default 5 (4 when `a1 == 2`); items 0..7 jump through the
/// table at 0x45d040 (handler 0x17d1ec gives 0, 0x17d1f4 gives 1).
/// The resume entries run `f_0017d1b8__entries`.
pub fn oc817_debug_menu_dispatch_item_action(c: &mut Cpu) {
    if c.pc != 0x0017d1b8 {
        return ee::s02::f_0017d1b8__entries(c);
    }
    c.cycles += 6;
    c.g[A1][0] ^= 0x2;
    c.g[A0][0] = 0x5;
    c.g[V0][0] = 0x4;
    c.g[V1][0] = (c.g[A2][0] < 0x7u64) as u64;
    let out_of_range = c.g[V1][0] == 0u64;
    if c.g[A1][0] == 0 {
        c.g[A0][0] = c.g[V0][0];
    }
    if !out_of_range {
        c.cycles += 7;
        c.g[V0][0] = 0x460000;
        c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2fc0));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
        let t = c.g[A0][0] as u32;
        match t {
            0x0017d1ec => {
                c.cycles += 2;
                c.g[A0][0] = 0u64;
            }
            0x0017d1f4 => {
                c.cycles += 1;
                c.g[A0][0] = 0x1;
            }
            _ => {
                c.jump_out(t, 0x0017d1b8, 0x0017d200);
                return;
            }
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.g[A0][0];
    c.pc = t;
}

/// `DebugMenu_StartMultiplayer` (0x0017d0a8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn oc817_debug_menu_start_multiplayer(c: &mut Cpu) {
    if c.pc != 0x0017d0a8 {
        return ee::s02::f_0017d0a8__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x0017d0a8;
        loop {
            match pc {
                0x0017d0a8 => {
                    c.cycles += 12;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = 0x7;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                    c.g[RA][0] = sx(0x0017d0c8);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                    call_ptr(c, 0x0014dd58, 0x0017d0c8)?;
                    c.g[V1][0] = 0x530000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4b30));
                    let t = c.g[V0][0] == 0u64;
                    c.g[S1][0] = c.g[V1][0];
                    if t {
                        pc = 0x0017d1a4;
                        continue;
                    }
                    pc = 0x0017d0d8;
                    continue;
                }
                0x0017d0d8 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    c.g[RA][0] = sx(0x0017d0e4);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    call_ptr(c, 0x00256038, 0x0017d0e4)?;
                    c.g[V0][0] = 0xa0a0000;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = c.g[V0][0] | 0x3519;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x460000;
                    if t {
                        pc = 0x0017d108;
                        continue;
                    }
                    pc = 0x0017d0f8;
                    continue;
                }
                0x0017d0f8 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x460000;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2fe8));
                    let t = true;
                    c.g[A0][0] = c.ld((c.g[V0][0] as u32).wrapping_sub(0x2fe8));
                    if t {
                        pc = 0x0017d110;
                        continue;
                    }
                    pc = 0x0017d108;
                    continue;
                }
                0x0017d108 => {
                    c.cycles += 2;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2fd8));
                    c.g[A0][0] = c.ld((c.g[V0][0] as u32).wrapping_sub(0x2fd8));
                    pc = 0x0017d110;
                    continue;
                }
                0x0017d110 => {
                    c.cycles += 7;
                    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[SP][0] as u32, c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4b30));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[SP][0];
                    if t {
                        pc = 0x0017d13c;
                        continue;
                    }
                    pc = 0x0017d12c;
                    continue;
                }
                0x0017d12c => {
                    c.cycles += 4;
                    c.g[A2][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    let t = true;
                    c.g[A3][0] = 0x1;
                    if t {
                        pc = 0x0017d148;
                        continue;
                    }
                    pc = 0x0017d13c;
                    continue;
                }
                0x0017d13c => {
                    c.cycles += 3;
                    c.g[A2][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    c.g[A3][0] = 0u64;
                    pc = 0x0017d148;
                    continue;
                }
                0x0017d148 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0017d150);
                    call_ptr(c, 0x00255bf8, 0x0017d150)?;
                    let t = true;
                    c.g[S0][0] = c.g[V0][0];
                    if t {
                        pc = 0x0017d170;
                        continue;
                    }
                    pc = 0x0017d158;
                    continue;
                }
                0x0017d158 => {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x0017d160);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    call_ptr(c, 0x00255a20, 0x0017d160)?;
                    c.g[RA][0] = sx(0x0017d168);
                    c.g[A0][0] = 0u64;
                    call_ptr(c, 0x003e5928, 0x0017d168)?;
                    c.g[RA][0] = sx(0x0017d170);
                    c.g[A0][0] = 0x1;
                    call_ptr(c, 0x003e52b0, 0x0017d170)?;
                    pc = 0x0017d170;
                    continue;
                }
                0x0017d170 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    c.g[RA][0] = sx(0x0017d17c);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x00255d50, 0x0017d17c)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4b30));
                    if t {
                        if back_edge_unwinds(c, 0x0017d158) {
                            return Ok(());
                        }
                        pc = 0x0017d158;
                        continue;
                    }
                    pc = 0x0017d184;
                    continue;
                }
                0x0017d184 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0017d190);
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    call_ptr(c, 0x00256bf8, 0x0017d190)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
                    c.g[RA][0] = sx(0x0017d19c);
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                    call_ptr(c, 0x00266d58, 0x0017d19c)?;
                    c.g[RA][0] = sx(0x0017d1a4);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
                    call_ptr(c, 0x00266e30, 0x0017d1a4)?;
                    pc = 0x0017d1a4;
                    continue;
                }
                0x0017d1a4 => {
                    c.cycles += 5;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0017d0a8, 0x0017d1b8);
                    return Ok(());
                }
            }
        }
    });
}

/// `dst = src + off` (`addiu`, sign-extended 32-bit result).
pub(crate) fn oc818_lea(c: &mut Cpu, dst: usize, src: usize, off: u32) {
    c.g[dst][0] = sx(r32(c, src).wrapping_add(off));
}

/// The destructor's loops over 0x24-byte members, walking backwards from the
/// list end in `s0` to the list head in `s2` and calling each member's
/// virtual destructor (`a0 = member + vtable->delta`, `a1 = 0`). `always_dec`:
/// the member pointer is decremented in the branch delay slot (the other
/// loop only when it goes round again).
pub(crate) fn oc818_dtor_walk(c: &mut Cpu, head: u32, ret: u32, always_dec: bool) -> Flow {
    loop {
        c.cycles += 8;
        c.g[A1][0] = 0;
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x10));
        c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(8));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0xc));
        let target = r32(c, V1);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
        call_ptr(c, target, ret)?;
        let more = c.g[S2][0] != c.g[S0][0];
        if always_dec || more {
            c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x24));
        }
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `void LobbyMenu_dtor(LobbyMenu* self, int flags)` (0x17d3b0): destroy the
/// lobby menu: its three strings, the seven sub-menus and the menu system in
/// reverse order, restore the base vtable and free the object when bit 0 of
/// `flags` is set. The generated body's resume entries stay in the generated
/// twin.
pub fn lobby_menu_dtor(c: &mut Cpu) {
    if c.pc != 0x0017_d3b0 {
        return ee::s02::f_0017d3b0__entries(c);
    }
    guest(c, |c| {
        c.cycles += 59;
        let frame = Frame::open(c, 0xb0);
        c.g[V0][0] = 0x47_0000;
        frame.save(c, S1, 0x90);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x2cb0));
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0xa0);
        frame.save(c, S2, 0x80);
        oc818_lea(c, A0, S1, 0x1c0c);
        frame.save(c, S4, 0x60);
        oc818_lea(c, S0, S1, 0x1aac);
        frame.save(c, S5, 0x50);
        oc818_lea(c, S2, S1, 0x18c0);
        frame.save(c, S3, 0x70);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S6, 0x40);
        c.g[A1][0] = 2;
        frame.save(c, S7, 0x30);
        oc818_lea(c, S4, S1, 0x1534);
        frame.save(c, FP, 0x20);
        frame.save_ra(c, 0x10);
        c.sw(r32(c, S1), c.g[V0][0]);
        c.sw(r32(c, GP).wrapping_add(0x1da0), 0);
        call_ptr(c, 0x0031_8260, 0x0017_d40c)?;
        oc818_lea(c, A0, S1, 0x1c08);
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0017_d418)?;
        oc818_lea(c, A0, S1, 0x1c04);
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0017_d424)?;
        oc818_lea(c, A0, S1, 0x1be0);
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_a280, 0x0017_d430)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_aa80, 0x0017_d43c)?;
        for (off, ret) in [
            (0x1a88u32, 0x0017_d448u32),
            (0x1a64, 0x0017_d454),
            (0x1a4c, 0x0017_d460),
            (0x1a30, 0x0017_d46c),
            (0x1a14, 0x0017_d478),
            (0x19f8, 0x0017_d484),
        ] {
            oc818_lea(c, A0, S1, off);
            c.g[A1][0] = 2;
            call_ptr(c, 0x002c_a280, ret)?;
        }
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_aa80, 0x0017_d490)?;

        // Members at s1+0x16c8..0x18c0.
        oc818_lea(c, V0, S1, 0x16c8);
        let empty = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[S2][0];
        let mut tail = true;
        if !empty {
            c.cycles += 2;
            if c.g[V0][0] == c.g[S0][0] {
                tail = false;
            } else {
                c.cycles += 3;
                c.g[S2][0] = c.g[V0][0];
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x24));
                oc818_dtor_walk(c, 0x0017_d4b0, 0x0017_d4c8, true)?;
            }
        }
        if tail {
            c.cycles += 1;
        }
        oc818_lea(c, A0, S4, 0x168);

        c.cycles += 31;
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_a280, 0x0017_d4dc)?;
        c.g[S5][0] &= 1;
        oc818_lea(c, A0, S4, 0x150);
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_a280, 0x0017_d4ec)?;
        oc818_lea(c, S0, S1, 0x1510);
        oc818_lea(c, A0, S4, 0x134);
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_a280, 0x0017_d4fc)?;
        oc818_lea(c, S3, S1, 0x1044);
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_aa80, 0x0017_d50c)?;
        c.g[S6][0] = c.g[S3][0];
        let sp0 = frame.at(c, 0);
        c.sw(sp0.wrapping_add(0xc), c.g[S5][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        oc818_lea(c, S4, S1, 0xd2c);
        call_ptr(c, 0x002c_a280, 0x0017_d524)?;
        oc818_lea(c, S5, S1, 0xe94);
        oc818_lea(c, V1, S1, 0x604);
        oc818_lea(c, V0, S1, 0x11b0);
        c.sw(sp0.wrapping_add(4), c.g[V1][0]);
        oc818_lea(c, S7, S1, 0x74c);
        oc818_lea(c, V1, S1, 0x70);
        oc818_lea(c, FP, S1, 0x734);
        c.sw(sp0, c.g[V1][0]);
        oc818_lea(c, V1, S1, 8);
        let empty = c.g[V0][0] == 0;
        c.sw(sp0.wrapping_add(8), c.g[V1][0]);

        // Members at s1+0x11b0..0x1510.
        let mut tail = true;
        if !empty {
            c.cycles += 2;
            if c.g[V0][0] == c.g[S0][0] {
                tail = false;
            } else {
                c.cycles += 2;
                c.g[S2][0] = c.g[V0][0];
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x24));
                oc818_dtor_walk(c, 0x0017_d560, 0x0017_d578, false)?;
            }
        }
        if tail {
            c.cycles += 1;
        }
        oc818_lea(c, A0, S3, 0x154);

        c.cycles += 10;
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_a280, 0x0017_d58c)?;
        oc818_lea(c, A0, S3, 0x138);
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_a280, 0x0017_d598)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_aa80, 0x0017_d5a4)?;
        let none = c.g[S5][0] == 0;
        c.g[S0][0] = c.g[S6][0];

        // Members at s1+0xe94..0x1044.
        if !none {
            c.cycles += 2;
            let same = c.g[S5][0] == c.g[S0][0];
            oc818_lea(c, S2, S4, 0x168);
            if !same {
                c.cycles += 1;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x24));
                oc818_dtor_walk(c, 0x0017_d5b8, 0x0017_d5d0, true)?;
            }
        }

        c.cycles += 30;
        oc818_lea(c, A0, S4, 0x150);
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_a280, 0x0017_d5e4)?;
        oc818_lea(c, A0, S4, 0x134);
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_a280, 0x0017_d5f0)?;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_aa80, 0x0017_d5fc)?;
        c.g[A0][0] = c.g[S7][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x001d_d0c8, 0x0017_d608)?;
        c.g[A0][0] = c.g[FP][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_a280, 0x0017_d614)?;
        c.g[A0][0] = c.lw(sp0.wrapping_add(4));
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_aa80, 0x0017_d620)?;
        c.g[A0][0] = c.lw(sp0);
        c.g[A1][0] = 2;
        call_ptr(c, 0x001d_cea0, 0x0017_d62c)?;
        c.g[A0][0] = c.lw(sp0.wrapping_add(8));
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_c048, 0x0017_d638)?;
        c.g[V1][0] = 0x47_0000;
        c.g[V0][0] = sx(r32(c, V1).wrapping_sub(0x26b8));
        c.sw(r32(c, S1), c.g[V0][0]);
        c.g[V0][0] = c.lw(sp0.wrapping_add(0xc));
        let keep = c.g[V0][0] == 0;
        frame.restore(c, S0, 0xa0);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0017_d658)?;
            frame.restore(c, S0, 0xa0);
        }
        c.cycles += 11;
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

/// `void LobbyMenu_ShowError(LobbyMenu* self, int code)` (0x17da30): store
/// the error code at `+0x1c14` and switch the menu to the error page that
/// fits the current state (state - 2 indexes a jump table of eleven
/// handlers; other states leave it alone).
pub fn lobby_menu_show_error(c: &mut Cpu) {
    if c.pc != 0x0017_da30 {
        return ee::s02::f_0017da30__entries(c);
    }
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw(r32(c, A2).wrapping_add(0x1c14), c.g[A1][0]);
        c.g[V1][0] = c.lw(r32(c, A2).wrapping_add(4));
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(2));
        c.g[V0][0] = (c.g[V1][0] < 0xc) as u64;
        let out = c.g[V0][0] == 0;
        c.g[V0][0] = 0x46_0000;
        // What each handler does before the common tail: (extra cycles,
        // page-state value for the common push), or one of the odd exits.
        enum Exit {
            Common(u32, u64),
            Last,
            Leave,
        }
        let exit = if out {
            Exit::Last
        } else {
            c.cycles += 6;
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x2e50));
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A0][0] = c.lw(r32(c, V1));
            let target = r32(c, A0);
            match target {
                0x0017_da6c => Exit::Common(2, 0x12),
                0x0017_da74 => Exit::Common(2, 0x14),
                0x0017_da7c => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
                    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x74));
                    let no_world_link = c.g[V0][0] == 0;
                    c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0xc8));
                    if no_world_link {
                        Exit::Common(2, 0x15)
                    } else {
                        c.cycles += 12;
                        c.g[A1][0] = c.lw(r32(c, V1).wrapping_add(0xc4));
                        oc818_lea(c, A0, A0, 0x48);
                        call_ptr(c, 0x0031_84e8, 0x0017_da98)?;
                        c.g[V1][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
                        c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0xc8));
                        c.g[A1][0] = c.lw(r32(c, V1).wrapping_add(0xc8));
                        oc818_lea(c, A0, A0, 0x4c);
                        call_ptr(c, 0x0031_84e8, 0x0017_daac)?;
                        c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0xc8));
                        call_ptr(c, 0x0025_c610, 0x0017_dab4)?;
                        Exit::Leave
                    }
                }
                0x0017_dac4 => Exit::Common(2, 0x16),
                0x0017_dacc => Exit::Common(2, 0x16),
                0x0017_dad4 => Exit::Common(2, 0x16),
                0x0017_dadc => Exit::Common(1, 0x17),
                0x0017_daf8 => {
                    c.cycles += 5;
                    c.g[V0][0] = 0x18;
                    oc818_lea(c, A1, A2, 0x604);
                    c.sw(r32(c, A2).wrapping_add(4), c.g[V0][0]);
                    oc818_lea(c, A0, A2, 8);
                    call_ptr(c, 0x002c_c3b8, 0x0017_db0c)?;
                    Exit::Last
                }
                0x0017_db0c => Exit::Last,
                _ => {
                    c.jump_out(target, 0x0017_da30, 0x0017_db18);
                    return Ok(());
                }
            }
        };
        match exit {
            Exit::Common(extra, state) => {
                c.cycles += extra as u64;
                c.g[V0][0] = state;
                c.cycles += 6;
                oc818_lea(c, A1, A2, 0x604);
                c.sw(r32(c, A2).wrapping_add(4), c.g[V0][0]);
                oc818_lea(c, A0, A2, 8);
                call_ptr(c, 0x002c_c3b8, 0x0017_daf0)?;
                frame.restore_ra(c, 0);
            }
            Exit::Last => {
                c.cycles += 1;
                frame.restore_ra(c, 0);
            }
            Exit::Leave => frame.restore_ra(c, 0),
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// Where `LobbyMenu_UpdateState` goes after a state's handler.
#[derive(Clone, Copy)]
pub(crate) enum Oc818Next {
    /// Another handler of the jump table (by its target address).
    Seg(u32),
    /// The tail that reloads the state first (`+1` cycle).
    Pre,
    /// The state comparison (`v0` already holds the state).
    Cmp,
}

/// `cMenuSys_IsActive(&self->menuSys)` into `v0`, `reg = &self->menuSys`.
pub(crate) fn oc818_us_is_active(c: &mut Cpu, reg: usize, ret: u32) -> Flow {
    c.cycles += 5;
    oc818_lea(c, reg, S1, 8);
    c.g[A0][0] = c.g[reg][0];
    call_ptr(c, 0x002c_c098, ret)
}

/// `cMenuSys_PushPage(&self->menuSys, page)`.
pub(crate) fn oc818_us_push(c: &mut Cpu, ret: u32) -> Flow {
    call_ptr(c, 0x002c_c3b8, ret)
}

/// `v0 = self->state`.
pub(crate) fn oc818_us_state(c: &mut Cpu) {
    c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(4));
}

/// One state handler of `LobbyMenu_UpdateState`, entered at the generated
/// block `target` of its jump table.
pub(crate) fn oc818_us_handler(c: &mut Cpu, target: u32) -> Result<Oc818Next, Unwound> {
    use Oc818Next::*;
    Ok(match target {
        // State 0: connect the network layer.
        0x0017_db5c => {
            c.cycles += 5;
            c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0x238));
            c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0x20));
            c.g[V0][0] ^= 1;
            let busy = c.g[V0][0] != 0;
            c.g[V0][0] = 1;
            if !busy {
                c.cycles += 3;
                c.g[A1][0] = 0;
                call_ptr(c, 0x0025_5f50, 0x0017_db78)?;
                c.g[V0][0] = 1;
            }
            c.cycles += 2;
            c.sw(r32(c, S1).wrapping_add(4), c.g[V0][0]);
            Pre
        }
        // State 1: wait for the connection.
        0x0017_db84 => {
            c.cycles += 6;
            c.g[V1][0] = c.lw(r32(c, GP).wrapping_sub(0x238));
            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x20));
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(3));
            c.g[V0][0] = (c.g[V0][0] < 2) as u64;
            if c.g[V0][0] != 0 {
                oc818_us_state(c);
                Cmp
            } else {
                c.cycles += 6;
                c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0xc8));
                c.g[A1][0] = 0;
                call_ptr(c, 0x0025_b6d8, 0x0017_dba8)?;
                c.g[V0][0] = 2;
                c.sw(r32(c, S1).wrapping_add(4), c.g[V0][0]);
                Pre
            }
        }
        // State 13: push the abort page.
        0x0017_dbb4 => {
            oc818_us_is_active(c, S0, 0x0017_dbc0)?;
            if c.g[V0][0] != 0 {
                oc818_us_state(c);
                Cmp
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                oc818_lea(c, A1, S1, 0x1aac);
                oc818_us_push(c, 0x0017_dbd4)?;
                oc818_us_state(c);
                Cmp
            }
        }
        // State 17: occupants page, state 12.
        0x0017_dbdc => {
            oc818_us_is_active(c, S0, 0x0017_dbe8)?;
            if c.g[V0][0] != 0 {
                oc818_us_state(c);
                Cmp
            } else {
                c.cycles += 7;
                c.g[V0][0] = 0xc;
                c.g[A0][0] = c.g[S0][0];
                c.sw(r32(c, S1).wrapping_add(4), c.g[V0][0]);
                oc818_lea(c, A1, S1, 0x1044);
                oc818_us_push(c, 0x0017_dc04)?;
                oc818_us_state(c);
                Cmp
            }
        }
        // State 24: occupants page, then state 12.
        0x0017_dc0c => {
            oc818_us_is_active(c, S0, 0x0017_dc18)?;
            if c.g[V0][0] != 0 {
                oc818_us_state(c);
                Cmp
            } else {
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                oc818_lea(c, A1, S1, 0x1044);
                oc818_us_push(c, 0x0017_dc2c)?;
                c.g[V0][0] = 0xc;
                c.sw(r32(c, S1).wrapping_add(4), c.g[V0][0]);
                Pre
            }
        }
        // States 21 and 20: login page, state 3.
        0x0017_dc38 | 0x0017_dc5c => {
            let ret = if target == 0x0017_dc38 {
                0x0017_dc44
            } else {
                0x0017_dc68
            };
            oc818_us_is_active(c, S0, ret)?;
            if c.g[V0][0] != 0 {
                oc818_us_state(c);
                Cmp
            } else {
                c.cycles += 4;
                c.g[V0][0] = 3;
                c.g[A0][0] = c.g[S0][0];
                c.sw(r32(c, S1).wrapping_add(4), c.g[V0][0]);
                Seg(0x0017_dda8)
            }
        }
        // States 18 and 19: back to state 0 once the menu is gone.
        0x0017_dc80 => {
            c.cycles += 4;
            oc818_lea(c, A0, S1, 8);
            call_ptr(c, 0x002c_c098, 0x0017_dc88)?;
            if c.g[V0][0] == 0 {
                c.sw(r32(c, S1).wrapping_add(4), 0);
                Pre
            } else {
                c.cycles += 2;
                oc818_us_state(c);
                Cmp
            }
        }
        // State 22: persona page, state 6.
        0x0017_dc98 => {
            oc818_us_is_active(c, S2, 0x0017_dca4)?;
            if c.g[V0][0] != 0 {
                oc818_us_state(c);
                Cmp
            } else {
                c.cycles += 10;
                oc818_lea(c, S0, S1, 0x74c);
                c.g[A1][0] = c.lw(r32(c, GP).wrapping_sub(0xc8));
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x001d_b418, 0x0017_dcbc)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S0][0];
                oc818_us_push(c, 0x0017_dcc8)?;
                c.g[V0][0] = 6;
                c.sw(r32(c, S1).wrapping_add(4), c.g[V0][0]);
                Pre
            }
        }
        // State 23: room page, state 10.
        0x0017_dcd4 => {
            oc818_us_is_active(c, S0, 0x0017_dce0)?;
            if c.g[V0][0] != 0 {
                oc818_us_state(c);
                Cmp
            } else {
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                oc818_lea(c, A1, S1, 0xd2c);
                oc818_us_push(c, 0x0017_dcf4)?;
                c.g[V0][0] = 0xa;
                c.sw(r32(c, S1).wrapping_add(4), c.g[V0][0]);
                Pre
            }
        }
        // States 15 and 6: re-push the challenge / persona page when no
        // page is up.
        0x0017_dd00 | 0x0017_dd1c => {
            let (page, ret) = if target == 0x0017_dd00 {
                (0x18c0, 0x0017_dd14)
            } else {
                (0x74c, 0x0017_dd30)
            };
            c.cycles += 3;
            c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(8));
            let up = c.g[V0][0] != 0;
            oc818_lea(c, A0, S1, 8);
            if up {
                Pre
            } else {
                c.cycles += 4;
                oc818_lea(c, A1, S1, page);
                oc818_us_push(c, ret)?;
                oc818_us_state(c);
                Cmp
            }
        }
        // State 10: room list.
        0x0017_dd38 => {
            c.cycles += 4;
            c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(8));
            oc818_lea(c, A0, S1, 8);
            let up = c.g[V0][0] != 0;
            oc818_lea(c, S0, S1, 0xd2c);
            if !up {
                c.cycles += 2;
                c.g[A1][0] = c.g[S0][0];
                oc818_us_push(c, 0x0017_dd50)?;
            }
            c.cycles += 5;
            c.g[A1][0] = c.lw(r32(c, GP).wrapping_sub(0xc8));
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x001d_b528, 0x0017_dd5c)?;
            oc818_us_state(c);
            Cmp
        }
        // State 12: occupants list, or leave the room.
        0x0017_dd64 => {
            c.cycles += 3;
            c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(8));
            let up = c.g[V0][0] != 0;
            c.g[A1][0] = c.lw(r32(c, GP).wrapping_sub(0xc8));
            if !up {
                c.cycles += 7;
                c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0xc8));
                c.g[A1][0] = u64::MAX;
                c.g[A2][0] = 0;
                call_ptr(c, 0x0025_d800, 0x0017_dd80)?;
                c.g[V0][0] = 0xb;
                c.sw(r32(c, S1).wrapping_add(4), c.g[V0][0]);
                Pre
            } else {
                c.cycles += 4;
                oc818_lea(c, A0, S1, 0x1044);
                call_ptr(c, 0x001d_b620, 0x0017_dd94)?;
                oc818_us_state(c);
                Cmp
            }
        }
        // State 3: re-push the login page when no page is up.
        0x0017_dd9c => {
            c.cycles += 3;
            c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(8));
            let up = c.g[V0][0] != 0;
            oc818_lea(c, A0, S1, 8);
            if up {
                Pre
            } else {
                Seg(0x0017_dda8)
            }
        }
        // Push the login page (`a0 = &menuSys`).
        0x0017_dda8 => {
            c.cycles += 4;
            oc818_lea(c, A1, S1, 0x70);
            oc818_us_push(c, 0x0017_ddb0)?;
            oc818_us_state(c);
            Cmp
        }
        // State 14: occupants page, state 12.
        0x0017_ddb8 => {
            c.cycles += 3;
            c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(8));
            let up = c.g[V0][0] != 0;
            oc818_lea(c, A0, S1, 8);
            if up {
                Pre
            } else {
                c.cycles += 6;
                c.g[V0][0] = 0xc;
                oc818_lea(c, A1, S1, 0x1044);
                c.sw(r32(c, S1).wrapping_add(4), c.g[V0][0]);
                oc818_us_push(c, 0x0017_ddd4)?;
                oc818_us_state(c);
                Cmp
            }
        }
        // State 16: when the game channel is up, start the pre-game load.
        0x0017_dddc => {
            c.cycles += 5;
            c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0x238));
            c.g[A1][0] = c.lw(r32(c, S1).wrapping_add(0x1c10));
            call_ptr(c, 0x0025_5d50, 0x0017_dde8)?;
            let down = c.g[V0][0] == 0;
            c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(0xc8));
            if !down {
                c.cycles += 22;
                c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x1c10));
                c.g[A1][0] = c.lw(r32(c, V0).wrapping_add(0xb4));
                call_ptr(c, 0x0025_6bf8, 0x0017_ddfc)?;
                c.g[A1][0] = 0x46_0000;
                c.g[A3][0] = 0;
                c.g[A2][0] = 0x100;
                c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x31a0));
                c.g[A0][0] = 8;
                call_ptr(c, 0x0031_7d70, 0x0017_de14)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0023_2720, 0x0017_de1c)?;
                c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0031_6ef8, 0x0017_de28)?;
                c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0x238));
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, 0x0025_6038, 0x0017_de34)?;
                c.g[A0][0] = c.lw(r32(c, GP).wrapping_add(0x304));
                c.g[A1][0] = c.lw(sp(c));
                call_ptr(c, 0x0026_6d58, 0x0017_de40)?;
                c.g[A0][0] = c.lw(r32(c, GP).wrapping_add(0x304));
                call_ptr(c, 0x0026_6e30, 0x0017_de48)?;
            }
            Pre
        }
        // Not a handled state (a table entry of 0x17de48).
        _ => Pre,
    })
}

/// `void LobbyMenu_UpdateState(LobbyMenu* self)` (0x17db18): the lobby
/// menu's state machine. Run the handler of the current state (a jump table
/// of 25 entries: connect, log in, persona and room pages, occupants, start
/// of the pre-game load); repeat while a handler changed the state.
pub fn lobby_menu_update_state(c: &mut Cpu) {
    if c.pc != 0x0017_db18 {
        return ee::s02::f_0017db18__entries(c);
    }
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S0, 0x50);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S3][0] = c.lw(r32(c, S1).wrapping_add(4));
        loop {
            c.cycles += 3;
            c.g[V0][0] = (c.g[S3][0] < 0x19) as u64;
            let out = c.g[V0][0] == 0;
            c.g[V0][0] = 0x46_0000;
            let mut next = if out {
                Oc818Next::Pre
            } else {
                c.cycles += 6;
                c.g[V1][0] = sx(r32(c, S3) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x2e20));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.g[A0][0] = c.lw(r32(c, V1));
                let target = r32(c, A0);
                match target {
                    0x0017_db5c | 0x0017_db84 | 0x0017_dbb4 | 0x0017_dbdc | 0x0017_dc0c
                    | 0x0017_dc38 | 0x0017_dc5c | 0x0017_dc80 | 0x0017_dc98 | 0x0017_dcd4
                    | 0x0017_dd00 | 0x0017_dd1c | 0x0017_dd38 | 0x0017_dd64 | 0x0017_dd9c
                    | 0x0017_ddb8 | 0x0017_dddc => Oc818Next::Seg(target),
                    0x0017_de48 => Oc818Next::Pre,
                    _ => {
                        c.jump_out(target, 0x0017_db18, 0x0017_de70);
                        return Ok(());
                    }
                }
            };
            loop {
                match next {
                    Oc818Next::Seg(target) => next = oc818_us_handler(c, target)?,
                    Oc818Next::Pre => {
                        c.cycles += 1;
                        c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(4));
                        break;
                    }
                    Oc818Next::Cmp => break,
                }
            }
            c.cycles += 2;
            if c.g[V0][0] != c.g[S3][0] {
                c.g[S3][0] = c.lw(r32(c, S1).wrapping_add(4));
                if event_due(c) {
                    deliver_event_here(c, 0x0017_db38);
                }
                continue;
            }
            c.cycles += 7;
            frame.restore(c, S0, 0x50);
            frame.restore(c, S1, 0x40);
            frame.restore(c, S2, 0x30);
            frame.restore(c, S3, 0x20);
            frame.restore_ra(c, 0x10);
            frame.close(c);
            return Ok(());
        }
    })
}

/// `void LobbyMenu_StartNetRace(LobbyMenu* self)` (0x17de70): configure the
/// new-race interface (net game, mode 4, no AI), the player interface
/// (players 0 and 1, riders 2..5 cleared) and the network interface (host or
/// guest from the lobby manager), open the game channel to the opponent's
/// address and enter state 16.
pub fn lobby_menu_start_net_race(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x160);
        frame.save(c, S4, 0x110);
        frame.save(c, S0, 0x150);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S1, 0x140);
        frame.save(c, S2, 0x130);
        frame.save(c, S3, 0x120);
        c.g[S2][0] = 2;
        frame.save_ra(c, 0x100);
        c.g[S3][0] = 0x53_0000;
        call_ptr(c, 0x0014_dc80, 0x0017_de9c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0017_dea8)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_5108, 0x0017_deb8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 4;
        call_ptr(c, 0x0014_4df0, 0x0017_dec4)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_4d98, 0x0017_ded0)?;
        c.g[V1][0] = 0x53_0000;
        c.g[A0][0] = 1;
        c.sw(r32(c, V1).wrapping_add(0x5c08), 0);
        c.sw(r32(c, GP).wrapping_sub(0x1f38), c.g[A0][0]);
        oc818_start_net_race_vcall(c, S0, 0x0017_def4)?;
        call_ptr(c, 0x0014_dc80, 0x0017_defc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x0017_df08)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S2][0];
        loop {
            c.cycles += 11;
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = u64::MAX;
            call_ptr(c, 0x0014_7220, 0x0017_df1c)?;
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = 0;
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
            call_ptr(c, 0x0014_7338, 0x0017_df30)?;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 6) as u64;
            let again = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S2][0];
            if !again {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0017_df10);
            }
        }
        c.cycles += 31;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = 0;
        oc818_lea(c, S0, S3, 0x4b30);
        call_ptr(c, 0x0014_72c8, 0x0017_df50)?;
        c.g[A2][0] = u64::MAX;
        c.g[A1][0] = 1;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0014_72c8, 0x0017_df60)?;
        oc818_start_net_race_vcall(c, S1, 0x0017_df74)?;
        c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
        c.g[A1][0] = 7;
        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0x78));
        call_ptr(c, 0x0014_dd58, 0x0017_df84)?;
        c.g[V1][0] = 1;
        c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0xc8));
        c.sw(r32(c, S3).wrapping_add(0x4b30), c.g[V1][0]);
        c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(0xb4));
        c.sw(r32(c, S0).wrapping_add(8), c.g[V1][0]);
        c.g[A1][0] = c.lw(r32(c, V0).wrapping_add(0xc));
        c.g[A0][0] = c.lh(r32(c, A1).wrapping_add(8));
        c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(0xc));
        let target = r32(c, V1);
        c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0017_dfac)?;
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(8));
        let host = c.g[V0][0] == 0;
        c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(0xc8));
        if host {
            c.cycles += 1;
            c.g[A2][0] = c.lw(r32(c, V0).wrapping_add(0xa8));
        } else {
            c.cycles += 2;
            c.g[A2][0] = c.lw(r32(c, V0).wrapping_add(0xac));
        }
        c.cycles += 27;
        c.g[T1][0] = c.g[A2][0] & 0xff;
        c.g[A3][0] = sx(r32(c, A2) >> 16);
        c.g[T0][0] = sx(r32(c, A2) >> 8);
        c.g[A1][0] = 0x46_0000;
        c.g[A0][0] = c.g[SP][0];
        c.g[T0][0] &= 0xff;
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x3068));
        c.g[A3][0] &= 0xff;
        c.g[A2][0] = sx(r32(c, A2) >> 24);
        call_ptr(c, 0x0041_7828, 0x0017_dfec)?;
        oc818_lea(c, V1, S3, 0x4b30);
        c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0x238));
        c.g[A3][0] = c.lw(r32(c, V1).wrapping_add(8));
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = c.lhu(r32(c, V1).wrapping_add(0xc));
        call_ptr(c, 0x0025_5bf8, 0x0017_e004)?;
        c.g[V1][0] = 0x10;
        c.sw(r32(c, S4).wrapping_add(0x1c10), c.g[V0][0]);
        c.sw(r32(c, S4).wrapping_add(4), c.g[V1][0]);
        frame.restore(c, S0, 0x150);
        frame.restore(c, S1, 0x140);
        frame.restore(c, S2, 0x130);
        frame.restore(c, S3, 0x120);
        frame.restore(c, S4, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// A virtual call on the object at `*(reg + 0xc)`: `a0 = reg + vtable->delta`
/// (halfword at `vtable + 8`), target at `vtable + 0xc`, returning to `ret`.
pub(crate) fn oc818_start_net_race_vcall(c: &mut Cpu, reg: usize, ret: u32) -> Flow {
    c.g[V0][0] = c.lw(r32(c, reg).wrapping_add(0xc));
    c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(8));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0xc));
    let target = r32(c, V1);
    c.g[A0][0] = sx(r32(c, reg).wrapping_add(r32(c, A0)));
    call_ptr(c, target, ret)
}

/// `LobbyMenu_Render` (0x0017e5c8, cDebugLobbyScreen_draw): the debug lobby screen.
/// Draws the status text of the lobby state (jump table on `self.mState`, 0..24:
/// centred 'Connecting...' captions, error texts through `LobbyMan_GetErrorString`,
/// the 'Play Game' table of game name, opponent, host flag and addresses), then
/// the menu (`MenuSys_Draw`) and, with the comm system in net state 5, the local
/// address. The normal entry, converted from the generated labelled-block body
/// (every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (`__entries` now; at install switch to its `__generated` alias).
pub fn lobby_menu_render(c: &mut Cpu) {
    if c.pc != 0x0017e5c8 {
        return ee::s02::f_0017e5c8__entries(c);
    }
    guest(c, |c| 'l_17e5c8: loop {
        'b_17f1a8: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S2][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x78));
            c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x68));
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x80));
            c.g[A0][0] = (c.g[A1][0] < 0x19u64) as u64;
            let t = c.g[A0][0] == 0u64;
            c.g[S1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
            if t {
                break 'b_17f1a8;
            } else {
                'b_17ef20: {
                    'b_17eee8: {
                        'b_17ee24: {
                            'b_17ee14: {
                                'b_17ed3c: {
                                    'b_17ec64: {
                                        'b_17eb8c: {
                                            'b_17eb78: {
                                                'b_17eaa0: {
                                                    'b_17e9c8: {
                                                        'b_17e998: {
                                                            'b_17e94c: {
                                                                'b_17e93c: {
                                                                    'b_17e928: {
                                                                        'b_17e8c8: {
                                                                            'b_17e868: {
                                                                                'b_17e808: {
                                                                                    'b_17e7a8: {
                                                                                        'b_17e748: {
                                                                                            'b_17e6e8: {
                                                                                                'b_17e6b0: {
                                                                                                    'b_17e688: {
                                                                                                        c.cycles += 7;
                                                                                                        c.g[V0][0] = 0x460000;
                                                                                                        c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                                                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2bb0));
                                                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                                                                        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                                                                        let t = c.g[A0][0] as u32;
                                                                                                        match t {
0x0017e5c8 => {
if event_due(c) { deliver_event_here(c, 0x0017e5c8); } continue 'l_17e5c8;
}
0x0017e628 => {
c.cycles += 24;
c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
c.g[A1][0] = 0x460000;
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2da8));
c.g[AT][0] = 0x3fc00000;
c.mtc1(1, 2);
c.g[A0][0] = c.g[S1][0];
c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5b10));
c.mul_s(0, 0, 2);
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 4);
c.mul_s(1, 1, 2);
c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5b0c));
c.g[AT][0] = 0x43a00000;
c.mtc1(1, 12);
c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x38));
c.swc1(4, c.g[SP][0] as u32);
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x4));
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xc));
c.g[AT][0] = 0x43480000;
c.mtc1(1, 13);
let t = true;
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x8));
if t {
break 'b_17e998;
} else {
break 'b_17e688;
}
}
0x0017e688 => {
break 'b_17e688;
}
0x0017e6b0 => {
break 'b_17e6b0;
}
0x0017e6e8 => {
break 'b_17e6e8;
}
0x0017e748 => {
break 'b_17e748;
}
0x0017e7a8 => {
break 'b_17e7a8;
}
0x0017e808 => {
break 'b_17e808;
}
0x0017e868 => {
break 'b_17e868;
}
0x0017e8c8 => {
break 'b_17e8c8;
}
0x0017e928 => {
break 'b_17e928;
}
0x0017e93c => {
break 'b_17e93c;
}
0x0017e9c8 => {
break 'b_17e9c8;
}
0x0017eaa0 => {
break 'b_17eaa0;
}
0x0017eb78 => {
break 'b_17eb78;
}
0x0017eb8c => {
break 'b_17eb8c;
}
0x0017ec64 => {
break 'b_17ec64;
}
0x0017ed3c => {
break 'b_17ed3c;
}
0x0017ee14 => {
break 'b_17ee14;
}
0x0017ef20 => {
break 'b_17ef20;
}
0x0017f1a8 => {
break 'b_17f1a8;
}
_ => { c.jump_out(t, 0x0017e5c8, 0x0017f2cc); return Ok(()); }
}
                                                                                                    }
                                                                                                    c.cycles += 10;
                                                                                                    c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
                                                                                                    c.g[A1][0] = 0x460000;
                                                                                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                                                                                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2d90));
                                                                                                    c.g[AT][0] = 0x3fc00000;
                                                                                                    c.mtc1(1, 2);
                                                                                                    c.g[A0][0] = c.g[S1][0];
                                                                                                    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5b08));
                                                                                                    c.mul_s(0, 0, 2);
                                                                                                    c.g[AT][0] = 0x3f800000;
                                                                                                    break 'b_17e6b0;
                                                                                                }
                                                                                                c.cycles += 14;
                                                                                                c.mtc1(1, 4);
                                                                                                c.mul_s(1, 1, 2);
                                                                                                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5b04));
                                                                                                c.g[AT][0] = 0x43a00000;
                                                                                                c.mtc1(1, 12);
                                                                                                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                                                                                                c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x38));
                                                                                                c.swc1(4, c.g[SP][0] as u32);
                                                                                                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x4));
                                                                                                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xc));
                                                                                                c.g[AT][0] = 0x43480000;
                                                                                                c.mtc1(1, 13);
                                                                                                let t = true;
                                                                                                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x8));
                                                                                                if t
                                                                                                {
                                                                                                    break 'b_17e998;
                                                                                                } else {
                                                                                                    break 'b_17e6e8;
                                                                                                }
                                                                                            }
                                                                                            c.cycles += 24;
                                                                                            c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
                                                                                            c.g[A1][0] = 0x460000;
                                                                                            c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                                                                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2d78));
                                                                                            c.g[AT][0] = 0x3fc00000;
                                                                                            c.mtc1(
                                                                                                1,
                                                                                                2,
                                                                                            );
                                                                                            c.g[A0][0] = c.g[S1][0];
                                                                                            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5b00));
                                                                                            c.mul_s(0, 0, 2);
                                                                                            c.g[AT][0] = 0x3f800000;
                                                                                            c.mtc1(
                                                                                                1,
                                                                                                4,
                                                                                            );
                                                                                            c.mul_s(1, 1, 2);
                                                                                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5afc));
                                                                                            c.g[AT][0] = 0x43a00000;
                                                                                            c.mtc1(
                                                                                                1,
                                                                                                12,
                                                                                            );
                                                                                            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                                                                                            c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x38));
                                                                                            c.swc1(4, c.g[SP][0] as u32);
                                                                                            c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x4));
                                                                                            c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xc));
                                                                                            c.g[AT][0] = 0x43480000;
                                                                                            c.mtc1(
                                                                                                1,
                                                                                                13,
                                                                                            );
                                                                                            let t =
                                                                                                true;
                                                                                            c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x8));
                                                                                            if t {
                                                                                                break 'b_17e998;
                                                                                            } else {
                                                                                                break 'b_17e748;
                                                                                            }
                                                                                        }
                                                                                        c.cycles +=
                                                                                            24;
                                                                                        c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
                                                                                        c.g[A1][0] = 0x460000;
                                                                                        c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                                                                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2d60));
                                                                                        c.g[AT][0] = 0x3fc00000;
                                                                                        c.mtc1(
                                                                                            1, 2,
                                                                                        );
                                                                                        c.g[A0]
                                                                                            [0] = c
                                                                                            .g[S1]
                                                                                            [0];
                                                                                        c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5af8));
                                                                                        c.mul_s(
                                                                                            0, 0, 2,
                                                                                        );
                                                                                        c.g[AT][0] = 0x3f800000;
                                                                                        c.mtc1(
                                                                                            1, 4,
                                                                                        );
                                                                                        c.mul_s(
                                                                                            1, 1, 2,
                                                                                        );
                                                                                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5af4));
                                                                                        c.g[AT][0] = 0x43a00000;
                                                                                        c.mtc1(
                                                                                            1, 12,
                                                                                        );
                                                                                        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                                                                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x38));
                                                                                        c.swc1(4, c.g[SP][0] as u32);
                                                                                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x4));
                                                                                        c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xc));
                                                                                        c.g[AT][0] = 0x43480000;
                                                                                        c.mtc1(
                                                                                            1, 13,
                                                                                        );
                                                                                        let t =
                                                                                            true;
                                                                                        c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x8));
                                                                                        if t {
                                                                                            break 'b_17e998;
                                                                                        } else {
                                                                                            break 'b_17e7a8;
                                                                                        }
                                                                                    }
                                                                                    c.cycles += 24;
                                                                                    c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
                                                                                    c.g[A1][0] =
                                                                                        0x460000;
                                                                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                                                                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2d48));
                                                                                    c.g[AT][0] =
                                                                                        0x3fc00000;
                                                                                    c.mtc1(1, 2);
                                                                                    c.g[A0][0] =
                                                                                        c.g[S1][0];
                                                                                    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5af0));
                                                                                    c.mul_s(
                                                                                        0, 0, 2,
                                                                                    );
                                                                                    c.g[AT][0] =
                                                                                        0x3f800000;
                                                                                    c.mtc1(1, 4);
                                                                                    c.mul_s(
                                                                                        1, 1, 2,
                                                                                    );
                                                                                    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5aec));
                                                                                    c.g[AT][0] =
                                                                                        0x43a00000;
                                                                                    c.mtc1(1, 12);
                                                                                    c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                                                                                    c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x38));
                                                                                    c.swc1(
                                                                                        4,
                                                                                        c.g[SP][0]
                                                                                            as u32,
                                                                                    );
                                                                                    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x4));
                                                                                    c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xc));
                                                                                    c.g[AT][0] =
                                                                                        0x43480000;
                                                                                    c.mtc1(1, 13);
                                                                                    let t = true;
                                                                                    c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x8));
                                                                                    if t {
                                                                                        break 'b_17e998;
                                                                                    } else {
                                                                                        break 'b_17e808;
                                                                                    }
                                                                                }
                                                                                c.cycles += 24;
                                                                                c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
                                                                                c.g[A1][0] =
                                                                                    0x460000;
                                                                                c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                                                                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2d30));
                                                                                c.g[AT][0] =
                                                                                    0x3fc00000;
                                                                                c.mtc1(1, 2);
                                                                                c.g[A0][0] =
                                                                                    c.g[S1][0];
                                                                                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5ae8));
                                                                                c.mul_s(0, 0, 2);
                                                                                c.g[AT][0] =
                                                                                    0x3f800000;
                                                                                c.mtc1(1, 4);
                                                                                c.mul_s(1, 1, 2);
                                                                                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5ae4));
                                                                                c.g[AT][0] =
                                                                                    0x43a00000;
                                                                                c.mtc1(1, 12);
                                                                                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                                                                                c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x38));
                                                                                c.swc1(
                                                                                    4,
                                                                                    c.g[SP][0]
                                                                                        as u32,
                                                                                );
                                                                                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x4));
                                                                                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xc));
                                                                                c.g[AT][0] =
                                                                                    0x43480000;
                                                                                c.mtc1(1, 13);
                                                                                let t = true;
                                                                                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x8));
                                                                                if t {
                                                                                    break 'b_17e998;
                                                                                } else {
                                                                                    break 'b_17e868;
                                                                                }
                                                                            }
                                                                            c.cycles += 24;
                                                                            c.lwc1(
                                                                                1,
                                                                                (c.g[S1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x30,
                                                                                    ),
                                                                            );
                                                                            c.g[A1][0] = 0x460000;
                                                                            c.lwc1(
                                                                                0,
                                                                                (c.g[S1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x34,
                                                                                    ),
                                                                            );
                                                                            c.g[A1][0] =
                                                                                sx((c.g[A1][0]
                                                                                    as u32)
                                                                                    .wrapping_sub(
                                                                                        0x2d18,
                                                                                    ));
                                                                            c.g[AT][0] = 0x3fc00000;
                                                                            c.mtc1(1, 2);
                                                                            c.g[A0][0] = c.g[S1][0];
                                                                            c.lwc1(
                                                                                5,
                                                                                (c.g[GP][0] as u32)
                                                                                    .wrapping_sub(
                                                                                        0x5ae0,
                                                                                    ),
                                                                            );
                                                                            c.mul_s(0, 0, 2);
                                                                            c.g[AT][0] = 0x3f800000;
                                                                            c.mtc1(1, 4);
                                                                            c.mul_s(1, 1, 2);
                                                                            c.lwc1(
                                                                                3,
                                                                                (c.g[GP][0] as u32)
                                                                                    .wrapping_sub(
                                                                                        0x5adc,
                                                                                    ),
                                                                            );
                                                                            c.g[AT][0] = 0x43a00000;
                                                                            c.mtc1(1, 12);
                                                                            c.swc1(
                                                                                0,
                                                                                (c.g[S1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x3c,
                                                                                    ),
                                                                            );
                                                                            c.swc1(
                                                                                1,
                                                                                (c.g[S1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x38,
                                                                                    ),
                                                                            );
                                                                            c.swc1(
                                                                                4,
                                                                                c.g[SP][0] as u32,
                                                                            );
                                                                            c.swc1(
                                                                                3,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x4,
                                                                                    ),
                                                                            );
                                                                            c.swc1(
                                                                                5,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xc,
                                                                                    ),
                                                                            );
                                                                            c.g[AT][0] = 0x43480000;
                                                                            c.mtc1(1, 13);
                                                                            let t = true;
                                                                            c.swc1(
                                                                                5,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x8,
                                                                                    ),
                                                                            );
                                                                            if t {
                                                                                break 'b_17e998;
                                                                            } else {
                                                                                break 'b_17e8c8;
                                                                            }
                                                                        }
                                                                        c.cycles += 24;
                                                                        c.lwc1(
                                                                            1,
                                                                            (c.g[S1][0] as u32)
                                                                                .wrapping_add(0x30),
                                                                        );
                                                                        c.g[A1][0] = 0x460000;
                                                                        c.lwc1(
                                                                            0,
                                                                            (c.g[S1][0] as u32)
                                                                                .wrapping_add(0x34),
                                                                        );
                                                                        c.g[A1][0] =
                                                                            sx((c.g[A1][0] as u32)
                                                                                .wrapping_sub(
                                                                                    0x2d00,
                                                                                ));
                                                                        c.g[AT][0] = 0x3fc00000;
                                                                        c.mtc1(1, 2);
                                                                        c.g[A0][0] = c.g[S1][0];
                                                                        c.lwc1(
                                                                            5,
                                                                            (c.g[GP][0] as u32)
                                                                                .wrapping_sub(
                                                                                    0x5ad8,
                                                                                ),
                                                                        );
                                                                        c.mul_s(0, 0, 2);
                                                                        c.g[AT][0] = 0x3f800000;
                                                                        c.mtc1(1, 4);
                                                                        c.mul_s(1, 1, 2);
                                                                        c.lwc1(
                                                                            3,
                                                                            (c.g[GP][0] as u32)
                                                                                .wrapping_sub(
                                                                                    0x5ad4,
                                                                                ),
                                                                        );
                                                                        c.g[AT][0] = 0x43a00000;
                                                                        c.mtc1(1, 12);
                                                                        c.swc1(
                                                                            0,
                                                                            (c.g[S1][0] as u32)
                                                                                .wrapping_add(0x3c),
                                                                        );
                                                                        c.swc1(
                                                                            1,
                                                                            (c.g[S1][0] as u32)
                                                                                .wrapping_add(0x38),
                                                                        );
                                                                        c.swc1(
                                                                            4,
                                                                            c.g[SP][0] as u32,
                                                                        );
                                                                        c.swc1(
                                                                            3,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x4),
                                                                        );
                                                                        c.swc1(
                                                                            5,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xc),
                                                                        );
                                                                        c.g[AT][0] = 0x43480000;
                                                                        c.mtc1(1, 13);
                                                                        let t = true;
                                                                        c.swc1(
                                                                            5,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x8),
                                                                        );
                                                                        if t {
                                                                            break 'b_17e998;
                                                                        } else {
                                                                            break 'b_17e928;
                                                                        }
                                                                    }
                                                                    c.cycles += 5;
                                                                    c.lwc1(
                                                                        1,
                                                                        (c.g[S1][0] as u32)
                                                                            .wrapping_add(0x30),
                                                                    );
                                                                    c.g[A1][0] = 0x460000;
                                                                    c.lwc1(
                                                                        0,
                                                                        (c.g[S1][0] as u32)
                                                                            .wrapping_add(0x34),
                                                                    );
                                                                    let t = true;
                                                                    c.g[A1][0] = sx((c.g[A1][0]
                                                                        as u32)
                                                                        .wrapping_sub(0x2cf0));
                                                                    if t {
                                                                        break 'b_17e94c;
                                                                    } else {
                                                                        break 'b_17e93c;
                                                                    }
                                                                }
                                                                c.cycles += 4;
                                                                c.lwc1(
                                                                    1,
                                                                    (c.g[S1][0] as u32)
                                                                        .wrapping_add(0x30),
                                                                );
                                                                c.g[A1][0] = 0x460000;
                                                                c.lwc1(
                                                                    0,
                                                                    (c.g[S1][0] as u32)
                                                                        .wrapping_add(0x34),
                                                                );
                                                                c.g[A1][0] = sx((c.g[A1][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x2cd8));
                                                                break 'b_17e94c;
                                                            }
                                                            c.cycles += 19;
                                                            c.g[AT][0] = 0x3fc00000;
                                                            c.mtc1(1, 2);
                                                            c.g[A0][0] = c.g[S1][0];
                                                            c.lwc1(
                                                                5,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x5ad0),
                                                            );
                                                            c.mul_s(0, 0, 2);
                                                            c.g[AT][0] = 0x3f800000;
                                                            c.mtc1(1, 4);
                                                            c.mul_s(1, 1, 2);
                                                            c.lwc1(
                                                                3,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x5acc),
                                                            );
                                                            c.g[AT][0] = 0x43a00000;
                                                            c.mtc1(1, 12);
                                                            c.swc1(
                                                                0,
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x3c),
                                                            );
                                                            c.swc1(
                                                                1,
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x38),
                                                            );
                                                            c.swc1(4, c.g[SP][0] as u32);
                                                            c.swc1(
                                                                3,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.swc1(
                                                                5,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc),
                                                            );
                                                            c.swc1(
                                                                5,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[AT][0] = 0x42c80000;
                                                            c.mtc1(1, 13);
                                                            break 'b_17e998;
                                                        }
                                                        c.cycles += 12;
                                                        c.ldl(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x7),
                                                        );
                                                        c.ldr(2, c.g[SP][0] as u32);
                                                        c.ldl(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xf),
                                                        );
                                                        c.ldr(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x8),
                                                        );
                                                        c.sdl(
                                                            2,
                                                            (c.g[S1][0] as u32).wrapping_add(0x47),
                                                        );
                                                        c.sdr(
                                                            2,
                                                            (c.g[S1][0] as u32).wrapping_add(0x40),
                                                        );
                                                        c.sdl(
                                                            3,
                                                            (c.g[S1][0] as u32).wrapping_add(0x4f),
                                                        );
                                                        c.sdr(
                                                            3,
                                                            (c.g[S1][0] as u32).wrapping_add(0x48),
                                                        );
                                                        c.g[RA][0] = sx(0x0017e9c0);
                                                        call_ptr(c, 0x00392908, 0x0017e9c0)?;
                                                        let t = true;

                                                        if t {
                                                            break 'b_17f1a8;
                                                        } else {
                                                            break 'b_17e9c8;
                                                        }
                                                    }
                                                    c.cycles += 54;
                                                    c.lwc1(
                                                        1,
                                                        (c.g[S1][0] as u32).wrapping_add(0x30),
                                                    );
                                                    c.g[A1][0] = 0x460000;
                                                    c.lwc1(
                                                        0,
                                                        (c.g[S1][0] as u32).wrapping_add(0x34),
                                                    );
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32)
                                                            .wrapping_sub(0x2cc0));
                                                    c.g[AT][0] = 0x3fc00000;
                                                    c.mtc1(1, 2);
                                                    c.g[A0][0] = c.g[S1][0];
                                                    c.lwc1(
                                                        3,
                                                        (c.g[GP][0] as u32).wrapping_sub(0x5ac8),
                                                    );
                                                    c.mul_s(0, 0, 2);
                                                    c.lwc1(
                                                        20,
                                                        (c.g[GP][0] as u32).wrapping_sub(0x5ac4),
                                                    );
                                                    c.mul_s(1, 1, 2);
                                                    c.g[AT][0] = 0x3f800000;
                                                    c.mtc1(1, 21);
                                                    c.g[AT][0] = 0x43a00000;
                                                    c.mtc1(1, 22);
                                                    c.swc1(
                                                        0,
                                                        (c.g[S1][0] as u32).wrapping_add(0x3c),
                                                    );
                                                    c.swc1(
                                                        1,
                                                        (c.g[S1][0] as u32).wrapping_add(0x38),
                                                    );
                                                    c.swc1(
                                                        3,
                                                        (c.g[SP][0] as u32).wrapping_add(0xc),
                                                    );
                                                    c.swc1(21, c.g[SP][0] as u32);
                                                    c.swc1(
                                                        20,
                                                        (c.g[SP][0] as u32).wrapping_add(0x4),
                                                    );
                                                    c.swc1(
                                                        3,
                                                        (c.g[SP][0] as u32).wrapping_add(0x8),
                                                    );
                                                    c.g[AT][0] = 0x42c80000;
                                                    c.mtc1(1, 13);
                                                    c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                                                    c.ldr(2, c.g[SP][0] as u32);
                                                    c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
                                                    c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                                                    c.sdl(
                                                        2,
                                                        (c.g[S1][0] as u32).wrapping_add(0x47),
                                                    );
                                                    c.sdr(
                                                        2,
                                                        (c.g[S1][0] as u32).wrapping_add(0x40),
                                                    );
                                                    c.sdl(
                                                        3,
                                                        (c.g[S1][0] as u32).wrapping_add(0x4f),
                                                    );
                                                    c.sdr(
                                                        3,
                                                        (c.g[S1][0] as u32).wrapping_add(0x48),
                                                    );
                                                    c.g[RA][0] = sx(0x0017ea4c);
                                                    c.mov_s(12, 22);
                                                    call_ptr(c, 0x00392908, 0x0017ea4c)?;
                                                    c.lwc1(
                                                        2,
                                                        (c.g[S1][0] as u32).wrapping_add(0x30),
                                                    );
                                                    c.lwc1(
                                                        0,
                                                        (c.g[S1][0] as u32).wrapping_add(0x34),
                                                    );
                                                    c.lwc1(
                                                        1,
                                                        (c.g[GP][0] as u32).wrapping_sub(0x5ac0),
                                                    );
                                                    c.mul_s(0, 0, 1);
                                                    c.mul_s(2, 2, 1);
                                                    c.swc1(
                                                        0,
                                                        (c.g[S1][0] as u32).wrapping_add(0x3c),
                                                    );
                                                    c.swc1(
                                                        2,
                                                        (c.g[S1][0] as u32).wrapping_add(0x38),
                                                    );
                                                    c.swc1(21, c.g[SP][0] as u32);
                                                    c.swc1(
                                                        20,
                                                        (c.g[SP][0] as u32).wrapping_add(0xc),
                                                    );
                                                    c.swc1(
                                                        20,
                                                        (c.g[SP][0] as u32).wrapping_add(0x4),
                                                    );
                                                    c.swc1(
                                                        20,
                                                        (c.g[SP][0] as u32).wrapping_add(0x8),
                                                    );
                                                    c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                                                    c.ldr(2, c.g[SP][0] as u32);
                                                    c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
                                                    c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                                                    c.sdl(
                                                        2,
                                                        (c.g[S1][0] as u32).wrapping_add(0x47),
                                                    );
                                                    c.sdr(
                                                        2,
                                                        (c.g[S1][0] as u32).wrapping_add(0x40),
                                                    );
                                                    c.sdl(
                                                        3,
                                                        (c.g[S1][0] as u32).wrapping_add(0x4f),
                                                    );
                                                    c.sdr(
                                                        3,
                                                        (c.g[S1][0] as u32).wrapping_add(0x48),
                                                    );
                                                    let t = true;
                                                    c.g[A0][0] = c.g[SP][0];
                                                    if t {
                                                        break 'b_17eee8;
                                                    } else {
                                                        break 'b_17eaa0;
                                                    }
                                                }
                                                c.cycles += 54;
                                                c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
                                                c.g[A1][0] = 0x460000;
                                                c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_sub(0x2ca8));
                                                c.g[AT][0] = 0x3fc00000;
                                                c.mtc1(1, 2);
                                                c.g[A0][0] = c.g[S1][0];
                                                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5abc));
                                                c.mul_s(0, 0, 2);
                                                c.lwc1(
                                                    20,
                                                    (c.g[GP][0] as u32).wrapping_sub(0x5ab8),
                                                );
                                                c.mul_s(1, 1, 2);
                                                c.g[AT][0] = 0x3f800000;
                                                c.mtc1(1, 21);
                                                c.g[AT][0] = 0x43a00000;
                                                c.mtc1(1, 22);
                                                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                                                c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x38));
                                                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xc));
                                                c.swc1(21, c.g[SP][0] as u32);
                                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
                                                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                                                c.g[AT][0] = 0x42c80000;
                                                c.mtc1(1, 13);
                                                c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
                                                c.ldr(7, c.g[SP][0] as u32);
                                                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0xf));
                                                c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x8));
                                                c.sdl(7, (c.g[S1][0] as u32).wrapping_add(0x47));
                                                c.sdr(7, (c.g[S1][0] as u32).wrapping_add(0x40));
                                                c.sdl(2, (c.g[S1][0] as u32).wrapping_add(0x4f));
                                                c.sdr(2, (c.g[S1][0] as u32).wrapping_add(0x48));
                                                c.g[RA][0] = sx(0x0017eb24);
                                                c.mov_s(12, 22);
                                                call_ptr(c, 0x00392908, 0x0017eb24)?;
                                                c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x30));
                                                c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                                                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5ab4));
                                                c.mul_s(0, 0, 1);
                                                c.mul_s(2, 2, 1);
                                                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                                                c.swc1(2, (c.g[S1][0] as u32).wrapping_add(0x38));
                                                c.swc1(21, c.g[SP][0] as u32);
                                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc));
                                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
                                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x8));
                                                c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
                                                c.ldr(7, c.g[SP][0] as u32);
                                                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0xf));
                                                c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x8));
                                                c.sdl(7, (c.g[S1][0] as u32).wrapping_add(0x47));
                                                c.sdr(7, (c.g[S1][0] as u32).wrapping_add(0x40));
                                                c.sdl(2, (c.g[S1][0] as u32).wrapping_add(0x4f));
                                                c.sdr(2, (c.g[S1][0] as u32).wrapping_add(0x48));
                                                let t = true;
                                                c.g[A0][0] = c.g[SP][0];
                                                if t {
                                                    break 'b_17eee8;
                                                } else {
                                                    break 'b_17eb78;
                                                }
                                            }
                                            c.cycles += 5;
                                            c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
                                            c.g[A1][0] = 0x460000;
                                            c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                                            let t = true;
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_sub(0x2c88));
                                            if t {
                                                break 'b_17ee24;
                                            } else {
                                                break 'b_17eb8c;
                                            }
                                        }
                                        c.cycles += 54;
                                        c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
                                        c.g[A1][0] = 0x460000;
                                        c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2c70));
                                        c.g[AT][0] = 0x3fc00000;
                                        c.mtc1(1, 2);
                                        c.g[A0][0] = c.g[S1][0];
                                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5ab0));
                                        c.mul_s(0, 0, 2);
                                        c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x5aac));
                                        c.mul_s(1, 1, 2);
                                        c.g[AT][0] = 0x3f800000;
                                        c.mtc1(1, 21);
                                        c.g[AT][0] = 0x43a00000;
                                        c.mtc1(1, 22);
                                        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x38));
                                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xc));
                                        c.swc1(21, c.g[SP][0] as u32);
                                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
                                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                                        c.g[AT][0] = 0x42c80000;
                                        c.mtc1(1, 13);
                                        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                                        c.ldr(3, c.g[SP][0] as u32);
                                        c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0xf));
                                        c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x8));
                                        c.sdl(3, (c.g[S1][0] as u32).wrapping_add(0x47));
                                        c.sdr(3, (c.g[S1][0] as u32).wrapping_add(0x40));
                                        c.sdl(6, (c.g[S1][0] as u32).wrapping_add(0x4f));
                                        c.sdr(6, (c.g[S1][0] as u32).wrapping_add(0x48));
                                        c.g[RA][0] = sx(0x0017ec10);
                                        c.mov_s(12, 22);
                                        call_ptr(c, 0x00392908, 0x0017ec10)?;
                                        c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x30));
                                        c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                                        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5aa8));
                                        c.mul_s(0, 0, 1);
                                        c.mul_s(2, 2, 1);
                                        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                                        c.swc1(2, (c.g[S1][0] as u32).wrapping_add(0x38));
                                        c.swc1(21, c.g[SP][0] as u32);
                                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc));
                                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
                                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x8));
                                        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                                        c.ldr(3, c.g[SP][0] as u32);
                                        c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0xf));
                                        c.ldr(5, (c.g[SP][0] as u32).wrapping_add(0x8));
                                        c.sdl(3, (c.g[S1][0] as u32).wrapping_add(0x47));
                                        c.sdr(3, (c.g[S1][0] as u32).wrapping_add(0x40));
                                        c.sdl(5, (c.g[S1][0] as u32).wrapping_add(0x4f));
                                        c.sdr(5, (c.g[S1][0] as u32).wrapping_add(0x48));
                                        let t = true;
                                        c.g[A0][0] = c.g[SP][0];
                                        if t {
                                            break 'b_17eee8;
                                        } else {
                                            break 'b_17ec64;
                                        }
                                    }
                                    c.cycles += 54;
                                    c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
                                    c.g[A1][0] = 0x460000;
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2c60));
                                    c.g[AT][0] = 0x3fc00000;
                                    c.mtc1(1, 2);
                                    c.g[A0][0] = c.g[S1][0];
                                    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5aa4));
                                    c.mul_s(0, 0, 2);
                                    c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x5aa0));
                                    c.mul_s(1, 1, 2);
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 21);
                                    c.g[AT][0] = 0x43a00000;
                                    c.mtc1(1, 22);
                                    c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                                    c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x38));
                                    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xc));
                                    c.swc1(21, c.g[SP][0] as u32);
                                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
                                    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                                    c.g[AT][0] = 0x42c80000;
                                    c.mtc1(1, 13);
                                    c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                                    c.ldr(2, c.g[SP][0] as u32);
                                    c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
                                    c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                                    c.sdl(2, (c.g[S1][0] as u32).wrapping_add(0x47));
                                    c.sdr(2, (c.g[S1][0] as u32).wrapping_add(0x40));
                                    c.sdl(3, (c.g[S1][0] as u32).wrapping_add(0x4f));
                                    c.sdr(3, (c.g[S1][0] as u32).wrapping_add(0x48));
                                    c.g[RA][0] = sx(0x0017ece8);
                                    c.mov_s(12, 22);
                                    call_ptr(c, 0x00392908, 0x0017ece8)?;
                                    c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x30));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5a9c));
                                    c.mul_s(0, 0, 1);
                                    c.mul_s(2, 2, 1);
                                    c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                                    c.swc1(2, (c.g[S1][0] as u32).wrapping_add(0x38));
                                    c.swc1(21, c.g[SP][0] as u32);
                                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc));
                                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
                                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x8));
                                    c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                                    c.ldr(2, c.g[SP][0] as u32);
                                    c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
                                    c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                                    c.sdl(2, (c.g[S1][0] as u32).wrapping_add(0x47));
                                    c.sdr(2, (c.g[S1][0] as u32).wrapping_add(0x40));
                                    c.sdl(3, (c.g[S1][0] as u32).wrapping_add(0x4f));
                                    c.sdr(3, (c.g[S1][0] as u32).wrapping_add(0x48));
                                    let t = true;
                                    c.g[A0][0] = c.g[SP][0];
                                    if t {
                                        break 'b_17eee8;
                                    } else {
                                        break 'b_17ed3c;
                                    }
                                }
                                c.cycles += 54;
                                c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
                                c.g[A1][0] = 0x460000;
                                c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2c40));
                                c.g[AT][0] = 0x3fc00000;
                                c.mtc1(1, 2);
                                c.g[A0][0] = c.g[S1][0];
                                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5a98));
                                c.mul_s(0, 0, 2);
                                c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x5a94));
                                c.mul_s(1, 1, 2);
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 21);
                                c.g[AT][0] = 0x43a00000;
                                c.mtc1(1, 22);
                                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                                c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x38));
                                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xc));
                                c.swc1(21, c.g[SP][0] as u32);
                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
                                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                                c.g[AT][0] = 0x42c80000;
                                c.mtc1(1, 13);
                                c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
                                c.ldr(7, c.g[SP][0] as u32);
                                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0xf));
                                c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x8));
                                c.sdl(7, (c.g[S1][0] as u32).wrapping_add(0x47));
                                c.sdr(7, (c.g[S1][0] as u32).wrapping_add(0x40));
                                c.sdl(2, (c.g[S1][0] as u32).wrapping_add(0x4f));
                                c.sdr(2, (c.g[S1][0] as u32).wrapping_add(0x48));
                                c.g[RA][0] = sx(0x0017edc0);
                                c.mov_s(12, 22);
                                call_ptr(c, 0x00392908, 0x0017edc0)?;
                                c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x30));
                                c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5a90));
                                c.mul_s(0, 0, 1);
                                c.mul_s(2, 2, 1);
                                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                                c.swc1(2, (c.g[S1][0] as u32).wrapping_add(0x38));
                                c.swc1(21, c.g[SP][0] as u32);
                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc));
                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x8));
                                c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
                                c.ldr(7, c.g[SP][0] as u32);
                                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0xf));
                                c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x8));
                                c.sdl(7, (c.g[S1][0] as u32).wrapping_add(0x47));
                                c.sdr(7, (c.g[S1][0] as u32).wrapping_add(0x40));
                                c.sdl(2, (c.g[S1][0] as u32).wrapping_add(0x4f));
                                c.sdr(2, (c.g[S1][0] as u32).wrapping_add(0x48));
                                let t = true;
                                c.g[A0][0] = c.g[SP][0];
                                if t {
                                    break 'b_17eee8;
                                } else {
                                    break 'b_17ee14;
                                }
                            }
                            c.cycles += 4;
                            c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
                            c.g[A1][0] = 0x460000;
                            c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2c28));
                            break 'b_17ee24;
                        }
                        c.cycles += 49;
                        c.g[AT][0] = 0x3fc00000;
                        c.mtc1(1, 2);
                        c.g[A0][0] = c.g[S1][0];
                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5a8c));
                        c.mul_s(0, 0, 2);
                        c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x5a88));
                        c.mul_s(1, 1, 2);
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 21);
                        c.g[AT][0] = 0x43a00000;
                        c.mtc1(1, 22);
                        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x38));
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xc));
                        c.swc1(21, c.g[SP][0] as u32);
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.g[AT][0] = 0x42c80000;
                        c.mtc1(1, 13);
                        c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.ldr(6, c.g[SP][0] as u32);
                        c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0xf));
                        c.ldr(7, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.sdl(6, (c.g[S1][0] as u32).wrapping_add(0x47));
                        c.sdr(6, (c.g[S1][0] as u32).wrapping_add(0x40));
                        c.sdl(7, (c.g[S1][0] as u32).wrapping_add(0x4f));
                        c.sdr(7, (c.g[S1][0] as u32).wrapping_add(0x48));
                        c.g[RA][0] = sx(0x0017ee98);
                        c.mov_s(12, 22);
                        call_ptr(c, 0x00392908, 0x0017ee98)?;
                        c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x30));
                        c.g[A0][0] = c.g[SP][0];
                        c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5a84));
                        c.mul_s(0, 0, 1);
                        c.mul_s(2, 2, 1);
                        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                        c.swc1(2, (c.g[S1][0] as u32).wrapping_add(0x38));
                        c.swc1(21, c.g[SP][0] as u32);
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc));
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.ldr(5, c.g[SP][0] as u32);
                        c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0xf));
                        c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.sdl(5, (c.g[S1][0] as u32).wrapping_add(0x47));
                        c.sdr(5, (c.g[S1][0] as u32).wrapping_add(0x40));
                        c.sdl(6, (c.g[S1][0] as u32).wrapping_add(0x4f));
                        c.sdr(6, (c.g[S1][0] as u32).wrapping_add(0x48));
                        break 'b_17eee8;
                    }
                    c.cycles += 14;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                    c.g[RA][0] = sx(0x0017eef4);
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c14));
                    call_ptr(c, 0x00260e50, 0x0017eef4)?;
                    c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[AT][0] = 0x43160000;
                    c.mtc1(1, 13);
                    c.g[RA][0] = sx(0x0017ef0c);
                    c.mov_s(12, 22);
                    call_ptr(c, 0x00392908, 0x0017ef0c)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x0017ef18);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00318260, 0x0017ef18)?;
                    let t = true;

                    if t {
                        break 'b_17f1a8;
                    } else {
                        break 'b_17ef20;
                    }
                }
                'b_17f0ac: {
                    'b_17f0a4: {
                        c.cycles += 95;
                        c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
                        c.g[A1][0] = 0x460000;
                        c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2c08));
                        c.g[AT][0] = 0x3fc00000;
                        c.mtc1(1, 2);
                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5a80));
                        c.mul_s(0, 0, 2);
                        c.lwc1(21, (c.g[GP][0] as u32).wrapping_sub(0x5a7c));
                        c.mul_s(1, 1, 2);
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 23);
                        c.g[AT][0] = 0x43a00000;
                        c.mtc1(1, 12);
                        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x38));
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xc));
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.swc1(23, c.g[SP][0] as u32);
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x4));
                        c.g[AT][0] = 0x42c80000;
                        c.mtc1(1, 13);
                        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.ldr(3, c.g[SP][0] as u32);
                        c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0xf));
                        c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.sdl(3, (c.g[S1][0] as u32).wrapping_add(0x47));
                        c.sdr(3, (c.g[S1][0] as u32).wrapping_add(0x40));
                        c.sdl(6, (c.g[S1][0] as u32).wrapping_add(0x4f));
                        c.sdr(6, (c.g[S1][0] as u32).wrapping_add(0x48));
                        c.g[RA][0] = sx(0x0017efa0);
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x00392908, 0x0017efa0)?;
                        c.g[AT][0] = 0x43480000;
                        c.mtc1(1, 24);
                        c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
                        c.g[A1][0] = 0x4a0000;
                        c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                        c.mov_s(13, 24);
                        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5a78));
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1328));
                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5a74));
                        c.g[A0][0] = c.g[S1][0];
                        c.mul_s(0, 0, 2);
                        c.g[AT][0] = 0x42dc0000;
                        c.mtc1(1, 20);
                        c.mul_s(1, 1, 2);
                        c.lwc1(22, (c.g[GP][0] as u32).wrapping_sub(0x5a70));
                        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x38));
                        c.swc1(23, c.g[SP][0] as u32);
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xc));
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x4));
                        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.ldr(3, c.g[SP][0] as u32);
                        c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0xf));
                        c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.sdl(3, (c.g[S1][0] as u32).wrapping_add(0x47));
                        c.sdr(3, (c.g[S1][0] as u32).wrapping_add(0x40));
                        c.sdl(6, (c.g[S1][0] as u32).wrapping_add(0x4f));
                        c.sdr(6, (c.g[S1][0] as u32).wrapping_add(0x48));
                        c.g[RA][0] = sx(0x0017f01c);
                        c.mov_s(12, 20);
                        call_ptr(c, 0x00391cb0, 0x0017f01c)?;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                        c.mov_s(13, 24);
                        c.g[A0][0] = c.g[S1][0];
                        c.mov_s(12, 22);
                        c.g[AT][0] = 0x435c0000;
                        c.mtc1(1, 24);
                        c.g[RA][0] = sx(0x0017f03c);
                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x88));
                        call_ptr(c, 0x00392a60, 0x0017f03c)?;
                        c.g[A1][0] = 0x460000;
                        c.mov_s(13, 24);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2bf8));
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x0017f054);
                        c.mov_s(12, 20);
                        call_ptr(c, 0x00391cb0, 0x0017f054)?;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                        c.mov_s(13, 24);
                        c.mov_s(12, 22);
                        c.g[A0][0] = c.g[S1][0];
                        c.g[AT][0] = 0x43700000;
                        c.mtc1(1, 24);
                        c.g[RA][0] = sx(0x0017f074);
                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x98));
                        call_ptr(c, 0x00392a60, 0x0017f074)?;
                        c.g[A1][0] = 0x460000;
                        c.mov_s(12, 20);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2be8));
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x0017f08c);
                        c.mov_s(13, 24);
                        call_ptr(c, 0x00391cb0, 0x0017f08c)?;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xb4));
                        let t = c.g[V1][0] == 0u64;
                        c.g[V0][0] = 0x4a0000;
                        if t {
                            break 'b_17f0a4;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12f8));
                            if t {
                                break 'b_17f0ac;
                            } else {
                                break 'b_17f0a4;
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[V0][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1330));
                    break 'b_17f0ac;
                }
                c.cycles += 63;
                c.g[AT][0] = 0x41a00000;
                c.mtc1(1, 20);
                c.g[A0][0] = c.g[S1][0];
                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x5a6c));
                c.g[AT][0] = 0x43700000;
                c.mtc1(1, 13);
                c.g[AT][0] = 0x42dc0000;
                c.mtc1(1, 21);
                c.g[RA][0] = sx(0x0017f0d4);
                c.add_s(24, 24, 20);
                call_ptr(c, 0x00392a60, 0x0017f0d4)?;
                c.lwc1(22, (c.g[GP][0] as u32).wrapping_sub(0x5a68));
                c.g[A1][0] = 0x460000;
                c.mov_s(13, 24);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2bd8));
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x0017f0f0);
                c.mov_s(12, 21);
                call_ptr(c, 0x00391cb0, 0x0017f0f0)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                c.g[S0][0] = 0x460000;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x3068));
                c.mov_s(13, 24);
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa8));
                c.add_s(24, 24, 20);
                c.g[T0][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0xa9));
                c.g[A1][0] = c.g[S0][0];
                c.g[A3][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0xaa));
                c.g[T1][0] = c.g[A2][0] & 0xff;
                c.g[A2][0] = sx((c.g[A2][0] as u32) >> 24);
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x0017f128);
                c.mov_s(12, 22);
                call_ptr(c, 0x001dca80, 0x0017f128)?;
                c.g[A1][0] = 0x460000;
                c.mov_s(13, 24);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2bc8));
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x0017f140);
                c.mov_s(12, 21);
                call_ptr(c, 0x00391cb0, 0x0017f140)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                c.mov_s(13, 24);
                c.g[A1][0] = c.g[S0][0];
                c.add_s(24, 24, 20);
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xac));
                c.g[A0][0] = c.g[S1][0];
                c.g[T0][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0xad));
                c.mov_s(12, 22);
                c.g[A3][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0xae));
                c.g[T1][0] = c.g[A2][0] & 0xff;
                c.g[RA][0] = sx(0x0017f170);
                c.g[A2][0] = sx((c.g[A2][0] as u32) >> 24);
                call_ptr(c, 0x001dca80, 0x0017f170)?;
                c.g[A1][0] = 0x4a0000;
                c.mov_s(12, 21);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1338));
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x0017f188);
                c.mov_s(13, 24);
                call_ptr(c, 0x00391cb0, 0x0017f188)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                c.g[A1][0] = 0x4a0000;
                c.mov_s(12, 22);
                c.mov_s(13, 24);
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xb0));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x12c0));
                c.g[RA][0] = sx(0x0017f1a8);
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x001dca80, 0x0017f1a8)?;
                break 'b_17f1a8;
            }
        }
        'b_17f1e4: {
            'b_17f1e0: {
                c.cycles += 10;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5a64));
                c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
                c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x34));
                c.mul_s(1, 1, 0);
                c.mul_s(2, 2, 0);
                c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x38));
                c.swc1(2, (c.g[S1][0] as u32).wrapping_add(0x3c));
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                if t {
                    break 'b_17f1e0;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    if t {
                        break 'b_17f1e4;
                    } else {
                        break 'b_17f1e0;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_17f1e4;
        }
        'b_17f1f8: {
            c.cycles += 2;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
            if t {
                break 'b_17f1f8;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x0017f1f4);
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8));
                call_ptr(c, 0x002cc350, 0x0017f1f4)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                break 'b_17f1f8;
            }
        }
        'b_17f2a4: {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x20));
            c.g[V0][0] = c.g[V0][0] ^ 0x5;
            let t = c.g[V0][0] != 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            if t {
                break 'b_17f2a4;
            } else {
                c.cycles += 39;
                c.g[RA][0] = sx(0x0017f210);
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                call_ptr(c, 0x00256038, 0x0017f210)?;
                c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
                c.g[A1][0] = 0x460000;
                c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                c.g[A0][0] = c.g[S1][0];
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5a60));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3068));
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5a5c));
                c.mul_s(0, 0, 2);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 3);
                c.mul_s(1, 1, 2);
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x5a58));
                c.g[AT][0] = 0x43a00000;
                c.mtc1(1, 12);
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x38));
                c.swc1(3, c.g[SP][0] as u32);
                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0xc));
                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(3, c.g[SP][0] as u32);
                c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0xf));
                c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(3, (c.g[S1][0] as u32).wrapping_add(0x47));
                c.sdr(3, (c.g[S1][0] as u32).wrapping_add(0x40));
                c.sdl(6, (c.g[S1][0] as u32).wrapping_add(0x4f));
                c.sdr(6, (c.g[S1][0] as u32).wrapping_add(0x48));
                c.g[AT][0] = 0x43d20000;
                c.mtc1(1, 13);
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A3][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x12));
                c.g[T1][0] = c.g[A2][0] & 0xff;
                c.g[T0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x11));
                c.g[RA][0] = sx(0x0017f2a0);
                c.g[A2][0] = sx((c.g[A2][0] as u32) >> 24);
                call_ptr(c, 0x001dca40, 0x0017f2a0)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_17f2a4;
            }
        }
        c.cycles += 10;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x78));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x68));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

/// `void DebugMenuModule_DrawCursorTrail(DebugMenuModule* self, short layer)`
/// (0x17f500): sets the 2D renderer's state words (blend and z flags in the
/// word at `+4` of the render state `*(*(gp-0x854)+0xe84)`, `layer` into its
/// halfword at `+0x10`), then draws the 64-point cursor trail stored in the
/// menu (`+0x10 + 8*i` = (x, y), ring head at `+0x210`) as 63 shrinking,
/// fading filled rectangles (callee 0x38ec40, Render2D_DrawFilledRect) and
/// a final box around the newest point. Registers and cycles follow the
/// generated body, so the body is kept instruction by instruction.
pub fn debug_menu_module_draw_cursor_trail(c: &mut Cpu) {
    guest(c, debug_menu_module_draw_cursor_trail_body);
}

pub(crate) fn debug_menu_module_draw_cursor_trail_body(c: &mut Cpu) -> Flow {
    c.cycles += 72;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x100));
    c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
    c.g[V1][0] = 0xfffffffffe7f0000;
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
    c.g[V1][0] = c.g[V1][0] | 0xffff;
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.g[T3][0] = 0x1000000;
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xa0));
    c.g[A2][0] = 0xffffffffffcf0000;
    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.g[A2][0] = c.g[A2][0] | 0xffff;
    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.g[T4][0] = 0x300000;
    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.g[A3][0] = 0xfffffffffff00000;
    c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0xf8));
    c.g[A3][0] = c.g[A3][0] | 0xfff;
    c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xf0));
    c.g[S0][0] = c.g[A0][0];
    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xe8));
    c.g[T5][0] = 0xffffffffffffff83;
    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
    c.g[T2][0] = 0xffffffffffbf0000;
    c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
    c.g[T2][0] = c.g[T2][0] | 0xffff;
    c.g[AT][0] = 0x40c00000;
    c.mtc1(1, 0);
    c.g[S1][0] = 0x1;
    c.g[T1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
    c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[AT][0] = 0x3f000000;
    c.mtc1(1, 20);
    c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x4));
    c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[AT][0] = 0x3c800000;
    c.mtc1(1, 23);
    c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
    c.lwc1(22, (c.g[GP][0] as u32).wrapping_sub(0x5a38));
    c.g[V0][0] = c.g[V0][0] | c.g[T3][0];
    c.lwc1(21, (c.g[GP][0] as u32).wrapping_sub(0x5a34));
    c.sw((c.g[T1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.g[T3][0] = 0x400000;
    c.g[S6][0] = 0xffffffffffffffff;
    c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
    c.g[V0][0] = c.g[V0][0] | c.g[T4][0];
    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
    c.g[V0][0] = c.g[V0][0] | 0xd000;
    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.g[V0][0] & c.g[T5][0];
    c.g[V0][0] = c.g[V0][0] | 0x14;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
    c.g[V0][0] = c.g[V0][0] | c.g[T3][0];
    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
    c.swc1(0, c.g[SP][0] as u32);
    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
    c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x210));
    'l_17f620: loop {
        c.cycles += 53;
        c.mtc1(17, 1);
        c.cvt_s_w(1, 1);
        c.lwc1(4, c.g[SP][0] as u32);
        c.g[A1][0] = c.g[S2][0];
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.lwc1(6, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x3f));
        c.g[V1][0] = ((c.g[S6][0] as i64) < (c.g[A3][0] as i64)) as u64;
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.g[A3][0];
        }
        c.mul_s(4, 4, 20);
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
        c.mul_s(6, 6, 20);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
        c.mul_s(1, 1, 23);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[A3][0] = sx((c.g[A3][0] as u32) << 3);
        c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x24));
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x10));
        c.mul_s(8, 1, 1);
        c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.mul_s(7, 1, 21);
        c.lwc1(3, c.g[A3][0] as u32);
        c.mul_s(9, 1, 20);
        c.lwc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
        c.mul_s(8, 8, 22);
        c.sub_s(5, 3, 4);
        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sub_s(0, 0, 6);
        c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x44));
        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5a30));
        c.add_s(3, 3, 4);
        c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.mul_s(1, 1, 9);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
        c.add_s(7, 7, 2);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x844));
        c.g[A2][0] = c.g[S3][0];
        c.lwc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
        c.mtc1(0, 12);
        c.g[A3][0] = c.g[S4][0];
        c.add_s(0, 0, 6);
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x58));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x34));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x5c));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x54));
        call_ptr(c, 0x0038ec40, 0x0017f6e8)?;
        c.g[V0][0] = ((c.g[S1][0] as i64) < 64i64) as u64;
        if c.g[V0][0] != 0u64 {
            c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x210));
            if event_due(c) {
                deliver_event_here(c, 0x0017f620);
            }
            continue 'l_17f620;
        } else {
            c.cycles += 48;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x210));
            c.g[A1][0] = c.g[S2][0];
            c.lwc1(5, c.g[SP][0] as u32);
            c.g[A2][0] = c.g[S5][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
            c.lwc1(6, (c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[AT][0] = 0x3f000000;
            c.mtc1(1, 4);
            c.g[V1][0] = c.g[V0][0];
            c.g[AT][0] = 0x3e800000;
            c.mtc1(1, 3);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[A3][0] = c.g[S3][0];
            c.lwc1(0, c.g[V0][0] as u32);
            c.sub_s(1, 1, 6);
            c.lwc1(2, c.g[V1][0] as u32);
            c.sub_s(0, 0, 5);
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x844));
            c.add_s(2, 2, 5);
            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.mtc1(0, 12);
            c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
            c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.add_s(0, 0, 6);
            c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x38));
            c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x3c));
            c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x24));
            c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x34));
            call_ptr(c, 0x0038ec40, 0x0017f77c)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
            c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0xf8));
            c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0xf0));
            c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0xe8));
            c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
            c.pc = t;
            return Ok(());
        }
    }
}

/// `DebugMenuModule* DebugMenuModule_ctor(DebugMenuModule* self)` (0x17f7b8):
/// runs `AppModule_ctor`, installs the vtable (0x46d900), clears the fields at
/// `+0x24..+0x30` and zero-fills the 0x200-byte cursor trail at `+0x34` with
/// `EA_MemSet` (after a 64-step countdown the compiler left in); returns self.
pub fn debug_menu_module_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x0023_1cd0,
            f: ee::s06::AppModule_ctor,
        }
        .call(c, 0x0017_f7cc)?;
        c.g[V0][0] = 0x47_0000;
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x24), 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2700));
        c.sw(this.wrapping_add(0x28), 0);
        c.sw(this, c.g[V0][0]);
        c.g[V1][0] = sx(this.wrapping_add(0x24));
        c.sw(this.wrapping_add(0x2c), 0);
        c.g[A0][0] = u64::MAX;
        c.sw(this.wrapping_add(0x30), 0);
        c.g[V0][0] = 0x3f;
        loop {
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            let more = c.g[V0][0] != c.g[A0][0];
            c.g[A1][0] = 0;
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0017_f7f8);
                }
                continue;
            }
            break;
        }
        c.cycles += 9;
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x210), 0);
        c.g[A2][0] = 0x200;
        Callee {
            addr: 0x003e_6448,
            f: ee::s14::EA_MemSet,
        }
        .call(c, 0x0017_f824)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `void DebugMenuModule_dtor(DebugMenuModule* self, int flags)` (0x17f838):
/// resets the vtable to the base class one (0x46d970) and, when bit 0 of
/// `flags` is set, frees the object (`__builtin_delete`, 0x317e50).
pub fn debug_menu_module_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[V0][0] = 0x47_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2690));
        frame.save_ra(c, 0);
        c.g[A1][0] &= 1;
        let keep = c.g[A1][0] == 0;
        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
        if !keep {
            c.cycles += 2;
            Callee {
                addr: 0x0031_7e50,
                f: ee::s10::__builtin_delete,
            }
            .call(c, 0x0017_f85c)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void DebugMenuModule_Init(DebugMenuModule* self)` (0x17f868): flushes
/// the renderer, creates the debug font and loads its two resource files
/// (`+0x18` font, `+0x1c` and `+0x20` texture handles, the loaded buffers are freed
/// again), clears `+0x10`/`+0xc` and builds the DebugMenu (0x8c48 bytes,
/// `+0xc`, `+0x14`) and the LobbyMenu (0x1c18 bytes, `+0x10`); `+0x14` ends up the
/// lobby menu when the world flag at `+0x74` is set, else the debug menu.
/// The body is kept instruction by instruction (registers and cycles).
pub fn debug_menu_module_init(c: &mut Cpu) {
    guest(c, debug_menu_module_init_body);
}

pub(crate) fn debug_menu_module_init_body(c: &mut Cpu) -> Flow {
    'b_17f964: {
        c.cycles += 45;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[A1][0] = 0u64;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x160));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x164));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0017f898)?;
        c.g[A1][0] = 0x460000;
        c.g[A3][0] = 0u64;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2b48));
        c.g[A0][0] = 0x84;
        c.g[A2][0] = 0u64;
        call_ptr(c, 0x00317d70, 0x0017f8b0)?;
        c.g[A1][0] = 0x460000;
        c.g[A2][0] = 0u64;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2b38));
        call_ptr(c, 0x003915e8, 0x0017f8c4)?;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5a2c));
        c.g[V1][0] = 0xffffffffffffffff;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
        c.g[A0][0] = 0x460000;
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2b20));
        c.swc1(0, c.g[SP][0] as u32);
        c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.ldr(5, c.g[SP][0] as u32);
        c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x37));
        c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x30));
        c.g[A1][0] = 0x100;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.ldl(9, (c.g[V0][0] as u32).wrapping_add(0x37));
        c.ldr(9, (c.g[V0][0] as u32).wrapping_add(0x30));
        c.sdl(9, (c.g[V0][0] as u32).wrapping_add(0x3f));
        c.sdr(9, (c.g[V0][0] as u32).wrapping_add(0x38));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
        call_ptr(c, 0x003e1908, 0x0017f910)?;
        c.g[S0][0] = c.g[V0][0];
        let t = c.g[S0][0] == 0u64;
        c.g[A0][0] = 0x460000;
        if t {
            break 'b_17f964;
        } else {
            'b_17f958: {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                let t = c.g[A1][0] == 0u64;
                c.g[T2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
                if t {
                    break 'b_17f958;
                } else {
                    c.cycles += 11;
                    c.g[A2][0] = 0x4a0000;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1340));
                    c.g[A3][0] = 0u64;
                    c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10d8));
                    c.g[T0][0] = 0x1;
                    c.g[T1][0] = 0xffffffffffffffff;
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x170));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x174));
                    let t = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0017f954)?;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                    break 'b_17f958;
                }
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x00317e98, 0x0017f960)?;
            c.g[A0][0] = 0x460000;
            break 'b_17f964;
        }
    }
    'b_17f9b8: {
        c.cycles += 19;
        c.g[A1][0] = 0x100;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2b08));
        call_ptr(c, 0x003e18c8, 0x0017f970)?;
        c.g[T2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[A2][0] = 0x4a0000;
        c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10d8));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1348));
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[A3][0] = 0u64;
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x170));
        c.g[T0][0] = 0x1;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x174));
        c.g[T1][0] = 0xffffffffffffffff;
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0017f9a8)?;
        let t = c.g[S0][0] == 0u64;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x20), c.g[V0][0]);
        if t {
            break 'b_17f9b8;
        } else {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x00317e98, 0x0017f9b8)?;
            break 'b_17f9b8;
        }
    }
    c.cycles += 2;
    c.g[V1][0] = 0x1;
    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
    'l_17f9c0: loop {
        c.cycles += 7;
        c.sw(c.g[V0][0] as u32, 0u64);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
        let t = (c.g[V1][0] as i64) >= 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
        if t {
            if event_due(c) {
                deliver_event_here(c, 0x0017f9c0);
            }
            continue 'l_17f9c0;
        } else {
            'b_17fa48: {
                'b_17fa44: {
                    c.cycles += 24;
                    c.g[A1][0] = 0x460000;
                    c.g[A2][0] = 0u64;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2af0));
                    c.g[A3][0] = 0u64;
                    c.g[A0][0] = 0x8c48;
                    call_ptr(c, 0x00317d70, 0x0017f9f4)?;
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0017b1b0, 0x0017f9fc)?;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x1c18;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2ae0));
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0u64;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    call_ptr(c, 0x00317d70, 0x0017fa1c)?;
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0017d298, 0x0017fa24)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x74));
                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                        break 'b_17fa44;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[A0][0]);
                        if t {
                            break 'b_17fa48;
                        } else {
                            break 'b_17fa44;
                        }
                    }
                }
                c.cycles += 1;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                break 'b_17fa48;
            }
            c.cycles += 5;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
            c.pc = t;
            return Ok(());
        }
    }
}

/// `bool DebugMenuModule_TryExit(void)` (0x17fab0): `AppModule_TryExit() > 0`.
pub fn debug_menu_module_try_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        Callee {
            addr: 0x0023_1cf0,
            f: ee::s06::AppModule_TryExit,
        }
        .call(c, 0x0017_fac0)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = (0 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    });
}

/// `bool FEUIInput_IsStart(FEUIInput* self)` (0x17fe80): button 0x70 of the
/// binding at `*(self + 0x18)` (`InputBinding_EvalBool`, 0x320c48).
pub fn feui_input_is_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x70;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0032_0c48, 0x0017_fe94)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `FEUIInput_IsSelect(input)` (0x017fea0): asks the pad object at `*(input + 0x18)` whether
/// button 0x71 (select) is pressed and returns its answer.
pub fn feui_input_is_select(c: &mut Cpu) {
    oc821_feui_eval(c, 0x71, 0x0017feb4);
}

/// `FEUIInput_IsPrevious(input)` (0x017fee0): asks the pad object at `*(input + 0x18)` whether
/// button 0x7b (previous) is pressed and returns its answer.
pub fn feui_input_is_previous(c: &mut Cpu) {
    oc821_feui_eval(c, 0x7b, 0x0017fef4);
}

/// `FEUIInput_IsOption(input)` (0x017ff00): asks the pad object at `*(input + 0x18)` whether
/// button 0x7c (option) is pressed and returns its answer.
pub fn feui_input_is_option(c: &mut Cpu) {
    oc821_feui_eval(c, 0x7c, 0x0017ff14);
}

/// `FEUIInput_IsMisc(input)` (0x017ff20): asks the pad object at `*(input + 0x18)` whether
/// button 0x7d (misc) is pressed and returns its answer.
pub fn feui_input_is_misc(c: &mut Cpu) {
    oc821_feui_eval(c, 0x7d, 0x0017ff34);
}

/// `FEUIInput_IsNextHeld(input)` (0x017ff40): asks the pad object at `*(input + 0x18)` whether
/// button 0x97 (next (held)) is pressed and returns its answer.
pub fn feui_input_is_next_held(c: &mut Cpu) {
    oc821_feui_eval(c, 0x97, 0x0017ff54);
}

/// `FEUIInput_IsPreviousHeld(input)` (0x017ff60): asks the pad object at `*(input + 0x18)` whether
/// button 0x98 (previous (held)) is pressed and returns its answer.
pub fn feui_input_is_previous_held(c: &mut Cpu) {
    oc821_feui_eval(c, 0x98, 0x0017ff74);
}

/// `FEUIInput_IsOptionHeld(input)` (0x017ff80): asks the pad object at `*(input + 0x18)` whether
/// button 0x99 (option (held)) is pressed and returns its answer.
pub fn feui_input_is_option_held(c: &mut Cpu) {
    oc822_feui_eval(c, 0x99, 0x0017ff94);
}

/// `FEUIInput_IsMiscHeld(input)` (0x017ffa0): asks the pad object at `*(input + 0x18)` whether
/// button 0x9a (misc (held)) is pressed and returns its answer.
pub fn feui_input_is_misc_held(c: &mut Cpu) {
    oc822_feui_eval(c, 0x9a, 0x0017ffb4);
}

/// `FEUIInput_IsNextReleased(input)` (0x017ffc0): asks the pad object at `*(input + 0x18)` whether
/// button 0x9b (next (released)) is pressed and returns its answer.
pub fn feui_input_is_next_released(c: &mut Cpu) {
    oc822_feui_eval(c, 0x9b, 0x0017ffd4);
}

/// `FEUIInput_IsPreviousReleased(input)` (0x017ffe0): asks the pad object at `*(input + 0x18)` whether
/// button 0x9c (previous (released)) is pressed and returns its answer.
pub fn feui_input_is_previous_released(c: &mut Cpu) {
    oc822_feui_eval(c, 0x9c, 0x0017fff4);
}

/// `FEUIInput_IsOptionReleased(input)` (0x0180000): asks the pad object at `*(input + 0x18)` whether
/// button 0x9d (option (released)) is pressed and returns its answer.
pub fn feui_input_is_option_released(c: &mut Cpu) {
    oc822_feui_eval(c, 0x9d, 0x00180014);
}

/// `FEUIInput_IsMiscReleased(input)` (0x0180020): asks the pad object at `*(input + 0x18)` whether
/// button 0x9e (misc (released)) is pressed and returns its answer.
pub fn feui_input_is_misc_released(c: &mut Cpu) {
    oc822_feui_eval(c, 0x9e, 0x00180034);
}

/// `FEUIInput_IsLeft(input)` (0x0180080): asks the pad object at `*(input + 0x18)` whether
/// button 0x74 (left) is pressed and returns its answer.
pub fn oc823_feui_is_left(c: &mut Cpu) {
    oc823_feui_eval(c, 0x74, 0x00180094);
}

/// `FEUIInput_IsRight(input)` (0x01800a0): asks the pad object at `*(input + 0x18)` whether
/// button 0x75 (right) is pressed and returns its answer.
pub fn oc823_feui_is_right(c: &mut Cpu) {
    oc823_feui_eval(c, 0x75, 0x001800b4);
}

/// `FEUIInput_IsUpHeld(input)` (0x01800c0): asks the pad object at `*(input + 0x18)` whether
/// button 0x93 (up (held)) is pressed and returns its answer.
pub fn oc823_feui_is_up_held(c: &mut Cpu) {
    oc823_feui_eval(c, 0x93, 0x001800d4);
}

/// `FEUIInput_IsDownHeld(input)` (0x01800e0): asks the pad object at `*(input + 0x18)` whether
/// button 0x94 (down (held)) is pressed and returns its answer.
pub fn oc823_feui_is_down_held(c: &mut Cpu) {
    oc823_feui_eval(c, 0x94, 0x001800f4);
}

/// `FEUIInput_IsLeftHeld(input)` (0x0180100): asks the pad object at `*(input + 0x18)` whether
/// button 0x95 (left (held)) is pressed and returns its answer.
pub fn oc823_feui_is_left_held(c: &mut Cpu) {
    oc823_feui_eval(c, 0x95, 0x00180114);
}

/// `FEUIInput_IsRightHeld(input)` (0x0180120): asks the pad object at `*(input + 0x18)` whether
/// button 0x96 (right (held)) is pressed and returns its answer.
pub fn oc823_feui_is_right_held(c: &mut Cpu) {
    oc823_feui_eval(c, 0x96, 0x00180134);
}

/// `FEUIInput_IsRShift1(input)` (0x0180140): asks the pad object at `*(input + 0x18)` whether
/// button 0x7e (RShift1) is pressed and returns its answer.
pub fn feui_input_is_r_shift1(c: &mut Cpu) {
    oc824_feui_eval(c, 0x7e, 0x00180154);
}

/// `FEUIInput_IsRShift2(input)` (0x0180160): asks the pad object at `*(input + 0x18)` whether
/// button 0x7f (RShift2) is pressed and returns its answer.
pub fn feui_input_is_r_shift2(c: &mut Cpu) {
    oc824_feui_eval(c, 0x7f, 0x00180174);
}

/// `FEUIInput_IsLShift1(input)` (0x0180180): asks the pad object at `*(input + 0x18)` whether
/// button 0x80 (LShift1) is pressed and returns its answer.
pub fn feui_input_is_l_shift1(c: &mut Cpu) {
    oc824_feui_eval(c, 0x80, 0x00180194);
}

/// `FEUIInput_IsLShift2(input)` (0x01801a0): asks the pad object at `*(input + 0x18)` whether
/// button 0x81 (LShift2) is pressed and returns its answer.
pub fn feui_input_is_l_shift2(c: &mut Cpu) {
    oc824_feui_eval(c, 0x81, 0x001801b4);
}

/// `FEUIInput_IsRShift1Held(input)` (0x01801c0): asks the pad object at `*(input + 0x18)` whether
/// button 0x82 (RShift1 (held)) is pressed and returns its answer.
pub fn feui_input_is_r_shift1_held(c: &mut Cpu) {
    oc824_feui_eval(c, 0x82, 0x001801d4);
}

/// `FEUIInput_IsRShift2Held(input)` (0x01801e0): asks the pad object at `*(input + 0x18)` whether
/// button 0x83 (RShift2 (held)) is pressed and returns its answer.
pub fn feui_input_is_r_shift2_held(c: &mut Cpu) {
    oc824_feui_eval(c, 0x83, 0x001801f4);
}

/// Shared body of the `FEUIInput_*` queries: `callee(*(input + 0x18), button)`;
/// `ret` is the call's return address.
pub(crate) fn oc825_feui_query(c: &mut Cpu, callee: u32, button: u64, ret: u32) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = button;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, callee, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `FEUIInput_IsLShift1Held(input)`: is button 0x84 (LShift1, held) down.
pub fn feui_input_is_l_shift1_held(c: &mut Cpu) {
    oc825_feui_query(c, 0x0032_0c48, 0x84, 0x0018_0214);
}

/// `FEUIInput_IsLShift2Held(input)`: is button 0x85 (LShift2, held) down.
pub fn feui_input_is_l_shift2_held(c: &mut Cpu) {
    oc825_feui_query(c, 0x0032_0c48, 0x85, 0x0018_0234);
}

/// `FEUIInput_GetCamZoom(input)`: analog value of button 0x90 (camera zoom).
pub fn feui_input_get_cam_zoom(c: &mut Cpu) {
    oc825_feui_query(c, 0x0032_0bf0, 0x90, 0x0018_02b4);
}

/// `FEUIInput_GetCamPan(input)`: analog value of button 0x92 (camera pan).
pub fn feui_input_get_cam_pan(c: &mut Cpu) {
    oc825_feui_query(c, 0x0032_0bf0, 0x92, 0x0018_02d4);
}

/// `FEUIInput_GetCamRotate(input)`: analog value of button 0x91 (camera rotate).
pub fn feui_input_get_cam_rotate(c: &mut Cpu) {
    oc825_feui_query(c, 0x0032_0bf0, 0x91, 0x0018_02f4);
}

/// `KeyboardScreen_BuildMaskedText(screen)`: fills `screen + 0x12c` with one `*` per
/// character of the high-score name at `screen + 0xec`, terminates it, returns the buffer end.
pub fn keyboard_screen_build_masked_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.sq(16, frame.at(c, 0x10));
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xec));
        call_ptr(c, 0x0041_6810, 0x0018_07f0)?;
        c.g[A1][0] = c.g[V0][0];
        let le = (c.g[A1][0] as i64) <= 0;
        c.g[A0][0] = 0;
        if !le {
            c.cycles += 3;
            c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x12c));
            c.g[A3][0] = 0x2a;
            loop {
                c.cycles += 7;
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.sb(c.g[V1][0] as u32, c.g[A3][0]);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0018_0808);
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x12c));
        frame.restore_ra(c, 0);
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.lq(16, frame.at(c, 0x10));
        c.sb(c.g[V1][0] as u32, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void KeyboardScreen_TypeKey(KeyboardScreen* self)` (0x180840): type the
/// highlighted key (`self+0x170`) into the text buffer (`self+0xec`) at the
/// cursor (`self+0x16c`, limit `self+0xd4`): digits, letters (upper case by
/// the caps flag `self+0xe4`), backspace (36), delete (37) and space (38).
/// Then tell the owner (`self+0xd0`) through its vtable: `(0, 13)` for key 39
/// (done), else `(0, 12)`.
pub fn keyboard_screen_type_key(c: &mut Cpu) {
    guest(c, |c| {
        'b_180954: {
            'b_180950: {
                'b_180938: {
                    'b_180874: {
                        c.cycles += 7;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                        c.g[A3][0] = c.g[A0][0];
                        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x170));
                        c.g[V0][0] = (c.g[V1][0] < 0xau64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xa));
                        if t {
                            break 'b_180874;
                        } else {
                            c.cycles += 6;
                            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x16c));
                            c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x170));
                            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x30));
                            let t = true;
                            c.sb((c.g[V1][0] as u32).wrapping_add(0xec), c.g[V0][0]);
                            if t {
                                break 'b_180938;
                            } else {
                                break 'b_180874;
                            }
                        }
                    }
                    'b_1808d0: {
                        c.cycles += 3;
                        c.g[V0][0] = (c.g[V0][0] < 0x1au64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = 0x61;
                        if t {
                            break 'b_1808d0;
                        } else {
                            'b_1808c0: {
                                c.cycles += 16;
                                c.g[A2][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xe4));
                                c.g[V1][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x170));
                                c.g[V0][0] = 0x41;
                                c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x16c));
                                if c.g[A2][0] == 0 {
                                    c.g[V0][0] = c.g[A1][0];
                                }
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xa));
                                c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(0xec));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                c.g[A0][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x16c));
                                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xd4));
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.sw((c.g[A3][0] as u32).wrapping_add(0x16c), c.g[V1][0]);
                                    break 'b_1808c0;
                                } else {
                                    break 'b_1808c0;
                                }
                            }
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x16c));
                            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            let t = true;
                            c.sb(c.g[V0][0] as u32, 0u64);
                            if t {
                                break 'b_180950;
                            } else {
                                break 'b_1808d0;
                            }
                        }
                    }
                    'b_180920: {
                        'b_180910: {
                            'b_1808f0: {
                                c.cycles += 3;
                                c.g[V0][0] = 0x24;
                                let t = c.g[V1][0] != c.g[V0][0];
                                c.g[V0][0] = 0x25;
                                if t {
                                    break 'b_1808f0;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x16c));
                                    let t = (c.g[V0][0] as i64) <= 0;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                    if t {
                                        break 'b_180910;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.sw((c.g[A3][0] as u32).wrapping_add(0x16c), c.g[V0][0]);
                                        if t {
                                            break 'b_180910;
                                        } else {
                                            break 'b_1808f0;
                                        }
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[V0][0] = 0x26;
                            if t {
                                break 'b_180920;
                            } else {
                                c.cycles += 6;
                                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x16c));
                                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xd4));
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.sw((c.g[A3][0] as u32).wrapping_add(0x16c), c.g[V1][0]);
                                    break 'b_180910;
                                } else {
                                    break 'b_180910;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x16c));
                        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        let t = true;
                        c.sb((c.g[V0][0] as u32).wrapping_add(0xec), 0u64);
                        if t {
                            break 'b_180950;
                        } else {
                            break 'b_180920;
                        }
                    }
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x170));
                        break 'b_180954;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x16c));
                        c.g[V1][0] = 0x20;
                        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.sb((c.g[V0][0] as u32).wrapping_add(0xec), c.g[V1][0]);
                        break 'b_180938;
                    }
                }
                c.cycles += 6;
                c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x16c));
                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xd4));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] != 0u64 {
                    c.sw((c.g[A3][0] as u32).wrapping_add(0x16c), c.g[A0][0]);
                    break 'b_180950;
                } else {
                    break 'b_180950;
                }
            }
            c.cycles += 1;
            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x170));
            break 'b_180954;
        }
        'b_1809a4: {
            'b_180984: {
                c.cycles += 3;
                c.g[V0][0] = 0x27;
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[A3][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xd0));
                if t {
                    break 'b_180984;
                } else {
                    c.cycles += 9;
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0xd;
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x98));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0018097c);
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0018097c)?;
                    let t = true;
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    if t {
                        break 'b_1809a4;
                    } else {
                        break 'b_180984;
                    }
                }
            }
            c.cycles += 8;
            c.g[A1][0] = 0u64;
            c.g[A2][0] = 0xc;
            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x98));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x001809a0);
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x001809a0)?;
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
            break 'b_1809a4;
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}
/// The `0x0017da10` exit shared by terminating `LobbyMenu_OnLobbyEvent`
/// arms: restore the remaining callee-saved registers and return.
/// (Dispatch table at `0x45d070`, verified against the ELF: 12 targets;
/// read from the image by the test fixture.)
pub(crate) fn lobby_menu_event_exit(c: &mut Cpu, frame: &Frame) -> Flow {
    c.cycles += 8;
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}
/// `void LobbyMenu_OnLobbyEvent(LobbyMenu* self, uint* event)` (0x17d6f0):
/// handle one lobby event: pop the menu page and run the per-event arm
/// (login, persona select, room list, challenge, error popup, net-race
/// start), pushing the next page and recording the new `self + 4` state.
/// 14 + per-arm cycles; out-of-range indices and unknown
/// table targets return with `self + 4` untouched.
pub fn lobby_menu_on_lobby_event(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        // The frame stays allocated across an event unwind: only its size
        // travels in this local, so resume rebuilds it without touching sp.
        let frame = Frame { size: 0x70 };
        loop {
            match pc {
                0x0017_d6f0 => {
                    c.cycles += 14;
                    c.g[SP][0] = sx(sp(c).wrapping_sub(0x70));
                    frame.save(c, S1, 0x50);
                    frame.save(c, S0, 0x60);
                    c.g[S1][0] = c.g[A0][0];
                    frame.save(c, S2, 0x40);
                    frame.save(c, S3, 0x30);
                    frame.save(c, S4, 0x20);
                    frame.save(c, S5, 0x10);
                    frame.save_ra(c, 0);
                    c.g[V0][0] = c.lw(r32(c, A1));
                    c.g[A0][0] = sx(r32(c, V0).wrapping_sub(0xc9));
                    c.g[V1][0] = (r32(c, A0) < 0x4d) as u64;
                    c.g[V0][0] = 0x460000;
                    if c.g[V1][0] == 0 {
                        c.cycles += 1;
                        frame.restore(c, S0, 0x60);
                        return lobby_menu_event_exit(c, &frame);
                    }
                    pc = 0x0017_d728;
                    continue;
                }
                0x0017_d728 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx(r32(c, A0) << 2);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x2f90));
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                    c.g[A0][0] = c.lw(r32(c, V1));
                    pc = c.g[A0][0] as u32;
                    continue;
                }
                0x0017_d740 => {
                    c.cycles += 4;
                    c.g[S0][0] = sx(r32(c, S1).wrapping_add(8));
                    c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(8));
                    c.g[S2][0] = c.g[S0][0];
                    if c.g[V0][0] == 0 {
                        pc = 0x0017_d764;
                        continue;
                    }
                    pc = 0x0017_d750;
                    continue;
                }
                0x0017_d750 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x0017_d758);
                    c.g[A0][0] = c.g[S0][0];
                    callee::MENU_SYS_POP_PAGE.call(c, 0x0017_d758)?;
                    c.g[V0][0] = c.lw(r32(c, S0));
                    if c.g[V0][0] != 0 {
                        if back_edge_unwinds(c, 0x0017_d750) {
                            return Ok(());
                        }
                        pc = 0x0017_d750;
                        continue;
                    }
                    pc = 0x0017_d764;
                    continue;
                }
                0x0017_d764 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(gp_8(c, -0x848));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x74));
                    c.g[V0][0] = 0x13;
                    if c.g[V1][0] == 0 {
                        pc = 0x0017_d77c;
                        continue;
                    }
                    pc = 0x0017_d774;
                    continue;
                }
                0x0017_d774 => {
                    c.cycles += 2;
                    c.sw(r32(c, S1).wrapping_add(4), 0);
                    c.cycles += 1;
                    frame.restore(c, S0, 0x60);
                    return lobby_menu_event_exit(c, &frame);
                }
                0x0017_d77c => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S2][0];
                    c.sw(r32(c, S1).wrapping_add(4), c.g[V0][0]);
                    c.g[A1][0] = sx(r32(c, S1).wrapping_add(0x604));
                    callee::MENU_SYS_PUSH_PAGE.call(c, 0x0017_d78c)?;
                    frame.restore(c, S0, 0x60);
                    return lobby_menu_event_exit(c, &frame);
                }
                0x0017_d794 => {
                    c.cycles += 5;
                    c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(8));
                    c.g[A0][0] = c.g[S1][0];
                    callee::LOBBY_MENU_SHOW_ERROR.call(c, 0x0017_d7a0)?;
                    frame.restore(c, S0, 0x60);
                    return lobby_menu_event_exit(c, &frame);
                }
                0x0017_d7a8 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(gp_8(c, -0x848));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x74));
                    c.g[A0][0] = c.lw(gp_8(c, LOBBY_INSTANCE_GP));
                    if c.g[V0][0] == 0 {
                        pc = 0x0017_d7ec;
                        continue;
                    }
                    pc = 0x0017_d7b8;
                    continue;
                }
                0x0017_d7b8 => {
                    c.cycles += 13;
                    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x48));
                    callee::C_BX_STRING_ASSIGN_CSTR.call(c, 0x0017_d7c4)?;
                    c.g[V1][0] = c.lw(gp_8(c, -0x848));
                    c.g[A0][0] = c.lw(gp_8(c, LOBBY_INSTANCE_GP));
                    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc8));
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x4c));
                    callee::C_BX_STRING_ASSIGN_CSTR.call(c, 0x0017_d7d8)?;
                    c.g[A0][0] = c.lw(gp_8(c, LOBBY_INSTANCE_GP));
                    callee::LOBBY_MAN_LOGIN.call(c, 0x0017_d7e0)?;
                    c.g[V0][0] = 5;
                    c.sw(r32(c, S1).wrapping_add(4), c.g[V0][0]);
                    c.cycles += 1;
                    frame.restore(c, S0, 0x60);
                    return lobby_menu_event_exit(c, &frame);
                }
                0x0017_d7ec => {
                    c.cycles += 7;
                    c.g[V0][0] = 3;
                    c.g[A1][0] = sx(r32(c, S1).wrapping_add(0x70));
                    c.sw(r32(c, S1).wrapping_add(4), c.g[V0][0]);
                    c.g[A0][0] = sx(r32(c, S1).wrapping_add(8));
                    callee::MENU_SYS_PUSH_PAGE.call(c, 0x0017_d800)?;
                    frame.restore(c, S0, 0x60);
                    return lobby_menu_event_exit(c, &frame);
                }
                0x0017_d808 => {
                    c.cycles += 14;
                    c.g[A0][0] = c.lw(gp_8(c, LOBBY_INSTANCE_GP));
                    c.g[V0][0] = 5;
                    c.g[A1][0] = c.lw(r32(c, S1).wrapping_add(0x1c04));
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x48));
                    c.sw(r32(c, S1).wrapping_add(4), c.g[V0][0]);
                    callee::C_BX_STRING_ASSIGN_CSTR.call(c, 0x0017_d820)?;
                    c.g[A0][0] = c.lw(gp_8(c, LOBBY_INSTANCE_GP));
                    c.g[A1][0] = c.lw(r32(c, S1).wrapping_add(0x1c08));
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x4c));
                    callee::C_BX_STRING_ASSIGN_CSTR.call(c, 0x0017_d830)?;
                    c.g[A0][0] = c.lw(gp_8(c, LOBBY_INSTANCE_GP));
                    callee::LOBBY_MAN_LOGIN.call(c, 0x0017_d838)?;
                    frame.restore(c, S0, 0x60);
                    return lobby_menu_event_exit(c, &frame);
                }
                0x0017_d840 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(gp_8(c, -0x848));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x74));
                    c.g[V0][0] = 7;
                    if c.g[V1][0] == 0 {
                        pc = 0x0017_d8d8;
                        continue;
                    }
                    pc = 0x0017_d850;
                    continue;
                }
                0x0017_d850 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw(gp_8(c, LOBBY_INSTANCE_GP));
                    c.sw(r32(c, S1).wrapping_add(4), c.g[V0][0]);
                    c.g[A1][0] = 0;
                    callee::LOBBY_MAN_SELECT_PERSONA_INDEX.call(c, 0x0017_d860)?;
                    frame.restore(c, S0, 0x60);
                    return lobby_menu_event_exit(c, &frame);
                }
                0x0017_d868 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(gp_8(c, -0x848));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x74));
                    c.g[S0][0] = 0;
                    if c.g[V1][0] == 0 {
                        pc = 0x0017_d924;
                        continue;
                    }
                    pc = 0x0017_d878;
                    continue;
                }
                0x0017_d878 => {
                    c.cycles += 3;
                    c.g[S3][0] = 0x4a0000;
                    c.g[S2][0] = 0xb;
                    pc = 0x0017_d8bc;
                    continue;
                }
                0x0017_d884 => {
                    c.cycles += 1;
                    pc = 0x0017_d888;
                    continue;
                }
                0x0017_d888 => {
                    c.cycles += 7;
                    c.g[A1][0] = c.g[S0][0];
                    callee::LOBBY_MAN_GET_ROOM_NAME.call(c, 0x0017_d890)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = sx(r32(c, S3).wrapping_add(0x1320));
                    callee::STRCMP.call(c, 0x0017_d89c)?;
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                        pc = 0x0017_d8bc;
                        continue;
                    }
                    pc = 0x0017_d8a4;
                    continue;
                }
                0x0017_d8a4 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw(gp_8(c, LOBBY_INSTANCE_GP));
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = 0;
                    callee::LOBBY_MAN_MOVE_TO_ROOM_INDEX.call(c, 0x0017_d8b4)?;
                    c.sw(r32(c, S1).wrapping_add(4), c.g[S2][0]);
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    pc = 0x0017_d8bc;
                    continue;
                }
                0x0017_d8bc => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw(gp_8(c, LOBBY_INSTANCE_GP));
                    callee::LOBBY_MAN_GET_ROOM_COUNT.call(c, 0x0017_d8c4)?;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[A0][0] = c.lw(gp_8(c, LOBBY_INSTANCE_GP));
                        if back_edge_unwinds(c, 0x0017_d888) {
                            return Ok(());
                        }
                        pc = 0x0017_d888;
                        continue;
                    }
                    pc = 0x0017_d8d0;
                    continue;
                }
                0x0017_d8d0 => {
                    c.cycles += 2;
                    frame.restore(c, S0, 0x60);
                    return lobby_menu_event_exit(c, &frame);
                }
                0x0017_d8d8 => {
                    c.cycles += 10;
                    c.g[S0][0] = sx(r32(c, S1).wrapping_add(0x74c));
                    c.g[A1][0] = c.lw(gp_8(c, LOBBY_INSTANCE_GP));
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x001d_b418, 0x0017_d8e8)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = sx(r32(c, S1).wrapping_add(8));
                    callee::MENU_SYS_PUSH_PAGE.call(c, 0x0017_d8f4)?;
                    c.g[V0][0] = 6;
                    c.sw(r32(c, S1).wrapping_add(4), c.g[V0][0]);
                    pc = 0x0017_da0c;
                    continue;
                }
                0x0017_d900 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw(gp_8(c, LOBBY_INSTANCE_GP));
                    callee::LOBBY_MAN_IS_IN_ROOM.call(c, 0x0017_d908)?;
                    c.g[A0][0] = sx(r32(c, S1).wrapping_add(8));
                    if c.g[V0][0] == 0 {
                        pc = 0x0017_d924;
                        continue;
                    }
                    pc = 0x0017_d910;
                    continue;
                }
                0x0017_d910 => {
                    c.cycles += 5;
                    c.g[A1][0] = sx(r32(c, S1).wrapping_add(0x1044));
                    callee::MENU_SYS_PUSH_PAGE.call(c, 0x0017_d918)?;
                    c.g[V0][0] = 0xc;
                    c.sw(r32(c, S1).wrapping_add(4), c.g[V0][0]);
                    pc = 0x0017_da0c;
                    continue;
                }
                0x0017_d924 => {
                    c.cycles += 6;
                    c.g[A0][0] = sx(r32(c, S1).wrapping_add(8));
                    c.g[A1][0] = sx(r32(c, S1).wrapping_add(0xd2c));
                    callee::MENU_SYS_PUSH_PAGE.call(c, 0x0017_d930)?;
                    c.g[V0][0] = 0xa;
                    c.sw(r32(c, S1).wrapping_add(4), c.g[V0][0]);
                    pc = 0x0017_da0c;
                    continue;
                }
                0x0017_d93c => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x11;
                    c.g[S0][0] = sx(r32(c, S1).wrapping_add(8));
                    c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(8));
                    c.g[S3][0] = sx(r32(c, S1).wrapping_add(0x604));
                    c.sw(r32(c, S1).wrapping_add(4), c.g[V0][0]);
                    c.g[S2][0] = c.g[S0][0];
                    if c.g[V1][0] == 0 {
                        pc = 0x0017_d96c;
                        continue;
                    }
                    pc = 0x0017_d958;
                    continue;
                }
                0x0017_d958 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    callee::MENU_SYS_POP_PAGE.call(c, 0x0017_d960)?;
                    c.g[V0][0] = c.lw(r32(c, S0));
                    if c.g[V0][0] != 0 {
                        if back_edge_unwinds(c, 0x0017_d958) {
                            return Ok(());
                        }
                        pc = 0x0017_d958;
                        continue;
                    }
                    pc = 0x0017_d96c;
                    continue;
                }
                0x0017_d96c => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S3][0];
                    callee::MENU_SYS_PUSH_PAGE.call(c, 0x0017_d978)?;
                    frame.restore(c, S0, 0x60);
                    return lobby_menu_event_exit(c, &frame);
                }
                0x0017_d980 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S1][0];
                    callee::LOBBY_MENU_START_NET_RACE.call(c, 0x0017_d988)?;
                    c.g[V0][0] = 0x10;
                    c.sw(r32(c, S1).wrapping_add(4), c.g[V0][0]);
                    pc = 0x0017_da0c;
                    continue;
                }
                0x0017_d994 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(4));
                    c.g[V0][0] = 0xc;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(8));
                        pc = 0x0017_da04;
                        continue;
                    }
                    pc = 0x0017_d9a4;
                    continue;
                }
                0x0017_d9a4 => {
                    c.cycles += 7;
                    c.g[S0][0] = sx(r32(c, S1).wrapping_add(8));
                    c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(8));
                    c.g[S4][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
                    c.g[S2][0] = c.g[S0][0];
                    c.g[S3][0] = sx(r32(c, S1).wrapping_add(0x1c0c));
                    c.g[S5][0] = sx(r32(c, S1).wrapping_add(0x18c0));
                    if c.g[V0][0] == 0 {
                        pc = 0x0017_d9d4;
                        continue;
                    }
                    pc = 0x0017_d9c0;
                    continue;
                }
                0x0017_d9c0 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    callee::MENU_SYS_POP_PAGE.call(c, 0x0017_d9c8)?;
                    c.g[V0][0] = c.lw(r32(c, S0));
                    if c.g[V0][0] != 0 {
                        if back_edge_unwinds(c, 0x0017_d9c0) {
                            return Ok(());
                        }
                        pc = 0x0017_d9c0;
                        continue;
                    }
                    pc = 0x0017_d9d4;
                    continue;
                }
                0x0017_d9d4 => {
                    c.cycles += 12;
                    c.g[V0][0] = 0xf;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S4][0];
                    c.sw(r32(c, S1).wrapping_add(4), c.g[V0][0]);
                    callee::C_BX_STRING_ASSIGN.call(c, 0x0017_d9e8)?;
                    c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(0x1c0c));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S5][0];
                    c.sw(r32(c, S1).wrapping_add(0x1a44), c.g[V1][0]);
                    callee::MENU_SYS_PUSH_PAGE.call(c, 0x0017_d9fc)?;
                    frame.restore(c, S0, 0x60);
                    return lobby_menu_event_exit(c, &frame);
                }
                0x0017_da04 => {
                    c.cycles += 2;
                    c.g[A0][0] = c.lw(gp_8(c, LOBBY_INSTANCE_GP));
                    callee::LOBBY_MAN_SEND_DECLINE.call(c, 0x0017_da0c)?;
                    pc = 0x0017_da0c;
                    continue;
                }
                0x0017_da0c => {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x60);
                    return lobby_menu_event_exit(c, &frame);
                }
                _ => {
                    c.jump_out(pc, 0x0017_d6f0, 0x0017_da30);
                    return Ok(());
                }
            }
        }
    })
}
/// void LobbyMenu_DebugPrintfStub(...)
/// (0x1da540): no-op debug stub: save, restore and return.
/// Cycles only.
pub fn lobby_menu_debug_printf_stub(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
        c.g[V0][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[A1][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A2][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x58), c.g[A3][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[T0][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x68), c.g[T1][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T2][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x78), c.g[T3][0]);
        c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0x38));
        c.swc1(14, (c.g[SP][0] as u32).wrapping_add(0x3c));
        c.swc1(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.swc1(18, (c.g[SP][0] as u32).wrapping_add(0x44));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        return Ok(());
    })
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
        check_all(
            ee::s02::f_0017d298__generated,
            lobby_menu_ctor,
            40,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x17d2);
                c.g[A0][0] = DATA as u64;
                let obj = DATA + 0x10_1000;
                c.bus.write32(W_5 + 0x80, obj);
                c.bus.write32(obj + 0x18, seed);
                c.bus.write32(obj + 0x20, seed * 3 + 1);
                c.bus.write32(W_5 + 0xb0, seed ^ 0x55);
                c.bus.write32(gp_at(0xda0), DATA + 0x10_2000);
            },
        );
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

    use crate::difftest::elf_image;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::game::camera::camera::tests::W_2;

    pub(crate) use crate::game::fe::festatecharselect::tests::oc826_machine;

    pub(crate) use crate::game::fe::festatecharselect::tests::oc826_strict;
    pub(crate) use crate::game::fe::festatecharselect::tests::oc826_zero;

    pub(crate) use crate::game::fe::festatecharselect::tests::OC826_OBJA;

    pub(crate) use crate::game::fe::festatecharselect::tests::OC826_SELF;

    /// Fixture objects, in DATA+0x200000 and up (clear of the fake log at
    /// DATA+0x100000 and of the pointers the fakes return).
    pub(crate) const OC816_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC816_MAGIC: u32 = 0x4f43_3831;

    pub(crate) const OC816_SELF: u32 = DATA + 0x21_0000;

    pub(crate) const OC816_WORLD: u32 = DATA + 0x23_0000;

    pub(crate) const OC816_OBJ: u32 = DATA + 0x24_0000;

    pub(crate) const OC816_FONT: u32 = DATA + 0x25_0000;

    pub(crate) const OC816_GRMAN: u32 = DATA + 0x26_0000;

    pub(crate) const OC816_GRVT: u32 = DATA + 0x27_0000;

    /// Offsets in the menu object (self + 0x8000 + ..): current row and
    /// the values the rows print; `self + 0x8c10 + 8` is the active image.
    pub(crate) const OC816_SEL_ROW: u32 = 0x8000 + 0xc1c;

    pub(crate) const OC816_IMAGE: u32 = 0x8c10 + 8;

    /// Gated fakes for the callees of `DebugMenu_Render`: with the gate word
    /// written they log the call like `fakes::fake_call`; otherwise they run
    /// the body they replaced.
    macro_rules! oc816_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC816_CTRL) != OC816_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc816_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc816_fakes!(
        (
            oc816_fake_17ae00,
            OC816_PREV_17AE00,
            ee::s02::OVERRIDE_0017AE00,
            ee::s02::f_0017ae00__generated,
            0x0017ae00
        ),
        (
            oc816_fake_14dc80,
            OC816_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014dc80
        ),
        (
            oc816_fake_14dd58,
            OC816_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014dd58
        ),
        (
            oc816_fake_391cb0,
            OC816_PREV_391CB0,
            ee::s12::OVERRIDE_00391CB0,
            ee::s12::f_00391cb0__generated,
            0x00391cb0
        ),
        (
            oc816_fake_392a60,
            OC816_PREV_392A60,
            ee::s13::OVERRIDE_00392A60,
            ee::s13::f_00392a60__generated,
            0x00392a60
        ),
        (
            oc816_fake_145378,
            OC816_PREV_145378,
            ee::s01::OVERRIDE_00145378,
            ee::s01::f_00145378__generated,
            0x00145378
        ),
        (
            oc816_fake_145398,
            OC816_PREV_145398,
            ee::s01::OVERRIDE_00145398,
            ee::s01::f_00145398__generated,
            0x00145398
        ),
        (
            oc816_fake_145340,
            OC816_PREV_145340,
            ee::s01::OVERRIDE_00145340,
            ee::s01::f_00145340__generated,
            0x00145340
        ),
        (
            oc816_fake_1dca80,
            OC816_PREV_1DCA80,
            ee::s04::OVERRIDE_001DCA80,
            ee::s04::f_001dca80__generated,
            0x001dca80
        ),
        (
            oc816_fake_145c38,
            OC816_PREV_145C38,
            ee::s01::OVERRIDE_00145C38,
            ee::s01::f_00145c38__generated,
            0x00145c38
        ),
        (
            oc816_fake_417828,
            OC816_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x00417828
        ),
        (
            oc816_fake_144bc0,
            OC816_PREV_144BC0,
            ee::s01::OVERRIDE_00144BC0,
            ee::s01::f_00144bc0__generated,
            0x00144bc0
        ),
        (
            oc816_fake_14ee28,
            OC816_PREV_14EE28,
            ee::s01::OVERRIDE_0014EE28,
            ee::s01::f_0014ee28__generated,
            0x0014ee28
        ),
        (
            oc816_fake_147618,
            OC816_PREV_147618,
            ee::s01::OVERRIDE_00147618,
            ee::s01::f_00147618__generated,
            0x00147618
        ),
        (
            oc816_fake_28b180,
            OC816_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028b180
        ),
        (
            oc816_fake_1dc990,
            OC816_PREV_1DC990,
            ee::s04::OVERRIDE_001DC990,
            ee::s04::f_001dc990__generated,
            0x001dc990
        ),
        (
            oc816_fake_1dca40,
            OC816_PREV_1DCA40,
            ee::s04::OVERRIDE_001DCA40,
            ee::s04::f_001dca40__generated,
            0x001dca40
        ),
        (
            oc816_fake_392908,
            OC816_PREV_392908,
            ee::s12::OVERRIDE_00392908,
            ee::s12::f_00392908__generated,
            0x00392908
        ),
        (
            oc816_fake_1dc9d0,
            OC816_PREV_1DC9D0,
            ee::s04::OVERRIDE_001DC9D0,
            ee::s04::f_001dc9d0__generated,
            0x001dc9d0
        ),
        (
            oc816_fake_3f3cc0,
            OC816_PREV_3F3CC0,
            ee::s14::OVERRIDE_003F3CC0,
            ee::s14::f_003f3cc0__generated,
            0x003f3cc0
        ),
    );

    /// `DebugMenu_Render(self)`: the active image negative (no background call)
    /// or not (only the background is drawn), each of the 15 highlighted
    /// rows (and one past them), the multiplayer and world flags, the local
    /// address call, and fake unwinds at random calls.
    #[test]
    fn oc816_debug_menu_render_matches() {
        oc816_install_fakes();
        check(
            ee::s02::f_0017c5f8__generated,
            debug_menu_render,
            480,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0017_c5f8, 70);
                c.bus.write32(OC816_CTRL, OC816_MAGIC);
                let bits = r.next_u32();
                fill(c, &mut r, OC816_FONT, 0x20);
                c.bus.write32(OC816_FONT + 0x14, r.next_u32() & 0xff);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), OC816_WORLD);
                c.bus.write32(OC816_WORLD + 0x80, OC816_OBJ);
                c.bus.write32(OC816_WORLD + 0x78, r.next_u32() & 0xfffff0);
                c.bus.write32(OC816_WORLD + 4, bits & 1);
                c.bus.write32(OC816_OBJ + 0x18, OC816_FONT);
                c.bus.write32(GAME_GP.wrapping_sub(0x1f38), (bits >> 1) & 1);
                c.bus.write32(GAME_GP.wrapping_sub(0x238), (bits >> 2) & 1);
                // The graphics manager's vtable slot 0x4c: a fake method, or a
                // real leaf that returns 0.
                c.bus.write32(GAME_GP.wrapping_sub(0x854), OC816_GRMAN);
                c.bus.write32(OC816_GRMAN + 0x10d8, OC816_GRVT);
                c.bus.write16(OC816_GRVT + 0x48, 0x10 * (seed % 3) as u16);
                c.bus.write32(
                    OC816_GRVT + 0x4c,
                    if (bits >> 3) & 1 == 0 {
                        fakes::FAKE + 0x4c
                    } else {
                        0x0014_06c0
                    },
                );
                fill(c, &mut r, OC816_SELF + 0x8c00, 0x10);
                let image = if (bits >> 4) % 8 == 0 {
                    0u32
                } else {
                    0xffff_ffff
                };
                c.bus.write32(OC816_SELF + OC816_IMAGE, image);
                c.bus.write32(OC816_SELF + OC816_SEL_ROW, seed % 17);
                c.bus.write32(OC816_SELF + 0x8000 + 0xc24, (bits >> 8) % 6);
                c.bus.write32(OC816_SELF + 0x8000 + 0xc30, r.next_u32());
                c.bus
                    .write32(OC816_SELF + 0x8000 + 0xc34, r.next_u32() & 0xffff);
                c.bus.write32(OC816_SELF + 0x8000 + 0xc44, (bits >> 12) % 4);
                c.g[A0][0] = sx(OC816_SELF);
            },
        );
    }

    /// `DebugMenu_StartMultiplayer`, no network session object (`0x534b30`
    /// is 0): the BE interface call and the return only.
    #[test]
    fn debug_menu_start_multiplayer_idle_matches() {
        check(
            ee::s02::f_0017d0a8__generated,
            oc817_debug_menu_start_multiplayer,
            30,
            &|c, seed| {
                let _ = fakes::machine_rt(c, seed, 0x817_0a8, 0);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x100);
                c.bus.write32(DATA + 0x100 + 0x78, DATA + 0x200);
                c.bus.write32(0x0053_4b30, 0);
                c.pc = 0x0017_d0a8;
            },
        );
    }

    /// `DebugMenu_StartMultiplayer` with a session object: runs the real,
    /// unported network callees (0x256038, 0x255bf8, 0x255d50, 0x256bf8), which
    /// may not be bounded by a fixture; drop this test if it spins.
    #[test]
    fn debug_menu_start_multiplayer_session_matches() {
        check(
            ee::s02::f_0017d0a8__generated,
            oc817_debug_menu_start_multiplayer,
            30,
            &|c, seed| {
                let _ = fakes::machine_rt(c, seed, 0x817_0a9, 0);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x100);
                c.bus.write32(DATA + 0x100 + 0x78, DATA + 0x200);
                c.bus.write32(GAME_GP.wrapping_sub(0x238), DATA + 0x1000);
                c.bus.write32(GAME_GP.wrapping_add(0x304), DATA + 0x2000);
                c.bus.write32(0x0053_4b30, DATA + 0x3000);
                c.bus.write32(0x0053_4b38, seed % 2);
                c.pc = 0x0017_d0a8;
            },
        );
    }

    /// `DebugMenu_DispatchItemAction(self, a1, item)`: every item 0..8 with a1
    /// 0..3 and the jump table holding the two handlers or a foreign address
    /// (which leaves through `jump_out`).
    #[test]
    fn debug_menu_dispatch_item_action_matches() {
        check(
            ee::s02::f_0017d1b8__generated,
            oc817_debug_menu_dispatch_item_action,
            200,
            &|c, seed| {
                let _ = machine(c, seed, 0x8171, 0);
                for i in 0..7u32 {
                    let v = match (seed / 9 + i) % 3 {
                        0 => 0x0017_d1ec,
                        1 => 0x0017_d1f4,
                        _ => 0x0017_d200 + 8 * i,
                    };
                    c.bus.write32(0x0045_d040 + 4 * i, v);
                }
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (seed % 9) as u64;
                c.g[RA][0] = sx(0x0020_0000 + 0x10 * seed);
                c.pc = 0x0017_d1b8;
            },
        );
    }

    /// The lobby menu object (0x1c18 bytes).
    pub(crate) const OC818_OBJ: u32 = DATA + 0x40_0000;

    /// The comm system, a channel and its vtable.
    pub(crate) const OC818_COMM: u32 = DATA + 0x41_0000;

    pub(crate) const OC818_WORLD: u32 = DATA + 0x42_0000;

    pub(crate) const OC818_LOBBY_MAN: u32 = DATA + 0x43_0000;

    pub(crate) const OC818_VT: u32 = DATA + 0x44_0000;

    pub(crate) const OC818_CHAN: u32 = DATA + 0x45_0000;

    pub(crate) const OC818_CHAN_VT: u32 = DATA + 0x46_0000;

    pub(crate) const OC818_VOIP: u32 = DATA + 0x47_0000;

    /// The gate of the callee fakes (a word in the machine's own RAM).
    pub(crate) const OC818_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC818_MAGIC: u32 = 0x0c81_8818;

    /// Forced results of the fakes: one word per fake at `FORCE + (addr & 0xffffc)`.
    pub(crate) const OC818_FORCE: u32 = DATA + 0x30_0000;

    pub(crate) fn oc818_w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// Make the fake for `addr` return 0.
    pub(crate) fn oc818_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(OC818_FORCE + (addr & 0xf_fffc), 1);
    }

    /// The gated fake callee: logs the call like `fakes::fake_call`, returns
    /// 0 when its mailbox says so.
    pub(crate) fn oc818_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(OC818_FORCE + (addr & 0xf_fffc));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail == 1 {
            c.g[V0][0] = 0;
        }
    }

    macro_rules! oc818_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC818_CTRL) != OC818_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc818_fake_call(c, $addr);
                }
            )*
            fn oc818_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc818_fakes!(
        (
            oc818_fake_318260,
            OC818_PREV_318260,
            ee::s10::OVERRIDE_00318260,
            ee::s10::f_00318260__generated,
            0x00318260
        ),
        (
            oc818_fake_317e50,
            OC818_PREV_317E50,
            ee::s10::OVERRIDE_00317E50,
            ee::s10::f_00317e50__generated,
            0x00317e50
        ),
        (
            oc818_fake_2ca280,
            OC818_PREV_2CA280,
            ee::s09::OVERRIDE_002CA280,
            ee::s09::f_002ca280__generated,
            0x002ca280
        ),
        (
            oc818_fake_2caa80,
            OC818_PREV_2CAA80,
            ee::s09::OVERRIDE_002CAA80,
            ee::s09::f_002caa80__generated,
            0x002caa80
        ),
        (
            oc818_fake_2cc048,
            OC818_PREV_2CC048,
            ee::s09::OVERRIDE_002CC048,
            ee::s09::f_002cc048__generated,
            0x002cc048
        ),
        (
            oc818_fake_2cc3b8,
            OC818_PREV_2CC3B8,
            ee::s09::OVERRIDE_002CC3B8,
            ee::s09::f_002cc3b8__generated,
            0x002cc3b8
        ),
        (
            oc818_fake_2cc098,
            OC818_PREV_2CC098,
            ee::s09::OVERRIDE_002CC098,
            ee::s09::f_002cc098__generated,
            0x002cc098
        ),
        (
            oc818_fake_1dd0c8,
            OC818_PREV_1DD0C8,
            ee::s04::OVERRIDE_001DD0C8,
            ee::s04::f_001dd0c8__generated,
            0x001dd0c8
        ),
        (
            oc818_fake_1dcea0,
            OC818_PREV_1DCEA0,
            ee::s04::OVERRIDE_001DCEA0,
            ee::s04::f_001dcea0__generated,
            0x001dcea0
        ),
        (
            oc818_fake_1db418,
            OC818_PREV_1DB418,
            ee::s04::OVERRIDE_001DB418,
            ee::s04::f_001db418__generated,
            0x001db418
        ),
        (
            oc818_fake_1db528,
            OC818_PREV_1DB528,
            ee::s04::OVERRIDE_001DB528,
            ee::s04::f_001db528__generated,
            0x001db528
        ),
        (
            oc818_fake_1db620,
            OC818_PREV_1DB620,
            ee::s04::OVERRIDE_001DB620,
            ee::s04::f_001db620__generated,
            0x001db620
        ),
        (
            oc818_fake_25b6d8,
            OC818_PREV_25B6D8,
            ee::s06::OVERRIDE_0025B6D8,
            ee::s06::f_0025b6d8__generated,
            0x0025b6d8
        ),
        (
            oc818_fake_25d800,
            OC818_PREV_25D800,
            ee::s06::OVERRIDE_0025D800,
            ee::s06::f_0025d800__generated,
            0x0025d800
        ),
        (
            oc818_fake_232720,
            OC818_PREV_232720,
            ee::s06::OVERRIDE_00232720,
            ee::s06::f_00232720__generated,
            0x00232720
        ),
        (
            oc818_fake_25c610,
            OC818_PREV_25C610,
            ee::s06::OVERRIDE_0025C610,
            ee::s06::f_0025c610__generated,
            0x0025c610
        ),
        (
            oc818_fake_317d70,
            OC818_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x00317d70
        ),
        (
            oc818_fake_316ef8,
            OC818_PREV_316EF8,
            ee::s10::OVERRIDE_00316EF8,
            ee::s10::f_00316ef8__generated,
            0x00316ef8
        ),
        (
            oc818_fake_3184e8,
            OC818_PREV_3184E8,
            ee::s10::OVERRIDE_003184E8,
            ee::s10::f_003184e8__generated,
            0x003184e8
        ),
        (
            oc818_fake_266d58,
            OC818_PREV_266D58,
            ee::s07::OVERRIDE_00266D58,
            ee::s07::f_00266d58__generated,
            0x00266d58
        ),
        (
            oc818_fake_266e30,
            OC818_PREV_266E30,
            ee::s07::OVERRIDE_00266E30,
            ee::s07::f_00266e30__generated,
            0x00266e30
        ),
        (
            oc818_fake_3f3cc0,
            OC818_PREV_3F3CC0,
            ee::s14::OVERRIDE_003F3CC0,
            ee::s14::f_003f3cc0__generated,
            0x003f3cc0
        ),
        (
            oc818_fake_14dc80,
            OC818_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014dc80
        ),
        (
            oc818_fake_14dd58,
            OC818_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014dd58
        ),
        (
            oc818_fake_145108,
            OC818_PREV_145108,
            ee::s01::OVERRIDE_00145108,
            ee::s01::f_00145108__generated,
            0x00145108
        ),
        (
            oc818_fake_144df0,
            OC818_PREV_144DF0,
            ee::s01::OVERRIDE_00144DF0,
            ee::s01::f_00144df0__generated,
            0x00144df0
        ),
        (
            oc818_fake_144d98,
            OC818_PREV_144D98,
            ee::s01::OVERRIDE_00144D98,
            ee::s01::f_00144d98__generated,
            0x00144d98
        ),
        (
            oc818_fake_147220,
            OC818_PREV_147220,
            ee::s01::OVERRIDE_00147220,
            ee::s01::f_00147220__generated,
            0x00147220
        ),
        (
            oc818_fake_147338,
            OC818_PREV_147338,
            ee::s01::OVERRIDE_00147338,
            ee::s01::f_00147338__generated,
            0x00147338
        ),
        (
            oc818_fake_1472c8,
            OC818_PREV_1472C8,
            ee::s01::OVERRIDE_001472C8,
            ee::s01::f_001472c8__generated,
            0x001472c8
        ),
        (
            oc818_fake_417828,
            OC818_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x00417828
        ),
    );

    /// The common machine: gated fakes on, the jump tables the game has in
    /// its data segment, the menu object at `OC818_OBJ`.
    pub(crate) fn oc818_machine(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        fakes::machine(c, seed, salt, 0);
        c.pc = entry;
        c.bus.write32(OC818_CTRL, OC818_MAGIC);
        c.g[A0][0] = OC818_OBJ as u64;
        oc818_w(c, GAME_GP.wrapping_sub(0x238), OC818_COMM);
        oc818_w(c, GAME_GP.wrapping_sub(0xc8), OC818_LOBBY_MAN);
        oc818_w(c, GAME_GP.wrapping_sub(0x848), OC818_WORLD);
    }

    pub(crate) fn oc818_table(c: &mut Cpu, base: u32, targets: &[u32]) {
        for (i, t) in targets.iter().enumerate() {
            c.bus.write32(base + 4 * i as u32, *t);
        }
    }

    /// The members of the destructor's three lists: a vtable pointer at +0x10
    /// whose slot (+8 delta, +0xc function) is a fake.
    pub(crate) fn oc818_dtor_members(c: &mut Cpu, seed: u32) {
        c.bus.write16(OC818_VT + 8, ((seed & 7) * 4) as u16);
        c.bus.write32(OC818_VT + 0xc, fakes::FAKE);
        for (from, to) in [(0x16c8u32, 0x18c0u32), (0x11b0, 0x1510), (0xe94, 0x1044)] {
            let mut m = from;
            while m < to {
                oc818_w(c, OC818_OBJ + m + 0x10, OC818_VT);
                m += 0x24;
            }
        }
    }

    /// `LobbyMenu_dtor`: all three member lists, both values of the
    /// `delete` flag.
    #[test]
    fn lobby_menu_dtor_matches() {
        oc818_install_fakes();
        check(
            ee::s02::f_0017d3b0__generated,
            lobby_menu_dtor,
            20,
            &|c, seed| {
                oc818_machine(c, seed, 0x17d3b0, 0x0017_d3b0);
                oc818_dtor_members(c, seed);
                c.g[A1][0] = [0u64, 1, 2, 3, 0xffff_fffe][seed as usize % 5];
            },
        );
    }

    pub(crate) const OC818_SHOW_ERROR_TABLE: [u32; 12] = [
        0x0017_da6c,
        0x0017_db0c,
        0x0017_da74,
        0x0017_da7c,
        0x0017_db0c,
        0x0017_dac4,
        0x0017_dacc,
        0x0017_dad4,
        0x0017_db0c,
        0x0017_dadc,
        0x0017_db0c,
        0x0017_daf8,
    ];

    /// `LobbyMenu_ShowError`: every state (in and out of the jump table's
    /// range), with the world link present and absent for state 5.
    #[test]
    fn lobby_menu_show_error_matches() {
        oc818_install_fakes();
        check(
            ee::s02::f_0017da30__generated,
            lobby_menu_show_error,
            40,
            &|c, seed| {
                oc818_machine(c, seed, 0x17da30, 0x0017_da30);
                oc818_table(c, 0x45_d1b0, &OC818_SHOW_ERROR_TABLE);
                let state = match seed % 18 {
                    16 => 0xffff_fffe,
                    17 => 0x40,
                    s => s,
                };
                oc818_w(c, OC818_OBJ + 4, state);
                oc818_w(c, OC818_WORLD + 0x74, (seed / 18) & 1);
            },
        );
    }

    pub(crate) const OC818_UPDATE_STATE_TABLE: [u32; 25] = [
        0x0017_db5c,
        0x0017_db84,
        0x0017_de48,
        0x0017_dd9c,
        0x0017_de48,
        0x0017_de48,
        0x0017_dd1c,
        0x0017_de48,
        0x0017_de48,
        0x0017_de48,
        0x0017_dd38,
        0x0017_de48,
        0x0017_dd64,
        0x0017_dbb4,
        0x0017_ddb8,
        0x0017_dd00,
        0x0017_dddc,
        0x0017_dbdc,
        0x0017_dc80,
        0x0017_dc80,
        0x0017_dc5c,
        0x0017_dc38,
        0x0017_dc98,
        0x0017_dcd4,
        0x0017_dc0c,
    ];

    /// The fixture of the state machine: the state, whether a page is up
    /// (`+8`), whether `cMenuSys_IsActive` says so, the comm system's mode
    /// word and the game channel (virtual `isConnected` returning `connected`).
    pub(crate) fn oc818_update_state_fixture(c: &mut Cpu, seed: u32, state: u32, connected: u32) {
        oc818_machine(c, seed, 0x17db18, 0x0017_db18);
        oc818_table(c, 0x45_d1e0, &OC818_UPDATE_STATE_TABLE);
        let variant = seed / 27;
        oc818_w(c, OC818_OBJ + 4, state);
        oc818_w(c, OC818_OBJ + 8, variant & 1);
        if variant & 2 != 0 {
            oc818_force_zero(c, 0x002c_c098);
        }
        oc818_w(c, OC818_OBJ + 0x1c10, 0x10);
        // Never 1: that would call the real `cCommSystem_netConnConnect`.
        let mode = [3u32, 0, 4, 2, 5, 0][(seed / 2) as usize % 6];
        oc818_w(c, OC818_COMM + 0x20, mode);
        oc818_w(c, OC818_COMM + 0x40, OC818_CHAN);
        oc818_w(c, OC818_CHAN + 8, OC818_CHAN_VT);
        c.bus.write16(OC818_CHAN_VT + 0x20, (seed & 3) as u16 * 8);
        oc818_w(c, OC818_CHAN_VT + 0x24, connected);
        oc818_w(c, GAME_GP.wrapping_add(0x304), OC818_VOIP);
    }

    /// `LobbyMenu_UpdateState` over every state, with a page up and none,
    /// the menu system active and not. The game channel is never connected
    /// here (the pre-game load has its own test).
    #[test]
    fn lobby_menu_update_state_matches() {
        oc818_install_fakes();
        check(
            ee::s02::f_0017db18__generated,
            lobby_menu_update_state,
            108,
            &|c, seed| {
                let state = match seed % 27 {
                    26 => 0xffff_fff0,
                    s => s,
                };
                oc818_update_state_fixture(c, seed, state, 0x0014_06c0);
            },
        );
    }

    /// State 16 with a connected game channel: build the game comm and
    /// start the pre-game load. The real `cGameComm_construct` runs here on
    /// fake memory.
    #[test]
    fn lobby_menu_update_state_game_start_matches() {
        oc818_install_fakes();
        check(
            ee::s02::f_0017db18__generated,
            lobby_menu_update_state,
            8,
            &|c, seed| {
                oc818_update_state_fixture(c, seed, 16, fakes::FAKE);
                oc818_w(c, OC818_LOBBY_MAN + 0xb4, seed);
            },
        );
    }

    /// `LobbyMenu_StartNetRace`: host and guest, with a comm system whose
    /// only channel slot is taken so `cCommSystem_openDSockChannel` returns -1.
    #[test]
    fn lobby_menu_start_net_race_matches() {
        oc818_install_fakes();
        check(
            ee::s02::f_0017de70__generated,
            lobby_menu_start_net_race,
            20,
            &|c, seed| {
                oc818_machine(c, seed, 0x17de70, 0x0017_de70);
                oc818_w(c, OC818_COMM, 0x0090_1230);
                oc818_w(c, OC818_WORLD + 0x78, 0x1234);
                oc818_w(c, OC818_LOBBY_MAN + 0xb4, seed & 1);
                oc818_w(c, OC818_LOBBY_MAN + 0xa8, 0xc0a8_0001 ^ seed);
                oc818_w(c, OC818_LOBBY_MAN + 0xac, 0x0a00_00fe ^ (seed << 8));
            },
        );
    }

    // Fixture objects, in DATA+0x200000 and up (clear of the fakes' call log at
    // DATA+0x100000 and of the pointers the fakes return).
    pub(crate) const OC819_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC819_MAGIC: u32 = 0x4f43_3831;

    pub(crate) const OC819_WORLD: u32 = DATA + 0x21_0000;

    pub(crate) const OC819_VIEW: u32 = DATA + 0x22_0000;

    pub(crate) const OC819_FONT: u32 = DATA + 0x23_0000;

    pub(crate) const OC819_SELF: u32 = DATA + 0x24_0000;

    pub(crate) const OC819_LOBBY: u32 = DATA + 0x25_0000;

    pub(crate) const OC819_COMM: u32 = DATA + 0x26_0000;

    /// Gated fake callees: logged like `fakes::fake_call` while the machine's
    /// gate word is set, the body they replaced otherwise.
    macro_rules! oc819_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC819_CTRL) != OC819_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc819_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc819_fakes!(
        (
            oc819_fake_392908,
            OC819_PREV_392908,
            ee::s12::OVERRIDE_00392908,
            ee::s12::f_00392908__generated,
            0x0039_2908
        ),
        (
            oc819_fake_391cb0,
            OC819_PREV_391CB0,
            ee::s12::OVERRIDE_00391CB0,
            ee::s12::f_00391cb0__generated,
            0x0039_1cb0
        ),
        (
            oc819_fake_392a60,
            OC819_PREV_392A60,
            ee::s13::OVERRIDE_00392A60,
            ee::s13::f_00392a60__generated,
            0x0039_2a60
        ),
        (
            oc819_fake_1dca80,
            OC819_PREV_1DCA80,
            ee::s04::OVERRIDE_001DCA80,
            ee::s04::f_001dca80__generated,
            0x001d_ca80
        ),
        (
            oc819_fake_1dca40,
            OC819_PREV_1DCA40,
            ee::s04::OVERRIDE_001DCA40,
            ee::s04::f_001dca40__generated,
            0x001d_ca40
        ),
        (
            oc819_fake_2cc350,
            OC819_PREV_2CC350,
            ee::s09::OVERRIDE_002CC350,
            ee::s09::f_002cc350__generated,
            0x002c_c350
        ),
        (
            oc819_fake_260e50,
            OC819_PREV_260E50,
            ee::s06::OVERRIDE_00260E50,
            ee::s06::f_00260e50__generated,
            0x0026_0e50
        ),
        (
            oc819_fake_318260,
            OC819_PREV_318260,
            ee::s10::OVERRIDE_00318260,
            ee::s10::f_00318260__generated,
            0x0031_8260
        ),
    );

    /// `LobbyMenu_Render`: every lobby state (0..24 with its jump-table target,
    /// 25..27 past the table), the font at `world->+0x80->+0x18`, the menu stack
    /// (none, or its top entry), the 'Play Game' table's host flag, and the
    /// comm system in a net state other than 5. The state-5 branch (the local
    /// address, through the unported `NetConnStatus` RPC chain at 0x3f3cc0) is
    /// not reached: that callee has no slot to fake.
    #[test]
    fn lobby_menu_render_matches() {
        oc819_install_fakes();
        check(
            ee::s02::f_0017e5c8__generated,
            lobby_menu_render,
            420,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x819, 24);
                c.bus.write32(OC819_CTRL, OC819_MAGIC);
                // World -> view -> font (scale at +0x30/+0x34, outputs +0x38..).
                c.bus.write32(GAME_GP.wrapping_sub(0x848), OC819_WORLD);
                c.bus.write32(OC819_WORLD + 0x80, OC819_VIEW);
                c.bus.write32(OC819_VIEW + 0x18, OC819_FONT);
                fill(c, &mut r, OC819_FONT, 0x20);
                let (scale_x, scale_y) = (r.edge_f32(), r.edge_f32());
                c.bus.write32(OC819_FONT + 0x30, scale_x);
                c.bus.write32(OC819_FONT + 0x34, scale_y);
                // The screen: state, menu stack count, entries, error code.
                fill(c, &mut r, OC819_SELF, 0x20);
                c.bus.write32(OC819_SELF + 4, seed % 28);
                let count = [0u32, 1, 2, 3, 0xffff_ffff][((seed / 28) % 5) as usize];
                c.bus.write32(OC819_SELF + 8, count);
                for i in 0..4u32 {
                    let top = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        DATA + 0x1000 * (i + 1)
                    };
                    c.bus.write32(OC819_SELF + 0xc + 4 * i, top);
                }
                let err = r.next_u32();
                c.bus.write32(OC819_SELF + 0x1c14, err);
                // The lobby manager (gp-0xc8): names, host flag, addresses, seed.
                c.bus.write32(GAME_GP.wrapping_sub(0xc8), OC819_LOBBY);
                fill(c, &mut r, OC819_LOBBY + 0x80, 0x20);
                c.bus.write32(OC819_LOBBY + 0xb4, seed % 2);
                // The comm system (gp-0x238): net state 0..4, never 5.
                c.bus.write32(GAME_GP.wrapping_sub(0x238), OC819_COMM);
                c.bus.write32(OC819_COMM + 0x20, seed % 5);
                c.g[A0][0] = sx(OC819_SELF);
                c.pc = 0x0017_e5c8;
            },
        );
    }

    /// Control word of this module's gated fakes (they only act when the
    /// machine's RAM holds the magic here).
    pub(crate) const OC820_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC820_MAGIC: u32 = 0x4f43_8200;

    /// Forced-result mailboxes, one word per fake callee.
    pub(crate) const OC820_MAIL: u32 = DATA + 0x20_1000;

    pub(crate) fn oc820_mail(addr: u32) -> u32 {
        OC820_MAIL + (addr & 0xffc)
    }

    /// Make the fake for `addr` return `v` in `v0` (zero-extended).
    pub(crate) fn oc820_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc820_mail(addr), v.wrapping_add(2));
    }

    /// Make the fake for `addr` return 0.
    pub(crate) fn oc820_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(oc820_mail(addr), 1);
    }

    /// Arm the gated fakes in this machine.
    pub(crate) fn oc820_arm(c: &mut Cpu) {
        c.bus.write32(OC820_CTRL, OC820_MAGIC);
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox (0 = the fake's pointer) unless it unwound.
    pub(crate) fn oc820_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc820_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = if mail == 1 {
                0
            } else {
                mail.wrapping_sub(2) as u64
            };
        }
    }

    macro_rules! oc820_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC820_CTRL) != OC820_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc820_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc820_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc820_fakes!(
        (
            oc820_fake_17b1b0,
            OC820_PREV_17B1B0,
            ee::s02::OVERRIDE_0017B1B0,
            ee::s02::f_0017b1b0__generated,
            0x0017_b1b0
        ),
        (
            oc820_fake_231cd0,
            OC820_PREV_231CD0,
            ee::s06::OVERRIDE_00231CD0,
            ee::s06::f_00231cd0__generated,
            0x0023_1cd0
        ),
        (
            oc820_fake_231cf0,
            OC820_PREV_231CF0,
            ee::s06::OVERRIDE_00231CF0,
            ee::s06::f_00231cf0__generated,
            0x0023_1cf0
        ),
        (
            oc820_fake_317e50,
            OC820_PREV_317E50,
            ee::s10::OVERRIDE_00317E50,
            ee::s10::f_00317e50__generated,
            0x0031_7e50
        ),
        (
            oc820_fake_317e98,
            OC820_PREV_317E98,
            ee::s10::OVERRIDE_00317E98,
            ee::s10::f_00317e98__generated,
            0x0031_7e98
        ),
        (
            oc820_fake_320c48,
            OC820_PREV_320C48,
            ee::s10::OVERRIDE_00320C48,
            ee::s10::f_00320c48__generated,
            0x0032_0c48
        ),
        (
            oc820_fake_38ec40,
            OC820_PREV_38EC40,
            ee::s12::OVERRIDE_0038EC40,
            ee::s12::f_0038ec40__generated,
            0x0038_ec40
        ),
        (
            oc820_fake_3915e8,
            OC820_PREV_3915E8,
            ee::s12::OVERRIDE_003915E8,
            ee::s12::f_003915e8__generated,
            0x0039_15e8
        ),
        (
            oc820_fake_3e18c8,
            OC820_PREV_3E18C8,
            ee::s14::OVERRIDE_003E18C8,
            ee::s14::f_003e18c8__generated,
            0x003e_18c8
        ),
        (
            oc820_fake_3e1908,
            OC820_PREV_3E1908,
            ee::s14::OVERRIDE_003E1908,
            ee::s14::f_003e1908__generated,
            0x003e_1908
        ),
        (
            oc820_fake_3e6448,
            OC820_PREV_3E6448,
            ee::s14::OVERRIDE_003E6448,
            ee::s14::f_003e6448__generated,
            0x003e_6448
        ),
    );

    /// `DebugMenuModule_TryExit`: `AppModule_TryExit() > 0` (unsigned compare
    /// of the forced result: zero, small, and values with the top bit set) and
    /// an unwinding callee.
    #[test]
    fn debug_menu_module_try_exit_matches() {
        oc820_install_fakes();
        check(
            ee::s02::f_0017fab0__generated,
            debug_menu_module_try_exit,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x17_fab0, 1);
                oc820_arm(c);
                match seed % 5 {
                    0 => {}
                    1 => oc820_force_zero(c, 0x0023_1cf0),
                    2 => oc820_force(c, 0x0023_1cf0, 5),
                    3 => oc820_force(c, 0x0023_1cf0, 0x8000_0000),
                    _ => oc820_force(c, 0x0023_1cf0, 0xffff_fff0),
                }
            },
        );
    }

    /// `FEUIInput_IsStart`: the button query on `*(input + 0x18)`.
    #[test]
    fn feui_input_is_start_matches() {
        oc820_install_fakes();
        check(
            ee::s02::f_0017fe80__generated,
            feui_input_is_start,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x17_fe80, 1);
                oc820_arm(c);
                c.g[A0][0] = sx(DATA + 0x8_0000);
                c.bus.write32(DATA + 0x8_0000 + 0x18, DATA + 0x8_1000);
                if seed % 3 == 1 {
                    oc820_force_zero(c, 0x0032_0c48);
                } else if seed % 3 == 2 {
                    oc820_force(c, 0x0032_0c48, 1);
                }
            },
        );
    }

    /// `DebugMenuModule_dtor`: base vtable store; the delete only when bit 0
    /// of the flags is set (other bits random), and an unwinding delete.
    #[test]
    fn debug_menu_module_dtor_matches() {
        oc820_install_fakes();
        check(
            ee::s02::f_0017f838__generated,
            debug_menu_module_dtor,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x17_f838, 1);
                oc820_arm(c);
                c.g[A0][0] = sx(DATA + 0x8_0000);
                c.g[A1][0] = (r.next_u32() & !1 | (seed & 1)) as u64;
            },
        );
    }

    /// `DebugMenuModule_ctor`: base constructor, field clears, vtable, the
    /// 0x200-byte fill, and unwinds in either call.
    #[test]
    fn debug_menu_module_ctor_matches() {
        oc820_install_fakes();
        check(
            ee::s02::f_0017f7b8__generated,
            debug_menu_module_ctor,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x17_f7b8, 2);
                oc820_arm(c);
                c.g[A0][0] = sx(DATA + 0x8_0000);
            },
        );
    }

    /// `DebugMenuModule_DrawCursorTrail`: the render-state words, the 63 trail
    /// rectangles (head positive and negative, so the ring index takes both
    /// sides of the sign correction) and the final box; unwinds at any of the
    /// 64 draw calls.
    #[test]
    fn debug_menu_module_draw_cursor_trail_matches() {
        oc820_install_fakes();
        check(
            ee::s02::f_0017f500__generated,
            debug_menu_module_draw_cursor_trail,
            48,
            &|c, seed| {
                let mut r = machine(c, seed, 0x17_f500, 65);
                oc820_arm(c);
                let menu = DATA + 0x8_0000;
                let rend = DATA + 0x8_1000;
                let state = DATA + 0x8_2000;
                c.bus.write32(GAME_GP.wrapping_sub(0x854), rend);
                c.bus.write32(GAME_GP.wrapping_sub(0x844), DATA + 0x8_3000);
                c.bus.write32(rend + 0xe84, state);
                c.bus.write32(state + 4, r.next_u32());
                c.bus.write32(state + 0x10, r.next_u32());
                let wild = seed % 4 == 0;
                for off in [0x5a38u32, 0x5a34, 0x5a30] {
                    let v = if wild { r.edge_f32() } else { r.f32(4.0) };
                    c.bus.write32(GAME_GP.wrapping_sub(off), v);
                }
                for i in 0..64u32 {
                    for k in 0..2u32 {
                        let v = if wild { r.edge_f32() } else { r.f32(300.0) };
                        c.bus.write32(menu + 0x10 + 8 * i + 4 * k, v);
                    }
                }
                let head = match seed % 6 {
                    0 => 0u32.wrapping_sub(1 + r.next_u32() % 100),
                    1 => 63,
                    _ => r.next_u32() % 200,
                };
                c.bus.write32(menu + 0x210, head);
                c.g[A0][0] = sx(menu);
            },
        );
    }

    /// `DebugMenuModule_Init`: the renderer flush and the two file loads (each
    /// absent, present with a buffer that is freed, or present with a zero
    /// payload pointer), the font setup, the two menus, and the final choice
    /// of the active menu by the world flag. Real pool allocator and real
    /// `LobbyMenu_ctor` (fixture of `lobby_menu_ctor_matches`); the file, font,
    /// free and debug-menu callees are gated fakes.
    #[test]
    fn debug_menu_module_init_matches() {
        oc820_install_fakes();
        check(
            ee::s02::f_0017f868__generated,
            debug_menu_module_init,
            24,
            &|c, seed| {
                let mut r = alloc_base(c, seed, 0x17_f868);
                // The lobby menu's own fixture.
                let obj = DATA + 0x10_1000;
                c.bus.write32(W_2 + 0x80, obj);
                c.bus.write32(obj + 0x18, seed);
                c.bus.write32(obj + 0x20, seed * 3 + 1);
                c.bus.write32(W_2 + 0xb0, seed ^ 0x55);
                c.bus.write32(gp_at(0xda0), DATA + 0x10_2000);
                oc820_arm(c);
                c.bus.write32(
                    LOG_UNWIND_AT,
                    if seed % 3 == 0 {
                        r.next_u32() % 8
                    } else {
                        u32::MAX
                    },
                );
                let menu = DATA + 0x8_0000;
                let rend = DATA + 0x8_1000;
                let vt = DATA + 0x8_2000;
                c.bus.write32(GAME_GP.wrapping_sub(0x854), rend);
                c.bus.write32(GAME_GP + 0x2a90, rend);
                c.bus.write32(rend + 0x10d8, vt);
                for slot in [0x160u32, 0x170] {
                    c.bus.write16(vt + slot, (r.next_u32() & 0x1c) as u16);
                    c.bus.write32(vt + slot + 4, fakes::FAKE + slot);
                }
                // The font and the debug menu live outside the allocator.
                oc820_force(c, 0x0039_15e8, DATA + 0x8_5000);
                oc820_force(c, 0x0017_b1b0, DATA + 0x8_6000);
                // The two file loads: absent, present, present with a
                // buffer whose payload pointer cancels to zero.
                let (p1, p2) = (DATA + 0x8_4000, DATA + 0x8_4400);
                match seed % 4 {
                    0 => {
                        oc820_force_zero(c, 0x003e_1908);
                        oc820_force(c, 0x003e_18c8, p2);
                        c.bus.write32(p2 + 0x14, 0x40);
                    }
                    1 => {
                        oc820_force(c, 0x003e_1908, p1);
                        c.bus.write32(p1 + 0x14, 0x40);
                        oc820_force_zero(c, 0x003e_18c8);
                    }
                    2 => {
                        oc820_force(c, 0x003e_1908, p1);
                        c.bus.write32(p1 + 0x14, 0u32.wrapping_sub(p1));
                        oc820_force(c, 0x003e_18c8, p2);
                        c.bus.write32(p2 + 0x14, 0u32.wrapping_sub(p2));
                    }
                    _ => {
                        oc820_force(c, 0x003e_1908, p1);
                        c.bus.write32(p1 + 0x14, 0x40);
                        oc820_force(c, 0x003e_18c8, p2);
                        c.bus.write32(p2 + 0x14, 0x40);
                    }
                }
                // The world flag picks the active menu.
                c.bus.write32(W_2 + 0x74, (seed / 4) % 2);
                c.g[A0][0] = sx(menu);
            },
        );
    }

    /// `FEUIInput_IsSelect`: the button query and its unwinds.
    #[test]
    fn feui_input_is_select_matches() {
        fakes::covered(
            ee::s02::f_0017fea0__generated,
            feui_input_is_select,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x17fea0, 1);
                c.pc = 0x0017fea0;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsPrevious`: the button query and its unwinds.
    #[test]
    fn feui_input_is_previous_matches() {
        fakes::covered(
            ee::s02::f_0017fee0__generated,
            feui_input_is_previous,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x17fee0, 1);
                c.pc = 0x0017fee0;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsOption`: the button query and its unwinds.
    #[test]
    fn feui_input_is_option_matches() {
        fakes::covered(
            ee::s02::f_0017ff00__generated,
            feui_input_is_option,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x17ff00, 1);
                c.pc = 0x0017ff00;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsMisc`: the button query and its unwinds.
    #[test]
    fn feui_input_is_misc_matches() {
        fakes::covered(
            ee::s02::f_0017ff20__generated,
            feui_input_is_misc,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x17ff20, 1);
                c.pc = 0x0017ff20;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsNextHeld`: the button query and its unwinds.
    #[test]
    fn feui_input_is_next_held_matches() {
        fakes::covered(
            ee::s02::f_0017ff40__generated,
            feui_input_is_next_held,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x17ff40, 1);
                c.pc = 0x0017ff40;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsPreviousHeld`: the button query and its unwinds.
    #[test]
    fn feui_input_is_previous_held_matches() {
        fakes::covered(
            ee::s02::f_0017ff60__generated,
            feui_input_is_previous_held,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x17ff60, 1);
                c.pc = 0x0017ff60;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsOptionHeld`: the button query and its unwinds.
    #[test]
    fn feui_input_is_option_held_matches() {
        fakes::covered(
            ee::s02::f_0017ff80__generated,
            feui_input_is_option_held,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x17ff80, 1);
                c.pc = 0x0017ff80;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsMiscHeld`: the button query and its unwinds.
    #[test]
    fn feui_input_is_misc_held_matches() {
        fakes::covered(
            ee::s02::f_0017ffa0__generated,
            feui_input_is_misc_held,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x17ffa0, 1);
                c.pc = 0x0017ffa0;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsNextReleased`: the button query and its unwinds.
    #[test]
    fn feui_input_is_next_released_matches() {
        fakes::covered(
            ee::s02::f_0017ffc0__generated,
            feui_input_is_next_released,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x17ffc0, 1);
                c.pc = 0x0017ffc0;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsPreviousReleased`: the button query and its unwinds.
    #[test]
    fn feui_input_is_previous_released_matches() {
        fakes::covered(
            ee::s02::f_0017ffe0__generated,
            feui_input_is_previous_released,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x17ffe0, 1);
                c.pc = 0x0017ffe0;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsOptionReleased`: the button query and its unwinds.
    #[test]
    fn feui_input_is_option_released_matches() {
        fakes::covered(
            ee::s02::f_00180000__generated,
            feui_input_is_option_released,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x180000, 1);
                c.pc = 0x00180000;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsMiscReleased`: the button query and its unwinds.
    #[test]
    fn feui_input_is_misc_released_matches() {
        fakes::covered(
            ee::s02::f_00180020__generated,
            feui_input_is_misc_released,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x180020, 1);
                c.pc = 0x00180020;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsLeft`: the button query and its unwinds.
    #[test]
    fn oc823_feui_is_left_matches() {
        fakes::covered(
            ee::s02::f_00180080__generated,
            oc823_feui_is_left,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x180080, 1);
                c.pc = 0x00180080;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsRight`: the button query and its unwinds.
    #[test]
    fn oc823_feui_is_right_matches() {
        fakes::covered(
            ee::s02::f_001800a0__generated,
            oc823_feui_is_right,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x1800a0, 1);
                c.pc = 0x001800a0;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsUpHeld`: the button query and its unwinds.
    #[test]
    fn oc823_feui_is_up_held_matches() {
        fakes::covered(
            ee::s02::f_001800c0__generated,
            oc823_feui_is_up_held,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x1800c0, 1);
                c.pc = 0x001800c0;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsDownHeld`: the button query and its unwinds.
    #[test]
    fn oc823_feui_is_down_held_matches() {
        fakes::covered(
            ee::s02::f_001800e0__generated,
            oc823_feui_is_down_held,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x1800e0, 1);
                c.pc = 0x001800e0;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsLeftHeld`: the button query and its unwinds.
    #[test]
    fn oc823_feui_is_left_held_matches() {
        fakes::covered(
            ee::s02::f_00180100__generated,
            oc823_feui_is_left_held,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x180100, 1);
                c.pc = 0x00180100;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsRightHeld`: the button query and its unwinds.
    #[test]
    fn oc823_feui_is_right_held_matches() {
        fakes::covered(
            ee::s02::f_00180120__generated,
            oc823_feui_is_right_held,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x180120, 1);
                c.pc = 0x00180120;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsRShift1`: the button query and its unwinds.
    #[test]
    fn feui_input_is_r_shift1_matches() {
        fakes::covered(
            ee::s02::f_00180140__generated,
            feui_input_is_r_shift1,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x180140, 1);
                c.pc = 0x00180140;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsRShift2`: the button query and its unwinds.
    #[test]
    fn feui_input_is_r_shift2_matches() {
        fakes::covered(
            ee::s02::f_00180160__generated,
            feui_input_is_r_shift2,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x180160, 1);
                c.pc = 0x00180160;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsLShift1`: the button query and its unwinds.
    #[test]
    fn feui_input_is_l_shift1_matches() {
        fakes::covered(
            ee::s02::f_00180180__generated,
            feui_input_is_l_shift1,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x180180, 1);
                c.pc = 0x00180180;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsLShift2`: the button query and its unwinds.
    #[test]
    fn feui_input_is_l_shift2_matches() {
        fakes::covered(
            ee::s02::f_001801a0__generated,
            feui_input_is_l_shift2,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x1801a0, 1);
                c.pc = 0x001801a0;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsRShift1Held`: the button query and its unwinds.
    #[test]
    fn feui_input_is_r_shift1_held_matches() {
        fakes::covered(
            ee::s02::f_001801c0__generated,
            feui_input_is_r_shift1_held,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x1801c0, 1);
                c.pc = 0x001801c0;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsRShift2Held`: the button query and its unwinds.
    #[test]
    fn feui_input_is_r_shift2_held_matches() {
        fakes::covered(
            ee::s02::f_001801e0__generated,
            feui_input_is_r_shift2_held,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x1801e0, 1);
                c.pc = 0x001801e0;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    pub(crate) fn oc825_query_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        let _r = machine(c, seed, entry as u64, 1);
        c.pc = entry;
        c.g[A0][0] = sx(DATA + 0x1000);
        c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
    }

    /// `FEUIInput_IsLShift1Held`: the button query and its unwinds.
    #[test]
    fn feui_input_is_l_shift1_held_matches() {
        fakes::covered(
            ee::s02::f_00180200__generated,
            feui_input_is_l_shift1_held,
            60,
            &|c, seed| oc825_query_fixture(c, seed, 0x0018_0200),
        );
    }

    /// `FEUIInput_IsLShift2Held`: the button query and its unwinds.
    #[test]
    fn feui_input_is_l_shift2_held_matches() {
        fakes::covered(
            ee::s02::f_00180220__generated,
            feui_input_is_l_shift2_held,
            60,
            &|c, seed| oc825_query_fixture(c, seed, 0x0018_0220),
        );
    }

    /// `FEUIInput_GetCamZoom`: the analog query and its unwinds.
    #[test]
    fn feui_input_get_cam_zoom_matches() {
        fakes::covered(
            ee::s02::f_001802a0__generated,
            feui_input_get_cam_zoom,
            60,
            &|c, seed| oc825_query_fixture(c, seed, 0x0018_02a0),
        );
    }

    /// `FEUIInput_GetCamPan`: the analog query and its unwinds.
    #[test]
    fn feui_input_get_cam_pan_matches() {
        fakes::covered(
            ee::s02::f_001802c0__generated,
            feui_input_get_cam_pan,
            60,
            &|c, seed| oc825_query_fixture(c, seed, 0x0018_02c0),
        );
    }

    /// `FEUIInput_GetCamRotate`: the analog query and its unwinds.
    #[test]
    fn feui_input_get_cam_rotate_matches() {
        fakes::covered(
            ee::s02::f_001802e0__generated,
            feui_input_get_cam_rotate,
            60,
            &|c, seed| oc825_query_fixture(c, seed, 0x0018_02e0),
        );
    }

    /// `KeyboardScreen_BuildMaskedText`: the name length drives the star loop.
    #[test]
    fn keyboard_screen_build_masked_text_matches() {
        crate::difftest::check(
            ee::s02::f_001807d8__generated,
            keyboard_screen_build_masked_text,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x1807d8, 1);
                c.pc = 0x0018_07d8;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0xec, 0x6968_6766);
                c.bus.write32(DATA + 0x1000 + 0xf0, 0x006b_6a00);
                // strlen is faked: force 1..=8 (the fake cannot return 0 or a sane random length).
                c.bus.write32(fakes::LOG_RESULTS, 1 + seed % 8);
            },
        );
    }

    /// A keyboard screen `self` at [`OC826_SELF`] with its owner object
    /// (`+0xd0`), text limit (`+0xd4`), cursor (`+0x16c`) and key (`+0x170`).
    pub(crate) fn oc826_keyboard(c: &mut Cpu, r: &mut Rng, key: u32) {
        oc826_zero(c, OC826_SELF, 0x200);
        let max = 1 + r.next_u32() % 8;
        c.bus.write32(OC826_SELF + 0xd0, OC826_OBJA);
        c.bus.write32(OC826_SELF + 0xd4, max);
        c.bus.write32(OC826_SELF + 0xe4, r.next_u32() % 2);
        c.bus.write32(OC826_SELF + 0x16c, r.next_u32() % (max + 1));
        c.bus.write32(OC826_SELF + 0x170, key);
    }

    /// `KeyboardScreen_TypeKey`: every key class (digit, letter, backspace,
    /// delete, space, done, others), caps on and off, cursor at either end.
    #[test]
    fn keyboard_screen_type_key_matches() {
        oc826_strict(
            ee::s02::f_00180840__generated,
            keyboard_screen_type_key,
            270,
            &|c, seed| {
                let mut r = oc826_machine(c, seed, 0x18_0840, 2);
                oc826_keyboard(c, &mut r, seed % 44);
                c.g[A0][0] = sx(OC826_SELF);
            },
        );
    }

    use crate::difftest::put_words;

    pub(crate) use crate::game::fe::feasyncfile::tests::vtable_slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::FAKE_ACTIVE;
    pub(crate) use crate::game::fe::feasyncfile::tests::FAKE_CONN;
    pub(crate) use crate::game::fe::feasyncfile::tests::FAKE_CONN_GATE;
    pub(crate) use crate::game::fe::feasyncfile::tests::FAKE_EV_GATE;
    pub(crate) use crate::game::fe::feasyncfile::tests::FAKE_IS_IN_ROOM;
    pub(crate) use crate::game::fe::feasyncfile::tests::FAKE_MAGIC;
    pub(crate) use crate::game::fe::feasyncfile::tests::FAKE_MAGIC_E6;
    pub(crate) use crate::game::fe::feasyncfile::tests::FAKE_MAGIC_U4;
    pub(crate) use crate::game::fe::feasyncfile::tests::FAKE_ROOM_COUNT;
    pub(crate) use crate::game::fe::feasyncfile::tests::FAKE_ROOM_NAME;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_19;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_21;
    pub(crate) use crate::game::fe::feasyncfile::tests::VT_3;

    driven!(lobby_menu_dtor_generated, ee::s02::f_0017d3b0__generated);

    driven!(lobby_menu_dtor_hand, lobby_menu_dtor);

    #[test]
    fn lobby_menu_dtor_events_match() {
        check_events(
            lobby_menu_dtor_generated,
            lobby_menu_dtor_hand,
            SEEDS_19,
            &|c, seed| {
                let mut r = base_machine_unwind(c, seed, 0x0017_d3b0, 0);
                // An arena object, so the final `__builtin_delete` frees a
                // real chunk when the flag word is nonzero.
                let this = alloc(c, 0x1c40);
                clear_words(c, this, 0x710);
                c.bus.write32(GAME_GP + 0x1da0, FIX + 0x2400);
                // Shared-empty strings: no buffer free, `flags & 1` decides.
                put_words(c, this + 0x1c04, &[0x0046_dcd4, 0x0046_dcd4, 0x0046_dcd4]);
                // Message-list drain: `*(self + 0x6cc)` is the head word;
                // `*head` walks N1..Nk and returns to N1, freeing N2..Nk.
                let n1 = alloc(c, 0x20);
                c.bus.write32(this + 0x6cc, n1);
                let mut prev = n1;
                // The drain loop only terminates on an empty list (it never
                // updates the head word), so all seeds keep it empty.
                let nodes = 0;
                for _ in 0..nodes {
                    let n = alloc(c, 0x20);
                    c.bus.write32(prev, n);
                    // `*(node + 8)`: null skips the vtable call, `VT`
                    // routes it to the `FAKE` slot.
                    c.bus.write32(n + 8, if seed % 2 == 0 { 0 } else { VT_3 });
                    c.bus.write32(n + 4, VT_3);
                    prev = n;
                }
                c.bus.write32(prev, n1);
                // The vtable object at `VT`: `*(VT + 4)` is its table, with
                // the destroy slot at `+0x8`/`+0xc` routed to `FAKE`.
                c.bus.write32(VT_3 + 4, VT_3);
                vtable_slot(c, VT_3 + 8, 0, FAKE);
                // `+0x6b8` free arm and the deleting flag word.
                let freeme = if seed % 4 == 0 { alloc(c, 0x20) } else { 0 };
                c.bus.write32(this + 0x6b8, freeme);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = match seed / 3 % 3 {
                    0 => 0,
                    1 => (this + 0x1044) as u64,
                    _ => 3,
                };
                c.g[RA][0] = SENTINEL_RA as u64;
                schedule(c, &mut r, seed, 0x0017_d3b0)
            },
        );
    }

    driven!(
        lobby_menu_on_lobby_event_generated,
        ee::s02::f_0017d6f0__generated
    );

    driven!(lobby_menu_on_lobby_event_hand, lobby_menu_on_lobby_event);

    pub(crate) const FAKE_MAGIC_EF: u32 = 0x0c11_85ef;

    /// `StartNetRace` stub for the 0xef arm: the real backend build runs
    /// away on this fixture (its EA alloc loop never terminates here,
    /// while the direct `StartNetRace` test terminates on its own
    /// fixture); the backend itself is covered by
    /// `lobby_menu_start_net_race_events_match`. The arm's call, state store
    /// and exit are what this test compares.
    pub(crate) fn start_net_race_fake(c: &mut Cpu) {
        if c.bus.read32(FAKE_CTRL) != FAKE_MAGIC_EF {
            return fakes::fall_back(
                c,
                fakes::prev_cell(start_net_race_fake),
                ee::s02::f_0017de70__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0017_de70, ra);
    }

    // ---------------------------------------------------------- lobby login

    #[test]
    fn lobby_menu_on_lobby_event_matches() {
        let resumes = check_events(
            lobby_menu_on_lobby_event_generated,
            lobby_menu_on_lobby_event_hand,
            SEEDS_19,
            &|c, seed| {
                // Event id: one per table target (the pop loops at 0x115,
                // 0xed and 0xec, the room loop at 0xda, the net-race
                // start at 0xef) plus out-of-range.
                let ev = match seed / 3 % 14 {
                    0 => 0x115,
                    1 => 0x102,
                    2 => 0xc9,
                    3 => 0xcb,
                    4 => 0xd4,
                    5 => 0xda,
                    6 => 0xd6,
                    7 => 0xdc,
                    8 => 0xed,
                    9 => 0xf0,
                    10 => 0xec,
                    11 => 0xca,
                    12 => 0xef,
                    _ => 0xc9 + 0x4d + (seed % 3),
                };
                let mut r = base_machine_unwind(c, seed, 0x0017_d6f0, 0);
                elf_image(c);
                clear_words(c, SELF, 0x710);
                comms(c);
                clear_words(c, COMMS_A, 0x100);
                clear_words(c, COMMS_B, 0x100);
                clear_words(c, LOBBY, 0x100);
                let this = SELF;
                fakes::hook(
                    std::ptr::addr_of_mut!(ee::s02::OVERRIDE_0017DE70),
                    start_net_race_fake,
                    fakes::prev_cell(start_net_race_fake),
                );
                // World-pointer sources for the ctor-published words.
                put_words(
                    c,
                    COMMS_A + 0x80,
                    &[FIX + 0x2000, FIX + 0x2100, FIX + 0x2200],
                );
                c.bus.write32(COMMS_A + 0xb0, FIX + 0x2300);
                // Login strings (the config holds pointers to them) and
                // the room names the scan compares with the selected
                // room's name at 0x4a1320 ("A.West"): a mismatch on even
                // seeds, a match on odd ones.
                str(c, STR + 0x40, b"user");
                str(c, STR + 0x60, b"pass");
                put_words(c, COMMS_A + 0xc4, &[STR + 0x40, STR + 0x60]);
                str(c, STR, b"room");
                str(c, STR + 0x80, b"A.West");
                // Menu flag (forced on for the room-loop id), lobby
                // instance, `is_in_room` answer.
                c.bus
                    .write32(COMMS_A + 0x74, if ev == 0xda { 1 } else { seed % 2 });
                c.bus.write32(LOBBY + 0xb4, FIX + 0x2400);
                // The net-race arm stubs `StartNetRace` (gated below).
                c.bus.write32(
                    FAKE_CTRL,
                    if ev == 0xef {
                        FAKE_MAGIC_EF
                    } else {
                        FAKE_MAGIC
                    },
                );
                c.bus.write32(FAKE_IS_IN_ROOM, seed / 2 % 2);
                // Room-loop answers: two rooms; the name fake answers the
                // same name for both, so neither matches the selected
                // room (even seeds) or both do (odd).
                c.bus.write32(FAKE_EV_GATE, FAKE_MAGIC_E6);
                c.bus.write32(FAKE_ROOM_COUNT, 2);
                c.bus
                    .write32(FAKE_ROOM_NAME, if seed % 2 == 0 { STR } else { STR + 0x80 });
                // Pop-loop ids get a two-deep page count (the real
                // pop decrements the slot, so the back edge runs
                // and exits), or an empty stack on seeds 4 mod 5.
                c.bus.write32(
                    this + 8,
                    if ev == 0x115 || ev == 0xed || ev == 0xf0 || ev == 0xec {
                        if seed % 5 == 4 {
                            0
                        } else {
                            2
                        }
                    } else if seed % 3 == 0 {
                        0
                    } else {
                        FIX + 0x2500
                    },
                );
                // The 0xec arm reads the state: 0xc takes the
                // room-page tail, anything else declines the invite.
                // Other ids only store it, so it starts nonzero there
                // and the `0x17d774` clear changes the word.
                c.bus.write32(
                    this + 4,
                    if ev == 0xec {
                        if seed / 5 % 2 == 0 {
                            0xc
                        } else {
                            0
                        }
                    } else {
                        0x5a5a_0000 | seed
                    },
                );
                c.bus.write32(FIX + 0x2400, ev);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (FIX + 0x2400) as u64;
                c.g[RA][0] = SENTINEL_RA as u64;
                schedule(c, &mut r, seed, 0x0017_d6f0)
            },
        );
        for head in [0x0017_d750, 0x0017_d888, 0x0017_d958, 0x0017_d9c0] {
            assert!(
                resumes.get(&head).copied().unwrap_or(0) > 0,
                "no unwind at head {head:#x}",
            );
        }
    }

    driven!(
        lobby_menu_update_state_generated,
        ee::s02::f_0017db18__generated
    );

    driven!(lobby_menu_update_state_hand, lobby_menu_update_state);

    #[test]
    fn lobby_menu_update_state_events_match() {
        check_events(
            lobby_menu_update_state_generated,
            lobby_menu_update_state_hand,
            // 28 states times the 8 steering-answer combos above the
            // state digit, so every state sees every combo.
            224,
            &|c, seed| {
                let mut r = base_machine_unwind(c, seed, 0x0017_db18, 0);
                elf_image(c);
                clear_words(c, SELF, 0x710);
                comms(c);
                clear_words(c, COMMS_A, 0x100);
                clear_words(c, COMMS_B, 0x100);
                clear_words(c, LOBBY, 0x100);
                c.bus.write32(GAME_GP + 0x304, FIX + 0x2600);
                let this = SELF;
                put_words(
                    c,
                    COMMS_A + 0x80,
                    &[FIX + 0x2000, FIX + 0x2100, FIX + 0x2200],
                );
                c.bus.write32(COMMS_A + 0xb0, FIX + 0x2300);
                c.bus.write32(FAKE_CTRL, FAKE_MAGIC);
                // Steering answers come from digits above the state
                // (`seed % 28`), so every state sees both sides of each
                // answer across the seeds.
                c.bus.write32(FAKE_ACTIVE, seed / 28 % 2);
                c.bus.write32(FAKE_IS_IN_ROOM, seed / 56 % 2);
                // Channel-connected answer for the state-16 arm: down on
                // half the seeds (the live lobby word disagrees, entering
                // the arm), up on the rest (both sides create the comm).
                c.bus.write32(FAKE_CONN_GATE, FAKE_MAGIC_U4);
                c.bus.write32(FAKE_CONN, seed / 112 % 2);
                // Connect flag and the page slot (empty skips the live
                // checks, live reuses the page).
                c.bus.write32(COMMS_B + 0x20, seed / 4 % 3);
                c.bus
                    .write32(this + 8, if seed % 3 == 0 { 0 } else { FIX + 0x2500 });
                // States 0..24 run the table; 25+ return at once.
                c.bus.write32(this + 4, seed % 28);
                c.g[A0][0] = this as u64;
                c.g[RA][0] = SENTINEL_RA as u64;
                schedule(c, &mut r, seed, 0x0017_db18)
            },
        );
    }

    driven!(
        lobby_menu_start_net_race_generated,
        ee::s02::f_0017de70__generated
    );

    driven!(lobby_menu_start_net_race_hand, lobby_menu_start_net_race);

    /// `LobbyMenu_StartNetRace`: the backend race/player interfaces are
    /// built through the BELibrary dummy (vtable calls land on `FAKE`),
    /// pads and characters are assigned to the six slots, and the room's
    /// channel is opened. The fixed `0x530000` table and the room block
    /// are fixture memory.

    // --------------------------------------------------------------- lobby

    #[test]
    fn lobby_menu_start_net_race_events_match() {
        check_events(
            lobby_menu_start_net_race_generated,
            lobby_menu_start_net_race_hand,
            SEEDS_19,
            &|c, seed| {
                let mut r = base_machine_unwind(c, seed, 0x0017_de70, 0);
                clear_words(c, SELF, 0x710);
                comms(c);
                clear_words(c, COMMS_A, 0x100);
                clear_words(c, COMMS_B, 0x100);
                clear_words(c, LOBBY, 0x100);
                c.bus.write32(GAME_GP + 0x304, FIX + 0x2600);
                // The room slot and flag word in the fixed table.
                c.bus.write32(0x0053_0000 + 0x4b30, FIX + 0x2400);
                c.bus.write32(0x0053_0000 + 0x5c08, 0);
                c.bus.write32(FIX + 0x2400 + 8, FIX + 0x2500);
                c.bus.write32(FIX + 0x2400 + 0xc, 0x0400 + seed % 2);
                c.bus.write32(FIX + 0x2600, seed % 2);
                c.g[A0][0] = SELF as u64;
                c.g[RA][0] = SENTINEL_RA as u64;
                schedule(c, &mut r, seed, 0x0017_de70)
            },
        );
    }

    driven!(
        lobby_menu_init_delete_persona_page_generated,
        ee::s04::f_001dabd0__generated
    );

    driven!(
        lobby_menu_init_delete_persona_page_hand,
        crate::game::fe::festatenethelppopup::lobby_menu_init_delete_persona_page_2
    );

    driven!(
        lobby_menu_init_select_persona_page_generated,
        ee::s04::f_001dacb8__generated
    );

    driven!(
        lobby_menu_init_select_persona_page_hand,
        crate::game::fe::festatenethelppopup::lobby_menu_init_select_persona_page
    );

    driven!(
        lobby_menu_init_select_room_page_generated,
        ee::s04::f_001dae20__generated
    );

    driven!(
        lobby_menu_init_select_room_page_hand,
        crate::game::fe::festatenethelppopup::lobby_menu_init_select_room_page_2
    );

    /// The delete/select pages run fixed-count drain loops (loop
    /// bounds are constants: 4 + 4 iterations); `check_events` drives
    /// real events through both heads. Same shared fixture.
    pub(crate) fn init_loop_fixture(
        c: &mut Cpu,
        seed: u32,
        entry: u32,
    ) -> crate::difftest::events::Events {
        let mut r = base_machine_ctrl(c, seed, entry as u64);
        elf_image(c);
        install_fakes_3();
        comms(c);
        clear_words(c, COMMS_A, 0x100);
        clear_words(c, COMMS_B, 0x100);
        clear_words(c, LOBBY, 0x100);
        clear_words(c, SELF, 0x710);
        let data = FIX + 0x3000;
        clear_words(c, data, 0x10);
        c.bus.write32(FAKE_CTRL, FAKE_MAGIC3);
        c.g[A0][0] = SELF as u64;
        c.g[A1][0] = if seed / 2 % 2 == 0 { 0 } else { data } as u64;
        c.g[A2][0] = 0;
        c.g[RA][0] = SENTINEL_RA as u64;
        schedule(c, &mut r, seed, entry)
    }

    #[test]
    fn lobby_menu_init_delete_persona_page_events_match() {
        check_events(
            lobby_menu_init_delete_persona_page_generated,
            lobby_menu_init_delete_persona_page_hand,
            SEEDS_21,
            &|c, seed| init_loop_fixture(c, seed, 0x001d_abd0),
        );
    }

    #[test]
    fn lobby_menu_init_select_persona_page_events_match() {
        check_events(
            lobby_menu_init_select_persona_page_generated,
            lobby_menu_init_select_persona_page_hand,
            SEEDS_21,
            &|c, seed| init_loop_fixture(c, seed, 0x001d_acb8),
        );
    }

    #[test]
    fn lobby_menu_init_select_room_page_events_match() {
        check_events(
            lobby_menu_init_select_room_page_generated,
            lobby_menu_init_select_room_page_hand,
            SEEDS_21,
            &|c, seed| init_loop_fixture(c, seed, 0x001d_ae20),
        );
    }
    use crate::difftest::check_paths;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::game::bx::bxstringctor::tests::alloc_base as alloc_base_bx;
    use crate::game::bx::bxstringctor::tests::be_library as be_library_bx;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_machine_ctrl;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_machine_unwind;
    pub(crate) use crate::game::fe::feasyncfile::tests::install_fakes_3 as install_fakes_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::str;
    pub(crate) use crate::game::fe::feasyncfile::tests::FAKE_CTRL;
    pub(crate) use crate::game::fe::feasyncfile::tests::FAKE_MAGIC3;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_19 as SEEDS_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_21 as SEEDS_7;

    pub(crate) const SEEDS_5: u32 = 200;

    pub(crate) const FIX: u32 = DATA + 0x1010;

    pub(crate) const SELF: u32 = DATA + 0x8010;

    pub(crate) const COMMS_A: u32 = FIX;

    pub(crate) const COMMS_B: u32 = FIX + 0x400;

    pub(crate) const LOBBY: u32 = FIX + 0x800;

    pub(crate) fn base_machine_calls(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = alloc_base_bx(c, seed, salt);
        be_library_bx(c);
        if calls > 0 {
            c.bus.write32(
                crate::difftest::fakes::LOG_UNWIND_AT,
                if seed % 3 == 0 {
                    (seed / 3) % calls
                } else {
                    u32::MAX
                },
            );
        }
        fill(c, &mut r, SELF, 0x2000);
        fill(c, &mut r, FIX, 0x1000);
        r
    }

    pub(crate) fn comms(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x848), COMMS_A);
        c.bus.write32(GAME_GP.wrapping_sub(0x238), COMMS_B);
        c.bus.write32(GAME_GP.wrapping_sub(0xc8), LOBBY);
    }

    pub(crate) fn clear_words(c: &mut Cpu, base: u32, words: u32) {
        for i in 0..words {
            c.bus.write32(base + 4 * i, 0);
        }
    }

    /// `LobbyMenu_Render` fixture: every real `0x45d450` table state (plus
    /// out-of-range states) with a font fixture, rank counts and lobby
    /// presence steered; the `0x17e5c8` self-loop and `0x17f0a4` blocks are
    /// unreachable from the real table and not taken.
    pub(crate) fn fixture(c: &mut Cpu, seed: u32) {
        let _r = base_machine_calls(c, seed, 0x0017_e5c8, 0);
        elf_image(c);
        clear_words(c, SELF, 0x710);
        comms(c);
        clear_words(c, COMMS_A, 0x100);
        clear_words(c, COMMS_B, 0x100);
        clear_words(c, LOBBY, 0x100);
        let this = SELF;
        // Font behind `gp - 0x848`: block, string pointer, object.
        let font = FIX + 0x2000;
        let fontobj = FIX + 0x2100;
        c.bus.write32(COMMS_A + 0x80, font);
        c.bus.write32(font + 0x18, fontobj);
        clear_words(c, fontobj, 0x20);
        // State: all 25 real states plus out-of-range.
        let states = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 0x19, 0xff,
        ];
        c.bus.write32(this + 4, states[(seed % 27) as usize]);
        // Rank icons: count and the first entries.
        c.bus.write32(this + 8, (seed / 3 % 3) as u32);
        c.bus
            .write32(this + 0xc, if seed / 5 % 2 == 0 { 0 } else { fontobj });
        c.bus
            .write32(this + 0x10, if seed / 7 % 2 == 0 { 0 } else { fontobj });
        // Lobby presence for the `0x17f0a4` word check.
        c.bus
            .write32(LOBBY + 0xb4, if seed / 11 % 2 == 0 { 0 } else { 1 });
        // Buddy/comm state and the auth word.
        c.bus
            .write32(COMMS_B + 0x20, if seed / 13 % 2 == 0 { 0 } else { 5 });
        c.bus.write32(this + 0x1c14, FIX + 0x2200);
        c.g[A0][0] = this as u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = 0x0017_e5c8;
    }
    #[test]
    fn lobby_menu_render_arm_totals_match() {
        const PATHS: &[u64] = &[
            47, 51, 54, 84, 92, 657, 720, 723, 750, 760, 788, 811, 873, 907, 944, 5774, 5780, 5810,
            5813, 7412, 7485, 7506, 7572, 7618, 7640, 7695, 7699, 7783, 7828, 7838, 7907, 7917,
            7993, 8025, 8047, 8121, 8152, 8210, 8249, 10158, 10193, 10348, 10381, 10413, 10487,
            10519, 10613, 10652, 14974, 15020, 15051, 15173, 15195, 15243, 15301, 15328, 15438,
            15469, 19708, 19739, 19766, 19881,
        ];
        check_paths(
            ee::s02::f_0017e5c8__generated,
            lobby_menu_render,
            SEEDS_5,
            PATHS,
            &fixture,
        );
    }

    /// `LobbyMenu_ShowError` by state: every table entry pushes its page
    /// (or logs in through the live persona page), with the menu flag
    /// steering the login arm; out-of-range states push nothing.
    /// Arm totals (port charges plus the state-dependent callee costs):
    /// 12 out-of-range, 18 bare epilogue, 62 the state-13 push, 65 the
    /// state-11 push, 67 the abort push, 74 the login-skipped push,
    /// 5865 the login-taken arm. The state-11/13 page-init costs are
    /// fixture-address-sensitive (they coincided at 66 before the
    /// nonzero-low-byte fixture shift); both are stable across the
    /// login flag and the fill garbage.
    #[test]
    fn lobby_menu_show_error_arm_totals_match() {
        const PATHS: &[u64] = &[12, 18, 62, 65, 67, 74, 5865];
        check_paths(
            ee::s02::f_0017da30__generated,
            lobby_menu_show_error,
            SEEDS_6,
            PATHS,
            &|c, seed| {
                let _r = base_machine_unwind(c, seed, 0x0017_da30, 0);
                elf_image(c);
                clear_words(c, SELF, 0x710);
                comms(c);
                // Cleared: the login callees read these blocks beyond the
                // steered words, and fill-garbage would wobble the totals.
                clear_words(c, COMMS_A, 0x100);
                clear_words(c, COMMS_B, 0x100);
                clear_words(c, LOBBY, 0x100);
                let this = SELF;
                str(c, COMMS_A + 0xc4, b"user");
                str(c, COMMS_A + 0xc8, b"pass");
                c.bus.write32(COMMS_A + 0x74, seed / 16 % 2);
                // States 2..13 hit all 12 table entries; 0, 1, 14, 15
                // take the out-of-range epilogue. The flag crosses all
                // states (it steers the login arm).
                // States 2..13 hit all 12 table entries; 0, 1, 14, 15
                // take the out-of-range epilogue. The flag crosses all
                // states (it steers the login arm). `a1` is fixed: it is
                // only stored, but the login callees read the word back
                // and would wobble the totals per seed.
                let states = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 0, 1, 14, 15];
                c.bus.write32(this + 4, states[(seed % 16) as usize]);
                c.bus.write32(COMMS_A + 0x74, seed / 16 % 2);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = 0;
                c.g[RA][0] = SENTINEL_RA as u64;
                c.pc = 0x0017_da30;
            },
        );
    }

    /// Shared fixture for the `LobbyMenu_Init*Page` builders: the menu
    /// (`A0`, cleared), the data block (`A1`: null or a cleared
    /// fixture block, `seed / 2 % 2`) and a null callback (`A2`).
    /// Everything runs for real on fixture blocks; the game ROM
    /// provides the label strings (`elf_image`).
    pub(crate) fn init_page_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        let _r = base_machine_ctrl(c, seed, entry as u64);
        elf_image(c);
        install_fakes_2();
        comms(c);
        clear_words(c, COMMS_A, 0x100);
        clear_words(c, COMMS_B, 0x100);
        clear_words(c, LOBBY, 0x100);
        clear_words(c, SELF, 0x710);
        let data = FIX + 0x3000;
        clear_words(c, data, 0x10);
        c.bus.write32(FAKE_CTRL, FAKE_MAGIC3);
        c.g[A0][0] = SELF as u64;
        c.g[A1][0] = if seed / 2 % 2 == 0 { 0 } else { data } as u64;
        c.g[A2][0] = 0;
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = entry;
    }

    #[test]
    fn lobby_menu_debug_printf_stub_matches() {
        check(
            ee::s04::f_001da540__generated,
            lobby_menu_debug_printf_stub,
            SEEDS_7,
            &|c, seed| init_page_fixture(c, seed, 0x001d_a540),
        );
    }

    pub(crate) const SEEDS_8: u32 = 256;

    pub(crate) const FAKE_MAGIC3_2: u32 = 0x0c11_8501;

    pub(crate) const FAKE_COUNT: u32 = FIX + 0xC04;

    pub(crate) const FAKE_STR: u32 = FIX + 0xC08;

    pub(crate) const FAKE_DATA: u32 = FIX + 0x3000;

    /// Gate helper: fall back to the real body unless the fixture magic
    /// word is set, then log the call and return to the caller.
    pub(crate) fn gate(
        c: &mut Cpu,
        slot: u32,
        prev: &'static std::sync::atomic::AtomicUsize,
        generated: fn(&mut Cpu),
    ) -> bool {
        if c.bus.read32(FAKE_CTRL) != FAKE_MAGIC3_2 {
            fakes::fall_back(c, prev, generated);
            return false;
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, slot, ra);
        true
    }

    /// `LobbyMan_GetRoomCount` with a seed-steered count: the real body
    /// counts visible rooms in the lobby list, which the test leaves
    /// cleared (always 0). The count drives the fill loop directly.
    pub(crate) fn room_count_fake(c: &mut Cpu) {
        if !gate(
            c,
            0x0025_f120,
            fakes::prev_cell(room_count_fake),
            ee::s06::f_0025f120__generated,
        ) {
            return;
        }
        if c.pc == c.g[RA][0] as u32 {
            c.g[V0][0] = c.bus.read32(FAKE_COUNT) as u64;
        }
    }

    /// `LobbyMan_GetRoomName` returning the fixture string: the real
    /// body walks the lobby list, which the test leaves cleared.
    pub(crate) fn room_name_fake(c: &mut Cpu) {
        if !gate(
            c,
            0x0025_f150,
            fakes::prev_cell(room_name_fake),
            ee::s06::f_0025f150__generated,
        ) {
            return;
        }
        if c.pc == c.g[RA][0] as u32 {
            c.g[V0][0] = c.bus.read32(FAKE_STR) as u64;
        }
    }

    /// `LobbyMan_GetRoomUserCount` (0x25f1b0) with a seed-steered count:
    /// same shape as the room-count fake. (The fellow map misnamed this
    /// `IS_ROOM_FLAGGED`; the port uses the `functions.tsv` name.)
    pub(crate) fn room_user_count_fake(c: &mut Cpu) {
        if !gate(
            c,
            0x0025_f1b0,
            fakes::prev_cell(room_user_count_fake),
            ee::s06::f_0025f1b0__generated,
        ) {
            return;
        }
        if c.pc == c.g[RA][0] as u32 {
            c.g[V0][0] = c.bus.read32(FAKE_COUNT) as u64;
        }
    }

    /// `LobbyMan_GetRoomUserName` returning the fixture string: same
    /// shape as the room-name fake.
    pub(crate) fn room_user_name_fake(c: &mut Cpu) {
        if !gate(
            c,
            0x0025_f1e0,
            fakes::prev_cell(room_user_name_fake),
            ee::s06::f_0025f1e0__generated,
        ) {
            return;
        }
        if c.pc == c.g[RA][0] as u32 {
            c.g[V0][0] = c.bus.read32(FAKE_STR) as u64;
        }
    }

    /// Fellow `LobbyMenu_*` action callees: each runs a whole login /
    /// persona / room / challenge flow over UI state the test does not
    /// build, so every fixture logs the call and returns. One fake per
    /// address, all with the same shape.
    macro_rules! action_fake {
        ($name:ident, $slot:expr, $gen:path) => {
            fn $name(c: &mut Cpu) {
                gate(c, $slot, fakes::prev_cell($name), $gen);
            }
        };
    }

    action_fake!(
        cancel_challenge_fake,
        0x0017_e558,
        ee::s02::f_0017e558__generated
    );

    action_fake!(
        accept_challenge_fake,
        0x0017_e490,
        ee::s02::f_0017e490__generated
    );

    action_fake!(
        decline_challenge_fake,
        0x0017_e4f0,
        ee::s02::f_0017e4f0__generated
    );

    action_fake!(send_chat_fake, 0x0017_e418, ee::s02::f_0017e418__generated);

    action_fake!(
        leave_occupants_fake,
        0x0017_e368,
        ee::s02::f_0017e368__generated
    );

    action_fake!(
        show_occupant_detail_fake,
        0x0017_e3b8,
        ee::s02::f_0017e3b8__generated
    );

    action_fake!(enter_room_fake, 0x0017_e2f8, ee::s02::f_0017e2f8__generated);

    action_fake!(
        delete_persona_fake,
        0x0017_e228,
        ee::s02::f_0017e228__generated
    );

    action_fake!(
        create_account_fake,
        0x0017_e030,
        ee::s02::f_0017e030__generated
    );

    action_fake!(login_fake, 0x0017_e118, ee::s02::f_0017e118__generated);

    action_fake!(
        select_persona_fake,
        0x0017_e290,
        ee::s02::f_0017e290__generated
    );

    action_fake!(
        create_persona_fake,
        0x0017_e1c0,
        ee::s02::f_0017e1c0__generated
    );

    pub(crate) fn install_fakes_3() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s06::OVERRIDE_0025F120),
            room_count_fake,
            fakes::prev_cell(room_count_fake),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s06::OVERRIDE_0025F150),
            room_name_fake,
            fakes::prev_cell(room_name_fake),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s06::OVERRIDE_0025F1B0),
            room_user_count_fake,
            fakes::prev_cell(room_user_count_fake),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s06::OVERRIDE_0025F1E0),
            room_user_name_fake,
            fakes::prev_cell(room_user_name_fake),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s02::OVERRIDE_0017E558),
            cancel_challenge_fake,
            fakes::prev_cell(cancel_challenge_fake),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s02::OVERRIDE_0017E490),
            accept_challenge_fake,
            fakes::prev_cell(accept_challenge_fake),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s02::OVERRIDE_0017E4F0),
            decline_challenge_fake,
            fakes::prev_cell(decline_challenge_fake),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s02::OVERRIDE_0017E418),
            send_chat_fake,
            fakes::prev_cell(send_chat_fake),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s02::OVERRIDE_0017E368),
            leave_occupants_fake,
            fakes::prev_cell(leave_occupants_fake),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s02::OVERRIDE_0017E3B8),
            show_occupant_detail_fake,
            fakes::prev_cell(show_occupant_detail_fake),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s02::OVERRIDE_0017E2F8),
            enter_room_fake,
            fakes::prev_cell(enter_room_fake),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s02::OVERRIDE_0017E228),
            delete_persona_fake,
            fakes::prev_cell(delete_persona_fake),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s02::OVERRIDE_0017E030),
            create_account_fake,
            fakes::prev_cell(create_account_fake),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s02::OVERRIDE_0017E118),
            login_fake,
            fakes::prev_cell(login_fake),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s02::OVERRIDE_0017E290),
            select_persona_fake,
            fakes::prev_cell(select_persona_fake),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s02::OVERRIDE_0017E1C0),
            create_persona_fake,
            fakes::prev_cell(create_persona_fake),
        );
    }

    driven!(
        lobby_menu_init_room_occupants_page_generated,
        ee::s04::f_001daf08__generated
    );

    driven!(
        lobby_menu_init_room_occupants_page_hand,
        crate::game::fe::festatenethelppopup::lobby_menu_init_room_occupants_page
    );

    driven!(
        lobby_menu_init_chat_page_generated,
        ee::s04::f_001db028__generated
    );

    driven!(
        lobby_menu_init_chat_page_hand,
        crate::game::fe::festatenethelppopup::lobby_menu_init_chat_page
    );

    /// `InitRoomOccupantsPage` fixture: the cleared page, a data block
    /// whose words are stored onto the page, and the second-loop trip
    /// count in `A2` (`seed % 3`). Both loops run every seed (24 fixed
    /// iterations, then `A2 + 1`).
    pub(crate) fn init_occupants_fixture(c: &mut Cpu, seed: u32, entry: u32) -> Events {
        let mut r = base_machine_cleared(c, seed, entry as u64);
        elf_image(c);
        install_fakes_3();
        comms(c);
        clear_words(c, COMMS_A, 0x100);
        clear_words(c, COMMS_B, 0x100);
        clear_words(c, LOBBY, 0x100);
        clear_words(c, FAKE_DATA, 0x10);
        c.bus.write32(FAKE_DATA, 0x1111_0000 + seed);
        c.bus.write32(FAKE_DATA + 4, 0x2222_0000 + seed);
        c.g[A0][0] = SELF as u64;
        c.g[A1][0] = FAKE_DATA as u64;
        c.g[A2][0] = (seed % 3) as u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        schedule(c, &mut r, seed, entry)
    }
    #[test]
    fn lobby_menu_init_room_occupants_page_events_match() {
        check_events(
            lobby_menu_init_room_occupants_page_generated,
            lobby_menu_init_room_occupants_page_hand,
            SEEDS_8,
            &|c, seed| init_occupants_fixture(c, seed, 0x001d_af08),
        );
    }

    /// `InitChatPage` fixture: same shape as the occupants page; both
    /// loops run a fixed 14 iterations every seed.
    pub(crate) fn init_chat_fixture(c: &mut Cpu, seed: u32, entry: u32) -> Events {
        let mut r = base_machine_cleared(c, seed, entry as u64);
        elf_image(c);
        install_fakes_3();
        comms(c);
        clear_words(c, COMMS_A, 0x100);
        clear_words(c, COMMS_B, 0x100);
        clear_words(c, LOBBY, 0x100);
        clear_words(c, FAKE_DATA, 0x10);
        c.bus.write32(FAKE_DATA, 0x3333_0000 + seed);
        c.g[A0][0] = SELF as u64;
        c.g[A1][0] = FAKE_DATA as u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        schedule(c, &mut r, seed, entry)
    }

    #[test]
    fn lobby_menu_init_chat_page_events_match() {
        check_events(
            lobby_menu_init_chat_page_generated,
            lobby_menu_init_chat_page_hand,
            SEEDS_8,
            &|c, seed| init_chat_fixture(c, seed, 0x001d_b028),
        );
    }

    pub(crate) const FAKE_FLAG_ID: u32 = FIX + 0xC04;

    pub(crate) const FAKE_CHAR: u32 = FIX + 0xC0C;

    pub(crate) const FAKE_INBOX: u32 = FIX + 0x3400;

    pub(crate) const STR: u32 = FIX + 0x1000;

    pub(crate) fn base_machine_cleared(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = alloc_base_bx(c, seed, salt);
        be_library_bx(c);
        fill(c, &mut r, SELF, 0x2000);
        fill(c, &mut r, FIX, 0x1000);
        clear_words(c, SELF, 0x710);
        clear_words(c, FAKE_CTRL, 0x10);
        r
    }

    pub(crate) fn alloc(c: &mut Cpu, size: u32) -> u32 {
        str(c, STR, b"oc");
        c.g[A0][0] = size as u64;
        c.g[A1][0] = STR as u64;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        ee::s10::f_00317d70(c);
        c.g[V0][0] as u32
    }
}
