//! Phase 2 wave 18 module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for fifty small game functions, each with a differential test
//! against the generated body.
//!
//! Groups:
//! - **Front end / profile save**: the PDA slot-list refresh, the memory-card
//!   slot-name formatter, the stats-process destructor.
//! - **Sections and game state**: the section-manager node pool initialiser and
//!   the "finish cinematic" state step.
//! - **Video / buddies / login**: the video-player constructor, buddy-manager
//!   destroy and update, the shared-login account name/password setters.
//! - **Replay**: instant-replay camera enabling, the save-stream emitter, the
//!   snapshot counter and the save footer.
//! - **Script engine**: list stop/on-seq-stopped, bank-slot search, the script
//!   dummy-object constructor, a command filter, and the WScript process list
//!   and task/destructor helpers.
//! - **Audio**: the DJ song intro, the three race-type range queries, the PS2
//!   sound-manager shutdown, bank-monitor stop-all and the path track init.
//! - **Transitions / lights**: the transition manager's "start out" and the
//!   world light manager's overlap-list release.
//! - **Bx / strings / spheres**: `cBXString::SetAt`, the rider sphere-tree
//!   setter and the sphere-tree transform reset.
//! - **World objects**: the `*_Save` family (world object, conveyor, counter,
//!   flex rail, fence, mesh anim, sync process, anim delta) and the flag-group
//!   matcher.
//! - **Interrupts / textures**: the VIF/VU handler installers and the BTexInfo
//!   free.

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0021_1220,
        ov_screen_profile_save_pda_v26,
        "OVScreenProfileSavePDA_v26",
    ),
    (
        0x0022_e228,
        section_man_init_node_pool,
        "SectionMan_InitNodePool",
    ),
    (
        0x0023_6f40,
        game_state_finish_cinematic_update,
        "GameStateFinishCinematic_Update",
    ),
    (
        0x0024_1dc8,
        mc_manager_format_slot_name,
        "MCManager_FormatSlotName",
    ),
    (0x0024_49a8, stats_process_dtor, "StatsProcess_dtor"),
    (0x0025_33c8, vid_engine_player_ctor, "VidEnginePlayer_ctor"),
    (0x0026_1008, buddy_man_destroy, "BuddyMan_Destroy"),
    (0x0026_1490, buddy_man_update, "BuddyMan_Update"),
    (
        0x0026_5f18,
        c_shared_login_man_set_account_name,
        "cSharedLoginMan_SetAccountName",
    ),
    (
        0x0026_5f68,
        c_shared_login_man_set_account_password,
        "cSharedLoginMan_SetAccountPassword",
    ),
    (
        0x0026_ef30,
        replay_enable_instant_replay_cameras,
        "Replay_EnableInstantReplayCameras",
    ),
    (0x0027_1058, replay_save_emit, "Replay_SaveEmit"),
    (
        0x0027_12f8,
        replay_count_save_snapshots,
        "Replay_CountSaveSnapshots",
    ),
    (
        0x0027_1708,
        replay_save_write_footer,
        "Replay_SaveWriteFooter",
    ),
    (
        0x0027_7400,
        script_list_on_seq_stopped,
        "ScriptList_OnSeqStopped",
    ),
    (
        0x0027_90f8,
        script_engine_stop_main_lists,
        "ScriptEngine_StopMainLists",
    ),
    (
        0x0027_b528,
        script_engine_find_free_bank_slot,
        "ScriptEngine_FindFreeBankSlot",
    ),
    (0x0027_bfd0, script_dummy_obj_ctor, "ScriptDummyObj_ctor"),
    (
        0x0028_1a00,
        script_view_obj_accept_command,
        "ScriptViewObj_AcceptCommand",
    ),
    (
        0x002a_3170,
        ssx_audio_system_play_dj_song_intro,
        "SSXAudioSystem_PlayDJSongIntro",
    ),
    (
        0x002a_42c8,
        ssx_audio_system_is_race_type6_to11,
        "SSXAudioSystem_IsRaceType6to11",
    ),
    (
        0x002a_4318,
        ssx_audio_system_is_race_type6_to8,
        "SSXAudioSystem_IsRaceType6to8",
    ),
    (
        0x002a_4368,
        ssx_audio_system_is_race_type9_to11,
        "SSXAudioSystem_IsRaceType9to11",
    ),
    (
        0x002a_8f50,
        ps2_sound_manager_shutdown,
        "PS2SoundManager_Shutdown",
    ),
    (0x002a_d3c0, bank_monitor_stop_all, "BankMonitor_StopAll"),
    (0x002a_e138, path_plat_track_init, "PATH_PlatTrackInit"),
    (0x002e_44f0, c_transition_man_start_out, "cTransitionMan_StartOut"),
    (
        0x002f_53b0,
        world_light_man_free_overlap_lists,
        "WorldLightMan_FreeOverlapLists",
    ),
    (0x0030_7d78, wscript_process_terminate, "WScriptProcess_Terminate"),
    (0x0030_8228, wscript_task_ctor, "WScriptTask_ctor"),
    (
        0x0030_8fe0,
        wscript_man_unload_section_and_free,
        "WScriptMan_UnloadSectionAndFree",
    ),
    (
        0x0030_aeb8,
        wscript_man_get_current_mission,
        "WScriptMan_GetCurrentMission",
    ),
    (
        0x0030_b270,
        wscript_process_list_push_back,
        "WScriptProcessList_PushBack",
    ),
    (
        0x0030_b2d0,
        wscript_process_list_remove,
        "WScriptProcessList_Remove",
    ),
    (
        0x0030_c390,
        wscript_man_reset_collect_set,
        "WScriptMan_ResetCollectSet",
    ),
    (0x0031_8e68, c_bx_string_set_at, "cBXString_SetAt"),
    (
        0x0032_9a90,
        rider_sphere_tree_set_spheres,
        "RiderSphereTree_SetSpheres",
    ),
    (
        0x0032_c540,
        sphere_tree_reset_transform,
        "SphereTree_ResetTransform",
    ),
    (0x0034_1a50, boost_save, "Boost_Save"),
    (0x0034_3718, conveyor_save, "Conveyor_Save"),
    (0x0034_3a18, counter_save, "Counter_Save"),
    (0x0034_8b40, flex_rail_save, "FlexRail_Save"),
    (
        0x0034_afe8,
        c_flag_manager_group_matches,
        "cFlagManager_GroupMatches",
    ),
    (0x0034_d960, fence_save, "Fence_Save"),
    (0x0035_2a58, mesh_anim_save, "MeshAnim_Save"),
    (
        0x0035_56a8,
        mod_obj_set_rail_modifier,
        "ModObj_SetRailModifier",
    ),
    (0x0035_8c30, rail_man_save, "RailMan_Save"),
    (0x0035_9410, anim_delta_save, "AnimDelta_Save"),
    (
        0x0036_1e30,
        intc_table_install_vif_vu_handlers,
        "IntcTable_InstallVifVuHandlers",
    ),
    (0x0036_72c0, tex_man_free_btex_info, "TexMan_FreeBTexInfo"),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const OV_SCREEN_PROFILE_SAVE_PDA_REFRESH_CARD_INFO: Callee = Callee {
        addr: 0x0021_1380,
        f: ee::s05::OVScreenProfileSavePDA_RefreshCardInfo,
    };
    pub const OV_SCREEN_PROFILE_SAVE_PDA_RESET_SLOT_LIST: Callee = Callee {
        addr: 0x0021_1270,
        f: ee::s05::OVScreenProfileSavePDA_ResetSlotList,
    };
    pub const GAME_STATE_GOTO_NEXT: Callee = Callee {
        addr: 0x0023_3aa0,
        f: ee::s06::GameState_GotoNext,
    };
    pub const VID_ENGINE_INSTALL_MEM_HOOKS: Callee = Callee {
        addr: 0x0025_3aa0,
        f: ee::s06::VidEngine_InstallMemHooks,
    };
    pub const VID_ENGINE_PLAYER_INIT_OPEN_PARAMS: Callee = Callee {
        addr: 0x0025_3390,
        f: ee::s06::VidEnginePlayer_InitOpenParams,
    };
    pub const BUDDY_MAN_CLEAR_LOGIN: Callee = Callee {
        addr: 0x0026_1058,
        f: ee::s06::BuddyMan_ClearLogin,
    };
    pub const BUDDY_MAN_CLEAR_MESSAGES: Callee = Callee {
        addr: 0x0026_1070,
        f: ee::s06::BuddyMan_ClearMessages,
    };
    pub const BUDDY_MAN_DISCONNECT: Callee = Callee {
        addr: 0x0026_1408,
        f: ee::s06::BuddyMan_Disconnect,
    };
    pub const REPLAY_SAVE_EMIT: Callee = Callee {
        addr: 0x0027_1058,
        f: ee::s07::Replay_SaveEmit,
    };
    pub const SCRIPT_LIST_IS_PLAYING_SCRIPT: Callee = Callee {
        addr: 0x0027_71c8,
        f: ee::s07::ScriptList_IsPlayingScript,
    };
    pub const SCRIPT_DATA_GET_INFO: Callee = Callee {
        addr: 0x0027_c070,
        f: ee::s07::ScriptData_GetInfo,
    };
    pub const SCRIPT_ENGINE_LIST_STOP: Callee = Callee {
        addr: 0x0027_90a0,
        f: ee::s07::ScriptEngine_ListStop,
    };
    pub const SCRIPT_ENGINE_GET_INSTANCE_SCRIPT: Callee = Callee {
        addr: 0x0027_a668,
        f: ee::s07::ScriptEngine_GetInstanceScript,
    };
    pub const SCRIPT_BINDING_CTOR: Callee = Callee {
        addr: 0x0028_2f60,
        f: ee::s07::ScriptBinding_ctor,
    };
    pub const SCRIPT_OBJ_CTOR: Callee = Callee {
        addr: 0x0028_3200,
        f: ee::s07::ScriptObj_ctor,
    };
    pub const SSX_AUDIO_SYSTEM_DJ_SAY_20E5: Callee = Callee {
        addr: 0x002a_26f0,
        f: ee::s08::SSXAudioSystem_DJSay20E5,
    };
    pub const SPEECH_SYSTEM_CANCEL_PENDING_INSTANCE: Callee = Callee {
        addr: 0x002b_1758,
        f: ee::s08::SpeechSystem_CancelPendingInstance,
    };
    pub const MUSIC_PLAYER_GET_SELECTED_SONG_SED_VALUE: Callee = Callee {
        addr: 0x002b_4908,
        f: ee::s08::MusicPlayer_GetSelectedSongSedValue,
    };
    pub const BANK_MONITOR_RESET_INSTANCE: Callee = Callee {
        addr: 0x002a_d398,
        f: ee::s08::BankMonitor_ResetInstance,
    };
    pub const C_TRANSITION_MAN_START: Callee = Callee {
        addr: 0x002e_4370,
        f: ee::s09::cTransitionMan_Start,
    };
    pub const WSCRIPT_MAN_DEACTIVATE_PROCESS: Callee = Callee {
        addr: 0x0030_9aa0,
        f: ee::s10::WScriptMan_DeactivateProcess,
    };
    pub const WSCRIPT_MAN_UNLOAD_SECTION: Callee = Callee {
        addr: 0x0030_8f70,
        f: ee::s10::WScriptMan_UnloadSection,
    };
    pub const C_BX_STRING_COPY_BEFORE_WRITE: Callee = Callee {
        addr: 0x0031_8120,
        f: ee::s10::cBXString_CopyBeforeWrite,
    };
    pub const WORLD_OBJ_SAVE: Callee = Callee {
        addr: 0x0034_fe90,
        f: ee::s11::WorldObj_Save,
    };
    pub const SYNC_PROCESS_SAVE: Callee = Callee {
        addr: 0x0035_4980,
        f: ee::s11::SyncProcess_Save,
    };
    pub const ANIM_OBJECT_SAVE: Callee = Callee {
        addr: 0x0034_2150,
        f: ee::s11::AnimObject_Save,
    };
    pub const MOD_OBJ_CREATE_MODIFIER_BLOCK: Callee = Callee {
        addr: 0x0035_5468,
        f: ee::s11::ModObj_CreateModifierBlock,
    };
    pub const RAIL_MODIFIER_LIST_PUSH_FRONT: Callee = Callee {
        addr: 0x0035_b670,
        f: ee::s11::RailModifier_ListPushFront,
    };
    pub const INTC_TABLE_ADD_HANDLER: Callee = Callee {
        addr: 0x0036_1f98,
        f: ee::s12::IntcTable_AddHandler,
    };
}

/// `gp`-relative global (`lw(gp + off)` / `sw(gp + off, v)`).
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `base + off` (32-bit address arithmetic).
#[inline(always)]
fn p(base: u32, off: u32) -> u32 {
    base.wrapping_add(off)
}

// ---------------------------------------------------------------------------
// Front end / profile save.

/// `void OVScreenProfileSavePDA_v26(OVScreenProfileSavePDA* self)`
/// (0x211220): flag the pending info write and refresh the slot list (or the
/// card info when the screen sits in mode 3).
pub fn ov_screen_profile_save_pda_v26(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[V0][0] = Addr(this).off(0x1c0).lw(c);
        let has_info = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if has_info {
            c.cycles += 1;
            Addr(this).off(0x22c).sw(c, c.g[V0][0]);
        }
        c.cycles += 4;
        c.g[V1][0] = Addr(this).off(0x214).lw(c);
        c.g[V0][0] = 3;
        if c.g[V1][0] != c.g[V0][0] {
            c.cycles += 3;
            callee::OV_SCREEN_PROFILE_SAVE_PDA_RESET_SLOT_LIST.call(c, 0x0021_1260)?;
        } else {
            c.cycles += 4;
            callee::OV_SCREEN_PROFILE_SAVE_PDA_REFRESH_CARD_INFO.call(c, 0x0021_1250)?;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_FormatSlotName(char* dst, int slot, int max)`
/// (0x241dc8): write the slot name (from the wide-char table at 0x4a2c28)
/// into a stack buffer, then format it into `dst` (`max + 1` chars).
pub fn mc_manager_format_slot_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x230);
        frame.save(c, S0, 0x220);
        c.g[A0][0] = c.g[SP][0];
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x210);
        c.g[A1][0] = 0x4a0000;
        frame.save_ra(c, 0x200);
        c.g[S1][0] = c.g[A2][0];
        c.g[A1][0] = sx(0x004a_2c28);
        call_ptr(c, 0x002c_2540, 0x0024_1df0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = sx(r32(c, S1).wrapping_add(1));
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x002c_26d0, 0x0024_1e00)?;
        frame.restore(c, S0, 0x220);
        frame.restore(c, S1, 0x210);
        frame.restore_ra(c, 0x200);
        frame.close(c);
        Ok(())
    })
}

/// `void StatsProcess_dtor(StatsProcess* self, int flags)` (0x2449a8):
/// install the destructor vtable, free the owned buffer and run the base
/// destructor.
pub fn stats_process_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x480000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0047_cae8);
        frame.save(c, S1, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        Addr(this).off(0xc).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(this).off(0x10).lw(c);
        call_ptr(c, 0x0031_7e50, 0x0024_49d4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0035_46c8, 0x0024_49e0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Sections and game state.

/// `void SectionMan_InitNodePool(SectionMan* self)` (0x22e228): link the 50
/// free-list nodes into a chain, then publish the free-list head.
pub fn section_man_init_node_pool(c: &mut Cpu) {
    c.cycles += 6;
    let this = r32(c, A0);
    Addr(this).off(0x198).sw(c, c.g[A0][0]);
    c.g[A2][0] = 0x31;
    c.g[V1][0] = sx(this.wrapping_add(8));
    c.g[V0][0] = c.g[A0][0];
    c.g[A1][0] = 0x31;
    loop {
        c.cycles += 7;
        Addr(r32(c, V0)).off(4).sw(c, c.g[V1][0]);
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
        Addr(r32(c, V0)).sw(c, c.g[A2][0]);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(8));
        let more = (c.g[A1][0] as i64) >= 0;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(8));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0022_e240);
        }
    }
    c.cycles += 6;
    c.g[V0][0] = 0x31;
    Addr(this).off(0x1a0).sw(c, 0);
    Addr(this).off(0x190).sw(c, c.g[V0][0]);
    Addr(this).off(0x194).sw(c, 0);
    Addr(this).off(0x19c).sw(c, 0);
    abi::ret(c);
}

/// `void GameStateFinishCinematic_Update(GameState* self)` (0x236f40): while
/// the script engine is idle (status 0 or 5), advance to the next state.
pub fn game_state_finish_cinematic_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(gp(c, -0x84c)).lw(c);
        call_ptr(c, 0x0027_9220, 0x0023_6f5c)?;
        let idle = c.g[V0][0] == 0;
        c.g[V1][0] = 0;
        if !idle {
            c.cycles += 2;
            c.g[V0][0] ^= 5;
            c.g[V1][0] = (c.g[V0][0] > 0) as u64;
        }
        c.cycles += 2;
        if c.g[V1][0] != 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::GAME_STATE_GOTO_NEXT.call(c, 0x0023_6f7c)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Video / buddies / login.

/// `VidEnginePlayer* VidEnginePlayer_ctor(VidEnginePlayer* self)`
/// (0x2533c8): install the memory hooks, init the open parameters and clear
/// the playback fields.
pub fn vid_engine_player_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        callee::VID_ENGINE_INSTALL_MEM_HOOKS.call(c, 0x0025_33dc)?;
        c.g[A0][0] = sx(this.wrapping_add(0x14));
        callee::VID_ENGINE_PLAYER_INIT_OPEN_PARAMS.call(c, 0x0025_33e4)?;
        c.lwc1(0, p(this, 0x34));
        c.g[V0][0] = c.g[S0][0];
        Addr(this).off(0x38).sw(c, 0);
        c.swc1(0, p(this, 0x10));
        Addr(this).off(0x3c).sw(c, 0);
        Addr(this).off(0xc).sw(c, 0);
        Addr(this).off(0x40).sw(c, 0);
        Addr(this).off(8).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BuddyMan_Destroy(BuddyMan* this)` (0x261008): tear down the global
/// buddy manager once (`gp+0x238`) and clear the global slot.
pub fn buddy_man_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
        let absent = c.g[A0][0] == 0;
        frame.save_ra(c, 0);
        if !absent {
            c.cycles += 9;
            callee::BUDDY_MAN_CLEAR_LOGIN.call(c, 0x0026_1020)?;
            c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
            callee::BUDDY_MAN_CLEAR_MESSAGES.call(c, 0x0026_1028)?;
            c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
            callee::BUDDY_MAN_DISCONNECT.call(c, 0x0026_1030)?;
            c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
            if c.g[A0][0] == 0 {
                Addr(gp(c, 0x238)).sw(c, 0);
            } else {
                c.cycles += 3;
                c.g[A1][0] = 3;
                call_ptr(c, 0x0026_89b8, 0x0026_1044)?;
                Addr(gp(c, 0x238)).sw(c, 0);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BuddyMan_Update(BuddyMan* this)` (0x261490): run the login update
/// and drop the connection when the pending flag is set.
pub fn buddy_man_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(this).off(0xc8).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 5;
            call_ptr(c, 0x003f_17d8, 0x0026_14b4)?;
            c.g[V0][0] = Addr(this).off(0xdc).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                callee::BUDDY_MAN_DISCONNECT.call(c, 0x0026_14c8)?;
                Addr(this).off(0xdc).sw(c, 0);
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cSharedLoginMan_SetAccountName(cSharedLoginMan* this, const char*
/// name)` (0x265f18): clear the 17-byte account-name buffer and copy `name`
/// into it.
pub fn c_shared_login_man_set_account_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x20);
        c.g[A2][0] = 0x11;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = 0x530000;
        c.g[S0][0] = sx(0x0053_6760);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_6448, 0x0026_5f44)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0041_66f4, 0x0026_5f50)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cSharedLoginMan_SetAccountPassword(cSharedLoginMan* this, const
/// char* password)` (0x265f68): same as the name setter, one field later.
pub fn c_shared_login_man_set_account_password(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x20);
        c.g[A2][0] = 0x11;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = 0x530000;
        c.g[S0][0] = sx(0x0053_6771);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_6448, 0x0026_5f94)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0041_66f4, 0x0026_5fa0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay.

/// `void Replay_EnableInstantReplayCameras(ReplayCache* self)` (0x26ef30):
/// refresh the camera triggers and step the instant-replay camera index
/// (`(n + 1) % 9`).
pub fn replay_enable_instant_replay_cameras(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x4c0000;
        c.g[A0][0] = sx(0x004c_5830);
        call_ptr(c, 0x0016_d1d8, 0x0026_ef4c)?;
        c.g[V1][0] = Addr(this).off(0x634).lw(c);
        c.g[V0][0] = 9;
        c.cycles += 8;
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
        frame.restore_ra(c, 0);
        c.div(0, V1, V0);
        c.g[A0][0] = c.hi[0];
        Addr(this).off(0x634).sw(c, c.g[A0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_SaveEmit(ReplayCache* self, const signed char* data, int
/// size)` (0x271058): account `size` bytes (and their byte sum) and remember
/// the last emitted block.
pub fn replay_save_emit(c: &mut Cpu) {
    c.cycles += 4;
    let this = r32(c, A0);
    c.g[V0][0] = Addr(this).off(0x680).lw(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A2)));
    let empty = (c.g[A2][0] as i64) <= 0;
    Addr(this).off(0x680).sw(c, c.g[V0][0]);
    if !empty {
        c.cycles += 2;
        c.g[T0][0] = c.g[A1][0];
        c.g[A3][0] = c.g[A2][0];
        loop {
            c.cycles += 7;
            c.g[V1][0] = c.lb(r32(c, T0));
            c.g[A3][0] = sx(r32(c, A3).wrapping_sub(1));
            c.g[V0][0] = Addr(this).off(0x684).lw(c);
            c.g[T0][0] = sx(r32(c, T0).wrapping_add(1));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            let more = c.g[A3][0] != 0;
            Addr(this).off(0x684).sw(c, c.g[V0][0]);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_1070);
            }
        }
    }
    c.cycles += 6;
    c.g[V0][0] = Addr(this).off(0x678).lw(c);
    Addr(this).off(0x644).sw(c, c.g[A1][0]);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A2)));
    Addr(this).off(0x648).sw(c, c.g[A2][0]);
    Addr(this).off(0x678).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `int Replay_CountSaveSnapshots(ReplayCache* self)` (0x2712f8): walk the
/// snapshot chain from `+0x3b0`, skipping the one at `+0x3d0`, and return how
/// many others there are.
pub fn replay_count_save_snapshots(c: &mut Cpu) {
    c.cycles += 4;
    let this = r32(c, A0);
    c.g[V0][0] = Addr(this).off(0x3b0).lw(c);
    c.g[V1][0] = 0;
    let empty = c.g[V0][0] == 0;
    Addr(this).off(0x65c).sw(c, c.g[V0][0]);
    if !empty {
        c.cycles += 2;
        c.g[A1][0] = Addr(this).off(0x3d0).lw(c);
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(this).off(0x65c).lw(c);
            if c.g[V0][0] != c.g[A1][0] {
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, V0)).off(0x14).lw(c);
                Addr(this).off(0x65c).sw(c, c.g[V0][0]);
                c.g[V0][0] = Addr(this).off(0x65c).lw(c);
            }
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, V0)).off(0x14).lw(c);
            let more = c.g[V0][0] != 0;
            Addr(this).off(0x65c).sw(c, c.g[V0][0]);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_1310);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[V1][0];
    abi::ret(c);
}

/// `void Replay_SaveWriteFooter(ReplayCache* self)` (0x271708): clear the
/// 8-byte header block and emit it as the save footer.
pub fn replay_save_write_footer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 8;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = sx(this.wrapping_add(0x680));
        c.g[A0][0] = Addr(this).off(0x640).lw(c);
        call_ptr(c, 0x003e_6574, 0x0027_1728)?;
        c.g[A1][0] = Addr(this).off(0x640).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 8;
        callee::REPLAY_SAVE_EMIT.call(c, 0x0027_1738)?;
        c.g[V0][0] = Addr(this).off(0x638).lw(c);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        Addr(this).off(0x638).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script engine.

/// `void ScriptList_OnSeqStopped(ScriptList* self, void* list, void*
/// script)` (0x277400): when `script` is still playing and its block exists,
/// notify the render-state manager with 0.
pub fn script_list_on_seq_stopped(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[S0][0];
        callee::SCRIPT_LIST_IS_PLAYING_SCRIPT.call(c, 0x0027_7418)?;
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            callee::SCRIPT_DATA_GET_INFO.call(c, 0x0027_7428)?;
            c.g[V1][0] = Addr(r32(c, V0)).lb(c);
            let no_flag = c.g[V1][0] == 0;
            frame.restore(c, S0, 0x10);
            if !no_flag {
                c.cycles += 3;
                c.g[A0][0] = 0;
                call_ptr(c, 0x002e_f378, 0x0027_743c)?;
                frame.restore(c, S0, 0x10);
            }
        } else {
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptEngine_StopMainLists(ScriptEngine* self)` (0x2790f8): stop
/// both main script lists (0 and 1).
pub fn script_engine_stop_main_lists(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = 0;
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = 0;
        loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S1][0];
            callee::SCRIPT_ENGINE_LIST_STOP.call(c, 0x0027_9120)?;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < 2) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S0][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_9118);
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

/// `int ScriptEngine_FindFreeBankSlot(ScriptEngine* self)` (0x27b528): first
/// free sound-bank slot in 2..9, else -1.
pub fn script_engine_find_free_bank_slot(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x648));
    c.g[A1][0] = 2;
    c.g[A2][0] = 1;
    loop {
        c.cycles += 3;
        let slot = r32(c, A0);
        c.g[V0][0] = Addr(slot).off(8).lw(c);
        let busy = c.g[V0][0] != 0;
        c.g[V1][0] = 0;
        if !busy {
            c.cycles += 3;
            c.g[V0][0] = Addr(slot).lw(c);
            c.g[V1][0] = c.g[A2][0];
            if c.g[V0][0] != 0 {
                c.g[V1][0] = 0;
            }
        }
        c.cycles += 2;
        if c.g[V1][0] == 0 {
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < 10) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
            if !more {
                c.cycles += 2;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                abi::ret(c);
                return;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_b538);
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.g[A1][0];
            abi::ret(c);
            return;
        }
    }
}

/// `ScriptDummyObj* ScriptDummyObj_ctor(ScriptDummyObj* self)` (0x27bfd0):
/// construct the binding base and the 4-slot script object, then install the
/// two vtables.
pub fn script_dummy_obj_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_BINDING_CTOR.call(c, 0x0027_bfe4)?;
        c.g[A0][0] = sx(this.wrapping_add(0xc));
        c.g[A1][0] = 4;
        callee::SCRIPT_OBJ_CTOR.call(c, 0x0027_bff0)?;
        c.g[V1][0] = 0x480000;
        c.g[A0][0] = 0x480000;
        c.g[V1][0] = sx(0x0048_2318);
        c.g[A0][0] = sx(0x0048_22e0);
        Addr(this).off(0x18).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(this).sw(c, c.g[A0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int ScriptViewObj_AcceptCommand(ScriptViewObj* self, void* arg, int
/// cmd)` (0x281a00): accept command 4 when the view object's script is the
/// engine's current one.
pub fn script_view_obj_accept_command(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 4;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        let other = c.g[A2][0] != c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if other {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 8;
            c.g[A1][0] = Addr(r32(c, A1)).off(8).lw(c);
            c.g[A0][0] = Addr(gp(c, -0x84c)).lw(c);
            callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT.call(c, 0x0028_1a24)?;
            c.g[V0][0] = Addr(r32(c, V0)).off(0xc).lw(c);
            c.g[V1][0] = Addr(this).off(0xf8).lw(c);
            c.g[V0][0] ^= c.g[V1][0];
            c.g[V0][0] = (c.g[V0][0] < 1) as u64;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Audio.

/// `void SSXAudioSystem_PlayDJSongIntro(SSXAudioSystem* self)` (0x2a3170):
/// cut the DJ speech, mark the intro as pending and remember the selected
/// song's SED value.
pub fn ssx_audio_system_play_dj_song_intro(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0;
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        callee::SSX_AUDIO_SYSTEM_DJ_SAY_20E5.call(c, 0x002a_318c)?;
        c.g[A0][0] = sx(this.wrapping_add(0x5560));
        c.g[A1][0] = 0;
        callee::SPEECH_SYSTEM_CANCEL_PENDING_INSTANCE.call(c, 0x002a_3198)?;
        c.g[V0][0] = 1;
        c.g[A0][0] = sx(this.wrapping_add(0x118));
        Addr(this).off(0x5774).sw(c, c.g[V0][0]);
        callee::MUSIC_PLAYER_GET_SELECTED_SONG_SED_VALUE.call(c, 0x002a_31a8)?;
        Addr(this).off(0x5778).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared body of the three race-type range checks: is the current race
/// type (the byte at 0x535c12) inside `[lo, hi]`?
fn race_type_in_range(c: &mut Cpu, lo: i64, hi: i64, rets: [u32; 3]) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    call_ptr(c, 0x0014_dc80, rets[0])?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x0014_dd58, rets[1])?;
    c.g[V1][0] = 0x530000;
    c.g[V1][0] = Addr(0x0053_5c12).lb(c);
    c.g[V0][0] = ((c.g[V1][0] as i64) < lo) as u64;
    let below = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if !below {
        c.cycles += 3;
        c.g[V1][0] = ((c.g[V1][0] as i64) < hi) as u64;
        let inside = c.g[V1][0] != 0;
        c.g[V0][0] = 1;
        if !inside {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
    }
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `bool SSXAudioSystem_IsRaceType6to11(void)` (0x2a42c8).
pub fn ssx_audio_system_is_race_type6_to11(c: &mut Cpu) {
    guest(c, |c| {
        race_type_in_range(c, 6, 12, [0x002a_42d8, 0x002a_42e4, 0x002a_4308])
    })
}

/// `bool SSXAudioSystem_IsRaceType6to8(void)` (0x2a4318).
pub fn ssx_audio_system_is_race_type6_to8(c: &mut Cpu) {
    guest(c, |c| {
        race_type_in_range(c, 6, 9, [0x002a_4328, 0x002a_4334, 0x002a_4358])
    })
}

/// `bool SSXAudioSystem_IsRaceType9to11(void)` (0x2a4368).
pub fn ssx_audio_system_is_race_type9_to11(c: &mut Cpu) {
    guest(c, |c| {
        race_type_in_range(c, 9, 12, [0x002a_4378, 0x002a_4384, 0x002a_43a8])
    })
}

/// `void PS2SoundManager_Shutdown(PS2SoundManager* self)` (0x2a8f50): stop
/// the SPU streams, shut the driver down and free the owned buffer.
pub fn ps2_sound_manager_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        frame.save(c, S0, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x003b_5b20, 0x002a_8f6c)?;
        call_ptr(c, 0x003b_5320, 0x002a_8f74)?;
        c.g[A0][0] = Addr(this).off(4).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x002a_8f88)?;
        }
        Addr(this).off(4).sw(c, 0);
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BankMonitor_StopAll(BankMonitor* self)` (0x2ad3c0): reset all 64
/// bank-monitor instances.
pub fn bank_monitor_stop_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = 0;
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = 0;
        loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S1][0];
            callee::BANK_MONITOR_RESET_INSTANCE.call(c, 0x002a_d3e8)?;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < 64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S0][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002a_d3e0);
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

/// `void PATH_PlatTrackInit(PATH_PlatTrack* self)` (0x2ae138): zero the
/// 0x70-byte track and mark it empty.
pub fn path_plat_track_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0x70;
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0041_6210, 0x002a_e154)?;
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        c.g[V0][0] = 1;
        Addr(this).off(0x65).sb(c, c.g[V0][0]);
        Addr(this).off(8).sw(c, c.g[V1][0]);
        Addr(this).sw(c, c.g[V1][0]);
        Addr(this).off(4).sw(c, c.g[V1][0]);
        Addr(this).off(0x66).sb(c, c.g[V1][0]);
        Addr(this).off(0x5c).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Transitions and lights.

/// `int cTransitionMan_StartOut(cTransitionMan* self, void* a1, int a2,
/// float f)` (0x2e44f0): while idle, start the "out" transition (0.0, 0.0
/// time); otherwise record the duration and target and report success.
pub fn c_transition_man_start_out(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[V0][0] = Addr(this).off(0x44).lw(c);
        let idle = c.g[V0][0] == 0;
        c.mov_s(14, 12);
        if idle {
            c.cycles += 7;
            c.mtc1(0, 12);
            c.g[A3][0] = c.g[A2][0];
            c.g[A2][0] = 0;
            c.g[T0][0] = 0;
            c.mov_s(13, 12);
            c.g[T1][0] = 0;
            callee::C_TRANSITION_MAN_START.call(c, 0x002e_4530)?;
        } else {
            c.cycles += 4;
            c.swc1(14, p(this, 0x5c));
            c.g[V0][0] = 1;
            Addr(this).off(0x50).sw(c, c.g[A2][0]);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WorldLightMan_FreeOverlapLists(WorldLightMan* self)` (0x2f53b0):
/// release the 8 overlap lists at `+0x40`, 0x40 bytes apart.
pub fn world_light_man_free_overlap_lists(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(r32(c, A0).wrapping_add(0x40));
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        loop {
            c.cycles += 6;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x40));
            call_ptr(c, 0x0031_7e50, 0x002f_53dc)?;
            c.g[V0][0] = (c.g[S1][0] < 8) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[A0][0] = Addr(r32(c, S0)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x002f_53d0);
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
// WScript.

/// `void WScriptProcess_Terminate(WScriptProcess* self)` (0x307d78): mark the
/// process dead, detach it from its manager and release the owned object.
pub fn wscript_process_terminate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 3;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S0][0];
        Addr(this).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(this).off(0x10).lw(c);
        callee::WSCRIPT_MAN_DEACTIVATE_PROCESS.call(c, 0x0030_7d9c)?;
        c.g[A0][0] = Addr(this).off(0x1c).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0022_4df0, 0x0030_7db0)?;
        }
        Addr(this).off(0x1c).sw(c, 0);
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `WScriptTask* WScriptTask_ctor(WScriptTask* self)` (0x308228): install the
/// task vtable, clear the 12-byte name and reset the counters.
pub fn wscript_task_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x490000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx(0x0048_9ae0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(this.wrapping_add(8));
        Addr(this).off(0x40).sw(c, c.g[V0][0]);
        c.g[A1][0] = 0;
        c.g[A2][0] = 0xc;
        call_ptr(c, 0x003e_6448, 0x0030_8254)?;
        c.g[V0][0] = c.g[S0][0];
        Addr(this).off(0x3c).sw(c, 0);
        Addr(this).off(4).sw(c, 0);
        Addr(this).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptMan_UnloadSectionAndFree(WScriptMan* self, int section)`
/// (0x308fe0): unload the section and release its cache entry.
pub fn wscript_man_unload_section_and_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        callee::WSCRIPT_MAN_UNLOAD_SECTION.call(c, 0x0030_8ffc)?;
        c.g[A0][0] = Addr(r32(c, S1)).off(0x28c).lw(c);
        c.g[V0][0] = 0xc;
        c.mult(0, S0, S0, V0);
        c.g[V1][0] = Addr(r32(c, A0)).off(4).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).off(4).lw(c);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S0)));
        call_ptr(c, 0x003a_cc90, 0x0030_9018)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* WScriptMan_GetCurrentMission(WScriptMan* self)` (0x30aeb8): the
/// mission object when it accepts the current frame, else 0.
pub fn wscript_man_get_current_mission(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = Addr(this).off(0x2a4).lw(c);
        let none = c.g[A1][0] == 0;
        c.g[V0][0] = 0;
        if !none {
            c.cycles += 7;
            c.g[V1][0] = Addr(r32(c, A1)).off(0x5c).lw(c);
            c.g[A0][0] = Addr(r32(c, V1)).off(0x18).lh(c);
            c.g[V0][0] = Addr(r32(c, V1)).off(0x1c).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x0030_aee8)?;
            let zero = c.g[V0][0] == 0;
            c.g[V0][0] = 0;
            if !zero {
                c.cycles += 1;
                c.g[V0][0] = Addr(this).off(0x2a4).lw(c);
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptProcessList_PushBack(WScriptProcessList* self,
/// cWScriptListNode* node)` (0x30b270): append `node` to the list.
pub fn wscript_process_list_push_back(c: &mut Cpu) {
    c.cycles += 4;
    let node = c.g[A1][0];
    Addr(r32(c, A1)).off(0x18).sw(c, 0);
    c.g[V1][0] = Addr(r32(c, A0)).lw(c);
    let empty = c.g[V1][0] == 0;
    if empty {
        c.cycles += 2;
        Addr(r32(c, A0)).sw(c, node);
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, V1)).off(0x18).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 1;
        c.g[V1][0] = Addr(r32(c, V1)).off(0x18).lw(c);
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(r32(c, V1)).off(0x18).lw(c);
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V1][0] = Addr(r32(c, V1)).off(0x18).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0030_b290);
            }
        }
    }
    c.cycles += 2;
    Addr(r32(c, V1)).off(0x18).sw(c, node);
    abi::ret(c);
}

/// `int WScriptProcessList_Remove(WScriptProcessList* self,
/// cWScriptListNode* node)` (0x30b2d0): unlink `node`, 1 when it was found.
pub fn wscript_process_list_remove(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, A0)).lw(c);
    if c.g[A1][0] != c.g[V0][0] {
        c.cycles += 2;
        let empty = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[V0][0];
        if empty {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, V0)).off(0x18).lw(c);
            if c.g[V0][0] == c.g[A1][0] {
                c.g[V1][0] = Addr(r32(c, V0)).off(0x18).lw(c);
                c.cycles += 3;
                c.g[V0][0] = 1;
                Addr(r32(c, A0)).off(0x18).sw(c, c.g[V1][0]);
                abi::ret(c);
                return;
            }
            c.cycles += 2;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[V0][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0030_b300);
            }
        }
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    c.g[V1][0] = Addr(r32(c, A1)).off(0x18).lw(c);
    c.g[V0][0] = 1;
    Addr(r32(c, A0)).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `void WScriptMan_ResetCollectSet(WScriptMan* self)` (0x30c390): reset the
/// collect-set bookkeeping and fill the 64 entries at `+0x0c..+0x108` with
/// -1.
pub fn wscript_man_reset_collect_set(c: &mut Cpu) {
    c.cycles += 10;
    let this = r32(c, A0);
    c.g[V0][0] = 0xffff_ffff_ffff_0000;
    c.g[A1][0] = 0xffff_ffff_ffff_0000;
    c.g[V0][0] |= 0xffff;
    Addr(this).off(4).sw(c, 0);
    Addr(this).sw(c, c.g[V0][0]);
    c.g[A1][0] |= 0xffff;
    Addr(this).off(8).sw(c, 0);
    c.g[V0][0] = sx(this.wrapping_add(0x108));
    c.g[V1][0] = 0x3f;
    loop {
        c.cycles += 7;
        Addr(r32(c, V0)).sw(c, c.g[A1][0]);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
        if !more {
            c.cycles += 2;
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0030_c3b8);
        }
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Bx strings and spheres.

/// `void cBXString_SetAt(cBXString* self, int idx, char ch)` (0x318e68): make
/// the buffer writable, then store `ch` at `idx`.
pub fn c_bx_string_set_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = sx(r32(c, A2) << 24);
        frame.save(c, S2, 0x10);
        let str = r32(c, A0);
        let idx = r32(c, A1);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = sx((((c.g[S0][0] as u32) as i32) >> 24) as u32);
        callee::C_BX_STRING_COPY_BEFORE_WRITE.call(c, 0x0031_8e90)?;
        c.g[V0][0] = Addr(str).lw(c);
        frame.restore(c, S1, 0x20);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(idx));
        frame.restore_ra(c, 0);
        Addr(r32(c, V0)).sb(c, c.g[S0][0]);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderSphereTree_SetSpheres(RiderSphereTree* self, int n,
/// const Sphere* spheres, const float* radii, const Vec4* center, ...)`
/// (0x329a90): copy up to `n` child sphere bounds, then the root sphere.
pub fn rider_sphere_tree_set_spheres(c: &mut Cpu) {
    c.cycles += 3;
    let this = r32(c, A0);
    Addr(this).off(0x2c).sw(c, c.g[A1][0]);
    let none = (c.g[A1][0] as i64) <= 0;
    c.g[T1][0] = 0;
    if !none {
        c.cycles += 1;
        c.g[V1][0] = c.g[A0][0];
        loop {
            c.cycles += 11;
            c.lq(2, r32(c, A2));
            c.g[T1][0] = sx(r32(c, T1).wrapping_add(1));
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x10));
            c.sq(2, p(r32(c, V1), 0x30));
            c.lwc1(0, r32(c, A3));
            c.g[A3][0] = sx(r32(c, A3).wrapping_add(4));
            c.swc1(0, p(r32(c, V1), 0x40));
            c.g[V0][0] = Addr(this).off(0x2c).lw(c);
            c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0032_9aa0);
            }
        }
    }
    c.cycles += 4;
    c.lq(2, r32(c, T0));
    c.swc1(12, p(this, 0x20));
    c.sq(2, p(this, 0x10));
    abi::ret(c);
}

/// `void SphereTree_ResetTransform(SphereTree* self)` (0x32c540): copy the 8
/// unit sphere nodes into the tree and install the identity transform and a
/// 1.0 radius.
pub fn sphere_tree_reset_transform(c: &mut Cpu) {
    c.cycles += 4;
    let this = r32(c, A0);
    c.g[V0][0] = 0x500000;
    c.g[V1][0] = c.g[A0][0];
    c.g[A2][0] = sx(0x004f_f640);
    c.g[A1][0] = 7;
    loop {
        c.cycles += 7;
        c.lq(2, r32(c, A2));
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x10));
        c.sq(2, r32(c, V1));
        let more = (c.g[A1][0] as i64) >= 0;
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x10));
        if !more {
            c.cycles += 8;
            c.g[V0][0] = 0x500000;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.g[V0][0] = sx(0x004f_f130);
            c.lq(3, r32(c, V0));
            c.swc1(0, p(this, 0x90));
            c.sq(3, p(this, 0x80));
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0032_c550);
        }
    }
}

// ---------------------------------------------------------------------------
// World object save family.

/// The shared save tail: run the type's `save` prelude, then call the stream's
/// serialiser slot (`vt[1]`) on `obj + off` for `size` bytes.
fn save_field_block(c: &mut Cpu, pre: Callee, off: u32, size: u32, r1: u32, r2: u32) -> Flow {
    c.cycles += 19;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    let obj = r32(c, A0);
    c.g[S1][0] = c.g[A0][0];
    pre.call(c, r1)?;
    c.g[V1][0] = Addr(r32(c, S0)).lw(c);
    c.g[A1][0] = sx(obj.wrapping_add(off));
    c.g[A2][0] = size as u64;
    c.g[A0][0] = Addr(r32(c, V1)).off(8).lh(c);
    c.g[V0][0] = Addr(r32(c, V1)).off(0xc).lw(c);
    let target = r32(c, V0);
    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
    call_ptr(c, target, r2)?;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void Boost_Save(Boost* self, Stream* stream)` (0x341a50): 0x30 bytes at
/// `+0x20`.
pub fn boost_save(c: &mut Cpu) {
    guest(c, |c| {
        save_field_block(c, callee::WORLD_OBJ_SAVE, 0x20, 0x30, 0x0034_1a6c, 0x0034_1a88)
    })
}

/// `void Conveyor_Save(Conveyor* self, Stream* stream)` (0x343718): 0x2c bytes
/// at `+0x1c`.
pub fn conveyor_save(c: &mut Cpu) {
    guest(c, |c| {
        save_field_block(c, callee::WORLD_OBJ_SAVE, 0x1c, 0x2c, 0x0034_3734, 0x0034_3750)
    })
}

/// `void Counter_Save(Counter* self, Stream* stream)` (0x343a18): 8 bytes at
/// `+0x1c`.
pub fn counter_save(c: &mut Cpu) {
    guest(c, |c| {
        save_field_block(c, callee::WORLD_OBJ_SAVE, 0x1c, 8, 0x0034_3a34, 0x0034_3a50)
    })
}

/// `void FlexRail_Save(FlexRail* self, Stream* stream)` (0x348b40): 0x30 bytes
/// at `+0x20`.
pub fn flex_rail_save(c: &mut Cpu) {
    guest(c, |c| {
        save_field_block(c, callee::WORLD_OBJ_SAVE, 0x20, 0x30, 0x0034_8b5c, 0x0034_8b78)
    })
}

/// `void Fence_Save(Fence* self, Stream* stream)` (0x34d960): 0x24 bytes at
/// `+0x1c`.
pub fn fence_save(c: &mut Cpu) {
    guest(c, |c| {
        save_field_block(c, callee::WORLD_OBJ_SAVE, 0x1c, 0x24, 0x0034_d97c, 0x0034_d998)
    })
}

/// `void MeshAnim_Save(MeshAnim* self, Stream* stream)` (0x352a58): 0x0c
/// bytes at `+0x1c`.
pub fn mesh_anim_save(c: &mut Cpu) {
    guest(c, |c| {
        save_field_block(c, callee::WORLD_OBJ_SAVE, 0x1c, 0x0c, 0x0035_2a74, 0x0035_2a90)
    })
}

/// `void RailMan_Save(SyncProcess* self, Stream* stream)` (0x358c30): 0x200
/// bytes at `+0x18`.
pub fn rail_man_save(c: &mut Cpu) {
    guest(c, |c| {
        save_field_block(c, callee::SYNC_PROCESS_SAVE, 0x18, 0x200, 0x0035_8c4c, 0x0035_8c68)
    })
}

/// `void AnimDelta_Save(AnimDelta* self, Stream* stream)` (0x359410): 4 bytes
/// at `+0x6c`.
pub fn anim_delta_save(c: &mut Cpu) {
    guest(c, |c| {
        save_field_block(c, callee::ANIM_OBJECT_SAVE, 0x6c, 4, 0x0035_942c, 0x0035_9448)
    })
}

/// `int cFlagManager_GroupMatches(cFlagManager* self, const void* data,
/// cFlagContext* ctx)` (0x34afe8): true when the group is live, its id matches
/// and the flag bytes compare equal.
pub fn c_flag_manager_group_matches(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        let this = r32(c, A0);
        c.g[A3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(this).off(0x5c).lw(c);
        let dead = c.g[V0][0] == 0;
        c.g[V0][0] = 0;
        if !dead {
            c.cycles += 5;
            c.g[V0][0] = Addr(r32(c, A2)).off(0x80).lw(c);
            c.g[A0][0] = Addr(this).off(0x58).lw(c);
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            if c.g[V1][0] != c.g[A0][0] {
                c.g[V0][0] = 0;
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[A1][0];
                c.g[A2][0] = 0x58;
                c.g[A1][0] = c.g[A3][0];
                call_ptr(c, 0x0041_5fc8, 0x0034_b024)?;
                c.g[V0][0] = (c.g[V0][0] < 1) as u64;
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ModObj_SetRailModifier(ModObj* self, RailModifier* item)` (0x3556a8):
/// create the modifier block when absent, then push `item` onto the rail
/// modifier list.
pub fn mod_obj_set_rail_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(this).off(0x1c).lw(c);
        let present = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if !present {
            c.cycles += 2;
            callee::MOD_OBJ_CREATE_MODIFIER_BLOCK.call(c, 0x0035_56d0)?;
        }
        c.cycles += 9;
        c.g[A0][0] = Addr(this).off(0x1c).lw(c);
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1c));
        callee::RAIL_MODIFIER_LIST_PUSH_FRONT.call(c, 0x0035_56e0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Interrupts and textures.

/// `void IntcTable_InstallVifVuHandlers(IntcTable* self)` (0x361e30):
/// install the two 0x362340/0x3623a8 handlers for the VIF0 and VU0 interrupts.
pub fn intc_table_install_vif_vu_handlers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0x360000;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = sx(0x0036_2340);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 5;
        c.g[A3][0] = 0;
        callee::INTC_TABLE_ADD_HANDLER.call(c, 0x0036_1e54)?;
        c.g[A2][0] = 0x360000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = sx(0x0036_23a8);
        c.g[A1][0] = 7;
        c.g[A3][0] = 0;
        callee::INTC_TABLE_ADD_HANDLER.call(c, 0x0036_1e6c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void TexMan_FreeBTexInfo(TexMan* self, int slot)` (0x3672c0): free the
/// BTexInfo in `slots[slot]` and decrement the live count.
pub fn tex_man_free_btex_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = sx(r32(c, A1) << 2);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(r32(c, S1).wrapping_add(8));
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, A1)));
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        call_ptr(c, 0x0031_7e50, 0x0036_72e8)?;
        Addr(r32(c, S0)).sw(c, 0);
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        Addr(r32(c, S1)).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, vtable, LOG_RESULTS};
    use crate::difftest::{check, put_words, Rng, DATA, GAME_GP, RA as TEST_RA};

    const SEEDS: u32 = 150;

    /// Scrambled registers, the game's `gp`, the fake call hooks (for calls
    /// through pointer slots) and no forced unwind.
    fn base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        machine(c, seed, salt, 0)
    }

    /// A `gp`-relative global address.
    fn gp_at(off: i32) -> u32 {
        GAME_GP.wrapping_add(off as u32)
    }

    /// Zero `words` words at `at`.
    fn clear(c: &mut Cpu, at: u32, words: u32) {
        for i in 0..words {
            c.bus.write32(at + 4 * i, 0);
        }
    }

    /// Call a generated helper as a setup step (arguments in a0..t1).
    fn step(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = TEST_RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, TEST_RA, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// The main heap's arena over 256 KiB of test RAM.
    fn alloc_ready(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        c.g[GP][0] = GAME_GP as u64;
        step(
            c,
            0x0031_ff60,
            ssx3_ee::s10::f_0031ff60,
            &[MEM_HEAP as u64, (DATA + 0x1_0000) as u64, 0x4_0000],
        );
        c.bus.write32(GAME_GP + 0xdd8, 1); // size-class pools ready
    }

    /// The BE library singleton: a zeroed dummy so `cBE_getBE` does not build
    /// the real one from disc.
    fn be_library(c: &mut Cpu) {
        c.bus.write32(gp_at(-0x1f04), DATA + 0x3f000);
    }

    /// The MC-manager and UI-manager singletons: zeroed dummies, so the
    /// callees that fetch them do not construct (and allocate) the real ones.
    fn dummy_singletons(c: &mut Cpu) {
        c.bus.write32(gp_at(-0x47c), DATA + 0x3e000);
        c.bus.write32(gp_at(-0x10c8), DATA + 0x3d000);
        clear(c, DATA + 0x3d000, 0x80);
        clear(c, DATA + 0x3e000, 0x200);
    }

    /// A stream whose first word is a vtable with slot `off` pointing at the
    /// fake callee.
    fn fake_stream(c: &mut Cpu, r: &mut Rng, stream: u32, off: u32) {
        clear(c, stream, 0x40);
        let vt = vtable(c, r, &[off]);
        c.bus.write32(stream, vt);
    }

    // ------------------------------------------------------------------
    // Front end / profile save.

    #[test]
    fn ov_screen_profile_save_pda_v26_matches() {
        check(
            ee::s05::f_00211220__generated,
            ov_screen_profile_save_pda_v26,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2112_20, 3);
                dummy_singletons(c);
                let this = DATA + 0x100;
                clear(c, this, 0x300);
                c.bus.write32(gp_at(-0x848), 0);
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x1c0, if seed % 2 == 0 { 0 } else { r.next_u32() });
                c.bus
                    .write32(this + 0x214, if seed % 3 == 0 { 3 } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn mc_manager_format_slot_name_matches() {
        check(
            ee::s06::f_00241dc8__generated,
            mc_manager_format_slot_name,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x241d_c8, 0);
                clear(c, DATA, 0x100);
                // The wide-character table the name is read from.
                put_words(
                    c,
                    0x004a_2c28,
                    &[0x31 + (r.next_u32() % 26), 0x61 + (seed % 26), 0],
                );
                let dst = DATA + 0x800;
                clear(c, dst, 0x80);
                c.g[A0][0] = sx(dst);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (seed % 40) as u64;
            },
        );
    }

    #[test]
    fn stats_process_dtor_matches() {
        check(
            ee::s06::f_002449a8__generated,
            stats_process_dtor,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2449_a8, 2);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x40);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
                c.bus.write32(this + 0x10, if seed % 3 == 0 { 0 } else { DATA + 0x1_2000 });
            },
        );
    }

    // ------------------------------------------------------------------
    // Sections and game state.

    #[test]
    fn section_man_init_node_pool_matches() {
        check(
            ee::s05::f_0022e228__generated,
            section_man_init_node_pool,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x22e2_28);
                let this = DATA + 0x1000;
                clear(c, this, 0x80);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn game_state_finish_cinematic_update_matches() {
        check(
            ee::s06::f_00236f40__generated,
            game_state_finish_cinematic_update,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x236f_40, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x20);
                c.g[A0][0] = sx(this);
                clear(c, DATA + 0x400, 0x40);
                c.bus.write32(gp_at(-0x84c), if seed % 3 == 0 { 0 } else { DATA + 0x400 });
                c.bus.write32(LOG_RESULTS, 5);
            },
        );
    }

    // ------------------------------------------------------------------
    // Video / buddies / login.

    #[test]
    fn vid_engine_player_ctor_matches() {
        check(
            ee::s06::f_002533c8__generated,
            vid_engine_player_ctor,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2533_c8, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x20);
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x34, r.edge_f32());
                c.bus.write32(GAME_GP + 0x2898, 0);
            },
        );
    }

    #[test]
    fn buddy_man_destroy_matches() {
        check(
            ee::s06::f_00261008__generated,
            buddy_man_destroy,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2610_08, 2);
                let buddy = DATA + 0x100;
                clear(c, buddy, 0x120);
                c.bus.write32(gp_at(0x238), if seed % 2 == 0 { 0 } else { buddy });
            },
        );
    }

    #[test]
    fn buddy_man_update_matches() {
        check(
            ee::s06::f_00261490__generated,
            buddy_man_update,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2614_90, 2);
                let this = DATA + 0x100;
                clear(c, this, 0x120);
                c.g[A0][0] = sx(this);
                // +0xc8 is the login object the update drives; keep it null
                // (the login update allocates and talks to the network).
            },
        );
    }

    #[test]
    fn c_shared_login_man_set_account_name_matches() {
        check(
            ee::s07::f_00265f18__generated,
            c_shared_login_man_set_account_name,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x265f_18, 0);
                clear(c, 0x0053_6740, 0x40);
                let name = DATA + 0x100;
                clear(c, name, 0x20);
                c.bus.write8(name, b'A' + (seed % 26) as u8);
                c.bus.write8(name + 1, b'a' + (seed % 26) as u8);
                c.bus.write8(name + 2, 0);
                c.g[A0][0] = sx(DATA + 0x200);
                c.g[A1][0] = sx(name);
            },
        );
    }

    #[test]
    fn c_shared_login_man_set_account_password_matches() {
        check(
            ee::s07::f_00265f68__generated,
            c_shared_login_man_set_account_password,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x265f_68, 0);
                clear(c, 0x0053_6740, 0x40);
                let name = DATA + 0x100;
                clear(c, name, 0x20);
                c.bus.write8(name, b'0' + (seed % 10) as u8);
                c.bus.write8(name + 1, 0);
                c.g[A0][0] = sx(DATA + 0x200);
                c.g[A1][0] = sx(name);
            },
        );
    }

    // ------------------------------------------------------------------
    // Replay.

    #[test]
    fn replay_enable_instant_replay_cameras_matches() {
        check(
            ee::s07::f_0026ef30__generated,
            replay_enable_instant_replay_cameras,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x26ef_30, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x200);
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x634, [0, 8, 9, 0xffff_ffff][(seed % 4) as usize]);
                // The camera trigger manager reads this global.
                clear(c, 0x004c_5830, 0x40);
            },
        );
    }

    #[test]
    fn replay_save_emit_matches() {
        check(
            ee::s07::f_00271058__generated,
            replay_save_emit,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x2710_58);
                let this = DATA + 0x100;
                clear(c, this, 0x200);
                let data = DATA + 0x400;
                for i in 0..0x40u32 {
                    c.bus.write8(data + i, r.next_u32() as u8);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(data);
                c.g[A2][0] = [0u64, 1, 8, 0x40][(seed % 4) as usize];
                c.bus.write32(this + 0x680, r.next_u32());
                c.bus.write32(this + 0x684, r.next_u32());
                c.bus.write32(this + 0x678, r.next_u32());
            },
        );
    }

    #[test]
    fn replay_count_save_snapshots_matches() {
        check(
            ee::s07::f_002712f8__generated,
            replay_count_save_snapshots,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x2712_f8);
                let this = DATA + 0x100;
                clear(c, this, 0x700);
                // A chain of 0..4 snapshots; `skip` is one of them.
                let n = seed % 5;
                let nodes: Vec<u32> = (0..n).map(|i| DATA + 0x200 + 0x20 * i).collect();
                for (i, &node) in nodes.iter().enumerate() {
                    let next = nodes.get(i + 1).copied().unwrap_or(0);
                    c.bus.write32(node + 0x14, next);
                    c.bus.write32(node + 4, r.next_u32());
                }
                c.bus.write32(this + 0x3b0, nodes.first().copied().unwrap_or(0));
                c.bus
                    .write32(this + 0x3d0, nodes.get((seed as usize) % (n as usize + 1)).copied().unwrap_or(0));
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn replay_save_write_footer_matches() {
        check(
            ee::s07::f_00271708__generated,
            replay_save_write_footer,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x2717_08);
                let this = DATA + 0x100;
                clear(c, this, 0x700);
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x638, r.next_u32());
                c.bus
                    .write32(this + 0x640, if seed % 3 == 0 { 0 } else { DATA + 0x900 });
                clear(c, DATA + 0x900, 0x40);
            },
        );
    }

    // ------------------------------------------------------------------
    // Script engine.

    #[test]
    fn script_list_on_seq_stopped_matches() {
        check(
            ee::s07::f_00277400__generated,
            script_list_on_seq_stopped,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2774_00, 2);
                let list = DATA + 0x100;
                clear(c, list, 0x100);
                // Playing only when the list sits in play mode 3.
                c.bus.write32(list + 0xa4, if seed % 2 == 0 { 0 } else { 3 });
                clear(c, DATA + 0x400, 0x40);
                c.bus.write32(gp_at(-0x84c), 0);
                c.g[A0][0] = sx(list);
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = 0;
            },
        );
    }

    #[test]
    fn script_engine_stop_main_lists_matches() {
        check(
            ee::s07::f_002790f8__generated,
            script_engine_stop_main_lists,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2790_f8, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x600);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn script_engine_find_free_bank_slot_matches() {
        check(
            ee::s07::f_0027b528__generated,
            script_engine_find_free_bank_slot,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x27b5_28);
                let this = DATA + 0x100;
                clear(c, this, 0x200);
                // The probes are at +0x648, 0x10 bytes apart, for slots 2..9:
                // `+8` is the busy flag, `+0` the "in use" word.
                let taken = if seed % 7 == 0 { 0 } else { 2 + seed % 8 };
                for i in 2..10u32 {
                    let slot = this + 0x648 + 0x10 * (i - 2);
                    let is_target = i == taken;
                    // The slot just before the target is idle but in use, so
                    // the inner "if in use -> not free" path is taken too.
                    let idle = is_target || i + 1 == taken;
                    c.bus.write32(slot + 8, if idle { 0 } else { 1 });
                    c.bus.write32(slot, if is_target { 0 } else { 1 });
                    c.bus.write32(slot + 4, r.next_u32());
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn script_dummy_obj_ctor_matches() {
        check(
            ee::s07::f_0027bfd0__generated,
            script_dummy_obj_ctor,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x27bf_d0, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x80);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn script_view_obj_accept_command_matches() {
        check(
            ee::s07::f_00281a00__generated,
            script_view_obj_accept_command,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x281a_00, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x120);
                c.bus.write32(this + 0xf8, r.next_u32());
                let arg = DATA + 0x300;
                clear(c, arg, 0x20);
                c.bus.write32(arg + 8, r.next_u32());
                clear(c, DATA + 0x400, 0x40);
                c.bus.write32(gp_at(-0x84c), if seed % 2 == 0 { 0 } else { DATA + 0x400 });
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(arg);
                c.g[A2][0] = [4u64, 3, 4, 5][(seed % 4) as usize];
            },
        );
    }

    // ------------------------------------------------------------------
    // Audio.

    #[test]
    fn ssx_audio_system_play_dj_song_intro_matches() {
        check(
            ee::s08::f_002a3170__generated,
            ssx_audio_system_play_dj_song_intro,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2a31_70, 2);
                let this = DATA + 0x100;
                clear(c, this, 0x1620);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn ssx_audio_system_is_race_type6_to11_matches() {
        race_type_test(
            ee::s08::f_002a42c8__generated,
            ssx_audio_system_is_race_type6_to11,
            0x2a42_c8,
        );
    }

    #[test]
    fn ssx_audio_system_is_race_type6_to8_matches() {
        race_type_test(ee::s08::f_002a4318__generated, ssx_audio_system_is_race_type6_to8, 0x2a43_18);
    }

    #[test]
    fn ssx_audio_system_is_race_type9_to11_matches() {
        race_type_test(
            ee::s08::f_002a4368__generated,
            ssx_audio_system_is_race_type9_to11,
            0x2a43_68,
        );
    }

    /// The three race-type queries: the BE library dummy, an empty BE table
    /// (the interface call lands in the fake hook) and every race type.
    fn race_type_test(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, SEEDS, &move |c, seed| {
            let _ = machine(c, seed, salt, 0);
            be_library(c);
            clear(c, 0x0053_08a8, 0x20);
            c.bus.write8(0x0053_5c12, (seed % 20) as u8);
        });
    }

    #[test]
    fn ps2_sound_manager_shutdown_matches() {
        check(
            ee::s08::f_002a8f50__generated,
            ps2_sound_manager_shutdown,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2a8f_50, 2);
                let this = DATA + 0x100;
                clear(c, this, 0x40);
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 4, if seed % 2 == 0 { 0 } else { DATA + 0x1_2000 });
            },
        );
    }

    #[test]
    fn bank_monitor_stop_all_matches() {
        // 64 reset calls per seed: keep the seed count (and the run time) down.
        check(
            ee::s08::f_002ad3c0__generated,
            bank_monitor_stop_all,
            30,
            &|c, seed| {
                let _ = machine(c, seed, 0x2ad3_c0, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x1000);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn path_plat_track_init_matches() {
        check(
            ee::s08::f_002ae138__generated,
            path_plat_track_init,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x2ae1_38);
                let this = DATA + 0x100;
                for i in 0..0x20u32 {
                    c.bus.write32(this + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    // ------------------------------------------------------------------
    // Transitions and lights.

    #[test]
    fn c_transition_man_start_out_matches() {
        check(
            ee::s09::f_002e44f0__generated,
            c_transition_man_start_out,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e44_f0, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x200);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x300);
                c.g[A2][0] = (seed % 4) as u64;
                c.f[12] = r.edge_f32();
                c.bus.write32(this + 0x44, if seed % 2 == 0 { 0 } else { 1 });
                clear(c, DATA + 0x300, 0x40);
            },
        );
    }

    #[test]
    fn world_light_man_free_overlap_lists_matches() {
        check(
            ee::s09::f_002f53b0__generated,
            world_light_man_free_overlap_lists,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2f53_b0, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x300);
                for i in 0..8u32 {
                    let slot = this + 0x40 + 0x40 * i;
                    c.bus.write32(slot, if seed % 3 == 0 { 0 } else { DATA + 0x1_4000 + 0x10 * i });
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    // ------------------------------------------------------------------
    // WScript.

    #[test]
    fn wscript_process_terminate_matches() {
        check(
            ee::s10::f_00307d78__generated,
            wscript_process_terminate,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x307d_78, 2);
                let this = DATA + 0x100;
                clear(c, this, 0x40);
                c.bus.write32(this + 0x10, DATA + 0x400);
                clear(c, DATA + 0x400, 0x60);
                c.bus.write32(this + 0x1c, if seed % 2 == 0 { 0 } else { DATA + 0x500 });
                clear(c, DATA + 0x500, 0x20);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn wscript_task_ctor_matches() {
        check(
            ee::s10::f_00308228__generated,
            wscript_task_ctor,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x3082_28);
                let this = DATA + 0x100;
                for i in 0..0x20u32 {
                    c.bus.write32(this + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn wscript_man_unload_section_and_free_matches() {
        check(
            ee::s10::f_00308fe0__generated,
            wscript_man_unload_section_and_free,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x308f_e0, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x400);
                let cache = DATA + 0x600;
                clear(c, cache, 0x40);
                c.bus.write32(this + 0x28c, cache);
                c.bus.write32(cache + 4, cache + 0x10);
                c.bus.write32(cache + 0x14, DATA + 0x700);
                clear(c, DATA + 0x700, 0x40);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 8) as u64;
            },
        );
    }

    #[test]
    fn wscript_man_get_current_mission_matches() {
        check(
            ee::s10::f_0030aeb8__generated,
            wscript_man_get_current_mission,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30ae_b8, 2);
                let this = DATA + 0x100;
                clear(c, this, 0x300);
                c.g[A0][0] = sx(this);
                let mission = DATA + 0x400;
                clear(c, mission, 0x80);
                c.bus.write32(this + 0x2a4, if seed % 2 == 0 { 0 } else { mission });
                // The mission's vtable slot: `lh(+0x18)` delta, `lw(+0x1c)` fn.
                let vt = vtable(c, &mut r, &[0x18]);
                c.bus.write32(mission + 0x5c, vt);
                c.bus.write32(LOG_RESULTS, if seed % 4 < 2 { 0x1234 } else { 0 });
            },
        );
    }

    #[test]
    fn wscript_process_list_push_back_matches() {
        check(
            ee::s10::f_0030b270__generated,
            wscript_process_list_push_back,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x30b2_70);
                let list = DATA + 0x100;
                clear(c, list, 0x40);
                // A chain of 0..3 nodes at +0x18.
                let n = seed % 4;
                let mut prev = 0;
                let mut first = 0;
                for i in 0..n {
                    let node = DATA + 0x200 + 0x20 * i;
                    clear(c, node, 8);
                    if i == 0 {
                        first = node;
                    } else {
                        c.bus.write32(prev + 0x18, node);
                    }
                    prev = node;
                }
                c.bus.write32(list, first);
                let node = DATA + 0x500;
                clear(c, node, 8);
                c.bus.write32(node + 0x18, r.next_u32());
                c.g[A0][0] = sx(list);
                c.g[A1][0] = sx(node);
            },
        );
    }

    #[test]
    fn wscript_process_list_remove_matches() {
        check(
            ee::s10::f_0030b2d0__generated,
            wscript_process_list_remove,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x30b2_d0);
                let list = DATA + 0x100;
                clear(c, list, 0x40);
                // A chain of 0..3 nodes; remove the head, one of the others,
                // an absent node, or from an empty list.
                let n = seed % 4;
                let mut nodes = Vec::new();
                let mut prev = 0;
                for i in 0..n {
                    let node = DATA + 0x200 + 0x20 * i;
                    clear(c, node, 8);
                    if i == 0 {
                        c.bus.write32(list, node);
                    } else {
                        c.bus.write32(prev + 0x18, node);
                    }
                    prev = node;
                    nodes.push(node);
                }
                let victim = match seed % 5 {
                    0 => nodes.first().copied().unwrap_or(DATA + 0x600),
                    1 => nodes.last().copied().unwrap_or(DATA + 0x600),
                    2 => *nodes.get(1).unwrap_or(&(DATA + 0x600)),
                    3 => DATA + 0x600,
                    _ => nodes.first().copied().unwrap_or(DATA + 0x600),
                };
                clear(c, DATA + 0x600, 8);
                c.bus.write32(DATA + 0x600 + 0x18, r.next_u32());
                c.g[A0][0] = sx(list);
                c.g[A1][0] = sx(victim);
            },
        );
    }

    #[test]
    fn wscript_man_reset_collect_set_matches() {
        check(
            ee::s10::f_0030c390__generated,
            wscript_man_reset_collect_set,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x30c3_90);
                let this = DATA + 0x100;
                for i in 0..0x50u32 {
                    c.bus.write32(this + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    // ------------------------------------------------------------------
    // Bx strings and spheres.

    #[test]
    fn c_bx_string_set_at_matches() {
        check(
            ee::s10::f_00318e68__generated,
            c_bx_string_set_at,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x318e_68, 1);
                alloc_ready(c);
                let str_ = DATA + 0x100;
                let buf = DATA + 0x1000;
                clear(c, str_, 0x10);
                c.bus.write32(str_, buf);
                for i in 0..0x20u32 {
                    c.bus.write8(buf + i, r.next_u32() as u8 | 1);
                }
                // The buffer's reference count sits 0xc bytes before it: 0/1
                // keeps the buffer (the writable path).
                c.bus.write32(buf - 0xc, (seed % 2) as u32);
                c.bus.write32(buf - 8, 0x10);
                c.g[A0][0] = sx(str_);
                c.g[A1][0] = (seed % 0x10) as u64;
                c.g[A2][0] = [0x41u64, 0x7f, 0x81, 0x100][(seed % 4) as usize];
            },
        );
    }

    #[test]
    fn rider_sphere_tree_set_spheres_matches() {
        check(
            ee::s10::f_00329a90__generated,
            rider_sphere_tree_set_spheres,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x329a_90);
                let this = DATA + 0x100;
                clear(c, this, 0x400);
                let spheres = DATA + 0x1000;
                let radii = DATA + 0x2000;
                for i in 0..6u32 {
                    for k in 0..4 {
                        c.bus.write32(spheres + 0x10 * i + 4 * k, r.next_u32());
                        c.bus.write32(radii + 4 * (4 * i + k), r.edge_f32());
                    }
                }
                let center = DATA + 0x3000;
                for k in 0..4 {
                    c.bus.write32(center + 4 * k, r.next_u32());
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = [-1i64 as u64, 0, 1, 3, 6][(seed % 5) as usize];
                c.g[A2][0] = sx(spheres);
                c.g[A3][0] = sx(radii);
                c.g[T0][0] = sx(center);
                c.f[12] = r.edge_f32();
            },
        );
    }

    #[test]
    fn sphere_tree_reset_transform_matches() {
        check(
            ee::s10::f_0032c540__generated,
            sphere_tree_reset_transform,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x32c5_40);
                let this = DATA + 0x100;
                clear(c, this, 0x100);
                let table = 0x004f_f640u32;
                for i in 0..8u32 {
                    for k in 0..4 {
                        c.bus.write32(table + 0x10 * i + 4 * k, r.next_u32());
                    }
                }
                for k in 0..4 {
                    c.bus.write32(0x004f_f130 + 4 * k, r.next_u32());
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    // ------------------------------------------------------------------
    // World object save family.

    /// A save-function test: a dummy object, a stream whose vtable slot 8 is a
    /// fake serialiser, and the type's `save` prelude reading the stream.
    fn save_test(generated: GuestFn, hand: GuestFn, salt: u64, off: u32) {
        check(generated, hand, SEEDS, &move |c, seed| {
            let mut r = machine(c, seed, salt, 0);
            let this = DATA + 0x100;
            let stream = DATA + 0x400;
            clear(c, this, 0x300);
            fake_stream(c, &mut r, stream, 8);
            c.bus.write32(this + off, if seed % 3 == 0 { 0 } else { DATA + 0x800 });
            clear(c, DATA + 0x800, 0x40);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(stream);
        });
    }

    #[test]
    fn boost_save_matches() {
        save_test(ee::s11::f_00341a50__generated, boost_save, 0x341a_50, 0x20);
    }

    #[test]
    fn conveyor_save_matches() {
        save_test(ee::s11::f_00343718__generated, conveyor_save, 0x3437_18, 0x1c);
    }

    #[test]
    fn counter_save_matches() {
        save_test(ee::s11::f_00343a18__generated, counter_save, 0x343a_18, 0x1c);
    }

    #[test]
    fn flex_rail_save_matches() {
        save_test(ee::s11::f_00348b40__generated, flex_rail_save, 0x348b_40, 0x20);
    }

    #[test]
    fn fence_save_matches() {
        save_test(ee::s11::f_0034d960__generated, fence_save, 0x34d9_60, 0x1c);
    }

    #[test]
    fn mesh_anim_save_matches() {
        save_test(ee::s11::f_00352a58__generated, mesh_anim_save, 0x352a_58, 0x1c);
    }

    #[test]
    fn rail_man_save_matches() {
        save_test(ee::s11::f_00358c30__generated, rail_man_save, 0x358c_30, 0x18);
    }

    #[test]
    fn anim_delta_save_matches() {
        save_test(ee::s11::f_00359410__generated, anim_delta_save, 0x3594_10, 0x6c);
    }

    #[test]
    fn c_flag_manager_group_matches_matches() {
        check(
            ee::s11::f_0034afe8__generated,
            c_flag_manager_group_matches,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34af_e8, 1);
                let this = DATA + 0x100;
                let data = DATA + 0x400;
                let ctx = DATA + 0x600;
                clear(c, this, 0x80);
                clear(c, data, 0x20);
                clear(c, ctx, 0x90);
                c.bus.write32(this + 0x5c, if seed % 3 == 0 { 0 } else { 1 });
                c.bus.write32(this + 0x58, r.next_u32());
                let group = c.bus.read32(this + 0x58);
                c.bus.write32(ctx + 0x80, DATA + 0x700);
                c.bus
                    .write32(DATA + 0x700, if seed % 3 == 1 { group } else { r.next_u32() });
                // The 0x58-byte flag block lives at `self`, so the copy at
                // `data` compares equal on some seeds.
                for i in 0..0x16u32 {
                    let v = if seed % 4 == 0 { 0 } else { r.next_u32() };
                    c.bus.write32(this + 4 * i, v);
                    c.bus.write32(data + 4 * i, v);
                }
                if seed % 4 != 0 {
                    c.bus.write32(data + 0x30, r.next_u32());
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(data);
                c.g[A2][0] = sx(ctx);
            },
        );
    }

    #[test]
    fn mod_obj_set_rail_modifier_matches() {
        check(
            ee::s11::f_003556a8__generated,
            mod_obj_set_rail_modifier,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x3556_a8, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x60);
                c.bus.write32(this + 0x1c, if seed % 2 == 0 { 0 } else { DATA + 0x800 });
                clear(c, DATA + 0x800, 0x60);
                clear(c, DATA + 0x900, 0x40);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x900);
            },
        );
    }

    // ------------------------------------------------------------------
    // Interrupts and textures.

    #[test]
    fn intc_table_install_vif_vu_handlers_matches() {
        check(
            ee::s12::f_00361e30__generated,
            intc_table_install_vif_vu_handlers,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x361e_30, 2);
                let this = DATA + 0x100;
                clear(c, this, 0x800);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn tex_man_free_btex_info_matches() {
        check(
            ee::s12::f_003672c0__generated,
            tex_man_free_btex_info,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3672_c0, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x200);
                c.bus.write32(this, r.next_u32());
                for i in 0..0x10u32 {
                    c.bus.write32(this + 8 + 4 * i, if seed % 3 == 0 { 0 } else { DATA + 0x1_1000 + 0x10 * i });
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 0x10) as u64;
            },
        );
    }
}
