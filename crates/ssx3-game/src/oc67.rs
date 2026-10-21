//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for small BE/UI/camera/audio/script helpers, each with a
//! differential test against the generated body.
//!
//! Mostly field accessors and thin wrappers: the BE mission/reward tables
//! (mission list at `0x42ee10`, 0x24-byte entries), the FE screen/state
//! boilerplate (ctors store their vtable, Hide/ShowWidget forward to a
//! virtual method), the GM save/load handlers (virtual `save`/`load` methods
//! on a handler object), the InSeq track class constructors, and the audio
//! facade (channel volumes, music stop, bank memory). Calls follow
//! [`crate::call`]: the same stack frame, the same registers at each call,
//! the unwind check after every call.

use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0015_1178, reward_get_collect_item_points, "BERewardInt_GetCollectItemPoints"),
    (0x0015_11b0, reward_get_mission_points, "BERewardInt_GetMissionPoints"),
    (0x0015_3fa0, mission_get_field18, "BEMissionInt_GetMissionField18"),
    (0x0015_3fd8, mission_get_prereq, "BEMissionInt_GetMissionPrereq"),
    (0x0015_47a0, mission_get_field22, "BEMissionInt_GetMissionField22"),
    (0x0015_a320, reward_test_granted_bit, "BERewardInt_TestGrantedBit"),
    (0x0015_a5f0, reward_new_rewards_test_bit, "BERewardInt_NewRewardsTestBit"),
    (0x0016_2170, chase_cam_reset_to_default, "ChaseCamController_ResetToDefault"),
    (0x0016_2258, chase_cam_start_post_race_camera, "ChaseCamController_StartPostRaceCamera"),
    (0x0016_c6e0, active_camera_trigger_list_find_by_id, "ActiveCameraTriggerList_FindById"),
    (0x0019_9420, fe_char_equip_on_deactivate, "cFEStateCharEquip_OnDeactivate"),
    (0x001a_bf70, fe_screen_blank_ctor, "FEScreenBlank_ctor"),
    (0x001b_d428, fe_lobby_menu_on_keyboard_done, "cFEStateLobbyMenu_OnKeyboardDone"),
    (0x001c_55f8, fe_popup_confirm_cfg_ctor, "cFEPopupConfirm_CfgCtor"),
    (0x001c_a488, fe_popup_confirm_hide_widget, "cFEPopupConfirm_HideWidget"),
    (0x001c_a4c0, fe_popup_confirm_show_widget, "cFEPopupConfirm_ShowWidget"),
    (0x001c_f238, fe_reward_gallery_update, "cFEStateRewardGallery_Update"),
    (0x001e_39f8, message_center_get_inbox, "MessageCenter_GetInbox"),
    (0x001e_4840, ov_message_center_hide_widget, "OVScreenMessageCenter_HideWidget"),
    (0x001e_4878, ov_message_center_show_widget, "OVScreenMessageCenter_ShowWidget"),
    (0x001f_7738, ov_bcfail_pop_event_arg, "OVScreenBCFail_PopEventArg"),
    (0x001f_d118, ov_multi_freestyle_pre_on_deactivate, "OVScreenMultiFreestylePre_OnDeactivate"),
    (0x0021_05d0, race_progress_set_path_data, "RaceProgress_SetPathData"),
    (0x0023_6b38, game_state_reset_ai_exit, "GameStateResetAI_Exit"),
    (0x0023_9d18, gm_handler0_save_state, "GMHandler0_SaveState"),
    (0x0023_9d50, gm_handler0_load_state, "GMHandler0_LoadState"),
    (0x0023_ac18, gm_handler1_save_state, "GMHandler1_SaveState"),
    (0x0023_ac50, gm_handler1_load_state, "GMHandler1_LoadState"),
    (0x0023_b038, gm_handler8_save_state, "GMHandler8_SaveState"),
    (0x0023_b070, gm_handler8_load_state, "GMHandler8_LoadState"),
    (0x0023_fb20, mc_manager_request_card_info, "MCManager_RequestCardInfo"),
    (0x0023_fd08, mc_manager_state26, "MCManager_State26"),
    (0x0024_24c8, mc_manager_dtor, "MCManager_dtor"),
    (0x0026_2400, buddy_man_is_temp_at, "BuddyMan_IsTempAt"),
    (0x0026_5ec8, shared_login_do_delete, "cSharedLoginMan_DoDelete"),
    (0x0026_ee68, replay_enable_replay_cameras, "Replay_EnableReplayCameras"),
    (0x0027_2288, replay_highlight_slot_init, "Replay_HighlightSlotInit"),
    (0x0027_25b8, in_seq_curve_track_ctor, "InSeq_CurveTrack_ctor"),
    (0x0027_2d68, in_seq_span_track_ctor, "InSeq_SpanTrack_ctor"),
    (0x0027_3180, in_seq_event_track_ctor, "InSeq_EventTrack_ctor"),
    (0x0027_34a8, in_seq_ramp_track_ctor, "InSeq_RampTrack_ctor"),
    (0x0027_8a98, script_engine_stop_all, "ScriptEngine_StopAll"),
    (0x0028_3000, script_binding_unbind, "ScriptBinding_Unbind"),
    (0x0028_5f48, audio_stop_all_sounds, "SSXAudioSystem_StopAllSounds"),
    (0x0028_7520, audio_set_speech_volume, "SSXAudioSystem_SetSpeechVolume"),
    (0x0028_bda8, snd_bank_man_set_bank_memory, "SndBankMan_SetBankMemory"),
    (0x0028_f520, audio_stop_fe_music, "SSXAudioSystem_StopFEMusic"),
    (0x0029_f128, audio_is_pa_speech_on, "SSXAudioSystem_IsPASpeechOn"),
    (0x0029_f160, audio_is_arcade_speech_on, "SSXAudioSystem_IsArcadeSpeechOn"),
    (0x002a_4ba8, crowd_audio_init_sample_list, "CrowdAudio_InitSampleList"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

mod callee {
    use super::{ee, Callee};

    pub const FIND_MISSION: Callee =
        Callee { addr: 0x0015_4240, f: ee::s01::BEMissionInt_FindMission };
    pub const CHASE_CAM_RESET_STATE: Callee =
        Callee { addr: 0x0016_2138, f: ee::s01::ChaseCamController_ResetState };
    pub const CHASE_CAM_SET_ALGORITHM_IMMEDIATE: Callee =
        Callee { addr: 0x0015_d050, f: ee::s01::ChaseCamController_SetAlgorithmImmediate };
    pub const CHASE_CAM_SET_ALGORITHM: Callee =
        Callee { addr: 0x0015_d078, f: ee::s01::ChaseCamController_SetAlgorithm };
    pub const GET_RIDER_PREVIEW: Callee =
        Callee { addr: 0x001a_0548, f: ee::s03::FEModule_GetRiderPreview };
    pub const RIDER_PREVIEW_SET_VISIBLE: Callee =
        Callee { addr: 0x0019_e538, f: ee::s03::FEModule_RiderPreviewSetVisible };
    pub const NET_FE_STATE_CTOR: Callee =
        Callee { addr: 0x001a_8500, f: ee::s03::NetFEState_ctor };
    pub const LOBBY_MENU_REQUEST_JOIN_ROOM: Callee =
        Callee { addr: 0x001b_d540, f: ee::s03::cFEStateLobbyMenu_RequestJoinRoom };
    pub const POPUP_CONFIRM_CFG_RESET: Callee =
        Callee { addr: 0x001c_5630, f: ee::s03::cFEPopupConfirm_CfgReset };
    pub const REWARD_GALLERY_CHECK_REWARDS_READY: Callee =
        Callee { addr: 0x001d_0238, f: ee::s04::cFEStateRewardGallery_CheckRewardsReady };
    pub const REWARD_GALLERY_POLL_SHAPE_LOAD: Callee =
        Callee { addr: 0x001c_ff10, f: ee::s04::cFEStateRewardGallery_PollShapeLoad };
    pub const BUDDY_MAN_GET_ROSTER_ENTRY: Callee =
        Callee { addr: 0x0026_25c8, f: ee::s07::BuddyMan_GetRosterEntry };
    pub const REPLAY_APPLY_CAMERA: Callee =
        Callee { addr: 0x0026_edd8, f: ee::s07::Replay_ApplyCamera };
    pub const RACE_PROGRESS_RESET_ALL: Callee =
        Callee { addr: 0x0021_08f8, f: ee::s05::RaceProgress_ResetAll };
    pub const MC_MANAGER_END_OPERATION: Callee =
        Callee { addr: 0x0023_ea30, f: ee::s06::MCManager_EndOperation };
    pub const SCRIPT_ENGINE_STOP_MAIN_LISTS: Callee =
        Callee { addr: 0x0027_90f8, f: ee::s07::ScriptEngine_StopMainLists };
    pub const SCRIPT_ENGINE_STOP_AUX_LISTS: Callee =
        Callee { addr: 0x0027_9488, f: ee::s07::ScriptEngine_StopAuxLists };
    pub const SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS: Callee =
        Callee { addr: 0x0028_2390, f: ee::s07::ScriptEngine_StopAllProcessSeqs };
    pub const SCRIPT_ENGINE_GET_INSTANCE: Callee =
        Callee { addr: 0x0027_d2e8, f: ee::s07::ScriptEngine_GetInstance };
    pub const SCRIPT_ENGINE_DEACTIVATE_BINDING: Callee =
        Callee { addr: 0x0027_d4a0, f: ee::s07::ScriptEngine_DeactivateBinding };
    pub const CROWD_AUDIO_STOP_ALL: Callee =
        Callee { addr: 0x002a_7718, f: ee::s08::CrowdAudio_StopAll };
    pub const BANK_MONITOR_STOP_ALL: Callee =
        Callee { addr: 0x002a_d3c0, f: ee::s08::BankMonitor_StopAll };
    pub const AUDIO_LEVEL_TO_VOLUME: Callee =
        Callee { addr: 0x0028_73d8, f: ee::s07::SSXAudioSystem_LevelToVolume };
    pub const AUDIO_SET_CHANNEL_VOLUME: Callee =
        Callee { addr: 0x0028_7700, f: ee::s07::SSXAudioSystem_SetChannelVolume };
    pub const SND_BANK_SET_MEMORY: Callee =
        Callee { addr: 0x0028_b588, f: ee::s07::SndBank_SetMemory };
    pub const AUDIO_IS_GAME_MUSIC_ENABLED: Callee =
        Callee { addr: 0x0028_d960, f: ee::s07::SSXAudioSystem_IsGameMusicEnabled };
    pub const MUSIC_PLAYER_STOP_SONG: Callee =
        Callee { addr: 0x002b_3ac0, f: ee::s08::MusicPlayer_StopSong };

    // Indirect calls the generated code keeps as raw addresses.
    pub const UI_STATE_UPDATE: u32 = 0x0039_e510;
    pub const BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;
    pub const BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;
    pub const BE_NEW_PLAYER_GET_INBOX: u32 = 0x0014_7908;
    pub const WSCRIPT_MAN_EVENT_QUEUE_POP: u32 = 0x0030_c820;
    pub const AUDIO_GET_INSTANCE: u32 = 0x0028_b180;
    pub const AUDIO_START_PRERACE_MUSIC: u32 = 0x0028_72a8;
    pub const SLOGIN_DELETE: u32 = 0x003f_80c0;
    pub const CAMERA_TRIGGER_MAN_UPDATE_MASK1: u32 = 0x0016_d1d8;
    pub const EA_MEMSET: u32 = 0x003e_6448;
    pub const AUDIO_GET_CHANNEL_VOLUME: u32 = 0x0028_7920;
}

/// BE mission table: 88 × 0x24-byte `BEMissionInt` records.
const MISSION_TABLE: u32 = 0x0044_0000u32.wrapping_sub(0x11f0);

// ---------------------------------------------------------------------------
// BE rewards.

/// `int BERewardInt_GetCollectItemPoints(BERewardInt* self, int level)`
/// (0x151178): 1000 points for level 1, 500 below that, 2000 for level 2.
pub fn reward_get_collect_item_points(c: &mut Cpu) {
    c.cycles += 3;
    let level = c.g[A1][0];
    c.g[V0][0] = 1;
    let is_one = level == c.g[V0][0];
    c.g[V0][0] = ((level as i64) < 2) as u64;
    if is_one {
        c.cycles += 2;
        c.g[V0][0] = 0x3e8;
        crate::abi::ret(c);
        return;
    }
    c.cycles += 2;
    let below_two = c.g[V0][0] != 0;
    c.g[V0][0] = 2;
    if !below_two {
        c.cycles += 2;
        if level == c.g[V0][0] {
            c.cycles += 2;
            c.g[V0][0] = 0x7d0;
            crate::abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0x1f4;
    crate::abi::ret(c);
}

/// `int BERewardInt_GetMissionPoints(BERewardInt* self, int level)`
/// (0x1511b0): 4000 for level 2, 2000 below that, 6000 for level 3.
pub fn reward_get_mission_points(c: &mut Cpu) {
    c.cycles += 3;
    let level = c.g[A1][0];
    c.g[V0][0] = 2;
    let is_two = level == c.g[V0][0];
    c.g[V0][0] = ((level as i64) < 3) as u64;
    if is_two {
        c.cycles += 2;
        c.g[V0][0] = 0xfa0;
        crate::abi::ret(c);
        return;
    }
    c.cycles += 2;
    let below_three = c.g[V0][0] != 0;
    c.g[V0][0] = 3;
    if !below_three {
        c.cycles += 2;
        if level == c.g[V0][0] {
            c.cycles += 2;
            c.g[V0][0] = 0x1770;
            crate::abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0x7d0;
    crate::abi::ret(c);
}

/// `int16 BEMissionInt_GetMissionField18(BEMissionInt* missions, int id)`
/// (0x153fa0): the signed halfword at +0x18 of mission `id`.
pub fn mission_get_field18(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::FIND_MISSION.call(c, 0x0015_3fb0)?;
        c.g[A0][0] = 0x24;
        c.g[V1][0] = 0x0044_0000;
        c.mult(0, V0, V0, A0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x11f0));
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x18)).lh(c);
        frame.close(c);
        Ok(())
    });
}

/// `uint32 BEMissionInt_GetMissionPrereq(BEMissionInt* missions, int id)`
/// (0x153fd8): the word at +0x1c of mission `id` (the prerequisite id).
pub fn mission_get_prereq(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::FIND_MISSION.call(c, 0x0015_3fe8)?;
        c.g[A0][0] = 0x24;
        c.g[V1][0] = 0x0044_0000;
        c.mult(0, V0, V0, A0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x11f0));
        frame.restore_ra(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
        frame.close(c);
        Ok(())
    });
}

/// `int16 BEMissionInt_GetMissionField22(BEMissionInt* missions, int id)`
/// (0x1547a0): linear search over the 88 mission records, halfword at +0x22.
pub fn mission_get_field22(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x0044_0000;
    c.g[V1][0] = sx(MISSION_TABLE);
    c.g[A0][0] = sx(MISSION_TABLE.wrapping_add(0xc60));
    c.g[V0][0] = Addr(MISSION_TABLE).lw(c);
    let mut cur = MISSION_TABLE;
    loop {
        c.cycles += 2;
        if c.g[V0][0] != c.g[A1][0] {
            cur = cur.wrapping_add(0x24);
            c.g[V1][0] = sx(cur);
            c.cycles += 3;
            let in_table = (sx(cur) as i64) < (c.g[A0][0] as i64);
            c.g[V0][0] = in_table as u64;
            if !in_table {
                c.cycles += 2;
                c.g[V0][0] = 0;
                crate::abi::ret(c);
                return;
            }
            c.g[V0][0] = Addr(cur).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0015_47b0);
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = Addr(cur.wrapping_add(0x22)).lh(c);
            crate::abi::ret(c);
            return;
        }
    }
}

/// `bool BERewardInt_TestGrantedBit(BERewardInt* self, int bit,
/// uint8_t* granted)` (0x15a320): bit `bit` of the byte array, with the
/// two's-complement rounding for negative indices.
fn test_bit(c: &mut Cpu, index: u64, base: u64) {
    c.g[V0][0] = u64::MAX;
    c.g[V1][0] = sx((index as u32).wrapping_add(7));
    c.g[V0][0] = ((u64::MAX as i64) < (index as i64)) as u64;
    c.g[A0][0] = sx((index as u32) >> 3);
    if c.g[V0][0] != 0 {
        c.g[V1][0] = index;
    }
    c.g[A2][0] = sx((base as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = sx((((c.g[V1][0] as u32) as i32) >> 3) as u32);
    c.g[V0][0] = Addr(c.g[A2][0] as u32).lbu(c);
    c.g[A0][0] = sx((c.g[V1][0] as u32) << 3);
    c.g[V1][0] = sx((index as u32).wrapping_sub(c.g[A0][0] as u32));
    c.g[V0][0] =
        sx((((c.g[V0][0] as u32) as i32) >> ((c.g[V1][0] as u32) & 31)) as u32);
    c.g[V0][0] &= 1;
}

/// `bool BERewardInt_TestGrantedBit(BERewardInt* self, int bit,
/// const uint8_t* bits)` (0x15a320).
pub fn reward_test_granted_bit(c: &mut Cpu) {
    c.cycles += 13;
    let (index, base) = (c.g[A1][0], c.g[A2][0]);
    test_bit(c, index, base);
    crate::abi::ret(c);
}

/// `bool BERewardInt_NewRewardsTestBit(BERewardInt* self, int bit,
/// const uint8_t* bits)` (0x15a5f0): identical bit test on the new-reward
/// bitset.
pub fn reward_new_rewards_test_bit(c: &mut Cpu) {
    c.cycles += 13;
    let (index, base) = (c.g[A1][0], c.g[A2][0]);
    test_bit(c, index, base);
    crate::abi::ret(c);
}

// ---------------------------------------------------------------------------
// Camera.

/// `void ChaseCamController_ResetToDefault(ChaseCamController* cam)`
/// (0x162170): reset the state and select the default camera algorithm.
pub fn chase_cam_reset_to_default(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::CHASE_CAM_RESET_STATE.call(c, 0x0016_2184)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0;
        c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x24)).lw(c);
        callee::CHASE_CAM_SET_ALGORITHM_IMMEDIATE.call(c, 0x0016_2194)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void ChaseCamController_StartPostRaceCamera(ChaseCamController* cam)`
/// (0x162258): switch to the post-race algorithm (id 0x44), unless the
/// camera is already in replay mode (bit 0 of +0x30).
pub fn chase_cam_start_post_race_camera(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let flags = Addr((c.g[A0][0] as u32).wrapping_add(0x30)).lw(c);
        c.g[V0][0] = flags & 1;
        let replaying = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if !replaying {
            c.cycles += 5;
            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x6b10));
            c.g[A1][0] = 0x44;
            c.g[A2][0] = 0;
            callee::CHASE_CAM_SET_ALGORITHM.call(c, 0x0016_2280)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `ActiveCameraTrigger* ActiveCameraTriggerList_FindById(list, int id)`
/// (0x16c6e0): walk the array of trigger pointers, return the node whose
/// first word is `id`.
pub fn active_camera_trigger_list_find_by_id(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A0][0] = Addr(c.g[A0][0] as u32).lw(c);
    if c.g[A0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        crate::abi::ret(c);
        return;
    }
    c.cycles += 1;
    c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        if c.g[V0][0] != c.g[A1][0] {
            c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(4)).lw(c);
            c.cycles += 2;
            if c.g[A0][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = 0;
                crate::abi::ret(c);
                return;
            }
            c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0016_c6f0);
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.g[V1][0];
            crate::abi::ret(c);
            return;
        }
    }
}

// ---------------------------------------------------------------------------
// Front-end screens.

/// `void cFEStateCharEquip_OnDeactivate(cFEStateCharEquip* self)`
/// (0x199420): hide the rider preview of the character selected in `+0x44`.
pub fn fe_char_equip_on_deactivate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        let world = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        frame.save_ra(c, 0);
        c.g[V0][0] = world;
        c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x44)).lb(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x7c)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
        callee::GET_RIDER_PREVIEW.call(c, 0x0019_943c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        callee::RIDER_PREVIEW_SET_VISIBLE.call(c, 0x0019_9448)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `FEScreenBlank* FEScreenBlank_ctor(FEScreenBlank* self)` (0x1abf70):
/// construct the net FE state and install the FEScreenBlank vtable.
pub fn fe_screen_blank_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::NET_FE_STATE_CTOR.call(c, 0x001a_bf84)?;
        c.g[V1][0] = 0x0047_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4908));
        frame.restore_ra(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(8)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `void cFEStateLobbyMenu_OnKeyboardDone(cFEStateLobbyMenu* self)`
/// (0x1bd428): when the pending room request is in state 8, join the room
/// with the typed name and room slot.
pub fn fe_lobby_menu_on_keyboard_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 8;
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x69c)).lw(c);
        c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x18)).lw(c);
        let pending = c.g[V0][0] != c.g[V1][0];
        frame.restore_ra(c, 0);
        if !pending {
            c.cycles += 4;
            c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xb80)).lw(c);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x74));
            callee::LOBBY_MENU_REQUEST_JOIN_ROOM.call(c, 0x001b_d450)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `cFEPopupConfirm* cFEPopupConfirm_CfgCtor(cFEPopupConfirm* self)`
/// (0x1c55f8): install the vtable and reset the configuration fields.
pub fn fe_popup_confirm_cfg_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x0047_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3358));
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0xc8)).sw(c, c.g[V0][0]);
        callee::POPUP_CONFIRM_CFG_RESET.call(c, 0x001c_5618)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `void cFEPopupConfirm_HideWidget(cFEPopupConfirm* self, Widget* w)`
/// (0x1ca488): forward `Show(false)` to the widget's virtual method
/// (vtable slot +0x4c, receiver offset at +0x48).
pub fn fe_popup_confirm_hide_widget(c: &mut Cpu) {
    hide_show_widget(c, 0, 0x001c_a4b0);
}

/// `void cFEPopupConfirm_ShowWidget(cFEPopupConfirm* self, Widget* w)`
/// (0x1ca4c0): forward `Show(true)` to the widget's virtual method.
pub fn fe_popup_confirm_show_widget(c: &mut Cpu) {
    hide_show_widget(c, 1, 0x001c_a4e8);
}

/// The shared body of the `*_HideWidget` / `*_ShowWidget` forwarders: call
/// the widget's virtual `Show(visible)` (slot +0x4c, `this` adjusted by the
/// signed halfword at +0x48).
fn hide_show_widget(c: &mut Cpu, visible: u64, ret: u32) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A1][0];
        let null = c.g[A2][0] == 0;
        frame.save_ra(c, 0);
        if !null {
            c.cycles += 6;
            c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(8)).lw(c);
            c.g[A1][0] = visible;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, ret)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cFEStateRewardGallery_Update(cFEStateRewardGallery* self)`
/// (0x1cf238): poll the reward readiness / shape loads and run the base UI
/// state update.
pub fn fe_reward_gallery_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::REWARD_GALLERY_CHECK_REWARDS_READY.call(c, 0x001c_f24c)?;
        c.g[A0][0] = c.g[S0][0];
        callee::REWARD_GALLERY_POLL_SHAPE_LOAD.call(c, 0x001c_f254)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::UI_STATE_UPDATE, 0x001c_f25c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void MessageCenter_GetInbox(Inbox* self)` (0x1e39f8): the inbox of the
/// local player's profile (`BELibrary` interface 1, slot 0).
pub fn message_center_get_inbox(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE, 0x001e_3a08)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x001e_3a14)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, callee::BE_NEW_PLAYER_GET_INBOX, 0x001e_3a20)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void OVScreenMessageCenter_HideWidget(OVScreenMessageCenter* self,
/// Widget* w)` (0x1e4840).
pub fn ov_message_center_hide_widget(c: &mut Cpu) {
    hide_show_widget(c, 0, 0x001e_4868);
}

/// `void OVScreenMessageCenter_ShowWidget(OVScreenMessageCenter* self,
/// Widget* w)` (0x1e4878).
pub fn ov_message_center_show_widget(c: &mut Cpu) {
    hide_show_widget(c, 1, 0x001e_48a0);
}

/// `void OVScreenBCFail_PopEventArg(OVScreenBCFail* self)` (0x1f7738):
/// pop one pending script event and stash its argument at +0x9c.
pub fn ov_bcfail_pop_event_arg(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xce8)).lw(c);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1c4));
        call_ptr(c, callee::WSCRIPT_MAN_EVENT_QUEUE_POP, 0x001f_7754)?;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        frame.restore_ra(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(0x9c)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `void OVScreenMultiFreestylePre_OnDeactivate(OVScreen* self)`
/// (0x1fd118): mark the freestyle screen inactive, then resume the
/// pre-race music.
pub fn ov_multi_freestyle_pre_on_deactivate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
        let none = c.g[V1][0] == 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
            Addr((c.g[V1][0] as u32).wrapping_add(0x64)).sw(c, c.g[V0][0]);
        }
        c.cycles += 7;
        call_ptr(c, callee::AUDIO_GET_INSTANCE, 0x001f_d138)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, callee::AUDIO_START_PRERACE_MUSIC, 0x001f_d140)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Race progress and game state.

/// `void RaceProgress_SetPathData(RaceProgress* self, FileBlock* path)`
/// (0x2105d0): record the path block, its data and the course-name string
/// in the progress globals, then reset all progress.
pub fn race_progress_set_path_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(4)).lw(c);
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        Addr((c.g[GP][0] as u32).wrapping_sub(0x9a0)).sw(c, c.g[A0][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        Addr((c.g[GP][0] as u32).wrapping_sub(0x99c)).sw(c, c.g[V0][0]);
        Addr((c.g[GP][0] as u32).wrapping_sub(0x998)).sw(c, c.g[A0][0]);
        callee::RACE_PROGRESS_RESET_ALL.call(c, 0x0021_05f8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void GameStateResetAI_Exit(GameState* self)` (0x236b38): tell the race
/// manager's AI controller (virtual slot +0x2c) to reset.
pub fn game_state_reset_ai_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x488)).lw(c);
        frame.save_ra(c, 0);
        c.g[A1][0] = 3;
        c.g[A2][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0xcc)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x28)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x2c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_6b60)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// GM save/load handlers: `save`/`load` are virtual methods on the handler
// object at `this`; each handler forwards its own state block.

/// One GM handler forwarder: `fn(this, handler, size)` calls the handler's
/// virtual method at `slot` with `(handler + delta, this + data_off, size)`.
fn gm_handler_forward(c: &mut Cpu, this_off: u32, save_slot: u32, size: u64, ret: u32) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(this_off));
        c.g[A2][0] = size;
        c.g[V1][0] = Addr(c.g[A3][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(save_slot)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(save_slot + 4)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void GMHandler0_SaveState(GMHandler0* self, Handler* h)` (0x239d18).
pub fn gm_handler0_save_state(c: &mut Cpu) {
    gm_handler_forward(c, 4, 0x08, 0xa0, 0x0023_9d40);
}

/// `void GMHandler0_LoadState(GMHandler0* self, Handler* h)` (0x239d50).
pub fn gm_handler0_load_state(c: &mut Cpu) {
    gm_handler_forward(c, 4, 0x10, 0xa0, 0x0023_9d78);
}

/// `void GMHandler1_SaveState(GMHandler1* self, Handler* h)` (0x23ac18).
pub fn gm_handler1_save_state(c: &mut Cpu) {
    gm_handler_forward(c, 0, 0x08, 0xd0, 0x0023_ac40);
}

/// `void GMHandler1_LoadState(GMHandler1* self, Handler* h)` (0x23ac50).
pub fn gm_handler1_load_state(c: &mut Cpu) {
    gm_handler_forward(c, 0, 0x10, 0xd0, 0x0023_ac78);
}

/// `void GMHandler8_SaveState(GMHandler8* self, Handler* h)` (0x23b038).
pub fn gm_handler8_save_state(c: &mut Cpu) {
    gm_handler_forward(c, 0, 0x08, 0x2c, 0x0023_b060);
}

/// `void GMHandler8_LoadState(GMHandler8* self, Handler* h)` (0x23b070).
pub fn gm_handler8_load_state(c: &mut Cpu) {
    gm_handler_forward(c, 0, 0x10, 0x2c, 0x0023_b098);
}

// ---------------------------------------------------------------------------
// MC (memory card) manager.

/// `void MCManager_RequestCardInfo(MCManager* self, int slot)` (0x23fb20):
/// remember the slot and ask the card device (virtual slot +0x2c) for its
/// info.
pub fn mc_manager_request_card_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        Addr((c.g[A0][0] as u32).wrapping_add(0x42c)).sw(c, c.g[V1][0]);
        c.g[A2][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x434)).lw(c);
        c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x28)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x2c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_fb48)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void MCManager_State26(MCManager* self)` (0x23fd08): end the current
/// operation, then run the state's virtual completion hook at +0x38.
pub fn mc_manager_state26(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MC_MANAGER_END_OPERATION.call(c, 0x0023_fd20)?;
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x38)).lw(c);
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, 0x0023_fd2c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void MCManager_dtor(MCManager* self, int del)` (0x2424c8): install the
/// dtor vtable, clear the manager global and (when `del & 1`) free `self`.
pub fn mc_manager_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x0048_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3428));
        Addr((c.g[A0][0] as u32).wrapping_add(0x748)).sw(c, c.g[V0][0]);
        c.g[A1][0] &= 1;
        let keep = c.g[A1][0] == 0;
        Addr((c.g[GP][0] as u32).wrapping_sub(0x478)).sw(c, 0);
        if !keep {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e50, 0x0024_24f0)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Buddy list.

/// `bool BuddyMan_IsTempAt(BuddyMan* self, int index)` (0x262400): the
/// "temporary buddy" flag of roster entry `index` (bit 20 of +0xa8).
pub fn buddy_man_is_temp_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::BUDDY_MAN_GET_ROSTER_ENTRY.call(c, 0x0026_2410)?;
        // The generated branches to the epilogue straight from the empty
        // roster case; the shift-and-mask (+2) belongs to the found-entry
        // path only.
        if c.g[V0][0] != 0 {
            c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xa8)).lw(c);
            c.cycles += 2;
            c.g[V0][0] = sx((((c.g[V0][0] as u32) as i32) >> 20) as u32);
            c.g[V0][0] &= 1;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Login manager.

/// `void cSharedLoginMan_DoDelete(cSharedLoginMan* self)` (0x265ec8): clear
/// the pending entry name and kick off the delete operation.
pub fn shared_login_do_delete(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(4)).sw(c, 0);
        Addr((c.g[GP][0] as u32).wrapping_add(0x2f0)).sw(c, 0);
        call_ptr(c, callee::SLOGIN_DELETE, 0x0026_5ee4)?;
        c.g[V1][0] = 0xf;
        frame.restore_ra(c, 0);
        Addr(c.g[S0][0] as u32).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Replay.

/// `void Replay_EnableReplayCameras(Replay* self)` (0x26ee68): refresh the
/// camera triggers, then apply the replay camera.
pub fn replay_enable_replay_cameras(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x004c_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5830));
        call_ptr(c, callee::CAMERA_TRIGGER_MAN_UPDATE_MASK1, 0x0026_ee84)?;
        c.g[A0][0] = c.g[S0][0];
        callee::REPLAY_APPLY_CAMERA.call(c, 0x0026_ee8c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void Replay_HighlightSlotInit(ReplaySlot* slot)` (0x272288): mark the
/// slot empty and clear its 0xac-byte payload.
pub fn replay_highlight_slot_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = u64::MAX;
        Addr(c.g[V0][0] as u32).sw(c, c.g[V1][0]);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
        Addr((c.g[V0][0] as u32).wrapping_add(0xb0)).sw(c, 0);
        c.g[A1][0] = 0;
        c.g[A2][0] = 0xac;
        call_ptr(c, callee::EA_MEMSET, 0x0027_22b0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// InSeq track constructors: install the class vtable at +4 and clear the
// following fields.

/// Shared body: install the class vtable at +4, clear the following fields
/// and return `self` (the ctors are leaves, so `pc` goes to `ra`).
fn in_seq_track_ctor(c: &mut Cpu, vtable: u32, zero_offsets: &[u32]) {
    c.g[V1][0] = 0x0048_0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx(vtable);
    Addr((c.g[A0][0] as u32).wrapping_add(8)).sw(c, 0);
    Addr((c.g[A0][0] as u32).wrapping_add(4)).sw(c, c.g[V1][0]);
    for &off in zero_offsets {
        Addr((c.g[A0][0] as u32).wrapping_add(off)).sw(c, 0);
    }
    crate::abi::ret(c);
}

/// `InSeq_CurveTrack* InSeq_CurveTrack_ctor(InSeq_CurveTrack* self)`
/// (0x2725b8).
pub fn in_seq_curve_track_ctor(c: &mut Cpu) {
    c.cycles += 13;
    in_seq_track_ctor(c, 0x0048_1a10, &[0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24]);
}

/// `InSeq_SpanTrack* InSeq_SpanTrack_ctor(InSeq_SpanTrack* self)`
/// (0x272d68).
pub fn in_seq_span_track_ctor(c: &mut Cpu) {
    c.cycles += 13;
    in_seq_track_ctor(c, 0x0048_19b0, &[0xc, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28]);
}

/// `InSeq_EventTrack* InSeq_EventTrack_ctor(InSeq_EventTrack* self)`
/// (0x273180).
pub fn in_seq_event_track_ctor(c: &mut Cpu) {
    c.cycles += 13;
    in_seq_track_ctor(c, 0x0048_1950, &[0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24]);
}

/// `InSeq_RampTrack* InSeq_RampTrack_ctor(InSeq_RampTrack* self)`
/// (0x2734a8).
pub fn in_seq_ramp_track_ctor(c: &mut Cpu) {
    c.cycles += 13;
    in_seq_track_ctor(c, 0x0048_18f0, &[0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24]);
}

// ---------------------------------------------------------------------------
// Script engine.

/// `void ScriptEngine_StopAll(ScriptEngine* self)` (0x278a98): stop the
/// main and auxiliary script lists and every process sequence.
pub fn script_engine_stop_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_ENGINE_STOP_MAIN_LISTS.call(c, 0x0027_8aac)?;
        c.g[A0][0] = c.g[S0][0];
        callee::SCRIPT_ENGINE_STOP_AUX_LISTS.call(c, 0x0027_8ab4)?;
        c.g[A0][0] = c.g[S0][0];
        callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS.call(c, 0x0027_8abc)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void ScriptBinding_Unbind(ScriptBinding* binding)` (0x283000):
/// deactivate the binding in the script engine and mark it unbound.
pub fn script_binding_unbind(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_ENGINE_GET_INSTANCE.call(c, 0x0028_3014)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        callee::SCRIPT_ENGINE_DEACTIVATE_BINDING.call(c, 0x0028_3020)?;
        Addr((c.g[S0][0] as u32).wrapping_add(8)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Audio system.

/// `void SSXAudioSystem_StopAllSounds(SSXAudioSystem* self)` (0x285f48):
/// stop the crowd and bank-monitor sources, then the sound-system sources.
pub fn audio_stop_all_sounds(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::CROWD_AUDIO_STOP_ALL.call(c, 0x0028_5f5c)?;
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x118)).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
        callee::BANK_MONITOR_STOP_ALL.call(c, 0x0028_5f6c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SSXAudioSystem_SetSpeechVolume(SSXAudioSystem* self, int level)`
/// (0x287520): convert the level to a volume and apply it to speech
/// channel 4.
pub fn audio_set_speech_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::AUDIO_LEVEL_TO_VOLUME.call(c, 0x0028_7534)?;
        c.g[A0][0] = c.g[S0][0];
        c.mov_s(12, 0);
        c.g[A1][0] = 4;
        callee::AUDIO_SET_CHANNEL_VOLUME.call(c, 0x0028_7544)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SndBankMan_SetBankMemory(SndBankMan* self, int bank, void* mem,
/// uint32 flags, uint32 arg)` (0x28bda8): pass the per-bank memory block of
/// the `mem` pointer to `SndBank_SetMemory`.
pub fn snd_bank_man_set_bank_memory(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x60;
        frame.save_ra(c, 0);
        c.mult(0, V1, A1, V0);
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xacc)).lw(c);
        c.g[A2][0] = c.g[A3][0];
        c.g[A3][0] = c.g[T0][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        callee::SND_BANK_SET_MEMORY.call(c, 0x0028_bdd0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SSXAudioSystem_StopFEMusic(SSXAudioSystem* self)` (0x28f520):
/// stop the front-end music player, if game music is enabled.
pub fn audio_stop_fe_music(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::AUDIO_IS_GAME_MUSIC_ENABLED.call(c, 0x0028_f534)?;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x118));
            callee::MUSIC_PLAYER_STOP_SONG.call(c, 0x0028_f544)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `bool SSXAudioSystem_IsPASpeechOn(SSXAudioSystem* self)` (0x29f128).
pub fn audio_is_pa_speech_on(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = 3;
        call_ptr(c, callee::AUDIO_GET_CHANNEL_VOLUME, 0x0029_f138)?;
        c.mtc1(0, 1);
        c.c_eq_s(0, 1);
        let silent = c.fcc();
        c.g[V0][0] = 0;
        if !silent {
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `bool SSXAudioSystem_IsArcadeSpeechOn(SSXAudioSystem* self)` (0x29f160).
pub fn audio_is_arcade_speech_on(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xa;
        call_ptr(c, callee::AUDIO_GET_CHANNEL_VOLUME, 0x0029_f170)?;
        c.mtc1(0, 1);
        c.c_eq_s(0, 1);
        let silent = c.fcc();
        c.g[V0][0] = 0;
        if !silent {
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void CrowdAudio_InitSampleList(CrowdAudio* self)` (0x2a4ba8): clear the
/// sample count and the five sample slots.
pub fn crowd_audio_init_sample_list(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = c.g[A0][0];
    c.g[A1][0] = 4;
    Addr(c.g[V0][0] as u32).sw(c, 0);
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
    loop {
        c.cycles += 7;
        Addr(c.g[V1][0] as u32).sw(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        let more = (c.g[A1][0] as i64) >= 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(4));
        if !more {
            c.cycles += 2;
            crate::abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x002a_4bb8);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, fill, Rng, DATA, GAME_GP};

    const SEEDS: u32 = 200;

    fn rng(seed: u32, salt: u64) -> Rng {
        Rng(0x9e37_79b9_7f4a_7c15 ^ (seed as u64) << 8 ^ salt)
    }

    /// `BELibrary_GetInstance` fast path: the singleton must be non-null,
    /// otherwise it allocates and initialises itself.
    fn be_library(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x3000);
    }

    /// A dummy world: `world -> +0x84 -> race manager -> +0x84 -> riders`
    /// (the rider count at the riders pointer is left 0, so rider loops stop).
    fn world(c: &mut Cpu) {
        let w = DATA + 0x4000;
        c.bus.write32(GAME_GP.wrapping_sub(0x848), w);
        c.bus.write32(w + 0x7c, DATA + 0x5000);
        c.bus.write32(w + 0x84, DATA + 0x6000);
        c.bus.write32(DATA + 0x6000 + 0x84, DATA + 0x7000);
        // No cut-scene transition to force off.
        c.bus.write32(DATA + 0x6000 + 0x64, 0);
    }

    /// The 88 mission records the BE mission accessors search.
    fn mission_table(c: &mut Cpu, r: &mut Rng) {
        for k in 0..88u32 {
            let e = 0x0042_ee10 + 0x24 * k;
            c.bus.write32(e, k * 7);
            c.bus.write32(e + 0x1c, r.next_u32());
            c.bus.write16(e + 0x18, r.next_u32() as u16);
            c.bus.write16(e + 0x22, r.next_u32() as u16);
        }
    }

    /// The engine's size-class pool allocator (`gp + 0xdd8` flag and the pool
    /// heads at `gp + 0x2a60..0x2a84`), so constructors that allocate do not
    /// grow the heap from a zeroed header.
    fn pool_allocator(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x32000),
            (0x2a7c, 0x25800),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        let nodes = DATA + 0x28_000;
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base = nodes + 0x100 * i as u32;
            for n in 0..4u32 {
                let node = base + 0x40 * n;
                c.bus.write32(node, if n == 3 { 0 } else { node + 0x40 });
            }
            c.bus.write32(pool + head, base);
            c.bus.write32(GAME_GP + slot, pool);
        }
    }

    // ------------------------------------------------------------------
    // BE rewards / missions.

    #[test]
    fn reward_get_collect_item_points_matches() {
        check(ssx3_ee::s01::f_00151178__generated, reward_get_collect_item_points, SEEDS, &|c, seed| {
            let levels: [u64; 8] = [0, 1, 2, 3, 9, u64::MAX, 0xffff_ffff, 0x8000_0000];
            c.g[A1][0] = levels[seed as usize % levels.len()];
        });
    }

    #[test]
    fn reward_get_mission_points_matches() {
        check(ssx3_ee::s01::f_001511b0__generated, reward_get_mission_points, SEEDS, &|c, seed| {
            let levels: [u64; 8] = [0, 1, 2, 3, 4, u64::MAX, 0xffff_ffff, 0x8000_0000];
            c.g[A1][0] = levels[seed as usize % levels.len()];
        });
    }

    #[test]
    fn mission_get_field18_matches() {
        check(ssx3_ee::s01::f_00153fa0__generated, mission_get_field18, SEEDS, &|c, seed| {
            let mut r = rng(seed, 0x3fa0);
            mission_table(c, &mut r);
            let ids: [u64; 6] = [0, 7, 609, 616, u64::MAX, 14];
            c.g[A1][0] = ids[seed as usize % ids.len()];
        });
    }

    #[test]
    fn mission_get_prereq_matches() {
        check(ssx3_ee::s01::f_00153fd8__generated, mission_get_prereq, SEEDS, &|c, seed| {
            let mut r = rng(seed, 0x3fd8);
            mission_table(c, &mut r);
            let ids: [u64; 6] = [0, 7, 609, 616, u64::MAX, 21];
            c.g[A1][0] = ids[seed as usize % ids.len()];
        });
    }

    #[test]
    fn mission_get_field22_matches() {
        check(ssx3_ee::s01::f_001547a0__generated, mission_get_field22, SEEDS, &|c, seed| {
            let mut r = rng(seed, 0x47a0);
            mission_table(c, &mut r);
            let ids: [u64; 6] = [0, 35, 609, 616, u64::MAX, 0xffff_ffff];
            c.g[A1][0] = ids[seed as usize % ids.len()];
        });
    }

    fn bit_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        let idx: [u64; 9] = [0, 1, 2, 7, 8, 63, 255, u64::MAX, 0xffff_ffff];
        c.g[A1][0] = idx[seed as usize % idx.len()];
        c.g[A2][0] = sx(DATA + 0x2000);
        for i in 0..64u32 {
            c.bus.write8(DATA + 0x2000 + i, r.next_u32() as u8);
        }
    }

    #[test]
    fn reward_test_granted_bit_matches() {
        check(ssx3_ee::s01::f_0015a320__generated, reward_test_granted_bit, SEEDS, &|c, seed| {
            bit_setup(c, seed, 0xa320);
        });
    }

    #[test]
    fn reward_new_rewards_test_bit_matches() {
        check(ssx3_ee::s01::f_0015a5f0__generated, reward_new_rewards_test_bit, SEEDS, &|c, seed| {
            bit_setup(c, seed, 0xa5f0);
        });
    }

    // ------------------------------------------------------------------
    // Camera.

    #[test]
    fn chase_cam_reset_to_default_matches() {
        check(ssx3_ee::s01::f_00162170__generated, chase_cam_reset_to_default, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x62170, 0);
            let cam = DATA + 0x100;
            // A small algorithm id takes SetAlgorithm's jump-table path: the
            // table is empty in RAM, so the call goes out through the hook.
            c.bus.write32(cam + 0x24, 1 + r.next_u32() % 6);
            c.bus.write32(cam + 0x30, r.next_u32());
            c.g[A0][0] = sx(cam);
        });
    }

    #[test]
    fn chase_cam_start_post_race_camera_matches() {
        check(ssx3_ee::s01::f_00162258__generated, chase_cam_start_post_race_camera, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x62258, 0);
            let cam = DATA + 0x100;
            // bit 0 of +0x30: already replaying, skip the algorithm change.
            c.bus.write32(cam + 0x30, (seed % 2) as u32);
            c.bus.write32(cam + 0x24, r.next_u32());
            c.g[A0][0] = sx(cam);
        });
    }

    #[test]
    fn active_camera_trigger_list_find_by_id_matches() {
        check(ssx3_ee::s02::f_0016c6e0__generated, active_camera_trigger_list_find_by_id, SEEDS, &|c, seed| {
            let _r = machine(c, seed, 0x6c6e0, 0);
            let head = DATA + 0x1000;
            let ids = [11u32, 22, 33];
            for k in 0..3u32 {
                let node = DATA + 0x3000 + 0x10 * k;
                c.bus.write32(head + 4 * k, DATA + 0x2000 + 0x10 * k);
                c.bus.write32(DATA + 0x2000 + 0x10 * k, node);
                c.bus.write32(node, ids[k as usize]);
            }
            c.bus.write32(head + 12, 0);
            if seed % 5 == 0 {
                c.bus.write32(head, 0); // empty list
            }
            let pick = [11u64, 33, 99, 22][(seed as usize / 5) % 4];
            c.g[A0][0] = sx(head);
            c.g[A1][0] = pick;
        });
    }

    // ------------------------------------------------------------------
    // FE screens.

    #[test]
    fn fe_char_equip_on_deactivate_matches() {
        check(ssx3_ee::s03::f_00199420__generated, fe_char_equip_on_deactivate, SEEDS, &|c, seed| {
            let _r = machine(c, seed, 0x99420, 0);
            world(c);
            let self_ = DATA + 0x300;
            c.bus.write8(self_ + 0x44, (seed % 4) as u8);
            c.g[A0][0] = sx(self_);
        });
    }

    #[test]
    fn fe_screen_blank_ctor_matches() {
        check(ssx3_ee::s03::f_001abf70__generated, fe_screen_blank_ctor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xabf70, 0);
            pool_allocator(c);
            be_library(c);
            let self_ = DATA + 0x100;
            fill(c, &mut r, self_, 0x40);
            c.g[A0][0] = sx(self_);
        });
    }

    #[test]
    fn fe_lobby_menu_on_keyboard_done_matches() {
        check(ssx3_ee::s03::f_001bd428__generated, fe_lobby_menu_on_keyboard_done, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xbd428, 0);
            let self_ = DATA + 0x100;
            let menu = DATA + 0x800;
            c.bus.write32(self_ + 0x69c, menu);
            // Not state 8: leave without requesting a room (the join path
            // needs a live lobby menu; see the report).
            c.bus.write32(menu + 0x18, r.next_u32() % 7);
            c.g[A0][0] = sx(self_);
        });
    }

    #[test]
    fn fe_popup_confirm_cfg_ctor_matches() {
        check(ssx3_ee::s03::f_001c55f8__generated, fe_popup_confirm_cfg_ctor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xc55f8, 0);
            let self_ = DATA + 0x100;
            fill(c, &mut r, self_, 0x40);
            c.g[A0][0] = sx(self_);
        });
    }

    fn widget_setup(c: &mut Cpu, seed: u32, salt: u64) -> u32 {
        let mut r = machine(c, seed, salt, 1);
        let w = DATA + 0x100;
        let vt = DATA + 0x200;
        c.bus.write32(w + 8, vt);
        c.bus.write16(vt + 0x48, ((r.next_u32() % 0x100) * 4) as u16);
        c.bus.write32(vt + 0x4c, FAKE);
        w
    }

    #[test]
    fn fe_popup_confirm_hide_widget_matches() {
        check(ssx3_ee::s03::f_001ca488__generated, fe_popup_confirm_hide_widget, SEEDS, &|c, seed| {
            let w = widget_setup(c, seed, 0xca488);
            c.g[A0][0] = sx(DATA + 0x300);
            c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(w) };
        });
    }

    #[test]
    fn fe_popup_confirm_show_widget_matches() {
        check(ssx3_ee::s03::f_001ca4c0__generated, fe_popup_confirm_show_widget, SEEDS, &|c, seed| {
            let w = widget_setup(c, seed, 0xca4c0);
            c.g[A0][0] = sx(DATA + 0x300);
            c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(w) };
        });
    }

    #[test]
    fn fe_reward_gallery_update_matches() {
        check(ssx3_ee::s04::f_001cf238__generated, fe_reward_gallery_update, SEEDS, &|c, seed| {
            let _r = machine(c, seed, 0xcf238, 3);
            world(c);
            let self_ = DATA + 0x100;
            // bits 1 and 3: both poll functions return at once, then the
            // base UI state update runs (its virtual calls land in the hook).
            let _ = seed;
            c.bus.write32(self_ + 0x48, 0xa);
            c.g[A0][0] = sx(self_);
        });
    }

    #[test]
    fn message_center_get_inbox_matches() {
        check(ssx3_ee::s04::f_001e39f8__generated, message_center_get_inbox, SEEDS, &|c, seed| {
            let _r = machine(c, seed, 0xe39f8, 2);
            be_library(c);
            // Empty BELibrary interface table: `GetInterface` resolves to
            // address 0, so the virtual call lands in the miss hook.
            c.bus.write32(GAME_GP.wrapping_sub(0x1efc), 0);
            c.bus.write32(0x0053_08a8, 0);
            c.g[A0][0] = sx(DATA + 0x300);
        });
    }

    #[test]
    fn ov_message_center_hide_widget_matches() {
        check(ssx3_ee::s04::f_001e4840__generated, ov_message_center_hide_widget, SEEDS, &|c, seed| {
            let w = widget_setup(c, seed, 0xe4840);
            c.g[A0][0] = sx(DATA + 0x300);
            c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(w) };
        });
    }

    #[test]
    fn ov_message_center_show_widget_matches() {
        check(ssx3_ee::s04::f_001e4878__generated, ov_message_center_show_widget, SEEDS, &|c, seed| {
            let w = widget_setup(c, seed, 0xe4878);
            c.g[A0][0] = sx(DATA + 0x300);
            c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(w) };
        });
    }

    #[test]
    fn ov_bcfail_pop_event_arg_matches() {
        check(ssx3_ee::s04::f_001f7738__generated, ov_bcfail_pop_event_arg, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xf7738, 0);
            let q = DATA + 0x200;
            c.bus.write32(GAME_GP + 0xce8, q);
            let head = r.next_u32();
            c.bus.write32(q + 0x1c4, head);
            c.bus.write32(q + 0x1c8, if seed % 3 == 0 { head } else { r.next_u32() });
            c.g[A0][0] = sx(DATA + 0x300);
        });
    }

    #[test]
    fn ov_multi_freestyle_pre_on_deactivate_matches() {
        check(ssx3_ee::s04::f_001fd118__generated, ov_multi_freestyle_pre_on_deactivate, SEEDS, &|c, seed| {
            let _r = machine(c, seed, 0xfd118, 1);
            be_library(c);
            world(c);
            c.bus.write32(GAME_GP + 0x410, DATA + 0x2000);
            c.bus.write32(
                GAME_GP.wrapping_sub(0x204),
                if seed % 2 == 0 { DATA + 0x2400 } else { 0 },
            );
            c.g[A0][0] = sx(DATA + 0x300);
        });
    }

    // ------------------------------------------------------------------
    // Race progress / game state.

    #[test]
    fn race_progress_set_path_data_matches() {
        check(ssx3_ee::s05::f_002105d0__generated, race_progress_set_path_data, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x105d0, 0);
            let p = DATA + 0x100;
            c.bus.write32(p + 4, r.next_u32() % 0x800);
            c.bus.write32(p + 0xc, r.next_u32() % 0x800);
            c.g[A0][0] = sx(p);
        });
    }

    #[test]
    fn game_state_reset_ai_exit_matches() {
        check(ssx3_ee::s06::f_00236b38__generated, game_state_reset_ai_exit, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x36b38, 1);
            let state = DATA + 0x400;
            let ai = DATA + 0x500;
            let vt = DATA + 0x600;
            c.bus.write32(GAME_GP.wrapping_sub(0x488), state);
            c.bus.write32(state + 0xc, ai);
            c.bus.write32(ai + 0xcc, vt);
            c.bus.write16(vt + 0x28, ((r.next_u32() % 0x40) * 4) as u16);
            c.bus.write32(vt + 0x2c, FAKE);
            c.g[A0][0] = sx(DATA + 0x300);
        });
    }

    // ------------------------------------------------------------------
    // GM save/load handlers.

    fn gm_setup(c: &mut Cpu, seed: u32, salt: u64, slot: u32) {
        let mut r = machine(c, seed, salt, 1);
        let h = DATA + 0x500;
        let vt = DATA + 0x600;
        c.bus.write32(h, vt);
        c.bus.write16(vt + slot, ((r.next_u32() % 0x40) * 4) as u16);
        c.bus.write32(vt + slot + 4, FAKE);
        c.g[A0][0] = sx(DATA + 0x300);
        c.g[A1][0] = sx(h);
    }

    #[test]
    fn gm_handler0_save_state_matches() {
        check(ssx3_ee::s06::f_00239d18__generated, gm_handler0_save_state, SEEDS, &|c, seed| {
            gm_setup(c, seed, 0x39d18, 0x08);
        });
    }

    #[test]
    fn gm_handler0_load_state_matches() {
        check(ssx3_ee::s06::f_00239d50__generated, gm_handler0_load_state, SEEDS, &|c, seed| {
            gm_setup(c, seed, 0x39d50, 0x10);
        });
    }

    #[test]
    fn gm_handler1_save_state_matches() {
        check(ssx3_ee::s06::f_0023ac18__generated, gm_handler1_save_state, SEEDS, &|c, seed| {
            gm_setup(c, seed, 0x3ac18, 0x08);
        });
    }

    #[test]
    fn gm_handler1_load_state_matches() {
        check(ssx3_ee::s06::f_0023ac50__generated, gm_handler1_load_state, SEEDS, &|c, seed| {
            gm_setup(c, seed, 0x3ac50, 0x10);
        });
    }

    #[test]
    fn gm_handler8_save_state_matches() {
        check(ssx3_ee::s06::f_0023b038__generated, gm_handler8_save_state, SEEDS, &|c, seed| {
            gm_setup(c, seed, 0x3b038, 0x08);
        });
    }

    #[test]
    fn gm_handler8_load_state_matches() {
        check(ssx3_ee::s06::f_0023b070__generated, gm_handler8_load_state, SEEDS, &|c, seed| {
            gm_setup(c, seed, 0x3b070, 0x10);
        });
    }

    // ------------------------------------------------------------------
    // Memory card manager.

    #[test]
    fn mc_manager_request_card_info_matches() {
        check(ssx3_ee::s06::f_0023fb20__generated, mc_manager_request_card_info, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3fb20, 1);
            let self_ = DATA + 0x100;
            let dev = DATA + 0x500;
            let vt = DATA + 0x600;
            c.bus.write32(self_ + 0x434, dev);
            c.bus.write32(dev, vt);
            c.bus.write16(vt + 0x28, ((r.next_u32() % 0x40) * 4) as u16);
            c.bus.write32(vt + 0x2c, FAKE);
            c.g[A0][0] = sx(self_);
            c.g[A1][0] = (r.next_u32() % 8) as u64;
        });
    }

    #[test]
    fn mc_manager_state26_matches() {
        check(ssx3_ee::s06::f_0023fd08__generated, mc_manager_state26, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3fd08, 1);
            let self_ = DATA + 0x100;
            fill(c, &mut r, self_, 0x60);
            c.bus.write32(self_ + 0xbc, 0);
            c.bus.write32(self_ + 0x38, FAKE);
            c.g[A0][0] = sx(self_);
        });
    }

    #[test]
    fn mc_manager_dtor_matches() {
        check(ssx3_ee::s06::f_002424c8__generated, mc_manager_dtor, SEEDS, &|c, seed| {
            let _r = machine(c, seed, 0x424c8, 0);
            // `del & 1 == 0`: keep the object (the freeing path is not
            // exercised here; see the report).
            c.g[A0][0] = sx(DATA + 0x100);
            c.g[A1][0] = 0;
        });
    }

    // ------------------------------------------------------------------
    // Buddy list.

    #[test]
    fn buddy_man_is_temp_at_matches() {
        check(ssx3_ee::s07::f_00262400__generated, buddy_man_is_temp_at, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x262400, 0);
            let buddy = DATA + 0x100;
            let list = DATA + 0x2000;
            let node = DATA + 0x3000;
            let entry = DATA + 0x4000;
            c.bus.write32(buddy + 0xcc, list);
            c.bus.write32(list + 0x18, [0u32, 1, 1, 1][seed as usize % 4]);
            c.bus.write32(list + 0x40, DATA + 0x5000);
            c.bus.write32(DATA + 0x5000, node);
            c.bus.write32(node, entry);
            c.bus.write32(entry + 0xa8, r.next_u32());
            let idx: [u64; 4] = [0, u64::MAX, 1, 0];
            c.g[A0][0] = sx(buddy);
            c.g[A1][0] = idx[seed as usize % 4];
        });
    }

    #[test]
    fn shared_login_do_delete_matches() {
        check(ssx3_ee::s07::f_00265ec8__generated, shared_login_do_delete, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x265ec8, 0);
            let self_ = DATA + 0x100;
            fill(c, &mut r, self_, 0x20);
            c.g[A0][0] = sx(self_);
        });
    }

    // ------------------------------------------------------------------
    // Replay.

    #[test]
    fn replay_enable_replay_cameras_matches() {
        check(ssx3_ee::s07::f_0026ee68__generated, replay_enable_replay_cameras, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x26ee68, 0);
            world(c);
            // No active triggers (`+4 <= 0`) and no riders (`*riders <= 0`).
            c.bus.write32(0x004c_5834, 0);
            c.bus.write32(DATA + 0x7000, 0);
            fill(c, &mut r, DATA + 0x300, 0x20);
            c.g[A0][0] = sx(DATA + 0x300);
        });
    }

    #[test]
    fn replay_highlight_slot_init_matches() {
        check(ssx3_ee::s07::f_00272288__generated, replay_highlight_slot_init, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x272288, 0);
            let slot = DATA + 0x100;
            fill(c, &mut r, slot, 0x40);
            c.g[A0][0] = sx(slot);
        });
    }

    // ------------------------------------------------------------------
    // InSeq track constructors.

    #[test]
    fn in_seq_curve_track_ctor_matches() {
        check(ssx3_ee::s07::f_002725b8__generated, in_seq_curve_track_ctor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2725b8, 0);
            let track = DATA + 0x100;
            fill(c, &mut r, track, 0x20);
            c.g[A0][0] = sx(track);
        });
    }

    #[test]
    fn in_seq_span_track_ctor_matches() {
        check(ssx3_ee::s07::f_00272d68__generated, in_seq_span_track_ctor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x272d68, 0);
            let track = DATA + 0x100;
            fill(c, &mut r, track, 0x20);
            c.g[A0][0] = sx(track);
        });
    }

    #[test]
    fn in_seq_event_track_ctor_matches() {
        check(ssx3_ee::s07::f_00273180__generated, in_seq_event_track_ctor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x273180, 0);
            let track = DATA + 0x100;
            fill(c, &mut r, track, 0x20);
            c.g[A0][0] = sx(track);
        });
    }

    #[test]
    fn in_seq_ramp_track_ctor_matches() {
        check(ssx3_ee::s07::f_002734a8__generated, in_seq_ramp_track_ctor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2734a8, 0);
            let track = DATA + 0x100;
            fill(c, &mut r, track, 0x20);
            c.g[A0][0] = sx(track);
        });
    }

    // ------------------------------------------------------------------
    // Script engine.

    #[test]
    fn script_engine_stop_all_matches() {
        check(ssx3_ee::s07::f_00278a98__generated, script_engine_stop_all, SEEDS, &|c, seed| {
            let _r = machine(c, seed, 0x278a98, 0);
            world(c);
            let engine = DATA + 0x100;
            // Script mode 2: the list stop functions skip their exit path.
            c.bus.write32(engine + 0x550, 2);
            c.g[A0][0] = sx(engine);
        });
    }

    #[test]
    fn script_binding_unbind_matches() {
        check(ssx3_ee::s07::f_00283000__generated, script_binding_unbind, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x283000, 1);
            let binding = DATA + 0x100;
            c.bus.write32(GAME_GP + 0x3ec, DATA + 0x2000);
            fill(c, &mut r, binding, 0x20);
            c.bus.write32(binding + 4, r.next_u32() % 8);
            c.g[A0][0] = sx(binding);
        });
    }

    // ------------------------------------------------------------------
    // Audio.

    #[test]
    fn audio_stop_all_sounds_matches() {
        check(ssx3_ee::s07::f_00285f48__generated, audio_stop_all_sounds, SEEDS, &|c, seed| {
            let _r = machine(c, seed, 0x285f48, 1);
            let sys = DATA + 0x100;
            // No crowd instances and no sound-system source list.
            c.bus.write32(sys + 0x14, 0);
            c.bus.write32(sys + 0x118, 0);
            c.g[A0][0] = sx(sys);
        });
    }

    #[test]
    fn audio_set_speech_volume_matches() {
        check(ssx3_ee::s07::f_00287520__generated, audio_set_speech_volume, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x287520, 2);
            let sys = DATA + 0x100;
            fill(c, &mut r, sys, 0x1000);
            c.bus.write32(GAME_GP.wrapping_sub(0x462c), r.edge_f32());
            c.g[A0][0] = sx(sys);
            c.g[A1][0] = (r.next_u32() % 12) as u64;
        });
    }

    #[test]
    fn snd_bank_man_set_bank_memory_matches() {
        check(ssx3_ee::s07::f_0028bda8__generated, snd_bank_man_set_bank_memory, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x28bda8, 0);
            let bank_man = DATA + 0x100;
            c.bus.write32(bank_man + 0xacc, DATA + 0x2000);
            fill(c, &mut r, DATA + 0x2000, 0x100);
            c.g[A0][0] = sx(bank_man);
            c.g[A1][0] = (r.next_u32() % 4) as u64;
            // Zero length and pointer: SndBank_SetMemory takes its early exit.
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0;
        });
    }

    #[test]
    fn audio_stop_fe_music_matches() {
        check(ssx3_ee::s07::f_0028f520__generated, audio_stop_fe_music, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x28f520, 1);
            let sys = DATA + 0x100;
            fill(c, &mut r, sys, 0x100);
            // +0x608c: 0/1/2 "music enabled", 3 also enabled, else disabled.
            c.bus.write32(sys + 0x608c, [0u32, 2, 3, 4][seed as usize % 4]);
            // No current song instance (+0x118 + 0x408): the player only
            // touches its own fields.
            c.bus.write32(sys + 0x520, 0);
            c.bus.write32(sys + 0x118, 0);
            c.g[A0][0] = sx(sys);
        });
    }

    #[test]
    fn audio_is_pa_speech_on_matches() {
        check(ssx3_ee::s08::f_0029f128__generated, audio_is_pa_speech_on, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x29f128, 0);
            let sys = DATA + 0x1000;
            // Channel 3 volume lives at +0x30 + 0x62bc.
            c.bus.write32(sys + 0x62ec, [0u32, 0x3f80_0000, 0x8000_0000, 0x4120_0000][seed as usize % 4]);
            let _ = r.next_u32();
            c.g[A0][0] = sx(sys);
        });
    }

    #[test]
    fn audio_is_arcade_speech_on_matches() {
        check(ssx3_ee::s08::f_0029f160__generated, audio_is_arcade_speech_on, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x29f160, 0);
            let sys = DATA + 0x1000;
            // Channel 0xa volume lives at +0xa0 + 0x62bc.
            c.bus.write32(sys + 0x635c, [0u32, 0xbf80_0000, 0x0000_0001, 0x7f80_0000][seed as usize % 4]);
            let _ = r.next_u32();
            c.g[A0][0] = sx(sys);
        });
    }

    #[test]
    fn crowd_audio_init_sample_list_matches() {
        check(ssx3_ee::s08::f_002a4ba8__generated, crowd_audio_init_sample_list, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2a4ba8, 0);
            let audio = DATA + 0x100;
            fill(c, &mut r, audio, 0x20);
            c.g[A0][0] = sx(audio);
        });
    }
}
