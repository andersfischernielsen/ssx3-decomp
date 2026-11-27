//! Staging module oc1005 (wave wave260): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1005`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0022_13d8, ov_screen_session_dtor, "OVScreenSession_dtor"),
    (
        0x0022_1408,
        ov_screen_read_private_message_ctor,
        "OVScreenReadPrivateMessage_ctor",
    ),
    (
        0x0022_1468,
        ov_screen_read_private_message_ctor_2,
        "OVScreenReadPrivateMessage_ctor_2",
    ),
    (
        0x0022_14d0,
        ov_screen_read_private_message_dtor,
        "OVScreenReadPrivateMessage_dtor",
    ),
    (
        0x0022_14f8,
        ov_screen_pause_freeride_ctor,
        "OVScreenPauseFreeride_ctor",
    ),
    (
        0x0022_1558,
        ov_screen_pause_freeride_ctor_2,
        "OVScreenPauseFreeride_ctor_2",
    ),
    (
        0x0022_15c8,
        ov_screen_pause_freeride_dtor,
        "OVScreenPauseFreeride_dtor",
    ),
    (
        0x0022_15f0,
        ov_screen_pause_mode1_ctor,
        "OVScreenPauseMode1_ctor",
    ),
];

/// `UIState_ctor` (0x39e2a0), the base constructor every `OVState` runs first.
const OV_STATE_UI_CTOR: u32 = 0x0039_e2a0;
/// `UIState_dtor` (0x39e390), run last by every `OVState` destructor.
const OV_STATE_UI_DTOR: u32 = 0x0039_e390;
/// `cOVState_vtbl`: the vtable every `OVState` starts with (stored at +0x8).
const OV_STATE_BASE_VTBL: u32 = 0x0047_4e08;
/// The game-image data page (`0x4a0000`) holding `str_sp_b[0]` at +0x2088.
const OV_STATE_DATA_PAGE: u32 = 0x004a_0000;

/// The field stores every `OVState` constructor makes after the base
/// `UIState` constructor has returned (`this` is `s0`): the overlay id at
/// +0x98, the transition, pause-menu and PDA flags (+0x48, +0x4c, +0x54)
/// cleared, the vtable at +0x8 (written twice: the base one, then `vptr`,
/// which is what the object finally has), the chat buffer's first halfword
/// at +0x58 and, for the pause screens (`pause_thread`), the thread at +0xc4.
/// Leaves `v0` = this, `v1` = `vptr`, `a1` = the data page, `a0` = the
/// halfword, as the generated body does.
fn ov_state_fill_fields(c: &mut Cpu, ov_id: u64, vptr: u32, pause_thread: bool) {
    let obj = c.g[S0][0] as u32;
    c.sw(obj.wrapping_add(0x98), ov_id);
    c.sw(obj.wrapping_add(0x48), 0);
    c.sw(obj.wrapping_add(0x8), OV_STATE_BASE_VTBL as u64);
    c.sw(obj.wrapping_add(0x4c), 0);
    c.sw(obj.wrapping_add(0x54), 0);
    c.g[V1][0] = vptr as u64;
    c.sw(obj.wrapping_add(0x8), c.g[V1][0]);
    c.g[V0][0] = c.g[S0][0];
    c.g[A1][0] = OV_STATE_DATA_PAGE as u64;
    c.g[A0][0] = c.lhu(OV_STATE_DATA_PAGE.wrapping_add(0x2088));
    c.sh(obj.wrapping_add(0x58), c.g[A0][0]);
    if pause_thread {
        c.sw(obj.wrapping_add(0xc4), 0);
    }
}

/// The body shared by the three `OVState` destructors: set the base vtable
/// on `this` (a0, copied to v1), run `UIState_dtor` on it and return to `ret`.
fn ov_state_dtor_body(c: &mut Cpu, ret: u32) -> Flow {
    c.cycles += 10;
    let f = Frame::open(c, 0x10);
    f.save_ra(c, 0);
    c.g[V1][0] = c.g[A0][0];
    c.g[V0][0] = OV_STATE_BASE_VTBL as u64;
    let this = c.g[V1][0] as u32;
    c.sw(this.wrapping_add(0x8), c.g[V0][0]);
    call_ptr(c, OV_STATE_UI_DTOR, ret)?;
    f.restore_ra(c, 0);
    f.close(c);
    Ok(())
}

/// `OVScreenSession_dtor` (0x2213d8): the destructor of the session overlay
/// state. Sets the base vtable and runs `UIState_dtor`.
pub fn ov_screen_session_dtor(c: &mut Cpu) {
    guest(c, |c| ov_state_dtor_body(c, 0x0022_13f4));
}

/// `OVScreenReadPrivateMessage_ctor` (0x221408): constructs the read-private-
/// message overlay state in `a0` with overlay id 0 and the message vtable.
pub fn ov_screen_read_private_message_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let f = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        f.save(c, 16, 0x10);
        f.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, OV_STATE_UI_CTOR, 0x0022_1420)?;
        ov_state_fill_fields(c, 0, 0x0047_3248, false);
        f.restore(c, 16, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `OVScreenReadPrivateMessage_ctor_2` (0x221468): as the constructor above,
/// with the overlay id taken from `a2`.
pub fn ov_screen_read_private_message_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let f = Frame::open(c, 0x30);
        f.save(c, 16, 0x20);
        c.g[S0][0] = c.g[A0][0];
        f.save(c, 17, 0x10);
        f.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, OV_STATE_UI_CTOR, 0x0022_1484)?;
        let ov_id = c.g[S1][0];
        ov_state_fill_fields(c, ov_id, 0x0047_3248, false);
        f.restore(c, 17, 0x10);
        f.restore(c, 16, 0x20);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `OVScreenReadPrivateMessage_dtor` (0x2214d0): the same destructor as the
/// session overlay's (base vtable, then `UIState_dtor`).
pub fn ov_screen_read_private_message_dtor(c: &mut Cpu) {
    guest(c, |c| ov_state_dtor_body(c, 0x0022_14ec));
}

/// `OVScreenPauseFreeride_ctor` (0x2214f8): the free-ride pause overlay state,
/// overlay id 0, pause-menu template vtable.
pub fn ov_screen_pause_freeride_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let f = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        f.save(c, 16, 0x10);
        f.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, OV_STATE_UI_CTOR, 0x0022_1510)?;
        ov_state_fill_fields(c, 0, 0x0047_3170, false);
        f.restore(c, 16, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `OVScreenPauseFreeride_ctor_2` (0x221558): as the constructor above, with
/// the overlay id from `a2` and the pause thread cleared.
pub fn ov_screen_pause_freeride_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let f = Frame::open(c, 0x30);
        f.save(c, 16, 0x20);
        c.g[S0][0] = c.g[A0][0];
        f.save(c, 17, 0x10);
        f.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, OV_STATE_UI_CTOR, 0x0022_1574)?;
        let ov_id = c.g[S1][0];
        ov_state_fill_fields(c, ov_id, 0x0047_3170, true);
        f.restore(c, 17, 0x10);
        f.restore(c, 16, 0x20);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `OVScreenPauseFreeride_dtor` (0x2215c8): the free-ride pause overlay's
/// destructor (base vtable, then `UIState_dtor`).
pub fn ov_screen_pause_freeride_dtor(c: &mut Cpu) {
    guest(c, |c| ov_state_dtor_body(c, 0x0022_15e4));
}

/// `OVScreenPauseMode1_ctor` (0x2215f0): the pause mode 1 overlay state
/// (competition pause vtable, pause thread cleared, overlay id 0).
pub fn ov_screen_pause_mode1_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let f = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        f.save(c, 16, 0x10);
        f.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, OV_STATE_UI_CTOR, 0x0022_1608)?;
        ov_state_fill_fields(c, 0, 0x0047_3098, true);
        f.restore(c, 16, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA};

    /// Random junk in every register and in the object (+0..0x100) at
    /// `DATA + 0x1000`, the object in `a0`. The constructors and destructors
    /// have no branches, so one path is the whole space; the junk makes sure
    /// every stored field is visibly written.
    fn ov_object_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x0022_1400 ^ seed as u64);
        scramble(c, &mut r);
        let obj = DATA + 0x1000;
        for k in 0..0x40 {
            c.bus.write32(obj + 4 * k, r.next_u32());
        }
        c.g[A0][0] = sx(obj);
    }

    #[test]
    fn ov_screen_session_dtor_matches() {
        check(
            ee::s05::f_002213d8__generated,
            ov_screen_session_dtor,
            200,
            &|c, seed| ov_object_setup(c, seed),
        );
    }

    #[test]
    fn ov_screen_read_private_message_ctor_matches() {
        check(
            ee::s05::f_00221408__generated,
            ov_screen_read_private_message_ctor,
            200,
            &|c, seed| ov_object_setup(c, seed),
        );
    }

    #[test]
    fn ov_screen_read_private_message_ctor_2_matches() {
        check(
            ee::s05::f_00221468__generated,
            ov_screen_read_private_message_ctor_2,
            200,
            &|c, seed| ov_object_setup(c, seed),
        );
    }

    #[test]
    fn ov_screen_read_private_message_dtor_matches() {
        check(
            ee::s05::f_002214d0__generated,
            ov_screen_read_private_message_dtor,
            200,
            &|c, seed| ov_object_setup(c, seed),
        );
    }

    #[test]
    fn ov_screen_pause_freeride_ctor_matches() {
        check(
            ee::s05::f_002214f8__generated,
            ov_screen_pause_freeride_ctor,
            200,
            &|c, seed| ov_object_setup(c, seed),
        );
    }

    #[test]
    fn ov_screen_pause_freeride_ctor_2_matches() {
        check(
            ee::s05::f_00221558__generated,
            ov_screen_pause_freeride_ctor_2,
            200,
            &|c, seed| ov_object_setup(c, seed),
        );
    }

    #[test]
    fn ov_screen_pause_freeride_dtor_matches() {
        check(
            ee::s05::f_002215c8__generated,
            ov_screen_pause_freeride_dtor,
            200,
            &|c, seed| ov_object_setup(c, seed),
        );
    }

    #[test]
    fn ov_screen_pause_mode1_ctor_matches() {
        check(
            ee::s05::f_002215f0__generated,
            ov_screen_pause_mode1_ctor,
            200,
            &|c, seed| ov_object_setup(c, seed),
        );
    }
}
