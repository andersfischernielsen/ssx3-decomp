//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for cold-path front-end, audio, script and object helpers,
//! each with a differential test against the generated body.
//!
//! - **Lists / screens**: `UIList_AcceptVisitor`, `UIList_v03`, `UIState_ctor`
//!   (the widget list and the base state object the front end is built from).
//! - **Front-end states**: net-messenger and chat-room event handlers, the
//!   online main-menu handler, the popup base constructor and standings
//!   screens.
//! - **Setup / buddies**: the setup manager constructor and its brag message,
//!   `BuddyMan_HasMessageFrom`.
//! - **Audio**: the sound system's create/volume/channel/board-loop helpers.
//! - **Script engine**: bank loading, the finish-script test, script view and
//!   dummy-object field readers, the view-object reset.
//! - **AI / riders / camera**: rider order mode, render-slot queries, spline
//!   camera reset, quit request, best jump.
//! - **Misc**: memory card saves, input sync readiness, the DL heap arena
//!   setup, the BE event record index, lens flare sizes.
//!
//! The replacements keep the state in the guest registers the originals use
//! where callers, callees or event handlers can see them, follow the call
//! protocol of [`crate::call`] and the event rules of [`crate::events`].

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0039_79f8, ui_list_accept_visitor, "UIList_AcceptVisitor"),
    (0x0039_7a68, ui_list_v03, "UIList_v03"),
    (0x0034_8058, flex_rail_update_space_node, "FlexRail_UpdateSpaceNode"),
    (0x0035_45d8, flex_bridge_springs_save, "FlexBridge_SpringsSave"),
    (0x001a_3f68, c_fe_state_net_messenger_init, "cFEStateNetMessenger_Init"),
    (0x001a_4928, c_fe_state_net_messenger_v33, "cFEStateNetMessenger_v33"),
    (
        0x001b_3760,
        c_fe_state_chat_room_on_go_to_char_select_event,
        "cFEStateChatRoom_OnGoToCharSelectEvent",
    ),
    (
        0x001b_37d8,
        c_fe_state_chat_room_on_go_to_char_select_event_2,
        "cFEStateChatRoom_OnGoToCharSelectEvent_2",
    ),
    (0x001c_0358, c_fe_state_online_main_menu_v31, "cFEStateOnlineMainMenu_v31"),
    (0x001e_51d0, cov_state_view_popup_ctor, "cOVState_ViewPopup_ctor"),
    (
        0x001e_65b8,
        ov_screen_freestyle_standings_handle_input,
        "OVScreenFreestyleStandings_HandleInput",
    ),
    (
        0x001e_7558,
        ov_screen_final_standings_on_widget_created,
        "OVScreenFinalStandings_OnWidgetCreated",
    ),
    (0x0026_2a38, buddy_man_has_message_from, "BuddyMan_HasMessageFrom"),
    (0x0026_40c0, c_setup_man_ctor, "cSetupMan_ctor"),
    (0x0026_5688, c_setup_man_send_brag_msg10, "cSetupMan_SendBragMsg10"),
    (
        0x0029_29d8,
        ssx_audio_system_restart_all_board_loops,
        "SSXAudioSystem_RestartAllBoardLoops",
    ),
    (
        0x002a_1388,
        ssx_audio_system_rider_say_20b9_callback,
        "SSXAudioSystem_RiderSay20B9Callback",
    ),
    (0x0027_97c8, script_engine_load_script_bank, "ScriptEngine_LoadScriptBank"),
    (
        0x0027_aa80,
        script_engine_is_finish_script_active,
        "ScriptEngine_IsFinishScriptActive",
    ),
    (0x0027_c8e0, script_view_read_fields, "ScriptView_ReadFields"),
    (0x0027_cbb0, script_dummy_obj_read_fields, "ScriptDummyObj_ReadFields"),
    (0x0028_1a50, script_view_obj_reset, "ScriptViewObj_Reset"),
    (0x002a_d4d8, bank_monitor_stop_by_owner, "BankMonitor_StopByOwner"),
    (0x0012_8a48, ai_set_rider_order_mode, "AI_SetRiderOrderMode"),
    (
        0x0012_abd0,
        ai_computer_riders_have_render_slots,
        "AI_ComputerRidersHaveRenderSlots",
    ),
    (
        0x0012_acf8,
        ai_actor_riders_have_render_slots,
        "AI_ActorRidersHaveRenderSlots",
    ),
    (0x002c_68f0, cps2_mem_card_count_saves, "cPS2MemCard_CountSaves"),
    (
        0x0016_1630,
        spline_camera_algorithm_reset_for_rider,
        "SplineCameraAlgorithm_ResetForRider",
    ),
    (0x0024_2050, mc_manager_return_to_slot_state, "MCManager_ReturnToSlotState"),
    (0x0035_7090, parent_modifier_ctor_2, "ParentModifier_ctor_2"),
    (0x0017_e418, lobby_menu_send_chat, "LobbyMenu_SendChat"),
    (0x0024_fa00, height_fog_menu_dtor, "HeightFogMenu_dtor"),
    (0x001d_20d8, c_fe_state_preview_reward_set_item, "cFEStatePreviewReward_SetItem"),
    (0x0028_4bb0, ssx_audio_system_create, "SSXAudioSystem_Create"),
    (0x0028_7410, ssx_audio_system_set_music_volume, "SSXAudioSystem_SetMusicVolume"),
    (
        0x0028_7558,
        ssx_audio_system_set_music_channel_enabled,
        "SSXAudioSystem_SetMusicChannelEnabled",
    ),
    (
        0x0019_aca0,
        c_fe_state_char_equip_refresh_rider_bolts,
        "cFEStateCharEquip_RefreshRiderBolts",
    ),
    (0x0019_d3b0, bolt_file_find_free_slot, "BoltFile_FindFreeSlot"),
    (0x0023_7950, input_sync_is_ready, "InputSync_IsReady"),
    (0x0012_53d0, rider_mark_quit_requested, "Rider_MarkQuitRequested"),
    (0x0028_b650, snd_bank_free_memory, "SndBank_FreeMemory"),
    (
        0x0018_d7f0,
        c_fe_state_options_device_select_ctor,
        "cFEStateOptionsDeviceSelect_ctor",
    ),
    (0x001d_8d68, mem_card_state_base_show_status, "MemCardStateBase_ShowStatus"),
    (
        0x001f_e8f0,
        ov_screen_save_replay_on_child_event,
        "OVScreenSaveReplay_OnChildEvent",
    ),
    (
        0x0028_2d30,
        c_transition_man_script_advance_timer,
        "cTransitionMan_ScriptAdvanceTimer",
    ),
    (0x0011_9448, rider_metrix_record_best_jump, "RiderMetrix_RecordBestJump"),
    (0x002e_27e8, lens_fx_get_flare_size_for_res, "LensFX_GetFlareSizeForRes"),
    (0x0031_ff60, dl_heap_init_arena, "DLHeap_InitArena"),
    (0x0014_ab68, be_get_event_record_index, "BE_GetEventRecordIndex"),
    (0x0039_e2a0, ui_state_ctor, "UIState_ctor"),
];

// ---------------------------------------------------------------------------
// Callees (direct aliases the generator emitted as direct calls).

mod callee {
    use super::{ee, Callee};

    pub const BUILD_BUDDY_LIST: Callee =
        Callee { addr: 0x001a_5898, f: ee::s03::cFEStateNetMessenger_BuildBuddyList };
    pub const REFRESH_PLAYER_LIST: Callee =
        Callee { addr: 0x001a_6978, f: ee::s03::cFEStateNetMessenger_RefreshPlayerList };
    pub const SET_SELECTED_USER: Callee =
        Callee { addr: 0x001a_9890, f: ee::s03::NetFEState_SetSelectedUser };
    pub const GET_SELECTED_USER: Callee =
        Callee { addr: 0x001a_98b0, f: ee::s03::NetFEState_GetSelectedUser };
    pub const SHOW_BUDDY_ACTION_POPUP: Callee =
        Callee { addr: 0x001a_b370, f: ee::s03::NetFEState_ShowBuddyActionPopup };
    pub const ON_CHILD_EVENT: Callee =
        Callee { addr: 0x001a_97b8, f: ee::s03::NetFEState_OnChildEvent };
    pub const GOTO_NEXT_STATE: Callee =
        Callee { addr: 0x001a_8a40, f: ee::s03::NetFEState_GotoNextState };
    pub const HANDLE_NET_EVENT: Callee =
        Callee { addr: 0x001a_8e40, f: ee::s03::NetFEState_HandleNetEvent };
    pub const ONLINE_CHARACTER_SELECT_CTOR: Callee =
        Callee { addr: 0x001b_0f60, f: ee::s03::cFEStateOnlineCharacterSelect_ctor };
    pub const CHANNEL_GET_TRACK: Callee =
        Callee { addr: 0x0027_42c8, f: ee::s07::InSeq_ChannelGetTrack };
    pub const LIST_GET_ENTRY_TYPE: Callee =
        Callee { addr: 0x0027_9148, f: ee::s07::ScriptEngine_ListGetEntryType };
    pub const FIND_FREE_BANK_SLOT: Callee =
        Callee { addr: 0x0027_b528, f: ee::s07::ScriptEngine_FindFreeBankSlot };
    pub const LOAD_BANK_ASYNC: Callee =
        Callee { addr: 0x0027_b370, f: ee::s07::ScriptEngine_LoadBankAsync };
    pub const REQUEST_IMAGE: Callee =
        Callee { addr: 0x001d_2150, f: ee::s04::cFEStatePreviewReward_RequestImage };
    pub const SEND_PACKET: Callee = Callee { addr: 0x0026_4be8, f: ee::s07::cSetupMan_SendPacket };
    pub const BANK_MONITOR_RESET: Callee =
        Callee { addr: 0x002a_bb38, f: ee::s08::BankMonitorInstance_Reset };
    pub const RIDER_SAY_20B9: Callee =
        Callee { addr: 0x002a_1400, f: ee::s08::SSXAudioSystem_RiderSay20B9 };
    pub const STOP_BOARD_LOOPS: Callee =
        Callee { addr: 0x0029_2b48, f: ee::s07::SSXAudioSystem_StopBoardLoops };
    pub const START_RIDER_BOARD_LOOPS: Callee =
        Callee { addr: 0x0029_2a50, f: ee::s07::SSXAudioSystem_StartRiderBoardLoops };
    pub const GET_RACE_MAN: Callee = Callee { addr: 0x0028_b1d8, f: ee::s07::SSXAudioSystem_GetRaceMan };
    pub const LEVEL_TO_VOLUME: Callee =
        Callee { addr: 0x0028_73d8, f: ee::s07::SSXAudioSystem_LevelToVolume };
    pub const SET_CHANNEL_VOLUME: Callee =
        Callee { addr: 0x0028_7700, f: ee::s07::SSXAudioSystem_SetChannelVolume };
    pub const GET_CHANNEL_VOLUME: Callee =
        Callee { addr: 0x0028_7920, f: ee::s07::SSXAudioSystem_GetChannelVolume };
    pub const AUDIO_SYSTEM_CTOR: Callee =
        Callee { addr: 0x0028_4c68, f: ee::s07::SSXAudioSystem_ctor };
    pub const STOP_TRANSITION: Callee =
        Callee { addr: 0x0028_2db0, f: ee::s07::cTransitionMan_StopTransition };
    pub const REPLAY_END_SAVE: Callee = Callee { addr: 0x0027_0ed8, f: ee::s07::Replay_EndSave };
    pub const REPLAY_SET_CONTROLLER_INDEX: Callee =
        Callee { addr: 0x0026_f898, f: ee::s07::Replay_SetControllerIndex };
    pub const SND_BANK_UNLOAD: Callee = Callee { addr: 0x0028_b528, f: ee::s07::SndBank_Unload };
    pub const UI_STATE_CTOR_2: Callee = Callee { addr: 0x0039_e318, f: ee::s13::UIState_ctor_2 };
    pub const UI_LIST_CTOR: Callee = Callee { addr: 0x0039_77e8, f: ee::s13::UIList_ctor };
    pub const UI_MENU_SET_SELECTION: Callee =
        Callee { addr: 0x0039_b960, f: ee::s13::UIMenu_SetSelection };
    pub const MEM_CARD_BASE_CTOR: Callee =
        Callee { addr: 0x001d_53b0, f: ee::s04::MemCardStateBase_ctor };
    pub const MEM_CARD_ON_CHILD_EVENT: Callee =
        Callee { addr: 0x001d_8700, f: ee::s04::MemCardStateBase_OnChildEvent };
    pub const RIDER_HAS_RENDER_SLOT: Callee =
        Callee { addr: 0x0011_c0e0, f: ee::s00::Rider_HasRenderSlot };
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
mod addr {
    pub const STR_HASH: u32 = 0x0031_7670;
    pub const UI_ENGINE_CREATE_SCREEN: u32 = 0x0039_7cc0;
    pub const UI_SCREEN_START_THREAD: u32 = 0x0039_c870;
    pub const CLOSE_UNPINNED_POPUPS: u32 = 0x0039_f600;
    pub const MEM_MAN_ALLOC: u32 = 0x0031_7d70;
    pub const STRCASECMP: u32 = 0x0041_aa88;
    /// `strcmp`.
    pub const STRCMP: u32 = 0x0041_65a8;
    pub const STRLEN: u32 = 0x0041_6810;
    pub const SPRINTF: u32 = 0x0041_7828;
    pub const MEMSET: u32 = 0x003e_6448;
    pub const BE_GET_BE: u32 = 0x0014_dc80;
    pub const BE_GET_INTERFACE: u32 = 0x0014_dd58;
    pub const CHASE_CAMERA_RESET_FOR_RIDER: u32 = 0x0016_6c60;
    pub const MENU_POP_PAGE: u32 = 0x002c_c460;
    pub const MENU_PUSH_PAGE: u32 = 0x002c_c3b8;
    pub const FE_NET_SEND: u32 = 0x0025_aae0;
    pub const MENU_DTOR_2: u32 = 0x002c_a280;
    pub const MENU_DTOR: u32 = 0x002c_aa80;
    pub const WORLD_GET: u32 = 0x0022_7f80;
    pub const MC_STATUS_TEXT: u32 = 0x0024_1e18;
    pub const RIDER_PREVIEW: u32 = 0x001a_0548;
    pub const RIDER_PREVIEW_BOLTS: u32 = 0x0019_fbe0;
    pub const BE_GET_INTERFACE_9: u32 = 0x0014_ad28;
    pub const HUD_SHOW_NUMBER_DELAYED: u32 = 0x0011_71a8;
    pub const SPATIAL_INSERT: u32 = 0x0032_91e0;
    pub const POSITION_MODIFIER_CTOR_3: u32 = 0x0035_6f68;
    pub const WORLD_PAINTER_2D: u32 = 0x002d_1be0;
    pub const UI_OBJECT_BY_HASH: u32 = 0x0039_d860;
    pub const BANK_MONITOR_ADD: u32 = 0x002a_dbf0;
    pub const UI_ENGINE_TEARDOWN: u32 = 0x0025_23a8;
}

// ---------------------------------------------------------------------------
// Register helpers.

/// A `gp`-relative global address.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `lw` at `reg + off`.
#[inline(always)]
fn lw_at(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(r32(c, reg).wrapping_add(off)).lw(c)
}

/// `sw` at `reg + off`.
#[inline(always)]
fn sw_at(c: &mut Cpu, reg: usize, off: u32, v: u64) {
    let a = r32(c, reg).wrapping_add(off);
    c.sw(a, v);
}

/// `sx(x as u32)` of a register's low word.
#[inline(always)]
fn low(c: &Cpu, reg: usize) -> u64 {
    sx(r32(c, reg))
}

// ---------------------------------------------------------------------------
// Lists and screens.

/// `UIList_AcceptVisitor` (0x3979F8): walk the list at `self->head` (self+4)
/// backwards calling each node's visitor (`node+0x10` delta, `node+0x14`
/// function, `this = node + delta`) with `arg`; return the first non-zero
/// result, else 0.
pub fn ui_list_accept_visitor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr(r32(c, A0).wrapping_add(0x4)).lw(c);
        c.g[V0][0] = Addr(r32(c, A2).wrapping_add(0x4)).lw(c);
        let at_head = c.g[V0][0] == c.g[A2][0];
        c.g[S1][0] = c.g[A1][0];
        let mut visited = false;
        if !at_head {
            c.cycles += 2;
            c.g[V1][0] = Addr(r32(c, A2).wrapping_add(0x8)).lw(c);
            loop {
                c.cycles += 8;
                c.g[A1][0] = c.g[S1][0];
                c.g[S0][0] = Addr(r32(c, A2).wrapping_add(0x4)).lw(c);
                c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x10)).lh(c);
                let visitor = Addr(r32(c, V1).wrapping_add(0x14)).lw(c);
                c.g[V0][0] = visitor;
                c.g[A0][0] = abi::addu(c.g[A2][0], c.g[A0][0]);
                call_ptr(c, visitor as u32, 0x0039_7a38)?;
                if c.g[V0][0] != 0 {
                    visited = true;
                    frame.restore(c, S0, 0x20);
                    break;
                }
                c.cycles += 4;
                c.g[A2][0] = c.g[S0][0];
                c.g[V0][0] = Addr(r32(c, A2).wrapping_add(0x4)).lw(c);
                if c.g[V0][0] == c.g[A2][0] {
                    break;
                }
                c.g[V1][0] = Addr(r32(c, A2).wrapping_add(0x8)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0039_7a20);
                }
            }
        }
        if !visited {
            c.cycles += 2;
            c.g[V0][0] = 0;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `UIList_v03` (0x397A68): the same backwards walk, using each node's
/// second visitor (`node+0x18` delta, `node+0x1c` function).
pub fn ui_list_v03(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr(r32(c, A0).wrapping_add(0x4)).lw(c);
        c.g[V0][0] = Addr(r32(c, A2).wrapping_add(0x4)).lw(c);
        let at_head = c.g[V0][0] == c.g[A2][0];
        c.g[S1][0] = c.g[A1][0];
        let mut visited = false;
        if !at_head {
            c.cycles += 2;
            c.g[V1][0] = Addr(r32(c, A2).wrapping_add(0x8)).lw(c);
            loop {
                c.cycles += 8;
                c.g[A1][0] = c.g[S1][0];
                c.g[S0][0] = Addr(r32(c, A2).wrapping_add(0x4)).lw(c);
                c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x18)).lh(c);
                let visitor = Addr(r32(c, V1).wrapping_add(0x1c)).lw(c);
                c.g[V0][0] = visitor;
                c.g[A0][0] = abi::addu(c.g[A2][0], c.g[A0][0]);
                call_ptr(c, visitor as u32, 0x0039_7aa8)?;
                if c.g[V0][0] != 0 {
                    visited = true;
                    frame.restore(c, S0, 0x20);
                    break;
                }
                c.cycles += 4;
                c.g[A2][0] = c.g[S0][0];
                c.g[V0][0] = Addr(r32(c, A2).wrapping_add(0x4)).lw(c);
                if c.g[V0][0] == c.g[A2][0] {
                    break;
                }
                c.g[V1][0] = Addr(r32(c, A2).wrapping_add(0x8)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0039_7a90);
                }
            }
        }
        if !visited {
            c.cycles += 2;
            c.g[V0][0] = 0;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `FlexRail_UpdateSpaceNode` (0x348058): refresh a rail's space node: call
/// its `vt[50]` update, get the world painter, and hand the rail's box to the
/// spatial hash (`SpatialHash_Insert(world, 2, id, box, &old)`).
pub fn flex_rail_update_space_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[S1][0] = low(c, S0).wrapping_add(0x30);
        c.lq(2, r32(c, S0).wrapping_add(0x40));
        c.lq(3, r32(c, S0).wrapping_add(0x30));
        c.sq(2, frame.at(c, 0x10));
        c.sq(3, frame.at(c, 0));
        c.g[V1][0] = lw_at(c, S0, 0xc);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x190)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x194)).lw(c);
        let update = r32(c, V0);
        c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
        call_ptr(c, update, 0x0034_8094)?;
        call_ptr(c, addr::WORLD_PAINTER_2D, 0x0034_809c)?;
        c.g[A2][0] = lw_at(c, S0, 0x54);
        c.g[A3][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 2;
        c.g[T0][0] = c.g[SP][0];
        call_ptr(c, addr::SPATIAL_INSERT, 0x0034_80b4)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `FlexBridge_SpringsSave` (0x3545D8): serialize a bridge's springs through
/// the stream's writer slot (twice: the header at +0x24 and the spring array
/// at +0x24 * 0x50 bytes).
pub fn flex_bridge_springs_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0x24;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x8)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0xc)).lw(c);
        let write = r32(c, V0);
        c.g[A0][0] = abi::addu(c.g[S1][0], c.g[A0][0]);
        call_ptr(c, write, 0x0035_460c)?;
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        c.g[A2][0] = 0x50;
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        c.mult(0, ZERO, A0, A2);
        c.g[A1][0] = lw_at(c, S0, 0x24);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x8)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0xc)).lw(c);
        c.g[A2][0] = c.lo[0];
        let write = r32(c, V0);
        c.g[A0][0] = abi::addu(c.g[S1][0], c.g[A0][0]);
        call_ptr(c, write, 0x0035_4634)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end states.

/// `cFEStateNetMessenger_Init` (0x1A3F68): look up the net-messenger screen by
/// hash name in the UI engine, store it at +0x40, and when it exists play its
/// frame and build the buddy/player lists.
pub fn c_fe_state_net_messenger_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x46_0000;
        c.g[A0][0] = low(c, A0).wrapping_add(0x1448);
        c.g[S0][0] = lw_at(c, S1, 0x10);
        call_ptr(c, addr::STR_HASH, 0x001a_3f8c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, addr::UI_ENGINE_CREATE_SCREEN, 0x001a_3fa0)?;
        let screen = c.g[V0][0];
        let created = screen != 0;
        sw_at(c, S1, 0x40, screen);
        if created {
            c.cycles += 8;
            c.g[A0][0] = screen;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, addr::UI_SCREEN_START_THREAD, 0x001a_3fb8)?;
            c.g[A0][0] = c.g[S1][0];
            callee::BUILD_BUDDY_LIST.call(c, 0x001a_3fc0)?;
            c.g[A0][0] = c.g[S1][0];
            callee::REFRESH_PLAYER_LIST.call(c, 0x001a_3fc8)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateNetMessenger_v33` (0x1A4928): on a child event with key 0xF, open
/// the buddy action popup for the selected user if their name matches the
/// "me" name; every other event goes to the base child-event handler.
pub fn c_fe_state_net_messenger_v33(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xf;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let child_event = c.g[A2][0] != c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if child_event {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::ON_CHILD_EVENT.call(c, 0x001a_498c)?;
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 6;
            c.g[V0][0] = Addr(gp(c, -0xc8)).lw(c);
            c.g[A1][0] = lw_at(c, S0, 0x6d0);
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x80)).lw(c);
            call_ptr(c, addr::STRCASECMP, 0x001a_4950)?;
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 11;
                c.g[A1][0] = lw_at(c, S0, 0x6d0);
                c.g[A0][0] = c.g[S0][0];
                callee::SET_SELECTED_USER.call(c, 0x001a_4964)?;
                c.g[A0][0] = c.g[S0][0];
                callee::GET_SELECTED_USER.call(c, 0x001a_496c)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                c.g[A2][0] = 0;
                callee::SHOW_BUDDY_ACTION_POPUP.call(c, 0x001a_497c)?;
                frame.restore(c, S0, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateChatRoom_OnGoToCharSelectEvent` (0x1B3760): close unpinned popups,
/// allocate and construct the character-select screen, and switch to it.
pub fn c_fe_state_chat_room_on_go_to_char_select_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xf;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let child_event = c.g[A2][0] != c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if child_event {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::ON_CHILD_EVENT.call(c, 0x001b_37c4)?;
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 17;
            c.g[A0][0] = lw_at(c, S0, 0x10);
            c.g[A0][0] = low(c, A0).wrapping_add(0x18);
            call_ptr(c, addr::CLOSE_UNPINNED_POPUPS, 0x001b_3784)?;
            c.g[A1][0] = 0x46_0000;
            c.g[A3][0] = 0;
            c.g[A1][0] = low(c, A1).wrapping_add(0x1c60);
            c.g[A0][0] = 0x6e4;
            c.g[A2][0] = 0x100;
            call_ptr(c, addr::MEM_MAN_ALLOC, 0x001b_379c)?;
            c.g[A1][0] = lw_at(c, S0, 0x10);
            c.g[A0][0] = c.g[V0][0];
            callee::ONLINE_CHARACTER_SELECT_CTOR.call(c, 0x001b_37a8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            callee::GOTO_NEXT_STATE.call(c, 0x001b_37b4)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateChatRoom_OnGoToCharSelectEvent_2` (0x1B37D8): a second entry point
/// of the same chat-room transition (identical body).
pub fn c_fe_state_chat_room_on_go_to_char_select_event_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xf;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let child_event = c.g[A2][0] != c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if child_event {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::ON_CHILD_EVENT.call(c, 0x001b_383c)?;
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 17;
            c.g[A0][0] = lw_at(c, S0, 0x10);
            c.g[A0][0] = low(c, A0).wrapping_add(0x18);
            call_ptr(c, addr::CLOSE_UNPINNED_POPUPS, 0x001b_37fc)?;
            c.g[A1][0] = 0x46_0000;
            c.g[A3][0] = 0;
            c.g[A1][0] = low(c, A1).wrapping_add(0x1c60);
            c.g[A0][0] = 0x6e4;
            c.g[A2][0] = 0x100;
            call_ptr(c, addr::MEM_MAN_ALLOC, 0x001b_3814)?;
            c.g[A1][0] = lw_at(c, S0, 0x10);
            c.g[A0][0] = c.g[V0][0];
            callee::ONLINE_CHARACTER_SELECT_CTOR.call(c, 0x001b_3820)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            callee::GOTO_NEXT_STATE.call(c, 0x001b_382c)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateOnlineMainMenu_v31` (0x1C0358): the online main menu's event
/// filter: 0xEF is swallowed, 0xF3 opens the character select screen, and
/// everything else falls through to the net event handler. Returns 0.
pub fn c_fe_state_online_main_menu_v31(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xef;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let is_ef = c.g[A1][0] == c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        let mut handled = true;
        if !is_ef {
            c.cycles += 3;
            c.g[V0][0] = 0xf3;
            if c.g[A1][0] == c.g[V0][0] {
                c.g[A1][0] = 0x46_0000;
            } else {
                c.cycles += 4;
                callee::HANDLE_NET_EVENT.call(c, 0x001c_0384)?;
                handled = false;
            }
            if handled {
                c.cycles += 11;
                c.g[A0][0] = 0x6e4;
                c.g[A1][0] = low(c, A1).wrapping_add(0x1c60);
                c.g[A2][0] = 0x100;
                c.g[A3][0] = 0;
                call_ptr(c, addr::MEM_MAN_ALLOC, 0x001c_03a0)?;
                c.g[A1][0] = lw_at(c, S0, 0x10);
                c.g[A0][0] = c.g[V0][0];
                callee::ONLINE_CHARACTER_SELECT_CTOR.call(c, 0x001c_03ac)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                callee::GOTO_NEXT_STATE.call(c, 0x001c_03b8)?;
            }
        }
        if handled {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cOVState_ViewPopup_ctor` (0x1E51D0): the base constructor of the
/// full-screen overlay popup state; runs the state constructor and clears the
/// popup's id/box/flag fields.
pub fn cov_state_view_popup_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[T0][0];
        callee::UI_STATE_CTOR_2.call(c, 0x001e_51f4)?;
        c.g[V1][0] = 0x47_0000;
        sw_at(c, S0, 0x68, c.g[S1][0]);
        c.g[V1][0] = low(c, V1).wrapping_add(0x4248);
        sw_at(c, S0, 0x6c, c.g[S2][0]);
        sw_at(c, S0, 0x8, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        for off in [0x48u32, 0x4c, 0x50, 0x54, 0x5c, 0x60, 0x64, 0x58] {
            sw_at(c, S0, off, 0);
        }
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenFreestyleStandings_HandleInput` (0x1E65B8): if the scoreboard's
/// freestyle row is selected (byte +0x95 == 2), feed the pressed controller
/// index to the replay's controller setting. Returns 0.
pub fn ov_screen_freestyle_standings_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        c.g[A0][0] = 0x47_0000;
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = low(c, A0).wrapping_sub(0x1af0);
        call_ptr(c, addr::STR_HASH, 0x001e_65dc)?;
        c.g[A0][0] = lw_at(c, S0, 0x40);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, addr::UI_OBJECT_BY_HASH, 0x001e_65e8)?;
        let found = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x20);
        let mut skip_tail = false;
        if found {
            c.cycles += 4;
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x95)).lbu(c);
            c.g[V0][0] = 2;
            let not_selected = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = 0;
            if not_selected {
                skip_tail = true;
            } else {
                c.cycles += 6;
                c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
                c.g[A1][0] = Addr(r32(c, S1).wrapping_add(0x4)).lbu(c);
                c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x84)).lw(c);
                c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x28)).lw(c);
                callee::REPLAY_SET_CONTROLLER_INDEX.call(c, 0x001e_6614)?;
                frame.restore(c, S0, 0x20);
            }
        }
        if !skip_tail {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenFinalStandings_OnWidgetCreated` (0x1E7558): when the created
/// widget is the scoreboard's menu, set its selected row when the "final"
/// flag is on.
pub fn ov_screen_final_standings_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0x47_0000;
        frame.save(c, S1, 0x10);
        c.g[A0][0] = low(c, A0).wrapping_sub(0x1af0);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = lw_at(c, S1, 0x38);
        call_ptr(c, addr::STR_HASH, 0x001e_757c)?;
        let not_menu = c.g[S0][0] != c.g[V0][0];
        frame.restore(c, S0, 0x20);
        if !not_menu {
            c.cycles += 10;
            call_ptr(c, addr::BE_GET_BE, 0x001e_758c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, addr::BE_GET_INTERFACE, 0x001e_7598)?;
            c.g[V1][0] = 0x53_0000;
            c.g[A0][0] = 1;
            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x5c11)).lb(c);
            let not_final = c.g[V0][0] != c.g[A0][0];
            frame.restore(c, S0, 0x20);
            if !not_final {
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 1;
                callee::UI_MENU_SET_SELECTION.call(c, 0x001e_75b8)?;
                frame.restore(c, S0, 0x20);
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BuddyMan_HasMessageFrom` (0x262A38): true when any queued buddy message is
/// from `name` (compared with `strcmp`, 0x4165a8).
pub fn buddy_man_has_message_from(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[S0][0] = lw_at(c, S1, 0xac);
        c.g[V0][0] = lw_at(c, S1, 0xb0);
        let empty = c.g[S0][0] == c.g[V0][0];
        c.g[S2][0] = c.g[A1][0];
        let mut found = false;
        if !empty {
            c.cycles += 2;
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            loop {
                c.cycles += 5;
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                call_ptr(c, addr::STRCMP, 0x0026_2a74)?;
                let is_from = c.g[V0][0] != 0;
                if !is_from {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    found = true;
                    break;
                }
                c.g[V0][0] = lw_at(c, S1, 0xb0);
                c.cycles += 3;
                c.g[S0][0] = low(c, S0).wrapping_add(0x4);
                if c.g[S0][0] == c.g[V0][0] {
                    break;
                }
                c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0026_2a68);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = 0;
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

/// `cSetupMan_ctor` (0x2640C0): the setup manager: stores its two handler
/// callbacks and id, clears its queues and initialises both name buffers.
pub fn c_setup_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        sw_at(c, S0, 0x120, c.g[A1][0]);
        c.g[A0][0] = low(c, S0).wrapping_add(0x10);
        sw_at(c, S0, 0x124, c.g[A2][0]);
        c.g[A1][0] = 0;
        sw_at(c, S0, 0x4, 0);
        c.g[A2][0] = 0x80;
        for off in [0x118u32, 0x11c, 0x8, 0xc] {
            sw_at(c, S0, off, 0);
        }
        sw_at(c, S0, 0x128, c.g[A3][0]);
        sw_at(c, S0, 0, c.g[V0][0]);
        call_ptr(c, addr::MEMSET, 0x0026_4108)?;
        sw_at(c, S0, 0x90, 0);
        c.g[A0][0] = low(c, S0).wrapping_add(0x98);
        sw_at(c, S0, 0x94, 0);
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x80;
        call_ptr(c, addr::MEMSET, 0x0026_4120)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cSetupMan_SendBragMsg10` (0x265688): send the "brag" message, advancing the
/// setup manager's state (5..7 → 9, 8 → 10) and carrying the state in the
/// packet header.
pub fn c_setup_man_send_brag_msg10(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x30);
        frame.save_ra(c, 0x20);
        c.g[V0][0] = lw_at(c, A0, 0x124);
        let have = c.g[V0][0] != 0;
        c.g[A1][0] = 0xb;
        // The code the packet carries, and whether self[0] is reloaded last.
        let mut reload = true;
        if have {
            c.cycles += 4;
            c.g[V1][0] = Addr(r32(c, A0)).lw(c);
            c.g[V0][0] = ((c.g[V1][0] as i64) < 5) as u64;
            let below5 = c.g[V0][0] != 0;
            c.g[A1][0] = 0xa;
            if below5 {
                reload = false;
            } else {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V1][0] as i64) < 8) as u64;
                let below8 = c.g[V0][0] != 0;
                c.g[V0][0] = 9;
                if below8 {
                    c.cycles += 2;
                    c.sw(r32(c, A0), c.g[V0][0]);
                    c.g[A1][0] = 0xa;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 8;
                    let not8 = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0xa;
                    if not8 {
                        reload = false;
                    } else {
                        c.cycles += 2;
                        c.sw(r32(c, A0), c.g[V0][0]);
                        c.g[A1][0] = 0xa;
                    }
                }
            }
        }
        if reload {
            c.cycles += 1;
            c.g[V1][0] = Addr(r32(c, A0)).lw(c);
        }
        c.cycles += 11;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.sw(r32(c, SP), c.g[A1][0]);
        c.g[V0][0] |= 0xffff;
        sw_at(c, SP, 0x4, c.g[V1][0]);
        c.g[A1][0] = c.g[SP][0];
        sw_at(c, SP, 0x8, c.g[V0][0]);
        c.g[A2][0] = 0x14;
        callee::SEND_PACKET.call(c, 0x0026_56f0)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Audio.

/// `SSXAudioSystem_RestartAllBoardLoops` (0x2929D8): stop every board loop and
/// restart one per rider in the race manager, then clear the "loops running"
/// flag. Dispatcher-shaped: a delivered event resumes at 0x292A00.
pub fn ssx_audio_system_restart_all_board_loops(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x0029_29d8 => {
                    c.cycles += 9;
                    c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x30));
                    c.sq(17, sp(c).wrapping_add(0x10));
                    c.sq(16, sp(c).wrapping_add(0x20));
                    c.g[S1][0] = c.g[A0][0];
                    c.sd(sp(c), c.g[RA][0]);
                    c.g[S0][0] = 0;
                    callee::STOP_BOARD_LOOPS.call(c, 0x0029_29f4)?;
                    pc = 0x0029_2a20;
                }
                0x0029_29fc => {
                    c.cycles += 1;
                    pc = 0x0029_2a00;
                }
                0x0029_2a00 => {
                    c.cycles += 8;
                    callee::GET_RACE_MAN.call(c, 0x0029_2a08)?;
                    c.g[V1][0] = sx(r32(c, S0) << 2);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[V0][0] = low(c, V0).wrapping_add(low(c, V1));
                    c.g[S0][0] = low(c, S0).wrapping_add(1);
                    c.g[A1][0] = Addr(r32(c, V0).wrapping_add(0x28)).lw(c);
                    callee::START_RIDER_BOARD_LOOPS.call(c, 0x0029_2a20)?;
                    pc = 0x0029_2a20;
                }
                0x0029_2a20 => {
                    c.cycles += 6;
                    callee::GET_RACE_MAN.call(c, 0x0029_2a28)?;
                    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x78)).lw(c);
                    c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let more = c.g[V1][0] != 0;
                    c.g[RA][0] = Addr(sp(c)).ld(c);
                    if more {
                        if c.cycles >= c.next_event {
                            c.pc = 0x0029_2a00;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x0029_2a00;
                    } else {
                        pc = 0x0029_2a38;
                    }
                }
                0x0029_2a38 => {
                    c.cycles += 5;
                    sw_at(c, S1, 0x5830, 0);
                    c.lq(16, sp(c).wrapping_add(0x20));
                    c.lq(17, sp(c).wrapping_add(0x10));
                    let ret = c.g[RA][0] as u32;
                    c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x30));
                    c.pc = ret;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0029_29d8, 0x0029_2a4c);
                    return Ok(());
                }
            }
        }
    })
}

/// `SSXAudioSystem_RiderSay20B9Callback` (0x2A1388): a rider's "say" callback:
/// map the object's rider index to a race rider id and fire the line. Returns
/// 1.
pub fn ssx_audio_system_rider_say_20b9_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = lw_at(c, A1, 0x1c);
        let out_of_range = (c.g[S0][0] as i64) < 0;
        c.g[S1][0] = c.g[A0][0];
        let mut have_id = false;
        if !out_of_range {
            c.cycles += 6;
            callee::GET_RACE_MAN.call(c, 0x002a_13ac)?;
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x78)).lw(c);
            c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            if c.g[V1][0] != 0 {
                c.cycles += 5;
                callee::GET_RACE_MAN.call(c, 0x002a_13c4)?;
                c.g[V1][0] = sx(r32(c, S0) << 2);
                c.g[V0][0] = low(c, V0).wrapping_add(low(c, V1));
                have_id = true;
            }
        }
        if !have_id {
            c.cycles += 2;
            callee::GET_RACE_MAN.call(c, 0x002a_13d8)?;
        }
        c.cycles += 9;
        c.g[A1][0] = Addr(r32(c, V0).wrapping_add(0x28)).lw(c);
        c.g[A0][0] = c.g[S1][0];
        callee::RIDER_SAY_20B9.call(c, 0x002a_13e4)?;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem_Create` (0x284BB0): lazily allocate and construct the sound
/// system singleton, then tick its bank monitor. Returns the singleton.
pub fn ssx_audio_system_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = Addr(gp(c, 0x410)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let exists = c.g[V0][0] != 0;
        c.g[S0][0] = c.g[A0][0];
        if !exists {
            c.cycles += 14;
            c.g[A1][0] = 0x48_0000;
            c.g[A2][0] = 0xffff_ffff_8000_0000;
            c.g[A1][0] = low(c, A1).wrapping_add(0x28b0);
            c.g[A0][0] = 0x7780;
            c.g[A2][0] |= 0x400;
            c.g[A3][0] = 0;
            call_ptr(c, addr::MEM_MAN_ALLOC, 0x0028_4be4)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[T0][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            c.g[A2][0] = 0x1c;
            c.g[A3][0] = 3;
            callee::AUDIO_SYSTEM_CTOR.call(c, 0x0028_4bfc)?;
            sw_at(c, GP, 0x410, c.g[V0][0]);
        }
        c.cycles += 9;
        c.g[V0][0] = Addr(gp(c, 0x410)).lw(c);
        c.g[V1][0] = lw_at(c, V0, 0x118);
        c.g[A0][0] = Addr(r32(c, V1)).lw(c);
        c.g[A0][0] = low(c, A0).wrapping_add(0x1d8);
        call_ptr(c, addr::BANK_MONITOR_ADD, 0x0028_4c14)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem_SetMusicVolume` (0x287410): set the music volume on the
/// music channels (1, 8 and, when the music is not on the alternative channel,
/// 2).
pub fn ssx_audio_system_set_music_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_fpr(c, 20, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::LEVEL_TO_VOLUME.call(c, 0x0028_7428)?;
        c.mov_s(20, 0);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        c.mov_s(12, 20);
        callee::SET_CHANNEL_VOLUME.call(c, 0x0028_743c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 8;
        c.mov_s(12, 20);
        callee::SET_CHANNEL_VOLUME.call(c, 0x0028_744c)?;
        c.g[V0][0] = lw_at(c, S0, 0x62b8);
        let no_music = c.g[V0][0] == 0;
        c.g[V0][0] = 2;
        if !no_music {
            c.cycles += 3;
            c.g[V1][0] = lw_at(c, S0, 0x608c);
            let alts = c.g[V1][0] == c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            if !alts {
                c.cycles += 3;
                c.mov_s(12, 20);
                c.g[A1][0] = 2;
                callee::SET_CHANNEL_VOLUME.call(c, 0x0028_7470)?;
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem_SetMusicChannelEnabled` (0x287558): turn the second music
/// channel on (remembering the first channel's volume) or mute it.
pub fn ssx_audio_system_set_music_channel_enabled(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let on = c.g[A1][0] != 0;
        c.g[S0][0] = c.g[A0][0];
        let mut enable_path = false;
        if on {
            c.cycles += 4;
            c.g[V1][0] = lw_at(c, S0, 0x608c);
            c.g[V0][0] = 2;
            let is_alt = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 3;
            if !is_alt {
                c.cycles += 2;
                let is_alt2 = c.g[V1][0] == c.g[V0][0];
                if !is_alt2 {
                    c.cycles += 9;
                    c.g[A1][0] = 1;
                    callee::GET_CHANNEL_VOLUME.call(c, 0x0028_758c)?;
                    c.mov_s(12, 0);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 2;
                    callee::SET_CHANNEL_VOLUME.call(c, 0x0028_759c)?;
                    c.g[V0][0] = 1;
                    sw_at(c, S0, 0x62b8, c.g[V0][0]);
                    enable_path = true;
                }
            }
        }
        if !enable_path {
            c.cycles += 5;
            c.mtc1(0, 12);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 2;
            callee::SET_CHANNEL_VOLUME.call(c, 0x0028_75b8)?;
            sw_at(c, S0, 0x62b8, 0);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script engine.

/// `ScriptEngine_LoadScriptBank` (0x2797C8): format the bank name and load it
/// asynchronously into the first free script bank slot. Returns 1.
pub fn script_engine_load_script_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0xb0);
        c.g[A2][0] = 0x48_0000;
        frame.save(c, S1, 0x90);
        c.g[A2][0] = low(c, A2).wrapping_add(0x1c78);
        frame.save(c, S0, 0xa0);
        c.g[S1][0] = c.g[A1][0];
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = 0x48_0000;
        frame.save_ra(c, 0x80);
        c.g[A0][0] = c.g[SP][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[A1][0] = low(c, A1).wrapping_add(0x1c68);
        call_ptr(c, addr::SPRINTF, 0x0027_97fc)?;
        c.g[A0][0] = c.g[S0][0];
        callee::FIND_FREE_BANK_SLOT.call(c, 0x0027_9804)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[T0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[SP][0];
        c.g[A3][0] = 0x100;
        c.g[T1][0] = 0;
        c.g[T2][0] = 0;
        callee::LOAD_BANK_ASYNC.call(c, 0x0027_9824)?;
        frame.restore(c, S0, 0xa0);
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// `ScriptEngine_IsFinishScriptActive` (0x27AA80): true while the finish-line
/// script (entry type 0x12/0x0C/0x13/0x0D/0x14/0x0E) is the engine's current
/// script.
pub fn script_engine_is_finish_script_active(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 1;
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_at(c, A0, 0x550);
        let not_finish = c.g[V0][0] != c.g[V1][0];
        c.g[V0][0] = 0;
        if !not_finish {
            c.cycles += 7;
            c.g[A1][0] = 1;
            c.g[A2][0] = 0;
            callee::LIST_GET_ENTRY_TYPE.call(c, 0x0027_aaa4)?;
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = 0x12;
            c.g[A0][0] = 0;
            let mut hit = c.g[V1][0] == c.g[V0][0];
            if !hit {
                c.cycles += 3;
                c.g[V0][0] = 0xc;
                hit = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0x13;
                if !hit {
                    c.cycles += 2;
                    hit = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0xd;
                    if !hit {
                        c.cycles += 2;
                        hit = c.g[V1][0] == c.g[V0][0];
                        c.g[V0][0] = 0x14;
                        if !hit {
                            c.cycles += 2;
                            hit = c.g[V1][0] == c.g[V0][0];
                            c.g[V0][0] = 0xe;
                            if !hit {
                                c.cycles += 2;
                                let other = c.g[V1][0] != c.g[V0][0];
                                c.g[V0][0] = c.g[A0][0];
                                hit = !other;
                            }
                        }
                    }
                }
            }
            if hit {
                c.cycles += 2;
                c.g[A0][0] = 1;
                c.g[V0][0] = c.g[A0][0];
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ScriptView_ReadFields` (0x27C8E0): read a script view's start/end times
/// from its channel's track object into `out[0]`/`out[1]`.
pub fn script_view_read_fields(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        callee::CHANNEL_GET_TRACK.call(c, 0x0027_c900)?;
        c.g[V1][0] = lw_at(c, V0, 0x4);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x20)).lh(c);
        c.g[A1][0] = Addr(r32(c, V1).wrapping_add(0x24)).lw(c);
        let get = r32(c, A1);
        c.g[A0][0] = abi::addu(c.g[V0][0], c.g[A0][0]);
        call_ptr(c, get, 0x0027_c914)?;
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        sw_at(c, S1, 0, c.g[V1][0]);
        callee::CHANNEL_GET_TRACK.call(c, 0x0027_c928)?;
        c.g[V1][0] = lw_at(c, V0, 0x4);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x28)).lh(c);
        c.g[A1][0] = Addr(r32(c, V1).wrapping_add(0x2c)).lw(c);
        let get = r32(c, A1);
        c.g[A0][0] = abi::addu(c.g[V0][0], c.g[A0][0]);
        call_ptr(c, get, 0x0027_c93c)?;
        sw_at(c, S1, 0x4, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ScriptDummyObj_ReadFields` (0x27CBB0): the same start/end time read for a
/// dummy object (identical body).
pub fn script_dummy_obj_read_fields(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        callee::CHANNEL_GET_TRACK.call(c, 0x0027_cbd0)?;
        c.g[V1][0] = lw_at(c, V0, 0x4);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x20)).lh(c);
        c.g[A1][0] = Addr(r32(c, V1).wrapping_add(0x24)).lw(c);
        let get = r32(c, A1);
        c.g[A0][0] = abi::addu(c.g[V0][0], c.g[A0][0]);
        call_ptr(c, get, 0x0027_cbe4)?;
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        sw_at(c, S1, 0, c.g[V1][0]);
        callee::CHANNEL_GET_TRACK.call(c, 0x0027_cbf8)?;
        c.g[V1][0] = lw_at(c, V0, 0x4);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x28)).lh(c);
        c.g[A1][0] = Addr(r32(c, V1).wrapping_add(0x2c)).lw(c);
        let get = r32(c, A1);
        c.g[A0][0] = abi::addu(c.g[V0][0], c.g[A0][0]);
        call_ptr(c, get, 0x0027_cc0c)?;
        sw_at(c, S1, 0x4, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ScriptViewObj_Reset` (0x281A50): reset a script view object: clear both id
/// slots, set the "active" flag and an effectively infinite best distance,
/// store the zero vector + w=1 at +0x100, and clear the counters.
pub fn script_view_obj_reset(c: &mut Cpu) {
    c.cycles += 4;
    c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x10));
    c.g[A1][0] = 0xffff_ffff_ffff_ffff;
    c.g[V1][0] = 1;
    c.g[V0][0] = low(c, A0).wrapping_add(0xe0);
    loop {
        c.cycles += 7;
        c.sw(r32(c, V0), c.g[A1][0]);
        c.g[V1][0] = low(c, V1).wrapping_sub(1);
        let again = (c.g[V1][0] as i64) >= 0;
        c.g[V0][0] = low(c, V0).wrapping_sub(4);
        if again {
            if event_due(c) {
                deliver_event_here(c, 0x0028_1a60);
            }
            continue;
        }
        c.cycles += 18;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.g[V1][0] = 1;
        c.sw(r32(c, SP), 0);
        c.swc1(0, sp(c).wrapping_add(0xc));
        c.sw(sp(c).wrapping_add(0x4), 0);
        c.sw(sp(c).wrapping_add(0x8), 0);
        c.lwc1(0, gp(c, -0x4788));
        c.lq(2, sp(c));
        sw_at(c, A0, 0xf0, c.g[V1][0]);
        c.swc1(0, r32(c, A0).wrapping_add(0xf4));
        c.sq(2, r32(c, A0).wrapping_add(0x100));
        for off in [0x110u32, 0xe4, 0xe8, 0xec] {
            sw_at(c, A0, off, 0);
        }
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x10));
        c.pc = ret;
        return;
    }
}

// ---------------------------------------------------------------------------
// Sound banks, AI and camera.

/// `BankMonitor_StopByOwner` (0x2AD4D8): walk the 64 bank monitor instances and
/// reset the ones owned by `owner`.
pub fn bank_monitor_stop_by_owner(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = 0x3f;
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A0][0];
        c.g[V0][0] = lw_at(c, S3, 0x8ec);
        loop {
            c.cycles += 6;
            c.g[S0][0] = low(c, S0).wrapping_sub(1);
            c.g[V0][0] = low(c, S1).wrapping_add(low(c, V0));
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x80)).lw(c);
            c.g[A0][0] = c.g[V0][0];
            let other_owner = c.g[V1][0] != c.g[S2][0];
            c.g[S1][0] = low(c, S1).wrapping_add(0xc0);
            if !other_owner {
                c.cycles += 2;
                callee::BANK_MONITOR_RESET.call(c, 0x002a_d528)?;
            }
            c.cycles += 2;
            if (c.g[S0][0] as i64) >= 0 {
                c.g[V0][0] = lw_at(c, S3, 0x8ec);
                if event_due(c) {
                    deliver_event_here(c, 0x002a_d508);
                }
                continue;
            }
            c.cycles += 7;
            frame.restore(c, S0, 0x40);
            frame.restore(c, S1, 0x30);
            frame.restore(c, S2, 0x20);
            frame.restore(c, S3, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            break;
        }
        Ok(())
    })
}

/// `AI_SetRiderOrderMode` (0x128A48): store the ordering mode and number every
/// rider (or clear all their order fields when the mode is 0).
pub fn ai_set_rider_order_mode(c: &mut Cpu) {
    c.cycles += 2;
    let enabled = c.g[A1][0] != 0;
    sw_at(c, A0, 0x74, c.g[A1][0]);
    if enabled {
        c.cycles += 3;
        c.g[V0][0] = lw_at(c, A0, 0x78);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[V1][0] = 0;
        if empty {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        c.g[A1][0] = low(c, A0).wrapping_add(0x28);
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(r32(c, A1)).lw(c);
            sw_at(c, V0, 0xec, c.g[V1][0]);
            c.g[V1][0] = low(c, V1).wrapping_add(1);
            c.g[V0][0] = lw_at(c, A0, 0x78);
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = low(c, A1).wrapping_add(4);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0012_8a98);
                }
                continue;
            }
            c.cycles += 2;
            abi::ret(c);
            return;
        }
    }
    c.cycles += 3;
    c.g[V0][0] = lw_at(c, A0, 0x78);
    let empty = (c.g[V0][0] as i64) <= 0;
    c.g[A2][0] = 0;
    if empty {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    c.g[A1][0] = low(c, A0).wrapping_add(0x28);
    loop {
        c.cycles += 7;
        c.g[V1][0] = Addr(r32(c, A1)).lw(c);
        c.g[A2][0] = low(c, A2).wrapping_add(1);
        sw_at(c, V1, 0xec, 0);
        c.g[V0][0] = lw_at(c, A0, 0x78);
        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A1][0] = low(c, A1).wrapping_add(4);
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0012_8a60);
            }
            continue;
        }
        break;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `AI_ComputerRidersHaveRenderSlots` (0x12ABD0): true when every computer
/// rider in the rider list has a render slot.
pub fn ai_computer_riders_have_render_slots(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_at(c, S2, 0x80);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = 1;
        if !empty {
            c.cycles += 1;
            c.g[S0][0] = low(c, S2).wrapping_add(0x48);
            loop {
                c.cycles += 9;
                c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                c.g[S1][0] = low(c, S1).wrapping_add(1);
                c.g[S0][0] = low(c, S0).wrapping_add(4);
                c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x18)).lw(c);
                callee::RIDER_HAS_RENDER_SLOT.call(c, 0x0012_ac14)?;
                c.g[V1][0] = lw_at(c, S2, 0x80);
                c.g[V1][0] = ((c.g[S1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let more = c.g[V1][0] != 0;
                if c.g[V0][0] == 0 {
                    c.g[S3][0] = 0;
                }
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0012_ac00);
                }
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `AI_ActorRidersHaveRenderSlots` (0x12ACF8): the same query for the actor
/// rider list (count at +0x84, riders at +0x5c).
pub fn ai_actor_riders_have_render_slots(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_at(c, S2, 0x84);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = 1;
        if !empty {
            c.cycles += 1;
            c.g[S0][0] = low(c, S2).wrapping_add(0x5c);
            loop {
                c.cycles += 9;
                c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                c.g[S1][0] = low(c, S1).wrapping_add(1);
                c.g[S0][0] = low(c, S0).wrapping_add(4);
                c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x18)).lw(c);
                callee::RIDER_HAS_RENDER_SLOT.call(c, 0x0012_ad3c)?;
                c.g[V1][0] = lw_at(c, S2, 0x84);
                c.g[V1][0] = ((c.g[S1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let more = c.g[V1][0] != 0;
                if c.g[V0][0] == 0 {
                    c.g[S3][0] = 0;
                }
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0012_ad28);
                }
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cPS2MemCard_CountSaves` (0x2C68F0): count the memory card's save slots whose
/// name is non-empty.
pub fn cps2_mem_card_count_saves(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_at(c, S2, 0x14);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = 0;
        if !empty {
            c.cycles += 1;
            c.g[S1][0] = low(c, S2).wrapping_add(0x1b0);
            loop {
                c.cycles += 9;
                c.g[A0][0] = c.g[S1][0];
                c.g[S3][0] = low(c, S3).wrapping_add(1);
                call_ptr(c, addr::STRLEN, 0x002c_692c)?;
                c.g[S1][0] = low(c, S1).wrapping_add(0x78);
                c.g[A0][0] = lw_at(c, S2, 0x14);
                c.g[V1][0] = low(c, S0).wrapping_add(1);
                c.g[A0][0] = ((c.g[S3][0] as i64) < (c.g[A0][0] as i64)) as u64;
                let more = c.g[A0][0] != 0;
                if c.g[V0][0] != 0 {
                    c.g[S0][0] = c.g[V1][0];
                }
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002c_6920);
                }
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x30);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SplineCameraAlgorithm_ResetForRider` (0x161630): if the rider id matches the
/// camera's current spline entry, reset the camera to it.
pub fn spline_camera_algorithm_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = lw_at(c, S0, 0x30);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x4)).lw(c);
        c.g[V1][0] = Addr(r32(c, A0).wrapping_add(0x6c0)).lw(c);
        c.g[A0][0] = low(c, A0).wrapping_add(0x6c0);
        c.g[A1][0] = Addr(r32(c, V1).wrapping_add(0x38)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x3c)).lw(c);
        let get = r32(c, V0);
        c.g[A0][0] = abi::addu(c.g[A0][0], c.g[A1][0]);
        call_ptr(c, get, 0x0016_1668)?;
        if c.g[S1][0] == c.g[V0][0] {
            c.cycles += 9;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, addr::CHASE_CAMERA_RESET_FOR_RIDER, 0x0016_167c)?;
            c.g[V0][0] = lw_at(c, S0, 0x10);
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x28)).lh(c);
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x2c)).lw(c);
            let reset = r32(c, V1);
            c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
            call_ptr(c, reset, 0x0016_1690)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MCManager_ReturnToSlotState` (0x242050): tell the movie player's active
/// object to return to the memory card's slot state (2 or 3).
pub fn mc_manager_return_to_slot_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        sw_at(c, S0, 0x338, 0);
        c.g[A1][0] = lw_at(c, S0, 0x434);
        c.g[V1][0] = Addr(r32(c, A1)).lw(c);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x58)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x5c)).lw(c);
        let busy = r32(c, V0);
        c.g[A0][0] = abi::addu(c.g[A1][0], c.g[A0][0]);
        call_ptr(c, busy, 0x0024_207c)?;
        let returning = c.g[V0][0] != 0;
        c.g[V0][0] = lw_at(c, S0, 0x748);
        if returning {
            c.cycles += 6;
            c.g[A1][0] = 3;
        } else {
            c.cycles += 7;
            c.g[A1][0] = 2;
        }
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x8)).lh(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0xc)).lw(c);
        let set = r32(c, V1);
        c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
        call_ptr(c, set, if returning { 0x0024_20b4 } else { 0x0024_2098 })?;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ParentModifier_ctor_2` (0x357090): construct a parent modifier from its
/// stream: base position modifier, its vtable, the parent read and the parent
/// id stored at +0xa0.
pub fn parent_modifier_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, addr::POSITION_MODIFIER_CTOR_3, 0x0035_70ac)?;
        c.g[V0][0] = 0x49_0000;
        c.g[A1][0] = low(c, S1).wrapping_add(0x90);
        c.g[V0][0] = low(c, V0).wrapping_sub(0xaf8);
        c.g[A2][0] = 4;
        sw_at(c, S1, 0, c.g[V0][0]);
        c.g[V1][0] = Addr(r32(c, S0)).lw(c);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x10)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x14)).lw(c);
        let read = r32(c, V0);
        c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
        call_ptr(c, read, 0x0035_70d4)?;
        c.g[V1][0] = Addr(r32(c, S0)).lw(c);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x18)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x1c)).lw(c);
        let parent = r32(c, V0);
        c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
        call_ptr(c, parent, 0x0035_70e8)?;
        sw_at(c, S1, 0xa0, c.g[V0][0]);
        c.g[V0][0] = c.g[S1][0];
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
        c.g[V0][0] = lw_at(c, S1, 0x8);
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

/// `HeightFogMenu_dtor` (0x24FA00): destroy the height fog menu's five
/// sub-menus and then the menu base.
pub fn height_fog_menu_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        for (off, ret) in [
            (0x1c8u32, 0x0024_fa24u32),
            (0x194, 0x0024_fa30),
            (0x164, 0x0024_fa3c),
            (0x14c, 0x0024_fa48),
            (0x130, 0x0024_fa54),
        ] {
            c.g[A0][0] = low(c, S0).wrapping_add(off as u64);
            c.g[A1][0] = 2;
            call_ptr(c, addr::MENU_DTOR_2, ret)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, addr::MENU_DTOR, 0x0024_fa60)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStatePreviewReward_SetItem` (0x1D20D8): store the reward item kind and
/// data on the state and request the item's image for kinds 3, 4, 5 and 7.
pub fn c_fe_state_preview_reward_set_item(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 4;
        frame.save_ra(c, 0);
        sw_at(c, A0, 0x68, c.g[A1][0]);
        let is4 = c.g[A1][0] == c.g[V0][0];
        sw_at(c, A0, 0x60, c.g[A2][0]);
        // Where the switch ends: 4 = the 4-cycle request at 0x1d2130,
        // 3 = the 3-cycle request at 0x1d2140, 0 = no request.
        //   kind 4        -> 4
        //   kind 3        -> 4
        //   kind 5        -> 4
        //   kind 7        -> 3
        //   anything else -> none
        let mut request = 0u32;
        if is4 {
            request = 4;
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < 5) as u64;
            let at_least5 = c.g[V0][0] == 0;
            c.g[V0][0] = 3;
            if at_least5 {
                // `break 'b_1d210c` jumps past the inner block to the second
                // comparison chain (the 5/7 arm), which charges 3 more.
                c.cycles += 3;
                c.g[V0][0] = 5;
                let is5 = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = 7;
                if is5 {
                    request = 4;
                } else {
                    c.cycles += 2;
                    let is7 = c.g[A1][0] == c.g[V0][0];
                    c.g[RA][0] = Addr(sp(c)).ld(c);
                    if is7 {
                        request = 3;
                    } else {
                        c.cycles += 2;
                    }
                }
            } else {
                c.cycles += 2;
                let is3 = c.g[A1][0] == c.g[V0][0];
                c.g[RA][0] = Addr(sp(c)).ld(c);
                if is3 {
                    request = 4;
                } else {
                    c.cycles += 2;
                }
            }
        }
        if request == 4 {
            c.cycles += 4;
            c.g[A1][0] = lw_at(c, A2, 0x4);
            callee::REQUEST_IMAGE.call(c, 0x001d_2130)?;
            c.g[RA][0] = Addr(sp(c)).ld(c);
        } else if request == 3 {
            c.cycles += 3;
            c.g[A1][0] = lw_at(c, A2, 0x4);
            callee::REQUEST_IMAGE.call(c, 0x001d_2140)?;
            c.g[RA][0] = Addr(sp(c)).ld(c);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Riders, cameras, memory cards.

/// `cFEStateCharEquip_RefreshRiderBolts` (0x19ACA0): ask the char-equip state's
/// rider preview for the rider's currently equipped bolts.
pub fn c_fe_state_char_equip_refresh_rider_bolts(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x7c)).lw(c);
        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(0x44)).lb(c);
        c.g[A0][0] = low(c, A0).wrapping_add(0xb0);
        call_ptr(c, addr::RIDER_PREVIEW, 0x0019_acc8)?;
        c.g[S1][0] = c.g[V0][0];
        call_ptr(c, addr::BE_GET_BE, 0x0019_acd0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 9;
        call_ptr(c, addr::BE_GET_INTERFACE, 0x0019_acdc)?;
        c.g[A2][0] = lw_at(c, S0, 0xbc);
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(0x44)).lb(c);
        call_ptr(c, addr::BE_GET_INTERFACE_9, 0x0019_acec)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = 0;
        c.g[A3][0] = 1;
        call_ptr(c, addr::RIDER_PREVIEW_BOLTS, 0x0019_ad00)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BoltFile_FindFreeSlot` (0x19D3B0): find a free bolt slot in `bank`'s 256
/// entries, preferring an unused-but-valid slot with the smallest stamp;
/// returns -1 when none.
pub fn bolt_file_find_free_slot(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x1_0000;
    c.g[A3][0] = 0;
    c.g[V0][0] |= 0x2000;
    c.g[A2][0] = 0xffff_ffff_ffff_ffff;
    c.mult(0, V1, A1, V0);
    c.g[A1][0] = 0;
    c.g[A0][0] = low(c, V1).wrapping_add(low(c, A0));
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, A0)).lw(c);
        if c.g[V0][0] != 0 {
            c.g[A1][0] = low(c, A1).wrapping_add(1);
        } else {
            c.cycles += 3;
            c.g[V0][0] = lw_at(c, A0, 0x104);
            let free = c.g[V0][0] == 0;
            c.g[V0][0] = c.g[A1][0];
            if free {
                c.cycles += 2;
                c.g[V0][0] = c.g[A1][0];
                abi::ret(c);
                return;
            }
            c.cycles += 2;
            let no_best = (c.g[A2][0] as i64) < 0;
            c.g[V1][0] = lw_at(c, A0, 0x11c);
            let mut take_best = no_best;
            if !no_best {
                c.cycles += 3;
                c.g[V0][0] = (c.g[V1][0] < c.g[A3][0]) as u64;
                if c.g[V0][0] == 0 {
                    c.g[A1][0] = low(c, A1).wrapping_add(1);
                    c.cycles += 3;
                    c.g[A0][0] = low(c, A0).wrapping_add(0x120);
                    c.g[V0][0] = ((c.g[A1][0] as i64) < 256) as u64;
                    if c.g[V0][0] != 0 {
                        if event_due(c) {
                            deliver_event_here(c, 0x0019_d3d0);
                        }
                        continue;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = c.g[A2][0];
                    abi::ret(c);
                    return;
                }
                take_best = true;
            }
            if take_best {
                c.cycles += 3;
                c.g[A3][0] = c.g[V1][0];
                c.g[A2][0] = c.g[A1][0];
                c.g[A1][0] = low(c, A1).wrapping_add(1);
            }
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A1][0] as i64) < 256) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A0][0] = low(c, A0).wrapping_add(0x120);
        if !more {
            c.cycles += 2;
            c.g[V0][0] = c.g[A2][0];
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0019_d3d0);
        }
    }
}

/// `InputSync_IsReady` (0x237950): true when the input sync's frame counter is
/// past its startup delay, it is not waiting on a load, and every self slot's
/// cursors agree.
pub fn input_sync_is_ready(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = Addr(gp(c, -0x204)).lw(c);
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x70)).lw(c);
    c.g[V0][0] = ((c.g[V0][0] as i64) < 14) as u64;
    let early = c.g[V0][0] != 0;
    if early {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x3f8)).lw(c);
    let loading = c.g[V0][0] != 0;
    c.g[A1][0] = 0;
    let mut busy = loading;
    if !loading {
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x3fc)).lw(c);
        let idle = (c.g[V0][0] as i64) <= 0;
        if !idle {
            busy = true;
        }
    }
    if busy {
        c.cycles += 1;
        c.g[A1][0] = 1;
    }
    c.cycles += 2;
    if c.g[A1][0] != 0 {
        // A busy/loading input sync is "not ready": the generated body breaks
        // out of the slot walk to the shared `return 0`.
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.g[V0][0] = lw_at(c, A0, 0x10);
    c.cycles += 2;
    let none = (c.g[V0][0] as i64) <= 0;
    c.g[A1][0] = 0;
    if !none {
        c.cycles += 2;
        c.g[A2][0] = c.g[V0][0];
        c.g[A0][0] = low(c, A0).wrapping_add(0x14);
        loop {
            c.cycles += 4;
            c.g[V1][0] = lw_at(c, A0, 0x4);
            c.g[V0][0] = lw_at(c, A0, 0x8);
            let full = c.g[V1][0] == c.g[V0][0];
            c.g[A1][0] = low(c, A1).wrapping_add(1);
            if full {
                c.cycles += 2;
                c.g[V0][0] = 0;
                abi::ret(c);
                return;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A2][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = low(c, A0).wrapping_add(0x18);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0023_79a0);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `Rider_MarkQuitRequested` (0x1253D0): when the rider's "finished" timer is
/// negative, flag the rider as a did-not-finish and set its AI index entry.
pub fn rider_mark_quit_requested(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.mtc1(ZERO, 1);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.lwc1(0, r32(c, A0).wrapping_add(0x470));
        c.c_le_s(1, 0);
        let finished = c.fcc();
        c.g[V0][0] = 1;
        if !finished {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 2;
        let skip = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x10);
        if !skip {
            c.cycles += 14;
            c.g[S0][0] = 1;
            c.g[V1][0] = lw_at(c, A0, 0x6c0);
            sw_at(c, A0, 0x480, c.g[S0][0]);
            c.g[A1][0] = low(c, A0).wrapping_add(0x6c0);
            c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x38)).lh(c);
            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x3c)).lw(c);
            let index = r32(c, V0);
            c.g[A0][0] = abi::addu(c.g[A1][0], c.g[A0][0]);
            call_ptr(c, index, 0x0012_5420)?;
            c.g[V1][0] = 0x53_0000;
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V1][0] = low(c, V1).wrapping_add(0x66d0);
            c.g[V0][0] = low(c, V0).wrapping_add(low(c, V1));
            c.sw(r32(c, V0), c.g[S0][0]);
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SndBank_FreeMemory` (0x28B650): unload a sound bank and release its stream
/// and memory.
pub fn snd_bank_free_memory(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SND_BANK_UNLOAD.call(c, 0x0028_b664)?;
        c.g[V1][0] = lw_at(c, S0, 0x8);
        let free = c.g[V1][0] == 0;
        c.g[V0][0] = 2;
        if free {
            c.cycles += 4;
            frame.restore(c, S0, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 2;
        let streaming = c.g[V1][0] != c.g[V0][0];
        c.g[V1][0] = Addr(gp(c, 0x524)).lw(c);
        if !streaming {
            c.cycles += 10;
            c.g[A1][0] = lw_at(c, S0, 0xc);
            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x1d8)).lw(c);
            c.g[A3][0] = Addr(r32(c, V0)).lw(c);
            c.g[V1][0] = Addr(r32(c, A3)).lw(c);
            c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x28)).lh(c);
            c.g[A2][0] = Addr(r32(c, V1).wrapping_add(0x2c)).lw(c);
            let stop = r32(c, A2);
            c.g[A0][0] = abi::addu(c.g[A3][0], c.g[A0][0]);
            call_ptr(c, stop, 0x0028_b698)?;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            sw_at(c, S0, 0xc, c.g[V0][0]);
        }
        c.cycles += 5;
        c.g[A0][0] = lw_at(c, S0, 0x10);
        call_ptr(c, addr::UI_ENGINE_TEARDOWN, 0x0028_b6a8)?;
        sw_at(c, S0, 0x8, 0);
        sw_at(c, S0, 0x10, 0);
        sw_at(c, S0, 0x14, 0);
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateOptionsDeviceSelect_ctor` (0x18D7F0): construct the device-select
/// options state, remembering its two handlers and clearing the slot state.
pub fn c_fe_state_options_device_select_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[T0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A3][0];
        callee::MEM_CARD_BASE_CTOR.call(c, 0x0018_d814)?;
        c.g[V1][0] = 0x47_0000;
        c.g[V0][0] = 0x21;
        c.g[V1][0] = low(c, V1).wrapping_sub(0x3dd8);
        sw_at(c, S2, 0x230, c.g[S0][0]);
        sw_at(c, S2, 0x8, c.g[V1][0]);
        sw_at(c, S2, 0xc, c.g[V0][0]);
        sw_at(c, S2, 0x22c, c.g[S1][0]);
        sw_at(c, S2, 0x234, 0);
        let no_handler = c.g[S1][0] == 0;
        sw_at(c, S2, 0x238, 0);
        if no_handler {
            c.cycles += 3;
            c.g[A0][0] = Addr(gp(c, -0x848)).lw(c);
            call_ptr(c, addr::WORLD_GET, 0x0018_d844)?;
            sw_at(c, V0, 0x11c, 0);
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MemCardStateBase_ShowStatus` (0x1D8D68): show the memory card status text and
/// hand it to the state's UI object.
pub fn mem_card_state_base_show_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x660);
        frame.save(c, S0, 0x650);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x640);
        c.g[A0][0] = Addr(gp(c, -0x848)).lw(c);
        call_ptr(c, addr::WORLD_GET, 0x001d_8d80)?;
        c.g[V1][0] = 0x16;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[SP][0];
        sw_at(c, S0, 0x19c, c.g[V1][0]);
        c.g[A2][0] = 0x320;
        call_ptr(c, addr::MC_STATUS_TEXT, 0x001d_8d98)?;
        c.g[V0][0] = lw_at(c, S0, 0x8);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 1;
        c.g[A3][0] = 1;
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x120)).lh(c);
        c.g[T0][0] = 0;
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x124)).lw(c);
        c.g[T1][0] = 0;
        c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
        let show = r32(c, V1);
        c.g[T2][0] = 1;
        call_ptr(c, show, 0x001d_8dc4)?;
        c.g[V0][0] = 1;
        c.g[RA][0] = Addr(frame.at(c, 0x640)).ld(c);
        sw_at(c, S0, 0x210, c.g[V0][0]);
        frame.restore(c, S0, 0x650);
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x660));
        c.pc = ret;
        Ok(())
    })
}

/// `OVScreenSaveReplay_OnChildEvent` (0x1FE8F0): on the "save done" child event
/// of the replay save screen, stop the replay save and clear the pending flag;
/// everything else goes to the memory card state's child-event handler.
pub fn ov_screen_save_replay_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x16;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        let other_key = c.g[S1][0] != c.g[V0][0];
        c.g[S2][0] = c.g[A1][0];
        let mut tail = true;
        if !other_key {
            c.cycles += 4;
            c.g[V1][0] = lw_at(c, S0, 0x19c);
            c.g[V0][0] = 5;
            let not_five = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            if not_five {
                tail = false;
            } else {
                c.cycles += 5;
                c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
                c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x84)).lw(c);
                c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x28)).lw(c);
                callee::REPLAY_END_SAVE.call(c, 0x001f_e938)?;
                sw_at(c, S0, 0x1c8, 0);
            }
        }
        if tail {
            c.cycles += 1;
            c.g[A0][0] = c.g[S0][0];
        }
        c.cycles += 9;
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S1][0];
        callee::MEM_CARD_ON_CHILD_EVENT.call(c, 0x001f_e94c)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cTransitionMan_ScriptAdvanceTimer` (0x282D30): advance a script transition's
/// timer by the frame delta and stop the transition when the hold time has
/// elapsed.
pub fn c_transition_man_script_advance_timer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = lw_at(c, A0, 0x14);
        let no_timer = (c.g[A1][0] as i64) < 0;
        c.g[RA][0] = Addr(frame.at(c, 0)).ld(c);
        if !no_timer {
            c.cycles += 15;
            c.lwc1(1, r32(c, A0).wrapping_add(0x20));
            c.lwc1(0, r32(c, A0).wrapping_add(0x1c));
            c.lwc1(2, gp(c, -0x4780));
            c.add_s(0, 0, 1);
            c.lwc1(4, r32(c, A0).wrapping_add(0x24));
            c.mul_s(2, 12, 2);
            c.lwc1(1, r32(c, A0).wrapping_add(0x2c));
            c.lwc1(3, gp(c, -0x477c));
            c.add_s(0, 0, 4);
            c.add_s(1, 1, 2);
            c.sub_s(0, 0, 3);
            c.c_le_s(0, 1);
            // The stop arm is the one where `(timer <= target)` holds: the
            // generated code breaks out when the comparison is false.
            let stop = c.fcc();
            c.swc1(1, r32(c, A0).wrapping_add(0x2c));
            if stop {
                c.cycles += 7;
                c.mtc1(ZERO, 12);
                c.g[A2][0] = 9;
                c.mov_s(13, 12);
                c.mov_s(14, 12);
                c.mov_s(15, 12);
                callee::STOP_TRANSITION.call(c, 0x0028_2d98)?;
                c.g[RA][0] = Addr(frame.at(c, 0)).ld(c);
            }
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `RiderMetrix_RecordBestJump` (0x119448): count a jump; when the rider beat its
/// best jump height, show the number and remember it. Returns 0.0f.
pub fn rider_metrix_record_best_jump(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_fpr(c, 20, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.mov_s(20, 12);
        c.g[V0][0] = lw_at(c, S0, 0x130);
        c.g[A0][0] = lw_at(c, S0, 0x1b0);
        c.g[V0][0] = low(c, V0).wrapping_add(1);
        let no_hud = c.g[A0][0] == 0;
        sw_at(c, S0, 0x130, c.g[V0][0]);
        if !no_hud {
            c.cycles += 5;
            c.lwc1(0, r32(c, S0).wrapping_add(0x18));
            c.c_lt_s(0, 20);
            let not_better = !c.fcc();
            c.g[A0][0] = low(c, A0).wrapping_add(0x270);
            if !not_better {
                c.cycles += 7;
                c.cvt_w_s(0, 20);
                c.mfc1(6, 0);
                c.mtc1(ZERO, 12);
                c.g[A1][0] = 4;
                c.g[A3][0] = 0;
                call_ptr(c, addr::HUD_SHOW_NUMBER_DELAYED, 0x0011_94a0)?;
                c.swc1(20, r32(c, S0).wrapping_add(0x18));
            }
        }
        c.cycles += 6;
        c.mtc1(ZERO, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `LensFX_GetFlareSizeForRes` (0x2E27E8): the lens flare size for a render
/// resolution type (100/350/180, 0 for unknown).
pub fn lens_fx_get_flare_size_for_res(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x20;
    let is_32 = c.g[A0][0] == c.g[V0][0];
    c.g[V0][0] = ((c.g[A0][0] as i64) < 33) as u64;
    if is_32 {
        c.cycles += 4;
        c.g[AT][0] = 0x42c8_0000;
        c.mtc1(AT, 0);
        abi::ret(c);
        return;
    }
    // 0: the >= 33 arm (0x40 test), 1: 180 (A0 == 0x10), 2: zero.
    let mut arm = 2;
    c.cycles += 2;
    let ge33 = c.g[V0][0] == 0;
    c.g[V0][0] = 0x10;
    if ge33 {
        arm = 0;
    } else {
        c.cycles += 2;
        let is_16 = c.g[A0][0] == c.g[V0][0];
        if is_16 {
            arm = 1;
        } else {
            c.cycles += 2;
        }
    }
    if arm == 1 {
        c.cycles += 4;
        c.g[AT][0] = 0x4334_0000;
        c.mtc1(AT, 0);
        abi::ret(c);
        return;
    }
    if arm == 0 {
        c.cycles += 3;
        c.g[V0][0] = 0x40;
        let is_64 = c.g[A0][0] == c.g[V0][0];
        if is_64 {
            c.cycles += 4;
            c.g[AT][0] = 0x43af_0000;
            c.mtc1(AT, 0);
            abi::ret(c);
            return;
        }
        c.cycles += 2;
    }
    c.cycles += 3;
    c.mtc1(ZERO, 0);
    abi::ret(c);
}

/// `DLHeap_InitArena` (0x31FF60): carve a 16-byte aligned block out of an arena
/// and initialise the heap header in it.
pub fn dl_heap_init_arena(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        c.g[V1][0] = 0xffff_ffff_ffff_0000;
        c.g[V0][0] = low(c, A1).wrapping_add(0xf);
        c.g[V1][0] |= 0xfff0;
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x20));
        c.g[V0][0] &= c.g[V1][0];
        c.sq(16, sp(c).wrapping_add(0x10));
        c.g[A1][0] = low(c, V0).wrapping_sub(low(c, A1));
        c.g[S0][0] = c.g[A0][0];
        c.g[A2][0] = low(c, A2).wrapping_sub(low(c, A1));
        c.sd(sp(c), c.g[RA][0]);
        c.g[A2][0] &= c.g[V1][0];
        c.g[A0][0] = c.g[V0][0];
        sw_at(c, S0, 0xc, c.g[A2][0]);
        c.g[A1][0] = 0;
        sw_at(c, S0, 0x4, c.g[V0][0]);
        c.g[A2][0] = 0x378;
        sw_at(c, S0, 0, c.g[V0][0]);
        call_ptr(c, addr::MEMSET, 0x0031_ffa8)?;
        c.g[V1][0] = lw_at(c, S0, 0x4);
        c.g[V0][0] = lw_at(c, S0, 0xc);
        c.g[V1][0] = low(c, V1).wrapping_add(0x380);
        c.g[RA][0] = Addr(sp(c)).ld(c);
        c.g[V0][0] = low(c, V0).wrapping_sub(0x380);
        sw_at(c, S0, 0x8, c.g[V1][0]);
        sw_at(c, S0, 0xc, c.g[V0][0]);
        sw_at(c, S0, 0x4, c.g[V1][0]);
        c.lq(16, sp(c).wrapping_add(0x10));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x20));
        c.pc = ret;
        Ok(())
    })
}

/// `BE_GetEventRecordIndex` (0x14AB68): map a BE event id `b` (and the player
/// `a`) to a score-type index; 26 when there is no record.
pub fn be_get_event_record_index(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = ((c.g[A0][0] as i64) < 17) as u64;
    let known_player = c.g[V0][0] != 0;
    c.g[A2][0] = low(c, A1).wrapping_sub(6);
    if !known_player {
        c.cycles += 2;
        c.g[V0][0] = 0x1a;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = (c.g[A2][0] < 3) as u64;
    let low_id = c.g[V0][0] != 0;
    c.g[V0][0] = low(c, A1).wrapping_sub(9);
    let mut table = false;
    if !low_id {
        c.cycles += 3;
        c.g[V0][0] = (c.g[V0][0] < 3) as u64;
        // `t` is `(b - 9) < 3` compared against zero: the table scan is the
        // arm where it is *not* a low id (b >= 12), the other one returns b-6.
        let high_id = c.g[V0][0] != 0;
        c.g[V0][0] = 0x46_0000;
        if !high_id {
            table = true;
        }
    }
    if !table {
        c.cycles += 2;
        c.g[V0][0] = c.g[A2][0];
        abi::ret(c);
        return;
    }
    c.cycles += 5;
    c.g[V1][0] = sx(r32(c, A0) << 4);
    c.g[V0][0] = low(c, V0).wrapping_sub(0x5d08);
    c.g[V1][0] = low(c, V1).wrapping_add(low(c, V0));
    c.g[A2][0] = c.g[V1][0];
    c.g[A3][0] = 0;
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, A2)).lw(c);
        let hit = c.g[V0][0] == c.g[A1][0];
        c.g[A3][0] = low(c, A3).wrapping_add(1);
        if hit {
            c.cycles += 2;
            c.g[V0][0] = lw_at(c, V1, 0x4);
            abi::ret(c);
            return;
        }
        c.cycles += 4;
        c.g[A2][0] = low(c, A2).wrapping_add(8);
        c.g[V0][0] = ((c.g[A3][0] as i64) < 2) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V1][0] = low(c, V1).wrapping_add(8);
        if !more {
            c.cycles += 2;
            c.g[V0][0] = 0x1a;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0014_abb8);
        }
    }
}

/// `UIState_ctor` (0x39E2A0): the base constructor of every UI state object
/// (list links, vtable, engine, id byte and the widget list at +0x24).
pub fn ui_state_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S0, 0x20);
        c.g[A2][0] = 0xf;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = 0xffff_ffff_ffff_ffff;
        c.sb(r32(c, S0).wrapping_add(0x14), c.g[S1][0]);
        c.g[V1][0] = 0x218;
        sw_at(c, S0, 0x4, c.g[S0][0]);
        c.g[V0][0] = low(c, V0).wrapping_add(0x46c8);
        sw_at(c, S0, 0, c.g[S0][0]);
        c.g[A0][0] = low(c, S0).wrapping_add(0x24);
        sw_at(c, S0, 0xc, 0);
        sw_at(c, S0, 0x20, 0);
        sw_at(c, S0, 0x10, c.g[A1][0]);
        c.sb(r32(c, S0).wrapping_add(0x15), c.g[A2][0]);
        c.sh(r32(c, S0).wrapping_add(0x1c), c.g[V1][0]);
        sw_at(c, S0, 0x8, c.g[V0][0]);
        callee::UI_LIST_CTOR.call(c, 0x0039_e2f4)?;
        c.sb(r32(c, S0).wrapping_add(0x44), c.g[S1][0]);
        c.g[V0][0] = c.g[S0][0];
        sw_at(c, S0, 0x40, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, DATA, GAME_GP};

    const SEEDS: u32 = 20;

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }
    fn hh(c: &mut Cpu, a: u32, v: u16) {
        c.bus.write16(a, v);
    }
    fn bb(c: &mut Cpu, a: u32, v: u8) {
        c.bus.write8(a, v);
    }
    fn fb(x: f32) -> u32 {
        x.to_bits()
    }

    /// The BELibrary singleton (`gp - 0x1f04`) pointed at a zeroed dummy.
    fn belib(c: &mut Cpu) {
        w(c, GAME_GP.wrapping_sub(0x1f04), DATA + 0x8000);
    }

    /// A dummy world (`gp - 0x848`) with the race-manager chain the callees
    /// walk: world+0x84 -> race man, race man+0x20 (painter), +0x28 (riders),
    /// +0x78 (rider count), world+0x7c (rider array).
    fn world(c: &mut Cpu) {
        let wld = DATA + 0x1000;
        w(c, GAME_GP.wrapping_sub(0x848), wld);
        w(c, wld + 0x84, DATA + 0x1200);
        w(c, DATA + 0x1200 + 0x20, DATA + 0x3000);
        w(c, DATA + 0x1200 + 0x28, DATA + 0x1400);
        w(c, DATA + 0x1200 + 0x78, 0);
        w(c, wld + 0x7c, DATA + 0x1500);
    }

    /// A vtable slot `{delta, fn}` at `vt + off` (the game's `lh`/`lw` pair).
    fn slot(c: &mut Cpu, vt: u32, off: u32, delta: u16, f: u32) {
        hh(c, vt + off, delta);
        w(c, vt + off + 4, f);
    }

    /// A NUL-terminated string at `a`.
    fn string(c: &mut Cpu, a: u32, s: &str) {
        for (i, ch) in s.bytes().enumerate() {
            bb(c, a + i as u32, ch);
        }
        bb(c, a + s.len() as u32, 0);
    }

    /// The allocator's main heap header (`D_004FF250`) over 256 KiB of test
    /// RAM, as `DLHeap_InitArena` leaves it (the arena itself stays zeroed).
    fn heap_setup(c: &mut Cpu) {
        const HEAP: u32 = 0x004f_f250;
        let base = DATA + 0x40_0000;
        let start = base.wrapping_add(0xf) & !0xf;
        let rest = 0x4_0000u32.wrapping_sub(start - base) & !0xf;
        w(c, HEAP + 0xc, rest);
        w(c, HEAP + 4, start);
        w(c, HEAP, start);
        w(c, HEAP + 4, start + 0x380);
        w(c, HEAP + 0xc, rest - 0x380);
        w(c, HEAP + 8, start + 0x380);
    }

    /// Run a guest helper during setup; its `v0`.
    fn call0(c: &mut Cpu, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3][i]][0] = *a;
        }
        c.g[RA][0] = crate::difftest::RA as u64;
        f(c);
        c.g[V0][0]
    }

    /// A list of `n` nodes at `base`, the last pointing at itself; every node
    /// points at the visitor table `vt` and holds an empty string at +0x20.
    fn list(c: &mut Cpu, base: u32, n: u32, head_field: u32, vt: u32) {
        w(c, head_field, base);
        for i in 0..n {
            let node = base + 0x100 * i;
            let next = if i + 1 == n { node } else { node + 0x100 };
            w(c, node + 4, next);
            w(c, node + 8, vt);
            string(c, node + 0x20, "");
        }
    }

    // ------------------------------------------------------------ list walks

    #[test]
    fn ui_list_accept_visitor_matches() {
        check(ee::s13::f_003979f8__generated, ui_list_accept_visitor, SEEDS, &|c, seed| {
            machine(c, seed, 0x39_79f8, 4);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x500) as u64;
            string(c, DATA + 0x500, "");
            list(c, DATA + 0x100, 3, DATA + 4, DATA + 0x600);
            let real = seed % 2 == 0;
            slot(
                c,
                DATA + 0x600,
                0x10,
                if real { 0x20 } else { 0 },
                if real { 0x0041_aa88 } else { FAKE },
            );
            c.pc = 0x0039_79f8;
        });
    }

    #[test]
    fn ui_list_v03_matches() {
        check(ee::s13::f_00397a68__generated, ui_list_v03, SEEDS, &|c, seed| {
            machine(c, seed, 0x39_7a68, 4);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x500) as u64;
            string(c, DATA + 0x500, "");
            list(c, DATA + 0x100, 3, DATA + 4, DATA + 0x600);
            let real = seed % 2 == 0;
            slot(
                c,
                DATA + 0x600,
                0x18,
                if real { 0x20 } else { 0 },
                if real { 0x0041_aa88 } else { FAKE },
            );
            c.pc = 0x0039_7a68;
        });
    }

    #[test]
    fn flex_rail_update_space_node_matches() {
        check(ee::s11::f_00348058__generated, flex_rail_update_space_node, SEEDS, &|c, seed| {
            machine(c, seed, 0x34_8058, 1);
            world(c);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            w(c, DATA + 0x2000 + 0xc, DATA + 0x2200);
            slot(c, DATA + 0x2200, 0x190, 0, FAKE);
            for (i, v) in [1.0f32, 2.0, 3.0, 0.0].iter().enumerate() {
                w(c, DATA + 0x2000 + 0x30 + 4 * i as u32, fb(*v));
            }
            for (i, v) in [-1.0f32, -2.0, -3.0, 0.0].iter().enumerate() {
                w(c, DATA + 0x2000 + 0x40 + 4 * i as u32, fb(*v));
            }
            w(c, DATA + 0x2000 + 0x54, 1 + seed % 3);
            c.pc = 0x0034_8058;
        });
    }

    #[test]
    fn flex_bridge_springs_save_matches() {
        check(ee::s11::f_003545d8__generated, flex_bridge_springs_save, SEEDS, &|c, seed| {
            machine(c, seed, 0x35_45d8, 2);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.g[A1][0] = (DATA + 0x2200) as u64;
            w(c, DATA + 0x2200, DATA + 0x2400);
            slot(c, DATA + 0x2400, 0x8, 0, FAKE);
            w(c, DATA + 0x2000, 32);
            w(c, DATA + 0x2000 + 0x24, 0x10);
            c.pc = 0x0035_45d8;
        });
    }

    // ---------------------------------------------------------- front-end states

    #[test]
    fn c_fe_state_net_messenger_init_matches() {
        check(ee::s03::f_001a3f68__generated, c_fe_state_net_messenger_init, 12, &|c, seed| {
            machine(c, seed, 0x1a_3f68, 5);
            belib(c);
            heap_setup(c);
            world(c);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            w(c, DATA + 0x2000 + 0x10, DATA + 0x2200);
            c.pc = 0x001a_3f68;
        });
    }

    #[test]
    fn c_fe_state_net_messenger_v33_matches() {
        check(ee::s03::f_001a4928__generated, c_fe_state_net_messenger_v33, SEEDS, &|c, seed| {
            machine(c, seed, 0x1a_4928, 4);
            belib(c);
            world(c);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            w(c, DATA + 0x2000 + 0x6d0, DATA + 0x2400);
            string(c, DATA + 0x2400, "");
            if seed % 2 == 0 {
                c.g[A2][0] = 0xf;
            } else {
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                w(c, DATA + 0x2000 + 0x69c, DATA + 0x2500);
            }
            c.pc = 0x001a_4928;
        });
    }

    #[test]
    fn c_fe_state_chat_room_on_go_to_char_select_event_matches() {
        check(
            ee::s03::f_001b3760__generated,
            c_fe_state_chat_room_on_go_to_char_select_event,
            12,
            &|c, seed| {
                machine(c, seed, 0x1b_3760, 5);
                belib(c);
                heap_setup(c);
                world(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                if seed % 2 == 0 {
                    c.g[A2][0] = 0xf;
                    w(c, DATA + 0x2000 + 0x10, DATA + 0x2200);
                } else {
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 0;
                    w(c, DATA + 0x2000 + 0x69c, DATA + 0x2500);
                }
                c.pc = 0x001b_3760;
            },
        );
    }

    #[test]
    fn c_fe_state_chat_room_on_go_to_char_select_event_2_matches() {
        check(
            ee::s03::f_001b37d8__generated,
            c_fe_state_chat_room_on_go_to_char_select_event_2,
            12,
            &|c, seed| {
                machine(c, seed, 0x1b_37d8, 5);
                belib(c);
                heap_setup(c);
                world(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                if seed % 2 == 0 {
                    c.g[A2][0] = 0xf;
                    w(c, DATA + 0x2000 + 0x10, DATA + 0x2200);
                } else {
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 0;
                    w(c, DATA + 0x2000 + 0x69c, DATA + 0x2500);
                }
                c.pc = 0x001b_37d8;
            },
        );
    }

    #[test]
    fn c_fe_state_online_main_menu_v31_matches() {
        check(ee::s03::f_001c0358__generated, c_fe_state_online_main_menu_v31, SEEDS, &|c, seed| {
            machine(c, seed, 0x1c_0358, 5);
            belib(c);
            heap_setup(c);
            world(c);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.g[A1][0] = [0xefu64, 0xf3, 0x42][(seed % 3) as usize];
            c.g[A2][0] = 0;
            w(c, DATA + 0x2000 + 0x10, DATA + 0x2200);
            c.pc = 0x001c_0358;
        });
    }

    #[test]
    fn cov_state_view_popup_ctor_matches() {
        check(ee::s04::f_001e51d0__generated, cov_state_view_popup_ctor, SEEDS, &|c, seed| {
            machine(c, seed, 0x1e_51d0, 1);
            belib(c);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.g[A1][0] = (DATA + 0x2200) as u64;
            c.g[A2][0] = (DATA + 0x2400) as u64;
            c.g[A3][0] = (seed % 7) as u64;
            c.g[T0][0] = (seed % 5) as u64;
            c.pc = 0x001e_51d0;
        });
    }

    #[test]
    fn ov_screen_freestyle_standings_handle_input_matches() {
        check(
            ee::s04::f_001e65b8__generated,
            ov_screen_freestyle_standings_handle_input,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1e_65b8, 3);
                belib(c);
                world(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.g[A1][0] = (DATA + 0x2400) as u64;
                bb(c, DATA + 0x2404, (seed % 64) as u8);
                w(c, DATA + 0x2000 + 0x40, DATA + 0x2600);
                // Give the screen one object whose name hash is what the
                // function asks for: the search returns it when the list is
                // walked (the hash itself comes from the name table below).
                w(c, DATA + 0x2600 + 4, DATA + 0x2800);
                w(c, DATA + 0x2800 + 4, DATA + 0x2800);
                c.pc = 0x001e_65b8;
            },
        );
    }

    #[test]
    fn ov_screen_final_standings_on_widget_created_matches() {
        check(
            ee::s04::f_001e7558__generated,
            ov_screen_final_standings_on_widget_created,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1e_7558, 4);
                belib(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.g[A1][0] = (DATA + 0x2400) as u64;
                let h = call0(c, ee::s10::f_00317670, &[0x0046_e510]);
                w(c, DATA + 0x2400 + 0x38, h as u32);
                bb(c, 0x0053_5c11, (seed % 2) as u8);
                c.pc = 0x001e_7558;
            },
        );
    }

    #[test]
    fn buddy_man_has_message_from_matches() {
        check(ee::s07::f_00262a38__generated, buddy_man_has_message_from, SEEDS, &|c, seed| {
            machine(c, seed, 0x26_2a38, 3);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.g[A1][0] = (DATA + 0x2300) as u64;
            c.g[A2][0] = (DATA + 0x2300) as u64;
            w(c, DATA + 0x2000 + 0xac, DATA + 0x2100);
            w(c, DATA + 0x2000 + 0xb0, DATA + 0x2110);
            w(c, DATA + 0x2100, DATA + 0x2200);
            w(c, DATA + 0x2104, DATA + 0x2210);
            w(c, DATA + 0x2200, DATA + 0x2500);
            w(c, DATA + 0x2210, DATA + 0x2600);
            // First message from the asked-for name, the second one, or neither.
            string(c, DATA + 0x2300, "rider");
            let first = seed % 3;
            string(c, DATA + 0x2500, if first == 0 { "rider" } else { "other" });
            string(c, DATA + 0x2600, if first == 1 { "rider" } else { "other" });
            c.pc = 0x0026_2a38;
        });
    }

    #[test]
    fn c_setup_man_ctor_matches() {
        check(ee::s07::f_002640c0__generated, c_setup_man_ctor, SEEDS, &|c, seed| {
            machine(c, seed, 0x26_40c0, 3);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.g[A1][0] = (DATA + 0x2200) as u64;
            c.g[A2][0] = (seed % 9) as u64;
            c.g[A3][0] = (DATA + 0x2400) as u64;
            c.pc = 0x0026_40c0;
        });
    }

    #[test]
    fn c_setup_man_send_brag_msg10_matches() {
        check(ee::s07::f_00265688__generated, c_setup_man_send_brag_msg10, SEEDS, &|c, seed| {
            machine(c, seed, 0x26_5688, 2);
            belib(c);
            heap_setup(c);
            world(c);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            w(c, DATA + 0x2000 + 0x124, if seed % 2 == 0 { 0 } else { 1 });
            w(c, DATA + 0x2000, (seed % 12) as u32);
            c.pc = 0x0026_5688;
        });
    }

    // --------------------------------------------------------------- audio

    #[test]
    fn ssx_audio_system_rider_say_20b9_callback_matches() {
        check(
            ee::s08::f_002a1388__generated,
            ssx_audio_system_rider_say_20b9_callback,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x2a_1388, 2);
                world(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.g[A1][0] = (DATA + 0x2400) as u64;
                w(c, DATA + 0x2400 + 0x1c, (seed % 5) as u32);
                w(c, DATA + 0x1200 + 0x78, (seed % 4) as u32);
                for i in 0..4u32 {
                    w(c, DATA + 0x1200 + 0x28 + 4 * i, 100 + i);
                    w(c, DATA + 0x1400 + 0x40 * i + 0x78, 0);
                }
                c.pc = 0x002a_1388;
            },
        );
    }

    #[test]
    fn ssx_audio_system_restart_all_board_loops_matches() {
        check(
            ee::s07::f_002929d8__generated,
            ssx_audio_system_restart_all_board_loops,
            12,
            &|c, seed| {
                machine(c, seed, 0x29_29d8, 3);
                world(c);
                c.g[A0][0] = (DATA + 0x20_0000) as u64;
                w(c, DATA + 0x1200 + 0x78, (seed % 3) as u32);
                for i in 0..3u32 {
                    w(c, DATA + 0x1200 + 0x28 + 4 * i, 100 + i);
                }
                c.pc = 0x0029_29d8;
            },
        );
    }

    #[test]
    fn ssx_audio_system_create_matches() {
        check(ee::s07::f_00284bb0__generated, ssx_audio_system_create, SEEDS, &|c, seed| {
            machine(c, seed, 0x28_4bb0, 3);
            belib(c);
            heap_setup(c);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            // An existing sound system (the singleton create path allocates
            // and constructs the whole object).
            w(c, GAME_GP + 0x410, DATA + 0x3000);
            w(c, DATA + 0x3000 + 0x118, DATA + 0x3200);
            w(c, DATA + 0x3200, DATA + 0x3400);
            c.pc = 0x0028_4bb0;
        });
    }

    #[test]
    fn ssx_audio_system_set_music_volume_matches() {
        check(ee::s07::f_00287410__generated, ssx_audio_system_set_music_volume, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x28_7410, 4);
            belib(c);
            c.g[A0][0] = (DATA + 0x7000) as u64;
            c.g[A1][0] = (seed % 11) as u64;
            c.f[12] = r.edge_f32();
            w(c, DATA + 0x7000 + 0x62b8, (seed % 2) as u32);
            w(c, DATA + 0x7000 + 0x608c, (seed % 4) as u32);
            c.pc = 0x0028_7410;
        });
    }

    #[test]
    fn ssx_audio_system_set_music_channel_enabled_matches() {
        check(
            ee::s07::f_00287558__generated,
            ssx_audio_system_set_music_channel_enabled,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x28_7558, 3);
                belib(c);
                c.g[A0][0] = (DATA + 0x7000) as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(c, DATA + 0x7000 + 0x608c, (seed % 4) as u32);
                c.pc = 0x0028_7558;
            },
        );
    }

    // ------------------------------------------------------------ script engine

    #[test]
    fn script_engine_load_script_bank_matches() {
        check(ee::s07::f_002797c8__generated, script_engine_load_script_bank, 12, &|c, seed| {
            machine(c, seed, 0x27_97c8, 4);
            belib(c);
            heap_setup(c);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.g[A1][0] = (seed % 32) as u64;
            // All bank slots busy: the finder returns -1 and the async load
            // takes its short path.
            for i in 0..8u32 {
                w(c, DATA + 0x2000 + 0x648 + 0x10 * i + 8, 1);
            }
            c.pc = 0x0027_97c8;
        });
    }

    #[test]
    fn script_engine_is_finish_script_active_matches() {
        check(ee::s07::f_0027aa80__generated, script_engine_is_finish_script_active, SEEDS, &|c, seed| {
            machine(c, seed, 0x27_aa80, 1);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            w(c, DATA + 0x2000 + 0x550, (seed % 2) as u32);
            w(c, DATA + 0x2000 + 0x54c, DATA + 0x2400);
            // The entry type the list query returns.
            w(c, DATA + 0x2400 + 0xcc, (seed % 20) as u32);
            c.pc = 0x0027_aa80;
        });
    }

    #[test]
    fn script_view_read_fields_matches() {
        check(ee::s07::f_0027c8e0__generated, script_view_read_fields, SEEDS, &|c, seed| {
            machine(c, seed, 0x27_c8e0, 3);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.g[A1][0] = (DATA + 0x2200) as u64;
            w(c, DATA + 0x2000 + 0x24, DATA + 0x2400);
            w(c, DATA + 0x2400 + 4, DATA + 0x2600);
            let (d1, f1) = (0x20u32, 0x24u32);
            let (d2, f2) = (0x28u32, 0x2c);
            w(c, DATA + 0x2600 + d1, 0);
            w(c, DATA + 0x2600 + f1, FAKE);
            w(c, DATA + 0x2600 + d2, 0);
            w(c, DATA + 0x2600 + f2, FAKE);
            c.pc = 0x0027_c8e0;
        });
    }

    #[test]
    fn script_dummy_obj_read_fields_matches() {
        check(ee::s07::f_0027cbb0__generated, script_dummy_obj_read_fields, SEEDS, &|c, seed| {
            machine(c, seed, 0x27_cbb0, 3);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.g[A1][0] = (DATA + 0x2200) as u64;
            w(c, DATA + 0x2000 + 0x24, DATA + 0x2400);
            w(c, DATA + 0x2400 + 4, DATA + 0x2600);
            w(c, DATA + 0x2600 + 0x20, 0);
            w(c, DATA + 0x2600 + 0x24, FAKE);
            w(c, DATA + 0x2600 + 0x28, 0);
            w(c, DATA + 0x2600 + 0x2c, FAKE);
            c.pc = 0x0027_cbb0;
        });
    }

    #[test]
    fn script_view_obj_reset_matches() {
        check(ee::s07::f_00281a50__generated, script_view_obj_reset, SEEDS, &|c, seed| {
            machine(c, seed, 0x28_1a50, 0);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.pc = 0x0028_1a50;
        });
    }

    #[test]
    fn bank_monitor_stop_by_owner_matches() {
        check(ee::s08::f_002ad4d8__generated, bank_monitor_stop_by_owner, 12, &|c, seed| {
            machine(c, seed, 0x2a_d4d8, 2);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            let owner = 1 + (seed % 3);
            c.g[A1][0] = owner as u64;
            w(c, DATA + 0x2000 + 0x8ec, DATA + 0x3000);
            for i in 0..64u32 {
                let inst = DATA + 0x3000 + 0xc0 * i;
                let id = if i % 5 == 0 { owner } else { 1 + (i % 4) };
                w(c, inst + 0x80, id);
            }
            c.pc = 0x002a_d4d8;
        });
    }

    // ------------------------------------------------------------- AI / riders

    #[test]
    fn ai_set_rider_order_mode_matches() {
        check(ee::s00::f_00128a48__generated, ai_set_rider_order_mode, SEEDS, &|c, seed| {
            machine(c, seed, 0x12_8a48, 0);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.g[A1][0] = (seed % 2) as u64;
            let n = (seed % 4) as u32;
            w(c, DATA + 0x2000 + 0x78, n);
            for i in 0..4u32 {
                let rider = DATA + 0x2400 + 0x40 * i;
                w(c, DATA + 0x2000 + 0x28 + 4 * i, rider);
                w(c, rider + 0xec, 0xffff_ffff);
            }
            c.pc = 0x0012_8a48;
        });
    }

    #[test]
    fn ai_computer_riders_have_render_slots_matches() {
        check(ee::s00::f_0012abd0__generated, ai_computer_riders_have_render_slots, SEEDS, &|c, seed| {
            machine(c, seed, 0x12_abd0, 3);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            let n = (seed % 4) as u32;
            w(c, DATA + 0x2000 + 0x80, n);
            for i in 0..4u32 {
                let rider = DATA + 0x2400 + 0x40 * i;
                w(c, DATA + 0x2000 + 0x48 + 4 * i, rider);
                w(c, rider + 0x18, DATA + 0x2600 + 0x40 * i);
                w(c, DATA + 0x2600 + 0x40 * i + 0x4, if i % 2 == 0 { 1 } else { 0 });
            }
            c.pc = 0x0012_abd0;
        });
    }

    #[test]
    fn ai_actor_riders_have_render_slots_matches() {
        check(ee::s00::f_0012acf8__generated, ai_actor_riders_have_render_slots, SEEDS, &|c, seed| {
            machine(c, seed, 0x12_acf8, 3);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            let n = (seed % 4) as u32;
            w(c, DATA + 0x2000 + 0x84, n);
            for i in 0..4u32 {
                let rider = DATA + 0x2400 + 0x40 * i;
                w(c, DATA + 0x2000 + 0x5c + 4 * i, rider);
                w(c, rider + 0x18, DATA + 0x2600 + 0x40 * i);
                w(c, DATA + 0x2600 + 0x40 * i + 0x4, if i % 2 == 0 { 1 } else { 0 });
            }
            c.pc = 0x0012_acf8;
        });
    }

    #[test]
    fn cps2_mem_card_count_saves_matches() {
        check(ee::s08::f_002c68f0__generated, cps2_mem_card_count_saves, SEEDS, &|c, seed| {
            machine(c, seed, 0x2c_68f0, 1);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            let n = (seed % 4) as u32;
            w(c, DATA + 0x2000 + 0x14, n);
            for i in 0..4u32 {
                let name = DATA + 0x2000 + 0x1b0 + 0x78 * i;
                if i % 2 == 0 && i < n {
                    string(c, name, "SAVE");
                } else {
                    string(c, name, "");
                }
            }
            c.pc = 0x002c_68f0;
        });
    }

    #[test]
    fn spline_camera_algorithm_reset_for_rider_matches() {
        check(
            ee::s01::f_00161630__generated,
            spline_camera_algorithm_reset_for_rider,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x16_1630, 4);
                belib(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                // Half the seeds match the camera's current entry (the BELibrary
                // singleton is a deterministic return value), the rest get a
                // fake visitor that returns an unrelated pointer.
                let same = seed % 2 == 0;
                c.g[A1][0] = if same { (DATA + 0x8000) as u64 } else { DATA as u64 };
                w(c, DATA + 0x2000 + 0x30, DATA + 0x2200);
                w(c, DATA + 0x2200 + 4, DATA + 0x2400);
                w(c, DATA + 0x2400 + 0x6c0, DATA + 0x2600);
                let get = if seed % 2 == 0 { 0x0014_dc80 } else { FAKE };
                slot(c, DATA + 0x2600, 0x38, 0, get);
                w(c, DATA + 0x2000 + 0x10, DATA + 0x2800);
                slot(c, DATA + 0x2800, 0x28, 0, FAKE);
                c.pc = 0x0016_1630;
            },
        );
    }

    #[test]
    fn mc_manager_return_to_slot_state_matches() {
        check(ee::s06::f_00242050__generated, mc_manager_return_to_slot_state, SEEDS, &|c, seed| {
            machine(c, seed, 0x24_2050, 3);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            w(c, DATA + 0x2000 + 0x434, DATA + 0x2200);
            w(c, DATA + 0x2000 + 0x748, DATA + 0x2400);
            // The busy probe: itself (strcasecmp -> 0, i.e. not busy) or a fake
            // (non-zero, i.e. busy).
            slot(
                c,
                DATA + 0x2200,
                0x58,
                0,
                if seed % 2 == 0 { 0x0041_aa88 } else { FAKE },
            );
            slot(c, DATA + 0x2400, 0x8, 0, FAKE);
            c.pc = 0x0024_2050;
        });
    }

    #[test]
    fn parent_modifier_ctor_2_matches() {
        check(ee::s11::f_00357090__generated, parent_modifier_ctor_2, SEEDS, &|c, seed| {
            machine(c, seed, 0x35_7090, 3);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.g[A1][0] = (DATA + 0x2200) as u64;
            w(c, DATA + 0x2200, DATA + 0x2400);
            slot(c, DATA + 0x2400, 0x10, 0, FAKE);
            slot(c, DATA + 0x2400, 0x18, 0, FAKE);
            c.pc = 0x0035_7090;
        });
    }

    #[test]
    fn lobby_menu_send_chat_matches() {
        check(ee::s02::f_0017e418__generated, lobby_menu_send_chat, SEEDS, &|c, seed| {
            machine(c, seed, 0x17_e418, 4);
            belib(c);
            heap_setup(c);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.g[A1][0] = (DATA + 0x3000) as u64;
            c.pc = 0x0017_e418;
        });
    }

    #[test]
    fn height_fog_menu_dtor_matches() {
        check(ee::s06::f_0024fa00__generated, height_fog_menu_dtor, SEEDS, &|c, seed| {
            machine(c, seed, 0x24_fa00, 4);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.g[A1][0] = (seed % 2) as u64;
            c.pc = 0x0024_fa00;
        });
    }

    #[test]
    fn c_fe_state_preview_reward_set_item_matches() {
        check(
            ee::s04::f_001d20d8__generated,
            c_fe_state_preview_reward_set_item,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1d_20d8, 2);
                world(c);
                belib(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.g[A1][0] = (seed % 10) as u64;
                c.g[A2][0] = (DATA + 0x2200) as u64;
                // The image request names a bolt slot that already exists in the
                // (otherwise zeroed) bolt file, so the request takes its "slot
                // found" arm instead of allocating a slot and starting a disc
                // read (0x19d780 is a lazy-load loop and runs away on test data).
                string(c, DATA + 0x2300, "BOLT");
                w(c, DATA + 0x2200 + 4, DATA + 0x2300);
                // The file base is `world->7c + 0x1a70` = DATA + 0x2f70; the
                // slot names sit at +4, 0x120 apart.
                string(c, DATA + 0x2f74, "BOLT");
                c.pc = 0x001d_20d8;
            },
        );
    }

    // ------------------------------------------------ riders, cards, misc

    #[test]
    fn c_fe_state_char_equip_refresh_rider_bolts_matches() {
        check(
            ee::s03::f_0019aca0__generated,
            c_fe_state_char_equip_refresh_rider_bolts,
            12,
            &|c, seed| {
                machine(c, seed, 0x19_aca0, 5);
                belib(c);
                world(c);
                heap_setup(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                bb(c, DATA + 0x2000 + 0x44, (seed % 3) as u8);
                w(c, DATA + 0x2000 + 0xbc, 7 + seed % 3);
                c.pc = 0x0019_aca0;
            },
        );
    }

    #[test]
    fn bolt_file_find_free_slot_matches() {
        check(ee::s03::f_0019d3b0__generated, bolt_file_find_free_slot, SEEDS, &|c, seed| {
            machine(c, seed, 0x19_d3b0, 0);
            c.g[A0][0] = DATA as u64;
            let bank = (seed % 2) as u32;
            c.g[A1][0] = bank as u64;
            let base = DATA + 0x1_2000 * bank;
            let free_at = if seed % 3 == 0 { 256 } else { 200 + seed % 40 };
            for i in 0..256u32 {
                let s = base + 0x120 * i;
                let empty = i >= free_at;
                let valid = if empty { 0 } else { 1 };
                let used = if empty { 0 } else if i % 4 == 0 { 0 } else { 1 };
                w(c, s, used);
                w(c, s + 0x104, valid);
                w(c, s + 0x11c, (i * 7) % 64);
            }
            c.pc = 0x0019_d3b0;
        });
    }

    #[test]
    fn input_sync_is_ready_matches() {
        check(ee::s06::f_00237950__generated, input_sync_is_ready, SEEDS, &|c, seed| {
            machine(c, seed, 0x23_7950, 0);
            w(c, GAME_GP.wrapping_sub(0x204), DATA + 0x3000);
            w(c, DATA + 0x3000 + 0x70, (seed % 20) as u32);
            w(c, DATA + 0x3000 + 0x3f8, (seed % 3 == 0) as u32);
            w(c, DATA + 0x3000 + 0x3fc, if seed % 3 == 1 { 2 } else { 0 });
            c.g[A0][0] = (DATA + 0x2000) as u64;
            let n = ((seed / 3) % 4) as u32;
            w(c, DATA + 0x2000 + 0x10, n);
            for i in 0..4u32 {
                let s = DATA + 0x2000 + 0x14 + 0x18 * i;
                w(c, s + 4, 1 + seed % 3);
                let full = i + 1 == n && seed % 5 == 0;
                w(c, s + 8, if full { 1 + seed % 3 } else { 2 });
            }
            c.pc = 0x0023_7950;
        });
    }

    #[test]
    fn rider_mark_quit_requested_matches() {
        check(ee::s00::f_001253d0__generated, rider_mark_quit_requested, SEEDS, &|c, seed| {
            machine(c, seed, 0x12_53d0, 1);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            w(c, DATA + 0x2000 + 0x470, if seed % 2 == 0 { fb(-1.0) } else { fb(0.5) });
            w(c, DATA + 0x2000 + 0x6c0, DATA + 0x2400);
            slot(c, DATA + 0x2400, 0x38, 0, 0x0027_7f08);
            c.pc = 0x0012_53d0;
        });
    }

    #[test]
    fn snd_bank_free_memory_matches() {
        check(ee::s07::f_0028b650__generated, snd_bank_free_memory, SEEDS, &|c, seed| {
            machine(c, seed, 0x28_b650, 2);
            heap_setup(c);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            w(c, DATA + 0x2000 + 0x8, [0u32, 1, 2][(seed % 3) as usize]);
            w(c, DATA + 0x2000 + 0xc, 0x11);
            w(c, DATA + 0x2000 + 0x10, DATA + 0x2600);
            w(c, DATA + 0x2000 + 0x14, 0x22);
            w(c, GAME_GP + 0x524, DATA + 0x3000);
            w(c, DATA + 0x3000 + 0x1d8, DATA + 0x3200);
            w(c, DATA + 0x3200, DATA + 0x3400);
            slot(c, DATA + 0x3400, 0x28, 0, FAKE);
            c.pc = 0x0028_b650;
        });
    }

    #[test]
    fn c_fe_state_options_device_select_ctor_matches() {
        check(
            ee::s02::f_0018d7f0__generated,
            c_fe_state_options_device_select_ctor,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x18_d7f0, 3);
                belib(c);
                world(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.g[A1][0] = (DATA + 0x2200) as u64;
                c.g[A2][0] = (seed % 5) as u64;
                c.g[A3][0] = if seed % 2 == 0 { 0 } else { (DATA + 0x2400) as u64 };
                c.g[T0][0] = (seed % 7) as u64;
                c.pc = 0x0018_d7f0;
            },
        );
    }

    #[test]
    fn mem_card_state_base_show_status_matches() {
        check(ee::s04::f_001d8d68__generated, mem_card_state_base_show_status, 12, &|c, seed| {
            machine(c, seed, 0x1d_8d68, 3);
            belib(c);
            world(c);
            heap_setup(c);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            w(c, DATA + 0x2000 + 0x8, DATA + 0x2400);
            slot(c, DATA + 0x2400, 0x120, 0, FAKE);
            c.pc = 0x001d_8d68;
        });
    }

    #[test]
    fn ov_screen_save_replay_on_child_event_matches() {
        check(
            ee::s05::f_001fe8f0__generated,
            ov_screen_save_replay_on_child_event,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1f_e8f0, 3);
                world(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.g[A1][0] = (DATA + 0x2200) as u64;
                c.g[A2][0] = if seed % 2 == 0 { 0x16 } else { 0x11 };
                w(c, DATA + 0x2000 + 0x19c, if seed % 3 == 0 { 5 } else { 4 });
                c.pc = 0x001f_e8f0;
            },
        );
    }

    #[test]
    fn c_transition_man_script_advance_timer_matches() {
        check(ee::s07::f_00282d30__generated, c_transition_man_script_advance_timer, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x28_2d30, 2);
            c.f[12] = r.edge_f32();
            c.g[A0][0] = (DATA + 0x2000) as u64;
            let (a, b_, d) = if seed % 2 == 0 {
                (fb(1.0), fb(2.0), fb(4.0))
            } else {
                (fb(100.0), fb(0.5), fb(0.25))
            };
            w(c, DATA + 0x2000 + 0x14, if seed % 3 == 0 { 0xffff_ffff } else { 3 });
            w(c, DATA + 0x2000 + 0x1c, a);
            w(c, DATA + 0x2000 + 0x20, fb(0.5));
            w(c, DATA + 0x2000 + 0x24, fb(0.25));
            w(c, DATA + 0x2000 + 0x2c, b_);
            let _ = d;
            c.pc = 0x0028_2d30;
        });
    }

    #[test]
    fn rider_metrix_record_best_jump_matches() {
        check(ee::s00::f_00119448__generated, rider_metrix_record_best_jump, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x11_9448, 2);
            c.f[12] = r.f32(500.0);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            w(c, DATA + 0x2000 + 0x130, seed % 9);
            w(c, DATA + 0x2000 + 0x1b0, if seed % 3 == 0 { 0 } else { DATA + 0x2200 });
            w(c, DATA + 0x2000 + 0x18, if seed % 2 == 0 { fb(-100.0) } else { fb(1e6) });
            c.pc = 0x0011_9448;
        });
    }

    #[test]
    fn lens_fx_get_flare_size_for_res_matches() {
        check(ee::s09::f_002e27e8__generated, lens_fx_get_flare_size_for_res, SEEDS, &|c, seed| {
            machine(c, seed, 0x2e_27e8, 0);
            c.g[A0][0] = match seed % 7 {
                0 => 0x20,
                1 => 0x10,
                2 => 0x40,
                3 => 0x30,
                4 => 33,
                5 => 0,
                _ => 0xffff_ffff_ffff_ffff,
            };
            c.pc = 0x002e_27e8;
        });
    }

    #[test]
    fn dl_heap_init_arena_matches() {
        check(ee::s10::f_0031ff60__generated, dl_heap_init_arena, SEEDS, &|c, seed| {
            machine(c, seed, 0x31_ff60, 1);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.g[A1][0] = (DATA + 0x3000 + 0x1000 * (seed % 4) + seed % 8) as u64;
            c.g[A2][0] = (0x4000 + 0x1000 * (seed % 3)) as u64;
            c.pc = 0x0031_ff60;
        });
    }

    #[test]
    fn be_get_event_record_index_matches() {
        check(ee::s01::f_0014ab68__generated, be_get_event_record_index, SEEDS, &|c, seed| {
            machine(c, seed, 0x14_ab68, 0);
            let a = (seed % 20) as u32;
            let b = [6u32, 7, 8, 9, 10, 11, 12, 3][(seed % 8) as usize];
            c.g[A0][0] = a as u64;
            c.g[A1][0] = b as u64;
            let row = DATA + 0x2200 + 0x20 * (a % 5);
            if a < 17 {
                w(c, 0x0045_a2f8 + 16 * a, row);
            }
            w(c, row, b);
            w(c, row + 4, 3 + seed % 4);
            w(c, row + 8, 0xffff_ffff);
            w(c, row + 0xc, 0x1a);
            c.pc = 0x0014_ab68;
        });
    }

    #[test]
    fn ui_state_ctor_matches() {
        check(ee::s13::f_0039e2a0__generated, ui_state_ctor, SEEDS, &|c, seed| {
            machine(c, seed, 0x39_e2a0, 2);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.g[A1][0] = (DATA + 0x2200) as u64;
            c.pc = 0x0039_e2a0;
        });
    }
}
