//! Staging module oc403 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc403`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0026_42b8, c_setup_man_update, "cSetupMan_Update"),
];

/// `cSetupMan` field: host flag (non-zero on the host peer).
pub(crate) const SETUP_MAN_UPDATE_HOST: u32 = 0x124;
/// `cSetupMan` field: ready flag (non-zero when the local peer is ready).
pub(crate) const SETUP_MAN_UPDATE_READY: u32 = 0x128;

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub const SETUP_MAN_RECEIVE_PACKETS: Callee = Callee {
        addr: 0x0026_4a58,
        f: ee::s07::cSetupMan_ReceivePackets,
    };
    pub const SETUP_MAN_FLUSH_SEND_QUEUE: Callee = Callee {
        addr: 0x0026_4b20,
        f: ee::s07::cSetupMan_FlushSendQueue,
    };
    pub const CSETUP_MAN_SEND_PACKET: Callee = Callee {
        addr: 0x0026_4be8,
        f: ee::s07::f_00264be8,
    };
    pub const SETUP_MAN_POP_PACKET: Callee = Callee {
        addr: 0x0026_49b0,
        f: ee::s07::cSetupMan_PopPacket,
    };
    pub const SETUP_MAN_SELECT_CHARACTER: Callee = Callee {
        addr: 0x0026_4f88,
        f: ee::s07::cSetupMan_SelectCharacter,
    };
    pub const SETUP_MAN_HOST_RECV_SELECT: Callee = Callee {
        addr: 0x0026_5190,
        f: ee::s07::cSetupMan_HostRecvSelect,
    };
    pub const SETUP_MAN_HOST_RECV_CONFIRM: Callee = Callee {
        addr: 0x0026_5290,
        f: ee::s07::cSetupMan_HostRecvConfirm,
    };
    pub const SETUP_MAN_HOST_RECV_CANCEL: Callee = Callee {
        addr: 0x0026_5398,
        f: ee::s07::cSetupMan_HostRecvCancel,
    };
    pub const SETUP_MAN_HOST_RECV_MSG7: Callee = Callee {
        addr: 0x0026_9478,
        f: ee::s07::cSetupMan_HostRecvMsg7,
    };
    pub const SETUP_MAN_HOST_RECV_MSG9: Callee = Callee {
        addr: 0x0026_9538,
        f: ee::s07::cSetupMan_HostRecvMsg9,
    };
    pub const SETUP_MAN_HOST_RECV_MSG11: Callee = Callee {
        addr: 0x0026_9608,
        f: ee::s07::cSetupMan_HostRecvMsg11,
    };
    pub const SETUP_MAN_HOST_RECV_MSG13: Callee = Callee {
        addr: 0x0026_96f8,
        f: ee::s07::cSetupMan_HostRecvMsg13,
    };
    pub const SETUP_MAN_CLIENT_RECV_MSG1: Callee = Callee {
        addr: 0x0026_9008,
        f: ee::s07::cSetupMan_ClientRecvMsg1,
    };
    pub const SETUP_MAN_CLIENT_RECV_MSG3: Callee = Callee {
        addr: 0x0026_90c8,
        f: ee::s07::cSetupMan_ClientRecvMsg3,
    };
    pub const SETUP_MAN_CLIENT_RECV_MSG5: Callee = Callee {
        addr: 0x0026_9188,
        f: ee::s07::cSetupMan_ClientRecvMsg5,
    };
    pub const SETUP_MAN_CLIENT_RECV_MSG0: Callee = Callee {
        addr: 0x0026_9240,
        f: ee::s07::cSetupMan_ClientRecvMsg0,
    };
    pub const SETUP_MAN_CLIENT_RECV_MSG2: Callee = Callee {
        addr: 0x0026_9300,
        f: ee::s07::cSetupMan_ClientRecvMsg2,
    };
    pub const SETUP_MAN_CLIENT_RECV_MSG4: Callee = Callee {
        addr: 0x0026_93c0,
        f: ee::s07::cSetupMan_ClientRecvMsg4,
    };
    pub const SETUP_MAN_CLIENT_RECV_MSG6: Callee = Callee {
        addr: 0x0026_97d0,
        f: ee::s07::cSetupMan_ClientRecvMsg6,
    };
    pub const SETUP_MAN_CLIENT_RECV_MSG7: Callee = Callee {
        addr: 0x0026_9890,
        f: ee::s07::cSetupMan_ClientRecvMsg7,
    };
    pub const SETUP_MAN_CLIENT_RECV_MSG8: Callee = Callee {
        addr: 0x0026_9948,
        f: ee::s07::cSetupMan_ClientRecvMsg8,
    };
    pub const SETUP_MAN_CLIENT_RECV_MSG9: Callee = Callee {
        addr: 0x0026_9a18,
        f: ee::s07::cSetupMan_ClientRecvMsg9,
    };
    pub const SETUP_MAN_CLIENT_RECV_MSG10: Callee = Callee {
        addr: 0x0026_9ae0,
        f: ee::s07::cSetupMan_ClientRecvMsg10,
    };
    pub const SETUP_MAN_CLIENT_RECV_MSG11: Callee = Callee {
        addr: 0x0026_9b90,
        f: ee::s07::cSetupMan_ClientRecvMsg11,
    };
    pub const SETUP_MAN_CLIENT_RECV_MSG12: Callee = Callee {
        addr: 0x0026_9c40,
        f: ee::s07::cSetupMan_ClientRecvMsg12,
    };
    pub const SETUP_MAN_CLIENT_RECV_MSG13: Callee = Callee {
        addr: 0x0026_9cf0,
        f: ee::s07::cSetupMan_ClientRecvMsg13,
    };
    pub const BUDDY_MAN_HAS_CHALLENGER: Callee = Callee {
        addr: 0x0026_3128,
        f: ee::s07::BuddyMan_HasChallenger,
    };
    pub const BUDDY_MAN_REMOVE_CHALLENGER: Callee = Callee {
        addr: 0x0026_3018,
        f: ee::s07::BuddyMan_RemoveChallenger,
    };

    /// Cross-shard targets (`call_indirect` in the generated body).
    pub const MEM_MAN_ALLOC: u32 = 0x0031_7d70;
    pub const CBE_GET_INTERFACE: u32 = 0x0014_dd58;
    pub const GET_CHAR_ID: u32 = 0x0014_7488;
    pub const NET_1: u32 = 0x0026_0f80;
    pub const NET_2: u32 = 0x0026_1460;
    pub const NET_3: u32 = 0x0026_13c8;
    pub const NET_FLUSH: u32 = 0x0025_b800;
    pub const NET_RESET: u32 = 0x0025_f740;
}

/// `void cSetupMan_Update(cSetupMan* self)` (0x002642b8): per-frame setup
/// manager update: receive pending packets and flush the send queue, advance
/// the host/ready handshake (states 5/10: host goes to state 12 and sends the
/// 0xF packet, a non-ready client goes to state 7 and posts the 0xF6 lobby
/// message), then pop one queued packet and dispatch it by type: 0xF completes
/// the handshake (state 0xD, clients first reply from state 0xC), 0x10 stores
/// the peer info and selects its character, 0xF8/0xFA drop the challenger
/// (state 0xB), and the remaining types go to the host or client handlers
/// split by the state word (< 7 or not). The generated body for this function
/// is a tree of labelled blocks with no loops and no event checks, so the port
/// keeps that nesting exactly; there are no resume heads.
pub fn c_setup_man_update(c: &mut Cpu) {
    guest(c, |c| {
let frame = Frame::open(c, 0x80);
'b_2643d4: {
'b_2642f4: {
c.cycles += 13;
frame.save(c, S1, 0x60);
frame.save(c, S0, 0x70);
frame.save(c, S2, 0x50);
frame.save_ra(c, 0x40);
c.g[S1][0] = c.g[A0][0];
callee::SETUP_MAN_RECEIVE_PACKETS.call(c, 0x0026_42d4)?;
c.g[A0][0] = c.g[S1][0];
callee::SETUP_MAN_FLUSH_SEND_QUEUE.call(c, 0x0026_42dc)?;
c.g[V1][0] = c.lw((c.g[S1][0] as u32));
c.g[V0][0] = 0x5;
let t = c.g[V1][0] == c.g[V0][0];
c.g[V0][0] = 0xa;
if t {
break 'b_2642f4;
} else {
c.cycles += 2;
if c.g[V1][0] != c.g[V0][0] {
c.g[A0][0] = c.g[S1][0];
break 'b_2643d4;
} else {
break 'b_2642f4;
}
}
}
'b_2643d0: {
'b_264344: {
'b_264310: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x128));
if c.g[V0][0] != 0u64 {
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x124));
break 'b_264310;
} else {
c.cycles += 3;
c.g[V0][0] = 0xa;
let t = c.g[V1][0] != c.g[V0][0];
c.g[V0][0] = 0x7;
if t {
break 'b_264344;
} else {
c.cycles += 1;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x124));
break 'b_264310;
}
}
}
c.cycles += 2;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0xc;
if t {
break 'b_2643d0;
} else {
c.cycles += 11;
c.g[V1][0] = 0xf;
c.sw((c.g[S1][0] as u32), c.g[V0][0]);
c.g[A0][0] = c.g[S1][0];
c.sw((c.g[SP][0] as u32), c.g[V1][0]);
c.g[A1][0] = c.g[SP][0];
c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
c.g[A2][0] = 0x14;
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
callee::CSETUP_MAN_SEND_PACKET.call(c, 0x0026_433c)?;
let t = true;
c.g[A0][0] = c.g[S1][0];
if t {
break 'b_2643d4;
} else {
break 'b_264344;
}
}
}
'b_2643ac: {
c.cycles += 26;
c.g[A1][0] = 0x480000;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
c.g[A0][0] = 0x8;
c.g[A2][0] = 0x20000000;
c.g[A3][0] = 0u64;
c.sw((c.g[S1][0] as u32), c.g[V0][0]);
call_ptr(c, callee::MEM_MAN_ALLOC, 0x0026_4364)?;
c.g[S2][0] = c.g[V0][0];
c.g[V1][0] = 0xf6;
c.g[V0][0] = 0x480000;
c.sw((c.g[S2][0] as u32), c.g[V1][0]);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
c.g[A1][0] = 0x480000;
c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
c.g[A0][0] = 0xc;
c.g[A2][0] = 0x20000000;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xf0));
c.g[A3][0] = 0u64;
c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
call_ptr(c, callee::MEM_MAN_ALLOC, 0x0026_439c)?;
c.g[A1][0] = c.g[V0][0];
c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
if c.g[V0][0] != 0u64 {
c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S2][0]);
break 'b_2643ac;
} else {
break 'b_2643ac;
}
}
c.cycles += 9;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
c.sw((c.g[A1][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
c.sw((c.g[A0][0] as u32), c.g[A1][0]);
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[A1][0]);
break 'b_2643d0;
}
c.cycles += 1;
c.g[A0][0] = c.g[S1][0];
break 'b_2643d4;
}
'b_26494c: {
'b_264948: {
'b_264944: {
c.cycles += 5;
c.g[A1][0] = c.g[SP][0];
c.g[A2][0] = 0x14;
callee::SETUP_MAN_POP_PACKET.call(c, 0x0026_43e0)?;
let t = c.g[V0][0] == 0u64;
c.g[V1][0] = c.lw((c.g[SP][0] as u32));
if t {
break 'b_264944;
} else {
'b_264430: {
c.cycles += 3;
c.g[V0][0] = 0xf;
let t = c.g[V1][0] != c.g[V0][0];
c.g[V0][0] = 0x10;
if t {
break 'b_264430;
} else {
'b_264428: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x124));
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = 0xd;
if t {
break 'b_264428;
} else {
c.cycles += 10;
c.g[V0][0] = 0xc;
c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V1][0]);
c.sw((c.g[S1][0] as u32), c.g[V0][0]);
c.g[A0][0] = c.g[S1][0];
c.sw((c.g[SP][0] as u32).wrapping_add(0x24), 0u64);
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
c.sw((c.g[SP][0] as u32).wrapping_add(0x28), 0u64);
c.g[A2][0] = 0x14;
callee::CSETUP_MAN_SEND_PACKET.call(c, 0x0026_4424)?;
c.g[V0][0] = 0xd;
break 'b_264428;
}
}
c.cycles += 2;
let t = true;
c.sw((c.g[S1][0] as u32), c.g[V0][0]);
if t {
break 'b_264944;
} else {
break 'b_264430;
}
}
}
'b_26449c: {
c.cycles += 2;
if c.g[V1][0] != c.g[V0][0] {
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x124));
break 'b_26449c;
} else {
c.cycles += 25;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A1][0] = 0x1;
c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x27));
c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x2f));
c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x28));
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0xd3));
c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0xcc));
c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xdb));
c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0xd4));
c.sw((c.g[V1][0] as u32).wrapping_add(0xdc), c.g[A2][0]);
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
call_ptr(c, callee::CBE_GET_INTERFACE, 0x0026_447c)?;
c.g[A0][0] = c.g[V0][0];
c.g[A1][0] = 0u64;
call_ptr(c, callee::GET_CHAR_ID, 0x0026_4488)?;
c.g[A0][0] = c.g[S1][0];
c.g[A1][0] = c.g[V0][0];
callee::SETUP_MAN_SELECT_CHARACTER.call(c, 0x0026_4494)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_264948;
} else {
break 'b_26449c;
}
}
}
'b_264750: {
'b_264698: {
'b_264694: {
'b_264684: {
'b_264670: {
'b_26465c: {
'b_264648: {
'b_264630: {
'b_264618: {
'b_264604: {
'b_2645f8: {
'b_2645b0: {
c.cycles += 2;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = c.lw((c.g[S1][0] as u32));
if t {
break 'b_2645b0;
} else {
'b_264514: {
c.cycles += 3;
c.g[V0][0] = ((c.g[V0][0] as i64) < 7i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x5;
if t {
break 'b_264514;
} else {
'b_264504: {
c.cycles += 2;
let t = c.g[V1][0] == c.g[V0][0];
c.g[V0][0] = (c.g[V1][0] < 0x6u64) as u64;
if t {
break 'b_264504;
} else {
c.cycles += 2;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x1;
if t {
break 'b_2645f8;
} else {
'b_2644f0: {
'b_2644d8: {
c.cycles += 2;
let t = c.g[V1][0] == c.g[V0][0];
c.g[V0][0] = 0x3;
if t {
break 'b_2644d8;
} else {
c.cycles += 2;
let t = c.g[V1][0] == c.g[V0][0];
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_2644f0;
} else {
c.cycles += 2;
let t = true;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
if t {
break 'b_26494c;
} else {
break 'b_2644d8;
}
}
}
}
c.cycles += 6;
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.g[S1][0];
c.g[A2][0] = c.g[SP][0] | 0xc;
callee::SETUP_MAN_HOST_RECV_SELECT.call(c, 0x0026_44e8)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_264948;
} else {
break 'b_2644f0;
}
}
c.cycles += 5;
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.g[S1][0];
callee::SETUP_MAN_HOST_RECV_CONFIRM.call(c, 0x0026_44fc)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_264948;
} else {
break 'b_264504;
}
}
}
}
c.cycles += 4;
c.g[A0][0] = c.g[S1][0];
callee::SETUP_MAN_HOST_RECV_CANCEL.call(c, 0x0026_450c)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_264948;
} else {
break 'b_264514;
}
}
}
'b_2645a0: {
'b_264590: {
c.cycles += 3;
c.g[V0][0] = 0xb;
let t = c.g[V1][0] == c.g[V0][0];
c.g[V0][0] = (c.g[V1][0] < 0xcu64) as u64;
if t {
break 'b_264590;
} else {
'b_264578: {
'b_264564: {
'b_264540: {
c.cycles += 2;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x7;
if t {
break 'b_264540;
} else {
c.cycles += 2;
let t = c.g[V1][0] == c.g[V0][0];
c.g[V0][0] = 0x9;
if t {
break 'b_264564;
} else {
c.cycles += 2;
let t = c.g[V1][0] == c.g[V0][0];
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_264578;
} else {
c.cycles += 2;
let t = true;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
if t {
break 'b_26494c;
} else {
break 'b_264540;
}
}
}
}
}
c.cycles += 3;
c.g[V0][0] = 0xf8;
let t = c.g[V1][0] == c.g[V0][0];
c.g[V0][0] = (c.g[V1][0] < 0xf9u64) as u64;
if t {
break 'b_264694;
} else {
c.cycles += 2;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0xd;
if t {
break 'b_264604;
} else {
c.cycles += 2;
let t = c.g[V1][0] == c.g[V0][0];
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_2645a0;
} else {
c.cycles += 2;
let t = true;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
if t {
break 'b_26494c;
} else {
break 'b_264564;
}
}
}
}
}
c.cycles += 5;
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.g[S1][0];
callee::SETUP_MAN_HOST_RECV_MSG7.call(c, 0x0026_4570)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_264948;
} else {
break 'b_264578;
}
}
c.cycles += 6;
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.g[S1][0];
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
callee::SETUP_MAN_HOST_RECV_MSG9.call(c, 0x0026_4588)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_264948;
} else {
break 'b_264590;
}
}
}
c.cycles += 4;
c.g[A0][0] = c.g[S1][0];
callee::SETUP_MAN_HOST_RECV_MSG11.call(c, 0x0026_4598)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_264948;
} else {
break 'b_2645a0;
}
}
c.cycles += 4;
c.g[A0][0] = c.g[S1][0];
callee::SETUP_MAN_HOST_RECV_MSG13.call(c, 0x0026_45a8)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_264948;
} else {
break 'b_2645b0;
}
}
}
c.cycles += 3;
c.g[V0][0] = ((c.g[V0][0] as i64) < 7i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x3;
if t {
break 'b_264750;
} else {
c.cycles += 2;
let t = c.g[V1][0] == c.g[V0][0];
c.g[V0][0] = (c.g[V1][0] < 0x4u64) as u64;
if t {
break 'b_264630;
} else {
'b_2645e4: {
c.cycles += 2;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x1;
if t {
break 'b_2645e4;
} else {
c.cycles += 2;
let t = c.g[V1][0] == c.g[V0][0];
c.g[V0][0] = (c.g[V1][0] < 0x2u64) as u64;
if t {
break 'b_264618;
} else {
c.cycles += 2;
let t = c.g[V0][0] == 0u64;
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
if t {
break 'b_264670;
} else {
c.cycles += 2;
let t = true;
c.g[A0][0] = c.g[S1][0];
if t {
break 'b_26465c;
} else {
break 'b_2645e4;
}
}
}
}
}
c.cycles += 3;
c.g[V0][0] = 0x5;
let t = c.g[V1][0] == c.g[V0][0];
c.g[V0][0] = (c.g[V1][0] < 0x5u64) as u64;
if t {
break 'b_264648;
} else {
c.cycles += 2;
let t = c.g[V0][0] != 0u64;
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
if t {
break 'b_264684;
} else {
break 'b_2645f8;
}
}
}
}
}
c.cycles += 3;
c.g[V0][0] = 0xf8;
let t = c.g[V1][0] == c.g[V0][0];
c.g[V0][0] = 0xb;
if t {
break 'b_264698;
} else {
break 'b_264604;
}
}
c.cycles += 3;
c.g[V0][0] = 0xfa;
let t = c.g[V1][0] == c.g[V0][0];
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_264694;
} else {
c.cycles += 2;
let t = true;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
if t {
break 'b_26494c;
} else {
break 'b_264618;
}
}
}
c.cycles += 6;
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.g[S1][0];
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
callee::SETUP_MAN_CLIENT_RECV_MSG1.call(c, 0x0026_4628)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_264948;
} else {
break 'b_264630;
}
}
c.cycles += 6;
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.g[S1][0];
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
callee::SETUP_MAN_CLIENT_RECV_MSG3.call(c, 0x0026_4640)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_264948;
} else {
break 'b_264648;
}
}
c.cycles += 5;
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
c.g[A0][0] = c.g[S1][0];
callee::SETUP_MAN_CLIENT_RECV_MSG5.call(c, 0x0026_4654)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_264948;
} else {
break 'b_26465c;
}
}
c.cycles += 5;
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
c.g[A3][0] = c.g[SP][0] | 0xc;
callee::SETUP_MAN_CLIENT_RECV_MSG0.call(c, 0x0026_4668)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_264948;
} else {
break 'b_264670;
}
}
c.cycles += 5;
c.g[A0][0] = c.g[S1][0];
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
callee::SETUP_MAN_CLIENT_RECV_MSG2.call(c, 0x0026_467c)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_264948;
} else {
break 'b_264684;
}
}
c.cycles += 4;
c.g[A0][0] = c.g[S1][0];
callee::SETUP_MAN_CLIENT_RECV_MSG4.call(c, 0x0026_468c)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_264948;
} else {
break 'b_264694;
}
}
c.cycles += 1;
c.g[V0][0] = 0xb;
break 'b_264698;
}
'b_2646ac: {
c.cycles += 3;
c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
let t = c.g[V1][0] != 0u64;
c.sw((c.g[S1][0] as u32), c.g[V0][0]);
if t {
break 'b_2646ac;
} else {
c.cycles += 2;
call_ptr(c, callee::NET_1, 0x0026_46ac)?;
break 'b_2646ac;
}
}
'b_2646c8: {
c.cycles += 4;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
call_ptr(c, callee::NET_2, 0x0026_46b4)?;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
if t {
break 'b_2646c8;
} else {
c.cycles += 3;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
call_ptr(c, callee::NET_3, 0x0026_46c4)?;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
break 'b_2646c8;
}
}
'b_2646dc: {
c.cycles += 2;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
if t {
break 'b_2646dc;
} else {
c.cycles += 3;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
call_ptr(c, callee::NET_FLUSH, 0x0026_46d8)?;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
break 'b_2646dc;
}
}
'b_26473c: {
c.cycles += 5;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x98));
callee::BUDDY_MAN_HAS_CHALLENGER.call(c, 0x0026_46e8)?;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
if t {
break 'b_26473c;
} else {
'b_264700: {
c.cycles += 2;
let t = c.g[V0][0] != 0u64;

if t {
break 'b_264700;
} else {
c.cycles += 2;
call_ptr(c, callee::NET_1, 0x0026_4700)?;
break 'b_264700;
}
}
'b_26471c: {
c.cycles += 4;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
call_ptr(c, callee::NET_2, 0x0026_4708)?;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
if t {
break 'b_26471c;
} else {
c.cycles += 3;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
call_ptr(c, callee::NET_3, 0x0026_4718)?;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
break 'b_26471c;
}
}
'b_264730: {
c.cycles += 2;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
if t {
break 'b_264730;
} else {
c.cycles += 3;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
call_ptr(c, callee::NET_FLUSH, 0x0026_472c)?;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
break 'b_264730;
}
}
c.cycles += 3;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x98));
callee::BUDDY_MAN_REMOVE_CHALLENGER.call(c, 0x0026_473c)?;
break 'b_26473c;
}
}
c.cycles += 5;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
c.g[A1][0] = c.lw((c.g[SP][0] as u32));
call_ptr(c, callee::NET_RESET, 0x0026_4748)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_264948;
} else {
break 'b_264750;
}
}
'b_264890: {
'b_26487c: {
'b_264868: {
'b_264854: {
'b_264840: {
c.cycles += 3;
c.g[V0][0] = 0xa;
let t = c.g[V1][0] == c.g[V0][0];
c.g[V0][0] = (c.g[V1][0] < 0xbu64) as u64;
if t {
break 'b_264840;
} else {
'b_264828: {
'b_264810: {
'b_2647fc: {
'b_2647e8: {
'b_2647a0: {
c.cycles += 2;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x7;
if t {
break 'b_2647a0;
} else {
c.cycles += 2;
let t = c.g[V1][0] == c.g[V0][0];
c.g[V0][0] = (c.g[V1][0] < 0x8u64) as u64;
if t {
break 'b_2647fc;
} else {
'b_264784: {
c.cycles += 2;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x6;
if t {
break 'b_264784;
} else {
c.cycles += 2;
let t = c.g[V1][0] == c.g[V0][0];
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_2647e8;
} else {
c.cycles += 2;
let t = true;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
if t {
break 'b_26494c;
} else {
break 'b_264784;
}
}
}
}
c.cycles += 3;
c.g[V0][0] = 0x8;
let t = c.g[V1][0] == c.g[V0][0];
c.g[V0][0] = 0x9;
if t {
break 'b_264810;
} else {
c.cycles += 2;
let t = c.g[V1][0] == c.g[V0][0];
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_264828;
} else {
c.cycles += 2;
let t = true;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
if t {
break 'b_26494c;
} else {
break 'b_2647a0;
}
}
}
}
}
}
c.cycles += 3;
c.g[V0][0] = 0xd;
let t = c.g[V1][0] == c.g[V0][0];
c.g[V0][0] = (c.g[V1][0] < 0xeu64) as u64;
if t {
break 'b_26487c;
} else {
'b_2647cc: {
c.cycles += 2;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0xb;
if t {
break 'b_2647cc;
} else {
c.cycles += 2;
let t = c.g[V1][0] == c.g[V0][0];
c.g[V0][0] = 0xc;
if t {
break 'b_264854;
} else {
c.cycles += 2;
let t = c.g[V1][0] == c.g[V0][0];
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_264868;
} else {
c.cycles += 2;
let t = true;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
if t {
break 'b_26494c;
} else {
break 'b_2647cc;
}
}
}
}
}
c.cycles += 3;
c.g[V0][0] = 0xf8;
let t = c.g[V1][0] == c.g[V0][0];
c.g[V0][0] = 0xfa;
if t {
break 'b_264890;
} else {
c.cycles += 2;
let t = c.g[V1][0] == c.g[V0][0];
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_264890;
} else {
c.cycles += 2;
let t = true;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
if t {
break 'b_26494c;
} else {
break 'b_2647e8;
}
}
}
}
}
c.cycles += 5;
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.g[S1][0];
callee::SETUP_MAN_CLIENT_RECV_MSG6.call(c, 0x0026_47f4)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_264948;
} else {
break 'b_2647fc;
}
}
c.cycles += 5;
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.g[S1][0];
callee::SETUP_MAN_CLIENT_RECV_MSG7.call(c, 0x0026_4808)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_264948;
} else {
break 'b_264810;
}
}
c.cycles += 6;
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.g[S1][0];
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
callee::SETUP_MAN_CLIENT_RECV_MSG8.call(c, 0x0026_4820)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_264948;
} else {
break 'b_264828;
}
}
c.cycles += 6;
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.g[S1][0];
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
callee::SETUP_MAN_CLIENT_RECV_MSG9.call(c, 0x0026_4838)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_264948;
} else {
break 'b_264840;
}
}
}
c.cycles += 5;
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.g[S1][0];
callee::SETUP_MAN_CLIENT_RECV_MSG10.call(c, 0x0026_484c)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_264948;
} else {
break 'b_264854;
}
}
c.cycles += 5;
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.g[S1][0];
callee::SETUP_MAN_CLIENT_RECV_MSG11.call(c, 0x0026_4860)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_264948;
} else {
break 'b_264868;
}
}
c.cycles += 5;
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.g[S1][0];
callee::SETUP_MAN_CLIENT_RECV_MSG12.call(c, 0x0026_4874)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_264948;
} else {
break 'b_26487c;
}
}
c.cycles += 5;
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.g[S1][0];
callee::SETUP_MAN_CLIENT_RECV_MSG13.call(c, 0x0026_4888)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_264948;
} else {
break 'b_264890;
}
}
'b_2648a8: {
c.cycles += 4;
c.g[V0][0] = 0xb;
c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
let t = c.g[V1][0] != 0u64;
c.sw((c.g[S1][0] as u32), c.g[V0][0]);
if t {
break 'b_2648a8;
} else {
c.cycles += 2;
call_ptr(c, callee::NET_1, 0x0026_48a8)?;
break 'b_2648a8;
}
}
'b_2648c4: {
c.cycles += 4;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
call_ptr(c, callee::NET_2, 0x0026_48b0)?;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
if t {
break 'b_2648c4;
} else {
c.cycles += 3;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
call_ptr(c, callee::NET_3, 0x0026_48c0)?;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
break 'b_2648c4;
}
}
'b_2648d8: {
c.cycles += 2;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
if t {
break 'b_2648d8;
} else {
c.cycles += 3;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
call_ptr(c, callee::NET_FLUSH, 0x0026_48d4)?;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
break 'b_2648d8;
}
}
'b_264938: {
c.cycles += 5;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x98));
callee::BUDDY_MAN_HAS_CHALLENGER.call(c, 0x0026_48e4)?;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
if t {
break 'b_264938;
} else {
'b_2648fc: {
c.cycles += 2;
let t = c.g[V0][0] != 0u64;

if t {
break 'b_2648fc;
} else {
c.cycles += 2;
call_ptr(c, callee::NET_1, 0x0026_48fc)?;
break 'b_2648fc;
}
}
'b_264918: {
c.cycles += 4;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
call_ptr(c, callee::NET_2, 0x0026_4904)?;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
if t {
break 'b_264918;
} else {
c.cycles += 3;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
call_ptr(c, callee::NET_3, 0x0026_4914)?;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
break 'b_264918;
}
}
'b_26492c: {
c.cycles += 2;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
if t {
break 'b_26492c;
} else {
c.cycles += 3;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
call_ptr(c, callee::NET_FLUSH, 0x0026_4928)?;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
break 'b_26492c;
}
}
c.cycles += 3;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x98));
callee::BUDDY_MAN_REMOVE_CHALLENGER.call(c, 0x0026_4938)?;
break 'b_264938;
}
}
c.cycles += 3;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
c.g[A1][0] = c.lw((c.g[SP][0] as u32));
call_ptr(c, callee::NET_RESET, 0x0026_4944)?;
break 'b_264944;
}
}
c.cycles += 1;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
break 'b_264948;
}
c.cycles += 1;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
break 'b_26494c;
}
c.cycles += 4;
frame.restore(c, S2, 0x50);
frame.restore_ra(c, 0x40);
frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{fall_back, hook, prev_cell};
    use crate::difftest::{check, fakes::machine, GAME_GP, DATA};

    /// Gate and steering words of the `cSetupMan_Update` fakes.
    const SETUP_MAN_UPDATE_CTRL: u32 = DATA + 0xf_0000;
    const SETUP_MAN_UPDATE_MAGIC: u32 = 0x0c40_32b8;
    const SETUP_MAN_UPDATE_POP_PRESENT: u32 = SETUP_MAN_UPDATE_CTRL + 0x04;
    const SETUP_MAN_UPDATE_MSG: u32 = SETUP_MAN_UPDATE_CTRL + 0x08;
    const SETUP_MAN_UPDATE_HAS: u32 = SETUP_MAN_UPDATE_CTRL + 0x18;
    const SETUP_MAN_UPDATE_NET2: u32 = SETUP_MAN_UPDATE_CTRL + 0x1c;
    const SETUP_MAN_UPDATE_UNWIND_AT: u32 = SETUP_MAN_UPDATE_CTRL + 0x24;
    const SETUP_MAN_UPDATE_FAKE_COUNT: u32 = SETUP_MAN_UPDATE_CTRL + 0x28;
    const SETUP_MAN_UPDATE_ALLOC_COUNT: u32 = SETUP_MAN_UPDATE_CTRL + 0x2c;

    /// Count one fake call; true (with `pc` parked) when this call unwinds.
    fn setup_man_update_fake_step(c: &mut Cpu) -> bool {
        let n = c.bus.read32(SETUP_MAN_UPDATE_FAKE_COUNT);
        c.bus.write32(SETUP_MAN_UPDATE_FAKE_COUNT, n + 1);
        if n == c.bus.read32(SETUP_MAN_UPDATE_UNWIND_AT) {
            c.pc = 0xdead_0000;
            return true;
        }
        false
    }

    /// The next fake allocation block (distinct, writable, non-zero).
    fn setup_man_update_fake_block(c: &mut Cpu) -> u64 {
        let n = c.bus.read32(SETUP_MAN_UPDATE_ALLOC_COUNT);
        c.bus.write32(SETUP_MAN_UPDATE_ALLOC_COUNT, n + 1);
        (DATA + 0x4_0000 + (n & 7) * 0x2000) as u64
    }

    macro_rules! setup_man_update_plain_fake {
        ($name:ident, $gen:path) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(SETUP_MAN_UPDATE_CTRL) != SETUP_MAN_UPDATE_MAGIC {
                    return fall_back(c, prev_cell($name), $gen);
                }
                if setup_man_update_fake_step(c) {
                    return;
                }
                c.cycles += 7;
                ret(c);
            }
        };
    }

    setup_man_update_plain_fake!(setup_man_update_fake_flush, ee::s07::f_00264b20__generated);
    setup_man_update_plain_fake!(setup_man_update_fake_delete, ee::s10::f_00317e50__generated);

    fn setup_man_update_fake_pop(c: &mut Cpu) {
        if c.bus.read32(SETUP_MAN_UPDATE_CTRL) != SETUP_MAN_UPDATE_MAGIC {
            return fall_back(c, prev_cell(setup_man_update_fake_pop), ee::s07::f_002649b0__generated);
        }
        if setup_man_update_fake_step(c) {
            return;
        }
        c.cycles += 7;
        if c.bus.read32(SETUP_MAN_UPDATE_POP_PRESENT) == 0 {
            c.g[V0][0] = 0;
        } else {
            let dst = c.g[A1][0] as u32;
            for i in 0..5u32 {
                let w = c.bus.read32(SETUP_MAN_UPDATE_MSG + 4 * i);
                c.bus.write32(dst + 4 * i, w);
            }
            c.g[V0][0] = 0x14;
        }
        ret(c);
    }

    fn setup_man_update_fake_alloc_packet(c: &mut Cpu) {
        if c.bus.read32(SETUP_MAN_UPDATE_CTRL) != SETUP_MAN_UPDATE_MAGIC {
            return fall_back(c, prev_cell(setup_man_update_fake_alloc_packet), ee::s07::f_00264960__generated);
        }
        if setup_man_update_fake_step(c) {
            return;
        }
        c.cycles += 7;
        c.g[V0][0] = setup_man_update_fake_block(c);
        ret(c);
    }

    fn setup_man_update_fake_mem_alloc(c: &mut Cpu) {
        if c.bus.read32(SETUP_MAN_UPDATE_CTRL) != SETUP_MAN_UPDATE_MAGIC {
            return fall_back(c, prev_cell(setup_man_update_fake_mem_alloc), ee::s10::f_00317d70__generated);
        }
        if setup_man_update_fake_step(c) {
            return;
        }
        c.cycles += 7;
        c.g[V0][0] = setup_man_update_fake_block(c);
        ret(c);
    }

    fn setup_man_update_fake_has_challenger(c: &mut Cpu) {
        if c.bus.read32(SETUP_MAN_UPDATE_CTRL) != SETUP_MAN_UPDATE_MAGIC {
            return fall_back(c, prev_cell(setup_man_update_fake_has_challenger), ee::s07::f_00263128__generated);
        }
        if setup_man_update_fake_step(c) {
            return;
        }
        c.cycles += 7;
        c.g[V0][0] = c.bus.read32(SETUP_MAN_UPDATE_HAS) as u64;
        ret(c);
    }

    fn setup_man_update_fake_net2(c: &mut Cpu) {
        if c.bus.read32(SETUP_MAN_UPDATE_CTRL) != SETUP_MAN_UPDATE_MAGIC {
            return fall_back(c, prev_cell(setup_man_update_fake_net2), ee::s06::f_00261460__generated);
        }
        if setup_man_update_fake_step(c) {
            return;
        }
        c.cycles += 7;
        c.g[V0][0] = c.bus.read32(SETUP_MAN_UPDATE_NET2) as u64;
        ret(c);
    }

    /// `cSetupMan_Update`: handshake phases (no packet / host send / client
    /// post), every packet type on both host and client dispatch tables and
    /// both state halves, the 0xF8 challenger drop with each fake answer,
    /// and fake-callee unwinds. Real callees: the slotless handlers and the
    /// channel vtable leaf (returns 0); faked (above): PopPacket,
    /// FlushSendQueue, AllocPacket, MemAlloc, operator delete,
    /// HasChallenger and NET_2, whose real bodies would run the whole
    /// network stack on fixture data. Never reached by the fixture: the
    /// `NET_1` create and `NET_FLUSH` calls (the comm/lobby globals are
    /// always non-zero here) and packet types the tables do not list.
    #[test]
    fn c_setup_man_update_matches() {
        hook(std::ptr::addr_of_mut!(ee::s07::OVERRIDE_002649B0), setup_man_update_fake_pop, prev_cell(setup_man_update_fake_pop));
        hook(std::ptr::addr_of_mut!(ee::s07::OVERRIDE_00264B20), setup_man_update_fake_flush, prev_cell(setup_man_update_fake_flush));
        hook(std::ptr::addr_of_mut!(ee::s07::OVERRIDE_00264960), setup_man_update_fake_alloc_packet, prev_cell(setup_man_update_fake_alloc_packet));
        hook(std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317D70), setup_man_update_fake_mem_alloc, prev_cell(setup_man_update_fake_mem_alloc));
        hook(std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317E50), setup_man_update_fake_delete, prev_cell(setup_man_update_fake_delete));
        hook(std::ptr::addr_of_mut!(ee::s07::OVERRIDE_00263128), setup_man_update_fake_has_challenger, prev_cell(setup_man_update_fake_has_challenger));
        hook(std::ptr::addr_of_mut!(ee::s06::OVERRIDE_00261460), setup_man_update_fake_net2, prev_cell(setup_man_update_fake_net2));
        check(ee::s07::f_002642b8__generated, c_setup_man_update, 300, &|c, seed| {
            // Real callees on the packet paths allocate: give both sides
            // the pool allocator and a DLHeap arena (the tagsunk fixture).
            let _r = crate::game::dirtysock::tagsunk::tests::alloc_machine(c, seed, 0x2642_b8);
            c.pc = 0x0026_42b8;
            let this = DATA;
            c.g[A0][0] = sx(this);
            let phase = seed % 6;
            let state = match phase {
                0 => 0,
                1 | 2 => 5,
                3 | 4 => 10,
                _ => {
                    if (seed / 11) % 2 == 0 {
                        0
                    } else {
                        8
                    }
                }
            };
            c.bus.write32(this, state);
            c.bus.write32(this + SETUP_MAN_UPDATE_HOST, (seed / 7) % 2);
            c.bus.write32(this + SETUP_MAN_UPDATE_READY, (seed / 13) % 2);
            c.bus.write32(this + 0x90, 0);
            c.bus.write32(this + 0x94, 0);
            c.bus.write32(this + 0x120, 0);
            // Fake steering (seed arithmetic only).
            c.bus.write32(SETUP_MAN_UPDATE_CTRL, SETUP_MAN_UPDATE_MAGIC);
            c.bus.write32(SETUP_MAN_UPDATE_POP_PRESENT, if seed % 5 == 4 { 0 } else { 1 });
            const SETUP_MAN_UPDATE_TYPES: [u32; 19] = [
                0xf, 0x10, 1, 3, 5, 0xf8, 0xfa, 7, 9, 0xb, 0xd, 0, 2, 4, 6, 8, 0xa, 0xc, 0x77,
            ];
            c.bus.write32(SETUP_MAN_UPDATE_MSG, SETUP_MAN_UPDATE_TYPES[(seed / 5) as usize % 19]);
            c.bus.write32(SETUP_MAN_UPDATE_MSG + 4, seed % 9);
            c.bus.write32(SETUP_MAN_UPDATE_MSG + 8, seed % 5);
            c.bus.write32(SETUP_MAN_UPDATE_MSG + 0xc, seed % 7);
            c.bus.write32(SETUP_MAN_UPDATE_MSG + 0x10, seed % 3);
            c.bus.write32(SETUP_MAN_UPDATE_HAS, (seed / 17) % 2);
            c.bus.write32(SETUP_MAN_UPDATE_NET2, (seed / 19) % 2);
            c.bus.write32(
                SETUP_MAN_UPDATE_UNWIND_AT,
                if seed % 7 == 0 { (seed / 7) % 12 } else { u32::MAX },
            );
            c.bus.write32(SETUP_MAN_UPDATE_FAKE_COUNT, 0);
            c.bus.write32(SETUP_MAN_UPDATE_ALLOC_COUNT, 0);
            // Globals: comm system (+/-0x238), lobby (-0xc8), app (-0x848).
            let comm = DATA + 0x1000;
            let lobby = DATA + 0x2000;
            let app = DATA + 0x3000;
            c.bus.write32(GAME_GP + 0x238, comm);
            c.bus.write32(GAME_GP - 0x238, comm);
            c.bus.write32(GAME_GP - 0xc8, lobby);
            c.bus.write32(GAME_GP - 0x848, app);
            c.bus.write32(comm, DATA + 0x1100);
            c.bus.write32(DATA + 0x1100 + 8, DATA + 0x1200);
            c.bus.write16(DATA + 0x1200 + 0x38, 0);
            c.bus.write32(DATA + 0x1200 + 0x3c, 0x0014_06c0);
            c.bus.write16(DATA + 0x1200 + 0x40, 0);
            c.bus.write32(DATA + 0x1200 + 0x44, 0x0014_06c0);
            c.bus.write32(lobby, 0);
            c.bus.write32(lobby + 0x98, DATA + 0x900);
            c.bus.write32(lobby + 0xbc, DATA + 0x200);
            c.bus.write32(lobby + 0xc0, DATA + 0x200);
            c.bus.write32(lobby + 0xf0, DATA + 0x2800);
            c.bus.write32(app + 0x78, DATA + 0x3100);
            // BELibrary dummy (writer-brief): GetInstance reads gp-0x1f04.
            c.bus.write32(GAME_GP - 0x1f04, DATA + 0x3200);
            c.bus.write32(DATA + 0x3200 + 4, DATA + 0x3300);
            c.bus.write16(DATA + 0x3300 + 8, 0);
            c.bus.write32(DATA + 0x3300 + 0xc, 0x0014_06c0);
        });
    }
}
