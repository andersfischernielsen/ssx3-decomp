//! Phase 2 wave 44 module `oc123` (docs/investigation/phase2-worker-guide.md):
//! typed replacements for fifty cold-path helpers from the front end, UI,
//! script engine, sound, replay, camera and input code, each with a
//! differential test against the generated body.
//!
//! Shapes covered here:
//!
//! - **list walks**: `UIState_ShowObject`, `ProcessList_DeletePriority`,
//!   `cWorldCache_StreamerGetViewClearanceAll` and the destructor loops
//!   (`cFlagManager_dtor`, `CameraTriggerMan_dtor`, `WorldTriggerManager_dtor`,
//!   `cWScriptCache_dtor`, the popup destructors) walk a table backwards or a
//!   list forwards and call one virtual slot per element;
//! - **event dispatch**: `cFEStateNetMessenger_OnChildEvent`,
//!   `NetFEState_OnWidgetAction`, `cFEOptionsCredits_HandleInput`,
//!   `cFEStateRiderBio_OnWidgetAction` route a message to a virtual handler by
//!   message kind;
//! - **FE state screens**: the `*_Init` / `_ctor` hooks build objects through
//!   the engine allocator and the UI engine;
//! - **pure math**: `RiderSpinSub_GetFlipHUDBlend`, `Irradiance_AddScaled`,
//!   `RailTube_ctor`, `Replay_FindWeakestHighlight`,
//!   `BoltFile_FindSlotByName`, `DebugMenu_NextEnabledItem`.
//!
//! Registers and cycles follow the generated body; calls go through the
//! function table exactly where the original used `jalr` or a cross-shard
//! tail, and every loop with a back-edge event check uses
//! [`crate::events::event_due`] + [`deliver_event_here`], never a bare
//! `c.event()`.

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0039_e948, ui_state_show_object, "UIState_ShowObject"),
    (
        0x0035_5118,
        process_list_delete_priority,
        "ProcessList_DeletePriority",
    ),
    (
        0x0019_46a8,
        cfe_state_transition_start_fade,
        "cFEStateTransition_StartFade",
    ),
    (
        0x0035_11d0,
        instances_man_alloc_by_handle,
        "InstancesMan_AllocByHandle",
    ),
    (
        0x001a_4c58,
        cfe_state_net_messenger_on_child_event,
        "cFEStateNetMessenger_OnChildEvent",
    ),
    (0x001a_89b0, net_fe_state_v26, "NetFEState_v26"),(
        0x001b_a9f8,
        cfe_state_create_step2_submit_account,
        "cFEStateCreateStep2_SubmitAccount",
    ),
    (
        0x001d_f4e8,
        cfe_state_bragging_rights_dtor,
        "cFEStateBraggingRights_dtor",
    ),
    (
        0x001e_3e30,
        ov_screen_message_center_shutdown,
        "OVScreenMessageCenter_Shutdown",
    ),
    (
        0x0026_2468,
        buddy_man_find_roster_entry,
        "BuddyMan_FindRosterEntry",
    ),
    (
        0x0027_99e0,
        script_engine_on_data_removed,
        "ScriptEngine_OnDataRemoved",
    ),(0x003a_62a0, c_ui_text_scroll_dtor, "cUITextScroll_dtor"),
    (0x0037_2520, rail_tube_ctor, "RailTube_ctor"),
    (0x0038_9590, irradiance_add_scaled, "Irradiance_AddScaled"),
    (0x0023_fe70, mc_manager_state25, "MCManager_State25"),(
        0x0024_5248,
        c_fe_state_progressive_init,
        "cFEStateProgressive_Init",
    ),
    (
        0x0017_a028,
        debug_menu_next_enabled_item,
        "DebugMenu_NextEnabledItem",
    ),
    (
        0x001d_1c08,
        c_fe_state_preview_reward_init,
        "cFEStatePreviewReward_Init",
    ),
    (
        0x001d_22f0,
        c_fe_state_video_player_ctor,
        "cFEStateVideoPlayer_ctor",
    ),
    (
        0x0019_d428,
        bolt_file_find_slot_by_name,
        "BoltFile_FindSlotByName",
    ),
    (
        0x0028_d5a0,
        ssx_audio_system_update_music_tier,
        "SSXAudioSystem_UpdateMusicTier",
    ),
    (
        0x0018_8e58,
        c_fe_state_options_open_reset_popup,
        "cFEStateOptions_OpenResetPopup",
    ),(
        0x0032_c898,
        sphere_tree_collide_sphere,
        "SphereTree_CollideSphere",
    ),
    (
        0x0032_c928,
        sphere_tree_collide_aabb,
        "SphereTree_CollideAABB",
    ),
    (
        0x0032_fdb0,
        coll_sphere_collide_sphere_tree,
        "CollSphere_CollideSphereTree",
    ),(0x0032_6d60, ps2_pad_create, "PS2Pad_Create"),
    (
        0x003a_9770,
        c_world_cache_streamer_get_view_clearance_all,
        "cWorldCache_StreamerGetViewClearanceAll",
    ),
    (
        0x0026_eea0,
        replay_restore_game_cameras,
        "Replay_RestoreGameCameras",
    ),
    (0x0034_add8, flag_ctor, "Flag_ctor"),
    (
        0x0035_ab60,
        multi_spline_modifier_v23,
        "MultiSplineModifier_v23",
    ),
    (
        0x0039_c398,
        c_ui_progress_update_anim,
        "cUIProgress_UpdateAnim",
    ),
    (0x0034_c4b8, c_flag_manager_dtor, "cFlagManager_dtor"),
    (
        0x0023_02a8,
        game_module_quick_restart,
        "GameModule_QuickRestart",
    ),
    (0x001f_3700, c_fe_state_lodge_ctor, "cFEStateLodge_ctor"),
    (0x0032_1500, input_parser_error, "InputParser_Error"),
    (
        0x0015_cd80,
        manual_cam_controller_ctor,
        "ManualCamController_ctor",
    ),
    (
        0x0028_3580,
        script_sound_bank_man_add_bank,
        "ScriptSoundBankMan_AddBank",
    ),
    (
        0x0018_63e8,
        c_fe_options_credits_handle_input,
        "cFEOptionsCredits_HandleInput",
    ),
    (
        0x0019_12d0,
        c_fe_state_rider_bio_on_widget_action,
        "cFEStateRiderBio_OnWidgetAction",
    ),
    (
        0x0016_c888,
        camera_trigger_man_dtor,
        "CameraTriggerMan_dtor",
    ),
    (
        0x001a_8918,
        net_fe_state_on_widget_action,
        "NetFEState_OnWidgetAction",
    ),
    (
        0x001a_bed8,
        net_fe_state_show_max_buddies_popup,
        "NetFEState_ShowMaxBuddiesPopup",
    ),
    (
        0x001a_c0b8,
        c_fe_use_existing_account_init,
        "cFEUseExistingAccount_Init",
    ),
];

// ---------------------------------------------------------------------------
// Callees (addresses the generated body reaches through the function table).

mod callee {
    pub const UI_LIST_FIRST: u32 = 0x0039_7840;
    pub const UI_LIST_NODE_IS_SENTINEL: u32 = 0x0039_7718;
    pub const UI_SCREEN_FIND_OBJECT: u32 = 0x0039_d860;
    pub const GET_HASH_VALUE32: u32 = 0x0031_7670;

    pub const PROCESS_LIST_FIRST: u32 = 0x0035_4ea8;
    pub const PROCESS_LIST_NEXT: u32 = 0x0035_4ef8;

    pub const UI_SCREEN_PLAY_FRAME: u32 = 0x0039_c870;
    pub const SSX_AUDIO_SYSTEM_GET_INSTANCE: u32 = 0x0028_b180;
    pub const AUDIO_SET_TRIGGER: u32 = 0x0029_4f78;

    pub const NET_FE_STATE_ON_CHILD_EVENT: u32 = 0x001a_97b8;
    pub const NET_FE_STATE_GET_POPUP: u32 = 0x001a_9c50;
    pub const NET_FE_STATE_SHOW_POPUP: u32 = 0x001a_9bc0;
    pub const LOBBY_MAN_HIDE: u32 = 0x0025_b6d8;
    pub const LOBBY_MAN_SHOW: u32 = 0x0025_b9c8;
    pub const NET_FE_STATE_SHOW_DISCONNECTED_POPUP: u32 = 0x001a_9930;

    pub const CFS_GET_CHARACTER_INFO: u32 = 0x0039_a738;
    pub const CFS_SHOW_CHARACTER: u32 = 0x001b_1b10;
    pub const CFS_SHOW_OPPONENT_CHAR: u32 = 0x001b_1760;
    pub const CFS_FILL_PLAYER_INFO: u32 = 0x001b_1808;
    pub const BE_GET_INTERFACE: u32 = 0x0014_dd58;

    pub const MESSAGE_CENTER_SET_FLAG: u32 = 0x001e_3a78;
    pub const OVSCREEN_PDA_TEMPLATE_CLOSE: u32 = 0x0020_d190;
    pub const OVSCREEN_MESSAGE_CENTER_DTOR: u32 = 0x0020_a430;

    pub const BUDDY_MAN_GET_ROSTER_COUNT: u32 = 0x0026_2598;
    pub const BUDDY_MAN_GET_ROSTER_ENTRY: u32 = 0x0026_25c8;
    pub const STRCMP: u32 = 0x0041_65a8;

    pub const SCRIPT_DATA_GET_BANK_LIST: u32 = 0x0027_c078;
    pub const SCRIPT_ENGINE_UNREGISTER_SCRIPT: u32 = 0x0028_2178;
    pub const SCRIPT_ANIM_BANK_MAN_REMOVE_BANK: u32 = 0x0027_5650;
    pub const SCRIPT_SOUND_BANK_MAN_REMOVE_BANK: u32 = 0x0028_3610;

    pub const SCRIPT_ENGINE_GET_INSTANCE: u32 = 0x0027_d2e8;
    pub const SCRIPT_ENGINE_SET_OBJ_BINDING_FILTER: u32 = 0x0027_d0c0;
    pub const SCRIPT_VIEW_OBJ_RESET: u32 = 0x0028_1a50;
    pub const SCRIPT_ENGINE_GET_SEQ_INSTANCE: u32 = 0x0028_2c18;
    pub const SCRIPT_ENGINE_IS_PLAYING_SCRIPT: u32 = 0x0027_9370;
    pub const SCRIPT_VIEW_OBJ_ON_ACTIVATED: u32 = 0x0010_33b0;
    pub const SCRIPT_OBJ_ACTIVATE: u32 = 0x0028_3430;

    pub const CUI_TEXT_SCROLL_CLEAR_LINES: u32 = 0x003a_18c0;
    pub const UI_LIST_DELETE_ALL: u32 = 0x0039_7948;
    pub const UI_LIST_NODE_DTOR_BODY: u32 = 0x003a_4868;
    pub const UI_OBJECT_DTOR: u32 = 0x0039_fc48;

    pub const WORLD_CACHE_STREAMER_UPDATE_VIEW: u32 = 0x003a_9d60;
    pub const GAME_MODULE_DTOR: u32 = 0x001a_85d0;
    pub const CAMERA_TRIGGER_MAN_SET_IN_GAME_TRIGGERS: u32 = 0x0016_d1b8;
    pub const RIDER_SET_INPUT_ENABLED: u32 = 0x0015_dfd8;
    pub const CAMERA_SET_OWNER: u32 = 0x0016_1f50;

    pub const WORLD_OBJ_CTOR: u32 = 0x0034_fb00;
    pub const FLAG_READ_PARAMS: u32 = 0x0034_ac88;
    pub const GET_FLAG_MANAGER: u32 = 0x002d_1cb0;
    pub const FLAG_MANAGER_ADD_FLAG: u32 = 0x0034_c548;

    pub const UI_ANIM_APPLY: u32 = 0x0039_75e0;
    pub const UI_ANIM_ADVANCE: u32 = 0x0039_74b0;
    pub const GAME_PROCESS_DTOR: u32 = 0x0035_46c8;

    pub const GAME_MODULE_RESET_RACE: u32 = 0x0023_0180;
    pub const GAME_MODE_MAN_RESET_MODE: u32 = 0x0023_82d8;
    pub const GAME_MODULE_REQUEST_STATE: u32 = 0x0023_1250;
    pub const GAME_MODULE_START_NEXT_MODE: u32 = 0x0028_70a0;

    pub const UI_OBJECT_CTOR: u32 = 0x0039_e2a0;
    pub const BE_GET_APP: u32 = 0x0014_dc80;
    pub const BE_INTERFACE_GET_INDEX: u32 = 0x0014_7318;
    pub const BE_GET_BE: u32 = 0x001a_1cb8;

    pub const SPRINTF: u32 = 0x0041_7828;
    pub const VSPRINTF: u32 = 0x0041_86c8;

    pub const CAM_CONTROLLER_CTOR: u32 = 0x0015_cba0;
    pub const MEM_ALLOC: u32 = 0x0031_7d70;
    pub const MANUAL_CAM_ALGO_CTOR: u32 = 0x0015_fd08;
    pub const EDITOR_CAM_ALGO_CTOR: u32 = 0x0015_ffb0;
    pub const MANUAL_CAM_SELECT_ALGORITHM: u32 = 0x0015_cf80;

    pub const SND_LIST_FIND_BANK: u32 = 0x0028_36d8;
    pub const SND_LIST_FIND_FREE_ENTRY: u32 = 0x0028_3760;
    pub const SND_BANK_MAN_LOAD_BANK_FROM_MEMORY: u32 = 0x0028_be60;

    pub const UI_OBJECT_CTOR_2: u32 = 0x0039_e318;
    pub const UI_STRING_CTOR: u32 = 0x0025_3390;
    pub const OPERATOR_DELETE: u32 = 0x0031_7e50;

    pub const AUDIO_GET_MUSIC_TIER: u32 = 0x002b_49e0;
    pub const AUDIO_GET_TRACK_TYPE: u32 = 0x002a_1e20;
    pub const AUDIO_PLAY_MUSIC: u32 = 0x0028_cf98;

    pub const SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS: u32 = 0x0028_2390;
    pub const SCRIPT_ENGINE_KILL_ALL_PROCESSES: u32 = 0x0028_23f0;
    pub const SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS: u32 = 0x0027_d050;
    pub const IN_SEQ_DTOR: u32 = 0x0027_28d0;
    pub const SCRIPT_ENGINE_RESET_PROCESS_TABLE: u32 = 0x0028_28b8;

    pub const SPHERE_TREE_COMPUTE_OFFSET: u32 = 0x0032_df28;
    pub const SPHERE_TREE_COLLIDE_SPHERE_NODE: u32 = 0x0032_cdb0;
    pub const SPHERE_TREE_COLLIDE_AABB_NODE: u32 = 0x0032_d028;
    pub const SPHERE_TREE_COLLIDE_SPHERE: u32 = 0x0032_c898;

    pub const WORLD_TRIGGER_INSTANCE_DTOR: u32 = 0x002b_4be0;
    pub const PS2_PAD_CTOR: u32 = 0x0032_6df0;

    pub const WSCRIPT_CACHE_NODE_DTOR: u32 = 0x003a_ca70;

    pub const UI_ENGINE_ADD_SCREEN_BY_HASH_NAME: u32 = 0x0039_7cc0;
    pub const UI_TEXT_SET_ASCII_STRING: u32 = 0x003a_0c50;

    pub const SCRIPT_ENGINE_GET_SEQ_INSTANCE_2: u32 = 0x001d_9d48;
    pub const CBE_STRING_CTOR: u32 = 0x0031_84e8;
    pub const NET_FE_STATE_INIT: u32 = 0x001a_8768;
    pub const POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID: u32 = 0x001c_57c0;
    pub const UI_OBJECT_ADD_CHILD: u32 = 0x0039_f290;
}

// ---------------------------------------------------------------------------
// Small register helpers.

/// `a + off` (32-bit address arithmetic).
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// The address `reg + off`.
#[inline(always)]
fn at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

// ---------------------------------------------------------------------------
// FE state transition and messenger.

/// `void cUIState_showObjSafe(cUIState* self, char* name)` (0x39e948): walk the
/// object list at `self + 0x24`, and for the first node whose object hash
/// name matches, call its virtual `show(1)` (slot 0x4c of the vtable at
/// `object + 8`).
pub fn ui_state_show_object(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        let name = abi::arg(c, 1);
        c.g[A0][0] = sx(p(abi::arg(c, 0), 0x24));
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(name);
        call_ptr(c, callee::UI_LIST_FIRST, 0x0039_e964)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            loop {
                c.cycles += 8;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, callee::GET_HASH_VALUE32, 0x0039_e978)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, callee::UI_SCREEN_FIND_OBJECT, 0x0039_e984)?;
                c.g[A2][0] = c.g[V0][0];
                let found = c.g[A2][0] != 0;
                c.g[A1][0] = 1;
                if found {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw(at(c, A2, 8));
                    c.g[A0][0] = c.lh(at(c, V0, 0x48));
                    c.g[V1][0] = c.lw(at(c, V0, 0x4c));
                    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                    let target = r32(c, V1);
                    call_ptr(c, target, 0x0039_e9a4)?;
                    frame.restore(c, S0, 0x20);
                    break;
                }
                c.cycles += 5;
                c.g[S0][0] = c.lw(at(c, S0, 4));
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, callee::UI_LIST_NODE_IS_SENTINEL, 0x0039_e9b8)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x20);
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_e970);
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cBucketMan_purgeBucket(cBucketMan* self, int index)` (0x355118): for
/// every node the indexed process list yields, call the node's virtual
/// `purge(3)` (slot 1 of the vtable at `node + 0xc`).
pub fn process_list_delete_priority(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        let self_ = abi::arg(c, 0);
        let index = abi::arg(c, 1);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        c.g[S3][0] = sx(self_);
        c.g[S2][0] = sx(index);
        call_ptr(c, callee::PROCESS_LIST_FIRST, 0x0035_513c)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            frame.restore(c, S0, 0x40);
        } else {
            c.cycles += 2;
            c.g[A0][0] = c.g[S3][0];
            loop {
                c.cycles += 5;
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.g[S2][0];
                call_ptr(c, callee::PROCESS_LIST_NEXT, 0x0035_515c)?;
                let done = c.g[S0][0] == 0;
                c.g[S1][0] = c.g[V0][0];
                if !done {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw(at(c, S0, 0xc));
                    c.g[A1][0] = 3;
                    c.g[A0][0] = c.lh(at(c, V0, 8));
                    c.g[V1][0] = c.lw(at(c, V0, 0xc));
                    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                    let target = r32(c, V1);
                    call_ptr(c, target, 0x0035_517c)?;
                }
                c.cycles += 3;
                c.g[S0][0] = c.g[S1][0];
                if c.g[S0][0] == 0 {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x40);
                    break;
                }
                c.g[A0][0] = c.g[S3][0];
                if event_due(c) {
                    deliver_event_here(c, 0x0035_5150);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateTransition_StartFade(void* self, int which)` (0x1946a8):
/// mark one of the two screens (`self + 0x48` for 0, `self + 0x4c` for 1) as
/// fading, play its next frame, and for 0 also kick the audio trigger 0xf.
pub fn cfe_state_transition_start_fade(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        let self_ = abi::arg(c, 0);
        let which = abi::arg64(c, 1);
        frame.save_ra(c, 0);
        if which != 0 {
            c.cycles += 3;
            c.g[V0][0] = 1;
            let not_one = which != 1;
            frame.restore_ra(c, 0);
            let mut reloaded = false;
            if !not_one {
                c.cycles += 3;
                c.g[V1][0] = c.lw(p(self_, 0x4c));
                let empty = c.g[V1][0] == 0;
                c.g[A1][0] = 0;
                if empty {
                    frame.restore_ra(c, 0);
                    reloaded = true;
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw(at(c, V1, 0x54));
                    c.g[A2][0] = 0;
                    c.g[V0][0] = c.g[V0][0] | 0x40;
                    c.sw(at(c, V1, 0x54), c.g[V0][0]);
                    c.g[A0][0] = c.lw(p(self_, 0x4c));
                    call_ptr(c, callee::UI_SCREEN_PLAY_FRAME, 0x0019_4724)?;
                    c.cycles += 1;
                    frame.restore_ra(c, 0);
                    reloaded = true;
                }
            }
            let _ = reloaded;
        } else {
            c.cycles += 3;
            c.g[V1][0] = c.lw(p(self_, 0x48));
            let empty = c.g[V1][0] == 0;
            c.g[A1][0] = 0;
            if empty {
                c.cycles += 1;
                frame.restore_ra(c, 0);
            } else {
                c.cycles += 13;
                c.g[V0][0] = c.lw(at(c, V1, 0x54));
                c.g[A2][0] = 0;
                c.g[V0][0] = c.g[V0][0] | 0x40;
                c.sw(at(c, V1, 0x54), c.g[V0][0]);
                c.g[A0][0] = c.lw(p(self_, 0x48));
                call_ptr(c, callee::UI_SCREEN_PLAY_FRAME, 0x0019_46d8)?;
                call_ptr(c, callee::SSX_AUDIO_SYSTEM_GET_INSTANCE, 0x0019_46e0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0xf;
                call_ptr(c, callee::AUDIO_SET_TRIGGER, 0x0019_46ec)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 3;
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    })
}

/// `void* InstancesMan_AllocByHandle(InstancesMan* self, unsigned id)`
/// (0x3511d0): unlink the free node whose 16-byte slot index matches
/// `id >> 8` from the instance's free list, tag it with the instance's type
/// and return it (0 when there is none).
pub fn instances_man_alloc_by_handle(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A3][0] = c.g[A0][0];
    c.g[A2][0] = c.lw(at(c, A3, 8));
    let empty = c.g[A2][0] == 0;
    c.g[A0][0] = sx(p(r32(c, A3), 8));
    if empty {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    c.g[T0][0] = 0xffff_ffff_ba2e_0000;
    c.g[A1][0] = sx(r32(c, A1) >> 8);
    c.g[T0][0] = c.g[T0][0] | 0x8ba3;
    c.g[V0][0] = c.lw(at(c, A3, 0xc));
    loop {
        c.cycles += 5;
        c.g[V0][0] = sx(r32(c, A2).wrapping_sub(r32(c, V0)));
        c.mult(0, 2, 2, 8);
        c.g[V0][0] = sx(((r32(c, V0) as i32) >> 4) as u32);
        if c.g[V0][0] != c.g[A1][0] {
            c.g[A0][0] = c.g[A2][0];
            c.cycles += 3;
            c.g[A2][0] = c.lw(r32(c, A2));
            if c.g[A2][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = 0;
                abi::ret(c);
                return;
            }
            c.g[V0][0] = c.lw(at(c, A3, 0xc));
            if event_due(c) {
                deliver_event_here(c, 0x0035_11f0);
            }
            continue;
        }
        c.cycles += 17;
        c.g[V1][0] = c.lw(r32(c, A2));
        c.g[V0][0] = c.g[A2][0];
        c.sw(r32(c, A0), c.g[V1][0]);
        c.g[A0][0] = c.lbu(r32(c, A3));
        c.g[A1][0] = c.lw(at(c, A2, 8));
        c.sb(at(c, A2, 0x78), c.g[A0][0]);
        c.g[A1][0] = c.g[A1][0] | 0x2000;
        c.g[V1][0] = c.lw(at(c, A3, 0xc));
        c.g[A0][0] = c.g[A0][0] & 0xff;
        c.sw(at(c, A2, 8), c.g[A1][0]);
        c.g[V1][0] = sx(r32(c, A2).wrapping_sub(r32(c, V1)));
        c.mult(0, 3, 3, 8);
        c.g[V1][0] = sx(((r32(c, V1) as i32) >> 4) as u32);
        c.g[V1][0] = sx(r32(c, V1) << 8);
        c.g[A0][0] = c.g[A0][0] | c.g[V1][0];
        let ra = r32(c, RA);
        c.sw(at(c, A2, 0x78), c.g[A0][0]);
        c.pc = ra;
        return;
    }
}

/// `void cFEStateNetMessenger_OnChildEvent(self, msg, a2)` (0x1a4c58):
/// dispatch a child message by its kind: 0x809/0x80b/0x80c go to the
/// messenger's own virtual handlers (slots 0x10c/0x114/0x11c), anything else
/// to [`NetFEState_OnChildEvent`].
pub fn cfe_state_net_messenger_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        let self_ = abi::arg(c, 0);
        let msg = abi::arg(c, 1);
        let a2 = abi::arg64(c, 2);
        frame.save_ra(c, 0);
        let kind = c.lw(p(msg, 0x18));
        c.g[A3][0] = sx(self_);
        let mut call_default = false;
        // The dispatch: 0x80b skips straight to its slot block (4 cycles),
        // 0x809 costs 3+2 in the dispatch and 3 in its slot block, 0x80c
        // costs 3+3 and 2, and the default paths 3+2+2 or 3+3+2.
        let (disp, target_off) = if kind == 0x80b {
            c.cycles += 4;
            (0x110u32, 0x114u32)
        } else if kind < 0x80c {
            if kind == 0x809 {
                c.cycles += 8;
                (0x108u32, 0x10cu32)
            } else {
                c.cycles += 7;
                call_default = true;
                (0u32, 0u32)
            }
        } else if kind == 0x80c {
            c.cycles += 8;
            (0x118u32, 0x11cu32)
        } else {
            c.cycles += 8;
            call_default = true;
            (0u32, 0u32)
        };
        if call_default {
            c.cycles += 3;
            c.g[A0][0] = c.g[A3][0];
            c.g[A1][0] = sx(msg);
            c.g[A2][0] = a2;
            call_ptr(c, callee::NET_FE_STATE_ON_CHILD_EVENT, 0x001a_4cdc)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw(p(r32(c, A3), 8));
            c.g[A0][0] = c.lh(at(c, V0, disp));
            c.g[V1][0] = c.lw(at(c, V0, target_off));
            c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
            let target = r32(c, V1);
            call_ptr(c, target, 0x001a_4ccc)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void* NetFEState_v26(self, msg)` (0x1a89b0): unless the message's kind is
/// one of the messenger's own (0x31, 0x33, 0x4d, 0x4e) or the FE library is
/// busy, drop the message (virtual `delete(3)`), show the disconnected popup
/// and return 0; otherwise pass the message through.
pub fn net_fe_state_v26(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        let self_ = abi::arg(c, 0);
        let msg = abi::arg(c, 1);
        c.g[A2][0] = sx(msg);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(self_);
        c.g[V0][0] = c.lw(p(self_, 0x6a8));
        let idle = c.g[V0][0] == 0;
        c.g[V1][0] = c.lw(p(msg, 0xc));
        let mut pass = idle;
        let mut busy = false;
        if !idle {
            c.cycles += 3;
            c.g[V0][0] = 0x31;
            let t = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0x33;
            pass = t;
            if !pass {
                c.cycles += 2;
                let t = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0x4d;
                pass = t;
                if !pass {
                    c.cycles += 2;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x4e;
                    pass = t;
                    if !pass {
                        c.cycles += 2;
                        let t = c.g[V1][0] == c.g[V0][0];
                        c.g[V0][0] = c.lw(gp(c, -0xc8));
                        pass = t;
                        if !pass {
                            c.cycles += 3;
                            c.g[V1][0] = c.lw(r32(c, V0));
                            busy = c.g[V1][0] != 0;
                            pass = false;
                            if busy {
                                c.g[V0][0] = c.g[A2][0];
                            } else {
                                // delete msg through its vtable, then the popup
                                c.cycles += 2;
                                c.g[A1][0] = 3;
                                if c.g[A2][0] != 0 {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw(at(c, A2, 8));
                                    c.g[A0][0] = c.lh(at(c, V0, 8));
                                    c.g[V1][0] = c.lw(at(c, V0, 0xc));
                                    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                                    let target = r32(c, V1);
                                    call_ptr(c, target, 0x001a_8a1c)?;
                                }
                                c.cycles += 4;
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(
                                    c,
                                    callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP,
                                    0x001a_8a24,
                                )?;
                                c.g[V0][0] = 0;
                            }
                        }
                    }
                }
            }
        }
        if pass {
            c.cycles += 1;
            c.g[V0][0] = c.g[A2][0];
        }
        let _ = busy;
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateOnlineCharacterSelect_TransitionIn(self, int a1)` (0x1b1068):
/// if entering, show the local character, ask the BE interface for
/// interface 7, and if a remote player is present also show their character;
/// then fill the player info panel and clear the "entering" flag.
pub fn cfe_state_online_character_select_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        let self_ = abi::arg(c, 0);
        let entering = abi::arg64(c, 1);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(self_);
        frame.save_ra(c, 0);
        if entering != 0 {
            c.cycles += 15;
            c.g[A0][0] = c.lw(p(self_, 0x6d0));
            call_ptr(c, callee::CFS_GET_CHARACTER_INFO, 0x001b_1088)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = 0;
            call_ptr(c, callee::CFS_SHOW_CHARACTER, 0x001b_1098)?;
            c.g[V0][0] = c.lw(gp(c, -0x848));
            c.g[A1][0] = 0x7;
            c.g[A0][0] = c.lw(at(c, V0, 0x78));
            call_ptr(c, callee::BE_GET_INTERFACE, 0x001b_10a8)?;
            c.g[V1][0] = 0x0053_0000;
            c.g[S1][0] = sx(p(r32(c, V1), 0x4b30));
            c.g[A1][0] = c.lw(at(c, S1, 0x32c));
            let absent = (c.g[A1][0] as i64) < 0;
            if !absent {
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, callee::CFS_SHOW_OPPONENT_CHAR, 0x001b_10c4)?;
                c.g[A1][0] = c.lw(at(c, S1, 0x32c));
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = 1;
                call_ptr(c, callee::CFS_SHOW_CHARACTER, 0x001b_10d4)?;
            }
        }
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::CFS_FILL_PLAYER_INFO, 0x001b_10dc)?;
        c.sw(at(c, S0, 0x6dc), 0);
        c.cycles += 6;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCreateStep2_SubmitAccount(self, int a1)` (0x1ba9f8): fill in
/// and show the network popup (a "connecting" message), then either hide the
/// lobby overlay or show it.
pub fn cfe_state_create_step2_submit_account(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x40);
        let self_ = abi::arg(c, 0);
        let a1 = abi::arg64(c, 1);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = sx(self_);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[S2][0] = a1;
        call_ptr(c, callee::NET_FE_STATE_GET_POPUP, 0x001b_aa18)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0x0046_0000;
        c.sw(at(c, S0, 0xbc), 0);
        c.g[A0][0] = sx(p(r32(c, A0), 0x4a38));
        call_ptr(c, callee::GET_HASH_VALUE32, 0x001b_aa2c)?;
        c.g[V1][0] = 3;
        c.g[A2][0] = 0x820;
        c.sw(at(c, S0, 0x164), 0);
        c.g[A0][0] = c.g[S1][0];
        c.sw(at(c, S0, 0xc4), c.g[V0][0]);
        c.g[A1][0] = c.g[S0][0];
        c.sw(at(c, S0, 0xc0), c.g[V1][0]);
        c.sw(at(c, S0, 0x150), c.g[A2][0]);
        call_ptr(c, callee::NET_FE_STATE_SHOW_POPUP, 0x001b_aa50)?;
        let hidden = c.g[S2][0] == 0;
        c.g[A0][0] = c.lw(gp(c, -0xc8));
        if hidden {
            c.cycles += 3;
            c.g[A1][0] = sx(p(r32(c, A0), 0x260));
            call_ptr(c, callee::LOBBY_MAN_SHOW, 0x001b_aa70)?;
        } else {
            c.cycles += 4;
            c.g[A1][0] = 0;
            call_ptr(c, callee::LOBBY_MAN_HIDE, 0x001b_aa60)?;
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Popup / screen destructors.

/// `void cFEStateBraggingRights_dtor(self, int flags)` (0x1df4e8): run the
/// destructors of the twelve embedded widgets at `self + 0x6d0` (virtual
/// `dtor(0)`, slot 1 of the vtable at `widget + 0x30`), then the FE state's
/// own destructor.
pub fn cfe_state_bragging_rights_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        let self_ = abi::arg(c, 0);
        let flags = abi::arg64(c, 1);
        c.g[V0][0] = 0x0047_0000;
        frame.save(c, S1, 0x30);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x5098));
        frame.save(c, S3, 0x10);
        c.g[S1][0] = sx(self_);
        frame.save(c, S0, 0x40);
        c.g[V1][0] = sx(p(self_, 0x6d0));
        frame.save(c, S2, 0x20);
        c.g[S3][0] = flags;
        frame.save_ra(c, 0);
        let no_widgets = c.g[V1][0] == 0;
        c.sw(at(c, S1, 8), c.g[V0][0]);
        if !no_widgets {
            c.cycles += 3;
            c.g[S0][0] = sx(p(r32(c, S1), 0x738));
            let empty = c.g[V1][0] == c.g[S0][0];
            c.g[S2][0] = c.g[V1][0];
            if !empty {
                c.cycles += 2;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x34));
                loop {
                    c.cycles += 8;
                    c.g[A1][0] = 0;
                    c.g[V0][0] = c.lw(at(c, S0, 0x30));
                    c.g[A0][0] = c.lh(at(c, V0, 8));
                    c.g[V1][0] = c.lw(at(c, V0, 0xc));
                    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                    let target = r32(c, V1);
                    call_ptr(c, target, 0x001d_f548)?;
                    let more = c.g[S2][0] != c.g[S0][0];
                    c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x34));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x001d_f530);
                    }
                }
            }
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, callee::GAME_MODULE_DTOR, 0x001d_f55c)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenMessageCenter_Shutdown(OVScreenMessageCenter* self)`
/// (0x1e3e30): pop the screen off the message centre's stack in the
/// direction its state says, then destroy the screen itself.
pub fn ov_screen_message_center_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        let self_ = abi::arg(c, 0);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(self_);
        c.g[V1][0] = c.lw(p(self_, 0x18));
        let is_enter = c.g[V1][0] == c.g[V0][0];
        c.g[A0][0] = 7;
        let push;
        if is_enter {
            c.cycles += 2;
            c.g[A1][0] = 0;
            push = true;
        } else {
            c.cycles += 2;
            let zero = c.g[V1][0] == 0;
            c.g[A1][0] = 1;
            push = zero;
        }
        if push {
            c.cycles += 18;
            call_ptr(c, callee::MESSAGE_CENTER_SET_FLAG, 0x001e_3e68)?;
            call_ptr(c, callee::OVSCREEN_PDA_TEMPLATE_CLOSE, 0x001e_3e70)?;
            c.g[V0][0] = c.lw(gp(c, -0x69c));
            c.g[A0][0] = 0x0053_0000;
            c.g[A0][0] = sx(p(r32(c, A0), 0x66e8));
            c.g[A1][0] = 0x0044_0000;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            c.g[A1][0] = sx(p(r32(c, A1), 0x28f0));
            c.g[V1][0] = sx(r32(c, V0) << 2);
            c.sw(gp(c, -0x69c), c.g[V0][0]);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
            c.g[V0][0] = c.lw(r32(c, V1));
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
            c.g[V1][0] = c.lw(r32(c, V0));
            c.sw(gp(c, -0x6a0), c.g[V1][0]);
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::OVSCREEN_MESSAGE_CENTER_DTOR, 0x001e_3eb0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cUITextScroll_dtor(cUITextScroll* self, int flags)` (0x3a62a0):
/// reset the vtable slots, drop the scroll lines and both list nodes, then
/// run the UI object destructor.
pub fn c_ui_text_scroll_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x50);
        let self_ = abi::arg(c, 0);
        let flags = abi::arg64(c, 1);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S1, 0x30);
        c.g[V0][0] = sx(p(r32(c, V0), 0x4278));
        c.g[S1][0] = sx(self_);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = sx(p(self_, 0x8c));
        frame.save_ra(c, 0);
        c.g[S3][0] = flags;
        c.sw(at(c, S1, 8), c.g[V0][0]);
        call_ptr(c, callee::CUI_TEXT_SCROLL_CLEAR_LINES, 0x003a_62d4)?;
        c.g[V0][0] = 0x0049_0000;
        c.g[A0][0] = c.g[S2][0];
        c.g[V0][0] = sx(p(r32(c, V0), 0x4c98));
        c.sw(at(c, S1, 0xa4), c.g[V0][0]);
        call_ptr(c, callee::UI_LIST_DELETE_ALL, 0x003a_62e8)?;
        c.g[S0][0] = 0x0049_0000;
        c.g[A0][0] = sx(p(r32(c, S1), 0x98));
        c.g[S0][0] = sx(p(r32(c, S0), 0x4cc0));
        c.sw(at(c, S1, 0xa0), c.g[S0][0]);
        call_ptr(c, callee::UI_LIST_NODE_DTOR_BODY, 0x003a_62fc)?;
        c.sw(at(c, S1, 0x94), c.g[S0][0]);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, callee::UI_LIST_NODE_DTOR_BODY, 0x003a_6308)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, callee::UI_OBJECT_DTOR, 0x003a_6314)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Buddy and script engine lookups.

/// `int BuddyMan_FindRosterEntry(BuddyMan* self, void* who)` (0x262468): index
/// of the roster entry whose name matches `who`, or -1.
pub fn buddy_man_find_roster_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        let self_ = abi::arg(c, 0);
        let who = abi::arg64(c, 1);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = sx(self_);
        frame.save(c, S0, 0x40);
        c.g[S3][0] = who;
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        c.g[S0][0] = 0;
        call_ptr(c, callee::BUDDY_MAN_GET_ROSTER_COUNT, 0x0026_2490)?;
        c.g[S1][0] = c.g[V0][0];
        let none = (c.g[S1][0] as i64) <= 0;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        if !none {
            c.cycles += 1;
            c.g[A0][0] = c.g[S2][0];
            loop {
                c.cycles += 4;
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, callee::BUDDY_MAN_GET_ROSTER_ENTRY, 0x0026_24a8)?;
                let missing = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[V0][0];
                let mut found = false;
                if missing {
                    c.cycles += 1;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                } else {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S3][0];
                    call_ptr(c, callee::STRCMP, 0x0026_24b8)?;
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = c.g[S0][0];
                        found = true;
                    }
                }
                if found {
                    break;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S2][0];
                if !more {
                    c.cycles += 1;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_24a0);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptEngine_OnDataRemoved(self, obj, a2, type, id)` (0x2799e0):
/// release the script object's binding for `type` 0, the anim bank for 1 and
/// the sound bank for 2.
pub fn script_engine_on_data_removed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        let self_ = abi::arg(c, 0);
        let obj = abi::arg(c, 1);
        let kind = abi::arg64(c, 3);
        let id = abi::arg64(c, 4);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x10);
        c.g[S0][0] = sx(self_);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(obj);
        let is_one = kind == c.g[V0][0];
        c.g[A1][0] = id;
        if is_one {
            c.cycles += 4;
            c.g[A0][0] = sx(p(r32(c, S0), 0xa28));
            c.g[A1][0] = id;
            call_ptr(c, callee::SCRIPT_ANIM_BANK_MAN_REMOVE_BANK, 0x0027_9a50)?;
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((kind as i64) < 2) as u64;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 2;
                c.cycles += 2;
                if kind == c.g[V0][0] {
                    c.cycles += 3;
                    c.g[A0][0] = sx(p(r32(c, S0), 0xa38));
                    c.g[A1][0] = id;
                    call_ptr(c, callee::SCRIPT_SOUND_BANK_MAN_REMOVE_BANK, 0x0027_9a60)?;
                } else {
                    c.cycles += 2;
                }
            } else {
                c.cycles += 2;
                if kind == 0 {
                    c.cycles += 7;
                    call_ptr(c, callee::SCRIPT_DATA_GET_BANK_LIST, 0x0027_9a34)?;
                    c.g[A1][0] = c.lw(at(c, V0, 0x18));
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, callee::SCRIPT_ENGINE_UNREGISTER_SCRIPT, 0x0027_9a40)?;
                } else {
                    c.cycles += 2;
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

/// `void ScriptViewObj_Activate(ScriptViewObj* self, int seq)` (0x281370):
/// bind the object to the sequence, reset it, and if the sequence is playing
/// tell the view object that it was activated.
pub fn script_view_obj_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        let self_ = abi::arg(c, 0);
        let seq = abi::arg64(c, 1);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(self_);
        frame.save_ra(c, 0);
        c.g[S1][0] = seq;
        call_ptr(c, callee::SCRIPT_ENGINE_GET_INSTANCE, 0x0028_138c)?;
        c.g[A1][0] = c.lw(at(c, S0, 4));
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S1][0];
        call_ptr(c, callee::SCRIPT_ENGINE_SET_OBJ_BINDING_FILTER, 0x0028_139c)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::SCRIPT_VIEW_OBJ_RESET, 0x0028_13a4)?;
        c.g[A0][0] = c.lw(gp(c, -0x84c));
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE, 0x0028_13b0)?;
        c.g[A0][0] = c.lw(gp(c, -0x84c));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, callee::SCRIPT_ENGINE_IS_PLAYING_SCRIPT, 0x0028_13bc)?;
        let stopped = c.g[V0][0] == 0;
        c.g[A0][0] = c.lw(gp(c, -0x848));
        if !stopped {
            c.cycles += 7;
            c.g[A1][0] = sx(p(r32(c, S0), 0x100));
            c.g[V0][0] = c.lw(at(c, A0, 0x84));
            c.g[V1][0] = c.lw(at(c, V0, 0xc));
            c.g[A0][0] = c.lw(at(c, V1, 0xa4));
            call_ptr(c, callee::SCRIPT_VIEW_OBJ_ON_ACTIVATED, 0x0028_13d8)?;
            c.g[V0][0] = 1;
            c.sw(at(c, S0, 0x110), c.g[V0][0]);
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, callee::SCRIPT_OBJ_ACTIVATE, 0x0028_13ec)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Highlight* Replay_FindWeakestHighlight(Replay* self, int i, Highlight*
/// best, int positive)` (0x270c58): scan the highlight list for the entry
/// with the weakest value in column `i` (`positive`: the strongest).
pub fn replay_find_weakest_highlight(c: &mut Cpu) {
    c.cycles += 3;
    c.g[T0][0] = c.lw(p(r32(c, A0), 0x3b0));
    let empty = c.g[T0][0] == 0;
    c.g[T1][0] = c.g[A2][0];
    if !empty {
        c.cycles += 1;
        c.g[V0][0] = c.lbu(at(c, T0, 0x1e));
        loop {
            c.cycles += 4;
            let shift = r32(c, A1) & 31;
            c.g[V0][0] = sx(((r32(c, V0) as i32) >> shift) as u32);
            c.g[V0][0] = c.g[V0][0] & 1;
            if c.g[V0][0] == 0 {
                c.g[T0][0] = c.lw(at(c, T0, 0x14));
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lbu(at(c, T0, 0x1f));
                if c.g[V0][0] != 0 {
                    c.g[T0][0] = c.lw(at(c, T0, 0x14));
                } else {
                    // Take the element or skip it: `land` is where the
                    // original's `break` goes (1: re-test V0, 2: skip).
                    let next = c.lw(at(c, T0, 0x14));
                    c.cycles += 2;
                    let negative = c.g[A3][0] == 0;
                    c.g[V1][0] = sx(r32(c, A1) << 2);
                    let mut land = 0u8;
                    if negative {
                        c.cycles += 4;
                        c.g[V0][0] = sx(r32(c, T0).wrapping_add(r32(c, V1)));
                        c.g[A0][0] = c.lw(at(c, V0, 0x20));
                        if (c.g[A0][0] as i64) >= 0 {
                            land = 2;
                        } else {
                            c.cycles += 3;
                            c.g[V1][0] = sx(r32(c, A2).wrapping_add(r32(c, V1)));
                            c.g[V0][0] = c.lw(at(c, V1, 0x20));
                            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            land = 1;
                        }
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = sx(r32(c, T0).wrapping_add(r32(c, V1)));
                        c.g[A0][0] = c.lw(at(c, V0, 0x20));
                        let not_positive = (c.g[A0][0] as i64) <= 0;
                        c.g[V1][0] = sx(r32(c, A2).wrapping_add(r32(c, V1)));
                        if not_positive {
                            land = 2;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(at(c, V1, 0x20));
                            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                            land = 1;
                        }
                    }
                    if land == 1 {
                        c.cycles += 2;
                        if c.g[V0][0] != 0 {
                            c.g[T0][0] = next;
                        } else {
                            c.cycles += 2;
                            c.g[T1][0] = c.g[T0][0];
                            c.g[A2][0] = c.g[T1][0];
                            c.cycles += 1;
                            c.g[T0][0] = next;
                        }
                    } else if land == 2 {
                        c.cycles += 1;
                        c.g[T0][0] = next;
                    } else {
                        c.g[T0][0] = next;
                    }
                }
            }
            c.cycles += 2;
            if c.g[T0][0] == 0 {
                break;
            }
            c.g[V0][0] = c.lbu(at(c, T0, 0x1e));
            if event_due(c) {
                deliver_event_here(c, 0x0027_0c68);
            }
        }
    }
    c.cycles += 2;
    let ra = r32(c, RA);
    c.g[V0][0] = c.g[T1][0];
    c.pc = ra;
}

/// `float RiderSpinSub_GetFlipHUDBlend(SpinSub* self)` (0x136168): blend of
/// the rider's spin angle folded into a quarter turn, then scaled.
pub fn rider_spin_sub_get_flip_hud_blend(c: &mut Cpu) {
    c.cycles += 11;
    c.lwc1(3, at(c, A0, 0x1c));
    c.lwc1(0, gp(c, -0x72f8));
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(AT, 1);
    c.mul_s(0, 3, 0);
    c.add_s(0, 0, 1);
    c.cvt_w_s(1, 0);
    c.cvt_s_w(1, 1);
    c.c_lt_s(0, 1);
    if !c.fcc() {
        c.cycles += 3;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.sub_s(1, 1, 0);
    }
    c.cycles += 11;
    c.lwc1(0, gp(c, -0x72f4));
    c.lwc1(2, gp(c, -0x72f0));
    c.mul_s(0, 1, 0);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 1);
    c.sub_s(0, 3, 0);
    c.abs_s(0, 0);
    c.mul_s(2, 0, 2);
    c.c_lt_s(1, 2);
    if !c.fcc() {
        c.cycles += 3;
        c.g[AT][0] = 0x4000_0000;
        c.mtc1(AT, 0);
        c.sub_s(2, 2, 0);
    }
    c.cycles += 8;
    c.g[AT][0] = 0x3e80_0000;
    c.mtc1(AT, 0);
    c.mtc1(ZERO, 1);
    c.sub_s(2, 2, 0);
    c.lwc1(0, gp(c, -0x72ec));
    c.max_s(2, 2, 1);
    let ra = r32(c, RA);
    c.mul_s(0, 2, 0);
    c.pc = ra;
}

/// `void* RailTube_ctor(RailTube* self, void* owner)` (0x372520): set the
/// vtable and owner; the first time, fill the 262-entry rail-vertex ring
/// with the default rail colours.
pub fn rail_tube_ctor(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = 0x0049_0000;
    c.g[V1][0] = c.lw(gp(c, 0x113c));
    c.g[V0][0] = sx(p(r32(c, V0), 0x3068));
    c.sw(at(c, A0, 4), c.g[A1][0]);
    c.sw(r32(c, A0), c.g[V0][0]);
    let ready = c.g[V1][0] != 0;
    c.sw(at(c, A0, 8), 0);
    if !ready {
        c.cycles += 9;
        c.g[V0][0] = 0x0050_0000;
        c.g[T0][0] = c.lw(gp(c, 0x10f0));
        c.g[A3][0] = c.lw(gp(c, 0x10f4));
        c.g[V1][0] = sx(p(r32(c, V0), 0x1440));
        c.g[A2][0] = c.lw(gp(c, 0x10f8));
        c.g[A1][0] = 0;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.g[T1][0] = 0xff;
        loop {
            c.cycles += 9;
            c.sw(at(c, V1, 0x1c), c.g[T1][0]);
            c.g[V0][0] = c.g[A1][0] & 1;
            c.sw(at(c, V1, 0x10), c.g[T0][0]);
            c.sw(at(c, V1, 0x14), c.g[A3][0]);
            c.sw(at(c, V1, 0x18), c.g[A2][0]);
            c.swc1(0, at(c, V1, 8));
            c.sw(at(c, V1, 0xc), 0);
            let even = c.g[V0][0] == 0;
            c.sw(r32(c, V1), 0);
            if even {
                c.cycles += 1;
                c.sw(at(c, V1, 4), 0);
            } else {
                c.cycles += 2;
                c.swc1(0, at(c, V1, 4));
            }
            c.cycles += 4;
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
            c.g[V0][0] = ((c.g[A1][0] as i64) < 262) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx(at(c, V1, 0x30));
            if !more {
                c.cycles += 2;
                c.g[V0][0] = 1;
                c.sw(gp(c, 0x113c), c.g[V0][0]);
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0037_2560);
            }
        }
    }
    c.cycles += 2;
    let ra = r32(c, RA);
    c.g[V0][0] = c.g[A0][0];
    c.pc = ra;
}

/// `void Irradiance_AddScaled(Vec4* a, const Vec4* b, const Vec4* c, float s)`
/// (0x389590): `a[i] += b[i] * (s * c)` with the components rotated
/// (x by s*c.y, y by s*c.z, z by s*c.w, w by s*c.x) for ten vertices.
pub fn irradiance_add_scaled(c: &mut Cpu) {
    c.cycles += 10;
    c.lwc1(1, at(c, A2, 0xc));
    c.g[V0][0] = 9;
    c.lwc1(2, r32(c, A2));
    c.mul_s(7, 12, 1);
    c.lwc1(0, at(c, A2, 4));
    c.lwc1(1, at(c, A2, 8));
    c.mul_s(6, 12, 2);
    c.mul_s(5, 12, 0);
    c.mul_s(12, 12, 1);
    loop {
        c.cycles += 24;
        c.lwc1(0, r32(c, A1));
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        c.lwc1(1, r32(c, A0));
        c.mul_s(0, 0, 5);
        c.lwc1(2, at(c, A0, 4));
        c.lwc1(3, at(c, A0, 8));
        c.lwc1(4, at(c, A0, 0xc));
        c.add_s(1, 1, 0);
        c.swc1(1, r32(c, A0));
        c.lwc1(0, at(c, A1, 4));
        c.mul_s(0, 0, 12);
        c.add_s(2, 2, 0);
        c.swc1(2, at(c, A0, 4));
        c.lwc1(0, at(c, A1, 8));
        c.mul_s(0, 0, 7);
        c.add_s(3, 3, 0);
        c.swc1(3, at(c, A0, 8));
        c.lwc1(0, at(c, A1, 0xc));
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x10));
        c.mul_s(0, 0, 6);
        c.add_s(4, 4, 0);
        c.swc1(4, at(c, A0, 0xc));
        let more = (c.g[V0][0] as i64) >= 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
        if !more {
            c.cycles += 2;
            let ra = r32(c, RA);
            c.pc = ra;
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0038_95b8);
        }
    }
}

// ---------------------------------------------------------------------------
// Movie player and misc state hooks.

/// `void MCManager_State25(MCManager* self)` (0x23fe70): poll the current
/// operation object; when it is finished, end the operation (or report it
/// done), otherwise call the pending callback.
pub fn mc_manager_state25(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        let self_ = abi::arg(c, 0);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(self_);
        c.g[A1][0] = c.lw(p(self_, 0x434));
        c.g[V1][0] = c.lw(r32(c, A1));
        c.g[A0][0] = c.lh(at(c, V1, 0x58));
        c.g[V0][0] = c.lw(at(c, V1, 0x5c));
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        let target = r32(c, V0);
        call_ptr(c, target, 0x0023_fe98)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x10);
        } else {
            let mut done = false;
            c.cycles += 5;
            c.g[V0][0] = c.lw(at(c, S0, 0x6c));
            let target = r32(c, V0);
            call_ptr(c, target, 0x0023_feac)?;
            let idle = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if idle {
                c.cycles += 5;
                c.g[V0][0] = c.lw(at(c, S0, 0x84));
                let target = r32(c, V0);
                call_ptr(c, target, 0x0023_fed0)?;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = c.lw(at(c, S0, 0x68));
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0023_ea90, 0x0023_fee0)?;
                    done = true;
                }
            } else {
                c.cycles += 4;
                c.g[A1][0] = 1;
                call_ptr(c, 0x0023_ea30, 0x0023_febc)?;
                done = true;
            }
            if done {
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 3;
                let target = r32(c, V0);
                call_ptr(c, target, 0x0023_fef0)?;
                frame.restore(c, S0, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cWScriptCache_dtor(cWScriptCache* self, int flags)` (0x3ac8f0):
/// destroy the script nodes of the cache backwards, free the node array,
/// clear the global cache and, with bit 0 of `flags`, delete the object.
pub fn c_w_script_cache_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        let self_ = abi::arg(c, 0);
        let flags = abi::arg64(c, 1);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = sx(self_);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = flags;
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw(at(c, S1, 4));
        let empty = c.g[A1][0] == 0;
        c.g[V0][0] = 0xc;
        if !empty {
            c.cycles += 5;
            c.g[V1][0] = c.lw(r32(c, A1).wrapping_sub(0x10));
            c.mult(0, 4, 3, 2);
            c.g[S0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
            if c.g[A1][0] == c.g[S0][0] {
                c.g[A0][0] = c.lw(at(c, S1, 4));
            } else {
                c.cycles += 1;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0xc));
                loop {
                    c.cycles += 6;
                    c.g[A1][0] = 0;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, callee::WSCRIPT_CACHE_NODE_DTOR, 0x003a_c93c)?;
                    c.g[V0][0] = c.lw(at(c, S1, 4));
                    let more = c.g[V0][0] != c.g[S0][0];
                    c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0xc));
                    if !more {
                        c.cycles += 1;
                        c.g[A0][0] = c.lw(at(c, S1, 4));
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003a_c930);
                    }
                }
            }
            c.cycles += 2;
            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x10));
            call_ptr(c, callee::OPERATOR_DELETE, 0x003a_c954)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S2][0] & 1;
        let no_free = c.g[V0][0] == 0;
        c.sw(gp(c, 0x1720), 0);
        if !no_free {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, callee::OPERATOR_DELETE, 0x003a_c968)?;
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

/// `void cFEStateProgressive_Init(self)` (0x245248): add the state's screen to
/// the UI engine by hash name, play it, then set the instruction text.
pub fn c_fe_state_progressive_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        let self_ = abi::arg(c, 0);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = sx(self_);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x004a_0000;
        c.g[A0][0] = sx(p(r32(c, A0), 0x2ad8));
        c.g[S0][0] = c.lw(at(c, S1, 0x10));
        call_ptr(c, callee::GET_HASH_VALUE32, 0x0024_526c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME, 0x0024_5280)?;
        let no_screen = c.g[V0][0] == 0;
        c.sw(at(c, S1, 0x40), c.g[V0][0]);
        if !no_screen {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, callee::UI_SCREEN_PLAY_FRAME, 0x0024_5298)?;
        }
        c.cycles += 8;
        c.g[A0][0] = 0x004a_0000;
        c.g[A0][0] = sx(p(r32(c, A0), 0x2ac0));
        call_ptr(c, callee::GET_HASH_VALUE32, 0x0024_52a4)?;
        c.g[A0][0] = c.lw(at(c, S1, 0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, callee::UI_SCREEN_FIND_OBJECT, 0x0024_52b0)?;
        let no_text = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[V0][0];
        if !no_text {
            c.cycles += 3;
            c.g[A1][0] = 0x0048_0000;
            c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x4678));
            call_ptr(c, callee::UI_TEXT_SET_ASCII_STRING, 0x0024_52c4)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int DebugMenu_NextEnabledItem(DebugMenu* self, int i)` (0x17a028): the
/// next entry index after `i` that is enabled, -1 when the menu is off.
pub fn debug_menu_next_enabled_item(c: &mut Cpu) {
    c.cycles += 6;
    c.g[A3][0] = c.g[A0][0];
    c.g[V0][0] = 0x8000;
    c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
    c.g[V1][0] = c.lw(at(c, V0, 0xc08));
    let present = c.g[V1][0] != 0;
    c.g[A0][0] = c.g[A1][0];
    if !present {
        c.cycles += 2;
        let ra = r32(c, RA);
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        c.pc = ra;
        return;
    }
    c.cycles += 7;
    c.g[A2][0] = c.lw(r32(c, A3));
    c.g[V0][0] = sx(r32(c, A0).wrapping_add(1));
    c.g[V1][0] = 0x8c;
    c.g[A3][0] = sx(r32(c, A3).wrapping_add(0x8c));
    c.div(0, 2, 6);
    c.cycles += 6;
    c.g[A0][0] = c.hi[0];
    c.mult(0, 3, 4, 3);
    c.g[V1][0] = sx(r32(c, A3).wrapping_add(r32(c, V1)));
    c.g[V0][0] = c.lw(r32(c, V1));
    let enabled = c.g[V0][0] != 0;
    c.g[A1][0] = c.g[A2][0];
    if !enabled {
        c.cycles += 3;
        c.g[T0][0] = 0;
        c.g[A2][0] = 0x8c;
        c.g[V1][0] = sx(r32(c, A0).wrapping_add(1));
        loop {
            c.cycles += 2;
            c.cycles += 8;
            c.div(0, 3, 5);
            c.g[V0][0] = c.hi[0];
            c.g[A0][0] = c.g[V0][0];
            c.mult(0, 2, 4, 6);
            c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, A3)));
            c.g[V0][0] = c.lw(r32(c, V1));
            let empty = c.g[V0][0] == 0;
            c.g[V1][0] = sx(r32(c, A0).wrapping_add(1));
            if !empty {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0017_a088);
            }
        }
    }
    c.cycles += 2;
    let ra = r32(c, RA);
    c.g[V0][0] = c.g[A0][0];
    c.pc = ra;
}

/// `void cFEStatePreviewReward_Init(self)` (0x1d1c08): add the reward preview
/// screen by hash name, play it, then flag the preview object.
pub fn c_fe_state_preview_reward_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        let self_ = abi::arg(c, 0);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = sx(self_);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx(p(r32(c, A0), 0x7140));
        c.g[S0][0] = c.lw(at(c, S1, 0x10));
        call_ptr(c, callee::GET_HASH_VALUE32, 0x001d_1c2c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME, 0x001d_1c40)?;
        let no_screen = c.g[V0][0] == 0;
        c.sw(at(c, S1, 0x40), c.g[V0][0]);
        if !no_screen {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, callee::UI_SCREEN_PLAY_FRAME, 0x001d_1c58)?;
        }
        c.cycles += 9;
        c.g[A0][0] = 0x004a_0000;
        c.g[A0][0] = sx(p(r32(c, A0), 0x1398));
        call_ptr(c, callee::GET_HASH_VALUE32, 0x001d_1c64)?;
        c.g[A0][0] = c.lw(at(c, S1, 0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, callee::UI_SCREEN_FIND_OBJECT, 0x001d_1c70)?;
        c.g[V1][0] = c.g[V0][0];
        let missing = c.g[V1][0] == 0;
        frame.restore(c, S0, 0x20);
        if !missing {
            c.cycles += 3;
            c.g[V0][0] = c.lw(at(c, V1, 0x90));
            c.g[V0][0] = c.g[V0][0] | 8;
            c.sw(at(c, V1, 0x90), c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cFEStateVideoPlayer_ctor(self, engine, owner, signed char player)`
/// (0x1d22f0): build the video player state's UI object and two strings.
pub fn c_fe_state_video_player_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x40);
        let self_ = abi::arg(c, 0);
        let engine = abi::arg64(c, 1);
        let owner = abi::arg64(c, 2);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = sx(self_);
        c.g[S1][0] = sx(r32(c, A3) << 24);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(((r32(c, S1) as i32) >> 24) as u32);
        c.g[S2][0] = sx(p(self_, 0x48));
        c.g[A1][0] = engine;
        c.g[A2][0] = owner;
        call_ptr(c, callee::UI_OBJECT_CTOR_2, 0x001d_2318)?;
        c.g[V0][0] = 0x0047_0000;
        c.sb(at(c, S0, 0x44), c.g[S1][0]);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x66d8));
        c.g[A1][0] = 6;
        c.g[V1][0] = 1;
        c.sb(at(c, S0, 0x48), 0);
        c.sb(at(c, S0, 0x148), 0);
        c.g[A0][0] = sx(p(r32(c, S0), 0x24c));
        c.sw(at(c, S0, 0x274), 0);
        c.g[S1][0] = sx(p(r32(c, S0), 0x148));
        c.sw(at(c, S0, 0x278), 0);
        c.sw(at(c, S0, 0x27c), 0);
        c.sw(at(c, S0, 0x248), 0);
        c.sw(at(c, S0, 8), c.g[V0][0]);
        c.sw(at(c, S0, 0xc), c.g[A1][0]);
        c.sw(at(c, S0, 0x270), c.g[V1][0]);
        call_ptr(c, callee::UI_STRING_CTOR, 0x001d_235c)?;
        c.sw(at(c, S0, 0x24c), c.g[S2][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sw(at(c, S0, 0x250), c.g[S1][0]);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int BoltFile_FindSlotByName(EquipTable* self, int bank, const char* name)`
/// (0x19d428): index of the named slot in the bank's 256-entry table, or -1.
pub fn bolt_file_find_slot_by_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        let self_ = abi::arg(c, 0);
        let bank = abi::arg64(c, 1);
        let name = abi::arg64(c, 2);
        c.g[V0][0] = 0x1_0000;
        c.g[V0][0] = c.g[V0][0] | 0x2000;
        frame.save(c, S0, 0x40);
        c.mult(0, 5, 5, 2);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S3][0] = name;
        c.g[V0][0] = sx(r32(c, A1).wrapping_add(4));
        c.g[S1][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
        c.g[S0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lb(at(c, S1, 4));
            let empty = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            let mut found = false;
            if empty {
                c.cycles += 1;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
            } else {
                c.cycles += 4;
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, callee::STRCMP, 0x0019_d474)?;
                if c.g[V0][0] != 0 {
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S2][0];
                    found = true;
                }
            }
            if found {
                break;
            }
            c.cycles += 4;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x120));
            c.g[V0][0] = ((c.g[S2][0] as i64) < 256) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x120));
            if !more {
                c.cycles += 1;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_d460);
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_UpdateMusicTier(self, int tier)` (0x28d5a0): when the
/// music system is not in state 0x17, play the track for the given tier if it
/// differs from the one already playing.
pub fn ssx_audio_system_update_music_tier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        let self_ = abi::arg(c, 0);
        let tier = abi::arg64(c, 1);
        c.g[V0][0] = 0x17;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = sx(self_);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.lw(at(c, S0, 0x6284));
        let idle = c.g[V1][0] == c.g[V0][0];
        c.g[S1][0] = tier;
        if !idle {
            c.cycles += 10;
            c.g[A0][0] = sx(p(r32(c, S0), 0x118));
            call_ptr(c, callee::AUDIO_GET_MUSIC_TIER, 0x0028_d5d0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[S2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, callee::AUDIO_GET_TRACK_TYPE, 0x0028_d5e0)?;
            c.g[A3][0] = c.g[V0][0];
            c.g[V0][0] = 1;
            let one = c.g[A3][0] == c.g[V0][0];
            c.g[A0][0] = 0x65;
            if !one {
                c.cycles += 4;
                c.g[A0][0] = 0x67;
                c.g[V1][0] = 0x66;
                c.g[V0][0] = c.g[A3][0] ^ 2;
                if c.g[V0][0] == 0 {
                    c.g[A0][0] = c.g[V1][0];
                }
            }
            c.cycles += 2;
            let same = c.g[A0][0] == c.g[S2][0];
            c.g[A0][0] = c.g[S0][0];
            if !same {
                c.cycles += 4;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                c.g[T0][0] = 0;
                call_ptr(c, callee::AUDIO_PLAY_MUSIC, 0x0028_d618)?;
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

// ---------------------------------------------------------------------------
// Script engine teardown.

/// `void cFEStateOptions_OpenResetPopup(self)` (0x188e58): allocate and build
/// the reset-options popup screen object, add it to the engine's screen list
/// and arm the global "reset" flags.
pub fn c_fe_state_options_open_reset_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x30);
        let self_ = abi::arg(c, 0);
        c.g[A1][0] = 0x0046_0000;
        frame.save(c, S1, 0x10);
        c.g[A3][0] = 0;
        c.g[S1][0] = sx(self_);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x2298));
        c.g[A0][0] = 0x4c;
        c.g[A2][0] = 0x100;
        call_ptr(c, callee::MEM_ALLOC, 0x0018_8e84)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.lw(at(c, S1, 0x10));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        call_ptr(c, callee::UI_OBJECT_CTOR_2, 0x0018_8e98)?;
        c.g[A0][0] = c.lw(at(c, S1, 0x10));
        c.g[V0][0] = 0x0047_0000;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x4c30));
        c.g[A1][0] = c.g[S0][0];
        c.sw(at(c, S0, 8), c.g[V0][0]);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
        c.sb(at(c, S0, 0x48), 0);
        call_ptr(c, callee::UI_OBJECT_ADD_CHILD, 0x0018_8eb8)?;
        c.g[V0][0] = 1;
        c.sw(gp(c, -0x1c18), 0);
        c.sw(gp(c, -0x1c10), c.g[V0][0]);
        c.sw(gp(c, -0x1c14), 0);
        c.sw(gp(c, -0x1c1c), 0);
        c.sw(gp(c, -0x1c20), 0);
        c.sw(at(c, S1, 0x4c), 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptEngine_DestroyScriptDatabase(ScriptEngine* self)` (0x282020):
/// stop and kill everything, drop the script object list, the process table
/// and the sequence bank, and reset the process table.
pub fn script_engine_destroy_script_database(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        let self_ = abi::arg(c, 0);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(self_);
        c.g[V0][0] = c.lw(at(c, S0, 0x2b0));
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 9;
            call_ptr(c, callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS, 0x0028_2044)?;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::SCRIPT_ENGINE_KILL_ALL_PROCESSES, 0x0028_204c)?;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS, 0x0028_2054)?;
            c.g[A0][0] = c.lw(at(c, S0, 0x2b0));
            if c.g[A0][0] == 0 {
                c.g[A0][0] = c.lw(at(c, S0, 0x2b8));
            } else {
                c.cycles += 3;
                call_ptr(c, callee::OPERATOR_DELETE, 0x0028_2068)?;
                c.g[A0][0] = c.lw(at(c, S0, 0x2b8));
            }
            c.cycles += 3;
            c.sw(at(c, S0, 0x2b0), 0);
            let no_seq = c.g[A0][0] == 0;
            c.sw(at(c, S0, 0x2b4), 0);
            if !no_seq {
                c.cycles += 2;
                c.g[A1][0] = 3;
                call_ptr(c, callee::IN_SEQ_DTOR, 0x0028_2080)?;
            }
            c.cycles += 3;
            c.g[A0][0] = c.lw(at(c, S0, 0x2bc));
            let no_bank = c.g[A0][0] == 0;
            c.sw(at(c, S0, 0x2b8), 0);
            if !no_bank {
                c.cycles += 2;
                call_ptr(c, callee::OPERATOR_DELETE, 0x0028_2094)?;
            }
            c.cycles += 4;
            c.sw(at(c, S0, 0x2bc), 0);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE, 0x0028_20a0)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Sphere tree collision.

/// `int SphereTree_CollideSphere(self, const Sphere* s, float f, void* a2,
/// void* a3)` (0x32c898): refresh the tree's node offset, then test a sphere
/// against the node at `self + 0x80`.
pub fn sphere_tree_collide_sphere(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        let self_ = abi::arg(c, 0);
        let a1 = abi::arg64(c, 1);
        let a2 = abi::arg64(c, 2);
        let a3 = abi::arg64(c, 3);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = sx(self_);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = a2;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = a3;
        frame.save_fpr(c, 20, 0x50);
        c.g[S3][0] = a1;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(at(c, S0, 0x98));
        c.g[V0][0] = c.lw(at(c, A0, 8));
        let no_offset = c.g[V0][0] == 0;
        c.mov_s(20, 12);
        if !no_offset {
            c.cycles += 4;
            call_ptr(c, callee::SPHERE_TREE_COMPUTE_OFFSET, 0x0032_c8dc)?;
            c.g[V1][0] = c.lw(at(c, S0, 0x98));
            c.sw(at(c, V1, 0x28), c.g[V0][0]);
        }
        c.cycles += 17;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S3][0];
        c.mov_s(12, 20);
        c.g[T1][0] = c.g[S1][0];
        c.g[T2][0] = c.g[S2][0];
        c.g[A2][0] = sx(p(r32(c, A0), 0x80));
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        call_ptr(c, callee::SPHERE_TREE_COLLIDE_SPHERE_NODE, 0x0032_c908)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x50);
        frame.close(c);
        Ok(())
    })
}

/// `int SphereTree_CollideAABB(self, const Box* b, void* a2, void* a3,
/// void* a4)` (0x32c928): refresh the tree's node offset, then test an AABB
/// against the node at `self + 0x80`.
pub fn sphere_tree_collide_aabb(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        let self_ = abi::arg(c, 0);
        let a1 = abi::arg64(c, 1);
        let a2 = abi::arg64(c, 2);
        let a3 = abi::arg64(c, 3);
        let a4 = abi::arg64(c, 4);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = sx(self_);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = a2;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = a3;
        frame.save(c, S4, 0x10);
        c.g[S3][0] = a4;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(at(c, S0, 0x98));
        c.g[V0][0] = c.lw(at(c, A0, 8));
        let no_offset = c.g[V0][0] == 0;
        c.g[S4][0] = a1;
        if !no_offset {
            c.cycles += 4;
            call_ptr(c, callee::SPHERE_TREE_COMPUTE_OFFSET, 0x0032_c96c)?;
            c.g[V1][0] = c.lw(at(c, S0, 0x98));
            c.sw(at(c, V1, 0x28), c.g[V0][0]);
        }
        c.cycles += 17;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S4][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[T2][0] = c.g[S2][0];
        c.g[T3][0] = c.g[S3][0];
        c.g[A3][0] = sx(p(r32(c, A0), 0x80));
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        call_ptr(c, callee::SPHERE_TREE_COLLIDE_AABB_NODE, 0x0032_c998)?;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int CollSphere_CollideSphereTree(self, SphereTree* tree, Hit* hit)`
/// (0x32fdb0): test the collider's sphere against the tree and, on a hit,
/// normalise the hit normal and return its length.
pub fn coll_sphere_collide_sphere_tree(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        let self_ = abi::arg(c, 0);
        let tree = abi::arg64(c, 1);
        let hit = abi::arg64(c, 2);
        c.g[V0][0] = c.g[A0][0];
        frame.save(c, S0, 0x10);
        c.g[A0][0] = tree;
        frame.save_ra(c, 0);
        c.g[S0][0] = hit;
        c.g[A1][0] = sx(p(self_, 0x60));
        c.g[A2][0] = sx(p(r32(c, S0), 0x10));
        c.lwc1(12, at(c, V0, 0x70));
        c.g[A3][0] = c.g[S0][0];
        call_ptr(c, callee::SPHERE_TREE_COLLIDE_SPHERE, 0x0032_fddc)?;
        let miss = c.g[V0][0] == 0;
        c.g[V0][0] = 0;
        if !miss {
            c.cycles += 19;
            c.lqc2(3, at(c, S0, 0x10));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(2, 22);
            c.mtc1(2, 0);
            c.swc1(0, at(c, S0, 0x40));
            c.g[V0][0] = 1;
            c.mfc1(3, 0);
            c.qmtc2(3, 3);
            c.vu0().div(0, 3, 3, 0);
            c.lqc2(4, at(c, S0, 0x10));
            c.vu0().waitq();
            c.vu0().mul(0b1111, 5, 4, Src::Q);
            c.sqc2(5, at(c, S0, 0x10));
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WorldTriggerManager_dtor(WorldTriggerManager* self, int flags)`
/// (0x2b5988): destroy the trigger instances backwards, free their array,
/// then delete the manager when bit 0 of `flags` says so.
pub fn world_trigger_manager_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        let self_ = abi::arg(c, 0);
        let flags = abi::arg64(c, 1);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = sx(self_);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = flags;
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw(r32(c, S1));
        let empty = c.g[A1][0] == 0;
        c.g[V0][0] = 0x70;
        if !empty {
            c.cycles += 5;
            c.g[V1][0] = c.lw(r32(c, A1).wrapping_sub(0x10));
            c.mult(0, 4, 3, 2);
            c.g[S0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
            if c.g[A1][0] == c.g[S0][0] {
                c.g[A0][0] = c.lw(r32(c, S1));
            } else {
                c.cycles += 1;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x70));
                loop {
                    c.cycles += 6;
                    c.g[A1][0] = 0;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, callee::WORLD_TRIGGER_INSTANCE_DTOR, 0x002b_59d4)?;
                    c.g[V0][0] = c.lw(r32(c, S1));
                    let more = c.g[V0][0] != c.g[S0][0];
                    c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x70));
                    if !more {
                        c.cycles += 1;
                        c.g[A0][0] = c.lw(r32(c, S1));
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002b_59c8);
                    }
                }
            }
            c.cycles += 2;
            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x10));
            call_ptr(c, callee::OPERATOR_DELETE, 0x002b_59ec)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S2][0] & 1;
        let keep = c.g[V0][0] == 0;
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, callee::OPERATOR_DELETE, 0x002b_5a00)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int PS2Pad_Create(PS2Pad** out, int n)` (0x326d60): allocate and build up
/// to two pads, storing them in `out`, and return how many were made.
pub fn ps2_pad_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        let out = abi::arg(c, 0);
        let n = abi::arg64(c, 1);
        c.g[V0][0] = ((c.g[A1][0] as i64) < 3) as u64;
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        c.g[S2][0] = 2;
        frame.save(c, S1, 0x30);
        if c.g[V0][0] != 0 {
            c.g[S2][0] = c.g[A1][0];
        }
        frame.save(c, S3, 0x10);
        c.g[S0][0] = 0;
        let none = (c.g[S2][0] as i64) <= 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 3;
            c.g[S1][0] = sx(out);
            c.g[S3][0] = 0x004a_0000;
            loop {
                c.cycles += 14;
                c.g[A0][0] = 0x180;
                c.g[A1][0] = sx(p(r32(c, S3), 0x3fb0));
                c.g[A2][0] = 0x400_0000;
                c.g[A3][0] = 0;
                call_ptr(c, callee::MEM_ALLOC, 0x0032_6dac)?;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = 0;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                call_ptr(c, callee::PS2_PAD_CTOR, 0x0032_6dc0)?;
                c.sw(r32(c, S1), c.g[V0][0]);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0032_6d98);
                }
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

/// `int cWorldCache_StreamerGetViewClearanceAll(cWorldCache* cache, int idx,
/// Vec4* out_pos, float* out_t)` (0x3a9770): update the view, copy the view
/// quad, and report whether the view's fade value is past the halfway point.
pub fn c_world_cache_streamer_get_view_clearance_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        let self_ = abi::arg(c, 0);
        let idx = abi::arg64(c, 1);
        let out_pos = abi::arg64(c, 2);
        let out_t = abi::arg64(c, 3);
        c.g[V0][0] = 0x50;
        c.mult(0, 3, 5, 2);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = out_pos;
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
        c.g[V0][0] = c.lw(at(c, S0, 0x240));
        let present = c.g[V0][0] != 0;
        c.g[S2][0] = out_t;
        if !present {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.cycles += 6;
            frame.restore(c, S0, 0x30);
            frame.restore(c, S1, 0x20);
            frame.restore(c, S2, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 14;
        call_ptr(c, callee::WORLD_CACHE_STREAMER_UPDATE_VIEW, 0x003a_97b0)?;
        c.lq(2, at(c, S0, 0x250));
        c.g[V1][0] = 1;
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(AT, 2);
        c.sq(2, r32(c, S1));
        c.mtc1(ZERO, 1);
        c.lwc1(0, at(c, S0, 0x288));
        c.sub_s(0, 0, 2);
        c.c_lt_s(1, 0);
        let positive = c.fcc();
        c.swc1(0, r32(c, S2));
        if !positive {
            c.cycles += 1;
            c.g[V1][0] = 0;
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[V1][0];
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int MultiSplineModifier_v23(self, item)` (0x35ab60): whether `item` is the
/// current spline (and its end flag), or hand the item to the current spine's
/// virtual "add" (slot 0x10c) and return the modifier's count.
pub fn multi_spline_modifier_v23(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        let self_ = abi::arg(c, 0);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(self_);
        c.g[V0][0] = c.lw(at(c, S0, 0x40));
        let is_current = c.g[A1][0] == c.g[V0][0];
        if is_current {
            c.cycles += 8;
            c.g[A1][0] = c.lw(at(c, A1, 0xc));
            c.g[V1][0] = c.lw(at(c, A1, 0xc));
            c.g[A0][0] = c.lh(at(c, V1, 0xd0));
            c.g[V0][0] = c.lw(at(c, V1, 0xd4));
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            let target = r32(c, V0);
            call_ptr(c, target, 0x0035_ab94)?;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 0;
                c.cycles += 1;
                frame.restore(c, S0, 0x10);
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 9;
            c.g[V0][0] = c.lw(at(c, S0, 0x40));
            c.g[A1][0] = c.lw(at(c, V0, 0xc));
            c.g[V1][0] = c.lw(at(c, A1, 0xc));
            c.g[A0][0] = c.lh(at(c, V1, 0xd8));
            c.g[V0][0] = c.lw(at(c, V1, 0xdc));
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            let target = r32(c, V0);
            call_ptr(c, target, 0x0035_abb8)?;
            frame.restore(c, S0, 0x10);
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.g[A3][0] = c.lw(at(c, V0, 0xc));
        c.cycles += 8;
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x10));
        c.g[A2][0] = c.lw(at(c, S0, 0x38));
        c.g[V0][0] = c.lw(at(c, A3, 0xc));
        c.g[A0][0] = c.lh(at(c, V0, 0x108));
        c.g[V1][0] = c.lw(at(c, V0, 0x10c));
        c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
        let target = r32(c, V1);
        call_ptr(c, target, 0x0035_abdc)?;
        c.g[V0][0] = c.lw(at(c, S0, 0x38));
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cUIProgress_UpdateAnim(self, unsigned short ev)` (0x39c398): advance
/// the progress animator, then pass the event to the `+0x78` and `+0x74`
/// virtual objects.
pub fn c_ui_progress_update_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        let self_ = abi::arg(c, 0);
        let ev = abi::arg(c, 1);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(self_);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(at(c, S0, 0xc));
        let no_anim = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0] & 0xffff;
        if !no_anim {
            c.cycles += 7;
            c.g[A2][0] = c.lbu(at(c, S0, 0x10));
            c.g[A1][0] = c.g[S0][0];
            c.g[A3][0] = c.g[S1][0];
            call_ptr(c, callee::UI_ANIM_APPLY, 0x0039_c3c8)?;
            c.g[A0][0] = c.lw(at(c, S0, 0xc));
            c.g[A1][0] = c.lbu(at(c, S0, 0x10));
            call_ptr(c, callee::UI_ANIM_ADVANCE, 0x0039_c3d4)?;
        }
        c.cycles += 3;
        c.g[A2][0] = c.lw(at(c, S0, 0x78));
        let no_a = c.g[A2][0] == 0;
        c.g[A1][0] = c.g[S1][0];
        if !no_a {
            c.cycles += 5;
            c.g[V0][0] = c.lw(at(c, A2, 8));
            c.g[A0][0] = c.lh(at(c, V0, 0x98));
            c.g[V1][0] = c.lw(at(c, V0, 0x9c));
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            let target = r32(c, V1);
            call_ptr(c, target, 0x0039_c3f4)?;
        }
        c.cycles += 3;
        c.g[A2][0] = c.lw(at(c, S0, 0x74));
        let no_b = c.g[A2][0] == 0;
        c.g[A1][0] = c.g[S1][0];
        if !no_b {
            c.cycles += 5;
            c.g[V0][0] = c.lw(at(c, A2, 8));
            c.g[A0][0] = c.lh(at(c, V0, 0x98));
            c.g[V1][0] = c.lw(at(c, V0, 0x9c));
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            let target = r32(c, V1);
            call_ptr(c, target, 0x0039_c414)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFlagManager_dtor(cFlagManager* self, int flags)` (0x34c4b8): destroy
/// the embedded flag set's elements backwards, then the game process.
pub fn c_flag_manager_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        let self_ = abi::arg(c, 0);
        let flags = abi::arg64(c, 1);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S1, 0x30);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x480));
        frame.save(c, S3, 0x10);
        c.g[S1][0] = sx(self_);
        frame.save(c, S0, 0x40);
        c.g[V1][0] = sx(p(self_, 0x20));
        frame.save(c, S2, 0x20);
        c.g[S3][0] = flags;
        frame.save_ra(c, 0);
        let no_elems = c.g[V1][0] == 0;
        c.sw(at(c, S1, 0xc), c.g[V0][0]);
        if !no_elems {
            c.cycles += 3;
            c.g[S0][0] = sx(p(r32(c, S1), 0x1718));
            let empty = c.g[V1][0] == c.g[S0][0];
            c.g[S2][0] = c.g[V1][0];
            if !empty {
                c.cycles += 2;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x188));
                loop {
                    c.cycles += 8;
                    c.g[A1][0] = 0;
                    c.g[V0][0] = c.lw(at(c, S0, 0x184));
                    c.g[A0][0] = c.lh(at(c, V0, 8));
                    c.g[V1][0] = c.lw(at(c, V0, 0xc));
                    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                    let target = r32(c, V1);
                    call_ptr(c, target, 0x0034_c518)?;
                    let more = c.g[S2][0] != c.g[S0][0];
                    c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x188));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0034_c500);
                    }
                }
            }
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, callee::GAME_PROCESS_DTOR, 0x0034_c52c)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_RestoreGameCameras(Replay* self)` (0x26eea0): after restoring
/// the in-game camera triggers, hand every rider's camera back to it and
/// clear the replay's camera-lock flag.
pub fn replay_restore_game_cameras(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        let self_ = abi::arg(c, 0);
        frame.save(c, S3, 0x10);
        c.g[S3][0] = sx(self_);
        frame.save(c, S0, 0x40);
        c.g[A0][0] = 0x004c_0000;
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(p(r32(c, A0), 0x5830));
        call_ptr(
            c,
            callee::CAMERA_TRIGGER_MAN_SET_IN_GAME_TRIGGERS,
            0x0026_eec8,
        )?;
        c.g[V0][0] = c.lw(gp(c, -0x848));
        c.g[V1][0] = c.lw(at(c, V0, 0x84));
        c.g[V0][0] = c.lw(at(c, V1, 0x84));
        c.g[V1][0] = c.lw(r32(c, V0));
        let none = (c.g[V1][0] as i64) <= 0;
        c.g[S2][0] = sx(p(r32(c, V0), 4));
        if !none {
            c.cycles += 2;
            c.g[S1][0] = c.g[V1][0];
            c.g[V0][0] = c.lw(r32(c, S2));
            loop {
                c.cycles += 10;
                c.g[A1][0] = 2;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(4));
                c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
                c.g[A0][0] = c.g[V0][0];
                c.g[S0][0] = c.lw(at(c, V0, 0xa8));
                call_ptr(c, callee::RIDER_SET_INPUT_ENABLED, 0x0026_ef00)?;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, callee::CAMERA_SET_OWNER, 0x0026_ef08)?;
                if c.g[S1][0] == 0 {
                    break;
                }
                c.g[V0][0] = c.lw(r32(c, S2));
                if event_due(c) {
                    deliver_event_here(c, 0x0026_eee8);
                }
            }
        }
        c.cycles += 8;
        c.sw(at(c, S3, 0x630), 0);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* Flag_ctor(Flag* self, void* node, const Params* params, void*
/// owner)` (0x34add8): build the world object, then read the flag parameters
/// and register the flag with the flag manager.
pub fn flag_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0xa0);
        let self_ = abi::arg(c, 0);
        let node = abi::arg64(c, 1);
        let params = abi::arg64(c, 2);
        let owner = abi::arg64(c, 3);
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        c.g[S0][0] = params;
        frame.save(c, S2, 0x70);
        c.g[S1][0] = sx(self_);
        c.g[S2][0] = owner;
        frame.save_ra(c, 0x60);
        c.g[A3][0] = c.g[S0][0];
        c.g[A1][0] = node;
        c.g[A2][0] = 0xa;
        call_ptr(c, callee::WORLD_OBJ_CTOR, 0x0034_ae04)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[A2][0] = 0xffff_ffff_ffff_0000;
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x3f0));
        c.g[A2][0] = c.g[A2][0] | 0xfffd;
        c.sw(at(c, S1, 0xc), c.g[V1][0]);
        c.g[A0][0] = sx(frame.at(c, 0));
        c.g[A1][0] = c.g[S2][0];
        c.g[V0][0] = c.lw(at(c, S0, 8));
        c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
        c.g[V0][0] = c.g[V0][0] | 4;
        c.sw(at(c, S0, 8), c.g[V0][0]);
        call_ptr(c, callee::FLAG_READ_PARAMS, 0x0034_ae34)?;
        call_ptr(c, callee::GET_FLAG_MANAGER, 0x0034_ae3c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = sx(frame.at(c, 0));
        call_ptr(c, callee::FLAG_MANAGER_ADD_FLAG, 0x0034_ae4c)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Game module and FE state entry points.

/// `void cGame_restart(GameModule* self)` (0x2302a8): reset the race and the
/// game mode, clear every rider's state word, then request the restart state
/// and re-enter the menu flow.
pub fn game_module_quick_restart(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        let self_ = abi::arg(c, 0);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(self_);
        call_ptr(c, callee::GAME_MODULE_RESET_RACE, 0x0023_02bc)?;
        c.g[V0][0] = c.lw(gp(c, -0x848));
        c.g[A0][0] = c.lw(at(c, V0, 0xc0));
        call_ptr(c, callee::GAME_MODE_MAN_RESET_MODE, 0x0023_02c8)?;
        c.g[V1][0] = c.lw(at(c, S0, 0xc));
        c.g[V0][0] = c.lw(at(c, V1, 0x78));
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 2;
            c.g[V1][0] = c.lw(at(c, S0, 0xc));
            c.g[A0][0] = sx(r32(c, A1) << 2);
            loop {
                c.cycles += 9;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
                c.g[V0][0] = c.lw(at(c, V1, 0x28));
                c.sw(at(c, V0, 0x2f8), 0);
                c.g[V1][0] = c.lw(at(c, S0, 0xc));
                c.g[V0][0] = c.lw(at(c, V1, 0x78));
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = sx(r32(c, A1) << 2);
                if event_due(c) {
                    deliver_event_here(c, 0x0023_02e0);
                }
            }
        }
        c.cycles += 13;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        c.g[A2][0] = 2;
        c.g[A3][0] = 1;
        call_ptr(c, callee::GAME_MODULE_REQUEST_STATE, 0x0023_0318)?;
        call_ptr(c, callee::SSX_AUDIO_SYSTEM_GET_INSTANCE, 0x0023_0320)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, callee::GAME_MODULE_START_NEXT_MODE, 0x0023_0328)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cFEStateLodge_ctor(self, owner)` (0x1f3700): build the lodge
/// screen's UI object, pick the BE interface index and enable the lodge's
/// message bit.
pub fn c_fe_state_lodge_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        let self_ = abi::arg(c, 0);
        let owner = abi::arg64(c, 1);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(self_);
        c.g[A1][0] = owner;
        call_ptr(c, callee::UI_OBJECT_CTOR, 0x001f_3718)?;
        c.g[V1][0] = 0x0047_0000;
        c.g[V0][0] = 0x27;
        c.g[V1][0] = sx(p(r32(c, V1), 0x3aa8));
        c.sw(at(c, S1, 0xc), c.g[V0][0]);
        c.sw(at(c, S1, 8), c.g[V1][0]);
        call_ptr(c, callee::BE_GET_APP, 0x001f_3730)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, callee::BE_GET_INTERFACE, 0x001f_373c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, callee::BE_INTERFACE_GET_INDEX, 0x001f_3748)?;
        c.g[V1][0] = c.g[V0][0];
        c.sb(at(c, S1, 0x44), 0);
        c.g[V0][0] = c.lw(at(c, S1, 0x10));
        c.g[A0][0] = c.lw(r32(c, V0));
        let no_be = c.g[A0][0] == 0;
        c.g[S0][0] = 1;
        if !no_be {
            c.cycles += 6;
            c.g[A1][0] = 0;
            let shift = r32(c, V1) & 31;
            c.g[S0][0] = sx(r32(c, S0) << shift);
            c.g[S0][0] = c.g[S0][0] & 0xff;
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, callee::BE_GET_BE, 0x001f_3774)?;
            c.sb(at(c, S1, 0x15), c.g[S0][0]);
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

/// `void InputParser_Error(InputParser* self, const char* fmt, ...)`
/// (0x321500): format the parser's name and the message into a local buffer
/// and flag the parser as failed.
pub fn input_parser_error(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x4b0);
        let self_ = abi::arg(c, 0);
        let fmt = abi::arg64(c, 1);
        let a2 = abi::arg64(c, 2);
        let a3 = abi::arg64(c, 3);
        let t0 = abi::arg64(c, 4);
        let t1 = abi::arg64(c, 5);
        let t2 = abi::arg64(c, 6);
        let t3 = abi::arg64(c, 7);
        frame.save(c, S0, 0x430);
        frame.save(c, S1, 0x420);
        c.g[S0][0] = sx(self_);
        frame.save(c, S2, 0x410);
        c.g[S1][0] = sx(frame.at(c, 0x480));
        frame.save_ra(c, 0x400);
        c.g[S2][0] = fmt;
        c.sd(frame.at(c, 0x488), a3);
        c.g[A1][0] = 0x0049_0000;
        c.g[A3][0] = c.lw(at(c, S0, 0x8c));
        c.g[A0][0] = sx(frame.at(c, 0));
        c.sd(frame.at(c, 0x480), a2);
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x22c0));
        c.sd(frame.at(c, 0x490), t0);
        c.g[A2][0] = sx(p(r32(c, S0), 0xc));
        c.sd(frame.at(c, 0x498), t1);
        c.sd(frame.at(c, 0x4a0), t2);
        c.sd(frame.at(c, 0x4a8), t3);
        frame.save_fpr(c, 12, 0x470);
        frame.save_fpr(c, 14, 0x474);
        frame.save_fpr(c, 16, 0x478);
        frame.save_fpr(c, 18, 0x47c);
        call_ptr(c, callee::SPRINTF, 0x0032_1560)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = sx(r32(c, SP).wrapping_add(r32(c, V0)));
        call_ptr(c, callee::VSPRINTF, 0x0032_1570)?;
        c.g[V1][0] = 1;
        frame.restore(c, S1, 0x420);
        c.sw(at(c, S0, 0x94), c.g[V1][0]);
        frame.restore(c, S0, 0x430);
        frame.restore(c, S2, 0x410);
        frame.restore_ra(c, 0x400);
        frame.close(c);
        Ok(())
    })
}

/// `void* ManualCamController_ctor(self, void* cam)` (0x15cd80): build the
/// camera controller and its two algorithms, then select the manual one.
pub fn manual_cam_controller_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x20);
        let self_ = abi::arg(c, 0);
        let cam = abi::arg64(c, 1);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(self_);
        call_ptr(c, callee::CAM_CONTROLLER_CTOR, 0x0015_cd94)?;
        c.g[V1][0] = 0x0046_0000;
        c.g[V0][0] = 1;
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x4728));
        c.g[A1][0] = 0x0046_0000;
        c.sw(at(c, S0, 0x14), c.g[V1][0]);
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x4dd0));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.sw(r32(c, S0), c.g[V0][0]);
        c.g[A0][0] = 0x40;
        let _ = cam;
        call_ptr(c, callee::MEM_ALLOC, 0x0015_cdc0)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, callee::MANUAL_CAM_ALGO_CTOR, 0x0015_cdc8)?;
        c.g[A1][0] = 0x0046_0000;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x4dc0));
        c.sw(at(c, S0, 0x18), c.g[V0][0]);
        c.g[A0][0] = 0x40;
        call_ptr(c, callee::MEM_ALLOC, 0x0015_cde4)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, callee::EDITOR_CAM_ALGO_CTOR, 0x0015_cdec)?;
        c.sw(at(c, S0, 0x1c), c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x51;
        call_ptr(c, callee::MANUAL_CAM_SELECT_ALGORITHM, 0x0015_cdfc)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int ScriptSoundBankMan_AddBank(SndList* self, int id, void* data, int b)`
/// (0x283580): reuse the bank entry for `id`, or take a free one, remember
/// the data and load the bank.
pub fn script_sound_bank_man_add_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        let self_ = abi::arg(c, 0);
        let id = abi::arg64(c, 1);
        let data = abi::arg64(c, 2);
        let b = abi::arg64(c, 3);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = sx(self_);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = id;
        frame.save(c, S4, 0x10);
        c.g[S3][0] = data;
        frame.save(c, S0, 0x50);
        frame.save_ra(c, 0);
        c.g[S4][0] = b;
        call_ptr(c, callee::SND_LIST_FIND_BANK, 0x0028_35b0)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.g[V0][0] = c.lw(at(c, S0, 4));
        } else {
            c.cycles += 13;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, callee::SND_LIST_FIND_FREE_ENTRY, 0x0028_35c4)?;
            c.g[S0][0] = c.g[V0][0];
            c.sw(r32(c, S0), c.g[S2][0]);
            c.sw(at(c, S0, 8), c.g[S3][0]);
            call_ptr(c, callee::SSX_AUDIO_SYSTEM_GET_INSTANCE, 0x0028_35d4)?;
            c.g[V1][0] = c.lw(at(c, V0, 0x118));
            c.g[A2][0] = c.g[S3][0];
            c.g[A1][0] = c.lw(at(c, S0, 4));
            c.g[A3][0] = c.g[S4][0];
            c.g[A0][0] = c.lw(r32(c, V1));
            call_ptr(c, callee::SND_BANK_MAN_LOAD_BANK_FROM_MEMORY, 0x0028_35ec)?;
            c.g[V0][0] = c.lw(at(c, S0, 4));
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEOptionsCredits_HandleInput(self, Pad* pad)` (0x1863e8): map the
/// pad's left/right presses to a credits scroll step (or a page turn).
pub fn c_fe_options_credits_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        let self_ = abi::arg(c, 0);
        let pad = abi::arg64(c, 1);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = pad;
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(self_);
        c.g[V1][0] = c.lw(at(c, S0, 8));
        c.g[A0][0] = c.lh(at(c, V1, 0x88));
        c.g[V0][0] = c.lw(at(c, V1, 0x8c));
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
        let target = r32(c, V0);
        call_ptr(c, target, 0x0018_6414)?;
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[V1][0] = c.lw(at(c, S1, 0x64));
            c.g[V0][0] = ((c.g[V1][0] as i64) < -7) as u64;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = 1;
            } else {
                c.cycles += 2;
                c.g[V0][0] = sx(r32(c, V1).wrapping_sub(1));
                c.cycles += 1;
                c.sw(at(c, S1, 0x64), c.g[V0][0]);
                c.cycles += 1;
                c.g[V0][0] = 1;
            }
        } else {
            c.g[V1][0] = c.lw(at(c, S0, 8));
            c.cycles += 6;
            c.g[A0][0] = c.lh(at(c, V1, 0x90));
            c.g[V0][0] = c.lw(at(c, V1, 0x94));
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
            let target = r32(c, V0);
            call_ptr(c, target, 0x0018_6444)?;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 0;
            } else {
                c.cycles += 4;
                c.g[V1][0] = c.lw(at(c, S1, 0x64));
                c.g[V0][0] = ((c.g[V1][0] as i64) < 8) as u64;
                let too_far = c.g[V0][0] == 0;
                c.g[V0][0] = sx(r32(c, V1).wrapping_add(1));
                if too_far {
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                } else {
                    c.cycles += 1;
                    c.sw(at(c, S1, 0x64), c.g[V0][0]);
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                }
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

/// `void cFEStateRiderBio_OnWidgetAction(self, item, msg)` (0x1912d0): on a
/// widget action (6) hand the event to the screen's virtual handler and swap
/// the screen; on a page change (9) remember the page and rebuild the bio.
pub fn c_fe_state_rider_bio_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        let self_ = abi::arg(c, 0);
        let item = abi::arg64(c, 1);
        let _msg = abi::arg64(c, 2);
        c.g[V1][0] = item;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let no_item = c.g[V1][0] == 0;
        c.g[S0][0] = sx(self_);
        if no_item {
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 6;
            let is_six = c.g[A2][0] == c.g[V0][0];
            c.g[V0][0] = 9;
            if is_six {
                c.cycles += 11;
                c.g[V0][0] = c.lw(at(c, S0, 0x10));
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.lw(at(c, V1, 0x18));
                c.g[A3][0] = c.lw(r32(c, V0));
                c.g[V1][0] = c.lw(at(c, A3, 4));
                c.g[A0][0] = c.lh(at(c, V1, 0x28));
                c.g[V0][0] = c.lw(at(c, V1, 0x2c));
                c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
                let target = r32(c, V0);
                call_ptr(c, target, 0x0019_1328)?;
                let nothing = c.g[V0][0] == 0;
                c.g[A1][0] = c.g[V0][0];
                if nothing {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x10);
                } else {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw(at(c, S0, 0x10));
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
                    call_ptr(c, callee::UI_OBJECT_ADD_CHILD, 0x0019_133c)?;
                    frame.restore(c, S0, 0x10);
                }
            } else {
                c.cycles += 2;
                let is_nine = c.g[A2][0] == c.g[V0][0];
                if is_nine {
                    c.g[V0][0] = c.lbu(at(c, V1, 0x319));
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.sw(at(c, S0, 0x4c), c.g[V0][0]);
                    call_ptr(c, 0x0019_0a08, 0x0019_1350)?;
                    c.cycles += 1;
                    frame.restore(c, S0, 0x10);
                } else {
                    c.cycles += 2;
                    frame.restore(c, S0, 0x10);
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CameraTriggerMan_dtor(self, int flags)` (0x16c888): purge the active
/// trigger list and the four trigger slots, then the manager itself.
pub fn camera_trigger_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        let self_ = abi::arg(c, 0);
        let flags = abi::arg64(c, 1);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = sx(self_);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[S3][0] = flags;
        call_ptr(c, 0x0016_cbe0, 0x0016_c8ac)?;
        c.g[V0][0] = sx(p(r32(c, S1), 0x14));
        let none = c.g[V0][0] == 0;
        c.g[S0][0] = sx(p(r32(c, S1), 0x1c));
        if !none {
            c.cycles += 2;
            let empty = c.g[V0][0] == c.g[S0][0];
            c.g[S2][0] = c.g[V0][0];
            if !empty {
                c.cycles += 2;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(4));
                loop {
                    c.cycles += 5;
                    c.g[A1][0] = 0;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0016_c5e0, 0x0016_c8d4)?;
                    let more = c.g[S2][0] != c.g[S0][0];
                    c.g[S0][0] = sx(r32(c, S0).wrapping_sub(4));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0016_c8c8);
                    }
                }
            }
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0016_bef8, 0x0016_c8e8)?;
        c.g[V0][0] = c.g[S3][0] & 1;
        let keep = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x40);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, callee::OPERATOR_DELETE, 0x0016_c8fc)?;
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void NetFEState_OnWidgetAction(self, item, msg)` (0x1a8918): on a `msg`
/// of 7, build the widget object, flag it and add it to the screen.
pub fn net_fe_state_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        let self_ = abi::arg(c, 0);
        let _msg = abi::arg64(c, 2);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = sx(self_);
        let no_widget = c.g[A1][0] == 0;
        frame.save_ra(c, 0);
        let mut tail = no_widget;
        if !no_widget {
            c.cycles += 3;
            c.g[V0][0] = 7;
            let other = c.g[A2][0] != c.g[V0][0];
            if !other {
                c.cycles += 23;
                c.g[A1][0] = 0x0046_0000;
                c.g[A3][0] = 0;
                c.g[A1][0] = sx(p(r32(c, A1), 0x1a70));
                c.g[A0][0] = 0x140;
                c.g[A2][0] = 0x100;
                call_ptr(c, callee::MEM_ALLOC, 0x001a_8954)?;
                c.g[A1][0] = c.lw(at(c, S1, 0x10));
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S1][0];
                call_ptr(c, callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE_2, 0x001a_8964)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = 1;
                c.g[A1][0] = c.g[S0][0];
                c.sw(at(c, S0, 0x50), c.g[V0][0]);
                c.g[V0][0] = c.lw(at(c, S1, 8));
                c.g[A0][0] = c.lh(at(c, V0, 0xc8));
                c.g[V1][0] = c.lw(at(c, V0, 0xcc));
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                let target = r32(c, V1);
                call_ptr(c, target, 0x001a_8988)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x10));
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
                call_ptr(c, callee::UI_OBJECT_ADD_CHILD, 0x001a_8998)?;
                tail = true;
            }
        }
        if tail {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void NetFEState_ShowMaxBuddiesPopup(self)` (0x1abed8): fill and show the
/// "maximum buddies" network popup.
pub fn net_fe_state_show_max_buddies_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x40);
        let self_ = abi::arg(c, 0);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(self_);
        call_ptr(c, callee::NET_FE_STATE_GET_POPUP, 0x001a_bef4)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0x0046_0000;
        c.g[V0][0] = 1;
        c.g[A0][0] = sx(p(r32(c, A0), 0x1618));
        c.sw(at(c, S0, 0xbc), c.g[V0][0]);
        c.g[S1][0] = sx(p(r32(c, S0), 0xbc));
        call_ptr(c, callee::GET_HASH_VALUE32, 0x001a_bf10)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = c.g[V0][0];
        call_ptr(c, callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID, 0x001a_bf20)?;
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx(p(r32(c, A0), 0x2b00));
        call_ptr(c, callee::GET_HASH_VALUE32, 0x001a_bf2c)?;
        c.g[A1][0] = 0x845;
        c.g[V1][0] = 3;
        c.g[A2][0] = 6;
        c.sw(at(c, S0, 0x150), c.g[A1][0]);
        c.sw(at(c, S0, 0xc4), c.g[V0][0]);
        c.g[A0][0] = c.g[S2][0];
        c.sw(at(c, S0, 0xc0), c.g[V1][0]);
        c.g[A1][0] = c.g[S0][0];
        c.sw(at(c, S0, 0x14c), c.g[A2][0]);
        call_ptr(c, callee::NET_FE_STATE_SHOW_POPUP, 0x001a_bf54)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEUseExistingAccount_Init(self)` (0x1ac0b8): create the account
/// screen, play it, clear the two global strings and re-init the net state.
pub fn c_fe_use_existing_account_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        let self_ = abi::arg(c, 0);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = sx(self_);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx(p(r32(c, A0), 0x2b20));
        c.g[S0][0] = c.lw(at(c, S1, 0x10));
        call_ptr(c, callee::GET_HASH_VALUE32, 0x001a_c0dc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME, 0x001a_c0f0)?;
        let no_screen = c.g[V0][0] == 0;
        c.sw(at(c, S1, 0x40), c.g[V0][0]);
        if !no_screen {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, callee::UI_SCREEN_PLAY_FRAME, 0x001a_c108)?;
        }
        c.cycles += 17;
        c.g[A0][0] = c.lw(gp(c, -0xc8));
        c.g[S0][0] = 0x004a_0000;
        c.g[S0][0] = sx(p(r32(c, S0), 0x1408));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x48));
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, callee::CBE_STRING_CTOR, 0x001a_c120)?;
        c.g[A0][0] = c.lw(gp(c, -0xc8));
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x4c));
        call_ptr(c, callee::CBE_STRING_CTOR, 0x001a_c130)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::NET_FE_STATE_INIT, 0x001a_c138)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Tests: one differential test per replacement, against the generated body.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, DATA, GAME_GP};
    use ssx3_ee as ee;

    const SEEDS: u32 = 200;
    /// `RA` the setup's helper calls return to.
    const RET: u32 = 0x0012_3450;
    /// An empty guest function (returns at once, 2 cycles).
    const EMPTY: u32 = 0x0038_dc60;
    /// `SSXAudioSystem_GetInstance`: returns `*(gp+0x410)`, 0 in tests.
    const RET_ZERO: u32 = 0x0028_b180;

    fn w8(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write8(a, v as u8);
    }
    fn w16(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write16(a, v as u16);
    }
    fn w32(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }
    /// A `gp`-relative global address.
    fn gpat(off: i32) -> u32 {
        GAME_GP.wrapping_add(off as u32)
    }
    /// A NUL-terminated string in the test data area.
    fn cstr(c: &mut Cpu, at: u32, s: &str) {
        for (i, b) in s.bytes().enumerate() {
            w8(c, at + i as u32, b as u32);
        }
        w8(c, at + s.len() as u32, 0);
    }
    /// `{ short delta; short; void (*fn)(); }` at `vt + off`.
    fn slot(c: &mut Cpu, vt: u32, off: u32, func: u32) {
        w16(c, vt + off, 0);
        w32(c, vt + off + 4, func);
    }
    /// A zeroed BE-library singleton at `gp-0x1f04` (otherwise the UI/FE
    /// helpers fall into the lazy disc load at `0x3de768`).
    fn be_library(c: &mut Cpu) {
        w32(c, gpat(-0x1f04), DATA + 0x30_0000);
    }
    /// The engine's size-class pool allocator: the "pools ready" flag plus
    /// sixteen free nodes per size class.
    fn pool(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w32(c, gpat(0xdd8), 1);
        for (i, (slot_off, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let pool = DATA + 0x40_0000 + 0x4_0000 * i;
            let base = DATA + 0x80_0000 + 0x4000 * i;
            for n in 0..16u32 {
                let node = base + 0x400 * n;
                w32(c, node, if n == 15 { 0 } else { node + 0x400 });
            }
            w32(c, pool + head, base);
            w32(c, GAME_GP + slot_off, pool);
        }
    }
    /// The world singleton at `gp-0x848` with an empty race-manager rider
    /// list (a rider loop on junk data runs away).
    fn world(c: &mut Cpu) {
        let w = DATA + 0x2_0000;
        w32(c, gpat(-0x848), w);
        w32(c, w + 0x78, DATA + 0x2_1000);
        w32(c, w + 0x84, DATA + 0x2_2000);
        w32(c, DATA + 0x2_2000 + 0xc, DATA + 0x2_3000);
        w32(c, DATA + 0x2_2000 + 0x84, DATA + 0x2_4000);
        w32(c, DATA + 0x2_4000, 0);
        w32(c, w + 0xc0, DATA + 0x2_5000);
    }
    /// All the standard fixtures.
    fn env(c: &mut Cpu) {
        be_library(c);
        world(c);
        pool(c);
    }
    /// `GetHashValue32(s)`, as the game computes it (the same machine state
    /// in both runs, so it can be called from the setup).
    fn hash(c: &mut Cpu, s: u32) -> u32 {
        c.g[RA][0] = RET as u64;
        c.g[A0][0] = sx(s);
        c.pc = 0x0031_7670;
        ssx3_ee_rt::funcs::call_indirect(c, 0x0031_7670);
        assert_eq!(c.pc, RET, "GetHashValue32 did not return");
        c.g[V0][0] as u32
    }

    // ---------------------------------------------------------------- FE/UI

    #[test]
    fn ui_state_show_object_matches() {
        check(
            ee::s13::f_0039e948__generated,
            ui_state_show_object,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x39e9, 0);
                env(c);
                let this = DATA + 0x400;
                let name = DATA + 0x800;
                cstr(c, name, "OBJ_HELLO");
                let list = this + 0x24;
                let sentinel = DATA + 0x1000;
                let node = DATA + 0x1100;
                w32(c, sentinel + 4, sentinel);
                match seed % 3 {
                    0 => {
                        w32(c, list + 4, sentinel);
                    }
                    1 => {
                        let obj = DATA + 0x1200;
                        let vt = DATA + 0x1300;
                        w32(c, node, sentinel);
                        w32(c, node + 4, sentinel);
                        w32(c, list + 4, node);
                        w32(c, node + 0x38, DATA + 0x1400);
                        w32(c, node + 0x3c, 1);
                        w32(c, DATA + 0x1400, obj);
                        let hv = hash(c, name);
                        w32(c, obj + 0x38, hv);
                        w32(c, obj + 8, vt);
                        slot(c, vt, 0x48, EMPTY);
                    }
                    _ => {
                        w32(c, node, sentinel);
                        w32(c, node + 4, sentinel);
                        w32(c, list + 4, node);
                        w32(c, node + 0x3c, 0);
                    }
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(name);
            },
        );
    }

    #[test]
    fn process_list_delete_priority_matches() {
        check(
            ee::s11::f_00355118__generated,
            process_list_delete_priority,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x3551, 0);
                env(c);
                let this = DATA + 0x400;
                let index = (seed % 2) as u32;
                let base = DATA + 0x2000;
                let desc = base + 0x44 * index;
                let sentinel = desc + 0x14;
                w32(c, this + 4, base);
                let vt = DATA + 0x3200;
                slot(c, vt, 8, EMPTY);
                let n0 = DATA + 0x3000;
                let n1 = DATA + 0x3100;
                w32(c, n0 + 0xc, vt);
                w32(c, n1 + 0xc, vt);
                match seed % 3 {
                    0 => w32(c, desc + 8, sentinel),
                    1 => {
                        w32(c, desc + 8, n0);
                        w32(c, n0 + 4, n1);
                        w32(c, n1 + 4, sentinel);
                    }
                    _ => {
                        w32(c, desc + 8, n0);
                        w32(c, n0 + 4, sentinel);
                    }
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = index as u64;
            },
        );
    }

    #[test]
    fn cfe_state_transition_start_fade_matches() {
        check(
            ee::s02::f_001946a8__generated,
            cfe_state_transition_start_fade,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1946, 0);
                env(c);
                let this = DATA + 0x400;
                let which = (seed % 3) as u64;
                let screen_a = if seed % 2 == 0 { 0 } else { DATA + 0x800 };
                let screen_b = if seed % 4 < 2 { 0 } else { DATA + 0x900 };
                w32(c, this + 0x48, screen_a);
                w32(c, this + 0x4c, screen_b);
                for s in [DATA + 0x800, DATA + 0x900] {
                    w32(c, s + 0x54, 0x10);
                    w32(c, s + 0x4c, 0);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = which;
            },
        );
    }

    #[test]
    fn instances_man_alloc_by_handle_matches() {
        check(
            ee::s11::f_003511d0__generated,
            instances_man_alloc_by_handle,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x3511, 0);
                env(c);
                let this = DATA + 0x400;
                w8(c, this, 5);
                match seed % 3 {
                    0 => w32(c, this + 8, 0),
                    1 => {
                        // `(lo(offset * 0xba2e8ba3) >> 4)` is `offset / 0xb0`
                        // for these offsets, so this node's handle matches.
                        let base = DATA + 0x2000;
                        let id = 3u32;
                        let node = base + 0xb0 * id;
                        w32(c, this + 8, node);
                        w32(c, this + 0xc, base);
                        w32(c, node, 0);
                        w32(c, node + 8, 0xff00);
                        c.g[A1][0] = (id << 8) as u64;
                    }
                    _ => {
                        // The free list holds one entry whose index does not
                        // match; `lw` of the address the multiplied handle
                        // lands on reads a clear word, so the loop ends.
                        let base = DATA + 0x3000;
                        let node = base + 0x10;
                        w32(c, this + 8, node);
                        w32(c, this + 0xc, base);
                        w32(c, node, 0);
                        c.g[A1][0] = 0x200;
                    }
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn cfe_state_net_messenger_on_child_event_matches() {
        check(
            ee::s03::f_001a4c58__generated,
            cfe_state_net_messenger_on_child_event,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1a4c, 0);
                env(c);
                let this = DATA + 0x400;
                let msg = DATA + 0x500;
                let vt = DATA + 0x800;
                let kind = [0x80bu32, 0x809, 0x80c, 0x800, 0x900][(seed % 5) as usize];
                for off in [0x108u32, 0x110, 0x118] {
                    slot(c, vt, off, EMPTY);
                }
                w32(c, this + 8, vt);
                w32(c, msg + 0x18, kind);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(msg);
                c.g[A2][0] = sx(DATA + 0x600);
            },
        );
    }

    #[test]
    fn net_fe_state_v26_matches() {
        check(ee::s03::f_001a89b0__generated, net_fe_state_v26, SEEDS, &|c, seed| {
            machine(c, seed, 0x1a89, 0);
            env(c);
            let this = DATA + 0x400;
            let msg = DATA + 0x500;
            let vt = DATA + 0x800;
            let popup = DATA + 0x900;
            let kind = [0x31u32, 0x33, 0x4d, 0x4e, 0x800, 0x999][(seed % 6) as usize];
            w32(c, this + 0x6a8, if seed % 4 == 0 { 0 } else { 1 });
            // The disconnected popup returns at once when it is already open
            // (its full path walks the UI tree and is not this test's subject).
            w32(c, this + 0x6ac, 1);
            w32(c, this + 0x6a0, popup);
            w32(c, gpat(-0xc8), if seed % 5 == 0 { 1 } else { 0 });
            w32(c, msg + 0xc, kind);
            w32(c, msg + 8, vt);
            slot(c, vt, 8, EMPTY);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(msg);
        });
    }

        #[test]
    fn cfe_state_create_step2_submit_account_matches() {
        check(
            ee::s03::f_001ba9f8__generated,
            cfe_state_create_step2_submit_account,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1ba9, 0);
                env(c);
                let this = DATA + 0x400;
                let popup = DATA + 0x900;
                w32(c, this + 0x6a0, popup);
                w32(c, this + 0x40, 0);
                w32(c, popup + 0x2b8, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn cfe_state_bragging_rights_dtor_matches() {
        check(
            ee::s04::f_001df4e8__generated,
            cfe_state_bragging_rights_dtor,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1df4, 0);
                env(c);
                let this = DATA + 0x400;
                let vt = DATA + 0x2000;
                slot(c, vt, 8, EMPTY);
                for i in 0..12u32 {
                    w32(c, this + 0x6d0 + 0x34 * i + 0x30, vt);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn ov_screen_message_center_shutdown_matches() {
        check(
            ee::s04::f_001e3e30__generated,
            ov_screen_message_center_shutdown,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1e3e, 0);
                env(c);
                let this = DATA + 0x400;
                w32(c, this + 0x18, [0, 1, 2, 0x7fff_ffff][(seed % 4) as usize]);
                w32(c, gpat(-0x69c), 4);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn buddy_man_find_roster_entry_matches() {
        check(
            ee::s07::f_00262468__generated,
            buddy_man_find_roster_entry,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x2624, 0);
                env(c);
                let this = DATA + 0x400;
                let list = DATA + 0x1000;
                let who = DATA + 0x800;
                cstr(c, who, "BUDDY2");
                let names = [DATA + 0x900, DATA + 0xa00, DATA + 0xb00];
                cstr(c, names[0], "BUDDY1");
                cstr(c, names[1], "BUDDY2");
                cstr(c, names[2], "BUDDY3");
                let n = (seed % 3) as u32; // 0, 1 or 2 entries
                w32(c, this + 0xcc, list);
                w32(c, list + 0x18, n);
                w32(c, list + 0x40, DATA + 0x1800);
                for i in 0..n {
                    let e = DATA + 0x2000 + 0x10 * i;
                    w32(c, DATA + 0x1800 + 4 * i, e);
                    w32(c, e, names[i as usize]);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(who);
            },
        );
    }

    #[test]
    fn script_engine_on_data_removed_matches() {
        check(
            ee::s07::f_002799e0__generated,
            script_engine_on_data_removed,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x2799, 0);
                env(c);
                let this = DATA + 0x400;
                let obj = DATA + 0x800;
                let kind = (seed % 4) as u64;
                w32(c, obj + 0x18, 0x11);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(obj);
                c.g[A2][0] = 0;
                c.g[A3][0] = kind;
                c.g[T0][0] = 5;
            },
        );
    }

                #[test]
    fn c_ui_text_scroll_dtor_matches() {
        check(
            ee::s13::f_003a62a0__generated,
            c_ui_text_scroll_dtor,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x3a62, 0);
                env(c);
                let this = DATA + 0x400;
                let sentinel = DATA + 0x1000;
                w32(c, sentinel + 4, sentinel);
                w32(c, this + 0x8c, sentinel);
                w32(c, this + 0x8c + 4, sentinel);
                w32(c, this + 0x98, sentinel);
                w32(c, this + 0x98 + 4, sentinel);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn rail_tube_ctor_matches() {
        check(ee::s12::f_00372520__generated, rail_tube_ctor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3725, 0);
            env(c);
            w32(c, gpat(0x113c), (seed % 3 == 0) as u32);
            w32(c, gpat(0x10f0), r.next_u32());
            w32(c, gpat(0x10f4), r.next_u32());
            w32(c, gpat(0x10f8), r.next_u32());
            c.g[A0][0] = sx(DATA + 0x400);
            c.g[A1][0] = sx(DATA + 0x800);
        });
    }

    #[test]
    fn irradiance_add_scaled_matches() {
        check(
            ee::s12::f_00389590__generated,
            irradiance_add_scaled,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3895, 0);
                env(c);
                let (a, b, cc) = (DATA + 0x1000, DATA + 0x2000, DATA + 0x3000);
                for i in 0..10u32 {
                    for k in 0..4u32 {
                        w32(c, a + 0x10 * i + 4 * k, r.edge_f32());
                        w32(c, b + 0x10 * i + 4 * k, r.edge_f32());
                    }
                }
                for k in 0..4u32 {
                    w32(c, cc + 4 * k, r.edge_f32());
                }
                c.g[A0][0] = sx(a);
                c.g[A1][0] = sx(b);
                c.g[A2][0] = sx(cc);
                c.f[12] = r.f32(4.0);
            },
        );
    }

    #[test]
    fn mc_manager_state25_matches() {
        check(
            ee::s06::f_0023fe70__generated,
            mc_manager_state25,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x23fe, 0);
                env(c);
                let this = DATA + 0x400;
                let o = DATA + 0x800;
                let vt = DATA + 0x900;
                w32(c, this + 0x434, o);
                w32(c, o, vt);
                slot(c, vt, 0x58, if seed % 2 == 0 { EMPTY } else { RET_ZERO });
                w32(c, this + 0x6c, [0, RET_ZERO][((seed / 2) % 2) as usize]);
                w32(c, this + 0x84, [0, RET_ZERO][((seed / 4) % 2) as usize]);
                w32(c, this + 0x68, [0, RET_ZERO][((seed / 8) % 2) as usize]);
                w32(c, gpat(0x410), 0);
                c.g[A0][0] = sx(this);
            },
        );
    }

        #[test]
    fn c_fe_state_progressive_init_matches() {
        check(
            ee::s06::f_00245248__generated,
            c_fe_state_progressive_init,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x2452, 0);
                env(c);
                let this = DATA + 0x400;
                w32(
                    c,
                    this + 0x10,
                    if seed % 2 == 0 { 0 } else { DATA + 0x1000 },
                );
                w32(c, this + 0x40, 0);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn debug_menu_next_enabled_item_matches() {
        check(
            ee::s02::f_0017a028__generated,
            debug_menu_next_enabled_item,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x17a0, 0);
                env(c);
                let this = DATA + 0x400;
                let n = 2 + (seed % 4); // 2..5 entries
                w32(c, this, n);
                let entries = this + 0x8c;
                // Every entry enabled, except one seed group that must scan a
                // little further.
                let dead = if seed % 3 == 0 { 0 } else { u32::MAX };
                for i in 0..n {
                    w32(c, entries + 0x8c * i, if i == dead { 0 } else { 1 });
                }
                w32(c, this + 0x8c08, (seed % 5 != 0) as u32);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % (n + 1)) as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_preview_reward_init_matches() {
        check(
            ee::s04::f_001d1c08__generated,
            c_fe_state_preview_reward_init,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1d1c, 0);
                env(c);
                let this = DATA + 0x400;
                w32(
                    c,
                    this + 0x10,
                    if seed % 2 == 0 { 0 } else { DATA + 0x1000 },
                );
                w32(c, this + 0x40, 0);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn c_fe_state_video_player_ctor_matches() {
        check(
            ee::s04::f_001d22f0__generated,
            c_fe_state_video_player_ctor,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1d22, 0);
                env(c);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x1000);
                c.g[A2][0] = sx(DATA + 0x2000);
                c.g[A3][0] = (-(seed as i32 % 100) as u64) & 0xff;
            },
        );
    }

    #[test]
    fn bolt_file_find_slot_by_name_matches() {
        check(
            ee::s03::f_0019d428__generated,
            bolt_file_find_slot_by_name,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x19d4, 0);
                env(c);
                let this = DATA + 0x400;
                let name = DATA + 0x800;
                cstr(c, name, "BOLT_B");
                // bank 1 selects the row at `this + 0x12000`.
                let row = this + 0x12000;
                match seed % 3 {
                    0 => {}
                    1 => cstr(c, row + 4, "BOLT_A"),
                    _ => {
                        cstr(c, row + 4, "BOLT_A");
                        cstr(c, row + 0x120 + 4, "BOLT_B");
                    }
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = 1;
                c.g[A2][0] = sx(name);
            },
        );
    }

    #[test]
    fn ssx_audio_system_update_music_tier_matches() {
        check(
            ee::s07::f_0028d5a0__generated,
            ssx_audio_system_update_music_tier,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x28d5, 0);
                env(c);
                let this = DATA + 0x400;
                w32(c, this + 0x6284, if seed % 4 == 0 { 0x17 } else { 0 });
                w32(c, gpat(0x410), 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 3) as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_options_open_reset_popup_matches() {
        check(
            ee::s02::f_00188e58__generated,
            c_fe_state_options_open_reset_popup,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x188e, 0);
                env(c);
                let this = DATA + 0x400;
                w32(
                    c,
                    this + 0x10,
                    if seed % 2 == 0 { 0 } else { DATA + 0x1000 },
                );
                c.g[A0][0] = sx(this);
            },
        );
    }

        #[test]
    fn sphere_tree_collide_sphere_matches() {
        check(
            ee::s10::f_0032c898__generated,
            sphere_tree_collide_sphere,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32c8, 0);
                env(c);
                let this = DATA + 0x400;
                let tree = DATA + 0x1000;
                w32(c, this + 0x98, tree);
                w32(c, tree + 8, (seed % 3 != 0) as u32);
                for k in 0..4u32 {
                    w32(c, DATA + 0x2000 + 4 * k, r.edge_f32());
                    w32(c, DATA + 0x2100 + 4 * k, r.edge_f32());
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = sx(DATA + 0x2100);
                c.g[A3][0] = sx(DATA + 0x2200);
                c.f[12] = r.f32(8.0);
            },
        );
    }

    #[test]
    fn sphere_tree_collide_aabb_matches() {
        check(
            ee::s10::f_0032c928__generated,
            sphere_tree_collide_aabb,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32c9, 0);
                env(c);
                let this = DATA + 0x400;
                let tree = DATA + 0x1000;
                w32(c, this + 0x98, tree);
                w32(c, tree + 8, (seed % 3 != 0) as u32);
                for k in 0..8u32 {
                    w32(c, DATA + 0x2000 + 4 * k, r.edge_f32());
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = sx(DATA + 0x2100);
                c.g[A3][0] = sx(DATA + 0x2200);
                c.g[T0][0] = sx(DATA + 0x2300);
            },
        );
    }

    #[test]
    fn coll_sphere_collide_sphere_tree_matches() {
        check(
            ee::s11::f_0032fdb0__generated,
            coll_sphere_collide_sphere_tree,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32fd, 0);
                env(c);
                let this = DATA + 0x400;
                let tree = DATA + 0x1000;
                let hit = DATA + 0x2000;
                for k in 0..4u32 {
                    w32(c, this + 0x60 + 4 * k, r.edge_f32());
                    w32(c, hit + 0x10 + 4 * k, r.f32(2.0));
                }
                w32(c, this + 0x70, r.f32(4.0));
                w32(c, tree + 0x98, DATA + 0x3000);
                w32(c, DATA + 0x3000 + 8, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(tree);
                c.g[A2][0] = sx(hit);
            },
        );
    }

        #[test]
    fn ps2_pad_create_matches() {
        check(ee::s10::f_00326d60__generated, ps2_pad_create, SEEDS, &|c, seed| {
            machine(c, seed, 0x326d, 0);
            env(c);
            c.g[A0][0] = sx(DATA + 0x800);
            c.g[A1][0] = (seed % 4) as u64;
        });
    }

    #[test]
    fn c_world_cache_streamer_get_view_clearance_all_matches() {
        check(
            ee::s13::f_003a9770__generated,
            c_world_cache_streamer_get_view_clearance_all,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a97, 0);
                env(c);
                let this = DATA + 0x400;
                let idx = (seed % 2) as u32;
                let e = this + 0x50 * idx;
                w32(c, e + 0x240, (seed % 3 != 0) as u32);
                for k in 0..4u32 {
                    w32(c, e + 0x250 + 4 * k, r.next_u32());
                }
                w32(c, e + 0x288, r.edge_f32());
                c.g[A0][0] = sx(this);
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = sx(DATA + 0x2000);
                c.g[A3][0] = sx(DATA + 0x2100);
            },
        );
    }

    #[test]
    fn replay_restore_game_cameras_matches() {
        check(
            ee::s07::f_0026eea0__generated,
            replay_restore_game_cameras,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x26ee, 0);
                env(c);
                let this = DATA + 0x400;
                let list = DATA + 0x2_4000;
                let n = (seed % 3) as u32;
                w32(c, list, n);
                for i in 0..n {
                    let rider = DATA + 0x3000 + 0x100 * i;
                    w32(c, DATA + 0x2_4000 + 4 + 4 * i, rider);
                    w32(c, rider + 0xa8, DATA + 0x2800 + 0x100 * i);
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn flag_ctor_matches() {
        check(ee::s11::f_0034add8__generated, flag_ctor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x34ad, 0);
            env(c);
            let this = DATA + 0x400;
            let node = DATA + 0x800;
            let src = DATA + 0x1000;
            w32(c, gpat(-0xc8), 0);
            w32(c, node + 8, r.next_u32());
            for k in 0..8u32 {
                w32(c, src + 4 * k, r.next_u32());
            }
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(node);
            c.g[A2][0] = sx(src);
            c.g[A3][0] = sx(DATA + 0x2000);
        });
    }

    #[test]
    fn multi_spline_modifier_v23_matches() {
        check(
            ee::s11::f_0035ab60__generated,
            multi_spline_modifier_v23,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x35ab, 0);
                env(c);
                let this = DATA + 0x400;
                let item = DATA + 0x800;
                let other = DATA + 0x900;
                let cur = [item, other, 0][(seed % 3) as usize];
                let vt = DATA + 0x1000;
                for off in [0xd0u32, 0xd8, 0x108] {
                    slot(c, vt, off, if seed % 2 == 0 { EMPTY } else { RET_ZERO });
                }
                w32(c, this + 0x40, cur);
                w32(c, this + 0x38, 0x21);
                w32(c, item + 0xc, DATA + 0x1200);
                w32(c, DATA + 0x1200 + 0xc, vt);
                w32(c, other + 0xc, DATA + 0x1300);
                w32(c, DATA + 0x1300 + 0xc, vt);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(item);
            },
        );
    }

    #[test]
    fn c_ui_progress_update_anim_matches() {
        check(
            ee::s13::f_0039c398__generated,
            c_ui_progress_update_anim,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x39c3, 0);
                env(c);
                let this = DATA + 0x400;
                let vt = DATA + 0x1000;
                slot(c, vt, 0x98, EMPTY);
                w32(c, this + 0xc, if seed % 2 == 0 { 0 } else { DATA + 0x2000 });
                w8(c, this + 0x10, seed % 8);
                w32(
                    c,
                    this + 0x78,
                    if seed % 3 == 0 { 0 } else { DATA + 0x3000 },
                );
                w32(
                    c,
                    this + 0x74,
                    if seed % 5 == 0 { 0 } else { DATA + 0x3100 },
                );
                w32(c, DATA + 0x3000 + 8, vt);
                w32(c, DATA + 0x3100 + 8, vt);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 0x1000) as u64;
            },
        );
    }

    #[test]
    fn c_flag_manager_dtor_matches() {
        check(
            ee::s11::f_0034c4b8__generated,
            c_flag_manager_dtor,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x34c4, 0);
                env(c);
                let this = DATA + 0x400;
                let vt = DATA + 0x2000;
                slot(c, vt, 8, EMPTY);
                for i in 0..15u32 {
                    w32(c, this + 0x20 + 0x188 * i + 0x184, vt);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn game_module_quick_restart_matches() {
        check(
            ee::s06::f_002302a8__generated,
            game_module_quick_restart,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x2302, 0);
                env(c);
                let this = DATA + 0x400;
                let race = DATA + 0x1000;
                let n = (seed % 4) as u32;
                w32(c, this + 0xc, race);
                w32(c, race + 0x78, n);
                for i in 0..n {
                    w32(c, race + 0x28 + 4 * i, DATA + 0x1800 + 0x40 * i);
                }
                w32(c, gpat(0x410), 0);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn c_fe_state_lodge_ctor_matches() {
        check(
            ee::s04::f_001f3700__generated,
            c_fe_state_lodge_ctor,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1f37, 0);
                env(c);
                let this = DATA + 0x400;
                w32(
                    c,
                    this + 0x10,
                    if seed % 2 == 0 { 0 } else { DATA + 0x1000 },
                );
                w32(
                    c,
                    DATA + 0x1000,
                    if seed % 3 == 0 { 0 } else { DATA + 0x1100 },
                );
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x800);
            },
        );
    }

    #[test]
    fn input_parser_error_matches() {
        check(
            ee::s10::f_00321500__generated,
            input_parser_error,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3215, 0);
                env(c);
                let this = DATA + 0x400;
                let fmt = DATA + 0x800;
                cstr(c, fmt, "%d %d %d %d %d %d");
                w32(c, this + 0x8c, r.next_u32());
                w32(c, this + 0xc, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(fmt);
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = r.next_u32() as u64;
                c.g[T1][0] = (seed % 7) as u64;
                c.g[T2][0] = r.next_u32() as u64;
                c.g[T3][0] = r.next_u32() as u64;
                for i in 0..7 {
                    c.f[12 + i] = r.edge_f32();
                }
            },
        );
    }

    #[test]
    fn manual_cam_controller_ctor_matches() {
        check(
            ee::s01::f_0015cd80__generated,
            manual_cam_controller_ctor,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x15cd, 0);
                env(c);
                let this = DATA + 0x400;
                w32(c, this + 0x10, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x800);
            },
        );
    }

    #[test]
    fn script_sound_bank_man_add_bank_matches() {
        check(
            ee::s07::f_00283580__generated,
            script_sound_bank_man_add_bank,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x2835, 0);
                env(c);
                let this = DATA + 0x400;
                let entries = DATA + 0x1000;
                let id = (seed % 3) as u32;
                if seed % 2 == 0 {
                    w32(c, this + 4, 1);
                    w32(c, this + 8, entries);
                    w32(c, entries, id);
                    w32(c, entries + 8, 1);
                } else {
                    w32(c, this + 4, 1);
                    w32(c, this + 8, entries);
                    w32(c, entries, id + 10);
                    w32(c, entries + 8, 0);
                }
                w32(c, gpat(0x410), 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = id as u64;
                c.g[A2][0] = sx(DATA + 0x2000);
                c.g[A3][0] = (seed % 4) as u64;
            },
        );
    }

    #[test]
    fn c_fe_options_credits_handle_input_matches() {
        check(
            ee::s02::f_001863e8__generated,
            c_fe_options_credits_handle_input,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1863, 0);
                env(c);
                let this = DATA + 0x400;
                let pad = DATA + 0x800;
                let vt = DATA + 0x1000;
                slot(c, vt, 0x88, if seed % 2 == 0 { RET_ZERO } else { EMPTY });
                slot(c, vt, 0x90, if seed % 3 == 0 { RET_ZERO } else { EMPTY });
                w32(c, pad + 8, vt);
                w32(
                    c,
                    this + 0x64,
                    [0i32, 1, -7, -8, 7, 8, 9][(seed % 7) as usize] as u32,
                );
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(pad);
            },
        );
    }

    #[test]
    fn c_fe_state_rider_bio_on_widget_action_matches() {
        check(
            ee::s02::f_001912d0__generated,
            c_fe_state_rider_bio_on_widget_action,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1912, 0);
                env(c);
                let this = DATA + 0x400;
                let item = DATA + 0x800;
                let engine = DATA + 0x1000;
                let obj = DATA + 0x1100;
                let vt = DATA + 0x1200;
                slot(c, vt, 0x28, if seed % 2 == 0 { RET_ZERO } else { EMPTY });
                w32(c, this + 0x10, engine);
                w32(c, engine, obj);
                w32(c, obj + 4, vt);
                w8(c, item + 0x319, seed % 4);
                w32(c, item + 0x18, seed % 8);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 5 == 0 { 0 } else { sx(item) };
                c.g[A2][0] = [6u64, 9, 0, 3][(seed % 4) as usize];
            },
        );
    }

    #[test]
    fn camera_trigger_man_dtor_matches() {
        check(
            ee::s02::f_0016c888__generated,
            camera_trigger_man_dtor,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x16c8, 0);
                env(c);
                let this = DATA + 0x400;
                w32(c, this + 4, (seed % 2) as u32);
                w32(c, this + 0x14, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn net_fe_state_on_widget_action_matches() {
        check(
            ee::s03::f_001a8918__generated,
            net_fe_state_on_widget_action,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1a89, 0x18);
                env(c);
                let this = DATA + 0x400;
                let item = DATA + 0x800;
                let vt = DATA + 0x1000;
                slot(c, vt, 0xc8, EMPTY);
                w32(c, this + 0x10, DATA + 0x1100);
                w32(c, this + 8, vt);
                w32(c, item + 0x18, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { sx(item) };
                c.g[A2][0] = [7u64, 0, 9][(seed % 3) as usize];
            },
        );
    }

    #[test]
    fn net_fe_state_show_max_buddies_popup_matches() {
        check(
            ee::s03::f_001abed8__generated,
            net_fe_state_show_max_buddies_popup,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1abe, 0);
                env(c);
                let this = DATA + 0x400;
                let popup = DATA + 0x1000;
                w32(c, this + 0x6a0, popup);
                w32(c, this + 0x40, 0);
                w32(c, popup + 0x2b8, 0);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn c_fe_use_existing_account_init_matches() {
        check(
            ee::s03::f_001ac0b8__generated,
            c_fe_use_existing_account_init,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1ac0, 0);
                env(c);
                let this = DATA + 0x400;
                w32(
                    c,
                    this + 0x10,
                    if seed % 2 == 0 { 0 } else { DATA + 0x1000 },
                );
                w32(c, this + 0x40, 0);
                w32(c, gpat(-0xc8), DATA + 0x1800);
                c.g[A0][0] = sx(this);
            },
        );
    }
}
