//! Lobby networking and string cipher utilities.

use crate::abi::{addu, r32, ret};
use crate::call::{call_ptr, guest, tail_wrapper, Callee, Frame, Target, Unwound};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::game::lib::libc::callee::UDIVDI3;
use crate::game::misc::MEMSET;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

mod callee {
    use super::{ee, Callee};

    pub(super) const NET_TICK: Callee = Callee {
        addr: 0x003f_4de8,
        f: ee::s14::NetTick,
    };
    pub(super) const NET_CONN_DISCONNECT: Callee = Callee {
        addr: 0x003f_40b8,
        f: ee::s14::NetConnDisconnect,
    };
    pub(super) const DIRTY_RPC_PROCESS_REPLIES: Callee = Callee {
        addr: 0x003f_4a78,
        f: ee::s14::DirtyRpc_ProcessReplies,
    };
    pub(super) const DIRTY_RPC_INIT_REQUEST: Callee = Callee {
        addr: 0x003f_4c98,
        f: ee::s14::DirtyRpc_InitRequest,
    };
    pub(super) const DIRTY_RPC_CALL: Callee = Callee {
        addr: 0x003f_4c68,
        f: ee::s14::DirtyRpc_Call,
    };
    pub(super) const NET_CONN_STATUS: Callee = Callee {
        addr: 0x003f_3cc0,
        f: ee::s14::NetConnStatus,
    };
    pub(super) const DIRTY_RPC_IS_STARTED: Callee = Callee {
        addr: 0x003f_5470,
        f: ee::s14::DirtyRpc_IsStarted,
    };
    pub(super) const DIRTY_RPC_STARTUP: Callee = Callee {
        addr: 0x003f_5000,
        f: ee::s14::DirtyRpc_Startup,
    };
    pub(super) const DIRTY_RPC_INIT_IOP: Callee = Callee {
        addr: 0x003f_4d68,
        f: ee::s14::DirtyRpc_InitIop,
    };
    pub(super) const NET_RPC_ALLOC: Callee = Callee {
        addr: 0x0025_6bb0,
        f: ee::s06::Net_RpcAlloc,
    };
    pub(super) const NET_RPC_FREE: Callee = Callee {
        addr: 0x0025_6bd8,
        f: ee::s06::Net_RpcFree,
    };
    pub(super) const SOCKET_LOOKUP_CREATE: Callee = Callee {
        addr: 0x003f_39c8,
        f: ee::s14::SocketLookup_Create,
    };
    pub(super) const LOBBY_CIPHER_INIT: Callee = Callee {
        addr: 0x003f_33b0,
        f: ee::s14::Lobby_CipherInit,
    };
    pub(super) const LOBBY_CIPHER_APPLY: Callee = Callee {
        addr: 0x003f_3530,
        f: ee::s14::Lobby_CipherApply,
    };
}

use callee::*;

const DIRTY_RPC_STATE_HI: u64 = 0x0052_0000;
const DIRTY_RPC_STATE: u32 = 0x0051_f0c0;
const NET_TICK_COUNT: u32 = 0x4df8;
const NET_TICK_EPOCH: u32 = 0x4e00;
const NET_TICK_MILLISECOND_CYCLES: u64 = 0x48000;
const NET_TICK_RETRY: u32 = 0x003f_4df8;
const TICK_FRAME: u32 = 0x30;
const TICK_SAVED_RA: u32 = 0x20;
const TICK_SAMPLE: u32 = 0;
const TICK_COUNT_SAMPLE: u32 = 4;
const TICK_EPOCH_SAMPLE: u32 = 8;
const TICK_ELAPSED: u32 = 0x10;

const NET_CONN_OPEN_GP: u32 = 0x1ab0;
const NET_CONN_HANDLE_GP: u32 = 0x1ab4;
const NET_CONN_NEXT_IDLE_GP: u32 = 0x1abc;
const NET_CONN_IDLE_CALLBACKS: u32 = 0x0051_f030;
const NET_CONN_IDLE_CALLBACK_COUNT: u64 = 16;
const NET_CONN_IDLE_CALLBACK_ARG: u32 = 4;
const NET_CONN_IDLE_CALLBACK_STRIDE: u32 = 8;
const NET_CONN_IDLE_HEAD: u32 = 0x003f_42f0;
const NET_CONN_IDLE_FRAME: u32 = 0x30;
const NET_CONN_IDLE_SAVED_RA: u32 = 0;
const NET_CONN_IDLE_SAVED_S1: u32 = 0x10;
const NET_CONN_IDLE_SAVED_S0: u32 = 0x20;
const NET_CONN_DELAY_MS: u32 = 5;

const NET_CONN_SLEEP_ENTRY: u32 = 0x003f_41b8;
const NET_CONN_SLEEP_HEAD: u32 = 0x003f_41e0;
const NET_CONN_SLEEP_END: u32 = 0x003f_420c;
const NET_CONN_SLEEP_FRAME: u32 = 0x20;
const NET_CONN_SLEEP_SAVED_S0: u32 = 0x10;

const NET_RPC_SERVICE: u64 = u32::from_be_bytes(*b"netw") as u64;
const NET_RPC_STOP: u64 = u32::from_be_bytes(*b"shut") as u64;
const NET_RPC_DESTROY: u64 = u32::from_be_bytes(*b"dest") as u64;
const NET_RPC_CREATE: u64 = u32::from_be_bytes(*b"crea") as u64;
const NET_RPC_START: u64 = u32::from_be_bytes(*b"star") as u64;
const NET_RPC_LIST: u64 = u32::from_be_bytes(*b"list") as u64;
const NET_RPC_STAT: u64 = u32::from_be_bytes(*b"stat") as u64;
const NET_RPC_CLIENT_ID: u64 = u32::from_be_bytes(*b"1731") as u64;
const NET_RPC_REPLY_WORD: u32 = 0x20;
const NET_RPC_STATUS_WORD: u32 = NET_RPC_REPLY_WORD + 4;
const RPC_WAIT_WITH_OUTPUT: u64 = 5;
const RPC_NUL_TERMINATED: u64 = u64::MAX;
const RPC_QUERY_ENTRY_SHIFT: u32 = 8;
const NET_CONN_STATUS_ENTRY: u32 = 0x003f_3cc0;
const NET_CONN_STATUS_BROADBAND_HEAD: u32 = 0x003f_3dc0;
const NET_CONN_STATUS_END: u32 = 0x003f_3f74;
const NET_CONN_STATUS_FRAME: u32 = 0x80;
const NET_CONN_STATUS_SAVED_RA: u32 = 0x30;
const NET_CONN_STATUS_SAVED_S0: u32 = 0x70;
const NET_CONN_STATUS_SAVED_S1: u32 = 0x60;
const NET_CONN_STATUS_SAVED_S2: u32 = 0x50;
const NET_CONN_STATUS_SAVED_S3: u32 = 0x40;
const NET_STATUS_OPEN: u64 = u32::from_be_bytes(*b"open") as u64;
const NET_STATUS_ONLINE: u64 = u32::from_be_bytes(*b"onln") as u64;
const NET_STATUS_CONNECTION: u64 = u32::from_be_bytes(*b"conn") as u64;
const NET_STATUS_HOST: u64 = u32::from_be_bytes(*b"host") as u64;
const NET_STATUS_ADDRESS: u64 = u32::from_be_bytes(*b"addr") as u64;
const NET_STATUS_MAC: u64 = u32::from_be_bytes(*b"macx") as u64;
const NET_STATUS_BROADBAND: u64 = u32::from_be_bytes(*b"bbnd") as u64;
const NET_STATUS_TYPE: u64 = u32::from_be_bytes(*b"type") as u64;
const NET_STATUS_BROADBAND_BIT: u32 = 2;
const SELECTOR_PREFIX_MASK: u64 = 0xffff_0000;

const NET_MAC_CACHE_BASE: u64 = 0x0045_0000;
const NET_MAC_CACHE_OFFSET: u32 = 0x12e8;
const NET_MAC_REPLY_SIZE: u64 = 16;
const NET_MAC_HEX_BASE: u32 = 0x0049_7028;
const NET_MAC_NIBBLE_MASK: u64 = 0xf;
const NET_MAC_PREFIX: u64 = b'$' as u64;
const NET_MAC_FRAME: u32 = 0x50;
const NET_MAC_SAVED_RA: u32 = 0x10;
const NET_MAC_SAVED_S0: u32 = 0x40;
const NET_MAC_SAVED_S1: u32 = 0x30;
const NET_MAC_SAVED_S2: u32 = 0x20;

const LOOKUP_STATUS: u32 = 0;
const LOOKUP_ADDRESS: u32 = 4;
const LOOKUP_DONE_CALLBACK: u32 = 8;
const LOOKUP_FREE_CALLBACK: u32 = 0xc;
const LOOKUP_REMOTE_HANDLE: u32 = 0x58;
const LOOKUP_SIZE: u32 = 0x5c;
const LOOKUP_RPC_SERVICE: u64 = u32::from_be_bytes(*b"name") as u64;
const LOOKUP_RPC_CREATE: u64 = u32::from_be_bytes(*b"look") as u64;
const LOOKUP_RPC_FREE: u64 = u32::from_be_bytes(*b"free") as u64;
const LOOKUP_RPC_DONE: u64 = u32::from_be_bytes(*b"done") as u64;
const LOOKUP_RPC_ADDRESS: u64 = u32::from_be_bytes(*b"addr") as u64;
const LOOKUP_RPC_DONE_DESC: u32 = 0x0051_f000;
const LOOKUP_RPC_FREE_DESC: u32 = LOOKUP_RPC_DONE_DESC + 0x10;
const LOOKUP_RPC_CREATE_DESC: u32 = LOOKUP_RPC_DONE_DESC + 0x20;
const LOOKUP_SYNC_HEAD: u32 = 0x003f_3ae0;
const LOOKUP_DONE_ADDR: u32 = 0x003f_3860;
const LOOKUP_FREE_ADDR: u32 = 0x003f_3940;
const CIPHER_STRING_FRAME: u32 = 0x1b0;
const CIPHER_STRING_S0: u32 = 0x1a0;
const CIPHER_STRING_S1: u32 = 0x190;
const CIPHER_STRING_S2: u32 = 0x180;
const CIPHER_STRING_S3: u32 = 0x170;
const CIPHER_STRING_S4: u32 = 0x160;
const CIPHER_STRING_S5: u32 = 0x150;
const CIPHER_STRING_S6: u32 = 0x140;
const CIPHER_STRING_S7: u32 = 0x130;
const CIPHER_STRING_RA: u32 = 0x120;
const CIPHER_STRING_BYTE: u32 = 0x110;
const CIPHER_STRING_STREAM_BYTE: u32 = CIPHER_STRING_BYTE + 1;
const CIPHER_PADDING_STATE: u32 = 0x0051_eef8;
const CIPHER_PADDING_BANK: u64 = 0x0052_0000;
const CIPHER_PADDING_KEY: u32 = 0x0049_7018;
const CIPHER_PADDING_MASK: u64 = 0x3f;
const CIPHER_FIRST_PRINTABLE: u32 = b' ' as u32;
const CIPHER_PRINTABLE_COUNT: u64 = 95;
const CIPHER_STRING_ALPHABET: u64 = 96;
const CIPHER_STRING_BIAS: u32 = 0x40;
const CIPHER_STRING_TERMINATOR: u64 = 0x7f;
const CIPHER_ENCRYPT_HEAD: u32 = 0x003f_3670;
const CIPHER_DECRYPT_HEAD: u32 = 0x003f_37c0;
const NET_RPC_SHORT_FRAME: u32 = 0x30;
const NET_RPC_SHORT_SAVED_RA: u32 = 0x20;
const NET_RPC_REQUEST: u32 = 0x10;
const RPC_HAS_REPLY: u32 = 0;
const RPC_REPLY_OUT: u32 = 8;
const RPC_WAIT: u64 = 1;

fn net_conn_gp(c: &Cpu, offset: u32) -> Addr {
    Addr(r32(c, GP)).off(offset)
}

/// Reads the EE clock against a consistent DirtyRPC epoch and returns milliseconds.
pub fn net_tick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, TICK_FRAME);
        c.g[V0][0] = DIRTY_RPC_STATE_HI;
        frame.save_ra(c, TICK_SAVED_RA);
        c.g[A0][0] = sx(DIRTY_RPC_STATE);
        loop {
            c.cycles += 11;
            let state = Addr(r32(c, A0));
            c.g[V0][0] = state.off(NET_TICK_COUNT).lw(c);
            c.sw(frame.at(c, TICK_COUNT_SAMPLE), c.g[V0][0]);
            c.g[V1][0] = state.off(NET_TICK_EPOCH).ld(c);
            c.sd(frame.at(c, TICK_EPOCH_SAMPLE), c.g[V1][0]);
            c.mfc0(V0, 9);
            c.sw(frame.at(c, TICK_SAMPLE), c.g[V0][0]);
            c.g[V1][0] = c.lw(frame.at(c, TICK_SAMPLE));
            c.g[V1][0] = c.lw(frame.at(c, TICK_COUNT_SAMPLE));
            c.g[V0][0] = state.off(NET_TICK_COUNT).lw(c);
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 4;
                c.g[V1][0] = c.ld(frame.at(c, TICK_EPOCH_SAMPLE));
                c.g[V0][0] = state.off(NET_TICK_EPOCH).ld(c);
                if c.g[V1][0] == c.g[V0][0] {
                    break;
                }
            }
            if event_due(c) {
                deliver_event_here(c, NET_TICK_RETRY);
            }
        }
        c.cycles += 18;
        c.g[V0][0] = c.lw(frame.at(c, TICK_SAMPLE));
        c.g[A1][0] = NET_TICK_MILLISECOND_CYCLES;
        c.g[A0][0] = c.lw(frame.at(c, TICK_COUNT_SAMPLE));
        c.g[V1][0] = c.ld(frame.at(c, TICK_EPOCH_SAMPLE));
        let since_epoch = r32(c, V0).wrapping_sub(r32(c, A0));
        c.g[V0][0] = u64::from(since_epoch);
        c.g[V1][0] = c.g[V1][0].wrapping_add(c.g[V0][0]);
        c.sd(frame.at(c, TICK_ELAPSED), c.g[V1][0]);
        c.g[A0][0] = c.ld(frame.at(c, TICK_ELAPSED));
        UDIVDI3.call(c, 0x003f_4e68)?;
        frame.restore_ra(c, TICK_SAVED_RA);
        c.g[V0][0] = sx(r32(c, V0));
        frame.close(c);
        Ok(())
    });
}

/// Always returns true, preserving the variadic argument spills.
pub fn net_conn_return_true(c: &mut Cpu) {
    const VARIADIC_FRAME: u32 = 0x80;
    const FIRST_GPR_SPILL: u32 = 0x48;
    const FIRST_FPR_SPILL: u32 = 0x38;
    c.cycles += 15;
    let frame = Frame::open(c, VARIADIC_FRAME);
    c.g[V0][0] = 1;
    for (index, reg) in [A1, A2, A3, T0, T1, T2, T3].into_iter().enumerate() {
        c.sd(frame.at(c, FIRST_GPR_SPILL + index as u32 * 8), c.g[reg][0]);
    }
    for (index, reg) in [12, 14, 16, 18].into_iter().enumerate() {
        c.swc1(reg, frame.at(c, FIRST_FPR_SPILL + index as u32 * 4));
    }
    frame.close(c);
}

/// Forwards the second networking timer callback to NetTick.
pub fn net_tick_wrapper2(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(NET_TICK), 0x003f_4340);
}

/// Polls pending RPC replies and invokes registered network idle callbacks.
pub fn net_conn_idle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, NET_CONN_IDLE_FRAME);
        c.g[V0][0] = net_conn_gp(c, NET_CONN_OPEN_GP).lw(c);
        frame.save(c, S0, NET_CONN_IDLE_SAVED_S0);
        frame.save(c, S1, NET_CONN_IDLE_SAVED_S1);
        frame.save_ra(c, NET_CONN_IDLE_SAVED_RA);
        if c.g[V0][0] != 0 {
            c.cycles += 6;
            NET_TICK.call(c, 0x003f_42c0)?;
            c.g[V1][0] = net_conn_gp(c, NET_CONN_NEXT_IDLE_GP).lw(c);
            c.g[V0][0] = u64::from(c.g[V0][0] < c.g[V1][0]);
            let wait = c.g[V0][0] != 0;
            frame.restore(c, S0, NET_CONN_IDLE_SAVED_S0);
            if wait {
                c.cycles += 4;
                frame.restore(c, S1, NET_CONN_IDLE_SAVED_S1);
                frame.restore_ra(c, NET_CONN_IDLE_SAVED_RA);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 4;
            NET_TICK.call(c, 0x003f_42d8)?;
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(NET_CONN_DELAY_MS));
            c.sw(net_conn_gp(c, NET_CONN_NEXT_IDLE_GP).0, c.g[V0][0]);
        }

        c.cycles += 4;
        c.g[S1][0] = NET_CONN_IDLE_CALLBACK_COUNT - 1;
        DIRTY_RPC_PROCESS_REPLIES.call(c, 0x003f_42e8)?;
        c.g[V0][0] = DIRTY_RPC_STATE_HI;
        c.g[S0][0] = sx(NET_CONN_IDLE_CALLBACKS);
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            let active = c.g[V0][0] != 0;
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            if active {
                c.cycles += 6;
                NET_TICK.call(c, 0x003f_4304)?;
                c.g[V1][0] = Addr(r32(c, S0)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                let callback = r32(c, V1);
                c.g[A0][0] = Addr(r32(c, S0)).off(NET_CONN_IDLE_CALLBACK_ARG).lw(c);
                call_ptr(c, callback, 0x003f_4314)?;
            }
            c.cycles += 2;
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(NET_CONN_IDLE_CALLBACK_STRIDE));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, NET_CONN_IDLE_HEAD);
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, NET_CONN_IDLE_SAVED_S0);
        c.cycles += 4;
        frame.restore(c, S1, NET_CONN_IDLE_SAVED_S1);
        frame.restore_ra(c, NET_CONN_IDLE_SAVED_RA);
        frame.close(c);
        Ok(())
    });
}

/// Busy-waits for the requested milliseconds, defaulting to five for zero.
pub fn net_conn_sleep(c: &mut Cpu) {
    guest(c, |c| {
        let frame = match c.pc {
            NET_CONN_SLEEP_ENTRY => {
                c.cycles += 6;
                let frame = Frame::open(c, NET_CONN_SLEEP_FRAME);
                frame.save(c, S0, NET_CONN_SLEEP_SAVED_S0);
                c.g[S0][0] = u64::from(NET_CONN_DELAY_MS);
                if c.g[A0][0] != 0 {
                    c.g[S0][0] = c.g[A0][0];
                }
                let wait = (c.g[S0][0] as i64) > 0;
                frame.save_ra(c, 0);
                if wait {
                    c.cycles += 4;
                    NET_TICK.call(c, 0x003f_41d8)?;
                    c.g[S0][0] = addu(c.g[V0][0], c.g[S0][0]);
                    net_conn_sleep_wait(c, false)?;
                }
                frame
            }
            NET_CONN_SLEEP_HEAD => {
                net_conn_sleep_wait(c, true)?;
                Frame {
                    size: NET_CONN_SLEEP_FRAME,
                }
            }
            pc => {
                c.jump_out(pc, NET_CONN_SLEEP_ENTRY, NET_CONN_SLEEP_END);
                return Ok(());
            }
        };
        c.cycles += 4;
        frame.restore(c, S0, NET_CONN_SLEEP_SAVED_S0);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

fn net_conn_sleep_wait(c: &mut Cpu, mut resumed: bool) -> crate::call::Flow {
    loop {
        if resumed {
            c.cycles += 2;
            NET_TICK.call(c, 0x003f_41e8)?;
        }
        c.cycles += 5;
        NET_TICK.call(c, 0x003f_41f0)?;
        c.g[V0][0] = u64::from(c.g[V0][0] < c.g[S0][0]);
        if c.g[V0][0] == 0 {
            return Ok(());
        }
        if back_edge_unwinds(c, NET_CONN_SLEEP_HEAD) {
            return Err(Unwound);
        }
        resumed = true;
    }
}

/// Forwards the networking timer callback to NetTick.
pub fn net_tick_wrapper(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(NET_TICK), 0x003f_41a8);
}

/// Shuts down the network connection and releases its RPC service.
pub fn net_conn_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, NET_RPC_SHORT_FRAME);
        c.g[V0][0] = net_conn_gp(c, NET_CONN_OPEN_GP).lw(c);
        frame.save_ra(c, NET_RPC_SHORT_SAVED_RA);
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            c.g[V0][0] = u64::MAX;
        } else {
            c.cycles += 23;
            NET_CONN_DISCONNECT.call(c, 0x003f_4130)?;
            net_conn_request(c, frame, NET_RPC_DESTROY, 0x003f_4148)?;
            c.g[V1][0] = RPC_WAIT;
            c.g[A2][0] = net_conn_gp(c, NET_CONN_HANDLE_GP).lw(c);
            net_conn_no_result(c, frame, 0x003f_4178)?;
            net_conn_gp(c, NET_CONN_OPEN_GP).sw(c, 0);
            c.g[V0][0] = 0;
        }
        c.cycles += 3;
        frame.restore_ra(c, NET_RPC_SHORT_SAVED_RA);
        frame.close(c);
        Ok(())
    });
}

/// Requests that the current network connection be disconnected.
pub fn net_conn_disconnect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, NET_RPC_SHORT_FRAME);
        frame.save_ra(c, NET_RPC_SHORT_SAVED_RA);
        net_conn_request(c, frame, NET_RPC_STOP, 0x003f_40d8)?;
        c.g[V1][0] = RPC_WAIT;
        c.g[A2][0] = net_conn_gp(c, NET_CONN_HANDLE_GP).lw(c);
        net_conn_no_result(c, frame, 0x003f_4108)?;
        frame.restore_ra(c, NET_RPC_SHORT_SAVED_RA);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    });
}

fn net_conn_request(c: &mut Cpu, frame: Frame, command: u64, return_pc: u32) -> crate::call::Flow {
    c.g[A1][0] = NET_RPC_SERVICE;
    c.g[A2][0] = command;
    c.g[A0][0] = sx(frame.at(c, NET_RPC_REQUEST));
    DIRTY_RPC_INIT_REQUEST.call(c, return_pc)
}

fn net_conn_no_result(c: &mut Cpu, frame: Frame, return_pc: u32) -> crate::call::Flow {
    c.sw(frame.at(c, RPC_HAS_REPLY), c.g[V1][0]);
    c.g[A0][0] = sx(frame.at(c, NET_RPC_REQUEST));
    c.sw(frame.at(c, RPC_REPLY_OUT), 0);
    c.g[A1][0] = RPC_WAIT;
    for reg in [A3, T0, T1, T2, T3] {
        c.g[reg][0] = 0;
    }
    DIRTY_RPC_CALL.call(c, return_pc)
}

/// Returns the cached hexadecimal address of the network adapter.
pub fn net_conn_mac(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, NET_MAC_FRAME);
        frame.save(c, S2, NET_MAC_SAVED_S2);
        c.g[S2][0] = NET_MAC_CACHE_BASE;
        frame.save(c, S1, NET_MAC_SAVED_S1);
        let cached = Addr(r32(c, S2)).off(NET_MAC_CACHE_OFFSET);
        c.g[V0][0] = cached.lb(c);
        c.g[S1][0] = sx(cached.0);
        frame.save(c, S0, NET_MAC_SAVED_S0);
        frame.save_ra(c, NET_MAC_SAVED_RA);
        if c.g[V0][0] == 0 {
            c.cycles += 64;
            c.g[A0][0] = NET_STATUS_MAC;
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = NET_MAC_REPLY_SIZE;
            NET_CONN_STATUS.call(c, 0x003f_3fb0)?;
            net_conn_mac_format(c, frame);
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, NET_MAC_SAVED_S0);
        frame.restore(c, S1, NET_MAC_SAVED_S1);
        frame.restore(c, S2, NET_MAC_SAVED_S2);
        frame.restore_ra(c, NET_MAC_SAVED_RA);
        frame.close(c);
        Ok(())
    });
}

fn net_conn_mac_format(c: &mut Cpu, frame: Frame) {
    // The conversion leaves the low nibbles in temporaries and the high
    // nibbles in the argument registers after consulting the guest table.
    c.g[V1][0] = c.lbu(frame.at(c, 0));
    c.g[V0][0] = sx(NET_MAC_HEX_BASE);
    c.g[A0][0] = c.lbu(frame.at(c, 1));
    c.g[T1][0] = c.g[V1][0] & NET_MAC_NIBBLE_MASK;
    c.g[A1][0] = c.lbu(frame.at(c, 2));
    c.g[V1][0] = sx(r32(c, V1) >> 4);
    c.g[A2][0] = c.lbu(frame.at(c, 3));
    c.g[V1][0] = addu(c.g[V1][0], c.g[V0][0]);
    c.g[T2][0] = c.g[A0][0] & NET_MAC_NIBBLE_MASK;
    c.g[A3][0] = c.lbu(frame.at(c, 4));
    c.g[T1][0] = addu(c.g[T1][0], c.g[V0][0]);
    c.g[T3][0] = c.g[A1][0] & NET_MAC_NIBBLE_MASK;
    c.g[S0][0] = c.lbu(r32(c, V1));
    c.g[A0][0] = sx(r32(c, A0) >> 4);
    c.g[T0][0] = c.lbu(frame.at(c, 5));
    c.g[A0][0] = addu(c.g[A0][0], c.g[V0][0]);
    c.g[T2][0] = addu(c.g[T2][0], c.g[V0][0]);
    c.g[T4][0] = c.g[A2][0] & NET_MAC_NIBBLE_MASK;
    c.g[V1][0] = c.lbu(r32(c, T1));
    c.g[A1][0] = sx(r32(c, A1) >> 4);
    c.g[T3][0] = addu(c.g[T3][0], c.g[V0][0]);
    c.g[A1][0] = addu(c.g[A1][0], c.g[V0][0]);
    c.g[T5][0] = c.g[A3][0] & NET_MAC_NIBBLE_MASK;
    c.g[T7][0] = c.lbu(r32(c, A0));
    c.g[A2][0] = sx(r32(c, A2) >> 4);
    c.g[T1][0] = c.lbu(r32(c, T2));
    c.g[A2][0] = addu(c.g[A2][0], c.g[V0][0]);
    c.g[T4][0] = addu(c.g[T4][0], c.g[V0][0]);
    c.g[T6][0] = c.g[T0][0] & NET_MAC_NIBBLE_MASK;
    c.g[A0][0] = c.lbu(r32(c, A1));
    c.g[A3][0] = sx(r32(c, A3) >> 4);
    c.g[T2][0] = c.lbu(r32(c, T3));
    c.g[A3][0] = addu(c.g[A3][0], c.g[V0][0]);
    c.g[T5][0] = addu(c.g[T5][0], c.g[V0][0]);
    c.g[A1][0] = c.lbu(r32(c, A2));
    c.g[T3][0] = c.lbu(r32(c, T4));
    c.g[T0][0] = sx(r32(c, T0) >> 4);
    c.g[T6][0] = addu(c.g[T6][0], c.g[V0][0]);
    c.g[T0][0] = addu(c.g[T0][0], c.g[V0][0]);
    c.g[A2][0] = c.lbu(r32(c, A3));
    c.g[V0][0] = NET_MAC_PREFIX;
    c.g[T4][0] = c.lbu(r32(c, T5));
    c.g[A3][0] = c.lbu(r32(c, T0));
    c.g[T5][0] = c.lbu(r32(c, T6));
    c.sb(Addr(r32(c, S2)).off(NET_MAC_CACHE_OFFSET).0, c.g[V0][0]);
    let digits = [S0, V1, T7, T1, A0, T2, A1, T3, A2, T4, A3, T5];
    let cached = Addr(r32(c, S1));
    for (index, reg) in digits.into_iter().enumerate() {
        c.sb(cached.off(index as u32 + 1).0, c.g[reg][0]);
    }
    cached.off(digits.len() as u32 + 1).sb(c, 0);
}

/// Gets the connection state or issues a DirtyRPC status query for the selector.
pub fn net_conn_status(c: &mut Cpu) {
    guest(c, |c| {
        let frame = match c.pc {
            NET_CONN_STATUS_ENTRY => {
                c.cycles += 12;
                let frame = Frame::open(c, NET_CONN_STATUS_FRAME);
                frame.save(c, S0, NET_CONN_STATUS_SAVED_S0);
                c.g[V0][0] = NET_STATUS_OPEN;
                frame.save(c, S2, NET_CONN_STATUS_SAVED_S2);
                c.g[S0][0] = c.g[A0][0];
                frame.save(c, S3, NET_CONN_STATUS_SAVED_S3);
                c.g[S2][0] = c.g[A1][0];
                frame.save(c, S1, NET_CONN_STATUS_SAVED_S1);
                c.g[S3][0] = c.g[A2][0];
                frame.save_ra(c, NET_CONN_STATUS_SAVED_RA);
                net_conn_status_query(c, frame)?;
                frame
            }
            NET_CONN_STATUS_BROADBAND_HEAD => {
                let frame = Frame {
                    size: NET_CONN_STATUS_FRAME,
                };
                net_conn_status_reply(c, frame);
                frame
            }
            pc => {
                c.jump_out(pc, NET_CONN_STATUS_ENTRY, NET_CONN_STATUS_END);
                return Ok(());
            }
        };
        c.cycles += 7;
        frame.restore(c, S0, NET_CONN_STATUS_SAVED_S0);
        frame.restore(c, S1, NET_CONN_STATUS_SAVED_S1);
        frame.restore(c, S2, NET_CONN_STATUS_SAVED_S2);
        frame.restore(c, S3, NET_CONN_STATUS_SAVED_S3);
        frame.restore_ra(c, NET_CONN_STATUS_SAVED_RA);
        frame.close(c);
        Ok(())
    });
}

fn net_conn_status_reply(c: &mut Cpu, frame: Frame) {
    c.cycles += 2;
    c.g[V0][0] = c.lw(frame.at(c, NET_RPC_REPLY_WORD));
}

fn net_conn_status_query(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    if c.g[S0][0] == c.g[V0][0] {
        c.cycles += 2;
        c.g[V0][0] = net_conn_gp(c, NET_CONN_OPEN_GP).lw(c);
        return Ok(());
    }

    c.cycles += 4;
    c.g[V0][0] = NET_STATUS_ONLINE;
    if c.g[S0][0] == c.g[V0][0] {
        c.cycles += 23;
        net_conn_request(c, frame, NET_RPC_STAT, 0x003f_3d20)?;
        net_conn_status_word_call(c, frame, NET_RPC_REPLY_WORD, NET_STATUS_HOST);
        net_conn_status_clear_buffers(c);
        DIRTY_RPC_CALL.call(c, 0x003f_3d58)?;
        c.g[V0][0] = c.lw(frame.at(c, NET_RPC_REPLY_WORD));
        c.g[V0][0] = u64::from(c.g[V0][0] != 0);
        return Ok(());
    }

    c.cycles += 3;
    c.g[V0][0] = NET_STATUS_CONNECTION;
    if c.g[S0][0] == c.g[V0][0] {
        c.cycles += 20;
        net_conn_request(c, frame, NET_RPC_STAT, 0x003f_3d88)?;
        c.g[V1][0] = sx(frame.at(c, NET_RPC_REPLY_WORD));
        c.g[V0][0] = RPC_WAIT;
        c.g[A2][0] = net_conn_gp(c, NET_CONN_HANDLE_GP).lw(c);
        c.g[A3][0] = NET_STATUS_CONNECTION & SELECTOR_PREFIX_MASK;
        c.sw(frame.at(c, RPC_HAS_REPLY), c.g[V0][0]);
        c.g[T2][0] = c.g[S2][0];
        c.sw(frame.at(c, RPC_REPLY_OUT), c.g[V1][0]);
        c.g[T3][0] = c.g[S3][0];
        c.g[A0][0] = sx(frame.at(c, NET_RPC_REQUEST));
        c.g[A1][0] = RPC_WAIT_WITH_OUTPUT;
        c.g[A3][0] = NET_STATUS_CONNECTION;
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        DIRTY_RPC_CALL.call(c, 0x003f_3dc0)?;
        net_conn_status_reply(c, frame);
        return Ok(());
    }

    c.cycles += 3;
    let host = c.g[S0][0] == NET_STATUS_HOST;
    c.g[V0][0] = NET_STATUS_ADDRESS & SELECTOR_PREFIX_MASK;
    if host {
        c.cycles += 16;
        net_conn_request(c, frame, NET_RPC_STAT, 0x003f_3dec)?;
        net_conn_status_word_call(c, frame, NET_RPC_STATUS_WORD, NET_STATUS_HOST);
        return net_conn_status_host_or_address(c, frame);
    }

    c.cycles += 3;
    c.g[V0][0] = NET_STATUS_ADDRESS;
    if c.g[S0][0] == c.g[V0][0] {
        c.cycles += 15;
        net_conn_request(c, frame, NET_RPC_STAT, 0x003f_3e38)?;
        net_conn_status_word_call(c, frame, NET_RPC_STATUS_WORD, NET_STATUS_ADDRESS);
        return net_conn_status_host_or_address(c, frame);
    }

    c.cycles += 3;
    c.g[V0][0] = NET_STATUS_MAC;
    let mac = c.g[S0][0] == c.g[V0][0];
    c.g[S1][0] = NET_STATUS_BROADBAND & SELECTOR_PREFIX_MASK;
    if mac {
        c.cycles += 21;
        net_conn_request(c, frame, NET_RPC_STAT, 0x003f_3e9c)?;
        c.g[V1][0] = RPC_WAIT;
        c.g[A2][0] = net_conn_gp(c, NET_CONN_HANDLE_GP).lw(c);
        c.g[A3][0] = NET_STATUS_MAC & SELECTOR_PREFIX_MASK;
        c.sw(frame.at(c, RPC_HAS_REPLY), c.g[V1][0]);
        c.g[T2][0] = c.g[S2][0];
        c.g[T3][0] = c.g[S3][0];
        c.sw(frame.at(c, RPC_REPLY_OUT), 0);
        c.g[A0][0] = sx(frame.at(c, NET_RPC_REQUEST));
        c.g[A1][0] = RPC_WAIT_WITH_OUTPUT;
        c.g[A3][0] = NET_STATUS_MAC;
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        DIRTY_RPC_CALL.call(c, 0x003f_3ed0)?;
        c.g[V0][0] = 0;
        return Ok(());
    }

    c.cycles += 3;
    c.g[S1][0] = NET_STATUS_BROADBAND;
    c.g[V0][0] = NET_STATUS_TYPE & SELECTOR_PREFIX_MASK;
    if c.g[S0][0] != c.g[S1][0] {
        c.cycles += 3;
        c.g[V0][0] = u64::MAX;
        if c.g[S0][0] != NET_STATUS_TYPE {
            return Ok(());
        }
    }
    c.cycles += 22;
    net_conn_request(c, frame, NET_RPC_STAT, 0x003f_3f08)?;
    c.g[V1][0] = RPC_WAIT;
    c.g[V0][0] = sx(frame.at(c, NET_RPC_REPLY_WORD));
    c.g[A3][0] = NET_STATUS_TYPE & SELECTOR_PREFIX_MASK;
    c.g[A2][0] = net_conn_gp(c, NET_CONN_HANDLE_GP).lw(c);
    c.sw(frame.at(c, RPC_HAS_REPLY), c.g[V1][0]);
    c.g[T2][0] = c.g[S2][0];
    c.sw(frame.at(c, RPC_REPLY_OUT), c.g[V0][0]);
    c.g[T3][0] = c.g[S3][0];
    c.g[A0][0] = sx(frame.at(c, NET_RPC_REQUEST));
    c.g[A1][0] = RPC_WAIT;
    c.g[A3][0] = NET_STATUS_TYPE;
    c.g[T0][0] = 0;
    c.g[T1][0] = 0;
    DIRTY_RPC_CALL.call(c, 0x003f_3f40)?;
    let broadband = c.g[S0][0] == c.g[S1][0];
    c.g[V0][0] = c.lw(frame.at(c, NET_RPC_REPLY_WORD));
    if broadband {
        c.cycles += 4;
        c.g[V0][0] = u64::from((r32(c, V0) >> NET_STATUS_BROADBAND_BIT) & 1);
        c.sw(frame.at(c, NET_RPC_REPLY_WORD), c.g[V0][0]);
        if back_edge_unwinds(c, NET_CONN_STATUS_BROADBAND_HEAD) {
            return Err(Unwound);
        }
        net_conn_status_reply(c, frame);
    }
    Ok(())
}

fn net_conn_status_word_call(c: &mut Cpu, frame: Frame, reply_offset: u32, selector: u64) {
    c.g[V1][0] = sx(frame.at(c, reply_offset));
    c.g[V0][0] = RPC_WAIT;
    c.g[A2][0] = net_conn_gp(c, NET_CONN_HANDLE_GP).lw(c);
    c.g[A3][0] = selector & SELECTOR_PREFIX_MASK;
    c.sw(frame.at(c, RPC_HAS_REPLY), c.g[V0][0]);
    c.g[A0][0] = sx(frame.at(c, NET_RPC_REQUEST));
    c.sw(frame.at(c, RPC_REPLY_OUT), c.g[V1][0]);
    c.g[A1][0] = RPC_WAIT;
    c.g[A3][0] = selector;
}

fn net_conn_status_clear_buffers(c: &mut Cpu) {
    for reg in [T0, T1, T2, T3] {
        c.g[reg][0] = 0;
    }
}

fn net_conn_status_host_or_address(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 7;
    net_conn_status_clear_buffers(c);
    DIRTY_RPC_CALL.call(c, 0x003f_3e70)?;
    c.g[V0][0] = c.lw(frame.at(c, NET_RPC_STATUS_WORD));
    Ok(())
}

/// Passes a connection string to the network RPC start command.
pub fn net_conn_connect(c: &mut Cpu) {
    const CONNECT_FRAME: u32 = 0x50;
    const CONNECT_SAVED_S0: u32 = 0x40;
    const CONNECT_SAVED_RA: u32 = 0x30;
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, CONNECT_FRAME);
        frame.save(c, S0, CONNECT_SAVED_S0);
        frame.save_ra(c, CONNECT_SAVED_RA);
        c.g[S0][0] = c.g[A0][0];
        net_conn_request(c, frame, NET_RPC_START, 0x003f_3c78)?;
        c.g[V1][0] = sx(frame.at(c, NET_RPC_REPLY_WORD));
        c.g[V0][0] = RPC_WAIT;
        c.g[A2][0] = net_conn_gp(c, NET_CONN_HANDLE_GP).lw(c);
        c.g[T0][0] = c.g[S0][0];
        c.sw(frame.at(c, RPC_HAS_REPLY), c.g[V0][0]);
        c.g[A0][0] = sx(frame.at(c, NET_RPC_REQUEST));
        c.sw(frame.at(c, RPC_REPLY_OUT), c.g[V1][0]);
        c.g[A1][0] = RPC_WAIT;
        c.g[A3][0] = 0;
        c.g[T1][0] = RPC_NUL_TERMINATED;
        c.g[T2][0] = 0;
        c.g[T3][0] = 0;
        DIRTY_RPC_CALL.call(c, 0x003f_3cac)?;
        c.g[V0][0] = c.lw(frame.at(c, NET_RPC_REPLY_WORD));
        frame.restore(c, S0, CONNECT_SAVED_S0);
        frame.restore_ra(c, CONNECT_SAVED_RA);
        frame.close(c);
        Ok(())
    });
}

/// Requests the network connection list into the caller's entry buffer.
pub fn net_conn_query(c: &mut Cpu) {
    const QUERY_FRAME: u32 = 0x70;
    const QUERY_SAVED_S0: u32 = 0x60;
    const QUERY_SAVED_S1: u32 = 0x50;
    const QUERY_SAVED_S2: u32 = 0x40;
    const QUERY_SAVED_RA: u32 = 0x30;
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, QUERY_FRAME);
        frame.save(c, S1, QUERY_SAVED_S1);
        frame.save(c, S2, QUERY_SAVED_S2);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, QUERY_SAVED_S0);
        c.g[S2][0] = c.g[A2][0];
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, QUERY_SAVED_RA);
        net_conn_request(c, frame, NET_RPC_LIST, 0x003f_3c00)?;
        c.g[V1][0] = sx(frame.at(c, NET_RPC_REPLY_WORD));
        c.g[V0][0] = RPC_WAIT;
        c.g[A2][0] = net_conn_gp(c, NET_CONN_HANDLE_GP).lw(c);
        c.g[T0][0] = c.g[S0][0];
        c.g[T2][0] = c.g[S1][0];
        c.g[T3][0] = sx(r32(c, S2) << RPC_QUERY_ENTRY_SHIFT);
        c.sw(frame.at(c, RPC_HAS_REPLY), c.g[V0][0]);
        c.g[A0][0] = sx(frame.at(c, NET_RPC_REQUEST));
        c.sw(frame.at(c, RPC_REPLY_OUT), c.g[V1][0]);
        c.g[A1][0] = RPC_WAIT_WITH_OUTPUT;
        c.g[A3][0] = 0;
        c.g[T1][0] = RPC_NUL_TERMINATED;
        DIRTY_RPC_CALL.call(c, 0x003f_3c34)?;
        c.g[V0][0] = c.lw(frame.at(c, NET_RPC_REPLY_WORD));
        frame.restore(c, S0, QUERY_SAVED_S0);
        frame.restore(c, S1, QUERY_SAVED_S1);
        frame.restore(c, S2, QUERY_SAVED_S2);
        frame.restore_ra(c, QUERY_SAVED_RA);
        frame.close(c);
        Ok(())
    });
}

/// Starts DirtyRPC if needed and opens the network service.
pub fn net_conn_startup(c: &mut Cpu) {
    const STARTUP_FRAME: u32 = 0x50;
    const STARTUP_SAVED_S0: u32 = 0x40;
    const STARTUP_SAVED_S1: u32 = 0x30;
    const STARTUP_SAVED_RA: u32 = 0x20;
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, STARTUP_FRAME);
        c.g[V0][0] = net_conn_gp(c, NET_CONN_OPEN_GP).lw(c);
        frame.save(c, S1, STARTUP_SAVED_S1);
        frame.save(c, S0, STARTUP_SAVED_S0);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, STARTUP_SAVED_RA);
        if c.g[V0][0] == 0 {
            c.cycles += 4;
            DIRTY_RPC_IS_STARTED.call(c, 0x003f_3b44)?;
            let started = (c.g[V0][0] as i64) > 0;
            c.g[A1][0] = NET_RPC_SERVICE & SELECTOR_PREFIX_MASK;
            if !started {
                c.cycles += 5;
                c.g[A0][0] = 0;
                DIRTY_RPC_STARTUP.call(c, 0x003f_3b54)?;
                DIRTY_RPC_INIT_IOP.call(c, 0x003f_3b5c)?;
                c.g[A1][0] = NET_RPC_SERVICE & SELECTOR_PREFIX_MASK;
            }
            c.cycles += 21;
            c.g[S0][0] = RPC_WAIT;
            net_conn_request(c, frame, NET_RPC_CREATE, 0x003f_3b78)?;
            c.g[V1][0] = sx(net_conn_gp(c, NET_CONN_HANDLE_GP).0);
            c.g[A3][0] = NET_RPC_CLIENT_ID & SELECTOR_PREFIX_MASK;
            c.sw(frame.at(c, RPC_REPLY_OUT), c.g[V1][0]);
            c.g[T0][0] = c.g[S1][0];
            c.sw(frame.at(c, RPC_HAS_REPLY), c.g[S0][0]);
            c.g[A0][0] = sx(frame.at(c, NET_RPC_REQUEST));
            c.g[A1][0] = RPC_WAIT;
            c.g[A2][0] = 0;
            c.g[A3][0] = NET_RPC_CLIENT_ID;
            c.g[T1][0] = RPC_NUL_TERMINATED;
            c.g[T2][0] = 0;
            c.g[T3][0] = 0;
            DIRTY_RPC_CALL.call(c, 0x003f_3bac)?;
            c.sw(net_conn_gp(c, NET_CONN_OPEN_GP).0, c.g[S0][0]);
            c.g[V0][0] = 0;
        } else {
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
        }
        c.cycles += 5;
        frame.restore(c, S0, STARTUP_SAVED_S0);
        frame.restore(c, S1, STARTUP_SAVED_S1);
        frame.restore_ra(c, STARTUP_SAVED_RA);
        frame.close(c);
        Ok(())
    });
}

/// Looks up a hostname and releases the request after completion.
pub fn socket_lookup_sync(c: &mut Cpu) {
    const SYNC_FRAME: u32 = 0x30;
    const SYNC_SAVED_S0: u32 = 0x20;
    const SYNC_SAVED_S1: u32 = 0x10;
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, SYNC_FRAME);
        frame.save(c, S0, SYNC_SAVED_S0);
        frame.save(c, S1, SYNC_SAVED_S1);
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        SOCKET_LOOKUP_CREATE.call(c, 0x003f_3ad0)?;
        c.g[S0][0] = c.g[V0][0];
        let allocated = c.g[S0][0] != 0;
        c.g[V0][0] = c.g[S1][0];
        if allocated {
            c.cycles += 1;
            c.g[V0][0] = Addr(r32(c, S0)).off(LOOKUP_DONE_CALLBACK).lw(c);
            loop {
                c.cycles += 4;
                let poll = r32(c, V0);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, poll, 0x003f_3ae8)?;
                if c.g[V0][0] != 0 {
                    break;
                }
                c.g[V0][0] = Addr(r32(c, S0)).off(LOOKUP_DONE_CALLBACK).lw(c);
                if event_due(c) {
                    deliver_event_here(c, LOOKUP_SYNC_HEAD);
                }
            }
            c.cycles += 5;
            c.g[V0][0] = Addr(r32(c, S0)).off(LOOKUP_FREE_CALLBACK).lw(c);
            c.g[A0][0] = c.g[S0][0];
            let free = r32(c, V0);
            c.g[S1][0] = Addr(r32(c, S0)).off(LOOKUP_ADDRESS).lw(c);
            call_ptr(c, free, 0x003f_3b00)?;
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, SYNC_SAVED_S0);
        frame.restore(c, S1, SYNC_SAVED_S1);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Starts an asynchronous hostname lookup.
pub fn socket_lookup(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(SOCKET_LOOKUP_CREATE), 0x003f_3aa8);
}

/// Allocates a hostname lookup and submits its name-service request.
pub fn socket_lookup_create(c: &mut Cpu) {
    const CREATE_FRAME: u32 = 0x60;
    const CREATE_SAVED_S0: u32 = 0x50;
    const CREATE_SAVED_S1: u32 = 0x40;
    const CREATE_SAVED_S2: u32 = 0x30;
    const CREATE_SAVED_S3: u32 = 0x20;
    const CREATE_SAVED_RA: u32 = 0x10;
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, CREATE_FRAME);
        frame.save(c, S3, CREATE_SAVED_S3);
        frame.save(c, S2, CREATE_SAVED_S2);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, CREATE_SAVED_S1);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, CREATE_SAVED_S0);
        frame.save_ra(c, CREATE_SAVED_RA);
        c.g[A0][0] = u64::from(LOOKUP_SIZE);
        NET_RPC_ALLOC.call(c, 0x003f_39f0)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        if c.g[S1][0] != 0 {
            c.cycles += 30;
            c.g[A1][0] = 0;
            c.g[A2][0] = u64::from(LOOKUP_SIZE);
            MEMSET.call(c, 0x003f_3a08)?;
            c.g[V1][0] = sx(LOOKUP_DONE_ADDR);
            c.g[S0][0] = sx(LOOKUP_RPC_CREATE_DESC);
            c.g[V0][0] = sx(LOOKUP_FREE_ADDR);
            c.g[A1][0] = LOOKUP_RPC_SERVICE & SELECTOR_PREFIX_MASK;
            c.g[A2][0] = LOOKUP_RPC_CREATE & SELECTOR_PREFIX_MASK;
            c.sw(Addr(r32(c, S1)).off(LOOKUP_DONE_CALLBACK).0, c.g[V1][0]);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = LOOKUP_RPC_SERVICE;
            c.g[A2][0] = LOOKUP_RPC_CREATE;
            c.sw(Addr(r32(c, S1)).off(LOOKUP_FREE_CALLBACK).0, c.g[V0][0]);
            DIRTY_RPC_INIT_REQUEST.call(c, 0x003f_3a40)?;
            c.g[V1][0] = RPC_WAIT;
            c.g[V0][0] = sx(Addr(r32(c, S1)).off(LOOKUP_REMOTE_HANDLE).0);
            c.sw(frame.at(c, RPC_HAS_REPLY), c.g[V1][0]);
            c.g[A0][0] = c.g[S0][0];
            c.sw(frame.at(c, RPC_REPLY_OUT), c.g[V0][0]);
            c.g[A3][0] = c.g[S2][0];
            c.g[T0][0] = c.g[S3][0];
            c.g[A1][0] = RPC_WAIT;
            c.g[A2][0] = 0;
            c.g[T1][0] = RPC_NUL_TERMINATED;
            c.g[T2][0] = 0;
            c.g[T3][0] = 0;
            DIRTY_RPC_CALL.call(c, 0x003f_3a74)?;
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, CREATE_SAVED_S0);
        frame.restore(c, S1, CREATE_SAVED_S1);
        frame.restore(c, S2, CREATE_SAVED_S2);
        frame.restore(c, S3, CREATE_SAVED_S3);
        frame.restore_ra(c, CREATE_SAVED_RA);
        frame.close(c);
        Ok(())
    });
}

/// Releases the remote lookup handle and frees the local request.
pub fn socket_lookup_free(c: &mut Cpu) {
    const FREE_FRAME: u32 = 0x50;
    const FREE_SAVED_S0: u32 = 0x40;
    const FREE_SAVED_S1: u32 = 0x30;
    const FREE_SAVED_S2: u32 = 0x20;
    const FREE_SAVED_RA: u32 = 0x10;
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, FREE_FRAME);
        c.g[A1][0] = LOOKUP_RPC_SERVICE & SELECTOR_PREFIX_MASK;
        frame.save(c, S0, FREE_SAVED_S0);
        c.g[A2][0] = LOOKUP_RPC_FREE & SELECTOR_PREFIX_MASK;
        frame.save(c, S2, FREE_SAVED_S2);
        c.g[S0][0] = DIRTY_RPC_STATE_HI;
        frame.save_ra(c, FREE_SAVED_RA);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, FREE_SAVED_S1);
        c.g[S0][0] = sx(LOOKUP_RPC_FREE_DESC);
        c.g[S1][0] = Addr(r32(c, S2)).off(LOOKUP_REMOTE_HANDLE).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = LOOKUP_RPC_SERVICE;
        c.g[A2][0] = LOOKUP_RPC_FREE;
        DIRTY_RPC_INIT_REQUEST.call(c, 0x003f_397c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        net_conn_status_clear_buffers(c);
        c.sw(frame.at(c, RPC_HAS_REPLY), 0);
        c.sw(frame.at(c, RPC_REPLY_OUT), 0);
        c.g[A1][0] = RPC_WAIT;
        DIRTY_RPC_CALL.call(c, 0x003f_39a8)?;
        c.g[A0][0] = c.g[S2][0];
        NET_RPC_FREE.call(c, 0x003f_39b0)?;
        frame.restore(c, S0, FREE_SAVED_S0);
        frame.restore(c, S1, FREE_SAVED_S1);
        frame.restore(c, S2, FREE_SAVED_S2);
        frame.restore_ra(c, FREE_SAVED_RA);
        frame.close(c);
        Ok(())
    });
}

/// Polls the name service and reads the address of a successful lookup.
pub fn socket_lookup_done(c: &mut Cpu) {
    const DONE_FRAME: u32 = 0x60;
    const DONE_SAVED_S0: u32 = 0x50;
    const DONE_SAVED_S1: u32 = 0x40;
    const DONE_SAVED_S2: u32 = 0x30;
    const DONE_SAVED_S3: u32 = 0x20;
    const DONE_SAVED_RA: u32 = 0x10;
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, DONE_FRAME);
        c.g[V0][0] = DIRTY_RPC_STATE_HI;
        frame.save(c, S0, DONE_SAVED_S0);
        c.g[A1][0] = LOOKUP_RPC_SERVICE & SELECTOR_PREFIX_MASK;
        frame.save(c, S1, DONE_SAVED_S1);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S3, DONE_SAVED_S3);
        c.g[S1][0] = sx(LOOKUP_RPC_DONE_DESC);
        frame.save_ra(c, DONE_SAVED_RA);
        c.g[A2][0] = LOOKUP_RPC_DONE & SELECTOR_PREFIX_MASK;
        frame.save(c, S2, DONE_SAVED_S2);
        c.g[A0][0] = c.g[S1][0];
        c.g[S2][0] = Addr(r32(c, S0)).off(LOOKUP_REMOTE_HANDLE).lw(c);
        c.g[A1][0] = LOOKUP_RPC_SERVICE;
        c.g[A2][0] = LOOKUP_RPC_DONE;
        c.g[S3][0] = RPC_WAIT;
        DIRTY_RPC_INIT_REQUEST.call(c, 0x003f_38a4)?;
        c.sw(frame.at(c, RPC_HAS_REPLY), c.g[S3][0]);
        c.g[A0][0] = c.g[S1][0];
        c.sw(frame.at(c, RPC_REPLY_OUT), c.g[S0][0]);
        c.g[A1][0] = RPC_WAIT;
        c.g[A2][0] = c.g[S2][0];
        c.g[A3][0] = 0;
        net_conn_status_clear_buffers(c);
        DIRTY_RPC_CALL.call(c, 0x003f_38d0)?;
        c.g[V1][0] = Addr(r32(c, S0)).off(LOOKUP_STATUS).lw(c);
        c.g[A1][0] = LOOKUP_RPC_SERVICE & SELECTOR_PREFIX_MASK;
        if (c.g[V1][0] as i64) > 0 {
            c.cycles += 17;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = LOOKUP_RPC_SERVICE;
            c.g[A2][0] = LOOKUP_RPC_ADDRESS;
            DIRTY_RPC_INIT_REQUEST.call(c, 0x003f_38f0)?;
            c.g[V1][0] = sx(Addr(r32(c, S0)).off(LOOKUP_ADDRESS).0);
            c.sw(frame.at(c, RPC_HAS_REPLY), c.g[S3][0]);
            c.sw(frame.at(c, RPC_REPLY_OUT), c.g[V1][0]);
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[A1][0] = RPC_WAIT;
            c.g[A3][0] = 0;
            net_conn_status_clear_buffers(c);
            DIRTY_RPC_CALL.call(c, 0x003f_3920)?;
        }
        c.cycles += 8;
        c.g[V0][0] = Addr(r32(c, S0)).off(LOOKUP_STATUS).lw(c);
        frame.restore(c, S0, DONE_SAVED_S0);
        frame.restore(c, S1, DONE_SAVED_S1);
        frame.restore(c, S2, DONE_SAVED_S2);
        frame.restore(c, S3, DONE_SAVED_S3);
        frame.restore_ra(c, DONE_SAVED_RA);
        frame.close(c);
        Ok(())
    });
}

/// Decodes a lobby string up to its terminator or the destination limit.
pub fn lobby_decrypt_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, CIPHER_STRING_FRAME);
        frame.save(c, S6, CIPHER_STRING_S6);
        frame.save(c, S3, CIPHER_STRING_S3);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, S4, CIPHER_STRING_S4);
        c.g[A1][0] = c.g[A3][0];
        c.g[S3][0] = c.g[A0][0];
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, S1, CIPHER_STRING_S1);
        c.g[A2][0] = c.g[T0][0];
        frame.save(c, S0, CIPHER_STRING_S0);
        c.g[A3][0] = c.g[T1][0];
        frame.save(c, S2, CIPHER_STRING_S2);
        c.g[A0][0] = c.g[SP][0];
        frame.save(c, S5, CIPHER_STRING_S5);
        c.g[S1][0] = 0;
        frame.save(c, S7, CIPHER_STRING_S7);
        frame.save_ra(c, CIPHER_STRING_RA);
        c.sb(frame.at(c, CIPHER_STRING_BYTE), 0);
        LOBBY_CIPHER_INIT.call(c, 0x003f_3798)?;

        c.g[V0][0] = sx(r32(c, S6).wrapping_sub(1));
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 3;
            c.g[S0][0] = c.lbu(r32(c, S4));
            let empty = c.g[S0][0] == 0;
            c.g[S1][0] = 1;
            if !empty {
                c.cycles += 4;
                c.g[S5][0] = c.g[V0][0];
                c.g[S2][0] = CIPHER_STRING_ALPHABET;
                c.g[S7][0] = CIPHER_STRING_TERMINATOR;
                loop {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx(frame.at(c, CIPHER_STRING_BYTE));
                    c.g[A2][0] = 1;
                    LOBBY_CIPHER_APPLY.call(c, 0x003f_37d0)?;
                    c.g[V1][0] = c.lbu(frame.at(c, CIPHER_STRING_BYTE));
                    c.cycles += 11;
                    c.divu(0, V1, S2);
                    c.g[V0][0] = c.hi[0];
                    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(CIPHER_STRING_BIAS));
                    c.g[V0][0] = sx(r32(c, S0).wrapping_sub(r32(c, V0)));
                    c.div(0, V0, S2);
                    c.g[V1][0] = c.hi[0];
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(CIPHER_FIRST_PRINTABLE));
                    c.g[V0][0] = c.g[V1][0] as u8 as i8 as i64 as u64;
                    let terminator = c.g[V0][0] == c.g[S7][0];
                    c.sb(r32(c, S3), c.g[V1][0]);
                    if terminator {
                        break;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = u64::from((c.g[S1][0] as i64) < (c.g[S5][0] as i64));
                    let room = c.g[V0][0] != 0;
                    c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
                    if !room {
                        break;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = addu(c.g[S4][0], c.g[S1][0]);
                    c.g[S0][0] = c.lbu(r32(c, V0));
                    let more = c.g[S0][0] != 0;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, CIPHER_DECRYPT_HEAD);
                    }
                }
            }
            c.cycles += 1;
        }
        c.g[V0][0] = u64::from((c.g[S1][0] as i64) < (c.g[S6][0] as i64));
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.sb(r32(c, S3), 0);
        }
        lobby_cipher_string_restore(c, frame);
        Ok(())
    });
}

/// Encodes a lobby string, padding the fixed-width destination after the input ends.
pub fn lobby_encrypt_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, CIPHER_STRING_FRAME);
        frame.save(c, S0, CIPHER_STRING_S0);
        frame.save(c, S1, CIPHER_STRING_S1);
        c.g[S0][0] = CIPHER_PADDING_BANK;
        frame.save(c, S2, CIPHER_STRING_S2);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S3, CIPHER_STRING_S3);
        c.g[S2][0] = c.g[T0][0];
        frame.save(c, S4, CIPHER_STRING_S4);
        c.g[S3][0] = c.g[T1][0];
        frame.save(c, S5, CIPHER_STRING_S5);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, S6, CIPHER_STRING_S6);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S7, CIPHER_STRING_S7);
        c.g[S6][0] = c.g[A0][0];
        c.g[A0][0] = c.g[SP][0];
        frame.save_ra(c, CIPHER_STRING_RA);
        c.sb(frame.at(c, CIPHER_STRING_STREAM_BYTE), 0);
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A3][0] = c.g[S3][0];
        c.g[S7][0] = c.g[S0][0];
        LOBBY_CIPHER_INIT.call(c, 0x003f_3630)?;
        c.g[S0][0] =
            sx(r32(c, S0).wrapping_sub((CIPHER_PADDING_BANK as u32) - CIPHER_PADDING_STATE));
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A3][0] = sx(0u32.wrapping_sub(r32(c, S3)));
        c.g[A0][0] = c.g[S0][0];
        LOBBY_CIPHER_INIT.call(c, 0x003f_3648)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(CIPHER_PADDING_KEY);
        c.g[A2][0] = u64::MAX;
        c.g[A3][0] = u64::MAX;
        LOBBY_CIPHER_INIT.call(c, 0x003f_3660)?;
        c.g[V0][0] = u64::from((c.g[S5][0] as i64) < 2);
        c.g[S1][0] = CIPHER_STRING_TERMINATOR;
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            c.g[S0][0] = CIPHER_STRING_ALPHABET;
            lobby_encrypt_string_bytes(c, frame)?;
        }
        c.cycles += 2;
        if (c.g[S5][0] as i64) > 0 {
            c.sb(r32(c, S6), 0);
        }
        lobby_cipher_string_restore(c, frame);
        Ok(())
    });
}

fn lobby_encrypt_string_bytes(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    loop {
        c.cycles += 2;
        if c.g[S4][0] == 0 {
            c.cycles += 9;
            c.g[A0][0] =
                sx(r32(c, S7).wrapping_sub((CIPHER_PADDING_BANK as u32) - CIPHER_PADDING_STATE));
            c.g[A1][0] = sx(frame.at(c, CIPHER_STRING_BYTE));
            c.g[A2][0] = 1;
            LOBBY_CIPHER_APPLY.call(c, 0x003f_3688)?;
            c.g[V0][0] = c.lbu(frame.at(c, CIPHER_STRING_BYTE));
            c.g[V0][0] &= CIPHER_PADDING_MASK;
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(CIPHER_FIRST_PRINTABLE));
            c.sb(frame.at(c, CIPHER_STRING_BYTE), c.g[V0][0]);
        } else {
            c.g[V0][0] = c.lbu(r32(c, S4));
            c.cycles += 4;
            c.g[S4][0] = sx(r32(c, S4).wrapping_add(1));
            c.g[V1][0] = c.g[V0][0];
            c.sb(frame.at(c, CIPHER_STRING_BYTE), c.g[V0][0]);
            if c.g[V1][0] == 0 {
                c.g[S4][0] = 0;
            }
        }
        c.cycles += 5;
        c.g[V0][0] = c.lbu(frame.at(c, CIPHER_STRING_BYTE));
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(CIPHER_FIRST_PRINTABLE));
        c.g[V0][0] = u64::from(c.g[V0][0] < CIPHER_PRINTABLE_COUNT);
        if c.g[V0][0] == 0 {
            c.sb(frame.at(c, CIPHER_STRING_BYTE), c.g[S1][0]);
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = sx(frame.at(c, CIPHER_STRING_STREAM_BYTE));
        c.g[A2][0] = 1;
        c.g[S5][0] = sx(r32(c, S5).wrapping_sub(1));
        LOBBY_CIPHER_APPLY.call(c, 0x003f_36d4)?;
        c.g[V0][0] = c.lbu(frame.at(c, CIPHER_STRING_STREAM_BYTE));
        c.cycles += 12;
        c.g[A0][0] = c.lbu(frame.at(c, CIPHER_STRING_BYTE));
        c.divu(0, V0, S0);
        c.g[A1][0] = u64::from((c.g[S5][0] as i64) < 2);
        c.g[V1][0] = c.hi[0];
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(CIPHER_STRING_BIAS));
        c.g[A0][0] = addu(c.g[A0][0], c.g[V1][0]);
        c.div(0, A0, S0);
        c.g[V0][0] = c.hi[0];
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(CIPHER_FIRST_PRINTABLE));
        c.sb(r32(c, S6), c.g[V0][0]);
        let more = c.g[A1][0] == 0;
        c.g[S6][0] = sx(r32(c, S6).wrapping_add(1));
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, CIPHER_ENCRYPT_HEAD);
        }
    }
}

fn lobby_cipher_string_restore(c: &mut Cpu, frame: Frame) {
    c.cycles += 12;
    frame.restore(c, S0, CIPHER_STRING_S0);
    c.g[V0][0] = 0;
    frame.restore(c, S1, CIPHER_STRING_S1);
    frame.restore(c, S2, CIPHER_STRING_S2);
    frame.restore(c, S3, CIPHER_STRING_S3);
    frame.restore(c, S4, CIPHER_STRING_S4);
    frame.restore(c, S5, CIPHER_STRING_S5);
    frame.restore(c, S6, CIPHER_STRING_S6);
    frame.restore(c, S7, CIPHER_STRING_S7);
    frame.restore_ra(c, CIPHER_STRING_RA);
    frame.close(c);
}

/// Replacements for the lobby networking and string cipher functions.
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003f_4de8, net_tick, "NetTick"),
    (0x003f_43d0, net_conn_return_true, "NetConn_ReturnTrue"),
    (0x003f_4330, net_tick_wrapper2, "NetTick_Wrapper2"),
    (0x003f_42a0, net_conn_idle, "NetConnIdle"),
    (0x003f_41b8, net_conn_sleep, "NetConnSleep"),
    (0x003f_4198, net_tick_wrapper, "NetTick_Wrapper"),
    (0x003f_4118, net_conn_shutdown, "NetConnShutdown"),
    (0x003f_40b8, net_conn_disconnect, "NetConnDisconnect"),
    (0x003f_3f78, net_conn_mac, "NetConnMAC"),
    (0x003f_3cc0, net_conn_status, "NetConnStatus"),
    (0x003f_3c50, net_conn_connect, "NetConnConnect"),
    (0x003f_3bc8, net_conn_query, "NetConnQuery"),
    (0x003f_3b18, net_conn_startup, "NetConnStartup"),
    (0x003f_3ab8, socket_lookup_sync, "SocketLookupSync"),
    (0x003f_3a98, socket_lookup, "SocketLookup"),
    (0x003f_39c8, socket_lookup_create, "SocketLookup_Create"),
    (0x003f_3940, socket_lookup_free, "SocketLookup_Free"),
    (0x003f_3860, socket_lookup_done, "SocketLookup_Done"),
    (0x003f_3748, lobby_decrypt_string, "Lobby_DecryptString"),
    (0x003f_35d0, lobby_encrypt_string, "Lobby_EncryptString"),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule, Events};
    use crate::difftest::fakes::{self, fall_back, hook, prev_cell};
    use crate::difftest::{
        check, check_paths, check_paths_lohi, elf_image, fill, scramble, Rng, DATA, GAME_GP,
    };
    use core::ptr::addr_of_mut;

    const NET_CLOCK_TEST_GATE: u32 = DATA + 0xf7340;
    const NET_CLOCK_TEST_ORIGIN: u32 = NET_CLOCK_TEST_GATE + 4;
    const NET_CLOCK_TEST_MAGIC: u32 = u32::from_be_bytes(*b"TICK");
    const NET_CLOCK_TEST_QUANTUM: u64 = 64;
    const NET_CLOCK_CALL_CYCLES: u64 = 37;

    fn net_clock_fixture(c: &mut Cpu, seed: u32, entry: u32) -> Rng {
        let mut rng = Rng(0x1fbed47c ^ u64::from(seed));
        scramble(c, &mut rng);
        c.pc = entry;
        c.g[GP][0] = sx(GAME_GP);
        rng
    }

    fn net_tick_fake_division(c: &mut Cpu) {
        if c.bus.read32(NET_CLOCK_TEST_GATE) != NET_CLOCK_TEST_MAGIC {
            return fall_back(
                c,
                prev_cell(net_tick_fake_division),
                ee::s15::f_00411538__generated,
            );
        }
        let origin = u64::from(c.bus.read32(NET_CLOCK_TEST_ORIGIN));
        c.g[V0][0] = (c.cycles - origin) / NET_CLOCK_TEST_QUANTUM;
        ret(c);
    }

    fn net_tick_install_fake_clock() {
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_00411538),
            net_tick_fake_division,
            prev_cell(net_tick_fake_division),
        );
    }

    fn net_tick_enable_fake_clock(c: &mut Cpu) {
        c.bus.write32(NET_CLOCK_TEST_GATE, NET_CLOCK_TEST_MAGIC);
        c.bus.write32(NET_CLOCK_TEST_ORIGIN, c.cycles as u32);
    }

    fn net_tick_snapshot(c: &mut Cpu, seed: u32) {
        c.bus.write32(
            DIRTY_RPC_STATE + NET_TICK_COUNT,
            [0, 537, u32::MAX - 31][seed as usize % 3],
        );
        c.bus.write64(
            DIRTY_RPC_STATE + NET_TICK_EPOCH,
            [0, 0x0000_0001_f000_0000, u64::MAX - 127][seed as usize / 3 % 3],
        );
        c.cycles = u64::from(seed % 3) * 10_000;
    }

    #[test]
    fn net_tick_matches() {
        check(ee::s14::f_003f4de8__generated, net_tick, 12, &|c, seed| {
            net_clock_fixture(c, seed, 0x003f_4de8);
            net_tick_snapshot(c, seed);
        });
    }

    #[test]
    fn net_conn_return_true_matches() {
        check(ee::s14::f_003f43d0__generated, net_conn_return_true, 12, &|c, seed| {
            net_clock_fixture(c, seed, 0x003f_43d0);
        });
    }

    #[test]
    fn net_tick_wrapper2_matches() {
        check(ee::s14::f_003f4330__generated, net_tick_wrapper2, 12, &|c, seed| {
            net_clock_fixture(c, seed, 0x003f_4330);
            net_tick_snapshot(c, seed);
        });
    }

    fn net_conn_idle_fixture(c: &mut Cpu, seed: u32) {
        net_clock_fixture(c, seed, 0x003f_42a0);
        net_tick_enable_fake_clock(c);
        let next_idle = if seed % 3 == 2 { 1 } else { 0 };
        c.bus
            .write32(GAME_GP + NET_CONN_OPEN_GP, u32::from(seed % 3 != 0));
        c.bus.write32(GAME_GP + NET_CONN_NEXT_IDLE_GP, next_idle);
        if seed / 3 % 2 != 0 {
            c.bus.write32(NET_CONN_IDLE_CALLBACKS, 0x003f_43d0);
            c.bus.write32(
                NET_CONN_IDLE_CALLBACKS + NET_CONN_IDLE_CALLBACK_ARG,
                DATA + seed * 4,
            );
        }
    }

    #[test]
    fn net_conn_idle_matches() {
        net_tick_install_fake_clock();
        // Throttled, closed/open with no callbacks, closed/open with one callback.
        check_paths(
            ee::s14::f_003f42a0__generated,
            net_conn_idle,
            6,
            &[53, 115, 199, 173, 257],
            &net_conn_idle_fixture,
        );
    }

    driven!(generated_net_conn_idle, ee::s14::f_003f42a0__generated);
    driven!(hand_net_conn_idle, net_conn_idle);

    #[test]
    fn net_conn_idle_events() {
        net_tick_install_fake_clock();
        check_events(
            generated_net_conn_idle,
            hand_net_conn_idle,
            9,
            &|c, seed| {
                net_conn_idle_fixture(c, 3);
                let mut rng = Rng(0x0ac9_98ae ^ u64::from(seed));
                let mode = schedule(c, &mut rng, seed, 0x003f_42a0);
                net_tick_enable_fake_clock(c);
                if mode != Events::Off {
                    // During the idle table scan, after the callback has returned.
                    c.next_event = c.cycles + 95;
                }
                mode
            },
        );
    }

    fn net_conn_sleep_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let rng = net_clock_fixture(c, seed, NET_CONN_SLEEP_ENTRY);
        net_tick_enable_fake_clock(c);
        c.g[A0][0] = [u64::MAX, 1, 0][seed as usize % 3];
        rng
    }

    #[test]
    fn net_conn_sleep_matches() {
        net_tick_install_fake_clock();
        let once = 19 + 2 * NET_CLOCK_CALL_CYCLES;
        let loop_cost = 7 + 2 * NET_CLOCK_CALL_CYCLES;
        check_paths(
            ee::s14::f_003f41b8__generated,
            net_conn_sleep,
            9,
            &[10, once, once + 3 * loop_cost],
            &|c, seed| {
                net_conn_sleep_fixture(c, seed);
            },
        );
    }

    driven!(generated_net_conn_sleep, ee::s14::f_003f41b8__generated);
    driven!(hand_net_conn_sleep, net_conn_sleep);

    #[test]
    fn net_conn_sleep_events() {
        net_tick_install_fake_clock();
        check_events(
            generated_net_conn_sleep,
            hand_net_conn_sleep,
            9,
            &|c, seed| {
                let mut rng = net_conn_sleep_fixture(c, 2 + 3 * seed);
                let mode = schedule(c, &mut rng, seed, NET_CONN_SLEEP_ENTRY);
                net_tick_enable_fake_clock(c);
                if mode != Events::Off {
                    c.next_event = c.cycles + 85;
                }
                mode
            },
        );
    }

    #[test]
    fn net_tick_wrapper_matches() {
        check(ee::s14::f_003f4198__generated, net_tick_wrapper, 12, &|c, seed| {
            net_clock_fixture(c, seed, 0x003f_4198);
            net_tick_snapshot(c, seed);
        });
    }

    #[test]
    fn net_conn_shutdown_matches() {
        check_paths(
            ee::s14::f_003f4118__generated,
            net_conn_shutdown,
            8,
            &[8, 188],
            &|c, seed| {
                net_clock_fixture(c, seed, 0x003f_4118);
                c.bus.write32(GAME_GP + NET_CONN_OPEN_GP, seed % 2);
                c.bus.write32(GAME_GP + NET_CONN_HANDLE_GP, seed / 2);
            },
        );
    }

    #[test]
    fn net_conn_disconnect_matches() {
        check(ee::s14::f_003f40b8__generated, net_conn_disconnect, 8, &|c, seed| {
            net_clock_fixture(c, seed, 0x003f_40b8);
            c.bus.write32(GAME_GP + NET_CONN_HANDLE_GP, seed);
        });
    }

    const NET_STATUS_RPC_STARTED_GP: u32 = 0x1ac0;
    const NET_STATUS_TEST_BUFFER: u32 = DATA + 0x100;
    const DIRTY_RPC_COMMANDS_BEGIN: u32 = 0x4d88;
    const DIRTY_RPC_COMMANDS_END: u32 = 0x4d8c;

    fn net_status_no_rpc_commands(c: &mut Cpu) {
        c.bus.write32(DIRTY_RPC_STATE + DIRTY_RPC_COMMANDS_BEGIN, 0);
        c.bus.write32(DIRTY_RPC_STATE + DIRTY_RPC_COMMANDS_END, 0);
    }

    #[test]
    fn net_conn_mac_matches() {
        check_paths(
            ee::s14::f_003f3f78__generated,
            net_conn_mac,
            8,
            &[16, 203],
            &|c, seed| {
                let mut rng = net_clock_fixture(c, seed, 0x003f_3f78);
                elf_image(c);
                net_status_no_rpc_commands(c);
                let cache = NET_MAC_CACHE_BASE as u32 + NET_MAC_CACHE_OFFSET;
                fill(c, &mut rng, cache, 4);
                c.bus.write8(cache, if seed % 2 == 0 { 0 } else { b'$' });
                let bytes = frame_addr_for_fixture(c, NET_MAC_FRAME, 0);
                for index in 0..6 {
                    c.bus.write8(bytes + index, (seed * 31 + index * 37) as u8);
                }
            },
        );
    }

    fn frame_addr_for_fixture(c: &Cpu, frame_size: u32, offset: u32) -> u32 {
        r32(c, SP) - frame_size + offset
    }

    fn net_conn_status_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let mut rng = net_clock_fixture(c, seed, NET_CONN_STATUS_ENTRY);
        let selector = [
            NET_STATUS_OPEN,
            NET_STATUS_ONLINE,
            NET_STATUS_CONNECTION,
            NET_STATUS_HOST,
            NET_STATUS_ADDRESS,
            NET_STATUS_MAC,
            NET_STATUS_BROADBAND,
            NET_STATUS_TYPE,
            0,
        ][seed as usize % 9];
        c.g[A0][0] = selector;
        c.g[A1][0] = sx(NET_STATUS_TEST_BUFFER);
        c.g[A2][0] = 16;
        c.bus.write32(GAME_GP + NET_CONN_OPEN_GP, seed / 9);
        c.bus.write32(GAME_GP + NET_CONN_HANDLE_GP, seed / 9 + 1);
        fill(c, &mut rng, NET_STATUS_TEST_BUFFER, 4);
        let result = [0, 4, 0xffff_ffff][seed as usize / 9 % 3];
        let word = frame_addr_for_fixture(c, NET_CONN_STATUS_FRAME, NET_RPC_REPLY_WORD);
        let other = frame_addr_for_fixture(c, NET_CONN_STATUS_FRAME, NET_RPC_STATUS_WORD);
        c.bus.write32(word, result);
        c.bus.write32(other, result.wrapping_add(37));
        rng
    }

    #[test]
    fn net_conn_status_matches() {
        check_paths(
            ee::s14::f_003f3cc0__generated,
            net_conn_status,
            27,
            &[21, 113, 115, 119, 121, 123, 133, 130, 41],
            &|c, seed| {
                net_conn_status_fixture(c, seed);
            },
        );
    }

    driven!(generated_net_conn_status, ee::s14::f_003f3cc0__generated);
    driven!(hand_net_conn_status, net_conn_status);

    #[test]
    fn net_conn_status_events() {
        check_events(
            generated_net_conn_status,
            hand_net_conn_status,
            9,
            &|c, seed| {
                let mut rng = net_conn_status_fixture(c, 6 + 9 * seed);
                let mode = schedule(c, &mut rng, seed, NET_CONN_STATUS_ENTRY);
                if mode != Events::Off {
                    // After the request, during the broadband result rewrite.
                    c.next_event = c.cycles + 121;
                }
                mode
            },
        );
    }

    #[test]
    fn net_conn_connect_matches() {
        check(ee::s14::f_003f3c50__generated, net_conn_connect, 8, &|c, seed| {
            net_clock_fixture(c, seed, 0x003f_3c50);
            c.g[A0][0] = sx(DATA);
            c.bus.write32(GAME_GP + NET_CONN_HANDLE_GP, seed);
            let word = frame_addr_for_fixture(c, 0x50, NET_RPC_REPLY_WORD);
            c.bus.write32(word, seed.wrapping_sub(2));
        });
    }

    #[test]
    fn net_conn_query_matches() {
        check(ee::s14::f_003f3bc8__generated, net_conn_query, 8, &|c, seed| {
            let mut rng = net_clock_fixture(c, seed, 0x003f_3bc8);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(NET_STATUS_TEST_BUFFER);
            c.g[A2][0] = u64::from(seed % 3);
            fill(c, &mut rng, NET_STATUS_TEST_BUFFER, 8);
            c.bus.write32(GAME_GP + NET_CONN_HANDLE_GP, seed);
            let word = frame_addr_for_fixture(c, 0x70, NET_RPC_REPLY_WORD);
            c.bus.write32(word, seed.wrapping_sub(2));
        });
    }

    #[test]
    fn net_conn_startup_matches() {
        check_paths(
            ee::s14::f_003f3b18__generated,
            net_conn_startup,
            8,
            &[14, 106],
            &|c, seed| {
                net_clock_fixture(c, seed, 0x003f_3b18);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(GAME_GP + NET_CONN_OPEN_GP, seed % 2);
                c.bus.write32(GAME_GP + NET_STATUS_RPC_STARTED_GP, 1);
            },
        );
    }

    const LOOKUP_TEST_GATE: u32 = DATA + 0xf7350;
    const LOOKUP_TEST_ALLOC: u32 = LOOKUP_TEST_GATE + 4;
    const LOOKUP_TEST_STATUS: u32 = LOOKUP_TEST_GATE + 8;
    const LOOKUP_TEST_MAGIC: u32 = u32::from_be_bytes(*b"LOOK");
    const LOOKUP_TEST_OBJECT: u32 = DATA + 0x200;
    const LOOKUP_FAKE_CYCLES: u64 = 4;

    fn lookup_log_call(c: &mut Cpu, address: u32) {
        let cycles = c.cycles;
        let return_pc = r32(c, RA);
        fakes::fake(c, address, return_pc);
        c.cycles = cycles + LOOKUP_FAKE_CYCLES;
    }

    fn lookup_fake_allocate(c: &mut Cpu) {
        if c.bus.read32(LOOKUP_TEST_GATE) != LOOKUP_TEST_MAGIC {
            return fall_back(
                c,
                prev_cell(lookup_fake_allocate),
                ee::s10::f_00317e30__generated,
            );
        }
        lookup_log_call(c, 0x0031_7e30);
        c.g[V0][0] = sx(c.bus.read32(LOOKUP_TEST_ALLOC));
    }

    fn lookup_fake_free(c: &mut Cpu) {
        if c.bus.read32(LOOKUP_TEST_GATE) != LOOKUP_TEST_MAGIC {
            return fall_back(
                c,
                prev_cell(lookup_fake_free),
                ee::s10::f_00317e98__generated,
            );
        }
        lookup_log_call(c, 0x0031_7e98);
    }

    fn lookup_finish_after_memset(c: &mut Cpu) {
        let object = r32(c, A0);
        let called_for_lookup = c.bus.read32(LOOKUP_TEST_GATE) == LOOKUP_TEST_MAGIC
            && object == LOOKUP_TEST_OBJECT
            && r32(c, A2) == LOOKUP_SIZE;
        let return_pc = r32(c, RA);
        fall_back(
            c,
            prev_cell(lookup_finish_after_memset),
            ee::s15::f_00416210__generated,
        );
        if called_for_lookup && c.pc == return_pc {
            let status = c.bus.read32(LOOKUP_TEST_STATUS);
            if status != 0 {
                // Supply the reply an IOP lookup would leave in the object.
                c.bus.write32(object + LOOKUP_STATUS, status);
                c.bus.write32(object + LOOKUP_ADDRESS, 0x7f00_0001);
            }
        }
    }

    fn socket_lookup_install_fakes() {
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            lookup_fake_allocate,
            prev_cell(lookup_fake_allocate),
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            lookup_fake_free,
            prev_cell(lookup_fake_free),
        );
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_00416210),
            lookup_finish_after_memset,
            prev_cell(lookup_finish_after_memset),
        );
    }

    fn socket_lookup_fixture(c: &mut Cpu, seed: u32, entry: u32, allocate: bool) {
        let mut rng = net_clock_fixture(c, seed, entry);
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = u64::from(seed % 3);
        c.bus.write8(DATA, b'x');
        c.bus.write8(DATA + 1, 0);
        c.bus.write32(LOOKUP_TEST_GATE, LOOKUP_TEST_MAGIC);
        c.bus.write32(
            LOOKUP_TEST_ALLOC,
            if allocate { LOOKUP_TEST_OBJECT } else { 0 },
        );
        c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
        fill(c, &mut rng, LOOKUP_TEST_OBJECT, LOOKUP_SIZE / 4);
    }

    #[test]
    fn socket_lookup_sync_matches() {
        socket_lookup_install_fakes();
        check_paths(
            ee::s14::f_003f3ab8__generated,
            socket_lookup_sync,
            6,
            &[49, 469, 555],
            &|c, seed| {
                let mode = seed % 3;
                socket_lookup_fixture(c, seed, 0x003f_3ab8, mode != 0);
                c.bus
                    .write32(LOOKUP_TEST_STATUS, [0, u32::MAX, 1][mode as usize]);
            },
        );
    }

    #[test]
    fn socket_lookup_matches() {
        socket_lookup_install_fakes();
        check_paths(
            ee::s14::f_003f3a98__generated,
            socket_lookup,
            6,
            &[42, 233],
            &|c, seed| {
                socket_lookup_fixture(c, seed, 0x003f_3a98, seed % 2 == 1);
            },
        );
    }

    #[test]
    fn socket_lookup_create_matches() {
        socket_lookup_install_fakes();
        check_paths(
            ee::s14::f_003f39c8__generated,
            socket_lookup_create,
            6,
            &[35, 226],
            &|c, seed| {
                socket_lookup_fixture(c, seed, 0x003f_39c8, seed % 2 == 1);
            },
        );
    }

    #[test]
    fn socket_lookup_free_matches() {
        socket_lookup_install_fakes();
        check(ee::s14::f_003f3940__generated, socket_lookup_free, 6, &|c, seed| {
            socket_lookup_fixture(c, seed, 0x003f_3940, false);
            c.g[A0][0] = sx(LOOKUP_TEST_OBJECT);
            c.bus
                .write32(LOOKUP_TEST_OBJECT + LOOKUP_REMOTE_HANDLE, seed);
        });
    }

    #[test]
    fn socket_lookup_done_matches() {
        check_paths(
            ee::s14::f_003f3860__generated,
            socket_lookup_done,
            6,
            &[106, 192],
            &|c, seed| {
                let mut rng = net_clock_fixture(c, seed, 0x003f_3860);
                c.g[A0][0] = sx(LOOKUP_TEST_OBJECT);
                fill(c, &mut rng, LOOKUP_TEST_OBJECT, LOOKUP_SIZE / 4);
                c.bus
                    .write32(LOOKUP_TEST_OBJECT + LOOKUP_REMOTE_HANDLE, seed / 3);
                c.bus.write32(
                    LOOKUP_TEST_OBJECT + LOOKUP_STATUS,
                    [0, u32::MAX, 1][seed as usize % 3],
                );
            },
        );
    }

    const CIPHER_TEST_READY_GP: u32 = 0x1aac;
    const CIPHER_TEST_INPUT: u32 = DATA + 0x300;
    const CIPHER_TEST_OUTPUT: u32 = DATA + 0x400;
    const CIPHER_DECRYPT_ENTRY: u32 = 0x003f_3748;
    const CIPHER_ENCRYPT_ENTRY: u32 = 0x003f_35d0;

    fn lobby_cipher_string_fixture(c: &mut Cpu, seed: u32, entry: u32) -> Rng {
        let rng = net_clock_fixture(c, seed, entry);
        elf_image(c);
        // Each call initializes its own permutation. Keep the one-time
        // process-wide padding initializer out of the per-string fixtures.
        c.bus.write32(GAME_GP + CIPHER_TEST_READY_GP, 1);
        c.g[A0][0] = sx(CIPHER_TEST_OUTPUT);
        c.g[A1][0] = u64::from(seed % 5);
        c.g[A2][0] = sx(CIPHER_TEST_INPUT);
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        for offset in 0..8 {
            c.bus.write8(CIPHER_TEST_OUTPUT + offset, 0xa5);
        }
        rng
    }

    fn lobby_decrypt_string_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let rng = lobby_cipher_string_fixture(c, seed, CIPHER_DECRYPT_ENTRY);
        // With the identity permutation the first keystream byte is zero.
        // A DEL input therefore reaches the decoded end-marker branch.
        let input = [b"\0\0", b"\x7f\0", b"A\0", b"AB"][seed as usize / 5 % 4];
        for (offset, &byte) in input.iter().enumerate() {
            c.bus.write8(CIPHER_TEST_INPUT + offset as u32, byte);
        }
        c.bus.write8(CIPHER_TEST_INPUT + 2, 0);
        rng
    }

    #[test]
    fn lobby_decrypt_string_matches() {
        check_paths_lohi(
            ee::s14::f_003f3748__generated,
            lobby_decrypt_string,
            20,
            &[1856, 1860, 1922, 1925, 1929, 1990, 1994],
            &|c, seed| {
                lobby_decrypt_string_fixture(c, seed);
            },
        );
    }

    driven!(generated_lobby_decrypt_string, ee::s14::f_003f3748__generated);
    driven!(hand_lobby_decrypt_string, lobby_decrypt_string);

    #[test]
    fn lobby_decrypt_string_events() {
        check_events(
            generated_lobby_decrypt_string,
            hand_lobby_decrypt_string,
            9,
            &|c, seed| {
                let mut rng = lobby_decrypt_string_fixture(c, 19);
                let mode = schedule(c, &mut rng, seed, CIPHER_DECRYPT_ENTRY);
                if mode != Events::Off {
                    c.next_event = c.cycles + 1910;
                }
                mode
            },
        );
    }

    fn lobby_encrypt_string_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let rng = lobby_cipher_string_fixture(c, seed, CIPHER_ENCRYPT_ENTRY);
        match seed / 5 % 4 {
            0 => c.g[A2][0] = 0,
            1 => c.bus.write8(CIPHER_TEST_INPUT, 0),
            2 => {
                c.bus.write8(CIPHER_TEST_INPUT, b'A');
                c.bus.write8(CIPHER_TEST_INPUT + 1, 0xff);
            }
            _ => {
                c.bus.write8(CIPHER_TEST_INPUT, b'A');
                c.bus.write8(CIPHER_TEST_INPUT + 1, b'B');
            }
        }
        c.bus.write8(CIPHER_TEST_INPUT + 2, 0);
        rng
    }

    #[test]
    fn lobby_encrypt_string_matches() {
        // A byte from input costs 71 cycles and a padding byte costs 116.
        // All runs start with the same three cipher initializations.
        const NO_OUTPUT: u64 = 11750;
        const INPUT_BYTE: u64 = 71;
        const PAD_BYTE: u64 = 116;
        const LOOP_ENTRY: u64 = 1;
        check_paths_lohi(
            ee::s14::f_003f35d0__generated,
            lobby_encrypt_string,
            20,
            &[
                NO_OUTPUT,
                NO_OUTPUT + LOOP_ENTRY + INPUT_BYTE,
                NO_OUTPUT + LOOP_ENTRY + PAD_BYTE,
                NO_OUTPUT + LOOP_ENTRY + 2 * INPUT_BYTE,
                NO_OUTPUT + LOOP_ENTRY + INPUT_BYTE + PAD_BYTE,
                NO_OUTPUT + LOOP_ENTRY + 2 * PAD_BYTE,
            ],
            &|c, seed| {
                lobby_encrypt_string_fixture(c, seed);
            },
        );
    }

    driven!(generated_lobby_encrypt_string, ee::s14::f_003f35d0__generated);
    driven!(hand_lobby_encrypt_string, lobby_encrypt_string);

    #[test]
    fn lobby_encrypt_string_events() {
        check_events(
            generated_lobby_encrypt_string,
            hand_lobby_encrypt_string,
            9,
            &|c, seed| {
                let mut rng = lobby_encrypt_string_fixture(c, 19);
                let mode = schedule(c, &mut rng, seed, CIPHER_ENCRYPT_ENTRY);
                if mode != Events::Off {
                    c.next_event = c.cycles + 11800;
                }
                mode
            },
        );
    }
}
