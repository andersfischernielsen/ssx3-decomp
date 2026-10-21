//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for front-end state ctors/dtors, the net-FE child-event
//! handlers, memory-card manager states, HUD label setters, lobby/buddy
//! manager helpers and the net event clone constructors, each with a
//! differential test against the generated body.
//!
//! The functions keep their state in the guest registers the originals use
//! (callers, callees and event handlers see the same values), follow the call
//! protocol of [`crate::call`] (same-shard direct calls through the generated
//! aliases, cross-shard calls and `jalr` through the function table) and the
//! event rules of [`crate::events`]: the two structured loops deliver events at
//! their back edge with [`deliver_event_here`], `BuddyMan_ClearMessages` is a
//! `match pc` dispatcher whose loop head (`0x261088`) is a resume entry and
//! returns when an event asks the thread to unwind.

use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

#[rustfmt::skip]
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0018_87a0, cfe_state_options_ctor, "cFEStateOptions_ctor"),
    (0x0019_5948, fe_locale_unload_all_tables, "FELocale_UnloadAllTables"),
    (0x0019_5e58, fe_locale_unload_game_tables, "FELocale_UnloadGameTables"),
    (0x001a_03c0, fe_module_init_rider_previews, "FEModule_InitRiderPreviews"),
    (0x001a_3bf0, fe_async_manager_find_free_slot, "FEAsyncManager_FindFreeSlot"),
    (0x001a_b370, net_fe_state_show_buddy_action_popup, "NetFEState_ShowBuddyActionPopup"),
    (0x001a_fe08, cfe_state_challenge_received_popup_ctor, "cFEStateChallengeReceivedPopup_ctor"),
    (0x001b_5cf8, cfe_state_net_config_on_info_popup_closed, "cFEStateNetConfig_OnInfoPopupClosed"),
    (0x001b_9428, cfe_state_create_step1_on_child_event, "cFEStateCreateStep1_OnChildEvent"),
    (0x001b_dde8, cfe_state_lobby_menu_on_child_event, "cFEStateLobbyMenu_OnChildEvent"),
    (0x001b_dfd8, cfe_state_lobby_menu_hide_room_tabs, "cFEStateLobbyMenu_HideRoomTabs"),
    (0x001c_02f8, cfe_state_online_main_menu_on_child_event, "cFEStateOnlineMainMenu_OnChildEvent"),
    (0x001c_2b48, cfe_state_net_quick_messages_on_child_event, "cFEStateNetQuickMessages_OnChildEvent"),
    (0x001c_a8a0, cui_state_buy_popup_reset_params, "cUIStateBuyPopup_ResetParams"),
    (0x001d_3160, cfe_state_mountain_room_request_image, "cFEStateMountainRoom_RequestImage"),
    (0x001d_9308, mem_card_state_base_set_use_save_title, "MemCardStateBase_SetUseSaveTitle"),
    (0x001d_9818, mem_card_state_base_on_activate, "MemCardStateBase_OnActivate"),
    (0x001d_f7c8, cfe_state_lobby_menu_dtor, "cFEStateLobbyMenu_dtor"),
    (0x001d_fa10, cfe_state_reward_gallery_dtor, "cFEStateRewardGallery_dtor"),
    (0x001d_fb28, cfe_state_reward_gallery_posters_dtor, "cFEStateRewardGalleryPosters_dtor"),
    (0x001d_fbf8, cfe_state_reward_gallery_trading_cards_dtor, "cFEStateRewardGalleryTradingCards_dtor"),
    (0x001d_fcc8, cfe_state_reward_gallery_concept_arts_dtor, "cFEStateRewardGalleryConceptArts_dtor"),
    (0x001d_fd98, cfe_state_reward_gallery_videos_dtor, "cFEStateRewardGalleryVideos_dtor"),
    (0x001d_fe68, cfe_state_reward_gallery_toys_dtor, "cFEStateRewardGalleryToys_dtor"),
    (0x001d_ff38, cfe_state_reward_gallery_cheat_chars_dtor, "cFEStateRewardGalleryCheatChars_dtor"),
    (0x001e_02f8, cfe_popup_select_multiplayer_mode_init, "cFEPopupSelectMultiplayerMode_Init"),
    (0x001e_09a0, cfe_state_challenge_popup_dtor, "cFEStateChallengePopup_dtor"),
    (0x001e_3388, message_center_find_message, "MessageCenter_FindMessage"),
    (0x001e_3510, message_center_reset_new_player_flag, "MessageCenter_ResetNewPlayerFlag"),
    (0x001e_94e0, hud_set_icon_text, "HUD_SetIconText"),
    (0x001e_9540, hud_set_icon_text_w, "HUD_SetIconTextW"),
    (0x0020_95e8, ov_screen_session_set_highlight_color, "OVScreenSession_SetHighlightColor"),
    (0x0020_9648, ov_screen_session_set_unlocked_color, "OVScreenSession_SetUnlockedColor"),
    (0x0022_81d8, ssx_app_start_game_module, "SSXApp_StartGameModule"),
    (0x0023_ea30, mc_manager_end_operation, "MCManager_EndOperation"),
    (0x0023_fb58, mc_manager_reset, "MCManager_Reset"),
    (0x0024_0800, mc_manager_state1c, "MCManager_State1C"),
    (0x0024_0c30, mc_manager_state29, "MCManager_State29"),
    (0x0024_1240, mc_manager_finish_load, "MCManager_FinishLoad"),
    (0x0024_1ac0, mc_manager_get_num_slots, "MCManager_GetNumSlots"),
    (0x0024_38f0, ssx_app_dtor, "SSXApp_dtor"),
    (0x0025_1f68, mem_debug_printf, "MEM_DebugPrintf"),
    (0x0025_3a40, vid_engine_player_resume, "VidEnginePlayer_Resume"),
    (0x0025_4dc0, vd_texture_sink_flush, "VD_TextureSink_Flush"),
    (0x0025_d800, lobby_man_move_to_room_index, "LobbyMan_MoveToRoomIndex"),
    (0x0025_dd08, lobby_man_ping_user, "LobbyMan_PingUser"),
    (0x0026_1070, buddy_man_clear_messages, "BuddyMan_ClearMessages"),
    (0x0026_7ec8, net_event102_clone, "NetEvent102_Clone"),
    (0x0026_7f58, net_event102_base_clone, "NetEvent102Base_Clone"),
    (0x0026_7fe8, net_error_event_clone, "NetErrorEvent_Clone"),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    // Same-shard direct calls (`jal`, emitted by the generator as the shard's
    // readable alias).
    pub const FE_LOCALE_UNLOAD_TABLE: Callee = Callee {
        addr: 0x0019_5a50,
        f: ee::s02::FELocale_UnloadTable,
    };
    pub const FE_MODULE_RIDER_PREVIEW_CTOR: Callee = Callee {
        addr: 0x0019_e3d0,
        f: ee::s03::FEModule_RiderPreviewCtor,
    };
    pub const FE_ASYNC_MANAGER_GET_STATE: Callee = Callee {
        addr: 0x001a_39c0,
        f: ee::s03::FEAsyncManager_GetState,
    };
    pub const NET_FE_STATE_GET_POPUP: Callee = Callee {
        addr: 0x001a_9c50,
        f: ee::s03::NetFEState_GetPopup,
    };
    pub const NET_FE_STATE_SETUP_BUDDY_ACTION_POPUP: Callee = Callee {
        addr: 0x001a_b478,
        f: ee::s03::NetFEState_SetupBuddyActionPopup,
    };
    pub const NET_FE_STATE_CTOR: Callee = Callee {
        addr: 0x001a_8500,
        f: ee::s03::NetFEState_ctor,
    };
    pub const NET_FE_STATE_ON_CHILD_EVENT: Callee = Callee {
        addr: 0x001a_97b8,
        f: ee::s03::NetFEState_OnChildEvent,
    };
    pub const CFE_STATE_CREATE_STEP1_ON_SAVE_ACCOUNT_ANSWER: Callee = Callee {
        addr: 0x001b_91c8,
        f: ee::s03::f_001b91c8,
    };
    pub const CFE_STATE_CREATE_STEP1_ON_EDIT_DONE_POPUP_EVENT: Callee = Callee {
        addr: 0x001b_92c8,
        f: ee::s03::f_001b92c8,
    };
    pub const CFE_STATE_LOBBY_MENU_ON_CREATE_ROOM_POPUP_EVENT: Callee = Callee {
        addr: 0x001b_dcf8,
        f: ee::s03::f_001bdcf8,
    };
    pub const CFE_STATE_LOBBY_MENU_PROMPT_ROOM_PASSWORD: Callee = Callee {
        addr: 0x001b_dd60,
        f: ee::s03::f_001bdd60,
    };
    pub const CFE_STATE_ONLINE_MAIN_MENU_ON_LEAVE_POPUP_EVENT: Callee = Callee {
        addr: 0x001c_0138,
        f: ee::s03::f_001c0138,
    };
    pub const CFE_STATE_ONLINE_MAIN_MENU_ON_DISCONNECT_POPUP_EVENT: Callee = Callee {
        addr: 0x001c_0240,
        f: ee::s03::f_001c0240,
    };
    pub const CFE_STATE_NET_QUICK_MESSAGES_ON_SAVE_POPUP_EVENT: Callee = Callee {
        addr: 0x001c_2c50,
        f: ee::s03::f_001c2c50,
    };
    pub const CFE_STATE_NET_QUICK_MESSAGES_ON_RESET_POPUP_EVENT: Callee = Callee {
        addr: 0x001c_2ba8,
        f: ee::s03::f_001c2ba8,
    };
    pub const CFE_STATE_REWARD_GALLERY_V29: Callee = Callee {
        addr: 0x001d_fac8,
        f: ee::s04::cFEStateRewardGallery_v29,
    };
    pub const CFE_STATE_REWARD_GALLERY_RELEASE_TEXTURES: Callee = Callee {
        addr: 0x001d_00c0,
        f: ee::s04::cFEStateRewardGallery_ReleaseTextures,
    };
    pub const MESSAGE_CENTER_GET_INBOX: Callee = Callee {
        addr: 0x001e_39f8,
        f: ee::s04::MessageCenter_GetInbox,
    };
    pub const HUD_LAYOUT_ICON_TEXT: Callee = Callee {
        addr: 0x001e_92a8,
        f: ee::s04::HUD_LayoutIconText,
    };
    pub const GAME_MODULE_CTOR: Callee = Callee {
        addr: 0x0022_e968,
        f: ee::s05::GameModule_ctor,
    };
    pub const MC_MANAGER_END_PROGRESS: Callee = Callee {
        addr: 0x0024_1180,
        f: ee::s06::f_00241180,
    };
    pub const MC_MANAGER_SET_MODE: Callee = Callee {
        addr: 0x0023_d570,
        f: ee::s06::MCManager_SetMode,
    };
    pub const MC_MANAGER_END_OPERATION: Callee = Callee {
        addr: 0x0023_ea30,
        f: ee::s06::MCManager_EndOperation,
    };
    pub const MC_MANAGER_REQUEST_CARD_INFO: Callee = Callee {
        addr: 0x0023_fb20,
        f: ee::s06::MCManager_RequestCardInfo,
    };
    pub const LOBBY_MAN_GET_ROOM_NAME: Callee = Callee {
        addr: 0x0025_f150,
        f: ee::s06::LobbyMan_GetRoomName,
    };
    pub const LOBBY_MAN_MOVE_TO_ROOM: Callee = Callee {
        addr: 0x0025_d860,
        f: ee::s06::LobbyMan_MoveToRoom,
    };
    pub const LOBBY_MAN_LOOKUP_PEER_ADDR: Callee = Callee {
        addr: 0x0025_a730,
        f: ee::s06::LobbyMan_LookupPeerAddr,
    };
}

// ---------------------------------------------------------------------------
// Small helpers.

/// The address `a + off` (32-bit guest address arithmetic).
#[inline(always)]
fn p(a: u64, off: u32) -> u32 {
    (a as u32).wrapping_add(off)
}

#[inline(always)]
fn lw(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lw(c)
}

#[inline(always)]
fn sw(c: &mut Cpu, a: u32, v: u64) {
    Addr(a).sw(c, v);
}

#[inline(always)]
fn lh(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lh(c)
}

#[inline(always)]
fn lb(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lb(c)
}

#[inline(always)]
fn lbu(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lbu(c)
}

#[inline(always)]
fn sh(c: &mut Cpu, a: u32, v: u64) {
    Addr(a).sh(c, v);
}

/// `gp + off` (globals).
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `sd reg, off(sp)`: a register the original spills through the frame.
#[inline(always)]
fn save_reg(c: &mut Cpu, frame: Frame, reg: usize, off: u32) {
    let a = frame.at(c, off);
    let v = c.g[reg][0];
    c.sd(a, v);
}

// ---------------------------------------------------------------------------
// FE state construction / locale tables.

/// `cFEStateOptions_ctor` (0x1887a0): construct the options screen state:
/// the base `UIList` (`0x39e2a0`), vtable `0x46c7c0`, size `0x18`, and, when
/// `reset` is non-null, reset the flow-state selection to entry `0x18`.
pub fn cfe_state_options_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        call_ptr(c, 0x0039_e2a0, 0x0018_87bc)?;
        c.g[V1][0] = 0x0047_0000;
        c.g[V0][0] = 0x18;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3840));
        sw(c, p(c.g[S1][0], 0xc), c.g[V0][0]);
        let reset = c.g[S0][0] == 0;
        sw(c, p(c.g[S1][0], 8), c.g[V1][0]);
        if !reset {
            c.cycles += 4;
            c.g[A0][0] = sx(gp(c, 0x1d90));
            c.g[A1][0] = 0x18;
            c.g[A2][0] = 0;
            call_ptr(c, 0x001a_0708, 0x0018_87e4)?;
        }
        c.cycles += 6;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `FELocale_UnloadAllTables(FELocale* self)` (0x195948): free all 20 locale
/// tables (indexes 0..18; the counter starts at `0x1000000`, the loop shifts
/// it down by 24 bits and stops at 20). Returns `self`.
pub fn fe_locale_unload_all_tables(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0x0100_0000;
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = 0x0100_0000;
        loop {
            c.cycles += 7;
            c.g[A0][0] = c.g[S1][0];
            callee::FE_LOCALE_UNLOAD_TABLE.call(c, 0x0019_5978)?;
            c.g[V0][0] = c.g[S0][0];
            c.g[A1][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            c.g[V0][0] = ((c.g[A1][0] as i64) < 20i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_5970);
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `FELocale_UnloadGameTables(FELocale* self)` (0x195e58): unless the locale
/// has the "game tables loaded" flag (byte `+3` bit 0), free tables 0xc..0xf.
pub fn fe_locale_unload_game_tables(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lb(c, p(c.g[S0][0], 3));
        c.g[V0][0] = c.g[V0][0] & 1;
        if c.g[V0][0] == 0 {
            c.cycles += 12;
            c.g[A1][0] = 0xc;
            callee::FE_LOCALE_UNLOAD_TABLE.call(c, 0x0019_5e80)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0xd;
            callee::FE_LOCALE_UNLOAD_TABLE.call(c, 0x0019_5e8c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0xe;
            callee::FE_LOCALE_UNLOAD_TABLE.call(c, 0x0019_5e98)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0xf;
            callee::FE_LOCALE_UNLOAD_TABLE.call(c, 0x0019_5ea4)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* FEModule_InitRiderPreviews(FEModule* self)` (0x1a03c0): construct
/// the two rider-preview objects 0xce0 bytes apart; returns `self`.
pub fn fe_module_init_rider_previews(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[S2][0];
        frame.save(c, S3, 0x10);
        c.g[S1][0] = 1;
        frame.save_ra(c, 0);
        c.g[S3][0] = 0xffff_ffff_ffff_ffff;
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            callee::FE_MODULE_RIDER_PREVIEW_CTOR.call(c, 0x001a_03f4)?;
            let more = c.g[S1][0] != c.g[S3][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xce0));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001a_03e8);
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int FEAsyncManager_FindFreeSlot(FEAsyncManager* m)` (0x1a3bf0): the first
/// of the five async slots whose state word is 0, or -1.
pub fn fe_async_manager_find_free_slot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = 0;
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A0][0] = c.g[S1][0];
        loop {
            c.cycles += 4;
            c.g[A1][0] = c.g[S0][0];
            callee::FE_ASYNC_MANAGER_GET_STATE.call(c, 0x001a_3c18)?;
            if c.g[V0][0] != 0 {
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.g[S0][0];
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 5i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S1][0];
            if !more {
                c.cycles += 1;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001a_3c10);
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Net FE popups and child-event handlers.

/// `void NetFEState_ShowBuddyActionPopup(NetFEState* self, int a1, int a2)`
/// (0x1ab370): mark the buddy-action popup as shown and set it up.
pub fn net_fe_state_show_buddy_action_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        callee::NET_FE_STATE_GET_POPUP.call(c, 0x001a_b394)?;
        c.g[V1][0] = 1;
        c.g[A0][0] = c.g[S0][0];
        sw(c, p(c.g[S0][0], 0x6c4), c.g[V1][0]);
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[T0][0] = 0;
        callee::NET_FE_STATE_SETUP_BUDDY_ACTION_POPUP.call(c, 0x001a_b3b4)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateChallengeReceivedPopup_ctor` (0x1afe08): the net-FE state base
/// plus the challenge-received popup's vtable, name string, and the global
/// "popup shown" word cleared.
pub fn cfe_state_challenge_received_popup_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A3][0];
        callee::NET_FE_STATE_CTOR.call(c, 0x001a_fe24)?;
        c.g[V1][0] = 0x0047_0000;
        c.g[A1][0] = c.g[S1][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x72e0));
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6d0));
        sw(c, p(c.g[S0][0], 8), c.g[V1][0]);
        call_ptr(c, 0x0031_82d8, 0x001a_fe3c)?;
        c.g[V1][0] = 0x40;
        sw(c, p(c.g[S0][0], 0x6d4), 0);
        sw(c, p(c.g[S0][0], 0xc), c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        sw(c, gp(c, 0x1d98), 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateNetConfig_OnInfoPopupClosed(NetFEState* self, void* ev,
/// int arg)` (0x1b5cf8): on the info-popup event (0x16) also close the popup
/// object's info, then pass the event to the base handler.
pub fn cfe_state_net_config_on_info_popup_closed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x16;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let info = c.g[A2][0] == c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if info {
            c.cycles += 11;
            c.g[A2][0] = 0x16;
            callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, 0x001b_5d18)?;
            c.g[A2][0] = lw(c, p(c.g[S0][0], 0x6ec));
            c.g[A1][0] = 0;
            c.g[V0][0] = lw(c, p(c.g[A2][0], 8));
            c.g[A0][0] = lh(c, p(c.g[V0][0], 0x40));
            c.g[V1][0] = lw(c, p(c.g[V0][0], 0x44));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001b_5d34)?;
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, 0x001b_5d44)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Dispatch shape shared by the four net-FE child-event handlers below:
/// compare the event id at `ev+0x18` with two ids and call the matching
/// handler (or the base handler), each returning to its own address.
macro_rules! child_event_switch {
    ($fname:ident, $doc:expr, $ev:expr, $a:expr, $b:expr, $ra:expr, $rb:expr,
     $base:expr, $fa:expr, $fb:expr) => {
        #[doc = $doc]
        pub fn $fname(c: &mut Cpu) {
            guest(c, |c| {
                c.cycles += 6;
                let frame = Frame::open(c, 0x10);
                c.g[V0][0] = $a;
                frame.save_ra(c, 0);
                c.g[V1][0] = lw(c, p(c.g[$ev][0], 0x18));
                let first = c.g[V1][0] == c.g[V0][0];
                let second;
                if first {
                    second = false;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = $b;
                    second = c.g[V1][0] == c.g[V0][0];
                    if !second {
                        c.cycles += 2;
                    }
                }
                if first {
                    c.cycles += 4;
                    $fa.call(c, $ra)?;
                } else if second {
                    c.cycles += 4;
                    $fb.call(c, $rb)?;
                } else {
                    c.cycles += 3;
                    callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, $base)?;
                }
                frame.restore_ra(c, 0);
                c.cycles += 2;
                frame.close(c);
                Ok(())
            })
        }
    };
}

child_event_switch!(
    cfe_state_create_step1_on_child_event,
    "`void cFEStateCreateStep1_OnChildEvent(NetFEState* self, void* ev, int arg)` \
     (0x1b9428): 0x81e -> save-account answer, 0x82c -> edit-done popup, else \
     the base handler.",
    A1,
    0x81e,
    0x82c,
    0x001b_9458,
    0x001b_9468,
    0x001b_9478,
    callee::CFE_STATE_CREATE_STEP1_ON_SAVE_ACCOUNT_ANSWER,
    callee::CFE_STATE_CREATE_STEP1_ON_EDIT_DONE_POPUP_EVENT
);

child_event_switch!(
    cfe_state_lobby_menu_on_child_event,
    "`void cFEStateLobbyMenu_OnChildEvent(NetFEState* self, void* ev, int arg)` \
     (0x1bdde8): 0x823 -> create-room popup, 0x82a -> room password prompt, \
     else the base handler.",
    A1,
    0x823,
    0x82a,
    0x001b_de18,
    0x001b_de28,
    0x001b_de38,
    callee::CFE_STATE_LOBBY_MENU_ON_CREATE_ROOM_POPUP_EVENT,
    callee::CFE_STATE_LOBBY_MENU_PROMPT_ROOM_PASSWORD
);

child_event_switch!(
    cfe_state_online_main_menu_on_child_event,
    "`void cFEStateOnlineMainMenu_OnChildEvent(NetFEState* self, void* ev, int arg)` \
     (0x1c02f8): 0x82d -> leave popup, 0x846 -> disconnect popup, else the \
     base handler.",
    A1,
    0x82d,
    0x846,
    0x001c_0328,
    0x001c_0338,
    0x001c_0348,
    callee::CFE_STATE_ONLINE_MAIN_MENU_ON_LEAVE_POPUP_EVENT,
    callee::CFE_STATE_ONLINE_MAIN_MENU_ON_DISCONNECT_POPUP_EVENT
);

child_event_switch!(
    cfe_state_net_quick_messages_on_child_event,
    "`void cFEStateNetQuickMessages_OnChildEvent(NetFEState* self, void* ev, int arg)` \
     (0x1c2b48): 0x83a -> save popup, 0x83b -> reset popup, else the base \
     handler.",
    A1,
    0x83a,
    0x83b,
    0x001c_2b78,
    0x001c_2b88,
    0x001c_2b98,
    callee::CFE_STATE_NET_QUICK_MESSAGES_ON_SAVE_POPUP_EVENT,
    callee::CFE_STATE_NET_QUICK_MESSAGES_ON_RESET_POPUP_EVENT
);

/// `void cFEStateLobbyMenu_HideRoomTabs(cFEStateLobbyMenu* self)` (0x1bdfd8):
/// hide the four room tabs at `self+0x730` through their vtable slot 0x48.
pub fn cfe_state_lobby_menu_hide_room_tabs(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x730));
        frame.save_ra(c, 0);
        c.g[S1][0] = 3;
        c.g[A2][0] = lw(c, p(c.g[S0][0], 0));
        loop {
            c.cycles += 10;
            c.g[A1][0] = 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            c.g[V0][0] = lw(c, p(c.g[A2][0], 8));
            c.g[A0][0] = lh(c, p(c.g[V0][0], 0x48));
            c.g[V1][0] = lw(c, p(c.g[V0][0], 0x4c));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001b_e018)?;
            if (c.g[S1][0] as i64) < 0 {
                break;
            }
            c.g[A2][0] = lw(c, p(c.g[S0][0], 0));
            if event_due(c) {
                deliver_event_here(c, 0x001b_dff8);
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cUIStateBuyPopup_ResetParams(cUIStateBuyPopup* self)` (0x1ca8a0):
/// zero the three 8-byte parameter blocks at +0xc, +0x14, +0x1c and the three
/// words at +0, +4, +8.
pub fn cui_state_buy_popup_reset_params(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 8;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
        call_ptr(c, 0x003e_6448, 0x001c_a8c0)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = 0;
        c.g[A2][0] = 8;
        call_ptr(c, 0x003e_6448, 0x001c_a8d0)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[A1][0] = 0;
        c.g[A2][0] = 8;
        call_ptr(c, 0x003e_6448, 0x001c_a8e0)?;
        sw(c, p(c.g[S0][0], 8), 0);
        sw(c, p(c.g[S0][0], 0), 0);
        sw(c, p(c.g[S0][0], 4), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateMountainRoom_RequestImage(cFEStateMountainRoom* self)`
/// (0x1d3160): if the image request has not started (`+0xbc < 0`) and an item
/// is queued (`+0xc0`), start loading item `+0xb0` through the bolt file of
/// the world's rider-0 manager.
pub fn cfe_state_mountain_room_request_image(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw(c, p(c.g[S0][0], 0xbc));
        if (c.g[V0][0] as i64) >= 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            c.g[V1][0] = lw(c, p(c.g[S0][0], 0xc0));
            let no_item = c.g[V1][0] == 0;
            c.g[V0][0] = lw(c, gp(c, -0x848));
            if !no_item {
                c.cycles += 9;
                c.g[A3][0] = 0xa;
                c.g[A2][0] = lw(c, p(c.g[V1][0], 4));
                c.g[T0][0] = 1;
                c.g[A0][0] = lw(c, p(c.g[V0][0], 0x7c));
                c.g[T1][0] = 0;
                c.g[A1][0] = lw(c, p(c.g[S0][0], 0xb0));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a70));
                call_ptr(c, 0x0019_da10, 0x001d_31a8)?;
                sw(c, p(c.g[S0][0], 0xbc), c.g[V0][0]);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MemCardStateBase_SetUseSaveTitle(MemCardStateBase* self, int on)`
/// (0x1d9308): store the flag and, when on, let the overlay manager's current
/// movie player know that the state now owns the title area.
pub fn mem_card_state_base_set_use_save_title(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let off = c.g[A1][0] == 0;
        sw(c, p(c.g[S0][0], 0x1f4), c.g[A1][0]);
        if !off {
            c.cycles += 12;
            c.g[A0][0] = lw(c, gp(c, -0x848));
            call_ptr(c, 0x0022_7f80, 0x001d_9328)?;
            c.g[A3][0] = lw(c, p(c.g[V0][0], 0x434));
            c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9c));
            c.g[A1][0] = lw(c, p(c.g[V0][0], 0xf8));
            c.g[V0][0] = lw(c, p(c.g[A3][0], 0));
            c.g[A0][0] = lh(c, p(c.g[V0][0], 0x70));
            c.g[V1][0] = lw(c, p(c.g[V0][0], 0x74));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001d_9348)?;
        } else {
            c.cycles += 2;
            sh(c, p(c.g[S0][0], 0x9c), 0);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MemCardStateBase_OnActivate(MemCardStateBase* self)` (0x1d9818):
/// activate the UI screen, then show the "busy" widget found by its name hash.
pub fn mem_card_state_base_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e4c0, 0x001d_982c)?;
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x78b0));
        call_ptr(c, 0x0031_7670, 0x001d_9838)?;
        c.g[A0][0] = lw(c, p(c.g[S0][0], 0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001d_9844)?;
        c.g[A2][0] = c.g[V0][0];
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = lw(c, p(c.g[A2][0], 8));
            c.g[A0][0] = lh(c, p(c.g[V0][0], 0x48));
            c.g[V1][0] = lw(c, p(c.g[V0][0], 0x4c));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001d_9864)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateLobbyMenu_dtor` (0x1df7c8): destroy the two embedded strings and
/// then the net-FE state base.
pub fn cfe_state_lobby_menu_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0047_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5808));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        sw(c, p(c.g[S0][0], 8), c.g[V0][0]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xb80));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x001d_f7f8)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x76c));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x001d_f804)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x001a_85d0, 0x001d_f810)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared body of the seven reward-gallery destructors: gallery vtable,
/// release the gallery's textures, base vtable, then the UI-list destructor.
macro_rules! reward_gallery_dtor {
    ($fname:ident, $doc:expr, $a:expr, $b:expr, $c:expr) => {
        #[doc = $doc]
        pub fn $fname(c: &mut Cpu) {
            guest(c, |c| {
                c.cycles += 23;
                let frame = Frame::open(c, 0x30);
                c.g[V0][0] = 0x0047_0000;
                frame.save(c, S0, 0x20);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5c38));
                c.g[S0][0] = c.g[A0][0];
                frame.save(c, S1, 0x10);
                frame.save_ra(c, 0);
                c.g[S1][0] = c.g[A1][0];
                sw(c, p(c.g[S0][0], 8), c.g[V0][0]);
                callee::CFE_STATE_REWARD_GALLERY_V29.call(c, $a)?;
                c.g[A0][0] = c.g[S0][0];
                callee::CFE_STATE_REWARD_GALLERY_RELEASE_TEXTURES.call(c, $b)?;
                c.g[V0][0] = 0x0047_0000;
                c.g[A0][0] = c.g[S0][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2f30));
                c.g[A1][0] = c.g[S1][0];
                sw(c, p(c.g[S0][0], 8), c.g[V0][0]);
                call_ptr(c, 0x0039_e390, $c)?;
                frame.restore(c, S0, 0x20);
                frame.restore(c, S1, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                Ok(())
            })
        }
    };
}

reward_gallery_dtor!(
    cfe_state_reward_gallery_dtor,
    "`cFEStateRewardGallery_dtor` (0x1dfa10): destroy the reward-gallery screen.",
    0x001d_fa38,
    0x001d_fa40,
    0x001d_fa58
);
reward_gallery_dtor!(
    cfe_state_reward_gallery_posters_dtor,
    "`cFEStateRewardGalleryPosters_dtor` (0x1dfb28): destroy the posters screen.",
    0x001d_fb50,
    0x001d_fb58,
    0x001d_fb70
);
reward_gallery_dtor!(
    cfe_state_reward_gallery_trading_cards_dtor,
    "`cFEStateRewardGalleryTradingCards_dtor` (0x1dfbf8): destroy the trading-cards screen.",
    0x001d_fc20,
    0x001d_fc28,
    0x001d_fc40
);
reward_gallery_dtor!(
    cfe_state_reward_gallery_concept_arts_dtor,
    "`cFEStateRewardGalleryConceptArts_dtor` (0x1dfcc8): destroy the concept-arts screen.",
    0x001d_fcf0,
    0x001d_fcf8,
    0x001d_fd10
);
reward_gallery_dtor!(
    cfe_state_reward_gallery_videos_dtor,
    "`cFEStateRewardGalleryVideos_dtor` (0x1dfd98): destroy the videos screen.",
    0x001d_fdc0,
    0x001d_fdc8,
    0x001d_fde0
);
reward_gallery_dtor!(
    cfe_state_reward_gallery_toys_dtor,
    "`cFEStateRewardGalleryToys_dtor` (0x1dfe68): destroy the toys screen.",
    0x001d_fe90,
    0x001d_fe98,
    0x001d_feb0
);
reward_gallery_dtor!(
    cfe_state_reward_gallery_cheat_chars_dtor,
    "`cFEStateRewardGalleryCheatChars_dtor` (0x1dff38): destroy the cheat-characters screen.",
    0x001d_ff60,
    0x001d_ff68,
    0x001d_ff80
);

/// `void cFEPopupSelectMultiplayerMode_Init(cFEPopupSelectMultiplayerMode* self)`
/// (0x1e02f8): name-hash the popup, add its screen to the UI engine and play
/// its first frame.
pub fn cfe_popup_select_multiplayer_mode_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(8));
        call_ptr(c, 0x0031_7670, 0x001e_0314)?;
        c.g[A0][0] = lw(c, p(c.g[S0][0], 0x10));
        c.g[A2][0] = c.g[V0][0];
        sw(c, p(c.g[S0][0], 0xc), c.g[V0][0]);
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001e_032c)?;
        let no_screen = c.g[V0][0] == 0;
        sw(c, p(c.g[S0][0], 0x40), c.g[V0][0]);
        if !no_screen {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001e_0344)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateChallengePopup_dtor` (0x1e09a0): notify the owned object (vtable
/// slot 0x100), then destroy the net-FE state base.
pub fn cfe_state_challenge_popup_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0047_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x71c8));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        sw(c, p(c.g[S0][0], 8), c.g[V0][0]);
        c.g[A2][0] = lw(c, p(c.g[S0][0], 0x20));
        c.g[V0][0] = lw(c, p(c.g[A2][0], 8));
        c.g[A0][0] = lh(c, p(c.g[V0][0], 0x100));
        c.g[V1][0] = lw(c, p(c.g[V0][0], 0x104));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001e_09dc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x001a_85d0, 0x001e_09e8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int MessageCenter_FindMessage(int id)` (0x1e3388): index of the message
/// with that id in the inbox list, or -1.
pub fn message_center_find_message(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MESSAGE_CENTER_GET_INBOX.call(c, 0x001e_339c)?;
        c.g[A0][0] = lbu(c, p(c.g[V0][0], 0xcc));
        let empty = c.g[A0][0] == 0;
        c.g[V1][0] = 0;
        let mut found = false;
        if !empty {
            c.g[A1][0] = c.g[A0][0];
            c.g[A0][0] = c.g[V0][0];
            loop {
                c.cycles += 3;
                c.g[V0][0] = lw(c, p(c.g[A0][0], 0));
                if c.g[S0][0] == c.g[V0][0] {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[V1][0];
                    found = true;
                    break;
                }
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(8));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001e_33b0);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MessageCenter_ResetNewPlayerFlag()` (0x1e3510): clear the "new
/// player" interface state, and if the global new-player flag is set, clear
/// it and enable the new-player interface's slot-3 flag.
pub fn message_center_reset_new_player_flag(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0014_dc80, 0x001e_3520)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x001e_352c)?;
        c.g[V1][0] = 0x0053_0000;
        c.g[V0][0] = lb(c, (c.g[V1][0] as u32).wrapping_add(0x5c11));
        let set = c.g[V0][0] != 0;
        if !set {
            c.cycles += 10;
            sw(c, gp(c, -0x1024), 0);
            call_ptr(c, 0x0014_dc80, 0x001e_3544)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x0014_dd58, 0x001e_3550)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 1;
            call_ptr(c, 0x0014_75c0, 0x001e_3560)?;
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void HUD_SetIconText(HUD_Icon* self, const char* str, int a2,
/// const Vec2* pos)` (0x1e94e0): copy the ASCII label into the object, store
/// the id and position, and lay the icon text out.
pub fn hud_set_icon_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        c.g[S2][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4c));
        call_ptr(c, 0x002c_2540, 0x001e_9508)?;
        sw(c, p(c.g[S0][0], 8), c.g[S1][0]);
        c.ldl(2, p(c.g[S2][0], 7));
        c.ldr(2, p(c.g[S2][0], 0));
        c.sdl(2, p(c.g[S0][0], 0x17));
        c.sdr(2, p(c.g[S0][0], 0x10));
        c.g[A0][0] = c.g[S0][0];
        callee::HUD_LAYOUT_ICON_TEXT.call(c, 0x001e_9524)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void HUD_SetIconTextW(HUD_Icon* self, const u16* str, int a2,
/// const Vec2* pos)` (0x1e9540): the wide-character variant of
/// [`hud_set_icon_text`].
pub fn hud_set_icon_text_w(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        c.g[S2][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4c));
        call_ptr(c, 0x002c_2508, 0x001e_9568)?;
        sw(c, p(c.g[S0][0], 8), c.g[S1][0]);
        c.ldl(2, p(c.g[S2][0], 7));
        c.ldr(2, p(c.g[S2][0], 0));
        c.sdl(2, p(c.g[S0][0], 0x17));
        c.sdr(2, p(c.g[S0][0], 0x10));
        c.g[A0][0] = c.g[S0][0];
        callee::HUD_LAYOUT_ICON_TEXT.call(c, 0x001e_9584)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenSession_SetHighlightColor(Session* s, void* obj, int on)`
/// (0x2095e8): forward the highlight colour (`D_004C8B38`) or the default
/// colour (`D_004C8B28`) to the object's vtable slot 0x58.
pub fn ov_screen_session_set_highlight_color(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        let color = c.g[A2][0];
        frame.save_ra(c, 0);
        if color != 0 {
            c.cycles += 9;
            c.g[V1][0] = lw(c, p(c.g[A3][0], 8));
            c.g[A1][0] = 0x004d_0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x74c8));
            c.g[A0][0] = lh(c, p(c.g[V1][0], 0x58));
            c.g[V0][0] = lw(c, p(c.g[V1][0], 0x5c));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0020_9614)?;
            c.g[RA][0] = Addr(p(c.g[SP][0], 0)).ld(c);
        } else {
            c.cycles += 8;
            c.g[V1][0] = lw(c, p(c.g[A3][0], 8));
            c.g[A1][0] = 0x004d_0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x74d8));
            c.g[A0][0] = lh(c, p(c.g[V1][0], 0x58));
            c.g[V0][0] = lw(c, p(c.g[V1][0], 0x5c));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0020_9638)?;
            c.g[RA][0] = Addr(p(c.g[SP][0], 0)).ld(c);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenSession_SetUnlockedColor(Session* s, void* obj, int on)`
/// (0x209648): forward the unlocked colour (`D_004C8B58`) or the locked
/// colour (`D_004C8B48`) to the object's vtable slot 0x58.
pub fn ov_screen_session_set_unlocked_color(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        let color = c.g[A2][0];
        frame.save_ra(c, 0);
        if color != 0 {
            c.cycles += 9;
            c.g[V1][0] = lw(c, p(c.g[A3][0], 8));
            c.g[A1][0] = 0x004d_0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x74a8));
            c.g[A0][0] = lh(c, p(c.g[V1][0], 0x58));
            c.g[V0][0] = lw(c, p(c.g[V1][0], 0x5c));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0020_9674)?;
            c.g[RA][0] = Addr(p(c.g[SP][0], 0)).ld(c);
        } else {
            c.cycles += 8;
            c.g[V1][0] = lw(c, p(c.g[A3][0], 8));
            c.g[A1][0] = 0x004d_0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x74b8));
            c.g[A0][0] = lh(c, p(c.g[V1][0], 0x58));
            c.g[V0][0] = lw(c, p(c.g[V1][0], 0x5c));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0020_9698)?;
            c.g[RA][0] = Addr(p(c.g[SP][0], 0)).ld(c);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void SSXApp_StartGameModule(SSXApp* self)` (0x2281d8): allocate and
/// construct the game module, hand it to the app manager, and record the game
/// mode manager.
pub fn ssx_app_start_game_module(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x0048_0000;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x57f8));
        c.g[A0][0] = 0x22c;
        call_ptr(c, 0x0031_7d70, 0x0022_8200)?;
        c.g[A0][0] = c.g[V0][0];
        callee::GAME_MODULE_CTOR.call(c, 0x0022_8208)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        sw(c, p(c.g[S0][0], 0x84), c.g[V0][0]);
        call_ptr(c, 0x0031_6ef8, 0x0022_8218)?;
        call_ptr(c, 0x0023_7cf8, 0x0022_8220)?;
        sw(c, p(c.g[S0][0], 0xc0), c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Memory-card manager states.

/// `void MCManager_EndOperation(MCManager* self)` (0x23ea30): clear the
/// operation flags, notify the movie player and start loading.
pub fn mc_manager_end_operation(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw(c, p(c.g[S0][0], 0xbc));
        let idle = c.g[V0][0] == 0;
        sw(c, gp(c, -0x478), 0);
        if !idle {
            c.cycles += 2;
            sw(c, p(c.g[S0][0], 0xbc), 0);
            callee::MC_MANAGER_END_PROGRESS.call(c, 0x0023_ea54)?;
        }
        c.cycles += 14;
        c.g[V0][0] = 1;
        sw(c, p(c.g[S0][0], 0xc0), 0);
        sw(c, p(c.g[S0][0], 0x440), c.g[V0][0]);
        c.g[A1][0] = 1;
        c.g[V0][0] = lw(c, p(c.g[S0][0], 0x748));
        sw(c, p(c.g[S0][0], 0xc4), 0);
        c.g[A0][0] = lh(c, p(c.g[V0][0], 8));
        c.g[V1][0] = lw(c, p(c.g[V0][0], 0xc));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_ea7c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_Reset(MCManager* self)` (0x23fb58): reset the operation
/// state and put the manager back in idle mode.
pub fn mc_manager_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        sw(c, p(c.g[S0][0], 0x430), c.g[V0][0]);
        c.g[V0][0] = lw(c, p(c.g[S0][0], 0x748));
        sw(c, p(c.g[S0][0], 0x42c), 0);
        sw(c, p(c.g[S0][0], 0xbc), 0);
        sw(c, p(c.g[S0][0], 0xc0), 0);
        sw(c, p(c.g[S0][0], 0xc4), 0);
        c.g[A0][0] = lh(c, p(c.g[V0][0], 8));
        c.g[V1][0] = lw(c, p(c.g[V0][0], 0xc));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_fb98)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        callee::MC_MANAGER_SET_MODE.call(c, 0x0023_fba4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State1C(MCManager* self)` (0x240800): when the overlay
/// reports no busy overlay, run the state's completion hook and end the
/// operation.
pub fn mc_manager_state1c(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = lw(c, p(c.g[S0][0], 0x434));
        c.g[V1][0] = lw(c, p(c.g[A1][0], 0));
        c.g[A0][0] = lh(c, p(c.g[V1][0], 0x58));
        c.g[V0][0] = lw(c, p(c.g[V1][0], 0x5c));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0024_0828)?;
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            c.g[V0][0] = lw(c, p(c.g[S0][0], 0x6c));
            let target = c.g[V0][0] as u32;
            call_ptr(c, target, 0x0024_083c)?;
            let nothing = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !nothing {
                c.cycles += 2;
                c.g[A1][0] = 0;
                callee::MC_MANAGER_END_OPERATION.call(c, 0x0024_084c)?;
            }
            c.cycles += 1;
        }
        // The original reloads s0 on both paths, before the epilogue's block.
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State29(MCManager* self)` (0x240c30): when the overlay
/// manager reports work to do, notify the movie player and request the card
/// info for the pending slot.
pub fn mc_manager_state29(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw(c, p(c.g[S0][0], 0x6c));
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, 0x0024_0c4c)?;
        let nothing = c.g[V0][0] == 0;
        c.g[A1][0] = 1;
        if !nothing {
            c.cycles += 10;
            c.g[V0][0] = lw(c, p(c.g[S0][0], 0x748));
            c.g[A0][0] = lh(c, p(c.g[V0][0], 8));
            c.g[V1][0] = lw(c, p(c.g[V0][0], 0xc));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0024_0c68)?;
            c.g[A1][0] = lw(c, p(c.g[S0][0], 0x428));
            c.g[A0][0] = c.g[S0][0];
            callee::MC_MANAGER_REQUEST_CARD_INFO.call(c, 0x0024_0c74)?;
            c.g[V0][0] = 1;
            sw(c, p(c.g[S0][0], 0x338), c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_FinishLoad(MCManager* self)` (0x241240): tell the overlay
/// manager that the load finished, notify the movie player and run the state's
/// completion hook.
pub fn mc_manager_finish_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = lw(c, p(c.g[S0][0], 0x434));
        c.g[V0][0] = lw(c, p(c.g[A1][0], 0));
        c.g[A0][0] = lh(c, p(c.g[V0][0], 0x108));
        c.g[V1][0] = lw(c, p(c.g[V0][0], 0x10c));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0024_1268)?;
        c.g[V0][0] = lw(c, p(c.g[S0][0], 0x748));
        c.g[A1][0] = 0x24;
        c.g[A0][0] = lh(c, p(c.g[V0][0], 8));
        c.g[V1][0] = lw(c, p(c.g[V0][0], 0xc));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0024_1280)?;
        c.g[V0][0] = lw(c, p(c.g[S0][0], 0x2c));
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, 0x0024_128c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int MCManager_GetNumSlots(MCManager* self)` (0x241ac0): the cached slot
/// count from the overlay manager, clamped to 1 when it reports 2 or more.
pub fn mc_manager_get_num_slots(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw(c, p(c.g[S0][0], 0x424));
        if (c.g[V0][0] as i64) < 0 {
            c.cycles += 9;
            c.g[A1][0] = lw(c, p(c.g[S0][0], 0x434));
            c.g[V1][0] = lw(c, p(c.g[A1][0], 0));
            c.g[A0][0] = lh(c, p(c.g[V1][0], 0x228));
            c.g[V0][0] = lw(c, p(c.g[V1][0], 0x22c));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0024_1af4)?;
            c.g[V1][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
            let small = c.g[V1][0] != 0;
            sw(c, p(c.g[S0][0], 0x424), c.g[V0][0]);
            if !small {
                c.cycles += 2;
                c.g[V0][0] = 1;
                sw(c, p(c.g[S0][0], 0x424), c.g[V0][0]);
            }
            c.cycles += 2;
            c.g[V0][0] = lw(c, p(c.g[S0][0], 0x424));
        }
        // The original reloads s0 on both paths, before the epilogue's block.
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SSXApp_dtor` (0x2438f0): destroy the three embedded strings, then the app
/// base.
pub fn ssx_app_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xcc));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0024_3914)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc8));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0024_3920)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc4));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0024_392c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0031_6d28, 0x0024_3938)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Debug printf and movie/audio helpers.

/// `void MEM_DebugPrintf(const char* fmt, ...)` (0x251f68): format the
/// varargs into a 0x200-byte stack buffer and hand it to the debug output.
pub fn mem_debug_printf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x290);
        save_reg(c, frame, A1, 0x258);
        save_reg(c, frame, A2, 0x260);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0x200);
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x258));
        save_reg(c, frame, A3, 0x268);
        save_reg(c, frame, T0, 0x270);
        save_reg(c, frame, T1, 0x278);
        save_reg(c, frame, T2, 0x280);
        save_reg(c, frame, T3, 0x288);
        c.swc1(12, p(c.g[SP][0], 0x248));
        c.swc1(14, p(c.g[SP][0], 0x24c));
        c.swc1(16, p(c.g[SP][0], 0x250));
        c.swc1(18, p(c.g[SP][0], 0x254));
        call_ptr(c, 0x0041_86c8, 0x0025_1fac)?;
        c.g[A0][0] = 2;
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x003e_5d30, 0x0025_1fb8)?;
        frame.restore_ra(c, 0x200);
        frame.close(c);
        Ok(())
    })
}

/// `void VidEnginePlayer_Resume(VidEnginePlayer* self)` (0x253a40): resume
/// playback if the player has data, is enabled, is not paused and is stopped.
pub fn vid_engine_player_resume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = lw(c, p(c.g[S0][0], 0x38));
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = lw(c, p(c.g[S0][0], 0x3c));
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = lw(c, p(c.g[S0][0], 0));
                if c.g[V0][0] == 0 {
                    c.cycles += 3;
                    c.g[V0][0] = lw(c, p(c.g[S0][0], 8));
                    if c.g[V0][0] != 0 {
                        c.cycles += 4;
                        call_ptr(c, 0x003a_e958, 0x0025_3a88)?;
                        sw(c, p(c.g[S0][0], 8), 0);
                    }
                }
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void VD_TextureSink_Flush(VD_TextureSink* self)` (0x254dc0): spin until
/// the sink's device reports completion, then release the pending buffer.
pub fn vd_texture_sink_flush(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = lw(c, p(c.g[S0][0], 8));
        loop {
            c.cycles += 7;
            c.g[V1][0] = lw(c, p(c.g[A1][0], 0x10d8));
            c.g[A0][0] = lh(c, p(c.g[V1][0], 0x90));
            c.g[V0][0] = lw(c, p(c.g[V1][0], 0x94));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0025_4dec)?;
            if c.g[V0][0] != 0 {
                break;
            }
            c.g[A1][0] = lw(c, p(c.g[S0][0], 8));
            if event_due(c) {
                deliver_event_here(c, 0x0025_4dd8);
            }
        }
        c.cycles += 3;
        c.g[A1][0] = lw(c, p(c.g[S0][0], 0xc));
        if c.g[A1][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = lw(c, p(c.g[S0][0], 0));
            call_ptr(c, 0x003b_0680, 0x0025_4e08)?;
            sw(c, p(c.g[S0][0], 0xc), 0);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_MoveToRoomIndex(LobbyMan* self, int index, int a2)`
/// (0x25d800): move to the room with that index (or the default room name
/// when the index is -1).
pub fn lobby_man_move_to_room_index(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let default = c.g[A1][0] == c.g[V0][0];
        c.g[S1][0] = c.g[A2][0];
        if !default {
            c.cycles += 4;
            callee::LOBBY_MAN_GET_ROOM_NAME.call(c, 0x0025_d828)?;
            c.g[A0][0] = c.g[S0][0];
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x004a_0000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2f40));
            c.g[A0][0] = c.g[S0][0];
        }
        c.cycles += 8;
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        callee::LOBBY_MAN_MOVE_TO_ROOM.call(c, 0x0025_d848)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_PingUser(LobbyMan* self, int key, void* def)` (0x25dd08):
/// queue a ping message for the peer address of `key`.
pub fn lobby_man_ping_user(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, p(c.g[S0][0], 0));
        let no_net = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_net {
            c.cycles += 10;
            callee::LOBBY_MAN_LOOKUP_PEER_ADDR.call(c, 0x0025_dd30)?;
            c.g[T0][0] = 0x0027_0000;
            c.g[A0][0] = lw(c, p(c.g[S0][0], 0x54));
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x7278));
            c.g[T1][0] = c.g[S0][0];
            c.g[A3][0] = 0x1388;
            call_ptr(c, 0x003e_8dc0, 0x0025_dd50)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BuddyMan_ClearMessages(BuddyMan* self)` (0x261070): pop every queued
/// message (`self+0xac` begin, `self+0xb0` end) and call its vtable slot 8 with
/// argument 3.
///
/// The original is a `match pc` dispatcher whose loop head (0x261088) is also
/// a resume entry after an event unwind; the loop state is in the registers
/// (`s0` = self, `v0` = end pointer, `a2` = the popped item).
pub fn buddy_man_clear_messages(c: &mut Cpu) {
    guest(c, |c| {
        let frame;
        // `true`: start at the popped-message body (the resume entry); `false`:
        // start at the loop head.
        let mut body;
        if c.pc == 0x0026_1070 {
            c.cycles += 5;
            frame = Frame::open(c, 0x20);
            frame.save(c, S0, 0x10);
            frame.save_ra(c, 0);
            c.g[S0][0] = c.g[A0][0];
            body = false;
        } else if c.pc == 0x0026_1088 {
            frame = Frame { size: 0x20 };
            body = true;
        } else {
            let pc = c.pc;
            c.jump_out(pc, 0x0026_1070, 0x0026_10cc);
            return Ok(());
        }
        loop {
            if !body {
                c.cycles += 4;
                c.g[V1][0] = lw(c, p(c.g[S0][0], 0xac));
                c.g[V0][0] = lw(c, p(c.g[S0][0], 0xb0));
                if c.g[V1][0] == c.g[V0][0] {
                    break;
                }
                c.g[A2][0] = lw(c, (c.g[V0][0] as u32).wrapping_sub(4));
                if back_edge_unwinds(c, 0x0026_1088) {
                    return Ok(());
                }
            }
            body = false;
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            let none = c.g[A2][0] == 0;
            sw(c, p(c.g[S0][0], 0xb0), c.g[V0][0]);
            if !none {
                c.cycles += 6;
                c.g[V0][0] = lw(c, p(c.g[A2][0], 0xc));
                c.g[A1][0] = 3;
                c.g[A0][0] = lh(c, p(c.g[V0][0], 8));
                c.g[V1][0] = lw(c, p(c.g[V0][0], 0xc));
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0026_10ac)?;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Net event clone constructors.

/// The three clone constructors: allocate 0xc bytes from pool `+0x488`,
/// store the event code, the class vtable and the copied sequence number.
macro_rules! event_clone {
    ($fname:ident, $doc:expr, $ret:expr, $word:expr, $vt:expr) => {
        #[doc = $doc]
        pub fn $fname(c: &mut Cpu) {
            guest(c, |c| {
                c.cycles += 23;
                let frame = Frame::open(c, 0x20);
                c.g[A1][0] = 0x0048_0000;
                frame.save(c, S0, 0x10);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                c.g[S0][0] = c.g[A0][0];
                frame.save_ra(c, 0);
                c.g[A0][0] = 0xc;
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, $ret)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = lw(c, p(c.g[S0][0], 8));
                c.g[V0][0] = $word;
                c.g[V1][0] = $vt;
                sw(c, (c.g[A0][0] as u32), c.g[V0][0]);
                frame.restore(c, S0, 0x10);
                c.g[V0][0] = c.g[A0][0];
                frame.restore_ra(c, 0);
                sw(c, p(c.g[A0][0], 4), c.g[V1][0]);
                sw(c, p(c.g[A0][0], 8), c.g[A1][0]);
                frame.close(c);
                Ok(())
            })
        }
    };
}

event_clone!(
    net_event102_clone,
    "`NetEvent102_Clone(NetEvent102* self)` (0x267ec8): copy the event code 0x102.",
    0x0026_7ef0,
    0x102,
    0x0048_1290
);
event_clone!(
    net_event102_base_clone,
    "`NetEvent102Base_Clone(NetEvent* self)` (0x267f58): copy the base event 0x102.",
    0x0026_7f80,
    0x102,
    0x0048_1290
);
event_clone!(
    net_error_event_clone,
    "`NetErrorEvent_Clone(NetErrorEvent* self)` (0x267fe8): copy the error event 0x116.",
    0x0026_8010,
    0x116,
    0x0048_1250
);

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, vtable, FAKE};
    use crate::difftest::{check, Rng, DATA, GAME_GP};

    /// The main heap's struct and a 1 MiB arena (below the test data area and
    /// clear of the fake-call log at 0x900000).
    const MEM_HEAP: u32 = 0x004f_f250;
    const ARENA: u32 = 0x0010_0000;
    const ARENA_SIZE: u32 = 0x0010_0000;

    /// The `cBXString` static-empty-string marker (the value the string dtor
    /// compares against before touching the heap).
    const EMPTY_STRING: u32 = 0x0048_dcd4;

    /// Runs a generated function to completion as a setup step.
    fn run(c: &mut Cpu, addr: u32, f: fn(&mut Cpu), args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(
            c.pc,
            crate::difftest::RA,
            "setup call {addr:#x} did not return"
        );
        c.g[V0][0]
    }

    /// A fresh dlmalloc arena for the main heap.
    fn heap_init(c: &mut Cpu) {
        run(
            c,
            0x0031_ff60,
            ssx3_ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, ARENA as u64, ARENA_SIZE as u64],
        );
    }

    /// Allocate `size` bytes and zero them.
    fn alloc(c: &mut Cpu, size: u32) -> u32 {
        let p = run(
            c,
            0x0032_0058,
            ssx3_ee::s10::MEM_HeapMemalign,
            &[MEM_HEAP as u64, 16, size as u64],
        ) as u32;
        for i in (0..size).step_by(4) {
            c.bus.write32(p + i, 0);
        }
        p
    }

    fn put_str(c: &mut Cpu, at: u32, s: &str) {
        for (i, b) in s.bytes().enumerate() {
            c.bus.write8(at + i as u32, b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    fn put_wide(c: &mut Cpu, at: u32, s: &str) {
        for (i, ch) in s.chars().enumerate() {
            c.bus.write16(at + 2 * i as u32, ch as u16);
        }
        c.bus.write16(at + 2 * s.chars().count() as u32, 0);
    }

    /// Scrambled registers, the fake hooks, a game-like `gp`, and the BELibrary
    /// singleton replaced by a zeroed dummy (so `cBE_getBE` takes its cheap
    /// path).
    fn setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = machine(c, seed, salt, 0);
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x1000);
        c.bus.write32(GAME_GP.wrapping_sub(0x1efc), 0);
        r
    }

    /// The BELibrary singleton with interface 0's vtable slot 8 pointing at a
    /// fake method.
    fn belibrary(c: &mut Cpu, r: &mut Rng) {
        let vt = vtable(c, r, &[8]);
        c.bus.write32(0x0053_08a8, DATA + 0x1100);
        c.bus.write32(DATA + 0x1104, vt);
    }

    /// One test per reward-gallery destructor.
    macro_rules! dtor_test {
        ($tname:ident, $addr:expr, $gen:expr, $hand:expr) => {
            #[test]
            fn $tname() {
                check($gen, $hand, 100, &|c, seed| {
                    let _r = setup(c, seed, $addr);
                    c.g[A0][0] = DATA as u64;
                    c.g[A1][0] = 0;
                    c.bus.write32(DATA + 0x78, 0);
                    c.bus.write32(DATA + 0x8ec, 0xffff_ffff);
                });
            }
        };
    }

    // ---------------------------------------------------------------- FE core

    #[test]
    fn cfe_state_options_ctor_matches() {
        check(
            ssx3_ee::s02::f_001887a0__generated,
            cfe_state_options_ctor,
            160,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1887a0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = if seed % 3 == 0 {
                    0
                } else {
                    (r.next_u32() | 1) as u64
                };
                c.bus.write32(0x004a_4e84, DATA + 0x800);
            },
        );
    }

    #[test]
    fn fe_locale_unload_all_tables_matches() {
        check(
            ssx3_ee::s02::f_00195948__generated,
            fe_locale_unload_all_tables,
            80,
            &|c, seed| {
                let _r = setup(c, seed, 0x195948);
                heap_init(c);
                c.g[A0][0] = DATA as u64;
                // A couple of the 20 tables exist, so the frees run.
                for k in 0..3u32 {
                    let slot = ((seed + k * 7) % 20) as u32;
                    let block = alloc(c, 0x20);
                    c.bus.write32(DATA + 8 + 4 * slot, block);
                }
            },
        );
    }

    #[test]
    fn fe_locale_unload_game_tables_matches() {
        check(
            ssx3_ee::s02::f_00195e58__generated,
            fe_locale_unload_game_tables,
            100,
            &|c, seed| {
                let _r = setup(c, seed, 0x195e58);
                heap_init(c);
                c.g[A0][0] = DATA as u64;
                c.bus.write8(DATA + 3, (seed % 2) as u8);
                for k in 0..2u32 {
                    let block = alloc(c, 0x20);
                    c.bus.write32(DATA + 8 + 4 * (0xc + (seed + k) % 4), block);
                }
            },
        );
    }

    #[test]
    fn fe_module_init_rider_previews_matches() {
        check(
            ssx3_ee::s03::f_001a03c0__generated,
            fe_module_init_rider_previews,
            100,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1a03c0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn fe_async_manager_find_free_slot_matches() {
        check(
            ssx3_ee::s03::f_001a3bf0__generated,
            fe_async_manager_find_free_slot,
            150,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1a3bf0);
                c.g[A0][0] = DATA as u64;
                // Slot index of the free slot; 5 leaves all five busy.
                let free = if seed % 4 == 0 { 5u32 } else { seed % 5 };
                for i in 0..5u32 {
                    let v = if i == free { 0 } else { r.next_u32() | 1 };
                    c.bus.write32(DATA + 0x114 + i * 0x11c, v);
                }
            },
        );
    }

    // ------------------------------------------------------- net FE handlers

    #[test]
    fn net_fe_state_show_buddy_action_popup_matches() {
        check(
            ssx3_ee::s03::f_001ab370__generated,
            net_fe_state_show_buddy_action_popup,
            40,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1ab370);
                heap_init(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                // An existing popup, so the getter does not construct one.
                c.bus.write32(this + 0x6a0, DATA + 0x400);
            },
        );
    }

    #[test]
    fn cfe_state_challenge_received_popup_ctor_matches() {
        check(
            ssx3_ee::s03::f_001afe08__generated,
            cfe_state_challenge_received_popup_ctor,
            100,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1afe08);
                heap_init(c);
                let this = DATA + 0x400;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = if seed % 3 == 0 {
                    0
                } else {
                    (DATA + 0x100) as u64
                };
                put_str(c, DATA + 0x100, "menu");
            },
        );
    }

    #[test]
    fn cfe_state_net_config_on_info_popup_closed_matches() {
        check(
            ssx3_ee::s03::f_001b5cf8__generated,
            cfe_state_net_config_on_info_popup_closed,
            150,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1b5cf8);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
                c.g[A2][0] = if seed % 2 == 0 { 0x16 } else { 0 };
                // The event object's id (unmapped: the base handler's default path).
                c.bus.write32(DATA + 0x318, 0);
                // The info popup object whose virtual "close info" runs.
                c.bus.write32(this + 0x6ec, DATA + 0x500);
                let vt = vtable(c, &mut r, &[0x40]);
                c.bus.write32(DATA + 0x508, vt);
            },
        );
    }

    #[test]
    fn cfe_state_create_step1_on_child_event_matches() {
        check(
            ssx3_ee::s03::f_001b9428__generated,
            cfe_state_create_step1_on_child_event,
            150,
            &|c, seed| {
                let _r = setup(c, seed, 0x1b9428);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
                c.g[A2][0] = (seed % 2) as u64;
                let id = [0u32, 0x81e, 0x82c][(seed % 3) as usize];
                c.bus.write32(DATA + 0x318, id);
                c.bus.write32(DATA + 0x69c, 0);
            },
        );
    }

    #[test]
    fn cfe_state_lobby_menu_on_child_event_matches() {
        check(
            ssx3_ee::s03::f_001bdde8__generated,
            cfe_state_lobby_menu_on_child_event,
            150,
            &|c, seed| {
                let _r = setup(c, seed, 0x1bdde8);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
                c.g[A2][0] = (seed % 2) as u64;
                let id = [0u32, 0x823, 0x82a][(seed % 3) as usize];
                c.bus.write32(DATA + 0x318, id);
                c.bus.write32(DATA + 0x69c, 0);
            },
        );
    }

    #[test]
    fn cfe_state_lobby_menu_hide_room_tabs_matches() {
        check(
            ssx3_ee::s03::f_001bdfd8__generated,
            cfe_state_lobby_menu_hide_room_tabs,
            150,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1bdfd8);
                let this = DATA;
                c.g[A0][0] = this as u64;
                for i in 0..4u32 {
                    let obj = DATA + 0x1000 + 0x100 * i;
                    let vt = vtable(c, &mut r, &[0x48]);
                    c.bus.write32(obj + 8, vt);
                    c.bus.write32(this + 0x730 + 4 * i, obj);
                }
            },
        );
    }

    #[test]
    fn cfe_state_online_main_menu_on_child_event_matches() {
        check(
            ssx3_ee::s03::f_001c02f8__generated,
            cfe_state_online_main_menu_on_child_event,
            150,
            &|c, seed| {
                let _r = setup(c, seed, 0x1c02f8);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
                c.g[A2][0] = (seed % 2) as u64;
                let id = [0u32, 0x82d, 0x846][(seed % 3) as usize];
                c.bus.write32(DATA + 0x318, id);
                c.bus.write32(DATA + 0x69c, 0);
            },
        );
    }

    #[test]
    fn cfe_state_net_quick_messages_on_child_event_matches() {
        check(
            ssx3_ee::s03::f_001c2b48__generated,
            cfe_state_net_quick_messages_on_child_event,
            150,
            &|c, seed| {
                let _r = setup(c, seed, 0x1c2b48);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
                c.g[A2][0] = (seed % 2) as u64;
                let id = [0u32, 0x83a, 0x83b][(seed % 3) as usize];
                c.bus.write32(DATA + 0x318, id);
                c.bus.write32(DATA + 0x69c, 0);
            },
        );
    }

    #[test]
    fn cui_state_buy_popup_reset_params_matches() {
        check(
            ssx3_ee::s04::f_001ca8a0__generated,
            cui_state_buy_popup_reset_params,
            150,
            &|c, seed| {
                let _r = setup(c, seed, 0x1ca8a0);
                c.g[A0][0] = DATA as u64;
                for i in 0..0xc {
                    c.bus.write32(DATA + 4 * i, 0x1111_1111);
                }
            },
        );
    }

    #[test]
    fn cfe_state_mountain_room_request_image_matches() {
        check(
            ssx3_ee::s04::f_001d3160__generated,
            cfe_state_mountain_room_request_image,
            120,
            &|c, seed| {
                let _r = setup(c, seed, 0x1d3160);
                let this = DATA;
                c.g[A0][0] = this as u64;
                // Both cheap exits on some seeds, the request path on the rest.
                c.bus
                    .write32(this + 0xbc, if seed % 4 == 0 { 0 } else { 0xffff_ffff });
                c.bus
                    .write32(this + 0xc0, if seed % 3 == 0 { 0 } else { DATA + 0x300 });
                c.bus.write32(this + 0xb0, 0);
                // The world, its rider-0 manager, and slot 0 of the bolt-file
                // table already holding the requested item: `FindSlotByName`
                // matches (strcmp of the two equal names) and returns slot 0,
                // so the callee takes the refcount path instead of the
                // free-slot/evict/load walk.
                c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x800);
                c.bus.write32(DATA + 0x87c, DATA + 0x1000);
                c.bus.write32(DATA + 0x304, DATA + 0x100);
                put_str(c, DATA + 0x100, "bolt");
                put_str(c, DATA + 0x2a74, "bolt");
            },
        );
    }

    #[test]
    fn mem_card_state_base_set_use_save_title_matches() {
        check(
            ssx3_ee::s04::f_001d9308__generated,
            mem_card_state_base_set_use_save_title,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1d9308);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = if seed % 3 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                // The overlay manager and its movie player.
                c.bus.write32(GAME_GP.wrapping_sub(0x47c), DATA + 0x700);
                c.bus.write32(DATA + 0x700 + 0x434, DATA + 0x900);
                let vt = vtable(c, &mut r, &[0x70]);
                c.bus.write32(DATA + 0x900, vt);
                c.bus.write32(DATA + 0x700 + 0xf8, r.next_u32());
            },
        );
    }

    #[test]
    fn mem_card_state_base_on_activate_matches() {
        check(
            ssx3_ee::s04::f_001d9818__generated,
            mem_card_state_base_on_activate,
            120,
            &|c, seed| {
                let _r = setup(c, seed, 0x1d9818);
                c.g[A0][0] = DATA as u64;
                // No UI engine: the by-hash lookup returns null and takes the
                // leave-the-widget-alone path.
                c.bus.write32(DATA + 0x40, 0);
            },
        );
    }

    #[test]
    fn cfe_state_lobby_menu_dtor_matches() {
        check(
            ssx3_ee::s04::f_001df7c8__generated,
            cfe_state_lobby_menu_dtor,
            120,
            &|c, seed| {
                let _r = setup(c, seed, 0x1df7c8);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 0;
                c.bus.write32(DATA + 0xb80, EMPTY_STRING);
                c.bus.write32(DATA + 0x76c, EMPTY_STRING);
            },
        );
    }

    dtor_test!(
        cfe_state_reward_gallery_dtor_matches,
        0x1dfa10,
        ssx3_ee::s04::f_001dfa10__generated,
        cfe_state_reward_gallery_dtor
    );
    dtor_test!(
        cfe_state_reward_gallery_posters_dtor_matches,
        0x1dfb28,
        ssx3_ee::s04::f_001dfb28__generated,
        cfe_state_reward_gallery_posters_dtor
    );
    dtor_test!(
        cfe_state_reward_gallery_trading_cards_dtor_matches,
        0x1dfbf8,
        ssx3_ee::s04::f_001dfbf8__generated,
        cfe_state_reward_gallery_trading_cards_dtor
    );
    dtor_test!(
        cfe_state_reward_gallery_concept_arts_dtor_matches,
        0x1dfcc8,
        ssx3_ee::s04::f_001dfcc8__generated,
        cfe_state_reward_gallery_concept_arts_dtor
    );
    dtor_test!(
        cfe_state_reward_gallery_videos_dtor_matches,
        0x1dfd98,
        ssx3_ee::s04::f_001dfd98__generated,
        cfe_state_reward_gallery_videos_dtor
    );
    dtor_test!(
        cfe_state_reward_gallery_toys_dtor_matches,
        0x1dfe68,
        ssx3_ee::s04::f_001dfe68__generated,
        cfe_state_reward_gallery_toys_dtor
    );
    dtor_test!(
        cfe_state_reward_gallery_cheat_chars_dtor_matches,
        0x1dff38,
        ssx3_ee::s04::f_001dff38__generated,
        cfe_state_reward_gallery_cheat_chars_dtor
    );

    #[test]
    fn cfe_popup_select_multiplayer_mode_init_matches() {
        check(
            ssx3_ee::s04::f_001e02f8__generated,
            cfe_popup_select_multiplayer_mode_init,
            100,
            &|c, seed| {
                let _r = setup(c, seed, 0x1e02f8);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 0;
                c.bus.write32(DATA + 0x10, 0);
            },
        );
    }

    #[test]
    fn cfe_state_challenge_popup_dtor_matches() {
        check(
            ssx3_ee::s04::f_001e09a0__generated,
            cfe_state_challenge_popup_dtor,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1e09a0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 0;
                c.bus.write32(DATA + 0x20, DATA + 0x900);
                let vt = vtable(c, &mut r, &[0x100]);
                c.bus.write32(DATA + 0x908, vt);
            },
        );
    }

    #[test]
    fn message_center_find_message_matches() {
        check(
            ssx3_ee::s04::f_001e3388__generated,
            message_center_find_message,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1e3388);
                belibrary(c, &mut r);
                c.g[A0][0] = (r.next_u32() % 8) as u64;
            },
        );
    }

    #[test]
    fn message_center_reset_new_player_flag_matches() {
        check(
            ssx3_ee::s04::f_001e3510__generated,
            message_center_reset_new_player_flag,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1e3510);
                belibrary(c, &mut r);
                c.bus.write8(0x0053_5c11, (seed % 2) as u8);
            },
        );
    }

    #[test]
    fn hud_set_icon_text_matches() {
        check(
            ssx3_ee::s04::f_001e94e0__generated,
            hud_set_icon_text,
            100,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1e94e0);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = (DATA + 0x300) as u64;
                put_str(c, DATA + 0x100, if seed % 3 == 0 { "" } else { "AB" });
                c.bus.write32(DATA + 0x300, r.next_u32());
                c.bus.write32(DATA + 0x304, r.next_u32());
                c.bus.write32(this + 8, DATA + 0x500);
            },
        );
    }

    #[test]
    fn hud_set_icon_text_w_matches() {
        check(
            ssx3_ee::s04::f_001e9540__generated,
            hud_set_icon_text_w,
            100,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1e9540);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = (DATA + 0x300) as u64;
                put_wide(c, DATA + 0x100, if seed % 3 == 0 { "" } else { "AB" });
                c.bus.write32(DATA + 0x300, r.next_u32());
                c.bus.write32(DATA + 0x304, r.next_u32());
                c.bus.write32(this + 8, DATA + 0x500);
            },
        );
    }

    /// The session colour setters: an object whose vtable slot 0x58 takes the
    /// colour, and both the on (custom colour) and off (default colour) paths.
    fn color_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = setup(c, seed, salt);
        let obj = DATA + 0x900;
        c.g[A0][0] = DATA as u64;
        c.g[A1][0] = obj as u64;
        c.g[A2][0] = if seed % 2 == 0 {
            0
        } else {
            r.next_u32() as u64
        };
        let vt = vtable(c, &mut r, &[0x58]);
        c.bus.write32(obj + 8, vt);
    }

    #[test]
    fn ov_screen_session_set_highlight_color_matches() {
        check(
            ssx3_ee::s05::f_002095e8__generated,
            ov_screen_session_set_highlight_color,
            120,
            &|c, seed| color_setup(c, seed, 0x2095e8),
        );
    }

    #[test]
    fn ov_screen_session_set_unlocked_color_matches() {
        check(
            ssx3_ee::s05::f_00209648__generated,
            ov_screen_session_set_unlocked_color,
            120,
            &|c, seed| color_setup(c, seed, 0x209648),
        );
    }

    #[test]
    fn ssx_app_start_game_module_matches() {
        check(
            ssx3_ee::s05::f_002281d8__generated,
            ssx_app_start_game_module,
            40,
            &|c, seed| {
                let _r = setup(c, seed, 0x2281d8);
                heap_init(c);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    // -------------------------------------------------------- memory card

    #[test]
    fn mc_manager_end_operation_matches() {
        check(
            ssx3_ee::s06::f_0023ea30__generated,
            mc_manager_end_operation,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x23ea30);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.bus.write32(this + 0xbc, (seed % 2) as u32);
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(this + 0x748, vt);
            },
        );
    }

    #[test]
    fn mc_manager_reset_matches() {
        check(
            ssx3_ee::s06::f_0023fb58__generated,
            mc_manager_reset,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x23fb58);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(this + 0x748, vt);
            },
        );
    }

    #[test]
    fn mc_manager_state1c_matches() {
        check(
            ssx3_ee::s06::f_00240800__generated,
            mc_manager_state1c,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x240800);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.bus.write32(this + 0x434, DATA + 0x900);
                let vt = vtable(c, &mut r, &[0x58]);
                c.bus.write32(DATA + 0x900, vt);
                c.bus.write32(this + 0x6c, FAKE);
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(this + 0x748, vt);
            },
        );
    }

    #[test]
    fn mc_manager_state29_matches() {
        check(
            ssx3_ee::s06::f_00240c30__generated,
            mc_manager_state29,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x240c30);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.bus.write32(this + 0x6c, FAKE);
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(this + 0x748, vt);
                c.bus.write32(this + 0x428, r.next_u32());
                c.bus.write32(this + 0x434, DATA + 0x900);
                let vt = vtable(c, &mut r, &[0x28]);
                c.bus.write32(DATA + 0x900, vt);
            },
        );
    }

    #[test]
    fn mc_manager_finish_load_matches() {
        check(
            ssx3_ee::s06::f_00241240__generated,
            mc_manager_finish_load,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x241240);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.bus.write32(this + 0x434, DATA + 0x900);
                let vt = vtable(c, &mut r, &[0x108]);
                c.bus.write32(DATA + 0x900, vt);
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(this + 0x748, vt);
                c.bus.write32(this + 0x2c, FAKE);
            },
        );
    }

    #[test]
    fn mc_manager_get_num_slots_matches() {
        check(
            ssx3_ee::s06::f_00241ac0__generated,
            mc_manager_get_num_slots,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x241ac0);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.bus.write32(
                    this + 0x424,
                    match seed % 3 {
                        0 => 0,
                        1 => 0xffff_ffff,
                        _ => 1,
                    },
                );
                c.bus.write32(this + 0x434, DATA + 0x900);
                let vt = vtable(c, &mut r, &[0x228]);
                c.bus.write32(DATA + 0x900, vt);
            },
        );
    }

    #[test]
    fn ssx_app_dtor_matches() {
        check(ssx3_ee::s06::f_002438f0__generated, ssx_app_dtor, 120, &|c, seed| {
            let mut r = setup(c, seed, 0x2438f0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = 0;
            for off in [0xcc, 0xc8, 0xc4] {
                c.bus.write32(DATA + off, EMPTY_STRING);
            }
            c.bus.write32(GAME_GP.wrapping_add(0x2a74), 0);
            let _ = &mut r;
        });
    }

    // -------------------------------------------------- debug and movie

    #[test]
    fn mem_debug_printf_matches() {
        check(
            ssx3_ee::s06::f_00251f68__generated,
            mem_debug_printf,
            100,
            &|c, seed| {
                let mut r = setup(c, seed, 0x251f68);
                put_str(
                    c,
                    DATA + 0x100,
                    if seed % 3 == 0 { "hello" } else { "hi %d" },
                );
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = r.next_u32() as u64;
                c.g[T1][0] = r.next_u32() as u64;
                c.g[T2][0] = r.next_u32() as u64;
                c.g[T3][0] = r.next_u32() as u64;
                for i in 0..4 {
                    c.f[12 + i] = r.edge_f32();
                }
            },
        );
    }

    #[test]
    fn vid_engine_player_resume_matches() {
        check(
            ssx3_ee::s06::f_00253a40__generated,
            vid_engine_player_resume,
            150,
            &|c, seed| {
                let _r = setup(c, seed, 0x253a40);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let stage = seed % 5;
                c.bus.write32(this + 0x38, if stage == 0 { 0 } else { 1 });
                c.bus.write32(this + 0x3c, if stage <= 1 { 0 } else { 1 });
                c.bus.write32(this + 0x00, if stage <= 2 { 0 } else { 1 });
                c.bus.write32(this + 0x08, if stage <= 3 { 0 } else { 1 });
            },
        );
    }

    #[test]
    fn vd_texture_sink_flush_matches() {
        check(
            ssx3_ee::s06::f_00254dc0__generated,
            vd_texture_sink_flush,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x254dc0);
                let this = DATA;
                c.g[A0][0] = this as u64;
                // A device whose completion method is a fake (non-zero result, so
                // the spin loop is left at once).
                c.bus.write32(this + 8, DATA + 0x900);
                let vt = vtable(c, &mut r, &[0x90]);
                c.bus.write32(DATA + 0x900 + 0x10d8, vt);
                c.bus.write32(this + 0xc, if seed % 3 == 0 { 0 } else { 1 });
                c.bus.write32(this + 0, DATA + 0xa00);
                let vt = vtable(c, &mut r, &[0x30]);
                c.bus.write32(DATA + 0xa00 + 0x14, vt);
            },
        );
    }

    #[test]
    fn lobby_man_move_to_room_index_matches() {
        check(
            ssx3_ee::s06::f_0025d800__generated,
            lobby_man_move_to_room_index,
            150,
            &|c, seed| {
                let mut r = setup(c, seed, 0x25d800);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 3 == 0 {
                    0xffff_ffff_ffff_ffff
                } else {
                    (seed % 4) as u64
                };
                c.g[A2][0] = r.next_u32() as u64;
                // No net object: `MoveToRoom` returns immediately.
                c.bus.write32(DATA, 0);
            },
        );
    }

    #[test]
    fn lobby_man_ping_user_matches() {
        check(
            ssx3_ee::s06::f_0025dd08__generated,
            lobby_man_ping_user,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x25dd08);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.bus.write32(DATA, (seed % 2) as u32);
                c.bus.write32(DATA + 0x54, r.next_u32());
                c.bus.write32(DATA + 0xf4, 0);
                c.bus.write32(DATA + 0x8, r.next_u32());
            },
        );
    }

    #[test]
    fn buddy_man_clear_messages_matches() {
        check(
            ssx3_ee::s06::f_00261070__generated,
            buddy_man_clear_messages,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x261070);
                let this = DATA;
                c.pc = 0x0026_1070;
                c.g[A0][0] = this as u64;
                // A queue of 0..3 entries with some nulls.
                let n = (seed % 4) as u32;
                c.bus.write32(this + 0xac, DATA + 0x1000);
                c.bus.write32(this + 0xb0, DATA + 0x1000 + 4 * n);
                for i in 0..n {
                    if (seed >> (i + 2)) & 1 == 0 {
                        let obj = DATA + 0x1200 + 0x100 * i;
                        c.bus.write32(DATA + 0x1000 + 4 * i, obj);
                        let vt = vtable(c, &mut r, &[8]);
                        c.bus.write32(obj + 0xc, vt);
                    } else {
                        c.bus.write32(DATA + 0x1000 + 4 * i, 0);
                    }
                }
            },
        );
    }

    // ------------------------------------------------------ net event clones

    /// A cloned event: pool `+0x488` allocation, the self's sequence number.
    fn clone_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = setup(c, seed, salt);
        heap_init(c);
        c.g[A0][0] = DATA as u64;
        c.bus.write32(DATA + 8, r.next_u32());
    }

    #[test]
    fn net_event102_clone_matches() {
        check(
            ssx3_ee::s07::f_00267ec8__generated,
            net_event102_clone,
            120,
            &|c, seed| clone_setup(c, seed, 0x267ec8),
        );
    }

    #[test]
    fn net_event102_base_clone_matches() {
        check(
            ssx3_ee::s07::f_00267f58__generated,
            net_event102_base_clone,
            120,
            &|c, seed| clone_setup(c, seed, 0x267f58),
        );
    }

    #[test]
    fn net_error_event_clone_matches() {
        check(
            ssx3_ee::s07::f_00267fe8__generated,
            net_error_event_clone,
            120,
            &|c, seed| clone_setup(c, seed, 0x267fe8),
        );
    }
}
