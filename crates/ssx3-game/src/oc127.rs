//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for cold front-end, audio, script-engine, UI, collision and
//! crowd-management helpers, each with a differential test against the
//! generated body.
//!
//! Most of these are small virtual-call wrappers or switch handlers: the call
//! protocol, cycle charges and the registers the generated code leaves behind
//! (notably `v0`/`v1`) are reproduced exactly. Collision functions
//! (`CollSphereTree_*`, `CollHull_*`) issue the original's VU0 macro ops so VF,
//! ACC, Q and the VU0 flags end as the generated body leaves them.

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, Callee, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x001a_f688, cfe_state_challenge_popup_show_set_options, "cFEStateChallengePopup_ShowSetOptions"),
    (0x001b_3f78, cfe_state_net_config_ctor, "cFEStateNetConfig_ctor"),
    (0x001b_a178, cfe_state_create_step1_backup_account_info, "cFEStateCreateStep1_BackupAccountInfo"),(0x0026_2e20, buddy_man_find_challenger_index, "BuddyMan_FindChallengerIndex"),
    (0x0026_6d58, c_voip_man_start, "cVoipMan_Start"),
    (0x002a_45c0, ssx_audio_system_record_race_result, "SSXAudioSystem_RecordRaceResult"),(0x0027_b6b0, script_engine_on_filter_loaded, "ScriptEngine_OnFilterLoaded"),
    (0x0027_d4a0, script_engine_deactivate_binding, "ScriptEngine_DeactivateBinding"),
    (0x002b_1f78, song_instance_dtor, "SongInstance_dtor"),
    (0x002c_c460, menu_sys_pop_page, "MenuSys_PopPage"),
    (0x0027_3d20, in_seq_channel_flush_events, "InSeq_ChannelFlushEvents"),
    (0x0023_e320, mc_manager_state36, "MCManager_State36"),(0x0024_54f8, c_game_load_state_transition_in, "cGameLoadState_TransitionIn"),
    (0x0035_7358, magnet_modifier_ctor_2, "MagnetModifier_ctor_2"),
    (0x0017_d008, debug_menu_set_game_mode, "DebugMenu_SetGameMode"),
    (0x0013_ad20, rider_railslide_state_enter, "RiderRailslideState_Enter"),
    (0x0028_a148, snd_load_queue_dtor, "SndLoadQueue_dtor"),
    (0x0019_e498, fe_module_rider_preview_dtor, "FEModule_RiderPreviewDtor"),
    (0x0023_67c8, game_state_event_select_load_state, "GameStateEventSelect_LoadState"),(0x001f_e648, ov_screen_save_replay_v39, "OVScreenSaveReplay_v39"),(0x0015_89b0, be_reward_int_get_collectible_tier, "BERewardInt_GetCollectibleTier"),
    (0x0015_8a50, be_reward_int_get_mission_tier, "BERewardInt_GetMissionTier"),
    (0x0014_5ef0, be_new_player_int_has_event_record, "BENewPlayerInt_HasEventRecord"),
    (0x0033_0788, coll_sphere_tree_collide_triangle, "CollSphereTree_CollideTriangle"),
    (0x0033_0828, coll_sphere_tree_collide_triangle2_sided, "CollSphereTree_CollideTriangle2Sided"),
    (0x0032_cb58, sphere_tree_collide_sphere_tree, "SphereTree_CollideSphereTree"),
    (0x0032_f990, coll_hull_collide_triangle2_sided, "CollHull_CollideTriangle2Sided"),
    (0x0039_c688, ui_screen_set_data, "UIScreen_SetData"),
    (0x0039_e6b8, ui_state_render, "UIState_Render"),
    (0x0038_e008, light_man_v12, "LightMan_v12"),
    (0x0034_2dd8, debounce_kill, "Debounce_Kill"),
    (0x0034_4240, crowd_man3d_load_state, "CrowdMan3d_LoadState"),
    (0x0023_4a30, game_state_pre_race_load_load_state, "GameStatePreRaceLoad_LoadState"),
    (0x001f_7e68, ov_screen_top_times_on_widget_created, "OVScreenTopTimes_OnWidgetCreated"),
    (0x0014_fe08, be_bagt_int_clear_uber_trick_bit, "BEBAGTInt_ClearUberTrickBit"),(0x001d_2ea0, cfe_state_mountain_room_handle_input, "cFEStateMountainRoom_HandleInput"),(0x0039_f718, ui_state_mgr_close_top_state_and_popups, "UIStateMgr_CloseTopStateAndPopups"),
    (0x0015_3520, be_mission_int_get_track_collected_count, "BEMissionInt_GetTrackCollectedCount"),(0x001a_b3d0, net_fe_state_refresh_buddy_action_popup, "NetFEState_RefreshBuddyActionPopup"),
    (0x001a_c448, cfe_use_existing_account_check_email, "cFEUseExistingAccount_CheckEmail"),
];

// ---------------------------------------------------------------------------
// Callees the generator emitted as direct calls (the readable alias, which
// dispatches through the override slot) and as cross-shard `call_indirect`s.

mod callee {
    use super::{ee, Callee};

    pub const FE_ON_ABORT_POPUP_EVENT: Callee = Callee { addr: 0x001a_ca88, f: ee::s03::f_001aca88 };
    pub const FE_ON_EMAIL_POPUP_EVENT: Callee = Callee { addr: 0x001a_c9c0, f: ee::s03::f_001ac9c0 };
    pub const FE_ON_LOGIN_POPUP_EVENT: Callee = Callee { addr: 0x001a_cb48, f: ee::s03::f_001acb48 };
    pub const FE_ON_SAVE_ACCOUNT_ANSWER: Callee = Callee { addr: 0x001a_ccd0, f: ee::s03::f_001accd0 };
    pub const NET_FE_STATE_ON_CHILD_EVENT: Callee = Callee { addr: 0x001a_97b8, f: ee::s03::f_001a97b8 };
    pub const CHALLENGE_POPUP_SET_SETTINGS_LOCKED: Callee =
        Callee { addr: 0x001a_ec60, f: ee::s03::f_001aec60 };
    pub const NET_FE_STATE_CTOR: Callee = Callee { addr: 0x001a_8500, f: ee::s03::f_001a8500 };
    pub const VOIP_MAN_DEACTIVATE: Callee = Callee { addr: 0x0026_6e88, f: ee::s07::f_00266e88 };
    pub const VOIP_MAN_ACTIVATE: Callee = Callee { addr: 0x0026_6e30, f: ee::s07::f_00266e30 };
    pub const AUDIO_RESET_RACE_RESULT_STATE: Callee =
        Callee { addr: 0x002a_4590, f: ee::s08::f_002a4590 };
    pub const AUDIO_IS_TRACK_GROUP0_EVENT: Callee =
        Callee { addr: 0x002a_4078, f: ee::s08::f_002a4078 };
    pub const AUDIO_STORE_RACE_RESULT: Callee = Callee { addr: 0x002a_4660, f: ee::s08::f_002a4660 };
    pub const INI_SKIP_WHITESPACE: Callee = Callee { addr: 0x002a_78f8, f: ee::s08::f_002a78f8 };
    pub const INI_GET_CHAR: Callee = Callee { addr: 0x002a_78e0, f: ee::s08::f_002a78e0 };
    pub const BE_GET_RIDER_PROFILE: Callee = Callee { addr: 0x0014_a0e0, f: ee::s01::f_0014a0e0 };
    pub const BE_GET_RIDER_CHARACTER: Callee = Callee { addr: 0x0014_a080, f: ee::s01::f_0014a080 };
    pub const BE_COUNT_COLLECTED_ON_PEAK: Callee = Callee { addr: 0x0015_3708, f: ee::s01::f_00153708 };
    pub const BE_COUNT_COMPLETED_ON_PEAK: Callee = Callee { addr: 0x0015_44d0, f: ee::s01::f_001544d0 };
    pub const POPUP_CONFIRM_GET_SELECTION: Callee = Callee { addr: 0x001c_a150, f: ee::s03::f_001ca150 };
    pub const SETUP_BUDDY_ACTION_POPUP: Callee = Callee { addr: 0x001a_b478, f: ee::s03::f_001ab478 };
    pub const NET_FE_STATE_GET_POPUP: Callee = Callee { addr: 0x001a_9c50, f: ee::s03::f_001a9c50 };
    pub const NET_FE_STATE_SHOW_POPUP: Callee = Callee { addr: 0x001a_9bc0, f: ee::s03::f_001a9bc0 };
    pub const POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID: Callee =
        Callee { addr: 0x001c_57c0, f: ee::s03::f_001c57c0 };
    pub const MEM_CARD_STATE_BASE_SHUTDOWN: Callee =
        Callee { addr: 0x001d_58b8, f: ee::s04::f_001d58b8 };
    pub const DEAD_NODE_CTOR: Callee = Callee { addr: 0x0035_06d8, f: ee::s11::f_003506d8 };
    pub const CROWD_MAN3D_RESET: Callee = Callee { addr: 0x0034_3bc0, f: ee::s11::f_00343bc0 };
    pub const CROWD_MAN3D_GROUP_LOAD: Callee = Callee { addr: 0x0034_4ff8, f: ee::s11::f_00344ff8 };
    pub const SPHERE_TREE_COLLIDE_TREE_NODE: Callee =
        Callee { addr: 0x0032_cbf8, f: ee::s10::f_0032cbf8 };
    pub const FEMODULE_RIDER_PREVIEW_UNLOAD: Callee =
        Callee { addr: 0x0019_eba0, f: ee::s03::f_0019eba0 };
    pub const RIDER_UPDATE_BOARD_LEAN: Callee = Callee { addr: 0x0011_fa10, f: ee::s00::f_0011fa10 };
    pub const RIDER_BUILD_WORLD_MATRIX: Callee = Callee { addr: 0x0011_e098, f: ee::s00::f_0011e098 };
    pub const GAME_STATE_LOAD_STATE: Callee = Callee { addr: 0x0023_3b28, f: ee::s06::f_00233b28 };
    pub const MC_MANAGER_ON_CARD_SCANNED: Callee = Callee { addr: 0x0024_1fd0, f: ee::s06::f_00241fd0 };
    pub const GAME_LOAD_UPDATE_PLAYER_RANKING: Callee =
        Callee { addr: 0x0024_6cf8, f: ee::s06::f_00246cf8 };
    pub const LIGHT_MAN_EMPTY_HOOK_D: Callee = Callee { addr: 0x0038_dc68, f: ee::s12::f_0038dc68 };
    pub const BE_GET_CHAR_PROFILE_EVENT_RECORD: Callee =
        Callee { addr: 0x0015_2398, f: ee::s01::f_00152398 };
    pub const BE_NEW_PLAYER_INT_GET_SLOT: Callee = Callee { addr: 0x0014_7798, f: ee::s01::f_00147798 };
    pub const MENU_SYS_SET_ACTIVE: Callee = Callee { addr: 0x002c_c0b8, f: ee::s09::f_002cc0b8 };
    pub const MPEG2_GET_BITS: Callee = Callee { addr: 0x003b_1300, f: ee::s13::f_003b1300 };
    pub const MPEG2_MARKER_BIT: Callee = Callee { addr: 0x003b_2360, f: ee::s13::f_003b2360 };
    pub const SND_LOAD_QUEUE_FLUSH: Callee = Callee { addr: 0x0028_a230, f: ee::s07::f_0028a230 };
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
mod table {
    pub const STR_HASH: u32 = 0x0031_7670;
    pub const UI_SCREEN_FIND_OBJECT: u32 = 0x0039_d860;
    pub const UI_TEXT_SET_TEXT_ID: u32 = 0x003a_1000;
    pub const BXSTRING_ASSIGN: u32 = 0x0031_8448;
    pub const BXSTRING_RELEASE: u32 = 0x0031_80b8;
    pub const UI_MENU_RESET_SCROLL: u32 = 0x0039_b760;
    pub const PS2_MEM_CARD_CREATE: u32 = 0x002c_22c8;
    pub const PATH_STOP_TRACKS: u32 = 0x003d_2a68;
    pub const EA_MEM_FREE: u32 = 0x0031_7e98;
    pub const BUILTIN_DELETE: u32 = 0x0031_7e50;
    pub const VOIP_RPC_START: u32 = 0x003f_6ab8;
    pub const VOIP_RPC_SET_PARAMS: u32 = 0x003f_6c10;
    pub const AUDIO_GET_RACE_MAN: u32 = 0x0028_b1d8;
    pub const ASYNC_FILE_GET_RESULT: u32 = 0x003d_f9d8;
    pub const SCRIPT_ENGINE_GET_RESULT_VTABLE: u32 = 0;
    pub const RIDER_MODEL_FIND: u32 = 0;
    pub const BE_GET_INSTANCE: u32 = 0x0014_dc80;
    pub const BE_GET_INTERFACE: u32 = 0x0014_dd58;
    pub const BE_NEW_RACE_SET_GAME_MODE: u32 = 0x0014_5108;
    pub const BE_NEW_RACE_SET_EVENT_TYPE: u32 = 0x0014_4df0;
    pub const REPLAY_END_SAVE: u32 = 0x0027_0ed8;
    pub const REPLAY_BEGIN_PLAYBACK: u32 = 0x0026_f8a0;
    pub const EA_MEM_ALLOC_NAMED: u32 = 0x0031_7d70;
    pub const OPERATOR_NEW_TAG: u32 = 0x0031_7e30;
    pub const UI_LIST_FIRST: u32 = 0x0039_7840;
    pub const UI_LIST_NODE_IS_SENTINEL: u32 = 0x0039_7718;
    pub const UI_TEXT_SET_WIDE_STRING: u32 = 0x003a_0e90;
    pub const SPRINTF: u32 = 0x002c_27c0;
    pub const EAC_GET_UNPACKED_SIZE: u32 = 0x003b_4818;
    pub const EAC_UNPACK: u32 = 0x003b_47f8;
    pub const COLL_SPHERE_TREE_TRIANGLE: u32 = 0x0032_ca78;
    pub const COLL_HULL_TRIANGLE: u32 = 0x0032_8030;
    pub const COLL_HULL_TRIANGLE2: u32 = 0x0032_aa28;
    pub const COLL_SPHERE_CALC: u32 = 0x0032_df28;
    pub const STR_CASECMP: u32 = 0x0041_aa88;
}

// ---------------------------------------------------------------------------
// Small helpers.

/// `reg + off`.
#[inline(always)]
fn at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `lw reg + off`.
#[inline(always)]
fn lw(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(at(c, reg, off)).lw(c)
}

/// `sw reg + off, value`.
#[inline(always)]
fn sw(c: &mut Cpu, reg: usize, off: u32, v: u64) {
    Addr(at(c, reg, off)).sw(c, v);
}

// ---------------------------------------------------------------------------
// Front-end / online state handlers.

/// `void cFEUseExistingAccount_OnChildEvent(self, child)` (0x1ac920): the
/// child event's id at `child+0x18` selects one of the four account popup
/// handlers (0x80E abort, 0x80F email, 0x81E login, 0x82C save answer);
/// anything else is forwarded to `NetFEState_OnChildEvent`.
pub fn cfe_use_existing_account_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x80f;
        frame.save_ra(c, 0);
        let event = Addr(abi::arg(c, 1)).off(0x18).lw(c);
        c.g[V1][0] = event;
        let below = (event < 0x810) as u64;
        c.g[V0][0] = below;
        if event == 0x80f {
            c.cycles += 4;
            callee::FE_ON_EMAIL_POPUP_EVENT.call(c, 0x001a_c984)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0x80e;
            if below != 0 {
                c.cycles += 2;
                if event == 0x80e {
                    c.cycles += 4;
                    callee::FE_ON_ABORT_POPUP_EVENT.call(c, 0x001a_c974)?;
                    frame.restore_ra(c, 0);
                } else {
                    c.cycles += 2;
                    c.cycles += 3;
                    callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, 0x001a_c9b4)?;
                    frame.restore_ra(c, 0);
                }
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x81e;
                let login = event == 0x81e;
                c.g[V0][0] = 0x82c;
                if login {
                    c.cycles += 4;
                    callee::FE_ON_LOGIN_POPUP_EVENT.call(c, 0x001a_c994)?;
                    frame.restore_ra(c, 0);
                } else {
                    c.cycles += 2;
                    if event == 0x82c {
                        c.cycles += 4;
                        callee::FE_ON_SAVE_ACCOUNT_ANSWER.call(c, 0x001a_c9a4)?;
                        frame.restore_ra(c, 0);
                    } else {
                        c.cycles += 2;
                        c.cycles += 3;
                        callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, 0x001a_c9b4)?;
                        frame.restore_ra(c, 0);
                    }
                }
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateChallengePopup_ShowSetOptions(self)` (0x1af688): reset the
/// settings lock, refresh the popup's unicode strings and re-apply the
/// settings-locked state.
pub fn cfe_state_challenge_popup_show_set_options(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        let this = abi::arg(c, 0);
        c.g[S1][0] = sx(this);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x0046_3468;
        Addr(this).off(0x6d0).sw(c, 0);
        call_ptr(c, table::STR_HASH, 0x001a_f6ac)?;
        c.g[A0][0] = lw(c, S1, 0x40);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, table::UI_SCREEN_FIND_OBJECT, 0x001a_f6b8)?;
        c.g[S0][0] = c.g[V0][0];
        let found = c.g[S0][0] == 0;
        c.g[A0][0] = 0x0046_0000;
        if !found {
            c.cycles += 20;
            c.g[A0][0] = 0x0046_3498;
            call_ptr(c, table::STR_HASH, 0x001a_f6cc)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, table::UI_TEXT_SET_TEXT_ID, 0x001a_f6d8)?;
            c.g[A0][0] = 0x0046_31c8;
            call_ptr(c, table::STR_HASH, 0x001a_f6e4)?;
            c.g[A0][0] = lw(c, S1, 0x40);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, table::UI_SCREEN_FIND_OBJECT, 0x001a_f6f0)?;
            c.g[V1][0] = lw(c, V0, 0x8);
            c.g[A1][0] = 0;
            c.g[A0][0] = Addr(at(c, V1, 0x40)).lh(c);
            c.g[A2][0] = lw(c, V1, 0x44);
            let target = c.g[A2][0] as u32;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001a_f708)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0;
            callee::CHALLENGE_POPUP_SET_SETTINGS_LOCKED.call(c, 0x001a_f714)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateNetConfig* cFEStateNetConfig_ctor(this)` (0x1b3f78): base
/// constructor, the state's vtable and lock fields, the mem-card handler.
pub fn cfe_state_net_config_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        callee::NET_FE_STATE_CTOR.call(c, 0x001b_3f90)?;
        c.g[V1][0] = 0x0046_8b00;
        sw(c, S0, 0x6d0, 0);
        c.g[V0][0] = 0x31;
        sw(c, S0, 0x6d4, 0);
        sw(c, S0, 0x6d8, 0);
        sw(c, S0, 0x6e4, 0);
        sw(c, S0, 0x6e8, 0);
        sw(c, S0, 0x6f4, 0);
        sw(c, S0, 0x6f8, 0);
        sw(c, S0, 0x6fc, 0);
        sw(c, S0, 0x700, 0);
        sw(c, S0, 0x704, 0);
        sw(c, S0, 0x708, 0);
        sw(c, S0, 0x710, 0);
        sw(c, S0, 0x714, 0);
        sw(c, S0, 0x718, 0);
        sw(c, S0, 0x6a8, 0);
        sw(c, S0, 0x8, c.g[V1][0]);
        sw(c, S0, 0xc, c.g[V0][0]);
        call_ptr(c, table::PS2_MEM_CARD_CREATE, 0x001b_3fe4)?;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        Addr(gp(c, -0x1578)).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(at(c, V1, 0x10)).lh(c);
        c.g[A1][0] = lw(c, V1, 0x14);
        let target = c.g[A1][0] as u32;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001b_3ffc)?;
        c.g[V1][0] = u64::MAX;
        c.g[V0][0] = c.g[S0][0];
        sw(c, S0, 0x6e0, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCreateStep1_BackupAccountInfo(self)` (0x1ba178): copy the
/// three account strings out of `self->account` (or release them when
/// already backed up) and clear the account's flags.
pub fn cfe_state_create_step1_backup_account_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        let already = lw(c, S0, 0x6ec) != 0;
        c.g[A0][0] = sx(this.wrapping_add(0x6d4));
        if already {
            c.cycles += 6;
            c.g[A0][0] = sx(this.wrapping_add(0x6d4));
            call_ptr(c, table::BXSTRING_RELEASE, 0x001b_a1e0)?;
            c.g[A0][0] = sx(this.wrapping_add(0x6d8));
            call_ptr(c, table::BXSTRING_RELEASE, 0x001b_a1e8)?;
            c.g[A0][0] = sx(this.wrapping_add(0x6dc));
            call_ptr(c, table::BXSTRING_RELEASE, 0x001b_a1f0)?;
        } else {
            c.cycles += 17;
            c.g[A1][0] = sx((lw(c, S0, 0x6d0) as u32).wrapping_add(0x4));
            call_ptr(c, table::BXSTRING_ASSIGN, 0x001b_a1a0)?;
            c.g[A1][0] = sx((lw(c, S0, 0x6d0) as u32).wrapping_add(0x8));
            c.g[A0][0] = sx(this.wrapping_add(0x6d8));
            call_ptr(c, table::BXSTRING_ASSIGN, 0x001b_a1b0)?;
            c.g[A1][0] = sx((lw(c, S0, 0x6d0) as u32).wrapping_add(0xc));
            c.g[A0][0] = sx(this.wrapping_add(0x6dc));
            call_ptr(c, table::BXSTRING_ASSIGN, 0x001b_a1c0)?;
            c.g[A0][0] = lw(c, S0, 0x6d0);
            c.g[V0][0] = lw(c, A0, 0x38);
            sw(c, S0, 0x6e0, c.g[V0][0]);
            c.g[V1][0] = lw(c, A0, 0x3c);
            sw(c, S0, 0x6e4, c.g[V1][0]);
        }
        c.cycles += 10;
        c.g[V0][0] = lw(c, S0, 0x6d0);
        c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
        sw(c, V0, 0x4c, 0);
        c.g[V1][0] = lw(c, S0, 0x6d0);
        sw(c, V1, 0x44, 0);
        c.g[V0][0] = lw(c, S0, 0x6d0);
        frame.restore(c, S0, 0x10);
        sw(c, V0, 0x48, 0);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = c.g[RA][0] as u32;
        return Ok(());
    })
}

/// `void cFEPopupSelectMultiplayerMode_OnWidgetCreated(self, item)` (0x1e0358):
/// the item hash selects one of three widget ids: show/refresh the
/// multiplayer list, clear a flag, or set it.
pub fn cfe_popup_select_multiplayer_mode_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0x0046_2000;
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        let item = abi::arg(c, 1);
        c.g[S1][0] = sx(item);
        frame.save_ra(c, 0);
        c.g[S0][0] = lw(c, S1, 0x38);
        call_ptr(c, table::STR_HASH, 0x001e_037c)?;
        let first = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = 0x004a_0000;
        if !first {
            c.cycles += 10;
            c.g[V0][0] = lw(c, S1, 0x14);
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 2;
            c.g[V0][0] |= 1;
            sw(c, S1, 0x14, c.g[V0][0]);
            call_ptr(c, table::UI_MENU_RESET_SCROLL, 0x001e_039c)?;
            c.g[V0][0] = lw(c, S1, 0x14) | 0x80;
            sw(c, S1, 0x14, c.g[V0][0]);
        } else {
            c.cycles += 5;
            c.g[S0][0] = lw(c, S1, 0x38);
            c.g[A0][0] = 0x004a_13a0;
            call_ptr(c, table::STR_HASH, 0x001e_03b8)?;
            if c.g[S0][0] == c.g[V0][0] {
                c.cycles += 2;
                sw(c, S1, 0x18, 0);
            } else {
                c.cycles += 5;
                c.g[S0][0] = lw(c, S1, 0x38);
                c.g[A0][0] = 0x004a_14f8;
                call_ptr(c, table::STR_HASH, 0x001e_03d4)?;
                let other = c.g[S0][0] != c.g[V0][0];
                frame.restore(c, S0, 0x20);
                if other {
                    c.cycles += 4;
                    frame.restore(c, S1, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                c.cycles += 2;
                c.g[V0][0] = 1;
                sw(c, S1, 0x18, c.g[V0][0]);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int BuddyMan_FindChallengerIndex(self, name)` (0x262e20): scan the buddy
/// list for the entry whose name matches (strcasecmp), -1 if absent.
pub fn buddy_man_find_challenger_index(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x10);
        let this = abi::arg(c, 0);
        c.g[S1][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, S1, 0xc0);
        c.g[V1][0] = lw(c, S1, 0xbc);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
        let empty = c.g[V0][0] == 0;
        c.g[S2][0] = c.g[A1][0];
        let mut out: i64 = -1;
        if !empty {
            c.cycles += 1;
            c.g[V0][0] = lw(c, S1, 0xbc);
            'scan: loop {
                c.cycles += 5;
                c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = lw(c, S1, 0xc0);
                } else {
                    c.cycles += 5;
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
                    c.g[A1][0] = c.g[S2][0];
                    call_ptr(c, table::STR_CASECMP, 0x0026_2e78)?;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = lw(c, S1, 0xc0);
                    } else {
                        c.cycles += 2;
                        out = c.g[S0][0] as i64;
                        break 'scan;
                    }
                }
                c.cycles += 7;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[V1][0] = lw(c, S1, 0xbc);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = lw(c, S1, 0xbc);
                    if event_due(c) {
                        deliver_event_here(c, 0x0026_2e58);
                    }
                    continue 'scan;
                }
                break 'scan;
            }
            if out >= 0 {
                c.g[V0][0] = out as u64;
            }
        }
        if out < 0 {
            c.cycles += 1;
            c.g[V0][0] = u64::MAX;
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

/// `void cVoipMan_Start(self, rpc)` (0x266d58): start the voice RPC, set its
/// parameters, then either deactivate (few failures) or, unless the voip
/// capability bit is clear, activate.
pub fn c_voip_man_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        c.g[V0][0] = Addr(this).lw(c);
        let running = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[A1][0];
        if !running {
            c.cycles += 12;
            c.g[A1][0] = 0x1770;
            c.g[A2][0] = 0x1770;
            call_ptr(c, table::VOIP_RPC_START, 0x0026_6d80)?;
            c.g[V0][0] = 1;
            c.g[A0][0] = Addr(gp(c, 0x308)).lw(c);
            Addr(this).sw(c, c.g[V0][0]);
            c.g[A1][0] = 0x55;
            call_ptr(c, table::VOIP_RPC_SET_PARAMS, 0x0026_6d94)?;
            c.g[V0][0] = Addr(gp(c, 0x308)).lw(c);
            c.g[V0][0] = ((c.g[V0][0] as i64) < 10) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 11;
                call_ptr(c, table::BE_GET_INSTANCE, 0x0026_6dbc)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 4;
                call_ptr(c, table::BE_GET_INTERFACE, 0x0026_6dc8)?;
                c.g[V1][0] = 0x0053_0000;
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x5610).lw(c);
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 18) as u32);
                c.g[V0][0] &= 1;
                if c.g[V0][0] == 0 {
                    frame.restore(c, S0, 0x10);
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    callee::VOIP_MAN_ACTIVATE.call(c, 0x0026_6de8)?;
                    c.cycles += 1;
                    frame.restore(c, S0, 0x10);
                }
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            callee::VOIP_MAN_DEACTIVATE.call(c, 0x0026_6dac)?;
            frame.restore(c, S0, 0x10);
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_RecordRaceResult(self)` (0x2a45c0): if the race is
/// still the current result slot, reset it; then store the track event result
/// for the current rider.
pub fn ssx_audio_system_record_race_result(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        let this = abi::arg(c, 0);
        c.g[S1][0] = sx(this);
        frame.save(c, S2, 0x10);
        c.g[A0][0] = 1;
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, A1, 0xc0);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        let not_current = c.g[V1][0] != c.g[A0][0];
        c.g[V0][0] = 0x0053_0000;
        if !not_current {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0;
            callee::AUDIO_RESET_RACE_RESULT_STATE.call(c, 0x002a_45fc)?;
            c.g[V0][0] = 0x0053_0000;
        }
        c.cycles += 7;
        c.g[S2][0] = Addr(c.g[V0][0] as u32).off(0x6730).lw(c);
        call_ptr(c, table::AUDIO_GET_RACE_MAN, 0x002a_4608)?;
        c.g[S0][0] = lw(c, V0, 0x28);
        c.g[A0][0] = c.g[S1][0];
        callee::AUDIO_IS_TRACK_GROUP0_EVENT.call(c, 0x002a_4614)?;
        let event = c.g[V0][0] != 0;
        if !event {
            c.g[V1][0] = lw(c, S0, 0x790);
            c.cycles += 1;
            c.g[A2][0] = 0;
        } else {
            c.cycles += 4;
            c.g[V0][0] = lw(c, S0, 0x790);
            c.g[V1][0] = c.g[V0][0];
            c.g[A2][0] = lw(c, V0, 0x128);
        }
        c.cycles += 12;
        c.g[T0][0] = lw(c, S0, 0x480);
        c.g[A0][0] = c.g[S1][0];
        c.g[A3][0] = lw(c, V1, 0x114);
        c.g[A1][0] = c.g[S2][0];
        c.g[T0][0] ^= 1;
        c.g[RA][0] = sx(0x002a_4648);
        c.pc = 0x002a_4660;
        (callee::AUDIO_STORE_RACE_RESULT.f)(c);
        if c.pc != 0x002a_4648 {
            return Err(Unwound);
        }
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `char IniParser_Expect(parser, name)` (0x2a7ef0): consume the expected
/// token, returning 0 on a match, 1 on a mismatch, or the offending char at
/// end of input.
pub fn ini_parser_expect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        let parser = abi::arg(c, 0);
        c.g[S1][0] = sx(parser);
        frame.save(c, S2, 0x20);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[S2][0] = 0;
        callee::INI_SKIP_WHITESPACE.call(c, 0x002a_7f18)?;
        c.g[V1][0] = lw(c, S1, 0x8);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lb(c);
        let at_end = c.g[V0][0] == 0;
        c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S2][0] as u32));
        let mut result: u64 = 0;
        if at_end {
            c.cycles += 1;
            result = Addr(c.g[V1][0] as u32).lb(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S3][0] as u32).lb(c);
            let name_end = c.g[V0][0] == 0;
            c.g[S0][0] = c.g[S3][0];
            if name_end {
                c.cycles += 1;
                c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S2][0] as u32));
                result = Addr(c.g[V1][0] as u32).lb(c);
            } else {
                c.cycles += 1;
                'expect: loop {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S1][0];
                    callee::INI_GET_CHAR.call(c, 0x002a_7f40)?;
                    c.g[V1][0] = Addr(c.g[S0][0] as u32).lb(c);
                    if c.g[V0][0] != c.g[V1][0] {
                        c.g[V0][0] = 1;
                        result = 1;
                        break 'expect;
                    }
                    c.cycles += 4;
                    c.g[V1][0] = lw(c, S1, 0x8);
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).lb(c);
                    let end = c.g[V0][0] == 0;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                    if end {
                        c.cycles += 1;
                        c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S2][0] as u32));
                        result = Addr(c.g[V1][0] as u32).lb(c);
                        break 'expect;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).lb(c);
                    let more = c.g[V1][0] != 0;
                    c.g[S0][0] = c.g[V0][0];
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x002a_7f38);
                        }
                        continue 'expect;
                    }
                    c.cycles += 1;
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    result = Addr(c.g[V1][0] as u32).lb(c);
                    break 'expect;
                }
                c.g[V0][0] = result;
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

/// `void ScriptEngine_OnFilterLoaded(self, file, data)` (0x27b6b0): take the
/// async filter result, bump the pending count and relocate each used
/// entry's offset.
pub fn script_engine_on_filter_loaded(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.g[A1][0];
        sw(c, S0, 0x538, 0);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, table::ASYNC_FILE_GET_RESULT, 0x0027_b6d4)?;
        c.g[V1][0] = lw(c, S0, 0x620);
        c.g[V0][0] = Addr(c.g[SP][0] as u32).off(4).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        let no_data = c.g[V0][0] == 0;
        sw(c, S0, 0x620, c.g[V1][0]);
        if !no_data {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[SP][0] as u32).lw(c);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
            sw(c, S0, 0x53c, c.g[V0][0]);
            sw(c, S0, 0x540, c.g[A0][0]);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            let empty = (c.g[V1][0] as i64) <= 0;
            c.g[A2][0] = 0;
            if !empty {
                c.cycles += 1;
                c.g[V0][0] = lw(c, S0, 0x540);
                'entries: loop {
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[A2][0] as u32) << 3);
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = Addr(c.g[A1][0] as u32).lw(c);
                    let skip = c.g[A0][0] == 0;
                    c.g[V0][0] = Addr(c.g[SP][0] as u32).lw(c);
                    if !skip {
                        c.cycles += 3;
                        c.g[V1][0] = Addr(c.g[A1][0] as u32).off(4).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        Addr(c.g[A1][0] as u32).off(4).sw(c, c.g[V0][0]);
                    }
                    c.cycles += 6;
                    c.g[V1][0] = lw(c, S0, 0x53c);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = lw(c, S0, 0x540);
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_b708);
                        }
                        continue 'entries;
                    }
                    break 'entries;
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptEngine_DeactivateBinding(self, key)` (0x27d4a0): clear the
/// binding slot for `key[1]` and notify its listeners.
pub fn script_engine_deactivate_binding(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x40);
        c.g[V1][0] = 0x14;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        let this = abi::arg(c, 0);
        c.g[S1][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[A0][0] = u64::MAX;
        c.g[V0][0] = Addr(abi::arg(c, 1)).off(4).lw(c);
        c.mult(0, V0, V0, V1);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x50));
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[S2][0] = lw(c, S0, 0x8);
        sw(c, S0, 0x8, c.g[A0][0]);
        sw(c, S0, 0xc, 0);
        c.g[A2][0] = c.g[S2][0];
        c.g[A1][0] = lw(c, S0, 0x4);
        c.g[V1][0] = lw(c, S1, 0x2a8);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x20).lh(c);
        c.g[V0][0] = lw(c, V1, 0x24);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0027_d4f8)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x30);
        } else {
            c.cycles += 11;
            c.g[V0][0] = lw(c, S0, 0x4);
            c.g[A1][0] = c.g[S2][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A2][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0xc).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x28).lh(c);
            c.g[V0][0] = lw(c, V1, 0x2c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0027_d528)?;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SongInstance* SongInstance_dtor(this, flags)` (0x2b1f78): cancel the
/// queued bank, stop the tracks and free the two owned buffers.
pub fn song_instance_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = Addr(gp(c, 0x6fc)).lw(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[A1][0] = lw(c, S0, 0x34);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        call_ptr(c, 0x0028_a728, 0x002b_1fa4)?;
        c.g[A0][0] = 0x1100_0000;
        c.g[A0][0] |= 1;
        call_ptr(c, table::PATH_STOP_TRACKS, 0x002b_1fb0)?;
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = lw(c, V1, 0x40);
        let no_tracks = (c.g[V0][0] as i64) <= 0;
        c.g[A0][0] = 0x1100_0000;
        if !no_tracks {
            c.cycles += 2;
            c.g[A0][0] |= 2;
            call_ptr(c, table::PATH_STOP_TRACKS, 0x002b_1fc8)?;
        }
        c.cycles += 3;
        c.g[A0][0] = lw(c, S0, 0x24);
        if c.g[A0][0] == 0 {
            c.g[A0][0] = lw(c, S0, 0x28);
        } else {
            c.cycles += 3;
            call_ptr(c, table::EA_MEM_FREE, 0x002b_1fdc)?;
            c.g[A0][0] = lw(c, S0, 0x28);
        }
        c.cycles += 2;
        let second_null = c.g[A0][0] == 0;
        c.g[V0][0] = c.g[S1][0] & 1;
        if !second_null {
            c.cycles += 3;
            call_ptr(c, table::EA_MEM_FREE, 0x002b_1ff0)?;
            c.g[V0][0] = c.g[S1][0] & 1;
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x002b_2004)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSys_PopPage(self)` (0x2cc460): dispose the top page and, if one
/// remains, activate its parent; otherwise clear the menu's active flag.
pub fn menu_sys_pop_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(this.wrapping_add(4));
        c.g[V0][0] = Addr(this).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        Addr(this).sw(c, c.g[V0][0]);
        c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A1][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[V0][0] = lw(c, A1, 0x12c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x38).lh(c);
        c.g[V1][0] = lw(c, V0, 0x3c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002c_c4a4)?;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        if !empty {
            c.cycles += 12;
            c.g[A1][0] = c.g[S0][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A3][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V1][0] = lw(c, A3, 0x12c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x30).lh(c);
            c.g[A2][0] = lw(c, V1, 0x34);
            let target = c.g[A2][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x002c_c4d4)?;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x20);
                c.cycles += 4;
                frame.restore(c, S1, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        callee::MENU_SYS_SET_ACTIVE.call(c, 0x002c_c4ec)?;
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// MenuSys_SetActive's direct callee is defined in `callee` above.

/// `int InSeq_ChannelFlushEvents(self)` (0x273d20): tell the channel's two
/// listeners to drop their events and clear the queued event count.
pub fn in_seq_channel_flush_events(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[T1][0] = lw(c, S0, 0x2c);
        let has = c.g[T1][0] != 0;
        c.g[S1][0] = lw(c, S0, 0x30);
        if has {
            c.cycles += 3;
            c.g[A3][0] = lw(c, S0, 0x34);
            let handler = c.g[A3][0] != 0;
            c.g[A1][0] = c.g[S0][0];
            if handler {
                c.cycles += 8;
                c.g[V0][0] = Addr(c.g[T1][0] as u32).lw(c);
                c.g[V1][0] = lw(c, S0, 0x1c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x18).lh(c);
                c.g[T0][0] = lw(c, V0, 0x1c);
                c.g[A2][0] = Addr(c.g[V1][0] as u32).off(3).lbu(c);
                c.mtc1(0, 12);
                let target = c.g[T0][0] as u32;
                c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0027_3d6c)?;
            }
            c.cycles += 12;
            c.g[A3][0] = lw(c, S0, 0x2c);
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = lw(c, S0, 0x1c);
            c.g[V0][0] = Addr(c.g[A3][0] as u32).lw(c);
            c.mtc1(0, 12);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x20).lh(c);
            c.g[V1][0] = lw(c, V0, 0x24);
            c.mov_s(13, 12);
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[A2][0] = Addr(c.g[T0][0] as u32).off(3).lbu(c);
            let target = c.g[V1][0] as u32;
            c.g[A3][0] = lw(c, S0, 0x34);
            call_ptr(c, target, 0x0027_3d9c)?;
        }
        c.cycles += 9;
        sw(c, S0, 0x38, 0);
        c.g[V0][0] = c.g[S1][0];
        sw(c, S0, 0x30, 0);
        sw(c, S0, 0x34, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State36(self)` (0x23e320): if the mem-card movie has no
/// pending scan, ask it whether this card is in the movie; either report the
/// card as scanned or run the scan callback.
pub fn mc_manager_state36(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[A1][0] = lw(c, S0, 0x434);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x58).lh(c);
        c.g[V0][0] = lw(c, V1, 0x5c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_e348)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 9;
            c.g[A2][0] = lw(c, S0, 0x434);
            c.g[A1][0] = lw(c, S0, 0x428);
            c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1a0).lh(c);
            c.g[V0][0] = lw(c, V1, 0x1a4);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_e36c)?;
            if c.g[V0][0] != 0 {
                c.g[A1][0] = 2;
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                callee::MC_MANAGER_ON_CARD_SCANNED.call(c, 0x0023_e3a4)?;
                c.g[V0][0] = lw(c, S0, 0x98);
                let target = c.g[V0][0] as u32;
                call_ptr(c, target, 0x0023_e3b0)?;
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 10;
                c.g[V0][0] = lw(c, S0, 0x748);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                c.g[V1][0] = lw(c, V0, 0xc);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0023_e388)?;
                c.g[V0][0] = lw(c, S0, 0x9c);
                let target = c.g[V0][0] as u32;
                call_ptr(c, target, 0x0023_e394)?;
                frame.restore(c, S0, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MPEG2_copyright_extension(void)` (0x3b23a0): parse the copyright
/// extension's fields out of the current bitstream into the decoder globals.
pub fn mpeg2_copyright_extension(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 1;
        callee::MPEG2_GET_BITS.call(c, 0x003b_23b0)?;
        c.g[A0][0] = 8;
        Addr(0x0050_9610).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_23c0)?;
        c.g[A0][0] = 1;
        Addr(0x0050_9614).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_23d0)?;
        c.g[A0][0] = 7;
        Addr(0x0050_9618).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_23e0)?;
        c.g[A0][0] = 0x0049_5630;
        callee::MPEG2_MARKER_BIT.call(c, 0x003b_23ec)?;
        c.g[A0][0] = 0x14;
        callee::MPEG2_GET_BITS.call(c, 0x003b_23f4)?;
        c.g[A0][0] = 0x0049_5658;
        Addr(0x0050_961c).sw(c, c.g[V0][0]);
        callee::MPEG2_MARKER_BIT.call(c, 0x003b_2408)?;
        c.g[A0][0] = 0x16;
        callee::MPEG2_GET_BITS.call(c, 0x003b_2410)?;
        c.g[A0][0] = 0x0049_5688;
        Addr(0x0050_9620).sw(c, c.g[V0][0]);
        callee::MPEG2_MARKER_BIT.call(c, 0x003b_2424)?;
        c.g[A0][0] = 0x16;
        callee::MPEG2_GET_BITS.call(c, 0x003b_242c)?;
        frame.restore_ra(c, 0);
        Addr(0x0050_9624).sw(c, c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

/// `void cGameLoadState_TransitionIn(self, refresh)` (0x2454f8): refresh the
/// loading screen's progress text and, when requested, both player rankings.
pub fn c_game_load_state_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x110);
        frame.save(c, S0, 0x100);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save(c, S2, 0xe0);
        frame.save(c, S1, 0xf0);
        frame.save_ra(c, 0xd0);
        c.g[S2][0] = c.g[A1][0];
        c.g[A0][0] = 0x004a_2bc8;
        call_ptr(c, table::STR_HASH, 0x0024_5520)?;
        c.g[A0][0] = lw(c, S0, 0x40);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, table::UI_SCREEN_FIND_OBJECT, 0x0024_552c)?;
        c.g[S1][0] = c.g[V0][0];
        let none = c.g[S1][0] == 0;
        c.g[V0][0] = Addr(gp(c, 0x2a74)).lw(c);
        if !none {
            c.cycles += 10;
            c.g[A1][0] = 0x004a_2bd0;
            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x38));
            c.cvt_w_s(1, 0);
            c.mfc1(6, 1);
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, table::SPRINTF, 0x0024_5554)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, table::UI_TEXT_SET_WIDE_STRING, 0x0024_5560)?;
        }
        c.cycles += 2;
        let no_refresh = c.g[S2][0] == 0;
        c.g[A0][0] = c.g[S0][0];
        if !no_refresh {
            c.cycles += 5;
            c.g[A1][0] = 0;
            callee::GAME_LOAD_UPDATE_PLAYER_RANKING.call(c, 0x0024_5570)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            callee::GAME_LOAD_UPDATE_PLAYER_RANKING.call(c, 0x0024_557c)?;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x100);
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0xf0);
        frame.restore(c, S2, 0xe0);
        frame.restore_ra(c, 0xd0);
        frame.close(c);
        Ok(())
    })
}

/// `void* MagnetModifier_ctor_2(self, stream)` (0x357358): install the
/// modifier's vtable and deserialize its two parameter blocks.
pub fn magnet_modifier_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x0048_f420;
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        let this = abi::arg(c, 0);
        c.g[S1][0] = sx(this);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(this.wrapping_add(0x10));
        Addr(this).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x18).lh(c);
        c.g[V0][0] = lw(c, V1, 0x1c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0035_7398)?;
        sw(c, S1, 0xa0, c.g[V0][0]);
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = 0x50;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x10).lh(c);
        c.g[V1][0] = lw(c, V0, 0x14);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0035_73b8)?;
        c.g[A3][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A1][0] = sx(this.wrapping_add(0x60));
        c.g[A2][0] = 0x40;
        c.g[A0][0] = Addr(c.g[A3][0] as u32).off(0x10).lh(c);
        c.g[V0][0] = lw(c, A3, 0x14);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0035_73d4)?;
        c.g[V1][0] = 1;
        c.g[V0][0] = c.g[S1][0];
        sw(c, S1, 0x50, c.g[V1][0]);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void DebugMenu_SetGameMode(self, index)` (0x17d008): store the selected
/// mode into the debug slot table and apply it to the race interface.
pub fn debug_menu_set_game_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
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

/// `void RiderRailslideState_Enter(self)` (0x13ad20): lean pose, orientation,
/// clear the railslide's balance/impulse accumulators and seed the state.
pub fn rider_railslide_state_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[A0][0] = lw(c, S0, 0x50);
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x110));
        call_ptr(c, 0x0011_fa10, 0x0013_ad3c)?;
        c.g[A0][0] = lw(c, S0, 0x50);
        call_ptr(c, 0x0011_e098, 0x0013_ad44)?;
        c.g[V1][0] = lw(c, S0, 0x50);
        c.lwc1(0, gp(c, -0x7184));
        sw(c, V1, 0x1f4, 0);
        sw(c, V1, 0x1f8, 0);
        sw(c, V1, 0x1f0, 0);
        c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x0));
        sw(c, V0, 0x20c, 0);
        sw(c, V0, 0x210, 0);
        sw(c, V0, 0x208, 0);
        c.g[V1][0] = lw(c, S0, 0x50);
        sw(c, V1, 0x254, 0);
        sw(c, V1, 0x258, 0);
        sw(c, V1, 0x250, 0);
        sw(c, S0, 0x10, 0);
        c.g[V1][0] = lw(c, S0, 0x50);
        sw(c, S0, 0x14, 0);
        sw(c, S0, 0x18, 0);
        sw(c, V1, 0x2dc, 0);
        c.g[V0][0] = lw(c, S0, 0x50);
        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x260));
        sw(c, V0, 0x264, 0);
        sw(c, S0, 0x20, 0);
        sw(c, S0, 0x1c, 0);
        c.g[V0][0] = lw(c, S0, 0x50);
        c.lq(3, (c.g[V0][0] as u32).wrapping_add(0x110));
        sw(c, S0, 0x40, 0);
        c.sq(3, (c.g[S0][0] as u32).wrapping_add(0x30));
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void SndLoadQueue_dtor(this, flags)` (0x28a148): flush the queue and free
/// the four owned buffers.
pub fn snd_load_queue_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0048_3b10;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        sw(c, S0, 0x1d4, c.g[V0][0]);
        callee::SND_LOAD_QUEUE_FLUSH.call(c, 0x0028_a170)?;
        c.g[A0][0] = lw(c, S0, 0x1c0);
        let first_null = c.g[A0][0] == 0;
        Addr(gp(c, 0x520)).sw(c, 0);
        if !first_null {
            c.cycles += 2;
            call_ptr(c, table::EA_MEM_FREE, 0x0028_a184)?;
        }
        c.cycles += 3;
        c.g[A0][0] = lw(c, S0, 0x1b0);
        if c.g[A0][0] == 0 {
            c.g[A0][0] = lw(c, S0, 0x1a0);
        } else {
            c.cycles += 3;
            call_ptr(c, table::EA_MEM_FREE, 0x0028_a198)?;
            c.g[A0][0] = lw(c, S0, 0x1a0);
        }
        c.cycles += 2;
        if c.g[A0][0] == 0 {
            c.g[A0][0] = lw(c, S0, 0x190);
        } else {
            c.cycles += 3;
            call_ptr(c, table::EA_MEM_FREE, 0x0028_a1ac)?;
            c.g[A0][0] = lw(c, S0, 0x190);
        }
        c.cycles += 2;
        let third_null = c.g[A0][0] == 0;
        c.g[V0][0] = c.g[S1][0] & 1;
        if !third_null {
            c.cycles += 3;
            call_ptr(c, table::EA_MEM_FREE, 0x0028_a1c0)?;
            c.g[V0][0] = c.g[S1][0] & 1;
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x0028_a1d4)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void FEModule_RiderPreviewDtor(this, flags)` (0x19e498): unload the rider
/// preview, then release every occupied preview slot (0x100 of them).
pub fn fe_module_rider_preview_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        let this = abi::arg(c, 0);
        c.g[S2][0] = sx(this);
        frame.save(c, S0, 0x50);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S4, 0x10);
        c.g[S0][0] = sx(this.wrapping_add(0x1c));
        frame.save_ra(c, 0);
        c.g[S4][0] = u64::MAX;
        callee::FEMODULE_RIDER_PREVIEW_UNLOAD.call(c, 0x0019_e4c8)?;
        c.g[S1][0] = 0xff;
        'slots: loop {
            c.cycles += 3;
            c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
            let free = (c.g[A1][0] as i64) < 0;
            c.g[A2][0] = Addr(gp(c, -0x854)).lw(c);
            if !free {
                c.cycles += 5;
                c.g[V0][0] = lw(c, A2, 0x10d8);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x190).lh(c);
                c.g[V1][0] = lw(c, V0, 0x194);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0019_e4f0)?;
            }
            c.cycles += 5;
            Addr(c.g[S0][0] as u32).sw(c, c.g[S4][0]);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            Addr(c.g[S0][0] as u32).off(4).sb(c, 0);
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0019_e4d0);
                }
                continue 'slots;
            }
            break 'slots;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S3][0] & 1;
        let no_delete = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x50);
        if !no_delete {
            c.cycles += 3;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x0019_e518)?;
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameStateEventSelect_LoadState(self, stream)` (0x2367c8): base load
/// plus four 4-byte stream fields.
pub fn game_state_event_select_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S1][0] = sx(this);
        callee::GAME_STATE_LOAD_STATE.call(c, 0x0023_67e4)?;
        for (off, ret) in [
            (0x10u32, 0x0023_6800u32),
            (0x14, 0x0023_681c),
            (0x8, 0x0023_6838),
            (0x18, 0x0023_6854),
        ] {
            c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(off));
            c.g[A2][0] = 4;
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x10).lh(c);
            c.g[V0][0] = lw(c, V1, 0x14);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, ret)?;
        }
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void InputSync_dtor(this, flags)` (0x2370a0): free each input record's
/// buffer and two objects.
pub fn input_sync_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        let this = abi::arg(c, 0);
        c.g[S2][0] = sx(this);
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0];
        let none = (lw(c, S2, 0x10) as i64) <= 0;
        if !none {
            c.cycles += 3;
            c.g[S0][0] = sx(this.wrapping_add(0x14));
            c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
            'records: loop {
                c.cycles += 2;
                if c.g[A0][0] == 0 {
                    c.g[A0][0] = lw(c, S0, 0x10);
                } else {
                    c.cycles += 3;
                    call_ptr(c, table::EA_MEM_FREE, 0x0023_70e8)?;
                    c.g[A0][0] = lw(c, S0, 0x10);
                }
                c.cycles += 9;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                call_ptr(c, table::BUILTIN_DELETE, 0x0023_70f4)?;
                c.g[A0][0] = lw(c, S0, 0x14);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                call_ptr(c, table::BUILTIN_DELETE, 0x0023_7100)?;
                c.g[V0][0] = lw(c, S2, 0x10);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0023_70d8);
                    }
                    continue 'records;
                }
                break 'records;
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S3][0] & 1;
        let no_delete = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x40);
        if !no_delete {
            c.cycles += 3;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x0023_7124)?;
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

/// `void OVScreenSaveReplay_Shutdown(self)` (0x1fde60): end the replay save if
/// active, pop the screen from the stack and begin playback.
pub fn ov_screen_save_replay_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        c.g[V0][0] = lw(c, S0, 0x1c8);
        let not_saving = c.g[V0][0] != c.g[V1][0];
        c.g[A1][0] = Addr(gp(c, -0x69c)).lw(c);
        if !not_saving {
            c.cycles += 6;
            c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
            c.g[A0][0] = lw(c, V1, 0x28);
            call_ptr(c, table::REPLAY_END_SAVE, 0x001f_de90)?;
            sw(c, S0, 0x1c8, 0);
            c.g[A1][0] = Addr(gp(c, -0x69c)).lw(c);
        }
        c.cycles += 26;
        c.g[V0][0] = 0x0053_66e8;
        c.g[A2][0] = 0x0044_28f0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
        c.g[A0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        Addr(gp(c, -0x69c)).sw(c, c.g[A1][0]);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        Addr(gp(c, -0x6a0)).sw(c, c.g[V1][0]);
        call_ptr(c, 0x001d_58b8, 0x001f_ded4)?;
        c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
        c.g[A1][0] = 0;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x84).lw(c);
        c.g[A0][0] = lw(c, V0, 0x28);
        call_ptr(c, table::REPLAY_BEGIN_PLAYBACK, 0x001f_def0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 OVScreenSaveReplay_v39(self, _, visible)` (0x1fe648) and
/// `OVScreenSaveRecords_v39` (0x1ff310): show/hide the screen's replay and
/// records widgets and store the inverted visibility.
fn ov_screen_set_widget_visibility(c: &mut Cpu, visible: u64) -> crate::call::Flow {
    let frame = Frame::open(c, 0x40);
    frame.save(c, S1, 0x20);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S0, 0x30);
    frame.save(c, S2, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = visible;
    c.g[A0][0] = 0x004a_2090;
    c.g[S0][0] ^= 1;
    call_ptr(c, table::STR_HASH, 0x001f_e674)?;
    c.g[A0][0] = lw(c, S1, 0x40);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, table::UI_SCREEN_FIND_OBJECT, 0x001f_e680)?;
    c.g[A0][0] = 0x004a_2568;
    c.g[S2][0] = c.g[V0][0];
    call_ptr(c, table::STR_HASH, 0x001f_e690)?;
    c.g[A0][0] = lw(c, S1, 0x40);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, table::UI_SCREEN_FIND_OBJECT, 0x001f_e69c)?;
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(8).lw(c);
    c.g[A1][0] = c.g[S0][0];
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x48).lh(c);
    c.g[A2][0] = lw(c, V1, 0x4c);
    let target = c.g[A2][0] as u32;
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x001f_e6b4)?;
    sw(c, S1, 0x224, c.g[S0][0]);
    c.g[A1][0] = c.g[S0][0];
    c.g[V0][0] = Addr(c.g[S2][0] as u32).off(8).lw(c);
    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
    c.g[V1][0] = lw(c, V0, 0x4c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x001f_e6d0)?;
    frame.restore(c, S0, 0x30);
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

pub fn ov_screen_save_replay_v39(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        ov_screen_set_widget_visibility(c, c.g[A2][0])
    })
}

pub fn ov_screen_save_records_v39(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        ov_screen_set_widget_visibility(c, c.g[A2][0])
    })
}

// ---------------------------------------------------------------------------
// BE profile/stat helpers.

/// `int BERewardInt_GetCollectibleTier(iface, rider, event, kind)` (0x1589b0):
/// compare the collected count with the four tier thresholds for `kind`.
pub fn be_reward_int_get_collectible_tier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        call_ptr(c, table::BE_GET_INSTANCE, 0x0015_89d4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xa;
        call_ptr(c, table::BE_GET_INTERFACE, 0x0015_89e0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A3][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        callee::BE_COUNT_COLLECTED_ON_PEAK.call(c, 0x0015_89f4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 4);
        c.g[V0][0] = 0x0045_afe8;
        c.g[V1][0] = 0;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        'tier: loop {
            c.cycles += 4;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.g[V1][0];
                break 'tier;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 4) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0015_8a10);
                }
                continue 'tier;
            }
            c.cycles += 1;
            c.g[V0][0] = u64::MAX;
            break 'tier;
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

/// `int BERewardInt_GetMissionTier(iface, rider, event, kind)` (0x158a50):
/// same tier lookup against the completed-mission thresholds.
pub fn be_reward_int_get_mission_tier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        call_ptr(c, table::BE_GET_INSTANCE, 0x0015_8a74)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xa;
        call_ptr(c, table::BE_GET_INTERFACE, 0x0015_8a80)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A3][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        callee::BE_COUNT_COMPLETED_ON_PEAK.call(c, 0x0015_8a94)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 4);
        c.g[V0][0] = 0x0045_b018;
        c.g[V1][0] = 0;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        'tier: loop {
            c.cycles += 4;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.g[V1][0];
                break 'tier;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 4) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0015_8ab0);
                }
                continue 'tier;
            }
            c.cycles += 1;
            c.g[V0][0] = u64::MAX;
            break 'tier;
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

/// `bool BENewPlayerInt_HasEventRecord(rider, a, b, c)` (0x145ef0): look the
/// rider's character profile up in the score-stat table and report whether
/// the event's record byte is set.
pub fn be_new_player_int_has_event_record(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        callee::BE_GET_RIDER_PROFILE.call(c, 0x0014_5f1c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S1][0] = c.g[V0][0];
        callee::BE_NEW_PLAYER_INT_GET_SLOT.call(c, 0x0014_5f28)?;
        c.g[V1][0] = 0x1c;
        c.g[A0][0] = 0x0053_0000;
        c.mult(0, V0, V0, V1);
        c.g[A0][0] = 0x0053_4fe0;
        c.g[T0][0] = 0xf88;
        c.g[V1][0] = 0x9b50;
        c.mult(0, S1, S1, V1);
        c.g[A3][0] = 0x004a_6ca8;
        c.g[A1][0] = c.g[S2][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[A2][0] = c.g[S3][0];
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x11).lb(c);
        c.mult(0, A0, A0, T0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        callee::BE_GET_CHAR_PROFILE_EVENT_RECORD.call(c, 0x0014_5f6c)?;
        c.g[V0][0] = Addr(c.g[V0][0] as u32).lb(c);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Collision queries (VU0: the original's COP2 ops are issued in order).

/// The length/normalise sequence shared by the sphere/hull triangle tests:
/// `v` at `sp` is the collision point, `out_len` (s0) receives |v| and
/// `out_dir` (s1) the normalised v.
fn collision_direction(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 22;
    c.lqc2(3, c.g[SP][0] as u32);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(2, 22);
    c.mtc1(2, 0);
    c.swc1(0, c.g[S0][0] as u32);
    c.mfc1(3, 0);
    c.qmtc2(3, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, c.g[SP][0] as u32);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 4, 4, Src::Q);
    c.sqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sq(2, c.g[S1][0] as u32);
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    frame_restore_s0(c);
    Ok(())
}

#[inline(always)]
fn frame_restore_s0(c: &mut Cpu) {
    c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
}

/// `int CollSphereTree_CollideTriangle(self, ..., outLen, outDir)` (0x330788):
/// collide the tree with a triangle and compute the contact direction.
pub fn coll_sphere_tree_collide_triangle(c: &mut Cpu) {
    guest(c, |c| coll_sphere_tree_common(c, table::COLL_SPHERE_TREE_TRIANGLE, 0x0033_07b0))
}

/// `int CollSphereTree_CollideTriangle2Sided(...)` (0x330828).
pub fn coll_sphere_tree_collide_triangle2_sided(c: &mut Cpu) {
    guest(c, |c| coll_sphere_tree_common(c, table::COLL_HULL_TRIANGLE, 0x0033_0850))
}

fn coll_sphere_tree_common(c: &mut Cpu, collide: u32, ret: u32) -> crate::call::Flow {
    c.cycles += 13;
    let frame = Frame::open(c, 0x60);
    frame.save(c, S0, 0x50);
    frame.save(c, S1, 0x40);
    c.g[S0][0] = c.g[T3][0];
    frame.save_ra(c, 0x30);
    c.g[S1][0] = c.g[T2][0];
    c.g[T2][0] = c.g[T1][0];
    c.g[A0][0] = Addr(abi::arg(c, 0)).off(0x60).lw(c);
    c.g[T1][0] = c.g[SP][0];
    call_ptr(c, collide, ret)?;
    c.g[A0][0] = c.g[V0][0];
    if c.g[A0][0] != 0 {
        collision_direction(c)?;
    } else {
        frame_restore_s0(c);
    }
    c.cycles += 5;
    c.g[V0][0] = c.g[A0][0];
    frame.restore(c, S1, 0x40);
    c.g[RA][0] = Addr(c.g[SP][0] as u32).off(0x30).ld(c);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
    c.pc = c.g[RA][0] as u32;
    Ok(())
}

/// `int CollHull_CollideTriangle2Sided(...)` (0x32f990): the hull-variant
/// collision with the contact direction.
pub fn coll_hull_collide_triangle2_sided(c: &mut Cpu) {
    guest(c, |c| coll_sphere_tree_common(c, table::COLL_HULL_TRIANGLE2, 0x0032_f9b8))
}

/// `void SphereTree_CollideSphereTree(self, other, ..., a2, a3)` (0x32cb58):
/// refresh both trees' cached bounds and descend.
pub fn sphere_tree_collide_sphere_tree(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A3][0];
        c.g[A0][0] = lw(c, S0, 0x98);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(8).lw(c);
        let skip = c.g[V0][0] == 0;
        if !skip {
            c.cycles += 4;
            call_ptr(c, table::COLL_SPHERE_CALC, 0x0032_cb94)?;
            c.g[V1][0] = lw(c, S0, 0x98);
            Addr(c.g[V1][0] as u32).off(0x28).sw(c, c.g[V0][0]);
        }
        c.cycles += 4;
        c.g[A0][0] = lw(c, S1, 0x98);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(8).lw(c);
        if c.g[V0][0] == 0 {
            c.g[A0][0] = c.g[S0][0];
        } else {
            c.cycles += 5;
            call_ptr(c, table::COLL_SPHERE_CALC, 0x0032_cbb4)?;
            c.g[V1][0] = lw(c, S1, 0x98);
            Addr(c.g[V1][0] as u32).off(0x28).sw(c, c.g[V0][0]);
            c.g[A0][0] = c.g[S0][0];
        }
        c.cycles += 14;
        c.g[A1][0] = c.g[S1][0];
        c.g[T1][0] = c.g[S2][0];
        c.g[T2][0] = c.g[S3][0];
        c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x80));
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        callee::SPHERE_TREE_COLLIDE_TREE_NODE.call(c, 0x0032_cbdc)?;
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
// UI states.

/// `void UIScreen_SetData(self, data)` (0x39c688): replace the screen's data
/// block with an unpacked copy (or clear the unpacked flag).
pub fn ui_screen_set_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        let this = abi::arg(c, 0);
        c.g[S1][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, table::EAC_GET_UNPACKED_SIZE, 0x0039_c6a8)?;
        let none = c.g[V0][0] == 0;
        c.g[A1][0] = 0x0049_0000;
        if !none {
            c.cycles += 14;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x0049_3e00;
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0;
            call_ptr(c, table::OPERATOR_NEW_TAG, 0x0039_c6c4)?;
            c.g[A0][0] = c.g[S0][0];
            sw(c, S1, 0xc, c.g[V0][0]);
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, table::EAC_UNPACK, 0x0039_c6d8)?;
            c.g[V1][0] = lw(c, S1, 0x14);
            c.g[V1][0] |= 2;
            sw(c, S1, 0x14, c.g[V1][0]);
        } else {
            c.cycles += 4;
            c.g[V0][0] = lw(c, S1, 0x14);
            c.g[V1][0] = 0xffff_ffff_ffff_fffd;
            c.g[V0][0] &= c.g[V1][0];
            sw(c, S1, 0x14, c.g[V0][0]);
        }
        c.cycles += 12;
        sw(c, S1, 0x10, c.g[S0][0]);
        c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(8).lw(c);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        sw(c, S1, 0x34, c.g[V0][0]);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        sw(c, S1, 0x38, c.g[V1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void UIState_Render(self)` (0x39e6b8): prepare the list and render each
/// visible node.
pub fn ui_state_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x10);
        let this = abi::arg(c, 0);
        c.g[S1][0] = sx(this);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(this.wrapping_add(0x24));
        frame.save_fpr(c, 20, 0x30);
        call_ptr(c, table::UI_LIST_FIRST, 0x0039_e6d8)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 7;
            c.g[V0][0] = lw(c, S1, 0x8);
            c.mtc1(0, 20);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x70).lh(c);
            c.g[V1][0] = lw(c, V0, 0x74);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0039_e6fc)?;
            c.g[V0][0] = lw(c, S0, 0x54);
            'nodes: loop {
                c.cycles += 4;
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
                c.g[V0][0] &= 1;
                let hidden = c.g[V0][0] == 0;
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
                if !hidden {
                    c.cycles += 7;
                    c.g[V0][0] = lw(c, S0, 0x48);
                    c.mov_s(12, 20);
                    c.mov_s(13, 20);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x88).lh(c);
                    c.g[V1][0] = lw(c, V0, 0x8c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0039_e72c)?;
                }
                c.cycles += 5;
                c.g[S0][0] = lw(c, S0, 0x4);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, table::UI_LIST_NODE_IS_SENTINEL, 0x0039_e738)?;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = lw(c, S0, 0x54);
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_e700);
                    }
                    continue 'nodes;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
                break 'nodes;
            }
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void LightMan_v12(obj, a, b, c, d, e, f12, f13, f14)` (0x38e008): call the
/// object's hook with the eight parameters, then the default empty hook.
pub fn light_man_v12(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[T1][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[T0][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A2][0];
        frame.save_fpr(c, 22, 0x60);
        frame.save_fpr(c, 21, 0x58);
        c.mov_s(22, 12);
        frame.save_fpr(c, 20, 0x50);
        c.mov_s(21, 13);
        c.mov_s(20, 14);
        c.g[V1][0] = Addr(abi::arg(c, 0)).lw(c);
        c.g[A2][0] = Addr(c.g[V1][0] as u32).off(0x10).lh(c);
        c.g[V0][0] = lw(c, V1, 0x14);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        call_ptr(c, target, 0x0038_e05c)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S2][0];
        c.mov_s(12, 22);
        c.mov_s(13, 21);
        c.g[T0][0] = c.g[S0][0];
        c.mov_s(14, 20);
        c.g[A0][0] = c.g[V0][0];
        callee::LIGHT_MAN_EMPTY_HOOK_D.call(c, 0x0038_e080)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x60);
        frame.restore_fpr(c, 21, 0x58);
        frame.restore_fpr(c, 20, 0x50);
        frame.close(c);
        Ok(())
    })
}

/// `void Debounce_Kill(self)` (0x342dd8): kill a debounce node and, if it has
/// no queued callback, allocate and post the dead node.
pub fn debounce_kill(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = c.g[A0][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, A2, 0x2c);
        let dead = (c.g[V0][0] as i64) < 0;
        frame.restore(c, S0, 0x10);
        if dead {
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = lw(c, A2, 0x30);
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            let no_parent = c.g[A2][0] == 0;
            c.g[A1][0] = 3;
            if !no_parent {
                c.cycles += 5;
                c.g[V0][0] = lw(c, A2, 0xc);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                c.g[V1][0] = lw(c, V0, 0xc);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0034_2e68)?;
            }
        } else {
            c.cycles += 2;
            let no_parent = c.g[A2][0] == 0;
            c.g[S0][0] = lw(c, A2, 0x18);
            if !no_parent {
                c.cycles += 6;
                c.g[V0][0] = lw(c, A2, 0xc);
                c.g[A1][0] = 3;
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                c.g[V1][0] = lw(c, V0, 0xc);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0034_2e20)?;
            }
            c.cycles += 11;
            c.g[A1][0] = 0x0048_e738;
            c.g[A0][0] = 0x1c;
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, table::EA_MEM_ALLOC_NAMED, 0x0034_2e38)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S0][0];
            callee::DEAD_NODE_CTOR.call(c, 0x0034_2e44)?;
            frame.restore(c, S0, 0x10);
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CrowdMan3d_LoadState(self, stream)` (0x344240): reset the five crowd
/// groups and load the ones whose flag is not -1.
pub fn crowd_man3d_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[S0][0];
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = 0xffff_ffff_ffff_0000;
        c.g[S4][0] = sx(this.wrapping_add(0x9c4));
        callee::CROWD_MAN3D_RESET.call(c, 0x0034_4274)?;
        c.g[S3][0] |= 0xffff;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(8));
        'groups: loop {
            c.cycles += 10;
            c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = 4;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x10).lh(c);
            c.g[V1][0] = lw(c, V0, 0x14);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0034_429c)?;
            c.g[V0][0] = lw(c, S1, 0x8);
            let done = c.g[V0][0] == c.g[S3][0];
            c.g[A0][0] = c.g[S1][0];
            if !done {
                c.cycles += 2;
                c.g[A1][0] = c.g[S2][0];
                callee::CROWD_MAN3D_GROUP_LOAD.call(c, 0x0034_42b0)?;
            }
            c.cycles += 4;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S4][0] as i64)) as u64;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1f4));
            if c.g[V0][0] != 0 {
                if event_due(c) {
                    deliver_event_here(c, 0x0034_4280);
                }
                continue 'groups;
            }
            c.cycles += 8;
            frame.restore(c, S0, 0x50);
            frame.restore(c, S1, 0x40);
            frame.restore(c, S2, 0x30);
            frame.restore(c, S3, 0x20);
            frame.restore(c, S4, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    })
}

/// `void GameStatePreRaceLoad_LoadState(self, stream)` (0x234a30): base load
/// plus four 4-byte stream fields (0x10, 0x14, 0x1c, 0x8).
pub fn game_state_pre_race_load_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S1][0] = sx(this);
        callee::GAME_STATE_LOAD_STATE.call(c, 0x0023_4a4c)?;
        for (off, ret) in [
            (0x10u32, 0x0023_4a68u32),
            (0x14, 0x0023_4a84),
            (0x1c, 0x0023_4aa0),
            (0x8, 0x0023_4abc),
        ] {
            c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(off));
            c.g[A2][0] = 4;
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x10).lh(c);
            c.g[V0][0] = lw(c, V1, 0x14);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, ret)?;
        }
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenTopTimes_OnWidgetCreated(self, widget)` (0x1f7e68): pick the
/// top-times title string and store the "online" flag.
pub fn ov_screen_top_times_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0x0046_f578;
        frame.save(c, S0, 0x20);
        let widget = abi::arg(c, 1);
        c.g[S0][0] = sx(widget);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = lw(c, S0, 0x38);
        call_ptr(c, table::STR_HASH, 0x001f_7e8c)?;
        c.g[A0][0] = 0x0047_0000;
        let other = c.g[S1][0] != c.g[V0][0];
        if other {
            c.cycles += 4;
            c.g[A0][0] = 0x0046_f5a8;
            call_ptr(c, table::STR_HASH, 0x001f_7ee4)?;
            if c.g[S1][0] != c.g[V0][0] {
                frame.restore(c, S0, 0x20);
            } else {
                c.cycles += 2;
                c.g[V0][0] = 1;
                sw(c, S0, 0x18, c.g[V0][0]);
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(gp(c, -0x9f0)).lw(c);
            if c.g[V0][0] != 0 {
                c.g[A0][0] = 0x0046_f598;
                c.cycles += 7;
                call_ptr(c, table::STR_HASH, 0x001f_7ec8)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, table::UI_TEXT_SET_TEXT_ID, 0x001f_7ed4)?;
                sw(c, S0, 0x18, 0);
            } else {
                c.cycles += 8;
                c.g[A0][0] = 0x0046_f588;
                call_ptr(c, table::STR_HASH, 0x001f_7eac)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, table::UI_TEXT_SET_TEXT_ID, 0x001f_7eb8)?;
                sw(c, S0, 0x18, 0);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BEBAGTInt_ClearUberTrickBit(rider, a1, slot, bit)` (0x14fe08): clear
/// the given bit in the rider's bag trick record.
pub fn be_bagt_int_clear_uber_trick_bit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        callee::BE_GET_RIDER_PROFILE.call(c, 0x0014_fe34)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[S2][0] = c.g[V0][0];
        callee::BE_GET_RIDER_CHARACTER.call(c, 0x0014_fe40)?;
        c.g[V1][0] = 0x1fe;
        c.g[A0][0] = 6;
        c.mult(0, V0, V0, V1);
        c.mult(0, S0, S0, A0);
        c.g[V1][0] = 0x13ec;
        c.g[A1][0] = 0x0053_0ec2;
        c.mult(0, S2, S2, V1);
        c.g[A0][0] = 1;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = sx((c.g[A0][0] as u32) << (c.g[S3][0] as u32 & 31));
        c.g[A0][0] = !(c.g[A0][0] | 0);
        frame.restore(c, S1, 0x30);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
        frame.restore(c, S3, 0x10);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        frame.restore(c, S2, 0x20);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lhu(c);
        c.g[V0][0] = 1;
        c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
        c.g[V1][0] &= c.g[A0][0];
        Addr(c.g[S0][0] as u32).sh(c, c.g[V1][0]);
        frame.restore(c, S0, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenProfileSavePDA_v39(self, _, visible)` (0x211a08): same widget
/// visibility update as the replay/records screens.
pub fn ov_screen_profile_save_pda_v39(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        ov_screen_set_widget_visibility(c, c.g[A2][0])
    })
}

/// `bool cFEStateMountainRoom_HandleInput(self, input)` (0x1d2ea0): when the
/// widget is shown and its sound event fires, play the activate sound and
/// hide the widget.
pub fn cfe_state_mountain_room_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        c.g[V1][0] = lw(c, S0, 0x48);
        let no_widget = c.g[V1][0] == 0;
        c.g[V0][0] = 0;
        if !no_widget {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x14).lw(c);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
            c.g[V0][0] &= 1;
        }
        c.cycles += 2;
        let hidden = c.g[V0][0] == 0;
        c.g[V0][0] = 0;
        if !hidden {
            c.cycles += 7;
            c.g[V1][0] = Addr(abi::arg(c, 1)).off(8).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x30).lh(c);
            c.g[V0][0] = lw(c, V1, 0x34);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001d_2ee4)?;
            let click = c.g[V0][0] == 0;
            c.g[A1][0] = 3;
            if !click {
                c.cycles += 16;
                c.g[V0][0] = lw(c, S0, 0x10);
                c.g[V1][0] = sx(gp(c, 0x2968));
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x14).lw(c);
                if c.g[A0][0] == 0 {
                    c.g[A0][0] = c.g[V1][0];
                }
                c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x10).lh(c);
                c.g[A2][0] = lw(c, V0, 0x14);
                let t = c.g[A2][0] as u32;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                call_ptr(c, t, 0x001d_2f10)?;
                c.g[A3][0] = lw(c, S0, 0x48);
                c.g[A1][0] = 0;
                c.g[V0][0] = Addr(c.g[A3][0] as u32).off(8).lw(c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
                c.g[V1][0] = lw(c, V0, 0x4c);
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001d_2f2c)?;
            }
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameModule_Enter(self)` (0x230050): tear down the front-end, reset
/// the managers and set the game-entered flags.
pub fn game_module_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        c.g[A0][0] = lw(c, V0, 0x8c);
        call_ptr(c, 0x0019_5d50, 0x0023_006c)?;
        c.g[A0][0] = lw(c, S0, 0x94);
        Addr(gp(c, -0x9e4)).sw(c, 0);
        call_ptr(c, 0x001f_30d8, 0x0023_0078)?;
        c.g[A0][0] = Addr(gp(c, -0x84c)).lw(c);
        call_ptr(c, 0x0027_8710, 0x0023_0080)?;
        c.g[A1][0] = Addr(gp(c, 0x1158)).lw(c);
        c.g[V0][0] = Addr(c.g[A1][0] as u32).off(4).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x88).lh(c);
        c.g[V1][0] = lw(c, V0, 0x8c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_0098)?;
        c.g[A1][0] = Addr(gp(c, 0x1158)).lw(c);
        c.g[V0][0] = Addr(c.g[A1][0] as u32).off(4).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x20).lh(c);
        c.g[V1][0] = lw(c, V0, 0x24);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_00b0)?;
        c.g[A0][0] = Addr(gp(c, -0x854)).lw(c);
        call_ptr(c, 0x0036_9ff8, 0x0023_00b8)?;
        c.g[A0][0] = lw(c, S0, 0x10);
        call_ptr(c, 0x003a_67e0, 0x0023_00c0)?;
        c.g[A0][0] = Addr(gp(c, 0x21e4)).lw(c);
        call_ptr(c, 0x002b_68d0, 0x0023_00c8)?;
        c.g[A0][0] = Addr(gp(c, 0x410)).lw(c);
        call_ptr(c, 0x0028_67e8, 0x0023_00d0)?;
        c.g[V0][0] = 1;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        Addr(gp(c, 0x12d8)).sw(c, c.g[V0][0]);
        Addr(gp(c, 0x1234)).sw(c, c.g[V0][0]);
        Addr(gp(c, 0x14e8)).sw(c, c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

/// `UIListNode* UIStateMgr_CloseTopStateAndPopups(list)` (0x39f718): for the
/// list's first node, clear the "visible" bit on each id<6 node until the
/// sentinel and return the sentinel's next.
pub fn ui_state_mgr_close_top_state_and_popups(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, table::UI_LIST_FIRST, 0x0039_f728)?;
        let first = c.g[V0][0];
        c.g[V1][0] = first;
        let empty = c.g[V1][0] == 0;
        c.g[V0][0] = 0;
        if empty {
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1c).lw(c);
        c.g[V0][0] &= 1;
        let clean = c.g[V0][0] == 0;
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1c).lw(c);
        if !clean {
            c.cycles += 3;
            c.g[A2][0] = 0xffff_ffff_ffff_c0ff;
            c.g[A1][0] = 0xffff_ffff_ffff_ff7f;
            'nodes: loop {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[A0][0] as u32) >> 8);
                c.g[V0][0] &= 0x3f;
                c.g[V0][0] = (c.g[V0][0] < 6) as u64;
                let small = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[A0][0] & c.g[A2][0];
                if !small {
                    c.cycles += 4;
                    c.g[V0][0] |= 0x700;
                    c.g[V0][0] &= c.g[A1][0];
                    c.g[V0][0] |= 0x80;
                    Addr(c.g[V1][0] as u32).off(0x1c).sw(c, c.g[V0][0]);
                }
                c.cycles += 5;
                c.g[V1][0] = Addr(c.g[V1][0] as u32).off(4).lw(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1c).lw(c);
                c.g[V0][0] &= 1;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1c).lw(c);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_f750);
                    }
                    continue 'nodes;
                }
                break 'nodes;
            }
        }
        c.cycles += 5;
        c.g[V0][0] = sx((c.g[A0][0] as u32) >> 8);
        c.g[V0][0] &= 0x3f;
        c.g[V0][0] = (c.g[V0][0] < 6) as u64;
        let small = c.g[V0][0] == 0;
        c.g[V0][0] = 0xffff_ffff_ffff_c0ff;
        if !small {
            c.cycles += 3;
            c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
            c.g[V0][0] |= 0x780;
            Addr(c.g[V1][0] as u32).off(0x1c).sw(c, c.g[V0][0]);
        }
        c.cycles += 1;
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(4).lw(c);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int BEMissionInt_GetTrackCollectedCount(rider, a, idx)` (0x153520): sum the
/// per-slot collected bytes of the character's profile record.
pub fn be_mission_int_get_track_collected_count(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        callee::BE_GET_RIDER_PROFILE.call(c, 0x0015_3544)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        callee::BE_GET_RIDER_CHARACTER.call(c, 0x0015_3550)?;
        c.g[V1][0] = 0xc;
        c.g[A0][0] = 0xf88;
        c.mult(0, S0, S0, V1);
        c.mult(0, V0, V0, A0);
        c.g[A1][0] = 0x9b50;
        c.g[V1][0] = 0x004a_6ca8;
        c.mult(0, S2, S2, A1);
        c.g[A1][0] = 0;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.g[A1][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.cycles += 7;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(4).lb(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
        let more = (c.g[A0][0] as i64) >= 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0015_3588);
            }
            // The counter starts at 0, so the second iteration never runs; the
            // generated loop is reproduced for shape.
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = c.g[A1][0];
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int RiderModel_FindBone(model, index, name)` (0x310c48): find the named
/// bone inside the indexed model part, returning its absolute bone index.
pub fn rider_model_find_bone(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save(c, S0, 0x40);
        c.g[V1][0] = 0x58;
        frame.save(c, S2, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S3, 0x10);
        frame.save(c, S1, 0x30);
        let this = abi::arg(c, 0);
        c.g[S3][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(this).off(0x1c).lw(c);
        c.g[A3][0] = Addr(this).off(0xc).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.mult(0, V0, V0, V1);
        c.g[S2][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = lw(c, S2, 0x44);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[A0][0] = lw(c, S2, 0x38);
        if none {
            c.cycles += 1;
            c.g[V0][0] = u64::MAX;
        } else {
            c.cycles += 1;
            c.g[S1][0] = c.g[A0][0];
            'bones: loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, table::STR_CASECMP, 0x0031_0ca4)?;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = lw(c, S2, 0x44);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = lw(c, S2, 0x4);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    break 'bones;
                }
                c.cycles += 4;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x50));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_0c98);
                    }
                    continue 'bones;
                }
                c.cycles += 1;
                c.g[V0][0] = u64::MAX;
                break 'bones;
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

/// `void NetFEState_RefreshBuddyActionPopup(self, a1, a2)` (0x1ab3d0): if the
/// buddy popup is open and selected, rebuild it from the stored popup.
pub fn net_fe_state_refresh_buddy_action_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        c.g[V0][0] = lw(c, S0, 0x1c);
        c.g[V0][0] &= 6;
        let inactive = c.g[V0][0] == 0;
        if inactive {
            refresh_popup_return(c, frame, 1);
            return Ok(());
        }
        c.cycles += 3;
        c.g[A0][0] = lw(c, S0, 0x6a0);
        if c.g[A0][0] == 0 {
            refresh_popup_return(c, frame, 0);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = lw(c, S0, 0x6c4);
        let no_selection = c.g[V0][0] == 0;
        c.g[V0][0] = 0x80b;
        if no_selection {
            refresh_popup_return(c, frame, 1);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x18).lw(c);
        let known = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x80c;
        if !known {
            c.cycles += 2;
            if c.g[V1][0] != c.g[V0][0] {
                refresh_popup_return(c, frame, 0);
                return Ok(());
            }
        }
        c.cycles += 5;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x1d).lbu(c);
        c.g[V0][0] &= 0x3f;
        c.g[V0][0] = (c.g[V0][0] < 6) as u64;
        if c.g[V0][0] == 0 {
            refresh_popup_return(c, frame, 0);
            return Ok(());
        }
        c.cycles += 8;
        callee::POPUP_CONFIRM_GET_SELECTION.call(c, 0x001a_b444)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = lw(c, A0, 0x6a0);
        c.g[A3][0] = c.g[S2][0];
        c.g[T0][0] = c.g[V0][0];
        callee::SETUP_BUDDY_ACTION_POPUP.call(c, 0x001a_b45c)?;
        refresh_popup_return(c, frame, 1);
        Ok(())
    })
}

/// The shared epilogue of `NetFEState_RefreshBuddyActionPopup`: the inline
/// `lq s0` plus the function's `+5` epilogue.
fn refresh_popup_return(c: &mut Cpu, frame: Frame, before: u64) {
    c.cycles += before;
    frame.restore(c, S0, 0x30);
    c.cycles += 5;
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `u32 cFEUseExistingAccount_CheckEmail(self)` (0x1ac448): if the account
/// email string is empty, configure and show the login popup; otherwise
/// report already handled.
pub fn cfe_use_existing_account_check_email(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        let this = abi::arg(c, 0);
        c.g[S2][0] = sx(this);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, S2, 0x6dc);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_sub(8)).lw(c);
        let handled = c.g[V1][0] != 0;
        c.g[V0][0] = 1;
        if !handled {
            c.cycles += 25;
            c.g[A0][0] = c.g[S2][0];
            callee::NET_FE_STATE_GET_POPUP.call(c, 0x001a_c478)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = 0x0046_2bf0;
            c.g[V0][0] = 1;
            sw(c, S0, 0xbc, c.g[V0][0]);
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
            call_ptr(c, table::STR_HASH, 0x001a_c494)?;
            c.g[V1][0] = 3;
            c.g[A0][0] = 0x0046_1618;
            sw(c, S0, 0xc0, c.g[V1][0]);
            sw(c, S0, 0xc4, c.g[V0][0]);
            call_ptr(c, table::STR_HASH, 0x001a_c4ac)?;
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0;
            callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID.call(c, 0x001a_c4bc)?;
            c.g[V0][0] = 0x80d;
            c.g[A0][0] = c.g[S2][0];
            sw(c, S0, 0x150, c.g[V0][0]);
            c.g[A1][0] = c.g[S0][0];
            callee::NET_FE_STATE_SHOW_POPUP.call(c, 0x001a_c4d0)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::SP as STACK;
    use crate::difftest::{check, check_lohi, fakes, put_vec, Rng, DATA, GAME_GP};

    // ------------------------------------------------------------------
    // Shared fixtures.

    fn w(c: &mut Cpu, at: u32, v: u32) {
        c.bus.write32(at, v);
    }

    fn put_text(c: &mut Cpu, at: u32, text: &[u8]) {
        for (i, b) in text.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
        c.bus.write8(at + text.len() as u32, 0);
    }

    /// `GetHashValue32` over the bytes at `addr` (the same algorithm as
    /// `StrHashElf`, for fixtures whose strings the test writes itself).
    fn hash(c: &mut Cpu, addr: u32) -> u32 {
        let mut h: u32 = 0;
        let mut at = addr;
        loop {
            let b = c.bus.read8(at);
            if b == 0 {
                return h;
            }
            h = h.wrapping_shl(4).wrapping_add(b as i32 as u32);
            let tmp = h & 0xf000_0000;
            if tmp != 0 {
                h ^= tmp >> 23;
                h ^= tmp;
            }
            at += 1;
        }
    }

    /// The BELibrary dummy, the world/race-manager chain and the interface
    /// table the game normally builds at boot.
    fn globals(c: &mut Cpu) {
        let be = DATA + 0x30_0000;
        let world = DATA + 0x8_0000;
        w(c, GAME_GP.wrapping_sub(0x1f04), be);
        w(c, GAME_GP.wrapping_sub(0x1efc), 0);
        w(c, 0x0053_08a8, DATA + 0x30_1000);
        w(c, GAME_GP.wrapping_sub(0x848), world);
        let game = world + 0x1000;
        w(c, world + 0x84, game);
        let race = world + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
        w(c, race + 0x28, world + 0x3000);
        w(c, DATA + 0x30_1000 + 4, DATA + 0x30_2000);
        w(c, DATA + 0x30_2000 + 8, 0);
        w(c, DATA + 0x30_2000 + 0xc, fakes::FAKE);
    }

    /// The engine's size-class pool allocator: each pool's start is stored in
    /// its `gp` slot, its free-list head at `start + size`, and eight free
    /// blocks inside the pool's range (so `MEM_IsPoolPointer` accepts them).
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
        w(c, GAME_GP.wrapping_add(0xdd8), 1);
        for (i, (slot, size)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let start = DATA + 0x40_0000 + 0x4_0000 * i;
            for n in 0..8u32 {
                let node = start + 0x40 * (n + 1);
                w(c, node, if n == 7 { 0 } else { node + 0x40 });
            }
            w(c, start + size, start + 0x40);
            w(c, GAME_GP + slot, start);
        }
    }

    fn heap_init(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        const RET: u64 = 0x0012_3450;
        c.g[A0][0] = MEM_HEAP as u64;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x20_0000;
        c.g[RA][0] = RET;
        c.pc = 0x0031_ff60;
        ee::s10::DLHeap_InitArena(c);
        assert_eq!(c.pc, RET as u32, "DLHeap_InitArena did not return");
    }

    fn base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        c.g[GP][0] = GAME_GP as u64;
        heap_init(c);
        let r = fakes::machine(c, seed, salt, calls);
        globals(c);
        pool(c);
        r
    }

    /// A pool block that `MEM_IsPoolPointer` accepts and `MEM_PoolFree` can
    /// take back.
    const POOL_BLOCK: u32 = DATA + 0x40_0000 + 0x40;

    /// A screen whose object list returns `node` for hash 0 (`StrHash("")` is 0
    /// on the test machine: the tag strings are not loaded).
    fn hash_object(c: &mut Cpu, screen: u32, node: u32) {
        w(c, screen + 0x38, DATA + 0x32_0100);
        w(c, DATA + 0x32_0100, 1);
        w(c, screen + 0x3c, DATA + 0x32_0000);
        w(c, DATA + 0x32_0000, node);
        w(c, node + 0x38, 0);
    }

    /// An object with a vtable whose `+off` slot is `{delta, fn}`.
    fn vt_slot(c: &mut Cpu, obj: u32, off: u32, delta: i16, f: u32) {
        w(c, obj, DATA + 0x34_0000);
        c.bus.write16(DATA + 0x34_0000 + off, delta as u16);
        w(c, DATA + 0x34_0000 + off + 4, f);
    }

    fn r32_seed(seed: u32) -> u32 {
        seed.wrapping_mul(0x9e37_79b9) ^ 0x1234
    }

    // ------------------------------------------------------------------
    // Front end / online states.

        #[test]
    fn cfe_state_challenge_popup_show_set_options_matches() {
        check(ssx3_ee::s03::f_001af688__generated, cfe_state_challenge_popup_show_set_options, 20, &|c, seed| {
            let _r = base(c, seed, 0x1af688, 0);
            let this = DATA + 0x1000;
            let screen = DATA + 0x2000;
            c.g[A0][0] = this as u64;
            w(c, this + 0x40, screen);
            w(c, this + 0x6d0, 0xdead);
            if seed % 2 == 0 {
                w(c, screen + 0x38, DATA + 0x32_0100);
                w(c, DATA + 0x32_0100, 0);
                w(c, screen + 0x3c, DATA + 0x32_0000);
            } else {
                let node = DATA + 0x3000;
                hash_object(c, screen, node);
                w(c, node + 8, DATA + 0x4000);
                c.bus.write16(DATA + 0x4000 + 0x40, 0x40);
                w(c, DATA + 0x4000 + 0x44, 1);
            }
        });
    }

    #[test]
    fn cfe_state_net_config_ctor_matches() {
        check(ssx3_ee::s03::f_001b3f78__generated, cfe_state_net_config_ctor, 8, &|c, seed| {
            let mut r = base(c, seed, 0x1b3f78, 2);
            let this = DATA + 0x1000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = (DATA + 0x2000) as u64;
            c.g[A2][0] = (r.next_u32() & 0xff) as u64;
            w(c, this + 0x6ec, r.next_u32());
        });
    }

    #[test]
    fn cfe_state_create_step1_backup_account_info_matches() {
        check(ssx3_ee::s03::f_001ba178__generated, cfe_state_create_step1_backup_account_info, 30, &|c, seed| {
            let _r = base(c, seed, 0x1ba178, 2);
            let this = DATA + 0x1000;
            let acct = DATA + 0x2000;
            c.g[A0][0] = this as u64;
            w(c, this + 0x6ec, if seed % 2 == 0 { 0 } else { 1 });
            w(c, this + 0x6d0, acct);
            w(c, acct + 4, 0);
            w(c, acct + 8, 0);
            w(c, acct + 0xc, 0);
            w(c, acct + 0x38, 0x11);
            w(c, acct + 0x3c, 0x22);
        });
    }

        #[test]
    fn buddy_man_find_challenger_index_matches() {
        check(ssx3_ee::s07::f_00262e20__generated, buddy_man_find_challenger_index, 30, &|c, seed| {
            let _r = base(c, seed, 0x262e20, 2);
            let this = DATA + 0x1000;
            let name = DATA + 0x5000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = name as u64;
            put_text(c, name, b"zed");
            let count = match seed % 4 {
                0 => 0,
                1 => 1,
                2 => 2,
                _ => 3,
            };
            let begin = DATA + 0x2000;
            for i in 0..count {
                let ent = DATA + 0x3000 + i * 0x100;
                let nm = DATA + 0x4000 + i * 0x100;
                let null = seed % 5 == 4 && i == count - 1;
                w(c, begin + 4 * i, if null { 0 } else { ent });
                put_text(c, nm, if seed % 3 == 0 { b"abc" } else { b"zzz" });
                w(c, ent, nm);
            }
            w(c, this + 0xbc, begin);
            w(c, this + 0xc0, begin + 4 * count);
        });
    }

    #[test]
    fn c_voip_man_start_matches() {
        check(ssx3_ee::s07::f_00266d58__generated, c_voip_man_start, 10, &|c, seed| {
            let _r = base(c, seed, 0x266d58, 2);
            let this = DATA + 0x1000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = (DATA + 0x2000) as u64;
            w(c, this, if seed % 2 == 0 { 0 } else { 1 });
            w(c, GAME_GP.wrapping_add(0x308), if seed % 3 == 0 { 5 } else { 20 });
            w(c, GAME_GP.wrapping_add(0x5610), if seed % 4 == 0 { 0 } else { 1 << 18 });
        });
    }

    #[test]
    fn ssx_audio_system_record_race_result_matches() {
        check(ssx3_ee::s08::f_002a45c0__generated, ssx_audio_system_record_race_result, 10, &|c, seed| {
            let _r = base(c, seed, 0x2a45c0, 3);
            let this = DATA + 0x1000;
            let world = DATA + 0x8_0000;
            c.g[A0][0] = this as u64;
            w(c, world + 0xc0, DATA + 0x7000);
            w(c, DATA + 0x7000, if seed % 2 == 0 { 1 } else { 0 });
            w(c, 0x0053_6730, r32_seed(seed));
            let race = world + 0x2000;
            let rider = DATA + 0x5000;
            w(c, race + 0x28, rider);
            w(c, rider + 0x790, DATA + 0x6000);
            w(c, DATA + 0x6000 + 0x128, seed);
            w(c, DATA + 0x6000 + 0x114, seed.wrapping_mul(3));
            w(c, rider + 0x480, seed & 1);
        });
    }

        #[test]
    fn script_engine_on_filter_loaded_matches() {
        check(ssx3_ee::s07::f_0027b6b0__generated, script_engine_on_filter_loaded, 20, &|c, seed| {
            let _r = base(c, seed, 0x27b6b0, 3);
            let this = DATA + 0x1000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = 0;
            w(c, this + 0x620, 2);
            let header = DATA + 0x3000;
            w(c, STACK - 0x30, if seed % 2 == 0 { 0 } else { header });
            w(c, STACK - 0x2c, if seed % 2 == 0 { 0 } else { 8 });
            let count = match seed % 3 {
                0 => 0,
                1 => 1,
                _ => 2,
            };
            w(c, header, count);
            w(c, header + 4, 1);
            w(c, header + 8, 0x20);
            w(c, header + 0xc, 0);
            w(c, header + 0x10, 0x40);
        });
    }

    #[test]
    fn script_engine_deactivate_binding_matches() {
        check_lohi(ssx3_ee::s07::f_0027d4a0__generated, script_engine_deactivate_binding, 30, &|c, seed| {
            let _r = base(c, seed, 0x27d4a0, 2);
            let this = DATA + 0x1000;
            let key = DATA + 0x2000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = key as u64;
            w(c, key + 4, (seed % 4) as u32);
            let slot = this + 0x50 + 0x14 * (seed % 4);
            w(c, slot + 4, 7);
            w(c, slot + 8, 0x55);
            w(c, slot + 0xc, 0);
            let zero_fn = ssx3_ee::s09::f_002cc098 as fn(&mut Cpu) as usize as u32;
            w(c, this + 0x2000, 0);
            w(c, this + 0x2a8, DATA + 0x3000);
            w(c, DATA + 0x3000, DATA + 0x3100);
            c.bus.write16(DATA + 0x3100 + 0x20, 0x2000);
            w(c, DATA + 0x3100 + 0x24, if seed % 2 == 0 { zero_fn } else { 0 });
            if seed % 3 == 0 {
                w(c, this, 0);
            } else {
                w(c, this, DATA + 0x4000);
                w(c, DATA + 0x4000 + 0xc, DATA + 0x4100);
                c.bus.write16(DATA + 0x4100 + 0x28, 0);
                w(c, DATA + 0x4100 + 0x2c, 0);
            }
        });
    }

    #[test]
    fn song_instance_dtor_matches() {
        check(ssx3_ee::s08::f_002b1f78__generated, song_instance_dtor, 30, &|c, seed| {
            let _r = base(c, seed, 0x2b1f78, 3);
            let this = DATA + 0x1000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = (seed & 1) as u64;
            w(c, GAME_GP.wrapping_add(0x6fc), DATA + 0x2000);
            w(c, DATA + 0x2000, DATA + 0x2100);
            let obj = DATA + 0x3000;
            w(c, this, obj);
            w(c, obj + 0x40, (seed % 3) as u32);
            w(c, this + 0x34, 0);
            w(c, this + 0x24, if seed % 2 == 0 { 0 } else { POOL_BLOCK });
            w(c, this + 0x28, if seed % 5 == 0 { 0 } else { POOL_BLOCK });
        });
    }

    #[test]
    fn menu_sys_pop_page_matches() {
        check(ssx3_ee::s09::f_002cc460__generated, menu_sys_pop_page, 30, &|c, seed| {
            let _r = base(c, seed, 0x2cc460, 2);
            let this = DATA + 0x1000;
            c.g[A0][0] = this as u64;
            let count = (seed % 3) as u32;
            w(c, this, count);
            for i in 0..3u32 {
                let page = DATA + 0x2000 + i * 0x100;
                w(c, this + 4 + 4 * i, page);
                vt_slot(c, page + 0x120, 0x38, 0, 0);
                w(c, page + 0x12c, page + 0x120);
            }
        });
    }

    #[test]
    fn in_seq_channel_flush_events_matches() {
        check(ssx3_ee::s07::f_00273d20__generated, in_seq_channel_flush_events, 30, &|c, seed| {
            let _r = base(c, seed, 0x273d20, 2);
            let this = DATA + 0x1000;
            c.g[A0][0] = this as u64;
            w(c, this + 0x2c, if seed % 2 == 0 { 0 } else { DATA + 0x2000 });
            w(c, this + 0x30, r32_seed(seed));
            w(c, this + 0x34, if seed % 3 == 0 { 0 } else { DATA + 0x3000 });
            w(c, this + 0x1c, DATA + 0x4000);
            w(c, DATA + 0x2000, DATA + 0x2100);
            w(c, DATA + 0x2100 + 0x18, 0);
            w(c, DATA + 0x2100 + 0x1c, 0);
            w(c, DATA + 0x2100 + 0x20, 0);
            w(c, DATA + 0x2100 + 0x24, 0);
        });
    }

    #[test]
    fn mc_manager_state36_matches() {
        check(ssx3_ee::s06::f_0023e320__generated, mc_manager_state36, 20, &|c, seed| {
            let _r = base(c, seed, 0x23e320, 2);
            let this = DATA + 0x1000;
            c.g[A0][0] = this as u64;
            let movie = DATA + 0x2000;
            w(c, this + 0x434, movie);
            w(c, movie, DATA + 0x2100);
            let zero_fn = ssx3_ee::s09::f_002cc098 as fn(&mut Cpu) as usize as u32;
            c.bus.write16(DATA + 0x2100 + 0x58, 0x2000);
            w(c, DATA + 0x2100 + 0x5c, if seed % 2 == 0 { zero_fn } else { 0 });
            w(c, this + 0x428, 0);
            w(c, this + 0x748, DATA + 0x3000);
            w(c, DATA + 0x3000, DATA + 0x3100);
            c.bus.write16(DATA + 0x3100 + 8, 0);
            w(c, DATA + 0x3100 + 0xc, 0);
            w(c, this + 0x98, 0);
            w(c, this + 0x9c, 0);
        });
    }

        #[test]
    fn c_game_load_state_transition_in_matches() {
        check(ssx3_ee::s06::f_002454f8__generated, c_game_load_state_transition_in, 20, &|c, seed| {
            let _r = base(c, seed, 0x2454f8, 2);
            let this = DATA + 0x1000;
            let screen = DATA + 0x2000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = (seed & 1) as u64;
            w(c, this + 0x40, screen);
            hash_object(c, screen, DATA + 0x3000);
            w(c, DATA + 0x3000 + 8, DATA + 0x3100);
            c.bus.write16(DATA + 0x3148, 0x48);
            w(c, DATA + 0x314c, 0);
            w(c, GAME_GP.wrapping_add(0x2a74), DATA + 0x4000);
            w(c, DATA + 0x4000 + 0x38, 3.5f32.to_bits());
            put_text(c, 0x004a_2bd0, b"");
        });
    }

    #[test]
    fn magnet_modifier_ctor_2_matches() {
        check(ssx3_ee::s11::f_00357358__generated, magnet_modifier_ctor_2, 10, &|c, seed| {
            let mut r = base(c, seed, 0x357358, 2);
            let this = DATA + 0x1000;
            let stream = DATA + 0x2000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = stream as u64;
            w(c, stream, DATA + 0x3000);
            for o in [0x10u32, 0x18] {
                c.bus.write16(DATA + 0x3000 + o, 0);
                w(c, DATA + 0x3000 + o + 4, 0);
            }
            w(c, this + 0x50, r.next_u32());
        });
    }

    #[test]
    fn debug_menu_set_game_mode_matches() {
        check(ssx3_ee::s02::f_0017d008__generated, debug_menu_set_game_mode, 8, &|c, seed| {
            let mut r = base(c, seed, 0x17d008, 3);
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
        });
    }

    #[test]
    fn rider_railslide_state_enter_matches() {
        check(ssx3_ee::s01::f_0013ad20__generated, rider_railslide_state_enter, 8, &|c, seed| {
            let _r = base(c, seed, 0x13ad20, 2);
            let this = DATA + 0x1000;
            let rider = DATA + 0x4000;
            c.g[A0][0] = this as u64;
            w(c, this + 0x50, rider);
            put_vec(c, rider + 0x110, [1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits(), 0]);
            w(c, rider + 0x10, 0);
            w(c, rider + 0x770, 0);
            w(c, GAME_GP.wrapping_sub(0x7184), 0.05f32.to_bits());
        });
    }

    #[test]
    fn snd_load_queue_dtor_matches() {
        check(ssx3_ee::s07::f_0028a148__generated, snd_load_queue_dtor, 30, &|c, seed| {
            let _r = base(c, seed, 0x28a148, 3);
            let this = DATA + 0x1000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = (seed & 1) as u64;
            for (i, off) in [0x1c0u32, 0x1b0, 0x1a0, 0x190].into_iter().enumerate() {
                let set = (seed >> i) & 1 != 0;
                w(c, this + off, if set { POOL_BLOCK } else { 0 });
            }
            w(c, GAME_GP.wrapping_add(0x520), 0x55);
        });
    }

    #[test]
    fn fe_module_rider_preview_dtor_matches() {
        check(ssx3_ee::s03::f_0019e498__generated, fe_module_rider_preview_dtor, 8, &|c, seed| {
            let _r = base(c, seed, 0x19e498, 2);
            let this = DATA + 0x1000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = (seed & 1) as u64;
            w(c, this + 8, 0);
            w(c, this + 0xc, 0);
            w(c, GAME_GP.wrapping_sub(0x854), DATA + 0x2000);
            w(c, DATA + 0x2000 + 0x10d8, DATA + 0x2100);
            c.bus.write16(DATA + 0x2100 + 0x190, 0);
            w(c, DATA + 0x2100 + 0x194, 0);
            for i in 0..256u32 {
                let slot = this + 0x1c + 0xc * i;
                let used = seed % 3 != 0 && i % 7 != 0;
                w(c, slot, if used { (i % 16) as u32 } else { 0xffff_ffff });
                c.bus.write8(slot + 4, 0xff);
            }
        });
    }

    #[test]
    fn game_state_event_select_load_state_matches() {
        check(ssx3_ee::s06::f_002367c8__generated, game_state_event_select_load_state, 10, &|c, seed| {
            let _r = base(c, seed, 0x2367c8, 2);
            let this = DATA + 0x1000;
            let stream = DATA + 0x2000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = stream as u64;
            w(c, stream, DATA + 0x3000);
            c.bus.write16(DATA + 0x3000 + 0x10, 0);
            w(c, DATA + 0x3000 + 0x14, 0);
            w(c, this + 0x10, r32_seed(seed));
        });
    }

            #[test]
    fn ov_screen_save_replay_v39_matches() {
        check(ssx3_ee::s05::f_001fe648__generated, ov_screen_save_replay_v39, 20, &|c, seed| {
            let _r = base(c, seed, 0x1fe648, 2);
            let this = DATA + 0x1000;
            let screen = DATA + 0x2000;
            c.g[A0][0] = this as u64;
            c.g[A2][0] = (seed & 1) as u64;
            w(c, this + 0x40, screen);
            hash_object(c, screen, DATA + 0x3000);
            w(c, DATA + 0x3000 + 8, DATA + 0x3100);
            c.bus.write16(DATA + 0x3148, 0x48);
            w(c, DATA + 0x314c, 0);
        });
    }

        #[test]
    fn be_reward_int_get_collectible_tier_matches() {
        check(ssx3_ee::s01::f_001589b0__generated, be_reward_int_get_collectible_tier, 10, &|c, seed| {
            let _r = base(c, seed, 0x1589b0, 3);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 6) as u64;
            c.g[A2][0] = ((seed / 6) % 6) as u64;
            c.g[A3][0] = (seed % 4) as u64;
            for i in 0..16u32 {
                w(c, 0x0045_afe8 + 4 * i, i % 5);
            }
        });
    }

    #[test]
    fn be_reward_int_get_mission_tier_matches() {
        check(ssx3_ee::s01::f_00158a50__generated, be_reward_int_get_mission_tier, 10, &|c, seed| {
            let _r = base(c, seed, 0x158a50, 3);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 6) as u64;
            c.g[A2][0] = ((seed / 6) % 6) as u64;
            c.g[A3][0] = (seed % 4) as u64;
            for i in 0..16u32 {
                w(c, 0x0045_b018 + 4 * i, i % 5);
            }
        });
    }

    #[test]
    fn be_new_player_int_has_event_record_matches() {
        check_lohi(ssx3_ee::s01::f_00145ef0__generated, be_new_player_int_has_event_record, 20, &|c, seed| {
            let _r = base(c, seed, 0x145ef0, 3);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((seed % 8) + 1) as u64;
            c.g[A2][0] = (seed % 5) as u64;
            c.g[A3][0] = ((seed / 5) % 3) as u64;
            for i in 0..1024u32 {
                c.bus.write8(0x004a_6ca8 + i, seed.wrapping_add(i) as u8);
            }
        });
    }

    // ------------------------------------------------------------------
    // Collision.

    fn coll_args(c: &mut Cpu, seed: u32, salt: u64, addr: u32, f: GuestFn) {
        let _r = base(c, seed, salt, 1);
        let this = DATA + 0x1000;
        c.g[A0][0] = this as u64;
        c.g[A1][0] = (DATA + 0x2000) as u64;
        c.g[A2][0] = (DATA + 0x2100) as u64;
        c.g[A3][0] = (DATA + 0x2200) as u64;
        c.g[T0][0] = (DATA + 0x2300) as u64;
        c.g[T1][0] = (DATA + 0x2400) as u64;
        c.g[T2][0] = (DATA + 0x5000) as u64;
        c.g[T3][0] = (DATA + 0x6000) as u64;
        w(c, this + 0x60, addr);
        put_vec(c, DATA + 0x2400, [1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits(), 0]);
        w(c, addr, 0);
        let _ = f;
    }

    #[test]
    fn coll_sphere_tree_collide_triangle_matches() {
        check(ssx3_ee::s11::f_00330788__generated, coll_sphere_tree_collide_triangle, 20, &|c, seed| {
            coll_args(c, seed, 0x330788, DATA + 0x3000, ssx3_ee::s11::f_00330788__generated);
        });
    }

    #[test]
    fn coll_sphere_tree_collide_triangle2_sided_matches() {
        check(
            ssx3_ee::s11::f_00330828__generated,
            coll_sphere_tree_collide_triangle2_sided,
            20,
            &|c, seed| {
                coll_args(c, seed, 0x330828, DATA + 0x3000, ssx3_ee::s11::f_00330828__generated);
            },
        );
    }

    #[test]
    fn coll_hull_collide_triangle2_sided_matches() {
        check(ssx3_ee::s11::f_0032f990__generated, coll_hull_collide_triangle2_sided, 20, &|c, seed| {
            coll_args(c, seed, 0x32f990, DATA + 0x3000, ssx3_ee::s11::f_0032f990__generated);
        });
    }

    #[test]
    fn sphere_tree_collide_sphere_tree_matches() {
        check(ssx3_ee::s10::f_0032cb58__generated, sphere_tree_collide_sphere_tree, 20, &|c, seed| {
            let _r = base(c, seed, 0x32cb58, 2);
            let a = DATA + 0x1000;
            let b = DATA + 0x2000;
            c.g[A0][0] = a as u64;
            c.g[A1][0] = b as u64;
            c.g[A2][0] = (DATA + 0x3000) as u64;
            c.g[A3][0] = (DATA + 0x4000) as u64;
            w(c, a + 0x98, DATA + 0x5000);
            w(c, b + 0x98, DATA + 0x6000);
            w(c, DATA + 0x5000 + 8, (seed & 1) as u32);
            w(c, DATA + 0x6000 + 8, ((seed >> 1) & 1) as u32);
            w(c, a + 0x80, 0);
        });
    }

    // ------------------------------------------------------------------
    // UI.

    #[test]
    fn ui_screen_set_data_matches() {
        check(ssx3_ee::s13::f_0039c688__generated, ui_screen_set_data, 10, &|c, seed| {
            let _r = base(c, seed, 0x39c688, 3);
            let this = DATA + 0x1000;
            let data = DATA + 0x2000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = data as u64;
            c.bus.write8(data, 0);
            c.bus.write8(data + 1, 0);
        });
    }

    #[test]
    fn ui_state_render_matches() {
        check(ssx3_ee::s13::f_0039e6b8__generated, ui_state_render, 20, &|c, seed| {
            let _r = base(c, seed, 0x39e6b8, 2);
            let this = DATA + 0x1000;
            c.g[A0][0] = this as u64;
            let list = this + 0x24;
            let sentinel = DATA + 0x5000;
            w(c, sentinel, sentinel);
            w(c, sentinel + 4, sentinel);
            if seed % 3 == 0 {
                w(c, list + 4, sentinel);
            } else {
                let node = DATA + 0x2000;
                w(c, list + 4, node);
                w(c, node, sentinel);
                w(c, node + 4, sentinel);
                w(c, node + 0x54, if seed % 2 == 0 { 0 } else { 1 << 6 });
                w(c, node + 0x48, DATA + 0x3000);
                w(c, DATA + 0x3000, DATA + 0x3100);
                c.bus.write16(DATA + 0x3100 + 0x88, 0x40);
                w(c, DATA + 0x3100 + 0x8c, 0);
            }
            w(c, this + 8, DATA + 0x4000);
            w(c, DATA + 0x4000, DATA + 0x4100);
            c.bus.write16(DATA + 0x4100 + 0x70, 0);
            w(c, DATA + 0x4100 + 0x74, 0);
        });
    }

    #[test]
    fn light_man_v12_matches() {
        check(ssx3_ee::s12::f_0038e008__generated, light_man_v12, 20, &|c, seed| {
            let mut r = base(c, seed, 0x38e008, 2);
            let obj = DATA + 0x1000;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = (DATA + 0x2000) as u64;
            c.g[A3][0] = (DATA + 0x3000) as u64;
            c.g[T0][0] = (DATA + 0x4000) as u64;
            c.g[T1][0] = (DATA + 0x5000) as u64;
            c.f[12] = r.edge_f32();
            c.f[13] = r.edge_f32();
            c.f[14] = r.edge_f32();
            w(c, obj, DATA + 0x6000);
            c.bus.write16(DATA + 0x6000 + 0x10, 0);
            w(c, DATA + 0x6000 + 0x14, 0);
        });
    }

    #[test]
    fn debounce_kill_matches() {
        check(ssx3_ee::s11::f_00342dd8__generated, debounce_kill, 30, &|c, seed| {
            let _r = base(c, seed, 0x342dd8, 3);
            let this = DATA + 0x1000;
            c.g[A0][0] = this as u64;
            w(c, this + 0x2c, if seed % 3 == 0 { 0xffff_ffff } else { (seed % 8) as u32 });
            w(c, this + 0x30, if (seed / 3) % 2 == 0 { 0 } else { 1 });
            w(c, this + 0x18, r32_seed(seed));
            w(c, this + 0xc, DATA + 0x2000);
            w(c, DATA + 0x2000, DATA + 0x2100);
            c.bus.write16(DATA + 0x2100 + 8, 0);
            w(c, DATA + 0x2100 + 0xc, 0);
        });
    }

    #[test]
    fn crowd_man3d_load_state_matches() {
        check(ssx3_ee::s11::f_00344240__generated, crowd_man3d_load_state, 8, &|c, seed| {
            let _r = base(c, seed, 0x344240, 3);
            let this = DATA + 0x1000;
            let stream = DATA + 0x5000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = stream as u64;
            w(c, stream, DATA + 0x6000);
            c.bus.write16(DATA + 0x6000 + 0x10, 0);
            w(c, DATA + 0x6000 + 0x14, 0);
            for i in 0..5u32 {
                w(c, this + 0x1f4 * i + 8, if seed % 2 == 0 { 0xffff_ffff } else { 0 });
            }
        });
    }

    #[test]
    fn game_state_pre_race_load_load_state_matches() {
        check(ssx3_ee::s06::f_00234a30__generated, game_state_pre_race_load_load_state, 10, &|c, seed| {
            let _r = base(c, seed, 0x234a30, 2);
            let this = DATA + 0x1000;
            let stream = DATA + 0x2000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = stream as u64;
            w(c, stream, DATA + 0x3000);
            c.bus.write16(DATA + 0x3000 + 0x10, 0);
            w(c, DATA + 0x3000 + 0x14, 0);
            w(c, this + 0x10, r32_seed(seed));
        });
    }

    #[test]
    fn ov_screen_top_times_on_widget_created_matches() {
        check(ssx3_ee::s04::f_001f7e68__generated, ov_screen_top_times_on_widget_created, 20, &|c, seed| {
            let _r = base(c, seed, 0x1f7e68, 2);
            let widget = DATA + 0x1000;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = widget as u64;
            put_text(c, 0x0046_f578, b"aaa");
            put_text(c, 0x0046_f588, b"bbb");
            put_text(c, 0x0046_f598, b"ccc");
            put_text(c, 0x0046_f5a8, b"ddd");
            let h1 = hash(c, 0x0046_f578);
            let h4 = hash(c, 0x0046_f5a8);
            let h = match seed % 4 {
                0 | 1 => h1,
                2 => h4,
                _ => 0x1111_2222,
            };
            w(c, widget + 0x38, h);
            w(c, GAME_GP.wrapping_sub(0x9f0), (seed % 3 == 0) as u32);
        });
    }

    #[test]
    fn be_bagt_int_clear_uber_trick_bit_matches() {
        check_lohi(ssx3_ee::s01::f_0014fe08__generated, be_bagt_int_clear_uber_trick_bit, 10, &|c, seed| {
            let _r = base(c, seed, 0x14fe08, 3);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 8) as u64;
            c.g[A2][0] = (seed % 4) as u64;
            c.g[A3][0] = (seed % 16) as u64;
            for i in 0..0x2000u32 {
                c.bus.write8(0x0053_0ec2 + i, seed.wrapping_add(i) as u8);
            }
        });
    }

        #[test]
    fn cfe_state_mountain_room_handle_input_matches() {
        check(ssx3_ee::s04::f_001d2ea0__generated, cfe_state_mountain_room_handle_input, 20, &|c, seed| {
            let _r = base(c, seed, 0x1d2ea0, 2);
            let this = DATA + 0x1000;
            let input = DATA + 0x2000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = input as u64;
            let widget = DATA + 0x3000;
            w(c, this + 0x48, if seed % 3 == 0 { 0 } else { widget });
            w(c, widget + 0x14, if seed % 2 == 0 { 0 } else { 1 << 6 });
            w(c, input + 8, DATA + 0x4000);
            let zero_fn = ssx3_ee::s09::f_002cc098 as fn(&mut Cpu) as usize as u32;
            c.bus.write16(DATA + 0x4000 + 0x30, 0x2000);
            w(c, DATA + 0x4000 + 0x34, if seed % 2 == 0 { zero_fn } else { 1 });
            w(c, this + 0x10, DATA + 0x5000);
            w(c, DATA + 0x5000 + 0x14, 0);
            w(c, GAME_GP.wrapping_add(0x2968), DATA + 0x6000);
            w(c, DATA + 0x6000, DATA + 0x6100);
            c.bus.write16(DATA + 0x6100 + 0x10, 0);
            w(c, DATA + 0x6100 + 0x14, 0);
            w(c, widget + 8, DATA + 0x7000);
            c.bus.write16(DATA + 0x7000 + 0x48, 0);
            w(c, DATA + 0x7000 + 0x4c, 0);
        });
    }

        #[test]
    fn ui_state_mgr_close_top_state_and_popups_matches() {
        check(ssx3_ee::s13::f_0039f718__generated, ui_state_mgr_close_top_state_and_popups, 30, &|c, seed| {
            let _r = base(c, seed, 0x39f718, 1);
            let list = DATA + 0x1000;
            c.g[A0][0] = list as u64;
            let sentinel = DATA + 0x5000;
            w(c, sentinel, sentinel);
            w(c, sentinel + 4, sentinel);
            if seed % 4 == 0 {
                w(c, list + 4, sentinel);
            } else {
                let n0 = DATA + 0x2000;
                let n1 = DATA + 0x3000;
                w(c, list + 4, n0);
                w(c, n0, sentinel);
                w(c, n0 + 4, if seed % 2 == 0 { n1 } else { sentinel });
                w(c, n0 + 0x1c, (seed % 2) | ((seed % 5) << 8));
                w(c, n1, sentinel);
                w(c, n1 + 4, sentinel);
                w(c, n1 + 0x1c, ((seed / 2) % 5) << 8);
            }
        });
    }

    #[test]
    fn be_mission_int_get_track_collected_count_matches() {
        check_lohi(ssx3_ee::s01::f_00153520__generated, be_mission_int_get_track_collected_count, 20, &|c, seed| {
            let _r = base(c, seed, 0x153520, 3);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 8) as u64;
            c.g[A2][0] = (seed % 4) as u64;
            for i in 0..0x20000u32 {
                c.bus.write8(0x004a_6ca8 + i, seed.wrapping_add(i) as u8);
            }
        });
    }

        #[test]
    fn net_fe_state_refresh_buddy_action_popup_matches() {
        check(ssx3_ee::s03::f_001ab3d0__generated, net_fe_state_refresh_buddy_action_popup, 30, &|c, seed| {
            let _r = base(c, seed, 0x1ab3d0, 2);
            let this = DATA + 0x1000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = (DATA + 0x2000) as u64;
            c.g[A2][0] = (DATA + 0x3000) as u64;
            w(c, this + 0x1c, match seed % 4 { 0 => 0, 1 => 2, 2 => 4, _ => 6 });
            let popup = DATA + 0x4000;
            w(c, this + 0x6a0, if seed % 5 == 0 { 0 } else { popup });
            w(c, this + 0x6c4, if seed % 3 == 0 { 0 } else { 1 });
            w(c, popup + 0x18, [0x80bu32, 0x80c, 0x900, 0x80b][seed as usize % 4]);
            c.bus.write8(popup + 0x1d, match seed % 3 { 0 => 0, 1 => 5, _ => 0x40 });
        });
    }

    #[test]
    fn cfe_use_existing_account_check_email_matches() {
        check(ssx3_ee::s03::f_001ac448__generated, cfe_use_existing_account_check_email, 20, &|c, seed| {
            let _r = base(c, seed, 0x1ac448, 3);
            let this = DATA + 0x1000;
            c.g[A0][0] = this as u64;
            let email = DATA + 0x2000;
            w(c, this + 0x6dc, email + 8);
            w(c, email, if seed % 2 == 0 { 0 } else { 0x10 });
            w(c, this + 0x6a0, 0);
        });
    }
}


