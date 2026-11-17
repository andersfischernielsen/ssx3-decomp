//! Staging module oc1063 (wave oc1063): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1063`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use callee::*;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003f_4448, dirty_rpc_alarm_handler, "DirtyRpc_AlarmHandler"),
    (0x003f_44f8, dirty_rpc_thread_main, "DirtyRpc_ThreadMain"),
    (
        0x003f_4a78,
        dirty_rpc_process_replies,
        "DirtyRpc_ProcessReplies",
    ),
    (0x003f_4c68, dirty_rpc_call, "DirtyRpc_Call"),
    (0x003f_4c98, dirty_rpc_init_request, "DirtyRpc_InitRequest"),
    (0x003f_4d68, dirty_rpc_init_iop, "DirtyRpc_InitIop"),
    (0x003f_4e80, dirty_rpc_sync_dcache, "DirtyRpc_SyncDCache"),
    (
        0x003f_4ea0,
        net_game_link_check_recv_buf,
        "NetGameLink_CheckRecvBuf",
    ),
    (0x003f_4f40, dirty_rpc_wait_iop, "DirtyRpc_WaitIop"),
    (0x003f_5000, dirty_rpc_startup, "DirtyRpc_Startup"),
    (0x003f_53f8, dirty_rpc_shutdown, "DirtyRpc_Shutdown"),
    (0x003f_5470, dirty_rpc_is_started, "DirtyRpc_IsStarted"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::lib::gcc::SIGNAL_SEMA;
    pub(crate) use crate::game::lib::gcc::WAIT_SEMA;
    pub(crate) use crate::game::misc::MEMSET;
    pub(crate) use crate::game::misc::SYNC_DCACHE;
    pub(crate) use crate::game::wscript::wscriptman::callee::REMOVE_INTC_HANDLER;

    pub const DIRTY_RPC_SEND: Callee = Callee {
        addr: 0x003f_4528,
        f: ee::s14::DirtyRpc_Send,
    };

    pub const DIRTY_RPC_PROCESS_REPLIES: Callee = Callee {
        addr: 0x003f_4a78,
        f: ee::s14::DirtyRpc_ProcessReplies,
    };

    pub const DIRTY_RPC_CALL: Callee = Callee {
        addr: 0x003f_4c68,
        f: ee::s14::DirtyRpc_Call,
    };

    pub const DIRTY_RPC_WAIT_IOP: Callee = Callee {
        addr: 0x003f_4f40,
        f: ee::s14::DirtyRpc_WaitIop,
    };

    pub const ADD_INTC_HANDLER: Callee = Callee {
        addr: 0x0042_3a90,
        f: ee::s15::f_00423a90,
    };

    pub const RELEASE_ALARM: Callee = Callee {
        addr: 0x0042_3b30,
        f: ee::s15::ReleaseAlarm,
    };

    pub const I_SET_ALARM: Callee = Callee {
        addr: 0x0042_3b80,
        f: ee::s15::iSetAlarm,
    };

    pub const CREATE_THREAD: Callee = Callee {
        addr: 0x0042_3ba0,
        f: ee::s15::CreateThread,
    };

    pub const DELETE_THREAD: Callee = Callee {
        addr: 0x0042_3bb0,
        f: ee::s15::DeleteThread,
    };

    pub const START_THREAD: Callee = Callee {
        addr: 0x0042_3bc0,
        f: ee::s15::_StartThread,
    };

    pub const TERMINATE_THREAD: Callee = Callee {
        addr: 0x0042_3bf0,
        f: ee::s15::TerminateThread,
    };

    pub const SLEEP_THREAD: Callee = Callee {
        addr: 0x0042_3cc0,
        f: ee::s15::SleepThread,
    };

    pub const WAKEUP_THREAD: Callee = Callee {
        addr: 0x0042_3cd0,
        f: ee::s15::WakeupThread,
    };

    pub const CREATE_SEMA: Callee = Callee {
        addr: 0x0042_3da0,
        f: ee::s15::CreateSema,
    };

    pub const DELETE_SEMA: Callee = Callee {
        addr: 0x0042_3db0,
        f: ee::s15::DeleteSema,
    };

    pub const FLUSH_CACHE: Callee = Callee {
        addr: 0x0042_4020,
        f: ee::s15::FlushCache,
    };

    pub const SCE_SIF_DMA_STAT: Callee = Callee {
        addr: 0x0042_4140,
        f: ee::s15::sceSifDmaStat,
    };

    pub const SCE_SIF_SET_DMA: Callee = Callee {
        addr: 0x0042_4160,
        f: ee::s15::sceSifSetDma,
    };

    pub const ENABLE_INTC: Callee = Callee {
        addr: 0x0042_48e8,
        f: ee::s15::EnableIntc,
    };

    pub const I_WAKEUP_THREAD: Callee = Callee {
        addr: 0x0042_4c50,
        f: ee::s15::iWakeupThread,
    };

    pub const SCE_SIF_INIT_CMD: Callee = Callee {
        addr: 0x0042_5d38,
        f: ee::s15::sceSifInitCmd,
    };

    pub const SCE_SIF_SEND_CMD: Callee = Callee {
        addr: 0x0042_61b0,
        f: ee::s15::sceSifSendCmd,
    };

    pub const SCE_SIF_ALLOC_IOP_HEAP: Callee = Callee {
        addr: 0x0042_aba0,
        f: ee::s15::sceSifAllocIopHeap,
    };
}

// ---------------------------------------------------------------------------
// DirtyRpc: the EE side of DirtySDK's IOP RPC transport.
//
// All state lives in one 0x4e40-byte block (`lui 0x52; addiu -0xf40`) that
// `DirtyRpc_Startup` clears. Its first 0x4c00 bytes are four regions the IOP
// also sees (through their uncached `0x2000_0000` aliases): the client table
// (16-byte entries), the request ring and the reply ring (0x20-byte slots, one
// pair per ring index) and the EE data buffer. Behind them come the DMA ids
// (0x40 bytes), one 16-byte waiter record per ring index, and the control
// words below.

/// The transport state block.
pub(crate) const DIRTY_RPC_STATE: u32 = 0x0051_f0c0;
/// Its size (cleared by the startup; the transport thread's stack follows).
pub(crate) const DIRTY_RPC_STATE_SIZE: u32 = 0x4e40;
/// The byte at +0: the IOP sets it when it answers the boot command.
pub(crate) const DIRTY_RPC_BOOT_REPLY: u32 = 0x0;
/// The first two words of the table the startup sends the IOP: 0x10, and
/// the limit request ids count modulo (0x7fff_ffff).
pub(crate) const DIRTY_RPC_TABLE_HEAD: u32 = 0x4d80;
pub(crate) const DIRTY_RPC_ID_LIMIT: u32 = 0x4d84;
/// The client table: start, end (EE uncached addresses) and its IOP address.
pub(crate) const DIRTY_RPC_CLIENTS: u32 = 0x4d88;
pub(crate) const DIRTY_RPC_CLIENTS_END: u32 = 0x4d8c;
pub(crate) const DIRTY_RPC_IOP_CLIENTS: u32 = 0x4d90;
/// The request ring (0x20-byte slots): start, end, IOP address.
pub(crate) const DIRTY_RPC_REQUESTS: u32 = 0x4d94;
pub(crate) const DIRTY_RPC_REQUESTS_END: u32 = 0x4d98;
pub(crate) const DIRTY_RPC_IOP_REQUESTS: u32 = 0x4d9c;
/// The reply ring (0x20-byte slots): start, end, IOP address.
pub(crate) const DIRTY_RPC_REPLIES: u32 = 0x4da0;
pub(crate) const DIRTY_RPC_REPLIES_END: u32 = 0x4da4;
pub(crate) const DIRTY_RPC_IOP_REPLIES: u32 = 0x4da8;
/// The EE data buffer: start, end, and the IOP address of its twin.
pub(crate) const DIRTY_RPC_EE_BUFFER: u32 = 0x4dac;
pub(crate) const DIRTY_RPC_EE_BUFFER_END: u32 = 0x4db0;
pub(crate) const DIRTY_RPC_IOP_BUFFER: u32 = 0x4db4;
/// The `sceSifSetDma` id per ring slot (a 0x40-byte cached area at
/// +0x4c00): start and end.
pub(crate) const DIRTY_RPC_DMA_IDS: u32 = 0x4db8;
pub(crate) const DIRTY_RPC_DMA_IDS_END: u32 = 0x4dbc;
/// The IOP heap block `sceSifAllocIopHeap` gave the transport.
pub(crate) const DIRTY_RPC_IOP_HEAP: u32 = 0x4dc0;
/// The waiter records (16 bytes per ring index, at +0x4c40).
pub(crate) const DIRTY_RPC_WAITERS: u32 = 0x4dc4;
/// A pointer to +0x4d40 the startup stores.
pub(crate) const DIRTY_RPC_WORD_4DC8: u32 = 0x4dc8;
/// The next request id (1 after the startup).
pub(crate) const DIRTY_RPC_NEXT_ID: u32 = 0x4dcc;
/// The ring: the slot the next send fills, the oldest unanswered slot (the
/// next reply to take), and the slot count.
pub(crate) const DIRTY_RPC_HEAD: u32 = 0x4dd0;
pub(crate) const DIRTY_RPC_TAIL: u32 = 0x4dd4;
pub(crate) const DIRTY_RPC_SLOTS: u32 = 0x4dd8;
/// The data buffer's read offset (just past the last taken reply's data).
pub(crate) const DIRTY_RPC_READ: u32 = 0x4de0;
/// The data buffer's size.
pub(crate) const DIRTY_RPC_BUFFER_SIZE: u32 = 0x4de4;
/// The transport thread.
pub(crate) const DIRTY_RPC_THREAD: u32 = 0x4de8;
/// Two alarm ids (negative: none): the poll alarm `DirtyRpc_Send` starts,
/// and the 0x10-tick alarm the alarm handler re-arms itself with.
pub(crate) const DIRTY_RPC_ALARM: u32 = 0x4dec;
pub(crate) const DIRTY_RPC_HANDLER_ALARM: u32 = 0x4df0;
/// The SIF interrupt handler id.
pub(crate) const DIRTY_RPC_INTC_HANDLER: u32 = 0x4df4;
/// The semaphore guarding the rings.
pub(crate) const DIRTY_RPC_SEMA: u32 = 0x4e08;

/// Request and reply slots: 0x20 bytes, matched by the word at +0x1c.
pub(crate) const DIRTY_RPC_SLOT_SHIFT: u32 = 5;
pub(crate) const DIRTY_RPC_SLOT_SEQ: u32 = 0x1c;
/// A request slot's data (an IOP address in the data buffer) and its size.
pub(crate) const DIRTY_RPC_REQUEST_DATA: u32 = 0x0;
pub(crate) const DIRTY_RPC_REQUEST_SIZE: u32 = 0x4;
/// A reply slot's words passed on to a callback (+0x14 is also the result
/// word stored for a waiter).
pub(crate) const DIRTY_RPC_REPLY_WORD_8: u32 = 0x8;
pub(crate) const DIRTY_RPC_REPLY_WORD_C: u32 = 0xc;
pub(crate) const DIRTY_RPC_REPLY_WORD_10: u32 = 0x10;
pub(crate) const DIRTY_RPC_REPLY_RESULT: u32 = 0x14;
/// Waiter records: 16 bytes. +0 is 0 (nobody), 1 (store the result word
/// through +4 and wake thread +0xc) or a callback called with +8 and the
/// reply's words (+4 as its stack argument).
pub(crate) const DIRTY_RPC_WAITER_SHIFT: u32 = 4;
pub(crate) const DIRTY_RPC_WAITER_KIND: u32 = 0x0;
pub(crate) const DIRTY_RPC_WAITER_RESULT: u32 = 0x4;
pub(crate) const DIRTY_RPC_WAITER_ARG: u32 = 0x8;
pub(crate) const DIRTY_RPC_WAITER_THREAD: u32 = 0xc;
/// The waiter kind that stores the result word.
pub(crate) const DIRTY_RPC_WAIT_RESULT: u64 = 1;

/// Client table entries (16 bytes) and the request header `DirtyRpc_InitRequest`
/// fills from them: +0 the entry's index, +4 the id looked up, +8 a second
/// word that must match too, +0xc the client's id word (0 until bound;
/// copied on a match, compared when the cached index is checked).
pub(crate) const DIRTY_RPC_CLIENT_INDEX: u32 = 0x0;
pub(crate) const DIRTY_RPC_CLIENT_SERVER: u32 = 0x4;
pub(crate) const DIRTY_RPC_CLIENT_KEY: u32 = 0x8;
pub(crate) const DIRTY_RPC_CLIENT_ID: u32 = 0xc;
pub(crate) const DIRTY_RPC_CLIENT_SIZE: u32 = 0x10;
/// The cached index is only used below this.
pub(crate) const DIRTY_RPC_MAX_CLIENTS: i64 = 129;

/// gp-relative flags: 1 while the transport thread waits for the semaphore,
/// 1 while it sleeps, and 1 once the transport is started.
pub(crate) const DIRTY_RPC_SEMA_WAIT_GP: u32 = 0x2a10;
pub(crate) const DIRTY_RPC_SLEEPING_GP: u32 = 0x2a14;
pub(crate) const DIRTY_RPC_STARTED_GP: u32 = 0x1ac0;

/// The boot command packet sent to the IOP (first byte: its size; the word
/// at +0xc gets the address the IOP answers to).
pub(crate) const DIRTY_RPC_BOOT_CMD: u32 = 0x0045_1340;
pub(crate) const DIRTY_RPC_BOOT_CMD_REPLY: u32 = 0xc;
/// How long the boot handshake waits (COP0 Count ticks).
pub(crate) const DIRTY_RPC_BOOT_TIMEOUT: u64 = 50_000_000;

/// The thread, alarm and interrupt the startup sets up.
pub(crate) const DIRTY_RPC_THREAD_MAIN: u32 = 0x003f_44f8;
pub(crate) const DIRTY_RPC_ALARM_HANDLER: u32 = 0x003f_4448;
pub(crate) const DIRTY_RPC_INTC_HANDLER_FN: u32 = 0x003f_4410;
pub(crate) const DIRTY_RPC_THREAD_STACK: u32 = DIRTY_RPC_STATE + DIRTY_RPC_STATE_SIZE;
pub(crate) const DIRTY_RPC_THREAD_STACK_SIZE: u64 = 0x2000;
pub(crate) const DIRTY_RPC_THREAD_PRIORITY: u64 = 0xa;
pub(crate) const DIRTY_RPC_THREAD_GP: u32 = 0x004a_30f0;
/// The handler alarm's period (ticks) and the SIF interrupt cause.
pub(crate) const DIRTY_RPC_HANDLER_TICKS: u64 = 0x10;
pub(crate) const DIRTY_RPC_INTC_SIF: u64 = 2;

/// `lw` of the word at `base + off`.
fn dirty_rpc_lw(c: &mut Cpu, base: u32, off: u32) -> u64 {
    Addr(base.wrapping_add(off)).lw(c)
}

/// `sw` of `v` to `base + off`.
fn dirty_rpc_sw(c: &mut Cpu, base: u32, off: u32, v: u64) {
    Addr(base.wrapping_add(off)).sw(c, v)
}

/// With `v0` = the read index's slot offset: the seq words of that reply slot
/// (`a1`) and request slot (`v1`), as `a0`/`v0` point at them; true when the
/// reply answers the request.
fn dirty_rpc_reply_arrived(c: &mut Cpu, rpc: u32) -> bool {
    c.g[A0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_REPLIES);
    c.g[V1][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_REQUESTS);
    c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    c.g[A1][0] = dirty_rpc_lw(c, r32(c, A0), DIRTY_RPC_SLOT_SEQ);
    c.g[V1][0] = dirty_rpc_lw(c, r32(c, V0), DIRTY_RPC_SLOT_SEQ);
    c.g[A1][0] == c.g[V1][0]
}

/// `void DirtyRpc_AlarmHandler(int alarm, u16 time, DirtyRpc* rpc)`
/// (0x3f4448): the transport's alarm callback. Forgets whichever of the two
/// alarm ids fired (-1), wakes the transport thread (`iWakeupThread`) when the
/// reply at the read index answers its request, and when neither alarm is
/// pending any more re-arms itself as the 0x10-tick handler alarm.
pub fn dirty_rpc_alarm_handler(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        let rpc = r32(c, S0);
        c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_ALARM);
        if c.g[A0][0] == c.g[V0][0] {
            c.cycles += 3;
            c.g[V0][0] = u64::MAX;
            dirty_rpc_sw(c, rpc, DIRTY_RPC_ALARM, c.g[V0][0]);
        }
        c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_HANDLER_ALARM);
        c.cycles += 2;
        if c.g[A0][0] == c.g[V0][0] {
            c.cycles += 3;
            c.g[V0][0] = u64::MAX;
            dirty_rpc_sw(c, rpc, DIRTY_RPC_HANDLER_ALARM, c.g[V0][0]);
        }
        c.g[V1][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_TAIL);
        // Whether the path ends through the shared `+1` block (nothing
        // pending, or the alarm was re-armed).
        let tail = 'checked: {
            c.cycles += 3;
            c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_HEAD);
            let empty = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = sx(r32(c, V1) << DIRTY_RPC_SLOT_SHIFT);
            if empty {
                break 'checked true;
            }
            c.cycles += 8;
            if dirty_rpc_reply_arrived(c, rpc) {
                c.cycles += 3;
                c.g[A0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_THREAD);
                I_WAKEUP_THREAD.call(c, 0x003f_44b8)?;
            }
            let rpc = r32(c, S0);
            c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_ALARM);
            c.cycles += 2;
            if (c.g[V0][0] as i64) >= 0 {
                break 'checked false;
            }
            c.cycles += 3;
            c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_HANDLER_ALARM);
            if (c.g[V0][0] as i64) >= 0 {
                break 'checked false;
            }
            c.cycles += 6;
            c.g[A1][0] = sx(DIRTY_RPC_ALARM_HANDLER);
            c.g[A0][0] = DIRTY_RPC_HANDLER_TICKS;
            c.g[A2][0] = c.g[S0][0];
            I_SET_ALARM.call(c, 0x003f_44e4)?;
            dirty_rpc_sw(c, r32(c, S0), DIRTY_RPC_HANDLER_ALARM, c.g[V0][0]);
            true
        };
        if tail {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void DirtyRpc_ThreadMain(void)` (0x3f44f8): the transport thread's body.
/// Forever: handle the replies that arrived (`DirtyRpc_ProcessReplies`), then
/// sleep until the alarm or the SIF interrupt wakes it, with the gp flag
/// at `gp+0x2a14` set to 1 while it sleeps. Never returns.
pub fn dirty_rpc_thread_main(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = 1;
        loop {
            c.cycles += 8;
            DIRTY_RPC_PROCESS_REPLIES.call(c, 0x003f_4510)?;
            dirty_rpc_sw(c, r32(c, GP), DIRTY_RPC_SLEEPING_GP, c.g[S0][0]);
            SLEEP_THREAD.call(c, 0x003f_451c)?;
            dirty_rpc_sw(c, r32(c, GP), DIRTY_RPC_SLEEPING_GP, 0);
            if event_due(c) {
                deliver_event_here(c, 0x003f_4508);
            }
        }
    })
}

/// `int DirtyRpc_ProcessReplies(void)` (0x3f4a78): take every reply that has
/// arrived, in ring order. Each one is taken under the ring semaphore (with
/// `gp+0x2a10` set to 1 while waiting for it): the waiter record of its ring
/// index gets the reply (nothing; the result word stored through +4 and
/// thread +0xc woken; or the callback called with the reply's words), the
/// data cursor moves past the reply's data, the read index advances modulo
/// the slot count. Stops at the write index or at a reply that has not
/// arrived (also when it changed while waiting). Returns how many it took.
pub fn dirty_rpc_process_replies(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x52_0000;
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        c.g[S0][0] = sx(DIRTY_RPC_STATE);
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0x10);
        let rpc = r32(c, S0);
        c.g[V1][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_TAIL);
        c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_HEAD);
        let empty = c.g[V1][0] == c.g[V0][0];
        c.g[S2][0] = 0;
        // 'counted: return the count (one more cycle); 'out: `v0` is set.
        'out: {
            'counted: {
                if empty {
                    break 'counted;
                }
                c.cycles += 9;
                c.g[V0][0] = sx(r32(c, V1) << DIRTY_RPC_SLOT_SHIFT);
                let arrived = dirty_rpc_reply_arrived(c, rpc);
                c.g[V0][0] = c.g[S2][0];
                if !arrived {
                    break 'out;
                }
                c.cycles += 3;
                c.g[S3][0] = 1;
                c.g[A0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_SEMA);
                loop {
                    c.cycles += 14;
                    dirty_rpc_sw(c, r32(c, GP), DIRTY_RPC_SEMA_WAIT_GP, c.g[S3][0]);
                    WAIT_SEMA.call(c, 0x003f_4ae4)?;
                    let rpc = r32(c, S0);
                    // The slots of the read index, again under the semaphore.
                    c.g[A3][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_TAIL);
                    c.g[V1][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_REPLIES);
                    c.g[A0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_REQUESTS);
                    c.g[V0][0] = sx(r32(c, A3) << DIRTY_RPC_SLOT_SHIFT);
                    c.g[A2][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                    dirty_rpc_sw(c, r32(c, GP), DIRTY_RPC_SEMA_WAIT_GP, 0);
                    c.g[T1][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    c.g[T6][0] = dirty_rpc_lw(c, r32(c, A2), DIRTY_RPC_SLOT_SEQ);
                    c.g[V0][0] = dirty_rpc_lw(c, r32(c, T1), DIRTY_RPC_SLOT_SEQ);
                    if c.g[T6][0] != c.g[V0][0] {
                        c.cycles += 4;
                        c.g[A0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_SEMA);
                        SIGNAL_SEMA.call(c, 0x003f_4b18)?;
                        c.g[V0][0] = c.g[S2][0];
                        break 'out;
                    }
                    c.g[A1][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_WAITERS);
                    // Hand the reply to its waiter (a2: reply slot, t1:
                    // request slot, t4: waiter record).
                    c.cycles += 16;
                    c.g[V0][0] = sx(r32(c, A3) << DIRTY_RPC_WAITER_SHIFT);
                    c.g[V1][0] = dirty_rpc_lw(c, r32(c, T1), DIRTY_RPC_REQUEST_DATA);
                    c.g[S1][0] = 0;
                    c.g[T4][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
                    c.g[A0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_IOP_BUFFER);
                    c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_EE_BUFFER);
                    c.g[V1][0] = sx(r32(c, V1).wrapping_sub(r32(c, A0)));
                    c.g[A3][0] = dirty_rpc_lw(c, r32(c, A2), DIRTY_RPC_REPLY_RESULT);
                    c.g[T2][0] = dirty_rpc_lw(c, r32(c, A2), DIRTY_RPC_REPLY_WORD_8);
                    c.g[T0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                    c.g[T3][0] = dirty_rpc_lw(c, r32(c, A2), DIRTY_RPC_REPLY_WORD_C);
                    c.g[T5][0] = dirty_rpc_lw(c, r32(c, T4), DIRTY_RPC_WAITER_KIND);
                    c.g[A1][0] = dirty_rpc_lw(c, r32(c, T4), DIRTY_RPC_WAITER_ARG);
                    c.g[T1][0] = dirty_rpc_lw(c, r32(c, T1), DIRTY_RPC_REQUEST_SIZE);
                    let nobody = c.g[T5][0] == 0;
                    c.g[A2][0] = dirty_rpc_lw(c, r32(c, A2), DIRTY_RPC_REPLY_WORD_10);
                    if !nobody {
                        c.cycles += 2;
                        let callback = c.g[T5][0] != DIRTY_RPC_WAIT_RESULT;
                        c.g[V0][0] = dirty_rpc_lw(c, r32(c, T4), DIRTY_RPC_WAITER_RESULT);
                        if callback {
                            c.cycles += 5;
                            Addr(sp(c)).sw(c, c.g[V0][0]);
                            c.g[V0][0] = dirty_rpc_lw(c, r32(c, T4), DIRTY_RPC_WAITER_KIND);
                            c.g[A0][0] = c.g[T6][0];
                            call_ptr(c, r32(c, V0), 0x003f_4b98)?;
                        } else {
                            c.cycles += 2;
                            if c.g[V0][0] != 0 {
                                Addr(r32(c, V0)).sw(c, c.g[A3][0]);
                            }
                            c.cycles += 6;
                            let rpc = r32(c, S0);
                            c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_TAIL);
                            c.g[V1][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_WAITERS);
                            c.g[V0][0] = sx(r32(c, V0) << DIRTY_RPC_WAITER_SHIFT);
                            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                            c.g[S1][0] = dirty_rpc_lw(c, r32(c, V0), DIRTY_RPC_WAITER_THREAD);
                        }
                    }
                    // Move the data cursor past the request's data (rounded
                    // up to 0x40 bytes).
                    c.cycles += 7;
                    let rpc = r32(c, S0);
                    c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_TAIL);
                    c.g[A0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_REQUESTS);
                    c.g[V0][0] = sx(r32(c, V0) << DIRTY_RPC_SLOT_SHIFT);
                    c.g[A1][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    c.g[V1][0] = dirty_rpc_lw(c, r32(c, A1), DIRTY_RPC_REQUEST_SIZE);
                    let no_data = (c.g[V1][0] as i64) <= 0;
                    c.g[A0][0] = 0xffff_ffff_ffff_ffc0;
                    if !no_data {
                        c.cycles += 9;
                        c.g[V1][0] = dirty_rpc_lw(c, r32(c, A1), DIRTY_RPC_REQUEST_DATA);
                        c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_IOP_BUFFER);
                        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(r32(c, V0)));
                        dirty_rpc_sw(c, rpc, DIRTY_RPC_READ, c.g[V1][0]);
                        c.g[V0][0] = dirty_rpc_lw(c, r32(c, A1), DIRTY_RPC_REQUEST_SIZE);
                        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x3f));
                        c.g[V0][0] &= c.g[A0][0];
                        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                        dirty_rpc_sw(c, rpc, DIRTY_RPC_READ, c.g[V1][0]);
                    }
                    // Advance the read index (modulo the slot count).
                    c.cycles += 6;
                    c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_TAIL);
                    c.g[V1][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_SLOTS);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                    c.div(0, V0, V1);
                    c.cycles += 6;
                    c.g[A0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_SEMA);
                    c.g[A1][0] = c.hi[0];
                    dirty_rpc_sw(c, rpc, DIRTY_RPC_TAIL, c.g[A1][0]);
                    SIGNAL_SEMA.call(c, 0x003f_4c04)?;
                    if c.g[S1][0] != 0 {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S1][0];
                        WAKEUP_THREAD.call(c, 0x003f_4c14)?;
                    }
                    let rpc = r32(c, S0);
                    c.g[V1][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_TAIL);
                    c.cycles += 3;
                    c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_HEAD);
                    let drained = c.g[V1][0] == c.g[V0][0];
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                    if drained {
                        break 'counted;
                    }
                    // The next slot pair (registers as the generated code
                    // leaves them: a0 reply, v1 request).
                    c.cycles += 9;
                    c.g[A0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_REPLIES);
                    c.g[V1][0] = sx(r32(c, V1) << DIRTY_RPC_SLOT_SHIFT);
                    c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_REQUESTS);
                    c.g[A0][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                    c.g[A1][0] = dirty_rpc_lw(c, r32(c, A0), DIRTY_RPC_SLOT_SEQ);
                    c.g[V0][0] = dirty_rpc_lw(c, r32(c, V1), DIRTY_RPC_SLOT_SEQ);
                    if c.g[A1][0] != c.g[V0][0] {
                        break 'counted;
                    }
                    c.g[A0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_SEMA);
                    if event_due(c) {
                        deliver_event_here(c, 0x003f_4ad8);
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S2][0];
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int DirtyRpc_Call(...)` (0x3f4c68): `DirtyRpc_Send` with the same
/// registers, the caller's stack words at +0 and +8 as its first and second
/// stack words and `a3` again as its third; returns what it returns.
pub fn dirty_rpc_call(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        c.g[T4][0] = c.g[A3][0];
        c.g[V0][0] = Addr(frame.at(c, 0x60)).lw(c);
        c.g[V1][0] = Addr(frame.at(c, 0x68)).lw(c);
        frame.save_ra(c, 0x50);
        Addr(frame.at(c, 0)).sw(c, c.g[V0][0]);
        Addr(frame.at(c, 0x8)).sw(c, c.g[V1][0]);
        Addr(frame.at(c, 0x10)).sw(c, c.g[T4][0]);
        DIRTY_RPC_SEND.call(c, 0x003f_4c8c)?;
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

/// `int DirtyRpc_InitRequest(Request* req, int id, int key)` (0x3f4c98):
/// bind a request header to the client table entry with this id and key.
/// Returns 0 when the header's cached entry (+0: index, below 129) still has
/// this id and the header's +0xc word; otherwise stores id and key and scans
/// the table: 1 and the entry's index and +0xc word copied on a match, -1 and
/// both cleared when there is none.
pub fn dirty_rpc_init_request(c: &mut Cpu) {
    c.cycles += 6;
    c.g[A3][0] = c.g[A0][0];
    c.g[V0][0] = 0x52_0000;
    let req = r32(c, A3);
    c.g[V1][0] = dirty_rpc_lw(c, req, DIRTY_RPC_CLIENT_SERVER);
    c.g[T0][0] = c.g[A1][0];
    let same_id = c.g[T0][0] == c.g[V1][0];
    c.g[T1][0] = sx(DIRTY_RPC_STATE);
    'scan: {
        // 'stale: the cached entry is not usable (one more cycle, then the
        // key is stored).
        'stale: {
            if !same_id {
                break 'stale;
            }
            c.cycles += 3;
            c.g[V0][0] = dirty_rpc_lw(c, req, DIRTY_RPC_CLIENT_KEY);
            if c.g[A2][0] != c.g[V0][0] {
                dirty_rpc_sw(c, req, DIRTY_RPC_CLIENT_KEY, c.g[A2][0]);
                break 'scan;
            }
            c.cycles += 3;
            c.g[V1][0] = dirty_rpc_lw(c, req, DIRTY_RPC_CLIENT_INDEX);
            let negative = (c.g[V1][0] as i64) < 0;
            c.g[V0][0] = ((c.g[V1][0] as i64) < DIRTY_RPC_MAX_CLIENTS) as u64;
            if negative {
                break 'stale;
            }
            c.cycles += 2;
            let too_big = c.g[V0][0] == 0;
            c.g[V1][0] = sx(r32(c, V1) << 4);
            if too_big {
                break 'stale;
            }
            c.cycles += 6;
            c.g[A0][0] = dirty_rpc_lw(c, r32(c, T1), DIRTY_RPC_CLIENTS);
            c.g[A1][0] = dirty_rpc_lw(c, req, DIRTY_RPC_CLIENT_ID);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
            let entry = r32(c, V1);
            c.g[V0][0] = dirty_rpc_lw(c, entry, DIRTY_RPC_CLIENT_ID);
            if c.g[V0][0] != c.g[A1][0] {
                dirty_rpc_sw(c, req, DIRTY_RPC_CLIENT_KEY, c.g[A2][0]);
                break 'scan;
            }
            c.cycles += 3;
            c.g[V0][0] = dirty_rpc_lw(c, entry, DIRTY_RPC_CLIENT_SERVER);
            if c.g[V0][0] != c.g[T0][0] {
                dirty_rpc_sw(c, req, DIRTY_RPC_CLIENT_KEY, c.g[A2][0]);
                break 'scan;
            }
            c.cycles += 2;
            c.g[V0][0] = 0;
            ret(c);
            return;
        }
        c.cycles += 1;
        dirty_rpc_sw(c, req, DIRTY_RPC_CLIENT_KEY, c.g[A2][0]);
    }
    c.cycles += 5;
    dirty_rpc_sw(c, req, DIRTY_RPC_CLIENT_SERVER, c.g[T0][0]);
    let rpc = r32(c, T1);
    c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_CLIENTS_END);
    c.g[A2][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_CLIENTS);
    let empty = c.g[A2][0] == c.g[V0][0];
    c.g[A1][0] = c.g[T0][0];
    if !empty {
        c.cycles += 2;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = dirty_rpc_lw(c, r32(c, A2), DIRTY_RPC_CLIENT_SERVER);
        loop {
            c.cycles += 2;
            if c.g[V0][0] == c.g[A1][0] {
                c.cycles += 4;
                let entry = r32(c, A2);
                c.g[V1][0] = dirty_rpc_lw(c, entry, DIRTY_RPC_CLIENT_KEY);
                c.g[V0][0] = dirty_rpc_lw(c, req, DIRTY_RPC_CLIENT_KEY);
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[A0][0] = dirty_rpc_lw(c, entry, DIRTY_RPC_CLIENT_INDEX);
                    c.cycles += 5;
                    c.g[V0][0] = 1;
                    dirty_rpc_sw(c, req, DIRTY_RPC_CLIENT_INDEX, c.g[A0][0]);
                    c.g[V1][0] = dirty_rpc_lw(c, entry, DIRTY_RPC_CLIENT_ID);
                    dirty_rpc_sw(c, req, DIRTY_RPC_CLIENT_ID, c.g[V1][0]);
                    ret(c);
                    return;
                }
                c.cycles += 1;
            }
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(DIRTY_RPC_CLIENT_SIZE));
            c.cycles += 2;
            if c.g[A2][0] == c.g[A0][0] {
                break;
            }
            c.g[V0][0] = dirty_rpc_lw(c, r32(c, A2), DIRTY_RPC_CLIENT_SERVER);
            if event_due(c) {
                deliver_event_here(c, 0x003f_4d30);
            }
        }
    }
    c.cycles += 4;
    dirty_rpc_sw(c, req, DIRTY_RPC_CLIENT_ID, 0);
    c.g[V0][0] = u64::MAX;
    dirty_rpc_sw(c, req, DIRTY_RPC_CLIENT_INDEX, 0);
    ret(c);
}

/// `int DirtyRpc_InitIop(void)` (0x3f4d68): send request 1 to the IOP side
/// (`DirtyRpc_Call` with a request header on the stack and the client table's
/// address and size), then take replies (`DirtyRpc_ProcessReplies`) until the
/// read index reaches the write index. Returns 0. The wait loop's head is a
/// resume entry (an event there can unwind).
pub fn dirty_rpc_init_iop(c: &mut Cpu) {
    const ENTRY: u32 = 0x003f_4d68;
    const POLL: u32 = 0x003f_4dc0;
    guest(c, |c| {
        let frame = Frame { size: 0x40 };
        let mut resumed = match c.pc {
            ENTRY => false,
            POLL => true,
            pc => {
                c.jump_out(pc, ENTRY, 0x003f_4de8);
                return Ok(());
            }
        };
        if !resumed {
            c.cycles += 21;
            Frame::open(c, 0x40);
            c.g[V1][0] = 1;
            frame.save(c, S0, 0x30);
            c.g[V0][0] = 0x52_0000;
            Addr(frame.at(c, 0)).sw(c, c.g[V1][0]);
            c.g[S0][0] = sx(DIRTY_RPC_STATE);
            frame.save_ra(c, 0x20);
            c.g[A0][0] = sx(frame.at(c, 0x10));
            c.g[A1][0] = 1;
            c.g[A2][0] = 0;
            c.g[T2][0] = dirty_rpc_lw(c, DIRTY_RPC_STATE, DIRTY_RPC_CLIENTS);
            c.g[A3][0] = 0;
            c.g[T3][0] = dirty_rpc_lw(c, DIRTY_RPC_STATE, DIRTY_RPC_CLIENTS_END);
            c.g[T0][0] = 0;
            Addr(frame.at(c, 0x1c)).sw(c, c.g[V1][0]);
            c.g[T1][0] = 0;
            c.g[T3][0] = sx(r32(c, T3).wrapping_sub(r32(c, T2)));
            Addr(frame.at(c, 0x8)).sw(c, 0);
            DIRTY_RPC_CALL.call(c, 0x003f_4db4)?;
            c.g[V1][0] = dirty_rpc_lw(c, r32(c, S0), DIRTY_RPC_HEAD);
        }
        loop {
            if !resumed {
                c.cycles += 3;
                c.g[V0][0] = dirty_rpc_lw(c, r32(c, S0), DIRTY_RPC_TAIL);
                let pending = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 0;
                if !pending {
                    break;
                }
                if back_edge_unwinds(c, POLL) {
                    return Ok(());
                }
            }
            resumed = false;
            c.cycles += 3;
            DIRTY_RPC_PROCESS_REPLIES.call(c, 0x003f_4dc8)?;
            c.g[V1][0] = dirty_rpc_lw(c, r32(c, S0), DIRTY_RPC_HEAD);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void DirtyRpc_SyncDCache(void* start, int size)` (0x3f4e80): write back
/// the data cache over `[start, start + size)` (`SyncDCache(start, start +
/// size)`).
pub fn dirty_rpc_sync_dcache(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
        SYNC_DCACHE.call(c, 0x003f_4e90)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// A receive record's header (read through its uncached alias): +0 the next
/// record, +8 a sequence word (the next record's must equal it), +0xc the
/// data length (0..0x8000); the data follows the 16-byte header.
pub(crate) const NET_RECV_NEXT: u32 = 0x0;
pub(crate) const NET_RECV_SEQ: u32 = 0x8;
pub(crate) const NET_RECV_LEN: u32 = 0xc;
pub(crate) const NET_RECV_MAX_LEN: i64 = 0x8000;

/// `int NetGameLink_CheckRecvBuf(void* buf, int* len)` (0x3f4ea0): check the
/// receive record at `buf` (uncached). Its next pointer must be set, its
/// length 0..0x8000, the next record must start right after the header and
/// the data rounded up to 16 bytes and point back to this one, and the two
/// sequence words must match. Returns the sequence word and stores the
/// length in `*len` (when `len` is set); -1 no record or a bad length, -2 a
/// bad link, -3 the sequence words differ.
pub fn net_game_link_check_recv_buf(c: &mut Cpu) {
    c.cycles += 8;
    c.g[A3][0] = 0x0fff_ffff;
    c.g[T2][0] = 0x2000_0000;
    c.g[A0][0] &= c.g[A3][0];
    c.g[A2][0] = c.g[A0][0] | c.g[T2][0];
    let buf = r32(c, A2);
    c.g[A0][0] = dirty_rpc_lw(c, buf, NET_RECV_NEXT);
    'bad: {
        if c.g[A0][0] == 0 {
            break 'bad;
        }
        c.cycles += 3;
        c.g[T0][0] = dirty_rpc_lw(c, buf, NET_RECV_LEN);
        let negative = (c.g[T0][0] as i64) < 0;
        c.g[V0][0] = NET_RECV_MAX_LEN as u64;
        if negative {
            break 'bad;
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[T0][0] as i64)) as u64;
        let fits = c.g[V0][0] == 0;
        c.g[V0][0] = 0x7fff_0000;
        if !fits {
            break 'bad;
        }
        'unlinked: {
            c.cycles += 9;
            c.g[V1][0] = sx(r32(c, T0).wrapping_add(0xf));
            c.g[V0][0] |= 0xfff0;
            c.g[A0][0] &= c.g[A3][0];
            c.g[V1][0] &= c.g[V0][0];
            c.g[A0][0] |= c.g[T2][0];
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x10));
            c.g[V1][0] = sx(r32(c, A2).wrapping_add(r32(c, V1)));
            let misplaced = c.g[A0][0] != c.g[V1][0];
            c.g[T1][0] = dirty_rpc_lw(c, buf, NET_RECV_SEQ);
            if misplaced {
                break 'unlinked;
            }
            c.cycles += 5;
            let next = r32(c, A0);
            c.g[V0][0] = dirty_rpc_lw(c, next, NET_RECV_NEXT);
            c.g[V0][0] &= c.g[A3][0];
            c.g[V0][0] |= c.g[T2][0];
            if c.g[V0][0] != c.g[A2][0] {
                break 'unlinked;
            }
            c.g[V0][0] = dirty_rpc_lw(c, next, NET_RECV_SEQ);
            c.cycles += 2;
            let torn = c.g[T1][0] != c.g[V0][0];
            c.g[V0][0] = (-3i64) as u64;
            if torn {
                c.cycles += 2;
                ret(c);
                return;
            }
            c.cycles += 2;
            if c.g[A1][0] != 0 {
                Addr(r32(c, A1)).sw(c, c.g[T0][0]);
            }
            c.cycles += 2;
            c.g[V0][0] = c.g[T1][0];
            ret(c);
            return;
        }
        c.cycles += 2;
        c.g[V0][0] = (-2i64) as u64;
        ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = u64::MAX;
    ret(c);
}

/// `int DirtyRpc_WaitIop(void)` (0x3f4f40): the boot handshake. Clears the
/// boot reply byte, puts its address in the boot command packet, flushes the
/// cache and sends the packet (`sceSifSendCmd(1, packet, size, 0, 0, 0)`);
/// then flushes and polls the byte until the IOP sets it (returns 1) or 50
/// million Count ticks pass (returns 0). The poll loop's head is a resume
/// entry (an event there can unwind).
pub fn dirty_rpc_wait_iop(c: &mut Cpu) {
    const ENTRY: u32 = 0x003f_4f40;
    const POLL: u32 = 0x003f_4fb0;
    guest(c, |c| {
        let frame = Frame { size: 0x50 };
        let mut resumed = match c.pc {
            ENTRY => false,
            POLL => true,
            pc => {
                c.jump_out(pc, ENTRY, 0x003f_4ffc);
                return Ok(());
            }
        };
        if !resumed {
            c.cycles += 28;
            Frame::open(c, 0x50);
            c.g[V1][0] = 0x52_0000;
            frame.save(c, S1, 0x30);
            c.g[A0][0] = 0;
            frame.save(c, S3, 0x10);
            c.g[S1][0] = sx(DIRTY_RPC_BOOT_CMD & 0xffff_0000);
            frame.save(c, S0, 0x40);
            c.g[S3][0] = sx(DIRTY_RPC_STATE);
            frame.save(c, S2, 0x20);
            c.g[S0][0] = sx(DIRTY_RPC_BOOT_CMD);
            frame.save_ra(c, 0);
            c.g[V0][0] = sx(DIRTY_RPC_STATE + DIRTY_RPC_BOOT_REPLY + 1);
            Addr(DIRTY_RPC_STATE + DIRTY_RPC_BOOT_REPLY).sb(c, 0);
            c.g[S2][0] = DIRTY_RPC_BOOT_TIMEOUT;
            dirty_rpc_sw(c, r32(c, S0), DIRTY_RPC_BOOT_CMD_REPLY, c.g[V0][0]);
            FLUSH_CACHE.call(c, 0x003f_4f84)?;
            c.g[A2][0] = Addr(r32(c, S1).wrapping_add(0x1340)).lbu(c);
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = 1;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0;
            c.g[T1][0] = 0;
            SCE_SIF_SEND_CMD.call(c, 0x003f_4fa0)?;
            c.g[S0][0] = 1;
            c.mfc0(S1, 9);
            c.g[V0][0] = Addr(r32(c, S3)).lb(c);
        }
        loop {
            if resumed {
                c.cycles += 3;
                c.g[A0][0] = 0;
                FLUSH_CACHE.call(c, 0x003f_4fb8)?;
                c.g[V0][0] = Addr(r32(c, S3)).lb(c);
            }
            resumed = true;
            c.cycles += 2;
            let answered = c.g[V0][0] != 0;
            c.g[V0][0] = c.g[S0][0];
            if answered {
                break;
            }
            c.cycles += 5;
            c.mfc0(V0, 9);
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, S1)));
            c.g[V0][0] = (c.g[S2][0] < c.g[V0][0]) as u64;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[S0][0] = 0;
                c.g[V0][0] = c.g[S0][0];
                break;
            }
            if back_edge_unwinds(c, POLL) {
                return Ok(());
            }
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int DirtyRpc_Startup(void)` (0x3f5000): bring the transport up. After
/// the boot handshake (`DirtyRpc_WaitIop`; -1 when the IOP never answers) it
/// clears the state block and lays out its regions, takes 0x4c80 bytes of IOP
/// heap for their IOP copies (-2 without), DMAs the 0x40-byte table of
/// region addresses (from the stack) to the heap block and then the 0x200
/// bytes of request slots to the IOP (-3/-4 when a DMA cannot be queued;
/// each waits for its DMA), sends the boot packet again with the heap address,
/// sets the ring indices and sizes, creates the ring semaphore and the
/// transport thread (`DirtyRpc_ThreadMain`, priority 10, a 0x2000-byte stack
/// after the state block; -5 when it fails), installs and enables the SIF
/// interrupt handler, starts the thread and sets the started flag. Returns 0.
pub fn dirty_rpc_startup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x110);
        c.g[V0][0] = 0x52_0000;
        frame.save(c, S1, 0xf0);
        frame.save(c, S0, 0x100);
        frame.save(c, S2, 0xe0);
        frame.save(c, S3, 0xd0);
        frame.save(c, S4, 0xc0);
        frame.save(c, S5, 0xb0);
        frame.save_ra(c, 0xa0);
        c.g[S1][0] = sx(DIRTY_RPC_STATE);
        DIRTY_RPC_WAIT_IOP.call(c, 0x003f_502c)?;
        'out: {
            let answered = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S1][0];
            if !answered {
                c.cycles += 2;
                c.g[V0][0] = u64::MAX;
                break 'out;
            }
            // Clear the block, then the region bounds (EE uncached `t1 |`
            // aliases) and the heap size the IOP copies need.
            c.cycles += 60;
            c.g[A1][0] = 0;
            c.g[A2][0] = sx(DIRTY_RPC_STATE_SIZE);
            c.g[S0][0] = 0x0fff_0000;
            MEMSET.call(c, 0x003f_504c)?;
            c.g[S0][0] |= 0xffff;
            let rpc = r32(c, S1);
            c.g[T0][0] = sx(rpc.wrapping_add(0x800));
            c.g[T1][0] = 0x2000_0000;
            c.g[T0][0] &= c.g[S0][0];
            c.g[T4][0] = c.g[S1][0] & c.g[S0][0];
            c.g[A3][0] = sx(rpc.wrapping_add(0xa00));
            c.g[T0][0] |= c.g[T1][0];
            c.g[T4][0] |= c.g[T1][0];
            c.g[A3][0] &= c.g[S0][0];
            c.g[A2][0] = sx(rpc.wrapping_add(0xc00));
            c.g[A3][0] |= c.g[T1][0];
            c.g[A2][0] &= c.g[S0][0];
            c.g[A0][0] = sx(r32(c, T0).wrapping_sub(r32(c, T4)));
            c.g[A2][0] |= c.g[T1][0];
            c.g[A0][0] = sx((r32(c, A0) as i32 >> 4) as u32);
            c.g[V0][0] = sx(r32(c, A3).wrapping_sub(r32(c, T0)));
            c.g[T5][0] = sx(rpc.wrapping_add(0x4c00));
            c.g[A0][0] = sx(r32(c, A0) << 4);
            c.g[V0][0] = sx((r32(c, V0) as i32 >> 5) as u32);
            c.g[V1][0] = sx(r32(c, A2).wrapping_sub(r32(c, A3)));
            c.g[V0][0] = sx(r32(c, V0) << 5);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x40));
            c.g[V1][0] = sx((r32(c, V1) as i32 >> 5) as u32);
            c.g[T2][0] = c.g[T5][0] & c.g[S0][0];
            c.g[T3][0] = sx(rpc.wrapping_add(0x4c40));
            c.g[T2][0] |= c.g[T1][0];
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
            c.g[V1][0] = sx(r32(c, V1) << 5);
            c.g[T1][0] = sx(r32(c, T2).wrapping_sub(r32(c, A2)));
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
            c.g[A1][0] = sx(r32(c, T3).wrapping_sub(r32(c, T5)));
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, T1)));
            c.g[A1][0] = sx((r32(c, A1) as i32 >> 2) as u32);
            c.g[V0][0] = 0x7fff_0000;
            c.g[A1][0] = sx(r32(c, A1) << 2);
            c.g[V0][0] |= 0xffff;
            c.g[V1][0] = 0x10;
            c.g[T1][0] = sx(rpc.wrapping_add(0x4d40));
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
            dirty_rpc_sw(c, rpc, DIRTY_RPC_TABLE_HEAD, c.g[V1][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_ID_LIMIT, c.g[V0][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_WORD_4DC8, c.g[T1][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_CLIENTS, c.g[T4][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_CLIENTS_END, c.g[T0][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_REQUESTS, c.g[T0][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_REQUESTS_END, c.g[A3][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_REPLIES, c.g[A3][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_REPLIES_END, c.g[A2][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_EE_BUFFER, c.g[A2][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_EE_BUFFER_END, c.g[T2][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_DMA_IDS, c.g[T5][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_DMA_IDS_END, c.g[T3][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_WAITERS, c.g[T3][0]);
            SCE_SIF_ALLOC_IOP_HEAP.call(c, 0x003f_5124)?;
            let rpc = r32(c, S1);
            let heap = c.g[V0][0] != 0;
            dirty_rpc_sw(c, rpc, DIRTY_RPC_IOP_HEAP, c.g[V0][0]);
            if !heap {
                c.cycles += 2;
                c.g[V0][0] = (-2i64) as u64;
                break 'out;
            }
            // The IOP addresses of the regions (packed into the heap block
            // after its 0x40-byte table) and the table itself on the stack.
            c.cycles += 70;
            c.g[T6][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_CLIENTS);
            c.g[T2][0] = c.g[V0][0] & c.g[S0][0];
            c.g[A2][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_CLIENTS_END);
            c.g[V0][0] = 0xffff_ffff_a000_0000;
            c.g[T5][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_REQUESTS);
            c.g[T2][0] |= c.g[V0][0];
            c.g[A1][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_REQUESTS_END);
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(r32(c, T6)));
            c.g[T4][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_REPLIES);
            c.g[A2][0] = sx((r32(c, A2) as i32 >> 4) as u32);
            c.g[V1][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_REPLIES_END);
            c.g[A1][0] = sx(r32(c, A1).wrapping_sub(r32(c, T5)));
            c.g[A0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_DMA_IDS);
            c.g[T1][0] = sx(r32(c, T2).wrapping_add(0x40));
            c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_DMA_IDS_END);
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(r32(c, T4)));
            c.g[A2][0] = sx(r32(c, A2) << 4);
            c.g[A1][0] = sx((r32(c, A1) as i32 >> 5) as u32);
            c.g[T3][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_EE_BUFFER);
            c.g[A2][0] = sx(r32(c, T1).wrapping_add(r32(c, A2)));
            c.g[A3][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_EE_BUFFER_END);
            c.g[A1][0] = sx(r32(c, A1) << 5);
            c.g[V1][0] = sx((r32(c, V1) as i32 >> 5) as u32);
            c.g[A1][0] = sx(r32(c, A2).wrapping_add(r32(c, A1)));
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, A0)));
            c.g[V1][0] = sx(r32(c, V1) << 5);
            c.g[V1][0] = sx(r32(c, A1).wrapping_add(r32(c, V1)));
            c.g[A3][0] = sx(r32(c, A3).wrapping_sub(r32(c, T3)));
            c.g[V0][0] = sx((r32(c, V0) as i32 >> 2) as u32);
            c.g[A0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_TABLE_HEAD);
            c.g[T0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_ID_LIMIT);
            c.g[A3][0] = sx(r32(c, V1).wrapping_add(r32(c, A3)));
            c.g[V0][0] = sx(r32(c, V0) << 2);
            let table = sp(c);
            dirty_rpc_sw(c, table, 0x0, c.g[A0][0]);
            c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
            dirty_rpc_sw(c, table, 0x4, c.g[T0][0]);
            dirty_rpc_sw(c, table, 0x3c, c.g[V0][0]);
            c.g[A0][0] = 0;
            dirty_rpc_sw(c, table, 0x10, c.g[T6][0]);
            dirty_rpc_sw(c, table, 0x1c, c.g[T5][0]);
            dirty_rpc_sw(c, table, 0x28, c.g[T4][0]);
            dirty_rpc_sw(c, table, 0x34, c.g[T3][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_IOP_CLIENTS, c.g[T1][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_IOP_REQUESTS, c.g[A2][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_IOP_REPLIES, c.g[A1][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_IOP_BUFFER, c.g[V1][0]);
            dirty_rpc_sw(c, table, 0x18, c.g[A1][0]);
            dirty_rpc_sw(c, table, 0x20, c.g[A1][0]);
            dirty_rpc_sw(c, table, 0x24, c.g[V1][0]);
            dirty_rpc_sw(c, table, 0x2c, c.g[V1][0]);
            dirty_rpc_sw(c, table, 0x8, c.g[T1][0]);
            dirty_rpc_sw(c, table, 0xc, c.g[A2][0]);
            dirty_rpc_sw(c, table, 0x14, c.g[A2][0]);
            dirty_rpc_sw(c, table, 0x30, c.g[A3][0]);
            dirty_rpc_sw(c, table, 0x38, c.g[A3][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_IOP_HEAP, c.g[T2][0]);
            FLUSH_CACHE.call(c, 0x003f_5218)?;
            // DMA the table to the heap block: { src, dest, size, attr } at
            // sp+0x40.
            let rpc = r32(c, S1);
            c.g[V1][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_IOP_HEAP);
            c.g[A0][0] = sx(frame.at(c, 0x40));
            c.g[V0][0] = 0x40;
            c.g[S2][0] = c.g[A0][0];
            Addr(frame.at(c, 0x44)).sw(c, c.g[V1][0]);
            c.g[A1][0] = 1;
            Addr(frame.at(c, 0x48)).sw(c, c.g[V0][0]);
            Addr(frame.at(c, 0x40)).sw(c, c.g[SP][0]);
            Addr(frame.at(c, 0x4c)).sw(c, 0);
            SCE_SIF_SET_DMA.call(c, 0x003f_5240)?;
            c.g[S0][0] = c.g[V0][0];
            let unqueued = c.g[S0][0] == 0;
            c.g[V0][0] = (-3i64) as u64;
            if unqueued {
                break 'out;
            }
            c.cycles += 1;
            loop {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                SCE_SIF_DMA_STAT.call(c, 0x003f_5258)?;
                let busy = (c.g[V0][0] as i64) >= 0;
                c.g[A2][0] = 0x200;
                if !busy {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003f_5250);
                }
            }
            // DMA the request slots to their IOP copy.
            c.cycles += 12;
            let rpc = r32(c, S1);
            c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_REQUESTS);
            c.g[V1][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_IOP_REQUESTS);
            c.g[A0][0] = c.g[S2][0];
            Addr(frame.at(c, 0x40)).sw(c, c.g[V0][0]);
            c.g[A1][0] = 1;
            Addr(frame.at(c, 0x44)).sw(c, c.g[V1][0]);
            Addr(frame.at(c, 0x48)).sw(c, c.g[A2][0]);
            Addr(frame.at(c, 0x4c)).sw(c, 0);
            SCE_SIF_SET_DMA.call(c, 0x003f_5284)?;
            c.g[S0][0] = c.g[V0][0];
            let queued = c.g[S0][0] != 0;
            c.g[S3][0] = sx(DIRTY_RPC_BOOT_CMD & 0xffff_0000);
            if !queued {
                c.cycles += 2;
                c.g[V0][0] = (-4i64) as u64;
                break 'out;
            }
            c.cycles += 2;
            c.g[S4][0] = sx(frame.at(c, 0x50));
            c.g[S5][0] = sx(frame.at(c, 0x70));
            loop {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                SCE_SIF_DMA_STAT.call(c, 0x003f_52a8)?;
                if (c.g[V0][0] as i64) < 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003f_52a0);
                }
            }
            // Boot packet with the heap address, ring words, semaphore and
            // thread.
            c.cycles += 57;
            c.g[S0][0] = sx(r32(c, S3).wrapping_add(0x1340));
            SCE_SIF_INIT_CMD.call(c, 0x003f_52b8)?;
            c.g[V0][0] = dirty_rpc_lw(c, r32(c, S1), DIRTY_RPC_IOP_HEAP);
            c.g[A0][0] = 0;
            c.g[S2][0] = 1;
            dirty_rpc_sw(c, r32(c, S0), DIRTY_RPC_BOOT_CMD_REPLY, c.g[V0][0]);
            FLUSH_CACHE.call(c, 0x003f_52cc)?;
            c.g[A2][0] = Addr(r32(c, S3).wrapping_add(0x1340)).lbu(c);
            c.g[T0][0] = 0;
            c.g[A3][0] = 0;
            c.g[T1][0] = 0;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = 1;
            SCE_SIF_SEND_CMD.call(c, 0x003f_52e8)?;
            let rpc = r32(c, S1);
            c.g[T0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_REQUESTS);
            c.g[A1][0] = 0;
            c.g[V1][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_REQUESTS_END);
            c.g[A2][0] = 0x18;
            c.g[A3][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_EE_BUFFER);
            c.g[A0][0] = c.g[S4][0];
            c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_EE_BUFFER_END);
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(r32(c, T0)));
            c.g[V1][0] = sx((r32(c, V1) as i32 >> DIRTY_RPC_SLOT_SHIFT) as u32);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_NEXT_ID, c.g[S2][0]);
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, A3)));
            dirty_rpc_sw(c, rpc, DIRTY_RPC_SLOTS, c.g[V1][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_BUFFER_SIZE, c.g[V0][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_TAIL, c.g[S2][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_HEAD, c.g[S2][0]);
            MEMSET.call(c, 0x003f_5328)?;
            // The semaphore parameters at sp+0x50: max and initial count 1.
            c.g[A0][0] = c.g[S4][0];
            Addr(frame.at(c, 0x58)).sw(c, c.g[S2][0]);
            Addr(frame.at(c, 0x54)).sw(c, c.g[S2][0]);
            Addr(frame.at(c, 0x64)).sw(c, 0);
            CREATE_SEMA.call(c, 0x003f_533c)?;
            // The thread parameters at sp+0x70.
            let rpc = r32(c, S1);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_SEMA, c.g[V0][0]);
            c.g[T0][0] = u64::MAX;
            c.g[V1][0] = sx(DIRTY_RPC_THREAD_MAIN);
            c.g[A1][0] = sx(DIRTY_RPC_THREAD_STACK);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_HANDLER_ALARM, c.g[T0][0]);
            c.g[A2][0] = DIRTY_RPC_THREAD_PRIORITY;
            c.g[A3][0] = DIRTY_RPC_THREAD_STACK_SIZE;
            c.g[V0][0] = sx(DIRTY_RPC_THREAD_GP);
            Addr(frame.at(c, 0x74)).sw(c, c.g[V1][0]);
            c.g[A0][0] = c.g[S5][0];
            Addr(frame.at(c, 0x84)).sw(c, c.g[A2][0]);
            Addr(frame.at(c, 0x78)).sw(c, c.g[A1][0]);
            Addr(frame.at(c, 0x7c)).sw(c, c.g[A3][0]);
            Addr(frame.at(c, 0x80)).sw(c, c.g[V0][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_ALARM, c.g[T0][0]);
            Addr(frame.at(c, 0x90)).sw(c, 0);
            CREATE_THREAD.call(c, 0x003f_538c)?;
            let rpc = r32(c, S1);
            let failed = (c.g[V0][0] as i64) <= 0;
            dirty_rpc_sw(c, rpc, DIRTY_RPC_THREAD, c.g[V0][0]);
            if failed {
                c.cycles += 1;
                c.g[V0][0] = (-5i64) as u64;
                break 'out;
            }
            c.cycles += 15;
            c.g[A1][0] = sx(DIRTY_RPC_INTC_HANDLER_FN);
            c.g[A2][0] = u64::MAX;
            c.g[A3][0] = c.g[S1][0];
            c.g[A0][0] = DIRTY_RPC_INTC_SIF;
            ADD_INTC_HANDLER.call(c, 0x003f_53ac)?;
            c.g[A0][0] = DIRTY_RPC_INTC_SIF;
            dirty_rpc_sw(c, r32(c, S1), DIRTY_RPC_INTC_HANDLER, c.g[V0][0]);
            ENABLE_INTC.call(c, 0x003f_53b8)?;
            c.g[A0][0] = dirty_rpc_lw(c, r32(c, S1), DIRTY_RPC_THREAD);
            c.g[A1][0] = c.g[S1][0];
            START_THREAD.call(c, 0x003f_53c4)?;
            dirty_rpc_sw(c, r32(c, GP), DIRTY_RPC_STARTED_GP, c.g[S2][0]);
            c.g[V0][0] = 0;
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x100);
        frame.restore(c, S1, 0xf0);
        frame.restore(c, S2, 0xe0);
        frame.restore(c, S3, 0xd0);
        frame.restore(c, S4, 0xc0);
        frame.restore(c, S5, 0xb0);
        frame.restore_ra(c, 0xa0);
        frame.close(c);
        Ok(())
    })
}

/// `int DirtyRpc_Shutdown(void)` (0x3f53f8): release the two alarms that
/// are set (ids >= 0), remove the SIF interrupt handler, terminate and delete
/// the transport thread, delete the ring semaphore and clear the started
/// flag. Returns 0.
pub fn dirty_rpc_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x52_0000;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(DIRTY_RPC_STATE);
        c.g[A0][0] = dirty_rpc_lw(c, r32(c, S0), DIRTY_RPC_ALARM);
        if (c.g[A0][0] as i64) >= 0 {
            c.cycles += 3;
            RELEASE_ALARM.call(c, 0x003f_5420)?;
        }
        c.g[A0][0] = dirty_rpc_lw(c, r32(c, S0), DIRTY_RPC_HANDLER_ALARM);
        c.cycles += 2;
        if (c.g[A0][0] as i64) >= 0 {
            c.cycles += 3;
            RELEASE_ALARM.call(c, 0x003f_5434)?;
        }
        c.g[A1][0] = dirty_rpc_lw(c, r32(c, S0), DIRTY_RPC_INTC_HANDLER);
        c.cycles += 14;
        c.g[A0][0] = DIRTY_RPC_INTC_SIF;
        REMOVE_INTC_HANDLER.call(c, 0x003f_5440)?;
        c.g[A0][0] = dirty_rpc_lw(c, r32(c, S0), DIRTY_RPC_THREAD);
        TERMINATE_THREAD.call(c, 0x003f_5448)?;
        c.g[A0][0] = dirty_rpc_lw(c, r32(c, S0), DIRTY_RPC_THREAD);
        DELETE_THREAD.call(c, 0x003f_5450)?;
        c.g[A0][0] = dirty_rpc_lw(c, r32(c, S0), DIRTY_RPC_SEMA);
        DELETE_SEMA.call(c, 0x003f_5458)?;
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = 0;
        frame.restore_ra(c, 0);
        dirty_rpc_sw(c, r32(c, GP), DIRTY_RPC_STARTED_GP, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int DirtyRpc_IsStarted(void)` (0x3f5470): the started flag
/// (`gp+0x1ac0`).
pub fn dirty_rpc_is_started(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = dirty_rpc_lw(c, r32(c, GP), DIRTY_RPC_STARTED_GP);
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule, Events};
    use crate::difftest::fakes::{
        fake_syscall, machine, mix, FAKE, LOG_COUNT, LOG_START, LOG_UNWIND_AT,
    };
    use crate::difftest::{
        check, check_paths, fill, fresh, put_words, run_generated, scramble, Rng, DATA, GAME_GP,
    };
    use std::collections::BTreeSet;

    // ------------------------------------------------ syscalls of the fixtures

    /// Control block of [`dirty_rpc_syscall`] in the test machine's RAM: the
    /// rule count, the cycles its clock rules added, and up to 8 rules of
    /// 0x20 bytes (syscall code, first and end call index of that code,
    /// kind, value, address, calls seen).
    pub(crate) const DIRTY_RPC_SYS: u32 = DATA + 0xf_0000;
    pub(crate) const DIRTY_RPC_SYS_RULES: u32 = DIRTY_RPC_SYS;
    pub(crate) const DIRTY_RPC_SYS_CLOCK: u32 = DIRTY_RPC_SYS + 4;
    pub(crate) const DIRTY_RPC_SYS_RULE0: u32 = DIRTY_RPC_SYS + 0x20;
    /// Rule kinds: force `v0` (sign-extended, so 0 and negative values too),
    /// write a word (an IOP reply arriving), move the clock on, or make an
    /// event due at the next check after the syscall wrapper's own.
    pub(crate) const DIRTY_RPC_SYS_RESULT: u32 = 1;
    pub(crate) const DIRTY_RPC_SYS_POKE: u32 = 2;
    pub(crate) const DIRTY_RPC_SYS_CLOCK_STEP: u32 = 3;
    pub(crate) const DIRTY_RPC_SYS_ARM: u32 = 4;
    /// Syscall codes (`v1`) of the kernel wrappers the transport calls.
    pub(crate) const DIRTY_RPC_SYS_FLUSH_CACHE: u32 = 0x64;
    pub(crate) const DIRTY_RPC_SYS_SET_DMA: u32 = 0x77;
    pub(crate) const DIRTY_RPC_SYS_DMA_STAT: u32 = 0x76;
    pub(crate) const DIRTY_RPC_SYS_CREATE_THREAD: u32 = 0x20;
    pub(crate) const DIRTY_RPC_SYS_WAIT_SEMA: u32 = 0x44;
    pub(crate) const DIRTY_RPC_SYS_SIGNAL_SEMA: u32 = 0x42;

    /// The syscall hook of the fixtures' machines: the logging fake
    /// (`fakes::fake_syscall`: log, pointer result, unwind at the chosen call),
    /// then the fixture's rules for this syscall code.
    pub(crate) fn dirty_rpc_syscall(c: &mut Cpu, at: u32) {
        let code = c.g[V1][0] as u32;
        fake_syscall(c, at);
        let rules = c.bus.read32(DIRTY_RPC_SYS_RULES).min(8);
        for i in 0..rules {
            let rule = DIRTY_RPC_SYS_RULE0 + 0x20 * i;
            if c.bus.read32(rule) != code {
                continue;
            }
            let n = c.bus.read32(rule + 0x18);
            c.bus.write32(rule + 0x18, n + 1);
            if n < c.bus.read32(rule + 0x4) || n >= c.bus.read32(rule + 0x8) {
                continue;
            }
            let value = c.bus.read32(rule + 0x10);
            match c.bus.read32(rule + 0xc) {
                DIRTY_RPC_SYS_RESULT => c.g[V0][0] = sx(value),
                DIRTY_RPC_SYS_POKE => {
                    let addr = c.bus.read32(rule + 0x14);
                    c.bus.write32(addr, value);
                }
                DIRTY_RPC_SYS_CLOCK_STEP => {
                    c.cycles += value as u64;
                    let total = c.bus.read32(DIRTY_RPC_SYS_CLOCK);
                    c.bus.write32(DIRTY_RPC_SYS_CLOCK, total.wrapping_add(value));
                }
                _ => c.next_event = c.cycles + 1,
            }
        }
    }

    /// Add a rule: for the calls `calls` (counted per rule) of syscall `code`.
    pub(crate) fn dirty_rpc_rule(
        c: &mut Cpu,
        code: u32,
        calls: std::ops::Range<u32>,
        kind: u32,
        value: u32,
        addr: u32,
    ) {
        let n = c.bus.read32(DIRTY_RPC_SYS_RULES);
        assert!(n < 8, "too many syscall rules");
        let rule = DIRTY_RPC_SYS_RULE0 + 0x20 * n;
        put_words(c, rule, &[code, calls.start, calls.end, kind, value, addr, 0, 0]);
        c.bus.write32(DIRTY_RPC_SYS_RULES, n + 1);
    }

    /// `fakes::machine` with [`dirty_rpc_syscall`] and the unwinding fake
    /// call chosen by the test (`u32::MAX`: none).
    pub(crate) fn dirty_rpc_machine(c: &mut Cpu, seed: u32, salt: u64, unwind_at: u32) -> Rng {
        let r = machine(c, seed, salt, 0);
        c.hooks.syscall = dirty_rpc_syscall;
        c.bus.write32(LOG_UNWIND_AT, unwind_at);
        r
    }

    /// The cycles a run charged outside the fakes: the total since `start`
    /// minus what each logged fake call added (`3 + hash & 7`, the hash
    /// recomputed from its log entry) and the clock rules' steps. With the
    /// fakes left out an arm's cost is a constant.
    pub(crate) fn dirty_rpc_own_cycles(c: &mut Cpu, start: u64) -> u64 {
        let calls = c.bus.read32(LOG_COUNT);
        let mut faked = c.bus.read32(DIRTY_RPC_SYS_CLOCK) as u64;
        for n in 0..calls {
            let entry = LOG_START + 0x220 * n;
            let at = c.bus.read64(entry + 8);
            let a0 = c.bus.read64(entry + 0x10 + 0x10 * A0 as u32);
            let a1 = c.bus.read64(entry + 0x10 + 0x10 * A1 as u32);
            let h = mix(a0 ^ (n as u64) << 40 ^ a1.rotate_left(17) ^ at);
            faked += 3 + (h & 7);
        }
        c.cycles - start - faked
    }

    /// `require_paths` on [`dirty_rpc_own_cycles`].
    pub(crate) fn dirty_rpc_require_paths(
        generated: GuestFn,
        seeds: u32,
        paths: &[u64],
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        let mut seen = BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            let start = c.cycles;
            run_generated(&mut c, generated);
            seen.insert(dirty_rpc_own_cycles(&mut c, start));
        }
        for p in paths {
            assert!(
                seen.contains(p),
                "path costing {p} cycles (fakes left out) never taken (seen {seen:?})"
            );
        }
    }

    /// `check` plus [`dirty_rpc_require_paths`].
    pub(crate) fn dirty_rpc_check_paths(
        generated: GuestFn,
        hand: GuestFn,
        seeds: u32,
        paths: &[u64],
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        check(generated, hand, seeds, setup);
        dirty_rpc_require_paths(generated, seeds, paths, setup);
    }

    /// Require that the generated function returns every value in `results`
    /// for some seed (arms told apart by their result).
    pub(crate) fn dirty_rpc_require_results(
        generated: GuestFn,
        seeds: u32,
        results: &[u64],
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        let mut seen = BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            run_generated(&mut c, generated);
            seen.insert(c.g[V0][0]);
        }
        for v in results {
            assert!(
                seen.contains(v),
                "result {v:#x} never returned (seen {seen:x?})"
            );
        }
    }

    // ------------------------------------------------ the rings

    /// Test layout of the rings: request slots, reply slots, waiter records,
    /// the data area (and its IOP address), result words.
    pub(crate) const DIRTY_RPC_T_REQUESTS: u32 = DATA + 0x1000;
    pub(crate) const DIRTY_RPC_T_REPLIES: u32 = DATA + 0x2000;
    pub(crate) const DIRTY_RPC_T_WAITERS: u32 = DATA + 0x3000;
    pub(crate) const DIRTY_RPC_T_DATA: u32 = DATA + 0x4000;
    pub(crate) const DIRTY_RPC_T_RESULTS: u32 = DATA + 0x5000;
    pub(crate) const DIRTY_RPC_T_DATA_IOP: u32 = 0x000c_0000;
    pub(crate) const DIRTY_RPC_T_SLOTS: u32 = 4;

    pub(crate) fn dirty_rpc_t_request(i: u32) -> u32 {
        DIRTY_RPC_T_REQUESTS + (i << DIRTY_RPC_SLOT_SHIFT)
    }

    pub(crate) fn dirty_rpc_t_reply(i: u32) -> u32 {
        DIRTY_RPC_T_REPLIES + (i << DIRTY_RPC_SLOT_SHIFT)
    }

    pub(crate) fn dirty_rpc_t_waiter(i: u32) -> u32 {
        DIRTY_RPC_T_WAITERS + (i << DIRTY_RPC_WAITER_SHIFT)
    }

    /// Random state words, then the ring words and four slots: `pending`
    /// replies from `read` on, each answering its request (equal seq
    /// words), nobody waiting, no data; everything else in the slots random.
    pub(crate) fn dirty_rpc_ring_fixture(c: &mut Cpu, r: &mut Rng, read: u32, pending: u32) {
        let slots = DIRTY_RPC_T_SLOTS;
        fill(c, r, DIRTY_RPC_STATE + DIRTY_RPC_TABLE_HEAD, 0x24);
        for (off, v) in [
            (DIRTY_RPC_REQUESTS, DIRTY_RPC_T_REQUESTS),
            (DIRTY_RPC_REPLIES, DIRTY_RPC_T_REPLIES),
            (DIRTY_RPC_WAITERS, DIRTY_RPC_T_WAITERS),
            (DIRTY_RPC_EE_BUFFER, DIRTY_RPC_T_DATA),
            (DIRTY_RPC_IOP_BUFFER, DIRTY_RPC_T_DATA_IOP),
            (DIRTY_RPC_TAIL, read),
            (DIRTY_RPC_HEAD, (read + pending) % slots),
            (DIRTY_RPC_SLOTS, slots),
        ] {
            c.bus.write32(DIRTY_RPC_STATE + off, v);
        }
        fill(c, r, DIRTY_RPC_T_REQUESTS, slots * 8);
        fill(c, r, DIRTY_RPC_T_REPLIES, slots * 8);
        fill(c, r, DIRTY_RPC_T_WAITERS, slots * 4);
        fill(c, r, DIRTY_RPC_T_RESULTS, slots);
        for i in 0..slots {
            let seq = 0x5e00 + 0x11 * i;
            c.bus.write32(dirty_rpc_t_request(i) + DIRTY_RPC_SLOT_SEQ, seq);
            c.bus.write32(dirty_rpc_t_reply(i) + DIRTY_RPC_SLOT_SEQ, seq);
            c.bus.write32(dirty_rpc_t_request(i) + DIRTY_RPC_REQUEST_SIZE, 0);
            c.bus.write32(dirty_rpc_t_waiter(i) + DIRTY_RPC_WAITER_KIND, 0);
        }
    }

    /// Make the reply in slot `i` not answer its request.
    pub(crate) fn dirty_rpc_t_unanswer(c: &mut Cpu, i: u32) {
        let seq = dirty_rpc_t_reply(i) + DIRTY_RPC_SLOT_SEQ;
        let v = c.bus.read32(seq);
        c.bus.write32(seq, v ^ 0x100);
    }

    // ------------------------------------------------ DirtyRpc_AlarmHandler

    /// `seed % 3`: the alarm that fired is the first (Send's), the handler alarm or
    /// neither; `/3 % 2`: a reply pending; `/6 % 2`: it answers its request
    /// (wake the thread); `/12 % 4`: the alarm ids' signs (bit 0 first, bit 1
    /// poll; both negative re-arms); `/48 % 2`: a fake call unwinds.
    /// Paths (fakes left out): 16/19 nothing pending, 25-42 pending, +22 with
    /// the wake-up (iWakeupThread's own 19), +11 re-armed.
    #[test]
    fn dirty_rpc_alarm_handler_matches() {
        const PATHS: &[u64] = &[16, 19, 25, 28, 31, 39, 42, 47, 50, 53, 61, 64];
        dirty_rpc_check_paths(
            ee::s14::f_003f4448__generated,
            dirty_rpc_alarm_handler,
            96,
            PATHS,
            &|c, seed| {
                let fired = seed % 3;
                let pending = seed / 3 % 2;
                let answered = seed / 6 % 2 == 1;
                let signs = seed / 12 % 4;
                let unwind = if seed / 48 % 2 == 1 { seed % 3 } else { u32::MAX };
                let mut r = dirty_rpc_machine(c, seed, 0x3f_4448, unwind);
                c.pc = 0x003f_4448;
                let read = r.next_u32() % DIRTY_RPC_T_SLOTS;
                dirty_rpc_ring_fixture(c, &mut r, read, pending);
                if !answered {
                    dirty_rpc_t_unanswer(c, read);
                }
                let alarm = if signs & 1 != 0 { 0xffff_ff00 + seed } else { 0x100 + seed };
                let poll = if signs & 2 != 0 { 0xffff_fe00 + seed } else { 0x200 + seed };
                c.bus.write32(DIRTY_RPC_STATE + DIRTY_RPC_ALARM, alarm);
                c.bus.write32(DIRTY_RPC_STATE + DIRTY_RPC_HANDLER_ALARM, poll);
                c.bus.write32(DIRTY_RPC_STATE + DIRTY_RPC_THREAD, 0x20 + seed % 8);
                c.g[A0][0] = sx(match fired {
                    0 => alarm,
                    1 => poll,
                    _ => 0x7777_0000 + seed,
                });
                c.g[A2][0] = sx(DIRTY_RPC_STATE);
            },
        );
    }

    // ------------------------------------------------ DirtyRpc_ThreadMain

    /// The thread never returns: the fake call `seed % 5` unwinds (a
    /// SleepThread, or a semaphore call inside ProcessReplies); `/5 % 3`
    /// replies are pending for the first round.
    #[test]
    fn dirty_rpc_thread_main_matches() {
        check(
            ee::s14::f_003f44f8__generated,
            dirty_rpc_thread_main,
            45,
            &|c, seed| {
                let mut r = dirty_rpc_machine(c, seed, 0x3f_44f8, seed % 5);
                c.pc = 0x003f_44f8;
                let read = r.next_u32() % DIRTY_RPC_T_SLOTS;
                dirty_rpc_ring_fixture(c, &mut r, read, seed / 5 % 3);
                fill(c, &mut r, GAME_GP + DIRTY_RPC_SLEEPING_GP, 1);
                fill(c, &mut r, GAME_GP + DIRTY_RPC_SEMA_WAIT_GP, 1);
            },
        );
    }

    // ------------------------------------------------ DirtyRpc_ProcessReplies

    /// `seed % 4` replies pending; `/4 % 4` the first one's waiter (nobody,
    /// result word with no pointer, result word stored, callback; the next
    /// ones rotate); `/16 % 2` the result waiters name a thread to wake;
    /// `/32 % 2` the first request has data; `/64 % 4`: all answered, the
    /// last one not answered, the first one changes while waiting for the
    /// semaphore (a WaitSema rule), or all answered with a fake call
    /// unwinding (`seed % 5`).
    pub(crate) fn dirty_rpc_process_replies_fixture(c: &mut Cpu, seed: u32) {
        let pending = seed % 4;
        let first_waiter = seed / 4 % 4;
        let wake = seed / 16 % 2 == 1;
        let sized = seed / 32 % 2 == 1;
        let ending = seed / 64 % 4;
        let unwind = if ending == 3 { seed % 5 } else { u32::MAX };
        let mut r = dirty_rpc_machine(c, seed, 0x3f_4a78, unwind);
        c.pc = 0x003f_4a78;
        let read = r.next_u32() % DIRTY_RPC_T_SLOTS;
        dirty_rpc_ring_fixture(c, &mut r, read, pending);
        for j in 0..pending {
            let i = (read + j) % DIRTY_RPC_T_SLOTS;
            let waiter = dirty_rpc_t_waiter(i);
            let (kind, result) = match (first_waiter + j) % 4 {
                0 => (0, 0),
                1 => (1, 0),
                2 => (1, DIRTY_RPC_T_RESULTS + 4 * i),
                _ => (FAKE + 0x10 * j, r.next_u32()),
            };
            put_words(c, waiter, &[kind, result]);
            c.bus
                .write32(waiter + DIRTY_RPC_WAITER_THREAD, if wake { 0x30 + i } else { 0 });
        }
        if sized && pending > 0 {
            c.bus.write32(
                dirty_rpc_t_request(read) + DIRTY_RPC_REQUEST_SIZE,
                0x41 + seed % 0x40,
            );
        }
        match ending {
            1 if pending > 0 => {
                dirty_rpc_t_unanswer(c, (read + pending - 1) % DIRTY_RPC_T_SLOTS);
            }
            2 => {
                let seq = dirty_rpc_t_reply(read) + DIRTY_RPC_SLOT_SEQ;
                let changed = c.bus.read32(seq) ^ 0x100;
                dirty_rpc_rule(
                    c,
                    DIRTY_RPC_SYS_WAIT_SEMA,
                    0..1,
                    DIRTY_RPC_SYS_POKE,
                    changed,
                    seq,
                );
            }
            _ => {}
        }
        fill(c, &mut r, GAME_GP + DIRTY_RPC_SEMA_WAIT_GP, 1);
    }

    /// Paths (fakes left out): 20 empty, 28 first not answered, 57 changed
    /// while waiting, one reply: 92 nobody, 101 with data, 99 callback, 102
    /// result word, 109 result word and wake-up.
    #[test]
    fn dirty_rpc_process_replies_matches() {
        const PATHS: &[u64] = &[20, 28, 57, 92, 99, 101, 102, 109];
        dirty_rpc_check_paths(
            ee::s14::f_003f4a78__generated,
            dirty_rpc_process_replies,
            256,
            PATHS,
            &dirty_rpc_process_replies_fixture,
        );
    }

    /// Three answered replies; a rule makes the event due right after the
    /// first (`seed / 3 % 2` = 0) or second SignalSema's own check, so it
    /// lands on the back edge 0x3f4ad8 (structured: delivered there, no
    /// resume).
    #[test]
    fn dirty_rpc_process_replies_events() {
        driven!(gen, ee::s14::f_003f4a78__generated);
        driven!(hand, dirty_rpc_process_replies);
        check_events(gen, hand, 48, &|c, seed| {
            let mut r = dirty_rpc_machine(c, seed, 0x3f_4a79, u32::MAX);
            let read = seed / 6 % DIRTY_RPC_T_SLOTS;
            dirty_rpc_ring_fixture(c, &mut r, read, 3);
            let mode = schedule(c, &mut r, seed, 0x003f_4a78);
            if mode != Events::Off {
                c.next_event = u64::MAX;
                let after = seed / 3 % 2;
                dirty_rpc_rule(
                    c,
                    DIRTY_RPC_SYS_SIGNAL_SEMA,
                    after..after + 1,
                    DIRTY_RPC_SYS_ARM,
                    0,
                    0,
                );
            }
            mode
        });
    }

    // ------------------------------------------------ DirtyRpc_Call

    /// DirtyRpc_Send's early -1 returns (`seed % 3`): no request (36 cycles),
    /// a request whose +0xc word is 0 (39), a client table start that is not
    /// 16-byte aligned (43); Call adds 12. The caller's stack words are
    /// random.
    #[test]
    fn dirty_rpc_call_matches() {
        check_paths(
            ee::s14::f_003f4c68__generated,
            dirty_rpc_call,
            30,
            &[48, 51, 55],
            &|c, seed| {
                let mut r = Rng(0x3f_4c68 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.pc = 0x003f_4c68;
                let req = DATA + 0x100;
                fill(c, &mut r, req, 8);
                fill(c, &mut r, crate::difftest::SP, 4);
                match seed % 3 {
                    0 => c.g[A0][0] = 0,
                    1 => {
                        c.g[A0][0] = sx(req);
                        c.bus.write32(req + 0xc, 0);
                    }
                    _ => {
                        c.g[A0][0] = sx(req);
                        c.bus.write32(req + 0xc, 1 + seed);
                        c.g[T2][0] = sx(0x2051_f0c0 | (1 + seed % 15));
                    }
                }
            },
        );
    }

    // ------------------------------------------------ DirtyRpc_InitRequest

    pub(crate) const DIRTY_RPC_T_CLIENTS: u32 = DATA + 0x6000;
    pub(crate) const DIRTY_RPC_T_REQUEST: u32 = DATA + 0x100;

    /// `cached`: how the header's cached binding fails (0 other id, 1 other
    /// key, 2 negative index, 3 index 129 or more, 4 the entry's +0xc word
    /// differs, 5 the entry's id differs) or holds (6). `scan` over `entries`
    /// table entries: 0 empty table, 1 the first entry matches, 2 a key
    /// mismatch then a match at the third, 3 a key mismatch and no match, 4
    /// no entry with the id.
    pub(crate) fn dirty_rpc_init_request_fixture(
        c: &mut Cpu,
        seed: u32,
        cached: u32,
        scan: u32,
        entries: u32,
    ) {
        let mut r = Rng(0x3f_4c98 ^ (seed as u64) << 4);
        scramble(c, &mut r);
        c.g[GP][0] = sx(GAME_GP);
        c.pc = 0x003f_4c98;
        let id = 0x1234_0000 + seed;
        let key = 0x77 + seed;
        let req = DIRTY_RPC_T_REQUEST;
        let table = DIRTY_RPC_T_CLIENTS;
        fill(c, &mut r, table, 4 * (entries + 8));
        for e in 0..entries {
            put_words(c, table + 0x10 * e, &[e + 3, 0x5555_0000 + e]);
        }
        let entry = |e: u32| table + 0x10 * e;
        match scan {
            1 => put_words(c, entry(0) + 4, &[id, key]),
            2 => {
                put_words(c, entry(1) + 4, &[id, key ^ 1]);
                put_words(c, entry(2) + 4, &[id, key]);
            }
            3 => put_words(c, entry(1) + 4, &[id, key ^ 1]),
            _ => {}
        }
        let end = if scan == 0 { table } else { entry(entries) };
        c.bus.write32(DIRTY_RPC_STATE + DIRTY_RPC_CLIENTS, table);
        c.bus.write32(DIRTY_RPC_STATE + DIRTY_RPC_CLIENTS_END, end);
        // The header and the (usable) entry its cached index names, past
        // the scanned ones.
        let index = entries + 4 + seed % 4;
        let handle = r.next_u32();
        put_words(c, req, &[index, id, key, handle]);
        let named = entry(index);
        put_words(c, named, &[index, id, r.next_u32(), handle]);
        match cached {
            0 => c.bus.write32(req + DIRTY_RPC_CLIENT_SERVER, id ^ 0x10),
            1 => c.bus.write32(req + DIRTY_RPC_CLIENT_KEY, key ^ 1),
            2 => c.bus.write32(req + DIRTY_RPC_CLIENT_INDEX, 0x8000_0000 | seed),
            3 => c.bus.write32(req + DIRTY_RPC_CLIENT_INDEX, 129 + seed % 0x100),
            4 => c.bus.write32(named + DIRTY_RPC_CLIENT_ID, handle ^ 1),
            5 => c.bus.write32(named + DIRTY_RPC_CLIENT_SERVER, id ^ 0x20),
            _ => {}
        }
        c.g[A0][0] = sx(req);
        c.g[A1][0] = sx(id);
        c.g[A2][0] = sx(key);
    }

    /// `seed % 7` the cached binding, `/7 % 4` the scan (4 entries). Paths:
    /// the checks before the scan cost 7, 9, 13, 15, 20, 23 (25 when the
    /// binding holds), the scan 9 (empty), 18, 31, 32.
    #[test]
    fn dirty_rpc_init_request_matches() {
        let mut paths = vec![25];
        for before in [7, 9, 13, 15, 20, 23] {
            for scan in [9, 18, 31, 32] {
                paths.push(before + scan);
            }
        }
        check_paths(
            ee::s14::f_003f4c98__generated,
            dirty_rpc_init_request,
            56,
            &paths,
            &|c, seed| {
                dirty_rpc_init_request_fixture(c, seed, seed % 7, seed / 7 % 4, 4);
            },
        );
    }

    /// Sixteen entries, none with the id: the event lands on the scan's back
    /// edge 0x3f4d30 (structured; leaf).
    #[test]
    fn dirty_rpc_init_request_events() {
        driven!(gen, ee::s14::f_003f4c98__generated);
        driven!(hand, dirty_rpc_init_request);
        check_events(gen, hand, 60, &|c, seed| {
            dirty_rpc_init_request_fixture(c, seed, 0, 4, 16);
            let mut r = Rng(0x3f_4d30 ^ seed as u64);
            schedule(c, &mut r, seed, 0x003f_4c98)
        });
    }

    // ------------------------------------------------ DirtyRpc_InitIop

    /// `pending` answered replies (nobody waiting, no data); a client table
    /// start that is not 16-byte aligned, so DirtyRpc_Send gives up at once.
    pub(crate) fn dirty_rpc_init_iop_fixture(c: &mut Cpu, seed: u32, pending: u32, unwind: u32) -> Rng {
        let mut r = dirty_rpc_machine(c, seed, 0x3f_4d68, unwind);
        c.pc = 0x003f_4d68;
        let read = r.next_u32() % DIRTY_RPC_T_SLOTS;
        dirty_rpc_ring_fixture(c, &mut r, read, pending);
        c.bus.write32(DIRTY_RPC_STATE + DIRTY_RPC_CLIENTS, 0x2051_f0c4);
        fill(c, &mut r, GAME_GP + DIRTY_RPC_SEMA_WAIT_GP, 1);
        r
    }

    /// `seed % 3` replies pending (paths 83 none, 181 one poll taking one,
    /// 250 one poll taking two; fakes left out); `/3 % 2` a fake call
    /// unwinds (`/6 % 4`).
    #[test]
    fn dirty_rpc_init_iop_matches() {
        dirty_rpc_check_paths(
            ee::s14::f_003f4d68__generated,
            dirty_rpc_init_iop,
            48,
            &[83, 181, 250],
            &|c, seed| {
                let unwind = if seed / 3 % 2 == 1 { seed / 6 % 4 } else { u32::MAX };
                dirty_rpc_init_iop_fixture(c, seed, seed % 3, unwind);
            },
        );
    }

    /// One or two replies pending (`seed / 3 % 2`): the first back edge
    /// (0x3f4dc0, a resume entry) comes 79 cycles in, before any call that
    /// checks events, so every event seed lands there.
    #[test]
    fn dirty_rpc_init_iop_events() {
        driven!(gen, ee::s14::f_003f4d68__generated);
        driven!(hand, dirty_rpc_init_iop);
        check_events(gen, hand, 60, &|c, seed| {
            let mut r = dirty_rpc_init_iop_fixture(c, seed, 1 + seed / 3 % 2, u32::MAX);
            schedule(c, &mut r, seed, 0x003f_4d68)
        });
    }

    // ------------------------------------------------ DirtyRpc_SyncDCache

    #[test]
    fn dirty_rpc_sync_dcache_matches() {
        check(
            ee::s14::f_003f4e80__generated,
            dirty_rpc_sync_dcache,
            32,
            &|c, seed| {
                let mut r = Rng(0x3f_4e80 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.pc = 0x003f_4e80;
                c.g[A0][0] = sx(DATA + 0x24 * seed);
                c.g[A1][0] = sx(0x40 * (seed % 9) + seed % 5);
            },
        );
    }

    // ------------------------------------------------ NetGameLink_CheckRecvBuf

    /// `seed % 8`: no next record (10 cycles), negative length (13), length
    /// over 0x8000 (16), next record misplaced (25), not linked back (30),
    /// sequence words differ (32), valid with no length pointer and with one
    /// (34); `/8 % 4` a valid length (0, 1, 0x7ff1, 0x8000); `/32 % 3` the
    /// segment bits of the pointers.
    #[test]
    fn net_game_link_check_recv_buf_matches() {
        check_paths(
            ee::s14::f_003f4ea0__generated,
            net_game_link_check_recv_buf,
            96,
            &[10, 13, 16, 25, 30, 32, 34],
            &|c, seed| {
                let mut r = Rng(0x3f_4ea0 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_4ea0;
                let arm = seed % 8;
                let digit = (seed / 32 % 3) as usize;
                let segment = [0, 0x2000_0000, 0x3000_0000][digit];
                let link = |p: u32| p | [0x2000_0000, 0, 0x3000_0000][digit];
                let buf = DATA + 0x100;
                let out = DATA + 0xa000;
                fill(c, &mut r, out, 1);
                let len: u32 = match arm {
                    1 => 0x8000_0000 | seed,
                    2 => 0x8001 + seed,
                    _ => [0, 1, 0x7ff1, 0x8000][(seed / 8 % 4) as usize],
                };
                let seq = r.next_u32();
                match arm {
                    0 => {
                        let w = r.next_u32();
                        put_words(c, buf, &[0, w, seq, len]);
                    }
                    1 | 2 => {
                        let w = r.next_u32();
                        put_words(c, buf, &[link(buf + 0x40), w, seq, len]);
                    }
                    _ => {
                        let next = buf + 0x10 + (len.wrapping_add(0xf) & 0x7fff_fff0);
                        let at = if arm == 3 { next + 0x10 } else { next };
                        let w = r.next_u32();
                        put_words(c, buf, &[link(at), w, seq, len]);
                        fill(c, &mut r, next, 4);
                        let back = if arm == 4 { buf + 0x20 } else { buf };
                        let next_seq = if arm == 5 { seq ^ 1 } else { seq };
                        let w = r.next_u32();
                        put_words(c, next, &[link(back), w, next_seq]);
                    }
                }
                c.g[A0][0] = sx(buf | segment);
                c.g[A1][0] = sx(if arm == 6 { 0 } else { out });
            },
        );
    }

    // ------------------------------------------------ DirtyRpc_WaitIop

    /// Random boot reply word; a random boot packet of size 0, so
    /// sceSifSendCmd sends nothing (41 cycles).
    pub(crate) fn dirty_rpc_wait_iop_fixture(c: &mut Cpu, seed: u32, unwind: u32) -> Rng {
        let mut r = dirty_rpc_machine(c, seed, 0x3f_4f40, unwind);
        c.pc = 0x003f_4f40;
        fill(c, &mut r, DIRTY_RPC_STATE, 1);
        fill(c, &mut r, DIRTY_RPC_BOOT_CMD, 4);
        c.bus.write8(DIRTY_RPC_BOOT_CMD, 0);
        r
    }

    /// `seed % 4`: the IOP answers (a FlushCache rule sets the reply byte)
    /// at the first flush (82 cycles), the first poll (96), the second
    /// (110), or never: each flush then moves the clock 20M cycles on and
    /// the third poll times out (131; events off, past the cycle budget).
    /// `/4 % 2` a fake call unwinds (`/8 % 4`).
    #[test]
    fn dirty_rpc_wait_iop_matches() {
        dirty_rpc_check_paths(
            ee::s14::f_003f4f40__generated,
            dirty_rpc_wait_iop,
            64,
            &[82, 96, 110, 131],
            &|c, seed| {
                let arm = seed % 4;
                let unwind = if seed / 4 % 2 == 1 { seed / 8 % 4 } else { u32::MAX };
                dirty_rpc_wait_iop_fixture(c, seed, unwind);
                if arm < 3 {
                    dirty_rpc_rule(
                        c,
                        DIRTY_RPC_SYS_FLUSH_CACHE,
                        arm..arm + 1,
                        DIRTY_RPC_SYS_POKE,
                        1,
                        DIRTY_RPC_STATE + DIRTY_RPC_BOOT_REPLY,
                    );
                } else {
                    dirty_rpc_rule(
                        c,
                        DIRTY_RPC_SYS_FLUSH_CACHE,
                        0..u32::MAX,
                        DIRTY_RPC_SYS_CLOCK_STEP,
                        20_000_000,
                        0,
                    );
                    c.next_event = u64::MAX;
                }
            },
        );
    }

    /// The IOP answers at flush `seed / 3 % 3 + 1`; a rule makes the event
    /// due right after flush `seed / 3 % 3` (its wrapper's own check sees
    /// none), so the first delivery is on the poll head 0x3f4fb0 (a resume
    /// entry), after 0, 1 or 2 polls.
    #[test]
    fn dirty_rpc_wait_iop_events() {
        driven!(gen, ee::s14::f_003f4f40__generated);
        driven!(hand, dirty_rpc_wait_iop);
        check_events(gen, hand, 54, &|c, seed| {
            let polls = seed / 3 % 3;
            let mut r = dirty_rpc_wait_iop_fixture(c, seed, u32::MAX);
            dirty_rpc_rule(
                c,
                DIRTY_RPC_SYS_FLUSH_CACHE,
                polls + 1..polls + 2,
                DIRTY_RPC_SYS_POKE,
                1,
                DIRTY_RPC_STATE + DIRTY_RPC_BOOT_REPLY,
            );
            let mode = schedule(c, &mut r, seed, 0x003f_4f40);
            if mode != Events::Off {
                c.next_event = u64::MAX;
                dirty_rpc_rule(
                    c,
                    DIRTY_RPC_SYS_FLUSH_CACHE,
                    polls..polls + 1,
                    DIRTY_RPC_SYS_ARM,
                    0,
                    0,
                );
            }
            mode
        });
    }

    // ------------------------------------------------ DirtyRpc_Startup

    /// The word `sceSifAllocIopHeap` checks first (negative: it returns 0 at
    /// once), libkernel's SIF RPC packet pool (`+4` packets, `+8` count), the
    /// receive word the IOP heap reply lands in, and `sceSifInitCmd`'s done
    /// flag (set: it returns at once).
    pub(crate) const DIRTY_RPC_T_IOP_HEAP_SERVER: u32 = 0x0045_5224;
    pub(crate) const DIRTY_RPC_T_RPC_PACKETS: u32 = 0x0052_d680;
    pub(crate) const DIRTY_RPC_T_IOP_HEAP_REPLY: u32 = 0x0052_ef00;
    pub(crate) const DIRTY_RPC_T_SIF_CMD_DONE: u32 = 0x0045_517c;

    /// `seed % 6` the arm: the IOP never answers (-1), no IOP heap (-2), the
    /// first or second DMA is not queued (-3, -4), CreateThread fails (-5;
    /// 0 or negative by `/6 % 2`), started (0). `/6 % 3` and `/18 % 3`: how
    /// often each DMA reports busy first. `/54 % 2`: fake call `seed % 17`
    /// unwinds. The heap comes through the real sceSifAllocIopHeap /
    /// sceSifCallRpc path (one free packet, fake semaphores and DMA).
    pub(crate) fn dirty_rpc_startup_fixture(c: &mut Cpu, seed: u32) {
        let arm = seed % 6;
        let first_busy = seed / 6 % 3;
        let second_busy = seed / 18 % 3;
        let unwind = if seed / 54 % 2 == 1 { seed % 17 } else { u32::MAX };
        let mut r = dirty_rpc_machine(c, seed, 0x3f_5000, unwind);
        c.pc = 0x003f_5000;
        fill(c, &mut r, DIRTY_RPC_STATE + DIRTY_RPC_TABLE_HEAD, 0x24);
        fill(c, &mut r, DIRTY_RPC_BOOT_CMD, 4);
        c.bus.write8(DIRTY_RPC_BOOT_CMD, 0);
        if arm == 0 {
            dirty_rpc_rule(
                c,
                DIRTY_RPC_SYS_FLUSH_CACHE,
                0..u32::MAX,
                DIRTY_RPC_SYS_CLOCK_STEP,
                20_000_000,
                0,
            );
            c.next_event = u64::MAX;
        } else {
            dirty_rpc_rule(
                c,
                DIRTY_RPC_SYS_FLUSH_CACHE,
                0..1,
                DIRTY_RPC_SYS_POKE,
                1,
                DIRTY_RPC_STATE + DIRTY_RPC_BOOT_REPLY,
            );
        }
        c.bus
            .write32(DIRTY_RPC_T_IOP_HEAP_SERVER, if arm == 1 { 0xffff_ffff } else { 0 });
        let packet = DATA + 0x6000;
        fill(c, &mut r, packet, 0x10);
        c.bus.write32(packet + 0x10, 0);
        let counter = r.next_u32();
        put_words(c, DIRTY_RPC_T_RPC_PACKETS, &[counter, packet, 1]);
        c.bus
            .write32(DIRTY_RPC_T_IOP_HEAP_REPLY, 0x000a_0000 + 0x40 * seed);
        c.bus.write32(DIRTY_RPC_T_SIF_CMD_DONE, 1);
        // sceSifSetDma call 0 is sceSifCallRpc's; 1 and 2 are the startup's.
        if arm == 2 || arm == 3 {
            dirty_rpc_rule(
                c,
                DIRTY_RPC_SYS_SET_DMA,
                arm - 1..arm,
                DIRTY_RPC_SYS_RESULT,
                0,
                0,
            );
        }
        // sceSifDmaStat: busy (the fakes' pointer result) until -1.
        dirty_rpc_rule(
            c,
            DIRTY_RPC_SYS_DMA_STAT,
            first_busy..first_busy + 1,
            DIRTY_RPC_SYS_RESULT,
            0xffff_ffff,
            0,
        );
        dirty_rpc_rule(
            c,
            DIRTY_RPC_SYS_DMA_STAT,
            first_busy + 1 + second_busy..u32::MAX,
            DIRTY_RPC_SYS_RESULT,
            0xffff_ffff,
            0,
        );
        if arm == 4 {
            let failed = if seed / 6 % 2 == 0 { 0 } else { 0xffff_fff0 };
            dirty_rpc_rule(
                c,
                DIRTY_RPC_SYS_CREATE_THREAD,
                0..u32::MAX,
                DIRTY_RPC_SYS_RESULT,
                failed,
                0,
            );
        }
        fill(c, &mut r, GAME_GP + DIRTY_RPC_STARTED_GP, 1);
    }

    #[test]
    fn dirty_rpc_startup_matches() {
        let results: Vec<u64> = [-1i64, -2, -3, -4, -5, 0].iter().map(|&v| v as u64).collect();
        check(
            ee::s14::f_003f5000__generated,
            dirty_rpc_startup,
            108,
            &dirty_rpc_startup_fixture,
        );
        dirty_rpc_require_results(ee::s14::f_003f5000__generated, 108, &results, &dirty_rpc_startup_fixture);
    }

    // ------------------------------------------------ DirtyRpc_Shutdown

    /// `seed % 2` / `/2 % 2`: the first / handler alarm is set (released) or
    /// negative (paths 40, 47, 54; fakes left out); `/4 % 2` a fake call
    /// unwinds (`/8 % 6`).
    #[test]
    fn dirty_rpc_shutdown_matches() {
        dirty_rpc_check_paths(
            ee::s14::f_003f53f8__generated,
            dirty_rpc_shutdown,
            48,
            &[40, 47, 54],
            &|c, seed| {
                let unwind = if seed / 4 % 2 == 1 { seed / 8 % 6 } else { u32::MAX };
                let mut r = dirty_rpc_machine(c, seed, 0x3f_53f8, unwind);
                c.pc = 0x003f_53f8;
                fill(c, &mut r, DIRTY_RPC_STATE + DIRTY_RPC_TABLE_HEAD, 0x24);
                let alarm = if seed % 2 == 0 { 0x10 + seed } else { 0x8000_0000 | seed };
                let poll = if seed / 2 % 2 == 0 { 0x20 + seed } else { 0xffff_ffff };
                c.bus.write32(DIRTY_RPC_STATE + DIRTY_RPC_ALARM, alarm);
                c.bus.write32(DIRTY_RPC_STATE + DIRTY_RPC_HANDLER_ALARM, poll);
                fill(c, &mut r, GAME_GP + DIRTY_RPC_STARTED_GP, 1);
            },
        );
    }

    // ------------------------------------------------ DirtyRpc_IsStarted

    #[test]
    fn dirty_rpc_is_started_matches() {
        check(
            ee::s14::f_003f5470__generated,
            dirty_rpc_is_started,
            16,
            &|c, seed| {
                let mut r = Rng(0x3f_5470 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.pc = 0x003f_5470;
                let flag = if seed % 2 == 0 { 0 } else { r.next_u32() };
                c.bus.write32(GAME_GP + DIRTY_RPC_STARTED_GP, flag);
            },
        );
    }
}
