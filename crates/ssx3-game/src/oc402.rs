//! Staging module oc402 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc402`.

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
    (0x0026_87c0, net_event10d_clone, "NetEvent10D_Clone"),
    (0x0026_70c8, lobby_man_list_clear, "LobbyMan_ListClear"),
    (0x0026_6fc0, c_voip_man_debug_printf_stub, "cVoipMan_DebugPrintfStub"),
    (0x0026_6850, c_usb_keyboard_dtor, "cUsbKeyboard_dtor"),
    (0x0026_5768, c_setup_man_send_quit, "cSetupMan_SendQuit"),
    (0x0026_5a68, c_shared_login_man_update, "cSharedLoginMan_Update"),
    (0x0026_6358, c_shared_login_man_mc_poll, "cSharedLoginMan_McPoll"),
    (0x0026_6578, c_shared_login_man_mc_map_error, "cSharedLoginMan_McMapError"),
    (0x0026_7128, buddy_man_message_vector_insert, "BuddyMan_MessageVectorInsert"),
    (0x0026_72c8, buddy_man_challenger_vector_insert, "BuddyMan_ChallengerVectorInsert"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    /// `cMemMan_alloc` / EA_MemAllocNamed (0x317d70), via `call_indirect`.
    pub const MEM_ALLOC_NAMED: u32 = 0x0031_7d70;
    /// `operator delete` (0x317e50), via `call_indirect`.
    pub const OPERATOR_DELETE: u32 = 0x0031_7e50;
    /// `strcpy` (0x4166f4), via `call_indirect`.
    pub const STRCPY: u32 = 0x0041_66f4;
    /// USB-keyboard teardown callee (0x3f62f8), via `call_indirect`.
    pub const USBKB_TEARDOWN: u32 = 0x003f_62f8;
    /// Lobby quit-notify callee (0x25f740), via `call_indirect`.
    pub const LOBBY_SEND_QUIT: u32 = 0x0025_f740;

    /// `cSetupMan_SendPacket` (0x264be8): direct call.
    pub const SETUP_MAN_SEND_PACKET: Callee = Callee {
        addr: 0x0026_4be8,
        f: ee::s07::f_00264be8,
    };
    /// Pointer-vector element copy (0x41610c), via `call_indirect`.
    pub const PTR_VEC_COPY: u32 = 0x0041_610c;
    /// `operator new` named alloc (0x317e30), via `call_indirect`.
    pub const OPERATOR_NEW_NAMED: u32 = 0x0031_7e30;
    /// EA heap free (0x317e98), via `call_indirect`.
    pub const EA_MEM_FREE: u32 = 0x0031_7e98;
    /// Indirect vtable-style call target read from memory (McPoll/Update).
    pub const MC_VCALL_2C51D0: u32 = 0x002c_51d0;
    /// McPoll helper (0x2c67e8), via `call_indirect`.
    pub const MC_POLL_2C67E8: u32 = 0x002c_67e8;
    /// McPoll helper (0x2c68c8), via `call_indirect`.
    pub const MC_POLL_2C68C8: u32 = 0x002c_68c8;
    /// McPoll helper (0x3fa360), via `call_indirect`.
    pub const MC_POLL_3FA360: u32 = 0x003f_a360;
    /// Update state-table call (0x3f80e0), via `call_indirect`.
    pub const UPDATE_3F80E0: u32 = 0x003f_80e0;

    /// `cSharedLoginMan_McMapError` (0x266578): direct call.
    pub const MC_MAP_ERROR: Callee = Callee {
        addr: 0x0026_6578,
        f: ee::s07::f_00266578,
    };
    /// `cSharedLoginMan_DoLoad` (0x265d98): direct call.
    pub const DO_LOAD: Callee = Callee {
        addr: 0x0026_5d98,
        f: ee::s07::f_00265d98,
    };
    /// `cSharedLoginMan_DoUpdate` (0x265e40): direct call.
    pub const DO_UPDATE: Callee = Callee {
        addr: 0x0026_5e40,
        f: ee::s07::f_00265e40,
    };
    /// `cSharedLoginMan_DoCreate` (0x265e80): direct call.
    pub const DO_CREATE: Callee = Callee {
        addr: 0x0026_5e80,
        f: ee::s07::f_00265e80,
    };
    /// `cSharedLoginMan_DoDelete` (0x265ec8): direct call.
    pub const DO_DELETE: Callee = Callee {
        addr: 0x0026_5ec8,
        f: ee::s07::f_00265ec8,
    };
    /// `cSharedLoginMan_DoFinish` (0x265f00): direct call.
    pub const DO_FINISH: Callee = Callee {
        addr: 0x0026_5f00,
        f: ee::s07::f_00265f00,
    };
}

// ---------------------------------------------------------------------------
// NetEvent10D_Clone.

pub(crate) const NET_EVENT_CLONE_SIZE: u32 = 0x164;
pub(crate) const NET_EVENT_CLONE_TAG: u32 = 0x0048_0488;
pub(crate) const NET_EVENT_CLONE_VTABLE: u32 = 0x0048_1110;
pub(crate) const NET_EVENT_CLONE_TYPE: u32 = 0x10d;

/// `void* NetEvent10D_Clone(NetEvent* self)` (0x2687c0): allocate a 0x164-byte
/// copy of the event, stamp type 0x10d and the clone vtable, copy the two
/// strings at +0x8/+0x28/+0x48, the words at +0x148/+0x14c and the blob at
/// +0x150, and return the copy.
pub fn net_event10d_clone(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 52;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = 0x48_0000;
        frame.save(c, S1, 0x30);
        c.g[A2][0] = 0x2000_0000;
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[A3][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[A1][0] = sx(NET_EVENT_CLONE_TAG);
        frame.save_ra(c, 0);
        c.g[A0][0] = NET_EVENT_CLONE_SIZE as u64;
        call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0026_87f4)?;
        c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x28));
        c.g[S0][0] = c.g[V0][0];
        c.g[A2][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x148)).lw(c);
        c.g[A3][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x14c)).lw(c);
        c.g[V1][0] = sx(NET_EVENT_CLONE_VTABLE);
        c.g[V0][0] = NET_EVENT_CLONE_TYPE as u64;
        Addr((c.g[S0][0] as u32).wrapping_add(0x4)).sw(c, c.g[V1][0]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
        Addr((c.g[S0][0] as u32).wrapping_add(0x148)).sw(c, c.g[A2][0]);
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
        Addr((c.g[S0][0] as u32).wrapping_add(0x14c)).sw(c, c.g[A3][0]);
        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x48));
        Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
        call_ptr(c, callee::STRCPY, 0x0026_8830)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x28));
        call_ptr(c, callee::STRCPY, 0x0026_883c)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x48));
        call_ptr(c, callee::STRCPY, 0x0026_8848)?;
        c.ldl(3, (c.g[S1][0] as u32).wrapping_add(0x157));
        c.ldr(3, (c.g[S1][0] as u32).wrapping_add(0x150));
        c.ldl(4, (c.g[S1][0] as u32).wrapping_add(0x15f));
        c.ldr(4, (c.g[S1][0] as u32).wrapping_add(0x158));
        c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x160)).lw(c);
        c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x157));
        c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x150));
        c.sdl(4, (c.g[S0][0] as u32).wrapping_add(0x15f));
        c.sdr(4, (c.g[S0][0] as u32).wrapping_add(0x158));
        Addr((c.g[S0][0] as u32).wrapping_add(0x160)).sw(c, c.g[A1][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// LobbyMan_ListClear.

pub(crate) const LIST_HEAD: u32 = 0x4;

/// `void LobbyMan_ListClear(list* self)` (0x2670c8): delete every node of the
/// circular list headed at self+4, then re-link the head to itself. The loop
/// head is 0x2670f0 (back-edge event check there, as generated).
pub fn lobby_man_list_clear(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(LIST_HEAD)).lw(c);
        c.g[S0][0] = Addr(c.g[V0][0] as u32).lw(c);
        if c.g[S0][0] != c.g[V0][0] {
            c.cycles += 1;
            c.g[A0][0] = c.g[S0][0];
            loop {
                c.cycles += 5;
                c.g[S0][0] = Addr(c.g[S0][0] as u32).lw(c);
                call_ptr(c, callee::OPERATOR_DELETE, 0x0026_70f8)?;
                c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(LIST_HEAD)).lw(c);
                let more = c.g[S0][0] != c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0026_70f0);
                    }
                    continue;
                }
                c.cycles += 2;
                c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(LIST_HEAD)).lw(c);
                break;
            }
        }
        c.cycles += 7;
        Addr(c.g[V0][0] as u32).sw(c, c.g[V0][0]);
        frame.restore_ra(c, 0);
        c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(LIST_HEAD)).lw(c);
        frame.restore(c, S1, 0x10);
        Addr((c.g[V1][0] as u32).wrapping_add(0x4)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cVoipMan_DebugPrintfStub.

/// `void* cVoipMan_DebugPrintfStub(void* self, ...)` (0x266fc0): varargs stub
/// that spills the argument registers/FPRs to its frame and returns `self`
/// unchanged. No event check (no loop).
pub fn c_voip_man_debug_printf_stub(c: &mut Cpu) {
    c.cycles += 15;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
    let base = sp(c);
    c.g[V0][0] = c.g[A0][0];
    c.sd(base.wrapping_add(0x48), c.g[A1][0]);
    c.sd(base.wrapping_add(0x50), c.g[A2][0]);
    c.sd(base.wrapping_add(0x58), c.g[A3][0]);
    c.sd(base.wrapping_add(0x60), c.g[T0][0]);
    c.sd(base.wrapping_add(0x68), c.g[T1][0]);
    c.sd(base.wrapping_add(0x70), c.g[T2][0]);
    c.sd(base.wrapping_add(0x78), c.g[T3][0]);
    c.swc1(12, base.wrapping_add(0x38));
    c.swc1(14, base.wrapping_add(0x3c));
    c.swc1(16, base.wrapping_add(0x40));
    c.swc1(18, base.wrapping_add(0x44));
    c.g[SP][0] = sx(base.wrapping_add(0x80));
    ret(c);
}

// ---------------------------------------------------------------------------
// cUsbKeyboard_dtor.

pub(crate) const USBKB_STATE_WORD: u32 = 0x460;

/// `void cUsbKeyboard_dtor(cUsbKeyboard* self, int flags)` (0x266850): tear
/// down the keyboard state at self+0x460 when set, clear it, and delete the
/// object when bit 0 of `flags` is set. No event check (no loop).
pub fn c_usb_keyboard_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(USBKB_STATE_WORD)).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !empty {
            c.cycles += 3;
            call_ptr(c, callee::USBKB_TEARDOWN, 0x0026_6878)?;
            Addr((c.g[S0][0] as u32).wrapping_add(USBKB_STATE_WORD)).sw(c, 0);
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 0x1;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::OPERATOR_DELETE, 0x0026_6890)?;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cSetupMan_SendQuit.

pub(crate) const ALLOC_TAG_4804A8: u32 = 0x0048_04a8;
pub(crate) const SETUP_STATE_QUIT: u32 = 0xb;
pub(crate) const SETUP_QUIT_ID: u32 = 0xf9;

/// `void cSetupMan_SendQuit(cSetupMan* self, int id)` (0x265768): unless the
/// setup state is already 10, build/send the quit packet (allocating and
/// queueing a 0x10c event when `id == 0xf9`), notify the lobby, and set the
/// state to 11. No event check (no loop).
pub fn c_setup_man_send_quit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = 0xa;
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        frame.save_ra(c, 0x20);
        c.g[V1][0] = Addr(c.g[S3][0] as u32).lw(c);
        let already = c.g[V1][0] == c.g[V0][0];
        c.g[S2][0] = c.g[A1][0];
        if !already {
            c.cycles += 3;
            c.g[V0][0] = SETUP_QUIT_ID as u64;
            let is_quit = c.g[S2][0] == c.g[V0][0];
            c.g[S0][0] = c.g[S2][0];
            if is_quit {
                c.cycles += 26;
                c.g[A1][0] = 0x48_0000;
                c.g[S0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0xc8)).lw(c);
                c.g[A1][0] = sx(0x0048_0488);
                c.g[A0][0] = 0x8;
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0026_57bc)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[V1][0] = 0x10c;
                c.g[V0][0] = 0x48_0000;
                Addr(c.g[S1][0] as u32).sw(c, c.g[V1][0]);
                c.g[V0][0] = sx(0x0048_12b0);
                c.g[A1][0] = 0x48_0000;
                Addr((c.g[S1][0] as u32).wrapping_add(0x4)).sw(c, c.g[V0][0]);
                c.g[A1][0] = sx(ALLOC_TAG_4804A8);
                c.g[A0][0] = 0xc;
                c.g[A2][0] = 0x2000_0000;
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xf0)).lw(c);
                c.g[A3][0] = 0;
                c.g[S0][0] = 0xf8;
                Addr(frame.at(c, 0x10)).sw(c, c.g[V0][0]);
                call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0026_57f8)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                if c.g[V0][0] != 0 {
                    Addr((c.g[A1][0] as u32).wrapping_add(0x8)).sw(c, c.g[S1][0]);
                }
                c.cycles += 9;
                c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
                Addr(c.g[A1][0] as u32).sw(c, c.g[V0][0]);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4)).lw(c);
                Addr((c.g[A1][0] as u32).wrapping_add(0x4)).sw(c, c.g[V1][0]);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4)).lw(c);
                Addr(c.g[A0][0] as u32).sw(c, c.g[A1][0]);
                c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
                Addr((c.g[V0][0] as u32).wrapping_add(0x4)).sw(c, c.g[A1][0]);
                Addr(frame.at(c, 0)).sw(c, c.g[A1][0]);
            }
            c.cycles += 14;
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A1][0] = c.g[SP][0];
            c.g[V0][0] |= 0xffff;
            Addr(sp(c)).sw(c, c.g[S0][0]);
            Addr(sp(c).wrapping_add(0x4)).sw(c, c.g[V0][0]);
            c.g[A0][0] = c.g[S3][0];
            Addr(sp(c).wrapping_add(0x8)).sw(c, 0);
            c.g[A2][0] = 0x14;
            callee::SETUP_MAN_SEND_PACKET.call(c, 0x0026_5850)?;
            c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0xc8)).lw(c);
            c.g[A1][0] = c.g[S2][0];
            call_ptr(c, callee::LOBBY_SEND_QUIT, 0x0026_585c)?;
            c.g[V0][0] = SETUP_STATE_QUIT as u64;
            Addr(c.g[S3][0] as u32).sw(c, c.g[V0][0]);
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

// ---------------------------------------------------------------------------
// BuddyMan vector inserts (shared shape, verified against generated bodies).

pub(crate) const BUDDY_VEC_FIRST: u32 = 0x4;
pub(crate) const BUDDY_VEC_LAST: u32 = 0x8;
pub(crate) const BUDDY_VEC_CAP: u32 = 0xc;

/// `void BuddyMan_MessageVectorInsert(vec* self, void** pos, void* const& x)`
/// (0x267128): insert `*x` at `pos` in the message pointer vector, growing
/// (alloc/copy/free) when full. No event check (no loop).
pub fn buddy_man_message_vector_insert(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x70);
        'outer: {
            'grow: {
                c.cycles += 14;
                frame.save(c, S1, 0x50);
                frame.save(c, S3, 0x30);
                c.g[S1][0] = c.g[A0][0];
                frame.save(c, S4, 0x20);
                c.g[S3][0] = c.g[A1][0];
                frame.save(c, S0, 0x60);
                frame.save(c, S2, 0x40);
                frame.save(c, S5, 0x10);
                frame.save_ra(c, 0);
                c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_LAST)).lw(c);
                c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_CAP)).lw(c);
                let full = c.g[V1][0] == c.g[V0][0];
                c.g[S4][0] = c.g[A2][0];
                if !full {
                    c.cycles += 2;
                    if c.g[V1][0] == 0 {
                        c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_LAST)).lw(c);
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_sub(0x4)).lw(c);
                        Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
                        c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_LAST)).lw(c);
                    }
                    c.cycles += 12;
                    c.g[A1][0] = c.g[S3][0];
                    c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4));
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[S3][0] as u32));
                    Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_LAST)).sw(c, c.g[V0][0]);
                    c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 2) as u32);
                    c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
                    c.g[S0][0] = Addr(c.g[S4][0] as u32).lw(c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[A2][0] as u32));
                    call_ptr(c, callee::PTR_VEC_COPY, 0x0026_719c)?;
                    Addr(c.g[S3][0] as u32).sw(c, c.g[S0][0]);
                    break 'outer;
                }
            }
            'alloc: {
                'need: {
                    c.cycles += 9;
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_FIRST)).lw(c);
                    c.g[S0][0] = 0x1;
                    c.g[A0][0] = 0x1;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                    c.g[V1][0] = sx((c.g[V0][0] as u32) << 1);
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = c.g[V1][0];
                    }
                    let need = c.g[S0][0] != c.g[A0][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
                    if !need {
                        c.cycles += 9;
                        c.g[A1][0] = 0x48_0000;
                        c.g[A0][0] = 0x4;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                        c.g[A2][0] = 0x2000_0000;
                        c.g[A3][0] = 0;
                        c.g[S5][0] = 0x4;
                        call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0026_71e4)?;
                        c.g[S2][0] = c.g[V0][0];
                        break 'alloc;
                    }
                }
                c.cycles += 8;
                c.g[A1][0] = 0x48_0000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                call_ptr(c, callee::OPERATOR_NEW_NAMED, 0x0026_7204)?;
                c.g[S5][0] = c.g[S0][0];
                c.g[S2][0] = c.g[V0][0];
            }
            // copy prefix to new buffer
            c.cycles += 9;
            c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_FIRST)).lw(c);
            c.g[A0][0] = c.g[S2][0];
            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(c.g[A1][0] as u32));
            c.g[A2][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
            call_ptr(c, callee::PTR_VEC_COPY, 0x0026_7228)?;
            let at_pos = c.g[S0][0] == 0;
            c.g[V1][0] = c.g[S0][0];
            if !at_pos {
                c.cycles += 2;
                c.g[V0][0] = Addr(c.g[S4][0] as u32).lw(c);
                Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
            }
            'tail: {
                'old: {
                    c.cycles += 14;
                    c.g[A2][0] = Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_LAST)).lw(c);
                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = c.g[S3][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[S3][0] as u32));
                    c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 2) as u32);
                    c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
                    call_ptr(c, callee::PTR_VEC_COPY, 0x0026_725c)?;
                    c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_FIRST)).lw(c);
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_CAP)).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                    let no_old = c.g[A0][0] == 0;
                    c.g[V1][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                    if !no_old {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        if c.g[V1][0] == c.g[V0][0] {
                            c.cycles += 4;
                            call_ptr(c, callee::OPERATOR_DELETE, 0x0026_7284)?;
                            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S5][0] as u32));
                            break 'tail;
                        }
                        c.cycles += 2;
                        call_ptr(c, callee::EA_MEM_FREE, 0x0026_7294)?;
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S5][0] as u32));
            }
            c.cycles += 3;
            Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_LAST)).sw(c, c.g[S3][0]);
            Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_CAP)).sw(c, c.g[V0][0]);
            Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_FIRST)).sw(c, c.g[S2][0]);
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BuddyMan_ChallengerVectorInsert(vec* self, void** pos, void* const& x)`
/// (0x2672c8): insert `*x` at `pos` in the challenger pointer vector, growing
/// (alloc/copy/free) when full. Same shape as the message-vector insert.
/// No event check (no loop).
pub fn buddy_man_challenger_vector_insert(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x70);
        'outer: {
            'grow: {
                c.cycles += 14;
                frame.save(c, S1, 0x50);
                frame.save(c, S3, 0x30);
                c.g[S1][0] = c.g[A0][0];
                frame.save(c, S4, 0x20);
                c.g[S3][0] = c.g[A1][0];
                frame.save(c, S0, 0x60);
                frame.save(c, S2, 0x40);
                frame.save(c, S5, 0x10);
                frame.save_ra(c, 0);
                c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_LAST)).lw(c);
                c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_CAP)).lw(c);
                let full = c.g[V1][0] == c.g[V0][0];
                c.g[S4][0] = c.g[A2][0];
                if !full {
                    c.cycles += 2;
                    if c.g[V1][0] == 0 {
                        c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_LAST)).lw(c);
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_sub(0x4)).lw(c);
                        Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
                        c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_LAST)).lw(c);
                    }
                    c.cycles += 12;
                    c.g[A1][0] = c.g[S3][0];
                    c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4));
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[S3][0] as u32));
                    Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_LAST)).sw(c, c.g[V0][0]);
                    c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 2) as u32);
                    c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
                    c.g[S0][0] = Addr(c.g[S4][0] as u32).lw(c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[A2][0] as u32));
                    call_ptr(c, callee::PTR_VEC_COPY, 0x0026_733c)?;
                    Addr(c.g[S3][0] as u32).sw(c, c.g[S0][0]);
                    break 'outer;
                }
            }
            'alloc: {
                'need: {
                    c.cycles += 9;
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_FIRST)).lw(c);
                    c.g[S0][0] = 0x1;
                    c.g[A0][0] = 0x1;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                    c.g[V1][0] = sx((c.g[V0][0] as u32) << 1);
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = c.g[V1][0];
                    }
                    let need = c.g[S0][0] != c.g[A0][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
                    if !need {
                        c.cycles += 9;
                        c.g[A1][0] = 0x48_0000;
                        c.g[A0][0] = 0x4;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                        c.g[A2][0] = 0x2000_0000;
                        c.g[A3][0] = 0;
                        c.g[S5][0] = 0x4;
                        call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0026_7384)?;
                        c.g[S2][0] = c.g[V0][0];
                        break 'alloc;
                    }
                }
                c.cycles += 8;
                c.g[A1][0] = 0x48_0000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                call_ptr(c, callee::OPERATOR_NEW_NAMED, 0x0026_73a4)?;
                c.g[S5][0] = c.g[S0][0];
                c.g[S2][0] = c.g[V0][0];
            }
            c.cycles += 9;
            c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_FIRST)).lw(c);
            c.g[A0][0] = c.g[S2][0];
            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(c.g[A1][0] as u32));
            c.g[A2][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
            call_ptr(c, callee::PTR_VEC_COPY, 0x0026_73c8)?;
            let at_pos = c.g[S0][0] == 0;
            c.g[V1][0] = c.g[S0][0];
            if !at_pos {
                c.cycles += 2;
                c.g[V0][0] = Addr(c.g[S4][0] as u32).lw(c);
                Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
            }
            'tail: {
                'old: {
                    c.cycles += 14;
                    c.g[A2][0] = Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_LAST)).lw(c);
                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = c.g[S3][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[S3][0] as u32));
                    c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 2) as u32);
                    c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
                    call_ptr(c, callee::PTR_VEC_COPY, 0x0026_73fc)?;
                    c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_FIRST)).lw(c);
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_CAP)).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                    let no_old = c.g[A0][0] == 0;
                    c.g[V1][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                    if !no_old {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        if c.g[V1][0] == c.g[V0][0] {
                            c.cycles += 4;
                            call_ptr(c, callee::OPERATOR_DELETE, 0x0026_7424)?;
                            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S5][0] as u32));
                            break 'tail;
                        }
                        c.cycles += 2;
                        call_ptr(c, callee::EA_MEM_FREE, 0x0026_7434)?;
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S5][0] as u32));
            }
            c.cycles += 3;
            Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_LAST)).sw(c, c.g[S3][0]);
            Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_CAP)).sw(c, c.g[V0][0]);
            Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_FIRST)).sw(c, c.g[S2][0]);
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cSharedLoginMan_McMapError: match-pc dispatcher (value jump tables at
// 0x480fe0/0x481020/0x481040/0x481060, event heads listed in the doc).

/// `int cSharedLoginMan_McMapError(int err)` (0x266578): map a memory-card
/// error code to a result code via jump tables indexed by the mode word at
/// gp+0x2e4 and by `err`. Dispatcher: resume heads (event unwind targets in
/// the generated body) are 0x2665b4, 0x2665b8, 0x2665c8, 0x2665dc, 0x26660c,
/// 0x266658 and 0x266660; every arm is re-enterable.
pub fn c_shared_login_man_mc_map_error(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x0026_6578 => {
                c.cycles += 5;
                c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2e4)).lw(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = (c.g[V1][0] < 0xe) as u64;
                let out = c.g[V0][0] == 0;
                c.g[A1][0] = c.g[A0][0];
                pc = if out { 0x0026_6718 } else { 0x0026_658c };
            }
            0x0026_658c => {
                c.cycles += 7;
                c.g[V0][0] = 0x48_0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xfe0));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                pc = c.g[A0][0] as u32;
            }
            0x0026_65a8 => {
                c.cycles += 3;
                c.g[V0][0] = 0xffff_ffff_ffff_fffc;
                pc = if c.g[A1][0] == c.g[V0][0] { 0x0026_65c8 } else { 0x0026_65b4 };
            }
            0x0026_65b4 => {
                c.cycles += 1;
                c.g[V0][0] = 0xffff_ffff_ffff_fffe;
                pc = 0x0026_65b8;
            }
            0x0026_65b8 => {
                c.cycles += 2;
                let ne = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0x3;
                pc = if ne { 0x0026_671c } else { 0x0026_65c0 };
            }
            0x0026_65c0 => { c.cycles += 2; c.g[V0][0] = 0x9; ret(c); return; }
            0x0026_65c8 => { c.cycles += 2; c.g[V0][0] = 0x4; ret(c); return; }
            0x0026_65d0 => {
                c.cycles += 3;
                c.g[V0][0] = 0xffff_ffff_ffff_fffe;
                let ne = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0x3;
                pc = if ne { 0x0026_671c } else { 0x0026_65dc };
            }
            0x0026_65dc => { c.cycles += 2; c.g[V0][0] = 0x11; ret(c); return; }
            0x0026_65e4 => {
                c.cycles += 4;
                c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7));
                c.g[V0][0] = (c.g[V1][0] < 0x6) as u64;
                let out = c.g[V0][0] == 0;
                c.g[V0][0] = 0x48_0000;
                pc = if out { 0x0026_6718 } else { 0x0026_65f4 };
            }
            0x0026_65f4 => {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1020));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                pc = c.g[A0][0] as u32;
            }
            0x0026_660c => { c.cycles += 2; c.g[V0][0] = 0xd; ret(c); return; }
            0x0026_6614 => {
                c.cycles += 3;
                c.g[V0][0] = 0xffff_ffff_ffff_fffc;
                let eq = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = 0xffff_ffff_ffff_fffe;
                if eq {
                    if back_edge_unwinds(c, 0x0026_65c8) { return; }
                    pc = 0x0026_65c8;
                } else { pc = 0x0026_6620; }
            }
            0x0026_6620 => {
                c.cycles += 2;
                if c.g[A1][0] != c.g[V0][0] {
                    if back_edge_unwinds(c, 0x0026_65dc) { return; }
                    pc = 0x0026_65dc;
                } else { pc = 0x0026_6628; }
            }
            0x0026_6628 => { c.cycles += 2; c.g[V0][0] = 0x9; ret(c); return; }
            0x0026_6630 => {
                c.cycles += 4;
                c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = (c.g[V1][0] < 0x7) as u64;
                let out = c.g[V0][0] == 0;
                c.g[V0][0] = 0x48_0000;
                pc = if out { 0x0026_6718 } else { 0x0026_6640 };
            }
            0x0026_6640 => {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1040));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                pc = c.g[A0][0] as u32;
            }
            0x0026_6658 => { c.cycles += 2; c.g[V0][0] = 0xe; ret(c); return; }
            0x0026_6660 => {
                c.cycles += 3;
                c.g[V0][0] = 0xffff_ffff_ffff_fffc;
                let ne = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0xffff_ffff_ffff_fffe;
                if ne {
                    if back_edge_unwinds(c, 0x0026_65b8) { return; }
                    pc = 0x0026_65b8;
                } else { pc = 0x0026_666c; }
            }
            0x0026_666c => {
                c.cycles += 2;
                if back_edge_unwinds(c, 0x0026_65c8) { return; }
                pc = 0x0026_65c8;
            }
            0x0026_6674 => {
                c.cycles += 3;
                c.g[V0][0] = 0xffff_ffff_ffff_fffc;
                let eq = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = ((c.g[A1][0] as i64) < -3) as u64;
                if eq {
                    if back_edge_unwinds(c, 0x0026_65c8) { return; }
                    pc = 0x0026_65c8;
                } else { pc = 0x0026_6680; }
            }
            0x0026_6680 => {
                c.cycles += 2;
                let z = c.g[V0][0] == 0;
                c.g[V0][0] = 0xffff_ffff_ffff_fffb;
                if z {
                    if back_edge_unwinds(c, 0x0026_65b4) { return; }
                    pc = 0x0026_65b4;
                } else { pc = 0x0026_6688; }
            }
            0x0026_6688 => {
                c.cycles += 2;
                let ne = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0x3;
                pc = if ne { 0x0026_671c } else { 0x0026_6690 };
            }
            0x0026_6690 => {
                c.cycles += 2;
                if back_edge_unwinds(c, 0x0026_660c) { return; }
                pc = 0x0026_660c;
            }
            0x0026_6698 => {
                c.cycles += 4;
                c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = (c.g[V1][0] < 0x7) as u64;
                let out = c.g[V0][0] == 0;
                c.g[V0][0] = 0x48_0000;
                pc = if out { 0x0026_6718 } else { 0x0026_66a8 };
            }
            0x0026_66a8 => {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1060));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                pc = c.g[A0][0] as u32;
            }
            0x0026_66c0 => {
                c.cycles += 3;
                c.g[V0][0] = 0xffff_ffff_ffff_fffd;
                let eq = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = ((c.g[A1][0] as i64) < -2) as u64;
                if eq {
                    if back_edge_unwinds(c, 0x0026_6658) { return; }
                    pc = 0x0026_6658;
                } else { pc = 0x0026_66cc; }
            }
            0x0026_66cc => {
                c.cycles += 2;
                let z = c.g[V0][0] == 0;
                c.g[V0][0] = 0xffff_ffff_ffff_fffc;
                if z {
                    if back_edge_unwinds(c, 0x0026_65b4) { return; }
                    pc = 0x0026_65b4;
                } else { pc = 0x0026_66d4; }
            }
            0x0026_66d4 => {
                c.cycles += 2;
                let ne = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0x3;
                pc = if ne { 0x0026_671c } else { 0x0026_66dc };
            }
            0x0026_66dc => { c.cycles += 2; c.g[V0][0] = 0x8; ret(c); return; }
            0x0026_66e4 => {
                c.cycles += 3;
                c.g[V0][0] = 0xffff_ffff_ffff_fffb;
                let eq = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = ((c.g[A1][0] as i64) < -4) as u64;
                pc = if eq { 0x0026_6708 } else { 0x0026_66f0 };
            }
            0x0026_66f0 => {
                c.cycles += 2;
                let z = c.g[V0][0] == 0;
                c.g[V0][0] = 0xffff_ffff_ffff_fffa;
                if z {
                    if back_edge_unwinds(c, 0x0026_6660) { return; }
                    pc = 0x0026_6660;
                } else { pc = 0x0026_66f8; }
            }
            0x0026_66f8 => {
                c.cycles += 2;
                let eq = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = 0x3;
                pc = if eq { 0x0026_6710 } else { 0x0026_6700 };
            }
            0x0026_6700 => { c.cycles += 2; pc = 0x0026_671c; }
            0x0026_6708 => { c.cycles += 2; c.g[V0][0] = 0x5; ret(c); return; }
            0x0026_6710 => { c.cycles += 2; c.g[V0][0] = 0x6; ret(c); return; }
            0x0026_6718 => { c.cycles += 1; c.g[V0][0] = 0x3; pc = 0x0026_671c; }
            0x0026_671c => { c.cycles += 2; ret(c); return; }
            _ => { c.jump_out(pc, 0x0026_6578, 0x0026_6724); return; }
        }
    }
}

// ---------------------------------------------------------------------------
// cSharedLoginMan_McPoll.

/// `void cSharedLoginMan_McPoll(void* self, int* out)` (0x266358): poll the
/// memory-card state (mode word at gp+0x2e4, manager at gp+0x2e8), dispatch
/// through the table at 0x480fa0 for modes < 15, and write the poll result
/// to `out`. Loop heads 0x266358 and 0x2664b0 check events with the result
/// ignored (structured loop, as generated).
pub fn c_shared_login_man_mc_poll(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x40);
        'l_266358: loop {
            'b_266538: {
                'b_266534: {
                    'b_266530: {
                        'b_26652c: {
                            'b_266510: {
                                'b_266504: {
                                    'b_2663f8: {
                                        'b_2663ec: {
                                            'b_2663d4: {
                                                'b_2663d0: {
                                                    'b_2663cc: {
                                                        'b_2663bc: {
                                                            c.cycles += 11;
                                                            frame.save(c, S2, 0x10);
                                                            frame.save(c, S0, 0x30);
                                                            c.g[S2][0] = c.g[A1][0];
                                                            frame.save(c, S1, 0x20);
                                                            frame.save_ra(c, 0);
                                                            Addr(c.g[S2][0] as u32).sw(c, 0);
                                                            c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2e4)).lw(c);
                                                            let is10 = c.g[V1][0] == 0xa;
                                                            c.g[S0][0] = c.g[A0][0];
                                                            if is10 {
                                                                break 'b_2663bc;
                                                            }
                                                            'b_2663b4: {
                                                                'b_2663a0: {
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = ((c.g[V1][0] as i64) < 11) as u64;
                                                                    let ge = c.g[V0][0] == 0;
                                                                    c.g[V0][0] = 0x3;
                                                                    if ge {
                                                                        break 'b_2663a0;
                                                                    }
                                                                    c.cycles += 2;
                                                                    let is3 = c.g[V1][0] == c.g[V0][0];
                                                                    c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2e8)).lw(c);
                                                                    if is3 {
                                                                        break 'b_2663b4;
                                                                    }
                                                                    c.cycles += 2;
                                                                    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                                                                    break 'b_2663d4;
                                                                }
                                                                c.cycles += 3;
                                                                c.g[V0][0] = ((c.g[V1][0] as i64) < 17) as u64;
                                                                let ge17 = c.g[V0][0] == 0;
                                                                c.g[V0][0] = ((c.g[V1][0] as i64) < 15) as u64;
                                                                if ge17 {
                                                                    break 'b_2663cc;
                                                                }
                                                                c.cycles += 2;
                                                                let lt15 = c.g[V0][0] != 0;
                                                                c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2e8)).lw(c);
                                                                if lt15 {
                                                                    break 'b_2663d0;
                                                                }
                                                                break 'b_2663b4;
                                                            }
                                                            c.cycles += 2;
                                                            c.g[V1][0] = 0x2;
                                                            break 'b_2663ec;
                                                        }
                                                        c.cycles += 4;
                                                        c.g[V0][0] = 0x2;
                                                        c.g[V1][0] = 0x800;
                                                        Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
                                                        break 'b_266504;
                                                    }
                                                    c.cycles += 1;
                                                    c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2e8)).lw(c);
                                                    break 'b_2663d0;
                                                }
                                                c.cycles += 1;
                                                c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                                                break 'b_2663d4;
                                            }
                                            c.cycles += 6;
                                            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x58)).lh(c);
                                            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c)).lw(c);
                                            let tgt = c.g[V0][0] as u32;
                                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, tgt, 0x0026_63e4)?;
                                            let zero = c.g[V0][0] == 0;
                                            c.g[V1][0] = 0x1;
                                            if zero {
                                                break 'b_2663f8;
                                            }
                                            break 'b_2663ec;
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = 0;
                                        Addr(c.g[S0][0] as u32).sw(c, c.g[V1][0]);
                                        break 'b_266534;
                                    }
                                    'b_26642c: {
                                        c.cycles += 7;
                                        c.g[V0][0] = 0x2;
                                        Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
                                        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2e8)).lw(c);
                                        c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xc)).lw(c);
                                        call_ptr(c, callee::MC_VCALL_2C51D0, 0x0026_640c)?;
                                        let zero = c.g[V0][0] == 0;
                                        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2e8)).lw(c);
                                        if zero {
                                            break 'b_26642c;
                                        }
                                        c.cycles += 6;
                                        c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xc)).lw(c);
                                        call_ptr(c, callee::MC_VCALL_2C51D0, 0x0026_641c)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        callee::MC_MAP_ERROR.call(c, 0x0026_6424)?;
                                        frame.restore(c, S0, 0x30);
                                        break 'b_266538;
                                    }
                                    c.cycles += 4;
                                    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2e4)).lw(c);
                                    c.g[V0][0] = (c.g[V1][0] < 0xf) as u64;
                                    let ge15 = c.g[V0][0] == 0;
                                    c.g[V0][0] = 0x48_0000;
                                    if ge15 {
                                        break 'b_266530;
                                    }
                                    'b_266500: {
                                        'b_2664f8: {
                                            'b_2664f0: {
                                                'b_2664e8: {
                                                    'b_26648c: {
                                                        c.cycles += 6;
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xfa0));
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                        c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                                                        let t = c.g[A0][0] as u32;
                                                        match t {
                                                            0x0026_6358 => {
                                                                if event_due(c) {
                                                                    deliver_event_here(c, 0x0026_6358);
                                                                }
                                                                continue 'l_266358;
                                                            }
                                                            0x0026_6454 => {
                                                                'b_266484: {
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2f4)).lw(c);
                                                                    let le = (c.g[V0][0] as i64) <= 0;
                                                                    c.g[V0][0] = 0x1;
                                                                    if !le {
                                                                        c.cycles += 9;
                                                                        Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
                                                                        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2f4)).lw(c);
                                                                        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2e8)).lw(c);
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                                                        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2ec)).lw(c);
                                                                        Addr((c.g[GP][0] as u32).wrapping_add(0x2f4)).sw(c, c.g[V1][0]);
                                                                        call_ptr(c, callee::MC_POLL_2C67E8, 0x0026_647c)?;
                                                                        c.g[V0][0] = 0;
                                                                        break 'b_266534;
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                Addr((c.g[GP][0] as u32).wrapping_add(0x2f0)).sw(c, c.g[V0][0]);
                                                                break 'b_266530;
                                                            }
                                                            0x0026_648c => break 'b_26648c,
                                                            0x0026_64e8 => break 'b_2664e8,
                                                            0x0026_64f0 => break 'b_2664f0,
                                                            0x0026_64f8 => break 'b_2664f8,
                                                            0x0026_6510 => break 'b_266510,
                                                            0x0026_6530 => break 'b_266530,
                                                            _ => {
                                                                c.jump_out(t, 0x0026_6358, 0x0026_654c);
                                                                return Ok(());
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 4;
                                                    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2e8)).lw(c);
                                                    c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x54)).lw(c);
                                                    if (c.g[V0][0] as i64) <= 0 {
                                                        Addr(c.g[S2][0] as u32).sw(c, 0);
                                                        break 'b_266530;
                                                    }
                                                    'b_2664e0: {
                                                        c.cycles += 2;
                                                        let le = (c.g[V0][0] as i64) <= 0;
                                                        c.g[S0][0] = 0;
                                                        if !le {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = 0x53_0000;
                                                            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7190));
                                                            c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2e8)).lw(c);
                                                            'l_2664b0: loop {
                                                                c.cycles += 7;
                                                                c.g[A1][0] = c.g[S0][0];
                                                                c.g[A2][0] = c.g[S1][0];
                                                                call_ptr(c, callee::MC_POLL_2C68C8, 0x0026_64bc)?;
                                                                c.g[A0][0] = c.g[S1][0];
                                                                call_ptr(c, callee::MC_POLL_3FA360, 0x0026_64c4)?;
                                                                let nz = c.g[V0][0] != 0;
                                                                c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2e8)).lw(c);
                                                                if nz {
                                                                    break 'b_266500;
                                                                }
                                                                c.cycles += 5;
                                                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x54)).lw(c);
                                                                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                                                if c.g[V0][0] != 0 {
                                                                    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2e8)).lw(c);
                                                                    if event_due(c) {
                                                                        deliver_event_here(c, 0x0026_64b0);
                                                                    }
                                                                    continue 'l_2664b0;
                                                                }
                                                                break 'b_2664e0;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    Addr(c.g[S2][0] as u32).sw(c, 0);
                                                    break 'b_266530;
                                                }
                                                c.cycles += 2;
                                                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x850)).lw(c);
                                                break 'b_26652c;
                                            }
                                            c.cycles += 2;
                                            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x850)).lw(c);
                                            break 'b_26652c;
                                        }
                                        c.cycles += 2;
                                        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x850)).lw(c);
                                        break 'b_26652c;
                                    }
                                    c.cycles += 1;
                                    c.g[V1][0] = 0x1;
                                    break 'b_266504;
                                }
                                c.cycles += 3;
                                c.g[V0][0] = 0;
                                Addr(c.g[S2][0] as u32).sw(c, c.g[V1][0]);
                                break 'b_266534;
                            }
                            c.cycles += 7;
                            c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2e8)).lw(c);
                            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xa0)).lh(c);
                            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xa4)).lw(c);
                            let tgt = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, tgt, 0x0026_6528)?;
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 10);
                            break 'b_26652c;
                        }
                        c.cycles += 1;
                        Addr(c.g[S2][0] as u32).sw(c, c.g[V0][0]);
                        break 'b_266530;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    break 'b_266534;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x30);
                break 'b_266538;
            }
            c.cycles += 5;
            frame.restore(c, S1, 0x20);
            frame.restore(c, S2, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    })
}

// ---------------------------------------------------------------------------
// cSharedLoginMan_Update.

/// `void cSharedLoginMan_Update(void* self)` (0x265a68): advance the shared
/// login state machine: call the current state's handler (vtable at the
/// manager at gp+0x2e8) and, for states < 0x17, dispatch through the table
/// at 0x480f40 to the Do* stage for that state. Value jump table, no
/// back-edge event checks (start entry only, as generated).
pub fn c_shared_login_man_update(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x30);
        'b_265ca0: {
            'b_265c9c: {
                c.cycles += 14;
                frame.save(c, S0, 0x20);
                frame.save_ra(c, 0x10);
                c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2e8)).lw(c);
                c.g[S0][0] = c.g[A0][0];
                c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x18)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x1c)).lw(c);
                let tgt = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, tgt, 0x0026_5a90)?;
                c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[V0][0] = (c.g[V1][0] < 0x17) as u64;
                let ge = c.g[V0][0] == 0;
                Addr(frame.at(c, 0)).sw(c, 0);
                if ge {
                    break 'b_265c9c;
                }
                'b_265c98: {
                    'b_265c68: {
                        'b_265c28: {
                            'b_265be4: {
                                'b_265bac: {
                                    'b_265b7c: {
                                        'b_265b4c: {
                                            'b_265b1c: {
                                                'b_265aec: {
                                                    c.cycles += 7;
                                                    c.g[V0][0] = 0x48_0000;
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf40));
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                    c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                                                    let t = c.g[A0][0] as u32;
                                                    match t {
                                                        0x0026_5abc => {
                                                            c.cycles += 8;
                                                            c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2e8)).lw(c);
                                                            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                                                            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x58)).lh(c);
                                                            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c)).lw(c);
                                                            let tgt2 = c.g[V0][0] as u32;
                                                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                                            call_ptr(c, tgt2, 0x0026_5ad4)?;
                                                            if c.g[V0][0] != 0 {
                                                                frame.restore(c, S0, 0x20);
                                                                break 'b_265ca0;
                                                            }
                                                            c.cycles += 4;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            callee::DO_LOAD.call(c, 0x0026_5ae4)?;
                                                            frame.restore(c, S0, 0x20);
                                                            break 'b_265ca0;
                                                        }
                                                        0x0026_5aec => break 'b_265aec,
                                                        0x0026_5b1c => break 'b_265b1c,
                                                        0x0026_5b4c => break 'b_265b4c,
                                                        0x0026_5b7c => break 'b_265b7c,
                                                        0x0026_5bac => break 'b_265bac,
                                                        0x0026_5be4 => break 'b_265be4,
                                                        0x0026_5c28 => break 'b_265c28,
                                                        0x0026_5c68 => break 'b_265c68,
                                                        0x0026_5c9c => break 'b_265c9c,
                                                        _ => {
                                                            c.jump_out(t, 0x0026_5a68, 0x0026_5cac);
                                                            return Ok(());
                                                        }
                                                    }
                                                }
                                                c.cycles += 8;
                                                c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2e8)).lw(c);
                                                c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                                                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x58)).lh(c);
                                                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c)).lw(c);
                                                let tgt = c.g[V0][0] as u32;
                                                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, tgt, 0x0026_5b04)?;
                                                if c.g[V0][0] != 0 {
                                                    frame.restore(c, S0, 0x20);
                                                    break 'b_265ca0;
                                                }
                                                c.cycles += 4;
                                                c.g[A0][0] = c.g[S0][0];
                                                callee::DO_UPDATE.call(c, 0x0026_5b14)?;
                                                frame.restore(c, S0, 0x20);
                                                break 'b_265ca0;
                                            }
                                            c.cycles += 8;
                                            c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2e8)).lw(c);
                                            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                                            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x58)).lh(c);
                                            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c)).lw(c);
                                            let tgt = c.g[V0][0] as u32;
                                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, tgt, 0x0026_5b34)?;
                                            if c.g[V0][0] != 0 {
                                                frame.restore(c, S0, 0x20);
                                                break 'b_265ca0;
                                            }
                                            c.cycles += 4;
                                            c.g[A0][0] = c.g[S0][0];
                                            callee::DO_CREATE.call(c, 0x0026_5b44)?;
                                            frame.restore(c, S0, 0x20);
                                            break 'b_265ca0;
                                        }
                                        c.cycles += 8;
                                        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2e8)).lw(c);
                                        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                                        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x58)).lh(c);
                                        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c)).lw(c);
                                        let tgt = c.g[V0][0] as u32;
                                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, tgt, 0x0026_5b64)?;
                                        if c.g[V0][0] != 0 {
                                            frame.restore(c, S0, 0x20);
                                            break 'b_265ca0;
                                        }
                                        c.cycles += 4;
                                        c.g[A0][0] = c.g[S0][0];
                                        callee::DO_DELETE.call(c, 0x0026_5b74)?;
                                        frame.restore(c, S0, 0x20);
                                        break 'b_265ca0;
                                    }
                                    c.cycles += 8;
                                    c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2e8)).lw(c);
                                    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                                    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x58)).lh(c);
                                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c)).lw(c);
                                    let tgt = c.g[V0][0] as u32;
                                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, tgt, 0x0026_5b94)?;
                                    if c.g[V0][0] != 0 {
                                        frame.restore(c, S0, 0x20);
                                        break 'b_265ca0;
                                    }
                                    c.cycles += 4;
                                    c.g[A0][0] = c.g[S0][0];
                                    callee::DO_FINISH.call(c, 0x0026_5ba4)?;
                                    frame.restore(c, S0, 0x20);
                                    break 'b_265ca0;
                                }
                                'b_265bd8: {
                                    c.cycles += 4;
                                    c.g[A0][0] = c.g[SP][0];
                                    call_ptr(c, callee::UPDATE_3F80E0, 0x0026_5bb4)?;
                                    let nz = c.g[V0][0] != 0;
                                    Addr((c.g[S0][0] as u32).wrapping_add(0x4)).sw(c, c.g[V0][0]);
                                    if nz {
                                        break 'b_265bd8;
                                    }
                                    c.cycles += 4;
                                    c.g[V1][0] = Addr(frame.at(c, 0)).lw(c);
                                    c.g[V0][0] = 0x2;
                                    if c.g[V1][0] != c.g[V0][0] {
                                        frame.restore(c, S0, 0x20);
                                        break 'b_265ca0;
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                    break 'b_265c98;
                                }
                                c.cycles += 3;
                                c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
                                break 'b_265c98;
                            }
                            c.cycles += 8;
                            c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2e8)).lw(c);
                            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x58)).lh(c);
                            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c)).lw(c);
                            let tgt = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, tgt, 0x0026_5bfc)?;
                            let nz = c.g[V0][0] != 0;
                            frame.restore(c, S0, 0x20);
                            if nz {
                                break 'b_265ca0;
                            }
                            c.cycles += 9;
                            c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2e8)).lw(c);
                            c.g[A1][0] = 0;
                            c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x28)).lh(c);
                            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x2c)).lw(c);
                            let tgt2 = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, tgt2, 0x0026_5c20)?;
                            frame.restore(c, S0, 0x20);
                            break 'b_265ca0;
                        }
                        c.cycles += 8;
                        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2e8)).lw(c);
                        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x58)).lh(c);
                        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c)).lw(c);
                        let tgt = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, tgt, 0x0026_5c40)?;
                        if c.g[V0][0] != 0 {
                            frame.restore(c, S0, 0x20);
                            break 'b_265ca0;
                        }
                        c.cycles += 8;
                        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2e8)).lw(c);
                        c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
                        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x40)).lh(c);
                        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x44)).lw(c);
                        let tgt2 = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, tgt2, 0x0026_5c60)?;
                        c.g[V0][0] = 0x14;
                        break 'b_265c98;
                    }
                    c.cycles += 8;
                    c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2e8)).lw(c);
                    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x58)).lh(c);
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c)).lw(c);
                    let tgt = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, tgt, 0x0026_5c80)?;
                    if c.g[V0][0] != 0 {
                        frame.restore(c, S0, 0x20);
                        break 'b_265ca0;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x850)).lw(c);
                    let nz = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x16;
                    if nz {
                        break 'b_265c98;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 0x15;
                    break 'b_265c98;
                }
                c.cycles += 1;
                Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
                break 'b_265c9c;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
            break 'b_265ca0;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{fall_back, hook, machine, prev_cell, FAKE};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    pub(crate) const OC402_CTRL: u32 = DATA + 0xe_0000;
    pub(crate) const OC402_MAGIC: u32 = 0x0c40_2001;

    fn oc402_fake_alloc(c: &mut Cpu) {
        if c.bus.read32(OC402_CTRL) != OC402_MAGIC {
            return fall_back(c, prev_cell(oc402_fake_alloc), ee::s10::f_00317d70__generated);
        }
        let n = c.bus.read32(OC402_CTRL + 4);
        c.bus.write32(OC402_CTRL + 4, n + 1);
        c.g[V0][0] = c.bus.read32(OC402_CTRL + 0x10 + 4 * (n & 7)) as u64;
        c.cycles += 11;
        ret(c);
    }

    fn oc402_fake_strcpy(c: &mut Cpu) {
        if c.bus.read32(OC402_CTRL) != OC402_MAGIC {
            return fall_back(c, prev_cell(oc402_fake_strcpy), ee::s15::f_004166f4__generated);
        }
        let dst = c.g[A0][0] as u32;
        let src = c.g[A1][0] as u32;
        for i in 0..0x1000u32 {
            let b = c.bus.read8(src.wrapping_add(i));
            c.bus.write8(dst.wrapping_add(i), b);
            if b == 0 {
                break;
            }
        }
        c.g[V0][0] = c.g[A0][0];
        c.cycles += 5;
        ret(c);
    }

    fn oc402_fake_delete(c: &mut Cpu) {
        if c.bus.read32(OC402_CTRL) != OC402_MAGIC {
            return fall_back(c, prev_cell(oc402_fake_delete), ee::s10::f_00317e50__generated);
        }
        c.cycles += 3;
        ret(c);
    }

    fn oc402_hooks() {
        hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc402_fake_alloc,
            prev_cell(oc402_fake_alloc),
        );
        hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_004166F4),
            oc402_fake_strcpy,
            prev_cell(oc402_fake_strcpy),
        );
        hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc402_fake_delete,
            prev_cell(oc402_fake_delete),
        );
    }

    #[test]
    fn net_event10d_clone_matches() {
        oc402_hooks();
        check(ee::s07::f_002687c0__generated, net_event10d_clone, 60, &|c, seed| {
            let _r = machine(c, seed, 0x2687c0, 0);
            c.pc = 0x0026_87c0;
            c.bus.write32(OC402_CTRL, OC402_MAGIC);
            c.bus.write32(OC402_CTRL + 4, 0);
            c.bus.write32(OC402_CTRL + 0x10, DATA + 0x4000);
            let self_ = DATA;
            c.g[A0][0] = self_ as u64;
            for (off, base) in [(0x8u32, b'a'), (0x28, b'm'), (0x48, b'z')] {
                for i in 0..8u32 {
                    c.bus.write8(self_ + off + i, base + ((seed + i) % 20) as u8);
                }
                c.bus.write8(self_ + off + 8, 0);
            }
            c.bus.write32(self_ + 0x148, 0x1111 * seed);
            c.bus.write32(self_ + 0x14c, 0x2222 * seed);
            for i in 0..0x14u32 {
                c.bus.write8(self_ + 0x150 + i, (seed + i) as u8);
            }
        });
    }

    #[test]
    fn lobby_man_list_clear_matches() {
        oc402_hooks();
        check(ee::s07::f_002670c8__generated, lobby_man_list_clear, 60, &|c, seed| {
            let _r = machine(c, seed, 0x2670c8, 0);
            c.pc = 0x0026_70c8;
            c.bus.write32(OC402_CTRL, OC402_MAGIC);
            let self_ = DATA;
            let head = DATA + 0x100;
            let n1 = DATA + 0x200;
            let n2 = DATA + 0x300;
            c.g[A0][0] = self_ as u64;
            c.bus.write32(self_ + LIST_HEAD, head);
            match seed % 3 {
                0 => {
                    c.bus.write32(head, head);
                    c.bus.write32(head + 4, head);
                }
                1 => {
                    c.bus.write32(head, n1);
                    c.bus.write32(head + 4, n1);
                    c.bus.write32(n1, head);
                    c.bus.write32(n1 + 4, head);
                }
                _ => {
                    c.bus.write32(head, n1);
                    c.bus.write32(head + 4, n2);
                    c.bus.write32(n1, n2);
                    c.bus.write32(n1 + 4, head);
                    c.bus.write32(n2, head);
                    c.bus.write32(n2 + 4, n1);
                }
            }
        });
    }

    #[test]
    fn c_voip_man_debug_printf_stub_matches() {
        check(
            ee::s07::f_00266fc0__generated,
            c_voip_man_debug_printf_stub,
            30,
            &|c, seed| {
                let mut r = Rng(0x266fc0 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0026_6fc0;
                c.g[A0][0] = (DATA + 0x100 * (seed % 8)) as u64;
            },
        );
    }

    #[test]
    fn c_usb_keyboard_dtor_matches() {
        oc402_hooks();
        check(ee::s07::f_00266850__generated, c_usb_keyboard_dtor, 40, &|c, seed| {
            let _r = machine(c, seed, 0x266850, 0);
            c.pc = 0x0026_6850;
            c.bus.write32(OC402_CTRL, OC402_MAGIC);
            let self_ = DATA + 0x1000;
            c.g[A0][0] = self_ as u64;
            c.g[A1][0] = (seed % 4) as u64;
            c.bus.write32(self_ + USBKB_STATE_WORD, 0);
        });
    }

    fn oc402_fake_new(c: &mut Cpu) {
        if c.bus.read32(OC402_CTRL) != OC402_MAGIC {
            return fall_back(c, prev_cell(oc402_fake_new), ee::s10::f_00317e30__generated);
        }
        c.g[V0][0] = c.bus.read32(OC402_CTRL + 0x18) as u64;
        c.cycles += 7;
        ret(c);
    }

    fn oc402_fake_free(c: &mut Cpu) {
        if c.bus.read32(OC402_CTRL) != OC402_MAGIC {
            return fall_back(c, prev_cell(oc402_fake_free), ee::s10::f_00317e98__generated);
        }
        c.cycles += 3;
        ret(c);
    }

    fn oc402_hooks2() {
        oc402_hooks();
        hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc402_fake_new,
            prev_cell(oc402_fake_new),
        );
        hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc402_fake_free,
            prev_cell(oc402_fake_free),
        );
    }

    fn buddy_vec_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        let _r = machine(c, seed, entry as u64, 0);
        c.pc = entry;
        c.bus.write32(OC402_CTRL, OC402_MAGIC);
        c.bus.write32(OC402_CTRL + 4, 0);
        c.bus.write32(OC402_CTRL + 0x10, DATA + 0x4000);
        c.bus.write32(OC402_CTRL + 0x18, DATA + 0x5000);
        let self_ = DATA;
        let buf = DATA + 0x1000;
        let xref = DATA + 0x2000;
        c.bus.write32(xref, DATA + 0x3000);
        c.g[A0][0] = self_ as u64;
        c.g[A2][0] = xref as u64;
        for i in 0..8u32 {
            c.bus.write32(buf + 4 * i, 0x1000 + i);
        }
        match seed % 6 {
            0 => {
                // not full, insert mid (3 elements; tail copy is 4 bytes)
                c.bus.write32(self_ + BUDDY_VEC_FIRST, buf);
                c.bus.write32(self_ + BUDDY_VEC_LAST, buf + 12);
                c.bus.write32(self_ + BUDDY_VEC_CAP, buf + 16);
                c.g[A1][0] = (buf + 4) as u64;
            }
            1 => {
                // not full, insert at the beginning (1 element; tail copy 0)
                c.bus.write32(self_ + BUDDY_VEC_FIRST, buf);
                c.bus.write32(self_ + BUDDY_VEC_LAST, buf + 4);
                c.bus.write32(self_ + BUDDY_VEC_CAP, buf + 16);
                c.g[A1][0] = buf as u64;
            }
            2 => {
                // full, count 1 -> operator-new grow, delete-old
                c.bus.write32(self_ + BUDDY_VEC_FIRST, buf);
                c.bus.write32(self_ + BUDDY_VEC_LAST, buf + 4);
                c.bus.write32(self_ + BUDDY_VEC_CAP, buf + 4);
                c.g[A1][0] = buf as u64;
            }
            3 => {
                // full, count 2 -> operator-new grow, free-old
                c.bus.write32(self_ + BUDDY_VEC_FIRST, buf);
                c.bus.write32(self_ + BUDDY_VEC_LAST, buf + 8);
                c.bus.write32(self_ + BUDDY_VEC_CAP, buf + 8);
                c.g[A1][0] = (buf + 4) as u64;
            }
            4 => {
                // full, count 0 (non-null) -> named-alloc grow, free-old
                c.bus.write32(self_ + BUDDY_VEC_FIRST, buf);
                c.bus.write32(self_ + BUDDY_VEC_LAST, buf);
                c.bus.write32(self_ + BUDDY_VEC_CAP, buf);
                c.g[A1][0] = buf as u64;
            }
            _ => {
                // full, empty null vector -> named-alloc grow, no free
                c.bus.write32(self_ + BUDDY_VEC_FIRST, 0);
                c.bus.write32(self_ + BUDDY_VEC_LAST, 0);
                c.bus.write32(self_ + BUDDY_VEC_CAP, 0);
                c.g[A1][0] = 0;
            }
        }
    }

    #[test]
    fn buddy_man_message_vector_insert_matches() {
        oc402_hooks2();
        check(ee::s07::f_00267128__generated, buddy_man_message_vector_insert, 60, &|c, seed| {
            buddy_vec_fixture(c, seed, 0x267128);
        });
    }

    #[test]
    fn buddy_man_challenger_vector_insert_matches() {
        oc402_hooks2();
        check(ee::s07::f_002672c8__generated, buddy_man_challenger_vector_insert, 60, &|c, seed| {
            buddy_vec_fixture(c, seed, 0x2672c8);
        });
    }

    fn mc_map_fixture(c: &mut Cpu, seed: u32) {
        let _r = machine(c, seed, 0x266578, 0);
        c.pc = 0x0026_6578;
        let mode = (seed % 16) as u32;
        c.bus.write32(GAME_GP + 0x2e4, mode);
        let err = (seed % 9) as i32 - 6;
        c.g[A0][0] = err as i64 as u64;
        const ARMS: [u32; 10] = [
            0x0026_65a8, 0x0026_65d0, 0x0026_65e4, 0x0026_6614, 0x0026_6630,
            0x0026_6660, 0x0026_6674, 0x0026_6698, 0x0026_66c0, 0x0026_66e4,
        ];
        for i in 0..14u32 {
            c.bus.write32(0x0048_0fe0 + 4 * i, ARMS[((seed + i) % 10) as usize]);
        }
        // Sub-tables (0x481020/40/60) get only arms that return directly:
        // a dispatch arm in a sub-table can index back to itself and spin
        // (seed 9: arm 0x266698 -> table 0x481060 -> 0x266698).
        const RET_ARMS: [u32; 7] = [
            0x0026_65c0, 0x0026_65c8, 0x0026_65dc, 0x0026_660c, 0x0026_6658, 0x0026_6708, 0x0026_6710,
        ];
        for i in 0..6u32 {
            c.bus.write32(0x0048_1020 + 4 * i, RET_ARMS[((seed + i) % 7) as usize]);
        }
        for i in 0..7u32 {
            c.bus.write32(0x0048_1040 + 4 * i, RET_ARMS[((seed + i + 2) % 7) as usize]);
            c.bus.write32(0x0048_1060 + 4 * i, RET_ARMS[((seed + i + 4) % 7) as usize]);
        }
    }

    #[test]
    fn c_shared_login_man_mc_map_error_matches() {
        check(ee::s07::f_00266578__generated, c_shared_login_man_mc_map_error, 96, &|c, seed| {
            mc_map_fixture(c, seed);
        });
    }

    #[test]
    fn c_shared_login_man_mc_map_error_events() {
        use crate::difftest::events::{check_events, schedule, Events};
        check_events(ee::s07::f_00266578__generated, c_shared_login_man_mc_map_error, 96, &|c, seed| {
            mc_map_fixture(c, seed);
            let mut r = Rng(0x266578 ^ seed as u64);
            // Force an arm with an event check: mode 4 -> first table slot.
            c.bus.write32(GAME_GP + 0x2e4, 4);
            c.bus.write32(0x0048_0fe0 + 12, 0x0026_6614);
            c.g[A0][0] = (-4i64) as u64;
            let _m: Events = schedule(c, &mut r, seed, 0x0026_6578);
            _m
        });
    }

    fn mc_mgr_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        let _r = machine(c, seed, entry as u64, 0);
        c.pc = entry;
        let mgr = DATA + 0x1000;
        let obj = DATA + 0x2000;
        c.bus.write32(GAME_GP + 0x2e8, mgr);
        c.bus.write32(mgr, obj);
        c.bus.write32(mgr + 0xc, DATA + 0x3000);
        c.bus.write32(obj + 0x54, 0);
        c.bus.write32(obj + 0x58, 0);
        c.bus.write32(obj + 0x5c, 0x0014_06c0);
        c.bus.write32(obj + 0x18, 0);
        c.bus.write32(obj + 0x1c, 0x0014_06c0);
        c.bus.write32(obj + 0x28, 0);
        c.bus.write32(obj + 0x2c, 0x0014_06c0);
        c.bus.write32(obj + 0x40, 0);
        c.bus.write32(obj + 0x44, 0x0014_06c0);
        c.bus.write32(obj + 0xa0, 0);
        c.bus.write32(obj + 0xa4, 0x0014_06c0);
        c.bus.write32(GAME_GP + 0x2f4, 0);
        c.bus.write32(GAME_GP + 0x850, 0);
    }

    #[test]
    fn c_shared_login_man_mc_poll_matches() {
        check(ee::s07::f_00266358__generated, c_shared_login_man_mc_poll, 60, &|c, seed| {
            mc_mgr_fixture(c, seed, 0x266358);
            let self_ = DATA;
            let out = DATA + 0x800;
            c.g[A0][0] = self_ as u64;
            c.g[A1][0] = out as u64;
            let mode = [10u32, 3, 5, 12, 20, 14][(seed % 6) as usize];
            c.bus.write32(GAME_GP + 0x2e4, mode);
            // dispatch table at 0x480fa0 for mode 14
            for i in 0..15u32 {
                c.bus.write32(0x0048_0fa0 + 4 * i, 0x0026_6454);
            }
            c.bus.write32(0x0048_0fa0 + 4 * 14, [0x0026_6454, 0x0026_648c][(seed % 2) as usize]);
        });
    }

    #[test]
    fn c_shared_login_man_update_matches() {
        check(ee::s07::f_00265a68__generated, c_shared_login_man_update, 72, &|c, seed| {
            mc_mgr_fixture(c, seed, 0x265a68);
            let self_ = DATA;
            c.g[A0][0] = self_ as u64;
            let state = seed % 24;
            c.bus.write32(self_, state);
            c.bus.write32(self_ + 4, 0);
            const ARMS: [u32; 10] = [
                0x0026_5abc, 0x0026_5aec, 0x0026_5b1c, 0x0026_5b4c, 0x0026_5b7c,
                0x0026_5bac, 0x0026_5be4, 0x0026_5c28, 0x0026_5c68, 0x0026_5c9c,
            ];
            for i in 0..23u32 {
                c.bus.write32(0x0048_0f40 + 4 * i, ARMS[((seed + i) % 10) as usize]);
            }
        });
    }

    #[test]
    fn c_setup_man_send_quit_matches() {
        oc402_hooks();
        check(ee::s07::f_00265768__generated, c_setup_man_send_quit, 60, &|c, seed| {
            let _r = machine(c, seed, 0x265768, 0);
            c.pc = 0x0026_5768;
            c.bus.write32(OC402_CTRL, OC402_MAGIC);
            c.bus.write32(OC402_CTRL + 4, 0);
            c.bus.write32(OC402_CTRL + 0x10, DATA + 0x800);
            c.bus.write32(OC402_CTRL + 0x14, DATA + 0x900);
            c.bus.write32(GAME_GP - 0x1f04, DATA + 0x9000);
            c.bus.write32(GAME_GP - 0x848, DATA + 0x9100);
            let self_ = DATA;
            c.g[A0][0] = self_ as u64;
            let lobby = DATA + 0x500;
            c.bus.write32(GAME_GP - 0xc8, lobby);
            c.bus.write32(lobby, 0);
            let head = DATA + 0x600;
            c.bus.write32(lobby + 0xf0, head);
            c.bus.write32(head + 4, DATA + 0x700);
            c.bus.write32(self_ + 0x90, 0);
            c.bus.write32(self_ + 0x94, 0);
            c.bus.write32(self_ + 0x120, 0);
            let comm = DATA + 0xa00;
            c.bus.write32(GAME_GP - 0x238, comm);
            c.bus.write32(comm, DATA + 0xa100);
            c.bus.write32(DATA + 0xa100 + 8, DATA + 0xa200);
            c.bus.write16(DATA + 0xa200 + 0x40, 0);
            c.bus.write32(DATA + 0xa200 + 0x44, FAKE);
            match seed % 3 {
                0 => {
                    c.bus.write32(self_, 10);
                    c.g[A1][0] = 0x42;
                }
                1 => {
                    c.bus.write32(self_, 5);
                    c.g[A1][0] = 0x42;
                }
                _ => {
                    c.bus.write32(self_, 5);
                    c.g[A1][0] = SETUP_QUIT_ID as u64;
                }
            }
        });
    }
}
