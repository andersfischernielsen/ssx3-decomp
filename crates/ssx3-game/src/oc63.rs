//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for the front-end / menu / memcard / net hooks and the family
//! of tiny deleting destructors, each with a differential test against the
//! generated body.
//!
//! - **Net/memcard**: `NetFEState_OnChildEventIgnoreClose` (a base state's
//!   child-event filter) and `cFEStateNetAccountMem_StoreAccountStrings`.
//! - **Message center**: `MessageCenter_GetMessageId` / `_GetMessageVariant`
//!   (a field of the nth inbox entry).
//! - **Menus**: the `OnActivate` / `OnShutdown` / `OnChildEvent` hooks of the
//!   event-select, buy-attribute, career-stats, reward-list, map and
//!   peak-challenge screens, plus the two load-screen updates.
//! - **`SectionMan_GetEventWorldSection`**: a table lookup, no calls.
//! - **Deleting destructors**: republish the class's vtable and
//!   `operator delete(this)` when flag bit 0 is set (37 of them, one shared
//!   body).
//!
//! Calls go through [`crate::call`] with the same registers, cycles and
//! return-address checks as the generated code.

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001a_9638,
        net_fe_state_on_child_event_ignore_close,
        "NetFEState_OnChildEventIgnoreClose",
    ),
    (
        0x001b_7778,
        cfe_state_net_account_mem_store_account_strings,
        "cFEStateNetAccountMem_StoreAccountStrings",
    ),
    (0x001c_5750, c_fe_popup_confirm_cfg_dtor, "cFEPopupConfirm_CfgDtor"),
    (0x001d_f440, rule_select_member_dtor, "RuleSelectMember_dtor"),
    (0x001e_0740, fe_ui_sound_v01, "FEUISound_v01"),
    (0x001e_0780, ov_ui_sound_v01, "OVUISound_v01"),
    (0x001e_1048, ui_sound_base_dtor, "UISoundBase_dtor"),
    (0x001e_10b8, ui_custom_base_dtor, "UICustomBase_dtor"),
    (0x001e_1108, ui_input_base_dtor, "UIInputBase_dtor"),
    (0x001e_3620, message_center_get_message_id, "MessageCenter_GetMessageId"),
    (
        0x001e_3650,
        message_center_get_message_variant,
        "MessageCenter_GetMessageVariant",
    ),
    (
        0x001e_8ed0,
        ov_screen_peak_challenge_results_on_activate,
        "OVScreenPeakChallengeResults_OnActivate",
    ),
    (0x001f_4a60, c_fe_state_buy_attrib_on_child_event, "cFEStateBuyAttrib_OnChildEvent"),
    (
        0x001f_55e8,
        c_fe_state_career_stats_on_activate,
        "cFEStateCareerStats_OnActivate",
    ),
    (
        0x001f_f6d0,
        ov_screen_reward_list_on_activate,
        "OVScreenRewardList_OnActivate",
    ),
    (0x0020_0ac0, event_select_on_activate, "EventSelect_OnActivate"),
    (0x0020_8588, ov_screen_map_shutdown, "OVScreenMap_Shutdown"),
    (
        0x0022_e098,
        section_man_get_event_world_section,
        "SectionMan_GetEventWorldSection",
    ),
    (0x0023_2538, init_load_screen_dtor, "InitLoadScreen_dtor"),
    (0x0023_2750, c_pre_game_load_screen_dtor, "cPreGameLoadScreen_dtor"),
    (0x0023_2780, c_pre_fe_load_screen_dtor, "cPreFELoadScreen_dtor"),
    (0x0023_3408, c_game_load_screen_update, "cGameLoadScreen_Update"),
    (0x0023_3a70, c_fe_load_screen_update, "cFELoadScreen_Update"),
    (0x0024_3a40, app_module_dtor, "AppModule_dtor"),
    (0x0024_3a80, pre_start_screen_dtor, "PreStartScreen_dtor"),
    (0x0024_3ae8, app_module_vt47d818_dtor, "AppModuleVt47d818_dtor"),
    (0x0024_3b50, start_screen_dtor, "StartScreen_dtor"),
    (0x0024_3b80, c_game_load_screen_dtor, "cGameLoadScreen_dtor"),
    (0x0024_3c00, c_fe_load_screen_dtor, "cFELoadScreen_dtor"),
    (0x0024_3c80, game_state_dtor, "GameState_dtor"),
    (0x0024_3cf0, game_state_quit_v01, "GameStateQuit_v01"),
    (0x0024_3d58, game_state_idle_v01, "GameStateIdle_v01"),
    (0x0024_3d88, game_state_finish_v01, "GameStateFinish_v01"),
    (0x0024_3db8, game_state_reset_ai_v01, "GameStateResetAI_v01"),
    (0x0024_3e20, game_state_pre_race_load_v01, "GameStatePreRaceLoad_v01"),
    (0x0024_3e50, game_state_countdown_v01, "GameStateCountdown_v01"),
    (0x0024_3eb8, game_state_race_v01, "GameStateRace_v01"),
    (0x0024_3f20, game_state_reset_v01, "GameStateReset_v01"),
    (0x0024_3f50, game_state_restart_race_v01, "GameStateRestartRace_v01"),
    (0x0024_3fc0, game_state_warp_to_event_v01, "GameStateWarpToEvent_v01"),
    (0x0024_4028, game_state_event_select_v01, "GameStateEventSelect_v01"),
    (
        0x0024_4060,
        game_state_event_switch_load_v01,
        "GameStateEventSwitchLoad_v01",
    ),
    (0x0024_4090, game_state_pre_race_menu_v01, "GameStatePreRaceMenu_v01"),
    (0x0024_40f8, game_state_results_v01, "GameStateResults_v01"),
    (
        0x0024_4160,
        game_state_finish_cinematic_v01,
        "GameStateFinishCinematic_v01",
    ),
    (
        0x0024_41c8,
        game_state_replay_playback_v01,
        "GameStateReplayPlayback_v01",
    ),
    (0x0024_43c0, generic_stream_pause_v01, "GenericStreamPause_v01"),
    (0x0024_44f0, gm_handler_dtor, "GMHandler_dtor"),
    (0x0024_4560, gm_handler0_v01, "GMHandler0_v01"),
    (0x0024_4590, gm_handler1_v01, "GMHandler1_v01"),
];

// ---------------------------------------------------------------------------
// Callees (cross-shard, through the function table as the generated code
// calls them).

mod callee {
    /// `NetFEState_OnChildEventDefault` (same shard as its caller).
    pub const NET_FE_ON_CHILD_EVENT_DEFAULT: u32 = 0x001a_93d0;
    /// `cSharedLoginMan_SetAccountName` / `..._Password`.
    pub const SHARED_LOGIN_SET_ACCOUNT_NAME: u32 = 0x0026_5f18;
    pub const SHARED_LOGIN_SET_ACCOUNT_PASSWORD: u32 = 0x0026_5f68;
    /// `MessageCenter_GetInbox`.
    pub const MESSAGE_CENTER_GET_INBOX: u32 = 0x001e_39f8;
    /// `UIState_OnActivate`.
    pub const UI_STATE_ON_ACTIVATE: u32 = 0x0039_e4c0;
    /// `SSXAudioSystem_GetInstance` / `_PlayFESound` / `_OnEventSelectEnter`.
    pub const AUDIO_GET_INSTANCE: u32 = 0x0028_b180;
    pub const AUDIO_PLAY_FE_SOUND: u32 = 0x0029_4f78;
    pub const AUDIO_ON_EVENT_SELECT_ENTER: u32 = 0x0028_f5b8;
    /// `cFEStateBuyAttrib_CommitPurchase`.
    pub const BUY_ATTRIB_COMMIT_PURCHASE: u32 = 0x001f_5300;
    /// `cFEState_OnActivate` / `cFEStateCareerStats_SelectPage`.
    pub const FE_STATE_ON_ACTIVATE: u32 = 0x0018_6518;
    pub const CAREER_STATS_SELECT_PAGE: u32 = 0x001f_5ba8;
    /// `OVScreenRewardList_ShowPage`.
    pub const REWARD_LIST_SHOW_PAGE: u32 = 0x001f_fd08;
    /// `EventSelect_SetMode` / `EventSelect_Shutdown`.
    pub const EVENT_SELECT_SET_MODE: u32 = 0x0020_30a0;
    pub const EVENT_SELECT_SHUTDOWN: u32 = 0x0020_0af0;
    /// `cOVStateOverlay_Shutdown`.
    pub const OV_STATE_OVERLAY_SHUTDOWN: u32 = 0x0020_a430;
    /// `UIEngine_Update` / `AppModule_Update`.
    pub const UI_ENGINE_UPDATE: u32 = 0x0039_8038;
    pub const APP_MODULE_UPDATE: u32 = 0x0023_1cb0;
    /// `operator delete`.
    pub const DELETE: u32 = 0x0031_7e50;
}

/// `a + off` (32-bit address arithmetic).
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

// ---------------------------------------------------------------------------
// Net / memcard hooks.

/// `void NetFEState_OnChildEventIgnoreClose(self, arg, msg, arg3)` (0x1a9638):
/// the child-event handler of the states that ignore a "close" message
/// (0xf) and a "destroy" message (0x14); everything else goes to the base
/// handler `NetFEState_OnChildEventDefault`.
pub fn net_fe_state_on_child_event_ignore_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xf;
        let msg = c.g[A2][0];
        frame.save_ra(c, 0);
        if msg == 0xf {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x14;
            if msg == 0x14 {
                frame.restore_ra(c, 0);
            } else {
                c.cycles += 2;
                call_ptr(c, callee::NET_FE_ON_CHILD_EVENT_DEFAULT, 0x001a_965c)?;
                c.cycles += 1;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateNetAccountMem_StoreAccountStrings(self)` (0x1b7778): copy the
/// account name (`self+0x6e0`) and password (`self+0x6f1`) into the shared
/// login manager.
pub fn cfe_state_net_account_mem_store_account_strings(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = r32(c, S0);
        c.g[A0][0] = sx(p(this, 0x6e0));
        call_ptr(c, callee::SHARED_LOGIN_SET_ACCOUNT_NAME, 0x001b_7790)?;
        c.g[A0][0] = sx(p(this, 0x6f1));
        call_ptr(c, callee::SHARED_LOGIN_SET_ACCOUNT_PASSWORD, 0x001b_7798)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Message center.

/// `int MessageCenter_GetMessageId(MessageCenter* mc, int index)` (0x1e3620):
/// the id field (`+0`) of the `index`th entry of the message center's inbox
/// (entries are 8 bytes wide).
pub fn message_center_get_message_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::MESSAGE_CENTER_GET_INBOX, 0x001e_3634)?;
        let index = r32(c, S0) << 3;
        c.g[S0][0] = sx(index);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S0)));
        c.g[V0][0] = c.lw(r32(c, V0));
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int MessageCenter_GetMessageVariant(MessageCenter* mc, int index)`
/// (0x1e3650): the variant byte (`+4`) of the `index`th inbox entry.
pub fn message_center_get_message_variant(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::MESSAGE_CENTER_GET_INBOX, 0x001e_3664)?;
        let index = r32(c, S0) << 3;
        c.g[S0][0] = sx(index);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S0)));
        c.g[V0][0] = c.lbu(p(r32(c, V0), 4));
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Screen activation / shutdown hooks.

/// `void OVScreenPeakChallengeResults_OnActivate(self)` (0x1e8ed0): activate
/// the base screen state, then play the peak-challenge screen's front-end
/// sound (id 0xe) on the audio system.
pub fn ov_screen_peak_challenge_results_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, callee::UI_STATE_ON_ACTIVATE, 0x001e_8ee0)?;
        call_ptr(c, callee::AUDIO_GET_INSTANCE, 0x001e_8ee8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xe;
        call_ptr(c, callee::AUDIO_PLAY_FE_SOUND, 0x001e_8ef4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateBuyAttrib_OnChildEvent(self, sender, msg, arg)` (0x1f4a60):
/// on the "confirm purchase" message (0x16), commit the purchase if the
/// sender has one pending (`+0x6c`).
pub fn c_fe_state_buy_attrib_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x16;
        let msg = c.g[A2][0];
        frame.save_ra(c, 0);
        if msg == 0x16 {
            c.cycles += 3;
            c.g[V0][0] = c.lw(p(abi::arg(c, 1), 0x6c));
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                call_ptr(c, callee::BUY_ATTRIB_COMMIT_PURCHASE, 0x001f_4a84)?;
                c.cycles += 1;
            }
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCareerStats_OnActivate(self, arg)` (0x1f55e8): activate the
/// base front-end state, then select the page held in `self+0x48`.
pub fn c_fe_state_career_stats_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::FE_STATE_ON_ACTIVATE, 0x001f_55fc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.lw(p(r32(c, A0), 0x48));
        call_ptr(c, callee::CAREER_STATS_SELECT_PAGE, 0x001f_5608)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenRewardList_OnActivate(self)` (0x1ff6d0): activate the base
/// screen state, then show the reward list's first page.
pub fn ov_screen_reward_list_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::UI_STATE_ON_ACTIVATE, 0x001ff6e4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        call_ptr(c, callee::REWARD_LIST_SHOW_PAGE, 0x001ff6f0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EventSelect_OnActivate(self)` (0x200ac0): put the map widget into
/// its current mode, then let the audio system react to entering event
/// select.
pub fn event_select_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.lw(abi::r32(c, GP).wrapping_sub(0xb54));
        frame.save_ra(c, 0);
        call_ptr(c, callee::EVENT_SELECT_SET_MODE, 0x0020_0ad4)?;
        call_ptr(c, callee::AUDIO_GET_INSTANCE, 0x0020_0adc)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, callee::AUDIO_ON_EVENT_SELECT_ENTER, 0x0020_0ae4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenMap_Shutdown(self)` (0x208588): shut the map's event-select
/// widget (`self+0x9c`) down, then the overlay state.
pub fn ov_screen_map_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(p(r32(c, S0), 0x9c));
        call_ptr(c, callee::EVENT_SELECT_SHUTDOWN, 0x0020_85a0)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::OV_STATE_OVERLAY_SHUTDOWN, 0x0020_85a8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Section manager.

/// `int SectionMan_GetEventWorldSection(SectionMan* man, int event)`
/// (0x22e098): the world section id of an event, through the two global
/// tables at 0x442498 (event records, 0x28 bytes each: a section record
/// index at `+0x10`) and 0x442168 (section records, 16 bytes each, id at
/// `+4`). The `mult` the compiler left in clobbers `a1` with its own product
/// (`mult a1, v0` with `rd = a1`), so the table index is `event * 0x28`.
pub fn section_man_get_event_world_section(c: &mut Cpu) {
    c.cycles += 12;
    c.g[V0][0] = 0x28;
    c.g[V1][0] = 0x440000;
    c.mult(0, A1, A1, V0);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x2488));
    c.g[A0][0] = 0x440000;
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x2168));
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
    c.g[V0][0] = c.lw(p(r32(c, V1), 0x10));
    c.g[V0][0] = sx(r32(c, V0) << 4);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
    c.g[V0][0] = c.lw(p(r32(c, A0), 4));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Load screens.

/// `void cGameLoadScreen_Update(self)` (0x233408): drive the game load
/// screen's UI engine (`self+0xc`) and then the app module's fade.
pub fn c_game_load_screen_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.lw(p(r32(c, S0), 0xc));
        call_ptr(c, callee::UI_ENGINE_UPDATE, 0x0023_3420)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::APP_MODULE_UPDATE, 0x0023_3428)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFELoadScreen_Update(self)` (0x233a70): the front-end load screen's
/// update (same shape as the game one).
pub fn c_fe_load_screen_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.lw(p(r32(c, S0), 0xc));
        call_ptr(c, callee::UI_ENGINE_UPDATE, 0x0023_3a88)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::APP_MODULE_UPDATE, 0x0023_3a90)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Deleting destructors.

/// The shared body of the tiny deleting destructors: republish the class's
/// vtable pointer at `off` in `this`, then `operator delete(this)` when bit 0
/// of the destructor flags (`a1`) is set. `ret` is the address the delete
/// returns to (the instruction after its call).
fn vtable_dtor(c: &mut Cpu, vtable: u32, off: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[V0][0] = vtable as u64;
        let frame = Frame::open(c, 0x10);
        let this = r32(c, A0);
        frame.save_ra(c, 0);
        c.g[A1][0] &= 1;
        let deleting = c.g[A1][0] != 0;
        let v = c.g[V0][0];
        c.sw(p(this, off), v);
        if deleting {
            c.cycles += 2;
            call_ptr(c, callee::DELETE, ret)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEPopupConfirm` config destructor (vtable at `+0xc8`).
pub fn c_fe_popup_confirm_cfg_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_cca8, 0xc8, 0x001c_5774);
}

/// `RuleSelectMember` destructor (vtable at `+0x30`).
pub fn rule_select_member_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_b078, 0x30, 0x001d_f464);
}

/// `FEUISound` destructor (vtable at `+0`).
pub fn fe_ui_sound_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_dd60, 0, 0x001e_0764);
}

/// `OVUISound` destructor (vtable at `+0`).
pub fn ov_ui_sound_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_dd60, 0, 0x001e_07a4);
}

/// `UISoundBase` destructor (vtable at `+0`).
pub fn ui_sound_base_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_dd60, 0, 0x001e_106c);
}

/// `UICustomBase` destructor (vtable at `+4`).
pub fn ui_custom_base_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_dd28, 4, 0x001e_10dc);
}

/// `UIInputBase` destructor (vtable at `+8`).
pub fn ui_input_base_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_dba8, 8, 0x001e_112c);
}

/// `InitLoadScreen` destructor (vtable at `+0`).
pub fn init_load_screen_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_d970, 0, 0x0023_255c);
}

/// `cPreGameLoadScreen` destructor (vtable at `+0`).
pub fn c_pre_game_load_screen_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_d970, 0, 0x0023_2774);
}

/// `cPreFELoadScreen` destructor (vtable at `+0`).
pub fn c_pre_fe_load_screen_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_d970, 0, 0x0023_27a4);
}

/// `AppModule` destructor (vtable at `+0`).
pub fn app_module_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_d970, 0, 0x0024_3a64);
}

/// `PreStartScreen` destructor (vtable at `+0`).
pub fn pre_start_screen_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_d970, 0, 0x0024_3aa4);
}

/// `AppModule`'s secondary vtable (0x47d818) destructor (vtable at `+0`).
pub fn app_module_vt47d818_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_d970, 0, 0x0024_3b0c);
}

/// `StartScreen` destructor (vtable at `+0`).
pub fn start_screen_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_d970, 0, 0x0024_3b74);
}

/// `cGameLoadScreen` destructor (vtable at `+0`).
pub fn c_game_load_screen_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_d970, 0, 0x0024_3ba4);
}

/// `cFELoadScreen` destructor (vtable at `+0`).
pub fn c_fe_load_screen_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_d970, 0, 0x0024_3c24);
}

/// `GameState` destructor (vtable at `+0xc`).
pub fn game_state_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_3ca4);
}

/// `GameStateQuit` destructor (vtable at `+0xc`).
pub fn game_state_quit_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_3d14);
}

/// `GameStateIdle` destructor (vtable at `+0xc`).
pub fn game_state_idle_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_3d7c);
}

/// `GameStateFinish` destructor (vtable at `+0xc`).
pub fn game_state_finish_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_3dac);
}

/// `GameStateResetAI` destructor (vtable at `+0xc`).
pub fn game_state_reset_ai_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_3ddc);
}

/// `GameStatePreRaceLoad` destructor (vtable at `+0xc`).
pub fn game_state_pre_race_load_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_3e44);
}

/// `GameStateCountdown` destructor (vtable at `+0xc`).
pub fn game_state_countdown_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_3e74);
}

/// `GameStateRace` destructor (vtable at `+0xc`).
pub fn game_state_race_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_3edc);
}

/// `GameStateReset` destructor (vtable at `+0xc`).
pub fn game_state_reset_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_3f44);
}

/// `GameStateRestartRace` destructor (vtable at `+0xc`).
pub fn game_state_restart_race_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_3f74);
}

/// `GameStateWarpToEvent` destructor (vtable at `+0xc`).
pub fn game_state_warp_to_event_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_3fe4);
}

/// `GameStateEventSelect` destructor (vtable at `+0xc`).
pub fn game_state_event_select_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_404c);
}

/// `GameStateEventSwitchLoad` destructor (vtable at `+0xc`).
pub fn game_state_event_switch_load_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_4084);
}

/// `GameStatePreRaceMenu` destructor (vtable at `+0xc`).
pub fn game_state_pre_race_menu_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_40b4);
}

/// `GameStateResults` destructor (vtable at `+0xc`).
pub fn game_state_results_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_411c);
}

/// `GameStateFinishCinematic` destructor (vtable at `+0xc`).
pub fn game_state_finish_cinematic_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_4184);
}

/// `GameStateReplayPlayback` destructor (vtable at `+0xc`).
pub fn game_state_replay_playback_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_41ec);
}

/// `GenericStreamPause` destructor (vtable at `+0`).
pub fn generic_stream_pause_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_db28, 0, 0x0024_43e4);
}

/// `GMHandler` destructor (vtable at `+0`).
pub fn gm_handler_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d008, 0, 0x0024_4514);
}

/// `GMHandler0` destructor (vtable at `+0`).
pub fn gm_handler0_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d008, 0, 0x0024_4584);
}

/// `GMHandler1` destructor (vtable at `+0`).
pub fn gm_handler1_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d008, 0, 0x0024_45b4);
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, ptr, vtable, LOG_RESULTS};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use ssx3_ee as ee;

    /// The allocator mutex (`EA_Mutex` at 0x538b00); "held by this thread"
    /// (id 0x42, which the first fake syscall also returns) so the real
    /// `__builtin_delete` takes its fast path.
    const MEM_MUTEX: u32 = 0x0053_8b00;

    fn alloc_setup(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(MEM_MUTEX + 4, 0x42);
        c.bus.write32(LOG_RESULTS, 0x42);
        c.bus.write32(MEM_MUTEX + 8, r.next_u32() % 4);
    }

    /// The behaviour library singleton (a zeroed dummy, so
    /// `BELibrary_GetInstance` neither constructs one nor reads the disc) and
    /// a world whose race manager holds one rider (rider loops in the sound
    /// and UI code would otherwise walk a garbage count and run away).
    fn globals(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x6_c000);
        let (world, ail, arr) = (DATA + 0x6_8000, DATA + 0x6_8100, DATA + 0x6_8200);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
        c.bus.write32(world + 0x84, ail);
        c.bus.write32(ail + 0xc, arr);
        c.bus.write32(arr + 8, 20);
        c.bus.write32(arr + 0x28, DATA + 0x6_8400);
        c.bus.write32(arr + 0x7c, 1);
    }

    /// Zero `words` words from the object page, so no callee reads a stale
    /// value.
    fn clear(c: &mut Cpu, base: u32, words: u32) {
        for i in 0..words {
            c.bus.write32(base + 4 * i, 0);
        }
    }

    /// The behaviour library interface slot: `BELibrary_GetInterface(1)`
    /// reads `D_00530 8A8[index]`, whose `+4` is a vtable with a fake method.
    fn be_interface(c: &mut Cpu, r: &mut Rng) {
        globals(c);
        c.bus.write32(GAME_GP.wrapping_sub(0x1efc), 0);
        let iface = DATA + 0x400;
        c.bus.write32(0x0053_08a8, iface);
        let vt = vtable(c, r, &[0x8]);
        c.bus.write32(iface + 4, vt);
    }

    // ---------------------------------------------------------------- net/fe

    #[test]
    fn net_fe_state_on_child_event_ignore_close_matches() {
        check(ee::s03::f_001a9638__generated, net_fe_state_on_child_event_ignore_close, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x1a9638, 1);
            globals(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = r.next_u32() as u64;
            // 0xf (ignored), 0x14 (ignored), and an id the base handler
            // forwards without reaching its jump table.
            c.g[A2][0] = match seed % 3 {
                0 => 0xf,
                1 => 0x14,
                _ => (3 + r.next_u32() % 4) as u64,
            };
            c.g[A3][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn cfe_state_net_account_mem_store_account_strings_matches() {
        check(
            ee::s03::f_001b7778__generated,
            cfe_state_net_account_mem_store_account_strings,
            60,
            &|c, seed| {
                machine(c, seed, 0x1b7778, 2);
                clear(c, DATA, 0x200);
                c.g[A0][0] = DATA as u64;
                // NUL-terminated name (+0x6e0) and password (+0x6f1).
                for i in 0..8u32 {
                    c.bus.write8(DATA + 0x6e0 + i, b'A' + i as u8);
                }
                c.bus.write8(DATA + 0x6e0 + 8, 0);
                c.bus.write8(DATA + 0x6e8, 0);
                for i in 0..8u32 {
                    c.bus.write8(DATA + 0x6f1 + i, b'k' + i as u8);
                }
                c.bus.write8(DATA + 0x6f1 + 8, 0);
            },
        );
    }

    // -------------------------------------------------------- message center

    #[test]
    fn message_center_get_message_id_matches() {
        check(ee::s04::f_001e3620__generated, message_center_get_message_id, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x1e3620, 1);
            clear(c, DATA, 0x400);
            be_interface(c, &mut r);
            c.g[A0][0] = (seed % 6) as u64;
        });
    }

    #[test]
    fn message_center_get_message_variant_matches() {
        check(ee::s04::f_001e3650__generated, message_center_get_message_variant, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x1e3650, 1);
            clear(c, DATA, 0x400);
            be_interface(c, &mut r);
            c.g[A0][0] = (seed % 6) as u64;
        });
    }

    // ------------------------------------------------------------- screens

    #[test]
    fn ov_screen_peak_challenge_results_on_activate_matches() {
        check(
            ee::s04::f_001e8ed0__generated,
            ov_screen_peak_challenge_results_on_activate,
            60,
            &|c, seed| {
                machine(c, seed, 0x1e8ed0, 2);
                globals(c);
                clear(c, DATA, 0x400);
                c.g[A0][0] = DATA as u64;
                // The audio singleton: null (the dispatcher's jump table is
                // empty on test data) or a dummy.
                c.bus.write32(GAME_GP.wrapping_add(0x410), if seed % 2 == 0 { 0 } else { DATA + 0x800 });
            },
        );
    }

    #[test]
    fn c_fe_state_buy_attrib_on_child_event_matches() {
        check(
            ee::s04::f_001f4a60__generated,
            c_fe_state_buy_attrib_on_child_event,
            60,
            &|c, seed| {
                machine(c, seed, 0x1f4a60, 6);
                globals(c);
                clear(c, DATA, 0x800);
                c.g[A0][0] = DATA as u64;
                let sender = DATA + 0x400;
                c.g[A1][0] = sender as u64;
                // The purchase confirmation message (0x16) with and without a
                // pending purchase, and another message.
                c.g[A2][0] = match seed % 3 {
                    0 => 0x15,
                    1 => 0x16,
                    _ => 0x16,
                };
                c.bus.write32(sender + 0x6c, if seed % 2 == 0 { 0 } else { 1 });
                c.g[A3][0] = 0;
            },
        );
    }

    #[test]
    fn c_fe_state_career_stats_on_activate_matches() {
        check(ee::s04::f_001f55e8__generated, c_fe_state_career_stats_on_activate, 20, &|c, seed| {
            machine(c, seed, 0x1f55e8, 8);
            globals(c);
            clear(c, DATA, 0x400);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed & 1) as u64;
            // The page to select (`self+0x48`); page 0 is the default and
            // keeps `cFEStateCareerStats_SelectPage` on its first path.
            c.bus.write32(DATA + 0x48, 0);
        });
    }

    #[test]
    fn ov_screen_reward_list_on_activate_matches() {
        check(ee::s05::f_001ff6d0__generated, ov_screen_reward_list_on_activate, 20, &|c, seed| {
            machine(c, seed, 0x1ff6d0, 6);
            globals(c);
            clear(c, DATA, 0x400);
            c.g[A0][0] = DATA as u64;
            let _ = seed;
        });
    }

    #[test]
    fn event_select_on_activate_matches() {
        check(ee::s05::f_00200ac0__generated, event_select_on_activate, 40, &|c, seed| {
            machine(c, seed, 0x200ac0, 4);
            globals(c);
            clear(c, DATA, 0x400);
            c.g[A0][0] = DATA as u64;
            // The map mode the widget is put into (`D_004A259C`).
            c.bus.write32(GAME_GP.wrapping_sub(0xb54), (seed % 4) as u32);
        });
    }

    #[test]
    fn ov_screen_map_shutdown_matches() {
        check(ee::s05::f_00208588__generated, ov_screen_map_shutdown, 40, &|c, seed| {
            machine(c, seed, 0x208588, 6);
            globals(c);
            clear(c, DATA, 0x400);
            c.g[A0][0] = DATA as u64;
            let _ = seed;
        });
    }

    #[test]
    fn section_man_get_event_world_section_matches() {
        check(ee::s05::f_0022e098__generated, section_man_get_event_world_section, 60, &|c, seed| {
            machine(c, seed, 0x22e098, 0);
            c.g[A0][0] = DATA as u64;
            // `event` is a byte offset into the event records; the function's
            // `mult` scales it by 0x28 to reach the record's section index.
            let event = (seed % 8) * 0x28;
            c.g[A1][0] = event as u64;
            let section = seed % 16;
            c.bus.write32(0x0044_2488 + 0x10 + event * 0x28, section);
            c.bus.write32(0x0044_216c + section * 16, 0x1000 + seed);
        });
    }

    // ---------------------------------------------------------- load screens

    #[test]
    fn c_game_load_screen_update_matches() {
        check(ee::s06::f_00233408__generated, c_game_load_screen_update, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x233408, 4);
            globals(c);
            clear(c, DATA, 0x1000);
            c.g[A0][0] = DATA as u64;
            // UIEngine -> (+0xc) view -> (+8) vtable -> +0x170 slot.
            let engine = DATA + 0x800;
            let view = DATA + 0x900;
            c.bus.write32(DATA + 0xc, engine);
            c.bus.write32(engine + 0xc, view);
            let vt = vtable(c, &mut r, &[0x170]);
            c.bus.write32(view + 8, vt);
            let _ = seed;
        });
    }

    #[test]
    fn c_fe_load_screen_update_matches() {
        check(ee::s06::f_00233a70__generated, c_fe_load_screen_update, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x233a70, 4);
            globals(c);
            clear(c, DATA, 0x1000);
            c.g[A0][0] = DATA as u64;
            // UIEngine -> (+0xc) view -> (+8) vtable -> +0x170 slot.
            let engine = DATA + 0x800;
            let view = DATA + 0x900;
            c.bus.write32(DATA + 0xc, engine);
            c.bus.write32(engine + 0xc, view);
            let vt = vtable(c, &mut r, &[0x170]);
            c.bus.write32(view + 8, vt);
            let _ = seed;
        });
    }

    // ------------------------------------------------------------ destructors

    /// A deleting-destructor test: `this` is a scratch object, `a1` picks the
    /// deleting / non-deleting branch, and the real `operator delete` runs
    /// (fast path of the allocator mutex).
    fn dtor_case(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 40, &|c, seed| {
            let mut r = machine(c, seed, salt, 1);
            alloc_setup(c, &mut r);
            c.g[A0][0] = ptr(&mut r) as u64;
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    #[test]
    fn c_fe_popup_confirm_cfg_dtor_matches() {
        dtor_case(ee::s03::f_001c5750__generated, c_fe_popup_confirm_cfg_dtor, 0x1c5750);
    }

    #[test]
    fn rule_select_member_dtor_matches() {
        dtor_case(ee::s04::f_001df440__generated, rule_select_member_dtor, 0x1df440);
    }

    #[test]
    fn fe_ui_sound_v01_matches() {
        dtor_case(ee::s04::f_001e0740__generated, fe_ui_sound_v01, 0x1e0740);
    }

    #[test]
    fn ov_ui_sound_v01_matches() {
        dtor_case(ee::s04::f_001e0780__generated, ov_ui_sound_v01, 0x1e0780);
    }

    #[test]
    fn ui_sound_base_dtor_matches() {
        dtor_case(ee::s04::f_001e1048__generated, ui_sound_base_dtor, 0x1e1048);
    }

    #[test]
    fn ui_custom_base_dtor_matches() {
        dtor_case(ee::s04::f_001e10b8__generated, ui_custom_base_dtor, 0x1e10b8);
    }

    #[test]
    fn ui_input_base_dtor_matches() {
        dtor_case(ee::s04::f_001e1108__generated, ui_input_base_dtor, 0x1e1108);
    }

    #[test]
    fn init_load_screen_dtor_matches() {
        dtor_case(ee::s06::f_00232538__generated, init_load_screen_dtor, 0x232538);
    }

    #[test]
    fn c_pre_game_load_screen_dtor_matches() {
        dtor_case(ee::s06::f_00232750__generated, c_pre_game_load_screen_dtor, 0x232750);
    }

    #[test]
    fn c_pre_fe_load_screen_dtor_matches() {
        dtor_case(ee::s06::f_00232780__generated, c_pre_fe_load_screen_dtor, 0x232780);
    }

    #[test]
    fn app_module_dtor_matches() {
        dtor_case(ee::s06::f_00243a40__generated, app_module_dtor, 0x243a40);
    }

    #[test]
    fn pre_start_screen_dtor_matches() {
        dtor_case(ee::s06::f_00243a80__generated, pre_start_screen_dtor, 0x243a80);
    }

    #[test]
    fn app_module_vt47d818_dtor_matches() {
        dtor_case(ee::s06::f_00243ae8__generated, app_module_vt47d818_dtor, 0x243ae8);
    }

    #[test]
    fn start_screen_dtor_matches() {
        dtor_case(ee::s06::f_00243b50__generated, start_screen_dtor, 0x243b50);
    }

    #[test]
    fn c_game_load_screen_dtor_matches() {
        dtor_case(ee::s06::f_00243b80__generated, c_game_load_screen_dtor, 0x243b80);
    }

    #[test]
    fn c_fe_load_screen_dtor_matches() {
        dtor_case(ee::s06::f_00243c00__generated, c_fe_load_screen_dtor, 0x243c00);
    }

    #[test]
    fn game_state_dtor_matches() {
        dtor_case(ee::s06::f_00243c80__generated, game_state_dtor, 0x243c80);
    }

    #[test]
    fn game_state_quit_v01_matches() {
        dtor_case(ee::s06::f_00243cf0__generated, game_state_quit_v01, 0x243cf0);
    }

    #[test]
    fn game_state_idle_v01_matches() {
        dtor_case(ee::s06::f_00243d58__generated, game_state_idle_v01, 0x243d58);
    }

    #[test]
    fn game_state_finish_v01_matches() {
        dtor_case(ee::s06::f_00243d88__generated, game_state_finish_v01, 0x243d88);
    }

    #[test]
    fn game_state_reset_ai_v01_matches() {
        dtor_case(ee::s06::f_00243db8__generated, game_state_reset_ai_v01, 0x243db8);
    }

    #[test]
    fn game_state_pre_race_load_v01_matches() {
        dtor_case(ee::s06::f_00243e20__generated, game_state_pre_race_load_v01, 0x243e20);
    }

    #[test]
    fn game_state_countdown_v01_matches() {
        dtor_case(ee::s06::f_00243e50__generated, game_state_countdown_v01, 0x243e50);
    }

    #[test]
    fn game_state_race_v01_matches() {
        dtor_case(ee::s06::f_00243eb8__generated, game_state_race_v01, 0x243eb8);
    }

    #[test]
    fn game_state_reset_v01_matches() {
        dtor_case(ee::s06::f_00243f20__generated, game_state_reset_v01, 0x243f20);
    }

    #[test]
    fn game_state_restart_race_v01_matches() {
        dtor_case(ee::s06::f_00243f50__generated, game_state_restart_race_v01, 0x243f50);
    }

    #[test]
    fn game_state_warp_to_event_v01_matches() {
        dtor_case(ee::s06::f_00243fc0__generated, game_state_warp_to_event_v01, 0x243fc0);
    }

    #[test]
    fn game_state_event_select_v01_matches() {
        dtor_case(ee::s06::f_00244028__generated, game_state_event_select_v01, 0x244028);
    }

    #[test]
    fn game_state_event_switch_load_v01_matches() {
        dtor_case(ee::s06::f_00244060__generated, game_state_event_switch_load_v01, 0x244060);
    }

    #[test]
    fn game_state_pre_race_menu_v01_matches() {
        dtor_case(ee::s06::f_00244090__generated, game_state_pre_race_menu_v01, 0x244090);
    }

    #[test]
    fn game_state_results_v01_matches() {
        dtor_case(ee::s06::f_002440f8__generated, game_state_results_v01, 0x2440f8);
    }

    #[test]
    fn game_state_finish_cinematic_v01_matches() {
        dtor_case(ee::s06::f_00244160__generated, game_state_finish_cinematic_v01, 0x244160);
    }

    #[test]
    fn game_state_replay_playback_v01_matches() {
        dtor_case(ee::s06::f_002441c8__generated, game_state_replay_playback_v01, 0x2441c8);
    }

    #[test]
    fn generic_stream_pause_v01_matches() {
        dtor_case(ee::s06::f_002443c0__generated, generic_stream_pause_v01, 0x2443c0);
    }

    #[test]
    fn gm_handler_dtor_matches() {
        dtor_case(ee::s06::f_002444f0__generated, gm_handler_dtor, 0x2444f0);
    }

    #[test]
    fn gm_handler0_v01_matches() {
        dtor_case(ee::s06::f_00244560__generated, gm_handler0_v01, 0x244560);
    }

    #[test]
    fn gm_handler1_v01_matches() {
        dtor_case(ee::s06::f_00244590__generated, gm_handler1_v01, 0x244590);
    }
}
