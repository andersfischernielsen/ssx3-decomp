//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for the online lobby/buddy message-map thunks, replay
//! snapshot stepping, a handful of destructors and small audio/memory-card/
//! option-backend accessors, each with a differential test against the
//! generated body.
//!
//! - **Seventeen `*Thunk`s** (0x268d38..0x268fe0): `LobbyMan`/`BuddyMan` reply
//!   callbacks are registered with a small adapter that adds a per-message
//!   member offset (an `int16` in the class message map) to the object base
//!   passed in `a2`, then tail-calls the real handler with the message in
//!   `a1`. All seventeen share [`message_thunk`].
//! - **Replay snapshot stepping** (`Replay_FindNextSnapshot`,
//!   `Replay_FindPrevSnapshot`): find the snapshot for the current frame, then
//!   follow its `+0x14`/`+0x18` link when it is not the end of the list.
//! - **Destructors** (seven `*_dtor`): delete the object when bit 0 of the
//!   `flags` argument is set (the classic gcc `(flags & 1)` deleting dtor).
//! - **Sequence / audio / object helpers**: `InSeq_Stop`,
//!   `ScriptEngine_SeqListenerOnDestroy`, `SpeechInstance_StopStream`,
//!   `cWorldSound_OnSectionLoaded`, `cAngerFX_Reset`,
//!   `cPS2MemCard_ClearDirEntry`, `cPS2MemCard_v15/v16`.
//! - **Backend accessors**: the handicap option, the BAGT trick table entry,
//!   the six `BERewardInt` audio/song accessors and the option-state clear.

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Flow, Frame};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0026_8d38, lobby_man_on_move_to_room_reply_thunk, "LobbyMan_OnMoveToRoomReply_Thunk"),
    (0x0026_8d60, lobby_man_on_create_room_reply_thunk, "LobbyMan_OnCreateRoomReply_Thunk"),
    (0x0026_8d88, lobby_man_on_ping_reply_thunk, "LobbyMan_OnPingReply_Thunk"),
    (0x0026_8db0, lobby_man_on_report_user_reply_thunk, "LobbyMan_OnReportUserReply_Thunk"),
    (0x0026_8dd8, lobby_man_on_news_reply_thunk, "LobbyMan_OnNewsReply_Thunk"),
    (0x0026_8e00, lobby_man_on_snapshot_reply_thunk, "LobbyMan_OnSnapshotReply_Thunk"),
    (0x0026_8e28, lobby_man_on_lookup_user_reply_thunk, "LobbyMan_OnLookupUserReply_Thunk"),
    (0x0026_8e78, lobby_man_on_quick_play_reply_thunk, "LobbyMan_OnQuickPlayReplyThunk"),
    (0x0026_8ea0, lobby_man_on_chal_reply_thunk, "LobbyMan_OnChalReplyThunk"),
    (0x0026_8ec8, lobby_man_on_mesg_reply_thunk, "LobbyMan_OnMesgReplyThunk"),
    (0x0026_8ef0, lobby_man_on_rank_reply_thunk, "LobbyMan_OnRankReplyThunk"),
    (0x0026_8f18, buddy_man_on_connect_event_thunk, "BuddyMan_OnConnectEventThunk"),
    (0x0026_8f40, buddy_man_on_message_received_thunk, "BuddyMan_OnMessageReceivedThunk"),
    (0x0026_8f68, buddy_man_on_roster_event_thunk, "BuddyMan_OnRosterEventThunk"),
    (0x0026_8f90, buddy_man_on_roster_remove_reply_thunk, "BuddyMan_OnRosterRemoveReplyThunk"),
    (0x0026_8fb8, buddy_man_on_roster_add_reply_thunk, "BuddyMan_OnRosterAddReplyThunk"),
    (0x0026_8fe0, buddy_man_on_send_message_reply_thunk, "BuddyMan_OnSendMessageReplyThunk"),
    (0x0026_e468, replay_find_next_snapshot, "Replay_FindNextSnapshot"),
    (0x0026_e490, replay_find_prev_snapshot, "Replay_FindPrevSnapshot"),
    (0x0027_28d0, in_seq_dtor, "InSeq_dtor"),
    (0x0027_2b58, in_seq_stop, "InSeq_Stop"),
    (0x0028_2b88, script_engine_seq_listener_on_destroy, "ScriptEngine_SeqListenerOnDestroy"),
    (0x002a_ba18, bank_monitor_instance_dtor, "BankMonitorInstance_dtor"),
    (0x002a_d860, bank_monitor_timer_dtor, "BankMonitor_TimerDtor"),
    (0x002b_0320, speech_instance_stop_stream, "SpeechInstance_StopStream"),
    (0x002b_4be0, world_trigger_instance_dtor, "WorldTriggerInstance_dtor"),
    (0x002b_6ae0, c_world_sound_dtor, "cWorldSound_dtor"),
    (0x002b_6b08, c_world_sound_on_section_loaded, "cWorldSound_OnSectionLoaded"),
    (0x002c_52d8, c_ps2_mem_card_clear_dir_entry, "cPS2MemCard_ClearDirEntry"),
    (0x002c_64a8, c_ps2_mem_card_v15, "cPS2MemCard_v15"),
    (0x002c_64d0, c_ps2_mem_card_v16, "cPS2MemCard_v16"),
    (0x002c_c048, menu_sys_dtor, "MenuSys_dtor"),
    (0x002d_4be0, c_anger_fx_reset, "cAngerFX_Reset"),
    (0x002e_4f28, snow_box_dtor, "SnowBox_dtor"),
    (0x002e_e368, world_painter_get_irradiance2, "WorldPainter_GetIrradiance2"),
    (0x0035_0c90, dead_fade_kill, "DeadFade_Kill"),
    (0x0036_ce00, emitter_set_desc_curve_a, "Emitter_SetDescCurveA"),
    (0x003a_0c00, ui_text_get_owned_string, "UIText_GetOwnedString"),
    (0x003a_0c28, ui_text_get_id_string, "UIText_GetIdString"),
    (0x0014_f7e8, be_option_int_get_computer_handicap, "BEOptionInt_GetComputerHandicap"),
    (0x0014_f870, be_option_int_get_max_handicap, "BEOptionInt_GetMaxHandicap"),
    (0x0014_ff90, be_bagt_int_get_trick_table_entry, "BEBAGTInt_GetTrickTableEntry"),
    (0x0015_8700, be_reward_int_get_audio_setting, "BERewardInt_GetAudioSetting"),
    (0x0015_8728, be_reward_int_get_unlocked_songs, "BERewardInt_GetUnlockedSongs"),
    (0x0015_8750, be_reward_int_get_enabled_songs, "BERewardInt_GetEnabledSongs"),
    (0x0015_87f8, be_reward_int_set_audio_setting, "BERewardInt_SetAudioSetting"),
    (0x0015_8820, be_reward_int_set_unlocked_songs, "BERewardInt_SetUnlockedSongs"),
    (0x0015_8848, be_reward_int_set_enabled_songs, "BERewardInt_SetEnabledSongs"),
    (0x0015_c228, be_clear_option_state_5308b8, "BE_ClearOptionState5308b8"),
    (0x0016_7e68, camera_algorithm_tick, "CameraAlgorithm_Tick"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

mod callee {
    use super::{ee, Callee};

    pub const REPLAY_FIND_SNAPSHOT: Callee =
        Callee { addr: 0x0026_e448, f: ee::s07::Replay_FindSnapshot };
    pub const INSEQ_GET_INSTANCE: Callee =
        Callee { addr: 0x0027_2cc0, f: ee::s07::InSeq_GetInstance };
    pub const INSEQ_INSTANCE_STOP: Callee =
        Callee { addr: 0x0027_4918, f: ee::s07::InSeq_InstanceStop };
    pub const SCRIPT_ENGINE_CLEAR_PROCESS: Callee =
        Callee { addr: 0x0028_2c88, f: ee::s07::ScriptEngine_ClearProcess };
    pub const STREAM_ARRAY_STOP: Callee =
        Callee { addr: 0x002a_ae40, f: ee::s08::StreamArray_Stop };
    pub const C_WORLD_SOUND_RELOCATE_EMITTERS: Callee =
        Callee { addr: 0x002b_6b50, f: ee::s08::cWorldSound_RelocateEmitters };
    pub const USTR_FROM_ASCII_2: Callee =
        Callee { addr: 0x002c_2540, f: ee::s08::UStr_FromAscii_2 };
    pub const WORLD_PAINTER_GET_IRRADIANCE_NAME2: Callee =
        Callee { addr: 0x002e_f198, f: ee::s09::WorldPainter_GetIrradianceName2 };
    pub const IRRADIANCE_REF_GET_ENTRY: Callee =
        Callee { addr: 0x002e_e010, f: ee::s09::IrradianceRef_GetEntry };
    pub const BE_OPTION_INT_GET_MAX_HANDICAP: Callee =
        Callee { addr: 0x0014_f870, f: ee::s01::BEOptionInt_GetMaxHandicap };
}

/// Cross-shard calls the generator emitted as `call_indirect(c, addr)`: they
/// resolve through the function table like the original's `jal`.
mod table_callee {
    /// `operator delete(p)` (`__builtin_delete`).
    pub const OPERATOR_DELETE: u32 = 0x0031_7e50;
    /// `memset(dst, value, size)` (`EA_MemSet`).
    pub const EA_MEMSET: u32 = 0x003e_6448;
    /// `World_CreateDeadNode(world, position)`.
    pub const WORLD_CREATE_DEAD_NODE: u32 = 0x002d_1cc0;
}

// ---------------------------------------------------------------------------
// Message-map thunks.

/// Base of the message maps: the thunk's `lui v0, 0x4a` and the `int16`
/// member offsets live at `0x4a3148..` (`D_004A3148` and friends).
const MESSAGE_MAP: u32 = 0x004a_0000;

/// The shared body of the reply-callback thunks:
/// `handler(a2 + message_map[entry], a1, ...)`.
///
/// `entry` is the absolute address of the message's `int16` member offset,
/// `target` the handler and `ret` the return address the original materialises
/// in `ra` before the tail call (it keeps the `0x10` frame so the handler's own
/// frame starts where the original's does).
fn message_thunk(c: &mut Cpu, entry: u32, target: u32, ret: u32) -> Flow {
    c.cycles += 9;
    let frame = Frame::open(c, 0x10);
    // The original materialises the map base in v0, then loads the offset.
    c.g[V0][0] = MESSAGE_MAP as u64;
    let this = (c.g[A2][0] as u32).wrapping_add(Addr(entry).lh(c) as u32);
    frame.save_ra(c, 0);
    c.g[A0][0] = sx(this);
    call_ptr(c, target, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void LobbyMan_OnMoveToRoomReply_Thunk(LobbyMan *self, void *msg, void *obj)`
/// (0x268d38): `LobbyMan_OnMoveToRoomReply(obj + D_004A3148, msg)`.
pub fn lobby_man_on_move_to_room_reply_thunk(c: &mut Cpu) {
    guest(c, |c| message_thunk(c, 0x004a_3148, 0x0025_d8f8, 0x0026_8d50))
}

/// `void LobbyMan_OnCreateRoomReply_Thunk(...)` (0x268d60):
/// `LobbyMan_OnCreateRoomReply(obj + D_004A3158, msg)`.
pub fn lobby_man_on_create_room_reply_thunk(c: &mut Cpu) {
    guest(c, |c| message_thunk(c, 0x004a_3158, 0x0025_db80, 0x0026_8d78))
}

/// `void LobbyMan_OnPingReply_Thunk(...)` (0x268d88):
/// `LobbyMan_OnPingReply(obj + D_004A3160, msg)`.
pub fn lobby_man_on_ping_reply_thunk(c: &mut Cpu) {
    guest(c, |c| message_thunk(c, 0x004a_3160, 0x0025_dde0, 0x0026_8da0))
}

/// `void LobbyMan_OnReportUserReply_Thunk(...)` (0x268db0):
/// `LobbyMan_OnReportUserReply(obj + D_004A3170, msg)`.
pub fn lobby_man_on_report_user_reply_thunk(c: &mut Cpu) {
    guest(c, |c| message_thunk(c, 0x004a_3170, 0x0025_df98, 0x0026_8dc8))
}

/// `void LobbyMan_OnNewsReply_Thunk(...)` (0x268dd8):
/// `LobbyMan_OnNewsReply(obj + D_004A3178, msg)`.
pub fn lobby_man_on_news_reply_thunk(c: &mut Cpu) {
    guest(c, |c| message_thunk(c, 0x004a_3178, 0x0025_e138, 0x0026_8df0))
}

/// `void LobbyMan_OnSnapshotReply_Thunk(...)` (0x268e00):
/// `LobbyMan_OnSnapshotReply(obj + D_004A31B0, msg)`.
pub fn lobby_man_on_snapshot_reply_thunk(c: &mut Cpu) {
    guest(c, |c| message_thunk(c, 0x004a_31b0, 0x0025_e590, 0x0026_8e18))
}

/// `void LobbyMan_OnLookupUserReply_Thunk(...)` (0x268e28):
/// `LobbyMan_OnLookupUserReply(obj + D_004A31B8, msg)`.
pub fn lobby_man_on_lookup_user_reply_thunk(c: &mut Cpu) {
    guest(c, |c| message_thunk(c, 0x004a_31b8, 0x0025_e7f0, 0x0026_8e40))
}

/// `void LobbyMan_OnQuickPlayReplyThunk(...)` (0x268e78):
/// `LobbyMan_OnQuickPlayReply(obj + D_004A3210, msg)`.
pub fn lobby_man_on_quick_play_reply_thunk(c: &mut Cpu) {
    guest(c, |c| message_thunk(c, 0x004a_3210, 0x0025_f030, 0x0026_8e90))
}

/// `void LobbyMan_OnChalReplyThunk(...)` (0x268ea0):
/// `LobbyMan_OnChalReply(obj + D_004A3248, msg)`.
pub fn lobby_man_on_chal_reply_thunk(c: &mut Cpu) {
    guest(c, |c| message_thunk(c, 0x004a_3248, 0x0025_f8a0, 0x0026_8eb8))
}

/// `void LobbyMan_OnMesgReplyThunk(...)` (0x268ec8):
/// `LobbyMan_OnMesgReply(obj + D_004A3268, msg)`.
pub fn lobby_man_on_mesg_reply_thunk(c: &mut Cpu) {
    guest(c, |c| message_thunk(c, 0x004a_3268, 0x0025_fc20, 0x0026_8ee0))
}

/// `void LobbyMan_OnRankReplyThunk(...)` (0x268ef0):
/// `LobbyMan_OnRankReply(obj + D_004A3318, msg)`.
pub fn lobby_man_on_rank_reply_thunk(c: &mut Cpu) {
    guest(c, |c| message_thunk(c, 0x004a_3318, 0x0026_0c68, 0x0026_8f08))
}

/// `void BuddyMan_OnConnectEventThunk(...)` (0x268f18):
/// `BuddyMan_OnConnectEvent(obj + D_004A3340, msg)`.
pub fn buddy_man_on_connect_event_thunk(c: &mut Cpu) {
    guest(c, |c| message_thunk(c, 0x004a_3340, 0x0026_15c8, 0x0026_8f30))
}

/// `void BuddyMan_OnMessageReceivedThunk(...)` (0x268f40):
/// `BuddyMan_OnMessageReceived(obj + D_004A3348, msg)`.
pub fn buddy_man_on_message_received_thunk(c: &mut Cpu) {
    guest(c, |c| message_thunk(c, 0x004a_3348, 0x0026_1a28, 0x0026_8f58))
}

/// `void BuddyMan_OnRosterEventThunk(...)` (0x268f68):
/// `BuddyMan_OnRosterEvent(obj + D_004A3350, msg)`.
pub fn buddy_man_on_roster_event_thunk(c: &mut Cpu) {
    guest(c, |c| message_thunk(c, 0x004a_3350, 0x0026_1770, 0x0026_8f80))
}

/// `void BuddyMan_OnRosterRemoveReplyThunk(...)` (0x268f90):
/// `BuddyMan_OnRosterRemoveReply(obj + D_004A3380, msg)`.
pub fn buddy_man_on_roster_remove_reply_thunk(c: &mut Cpu) {
    guest(c, |c| message_thunk(c, 0x004a_3380, 0x0026_1970, 0x0026_8fa8))
}

/// `void BuddyMan_OnRosterAddReplyThunk(...)` (0x268fb8):
/// `BuddyMan_OnRosterAddReply(obj + D_004A3388, msg)`.
pub fn buddy_man_on_roster_add_reply_thunk(c: &mut Cpu) {
    guest(c, |c| message_thunk(c, 0x004a_3388, 0x0026_18c0, 0x0026_8fd0))
}

/// `void BuddyMan_OnSendMessageReplyThunk(...)` (0x268fe0):
/// `BuddyMan_OnSendMessageReply(obj + D_004A33A0, msg)`.
pub fn buddy_man_on_send_message_reply_thunk(c: &mut Cpu) {
    guest(c, |c| message_thunk(c, 0x004a_33a0, 0x0026_1a20, 0x0026_8ff8))
}

// ---------------------------------------------------------------------------
// Replay snapshot stepping.

/// `Snapshot *Replay_FindNextSnapshot(Replay *r, int frame)` (0x26e468): the
/// snapshot found for `frame`, or its `+0x14` successor when it has one.
pub fn replay_find_next_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::REPLAY_FIND_SNAPSHOT.call(c, 0x0026_e478)?;
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x14)).lw(c);
        frame.restore_ra(c, 0);
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.g[V1][0];
        }
        frame.close(c);
        Ok(())
    })
}

/// `Snapshot *Replay_FindPrevSnapshot(Replay *r, int frame)` (0x26e490): the
/// snapshot found for `frame`, or its `+0x18` predecessor when it has one.
pub fn replay_find_prev_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::REPLAY_FIND_SNAPSHOT.call(c, 0x0026_e4a0)?;
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x18)).lw(c);
        frame.restore_ra(c, 0);
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.g[V1][0];
        }
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Deleting destructors.

/// The shared body of the `(flags & 1)` deleting destructors: delete `this`
/// when the low bit of `a1` is set (the frame is still opened either way).
fn destroy_if_flag(c: &mut Cpu, ret: u32) -> Flow {
    c.cycles += 4;
    let frame = Frame::open(c, 0x10);
    c.g[A1][0] &= 1;
    frame.save_ra(c, 0);
    if c.g[A1][0] != 0 {
        c.cycles += 2;
        call_ptr(c, table_callee::OPERATOR_DELETE, ret)?;
    }
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void InSeq_dtor(InSeq *this, int flags)` (0x2728d0).
pub fn in_seq_dtor(c: &mut Cpu) {
    guest(c, |c| destroy_if_flag(c, 0x0027_28e8))
}

/// `void BankMonitorInstance_dtor(BankMonitorInstance *this, int flags)`
/// (0x2aba18).
pub fn bank_monitor_instance_dtor(c: &mut Cpu) {
    guest(c, |c| destroy_if_flag(c, 0x002a_ba30))
}

/// `void BankMonitor_TimerDtor(BankMonitor_Timer *this, int flags)`
/// (0x2ad860).
pub fn bank_monitor_timer_dtor(c: &mut Cpu) {
    guest(c, |c| destroy_if_flag(c, 0x002a_d878))
}

/// `void WorldTriggerInstance_dtor(WorldTriggerInstance *this, int flags)`
/// (0x2b4be0).
pub fn world_trigger_instance_dtor(c: &mut Cpu) {
    guest(c, |c| destroy_if_flag(c, 0x002b_4bf8))
}

/// `void cWorldSound_dtor(cWorldSound *this, int flags)` (0x2b6ae0).
pub fn c_world_sound_dtor(c: &mut Cpu) {
    guest(c, |c| destroy_if_flag(c, 0x002b_6af8))
}

/// `void MenuSys_dtor(MenuSys *this, int flags)` (0x2cc048).
pub fn menu_sys_dtor(c: &mut Cpu) {
    guest(c, |c| destroy_if_flag(c, 0x002c_c060))
}

/// `void SnowBox_dtor(SnowBox *this, int flags)` (0x2e4f28).
pub fn snow_box_dtor(c: &mut Cpu) {
    guest(c, |c| destroy_if_flag(c, 0x002e_4f40))
}

// ---------------------------------------------------------------------------
// Sequence / audio / object helpers.

/// `void InSeq_Stop(InSeq *self, int slot)` (0x272b58): fetch the sequence
/// instance for `slot` and stop it.
pub fn in_seq_stop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::INSEQ_GET_INSTANCE.call(c, 0x0027_2b68)?;
        c.g[A0][0] = c.g[V0][0];
        callee::INSEQ_INSTANCE_STOP.call(c, 0x0027_2b70)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptEngine_SeqListenerOnDestroy(ScriptEngine *self, SeqListener *l)`
/// (0x282b88): take the listener's process (`l->+0xc`), mark it destroyed
/// (`state = 7` at `+0x4`) and clear it.
pub fn script_engine_seq_listener_on_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 7; // process state: destroyed
        frame.save_ra(c, 0);
        let ctx = Addr((c.g[A1][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[A1][0] = ctx;
        Addr((ctx as u32).wrapping_add(4)).sw(c, c.g[V0][0]);
        callee::SCRIPT_ENGINE_CLEAR_PROCESS.call(c, 0x0028_2ba0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SpeechInstance_StopStream(SpeechInstance *this)` (0x2b0320): stop the
/// stream array's element this instance refers to (`+0x4`).
pub fn speech_instance_stop_stream(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        // The global holds a pointer to the owner whose first word is the
        // stream-array singleton (`D_004A37B4`, `gp + 0x6c4`).
        let owner = Addr((c.g[GP][0] as u32).wrapping_add(0x6c4)).lw(c);
        c.g[V0][0] = owner;
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(4)).lw(c);
        c.g[A0][0] = Addr(owner as u32).lw(c);
        callee::STREAM_ARRAY_STOP.call(c, 0x002b_0338)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool cWorldSound_OnSectionLoaded(cWorldSound *this, Section *s)`
/// (0x2b6b08): relocate the emitters when a section arrived; always true.
pub fn c_world_sound_on_section_loaded(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        let section = c.g[A1][0];
        frame.save_ra(c, 0);
        if section != 0 {
            c.cycles += 2;
            callee::C_WORLD_SOUND_RELOCATE_EMITTERS.call(c, 0x002b_6b1c)?;
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 1;
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_ClearDirEntry(cPS2MemCard *this)` (0x2c52d8): zero the
/// 0x40-byte directory entry at `this + 0x480`.
pub fn c_ps2_mem_card_clear_dir_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x480));
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x40;
        call_ptr(c, table_callee::EA_MEMSET, 0x002c_52f0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared body of the two `cPS2MemCard` string accessors: build the UTF-16
/// string for the ASCII text in `a2` (the `a0`/`a1` arguments are unused).
fn mem_card_string(c: &mut Cpu, ret: u32) -> Flow {
    c.cycles += 9;
    let frame = Frame::open(c, 0x10);
    c.g[A1][0] = MESSAGE_MAP as u64;
    frame.save_ra(c, 0);
    c.g[A0][0] = c.g[A2][0];
    c.g[A1][0] = sx(MESSAGE_MAP.wrapping_add(0x3968));
    callee::USTR_FROM_ASCII_2.call(c, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `UStr cPS2MemCard_v15(cPS2MemCard *this, int unused, char *ascii)`
/// (0x2c64a8): `UStr_FromAscii_2(ascii, D_004A3968)`.
pub fn c_ps2_mem_card_v15(c: &mut Cpu) {
    guest(c, |c| mem_card_string(c, 0x002c_64c0))
}

/// `UStr cPS2MemCard_v16(cPS2MemCard *this, int unused, char *ascii)`
/// (0x2c64d0): `UStr_FromAscii_2(ascii, D_004A3968)`.
pub fn c_ps2_mem_card_v16(c: &mut Cpu) {
    guest(c, |c| mem_card_string(c, 0x002c_64e8))
}

/// `void cAngerFX_Reset(cAngerFX *this)` (0x2d4be0): zero the 0x58-byte state
/// object at `this->state` (`+0x4`).
pub fn c_anger_fx_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0;
        frame.save_ra(c, 0);
        c.g[A2][0] = 0x58;
        c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(4)).lw(c);
        call_ptr(c, table_callee::EA_MEMSET, 0x002d_4bf8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Irradiance *WorldPainter_GetIrradiance2(void)` (0x2ee368): resolve the
/// painter's irradiance name to a reference, then look the entry up.
pub fn world_painter_get_irradiance2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::WORLD_PAINTER_GET_IRRADIANCE_NAME2.call(c, 0x002e_e378)?;
        c.g[A0][0] = c.g[V0][0];
        callee::IRRADIANCE_REF_GET_ENTRY.call(c, 0x002e_e380)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void DeadFade_Kill(DeadFade *this)` (0x350c90): spawn a dead node in the
/// world (`this->world` at `+0x18`) at `this + 0x1c`.
pub fn dead_fade_kill(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x18)).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1c));
        call_ptr(c, table_callee::WORLD_CREATE_DEAD_NODE, 0x0035_0ca8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Emitter curve.

/// `void Emitter_SetDescCurveA(EmitterDesc *d, float a, float b, float c)`
/// (0x36ce00): store the third value, the `b - a` span, and `a - 1.5 * c` into
/// the curve descriptor at `+0x10`, `+0x20` and `+0x18`.
pub fn emitter_set_desc_curve_a(c: &mut Cpu) {
    c.cycles += 9;
    let base = c.g[A0][0] as u32;
    let a = FpuF32(c.f[12]);
    let b = FpuF32(c.f[13]);
    let c3 = FpuF32(c.f[14]);
    c.g[AT][0] = 0x3fc0_0000; // 1.5
    c.mtc1(AT, 0);
    c.f[13] = (b - a).0; // span into f13
    Addr(base.wrapping_add(0x10)).write_fpu(c, c3);
    c.f[0] = (c3 * FpuF32(c.f[0])).0; // 1.5 * c
    Addr(base.wrapping_add(0x20)).write_fpu(c, FpuF32(c.f[13]));
    c.f[12] = (a - FpuF32(c.f[0])).0; // a - 1.5 * c
    Addr(base.wrapping_add(0x18)).write_fpu(c, FpuF32(c.f[12]));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// UI text.

/// `UStr UIText_GetOwnedString(UIText *this)` (0x3a0c00): the owned string at
/// `+0xb4`, or 0 when bit 3 of the flags at `+0x74` is set.
pub fn ui_text_get_owned_string(c: &mut Cpu) {
    c.cycles += 5;
    let this = c.g[A0][0] as u32;
    let flags = Addr(this.wrapping_add(0x74)).lw(c);
    c.g[V0][0] = flags;
    c.g[V0][0] = sx(((flags as i32) >> 3) as u32);
    c.g[V0][0] &= 1;
    let owned = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if owned {
        c.cycles += 2;
        abi::ret(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = Addr(this.wrapping_add(0xb4)).lw(c);
        abi::ret(c);
    }
}

/// `UStr UIText_GetIdString(UIText *this)` (0x3a0c28): the id string at
/// `+0xb4`, or 0 when bit 3 of the flags at `+0x74` is clear.
pub fn ui_text_get_id_string(c: &mut Cpu) {
    c.cycles += 5;
    let this = c.g[A0][0] as u32;
    let flags = Addr(this.wrapping_add(0x74)).lw(c);
    c.g[V0][0] = flags;
    c.g[V0][0] = sx(((flags as i32) >> 3) as u32);
    c.g[V0][0] &= 1;
    let owned = c.g[V0][0] == 0;
    c.g[V0][0] = 0;
    if owned {
        c.cycles += 2;
        abi::ret(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = Addr(this.wrapping_add(0xb4)).lw(c);
        abi::ret(c);
    }
}

// ---------------------------------------------------------------------------
// Option / reward backend accessors.

/// `int BEOptionInt_GetComputerHandicap(void)` (0x14f7e8): the maximum
/// handicap minus the configured computer handicap (`D_005308D4`).
pub fn be_option_int_get_computer_handicap(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::BE_OPTION_INT_GET_MAX_HANDICAP.call(c, 0x0014_f7f8)?;
        c.g[V1][0] = 0x0053_0000;
        frame.restore_ra(c, 0);
        let configured = Addr(0x0053_08d4).lw(c);
        c.g[A0][0] = configured;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(configured as u32));
        frame.close(c);
        Ok(())
    })
}

/// `int BEOptionInt_GetMaxHandicap(void)` (0x14f870): the largest of the three
/// handicap limits in the option state at `0x5308b8`.
pub fn be_option_int_get_max_handicap(c: &mut Cpu) {
    c.cycles += 10;
    let base = 0x0053_08b8u32;
    c.g[V1][0] = sx(base);
    c.g[V0][0] = Addr(base.wrapping_add(0x1c)).lw(c);
    c.g[A1][0] = Addr(base.wrapping_add(0x8)).lw(c);
    c.g[A2][0] = Addr(base.wrapping_add(0x14)).lw(c);
    c.g[A0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
    if c.g[A0][0] != 0 {
        c.g[V0][0] = c.g[A1][0];
    }
    c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[A2][0] as i64)) as u64;
    let ret = c.g[RA][0] as u32;
    if c.g[V1][0] != 0 {
        c.g[V0][0] = c.g[A2][0];
    }
    c.pc = ret;
}

/// `sTrick *BEBAGTInt_GetTrickTableEntry(void *this, int rider, int idx,
/// int *count)` (0x14ff90): entry `idx` of the 0x14-byte trick table
/// (`D_0045AEB8`), its `+0x10` value in v0 and its `+0xc` count stored in
/// `*count`.
pub fn be_bagt_int_get_trick_table_entry(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V0][0] = 0x14; // entry stride
    c.g[V1][0] = 0x0046_0000;
    c.mult(0, A2, A2, V0);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5148)); // &D_0045AEB8
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lhu(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x10)).lw(c);
    let ret = c.g[RA][0] as u32;
    c.sw(c.g[A3][0] as u32, c.g[A0][0]);
    c.pc = ret;
}

/// The per-player (`a1`, stride 0x9b50) per-slot (`a2`, stride 0xf88) reward
/// record inside `D_004A6CA8`; returns its address.
fn reward_record(c: &mut Cpu) -> u32 {
    c.cycles += 10;
    c.g[V0][0] = 0x0f88;
    c.g[V1][0] = 0x9b50;
    c.mult(0, A2, A2, V0);
    c.mult(0, A1, A1, V1);
    c.g[V0][0] = sx(MESSAGE_MAP.wrapping_add(0x6ca8)); // &D_004A6CA8
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(MESSAGE_MAP.wrapping_add(0x6ca8)));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
    c.g[A1][0] as u32
}

/// `int BERewardInt_GetAudioSetting(void *this, int player, int slot)`
/// (0x158700): the record's `+0xf80` audio setting.
pub fn be_reward_int_get_audio_setting(c: &mut Cpu) {
    let rec = reward_record(c);
    c.g[V0][0] = Addr(rec.wrapping_add(0xf80)).lw(c);
    abi::ret(c);
}

/// `u64 BERewardInt_GetUnlockedSongs(void *this, int player, int slot)`
/// (0x158728): the record's `+0xf70` unlocked-song mask.
pub fn be_reward_int_get_unlocked_songs(c: &mut Cpu) {
    let rec = reward_record(c);
    c.g[V0][0] = Addr(rec.wrapping_add(0xf70)).ld(c);
    abi::ret(c);
}

/// `u64 BERewardInt_GetEnabledSongs(void *this, int player, int slot)`
/// (0x158750): the record's `+0xf78` enabled-song mask.
pub fn be_reward_int_get_enabled_songs(c: &mut Cpu) {
    let rec = reward_record(c);
    c.g[V0][0] = Addr(rec.wrapping_add(0xf78)).ld(c);
    abi::ret(c);
}

/// `void BERewardInt_SetAudioSetting(void *this, int player, int slot,
/// int value)` (0x1587f8).
pub fn be_reward_int_set_audio_setting(c: &mut Cpu) {
    let rec = reward_record(c);
    Addr(rec.wrapping_add(0xf80)).sw(c, c.g[A3][0]);
    abi::ret(c);
}

/// `void BERewardInt_SetUnlockedSongs(void *this, int player, int slot,
/// u64 mask)` (0x158820).
pub fn be_reward_int_set_unlocked_songs(c: &mut Cpu) {
    let rec = reward_record(c);
    Addr(rec.wrapping_add(0xf70)).sd(c, c.g[A3][0]);
    abi::ret(c);
}

/// `void BERewardInt_SetEnabledSongs(void *this, int player, int slot,
/// u64 mask)` (0x158848).
pub fn be_reward_int_set_enabled_songs(c: &mut Cpu) {
    let rec = reward_record(c);
    Addr(rec.wrapping_add(0xf78)).sd(c, c.g[A3][0]);
    abi::ret(c);
}

/// `void BE_ClearOptionState5308b8(void)` (0x15c228): zero the 0x20-byte option
/// state at `0x5308b8`.
pub fn be_clear_option_state_5308b8(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x0053_0000;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
        c.g[A2][0] = 0x20;
        call_ptr(c, table_callee::EA_MEMSET, 0x0015_c244)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CameraAlgorithm_Tick(cCameraAlgorithm *this)` (0x167e68): call the
/// algorithm's virtual `v05` at `vt + 0x2c` with `this` shifted by the `+0x28`
/// delta.
pub fn camera_algorithm_tick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let obj = Addr((c.g[A0][0] as u32).wrapping_add(0x10)).lw(c);
        c.g[V0][0] = obj;
        c.g[V1][0] = Addr((obj as u32).wrapping_add(0x28)).lh(c);
        let method = Addr((obj as u32).wrapping_add(0x2c)).lw(c);
        c.g[A1][0] = method;
        c.g[RA][0] = sx(0x0016_7e84);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        call_ptr(c, method as u32, 0x0016_7e84)?;
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
    use crate::difftest::fakes::{self, machine, ptr};
    use crate::difftest::{check, Rng, DATA};

    /// `this`/`self` scratch object, message object and other fixtures all
    /// live in the (unused) data area the differential tests own.
    const SCRATCH: u32 = DATA + 0x1000;

    /// The game singletons a real callee may reach: a zeroed world at
    /// `gp - 0x848` (`+0x84` module, `+0xc` race manager, no riders at `+0x7c`)
    /// and a zeroed BE library at `gp - 0x1f04`, so nothing tries to build one
    /// through the file system or walk a rider loop.
    fn game_globals(c: &mut Cpu) {
        let gp = crate::difftest::GAME_GP;
        let world = SCRATCH + 0x9000;
        let module = SCRATCH + 0x9100;
        let race = SCRATCH + 0x9200;
        c.bus.write32(gp - 0x848, world);
        c.bus.write32(world + 0x84, module);
        c.bus.write32(module + 0xc, race);
        c.bus.write32(race + 0x7c, 0);
        c.bus.write32(gp - 0x1f04, SCRATCH + 0x9300);
        c.bus.write32(gp - 0x1efc, 0);
    }

    /// [`machine`] plus the game singletons (most callees read them).
    fn game(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        game_globals(c);
        r
    }

    /// A fixture for the message-map thunks: a fake-call machine (so the
    /// handler's own indirect calls and any unwind are compared), a reply
    /// message in `a1`, the object base in `a2`, and a small member offset in
    /// the message map.
    fn thunk_setup(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        let mut r = game(c, seed, salt, 1);
        let message = SCRATCH + 0x7000;
        // Half the seeds hand the handler a null message: the reply handlers
        // take their "nothing arrived" path then.
        c.g[A1][0] = if seed % 2 == 0 { 0 } else { message as u64 };
        c.g[A2][0] = SCRATCH as u64;
        c.g[A0][0] = ptr(&mut r) as u64;
        c.bus.write32(message + 8, if seed % 4 == 0 { r.next_u32() } else { 0 });
        c.bus.write32(message + 0xc, if seed % 4 == 1 { r.next_u32() % 8 } else { 0 });
        // The member offset the thunk adds to the object base (positive, so
        // the adjusted `this` stays inside the scratch area).
        c.bus.write16(entry, (r.next_u32() & 0xff8) as u16);
    }

    /// The engine's size-class pool allocator (`MEM_PoolAlloc`): blocks up to
    /// 0x400 bytes are popped from per-size free lists hanging off the seven
    /// pool globals `gp + 0x2a60..0x2a84`, but only when the "pools ready" flag
    /// at `gp + 0xdd8` is set. On a fresh test machine that flag is clear, so an
    /// allocation falls back to growing the `DLHeap` from a zeroed header; the
    /// reply handlers that allocate a list node then walk a corrupt bucket
    /// chain in `DLHeap_Consolidate` and never terminate. Give every pool a
    /// short free list of zeroed nodes instead: the handlers' allocations then
    /// just pop one, exactly as they do in the game (each `MEM_Pool*Alloc` is
    /// two loads and two stores off `pool + head`).
    fn pool_allocator(c: &mut Cpu) {
        let gp = crate::difftest::GAME_GP;
        c.bus.write32(gp + 0xdd8, 1); // pools ready
        // (pool pointer global, free-list head offset in the pool object) for
        // the 8, 16, 32, 64, 128, 256 and 512-byte size classes.
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let pool = 0x008c_0000 + 0x1000 * i;
            let base = 0x008b_0000 + 0x100 * i;
            // Four free nodes per class, spaced wider than the largest block
            // of that class (the head is updated on pop, so only the next
            // pointer of a node that is still queued is ever read).
            for n in 0..4u32 {
                let node = base + 0x40 * n;
                c.bus.write32(node, if n == 3 { 0 } else { node + 0x40 });
            }
            c.bus.write32(pool + head, base);
            c.bus.write32(gp + slot, pool);
        }
    }

    /// [`thunk_setup`] plus the pool allocator: for the reply handlers that
    /// allocate on every call.
    fn thunk_setup_alloc(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        thunk_setup(c, seed, salt, entry);
        pool_allocator(c);
    }

    #[test]
    fn lobby_man_on_move_to_room_reply_thunk_matches() {
        check(ee::s07::f_00268d38__generated, lobby_man_on_move_to_room_reply_thunk, 48, &|c, seed| {
            thunk_setup(c, seed, 0x268d38, 0x004a_3148);
        });
    }

    #[test]
    fn lobby_man_on_create_room_reply_thunk_matches() {
        check(ee::s07::f_00268d60__generated, lobby_man_on_create_room_reply_thunk, 48, &|c, seed| {
            thunk_setup(c, seed, 0x268d60, 0x004a_3158);
        });
    }

    #[test]
    fn lobby_man_on_ping_reply_thunk_matches() {
        check(ee::s07::f_00268d88__generated, lobby_man_on_ping_reply_thunk, 48, &|c, seed| {
            thunk_setup(c, seed, 0x268d88, 0x004a_3160);
        });
    }

    #[test]
    fn lobby_man_on_report_user_reply_thunk_matches() {
        check(ee::s07::f_00268db0__generated, lobby_man_on_report_user_reply_thunk, 48, &|c, seed| {
            thunk_setup(c, seed, 0x268db0, 0x004a_3170);
        });
    }

    #[test]
    fn lobby_man_on_news_reply_thunk_matches() {
        check(ee::s07::f_00268dd8__generated, lobby_man_on_news_reply_thunk, 48, &|c, seed| {
            thunk_setup(c, seed, 0x268dd8, 0x004a_3178);
        });
    }

    #[test]
    fn lobby_man_on_snapshot_reply_thunk_matches() {
        check(ee::s07::f_00268e00__generated, lobby_man_on_snapshot_reply_thunk, 48, &|c, seed| {
            thunk_setup(c, seed, 0x268e00, 0x004a_31b0);
        });
    }

    #[test]
    fn lobby_man_on_lookup_user_reply_thunk_matches() {
        check(ee::s07::f_00268e28__generated, lobby_man_on_lookup_user_reply_thunk, 48, &|c, seed| {
            thunk_setup_alloc(c, seed, 0x268e28, 0x004a_31b8);
        });
    }

    #[test]
    fn lobby_man_on_quick_play_reply_thunk_matches() {
        check(ee::s07::f_00268e78__generated, lobby_man_on_quick_play_reply_thunk, 48, &|c, seed| {
            thunk_setup(c, seed, 0x268e78, 0x004a_3210);
        });
    }

    #[test]
    fn lobby_man_on_chal_reply_thunk_matches() {
        check(ee::s07::f_00268ea0__generated, lobby_man_on_chal_reply_thunk, 48, &|c, seed| {
            thunk_setup(c, seed, 0x268ea0, 0x004a_3248);
        });
    }

    #[test]
    fn lobby_man_on_mesg_reply_thunk_matches() {
        check(ee::s07::f_00268ec8__generated, lobby_man_on_mesg_reply_thunk, 48, &|c, seed| {
            thunk_setup(c, seed, 0x268ec8, 0x004a_3268);
        });
    }

    #[test]
    fn lobby_man_on_rank_reply_thunk_matches() {
        check(ee::s07::f_00268ef0__generated, lobby_man_on_rank_reply_thunk, 48, &|c, seed| {
            thunk_setup(c, seed, 0x268ef0, 0x004a_3318);
        });
    }

    #[test]
    fn buddy_man_on_connect_event_thunk_matches() {
        check(ee::s07::f_00268f18__generated, buddy_man_on_connect_event_thunk, 48, &|c, seed| {
            thunk_setup(c, seed, 0x268f18, 0x004a_3340);
        });
    }

    #[test]
    fn buddy_man_on_message_received_thunk_matches() {
        check(ee::s07::f_00268f40__generated, buddy_man_on_message_received_thunk, 48, &|c, seed| {
            thunk_setup(c, seed, 0x268f40, 0x004a_3348);
        });
    }

    #[test]
    fn buddy_man_on_roster_event_thunk_matches() {
        check(ee::s07::f_00268f68__generated, buddy_man_on_roster_event_thunk, 48, &|c, seed| {
            thunk_setup_alloc(c, seed, 0x268f68, 0x004a_3350);
        });
    }

    #[test]
    fn buddy_man_on_roster_remove_reply_thunk_matches() {
        check(ee::s07::f_00268f90__generated, buddy_man_on_roster_remove_reply_thunk, 48, &|c, seed| {
            thunk_setup_alloc(c, seed, 0x268f90, 0x004a_3380);
        });
    }

    #[test]
    fn buddy_man_on_roster_add_reply_thunk_matches() {
        check(ee::s07::f_00268fb8__generated, buddy_man_on_roster_add_reply_thunk, 48, &|c, seed| {
            thunk_setup_alloc(c, seed, 0x268fb8, 0x004a_3388);
        });
    }

    #[test]
    fn buddy_man_on_send_message_reply_thunk_matches() {
        check(ee::s07::f_00268fe0__generated, buddy_man_on_send_message_reply_thunk, 48, &|c, seed| {
            thunk_setup(c, seed, 0x268fe0, 0x004a_33a0);
        });
    }

    // ------------------------------------------------------ replay snapshots

    /// A two-snapshot list at `+0x3b0` of the replay object; the frame searched
    /// for picks whether the found snapshot is the last one or not.
    fn snapshot_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = game(c, seed, salt, 1);
        let replay = SCRATCH;
        c.g[A0][0] = replay as u64;
        c.g[A1][0] = (if seed % 2 == 0 { 100u32 } else { 250u32 }) as u64;
        let a = SCRATCH + 0x200;
        let b = SCRATCH + 0x300;
        c.bus.write32(replay + 0x3b0, a);
        // a: frame 100, next/prev b; b: frame 200, end of the list.
        c.bus.write32(a + 0x14, b);
        c.bus.write32(a + 0x18, b);
        c.bus.write32(b + 0x14, 0);
        c.bus.write32(b + 0x18, 0);
        c.bus.write32(a + 0x30, 100);
        c.bus.write32(b + 0x30, 200u32 + r.next_u32() % 4);
        c.bus.write32(a + 0x34, r.next_u32() % 16);
    }

    #[test]
    fn replay_find_next_snapshot_matches() {
        check(ee::s07::f_0026e468__generated, replay_find_next_snapshot, 100, &|c, seed| {
            snapshot_setup(c, seed, 0x26e468);
        });
    }

    #[test]
    fn replay_find_prev_snapshot_matches() {
        check(ee::s07::f_0026e490__generated, replay_find_prev_snapshot, 100, &|c, seed| {
            snapshot_setup(c, seed, 0x26e490);
        });
    }

    // ------------------------------------------------------------ destructors

    /// `flags = seed % 2` so both paths run; on the deleting path `this` is a
    /// plain data pointer (the allocator tolerates it, as in its own test).
    fn dtor_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = game(c, seed, salt, 6);
        c.g[A0][0] = if seed % 8 == 0 { 0 } else { ptr(&mut r) as u64 };
        c.g[A1][0] = (seed % 2) as u64;
    }

    #[test]
    fn in_seq_dtor_matches() {
        check(ee::s07::f_002728d0__generated, in_seq_dtor, 60, &|c, seed| dtor_setup(c, seed, 0x2728d0));
    }

    #[test]
    fn bank_monitor_instance_dtor_matches() {
        check(ee::s08::f_002aba18__generated, bank_monitor_instance_dtor, 60, &|c, seed| {
            dtor_setup(c, seed, 0x2aba18)
        });
    }

    #[test]
    fn bank_monitor_timer_dtor_matches() {
        check(ee::s08::f_002ad860__generated, bank_monitor_timer_dtor, 60, &|c, seed| {
            dtor_setup(c, seed, 0x2ad860)
        });
    }

    #[test]
    fn world_trigger_instance_dtor_matches() {
        check(ee::s08::f_002b4be0__generated, world_trigger_instance_dtor, 60, &|c, seed| {
            dtor_setup(c, seed, 0x2b4be0)
        });
    }

    #[test]
    fn c_world_sound_dtor_matches() {
        check(ee::s08::f_002b6ae0__generated, c_world_sound_dtor, 60, &|c, seed| {
            dtor_setup(c, seed, 0x2b6ae0)
        });
    }

    #[test]
    fn menu_sys_dtor_matches() {
        check(ee::s09::f_002cc048__generated, menu_sys_dtor, 60, &|c, seed| dtor_setup(c, seed, 0x2cc048));
    }

    #[test]
    fn snow_box_dtor_matches() {
        check(ee::s09::f_002e4f28__generated, snow_box_dtor, 60, &|c, seed| dtor_setup(c, seed, 0x2e4f28));
    }

    // ----------------------------------------------- sequence / audio helpers

    #[test]
    fn in_seq_stop_matches() {
        check(ee::s07::f_00272b58__generated, in_seq_stop, 100, &|c, seed| {
            let _r = game(c, seed, 0x272b58, 2);
            let array = SCRATCH + 0x1000;
            c.g[A0][0] = SCRATCH as u64;
            c.g[A1][0] = (seed % 4) as u64;
            c.bus.write32(SCRATCH + 0x14, array);
            for i in 0..4u32 {
                // Half the instances exist, half have no instance at all.
                c.bus.write32(array + 0x58 * i + 0xc, ((seed + i) % 2) as u32);
                // `+0x20 == 0` stops the instance immediately.
                c.bus.write32(array + 0x58 * i + 0x28, 0);
            }
        });
    }

    #[test]
    fn script_engine_seq_listener_on_destroy_matches() {
        check(ee::s07::f_00282b88__generated, script_engine_seq_listener_on_destroy, 100, &|c, seed| {
            let mut r = game(c, seed, 0x282b88, 1);
            c.g[A0][0] = SCRATCH as u64;
            c.g[A1][0] = (SCRATCH + 0x2000) as u64;
            let ctx = ptr(&mut r);
            c.bus.write32(SCRATCH + 0x200c, ctx);
        });
    }

    #[test]
    fn speech_instance_stop_stream_matches() {
        check(ee::s08::f_002b0320__generated, speech_instance_stop_stream, 100, &|c, seed| {
            let _r = game(c, seed, 0x2b0320, 1);
            let instance = SCRATCH + 0x3000;
            let owner = SCRATCH + 0x4000;
            let array = SCRATCH + 0x5000;
            let elements = SCRATCH + 0x6000;
            c.g[A0][0] = instance as u64;
            c.bus.write32(difftest_gp() + 0x6c4, owner); // D_004A37B4
            c.bus.write32(owner, array);
            c.bus.write32(instance + 4, (seed % 8) as u32);
            c.bus.write32(array + 8, elements);
            // No element is live, so the array only reads them.
            for i in 0..8u32 {
                c.bus.write32(elements + 4 * i, 0);
            }
        });
    }

    #[test]
    fn c_world_sound_on_section_loaded_matches() {
        check(ee::s08::f_002b6b08__generated, c_world_sound_on_section_loaded, 80, &|c, seed| {
            let mut r = game(c, seed, 0x2b6b08, 1);
            c.g[A0][0] = SCRATCH as u64;
            // A null section skips the relocate call; a section whose name is
            // not empty makes the relocate return at once.
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { (SCRATCH + 0x800) as u64 };
            c.g[A2][0] = r.next_u32() as u64;
            if seed % 2 != 0 {
                c.bus.write8(SCRATCH + 0x800, 0xff);
            }
        });
    }

    #[test]
    fn c_ps2_mem_card_clear_dir_entry_matches() {
        check(ee::s08::f_002c52d8__generated, c_ps2_mem_card_clear_dir_entry, 60, &|c, seed| {
            let _r = game(c, seed, 0x2c52d8, 1);
            c.g[A0][0] = SCRATCH as u64;
        });
    }

    #[test]
    fn c_ps2_mem_card_v15_matches() {
        check(ee::s08::f_002c64a8__generated, c_ps2_mem_card_v15, 80, &|c, seed| {
            let mut r = game(c, seed, 0x2c64a8, 1);
            c.g[A2][0] = (SCRATCH + 0x900) as u64;
            c.g[A0][0] = ptr(&mut r) as u64;
            c.g[A1][0] = r.next_u32() as u64;
            // The ASCII source the second argument points at (D_004A3968).
            for (i, b) in b"memcard\0".iter().enumerate() {
                c.bus.write8(0x004a_3968 + i as u32, *b);
            }
        });
    }

    #[test]
    fn c_ps2_mem_card_v16_matches() {
        check(ee::s08::f_002c64d0__generated, c_ps2_mem_card_v16, 80, &|c, seed| {
            let mut r = game(c, seed, 0x2c64d0, 1);
            c.g[A2][0] = (SCRATCH + 0x900) as u64;
            c.g[A0][0] = ptr(&mut r) as u64;
            c.g[A1][0] = r.next_u32() as u64;
            for (i, b) in b"memcard\0".iter().enumerate() {
                c.bus.write8(0x004a_3968 + i as u32, *b);
            }
        });
    }

    #[test]
    fn c_anger_fx_reset_matches() {
        check(ee::s09::f_002d4be0__generated, c_anger_fx_reset, 60, &|c, seed| {
            let _r = game(c, seed, 0x2d4be0, 1);
            c.g[A0][0] = SCRATCH as u64;
            c.bus.write32(SCRATCH + 4, SCRATCH + 0xb00 + 0x80 * (seed % 4));
        });
    }

    #[test]
    fn world_painter_get_irradiance2_matches() {
        check(ee::s09::f_002ee368__generated, world_painter_get_irradiance2, 80, &|c, seed| {
            let _r = game(c, seed, 0x2ee368, 2);
            // A non-null painter singleton makes the name lookup return the
            // static name at `gp + 0x24c0`; the reference lookup then resolves
            // that name.
            c.bus.write32(difftest_gp() + 0xa74, SCRATCH);
            c.g[A0][0] = (seed * 4) as u64;
        });
    }

    #[test]
    fn dead_fade_kill_matches() {
        check(ee::s11::f_00350c90__generated, dead_fade_kill, 60, &|c, seed| {
            let mut r = game(c, seed, 0x350c90, 1);
            c.g[A0][0] = SCRATCH as u64;
            let world = SCRATCH + 0x2000;
            c.bus.write32(SCRATCH + 0x18, world);
            for i in 0..3u32 {
                c.bus.write32(SCRATCH + 0x1c + 4 * i, r.f32(20.0));
            }
            c.bus.write32(world + 0xc, 0);
        });
    }

    // -------------------------------------------------------- emitter / UI text

    #[test]
    fn emitter_set_desc_curve_a_matches() {
        check(ee::s12::f_0036ce00__generated, emitter_set_desc_curve_a, 200, &|c, seed| {
            let mut r = Rng(0x36ce_0000 ^ seed as u64);
            crate::difftest::scramble(c, &mut r);
            c.g[A0][0] = SCRATCH as u64;
            for f in 12..15 {
                c.f[f] = r.edge_f32();
            }
        });
    }

    #[test]
    fn ui_text_get_owned_string_matches() {
        check(ee::s13::f_003a0c00__generated, ui_text_get_owned_string, 120, &|c, seed| {
            let mut r = Rng(0x3a0c_0000 ^ seed as u64);
            crate::difftest::scramble(c, &mut r);
            c.g[A0][0] = SCRATCH as u64;
            c.bus.write32(SCRATCH + 0x74, if seed % 2 == 0 { 0 } else { 1 << 3 });
            c.bus.write32(SCRATCH + 0xb4, r.next_u32());
        });
    }

    #[test]
    fn ui_text_get_id_string_matches() {
        check(ee::s13::f_003a0c28__generated, ui_text_get_id_string, 120, &|c, seed| {
            let mut r = Rng(0x3a0c_0028 ^ seed as u64);
            crate::difftest::scramble(c, &mut r);
            c.g[A0][0] = SCRATCH as u64;
            c.bus.write32(SCRATCH + 0x74, if seed % 2 == 0 { 0 } else { 1 << 3 });
            c.bus.write32(SCRATCH + 0xb4, r.next_u32());
        });
    }

    // ------------------------------------------------------ option / reward

    #[test]
    fn be_option_int_get_max_handicap_matches() {
        check(ee::s01::f_0014f870__generated, be_option_int_get_max_handicap, 200, &|c, seed| {
            let mut r = Rng(0x14f8_7000 ^ seed as u64);
            crate::difftest::scramble(c, &mut r);
            // Three signed limits; keep the ordering varied so both `max`
            // steps take both outcomes.
            let vals = [
                (r.next_u32() % 7) as i32 - 3,
                (r.next_u32() % 7) as i32 - 3,
                (r.next_u32() % 7) as i32 - 3,
            ];
            for (i, off) in [0x1c, 0x8, 0x14].into_iter().enumerate() {
                c.bus.write32(0x0053_08b8 + off, vals[i] as u32);
            }
        });
    }

    #[test]
    fn be_option_int_get_computer_handicap_matches() {
        check(ee::s01::f_0014f7e8__generated, be_option_int_get_computer_handicap, 120, &|c, seed| {
            let mut r = Rng(0x14f7_e800 ^ seed as u64);
            crate::difftest::scramble(c, &mut r);
            for (i, off) in [0x1c, 0x8, 0x14, 0x1c].into_iter().enumerate() {
                c.bus.write32(0x0053_08b8 + off, (r.next_u32() % 9) as u32 + i as u32);
            }
            c.bus.write32(0x0053_08d4, r.next_u32() % 9);
        });
    }

    #[test]
    fn be_bagt_int_get_trick_table_entry_matches() {
        check(ee::s01::f_0014ff90__generated, be_bagt_int_get_trick_table_entry, 200, &|c, seed| {
            let mut r = Rng(0x14ff_9000 ^ seed as u64);
            crate::difftest::scramble(c, &mut r);
            let index = seed % 8;
            c.g[A2][0] = index as u64;
            c.g[A3][0] = (SCRATCH + 0x1200) as u64;
            let table = 0x0045_aeb8u32;
            for i in 0..8u32 {
                let e = table + 0x14 * i;
                c.bus.write16(e + 0xc, r.next_u32() as u16);
                c.bus.write32(e + 0x10, r.next_u32());
            }
        });
    }

    /// The reward record for `a1 = player`, `a2 = slot`.
    fn reward_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = Rng(salt ^ (seed as u64) << 8);
        crate::difftest::scramble(c, &mut r);
        let player = seed % 4;
        let slot = seed % 8;
        c.g[A1][0] = player as u64;
        c.g[A2][0] = slot as u64;
        c.g[A3][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        let rec = 0x004a_6ca8u32 + player * 0x9b50 + slot * 0xf88;
        c.bus.write32(rec + 0xf80, r.next_u32());
        c.bus.write64(rec + 0xf70, (r.next_u32() as u64) << 32 | r.next_u32() as u64);
        c.bus.write64(rec + 0xf78, (r.next_u32() as u64) << 32 | r.next_u32() as u64);
    }

    #[test]
    fn be_reward_int_get_audio_setting_matches() {
        check(ee::s01::f_00158700__generated, be_reward_int_get_audio_setting, 120, &|c, seed| {
            reward_setup(c, seed, 0x1587_0000)
        });
    }

    #[test]
    fn be_reward_int_get_unlocked_songs_matches() {
        check(ee::s01::f_00158728__generated, be_reward_int_get_unlocked_songs, 120, &|c, seed| {
            reward_setup(c, seed, 0x1587_0028)
        });
    }

    #[test]
    fn be_reward_int_get_enabled_songs_matches() {
        check(ee::s01::f_00158750__generated, be_reward_int_get_enabled_songs, 120, &|c, seed| {
            reward_setup(c, seed, 0x1587_0050)
        });
    }

    #[test]
    fn be_reward_int_set_audio_setting_matches() {
        check(ee::s01::f_001587f8__generated, be_reward_int_set_audio_setting, 120, &|c, seed| {
            reward_setup(c, seed, 0x1587_00f8)
        });
    }

    #[test]
    fn be_reward_int_set_unlocked_songs_matches() {
        check(ee::s01::f_00158820__generated, be_reward_int_set_unlocked_songs, 120, &|c, seed| {
            reward_setup(c, seed, 0x1587_0120)
        });
    }

    #[test]
    fn be_reward_int_set_enabled_songs_matches() {
        check(ee::s01::f_00158848__generated, be_reward_int_set_enabled_songs, 120, &|c, seed| {
            reward_setup(c, seed, 0x1587_0148)
        });
    }

    #[test]
    fn be_clear_option_state_5308b8_matches() {
        check(ee::s01::f_0015c228__generated, be_clear_option_state_5308b8, 60, &|c, seed| {
            let mut r = game(c, seed, 0x15c228, 1);
            for i in 0..8u32 {
                c.bus.write32(0x0053_08b8 + 4 * i, r.next_u32());
            }
        });
    }

    // ---------------------------------------------------------- camera vtable

    #[test]
    fn camera_algorithm_tick_matches() {
        check(ee::s02::f_00167e68__generated, camera_algorithm_tick, 100, &|c, seed| {
            let mut r = game(c, seed, 0x167e68, 1);
            let this = SCRATCH + 0x3000;
            c.g[A0][0] = this as u64;
            // `*(this + 0x10)` is the algorithm object; `+0x28` holds the
            // `this` delta the call adds and `+0x2c` the method pointer (a fake).
            let obj = fakes::ptr(&mut r);
            c.bus.write32(this + 0x10, obj);
            c.bus.write16(obj + 0x28, (r.next_u32() & 0xff8) as u16);
            c.bus.write32(obj + 0x2c, fakes::FAKE);
        });
    }

    /// The `gp` value the tests run with (`machine` sets it).
    fn difftest_gp() -> u32 {
        crate::difftest::GAME_GP
    }
}
