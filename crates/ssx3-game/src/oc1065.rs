//! Staging module oc1065 (wave oc1065): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1065`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Flow, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003f_5478, proto_aries_create, "ProtoAriesCreate"),
    (0x003f_55b8, proto_aries_destroy, "ProtoAriesDestroy"),
    (0x003f_5638, proto_aries_connect, "ProtoAriesConnect"),
    (0x003f_5738, proto_aries_unconnect, "ProtoAriesUnconnect"),
    (0x003f_57c8, proto_aries_status, "ProtoAriesStatus"),
    (0x003f_5908, proto_aries_nop, "ProtoAries_Nop"),
    (0x003f_5910, proto_aries_send, "ProtoAriesSend"),
    (0x003f_5a70, proto_aries_peek, "ProtoAriesPeek"),
    (0x003f_5c38, proto_aries_recv, "ProtoAriesRecv"),
    (0x003f_5d88, proto_aries_tick, "ProtoAriesTick"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub const NET_RPC_ALLOC: Callee = Callee {
        addr: 0x0025_6bb0,
        f: ee::s06::f_00256bb0,
    };

    pub const NET_RPC_FREE: Callee = Callee {
        addr: 0x0025_6bd8,
        f: ee::s06::f_00256bd8,
    };

    pub const DIRTY_RPC_CALL: Callee = Callee {
        addr: 0x003f_4c68,
        f: ee::s14::DirtyRpc_Call,
    };

    pub const DIRTY_RPC_INIT_REQUEST: Callee = Callee {
        addr: 0x003f_4c98,
        f: ee::s14::DirtyRpc_InitRequest,
    };

    pub const NET_TICK: Callee = Callee {
        addr: 0x003f_4de8,
        f: ee::s14::NetTick,
    };

    pub const PROTO_ARIES_NOP: Callee = Callee {
        addr: 0x003f_5908,
        f: ee::s14::ProtoAries_Nop,
    };

    pub const PROTO_ARIES_PEEK: Callee = Callee {
        addr: 0x003f_5a70,
        f: ee::s14::ProtoAriesPeek,
    };

    pub const MEMCPY: Callee = Callee {
        addr: 0x0041_605c,
        f: ee::s15::memcpy,
    };

    pub const MEMSET: Callee = Callee {
        addr: 0x0041_6210,
        f: ee::s15::memset,
    };

    pub const STRLEN: Callee = Callee {
        addr: 0x0041_6810,
        f: ee::s15::strlen,
    };

    pub const FLUSH_CACHE: Callee = Callee {
        addr: 0x0042_4020,
        f: ee::s15::FlushCache,
    };

    pub const SYNC_DCACHE: Callee = Callee {
        addr: 0x0042_4698,
        f: ee::s15::SyncDCache,
    };

    pub const INVALID_DCACHE: Callee = Callee {
        addr: 0x0042_47d8,
        f: ee::s15::InvalidDCache,
    };
}

// ---------------------------------------------------------------------------
// ProtoAries: Aries sockets over DirtyRpc. The socket itself lives in the
// IOP's Aries server; the EE object holds its handle, an address block the
// IOP reads and writes, and a receive buffer the IOP fills with records.

/// The socket object's word 0: the IOP-side socket handle (the create
/// request's reply writes it).
pub(crate) const PROTO_ARIES_HANDLE: u32 = 0x00;
/// Pointer to the 0x20-byte address block shared with the IOP (its own
/// 64-byte cache line): the peer address at +0, the local one at +0x10.
pub(crate) const PROTO_ARIES_ADDRS: u32 = 0x04;
/// Non-zero while a receive request is outstanding.
pub(crate) const PROTO_ARIES_RECV_PENDING: u32 = 0x08;
/// Read offset into the receive buffer.
pub(crate) const PROTO_ARIES_RECV_READ: u32 = 0x0c;
/// Bytes the last receive reply put in the buffer (what `'ibuf'` reports).
pub(crate) const PROTO_ARIES_RECV_FILL: u32 = 0x10;
/// Receive buffer size and address.
pub(crate) const PROTO_ARIES_RECV_SIZE: u32 = 0x14;
pub(crate) const PROTO_ARIES_RECV_BUF: u32 = 0x18;
/// Bytes of the object itself; the address block and the receive buffer
/// follow it in the same allocation.
pub(crate) const PROTO_ARIES_OBJECT_SIZE: u32 = 0x9c;
/// `(self + 0x5b) & ~0x3f`: the first 64-byte line at or after `self + 0x1c`.
pub(crate) const PROTO_ARIES_ADDRS_ROUND: u32 = 0x5b;
/// The address block's size, and the receive buffer's distance from it (one
/// cache line).
pub(crate) const PROTO_ARIES_ADDRS_SIZE: u32 = 0x20;
pub(crate) const PROTO_ARIES_ADDRS_LINE: u32 = 0x40;
/// Receive buffer size limits (`ProtoAriesCreate` clamps to them).
pub(crate) const PROTO_ARIES_MIN_BUFFER: u64 = 0x1000;
pub(crate) const PROTO_ARIES_MAX_BUFFER: u64 = 0x2000;

/// The two halves of the address block, each laid out like a `sockaddr_in`:
/// family halfword, big-endian port, big-endian IPv4 address.
pub(crate) const PROTO_ARIES_PEER: u32 = 0x00;
pub(crate) const PROTO_ARIES_LOCAL: u32 = 0x10;
pub(crate) const PROTO_ARIES_ADDR_FAMILY: u32 = 0x0;
pub(crate) const PROTO_ARIES_ADDR_PORT: u32 = 0x2;
pub(crate) const PROTO_ARIES_ADDR_IP: u32 = 0x4;
/// `AF_INET`.
pub(crate) const PROTO_ARIES_AF_INET: u64 = 2;

/// A receive-buffer record: two big-endian words, the big-endian record
/// length (header included), then the payload.
pub(crate) const PROTO_ARIES_RECORD_WORD0: u32 = 0x0;
pub(crate) const PROTO_ARIES_RECORD_WORD1: u32 = 0x4;
pub(crate) const PROTO_ARIES_RECORD_LEN: u32 = 0x8;
pub(crate) const PROTO_ARIES_RECORD_HEADER: u32 = 0xc;
/// `""` in the read-only data (`ProtoAriesPeek`'s payload for an empty
/// record).
pub(crate) const PROTO_ARIES_EMPTY_STRING: u32 = 0x004a_4bb8;

/// The DirtyRpc request records, one per call site (`lui 0x52` then the low
/// half; `s0` holds the `lui` half first where a call comes between).
pub(crate) const PROTO_ARIES_REQUESTS_HI: u64 = 0x0052_0000;
pub(crate) const PROTO_ARIES_CREATE_REQUEST: u32 = 0x0052_5f00;
pub(crate) const PROTO_ARIES_DESTROY_REQUEST: u32 = 0x0052_5f10;
pub(crate) const PROTO_ARIES_CONNECT_REQUEST: u32 = 0x0052_5f20;
pub(crate) const PROTO_ARIES_UNCONNECT_REQUEST: u32 = 0x0052_5f30;
pub(crate) const PROTO_ARIES_STATUS_REQUEST: u32 = 0x0052_5f40;
pub(crate) const PROTO_ARIES_SEND_REQUEST: u32 = 0x0052_5f50;
pub(crate) const PROTO_ARIES_PEEK_REQUEST: u32 = 0x0052_5f60;
pub(crate) const PROTO_ARIES_RECV_REQUEST: u32 = 0x0052_5f70;

/// The Aries server's DirtyRpc id, `'sess'`.
pub(crate) const PROTO_ARIES_SERVER: u32 = 0x7365_7373;
/// The server's entry points: `'crea'`, `'dest'`, `'conn'`, `'disc'`,
/// `'stat'`, `'send'`, `'recv'`.
pub(crate) const PROTO_ARIES_CREATE: u32 = 0x6372_6561;
pub(crate) const PROTO_ARIES_DESTROY: u32 = 0x6465_7374;
pub(crate) const PROTO_ARIES_CONNECT: u32 = 0x636f_6e6e;
pub(crate) const PROTO_ARIES_DISCONNECT: u32 = 0x6469_7363;
pub(crate) const PROTO_ARIES_STATUS: u32 = 0x7374_6174;
pub(crate) const PROTO_ARIES_SEND: u32 = 0x7365_6e64;
pub(crate) const PROTO_ARIES_RECV: u32 = 0x7265_6376;
/// `ProtoAriesStatus` selectors: `'port'`, `'addr'` (the peer's), `'lprt'`,
/// `'ladr'` (the local ones), `'ibuf'` (buffered receive bytes).
pub(crate) const PROTO_ARIES_SEL_PORT: u32 = 0x706f_7274;
pub(crate) const PROTO_ARIES_SEL_ADDR: u32 = 0x6164_6472;
pub(crate) const PROTO_ARIES_SEL_LOCAL_ADDR: u32 = 0x6c61_6472;
pub(crate) const PROTO_ARIES_SEL_LOCAL_PORT: u32 = 0x6c70_7274;
pub(crate) const PROTO_ARIES_SEL_INPUT: u32 = 0x6962_7566;
/// `ProtoAries_RpcReplyCallback`, the receive request's completion callback.
pub(crate) const PROTO_ARIES_RPC_REPLY_CALLBACK: u32 = 0x003f_5a50;

/// The address of the socket object's field `off` (`s1` holds the object in
/// every ProtoAries function).
fn proto_aries_field(c: &Cpu, off: u32) -> u32 {
    r32(c, S1).wrapping_add(off)
}

/// `sra reg, n` on the low word.
fn proto_aries_sra(c: &Cpu, reg: usize, n: u32) -> u64 {
    sx(((c.g[reg][0] as u32) as i32 >> n) as u32)
}

/// Point `s0`/`a0` at `request` and look up the Aries server's entry point
/// `code`: `DirtyRpc_InitRequest(request, 'sess', code)`.
fn proto_aries_init_request(c: &mut Cpu, request: u32, code: u32, ret: u32) -> Flow {
    c.g[S0][0] = sx(request);
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = PROTO_ARIES_SERVER as u64;
    c.g[A2][0] = code as u64;
    callee::DIRTY_RPC_INIT_REQUEST.call(c, ret)
}

/// `ProtoAriesRefT *ProtoAriesCreate(int32_t bufsize)` (0x3f5478): allocate a
/// socket object with its address block and a receive buffer of `bufsize`
/// bytes (clamped to 0x1000..=0x2000, 64-bit compares), zero both, write them
/// back from the data cache and ask the IOP's Aries server for a socket
/// (`'crea'`, synchronous, with the buffer size). Returns the object; 0 when
/// the allocation fails or the reply left no handle in word 0 (the object is
/// freed again).
pub fn proto_aries_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = PROTO_ARIES_MIN_BUFFER - 1;
        frame.save(c, S2, 0x20);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
        c.g[S2][0] = PROTO_ARIES_MIN_BUFFER;
        c.g[A1][0] = PROTO_ARIES_MAX_BUFFER;
        if c.g[V0][0] != 0 {
            c.g[S2][0] = c.g[A0][0];
        }
        frame.save(c, S1, 0x30);
        c.g[V1][0] = ((c.g[S2][0] as i64) < PROTO_ARIES_MAX_BUFFER as i64 + 1) as u64;
        frame.save(c, S0, 0x40);
        if c.g[V1][0] == 0 {
            c.g[S2][0] = c.g[A1][0];
        }
        frame.save_ra(c, 0x10);
        c.g[A0][0] = sx(r32(c, S2).wrapping_add(PROTO_ARIES_OBJECT_SIZE));
        call_ptr(c, callee::NET_RPC_ALLOC.addr, 0x003f_54b0)?;
        c.g[S1][0] = c.g[V0][0];
        if c.g[S1][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.g[A0][0] = c.g[S1][0];
            c.cycles += 50;
            c.g[A1][0] = 0;
            c.g[A2][0] = PROTO_ARIES_OBJECT_SIZE as u64;
            call_ptr(c, callee::MEMSET.addr, 0x003f_54d0)?;
            // The address block's line, then the receive buffer one line on.
            c.g[V0][0] = sx(!0x3f);
            c.g[V1][0] = sx(r32(c, S1).wrapping_add(PROTO_ARIES_ADDRS_ROUND));
            c.g[V1][0] &= c.g[V0][0];
            Addr(proto_aries_field(c, PROTO_ARIES_RECV_SIZE)).sw(c, c.g[S2][0]);
            c.g[V0][0] = sx(r32(c, V1).wrapping_add(PROTO_ARIES_ADDRS_LINE));
            c.g[A0][0] = c.g[V1][0];
            Addr(proto_aries_field(c, PROTO_ARIES_ADDRS)).sw(c, c.g[V1][0]);
            c.g[A1][0] = 0;
            c.g[A2][0] = PROTO_ARIES_ADDRS_SIZE as u64;
            Addr(proto_aries_field(c, PROTO_ARIES_RECV_BUF)).sw(c, c.g[V0][0]);
            call_ptr(c, callee::MEMSET.addr, 0x003f_54fc)?;
            c.g[A2][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_SIZE)).lw(c);
            c.g[A1][0] = 0;
            c.g[A0][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_BUF)).lw(c);
            call_ptr(c, callee::MEMSET.addr, 0x003f_550c)?;
            c.g[A0][0] = Addr(proto_aries_field(c, PROTO_ARIES_ADDRS)).lw(c);
            c.g[A1][0] = sx(r32(c, A0).wrapping_add(PROTO_ARIES_ADDRS_SIZE));
            call_ptr(c, callee::SYNC_DCACHE.addr, 0x003f_5518)?;
            c.g[A0][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_BUF)).lw(c);
            c.g[A1][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_SIZE)).lw(c);
            c.g[A1][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
            call_ptr(c, callee::SYNC_DCACHE.addr, 0x003f_5528)?;
            c.g[A0][0] = 0;
            call_ptr(c, callee::FLUSH_CACHE.addr, 0x003f_5530)?;
            proto_aries_init_request(
                c,
                PROTO_ARIES_CREATE_REQUEST,
                PROTO_ARIES_CREATE,
                0x003f_5550,
            )?;
            // DirtyRpc_Call(request, 1, 0, size, 0, 0, 0, 0; sync, self).
            c.g[V1][0] = 1;
            c.g[A0][0] = c.g[S0][0];
            Addr(frame.at(c, 0)).sw(c, c.g[V1][0]);
            c.g[A3][0] = c.g[S2][0];
            Addr(frame.at(c, 8)).sw(c, c.g[S1][0]);
            c.g[A1][0] = 1;
            c.g[A2][0] = 0;
            c.g[T0][0] = 0;
            c.g[T1][0] = 0;
            c.g[T2][0] = 0;
            c.g[T3][0] = 0;
            callee::DIRTY_RPC_CALL.call(c, 0x003f_5580)?;
            c.g[V1][0] = Addr(proto_aries_field(c, PROTO_ARIES_HANDLE)).lw(c);
            c.g[V0][0] = c.g[S1][0];
            if c.g[V1][0] == 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, callee::NET_RPC_FREE.addr, 0x003f_5594)?;
                c.g[S1][0] = 0;
                c.g[V0][0] = c.g[S1][0];
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void ProtoAriesDestroy(ProtoAriesRefT *ref)` (0x3f55b8): ask the IOP to
/// destroy the socket (`'dest'`, synchronous) and free the object.
pub fn proto_aries_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        proto_aries_init_request(
            c,
            PROTO_ARIES_DESTROY_REQUEST,
            PROTO_ARIES_DESTROY,
            0x003f_55ec,
        )?;
        // DirtyRpc_Call(request, 1, handle, 0, 0, 0, 0, 0; sync, 0).
        c.g[A2][0] = Addr(proto_aries_field(c, PROTO_ARIES_HANDLE)).lw(c);
        c.g[V0][0] = 1;
        c.g[A0][0] = c.g[S0][0];
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        c.g[T2][0] = 0;
        c.g[T3][0] = 0;
        Addr(frame.at(c, 0)).sw(c, c.g[V0][0]);
        c.g[A1][0] = 1;
        Addr(frame.at(c, 8)).sw(c, 0);
        callee::DIRTY_RPC_CALL.call(c, 0x003f_561c)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::NET_RPC_FREE.addr, 0x003f_5624)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `reg = self->addrs` (the original reloads the pointer for every store),
/// then `store(reg + off, value)`.
fn proto_aries_addrs_store(
    c: &mut Cpu,
    reg: usize,
    off: u32,
    store: fn(Addr, &mut Cpu, u64),
    value: u64,
) {
    c.g[reg][0] = Addr(proto_aries_field(c, PROTO_ARIES_ADDRS)).lw(c);
    let at = r32(c, reg).wrapping_add(off);
    store(Addr(at), c, value);
}

/// `int32_t ProtoAriesConnect(ProtoAriesRefT *ref, uint32_t addr, int32_t
/// port)` (0x3f5638): -1 if the peer address's family is already set;
/// otherwise store `AF_INET`, the port and the IPv4 address (network order)
/// as the peer address, write the block back from the data cache, send the
/// 16-byte address with `'conn'` (asynchronous), clear the buffered-bytes
/// word and return 0.
pub fn proto_aries_connect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[V1][0] = Addr(proto_aries_field(c, PROTO_ARIES_ADDRS)).lw(c);
        c.g[V0][0] =
            Addr(r32(c, V1).wrapping_add(PROTO_ARIES_PEER + PROTO_ARIES_ADDR_FAMILY)).lhu(c);
        if c.g[V0][0] != 0 {
            c.g[V0][0] = u64::MAX;
        } else {
            c.cycles += 50;
            c.g[V0][0] = PROTO_ARIES_AF_INET;
            c.g[A0][0] = proto_aries_sra(c, A2, 8);
            Addr(r32(c, V1).wrapping_add(PROTO_ARIES_PEER + PROTO_ARIES_ADDR_FAMILY))
                .sh(c, c.g[V0][0]);
            c.g[A3][0] = sx(r32(c, A1) >> 8);
            // Clear the rest of the peer address, then the port and address
            // bytes, most significant first in memory.
            let port = PROTO_ARIES_PEER + PROTO_ARIES_ADDR_PORT;
            let ip = PROTO_ARIES_PEER + PROTO_ARIES_ADDR_IP;
            proto_aries_addrs_store(c, V0, port, Addr::sh, 0);
            proto_aries_addrs_store(c, V1, ip, Addr::sw, 0);
            proto_aries_addrs_store(c, V0, ip + 4, Addr::sw, 0);
            proto_aries_addrs_store(c, V1, ip + 8, Addr::sw, 0);
            let port_hi = c.g[A0][0];
            proto_aries_addrs_store(c, V0, port, Addr::sb, port_hi);
            let port_lo = c.g[A2][0];
            proto_aries_addrs_store(c, V1, port + 1, Addr::sb, port_lo);
            let ip0 = c.g[A1][0];
            proto_aries_addrs_store(c, V0, ip + 3, Addr::sb, ip0);
            let ip8 = c.g[A3][0];
            proto_aries_addrs_store(c, V1, ip + 2, Addr::sb, ip8);
            c.g[A3][0] = sx(r32(c, A1) >> 16);
            let ip16 = c.g[A3][0];
            proto_aries_addrs_store(c, V0, ip + 1, Addr::sb, ip16);
            c.g[A3][0] = sx(r32(c, A1) >> 24);
            let ip24 = c.g[A3][0];
            proto_aries_addrs_store(c, V0, ip, Addr::sb, ip24);
            c.g[A0][0] = Addr(proto_aries_field(c, PROTO_ARIES_ADDRS)).lw(c);
            c.g[A1][0] = sx(r32(c, A0).wrapping_add(PROTO_ARIES_ADDRS_SIZE));
            call_ptr(c, callee::SYNC_DCACHE.addr, 0x003f_56d0)?;
            proto_aries_init_request(
                c,
                PROTO_ARIES_CONNECT_REQUEST,
                PROTO_ARIES_CONNECT,
                0x003f_56f0,
            )?;
            // DirtyRpc_Call(request, 0, handle, 0, addrs, 16, 0, 0; 0, 0).
            c.g[A2][0] = Addr(proto_aries_field(c, PROTO_ARIES_HANDLE)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[T0][0] = Addr(proto_aries_field(c, PROTO_ARIES_ADDRS)).lw(c);
            c.g[A1][0] = 0;
            Addr(frame.at(c, 0)).sw(c, 0);
            c.g[A3][0] = 0;
            Addr(frame.at(c, 8)).sw(c, 0);
            c.g[T1][0] = 0x10;
            c.g[T2][0] = 0;
            c.g[T3][0] = 0;
            callee::DIRTY_RPC_CALL.call(c, 0x003f_571c)?;
            Addr(proto_aries_field(c, PROTO_ARIES_RECV_FILL)).sw(c, 0);
            c.g[V0][0] = 0;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int32_t ProtoAriesUnconnect(ProtoAriesRefT *ref)` (0x3f5738): send
/// `'disc'` (asynchronous), clear the address block and the buffered-bytes
/// word; returns 0.
pub fn proto_aries_unconnect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        proto_aries_init_request(
            c,
            PROTO_ARIES_UNCONNECT_REQUEST,
            PROTO_ARIES_DISCONNECT,
            0x003f_576c,
        )?;
        // DirtyRpc_Call(request, 0, handle, 0, 0, 0, 0, 0; 0, 0).
        c.g[A2][0] = Addr(proto_aries_field(c, PROTO_ARIES_HANDLE)).lw(c);
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        c.g[T2][0] = 0;
        c.g[T3][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        Addr(frame.at(c, 0)).sw(c, 0);
        Addr(frame.at(c, 8)).sw(c, 0);
        c.g[A1][0] = 0;
        callee::DIRTY_RPC_CALL.call(c, 0x003f_5798)?;
        c.g[A0][0] = Addr(proto_aries_field(c, PROTO_ARIES_ADDRS)).lw(c);
        c.g[A1][0] = 0;
        c.g[A2][0] = PROTO_ARIES_ADDRS_SIZE as u64;
        call_ptr(c, callee::MEMSET.addr, 0x003f_57a8)?;
        Addr(proto_aries_field(c, PROTO_ARIES_RECV_FILL)).sw(c, 0);
        c.g[V0][0] = 0;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `ProtoAriesStatus`'s big-endian halfword at `off` in the address block:
/// `v1 = self->addrs`, the bytes in `v0`/`a0`, folded into `v0`.
fn proto_aries_status_port(c: &mut Cpu, off: u32) {
    c.g[V1][0] = Addr(proto_aries_field(c, PROTO_ARIES_ADDRS)).lw(c);
    let at = r32(c, V1).wrapping_add(off);
    c.g[V0][0] = Addr(at).lbu(c);
    c.g[A0][0] = Addr(at + 1).lbu(c);
    c.cycles += 3;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
    c.g[V0][0] |= c.g[A0][0];
}

/// `ProtoAriesStatus`'s big-endian word at `off` in the address block:
/// `v1 = self->addrs`, the low three bytes left in `a0`/`a1`/`a2`, the word
/// (sign-extended) in `v0`.
fn proto_aries_status_addr(c: &mut Cpu, off: u32) {
    c.g[V1][0] = Addr(proto_aries_field(c, PROTO_ARIES_ADDRS)).lw(c);
    let at = r32(c, V1).wrapping_add(off);
    c.g[V0][0] = Addr(at).lbu(c);
    c.g[A0][0] = Addr(at + 1).lbu(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
    c.g[A1][0] = Addr(at + 2).lbu(c);
    c.g[V0][0] |= c.g[A0][0];
    c.g[A2][0] = Addr(at + 3).lbu(c);
    c.cycles += 5;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
    c.g[V0][0] |= c.g[A1][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
    c.g[V0][0] |= c.g[A2][0];
}

/// `int32_t ProtoAriesStatus(ProtoAriesRefT *ref, int32_t select)`
/// (0x3f57c8): refresh the address block from the IOP (`'stat'`,
/// synchronous, 0x20 bytes back into the block), then answer `select` (a
/// 64-bit compare): `'port'`/`'addr'` the peer's port and address,
/// `'lprt'`/`'ladr'` the local ones, `'ibuf'` the buffered receive bytes,
/// anything else 0.
pub fn proto_aries_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save(c, S1, 0x30);
        c.g[S2][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        proto_aries_init_request(
            c,
            PROTO_ARIES_STATUS_REQUEST,
            PROTO_ARIES_STATUS,
            0x003f_5804,
        )?;
        // DirtyRpc_Call(request, 0, handle, 0, 0, 0, addrs, 0x20; sync, 0).
        c.g[V1][0] = 1;
        c.g[A2][0] = Addr(proto_aries_field(c, PROTO_ARIES_HANDLE)).lw(c);
        c.g[T2][0] = Addr(proto_aries_field(c, PROTO_ARIES_ADDRS)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        Addr(frame.at(c, 0)).sw(c, c.g[V1][0]);
        c.g[A1][0] = 0;
        Addr(frame.at(c, 8)).sw(c, 0);
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        c.g[T3][0] = PROTO_ARIES_ADDRS_SIZE as u64;
        callee::DIRTY_RPC_CALL.call(c, 0x003f_5834)?;
        c.g[V1][0] = PROTO_ARIES_SEL_PORT as u64;
        if c.g[S2][0] == c.g[V1][0] {
            c.cycles += 4;
            proto_aries_status_port(c, PROTO_ARIES_PEER + PROTO_ARIES_ADDR_PORT);
        } else {
            c.cycles += 3;
            c.g[V0][0] = PROTO_ARIES_SEL_ADDR as u64;
            if c.g[S2][0] == c.g[V0][0] {
                c.cycles += 8;
                proto_aries_status_addr(c, PROTO_ARIES_PEER + PROTO_ARIES_ADDR_IP);
            } else {
                c.cycles += 3;
                c.g[V0][0] = PROTO_ARIES_SEL_LOCAL_ADDR as u64;
                if c.g[S2][0] == c.g[V0][0] {
                    c.cycles += 7;
                    proto_aries_status_addr(c, PROTO_ARIES_LOCAL + PROTO_ARIES_ADDR_IP);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = PROTO_ARIES_SEL_LOCAL_PORT as u64;
                    if c.g[S2][0] == c.g[V0][0] {
                        c.cycles += 3;
                        proto_aries_status_port(c, PROTO_ARIES_LOCAL + PROTO_ARIES_ADDR_PORT);
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = PROTO_ARIES_SEL_INPUT as u64;
                        let other = c.g[S2][0] != c.g[V0][0];
                        c.g[V0][0] = 0;
                        if !other {
                            c.cycles += 1;
                            c.g[V0][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_FILL)).lw(c);
                        }
                    }
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void ProtoAries_Nop(void)` (0x3f5908): does nothing (`ProtoAriesSend`
/// calls it first).
pub fn proto_aries_nop(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `int32_t ProtoAriesSend(ProtoAriesRefT *ref, uint32_t word0, uint32_t
/// word1, const char *data, int32_t len)` (0x3f5910): send one record, a
/// 12-byte big-endian header (`word0`, `word1`, record length) followed by
/// `len` bytes of `data` (`strlen(data) + 1` when `len` is negative, a 64-bit
/// compare), as a two-part send list with `'send'` (asynchronous). Returns 0.
pub fn proto_aries_send(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S6, 0x50);
        frame.save(c, S2, 0x90);
        c.g[S6][0] = c.g[A0][0];
        frame.save(c, S3, 0x80);
        c.g[S2][0] = c.g[T0][0];
        frame.save(c, S4, 0x70);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S5, 0x60);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, S0, 0xb0);
        frame.save(c, S1, 0xa0);
        frame.save_ra(c, 0x40);
        c.g[S5][0] = c.g[A3][0];
        callee::PROTO_ARIES_NOP.call(c, 0x003f_594c)?;
        let given = (c.g[S2][0] as i64) >= 0;
        c.g[S2][0] = sx(r32(c, S2).wrapping_add(PROTO_ARIES_RECORD_HEADER));
        if !given {
            c.cycles += 4;
            c.g[A0][0] = c.g[S5][0];
            call_ptr(c, callee::STRLEN.addr, 0x003f_595c)?;
            c.g[S2][0] = sx(r32(c, V0).wrapping_add(1));
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(PROTO_ARIES_RECORD_HEADER));
        }
        c.cycles += 59;
        // `s2` is the record length; the header bytes, most significant
        // first: word0 (`s3`), word1 (`s4`), the length.
        c.g[V1][0] = proto_aries_sra(c, S2, 16);
        c.g[T4][0] = proto_aries_sra(c, S4, 16);
        c.g[T5][0] = proto_aries_sra(c, S4, 8);
        c.g[T7][0] = sx(frame.at(c, 0x10));
        c.g[A3][0] = proto_aries_sra(c, S2, 8);
        c.g[T0][0] = proto_aries_sra(c, S3, 24);
        c.g[T1][0] = proto_aries_sra(c, S3, 16);
        c.g[T2][0] = proto_aries_sra(c, S3, 8);
        c.g[T3][0] = proto_aries_sra(c, S4, 24);
        c.g[T6][0] = PROTO_ARIES_RECORD_HEADER as u64;
        c.g[S1][0] = sx(r32(c, S2).wrapping_sub(PROTO_ARIES_RECORD_HEADER));
        c.g[V0][0] = proto_aries_sra(c, S2, 24);
        let header = frame.at(c, 0x10);
        let bytes = [T0, T1, T2, S3, T3, T4, T5, S4, V0, V1, A3, S2];
        for (i, reg) in bytes.into_iter().enumerate() {
            let v = c.g[reg][0];
            Addr(header + i as u32).sb(c, v);
        }
        // The send list, `{length, data}` pairs ended by a zero length: the
        // header, then the data.
        Addr(frame.at(c, 0x20)).sw(c, c.g[T6][0]);
        Addr(frame.at(c, 0x24)).sw(c, c.g[T7][0]);
        Addr(frame.at(c, 0x28)).sw(c, c.g[S1][0]);
        Addr(frame.at(c, 0x2c)).sw(c, c.g[S5][0]);
        Addr(frame.at(c, 0x30)).sw(c, 0);
        proto_aries_init_request(c, PROTO_ARIES_SEND_REQUEST, PROTO_ARIES_SEND, 0x003f_59f8)?;
        // DirtyRpc_Call(request, 2 (t0 is a send list), handle, 0, list, 0,
        // 0, 0; 0, 0).
        c.g[A2][0] = Addr(r32(c, S6)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        Addr(frame.at(c, 0)).sw(c, 0);
        c.g[A1][0] = 2;
        Addr(frame.at(c, 8)).sw(c, 0);
        c.g[A3][0] = 0;
        c.g[T0][0] = sx(frame.at(c, 0x20));
        c.g[T1][0] = 0;
        c.g[T2][0] = 0;
        c.g[T3][0] = 0;
        callee::DIRTY_RPC_CALL.call(c, 0x003f_5a24)?;
        frame.restore(c, S0, 0xb0);
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore(c, S6, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `v0 = self->recv_read < self->recv_fill` (signed): unread bytes in the
/// receive buffer.
fn proto_aries_unread(c: &mut Cpu) -> bool {
    c.g[V0][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_READ)).lw(c);
    c.g[V1][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_FILL)).lw(c);
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    c.g[V0][0] != 0
}

/// A big-endian word at `at`, `lbu` into `regs` and folded as the compiled
/// code does: `regs[0..3]` keep the top three bytes shifted into place (the
/// first or-ed together), `regs[3]` the low byte; returns the whole word
/// (sign-extended).
fn proto_aries_be_word(c: &mut Cpu, at: u32, regs: [usize; 4]) -> u64 {
    for (i, reg) in regs.into_iter().enumerate() {
        c.g[reg][0] = Addr(at.wrapping_add(i as u32)).lbu(c);
    }
    c.g[regs[0]][0] = sx((c.g[regs[0]][0] as u32) << 24);
    c.g[regs[1]][0] = sx((c.g[regs[1]][0] as u32) << 16);
    c.g[regs[2]][0] = sx((c.g[regs[2]][0] as u32) << 8);
    c.g[regs[0]][0] |= c.g[regs[1]][0];
    c.g[regs[0]][0] |= c.g[regs[2]][0];
    c.g[regs[0]][0] | c.g[regs[3]][0]
}

/// `ProtoAriesPeek`'s record arm: the record at the read offset (`t0`); its
/// payload length (`t1`, record length - 12), the payload (`a2`, its last
/// byte overwritten with a NUL, or `""` when empty) and the two header words
/// stored through the non-null outputs `t2`, `t3`, `a3`; `v0 = t1`.
fn proto_aries_peek_record(c: &mut Cpu) {
    c.cycles += 19;
    c.g[A0][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_READ)).lw(c);
    c.g[V1][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_BUF)).lw(c);
    c.g[A1][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_READ)).lw(c);
    c.g[T0][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
    c.g[A2][0] = sx(r32(c, V1).wrapping_add(PROTO_ARIES_RECORD_HEADER));
    let record = r32(c, T0);
    let len = proto_aries_be_word(
        c,
        record.wrapping_add(PROTO_ARIES_RECORD_LEN),
        [V0, A0, A1, V1],
    );
    c.g[T1][0] = len;
    c.g[T1][0] = sx(r32(c, T1).wrapping_sub(PROTO_ARIES_RECORD_HEADER));
    let has_payload = (c.g[T1][0] as i64) > 0;
    c.g[V0][0] = sx(r32(c, T1).wrapping_add(r32(c, A2)));
    if has_payload {
        c.cycles += 1;
        Addr(r32(c, V0).wrapping_sub(1)).sb(c, 0);
    } else {
        c.cycles += 3;
        c.g[V0][0] = sx(PROTO_ARIES_EMPTY_STRING & 0xffff_0000);
        c.g[A2][0] = sx(PROTO_ARIES_EMPTY_STRING);
    }
    c.cycles += 2;
    if c.g[T2][0] != 0 {
        c.cycles += 11;
        c.g[V0][0] = proto_aries_be_word(
            c,
            record.wrapping_add(PROTO_ARIES_RECORD_WORD0),
            [V0, V1, A0, A1],
        );
        Addr(r32(c, T2)).sw(c, c.g[V0][0]);
    }
    c.cycles += 2;
    if c.g[T3][0] != 0 {
        c.cycles += 11;
        c.g[V0][0] = proto_aries_be_word(
            c,
            record.wrapping_add(PROTO_ARIES_RECORD_WORD1),
            [V0, V1, A0, A1],
        );
        Addr(r32(c, T3)).sw(c, c.g[V0][0]);
    }
    c.cycles += 2;
    if c.g[A3][0] != 0 {
        Addr(r32(c, A3)).sw(c, c.g[A2][0]);
    }
    c.cycles += 1;
    c.g[V0][0] = c.g[T1][0];
}

/// Where `ProtoAriesPeek` and `ProtoAriesRecv` start the next receive (the
/// same code at two places): the request record, the return addresses of
/// its three calls and the charge of the block before `DirtyRpc_InitRequest`.
pub(crate) struct ProtoAriesReceiveSite {
    pub(crate) request: u32,
    pub(crate) invalidate_ret: u32,
    pub(crate) init_ret: u32,
    pub(crate) call_ret: u32,
    pub(crate) cycles: u64,
}

pub(crate) const PROTO_ARIES_PEEK_RECEIVE: ProtoAriesReceiveSite = ProtoAriesReceiveSite {
    request: PROTO_ARIES_PEEK_REQUEST,
    invalidate_ret: 0x003f_5aec,
    init_ret: 0x003f_5b18,
    call_ret: 0x003f_5b4c,
    cycles: 22,
};

pub(crate) const PROTO_ARIES_RECV_RECEIVE: ProtoAriesReceiveSite = ProtoAriesReceiveSite {
    request: PROTO_ARIES_RECV_REQUEST,
    invalidate_ret: 0x003f_5d04,
    init_ret: 0x003f_5d30,
    call_ret: 0x003f_5d64,
    cycles: 20,
};

/// Mark a receive pending; if the last reply put bytes in the buffer, drop
/// the buffer from the data cache and rewind it; then ask the IOP to fill it
/// (`'recv'`, completion through `ProtoAries_RpcReplyCallback(self)`).
fn proto_aries_request_receive(c: &mut Cpu, site: &ProtoAriesReceiveSite) -> Flow {
    c.cycles += 5;
    c.g[V0][0] = 1;
    Addr(proto_aries_field(c, PROTO_ARIES_RECV_PENDING)).sw(c, c.g[V0][0]);
    c.g[V1][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_FILL)).lw(c);
    let empty = (c.g[V1][0] as i64) <= 0;
    c.g[S0][0] = PROTO_ARIES_REQUESTS_HI;
    if !empty {
        c.cycles += 10;
        c.g[A1][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_BUF)).lw(c);
        c.g[V0][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_SIZE)).lw(c);
        c.g[A0][0] = c.g[A1][0];
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
        call_ptr(c, callee::INVALID_DCACHE.addr, site.invalidate_ret)?;
        Addr(proto_aries_field(c, PROTO_ARIES_RECV_FILL)).sw(c, 0);
        c.g[V0][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_FILL)).lw(c);
        Addr(proto_aries_field(c, PROTO_ARIES_RECV_READ)).sw(c, c.g[V0][0]);
        c.g[S0][0] = PROTO_ARIES_REQUESTS_HI;
    }
    c.cycles += site.cycles;
    proto_aries_init_request(c, site.request, PROTO_ARIES_RECV, site.init_ret)?;
    // DirtyRpc_Call(request, 0, handle, 0, 0, 0, buf, size; callback, self).
    c.g[V1][0] = sx(PROTO_ARIES_RPC_REPLY_CALLBACK);
    c.g[T3][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_SIZE)).lw(c);
    c.g[A2][0] = Addr(proto_aries_field(c, PROTO_ARIES_HANDLE)).lw(c);
    c.g[T2][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_BUF)).lw(c);
    c.g[A0][0] = c.g[S0][0];
    Addr(sp(c)).sw(c, c.g[V1][0]);
    c.g[A1][0] = 0;
    Addr(sp(c).wrapping_add(8)).sw(c, c.g[S1][0]);
    c.g[A3][0] = 0;
    c.g[T0][0] = 0;
    c.g[T1][0] = 0;
    callee::DIRTY_RPC_CALL.call(c, site.call_ret)
}

/// `int32_t ProtoAriesPeek(ProtoAriesRefT *ref, uint32_t *word0, uint32_t
/// *word1, char **data)` (0x3f5a70): with unread bytes in the receive
/// buffer, look at the record at the read offset without consuming it
/// (returns its payload length; see `proto_aries_peek_record`). Otherwise
/// -1, and unless a receive is already pending, start the next one.
pub fn proto_aries_peek(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        c.g[T2][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        c.g[T3][0] = c.g[A2][0];
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        if proto_aries_unread(c) {
            proto_aries_peek_record(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_PENDING)).lw(c);
            let pending = c.g[V0][0] != 0;
            c.g[V0][0] = u64::MAX;
            if !pending {
                c.cycles += 5;
                let unread = proto_aries_unread(c);
                c.g[V0][0] = u64::MAX;
                if !unread {
                    proto_aries_request_receive(c, &PROTO_ARIES_PEEK_RECEIVE)?;
                    c.g[V0][0] = u64::MAX;
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int32_t ProtoAriesRecv(ProtoAriesRefT *ref, uint32_t *word0, uint32_t
/// *word1, char *buf, int32_t len)` (0x3f5c38): peek at the next record; if
/// there is one, copy up to `len` payload bytes into `buf` (when non-null),
/// step the read offset past the record and, once the buffer is drained and
/// no receive is pending, start the next receive. Returns the payload length
/// (or `ProtoAriesPeek`'s negative result).
pub fn proto_aries_recv(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S3, 0x20);
        c.g[S0][0] = c.g[T0][0];
        frame.save_ra(c, 0x10);
        c.g[A3][0] = 0;
        callee::PROTO_ARIES_PEEK.call(c, 0x003f_5c64)?;
        c.g[S3][0] = c.g[V0][0];
        let none = (c.g[S3][0] as i64) < 0;
        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S0][0] as i64)) as u64;
        // Whether the exit goes through `v0 = s3` at 0x3f5d64 (after a failed
        // peek or a new receive) or straight to the epilogue.
        let via_result = 'consume: {
            if none {
                break 'consume true;
            }
            c.cycles += 2;
            let discard = c.g[S2][0] == 0;
            if c.g[V0][0] != 0 {
                c.g[S0][0] = c.g[S3][0];
            }
            if !discard {
                c.cycles += 7;
                c.g[A1][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_BUF)).lw(c);
                c.g[A0][0] = c.g[S2][0];
                c.g[V0][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_READ)).lw(c);
                c.g[A2][0] = c.g[S0][0];
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(PROTO_ARIES_RECORD_HEADER));
                call_ptr(c, callee::MEMCPY.addr, 0x003f_5c94)?;
            }
            c.cycles += 9;
            c.g[V0][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_READ)).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(PROTO_ARIES_RECORD_HEADER));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S3)));
            Addr(proto_aries_field(c, PROTO_ARIES_RECV_READ)).sw(c, c.g[V0][0]);
            c.g[V1][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_READ)).lw(c);
            c.g[V0][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_FILL)).lw(c);
            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V1][0] != 0;
            c.g[V0][0] = c.g[S3][0];
            if more {
                break 'consume false;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_PENDING)).lw(c);
            let pending = c.g[V0][0] != 0;
            c.g[V0][0] = c.g[S3][0];
            if pending {
                break 'consume false;
            }
            c.cycles += 5;
            let unread = proto_aries_unread(c);
            c.g[V0][0] = c.g[S3][0];
            if unread {
                break 'consume false;
            }
            proto_aries_request_receive(c, &PROTO_ARIES_RECV_RECEIVE)?;
            true
        };
        if via_result {
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
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

/// `uint32_t ProtoAriesTick(void)` (0x3f5d88): the network clock,
/// `NetTick()`.
pub fn proto_aries_tick(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(callee::NET_TICK), 0x003f_5d98);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, check_paths, fill, fresh, run_generated, Rng, DATA};
    use crate::game::dirtysock::tagsunk::tests::{
        mem_fakes_hook_with_new, MEM_FAKES_CTRL, MEM_FAKES_MAGIC,
    };
    use std::collections::BTreeSet;

    // The test socket: the object, its address block, its receive buffer.
    pub(crate) const PROTO_ARIES_SOCK: u32 = DATA + 0x1_0000;
    pub(crate) const PROTO_ARIES_SOCK_ADDRS: u32 = DATA + 0x1_0100;
    pub(crate) const PROTO_ARIES_SOCK_BUF: u32 = DATA + 0x1_1000;
    pub(crate) const PROTO_ARIES_SOCK_BUF_SIZE: u32 = 0x1000;
    /// Where `ProtoAriesCreate`'s allocation lands.
    pub(crate) const PROTO_ARIES_NEW: u32 = DATA + 0x2_0000;
    /// Output cells, and the send data / receive destination.
    pub(crate) const PROTO_ARIES_OUT: u32 = DATA + 0x5_0000;
    pub(crate) const PROTO_ARIES_PAYLOAD: u32 = DATA + 0x6_0000;

    // DirtyRpc's state (`lui 0x52; addiu -0xf40`): the fields its request
    // lookup and send path read.
    pub(crate) const PROTO_ARIES_RPC: u32 = 0x0051_f0c0;
    pub(crate) const PROTO_ARIES_RPC_ID_MOD: u32 = 0x4d84;
    pub(crate) const PROTO_ARIES_RPC_CLIENTS: u32 = 0x4d88;
    pub(crate) const PROTO_ARIES_RPC_CLIENTS_END: u32 = 0x4d8c;
    pub(crate) const PROTO_ARIES_RPC_RING: u32 = 0x4d94;
    pub(crate) const PROTO_ARIES_RPC_REPLY_RING: u32 = 0x4d9c;
    pub(crate) const PROTO_ARIES_RPC_SEND_BUF: u32 = 0x4dac;
    pub(crate) const PROTO_ARIES_RPC_SEND_IOP: u32 = 0x4db4;
    pub(crate) const PROTO_ARIES_RPC_DMA_IDS: u32 = 0x4db8;
    pub(crate) const PROTO_ARIES_RPC_SLOTS: u32 = 0x4dc4;
    pub(crate) const PROTO_ARIES_RPC_NEXT_ID: u32 = 0x4dcc;
    pub(crate) const PROTO_ARIES_RPC_HEAD: u32 = 0x4dd0;
    pub(crate) const PROTO_ARIES_RPC_TAIL: u32 = 0x4dd4;
    pub(crate) const PROTO_ARIES_RPC_RING_LEN: u32 = 0x4dd8;
    pub(crate) const PROTO_ARIES_RPC_SEND_AT: u32 = 0x4ddc;
    pub(crate) const PROTO_ARIES_RPC_SEND_LIMIT: u32 = 0x4de0;
    pub(crate) const PROTO_ARIES_RPC_SEND_END: u32 = 0x4de4;
    pub(crate) const PROTO_ARIES_RPC_ALARM: u32 = 0x4dec;
    pub(crate) const PROTO_ARIES_RPC_CLOCK: u32 = 0x4df8;
    pub(crate) const PROTO_ARIES_RPC_CLOCK_BASE: u32 = 0x4e00;
    pub(crate) const PROTO_ARIES_RPC_SEMA: u32 = 0x4e08;
    /// The tables those fields point at in the test machine.
    pub(crate) const PROTO_ARIES_RPC_TABLE: u32 = DATA + 0x4_0000;
    pub(crate) const PROTO_ARIES_RPC_SENDS: u32 = DATA + 0x4_1000;
    pub(crate) const PROTO_ARIES_RPC_IDS: u32 = DATA + 0x4_2000;
    pub(crate) const PROTO_ARIES_RPC_REPLY_SLOTS: u32 = DATA + 0x4_2100;
    pub(crate) const PROTO_ARIES_RPC_REQUESTS: u32 = DATA + 0x4_2200;
    /// A request record's client handle (0 when `DirtyRpc_InitRequest` found
    /// no client).
    pub(crate) const PROTO_ARIES_REQUEST_CLIENT: u32 = 0xc;

    /// Register the Aries server's entry points with DirtyRpc and give its
    /// send path an empty 8-entry ring, room in the send buffer and tables
    /// (pre-filled) to write; the kernel calls it makes are the machine's
    /// fake syscalls. Returns the ring head.
    pub(crate) fn proto_aries_rpc_server(c: &mut Cpu, r: &mut Rng) -> u32 {
        let codes = [
            PROTO_ARIES_CREATE,
            PROTO_ARIES_DESTROY,
            PROTO_ARIES_CONNECT,
            PROTO_ARIES_DISCONNECT,
            PROTO_ARIES_STATUS,
            PROTO_ARIES_SEND,
            PROTO_ARIES_RECV,
        ];
        for (i, code) in codes.into_iter().enumerate() {
            let e = PROTO_ARIES_RPC_TABLE + 0x10 * i as u32;
            c.bus.write32(e, i as u32);
            c.bus.write32(e + 4, PROTO_ARIES_SERVER);
            c.bus.write32(e + 8, code);
            c.bus.write32(e + 0xc, r.next_u32() | 1);
        }
        let head = r.next_u32() % 8;
        let rpc = PROTO_ARIES_RPC;
        let fields = [
            (PROTO_ARIES_RPC_CLIENTS, PROTO_ARIES_RPC_TABLE),
            (
                PROTO_ARIES_RPC_CLIENTS_END,
                PROTO_ARIES_RPC_TABLE + 0x10 * codes.len() as u32,
            ),
            (PROTO_ARIES_RPC_ID_MOD, 0x100),
            (PROTO_ARIES_RPC_NEXT_ID, r.next_u32() % 0x100),
            (PROTO_ARIES_RPC_RING_LEN, 8),
            (PROTO_ARIES_RPC_HEAD, head),
            (PROTO_ARIES_RPC_TAIL, head),
            (PROTO_ARIES_RPC_SEND_AT, 0),
            (PROTO_ARIES_RPC_SEND_LIMIT, 0x800),
            (PROTO_ARIES_RPC_SEND_END, 0x800),
            (PROTO_ARIES_RPC_SEND_BUF, PROTO_ARIES_RPC_SENDS),
            (PROTO_ARIES_RPC_SEND_IOP, r.next_u32() & !0x3f),
            (PROTO_ARIES_RPC_DMA_IDS, PROTO_ARIES_RPC_IDS),
            (PROTO_ARIES_RPC_SLOTS, PROTO_ARIES_RPC_REPLY_SLOTS),
            (PROTO_ARIES_RPC_RING, PROTO_ARIES_RPC_REQUESTS),
            (PROTO_ARIES_RPC_REPLY_RING, r.next_u32() & !0x1f),
            (PROTO_ARIES_RPC_ALARM, r.next_u32() >> 1),
            (PROTO_ARIES_RPC_SEMA, r.next_u32() & 0xff),
        ];
        for (off, v) in fields {
            c.bus.write32(rpc + off, v);
        }
        fill(c, r, PROTO_ARIES_RPC_SENDS, 0x80);
        fill(c, r, PROTO_ARIES_RPC_IDS, 8);
        fill(c, r, PROTO_ARIES_RPC_REPLY_SLOTS, 0x20);
        fill(c, r, PROTO_ARIES_RPC_REQUESTS, 0x40);
        head
    }

    /// A fake machine entering at `entry` with `a0` = a socket object at
    /// `PROTO_ARIES_SOCK` (fields, address block and receive buffer filled
    /// with junk) and the allocator fakes on. With `server` the Aries server
    /// is registered (`proto_aries_rpc_server`); without it
    /// `DirtyRpc_InitRequest` finds no client and `DirtyRpc_Send` returns at
    /// once.
    pub(crate) fn proto_aries_socket_fixture(
        c: &mut Cpu,
        seed: u32,
        entry: u32,
        server: bool,
    ) -> Rng {
        mem_fakes_hook_with_new();
        let mut r = machine(c, seed, entry as u64, 0);
        c.pc = entry;
        c.bus.write32(MEM_FAKES_CTRL, MEM_FAKES_MAGIC);
        let sock = PROTO_ARIES_SOCK;
        fill(c, &mut r, sock, 0x28);
        fill(c, &mut r, PROTO_ARIES_SOCK_ADDRS, 0x10);
        fill(
            c,
            &mut r,
            PROTO_ARIES_SOCK_BUF - 0x100,
            (PROTO_ARIES_SOCK_BUF_SIZE + 0x200) / 4,
        );
        c.bus
            .write32(sock + PROTO_ARIES_ADDRS, PROTO_ARIES_SOCK_ADDRS);
        c.bus
            .write32(sock + PROTO_ARIES_RECV_SIZE, PROTO_ARIES_SOCK_BUF_SIZE);
        c.bus
            .write32(sock + PROTO_ARIES_RECV_BUF, PROTO_ARIES_SOCK_BUF);
        c.g[A0][0] = sx(sock);
        if server {
            proto_aries_rpc_server(c, &mut r);
        }
        r
    }

    /// Whether `request` found the server (its client handle was copied in).
    pub(crate) fn proto_aries_sent(c: &mut Cpu, request: u32) -> bool {
        c.bus.read32(request + PROTO_ARIES_REQUEST_CLIENT) != 0
    }

    pub(crate) fn proto_aries_put_be(c: &mut Cpu, at: u32, v: u32) {
        for (i, b) in v.to_be_bytes().into_iter().enumerate() {
            c.bus.write8(at + i as u32, b);
        }
    }

    pub(crate) fn proto_aries_get_be(c: &mut Cpu, at: u32) -> u32 {
        let b = [
            c.bus.read8(at),
            c.bus.read8(at + 1),
            c.bus.read8(at + 2),
            c.bus.read8(at + 3),
        ];
        u32::from_be_bytes(b)
    }

    /// Run the generated body over the seeds and require every `want` arm,
    /// as `arm` reads it from the final state (for arms whose cycle totals
    /// depend on callee costs: fake syscalls, cache loops, `memset`).
    pub(crate) fn proto_aries_require_arms<T: Ord + std::fmt::Debug>(
        generated: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
        arm: &dyn Fn(&mut Cpu, u32) -> T,
        want: &[T],
    ) {
        let mut seen = BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            run_generated(&mut c, generated);
            seen.insert(arm(&mut c, seed));
        }
        for w in want {
            assert!(seen.contains(w), "arm {w:?} never taken (seen {seen:?})");
        }
    }

    // --------------------------------------------------- ProtoAriesCreate

    /// `bufsize` for `seed % 6`, and the size the clamp keeps.
    pub(crate) const PROTO_ARIES_CREATE_SIZES: [(u64, u32); 6] = [
        (0x800, 0x1000),
        (0xffff_ffff_ffff_f000, 0x1000),
        (0x1801, 0x1801),
        (0x2000, 0x2000),
        (0x2001, 0x2000),
        (0x0000_0001_0000_1000, 0x2000),
    ];

    /// The new object: two placements whose address blocks round to
    /// different lines.
    pub(crate) fn proto_aries_create_object(seed: u32) -> u32 {
        PROTO_ARIES_NEW + 0x28 * (seed / 24 % 2)
    }

    /// `seed % 6`: the size (below the minimum, negative, in range, the
    /// maximum, above it, above it only in the upper word). `seed / 6 % 4`:
    /// the allocation fails; no server; a server whose reply leaves no
    /// handle; a reply that leaves one (the DMA-id table is placed over the
    /// new object, so the id `DirtyRpc_Send` stores for this request lands in
    /// word 0 as the IOP's reply would). `seed / 24 % 2`: the placement.
    pub(crate) fn proto_aries_create_fixture(c: &mut Cpu, seed: u32) {
        let outcome = seed / 6 % 4;
        let mut r = proto_aries_socket_fixture(c, seed, 0x003f_5478, outcome >= 2);
        c.g[A0][0] = PROTO_ARIES_CREATE_SIZES[(seed % 6) as usize].0;
        let obj = proto_aries_create_object(seed);
        let span = 0x40 + PROTO_ARIES_OBJECT_SIZE + PROTO_ARIES_MAX_BUFFER as u32 + 0x40;
        fill(c, &mut r, obj - 0x40, span / 4);
        c.bus
            .write32(MEM_FAKES_CTRL + 0x18, if outcome == 0 { 0 } else { obj });
        if outcome == 3 {
            let head = c.bus.read32(PROTO_ARIES_RPC + PROTO_ARIES_RPC_HEAD);
            c.bus.write32(
                PROTO_ARIES_RPC + PROTO_ARIES_RPC_DMA_IDS,
                obj.wrapping_sub(4 * head),
            );
        }
    }

    /// (result, kept buffer size, request sent).
    pub(crate) fn proto_aries_create_arm(c: &mut Cpu, seed: u32) -> (&'static str, u32, bool) {
        let obj = proto_aries_create_object(seed);
        let size = c.bus.read32(obj + PROTO_ARIES_RECV_SIZE);
        let sent = proto_aries_sent(c, PROTO_ARIES_CREATE_REQUEST);
        if c.g[V0][0] != 0 {
            ("kept", size, sent)
        } else if (0x1000..=0x2000).contains(&size) {
            ("released", size, sent)
        } else {
            ("no memory", 0, sent)
        }
    }

    #[test]
    fn proto_aries_create_matches() {
        let setup = proto_aries_create_fixture;
        check(ee::s14::f_003f5478__generated, proto_aries_create, 48, &setup);
        let mut want: Vec<(&'static str, u32, bool)> = vec![("no memory", 0, false)];
        for size in [0x1000, 0x1801, 0x2000] {
            want.push(("kept", size, true));
            want.push(("released", size, false));
            want.push(("released", size, true));
        }
        proto_aries_require_arms(
            ee::s14::f_003f5478__generated,
            48,
            &setup,
            &proto_aries_create_arm,
            &want[..],
        );
    }

    // -------------------------------------------------- ProtoAriesDestroy

    /// `seed % 2`: no server / the server.
    #[test]
    fn proto_aries_destroy_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            proto_aries_socket_fixture(c, seed, 0x003f_55b8, seed % 2 == 1);
        };
        check(ee::s14::f_003f55b8__generated, proto_aries_destroy, 16, &setup);
        proto_aries_require_arms(
            ee::s14::f_003f55b8__generated,
            16,
            &setup,
            &|c, _| proto_aries_sent(c, PROTO_ARIES_DESTROY_REQUEST),
            &[false, true],
        );
    }

    // -------------------------------------------------- ProtoAriesConnect

    /// `seed % 2`: the peer family is 0 / already set (-1). `seed / 2 % 2`:
    /// no server / the server. Address and port carry junk upper halves.
    pub(crate) fn proto_aries_connect_fixture(c: &mut Cpu, seed: u32) {
        let mut r = proto_aries_socket_fixture(c, seed, 0x003f_5638, seed / 2 % 2 == 1);
        let family = if seed % 2 == 0 {
            0
        } else {
            r.next_u32() as u16 | 1
        };
        c.bus
            .write16(PROTO_ARIES_SOCK_ADDRS + PROTO_ARIES_PEER, family);
        c.g[A1][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        c.g[A2][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
    }

    #[test]
    fn proto_aries_connect_matches() {
        let setup = proto_aries_connect_fixture;
        // Already connected: 9 + 5.
        check_paths(ee::s14::f_003f5638__generated, proto_aries_connect, 32, &[14], &setup);
        proto_aries_require_arms(
            ee::s14::f_003f5638__generated,
            32,
            &setup,
            &|c, _| {
                let busy = c.g[V0][0] == u64::MAX;
                (busy, proto_aries_sent(c, PROTO_ARIES_CONNECT_REQUEST))
            },
            &[(true, false), (false, false), (false, true)],
        );
    }

    // ------------------------------------------------ ProtoAriesUnconnect

    /// `seed % 2`: no server / the server.
    #[test]
    fn proto_aries_unconnect_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            proto_aries_socket_fixture(c, seed, 0x003f_5738, seed % 2 == 1);
        };
        check(ee::s14::f_003f5738__generated, proto_aries_unconnect, 16, &setup);
        proto_aries_require_arms(
            ee::s14::f_003f5738__generated,
            16,
            &setup,
            &|c, _| proto_aries_sent(c, PROTO_ARIES_UNCONNECT_REQUEST),
            &[false, true],
        );
    }

    // --------------------------------------------------- ProtoAriesStatus

    /// `seed % 7`: `'port'`, `'addr'`, `'ladr'`, `'lprt'`, `'ibuf'`, another
    /// code, `'port'` with a non-zero upper word (the compare is 64-bit).
    /// `seed / 7 % 2`: no server / the server.
    pub(crate) fn proto_aries_status_fixture(c: &mut Cpu, seed: u32) {
        proto_aries_socket_fixture(c, seed, 0x003f_57c8, seed / 7 % 2 == 1);
        c.g[A1][0] = match seed % 7 {
            0 => PROTO_ARIES_SEL_PORT as u64,
            1 => PROTO_ARIES_SEL_ADDR as u64,
            2 => PROTO_ARIES_SEL_LOCAL_ADDR as u64,
            3 => PROTO_ARIES_SEL_LOCAL_PORT as u64,
            4 => PROTO_ARIES_SEL_INPUT as u64,
            5 => 0x7a7a_7a7a,
            _ => 1 << 32 | PROTO_ARIES_SEL_PORT as u64,
        };
    }

    #[test]
    fn proto_aries_status_matches() {
        let setup = proto_aries_status_fixture;
        // Without the server: 31 + 16 (InitRequest) + 51 (Call), then 'port'
        // 13, 'addr' 22, 'ladr' 24, 'lprt' 21, 'ibuf' 19, other 18.
        check_paths(
            ee::s14::f_003f57c8__generated,
            proto_aries_status,
            42,
            &[111, 120, 122, 119, 117, 116],
            &setup,
        );
        proto_aries_require_arms(
            ee::s14::f_003f57c8__generated,
            42,
            &setup,
            &|c, _| proto_aries_sent(c, PROTO_ARIES_STATUS_REQUEST),
            &[false, true],
        );
    }

    // ---------------------------------------------------- ProtoAries_Nop

    #[test]
    fn proto_aries_nop_matches() {
        check(ee::s14::f_003f5908__generated, proto_aries_nop, 8, &|c, seed| {
            let _r = machine(c, seed, 0x3f5908, 0);
            c.pc = 0x003f_5908;
        });
    }

    // ----------------------------------------------------- ProtoAriesSend

    /// `seed % 3`: a length (0..=0x30), a negative length (the data is a
    /// string: `strlen + 1`), 0x8000_0000 zero-extended (non-negative as a
    /// 64-bit value). `seed / 3 % 2`: no server / the server. `seed / 6 % 4`:
    /// the string length (0, 5, 10, 15). The header words carry junk upper
    /// halves.
    pub(crate) fn proto_aries_send_fixture(c: &mut Cpu, seed: u32) {
        let mut r = proto_aries_socket_fixture(c, seed, 0x003f_5910, seed / 3 % 2 == 1);
        for i in 0..0x10 {
            c.bus
                .write32(PROTO_ARIES_PAYLOAD + 4 * i, r.next_u32() | 0x0101_0101);
        }
        c.bus.write8(PROTO_ARIES_PAYLOAD + seed / 6 % 4 * 5, 0);
        c.g[A1][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        c.g[A2][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        c.g[A3][0] = sx(PROTO_ARIES_PAYLOAD);
        c.g[T0][0] = match seed % 3 {
            0 => (r.next_u32() % 0x31) as u64,
            1 => sx(r.next_u32() | 0x8000_0000),
            _ => 0x8000_0000,
        };
    }

    /// (length form, the header's length right for the string form, sent).
    pub(crate) fn proto_aries_send_arm(c: &mut Cpu, seed: u32) -> (&'static str, bool, bool) {
        let header = crate::difftest::SP - 0xc0 + 0x10;
        let total = proto_aries_get_be(c, header + PROTO_ARIES_RECORD_LEN);
        let sent = proto_aries_sent(c, PROTO_ARIES_SEND_REQUEST);
        if seed % 3 == 1 {
            (
                "string",
                total == seed / 6 % 4 * 5 + 1 + PROTO_ARIES_RECORD_HEADER,
                sent,
            )
        } else {
            ("given", true, sent)
        }
    }

    #[test]
    fn proto_aries_send_matches() {
        let setup = proto_aries_send_fixture;
        // A given length without the server: 17 + 2 (Nop) + 59 + 16 + 51.
        check_paths(ee::s14::f_003f5910__generated, proto_aries_send, 48, &[145], &setup);
        proto_aries_require_arms(
            ee::s14::f_003f5910__generated,
            48,
            &setup,
            &proto_aries_send_arm,
            &[
                ("given", true, false),
                ("given", true, true),
                ("string", true, false),
                ("string", true, true),
            ],
        );
    }

    // ----------------------------------------------------- ProtoAriesPeek

    /// `seed % 4`: a record at the read offset; nothing unread with a receive
    /// pending; nothing unread after a reply with bytes (the buffer is
    /// dropped and rewound, then a new receive); nothing unread and the fill
    /// word 0 or negative (a new receive only). For a record, `seed / 4 % 4`:
    /// its length (a payload of 1..=0x20 bytes, exactly the header, shorter
    /// than the header, a huge one that is negative as a word) and
    /// `seed / 16 % 8`: which of the three outputs are non-null. Otherwise
    /// `seed / 4 % 2`: no server / the server, and `seed / 8 % 2` the read
    /// offset equal to or past the fill (pending, drained) or the fill 0 or
    /// negative.
    pub(crate) fn proto_aries_peek_fixture(c: &mut Cpu, seed: u32) {
        let state = seed % 4;
        let server = state >= 2 && seed / 4 % 2 == 1;
        let mut r = proto_aries_socket_fixture(c, seed, 0x003f_5a70, server);
        fill(c, &mut r, PROTO_ARIES_OUT, 4);
        let outs = seed / 16 % 8;
        c.g[A1][0] = if outs & 1 != 0 {
            sx(PROTO_ARIES_OUT)
        } else {
            0
        };
        c.g[A2][0] = if outs & 2 != 0 {
            sx(PROTO_ARIES_OUT + 4)
        } else {
            0
        };
        c.g[A3][0] = if outs & 4 != 0 {
            sx(PROTO_ARIES_OUT + 8)
        } else {
            0
        };
        let (read, filled, pending) = match state {
            0 => {
                let read = r.next_u32() % 8 * 0x10;
                let len = match seed / 4 % 4 {
                    0 => PROTO_ARIES_RECORD_HEADER + 1 + r.next_u32() % 0x20,
                    1 => PROTO_ARIES_RECORD_HEADER,
                    2 => 5,
                    _ => 0xffff_fff0,
                };
                proto_aries_put_be(c, PROTO_ARIES_SOCK_BUF + read + PROTO_ARIES_RECORD_LEN, len);
                (read, read + 0x100, r.next_u32())
            }
            1 => (0x40, 0x40 - 4 * (seed / 8 % 2), r.next_u32() | 1),
            2 => (0x40 + 0x10 * (seed / 8 % 2), 0x40, 0),
            _ => (0x10, if seed / 8 % 2 == 0 { 0 } else { 0xffff_ffe0 }, 0),
        };
        let sock = PROTO_ARIES_SOCK;
        c.bus.write32(sock + PROTO_ARIES_RECV_READ, read);
        c.bus.write32(sock + PROTO_ARIES_RECV_FILL, filled);
        c.bus.write32(sock + PROTO_ARIES_RECV_PENDING, pending);
    }

    /// (the buffer was rewound, a receive was sent).
    pub(crate) fn proto_aries_peek_arm(c: &mut Cpu, _seed: u32) -> (bool, bool) {
        let sock = PROTO_ARIES_SOCK;
        let rewound = c.g[V0][0] == u64::MAX
            && c.bus.read32(sock + PROTO_ARIES_RECV_FILL) == 0
            && c.bus.read32(sock + PROTO_ARIES_RECV_READ) == 0;
        (rewound, proto_aries_sent(c, PROTO_ARIES_PEEK_REQUEST))
    }

    #[test]
    fn proto_aries_peek_matches() {
        let setup = proto_aries_peek_fixture;
        // A record: 44 with a payload, 46 without, + 11 per non-null header
        // output (the data output costs nothing extra). Pending: 20. A new
        // receive without the server and without a rewind: 119.
        check_paths(
            ee::s14::f_003f5a70__generated,
            proto_aries_peek,
            128,
            &[44, 55, 66, 46, 57, 68, 20, 119],
            &setup,
        );
        proto_aries_require_arms(
            ee::s14::f_003f5a70__generated,
            128,
            &setup,
            &proto_aries_peek_arm,
            &[(true, false), (true, true), (false, true)],
        );
    }

    // ----------------------------------------------------- ProtoAriesRecv

    /// `seed % 3`: a record; a record shorter than its header (Peek's
    /// negative length); nothing unread with a receive pending (-1). For a
    /// record, `seed / 3 % 4`: after it, more is unread; the buffer is drained
    /// with a receive pending; drained after a reply with bytes (rewind and a
    /// new receive); drained with the fill word negative (a record ending at
    /// the buffer start: a new receive only). `seed / 12 % 2`: no destination
    /// / a destination. `seed / 24 % 4`: (payload, len) = (0x15, 0x12),
    /// (0x15, 0x15), (0x15, 0x40), (0, 4). `seed / 96 % 2`: no server / the
    /// server. For the short record `seed / 3 % 4` picks its length.
    pub(crate) fn proto_aries_recv_fixture(c: &mut Cpu, seed: u32) {
        let after = seed / 3 % 4;
        let mut r = proto_aries_socket_fixture(c, seed, 0x003f_5c38, seed / 96 % 2 == 1);
        fill(c, &mut r, PROTO_ARIES_OUT, 2);
        fill(c, &mut r, PROTO_ARIES_PAYLOAD, 0x20);
        c.g[A1][0] = sx(PROTO_ARIES_OUT);
        c.g[A2][0] = sx(PROTO_ARIES_OUT + 4);
        c.g[A3][0] = if seed / 12 % 2 == 1 {
            sx(PROTO_ARIES_PAYLOAD)
        } else {
            0
        };
        let buf = PROTO_ARIES_SOCK_BUF;
        let (read, filled, pending, len) = match seed % 3 {
            0 => {
                let (payload, len) = match seed / 24 % 4 {
                    0 => (0x15, 0x12),
                    1 => (0x15, 0x15),
                    2 => (0x15, 0x40),
                    _ => (0, 4),
                };
                let record = PROTO_ARIES_RECORD_HEADER + payload;
                let read = if after == 3 {
                    record.wrapping_neg()
                } else {
                    0x20
                };
                let at = buf.wrapping_add(read) + PROTO_ARIES_RECORD_LEN;
                proto_aries_put_be(c, at, record);
                let (filled, pending) = match after {
                    0 => (read + record + 0x10, r.next_u32()),
                    1 => (read + record, r.next_u32() | 1),
                    2 => (read + record, 0),
                    _ => (0xffff_fffc, 0),
                };
                (read, filled, pending, len)
            }
            1 => {
                proto_aries_put_be(c, buf + 0x20 + PROTO_ARIES_RECORD_LEN, after * 3);
                (0x20, 0x80, r.next_u32(), 0x10)
            }
            _ => (0x40, 0x40, r.next_u32() | 1, 0x10),
        };
        let sock = PROTO_ARIES_SOCK;
        c.bus.write32(sock + PROTO_ARIES_RECV_READ, read);
        c.bus.write32(sock + PROTO_ARIES_RECV_FILL, filled);
        c.bus.write32(sock + PROTO_ARIES_RECV_PENDING, pending);
        c.g[T0][0] = len as u64;
    }

    /// (negative result, the buffer was rewound, a receive was sent).
    pub(crate) fn proto_aries_recv_arm(c: &mut Cpu, _seed: u32) -> (bool, bool, bool) {
        let sock = PROTO_ARIES_SOCK;
        let none = (c.g[V0][0] as i64) < 0;
        let rewound = c.bus.read32(sock + PROTO_ARIES_RECV_FILL) == 0
            && c.bus.read32(sock + PROTO_ARIES_RECV_READ) == 0;
        (none, rewound, proto_aries_sent(c, PROTO_ARIES_RECV_REQUEST))
    }

    #[test]
    fn proto_aries_recv_matches() {
        let setup = proto_aries_recv_fixture;
        // No destination, no server: pending 14 + 20 + 1 + 7 = 42; the short
        // record 14 + 68 + 8 = 90; a record (Peek 66, or 68 for an empty
        // payload) then 32 more unread, 35 pending, 133 a new receive.
        check_paths(
            ee::s14::f_003f5c38__generated,
            proto_aries_recv,
            192,
            &[42, 90, 98, 101, 199, 100, 103, 201],
            &setup,
        );
        proto_aries_require_arms(
            ee::s14::f_003f5c38__generated,
            192,
            &setup,
            &proto_aries_recv_arm,
            &[
                (true, false, false),
                (false, false, false),
                (false, true, false),
                (false, true, true),
                (false, false, true),
            ],
        );
    }

    // ----------------------------------------------------- ProtoAriesTick

    #[test]
    fn proto_aries_tick_matches() {
        check(ee::s14::f_003f5d88__generated, proto_aries_tick, 8, &|c, seed| {
            let mut r = machine(c, seed, 0x3f5d88, 0);
            c.pc = 0x003f_5d88;
            c.bus
                .write32(PROTO_ARIES_RPC + PROTO_ARIES_RPC_CLOCK, r.next_u32());
            let base = (r.next_u32() as u64) << 16 | r.next_u32() as u64;
            c.bus
                .write64(PROTO_ARIES_RPC + PROTO_ARIES_RPC_CLOCK_BASE, base);
        });
    }
}
