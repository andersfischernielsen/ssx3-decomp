//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for the front-end / memory-card / net / replay / audio hooks
//! in this wave, each with a differential test against the generated body.
//!
//! - **Twenty-three deleting destructors**: 8 `GMHandler*_v01`, the front-end
//!   stream/state destructors, the two `FENetHook`s, the eight `NetEvent*`
//!   events, and the two replay stream destructors. They republish a vtable
//!   word (at `+0`, `+4`, `+8` or `+0x14`) and delete the object when bit 0 of
//!   `flags` is set; all share [`deleting_dtor`].
//! - **Lobby roster accessors** (`LobbyMan_GetKickReason`, `GetRoomCount`,
//!   `IsRoomFlagged`, `GetRoomUserCount`): field reads plus the small
//!   `Lobby_List*` queries at 0x3ee550..0x3ee560.
//! - **Buddy accessors** (`BuddyMan_IsBuddy`, `IsBlocked`, `IsTemp`,
//!   `GetRosterCount`, `CountMessagesFrom`, `IsOnline`, `IsSameProduct`): look
//!   the name up in the roster (`BuddyMan_FindRosterEntry`) and forward the
//!   entry index to the matching `*At` predicate.
//! - **Script / sequence helpers**: `InSeq_Play`, `ScriptEngine_ListAdd`,
//!   `ScriptEngine_ListSkip`.
//! - **Audio hooks**: the three volume-level getters, the crowd-state
//!   callback, `SndBankMan_GetBankBuffer`, `SSXAudioSystem_TickListener`,
//!   `SSXAudioSystem_OnScriptRiderSpeech2`, `SSXAudioSystem_ResetRaceResultState`
//!   and the three `CrowdAudio_*` voice dispatchers.

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
    (0x0024_45c0, gm_handler8_v01, "GMHandler8_v01"),
    (0x0024_45f0, gm_handler9_v01, "GMHandler9_v01"),
    (0x0024_4630, gm_handler2_v01, "GMHandler2_v01"),
    (0x0024_4660, gm_handler4_v01, "GMHandler4_v01"),
    (0x0024_4690, gm_handler5_v01, "GMHandler5_v01"),
    (0x0024_46c0, gm_handler6_v01, "GMHandler6_v01"),
    (0x0024_46f0, gm_handler7_v01, "GMHandler7_v01"),
    (0x0024_4720, gm_handler_unused_v01, "GMHandlerUnused_v01"),
    (0x0024_5f00, generic_stream_pause_dtor, "GenericStreamPause_dtor"),
    (0x0025_5638, vd_sink_base_dtor, "VD_SinkBase_dtor"),
    (0x0025_5800, c_d_sock_channel_base_dtor, "cDSockChannel_BaseDtor"),
    (0x0025_da90, lobby_man_get_kick_reason, "LobbyMan_GetKickReason"),
    (0x0025_f120, lobby_man_get_room_count, "LobbyMan_GetRoomCount"),
    (0x0025_f180, lobby_man_is_room_flagged, "LobbyMan_IsRoomFlagged"),
    (0x0025_f1b0, lobby_man_get_room_user_count, "LobbyMan_GetRoomUserCount"),
    (0x0026_2360, buddy_man_is_buddy, "BuddyMan_IsBuddy"),
    (0x0026_23b0, buddy_man_is_blocked, "BuddyMan_IsBlocked"),
    (0x0026_2438, buddy_man_is_temp, "BuddyMan_IsTemp"),
    (0x0026_2598, buddy_man_get_roster_count, "BuddyMan_GetRosterCount"),
    (0x0026_2a08, buddy_man_count_messages_from, "BuddyMan_CountMessagesFrom"),
    (0x0026_33c8, buddy_man_is_online, "BuddyMan_IsOnline"),
    (0x0026_3490, buddy_man_is_same_product, "BuddyMan_IsSameProduct"),
    (0x0026_3678, fe_net_hook_dtor, "FENetHook_dtor"),
    (0x0026_39b0, fe_net_hook2_dtor, "FENetHook2_dtor"),
    (0x0026_5d68, c_shared_login_man_is_card_missing, "cSharedLoginMan_IsCardMissing"),
    (0x0026_7e18, net_event_dtor, "NetEvent_dtor"),
    (0x0026_7e98, net_event102_dtor, "NetEvent102_dtor"),
    (0x0026_7f28, net_event102_base_dtor, "NetEvent102Base_dtor"),
    (0x0026_7fb8, net_error_event_dtor, "NetErrorEvent_dtor"),
    (0x0026_8210, net_event102_tooy_dtor, "NetEvent102Tooy_dtor"),
    (0x0026_8658, net_event_fb_dtor, "NetEventFB_dtor"),
    (0x0026_86f0, net_event_fc_dtor, "NetEventFC_dtor"),
    (0x0026_8790, net_event10d_dtor, "NetEvent10D_dtor"),
    (0x0027_21d0, replay_stream_dtor, "ReplayStream_dtor"),
    (0x0027_2258, world_state_stream_dtor, "WorldStateStream_dtor"),
    (0x0027_2ad0, in_seq_play, "InSeq_Play"),
    (0x0027_8e20, script_engine_list_add, "ScriptEngine_ListAdd"),
    (0x0027_9070, script_engine_list_skip, "ScriptEngine_ListSkip"),
    (0x0028_7670, ssx_audio_system_get_music_volume_level, "SSXAudioSystem_GetMusicVolumeLevel"),
    (0x0028_76a0, ssx_audio_system_get_sfx_volume_level, "SSXAudioSystem_GetSfxVolumeLevel"),
    (0x0028_76d0, ssx_audio_system_get_speech_volume_level, "SSXAudioSystem_GetSpeechVolumeLevel"),
    (0x0028_9650, ssx_audio_system_crowd_state_callback, "SSXAudioSystem_CrowdStateCallback"),
    (0x0028_bee0, snd_bank_man_get_bank_buffer, "SndBankMan_GetBankBuffer"),
    (0x0029_4f48, ssx_audio_system_tick_listener, "SSXAudioSystem_TickListener"),
    (0x002a_1b58, ssx_audio_system_on_script_rider_speech2, "SSXAudioSystem_OnScriptRiderSpeech2"),
    (0x002a_4590, ssx_audio_system_reset_race_result_state, "SSXAudioSystem_ResetRaceResultState"),
    (0x002a_73a8, crowd_audio_play_trick, "CrowdAudio_PlayTrick"),
    (0x002a_73d8, crowd_audio_play_fall, "CrowdAudio_PlayFall"),
    (0x002a_7438, crowd_audio_stop_anticipate, "CrowdAudio_StopAnticipate"),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const LOBBY_LIST_COUNT: Callee = Callee {
        addr: 0x003e_e550,
        f: ee::s14::Lobby_ListGetCount,
    };
    pub const LOBBY_LIST_VISIBLE_COUNT: Callee = Callee {
        addr: 0x003e_e558,
        f: ee::s14::Lobby_ListGetVisibleCount,
    };
    pub const LOBBY_LIST_GET_AT: Callee = Callee {
        addr: 0x003e_e560,
        f: ee::s14::Lobby_ListGetAt,
    };

    pub const BUDDY_FIND_ROSTER_ENTRY: Callee = Callee {
        addr: 0x0026_2468,
        f: ee::s07::BuddyMan_FindRosterEntry,
    };
    pub const BUDDY_IS_BUDDY_AT: Callee = Callee {
        addr: 0x0026_2390,
        f: ee::s07::BuddyMan_IsBuddyAt,
    };
    pub const BUDDY_IS_BLOCKED_AT: Callee = Callee {
        addr: 0x0026_23e0,
        f: ee::s07::BuddyMan_IsBlockedAt,
    };
    pub const BUDDY_IS_TEMP_AT: Callee = Callee {
        addr: 0x0026_2400,
        f: ee::s07::BuddyMan_IsTempAt,
    };
    pub const BUDDY_IS_ONLINE_AT: Callee = Callee {
        addr: 0x0026_3338,
        f: ee::s07::BuddyMan_IsOnlineAt,
    };
    pub const BUDDY_IS_SAME_PRODUCT_AT: Callee = Callee {
        addr: 0x0026_33f8,
        f: ee::s07::BuddyMan_IsSameProductAt,
    };
    pub const BUDDY_COUNT_MESSAGES_FROM_AT: Callee = Callee {
        addr: 0x0026_2950,
        f: ee::s07::BuddyMan_CountMessagesFromAt,
    };

    pub const INSEQ_GET_INSTANCE: Callee = Callee {
        addr: 0x0027_2cc0,
        f: ee::s07::InSeq_GetInstance,
    };
    pub const INSEQ_INSTANCE_START: Callee = Callee {
        addr: 0x0027_4830,
        f: ee::s07::InSeq_InstanceStart,
    };
    pub const SCRIPT_LIST_ADD: Callee = Callee {
        addr: 0x0027_5dd8,
        f: ee::s07::ScriptList_Add,
    };
    pub const SCRIPT_LIST_SKIP: Callee = Callee {
        addr: 0x0027_66d0,
        f: ee::s07::ScriptList_Skip,
    };
    pub const SSX_AUDIO_GET_CHANNEL_VOLUME: Callee = Callee {
        addr: 0x0028_7920,
        f: ee::s07::SSXAudioSystem_GetChannelVolume,
    };
    pub const SSX_AUDIO_EMPTY_CROWD_HANDLER: Callee = Callee {
        addr: 0x0028_9680,
        f: ee::s07::SSXAudioSystem_EmptyCrowdHandler,
    };
    pub const SND_BANK_GET_BUFFER: Callee = Callee {
        addr: 0x0028_b8d8,
        f: ee::s07::SndBank_GetBuffer,
    };
    pub const SSX_AUDIO_RIDER_SAY_20B9: Callee = Callee {
        addr: 0x002a_1400,
        f: ee::s08::SSXAudioSystem_RiderSay20B9,
    };
    pub const CROWD_INSTANCE_PLAY_TRICK: Callee = Callee {
        addr: 0x002a_64a0,
        f: ee::s08::CrowdAudio_InstancePlayTrick,
    };
    pub const CROWD_INSTANCE_PLAY_FALL: Callee = Callee {
        addr: 0x002a_6648,
        f: ee::s08::CrowdAudio_InstancePlayFall,
    };
    pub const CROWD_INSTANCE_STOP_ANTICIPATE: Callee = Callee {
        addr: 0x002a_6808,
        f: ee::s08::CrowdAudio_InstanceStopAnticipate,
    };
}

/// `operator delete(void *p)` (`__builtin_delete`), reached through the
/// function table as the generated code does (`call_indirect`).
const OPERATOR_DELETE: u32 = 0x0031_7e50;

// ---------------------------------------------------------------------------
// Deleting destructors.

/// The shared body of the `(flags & 1)` deleting destructors: republish
/// `vtable` at `self + offset` and delete `self` when bit 0 of `flags` is set.
fn deleting_dtor(c: &mut Cpu, vtable: u32, offset: u32, ret: u32) -> Flow {
    c.cycles += 7;
    let frame = Frame::open(c, 0x10);
    let vptr = sx(vtable);
    c.g[V0][0] = vptr;
    frame.save_ra(c, 0);
    c.g[A1][0] &= 1;
    Addr((c.g[A0][0] as u32).wrapping_add(offset)).sw(c, vptr);
    if c.g[A1][0] != 0 {
        c.cycles += 2;
        call_ptr(c, OPERATOR_DELETE, ret)?;
    }
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void GMHandler8_v01(void *self, int flags)` (0x2445c0): deleting
/// destructor that republishes `D_0047D008` at `self+0`.
pub fn gm_handler8_v01(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0047_d008, 0, 0x0024_45e4))
}

/// `void GMHandler9_v01(void *self, int flags)` (0x2445f0): republishes
/// `D_0047D008` at `self+0` and deletes when asked.
pub fn gm_handler9_v01(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0047_d008, 0, 0x0024_4614))
}

/// `void GMHandler2_v01(void *self, int flags)` (0x244630): republishes
/// `D_0047D008` at `self+0` and deletes when asked.
pub fn gm_handler2_v01(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0047_d008, 0, 0x0024_4654))
}

/// `void GMHandler4_v01(void *self, int flags)` (0x244660): republishes
/// `D_0047D008` at `self+0` and deletes when asked.
pub fn gm_handler4_v01(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0047_d008, 0, 0x0024_4684))
}

/// `void GMHandler5_v01(void *self, int flags)` (0x244690): republishes
/// `D_0047D008` at `self+0` and deletes when asked.
pub fn gm_handler5_v01(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0047_d008, 0, 0x0024_46b4))
}

/// `void GMHandler6_v01(void *self, int flags)` (0x2446c0): republishes
/// `D_0047D008` at `self+0` and deletes when asked.
pub fn gm_handler6_v01(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0047_d008, 0, 0x0024_46e4))
}

/// `void GMHandler7_v01(void *self, int flags)` (0x2446f0): republishes
/// `D_0047D008` at `self+0` and deletes when asked.
pub fn gm_handler7_v01(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0047_d008, 0, 0x0024_4714))
}

/// `void GMHandlerUnused_v01(void *self, int flags)` (0x244720): republishes
/// `D_0047D008` at `self+0` and deletes when asked.
pub fn gm_handler_unused_v01(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0047_d008, 0, 0x0024_4744))
}

/// `void GenericStreamPause_dtor(GenericStreamPause *self, int flags)`
/// (0x245f00): republishes `D_0047DB28` at `self+0`, deletes on bit 0.
pub fn generic_stream_pause_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0047_db28, 0, 0x0024_5f24))
}

/// `void VD_SinkBase_dtor(VD_SinkBase *self, int flags)` (0x255638):
/// republishes `D_004802B0` at `self+4`, deletes on bit 0.
pub fn vd_sink_base_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_02b0, 4, 0x0025_565c))
}

/// `void cDSockChannel_BaseDtor(cDSockChannel *self, int flags)` (0x255800):
/// republishes `D_00481320` at `self+8`, deletes on bit 0.
pub fn c_d_sock_channel_base_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_1320, 8, 0x0025_5824))
}

/// `void FENetHook_dtor(FENetHook *self, int flags)` (0x263678): republishes
/// `D_004810C8` at `self+8`, deletes on bit 0.
pub fn fe_net_hook_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_10c8, 8, 0x0026_369c))
}

/// `void FENetHook2_dtor(FENetHook2 *self, int flags)` (0x2639b0):
/// republishes `D_004810B0` at `self+0x14`, deletes on bit 0.
pub fn fe_net_hook2_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_10b0, 0x14, 0x0026_39d4))
}

/// `void NetEvent_dtor(NetEvent *self, int flags)` (0x267e18): republishes
/// `D_004812B0` at `self+4`, deletes on bit 0.
pub fn net_event_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_12b0, 4, 0x0026_7e3c))
}

/// `void NetEvent102_dtor(NetEvent102 *self, int flags)` (0x267e98): the same
/// `D_004812B0` destructor.
pub fn net_event102_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_12b0, 4, 0x0026_7ebc))
}

/// `void NetEvent102Base_dtor(NetEvent102Base *self, int flags)` (0x267f28):
/// the same `D_004812B0` destructor.
pub fn net_event102_base_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_12b0, 4, 0x0026_7f4c))
}

/// `void NetErrorEvent_dtor(NetErrorEvent *self, int flags)` (0x267fb8): the
/// same `D_004812B0` destructor.
pub fn net_error_event_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_12b0, 4, 0x0026_7fdc))
}

/// `void NetEvent102Tooy_dtor(NetEvent102Tooy *self, int flags)` (0x268210):
/// the same `D_004812B0` destructor.
pub fn net_event102_tooy_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_12b0, 4, 0x0026_8234))
}

/// `void NetEventFB_dtor(NetEventFB *self, int flags)` (0x268658): the same
/// `D_004812B0` destructor.
pub fn net_event_fb_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_12b0, 4, 0x0026_867c))
}

/// `void NetEventFC_dtor(NetEventFC *self, int flags)` (0x2686f0): the same
/// `D_004812B0` destructor.
pub fn net_event_fc_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_12b0, 4, 0x0026_8714))
}

/// `void NetEvent10D_dtor(NetEvent10D *self, int flags)` (0x268790): the same
/// `D_004812B0` destructor.
pub fn net_event10d_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_12b0, 4, 0x0026_87b4))
}

/// `void ReplayStream_dtor(ReplayStream *self, int flags)` (0x2721d0):
/// republishes `D_00481898` at `self+0`, deletes on bit 0.
pub fn replay_stream_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_1898, 0, 0x0027_21f4))
}

/// `void WorldStateStream_dtor(WorldStateStream *self, int flags)`
/// (0x272258): republishes `D_00481898` at `self+0`, deletes on bit 0.
pub fn world_state_stream_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_1898, 0, 0x0027_227c))
}

// ---------------------------------------------------------------------------
// Lobby accessors.

/// `char *LobbyMan_GetKickReason(LobbyMan *self)` (0x25da90): the kick reason
/// string at `self+0x84`, but only when the session is live (`self+0x54`) and
/// the string is not empty.
pub fn lobby_man_get_kick_reason(c: &mut Cpu) {
    /// Non-zero while the lobby session is connected.
    const SESSION: u32 = 0x54;
    /// `char *` to the server's last kick message.
    const REASON: u32 = 0x84;

    c.cycles += 3;
    let this = abi::arg(c, 0);
    c.g[V0][0] = Addr(this).off(SESSION).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = Addr(this).off(REASON).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lb(c);
            if c.g[V1][0] == 0 {
                c.g[V0][0] = 0;
            }
        } else {
            c.g[V0][0] = 0;
        }
        c.cycles += 2;
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

/// `int LobbyMan_GetRoomCount(LobbyMan *self)` (0x25f120): the visible room
/// count of the room list at `self+0x5c`, or 0 when the lobby is not up.
pub fn lobby_man_get_room_count(c: &mut Cpu) {
    guest(c, |c| {

    /// `Lobby_List *` of rooms.
    const ROOMS: u32 = 0x5c;

    c.cycles += 5;
    let this = abi::arg(c, 0);
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[V0][0] = Addr(this).lw(c);
    let live = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if live {
        c.cycles += 4;
        c.g[A0][0] = Addr(this).off(ROOMS).lw(c);
        callee::LOBBY_LIST_VISIBLE_COUNT.call(c, 0x0025_f13c)?;
    } else {
        c.cycles += 1;
    }
    frame.restore_ra(c, 0);
    c.cycles += 2;
    frame.close(c);

        Ok(())
    });
}

/// `int LobbyMan_IsRoomFlagged(LobbyMan *self, int index)` (0x25f180): bit 0
/// of the `int16` at `+0xa` of room `index` in the list at `self+0x5c`.
pub fn lobby_man_is_room_flagged(c: &mut Cpu) {
    guest(c, |c| {

    /// `Lobby_List *` of rooms.
    const ROOMS: u32 = 0x5c;

    c.cycles += 5;
    let this = abi::arg(c, 0);
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[V0][0] = Addr(this).lw(c);
    if c.g[V0][0] == 0 {
        c.g[V0][0] = 0;
    } else {
        c.cycles += 4;
        c.g[A0][0] = Addr(this).off(ROOMS).lw(c);
        callee::LOBBY_LIST_GET_AT.call(c, 0x0025_f19c)?;
        c.g[V0][0] = Addr(c.g[V0][0] as u32).off(0xa).lh(c);
        c.g[V0][0] &= 1;
    }
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);

        Ok(())
    });
}

/// `int LobbyMan_GetRoomUserCount(LobbyMan *self)` (0x25f1b0): the visible
/// user count of the list at `self+0x60`, or 0 when the lobby is not up.
pub fn lobby_man_get_room_user_count(c: &mut Cpu) {
    guest(c, |c| {

    /// `Lobby_List *` of the room's users.
    const USERS: u32 = 0x60;

    c.cycles += 5;
    let this = abi::arg(c, 0);
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[V0][0] = Addr(this).lw(c);
    let live = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if live {
        c.cycles += 4;
        c.g[A0][0] = Addr(this).off(USERS).lw(c);
        callee::LOBBY_LIST_VISIBLE_COUNT.call(c, 0x0025_f1cc)?;
    } else {
        c.cycles += 1;
    }
    frame.restore_ra(c, 0);
    c.cycles += 2;
    frame.close(c);

        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Buddy accessors.

/// `Lobby_List *` holding the buddy roster (`self + 0xcc`).
const BUDDY_ROSTER: u32 = 0xcc;

/// The shared body of the seven `BuddyMan_*` accessors: find the roster entry
/// for the name in `a1` and forward its index to `query(self, index)`.
fn roster_lookup(c: &mut Cpu, query: Callee, ret: u32) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    callee::BUDDY_FIND_ROSTER_ENTRY.call(c, ret)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[V0][0];
    query.call(c, ret + 0xc)?;
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `bool BuddyMan_IsBuddy(BuddyMan *self, char *name)` (0x262360).
pub fn buddy_man_is_buddy(c: &mut Cpu) {
    guest(c, |c| roster_lookup(c, callee::BUDDY_IS_BUDDY_AT, 0x0026_2374))
}

/// `bool BuddyMan_IsBlocked(BuddyMan *self, char *name)` (0x2623b0).
pub fn buddy_man_is_blocked(c: &mut Cpu) {
    guest(c, |c| roster_lookup(c, callee::BUDDY_IS_BLOCKED_AT, 0x0026_23c4))
}

/// `bool BuddyMan_IsTemp(BuddyMan *self, char *name)` (0x262438).
pub fn buddy_man_is_temp(c: &mut Cpu) {
    guest(c, |c| roster_lookup(c, callee::BUDDY_IS_TEMP_AT, 0x0026_244c))
}

/// `int BuddyMan_CountMessagesFrom(BuddyMan *self, char *name)` (0x262a08).
pub fn buddy_man_count_messages_from(c: &mut Cpu) {
    guest(c, |c| roster_lookup(c, callee::BUDDY_COUNT_MESSAGES_FROM_AT, 0x0026_2a1c))
}

/// `bool BuddyMan_IsOnline(BuddyMan *self, char *name)` (0x2633c8).
pub fn buddy_man_is_online(c: &mut Cpu) {
    guest(c, |c| roster_lookup(c, callee::BUDDY_IS_ONLINE_AT, 0x0026_33dc))
}

/// `bool BuddyMan_IsSameProduct(BuddyMan *self, char *name)` (0x263490).
pub fn buddy_man_is_same_product(c: &mut Cpu) {
    guest(c, |c| roster_lookup(c, callee::BUDDY_IS_SAME_PRODUCT_AT, 0x0026_34a4))
}

/// `int BuddyMan_GetRosterCount(BuddyMan *self)` (0x262598): the counted
/// entries of the roster list at `self+0xcc`, or 0 when there is no list.
pub fn buddy_man_get_roster_count(c: &mut Cpu) {
    guest(c, |c| {

    c.cycles += 5;
    let this = abi::arg(c, 0);
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[A0][0] = Addr(this).off(BUDDY_ROSTER).lw(c);
    let has_list = c.g[A0][0] != 0;
    c.g[V0][0] = 0;
    if has_list {
        c.cycles += 4;
        callee::LOBBY_LIST_COUNT.call(c, 0x0026_25b4)?;
    } else {
        c.cycles += 1;
    }
    frame.restore_ra(c, 0);
    c.cycles += 2;
    frame.close(c);

        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Login manager.

/// `bool cSharedLoginMan_IsCardMissing(void)` (0x265d68): negate the login
/// manager's virtual `+0x1a8/0x1ac` slot ("is the memory card present").
pub fn c_shared_login_man_is_card_missing(c: &mut Cpu) {
    /// `gp + 0x2e8`: the shared login manager singleton.
    const LOGIN_MANAGER: u32 = 0x2e8;

    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_MANAGER)).lw(c);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        let slot = Addr(c.g[V1][0] as u32).off(0x1a8);
        c.g[A0][0] = slot.lh(c);
        c.g[V0][0] = slot.off(4).lw(c);
        let method = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, method, 0x0026_5d88)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] ^= 1;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script / sequence.

/// `void InSeq_Play(Channel *ch, int index, void *owner)` (0x272ad0): fetch
/// the instrument-sequence instance for the slot and start it with `a2`.
pub fn in_seq_play(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        callee::INSEQ_GET_INSTANCE.call(c, 0x0027_2ae4)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        callee::INSEQ_INSTANCE_START.call(c, 0x0027_2af0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptEngine_ListAdd(ScriptEngine *self, int index, void *cmd,
/// void *arg)` (0x278e20): append a command to list `index` (each list is
/// 0xcc bytes inside the array at `self+0x54c`).
pub fn script_engine_list_add(c: &mut Cpu) {
    /// The array of per-slot script lists.
    const LISTS: u32 = 0x54c;
    /// Size of one `ScriptList`.
    const LIST_SIZE: u32 = 0xcc;

    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = LIST_SIZE as u64;
        frame.save_ra(c, 0);
        c.mult(0, V1, A1, V0);
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = Addr(c.g[A0][0] as u32).off(LISTS).lw(c);
        c.g[A2][0] = c.g[A3][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        callee::SCRIPT_LIST_ADD.call(c, 0x0027_8e44)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptEngine_ListSkip(ScriptEngine *self, int index, void *arg)`
/// (0x279070): advance list `index` past `arg` entries (same list array as
/// [`script_engine_list_add`]).
pub fn script_engine_list_skip(c: &mut Cpu) {
    /// The array of per-slot script lists.
    const LISTS: u32 = 0x54c;
    /// Size of one `ScriptList`.
    const LIST_SIZE: u32 = 0xcc;

    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = LIST_SIZE as u64;
        frame.save_ra(c, 0);
        c.mult(0, V1, A1, V0);
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = Addr(c.g[A0][0] as u32).off(LISTS).lw(c);
        c.g[A2][0] = c.g[A3][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        callee::SCRIPT_LIST_SKIP.call(c, 0x0027_9094)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Audio.

/// The shared body of the three volume-level getters: read the channel's
/// volume and scale the 0..1 value to the 0..11 menu level.
fn volume_level(c: &mut Cpu, channel: u32, ret: u32) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[A1][0] = channel as u64;
    callee::SSX_AUDIO_GET_CHANNEL_VOLUME.call(c, ret)?;
    // f1 = 11.0, then the usual `(int)(volume * 11.0f)`.
    c.g[AT][0] = 0x4130_0000; // 11.0
    c.mtc1(AT, 1);
    frame.restore_ra(c, 0);
    let level = FpuF32(c.f[0]) * FpuF32(c.f[1]);
    c.f[0] = level.0;
    c.f[1] = level.to_i32() as u32;
    c.mfc1(V0, 1);
    frame.close(c);
    Ok(())
}

/// `int SSXAudioSystem_GetMusicVolumeLevel(SSXAudioSystem *self)` (0x287670).
pub fn ssx_audio_system_get_music_volume_level(c: &mut Cpu) {
    guest(c, |c| volume_level(c, 1, 0x0028_7680))
}

/// `int SSXAudioSystem_GetSfxVolumeLevel(SSXAudioSystem *self)` (0x2876a0).
pub fn ssx_audio_system_get_sfx_volume_level(c: &mut Cpu) {
    guest(c, |c| volume_level(c, 5, 0x0028_76b0))
}

/// `int SSXAudioSystem_GetSpeechVolumeLevel(SSXAudioSystem *self)` (0x2876d0).
pub fn ssx_audio_system_get_speech_volume_level(c: &mut Cpu) {
    guest(c, |c| volume_level(c, 4, 0x0028_76e0))
}

/// `void SSXAudioSystem_CrowdStateCallback(SSXAudioSystem *self, int *state)`
/// (0x289650): when the incoming state is 3 (`idle`) reset the crowd handler.
pub fn ssx_audio_system_crowd_state_callback(c: &mut Cpu) {
    /// `gp + 0x410`: the crowd audio singleton.
    const CROWD: u32 = 0x410;

    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = 3;
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
        let idle = c.g[V0][0] == c.g[V1][0];
        frame.restore_ra(c, 0);
        if idle {
            c.cycles += 3;
            c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(CROWD)).lw(c);
            callee::SSX_AUDIO_EMPTY_CROWD_HANDLER.call(c, 0x0028_9674)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void *SndBankMan_GetBankBuffer(SndBankMan *self, int index)` (0x28bee0):
/// the memory buffer of bank `index` (each `SndBank` is 0x60 bytes in the
/// array at `self+0xacc`).
pub fn snd_bank_man_get_bank_buffer(c: &mut Cpu) {
    /// The array of `SndBank`s.
    const BANKS: u32 = 0xacc;
    /// Size of one `SndBank`.
    const BANK_SIZE: u32 = 0x60;

    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = BANK_SIZE as u64;
        frame.save_ra(c, 0);
        c.mult(0, V1, A1, V0);
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = Addr(c.g[A0][0] as u32).off(BANKS).lw(c);
        c.g[A2][0] = c.g[A3][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        callee::SND_BANK_GET_BUFFER.call(c, 0x0028_bf04)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_TickListener(SSXAudioSystem *self)` (0x294f48): call
/// the listener object's virtual tick (`vtable+0xc`) with no arguments.
pub fn ssx_audio_system_tick_listener(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0;
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0xc).lw(c);
        c.g[A3][0] = Addr(c.g[V1][0] as u32).off(8).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xc).lw(c);
        let method = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        call_ptr(c, method, 0x0029_4f6c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SSXAudioSystem_OnScriptRiderSpeech2(SSXAudioSystem *self, int msg,
/// void *arg)` (0x2a1b58): a rider speech request (`msg == 2`) is forwarded
/// and acknowledged; anything else is ignored.
pub fn ssx_audio_system_on_script_rider_speech2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 2;
        let speech = c.g[A1][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        if speech {
            c.cycles += 3;
            c.g[A1][0] = c.g[A2][0];
            callee::SSX_AUDIO_RIDER_SAY_20B9.call(c, 0x002a_1b78)?;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_ResetRaceResultState(SSXAudioSystem *self, int flag)`
/// (0x2a4590): clear the race-result announcement state; the two "no result
/// yet" cells (`+0x5800`, `+0x5810`, and `+0x580c` when `flag`) become -1 and
/// the queue position (`+0x5814`) 0x17.
pub fn ssx_audio_system_reset_race_result_state(c: &mut Cpu) {
    c.cycles += 8;
    let this = abi::arg(c, 0);
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    let base = Addr(this);
    base.off(0x57f8).sw(c, 0);
    base.off(0x57fc).sw(c, 0);
    base.off(0x5804).sw(c, 0);
    base.off(0x5808).sw(c, 0);
    base.off(0x5800).sw(c, c.g[V0][0]);
    let flag = c.g[A1][0] == 0;
    base.off(0x5810).sw(c, c.g[V0][0]);
    if !flag {
        c.cycles += 1;
        base.off(0x580c).sw(c, c.g[V0][0]);
    }
    c.cycles += 3;
    c.g[V0][0] = 0x17;
    base.off(0x5814).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// The shared body of the three crowd-voice dispatchers: pick the voice at
/// `index * 0x190` in the array at `self+0x18` and forward to `play(voice,
/// arg)`; do nothing while no crowd audio object is loaded.
fn crowd_voice(c: &mut Cpu, index_reg: usize, play: Callee, ret: u32) -> Flow {
    /// The pool of 0x190-byte crowd voices.
    const VOICES: u32 = 0x18;
    /// Size of one `sSndVoice190`.
    const VOICE_SIZE: u32 = 0x190;

    c.cycles += 5;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(VOICES).lw(c);
    let no_voices = c.g[V0][0] == 0;
    frame.restore_ra(c, 0);
    if !no_voices {
        c.cycles += 5;
        c.g[A0][0] = VOICE_SIZE as u64;
        c.mult(0, V1, index_reg, A0);
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        play.call(c, ret)?;
        frame.restore_ra(c, 0);
    }
    c.cycles += 2;
    frame.close(c);
    Ok(())
}

/// `void CrowdAudio_PlayTrick(CrowdAudio *self, int arg, int voice)`
/// (0x2a73a8).
pub fn crowd_audio_play_trick(c: &mut Cpu) {
    guest(c, |c| {
        crowd_voice(c, A2, callee::CROWD_INSTANCE_PLAY_TRICK, 0x002a_73cc)
    })
}

/// `void CrowdAudio_PlayFall(CrowdAudio *self, int arg, int voice)`
/// (0x2a73d8).
pub fn crowd_audio_play_fall(c: &mut Cpu) {
    guest(c, |c| {
        crowd_voice(c, A2, callee::CROWD_INSTANCE_PLAY_FALL, 0x002a_73fc)
    })
}

/// `void CrowdAudio_StopAnticipate(CrowdAudio *self, int voice)` (0x2a7438):
/// the voice index is in `a1` here, not `a2`.
pub fn crowd_audio_stop_anticipate(c: &mut Cpu) {
    guest(c, |c| {
        crowd_voice(c, A1, callee::CROWD_INSTANCE_STOP_ANTICIPATE, 0x002a_745c)
    })
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, Rng, DATA};

    /// Scratch area for the object fixtures (the tests own the data area).
    const SCRATCH: u32 = DATA + 0x1000;

    // ------------------------------------------------------------ destructors

    /// The deleting-destructor fixture: `this` is a data pointer (the
    /// allocator tolerates it, as in its own test) and `flags % 3` runs both
    /// the plain and the deleting path.
    fn dtor_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 6);
        c.g[A0][0] = if seed % 8 == 0 {
            0
        } else {
            (DATA + 0x1000 + (r.next_u32() & 0x7_fff0)) as u64
        };
        c.g[A1][0] = (seed % 3) as u64;
    }

    #[test]
    fn gm_handler8_v01_matches() {
        check(ee::s06::f_002445c0__generated, gm_handler8_v01, 60, &|c, seed| {
            dtor_setup(c, seed, 0x2445c0)
        });
    }

    #[test]
    fn gm_handler9_v01_matches() {
        check(ee::s06::f_002445f0__generated, gm_handler9_v01, 60, &|c, seed| {
            dtor_setup(c, seed, 0x2445f0)
        });
    }

    #[test]
    fn gm_handler2_v01_matches() {
        check(ee::s06::f_00244630__generated, gm_handler2_v01, 60, &|c, seed| {
            dtor_setup(c, seed, 0x244630)
        });
    }

    #[test]
    fn gm_handler4_v01_matches() {
        check(ee::s06::f_00244660__generated, gm_handler4_v01, 60, &|c, seed| {
            dtor_setup(c, seed, 0x244660)
        });
    }

    #[test]
    fn gm_handler5_v01_matches() {
        check(ee::s06::f_00244690__generated, gm_handler5_v01, 60, &|c, seed| {
            dtor_setup(c, seed, 0x244690)
        });
    }

    #[test]
    fn gm_handler6_v01_matches() {
        check(ee::s06::f_002446c0__generated, gm_handler6_v01, 60, &|c, seed| {
            dtor_setup(c, seed, 0x2446c0)
        });
    }

    #[test]
    fn gm_handler7_v01_matches() {
        check(ee::s06::f_002446f0__generated, gm_handler7_v01, 60, &|c, seed| {
            dtor_setup(c, seed, 0x2446f0)
        });
    }

    #[test]
    fn gm_handler_unused_v01_matches() {
        check(ee::s06::f_00244720__generated, gm_handler_unused_v01, 60, &|c, seed| {
            dtor_setup(c, seed, 0x244720)
        });
    }

    #[test]
    fn generic_stream_pause_dtor_matches() {
        check(ee::s06::f_00245f00__generated, generic_stream_pause_dtor, 60, &|c, seed| {
            dtor_setup(c, seed, 0x245f00)
        });
    }

    #[test]
    fn vd_sink_base_dtor_matches() {
        check(ee::s06::f_00255638__generated, vd_sink_base_dtor, 60, &|c, seed| {
            dtor_setup(c, seed, 0x255638)
        });
    }

    #[test]
    fn c_d_sock_channel_base_dtor_matches() {
        check(ee::s06::f_00255800__generated, c_d_sock_channel_base_dtor, 60, &|c, seed| {
            dtor_setup(c, seed, 0x255800)
        });
    }

    #[test]
    fn fe_net_hook_dtor_matches() {
        check(ee::s07::f_00263678__generated, fe_net_hook_dtor, 60, &|c, seed| {
            dtor_setup(c, seed, 0x263678)
        });
    }

    #[test]
    fn fe_net_hook2_dtor_matches() {
        check(ee::s07::f_002639b0__generated, fe_net_hook2_dtor, 60, &|c, seed| {
            dtor_setup(c, seed, 0x2639b0)
        });
    }

    #[test]
    fn net_event_dtor_matches() {
        check(ee::s07::f_00267e18__generated, net_event_dtor, 60, &|c, seed| {
            dtor_setup(c, seed, 0x267e18)
        });
    }

    #[test]
    fn net_event102_dtor_matches() {
        check(ee::s07::f_00267e98__generated, net_event102_dtor, 60, &|c, seed| {
            dtor_setup(c, seed, 0x267e98)
        });
    }

    #[test]
    fn net_event102_base_dtor_matches() {
        check(ee::s07::f_00267f28__generated, net_event102_base_dtor, 60, &|c, seed| {
            dtor_setup(c, seed, 0x267f28)
        });
    }

    #[test]
    fn net_error_event_dtor_matches() {
        check(ee::s07::f_00267fb8__generated, net_error_event_dtor, 60, &|c, seed| {
            dtor_setup(c, seed, 0x267fb8)
        });
    }

    #[test]
    fn net_event102_tooy_dtor_matches() {
        check(ee::s07::f_00268210__generated, net_event102_tooy_dtor, 60, &|c, seed| {
            dtor_setup(c, seed, 0x268210)
        });
    }

    #[test]
    fn net_event_fb_dtor_matches() {
        check(ee::s07::f_00268658__generated, net_event_fb_dtor, 60, &|c, seed| {
            dtor_setup(c, seed, 0x268658)
        });
    }

    #[test]
    fn net_event_fc_dtor_matches() {
        check(ee::s07::f_002686f0__generated, net_event_fc_dtor, 60, &|c, seed| {
            dtor_setup(c, seed, 0x2686f0)
        });
    }

    #[test]
    fn net_event10d_dtor_matches() {
        check(ee::s07::f_00268790__generated, net_event10d_dtor, 60, &|c, seed| {
            dtor_setup(c, seed, 0x268790)
        });
    }

    #[test]
    fn replay_stream_dtor_matches() {
        check(ee::s07::f_002721d0__generated, replay_stream_dtor, 60, &|c, seed| {
            dtor_setup(c, seed, 0x2721d0)
        });
    }

    #[test]
    fn world_state_stream_dtor_matches() {
        check(ee::s07::f_00272258__generated, world_state_stream_dtor, 60, &|c, seed| {
            dtor_setup(c, seed, 0x272258)
        });
    }

    // ---------------------------------------------------------------- lobby

    /// A lobby object at `SCRATCH`: the session word at `+0`, a `Lobby_List`
    /// at `+0x1000` with `count` entries whose objects carry random flags.
    fn lobby_object(c: &mut Cpu, seed: u32, salt: u64, list_offsets: &[u32]) {
        let mut r = machine(c, seed, salt, 1);
        c.g[A0][0] = SCRATCH as u64;
        // Half the seeds see a live session, half a dead one.
        c.bus.write32(SCRATCH, (seed % 2 == 0) as u32);
        // Room index for `Lobby_ListGetAt`.
        c.g[A1][0] = (seed % 4) as u64;
        let list = SCRATCH + 0x1000;
        for &off in list_offsets {
            c.bus.write32(SCRATCH + off, list);
        }
        c.bus.write32(list + 0x14, r.next_u32() % 32);
        c.bus.write32(list + 0x18, 4);
        c.bus.write32(list + 0x40, SCRATCH + 0x1800);
        for i in 0..4u32 {
            let room = SCRATCH + 0x2000 + 0x100 * i;
            c.bus.write32(SCRATCH + 0x1800 + 4 * i, room);
            c.bus.write16(room + 0xa, r.next_u32() as u16);
            c.bus.write32(room, r.next_u32());
        }
    }

    #[test]
    fn lobby_man_get_kick_reason_matches() {
        check(ee::s06::f_0025da90__generated, lobby_man_get_kick_reason, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x25da90, 1);
            c.g[A0][0] = SCRATCH as u64;
            c.bus.write32(SCRATCH + 0x54, (seed % 3) as u32);
            let reason = SCRATCH + 0x200;
            match seed % 4 {
                0 => c.bus.write32(SCRATCH + 0x84, 0),
                1 => {
                    c.bus.write32(SCRATCH + 0x84, reason);
                    c.bus.write8(reason, 0);
                }
                _ => {
                    c.bus.write32(SCRATCH + 0x84, reason);
                    c.bus.write8(reason, (r.next_u32() | 1) as u8);
                }
            }
        });
    }

    #[test]
    fn lobby_man_get_room_count_matches() {
        check(ee::s06::f_0025f120__generated, lobby_man_get_room_count, 120, &|c, seed| {
            lobby_object(c, seed, 0x25f120, &[0x5c]);
        });
    }

    #[test]
    fn lobby_man_is_room_flagged_matches() {
        check(ee::s06::f_0025f180__generated, lobby_man_is_room_flagged, 120, &|c, seed| {
            lobby_object(c, seed, 0x25f180, &[0x5c]);
        });
    }

    #[test]
    fn lobby_man_get_room_user_count_matches() {
        check(ee::s06::f_0025f1b0__generated, lobby_man_get_room_user_count, 120, &|c, seed| {
            lobby_object(c, seed, 0x25f1b0, &[0x60]);
        });
    }

    // ---------------------------------------------------------------- buddy

    /// A `BuddyMan` at `SCRATCH` with a two-entry roster at `+0xcc`; the name
    /// `bob` in `a1` matches entry 1 only for `seed % 4 == 1`. Variants: no
    /// roster (3), empty roster (0), match (1) and no match (2).
    fn buddy_object(c: &mut Cpu, seed: u32, salt: u64) {
        let this = SCRATCH;
        let mut r = machine(c, seed, salt, 8);
        c.g[A0][0] = this as u64;
        let name = SCRATCH + 0x4000;
        c.g[A1][0] = name as u64;
        let variant = seed % 4;
        for (i, byte) in b"bob\0".iter().enumerate() {
            c.bus.write8(name + i as u32, *byte);
        }
        for i in 0..2u32 {
            let entry = SCRATCH + 0x2000 + 0x400 * i;
            let bytes: &[u8; 4] = if variant == 1 && i == 1 {
                b"bob\0"
            } else if i == 0 {
                b"amy\0"
            } else {
                b"zoe\0"
            };
            for (j, byte) in bytes.iter().enumerate() {
                c.bus.write8(entry + j as u32, *byte);
            }
            // The product tag read by `BuddyMan_IsSameProductAt` (empty) and
            // the buddy flags at +0xa8 / the online byte at +0xa7.
            c.bus.write8(entry + 0x54, 0);
            c.bus.write32(entry + 0xa8, r.next_u32());
            c.bus.write8(entry + 0xa7, (r.next_u32() & 1) as u8);
            c.bus.write32(SCRATCH + 0x1800 + 4 * i, entry);
        }
        // The message window `[+0xac, +0xb0)` is empty and the product
        // string is a one-character name.
        c.bus.write32(this + 0xac, 0);
        c.bus.write32(this + 0xb0, 0);
        let product = SCRATCH + 0x6000;
        c.bus.write8(product, b'x');
        c.bus.write8(product + 1, 0);
        c.bus.write32(this + 0xec, product);
        if variant == 3 {
            c.bus.write32(this + BUDDY_ROSTER, 0);
        } else {
            let list = SCRATCH + 0x1000;
            c.bus.write32(this + BUDDY_ROSTER, list);
            c.bus.write32(list + 0x18, if variant == 0 { 0 } else { 2 });
            c.bus.write32(list + 0x40, SCRATCH + 0x1800);
        }
    }

    #[test]
    fn buddy_man_is_buddy_matches() {
        check(ee::s07::f_00262360__generated, buddy_man_is_buddy, 100, &|c, seed| {
            buddy_object(c, seed, 0x262360)
        });
    }

    #[test]
    fn buddy_man_is_blocked_matches() {
        check(ee::s07::f_002623b0__generated, buddy_man_is_blocked, 100, &|c, seed| {
            buddy_object(c, seed, 0x2623b0)
        });
    }

    #[test]
    fn buddy_man_is_temp_matches() {
        check(ee::s07::f_00262438__generated, buddy_man_is_temp, 100, &|c, seed| {
            buddy_object(c, seed, 0x262438)
        });
    }

    #[test]
    fn buddy_man_get_roster_count_matches() {
        check(ee::s07::f_00262598__generated, buddy_man_get_roster_count, 100, &|c, seed| {
            buddy_object(c, seed, 0x262598)
        });
    }

    #[test]
    fn buddy_man_count_messages_from_matches() {
        check(ee::s07::f_00262a08__generated, buddy_man_count_messages_from, 100, &|c, seed| {
            buddy_object(c, seed, 0x262a08)
        });
    }

    #[test]
    fn buddy_man_is_online_matches() {
        check(ee::s07::f_002633c8__generated, buddy_man_is_online, 100, &|c, seed| {
            buddy_object(c, seed, 0x2633c8)
        });
    }

    #[test]
    fn buddy_man_is_same_product_matches() {
        check(ee::s07::f_00263490__generated, buddy_man_is_same_product, 100, &|c, seed| {
            buddy_object(c, seed, 0x263490)
        });
    }

    // ---------------------------------------------------------- login manager

    #[test]
    fn c_shared_login_man_is_card_missing_matches() {
        check(ee::s07::f_00265d68__generated, c_shared_login_man_is_card_missing, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x265d68, 1);
            let manager = SCRATCH;
            let vt = SCRATCH + 0x200;
            c.bus.write32(crate::difftest::GAME_GP + 0x2e8, manager); // D_004A33D8
            c.bus.write32(manager, vt);
            // The `+0x1a8` slot: a signed this-delta and the method pointer,
            // which lands in the fake-call hook.
            c.bus.write16(vt + 0x1a8, (r.next_u32() & 0x7f0) as u16);
            c.bus.write32(vt + 0x1ac, fakes::FAKE);
        });
    }

    // ------------------------------------------------------- script / sequence

    /// An instrument-sequence channel at `SCRATCH` whose `+0x14` table has one
    /// live record (so `InSeq_GetInstance` returns `record + 8`).
    fn in_seq_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let _r = machine(c, seed, salt, 2);
        let table = SCRATCH + 0x1000;
        let record = table + (seed % 4) as u32 * 0x58;
        c.g[A0][0] = SCRATCH as u64;
        c.g[A1][0] = (seed % 4) as u64;
        c.g[A2][0] = if seed % 2 == 0 { 0 } else { (SCRATCH + 0x800) as u64 };
        c.bus.write32(SCRATCH + 0x14, table);
        c.bus.write32(record + 0xc, 1);
    }

    #[test]
    fn in_seq_play_matches() {
        check(ee::s07::f_00272ad0__generated, in_seq_play, 100, &|c, seed| {
            in_seq_setup(c, seed, 0x272ad0)
        });
    }

    /// A script engine at `SCRATCH` with the per-slot list array at `+0x54c`;
    /// the list's command queue is marked full (`+0xa0`) so `ScriptList_Add`
    /// returns without touching the queue, and the skip count (`a2`) is zero
    /// so `ScriptList_Skip` returns at once.
    fn script_engine_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 2);
        let lists = SCRATCH + 0x1000;
        let index = seed % 4;
        let list = lists + 0xcc * index;
        c.g[A0][0] = SCRATCH as u64;
        c.g[A1][0] = index as u64;
        c.g[A2][0] = r.next_u32() as u64;
        c.g[A3][0] = (SCRATCH + 0x2000) as u64;
        c.bus.write32(SCRATCH + 0x54c, lists);
        c.bus.write32(list + 0xa0, 1); // ScriptCmdQueue_IsFull
        c.bus.write32(list + 0xc0, 0); // skip cursor
        c.bus.write32(list + 0xbc, 0);
    }

    #[test]
    fn script_engine_list_add_matches() {
        check(ee::s07::f_00278e20__generated, script_engine_list_add, 100, &|c, seed| {
            script_engine_setup(c, seed, 0x278e20)
        });
    }

    #[test]
    fn script_engine_list_skip_matches() {
        check(ee::s07::f_00279070__generated, script_engine_list_skip, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x279070, 2);
            let lists = SCRATCH + 0x1000;
            let index = seed % 4;
            let list = lists + 0xcc * index;
            c.g[A0][0] = SCRATCH as u64;
            c.g[A1][0] = index as u64;
            c.g[A2][0] = 0; // no entries to skip: Skip returns at once
            c.g[A3][0] = r.next_u32() as u64;
            c.bus.write32(SCRATCH + 0x54c, lists);
            c.bus.write32(list + 0xa0, 1);
            c.bus.write32(list + 0xc0, 0);
            c.bus.write32(list + 0xbc, 0);
        });
    }

    // ---------------------------------------------------------------- audio

    /// A `SSXAudioSystem` at `SCRATCH` with a per-channel volume at
    /// `+0x62bc + channel*16`.
    fn audio_volume_setup(c: &mut Cpu, seed: u32, salt: u64, channel: u32) {
        let mut r = Rng(salt ^ (seed as u64) << 9);
        crate::difftest::scramble(c, &mut r);
        c.g[GP][0] = crate::difftest::GAME_GP as u64;
        c.g[A0][0] = SCRATCH as u64;
        c.bus.write32(
            SCRATCH + 0x62bc + 16 * channel,
            if seed % 3 == 0 { r.edge_f32() } else { r.f32(1.0) },
        );
    }

    #[test]
    fn ssx_audio_system_get_music_volume_level_matches() {
        check(
            ee::s07::f_00287670__generated,
            ssx_audio_system_get_music_volume_level,
            120,
            &|c, seed| audio_volume_setup(c, seed, 0x287670, 1),
        );
    }

    #[test]
    fn ssx_audio_system_get_sfx_volume_level_matches() {
        check(
            ee::s07::f_002876a0__generated,
            ssx_audio_system_get_sfx_volume_level,
            120,
            &|c, seed| audio_volume_setup(c, seed, 0x2876a0, 5),
        );
    }

    #[test]
    fn ssx_audio_system_get_speech_volume_level_matches() {
        check(
            ee::s07::f_002876d0__generated,
            ssx_audio_system_get_speech_volume_level,
            120,
            &|c, seed| audio_volume_setup(c, seed, 0x2876d0, 4),
        );
    }

    #[test]
    fn ssx_audio_system_crowd_state_callback_matches() {
        check(ee::s07::f_00289650__generated, ssx_audio_system_crowd_state_callback, 100, &|c, seed| {
            let _r = machine(c, seed, 0x289650, 1);
            let state = SCRATCH + 0x200;
            c.g[A0][0] = state as u64;
            c.bus.write32(state, if seed % 2 == 0 { 3 } else { (seed % 5) as u32 });
            c.bus.write32(crate::difftest::GAME_GP + 0x410, SCRATCH + 0x300); // D_004A3500
        });
    }

    #[test]
    fn snd_bank_man_get_bank_buffer_matches() {
        check(ee::s07::f_0028bee0__generated, snd_bank_man_get_bank_buffer, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x28bee0, 3);
            let banks = SCRATCH + 0x1000;
            let index = seed % 4;
            let bank = banks + 0x60 * index;
            // A loaded bank (`+8` non-zero) just hands back its buffer at
            // `+0x10`; an empty one allocates through the sound heap, whose
            // arena must be set up first (see `MEM_AllocNamed128`'s test).
            let loaded = seed % 2 == 0;
            if !loaded {
                heap_init(c);
            }
            c.g[A0][0] = SCRATCH as u64;
            c.g[A1][0] = index as u64;
            // `a2` becomes the allocation size and `a3` its flags (both are
            // forwarded to `MEM_AllocNamed128`), so keep the size modest.
            c.g[A2][0] = (0x40 + seed % 0x100) as u64;
            c.g[A3][0] = (r.next_u32() & 1) as u64;
            c.bus.write32(SCRATCH + 0xacc, banks);
            c.bus.write32(bank + 8, loaded as u32);
            c.bus.write32(bank + 0x10, r.next_u32());
        });
    }

    /// Set up the sound heap the allocator inside `SndBank_GetBuffer` uses.
    fn heap_init(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        c.g[GP][0] = crate::difftest::GAME_GP as u64;
        load(
            c,
            0x0031_ff60,
            ssx3_ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
    }

    /// Call a setup helper and check that it returned.
    fn load(c: &mut Cpu, addr: u32, f: fn(&mut Cpu), args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, crate::difftest::RA, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    #[test]
    fn ssx_audio_system_tick_listener_matches() {
        check(ee::s08::f_00294f48__generated, ssx_audio_system_tick_listener, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x294f48, 1);
            let this = SCRATCH;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
            // `*(this + 0xc)` is the listener vtable; slot `+8/+0xc` is the
            // tick method, which lands in the fake-call hook.
            let vt = fakes::vtable(c, &mut r, &[8]);
            c.bus.write32(this + 0xc, vt);
        });
    }

    #[test]
    fn ssx_audio_system_on_script_rider_speech2_matches() {
        check(
            ee::s08::f_002a1b58__generated,
            ssx_audio_system_on_script_rider_speech2,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a1b58, 2);
                c.g[A0][0] = SCRATCH as u64;
                c.g[A1][0] = (seed % 3) as u64; // 2 = a rider speech request
                c.g[A2][0] = (SCRATCH + 0x300) as u64;
                // Speech channel 4's volume: a plain 0.5 keeps the volume
                // query on its "not muted" early-out.
                c.bus.write32(SCRATCH + 0x62fc, 0.5f32.to_bits());
                c.bus.write32(SCRATCH + 0x300, r.next_u32());
            },
        );
    }

    #[test]
    fn ssx_audio_system_reset_race_result_state_matches() {
        check(
            ee::s08::f_002a4590__generated,
            ssx_audio_system_reset_race_result_state,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a4590, 0);
                c.g[A0][0] = SCRATCH as u64;
                c.g[A1][0] = (seed % 2) as u64;
                crate::difftest::fill(c, &mut r, SCRATCH + 0x57e0, 0x10);
            },
        );
    }

    /// A crowd-audio object at `SCRATCH` with four voices at `+0x18`. Every
    /// voice has no pending sound (`+0x174 == -1`) and no sound to stop
    /// (`+0xd4 == 0`), so the instance helpers return immediately. Both the
    /// play form (index in `a2`) and the stop form (index in `a1`) get a small
    /// in-range index.
    fn crowd_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 4);
        c.g[A0][0] = SCRATCH as u64;
        c.g[A1][0] = (seed % 4) as u64;
        c.g[A2][0] = ((seed / 4) % 4) as u64;
        let voices = SCRATCH + 0x1000;
        c.bus.write32(SCRATCH + 0x18, voices);
        for i in 0..4u32 {
            let voice = voices + 0x190 * i;
            c.bus.write32(voice + 0x174, 0xffff_ffff);
            c.bus.write32(voice + 0xd4, 0);
            c.bus.write32(voice + 0x188, r.next_u32() & 0x3f80_0000);
        }
    }

    #[test]
    fn crowd_audio_play_trick_matches() {
        check(ee::s08::f_002a73a8__generated, crowd_audio_play_trick, 100, &|c, seed| {
            crowd_setup(c, seed, 0x2a73a8)
        });
    }

    #[test]
    fn crowd_audio_play_fall_matches() {
        check(ee::s08::f_002a73d8__generated, crowd_audio_play_fall, 100, &|c, seed| {
            crowd_setup(c, seed, 0x2a73d8)
        });
    }

    #[test]
    fn crowd_audio_stop_anticipate_matches() {
        check(ee::s08::f_002a7438__generated, crowd_audio_stop_anticipate, 100, &|c, seed| {
            crowd_setup(c, seed, 0x2a7438)
        });
    }
}
