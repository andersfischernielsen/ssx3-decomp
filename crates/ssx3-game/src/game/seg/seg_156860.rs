//! `src/seg/seg_156860.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0025_5a10,
        c_comm_system_get_version_tag,
        "cCommSystem_GetVersionTag",
    ),
    (0x0025_5a20, c_comm_system_update, "cCommSystem_Update"),
    (
        0x0025_5d38,
        c_comm_system_has_channel,
        "cCommSystem_HasChannel",
    ),
];

/// `const char *cCommSystem_GetVersionTag(void)` (0x255a10): returns the
/// version-tag string at 0x481370.
pub fn c_comm_system_get_version_tag(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = sx(0x48_0000u32.wrapping_add(0x1370));
    abi::ret(c);
}

/// `bool cCommSystem_HasChannel(cCommSystem *self, int index)` (0x255d38):
/// whether `self->channels[index]` is non-null.
pub fn c_comm_system_has_channel(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A1][0] = sx(abi::arg(c, 1) << 2);
    c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    abi::ret(c);
}

/// Ticks of connect timeout from link-up to a local address.
const ADDRESS_WAIT_TICKS: u32 = 1800;

/// `cCommSystem` fields, its connection states and the DirtySock `NetConn`
/// calls it makes. The game's names for these are unknown.
mod comm_system {
    /// Index of the active entry in [`NET_CONFIGS`]; `cCommSystem_SetError`
    /// sets it to -1 and disconnects when it is not already negative.
    pub const CONFIG_INDEX: u32 = 0x1c;
    /// Connection state, one of the `STATE_*` values.
    pub const STATE: u32 = 0x20;
    /// Connect timeout: ticks left before the attempt fails with
    /// [`ERROR_TIMEOUT`].
    pub const CONNECT_TIMER: u32 = 0x24;
    /// Error passed to `cCommSystem_SetError`; `cCommSystem_GetChannelStatus`
    /// reads it.
    #[cfg(test)]
    pub const ERROR_CODE: u32 = 0x28;
    /// Set when the `'bbnd'` status reports a broadband link.
    pub const BROADBAND: u32 = 0x2c;
    /// The `'type'` status of the connection.
    pub const CONNECTION_TYPE: u32 = 0x30;

    /// Requests the connection unless `NetConn` is already online.
    #[cfg(test)]
    pub const STATE_REQUEST_CONNECT: u64 = 2;
    /// Waiting for `NetConn` to come online (after a connect request).
    pub const STATE_CONNECTING: u64 = 3;
    /// Online; waiting for a local address.
    pub const STATE_ADDRESS_WAIT: u64 = 4;
    /// Connected; the channel is serviced every tick.
    pub const STATE_CONNECTED: u64 = 5;
    /// Failed; written by `cCommSystem_SetError`. Takes the idle arm.
    #[cfg(test)]
    pub const STATE_ERROR: u64 = 6;
    /// States at or past this skip the state table.
    pub const STATE_COUNT: u64 = 7;
    /// Handler address for each state, in `.rodata`.
    pub const STATE_TABLE: u32 = 0x0048_0330;
    /// The handlers in [`STATE_TABLE`]: `STATE_REQUEST_CONNECT` and on.
    pub const HANDLER_REQUEST_CONNECT: u32 = 0x0025_5a64;
    pub const HANDLER_CONNECTING: u32 = 0x0025_5aa8;
    pub const HANDLER_ADDRESS_WAIT: u32 = 0x0025_5b5c;
    pub const HANDLER_CONNECTED: u32 = 0x0025_5b80;
    /// States 0, 1 and `STATE_ERROR`: nothing but the connect timeout.
    pub const HANDLER_IDLE: u32 = 0x0025_5bb4;

    /// Connection settings table, indexed by [`CONFIG_INDEX`].
    pub const NET_CONFIGS: u32 = 0x0053_6810;
    /// Bytes per [`NET_CONFIGS`] entry.
    pub const CONFIG_STRIDE: u32 = 0x100;

    /// `NetConnStatus` selector: non-zero once online.
    pub const STATUS_ONLINE: u32 = u32::from_be_bytes(*b"onln");
    /// `NetConnStatus` selector: non-zero once the host address is known.
    pub const STATUS_HOST: u32 = u32::from_be_bytes(*b"host");
    /// `NetConnStatus` selector: the connection type.
    pub const STATUS_TYPE: u32 = u32::from_be_bytes(*b"type");
    /// `NetConnStatus` selector: 1 on a broadband link.
    pub const STATUS_BROADBAND: u32 = u32::from_be_bytes(*b"bbnd");
    /// `NetConnStatus` selector: progress of the connection; a reply whose
    /// top byte is [`ERROR_PREFIX`] is an error code.
    pub const STATUS_CONNECT: u32 = u32::from_be_bytes(*b"conn");
    /// Top byte of a `'conn'` reply that reports an error.
    pub const ERROR_PREFIX: u64 = b'-' as u64;
    /// Error recorded when [`CONNECT_TIMER`] runs out.
    pub const ERROR_TIMEOUT: u32 = u32::from_be_bytes(*b"time");

    /// `cDSockChannel` field: its vtable.
    pub const CHANNEL_VTABLE: u32 = 8;
    /// Vtable entry: `this` adjustment (halfword) for the update method.
    pub const UPDATE_DELTA: u32 = 0x18;
    /// Vtable entry: the update method.
    pub const UPDATE_METHOD: u32 = 0x1c;

    /// `NetConnIdle` (0x3f42a0): pump the network library.
    pub const NET_CONN_IDLE: u32 = 0x003f_42a0;
    /// `int NetConnStatus(int selector, void *buffer, int size)` (0x3f3cc0).
    pub const NET_CONN_STATUS: u32 = 0x003f_3cc0;
    /// `NetConnConnect` (0x3f3c50): start connecting with a settings entry.
    pub const NET_CONN_CONNECT: u32 = 0x003f_3c50;
}

/// `void cCommSystem_Update(cCommSystem *self)` (0x255a20): advance the
/// connection handshake, discover the local address, service its channel,
/// and report an error when the connect timeout expires.
pub fn c_comm_system_update(c: &mut Cpu) {
    use comm_system::*;
    const LOCAL_ADDRESS: u32 = 0;
    const SAVED_RA: u32 = 0x10;
    const SAVED_CHANNEL_INDEX: u32 = 0x20;
    const SAVED_SELF: u32 = 0x30;
    const SAVED_CHANNEL: u32 = 0x40;
    const SET_ERROR: Callee = Callee {
        addr: 0x0025_5fe8,
        f: ssx3_ee::s06::cCommSystem_SetError,
    };
    const GET_LOCAL_ADDRESS: Callee = Callee {
        addr: 0x0025_6038,
        f: ssx3_ee::s06::cCommSystem_GetLocalAddr,
    };
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, SAVED_SELF);
        frame.save(c, S0, SAVED_CHANNEL);
        frame.save(c, S2, SAVED_CHANNEL_INDEX);
        frame.save_ra(c, SAVED_RA);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, NET_CONN_IDLE, 0x0025_5a3c)?;
        let comm = Addr(abi::r32(c, S1));
        c.g[V1][0] = comm.off(STATE).lw(c);
        c.g[V0][0] = (c.g[V1][0] < STATE_COUNT) as u64;
        let known_state = c.g[V0][0] != 0;
        c.g[V0][0] = (STATE_TABLE & 0xffff_0000) as u64;
        let mut reload_timer = true;
        if known_state {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx(STATE_TABLE);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = Addr(abi::r32(c, V1)).lw(c);
            match c.g[A0][0] as u32 {
                HANDLER_REQUEST_CONNECT => {
                    // STATE_REQUEST_CONNECT: connect if not online yet.
                    c.cycles += 7;
                    c.g[A0][0] = STATUS_ONLINE as u64;
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 0;
                    call_ptr(c, NET_CONN_STATUS, 0x0025_5a78)?;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = comm.off(CONNECT_TIMER).lw(c);
                        reload_timer = false;
                    } else {
                        c.cycles += 10;
                        c.g[V0][0] = comm.off(CONFIG_INDEX).lw(c);
                        c.g[A0][0] = sx(NET_CONFIGS);
                        c.g[A1][0] = 0;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_mul(CONFIG_STRIDE));
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, NET_CONN_CONNECT, 0x0025_5a9c)?;
                        c.g[V1][0] = STATE_CONNECTING;
                        comm.off(STATE).sw(c, c.g[V1][0]);
                    }
                }
                HANDLER_CONNECTING => {
                    // STATE_CONNECTING: online with a host arms the address wait.
                    c.cycles += 7;
                    c.g[A0][0] = STATUS_ONLINE as u64;
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 0;
                    call_ptr(c, NET_CONN_STATUS, 0x0025_5abc)?;
                    let online = c.g[V0][0] != 0;
                    if online {
                        c.cycles += 6;
                        c.g[A1][0] = 0;
                        c.g[A0][0] = STATUS_HOST as u64;
                        c.g[A2][0] = 0;
                        call_ptr(c, NET_CONN_STATUS, 0x0025_5ad4)?;
                        let host_ready = c.g[V0][0] != 0;
                        c.g[V1][0] = ADDRESS_WAIT_TICKS as u64;
                        if host_ready {
                            c.cycles += 18;
                            c.g[V0][0] = STATE_ADDRESS_WAIT;
                            comm.off(CONNECT_TIMER).sw(c, c.g[V1][0]);
                            c.g[A0][0] = STATUS_TYPE as u64;
                            c.g[A1][0] = 0;
                            c.g[A2][0] = 0;
                            comm.off(STATE).sw(c, c.g[V0][0]);
                            call_ptr(c, NET_CONN_STATUS, 0x0025_5afc)?;
                            comm.off(CONNECTION_TYPE).sw(c, c.g[V0][0]);
                            c.g[A0][0] = STATUS_BROADBAND as u64;
                            c.g[A1][0] = 0;
                            c.g[A2][0] = 0;
                            call_ptr(c, NET_CONN_STATUS, 0x0025_5b14)?;
                            c.g[V1][0] = c.g[V0][0];
                            c.g[V0][0] = 1;
                            if c.g[V1][0] == c.g[V0][0] {
                                comm.off(BROADBAND).sw(c, c.g[V1][0]);
                            }
                        }
                    }
                    c.cycles += 10;
                    c.g[A0][0] = STATUS_CONNECT as u64;
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 0;
                    call_ptr(c, NET_CONN_STATUS, 0x0025_5b38)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V1][0] = ERROR_PREFIX;
                    c.g[V0][0] = sx(((c.g[A1][0] as u32 as i32) >> 24) as u32);
                    if c.g[V0][0] != c.g[V1][0] {
                        c.g[V0][0] = comm.off(CONNECT_TIMER).lw(c);
                        reload_timer = false;
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S1][0];
                        SET_ERROR.call(c, 0x0025_5b54)?;
                        comm.off(CONNECT_TIMER).sw(c, 0);
                    }
                }
                HANDLER_ADDRESS_WAIT => {
                    // STATE_ADDRESS_WAIT: stop waiting once there is an address.
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[SP][0];
                    GET_LOCAL_ADDRESS.call(c, 0x0025_5b68)?;
                    c.g[V0][0] = Addr(frame.at(c, LOCAL_ADDRESS)).lw(c);
                    let has_address = c.g[V0][0] != 0;
                    c.g[V0][0] = STATE_CONNECTED;
                    if has_address {
                        c.cycles += 3;
                        comm.off(CONNECT_TIMER).sw(c, 0);
                        comm.off(STATE).sw(c, c.g[V0][0]);
                    }
                }
                HANDLER_CONNECTED => {
                    // STATE_CONNECTED: update the channel. One slot; the loop
                    // is kept as compiled.
                    c.cycles += 2;
                    c.g[S0][0] = c.g[S1][0];
                    c.g[S2][0] = 0;
                    loop {
                        c.cycles += 3;
                        c.g[A1][0] = Addr(abi::r32(c, S0)).lw(c);
                        if c.g[A1][0] != 0 {
                            c.cycles += 6;
                            c.g[V0][0] = Addr(abi::r32(c, A1)).off(CHANNEL_VTABLE).lw(c);
                            let vtable = Addr(abi::r32(c, V0));
                            c.g[A0][0] = vtable.off(UPDATE_DELTA).lh(c);
                            c.g[V1][0] = vtable.off(UPDATE_METHOD).lw(c);
                            let update = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, update, 0x0025_5ba8)?;
                        }
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                        c.cycles += 2;
                        let again = (c.g[S2][0] as i64) >= 0;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                        if !again {
                            break;
                        }
                        if crate::events::event_due(c) {
                            crate::events::deliver_event_here(c, 0x0025_5b88);
                        }
                    }
                }
                // Nothing to do but run the connect timeout below.
                HANDLER_IDLE => {}
                target => {
                    c.jump_out(target, 0x0025_5a20, 0x0025_5bf4);
                    return Ok(());
                }
            }
        }
        if reload_timer {
            c.cycles += 1;
            c.g[V0][0] = comm.off(CONNECT_TIMER).lw(c);
        }
        c.cycles += 2;
        let timer_running = (c.g[V0][0] as i64) > 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        if timer_running {
            c.cycles += 2;
            let expired = (c.g[V0][0] as i64) <= 0;
            comm.off(CONNECT_TIMER).sw(c, c.g[V0][0]);
            if expired {
                c.cycles += 5;
                c.g[A1][0] = ERROR_TIMEOUT as u64;
                c.g[A0][0] = c.g[S1][0];
                SET_ERROR.call(c, 0x0025_5bd8)?;
                comm.off(CONNECT_TIMER).sw(c, 0);
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, SAVED_CHANNEL);
        frame.restore(c, S1, SAVED_SELF);
        frame.restore(c, S2, SAVED_CHANNEL_INDEX);
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

    /// `NetConnIdle`'s first callback entry: function, then its context.
    const IDLE_CALLBACK_FUNCTION: u32 = 0x0051_f030;
    const IDLE_CALLBACK_CONTEXT: u32 = 0x0051_f034;
    /// gp-relative `NetConn` "started" flag; clear, so idle skips its ticking.
    const NET_CONN_STARTED_GP: u32 = 0x1ab0;
    /// Fake idle callback, and the reply words it plants for `NetConnStatus`.
    const IDLE_CALLBACK: u32 = crate::difftest::fakes::FAKE + 0x10;
    const REPLY_WORD: u32 = DATA + 0x4000;
    const SECOND_REPLY_WORD: u32 = DATA + 0x4004;
    /// Stack frames between the idle callback and the slots read as replies.
    /// `NetConnStatus` reads the first reply for `'onln'`, `'conn'` and
    /// `'type'`, and the second for `'host'` and `'addr'`.
    const IDLE_FRAME: u32 = 0x30;
    const STATUS_FRAME: u32 = 0x80;
    const GET_ADDRESS_FRAME: u32 = 0x20;
    /// `NetConnStatus` reads its replies at these frame offsets.
    const STATUS_REPLY: u32 = 0x20;
    const STATUS_SECOND_REPLY: u32 = 0x24;
    /// Fixture channel object and its vtable.
    const CHANNEL: u32 = DATA + 0x1000;
    const CHANNEL_VTABLE_BLOCK: u32 = DATA + 0x2000;

    /// SetError's value when it has not run.
    const UNTOUCHED_ERROR: u32 = 0x5555_aaaa;

    /// 0-6 are the table's states, 7 is just past it, 8 is `u32::MAX`.
    const STATE_CASES: u32 = 9;
    const CONNECT_TIMERS: [u32; 7] = [0, 1, 2, 1800, u32::MAX, 0x8000_0000, 0x7fff_ffff];
    /// (first reply, second reply): offline; online, no host; online with a
    /// host; the same on broadband; a `'-'` error; an address; offline again.
    const REPLIES: [(u32, u32); 7] = [
        (0, 0),
        (1, 0),
        (1, 1),
        (4, 1),
        (0x2d00_0001, 0),
        (1, 0x7f00_0001),
        (0, 0x80),
    ];
    /// Every (timer, reply) pair, for each state.
    const UPDATE_SEEDS: u32 = STATE_CASES * (CONNECT_TIMERS.len() * REPLIES.len()) as u32;

    /// Indices into [`CONNECT_TIMERS`].
    const TIMER_STOPPED: u32 = 0;
    const TIMER_ONE: u32 = 1;
    const TIMER_TWO: u32 = 2;
    const TIMER_1800: u32 = 3;
    const TIMER_NEGATIVE: u32 = 4;
    /// Indices into [`REPLIES`].
    const REPLY_OFFLINE: u32 = 0;
    const REPLY_ONLINE: u32 = 1;
    const REPLY_HOST: u32 = 2;
    const REPLY_BROADBAND: u32 = 3;
    const REPLY_ERROR: u32 = 4;
    const REPLY_ADDRESS: u32 = 5;

    /// The case that pairs timer index `timer` with reply index `reply`.
    fn case(timer: u32, reply: u32) -> u32 {
        timer + CONNECT_TIMERS.len() as u32 * reply
    }

    /// Whether the fixture for `case` plants a channel in the object's one
    /// slot. Odd cases do, so every timer and every reply meets both outcomes.
    fn has_channel(case: u32) -> bool {
        case % 2 != 0
    }

    /// The seed that runs `case` (a timer and reply pair) in `state`.
    fn seed(case: u32, state: u32) -> u32 {
        case * STATE_CASES + state
    }

    /// Runs the fake idle callback, then leaves the fixture's replies in the
    /// stack slots `NetConnStatus` will read. With an empty RPC table the send
    /// returns without replacing them, so the real `NetConnIdle`,
    /// `NetConnStatus` and `cCommSystem_GetLocalAddr` reach their
    /// success, pending, error and address arms without a network.
    fn comm_fixture_callback(c: &mut Cpu, target: u32) {
        use crate::difftest::fakes;
        let ret = c.g[RA][0] as u32;
        fakes::fake_call(c, target);
        if target == IDLE_CALLBACK && c.pc == ret {
            let status_sp = c.g[SP][0] as u32 + IDLE_FRAME - STATUS_FRAME;
            let reply = c.bus.read32(REPLY_WORD);
            let second = c.bus.read32(SECOND_REPLY_WORD);
            c.bus.write32(status_sp + STATUS_REPLY, reply);
            c.bus.write32(status_sp + STATUS_SECOND_REPLY, second);
            // The same reply, one frame deeper under cCommSystem_GetLocalAddr.
            c.bus
                .write32(status_sp - GET_ADDRESS_FRAME + STATUS_SECOND_REPLY, second);
        }
    }

    fn comm_update_setup(c: &mut Cpu, seed: u32) {
        use crate::difftest::{fakes, GAME_GP};
        use comm_system::*;
        fakes::machine(c, seed, 0x255a_2000, 3);
        c.hooks.miss = comm_fixture_callback;
        c.pc = 0x0025_5a20;
        c.g[A0][0] = sx(DATA);
        let state = seed % STATE_CASES;
        let case = (seed / STATE_CASES) as usize;
        c.bus
            .write32(DATA + STATE, if state == 8 { u32::MAX } else { state });
        // A negative index keeps SetError from disconnecting.
        c.bus.write32(DATA + CONFIG_INDEX, u32::MAX);
        c.bus.write32(DATA + ERROR_CODE, UNTOUCHED_ERROR);
        // Separate axes: every timer value meets every reply.
        c.bus.write32(
            DATA + CONNECT_TIMER,
            CONNECT_TIMERS[case % CONNECT_TIMERS.len()],
        );
        // The test machine has no .rodata, so the fixture plants the table.
        for (index, target) in [
            HANDLER_IDLE,
            HANDLER_IDLE,
            HANDLER_REQUEST_CONNECT,
            HANDLER_CONNECTING,
            HANDLER_ADDRESS_WAIT,
            HANDLER_CONNECTED,
            HANDLER_IDLE,
        ]
        .into_iter()
        .enumerate()
        {
            c.bus.write32(STATE_TABLE + index as u32 * 4, target);
        }
        let (reply, second) = REPLIES[(case / CONNECT_TIMERS.len()) % REPLIES.len()];
        c.bus.write32(REPLY_WORD, reply);
        c.bus.write32(SECOND_REPLY_WORD, second);
        c.bus.write32(IDLE_CALLBACK_FUNCTION, IDLE_CALLBACK);
        c.bus.write32(IDLE_CALLBACK_CONTEXT, DATA);
        c.bus.write32(GAME_GP + NET_CONN_STARTED_GP, 0);
        if has_channel(case as u32) {
            c.bus.write32(DATA, CHANNEL);
            c.bus
                .write32(CHANNEL + CHANNEL_VTABLE, CHANNEL_VTABLE_BLOCK);
            c.bus
                .write16(CHANNEL_VTABLE_BLOCK + UPDATE_DELTA, (-8i16) as u16);
            c.bus
                .write32(CHANNEL_VTABLE_BLOCK + UPDATE_METHOD, fakes::FAKE);
        }
    }

    #[test]
    fn c_comm_system_update_matches() {
        check_paths(
            ssx3_ee::s06::f_00255a20__generated,
            c_comm_system_update,
            UPDATE_SEEDS,
            // Reached costs. A cost can be shared by several paths, so
            // `c_comm_system_update_transitions` asserts what the main arms
            // write.
            &[
                263, 269, 271, 276, 288, 293, 388, 416, 419, 494, 529, 670, 692, 985,
            ],
            &comm_update_setup,
        );
    }

    /// Literal values, so these do not move with the port's constants.
    #[test]
    fn c_comm_system_update_transitions() {
        use crate::difftest::{fresh, run_generated, RA};
        use comm_system::*;
        const TIME: u32 = u32::from_be_bytes(*b"time");
        const OK: u32 = UNTOUCHED_ERROR;
        #[rustfmt::skip]
        let rows = [
            // timer, reply, state, next state, timer after, error, broadband
            (TIMER_STOPPED, REPLY_OFFLINE, STATE_REQUEST_CONNECT, 3, 0, OK, 0),
            (TIMER_STOPPED, REPLY_ONLINE, STATE_REQUEST_CONNECT, 2, 0, OK, 0),
            (TIMER_STOPPED, REPLY_OFFLINE, STATE_CONNECTING, 3, 0, OK, 0),
            (TIMER_STOPPED, REPLY_HOST, STATE_CONNECTING, 4, 1799, OK, 0),
            (TIMER_STOPPED, REPLY_BROADBAND, STATE_CONNECTING, 4, 1799, OK, 1),
            (TIMER_STOPPED, REPLY_ERROR, STATE_CONNECTING, 6, 0, 0x2d00_0001, 0),
            (TIMER_STOPPED, REPLY_ADDRESS, STATE_ADDRESS_WAIT, 5, 0, OK, 0),
            (TIMER_STOPPED, REPLY_OFFLINE, STATE_ADDRESS_WAIT, 4, 0, OK, 0),
            (TIMER_TWO, REPLY_OFFLINE, 0, 0, 1, OK, 0),
            (TIMER_ONE, REPLY_OFFLINE, 1, 6, 0, TIME, 0),
            (TIMER_STOPPED, REPLY_OFFLINE, STATE_ERROR, 6, 0, OK, 0),
            (TIMER_STOPPED, REPLY_OFFLINE, 7, 7, 0, OK, 0),
            (TIMER_STOPPED, REPLY_ONLINE, STATE_CONNECTING, 3, 0, OK, 0),
            (TIMER_NEGATIVE, REPLY_OFFLINE, 0, 0, u32::MAX, OK, 0),
            // State case 8 plants u32::MAX.
            (TIMER_STOPPED, REPLY_OFFLINE, 8, u32::MAX, 0, OK, 0),
        ];
        for (timer, reply, state, next, after, error, broadband) in rows {
            let label = format!("timer {timer} reply {reply} state {state}");
            let mut c = fresh();
            comm_update_setup(&mut c, seed(case(timer, reply), state as u32));
            run_generated(&mut c, c_comm_system_update);
            assert_eq!(c.pc, RA, "{label}: unwound");
            assert_eq!(c.bus.read32(DATA + STATE), next, "{label}");
            assert_eq!(c.bus.read32(DATA + CONNECT_TIMER), after, "{label}");
            assert_eq!(c.bus.read32(DATA + ERROR_CODE), error, "{label}");
            assert_eq!(c.bus.read32(DATA + BROADBAND), broadband, "{label}");
        }
    }

    /// The connected state calls the channel's update only when there is one.
    #[test]
    fn c_comm_system_update_services_the_channel() {
        use crate::difftest::{fakes, fresh, run_generated, RA};
        use comm_system::*;
        for (timer, channel) in [(TIMER_STOPPED, false), (TIMER_1800, true)] {
            let pair = case(timer, REPLY_OFFLINE);
            assert_eq!(has_channel(pair), channel, "{timer}: fixture");
            let mut c = fresh();
            comm_update_setup(&mut c, seed(pair, STATE_CONNECTED as u32));
            run_generated(&mut c, c_comm_system_update);
            assert_eq!(c.pc, RA, "{timer}: unwound");
            // The idle callback, then the channel's update if there is one.
            let calls = 1 + channel as u32;
            assert_eq!(c.bus.read32(fakes::LOG_COUNT), calls, "{timer}");
        }
    }

    #[test]
    fn c_comm_system_get_version_tag_matches() {
        check(
            ssx3_ee::s06::f_00255a10,
            c_comm_system_get_version_tag,
            30,
            &|c, seed| {
                let mut r = Rng(0x255a_1000 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn c_comm_system_has_channel_matches() {
        check(
            ssx3_ee::s06::f_00255d38,
            c_comm_system_has_channel,
            200,
            &|c, seed| {
                let mut r = Rng(0x255d_3800 ^ seed as u64);
                scramble(c, &mut r);
                let idx = (seed % 4) as u32;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 2 == 0 {
                    idx as u64
                } else {
                    0x1_0000_0000u64 | idx as u64
                };
                c.bus.write32(
                    DATA + idx * 4,
                    match seed % 5 {
                        0 => 0,
                        1 => 1,
                        2 => 0x8000_0000,
                        3 => 0x7fff_ffff,
                        _ => r.next_u32(),
                    },
                );
            },
        );
    }
}
