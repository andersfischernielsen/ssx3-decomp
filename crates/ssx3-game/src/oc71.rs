//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for fifty small front-end, replay, script, sound, render and
//! memory-manager functions, each with a differential test against the
//! generated body.
//!
//! - **Front-end state machine** (`cFEState*` constructors, OnActivate /
//!   OnChildEvent / Update handlers): a base constructor writes the class
//!   vtable and state id, the update handlers poll widgets and image loads,
//!   the child-event handlers dispatch on the event id at `a1+0x18`.
//! - **Replay** (`Replay_SeekNextSnapshot`, `Replay_SeekPrevSnapshot`,
//!   `Replay_NextCamera`, `Replay_AdvanceFromState10`) and the world/replay
//!   stream constructors.
//! - **Script engine** (`ScriptList_GetState`,
//!   `ScriptList_IsCurrentSkippable`, `ScriptEngine_ClearLocatorTable`,
//!   `ScriptEngine_ListAddMovie`, `tWScriptCache_SetData`).
//! - **Sound** (`SSXAudioSystem_GetPresetGain`,
//!   `CrowdAudio_InstanceStopAnticipate`, `MusicPlayer_GetDuckToLoops`,
//!   `WorldTrigger_IsType5`, `cWorldSound_ctor`).
//! - **Memory / pads / graphics / UI** (`MEM_GetBlockSize`,
//!   `MEM_ReportStatsLocked`, `PadQueue_Init`, `mEmitter_ctor`,
//!   `PS2GraphicsMan_FreeBlendedMatrixHistory`,
//!   `cUITextScroll_ReplaceWideString`, `cStreamMan_Init`).
//!
//! The replacements keep their state in the guest registers the originals
//! use (callers, callees and event handlers see the same values), follow the
//! call protocol of [`crate::call`] (the same stack frame, the same registers
//! at each call, the unwind check after it) and the event rules of
//! [`crate::events`] at loop back edges.

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

#[rustfmt::skip]
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0019_5498, fe_state_bonus_material_ctor, "cFEStateBonusMaterial_ctor"),
    (0x0019_7b88, fe_state_request_line_refresh_list, "cFEStateRequestLine_RefreshList"),
    (0x001a_06b0, fe_flow_state_free, "FEFlowState_Free"),
    (0x001a_3218, fe_state_mp_circuit_ctor, "cFEStateMPCircuit_ctor"),
    (0x001b_3528, fe_state_chat_room_on_activate, "cFEStateChatRoom_OnActivate"),
    (0x001c_1b48, fe_state_net_quick_play_popup_on_child_event, "cFEStateNetQuickPlayPopup_OnChildEvent"),
    (0x001c_4b98, fe_state_stats_on_widget_notify, "cFEStateStats_OnWidgetNotify"),
    (0x001c_9b28, fe_popup_confirm_set_text_measured, "cFEPopupConfirm_SetTextMeasured"),
    (0x001d_1fb0, fe_state_preview_reward_update, "cFEStatePreviewReward_Update"),
    (0x001d_2a90, fe_state_mountain_room_on_activate, "cFEStateMountainRoom_OnActivate"),
    (0x001d_4760, fe_state_trophy_room_update, "cFEStateTrophyRoom_Update"),
    (0x001d_5038, fe_state_relogin_ctor, "cFEStateRelogin_ctor"),
    (0x001d_5240, fe_state_relogin_on_child_event, "cFEStateRelogin_OnChildEvent"),
    (0x001f_80d8, ov_screen_top_times_on_widget_action, "OVScreenTopTimes_OnWidgetAction"),
    (0x0023_1c70, app_module_try_exit_when_loaded, "AppModule_TryExitWhenLoaded"),
    (0x0023_5570, game_state_reset_overlay_exit, "GameStateReset_OverlayExit"),
    (0x0023_6e60, game_state_results_update, "GameStateResults_Update"),
    (0x0024_1200, mc_manager_read_save_chunk, "MCManager_ReadSaveChunk"),
    (0x0026_5e40, shared_login_man_do_update, "cSharedLoginMan_DoUpdate"),
    (0x0026_68a8, usb_keyboard_on_connect, "cUsbKeyboard_OnConnect"),
    (0x0026_d5e8, world_state_stream_ctor, "WorldStateStream_ctor"),
    (0x0026_e528, replay_seek_next_snapshot, "Replay_SeekNextSnapshot"),
    (0x0026_e568, replay_seek_prev_snapshot, "Replay_SeekPrevSnapshot"),
    (0x0026_ed98, replay_next_camera, "Replay_NextCamera"),
    (0x0026_f7b8, replay_advance_from_state10, "Replay_AdvanceFromState10"),
    (0x0027_6b98, script_list_get_state, "ScriptList_GetState"),
    (0x0027_70c0, script_list_is_current_skippable, "ScriptList_IsCurrentSkippable"),
    (0x0027_8a58, script_engine_clear_locator_table, "ScriptEngine_ClearLocatorTable"),
    (0x0027_8e90, script_engine_list_add_movie, "ScriptEngine_ListAddMovie"),
    (0x0028_8370, ssx_audio_system_get_preset_gain, "SSXAudioSystem_GetPresetGain"),
    (0x002a_6808, crowd_audio_instance_stop_anticipate, "CrowdAudio_InstanceStopAnticipate"),
    (0x002a_e1f0, path_plat_bank_slot_init, "PATH_PlatBankSlotInit"),
    (0x002b_4878, music_player_get_duck_to_loops, "MusicPlayer_GetDuckToLoops"),
    (0x002b_5838, world_trigger_is_type5, "WorldTrigger_IsType5"),
    (0x002b_6aa0, world_sound_ctor, "cWorldSound_ctor"),
    (0x002c_03a8, world_painter_query_reset_list, "WorldPainterQuery_ResetList"),
    (0x002c_2540, ustr_from_ascii_2, "UStr_FromAscii_2"),
    (0x002c_25b8, ustr_copy_n, "UStr_CopyN"),
    (0x002e_a820, cut_scene_transition_man_force_off, "cCutSceneTransitionMan_ForceOff"),
    (0x0031_9b98, mem_get_block_size, "MEM_GetBlockSize"),
    (0x0031_9cc8, mem_report_stats_locked, "MEM_ReportStatsLocked"),
    (0x0032_6a28, pad_queue_init, "PadQueue_Init"),
    (0x0035_93d0, anim_delta_v36, "AnimDelta_v36"),
    (0x0035_b690, rail_modifier_list_push_back, "RailModifier_ListPushBack"),
    (0x0036_98a0, ps2_graphics_man_free_blended_matrix_history, "PS2GraphicsMan_FreeBlendedMatrixHistory"),
    (0x0037_14b8, emitter_ctor, "mEmitter_ctor"),
    (0x003a_1cf0, ui_text_scroll_replace_wide_string, "cUITextScroll_ReplaceWideString"),
    (0x003a_6e98, stream_man_init, "cStreamMan_Init"),
    (0x003a_a5c8, ps2_chunk_resolve_v14, "cPS2ChunkResolve_v14"),
    (0x003a_cc50, wscript_cache_set_data, "tWScriptCache_SetData"),
];

// ---------------------------------------------------------------------------
// Callees (the generated name of each address, so a later hand-written
// replacement of a callee is reached through its override slot too).

mod callee {
    use super::{ee, Callee};

    pub const FE_STATE_ON_ACTIVATE: Callee = Callee { addr: 0x0018_6518, f: ee::s02::cFEState_OnActivate };
    pub const REQUEST_LINE_UPDATE_SONG_NAMES: Callee = Callee { addr: 0x0019_7bc8, f: ee::s02::cFEStateRequestLine_UpdateSongNames };
    pub const REQUEST_LINE_UPDATE_SONG_ICONS: Callee = Callee { addr: 0x0019_7ca0, f: ee::s02::cFEStateRequestLine_UpdateSongIcons };
    pub const REQUEST_LINE_UPDATE_SCROLL_ARROWS: Callee = Callee { addr: 0x0019_7db8, f: ee::s02::cFEStateRequestLine_UpdateScrollArrows };
    pub const REQUEST_LINE_UPDATE_SONG_COUNT: Callee = Callee { addr: 0x0019_8118, f: ee::s02::cFEStateRequestLine_UpdateSongCount };
    pub const NET_FE_STATE_ON_DISCONNECTED_POPUP_EVENT: Callee = Callee { addr: 0x001a_9668, f: ee::s03::NetFEState_OnDisconnectedPopupEvent };
    pub const NET_FE_STATE_ON_CHILD_EVENT: Callee = Callee { addr: 0x001a_97b8, f: ee::s03::NetFEState_OnChildEvent };
    pub const NET_FE_STATE_ON_ACTIVATE: Callee = Callee { addr: 0x001a_87d0, f: ee::s03::NetFEState_OnActivate };
    pub const CHAT_ROOM_UPDATE_PLAYER_QUALITY: Callee = Callee { addr: 0x001b_3bf0, f: ee::s03::cFEStateChatRoom_UpdatePlayerQuality };
    pub const FE_STATE_LOGIN_CTOR: Callee = Callee { addr: 0x001b_ee30, f: ee::s03::cFEStateLogin_ctor };
    pub const FE_STATE_LOGIN_ON_CHILD_EVENT: Callee = Callee { addr: 0x001b_f6c0, f: ee::s03::cFEStateLogin_OnChildEvent };
    pub const NET_QUICK_PLAY_POPUP_ON_OPTIONS_EVENT: Callee = Callee { addr: 0x001c_1b88, f: ee::s03::cFEStateNetQuickPlayPopup_OnOptionsPopupEvent };
    pub const FE_STATE_STATS_REFRESH_RESULTS: Callee = Callee { addr: 0x001c_3310, f: ee::s03::cFEStateStats_RefreshResults };
    pub const POPUP_CONFIRM_GET_TEXT_BOX_SIZE: Callee = Callee { addr: 0x001c_9210, f: ee::s03::cFEPopupConfirm_GetTextBoxSize };
    pub const PREVIEW_REWARD_POLL_IMAGE_LOAD: Callee = Callee { addr: 0x001d_2198, f: ee::s04::cFEStatePreviewReward_PollImageLoad };
    pub const MOUNTAIN_ROOM_UPDATE_EVENT_ICONS: Callee = Callee { addr: 0x001d_3340, f: ee::s04::cFEStateMountainRoom_UpdateEventIcons };
    pub const TROPHY_ROOM_LOAD_TROPHY_TEXTURES: Callee = Callee { addr: 0x001d_4a20, f: ee::s04::cFEStateTrophyRoom_LoadTrophyTextures };
    pub const SECTION_MAN_FIND_LOGICAL_SECTION: Callee = Callee { addr: 0x0022_e0e0, f: ee::s05::SectionMan_FindLogicalSection };
    pub const GAME_MODULE_REQUEST_STATE: Callee = Callee { addr: 0x0023_1250, f: ee::s06::GameModule_RequestState };
    pub const APP_MODULE_TRY_EXIT: Callee = Callee { addr: 0x0023_1cf0, f: ee::s06::AppModule_TryExit };
    pub const S_LOGIN_UPDATE: Callee = Callee { addr: 0x003f_8048, f: ee::s14::SLogin_Update };
    pub const USB_KB_DEFAULT_CONNECT_CB: Callee = Callee { addr: 0x003f_6358, f: ee::s14::UsbKb_DefaultConnectCb };
    pub const WORLD_STATE_STREAM_RESET: Callee = Callee { addr: 0x0026_d628, f: ee::s07::WorldStateStream_Reset };
    pub const REPLAY_STREAM_CTOR: Callee = Callee { addr: 0x0026_e8e0, f: ee::s07::ReplayStream_ctor };
    pub const REPLAY_FIND_NEXT_SNAPSHOT: Callee = Callee { addr: 0x0026_e468, f: ee::s07::Replay_FindNextSnapshot };
    pub const REPLAY_FIND_PREV_SNAPSHOT: Callee = Callee { addr: 0x0026_e490, f: ee::s07::Replay_FindPrevSnapshot };
    pub const REPLAY_SNAP_TO_SNAPSHOT: Callee = Callee { addr: 0x0026_e4b8, f: ee::s07::Replay_SnapToSnapshot };
    pub const REPLAY_APPLY_CAMERA: Callee = Callee { addr: 0x0026_edd8, f: ee::s07::Replay_ApplyCamera };
    pub const REPLAY_SET_STATE: Callee = Callee { addr: 0x0026_f4a0, f: ee::s07::Replay_SetState };
    pub const SCRIPT_LIST_ADD_MOVIE: Callee = Callee { addr: 0x0027_6048, f: ee::s07::ScriptList_AddMovie };
    pub const SCRIPT_LIST_POLL_LOADING: Callee = Callee { addr: 0x0027_7778, f: ee::s07::ScriptList_PollLoading };
    pub const SCRIPT_CMD_QUEUE_AT: Callee = Callee { addr: 0x0028_3d70, f: ee::s07::ScriptCmdQueue_At };
    pub const SCRIPT_ENGINE_UNLOAD_WORLD_SCRIPTS: Callee = Callee { addr: 0x0027_8cd8, f: ee::s07::ScriptEngine_UnloadWorldScripts };
    pub const SCRIPT_ENGINE_LIST_STOP: Callee = Callee { addr: 0x0027_90a0, f: ee::s07::ScriptEngine_ListStop };
    pub const CROWD_AUDIO_INSTANCE_STOP_SOUND: Callee = Callee { addr: 0x002a_6848, f: ee::s08::CrowdAudio_InstanceStopSound };
    pub const SONG_INSTANCE_GET_DUCK_TO_LOOPS: Callee = Callee { addr: 0x002b_4868, f: ee::s08::SongInstance_GetDuckToLoops };
    pub const WORLD_TRIGGER_MANAGER_GET_TRIGGER_INFO: Callee = Callee { addr: 0x002b_63d0, f: ee::s08::WorldTriggerManager_GetTriggerInfo };
    pub const MEM_MAN_FREE: Callee = Callee { addr: 0x0031_7e98, f: ee::s10::f_00317e98 };
    pub const MEM_GET_POOL_BLOCK_SIZE: Callee = Callee { addr: 0x0031_a130, f: ee::s10::MEM_GetPoolBlockSize };
    pub const DL_HEAP_USABLE_SIZE: Callee = Callee { addr: 0x0031_fbb8, f: ee::s10::DLHeap_UsableSize };
    pub const MEM_REPORT_POOL_STATS: Callee = Callee { addr: 0x0031_a200, f: ee::s10::MEM_ReportPoolStats };
    pub const PS2_PAD_CREATE: Callee = Callee { addr: 0x0032_6d60, f: ee::s10::PS2Pad_Create };
    pub const ANIM_OBJECT_V36: Callee = Callee { addr: 0x0034_1fe8, f: ee::s11::AnimObject_v36 };
    pub const DYNAMIC_EMITTER_CTOR: Callee = Callee { addr: 0x0037_0b60, f: ee::s12::DynamicEmitter_ctor };
    pub const UI_STATE_CTOR: Callee = Callee { addr: 0x0039_e2a0, f: ee::s13::UIState_ctor };
    pub const UI_STATE_UPDATE: Callee = Callee { addr: 0x0039_e510, f: ee::s13::UIState_Update };
    pub const UI_TEXT_SET_STRING: Callee = Callee { addr: 0x003a_0c50, f: ee::s13::UIText_SetString };
    pub const UI_TEXT_SCROLL_CLEAR_LINES: Callee = Callee { addr: 0x003a_18c0, f: ee::s13::cUITextScroll_ClearLines };
    pub const UI_TEXT_SCROLL_SET_WIDE_STRING: Callee = Callee { addr: 0x003a_1d30, f: ee::s13::cUITextScroll_SetWideString };
    pub const TWSCRIPT_CACHE_RELOCATE: Callee = Callee { addr: 0x003a_ccd8, f: ee::s13::tWScriptCache_Relocate };
    pub const EA_MUTEX_LOCK: Callee = Callee { addr: 0x003e_5700, f: ee::s14::EAMutex_Lock };
    pub const EA_MUTEX_UNLOCK: Callee = Callee { addr: 0x003e_5760, f: ee::s14::EAMutex_Unlock };
    pub const SCE_USBKB_SET_ARRANGEMENT: Callee = Callee { addr: 0x0040_c7c0, f: ee::s15::sceUsbKbSetArrangement };
    pub const SCE_USBKB_SET_READ_MODE: Callee = Callee { addr: 0x0040_d430, f: ee::s15::sceUsbKbSetReadMode };
    pub const MEMSET: Callee = Callee { addr: 0x0041_6210, f: ee::s15::memset };
    pub const SPRINTF: Callee = Callee { addr: 0x0041_7828, f: ee::s15::sprintf };
}

// ---------------------------------------------------------------------------
// Helpers.

/// The low word of a register.
#[inline(always)]
fn r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

/// A `gp`-relative global word.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r(c, GP).wrapping_add(off as u32)
}

/// `lw`/`lh`/`sw` into a named register, and `lwc1`/`swc1`. Methods on the CPU
/// so that the address may read the registers (two-phase borrows).
trait Ops {
    /// `lw reg, [a]`, leaving the sign-extended value in the register.
    fn lw_r(&mut self, reg: usize, a: u32) -> u64;
    /// `lh reg, [a]`.
    fn lh_r(&mut self, reg: usize, a: u32) -> u64;
    /// `sw reg, [a]`.
    fn sw_r(&mut self, a: u32, reg: usize);
    /// `reg += n` (`addiu`, sign-extended result).
    fn inc_r(&mut self, reg: usize, n: u32);
    /// `lwc1 f{n}, [a]`.
    fn ld_f(&mut self, n: usize, a: u32);
    /// `swc1 f{n}, [a]`.
    fn st_f(&mut self, n: usize, a: u32);
}

impl Ops for Cpu {
    #[inline(always)]
    fn lw_r(&mut self, reg: usize, a: u32) -> u64 {
        let v = self.lw(a);
        self.g[reg][0] = v;
        v
    }
    #[inline(always)]
    fn lh_r(&mut self, reg: usize, a: u32) -> u64 {
        let v = self.lh(a);
        self.g[reg][0] = v;
        v
    }
    #[inline(always)]
    fn sw_r(&mut self, a: u32, reg: usize) {
        let v = self.g[reg][0];
        self.sw(a, v);
    }
    #[inline(always)]
    fn inc_r(&mut self, reg: usize, n: u32) {
        let v = r(self, reg).wrapping_add(n);
        self.g[reg][0] = sx(v);
    }
    #[inline(always)]
    fn ld_f(&mut self, n: usize, a: u32) {
        self.lwc1(n, a);
    }
    #[inline(always)]
    fn st_f(&mut self, n: usize, a: u32) {
        self.swc1(n, a);
    }
}

/// The `+0x8`/`+0xC` pair a front-end state constructor writes: the class
/// vtable and the state id.
#[inline(always)]
fn init_state(c: &mut Cpu, this: u64, vtable: u32, id: u32) {
    c.g[V0][0] = id as u64;
    c.g[V1][0] = sx(vtable);
    c.sw_r((this as u32).wrapping_add(0xc), V0);
    c.sw_r((this as u32).wrapping_add(0x8), V1);
    c.g[V0][0] = this;
}

// ---------------------------------------------------------------------------
// Front-end states.

/// `cFEStateBonusMaterial* cFEStateBonusMaterial_ctor(cFEStateBonusMaterial*)`
/// (0x195498): the shared UI-state constructor, then vtable `0x46b918` at +0x8
/// and state id 5 at +0xC.
pub fn fe_state_bonus_material_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        callee::UI_STATE_CTOR.call(c, 0x0019_54ac)?;
        init_state(c, this, 0x0046_b918, 5);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateRequestLine_RefreshList(cFEStateRequestLine* self)`
/// (0x197B88): refresh the song names, icons, scroll arrows and the song
/// count of the request-line screen.
pub fn fe_state_request_line_refresh_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        callee::REQUEST_LINE_UPDATE_SONG_NAMES.call(c, 0x0019_7b9c)?;
        c.g[A0][0] = c.g[S0][0];
        callee::REQUEST_LINE_UPDATE_SONG_ICONS.call(c, 0x0019_7ba4)?;
        c.g[A0][0] = c.g[S0][0];
        callee::REQUEST_LINE_UPDATE_SCROLL_ARROWS.call(c, 0x0019_7bac)?;
        c.g[A0][0] = c.g[S0][0];
        callee::REQUEST_LINE_UPDATE_SONG_COUNT.call(c, 0x0019_7bb4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void FEFlowState_Free(FEFlowState* self)` (0x1A06B0): close the state,
/// freeing the buffer it owns at +0x4.
pub fn fe_flow_state_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        let owned = Addr(r(c, S0).wrapping_add(4)).lw(c);
        c.g[A0][0] = owned;
        if owned == 0 {
            Addr(r(c, S0)).sb(c, 0);
        } else {
            c.cycles += 4;
            callee::MEM_MAN_FREE.call(c, 0x001a_06d4)?;
            Addr(r(c, S0).wrapping_add(4)).sw(c, 0);
            Addr(r(c, S0)).sb(c, 0);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateMPCircuit* cFEStateMPCircuit_ctor(cFEStateMPCircuit*)` (0x1A3218):
/// the UI-state constructor, vtable `0x469178` and state id 0x17.
pub fn fe_state_mp_circuit_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        callee::UI_STATE_CTOR.call(c, 0x001a_322c)?;
        init_state(c, this, 0x0046_9178, 0x17);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateChatRoom_OnActivate(cFEStateChatRoom* self, void* arg)`
/// (0x1B3528): refresh the player quality, then run the network state's own
/// activation.
pub fn fe_state_chat_room_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY.call(c, 0x001b_3544)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        callee::NET_FE_STATE_ON_ACTIVATE.call(c, 0x001b_3550)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateNetQuickPlayPopup_OnChildEvent(self, child, event)` (0x1C1B48):
/// the options popup (id 0x82F) handles the event itself, every other child
/// goes to the network state.
pub fn fe_state_net_quick_play_popup_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x82f;
        frame.save_ra(c, 0);
        let event = Addr(r(c, A1).wrapping_add(0x18)).lw(c);
        c.g[V1][0] = event;
        if event == c.g[V0][0] {
            c.cycles += 4;
            callee::NET_QUICK_PLAY_POPUP_ON_OPTIONS_EVENT.call(c, 0x001c_1b68)?;
        } else {
            c.cycles += 3;
            callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, 0x001c_1b78)?;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateStats_OnWidgetNotify(self, widget, kind, value)` (0x1C4B98):
/// a stats-tab change (kind 4) records the tab and rebuilds the results list;
/// the other kinds only report their widget id.
pub fn fe_state_stats_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 4;
        let kind = c.g[A2][0];
        frame.save_ra(c, 0);
        if kind == c.g[V0][0] {
            c.cycles += 3;
            let tab = c.g[A3][0];
            Addr(r(c, A0).wrapping_add(0x6e8)).sw(c, tab);
            callee::FE_STATE_STATS_REFRESH_RESULTS.call(c, 0x001c_4bc4)?;
            c.g[V0][0] = 0x101;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 6;
            if kind == c.g[V0][0] {
                c.cycles += 2;
                c.g[V0][0] = 0x100;
            } else {
                c.g[V0][0] = 0x101;
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEPopupConfirm_SetTextMeasured(self, cUIText* text, const char* str,
/// int flags)` (0x1C9B28): size the confirm popup's text box for the string,
/// then set the ASCII string on it.
pub fn fe_popup_confirm_set_text_measured(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        callee::POPUP_CONFIRM_GET_TEXT_BOX_SIZE.call(c, 0x001c_9b44)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        callee::UI_TEXT_SET_STRING.call(c, 0x001c_9b50)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStatePreviewReward_Update(cFEStatePreviewReward* self)`
/// (0x1D1FB0): poll the reward image while one is loading (handle >= 0), then
/// run the UI state update.
pub fn fe_state_preview_reward_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let handle = c.lw_r(V0, r(c, S0).wrapping_add(0x6c));
        if (handle as i64) >= 0 {
            c.cycles += 2;
            callee::PREVIEW_REWARD_POLL_IMAGE_LOAD.call(c, 0x001d_1fd4)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        callee::UI_STATE_UPDATE.call(c, 0x001d_1fdc)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateMountainRoom_OnActivate(self, arg)` (0x1D2A90): rebuild the
/// event icons, then run the shared state activation.
pub fn fe_state_mountain_room_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        callee::MOUNTAIN_ROOM_UPDATE_EVENT_ICONS.call(c, 0x001d_2aac)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        callee::FE_STATE_ON_ACTIVATE.call(c, 0x001d_2ab8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateTrophyRoom_Update(cFEStateTrophyRoom* self)` (0x1D4760):
/// load the trophy textures while any are pending (count > 0), then run the
/// UI state update.
pub fn fe_state_trophy_room_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let pending = c.lw_r(V0, r(c, S0).wrapping_add(0x5c));
        if (pending as i64) > 0 {
            c.cycles += 2;
            callee::TROPHY_ROOM_LOAD_TROPHY_TEXTURES.call(c, 0x001d_4784)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        callee::UI_STATE_UPDATE.call(c, 0x001d_478c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateRelogin* cFEStateRelogin_ctor(cFEStateRelogin*)` (0x1D5038): the
/// login-state constructor, vtable `0x468050` and state id 0x34.
pub fn fe_state_relogin_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        callee::FE_STATE_LOGIN_CTOR.call(c, 0x001d_504c)?;
        init_state(c, this, 0x0046_8050, 0x34);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateRelogin_OnChildEvent(self, child, event)` (0x1D5240): the
/// disconnect popup (id 0x102) handles the event itself, every other child
/// goes to the login state.
pub fn fe_state_relogin_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x102;
        frame.save_ra(c, 0);
        let event = Addr(r(c, A1).wrapping_add(0x18)).lw(c);
        c.g[V1][0] = event;
        if event == c.g[V0][0] {
            c.cycles += 4;
            callee::NET_FE_STATE_ON_DISCONNECTED_POPUP_EVENT.call(c, 0x001d_5260)?;
        } else {
            c.cycles += 3;
            callee::FE_STATE_LOGIN_ON_CHILD_EVENT.call(c, 0x001d_5270)?;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenTopTimes_OnWidgetAction(self, widget, action)` (0x1F80D8):
/// action 5 on a widget forwards to the screen's virtual handler at vtable
/// slot `+0xD0`, with `this` adjusted by the slot's short delta.
pub fn ov_screen_top_times_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let this = c.g[A0][0];
        c.g[A3][0] = this;
        let widget = c.g[A1][0];
        frame.save_ra(c, 0);
        if widget == 0 {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 5;
            let action = c.g[A2][0];
            frame.restore_ra(c, 0);
            if action == c.g[V0][0] {
                c.cycles += 5;
                let vt = c.lw_r(V0, r(c, A3).wrapping_add(8));
                let delta = c.lh_r(A0, (vt as u32).wrapping_add(0xd0));
                let target = c.lw_r(V1, (vt as u32).wrapping_add(0xd4));
                let t = target as u32;
                let a0 = sx(r(c, A3).wrapping_add(delta as u32));
                c.g[A0][0] = a0;
                call_ptr(c, t, 0x001f_8108)?;
                c.cycles += 1;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Game loading and results.

/// `int AppModule_TryExitWhenLoaded(AppModule* self)` (0x231C70): once the
/// loading progress global reaches 100%, ask the module to exit.
pub fn app_module_try_exit_when_loaded(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        let loader = c.lw_r(V0, gp(c, 0x2a74));
        frame.save_ra(c, 0);
        c.g[AT][0] = 0x42c8_0000;
        c.mtc1(1, 1);
        c.ld_f(0, (loader as u32).wrapping_add(0x38));
        c.c_eq_s(0, 1);
        let loaded = c.fcc();
        c.g[V0][0] = 0;
        if loaded {
            c.cycles += 2;
            callee::APP_MODULE_TRY_EXIT.call(c, 0x0023_1ca0)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameStateReset_OverlayExit(GameStateReset* self)` (0x235570): unload
/// the world scripts, stop the script lists, and move to state 5.
pub fn game_state_reset_overlay_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let world = c.lw_r(A0, gp(c, -0x84c));
        let _ = world;
        callee::SCRIPT_ENGINE_UNLOAD_WORLD_SCRIPTS.call(c, 0x0023_5588)?;
        let world = c.lw_r(A0, gp(c, -0x84c));
        let _ = world;
        c.g[A1][0] = 1;
        callee::SCRIPT_ENGINE_LIST_STOP.call(c, 0x0023_5594)?;
        c.g[V0][0] = 5;
        frame.restore_ra(c, 0);
        Addr(r(c, S0).wrapping_add(0x10)).sw(c, 5);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void GameStateResults_Update(GameStateResults* self)` (0x236E60): once
/// the results screen reports itself ready (field +0x8 == 1), ask the game
/// module for state 0xE (leaving the results screen).
pub fn game_state_results_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 1;
        frame.save_ra(c, 0);
        let state = c.lw_r(V1, r(c, A0).wrapping_add(8));
        frame.restore_ra(c, 0);
        if state == c.g[V0][0] {
            c.cycles += 7;
            let world = c.lw_r(V0, gp(c, -0x848));
            c.g[A1][0] = 0xe;
            c.g[A2][0] = 2;
            c.g[A3][0] = 0;
            let module = c.lw_r(A0, (world as u32).wrapping_add(0x84));
            let _ = module;
            callee::GAME_MODULE_REQUEST_STATE.call(c, 0x0023_6e90)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_ReadSaveChunk(MCManager* self, char* buf, int n)` (0x241200):
/// account `n` more bytes read, then pass the chunk on to the current save
/// source (virtual at `*(self+0x434)`, slot `+0xF0`).
pub fn mc_manager_read_save_chunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        let done = c.lw_r(V0, (this as u32).wrapping_add(0xe8));
        let source = c.lw_r(A3, (this as u32).wrapping_add(0x434));
        let total = sx((done as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = total;
        Addr((this as u32).wrapping_add(0xe8)).sw(c, total);
        let vt = c.lw_r(V1, source as u32);
        let delta = c.lh_r(A0, (vt as u32).wrapping_add(0xf0));
        let target = c.lw_r(V0, (vt as u32).wrapping_add(0xf4));
        let t = target as u32;
        let a0 = sx((source as u32).wrapping_add(delta as u32));
        c.g[A0][0] = a0;
        call_ptr(c, t, 0x0024_1230)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Login / USB keyboard.

/// `void cSharedLoginMan_DoUpdate(cSharedLoginMan* self)` (0x265E40): drop the
/// pending request, clear the shared-login global and tell the SLogin service
/// to update; the manager's state becomes 7.
pub fn shared_login_man_do_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save_ra(c, 0);
        Addr((this as u32).wrapping_add(4)).sw(c, 0);
        c.g[A0][0] = 0x0053_6760;
        Addr(gp(c, 0x2f0)).sw(c, 0);
        callee::S_LOGIN_UPDATE.call(c, 0x0026_5e64)?;
        c.g[V1][0] = 7;
        frame.restore_ra(c, 0);
        Addr(this as u32).sw(c, 7);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void cUsbKeyboard_OnConnect(cUsbKeyboard* self)` (0x2668A8): install the
/// default connect callback and reset the keyboard's arrangement and read
/// mode to zero.
pub fn usb_keyboard_on_connect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::USB_KB_DEFAULT_CONNECT_CB.call(c, 0x0026_68bc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        callee::SCE_USBKB_SET_ARRANGEMENT.call(c, 0x0026_68c8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        callee::SCE_USBKB_SET_READ_MODE.call(c, 0x0026_68d4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// World / replay streams.

/// `WorldStateStream* WorldStateStream_ctor(WorldStateStream*)` (0x26D5E8): the
/// replay-stream constructor, vtable `0x481850`, and an empty stream.
pub fn world_state_stream_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        callee::REPLAY_STREAM_CTOR.call(c, 0x0026_d5fc)?;
        c.g[V1][0] = sx(0x0048_1850);
        c.g[A0][0] = c.g[S0][0];
        Addr(this as u32).sw(c, 0x0048_1850);
        callee::WORLD_STATE_STREAM_RESET.call(c, 0x0026_d610)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_SeekNextSnapshot(Replay* self)` (0x26E528): find the snapshot
/// after the current one (+0x484 index), keep it at +0x488, advance the index
/// to its successor and snap the replay there.
pub fn replay_seek_next_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let index = c.lw_r(A1, r(c, S0).wrapping_add(0x484));
        let _ = index;
        callee::REPLAY_FIND_NEXT_SNAPSHOT.call(c, 0x0026_e540)?;
        let snapshot = c.g[V0][0];
        Addr(r(c, S0).wrapping_add(0x488)).sw(c, snapshot);
        c.g[A0][0] = c.g[S0][0];
        let next = c.lw_r(V1, (snapshot as u32).wrapping_add(0x30));
        Addr(r(c, S0).wrapping_add(0x484)).sw(c, next);
        callee::REPLAY_SNAP_TO_SNAPSHOT.call(c, 0x0026_e554)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_SeekPrevSnapshot(Replay* self)` (0x26E568): the same one
/// snapshot backwards (index at +0x484, snapshot at +0x488).
pub fn replay_seek_prev_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let index = c.lw_r(A1, r(c, S0).wrapping_add(0x484));
        let _ = index;
        callee::REPLAY_FIND_PREV_SNAPSHOT.call(c, 0x0026_e580)?;
        let snapshot = c.g[V0][0];
        Addr(r(c, S0).wrapping_add(0x488)).sw(c, snapshot);
        c.g[A0][0] = c.g[S0][0];
        let next = c.lw_r(V1, (snapshot as u32).wrapping_add(0x30));
        Addr(r(c, S0).wrapping_add(0x484)).sw(c, next);
        callee::REPLAY_SNAP_TO_SNAPSHOT.call(c, 0x0026_e594)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_NextCamera(Replay* self)` (0x26ED98): step the camera index at
/// +0x630 through the nine replay cameras (`(n + 1) % 9`) and apply it.
pub fn replay_next_camera(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 9;
        c.cycles += 9;
        let index = c.lw_r(V0, r(c, A2).wrapping_add(0x630));
        let bumped = sx((index as u32).wrapping_add(1));
        c.g[V0][0] = bumped;
        let divisor = c.g[A1][0] as u32;
        let (_, rem) = abi::div(c, bumped as u32, divisor);
        c.g[V1][0] = sx(rem);
        Addr(r(c, A2).wrapping_add(0x630)).sw(c, sx(rem));
        callee::REPLAY_APPLY_CAMERA.call(c, 0x0026_edc8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_AdvanceFromState10(Replay* self)` (0x26F7B8): a replay in state
/// 10 moves on to state 11; states 14 and 15 are ignored.
pub fn replay_advance_from_state10(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let state = c.lw_r(V1, r(c, A0));
        let frame = Frame::open(c, 0x10);
        let d = sx((state as u32).wrapping_sub(0xe));
        c.g[V0][0] = d;
        let ignored = d < 2;
        c.g[V0][0] = (ignored as u64) ^ 1;
        frame.save_ra(c, 0);
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0xa;
            let not_ten = c.g[V1][0] != c.g[V0][0];
            frame.restore_ra(c, 0);
            if !not_ten {
                c.cycles += 2;
                c.g[A1][0] = 0xb;
                // The call returns to 0x26F7E8, which is the loop head: control
                // falls through the shared tail block (one more cycle, `ra`
                // reloaded from the frame) instead of returning straight away.
                callee::REPLAY_SET_STATE.call(c, 0x0026_f7e8)?;
                c.cycles += 1;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script engine.

/// `int ScriptList_GetState(ScriptList* self)` (0x276B98): while the list is
/// loading (state +0xA4 == 1) poll the load, then return the state.
pub fn script_list_get_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let state = c.lw_r(V0, r(c, S0).wrapping_add(0xa4));
        if state == c.g[V1][0] {
            c.cycles += 4;
            callee::SCRIPT_LIST_POLL_LOADING.call(c, 0x0027_6bc0)?;
            let state = c.lw_r(V0, r(c, S0).wrapping_add(0xa4));
            let _ = state;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int ScriptList_IsCurrentSkippable(ScriptList* self)` (0x2770C0): only a
/// running list (state 3) can be skipped, and only when the current command's
/// flags at +0x8 have bit 1 set.
pub fn script_list_is_current_skippable(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 3;
        frame.save_ra(c, 0);
        let this = r(c, A0);
        let state = c.lw_r(V0, this.wrapping_add(0xa4));
        if state != c.g[V1][0] {
            c.g[V0][0] = 1;
        } else {
            c.cycles += 6;
            c.g[A0][0] = sx(this.wrapping_add(0xc));
            c.g[A1][0] = 0;
            callee::SCRIPT_CMD_QUEUE_AT.call(c, 0x0027_70e4)?;
            let cmd = c.lw_r(V0, r(c, V0).wrapping_add(8));
            let flags = sx(((cmd as u32) as i32 >> 2) as u32);
            c.g[V0][0] = flags & 1;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptEngine_ClearLocatorTable(ScriptEngine* self, int key)`
/// (0x278A58): clear the locator-table entry for a logical section (found in
/// the current world's section manager).
pub fn script_engine_clear_locator_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        let world = c.lw_r(V1, gp(c, -0x848));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let engine = c.g[A0][0];
        c.g[S0][0] = engine;
        let manager = c.lw_r(V0, (world as u32).wrapping_add(0x84));
        let key = c.lw_r(A0, (manager as u32).wrapping_add(0x78));
        let _ = key;
        callee::SECTION_MAN_FIND_LOGICAL_SECTION.call(c, 0x0027_8a78)?;
        let index = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = index;
        frame.restore_ra(c, 0);
        let entry = sx((engine as u32).wrapping_add(index as u32));
        c.g[S0][0] = entry;
        Addr((entry as u32).wrapping_add(0x558)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptEngine_ListAddMovie(ScriptEngine* self, int list, tag a, tag b,
/// tag c, tag d, tag f)` (0x278E90): add a movie entry to list `list` (0xCC
/// bytes each at `*(self+0x54C) + list*0xCC`).
pub fn script_engine_list_add_movie(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        c.g[V0][0] = 0xcc;
        let frame = Frame::open(c, 0x10);
        let index = abi::arg(c, 1);
        let lo = abi::mult(c, index, 0xcc);
        c.g[V1][0] = sx(lo);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A2][0];
        c.g[A2][0] = c.g[A3][0];
        let lists = c.lw_r(A0, r(c, A0).wrapping_add(0x54c));
        c.g[A3][0] = c.g[T0][0];
        c.g[T0][0] = c.g[T1][0];
        let entry = sx((lists as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = entry;
        c.g[T1][0] = c.g[T2][0];
        callee::SCRIPT_LIST_ADD_MOVIE.call(c, 0x0027_8ec0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void tWScriptCache_SetData(sWScriptCacheItem* e, int index, int value)`
/// (0x3ACC50): record the data value and type 2 / owner 1, then relocate the
/// item in the script cache.
pub fn wscript_cache_set_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        let item = c.g[A0][0];
        c.g[V0][0] = item;
        frame.save_ra(c, 0);
        c.g[A3][0] = c.g[A1][0];
        let value = c.g[A2][0];
        Addr((item as u32).wrapping_add(8)).sw(c, value);
        c.g[V1][0] = 1;
        let tags = c.lw_r(A1, gp(c, 0x16c8));
        let _ = tags;
        c.g[A2][0] = 2;
        Addr(item as u32).sh(c, 2);
        Addr((item as u32).wrapping_add(2)).sh(c, 1);
        c.g[A2][0] = c.g[A3][0];
        callee::TWSCRIPT_CACHE_RELOCATE.call(c, 0x003a_cc80)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Sound.

/// `float SSXAudioSystem_GetPresetGain(SSXAudioSystem* self, int i, int pct)`
/// (0x288370): a non-negative percentage is scaled by 0.01, otherwise the
/// preset's stored gain (custom table at +0x6440 when +0x6410 is set, the
/// default table at +0x63E4 otherwise).
pub fn ssx_audio_system_get_preset_gain(c: &mut Cpu) {
    c.cycles += 2;
    let this = r(c, A0);
    let index = c.g[A1][0];
    if (c.g[A2][0] as i64) < 0 {
        let custom = c.lw_r(V0, this.wrapping_add(0x6410));
        c.cycles += 2;
        c.g[V0][0] = sx((index as u32) << 2);
        let slot = sx(this.wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = slot;
        if custom != 0 {
            c.cycles += 3;
            c.ld_f(0, (slot as u32).wrapping_add(0x6440));
        } else {
            c.cycles += 3;
            c.ld_f(0, (slot as u32).wrapping_add(0x63e4));
        }
    } else {
        c.cycles += 5;
        c.mtc1(A2, 1);
        c.cvt_s_w(1, 1);
        c.ld_f(0, gp(c, -0x4618));
        c.mul_s(0, 1, 0);
    }
    abi::ret(c);
}

/// `void CrowdAudio_InstanceStopAnticipate(CrowdAudioInstance* self)`
/// (0x2A6808): stop the instance if it is in the "sound started" state (2),
/// then mark the slot idle.
pub fn crowd_audio_instance_stop_anticipate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 2;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let state = c.lw_r(V0, r(c, S0).wrapping_add(0x180));
        if state == c.g[V1][0] {
            c.cycles += 3;
            callee::CROWD_AUDIO_INSTANCE_STOP_SOUND.call(c, 0x002a_6830)?;
        }
        Addr(r(c, S0).wrapping_add(0x180)).sw(c, 0);
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PATH_PlatBankSlotInit(int* self)` (0x2AE1F0): clear the 0x28-byte bank
/// slot and mark its three ids -1.
pub fn path_plat_bank_slot_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0x28;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MEMSET.call(c, 0x002a_e20c)?;
        c.g[V1][0] = u64::MAX;
        frame.restore_ra(c, 0);
        Addr(r(c, S0)).sw(c, u64::MAX);
        Addr(r(c, S0).wrapping_add(8)).sw(c, u64::MAX);
        Addr(r(c, S0).wrapping_add(4)).sw(c, u64::MAX);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int MusicPlayer_GetDuckToLoops(MusicPlayer* self)` (0x2B4878): how far the
/// music should duck under the loops; 1 while no song is playing (+0x418
/// clear) or the instance is missing (+0x408 clear), else the song's own value.
pub fn music_player_get_duck_to_loops(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = r(c, A0);
        let playing = c.lw_r(V0, this.wrapping_add(0x418));
        let idle = playing == 0;
        c.g[V0][0] = 1;
        if idle {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 3;
            let song = c.lw_r(A0, this.wrapping_add(0x408));
            frame.restore_ra(c, 0);
            if song != 0 {
                c.cycles += 4;
                callee::SONG_INSTANCE_GET_DUCK_TO_LOOPS.call(c, 0x002b_48a0)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `int WorldTrigger_IsType5(int index)` (0x2B5838): look the trigger up in the
/// world trigger manager and report whether its type is 5.
pub fn world_trigger_is_type5(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        let manager = c.lw_r(A0, gp(c, 0x21e4));
        let _ = manager;
        frame.save_ra(c, 0x10);
        c.g[A2][0] = c.g[SP][0];
        c.g[A3][0] = c.g[SP][0] | 4;
        c.g[T0][0] = c.g[SP][0] | 8;
        c.g[T1][0] = 0;
        c.g[T2][0] = 1;
        callee::WORLD_TRIGGER_MANAGER_GET_TRIGGER_INFO.call(c, 0x002b_585c)?;
        let kind = c.lw_r(V0, r(c, SP));
        let ra = Addr(r(c, SP).wrapping_add(0x10)).ld(c);
        c.g[RA][0] = ra;
        let diff = (kind as u32) ^ 5;
        c.g[V0][0] = (diff < 1) as u64;
        frame.close(c);
        Ok(())
    })
}

/// `cWorldSound* cWorldSound_ctor(cWorldSound* self)` (0x2B6AA0): clear each of
/// the two 0x120-byte trigger slots and give it the shared trigger data
/// (`D_004FF120`) at +0x110.
pub fn world_sound_ctor(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x0050_0000;
    c.g[V1][0] = c.g[A0][0];
    c.g[A3][0] = sx(0x0050_0000u32.wrapping_sub(0xee0));
    c.g[A1][0] = 1;
    c.g[A2][0] = u64::MAX;
    loop {
        c.cycles += 7;
        Addr(r(c, V1)).sw(c, 0);
        c.g[A1][0] = sx(r(c, A1).wrapping_sub(1));
        c.lq(V0, r(c, A3));
        let slot = r(c, V1).wrapping_add(0x110);
        c.sq(V0, slot);
        let more = c.g[A1][0] != c.g[A2][0];
        c.inc_r(V1, 0x120);
        if !more {
            c.cycles += 2;
            c.g[V0][0] = c.g[A0][0];
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x002b_6ab8);
        }
    }
}

// ---------------------------------------------------------------------------
// World painter and string helpers.

/// `void cWorldPainterQuery_reset(cWorldPainterQuery* self)` (0x2C03A8): set
/// every queued query's result word to -99999.0f.
pub fn world_painter_query_reset_list(c: &mut Cpu) {
    c.cycles += 3;
    let head = c.lw_r(V1, gp(c, 0x790));
    if head != 0 {
        c.cycles += 3;
        c.ld_f(0, gp(c, -0x428c));
        c.lw_r(V0, head as u32);
        loop {
            c.cycles += 7;
            let result = r(c, V0);
            c.st_f(0, result);
            c.lw_r(V1, r(c, V1).wrapping_add(0x14));
            if c.g[V1][0] == 0 {
                break;
            }
            c.lw_r(V0, r(c, V1));
            if event_due(c) {
                deliver_event_here(c, 0x002c_03c0);
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `unsigned short* UStr_FromAscii_2(unsigned short* dst, const char* src)`
/// (0x2C2540): copy a NUL-terminated byte string into `dst` as 16-bit
/// characters (sign-extending each byte) and terminate it; returns `dst`.
pub fn ustr_from_ascii_2(c: &mut Cpu) {
    c.cycles += 4;
    let mut src = r(c, A1);
    let first = Addr(src).lb(c);
    c.g[V0][0] = first;
    c.g[A2][0] = c.g[A0][0];
    c.g[V1][0] = Addr(src).lbu(c);
    let mut dst = r(c, A0);
    if first != 0 {
        loop {
            c.cycles += 8;
            c.g[V0][0] = sx(r(c, V1) << 24);
            src = src.wrapping_add(1);
            c.g[A1][0] = sx(src);
            c.g[V0][0] = sx(((r(c, V0) as i32) >> 24) as u32);
            let ch = c.g[V0][0];
            Addr(dst).sh(c, ch);
            dst = dst.wrapping_add(2);
            c.g[A0][0] = sx(dst);
            c.g[V0][0] = Addr(src).lb(c);
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = Addr(src).lbu(c);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002c_2550);
            }
        }
    }
    c.cycles += 3;
    Addr(dst).sh(c, 0);
    c.g[V0][0] = c.g[A2][0];
    abi::ret(c);
}

/// `unsigned short* UStr_CopyN(unsigned short* dst, const unsigned short* src,
/// int n)` (0x2C25B8): copy at most `n` 16-bit characters and terminate.
pub fn ustr_copy_n(c: &mut Cpu) {
    c.cycles += 3;
    let mut src = r(c, A1);
    let first = Addr(src).lhu(c);
    c.g[V1][0] = first;
    let dst0 = c.g[A0][0];
    c.g[V0][0] = dst0;
    let mut dst = dst0 as u32;
    if first != 0 {
        c.cycles += 2;
        if (c.g[A2][0] as i64) > 0 {
            c.cycles += 1;
            let ch = c.g[V1][0];
            Addr(dst).sh(c, ch);
            loop {
                c.cycles += 5;
                src = src.wrapping_add(2);
                c.g[A1][0] = sx(src);
                c.g[V1][0] = Addr(src).lhu(c);
                dst = dst.wrapping_add(2);
                c.g[A0][0] = sx(dst);
                let at_end = c.g[V1][0] == 0;
                c.g[A2][0] = sx(r(c, A2).wrapping_sub(1));
                if at_end {
                    break;
                }
                c.cycles += 2;
                if (c.g[A2][0] as i64) > 0 {
                    let ch = c.g[V1][0];
                    Addr(dst).sh(c, ch);
                } else {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002c_25d0);
                }
            }
        }
    }
    c.cycles += 2;
    Addr(dst).sh(c, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Visual effects, memory manager, input, animation, render.

/// `void cCutSceneTransitionMan_ForceOff(cCutSceneTransitionMan* self)`
/// (0x2EA820): end a running cut-scene transition at once (virtual call with
/// this+0x20 as the argument) and clear the active flag.
pub fn cut_scene_transition_man_force_off(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        let fading = c.lw_r(V0, (this as u32).wrapping_add(0x10));
        c.g[A1][0] = sx((this as u32).wrapping_add(0x20));
        if fading != 0 {
            c.cycles += 7;
            Addr((this as u32).wrapping_add(0x10)).sw(c, 0);
            let manager = c.lw_r(A2, gp(c, 0x2a90));
            let vt = c.lw_r(V0, (manager as u32).wrapping_add(0x10d8));
            let delta = c.lh_r(A0, (vt as u32).wrapping_add(0x148));
            let target = c.lw_r(V1, (vt as u32).wrapping_add(0x14c));
            let t = target as u32;
            let a0 = sx((manager as u32).wrapping_add(delta as u32));
            c.g[A0][0] = a0;
            call_ptr(c, t, 0x002e_a850)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MEM_GetBlockSize(void* p)` (0x319B98): the pool class of `p` when it
/// lies in one of the six memory pools, else the heap's usable size.
pub fn mem_get_block_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        callee::MEM_GET_POOL_BLOCK_SIZE.call(c, 0x0031_9bac)?;
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            c.g[A0][0] = sx(0x0050_0000u32.wrapping_sub(0xdb0));
            c.g[A1][0] = c.g[S0][0];
            callee::DL_HEAP_USABLE_SIZE.call(c, 0x0031_9bc4)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MEM_ReportStatsLocked(void)` (0x319CC8): report the pool statistics
/// under the memory-manager lock (`D_00538B00`).
pub fn mem_report_stats_locked(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = sx(0x0054_0000u32.wrapping_sub(0x7500));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        callee::EA_MUTEX_LOCK.call(c, 0x0031_9ce4)?;
        callee::MEM_REPORT_POOL_STATS.call(c, 0x0031_9cec)?;
        c.g[A0][0] = c.g[S0][0];
        callee::EA_MUTEX_UNLOCK.call(c, 0x0031_9cf4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PadQueue_Init(PadQueue* self)` (0x326A28): create the four pad queues
/// at +0x2EEC and set the queue parameters (0x1D, 0).
pub fn pad_queue_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 4;
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r(c, S0).wrapping_add(0x2eec));
        callee::PS2_PAD_CREATE.call(c, 0x0032_6a44)?;
        c.g[V1][0] = 0x1d;
        c.sw_r(r(c, S0).wrapping_add(0x2ee8), V0);
        Addr(r(c, S0).wrapping_add(0x2ee0)).sw(c, 0x1d);
        Addr(r(c, S0).wrapping_add(0x2ee4)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AnimDelta_v36(AnimObject* self, int id, float v)` (0x3593D0): id 0x8C
/// advances the object's delta at +0x6C by v/30, any other id is the generic
/// anim-object update.
pub fn anim_delta_v36(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x8c;
        let id = c.g[A1][0];
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        if id != c.g[V0][0] {
            c.cycles += 2;
            callee::ANIM_OBJECT_V36.call(c, 0x0035_9400)?;
        } else {
            c.cycles += 6;
            c.ld_f(0, gp(c, -0x2a68));
            c.ld_f(1, (this as u32).wrapping_add(0x6c));
            c.mul_s(0, 12, 0);
            c.add_s(1, 1, 0);
            c.st_f(1, (this as u32).wrapping_add(0x6c));
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RailModifier_ListPushBack(Link** list, Link* item)` (0x35B690): append
/// `item` to a doubly linked list whose head word sits at `*list`.
pub fn rail_modifier_list_push_back(c: &mut Cpu) {
    c.cycles += 3;
    let head_slot = r(c, A0);
    let item = c.g[A1][0];
    let first = c.lw_r(V0, head_slot);
    let mut link = head_slot;
    if first != 0 {
        c.cycles += 1;
        link = c.lw_r(A0, head_slot) as u32;
        loop {
            c.cycles += 7;
            let next = c.lw_r(V0, link);
            if next == 0 {
                c.cycles += 1;
                Addr(item as u32).sw(c, 0);
                break;
            }
            link = c.lw_r(A0, link) as u32;
            if event_due(c) {
                deliver_event_here(c, 0x0035_b6a0);
            }
        }
    } else {
        Addr(item as u32).sw(c, 0);
    }
    c.cycles += 3;
    Addr((item as u32).wrapping_add(4)).sw(c, sx(link));
    Addr(link).sw(c, item);
    abi::ret(c);
}

/// `void PS2GraphicsMan_FreeBlendedMatrixHistory(PS2GraphicsMan* self)`
/// (0x3698A0): free the blended-matrix history buffer at +0xF48 and clear its
/// count.
pub fn ps2_graphics_man_free_blended_matrix_history(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let buffer = c.lw_r(A0, r(c, S0).wrapping_add(0xf48));
        if buffer == 0 {
            Addr(r(c, S0).wrapping_add(0xf4c)).sw(c, 0);
        } else {
            c.cycles += 3;
            callee::MEM_MAN_FREE.call(c, 0x0036_98c4)?;
            Addr(r(c, S0).wrapping_add(0xf4c)).sw(c, 0);
        }
        c.cycles += 5;
        Addr(r(c, S0).wrapping_add(0xf48)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `mEmitter* mEmitter_ctor(mEmitter* self)` (0x3714B8): the dynamic-emitter
/// constructor, vtable `0x4930D0` and empty particle lists.
pub fn emitter_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        callee::DYNAMIC_EMITTER_CTOR.call(c, 0x0037_14cc)?;
        c.g[V1][0] = sx(0x0049_30d0);
        c.g[V0][0] = c.g[S0][0];
        Addr((this as u32).wrapping_add(0x200)).sw(c, 0);
        Addr((this as u32).wrapping_add(0x1f8)).sw(c, 0x0049_30d0);
        Addr((this as u32).wrapping_add(0x204)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cUITextScroll_ReplaceWideString(cUITextScroll* self, const
/// unsigned short* str)` (0x3A1CF0): drop the current lines and set the new
/// wide string.
pub fn ui_text_scroll_replace_wide_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        callee::UI_TEXT_SCROLL_CLEAR_LINES.call(c, 0x003a_1d0c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        callee::UI_TEXT_SCROLL_SET_WIDE_STRING.call(c, 0x003a_1d18)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cStreamMan_Init(cStreamMan* self, const char* name, void* owner)`
/// (0x3A6E98): remember the owner at +0, format the stream's name into +4 and
/// report success.
pub fn stream_man_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        let name = c.g[A1][0];
        c.g[V0][0] = name;
        frame.save_ra(c, 0);
        let this = r(c, A0);
        let owner = c.g[A2][0];
        Addr(this).sw(c, owner);
        c.g[A0][0] = sx(this.wrapping_add(4));
        c.g[A1][0] = sx(0x004a_47d8);
        c.g[A2][0] = c.g[V0][0];
        c.g[A3][0] = sx(0x004a_47e0);
        callee::SPRINTF.call(c, 0x003a_6ec4)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 1;
        frame.close(c);
        Ok(())
    })
}

/// `int cPS2ChunkResolve_v14(void* self, void* chunk, int a2)` (0x3AA5C8): make
/// the chunk's relative pointer at +0x8 absolute and let the owner's virtual
/// handler at +0x7C resolve it; returns 1.
pub fn ps2_chunk_resolve_v14(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        let this = r(c, A0);
        let chunk = c.g[A1][0];
        c.g[V1][0] = chunk;
        frame.save_ra(c, 0);
        let offset = c.lw_r(V0, (chunk as u32).wrapping_add(8));
        let absolute = sx((offset as u32).wrapping_add(chunk as u32));
        c.g[V0][0] = absolute;
        Addr((chunk as u32).wrapping_add(8)).sw(c, absolute);
        let owner = c.lw_r(A2, this.wrapping_add(8));
        let delta = c.lh_r(V1, (owner as u32).wrapping_add(0x78));
        let target = c.lw_r(V0, (owner as u32).wrapping_add(0x7c));
        let t = target as u32;
        let a0 = sx(this.wrapping_add(delta as u32));
        c.g[A0][0] = a0;
        call_ptr(c, t, 0x003a_a5f4)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 1;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Differential tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, vtable};
    use crate::difftest::{check, check_lohi, put_vec, Rng, DATA, GAME_GP};

    const SEEDS: u32 = 100;

    /// A 32-bit word of test data.
    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// A 16-bit word of test data.
    fn h(c: &mut Cpu, a: u32, v: u16) {
        c.bus.write16(a, v);
    }

    /// A byte of test data.
    fn b(c: &mut Cpu, a: u32, v: u8) {
        c.bus.write8(a, v);
    }

    fn fb(x: f32) -> u32 {
        x.to_bits()
    }

    /// The BELibrary singleton (`gp - 0x1f04`) and a zeroed dummy object it
    /// points at: without it a library lookup tries to load from disc forever.
    fn belib(c: &mut Cpu) {
        w(c, GAME_GP - 0x1f04, DATA + 0x8000);
    }

    const GP_DB0: u32 = 0x49ea_d8; // gp - 0x4618: the 0.01 gain step
    const GP_1_30: u32 = 0x4a06_88; // gp - 0x2a68: 1/30
    const GP_NEG99999: u32 = 0x49ee_64; // gp - 0x428c

    // -------------------------------------------------------------- front end

    #[test]
    fn fe_state_bonus_material_ctor_matches() {
        check(ssx3_ee::s02::f_00195498__generated, fe_state_bonus_material_ctor, SEEDS, &|c, seed| {
            machine(c, seed, 0x1954_98, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn fe_state_request_line_refresh_list_matches() {
        check(ssx3_ee::s02::f_00197b88__generated, fe_state_request_line_refresh_list, 60, &|c, seed| {
            machine(c, seed, 0x197b_88, 4);
            belib(c);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn fe_flow_state_free_matches() {
        check(ssx3_ee::s03::f_001a06b0__generated, fe_flow_state_free, SEEDS, &|c, seed| {
            machine(c, seed, 0x1a06_b0, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            let owned = if seed % 2 == 0 { 0 } else { DATA + 0x800 };
            w(c, DATA + 4, owned);
        });
    }

    #[test]
    fn fe_state_mp_circuit_ctor_matches() {
        check(ssx3_ee::s03::f_001a3218__generated, fe_state_mp_circuit_ctor, SEEDS, &|c, seed| {
            machine(c, seed, 0x1a32_18, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn fe_state_chat_room_on_activate_matches() {
        check(ssx3_ee::s03::f_001b3528__generated, fe_state_chat_room_on_activate, 60, &|c, seed| {
            machine(c, seed, 0x1b35_28, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
        });
    }

    #[test]
    fn fe_state_net_quick_play_popup_on_child_event_matches() {
        check(ssx3_ee::s03::f_001c1b48__generated, fe_state_net_quick_play_popup_on_child_event, 60, &|c, seed| {
            machine(c, seed, 0x1c1b_48, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x300) as u64;
            let event = if seed % 2 == 0 { 0x82f } else { 0x900 };
            w(c, DATA + 0x300 + 0x18, event);
        });
    }

    #[test]
    fn fe_state_stats_on_widget_notify_matches() {
        check(ssx3_ee::s03::f_001c4b98__generated, fe_state_stats_on_widget_notify, SEEDS, &|c, seed| {
            machine(c, seed, 0x1c4b_98, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.g[A2][0] = match seed % 3 {
                0 => 4,
                1 => 6,
                _ => 8,
            };
            c.g[A3][0] = 0x1234;
        });
    }

    #[test]
    fn fe_popup_confirm_set_text_measured_matches() {
        check(ssx3_ee::s03::f_001c9b28__generated, fe_popup_confirm_set_text_measured, 60, &|c, seed| {
            machine(c, seed, 0x1c9b_28, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x400) as u64;
            c.g[A2][0] = (DATA + 0x500) as u64;
            b(c, DATA + 0x500, 0);
        });
    }

    #[test]
    fn fe_state_preview_reward_update_matches() {
        check(ssx3_ee::s04::f_001d1fb0__generated, fe_state_preview_reward_update, 60, &|c, seed| {
            machine(c, seed, 0x1d1f_b0, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            let handle = if seed % 2 == 0 { 0xffff_ffffu32 } else { 5 };
            w(c, DATA + 0x6c, handle);
        });
    }

    #[test]
    fn fe_state_mountain_room_on_activate_matches() {
        check(ssx3_ee::s04::f_001d2a90__generated, fe_state_mountain_room_on_activate, 40, &|c, seed| {
            machine(c, seed, 0x1d2a_90, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
        });
    }

    #[test]
    fn fe_state_trophy_room_update_matches() {
        check(ssx3_ee::s04::f_001d4760__generated, fe_state_trophy_room_update, 40, &|c, seed| {
            machine(c, seed, 0x1d47_60, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            let pending = if seed % 2 == 0 { 0 } else { 2 };
            w(c, DATA + 0x5c, pending);
        });
    }

    #[test]
    fn fe_state_relogin_ctor_matches() {
        check(ssx3_ee::s04::f_001d5038__generated, fe_state_relogin_ctor, SEEDS, &|c, seed| {
            machine(c, seed, 0x1d50_38, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn fe_state_relogin_on_child_event_matches() {
        check(ssx3_ee::s04::f_001d5240__generated, fe_state_relogin_on_child_event, 60, &|c, seed| {
            machine(c, seed, 0x1d52_40, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x300) as u64;
            let event = if seed % 2 == 0 { 0x102 } else { 0x100 };
            w(c, DATA + 0x300 + 0x18, event);
        });
    }

    #[test]
    fn ov_screen_top_times_on_widget_action_matches() {
        check(ssx3_ee::s04::f_001f80d8__generated, ov_screen_top_times_on_widget_action, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x1f80_d8, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            match seed % 3 {
                0 => {
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 5;
                }
                1 => {
                    c.g[A1][0] = (DATA + 0x300) as u64;
                    c.g[A2][0] = 4;
                }
                _ => {
                    c.g[A1][0] = (DATA + 0x300) as u64;
                    c.g[A2][0] = 5;
                    let vt = vtable(c, &mut r, &[0xd0]);
                    w(c, DATA + 8, vt);
                }
            }
        });
    }

    // ------------------------------------------------------- game and results

    #[test]
    fn app_module_try_exit_when_loaded_matches() {
        check(ssx3_ee::s06::f_00231c70__generated, app_module_try_exit_when_loaded, SEEDS, &|c, seed| {
            machine(c, seed, 0x231c_70, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            w(c, GAME_GP + 0x2a74, DATA + 0x100);
            let progress = if seed % 2 == 0 { 100.0 } else { 1.0 };
            w(c, DATA + 0x100 + 0x38, fb(progress));
        });
    }

    #[test]
    fn game_state_reset_overlay_exit_matches() {
        check(ssx3_ee::s06::f_00235570__generated, game_state_reset_overlay_exit, 60, &|c, seed| {
            machine(c, seed, 0x2355_70, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn game_state_results_update_matches() {
        check(ssx3_ee::s06::f_00236e60__generated, game_state_results_update, SEEDS, &|c, seed| {
            machine(c, seed, 0x236e_60, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            w(c, DATA + 8, if seed % 2 == 0 { 1 } else { 0 });
        });
    }

    #[test]
    fn mc_manager_read_save_chunk_matches() {
        check(ssx3_ee::s06::f_00241200__generated, mc_manager_read_save_chunk, SEEDS, &|c, seed| {
            machine(c, seed, 0x2412_00, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x400) as u64;
            c.g[A2][0] = if seed % 2 == 0 { 0x10 } else { 0 };
        });
    }

    // ---------------------------------------------------------- login / usb

    #[test]
    fn shared_login_man_do_update_matches() {
        check(ssx3_ee::s07::f_00265e40__generated, shared_login_man_do_update, SEEDS, &|c, seed| {
            machine(c, seed, 0x265e_40, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn usb_keyboard_on_connect_matches() {
        check(ssx3_ee::s07::f_002668a8__generated, usb_keyboard_on_connect, 60, &|c, seed| {
            machine(c, seed, 0x2668_a8, 3);
            belib(c);
            c.g[A0][0] = DATA as u64;
        });
    }

    // ------------------------------------------------------------- replay

    #[test]
    fn world_state_stream_ctor_matches() {
        check(ssx3_ee::s07::f_0026d5e8__generated, world_state_stream_ctor, 60, &|c, seed| {
            machine(c, seed, 0x26d5_e8, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn replay_seek_next_snapshot_matches() {
        check(ssx3_ee::s07::f_0026e528__generated, replay_seek_next_snapshot, 60, &|c, seed| {
            machine(c, seed, 0x26e5_28, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            w(c, DATA + 0x484, seed % 4);
        });
    }

    #[test]
    fn replay_seek_prev_snapshot_matches() {
        check(ssx3_ee::s07::f_0026e568__generated, replay_seek_prev_snapshot, 60, &|c, seed| {
            machine(c, seed, 0x26e5_68, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            w(c, DATA + 0x484, seed % 4);
        });
    }

    #[test]
    fn replay_next_camera_matches() {
        check_lohi(ssx3_ee::s07::f_0026ed98__generated, replay_next_camera, SEEDS, &|c, seed| {
            machine(c, seed, 0x26ed_98, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            w(c, DATA + 0x630, seed % 9);
        });
    }

    #[test]
    fn replay_advance_from_state10_matches() {
        check(ssx3_ee::s07::f_0026f7b8__generated, replay_advance_from_state10, SEEDS, &|c, seed| {
            machine(c, seed, 0x26f7_b8, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            let state = match seed % 3 {
                0 => 0x0a,
                1 => 0x0e,
                _ => 3,
            };
            w(c, DATA, state);
        });
    }

    // ------------------------------------------------------ script engine

    #[test]
    fn script_list_get_state_matches() {
        check(ssx3_ee::s07::f_00276b98__generated, script_list_get_state, 60, &|c, seed| {
            machine(c, seed, 0x276b_98, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            w(c, DATA + 0xa4, if seed % 2 == 0 { 1 } else { 2 });
        });
    }

    #[test]
    fn script_list_is_current_skippable_matches() {
        check(ssx3_ee::s07::f_002770c0__generated, script_list_is_current_skippable, 60, &|c, seed| {
            machine(c, seed, 0x2770_c0, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            w(c, DATA + 0xa4, if seed % 2 == 0 { 3 } else { 0 });
        });
    }

    #[test]
    fn script_engine_clear_locator_table_matches() {
        check(ssx3_ee::s07::f_00278a58__generated, script_engine_clear_locator_table, 60, &|c, seed| {
            machine(c, seed, 0x278a_58, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            if seed % 2 == 0 {
                w(c, GAME_GP - 0x848, DATA + 0x200);
                w(c, DATA + 0x200 + 0x84, DATA + 0x300);
                w(c, DATA + 0x300 + 0x78, seed % 8);
            }
        });
    }

    #[test]
    fn script_engine_list_add_movie_matches() {
        check_lohi(ssx3_ee::s07::f_00278e90__generated, script_engine_list_add_movie, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x278e_90, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            w(c, DATA + 0x54c, DATA + 0x1000);
            c.g[A1][0] = (seed % 3) as u64;
            c.g[A2][0] = r.next_u32() as u64;
            c.g[A3][0] = r.next_u32() as u64;
            c.g[T0][0] = r.next_u32() as u64;
            c.g[T1][0] = r.next_u32() as u64;
            c.g[T2][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn wscript_cache_set_data_matches() {
        check(ssx3_ee::s13::f_003acc50__generated, wscript_cache_set_data, SEEDS, &|c, seed| {
            machine(c, seed, 0x3acc_50, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 4) as u64;
            c.g[A2][0] = (if seed % 2 == 0 { 0 } else { DATA + 0x1000 }) as u64;
        });
    }

    // --------------------------------------------------------------- sound

    #[test]
    fn ssx_audio_system_get_preset_gain_matches() {
        check(ssx3_ee::s07::f_00288370__generated, ssx_audio_system_get_preset_gain, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x2883_70, 0);
            belib(c);
            c.g[A0][0] = DATA as u64;
            let index = seed % 4;
            c.g[A1][0] = index as u64;
            w(c, GP_DB0, fb(0.01));
            for i in 0..4u32 {
                w(c, DATA + 0x6440 + i * 4, r.edge_f32());
                w(c, DATA + 0x63e4 + i * 4, r.edge_f32());
            }
            match seed % 3 {
                0 => c.g[A2][0] = 0,
                1 => c.g[A2][0] = (r.next_u32() % 1000) as u64,
                _ => {
                    c.g[A2][0] = sx(0xffff_ffff);
                    w(c, DATA + 0x6410, (seed % 2) as u32);
                }
            }
        });
    }

    #[test]
    fn crowd_audio_instance_stop_anticipate_matches() {
        check(ssx3_ee::s08::f_002a6808__generated, crowd_audio_instance_stop_anticipate, SEEDS, &|c, seed| {
            machine(c, seed, 0x2a68_08, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            w(c, DATA + 0x180, if seed % 2 == 0 { 2 } else { 0 });
        });
    }

    #[test]
    fn path_plat_bank_slot_init_matches() {
        check(ssx3_ee::s08::f_002ae1f0__generated, path_plat_bank_slot_init, SEEDS, &|c, seed| {
            machine(c, seed, 0x2ae1_f0, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn music_player_get_duck_to_loops_matches() {
        check(ssx3_ee::s08::f_002b4878__generated, music_player_get_duck_to_loops, SEEDS, &|c, seed| {
            machine(c, seed, 0x2b48_78, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            match seed % 3 {
                0 => {}
                1 => {
                    w(c, DATA + 0x418, 1);
                }
                _ => {
                    w(c, DATA + 0x418, 1);
                    w(c, DATA + 0x408, DATA + 0x1000);
                }
            }
        });
    }

    #[test]
    fn world_trigger_is_type5_matches() {
        check(ssx3_ee::s08::f_002b5838__generated, world_trigger_is_type5, 60, &|c, seed| {
            machine(c, seed, 0x2b58_38, 1);
            belib(c);
            c.g[A1][0] = (seed % 6) as u64;
            if seed % 2 == 0 {
                w(c, GAME_GP + 0x21e4, DATA + 0x200);
            }
        });
    }

    #[test]
    fn world_sound_ctor_matches() {
        check(ssx3_ee::s08::f_002b6aa0__generated, world_sound_ctor, SEEDS, &|c, seed| {
            machine(c, seed, 0x2b6a_a0, 0);
            belib(c);
            c.g[A0][0] = DATA as u64;
            put_vec(c, 0x004f_f120, [seed | 0x1000, 2, 3, 4]);
        });
    }

    // --------------------------------------------------- painter and strings

    #[test]
    fn world_painter_query_reset_list_matches() {
        check(ssx3_ee::s08::f_002c03a8__generated, world_painter_query_reset_list, SEEDS, &|c, seed| {
            machine(c, seed, 0x2c03_a8, 0);
            belib(c);
            w(c, GP_NEG99999, fb(-99999.0));
            match seed % 3 {
                0 => {}
                1 => {
                    w(c, GAME_GP + 0x790, DATA);
                    w(c, DATA, DATA + 0x1000);
                    w(c, DATA + 0x14, 0);
                }
                _ => {
                    w(c, GAME_GP + 0x790, DATA);
                    w(c, DATA, DATA + 0x1000);
                    w(c, DATA + 0x14, DATA + 0x20);
                    w(c, DATA + 0x20, DATA + 0x1010);
                    w(c, DATA + 0x34, 0);
                }
            }
        });
    }

    #[test]
    fn ustr_from_ascii_2_matches() {
        check(ssx3_ee::s08::f_002c2540__generated, ustr_from_ascii_2, SEEDS, &|c, seed| {
            machine(c, seed, 0x2c25_40, 0);
            belib(c);
            c.g[A0][0] = (DATA + 0x200) as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            let text: &[u8] = match seed % 4 {
                0 => &b""[..],
                1 => &b"Hello"[..],
                2 => &[0x41, 0xc3, 0x80, 0x7f][..],
                _ => &[0x80, 0x81, 0xff][..],
            };
            for (i, &ch) in text.iter().enumerate() {
                b(c, DATA + 0x100 + i as u32, ch);
            }
            b(c, DATA + 0x100 + text.len() as u32, 0);
        });
    }

    #[test]
    fn ustr_copy_n_matches() {
        check(ssx3_ee::s08::f_002c25b8__generated, ustr_copy_n, SEEDS, &|c, seed| {
            machine(c, seed, 0x2c25_b8, 0);
            belib(c);
            c.g[A0][0] = (DATA + 0x200) as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.g[A2][0] = match seed % 3 {
                0 => 3,
                1 => 0,
                _ => sx(0xffff_ffff),
            };
            let text: &[u16] = match seed % 4 {
                0 => &[][..],
                1 => &[0x41, 0x42, 0x43][..],
                2 => &[0x1234, 0x8000][..],
                _ => &[0x41][..],
            };
            for (i, &ch) in text.iter().enumerate() {
                h(c, DATA + 0x100 + 2 * i as u32, ch);
            }
            h(c, DATA + 0x100 + 2 * text.len() as u32, 0);
        });
    }

    // ------------------------------------------------- visual fx and memory

    #[test]
    fn cut_scene_transition_man_force_off_matches() {
        check(ssx3_ee::s09::f_002ea820__generated, cut_scene_transition_man_force_off, SEEDS, &|c, seed| {
            machine(c, seed, 0x2ea8_20, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            w(c, DATA + 0x10, if seed % 2 == 0 { 0 } else { 1 });
            if seed % 4 == 3 {
                w(c, GAME_GP + 0x2a90, DATA + 0x200);
            }
        });
    }

    #[test]
    fn mem_get_block_size_matches() {
        check(ssx3_ee::s10::f_00319b98__generated, mem_get_block_size, SEEDS, &|c, seed| {
            machine(c, seed, 0x319b_98, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            if seed % 2 == 0 {
                w(c, GAME_GP + 0x2a70, DATA);
            }
        });
    }

    #[test]
    fn mem_report_stats_locked_matches() {
        check(ssx3_ee::s10::f_00319cc8__generated, mem_report_stats_locked, 60, &|c, seed| {
            machine(c, seed, 0x319c_c8, 3);
            belib(c);
        });
    }

    #[test]
    fn pad_queue_init_matches() {
        check(ssx3_ee::s10::f_00326a28__generated, pad_queue_init, SEEDS, &|c, seed| {
            machine(c, seed, 0x326a_28, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn anim_delta_v36_matches() {
        check(ssx3_ee::s11::f_003593d0__generated, anim_delta_v36, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3593_d0, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = if seed % 2 == 0 { 0x8c } else { 0x10 };
            c.f[12] = r.edge_f32();
            w(c, DATA + 0x6c, r.edge_f32());
            w(c, GP_1_30, fb(1.0 / 30.0));
        });
    }

    #[test]
    fn rail_modifier_list_push_back_matches() {
        check(ssx3_ee::s11::f_0035b690__generated, rail_modifier_list_push_back, SEEDS, &|c, seed| {
            machine(c, seed, 0x35b6_90, 0);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            match seed % 3 {
                0 => {}
                1 => {
                    w(c, DATA, DATA + 0x20);
                    w(c, DATA + 0x20, 0);
                }
                _ => {
                    w(c, DATA, DATA + 0x20);
                    w(c, DATA + 0x20, DATA + 0x40);
                    w(c, DATA + 0x40, 0);
                }
            }
        });
    }

    #[test]
    fn ps2_graphics_man_free_blended_matrix_history_matches() {
        check(ssx3_ee::s12::f_003698a0__generated, ps2_graphics_man_free_blended_matrix_history, SEEDS, &|c, seed| {
            machine(c, seed, 0x3698_a0, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            let buffer = if seed % 2 == 0 { 0 } else { DATA + 0x1000 };
            w(c, DATA + 0xf48, buffer);
        });
    }

    #[test]
    fn emitter_ctor_matches() {
        check(ssx3_ee::s12::f_003714b8__generated, emitter_ctor, 60, &|c, seed| {
            machine(c, seed, 0x3714_b8, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn ui_text_scroll_replace_wide_string_matches() {
        check(ssx3_ee::s13::f_003a1cf0__generated, ui_text_scroll_replace_wide_string, 60, &|c, seed| {
            machine(c, seed, 0x3a1c_f0, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x400) as u64;
            match seed % 3 {
                0 => {}
                1 => h(c, DATA + 0x400, 0x41),
                _ => {
                    h(c, DATA + 0x400, 0x41);
                    h(c, DATA + 0x402, 0x42);
                    h(c, DATA + 0x404, 0);
                }
            }
        });
    }

    #[test]
    fn stream_man_init_matches() {
        check(ssx3_ee::s13::f_003a6e98__generated, stream_man_init, SEEDS, &|c, seed| {
            machine(c, seed, 0x3a6e_98, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.g[A2][0] = 0x1234;
            let name = b"stream";
            for (i, &ch) in name.iter().enumerate() {
                b(c, DATA + 0x100 + i as u32, ch);
            }
            b(c, DATA + 0x100 + name.len() as u32, 0);
        });
    }

    #[test]
    fn ps2_chunk_resolve_v14_matches() {
        check(ssx3_ee::s13::f_003aa5c8__generated, ps2_chunk_resolve_v14, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3aa5_c8, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
            c.g[A2][0] = (seed % 3) as u64;
            w(c, DATA + 0x200 + 8, (seed as u32) << 4);
            if seed % 2 == 0 {
                let vt = vtable(c, &mut r, &[0x78]);
                w(c, DATA + 8, vt);
            }
        });
    }
}
