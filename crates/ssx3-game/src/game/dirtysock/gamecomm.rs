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
    pub(crate) use crate::game::dirtysock::tagsunk::callee::C_GAME_COMM_SEND_PACKET as C_GAME_COMM_SEND_PACKET_2;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::STR_HASH;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::USTR_COPY;

    /// `cGameComm_SendPacket` (0x00257e98): queue a packet for the peer.
    pub(crate) const C_GAME_COMM_SEND_PACKET: Callee = Callee {
        addr: 0x0025_7e98,
        f: ee::s06::cGameComm_SendPacket,
    };

    /// `cGameComm_SetTimeoutTimer` (0x00258870; main:
    /// `gamecomm::c_game_comm_set_timeout_timer`): rearm the session deadline.
    pub(crate) const C_GAME_COMM_SET_TIMEOUT_TIMER: Callee = Callee {
        addr: 0x0025_8870,
        f: ee::s06::cGameComm_SetTimeoutTimer,
    };

    /// `cGameComm_Update` (0x002570d8; unported): advance the comm state
    /// machine. Routes through the test-gated shim (real body in real
    /// gameplay, logging no-op on armed test machines).
    pub(crate) const C_GAME_COMM_UPDATE: Callee = Callee {
        addr: 0x0025_70d8,
        f: ee::s06::cGameComm_Update,
    };

    /// `cGameComm_UpdateIdleKickTimer` (0x002588a8, ported in this module).
    pub(crate) const C_GAME_COMM_UPDATE_IDLE_KICK_TIMER: Callee = Callee {
        addr: 0x0025_88a8,
        f: ee::s06::cGameComm_UpdateIdleKickTimer,
    };

    /// `cGameComm_SetState` (0x00257038): request a comm state change.
    pub(crate) const C_GAME_COMM_SET_STATE: Callee = Callee {
        addr: 0x0025_7038,
        f: ee::s06::cGameComm_SetState,
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

    /// `BELibrary_GetInterface` (0x0014dd58).
    pub(crate) const BE_LIBRARY_GET_INTERFACE: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::BELibrary_GetInterface,
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
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes;
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
