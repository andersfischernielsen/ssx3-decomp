//! Staging module oc1100 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1100`.

use crate::abi::ret;
use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003e_66f0, net_game_link_create, "NetGameLinkCreate"),
    (0x003e_67f8, net_game_link_destroy, "NetGameLinkDestroy"),
    (
        0x003e_6878,
        net_game_link_copy_stats,
        "NetGameLink_CopyStats",
    ),
    (0x003e_6958, net_game_link_status, "NetGameLinkStatus"),
    (
        0x003e_6a70,
        net_game_link_peek_packet,
        "NetGameLink_PeekPacket",
    ),
    (0x003e_6b10, net_game_link_recv, "NetGameLinkRecv"),
    (0x003e_6c80, net_game_link_send, "NetGameLinkSend"),
    (0x003e_6d78, net_game_link_control, "NetGameLinkControl"),
    (0x003e_6e08, net_game_util_create, "NetGameUtilCreate"),
    (0x003e_6e70, net_game_util_destroy, "NetGameUtilDestroy"),
    (0x003e_6ed8, net_game_util_reset, "NetGameUtilReset"),
    (0x003e_6f40, net_game_util_connect, "NetGameUtilConnect"),
    (
        0x003e_6fc8,
        net_game_util_complete,
        "NetGameUtilComplete",
    ),
    (0x003e_7038, net_game_util_status, "NetGameUtilStatus"),
    (0x003e_70b8, net_game_util_stat, "NetGameUtilStat"),
    (0x003e_7120, net_game_util_advert, "NetGameUtilAdvert"),
    (
        0x003e_71b0,
        net_game_util_withdraw,
        "NetGameUtilWithdraw",
    ),
    (0x003e_7218, net_game_util_locate, "NetGameUtilLocate"),
    (0x003e_72a0, net_game_util_query, "NetGameUtilQuery"),
    (0x003e_7340, proto_ping_create, "ProtoPingCreate"),
];

// ---------------------------------------------------------------------------
// Direct-call targets (same-shard direct calls the generated bodies make as
// `c.pc = ADDR; Name(c)`). Cross-shard calls (the generator emits those as
// `call_indirect`) go through `call_ptr` at the call site.

pub(crate) mod callee {
    use super::Callee;
    use super::ee;

    pub const DIRTY_RPC_INIT_REQUEST: Callee = Callee {
        addr: 0x003f_4c98,
        f: ee::s14::DirtyRpc_InitRequest,
    };
    pub const DIRTY_RPC_CALL: Callee = Callee {
        addr: 0x003f_4c68,
        f: ee::s14::DirtyRpc_Call,
    };
    pub const DIRTY_RPC_SYNC_DCACHE: Callee = Callee {
        addr: 0x003f_4e80,
        f: ee::s14::DirtyRpc_SyncDCache,
    };
    pub const NET_TICK: Callee = Callee {
        addr: 0x003f_4de8,
        f: ee::s14::NetTick,
    };
    pub const NET_GAME_LINK_CHECK_RECV_BUF: Callee = Callee {
        addr: 0x003f_4ea0,
        f: ee::s14::NetGameLink_CheckRecvBuf,
    };
    pub const NET_GAME_LINK_COPY_STATS: Callee = Callee {
        addr: 0x003e_6878,
        f: ee::s14::NetGameLink_CopyStats,
    };
    pub const NET_GAME_LINK_PEEK_PACKET: Callee = Callee {
        addr: 0x003e_6a70,
        f: ee::s14::NetGameLink_PeekPacket,
    };
}

// ---------------------------------------------------------------------------
// NetGameLink* (link-object lifecycle and ring-buffer I/O).

/// `NetGameLink* NetGameLinkCreate(int ctx, int unused, int size)` (0x3e66f0):
/// allocate a `size + 0x394`-byte link object, zero it, derive two
/// 64-byte-aligned buffer pointers from its base and issue the `gmlb/crea`
/// RPC. Returns the object.
pub fn net_game_link_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 66;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S2, 0x40);
        frame.save(c, S1, 0x50);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S3, 0x30);
        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x394));
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        frame.save_ra(c, 0x20);
        c.g[RA][0] = sx(0x003e_671c);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0025_6bb0, 0x003e_671c)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x003e_6730);
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x0041_6210, 0x003e_6730)?;
        c.g[V1][0] = 0xffffffffffffffc0;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1bf));
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[A0][0] = 0xffffffffffffffff;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x16c), c.g[V0][0]);
        c.g[A1][0] = 0x150;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[A0][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3f));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x168), c.g[S2][0]);
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x174), c.g[S2][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x158), c.g[V0][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x18f));
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x158));
        c.g[RA][0] = sx(0x003e_6778);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x17c), c.g[V0][0]);
        callee::DIRTY_RPC_SYNC_DCACHE.call(c, 0x003e_6778)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x16c));
        c.g[RA][0] = sx(0x003e_6784);
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x168));
        callee::DIRTY_RPC_SYNC_DCACHE.call(c, 0x003e_6784)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x17c));
        c.g[RA][0] = sx(0x003e_6790);
        c.g[A1][0] = 0x4;
        callee::DIRTY_RPC_SYNC_DCACHE.call(c, 0x003e_6790)?;
        c.g[A1][0] = 0x676d0000;
        c.g[A2][0] = 0x63720000;
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[A1][0] | 0x6c6b;
        c.g[RA][0] = sx(0x003e_67a8);
        c.g[A2][0] = c.g[A2][0] | 0x6561;
        callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_67a8)?;
        c.g[V1][0] = 0x1;
        c.g[A2][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S2][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[S0][0]);
        c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x17c));
        c.sw(c.g[SP][0] as u32, c.g[V1][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = 0x1;
        c.g[T1][0] = 0x4;
        c.g[T2][0] = 0u64;
        c.g[RA][0] = sx(0x003e_67d8);
        c.g[T3][0] = 0u64;
        callee::DIRTY_RPC_CALL.call(c, 0x003e_67d8)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x50);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void NetGameLinkDestroy(NetGameLink* link)` (0x3e67f8): issue the
/// `gmlb/dest` RPC for the link, then free the object.
pub fn net_game_link_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = 0x676d0000;
        frame.save(c, S0, 0x30);
        c.g[A2][0] = 0x64650000;
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0x520000;
        c.g[S1][0] = c.g[A0][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1168));
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[A1][0] | 0x6c6b;
        c.g[RA][0] = sx(0x003e_682c);
        c.g[A2][0] = c.g[A2][0] | 0x7374;
        callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_682c)?;
        c.g[A2][0] = c.lw(c.g[S1][0] as u32);
        c.g[V0][0] = 0x1;
        c.g[A0][0] = c.g[S0][0];
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0u64;
        c.g[T1][0] = 0u64;
        c.g[T2][0] = 0u64;
        c.g[T3][0] = 0u64;
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        c.g[A1][0] = 0u64;
        c.g[RA][0] = sx(0x003e_685c);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
        callee::DIRTY_RPC_CALL.call(c, 0x003e_685c)?;
        c.g[RA][0] = sx(0x003e_6864);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0025_6bd8, 0x003e_6864)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `long NetGameLink_CopyStats(NetGameLink* link)` (0x3e6878): copy 0x150
/// bytes from the link's buffer (word at +0x158, masked and made uncached)
/// to the stat area at `link + 8`, with a 32-byte unaligned loop when
/// `(src | dst) & 7 != 0` and an 8-byte aligned loop otherwise, then a
/// 16-byte tail; clear the word at `link + 4`. Returns 0.
pub fn net_game_link_copy_stats(c: &mut Cpu) {
    loop {
        match c.pc {
            0x003e_6878 => {
                c.cycles += 12;
                c.g[A2][0] = c.lw(c.g[SP][0] as u32);
                c.g[V0][0] = 0xfff0000;
                c.g[V0][0] = c.g[V0][0] | 0xffff;
                c.g[A0][0] = 0x20000000;
                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x158));
                c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
                c.g[V0][0] = c.g[V1][0] | c.g[A1][0];
                c.g[V0][0] = c.g[V0][0] & 7;
                let aligned = c.g[V0][0] == 0;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x140));
                c.pc = if aligned { 0x003e_68fc } else { 0x003e_68a8 };
            }
            0x003e_68a8 => {
                c.cycles += 19;
                c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.ldr(4, c.g[V1][0] as u32);
                c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0xf));
                c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x17));
                c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x10));
                c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x1f));
                c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x18));
                c.sdl(4, (c.g[A1][0] as u32).wrapping_add(0x7));
                c.sdr(4, c.g[A1][0] as u32);
                c.sdl(7, (c.g[A1][0] as u32).wrapping_add(0xf));
                c.sdr(7, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0x17));
                c.sdr(8, (c.g[A1][0] as u32).wrapping_add(0x10));
                c.sdl(9, (c.g[A1][0] as u32).wrapping_add(0x1f));
                c.sdr(9, (c.g[A1][0] as u32).wrapping_add(0x18));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                let done = c.g[V1][0] == c.g[V0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                if !done {
                    if event_due(c) {
                        deliver_event_here(c, 0x003e_68a8);
                    }
                    c.pc = 0x003e_68a8;
                } else {
                    c.cycles += 2;
                    c.pc = 0x003e_6928;
                }
            }
            0x003e_68fc => {
                c.cycles += 11;
                c.g[A0][0] = c.ld(c.g[V1][0] as u32);
                c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                c.sd(c.g[A1][0] as u32, c.g[A0][0]);
                c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                let done = c.g[V1][0] == c.g[V0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                if !done {
                    if event_due(c) {
                        deliver_event_here(c, 0x003e_68fc);
                    }
                    c.pc = 0x003e_68fc;
                } else {
                    c.pc = 0x003e_6928;
                }
            }
            0x003e_6928 => {
                c.cycles += 11;
                c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.ldr(2, c.g[V1][0] as u32);
                c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
                c.ldr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.sdl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
                c.sdr(2, c.g[A1][0] as u32);
                c.sdl(4, (c.g[A1][0] as u32).wrapping_add(0xf));
                c.sdr(4, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = 0u64;
                c.sw((c.g[A2][0] as u32).wrapping_add(0x4), 0u64);
                ret(c);
                return;
            }
            pc => {
                c.jump_out(pc, 0x003e_6878, 0x003e_6954);
                return;
            }
        }
    }
}

/// `long NetGameLinkStatus(NetGameLink* link)` (0x3e6958): refresh the
/// link's stat area and return `link + 8`. A fresh link (word at +0x4 is
/// -1) is cleared and takes the copy path (`gmlb/stat` RPC plus
/// `NetGameLink_CopyStats`); a zero word takes the RPC-only path; any other
/// value returns at once.
pub fn net_game_link_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0xffffffffffffffff;
        frame.save(c, S0, 0x50);
        frame.save(c, S3, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0x10);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        let stale = c.g[V1][0] != c.g[V0][0];
        c.g[S3][0] = 0u64;
        if stale {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            let pending = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
            if pending {
                c.cycles += 7;
                frame.restore(c, S1, 0x40);
                frame.restore(c, S0, 0x50);
                frame.restore(c, S2, 0x30);
                frame.restore(c, S3, 0x20);
                frame.restore_ra(c, 0x10);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 12;
            c.g[S2][0] = 0x1;
            c.g[V0][0] = 0x520000;
            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1158));
            c.g[A1][0] = 0x676d0000;
            c.g[A2][0] = 0x73740000;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[S2][0]);
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[A1][0] | 0x6c6b;
            c.g[RA][0] = sx(0x003e_69c0);
            c.g[A2][0] = c.g[A2][0] | 0x6174;
            callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_69c0)?;
            let copied = c.g[S3][0] != 0;
            c.g[A2][0] = c.lw(c.g[S0][0] as u32);
            if !copied {
                c.cycles += 14;
                c.g[V0][0] = 0x3e0000;
                c.g[T2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x158));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6878));
                c.sw(c.g[SP][0] as u32, c.g[V0][0]);
                c.g[A0][0] = c.g[S1][0];
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[S0][0]);
                c.g[A1][0] = 0u64;
                c.g[A3][0] = 0u64;
                c.g[T0][0] = 0u64;
                c.g[T1][0] = 0u64;
                c.g[RA][0] = sx(0x003e_69f8);
                c.g[T3][0] = 0x150;
                callee::DIRTY_RPC_CALL.call(c, 0x003e_69f8)?;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
            } else {
                c.cycles += 21;
                c.g[A0][0] = c.g[S1][0];
                c.g[T2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x158));
                c.g[A1][0] = 0x1;
                c.g[A3][0] = 0u64;
                c.g[T0][0] = 0u64;
                c.g[T1][0] = 0u64;
                c.g[T3][0] = 0x150;
                c.sw(c.g[SP][0] as u32, c.g[S2][0]);
                c.g[RA][0] = sx(0x003e_6a28);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                callee::DIRTY_RPC_CALL.call(c, 0x003e_6a28)?;
                c.sw(c.g[SP][0] as u32, c.g[S0][0]);
                c.g[A0][0] = 0u64;
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                c.g[T0][0] = 0u64;
                c.g[T1][0] = 0u64;
                c.g[T2][0] = 0u64;
                c.g[RA][0] = sx(0x003e_6a50);
                c.g[T3][0] = 0u64;
                callee::NET_GAME_LINK_COPY_STATS.call(c, 0x003e_6a50)?;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
            }
        } else {
            c.cycles += 2;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
            c.g[S3][0] = 0x1;
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            let pending = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
            if !pending {
                c.cycles += 12;
                c.g[S2][0] = 0x1;
                c.g[V0][0] = 0x520000;
                c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1158));
                c.g[A1][0] = 0x676d0000;
                c.g[A2][0] = 0x73740000;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[S2][0]);
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[A1][0] | 0x6c6b;
                c.g[RA][0] = sx(0x003e_69c0);
                c.g[A2][0] = c.g[A2][0] | 0x6174;
                callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_69c0)?;
                let copied = c.g[S3][0] != 0;
                c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                if copied {
                    c.cycles += 21;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[T2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x158));
                    c.g[A1][0] = 0x1;
                    c.g[A3][0] = 0u64;
                    c.g[T0][0] = 0u64;
                    c.g[T1][0] = 0u64;
                    c.g[T3][0] = 0x150;
                    c.sw(c.g[SP][0] as u32, c.g[S2][0]);
                    c.g[RA][0] = sx(0x003e_6a28);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                    callee::DIRTY_RPC_CALL.call(c, 0x003e_6a28)?;
                    c.sw(c.g[SP][0] as u32, c.g[S0][0]);
                    c.g[A0][0] = 0u64;
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0u64;
                    c.g[T0][0] = 0u64;
                    c.g[T1][0] = 0u64;
                    c.g[T2][0] = 0u64;
                    c.g[RA][0] = sx(0x003e_6a50);
                    c.g[T3][0] = 0u64;
                    callee::NET_GAME_LINK_COPY_STATS.call(c, 0x003e_6a50)?;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `long NetGameLink_PeekPacket(NetGameLink* link, void** out)` (0x3e6a70):
/// if a packet is queued, copy its 4-byte header into `*out` and `memcpy`
/// its body after it; store 0 to `*out` when there is nothing (or no `out`).
/// Returns `write cursor - read cursor` at +0x164/+0x160.
pub fn net_game_link_peek_packet(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let have_out = c.g[A1][0] != 0;
        c.g[S0][0] = c.g[A0][0];
        // The packet arm breaks straight out to the tail; the other arms
        // rejoin through the inner breaks (store 0, reload the read cursor).
        let mut packet = false;
        if have_out {
            c.cycles += 5;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x160));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x164));
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let empty = c.g[V0][0] == 0;
            c.g[V1][0] = 0xfff0000;
            if !empty {
                c.cycles += 21;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x16c));
                c.g[V1][0] = c.g[V1][0] | 0xffff;
                c.g[A3][0] = 0x20000000;
                c.g[V0][0] =
                    sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[A0][0] = c.lw(c.g[A1][0] as u32);
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
                c.lwl(6, (c.g[V0][0] as u32).wrapping_add(0x3));
                c.lwr(6, c.g[V0][0] as u32);
                c.swl(6, (c.g[A0][0] as u32).wrapping_add(0x3));
                c.swr(6, c.g[A0][0] as u32);
                c.g[A0][0] = c.lw(c.g[A1][0] as u32);
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x16c));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x160));
                c.g[A2][0] = c.lh(c.g[A0][0] as u32);
                c.g[A1][0] =
                    sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A1][0] = c.g[A1][0] & c.g[V1][0];
                c.g[RA][0] = sx(0x003e_6ae4);
                c.g[A1][0] = c.g[A1][0] | c.g[A3][0];
                call_ptr(c, 0x0041_605c, 0x003e_6ae4)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x160));
                packet = true;
            } else {
                c.cycles += 1;
                c.sw(c.g[A1][0] as u32, 0u64);
            }
        } else {
            // No out pointer: rejoin with the read-cursor reload, no store.
        }
        if !packet {
            c.cycles += 1;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x160));
        }
        c.cycles += 6;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x164));
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        frame.close(c);
        Ok(())
    })
}

/// `long NetGameLinkRecv(NetGameLink* link, void* out, long maxlen)`
/// (0x3e6b10): drain queued bytes into `out`. With an invalid write cursor
/// the receive buffer is refilled first (`NetGameLink_CheckRecvBuf`); with
/// `maxlen == 1` a single packet is peeked and the read cursor advanced past
/// it, otherwise up to `maxlen` bytes are copied. When the cursors meet, the
/// link is reset (`NetTick` plus the `gmlb/recv` RPC). Returns the bytes
/// moved (or 0).
pub fn net_game_link_recv(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[A1][0]);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x164));
        let refill = (c.g[V0][0] as i64) < 0;
        c.g[S2][0] = 0u64;
        if refill {
            c.cycles += 6;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x16c));
            c.g[RA][0] = sx(0x003e_6b48);
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            callee::NET_GAME_LINK_CHECK_RECV_BUF.call(c, 0x003e_6b48)?;
            c.g[A0][0] = c.g[V0][0];
            if (c.g[A0][0] as i64) < 0 {
                // Refill refused: rejoin at drain dispatch (no tail charge).
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x160));
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x15c));
                let same = c.g[A0][0] == c.g[V0][0];
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                if !same {
                    c.cycles += 5;
                    c.g[V0][0] = 0x10;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x160), c.g[V0][0]);
                    c.g[V1][0] =
                        sx((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x15c), c.g[A0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x164), c.g[V1][0]);
                }
                // Both refill arms rejoin through the inner break: reload the
                // read cursor (the refill arm just stored it).
                c.cycles += 1;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x160));
            }
        } else {
            c.cycles += 1;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x160));
        }
        c.cycles += 4;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x164));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
        let drained = c.g[V0][0] == 0;
        c.g[V0][0] = 0x1;
        if !drained {
            c.cycles += 2;
            let peek = c.g[S0][0] == c.g[V0][0];
            c.g[A0][0] =
                sx((c.g[A0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            if peek {
                c.cycles += 9;
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x003e_6b9c);
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x14));
                callee::NET_GAME_LINK_PEEK_PACKET.call(c, 0x003e_6b9c)?;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x160));
                c.g[S2][0] = c.lh(c.g[V1][0] as u32);
                c.g[V0][0] =
                    sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.sw((c.g[S1][0] as u32).wrapping_add(0x160), c.g[V0][0]);
                // Peek arm rejoins through the cursor-reload tail.
                c.cycles += 1;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x160));
            } else {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x160));
                } else {
                    c.cycles += 13;
                    c.g[A1][0] =
                        c.lw((c.g[S1][0] as u32).wrapping_add(0x16c));
                    c.g[V0][0] = 0xfff0000;
                    c.g[S2][0] = c.g[A0][0];
                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                    c.g[A1][0] = sx(
                        (c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32),
                    );
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                    c.g[V1][0] = 0x20000000;
                    c.g[A1][0] = c.g[A1][0] & c.g[V0][0];
                    c.g[A1][0] = c.g[A1][0] | c.g[V1][0];
                    c.g[RA][0] = sx(0x003e_6bec);
                    c.g[A2][0] = c.g[S2][0];
                    call_ptr(c, 0x0041_605c, 0x003e_6bec)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x164));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x160), c.g[V1][0]);
                    // Memcpy arm rejoins through the cursor-reload tail.
                    c.cycles += 1;
                    c.g[V1][0] =
                        c.lw((c.g[S1][0] as u32).wrapping_add(0x160));
                }
            }
        } else {
            c.cycles += 1;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x160));
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x164));
        let synced = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = c.g[S2][0];
        if !synced {
            // Cursors differ: done, return the moved count.
        } else {
            c.cycles += 25;
            c.g[V0][0] = 0xffffffffffffffff;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x160), 0u64);
            c.g[RA][0] = sx(0x003e_6c14);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x164), c.g[V0][0]);
            callee::NET_TICK.call(c, 0x003e_6c14)?;
            c.g[S0][0] = 0x520000;
            c.g[A1][0] = 0x676d0000;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1148));
            c.g[A2][0] = 0x72650000;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[A1][0] | 0x6c6b;
            c.g[A2][0] = c.g[A2][0] | 0x6376;
            c.g[RA][0] = sx(0x003e_6c38);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x170), c.g[V0][0]);
            callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_6c38)?;
            c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x16c));
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.lw(c.g[S1][0] as u32);
            c.g[A1][0] = 0x1;
            c.sw(c.g[SP][0] as u32, 0u64);
            c.g[T0][0] = 0u64;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
            c.g[T1][0] = 0u64;
            c.g[T2][0] = 0u64;
            c.g[RA][0] = sx(0x003e_6c64);
            c.g[T3][0] = 0u64;
            callee::DIRTY_RPC_CALL.call(c, 0x003e_6c64)?;
            c.g[V0][0] = c.g[S2][0];
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `long NetGameLinkSend(NetGameLink* link, void* buf, long len)`
/// (0x3e6c80): queue `len` bytes from `buf` for sending. With `len == 1`
/// the packet length prefix at `buf` is normalized first (low 13 bits of
/// `buf[3] + 0x17`). Returns 1 when the ring had room (and issues the
/// `gmlb/send` RPC), else 0.
pub fn net_game_link_send(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = 0x1;
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A1][0];
        let fixup = c.g[S1][0] == c.g[V0][0];
        frame.save_ra(c, 0x20);
        if fixup {
            c.cycles += 4;
            c.g[V0][0] = c.lbu((c.g[S3][0] as u32).wrapping_add(0x3));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x17));
            c.g[S1][0] = c.g[V0][0] & 0x7ffc;
            c.sh(c.g[S3][0] as u32, c.g[S1][0]);
        }
        c.cycles += 18;
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x17c));
        c.g[V0][0] = 0xfff0000;
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.g[A1][0] = 0x20000000;
        c.g[A0][0] = c.g[A0][0] & c.g[V0][0];
        c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x178));
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x174));
        c.g[A0][0] = c.g[A0][0] | c.g[A1][0];
        c.lwl(2, (c.g[A0][0] as u32).wrapping_add(0x3));
        c.lwr(2, c.g[A0][0] as u32);
        c.swl(2, (c.g[SP][0] as u32).wrapping_add(0x13));
        c.swr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[S1][0] as i64)) as u64;
        if c.g[V1][0] != 0 {
            c.g[V0][0] = 0u64;
        } else {
            c.cycles += 22;
            c.g[V0][0] =
                sx((c.g[A2][0] as u32).wrapping_add(c.g[S1][0] as u32));
            c.g[S0][0] = 0x520000;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1138));
            c.g[A1][0] = 0x676d0000;
            c.g[A2][0] = 0x73650000;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[A1][0] | 0x6c6b;
            c.g[A2][0] = c.g[A2][0] | 0x6e64;
            c.g[RA][0] = sx(0x003e_6d2c);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x178), c.g[V0][0]);
            callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_6d2c)?;
            c.g[A2][0] = c.lw(c.g[S2][0] as u32);
            c.g[A0][0] = c.g[S0][0];
            c.g[T0][0] = c.g[S3][0];
            c.g[T1][0] = c.g[S1][0];
            c.sw(c.g[SP][0] as u32, 0u64);
            c.g[A1][0] = 0x1;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
            c.g[A3][0] = 0u64;
            c.g[T2][0] = 0u64;
            c.g[RA][0] = sx(0x003e_6d58);
            c.g[T3][0] = 0u64;
            callee::DIRTY_RPC_CALL.call(c, 0x003e_6d58)?;
            c.g[V0][0] = 0x1;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `long NetGameLinkControl(NetGameLink* link, long op, long arg)`
/// (0x3e6d78): issue the `gmlb/ctrl` RPC with the link word, `op` and a
/// pointer to `arg`. Returns the reply word the RPC left on the frame.
pub fn net_game_link_control(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        c.g[S0][0] = 0x520000;
        frame.save(c, S1, 0x40);
        c.g[S2][0] = c.g[A1][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A2][0]);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1128));
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = 0x676d0000;
        c.g[A2][0] = 0x63740000;
        frame.save_ra(c, 0x20);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[A1][0] | 0x6c6b;
        c.g[RA][0] = sx(0x003e_6db8);
        c.g[A2][0] = c.g[A2][0] | 0x726c;
        callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_6db8)?;
        c.g[A2][0] = c.lw(c.g[S1][0] as u32);
        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x14));
        c.g[V1][0] = 0x1;
        c.g[A0][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S2][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.sw(c.g[SP][0] as u32, c.g[V1][0]);
        c.g[A1][0] = 0x1;
        c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[T1][0] = 0x4;
        c.g[T2][0] = 0u64;
        c.g[RA][0] = sx(0x003e_6dec);
        c.g[T3][0] = 0u64;
        callee::DIRTY_RPC_CALL.call(c, 0x003e_6dec)?;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// NetGameUtil* (matchmaking RPC wrappers).

/// `long NetGameUtilCreate()` (0x3e6e08): issue the `gmut/crea` RPC.
/// Returns the reply word the RPC left on the frame.
pub fn net_game_util_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = 0x676d0000;
        c.g[A2][0] = 0x63720000;
        frame.save_ra(c, 0x30);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[A1][0] | 0x7574;
        c.g[RA][0] = sx(0x003e_6e28);
        c.g[A2][0] = c.g[A2][0] | 0x6561;
        callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_6e28)?;
        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = 0x1;
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = 0x1;
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0u64;
        c.g[T1][0] = 0u64;
        c.g[T2][0] = 0u64;
        c.g[RA][0] = sx(0x003e_6e5c);
        c.g[T3][0] = 0u64;
        callee::DIRTY_RPC_CALL.call(c, 0x003e_6e5c)?;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void NetGameUtilDestroy(long ctx)` (0x3e6e70): issue the `gmut/dest`
/// RPC for `ctx`.
pub fn net_game_util_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = 0x676d0000;
        frame.save(c, S0, 0x30);
        c.g[A2][0] = 0x64650000;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[A1][0] | 0x7574;
        c.g[RA][0] = sx(0x003e_6e98);
        c.g[A2][0] = c.g[A2][0] | 0x7374;
        callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_6e98)?;
        c.g[V1][0] = 0x1;
        c.g[A2][0] = c.g[S0][0];
        c.sw(c.g[SP][0] as u32, c.g[V1][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
        c.g[A1][0] = 0x1;
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0u64;
        c.g[T1][0] = 0u64;
        c.g[T2][0] = 0u64;
        c.g[RA][0] = sx(0x003e_6ec8);
        c.g[T3][0] = 0u64;
        callee::DIRTY_RPC_CALL.call(c, 0x003e_6ec8)?;
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void NetGameUtilReset(long ctx)` (0x3e6ed8): issue the `gmut/rset`
/// RPC for `ctx`.
pub fn net_game_util_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = 0x676d0000;
        frame.save(c, S0, 0x30);
        c.g[A2][0] = 0x72730000;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[A1][0] | 0x7574;
        c.g[RA][0] = sx(0x003e_6f00);
        c.g[A2][0] = c.g[A2][0] | 0x6574;
        callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_6f00)?;
        c.g[V1][0] = 0x1;
        c.g[A2][0] = c.g[S0][0];
        c.sw(c.g[SP][0] as u32, c.g[V1][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
        c.g[A1][0] = 0x1;
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0u64;
        c.g[T1][0] = 0u64;
        c.g[T2][0] = 0u64;
        c.g[RA][0] = sx(0x003e_6f30);
        c.g[T3][0] = 0u64;
        callee::DIRTY_RPC_CALL.call(c, 0x003e_6f30)?;
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `long NetGameUtilConnect(long a, long b, long c)` (0x3e6f40): issue the
/// `gmut/conn` RPC. Returns the reply word the RPC left on the frame.
pub fn net_game_util_connect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x60);
        c.g[S2][0] = c.g[A2][0];
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = 0x676d0000;
        c.g[A2][0] = 0x636f0000;
        frame.save_ra(c, 0x30);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[A1][0] | 0x7574;
        c.g[RA][0] = sx(0x003e_6f78);
        c.g[A2][0] = c.g[A2][0] | 0x6e6e;
        callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_6f78)?;
        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = 0x1;
        c.g[A2][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[T0][0] = c.g[S2][0];
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = 0x1;
        c.g[T1][0] = 0xffffffffffffffff;
        c.g[T2][0] = 0u64;
        c.g[RA][0] = sx(0x003e_6fac);
        c.g[T3][0] = 0u64;
        callee::DIRTY_RPC_CALL.call(c, 0x003e_6fac)?;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `long NetGameUtilComplete(long ctx)` (0x3e6fc8): issue the `gmut/comp`
/// RPC. Returns the reply word the RPC left on the frame.
pub fn net_game_util_complete(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = 0x676d0000;
        frame.save(c, S0, 0x40);
        c.g[A2][0] = 0x636f0000;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[A1][0] | 0x7574;
        c.g[RA][0] = sx(0x003e_6ff0);
        c.g[A2][0] = c.g[A2][0] | 0x6d70;
        callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_6ff0)?;
        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = 0x1;
        c.g[A2][0] = c.g[S0][0];
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = 0x1;
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0u64;
        c.g[T1][0] = 0u64;
        c.g[T2][0] = 0u64;
        c.g[RA][0] = sx(0x003e_7024);
        c.g[T3][0] = 0u64;
        callee::DIRTY_RPC_CALL.call(c, 0x003e_7024)?;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
        frame.restore(c, S0, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `long NetGameUtilStatus(long ctx, long slot)` (0x3e7038): issue the
/// `gmut/stat` RPC. Returns the reply word the RPC left on the frame.
pub fn net_game_util_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x60);
        c.g[A2][0] = 0x73740000;
        frame.save(c, S1, 0x40);
        c.g[A2][0] = c.g[A2][0] | 0x6174;
        frame.save(c, S0, 0x50);
        c.g[S1][0] = c.g[A1][0];
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = 0x676d0000;
        frame.save_ra(c, 0x30);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = sx(0x003e_7068);
        c.g[A1][0] = c.g[A1][0] | 0x7574;
        callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_7068)?;
        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = 0x1;
        c.g[A2][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S1][0];
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = 0x1;
        c.g[T0][0] = 0u64;
        c.g[T1][0] = 0u64;
        c.g[T2][0] = 0u64;
        c.g[RA][0] = sx(0x003e_709c);
        c.g[T3][0] = 0u64;
        callee::DIRTY_RPC_CALL.call(c, 0x003e_709c)?;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void NetGameUtilStat(long ctx)` (0x3e70b8): issue the `gmut/stat` RPC
/// for `ctx` (no reply read back).
pub fn net_game_util_stat(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = 0x676d0000;
        frame.save(c, S0, 0x30);
        c.g[A2][0] = 0x73740000;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[A1][0] | 0x7574;
        c.g[RA][0] = sx(0x003e_70e0);
        c.g[A2][0] = c.g[A2][0] | 0x6174;
        callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_70e0)?;
        c.g[V1][0] = 0x1;
        c.g[A2][0] = c.g[S0][0];
        c.sw(c.g[SP][0] as u32, c.g[V1][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
        c.g[A1][0] = 0x1;
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0u64;
        c.g[T1][0] = 0u64;
        c.g[T2][0] = 0u64;
        c.g[RA][0] = sx(0x003e_7110);
        c.g[T3][0] = 0u64;
        callee::DIRTY_RPC_CALL.call(c, 0x003e_7110)?;
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void NetGameUtilAdvert(long ctx, const char* a, const char* b)`
/// (0x3e7120): format `a` and `b` with `"%s\t%s"` and issue the
/// `gmut/advt` RPC for `ctx` with the result.
pub fn net_game_util_advert(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x150);
        c.g[V0][0] = c.g[A1][0];
        frame.save(c, S1, 0x130);
        c.g[A3][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = 0x4a0000;
        frame.save(c, S0, 0x140);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        frame.save_ra(c, 0x120);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4860));
        c.g[RA][0] = sx(0x003e_7150);
        c.g[A2][0] = c.g[V0][0];
        call_ptr(c, 0x0041_7828, 0x003e_7150)?;
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[A1][0] = 0x676d0000;
        c.g[A2][0] = 0x61640000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[A1][0] | 0x7574;
        c.g[RA][0] = sx(0x003e_716c);
        c.g[A2][0] = c.g[A2][0] | 0x7674;
        callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_716c)?;
        c.g[V1][0] = 0x1;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.sw(c.g[SP][0] as u32, c.g[V1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
        c.g[A1][0] = 0x1;
        c.g[A3][0] = 0u64;
        c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[T1][0] = 0xffffffffffffffff;
        c.g[T2][0] = 0u64;
        c.g[RA][0] = sx(0x003e_719c);
        c.g[T3][0] = 0u64;
        callee::DIRTY_RPC_CALL.call(c, 0x003e_719c)?;
        frame.restore(c, S0, 0x140);
        frame.restore(c, S1, 0x130);
        frame.restore_ra(c, 0x120);
        frame.close(c);
        Ok(())
    })
}

/// `void NetGameUtilWithdraw(long ctx)` (0x3e71b0): issue the `gmut/with`
/// RPC for `ctx`.
pub fn net_game_util_withdraw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = 0x676d0000;
        frame.save(c, S0, 0x30);
        c.g[A2][0] = 0x77690000;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[A1][0] | 0x7574;
        c.g[RA][0] = sx(0x003e_71d8);
        c.g[A2][0] = c.g[A2][0] | 0x7468;
        callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_71d8)?;
        c.g[V1][0] = 0x1;
        c.g[A2][0] = c.g[S0][0];
        c.sw(c.g[SP][0] as u32, c.g[V1][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
        c.g[A1][0] = 0x1;
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0u64;
        c.g[T1][0] = 0u64;
        c.g[T2][0] = 0u64;
        c.g[RA][0] = sx(0x003e_7208);
        c.g[T3][0] = 0u64;
        callee::DIRTY_RPC_CALL.call(c, 0x003e_7208)?;
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `long NetGameUtilLocate(long ctx, const char* name)` (0x3e7218): issue
/// the `gmut/loca` RPC; a null `name` falls back to the default string.
/// Returns the reply word the RPC left on the frame.
pub fn net_game_util_locate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0x30);
        let have_name = c.g[S0][0] != 0;
        c.g[S1][0] = c.g[A0][0];
        if !have_name {
            c.cycles += 2;
            c.g[V0][0] = 0x4a0000;
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4868));
        }
        c.cycles += 25;
        c.g[A1][0] = 0x676d0000;
        c.g[A2][0] = 0x6c6f0000;
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[A1][0] | 0x7574;
        c.g[RA][0] = sx(0x003e_7254);
        c.g[A2][0] = c.g[A2][0] | 0x6361;
        callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_7254)?;
        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = 0x1;
        c.g[A2][0] = c.g[S1][0];
        c.g[T0][0] = c.g[S0][0];
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = 0x1;
        c.g[A3][0] = 0u64;
        c.g[T1][0] = 0xffffffffffffffff;
        c.g[T2][0] = 0u64;
        c.g[RA][0] = sx(0x003e_7288);
        c.g[T3][0] = 0u64;
        callee::DIRTY_RPC_CALL.call(c, 0x003e_7288)?;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `long NetGameUtilQuery(long ctx, const char* name, long a, long b)`
/// (0x3e72a0): issue the `gmut/quer` RPC; a null `name` falls back to the
/// default string. Returns the reply word the RPC left on the frame.
pub fn net_game_util_query(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x50);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x40);
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0x30);
        let have_name = c.g[S0][0] != 0;
        c.g[S3][0] = c.g[A3][0];
        if !have_name {
            c.cycles += 2;
            c.g[V0][0] = 0x4a0000;
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4868));
        }
        c.cycles += 27;
        c.g[A1][0] = 0x676d0000;
        c.g[A2][0] = 0x71750000;
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[A1][0] | 0x7574;
        c.g[RA][0] = sx(0x003e_72ec);
        c.g[A2][0] = c.g[A2][0] | 0x6572;
        callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_72ec)?;
        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = 0x1;
        c.g[A2][0] = c.g[S1][0];
        c.g[T0][0] = c.g[S0][0];
        c.g[T2][0] = c.g[S2][0];
        c.g[T3][0] = c.g[S3][0];
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = 0x5;
        c.g[A3][0] = 0u64;
        c.g[RA][0] = sx(0x003e_7320);
        c.g[T1][0] = 0xffffffffffffffff;
        callee::DIRTY_RPC_CALL.call(c, 0x003e_7320)?;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `ProtoPing* ProtoPingCreate()` (0x3e7340): allocate a 0x854-byte ping
/// object, zero it, align a pointer 64 bytes down from `obj + 0x53` into
/// `obj + 0x10` and issue the `ping/crea` RPC. Returns the object.
pub fn proto_ping_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0x20);
        c.g[RA][0] = sx(0x003e_7354);
        c.g[A0][0] = 0x854;
        call_ptr(c, 0x0025_6bb0, 0x003e_7354)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 0u64;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x003e_7368);
        c.g[A2][0] = 0x854;
        call_ptr(c, 0x0041_6210, 0x003e_7368)?;
        c.g[V1][0] = 0xffffffffffff0000;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x53));
        c.g[V1][0] = c.g[V1][0] | 0xffc0;
        c.g[A1][0] = 0x70690000;
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[A2][0] = 0x63720000;
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.g[A1][0] = c.g[A1][0] | 0x6e67;
        c.g[RA][0] = sx(0x003e_7394);
        c.g[A2][0] = c.g[A2][0] | 0x6561;
        callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_7394)?;
        c.g[V1][0] = 0x1;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
        c.sw(c.g[SP][0] as u32, c.g[V1][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[A1][0] = 0x1;
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0u64;
        c.g[T1][0] = 0u64;
        c.g[T2][0] = 0u64;
        c.g[RA][0] = sx(0x003e_73c8);
        c.g[T3][0] = 0u64;
        callee::DIRTY_RPC_CALL.call(c, 0x003e_73c8)?;
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        frame.restore(c, S0, 0x30);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule, Events};
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, elf_image, DATA, GAME_GP, RA};

    /// The main heap over 256 KiB of test RAM, so `Net_RpcAlloc` has an
    /// arena to allocate from.
    fn link_arena(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        c.g[A0][0] = 0x004f_f250;
        c.g[A1][0] = (DATA + 0x6_0000) as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[ssx3_ee_rt::cpu::reg::RA][0] = RA as u64;
        c.pc = 0x0031_ff60;
        ee::s10::DLHeap_InitArena(c);
        assert_eq!(c.pc, RA, "arena init did not return");
    }

    /// Write `s` and a terminating NUL at `at`.
    fn put_cstr(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    #[test]
    fn net_game_link_create_matches() {
        check(
            ee::s14::f_003e66f0__generated,
            net_game_link_create,
            24,
            &|c, seed| {
                machine(c, seed, 0x3e66f0, 0);
                link_arena(c);
                c.pc = 0x003e_66f0;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.bus.write32(DATA + 0x100, 0x5555_0000 + seed);
                c.g[A1][0] = (seed as u64).wrapping_mul(0x9e37);
                c.g[A2][0] = (0x40 + (seed % 4) * 0x40) as u64;
            },
        );
    }

    #[test]
    fn net_game_link_destroy_matches() {
        check(
            ee::s14::f_003e67f8__generated,
            net_game_link_destroy,
            16,
            &|c, seed| {
                machine(c, seed, 0x3e67f8, 0);
                link_arena(c);
                if seed % 2 == 0 {
                    // A live heap object, like the create path hands out.
                    c.g[A0][0] = 0x394;
                    c.g[ssx3_ee_rt::cpu::reg::RA][0] = RA as u64;
                    c.pc = 0x0025_6bb0;
                    ee::s06::f_00256bb0(c);
                    assert_eq!(c.pc, RA);
                    let link = c.g[V0][0] as u32;
                    for i in 0..8u32 {
                        c.bus.write32(link + 4 * i, 0xA500_0000 + seed + i);
                    }
                    c.g[A0][0] = link as u64;
                } else {
                    c.g[A0][0] = 0;
                }
                c.pc = 0x003e_67f8;
            },
        );
    }

    driven!(gen_copy_stats, ee::s14::f_003e6878__generated);
    driven!(hand_copy_stats, net_game_link_copy_stats);

    #[test]
    fn net_game_link_copy_stats_events_matches() {
        check_events(gen_copy_stats, hand_copy_stats, 48, &|c, seed| {
            let mut r = machine(c, seed, 0x3e6878, 0);
            // Aligned ld/sd loop on even seeds, unaligned ldl/ldr loop when
            // either endpoint is unaligned.
            let link = DATA + 0x100 + (seed % 2) * 4;
            let src = DATA + 0x2000 + (seed / 2 % 2) * 4;
            for i in 0..0x54u32 {
                c.bus.write32(src + 4 * i, r.next_u32() | 0x0101_0101);
            }
            for i in 0..0x54u32 {
                c.bus.write32(link + 8 + 4 * i, 0xA5A5_0000 + seed + i);
            }
            c.bus.write32(link + 4, 0xDEAD_0000 + seed);
            c.bus.write32(link + 0x158, src);
            let mode = schedule(c, &mut r, seed, 0x003e_6878);
            // Land the first event on the first back edge of whichever loop
            // runs (entry 12 cycles, then 19/11 per iteration).
            if mode != Events::Off {
                c.next_event = c.cycles + 4;
            }
            c.bus.write32(c.g[SP][0] as u32, link);
            mode
        });
    }

    #[test]
    fn net_game_link_status_matches() {
        check(
            ee::s14::f_003e6958__generated,
            net_game_link_status,
            32,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e6958, 0);
                c.pc = 0x003e_6958;
                let link = DATA + 0x100;
                // Fresh (-1: copy path), idle (0: RPC-only path), or busy
                // (immediate return).
                c.bus.write32(
                    link + 4,
                    [0xffff_ffff, 0, 5][seed as usize % 3],
                );
                c.bus.write32(link, 0x1111_0000 + seed);
                let src = DATA + 0x2000 + (seed / 3 % 2) * 4;
                for i in 0..0x54u32 {
                    c.bus.write32(src + 4 * i, r.next_u32() | 0x0202_0202);
                }
                for i in 0..0x54u32 {
                    c.bus.write32(link + 8 + 4 * i, 0xA5A5_0000 + seed + i);
                }
                c.bus.write32(link + 0x158, src);
                c.g[A0][0] = link as u64;
            },
        );
    }

    #[test]
    fn net_game_link_peek_packet_matches() {
        check(
            ee::s14::f_003e6a70__generated,
            net_game_link_peek_packet,
            32,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e6a70, 0);
                c.pc = 0x003e_6a70;
                let link = DATA + 0x100;
                let base = DATA + 0x3000;
                let out_ptr = DATA + 0x200;
                let out = DATA + 0x400;
                c.bus.write32(link + 0x16c, base);
                c.bus.write32(out_ptr, out);
                for i in 0..0x10u32 {
                    c.bus.write32(out + 4 * i, 0xA5A5_0000 + seed + i);
                }
                for i in 0..0x10u32 {
                    c.bus.write32(
                        base + 4 * i,
                        r.next_u32() | 0x0303_0303,
                    );
                }
                match seed % 3 {
                    // No out pointer: store to address 0.
                    0 => {
                        c.g[A1][0] = 0;
                        c.bus.write32(link + 0x160, 0);
                        c.bus.write32(link + 0x164, 0x10);
                    }
                    // Empty: read caught up with write.
                    1 => {
                        c.g[A1][0] = out_ptr as u64;
                        c.bus.write32(link + 0x160, 0);
                        c.bus.write32(link + 0x164, 0);
                    }
                    // A queued packet: length prefix plus body.
                    _ => {
                        c.g[A1][0] = out_ptr as u64;
                        let len = (4 + (seed / 3 % 12)) as u16;
                        c.bus.write16(base, len);
                        c.bus.write32(link + 0x160, 0);
                        c.bus.write32(link + 0x164, 0x20);
                    }
                }
                c.g[A0][0] = link as u64;
            },
        );
    }

    #[test]
    fn net_game_link_recv_matches() {
        check(
            ee::s14::f_003e6b10__generated,
            net_game_link_recv,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e6b10, 0);
                c.pc = 0x003e_6b10;
                let link = DATA + 0x100;
                let base = DATA + 0x3000;
                let out_ptr = DATA + 0x200;
                let out = DATA + 0x400;
                c.bus.write32(link + 0x16c, base);
                c.bus.write32(link, 0x2222_0000 + seed);
                c.bus.write32(out_ptr, out);
                for i in 0..0x20u32 {
                    c.bus.write32(out + 4 * i, 0xA5A5_0000 + seed + i);
                }
                for i in 0..0x40u32 {
                    c.bus.write32(
                        base + 4 * i,
                        r.next_u32() | 0x0404_0404,
                    );
                }
                // Peek classes need a length prefix at the read cursor.
                let len = (4 + (seed / 8 % 8)) as u16;
                c.bus.write16(base, len);
                match seed % 8 {
                    // Drained: cursors meet, reset path.
                    0 => {
                        c.g[A2][0] = 0x10;
                        c.bus.write32(out_ptr, out);
                        c.bus.write32(link + 0x160, 0);
                        c.bus.write32(link + 0x164, 0);
                    }
                    // Peek mode: advance past one packet, bytes remain.
                    1 => {
                        c.g[A2][0] = 1;
                        c.bus.write32(out_ptr, out);
                        c.bus.write32(link + 0x160, 0);
                        c.bus.write32(link + 0x164, 0x20);
                    }
                    // Peek mode: the packet was the last one, reset path.
                    2 => {
                        c.g[A2][0] = 1;
                        c.bus.write32(out_ptr, out);
                        c.bus.write32(link + 0x160, 0);
                        c.bus.write32(link + 0x164, len as u32);
                    }
                    // Copy too small for what's queued: return 0.
                    3 => {
                        c.g[A2][0] = 8;
                        c.bus.write32(out_ptr, out);
                        c.bus.write32(link + 0x160, 0x10);
                        c.bus.write32(link + 0x164, 0x30);
                    }
                    // Copy the queued bytes, then reset path.
                    4 => {
                        c.g[A2][0] = 0x40;
                        c.bus.write32(out_ptr, out);
                        c.bus.write32(link + 0x160, 0x10);
                        c.bus.write32(link + 0x164, 0x30);
                    }
                    // Refill fails: no new bytes, return 0.
                    5 => {
                        c.g[A2][0] = 8;
                        c.bus.write32(out_ptr, out);
                        c.bus.write32(base, 0);
                        c.bus.write32(link + 0x160, 0);
                        c.bus.write32(link + 0x164, 0xffff_ffff);
                    }
                    // Refill reports the known sequence: drain the delta.
                    6 => {
                        c.g[A2][0] = 8;
                        c.bus.write32(out_ptr, out);
                        let masked =
                            (base & 0x0fff_ffff) | 0x2000_0000;
                        let seq = 0x1111_0000 + (seed / 8 % 0x100);
                        c.bus.write32(base, base + 0x8020);
                        c.bus.write32(base + 8, seq);
                        c.bus.write32(base + 0xc, 0x8010);
                        c.bus.write32(base + 0x8020, masked);
                        c.bus.write32(base + 0x8028, seq);
                        c.bus.write32(link + 0x15c, seq);
                        c.bus.write32(link + 0x160, 0);
                        c.bus.write32(link + 0x164, 0xffff_ffff);
                    }
                    // Refill reports a new sequence: resync, return 0.
                    _ => {
                        c.g[A2][0] = 8;
                        c.bus.write32(out_ptr, out);
                        let masked =
                            (base & 0x0fff_ffff) | 0x2000_0000;
                        let seq = 0x1111_0000 + (seed / 8 % 0x100);
                        c.bus.write32(base, base + 0x8020);
                        c.bus.write32(base + 8, seq);
                        c.bus.write32(base + 0xc, 0x8010);
                        c.bus.write32(base + 0x8020, masked);
                        c.bus.write32(base + 0x8028, seq);
                        c.bus.write32(link + 0x15c, seq ^ 0x1111);
                        c.bus.write32(link + 0x160, 0);
                        c.bus.write32(link + 0x164, 0xffff_ffff);
                    }
                }
                c.g[A0][0] = link as u64;
                c.g[A1][0] = out_ptr as u64;
            },
        );
    }

    #[test]
    fn net_game_link_send_matches() {
        check(
            ee::s14::f_003e6c80__generated,
            net_game_link_send,
            32,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e6c80, 0);
                c.pc = 0x003e_6c80;
                let link = DATA + 0x100;
                let base = DATA + 0x3000;
                let buf = DATA + 0x200;
                let read = base + 0x40;
                for i in 0..0x10u32 {
                    c.bus.write32(buf + 4 * i, r.next_u32() | 0x0505_0505);
                }
                // The head snapshot word is the read cursor, so free space
                // is `end - read`.
                c.bus.write32(base, read);
                c.bus.write32(link, 0x3333_0000 + seed);
                c.bus.write32(link + 0x17c, base);
                c.bus.write32(link + 0x178, read);
                let big = seed / 2 % 2 == 0;
                c.bus.write32(link + 0x174, read + if big { 0x300 } else { 8 });
                if seed % 2 == 0 {
                    // Length-prefix fixup path (`len == 1`).
                    c.g[A2][0] = 1;
                    c.bus.write8(buf + 3, (seed / 4 % 64) as u8);
                } else {
                    c.g[A2][0] = if big { 0x10 } else { 0x400 };
                }
                c.g[A0][0] = link as u64;
                c.g[A1][0] = buf as u64;
            },
        );
    }

    #[test]
    fn net_game_link_control_matches() {
        check(
            ee::s14::f_003e6d78__generated,
            net_game_link_control,
            24,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e6d78, 0);
                c.pc = 0x003e_6d78;
                let link = DATA + 0x100;
                for i in 0..8u32 {
                    c.bus.write32(
                        link + 4 * i,
                        r.next_u32() | 0x0606_0606,
                    );
                }
                c.bus.write32(link, 0x4444_0000 + seed);
                // The reply word slot on the frame: pre-filled non-zero so
                // a missing load changes RAM.
                c.bus.write32(
                    crate::difftest::SP - 0x60 + 0x14,
                    0x1234_0000 + seed,
                );
                c.g[A0][0] = link as u64;
                c.g[A1][0] = (0x1000 + seed) as u64;
                c.g[A2][0] = (0x2000 + seed) as u64;
            },
        );
    }

    #[test]
    fn net_game_util_create_matches() {
        check(
            ee::s14::f_003e6e08__generated,
            net_game_util_create,
            24,
            &|c, seed| {
                machine(c, seed, 0x3e6e08, 0);
                c.pc = 0x003e_6e08;
                c.bus.write32(
                    crate::difftest::SP - 0x40 + 0x20,
                    0x2345_0000 + seed,
                );
            },
        );
    }

    #[test]
    fn net_game_util_destroy_matches() {
        check(
            ee::s14::f_003e6e70__generated,
            net_game_util_destroy,
            16,
            &|c, seed| {
                machine(c, seed, 0x3e6e70, 0);
                c.pc = 0x003e_6e70;
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 0x100, 0x7777_0000 + seed);
                    c.g[A0][0] = (DATA + 0x100) as u64;
                } else {
                    c.g[A0][0] = 0;
                }
            },
        );
    }

    #[test]
    fn net_game_util_reset_matches() {
        check(
            ee::s14::f_003e6ed8__generated,
            net_game_util_reset,
            16,
            &|c, seed| {
                machine(c, seed, 0x3e6ed8, 0);
                c.pc = 0x003e_6ed8;
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 0x100, 0x8888_0000 + seed);
                    c.g[A0][0] = (DATA + 0x100) as u64;
                } else {
                    c.g[A0][0] = 0;
                }
            },
        );
    }

    #[test]
    fn net_game_util_connect_matches() {
        check(
            ee::s14::f_003e6f40__generated,
            net_game_util_connect,
            24,
            &|c, seed| {
                machine(c, seed, 0x3e6f40, 0);
                c.pc = 0x003e_6f40;
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 0x100, 0x9999_0000 + seed);
                    c.g[A0][0] = (DATA + 0x100) as u64;
                } else {
                    c.g[A0][0] = 0;
                }
                c.g[A1][0] = (0x3000 + seed) as u64;
                c.g[A2][0] = (0x4000 + seed) as u64;
                c.bus.write32(
                    crate::difftest::SP - 0x70 + 0x20,
                    0x3456_0000 + seed,
                );
            },
        );
    }

    #[test]
    fn net_game_util_complete_matches() {
        check(
            ee::s14::f_003e6fc8__generated,
            net_game_util_complete,
            24,
            &|c, seed| {
                machine(c, seed, 0x3e6fc8, 0);
                c.pc = 0x003e_6fc8;
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 0x100, 0xAAAA_0000 + seed);
                    c.g[A0][0] = (DATA + 0x100) as u64;
                } else {
                    c.g[A0][0] = 0;
                }
                c.bus.write32(
                    crate::difftest::SP - 0x50 + 0x20,
                    0x4567_0000 + seed,
                );
            },
        );
    }

    #[test]
    fn net_game_util_status_matches() {
        check(
            ee::s14::f_003e7038__generated,
            net_game_util_status,
            24,
            &|c, seed| {
                machine(c, seed, 0x3e7038, 0);
                c.pc = 0x003e_7038;
                c.bus.write32(DATA + 0x100, 0xBBBB_0000 + seed);
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (seed % 5) as u64;
                c.bus.write32(
                    crate::difftest::SP - 0x60 + 0x20,
                    0x5678_0000 + seed,
                );
            },
        );
    }

    #[test]
    fn net_game_util_stat_matches() {
        check(
            ee::s14::f_003e70b8__generated,
            net_game_util_stat,
            16,
            &|c, seed| {
                machine(c, seed, 0x3e70b8, 0);
                c.pc = 0x003e_70b8;
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 0x100, 0xCCCC_0000 + seed);
                    c.g[A0][0] = (DATA + 0x100) as u64;
                } else {
                    c.g[A0][0] = 0;
                }
            },
        );
    }

    #[test]
    fn net_game_util_advert_matches() {
        check(
            ee::s14::f_003e7120__generated,
            net_game_util_advert,
            24,
            &|c, seed| {
                machine(c, seed, 0x3e7120, 0);
                elf_image(c);
                c.pc = 0x003e_7120;
                c.bus.write32(DATA + 0x100, 0xDDDD_0000 + seed);
                c.g[A0][0] = (DATA + 0x100) as u64;
                let names: [&[u8]; 4] = [b"srv1", b"a-longer-name", b"", b"x"];
                let rooms: [&[u8]; 4] =
                    [b"room", b"r2", b"the-lobby", b""];
                put_cstr(c, DATA + 0x500, names[seed as usize % 4]);
                put_cstr(c, DATA + 0x600, rooms[seed as usize / 4 % 4]);
                c.g[A1][0] = (DATA + 0x500) as u64;
                c.g[A2][0] = (DATA + 0x600) as u64;
            },
        );
    }

    #[test]
    fn net_game_util_withdraw_matches() {
        check(
            ee::s14::f_003e71b0__generated,
            net_game_util_withdraw,
            16,
            &|c, seed| {
                machine(c, seed, 0x3e71b0, 0);
                c.pc = 0x003e_71b0;
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 0x100, 0xEEEE_0000 + seed);
                    c.g[A0][0] = (DATA + 0x100) as u64;
                } else {
                    c.g[A0][0] = 0;
                }
            },
        );
    }

    #[test]
    fn net_game_util_locate_matches() {
        check(
            ee::s14::f_003e7218__generated,
            net_game_util_locate,
            24,
            &|c, seed| {
                machine(c, seed, 0x3e7218, 0);
                c.pc = 0x003e_7218;
                c.bus.write32(DATA + 0x100, 0xFFFF_0000 + seed);
                c.g[A0][0] = (DATA + 0x100) as u64;
                if seed % 2 == 0 {
                    c.g[A2][0] = 0;
                } else {
                    put_cstr(c, DATA + 0x500, b"arena-7");
                    c.g[A2][0] = (DATA + 0x500) as u64;
                }
                c.bus.write32(
                    crate::difftest::SP - 0x60 + 0x20,
                    0x6789_0000 + seed,
                );
            },
        );
    }

    #[test]
    fn net_game_util_query_matches() {
        check(
            ee::s14::f_003e72a0__generated,
            net_game_util_query,
            32,
            &|c, seed| {
                machine(c, seed, 0x3e72a0, 0);
                c.pc = 0x003e_72a0;
                c.bus.write32(DATA + 0x100, 0x1212_0000 + seed);
                c.g[A0][0] = (DATA + 0x100) as u64;
                if seed % 2 == 0 {
                    c.g[A1][0] = 0;
                } else {
                    put_cstr(c, DATA + 0x500, b"query-x");
                    c.g[A1][0] = (DATA + 0x500) as u64;
                }
                c.g[A2][0] = (0x5000 + seed / 2 % 0x100) as u64;
                c.g[A3][0] = (0x6000 + seed / 4 % 0x100) as u64;
                c.bus.write32(
                    crate::difftest::SP - 0x80 + 0x20,
                    0x789A_0000 + seed,
                );
            },
        );
    }

    #[test]
    fn proto_ping_create_matches() {
        check(
            ee::s14::f_003e7340__generated,
            proto_ping_create,
            24,
            &|c, seed| {
                machine(c, seed, 0x3e7340, 0);
                link_arena(c);
                c.pc = 0x003e_7340;
            },
        );
    }
}
