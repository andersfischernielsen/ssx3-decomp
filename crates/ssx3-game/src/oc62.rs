//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for small front-end, memory-card, movie-player, lobby,
//! script-engine and network wrappers of shards s03-s08, each with a
//! differential test against the generated body.
//!
//! Nearly all of these are forwarding wrappers: open the compiler's 0x10/0x20
//! frame, save `ra` (and `s0` where the argument is needed twice), charge the
//! block's cycles, load a member object or a vtable, and call one method. The
//! ports keep the frame, the cycle charges of every block, the scratch values
//! the generated code leaves in `v0`/`v1`/`a*` at each call, the callee-saved
//! `s0`, and the unwind check through [`crate::call`].
//!
//! Two groups are not wrappers:
//! - `SSXAudioSystem_IsGameMusicEnabled` (already replaced in `oc59`) is a pure
//!   leaf; it is deliberately **not** repeated here.
//! - `cFEStateNetConfig_Shutdown`, `OVScreenMap_*` and the script-engine
//!   helpers chain two or three real calls; their tests set the BE-library and
//!   world globals up as the guide describes.

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Frame, VSlot};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    // Front end: preview characters and the state-machine hooks.
    (0x001a_0508, fe_module_request_preview_char, "FEModule_RequestPreviewChar"),
    (0x001a_dfb8, c_fe_state_challenge_popup_transition_in, "cFEStateChallengePopup_TransitionIn"),
    (0x001b_4168, c_fe_state_net_config_shutdown, "cFEStateNetConfig_Shutdown"),
    (0x001b_6740, net_config_auth_popup_v24, "NetConfigAuthPopup_v24"),
    (0x001b_6e10, c_fe_state_auth_dnas_error_v24, "cFEStateAuthDNASError_v24"),
    (0x001b_8860, c_fe_state_net_account_mem_v24, "cFEStateNetAccountMem_v24"),
    (0x001b_e510, c_fe_state_lobby_menu_refresh_player_list, "cFEStateLobbyMenu_RefreshPlayerList"),
    (0x001b_ed28, c_fe_state_net_create_room_popup_v31, "cFEStateNetCreateRoomPopup_v31"),
    (0x001c_a4f8, c_fe_popup_confirm_v24, "cFEPopupConfirm_v24"),
    (0x001d_9d18, c_fe_popup_confirm_v25, "cFEPopupConfirm_v25"),
    (0x001d_a478, c_fe_state_net_help_popup_on_widget_action, "cFEStateNetHelpPopup_OnWidgetAction"),
    (0x001e_0fc0, c_fe_state_net_messenger_transition_in, "cFEStateNetMessenger_TransitionIn"),
    // Overlay screens.
    (0x0020_8558, ov_screen_map_on_activate, "OVScreenMap_OnActivate"),
    (0x0020_8678, ov_screen_map_update, "OVScreenMap_Update"),
    // Luno tables and the crowd manager.
    (0x0022_4da0, c_luno_table_ctor, "cLunoTable_ctor"),
    (0x0022_9498, crowd_man2d_clear, "CrowdMan2d_Clear"),
    // Game and section managers.
    (0x0022_cce8, section_man_ctor, "SectionMan_ctor"),
    (0x0023_3aa0, game_state_goto_next, "GameState_GotoNext"),
    (0x0023_6198, game_state_warp_to_event_update, "GameStateWarpToEvent_Update"),
    // Memory-card manager state hooks.
    (0x0023_ca70, mc_manager_card_set_state_1e, "MCManager_CardSetState1E"),
    (0x0023_d5e8, mc_manager_destroy_card, "MCManager_DestroyCard"),
    (0x0023_eb50, mc_manager_set_state_no_card, "MCManager_SetStateNoCard"),
    (0x0023_f2d8, mc_manager_state_13, "MCManager_State13"),
    (0x0023_ffc0, mc_manager_state_23, "MCManager_State23"),
    (0x0024_0c90, mc_manager_state_2a, "MCManager_State2A"),
    (0x0024_0eb0, mc_manager_state_2d, "MCManager_State2D"),
    (0x0024_0ee0, mc_manager_state_2e, "MCManager_State2E"),
    (0x0024_10a0, mc_manager_close_card, "MCManager_CloseCard"),
    // Vfx process hooks.
    (0x0024_4478, vfx_process_update, "VfxProcess_Update"),
    (0x0024_44a8, vfx_process_render, "VfxProcess_Render"),
    // Video, lobby and setup managers.
    (0x0025_3860, vid_engine_player_play, "VidEnginePlayer_Play"),
    (0x0025_f150, lobby_man_get_room_name, "LobbyMan_GetRoomName"),
    (0x0026_4960, c_setup_man_alloc_packet, "cSetupMan_AllocPacket"),
    // Shared login manager card slots.
    (0x0026_5a38, c_shared_login_man_card_slot20, "cSharedLoginMan_CardSlot20"),
    (0x0026_5d08, c_shared_login_man_card_slot47, "cSharedLoginMan_CardSlot47"),
    (0x0026_5d38, c_shared_login_man_card_slot63, "cSharedLoginMan_CardSlot63"),
    (0x0026_6f90, c_voip_man_get_status, "cVoipMan_GetStatus"),
    // Path data, playback, replay.
    (0x0026_ad70, path_data_set_buffer, "PathData_SetBuffer"),
    (0x0026_cc18, playback_man_restore_world_state, "PlaybackMan_RestoreWorldState"),
    (0x0026_e670, snapshot_list_pop_front, "SnapshotList_PopFront"),
    // Script engine.
    (0x0027_89e0, script_engine_stop_lists, "ScriptEngine_StopLists"),
    (0x0027_8f38, script_engine_list_prepare, "ScriptEngine_ListPrepare"),
    (0x0027_9040, script_engine_list_set_paused, "ScriptEngine_ListSetPaused"),
    (0x0027_9148, script_engine_list_get_entry_type, "ScriptEngine_ListGetEntryType"),
    (0x0027_d7f0, script_obj_cam_v02, "ScriptObjCam_v02"),
    (0x0027_d820, script_obj_cam_v03, "ScriptObjCam_v03"),
    (0x0028_2f00, c_transition_man_script_pause, "cTransitionMan_ScriptPause"),
    (0x0028_2f30, c_transition_man_script_resume, "cTransitionMan_ScriptResume"),
];

// ---------------------------------------------------------------------------
// Callees (the generated crate's readable aliases, which dispatch through
// their override slot).

mod callee {
    use super::{ee, Callee};

    pub const FE_MODULE_RIDER_PREVIEW_REQUEST_CHAR: Callee = Callee {
        addr: 0x0019_e588,
        f: ee::s03::FEModule_RiderPreviewRequestChar,
    };
    pub const C_FE_STATE_LOBBY_MENU_FILL_PLAYER_LIST: Callee = Callee {
        addr: 0x001b_e540,
        f: ee::s03::cFEStateLobbyMenu_FillPlayerList,
    };
    pub const C_FE_STATE_LOBBY_MENU_UPDATE_PLAYER_SCROLL_ARROWS: Callee = Callee {
        addr: 0x001b_e720,
        f: ee::s03::cFEStateLobbyMenu_UpdatePlayerScrollArrows,
    };
    pub const EVENT_SELECT_ON_ACTIVATE: Callee = Callee {
        addr: 0x0020_0ac0,
        f: ee::s05::EventSelect_OnActivate,
    };
    pub const EVENT_SELECT_UPDATE: Callee = Callee {
        addr: 0x0020_2828,
        f: ee::s05::EventSelect_Update,
    };
    pub const C_OV_STATE_OVERLAY_UPDATE: Callee = Callee {
        addr: 0x0020_e900,
        f: ee::s05::cOVStateOverlay_Update,
    };
    pub const C_LUNO_TABLE_ALLOC: Callee = Callee {
        addr: 0x0022_4e50,
        f: ee::s05::cLunoTable_Alloc,
    };
    pub const CROWD_MAN2D_REMOVE_ALL: Callee = Callee {
        addr: 0x0022_9398,
        f: ee::s05::CrowdMan2d_RemoveAll,
    };
    pub const CROWD_MAN2D_RESET: Callee = Callee {
        addr: 0x0022_92e0,
        f: ee::s05::CrowdMan2d_Reset,
    };
    pub const SECTION_MAN_INIT: Callee = Callee {
        addr: 0x0022_cd40,
        f: ee::s05::SectionMan_Init,
    };
    pub const GAME_MODULE_REQUEST_STATE: Callee = Callee {
        addr: 0x0023_1250,
        f: ee::s06::GameModule_RequestState,
    };
    pub const VID_ENGINE_PLAYER_START_PLAYBACK: Callee = Callee {
        addr: 0x0025_36c8,
        f: ee::s06::VidEnginePlayer_StartPlayback,
    };
    pub const SCRIPT_LIST_PREPARE: Callee = Callee {
        addr: 0x0027_6270,
        f: ee::s07::ScriptList_Prepare,
    };
    pub const SCRIPT_LIST_SET_PAUSED_NESTED: Callee = Callee {
        addr: 0x0027_66b0,
        f: ee::s07::ScriptList_SetPausedNested,
    };
    pub const SCRIPT_LIST_GET_ENTRY_TYPE: Callee = Callee {
        addr: 0x0027_6998,
        f: ee::s07::ScriptList_GetEntryType,
    };
    pub const SCRIPT_ENGINE_STOP_MAIN_LISTS: Callee = Callee {
        addr: 0x0027_90f8,
        f: ee::s07::ScriptEngine_StopMainLists,
    };
    pub const SCRIPT_ENGINE_STOP_AUX_LISTS: Callee = Callee {
        addr: 0x0027_9488,
        f: ee::s07::ScriptEngine_StopAuxLists,
    };
    pub const SNAPSHOT_LIST_REMOVE: Callee = Callee {
        addr: 0x0026_e608,
        f: ee::s07::SnapshotList_Remove,
    };
    pub const PATH_DATA_PARSE: Callee = Callee {
        addr: 0x0026_adf0,
        f: ee::s07::PathData_Parse,
    };
    pub const WORLD_STATE_RESTORE: Callee = Callee {
        addr: 0x0026_dbd0,
        f: ee::s07::WorldState_Restore,
    };
    pub const EA_MEM_ALLOC_NAMED: Callee = Callee {
        addr: 0x0031_7d70,
        f: ee::s10::EA_MemAllocNamed,
    };
}

/// Cross-shard calls the generator emitted as `call_indirect` (no alias).
mod table {
    pub const UI_STATE_ON_ACTIVATE: u32 = 0x0039_e4c0;
    pub const UI_STATE_SHUTDOWN: u32 = 0x0039_e4a0;
    pub const UI_LIST_SET_FIRST_FLAGS: u32 = 0x0039_f190;
    pub const SSX_AUDIO_SYSTEM_GET_INSTANCE: u32 = 0x0028_b180;
    pub const SSX_AUDIO_SYSTEM_SET_GAME_STATE: u32 = 0x0028_fc58;
    pub const LOBBY_LIST_GET_NAME: u32 = 0x003e_e560;
    pub const VOIP_RPC_CALL: u32 = 0x003f_6ba0;
    pub const MC_CARD_DESTROY: u32 = 0x002c_2300;
}

/// `lw` at `base + off` (sign-extended).
#[inline(always)]
fn lw_at(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr((base as u32).wrapping_add(off)).lw(c)
}

// ---------------------------------------------------------------------------
// Front end: preview characters.

/// `FEModule_RequestPreviewChar(module, rider, a2, a3)` 0x1a0508: forward to
/// rider `idx`'s preview object (`module + idx * 0xce0`), passing `a2`/`a3` as
/// the request's character and flag.
pub fn fe_module_request_preview_char(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let module = abi::arg(c, 0);
        let idx = abi::arg(c, 1);
        // The arguments are moved, not truncated: `a1 = a2; a2 = a3`.
        let a2 = abi::arg64(c, 2);
        let a3 = abi::arg64(c, 3);
        c.g[V0][0] = 0xce0;
        let frame = Frame::open(c, 0x10);
        // `mult v1, a1, v0`: the byte stride to the rider's preview object.
        let stride = abi::mult(c, idx, 0xce0);
        c.g[V1][0] = sx(stride);
        frame.save_ra(c, 0);
        c.g[A1][0] = a2;
        c.g[A2][0] = a3;
        c.g[A0][0] = sx(module.wrapping_add(stride));
        callee::FE_MODULE_RIDER_PREVIEW_REQUEST_CHAR.call(c, 0x001a_0528)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end state hooks: virtual methods on a member object.

/// `cFEStateChallengePopup_TransitionIn(this)` 0x1adfb8: slot +0x100 of the
/// object's vtable at `*(this + 8)`, then `true`.
pub fn c_fe_state_challenge_popup_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        let vt = lw_at(c, this as u64, 8);
        c.g[V0][0] = vt;
        let m = VSlot::read(c, vt, 0x100);
        c.g[V1][0] = m.delta;
        c.g[A1][0] = m.func;
        c.g[A0][0] = sx(this.wrapping_add(m.delta as u32));
        call_ptr(c, m.func as u32, 0x001a_dfd4)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 1;
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateNetMessenger_TransitionIn(this)` 0x1e0fc0: same shape as the
/// challenge popup's transition-in (vtable at `*(this + 8)`, slot +0x100).
pub fn c_fe_state_net_messenger_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        let vt = lw_at(c, this as u64, 8);
        c.g[V0][0] = vt;
        let m = VSlot::read(c, vt, 0x100);
        c.g[V1][0] = m.delta;
        c.g[A1][0] = m.func;
        c.g[A0][0] = sx(this.wrapping_add(m.delta as u32));
        call_ptr(c, m.func as u32, 0x001e_0fdc)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 1;
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateNetConfig_Shutdown(this)` 0x1b4168: tear down the state's widget
/// list, then hand the audio system's singleton to its game-state teardown.
pub fn c_fe_state_net_config_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, table::UI_STATE_SHUTDOWN, 0x001b_4178)?;
        call_ptr(c, table::SSX_AUDIO_SYSTEM_GET_INSTANCE, 0x001b_4180)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, table::SSX_AUDIO_SYSTEM_SET_GAME_STATE, 0x001b_4188)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The `OnWidgetAction` hooks: `A1` is forwarded from the caller, the method
/// is slot +0xc0 of the vtable at `*(*(this + 0x20) + 8)`.
fn widget_action_v24(c: &mut Cpu, ret: u32) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        let obj = lw_at(c, this as u64, 0x20);
        c.g[A3][0] = obj;
        let vt = lw_at(c, obj, 8);
        c.g[V1][0] = vt;
        let m = VSlot::read(c, vt, 0xc0);
        c.g[A0][0] = m.delta;
        c.g[V0][0] = m.func;
        c.g[A0][0] = sx((obj as u32).wrapping_add(m.delta as u32));
        call_ptr(c, m.func as u32, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `NetConfigAuthPopup_v24(this, a1, a2)` 0x1b6740: forward the widget action.
pub fn net_config_auth_popup_v24(c: &mut Cpu) {
    widget_action_v24(c, 0x001b_6760)
}

/// `cFEStateAuthDNASError_v24(this, a1, a2)` 0x1b6e10: forward the widget
/// action.
pub fn c_fe_state_auth_dnas_error_v24(c: &mut Cpu) {
    widget_action_v24(c, 0x001b_6e30)
}

/// `cFEStateNetAccountMem_v24(this, a1, a2)` 0x1b8860: forward the widget
/// action.
pub fn c_fe_state_net_account_mem_v24(c: &mut Cpu) {
    widget_action_v24(c, 0x001b_8880)
}

/// `cFEStateNetCreateRoomPopup_v31(this, a1, a2)` 0x1bed28: forward the widget
/// action.
pub fn c_fe_state_net_create_room_popup_v31(c: &mut Cpu) {
    widget_action_v24(c, 0x001b_ed48)
}

/// `cFEPopupConfirm_v24(this, a1, a2)` 0x1ca4f8: forward the widget action.
pub fn c_fe_popup_confirm_v24(c: &mut Cpu) {
    widget_action_v24(c, 0x001c_a518)
}

/// `cFEStateLobbyMenu_RefreshPlayerList(this)` 0x1be510: rebuild the lobby's
/// player list, then refresh the scroll arrows.
pub fn c_fe_state_lobby_menu_refresh_player_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg64(c, 0);
        c.g[S0][0] = this;
        callee::C_FE_STATE_LOBBY_MENU_FILL_PLAYER_LIST.call(c, 0x001b_e524)?;
        c.g[A0][0] = c.g[S0][0];
        callee::C_FE_STATE_LOBBY_MENU_UPDATE_PLAYER_SCROLL_ARROWS.call(c, 0x001b_e52c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEPopupConfirm_...` (v24) 0x1d9d18: store `a1` at +0x168 and, when the
/// popup has a live object at +0x40, apply its layout (`cFEStateNetHelpPopup`'s
/// sibling hook).
pub fn c_fe_popup_confirm_v25(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        let value = c.g[A1][0];
        c.sw(this.wrapping_add(0x168), value);
        let obj = c.lw(this.wrapping_add(0x40));
        c.g[V0][0] = obj;
        let idle = obj == 0;
        frame.restore_ra(c, 0);
        if !idle {
            c.cycles += 3;
            call_ptr(c, 0x001c_5dd8, 0x001d_9d38)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateNetHelpPopup_OnWidgetAction(this, a1, msg)` 0x1da478: on a
/// "confirm" (message 6) put the popup's first-list entry into the pressed
/// state.
pub fn c_fe_state_net_help_popup_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 6;
        let confirm = c.g[A2][0] == 6;
        frame.save_ra(c, 0);
        if confirm {
            c.cycles += 4;
            let this = abi::arg(c, 0);
            let obj = lw_at(c, this as u64, 0x10);
            c.g[A0][0] = obj;
            c.g[A1][0] = 1;
            c.g[A0][0] = sx((obj as u32).wrapping_add(0x18));
            call_ptr(c, table::UI_LIST_SET_FIRST_FLAGS, 0x001d_a498)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Overlay screens.

/// `OVScreenMap_OnActivate(this)` 0x208558: activate the screen's widget list
/// and switch the event select into map mode.
pub fn ov_screen_map_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg64(c, 0);
        c.g[S0][0] = this;
        call_ptr(c, table::UI_STATE_ON_ACTIVATE, 0x0020_856c)?;
        c.g[A0][0] = sx((this as u32).wrapping_add(0x9c));
        callee::EVENT_SELECT_ON_ACTIVATE.call(c, 0x0020_8574)?;
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenMap_Update(this)` 0x208678: update the overlay's own state, then
/// the event-select widget embedded at +0x9c.
pub fn ov_screen_map_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg64(c, 0);
        c.g[S0][0] = this;
        callee::C_OV_STATE_OVERLAY_UPDATE.call(c, 0x0020_868c)?;
        c.g[A0][0] = sx((this as u32).wrapping_add(0x9c));
        callee::EVENT_SELECT_UPDATE.call(c, 0x0020_8694)?;
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Luno table and crowd manager.

/// `cLunoTable_ctor(mem, a1)` 0x224da0: clear the table pointer, build the
/// table, return the object.
pub fn c_luno_table_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg64(c, 0);
        c.g[S0][0] = this;
        c.sw(this as u32, 0);
        callee::C_LUNO_TABLE_ALLOC.call(c, 0x0022_4db8)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `CrowdMan2d_Clear(this)` 0x229498: remove every slot and reset the manager.
pub fn crowd_man2d_clear(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg64(c, 0);
        c.g[S0][0] = this;
        callee::CROWD_MAN2D_REMOVE_ALL.call(c, 0x0022_94ac)?;
        c.g[A0][0] = c.g[S0][0];
        callee::CROWD_MAN2D_RESET.call(c, 0x0022_94b4)?;
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Game and section managers.

/// `SectionMan_ctor(p, game)` 0x22cce8: remember the game object at +0x1b0 and
/// initialise the section tables; returns `p`.
pub fn section_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg64(c, 0);
        c.g[S0][0] = this;
        let game = c.g[A1][0];
        c.sw((this as u32).wrapping_add(0x1b0), game);
        callee::SECTION_MAN_INIT.call(c, 0x0022_cd00)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `GameState_GotoNext(state)` 0x233aa0: if the state names a target state
/// (`*(state) != 0`), ask the game module for it with the delay at +4.
pub fn game_state_goto_next(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        let target = c.lw(this);
        c.g[A1][0] = target;
        let none = target == 0;
        c.g[A3][0] = 0;
        if !none {
            c.cycles += 3;
            c.g[A2][0] = lw_at(c, this as u64, 4);
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            callee::GAME_MODULE_REQUEST_STATE.call(c, 0x0023_3ac0)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `GameStateWarpToEvent_Update()` 0x236198: request the game module's race
/// state (`world->+0x84`, param 1).
pub fn game_state_warp_to_event_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        let world = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[V0][0] = world;
        frame.save_ra(c, 0);
        c.g[A1][0] = 1;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A0][0] = lw_at(c, world, 0x84);
        callee::GAME_MODULE_REQUEST_STATE.call(c, 0x0023_61b8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Memory-card manager state hooks.

/// `MCManager_CardSetState1E(this)` 0x23ca70: call entry 0x20 of the card
/// object's vtable (`*(this + 0x434)`).
pub fn mc_manager_card_set_state_1e(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        let card = lw_at(c, this as u64, 0x434);
        c.g[A1][0] = card;
        let vt = c.lw(card as u32);
        c.g[V0][0] = vt;
        let m = VSlot::read(c, vt, 0x100);
        c.g[A0][0] = m.delta;
        c.g[V1][0] = m.func;
        c.g[A0][0] = sx((card as u32).wrapping_add(m.delta as u32));
        call_ptr(c, m.func as u32, 0x0023_ca90)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MCManager_DestroyCard(this)` 0x23d5e8: destroy the active card object and
/// forget it.
pub fn mc_manager_destroy_card(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg64(c, 0);
        c.g[S0][0] = this;
        c.g[A0][0] = c.lw((this as u32).wrapping_add(0x434));
        call_ptr(c, table::MC_CARD_DESTROY, 0x0023_d600)?;
        c.sw((this as u32).wrapping_add(0x434), 0);
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The `MCManager_State*` entry wrappers: pass `state` as the second argument
/// to the vtable method at slot +8 of `*(this + 0x748)` (the card state
/// machine).
fn mc_manager_state_entry(c: &mut Cpu, state: u64, ret: u32) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = state;
        let this = abi::arg(c, 0);
        frame.save_ra(c, 0);
        let vt = lw_at(c, this as u64, 0x748);
        c.g[V0][0] = vt;
        let m = VSlot::read(c, vt, 8);
        c.g[V1][0] = m.delta;
        c.g[A2][0] = m.func;
        c.g[A0][0] = sx(this.wrapping_add(m.delta as u32));
        call_ptr(c, m.func as u32, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MCManager_SetStateNoCard(this)` 0x23eb50: request state 4.
pub fn mc_manager_set_state_no_card(c: &mut Cpu) {
    mc_manager_state_entry(c, 4, 0x0023_eb70)
}

/// `MCManager_State13(this)` 0x23f2d8: request state 1.
pub fn mc_manager_state_13(c: &mut Cpu) {
    mc_manager_state_entry(c, 1, 0x0023_f2f8)
}

/// `MCManager_State23(this)` 0x23ffc0: request state 1.
pub fn mc_manager_state_23(c: &mut Cpu) {
    mc_manager_state_entry(c, 1, 0x0023_ffe0)
}

/// `MCManager_State2A(this)` 0x240c90: request state 6.
pub fn mc_manager_state_2a(c: &mut Cpu) {
    mc_manager_state_entry(c, 6, 0x0024_0cb0)
}

/// `MCManager_State2D(this)` 0x240eb0: request state 1.
pub fn mc_manager_state_2d(c: &mut Cpu) {
    mc_manager_state_entry(c, 1, 0x0024_0ed0)
}

/// `MCManager_State2E(this)` 0x240ee0: request state 6.
pub fn mc_manager_state_2e(c: &mut Cpu) {
    mc_manager_state_entry(c, 6, 0x0024_0f00)
}

/// `MCManager_CloseCard(this)` 0x2410a0: call slot +0x60 of the vtable at
/// `*(*(this + 0x434))` (the card's close method).
pub fn mc_manager_close_card(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        let card = lw_at(c, this as u64, 0x434);
        c.g[A1][0] = card;
        let vt = c.lw(card as u32);
        c.g[V0][0] = vt;
        let m = VSlot::read(c, vt, 0x60);
        c.g[A0][0] = m.delta;
        c.g[V1][0] = m.func;
        c.g[A0][0] = sx((card as u32).wrapping_add(m.delta as u32));
        call_ptr(c, m.func as u32, 0x0024_10c0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Vfx process hooks.

/// The `VfxProcess` hooks take the process object at `*(this + 0x10)`, whose
/// vtable lives at `+0x44`.
fn vfx_process_vcall(c: &mut Cpu, slot: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        let obj = lw_at(c, this as u64, 0x10);
        c.g[A1][0] = obj;
        let vt = lw_at(c, obj, 0x44);
        c.g[V0][0] = vt;
        let m = VSlot::read(c, vt, slot);
        c.g[A0][0] = m.delta;
        c.g[V1][0] = m.func;
        c.g[A0][0] = sx((obj as u32).wrapping_add(m.delta as u32));
        call_ptr(c, m.func as u32, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `VfxProcess_Update(this)` 0x244478: slot +0x10 of the process vtable.
pub fn vfx_process_update(c: &mut Cpu) {
    vfx_process_vcall(c, 0x10, 0x0024_4498)
}

/// `VfxProcess_Render(this)` 0x2444a8: slot +0x18 of the process vtable.
pub fn vfx_process_render(c: &mut Cpu) {
    vfx_process_vcall(c, 0x18, 0x0024_44c8)
}

// ---------------------------------------------------------------------------
// Video, lobby and setup managers.

/// `VidEnginePlayer_Play(this)` 0x253860: start playback when a video file is
/// queued (+0x38); reports whether it started.
pub fn vid_engine_player_play(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        let file = lw_at(c, this as u64, 0x38);
        c.g[V0][0] = file;
        let none = file == 0;
        c.g[V0][0] = 0;
        if !none {
            c.cycles += 3;
            callee::VID_ENGINE_PLAYER_START_PLAYBACK.call(c, 0x0025_387c)?;
            c.g[V0][0] = 1;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMan_GetRoomName(net, i)` 0x25f150: the room name of the session in
/// the lobby list (`*(net + 0x5c)`), or null when the lobby is not up.
pub fn lobby_man_get_room_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let net = abi::arg(c, 0);
        let up = c.lw(net);
        c.g[V0][0] = up;
        if up == 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 3;
            c.g[A0][0] = lw_at(c, net as u64, 0x5c);
            call_ptr(c, table::LOBBY_LIST_GET_NAME, 0x0025_f16c)?;
            let name = c.g[V0][0];
            c.g[V0][0] = sx((name as u32).wrapping_add(0x1c));
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cSetupMan_AllocPacket()` 0x264960: allocate a 0xf0-byte setup packet from
/// the game heap.
pub fn c_setup_man_alloc_packet(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x48_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = 0xf0;
        c.g[A1][0] = sx(0x48_05a8);
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        callee::EA_MEM_ALLOC_NAMED.call(c, 0x0026_4980)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Shared login manager: card slots (virtual getters on the login object at
// `*(gp + 0x2e8)`).

/// `cSharedLoginMan_CardSlot20(this)` 0x265a38: slot +0xa0 of the login
/// object's vtable.
pub fn c_shared_login_man_card_slot20(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2e8));
        frame.save_ra(c, 0);
        let obj = c.lw((c.g[A1][0] as u32));
        c.g[V1][0] = obj;
        let m = VSlot::read(c, obj, 0xa0);
        c.g[A0][0] = m.delta;
        c.g[V0][0] = m.func;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(m.delta as u32));
        call_ptr(c, m.func as u32, 0x0026_5a58)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cSharedLoginMan_CardSlot47(this)` 0x265d08: slot +0x178 of the login
/// object's vtable.
pub fn c_shared_login_man_card_slot47(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2e8));
        frame.save_ra(c, 0);
        let vt = c.lw((c.g[A1][0] as u32));
        c.g[V0][0] = vt;
        let m = VSlot::read(c, vt, 0x178);
        c.g[A0][0] = m.delta;
        c.g[V1][0] = m.func;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(m.delta as u32));
        call_ptr(c, m.func as u32, 0x0026_5d28)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cSharedLoginMan_CardSlot63(this)` 0x265d38: slot +0x1f8 of the login
/// object's vtable.
pub fn c_shared_login_man_card_slot63(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2e8));
        frame.save_ra(c, 0);
        let vt = c.lw((c.g[A1][0] as u32));
        c.g[V1][0] = vt;
        let m = VSlot::read(c, vt, 0x1f8);
        c.g[A0][0] = m.delta;
        c.g[V0][0] = m.func;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(m.delta as u32));
        call_ptr(c, m.func as u32, 0x0026_5d58)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cVoipMan_GetStatus(self, a, b)` 0x266f90: when the voip manager is up,
/// copy the RPC status into `a` and `b`.
pub fn c_voip_man_get_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        let up = c.lw(this);
        c.g[V0][0] = up;
        // Both pointer arguments are passed through as whole registers.
        let a = abi::arg64(c, 1);
        let b = abi::arg64(c, 2);
        c.g[A0][0] = a;
        let idle = up == 0;
        c.g[A1][0] = b;
        if !idle {
            c.cycles += 2;
            call_ptr(c, table::VOIP_RPC_CALL, 0x0026_6fb0)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Path data, playback and the replay snapshot list.

/// `PathData_SetBuffer(self, v)` 0x26ad70: remember the path blob and re-parse
/// it; always reports success.
pub fn path_data_set_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        let this64 = abi::arg64(c, 0);
        c.g[V0][0] = this64;
        let this = this64 as u32;
        frame.save_ra(c, 0);
        c.g[V1][0] = 1;
        let blob = c.g[A1][0];
        c.sw(this.wrapping_add(0x48), blob);
        c.sw(this.wrapping_add(0x4c), 1);
        callee::PATH_DATA_PARSE.call(c, 0x0026_ad8c)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 1;
        frame.close(c);
        Ok(())
    })
}

/// `PlaybackMan_RestoreWorldState(this)` 0x26cc18: restore the saved world
/// state when a snapshot is pending at +0x10.
pub fn playback_man_restore_world_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        let pending = lw_at(c, this as u64, 0x10);
        c.g[V0][0] = pending;
        c.g[A1][0] = sx(this.wrapping_add(0x14));
        if pending != 0 {
            c.cycles += 3;
            let stream = lw_at(c, this as u64, 0x5c);
            c.g[V0][0] = stream;
            c.g[A0][0] = lw_at(c, stream, 0x28);
            callee::WORLD_STATE_RESTORE.call(c, 0x0026_cc38)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SnapshotList_PopFront(list)` 0x26e670: unlink and return the list's first
/// snapshot node.
pub fn snapshot_list_pop_front(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let list = abi::arg(c, 0);
        let node = c.lw(list);
        c.g[S0][0] = node;
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[S0][0];
        callee::SNAPSHOT_LIST_REMOVE.call(c, 0x0026_e688)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script engine.

/// `ScriptEngine_StopLists(engine)` 0x2789e0: stop the main and auxiliary
/// script lists.
pub fn script_engine_stop_lists(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let engine = abi::arg64(c, 0);
        c.g[S0][0] = engine;
        callee::SCRIPT_ENGINE_STOP_MAIN_LISTS.call(c, 0x0027_89f4)?;
        c.g[A0][0] = c.g[S0][0];
        callee::SCRIPT_ENGINE_STOP_AUX_LISTS.call(c, 0x0027_89fc)?;
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The `ScriptEngine_List*` helpers: address list `i` of the engine's rider
/// script array (`*(engine + 0x54c)` + `i * 0xcc`).
fn script_list_call(c: &mut Cpu, ret: u32, second: Option<u64>, list: Callee) {
    guest(c, |c| {
        c.cycles += 11;
        let engine = abi::arg(c, 0);
        let index = abi::arg(c, 1);
        c.g[V0][0] = 0xcc;
        let frame = Frame::open(c, 0x10);
        let stride = abi::mult(c, index, 0xcc);
        c.g[V1][0] = sx(stride);
        frame.save_ra(c, 0);
        match second {
            Some(v) => c.g[A1][0] = v,
            None => c.g[A1][0] = 0,
        }
        let base = c.lw(engine.wrapping_add(0x54c));
        c.g[A0][0] = sx((base as u32).wrapping_add(stride));
        list.call(c, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ScriptEngine_ListPrepare(engine, i)` 0x278f38: prepare list `i`.
pub fn script_engine_list_prepare(c: &mut Cpu) {
    script_list_call(c, 0x0027_8f58, None, callee::SCRIPT_LIST_PREPARE)
}

/// `ScriptEngine_ListSetPaused(engine, i, paused)` 0x279040: pause/resume list
/// `i` (and nested lists).
pub fn script_engine_list_set_paused(c: &mut Cpu) {
    let second = abi::arg64(c, 2);
    script_list_call(
        c,
        0x0027_9060,
        Some(second),
        callee::SCRIPT_LIST_SET_PAUSED_NESTED,
    )
}

/// `ScriptEngine_ListGetEntryType(engine, i, a)` 0x279148: entry type of list
/// `i`'s entry `a`.
pub fn script_engine_list_get_entry_type(c: &mut Cpu) {
    let second = abi::arg64(c, 2);
    script_list_call(
        c,
        0x0027_9168,
        Some(second),
        callee::SCRIPT_LIST_GET_ENTRY_TYPE,
    )
}

/// The `ScriptObjCam`/`cTransitionMan` script hooks: a virtual method on the
/// script binding at `*(this + 0x10)`, whose vtable is at `*(binding + 0xc)`.
fn script_binding_vcall(c: &mut Cpu, slot: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        let binding = lw_at(c, this as u64, 0x10);
        c.g[A1][0] = binding;
        let vt = lw_at(c, binding, 0xc);
        c.g[V0][0] = vt;
        let m = VSlot::read(c, vt, slot);
        c.g[A0][0] = m.delta;
        c.g[V1][0] = m.func;
        c.g[A0][0] = sx((binding as u32).wrapping_add(m.delta as u32));
        call_ptr(c, m.func as u32, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ScriptObjCam_v02(this)` 0x27d7f0: slot +0x10 of the script binding's
/// vtable (script camera stop).
pub fn script_obj_cam_v02(c: &mut Cpu) {
    script_binding_vcall(c, 0x10, 0x0027_d810)
}

/// `ScriptObjCam_v03(this)` 0x27d820: slot +0x18 of the script binding's
/// vtable (script camera start).
pub fn script_obj_cam_v03(c: &mut Cpu) {
    script_binding_vcall(c, 0x18, 0x0027_d840)
}

/// `cTransitionMan_ScriptPause(this)` 0x282f00: slot +0x10 of the script
/// binding's vtable.
pub fn c_transition_man_script_pause(c: &mut Cpu) {
    script_binding_vcall(c, 0x10, 0x0028_2f20)
}

/// `cTransitionMan_ScriptResume(this)` 0x282f30: slot +0x18 of the script
/// binding's vtable.
pub fn c_transition_man_script_resume(c: &mut Cpu) {
    script_binding_vcall(c, 0x18, 0x0028_2f50)
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, vtable, LOG_RESULTS};
    use crate::difftest::{check, Rng, DATA, GAME_GP};

    /// A non-null, zeroed stand-in for the BE-library singleton at
    /// `gp - 0x1f04` (its lazy constructor would otherwise load from disc).
    const BE_LIBRARY: u32 = DATA + 0x1_f000;
    /// The memory-card/setup heap mutex the allocator waits on.
    const MEM_MUTEX: u32 = 0x0053_8b00;

    /// Globals most of these wrappers read: the BE library, and a small zeroed
    /// world (`world -> +0x84 -> +0xc`, rider count at `+0x7c`).
    fn globals(c: &mut Cpu) {
        c.bus.write32(GAME_GP - 0x1f04, BE_LIBRARY);
        let world = DATA + 0x2_0000;
        c.bus.write32(GAME_GP - 0x848, world);
        let game = world + 0x1000;
        c.bus.write32(world + 0x84, game);
        let race = world + 0x2000;
        c.bus.write32(game + 0xc, race);
        c.bus.write32(race + 0x7c, 0);
    }

    /// The allocator's mutex looks held by this thread, so `EA_MemAllocNamed`
    /// takes its fast path (the `dispatch.rs` tests' recipe).
    fn alloc_mutex(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(MEM_MUTEX + 4, 0x42);
        c.bus.write32(LOG_RESULTS, 0x42);
        c.bus.write32(MEM_MUTEX + 8, r.next_u32() % 4);
    }

    /// A vtable for `*(obj + 8)` whose two slots call the fake (delta 0, so
    /// the callee is `obj` itself).
    fn plain_vtable(c: &mut Cpu, r: &mut Rng) -> u32 {
        vtable(c, r, &[0x40, 0x48])
    }

    #[test]
    fn fe_module_request_preview_char_matches() {
        check(
            ssx3_ee::s03::f_001a0508__generated,
            fe_module_request_preview_char,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0xa1, 1);
                globals(c);
                let idx = (seed % 3) as u32;
                // The callee compares the character it is asked for with
                // `*(preview)`; `*(preview + 0x10)` and `*(+0xcd4)` stay zero, so
                // both variants take one of the two call-free paths.
                let want = (seed % 2) as u32;
                let stored = if seed % 4 < 2 { want } else { want ^ 1 };
                c.bus.write32(DATA + idx * 0xce0, stored);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = want as u64;
                c.g[A3][0] = 0;
            },
        );
    }

    #[test]
    fn c_fe_state_challenge_popup_transition_in_matches() {
        covered(
            ssx3_ee::s03::f_001adfb8__generated,
            c_fe_state_challenge_popup_transition_in,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0xa2, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                // Half the seeds get a real vtable (non-zero delta and method);
                // the rest point at zeros (a call through the miss hook).
                if seed % 2 == 0 {
                    let vt = vtable(c, &mut r, &[0x100]);
                    c.bus.write32(DATA + 8, vt);
                }
            },
        );
    }

    #[test]
    fn c_fe_state_net_messenger_transition_in_matches() {
        covered(
            ssx3_ee::s04::f_001e0fc0__generated,
            c_fe_state_net_messenger_transition_in,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0xa3, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                if seed % 2 == 0 {
                    let vt = vtable(c, &mut r, &[0x100]);
                    c.bus.write32(DATA + 8, vt);
                }
            },
        );
    }

    #[test]
    fn c_fe_state_net_config_shutdown_matches() {
        check(
            ssx3_ee::s03::f_001b4168__generated,
            c_fe_state_net_config_shutdown,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0xa4, 3);
                globals(c);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    /// The five `OnWidgetAction`/`v24` wrappers share one shape.
    fn widget_action_test(generated: GuestFn, hand: GuestFn, salt: u64, obj_off: u32) {
        check(generated, hand, 40, &|c, seed| {
            let _r = machine(c, seed, salt, 1);
            globals(c);
            c.g[A0][0] = sx(DATA);
            // `*(this + obj_off)` is the widget object; its vtable (at +8) is
            // zero for odd seeds and a fake vtable for even ones.
            c.bus.write32(
                DATA + obj_off,
                if seed % 2 == 0 { DATA + 0x3000 } else { 0 },
            );
        });
    }

    #[test]
    fn net_config_auth_popup_v24_matches() {
        widget_action_test(
            ssx3_ee::s03::f_001b6740__generated,
            net_config_auth_popup_v24,
            0xa5,
            0x20,
        );
    }

    #[test]
    fn c_fe_state_auth_dnas_error_v24_matches() {
        widget_action_test(
            ssx3_ee::s03::f_001b6e10__generated,
            c_fe_state_auth_dnas_error_v24,
            0xa6,
            0x20,
        );
    }

    #[test]
    fn c_fe_state_net_account_mem_v24_matches() {
        widget_action_test(
            ssx3_ee::s03::f_001b8860__generated,
            c_fe_state_net_account_mem_v24,
            0xa7,
            0x20,
        );
    }

    #[test]
    fn c_fe_state_net_create_room_popup_v31_matches() {
        widget_action_test(
            ssx3_ee::s03::f_001bed28__generated,
            c_fe_state_net_create_room_popup_v31,
            0xa8,
            0x20,
        );
    }

    #[test]
    fn c_fe_popup_confirm_v24_matches() {
        widget_action_test(ssx3_ee::s03::f_001ca4f8__generated, c_fe_popup_confirm_v24, 0xa9, 0x20);
    }

    #[test]
    fn c_fe_state_lobby_menu_refresh_player_list_matches() {
        covered(
            ssx3_ee::s03::f_001be510__generated,
            c_fe_state_lobby_menu_refresh_player_list,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0xaa, 30);
                globals(c);
                // `FillPlayerList`'s "-1 = reset" path: 10 list entries whose
                // objects carry the fake vtable at +8 (slots 0x40/0x48).
                c.g[A1][0] = u64::MAX;
                c.g[A0][0] = sx(DATA);
                let vt = plain_vtable(c, &mut r);
                for k in 0..10u32 {
                    let obj = DATA + 0x8000 + 0x100 * k;
                    c.bus.write32(DATA + 0x6d8 + 4 * k, obj);
                    c.bus.write32(obj + 8, vt);
                }
                // `UpdatePlayerScrollArrows`: two list objects behind the two
                // pointer slots, and the rider-count fields that pick the branch.
                for obj in [DATA + 0x9000, DATA + 0xa000] {
                    c.bus.write32(obj + 8, vt);
                }
                c.bus.write32(DATA + 0x728, DATA + 0x9000);
                c.bus.write32(DATA + 0x72c, DATA + 0xa000);
                match seed % 4 {
                    0 => {}
                    1 => c.bus.write32(DATA + 0x77c, 0x100),
                    2 => c.bus.write32(DATA + 0x778, 1),
                    _ => {
                        c.bus.write32(DATA + 0x778, 1);
                        c.bus.write32(DATA + 0x77c, 0x100);
                    }
                }
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_v25_matches() {
        check(
            ssx3_ee::s04::f_001d9d18__generated,
            c_fe_popup_confirm_v25,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0xab, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed as u64) << 4 | 3;
                let live = (seed % 2) as u32;
                c.bus.write32(DATA + 0x40, live);
                // `cFEPopupConfirm_ResetLayout` (+0x2bc) is not up: quick return.
                c.bus.write32(DATA + 0x2bc, 0);
            },
        );
    }

    #[test]
    fn c_fe_state_net_help_popup_on_widget_action_matches() {
        check(
            ssx3_ee::s04::f_001da478__generated,
            c_fe_state_net_help_popup_on_widget_action,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0xac, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                c.g[A2][0] = if seed % 3 == 0 { 6 } else { 7 };
                c.bus.write32(DATA + 0x10, DATA + 0x3000);
            },
        );
    }

    #[test]
    fn ov_screen_map_on_activate_matches() {
        check(
            ssx3_ee::s05::f_00208558__generated,
            ov_screen_map_on_activate,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0xad, 3);
                globals(c);
                c.g[A0][0] = sx(DATA);
                // EventSelect mode 0 (the map tab).
                c.bus.write32(GAME_GP - 0xb54, 0);
            },
        );
    }

    #[test]
    fn ov_screen_map_update_matches() {
        check(
            ssx3_ee::s05::f_00208678__generated,
            ov_screen_map_update,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0xae, 3);
                globals(c);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(GAME_GP - 0xb54, 0);
            },
        );
    }

    #[test]
    fn c_luno_table_ctor_matches() {
        check(
            ssx3_ee::s05::f_00224da0__generated,
            c_luno_table_ctor,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0xaf, 6);
                globals(c);
                alloc_mutex(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 8) as u64;
            },
        );
    }

    #[test]
    fn crowd_man2d_clear_matches() {
        check(
            ssx3_ee::s05::f_00229498__generated,
            crowd_man2d_clear,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0xb0, 0);
                globals(c);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn section_man_ctor_matches() {
        check(
            ssx3_ee::s05::f_0022cce8__generated,
            section_man_ctor,
            20,
            &|c, seed| {
                let _r = machine(c, seed, 0xb1, 0);
                globals(c);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x3_0000 + (seed % 4) * 0x1000);
            },
        );
    }

    #[test]
    fn game_state_goto_next_matches() {
        check(
            ssx3_ee::s06::f_00233aa0__generated,
            game_state_goto_next,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0xb2, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                let target = if seed % 2 == 0 {
                    0
                } else {
                    r.next_u32() % 0x40
                };
                c.bus.write32(DATA, target);
                c.bus.write32(DATA + 4, r.next_u32() % 0x100);
            },
        );
    }

    #[test]
    fn game_state_warp_to_event_update_matches() {
        check(
            ssx3_ee::s06::f_00236198__generated,
            game_state_warp_to_event_update,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0xb3, 1);
                globals(c);
            },
        );
    }

    /// The eight memory-card manager hooks share the same fixture.
    fn mc_manager_test(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 40, &|c, seed| {
            let _r = machine(c, seed, salt, 1);
            globals(c);
            c.g[A0][0] = sx(DATA);
            // The card object at +0x434 and the state machine at +0x748 are
            // null for odd seeds (calls through the miss hook), zeroed objects
            // at DATA + 0x3000 for even ones.
            c.bus
                .write32(DATA + 0x434, if seed % 2 == 0 { DATA + 0x3000 } else { 0 });
            c.bus
                .write32(DATA + 0x748, if seed % 2 == 0 { DATA + 0x4000 } else { 0 });
        });
    }

    #[test]
    fn mc_manager_card_set_state_1e_matches() {
        mc_manager_test(ssx3_ee::s06::f_0023ca70__generated, mc_manager_card_set_state_1e, 0xb4);
    }

    #[test]
    fn mc_manager_destroy_card_matches() {
        mc_manager_test(ssx3_ee::s06::f_0023d5e8__generated, mc_manager_destroy_card, 0xb5);
    }

    #[test]
    fn mc_manager_set_state_no_card_matches() {
        mc_manager_test(ssx3_ee::s06::f_0023eb50__generated, mc_manager_set_state_no_card, 0xb6);
    }

    #[test]
    fn mc_manager_state_13_matches() {
        mc_manager_test(ssx3_ee::s06::f_0023f2d8__generated, mc_manager_state_13, 0xb7);
    }

    #[test]
    fn mc_manager_state_23_matches() {
        mc_manager_test(ssx3_ee::s06::f_0023ffc0__generated, mc_manager_state_23, 0xb8);
    }

    #[test]
    fn mc_manager_state_2a_matches() {
        mc_manager_test(ssx3_ee::s06::f_00240c90__generated, mc_manager_state_2a, 0xb9);
    }

    #[test]
    fn mc_manager_state_2d_matches() {
        mc_manager_test(ssx3_ee::s06::f_00240eb0__generated, mc_manager_state_2d, 0xba);
    }

    #[test]
    fn mc_manager_state_2e_matches() {
        mc_manager_test(ssx3_ee::s06::f_00240ee0__generated, mc_manager_state_2e, 0xbb);
    }

    #[test]
    fn mc_manager_close_card_matches() {
        mc_manager_test(ssx3_ee::s06::f_002410a0__generated, mc_manager_close_card, 0xbc);
    }

    /// `VfxProcess_Update`/`Render`: process object at +0x10.
    fn vfx_test(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 40, &|c, seed| {
            let _r = machine(c, seed, salt, 1);
            globals(c);
            c.g[A0][0] = sx(DATA);
            c.bus
                .write32(DATA + 0x10, if seed % 2 == 0 { DATA + 0x3000 } else { 0 });
        });
    }

    #[test]
    fn vfx_process_update_matches() {
        vfx_test(ssx3_ee::s06::f_00244478__generated, vfx_process_update, 0xbd);
    }

    #[test]
    fn vfx_process_render_matches() {
        vfx_test(ssx3_ee::s06::f_002444a8__generated, vfx_process_render, 0xbe);
    }

    #[test]
    fn vid_engine_player_play_matches() {
        check(
            ssx3_ee::s06::f_00253860__generated,
            vid_engine_player_play,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0xbf, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                c.bus
                    .write32(DATA + 0x38, if seed % 2 == 0 { 0 } else { DATA + 0x3000 });
            },
        );
    }

    #[test]
    fn lobby_man_get_room_name_matches() {
        check(
            ssx3_ee::s06::f_0025f150__generated,
            lobby_man_get_room_name,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0xc0, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                // Lobby not up (early return) / up with a null session object (the
                // name lookup returns null and is offset).
                c.bus.write32(DATA, if seed % 2 == 0 { 0 } else { 1 });
            },
        );
    }

    #[test]
    fn c_setup_man_alloc_packet_matches() {
        check(
            ssx3_ee::s07::f_00264960__generated,
            c_setup_man_alloc_packet,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0xc1, 6);
                globals(c);
                alloc_mutex(c, &mut r);
            },
        );
    }

    /// The three shared-login card-slot getters.
    fn card_slot_test(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 40, &|c, seed| {
            let _r = machine(c, seed, salt, 1);
            globals(c);
            c.g[A0][0] = sx(DATA);
            // The login manager at gp + 0x2e8, its vtable at +0.
            c.bus.write32(
                GAME_GP + 0x2e8,
                if seed % 2 == 0 { DATA + 0x3000 } else { 0 },
            );
        });
    }

    #[test]
    fn c_shared_login_man_card_slot20_matches() {
        card_slot_test(
            ssx3_ee::s07::f_00265a38__generated,
            c_shared_login_man_card_slot20,
            0xc2,
        );
    }

    #[test]
    fn c_shared_login_man_card_slot47_matches() {
        card_slot_test(
            ssx3_ee::s07::f_00265d08__generated,
            c_shared_login_man_card_slot47,
            0xc3,
        );
    }

    #[test]
    fn c_shared_login_man_card_slot63_matches() {
        card_slot_test(
            ssx3_ee::s07::f_00265d38__generated,
            c_shared_login_man_card_slot63,
            0xc4,
        );
    }

    #[test]
    fn c_voip_man_get_status_matches() {
        check(
            ssx3_ee::s07::f_00266f90__generated,
            c_voip_man_get_status,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0xc5, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[A2][0] = sx(DATA + 0x200);
                // Voip not up: the RPC is skipped (its path needs the network
                // stack); the argument registers still have to be forwarded.
                c.bus.write32(DATA, 0);
            },
        );
    }

    #[test]
    fn path_data_set_buffer_matches() {
        check(
            ssx3_ee::s07::f_0026ad70__generated,
            path_data_set_buffer,
            20,
            &|c, seed| {
                let mut r = machine(c, seed, 0xc6, 6);
                globals(c);
                alloc_mutex(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x4000);
                // A zeroed blob parses as empty sections (no allocation loops).
                c.bus.write32(DATA + 0x48, DATA + 0x4000);
                c.bus.write32(DATA + 0x4000, 0);
            },
        );
    }

    #[test]
    fn playback_man_restore_world_state_matches() {
        check(
            ssx3_ee::s07::f_0026cc18__generated,
            playback_man_restore_world_state,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0xc7, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                // No snapshot pending: the world-state reader is skipped.
                c.bus.write32(DATA + 0x10, 0);
            },
        );
    }

    #[test]
    fn snapshot_list_pop_front_matches() {
        check(
            ssx3_ee::s07::f_0026e670__generated,
            snapshot_list_pop_front,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0xc8, 0);
                globals(c);
                c.g[A0][0] = sx(DATA);
                // Empty list (null head) or one node with no neighbours.
                let head = if seed % 2 == 0 { 0 } else { DATA + 0x100 };
                c.bus.write32(DATA, head);
                c.bus.write32(DATA + 0x100 + 0x14, 0);
                c.bus.write32(DATA + 0x100 + 0x18, 0);
            },
        );
    }

    #[test]
    fn script_engine_stop_lists_matches() {
        check(
            ssx3_ee::s07::f_002789e0__generated,
            script_engine_stop_lists,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0xc9, 2);
                globals(c);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    /// The three script-list helpers: engine + rider index (+ payload).
    fn script_list_test(generated: GuestFn, hand: GuestFn, salt: u64, payload: bool) {
        check(generated, hand, 40, &|c, seed| {
            let mut r = machine(c, seed, salt, 1);
            globals(c);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 3) as u64;
            if payload {
                c.g[A2][0] = (seed % 2) as u64;
            }
            // The rider script-list array.
            c.bus.write32(DATA + 0x54c, DATA + 0x4000);
            let _ = r.next_u32();
        });
    }

    #[test]
    fn script_engine_list_prepare_matches() {
        script_list_test(
            ssx3_ee::s07::f_00278f38__generated,
            script_engine_list_prepare,
            0xca,
            false,
        );
    }

    #[test]
    fn script_engine_list_set_paused_matches() {
        script_list_test(
            ssx3_ee::s07::f_00279040__generated,
            script_engine_list_set_paused,
            0xcb,
            true,
        );
    }

    #[test]
    fn script_engine_list_get_entry_type_matches() {
        script_list_test(
            ssx3_ee::s07::f_00279148__generated,
            script_engine_list_get_entry_type,
            0xcc,
            true,
        );
    }

    /// The four script-binding hooks (ScriptObjCam / cTransitionMan).
    fn script_binding_test(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 40, &|c, seed| {
            let _r = machine(c, seed, salt, 1);
            globals(c);
            c.g[A0][0] = sx(DATA);
            c.bus
                .write32(DATA + 0x10, if seed % 2 == 0 { DATA + 0x3000 } else { 0 });
        });
    }

    #[test]
    fn script_obj_cam_v02_matches() {
        script_binding_test(ssx3_ee::s07::f_0027d7f0__generated, script_obj_cam_v02, 0xcd);
    }

    #[test]
    fn script_obj_cam_v03_matches() {
        script_binding_test(ssx3_ee::s07::f_0027d820__generated, script_obj_cam_v03, 0xce);
    }

    #[test]
    fn c_transition_man_script_pause_matches() {
        script_binding_test(
            ssx3_ee::s07::f_00282f00__generated,
            c_transition_man_script_pause,
            0xcf,
        );
    }

    #[test]
    fn c_transition_man_script_resume_matches() {
        script_binding_test(
            ssx3_ee::s07::f_00282f30__generated,
            c_transition_man_script_resume,
            0xd0,
        );
    }
}
