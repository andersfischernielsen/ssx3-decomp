//! `src/dirtysock/gamecomm.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::game::ai::rider::gp_at;
use crate::game::bx::memblockman::cd_tray::GP_TRAY_OPENED;
use crate::game::misc::RACE_WORLD_GAME_MODULE;
use crate::game::misc::RACE_WORLD_GP;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
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
    const LATCH_RESULTS: Callee = Callee {
        addr: 0x0025_7678,
        f: ssx3_ee::s06::cGameComm_LatchResultFlags,
    };
    const END_SESSION: Callee = Callee {
        addr: 0x0025_78f8,
        f: ssx3_ee::s06::cGameComm_EndSession,
    };
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
                        LATCH_RESULTS.call(c, 0x0025_7ba4)?;
                        c.g[A0][0] = c.g[S0][0];
                        END_SESSION.call(c, 0x0025_7bac)?;
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
                END_SESSION.call(c, 0x0025_7bc8)?;
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
            if seed % 7 == 0 { 1 } else { u32::MAX },
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
}
