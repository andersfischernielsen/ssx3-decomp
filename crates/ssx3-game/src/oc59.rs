//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for the script-camera reset hooks, the camera trigger stack,
//! front-end state hooks, the game load-state destructors, the memory-manager
//! tag/alloc helpers, the lobby-man challenge plumbing, the shared login
//! manager's memory-card requests, the script-engine lookups and the sound
//! bank-manager wrappers, each with a differential test against the generated
//! body.
//!
//! Almost all of these are forwarding wrappers: a stack frame, one (or two)
//! calls, and the callee's result. The port keeps the exact frame, the cycle
//! charges of each block, the scratch registers the generated code leaves in
//! `v0`/`v1`/`a*` at the call, and the unwind check through [`crate::call`].
//!
//! Two groups are not wrappers:
//! - `MEM_GetBlockTag` reads the allocation tag out of a memory block's
//!   header (bit `0x800` of the size word at `-0xe`, then the unaligned tag
//!   pointer at `-0xc`).
//! - `ScriptViewObj_GetSlotCam`, `ScriptMovie_GetId` and
//!   `SSXAudioSystem_IsGameMusicEnabled` are pure leaf functions.

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, Callee, Flow, Frame};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0016_9518, manual_script_camera_algo_reset_for_rider, "ManualScriptCameraAlgo_ResetForRider"),
    (0x0016_9d58, target_script_camera_algo_reset_for_rider, "TargetScriptCameraAlgo_ResetForRider"),
    (0x0016_a4b8, subject_script_camera_algo_reset_for_rider, "SubjectScriptCameraAlgo_ResetForRider"),
    (0x0016_d210, camera_trigger_stack_ctor, "CameraTriggerStack_ctor"),
    (0x0017_6840, camera_trigger_array_45c1d0_v01, "CameraTriggerArray_45c1d0_v01"),
    (0x0017_6868, camera_trigger_array_45c1b8_v01, "CameraTriggerArray_45c1b8_v01"),
    (0x0018_6950, c_fe_state_event_select_on_widget_notify, "cFEStateEventSelect_OnWidgetNotify"),
    (0x0019_cb38, bolt_file_index_manager_ctor, "BoltFileIndexManager_ctor"),
    (0x001a_0570, fe_module_set_preview_visible, "FEModule_SetPreviewVisible"),
    (0x0020_8610, ov_screen_map_on_widget_notify, "OVScreenMap_OnWidgetNotify"),
    (0x0023_1320, game_module_advance_state, "GameModule_AdvanceState"),
    (0x0023_3b60, game_state_quit_enter, "GameStateQuit_Enter"),
    (0x0023_5f20, game_state_restart_race_exit, "GameStateRestartRace_Exit"),
    (0x0023_7038, game_state_replay_playback_exit, "GameStateReplayPlayback_Exit"),
    (0x0024_53f0, c_game_load_state_dtor, "cGameLoadState_dtor"),
    (0x0024_5600, c_game_load_state_out_lodge_dtor, "cGameLoadStateOutLodge_dtor"),
    (0x0024_5708, c_game_load_state_conquer_dtor, "cGameLoadStateConquer_dtor"),
    (0x0024_5928, c_fe_load_hint_state_dtor, "cFELoadHintState_dtor"),
    (0x0024_5b58, c_fe_load_state_dtor, "cFELoadState_dtor"),
    (0x0024_5d28, c_fe_load_state_in_lodge_dtor, "cFELoadStateInLodge_dtor"),
    (0x0025_2660, mem_get_block_tag, "MEM_GetBlockTag"),
    (0x0025_2fa0, mem_alloc_named128, "MEM_AllocNamed128"),
    (0x0025_fb58, lobby_man_accept_challenge, "LobbyMan_AcceptChallenge"),
    (0x0026_4098, c_setup_man_destroy, "cSetupMan_Destroy"),
    (0x0026_58b8, c_shared_login_man_destroy, "cSharedLoginMan_Destroy"),
    (0x0026_6148, c_shared_login_man_mc_close, "cSharedLoginMan_McClose"),
    (0x0026_6298, c_shared_login_man_mc_read, "cSharedLoginMan_McRead"),
    (0x0026_62c0, c_shared_login_man_mc_write, "cSharedLoginMan_McWrite"),
    (0x0026_6320, c_shared_login_man_mc_flush, "cSharedLoginMan_McFlush"),
    (0x0026_6550, c_shared_login_man_mc_get_found_name, "cSharedLoginMan_McGetFoundName"),
    (0x0026_6cd8, c_voip_man_destroy, "cVoipMan_Destroy"),
    (0x0027_9220, script_engine_list_get_play_status, "ScriptEngine_ListGetPlayStatus"),
    (0x0027_9248, script_engine_list_count, "ScriptEngine_ListCount"),
    (0x0027_97a0, script_engine_find_script_index, "ScriptEngine_FindScriptIndex"),
    (0x0027_a688, script_engine_get_world_section2b, "ScriptEngine_GetWorldSection2B"),
    (0x0028_1c40, script_view_obj_get_slot_cam, "ScriptViewObj_GetSlotCam"),
    (0x0028_2bf0, script_engine_get_process_in_seq, "ScriptEngine_GetProcessInSeq"),
    (0x0028_2f90, script_binding_clear_listener, "ScriptBinding_ClearListener"),
    (0x0028_3038, script_binding_pause_seq, "ScriptBinding_PauseSeq"),
    (0x0028_3060, script_binding_resume_seq, "ScriptBinding_ResumeSeq"),
    (0x0028_3bb8, script_movie_get_id, "ScriptMovie_GetId"),
    (0x0028_3c30, script_cmd_queue_ctor, "ScriptCmdQueue_ctor"),
    (0x0028_bce8, snd_bank_man_unload_bank, "SndBankMan_UnloadBank"),
    (0x0028_bde0, snd_bank_man_free_bank_memory, "SndBankMan_FreeBankMemory"),
    (0x0028_be90, snd_bank_man_unload_memory_bank, "SndBankMan_UnloadMemoryBank"),
    (0x0028_beb8, snd_bank_man_release_bank, "SndBankMan_ReleaseBank"),
    (0x0028_bf10, snd_bank_man_get_bank_buffer_size, "SndBankMan_GetBankBufferSize"),
    (0x0028_d960, ssx_audio_system_is_game_music_enabled, "SSXAudioSystem_IsGameMusicEnabled"),
    (0x002a_d398, bank_monitor_reset_instance, "BankMonitor_ResetInstance"),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const CAMERA_TRIGGER_STACK_CLEAR: Callee =
        Callee { addr: 0x0016_d200, f: ee::s02::CameraTriggerStack_Clear };
    pub const BOLT_FILE_INDEX_MANAGER_RESET: Callee =
        Callee { addr: 0x0019_cb60, f: ee::s03::BoltFileIndexManager_Reset };
    pub const FE_MODULE_RIDER_PREVIEW_SET_VISIBLE: Callee =
        Callee { addr: 0x0019_e538, f: ee::s03::FEModule_RiderPreviewSetVisible };
    pub const EVENT_SELECT_ON_WIDGET_NOTIFY: Callee =
        Callee { addr: 0x0020_2738, f: ee::s05::EventSelect_OnWidgetNotify };
    pub const GAME_STATE_GOTO_NEXT: Callee =
        Callee { addr: 0x0023_3aa0, f: ee::s06::GameState_GotoNext };
    pub const GAME_MODULE_REQUEST_STATE: Callee =
        Callee { addr: 0x0023_1250, f: ee::s06::GameModule_RequestState };
    pub const MEM_ALLOC_NAMED_ALIGNED: Callee =
        Callee { addr: 0x0025_2980, f: ee::s06::MEM_AllocNamedAligned };
    pub const LOBBY_MAN_SEND_LOCK_CHAL: Callee =
        Callee { addr: 0x0025_fc70, f: ee::s06::LobbyMan_SendLockChal };
    pub const C_SETUP_MAN_DTOR: Callee =
        Callee { addr: 0x0026_4138, f: ee::s07::cSetupMan_dtor };
    pub const C_SHARED_LOGIN_MAN_DTOR: Callee =
        Callee { addr: 0x0026_5908, f: ee::s07::cSharedLoginMan_dtor };
    pub const C_VOIP_MAN_DTOR: Callee =
        Callee { addr: 0x0026_6d10, f: ee::s07::cVoipMan_dtor };
    pub const SCRIPT_LIST_GET_PLAY_STATUS: Callee =
        Callee { addr: 0x0027_6bd8, f: ee::s07::ScriptList_GetPlayStatus };
    pub const SCRIPT_LIST_COUNT: Callee =
        Callee { addr: 0x0027_6ca8, f: ee::s07::ScriptList_Count };
    pub const SCRIPT_ENGINE_CHOOSE_SCRIPT: Callee =
        Callee { addr: 0x0027_b0c0, f: ee::s07::ScriptEngine_ChooseScript };
    pub const SCRIPT_ENGINE_GET_PROCESS: Callee =
        Callee { addr: 0x0028_2cb0, f: ee::s07::ScriptEngine_GetProcess };
    pub const IN_SEQ_CHANNEL_SET_LISTENER: Callee =
        Callee { addr: 0x0027_4240, f: ee::s07::InSeq_ChannelSetListener };
    pub const IN_SEQ_INSTANCE_PAUSE: Callee =
        Callee { addr: 0x0027_49e8, f: ee::s07::InSeq_InstancePause };
    pub const IN_SEQ_INSTANCE_RESUME: Callee =
        Callee { addr: 0x0027_4a08, f: ee::s07::InSeq_InstanceResume };
    pub const SCRIPT_CMD_QUEUE_CLEAR: Callee =
        Callee { addr: 0x0028_3c20, f: ee::s07::ScriptCmdQueue_Clear };
    pub const SND_BANK_UNLOAD: Callee = Callee { addr: 0x0028_b528, f: ee::s07::SndBank_Unload };
    pub const SND_BANK_FREE_MEMORY: Callee =
        Callee { addr: 0x0028_b650, f: ee::s07::SndBank_FreeMemory };
    pub const SND_BANK_UNLOAD_MEMORY: Callee =
        Callee { addr: 0x0028_b7e0, f: ee::s07::SndBank_UnloadMemory };
    pub const SND_BANK_RELEASE: Callee = Callee { addr: 0x0028_b878, f: ee::s07::SndBank_Release };
    pub const SND_BANK_GET_BUFFER_SIZE: Callee =
        Callee { addr: 0x0028_b928, f: ee::s07::SndBank_GetBufferSize };
    pub const BANK_MONITOR_INSTANCE_RESET: Callee =
        Callee { addr: 0x002a_bb38, f: ee::s08::BankMonitorInstance_Reset };

    /// Cross-shard targets (`call_indirect` in the generated body).
    pub const OPERATOR_DELETE: u32 = 0x0031_7e50;
    pub const LOAD_INDICATOR_CLEAR_BUSY: u32 = 0x001f_36d8;
    pub const WSCRIPT_MAN_DELETE_ALL_PROCESSES: u32 = 0x0030_8f38;
    pub const WSCRIPT_MAN_SPAWN_MISSIONS: u32 = 0x0030_8c60;
    pub const UI_STATE_DTOR: u32 = 0x0039_e390;
    pub const MC_REQUEST_CLOSE: u32 = 0x002c_6a08;
    pub const MC_REQUEST_READ: u32 = 0x002c_6b50;
    pub const MC_REQUEST_WRITE: u32 = 0x002c_6bd0;
    pub const MC_REQUEST_FLUSH: u32 = 0x002c_6ae0;
    pub const STRCPY: u32 = 0x0041_66f4;
    pub const SECTION_MAN_GET_WORLD_SECTION_INDEX: u32 = 0x0022_e0c8;
}

// ---------------------------------------------------------------------------
// Small register helpers.

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// A 32-bit guest pointer argument.
#[inline(always)]
fn arg(c: &Cpu, i: usize) -> u32 {
    abi::arg(c, i)
}

// ---------------------------------------------------------------------------
// Script camera algorithms: the reset hook forwards to `self->v05()`.

/// `void ManualScriptCameraAlgo_ResetForRider(ScriptCamAlgo* self)`
/// (0x169518), `Target...` (0x169d58) and `Subject...` (0x16a4b8): call the
/// algorithm's `v05` virtual method with the object.
fn script_camera_algo_reset_for_rider(c: &mut Cpu, ret: u32) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    let obj = arg(c, 0);
    let vptr = Addr(obj.wrapping_add(0x10)).lw(c);
    c.g[V0][0] = vptr;
    let delta = Addr((vptr as u32).wrapping_add(0x28)).lh(c);
    c.g[V1][0] = delta;
    let method = Addr((vptr as u32).wrapping_add(0x2c)).lw(c);
    c.g[A1][0] = method;
    c.g[A0][0] = sx(obj.wrapping_add(delta as u32));
    call_ptr(c, method as u32, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// The manual script camera algorithm's `ResetForRider`.
pub fn manual_script_camera_algo_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| script_camera_algo_reset_for_rider(c, 0x0016_9534))
}

/// The target script camera algorithm's `ResetForRider`.
pub fn target_script_camera_algo_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| script_camera_algo_reset_for_rider(c, 0x0016_9d74))
}

/// The subject script camera algorithm's `ResetForRider`.
pub fn subject_script_camera_algo_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| script_camera_algo_reset_for_rider(c, 0x0016_a4d4))
}

// ---------------------------------------------------------------------------
// Camera trigger stack and array element destructors.

/// `cCameraTriggerStack* CameraTriggerStack_ctor(cCameraTriggerStack* self)`
/// (0x16d210): clear the stack and return it.
pub fn camera_trigger_stack_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::CAMERA_TRIGGER_STACK_CLEAR.call(c, 0x0016_d224)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void CameraTriggerArray_45c1d0_v01(void* self)` (0x176840): delete the
/// heap array hanging off `+0x8` if there is one.
pub fn camera_trigger_array_45c1d0_v01(c: &mut Cpu) {
    delete_array_member(c, 0x0017_685c)
}

/// `void CameraTriggerArray_45c1b8_v01(void* self)` (0x176868): same
/// destructor for the second camera-trigger array.
pub fn camera_trigger_array_45c1b8_v01(c: &mut Cpu) {
    delete_array_member(c, 0x0017_6884)
}

/// `int* p = *(int**)((char*)self + 8); if (p) operator_delete(p);`
fn delete_array_member(c: &mut Cpu, ret: u32) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(arg(c, 0).wrapping_add(0x8)).lw(c);
        let none = c.g[A0][0] == 0;
        frame.restore_ra(c, 0);
        if !none {
            c.cycles += 3;
            call_ptr(c, callee::OPERATOR_DELETE, ret)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end state hooks.

/// `int cFEStateEventSelect_OnWidgetNotify(self, a1, notify)` (0x186950): run
/// the event-select notify on the state object at `self+0x48`, and report
/// `0x101` when it handled the notification, else 0.
pub fn c_fe_state_event_select_on_widget_notify(c: &mut Cpu) {
    event_select_notify(c, 0x48, 0x0018_6960)
}

/// `int OVScreenMap_OnWidgetNotify(self, a1, notify)` (0x208610): the same
/// notify hook on the screen map's event-select member at `self+0x9c`.
pub fn ov_screen_map_on_widget_notify(c: &mut Cpu) {
    event_select_notify(c, 0x9c, 0x0020_8620)
}

/// `if (EventSelect_OnWidgetNotify(self + off, a1, notify)) return 0x101;`
fn event_select_notify(c: &mut Cpu, off: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(arg(c, 0).wrapping_add(off));
        call_ptr(c, callee::EVENT_SELECT_ON_WIDGET_NOTIFY.addr, ret)?;
        c.g[V1][0] = 0x101;
        frame.restore_ra(c, 0);
        if c.g[V0][0] == 0 {
            c.g[V1][0] = 0;
        }
        c.g[V0][0] = c.g[V1][0];
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Other constructors that clear their object.

/// `void* BoltFileIndexManager_ctor(void* self)` (0x19cb38): reset the two
/// arrays and return `self`.
pub fn bolt_file_index_manager_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::BOLT_FILE_INDEX_MANAGER_RESET.call(c, 0x0019_cb4c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptCmdQueue_ctor(ScriptCmdQueue* self)` (0x283c30): clear the
/// queue and return it.
pub fn script_cmd_queue_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_CMD_QUEUE_CLEAR.call(c, 0x0028_3c44)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end module: preview visibility.

/// `void FEModule_SetPreviewVisible(self, int rider, int visible)`
/// (0x1a0570): forward to the rider preview object at `self + rider*0xce0`.
pub fn fe_module_set_preview_visible(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[V0][0] = 0xce0;
        let frame = Frame::open(c, 0x10);
        c.mult(0, V1, A1, V0);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = sx(arg(c, 0).wrapping_add(c.g[V1][0] as u32));
        callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE.call(c, 0x001a_058c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Game states.

/// `void GameModule_AdvanceState(GameModule* self)` (0x231320): if the module
/// has a pending state at `+0x200`, ask it for the next state.
pub fn game_module_advance_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(arg(c, 0).wrapping_add(0x200)).lw(c);
        let none = c.g[A0][0] == 0;
        frame.restore_ra(c, 0);
        if !none {
            c.cycles += 3;
            callee::GAME_STATE_GOTO_NEXT.call(c, 0x0023_133c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void GameStateQuit_Enter(void)` (0x233b60): request game state 10 (quit).
pub fn game_state_quit_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[A0][0] = Addr(gp(c, -0x488)).lw(c);
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xa;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        callee::GAME_MODULE_REQUEST_STATE.call(c, 0x0023_3b7c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameStateRestartRace_Exit(void)` (0x235f20): clear flag 3 of the
/// race manager's load indicator.
pub fn game_state_restart_race_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save_ra(c, 0);
        c.g[A1][0] = 3;
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x94)).lw(c);
        call_ptr(c, callee::LOAD_INDICATOR_CLEAR_BUSY, 0x0023_5f3c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameStateReplayPlayback_Exit(void)` (0x237038): tear down the
/// playback world script manager's processes and respawn its missions.
pub fn game_state_replay_playback_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(gp(c, 0xce8)).lw(c);
        frame.save_ra(c, 0);
        call_ptr(c, callee::WSCRIPT_MAN_DELETE_ALL_PROCESSES, 0x0023_704c)?;
        c.g[A0][0] = Addr(gp(c, 0xce8)).lw(c);
        call_ptr(c, callee::WSCRIPT_MAN_SPAWN_MISSIONS, 0x0023_7054)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Game/load-state destructors: store the derived vtable and run UIState_dtor.

/// Store `vtable` at `self+0x8` and call `UIState_dtor(self)`.
fn ui_state_dtor_with_vtable(c: &mut Cpu, vtable: u32, ret: u32) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = 0x480000;
    frame.save_ra(c, 0);
    c.g[V1][0] = c.g[A0][0];
    c.g[V0][0] = sx(0x0048_0000u32.wrapping_sub(vtable));
    Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
    call_ptr(c, callee::UI_STATE_DTOR, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void cGameLoadState_dtor(self)` (0x2453f0).
pub fn c_game_load_state_dtor(c: &mut Cpu) {
    guest(c, |c| ui_state_dtor_with_vtable(c, 0x36b8, 0x0024_540c))
}

/// `void cGameLoadStateOutLodge_dtor(self)` (0x245600).
pub fn c_game_load_state_out_lodge_dtor(c: &mut Cpu) {
    guest(c, |c| ui_state_dtor_with_vtable(c, 0x3788, 0x0024_561c))
}

/// `void cGameLoadStateConquer_dtor(self)` (0x245708).
pub fn c_game_load_state_conquer_dtor(c: &mut Cpu) {
    guest(c, |c| ui_state_dtor_with_vtable(c, 0x3858, 0x0024_5724))
}

/// `void cFELoadHintState_dtor(self)` (0x245928).
pub fn c_fe_load_hint_state_dtor(c: &mut Cpu) {
    guest(c, |c| ui_state_dtor_with_vtable(c, 0x3928, 0x0024_5944))
}

/// `void cFELoadState_dtor(self)` (0x245b58).
pub fn c_fe_load_state_dtor(c: &mut Cpu) {
    guest(c, |c| ui_state_dtor_with_vtable(c, 0x39f8, 0x0024_5b74))
}

/// `void cFELoadStateInLodge_dtor(self)` (0x245d28).
pub fn c_fe_load_state_in_lodge_dtor(c: &mut Cpu) {
    guest(c, |c| ui_state_dtor_with_vtable(c, 0x3ac8, 0x0024_5d44))
}

// ---------------------------------------------------------------------------
// Memory manager.

/// The tag of an allocated block whose size word has bit 0x800 set, at
/// `+4` of the tag record the block's `-0xc` word points at.
const BLOCK_HEADER_SIZE: u32 = 0xe;
const BLOCK_TAG_POINTER: u32 = 0xc;
const BLOCK_TAGGED: u32 = 0x800;

/// `int MEM_GetBlockTag(void* block)` (0x252660): the block's tag string, or
/// 0 when the block is not tagged.
pub fn mem_get_block_tag(c: &mut Cpu) {
    let p = arg(c, 0);
    c.cycles += 4;
    c.g[V0][0] = Addr(p.wrapping_sub(BLOCK_HEADER_SIZE)).lhu(c);
    c.g[V0][0] &= BLOCK_TAGGED as u64;
    let tagged = c.g[V0][0] != 0;
    c.g[V1][0] = 0;
    if tagged {
        c.cycles += 4;
        c.g[V0][0] = Addr(p.wrapping_sub(BLOCK_TAG_POINTER)).lw(c);
        c.g[V0][0] = sx(p.wrapping_add(c.g[V0][0] as u32));
        let rec = c.g[V0][0] as u32;
        c.lwl(V1, rec.wrapping_add(7));
        c.lwr(V1, rec.wrapping_add(4));
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[V1][0];
    abi::ret(c);
}

/// `void* MEM_AllocNamed128(const char* name, size_t size, uint32_t flags)`
/// (0x252fa0): named allocation with 128-byte alignment
/// (`MEM_AllocNamedAligned(name, size, 0x80, 0, flags, 1)`).
pub fn mem_alloc_named128(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[T0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = 0x80;
        c.g[A3][0] = 0;
        c.g[T1][0] = 1;
        callee::MEM_ALLOC_NAMED_ALIGNED.call(c, 0x0025_2fbc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Lobby.

/// `void LobbyMan_AcceptChallenge(self, a1, challenge)` (0x25fb58): store the
/// challenge id and send the lock message when the lobby is connected.
pub fn lobby_man_accept_challenge(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let this = arg(c, 0);
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(this).lw(c);
        let offline = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        if !offline {
            c.cycles += 3;
            Addr(this.wrapping_add(0xa4)).sw(c, c.g[A2][0]);
            c.g[A2][0] = 0x17;
            callee::LOBBY_MAN_SEND_LOCK_CHAL.call(c, 0x0025_fb74)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Singletons: destroy.

/// `void cSetupMan_Destroy(void)` (0x264098): destruct the setup manager (flag
/// 3) and forget it.
pub fn c_setup_man_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(gp(c, 0x2dc)).lw(c);
        let none = c.g[A0][0] == 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 3;
            c.g[A1][0] = 3;
            callee::C_SETUP_MAN_DTOR.call(c, 0x0026_40b0)?;
            Addr(gp(c, 0x2dc)).sw(c, 0);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cSharedLoginMan_Destroy(void)` (0x2658b8): destruct the shared login
/// manager (flag 3) and forget it.
pub fn c_shared_login_man_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(gp(c, 0x2e0)).lw(c);
        let none = c.g[A0][0] == 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 3;
            c.g[A1][0] = 3;
            callee::C_SHARED_LOGIN_MAN_DTOR.call(c, 0x0026_58d0)?;
            Addr(gp(c, 0x2e0)).sw(c, 0);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cVoipMan_Destroy(void)` (0x266cd8): destruct the VoIP manager and
/// forget it.
pub fn c_voip_man_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(gp(c, 0x304)).lw(c);
        let none = c.g[A0][0] == 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 3;
            c.g[A1][0] = 3;
            callee::C_VOIP_MAN_DTOR.call(c, 0x0026_6cf0)?;
            Addr(gp(c, 0x304)).sw(c, 0);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Shared login manager: memory-card requests (one wrapper per request code).

/// The memory-card request wrappers: call the request on the manager's card,
/// then record the request kind in the login-manager status word.
fn login_man_mc_request(c: &mut Cpu, target: u32, ret: u32, status: u32) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(gp(c, 0x2e8)).lw(c);
        frame.save_ra(c, 0);
        call_ptr(c, target, ret)?;
        c.g[V0][0] = status as u64;
        frame.restore_ra(c, 0);
        Addr(gp(c, 0x2e4)).sw(c, c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

/// `void cSharedLoginMan_McClose(void)` (0x266148): request a card close.
pub fn c_shared_login_man_mc_close(c: &mut Cpu) {
    login_man_mc_request(c, callee::MC_REQUEST_CLOSE, 0x0026_615c, 8)
}

/// `void cSharedLoginMan_McRead(void)` (0x266298): request a card read.
pub fn c_shared_login_man_mc_read(c: &mut Cpu) {
    login_man_mc_request(c, callee::MC_REQUEST_READ, 0x0026_62ac, 6)
}

/// `void cSharedLoginMan_McWrite(void)` (0x2662c0): request a card write.
pub fn c_shared_login_man_mc_write(c: &mut Cpu) {
    login_man_mc_request(c, callee::MC_REQUEST_WRITE, 0x0026_62d4, 0xe)
}

/// `void cSharedLoginMan_McFlush(void)` (0x266320): request a card flush.
pub fn c_shared_login_man_mc_flush(c: &mut Cpu) {
    login_man_mc_request(c, callee::MC_REQUEST_FLUSH, 0x0026_6334, 7)
}

/// `void cSharedLoginMan_McGetFoundName(char* dst)` (0x266550): copy the
/// found save name out and mark the status 0x10.
pub fn c_shared_login_man_mc_get_found_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x530000;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(0x0053_7190);
        call_ptr(c, callee::STRCPY, 0x0026_6564)?;
        c.g[V1][0] = 0x10;
        frame.restore_ra(c, 0);
        Addr(gp(c, 0x2e4)).sw(c, c.g[V1][0]);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script engine lookups.

/// `int ScriptEngine_ListGetPlayStatus(ScriptEngine* eng, int list)`
/// (0x279220): the play status of list `list` (`&eng->lists[list]`).
pub fn script_engine_list_get_play_status(c: &mut Cpu) {
    script_list_lookup(c, callee::SCRIPT_LIST_GET_PLAY_STATUS, 0x0027_923c)
}

/// `int ScriptEngine_ListCount(ScriptEngine* eng, int list)` (0x279248): the
/// number of pending commands in list `list`.
pub fn script_engine_list_count(c: &mut Cpu) {
    script_list_lookup(c, callee::SCRIPT_LIST_COUNT, 0x0027_9264)
}

/// `callee(&eng->lists[list])` where `lists` is at `+0x54c` and each element
/// is 0xcc bytes.
fn script_list_lookup(c: &mut Cpu, target: Callee, ret: u32) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xcc;
        frame.save_ra(c, 0);
        c.mult(0, A1, A1, V0);
        let base = Addr(arg(c, 0).wrapping_add(0x54c)).lw(c);
        c.g[A0][0] = sx((base as u32).wrapping_add(c.g[A1][0] as u32));
        target.call(c, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int ScriptEngine_FindScriptIndex(eng, category, ...)` (0x2797a0): the
/// chosen script's id, or -1.
pub fn script_engine_find_script_index(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::SCRIPT_ENGINE_CHOOSE_SCRIPT.call(c, 0x0027_97b0)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        } else {
            c.cycles += 1;
            c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int ScriptEngine_GetWorldSection2B(void)` (0x27a688): the race manager's
/// world section 0x2b.
pub fn script_engine_get_world_section2b(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x2b;
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x78)).lw(c);
        call_ptr(c, callee::SECTION_MAN_GET_WORLD_SECTION_INDEX, 0x0027_a6a4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int ScriptViewObj_GetSlotCam(ScriptViewObj* self, int slot)` (0x281c40):
/// the camera object in view slot `slot` (`slot` indexes the 16-bit slot
/// table at `+0xdc`; negative entries mean "no camera").
pub fn script_view_obj_get_slot_cam(c: &mut Cpu) {
    c.cycles += 5;
    let obj = arg(c, 0);
    c.g[A1][0] = sx(arg(c, 1) << 2);
    c.g[A1][0] = sx(obj.wrapping_add(c.g[A1][0] as u32));
    c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0xdc)).lw(c);
    let negative = (c.g[V1][0] as i64) < 0;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    if negative {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
    } else {
        c.cycles += 3;
        c.g[V1][0] = sx(obj.wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xd0)).lw(c);
        abi::ret(c);
    }
}

/// `int* ScriptEngine_GetProcessInSeq(eng, seq)` (0x282bf0): the process
/// record of `seq`, or 0 when the slot is empty.
pub fn script_engine_get_process_in_seq(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::SCRIPT_ENGINE_GET_PROCESS.call(c, 0x0028_2c00)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 1;
            c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script bindings.

/// `void ScriptBinding_ClearListener(ScriptBinding* self)` (0x282f90):
/// detach the listener from the binding's channel.
pub fn script_binding_clear_listener(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(arg(c, 0).wrapping_add(0x8)).lw(c);
        let none = c.g[A0][0] == 0;
        frame.restore_ra(c, 0);
        if !none {
            c.cycles += 3;
            c.g[A1][0] = 0;
            callee::IN_SEQ_CHANNEL_SET_LISTENER.call(c, 0x0028_2fac)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptBinding_PauseSeq(ScriptBinding* self)` (0x283038): pause the
/// sequence instance behind the binding's channel.
pub fn script_binding_pause_seq(c: &mut Cpu) {
    instance_pause_resume(c, callee::IN_SEQ_INSTANCE_PAUSE, 0x0028_3054)
}

/// `void ScriptBinding_ResumeSeq(ScriptBinding* self)` (0x283060): resume it.
pub fn script_binding_resume_seq(c: &mut Cpu) {
    instance_pause_resume(c, callee::IN_SEQ_INSTANCE_RESUME, 0x0028_307c)
}

/// `if (self->channel) InstancePause/Resume(self->channel->instance)`.
fn instance_pause_resume(c: &mut Cpu, target: Callee, ret: u32) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(arg(c, 0).wrapping_add(0x8)).lw(c);
        let none = c.g[V0][0] == 0;
        frame.restore_ra(c, 0);
        if !none {
            c.cycles += 3;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lw(c);
            target.call(c, ret)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Movies.

/// `int ScriptMovie_GetId(ScriptMovie* self)` (0x283bb8): the id of the
/// loaded movie, or 0 when it has no data.
pub fn script_movie_get_id(c: &mut Cpu) {
    'outer: {
        c.cycles += 3;
        c.g[V1][0] = Addr(arg(c, 0)).lw(c);
        if c.g[V1][0] == 0 {
            break 'outer;
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(arg(c, 0).wrapping_add(0x4)).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
            abi::ret(c);
            return;
        }
        break 'outer;
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Sound bank manager.

/// `SndBankMan` methods index a 0x60-byte `SndBank` array at `self+0xacc`.
fn snd_bank_call(c: &mut Cpu, target: Callee, ret: u32) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = 0x60;
    frame.save_ra(c, 0);
    c.mult(0, A1, A1, V0);
    let base = Addr(arg(c, 0).wrapping_add(0xacc)).lw(c);
    c.g[A0][0] = sx((base as u32).wrapping_add(c.g[A1][0] as u32));
    target.call(c, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void SndBankMan_UnloadBank(self, int bank)` (0x28bce8).
pub fn snd_bank_man_unload_bank(c: &mut Cpu) {
    guest(c, |c| snd_bank_call(c, callee::SND_BANK_UNLOAD, 0x0028_bd04))
}

/// `void SndBankMan_FreeBankMemory(self, int bank)` (0x28bde0).
pub fn snd_bank_man_free_bank_memory(c: &mut Cpu) {
    guest(c, |c| snd_bank_call(c, callee::SND_BANK_FREE_MEMORY, 0x0028_bdfc))
}

/// `void SndBankMan_UnloadMemoryBank(self, int bank)` (0x28be90).
pub fn snd_bank_man_unload_memory_bank(c: &mut Cpu) {
    guest(c, |c| snd_bank_call(c, callee::SND_BANK_UNLOAD_MEMORY, 0x0028_beac))
}

/// `void SndBankMan_ReleaseBank(self, int bank)` (0x28beb8).
pub fn snd_bank_man_release_bank(c: &mut Cpu) {
    guest(c, |c| snd_bank_call(c, callee::SND_BANK_RELEASE, 0x0028_bed4))
}

/// `int SndBankMan_GetBankBufferSize(self, int bank)` (0x28bf10).
pub fn snd_bank_man_get_bank_buffer_size(c: &mut Cpu) {
    guest(c, |c| snd_bank_call(c, callee::SND_BANK_GET_BUFFER_SIZE, 0x0028_bf2c))
}

/// `bool SSXAudioSystem_IsGameMusicEnabled(SSXAudioSystem* self)`
/// (0x28d960): true when the game-music state at `+0x608c` is 0, 1 or 3.
pub fn ssx_audio_system_is_game_music_enabled(c: &mut Cpu) {
    c.cycles += 4;
    let state = Addr(arg(c, 0).wrapping_add(0x608c)).lw(c);
    c.g[V1][0] = state;
    let below_two = state < 2;
    c.g[V0][0] = below_two as u64;
    c.g[V0][0] = 3;
    if !below_two {
        c.cycles += 2;
        if state != 3 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `void BankMonitor_ResetInstance(self, int instance)` (0x2ad398): reset the
/// 0xc0-byte bank monitor instance at `self + instance*0xc0`.
pub fn bank_monitor_reset_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xc0;
        frame.save_ra(c, 0);
        c.mult(0, A1, A1, V0);
        let base = Addr(arg(c, 0).wrapping_add(0x8ec)).lw(c);
        c.g[A0][0] = sx((base as u32).wrapping_add(c.g[A1][0] as u32));
        callee::BANK_MONITOR_INSTANCE_RESET.call(c, 0x002a_d3b4)?;
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
    use crate::difftest::fakes::{covered, machine, ptr, vtable};
    use crate::difftest::{check, check_lohi, scramble, Rng, DATA, GAME_GP};

    fn rng(seed: u32, salt: u64) -> Rng {
        Rng(0x9e37_79b9_7f4a_7c15 ^ (seed as u64) << 8 ^ salt)
    }

    // ---------------------------------------------------------------- camera

    /// A camera algorithm object at a random pointer whose `v05` slot calls
    /// the fake, or unwinds at it.
    fn algo_reset_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 1);
        let obj = ptr(&mut r);
        c.g[A0][0] = sx(obj);
        let vt = vtable(c, &mut r, &[0x28]);
        c.bus.write32(obj.wrapping_add(0x10), vt);
    }

    #[test]
    fn manual_script_camera_algo_reset_for_rider_matches() {
        covered(ssx3_ee::s02::f_00169518__generated, manual_script_camera_algo_reset_for_rider, 120, &|c, seed| {
            algo_reset_setup(c, seed, 0x11)
        });
    }

    #[test]
    fn target_script_camera_algo_reset_for_rider_matches() {
        covered(ssx3_ee::s02::f_00169d58__generated, target_script_camera_algo_reset_for_rider, 120, &|c, seed| {
            algo_reset_setup(c, seed, 0x12)
        });
    }

    #[test]
    fn subject_script_camera_algo_reset_for_rider_matches() {
        covered(ssx3_ee::s02::f_0016a4b8__generated, subject_script_camera_algo_reset_for_rider, 120, &|c, seed| {
            algo_reset_setup(c, seed, 0x13)
        });
    }

    #[test]
    fn camera_trigger_stack_ctor_matches() {
        check(ssx3_ee::s02::f_0016d210__generated, camera_trigger_stack_ctor, 200, &|c, seed| {
            let mut r = rng(seed, 0x14);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA + 0x2000);
            for i in 0..4u32 {
                c.bus.write32(DATA + 0x2000 + 4 * i, r.next_u32());
            }
        });
    }

    /// The two camera-trigger array element destructors: the member pointer is
    /// either null or a block from the test heap (which `operator_delete`
    /// legitimately releases).
    fn trigger_array_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let _ = machine(c, seed, salt, 0);
        let obj = DATA + 0x3000;
        c.g[A0][0] = sx(obj);
        let member = if seed % 2 == 0 { 0 } else { heap_block(c, 0x40) };
        c.bus.write32(obj + 8, member);
    }

    #[test]
    fn camera_trigger_array_45c1d0_v01_matches() {
        check(ssx3_ee::s02::f_00176840__generated, camera_trigger_array_45c1d0_v01, 80, &|c, seed| {
            trigger_array_setup(c, seed, 0x15)
        });
    }

    #[test]
    fn camera_trigger_array_45c1b8_v01_matches() {
        check(ssx3_ee::s02::f_00176868__generated, camera_trigger_array_45c1b8_v01, 80, &|c, seed| {
            trigger_array_setup(c, seed, 0x16)
        });
    }

    // ------------------------------------------------------------- front end

    fn notify_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        c.g[A0][0] = sx(DATA + 0x4000);
        c.g[A1][0] = r.next_u32() as u64;
        c.g[A2][0] = match seed % 4 {
            0 => 6,
            1 => 9,
            2 => 8,
            _ => r.next_u32() as u64,
        };
    }

    #[test]
    fn c_fe_state_event_select_on_widget_notify_matches() {
        check(ssx3_ee::s02::f_00186950__generated, c_fe_state_event_select_on_widget_notify, 150, &|c, seed| {
            notify_setup(c, seed, 0x17)
        });
    }

    #[test]
    fn ov_screen_map_on_widget_notify_matches() {
        check(ssx3_ee::s05::f_00208610__generated, ov_screen_map_on_widget_notify, 150, &|c, seed| {
            notify_setup(c, seed, 0x18)
        });
    }

    #[test]
    fn bolt_file_index_manager_ctor_matches() {
        check(ssx3_ee::s03::f_0019cb38__generated, bolt_file_index_manager_ctor, 40, &|c, seed| {
            let mut r = rng(seed, 0x19);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA + 0x2000);
            for i in 0..4u32 {
                c.bus.write32(DATA + 0x2000 + 4 * i, r.next_u32());
            }
        });
    }

    #[test]
    fn fe_module_set_preview_visible_matches() {
        check_lohi(ssx3_ee::s03::f_001a0570__generated, fe_module_set_preview_visible, 200, &|c, seed| {
            let mut r = rng(seed, 0x1a);
            scramble(c, &mut r);
            let index = seed % 4;
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = index as u64;
            c.g[A2][0] = (seed % 3 != 0) as u64;
            let base = DATA + index * 0xce0;
            // Cover the early-out branches of the preview update.
            for off in [0xcb4u32, 0xcb8, 0xcc0, 0xcc4, 0xcc8] {
                c.bus.write32(base + off, if r.next_u32() % 2 == 0 { 0 } else { r.next_u32() });
            }
            c.bus.write32(base + 0xcd4, r.next_u32() | 0x8000_0000);
        });
    }

    // ------------------------------------------------------------ game state

    #[test]
    fn game_module_advance_state_matches() {
        check(ssx3_ee::s06::f_00231320__generated, game_module_advance_state, 150, &|c, seed| {
            let mut r = rng(seed, 0x1b);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP - 0x488, DATA + 0x3000);
            c.g[A0][0] = sx(DATA + 0x4000);
            let pending = DATA + 0x5000;
            c.bus.write32(DATA + 0x4000 + 0x200, if seed % 3 == 0 { 0 } else { pending });
            c.bus.write32(pending, if seed % 6 == 0 { 0 } else { r.next_u32() });
            c.bus.write32(pending + 4, r.next_u32());
        });
    }

    #[test]
    fn game_state_quit_enter_matches() {
        check(ssx3_ee::s06::f_00233b60__generated, game_state_quit_enter, 100, &|c, seed| {
            let mut r = rng(seed, 0x1c);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP - 0x488, DATA + 0x3000);
        });
    }

    #[test]
    fn game_state_restart_race_exit_matches() {
        check(ssx3_ee::s06::f_00235f20__generated, game_state_restart_race_exit, 200, &|c, seed| {
            let mut r = rng(seed, 0x1d);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let world = DATA + 0x8000;
            let race = DATA + 0x9000;
            let rider = DATA + 0xa000;
            c.bus.write32(GAME_GP - 0x848, world);
            c.bus.write32(world + 0x84, race);
            c.bus.write32(race + 0x94, rider);
            c.bus.write32(rider + 4, r.next_u32());
            c.bus.write32(rider + 8, r.next_u32());
        });
    }

    /// The replay-playback world: a world whose race manager owns an empty
    /// playback record and an empty world-script manager. The BE library
    /// singleton is a zeroed dummy and its interface slot points at a leaf
    /// function, so `WScriptMan_SpawnMissions` takes its early return.
    fn replay_exit_setup(c: &mut Cpu, seed: u32) {
        let mut r = rng(seed, 0x1e);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        let world = DATA + 0x8000;
        let race = DATA + 0x9000;
        let playback = DATA + 0xb000;
        let wscript = DATA + 0xa000;
        c.bus.write32(GAME_GP - 0x848, world);
        c.bus.write32(world + 0x84, race);
        c.bus.write32(race + 0x34, playback);
        // PlaybackMan_FreeData: a -1 block handle skips the release, and a
        // zeroed record takes the null-list path down to the null frees.
        c.bus.write32(playback + 0x78, u32::MAX);
        c.bus.write32(playback + 0x4c, 0);
        c.bus.write32(playback + 0x50, 0);
        c.bus.write32(playback + 0x58, 0);
        // WScriptMan_DeleteAllProcesses: empty process lists.
        c.bus.write32(GAME_GP + 0xce8, wscript);
        // BELibrary_GetInstance returns the existing singleton.
        c.bus.write32(GAME_GP - 0x1f04, DATA + 0xc000);
        // BELibrary_GetInterface: index 0 -> lib -> vtable slot 8 -> the
        // camera-trigger-stack clear leaf, which only writes its argument.
        const BE_LIBRARY: u32 = 0x0053_08a8;
        let lib = DATA + 0xd000;
        let vt = DATA + 0xe000;
        c.bus.write32(GAME_GP - 0x1efc, 0);
        c.bus.write32(BE_LIBRARY, lib);
        c.bus.write32(lib + 4, vt);
        c.bus.write16(vt + 8, 0);
        c.bus.write32(vt + 0xc, 0x0016_d200);
        // WScriptMan_SpawnMissions leaves early unless the world type is 4.
        c.bus.write8(0x0053_5c10, (seed % 3) as u8);
    }

    #[test]
    fn game_state_replay_playback_exit_matches() {
        check(ssx3_ee::s06::f_00237038__generated, game_state_replay_playback_exit, 60, &|c, seed| {
            replay_exit_setup(c, seed)
        });
    }

    // ---------------------------------------------------------- load states

    /// A zeroed load-state object; `a1`'s low bit selects whether UIState_dtor
    /// frees it, so keep it clear (the object is not a heap block).
    fn load_state_dtor_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = rng(seed, salt);
        scramble(c, &mut r);
        c.g[A0][0] = sx(DATA + 0x4000);
        c.g[A1][0] = ((r.next_u32() % 4) * 2) as u64;
        for off in [0x24u32, 0x2c, 0x30, 0x38, 0x3c] {
            c.bus.write32(DATA + 0x4000 + off, 0);
        }
    }

    #[test]
    fn c_game_load_state_dtor_matches() {
        check(ssx3_ee::s06::f_002453f0__generated, c_game_load_state_dtor, 80, &|c, seed| {
            load_state_dtor_setup(c, seed, 0x1f)
        });
    }

    #[test]
    fn c_game_load_state_out_lodge_dtor_matches() {
        check(ssx3_ee::s06::f_00245600__generated, c_game_load_state_out_lodge_dtor, 80, &|c, seed| {
            load_state_dtor_setup(c, seed, 0x20)
        });
    }

    #[test]
    fn c_game_load_state_conquer_dtor_matches() {
        check(ssx3_ee::s06::f_00245708__generated, c_game_load_state_conquer_dtor, 80, &|c, seed| {
            load_state_dtor_setup(c, seed, 0x21)
        });
    }

    #[test]
    fn c_fe_load_hint_state_dtor_matches() {
        check(ssx3_ee::s06::f_00245928__generated, c_fe_load_hint_state_dtor, 80, &|c, seed| {
            load_state_dtor_setup(c, seed, 0x22)
        });
    }

    #[test]
    fn c_fe_load_state_dtor_matches() {
        check(ssx3_ee::s06::f_00245b58__generated, c_fe_load_state_dtor, 80, &|c, seed| {
            load_state_dtor_setup(c, seed, 0x23)
        });
    }

    #[test]
    fn c_fe_load_state_in_lodge_dtor_matches() {
        check(ssx3_ee::s06::f_00245d28__generated, c_fe_load_state_in_lodge_dtor, 80, &|c, seed| {
            load_state_dtor_setup(c, seed, 0x24)
        });
    }

    // -------------------------------------------------------------- memory

    #[test]
    fn mem_get_block_tag_matches() {
        check(ssx3_ee::s06::f_00252660__generated, mem_get_block_tag, 250, &|c, seed| {
            let mut r = rng(seed, 0x25);
            scramble(c, &mut r);
            let block = DATA + 0x8000;
            let tagged = seed % 2 == 0;
            let size_word = if tagged {
                0x800u16 | (r.next_u32() as u16 & 0x7fff)
            } else {
                r.next_u32() as u16 & 0x7ff
            };
            c.bus.write16(block - 0xe, size_word);
            // The word at -0xc is the signed offset from the block to the tag
            // record; the value is the unaligned word at record + 4.
            let tag = DATA + 0x9000;
            let extra = match seed % 4 {
                0 => 0u32,
                1 => 0x10,
                2 => 0x1000,
                _ => (-0x20i32) as u32,
            };
            c.bus.write32(block - 0xc, (tag as u32).wrapping_sub(block as u32).wrapping_add(extra));
            for i in 0..8u32 {
                c.bus.write32(tag + 4 * i, r.next_u32());
            }
            c.g[A0][0] = sx(block);
        });
    }

    #[test]
    fn mem_alloc_named128_matches() {
        check(ssx3_ee::s06::f_00252fa0__generated, mem_alloc_named128, 60, &|c, seed| {
            let mut r = rng(seed, 0x26);
            scramble(c, &mut r);
            heap_init(c);
            let name = DATA + 0x9000;
            let len = 1 + (seed % 8) as u32;
            for i in 0..len {
                c.bus.write8(name + i, b'A' + (r.next_u32() % 26) as u8);
            }
            c.bus.write8(name + len, 0);
            c.g[A0][0] = sx(name);
            c.g[A1][0] = (8 + (seed as u64 % 0x400)) as u64;
            c.g[A2][0] = (r.next_u32() & 0x0f00_0000) as u64;
        });
    }

    // ---------------------------------------------------------------- lobby

    #[test]
    fn lobby_man_accept_challenge_matches() {
        check(ssx3_ee::s06::f_0025fb58__generated, lobby_man_accept_challenge, 60, &|c, seed| {
            let mut r = rng(seed, 0x27);
            scramble(c, &mut r);
            heap_init(c);
            let lobby = DATA + 0x4000;
            let packet = DATA + 0x6000;
            // The lobby is "connected" only for some seeds.
            c.bus.write32(lobby, (seed % 2) as u32);
            // No lobby API: the private-message send returns -1 at once.
            c.bus.write32(lobby + 0x54, 0);
            c.bus.write32(lobby + 0xf0, packet);
            c.bus.write32(packet, 0);
            c.bus.write32(packet + 4, DATA + 0x7000);
            c.g[A0][0] = sx(lobby);
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
        });
    }

    // ------------------------------------------------------------ singletons

    #[test]
    fn c_setup_man_destroy_matches() {
        check(ssx3_ee::s07::f_00264098__generated, c_setup_man_destroy, 60, &|c, seed| {
            let mut r = rng(seed, 0x28);
            scramble(c, &mut r);
            heap_init(c);
            let manager = if seed % 3 == 0 { 0 } else { heap_block(c, 0x120) };
            c.bus.write32(GAME_GP + 0x2dc, manager);
        });
    }

    #[test]
    fn c_shared_login_man_destroy_matches() {
        check(ssx3_ee::s07::f_002658b8__generated, c_shared_login_man_destroy, 60, &|c, seed| {
            let mut r = rng(seed, 0x29);
            scramble(c, &mut r);
            heap_init(c);
            // No memory card and no SLogin store: shutdown takes its "not
            // initialised" early outs. The store-error mapper reads a game
            // jump table, so give it the entry for "not initialised" (2).
            const SLOGIN_STORE_ERRORS: u32 = 0x0049_7150;
            c.bus.write32(SLOGIN_STORE_ERRORS + 8, 0x003f_81dc);
            c.bus.write32(GAME_GP + 0x2e8, 0);
            c.bus.write32(GAME_GP + 0x1ad4, 0);
            c.bus.write32(GAME_GP + 0x2a30, 0);
            let manager = if seed % 3 == 0 { 0 } else { heap_block(c, 0x120) };
            c.bus.write32(GAME_GP + 0x2e0, manager);
        });
    }

    /// The memory-card request wrappers: `gp+0x2e8` is the card. The state
    /// machine dispatches through the table at 0x4860c0; the request states
    /// (0x43, 0x45, 0x47, 0x49) all take its common "request" arm, whose only
    /// work is `DoGetInfo` when `+0x40` is clear — keep it set.
    fn mc_request_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        const STATE_TABLE: u32 = 0x0048_60c0;
        const REQUEST_ARM: u32 = 0x002c_50ac;
        for state in [0x43u32, 0x45, 0x47, 0x49] {
            c.bus.write32(STATE_TABLE + (state - 1) * 4, REQUEST_ARM);
        }
        let card = DATA + 0x4000;
        c.bus.write32(card + 0x40, 1);
        c.bus.write32(card + 0x4, 0);
        c.bus.write32(card + 0x18, if seed % 2 == 0 { u32::MAX } else { r.next_u32() });
        c.bus.write32(GAME_GP + 0x2e8, card);
    }

    #[test]
    fn c_shared_login_man_mc_close_matches() {
        check(ssx3_ee::s07::f_00266148__generated, c_shared_login_man_mc_close, 60, &|c, seed| {
            mc_request_setup(c, seed, 0x2a)
        });
    }

    #[test]
    fn c_shared_login_man_mc_read_matches() {
        check(ssx3_ee::s07::f_00266298__generated, c_shared_login_man_mc_read, 60, &|c, seed| {
            mc_request_setup(c, seed, 0x2b)
        });
    }

    #[test]
    fn c_shared_login_man_mc_write_matches() {
        check(ssx3_ee::s07::f_002662c0__generated, c_shared_login_man_mc_write, 60, &|c, seed| {
            mc_request_setup(c, seed, 0x2c)
        });
    }

    #[test]
    fn c_shared_login_man_mc_flush_matches() {
        check(ssx3_ee::s07::f_00266320__generated, c_shared_login_man_mc_flush, 60, &|c, seed| {
            mc_request_setup(c, seed, 0x2d)
        });
    }

    #[test]
    fn c_shared_login_man_mc_get_found_name_matches() {
        check(ssx3_ee::s07::f_00266550__generated, c_shared_login_man_mc_get_found_name, 150, &|c, seed| {
            let mut r = rng(seed, 0x2e);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let src = 0x0053_7190;
            let len = (seed % 12) as u32;
            for i in 0..len {
                c.bus.write8(src + i, b'a' + (r.next_u32() % 26) as u8);
            }
            c.bus.write8(src + len, 0);
            c.g[A0][0] = sx(DATA + 0x5000);
        });
    }

    #[test]
    fn c_voip_man_destroy_matches() {
        check(ssx3_ee::s07::f_00266cd8__generated, c_voip_man_destroy, 60, &|c, seed| {
            let mut r = rng(seed, 0x2f);
            scramble(c, &mut r);
            heap_init(c);
            let manager = if seed % 3 == 0 { 0 } else { heap_block(c, 0x80) };
            if manager != 0 {
                c.bus.write32(manager, 0);
            }
            c.bus.write32(GAME_GP + 0x304, manager);
        });
    }

    // --------------------------------------------------------------- script

    fn script_list_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        let engine = DATA + 0x4000;
        let list = DATA + 0x5000;
        c.bus.write32(engine + 0x54c, list);
        let index = seed % 4;
        let slot = list + index * 0xcc;
        // ScriptList_GetPlayStatus: state 3 with a non-zero "loaded" word
        // returns 3, anything else returns 0.
        c.bus.write32(slot + 0xa4, if seed % 2 == 0 { 0 } else { 3 });
        c.bus.write32(slot + 0xc4, 1);
        for i in 0..4u32 {
            c.bus.write32(slot + 0x8c + 4 * i, r.next_u32());
        }
        c.g[A0][0] = sx(engine);
        c.g[A1][0] = index as u64;
    }

    #[test]
    fn script_engine_list_get_play_status_matches() {
        check(ssx3_ee::s07::f_00279220__generated, script_engine_list_get_play_status, 100, &|c, seed| {
            script_list_setup(c, seed, 0x30)
        });
    }

    #[test]
    fn script_engine_list_count_matches() {
        check(ssx3_ee::s07::f_00279248__generated, script_engine_list_count, 100, &|c, seed| {
            script_list_setup(c, seed, 0x31)
        });
    }

    #[test]
    fn script_engine_find_script_index_matches() {
        check(ssx3_ee::s07::f_002797a0__generated, script_engine_find_script_index, 100, &|c, seed| {
            let mut r = rng(seed, 0x32);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA + 0x4000);
            // `ChooseScript` returns 0 for a category with no scripts, both
            // when the category table is empty and when the chosen entry is
            // null.
            let engine = DATA + 0x4000;
            c.bus.write32(engine + 0x53c, (seed % 2) as u32);
            c.bus.write32(engine + 0x540, DATA + 0x5000);
            c.g[A1][0] = (seed % 4) as u64;
        });
    }

    #[test]
    fn script_engine_get_world_section2b_matches() {
        check(ssx3_ee::s07::f_0027a688__generated, script_engine_get_world_section2b, 200, &|c, seed| {
            let mut r = rng(seed, 0x33);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let world = DATA + 0x8000;
            c.bus.write32(GAME_GP - 0x848, world);
            c.bus.write32(world + 0x84, DATA + 0x9000);
            c.bus.write32(DATA + 0x9000 + 0x78, DATA + 0xa000);
        });
    }

    #[test]
    fn script_view_obj_get_slot_cam_matches() {
        check(ssx3_ee::s07::f_00281c40__generated, script_view_obj_get_slot_cam, 300, &|c, seed| {
            let mut r = rng(seed, 0x34);
            scramble(c, &mut r);
            let obj = DATA + 0x4000;
            let slot = (seed % 8) as u32;
            let slot_value = if seed % 5 == 0 {
                u32::MAX - (seed % 8)
            } else {
                seed % 32
            };
            c.bus.write32(obj + 0xdc + slot * 4, slot_value);
            for i in 0..32u32 {
                c.bus.write32(obj + 0xd0 + 4 * i, r.next_u32());
            }
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = slot as u64;
        });
    }

    #[test]
    fn script_engine_get_process_in_seq_matches() {
        check(ssx3_ee::s07::f_00282bf0__generated, script_engine_get_process_in_seq, 200, &|c, seed| {
            let mut r = rng(seed, 0x35);
            scramble(c, &mut r);
            let engine = DATA + 0x4000;
            let index = seed % 6;
            let proc = engine + 0x2c0 + index * 0x24;
            c.bus.write32(proc + 0x20, (seed % 2) as u32);
            c.bus.write32(proc, r.next_u32());
            c.g[A0][0] = sx(engine);
            c.g[A1][0] = index as u64;
        });
    }

    #[test]
    fn script_binding_clear_listener_matches() {
        check(ssx3_ee::s07::f_00282f90__generated, script_binding_clear_listener, 150, &|c, seed| {
            let mut r = rng(seed, 0x36);
            scramble(c, &mut r);
            let binding = DATA + 0x4000;
            let channel = DATA + 0x5000;
            c.bus.write32(binding + 8, if seed % 2 == 0 { 0 } else { channel });
            // No listener: the channel just records the null listener.
            c.bus.write32(channel + 0x2c, 0);
            c.bus.write32(channel + 0x1c, DATA + 0x6000);
            c.bus.write8(DATA + 0x6003, r.next_u32() as u8);
            c.g[A0][0] = sx(binding);
        });
    }

    fn binding_pause_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        let binding = DATA + 0x4000;
        let channel = DATA + 0x5000;
        let inst = DATA + 0x6000;
        c.bus.write32(binding + 8, if seed % 2 == 0 { 0 } else { channel });
        c.bus.write32(channel + 8, inst);
        c.bus.write32(inst + 0x20, (seed % 3 != 0) as u32);
        c.bus.write32(inst + 0x2c, match seed % 4 {
            0 => 0,
            1 => 1,
            _ => r.next_u32(),
        });
        c.g[A0][0] = sx(binding);
    }

    #[test]
    fn script_binding_pause_seq_matches() {
        check(ssx3_ee::s07::f_00283038__generated, script_binding_pause_seq, 200, &|c, seed| {
            binding_pause_setup(c, seed, 0x37)
        });
    }

    #[test]
    fn script_binding_resume_seq_matches() {
        check(ssx3_ee::s07::f_00283060__generated, script_binding_resume_seq, 200, &|c, seed| {
            binding_pause_setup(c, seed, 0x38)
        });
    }

    #[test]
    fn script_movie_get_id_matches() {
        check(ssx3_ee::s07::f_00283bb8__generated, script_movie_get_id, 300, &|c, seed| {
            let mut r = rng(seed, 0x39);
            scramble(c, &mut r);
            let movie = DATA + 0x4000;
            let data = DATA + 0x5000;
            c.bus.write32(movie, if seed % 3 == 0 { 0 } else { data });
            c.bus.write32(movie + 4, (seed % 5 != 0) as u32);
            c.bus.write32(data, r.next_u32());
            c.g[A0][0] = sx(movie);
        });
    }

    #[test]
    fn script_cmd_queue_ctor_matches() {
        check(ssx3_ee::s07::f_00283c30__generated, script_cmd_queue_ctor, 150, &|c, seed| {
            let mut r = rng(seed, 0x3a);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA + 0x4000);
            for i in 0..4u32 {
                c.bus.write32(DATA + 0x4000 + 4 * i, r.next_u32());
            }
        });
    }

    // ---------------------------------------------------------------- sound

    /// A sound bank manager with four zeroed 0x60-byte banks. The banks are
    /// arranged so `SndBank_Unload`/`_UnloadMemory`/`_Release` take their
    /// short paths and the frees see a null payload.
    fn snd_bank_man_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        let manager = DATA + 0x4000;
        let banks = DATA + 0x5000;
        c.bus.write32(manager + 0xacc, banks);
        for i in 0..4u32 {
            let bank = banks + i * 0x60;
            c.bus.write32(bank, (seed % 2) * 2);
            c.bus.write32(bank + 0x4, u32::MAX);
            c.bus.write32(bank + 0x8, (seed % 2) * 3);
            c.bus.write32(bank + 0x10, 0);
            c.bus.write32(bank + 0x14, r.next_u32());
            c.bus.write32(bank + 0x18, 0);
        }
        c.g[A0][0] = sx(manager);
        c.g[A1][0] = (seed % 4) as u64;
    }

    #[test]
    fn snd_bank_man_unload_bank_matches() {
        check_lohi(ssx3_ee::s07::f_0028bce8__generated, snd_bank_man_unload_bank, 100, &|c, seed| {
            snd_bank_man_setup(c, seed, 0x3b)
        });
    }

    #[test]
    fn snd_bank_man_free_bank_memory_matches() {
        check_lohi(ssx3_ee::s07::f_0028bde0__generated, snd_bank_man_free_bank_memory, 100, &|c, seed| {
            snd_bank_man_setup(c, seed, 0x3c)
        });
    }

    #[test]
    fn snd_bank_man_unload_memory_bank_matches() {
        check_lohi(ssx3_ee::s07::f_0028be90__generated, snd_bank_man_unload_memory_bank, 100, &|c, seed| {
            snd_bank_man_setup(c, seed, 0x3d)
        });
    }

    #[test]
    fn snd_bank_man_release_bank_matches() {
        check_lohi(ssx3_ee::s07::f_0028beb8__generated, snd_bank_man_release_bank, 100, &|c, seed| {
            snd_bank_man_setup(c, seed, 0x3e)
        });
    }

    #[test]
    fn snd_bank_man_get_bank_buffer_size_matches() {
        check_lohi(ssx3_ee::s07::f_0028bf10__generated, snd_bank_man_get_bank_buffer_size, 100, &|c, seed| {
            snd_bank_man_setup(c, seed, 0x3f)
        });
    }

    #[test]
    fn ssx_audio_system_is_game_music_enabled_matches() {
        check(ssx3_ee::s07::f_0028d960__generated, ssx_audio_system_is_game_music_enabled, 300, &|c, seed| {
            let mut r = rng(seed, 0x40);
            scramble(c, &mut r);
            let state = match seed % 6 {
                0 => 0,
                1 => 1,
                2 => 3,
                3 => 2,
                4 => r.next_u32(),
                _ => 4,
            };
            c.bus.write32(DATA + 0x608c, state);
            c.g[A0][0] = sx(DATA);
        });
    }

    #[test]
    fn bank_monitor_reset_instance_matches() {
        check_lohi(ssx3_ee::s08::f_002ad398__generated, bank_monitor_reset_instance, 100, &|c, seed| {
            machine(c, seed, 0x41, 0);
            let monitor = DATA + 0x4000;
            let instances = DATA + 0x5000;
            c.bus.write32(monitor + 0x8ec, instances);
            for i in 0..4u32 {
                // FadeOut returns at once when the instance is fading.
                c.bus.write32(instances + i * 0xc0 + 0x54, 1);
            }
            c.g[A0][0] = sx(monitor);
            c.g[A1][0] = (seed % 4) as u64;
        });
    }

    // ------------------------------------------------------------- fixtures

    /// The main heap's dlmalloc arena over 256 KiB of test RAM, as
    /// `containers.rs` sets up for its allocator tests.
    const MEM_HEAP: u32 = 0x004f_f250;

    fn run(c: &mut Cpu, addr: u32, f: fn(&mut Cpu), args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, crate::difftest::RA, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    fn heap_init(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        run(
            c,
            0x0031_ff60,
            ssx3_ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x40000) as u64, 0x40000],
        );
    }

    fn heap_block(c: &mut Cpu, size: u32) -> u32 {
        heap_init(c);
        run(
            c,
            0x0032_0058,
            ssx3_ee::s10::MEM_HeapMemalign,
            &[MEM_HEAP as u64, 16, size as u64],
        ) as u32
    }
}
