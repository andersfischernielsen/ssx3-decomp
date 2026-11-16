//! Staging module oc404 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc404`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0026_5568, c_setup_man_send_brag_msg8, "cSetupMan_SendBragMsg8"),
    (0x0026_5458, c_setup_man_send_brag_msg6, "cSetupMan_SendBragMsg6"),
    (0x0026_5290, c_setup_man_host_recv_confirm, "cSetupMan_HostRecvConfirm"),
    (0x0026_5190, c_setup_man_host_recv_select, "cSetupMan_HostRecvSelect"),
    (0x0026_5018, c_setup_man_confirm_character, "cSetupMan_ConfirmCharacter"),
    (0x0026_4f88, c_setup_man_select_character, "cSetupMan_SelectCharacter"),
    (0x0026_4e50, c_setup_man_send_character_msg, "cSetupMan_SendCharacterMsg"),
    (0x0026_4be8, c_setup_man_send_packet, "cSetupMan_SendPacket"),
    (0x0026_4a58, c_setup_man_receive_packets, "cSetupMan_ReceivePackets"),
    (0x0026_4138, c_setup_man_dtor, "cSetupMan_dtor"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub const SEND_PACKET: Callee = Callee { addr: 0x0026_4be8, f: ee::s07::cSetupMan_SendPacket };
    pub const SEND_CHARACTER_MSG: Callee = Callee { addr: 0x0026_4e50, f: ee::s07::cSetupMan_SendCharacterMsg };
    pub const SET_LOCAL_CHARACTER: Callee = Callee { addr: 0x0026_4cf0, f: ee::s07::cSetupMan_SetLocalCharacter };
    pub const SET_REMOTE_CHARACTER: Callee = Callee { addr: 0x0026_4d80, f: ee::s07::cSetupMan_SetRemoteCharacter };
    pub const FLUSH_SEND_QUEUE: Callee = Callee { addr: 0x0026_4b20, f: ee::s07::cSetupMan_FlushSendQueue };
    pub const ALLOC_PACKET: Callee = Callee { addr: 0x0026_4960, f: ee::s07::cSetupMan_AllocPacket };
    pub const FREE_PACKET: Callee = Callee { addr: 0x0026_4990, f: ee::s07::cSetupMan_FreePacket };
}

// Field offsets of the cSetupMan object (one name per offset).
pub(crate) const SETUP_STATE: u32 = 0x0;
pub(crate) const SETUP_CUR_PACKET: u32 = 0x4;
pub(crate) const SETUP_RECV_TAIL: u32 = 0x8;
pub(crate) const SETUP_RECV_HEAD: u32 = 0xc;
pub(crate) const SETUP_RECV_SLOTS: u32 = 0x10;
pub(crate) const SETUP_SEND_TAIL: u32 = 0x90;
pub(crate) const SETUP_SEND_HEAD: u32 = 0x94;
pub(crate) const SETUP_SEND_SLOTS: u32 = 0x98;
pub(crate) const SETUP_CHANNEL: u32 = 0x120;
pub(crate) const SETUP_HOST_FLAG: u32 = 0x124;
pub(crate) const SETUP_STATS_FLAG: u32 = 0x128;
/// Offset of the message-list head inside the globals struct at `gp-0xc8`.
pub(crate) const SETUP_LIST_HEAD: u32 = 0xf0;

const MEM_ALLOC: u32 = 0x0031_7d70;
const MEM_DELETE: u32 = 0x0031_7e50;
const MEMCPY: u32 = 0x003e_6574;
const CHANNEL_SEND: u32 = 0x0025_5e00;
const CHANNEL_RECV: u32 = 0x0025_5dc0;
const BE_GET_BE: u32 = 0x0014_dc80;
const BE_GET_INTERFACE: u32 = 0x0014_dd58;

/// The shared tail of the brag/host/confirm ports: link the freshly
/// allocated message node (`s2` = node, head saved at `sp+head_off`) into
/// the global message list. Transcribed once from the generated insert
/// sequence; callers pass their own frame offset and cycle charge.
fn setup_insert_list_tail(c: &mut Cpu, head_off: u32) {
    c.g[V0][0] = Addr(sp(c).wrapping_add(head_off)).lw(c);
    Addr(c.g[A1][0] as u32).sw(c, c.g[V0][0]);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(4).lw(c);
    Addr(c.g[A1][0] as u32).off(4).sw(c, c.g[V1][0]);
    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(4).lw(c);
    Addr(c.g[A0][0] as u32).sw(c, c.g[A1][0]);
    c.g[V0][0] = Addr(sp(c).wrapping_add(head_off)).lw(c);
    Addr(c.g[V0][0] as u32).off(4).sw(c, c.g[A1][0]);
}

/// `void cSetupMan_SendBragMsg8(cSetupMan* self, int slot, int value)`
/// (0x265568): unless the setup state is 12, post an 8-type brag message
/// (9 when the host flag at +0x124 is clear; a state in [5,7) first
/// advances to 7) through SendPacket, then queue a 0xfc net message
/// carrying slot/value on the global message list.
pub fn c_setup_man_send_brag_msg8(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = 0xc;
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x70);
        frame.save_ra(c, 0x40);
        c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
        let done = c.g[V1][0] == c.g[V0][0];
        c.g[S2][0] = c.g[A2][0];
        if !done {
            // Message type in v0 (8, or 9 when the host flag is clear).
            c.cycles += 3;
            let flag = Addr(c.g[A0][0] as u32).off(SETUP_HOST_FLAG).lw(c);
            c.g[V0][0] = ((c.g[V1][0] as i64) < 5) as u64;
            if flag == 0 {
                c.cycles += 1;
                c.g[V0][0] = 0x9;
            } else {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = 0x8;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 7) as u64;
                    let below7 = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x7;
                    if below7 {
                        c.cycles += 1;
                        Addr(c.g[A0][0] as u32).sw(c, c.g[V0][0]);
                    }
                    c.cycles += 2;
                    c.g[V0][0] = 0x8;
                }
            }
            c.cycles += 34;
            c.g[A1][0] = c.g[SP][0];
            Addr(sp(c)).sw(c, c.g[V0][0]);
            Addr(sp(c).wrapping_add(4)).sw(c, c.g[S1][0]);
            c.g[A2][0] = 0x14;
            Addr(sp(c).wrapping_add(8)).sw(c, c.g[S2][0]);
            callee::SEND_PACKET.call(c, 0x0026_55d8)?;
            c.g[A1][0] = 0x0048_0000;
            c.g[A0][0] = 0x14;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf18));
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, MEM_ALLOC, 0x0026_55f0)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V1][0] = 0x0048_0000;
            c.g[V0][0] = 0xfc;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1130));
            Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
            c.g[A1][0] = 0x0048_0000;
            Addr(c.g[S0][0] as u32).off(4).sw(c, c.g[V1][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
            Addr(c.g[S0][0] as u32).off(0xc).sw(c, c.g[S1][0]);
            c.g[A0][0] = 0xc;
            Addr(c.g[S0][0] as u32).off(0x10).sw(c, c.g[S2][0]);
            c.g[A2][0] = 0x2000_0000;
            Addr(c.g[S0][0] as u32).off(8).sw(c, 0);
            c.g[A3][0] = 0;
            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0xc8)).lw(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(SETUP_LIST_HEAD).lw(c);
            Addr(sp(c).wrapping_add(0x30)).sw(c, c.g[V1][0]);
            call_ptr(c, MEM_ALLOC, 0x0026_5638)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
            if c.g[V0][0] != 0 {
                Addr(c.g[A1][0] as u32).off(8).sw(c, c.g[S0][0]);
            }
            c.cycles += 9;
            setup_insert_list_tail(c, 0x30);
            Addr(sp(c).wrapping_add(0x20)).sw(c, c.g[A1][0]);
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `void cSetupMan_SendBragMsg6(cSetupMan* self, int id)` (0x265458):
/// like SendBragMsg8, but the message type (6, or 7 when the host flag
/// is clear) is carried in v1, the packet payload is (type, id, current
/// state), and the queued net message is 0xfb with only the id payload.
/// Diffed line by line against SendBragMsg8: frame 0x70 (no s2 save),
/// prologue 8 / epilogue 5 cycles, alloc sizes 0x10 then 0xc, net type
/// 0xfb, vtable global 0x481150, no +0x10 payload store.
pub fn c_setup_man_send_brag_msg6(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = 0xc;
        frame.save(c, S1, 0x50);
        frame.save(c, S0, 0x60);
        frame.save_ra(c, 0x40);
        c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
        let done = c.g[V1][0] == c.g[V0][0];
        c.g[S1][0] = c.g[A1][0];
        if !done {
            c.cycles += 3;
            let flag = Addr(c.g[A0][0] as u32).off(SETUP_HOST_FLAG).lw(c);
            c.g[V0][0] = ((c.g[V1][0] as i64) < 5) as u64;
            if flag == 0 {
                c.cycles += 1;
                c.g[V1][0] = 0x7;
            } else {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = 0x6;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 7) as u64;
                    let below7 = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x7;
                    if below7 {
                        c.cycles += 1;
                        Addr(c.g[A0][0] as u32).sw(c, c.g[V0][0]);
                    }
                    c.cycles += 2;
                    c.g[V1][0] = 0x6;
                }
            }
            c.cycles += 34;
            c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
            c.g[A1][0] = c.g[SP][0];
            Addr(sp(c)).sw(c, c.g[V1][0]);
            c.g[A2][0] = 0x14;
            Addr(sp(c).wrapping_add(8)).sw(c, c.g[V0][0]);
            Addr(sp(c).wrapping_add(4)).sw(c, c.g[S1][0]);
            callee::SEND_PACKET.call(c, 0x0026_54c4)?;
            c.g[A1][0] = 0x0048_0000;
            c.g[A0][0] = 0x10;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf18));
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, MEM_ALLOC, 0x0026_54dc)?;
            c.g[V1][0] = 0x0048_0000;
            c.g[S0][0] = c.g[V0][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1150));
            c.g[V0][0] = 0xfb;
            Addr(c.g[S0][0] as u32).off(4).sw(c, c.g[V1][0]);
            c.g[A1][0] = 0x0048_0000;
            Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
            Addr(c.g[S0][0] as u32).off(0xc).sw(c, c.g[S1][0]);
            c.g[A0][0] = 0xc;
            Addr(c.g[S0][0] as u32).off(8).sw(c, 0);
            c.g[A2][0] = 0x2000_0000;
            c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0xc8)).lw(c);
            c.g[A3][0] = 0;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(SETUP_LIST_HEAD).lw(c);
            Addr(sp(c).wrapping_add(0x30)).sw(c, c.g[V0][0]);
            call_ptr(c, MEM_ALLOC, 0x0026_5520)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
            if c.g[V0][0] != 0 {
                Addr(c.g[A1][0] as u32).off(8).sw(c, c.g[S0][0]);
            }
            c.cycles += 9;
            setup_insert_list_tail(c, 0x30);
            Addr(sp(c).wrapping_add(0x20)).sw(c, c.g[A1][0]);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `void cSetupMan_HostRecvSelect(cSetupMan* self, int rider)`
/// (0x265190): advance state 1 -> 2 (state 2 and >= 3 pass through),
/// then set the remote character, send character message 1 and queue a
/// 0xf3 net message. Diffed against HostRecvConfirm: no second
/// SetRemote argument (the caller's a2 passes through), message 1,
/// net type 0xf3, return addresses 0x2651d8/0x2651f0/0x26520c/0x265240.
pub fn c_setup_man_host_recv_select(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save_ra(c, 0x20);
        c.g[A0][0] = 0x2;
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        let is2 = c.g[V1][0] == c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        if !is2 {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 3) as u64;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = 0x1;
                if c.g[V1][0] == c.g[V0][0] {
                    Addr(c.g[S0][0] as u32).sw(c, c.g[A0][0]);
                }
            }
        }
        c.cycles += 33;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S2][0];
        callee::SET_REMOTE_CHARACTER.call(c, 0x0026_51d8)?;
        c.g[S1][0] = 0xf3;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A3][0] = Addr(c.g[A0][0] as u32).lw(c);
        c.g[A1][0] = 0x1;
        callee::SEND_CHARACTER_MSG.call(c, 0x0026_51f0)?;
        c.g[A1][0] = 0x0048_0000;
        c.g[S0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0xc8)).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
        c.g[A0][0] = 0x8;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, MEM_ALLOC, 0x0026_520c)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = 0x0048_0000;
        c.g[V0][0] = 0x0048_0000;
        Addr(c.g[S2][0] as u32).sw(c, c.g[S1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
        Addr(c.g[S2][0] as u32).off(4).sw(c, c.g[V0][0]);
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(SETUP_LIST_HEAD).lw(c);
        Addr(sp(c).wrapping_add(0x10)).sw(c, c.g[V0][0]);
        call_ptr(c, MEM_ALLOC, 0x0026_5240)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
        if c.g[V0][0] != 0 {
            Addr(c.g[A1][0] as u32).off(8).sw(c, c.g[S2][0]);
        }
        c.cycles += 15;
        setup_insert_list_tail(c, 0x10);
        Addr(sp(c)).sw(c, c.g[A1][0]);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void cSetupMan_HostRecvConfirm(cSetupMan* self, int rider)`
/// (0x265290): advance state 1/2 -> 3 and 4 -> 5, then set the remote
/// character (with a zero third argument), send character message 3
/// and queue a 0xf5 net message.
pub fn c_setup_man_host_recv_confirm(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save_ra(c, 0x20);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        let positive = (c.g[V1][0] as i64) > 0;
        c.g[S2][0] = c.g[A1][0];
        let mut skip_common = false;
        if positive {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 3) as u64;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = 0x3;
                c.cycles += 1;
                Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x4;
                c.g[A0][0] = c.g[S0][0];
                if c.g[V1][0] == c.g[V0][0] {
                    c.cycles += 1;
                    c.g[V0][0] = 0x5;
                    c.cycles += 1;
                    Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
                } else {
                    skip_common = true;
                }
            }
        }
        if !skip_common {
            c.cycles += 1;
            c.g[A0][0] = c.g[S0][0];
        }
        c.cycles += 33;
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = 0;
        c.g[S1][0] = 0xf5;
        callee::SET_REMOTE_CHARACTER.call(c, 0x0026_52e8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A3][0] = Addr(c.g[A0][0] as u32).lw(c);
        c.g[A1][0] = 0x3;
        callee::SEND_CHARACTER_MSG.call(c, 0x0026_52fc)?;
        c.g[A1][0] = 0x0048_0000;
        c.g[S0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0xc8)).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
        c.g[A0][0] = 0x8;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, MEM_ALLOC, 0x0026_5318)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = 0x0048_0000;
        c.g[V0][0] = 0x0048_0000;
        Addr(c.g[S2][0] as u32).sw(c, c.g[S1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
        Addr(c.g[S2][0] as u32).off(4).sw(c, c.g[V0][0]);
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(SETUP_LIST_HEAD).lw(c);
        Addr(sp(c).wrapping_add(0x10)).sw(c, c.g[V0][0]);
        call_ptr(c, MEM_ALLOC, 0x0026_534c)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
        if c.g[V0][0] != 0 {
            Addr(c.g[A1][0] as u32).off(8).sw(c, c.g[S2][0]);
        }
        c.cycles += 15;
        setup_insert_list_tail(c, 0x10);
        Addr(sp(c)).sw(c, c.g[A1][0]);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void cSetupMan_ConfirmCharacter(cSetupMan* self, int rider)`
/// (0x265018): with the host flag set, advance state 1/2 -> 4 and
/// 3 -> 5, set the local character, send character message 2 and
/// queue a 0xf4 net message; with the flag clear, only send
/// character message 3.
pub fn c_setup_man_confirm_character(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save_ra(c, 0x20);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(SETUP_HOST_FLAG).lw(c);
        let host = c.g[V0][0] != 0;
        c.g[S2][0] = c.g[A1][0];
        if host {
            c.cycles += 3;
            c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
            let positive = (c.g[V1][0] as i64) > 0;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 3) as u64;
            let mut skip_common = false;
            if positive {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = 0x4;
                    c.cycles += 1;
                    Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0x3;
                    c.g[A0][0] = c.g[S0][0];
                    if c.g[V1][0] == c.g[V0][0] {
                        c.cycles += 1;
                        c.g[V0][0] = 0x5;
                        c.cycles += 1;
                        Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
                    } else {
                        skip_common = true;
                    }
                }
            }
            if !skip_common {
                c.cycles += 1;
                c.g[A0][0] = c.g[S0][0];
            }
            c.cycles += 32;
            c.g[A1][0] = c.g[S2][0];
            callee::SET_LOCAL_CHARACTER.call(c, 0x0026_5070)?;
            c.g[S1][0] = 0xf4;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[A3][0] = Addr(c.g[A0][0] as u32).lw(c);
            c.g[A1][0] = 0x2;
            callee::SEND_CHARACTER_MSG.call(c, 0x0026_5088)?;
            c.g[A1][0] = 0x0048_0000;
            c.g[S0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0xc8)).lw(c);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
            c.g[A0][0] = 0x8;
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, MEM_ALLOC, 0x0026_50a4)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[A1][0] = 0x0048_0000;
            c.g[V0][0] = 0x0048_0000;
            Addr(c.g[S2][0] as u32).sw(c, c.g[S1][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
            Addr(c.g[S2][0] as u32).off(4).sw(c, c.g[V0][0]);
            c.g[A0][0] = 0xc;
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(SETUP_LIST_HEAD).lw(c);
            Addr(sp(c).wrapping_add(0x10)).sw(c, c.g[V0][0]);
            call_ptr(c, MEM_ALLOC, 0x0026_50d8)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
            if c.g[V0][0] != 0 {
                Addr(c.g[A1][0] as u32).off(8).sw(c, c.g[S2][0]);
            }
            c.cycles += 10;
            setup_insert_list_tail(c, 0x10);
            Addr(sp(c)).sw(c, c.g[A1][0]);
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[A3][0] = Addr(c.g[A0][0] as u32).lw(c);
            c.g[A1][0] = 0x3;
            callee::SEND_CHARACTER_MSG.call(c, 0x0026_5124)?;
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

/// `void cSetupMan_SelectCharacter(cSetupMan* self, int rider)`
/// (0x264f88): unless the state is 12, with the host flag set advance
/// state 1 -> 2, set the local character and send character message
/// 0; with the flag clear send character message 1 with a zero state.
pub fn c_setup_man_select_character(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0xc;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        let done = c.g[V1][0] == c.g[V0][0];
        c.g[S1][0] = c.g[A1][0];
        if !done {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(SETUP_HOST_FLAG).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[A1][0] = 0x1;
                c.g[A3][0] = 0;
                callee::SEND_CHARACTER_MSG.call(c, 0x0026_5000)?;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0x1;
                if c.g[V1][0] == c.g[V0][0] {
                    c.cycles += 2;
                    c.g[V0][0] = 0x2;
                    Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
                }
                c.cycles += 9;
                c.g[A1][0] = c.g[S1][0];
                callee::SET_LOCAL_CHARACTER.call(c, 0x0026_4fd0)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[A3][0] = Addr(c.g[A0][0] as u32).lw(c);
                c.g[A1][0] = 0;
                callee::SEND_CHARACTER_MSG.call(c, 0x0026_4fe4)?;
                frame.restore(c, S0, 0x20);
                c.cycles += 4;
                frame.restore(c, S1, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cSetupMan_SendCharacterMsg(cSetupMan* self, int event, int rider,
/// int state)` (0x264e50): build the 0x14-byte character message
/// (event, rider, state, then seven stat bytes: all 40 when the stats
/// flag at +0x128 is set, else the seven BE character attribute
/// getters) and post it through SendPacket.
pub fn c_setup_man_send_character_msg(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S0, 0x50);
        c.g[S2][0] = c.g[A0][0];
        Addr(sp(c)).sw(c, c.g[A1][0]);
        frame.save_ra(c, 0x20);
        Addr(sp(c).wrapping_add(8)).sw(c, c.g[A3][0]);
        Addr(sp(c).wrapping_add(4)).sw(c, c.g[S1][0]);
        call_ptr(c, BE_GET_BE, 0x0026_4e7c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x3;
        call_ptr(c, BE_GET_INTERFACE, 0x0026_4e88)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 0x7;
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x78).lw(c);
        call_ptr(c, BE_GET_INTERFACE, 0x0026_4e9c)?;
        c.g[V1][0] = Addr(c.g[S2][0] as u32).off(SETUP_STATS_FLAG).lw(c);
        if c.g[V1][0] != 0 {
            c.cycles += 2;
            c.g[V1][0] = 0x6;
            c.g[A0][0] = 0x28;
            c.g[V0][0] = sx(sp(c).wrapping_add(0x12));
            loop {
                c.cycles += 7;
                Addr(c.g[V0][0] as u32).sb(c, c.g[A0][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                let more = (c.g[V1][0] as i64) >= 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0026_4eb0);
                    }
                    continue;
                }
                c.cycles += 2;
                c.g[A0][0] = c.g[S2][0];
                break;
            }
        } else {
            c.cycles += 36;
            // Seven attribute getters, one stat byte each at sp+0xc..0x12.
            const GETTERS: [(u32, u32, u32); 7] = [
                (0x0014_8bc8, 0x0026_4ee4, 0xc),
                (0x0014_8c68, 0x0026_4ef8, 0xd),
                (0x0014_8b78, 0x0026_4f0c, 0xe),
                (0x0014_8cb8, 0x0026_4f20, 0xf),
                (0x0014_8d58, 0x0026_4f34, 0x10),
                (0x0014_8d08, 0x0026_4f48, 0x11),
                (0x0014_8c18, 0x0026_4f5c, 0x12),
            ];
            for (addr, ret_addr, off) in GETTERS {
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = c.g[S1][0];
                call_ptr(c, addr, ret_addr)?;
                Addr(sp(c).wrapping_add(off)).sb(c, c.g[V0][0]);
            }
            c.g[A0][0] = c.g[S2][0];
        }
        c.cycles += 9;
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x14;
        callee::SEND_PACKET.call(c, 0x0026_4f70)?;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `int cSetupMan_SendPacket(cSetupMan* self, void* msg, int size)`
/// (0x264be8): flush the send queue, wrap the message in a packet
/// header (kind/flags zero, size+4), try the channel send; on failure
/// copy the packet into a freshly allocated ring slot. Always
/// returns 1 in v0.
pub fn c_setup_man_send_packet(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x130);
        frame.save(c, S2, 0x100);
        frame.save(c, S0, 0x120);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x110);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0xf0);
        c.g[S1][0] = c.g[A1][0];
        callee::FLUSH_SEND_QUEUE.call(c, 0x0026_4c0c)?;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[SP][0] | 0x4;
        Addr(sp(c).wrapping_add(2)).sh(c, c.g[V0][0]);
        Addr(sp(c)).sb(c, 0);
        Addr(sp(c).wrapping_add(1)).sb(c, 0);
        call_ptr(c, MEMCPY, 0x0026_4c2c)?;
        c.g[A3][0] = Addr(sp(c).wrapping_add(2)).lhu(c);
        c.g[A2][0] = c.g[SP][0];
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x238)).lw(c);
        c.g[A1][0] = Addr(c.g[S2][0] as u32).off(SETUP_CHANNEL).lw(c);
        call_ptr(c, CHANNEL_SEND, 0x0026_4c40)?;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = 0x1;
        } else {
            c.cycles += 22;
            c.g[A0][0] = c.g[S2][0];
            callee::ALLOC_PACKET.call(c, 0x0026_4c50)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A2][0] = Addr(sp(c).wrapping_add(2)).lhu(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, MEMCPY, 0x0026_4c64)?;
            c.g[A2][0] = Addr(c.g[S2][0] as u32).off(SETUP_SEND_TAIL).lw(c);
            c.g[A0][0] = 0xffff_ffff_ffff_ffff;
            c.g[V0][0] = 0x1;
            c.g[A1][0] = c.g[A2][0];
            c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1f));
            if c.g[A0][0] != 0 {
                c.g[V1][0] = c.g[A1][0];
            }
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 5) as u32);
            Addr(c.g[S2][0] as u32).off(SETUP_SEND_TAIL).sw(c, c.g[A2][0]);
            c.g[A0][0] = sx((c.g[V1][0] as u32) << 5);
            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
            Addr(c.g[V1][0] as u32).off(SETUP_SEND_SLOTS).sw(c, c.g[S0][0]);
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x120);
        frame.restore(c, S1, 0x110);
        frame.restore(c, S2, 0x100);
        frame.restore_ra(c, 0xf0);
        frame.close(c);
        Ok(())
    })
}

/// `void cSetupMan_ReceivePackets(cSetupMan* self)` (0x264a58): drain
/// the receive channel into the 32-slot ring at +0x10 (indices at
/// +0x8/+0xc, current packet at +0x4) until the channel is empty or
/// the ring holds 31 packets; a packet that arrives when the ring is
/// checked is deleted unused. Dispatcher-shaped: re-enters at the
/// loop head 0x264a80 after an event unwind. The generated state
/// 0x264a7c is never targeted (entry jumps straight to the check).
pub fn c_setup_man_receive_packets(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x0026_4a58 => {
                c.cycles += 9;
                let frame = Frame::open(c, 0x40);
                frame.save(c, S0, 0x30);
                frame.save(c, S1, 0x20);
                c.g[S0][0] = c.g[A0][0];
                frame.save(c, S2, 0x10);
                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(SETUP_RECV_SLOTS));
                frame.save_ra(c, 0);
                c.g[S2][0] = 0xffff_ffff_ffff_ffff;
                pc = 0x0026_4ac8;
            }
            0x0026_4a80 => {
                c.cycles += 6;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(SETUP_RECV_HEAD).lw(c);
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[V0][0] = ((c.g[V0][0] as i64) < 31) as u64;
                c.g[V0][0] ^= 0x1;
                let full = c.g[V0][0] != 0;
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[A1][0] as i64)) as u64;
                pc = if full { 0x0026_4b08 } else { 0x0026_4a98 };
            }
            0x0026_4a98 => {
                c.cycles += 12;
                c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1f));
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = c.g[A1][0];
                }
                c.g[A0][0] = Addr(c.g[S0][0] as u32).off(SETUP_CUR_PACKET).lw(c);
                c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 5) as u32);
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                Addr(c.g[S0][0] as u32).off(SETUP_RECV_TAIL).sw(c, c.g[V0][0]);
                c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                Addr(c.g[V0][0] as u32).sw(c, c.g[A0][0]);
                Addr(c.g[S0][0] as u32).off(SETUP_CUR_PACKET).sw(c, 0);
                pc = 0x0026_4ac8;
            }
            0x0026_4ac8 => {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(SETUP_CUR_PACKET).lw(c);
                if c.g[V0][0] != 0 {
                    c.g[A1][0] = Addr(c.g[S0][0] as u32).off(SETUP_RECV_TAIL).lw(c);
                    if back_edge_unwinds(c, 0x0026_4a80) {
                        return;
                    }
                    pc = 0x0026_4a80;
                } else {
                    pc = 0x0026_4ad4;
                }
            }
            0x0026_4ad4 => {
                c.cycles += 10;
                c.g[A0][0] = c.g[S0][0];
                if callee::ALLOC_PACKET.call(c, 0x0026_4adc).is_err() {
                    return;
                }
                c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x238)).lw(c);
                c.g[A2][0] = c.g[V0][0];
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(SETUP_CHANNEL).lw(c);
                c.g[A3][0] = 0xf0;
                Addr(c.g[S0][0] as u32).off(SETUP_CUR_PACKET).sw(c, c.g[V0][0]);
                if call_ptr(c, CHANNEL_RECV, 0x0026_4af4).is_err() {
                    return;
                }
                if c.g[V0][0] != 0 {
                    c.g[A1][0] = Addr(c.g[S0][0] as u32).off(SETUP_RECV_TAIL).lw(c);
                    if back_edge_unwinds(c, 0x0026_4a80) {
                        return;
                    }
                    pc = 0x0026_4a80;
                } else {
                    pc = 0x0026_4afc;
                }
            }
            0x0026_4afc => {
                c.cycles += 3;
                c.g[A0][0] = Addr(c.g[S0][0] as u32).off(SETUP_CUR_PACKET).lw(c);
                if call_ptr(c, MEM_DELETE, 0x0026_4b04).is_err() {
                    return;
                }
                Addr(c.g[S0][0] as u32).off(SETUP_CUR_PACKET).sw(c, 0);
                pc = 0x0026_4b08;
            }
            0x0026_4b08 => {
                c.cycles += 6;
                let frame = Frame { size: 0x40 };
                frame.restore(c, S0, 0x30);
                frame.restore(c, S1, 0x20);
                frame.restore(c, S2, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return;
            }
            _ => {
                c.jump_out(pc, 0x0026_4a58, 0x0026_4b20);
                return;
            }
        }
    }
}

/// `void cSetupMan_dtor(cSetupMan* self, int flags)` (0x264138):
/// free the current packet and every packet in both 32-slot rings
/// (receive ring at +0x10 with indices +0xc/+0x8, send ring at +0x98
/// with indices +0x94/+0x90), mark the object destroyed (+0 = 1) and,
/// when flags bit 0 is set, delete the object itself.
/// Dispatcher-shaped: re-enters at 0x264178 / 0x264208 after an event
/// unwind. The generated states 0x264174, 0x2641f0-via-loop and the
/// 0x264208 fall-through from 0x2641fc are unreachable (the generator
/// emits them for unconditional branches).
pub fn c_setup_man_dtor(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x0026_4138 => {
                c.cycles += 11;
                let frame = Frame::open(c, 0x50);
                frame.save(c, S0, 0x40);
                frame.save(c, S1, 0x30);
                c.g[S0][0] = c.g[A0][0];
                frame.save(c, S2, 0x20);
                c.g[S1][0] = c.g[A1][0];
                frame.save(c, S3, 0x10);
                frame.save_ra(c, 0);
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(SETUP_CUR_PACKET).lw(c);
                let empty = c.g[A1][0] == 0;
                c.g[S3][0] = c.g[S1][0] & 0x1;
                pc = if empty { 0x0026_41b4 } else { 0x0026_4164 };
            }
            0x0026_4164 => {
                c.cycles += 4;
                if callee::FREE_PACKET.call(c, 0x0026_416c).is_err() {
                    return;
                }
                c.g[A0][0] = Addr(c.g[S0][0] as u32).off(SETUP_RECV_HEAD).lw(c);
                pc = 0x0026_41b8;
            }
            0x0026_4178 => {
                c.cycles += 15;
                c.g[A1][0] = 0xffff_ffff_ffff_ffff;
                c.g[A0][0] = c.g[S0][0];
                c.g[V1][0] = c.g[A2][0];
                c.g[A1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1f));
                if c.g[A1][0] != 0 {
                    c.g[V0][0] = c.g[V1][0];
                }
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
                Addr(c.g[S0][0] as u32).off(SETUP_RECV_HEAD).sw(c, c.g[A2][0]);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A1][0] = Addr(c.g[V1][0] as u32).off(SETUP_RECV_SLOTS).lw(c);
                if callee::FREE_PACKET.call(c, 0x0026_41b4).is_err() {
                    return;
                }
                pc = 0x0026_41b4;
            }
            0x0026_41b4 => {
                c.cycles += 1;
                c.g[A0][0] = Addr(c.g[S0][0] as u32).off(SETUP_RECV_HEAD).lw(c);
                pc = 0x0026_41b8;
            }
            0x0026_41b8 => {
                c.cycles += 4;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(SETUP_RECV_TAIL).lw(c);
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                pc = if more { 0x0026_41c8 } else { 0x0026_41f0 };
            }
            0x0026_41c8 => {
                c.cycles += 10;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1f));
                c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                if c.g[V1][0] != 0 {
                    c.g[V0][0] = c.g[A0][0];
                }
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V0][0] = Addr(c.g[V0][0] as u32).off(SETUP_RECV_SLOTS).lw(c);
                pc = 0x0026_41f4;
            }
            0x0026_41f0 => {
                c.cycles += 1;
                c.g[V0][0] = 0;
                pc = 0x0026_41f4;
            }
            0x0026_41f4 => {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[A2][0] = Addr(c.g[S0][0] as u32).off(SETUP_RECV_HEAD).lw(c);
                    if back_edge_unwinds(c, 0x0026_4178) {
                        return;
                    }
                    pc = 0x0026_4178;
                } else {
                    pc = 0x0026_41fc;
                }
            }
            0x0026_41fc => {
                c.cycles += 3;
                c.g[S2][0] = 0xffff_ffff_ffff_ffff;
                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(SETUP_SEND_SLOTS));
                pc = 0x0026_4240;
            }
            0x0026_4208 => {
                c.cycles += 14;
                c.g[A0][0] = c.g[S0][0];
                c.g[V1][0] = c.g[A2][0];
                c.g[A1][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1f));
                if c.g[A1][0] != 0 {
                    c.g[V0][0] = c.g[V1][0];
                }
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
                Addr(c.g[S0][0] as u32).off(SETUP_SEND_HEAD).sw(c, c.g[A2][0]);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A1][0] = Addr(c.g[V1][0] as u32).lw(c);
                if callee::FREE_PACKET.call(c, 0x0026_4240).is_err() {
                    return;
                }
                pc = 0x0026_4240;
            }
            0x0026_4240 => {
                c.cycles += 5;
                c.g[A0][0] = Addr(c.g[S0][0] as u32).off(SETUP_SEND_HEAD).lw(c);
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(SETUP_SEND_TAIL).lw(c);
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1f));
                pc = if more { 0x0026_4254 } else { 0x0026_4278 };
            }
            0x0026_4254 => {
                c.cycles += 9;
                c.g[V1][0] = ((c.g[S2][0] as i64) < (c.g[A0][0] as i64)) as u64;
                if c.g[V1][0] != 0 {
                    c.g[V0][0] = c.g[A0][0];
                }
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                pc = 0x0026_427c;
            }
            0x0026_4278 => {
                c.cycles += 1;
                c.g[V0][0] = 0;
                pc = 0x0026_427c;
            }
            0x0026_427c => {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[A2][0] = Addr(c.g[S0][0] as u32).off(SETUP_SEND_HEAD).lw(c);
                    if back_edge_unwinds(c, 0x0026_4208) {
                        return;
                    }
                    pc = 0x0026_4208;
                } else {
                    pc = 0x0026_4284;
                }
            }
            0x0026_4284 => {
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                let keep = c.g[S3][0] == 0;
                Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
                pc = if keep { 0x0026_4298 } else { 0x0026_4290 };
            }
            0x0026_4290 => {
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                if call_ptr(c, MEM_DELETE, 0x0026_4298).is_err() {
                    return;
                }
                pc = 0x0026_4298;
            }
            0x0026_4298 => {
                c.cycles += 7;
                let frame = Frame { size: 0x50 };
                frame.restore(c, S0, 0x40);
                frame.restore(c, S1, 0x30);
                frame.restore(c, S2, 0x20);
                frame.restore(c, S3, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return;
            }
            _ => {
                c.jump_out(pc, 0x0026_4138, 0x0026_42b4);
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule, Events};
    use crate::difftest::fakes::{fall_back, hook, prev_cell, FAKE};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    const CTRL: u32 = DATA + 0xe_0000;
    const MAGIC: u32 = 0x0c40_4001;
    const LEAF_ZERO: u32 = 0x0014_06c0;
    const SELF: u32 = DATA + 0x400;

    fn zero(c: &mut Cpu, addr: u32, bytes: u32) {
        for i in (0..bytes).step_by(4) {
            c.bus.write32(addr + i, 0);
        }
    }

    fn fake_alloc(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fall_back(c, prev_cell(fake_alloc), ee::s10::f_00317d70__generated);
        }
        let n = c.bus.read32(CTRL + 4);
        c.bus.write32(CTRL + 4, n + 1);
        c.g[V0][0] = sx(DATA + 0x8_0000 + (n & 7) * 0x40);
        c.cycles += 11;
        ret(c);
    }
    fn fake_delete(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fall_back(c, prev_cell(fake_delete), ee::s10::f_00317e50__generated);
        }
        c.cycles += 6;
        ret(c);
    }
    fn fake_get_be(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fall_back(c, prev_cell(fake_get_be), ee::s01::f_0014dc80__generated);
        }
        c.g[V0][0] = sx(DATA + 0x3_0000);
        c.cycles += 5;
        ret(c);
    }
    fn fake_get_interface(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fall_back(c, prev_cell(fake_get_interface), ee::s01::f_0014dd58__generated);
        }
        c.g[V0][0] = sx(DATA + 0x3_b000);
        c.cycles += 5;
        ret(c);
    }
    fn fake_set_local(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fall_back(c, prev_cell(fake_set_local), ee::s07::f_00264cf0__generated);
        }
        c.g[V0][0] = sx(DATA + 0x3_9000);
        c.cycles += 7;
        ret(c);
    }
    fn fake_set_remote(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fall_back(c, prev_cell(fake_set_remote), ee::s07::f_00264d80__generated);
        }
        c.g[V0][0] = sx(DATA + 0x3_9000);
        c.cycles += 7;
        ret(c);
    }
    fn fake_flush(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fall_back(c, prev_cell(fake_flush), ee::s07::f_00264b20__generated);
        }
        c.cycles += 5;
        ret(c);
    }
    fn fake_alloc_packet(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fall_back(c, prev_cell(fake_alloc_packet), ee::s07::f_00264960__generated);
        }
        let n = c.bus.read32(CTRL + 8);
        c.bus.write32(CTRL + 8, n + 1);
        c.g[V0][0] = sx(DATA + 0xa_0000 + (n & 3) * 0x100);
        c.cycles += 6;
        ret(c);
    }
    fn fake_free_packet(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fall_back(c, prev_cell(fake_free_packet), ee::s07::f_00264990__generated);
        }
        c.cycles += 6;
        ret(c);
    }

    fn install_fakes() {
        hook(addr_of_mut!(ee::s10::OVERRIDE_00317D70), fake_alloc, prev_cell(fake_alloc));
        hook(addr_of_mut!(ee::s10::OVERRIDE_00317E50), fake_delete, prev_cell(fake_delete));
        hook(addr_of_mut!(ee::s01::OVERRIDE_0014DC80), fake_get_be, prev_cell(fake_get_be));
        hook(addr_of_mut!(ee::s01::OVERRIDE_0014DD58), fake_get_interface, prev_cell(fake_get_interface));
        hook(addr_of_mut!(ee::s07::OVERRIDE_00264CF0), fake_set_local, prev_cell(fake_set_local));
        hook(addr_of_mut!(ee::s07::OVERRIDE_00264D80), fake_set_remote, prev_cell(fake_set_remote));
        hook(addr_of_mut!(ee::s07::OVERRIDE_00264B20), fake_flush, prev_cell(fake_flush));
        hook(addr_of_mut!(ee::s07::OVERRIDE_00264960), fake_alloc_packet, prev_cell(fake_alloc_packet));
        hook(addr_of_mut!(ee::s07::OVERRIDE_00264990), fake_free_packet, prev_cell(fake_free_packet));
    }

    /// Scrambled machine, gated fakes, and the globals every port here
    /// touches: BELibrary dummy, world, comm channel 0 vtable, and the
    /// global message-list head chain.
    fn net_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        install_fakes();
        let r = crate::difftest::fakes::machine(c, seed, salt, 0);
        c.bus.write32(CTRL, MAGIC);
        c.bus.write32(CTRL + 4, 0);
        c.bus.write32(CTRL + 8, 0);
        zero(c, DATA + 0x3_0000, 0x100);
        zero(c, DATA + 0x3_1000, 0x100);
        zero(c, DATA + 0x3_2000, 0x300);
        zero(c, DATA + 0x3_3000, 0x400);
        c.bus.write32(GAME_GP.wrapping_sub(0xc8), DATA + 0x3_3000);
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x3_0000);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x3_1000);
        c.bus.write32(GAME_GP.wrapping_sub(0x238), DATA + 0x3_2000);
        c.bus.write32(DATA + 0x3_3000 + SETUP_LIST_HEAD, DATA + 0x3_3200);
        c.bus.write32(DATA + 0x3_3200 + 4, DATA + 0x3_3300);
        // Comm channel 0 -> object -> vtable (send slot +0x44, recv +0x3c).
        c.bus.write32(DATA + 0x3_2000, DATA + 0x3_2100);
        c.bus.write32(DATA + 0x3_2100 + 8, DATA + 0x3_2200);
        c.bus.write16(DATA + 0x3_2200 + 0x38, 0);
        c.bus.write16(DATA + 0x3_2200 + 0x40, 0);
        c.bus.write32(DATA + 0x3_2200 + 0x3c, FAKE);
        c.bus.write32(DATA + 0x3_2200 + 0x44, FAKE);
        zero(c, SELF, 0x200);
        r
    }

    fn self_with_state(c: &mut Cpu, state: u32, flag124: u32) {
        c.bus.write32(SELF + SETUP_STATE, state);
        c.bus.write32(SELF + SETUP_HOST_FLAG, flag124);
        c.g[A0][0] = sx(SELF);
    }

    #[test]
    fn c_setup_man_send_brag_msg8_matches() {
        check(ee::s07::f_00265568__generated, c_setup_man_send_brag_msg8, 210, &|c, seed| {
            let _r = net_machine(c, seed, 0x265568);
            c.pc = 0x0026_5568;
            let state = [12u32, 0, 3, 5, 6, 7, 9][(seed % 7) as usize];
            self_with_state(c, state, seed % 2);
            c.g[A1][0] = sx(seed % 16);
            c.g[A2][0] = sx((seed / 7 % 16) as u32);
        });
    }

    #[test]
    fn c_setup_man_send_brag_msg6_matches() {
        check(ee::s07::f_00265458__generated, c_setup_man_send_brag_msg6, 210, &|c, seed| {
            let _r = net_machine(c, seed, 0x265458);
            c.pc = 0x0026_5458;
            let state = [12u32, 0, 3, 5, 6, 7, 9][(seed % 7) as usize];
            self_with_state(c, state, seed % 2);
            c.g[A1][0] = sx(seed % 16);
        });
    }

    #[test]
    fn c_setup_man_host_recv_select_matches() {
        check(ee::s07::f_00265190__generated, c_setup_man_host_recv_select, 200, &|c, seed| {
            let _r = net_machine(c, seed, 0x265190);
            c.pc = 0x0026_5190;
            let state = [2u32, 1, 0, 3, 0xffff_ffff][(seed % 5) as usize];
            self_with_state(c, state, 0);
            c.bus.write32(SELF + SETUP_STATS_FLAG, 1);
            c.g[A1][0] = sx(seed % 8);
            c.g[A2][0] = sx(DATA + 0x5000);
        });
    }

    #[test]
    fn c_setup_man_host_recv_confirm_matches() {
        check(ee::s07::f_00265290__generated, c_setup_man_host_recv_confirm, 210, &|c, seed| {
            let _r = net_machine(c, seed, 0x265290);
            c.pc = 0x0026_5290;
            let state = [0u32, 0xffff_ffff, 1, 2, 3, 4, 5][(seed % 7) as usize];
            self_with_state(c, state, 0);
            c.bus.write32(SELF + SETUP_STATS_FLAG, 1);
            c.g[A1][0] = sx(seed % 8);
        });
    }

    #[test]
    fn c_setup_man_confirm_character_matches() {
        check(ee::s07::f_00265018__generated, c_setup_man_confirm_character, 240, &|c, seed| {
            let _r = net_machine(c, seed, 0x265018);
            c.pc = 0x0026_5018;
            let state = [0u32, 1, 2, 3, 4, 0xffff_ffff][(seed % 6) as usize];
            self_with_state(c, state, (seed / 6 % 2) as u32);
            c.bus.write32(SELF + SETUP_STATS_FLAG, 1);
            c.g[A1][0] = sx(seed % 8);
        });
    }

    #[test]
    fn c_setup_man_select_character_matches() {
        check(ee::s07::f_00264f88__generated, c_setup_man_select_character, 240, &|c, seed| {
            let _r = net_machine(c, seed, 0x264f88);
            c.pc = 0x0026_4f88;
            let state = [12u32, 1, 0, 5][(seed % 4) as usize];
            self_with_state(c, state, (seed / 4 % 2) as u32);
            c.bus.write32(SELF + SETUP_STATS_FLAG, 1);
            c.g[A1][0] = sx(seed % 8);
        });
    }

    #[test]
    fn c_setup_man_send_character_msg_matches() {
        check(ee::s07::f_00264e50__generated, c_setup_man_send_character_msg, 120, &|c, seed| {
            let _r = net_machine(c, seed, 0x264e50);
            c.pc = 0x0026_4e50;
            self_with_state(c, 0, 0);
            c.bus.write32(SELF + SETUP_STATS_FLAG, seed % 2);
            c.g[A1][0] = sx(seed % 4);
            c.g[A2][0] = sx((seed / 2 % 8) as u32);
            c.g[A3][0] = sx((seed / 4 % 16) as u32);
        });
        check_events(ee::s07::f_00264e50__generated, c_setup_man_send_character_msg, 120, &|c, seed| {
            let mut r = net_machine(c, seed, 0x264e51);
            self_with_state(c, 0, 0);
            c.bus.write32(SELF + SETUP_STATS_FLAG, 1);
            c.g[A1][0] = sx(seed % 4);
            c.g[A2][0] = sx((seed / 2 % 8) as u32);
            c.g[A3][0] = sx((seed / 4 % 16) as u32);
            schedule(c, &mut r, seed, 0x0026_4e50)
        });
    }

    #[test]
    fn c_setup_man_send_packet_matches() {
        check(ee::s07::f_00264be8__generated, c_setup_man_send_packet, 200, &|c, seed| {
            let _r = net_machine(c, seed, 0x264be8);
            c.pc = 0x0026_4be8;
            self_with_state(c, 0, 0);
            c.bus.write32(SELF + SETUP_SEND_TAIL, seed % 32);
            let msg = DATA + 0x2000;
            for i in 0..16u32 {
                c.bus.write32(msg + i * 4, 0x1000 + i);
            }
            if seed % 2 == 0 {
                c.bus.write32(DATA + 0x3_2200 + 0x44, LEAF_ZERO);
            }
            c.g[A1][0] = sx(msg);
            c.g[A2][0] = sx([0u32, 4, 0x14, 0x40][(seed % 4) as usize]);
        });
    }

    #[test]
    fn c_setup_man_receive_packets_matches() {
        check(ee::s07::f_00264a58__generated, c_setup_man_receive_packets, 200, &|c, seed| {
            let _r = net_machine(c, seed, 0x264a58);
            c.pc = 0x0026_4a58;
            self_with_state(c, 0, 0);
            if seed % 3 != 0 {
                c.bus.write32(SELF + SETUP_CUR_PACKET, DATA + 0x6000);
            }
            c.bus.write32(SELF + SETUP_RECV_TAIL, [0u32, 5, 31, 40][(seed % 4) as usize]);
            if seed % 2 == 0 {
                c.bus.write32(DATA + 0x3_2200 + 0x3c, LEAF_ZERO);
            }
        });
        check_events(ee::s07::f_00264a58__generated, c_setup_man_receive_packets, 120, &|c, seed| {
            let mut r = net_machine(c, seed, 0x264a59);
            self_with_state(c, 0, 0);
            c.bus.write32(SELF + SETUP_CUR_PACKET, DATA + 0x6000);
            schedule(c, &mut r, seed, 0x0026_4a58)
        });
    }

    #[test]
    fn c_setup_man_dtor_matches() {
        check(ee::s07::f_00264138__generated, c_setup_man_dtor, 240, &|c, seed| {
            let _r = net_machine(c, seed, 0x264138);
            c.pc = 0x0026_4138;
            self_with_state(c, 0, 0);
            if seed % 2 == 0 {
                c.bus.write32(SELF + SETUP_CUR_PACKET, DATA + 0x6000);
            }
            // Receive ring: head +0xc, tail +0x8, slots at +0x10.
            let (head, tail) = [(0u32, 0u32), (0, 2), (2, 2), (31, 33)][(seed % 4) as usize];
            c.bus.write32(SELF + SETUP_RECV_HEAD, head);
            c.bus.write32(SELF + SETUP_RECV_TAIL, tail);
            for i in 0..4u32 {
                c.bus.write32(SELF + SETUP_RECV_SLOTS + i * 4, DATA + 0x6000 + i * 0x10);
            }
            let (shead, stail) = [(0u32, 0u32), (0, 1), (3, 3)][(seed / 4 % 3) as usize];
            c.bus.write32(SELF + SETUP_SEND_HEAD, shead);
            c.bus.write32(SELF + SETUP_SEND_TAIL, stail);
            for i in 0..4u32 {
                c.bus.write32(SELF + SETUP_SEND_SLOTS + i * 4, DATA + 0x7000 + i * 0x10);
            }
            c.g[A1][0] = sx(seed % 2);
        });
        check_events(ee::s07::f_00264138__generated, c_setup_man_dtor, 120, &|c, seed| {
            let mut r = net_machine(c, seed, 0x264139);
            self_with_state(c, 0, 0);
            c.bus.write32(SELF + SETUP_RECV_HEAD, 0);
            c.bus.write32(SELF + SETUP_RECV_TAIL, 3);
            for i in 0..4u32 {
                c.bus.write32(SELF + SETUP_RECV_SLOTS + i * 4, DATA + 0x6000 + i * 0x10);
            }
            c.g[A1][0] = 0;
            schedule(c, &mut r, seed, 0x0026_4138)
        });
    }
}
