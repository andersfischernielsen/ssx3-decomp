//! Staging module oc1350 (wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1350`.

#![allow(unused_imports)]

use crate::abi::{arg, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001a_8828,
        net_fe_state_transition_in,
        "NetFEState_TransitionIn",
    ),
    (
        0x001d_e860,
        net_fe_state_handle_input,
        "NetFEState_HandleInput",
    ),
    (0x001d_e868, net_fe_state_v25, "NetFEState_v25"),
    (0x001d_e870, net_fe_state_v29, "NetFEState_v29"),
    (0x001d_e8a8, net_fe_state_v30, "NetFEState_v30"),
    (
        0x001e_0c10,
        net_config_auth_popup_transition_in,
        "NetConfigAuthPopup_TransitionIn",
    ),
    (
        0x001e_0c18,
        net_config_auth_popup_transition_out,
        "NetConfigAuthPopup_TransitionOut",
    ),
    (
        0x001e_0c20,
        net_config_auth_popup_v32,
        "NetConfigAuthPopup_v32",
    ),
    (
        0x001e_0c28,
        net_config_auth_popup_v29,
        "NetConfigAuthPopup_v29",
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
];

/// Status word the `NetFEState` table reads at +0x58.
pub(crate) const NET_FE_STATE_HANDLE_INPUT_STATUS: u32 = 0x58;
/// First word `NetFEState_v30` tests at +0x40.
pub(crate) const NET_FE_STATE_V30_GATE: u32 = 0x40;
/// Second word `NetFEState_v30` tests at +0x48.
pub(crate) const NET_FE_STATE_V30_FLAG: u32 = 0x48;

/// `cGameComm` fields the pause/quit/reset paths read and write. Names follow
/// the `game_comm` unit in `game/dirtysock/gamecomm.rs` where it names them.
mod send_pause_state {
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

/// Fields `cGameComm_UpdateIdleKickTimer` reads.
mod update_idle_kick_timer {
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

/// Fields `cGameComm_OnRaceStartReset` clears and sets.
mod on_race_start_reset {
    /// Peer-lag accumulator cleared on reset.
    pub const LAG: u32 = 0x48;
    /// Wait-for-peer notice cleared on reset.
    pub const WAIT_NOTICE: u32 = 0x78;
    /// Race-active flag cleared on reset.
    pub const RACE_ACTIVE: u32 = 0x84;
}

/// Fields `cGameComm_OnCountdownStart` reads and writes.
mod on_countdown_start {
    /// Session liveness: below 15 means the online session is up.
    pub const SESSION: u32 = 0x70;
    /// Set once the countdown has started.
    pub const COUNTDOWN_STARTED: u32 = 0x7c;
    /// Rearmed on countdown start.
    pub const DEADLINE: u32 = 0x94;
}

/// Fields `cGameComm_OnRaceStart` reads and writes.
mod on_race_start {
    /// Session liveness: below 15 means the online session is up.
    pub const SESSION: u32 = 0x70;
    /// `GameModule` race-state value written on race start.
    pub const RACE_STATE: u32 = 0x80;
    /// Session deadline cleared on race start.
    pub const DEADLINE: u32 = 0x94;
    /// Frame counter latched from the world clock.
    pub const FRAME: u32 = 0x34;
}

/// Fields `cGameComm_OnRaceEnd` reads and writes.
mod on_race_end {
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

/// `cGameComm_SendResync` fields.
mod send_resync {
    /// Non-zero once the session has closed.
    pub const CLOSED: u32 = 0x4;
    /// Resync sequence number, bumped per resync packet.
    pub const SEQUENCE: u32 = 0x54;
}

/// Direct-call targets of this module's call-making ports. Each goes through
/// the generated wrapper, so at install the override slots resolve to the
/// hand-written units (the buddy/lobby ports in
/// `game/dirtysock/tagsunk.rs`, `cGameComm_SetTimeoutTimer` in
/// `game/dirtysock/gamecomm.rs`) and the differential tests can gate fakes
/// on those same slots.
/// Gate of the callee fakes: they answer only while this word holds
/// `OC1350_MAGIC` (otherwise the body they replaced runs). Lives at module
/// scope (not in tests) so the wait-loop port's generated callees can read
/// it in all builds; test machines arm it, real gameplay never does. The
/// base mirrors `difftest::DATA` literally to avoid a test-only dependency.
/// (`cGameComm_Update` 0x002570d8 has no override slot, so the wait loop
/// calls the real generated body; only the hooked yield callee is gated.)
#[cfg(test)]
const OC1350_RAM_BASE: u32 = 0x0080_0000;
#[cfg(test)]
pub(crate) const OC1350_CTRL: u32 = OC1350_RAM_BASE + 0x000e_3000;
#[cfg(test)]
pub(crate) const OC1350_MAGIC: u32 = 0x0c13_5000;
/// Gated-yield tick count (see the waits test): the fake reports the peer
/// ready (`OC1350_COMM + 0x4`) exactly once, on the 3rd gated tick. The
/// comm object lives at this test's fixed `OC1350_COMM` address.
#[cfg(test)]
pub(crate) const OC1350_YIELD_HITS: u32 = OC1350_RAM_BASE + 0x000e_3004;

pub(crate) mod callee {
    use super::{ee, Callee};

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
}

/// `NetFEState_TransitionIn` (0x001a8828): enter the network front-end state.
/// Always reports 0.
pub fn net_fe_state_transition_in(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

/// `NetFEState_HandleInput` (0x001de860): poll the network front-end state.
/// Returns its status word.
pub fn net_fe_state_handle_input(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = Addr(arg(c, 0)).off(NET_FE_STATE_HANDLE_INPUT_STATUS).lw(c);
    ret(c);
}

/// `NetFEState_v25` (0x001de868): an unhandled `NetFEState` virtual slot.
/// Does nothing.
pub fn net_fe_state_v25(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `NetFEState_v29` (0x001de870): a `NetFEState` virtual slot that always
/// reports handled (1).
pub fn net_fe_state_v29(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 1;
    ret(c);
}

/// `NetFEState_v30` (0x001de8a8): a `NetFEState` readiness check. Reports
/// whether the gate word is set and the flag word is non-zero.
pub fn net_fe_state_v30(c: &mut Cpu) {
    c.cycles += 3;
    let state = Addr(arg(c, 0));
    let mut ready = 0;
    if state.off(NET_FE_STATE_V30_GATE).lw(c) != 0 {
        c.cycles += 2;
        ready = (state.off(NET_FE_STATE_V30_FLAG).lw(c) != 0) as u64;
    }
    c.cycles += 2;
    c.g[V1][0] = ready;
    c.g[V0][0] = ready;
    ret(c);
}

/// `NetConfigAuthPopup_TransitionIn` (0x001e0c10): enter the network-config
/// authorisation popup. Always reports handled (1).
pub fn net_config_auth_popup_transition_in(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 1;
    ret(c);
}

/// `NetConfigAuthPopup_TransitionOut` (0x001e0c18): leave the network-config
/// authorisation popup. Always reports handled (1).
pub fn net_config_auth_popup_transition_out(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 1;
    ret(c);
}

/// `NetConfigAuthPopup_v32` (0x001e0c20): an unhandled authorisation-popup
/// virtual slot. Does nothing.
pub fn net_config_auth_popup_v32(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `NetConfigAuthPopup_v29` (0x001e0c28): an authorisation-popup virtual slot
/// that always reports 0.
pub fn net_config_auth_popup_v29(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

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

fn c_game_comm_send_pause_state_body(c: &mut Cpu) -> Flow {
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
/// buddy list is fresh. Clears the grace first; leaves it clear unless every
/// gate passes, then sets it to `IDLE_GRACE`.
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

/// `void cGameComm_OnRaceStartReset(cGameComm *self)` (0x00258950): clear the
/// race state for a fresh start: lag, wait notice and race-active flag.
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
/// countdown started while the online session is up, rearming the deadline.
pub fn c_game_comm_on_countdown_start(c: &mut Cpu) {
    c.cycles += 4;
    let comm = Addr(arg(c, 0));
    c.g[V0][0] = comm.off(on_countdown_start::SESSION).lw(c);
    c.g[V0][0] = ((c.g[V0][0] as i64) < 15i64) as u64;
    let live = c.g[V0][0] == 0;
    c.g[V0][0] = 1;
    if !live {
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

fn c_game_comm_on_race_start_body(c: &mut Cpu) -> Flow {
    use on_race_start::*;
    c.cycles += 8;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    let comm = Addr(c.g[S0][0] as u32);
    c.g[V0][0] = comm.off(SESSION).lw(c);
    c.g[V0][0] = ((c.g[V0][0] as i64) < 15i64) as u64;
    let live = c.g[V0][0] == 0;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    if !live {
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

fn c_game_comm_on_race_end_body(c: &mut Cpu) -> Flow {
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
    let live = c.g[V0][0] == 0;
    comm.off(PAUSE_RELATED).sw(c, 0);
    if !live {
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

fn c_game_comm_quit_race_body(c: &mut Cpu) -> Flow {
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

fn c_game_comm_send_quit_body(c: &mut Cpu) -> Flow {
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

fn c_game_comm_send_resync_body(c: &mut Cpu) -> Flow {
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

fn c_game_comm_send_string_body(c: &mut Cpu) -> Flow {
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
            // Gone arm: straight to the epilogue, skipping the `+1` tail.
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
/// until every peer reports ready or the wait times out.
/// First-entry transcription of the generated body via
/// `tools/port_dispatcher.py` (every register, cycle and call kept);
/// any other `pc` (a resume after an unwound callee or event) runs the
/// generated body.
pub fn c_game_comm_wait_for_peer_ready(c: &mut Cpu) {
    if c.pc != 0x002586b0 {
        return ee::s06::f_002586b0__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x002586b0;
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
                    Callee {
                        addr: 0x003e_52b0,
                        f: ee::s14::THREAD_yieldticks,
                    }
                    .call(c, 0x00258730)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The `cGameComm` under test.
    pub(crate) const OC1350_COMM: u32 = DATA + 0x1000;
    /// The world at `gp - 0x848`.
    pub(crate) const OC1350_WORLD: u32 = DATA + 0x2000;
    /// The comm system at `gp - 0x238` (send-path channels).
    pub(crate) const OC1350_COMMSYS: u32 = DATA + 0x3000;
    /// The lobby manager at `gp - 0xc8`.
    pub(crate) const OC1350_LOBBY: u32 = DATA + 0x4000;
    /// A live buddy manager for `gp + 0x238`.
    pub(crate) const OC1350_BUDDY: u32 = DATA + 0x5000;
    /// The string `cGameComm_SendString` sends.
    pub(crate) const OC1350_STR: u32 = DATA + 0x6000;
    /// The popup vtable the quit path calls through.
    pub(crate) const OC1350_VT: u32 = DATA + 0x7000;
    /// Answer the `BuddyMan_IsConnected` fake reports.
    pub(crate) const OC1350_BUDDY_ANSWER: u32 = DATA + 0x000e_3100;
    /// Pointer the `BELibrary_GetInterface` fake reports.
    pub(crate) const OC1350_BE_IFACE: u32 = DATA + 0x000e_3200;
    /// Pointer the `BELibrary_GetInstance` fake reports.
    pub(crate) const OC1350_BE: u32 = DATA + 0x000e_3300;

    static OC1350_PREV_SET_TIMEOUT: AtomicUsize = AtomicUsize::new(0);
    static OC1350_PREV_CREATE: AtomicUsize = AtomicUsize::new(0);
    static OC1350_PREV_IS_CONNECTED: AtomicUsize = AtomicUsize::new(0);
    static OC1350_PREV_RECONNECT: AtomicUsize = AtomicUsize::new(0);
    static OC1350_PREV_DISCONNECT: AtomicUsize = AtomicUsize::new(0);
    static OC1350_PREV_PRESENCE: AtomicUsize = AtomicUsize::new(0);
    static OC1350_PREV_GET_INTERFACE: AtomicUsize = AtomicUsize::new(0);
    static OC1350_PREV_YIELD: AtomicUsize = AtomicUsize::new(0);
    static OC1350_PREV_GET_BE: AtomicUsize = AtomicUsize::new(0);
    static OC1350_PREV_COPY_PROFILE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn oc1350_gated(c: &mut Cpu) -> bool {
        c.bus.read32(OC1350_CTRL) == OC1350_MAGIC
    }

    fn oc1350_fake_set_timeout(c: &mut Cpu) {
        if !oc1350_gated(c) {
            return fakes::fall_back(c, &OC1350_PREV_SET_TIMEOUT, ee::s06::f_00258870__generated);
        }
        fakes::fake_call(c, 0x0025_8870);
    }

    fn oc1350_fake_create(c: &mut Cpu) {
        if !oc1350_gated(c) {
            return fakes::fall_back(c, &OC1350_PREV_CREATE, ee::s06::f_00260f80__generated);
        }
        fakes::fake_call(c, 0x0026_0f80);
    }

    fn oc1350_fake_is_connected(c: &mut Cpu) {
        if !oc1350_gated(c) {
            return fakes::fall_back(c, &OC1350_PREV_IS_CONNECTED, ee::s06::f_00261460__generated);
        }
        fakes::fake_call(c, 0x0026_1460);
        c.g[V0][0] = c.bus.read32(OC1350_BUDDY_ANSWER) as u64;
    }

    fn oc1350_fake_reconnect(c: &mut Cpu) {
        if !oc1350_gated(c) {
            return fakes::fall_back(c, &OC1350_PREV_RECONNECT, ee::s06::f_002613c8__generated);
        }
        fakes::fake_call(c, 0x0026_13c8);
    }

    fn oc1350_fake_disconnect(c: &mut Cpu) {
        if !oc1350_gated(c) {
            return fakes::fall_back(c, &OC1350_PREV_DISCONNECT, ee::s06::f_0025b800__generated);
        }
        fakes::fake_call(c, 0x0025_b800);
    }

    fn oc1350_fake_presence(c: &mut Cpu) {
        if !oc1350_gated(c) {
            return fakes::fall_back(c, &OC1350_PREV_PRESENCE, ee::s07::f_00262768__generated);
        }
        fakes::fake_call(c, 0x0026_2768);
    }

    fn oc1350_fake_get_interface(c: &mut Cpu) {
        if !oc1350_gated(c) {
            return fakes::fall_back(
                c,
                &OC1350_PREV_GET_INTERFACE,
                ee::s01::f_0014dd58__generated,
            );
        }
        fakes::fake_call(c, 0x0014_dd58);
        c.g[V0][0] = c.bus.read32(OC1350_BE_IFACE) as u64;
    }

    /// `THREAD_yieldticks` (0x003e52b0): the real body runs kernel
    /// thread calls that panic on test machines, so the wait-loop test
    /// stubs it out as a clean return. Gated runs also model the peer
    /// arrival the loop is waiting for: the 3rd tick reports the peer
    /// ready (see the waits test), exactly once.
    fn oc1350_fake_yield(c: &mut Cpu) {
        if !oc1350_gated(c) {
            return fakes::fall_back(c, &OC1350_PREV_YIELD, ee::s14::THREAD_yieldticks);
        }
        fakes::fake_call(c, 0x003e_52b0);
        let hits = c.bus.read32(OC1350_YIELD_HITS) + 1;
        c.bus.write32(OC1350_YIELD_HITS, hits);
        if hits == 3 {
            c.bus.write32(OC1350_COMM + 4, 1);
        }
    }

    fn oc1350_fake_get_be(c: &mut Cpu) {
        if !oc1350_gated(c) {
            return fakes::fall_back(c, &OC1350_PREV_GET_BE, ee::s01::f_0014dc80__generated);
        }
        fakes::fake_call(c, 0x0014_dc80);
        c.g[V0][0] = c.bus.read32(OC1350_BE) as u64;
    }

    fn oc1350_fake_copy_profile(c: &mut Cpu) {
        if !oc1350_gated(c) {
            return fakes::fall_back(c, &OC1350_PREV_COPY_PROFILE, ee::s01::f_0014e9c0__generated);
        }
        fakes::fake_call(c, 0x0014_e9c0);
    }

    pub(crate) fn oc1350_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00258870),
            oc1350_fake_set_timeout,
            &OC1350_PREV_SET_TIMEOUT,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00260F80),
            oc1350_fake_create,
            &OC1350_PREV_CREATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00261460),
            oc1350_fake_is_connected,
            &OC1350_PREV_IS_CONNECTED,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_002613C8),
            oc1350_fake_reconnect,
            &OC1350_PREV_RECONNECT,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_0025B800),
            oc1350_fake_disconnect,
            &OC1350_PREV_DISCONNECT,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00262768),
            oc1350_fake_presence,
            &OC1350_PREV_PRESENCE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc1350_fake_get_interface,
            &OC1350_PREV_GET_INTERFACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc1350_fake_get_be,
            &OC1350_PREV_GET_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014E9C0),
            oc1350_fake_copy_profile,
            &OC1350_PREV_COPY_PROFILE,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E52B0),
            oc1350_fake_yield,
            &OC1350_PREV_YIELD,
        );
    }

    /// Distinctive nonzero fill word.
    pub(crate) fn oc1350_word(r: &mut Rng) -> u32 {
        r.next_u32() | 0x0101_0101
    }

    /// The world the comm functions read at `gp - 0x848`.
    pub(crate) fn oc1350_world(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(GAME_GP - 0x848, OC1350_WORLD);
        c.bus.write32(OC1350_WORLD + 0x10, 60);
        c.bus.write32(OC1350_WORLD + 0x18, 1800);
        c.bus.write32(OC1350_WORLD + 0x78, 7);
        c.bus.write32(OC1350_WORLD + 0x84, 0);
        let _ = r;
    }

    /// The send path: an empty queue and two channels whose send method is
    /// unmapped, so `cCommSystem_ChannelSend` lands on the miss-hook fake and
    /// reports success, and `cGameComm_SendPacket` never reaches the packet
    /// allocator.
    pub(crate) fn oc1350_packet_rig(c: &mut Cpu) {
        c.bus.write32(OC1350_COMM + 0x364, 0);
        c.bus.write32(OC1350_COMM + 0x368, 0);
        c.bus.write32(GAME_GP - 0x238, OC1350_COMMSYS);
        for ch in 0..2u32 {
            c.bus
                .write32(OC1350_COMMSYS + 4 * ch, OC1350_COMMSYS + 0x100 + 0x10 * ch);
            c.bus.write32(
                OC1350_COMMSYS + 0x100 + 0x10 * ch + 8,
                OC1350_COMMSYS + 0x200 + 0x10 * ch,
            );
            c.bus.write16(OC1350_COMMSYS + 0x200 + 0x10 * ch + 0x40, 0);
            c.bus
                .write32(OC1350_COMMSYS + 0x200 + 0x10 * ch + 0x44, 0x0000_5000);
        }
    }

    #[test]
    fn net_fe_state_transition_in_matches() {
        check(
            ee::s03::f_001a8828__generated,
            net_fe_state_transition_in,
            300,
            &|c, seed| {
                let mut r = Rng(0x1350_0001 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x001a_8828;
            },
        );
    }

    #[test]
    fn net_fe_state_handle_input_matches() {
        check(
            ee::s04::f_001de860__generated,
            net_fe_state_handle_input,
            300,
            &|c, seed| {
                let mut r = Rng(0x1350_0002 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x001d_e860;
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA, oc1350_word(&mut r));
                c.bus.write32(
                    DATA + NET_FE_STATE_HANDLE_INPUT_STATUS,
                    if seed % 2 == 0 {
                        0
                    } else {
                        oc1350_word(&mut r)
                    },
                );
            },
        );
    }

    #[test]
    fn net_fe_state_v25_matches() {
        check(
            ee::s04::f_001de868__generated,
            net_fe_state_v25,
            300,
            &|c, seed| {
                let mut r = Rng(0x1350_0003 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x001d_e868;
            },
        );
    }

    #[test]
    fn net_fe_state_v29_matches() {
        check(
            ee::s04::f_001de870__generated,
            net_fe_state_v29,
            300,
            &|c, seed| {
                let mut r = Rng(0x1350_0004 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x001d_e870;
            },
        );
    }

    #[test]
    fn net_fe_state_v30_matches() {
        check(
            ee::s04::f_001de8a8__generated,
            net_fe_state_v30,
            300,
            &|c, seed| {
                let mut r = Rng(0x1350_0005 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x001d_e8a8;
                c.g[A0][0] = sx(DATA);
                // Gate shut, gate open with flag clear, gate open with flag set.
                c.bus.write32(
                    DATA + NET_FE_STATE_V30_GATE,
                    if seed % 3 == 0 {
                        0
                    } else {
                        oc1350_word(&mut r)
                    },
                );
                c.bus.write32(
                    DATA + NET_FE_STATE_V30_FLAG,
                    if seed % 3 == 2 {
                        oc1350_word(&mut r)
                    } else {
                        0
                    },
                );
            },
        );
    }

    #[test]
    fn net_config_auth_popup_transition_in_matches() {
        check(
            ee::s04::f_001e0c10__generated,
            net_config_auth_popup_transition_in,
            300,
            &|c, seed| {
                let mut r = Rng(0x1350_0006 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x001e_0c10;
            },
        );
    }

    #[test]
    fn net_config_auth_popup_transition_out_matches() {
        check(
            ee::s04::f_001e0c18__generated,
            net_config_auth_popup_transition_out,
            300,
            &|c, seed| {
                let mut r = Rng(0x1350_0007 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x001e_0c18;
            },
        );
    }

    #[test]
    fn net_config_auth_popup_v32_matches() {
        check(
            ee::s04::f_001e0c20__generated,
            net_config_auth_popup_v32,
            300,
            &|c, seed| {
                let mut r = Rng(0x1350_0008 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x001e_0c20;
            },
        );
    }

    #[test]
    fn net_config_auth_popup_v29_matches() {
        check(
            ee::s04::f_001e0c28__generated,
            net_config_auth_popup_v29,
            300,
            &|c, seed| {
                let mut r = Rng(0x1350_0009 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x001e_0c28;
            },
        );
    }

    #[test]
    fn c_game_comm_send_pause_state_matches() {
        oc1350_install_fakes();
        check(
            ee::s06::f_00258790__generated,
            c_game_comm_send_pause_state,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0025_8790, 2);
                c.pc = 0x0025_8790;
                c.bus.write32(OC1350_CTRL, OC1350_MAGIC);
                oc1350_world(c, &mut r);
                let comm = OC1350_COMM;
                for w in 0..0x140u32 {
                    c.bus.write32(comm + 4 * w, oc1350_word(&mut r));
                }
                // After the fill: the send queue must read empty (heads at
                // +0x364/+0x368), or real FlushSendQueue walks garbage.
                oc1350_packet_rig(c);
                // Closed, gone, then the live pause/resume arms.
                c.bus.write32(
                    comm + 0x4,
                    if seed % 5 == 0 {
                        oc1350_word(&mut r)
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
                        oc1350_word(&mut r)
                    },
                );
                c.bus.write32(comm + 0x6c, (seed >> 1) % 2);
                c.bus.write32(comm + 0x48, oc1350_word(&mut r));
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
                oc1350_world(c, &mut r);
                let comm = OC1350_COMM;
                for w in 0..0x140u32 {
                    c.bus.write32(comm + 4 * w, oc1350_word(&mut r));
                }
                let t6 = OC1350_COMM + 0x300;
                let t4 = OC1350_COMM + 0x380;
                c.bus.write32(OC1350_WORLD + 0x84, t6);
                c.bus.write32(t6 + 0x214, if seed % 6 == 0 { 2 } else { 0 });
                c.bus.write32(t6 + 0xc, t4);
                c.bus.write32(
                    comm + 0x4,
                    if seed % 6 == 1 {
                        oc1350_word(&mut r)
                    } else {
                        0
                    },
                );
                c.bus.write32(
                    comm + 0x48,
                    if seed % 6 == 2 {
                        oc1350_word(&mut r)
                    } else {
                        0
                    },
                );
                c.bus.write32(comm, if seed % 6 == 3 { 0 } else { comm });
                c.bus.write32(
                    comm + 0x70,
                    if seed % 4 == 0 {
                        15 + seed % 7
                    } else {
                        seed % 15
                    },
                );
                c.bus.write32(t4, if seed % 2 == 0 { 4 } else { 5 });
                c.bus
                    .write32(t4 + 0x1c, [0, 150, 200][(seed >> 1) as usize % 3]);
                c.bus.write32(
                    t4 + 0x18,
                    if (seed >> 2) % 2 == 0 {
                        0
                    } else {
                        oc1350_word(&mut r)
                    },
                );
                c.bus.write32(
                    comm + 0x74,
                    if (seed >> 3) % 2 == 0 {
                        0
                    } else {
                        oc1350_word(&mut r)
                    },
                );
                c.bus.write32(
                    comm + 0x78,
                    if (seed >> 4) % 2 == 0 {
                        0
                    } else {
                        oc1350_word(&mut r)
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
                let mut r = Rng(0x1350_000d ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0025_8950;
                c.g[A0][0] = sx(OC1350_COMM);
                for w in [0x48u32, 0x78, 0x84] {
                    c.bus.write32(OC1350_COMM + w, oc1350_word(&mut r));
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
                let mut r = Rng(0x1350_000e ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0025_8968;
                c.g[A0][0] = sx(OC1350_COMM);
                c.bus
                    .write32(OC1350_COMM + 0x70, if seed % 2 == 0 { 3 } else { 16 });
                c.bus.write32(OC1350_COMM + 0x94, oc1350_word(&mut r));
                c.bus.write32(OC1350_COMM + 0x7c, oc1350_word(&mut r));
            },
        );
    }

    #[test]
    fn c_game_comm_on_race_start_matches() {
        oc1350_install_fakes();
        check(
            ee::s06::f_00258988__generated,
            c_game_comm_on_race_start,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0025_8988, 2);
                c.pc = 0x0025_8988;
                c.bus.write32(OC1350_CTRL, OC1350_MAGIC);
                oc1350_world(c, &mut r);
                let comm = OC1350_COMM;
                for w in 0..0x140u32 {
                    c.bus.write32(comm + 4 * w, oc1350_word(&mut r));
                }
                c.bus
                    .write32(comm + 0x70, if seed % 3 == 0 { 20 } else { 5 });
                let buddy = if seed % 3 == 1 { 0 } else { OC1350_BUDDY };
                c.bus.write32(GAME_GP + 0x238, buddy);
                c.bus.write32(OC1350_BUDDY_ANSWER, (seed >> 2) % 2);
                c.bus.write32(GAME_GP - 0xc8, OC1350_LOBBY);
                c.bus.write32(OC1350_LOBBY + 0x98, oc1350_word(&mut r));
                c.g[A0][0] = sx(comm);
            },
        );
    }

    #[test]
    fn c_game_comm_on_race_end_matches() {
        oc1350_install_fakes();
        check(
            ee::s06::f_00258a48__generated,
            c_game_comm_on_race_end,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0025_8a48, 1);
                c.pc = 0x0025_8a48;
                c.bus.write32(OC1350_CTRL, OC1350_MAGIC);
                oc1350_world(c, &mut r);
                let comm = OC1350_COMM;
                for w in 0..0x140u32 {
                    c.bus.write32(comm + 4 * w, oc1350_word(&mut r));
                }
                c.bus
                    .write32(comm + 0x70, if seed % 2 == 0 { 20 } else { 3 });
                c.bus.write32(
                    comm + 0x38,
                    if seed % 3 == 0 {
                        0
                    } else {
                        oc1350_word(&mut r)
                    },
                );
                c.bus.write32(
                    comm + 0x3c,
                    if (seed >> 1) % 2 == 0 {
                        0
                    } else {
                        oc1350_word(&mut r)
                    },
                );
                c.g[A0][0] = sx(comm);
            },
        );
    }

    #[test]
    fn c_game_comm_quit_race_matches() {
        oc1350_install_fakes();
        check(
            ee::s06::f_00258ae0__generated,
            c_game_comm_quit_race,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0025_8ae0, 3);
                c.pc = 0x0025_8ae0;
                c.bus.write32(OC1350_CTRL, OC1350_MAGIC);
                oc1350_world(c, &mut r);
                let comm = OC1350_COMM;
                for w in 0..0x140u32 {
                    c.bus.write32(comm + 4 * w, oc1350_word(&mut r));
                }
                // After the fill: the send queue must read empty (heads at
                // +0x364/+0x368), or real FlushSendQueue walks garbage.
                oc1350_packet_rig(c);
                c.bus.write32(
                    comm + 0x4,
                    if (seed >> 5) % 2 == 0 {
                        0
                    } else {
                        oc1350_word(&mut r)
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
                        oc1350_word(&mut r)
                    },
                );
                c.bus.write32(
                    comm + 0x3c,
                    if (seed >> 1) % 2 == 0 {
                        0
                    } else {
                        oc1350_word(&mut r)
                    },
                );
                let buddy = if seed % 3 == 1 { 0 } else { OC1350_BUDDY };
                c.bus.write32(GAME_GP + 0x238, buddy);
                c.bus.write32(OC1350_BUDDY_ANSWER, (seed >> 2) % 2);
                c.bus.write32(GAME_GP - 0xc8, OC1350_LOBBY);
                c.bus.write32(OC1350_LOBBY + 0x98, oc1350_word(&mut r));
                // The wait-popup word and the fake interface chain.
                c.bus.write32(0x0053_4b38, (seed >> 4) % 2);
                let iface = OC1350_COMM + 0x400;
                c.bus.write32(OC1350_BE_IFACE, iface);
                c.bus.write32(OC1350_BE, OC1350_LOBBY);
                c.bus.write32(iface + 0xc, OC1350_VT);
                c.bus.write16(OC1350_VT + 0x10, 0);
                c.bus.write32(OC1350_VT + 0x14, 0x0000_5000);
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
                oc1350_world(c, &mut r);
                let comm = OC1350_COMM;
                for w in 0..0x140u32 {
                    c.bus.write32(comm + 4 * w, oc1350_word(&mut r));
                }
                // After the fill: the send queue must read empty (heads at
                // +0x364/+0x368), or real FlushSendQueue walks garbage.
                oc1350_packet_rig(c);
                c.bus.write32(
                    comm + 0x4,
                    if seed % 2 == 0 {
                        0
                    } else {
                        oc1350_word(&mut r)
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
                oc1350_world(c, &mut r);
                let comm = OC1350_COMM;
                for w in 0..0x140u32 {
                    c.bus.write32(comm + 4 * w, oc1350_word(&mut r));
                }
                // After the fill: the send queue must read empty (heads at
                // +0x364/+0x368), or real FlushSendQueue walks garbage.
                oc1350_packet_rig(c);
                c.bus.write32(
                    comm + 0x4,
                    if seed % 3 == 0 {
                        oc1350_word(&mut r)
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
                oc1350_world(c, &mut r);
                let comm = OC1350_COMM;
                for w in 0..0x140u32 {
                    c.bus.write32(comm + 4 * w, oc1350_word(&mut r));
                }
                // After the fill: the send queue must read empty (heads at
                // +0x364/+0x368), or real FlushSendQueue walks garbage.
                oc1350_packet_rig(c);
                c.bus.write32(
                    comm + 0x4,
                    if seed % 3 == 0 {
                        oc1350_word(&mut r)
                    } else {
                        0
                    },
                );
                c.bus.write32(comm, if seed % 3 == 1 { 0 } else { comm });
                c.bus.write32(comm + 0x6c, (seed >> 1) % 2);
                let len = seed % 24;
                for i in 0..len {
                    c.bus.write8(OC1350_STR + i, 0x41 + ((seed + i) % 26) as u8);
                }
                c.bus.write8(OC1350_STR + len, 0);
                c.g[A0][0] = sx(comm);
                c.g[A1][0] = sx(OC1350_STR);
            },
        );
    }

    use crate::difftest::events::{check_events, driven, schedule};

    /// Shared fixture: comm object with an empty send queue and unmapped
    /// channels (ChannelSend lands on the miss-hook fake and reports
    /// success), on a miss-hooked machine.
    fn wait_for_peer_setup(c: &mut Cpu, seed: u32, flag_3f4: u32, flag_4: u32) {
        let mut r = fakes::machine(c, seed, 0x0025_86b0, 1);
        wait_for_peer_setup_r(c, seed, &mut r, flag_3f4, flag_4);
    }

    fn wait_for_peer_setup_r(c: &mut Cpu, seed: u32, r: &mut Rng, flag_3f4: u32, flag_4: u32) {
        c.pc = 0x0025_86b0;
        oc1350_install_fakes();
        // Arm the gate so the gated callees (yield) take their test
        // doubles instead of the real bodies.
        c.bus.write32(OC1350_CTRL, OC1350_MAGIC);
        oc1350_world(c, r);
        let comm = OC1350_COMM;
        for w in 0..0x140u32 {
            c.bus.write32(comm + 4 * w, oc1350_word(r));
        }
        // After the fill: the send queue must read empty.
        oc1350_packet_rig(c);
        c.bus.write32(comm + 0x3f4, flag_3f4);
        // Pin Update's cheap path (see the waits test): `+0x8 = 1` takes
        // CheckConnection's TickQuitCountdown early-out every pass, and
        // `+0x138` stays positive so every pass stays on the light path.
        c.bus.write32(comm + 0x8, 1);
        c.bus.write32(comm + 0x138, 16);
        c.bus.write32(comm + 0x4, flag_4);
        // The gated yield fake counts ticks here; the ready report lands on
        // the 3rd tick (reset per side by the test setup).
        c.bus.write32(OC1350_YIELD_HITS, 0);
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
    /// Termination rig (documented, not hidden): real `cGameComm_Update`
    /// only reports ready via its `CheckConnection` packet path, which is a
    /// fixture lottery over the unported receive code — about half the
    /// random fills spin forever on BOTH sides. So the fixture pins
    /// Update's cheap path (`+0x8 = 1` takes the `TickQuitCountdown`
    /// early-out, `+0x138 = 16` keeps every pass on the light path) and
    /// the gated yield fake below reports the peer ready on the 3rd tick,
    /// the way a real peer arrival would land mid-wait. Both sides share
    /// the same fake and the same fixture, so the comparison stays exact.
    #[test]
    fn c_game_comm_wait_for_peer_ready_waits() {
        check_events(wait_ready_gen, wait_ready_hand, 60, &|c, seed| {
            // No fake-call unwinds here: the wait loop is terminated by the
            // scheduled event unwinds, not by miss-hook unwinds (whose
            // 0xdead0000 pc the driver does not resume).
            let mut r = fakes::machine(c, seed, 0x0025_86b0, 0);
            wait_for_peer_setup_r(c, seed, &mut r, 0, 0);
            schedule(c, &mut r, seed, 0x0025_86b0)
        });
    }
}
