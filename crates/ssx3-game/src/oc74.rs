//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for front-end state constructors, destructors, lobby/login
//! managers, the replay cache/stream and the script engine's small accessors,
//! each with a differential test against the generated body.
//!
//! Most of these are one-liners in the original: a constructor installs a
//! vtable and a state id after calling its base constructor, a destructor
//! runs the shutdown/close path and then `operator delete` when the low bit
//! of the flag argument is set, and the accessors test/reload a couple of
//! fields. The replacements keep the frame layout, the cycle counts and every
//! register the originals leave behind; calls use [`crate::call`] and the
//! three loops follow the event rules of [`crate::events`].

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0015_4630,
        be_mission_int_session_type_at_least,
        "BEMissionInt_SessionTypeAtLeast",
    ),
    (0x0015_a7d0, rewards_file_dtor, "RewardsFile_dtor"),
    (
        0x0017_4190,
        bounded_camera_algorithm_ctor,
        "BoundedCameraAlgorithm_ctor",
    ),
    (
        0x0018_4b70,
        fe_state_uber_trick_shutdown,
        "cFEStateUberTrick_Shutdown",
    ),
    (
        0x0018_6610,
        fe_state_event_select_ctor,
        "cFEStateEventSelect_ctor",
    ),
    (
        0x0018_7318,
        fe_state_mem_card_finish,
        "cFEStateMemCard_Finish",
    ),
    (
        0x0018_c658,
        fe_state_options_controller_ctor,
        "cFEStateOptionsController_ctor",
    ),
    (
        0x0019_4a60,
        fe_state_main_menu_ctor,
        "cFEStateMainMenu_ctor",
    ),
    (0x0019_5a50, fe_locale_unload_table, "FELocale_UnloadTable"),
    (
        0x001a_d130,
        fe_state_select_persona_ctor,
        "cFEStateSelectPersona_ctor",
    ),
    (
        0x001a_f3e0,
        fe_state_challenge_popup_v33,
        "cFEStateChallengePopup_v33",
    ),
    (
        0x001b_0628,
        fe_state_challenge_received_popup_v33,
        "cFEStateChallengeReceivedPopup_v33",
    ),
    (
        0x001b_1590,
        fe_state_online_character_select_handle_input,
        "cFEStateOnlineCharacterSelect_HandleInput",
    ),
    (
        0x001b_a3e0,
        fe_state_create_step2_ctor,
        "cFEStateCreateStep2_ctor",
    ),
    (
        0x001c_1038,
        fe_state_net_quick_play_popup_update,
        "cFEStateNetQuickPlayPopup_Update",
    ),
    (
        0x001e_3a30,
        message_center_is_flag_set,
        "MessageCenter_IsFlagSet",
    ),
    (
        0x001f_f700,
        ov_screen_reward_list_on_widget_action,
        "OVScreenRewardList_OnWidgetAction",
    ),
    (0x0020_06b8, event_select_ctor, "EventSelect_ctor"),
    (
        0x0020_a6a8,
        ov_state_overlay_on_widget_action,
        "cOVStateOverlay_OnWidgetAction",
    ),
    (
        0x0022_d390,
        section_man_is_event_ready,
        "SectionMan_IsEventReady",
    ),
    (
        0x0023_6fe8,
        game_state_replay_playback_enter,
        "GameStateReplayPlayback_Enter",
    ),
    (0x0023_e4f8, mc_manager_state02, "MCManager_State02"),
    (
        0x0024_1138,
        mc_manager_write_save_chunk,
        "MCManager_WriteSaveChunk",
    ),
    (0x0025_3418, vid_engine_player_dtor, "VidEnginePlayer_dtor"),
    (
        0x0025_a730,
        lobby_man_lookup_peer_addr,
        "LobbyMan_LookupPeerAddr",
    ),
    (
        0x0025_aa98,
        lobby_man_destroy_lobby_api,
        "LobbyMan_DestroyLobbyApi",
    ),
    (0x0025_b608, lobby_man_send_request, "LobbyMan_SendRequest"),
    (0x0025_b800, lobby_man_disconnect, "LobbyMan_Disconnect"),
    (0x0026_5908, shared_login_man_dtor, "cSharedLoginMan_dtor"),
    (
        0x0026_5e80,
        shared_login_man_do_create,
        "cSharedLoginMan_DoCreate",
    ),
    (0x0026_6d10, voip_man_dtor, "cVoipMan_dtor"),
    (0x0026_6e88, voip_man_deactivate, "cVoipMan_Deactivate"),
    (0x0026_b598, path_data_has_marker, "PathData_HasMarker"),
    (0x0026_d0e8, replay_cache_dtor, "ReplayCache_dtor"),
    (
        0x0026_e968,
        replay_stream_free_buffer,
        "ReplayStream_FreeBuffer",
    ),
    (0x0026_ea20, replay_stream_read, "ReplayStream_Read"),
    (
        0x0027_0670,
        replay_end_instant_replay,
        "Replay_EndInstantReplay",
    ),
    (
        0x0027_0f78,
        replay_free_save_buffers,
        "Replay_FreeSaveBuffers",
    ),
    (0x0027_3970, in_seq_channel_dtor, "InSeq_Channel_dtor"),
    (
        0x0027_3a60,
        in_seq_channel_select_block,
        "InSeq_ChannelSelectBlock",
    ),
    (
        0x0027_5650,
        script_anim_bank_man_remove_bank,
        "ScriptAnimBankMan_RemoveBank",
    ),
    (
        0x0027_5718,
        script_anim_bank_man_find_bank,
        "ScriptAnimBankMan_FindBank",
    ),
    (
        0x0027_8d58,
        script_engine_is_load_done,
        "ScriptEngine_IsLoadDone",
    ),
    (
        0x0027_8da0,
        script_engine_is_active_list_skippable,
        "ScriptEngine_IsActiveListSkippable",
    ),
    (
        0x0027_91d8,
        script_engine_list_get_state,
        "ScriptEngine_ListGetState",
    ),
    (
        0x0027_9298,
        script_engine_get_active_list_field_b0,
        "ScriptEngine_GetActiveListFieldB0",
    ),
    (
        0x0027_9370,
        script_engine_is_playing_script,
        "ScriptEngine_IsPlayingScript",
    ),
    (
        0x0027_9840,
        script_engine_cancel_script_bank,
        "ScriptEngine_CancelScriptBank",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const REWARDS_FILE_UNLOAD: Callee = Callee {
        addr: 0x0015_ae00,
        f: ee::s01::RewardsFile_Unload,
    };
    pub const BOUNDED_CAMERA_ALGORITHM_BASE_CTOR: Callee = Callee {
        addr: 0x0017_3208,
        f: ee::s02::BoundedCameraAlgorithmBase_ctor,
    };
    pub const NET_FE_STATE_CTOR: Callee = Callee {
        addr: 0x001a_8500,
        f: ee::s03::NetFEState_ctor,
    };
    pub const NET_FE_STATE_ON_CHILD_EVENT: Callee = Callee {
        addr: 0x001a_97b8,
        f: ee::s03::NetFEState_OnChildEvent,
    };
    pub const NET_FE_STATE_UPDATE: Callee = Callee {
        addr: 0x001a_8770,
        f: ee::s03::NetFEState_Update,
    };
    pub const SECTION_MAN_IS_EVENT_LOADED: Callee = Callee {
        addr: 0x0022_d278,
        f: ee::s05::SectionMan_IsEventLoaded,
    };
    pub const VID_ENGINE_PLAYER_CLOSE: Callee = Callee {
        addr: 0x0025_35f8,
        f: ee::s06::VidEnginePlayer_Close,
    };
    pub const LOBBY_MAN_DESTROY_LISTS: Callee = Callee {
        addr: 0x0025_a8b0,
        f: ee::s06::LobbyMan_DestroyLists,
    };
    pub const LOBBY_MAN_DESTROY_LOBBY_API: Callee = Callee {
        addr: 0x0025_aa98,
        f: ee::s06::LobbyMan_DestroyLobbyApi,
    };
    pub const LOBBY_MAN_DESTROY_PEER_TABLE: Callee = Callee {
        addr: 0x0025_a6f8,
        f: ee::s06::LobbyMan_DestroyPeerTable,
    };
    pub const SHARED_LOGIN_MAN_SHUTDOWN: Callee = Callee {
        addr: 0x0026_59c0,
        f: ee::s07::cSharedLoginMan_Shutdown,
    };
    pub const VOIP_MAN_STOP: Callee = Callee {
        addr: 0x0026_6df8,
        f: ee::s07::cVoipMan_Stop,
    };
    pub const REPLAY_CACHE_FREE: Callee = Callee {
        addr: 0x0026_d130,
        f: ee::s07::ReplayCache_Free,
    };
    pub const REPLAY_SNAPSHOT_EMPTY_HOOK_B: Callee = Callee {
        addr: 0x0026_ed90,
        f: ee::s07::Replay_SnapshotEmptyHookB,
    };
    pub const WORLD_STATE_RESTORE: Callee = Callee {
        addr: 0x0026_dbd0,
        f: ee::s07::WorldState_Restore,
    };
    pub const REPLAY_RESTORE_SNAPSHOT: Callee = Callee {
        addr: 0x0026_db88,
        f: ee::s07::Replay_RestoreSnapshot,
    };
    pub const REPLAY_FREE_SNAPSHOT: Callee = Callee {
        addr: 0x0027_0238,
        f: ee::s07::Replay_FreeSnapshot,
    };
    pub const REPLAY_END_PLAYBACK: Callee = Callee {
        addr: 0x0026_f980,
        f: ee::s07::Replay_EndPlayback,
    };
    pub const IN_SEQ_CHANNEL_RELEASE: Callee = Callee {
        addr: 0x0027_3a00,
        f: ee::s07::InSeq_ChannelRelease,
    };
    pub const IN_SEQ_HEAP_DELETE: Callee = Callee {
        addr: 0x0027_24e8,
        f: ee::s07::InSeq_HeapDelete,
    };
    pub const SCRIPT_ANIM_BANK_MAN_FIND_BANK: Callee = Callee {
        addr: 0x0027_5718,
        f: ee::s07::ScriptAnimBankMan_FindBank,
    };
    pub const SCRIPT_ENGINE_POLL_ASYNC_LOADS: Callee = Callee {
        addr: 0x0027_8918,
        f: ee::s07::ScriptEngine_PollAsyncLoads,
    };
    pub const SCRIPT_ENGINE_IS_BANK_SLOT_LOADED: Callee = Callee {
        addr: 0x0027_bf50,
        f: ee::s07::ScriptEngine_IsBankSlotLoaded,
    };
    pub const SCRIPT_LIST_IS_CURRENT_SKIPPABLE: Callee = Callee {
        addr: 0x0027_70c0,
        f: ee::s07::ScriptList_IsCurrentSkippable,
    };
    pub const SCRIPT_LIST_GET_STATE: Callee = Callee {
        addr: 0x0027_6b98,
        f: ee::s07::ScriptList_GetState,
    };
    pub const SCRIPT_LIST_GET_FIELD_B0: Callee = Callee {
        addr: 0x0027_72b8,
        f: ee::s07::ScriptList_GetFieldB0,
    };
    pub const SCRIPT_LIST_IS_PLAYING_SCRIPT: Callee = Callee {
        addr: 0x0027_71c8,
        f: ee::s07::ScriptList_IsPlayingScript,
    };
    pub const SCRIPT_ENGINE_FIND_BANK_SLOT_BY_ID: Callee = Callee {
        addr: 0x0027_b578,
        f: ee::s07::ScriptEngine_FindBankSlotById,
    };
    pub const SCRIPT_ENGINE_CANCEL_BANK_LOAD: Callee = Callee {
        addr: 0x0027_b4b8,
        f: ee::s07::ScriptEngine_CancelBankLoad,
    };

    // Called through the function table (`call_indirect` in the original).
    pub const UI_STATE_CTOR: u32 = 0x0039_e2a0;
    pub const UI_STATE_SHUTDOWN: u32 = 0x0039_e4a0;
    pub const UI_LIST_CLOSE_ALL_POPUPS: u32 = 0x0039_f698;
    pub const BUILTIN_DELETE: u32 = 0x0031_7e50;
    pub const EA_MEM_FREE: u32 = 0x0031_7e98;
    pub const FEMODULE_SET_PREVIEW_VISIBLE: u32 = 0x001a_0570;
    pub const SSX_APP_START_FE_MODULE: u32 = 0x0022_8238;
    pub const LOC_STRING_TABLE_DTOR: u32 = 0x002c_6d70;
    pub const BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;
    pub const BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;
    pub const BENEW_PLAYER_INT_IS_MESSAGE_FLAG_SET: u32 = 0x0014_7980;
    pub const BE_MESSAGE_FLAG_SET: u32 = 0x0015_8e30;
    pub const BE_GET_INTERFACE_RIDER: u32 = 0x0014_4bc0;
    pub const SCRIPT_MAN_LOAD: u32 = 0x0030_9030;
    pub const SCRIPT_MAN_UNLOAD: u32 = 0x0030_9f18;
    pub const WORLD_VIEW_SET: u32 = 0x001f_36d8;
    pub const MENU_SYS_EVENT: u32 = 0x0020_ab50;
    pub const REPLAY_STREAM_WRITE: u32 = 0x003e_6574;
    pub const LOBBY_MAN_REQUEST_UPDATE: u32 = 0x003e_80c0;
    pub const LOBBY_MAN_REQUEST_FREE: u32 = 0x003e_81c8;
    pub const LOBBY_API_ADD_REQUEST: u32 = 0x003e_8b10;
    pub const DIRTY_SOCK_CREATE: u32 = 0x003f_7f48;
    pub const DIRTY_SOCK_DESTROY: u32 = 0x003f_6b50;
    pub const VID_ENGINE_PLAYER_REQUEST: u32 = 0x003e_7218;
    pub const APP_SOMETHING: u32 = 0x0011_73b8;
}

// ---------------------------------------------------------------------------
// Small helpers.

/// The address `reg + off`.
#[inline(always)]
fn at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}

/// The address `base + off`.
#[inline(always)]
fn p(base: u32, off: u32) -> u32 {
    base.wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `sw reg, base + off`.
#[inline(always)]
fn sw(c: &mut Cpu, base: u32, off: u32, v: u64) {
    c.sw(p(base, off), v)
}

/// `sd`/`lq` frame teardown: `addiu sp, sp, size; jr ra`.
#[inline(always)]
fn leave(c: &mut Cpu, size: u32) {
    let ra = r32(c, RA);
    c.g[SP][0] = sx(sp(c).wrapping_add(size));
    c.pc = ra;
}

/// The constant an original builds with `lui reg, hi; addiu reg, reg, lo`
/// (the vtables): its 32-bit value, sign-extended.
#[inline(always)]
fn const_reg(c: &mut Cpu, reg: usize, hi: u32, lo: i32) {
    c.g[reg][0] = sx(hi.wrapping_add(lo as u32));
}

// ---------------------------------------------------------------------------
// BE mission accessor.

/// The 22 mission-session records `BEMissionInt_SessionTypeAtLeast` scans:
/// `{ int id; int pad[5]; int value; }` (0x2c bytes each).
const MISSION_SESSIONS: u32 = 0x0044_0770;
const MISSION_SESSION_COUNT: u32 = 22;
const MISSION_SESSION_SIZE: u32 = 0x2c;
const SESSION_ID: u32 = 0x00;
const SESSION_VALUE: u32 = 0x18;

/// `bool BEMissionInt_SessionTypeAtLeast(int value /*a1*/, int session_type
/// /*a2*/)` (0x154630): true when the session type's record is present and
/// its value is at least `value`.
pub fn be_mission_int_session_type_at_least(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = (MISSION_SESSIONS & 0xffff_0000) as u64;
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(MISSION_SESSIONS & 0xffff));
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(MISSION_SESSION_SIZE * MISSION_SESSION_COUNT));
    c.g[V0][0] = c.lw(r32(c, V1));
    loop {
        // `id == session_type`: is the record's value at least `value`?
        c.cycles += 2;
        if c.g[V0][0] != c.g[A2][0] {
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(MISSION_SESSION_SIZE));
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw(p(r32(c, V1), SESSION_ID + SESSION_VALUE));
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
            let too_small = c.g[V0][0] != 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(MISSION_SESSION_SIZE));
            if !too_small {
                c.cycles += 2;
                c.g[V0][0] = 1;
                abi::ret(c);
                return;
            }
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = c.lw(r32(c, V1));
            if event_due(c) {
                deliver_event_here(c, 0x0015_4640);
            }
            continue;
        }
        break;
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Rewards file: destructor.

/// `void RewardsFile_dtor(RewardsFile* self, int flags)` (0x15a7d0): unload
/// the file's reward table, then `operator delete` it when `flags & 1`.
pub fn rewards_file_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        callee::REWARDS_FILE_UNLOAD.call(c, 0x0015_a7ec)?;
        c.g[S0][0] = c.g[S0][0] & 1;
        let keep = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, callee::BUILTIN_DELETE, 0x0015_a800)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Camera algorithm.

/// `BoundedCameraAlgorithm* BoundedCameraAlgorithm_ctor(BoundedCameraAlgorithm*
/// self)` (0x174190): run the base constructor and install the vtable, state
/// id 0x5b and the two cleared link fields.
pub fn bounded_camera_algorithm_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::BOUNDED_CAMERA_ALGORITHM_BASE_CTOR.call(c, 0x0017_41a4)?;
        let this = r32(c, S0);
        c.g[V0][0] = 0x5b;
        c.g[V1][0] = 0x46_0000;
        sw(c, this, 0x0c, c.g[V0][0]);
        const_reg(c, V1, 0x46_0000, -0x3ba8);
        sw(c, this, 0x10, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        sw(c, this, 0x80, 0);
        sw(c, this, 0x74, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Front-end states.

/// `void cFEStateUberTrick_Shutdown(cFEStateUberTrick* self)` (0x184b70):
/// hide the rider preview when there is a rider manager, then shut the state
/// down.
pub fn fe_state_uber_trick_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.lw(gp(c, -0x848));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.lw(at(c, V0, 0x7c));
        let none = c.g[A0][0] == 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
        if !none {
            c.cycles += 3;
            c.g[A1][0] = c.lb(at(c, S0, 0x44));
            c.g[A2][0] = 0;
            call_ptr(c, callee::FEMODULE_SET_PREVIEW_VISIBLE, 0x0018_4b9c)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::UI_STATE_SHUTDOWN, 0x0018_4ba4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cFEStateEventSelect* cFEStateEventSelect_ctor(self)` (0x186610): UI state
/// constructor, then install the vtable/state and construct the embedded
/// event-select sub-object at +0x48.
pub fn fe_state_event_select_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::UI_STATE_CTOR, 0x0018_6624)?;
        let this = r32(c, S0);
        const_reg(c, V1, 0x47_0000, -0x42d8);
        c.g[V0][0] = 0x13;
        sw(c, this, 0x0c, c.g[V0][0]);
        sw(c, this, 0x08, c.g[V1][0]);
        c.g[A0][0] = sx(this.wrapping_add(0x48));
        call_ptr(c, 0x0020_06b8, 0x0018_6640)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        leave(c, 0x20);
        Ok(())
    });
}

/// `void cFEStateMemCard_Finish(cFEStateMemCard* self)` (0x187318): ask the
/// front end to start its module once (when the "started" flag is clear),
/// then mark the state started.
pub fn fe_state_mem_card_finish(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 6;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        sw(c, r32(c, S0), 0x1c0, c.g[V0][0]);
        c.g[V1][0] = c.lw(at(c, S0, 0x1ac));
        let already = c.g[V1][0] != 0;
        c.g[V0][0] = 1;
        if !already {
            c.cycles += 3;
            c.g[A0][0] = c.lw(gp(c, -0x848));
            call_ptr(c, callee::SSX_APP_START_FE_MODULE, 0x0018_7344)?;
            c.g[V0][0] = 1;
        }
        c.cycles += 5;
        frame.restore_ra(c, 0);
        sw(c, r32(c, S0), 0x1ac, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        leave(c, 0x20);
        Ok(())
    });
}

/// `cFEStateOptionsController* cFEStateOptionsController_ctor(self)`
/// (0x18c658): UI state constructor, then the options-controller vtable and
/// two cleared fields; clears the "options open" global.
pub fn fe_state_options_controller_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::UI_STATE_CTOR, 0x0018_c66c)?;
        let this = r32(c, S0);
        c.g[V0][0] = 0x1f;
        const_reg(c, V1, 0x47_0000, -0x3b80);
        sw(c, this, 0x0c, c.g[V0][0]);
        sw(c, this, 0x08, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sb(p(this, 0x50), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        sw(c, gp(c, -0x1c14), 0, 0);
        leave(c, 0x20);
        Ok(())
    });
}

/// `cFEStateMainMenu* cFEStateMainMenu_ctor(self)` (0x194a60): UI state
/// constructor, then the main-menu vtable/state and two cleared fields.
pub fn fe_state_main_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::UI_STATE_CTOR, 0x0019_4a74)?;
        let this = r32(c, S0);
        c.g[V0][0] = 2;
        const_reg(c, V1, 0x47_0000, -0x4618);
        sw(c, this, 0x0c, c.g[V0][0]);
        sw(c, this, 0x08, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        sw(c, this, 0x48, 0);
        sw(c, this, 0x4c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void FELocale_UnloadTable(FELocale* self, int index)` (0x195a50): drop
/// the locale string table entry at `index`, if it is loaded.
pub fn fe_locale_unload_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
        frame.save(c, S0, 0x10);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(8));
        frame.save_ra(c, 0);
        // `sll a1, 24; sra a1, 22`: the byte index times 4.
        c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 22) as u32);
        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[A0][0] = c.lw(r32(c, S0));
        if c.g[A0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 4;
            c.g[A1][0] = 3;
            call_ptr(c, callee::LOC_STRING_TABLE_DTOR, 0x0019_5a80)?;
            sw(c, r32(c, S0), 0, 0);
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cFEStateSelectPersona* cFEStateSelectPersona_ctor(self)` (0x1ad130): net
/// front-end state constructor (with a null engine argument), then the
/// persona-select vtable/state and one cleared field.
pub fn fe_state_select_persona_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::NET_FE_STATE_CTOR.call(c, 0x001a_d148)?;
        let this = r32(c, S0);
        c.g[V0][0] = 0x38;
        const_reg(c, V1, 0x47_0000, -0x70b0);
        sw(c, this, 0x0c, c.g[V0][0]);
        sw(c, this, 0x08, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        sw(c, this, 0x6d0, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cFEStateChallengePopup_v33(self, event, code)` (0x1af3e0): on the
/// two popup codes close the state manager's popup list, otherwise pass the
/// event to the net front-end state.
pub fn fe_state_challenge_popup_v33(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xf;
        let is_f = c.g[A2][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        let popup_code = if is_f {
            true
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x14;
            c.g[A2][0] == c.g[V0][0]
        };
        if popup_code {
            c.cycles += 5;
            c.g[A0][0] = c.lw(at(c, A0, 0x10));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, callee::UI_LIST_CLOSE_ALL_POPUPS, 0x001a_f408)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 3;
            callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, 0x001a_f418)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `void cFEStateChallengeReceivedPopup_v33(self, event, code)` (0x1b0628):
/// same dispatch as [`fe_state_challenge_popup_v33`] for the "challenge
/// received" popup.
pub fn fe_state_challenge_received_popup_v33(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xf;
        let is_f = c.g[A2][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        let popup_code = if is_f {
            true
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x14;
            c.g[A2][0] == c.g[V0][0]
        };
        if popup_code {
            c.cycles += 5;
            c.g[A0][0] = c.lw(at(c, A0, 0x10));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, callee::UI_LIST_CLOSE_ALL_POPUPS, 0x001b_0650)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 3;
            callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, 0x001b_0660)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `bool cFEStateOnlineCharacterSelect_HandleInput(self, input)`
/// (0x1b1590): unless the menu is disabled (or the pad global is clear),
/// let the input object handle the event and return its inverted result.
pub fn fe_state_online_character_select_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(at(c, A0, 0x6dc));
        if c.g[V0][0] != 0 {
            c.g[V1][0] = c.lw(at(c, A1, 0x08));
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw(gp(c, 0x2dc));
            if c.g[V0][0] != 0 {
                c.g[V0][0] = 0;
                frame.restore_ra(c, 0);
                c.cycles += 3;
                frame.close(c);
                return Ok(());
            }
            c.cycles += 1;
            c.g[V1][0] = c.lw(at(c, A1, 0x08));
        }
        c.cycles += 5;
        c.g[A0][0] = c.lh(p(r32(c, V1), 0x30));
        c.g[V0][0] = c.lw(p(r32(c, V1), 0x34));
        let target = r32(c, V0);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001b_15c4)?;
        c.g[V0][0] = c.g[V0][0] ^ 0x1;
        frame.restore_ra(c, 0);
        c.cycles += 3;
        frame.close(c);
        Ok(())
    });
}

/// `cFEStateCreateStep2* cFEStateCreateStep2_ctor(self)` (0x1ba3e0): net
/// front-end state constructor with a null engine, then the create-step-2
/// vtable/state and one cleared field.
pub fn fe_state_create_step2_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::NET_FE_STATE_CTOR.call(c, 0x001b_a3f8)?;
        let this = r32(c, S0);
        c.g[V0][0] = 0x4e;
        const_reg(c, V1, 0x47_0000, -0x7a50);
        sw(c, this, 0x0c, c.g[V0][0]);
        sw(c, this, 0x08, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        sw(c, this, 0x6a8, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cFEStateNetQuickPlayPopup_Update(self)` (0x1c1038): count the
/// popup's timeout down (wrapping to 30) while it is armed, then run the net
/// front-end state update.
pub fn fe_state_net_quick_play_popup_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(at(c, A0, 0x6ec));
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = c.lw(at(c, A0, 0x6e8));
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            let nonzero = c.g[V1][0] != 0;
            sw(c, r32(c, A0), 0x6e8, c.g[V0][0]);
            if !nonzero {
                c.cycles += 2;
                c.g[V0][0] = 0x1e;
                sw(c, r32(c, A0), 0x6e8, c.g[V0][0]);
            }
        }
        c.cycles += 5;
        callee::NET_FE_STATE_UPDATE.call(c, 0x001c_1070)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Message center and menu screens.

/// `bool MessageCenter_IsFlagSet(self, flag /*a1*/, key /*a2*/)`
/// (0x1e3a30): true when the BE message flag `key` is set for rider slot 1.
pub fn message_center_is_flag_set(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE, 0x001e_3a44)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x001e_3a50)?;
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, callee::BENEW_PLAYER_INT_IS_MESSAGE_FLAG_SET, 0x001e_3a60)?;
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.restore_ra(c, 0);
        leave(c, 0x20);
        Ok(())
    });
}

/// `void OVScreenRewardList_OnWidgetAction(self, widget /*a1*/, action
/// /*a2*/)` (0x1ff700): when the "confirm" key is pressed, arm the reward
/// list and tell rider slot 0.
pub fn ov_screen_reward_list_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        let no_widget = c.g[A1][0] == 0;
        frame.save_ra(c, 0);
        if !no_widget {
            c.cycles += 3;
            c.g[V0][0] = 5;
            let not_confirm = c.g[A2][0] != c.g[V0][0];
            c.g[RA][0] = c.ld(p(sp(c), 0));
            if not_confirm {
                c.cycles += 2;
                frame.close(c);
                return Ok(());
            }
            c.cycles += 8;
            c.g[V0][0] = 1;
            sw(c, gp(c, -0x9f4), 0, c.g[V0][0]);
            call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE, 0x001f_f724)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0xd;
            call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x001f_f730)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::BE_MESSAGE_FLAG_SET, 0x001f_f738)?;
        }
        // Both the no-widget path and the confirm path join here.
        c.cycles += 1;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `EventSelect* EventSelect_ctor(EventSelect* self)` (0x2006b8): when the
/// reward screen is in play, cache its selected index for the widget layer.
pub fn event_select_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        c.g[V0][0] = c.lw(gp(c, -0xb5c));
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = (c.g[V0][0] < 2u64) as u64;
        frame.save_ra(c, 0);
        if c.g[V0][0] == 0 {
            c.cycles += 9;
            call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE, 0x0020_06d4)?;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x0020_06e0)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::BE_GET_INTERFACE_RIDER, 0x0020_06e8)?;
            c.g[V1][0] = c.lw(at(c, V0, 0x54));
            sw(c, gp(c, -0xb4c), 0, c.g[V1][0]);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cOVStateOverlay_OnWidgetAction(self, widget, action)` (0x20a6a8):
/// pass the action to the overlay screen's widget handler, when one exists.
pub fn ov_state_overlay_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw(gp(c, -0x848));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(at(c, V0, 0x84));
        let no_race = c.g[A0][0] == 0;
        c.g[RA][0] = c.ld(p(sp(c), 0));
        if no_race {
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[A3][0] = c.lw(at(c, A0, 0x200));
        let no_screen = c.g[A3][0] == 0;
        if no_screen {
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 6;
        c.g[V0][0] = c.lw(at(c, A3, 0x0c));
        c.g[A0][0] = c.lh(p(r32(c, V0), 0x48));
        c.g[V1][0] = c.lw(p(r32(c, V0), 0x4c));
        let target = r32(c, V1);
        c.g[RA][0] = sx(0x0020_a6e0);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0020_a6e0)?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Section manager, replay playback and the memory-card manager.

/// `bool SectionMan_IsEventReady(SectionMan* self)` (0x22d390): true when the
/// event is loaded and its ready flag (+0x1c0) is set.
pub fn section_man_is_event_ready(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        callee::SECTION_MAN_IS_EVENT_LOADED.call(c, 0x0022_d3ac)?;
        let not_loaded = c.g[V0][0] == 0;
        c.g[V0][0] = c.g[S1][0];
        if !not_loaded {
            c.cycles += 3;
            c.g[V0][0] = c.lw(at(c, S0, 0x1c0));
            c.g[S1][0] = (0u64 < c.g[V0][0]) as u64;
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void GameStateReplayPlayback_Enter(void)` (0x236fe8): start a replay
/// playback: load the script, unload it, quiet the race manager's movie
/// player and step the replay manager.
pub fn game_state_replay_playback_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.lw(gp(c, 0xce8));
        frame.save_ra(c, 0);
        call_ptr(c, callee::SCRIPT_MAN_LOAD, 0x0023_6ffc)?;
        c.g[A0][0] = c.lw(gp(c, 0xce8));
        call_ptr(c, callee::SCRIPT_MAN_UNLOAD, 0x0023_7004)?;
        c.g[V0][0] = c.lw(gp(c, -0x848));
        c.g[A1][0] = 0;
        c.g[V1][0] = c.lw(at(c, V0, 0x84));
        c.g[A0][0] = c.lw(at(c, V1, 0x94));
        call_ptr(c, callee::WORLD_VIEW_SET, 0x0023_7018)?;
        c.g[A0][0] = 0x11;
        call_ptr(c, callee::MENU_SYS_EVENT, 0x0023_7020)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void MCManager_State02(MCManager* self)` (0x23e4f8): keep the memory-card
/// frame counter above zero; when it runs out, have the movie player show
/// its "insert card" screen.
pub fn mc_manager_state02(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = r32(c, A2);
        c.g[V0][0] = c.lw(p(this, 0x108));
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        let left = (c.g[V1][0] as i64) > 0;
        sw(c, this, 0x108, c.g[V0][0]);
        if !left {
            c.cycles += 6;
            c.g[V0][0] = c.lw(p(this, 0x748));
            c.g[A1][0] = 3;
            c.g[A0][0] = c.lh(p(r32(c, V0), 0x08));
            c.g[V1][0] = c.lw(p(r32(c, V0), 0x0c));
            let target = r32(c, V1);
            c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_e530)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void MCManager_WriteSaveChunk(MCManager* self, int size /*a2*/)`
/// (0x241138): advance the written-byte counter, mark the save active and
/// have the movie player write the chunk.
pub fn mc_manager_write_save_chunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = 1;
        c.g[V0][0] = c.lw(at(c, A0, 0xe4));
        c.g[T0][0] = c.lw(at(c, A0, 0x434));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        sw(c, r32(c, A0), 0xe4, c.g[V0][0]);
        sw(c, gp(c, -0x478), 0, c.g[A3][0]);
        c.g[V0][0] = c.lw(r32(c, T0));
        c.g[A0][0] = c.lh(p(r32(c, V0), 0x148));
        c.g[V1][0] = c.lw(p(r32(c, V0), 0x14c));
        let target = r32(c, V1);
        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0024_1170)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void VidEnginePlayer_dtor(VidEnginePlayer* self, int flags)` (0x253418):
/// close the player, then `operator delete` it when `flags & 1`.
pub fn vid_engine_player_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        callee::VID_ENGINE_PLAYER_CLOSE.call(c, 0x0025_3434)?;
        c.g[S0][0] = c.g[S0][0] & 1;
        let keep = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, callee::BUILTIN_DELETE, 0x0025_3448)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Lobby manager.

/// `void* LobbyMan_LookupPeerAddr(LobbyMan* self, int key /*a1*/, void* def
/// /*a2*/)` (0x25a730): the address the lobby API returns for `key`, or
/// `def` when nothing is registered.
pub fn lobby_man_lookup_peer_addr(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(at(c, A0, 0xf4));
        let no_table = c.g[A0][0] == 0;
        c.g[S0][0] = c.g[A2][0];
        if !no_table {
            c.cycles += 5;
            c.g[A1][0] = 0x48_0000;
            c.g[A2][0] = c.g[V0][0];
            const_reg(c, A1, 0x48_0000, 0x13a0);
            call_ptr(c, callee::VID_ENGINE_PLAYER_REQUEST, 0x0025_a75c)?;
            if c.g[V0][0] != 0 {
                c.g[S0][0] = c.g[V0][0];
            }
        }
        c.cycles += 5;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        leave(c, 0x20);
        Ok(())
    });
}

/// `void LobbyMan_DestroyLobbyApi(LobbyMan* self)` (0x25aa98): destroy the
/// peer lists, then release the lobby API objects.
pub fn lobby_man_destroy_lobby_api(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::LOBBY_MAN_DESTROY_LISTS.call(c, 0x0025_aaac)?;
        c.g[A0][0] = c.lw(at(c, S0, 0x54));
        if c.g[A0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 6;
            call_ptr(c, callee::LOBBY_MAN_REQUEST_UPDATE, 0x0025_aac0)?;
            c.g[A0][0] = c.lw(at(c, S0, 0x54));
            call_ptr(c, callee::LOBBY_MAN_REQUEST_FREE, 0x0025_aac8)?;
            sw(c, r32(c, S0), 0x54, 0);
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void LobbyMan_SendRequest(LobbyMan* self, int tag /*a1*/, int cb /*a2*/,
/// int arg /*a3*/, int a4 /*t0*/, float f /*f12*/)` (0x25b608): fill the
/// request record and hand it to the lobby API.
pub fn lobby_man_send_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.sw(this, c.g[A1][0]);
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = c.lw(p(this, 0x54));
        c.g[A2][0] = c.g[A3][0];
        c.swc1(12, p(this, 0xe0));
        c.g[A3][0] = c.g[T0][0];
        c.g[T0][0] = c.g[S0][0];
        call_ptr(c, callee::LOBBY_API_ADD_REQUEST, 0x0025_b638)?;
        sw(c, r32(c, S0), 0x40, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        leave(c, 0x20);
        Ok(())
    });
}

/// `void LobbyMan_Disconnect(LobbyMan* self)` (0x25b800): drop the peer
/// table, then the lobby API, and clear the connection fields.
pub fn lobby_man_disconnect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::LOBBY_MAN_DESTROY_PEER_TABLE.call(c, 0x0025_b814)?;
        c.g[V0][0] = c.lw(at(c, S0, 0x54));
        if c.g[V0][0] == 0 {
            sw(c, r32(c, S0), 0, 0);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::LOBBY_MAN_DESTROY_LOBBY_API.call(c, 0x0025_b828)?;
            sw(c, r32(c, S0), 0, 0);
        }
        c.cycles += 6;
        sw(c, r32(c, S0), 0x08, 0);
        sw(c, r32(c, S0), 0x0c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Login and voice managers.

/// `void cSharedLoginMan_dtor(cSharedLoginMan* self, int flags)` (0x265908):
/// shut the login manager down, then `operator delete` it when `flags & 1`.
pub fn shared_login_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        callee::SHARED_LOGIN_MAN_SHUTDOWN.call(c, 0x0026_5924)?;
        c.g[S0][0] = c.g[S0][0] & 1;
        let keep = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, callee::BUILTIN_DELETE, 0x0026_5938)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cSharedLoginMan_DoCreate(cSharedLoginMan* self)` (0x265e80):
/// create the login service (the dirty-socket tag) and mark the manager
/// created.
pub fn shared_login_man_do_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        const_reg(c, A1, 0x53_0000, 0x6760);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        sw(c, r32(c, S0), 0x04, 0);
        const_reg(c, A0, 0x44_0000, 0x2a60);
        sw(c, gp(c, 0x2f0), 0, 0);
        call_ptr(c, callee::DIRTY_SOCK_CREATE, 0x0026_5eac)?;
        c.g[V1][0] = 0xb;
        frame.restore_ra(c, 0);
        sw(c, r32(c, S0), 0, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        leave(c, 0x20);
        Ok(())
    });
}

/// `void cVoipMan_dtor(cVoipMan* self, int flags)` (0x266d10): stop the voice
/// manager, then `operator delete` it when `flags & 1`.
pub fn voip_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        callee::VOIP_MAN_STOP.call(c, 0x0026_6d2c)?;
        c.g[S0][0] = c.g[S0][0] & 1;
        let keep = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, callee::BUILTIN_DELETE, 0x0026_6d40)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cVoipMan_Deactivate(cVoipMan* self)` (0x266e88): tear the voice
/// session down when both the manager and its session are live.
pub fn voip_man_deactivate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw(r32(c, S0));
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw(at(c, S0, 0x04));
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 4;
                call_ptr(c, callee::DIRTY_SOCK_DESTROY, 0x0026_6eb8)?;
                sw(c, r32(c, S0), 0x04, 0);
                frame.restore(c, S0, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `bool PathData_HasMarker(PathData* self, int a /*a1*/, int b /*a2*/)`
/// (0x26b598): true when one of the `count` markers (0x28 bytes each, a
/// 16-byte head at +4) matches `(b, a)`.
pub fn path_data_has_marker(c: &mut Cpu) {
    c.cycles += 3;
    let count = c.lw(r32(c, A0));
    c.g[A3][0] = count;
    let none = (count as i64) <= 0;
    c.g[V1][0] = 0;
    if !none {
        c.cycles += 1;
        c.g[A0][0] = c.lw(at(c, A0, 0x04));
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(at(c, A0, 0x04));
            let mismatch = c.g[V0][0] != c.g[A1][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            if mismatch {
                c.cycles += 1;
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A3][0] as i64)) as u64;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw(r32(c, A0));
                let mismatch = c.g[V0][0] != c.g[A2][0];
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A3][0] as i64)) as u64;
                if !mismatch {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    abi::ret(c);
                    return;
                }
            }
            c.cycles += 2;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x28));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0026_b5a8);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Replay cache and stream.

/// `void ReplayCache_dtor(ReplayCache* self, int flags)` (0x26d0e8): free the
/// cache, then `operator delete` it when `flags & 1`.
pub fn replay_cache_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        callee::REPLAY_CACHE_FREE.call(c, 0x0026_d104)?;
        c.g[S0][0] = c.g[S0][0] & 1;
        let keep = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, callee::BUILTIN_DELETE, 0x0026_d118)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void Replay_RestoreSnapshot(Replay* self, ReplayFrame* frame)`
/// (0x26db88): mark the frame used, then restore the world state from it.
pub fn replay_restore_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        callee::REPLAY_SNAPSHOT_EMPTY_HOOK_B.call(c, 0x0026_dbac)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        callee::WORLD_STATE_RESTORE.call(c, 0x0026_dbb8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void ReplayStream_FreeBuffer(ReplayStream* self)` (0x26e968): free the
/// stream's frame buffer and clear its block fields.
pub fn replay_stream_free_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.lw(at(c, S0, 0x04));
        if c.g[A0][0] == 0 {
            sw(c, r32(c, S0), 0x10, 0);
        } else {
            c.cycles += 3;
            call_ptr(c, callee::EA_MEM_FREE, 0x0026_e98c)?;
            sw(c, r32(c, S0), 0x10, 0);
        }
        c.cycles += 7;
        sw(c, r32(c, S0), 0x04, 0);
        sw(c, r32(c, S0), 0x08, 0);
        sw(c, r32(c, S0), 0x0c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void ReplayStream_Read(ReplayStream* self, void* dst /*a1*/, int n
/// /*a2*/)` (0x26ea20): copy `n` bytes to `dst` and advance the stream's
/// write cursor.
pub fn replay_stream_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[A1][0] = c.lw(at(c, S0, 0x0c));
        call_ptr(c, callee::REPLAY_STREAM_WRITE, 0x0026_ea44)?;
        c.g[V0][0] = c.lw(at(c, S0, 0x0c));
        c.g[RA][0] = c.ld(p(sp(c), 0));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
        sw(c, r32(c, S0), 0x0c, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        leave(c, 0x30);
        Ok(())
    });
}

/// `void Replay_RestoreResumeSnapshot(Replay* self)` (0x26f850): restore the
/// resume snapshot, then free it.
pub fn replay_restore_resume_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = c.lw(at(c, S0, 0x3d0));
        if c.g[A1][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 7;
            callee::REPLAY_RESTORE_SNAPSHOT.call(c, 0x0026_f874)?;
            c.g[A1][0] = c.lw(at(c, S0, 0x3d0));
            c.g[A0][0] = c.g[S0][0];
            callee::REPLAY_FREE_SNAPSHOT.call(c, 0x0026_f880)?;
            sw(c, r32(c, S0), 0x3d0, 0);
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void Replay_EndInstantReplay(Replay* self)` (0x270670): when an instant
/// replay is running, end the playback and let the rider resume.
pub fn replay_end_instant_replay(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(at(c, A0, 0x620));
        let idle = c.g[V0][0] == 0;
        c.g[RA][0] = c.ld(p(sp(c), 0));
        if !idle {
            c.cycles += 10;
            callee::REPLAY_END_PLAYBACK.call(c, 0x0027_068c)?;
            c.g[V0][0] = c.lw(gp(c, -0x848));
            c.g[V1][0] = c.lw(at(c, V0, 0x84));
            c.g[A0][0] = c.lw(at(c, V1, 0x0c));
            c.g[V0][0] = c.lw(at(c, A0, 0x40));
            c.g[V1][0] = c.lw(at(c, V0, 0x18));
            c.g[A0][0] = c.lw(at(c, V1, 0x790));
            call_ptr(c, callee::APP_SOMETHING, 0x0027_06a8)?;
            c.g[RA][0] = c.ld(p(sp(c), 0));
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `void Replay_FreeSaveBuffers(Replay* self)` (0x270f78): free both save
/// buffers and clear the fields that point at them.
pub fn replay_free_save_buffers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.lw(at(c, S0, 0x63c));
        call_ptr(c, callee::BUILTIN_DELETE, 0x0027_0f90)?;
        c.g[A0][0] = c.lw(at(c, S0, 0x640));
        if c.g[A0][0] == 0 {
            sw(c, r32(c, S0), 0x640, 0);
        } else {
            c.cycles += 3;
            call_ptr(c, callee::EA_MEM_FREE, 0x0027_0fa4)?;
            sw(c, r32(c, S0), 0x640, 0);
        }
        c.cycles += 5;
        sw(c, r32(c, S0), 0x63c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// In-sequence channel data.

/// `void InSeq_Channel_dtor(InSeq_Channel* self, int flags)` (0x273970):
/// release the channel's block, then delete it from the heap when `flags &
/// 1`.
pub fn in_seq_channel_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        callee::IN_SEQ_CHANNEL_RELEASE.call(c, 0x0027_398c)?;
        c.g[S0][0] = c.g[S0][0] & 1;
        let keep = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            callee::IN_SEQ_HEAP_DELETE.call(c, 0x0027_39a0)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void InSeq_ChannelSelectBlock(InSeq_Channel* self, int index)`
/// (0x273a60): select the sequence block at `index` and point at its first
/// event record.
pub fn in_seq_channel_select_block(c: &mut Cpu) {
    c.cycles += 13;
    c.g[A2][0] = c.g[A0][0];
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[V1][0] = c.lw(at(c, A2, 0x14));
    sw(c, r32(c, A2), 0x18, c.g[A1][0]);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A0][0] = c.lw(at(c, A2, 0x04));
    c.g[V1][0] = c.lw(r32(c, V0));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    sw(c, r32(c, A2), 0x1c, c.g[A0][0]);
    c.g[V0][0] = c.lhu(r32(c, A0));
    c.g[V0][0] = (c.g[V0][0] < 9u64) as u64;
    let small = c.g[V0][0] != 0;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x08));
    if small {
        c.cycles += 2;
        sw(c, r32(c, A2), 0x20, 0);
    } else {
        c.cycles += 2;
        sw(c, r32(c, A2), 0x20, c.g[V0][0]);
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Script animation banks.

/// `void ScriptAnimBankMan_RemoveBank(ScriptAnimBankMan* self, int id)`
/// (0x275650): when the bank exists, remove it through the manager's list.
pub fn script_anim_bank_man_remove_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_ANIM_BANK_MAN_FIND_BANK.call(c, 0x0027_5664)?;
        c.g[A1][0] = c.g[V0][0];
        if c.g[A1][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 6;
            c.g[V0][0] = c.lw(at(c, S0, 0x0c));
            c.g[A0][0] = c.lh(p(r32(c, V0), 0x10));
            c.g[V1][0] = c.lw(p(r32(c, V0), 0x14));
            let target = r32(c, V1);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0027_5684)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `ScriptAnimBank* ScriptAnimBankMan_FindBank(ScriptAnimBankMan* self, int
/// id /*a1*/)` (0x275718): walk the bank list (0x14 bytes each, head at +8),
/// returning the first live entry with `id`.
pub fn script_anim_bank_man_find_bank(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A2][0] = c.lw(at(c, A0, 0x04));
    let none = (c.g[A2][0] as i64) <= 0;
    c.g[V1][0] = 0;
    if !none {
        c.cycles += 1;
        c.g[A0][0] = c.lw(at(c, A0, 0x08));
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(at(c, A0, 0x08));
            let dead = c.g[V0][0] == 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            if dead {
                c.cycles += 1;
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw(r32(c, A0));
                let mismatch = c.g[V0][0] != c.g[A1][0];
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                if !mismatch {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[A0][0];
                    abi::ret(c);
                    return;
                }
            }
            c.cycles += 2;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_5728);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Script engine accessors.

/// `bool ScriptEngine_IsLoadDone(ScriptEngine* self)` (0x278d58): true when
/// no async load is pending and bank slot 1 is not loaded.
pub fn script_engine_is_load_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS.call(c, 0x0027_8d6c)?;
        c.g[V0][0] = c.lw(at(c, S0, 0x538));
        let pending = c.g[V0][0] != 0;
        c.g[V1][0] = 0;
        if !pending {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            callee::SCRIPT_ENGINE_IS_BANK_SLOT_LOADED.call(c, 0x0027_8d84)?;
            c.g[V1][0] = (c.g[V0][0] < 1u64) as u64;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = c.g[V1][0];
        frame.restore_ra(c, 0);
        leave(c, 0x20);
        Ok(())
    });
}

/// `bool ScriptEngine_IsActiveListSkippable(ScriptEngine* self)` (0x278da0):
/// true unless the active script list says otherwise.
pub fn script_engine_is_active_list_skippable(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = 2;
        c.g[V1][0] = c.lw(at(c, A1, 0x550));
        let idle = c.g[V1][0] == c.g[V0][0];
        c.g[A0][0] = 0xcc;
        if idle {
            c.cycles += 2;
            c.g[V0][0] = 1;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 6;
            c.g[V0][0] = c.lw(at(c, A1, 0x54c));
            c.mult(0, A0, V1, A0);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            callee::SCRIPT_LIST_IS_CURRENT_SKIPPABLE.call(c, 0x0027_8dcc)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `int ScriptEngine_ListGetState(ScriptEngine* self, int index)` (0x2791d8):
/// the state of the active script list at `index`.
pub fn script_engine_list_get_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS.call(c, 0x0027_91f4)?;
        c.g[V0][0] = 0xcc;
        c.g[A0][0] = c.lw(at(c, S1, 0x54c));
        c.mult(0, S0, S0, V0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        callee::SCRIPT_LIST_GET_STATE.call(c, 0x0027_9208)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int ScriptEngine_GetActiveListFieldB0(ScriptEngine* self)` (0x279298):
/// field 0xb0 of the current active script list (0 when idle).
pub fn script_engine_get_active_list_field_b0(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = 2;
        c.g[V1][0] = c.lw(at(c, A1, 0x550));
        let idle = c.g[V1][0] == c.g[V0][0];
        c.g[A0][0] = 0xcc;
        if idle {
            c.cycles += 2;
            c.g[V0][0] = 0;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 6;
            c.g[V0][0] = c.lw(at(c, A1, 0x54c));
            c.mult(0, A0, V1, A0);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            callee::SCRIPT_LIST_GET_FIELD_B0.call(c, 0x0027_92c4)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `bool ScriptEngine_IsPlayingScript(ScriptEngine* self, int a)` (0x279370):
/// whether the current active script list is playing a script.
pub fn script_engine_is_playing_script(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = 2;
        c.g[V1][0] = c.lw(at(c, A2, 0x550));
        let idle = c.g[V1][0] == c.g[V0][0];
        c.g[A0][0] = 0xcc;
        if idle {
            c.cycles += 2;
            c.g[V0][0] = 0;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 6;
            c.g[V0][0] = c.lw(at(c, A2, 0x54c));
            c.mult(0, A0, V1, A0);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            callee::SCRIPT_LIST_IS_PLAYING_SCRIPT.call(c, 0x0027_939c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `void ScriptEngine_CancelScriptBank(ScriptEngine* self, int id)`
/// (0x279840): find the bank slot for `id` and cancel its load.
pub fn script_engine_cancel_script_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 2;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        callee::SCRIPT_ENGINE_FIND_BANK_SLOT_BY_ID.call(c, 0x0027_9860)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        callee::SCRIPT_ENGINE_CANCEL_BANK_LOAD.call(c, 0x0027_9870)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule, Events};
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, DATA, GAME_GP};

    /// `BELibrary_GetInstance`'s fast path: a non-null singleton, so the game
    /// never lazily allocates it and reads the rewards file off the disc.
    fn be_library(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x3_0000);
    }

    /// A dummy world (`gp-0x848`) and race manager: rider manager at +0x7c,
    /// movie player at +0x94, overlay screen slot at +0x200.
    fn world(c: &mut Cpu) {
        let w = DATA + 0x4000;
        c.bus.write32(GAME_GP.wrapping_sub(0x848), w);
        c.bus.write32(w + 0x7c, DATA + 0x5000);
        c.bus.write32(w + 0x84, DATA + 0x6000);
        c.bus.write32(DATA + 0x6000 + 0x94, DATA + 0x7000);
    }

    /// The game's pool allocator: `gp+0xdd8` ready plus four free nodes in
    /// each of the seven size classes (8 to 512 bytes).
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
        let nodes = DATA + 0x2_8000;
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let pool = DATA + 0x2_0000 + 0x100 * i;
            let base = nodes + 0x100 * i;
            for n in 0..4u32 {
                let node = base + 0x40 * n;
                c.bus.write32(node, if n == 3 { 0 } else { node + 0x40 });
            }
            c.bus.write32(pool + head, base);
            c.bus.write32(GAME_GP + slot, pool);
        }
    }

    /// `DLHeap_InitArena(&MEM_HEAP, arena, size)`: the game heap over a
    /// scratch arena. The replay restore/free path allocates and frees
    /// through it, and a zeroed heap sends `DLHeap_Consolidate` into a walk
    /// over garbage.
    fn heap_init(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        const ARENA: u32 = DATA + 0x4_0000;
        c.g[GP][0] = GAME_GP as u64;
        c.g[A0][0] = MEM_HEAP as u64;
        c.g[A1][0] = ARENA as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = 0x0031_ff60;
        ssx3_ee::s10::DLHeap_InitArena(c);
        assert_eq!(c.pc, crate::difftest::RA, "DLHeap_InitArena did not return");
    }

    /// A dummy movie-player object whose method slot `off` calls the fake
    /// callee.
    fn fake_method(c: &mut Cpu, base: u32, off: u32) {
        c.bus.write16(base + off, 0);
        c.bus.write32(base + off + 4, FAKE);
    }

    #[test]
    fn be_mission_int_session_type_at_least_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 0x1546, 0);
            be_library(c);
            for i in 0..22u32 {
                let e = 0x0044_0770 + 0x2c * i;
                c.bus.write32(e, i);
                c.bus.write32(e + 0x18, 10 * i);
            }
            // (session type, value): no match, matching with a high enough
            // value, matching with too small a value, the last record, and a
            // signed value.
            let (id, value): (u64, u64) = match seed % 5 {
                0 => (999, 0),
                1 => (5, 30),
                2 => (5, 100),
                3 => (21, 210),
                _ => (0, u64::MAX),
            };
            c.g[A1][0] = value;
            c.g[A2][0] = id;
            let _ = r.next_u32();
            schedule(c, &mut r, seed, 0x0015_4630)
        };
        check_events(
            ssx3_ee::s01::f_00154630__generated,
            be_mission_int_session_type_at_least,
            80,
            &setup,
        );
    }

    #[test]
    fn rewards_file_dtor_matches() {
        check(
            ssx3_ee::s01::f_0015a7d0__generated,
            rewards_file_dtor,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15a7d0, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                // A non-negative tag runs the unload path; the nested pointer
                // (+4) stays null, so only the outer object is deleted.
                c.bus.write32(this_, r.next_u32() & 0x7fff_ffff);
                c.bus.write32(this_ + 4, 0);
                c.bus.write32(this_ + 0x4c, r.next_u32());
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = (seed & 1) as u64;
            },
        );
    }

    #[test]
    fn bounded_camera_algorithm_ctor_matches() {
        check(
            ssx3_ee::s02::f_00174190__generated,
            bounded_camera_algorithm_ctor,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x174190, 0);
                be_library(c);
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }

    #[test]
    fn fe_state_uber_trick_shutdown_matches() {
        check(
            ssx3_ee::s02::f_00184b70__generated,
            fe_state_uber_trick_shutdown,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x184b70, 0);
                be_library(c);
                if seed % 3 != 0 {
                    world(c);
                }
                let this_ = DATA + 0x100;
                // The rider index selects the preview object (0xce0 bytes each).
                c.bus.write8(this_ + 0x44, (seed % 4) as u8);
                c.g[A0][0] = sx(this_);
            },
        );
    }

    #[test]
    fn fe_state_event_select_ctor_matches() {
        check(
            ssx3_ee::s02::f_00186610__generated,
            fe_state_event_select_ctor,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x186610, 0);
                be_library(c);
                // Below 2 the event-select constructor skips the BE interface.
                c.bus.write32(
                    GAME_GP.wrapping_sub(0xb5c),
                    if seed % 2 == 0 { 0 } else { 5 },
                );
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }

    #[test]
    fn fe_state_mem_card_finish_matches() {
        check(
            ssx3_ee::s02::f_00187318__generated,
            fe_state_mem_card_finish,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x187318, 0);
                be_library(c);
                pool_allocator(c);
                let this_ = DATA + 0x100;
                c.bus
                    .write32(this_ + 0x1ac, if seed % 2 == 0 { 0 } else { 1 });
                c.g[A0][0] = sx(this_);
            },
        );
    }

    #[test]
    fn fe_state_options_controller_ctor_matches() {
        check(
            ssx3_ee::s02::f_0018c658__generated,
            fe_state_options_controller_ctor,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x18c658, 0);
                be_library(c);
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }

    #[test]
    fn fe_state_main_menu_ctor_matches() {
        check(
            ssx3_ee::s02::f_00194a60__generated,
            fe_state_main_menu_ctor,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x194a60, 0);
                be_library(c);
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }

    #[test]
    fn fe_locale_unload_table_matches() {
        check(
            ssx3_ee::s02::f_00195a50__generated,
            fe_locale_unload_table,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x195a50, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                let entry = DATA + 0x2000;
                let idx: u64 = [0, 1, 2, 7, 0x3c][seed as usize % 5];
                let slot = this_ + 8 + 4 * idx as u32;
                c.bus.write32(slot, if seed % 3 == 0 { 0 } else { entry });
                // A null nested pointer: the table's destructor only frees itself.
                c.bus.write32(entry + 4, 0);
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = idx;
            },
        );
    }

    /// `NetFEState_ctor` allocates from the pool: the constructors below need
    /// the dummy BE library and the allocator.
    fn net_fe_state_fixture(c: &mut Cpu, seed: u32, salt: u64) {
        let _ = machine(c, seed, salt, 0);
        be_library(c);
        pool_allocator(c);
        c.g[A0][0] = sx(DATA + 0x100);
        c.g[A1][0] = 0;
    }

    #[test]
    fn fe_state_select_persona_ctor_matches() {
        check(
            ssx3_ee::s03::f_001ad130__generated,
            fe_state_select_persona_ctor,
            40,
            &|c, seed| {
                net_fe_state_fixture(c, seed, 0x1ad130);
            },
        );
    }

    #[test]
    fn fe_state_challenge_popup_v33_matches() {
        check(
            ssx3_ee::s03::f_001af3e0__generated,
            fe_state_challenge_popup_v33,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x1af3e0, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                // 0xf/0x14 close the popup list; other codes reach the
                // net-state event dispatcher (which returns at once for them).
                let code: u64 = [0x0f, 0x14, 0, 1][seed as usize % 4];
                c.bus.write32(this_ + 0x10, DATA + 0x1000);
                c.bus.write32(this_ + 0x69c, 0);
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = sx(DATA + 0x1400);
                c.g[A2][0] = code;
            },
        );
    }

    #[test]
    fn fe_state_challenge_received_popup_v33_matches() {
        check(
            ssx3_ee::s03::f_001b0628__generated,
            fe_state_challenge_received_popup_v33,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x1b0628, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                let code: u64 = [0x0f, 0x14, 0, 1][seed as usize % 4];
                c.bus.write32(this_ + 0x10, DATA + 0x1000);
                c.bus.write32(this_ + 0x69c, 0);
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = sx(DATA + 0x1400);
                c.g[A2][0] = code;
            },
        );
    }

    #[test]
    fn fe_state_online_character_select_handle_input_matches() {
        check(
            ssx3_ee::s03::f_001b1590__generated,
            fe_state_online_character_select_handle_input,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x1b1590, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                let input = DATA + 0x1000;
                let vt = DATA + 0x2000;
                // Menu enabled, or disabled with the pad global set: the early
                // return. Otherwise the input's handler is called.
                let enabled = seed % 4 != 0;
                c.bus.write32(this_ + 0x6dc, if enabled { 1 } else { 0 });
                c.bus
                    .write32(GAME_GP.wrapping_add(0x2dc), if enabled { 0 } else { 1 });
                c.bus.write32(input + 8, vt);
                fake_method(c, vt, 0x30);
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = sx(input);
            },
        );
    }

    #[test]
    fn fe_state_create_step2_ctor_matches() {
        check(
            ssx3_ee::s03::f_001ba3e0__generated,
            fe_state_create_step2_ctor,
            40,
            &|c, seed| {
                net_fe_state_fixture(c, seed, 0x1ba3e0);
            },
        );
    }

    #[test]
    fn fe_state_net_quick_play_popup_update_matches() {
        check(
            ssx3_ee::s03::f_001c1038__generated,
            fe_state_net_quick_play_popup_update,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x1c1038, 0);
                be_library(c);
                pool_allocator(c);
                let this_ = DATA + 0x100;
                c.bus
                    .write32(this_ + 0x6ec, if seed % 3 == 0 { 0 } else { 1 });
                let left: [u32; 4] = [0, 1, 5, 30];
                c.bus.write32(this_ + 0x6e8, left[seed as usize % 4]);
                c.g[A0][0] = sx(this_);
            },
        );
    }

    #[test]
    fn message_center_is_flag_set_matches() {
        check(
            ssx3_ee::s04::f_001e3a30__generated,
            message_center_is_flag_set,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x1e3a30, 0);
                be_library(c);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = sx(DATA + 0x1400);
            },
        );
    }

    #[test]
    fn ov_screen_reward_list_on_widget_action_matches() {
        check(
            ssx3_ee::s05::f_001ff700__generated,
            ov_screen_reward_list_on_widget_action,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x1ff700, 0);
                be_library(c);
                c.g[A0][0] = sx(DATA + 0x100);
                // No widget, or a widget with an action other than "confirm":
                // both return without touching the BE library.
                c.g[A1][0] = if seed % 2 == 0 { 0 } else { sx(DATA + 0x1400) };
                c.g[A2][0] = if seed % 4 == 1 { 5 } else { 0 };
            },
        );
    }

    #[test]
    fn event_select_ctor_matches() {
        check(
            ssx3_ee::s05::f_002006b8__generated,
            event_select_ctor,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x2006b8, 0);
                be_library(c);
                c.bus.write32(
                    GAME_GP.wrapping_sub(0xb5c),
                    if seed % 2 == 0 { 0 } else { 2 },
                );
            },
        );
    }

    #[test]
    fn ov_state_overlay_on_widget_action_matches() {
        check(
            ssx3_ee::s05::f_0020a6a8__generated,
            ov_state_overlay_on_widget_action,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x20a6a8, 0);
                be_library(c);
                let screen = DATA + 0x8000;
                let vt = DATA + 0x9000;
                match seed % 3 {
                    0 => {}
                    1 => {
                        world(c);
                    }
                    _ => {
                        world(c);
                        c.bus.write32(DATA + 0x6000 + 0x200, screen);
                        c.bus.write32(screen + 0x0c, vt);
                        fake_method(c, vt, 0x48);
                    }
                }
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(DATA + 0x1400);
                c.g[A2][0] = 3;
            },
        );
    }

    #[test]
    fn section_man_is_event_ready_matches() {
        check(
            ssx3_ee::s05::f_0022d390__generated,
            section_man_is_event_ready,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x22d390, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                c.bus
                    .write32(this_ + 0x1c0, if seed % 2 == 0 { 0 } else { 1 });
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = 0;
            },
        );
    }

    #[test]
    fn game_state_replay_playback_enter_matches() {
        check(
            ssx3_ee::s06::f_00236fe8__generated,
            game_state_replay_playback_enter,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x236fe8, 0);
                be_library(c);
                pool_allocator(c);
                world(c);
                // No script manager installed: both script calls take their empty
                // path.
                c.bus.write32(GAME_GP.wrapping_add(0xce8), 0);
            },
        );
    }

    #[test]
    fn mc_manager_state02_matches() {
        check(
            ssx3_ee::s06::f_0023e4f8__generated,
            mc_manager_state02,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x23e4f8, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                let vt = DATA + 0x2000;
                let left: [u32; 4] = [0, 1, 5, 0x8000_0000];
                c.bus.write32(this_ + 0x108, left[seed as usize % 4]);
                c.bus.write32(this_ + 0x748, vt);
                fake_method(c, vt, 0x08);
                c.g[A0][0] = sx(this_);
            },
        );
    }

    #[test]
    fn mc_manager_write_save_chunk_matches() {
        check(
            ssx3_ee::s06::f_00241138__generated,
            mc_manager_write_save_chunk,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x241138, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                let movie = DATA + 0x2000;
                let vt = DATA + 0x3000;
                c.bus.write32(this_ + 0xe4, (seed as u32).wrapping_mul(7));
                c.bus.write32(this_ + 0x434, movie);
                c.bus.write32(movie, vt);
                fake_method(c, vt, 0x148);
                c.g[A0][0] = sx(this_);
                c.g[A2][0] = (seed % 128) as u64;
            },
        );
    }

    #[test]
    fn vid_engine_player_dtor_matches() {
        check(
            ssx3_ee::s06::f_00253418__generated,
            vid_engine_player_dtor,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x253418, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = (seed & 1) as u64;
            },
        );
    }

    #[test]
    fn lobby_man_lookup_peer_addr_matches() {
        check(
            ssx3_ee::s06::f_0025a730__generated,
            lobby_man_lookup_peer_addr,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x25a730, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                let tags = DATA + 0x2000;
                if seed % 2 == 0 {
                    c.bus.write32(this_ + 0xf4, 0);
                } else {
                    c.bus.write32(this_ + 0xf4, tags);
                }
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = (seed % 32) as u64;
                c.g[A2][0] = sx(DATA + 0x1800);
            },
        );
    }

    #[test]
    fn lobby_man_destroy_lobby_api_matches() {
        check(
            ssx3_ee::s06::f_0025aa98__generated,
            lobby_man_destroy_lobby_api,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x25aa98, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                // A null lobby API takes the short path; a dummy (zeroed) one runs
                // the request teardown, which walks its empty buckets.
                c.bus
                    .write32(this_ + 0x54, if seed % 2 == 0 { 0 } else { DATA + 0x2000 });
                c.g[A0][0] = sx(this_);
            },
        );
    }

    #[test]
    fn lobby_man_send_request_matches() {
        check(
            ssx3_ee::s06::f_0025b608__generated,
            lobby_man_send_request,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x25b608, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                // A null lobby API: the request add returns -1 at once.
                c.bus.write32(this_ + 0x54, 0);
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = sx(DATA + 0x1800);
                c.g[A3][0] = sx(DATA + 0x1c00);
                c.g[T0][0] = sx(DATA + 0x1c40);
                c.f[12] = r.f32(1000.0);
            },
        );
    }

    #[test]
    fn lobby_man_disconnect_matches() {
        check(
            ssx3_ee::s06::f_0025b800__generated,
            lobby_man_disconnect,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x25b800, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                c.bus
                    .write32(this_ + 0x54, if seed % 2 == 0 { 0 } else { DATA + 0x2000 });
                c.g[A0][0] = sx(this_);
            },
        );
    }

    #[test]
    fn shared_login_man_dtor_matches() {
        check(
            ssx3_ee::s07::f_00265908__generated,
            shared_login_man_dtor,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x265908, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = (seed & 1) as u64;
            },
        );
    }

    #[test]
    fn shared_login_man_do_create_matches() {
        check(
            ssx3_ee::s07::f_00265e80__generated,
            shared_login_man_do_create,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x265e80, 0);
                be_library(c);
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }

    #[test]
    fn voip_man_dtor_matches() {
        check(ssx3_ee::s07::f_00266d10__generated, voip_man_dtor, 60, &|c, seed| {
            let _ = machine(c, seed, 0x266d10, 0);
            be_library(c);
            c.g[A0][0] = sx(DATA + 0x100);
            c.g[A1][0] = (seed & 1) as u64;
        });
    }

    #[test]
    fn voip_man_deactivate_matches() {
        check(
            ssx3_ee::s07::f_00266e88__generated,
            voip_man_deactivate,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x266e88, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                c.bus.write32(this_, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(this_ + 4, if seed % 4 < 2 { 0 } else { 1 });
                c.g[A0][0] = sx(this_);
            },
        );
    }

    #[test]
    fn path_data_has_marker_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 0x26b598, 0);
            be_library(c);
            let path = DATA + 0x100;
            let count: u32 = match seed % 5 {
                0 => 0,
                1 => 0x8000_0000,
                _ => 5,
            };
            c.bus.write32(path, count);
            c.bus.write32(path + 4, DATA + 0x1000);
            for i in 0..5u32 {
                let e = DATA + 0x1000 + 0x28 * i;
                c.bus.write32(e, i);
                c.bus.write32(e + 4, 100 + i);
            }
            // Match the first marker, the third, or none.
            let (a, b): (u64, u64) = match seed % 3 {
                0 => (100, 0),
                1 => (102, 2),
                _ => (999, 9),
            };
            c.g[A0][0] = sx(path);
            c.g[A1][0] = a;
            c.g[A2][0] = b;
            let mode = schedule(c, &mut r, seed, 0x0026_b598);
            if mode != Events::Off {
                // The walk is at most five entries: make the event due at the
                // first back edge so the event path is really covered.
                c.next_event = c.cycles + (r.next_u32() % 4) as u64;
            }
            mode
        };
        check_events(ssx3_ee::s07::f_0026b598__generated, path_data_has_marker, 80, &setup);
    }

    #[test]
    fn replay_cache_dtor_matches() {
        check(
            ssx3_ee::s07::f_0026d0e8__generated,
            replay_cache_dtor,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x26d0e8, 0);
                be_library(c);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = (seed & 1) as u64;
            },
        );
    }

    

    #[test]
    fn replay_stream_free_buffer_matches() {
        check(
            ssx3_ee::s07::f_0026e968__generated,
            replay_stream_free_buffer,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x26e968, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                c.bus
                    .write32(this_ + 4, if seed % 2 == 0 { 0 } else { DATA + 0x2000 });
                c.g[A0][0] = sx(this_);
            },
        );
    }

    #[test]
    fn replay_stream_read_matches() {
        check(
            ssx3_ee::s07::f_0026ea20__generated,
            replay_stream_read,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x26ea20, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                c.bus.write32(this_ + 0x0c, 0x100);
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = (seed % 0x40) as u64;
            },
        );
    }

    

    #[test]
    fn replay_end_instant_replay_matches() {
        check(
            ssx3_ee::s07::f_00270670__generated,
            replay_end_instant_replay,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x270670, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                // Only the idle path: an active playback runs the whole replay
                // end-of-playback chain, which needs a live replay object.
                c.bus.write32(this_ + 0x620, 0);
                c.g[A0][0] = sx(this_);
            },
        );
    }

    #[test]
    fn replay_free_save_buffers_matches() {
        check(
            ssx3_ee::s07::f_00270f78__generated,
            replay_free_save_buffers,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x270f78, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                c.bus
                    .write32(this_ + 0x63c, if seed % 3 == 0 { 0 } else { DATA + 0x2000 });
                c.bus
                    .write32(this_ + 0x640, if seed % 2 == 0 { 0 } else { DATA + 0x2400 });
                c.g[A0][0] = sx(this_);
            },
        );
    }

    #[test]
    fn in_seq_channel_dtor_matches() {
        check(
            ssx3_ee::s07::f_00273970__generated,
            in_seq_channel_dtor,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x273970, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                c.bus.write32(this_ + 0x14, DATA + 0x2000);
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = (seed & 1) as u64;
            },
        );
    }

    #[test]
    fn in_seq_channel_select_block_matches() {
        check(
            ssx3_ee::s07::f_00273a60__generated,
            in_seq_channel_select_block,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x273a60, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                let offsets = DATA + 0x2000;
                let blocks = DATA + 0x3000;
                c.bus.write32(this_ + 0x04, blocks);
                c.bus.write32(this_ + 0x14, offsets);
                // The selected block's first event record has a small/large count.
                let idx: u32 = seed % 4;
                c.bus.write32(offsets + 4 * idx, 0x40 * idx);
                c.bus
                    .write16(blocks + 0x40 * idx, if seed % 3 == 0 { 0 } else { 12 });
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = idx as u64;
            },
        );
    }

    #[test]
    fn script_anim_bank_man_remove_bank_matches() {
        check(
            ssx3_ee::s07::f_00275650__generated,
            script_anim_bank_man_remove_bank,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x275650, 0);
                be_library(c);
                let mgr = DATA + 0x100;
                let banks = DATA + 0x2000;
                let count = [0u32, 1, 2, 4][seed as usize % 4];
                c.bus.write32(mgr + 4, count);
                c.bus.write32(mgr + 8, banks);
                for i in 0..4u32 {
                    let e = banks + 0x14 * i;
                    c.bus.write32(e, i + 1);
                    c.bus.write32(e + 8, if i < count { 1 } else { 0 });
                }
                // A bank list method is only called when the bank is found.
                let vt = DATA + 0x3000;
                c.bus.write32(mgr + 0x0c, vt);
                fake_method(c, vt, 0x10);
                c.g[A0][0] = sx(mgr);
                c.g[A1][0] = 1 + (seed % 4) as u64;
            },
        );
    }

    #[test]
    fn script_anim_bank_man_find_bank_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 0x275718, 0);
            be_library(c);
            let mgr = DATA + 0x100;
            let banks = DATA + 0x2000;
            let count: u32 = [0, 1, 2, 6, 0x8000_0000][seed as usize % 5];
            c.bus.write32(mgr + 4, count);
            c.bus.write32(mgr + 8, banks);
            for i in 0..6u32 {
                let e = banks + 0x14 * i;
                c.bus.write32(e, i);
                c.bus.write32(e + 8, if i == 2 || i == 4 { 0 } else { 1 });
            }
            c.g[A0][0] = sx(mgr);
            c.g[A1][0] = [0u64, 2, 4, 5, 9][seed as usize % 5];
            let _ = r.next_u32();
            let ev = schedule(c, &mut r, seed, 0x0027_5718);
            if ev != Events::Off {
                c.next_event = c.cycles + (r.next_u32() % 4) as u64;
            }
            ev
        };
        check_events(
            ssx3_ee::s07::f_00275718__generated,
            script_anim_bank_man_find_bank,
            80,
            &setup,
        );
    }

    #[test]
    fn script_engine_is_load_done_matches() {
        check(
            ssx3_ee::s07::f_00278d58__generated,
            script_engine_is_load_done,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x278d58, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                // A pending async load, or none (then bank slot 1 is checked).
                c.bus
                    .write32(this_ + 0x538, if seed % 2 == 0 { 0 } else { DATA + 0x2000 });
                c.bus.write32(this_ + 0x620, 0);
                c.g[A0][0] = sx(this_);
            },
        );
    }

    #[test]
    fn script_engine_is_active_list_skippable_matches() {
        check(
            ssx3_ee::s07::f_00278da0__generated,
            script_engine_is_active_list_skippable,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x278da0, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                let list = DATA + 0x2000;
                c.bus
                    .write32(this_ + 0x550, if seed % 2 == 0 { 2 } else { 0 });
                c.bus.write32(this_ + 0x54c, list);
                // Not state 3: the list accessor answers without looking further.
                c.bus
                    .write32(list + 0xa4, if seed % 4 == 0 { 3 } else { 0 });
                c.bus.write32(list + 0x0c, list + 0x100);
                c.g[A0][0] = sx(this_);
            },
        );
    }

    #[test]
    fn script_engine_list_get_state_matches() {
        check(
            ssx3_ee::s07::f_002791d8__generated,
            script_engine_list_get_state,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x2791d8, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                let lists = DATA + 0x2000;
                c.bus.write32(this_ + 0x54c, lists);
                c.bus.write32(this_ + 0x620, 0);
                for i in 0..4u32 {
                    c.bus
                        .write32(lists + 0xcc * i + 0xa4, if i == 1 { 1 } else { 0 });
                }
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = (seed % 4) as u64;
            },
        );
    }

    #[test]
    fn script_engine_get_active_list_field_b0_matches() {
        check(
            ssx3_ee::s07::f_00279298__generated,
            script_engine_get_active_list_field_b0,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x279298, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                let lists = DATA + 0x2000;
                c.bus
                    .write32(this_ + 0x550, if seed % 2 == 0 { 2 } else { 0 });
                c.bus.write32(this_ + 0x54c, lists);
                c.bus.write32(lists + 0xa4, 0);
                c.g[A0][0] = sx(this_);
            },
        );
    }

    #[test]
    fn script_engine_is_playing_script_matches() {
        check(
            ssx3_ee::s07::f_00279370__generated,
            script_engine_is_playing_script,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x279370, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                let lists = DATA + 0x2000;
                c.bus
                    .write32(this_ + 0x550, if seed % 2 == 0 { 2 } else { 0 });
                c.bus.write32(this_ + 0x54c, lists);
                c.bus.write32(lists + 0xa4, 0);
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = (seed % 4) as u64;
            },
        );
    }

    #[test]
    fn script_engine_cancel_script_bank_matches() {
        check(
            ssx3_ee::s07::f_00279840__generated,
            script_engine_cancel_script_bank,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x279840, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                // A free slot table: the lookup returns -1 and the cancel is a
                // no-op (no file handle to release).
                for i in 0..10u32 {
                    c.bus.write32(this_ + 0x62c + 0x10 * i, 0);
                }
                c.bus.write32(this_ + 0x620, 0);
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = (seed % 16) as u64;
            },
        );
    }
}
