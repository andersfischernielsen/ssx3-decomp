//! `src/dirtysock/gamecomm.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::arg;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::game::ai::rider::gp_at;
use crate::game::bx::memblockman::cd_tray::GP_TRAY_OPENED;
use crate::game::misc::RACE_WORLD_GAME_MODULE;
use crate::game::misc::RACE_WORLD_GP;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0025_7a80,
        c_game_comm_check_peer_timeout,
        "cGameComm_CheckPeerTimeout",
    ),
    (
        0x0025_8330,
        c_game_comm_update_max_latency,
        "cGameComm_UpdateMaxLatency",
    ),
    (
        0x0025_8870,
        c_game_comm_set_timeout_timer,
        "cGameComm_SetTimeoutTimer",
    ),
    (0x0025_6bf8, c_game_comm_create, "cGameComm_Create"),
    (0x0025_6c50, c_game_comm_destroy, "cGameComm_Destroy"),
    (0x0025_6c78, c_game_comm_ctor, "cGameComm_ctor"),
    (0x0025_6ee0, c_game_comm_dtor, "cGameComm_dtor"),
    (0x0025_7038, c_game_comm_set_state, "cGameComm_SetState"),
    (0x0025_70d8, c_game_comm_update, "cGameComm_Update"),
    (
        0x0025_7610,
        c_game_comm_reset_race_sync,
        "cGameComm_ResetRaceSync",
    ),
    (
        0x0025_7678,
        c_game_comm_latch_result_flags,
        "cGameComm_LatchResultFlags",
    ),
    (
        0x0025_7750,
        c_game_comm_tick_countdowns,
        "cGameComm_TickCountdowns",
    ),
    (
        0x0025_77c8,
        c_game_comm_check_connection,
        "cGameComm_CheckConnection",
    ),
    (0x0025_78f8, c_game_comm_end_session, "cGameComm_EndSession"),
    (
        0x0025_79c8,
        c_game_comm_tick_quit_countdown,
        "cGameComm_TickQuitCountdown",
    ),
    (0x0025_7bd8, c_game_comm_pop_packet, "cGameComm_PopPacket"),
    (
        0x0025_7cd8,
        c_game_comm_receive_packets,
        "cGameComm_ReceivePackets",
    ),
    (
        0x0025_7dc0,
        c_game_comm_flush_send_queue,
        "cGameComm_FlushSendQueue",
    ),
    (0x0025_7e98, c_game_comm_send_packet, "cGameComm_SendPacket"),
    (
        0x0025_7f98,
        c_game_comm_process_control_packets,
        "cGameComm_ProcessControlPackets",
    ),
    (
        0x0025_8160,
        c_game_comm_recv_game_packet,
        "cGameComm_RecvGamePacket",
    ),
    (
        0x0025_8188,
        c_game_comm_alloc_packet,
        "cGameComm_AllocPacket",
    ),
    (0x0025_81b8, c_game_comm_free_packet, "cGameComm_FreePacket"),
    (
        0x0025_81d8,
        c_game_comm_is_timer_a124_idle,
        "cGameComm_IsTimerA124Idle",
    ),
    (
        0x0025_81e8,
        c_game_comm_set_timer_a124,
        "cGameComm_SetTimerA124",
    ),
    (
        0x0025_81f8,
        c_game_comm_recv_bulk_data,
        "cGameComm_RecvBulkData",
    ),
    (
        0x0025_8280,
        c_game_comm_is_timer_b12c_idle,
        "cGameComm_IsTimerB12CIdle",
    ),
    (
        0x0025_8290,
        c_game_comm_set_timer_b12c,
        "cGameComm_SetTimerB12C",
    ),
    (
        0x0025_82a0,
        c_game_comm_send_bulk_data,
        "cGameComm_SendBulkData",
    ),
    (
        0x0025_8360,
        c_game_comm_get_input_delay,
        "cGameComm_GetInputDelay",
    ),
    (0x0025_83a8, c_game_comm_ping_server, "cGameComm_PingServer"),
    (
        0x0025_8448,
        c_game_comm_exchange_setup_data,
        "cGameComm_ExchangeSetupData",
    ),
    (
        0x0025_86b0,
        c_game_comm_wait_for_peer_ready,
        "cGameComm_WaitForPeerReady",
    ),
    (
        0x0025_8790,
        c_game_comm_send_pause_state,
        "cGameComm_SendPauseState",
    ),
    (
        0x0025_88a8,
        c_game_comm_update_idle_kick_timer,
        "cGameComm_UpdateIdleKickTimer",
    ),
    (
        0x0025_8950,
        c_game_comm_on_race_start_reset,
        "cGameComm_OnRaceStartReset",
    ),
    (
        0x0025_8968,
        c_game_comm_on_countdown_start,
        "cGameComm_OnCountdownStart",
    ),
    (
        0x0025_8988,
        c_game_comm_on_race_start,
        "cGameComm_OnRaceStart",
    ),
    (0x0025_8a48, c_game_comm_on_race_end, "cGameComm_OnRaceEnd"),
    (0x0025_8ae0, c_game_comm_quit_race, "cGameComm_QuitRace"),
    (0x0025_8bc8, c_game_comm_send_quit, "cGameComm_SendQuit"),
    (0x0025_8c00, c_game_comm_send_resync, "cGameComm_SendResync"),
    (0x0025_8c50, c_game_comm_send_string, "cGameComm_SendString"),
    (
        0x0025_8cb0,
        c_game_comm_update_wait_popup,
        "cGameComm_UpdateWaitPopup",
    ),
    (
        0x0025_8f20,
        c_game_comm_send_sync_check,
        "cGameComm_SendSyncCheck",
    ),
];

// ---------------------------------------------------------------------------
// Game comm.

/// Ticks of peer silence before the "waiting for peer" notice appears.
const PEER_WAIT_NOTICE_TICKS: u32 = 60;
/// [`game_comm::PEER_NO_RESPONSE_TICKS`] at which the session ends as timed
/// out.
const PEER_NO_RESPONSE_LIMIT: u32 = 1800;
/// Late ticks at which the lag meter starts scoring. The tick that reaches it
/// scores too, and counts one lag event.
const LAG_ONSET_TICKS: u32 = 10;
/// [`game_comm::ABORT_REASON`] for opening the disc tray.
const ABORT_REASON_TRAY_OPENED: u64 = 1;
/// The AI race manager's requested state: `Countdown` and `Race` (the labels
/// `AI_SelectRaceState` stores for these two).
const RACE_STATE_COUNTDOWN: u32 = 4;
const RACE_STATE_RACE: u32 = 5;

/// `cGameComm` fields and the game objects its peer checks read. The game's
/// names for these are unknown; each is named for how the code uses it.
mod game_comm {
    /// Non-zero once the session has closed. `cGameComm_EndSession` clears
    /// +0x84, +0x78, +0x3f8 and +0x3fc first, then skips the rest if set.
    #[cfg(test)]
    pub const SESSION_CLOSED: u32 = 0x4;
    /// Set by `cGameComm_SendSyncCheck` when the sync ring at +0x410 disagrees
    /// with the peer's ring at +0x500 for the same frame. Ends the session.
    pub const DESYNCED: u32 = 0xc;
    /// Non-zero once the session is ending, and why; stops every peer check.
    pub const ABORT_REASON: u32 = 0x24;
    /// Set when the peer stopped responding or the session deadline ran out.
    pub const TIMED_OUT: u32 = 0x2c;
    /// Counts every tick while [`RACE_ACTIVE`] and the peer is not paused, up
    /// to [`PEER_NO_RESPONSE_LIMIT`](super::PEER_NO_RESPONSE_LIMIT).
    /// `cGameComm_PopPacket`, both pause packets and
    /// `cGameComm_SendPauseState` clear it.
    pub const PEER_NO_RESPONSE_TICKS: u32 = 0x5c;
    /// Ticks without a packet from the peer, counted towards
    /// [`PEER_WAIT_NOTICE_TICKS`](super::PEER_WAIT_NOTICE_TICKS). Goes up
    /// together with [`LATE_TICKS`]; control packets also clear this one.
    pub const PEER_SILENT_TICKS: u32 = 0x60;
    /// Set by `cGameComm_SendPauseState` while this player has paused.
    pub const LOCAL_PAUSED: u32 = 0x74;
    /// Set by the peer's pause packet and cleared by its resume
    /// (`cGameComm_ProcessControlPackets`); mirrors [`LOCAL_PAUSED`].
    pub const PEER_PAUSED: u32 = 0x78;
    /// Set by `cGameComm_OnCountdownStart`. Gates late counting: before the
    /// countdown a silent peer is not late.
    pub const COUNTDOWN_STARTED: u32 = 0x7c;
    /// `cGameComm_OnRaceStart` sets it to 1; `cGameComm_OnRaceEnd` and
    /// `cGameComm_ResetRaceSync` clear it.
    pub const RACE_ACTIVE: u32 = 0x80;
    /// Shows the "waiting for peer" notice.
    pub const SHOW_PEER_WAIT: u32 = 0x84;
    /// Session deadline, as a count of checks left. `cGameComm_CheckConnection`
    /// (0x2577c8) decrements it on each call, and `cGameComm_Update` calls that
    /// twice, so 1800 lasts about 15 s at 60 Hz. At 0 it sets [`TIMED_OUT`],
    /// latches the results and ends the session. `cGameComm_PopPacket` rearms
    /// it to 1800 while it is positive; `cGameComm_OnCountdownStart`,
    /// `cGameComm_OnRaceStart` and `cGameComm_WaitForPeerReady` clear it.
    pub const SESSION_DEADLINE: u32 = 0x94;
    /// Ticks the peer has been late, for the lag meter. Goes up together with
    /// [`PEER_SILENT_TICKS`]; only a data packet clears it.
    pub const LATE_TICKS: u32 = 0xa8;
    /// Lag meter (float): +1.0 each tick from
    /// [`LAG_ONSET_TICKS`](super::LAG_ONSET_TICKS) on.
    /// `cGameComm_CheckConnection` ends the session once it reaches 3600.0,
    /// a minute of lag at 60 Hz.
    pub const LAG_SCORE: u32 = 0xb0;
    /// Number of lag episodes: +1 on the tick [`LATE_TICKS`] hits the onset.
    pub const LAG_EVENTS: u32 = 0xb4;
    /// Per-tick countdown (`cGameComm_Update` decrements it, armed to 180 by
    /// `cGameComm_UpdateIdleKickTimer`); while positive the peer is not late.
    pub const RESUME_GRACE: u32 = 0x3fc;
    /// `GameModule`: the AI race manager, whose first word is the requested
    /// race state.
    pub const AI_RACE_MANAGER: u32 = 0xc;
}

pub(crate) mod callee {
    use super::Callee;

    pub(crate) const C_GAME_COMM_LATCH_RESULT_FLAGS: Callee = Callee {
        addr: 0x0025_7678,
        f: ssx3_ee::s06::cGameComm_LatchResultFlags,
    };

    pub(crate) const C_GAME_COMM_END_SESSION: Callee = Callee {
        addr: 0x0025_78f8,
        f: ssx3_ee::s06::cGameComm_EndSession,
    };
    use super::ee;
    pub(crate) use crate::game::fe::fepopupmisc::callee::BE_LIBRARY_GET_INTERFACE_4 as BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::fe::fepopupmisc::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::fe::fepopupmisc::callee::C_GAME_COMM_UPDATE;
    pub(crate) use crate::game::fe::fepopupmisc::callee::EA_MEMSET;
    pub(crate) use crate::game::fe::fepopupmisc::callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::fe::fepopupmisc::callee::EA_MEM_FREE;
    pub(crate) use crate::game::fe::fepopupmisc::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::misc::callee::BE_NETWORK_INT_GET_SYNC_DATA_SIZE;
    pub(crate) use crate::game::misc::callee::C_GAME_COMM_CTOR;
    pub(crate) use crate::game::misc::callee::C_GAME_COMM_DTOR;
    pub(crate) use crate::game::misc::callee::C_GAME_COMM_FREE_PACKET;
    pub(crate) use crate::game::misc::callee::C_GAME_COMM_SET_TIMEOUT_TIMER;
    pub(crate) use crate::game::misc::callee::C_GAME_COMM_UPDATE_MAX_LATENCY;
    pub(crate) use crate::game::misc::callee::PROTO_PING_CREATE;
    pub(crate) use crate::game::misc::callee::PROTO_PING_DESTROY;
    pub(crate) use crate::game::misc::callee::SSX_CLEAR_TRAY_OPENED;

    pub const PROTO_PING_REQUEST: Callee = Callee {
        addr: 0x003e_7450,
        f: ee::s14::f_003e7450,
    };

    pub const PROTO_PING_RESPONSE: Callee = Callee {
        addr: 0x003e_7500,
        f: ee::s14::f_003e7500,
    };

    pub const STRNCMP: Callee = Callee {
        addr: 0x0041_6b18,
        f: ee::s15::f_00416b18,
    };

    pub const C_COMM_SYSTEM_GET_CHANNEL_LATENCY: Callee = Callee {
        addr: 0x0025_5d88,
        f: ee::s06::f_00255d88,
    };
    pub(crate) use crate::game::dirtysock::tagsunk::callee::C_GAME_COMM_SEND_PACKET as C_GAME_COMM_SEND_PACKET_2;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::STR_HASH;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::USTR_COPY;

    /// `cGameComm_SendPacket` (0x00257e98): queue a packet for the peer.
    pub(crate) const C_GAME_COMM_SEND_PACKET: Callee = Callee {
        addr: 0x0025_7e98,
        f: ee::s06::cGameComm_SendPacket,
    };

    /// `cGameComm_UpdateIdleKickTimer` (0x002588a8, ported in this module).
    pub(crate) const C_GAME_COMM_UPDATE_IDLE_KICK_TIMER: Callee = Callee {
        addr: 0x0025_88a8,
        f: ee::s06::cGameComm_UpdateIdleKickTimer,
    };

    pub const C_COMM_SYSTEM_CHANNEL_RECV: Callee = Callee {
        addr: 0x0025_5dc0,
        f: ee::s06::f_00255dc0,
    };

    pub const C_GAME_COMM_ALLOC_PACKET: Callee = Callee {
        addr: 0x0025_8188,
        f: ee::s06::f_00258188,
    };

    pub const C_COMM_SYSTEM_CHANNEL_SEND: Callee = Callee {
        addr: 0x0025_5e00,
        f: ee::s06::f_00255e00,
    };

    /// `cGameComm_SetState` (0x00257038): request a comm state change.
    pub(crate) const C_GAME_COMM_SET_STATE: Callee = Callee {
        addr: 0x0025_7038,
        f: ee::s06::cGameComm_SetState,
    };

    pub const C_GAME_COMM_TICK_COUNTDOWNS: Callee = Callee {
        addr: 0x0025_7750,
        f: ee::s06::f_00257750,
    };

    pub const C_GAME_COMM_CHECK_CONNECTION: Callee = Callee {
        addr: 0x0025_77c8,
        f: ee::s06::f_002577c8,
    };

    pub const C_GAME_COMM_RECEIVE_PACKETS: Callee = Callee {
        addr: 0x0025_7cd8,
        f: ee::s06::f_00257cd8,
    };

    pub const C_GAME_COMM_FLUSH_SEND_QUEUE: Callee = Callee {
        addr: 0x0025_7dc0,
        f: ee::s06::f_00257dc0,
    };

    pub const C_GAME_COMM_RECV_BULK_DATA: Callee = Callee {
        addr: 0x0025_81f8,
        f: ee::s06::f_002581f8,
    };

    pub const C_GAME_COMM_SEND_BULK_DATA: Callee = Callee {
        addr: 0x0025_82a0,
        f: ee::s06::f_002582a0,
    };

    pub const C_GAME_COMM_PROCESS_CONTROL_PACKETS: Callee = Callee {
        addr: 0x0025_7f98,
        f: ee::s06::f_00257f98,
    };

    pub const C_GAME_COMM_IS_TIMER_A124_IDLE: Callee = Callee {
        addr: 0x0025_81d8,
        f: ee::s06::f_002581d8,
    };

    pub const C_GAME_COMM_SET_TIMER_A124: Callee = Callee {
        addr: 0x0025_81e8,
        f: ee::s06::f_002581e8,
    };

    pub const C_GAME_COMM_IS_TIMER_B12C_IDLE: Callee = Callee {
        addr: 0x0025_8280,
        f: ee::s06::f_00258280,
    };

    pub const C_GAME_COMM_SET_TIMER_B12C: Callee = Callee {
        addr: 0x0025_8290,
        f: ee::s06::f_00258290,
    };

    pub const C_GAME_COMM_CHECK_PEER_TIMEOUT: Callee = Callee {
        addr: 0x0025_7a80,
        f: ee::s06::f_00257a80,
    };

    pub const C_GAME_COMM_UPDATE_WAIT_POPUP: Callee = Callee {
        addr: 0x0025_8cb0,
        f: ee::s06::f_00258cb0,
    };

    pub const C_VOIP_MAN_STOP: Callee = Callee {
        addr: 0x0026_6df8,
        f: ee::s07::f_00266df8,
    };

    pub const GAME_UI_OPEN_SCREEN: Callee = Callee {
        addr: 0x0020_ab50,
        f: ee::s05::f_0020ab50,
    };

    pub const UI_STATE_MGR_GET_ACTIVE_STATE: Callee = Callee {
        addr: 0x0039_fa88,
        f: ee::s13::f_0039fa88,
    };

    pub const EA_MEM_COPY: Callee = Callee {
        addr: 0x003e_6574,
        f: ee::s14::f_003e6574,
    };

    pub const THREAD_YIELDTICKS: Callee = Callee {
        addr: 0x003e_52b0,
        f: ee::s14::f_003e52b0,
    };

    pub const RAND_NEXT_GAME: Callee = Callee {
        addr: 0x0031_7810,
        f: ee::s10::f_00317810,
    };

    pub const RAND_SAVE_GAME_STATE: Callee = Callee {
        addr: 0x0031_78e0,
        f: ee::s10::f_003178e0,
    };

    pub const RAND_RESTORE_GAME_STATE: Callee = Callee {
        addr: 0x0031_7908,
        f: ee::s10::f_00317908,
    };

    pub const BE_NEW_PLAYER_INT_SET_PAD_INDEX_BY_SLOT: Callee = Callee {
        addr: 0x0014_72c8,
        f: ee::s01::f_001472c8,
    };

    pub const BE_NEW_PLAYER_INT_GET_PAD_INDEX_BY_SLOT: Callee = Callee {
        addr: 0x0014_7318,
        f: ee::s01::f_00147318,
    };

    pub const BELIBRARY_GET_INSTANCE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::f_0014dc80,
    };

    pub const BE_NETWORK_INT_PACK_PLAYER_DATA: Callee = Callee {
        addr: 0x0014_e2c8,
        f: ee::s01::f_0014e2c8,
    };

    pub const BE_NETWORK_INT_UNPACK_PLAYER_DATA: Callee = Callee {
        addr: 0x0014_e5c8,
        f: ee::s01::f_0014e5c8,
    };

    pub const BE_PREPARE_NET_PROFILES: Callee = Callee {
        addr: 0x0014_e9f8,
        f: ee::s01::f_0014e9f8,
    };

    pub const BE_RESET_NET_INTERFACES: Callee = Callee {
        addr: 0x0014_ea90,
        f: ee::s01::f_0014ea90,
    };

    pub const BEOPTION_INT_SWAP_PLAYER_SETTINGS: Callee = Callee {
        addr: 0x0014_f898,
        f: ee::s01::f_0014f898,
    };

    pub const STRNCPY: Callee = Callee {
        addr: 0x0041_6ce0,
        f: ee::s15::f_00416ce0,
    };

    pub const GAME_RAND_SEED: Callee = Callee {
        addr: 0x0023_7cb0,
        f: ee::s06::f_00237cb0,
    };

    pub const C_COMM_SYSTEM_GET_CHANNEL_STATUS: Callee = Callee {
        addr: 0x0025_5e40,
        f: ee::s06::f_00255e40,
    };

    pub const C_GAME_COMM_TICK_QUIT_COUNTDOWN: Callee = Callee {
        addr: 0x0025_79c8,
        f: ee::s06::f_002579c8,
    };

    pub const C_GAME_COMM_POP_PACKET: Callee = Callee {
        addr: 0x0025_7bd8,
        f: ee::s06::f_00257bd8,
    };

    /// `cGameComm_OnRaceEnd` (0x00258a48, ported in this module).
    pub(crate) const C_GAME_COMM_ON_RACE_END: Callee = Callee {
        addr: 0x0025_8a48,
        f: ee::s06::cGameComm_OnRaceEnd,
    };

    /// `cGameComm_SendQuit` (0x00258bc8, ported in this module).
    pub(crate) const C_GAME_COMM_SEND_QUIT: Callee = Callee {
        addr: 0x0025_8bc8,
        f: ee::s06::cGameComm_SendQuit,
    };

    /// `BuddyMan_Create` (0x00260f80; main: `tagsunk::buddyman_create`).
    pub(crate) const BUDDYMAN_CREATE: Callee = Callee {
        addr: 0x0026_0f80,
        f: ee::s06::BuddyMan_Create,
    };

    /// `BuddyMan_IsConnected` (0x00261460; main:
    /// `tagsunk::buddy_man_is_connected`).
    pub(crate) const BUDDY_MAN_IS_CONNECTED: Callee = Callee {
        addr: 0x0026_1460,
        f: ee::s06::BuddyMan_IsConnected,
    };

    /// `BuddyMan_Reconnect` (0x002613c8; main:
    /// `tagsunk::buddy_man_reconnect`).
    pub(crate) const BUDDY_MAN_RECONNECT: Callee = Callee {
        addr: 0x0026_13c8,
        f: ee::s06::BuddyMan_Reconnect,
    };

    /// `LobbyMan_Disconnect` (0x0025b800; main:
    /// `tagsunk::lobby_man_disconnect`).
    pub(crate) const LOBBY_MAN_DISCONNECT: Callee = Callee {
        addr: 0x0025_b800,
        f: ee::s06::LobbyMan_Disconnect,
    };

    /// `BuddyMan_SetPresence` (0x00262768; main:
    /// `tagsunk::buddy_man_set_presence`).
    pub(crate) const BUDDY_MAN_SET_PRESENCE: Callee = Callee {
        addr: 0x0026_2768,
        f: ee::s07::BuddyMan_SetPresence,
    };

    /// `BELibrary_GetInstance` (0x0014dc80).
    pub(crate) const BE_LIBRARY_GET_INSTANCE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::BELibrary_GetInstance,
    };

    /// `BE_CopyProfile` (0x0014e9c0).
    pub(crate) const BE_COPY_PROFILE: Callee = Callee {
        addr: 0x0014_e9c0,
        f: ee::s01::BE_CopyProfile,
    };

    /// `strlen` (0x00416810): string length for the string packet.
    pub(crate) const STRLEN: Callee = Callee {
        addr: 0x0041_6810,
        f: ee::s15::f_00416810,
    };

    /// `THREAD_yieldticks` (0x003e52b0): yield the thread. Only called from
    /// the wait loop, whose test gates it (see `gcomm_fix_fake_yield`).
    pub(crate) const THREAD_YIELD_TICKS: Callee = Callee {
        addr: 0x003e_52b0,
        f: ee::s14::THREAD_yieldticks,
    };
}

/// `void cGameComm_UpdateMaxLatency(cGameComm *self)` (0x258330): keep the
/// running maximum of the two latency samples, clamped to the world's target
/// latency.
pub fn c_game_comm_update_max_latency(c: &mut Cpu) {
    c.cycles += 10;
    let this = Addr(abi::arg(c, 0));
    let a = this.off(0x114).read_fpu(c);
    let b = this.off(0x118).read_fpu(c);
    c.f[0] = a.0;
    c.f[1] = b.0;
    let world = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[V0][0] = world;
    let max = b.max(a);
    c.f[1] = max.0;
    this.off(0x11c).write_fpu(c, max);
    let limit = Addr(world as u32).off(0x14).read_fpu(c);
    c.f[0] = limit.0;
    if abi::c_lt_s(c, max, limit) {
        this.off(0x11c).write_fpu(c, limit);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void cGameComm_SetTimeoutTimer(cGameComm *self, int count, bool force)`
/// (0x258870): rearm the session deadline, unless either player is paused
/// before the countdown. `force` or a started countdown rearms anyway. Before
/// the countdown, both pause paths (the peer's pause packet and
/// `cGameComm_SendPauseState`) force 3660, and while paused the unforced
/// rearms are skipped. The deadline keeps counting down during a pause.
pub fn c_game_comm_set_timeout_timer(c: &mut Cpu) {
    use game_comm::*;
    let comm = Addr(abi::arg(c, 0));
    c.cycles += 2;
    let rearm = if c.g[A2][0] != 0 {
        true
    } else {
        c.cycles += 3;
        c.g[V0][0] = comm.off(PEER_PAUSED).lw(c);
        if c.g[V0][0] != 0 {
            c.g[V0][0] = comm.off(COUNTDOWN_STARTED).lw(c);
            c.cycles += 2;
            c.g[V0][0] != 0
        } else {
            c.cycles += 3;
            c.g[V0][0] = comm.off(LOCAL_PAUSED).lw(c);
            if c.g[V0][0] == 0 {
                true
            } else {
                c.cycles += 1;
                c.g[V0][0] = comm.off(COUNTDOWN_STARTED).lw(c);
                c.cycles += 2;
                c.g[V0][0] != 0
            }
        }
    };
    if rearm {
        comm.off(SESSION_DEADLINE).sw(c, c.g[A1][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void cGameComm_CheckPeerTimeout(cGameComm *self)` (0x257a80): track an
/// unresponsive peer, show the wait notice and end a timed-out or desynced
/// session. A paused peer is not late, because it sends nothing while paused.
/// The lag score and lag-event count only change during the countdown and the
/// race. Opening the disc tray ends the session too, with
/// [`ABORT_REASON_TRAY_OPENED`].
pub fn c_game_comm_check_peer_timeout(c: &mut Cpu) {
    use game_comm::*;
    const SAVED_RA: u32 = 0;
    const SAVED_SELF: u32 = 0x10;
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, SAVED_SELF);
        frame.save_ra(c, SAVED_RA);
        c.g[S0][0] = c.g[A0][0];
        let comm = Addr(abi::r32(c, S0));
        c.g[V0][0] = comm.off(ABORT_REASON).lw(c);
        'session: {
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, SAVED_SELF);
                break 'session;
            }
            c.cycles += 3;
            c.g[V0][0] = comm.off(LOCAL_PAUSED).lw(c);
            let local_paused = c.g[V0][0] != 0;
            c.g[V0][0] = gp_at(c, GP_TRAY_OPENED).lw(c);
            if !local_paused {
                c.cycles += 3;
                c.g[V0][0] = comm.off(COUNTDOWN_STARTED).lw(c);
                if c.g[V0][0] != 0 {
                    c.cycles += 3;
                    c.g[V0][0] = comm.off(PEER_PAUSED).lw(c);
                    let mut count_late = false;
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = comm.off(PEER_SILENT_TICKS).lw(c);
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = comm.off(RESUME_GRACE).lw(c);
                        if (c.g[V0][0] as i64) > 0 {
                            c.g[V1][0] = comm.off(PEER_SILENT_TICKS).lw(c);
                        } else {
                            count_late = true;
                        }
                    }
                    if count_late {
                        c.cycles += 14;
                        c.g[V1][0] = comm.off(LATE_TICKS).lw(c);
                        c.g[V0][0] = comm.off(PEER_SILENT_TICKS).lw(c);
                        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(RACE_WORLD_GP)).lw(c);
                        c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                        comm.off(LATE_TICKS).sw(c, c.g[A2][0]);
                        comm.off(PEER_SILENT_TICKS).sw(c, c.g[V0][0]);
                        c.g[V1][0] = Addr(abi::r32(c, A1)).off(RACE_WORLD_GAME_MODULE).lw(c);
                        c.g[A0][0] = Addr(abi::r32(c, V1)).off(AI_RACE_MANAGER).lw(c);
                        c.g[V0][0] = Addr(abi::r32(c, A0)).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(RACE_STATE_COUNTDOWN));
                        c.g[V0][0] = (c.g[V0][0]
                            < (RACE_STATE_RACE - RACE_STATE_COUNTDOWN + 1) as u64)
                            as u64;
                        let countdown_or_race = c.g[V0][0] != 0;
                        c.g[V0][0] = ((c.g[A2][0] as i64) < LAG_ONSET_TICKS as i64) as u64;
                        let mut reload_silent_ticks = true;
                        if countdown_or_race {
                            c.cycles += 2;
                            if c.g[V0][0] == 0 {
                                c.cycles += 6;
                                c.f[0] = comm.off(LAG_SCORE).read_fpu(c).0;
                                c.g[AT][0] = FpuF32::ONE.0 as u64;
                                c.f[1] = FpuF32::ONE.0;
                                c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[1])).0;
                                comm.off(LAG_SCORE).write_fpu(c, FpuF32(c.f[0]));
                            }
                            c.g[V1][0] = comm.off(LATE_TICKS).lw(c);
                            c.cycles += 3;
                            c.g[V0][0] = LAG_ONSET_TICKS as u64;
                            if c.g[V1][0] != c.g[V0][0] {
                                c.g[V1][0] = comm.off(PEER_SILENT_TICKS).lw(c);
                                reload_silent_ticks = false;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = comm.off(LAG_EVENTS).lw(c);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                                comm.off(LAG_EVENTS).sw(c, c.g[V0][0]);
                            }
                        }
                        if reload_silent_ticks {
                            c.cycles += 1;
                            c.g[V1][0] = comm.off(PEER_SILENT_TICKS).lw(c);
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = PEER_WAIT_NOTICE_TICKS as u64;
                    if c.g[V1][0] == c.g[V0][0] {
                        c.cycles += 3;
                        c.g[V0][0] = 1;
                        comm.off(SHOW_PEER_WAIT).sw(c, c.g[V0][0]);
                    }
                }
                c.g[V0][0] = comm.off(RACE_ACTIVE).lw(c);
                c.cycles += 2;
                let race_active = c.g[V0][0] != 0;
                c.g[V0][0] = gp_at(c, GP_TRAY_OPENED).lw(c);
                if race_active {
                    c.cycles += 3;
                    c.g[V0][0] = comm.off(DESYNCED).lw(c);
                    let desynced = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S0][0];
                    let mut end_session = desynced;
                    if !desynced {
                        c.cycles += 3;
                        c.g[V0][0] = comm.off(PEER_PAUSED).lw(c);
                        let peer_paused = c.g[V0][0] != 0;
                        c.g[V0][0] = comm.off(PEER_NO_RESPONSE_TICKS).lw(c);
                        if !peer_paused {
                            c.cycles += 3;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                            comm.off(PEER_NO_RESPONSE_TICKS).sw(c, c.g[V0][0]);
                            c.g[V0][0] = comm.off(PEER_NO_RESPONSE_TICKS).lw(c);
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[V0][0] as i64) < PEER_NO_RESPONSE_LIMIT as i64) as u64;
                        let timed_out = c.g[V0][0] == 0;
                        c.g[V0][0] = gp_at(c, GP_TRAY_OPENED).lw(c);
                        if timed_out {
                            c.cycles += 3;
                            c.g[V0][0] = 1;
                            c.g[A0][0] = c.g[S0][0];
                            comm.off(TIMED_OUT).sw(c, c.g[V0][0]);
                            end_session = true;
                        }
                    }
                    if end_session {
                        c.cycles += 6;
                        callee::C_GAME_COMM_LATCH_RESULT_FLAGS.call(c, 0x0025_7ba4)?;
                        c.g[A0][0] = c.g[S0][0];
                        callee::C_GAME_COMM_END_SESSION.call(c, 0x0025_7bac)?;
                        frame.restore(c, S0, SAVED_SELF);
                        break 'session;
                    }
                }
            }
            c.cycles += 2;
            // v0 still holds the tray flag, loaded in the delay slot of
            // whichever branch came here.
            let tray_opened = c.g[V0][0] != 0;
            c.g[V0][0] = ABORT_REASON_TRAY_OPENED;
            if tray_opened {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                comm.off(ABORT_REASON).sw(c, c.g[V0][0]);
                callee::C_GAME_COMM_END_SESSION.call(c, 0x0025_7bc8)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, SAVED_SELF);
        }
        c.cycles += 3;
        frame.restore_ra(c, SAVED_RA);
        frame.close(c);
        Ok(())
    });
}

/// `cGameComm_Update` (0x002570d8): Run one game-comm update: ping countdown and peer ping/pong,
/// frame-time and countdown ticks, packet receive/flush/bulk/control, then the connection-state
/// machine. Fixed ping loops with event checks plus a jump-table state dispatch.
pub fn c_game_comm_update(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x002570d8 => {
                    c.cycles += 9;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x160));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
                    c.g[S0][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x130));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x120), c.g[RA][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x138));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    if t {
                        pc = 0x00257218;
                        continue;
                    }
                    pc = 0x002570fc;
                    continue;
                }
                0x002570fc => {
                    c.cycles += 4;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x138), c.g[V0][0]);
                    c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
                    let t = true;
                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                    if t {
                        pc = 0x0025718c;
                        continue;
                    }
                    pc = 0x0025710c;
                    continue;
                }
                0x0025710c => {
                    c.cycles += 1;
                    pc = 0x00257110;
                    continue;
                }
                0x00257110 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1380));
                    c.g[RA][0] = sx(0x00257120);
                    c.g[A2][0] = 0xd;
                    callee::STRNCMP.call(c, 0x00257120)?;
                    c.g[V1][0] = c.g[V0][0];
                    let t = c.g[V1][0] != 0u64;
                    c.g[V0][0] = 0x100;
                    if t {
                        pc = 0x00257190;
                        continue;
                    }
                    pc = 0x0025712c;
                    continue;
                }
                0x0025712c => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xb8));
                    c.g[A2][0] = c.g[V0][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x00257180;
                        continue;
                    }
                    pc = 0x00257140;
                    continue;
                }
                0x00257140 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc4));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x108));
                    let t = c.g[V0][0] == c.g[A0][0];
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x00257178;
                        continue;
                    }
                    pc = 0x00257150;
                    continue;
                }
                0x00257150 => {
                    c.cycles += 2;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    pc = 0x00257158;
                    continue;
                }
                0x00257158 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                    if t {
                        pc = 0x00257180;
                        continue;
                    }
                    pc = 0x00257164;
                    continue;
                }
                0x00257164 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc4));
                    if c.g[V1][0] != c.g[A0][0] {
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                        if c.cycles >= c.next_event {
                            c.pc = 0x00257158;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x00257158;
                        continue;
                    }
                    pc = 0x00257174;
                    continue;
                }
                0x00257174 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x00257178;
                    continue;
                }
                0x00257178 => {
                    c.cycles += 2;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x138), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x13c), c.g[V0][0]);
                    pc = 0x00257180;
                    continue;
                }
                0x00257180 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x138));
                    if (c.g[V0][0] as i64) <= 0 {
                        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0xb0));
                        pc = 0x0025721c;
                        continue;
                    }
                    pc = 0x0025718c;
                    continue;
                }
                0x0025718c => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x100;
                    pc = 0x00257190;
                    continue;
                }
                0x00257190 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x110), c.g[V0][0]);
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A2][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x002571a8);
                    c.g[A3][0] = c.g[S1][0];
                    callee::PROTO_PING_RESPONSE.call(c, 0x002571a8)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = 0x480000;
                    if t {
                        if c.cycles >= c.next_event {
                            c.pc = 0x00257110;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x00257110;
                        continue;
                    }
                    pc = 0x002571b0;
                    continue;
                }
                0x002571b0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x138));
                    if (c.g[V0][0] as i64) <= 0 {
                        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0xb0));
                        pc = 0x0025721c;
                        continue;
                    }
                    pc = 0x002571bc;
                    continue;
                }
                0x002571bc => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xb8));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 4i64) as u64;
                    if c.g[V0][0] == 0u64 {
                        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0xb0));
                        pc = 0x0025721c;
                        continue;
                    }
                    pc = 0x002571cc;
                    continue;
                }
                0x002571cc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    if t {
                        pc = 0x002571e0;
                        continue;
                    }
                    pc = 0x002571d8;
                    continue;
                }
                0x002571d8 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                    if t {
                        pc = 0x00257218;
                        continue;
                    }
                    pc = 0x002571e0;
                    continue;
                }
                0x002571e0 => {
                    c.cycles += 14;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
                    c.g[A2][0] = 0x480000;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc0));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1380));
                    c.g[RA][0] = sx(0x002571f8);
                    c.g[A3][0] = 0xe;
                    callee::PROTO_PING_REQUEST.call(c, 0x002571f8)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xb8));
                    c.g[A1][0] = 0x4;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xb8), c.g[A0][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[A1][0]);
                    pc = 0x00257218;
                    continue;
                }
                0x00257218 => {
                    c.cycles += 1;
                    c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0xb0));
                    pc = 0x0025721c;
                    continue;
                }
                0x0025721c => {
                    c.cycles += 7;
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4b78));
                    c.mtc1(0, 2);
                    c.sub_s(0, 0, 1);
                    c.c_lt_s(0, 2);
                    let t = !c.fcc();
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0xb0));
                    if t {
                        pc = 0x0025723c;
                        continue;
                    }
                    pc = 0x00257238;
                    continue;
                }
                0x00257238 => {
                    c.cycles += 1;
                    c.swc1(2, (c.g[S0][0] as u32).wrapping_add(0xb0));
                    pc = 0x0025723c;
                    continue;
                }
                0x0025723c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3fc));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    if t {
                        pc = 0x0025724c;
                        continue;
                    }
                    pc = 0x00257248;
                    continue;
                }
                0x00257248 => {
                    c.cycles += 1;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x3fc), c.g[V0][0]);
                    pc = 0x0025724c;
                    continue;
                }
                0x0025724c => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x00257254);
                    c.g[A0][0] = c.g[S0][0];
                    callee::C_GAME_COMM_UPDATE_WAIT_POPUP.call(c, 0x00257254)?;
                    c.g[RA][0] = sx(0x0025725c);
                    c.g[A0][0] = c.g[S0][0];
                    callee::C_GAME_COMM_TICK_COUNTDOWNS.call(c, 0x0025725c)?;
                    c.g[RA][0] = sx(0x00257264);
                    c.g[A0][0] = c.g[S0][0];
                    callee::C_GAME_COMM_CHECK_CONNECTION.call(c, 0x00257264)?;
                    if c.g[V0][0] != 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                        pc = 0x002575fc;
                        continue;
                    }
                    pc = 0x0025726c;
                    continue;
                }
                0x0025726c => {
                    c.cycles += 14;
                    c.g[RA][0] = sx(0x00257274);
                    c.g[A0][0] = c.g[S0][0];
                    callee::C_GAME_COMM_RECEIVE_PACKETS.call(c, 0x00257274)?;
                    c.g[RA][0] = sx(0x0025727c);
                    c.g[A0][0] = c.g[S0][0];
                    callee::C_GAME_COMM_FLUSH_SEND_QUEUE.call(c, 0x0025727c)?;
                    c.g[RA][0] = sx(0x00257284);
                    c.g[A0][0] = c.g[S0][0];
                    callee::C_GAME_COMM_RECV_BULK_DATA.call(c, 0x00257284)?;
                    c.g[RA][0] = sx(0x0025728c);
                    c.g[A0][0] = c.g[S0][0];
                    callee::C_GAME_COMM_SEND_BULK_DATA.call(c, 0x0025728c)?;
                    c.g[RA][0] = sx(0x00257294);
                    c.g[A0][0] = c.g[S0][0];
                    callee::C_GAME_COMM_PROCESS_CONTROL_PACKETS.call(c, 0x00257294)?;
                    c.g[RA][0] = sx(0x0025729c);
                    c.g[A0][0] = c.g[S0][0];
                    callee::C_GAME_COMM_CHECK_CONNECTION.call(c, 0x0025729c)?;
                    if c.g[V0][0] != 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                        pc = 0x002575fc;
                        continue;
                    }
                    pc = 0x002572a4;
                    continue;
                }
                0x002572a4 => {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A1][0] = 0x7;
                    c.g[RA][0] = sx(0x002572b4);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                    callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x002572b4)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x44));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70));
                    if t {
                        pc = 0x002573b8;
                        continue;
                    }
                    pc = 0x002572c4;
                    continue;
                }
                0x002572c4 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V1][0] < 0x10u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x002575f8;
                        continue;
                    }
                    pc = 0x002572d0;
                    continue;
                }
                0x002572d0 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x510));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;
                    pc = t;
                    continue;
                }
                0x002572e8 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xd4));
                    let t = true;
                    c.g[A2][0] = 0x2c;
                    if t {
                        pc = 0x00257448;
                        continue;
                    }
                    pc = 0x002572f8;
                    continue;
                }
                0x002572f8 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00257300);
                    c.g[A0][0] = c.g[S0][0];
                    callee::C_GAME_COMM_IS_TIMER_B12C_IDLE.call(c, 0x00257300)?;
                    if c.g[V0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                        pc = 0x002575fc;
                        continue;
                    }
                    pc = 0x00257308;
                    continue;
                }
                0x00257308 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        pc = 0x002573e0;
                        continue;
                    }
                    pc = 0x00257310;
                    continue;
                }
                0x00257310 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x100));
                    let t = true;
                    c.g[A2][0] = 0x14;
                    if t {
                        pc = 0x002573fc;
                        continue;
                    }
                    pc = 0x00257320;
                    continue;
                }
                0x00257320 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00257328);
                    c.g[A0][0] = c.g[S0][0];
                    callee::C_GAME_COMM_IS_TIMER_A124_IDLE.call(c, 0x00257328)?;
                    if c.g[V0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                        pc = 0x002575fc;
                        continue;
                    }
                    pc = 0x00257330;
                    continue;
                }
                0x00257330 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        pc = 0x002574d0;
                        continue;
                    }
                    pc = 0x00257338;
                    continue;
                }
                0x00257338 => {
                    c.cycles += 4;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3ec));
                    c.g[A0][0] = c.g[S1][0];
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x002574ec;
                        continue;
                    }
                    pc = 0x00257348;
                    continue;
                }
                0x00257348 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00257350);
                    c.g[A0][0] = c.g[S0][0];
                    callee::C_GAME_COMM_IS_TIMER_A124_IDLE.call(c, 0x00257350)?;
                    if c.g[V0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                        pc = 0x002575fc;
                        continue;
                    }
                    pc = 0x00257358;
                    continue;
                }
                0x00257358 => {
                    c.cycles += 6;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3ec));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00257368);
                    c.g[A1][0] = 0x1;
                    callee::BE_NETWORK_INT_UNPACK_PLAYER_DATA.call(c, 0x00257368)?;
                    let t = true;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        pc = 0x00257574;
                        continue;
                    }
                    pc = 0x00257370;
                    continue;
                }
                0x00257370 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    c.g[RA][0] = sx(0x0025737c);
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                    callee::C_COMM_SYSTEM_GET_CHANNEL_LATENCY.call(c, 0x0025737c)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x114));
                    let t = true;
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x114));
                    if t {
                        pc = 0x0025759c;
                        continue;
                    }
                    pc = 0x0025738c;
                    continue;
                }
                0x0025738c => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    let t = true;
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x118));
                    if t {
                        pc = 0x00257544;
                        continue;
                    }
                    pc = 0x00257398;
                    continue;
                }
                0x00257398 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002573a0);
                    c.g[A0][0] = c.g[S0][0];
                    callee::C_GAME_COMM_IS_TIMER_A124_IDLE.call(c, 0x002573a0)?;
                    let t = true;
                    if t {
                        pc = 0x002575c0;
                        continue;
                    }
                    pc = 0x002573a8;
                    continue;
                }
                0x002573a8 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002573b0);
                    c.g[A0][0] = c.g[S0][0];
                    callee::C_GAME_COMM_CHECK_PEER_TIMEOUT.call(c, 0x002573b0)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                    if t {
                        pc = 0x002575fc;
                        continue;
                    }
                    pc = 0x002573b8;
                    continue;
                }
                0x002573b8 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V1][0] < 0x10u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x002575f8;
                        continue;
                    }
                    pc = 0x002573c4;
                    continue;
                }
                0x002573c4 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x550));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;
                    pc = t;
                    continue;
                }
                0x002573dc => {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S0][0];
                    pc = 0x002573e0;
                    continue;
                }
                0x002573e0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002573e8);
                    c.g[A1][0] = 0x3;
                    callee::C_GAME_COMM_SET_STATE.call(c, 0x002573e8)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                    if t {
                        pc = 0x002575fc;
                        continue;
                    }
                    pc = 0x002573f0;
                    continue;
                }
                0x002573f0 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xd4));
                    c.g[A2][0] = 0x2c;
                    pc = 0x002573fc;
                    continue;
                }
                0x002573fc => {
                    c.cycles += 7;
                    callee::C_GAME_COMM_SET_TIMER_A124.call(c, 0x00257404)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00257410);
                    c.g[A1][0] = 0x4;
                    callee::C_GAME_COMM_SET_STATE.call(c, 0x00257410)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                    if t {
                        pc = 0x002575fc;
                        continue;
                    }
                    pc = 0x00257418;
                    continue;
                }
                0x00257418 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00257420);
                    c.g[A0][0] = c.g[S0][0];
                    callee::C_GAME_COMM_IS_TIMER_A124_IDLE.call(c, 0x00257420)?;
                    if c.g[V0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                        pc = 0x002575fc;
                        continue;
                    }
                    pc = 0x00257428;
                    continue;
                }
                0x00257428 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00257434);
                    c.g[A1][0] = 0x1;
                    callee::C_GAME_COMM_SET_STATE.call(c, 0x00257434)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                    if t {
                        pc = 0x002575fc;
                        continue;
                    }
                    pc = 0x0025743c;
                    continue;
                }
                0x0025743c => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x100));
                    c.g[A2][0] = 0x14;
                    pc = 0x00257448;
                    continue;
                }
                0x00257448 => {
                    c.cycles += 7;
                    callee::C_GAME_COMM_SET_TIMER_B12C.call(c, 0x00257450)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0025745c);
                    c.g[A1][0] = 0x2;
                    callee::C_GAME_COMM_SET_STATE.call(c, 0x0025745c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                    if t {
                        pc = 0x002575fc;
                        continue;
                    }
                    pc = 0x00257464;
                    continue;
                }
                0x00257464 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0025746c);
                    c.g[A0][0] = c.g[S0][0];
                    callee::C_GAME_COMM_IS_TIMER_B12C_IDLE.call(c, 0x0025746c)?;
                    if c.g[V0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                        pc = 0x002575fc;
                        continue;
                    }
                    pc = 0x00257474;
                    continue;
                }
                0x00257474 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00257480);
                    c.g[A1][0] = 0x7;
                    callee::C_GAME_COMM_SET_STATE.call(c, 0x00257480)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                    if t {
                        pc = 0x002575fc;
                        continue;
                    }
                    pc = 0x00257488;
                    continue;
                }
                0x00257488 => {
                    c.cycles += 9;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3ec));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00257498);
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3f0));
                    callee::C_GAME_COMM_SET_TIMER_A124.call(c, 0x00257498)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x002574a4);
                    c.g[A1][0] = 0x8;
                    callee::C_GAME_COMM_SET_STATE.call(c, 0x002574a4)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                    if t {
                        pc = 0x002575fc;
                        continue;
                    }
                    pc = 0x002574ac;
                    continue;
                }
                0x002574ac => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002574b4);
                    c.g[A0][0] = c.g[S0][0];
                    callee::C_GAME_COMM_IS_TIMER_A124_IDLE.call(c, 0x002574b4)?;
                    if c.g[V0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                        pc = 0x002575fc;
                        continue;
                    }
                    pc = 0x002574bc;
                    continue;
                }
                0x002574bc => {
                    c.cycles += 5;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3ec));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002574cc);
                    c.g[A1][0] = 0u64;
                    callee::BE_NETWORK_INT_UNPACK_PLAYER_DATA.call(c, 0x002574cc)?;
                    c.g[A0][0] = c.g[S0][0];
                    pc = 0x002574d0;
                    continue;
                }
                0x002574d0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002574d8);
                    c.g[A1][0] = 0x5;
                    callee::C_GAME_COMM_SET_STATE.call(c, 0x002574d8)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                    if t {
                        pc = 0x002575fc;
                        continue;
                    }
                    pc = 0x002574e0;
                    continue;
                }
                0x002574e0 => {
                    c.cycles += 3;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3ec));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = 0x1;
                    pc = 0x002574ec;
                    continue;
                }
                0x002574ec => {
                    c.cycles += 11;
                    callee::BE_NETWORK_INT_PACK_PLAYER_DATA.call(c, 0x002574f4)?;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3ec));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00257504);
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3f0));
                    callee::C_GAME_COMM_SET_TIMER_B12C.call(c, 0x00257504)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00257510);
                    c.g[A1][0] = 0x6;
                    callee::C_GAME_COMM_SET_STATE.call(c, 0x00257510)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                    if t {
                        pc = 0x002575fc;
                        continue;
                    }
                    pc = 0x00257518;
                    continue;
                }
                0x00257518 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00257520);
                    c.g[A0][0] = c.g[S0][0];
                    callee::C_GAME_COMM_IS_TIMER_B12C_IDLE.call(c, 0x00257520)?;
                    if c.g[V0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                        pc = 0x002575fc;
                        continue;
                    }
                    pc = 0x00257528;
                    continue;
                }
                0x00257528 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00257534);
                    c.g[A1][0] = 0xb;
                    callee::C_GAME_COMM_SET_STATE.call(c, 0x00257534)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                    if t {
                        pc = 0x002575fc;
                        continue;
                    }
                    pc = 0x0025753c;
                    continue;
                }
                0x0025753c => {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x114));
                    pc = 0x00257544;
                    continue;
                }
                0x00257544 => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x0025754c);
                    c.g[A2][0] = 0x4;
                    callee::C_GAME_COMM_SET_TIMER_A124.call(c, 0x0025754c)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00257558);
                    c.g[A1][0] = 0xc;
                    callee::C_GAME_COMM_SET_STATE.call(c, 0x00257558)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                    if t {
                        pc = 0x002575fc;
                        continue;
                    }
                    pc = 0x00257560;
                    continue;
                }
                0x00257560 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00257568);
                    c.g[A0][0] = c.g[S0][0];
                    callee::C_GAME_COMM_IS_TIMER_A124_IDLE.call(c, 0x00257568)?;
                    if c.g[V0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                        pc = 0x002575fc;
                        continue;
                    }
                    pc = 0x00257570;
                    continue;
                }
                0x00257570 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S0][0];
                    pc = 0x00257574;
                    continue;
                }
                0x00257574 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0025757c);
                    c.g[A1][0] = 0x9;
                    callee::C_GAME_COMM_SET_STATE.call(c, 0x0025757c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                    if t {
                        pc = 0x002575fc;
                        continue;
                    }
                    pc = 0x00257584;
                    continue;
                }
                0x00257584 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    c.g[RA][0] = sx(0x00257590);
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                    callee::C_COMM_SYSTEM_GET_CHANNEL_LATENCY.call(c, 0x00257590)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x118));
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x118));
                    pc = 0x0025759c;
                    continue;
                }
                0x0025759c => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x002575a4);
                    c.g[A2][0] = 0x4;
                    callee::C_GAME_COMM_SET_TIMER_B12C.call(c, 0x002575a4)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x002575b0);
                    c.g[A1][0] = 0xa;
                    callee::C_GAME_COMM_SET_STATE.call(c, 0x002575b0)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                    if t {
                        pc = 0x002575fc;
                        continue;
                    }
                    pc = 0x002575b8;
                    continue;
                }
                0x002575b8 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x002575c0);
                    c.g[A0][0] = c.g[S0][0];
                    callee::C_GAME_COMM_IS_TIMER_B12C_IDLE.call(c, 0x002575c0)?;
                    pc = 0x002575c0;
                    continue;
                }
                0x002575c0 => {
                    c.cycles += 2;
                    if c.g[V0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                        pc = 0x002575fc;
                        continue;
                    }
                    pc = 0x002575c8;
                    continue;
                }
                0x002575c8 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x002575d4);
                    c.g[A1][0] = 0xd;
                    callee::C_GAME_COMM_SET_STATE.call(c, 0x002575d4)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                    if t {
                        pc = 0x002575fc;
                        continue;
                    }
                    pc = 0x002575dc;
                    continue;
                }
                0x002575dc => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x002575e8);
                    c.g[A1][0] = 0xe;
                    callee::C_GAME_COMM_SET_STATE.call(c, 0x002575e8)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                    if t {
                        pc = 0x002575fc;
                        continue;
                    }
                    pc = 0x002575f0;
                    continue;
                }
                0x002575f0 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x002575f8);
                    c.g[A0][0] = c.g[S0][0];
                    callee::C_GAME_COMM_CHECK_PEER_TIMEOUT.call(c, 0x002575f8)?;
                    pc = 0x002575f8;
                    continue;
                }
                0x002575f8 => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                    pc = 0x002575fc;
                    continue;
                }
                0x002575fc => {
                    c.cycles += 5;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x130));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x120));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002570d8, 0x00257610);
                    return Ok(());
                }
            }
        }
    })
}

/// `cGameComm_Create` (0x00256bf8): Allocate and construct the game-comm block, publishing it
/// behind gp. Straight-line.
pub fn c_game_comm_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.g[A3][0] = 0u64;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A2][0] = 0x20000000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.g[A1][0];
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = 0x480000;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A0][0] = 0x5f0;
        c.g[RA][0] = sx(0x00256c28);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4b8));
        callee::EA_MEM_ALLOC_NAMED.call(c, 0x00256c28)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x00256c38);
        c.g[A0][0] = c.g[V0][0];
        callee::C_GAME_COMM_CTOR.call(c, 0x00256c38)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x204), c.g[V0][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    })
}

/// `cGameComm_Destroy` (0x00256c50): Destroy the game-comm block behind gp when present.
/// Straight-line dispatch.
pub fn c_game_comm_destroy(c: &mut Cpu) {
    guest(c, |c| {
        'b_256c6c: {
            c.cycles += 4;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
            let t = c.g[A0][0] == 0u64;
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            if t {
                break 'b_256c6c;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00256c68);
                c.g[A1][0] = 0x3;
                callee::C_GAME_COMM_DTOR.call(c, 0x00256c68)?;
                c.sw((c.g[GP][0] as u32).wrapping_sub(0x204), 0u64);
                break 'b_256c6c;
            }
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    })
}

/// `cGameComm_ctor` (0x00256c78): Construct the game-comm block: state, BE interface, cleared
/// words, packet areas and ping. Two fixed loops with event checks.
pub fn c_game_comm_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 84;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x6c), c.g[A1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x44), c.g[A2][0]);
        c.g[RA][0] = sx(0x00256cb0);
        c.g[A1][0] = 0u64;
        callee::C_GAME_COMM_SET_STATE.call(c, 0x00256cb0)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A1][0] = 0x7;
        c.g[RA][0] = sx(0x00256cc0);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
        callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x00256cc0)?;
        c.g[RA][0] = sx(0x00256cc8);
        c.g[A0][0] = c.g[V0][0];
        callee::BE_NETWORK_INT_GET_SYNC_DATA_SIZE.call(c, 0x00256cc8)?;
        c.g[A0][0] = 0x1;
        c.g[A1][0] = 0xffffffffffffffff;
        c.g[V1][0] = 0x1e;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x3f0), c.g[V0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x60), c.g[V1][0]);
        c.g[A2][0] = 0x3;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x68), c.g[A1][0]);
        c.g[A3][0] = 0x3;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xac), c.g[A0][0]);
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xd0));
        c.sw(c.g[S0][0] as u32, c.g[A0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x3ec), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x40), c.g[A0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x24), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x28), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x5c), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x64), c.g[A1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x7c), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x80), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x84), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x30), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x94), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x88), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8c), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x90), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), c.g[A0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xa4), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xb0), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xa8), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x98), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x50), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x58), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x124), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x12c), 0u64);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x128), 0u64);
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x114));
        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
        c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x118));
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x120), c.g[A2][0]);
        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x11c));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x134), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x74), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x78), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x3f4), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x3f8), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x3fc), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xb4), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xb8), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), 0u64);
        'l_256dc8: loop {
            c.cycles += 7;
            c.sw(c.g[V1][0] as u32, 0u64);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
            let t = (c.g[A3][0] as i64) >= 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00256dc8);
                }
                continue 'l_256dc8;
            } else {
                'b_256e0c: {
                    c.cycles += 10;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x36c));
                    c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x410));
                    c.g[S6][0] = sx((c.g[S0][0] as u32).wrapping_add(0x500));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
                    c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x144));
                    if c.g[V0][0] == 0u64 {
                        break 'b_256e0c;
                    } else {
                        break 'b_256e0c;
                    }
                }
                c.cycles += 9;
                c.g[S3][0] = 0x3;
                c.div(0, 3, 2);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x38), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x3c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x138), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x140), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x13c), 0u64);
                c.g[V1][0] = c.lo[0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0x34), c.g[V1][0]);
                'l_256e30: loop {
                    c.cycles += 10;
                    c.sw(c.g[S1][0] as u32, 0u64);
                    c.g[A0][0] = c.g[S2][0];
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4), 0u64);
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0x80;
                    c.g[RA][0] = sx(0x00256e4c);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x88));
                    callee::EA_MEMSET.call(c, 0x00256e4c)?;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[S3][0] as i64) >= 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x88));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00256e30);
                        }
                        continue 'l_256e30;
                    } else {
                        c.cycles += 33;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x364), 0u64);
                        c.g[A0][0] = c.g[S4][0];
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x368), 0u64);
                        c.g[A1][0] = 0u64;
                        c.g[RA][0] = sx(0x00256e70);
                        c.g[A2][0] = 0x80;
                        callee::EA_MEMSET.call(c, 0x00256e70)?;
                        c.g[V0][0] = 0xffffffffffffffff;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x400), 0u64);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x404), 0u64);
                        c.g[A0][0] = c.g[S5][0];
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x40c), c.g[V0][0]);
                        c.g[A1][0] = 0u64;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x408), c.g[V0][0]);
                        c.g[RA][0] = sx(0x00256e94);
                        c.g[A2][0] = 0xf0;
                        callee::EA_MEMSET.call(c, 0x00256e94)?;
                        c.g[A0][0] = c.g[S6][0];
                        c.g[A1][0] = 0u64;
                        c.g[RA][0] = sx(0x00256ea4);
                        c.g[A2][0] = 0xf0;
                        callee::EA_MEMSET.call(c, 0x00256ea4)?;
                        callee::PROTO_PING_CREATE.call(c, 0x00256eac)?;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x140), c.g[V0][0]);
                        c.g[V0][0] = c.g[S0][0];
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    })
}

/// `cGameComm_dtor` (0x00256ee0): Destroy the game-comm block: free-list packets, ping teardown and
/// conditional delete. Bounded free loop with event checks plus dispatcher.
pub fn c_game_comm_dtor(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00256ee0 => {
                    c.cycles += 12;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S0][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3ec));
                    let t = c.g[A0][0] == 0u64;
                    c.g[S1][0] = c.g[A1][0];
                    if t {
                        pc = 0x00256f18;
                        continue;
                    }
                    pc = 0x00256f10;
                    continue;
                }
                0x00256f10 => {
                    c.cycles += 2;
                    callee::EA_MEM_FREE.call(c, 0x00256f18)?;
                    pc = 0x00256f18;
                    continue;
                }
                0x00256f18 => {
                    c.cycles += 3;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x134));
                    let t = c.g[A1][0] == 0u64;
                    c.g[S5][0] = c.g[S1][0] & 0x1;
                    if t {
                        pc = 0x00256f30;
                        continue;
                    }
                    pc = 0x00256f24;
                    continue;
                }
                0x00256f24 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00256f2c);
                    c.g[A0][0] = c.g[S0][0];
                    callee::C_GAME_COMM_FREE_PACKET.call(c, 0x00256f2c)?;
                    c.g[S5][0] = c.g[S1][0] & 0x1;
                    pc = 0x00256f30;
                    continue;
                }
                0x00256f30 => {
                    c.cycles += 5;
                    c.g[S4][0] = 0x88;
                    c.g[S1][0] = 0u64;
                    c.g[S3][0] = 0xffffffffffffffff;
                    let t = true;
                    c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x00256f8c;
                        continue;
                    }
                    pc = 0x00256f44;
                    continue;
                }
                0x00256f44 => {
                    c.cycles += 1;
                    pc = 0x00256f48;
                    continue;
                }
                0x00256f48 => {
                    c.cycles += 17;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x144));
                    c.g[A2][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = c.g[A2][0];
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1f));
                    c.g[V1][0] = ((c.g[S3][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    if c.g[V1][0] != 0 {
                        c.g[V0][0] = c.g[A1][0];
                    }
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
                    c.sw((c.g[A3][0] as u32).wrapping_add(0x4), c.g[A2][0]);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[RA][0] = sx(0x00256f8c);
                    c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                    callee::C_GAME_COMM_FREE_PACKET.call(c, 0x00256f8c)?;
                    pc = 0x00256f8c;
                    continue;
                }
                0x00256f8c => {
                    c.cycles += 8;
                    c.mult(0, 2, 17, 20);
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x144));
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1f));
                    if t {
                        pc = 0x00256fd0;
                        continue;
                    }
                    pc = 0x00256fac;
                    continue;
                }
                0x00256fac => {
                    c.cycles += 9;
                    c.g[V1][0] = ((c.g[S3][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    if c.g[V1][0] != 0 {
                        c.g[V0][0] = c.g[A0][0];
                    }
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x00256fd4;
                        continue;
                    }
                    pc = 0x00256fd0;
                    continue;
                }
                0x00256fd0 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = 0x00256fd4;
                    continue;
                }
                0x00256fd4 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.mult(0, 2, 17, 20);
                    if t {
                        if c.cycles >= c.next_event {
                            c.pc = 0x00256f48;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x00256f48;
                        continue;
                    }
                    pc = 0x00256fdc;
                    continue;
                }
                0x00256fdc => {
                    c.cycles += 4;
                    c.g[S1][0] = c.g[S2][0];
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 4i64) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        if c.cycles >= c.next_event {
                            c.pc = 0x00256f8c;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x00256f8c;
                        continue;
                    }
                    pc = 0x00256fec;
                    continue;
                }
                0x00256fec => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
                    let t = c.g[A0][0] == 0u64;
                    if t {
                        pc = 0x00257004;
                        continue;
                    }
                    pc = 0x00256ff8;
                    continue;
                }
                0x00256ff8 => {
                    c.cycles += 3;
                    callee::PROTO_PING_DESTROY.call(c, 0x00257000)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x140), 0u64);
                    pc = 0x00257004;
                    continue;
                }
                0x00257004 => {
                    c.cycles += 2;
                    if c.g[S5][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                        pc = 0x00257018;
                        continue;
                    }
                    pc = 0x0025700c;
                    continue;
                }
                0x0025700c => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00257014);
                    c.g[A0][0] = c.g[S0][0];
                    callee::BUILTIN_DELETE.call(c, 0x00257014)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    pc = 0x00257018;
                    continue;
                }
                0x00257018 => {
                    c.cycles += 8;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00256ee0, 0x00257038);
                    return Ok(());
                }
            }
        }
    })
}

/// `cGameComm_SetState` (0x00257038): Publish the connection state through the jump table.
/// Jump-table dispatch.
pub fn c_game_comm_set_state(c: &mut Cpu) {
    guest(c, |c| {
        if c.pc != 0x00257038 {
            return Ok(c_game_comm_set_state_resume(c));
        }
        'b_2570c0: {
            'b_2570bc: {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S1][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[V0][0] = (c.g[V1][0] < 0xfu64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[S0][0] = c.g[A0][0];
                if t {
                    break 'b_2570bc;
                } else {
                    'b_2570ac: {
                        'b_2570a0: {
                            c.cycles += 7;
                            c.g[V0][0] = 0x480000;
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4d0));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                            let t = c.g[A0][0] as u32;
                            match t {
                                0x00257078 => {
                                    c.cycles += 10;
                                    c.g[RA][0] = sx(0x00257080);
                                    c.g[A0][0] = c.g[S0][0];
                                    callee::C_GAME_COMM_UPDATE_MAX_LATENCY.call(c, 0x00257080)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = 0xe10;
                                    c.g[RA][0] = sx(0x00257090);
                                    c.g[A2][0] = 0u64;
                                    callee::C_GAME_COMM_SET_TIMEOUT_TIMER.call(c, 0x00257090)?;
                                    callee::SSX_CLEAR_TRAY_OPENED.call(c, 0x00257098)?;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x70), c.g[S1][0]);
                                    break 'b_2570c0;
                                }
                                0x002570a0 => {
                                    break 'b_2570a0;
                                }
                                0x002570ac => {
                                    break 'b_2570ac;
                                }
                                0x002570bc => {
                                    break 'b_2570bc;
                                }
                                _ => {
                                    c.jump_out(t, 0x00257038, 0x002570d4);
                                    return Ok(());
                                }
                            }
                        }
                        c.cycles += 3;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x84), 0u64);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x78), 0u64);
                        break 'b_2570bc;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x708;
                    c.g[RA][0] = sx(0x002570bc);
                    c.g[A2][0] = 0u64;
                    callee::C_GAME_COMM_SET_TIMEOUT_TIMER.call(c, 0x002570bc)?;
                    break 'b_2570bc;
                }
            }
            c.cycles += 1;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x70), c.g[S1][0]);
            break 'b_2570c0;
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    })
}

/// Native resume of `c_game_comm_set_state` (converted from the `entries` body): any entry
/// other than 0x00257038, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn c_game_comm_set_state_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00257038 => {
                    c.cycles += 9;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[V0][0] = (c.g[V1][0] < 0xfu64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S0][0] = c.g[A0][0];
                    if t {
                        pc = 0x002570bc;
                        continue;
                    }
                    pc = 0x0025705c;
                    continue;
                }
                0x0025705c => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x480000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4d0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x00257078 => {
                    c.cycles += 10;
                    c.g[RA][0] = sx(0x00257080);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00258330,
                        f: ee::s06::cGameComm_UpdateMaxLatency,
                    }
                    .call(c, 0x00257080)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0xe10;
                    c.g[RA][0] = sx(0x00257090);
                    c.g[A2][0] = 0u64;
                    Callee {
                        addr: 0x00258870,
                        f: ee::s06::cGameComm_SetTimeoutTimer,
                    }
                    .call(c, 0x00257090)?;
                    c.g[RA][0] = sx(0x00257098);
                    call_ptr(c, 0x0031ad18, 0x00257098)?;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x70), c.g[S1][0]);
                    if t {
                        pc = 0x002570c0;
                        continue;
                    }
                    pc = 0x002570a0;
                    continue;
                }
                0x002570a0 => {
                    c.cycles += 3;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x84), 0u64);
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x78), 0u64);
                    if t {
                        pc = 0x002570bc;
                        continue;
                    }
                    pc = 0x002570ac;
                    continue;
                }
                0x002570ac => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x708;
                    c.g[RA][0] = sx(0x002570bc);
                    c.g[A2][0] = 0u64;
                    Callee {
                        addr: 0x00258870,
                        f: ee::s06::cGameComm_SetTimeoutTimer,
                    }
                    .call(c, 0x002570bc)?;
                    pc = 0x002570bc;
                    continue;
                }
                0x002570bc => {
                    c.cycles += 1;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x70), c.g[S1][0]);
                    pc = 0x002570c0;
                    continue;
                }
                0x002570c0 => {
                    c.cycles += 5;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00257038, 0x002570d4);
                    return Ok(());
                }
            }
        }
    });
}

/// `cGameComm_ResetRaceSync` (0x00257610): Reset the race-sync words when the guards pass.
/// Straight-line dispatch.
pub fn c_game_comm_reset_race_sync(c: &mut Cpu) {
    guest(c, |c| {
        'b_257664: {
            'b_257634: {
                c.cycles += 2;
                let t = c.g[A1][0] != 0u64;
                c.g[A2][0] = c.g[A0][0];
                if t {
                    break 'b_257634;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x1;
                    if t {
                        break 'b_257664;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x70));
                        c.g[V0][0] = ((c.g[V0][0] as i64) < 15i64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = 0x1;
                        if t {
                            break 'b_257664;
                        } else {
                            break 'b_257634;
                        }
                    }
                }
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = 0x1;
            if t {
                break 'b_257664;
            } else {
                'b_257654: {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                    if c.g[V1][0] == 0u64 {
                        break 'b_257654;
                    } else {
                        break 'b_257654;
                    }
                }
                c.cycles += 4;
                c.div(0, 2, 3);
                c.g[V0][0] = c.lo[0];
                c.sw((c.g[A2][0] as u32).wrapping_add(0x38), c.g[V0][0]);
                c.g[V0][0] = 0x1;
                break 'b_257664;
            }
        }
        c.cycles += 5;
        c.sw(c.g[A2][0] as u32, 0u64);
        c.sw((c.g[A2][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.sw((c.g[A2][0] as u32).wrapping_add(0x80), 0u64);
        let t = c.g[RA][0] as u32;
        c.sw((c.g[A2][0] as u32).wrapping_add(0x78), 0u64);
        c.pc = t;
        return Ok(());
    })
}

/// `cGameComm_LatchResultFlags` (0x00257678): Fold the result words into the latched flags.
/// Straight-line dispatch.
pub fn c_game_comm_latch_result_flags(c: &mut Cpu) {
    guest(c, |c| {
        'b_257744: {
            c.cycles += 4;
            c.g[T0][0] = c.g[A0][0];
            c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x30));
            let t = c.g[V0][0] != 0u64;
            c.g[A1][0] = 0x1;
            if t {
                break 'b_257744;
            } else {
                'b_2576bc: {
                    'b_2576a8: {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xa0));
                        c.sw((c.g[T0][0] as u32).wrapping_add(0x30), c.g[A1][0]);
                        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x18));
                        let t = c.g[V0][0] == 0u64;
                        c.g[V1][0] = 0u64;
                        if t {
                            break 'b_2576a8;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xa4));
                            if c.g[V0][0] != 0u64 {
                                c.sw(c.g[A0][0] as u32, c.g[V1][0]);
                                break 'b_2576bc;
                            } else {
                                break 'b_2576a8;
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x70));
                    c.g[V1][0] = 0u64;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 15i64) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.g[A1][0];
                    }
                    c.sw(c.g[A0][0] as u32, c.g[V1][0]);
                    break 'b_2576bc;
                }
                c.cycles += 34;
                c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xa4));
                c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xa0));
                c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
                c.g[A1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x14));
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x7c));
                c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32) << 3);
                c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
                c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x80));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                c.g[A1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x48));
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32) << 5);
                c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x88));
                c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32) << 6);
                c.g[A3][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8c));
                c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32) << 7);
                c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x90));
                c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
                c.g[A3][0] = sx((c.g[A3][0] as u32) << 8);
                c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xac));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 9);
                c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32) << 10);
                c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x70));
                c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                c.sw((c.g[T0][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                c.sw((c.g[T0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
                break 'b_257744;
            }
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.pc = t;
        return Ok(());
    })
}

/// `cGameComm_TickCountdowns` (0x00257750): Tick the countdown and timer words down toward zero.
/// Straight-line dispatch.
pub fn c_game_comm_tick_countdowns(c: &mut Cpu) {
    guest(c, |c| {
        'b_2577bc: {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
            let t = c.g[V0][0] != 0u64;
            if t {
                break 'b_2577bc;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x48));
                let t = c.g[V0][0] != 0u64;
                if t {
                    break 'b_2577bc;
                } else {
                    'b_257784: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                        if c.g[V0][0] == 0u64 {
                            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x64));
                            break 'b_257784;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x80));
                            let t = c.g[V0][0] != 0u64;
                            if t {
                                break 'b_2577bc;
                            } else {
                                c.cycles += 1;
                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x64));
                                break 'b_257784;
                            }
                        }
                    }
                    'b_2577b0: {
                        c.cycles += 2;
                        if (c.g[V1][0] as i64) <= 0 {
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x68));
                            break 'b_2577b0;
                        } else {
                            'b_2577a8: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x74));
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                if t {
                                    break 'b_2577a8;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x78));
                                    if c.g[V0][0] != 0u64 {
                                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x68));
                                        break 'b_2577b0;
                                    } else {
                                        c.cycles += 1;
                                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                        break 'b_2577a8;
                                    }
                                }
                            }
                            c.cycles += 2;
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x64), c.g[V0][0]);
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x68));
                            break 'b_2577b0;
                        }
                    }
                    c.cycles += 2;
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    if t {
                        break 'b_2577bc;
                    } else {
                        c.cycles += 1;
                        c.sw((c.g[A0][0] as u32).wrapping_add(0x68), c.g[V0][0]);
                        break 'b_2577bc;
                    }
                }
            }
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.pc = t;
        return Ok(());
    })
}

/// `cGameComm_CheckConnection` (0x002577c8): Check the connection state: quit countdown, channel
/// status and the timeout/countdown guards, ending the session when down. Straight-line dispatch.
pub fn c_game_comm_check_connection(c: &mut Cpu) {
    guest(c, |c| {
        'b_2578e8: {
            'b_2577f4: {
                c.cycles += 7;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[S0][0] = c.g[A0][0];
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                if (c.g[V0][0] as i64) <= 0 {
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    break 'b_2577f4;
                } else {
                    c.cycles += 4;
                    callee::C_GAME_COMM_TICK_QUIT_COUNTDOWN.call(c, 0x002577ec)?;
                    c.g[V0][0] = 0x1;
                    break 'b_2578e8;
                }
            }
            c.cycles += 2;
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = 0x1;
            if t {
                break 'b_2578e8;
            } else {
                'b_2578e4: {
                    'b_2578d4: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8c));
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = c.g[S0][0];
                        if t {
                            break 'b_2578d4;
                        } else {
                            'b_2578c8: {
                                c.cycles += 5;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                                c.g[RA][0] = sx(0x00257814);
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                                callee::C_COMM_SYSTEM_GET_CHANNEL_STATUS.call(c, 0x00257814)?;
                                let t = c.g[V0][0] != 0u64;
                                c.g[A0][0] = c.g[S0][0];
                                if t {
                                    break 'b_2578c8;
                                } else {
                                    'b_25783c: {
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
                                        c.g[V0][0] = ((c.g[V0][0] as i64) < 3i64) as u64;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                                            break 'b_25783c;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] = 0x3;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x24),
                                                c.g[V0][0],
                                            );
                                            break 'b_2578d4;
                                        }
                                    }
                                    'b_257858: {
                                        c.cycles += 3;
                                        c.g[V0][0] = ((c.g[V0][0] as i64) < 6i64) as u64;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                                            break 'b_257858;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] = 0x2;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x24),
                                                c.g[V0][0],
                                            );
                                            break 'b_2578d4;
                                        }
                                    }
                                    'b_257874: {
                                        c.cycles += 3;
                                        c.g[V0][0] = ((c.g[V0][0] as i64) < 3i64) as u64;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                                            break 'b_257874;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] = 0x3;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x28),
                                                c.g[V0][0],
                                            );
                                            break 'b_2578d4;
                                        }
                                    }
                                    'b_257890: {
                                        c.cycles += 3;
                                        c.g[V0][0] = ((c.g[V0][0] as i64) < 6i64) as u64;
                                        if c.g[V0][0] != 0u64 {
                                            c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0xb0));
                                            break 'b_257890;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] = 0x2;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x28),
                                                c.g[V0][0],
                                            );
                                            break 'b_2578d4;
                                        }
                                    }
                                    c.cycles += 6;
                                    c.g[AT][0] = 0x45610000;
                                    c.mtc1(1, 0);
                                    c.c_le_s(0, 1);
                                    let t = c.fcc();
                                    c.g[A0][0] = c.g[S0][0];
                                    if t {
                                        break 'b_2578c8;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x94));
                                        let t = (c.g[V0][0] as i64) <= 0;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                        if t {
                                            break 'b_2578e4;
                                        } else {
                                            c.cycles += 2;
                                            let t = (c.g[V0][0] as i64) > 0;
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x94),
                                                c.g[V0][0],
                                            );
                                            if t {
                                                break 'b_2578e4;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x1;
                                                c.g[A0][0] = c.g[S0][0];
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0x2c),
                                                    c.g[V0][0],
                                                );
                                                break 'b_2578c8;
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 3;
                            callee::C_GAME_COMM_LATCH_RESULT_FLAGS.call(c, 0x002578d0)?;
                            c.g[A0][0] = c.g[S0][0];
                            break 'b_2578d4;
                        }
                    }
                    c.cycles += 4;
                    callee::C_GAME_COMM_END_SESSION.call(c, 0x002578dc)?;
                    c.g[V0][0] = 0x1;
                    break 'b_2578e8;
                }
                c.cycles += 1;
                c.g[V0][0] = 0u64;
                break 'b_2578e8;
            }
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    })
}

/// `cGameComm_EndSession` (0x002578f8): Tear down the session: voip stop, wait popup and the quit
/// countdown. Straight-line dispatch.
pub fn c_game_comm_end_session(c: &mut Cpu) {
    guest(c, |c| {
        'b_2579b4: {
            c.cycles += 10;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x84), 0u64);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x78), 0u64);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x3f8), 0u64);
            let t = c.g[V0][0] != 0u64;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x3fc), 0u64);
            if t {
                break 'b_2579b4;
            } else {
                'b_257980: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    if c.g[V0][0] != 0u64 {
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
                        break 'b_257980;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                        if c.g[V0][0] != 0u64 {
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
                            break 'b_257980;
                        } else {
                            'b_257954: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
                                if c.g[V0][0] == 0u64 {
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                                    break 'b_257954;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x88));
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
                                    if t {
                                        break 'b_257980;
                                    } else {
                                        c.cycles += 1;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                                        break 'b_257954;
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[V0][0] != 0u64;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
                            if t {
                                break 'b_257980;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                if t {
                                    break 'b_257980;
                                } else {
                                    'b_25797c: {
                                        c.cycles += 4;
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x224));
                                        let t = c.g[A0][0] == 0u64;
                                        c.g[V0][0] = 0x4;
                                        if t {
                                            break 'b_25797c;
                                        } else {
                                            c.cycles += 1;
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x24),
                                                c.g[V0][0],
                                            );
                                            break 'b_25797c;
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
                                    break 'b_257980;
                                }
                            }
                        }
                    }
                }
                'b_257990: {
                    c.cycles += 2;
                    let t = c.g[A0][0] == 0u64;
                    if t {
                        break 'b_257990;
                    } else {
                        c.cycles += 2;
                        callee::C_VOIP_MAN_STOP.call(c, 0x00257990)?;
                        break 'b_257990;
                    }
                }
                c.cycles += 9;
                c.g[RA][0] = sx(0x00257998);
                c.g[A0][0] = c.g[S0][0];
                callee::C_GAME_COMM_UPDATE_WAIT_POPUP.call(c, 0x00257998)?;
                c.g[V0][0] = 0x1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x40), 0u64);
                c.sw(c.g[S0][0] as u32, 0u64);
                c.g[A0][0] = c.g[S0][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[RA][0] = sx(0x002579b4);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                callee::C_GAME_COMM_TICK_QUIT_COUNTDOWN.call(c, 0x002579b4)?;
                break 'b_2579b4;
            }
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    })
}

/// `cGameComm_TickQuitCountdown` (0x002579c8): Tick the quit countdown down, opening the quit
/// screen at zero. Straight-line dispatch.
pub fn c_game_comm_tick_quit_countdown(c: &mut Cpu) {
    guest(c, |c| {
        'b_257a74: {
            c.cycles += 7;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xac));
            if c.g[V0][0] != 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_257a74;
            } else {
                'b_257a70: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        break 'b_257a70;
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x48));
                        let t = c.g[A0][0] == 0u64;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                        if t {
                            break 'b_257a70;
                        } else {
                            c.cycles += 2;
                            if c.g[A0][0] == 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                                break 'b_257a74;
                            } else {
                                'b_257a28: {
                                    c.cycles += 4;
                                    callee::UI_STATE_MGR_GET_ACTIVE_STATE.call(c, 0x00257a10)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V1][0] = 0x5;
                                    if t {
                                        break 'b_257a28;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x1d));
                                        c.g[V0][0] = c.g[V0][0] & 0x3f;
                                        if c.g[V0][0] != c.g[V1][0] {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                                            break 'b_257a74;
                                        } else {
                                            break 'b_257a28;
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                let t = (c.g[V0][0] as i64) > 0;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                if t {
                                    break 'b_257a70;
                                } else {
                                    c.cycles += 14;
                                    c.g[RA][0] = sx(0x00257a40);
                                    c.g[A0][0] = 0x27;
                                    callee::GAME_UI_OPEN_SCREEN.call(c, 0x00257a40)?;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
                                    c.g[A1][0] = 0x440000;
                                    c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2924));
                                    c.g[V1][0] = 0x530000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x66e8));
                                    c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
                                    c.g[A1][0] = 0xd;
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[A2][0]);
                                    c.sw(c.g[V0][0] as u32, c.g[A1][0]);
                                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[A0][0]);
                                    break 'b_257a70;
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_257a74;
            }
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    })
}

/// `cGameComm_PopPacket` (0x00257bd8): Pop one packet off the receive ring into the caller buffer.
/// Straight-line dispatch with a mult.
pub fn c_game_comm_pop_packet(c: &mut Cpu) {
    guest(c, |c| {
        'b_257cb8: {
            'b_257cb4: {
                c.cycles += 11;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S0][0] = c.g[A2][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S1][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                let t = c.g[V0][0] != 0u64;
                c.g[S3][0] = c.g[A1][0];
                if t {
                    break 'b_257cb4;
                } else {
                    c.cycles += 10;
                    c.g[RA][0] = sx(0x00257c0c);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00257cd8, 0x00257c0c)?;
                    c.g[V0][0] = 0x88;
                    c.mult(0, 2, 19, 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x144));
                    c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                    let t = c.g[A1][0] == c.g[V0][0];
                    c.g[V1][0] = 0xffffffffffffffff;
                    if t {
                        break 'b_257cb4;
                    } else {
                        'b_257c98: {
                            c.cycles += 22;
                            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1f));
                            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                            if c.g[V1][0] != 0 {
                                c.g[V0][0] = c.g[A1][0];
                            }
                            c.sw((c.g[A2][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
                            c.g[A0][0] = c.g[S0][0];
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
                            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x2));
                            c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4));
                            c.g[RA][0] = sx(0x00257c70);
                            c.g[A2][0] = c.g[S2][0];
                            callee::EA_MEM_COPY.call(c, 0x00257c70)?;
                            c.g[A1][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x00257c7c);
                            c.g[A0][0] = c.g[S1][0];
                            callee::C_GAME_COMM_FREE_PACKET.call(c, 0x00257c7c)?;
                            if c.g[S3][0] != 0u64 {
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x94));
                                break 'b_257c98;
                            } else {
                                c.cycles += 5;
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x5c), 0u64);
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x60), 0u64);
                                c.sw((c.g[S1][0] as u32).wrapping_add(0xa8), 0u64);
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x84), 0u64);
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x94));
                                break 'b_257c98;
                            }
                        }
                        'b_257cac: {
                            c.cycles += 2;
                            let t = (c.g[V0][0] as i64) <= 0;
                            c.g[A0][0] = c.g[S1][0];
                            if t {
                                break 'b_257cac;
                            } else {
                                c.cycles += 3;
                                c.g[A1][0] = 0x708;
                                c.g[RA][0] = sx(0x00257cac);
                                c.g[A2][0] = 0u64;
                                callee::C_GAME_COMM_SET_TIMEOUT_TIMER.call(c, 0x00257cac)?;
                                break 'b_257cac;
                            }
                        }
                        c.cycles += 2;
                        c.g[V0][0] = c.g[S2][0];
                        break 'b_257cb8;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_257cb8;
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    })
}

/// `cGameComm_ReceivePackets` (0x00257cd8): Drain the channel into the receive ring. Bounded drain
/// loop with event checks.
pub fn c_game_comm_receive_packets(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00257cd8 => {
                    c.cycles += 9;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S0][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S1][0] = 0xffffffffffffffff;
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    let t = true;
                    c.g[S2][0] = 0x88;
                    if t {
                        pc = 0x00257d58;
                        continue;
                    }
                    pc = 0x00257cfc;
                    continue;
                }
                0x00257cfc => {
                    c.cycles += 1;
                    pc = 0x00257d00;
                    continue;
                }
                0x00257d00 => {
                    c.cycles += 11;
                    c.g[V0][0] = c.lbu(c.g[A3][0] as u32);
                    c.mult(0, 2, 2, 18);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x144));
                    c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A1][0] = c.lw(c.g[A2][0] as u32);
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 31i64) as u64;
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    if t {
                        pc = 0x00257da4;
                        continue;
                    }
                    pc = 0x00257d2c;
                    continue;
                }
                0x00257d2c => {
                    c.cycles += 11;
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1f));
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.g[A1][0];
                    }
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> 5) as u32);
                    c.sw(c.g[A2][0] as u32, c.g[A0][0]);
                    c.g[V1][0] = sx((c.g[V0][0] as u32) << 5);
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x134), 0u64);
                    pc = 0x00257d58;
                    continue;
                }
                0x00257d58 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    if c.g[V0][0] != 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        pc = 0x00257da8;
                        continue;
                    }
                    pc = 0x00257d64;
                    continue;
                }
                0x00257d64 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x134));
                    if c.g[V0][0] != 0u64 {
                        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x134));
                        if c.cycles >= c.next_event {
                            c.pc = 0x00257d00;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x00257d00;
                        continue;
                    }
                    pc = 0x00257d70;
                    continue;
                }
                0x00257d70 => {
                    c.cycles += 10;
                    c.g[RA][0] = sx(0x00257d78);
                    c.g[A0][0] = c.g[S0][0];
                    callee::C_GAME_COMM_ALLOC_PACKET.call(c, 0x00257d78)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                    c.g[A3][0] = 0xf0;
                    c.g[RA][0] = sx(0x00257d90);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x134), c.g[V0][0]);
                    callee::C_COMM_SYSTEM_CHANNEL_RECV.call(c, 0x00257d90)?;
                    if c.g[V0][0] != 0u64 {
                        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x134));
                        if c.cycles >= c.next_event {
                            c.pc = 0x00257d00;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x00257d00;
                        continue;
                    }
                    pc = 0x00257d98;
                    continue;
                }
                0x00257d98 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00257da0);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x134));
                    callee::BUILTIN_DELETE.call(c, 0x00257da0)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x134), 0u64);
                    pc = 0x00257da4;
                    continue;
                }
                0x00257da4 => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    pc = 0x00257da8;
                    continue;
                }
                0x00257da8 => {
                    c.cycles += 5;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00257cd8, 0x00257dbc);
                    return Ok(());
                }
            }
        }
    })
}

/// `cGameComm_FlushSendQueue` (0x00257dc0): Send one queued packet, then free it. Single-attempt
/// loop with an event check.
pub fn c_game_comm_flush_send_queue(c: &mut Cpu) {
    guest(c, |c| {
        'b_257e80: {
            'b_257e7c: {
                c.cycles += 11;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S1][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x368));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x364));
                let t = c.g[V1][0] == c.g[V0][0];
                c.g[S3][0] = 0xffffffffffffffff;
                if t {
                    break 'b_257e7c;
                } else {
                    c.cycles += 3;
                    c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x36c));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    'l_257df8: loop {
                        c.cycles += 2;
                        let t = c.g[V0][0] != 0u64;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                        if t {
                            break 'b_257e80;
                        } else {
                            'b_257e3c: {
                                'b_257e38: {
                                    c.cycles += 5;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x368));
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x364));
                                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1f));
                                    if t {
                                        break 'b_257e38;
                                    } else {
                                        c.cycles += 9;
                                        c.g[V1][0] =
                                            ((c.g[S3][0] as i64) < (c.g[A0][0] as i64)) as u64;
                                        if c.g[V1][0] != 0 {
                                            c.g[V0][0] = c.g[A0][0];
                                        }
                                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
                                        c.g[V0][0] =
                                            sx((c.g[A0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                        c.g[V0][0] =
                                            sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.g[S0][0] = c.lw(c.g[V0][0] as u32);
                                        break 'b_257e3c;
                                    }
                                }
                                c.cycles += 1;
                                c.g[S0][0] = 0u64;
                                break 'b_257e3c;
                            }
                            c.cycles += 7;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                            c.g[A2][0] = c.g[S0][0];
                            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
                            c.g[RA][0] = sx(0x00257e50);
                            c.g[A3][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x2));
                            callee::C_COMM_SYSTEM_CHANNEL_SEND.call(c, 0x00257e50)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = c.g[S0][0];
                            if t {
                                break 'b_257e7c;
                            } else {
                                c.cycles += 9;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x368));
                                c.g[A0][0] = c.g[S1][0];
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                c.g[RA][0] = sx(0x00257e6c);
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x368), c.g[V0][0]);
                                callee::C_GAME_COMM_FREE_PACKET.call(c, 0x00257e6c)?;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x368));
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x364));
                                if c.g[V1][0] != c.g[V0][0] {
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00257df8);
                                    }
                                    continue 'l_257df8;
                                } else {
                                    break 'b_257e7c;
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_257e80;
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    })
}

/// `cGameComm_SendPacket` (0x00257e98): Queue one packet for sending through the channel.
/// Straight-line dispatch.
pub fn c_game_comm_send_packet(c: &mut Cpu) {
    guest(c, |c| {
        'b_257f78: {
            'b_257ed0: {
                c.cycles += 12;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x140));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.g[S0][0] = c.g[A3][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.g[S1][0] = c.g[A0][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.g[S2][0] = c.g[A1][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0xf0), c.g[RA][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                let t = c.g[V0][0] == 0u64;
                c.g[S3][0] = c.g[A2][0];
                if t {
                    break 'b_257ed0;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0u64;
                    break 'b_257f78;
                }
            }
            'b_257f20: {
                c.cycles += 14;
                c.g[RA][0] = sx(0x00257ed8);
                c.g[A0][0] = c.g[S1][0];
                callee::C_GAME_COMM_FLUSH_SEND_QUEUE.call(c, 0x00257ed8)?;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                c.sb(c.g[SP][0] as u32, c.g[S2][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x2), c.g[V0][0]);
                c.g[A1][0] = c.g[S3][0];
                c.g[A2][0] = c.g[S0][0];
                c.sb((c.g[SP][0] as u32).wrapping_add(0x1), 0u64);
                c.g[RA][0] = sx(0x00257ef8);
                c.g[A0][0] = c.g[SP][0] | 0x4;
                callee::EA_MEM_COPY.call(c, 0x00257ef8)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x368));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x364));
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[A3][0] = c.lhu((c.g[SP][0] as u32).wrapping_add(0x2));
                if t {
                    break 'b_257f20;
                } else {
                    c.cycles += 6;
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    c.g[RA][0] = sx(0x00257f18);
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
                    callee::C_COMM_SYSTEM_CHANNEL_SEND.call(c, 0x00257f18)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x1;
                    if t {
                        break 'b_257f78;
                    } else {
                        break 'b_257f20;
                    }
                }
            }
            c.cycles += 22;
            c.g[RA][0] = sx(0x00257f28);
            c.g[A0][0] = c.g[S1][0];
            callee::C_GAME_COMM_ALLOC_PACKET.call(c, 0x00257f28)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A2][0] = c.lhu((c.g[SP][0] as u32).wrapping_add(0x2));
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x00257f3c);
            c.g[A1][0] = c.g[SP][0];
            callee::EA_MEM_COPY.call(c, 0x00257f3c)?;
            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x364));
            c.g[A0][0] = 0xffffffffffffffff;
            c.g[V0][0] = 0x1;
            c.g[A1][0] = c.g[A2][0];
            c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1f));
            if c.g[A0][0] != 0 {
                c.g[V1][0] = c.g[A1][0];
            }
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 5) as u32);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x364), c.g[A2][0]);
            c.g[A0][0] = sx((c.g[V1][0] as u32) << 5);
            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x36c), c.g[S0][0]);
            break 'b_257f78;
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xf0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
        c.pc = t;
        return Ok(());
    })
}

/// `cGameComm_ProcessControlPackets` (0x00257f98): Drain control packets through the jump table.
/// Bounded drain loop with an event check.
pub fn c_game_comm_process_control_packets(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00257f98 => {
                    c.cycles += 5;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    let t = true;
                    c.g[S0][0] = c.g[A0][0];
                    if t {
                        pc = 0x00258130;
                        continue;
                    }
                    pc = 0x00257fac;
                    continue;
                }
                0x00257fac => {
                    c.cycles += 1;
                    pc = 0x00257fb0;
                    continue;
                }
                0x00257fb0 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V1][0] < 0x6u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x00258130;
                        continue;
                    }
                    pc = 0x00257fbc;
                    continue;
                }
                0x00257fbc => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x590));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;
                    pc = t;
                    continue;
                }
                0x00257fd4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x00257ff8;
                        continue;
                    }
                    pc = 0x00257fe0;
                    continue;
                }
                0x00257fe0 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0xe4c;
                    c.g[RA][0] = sx(0x00257ff0);
                    c.g[A2][0] = 0x1;
                    callee::C_GAME_COMM_SET_TIMEOUT_TIMER.call(c, 0x00257ff0)?;
                    let t = true;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                    if t {
                        pc = 0x00258000;
                        continue;
                    }
                    pc = 0x00257ff8;
                    continue;
                }
                0x00257ff8 => {
                    c.cycles += 2;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x3f8), c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                    pc = 0x00258000;
                    continue;
                }
                0x00258000 => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x1;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x5c), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x50), c.g[V1][0]);
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x3fc), 0u64);
                    if t {
                        pc = 0x00258130;
                        continue;
                    }
                    pc = 0x0025801c;
                    continue;
                }
                0x0025801c => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x5c), 0u64);
                    let t = c.g[V0][0] == 0u64;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x78), 0u64);
                    if t {
                        pc = 0x0025803c;
                        continue;
                    }
                    pc = 0x0025802c;
                    continue;
                }
                0x0025802c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[V0][0] = 0x4b0;
                    if t {
                        pc = 0x0025803c;
                        continue;
                    }
                    pc = 0x00258038;
                    continue;
                }
                0x00258038 => {
                    c.cycles += 1;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x64), c.g[V0][0]);
                    pc = 0x0025803c;
                    continue;
                }
                0x0025803c => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x3f8), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x60), 0u64);
                    let t = c.g[V0][0] != 0u64;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x84), 0u64);
                    if t {
                        pc = 0x00258068;
                        continue;
                    }
                    pc = 0x00258050;
                    continue;
                }
                0x00258050 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x708;
                    c.g[RA][0] = sx(0x00258060);
                    c.g[A2][0] = 0u64;
                    callee::C_GAME_COMM_SET_TIMEOUT_TIMER.call(c, 0x00258060)?;
                    let t = true;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        pc = 0x00258134;
                        continue;
                    }
                    pc = 0x00258068;
                    continue;
                }
                0x00258068 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00258070);
                    c.g[A0][0] = c.g[S0][0];
                    callee::C_GAME_COMM_UPDATE_IDLE_KICK_TIMER.call(c, 0x00258070)?;
                    let t = true;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        pc = 0x00258134;
                        continue;
                    }
                    pc = 0x00258078;
                    continue;
                }
                0x00258078 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x3f4), c.g[V0][0]);
                    if t {
                        pc = 0x00258130;
                        continue;
                    }
                    pc = 0x00258084;
                    continue;
                }
                0x00258084 => {
                    c.cycles += 4;
                    c.g[A1][0] = 0x1;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3c));
                    let t = c.g[V0][0] != 0u64;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x88), c.g[A1][0]);
                    if t {
                        pc = 0x002580b4;
                        continue;
                    }
                    pc = 0x00258094;
                    continue;
                }
                0x00258094 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                    if c.g[V1][0] == 0u64 {
                        pc = 0x002580a8;
                        continue;
                    }
                    pc = 0x002580a8;
                    continue;
                }
                0x002580a8 => {
                    c.cycles += 3;
                    c.div(0, 2, 3);
                    c.g[V0][0] = c.lo[0];
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
                    pc = 0x002580b4;
                    continue;
                }
                0x002580b4 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x78), 0u64);
                    c.sw(c.g[S0][0] as u32, 0u64);
                    let t = c.g[V0][0] != 0u64;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[A1][0]);
                    if t {
                        pc = 0x00258130;
                        continue;
                    }
                    pc = 0x002580c8;
                    continue;
                }
                0x002580c8 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 15i64) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x8c), c.g[A1][0]);
                        pc = 0x00258130;
                        continue;
                    }
                    pc = 0x002580d8;
                    continue;
                }
                0x002580d8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        pc = 0x00258134;
                        continue;
                    }
                    pc = 0x002580e0;
                    continue;
                }
                0x002580e0 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    if t {
                        pc = 0x00258130;
                        continue;
                    }
                    pc = 0x002580f0;
                    continue;
                }
                0x002580f0 => {
                    c.cycles += 9;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = 0x3c;
                    c.g[A0][0] = 0x1;
                    c.g[A1][0] = 0x3c;
                    c.div(0, 2, 3);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x404), c.g[A0][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x40c), c.g[V0][0]);
                    if c.g[V1][0] == 0u64 {
                        pc = 0x00258114;
                        continue;
                    }
                    pc = 0x00258114;
                    continue;
                }
                0x00258114 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = c.lo[0];
                    c.div(0, 5, 3);
                    c.g[V0][0] = c.hi[0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x500), c.g[A0][0]);
                    pc = 0x00258130;
                    continue;
                }
                0x00258130 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S0][0];
                    pc = 0x00258134;
                    continue;
                }
                0x00258134 => {
                    c.cycles += 6;
                    c.g[A1][0] = 0x3;
                    c.g[A2][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x00258144);
                    c.g[A3][0] = 0xc;
                    callee::C_GAME_COMM_POP_PACKET.call(c, 0x00258144)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = c.lbu(c.g[SP][0] as u32);
                    if t {
                        if c.cycles >= c.next_event {
                            c.pc = 0x00257fb0;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x00257fb0;
                        continue;
                    }
                    pc = 0x0025814c;
                    continue;
                }
                0x0025814c => {
                    c.cycles += 4;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00257f98, 0x0025815c);
                    return Ok(());
                }
            }
        }
    })
}

/// `cGameComm_RecvGamePacket` (0x00258160): Pop one game packet and report whether one arrived.
/// Straight-line.
pub fn c_game_comm_recv_game_packet(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[A3][0] = c.g[A2][0];
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[A2][0] = c.g[A1][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x00258178);
        c.g[A1][0] = 0x1;
        callee::C_GAME_COMM_POP_PACKET.call(c, 0x00258178)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.g[V0][0] = ((0u64 as i64) < (c.g[V0][0] as i64)) as u64;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    })
}

/// `cGameComm_AllocPacket` (0x00258188): Allocate a packet block on the game heap. Straight-line.
pub fn c_game_comm_alloc_packet(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[A1][0] = 0x480000;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A0][0] = 0xf0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5a8));
        c.g[A2][0] = 0x20000000;
        c.g[RA][0] = sx(0x002581a8);
        c.g[A3][0] = 0u64;
        callee::EA_MEM_ALLOC_NAMED.call(c, 0x002581a8)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    })
}

/// `cGameComm_FreePacket` (0x002581b8): Free the packet block unless null. Straight-line.
pub fn c_game_comm_free_packet(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x002581c8);
        c.g[A0][0] = c.g[A1][0];
        callee::BUILTIN_DELETE.call(c, 0x002581c8)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    })
}

/// `cGameComm_IsTimerA124Idle` (0x002581d8): Report whether timer A124 has expired. Straight-line.
pub fn c_game_comm_is_timer_a124_idle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x124));
        let t = c.g[RA][0] as u32;
        c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
        c.pc = t;
        return Ok(());
    })
}

/// `cGameComm_SetTimerA124` (0x002581e8): Arm timer A124. Straight-line.
pub fn c_game_comm_set_timer_a124(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x128), c.g[A2][0]);
        let t = c.g[RA][0] as u32;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x124), c.g[A1][0]);
        c.pc = t;
        return Ok(());
    })
}

/// `cGameComm_RecvBulkData` (0x002581f8): Pull bulk data through the packet ring. Bounded pull loop
/// with an event check.
pub fn c_game_comm_recv_bulk_data(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x002581f8 => {
                    c.cycles += 8;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S2][0] = 0xec;
                    let t = true;
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    if t {
                        pc = 0x00258238;
                        continue;
                    }
                    pc = 0x00258218;
                    continue;
                }
                0x00258218 => {
                    c.cycles += 6;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x128));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x124), c.g[V0][0]);
                    let t = (c.g[V1][0] as i64) > 0;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x128), c.g[V1][0]);
                    if t {
                        pc = 0x00258238;
                        continue;
                    }
                    pc = 0x00258230;
                    continue;
                }
                0x00258230 => {
                    c.cycles += 2;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x124), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x128), 0u64);
                    pc = 0x00258238;
                    continue;
                }
                0x00258238 => {
                    c.cycles += 3;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x124));
                    let t = c.g[A2][0] == 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        pc = 0x00258264;
                        continue;
                    }
                    pc = 0x00258244;
                    continue;
                }
                0x00258244 => {
                    c.cycles += 8;
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x128));
                    c.g[A1][0] = 0x2;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 237i64) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[S1][0] = c.g[S2][0];
                    }
                    c.g[RA][0] = sx(0x0025825c);
                    c.g[A3][0] = c.g[S1][0];
                    callee::C_GAME_COMM_POP_PACKET.call(c, 0x0025825c)?;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x124));
                        if c.cycles >= c.next_event {
                            c.pc = 0x00258218;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x00258218;
                        continue;
                    }
                    pc = 0x00258264;
                    continue;
                }
                0x00258264 => {
                    c.cycles += 6;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002581f8, 0x0025827c);
                    return Ok(());
                }
            }
        }
    })
}

/// `cGameComm_IsTimerB12CIdle` (0x00258280): Report whether timer B12C has expired. Straight-line.
pub fn c_game_comm_is_timer_b12c_idle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x12c));
        let t = c.g[RA][0] as u32;
        c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
        c.pc = t;
        return Ok(());
    })
}

/// `cGameComm_SetTimerB12C` (0x00258290): Arm timer B12C. Straight-line.
pub fn c_game_comm_set_timer_b12c(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x130), c.g[A2][0]);
        let t = c.g[RA][0] as u32;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x12c), c.g[A1][0]);
        c.pc = t;
        return Ok(());
    })
}

/// `cGameComm_SendBulkData` (0x002582a0): Queue one bulk chunk for sending. Straight-line dispatch.
pub fn c_game_comm_send_bulk_data(c: &mut Cpu) {
    guest(c, |c| {
        'b_25831c: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x12c));
            if c.g[A2][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_25831c;
            } else {
                c.cycles += 4;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x368));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x364));
                if c.g[V1][0] != c.g[V0][0] {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    break 'b_25831c;
                } else {
                    c.cycles += 9;
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x130));
                    c.g[V1][0] = 0xec;
                    c.g[A1][0] = 0x2;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 237i64) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[S1][0] = c.g[V1][0];
                    }
                    c.g[RA][0] = sx(0x002582ec);
                    c.g[A3][0] = c.g[S1][0];
                    callee::C_GAME_COMM_SEND_PACKET.call(c, 0x002582ec)?;
                    if c.g[V0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        break 'b_25831c;
                    } else {
                        'b_258318: {
                            c.cycles += 7;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x12c));
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x130));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x12c), c.g[V0][0]);
                            let t = (c.g[V1][0] as i64) > 0;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x130), c.g[V1][0]);
                            if t {
                                break 'b_258318;
                            } else {
                                c.cycles += 2;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x130), 0u64);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x12c), 0u64);
                                break 'b_258318;
                            }
                        }
                        c.cycles += 1;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        break 'b_25831c;
                    }
                }
            }
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    })
}

/// `cGameComm_GetInputDelay` (0x00258360): Read the channel latency into the delay estimate.
/// Straight-line with FPU.
pub fn c_game_comm_get_input_delay(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
        c.g[RA][0] = sx(0x0025837c);
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
        callee::C_COMM_SYSTEM_GET_CHANNEL_LATENCY.call(c, 0x0025837c)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x120));
        c.cvt_s_w(1, 1);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x14));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.mul_s(1, 1, 2);
        c.add_s(0, 0, 1);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    })
}

/// `cGameComm_PingServer` (0x002583a8): Ping the server and record the reply slot. Straight-line
/// dispatch with a mult.
pub fn c_game_comm_ping_server(c: &mut Cpu) {
    guest(c, |c| {
        'b_258418: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.g[V1][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x138));
            let t = (c.g[V0][0] as i64) > 0;
            c.g[S1][0] = c.g[A2][0];
            if t {
                break 'b_258418;
            } else {
                c.cycles += 19;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
                c.g[A2][0] = 0x480000;
                c.sw((c.g[S0][0] as u32).wrapping_add(0xb8), 0u64);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1380));
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                c.g[RA][0] = sx(0x002583e8);
                c.g[A3][0] = 0xe;
                callee::PROTO_PING_REQUEST.call(c, 0x002583e8)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xb8));
                c.g[A1][0] = 0x3c;
                c.mult(0, 5, 17, 5);
                c.g[A2][0] = 0x4;
                c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xb8), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[A2][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x138), c.g[A1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x13c), 0u64);
                break 'b_258418;
            }
        }
        'b_258434: {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
            if c.g[V1][0] != 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_258434;
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x13c), c.g[V0][0]);
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_258434;
            }
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    })
}

/// `cGameComm_ExchangeSetupData` (0x00258448): Exchange setup data: profiles, pads, RNG and the
/// session wait. Bounded session-wait loop with an event check.
pub fn c_game_comm_exchange_setup_data(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00258448 => {
                    c.cycles += 27;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[A1][0] = 0x7;
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[RA][0] = sx(0x00258474);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                    callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x00258474)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3f0));
                    c.g[A1][0] = 0x480000;
                    c.g[S2][0] = c.g[V0][0];
                    c.g[A3][0] = 0u64;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5b8));
                    c.g[RA][0] = sx(0x00258490);
                    c.g[A2][0] = 0x100;
                    callee::OPERATOR_NEW_NAMED.call(c, 0x00258490)?;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x3ec), c.g[V0][0]);
                    c.g[RA][0] = sx(0x0025849c);
                    c.g[A0][0] = c.g[S2][0];
                    callee::BE_PREPARE_NET_PROFILES.call(c, 0x0025849c)?;
                    c.g[V0][0] = 0x530000;
                    c.g[S3][0] = c.g[V0][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4b30));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    let t = c.g[V1][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x0025850c;
                        continue;
                    }
                    pc = 0x002584b4;
                    continue;
                }
                0x002584b4 => {
                    c.cycles += 22;
                    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xf0));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x002584c4);
                    c.g[A2][0] = 0x10;
                    callee::EA_MEMSET.call(c, 0x002584c4)?;
                    c.g[A1][0] = 0x480000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5c8));
                    c.g[RA][0] = sx(0x002584d8);
                    c.g[A2][0] = 0xf;
                    callee::STRNCPY.call(c, 0x002584d8)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3ec));
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                    c.g[RA][0] = sx(0x002584f4);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xec), c.g[V0][0]);
                    callee::BE_NETWORK_INT_PACK_PLAYER_DATA.call(c, 0x002584f4)?;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3ec));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00258504);
                    c.g[A1][0] = 0u64;
                    callee::BE_NETWORK_INT_UNPACK_PLAYER_DATA.call(c, 0x00258504)?;
                    let t = true;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xd4));
                    if t {
                        pc = 0x002585b0;
                        continue;
                    }
                    pc = 0x0025850c;
                    continue;
                }
                0x0025850c => {
                    c.cycles += 41;
                    c.g[RA][0] = sx(0x00258514);
                    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x104));
                    callee::BEOPTION_INT_SWAP_PLAYER_SETTINGS.call(c, 0x00258514)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x00258524);
                    c.g[A2][0] = 0x10;
                    callee::EA_MEMSET.call(c, 0x00258524)?;
                    c.g[A1][0] = 0x480000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5c8));
                    c.g[RA][0] = sx(0x00258538);
                    c.g[A2][0] = 0xf;
                    callee::STRNCPY.call(c, 0x00258538)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3ec));
                    c.g[A1][0] = 0x1;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                    c.g[RA][0] = sx(0x00258554);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x100), c.g[V0][0]);
                    callee::BE_NETWORK_INT_PACK_PLAYER_DATA.call(c, 0x00258554)?;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3ec));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00258564);
                    c.g[A1][0] = 0x1;
                    callee::BE_NETWORK_INT_UNPACK_PLAYER_DATA.call(c, 0x00258564)?;
                    callee::BELIBRARY_GET_INSTANCE.call(c, 0x0025856c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00258578);
                    c.g[A1][0] = 0x1;
                    callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x00258578)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x00258588);
                    c.g[A0][0] = c.g[S0][0];
                    callee::BE_NEW_PLAYER_INT_GET_PAD_INDEX_BY_SLOT.call(c, 0x00258588)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00258598);
                    c.g[A1][0] = 0x1;
                    callee::BE_NEW_PLAYER_INT_SET_PAD_INDEX_BY_SLOT.call(c, 0x00258598)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x002585ac);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x002585ac)?;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xd4));
                    pc = 0x002585b0;
                    continue;
                }
                0x002585b0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002585b8);
                    c.g[S0][0] = c.g[A0][0];
                    callee::RAND_SAVE_GAME_STATE.call(c, 0x002585b8)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                    if t {
                        pc = 0x002585d4;
                        continue;
                    }
                    pc = 0x002585c0;
                    continue;
                }
                0x002585c0 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x002585c8);
                    c.g[A0][0] = c.g[S1][0];
                    callee::C_GAME_COMM_UPDATE.call(c, 0x002585c8)?;
                    c.g[RA][0] = sx(0x002585d0);
                    c.g[A0][0] = 0x1;
                    callee::THREAD_YIELDTICKS.call(c, 0x002585d0)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                    pc = 0x002585d4;
                    continue;
                }
                0x002585d4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] ^ 0xe;
                    if c.g[V0][0] == 0u64 {
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3ec));
                        pc = 0x002585f0;
                        continue;
                    }
                    pc = 0x002585e0;
                    continue;
                }
                0x002585e0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    let t = c.g[V0][0] == 0u64;
                    if t {
                        if c.cycles >= c.next_event {
                            c.pc = 0x002585c0;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x002585c0;
                        continue;
                    }
                    pc = 0x002585ec;
                    continue;
                }
                0x002585ec => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3ec));
                    pc = 0x002585f0;
                    continue;
                }
                0x002585f0 => {
                    c.cycles += 2;
                    let t = c.g[A0][0] == 0u64;
                    c.g[V0][0] = 0x3;
                    if t {
                        pc = 0x00258604;
                        continue;
                    }
                    pc = 0x002585f8;
                    continue;
                }
                0x002585f8 => {
                    c.cycles += 3;
                    callee::EA_MEM_FREE.call(c, 0x00258600)?;
                    c.g[V0][0] = 0x3;
                    pc = 0x00258604;
                    continue;
                }
                0x00258604 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x120), c.g[V0][0]);
                    let t = c.g[V1][0] != 0u64;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x3ec), 0u64);
                    if t {
                        pc = 0x00258690;
                        continue;
                    }
                    pc = 0x00258614;
                    continue;
                }
                0x00258614 => {
                    c.cycles += 14;
                    c.g[RA][0] = sx(0x0025861c);
                    c.g[A0][0] = c.g[S2][0];
                    callee::BE_RESET_NET_INTERFACES.call(c, 0x0025861c)?;
                    c.g[RA][0] = sx(0x00258624);
                    c.g[A0][0] = c.g[S0][0];
                    callee::RAND_RESTORE_GAME_STATE.call(c, 0x00258624)?;
                    callee::RAND_NEXT_GAME.call(c, 0x0025862c)?;
                    c.g[RA][0] = sx(0x00258634);
                    c.g[A0][0] = c.g[V0][0];
                    callee::GAME_RAND_SEED.call(c, 0x00258634)?;
                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x11c));
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4b30));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xec));
                    if c.g[V1][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                        pc = 0x00258660;
                        continue;
                    }
                    pc = 0x0025864c;
                    continue;
                }
                0x0025864c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x100));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x2;
                    if t {
                        pc = 0x0025865c;
                        continue;
                    }
                    pc = 0x00258658;
                    continue;
                }
                0x00258658 => {
                    c.cycles += 1;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x120), c.g[V0][0]);
                    pc = 0x0025865c;
                    continue;
                }
                0x0025865c => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    pc = 0x00258660;
                    continue;
                }
                0x00258660 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        pc = 0x00258694;
                        continue;
                    }
                    pc = 0x00258668;
                    continue;
                }
                0x00258668 => {
                    c.cycles += 6;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xf0));
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x104));
                    c.g[RA][0] = sx(0x00258678);
                    c.g[A2][0] = 0x9;
                    callee::STRNCMP.call(c, 0x00258678)?;
                    let t = c.g[V0][0] == 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        pc = 0x00258694;
                        continue;
                    }
                    pc = 0x00258680;
                    continue;
                }
                0x00258680 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00258688);
                    c.g[A0][0] = c.g[S1][0];
                    callee::C_GAME_COMM_LATCH_RESULT_FLAGS.call(c, 0x00258688)?;
                    c.g[RA][0] = sx(0x00258690);
                    c.g[A0][0] = c.g[S1][0];
                    callee::C_GAME_COMM_END_SESSION.call(c, 0x00258690)?;
                    pc = 0x00258690;
                    continue;
                }
                0x00258690 => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    pc = 0x00258694;
                    continue;
                }
                0x00258694 => {
                    c.cycles += 6;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00258448, 0x002586ac);
                    return Ok(());
                }
            }
        }
    })
}

pub(crate) mod send_pause_state {

    /// Non-zero once the session has closed (`cGameComm_EndSession` sets it).
    pub const CLOSED: u32 = 0x4;

    /// Pause-packet counter, bumped while paused.
    pub const PAUSE_COUNT: u32 = 0x4c;

    /// Ticks without a peer packet (late counter).
    pub const SILENT_TICKS: u32 = 0x60;

    /// Set while this player has paused.
    pub const LOCAL_PAUSED: u32 = 0x74;

    /// Set once the countdown has started.
    pub const COUNTDOWN_STARTED: u32 = 0x7c;

    /// Shows the "waiting for peer" notice.
    pub const SHOW_PEER_WAIT: u32 = 0x84;

    /// Set while paused after the countdown started.
    pub const COUNTDOWN_PAUSED: u32 = 0x3f8;

    /// Resume grace ticks left.
    pub const RESUME_GRACE: u32 = 0x3fc;

    /// Lag score accumulator.
    pub const LAG_SCORE: u32 = 0x5c;

    /// Rearm value for an unforced pause rearm.
    pub const PAUSE_TIMEOUT: u32 = 0x708;

    /// Rearm value for a forced pause rearm.
    pub const FORCE_TIMEOUT: u32 = 0xe4c;
}

pub(crate) mod update_idle_kick_timer {

    /// Non-zero once the session has closed.
    pub const CLOSED: u32 = 0x4;

    /// Wait-for-peer flag.
    pub const WAIT_PEER: u32 = 0x48;

    /// Session liveness: below 15 means the online session is up.
    pub const SESSION: u32 = 0x70;

    /// Set while this player has paused.
    pub const LOCAL_PAUSED: u32 = 0x74;

    /// Set while the peer has paused.
    pub const PEER_PAUSED: u32 = 0x78;

    /// Resume grace ticks left.
    pub const RESUME_GRACE: u32 = 0x3fc;

    /// Buddy-list generation at +0x214 of the network object.
    pub const BUDDY_GENERATION: u32 = 0x214;

    /// Idle-kick grace the timer arms.
    pub const IDLE_GRACE: u32 = 0xb4;
}

pub(crate) mod on_race_start_reset {

    /// Peer-lag word set to 1 on reset (not cleared).
    pub const LAG: u32 = 0x48;

    /// Wait-for-peer notice cleared on reset.
    pub const WAIT_NOTICE: u32 = 0x78;

    /// Race-active flag cleared on reset.
    pub const RACE_ACTIVE: u32 = 0x84;
}

pub(crate) mod on_countdown_start {

    /// Session liveness: below 15 means the online session is up.
    pub const SESSION: u32 = 0x70;

    /// Set once the countdown has started.
    pub const COUNTDOWN_STARTED: u32 = 0x7c;

    /// Cleared on countdown start.
    pub const DEADLINE: u32 = 0x94;
}

pub(crate) mod on_race_start {

    /// Session liveness: below 15 means the online session is up.
    pub const SESSION: u32 = 0x70;

    /// `GameModule` race-state value written on race start.
    pub const RACE_STATE: u32 = 0x80;

    /// Session deadline cleared on race start.
    pub const DEADLINE: u32 = 0x94;

    /// Frame counter latched from the world clock.
    pub const FRAME: u32 = 0x34;
}

pub(crate) mod on_race_end {

    /// Latched race length, filled from the world clock when 0.
    pub const RACE_LENGTH: u32 = 0x38;

    /// Latched second length, filled from the world clock when 0.
    pub const RACE_LENGTH_B: u32 = 0x3c;

    /// Cleared on race end.
    pub const PAUSE_RELATED: u32 = 0x40;

    /// Session liveness: below 15 means the online session is up.
    pub const SESSION: u32 = 0x70;

    /// Cleared on race end.
    pub const RACE_STATE: u32 = 0x80;
}

pub(crate) mod send_resync {

    /// Non-zero once the session has closed.
    pub const CLOSED: u32 = 0x4;

    /// Resync sequence number, bumped per resync packet.
    pub const SEQUENCE: u32 = 0x54;
}

/// Test-only gate of the callee fakes below: they answer only while this
/// word holds `GCOMM_FIX_MAGIC` (otherwise the body they replaced runs).
/// Test machines arm it; real gameplay never does, and the consts vanish
/// in non-test builds (`#[cfg(test)]`).
/// (`cGameComm_Update` 0x002570d8 has no override slot, so the wait loop
/// calls the real generated body; only the hooked yield callee is gated.)
#[cfg(test)]
pub(crate) const GCOMM_FIX_RAM_BASE: u32 = 0x0080_0000;

#[cfg(test)]
pub(crate) const GCOMM_FIX_CTRL: u32 = GCOMM_FIX_RAM_BASE + 0x000e_3000;

#[cfg(test)]
pub(crate) const GCOMM_FIX_MAGIC: u32 = 0x4741_5446;

/// Gated-yield tick count (see the waits test): the fake reports the peer
/// ready (`GCOMM_FIX_COMM + 0x4`) exactly once, on the 3rd gated tick. The
/// comm object lives at this test's fixed `GCOMM_FIX_COMM` address.
#[cfg(test)]
pub(crate) const GCOMM_FIX_YIELD_HITS: u32 = GCOMM_FIX_RAM_BASE + 0x000e_3004;

/// `void cGameComm_SendPauseState(cGameComm *self, int pausing)` (0x00258790):
/// tell the peer this player paused (`pausing` non-zero) or resumed (zero).
/// Pausing rearms the session deadline (forced while nobody has started the
/// countdown), marks this player paused, bumps the pause counter and sends a
/// type-0 packet; resuming clears the pause state, rearms the deadline
/// unforced and sends a type-1 packet. Does nothing once the session closed
/// or when the comm object is gone.
pub fn c_game_comm_send_pause_state(c: &mut Cpu) {
    guest(c, c_game_comm_send_pause_state_body);
}

pub(crate) fn c_game_comm_send_pause_state_body(c: &mut Cpu) -> Flow {
    use send_pause_state::*;
    c.cycles += 7;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save_ra(c, 0x10);
    c.g[S0][0] = c.g[A0][0];
    let comm = Addr(c.g[S0][0] as u32);
    c.g[V0][0] = comm.off(CLOSED).lw(c);
    if c.g[V0][0] != 0 {
        frame.restore(c, S0, 0x20);
    } else {
        c.cycles += 3;
        c.g[V0][0] = comm.lw(c);
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            'dispatch: {
                'pausedone: {
                    'argtest: {
                        c.cycles += 2;
                        let resuming = c.g[A1][0] == 0;
                        c.g[V0][0] = 1;
                        if resuming {
                            break 'argtest;
                        }
                        'pausearm: {
                            'pausetest: {
                                c.cycles += 3;
                                c.g[V0][0] = comm.off(COUNTDOWN_STARTED).lw(c);
                                let started = c.g[V0][0] != 0;
                                c.g[V0][0] = 1;
                                if started {
                                    break 'pausetest;
                                }
                                c.cycles += 5;
                                c.g[A1][0] = FORCE_TIMEOUT as u64;
                                c.g[A2][0] = 1;
                                callee::C_GAME_COMM_SET_TIMEOUT_TIMER.call(c, 0x0025_87d8)?;
                                c.g[V1][0] = comm.off(PAUSE_COUNT).lw(c);
                                break 'pausearm;
                            }
                            c.cycles += 2;
                            comm.off(COUNTDOWN_PAUSED).sw(c, c.g[V0][0]);
                            c.g[V1][0] = comm.off(PAUSE_COUNT).lw(c);
                            break 'pausearm;
                        }
                        c.cycles += 8;
                        c.g[V0][0] = 1;
                        comm.off(LOCAL_PAUSED).sw(c, c.g[V0][0]);
                        Addr(sp(c)).sb(c, 0);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                        comm.off(PAUSE_COUNT).sw(c, c.g[V1][0]);
                        comm.off(RESUME_GRACE).sw(c, 0);
                        comm.off(LAG_SCORE).sw(c, 0);
                        break 'pausedone;
                    }
                    'resume: {
                        c.cycles += 8;
                        c.g[V1][0] = comm.off(COUNTDOWN_STARTED).lw(c);
                        Addr(sp(c)).sb(c, c.g[V0][0]);
                        comm.off(LOCAL_PAUSED).sw(c, 0);
                        comm.off(COUNTDOWN_PAUSED).sw(c, 0);
                        comm.off(SILENT_TICKS).sw(c, 0);
                        comm.off(SHOW_PEER_WAIT).sw(c, 0);
                        let started = c.g[V1][0] != 0;
                        comm.off(LAG_SCORE).sw(c, 0);
                        if started {
                            break 'resume;
                        }
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = PAUSE_TIMEOUT as u64;
                        c.g[A2][0] = 0;
                        callee::C_GAME_COMM_SET_TIMEOUT_TIMER.call(c, 0x0025_8838)?;
                        c.g[A0][0] = c.g[S0][0];
                        break 'dispatch;
                    }
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    callee::C_GAME_COMM_UPDATE_IDLE_KICK_TIMER.call(c, 0x0025_8848)?;
                    break 'pausedone;
                }
                c.cycles += 1;
                c.g[A0][0] = c.g[S0][0];
                break 'dispatch;
            }
            c.cycles += 5;
            c.g[A1][0] = 3;
            c.g[A2][0] = c.g[SP][0];
            c.g[A3][0] = 0xc;
            callee::C_GAME_COMM_SEND_PACKET.call(c, 0x0025_885c)?;
            frame.restore(c, S0, 0x20);
        }
    }
    c.cycles += 3;
    frame.restore_ra(c, 0x10);
    frame.close(c);
    Ok(())
}

/// `void cGameComm_UpdateIdleKickTimer(cGameComm *self)` (0x002588a8): arm
/// the resume-grace timer when the session is up, nobody paused and the
/// buddy-list generation word is stale (`+0x214 != 2`). Clears the grace
/// first; leaves it clear unless every gate passes, then sets it to
/// `IDLE_GRACE`.
pub fn c_game_comm_update_idle_kick_timer(c: &mut Cpu) {
    c.cycles += 8;
    c.g[A1][0] = c.g[A0][0];
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    Addr(c.g[A1][0] as u32)
        .off(update_idle_kick_timer::RESUME_GRACE)
        .sw(c, 0);
    c.g[A0][0] = 2;
    c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(update_idle_kick_timer::BUDDY_GENERATION));
    'done: {
        let fresh_list = c.g[V1][0] == c.g[A0][0];
        if !fresh_list {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[A1][0] as u32)
                .off(update_idle_kick_timer::CLOSED)
                .lw(c);
            let closed = c.g[V0][0] != 0;
            if !closed {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[A1][0] as u32)
                    .off(update_idle_kick_timer::WAIT_PEER)
                    .lw(c);
                let waiting = c.g[V0][0] != 0;
                if !waiting {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
                    let gone = c.g[V0][0] == 0;
                    if !gone {
                        c.cycles += 4;
                        c.g[V0][0] = Addr(c.g[A1][0] as u32)
                            .off(update_idle_kick_timer::SESSION)
                            .lw(c);
                        c.g[V0][0] = ((c.g[V0][0] as i64) < 15i64) as u64;
                        let live = c.g[V0][0] != 0;
                        c.g[V1][0] = 4;
                        if live {
                            'buddy: {
                                c.cycles += 4;
                                c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                                c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                                if c.g[V0][0] != c.g[V1][0] {
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                                    break 'buddy;
                                }
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                                c.g[V0][0] = ((c.g[V0][0] as i64) < 151i64) as u64;
                                let cool = c.g[V0][0] == 0;
                                if cool {
                                    break 'done;
                                }
                                c.cycles += 1;
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                                break 'buddy;
                            }
                            c.cycles += 2;
                            let idle = (c.g[V0][0] as i64) > 0;
                            if !idle {
                                c.cycles += 3;
                                c.g[V0][0] = Addr(c.g[A1][0] as u32)
                                    .off(update_idle_kick_timer::LOCAL_PAUSED)
                                    .lw(c);
                                let paused = c.g[V0][0] != 0;
                                if !paused {
                                    c.cycles += 3;
                                    c.g[V0][0] = Addr(c.g[A1][0] as u32)
                                        .off(update_idle_kick_timer::PEER_PAUSED)
                                        .lw(c);
                                    let peer_paused = c.g[V0][0] != 0;
                                    c.g[V0][0] = update_idle_kick_timer::IDLE_GRACE as u64;
                                    if !peer_paused {
                                        c.cycles += 1;
                                        Addr(c.g[A1][0] as u32)
                                            .off(update_idle_kick_timer::RESUME_GRACE)
                                            .sw(c, c.g[V0][0]);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.pc = t;
}

/// `void cGameComm_OnRaceStartReset(cGameComm *self)` (0x00258950): reset
/// the race state for a fresh start: set the lag word to 1 and clear the
/// wait notice and race-active flag.
pub fn c_game_comm_on_race_start_reset(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = 1;
    let comm = Addr(arg(c, 0));
    comm.off(on_race_start_reset::WAIT_NOTICE).sw(c, 0);
    comm.off(on_race_start_reset::LAG).sw(c, c.g[V0][0]);
    let t = c.g[RA][0] as u32;
    comm.off(on_race_start_reset::RACE_ACTIVE).sw(c, 0);
    c.pc = t;
}

/// `void cGameComm_OnCountdownStart(cGameComm *self)` (0x00258968): mark the
/// countdown started while the online session is up, clearing the deadline.
pub fn c_game_comm_on_countdown_start(c: &mut Cpu) {
    c.cycles += 4;
    let comm = Addr(arg(c, 0));
    c.g[V0][0] = comm.off(on_countdown_start::SESSION).lw(c);
    c.g[V0][0] = ((c.g[V0][0] as i64) < 15i64) as u64;
    let down = c.g[V0][0] == 0;
    c.g[V0][0] = 1;
    if !down {
        c.cycles += 2;
        comm.off(on_countdown_start::DEADLINE).sw(c, 0);
        comm.off(on_countdown_start::COUNTDOWN_STARTED)
            .sw(c, c.g[V0][0]);
    }
    c.cycles += 2;
    ret(c);
}

/// `void cGameComm_OnRaceStart(cGameComm *self)` (0x00258988): open the race
/// on the buddy and lobby managers and latch the frame counter from the
/// world clock. Does nothing unless the online session is up.
pub fn c_game_comm_on_race_start(c: &mut Cpu) {
    guest(c, c_game_comm_on_race_start_body);
}

pub(crate) fn c_game_comm_on_race_start_body(c: &mut Cpu) -> Flow {
    use on_race_start::*;
    c.cycles += 8;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    let comm = Addr(c.g[S0][0] as u32);
    c.g[V0][0] = comm.off(SESSION).lw(c);
    c.g[V0][0] = ((c.g[V0][0] as i64) < 15i64) as u64;
    let down = c.g[V0][0] == 0;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    if !down {
        'buddy: {
            c.cycles += 2;
            let missing = c.g[V0][0] != 0;
            if missing {
                break 'buddy;
            }
            c.cycles += 2;
            callee::BUDDYMAN_CREATE.call(c, 0x0025_89b8)?;
            break 'buddy;
        }
        'connected: {
            c.cycles += 4;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
            callee::BUDDY_MAN_IS_CONNECTED.call(c, 0x0025_89c0)?;
            let online = c.g[V0][0] != 0;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
            if online {
                break 'connected;
            }
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
            callee::BUDDY_MAN_RECONNECT.call(c, 0x0025_89d0)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
            break 'connected;
        }
        'lobby: {
            c.cycles += 2;
            let missing = c.g[V0][0] != 0;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
            if missing {
                break 'lobby;
            }
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
            callee::LOBBY_MAN_DISCONNECT.call(c, 0x0025_89e4)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
            break 'lobby;
        }
        'presence: {
            c.cycles += 2;
            let missing = c.g[A0][0] == 0;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
            if missing {
                break 'presence;
            }
            c.cycles += 6;
            c.g[A1][0] = 1;
            c.g[A2][0] = 9;
            c.g[T0][0] = 0;
            c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x98));
            c.g[T1][0] = 0;
            call_ptr(c, callee::BUDDY_MAN_SET_PRESENCE.addr, 0x0025_8a08)?;
            break 'presence;
        }
        'clock: {
            c.cycles += 6;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A1][0] = 1;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
            if c.g[V0][0] == 0 {
                break 'clock;
            } else {
                break 'clock;
            }
        }
        c.cycles += 5;
        c.div(0, 3, 2);
        comm.off(RACE_STATE).sw(c, c.g[A1][0]);
        comm.off(DEADLINE).sw(c, 0);
        c.g[V1][0] = c.lo[0];
        comm.off(FRAME).sw(c, c.g[V1][0]);
    }
    c.cycles += 4;
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void cGameComm_OnRaceEnd(cGameComm *self)` (0x00258a48): close the race:
/// clear the race state, move to the lobby state while the session is up,
/// and latch both race lengths from the world clock when still unset.
pub fn c_game_comm_on_race_end(c: &mut Cpu) {
    guest(c, c_game_comm_on_race_end_body);
}

pub(crate) fn c_game_comm_on_race_end_body(c: &mut Cpu) -> Flow {
    use on_race_end::*;
    c.cycles += 9;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    let comm = Addr(c.g[S0][0] as u32);
    comm.off(RACE_STATE).sw(c, 0);
    c.g[V0][0] = comm.off(SESSION).lw(c);
    c.g[V0][0] = ((c.g[V0][0] as i64) < 15i64) as u64;
    let down = c.g[V0][0] == 0;
    comm.off(PAUSE_RELATED).sw(c, 0);
    if !down {
        c.cycles += 2;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0xf;
        callee::C_GAME_COMM_SET_STATE.call(c, 0x0025_8a74)?;
    }
    'length_a: {
        c.cycles += 3;
        c.g[V0][0] = comm.off(RACE_LENGTH).lw(c);
        if c.g[V0][0] != 0 {
            c.g[V0][0] = comm.off(RACE_LENGTH_B).lw(c);
            break 'length_a;
        }
        'clock_a: {
            c.cycles += 5;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
            if c.g[V1][0] == 0 {
                break 'clock_a;
            } else {
                break 'clock_a;
            }
        }
        c.cycles += 4;
        c.div(0, 2, 3);
        c.g[V0][0] = c.lo[0];
        comm.off(RACE_LENGTH).sw(c, c.g[V0][0]);
        c.g[V0][0] = comm.off(RACE_LENGTH_B).lw(c);
        break 'length_a;
    }
    'length_b: {
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x10);
            break 'length_b;
        }
        'clock_b: {
            c.cycles += 5;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
            if c.g[V1][0] == 0 {
                break 'clock_b;
            } else {
                break 'clock_b;
            }
        }
        c.cycles += 4;
        c.div(0, 2, 3);
        c.g[V0][0] = c.lo[0];
        comm.off(RACE_LENGTH_B).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        break 'length_b;
    }
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void cGameComm_QuitRace(cGameComm *self)` (0x00258ae0): leave the race:
/// make sure the buddy manager is up, send the quit packet, close the race
/// and drop the lobby session, hiding the wait popup when it is not showing.
pub fn c_game_comm_quit_race(c: &mut Cpu) {
    guest(c, c_game_comm_quit_race_body);
}

pub(crate) fn c_game_comm_quit_race_body(c: &mut Cpu) -> Flow {
    c.cycles += 6;
    let frame = Frame::open(c, 0x20);
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    let missing = c.g[V0][0] != 0;
    c.g[S0][0] = c.g[A0][0];
    if !missing {
        c.cycles += 2;
        callee::BUDDYMAN_CREATE.call(c, 0x0025_8b00)?;
    }
    'connected: {
        c.cycles += 4;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        callee::BUDDY_MAN_IS_CONNECTED.call(c, 0x0025_8b08)?;
        let online = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        if online {
            break 'connected;
        }
        c.cycles += 3;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        callee::BUDDY_MAN_RECONNECT.call(c, 0x0025_8b18)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        break 'connected;
    }
    'lobby: {
        c.cycles += 2;
        let missing = c.g[V0][0] != 0;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        if missing {
            break 'lobby;
        }
        c.cycles += 3;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        callee::LOBBY_MAN_DISCONNECT.call(c, 0x0025_8b2c)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        break 'lobby;
    }
    'presence: {
        c.cycles += 2;
        let missing = c.g[A0][0] == 0;
        c.g[A1][0] = 1;
        if missing {
            break 'presence;
        }
        c.cycles += 5;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        call_ptr(c, callee::BUDDY_MAN_SET_PRESENCE.addr, 0x0025_8b4c)?;
        break 'presence;
    }
    'leave: {
        c.cycles += 13;
        c.g[A0][0] = c.g[S0][0];
        callee::C_GAME_COMM_SEND_QUIT.call(c, 0x0025_8b54)?;
        c.g[A0][0] = c.g[S0][0];
        callee::C_GAME_COMM_ON_RACE_END.call(c, 0x0025_8b5c)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A1][0] = 7;
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
        call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE.addr, 0x0025_8b6c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = 0x530000;
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4b38));
        let popup_showing = c.g[V1][0] != 0;
        frame.restore(c, S0, 0x10);
        if popup_showing {
            break 'leave;
        }
        c.cycles += 14;
        c.g[A1][0] = 1;
        c.g[A2][0] = 0;
        call_ptr(c, callee::BE_COPY_PROFILE.addr, 0x0025_8b8c)?;
        call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE.addr, 0x0025_8b94)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 3;
        call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE.addr, 0x0025_8ba0)?;
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let t = c.g[A1][0] as u32;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0025_8bb4)?;
        frame.restore(c, S0, 0x10);
        break 'leave;
    }
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void cGameComm_SendQuit(cGameComm *self)` (0x00258bc8): send the
/// quit-race packet (type 5), unless the session already closed.
pub fn c_game_comm_send_quit(c: &mut Cpu) {
    guest(c, c_game_comm_send_quit_body);
}

pub(crate) fn c_game_comm_send_quit_body(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    let frame = Frame::open(c, 0x20);
    frame.save_ra(c, 0x10);
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
    let closed = c.g[V0][0] != 0;
    c.g[RA][0] = c.ld(frame.at(c, 0x10));
    if !closed {
        c.cycles += 7;
        c.g[V0][0] = 5;
        c.g[A1][0] = 3;
        Addr(sp(c)).sb(c, c.g[V0][0]);
        c.g[A2][0] = c.g[SP][0];
        c.g[A3][0] = 0xc;
        callee::C_GAME_COMM_SEND_PACKET.call(c, 0x0025_8bf4)?;
        c.g[RA][0] = c.ld(frame.at(c, 0x10));
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    c.pc = t;
    Ok(())
}

/// `void cGameComm_SendResync(cGameComm *self)` (0x00258c00): send the
/// resync packet (type 2) with the next sequence number, unless the session
/// closed or the comm object is gone.
pub fn c_game_comm_send_resync(c: &mut Cpu) {
    guest(c, c_game_comm_send_resync_body);
}

pub(crate) fn c_game_comm_send_resync_body(c: &mut Cpu) -> Flow {
    c.cycles += 6;
    let frame = Frame::open(c, 0x20);
    c.g[A2][0] = c.g[A0][0];
    frame.save_ra(c, 0x10);
    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(send_resync::CLOSED));
    let closed = c.g[V0][0] != 0;
    c.g[RA][0] = c.ld(frame.at(c, 0x10));
    if !closed {
        c.cycles += 3;
        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
        let gone = c.g[V0][0] == 0;
        c.g[V0][0] = 2;
        if !gone {
            c.cycles += 9;
            let comm = Addr(c.g[A2][0] as u32);
            c.g[V1][0] = comm.off(send_resync::SEQUENCE).lw(c);
            Addr(sp(c)).sb(c, c.g[V0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            c.g[A1][0] = 3;
            comm.off(send_resync::SEQUENCE).sw(c, c.g[V1][0]);
            c.g[A3][0] = 0xc;
            c.g[A2][0] = c.g[SP][0];
            callee::C_GAME_COMM_SEND_PACKET.call(c, 0x0025_8c44)?;
            c.g[RA][0] = c.ld(frame.at(c, 0x10));
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    c.pc = t;
    Ok(())
}

/// `void cGameComm_SendString(cGameComm *self, const char *text)`
/// (0x00258c50): send a type-1 packet carrying `text` (with its terminator),
/// unless the session closed or the comm object is gone.
pub fn c_game_comm_send_string(c: &mut Cpu) {
    guest(c, c_game_comm_send_string_body);
}

pub(crate) fn c_game_comm_send_string_body(c: &mut Cpu) -> Flow {
    c.cycles += 8;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
    let closed = c.g[V0][0] != 0;
    c.g[S1][0] = c.g[A1][0];
    if !closed {
        c.cycles += 3;
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        if c.g[V0][0] == 0 {
            // Gone arm: straight to the epilogue without sending.
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 7;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, callee::STRLEN.addr, 0x0025_8c84)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            c.g[A1][0] = 1;
            callee::C_GAME_COMM_SEND_PACKET.call(c, 0x0025_8c98)?;
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
    } else {
        c.cycles += 1;
        frame.restore(c, S0, 0x20);
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void cGameComm_WaitForPeerReady(cGameComm *self)` (0x002586b0):
/// block on the peer-ready handshake: send a sync-check packet, arm the
/// join timeout, then loop driving `cGameComm_Update` (yielding ticks)
/// until every peer reports ready or the wait times out. The loop's
/// back-edge event check resumes at its loop head, which is a match arm
/// below.
pub fn c_game_comm_wait_for_peer_ready(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x002586b0 => {
                    c.cycles += 15;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[A1][0] = 0x7;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S0][0] = c.g[A0][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    c.g[RA][0] = sx(0x002586d4);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                    callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x002586d4)?;
                    c.g[V1][0] = 0x530000;
                    c.g[S1][0] = c.g[V1][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4b30));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x3;
                    if t {
                        pc = 0x00258704;
                        continue;
                    }
                    pc = 0x002586ec;
                    continue;
                }
                0x002586ec => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.sb(c.g[SP][0] as u32, c.g[V0][0]);
                    c.g[A1][0] = 0x3;
                    c.g[A2][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x00258704);
                    c.g[A3][0] = 0xc;
                    callee::C_GAME_COMM_SEND_PACKET.call(c, 0x00258704)?;
                    pc = 0x00258704;
                    continue;
                }
                0x00258704 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0xe10;
                    c.g[RA][0] = sx(0x00258714);
                    c.g[A2][0] = 0u64;
                    callee::C_GAME_COMM_SET_TIMEOUT_TIMER.call(c, 0x00258714)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3f4));
                    if t {
                        pc = 0x00258734;
                        continue;
                    }
                    pc = 0x0025871c;
                    continue;
                }
                0x0025871c => {
                    c.cycles += 1;
                    pc = 0x00258720;
                    continue;
                }
                0x00258720 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x00258728);
                    c.g[A0][0] = c.g[S0][0];
                    callee::C_GAME_COMM_UPDATE.call(c, 0x00258728)?;
                    c.g[RA][0] = sx(0x00258730);
                    c.g[A0][0] = 0x1;
                    callee::THREAD_YIELD_TICKS.call(c, 0x00258730)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3f4));
                    pc = 0x00258734;
                    continue;
                }
                0x00258734 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x00258744;
                        continue;
                    }
                    pc = 0x0025873c;
                    continue;
                }
                0x0025873c => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        if back_edge_unwinds(c, 0x00258720) {
                            return Ok(());
                        }
                        pc = 0x00258720;
                        continue;
                    }
                    pc = 0x00258744;
                    continue;
                }
                0x00258744 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x94), 0u64);
                    if t {
                        pc = 0x00258778;
                        continue;
                    }
                    pc = 0x0025874c;
                    continue;
                }
                0x0025874c => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4b30));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));

                    if c.g[V1][0] != 0u64 {
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xac), 0u64);
                        pc = 0x0025877c;
                        continue;
                    }
                    pc = 0x0025875c;
                    continue;
                }
                0x0025875c => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x3;
                    c.g[A0][0] = c.g[S0][0];
                    c.sb(c.g[SP][0] as u32, c.g[V0][0]);
                    c.g[A1][0] = 0x3;
                    c.g[A2][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x00258778);
                    c.g[A3][0] = 0xc;
                    callee::C_GAME_COMM_SEND_PACKET.call(c, 0x00258778)?;
                    pc = 0x00258778;
                    continue;
                }
                0x00258778 => {
                    c.cycles += 1;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xac), 0u64);
                    pc = 0x0025877c;
                    continue;
                }
                0x0025877c => {
                    c.cycles += 5;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002586b0, 0x00258790);
                    return Ok(());
                }
            }
        }
    });
}

// ---------------------------------------------------------------------------
// `cGameComm_UpdateWaitPopup` (0x258cb0) constants.

/// Stack frame size of `cGameComm_UpdateWaitPopup`.
pub(crate) const WAIT_POPUP_FRAME: u32 = 0x150;

/// Frame slot holding the caller's `s3` (`self`).
pub(crate) const WAIT_POPUP_S3: u32 = 0x110;

/// Frame slot holding the caller's `s0` (UI manager + 0x18).
pub(crate) const WAIT_POPUP_S0: u32 = 0x140;

/// Frame slot holding the caller's `s1` (string object).
pub(crate) const WAIT_POPUP_S1: u32 = 0x130;

/// Frame slot holding the caller's `s2` (UI state).
pub(crate) const WAIT_POPUP_S2: u32 = 0x120;

/// Frame slot holding the return address.
pub(crate) const WAIT_POPUP_RA: u32 = 0x100;

/// `cGameComm` field: nonzero once the session has closed (clears +0x60).
pub(crate) const WAIT_POPUP_SHOW_WAIT: u32 = 0x84;

/// `cGameComm` field: cleared when the popup closes.
pub(crate) const WAIT_POPUP_RESULT: u32 = 0x60;

/// `cGameComm` field: peer pause flag (matches `game_comm`), tested to
/// pick the lobby message.
pub(crate) const WAIT_POPUP_PEER_PAUSED: u32 = 0x78;

/// `cGameComm` field: popup mode (0 hidden, 1 waiting, 2 counting down).
pub(crate) const WAIT_POPUP_MODE: u32 = 0x98;

/// `cGameComm` field: local pause flag (matches `game_comm`).
pub(crate) const WAIT_POPUP_LOCAL_PAUSED: u32 = 0x74;

/// `cGameComm` field: resume grace left.
pub(crate) const WAIT_POPUP_RESUME_GRACE: u32 = 0x3fc;

/// `void cGameComm_UpdateWaitPopup(cGameComm *self)` (0x258cb0): refresh the
/// net lobby's wait popup. When the session closed, clear the result and
/// hide the popup; otherwise look up the wait UI state and show, update or
/// hide the popup from the pause flags and the popup mode.
pub fn c_game_comm_update_wait_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, WAIT_POPUP_FRAME);
        frame.save(c, S3, WAIT_POPUP_S3);
        frame.save(c, S0, WAIT_POPUP_S0);
        frame.save(c, S1, WAIT_POPUP_S1);
        frame.save(c, S2, WAIT_POPUP_S2);
        frame.save_ra(c, WAIT_POPUP_RA);
        c.g[S3][0] = c.g[A0][0];
        let comm = Addr(r32(c, S3));
        c.g[V0][0] = comm.off(WAIT_POPUP_SHOW_WAIT).lw(c);
        if c.g[V0][0] != 0 {
            comm.off(WAIT_POPUP_RESULT).sw(c, 0);
        }
        'b_258f04: {
            'b_258f00: {
                c.cycles += 5;
                let app = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                c.g[V0][0] = app;
                c.g[V1][0] = Addr(app as u32).off(0x84).lw(c);
                c.g[S0][0] = Addr(c.g[V1][0] as u32).off(0x48).lw(c);
                let no_ui = c.g[S0][0] == 0u64;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                if no_ui {
                    break 'b_258f00;
                }
                c.cycles += 9;
                c.g[A0][0] = sx(0x004a_2ef0);
                callee::STR_HASH.call(c, 0x0025_8cf8)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                callee::UI_STATE_MGR_FIND_STATE.call(c, 0x0025_8d04)?;
                c.g[S2][0] = c.g[V0][0];
                if c.g[S2][0] == 0u64 {
                    frame.restore(c, S0, WAIT_POPUP_S0);
                    break 'b_258f04;
                }
                'b_258d98: {
                    'b_258d5c: {
                        c.cycles += 3;
                        c.g[V0][0] = comm.off(WAIT_POPUP_PEER_PAUSED).lw(c);
                        let waiting = c.g[V0][0] == 0u64;
                        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                        if waiting {
                            break 'b_258d5c;
                        }
                        c.cycles += 16;
                        c.g[A0][0] = sx(0x0048_05d8);
                        c.g[S0][0] = Addr(c.g[V0][0] as u32).off(0x8c).lw(c);
                        c.g[S1][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
                        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x20).lh(c);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        callee::STR_HASH.call(c, 0x0025_8d38)?;
                        c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x24).lw(c);
                        c.g[A0][0] = c.g[S0][0];
                        let resolve = c.g[V1][0] as u32;
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, resolve, 0x0025_8d48)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A0][0] = sx(sp(c));
                        callee::USTR_COPY.call(c, 0x0025_8d54)?;
                        c.g[V1][0] = comm.off(WAIT_POPUP_MODE).lw(c);
                        break 'b_258d98;
                    }
                    c.cycles += 15;
                    c.g[A0][0] = sx(0x0048_05f8);
                    c.g[S0][0] = Addr(c.g[V0][0] as u32).off(0x8c).lw(c);
                    c.g[S1][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x20).lh(c);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    callee::STR_HASH.call(c, 0x0025_8d78)?;
                    c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x24).lw(c);
                    c.g[A0][0] = c.g[S0][0];
                    let resolve = c.g[V1][0] as u32;
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, resolve, 0x0025_8d88)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = sx(sp(c));
                    callee::USTR_COPY.call(c, 0x0025_8d94)?;
                    c.g[V1][0] = comm.off(WAIT_POPUP_MODE).lw(c);
                }
                'b_258ee4: {
                    'b_258e9c: {
                        c.cycles += 3;
                        c.g[S0][0] = 0x1;
                        let is_waiting = c.g[V1][0] == c.g[S0][0];
                        c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                        if is_waiting {
                            break 'b_258e9c;
                        }
                        'b_258e80: {
                            'b_258e60: {
                                'b_258e48: {
                                    'b_258dcc: {
                                        'b_258dbc: {
                                            c.cycles += 2;
                                            let at_least_two = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0x2;
                                            if at_least_two {
                                                break 'b_258dbc;
                                            }
                                            c.cycles += 2;
                                            if c.g[V1][0] == 0 {
                                                c.g[V0][0] =
                                                    comm.off(WAIT_POPUP_LOCAL_PAUSED).lw(c);
                                                break 'b_258dcc;
                                            }
                                            c.cycles += 2;
                                            c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0x8).lw(c);
                                            break 'b_258ee4;
                                        }
                                        c.cycles += 2;
                                        if c.g[V1][0] == c.g[V0][0] {
                                            c.g[V0][0] = comm.off(WAIT_POPUP_LOCAL_PAUSED).lw(c);
                                            break 'b_258e48;
                                        }
                                        c.cycles += 2;
                                        c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0x8).lw(c);
                                        break 'b_258ee4;
                                    }
                                    c.cycles += 2;
                                    if c.g[V0][0] != 0u64 {
                                        frame.restore(c, S0, WAIT_POPUP_S0);
                                        break 'b_258f04;
                                    }
                                    'b_258e18: {
                                        c.cycles += 3;
                                        c.g[V0][0] = comm.off(WAIT_POPUP_PEER_PAUSED).lw(c);
                                        let waiting = c.g[V0][0] == 0u64;
                                        c.g[V0][0] =
                                            Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                                        if waiting {
                                            break 'b_258e18;
                                        }
                                        c.cycles += 7;
                                        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
                                        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0xc).lw(c);
                                        c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                                        c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0x8).lw(c);
                                            break 'b_258e60;
                                        }
                                        c.cycles += 5;
                                        c.g[V1][0] =
                                            Addr((c.g[GP][0] as u32).wrapping_sub(0x84c)).lw(c);
                                        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x550).lw(c);
                                        c.g[V0][0] = c.g[V0][0] ^ 0x2;
                                        if c.g[V0][0] == 0u64 {
                                            frame.restore(c, S0, WAIT_POPUP_S0);
                                            break 'b_258f04;
                                        }
                                        c.cycles += 2;
                                        c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0x8).lw(c);
                                        break 'b_258e60;
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = comm.off(WAIT_POPUP_SHOW_WAIT).lw(c);
                                    let hidden = c.g[V0][0] == 0u64;
                                    c.g[A1][0] = 0xe;
                                    if hidden {
                                        break 'b_258f00;
                                    }
                                    c.cycles += 9;
                                    c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0x8).lw(c);
                                    c.g[A2][0] = sx(sp(c));
                                    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0xc0).lh(c);
                                    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xc4).lw(c);
                                    let show = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, show, 0x0025_8e3c)?;
                                    c.g[V1][0] = 0x2;
                                    comm.off(WAIT_POPUP_MODE).sw(c, c.g[V1][0]);
                                    break 'b_258f00;
                                }
                                c.cycles += 2;
                                if c.g[V0][0] != 0u64 {
                                    c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0x8).lw(c);
                                    break 'b_258ee4;
                                }
                                c.cycles += 3;
                                c.g[V0][0] = comm.off(WAIT_POPUP_PEER_PAUSED).lw(c);
                                if c.g[V0][0] == 0u64 {
                                    c.g[V0][0] = comm.off(WAIT_POPUP_SHOW_WAIT).lw(c);
                                    break 'b_258e80;
                                }
                                c.cycles += 1;
                                c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0x8).lw(c);
                                break 'b_258e60;
                            }
                            c.cycles += 8;
                            c.g[A1][0] = 0xe;
                            c.g[A2][0] = sx(sp(c));
                            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0xc0).lh(c);
                            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xc4).lw(c);
                            let show = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, show, 0x0025_8e78)?;
                            comm.off(WAIT_POPUP_MODE).sw(c, c.g[S0][0]);
                            break 'b_258f00;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] == 0u64 {
                            c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0x8).lw(c);
                            break 'b_258ee4;
                        }
                        // Resume grace, not the peer flag: an expired (signed <= 0)
                        // grace returns without the tail below; a live grace
                        // goes to the hide tail.
                        c.cycles += 3;
                        c.g[V0][0] = comm.off(WAIT_POPUP_RESUME_GRACE).lw(c);
                        let expired = (c.g[V0][0] as i64) <= 0;
                        frame.restore(c, S0, WAIT_POPUP_S0);
                        if expired {
                            break 'b_258f04;
                        }
                        c.cycles += 2;
                        c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0x8).lw(c);
                        break 'b_258ee4;
                    }
                    'b_258ecc: {
                        c.cycles += 6;
                        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                        c.g[A1][0] = 0x2;
                        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x84).lw(c);
                        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x214).lw(c);
                        if c.g[V1][0] != c.g[A1][0] {
                            c.g[V0][0] = comm.off(WAIT_POPUP_PEER_PAUSED).lw(c);
                            break 'b_258ecc;
                        }
                        c.cycles += 5;
                        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x84c)).lw(c);
                        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x550).lw(c);
                        c.g[V0][0] = c.g[V0][0] ^ 0x2;
                        if c.g[V0][0] == 0u64 {
                            c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0x8).lw(c);
                            break 'b_258ee4;
                        }
                        c.cycles += 1;
                        c.g[V0][0] = comm.off(WAIT_POPUP_PEER_PAUSED).lw(c);
                        break 'b_258ecc;
                    }
                    c.cycles += 2;
                    if c.g[V0][0] == 0u64 {
                        c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0x8).lw(c);
                        break 'b_258ee4;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = comm.off(WAIT_POPUP_LOCAL_PAUSED).lw(c);
                    let peer_quiet = c.g[V0][0] == 0u64;
                    frame.restore(c, S0, WAIT_POPUP_S0);
                    if peer_quiet {
                        break 'b_258f04;
                    }
                    c.cycles += 1;
                    c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0x8).lw(c);
                    break 'b_258ee4;
                }
                c.cycles += 7;
                c.g[A1][0] = 0xf;
                c.g[A2][0] = 0u64;
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0xc0).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xc4).lw(c);
                let hide = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, hide, 0x0025_8efc)?;
                comm.off(WAIT_POPUP_MODE).sw(c, 0u64);
                break 'b_258f00;
            }
            c.cycles += 1;
            frame.restore(c, S0, WAIT_POPUP_S0);
            break 'b_258f04;
        }
        c.cycles += 6;
        frame.restore(c, S1, WAIT_POPUP_S1);
        frame.restore(c, S2, WAIT_POPUP_S2);
        frame.restore(c, S3, WAIT_POPUP_S3);
        frame.restore_ra(c, WAIT_POPUP_RA);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cGameComm_SendSyncCheck` (0x258f20) constants.

/// Stack frame size of `cGameComm_SendSyncCheck`.
pub(crate) const SYNC_CHECK_FRAME: u32 = 0x40;

/// Frame slot holding the caller's `s0` (`self`).
pub(crate) const SYNC_CHECK_S0: u32 = 0x30;

/// Frame slot holding the caller's `s1` (ring base).
pub(crate) const SYNC_CHECK_S1: u32 = 0x20;

/// Frame slot holding the return address.
pub(crate) const SYNC_CHECK_RA: u32 = 0x10;

/// `cGameComm` field: nonzero while the session is up (the check only
/// runs then).
pub(crate) const SYNC_CHECK_READY: u32 = 0x40;

/// `cGameComm` field: nonzero once the session has closed (the check
/// exits early).
pub(crate) const SYNC_CHECK_CLOSED: u32 = 0x4;

/// `cGameComm` field: set when the rings disagree.
pub(crate) const SYNC_CHECK_DESYNCED: u32 = 0xc;

/// `cGameComm` field: sync packet already sent.
pub(crate) const SYNC_CHECK_SENT: u32 = 0x400;

/// `cGameComm` field: sync reply received.
pub(crate) const SYNC_CHECK_REPLIED: u32 = 0x404;

/// `cGameComm` field: last frame checked.
pub(crate) const SYNC_CHECK_FRAME_NO: u32 = 0x408;

/// `cGameComm` field: earliest frame either side has seen.
pub(crate) const SYNC_CHECK_MIN_FRAME: u32 = 0x40c;

/// `cGameComm` field: ring of recent check values, 60 entries.
pub(crate) const SYNC_CHECK_RING: u32 = 0x410;

/// `cGameComm` field: the peer's ring of check values, 60 entries.
pub(crate) const SYNC_CHECK_PEER_RING: u32 = 0x500;

/// Ring length: frames are reduced modulo 60 twice.
pub(crate) const SYNC_CHECK_RING_LEN: u32 = 0x3c;

/// `void cGameComm_SendSyncCheck(cGameComm *self, int frame, int value)`
/// (0x258f20): record this frame's check `value` in the sync ring and, the
/// first time, send it to the peer; then compare the earliest frame both
/// sides have reported and flag desync when the rings disagree. Only runs
/// while the lobby is in game (states 3 and 5) and the session is live.
pub fn c_game_comm_send_sync_check(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, SYNC_CHECK_FRAME);
        frame.save(c, S0, SYNC_CHECK_S0);
        frame.save(c, S1, SYNC_CHECK_S1);
        frame.save_ra(c, SYNC_CHECK_RA);
        c.g[A3][0] = c.g[A1][0];
        c.g[S0][0] = c.g[A0][0];
        'b_259084: {
            'b_258f68: {
                let app = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                c.g[V0][0] = app;
                c.g[V1][0] = 0x5;
                c.g[A1][0] = Addr(app as u32).off(0x84).lw(c);
                let mgr = c.g[A1][0];
                c.g[V0][0] = Addr(mgr as u32).off(0xc).lw(c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[V0][0] = 0x3;
                if c.g[A0][0] == c.g[V1][0] {
                    break 'b_258f68;
                }
                c.cycles += 2;
                if c.g[A0][0] != c.g[V0][0] {
                    frame.restore(c, S0, SYNC_CHECK_S0);
                    break 'b_259084;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(mgr as u32).off(0x214).lw(c);
                if c.g[V0][0] != c.g[A0][0] {
                    frame.restore(c, S0, SYNC_CHECK_S0);
                    break 'b_259084;
                }
            }
            let comm = Addr(r32(c, S0));
            c.cycles += 3;
            c.g[V0][0] = comm.off(SYNC_CHECK_READY).lw(c);
            if c.g[V0][0] == 0u64 {
                frame.restore(c, S0, SYNC_CHECK_S0);
                break 'b_259084;
            }
            c.cycles += 3;
            c.g[V0][0] = comm.off(SYNC_CHECK_CLOSED).lw(c);
            if c.g[V0][0] != 0u64 {
                frame.restore(c, S0, SYNC_CHECK_S0);
                break 'b_259084;
            }
            c.cycles += 3;
            c.g[V0][0] = comm.off(0x0).lw(c);
            if c.g[V0][0] == 0u64 {
                frame.restore(c, S0, SYNC_CHECK_S0);
                break 'b_259084;
            }
            c.cycles += 3;
            c.g[V0][0] = comm.off(SYNC_CHECK_DESYNCED).lw(c);
            if c.g[V0][0] != 0u64 {
                frame.restore(c, S0, SYNC_CHECK_S0);
                break 'b_259084;
            }
            // Ring slot ((frame / 60) % 60): two divides, quotient then
            // remainder, through LO/HI exactly as the original. The first
            // divide sits in its own +5 block; both generated arms break
            // out of it, so only the break is kept.
            'b_258fac: {
                c.cycles += 5;
                c.g[A0][0] = SYNC_CHECK_RING_LEN as u64;
                c.g[V1][0] = SYNC_CHECK_RING_LEN as u64;
                c.div(0, A3, A0);
                break 'b_258fac;
            }
            'b_259020: {
                'b_25901c: {
                    'b_258ff4: {
                        c.cycles += 12;
                        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(SYNC_CHECK_RING));
                        c.g[S1][0] = c.g[A1][0];
                        c.g[V1][0] = c.lo[0];
                        c.div(0, V1, A0);
                        c.g[V0][0] = c.hi[0];
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        let slot = Addr(c.g[A1][0] as u32);
                        slot.sw(c, c.g[A2][0]);
                        comm.off(SYNC_CHECK_FRAME_NO).sw(c, c.g[A3][0]);
                        c.g[V0][0] = comm.off(SYNC_CHECK_SENT).lw(c);
                        let need_send = c.g[V0][0] == 0u64;
                        c.g[V0][0] = 0x4;
                        if need_send {
                            break 'b_258ff4;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = comm.off(SYNC_CHECK_REPLIED).lw(c);
                        if c.g[V0][0] == 0u64 {
                            c.g[A0][0] = comm.off(SYNC_CHECK_FRAME_NO).lw(c);
                            break 'b_259020;
                        }
                        c.cycles += 3;
                        comm.off(SYNC_CHECK_REPLIED).sw(c, 0u64);
                        comm.off(SYNC_CHECK_SENT).sw(c, 0u64);
                        break 'b_25901c;
                    }
                    c.cycles += 10;
                    Addr(sp(c).wrapping_add(0x4)).sw(c, c.g[A3][0]);
                    Addr(sp(c).wrapping_add(0x8)).sw(c, c.g[A2][0]);
                    c.g[A0][0] = c.g[S0][0];
                    Addr(sp(c)).sb(c, c.g[V0][0]);
                    c.g[A1][0] = 0x3;
                    c.g[A2][0] = sx(sp(c));
                    c.g[A3][0] = 0xc;
                    callee::C_GAME_COMM_SEND_PACKET_2.call(c, 0x0025_9014)?;
                    c.g[V1][0] = 0x1;
                    comm.off(SYNC_CHECK_SENT).sw(c, c.g[V1][0]);
                }
                c.cycles += 1;
                c.g[A0][0] = comm.off(SYNC_CHECK_FRAME_NO).lw(c);
            }
            c.cycles += 2;
            if (c.g[A0][0] as i64) < 0 {
                frame.restore(c, S0, SYNC_CHECK_S0);
                break 'b_259084;
            }
            'b_259080: {
                c.cycles += 3;
                c.g[V1][0] = comm.off(SYNC_CHECK_MIN_FRAME).lw(c);
                let min_pending = (c.g[V1][0] as i64) < 0;
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                if min_pending {
                    break 'b_259080;
                }
                'b_259044: {
                    c.cycles += 4;
                    c.g[A1][0] = SYNC_CHECK_RING_LEN as u64;
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = c.g[A0][0];
                    }
                    break 'b_259044;
                }
                c.cycles += 14;
                c.div(0, V1, A1);
                c.g[V0][0] = SYNC_CHECK_RING_LEN as u64;
                c.g[V1][0] = SYNC_CHECK_RING_LEN as u64;
                c.g[A0][0] = SYNC_CHECK_RING_LEN as u64;
                c.g[V1][0] = c.lo[0];
                c.div(0, V1, A1);
                c.g[V0][0] = c.hi[0];
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A1][0] = Addr(c.g[A0][0] as u32).lw(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(SYNC_CHECK_PEER_RING).lw(c);
                let same = c.g[A1][0] == c.g[V1][0];
                c.g[V0][0] = 0x1;
                if same {
                    break 'b_259080;
                }
                c.cycles += 1;
                comm.off(SYNC_CHECK_DESYNCED).sw(c, c.g[V0][0]);
                break 'b_259080;
            }
            c.cycles += 1;
            frame.restore(c, S0, SYNC_CHECK_S0);
        }
        c.cycles += 4;
        frame.restore(c, S1, SYNC_CHECK_S1);
        frame.restore_ra(c, SYNC_CHECK_RA);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_paths;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    /// Every value the game's direct callers pass, plus zero, negative and
    /// signed-boundary values.
    const TIMEOUT_ARGUMENTS: [u32; 8] =
        [0, 1, 1800, 3600, 3660, u32::MAX, 0x8000_0000, 0x7fff_ffff];

    /// Fixture objects behind `gp - 0x848`.
    const APP: u32 = DATA + 0x1000;
    const GAME_MODULE: u32 = DATA + 0x2000;
    const RACE_MANAGER: u32 = DATA + 0x3000;

    /// Flag bits of a `peer_timeout_setup` seed.
    const FLAG_LOCAL_PAUSED: u32 = 1 << 0;
    const FLAG_PEER_PAUSED: u32 = 1 << 1;
    const FLAG_COUNTDOWN: u32 = 1 << 2;
    const FLAG_RACE: u32 = 1 << 3;
    const FLAG_DESYNCED: u32 = 1 << 4;
    /// Counts seeds per counter row in `peer_timeout_setup`.
    const FLAG_COMBINATIONS: u32 = 32;

    /// The seed that runs counter row `case` with `flags` set.
    fn peer_seed(case: u32, flags: u32) -> u32 {
        case * FLAG_COMBINATIONS + flags
    }

    /// Seed bits 0 to 4 set the `FLAG_*` fields; `seed / FLAG_COMBINATIONS`
    /// picks the counter row: zero, below each limit, at it, or `u32::MAX`.
    fn peer_timeout_setup(c: &mut Cpu, seed: u32) {
        use game_comm::*;
        let mut rng = Rng(0x257a_8000 ^ seed as u64);
        scramble(c, &mut rng);
        c.g[GP][0] = sx(GAME_GP);
        c.g[A0][0] = sx(DATA);
        c.bus.write32(GAME_GP - RACE_WORLD_GP, APP);
        c.bus.write32(APP + RACE_WORLD_GAME_MODULE, GAME_MODULE);
        c.bus.write32(GAME_MODULE + AI_RACE_MANAGER, RACE_MANAGER);
        // Countdown, Race, then EndRace (which does not score lag).
        c.bus.write32(RACE_MANAGER, RACE_STATE_COUNTDOWN + seed % 3);
        // EndSession's already-closed path is bounded, but still executes the
        // real callee and its clearing stores. LatchResultFlags is real too.
        c.bus.write32(DATA + SESSION_CLOSED, 1);
        c.bus.write32(DATA + ABORT_REASON, (seed % 31 == 30) as u32);
        for (bit, field) in [
            LOCAL_PAUSED,
            PEER_PAUSED,
            COUNTDOWN_STARTED,
            RACE_ACTIVE,
            DESYNCED,
        ]
        .into_iter()
        .enumerate()
        {
            c.bus.write32(DATA + field, (seed >> bit) & 1);
        }
        c.bus
            .write32(GAME_GP + GP_TRAY_OPENED as u32, (seed % 11 == 0) as u32);
        c.bus.write32(
            DATA + RESUME_GRACE,
            if seed % 7 == 0 {
                1
            } else if seed % 7 == 1 {
                // Just expired: the grace check flips to the late path.
                0
            } else {
                u32::MAX
            },
        );
        // Counter values either side of each limit, in step across counters.
        let case = (seed / FLAG_COMBINATIONS) as usize;
        c.bus.write32(
            DATA + LATE_TICKS,
            [0, LAG_ONSET_TICKS - 1, LAG_ONSET_TICKS, u32::MAX][case],
        );
        c.bus.write32(
            DATA + PEER_SILENT_TICKS,
            [
                0,
                PEER_WAIT_NOTICE_TICKS - 1,
                PEER_WAIT_NOTICE_TICKS,
                u32::MAX,
            ][case],
        );
        c.bus.write32(
            DATA + PEER_NO_RESPONSE_TICKS,
            [
                0,
                PEER_NO_RESPONSE_LIMIT - 1,
                PEER_NO_RESPONSE_LIMIT,
                u32::MAX,
            ][case],
        );
        c.bus.write32(DATA + LAG_SCORE, 7.0f32.to_bits());
        c.bus.write32(DATA + LAG_EVENTS, 2);
    }

    #[test]
    fn c_game_comm_check_peer_timeout_matches() {
        check_paths(
            ssx3_ee::s06::f_00257a80__generated,
            c_game_comm_check_peer_timeout,
            128,
            // Reached costs. A cost can be shared by several paths, so
            // `c_game_comm_check_peer_timeout_arms` asserts what the main arms
            // write.
            &[10, 16, 21, 30, 33, 45, 49, 62, 91, 103],
            &peer_timeout_setup,
        );
    }

    /// The fields each arm writes, from its fixture values.
    #[test]
    fn c_game_comm_check_peer_timeout_arms() {
        use crate::difftest::fresh;
        use crate::difftest::run_generated;
        use game_comm::*;
        #[rustfmt::skip]
        let rows: [(u32, [u32; 3], u32, f32, u32, u32, u32, Option<u64>); 13] = [
            // seed, [late, silent, no-response], lag events, score, notice,
            // timed out, abort reason, cost (where the writes do not tell)
            (peer_seed(0, FLAG_COUNTDOWN), [1, 1, 0], 2, 7.0, 0, 0, 0, None),
            (peer_seed(1, FLAG_COUNTDOWN), [10, 60, 1799], 3, 8.0, 1, 0, 0, None),
            (peer_seed(0, FLAG_RACE), [0, 0, 1], 2, 7.0, 0, 0, 0, None),
            (peer_seed(1, FLAG_RACE), [9, 59, 1800], 2, 7.0, 0, 1, 0, None),
            (peer_seed(2, FLAG_PEER_PAUSED | FLAG_COUNTDOWN), [10, 60, 1800], 2, 7.0, 1, 0, 0, None),
            (peer_seed(0, FLAG_RACE | FLAG_DESYNCED), [0, 0, 0], 2, 7.0, 0, 0, 0, Some(91)),
            (peer_seed(0, FLAG_LOCAL_PAUSED), [0, 0, 0], 2, 7.0, 0, 0, 0, Some(16)),
            (peer_seed(0, 0), [0, 0, 0], 2, 7.0, 0, 0, 1, None),
            (peer_seed(0, FLAG_DESYNCED), [0, 0, 0], 2, 7.0, 0, 0, 0, Some(21)),
            // State 6 (EndRace) does not score. The notice this arm sets is
            // cleared again by EndSession.
            (peer_seed(1, FLAG_COUNTDOWN | FLAG_RACE), [10, 60, 1800], 2, 7.0, 0, 1, 0, None),
            // Seed 84 is in resume grace: nothing counts, but the notice shows.
            (peer_seed(2, FLAG_COUNTDOWN | FLAG_DESYNCED), [10, 60, 1800], 2, 7.0, 1, 0, 0, None),
            // Tray opened, reached after the race checks find nothing to end.
            (peer_seed(3, FLAG_PEER_PAUSED | FLAG_COUNTDOWN | FLAG_RACE), [u32::MAX; 3], 2, 7.0, 0, 0, 1, None),
            // Seed 30 has an abort reason already, so it leaves at once.
            (peer_seed(0, FLAG_PEER_PAUSED | FLAG_COUNTDOWN | FLAG_RACE | FLAG_DESYNCED), [0, 0, 0], 2, 7.0, 0, 0, 1, Some(10)),
        ];
        for (seed, counters, events, score, notice, timed_out, abort, cost) in rows {
            let mut c = fresh();
            peer_timeout_setup(&mut c, seed);
            let start = c.cycles;
            run_generated(&mut c, c_game_comm_check_peer_timeout);
            let spent = c.cycles - start;
            let read = |c: &mut Cpu, field| c.bus.read32(DATA + field);
            assert_eq!(read(&mut c, LATE_TICKS), counters[0], "{seed}");
            assert_eq!(read(&mut c, PEER_SILENT_TICKS), counters[1], "{seed}");
            assert_eq!(read(&mut c, PEER_NO_RESPONSE_TICKS), counters[2], "{seed}");
            assert_eq!(read(&mut c, LAG_EVENTS), events, "{seed}");
            assert_eq!(read(&mut c, LAG_SCORE), score.to_bits(), "{seed}");
            assert_eq!(read(&mut c, SHOW_PEER_WAIT), notice, "{seed}");
            assert_eq!(read(&mut c, TIMED_OUT), timed_out, "{seed}");
            assert_eq!(read(&mut c, ABORT_REASON), abort, "{seed}");
            if let Some(cost) = cost {
                assert_eq!(spent, cost, "{seed}");
            }
        }
    }

    #[test]
    fn c_game_comm_set_timeout_timer_matches() {
        // Cartesian coverage of every flag combination and every argument:
        // that is what proves the outcomes, since costs 9 and 13 each cover
        // two of them.
        check_paths(
            ssx3_ee::s06::f_00258870__generated,
            c_game_comm_set_timeout_timer,
            128,
            &[4, 9, 10, 13],
            &|c, seed| {
                let mut rng = Rng(0x2588_7000 ^ seed as u64);
                scramble(c, &mut rng);
                let flags = seed / TIMEOUT_ARGUMENTS.len() as u32;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(TIMEOUT_ARGUMENTS[seed as usize % TIMEOUT_ARGUMENTS.len()]);
                // `force` set only in the upper half: the compare is 64-bit.
                c.g[A2][0] = if flags & 8 != 0 { 0x1_0000_0000 } else { 0 };
                c.bus.write32(DATA + game_comm::LOCAL_PAUSED, flags & 1);
                c.bus
                    .write32(DATA + game_comm::PEER_PAUSED, (flags >> 1) & 1);
                c.bus
                    .write32(DATA + game_comm::COUNTDOWN_STARTED, (flags >> 2) & 1);
                c.bus.write32(DATA + game_comm::SESSION_DEADLINE, 777);
            },
        );
    }

    #[test]
    fn c_game_comm_update_max_latency_matches() {
        check(
            ssx3_ee::s06::f_00258330,
            c_game_comm_update_max_latency,
            300,
            &|c, seed| {
                let mut r = Rng(0x2583_3000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                let world = DATA + 0x400;
                c.bus.write32(GAME_GP - 0x848, world);
                // Seeds 0 and 1 force the FPU compare both ways; the rest fuzz.
                let (a, b, limit) = match seed % 3 {
                    0 => (1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits()),
                    1 => (5.0f32.to_bits(), 4.0f32.to_bits(), 1.0f32.to_bits()),
                    _ => (r.edge_f32(), r.edge_f32(), r.edge_f32()),
                };
                c.bus.write32(DATA + 0x114, a);
                c.bus.write32(DATA + 0x118, b);
                c.bus.write32(world + 0x14, limit);
                c.bus.write32(DATA + 0x11c, r.next_u32());
            },
        );
    }
    use crate::difftest::check_paths_lohi;
    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes;
    use crate::difftest::RA as SENTINEL_RA;
    pub(crate) use crate::game::fe::debugfe::tests::SELF;
    pub(crate) use crate::game::fe::fepopupmisc::tests::base_machine_cleared as base;
    pub(crate) use crate::game::fe::fepopupmisc::tests::clear_words;
    pub(crate) use crate::game::fe::fepopupmisc::tests::comms_2 as comms;
    pub(crate) use crate::game::fe::fepopupmisc::tests::COMMS_A;
    pub(crate) use crate::game::fe::fepopupmisc::tests::COMMS_B;
    pub(crate) use crate::game::fe::fepopupmisc::tests::FAKE_CTRL;
    pub(crate) use crate::game::fe::fepopupmisc::tests::FIX;
    pub(crate) use crate::game::fe::fepopupmisc::tests::LOBBY;
    pub(crate) use crate::game::fe::fepopupmisc::tests::SEEDS_13 as SEEDS;
    pub(crate) use crate::game::misc::tests::alloc;
    pub(crate) use crate::game::misc::tests::install_fakes_2 as install_fakes_3;
    pub(crate) use crate::game::misc::tests::str;
    pub(crate) use crate::game::misc::tests::FAKE_MAGIC0;
    use ssx3_ee_rt::cpu::sx;

    pub(crate) const FAKE_MAGIC9: u32 = 0x0c11_8901;

    pub(crate) fn gate_2(
        c: &mut Cpu,
        slot: u32,
        prev: &'static std::sync::atomic::AtomicUsize,
        generated: fn(&mut Cpu),
    ) -> bool {
        if c.bus.read32(FAKE_CTRL) != FAKE_MAGIC9 {
            fakes::fall_back(c, prev, generated);
            return false;
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, slot, ra);
        true
    }

    /// `CGameComm_CheckPeerTimeout` logging the call and returning: the
    /// caller ignores the result (always continues to SetState).
    pub(crate) fn check_peer_timeout_fake(c: &mut Cpu) {
        gate_2(
            c,
            0x0025_7a80,
            fakes::prev_cell(check_peer_timeout_fake),
            ee::s06::f_00257a80__generated,
        );
    }

    /// `DirtyRpc_Call` / `DirtyRpc_InitRequest` logging the call and
    /// returning: the real call waits on a live peer (socket/file
    /// queue) via `EAFile_AllocRequest`, which never completes
    /// in-test. Reached through the real `ProtoPingRequest` on the
    /// no-response path (which has no override slot). The caller
    /// ignores the result.
    pub(crate) fn dirty_rpc_call_fake(c: &mut Cpu) {
        gate_2(
            c,
            0x003f_4c68,
            fakes::prev_cell(dirty_rpc_call_fake),
            ee::s14::f_003f4c68__generated,
        );
    }

    pub(crate) fn dirty_rpc_init_fake(c: &mut Cpu) {
        gate_2(
            c,
            0x003f_4c98,
            fakes::prev_cell(dirty_rpc_init_fake),
            ee::s14::f_003f4c98__generated,
        );
    }

    /// Twins of the DirtyRpc fakes gating on `FAKE_MAGIC1` (the bare
    /// names gate on `FAKE_MAGIC9`): identical bodies, for fixtures
    /// that arm the MAGIC1 gate.
    pub(crate) fn dirty_rpc_call_fake_1(c: &mut Cpu) {
        gate_4(
            c,
            0x003f_4c68,
            fakes::prev_cell(dirty_rpc_call_fake_1),
            ee::s14::f_003f4c68__generated,
        );
    }

    pub(crate) fn dirty_rpc_init_fake_1(c: &mut Cpu) {
        gate_4(
            c,
            0x003f_4c98,
            fakes::prev_cell(dirty_rpc_init_fake_1),
            ee::s14::f_003f4c98__generated,
        );
    }

    /// Twins of the file-queue fakes gating on `FAKE_MAGIC9`:
    /// identical bodies, for fixtures that arm the MAGIC9 gate.
    pub(crate) fn bolts_read_fake_9(c: &mut Cpu) {
        gate_2(
            c,
            0x0014_be70,
            fakes::prev_cell(bolts_read_fake_9),
            ee::s01::f_0014be70__generated,
        );
    }

    pub(crate) fn queue_open_fake_9(c: &mut Cpu) {
        gate_2(
            c,
            0x003d_d4e8,
            fakes::prev_cell(queue_open_fake_9),
            ee::s14::f_003dd4e8__generated,
        );
    }

    /// Twins of the file-queue fakes gating on `FAKE_MAGIC1` (same reason).
    pub(crate) fn bolts_read_fake_1(c: &mut Cpu) {
        gate_4(
            c,
            0x0014_be70,
            fakes::prev_cell(bolts_read_fake_1),
            ee::s01::f_0014be70__generated,
        );
    }

    pub(crate) fn queue_open_fake_1(c: &mut Cpu) {
        gate_4(
            c,
            0x003d_d4e8,
            fakes::prev_cell(queue_open_fake_1),
            ee::s14::f_003dd4e8__generated,
        );
    }

    pub(crate) fn install_fakes_2() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s06::OVERRIDE_00257A80),
            check_peer_timeout_fake,
            fakes::prev_cell(check_peer_timeout_fake),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003F4C68),
            dirty_rpc_call_fake,
            fakes::prev_cell(dirty_rpc_call_fake),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003F4C98),
            dirty_rpc_init_fake,
            fakes::prev_cell(dirty_rpc_init_fake),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s01::OVERRIDE_0014BE70),
            bolts_read_fake_9,
            fakes::prev_cell(bolts_read_fake_9),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003DD4E8),
            queue_open_fake_9,
            fakes::prev_cell(queue_open_fake_9),
        );
    }

    /// `Update` fixture: the cleared game-comm block; the countdown,
    /// state (all sixteen arms plus out-of-range), switch selector and
    /// A-timer idle word are seed-steered (fully crossed); the B-timer
    /// stays idle and the econ words are fixed. The channel array
    /// behind the comms block routes vtable calls to the miss-hook
    /// fake. Seeds 128+ steer the ping section: a planted ping reply
    /// (response with or without a room-name match), the ping-loop bound
    /// and the countdown words. Seeds 0-127 keep the cleared block, so
    /// every previously pinned path total stays covered; the new seeds
    /// only add totals.
    pub(crate) fn update_fixture(c: &mut Cpu, seed: u32) {
        // Installed here (not in the events wrapper) so every test
        // using this fixture sees the same slot, regardless of test
        // thread order; the per-machine flag gates the behavior.
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_00416B18),
            strncmp_scan_fake,
            fakes::prev_cell(strncmp_scan_fake),
        );
        let _r = base(c, seed, 0x0025_70d8);
        elf_image(c);
        install_fakes_2();
        comms(c);
        clear_words(c, COMMS_A, 0x100);
        clear_words(c, COMMS_B, 0x100);
        clear_words(c, LOBBY, 0x100);
        c.bus.write32(FAKE_CTRL, FAKE_MAGIC9);
        let arr = COMMS_B;
        clear_words(c, arr, 0x10);
        for i in 0..3u32 {
            let item = FIX + 0x3100 + i * 0x40;
            clear_words(c, item, 0x10);
            let vt = FIX + 0x3300 + i * 0x40;
            clear_words(c, vt, 0x10);
            c.bus.write32(item + 8, vt);
            c.bus.write16(vt + 0x28, 0);
            c.bus.write32(vt + 0x2c, fakes::FAKE);
            c.bus.write32(arr + i * 4, item);
        }
        c.g[A0][0] = SELF as u64;
        c.bus.write32(SELF + 0x138, (seed % 3) as u32);
        c.bus.write32(SELF + 0x70, (seed / 3 % 18) as u32);
        c.bus.write32(SELF + 0x44, (seed / 54 % 2) as u32);
        c.bus.write32(SELF + 0x6c, 0);
        c.bus.write32(SELF + 0x124, (seed / 108 % 2) as u32);
        c.bus.write32(SELF + 0x3ec, 0x10);
        c.bus.write32(SELF + 0x3f0, 0x20);
        // Ping-section steering (a sparse seed subset, so every previously
        // pinned path total stays covered): the countdown word, the
        // ping-loop bound, and a planted ping reply. The reply struct
        // points at a header (word 0 nonzero = response) whose payload
        // after it is copied onto the stack, so STRNCMP compares the
        // payload against the ELF room table (zeros and pointers).
        // Match seeds copy that table (and steer the loop-match word
        // to the header's word 2); the re-arm flag stays set so no
        // RPC fires. One match seed also zeroes the loop bound for the
        // boundary mutant.
        if seed % 16 == 4 {
            let sg = seed / 16 % 5;
            if sg == 0 {
                c.bus.write32(SELF + 0xbc, 7);
            } else if sg == 1 {
                c.bus.write32(SELF + 0xb8, 4);
            } else if sg == 2 {
                c.bus.write32(SELF + 0xb8, 2);
            } else if sg >= 3 {
                const REPLY: u32 = FIX + 0x3600;
                const HDR: u32 = FIX + 0x3700;
                clear_words(c, REPLY, 0x10);
                clear_words(c, HDR, 0x10);
                c.bus.write32(REPLY, 0x20);
                c.bus.write32(REPLY + 4, 1);
                c.bus.write32(REPLY + 0x10, HDR);
                if sg == 4 {
                    c.bus
                        .write32(SELF + 0xb8, if seed % 3 == 1 { 0 } else { 5 });
                    c.bus.write32(HDR, 1);
                    c.bus.write32(HDR + 4, 0);
                    c.bus.write32(HDR + 8, 0x42);
                    c.bus.write32(HDR + 12, 0);
                    c.bus.write32(HDR + 16, 0x5058_5353);
                    c.bus.write32(HDR + 20, 0x5367_6e69);
                    c.bus.write32(HDR + 24, 0x6572_7665);
                    c.bus.write32(HDR + 28, 0x0000_0072);
                    c.bus.write32(SELF + 0xd0, 0x42);
                } else {
                    c.bus.write32(HDR, 1);
                }
                c.bus.write32(SELF + 0x140, REPLY);
            }
        }
        // Ping-match scan steering (the 0x25712c-0x257180 scan and the
        // 0x257158 unwind; old seeds keep scrambled A1 and never
        // match): S == 1 matches on the second slot, S == 2 runs to
        // the bound. A1 points at a match string so STRNCMP hits
        // zero; the S0+0xc4 array and the SP+0x108 word steer the
        // scan length; +0xb8 bounds it. Only the four scan seeds
        // below reach the check.
        // The +0x3fc countdown is steered positive on its own digit.
        // Scan seeds are four purpose-built Unwind seeds (planted
        // ring, clear flag): 34/46 match at k=1, 70/82 run to the
        // bound. Nothing else is repurposed, so the pinned path
        // totals below do not move. (Scan skips the 0x257110 unwind
        // via the fixture flag below so the VBlank reaches 0x257158;
        // the reply-steered %16==4 seeds keep proving 0x257110.)
        let scan = if seed == 34 || seed == 46 {
            1
        } else if seed == 70 || seed == 82 {
            2
        } else {
            0
        };
        // Positive countdown rides on two of the repurposed scan
        // seeds (no additional path totals move).
        if seed == 34 {
            c.bus.write32(SELF + 0x3fc, 1);
        } else if seed == 46 {
            c.bus.write32(SELF + 0x3fc, 5);
        }
        // The ping countdown at +0x138 starts at 2 on the matching
        // seeds, so the 0x2570fc decrement leaves 1 and the 0x257178
        // clear changes the word.
        if scan == 1 {
            c.bus.write32(SELF + 0x138, 2);
        }
        if scan != 0 {
            // The scan runs only when the ping response is nonzero,
            // so plant the reply/header pair too (sg==3 style).
            const REPLY: u32 = FIX + 0x3600;
            const HDR: u32 = FIX + 0x3700;
            clear_words(c, REPLY, 0x10);
            clear_words(c, HDR, 0x10);
            c.bus.write32(REPLY, 0x20);
            c.bus.write32(REPLY + 4, 1);
            c.bus.write32(REPLY + 0x10, HDR);
            c.bus.write32(HDR, 1);
            c.bus.write32(SELF + 0x140, REPLY);
            // The compare itself is faked to zero on scan seeds (see
            // the hook below): the real stack buffer is wiped after
            // the fixture and the response clobbers entry A1, so no
            // planted input can steer the real compare; the scan arms
            // (the actual test target) run identically either way.
            let sp = c.g[SP][0] as u32;
            let sbuf = sp.wrapping_sub(0x160);
            for (i, &b) in b"SSXPingServer".iter().enumerate() {
                c.bus.write8(sbuf + i as u32, b);
            }
            c.bus.write8(sbuf + 13, 0);
            const M: u32 = 0x5001;
            for i in 0..4u32 {
                c.bus.write32(
                    SELF + 0xc4 + i * 4,
                    if scan == 1 && i == 1 { M } else { 0x5000 + i },
                );
            }
            // The ping response copies the 16-byte header to `sp + 0x100`,
            // so the scan's `sp + 0x108` word is `HDR + 8`.
            c.bus.write32(HDR + 8, if scan == 1 { M } else { 0x6000 });
            c.bus.write32(SELF + 0xb8, 4);
            // Tell the event hook to skip the 0x257110 unwind on
            // scan seeds so the VBlank reaches the scan instead.
            c.bus.write32(FIX + 0x3a00, SCAN_SEED);
        } else {
            c.bus.write32(FIX + 0x3a00, 0);
        }
        // (+0x3fc countdown lives on scan seeds above.)
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = 0x0025_70d8;
    }

    #[test]
    fn c_game_comm_update_matches() {
        // Pinned totals after the scan/countdown steering (scan seeds
        // 34/46 match at k=1, 70/82 run to the bound; countdown on
        // 34/46): the dropped totals' arms are re-proven by the
        // mutation sweep, not by cycle counts.
        const PATHS: &[u64] = &[
            13493, 13504, 13505, 13526, 13527, 13528, 13533, 13540, 13541, 13542, 13552, 13553,
            13562, 13568, 13574, 13576, 13578, 13579, 13601, 13602, 13603, 13608, 13615, 13616,
            13625, 13627, 13629, 13696, 13729, 13752, 13757, 13802, 13811, 13892, 15664, 15670,
            15677, 15682, 15703, 15704, 15705, 15710, 15717, 15718, 15719, 15729, 15730, 15732,
            15738, 15744, 15746, 15747, 15771, 15772, 15773, 15778, 15779, 15785, 15786, 15792,
            15794, 15796, 15797, 15798, 15803, 15810, 15831, 15832, 15833, 15838, 15845, 15846,
            15847, 15853, 15857, 15859, 15865, 15871, 15873, 15876, 15877, 15898, 15899, 15900,
            15905, 15912, 15913, 15924, 15925, 19030, 19031, 19105, 19106, 19234, 21207, 21275,
            21276, 21335, 21336, 21402, 21403, 515001, 517113, 517173, 517292, 517332,
        ];
        check_paths(
            ee::s06::f_002570d8__generated,
            c_game_comm_update,
            SEEDS,
            PATHS,
            &update_fixture,
        );
    }

    driven!(c_game_comm_update_generated, ee::s06::f_002570d8__generated);

    driven!(c_game_comm_update_hand, c_game_comm_update);

    /// Marks an `Update` scan seed at `FIX + 0x3a00`. A magic word, not
    /// just nonzero: other fixtures (`ExchangeSetupData` runs the real
    /// `Update`) leave data there and must keep the real `STRNCMP`.
    pub(crate) const SCAN_SEED: u32 = 0x5ca2_5eed;

    /// `STRNCMP` returning zero on scan seeds: the real stack buffer
    /// is wiped after the fixture and the ping response clobbers the
    /// entry A1, so no planted input can steer the real compare; the
    /// scan arms under test run identically with a zero result.
    /// Other seeds fall back to the real body.
    pub(crate) fn strncmp_scan_fake(c: &mut Cpu) {
        if c.bus.read32(FIX + 0x3a00) != SCAN_SEED {
            return fakes::fall_back(
                c,
                fakes::prev_cell(strncmp_scan_fake),
                ee::s15::f_00416b18__generated,
            );
        }
        c.g[V0][0] = 0;
        c.pc = c.g[RA][0] as u32;
    }

    /// Event hook for the `Update` test: service only the two
    /// back-edge heads (`0x257110`, `0x257158`); every other check
    /// reports no event without servicing, so nested loops can
    /// neither consume the VBlank nor unwind first. Scan seeds skip
    /// the `0x257110` unwind (fixture flag) so the VBlank survives
    /// to the scan's own `0x257158` check.
    pub(crate) fn update_rerun_event(c: &mut Cpu) -> bool {
        if c.pc != 0x0025_7110 && c.pc != 0x0025_7158 {
            return false;
        }
        if c.pc == 0x0025_7110 && c.bus.read32(FIX + 0x3a00) == SCAN_SEED {
            return false;
        }
        if c.cycles >= crate::difftest::CYCLE_BUDGET {
            panic!(
                "cycle budget exceeded ({} cycles, pc {:#x}): runaway on test data",
                c.cycles, c.pc
            );
        }
        (ssx3_ps2::platform::HOOKS.event)(c)
    }

    /// `Update` event test: the ping-response unwind at `0x257110`
    /// and the ping-match scan unwind at `0x257158` must both fire;
    /// the resume map is recorded for the branch table. The schedule
    /// lives in this wrapper (not the shared fixture) so the
    /// `check_paths` totals above stay on deterministic cycles. The
    /// pc-gated hook services events only at the two heads: nested
    /// checks report "no event" without servicing, so no nested
    /// unwind can re-enter the top hand at a nested pc.
    #[test]
    fn c_game_comm_update_events_matches() {
        let resumes = check_events(
            c_game_comm_update_generated,
            c_game_comm_update_hand,
            SEEDS,
            &|c, seed| {
                update_fixture(c, seed);
                let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ (seed as u64) << 8 ^ 0x257108);
                let mode = schedule(c, &mut r, seed, 0x0025_70d8);
                if mode == Events::Unwind {
                    c.next_event = c.cycles;
                    ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                        .sched
                        .next_vblank = c.cycles;
                    c.hooks.event = update_rerun_event;
                }
                mode
            },
        );
        for head in [0x0025_7110, 0x0025_7158] {
            assert!(
                resumes.get(&head).copied().unwrap_or(0) > 0,
                "no unwind at head {head:#x} (resumes {resumes:x?})",
            );
        }
    }

    /// Game-comm block helper: the arena block constructed by the real
    /// fellow ctor (needs the comms block, the BE library dummy and
    /// the DirtyRpc fakes for the ping subtree).
    pub(crate) fn gcomm_block(c: &mut Cpu, seed: u32, a0: u32, a1: u32) -> u32 {
        let _r = base(c, seed, 0x0025_6c78);
        elf_image(c);
        install_fakes_3();
        comms(c);
        c.bus.write32(FAKE_CTRL, FAKE_MAGIC0);
        let this = alloc(c, 0x600);
        clear_words(c, this, 0x180);
        c.g[A0][0] = this as u64;
        c.g[A1][0] = a0 as u64;
        c.g[A2][0] = a1 as u64;
        c.g[RA][0] = sx(0x0025_6ca4);
        callee::C_GAME_COMM_CTOR.call(c, 0x0025_6ca4).unwrap();
        this
    }

    /// `Create` fixture: the stored words are seed-steered small ints;
    /// the real allocator + fellow ctor run, publishing behind gp.
    pub(crate) fn gcomm_create_fixture(c: &mut Cpu, seed: u32) {
        let _r = base(c, seed, 0x0025_6bf8);
        elf_image(c);
        install_fakes_3();
        comms(c);
        c.bus.write32(FAKE_CTRL, FAKE_MAGIC0);
        c.g[A0][0] = (seed % 2) as u64;
        c.g[A1][0] = (seed / 2 % 2) as u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = 0x0025_6bf8;
    }

    #[test]
    fn c_game_comm_create_matches() {
        const PATHS: &[u64] = &[2962];
        check_paths(
            ee::s06::f_00256bf8__generated,
            c_game_comm_create,
            SEEDS,
            PATHS,
            &gcomm_create_fixture,
        );
    }

    /// `Destroy` fixture: a null gp slot takes the early return, a
    /// constructed block the real fellow dtor (delete arm, `A1 = 3`).
    pub(crate) fn gcomm_destroy_fixture(c: &mut Cpu, seed: u32) {
        if seed % 2 == 0 {
            let _r = base(c, seed, 0x0025_6c50);
            elf_image(c);
            install_fakes_3();
            comms(c);
            c.bus.write32(GAME_GP.wrapping_sub(0x204), 0);
        } else {
            let this = gcomm_block(c, seed, 0, 0);
            c.bus.write32(GAME_GP.wrapping_sub(0x204), this);
        }
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = 0x0025_6c50;
    }

    #[test]
    fn c_game_comm_destroy_matches() {
        const PATHS: &[u64] = &[7, 510];
        check_paths(
            ee::s06::f_00256c50__generated,
            c_game_comm_destroy,
            SEEDS,
            PATHS,
            &gcomm_destroy_fixture,
        );
    }

    /// `SetState` fixture: the cleared block; the state word crosses
    /// all sixteen arms plus out-of-range (the jump table lives in the
    /// image). Callees run real on the fixture blocks.
    pub(crate) fn set_state_fixture(c: &mut Cpu, seed: u32) {
        let _r = base(c, seed, 0x0025_7038);
        elf_image(c);
        install_fakes_3();
        comms(c);
        clear_words(c, COMMS_A, 0x100);
        let this = FIX + 0x3400;
        clear_words(c, this, 0x180);
        c.g[A0][0] = this as u64;
        c.g[A1][0] = (seed % 18) as u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = 0x0025_7038;
    }

    #[test]
    fn c_game_comm_set_state_matches() {
        const PATHS: &[u64] = &[15, 22, 25, 36, 55];
        check_paths(
            ee::s06::f_00257038__generated,
            c_game_comm_set_state,
            SEEDS,
            PATHS,
            &set_state_fixture,
        );
    }

    /// `ResetRaceSync` fixture: `A1` plus the `+0x4`/`+0x70`/`+0x38`
    /// words cross every guard arm (the comms words stay cleared).
    pub(crate) fn reset_fixture(c: &mut Cpu, seed: u32) {
        let _r = base(c, seed, 0x0025_7610);
        elf_image(c);
        install_fakes_3();
        comms(c);
        clear_words(c, COMMS_A, 0x100);
        let this = FIX + 0x3400;
        clear_words(c, this, 0x180);
        c.bus.write32(this + 0x4, (seed / 2 % 2) as u32);
        c.bus
            .write32(this + 0x70, if seed / 4 % 2 == 0 { 5 } else { 20 });
        c.bus.write32(this + 0x38, (seed / 8 % 2) as u32);
        c.g[A0][0] = this as u64;
        c.g[A1][0] = (seed % 2) as u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = 0x0025_7610;
    }

    #[test]
    fn c_game_comm_reset_race_sync_matches() {
        const PATHS: &[u64] = &[10, 14, 17, 19, 26];
        check_paths(
            ee::s06::f_00257610__generated,
            c_game_comm_reset_race_sync,
            SEEDS,
            PATHS,
            &reset_fixture,
        );
    }

    /// `LatchResultFlags` fixture: the `+0x30` early-out, `+0xa0` /
    /// `+0xa4` compares and the `+0x70` threshold cross every arm. The
    /// result OR/shift chain also needs nonzero flag words (`|` and
    /// `<<` on all-zero words are invisible): dense words (full bytes,
    /// so the ORs overlap) plus a sparse mode (words of 1, keeping
    /// every shift's bit disjoint from the accumulator for the late
    /// shifts the dense set already covers).
    pub(crate) fn latch_fixture(c: &mut Cpu, seed: u32) {
        let _r = base(c, seed, 0x0025_7678);
        elf_image(c);
        install_fakes_3();
        comms(c);
        let this = FIX + 0x3400;
        clear_words(c, this, 0x180);
        c.bus.write32(this + 0x30, (seed % 2) as u32);
        let dense = seed / 16 % 2 == 1;
        let sparse = !dense && seed / 32 % 2 == 1;
        let spread = if dense { 0xff } else { 1 };
        c.bus.write32(this + 0xa0, (seed / 2 % 2) as u32 * spread);
        c.bus.write32(this + 0xa4, (seed / 4 % 2) as u32 * spread);
        c.bus.write32(
            this + 0x70,
            if seed == 8 {
                15
            } else if seed / 8 % 2 == 0 {
                5
            } else {
                20
            },
        );
        if dense || sparse {
            let w = if dense { 0xff } else { 1 };
            for off in [0x10u32, 0x14, 0x7c, 0x80, 0x48, 0x88, 0x8c, 0x90, 0xac] {
                c.bus.write32(this + off, w);
            }
        }
        c.g[A0][0] = this as u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = 0x0025_7678;
    }

    #[test]
    fn c_game_comm_latch_result_flags_matches() {
        const PATHS: &[u64] = &[6, 48, 50, 53];
        check_paths(
            ee::s06::f_00257678__generated,
            c_game_comm_latch_result_flags,
            SEEDS,
            PATHS,
            &latch_fixture,
        );
    }

    /// `TickCountdowns` fixture: one seed bit per guard word, crossing
    /// every combination of the eight dispatch bits.
    pub(crate) fn tick_fixture(c: &mut Cpu, seed: u32) {
        let _r = base(c, seed, 0x0025_7750);
        elf_image(c);
        install_fakes_3();
        comms(c);
        let this = FIX + 0x3400;
        clear_words(c, this, 0x180);
        c.bus.write32(this + 0x8, (seed % 2) as u32);
        c.bus.write32(this + 0x48, (seed / 2 % 2) as u32);
        c.bus.write32(this + 0x4, (seed / 4 % 2) as u32);
        c.bus.write32(this + 0x80, (seed / 8 % 2) as u32);
        c.bus
            .write32(this + 0x64, if seed / 16 % 2 == 0 { 0 } else { 5 });
        c.bus.write32(this + 0x74, (seed / 32 % 2) as u32);
        c.bus.write32(this + 0x78, (seed / 64 % 2) as u32);
        c.bus
            .write32(this + 0x68, if seed / 128 % 2 == 0 { 0 } else { 5 });
        c.g[A0][0] = this as u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = 0x0025_7750;
    }

    #[test]
    fn c_game_comm_tick_countdowns_matches() {
        const PATHS: &[u64] = &[5, 8, 14, 15, 16, 19, 20, 21, 22, 24, 25, 26, 28, 29];
        check_paths(
            ee::s06::f_00257750__generated,
            c_game_comm_tick_countdowns,
            SEEDS,
            PATHS,
            &tick_fixture,
        );
    }

    pub(crate) const FAKE_MAGIC1: u32 = 0x0c11_9101;

    pub(crate) const FAKE_UIMODE: u32 = FIX + 0xC08;

    pub(crate) fn comms_2(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x848), COMMS_A);
        c.bus.write32(GAME_GP.wrapping_sub(0x238), COMMS_B);
        c.bus.write32(GAME_GP.wrapping_sub(0xc8), LOBBY);
        c.bus.write32(GAME_GP.wrapping_add(0x304), 0);
        c.bus.write32(FIX + 0x3d0, 0);
        // DirtySock already started: the real `NetConn*` calls would
        // otherwise issue an RPC `Call` that never completes in-test.
        c.bus.write32(GAME_GP.wrapping_add(0x1ab0), 1);
    }

    pub(crate) fn gate_4(
        c: &mut Cpu,
        slot: u32,
        prev: &'static std::sync::atomic::AtomicUsize,
        generated: fn(&mut Cpu),
    ) -> bool {
        if c.bus.read32(FAKE_CTRL) != FAKE_MAGIC1 {
            fakes::fall_back(c, prev, generated);
            return false;
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, slot, ra);
        true
    }

    /// Bounded-progress counter for the channel fakes below: while
    /// nonzero, the fake reports a packet (V0 nonzero) and decrements;
    /// at zero it reports empty (V0 zero), so receive/send loops make
    /// progress and then terminate. Fixtures that never set it keep
    /// the historical always-empty behavior exactly.
    pub(crate) const FAKE_PROGRESS: u32 = FIX + 0x3D0;

    /// `cCommSystem_ChannelRecv` returning empty: the vtable fake never
    /// returns 0, which would spin every packet loop forever in-test
    /// (needs a live peer). The real channel path is covered by its
    /// own test; here the empty reply terminates each loop.
    /// With `FAKE_PROGRESS` set, reports a packet that many times first.
    pub(crate) fn channel_recv_zero_fake(c: &mut Cpu) {
        if !gate_4(
            c,
            0x0025_5dc0,
            fakes::prev_cell(channel_recv_zero_fake),
            ee::s06::f_00255dc0__generated,
        ) {
            return;
        }
        if c.pc == c.g[RA][0] as u32 {
            let n = c.bus.read32(FAKE_PROGRESS);
            if n > 0 {
                c.bus.write32(FAKE_PROGRESS, n - 1);
                c.g[V0][0] = 1;
            } else {
                c.g[V0][0] = 0;
            }
        }
    }

    /// `cCommSystem_ChannelSend` reporting nothing sent: same live-peer
    /// reasoning as the receive fake; terminates every send loop.
    /// With `FAKE_PROGRESS` set, reports progress that many times first.
    pub(crate) fn channel_send_zero_fake(c: &mut Cpu) {
        if !gate_4(
            c,
            0x0025_5e00,
            fakes::prev_cell(channel_send_zero_fake),
            ee::s06::f_00255e00__generated,
        ) {
            return;
        }
        if c.pc == c.g[RA][0] as u32 {
            let n = c.bus.read32(FAKE_PROGRESS);
            if n > 0 {
                c.bus.write32(FAKE_PROGRESS, n - 1);
                c.g[V0][0] = 1;
            } else {
                c.g[V0][0] = 0;
            }
        }
    }

    /// `UIStateMgr_GetActiveState` returning an empty (0) or a live
    /// state block: the real walk needs live UI state, which never
    /// exists in-test. Mode 0 falls back to the real body (empty list
    /// on the fixture); modes 1/2 return the scratch block with the
    /// challenge byte forced / cleared, exercising the countdown.
    pub(crate) fn ui_active_fake(c: &mut Cpu) {
        if c.bus.read32(FAKE_UIMODE) == 0 {
            return fakes::fall_back(
                c,
                fakes::prev_cell(ui_active_fake),
                ee::s13::f_0039fa88__generated,
            );
        }
        if !gate_4(
            c,
            0x0039_fa88,
            fakes::prev_cell(ui_active_fake),
            ee::s13::f_0039fa88__generated,
        ) {
            return;
        }
        if c.pc == c.g[RA][0] as u32 {
            let blk = FIX + 0x3E00;
            let challenge = c.bus.read32(FAKE_UIMODE) == 1;
            c.bus.write8(blk + 0x1d, if challenge { 5 } else { 0 });
            c.g[V0][0] = blk as u64;
        }
    }

    pub(crate) fn install_fakes_4() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003F4C68),
            dirty_rpc_call_fake_1,
            fakes::prev_cell(dirty_rpc_call_fake_1),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003F4C98),
            dirty_rpc_init_fake_1,
            fakes::prev_cell(dirty_rpc_init_fake_1),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s01::OVERRIDE_0014BE70),
            bolts_read_fake_1,
            fakes::prev_cell(bolts_read_fake_1),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003DD4E8),
            queue_open_fake_1,
            fakes::prev_cell(queue_open_fake_1),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s06::OVERRIDE_00255DC0),
            channel_recv_zero_fake,
            fakes::prev_cell(channel_recv_zero_fake),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s06::OVERRIDE_00255E00),
            channel_send_zero_fake,
            fakes::prev_cell(channel_send_zero_fake),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_0039FA88),
            ui_active_fake,
            fakes::prev_cell(ui_active_fake),
        );
    }

    driven!(
        c_game_comm_receive_packets_generated,
        ee::s06::f_00257cd8__generated
    );

    driven!(
        c_game_comm_receive_packets_hand,
        c_game_comm_receive_packets
    );

    driven!(
        c_game_comm_process_control_packets_generated,
        ee::s06::f_00257f98__generated
    );

    driven!(
        c_game_comm_process_control_packets_hand,
        c_game_comm_process_control_packets
    );

    driven!(
        c_game_comm_recv_bulk_data_generated,
        ee::s06::f_002581f8__generated
    );

    driven!(c_game_comm_recv_bulk_data_hand, c_game_comm_recv_bulk_data);

    /// Game-comm block helper: the arena block constructed by the real
    /// fellow ctor (needs the comms block, the BE library dummy and
    /// the DirtyRpc fakes for the ping subtree).
    pub(crate) fn gcomm_block_2(c: &mut Cpu, seed: u32, a0: u32, a1: u32) -> u32 {
        let _r = base(c, seed, 0x0025_6c78);
        elf_image(c);
        install_fakes_4();
        comms_2(c);
        c.bus.write32(FAKE_CTRL, FAKE_MAGIC1);
        let this = alloc(c, 0x600);
        clear_words(c, this, 0x180);
        c.g[A0][0] = this as u64;
        c.g[A1][0] = a0 as u64;
        c.g[A2][0] = a1 as u64;
        c.g[RA][0] = sx(0x0025_6ca4);
        callee::C_GAME_COMM_CTOR.call(c, 0x0025_6ca4).unwrap();
        this
    }

    /// Latency channel array: the slot array with item blocks whose
    /// vtable read/call slots route to the miss-hook fake (used by
    /// the real `GetChannelLatency`).
    pub(crate) fn channel_array(c: &mut Cpu) {
        let arr = FIX + 0x3000;
        clear_words(c, arr, 0x10);
        for i in 0..3u32 {
            let item = FIX + 0x3100 + i * 0x40;
            clear_words(c, item, 0x10);
            let vt = FIX + 0x3300 + i * 0x40;
            clear_words(c, vt, 0x14);
            c.bus.write32(item + 8, vt);
            c.bus.write16(vt + 0x20, 0);
            c.bus.write32(vt + 0x24, fakes::FAKE);
            c.bus.write16(vt + 0x28, 0);
            c.bus.write32(vt + 0x2c, fakes::FAKE);
            c.bus.write32(item + 0xc, fakes::FAKE);
            c.bus.write32(arr + i * 4, item);
        }
    }

    /// Ring plant helper: one packet in the ring slot for `s3`, so the
    /// pop path copies exactly once and the next pop sees an empty
    /// ring (tail = head + 1, head = tail - 1 before the copy). The
    /// first copied payload byte is the control type `ty`, which is
    /// what `ProcessControlPackets` dispatches on.
    pub(crate) fn plant_packet(c: &mut Cpu, this: u32, s3: u32, ty: u8) {
        let slot = this + s3 * 0x88 + 0x144;
        let pkt = alloc(c, 0x40);
        c.bus.write16(pkt + 2, 12);
        c.bus.write8(pkt + 4, ty);
        c.bus.write32(slot, 2);
        c.bus.write32(slot + 4, 1);
        c.bus.write32(slot + 12, pkt);
    }

    /// `CheckConnection` fixture: seeds 0..63 run the full-pass
    /// prefix with `+0x94` cycling; seeds 64..255 vary every guard as
    /// an independent bit, so each branch takes both outcomes in many
    /// passing-prefix contexts (single-fail seeds alone cannot reach
    /// the deep guards: reaching `+0x50` needs `+0x54 < 3` *and*
    /// `+0x58 < 6` together).
    pub(crate) fn check_connection_fixture(c: &mut Cpu, seed: u32) {
        let _r = base(c, seed, 0x0025_77c8);
        elf_image(c);
        install_fakes_4();
        comms_2(c);
        c.bus.write32(FAKE_CTRL, FAKE_MAGIC1);
        // The channel-status word and array: cleared, so the real
        // `GetChannelStatus` reads a steered status (the uncleared fill
        // left random words here, so the deep A/B/C/D chain only ran on
        // lucky seeds). `[B+0x28]`: 0 takes the array read (all zeroed,
        // status 0, deep chain); 1 returns status 1 (the Latch path).
        // Non-boundary seeds steer both ways on an independent digit.
        clear_words(c, COMMS_B, 0x100);
        c.bus.write32(
            COMMS_B + 0x28,
            if seed >= 240 {
                0
            } else if seed < 64 || (seed / 32) % 2 == 0 {
                0
            } else {
                1
            },
        );
        let this = FIX + 0x3400;
        clear_words(c, this, 0x180);
        for i in 0..3u32 {
            let item = FIX + 0x3200 + i * 0x40;
            clear_words(c, item, 0x10);
            c.bus.write32(item + 4, 0);
            c.bus.write32(this + i * 4, item);
        }
        let m = if seed < 64 { 0 } else { seed - 64 };
        // Seeds 240..255: dedicated boundary run — every earlier
        // guard passing, +0x54 = 2, +0x58 in {1,2,3} (the value the
        // '< 3' guard actually compares), +0x4c 0 or >= 6 (block-B exit
        // arm), +0x50 in {0,8} (next guard both ways), float both arms.
        // Covers the '< 2' boundary, the +0x94 countdown, and the two
        // deep-exit stores (+0x28 = 3 and +0x24 = 2).
        let bnd = seed >= 240;
        let bi = seed.wrapping_sub(240) as usize;
        c.bus.write32(
            this + 0x8,
            if bnd {
                0
            } else if seed < 64 || m & 1 == 0 {
                0
            } else {
                5
            },
        );
        c.bus.write32(
            this + 0x4,
            if bnd {
                0
            } else if seed < 64 || m & 2 == 0 {
                0
            } else {
                1
            },
        );
        c.bus.write32(
            this + 0x8c,
            if bnd {
                0
            } else if seed < 64 || m & 4 == 0 {
                0
            } else {
                1
            },
        );
        c.bus.write32(
            this + 0x28,
            if bnd {
                0
            } else if seed < 64 || m & 8 == 0 {
                0
            } else {
                1
            },
        );
        c.bus.write32(
            this + 0x54,
            if bnd {
                // Seed 242 steers the 2577 `< 3` boundary (==3 kills `<=`).
                if seed == 242 {
                    3
                } else {
                    2
                }
            } else if seed < 64 || m & 16 == 0 {
                5
            } else {
                2
            },
        );
        c.bus.write32(
            this + 0x4c,
            // Seed 245 steers the 2595 `< 6` boundary (==6 kills `<=`).
            if seed == 245 {
                6
            } else if bnd && bi % 4 == 3 {
                7
            } else {
                0
            },
        );
        c.bus.write32(
            this + 0x58,
            if bnd {
                [1, 2, 3][bi % 3]
            } else if seed < 64 || m & 32 == 0 {
                8
            } else {
                5
            },
        );
        c.bus.write32(
            this + 0x50,
            if bnd {
                // Seed 241 steers the 2631 `< 6` boundary (==6 kills `<=`;
                // 241 reaches it: +0x54 < 3, +0x4c == 0 < 6, +0x58 == 2).
                // (247 was tried first but its +0x4c == 7 exits earlier.)
                if seed == 241 {
                    6
                } else {
                    [0, 8][bi % 2]
                }
            } else if seed < 64 || m & 64 == 0 {
                5
            } else {
                2
            },
        );
        if (bnd && (bi == 0 || bi == 6 || bi == 12)) || (!bnd && seed >= 64 && m & 128 != 0) {
            c.bus.write32(this + 0xb0, 0x40a0_0000);
        } else {
            c.bus.write32(this + 0xb0, 0x45b1_0000);
        }
        c.bus.write32(this + 0x6c, (seed % 3) as u32);
        // Seed 252 steers the 2663 countdown to exactly 1 (post-decrement
        // 0 kills `>=`; 252 reaches it: bi == 12 takes the float arm now).
        c.bus
            .write32(this + 0x94, if seed == 252 { 1 } else { (seed % 8) as u32 });
        c.g[A0][0] = this as u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = 0x0025_77c8;
    }

    #[test]
    fn c_game_comm_check_connection_matches() {
        // Boundary seeds steer the +0x58 guard and the deeper chain.
        const PATHS: &[u64] = &[13, 33, 51, 53, 99, 111, 114, 125, 128, 160, 181, 1676, 1688];
        check_paths(
            ee::s06::f_002577c8__generated,
            c_game_comm_check_connection,
            SEEDS,
            PATHS,
            &check_connection_fixture,
        );
    }

    /// `EndSession` fixture: one seed bit per chain guard; the gp
    /// host word steers the Voip stop, and `+0xac` follows `+0x4`
    /// (irrelevant on the early exit, steering the quit countdown
    /// otherwise).
    pub(crate) fn end_session_fixture(c: &mut Cpu, seed: u32) {
        let _r = base(c, seed, 0x0025_78f8);
        elf_image(c);
        install_fakes_4();
        comms_2(c);
        c.bus.write32(FAKE_CTRL, FAKE_MAGIC1);
        let this = FIX + 0x3400;
        clear_words(c, this, 0x180);
        let host = alloc(c, 0x40);
        c.bus.write32(
            GAME_GP.wrapping_add(0x304),
            if seed / 128 == 0 { 0 } else { host },
        );
        c.bus.write32(this + 0x4, (seed % 2) as u32);
        c.bus.write32(this + 0xc, (seed / 2 % 2) as u32);
        c.bus.write32(this + 0x24, (seed / 4 % 2) as u32);
        c.bus.write32(this + 0x30, (seed / 8 % 2) as u32);
        c.bus.write32(this + 0xa0, (seed / 16 % 2) as u32);
        c.bus.write32(this + 0x88, (seed / 32 % 2) as u32);
        c.bus.write32(this + 0x80, (seed / 64 % 2) as u32);
        c.bus.write32(this + 0xac, (seed / 2 % 2) as u32);
        c.g[A0][0] = this as u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = 0x0025_78f8;
    }

    #[test]
    fn c_game_comm_end_session_matches() {
        const PATHS: &[u64] = &[
            14, 60, 71, 72, 76, 77, 79, 80, 83, 84, 88, 89, 91, 92, 95, 96, 100,
        ];
        check_paths(
            ee::s06::f_002578f8__generated,
            c_game_comm_end_session,
            SEEDS,
            PATHS,
            &end_session_fixture,
        );
    }

    /// `TickQuitCountdown` fixture: the early-out flag, a live
    /// countdown, the UI-state mode, and the interface-chain word.
    /// `+0x8` spans {0, 1, 3}: 0 returns immediately, 3 ticks once,
    /// 1 runs the OpenScreen tail (the tail needs exactly 1).
    pub(crate) fn tick_quit_fixture(c: &mut Cpu, seed: u32) {
        let _r = base(c, seed, 0x0025_79c8);
        elf_image(c);
        install_fakes_4();
        comms_2(c);
        c.bus.write32(FAKE_CTRL, FAKE_MAGIC1);
        clear_words(c, COMMS_A, 0x100);
        let this = FIX + 0x3400;
        clear_words(c, this, 0x180);
        c.bus.write32(this + 0xac, (seed % 2) as u32);
        c.bus
            .write32(this + 0x8, [0, 1, 3][(seed / 2 % 3) as usize]);
        c.bus.write32(FAKE_UIMODE, (seed / 4 % 3) as u32);
        let chain = alloc(c, 0x80);
        c.bus.write32(COMMS_A + 0x84, chain);
        c.bus
            .write32(chain + 0x48, if seed / 16 % 2 == 0 { 0 } else { 8 });
        c.g[A0][0] = this as u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = 0x0025_79c8;
    }

    #[test]
    fn c_game_comm_tick_quit_countdown_matches() {
        const PATHS: &[u64] = &[10, 14, 18, 31, 36, 609];
        check_paths(
            ee::s06::f_002579c8__generated,
            c_game_comm_tick_quit_countdown,
            SEEDS,
            PATHS,
            &tick_quit_fixture,
        );
    }

    /// `PopPacket` fixture: the receive flag, the ring slot selector,
    /// an empty or planted ring, and the timeout word.
    pub(crate) fn pop_packet_fixture(c: &mut Cpu, seed: u32) {
        let _r = base(c, seed, 0x0025_7bd8);
        elf_image(c);
        install_fakes_4();
        comms_2(c);
        c.bus.write32(FAKE_CTRL, FAKE_MAGIC1);
        let dest = alloc(c, 0x200);
        let this = FIX + 0x3400;
        clear_words(c, this, 0x180);
        let s3 = (seed / 2 % 2) as u32;
        c.bus.write32(this + 0x4, (seed % 2) as u32);
        if seed / 4 % 2 == 1 {
            plant_packet(c, this, s3, 0);
        }
        // Seeds 14/6 steer the 2975/2983 boundaries: the body always reads
        // slot s3==1 (this+0x1CC), so only seed%8==6 seeds (planted and
        // s3==1 and [S1+4]==0) reach the shift block with a steered word.
        // -1 kills the 2975 `<=` (V1=(-1<-1) false vs true); 64 kills the
        // 2983 `>>` ((64>>5)=2: 2<<5=64 vs 2>>5=0).
        if seed == 14 || seed == 6 {
            let slot = this + (seed / 2 % 2) * 0x88 + 0x144;
            c.bus
                .write32(slot + 4, if seed == 14 { 0xffff_ffff } else { 64 });
        }
        c.bus
            .write32(this + 0x94, if seed / 8 % 2 == 0 { 0 } else { 5 });
        c.g[A0][0] = this as u64;
        c.g[A1][0] = s3 as u64;
        c.g[A2][0] = dest as u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = 0x0025_7bd8;
    }

    #[test]
    fn c_game_comm_pop_packet_matches() {
        const PATHS: &[u64] = &[19, 561, 849, 854, 862, 867];
        check_paths_lohi(
            ee::s06::f_00257bd8__generated,
            c_game_comm_pop_packet,
            SEEDS,
            PATHS,
            &pop_packet_fixture,
        );
    }

    /// `ReceivePackets` fixture: the present flag, an empty or planted
    /// ring head, and the connection slot (unread under the hooked
    /// channel call, set anyway).
    pub(crate) fn receive_packets_fixture(c: &mut Cpu, seed: u32) -> Events {
        let mut r = base(c, seed, 0x0025_7cd8);
        elf_image(c);
        install_fakes_4();
        comms_2(c);
        c.bus.write32(FAKE_CTRL, FAKE_MAGIC1);
        let this = FIX + 0x3400;
        clear_words(c, this, 0x180);
        c.bus.write32(this + 0x4, (seed % 2) as u32);
        if seed / 2 % 2 == 1 {
            let pkt = alloc(c, 0x100);
            // The packet's first byte picks its ring (0x88 bytes each,
            // from +0x144).
            c.bus.write8(pkt, (seed / 8 % 2) as u8);
            c.bus.write32(this + 0x134, pkt);
        }
        // Boundary plants for the 0x257d00 ring block (its head and
        // tail words). The block runs on the ring-drain path: even seeds
        // ([S0+4]==0 proceed past 0x257d58; odd seeds return early) with
        // a planted packet (seed/2 odd), so plant seeds satisfy
        // seed%4==2. A2 is S0 + lbu(pkt) * 0x88 + 0x144, as the body
        // computes it. Seed 2: diff==31 kills
        // the 3088 `<=` (then jumps away by construction). Seed 6 (65,
        // 40): diff 25 falls through, A1=65 unaligned kills 3091-`==`/`>`
        // and feeds 3108/3110 (V0_in=2). Seed 10: A1==-1 kills 3091-`<=`
        // (and `==`). check_events has no pinned totals.
        if seed == 2 || seed == 6 || seed == 10 {
            let pkt = c.bus.read32(this + 0x134);
            let b = c.bus.read8(pkt) as u32;
            let a2 = this.wrapping_add(b * 0x88).wrapping_add(0x144);
            let (w0, w1) = if seed == 2 {
                (31, 0)
            } else if seed == 6 {
                (65, 40)
            } else {
                (0xffff_ffff, 0)
            };
            c.bus.write32(a2, w0);
            c.bus.write32(a2.wrapping_add(4), w1);
        }
        c.bus.write32(this + 0x6c, (seed / 4 % 3) as u32);
        // Bounded progress on planted+clear seeds: the first receive
        // reports a packet (re-running the drain), then empty.
        if seed % 4 == 2 {
            c.bus.write32(FAKE_PROGRESS, 1);
        }
        c.g[A0][0] = this as u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        let mode = schedule(c, &mut r, seed, 0x0025_7cd8);
        if mode == Events::Unwind {
            // Service events only at the drain back edge (proved
            // below); nested checks report no event without servicing.
            c.next_event = c.cycles;
            ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                .sched
                .next_vblank = c.cycles;
            c.hooks.event = receive_packets_rerun_event;
        }
        mode
    }

    /// Event hook for the `ReceivePackets` test: service only the
    /// drain back edge (`0x257d00`); every other check reports no
    /// event without servicing.
    pub(crate) fn receive_packets_rerun_event(c: &mut Cpu) -> bool {
        if c.pc != 0x0025_7d00 {
            return false;
        }
        if c.cycles >= crate::difftest::CYCLE_BUDGET {
            panic!(
                "cycle budget exceeded ({} cycles, pc {:#x}): runaway on test data",
                c.cycles, c.pc
            );
        }
        (ssx3_ps2::platform::HOOKS.event)(c)
    }

    #[test]
    fn c_game_comm_receive_packets_matches() {
        let resumes = check_events(
            c_game_comm_receive_packets_generated,
            c_game_comm_receive_packets_hand,
            SEEDS,
            &|c, seed| receive_packets_fixture(c, seed),
        );
        assert!(
            resumes.get(&0x0025_7d00).copied().unwrap_or(0) > 0,
            "no unwind at drain head 0x257d00 (resumes {resumes:x?})",
        );
    }

    /// `FlushSendQueue` fixture: an empty queue (tail `+0x368` equal to
    /// head `+0x364`) or one queued packet at ring index 1; the hooked
    /// send reports nothing sent, except on progress seeds, where the
    /// first send goes out (the packet is freed and the queue drains).
    pub(crate) fn flush_fixture(c: &mut Cpu, seed: u32) {
        let _r = base(c, seed, 0x0025_7dc0);
        elf_image(c);
        install_fakes_4();
        comms_2(c);
        c.bus.write32(FAKE_CTRL, FAKE_MAGIC1);
        let this = FIX + 0x3400;
        clear_words(c, this, 0x180);
        if seed % 2 == 0 {
            c.bus.write32(this + 0x368, 0);
            c.bus.write32(this + 0x364, 0);
        } else {
            c.bus.write32(this + 0x368, 1);
            c.bus.write32(this + 0x364, 2);
            let pkt = alloc(c, 0x40);
            c.bus.write16(pkt + 2, 12);
            c.bus.write32(this + 0x370, pkt);
        }
        c.bus.write32(this + 0x4, (seed / 4 % 2) as u32);
        c.bus.write32(this + 0x6c, 0);
        // Bounded progress on queued seeds that stay connected (+4
        // clear): the first send reports progress, then empty. Other
        // seeds keep the always-empty behavior.
        if seed % 8 == 3 {
            c.bus.write32(FAKE_PROGRESS, 1);
        }
        c.g[A0][0] = this as u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = 0x0025_7dc0;
    }

    #[test]
    fn c_game_comm_flush_send_queue_matches() {
        const PATHS: &[u64] = &[18, 22, 47, 239];
        check_paths(
            ee::s06::f_00257dc0__generated,
            c_game_comm_flush_send_queue,
            SEEDS,
            PATHS,
            &flush_fixture,
        );
    }

    /// `SendPacket` fixture: the queued flag, the flag byte, a mapped
    /// source and a short length; queue heads stay equal so the
    /// re-flush loop never runs (needs a live peer).
    pub(crate) fn send_packet_fixture(c: &mut Cpu, seed: u32) {
        let _r = base(c, seed, 0x0025_7e98);
        elf_image(c);
        install_fakes_4();
        comms_2(c);
        c.bus.write32(FAKE_CTRL, FAKE_MAGIC1);
        let this = FIX + 0x3400;
        clear_words(c, this, 0x180);
        let src = alloc(c, 0x40);
        c.bus.write32(this + 0x4, (seed % 2) as u32);
        c.bus.write32(this + 0x6c, 0);
        // Seeds seed%64==0 steer 32-unaligned queue heads (33): the
        // rounding erases the vint difference on aligned heads, so
        // 3396/3398 need A1 unaligned and 3404/3406 need A1 >= 32.
        // Heads stay equal, so the pre-check still passes. (Even seeds:
        // [S1+4]==0 proceeds past the first gate; odd seeds exit early.)
        // Seed 64 steers heads -1/-1: the 3396 `<=` boundary (-1<A1 vs
        // -1<=A1 differ only at A1==-1).
        if seed == 64 {
            c.bus.write32(this + 0x364, 0xffffffff);
            c.bus.write32(this + 0x368, 0xffffffff);
        } else if seed % 64 == 0 {
            c.bus.write32(this + 0x364, 33);
            c.bus.write32(this + 0x368, 33);
        }
        c.g[A0][0] = this as u64;
        c.g[A1][0] = (seed / 2 % 2) as u64;
        c.g[A2][0] = src as u64;
        c.g[A3][0] = 16u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = 0x0025_7e98;
    }

    #[test]
    fn c_game_comm_send_packet_matches() {
        const PATHS: &[u64] = &[21, 586];
        check_paths(
            ee::s06::f_00257e98__generated,
            c_game_comm_send_packet,
            SEEDS,
            PATHS,
            &send_packet_fixture,
        );
    }

    /// `ProcessControlPackets` fixture: the receive flag plus an empty
    /// or planted ring (single packet, so the drain loop terminates).
    /// The dispatch index is the saved-register low byte (zero on the
    /// fresh stack); steer it over the table arms (0-5) plus the
    /// skip-dispatch value (6) on empty-ring seeds. The arm-condition
    /// words ride free seed digits (`+0x74`, `+0x64`, `+0x3c`, `+0x4`,
    /// `+0x70` to the 15 boundary); the two `0x58094`/`0x580e0`-style
    /// unconditional jumps stay untouched (both sides dead).
    /// Index 4 also steers the div-path stack word nonzero.
    pub(crate) fn process_control_fixture(c: &mut Cpu, seed: u32) -> Events {
        let mut r = base(c, seed, 0x0025_7f98);
        elf_image(c);
        install_fakes_4();
        comms_2(c);
        c.bus.write32(FAKE_CTRL, FAKE_MAGIC1);
        let this = FIX + 0x3400;
        clear_words(c, this, 0x180);
        c.bus.write32(this + 0x4, (seed % 2) as u32);
        if seed / 2 % 2 == 1 {
            // The planted type steers the dispatch: 0-5 run the six
            // handler chains, 6 takes the `< 6` bound exit; each
            // crosses the arm-condition words below.
            plant_packet(c, this, 3, (seed / 4 % 7) as u8);
        } else {
            c.bus
                .write8(crate::difftest::SP - 0x30, (seed / 4 % 7) as u8);
            if seed / 4 % 7 == 4 {
                c.bus.write8(crate::difftest::SP - 0x30 + 4, 0x79);
            }
        }
        c.bus.write32(this + 0x74, (seed / 8 % 2) as u32);
        c.bus.write32(this + 0x64, (seed / 16 % 2) as u32 * 5);
        c.bus.write32(this + 0x3c, (seed / 32 % 2) as u32);
        c.bus.write32(this + 0x70, (seed / 128 % 2) as u32 * 15);
        c.g[A0][0] = this as u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        let mode = schedule(c, &mut r, seed, 0x0025_7f98);
        if mode == Events::Unwind {
            // Service events only at the drain back edge
            // (`0x257fb0`, which the test asserts resumes): nested
            // deliver-style checks would otherwise consume the VBlank
            // and reschedule past the check. Non-head calls return
            // "no event" without servicing, so the pre-check path —
            // type-independent — needs no calibration.
            c.next_event = c.cycles;
            ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                .sched
                .next_vblank = c.cycles;
            c.hooks.event = process_control_rerun_event;
        }
        mode
    }

    /// Event hook for the `ProcessControlPackets` drain-back-edge
    /// resume proof: only the `0x257fb0` check is serviced (so its
    /// unwind lands exactly there); every other check reports no
    /// event without servicing.
    pub(crate) fn process_control_rerun_event(c: &mut Cpu) -> bool {
        if c.pc != 0x0025_7fb0 {
            return false;
        }
        if c.cycles >= crate::difftest::CYCLE_BUDGET {
            panic!(
                "cycle budget exceeded ({} cycles, pc {:#x}): runaway on test data",
                c.cycles, c.pc
            );
        }
        (ssx3_ps2::platform::HOOKS.event)(c)
    }

    #[test]
    fn c_game_comm_process_control_packets_matches() {
        let resumes = check_events(
            c_game_comm_process_control_packets_generated,
            c_game_comm_process_control_packets_hand,
            SEEDS,
            &|c, seed| process_control_fixture(c, seed),
        );
        // The drain loop's event check must unwind: the planted
        // packets of every type keep the loop running past the
        // first pop.
        assert!(
            resumes.get(&0x0025_7fb0).copied().unwrap_or(0) > 0,
            "no unwind at drain head 0x257fb0 (resumes {resumes:x?})",
        );
    }

    /// `RecvGamePacket` fixture: the receive flag plus an empty or
    /// planted ring (slot selector fixed to 1 by the caller).
    pub(crate) fn recv_game_packet_fixture(c: &mut Cpu, seed: u32) {
        let _r = base(c, seed, 0x0025_8160);
        elf_image(c);
        install_fakes_4();
        comms_2(c);
        c.bus.write32(FAKE_CTRL, FAKE_MAGIC1);
        let this = FIX + 0x3400;
        clear_words(c, this, 0x180);
        c.bus.write32(this + 0x4, (seed % 2) as u32);
        if seed / 2 % 2 == 1 {
            let dest = alloc(c, 0x200);
            plant_packet(c, this, 1, 0);
            c.g[A1][0] = 1u64;
            c.g[A2][0] = dest as u64;
        } else {
            c.g[A1][0] = 1u64;
            c.g[A2][0] = 0u64;
        }
        c.g[A0][0] = this as u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = 0x0025_8160;
    }

    #[test]
    fn c_game_comm_recv_game_packet_matches() {
        const PATHS: &[u64] = &[29, 849, 1469];
        check_paths(
            ee::s06::f_00258160__generated,
            c_game_comm_recv_game_packet,
            SEEDS,
            PATHS,
            &recv_game_packet_fixture,
        );
    }

    /// `AllocPacket` fixture: no inputs; the real allocator runs on
    /// the DLHeap arena deterministically.
    pub(crate) fn alloc_packet_fixture(c: &mut Cpu, seed: u32) {
        let _r = base(c, seed, 0x0025_8188);
        elf_image(c);
        install_fakes_4();
        comms_2(c);
        c.bus.write32(FAKE_CTRL, FAKE_MAGIC1);
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = 0x0025_8188;
    }

    #[test]
    fn c_game_comm_alloc_packet_matches() {
        const PATHS: &[u64] = &[1209];
        check_paths(
            ee::s06::f_00258188__generated,
            c_game_comm_alloc_packet,
            SEEDS,
            PATHS,
            &alloc_packet_fixture,
        );
    }

    /// `FreePacket` fixture: null takes the same path as a heap block
    /// (no branch); both run the real delete.
    pub(crate) fn free_packet_fixture(c: &mut Cpu, seed: u32) {
        let _r = base(c, seed, 0x0025_81b8);
        elf_image(c);
        install_fakes_4();
        comms_2(c);
        c.bus.write32(FAKE_CTRL, FAKE_MAGIC1);
        let blk = alloc(c, 0x40);
        c.g[A1][0] = if seed % 2 == 0 { 0 } else { blk as u64 };
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = 0x0025_81b8;
    }

    #[test]
    fn c_game_comm_free_packet_matches() {
        const PATHS: &[u64] = &[131, 198];
        check_paths(
            ee::s06::f_002581b8__generated,
            c_game_comm_free_packet,
            SEEDS,
            PATHS,
            &free_packet_fixture,
        );
    }

    /// Timer stub fixtures: the word steers idle vs armed; the
    /// setters take their values from `A1`/`A2`.
    pub(crate) fn timer_fixture(c: &mut Cpu, seed: u32, entry: u32, off: u32) {
        let _r = base(c, seed, entry as u64);
        elf_image(c);
        install_fakes_4();
        comms_2(c);
        let this = FIX + 0x3400;
        clear_words(c, this, 0x180);
        c.bus.write32(this + off, (seed % 2) as u32);
        c.g[A0][0] = this as u64;
        c.g[A1][0] = (seed / 2 % 3) as u64;
        c.g[A2][0] = (seed / 6 % 3) as u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = entry;
    }

    #[test]
    fn c_game_comm_is_timer_a124_idle_matches() {
        const PATHS: &[u64] = &[3];
        check_paths(
            ee::s06::f_002581d8__generated,
            c_game_comm_is_timer_a124_idle,
            SEEDS,
            PATHS,
            &|c, seed| timer_fixture(c, seed, 0x0025_81d8, 0x124),
        );
    }

    #[test]
    fn c_game_comm_set_timer_a124_matches() {
        const PATHS: &[u64] = &[3];
        check_paths(
            ee::s06::f_002581e8__generated,
            c_game_comm_set_timer_a124,
            SEEDS,
            PATHS,
            &|c, seed| timer_fixture(c, seed, 0x0025_81e8, 0x124),
        );
    }

    #[test]
    fn c_game_comm_is_timer_b12c_idle_matches() {
        const PATHS: &[u64] = &[3];
        check_paths(
            ee::s06::f_00258280__generated,
            c_game_comm_is_timer_b12c_idle,
            SEEDS,
            PATHS,
            &|c, seed| timer_fixture(c, seed, 0x0025_8280, 0x12c),
        );
    }

    #[test]
    fn c_game_comm_set_timer_b12c_matches() {
        const PATHS: &[u64] = &[3];
        check_paths(
            ee::s06::f_00258290__generated,
            c_game_comm_set_timer_b12c,
            SEEDS,
            PATHS,
            &|c, seed| timer_fixture(c, seed, 0x0025_8290, 0x12c),
        );
    }

    /// `RecvBulkData` fixture: the remaining count, the chunk clamp
    /// and the timeout word; the ring holds one planted packet (slot
    /// selector fixed to 2 by the caller), so the pull loop runs it
    /// once and exits on the empty ring. `+0x4` stays clear so the pop
    /// takes the receive path (the hooked channel reports nothing).
    pub(crate) fn recv_bulk_fixture(c: &mut Cpu, seed: u32) -> Events {
        let mut r = base(c, seed, 0x0025_81f8);
        elf_image(c);
        install_fakes_4();
        comms_2(c);
        c.bus.write32(FAKE_CTRL, FAKE_MAGIC1);
        let this = FIX + 0x3400;
        clear_words(c, this, 0x180);
        c.bus
            .write32(this + 0x124, if seed % 2 == 0 { 0 } else { 0x100 });
        c.bus
            .write32(this + 0x128, if seed / 2 % 2 == 0 { 100 } else { 300 });
        // Seed 5 steers the 3912 `< 237` boundary (==237 kills `<=`;
        // check_events has no pinned totals, so any seed is safe).
        if seed % 32 == 5 {
            c.bus.write32(this + 0x128, 237);
        }
        c.bus.write32(this + 0x4, 0);
        plant_packet(c, this, 2, 0);
        c.bus
            .write32(this + 0x94, if seed / 4 % 2 == 0 { 0 } else { 5 });
        c.g[A0][0] = this as u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        schedule(c, &mut r, seed, 0x0025_81f8)
    }

    #[test]
    fn c_game_comm_recv_bulk_data_matches() {
        check_events(
            c_game_comm_recv_bulk_data_generated,
            c_game_comm_recv_bulk_data_hand,
            SEEDS,
            &|c, seed| recv_bulk_fixture(c, seed),
        );
    }

    /// `SendBulkData` fixture: the send cursor (null skips, a heap
    /// block sends), the remaining count and the queued flag; queue
    /// heads stay equal so the re-flush loop never runs.
    pub(crate) fn send_bulk_fixture(c: &mut Cpu, seed: u32) {
        let _r = base(c, seed, 0x0025_82a0);
        elf_image(c);
        install_fakes_4();
        comms_2(c);
        c.bus.write32(FAKE_CTRL, FAKE_MAGIC1);
        let this = FIX + 0x3400;
        clear_words(c, this, 0x180);
        let src = alloc(c, 0x200);
        c.bus
            .write32(this + 0x12c, if seed % 2 == 0 { 0 } else { src });
        c.bus
            .write32(this + 0x130, if seed / 2 % 2 == 0 { 64 } else { 300 });
        c.bus.write32(this + 0x4, (seed / 4 % 2) as u32);
        c.bus.write32(this + 0x6c, 0);
        c.g[A0][0] = this as u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = 0x0025_82a0;
    }

    #[test]
    fn c_game_comm_send_bulk_data_matches() {
        const PATHS: &[u64] = &[12, 46, 1034, 2281];
        check_paths(
            ee::s06::f_002582a0__generated,
            c_game_comm_send_bulk_data,
            SEEDS,
            PATHS,
            &send_bulk_fixture,
        );
    }

    /// `GetInputDelay` fixture: the latency channel slot plus cleared
    /// float state; the slot words point at mapped items so the real
    /// `GetChannelLatency` reaches the vtable fake (an unmapped slot
    /// would read zeros and never steer).
    pub(crate) fn input_delay_fixture(c: &mut Cpu, seed: u32) {
        let _r = base(c, seed, 0x0025_8360);
        elf_image(c);
        install_fakes_4();
        comms_2(c);
        c.bus.write32(FAKE_CTRL, FAKE_MAGIC1);
        clear_words(c, COMMS_B, 0x100);
        channel_array(c);
        // `GetChannelLatency` reads its array at `COMMS_B` (gp-0x238),
        // not at the helper's array: link the slots at the items.
        for i in 0..3u32 {
            c.bus.write32(COMMS_B + i * 4, FIX + 0x3100 + i * 0x40);
        }
        let this = FIX + 0x3400;
        clear_words(c, this, 0x180);
        c.g[A0][0] = this as u64;
        c.bus.write32(this + 0x6c, (seed % 3) as u32);
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = 0x0025_8360;
    }

    #[test]
    fn c_game_comm_get_input_delay_matches() {
        const PATHS: &[u64] = &[33, 34, 35];
        check_paths(
            ee::s06::f_00258360__generated,
            c_game_comm_get_input_delay,
            SEEDS,
            PATHS,
            &input_delay_fixture,
        );
    }

    /// `PingServer` fixture: the countdown either skips the ping or
    /// runs the real `ProtoPingRequest` (RPC faked); the slot index
    /// and the comms word steer the tail arms.
    pub(crate) fn ping_server_fixture(c: &mut Cpu, seed: u32) {
        let _r = base(c, seed, 0x0025_83a8);
        elf_image(c);
        install_fakes_4();
        comms_2(c);
        c.bus.write32(FAKE_CTRL, FAKE_MAGIC1);
        let this = gcomm_block_2(c, seed, 0, 0);
        c.bus
            .write32(this + 0x138, if seed % 2 == 0 { 0 } else { 5 });
        c.bus.write32(this + 0xb8, (seed / 8 % 2) as u32);
        c.bus.write32(COMMS_B + 0x2c, (seed / 16 % 2) as u32);
        c.g[A0][0] = this as u64;
        c.g[A1][0] = (seed / 2 % 2) as u64;
        c.g[A2][0] = (seed / 4 % 2) as u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = 0x0025_83a8;
    }

    #[test]
    fn c_game_comm_ping_server_matches() {
        const PATHS: &[u64] = &[17, 20, 82, 85];
        check_paths_lohi(
            ee::s06::f_002583a8__generated,
            c_game_comm_ping_server,
            SEEDS,
            PATHS,
            &ping_server_fixture,
        );
    }

    /// `ExchangeSetupData` fixture: the constructed block with the
    /// session-wait short-circuited (`+0x70 == 0xe`, which needs a live
    /// session to clear) and the profile-present global steering the
    /// pack path (the swap path needs live pad state). Plain `check`
    /// (no schedule): the BE subtree's event checks would derail the
    /// event driver on unwind seeds, so event resumption there is a
    /// documented fixture gap; the wait loop itself never runs in-test.
    pub(crate) fn exchange_setup_fixture(c: &mut Cpu, seed: u32) {
        let _r = base(c, seed, 0x0025_8448);
        elf_image(c);
        install_fakes_4();
        comms_2(c);
        c.bus.write32(FAKE_CTRL, FAKE_MAGIC1);
        let this = gcomm_block_2(c, seed, 0, 0);
        // Seed 9 steers the profile-absent arm (0x25850c): [0x534b38]==0
        // takes the swap path instead of the pack path.
        // +0x70=0 on bnd seed 2 (even, 636937): the wait loop is entered;
        // green passed in 0.57s (loop terminates: UPDATE sets +0x70).
        c.bus.write32(0x0053_4b38, if seed == 9 { 0 } else { 1 });
        // Seeds seed%32==1 steer the 2585d4 dispatch ((V0^0xe)==0):
        // +0x70=0 misses 0xe, reaching 0x2585e0 on odd seeds ([S1+4]!==0
        // exits via 0x2585ec — no wait loop). Covers the 4317/4320/4334
        // arms. 635932 keeps ~119 witnesses.
        // Seed 2 (even, 636937) steers the wait-loop entries (4294/4322):
        // [S1+4]==0 takes the event check → 0x2585c0 UPDATE/YIELD loop.
        // HAZARD: terminates only if UPDATE sets +0x70; green runs under
        // `timeout`, record-why if it hangs (needs live session progress).
        c.bus.write32(
            this + 0x70,
            if seed % 32 == 1 || seed == 2 { 0 } else { 0xe },
        );
        c.bus.write32(this + 0x4, (seed % 2) as u32);
        c.bus.write32(this + 0xec, (seed / 2 % 2) as u32);
        c.bus.write32(this + 0x100, (seed / 4 % 2) as u32);
        if seed / 8 % 2 == 1 {
            str(c, this + 0xf0, b"AAAAAAAAA");
        }
        c.g[A0][0] = this as u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = 0x0025_8448;
    }

    #[test]
    fn c_game_comm_exchange_setup_data_matches() {
        const PATHS: &[u64] = &[635926, 635932, 636932, 636937, 636938];
        check_paths(
            ee::s06::f_00258448__generated,
            c_game_comm_exchange_setup_data,
            SEEDS,
            PATHS,
            &exchange_setup_fixture,
        );
    }

    driven!(c_game_comm_ctor_generated, ee::s06::f_00256c78__generated);

    driven!(c_game_comm_ctor_hand, c_game_comm_ctor);

    driven!(c_game_comm_dtor_generated, ee::s06::f_00256ee0__generated);

    driven!(c_game_comm_dtor_hand, c_game_comm_dtor);

    /// `ctor` fixture: the arena block; two fixed loops with event
    /// checks; the BE/part state runs real on the fixture blocks.
    pub(crate) fn ctor_fixture(c: &mut Cpu, seed: u32) -> Events {
        let mut r = base(c, seed, 0x0025_6c78);
        elf_image(c);
        install_fakes_2();
        comms(c);
        c.bus.write32(FAKE_CTRL, FAKE_MAGIC0);
        let this = FIX + 0x3400;
        clear_words(c, this, 0x180);
        c.g[A0][0] = this as u64;
        c.g[A1][0] = (seed % 2) as u64;
        c.g[A2][0] = (seed / 2 % 2) as u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        schedule(c, &mut r, seed, 0x0025_6c78)
    }

    #[test]
    fn c_game_comm_ctor_matches() {
        check_events(
            c_game_comm_ctor_generated,
            c_game_comm_ctor_hand,
            SEEDS,
            &|c, seed| ctor_fixture(c, seed),
        );
    }

    /// `dtor` fixture: the constructed block; the free-list head, the
    /// `+0x3ec` free, the ping handle and the delete arm are
    /// seed-steered (16 combos). Packet words stay zeroed so the free
    /// loop takes the null-packet path (a wild walk would read
    /// unmapped memory on both sides identically, so it stays out).
    /// The block is built by the real ctor, which reaches the DirtyRpc
    /// slots through `ProtoPingCreate`: install the `FAKE_MAGIC0` rpc
    /// fakes first, so both machines take the same (fake) path even
    /// when another test hooks them mid-run (hook-once chains are
    /// process-global; a hook landing between the two setups used to
    /// diverge the heaps and the fake log at seed 0).
    pub(crate) fn dtor_fixture_2(c: &mut Cpu, seed: u32) -> Events {
        let mut r = base(c, seed, 0x0025_6ee0);
        elf_image(c);
        install_fakes_2();
        install_fakes_3();
        comms(c);
        c.bus.write32(FAKE_CTRL, FAKE_MAGIC0);
        let this = alloc(c, 0x600);
        c.bus.write32(FIX + 0x3f0, seed);
        clear_words(c, this, 0x180);
        c.g[A0][0] = this as u64;
        c.g[A1][0] = 0u64;
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x0025_6ca4);
        callee::C_GAME_COMM_CTOR.call(c, 0x0025_6ca4).unwrap();
        // Heap blocks for the steered frees (a double free of one
        // block would corrupt the DLHeap on both sides identically,
        // so each site gets its own).
        let blk134 = alloc(c, 0x40);
        let blk3ec = alloc(c, 0x40);
        c.bus
            .write32(this + 0x134, if seed % 2 == 0 { 0 } else { blk134 });
        c.bus
            .write32(this + 0x3ec, if seed / 2 % 2 == 0 { 0 } else { blk3ec });
        if seed / 4 % 2 == 0 {
            c.bus.write32(this + 0x140, 0);
        }
        // Walk-steering plants (check_events: no pinned totals, any seed
        // is safe). The 256f8c scan reads [A1],[A1+4] with A1 = V0+S0
        // +0x144 (V0 is 0 at entry: ctor returns it cleared); the 256fac
        // word comes from [A1+8]. Steering that chain:
        // - [this+0x144]=64, [this+0x148]=-1: scan true (0xFFFFFFFF<64);
        //   2072-`<=`/`==` diverge on A0==-1; 2060-`>` and 2070 fire.
        // - [this+0x14c]=W (nonzero): loop entry; walk base becomes
        //   W+this+0x144, so each seed gets its own walk addresses.
        // - Pattern N (seeds 7,8): [base]=64,[base+4]=-1 kills 2036-`<=`,
        //   `==` and 2037 at the walk. Pattern U (13,14): 64/65 kills
        //   2036-`>`, 2043, 2045. Pattern R (19,20): A0=64 at 256fac
        //   kills 2077/2079. Unwind (%3==1: 7,13,19) + Deliver (8,14,20)
        //   cover the 2101 unwind-divergence and the free loop.
        // - 2028 (walk arm) and 2073 (always-flipping `!=`) kill on any
        //   seed that reaches their blocks.
        let w = match seed {
            7 | 8 => 0x100,
            13 | 14 => 0x200,
            19 | 20 => 0x300,
            // 2036-`<` kill seeds: N-copy with [this+0x148]=-2. Visit 1
            // then has A1=-2 (S3 stays -1): orig V1=0, mut V1=1, and the
            // R-chain differs (orig R=-2, mut R=30) so the mutant frees
            // a different block. Fresh seeds: zero risk to the N/U/R
            // kills that rely on -1/1/64.
            21 | 22 | 23 => 0x100,
            _ => 0,
        };
        if w != 0 {
            c.bus
                .write32(this + 0x144, if seed >= 19 { 128 } else { 64 });
            c.bus.write32(
                this + 0x148,
                if seed == 7 || seed == 8 {
                    0xffff_ffff
                } else if seed == 13 || seed == 14 {
                    1
                } else if seed == 19 || seed == 20 {
                    64
                } else if seed == 21 || seed == 22 || seed == 23 {
                    0xffff_fffe
                } else {
                    0
                },
            );
            c.bus.write32(this + 0x14c, w);
            let base = this.wrapping_add(w).wrapping_add(0x144);
            let (bw0, bw1) = if seed == 13 || seed == 14 || seed >= 19 {
                (64, 65)
            } else {
                (64, 0xffff_ffff)
            };
            c.bus.write32(base, bw0);
            c.bus.write32(base.wrapping_add(4), bw1);
        }
        // The 256fac V0-chain lands off-FIX: steer the loop word through
        // real RAM at its computed address (no absolute assumptions:
        // replicate the body's u64 arithmetic on `this`, which the
        // fixture knows; V0_entry==0 after FREE_PACKET is confirmed by
        // the 2072 kills landing on [this+0x148]).
        // V0_loop=0x100 puts the walk A3 at this+0x244 for seeds 19,20,
        // so their walk words live there (not at base): A1=65 kills `>`.
        if seed == 19 || seed == 20 {
            let a1 = this as u64 + 0x144;
            let v0 = 64u64;
            let v0 = ((v0 as u32 as i32 >> 5) as u32) as u64;
            let v0 = v0 << 5;
            let v0 = a1.wrapping_sub(v0);
            let v0 = v0 << 2;
            let vfac = a1.wrapping_add(v0);
            let raddr = (vfac as u32).wrapping_add(8);
            c.bus.write32(raddr, 0x100);
            c.bus.write32(this + 0x244, 64);
            c.bus.write32(this + 0x248, 65);
        }
        // Seed 14 (Deliver + ring: walks freely, no unwind-return): same
        // treatment with its own A0=0 chain (V0 stays 0 through the
        // downshift). TIMEOUT-GUARDED: forcing V0_loop constant risks a
        // non-terminating drain loop if nothing ever reads 0 (the walk
        // writes FIX, never the RAM word). On hang (RC=124) revert this
        // block and record-why.
        if seed == 14 {
            let a1 = this as u64 + 0x144;
            let v0 = 0u64;
            let v0 = ((v0 as u32 as i32 >> 5) as u32) as u64;
            let v0 = v0 << 5;
            let v0 = a1.wrapping_sub(v0);
            let v0 = v0 << 2;
            let vfac = a1.wrapping_add(v0);
            let raddr = (vfac as u32).wrapping_add(8);
            c.bus.write32(raddr, 0x100);
            c.bus.write32(this + 0x244, 64);
            c.bus.write32(this + 0x248, 65);
        }
        c.g[A0][0] = this as u64;
        c.g[A1][0] = (seed / 8 % 2) as u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        schedule(c, &mut r, seed, 0x0025_6ee0)
    }

    #[test]
    fn c_game_comm_dtor_matches() {
        check_events(
            c_game_comm_dtor_generated,
            c_game_comm_dtor_hand,
            SEEDS,
            &|c, seed| dtor_fixture_2(c, seed),
        );
    }
    use crate::difftest::fakes::machine;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::lobbyrig_arm;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::lobbyrig_mail;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The `cGameComm` under test.
    pub(crate) const GCOMM_FIX_COMM: u32 = DATA + 0x1000;

    /// The world at `gp - 0x848`.
    pub(crate) const GCOMM_FIX_WORLD: u32 = DATA + 0x2000;

    /// The comm system at `gp - 0x238` (send-path channels).
    pub(crate) const GCOMM_FIX_COMMSYS: u32 = DATA + 0x3000;

    /// The lobby manager at `gp - 0xc8`.
    pub(crate) const GCOMM_FIX_LOBBY: u32 = DATA + 0x4000;

    /// A live buddy manager for `gp + 0x238`.
    pub(crate) const GCOMM_FIX_BUDDY: u32 = DATA + 0x5000;

    /// The string `cGameComm_SendString` sends.
    pub(crate) const GCOMM_FIX_STR: u32 = DATA + 0x6000;

    /// The popup vtable the quit path calls through.
    pub(crate) const GCOMM_FIX_VT: u32 = DATA + 0x7000;

    /// Answer the `BuddyMan_IsConnected` fake reports.
    pub(crate) const GCOMM_FIX_BUDDY_ANSWER: u32 = DATA + 0x000e_3100;

    /// Pointer the `BELibrary_GetInterface` fake reports.
    pub(crate) const GCOMM_FIX_BE_IFACE: u32 = DATA + 0x000e_3200;

    /// Pointer the `BELibrary_GetInstance` fake reports.
    pub(crate) const GCOMM_FIX_BE: u32 = DATA + 0x000e_3300;

    pub(crate) static GCOMM_FIX_PREV_SET_TIMEOUT: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static GCOMM_FIX_PREV_CREATE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static GCOMM_FIX_PREV_IS_CONNECTED: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static GCOMM_FIX_PREV_RECONNECT: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static GCOMM_FIX_PREV_DISCONNECT: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static GCOMM_FIX_PREV_PRESENCE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static GCOMM_FIX_PREV_GET_INTERFACE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static GCOMM_FIX_PREV_YIELD: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static GCOMM_FIX_PREV_GET_BE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static GCOMM_FIX_PREV_COPY_PROFILE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn gcomm_fix_gated(c: &mut Cpu) -> bool {
        c.bus.read32(GCOMM_FIX_CTRL) == GCOMM_FIX_MAGIC
    }

    pub(crate) fn gcomm_fix_fake_set_timeout(c: &mut Cpu) {
        if !gcomm_fix_gated(c) {
            return fakes::fall_back(
                c,
                &GCOMM_FIX_PREV_SET_TIMEOUT,
                ee::s06::f_00258870__generated,
            );
        }
        fakes::fake_call(c, 0x0025_8870);
    }

    pub(crate) fn gcomm_fix_fake_create(c: &mut Cpu) {
        if !gcomm_fix_gated(c) {
            return fakes::fall_back(c, &GCOMM_FIX_PREV_CREATE, ee::s06::f_00260f80__generated);
        }
        fakes::fake_call(c, 0x0026_0f80);
    }

    pub(crate) fn gcomm_fix_fake_is_connected(c: &mut Cpu) {
        if !gcomm_fix_gated(c) {
            return fakes::fall_back(
                c,
                &GCOMM_FIX_PREV_IS_CONNECTED,
                ee::s06::f_00261460__generated,
            );
        }
        fakes::fake_call(c, 0x0026_1460);
        c.g[V0][0] = c.bus.read32(GCOMM_FIX_BUDDY_ANSWER) as u64;
    }

    pub(crate) fn gcomm_fix_fake_reconnect(c: &mut Cpu) {
        if !gcomm_fix_gated(c) {
            return fakes::fall_back(c, &GCOMM_FIX_PREV_RECONNECT, ee::s06::f_002613c8__generated);
        }
        fakes::fake_call(c, 0x0026_13c8);
    }

    pub(crate) fn gcomm_fix_fake_disconnect(c: &mut Cpu) {
        if !gcomm_fix_gated(c) {
            return fakes::fall_back(
                c,
                &GCOMM_FIX_PREV_DISCONNECT,
                ee::s06::f_0025b800__generated,
            );
        }
        fakes::fake_call(c, 0x0025_b800);
    }

    pub(crate) fn gcomm_fix_fake_presence(c: &mut Cpu) {
        if !gcomm_fix_gated(c) {
            return fakes::fall_back(c, &GCOMM_FIX_PREV_PRESENCE, ee::s07::f_00262768__generated);
        }
        fakes::fake_call(c, 0x0026_2768);
    }

    pub(crate) fn gcomm_fix_fake_get_interface(c: &mut Cpu) {
        if !gcomm_fix_gated(c) {
            return fakes::fall_back(
                c,
                &GCOMM_FIX_PREV_GET_INTERFACE,
                ee::s01::f_0014dd58__generated,
            );
        }
        fakes::fake_call(c, 0x0014_dd58);
        c.g[V0][0] = c.bus.read32(GCOMM_FIX_BE_IFACE) as u64;
    }

    /// `THREAD_yieldticks` (0x003e52b0): the real body runs kernel
    /// thread calls that panic on test machines, so the wait-loop test
    /// stubs it out as a clean return. Gated runs also model the peer
    /// arrival the loop is waiting for: the 3rd tick reports the peer
    /// ready (see the waits test), exactly once.
    pub(crate) fn gcomm_fix_fake_yield(c: &mut Cpu) {
        if !gcomm_fix_gated(c) {
            return fakes::fall_back(c, &GCOMM_FIX_PREV_YIELD, ee::s14::THREAD_yieldticks);
        }
        fakes::fake_call(c, 0x003e_52b0);
        let hits = c.bus.read32(GCOMM_FIX_YIELD_HITS) + 1;
        c.bus.write32(GCOMM_FIX_YIELD_HITS, hits);
        if hits == 3 {
            c.bus.write32(GCOMM_FIX_COMM + 4, 1);
        }
    }

    pub(crate) fn gcomm_fix_fake_get_be(c: &mut Cpu) {
        if !gcomm_fix_gated(c) {
            return fakes::fall_back(c, &GCOMM_FIX_PREV_GET_BE, ee::s01::f_0014dc80__generated);
        }
        fakes::fake_call(c, 0x0014_dc80);
        c.g[V0][0] = c.bus.read32(GCOMM_FIX_BE) as u64;
    }

    pub(crate) fn gcomm_fix_fake_copy_profile(c: &mut Cpu) {
        if !gcomm_fix_gated(c) {
            return fakes::fall_back(
                c,
                &GCOMM_FIX_PREV_COPY_PROFILE,
                ee::s01::f_0014e9c0__generated,
            );
        }
        fakes::fake_call(c, 0x0014_e9c0);
    }

    pub(crate) fn gcomm_fix_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00258870),
            gcomm_fix_fake_set_timeout,
            &GCOMM_FIX_PREV_SET_TIMEOUT,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00260F80),
            gcomm_fix_fake_create,
            &GCOMM_FIX_PREV_CREATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00261460),
            gcomm_fix_fake_is_connected,
            &GCOMM_FIX_PREV_IS_CONNECTED,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_002613C8),
            gcomm_fix_fake_reconnect,
            &GCOMM_FIX_PREV_RECONNECT,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_0025B800),
            gcomm_fix_fake_disconnect,
            &GCOMM_FIX_PREV_DISCONNECT,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00262768),
            gcomm_fix_fake_presence,
            &GCOMM_FIX_PREV_PRESENCE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            gcomm_fix_fake_get_interface,
            &GCOMM_FIX_PREV_GET_INTERFACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            gcomm_fix_fake_get_be,
            &GCOMM_FIX_PREV_GET_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014E9C0),
            gcomm_fix_fake_copy_profile,
            &GCOMM_FIX_PREV_COPY_PROFILE,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E52B0),
            gcomm_fix_fake_yield,
            &GCOMM_FIX_PREV_YIELD,
        );
    }

    /// Distinctive nonzero fill word.
    pub(crate) fn gcomm_fix_word(r: &mut Rng) -> u32 {
        r.next_u32() | 0x0101_0101
    }

    /// The world the comm functions read at `gp - 0x848`.
    pub(crate) fn gcomm_fix_world(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(GAME_GP - 0x848, GCOMM_FIX_WORLD);
        c.bus.write32(GCOMM_FIX_WORLD + 0x10, 60);
        c.bus.write32(GCOMM_FIX_WORLD + 0x18, 1800);
        c.bus.write32(GCOMM_FIX_WORLD + 0x78, 7);
        c.bus.write32(GCOMM_FIX_WORLD + 0x84, 0);
        let _ = r;
    }

    /// The send path: an empty queue and two channels whose send method is
    /// unmapped, so `cCommSystem_ChannelSend` lands on the miss-hook fake and
    /// reports success, and `cGameComm_SendPacket` never reaches the packet
    /// allocator.
    pub(crate) fn gcomm_fix_packet_rig(c: &mut Cpu) {
        c.bus.write32(GCOMM_FIX_COMM + 0x364, 0);
        c.bus.write32(GCOMM_FIX_COMM + 0x368, 0);
        c.bus.write32(GAME_GP - 0x238, GCOMM_FIX_COMMSYS);
        for ch in 0..2u32 {
            c.bus.write32(
                GCOMM_FIX_COMMSYS + 4 * ch,
                GCOMM_FIX_COMMSYS + 0x100 + 0x10 * ch,
            );
            c.bus.write32(
                GCOMM_FIX_COMMSYS + 0x100 + 0x10 * ch + 8,
                GCOMM_FIX_COMMSYS + 0x200 + 0x10 * ch,
            );
            c.bus
                .write16(GCOMM_FIX_COMMSYS + 0x200 + 0x10 * ch + 0x40, 0);
            c.bus
                .write32(GCOMM_FIX_COMMSYS + 0x200 + 0x10 * ch + 0x44, 0x0000_5000);
        }
    }

    #[test]
    fn c_game_comm_send_pause_state_matches() {
        gcomm_fix_install_fakes();
        check(
            ee::s06::f_00258790__generated,
            c_game_comm_send_pause_state,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0025_8790, 2);
                c.pc = 0x0025_8790;
                c.bus.write32(GCOMM_FIX_CTRL, GCOMM_FIX_MAGIC);
                gcomm_fix_world(c, &mut r);
                let comm = GCOMM_FIX_COMM;
                for w in 0..0x140u32 {
                    c.bus.write32(comm + 4 * w, gcomm_fix_word(&mut r));
                }
                // After the fill: the send queue must read empty (heads at
                // +0x364/+0x368), or real FlushSendQueue walks garbage.
                gcomm_fix_packet_rig(c);
                // Closed, gone, then the live pause/resume arms.
                c.bus.write32(
                    comm + 0x4,
                    if seed % 5 == 0 {
                        gcomm_fix_word(&mut r)
                    } else {
                        0
                    },
                );
                c.bus.write32(comm, if seed % 5 == 1 { 0 } else { comm });
                c.g[A0][0] = sx(comm);
                c.g[A1][0] = (seed % 2) as u64;
                c.bus.write32(
                    comm + 0x7c,
                    if seed % 3 == 0 {
                        0
                    } else {
                        gcomm_fix_word(&mut r)
                    },
                );
                c.bus.write32(comm + 0x6c, (seed >> 1) % 2);
                c.bus.write32(comm + 0x48, gcomm_fix_word(&mut r));
            },
        );
    }

    #[test]
    fn c_game_comm_update_idle_kick_timer_matches() {
        check(
            ee::s06::f_002588a8__generated,
            c_game_comm_update_idle_kick_timer,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0025_88a8, 0);
                c.pc = 0x0025_88a8;
                gcomm_fix_world(c, &mut r);
                let comm = GCOMM_FIX_COMM;
                for w in 0..0x140u32 {
                    c.bus.write32(comm + 4 * w, gcomm_fix_word(&mut r));
                }
                let t6 = GCOMM_FIX_COMM + 0x300;
                let t4 = GCOMM_FIX_COMM + 0x380;
                c.bus.write32(GCOMM_FIX_WORLD + 0x84, t6);
                c.bus.write32(t6 + 0x214, if seed % 6 == 0 { 2 } else { 0 });
                c.bus.write32(t6 + 0xc, t4);
                c.bus.write32(
                    comm + 0x4,
                    if seed % 6 == 1 {
                        gcomm_fix_word(&mut r)
                    } else {
                        0
                    },
                );
                c.bus.write32(
                    comm + 0x48,
                    if seed % 6 == 2 {
                        gcomm_fix_word(&mut r)
                    } else {
                        0
                    },
                );
                c.bus.write32(comm, if seed % 6 == 3 { 0 } else { comm });
                // Below, on and above the `< 15` gate on every `% 4`
                // residue, so the buddy block below stays reachable on the
                // seeds that carry its `151` boundary.
                c.bus.write32(
                    comm + 0x70,
                    match seed % 4 {
                        0 => 3,
                        _ => [14, 15, 16][seed as usize % 3],
                    },
                );
                c.bus.write32(t4, if seed % 2 == 0 { 4 } else { 5 });
                c.bus
                    .write32(t4 + 0x1c, [0, 150, 151, 200][(seed >> 1) as usize % 4]);
                c.bus.write32(
                    t4 + 0x18,
                    if (seed >> 2) % 2 == 0 {
                        0
                    } else {
                        gcomm_fix_word(&mut r)
                    },
                );
                c.bus.write32(
                    comm + 0x74,
                    if (seed >> 3) % 2 == 0 {
                        0
                    } else {
                        gcomm_fix_word(&mut r)
                    },
                );
                c.bus.write32(
                    comm + 0x78,
                    if (seed >> 4) % 2 == 0 {
                        0
                    } else {
                        gcomm_fix_word(&mut r)
                    },
                );
                c.g[A0][0] = sx(comm);
            },
        );
    }

    #[test]
    fn c_game_comm_on_race_start_reset_matches() {
        check(
            ee::s06::f_00258950__generated,
            c_game_comm_on_race_start_reset,
            300,
            &|c, seed| {
                let mut r = Rng(0x2589_5000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0025_8950;
                c.g[A0][0] = sx(GCOMM_FIX_COMM);
                for w in [0x48u32, 0x78, 0x84] {
                    c.bus.write32(GCOMM_FIX_COMM + w, gcomm_fix_word(&mut r));
                }
            },
        );
    }

    #[test]
    fn c_game_comm_on_countdown_start_matches() {
        check(
            ee::s06::f_00258968__generated,
            c_game_comm_on_countdown_start,
            300,
            &|c, seed| {
                let mut r = Rng(0x2589_6800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0025_8968;
                c.g[A0][0] = sx(GCOMM_FIX_COMM);
                c.bus.write32(
                    GCOMM_FIX_COMM + 0x70,
                    match seed % 4 {
                        0 => 3,
                        1 => 14,
                        2 => 15,
                        _ => 16,
                    },
                );
                c.bus.write32(GCOMM_FIX_COMM + 0x94, gcomm_fix_word(&mut r));
                c.bus.write32(GCOMM_FIX_COMM + 0x7c, gcomm_fix_word(&mut r));
            },
        );
    }

    #[test]
    fn c_game_comm_on_race_start_matches() {
        gcomm_fix_install_fakes();
        check(
            ee::s06::f_00258988__generated,
            c_game_comm_on_race_start,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0025_8988, 2);
                c.pc = 0x0025_8988;
                c.bus.write32(GCOMM_FIX_CTRL, GCOMM_FIX_MAGIC);
                gcomm_fix_world(c, &mut r);
                let comm = GCOMM_FIX_COMM;
                for w in 0..0x140u32 {
                    c.bus.write32(comm + 4 * w, gcomm_fix_word(&mut r));
                }
                // At the `< 15` boundary the session counts as down, so the
                // buddy block is skipped (this kills the `<=` mutant).
                c.bus.write32(
                    comm + 0x70,
                    match seed % 4 {
                        0 => 20,
                        1 => 15,
                        _ => 5,
                    },
                );
                let buddy = if seed % 3 == 1 { 0 } else { GCOMM_FIX_BUDDY };
                c.bus.write32(GAME_GP + 0x238, buddy);
                c.bus.write32(GCOMM_FIX_BUDDY_ANSWER, (seed >> 2) % 2);
                c.bus.write32(GAME_GP - 0xc8, GCOMM_FIX_LOBBY);
                c.bus
                    .write32(GCOMM_FIX_LOBBY + 0x98, gcomm_fix_word(&mut r));
                c.g[A0][0] = sx(comm);
            },
        );
    }

    #[test]
    fn c_game_comm_on_race_end_matches() {
        gcomm_fix_install_fakes();
        check(
            ee::s06::f_00258a48__generated,
            c_game_comm_on_race_end,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0025_8a48, 1);
                c.pc = 0x0025_8a48;
                c.bus.write32(GCOMM_FIX_CTRL, GCOMM_FIX_MAGIC);
                gcomm_fix_world(c, &mut r);
                let comm = GCOMM_FIX_COMM;
                for w in 0..0x140u32 {
                    c.bus.write32(comm + 4 * w, gcomm_fix_word(&mut r));
                }
                // At the `< 15` boundary the session counts as down, so the
                // state change is skipped (this kills the `<=` mutant).
                c.bus.write32(
                    comm + 0x70,
                    match seed % 4 {
                        0 => 20,
                        1 => 15,
                        _ => 3,
                    },
                );
                c.bus.write32(
                    comm + 0x38,
                    if seed % 3 == 0 {
                        0
                    } else {
                        gcomm_fix_word(&mut r)
                    },
                );
                c.bus.write32(
                    comm + 0x3c,
                    if (seed >> 1) % 2 == 0 {
                        0
                    } else {
                        gcomm_fix_word(&mut r)
                    },
                );
                c.g[A0][0] = sx(comm);
            },
        );
    }

    #[test]
    fn c_game_comm_quit_race_matches() {
        gcomm_fix_install_fakes();
        check(
            ee::s06::f_00258ae0__generated,
            c_game_comm_quit_race,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0025_8ae0, 3);
                c.pc = 0x0025_8ae0;
                c.bus.write32(GCOMM_FIX_CTRL, GCOMM_FIX_MAGIC);
                gcomm_fix_world(c, &mut r);
                let comm = GCOMM_FIX_COMM;
                for w in 0..0x140u32 {
                    c.bus.write32(comm + 4 * w, gcomm_fix_word(&mut r));
                }
                // After the fill: the send queue must read empty (heads at
                // +0x364/+0x368), or real FlushSendQueue walks garbage.
                gcomm_fix_packet_rig(c);
                c.bus.write32(
                    comm + 0x4,
                    if (seed >> 5) % 2 == 0 {
                        0
                    } else {
                        gcomm_fix_word(&mut r)
                    },
                );
                c.bus.write32(comm + 0x6c, (seed >> 3) % 2);
                c.bus
                    .write32(comm + 0x70, if seed % 2 == 0 { 20 } else { 3 });
                c.bus.write32(
                    comm + 0x38,
                    if seed % 3 == 0 {
                        0
                    } else {
                        gcomm_fix_word(&mut r)
                    },
                );
                c.bus.write32(
                    comm + 0x3c,
                    if (seed >> 1) % 2 == 0 {
                        0
                    } else {
                        gcomm_fix_word(&mut r)
                    },
                );
                let buddy = if seed % 3 == 1 { 0 } else { GCOMM_FIX_BUDDY };
                c.bus.write32(GAME_GP + 0x238, buddy);
                c.bus.write32(GCOMM_FIX_BUDDY_ANSWER, (seed >> 2) % 2);
                c.bus.write32(GAME_GP - 0xc8, GCOMM_FIX_LOBBY);
                c.bus
                    .write32(GCOMM_FIX_LOBBY + 0x98, gcomm_fix_word(&mut r));
                // The wait-popup word and the fake interface chain.
                c.bus.write32(0x0053_4b38, (seed >> 4) % 2);
                let iface = GCOMM_FIX_COMM + 0x400;
                c.bus.write32(GCOMM_FIX_BE_IFACE, iface);
                c.bus.write32(GCOMM_FIX_BE, GCOMM_FIX_LOBBY);
                c.bus.write32(iface + 0xc, GCOMM_FIX_VT);
                c.bus.write16(GCOMM_FIX_VT + 0x10, 0);
                c.bus.write32(GCOMM_FIX_VT + 0x14, 0x0000_5000);
                c.g[A0][0] = sx(comm);
            },
        );
    }

    #[test]
    fn c_game_comm_send_quit_matches() {
        check(
            ee::s06::f_00258bc8__generated,
            c_game_comm_send_quit,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0025_8bc8, 1);
                c.pc = 0x0025_8bc8;
                gcomm_fix_world(c, &mut r);
                let comm = GCOMM_FIX_COMM;
                for w in 0..0x140u32 {
                    c.bus.write32(comm + 4 * w, gcomm_fix_word(&mut r));
                }
                // After the fill: the send queue must read empty (heads at
                // +0x364/+0x368), or real FlushSendQueue walks garbage.
                gcomm_fix_packet_rig(c);
                c.bus.write32(
                    comm + 0x4,
                    if seed % 2 == 0 {
                        0
                    } else {
                        gcomm_fix_word(&mut r)
                    },
                );
                c.bus.write32(comm + 0x6c, (seed >> 1) % 2);
                c.g[A0][0] = sx(comm);
            },
        );
    }

    #[test]
    fn c_game_comm_send_resync_matches() {
        check(
            ee::s06::f_00258c00__generated,
            c_game_comm_send_resync,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0025_8c00, 1);
                c.pc = 0x0025_8c00;
                gcomm_fix_world(c, &mut r);
                let comm = GCOMM_FIX_COMM;
                for w in 0..0x140u32 {
                    c.bus.write32(comm + 4 * w, gcomm_fix_word(&mut r));
                }
                // After the fill: the send queue must read empty (heads at
                // +0x364/+0x368), or real FlushSendQueue walks garbage.
                gcomm_fix_packet_rig(c);
                c.bus.write32(
                    comm + 0x4,
                    if seed % 3 == 0 {
                        gcomm_fix_word(&mut r)
                    } else {
                        0
                    },
                );
                c.bus.write32(comm, if seed % 3 == 1 { 0 } else { comm });
                c.bus.write32(comm + 0x54, seed);
                c.bus.write32(comm + 0x6c, (seed >> 1) % 2);
                c.g[A0][0] = sx(comm);
            },
        );
    }

    #[test]
    fn c_game_comm_send_string_matches() {
        check(
            ee::s06::f_00258c50__generated,
            c_game_comm_send_string,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0025_8c50, 1);
                c.pc = 0x0025_8c50;
                gcomm_fix_world(c, &mut r);
                let comm = GCOMM_FIX_COMM;
                for w in 0..0x140u32 {
                    c.bus.write32(comm + 4 * w, gcomm_fix_word(&mut r));
                }
                // After the fill: the send queue must read empty (heads at
                // +0x364/+0x368), or real FlushSendQueue walks garbage.
                gcomm_fix_packet_rig(c);
                c.bus.write32(
                    comm + 0x4,
                    if seed % 3 == 0 {
                        gcomm_fix_word(&mut r)
                    } else {
                        0
                    },
                );
                c.bus.write32(comm, if seed % 3 == 1 { 0 } else { comm });
                c.bus.write32(comm + 0x6c, (seed >> 1) % 2);
                let len = seed % 24;
                for i in 0..len {
                    c.bus
                        .write8(GCOMM_FIX_STR + i, 0x41 + ((seed + i) % 26) as u8);
                }
                c.bus.write8(GCOMM_FIX_STR + len, 0);
                c.g[A0][0] = sx(comm);
                c.g[A1][0] = sx(GCOMM_FIX_STR);
            },
        );
    }

    /// Shared fixture: comm object with an empty send queue and unmapped
    /// channels (ChannelSend lands on the miss-hook fake and reports
    /// success), on a miss-hooked machine.
    pub(crate) fn wait_for_peer_setup(c: &mut Cpu, seed: u32, flag_3f4: u32, flag_4: u32) {
        let mut r = fakes::machine(c, seed, 0x0025_86b0, 1);
        wait_for_peer_setup_r(c, seed, &mut r, flag_3f4, flag_4);
    }

    pub(crate) fn wait_for_peer_setup_r(
        c: &mut Cpu,
        seed: u32,
        r: &mut Rng,
        flag_3f4: u32,
        flag_4: u32,
    ) {
        c.pc = 0x0025_86b0;
        gcomm_fix_install_fakes();
        // Arm the gate so the gated callees (yield) take their test
        // doubles instead of the real bodies.
        c.bus.write32(GCOMM_FIX_CTRL, GCOMM_FIX_MAGIC);
        gcomm_fix_world(c, r);
        let comm = GCOMM_FIX_COMM;
        for w in 0..0x140u32 {
            c.bus.write32(comm + 4 * w, gcomm_fix_word(r));
        }
        // After the fill: the send queue must read empty.
        gcomm_fix_packet_rig(c);
        c.bus.write32(comm + 0x3f4, flag_3f4);
        // Pin Update's cheap path (see the waits test): `+0x8 = 1` takes
        // CheckConnection's TickQuitCountdown early-out every pass, and
        // `+0x138` stays positive so every pass stays on the light path.
        c.bus.write32(comm + 0x8, 1);
        c.bus.write32(comm + 0x138, 16);
        c.bus.write32(comm + 0x4, flag_4);
        // Steer the entry dispatch: a set word takes the 0x002586ec
        // send-packet arm, a clear word skips it. The entry reads
        // `lw(0x530000 + 0x4b30 + 8)`.
        c.bus.write32(0x0053_4B30 + 0x8, (seed % 2 == 0) as u32);
        // The gated yield fake counts ticks here; the ready report lands on
        // the 3rd tick (reset per side by the test setup).
        c.bus.write32(GCOMM_FIX_YIELD_HITS, 0);
        c.bus.write32(comm + 0x6c, (seed >> 1) % 2);
        c.g[A0][0] = sx(comm);
    }

    /// Both exit arms, no loop: a set +0x3f4 skips straight to teardown,
    /// and a set +0x4 with clear +0x3f4 skips the wait loop too.
    #[test]
    fn c_game_comm_wait_for_peer_ready_exits() {
        check(
            ee::s06::f_002586b0__generated,
            c_game_comm_wait_for_peer_ready,
            300,
            &|c, seed| {
                if seed % 2 == 0 {
                    wait_for_peer_setup(c, seed, 1, 0);
                } else {
                    wait_for_peer_setup(c, seed, 0, 1);
                }
            },
        );
    }

    driven!(wait_ready_gen, ee::s06::f_002586b0__generated);

    driven!(wait_ready_hand, c_game_comm_wait_for_peer_ready);

    /// The wait loop itself: every seed runs three loop passes (Update +
    /// yield + flag checks + back-edge event check) and then the peer
    /// reports ready. Both sides must agree through the passes, any event
    /// unwinds and re-entries, and the ready exit.
    ///
    /// Termination rig: the fixture pins Update's cheap path (`+0x8 = 1`,
    /// `+0x138 = 16`) and the gated yield fake reports the peer ready on
    /// the 3rd tick. Both sides share the fake, so the comparison is exact.
    #[test]
    fn c_game_comm_wait_for_peer_ready_waits() {
        let resumes = check_events(wait_ready_gen, wait_ready_hand, 60, &|c, seed| {
            // No fake-call unwinds here: the wait loop is terminated by the
            // scheduled event unwinds, not by miss-hook unwinds (whose
            // 0xdead0000 pc the driver does not resume).
            let mut r = fakes::machine(c, seed, 0x0025_86b0, 0);
            wait_for_peer_setup_r(c, seed, &mut r, 0, 0);
            schedule(c, &mut r, seed, 0x0025_86b0)
        });
        assert!(
            resumes.get(&0x0025_8720).copied().unwrap_or(0) > 0,
            "wait loop head never resumed (map: {resumes:x?})",
        );
    }

    /// A scrambled machine with this module's fakes armed and no unwinds.
    pub(crate) fn machine_box(salt: u64) -> Box<Cpu> {
        let mut c = crate::difftest::fresh();
        machine(&mut c, 0, salt, 0);
        lobbyrig_arm(&mut c);
        c
    }

    // ------------------------------------------------------------------
    // `cGameComm_UpdateWaitPopup` (0x258cb0).

    pub(crate) const WAIT_POPUP_COMM: u32 = DATA + 0x1000;

    pub(crate) const WAIT_POPUP_APP: u32 = DATA + 0x2000;

    pub(crate) const WAIT_POPUP_MGR: u32 = DATA + 0x3000;

    pub(crate) const WAIT_POPUP_UIMGR: u32 = DATA + 0x4000;

    pub(crate) const WAIT_POPUP_STATE: u32 = DATA + 0x5000;

    pub(crate) const WAIT_POPUP_POPUP: u32 = DATA + 0x6000;

    pub(crate) const WAIT_POPUP_STROBJ: u32 = DATA + 0x7000;

    pub(crate) const WAIT_POPUP_STRS1: u32 = DATA + 0x7100;

    pub(crate) const WAIT_POPUP_Y: u32 = DATA + 0x8000;

    pub(crate) const WAIT_POPUP_W: u32 = DATA + 0x8100;

    pub(crate) fn wait_popup_setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 0x258c_b0, 4);
        lobbyrig_arm(c);
        c.g[A0][0] = sx(WAIT_POPUP_COMM);
        c.bus.write32(WAIT_POPUP_COMM + WAIT_POPUP_RESULT, 7);
        c.bus.write32(
            WAIT_POPUP_COMM + WAIT_POPUP_SHOW_WAIT,
            (seed % 3 == 0) as u32,
        );
        c.bus.write32(
            WAIT_POPUP_COMM + WAIT_POPUP_MODE,
            [0, 1, 2, 3, 7][seed as usize / 8 % 5],
        );
        c.bus
            .write32(WAIT_POPUP_COMM + WAIT_POPUP_LOCAL_PAUSED, seed / 40 % 2);
        c.bus
            .write32(WAIT_POPUP_COMM + WAIT_POPUP_PEER_PAUSED, seed / 80 % 2);
        c.bus.write32(
            WAIT_POPUP_COMM + WAIT_POPUP_RESUME_GRACE,
            [1, 0, 0xffff_ffffu32][seed as usize / 7 % 3],
        );
        c.bus.write32(GAME_GP - 0x848, WAIT_POPUP_APP);
        c.bus.write32(GAME_GP - 0x84c, WAIT_POPUP_Y);
        c.bus.write32(WAIT_POPUP_APP + 0x84, WAIT_POPUP_MGR);
        c.bus.write32(WAIT_POPUP_APP + 0x8c, WAIT_POPUP_STROBJ);
        c.bus.write32(
            WAIT_POPUP_MGR + 0x48,
            if seed % 8 == 0 { 0 } else { WAIT_POPUP_UIMGR },
        );
        c.bus
            .write32(WAIT_POPUP_MGR + 0x214, [1, 2, 3][seed as usize / 3 % 3]);
        c.bus.write32(WAIT_POPUP_MGR + 0xc, WAIT_POPUP_W);
        c.bus
            .write32(WAIT_POPUP_W, [0, 4, 5, 6][seed as usize / 11 % 4]);
        c.bus
            .write32(WAIT_POPUP_Y + 0x550, [0, 1, 2, 3][seed as usize / 5 % 4]);
        c.bus.write32(WAIT_POPUP_STROBJ + 0x4, WAIT_POPUP_STRS1);
        c.bus.write16(WAIT_POPUP_STRS1 + 0x20, 0);
        c.bus.write32(WAIT_POPUP_STRS1 + 0x24, fakes::FAKE + 0x10);
        c.bus.write32(
            lobbyrig_mail(0x0039_f9d8),
            if seed % 8 == 1 { 0 } else { WAIT_POPUP_STATE },
        );
        c.bus.write32(WAIT_POPUP_STATE + 0x8, WAIT_POPUP_POPUP);
        c.bus.write16(WAIT_POPUP_POPUP + 0xc0, 0);
        c.bus.write32(WAIT_POPUP_POPUP + 0xc4, fakes::FAKE + 0x20);
    }

    /// The wait popup refreshes, shows and hides the net lobby notice:
    /// early exits leave the mode alone, the show tails set it to 1 or 2
    /// and the hide tail clears it.
    #[test]
    fn c_game_comm_update_wait_popup_matches() {
        fakes::covered(
            ee::s06::f_00258cb0__generated,
            c_game_comm_update_wait_popup,
            300,
            &wait_popup_setup,
        );
    }

    /// Each tail writes the mode the popup ends in (hide 0, show-waiting 1,
    /// show-countdown 2, untouched 9).
    #[test]
    fn c_game_comm_update_wait_popup_arms() {
        use crate::difftest::run_generated;
        // (show, ui_null, found_null, mode, local, peer, w, want_mode)
        let rows: [(u32, u32, u32, u32, u32, u32, u32, u32); 4] = [
            // Mode 7 hides through the tail: 0.
            (1, 0, 0, 7, 0, 0, 5, 0),
            // Hidden, then shown counting down: 2.
            (1, 0, 0, 0, 0, 0, 5, 2),
            // Peer pause picks the other message, still shows waiting: 1.
            (1, 0, 0, 0, 0, 1, 5, 1),
            // No UI manager: the mode is untouched.
            (1, 1, 0, 9, 0, 0, 5, 9),
        ];
        for (show, ui_null, found_null, mode, local, peer, w, want) in rows {
            for hand in [false, true] {
                let mut c = machine_box(0x258c_b0);
                c.g[A0][0] = sx(WAIT_POPUP_COMM);
                c.bus.write32(WAIT_POPUP_COMM + WAIT_POPUP_RESULT, 7);
                c.bus.write32(WAIT_POPUP_COMM + WAIT_POPUP_SHOW_WAIT, show);
                c.bus.write32(WAIT_POPUP_COMM + WAIT_POPUP_MODE, mode);
                c.bus
                    .write32(WAIT_POPUP_COMM + WAIT_POPUP_LOCAL_PAUSED, local);
                c.bus
                    .write32(WAIT_POPUP_COMM + WAIT_POPUP_PEER_PAUSED, peer);
                c.bus.write32(WAIT_POPUP_COMM + WAIT_POPUP_RESUME_GRACE, 1);
                c.bus.write32(GAME_GP - 0x848, WAIT_POPUP_APP);
                c.bus.write32(GAME_GP - 0x84c, WAIT_POPUP_Y);
                c.bus.write32(WAIT_POPUP_APP + 0x84, WAIT_POPUP_MGR);
                c.bus.write32(WAIT_POPUP_APP + 0x8c, WAIT_POPUP_STROBJ);
                c.bus.write32(
                    WAIT_POPUP_MGR + 0x48,
                    if ui_null != 0 { 0 } else { WAIT_POPUP_UIMGR },
                );
                c.bus.write32(WAIT_POPUP_MGR + 0x214, 3);
                c.bus.write32(WAIT_POPUP_MGR + 0xc, WAIT_POPUP_W);
                c.bus.write32(WAIT_POPUP_W, w);
                c.bus.write32(WAIT_POPUP_Y + 0x550, 0);
                c.bus.write32(WAIT_POPUP_STROBJ + 0x4, WAIT_POPUP_STRS1);
                c.bus.write16(WAIT_POPUP_STRS1 + 0x20, 0);
                c.bus.write32(WAIT_POPUP_STRS1 + 0x24, fakes::FAKE + 0x10);
                c.bus.write32(
                    lobbyrig_mail(0x0039_f9d8),
                    if found_null != 0 { 0 } else { WAIT_POPUP_STATE },
                );
                c.bus.write32(WAIT_POPUP_STATE + 0x8, WAIT_POPUP_POPUP);
                c.bus.write16(WAIT_POPUP_POPUP + 0xc0, 0);
                c.bus.write32(WAIT_POPUP_POPUP + 0xc4, fakes::FAKE + 0x20);
                if hand {
                    run_generated(&mut c, c_game_comm_update_wait_popup);
                } else {
                    run_generated(&mut c, ee::s06::f_00258cb0__generated);
                }
                assert_eq!(c.bus.read32(WAIT_POPUP_COMM + WAIT_POPUP_MODE), want);
            }
        }
    }

    /// The generation gate and the two pause exits: a stale generation
    /// with every gate passing arms the grace; anything else leaves it
    /// clear. Covers the state-not-4 buddy arm and the `+0x18 > 0` exit,
    /// which the seeded fixture's coupled digits never reach together.
    #[test]
    fn c_game_comm_update_idle_kick_timer_arms() {
        use crate::difftest::run_generated;
        // (gen, closed, wait, gone, session, buddy, plus18, plus1c, local,
        // peer, want_grace)
        let rows: [(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32); 7] = [
            // Stale gen, all gates pass, state not 4, +0x18 clear: arm.
            (0, 0, 0, 9, 3, 5, 0, 0, 0, 0, 0xb4),
            // Stale gen, states agree but +0x18 set: idle exit, no arm.
            (0, 0, 0, 9, 3, 5, 7, 0, 0, 0, 0),
            // Stale gen, local pause: the pause exit skips the arm.
            (0, 0, 0, 9, 3, 5, 0, 0, 1, 0, 0),
            // Stale gen, peer pause: skips the arm.
            (0, 0, 0, 9, 3, 5, 0, 0, 0, 1, 0),
            // Fresh gen: never reaches the buddy block.
            (2, 0, 0, 9, 3, 5, 0, 0, 0, 0, 0),
            // State 4 with a cool word: the `151` exit skips the arm.
            (0, 0, 0, 9, 3, 4, 0, 200, 0, 0, 0),
            // State 4, warm word, +0x18 clear, nobody paused: arm.
            (0, 0, 0, 9, 3, 4, 0, 100, 0, 0, 0xb4),
        ];
        for (gen, closed, wait, gone, session, buddy, plus18, plus1c, local, peer, want) in rows {
            for hand in [false, true] {
                let mut c = machine_box(0x2588_a8);
                c.g[A0][0] = sx(GCOMM_FIX_COMM);
                c.pc = 0x0025_88a8;
                c.bus.write32(GAME_GP - 0x848, GCOMM_FIX_WORLD);
                c.bus
                    .write32(GCOMM_FIX_WORLD + 0x84, GCOMM_FIX_COMM + 0x300);
                c.bus.write32(GCOMM_FIX_COMM + 0x300 + 0x214, gen);
                c.bus
                    .write32(GCOMM_FIX_COMM + 0x300 + 0xc, GCOMM_FIX_COMM + 0x380);
                c.bus.write32(GCOMM_FIX_COMM + 0x4, closed);
                c.bus.write32(GCOMM_FIX_COMM + 0x48, wait);
                c.bus.write32(GCOMM_FIX_COMM, gone);
                c.bus.write32(GCOMM_FIX_COMM + 0x70, session);
                c.bus.write32(GCOMM_FIX_COMM + 0x380, buddy);
                c.bus.write32(GCOMM_FIX_COMM + 0x380 + 0x18, plus18);
                c.bus.write32(GCOMM_FIX_COMM + 0x380 + 0x1c, plus1c);
                c.bus.write32(GCOMM_FIX_COMM + 0x74, local);
                c.bus.write32(GCOMM_FIX_COMM + 0x78, peer);
                if hand {
                    run_generated(&mut c, c_game_comm_update_idle_kick_timer);
                } else {
                    run_generated(&mut c, ee::s06::f_002588a8__generated);
                }
                assert_eq!(c.bus.read32(GCOMM_FIX_COMM + 0x3fc), want);
            }
        }
    }

    /// The negative mode arm and the mode-0 / countdown-2 exit: the first
    /// hides (mode 0), the second leaves the mode untouched (where the
    /// show tail would set 2).
    #[test]
    fn c_game_comm_update_wait_popup_arms2() {
        use crate::difftest::run_generated;
        // (mode, local, peer, w, y, want_mode)
        let rows: [(u32, u32, u32, u32, u32, u32); 2] = [
            // Negative mode takes the hide tail: 0.
            (0xffff_ffff, 0, 0, 5, 0, 0),
            // Mode 0, local clear, peer set, W past the window, countdown
            // at 2: the early exit leaves the mode alone.
            (0, 0, 1, 6, 2, 0),
        ];
        for (mode, local, peer, w, y, want) in rows {
            for hand in [false, true] {
                let mut c = machine_box(0x258c_b0);
                c.g[A0][0] = sx(WAIT_POPUP_COMM);
                c.bus.write32(WAIT_POPUP_COMM + WAIT_POPUP_RESULT, 7);
                c.bus.write32(WAIT_POPUP_COMM + WAIT_POPUP_SHOW_WAIT, 1);
                c.bus.write32(WAIT_POPUP_COMM + WAIT_POPUP_MODE, mode);
                c.bus
                    .write32(WAIT_POPUP_COMM + WAIT_POPUP_LOCAL_PAUSED, local);
                c.bus
                    .write32(WAIT_POPUP_COMM + WAIT_POPUP_PEER_PAUSED, peer);
                c.bus.write32(WAIT_POPUP_COMM + WAIT_POPUP_RESUME_GRACE, 1);
                c.bus.write32(GAME_GP - 0x848, WAIT_POPUP_APP);
                c.bus.write32(GAME_GP - 0x84c, WAIT_POPUP_Y);
                c.bus.write32(WAIT_POPUP_APP + 0x84, WAIT_POPUP_MGR);
                c.bus.write32(WAIT_POPUP_APP + 0x8c, WAIT_POPUP_STROBJ);
                c.bus.write32(WAIT_POPUP_MGR + 0x48, WAIT_POPUP_UIMGR);
                c.bus.write32(WAIT_POPUP_MGR + 0x214, 3);
                c.bus.write32(WAIT_POPUP_MGR + 0xc, WAIT_POPUP_W);
                c.bus.write32(WAIT_POPUP_W, w);
                c.bus.write32(WAIT_POPUP_Y + 0x550, y);
                c.bus.write32(WAIT_POPUP_STROBJ + 0x4, WAIT_POPUP_STRS1);
                c.bus.write16(WAIT_POPUP_STRS1 + 0x20, 0);
                c.bus.write32(WAIT_POPUP_STRS1 + 0x24, fakes::FAKE + 0x10);
                c.bus.write32(lobbyrig_mail(0x0039_f9d8), WAIT_POPUP_STATE);
                c.bus.write32(WAIT_POPUP_STATE + 0x8, WAIT_POPUP_POPUP);
                c.bus.write16(WAIT_POPUP_POPUP + 0xc0, 0);
                c.bus.write32(WAIT_POPUP_POPUP + 0xc4, fakes::FAKE + 0x20);
                if hand {
                    run_generated(&mut c, c_game_comm_update_wait_popup);
                } else {
                    run_generated(&mut c, ee::s06::f_00258cb0__generated);
                }
                assert_eq!(c.bus.read32(WAIT_POPUP_COMM + WAIT_POPUP_MODE), want);
            }
        }
    }

    // ------------------------------------------------------------------
    // `cGameComm_SendSyncCheck` (0x258f20).

    pub(crate) const SYNC_CHECK_COMM: u32 = DATA + 0x9000;

    pub(crate) const SYNC_CHECK_APP: u32 = DATA + 0xa000;

    pub(crate) const SYNC_CHECK_MGR: u32 = DATA + 0xb000;

    pub(crate) const SYNC_CHECK_X: u32 = DATA + 0xc000;

    pub(crate) fn sync_check_setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 0x258f_20, 0);
        lobbyrig_arm(c);
        let frames: [u32; 10] = [0, 1, 59, 60, 61, 119, 120, 121, 1000, 0xffff_ffff];
        let frame = frames[seed as usize % 10];
        let value = 0x1000 + seed;
        c.g[A0][0] = sx(SYNC_CHECK_COMM);
        c.g[A1][0] = sx(frame);
        c.g[A2][0] = value as u64;
        c.bus.write32(GAME_GP - 0x848, SYNC_CHECK_APP);
        c.bus.write32(SYNC_CHECK_APP + 0x84, SYNC_CHECK_MGR);
        // Session-state gate: 5 runs at once, 3 needs the race flag,
        // anything else leaves early.
        let state = [5, 3, 3, 2, 7][seed as usize / 10 % 5];
        c.bus.write32(SYNC_CHECK_MGR + 0xc, SYNC_CHECK_X);
        c.bus.write32(SYNC_CHECK_X, state);
        c.bus.write32(
            SYNC_CHECK_MGR + 0x214,
            if seed / 50 % 2 == 0 { 3 } else { 5 },
        );
        c.bus.write32(
            SYNC_CHECK_COMM + SYNC_CHECK_READY,
            if seed / 3 % 5 == 0 { 0 } else { 1 },
        );
        c.bus.write32(
            SYNC_CHECK_COMM + SYNC_CHECK_CLOSED,
            if seed / 7 % 4 == 0 { 1 } else { 0 },
        );
        c.bus.write32(
            SYNC_CHECK_COMM + 0x0,
            if seed / 11 % 3 == 0 { 0 } else { 9 },
        );
        c.bus.write32(SYNC_CHECK_COMM + SYNC_CHECK_DESYNCED, 0);
        c.bus.write32(
            SYNC_CHECK_COMM + SYNC_CHECK_SENT,
            if seed / 13 % 3 == 0 { 0 } else { 1 },
        );
        c.bus.write32(
            SYNC_CHECK_COMM + SYNC_CHECK_REPLIED,
            if seed / 17 % 2 == 0 { 0 } else { 1 },
        );
        c.bus.write32(SYNC_CHECK_COMM + SYNC_CHECK_FRAME_NO, 0);
        let min = [-1i32, 0, 5, 100][seed as usize / 19 % 4];
        c.bus
            .write32(SYNC_CHECK_COMM + SYNC_CHECK_MIN_FRAME, min as u32);
        c.bus.write32(SYNC_CHECK_COMM + 0x364, 0);
        c.bus.write32(SYNC_CHECK_COMM + 0x368, 0);
        c.bus.write32(SYNC_CHECK_COMM + 0x6c, 0);
        // Nonzero ring fills so a wrong slot index shows as data. On
        // mismatch seeds the compared slot and the stored slot both
        // disagree, whichever the tail reaches.
        let frame_s = frame as i32;
        let m = if frame_s < 0 || min < 0 {
            0
        } else {
            (frame_s.min(min) / 60 % 60) as u32
        };
        let slot = ((frame_s as u32 / 60) % 60) % 60;
        for i in 0..60u32 {
            c.bus
                .write32(SYNC_CHECK_COMM + SYNC_CHECK_RING + 4 * i, 0xA100_0000 + i);
            c.bus.write32(
                SYNC_CHECK_COMM + SYNC_CHECK_PEER_RING + 4 * i,
                0xA100_0000 + i,
            );
        }
        if seed / 23 % 3 != 0 {
            c.bus.write32(
                SYNC_CHECK_COMM + SYNC_CHECK_PEER_RING + 4 * m,
                0xA100_0000 + m ^ 0xff,
            );
            c.bus.write32(
                SYNC_CHECK_COMM + SYNC_CHECK_PEER_RING + 4 * slot,
                0xA100_0000 + slot ^ 0xff,
            );
        }
    }

    /// The sync check gates on the lobby state and the session flags,
    /// records the frame in the ring (sending it first), then flags a
    /// mismatch between the two rings.
    #[test]
    fn c_game_comm_send_sync_check_matches() {
        check(
            ee::s06::f_00258f20__generated,
            c_game_comm_send_sync_check,
            300,
            &sync_check_setup,
        );
    }

    /// The send path stores the frame and value and raises SENT; a
    /// matching peer ring leaves DESYNCED clear, a mismatch sets it.
    #[test]
    fn c_game_comm_send_sync_check_arms() {
        use crate::difftest::run_generated;
        // (sent_in, replied_in, min, frame, value, want_sent, want_desync)
        // Note: the ring store runs unconditionally (even when SENT is
        // already set), so rows 1-2 still overwrite ring[slot] and the
        // compare that follows always sees the 0x42 mismatch: DESYNCED ends
        // up set on both the generated and the hand side.
        let rows: [(u32, u32, i32, u32, u32, u32, u32); 4] = [
            (0, 0, 200, 63, 0x42, 1, 1),
            (1, 0, 200, 63, 0x42, 1, 1),
            (1, 1, 200, 63, 0x42, 0, 1),
            (0, 0, -1, 63, 0x42, 1, 0),
        ];
        for (sent_in, replied_in, min, frame, value, want_sent, want_desync) in rows {
            for hand in [false, true] {
                let mut c = machine_box(0x258f_20);
                c.g[A0][0] = sx(SYNC_CHECK_COMM);
                c.g[A1][0] = sx(frame);
                c.g[A2][0] = value as u64;
                c.bus.write32(GAME_GP - 0x848, SYNC_CHECK_APP);
                c.bus.write32(SYNC_CHECK_APP + 0x84, SYNC_CHECK_MGR);
                c.bus.write32(SYNC_CHECK_MGR + 0xc, SYNC_CHECK_X);
                c.bus.write32(SYNC_CHECK_X, 5);
                c.bus.write32(SYNC_CHECK_COMM + SYNC_CHECK_READY, 1);
                c.bus.write32(SYNC_CHECK_COMM + SYNC_CHECK_CLOSED, 0);
                c.bus.write32(SYNC_CHECK_COMM + 0x0, 9);
                c.bus.write32(SYNC_CHECK_COMM + SYNC_CHECK_DESYNCED, 0);
                c.bus.write32(SYNC_CHECK_COMM + SYNC_CHECK_SENT, sent_in);
                c.bus
                    .write32(SYNC_CHECK_COMM + SYNC_CHECK_REPLIED, replied_in);
                c.bus
                    .write32(SYNC_CHECK_COMM + SYNC_CHECK_MIN_FRAME, min as u32);
                c.bus.write32(SYNC_CHECK_COMM + 0x364, 0);
                c.bus.write32(SYNC_CHECK_COMM + 0x368, 0);
                c.bus.write32(SYNC_CHECK_COMM + 0x6c, 0);
                let slot = (frame / 60 % 60) % 60;
                let m = if min < 0 {
                    0
                } else {
                    ((frame.min(min as u32)) / 60 % 60) % 60
                };
                for i in 0..60u32 {
                    c.bus
                        .write32(SYNC_CHECK_COMM + SYNC_CHECK_RING + 4 * i, 0xA100_0000 + i);
                    c.bus.write32(
                        SYNC_CHECK_COMM + SYNC_CHECK_PEER_RING + 4 * i,
                        0xA100_0000 + i,
                    );
                }
                // Mismatch the peer slot the tail compares, mirroring the
                // seeded fixture.
                c.bus.write32(
                    SYNC_CHECK_COMM + SYNC_CHECK_PEER_RING + 4 * m,
                    0xA100_0000 + m ^ 0xff,
                );
                if hand {
                    run_generated(&mut c, c_game_comm_send_sync_check);
                } else {
                    run_generated(&mut c, ee::s06::f_00258f20__generated);
                }
                assert_eq!(c.bus.read32(SYNC_CHECK_COMM + SYNC_CHECK_SENT), want_sent);
                assert_eq!(
                    c.bus.read32(SYNC_CHECK_COMM + SYNC_CHECK_DESYNCED),
                    want_desync
                );
                if sent_in == 0 {
                    assert_eq!(
                        c.bus.read32(SYNC_CHECK_COMM + SYNC_CHECK_RING + 4 * slot),
                        value
                    );
                    assert_eq!(c.bus.read32(SYNC_CHECK_COMM + SYNC_CHECK_FRAME_NO), frame);
                }
            }
        }
    }

    /// The desynced exit: with DESYNCED already set the check returns at
    /// once, sending nothing and storing nothing.
    #[test]
    fn c_game_comm_send_sync_check_desynced_exit() {
        use crate::difftest::run_generated;
        for hand in [false, true] {
            let mut c = machine_box(0x258f_20);
            c.g[A0][0] = sx(SYNC_CHECK_COMM);
            c.g[A1][0] = sx(63);
            c.g[A2][0] = 0x42;
            c.bus.write32(GAME_GP - 0x848, SYNC_CHECK_APP);
            c.bus.write32(SYNC_CHECK_APP + 0x84, SYNC_CHECK_MGR);
            c.bus.write32(SYNC_CHECK_MGR + 0xc, SYNC_CHECK_X);
            c.bus.write32(SYNC_CHECK_X, 5);
            c.bus.write32(SYNC_CHECK_COMM + SYNC_CHECK_READY, 1);
            c.bus.write32(SYNC_CHECK_COMM + SYNC_CHECK_CLOSED, 0);
            c.bus.write32(SYNC_CHECK_COMM + 0x0, 9);
            c.bus.write32(SYNC_CHECK_COMM + SYNC_CHECK_DESYNCED, 1);
            c.bus.write32(SYNC_CHECK_COMM + SYNC_CHECK_SENT, 0);
            c.bus.write32(SYNC_CHECK_COMM + SYNC_CHECK_REPLIED, 0);
            c.bus.write32(SYNC_CHECK_COMM + SYNC_CHECK_MIN_FRAME, 200);
            c.bus.write32(SYNC_CHECK_COMM + 0x364, 0);
            c.bus.write32(SYNC_CHECK_COMM + 0x368, 0);
            c.bus.write32(SYNC_CHECK_COMM + 0x6c, 0);
            if hand {
                run_generated(&mut c, c_game_comm_send_sync_check);
            } else {
                run_generated(&mut c, ee::s06::f_00258f20__generated);
            }
            assert_eq!(c.bus.read32(SYNC_CHECK_COMM + SYNC_CHECK_SENT), 0);
            assert_eq!(c.bus.read32(SYNC_CHECK_COMM + SYNC_CHECK_DESYNCED), 1);
        }
    }
}
