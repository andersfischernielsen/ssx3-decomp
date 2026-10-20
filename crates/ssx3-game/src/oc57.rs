//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for small wrappers, thunks and destructors — the app's main
//! thread priority, the (endless) `PS2ExecMan` idle loop, the emitter
//! "desc curve" setter, UI text/pair-flag helpers, mission/reward lookups,
//! camera-algorithm push/set, the `cFEState*_TransitionIn`/`*_Exit` hooks, the
//! shared-login state callbacks, `TibExtMemFree` and the seventeen
//! `LobbyMan_On*_Thunk` callbacks — each with a differential test against the
//! generated body.
//!
//! The wrappers are almost all "same frame, same registers at one call, same
//! cycle charge, unwind check" shapes, so the bodies below follow
//! [`crate::call`]: `Frame::open`/`save_ra` for the stack frame, the callee
//! exactly as the generated code calls it (`Callee` for a direct call,
//! `call_ptr` for `call_indirect`), the `?`/`guest` unwind check after it and
//! `Frame::close` for `jr ra`. `v0`/`v1` are left as the generated code leaves
//! them, including the constant scratch values (`0x4a0000`, the `-1` in
//! `GameModule_InvalidateFocus`).

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_a6b8, ssx_app_set_main_thread_priority, "SSXApp_SetMainThreadPriority"),
    (0x0031_ab10, ps2_exec_man_v14, "PS2ExecMan_v14"),
    (0x0034_1fc8, anim_object_kill_restorable, "AnimObject_KillRestorable"),
    (0x0035_8120, multi_particle_update, "MultiParticle_Update"),
    (0x0036_ce28, emitter_set_desc_curve_b, "Emitter_SetDescCurveB"),
    (0x003a_0e90, ui_text_set_wide_string, "UIText_SetWideString"),
    (0x003a_5508, ui_pair_flags_ctor, "UIPairFlags_ctor"),
    (0x003b_22f8, mpeg2_picture_temporal_scalable_extension, "MPEG2_picture_temporal_scalable_extension"),
    (0x0011_3170, rider_dtor_free, "Rider_dtorFree"),
    (0x0015_4278, be_mission_int_count_missions_on_current_track, "BEMissionInt_CountMissionsOnCurrentTrack"),
    (0x0015_4588, be_mission_int_get_nth_mission_on_current_track, "BEMissionInt_GetNthMissionOnCurrentTrack"),
    (0x0015_6a10, be_reward_int_load_rewards_file, "BERewardInt_LoadRewardsFile"),
    (0x0015_d050, chase_cam_controller_set_algorithm_immediate, "ChaseCamController_SetAlgorithmImmediate"),
    (0x0016_9340, script_cam_controller_push_algorithm, "ScriptCamController_PushAlgorithm"),
    (0x0016_bef8, camera_trigger_list_dtor, "CameraTriggerList_dtor"),
    (0x0016_cf18, camera_trigger_man_empty_global_call, "CameraTriggerMan_EmptyGlobalCall"),
    (0x0018_4bb8, fe_state_uber_trick_transition_in, "cFEStateUberTrick_TransitionIn"),
    (0x0018_7180, fe_state_memcard_update_card_scan, "cFEStateMemCard_UpdateCardScan"),
    (0x001b_6770, net_config_auth_popup_on_widget_action, "NetConfigAuthPopup_OnWidgetAction"),
    (0x001e_1090, fe_locale_set_flag27, "FELocale_SetFlag27"),
    (0x001e_52d0, ov_state_view_popup_transition_in, "cOVState_ViewPopup_TransitionIn"),
    (0x001f_49d0, fe_state_buy_attrib_transition_in, "cFEStateBuyAttrib_TransitionIn"),
    (0x001f_55c0, fe_state_career_stats_transition_in, "cFEStateCareerStats_TransitionIn"),
    (0x0022_cd18, section_man_dtor, "SectionMan_dtor"),
    (0x0023_0338, game_module_invalidate_focus, "GameModule_InvalidateFocus"),
    (0x0023_6ea0, game_state_results_exit, "GameStateResults_Exit"),
    (0x0023_7cb0, game_rand_seed, "GameRand_Seed"),
    (0x0024_2518, occlusion_man_dtor, "OcclusionMan_dtor"),
    (0x0026_59c0, shared_login_man_shutdown, "cSharedLoginMan_Shutdown"),
    (0x0026_6010, shared_login_man_mc_cb_state09, "cSharedLoginMan_McCb_State09"),
    (0x0026_6038, shared_login_man_mc_cb_state0a, "cSharedLoginMan_McCb_State0A"),
    (0x0026_60a0, shared_login_man_mc_cb_state03, "cSharedLoginMan_McCb_State03"),
    (0x0026_6760, tib_ext_mem_free, "TibExtMemFree"),
    (0x0026_8a90, lobby_man_on_user_list_changed_thunk, "LobbyMan_OnUserListChanged_Thunk"),
    (0x0026_8ab8, lobby_man_on_room_list_changed_thunk, "LobbyMan_OnRoomListChanged_Thunk"),
    (0x0026_8ae0, lobby_man_on_rank_list_changed_thunk, "LobbyMan_OnRankListChanged_Thunk"),
    (0x0026_8b08, lobby_man_on_recv_debug_thunk, "LobbyMan_OnRecvDebug_Thunk"),
    (0x0026_8b30, lobby_man_on_message_thunk, "LobbyMan_OnMessage_Thunk"),
    (0x0026_8b58, lobby_man_on_disconnect_thunk, "LobbyMan_OnDisconnect_Thunk"),
    (0x0026_8b80, lobby_man_on_game_start_thunk, "LobbyMan_OnGameStart_Thunk"),
    (0x0026_8ba8, lobby_man_on_connect_reply_thunk, "LobbyMan_OnConnectReply_Thunk"),
    (0x0026_8bd0, lobby_man_on_create_account_reply_thunk, "LobbyMan_OnCreateAccountReply_Thunk"),
    (0x0026_8bf8, lobby_man_on_edit_account_reply_thunk, "LobbyMan_OnEditAccountReply_Thunk"),
    (0x0026_8c20, lobby_man_on_lost_by_email_reply_thunk, "LobbyMan_OnLostByEmailReply_Thunk"),
    (0x0026_8c48, lobby_man_on_lost_by_name_reply_thunk, "LobbyMan_OnLostByNameReply_Thunk"),
    (0x0026_8c70, lobby_man_on_login_reply_thunk, "LobbyMan_OnLoginReply_Thunk"),
    (0x0026_8c98, lobby_man_on_select_persona_reply_thunk, "LobbyMan_OnSelectPersonaReply_Thunk"),
    (0x0026_8cc0, lobby_man_on_own_stats_reply_thunk, "LobbyMan_OnOwnStatsReply_Thunk"),
    (0x0026_8ce8, lobby_man_on_create_persona_reply_thunk, "LobbyMan_OnCreatePersonaReply_Thunk"),
    (0x0026_8d10, lobby_man_on_delete_persona_reply_thunk, "LobbyMan_OnDeletePersonaReply_Thunk"),
];

// ---------------------------------------------------------------------------
// Callees (direct calls: the generated code calls the shard function itself).

mod callee {
    use super::{ee, Callee};

    /// `WorldObj_RunKillScript` (0x34fcc0).
    pub const WORLD_OBJ_RUN_KILL_SCRIPT: Callee =
        Callee { addr: 0x0034_fcc0, f: ee::s11::f_0034fcc0 };
    /// `UIText_SetWideStringInternal` (0x3a0eb0).
    pub const UITEXT_SET_WIDE_STRING_INTERNAL: Callee =
        Callee { addr: 0x003a_0eb0, f: ee::s13::f_003a0eb0 };
    /// `MPEG2_Error` (0x3b11e8; an empty stub).
    pub const MPEG2_ERROR: Callee = Callee { addr: 0x003b_11e8, f: ee::s13::f_003b11e8 };
    /// `BEMissionInt_CountMissionsOnTrack` (0x1542a0).
    pub const MISSION_COUNT: Callee = Callee { addr: 0x0015_42a0, f: ee::s01::f_001542a0 };
    /// `BEMissionInt_GetNthMissionOnTrack` (0x1545b0).
    pub const MISSION_NTH: Callee = Callee { addr: 0x0015_45b0, f: ee::s01::f_001545b0 };
    /// `RewardsFile_Load` (0x15ad58).
    pub const REWARDS_FILE_LOAD: Callee = Callee { addr: 0x0015_ad58, f: ee::s01::f_0015ad58 };
    /// `ChaseCamController_SetAlgorithm` (0x15d078).
    pub const CHASE_CAM_SET_ALGORITHM: Callee =
        Callee { addr: 0x0015_d078, f: ee::s01::f_0015d078 };
    /// `CameraTriggerMan_EmptyStub` (0x16cf40).
    pub const CAMERA_TRIGGER_EMPTY_STUB: Callee =
        Callee { addr: 0x0016_cf40, f: ee::s02::f_0016cf40 };
    /// `cFEStateUberTrick_RefreshTrickList` (0x183de0).
    pub const UBER_TRICK_REFRESH: Callee = Callee { addr: 0x0018_3de0, f: ee::s02::f_00183de0 };
    /// `cFEStateMemCard_ScanStep` (0x187a38).
    pub const MEMCARD_SCAN_STEP: Callee = Callee { addr: 0x0018_7a38, f: ee::s02::f_00187a38 };
    /// `NetFEState_OnWidgetAction` (0x1a8918).
    pub const NET_FE_STATE_ON_WIDGET_ACTION: Callee =
        Callee { addr: 0x001a_8918, f: ee::s03::f_001a8918 };
    /// `cOVState_ViewPopup_ShowMessage` (0x1e5428).
    pub const VIEW_POPUP_SHOW_MESSAGE: Callee =
        Callee { addr: 0x001e_5428, f: ee::s04::f_001e5428 };
    /// `cFEStateBuyAttrib_UpdateBars` (0x1f50b0).
    pub const BUY_ATTRIB_UPDATE_BARS: Callee =
        Callee { addr: 0x001f_50b0, f: ee::s04::f_001f50b0 };
    /// `cFEStateCareerStats_ShowHighlightsPage` (0x1f5da0).
    pub const CAREER_STATS_SHOW_HIGHLIGHTS: Callee =
        Callee { addr: 0x001f_5da0, f: ee::s04::f_001f5da0 };
}

/// `__builtin_delete` (operator delete, 0x317e50): the destructors call it
/// through the function table.
const DELETE: u32 = 0x0031_7e50;
/// `EA_MemFree` (0x317e98).
const MEM_FREE: u32 = 0x0031_7e98;

/// The `gp`-relative game global holding the main thread handle
/// (`D_004A3ECC`).
const APP_MAIN_THREAD_GP: u32 = 0x0ddc;

// ---------------------------------------------------------------------------
// SSXApp_SetMainThreadPriority / PS2ExecMan_v14.

/// `void SSXApp_SetMainThreadPriority(void)` (0x31a6b8): set the app thread's
/// priority to 0x65 (`ChangeThreadPriority(handle, 0x65)`).
pub fn ssx_app_set_main_thread_priority(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.lw(r32(c, GP).wrapping_add(APP_MAIN_THREAD_GP));
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x65;
        call_ptr(c, 0x0042_3c30, 0x0031_a6cc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PS2ExecMan_v14(void)` (0x31ab10): hand the semaphore at `self + 0xc`
/// over (`SignalSema`), then sleep forever (`SleepThread`) — the idle routine
/// of a thread. The `0x31ab30` tail call the compiler left after the loop is
/// unreachable (it emitted `li t0, 1` before the branch), so the loop below is
/// the whole body; a thread switch requested by the event hook unwinds it.
pub fn ps2_exec_man_v14(c: &mut Cpu) {
    guest(c, |c: &mut Cpu| -> Flow {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(r32(c, A0).wrapping_add(0xc));
        call_ptr(c, 0x0042_3dc0, 0x0031_ab20)?;
        loop {
            c.cycles += 4;
            call_ptr(c, 0x0042_3cc0, 0x0031_ab28)?;
            if event_due(c) {
                deliver_event_here(c, 0x0031_ab20);
            }
        }
    })
}

// ---------------------------------------------------------------------------
// AnimObject_KillRestorable / MultiParticle_Update.

/// `void AnimObject_KillRestorable(AnimObject* o, int kill)` (0x341fc8): run
/// the object's kill script (`WorldObj_RunKillScript(o + 0x30)`) when `kill`
/// is non-zero.
pub fn anim_object_kill_restorable(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        let kill = c.g[A1][0] != 0;
        frame.save_ra(c, 0);
        if kill {
            c.cycles += 2;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x30));
            callee::WORLD_OBJ_RUN_KILL_SCRIPT.call(c, 0x0034_1fdc)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MultiParticle_Update(MultiParticle* p)` (0x358120): update the
/// particle system at `p->+0x10`, stepping it by the frame time at
/// `gp - 0x2a80`.
pub fn multi_particle_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.lwc1(12, r32(c, GP).wrapping_sub(0x2a80));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(r32(c, A0).wrapping_add(0x10));
        call_ptr(c, 0x0037_0788, 0x0035_8134)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Emitter_SetDescCurveB.

/// `void Emitter_SetDescCurveB(Emitter* e, float a, float b)` (0x36ce28):
/// scale the descriptor curve by the emitter's rate (`+0x4c`) and store the
/// two ramps: `+0x14 = (b - a) * s`, `+0x1c = a * s - (b - a) * s`.
pub fn emitter_set_desc_curve_b(c: &mut Cpu) {
    c.cycles += 8;
    let this = abi::arg(c, 0);
    // f13 = b, f12 = a (the EABI float argument order).
    c.sub_s(13, 13, 12);
    c.lwc1(0, this.wrapping_add(0x4c));
    c.mul_s(12, 12, 0);
    c.mul_s(13, 13, 0);
    c.sub_s(12, 12, 13);
    c.swc1(13, this.wrapping_add(0x14));
    c.swc1(12, this.wrapping_add(0x1c));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// UIText_SetWideString / UIPairFlags_ctor / MPEG2_picture_temporal_scalable_extension.

/// `void UIText_SetWideString(UIText* t, char* str)` (0x3a0e90): clear the
/// cached byte length (`+0xb0`) and install a new wide string.
pub fn ui_text_set_wide_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(r32(c, V0).wrapping_add(0xb0)).sw(c, 0);
        callee::UITEXT_SET_WIDE_STRING_INTERNAL.call(c, 0x003a_0ea4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `UIPairFlags* UIPairFlags_ctor(UIPairFlags* f)` (0x3a5508): mask the two
/// pair-flag bytes (`+0` and `+1`) down to their top two bits and return the
/// object.
pub fn ui_pair_flags_ctor(c: &mut Cpu) {
    c.cycles += 8;
    let flags = r32(c, A0);
    c.g[A1][0] = Addr(flags.wrapping_add(1)).lbu(c);
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = Addr(flags).lbu(c);
    c.g[A1][0] = c.g[A1][0] & 0xc0;
    c.g[V1][0] = c.g[V1][0] & 0xc0;
    Addr(flags.wrapping_add(1)).sb(c, c.g[A1][0]);
    Addr(flags).sb(c, c.g[V1][0]);
    abi::ret(c);
}

/// `void MPEG2_picture_temporal_scalable_extension(void)` (0x3b22f8):
/// `MPEG2_Error(D_00495608)` (the error handler is an empty stub).
pub fn mpeg2_picture_temporal_scalable_extension(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x0049_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x5608));
        callee::MPEG2_ERROR.call(c, 0x003b_230c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Destructors sharing the `if (flags & 1) delete this` shape.

/// `if (flags & 1) operator delete(this)` — the four destructors below.
fn delete_if_flag(c: &mut Cpu, ret: u32) -> Flow {
    c.cycles += 4;
    let frame = Frame::open(c, 0x10);
    c.g[A1][0] = c.g[A1][0] & 0x1;
    let free = c.g[A1][0] != 0;
    frame.save_ra(c, 0);
    if free {
        c.cycles += 2;
        call_ptr(c, DELETE, ret)?;
    }
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void Rider_dtorFree(Rider* r, int flags)` (0x113170).
pub fn rider_dtor_free(c: &mut Cpu) {
    guest(c, |c| delete_if_flag(c, 0x0011_3188))
}

/// `void CameraTriggerList_dtor(CameraTriggerList* l, int flags)` (0x16bef8).
pub fn camera_trigger_list_dtor(c: &mut Cpu) {
    guest(c, |c| delete_if_flag(c, 0x0016_bf10))
}

/// `void SectionMan_dtor(SectionMan* m, int flags)` (0x22cd18).
pub fn section_man_dtor(c: &mut Cpu) {
    guest(c, |c| delete_if_flag(c, 0x0022_cd30))
}

/// `void OcclusionMan_dtor(OcclusionMan* m, int flags)` (0x242518).
pub fn occlusion_man_dtor(c: &mut Cpu) {
    guest(c, |c| delete_if_flag(c, 0x0024_2530))
}

// ---------------------------------------------------------------------------
// BE mission / reward lookups.

/// `int BEMissionInt_CountMissionsOnCurrentTrack(BEMissionInt* m)`
/// (0x154278): count the missions whose track id equals the current one
/// (`D_005305F0`).
pub fn be_mission_int_count_missions_on_current_track(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[V0][0] = 0x0053_0000;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.lw(r32(c, V0).wrapping_add(0x5f0));
        frame.save_ra(c, 0);
        callee::MISSION_COUNT.call(c, 0x0015_4290)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int BEMissionInt_GetNthMissionOnCurrentTrack(BEMissionInt* m, int n)`
/// (0x154588): the `n`-th mission on the current track (`D_005305F0`), or 0.
pub fn be_mission_int_get_nth_mission_on_current_track(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[V0][0] = 0x0053_0000;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.lw(r32(c, V0).wrapping_add(0x5f0));
        frame.save_ra(c, 0);
        callee::MISSION_NTH.call(c, 0x0015_45a0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BERewardInt_LoadRewardsFile(BERewardInt* r)` (0x156a10): (re)load the
/// rewards database (`D_004C3E98`) through `RewardsFile_Load(db, 1)`.
pub fn be_reward_int_load_rewards_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x004c_0000;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x1;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x3e98));
        callee::REWARDS_FILE_LOAD.call(c, 0x0015_6a28)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Camera algorithms.

/// `ChaseCamController_SetAlgorithmImmediate(Ctrl* c, int algo, void* arg)`
/// (0x15d050): `SetAlgorithm(c, algo, arg, 1.0f)`.
pub fn chase_cam_controller_set_algorithm_immediate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 12);
        frame.save_ra(c, 0);
        callee::CHASE_CAM_SET_ALGORITHM.call(c, 0x0015_d068)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptCamController_PushAlgorithm(ScriptCamController* c, void* algo)`
/// (0x169340): push `algo` onto the list at `c->+8` with weight 1.0f.
pub fn script_cam_controller_push_algorithm(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 12);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(r32(c, A0).wrapping_add(0x8));
        call_ptr(c, 0x0015_c988, 0x0016_9358)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// CameraTriggerMan_EmptyGlobalCall.

/// `void CameraTriggerMan_EmptyGlobalCall(CameraTriggerMan* m)` (0x16cf18):
/// `CameraTriggerMan_EmptyStub(D_004C5830, m)`.
pub fn camera_trigger_man_empty_global_call(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[A1][0] = c.g[A0][0];
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x004c_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x5830));
        callee::CAMERA_TRIGGER_EMPTY_STUB.call(c, 0x0016_cf30)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// FE state transition hooks: `if (on) refresh(this); return 0;`.

/// The shared shape of the four `TransitionIn` hooks below.
fn transition_in(c: &mut Cpu, ret: u32, refresh: Callee) -> Flow {
    c.cycles += 3;
    let frame = Frame::open(c, 0x10);
    let on = c.g[A1][0] != 0;
    frame.save_ra(c, 0);
    if on {
        c.cycles += 2;
        refresh.call(c, ret)?;
    }
    c.cycles += 4;
    frame.restore_ra(c, 0);
    c.g[V0][0] = 0;
    frame.close(c);
    Ok(())
}

/// `bool cFEStateUberTrick_TransitionIn(cFEStateUberTrick* s, int on)`
/// (0x184bb8): refresh the trick list when entering, and report success.
pub fn fe_state_uber_trick_transition_in(c: &mut Cpu) {
    guest(c, |c| transition_in(c, 0x0018_4bcc, callee::UBER_TRICK_REFRESH))
}

/// `bool cOVState_ViewPopup_TransitionIn(cOVState_ViewPopup* s, int on)`
/// (0x1e52d0).
pub fn ov_state_view_popup_transition_in(c: &mut Cpu) {
    guest(c, |c| transition_in(c, 0x001e_52e4, callee::VIEW_POPUP_SHOW_MESSAGE))
}

/// `bool cFEStateBuyAttrib_TransitionIn(cFEStateBuyAttrib* s, int on)`
/// (0x1f49d0).
pub fn fe_state_buy_attrib_transition_in(c: &mut Cpu) {
    guest(c, |c| transition_in(c, 0x001f_49e4, callee::BUY_ATTRIB_UPDATE_BARS))
}

/// `bool cFEStateCareerStats_TransitionIn(cFEStateCareerStats* s, int on)`
/// (0x1f55c0).
pub fn fe_state_career_stats_transition_in(c: &mut Cpu) {
    guest(c, |c| transition_in(c, 0x001f_55d4, callee::CAREER_STATS_SHOW_HIGHLIGHTS))
}

// ---------------------------------------------------------------------------
// cFEStateMemCard_UpdateCardScan.

/// `void cFEStateMemCard_UpdateCardScan(cFEStateMemCard* s)` (0x187180): poll
/// the memory cards, then advance the scan at `D_004A14B8`.
pub fn fe_state_memcard_update_card_scan(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x001d_83a8, 0x0018_7190)?;
        c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0x1c38));
        callee::MEMCARD_SCAN_STEP.call(c, 0x0018_7198)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// NetConfigAuthPopup_OnWidgetAction.

/// `void NetConfigAuthPopup_OnWidgetAction(Popup* p, int a1, int msg)`
/// (0x1b6770): forward every action but the "auth ok" one (0xd) to the base
/// state.
pub fn net_config_auth_popup_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xd;
        let handled = c.g[A2][0] == 0xd;
        frame.save_ra(c, 0);
        if !handled {
            c.cycles += 2;
            callee::NET_FE_STATE_ON_WIDGET_ACTION.call(c, 0x001b_6788)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// FELocale_SetFlag27.

/// `void FELocale_SetFlag27(FELocale* l, int on)` (0x1e1090): set bit 27 of
/// the locale's flags word (`0xf7ffffff` masked, the flag shifted into place).
pub fn fe_locale_set_flag27(c: &mut Cpu) {
    c.cycles += 9;
    let this = r32(c, A0);
    c.g[V0][0] = c.lw(this);
    c.g[V1][0] = 0xffff_ffff_f7ff_0000;
    c.g[A1][0] = (0u64 < c.g[A1][0]) as u64;
    c.g[V1][0] = c.g[V1][0] | 0xffff;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 27);
    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
    c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
    Addr(this).sw(c, c.g[V0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// GameModule_InvalidateFocus / GameStateResults_Exit / GameRand_Seed.

/// `void GameModule_InvalidateFocus(GameModule* m)` (0x230338): drop the
/// focused widget by setting its focus timer (`+0xd0`) to -1.
pub fn game_module_invalidate_focus(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0xc));
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0xa4));
        let none = c.g[V1][0] == 0;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        if !none {
            c.cycles += 1;
            Addr(r32(c, V1).wrapping_add(0xd0)).sw(c, c.g[V0][0]);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void GameStateResults_Exit(GameStateResults* s)` (0x236ea0): close every
/// UI state below the results state manager
/// (`UIStateMgr_CloseAll(D_004A2C68->+0x48 + 0x18)`).
pub fn game_state_results_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(0x488));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0x48));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
        call_ptr(c, 0x0039_f840, 0x0023_6eb8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameRand_Seed(unsigned seed)` (0x237cb0): seed the pseudo random
/// generator at `D_004C9548`.
pub fn game_rand_seed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[A1][0] = c.g[A0][0];
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x004d_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x6ab8));
        call_ptr(c, 0x0031_7958, 0x0023_7cc8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cSharedLoginMan_Shutdown and the memory-card callbacks.

/// `void cSharedLoginMan_Shutdown(cSharedLoginMan* m)` (0x2659c0): clear the
/// state word, shut the login library down and destroy the memory-card
/// object (`gp + 0x2e8`).
pub fn shared_login_man_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        Addr(r32(c, A0)).sw(c, 0);
        call_ptr(c, 0x003f_7f18, 0x0026_59d0)?;
        c.g[A0][0] = c.lw(r32(c, GP).wrapping_add(0x2e8));
        call_ptr(c, 0x002c_2300, 0x0026_59d8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared shape of the three memory-card result callbacks: fetch the port
/// slot for `code` into a stack result, then post the new state
/// (`gp + 0x2e4`).
fn login_mc_callback(c: &mut Cpu, ret: u32, state: u64) -> Flow {
    c.cycles += 9;
    let frame = Frame::open(c, 0x20);
    frame.save_ra(c, 0x10);
    c.g[A1][0] = c.g[SP][0];
    call_ptr(c, 0x003f_a1d0, ret)?;
    c.g[V1][0] = state;
    frame.restore_ra(c, 0x10);
    Addr(r32(c, GP).wrapping_add(0x2e4)).sw(c, c.g[V1][0]);
    frame.close(c);
    Ok(())
}

/// `void cSharedLoginMan_McCb_State09(int code)` (0x266010): state 9.
pub fn shared_login_man_mc_cb_state09(c: &mut Cpu) {
    guest(c, |c| login_mc_callback(c, 0x0026_6020, 0x9))
}

/// `void cSharedLoginMan_McCb_State0A(int code)` (0x266038): state 0xa.
pub fn shared_login_man_mc_cb_state0a(c: &mut Cpu) {
    guest(c, |c| login_mc_callback(c, 0x0026_6048, 0xa))
}

/// `void cSharedLoginMan_McCb_State03(int code)` (0x2660a0): state 3.
pub fn shared_login_man_mc_cb_state03(c: &mut Cpu) {
    guest(c, |c| login_mc_callback(c, 0x0026_60b0, 0x3))
}

// ---------------------------------------------------------------------------
// TibExtMemFree.

/// `void TibExtMemFree(void* unused, void* p)` (0x266760): free `p` when it is
/// not null.
pub fn tib_ext_mem_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.g[A1][0];
        let none = c.g[A0][0] == 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 2;
            call_ptr(c, MEM_FREE, 0x0026_6778)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// LobbyMan_On*_Thunk.

/// `gp`-relative base of the lobby callback table (`D_004A2FB0`).
const LOBBY_CALLBACKS: u32 = 0x004a_2fb0;

/// The shared shape of the seventeen lobby callbacks: add the (16-bit) lobby
/// object offset published in the callback table to the handler's third
/// argument (`a2`, the lobby object) and tail into the matching `On*` handler.
fn lobby_thunk(c: &mut Cpu, ret: u32, slot: u32, target: u32) -> Flow {
    c.cycles += 9;
    c.g[V0][0] = 0x004a_0000;
    let frame = Frame::open(c, 0x10);
    c.g[A0][0] = Addr(LOBBY_CALLBACKS.wrapping_add(slot)).lh(c);
    frame.save_ra(c, 0);
    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
    call_ptr(c, target, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void LobbyMan_OnUserListChanged_Thunk(void* m, void* h, void* obj)`
/// (0x268a90).
pub fn lobby_man_on_user_list_changed_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8aa8, 0x2fb0, 0x0025_d6c0))
}

/// `void LobbyMan_OnRoomListChanged_Thunk(void* m, void* h, void* obj)`
/// (0x268ab8).
pub fn lobby_man_on_room_list_changed_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8ad0, 0x2fb8, 0x0025_99b0))
}

/// `void LobbyMan_OnRankListChanged_Thunk(void* m, void* h, void* obj)`
/// (0x268ae0).
pub fn lobby_man_on_rank_list_changed_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8af8, 0x2fc0, 0x0025_9a60))
}

/// `void LobbyMan_OnRecvDebug_Thunk(void* m, void* h, void* obj)` (0x268b08).
pub fn lobby_man_on_recv_debug_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8b20, 0x2ff0, 0x0025_98a8))
}

/// `void LobbyMan_OnMessage_Thunk(void* m, void* h, void* obj)` (0x268b30).
pub fn lobby_man_on_message_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8b48, 0x2ff8, 0x0025_9b10))
}

/// `void LobbyMan_OnDisconnect_Thunk(void* m, void* h, void* obj)` (0x268b58).
pub fn lobby_man_on_disconnect_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8b70, 0x3000, 0x0025_98c8))
}

/// `void LobbyMan_OnGameStart_Thunk(void* m, void* h, void* obj)` (0x268b80).
pub fn lobby_man_on_game_start_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8b98, 0x3008, 0x0025_a1d0))
}

/// `void LobbyMan_OnConnectReply_Thunk(void* m, void* h, void* obj)`
/// (0x268ba8).
pub fn lobby_man_on_connect_reply_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8bc0, 0x3020, 0x0025_b848))
}

/// `void LobbyMan_OnCreateAccountReply_Thunk(void* m, void* h, void* obj)`
/// (0x268bd0).
pub fn lobby_man_on_create_account_reply_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8be8, 0x3088, 0x0025_bd30))
}

/// `void LobbyMan_OnEditAccountReply_Thunk(void* m, void* h, void* obj)`
/// (0x268bf8).
pub fn lobby_man_on_edit_account_reply_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8c10, 0x30a8, 0x0025_c0c0))
}

/// `void LobbyMan_OnLostByEmailReply_Thunk(void* m, void* h, void* obj)`
/// (0x268c20).
pub fn lobby_man_on_lost_by_email_reply_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8c38, 0x30b0, 0x0025_c2d8))
}

/// `void LobbyMan_OnLostByNameReply_Thunk(void* m, void* h, void* obj)`
/// (0x268c48).
pub fn lobby_man_on_lost_by_name_reply_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8c60, 0x30b8, 0x0025_c4f0))
}

/// `void LobbyMan_OnLoginReply_Thunk(void* m, void* h, void* obj)` (0x268c70).
pub fn lobby_man_on_login_reply_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8c88, 0x30e8, 0x0025_c8a8))
}

/// `void LobbyMan_OnSelectPersonaReply_Thunk(void* m, void* h, void* obj)`
/// (0x268c98).
pub fn lobby_man_on_select_persona_reply_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8cb0, 0x3118, 0x0025_ceb0))
}

/// `void LobbyMan_OnOwnStatsReply_Thunk(void* m, void* h, void* obj)`
/// (0x268cc0).
pub fn lobby_man_on_own_stats_reply_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8cd8, 0x3120, 0x0025_d048))
}

/// `void LobbyMan_OnCreatePersonaReply_Thunk(void* m, void* h, void* obj)`
/// (0x268ce8).
pub fn lobby_man_on_create_persona_reply_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8d00, 0x3138, 0x0025_d2d8))
}

/// `void LobbyMan_OnDeletePersonaReply_Thunk(void* m, void* h, void* obj)`
/// (0x268d10).
pub fn lobby_man_on_delete_persona_reply_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8d28, 0x3140, 0x0025_d538))
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, LOG_UNWIND_AT};
    use crate::difftest::{scramble, Rng, DATA, GAME_GP};

    const SEEDS: u32 = 60;

    /// The lazily built BELibrary singleton (`gp - 0x1f04`): a zeroed dummy so
    /// the file/allocator paths fail fast instead of waiting on the disc.
    fn be_library(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
    }

    #[test]
    fn ssx_app_set_main_thread_priority_matches() {
        crate::difftest::check(ee::s10::f_0031a6b8, ssx_app_set_main_thread_priority, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x31a6_b8, 2);
            // The app singleton's main thread handle (null covers the
            // "not created yet" path through ChangeThreadPriority).
            c.bus.write32(GAME_GP + APP_MAIN_THREAD_GP, if seed % 3 == 0 { 0 } else { r.next_u32() });
        });
    }

    #[test]
    fn ps2_exec_man_v14_matches() {
        crate::difftest::check(ee::s10::f_0031ab10, ps2_exec_man_v14, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x31ab_10, 1);
            // The body sleeps forever; the fake syscall unwinds after a few
            // iterations, which is how the original leaves the loop too.
            c.bus.write32(LOG_UNWIND_AT, seed % 4);
            c.g[A0][0] = sx(DATA + 0x100);
            c.bus.write32(DATA + 0x10c, r.next_u32());
        });
    }

    #[test]
    fn anim_object_kill_restorable_matches() {
        crate::difftest::check(ee::s11::f_00341fc8, anim_object_kill_restorable, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x341f_c8, 2);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { 1 + (r.next_u32() & 0x10) as u64 };
            // The kill-script handle `WorldObj_RunKillScript` follows
            // (`o + 0x30 + 0x18`).
            c.bus.write32(DATA + 0x48, 0);
        });
    }

    #[test]
    fn multi_particle_update_matches() {
        crate::difftest::check(ee::s11::f_00358120, multi_particle_update, 50, &|c, seed| {
            let mut r = machine(c, seed, 0x3581_20, 2);
            be_library(c);
            c.g[A0][0] = sx(DATA);
            // `p + 0x10` is the particle system; a null (or zeroed) system
            // makes `Emitter_Update` take its "descriptor time 0" exit.
            c.bus.write32(DATA + 0x10, if seed % 2 == 0 { 0 } else { DATA + 0x1000 });
            c.bus.write32(DATA + 0x1000, 0);
            c.bus.write32(GAME_GP.wrapping_sub(0x2a80), r.f32(1.0));
        });
    }

    #[test]
    fn emitter_set_desc_curve_b_matches() {
        crate::difftest::check(ee::s12::f_0036ce28, emitter_set_desc_curve_b, 200, &|c, seed| {
            let mut r = Rng(0x36ce_2801 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.f[12] = r.edge_f32();
            c.f[13] = r.edge_f32();
            c.bus.write32(DATA + 0x4c, r.edge_f32());
        });
    }

    #[test]
    fn ui_text_set_wide_string_matches() {
        crate::difftest::check(ee::s13::f_003a0e90, ui_text_set_wide_string, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x3a0e_90, 2);
            be_library(c);
            c.g[A0][0] = sx(DATA);
            // The wide string: a zeroed run so the length/copy loops stop at
            // the first unit.
            c.g[A1][0] = sx(DATA + 0x1000);
            for k in 0..8 {
                c.bus.write32(DATA + 0x1000 + 4 * k, 0);
            }
            c.bus.write32(DATA + 8, if seed % 3 == 0 { 0 } else { DATA + 0x200 });
        });
    }

    #[test]
    fn ui_pair_flags_ctor_matches() {
        crate::difftest::check(ee::s13::f_003a5508, ui_pair_flags_ctor, 200, &|c, seed| {
            let mut r = Rng(0x3a55_0801 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.bus.write32(DATA, r.next_u32());
        });
    }

    #[test]
    fn mpeg2_picture_temporal_scalable_extension_matches() {
        crate::difftest::check(ee::s13::f_003b22f8, mpeg2_picture_temporal_scalable_extension, 30, &|c, seed| {
            machine(c, seed, 0x3b22_f8, 1);
        });
    }

    #[test]
    fn rider_dtor_free_matches() {
        crate::difftest::check(ee::s00::f_00113170, rider_dtor_free, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x11_3170, 2);
            be_library(c);
            // A null object keeps `operator delete` on its "nothing to free"
            // path; a real one exercises the heap manager.
            if seed % 2 == 1 {
                c.g[A0][0] = 0;
                c.g[A1][0] = 1;
            } else {
                c.g[A0][0] = sx(DATA + (r.next_u32() & 0xfff0));
                c.g[A1][0] = sx(r.next_u32() & !1);
            }
        });
    }

    #[test]
    fn be_mission_int_count_missions_on_current_track_matches() {
        crate::difftest::check(
            ee::s01::f_00154278,
            be_mission_int_count_missions_on_current_track,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1542_78, 2);
                // The current track id (`D_005305F0`); 0 makes every table
                // entry match, other values only some.
                c.bus.write32(0x0053_05f0, if seed % 3 == 0 { 0 } else { r.next_u32() });
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn be_mission_int_get_nth_mission_on_current_track_matches() {
        crate::difftest::check(
            ee::s01::f_00154588,
            be_mission_int_get_nth_mission_on_current_track,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1545_88, 2);
                c.bus.write32(0x0053_05f0, if seed % 3 == 0 { 0 } else { r.next_u32() });
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 4) as u64;
            },
        );
    }

    #[test]
    fn be_reward_int_load_rewards_file_matches() {
        crate::difftest::check(ee::s01::f_00156a10, be_reward_int_load_rewards_file, 30, &|c, seed| {
            let mut r = machine(c, seed, 0x156a_10, 2);
            be_library(c);
            // `RewardsFile_Load` reads the world's file slot (`world + 0x7c`)
            // and the path table (`gp - 0x1e80`); a dummy world keeps the
            // bolt-file lookup on its "no such slot" exit.
            c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x2000);
            c.bus.write32(DATA + 0x207c, 0);
            c.bus.write32(GAME_GP.wrapping_sub(0x1e80), 0);
            c.bus.write32(DATA, r.next_u32());
        });
    }

    #[test]
    fn chase_cam_controller_set_algorithm_immediate_matches() {
        crate::difftest::check(
            ee::s01::f_0015d050,
            chase_cam_controller_set_algorithm_immediate,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15d0_50, 2);
                be_library(c);
                // Algorithm indices below 0x62 go through the (unloaded, all
                // zero) jump table, which leaves the function at once.
                c.g[A1][0] = (r.next_u32() % 0x62) as u64;
                c.g[A2][0] = sx(DATA + 0x400);
            },
        );
    }

    #[test]
    fn script_cam_controller_push_algorithm_matches() {
        crate::difftest::check(ee::s02::f_00169340, script_cam_controller_push_algorithm, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x1693_40, 2);
            be_library(c);
            c.g[A0][0] = sx(DATA);
            // The new algorithm; a null vtable makes its "notify" call go to
            // the fake hook (or be skipped).
            c.g[A1][0] = sx(DATA + 0x300);
            c.bus.write32(DATA + 8, if seed % 2 == 0 { 0 } else { DATA + 0x500 });
            c.bus.write32(DATA + 0x300 + 0x10, if seed % 3 == 0 { 0 } else { DATA + 0x600 });
            c.bus.write32(DATA + 0x600 + 0x24, 0);
            c.bus.write32(DATA + 0x500 + 4, r.next_u32());
        });
    }

    #[test]
    fn camera_trigger_list_dtor_matches() {
        crate::difftest::check(ee::s02::f_0016bef8, camera_trigger_list_dtor, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x16be_f8, 2);
            be_library(c);
            if seed % 2 == 1 {
                c.g[A0][0] = 0;
                c.g[A1][0] = 1;
            } else {
                c.g[A0][0] = sx(DATA + (r.next_u32() & 0xfff0));
                c.g[A1][0] = sx(r.next_u32() & !1);
            }
        });
    }

    #[test]
    fn camera_trigger_man_empty_global_call_matches() {
        crate::difftest::check(ee::s02::f_0016cf18, camera_trigger_man_empty_global_call, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x16cf_18, 1);
            c.g[A0][0] = if seed % 3 == 0 { 0 } else { sx(DATA + (r.next_u32() & 0xfff0)) };
        });
    }

    #[test]
    fn fe_state_uber_trick_transition_in_matches() {
        crate::difftest::check(ee::s02::f_00184bb8, fe_state_uber_trick_transition_in, 40, &|c, seed| {
            machine(c, seed, 0x184b_b8, 2);
            be_library(c);
            // A zeroed state object: its trick lists are empty, so the
            // refresh walks terminate at once.
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    #[test]
    fn fe_state_memcard_update_card_scan_matches() {
        crate::difftest::check(ee::s02::f_00187180, fe_state_memcard_update_card_scan, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x1871_80, 2);
            be_library(c);
            // `D_004A14B8`, the card scan state the second call advances.
            c.bus.write32(GAME_GP.wrapping_sub(0x1c38), if seed % 2 == 0 { 0 } else { DATA + 0x800 });
            c.g[A0][0] = sx(DATA);
            c.bus.write32(DATA + 0x800, r.next_u32() % 4);
        });
    }

    #[test]
    fn net_config_auth_popup_on_widget_action_matches() {
        crate::difftest::check(ee::s03::f_001b6770, net_config_auth_popup_on_widget_action, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x1b67_70, 2);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x300);
            c.g[A2][0] = if seed % 2 == 0 { 0xd } else { (r.next_u32() & 0xff) as u64 };
            c.bus.write32(DATA + 4, r.next_u32());
        });
    }

    #[test]
    fn fe_locale_set_flag27_matches() {
        crate::difftest::check(ee::s04::f_001e1090, fe_locale_set_flag27, 200, &|c, seed| {
            let mut r = Rng(0x1e10_9001 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.bus.write32(DATA, r.next_u32());
            c.g[A1][0] = if seed % 3 == 0 { 0 } else { r.next_u32() as u64 };
        });
    }

    #[test]
    fn ov_state_view_popup_transition_in_matches() {
        crate::difftest::check(ee::s04::f_001e52d0, ov_state_view_popup_transition_in, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x1e52_d0, 2);
            be_library(c);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 2) as u64;
            c.bus.write32(DATA, r.next_u32());
        });
    }

    #[test]
    fn fe_state_buy_attrib_transition_in_matches() {
        crate::difftest::check(ee::s04::f_001f49d0, fe_state_buy_attrib_transition_in, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x1f49_d0, 2);
            be_library(c);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 2) as u64;
            c.bus.write32(DATA, r.next_u32());
        });
    }

    #[test]
    fn fe_state_career_stats_transition_in_matches() {
        crate::difftest::check(ee::s04::f_001f55c0, fe_state_career_stats_transition_in, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x1f55_c0, 2);
            be_library(c);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 2) as u64;
            c.bus.write32(DATA, r.next_u32());
        });
    }

    #[test]
    fn section_man_dtor_matches() {
        crate::difftest::check(ee::s05::f_0022cd18, section_man_dtor, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x22cd_18, 2);
            be_library(c);
            if seed % 2 == 1 {
                c.g[A0][0] = 0;
                c.g[A1][0] = 1;
            } else {
                c.g[A0][0] = sx(DATA + (r.next_u32() & 0xfff0));
                c.g[A1][0] = sx(r.next_u32() & !1);
            }
        });
    }

    #[test]
    fn game_module_invalidate_focus_matches() {
        crate::difftest::check(ee::s06::f_00230338, game_module_invalidate_focus, 200, &|c, seed| {
            let mut r = Rng(0x2303_3801 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            match seed % 3 {
                0 => c.bus.write32(DATA + 0xc, 0),
                1 => {
                    c.bus.write32(DATA + 0xc, DATA + 0x100);
                    c.bus.write32(DATA + 0x100 + 0xa4, 0);
                }
                _ => {
                    c.bus.write32(DATA + 0xc, DATA + 0x100);
                    c.bus.write32(DATA + 0x100 + 0xa4, DATA + 0x200);
                    c.bus.write32(DATA + 0x200 + 0xd0, r.next_u32());
                }
            }
        });
    }

    #[test]
    fn game_state_results_exit_matches() {
        crate::difftest::check(ee::s06::f_00236ea0, game_state_results_exit, 40, &|c, seed| {
            machine(c, seed, 0x236e_a0, 2);
            be_library(c);
            // `D_004A2C68` (the game state manager): a dummy whose `+0x48`
            // state list head is empty, so the close-all walk stops at once.
            c.bus.write32(GAME_GP.wrapping_sub(0x488), DATA + 0x2000);
            c.bus.write32(DATA + 0x2048, if seed % 2 == 0 { 0 } else { DATA + 0x2000 });
            c.g[A0][0] = sx(DATA);
        });
    }

    #[test]
    fn game_rand_seed_matches() {
        crate::difftest::check(ee::s06::f_00237cb0, game_rand_seed, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x237c_b0, 1);
            c.g[A0][0] = if seed % 3 == 0 { 0 } else { r.next_u32() as u64 };
        });
    }

    #[test]
    fn occlusion_man_dtor_matches() {
        crate::difftest::check(ee::s06::f_00242518, occlusion_man_dtor, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x2425_18, 2);
            be_library(c);
            if seed % 2 == 1 {
                c.g[A0][0] = 0;
                c.g[A1][0] = 1;
            } else {
                c.g[A0][0] = sx(DATA + (r.next_u32() & 0xfff0));
                c.g[A1][0] = sx(r.next_u32() & !1);
            }
        });
    }

    #[test]
    fn shared_login_man_shutdown_matches() {
        crate::difftest::check(ee::s07::f_002659c0, shared_login_man_shutdown, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x2659_c0, 2);
            be_library(c);
            c.g[A0][0] = sx(DATA);
            // The memory-card object (`gp + 0x2e8`): null skips the destroy.
            c.bus.write32(GAME_GP + 0x2e8, if seed % 2 == 0 { 0 } else { DATA + 0x800 });
            c.bus.write32(DATA, r.next_u32());
        });
    }

    #[test]
    fn shared_login_man_mc_cb_state09_matches() {
        crate::difftest::check(ee::s07::f_00266010, shared_login_man_mc_cb_state09, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x2660_10, 2);
            c.g[A0][0] = (seed % 10) as u64;
            c.bus.write32(GAME_GP + 0x1aec, if seed % 2 == 0 { 0 } else { 1 });
            c.bus.write32(GAME_GP + 0x2e4, r.next_u32());
        });
    }

    #[test]
    fn shared_login_man_mc_cb_state0a_matches() {
        crate::difftest::check(ee::s07::f_00266038, shared_login_man_mc_cb_state0a, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x2660_38, 2);
            c.g[A0][0] = (seed % 10) as u64;
            c.bus.write32(GAME_GP + 0x1aec, if seed % 2 == 0 { 0 } else { 1 });
            c.bus.write32(GAME_GP + 0x2e4, r.next_u32());
        });
    }

    #[test]
    fn shared_login_man_mc_cb_state03_matches() {
        crate::difftest::check(ee::s07::f_002660a0, shared_login_man_mc_cb_state03, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x2660_a0, 2);
            c.g[A0][0] = (seed % 10) as u64;
            c.bus.write32(GAME_GP + 0x1aec, if seed % 2 == 0 { 0 } else { 1 });
            c.bus.write32(GAME_GP + 0x2e4, r.next_u32());
        });
    }

    #[test]
    fn tib_ext_mem_free_matches() {
        crate::difftest::check(ee::s07::f_00266760, tib_ext_mem_free, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x2667_60, 2);
            be_library(c);
            c.g[A0][0] = sx(r.next_u32());
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { sx(DATA + (r.next_u32() & 0xfff0)) };
        });
    }

    /// One lobby-thunk test: the handler's `a2` points at a zeroed object, the
    /// callback table slot holds a small offset (so the `lh` + add is
    /// exercised) and `a1` a zeroed packet.
    fn lobby_setup(c: &mut Cpu, seed: u32, salt: u64, slot: u32) {
        let mut r = machine(c, seed, salt, 2);
        be_library(c);
        c.g[A1][0] = sx(DATA + 0x400);
        c.g[A2][0] = sx(DATA + 0x10);
        c.bus.write16(LOBBY_CALLBACKS.wrapping_add(slot), if seed % 3 == 0 { 0 } else { (4 * (r.next_u32() % 8)) as u16 });
        // The packet `LobbyApi_DebugPacket`/`TagField*` read: zeroed fields.
        for k in 0..8 {
            c.bus.write32(DATA + 0x400 + 4 * k, 0);
        }
    }

    #[test]
    fn lobby_man_on_user_list_changed_thunk_matches() {
        crate::difftest::check(ee::s07::f_00268a90, lobby_man_on_user_list_changed_thunk, 30, &|c, seed| {
            lobby_setup(c, seed, 0x268a_90, 0x2fb0);
        });
    }

    #[test]
    fn lobby_man_on_room_list_changed_thunk_matches() {
        crate::difftest::check(ee::s07::f_00268ab8, lobby_man_on_room_list_changed_thunk, 30, &|c, seed| {
            lobby_setup(c, seed, 0x268a_b8, 0x2fb8);
        });
    }

    #[test]
    fn lobby_man_on_rank_list_changed_thunk_matches() {
        crate::difftest::check(ee::s07::f_00268ae0, lobby_man_on_rank_list_changed_thunk, 30, &|c, seed| {
            lobby_setup(c, seed, 0x268a_e0, 0x2fc0);
        });
    }

    #[test]
    fn lobby_man_on_recv_debug_thunk_matches() {
        crate::difftest::check(ee::s07::f_00268b08, lobby_man_on_recv_debug_thunk, 30, &|c, seed| {
            lobby_setup(c, seed, 0x268b_08, 0x2ff0);
        });
    }

    #[test]
    fn lobby_man_on_message_thunk_matches() {
        crate::difftest::check(ee::s07::f_00268b30, lobby_man_on_message_thunk, 24, &|c, seed| {
            lobby_setup(c, seed, 0x268b_30, 0x2ff8);
        });
    }

    #[test]
    fn lobby_man_on_disconnect_thunk_matches() {
        crate::difftest::check(ee::s07::f_00268b58, lobby_man_on_disconnect_thunk, 30, &|c, seed| {
            lobby_setup(c, seed, 0x268b_58, 0x3000);
        });
    }

    #[test]
    fn lobby_man_on_game_start_thunk_matches() {
        crate::difftest::check(ee::s07::f_00268b80, lobby_man_on_game_start_thunk, 24, &|c, seed| {
            lobby_setup(c, seed, 0x268b_80, 0x3008);
        });
    }

    #[test]
    fn lobby_man_on_connect_reply_thunk_matches() {
        crate::difftest::check(ee::s07::f_00268ba8, lobby_man_on_connect_reply_thunk, 30, &|c, seed| {
            lobby_setup(c, seed, 0x268b_a8, 0x3020);
        });
    }

    #[test]
    fn lobby_man_on_create_account_reply_thunk_matches() {
        crate::difftest::check(ee::s07::f_00268bd0, lobby_man_on_create_account_reply_thunk, 30, &|c, seed| {
            lobby_setup(c, seed, 0x268b_d0, 0x3088);
        });
    }

    #[test]
    fn lobby_man_on_edit_account_reply_thunk_matches() {
        crate::difftest::check(ee::s07::f_00268bf8, lobby_man_on_edit_account_reply_thunk, 30, &|c, seed| {
            lobby_setup(c, seed, 0x268b_f8, 0x30a8);
        });
    }

    #[test]
    fn lobby_man_on_lost_by_email_reply_thunk_matches() {
        crate::difftest::check(ee::s07::f_00268c20, lobby_man_on_lost_by_email_reply_thunk, 30, &|c, seed| {
            lobby_setup(c, seed, 0x268c_20, 0x30b0);
        });
    }

    #[test]
    fn lobby_man_on_lost_by_name_reply_thunk_matches() {
        crate::difftest::check(ee::s07::f_00268c48, lobby_man_on_lost_by_name_reply_thunk, 30, &|c, seed| {
            lobby_setup(c, seed, 0x268c_48, 0x30b8);
        });
    }

    #[test]
    fn lobby_man_on_login_reply_thunk_matches() {
        crate::difftest::check(ee::s07::f_00268c70, lobby_man_on_login_reply_thunk, 24, &|c, seed| {
            lobby_setup(c, seed, 0x268c_70, 0x30e8);
        });
    }

    #[test]
    fn lobby_man_on_select_persona_reply_thunk_matches() {
        crate::difftest::check(ee::s07::f_00268c98, lobby_man_on_select_persona_reply_thunk, 30, &|c, seed| {
            lobby_setup(c, seed, 0x268c_98, 0x3118);
        });
    }

    #[test]
    fn lobby_man_on_own_stats_reply_thunk_matches() {
        crate::difftest::check(ee::s07::f_00268cc0, lobby_man_on_own_stats_reply_thunk, 30, &|c, seed| {
            lobby_setup(c, seed, 0x268c_c0, 0x3120);
        });
    }

    #[test]
    fn lobby_man_on_create_persona_reply_thunk_matches() {
        crate::difftest::check(ee::s07::f_00268ce8, lobby_man_on_create_persona_reply_thunk, 30, &|c, seed| {
            lobby_setup(c, seed, 0x268c_e8, 0x3138);
        });
    }

    #[test]
    fn lobby_man_on_delete_persona_reply_thunk_matches() {
        crate::difftest::check(ee::s07::f_00268d10, lobby_man_on_delete_persona_reply_thunk, 30, &|c, seed| {
            lobby_setup(c, seed, 0x268d_10, 0x3140);
        });
    }
}
