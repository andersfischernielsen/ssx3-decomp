//! Staging module oc1006 (wave wave260): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1006`.

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
    (
        0x0022_1658,
        ov_screen_pause_mode1_ctor_2,
        "OVScreenPauseMode1_ctor_2",
    ),
    (
        0x0022_16c8,
        ov_screen_pause_mode1_dtor,
        "OVScreenPauseMode1_dtor",
    ),
    (
        0x0022_16f0,
        ov_screen_pause_mode2_ctor,
        "OVScreenPauseMode2_ctor",
    ),
    (
        0x0022_1758,
        ov_screen_pause_mode2_ctor_2,
        "OVScreenPauseMode2_ctor_2",
    ),
    (
        0x0022_17c8,
        ov_screen_pause_mode2_dtor,
        "OVScreenPauseMode2_dtor",
    ),
];

/// `UIState_ctor(this, flags)` (0x39e2a0), the base constructor of the pause
/// overlay states.
const OV_PAUSE_UI_STATE_CTOR: u32 = 0x0039_e2a0;
/// `UIState_dtor(this, flags)` (0x39e390), the base destructor.
const OV_PAUSE_UI_STATE_DTOR: u32 = 0x0039_e390;
/// The `cOVState` base vtable (`0x470000 + 0x4e08`): written at `+8` first by
/// each pause constructor, and by both destructors.
const OV_PAUSE_BASE_VTABLE: u32 = 0x0047_4e08;
/// The `cOVState_PAUSE_COMPETE` vtable (`0x470000 + 0x3098`), Mode1's final one.
const OV_PAUSE_COMPETE_VTABLE: u32 = 0x0047_3098;
/// The `cOVState_PAUSE_BIGCHAL` vtable (`0x470000 + 0x2fc0`), Mode2's final one.
const OV_PAUSE_BIGCHAL_VTABLE: u32 = 0x0047_2fc0;
/// The data page the constructors read from (`0x4a0000`): the chat-buffer
/// initial value `str_sp_b[0]` is the halfword at `+0x2088`.
const OV_PAUSE_DATA_PAGE: u32 = 0x004a_0000;

/// The fields both pause constructors of the `_ctor_2` shape set after
/// `UIState_ctor(this, engine)`: the overlay id (`s1`) at `+0x98`, zeroed
/// words at `+0x48`, `+0x4c`, `+0x54`, the base vtable at `+8` and then
/// `final_vtable` at `+8`, the chat halfword at `+0x58`, and zero at `+0xc4`.
/// `ret_ctor` is the return address of the base constructor call.
fn ov_pause_ctor_2(c: &mut Cpu, ret_ctor: u32, final_vtable: u32) -> Flow {
    c.cycles += 27;
    let f = Frame::open(c, 0x30);
    f.save(c, S0, 0x20);
    c.g[S0][0] = c.g[A0][0];
    f.save(c, S1, 0x10);
    f.save_ra(c, 0);
    c.g[S1][0] = c.g[A2][0];
    call_ptr(c, OV_PAUSE_UI_STATE_CTOR, ret_ctor)?;

    let this = c.g[S0][0] as u32;
    c.g[V1][0] = 0x0047_0000;
    c.sw(this.wrapping_add(0x98), c.g[S1][0]);
    c.g[V1][0] = sx(OV_PAUSE_BASE_VTABLE);
    c.sw(this.wrapping_add(0x48), 0);
    c.sw(this.wrapping_add(0x8), c.g[V1][0]);
    c.g[A1][0] = sx(OV_PAUSE_DATA_PAGE);
    c.sw(this.wrapping_add(0x4c), 0);
    c.g[V1][0] = 0x0047_0000;
    c.sw(this.wrapping_add(0x54), 0);
    c.g[V1][0] = sx(final_vtable);
    c.g[V0][0] = c.g[S0][0];
    f.restore(c, S1, 0x10);
    c.g[A0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2088));
    c.sw(this.wrapping_add(0x8), c.g[V1][0]);
    c.sh(this.wrapping_add(0x58), c.g[A0][0]);
    c.sw(this.wrapping_add(0xc4), 0);
    f.restore(c, S0, 0x20);
    f.restore_ra(c, 0);
    f.close(c);
    Ok(())
}

/// `OVScreenPauseMode1_ctor_2(this, engine, overlayId)` (0x221658): the
/// COMPETE pause overlay's constructor (the overlay id is stored at `+0x98`).
/// Registers and cycles follow the generated body.
pub fn ov_screen_pause_mode1_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        ov_pause_ctor_2(c, 0x0022_1674, OV_PAUSE_COMPETE_VTABLE)
    });
}

/// `OVScreenPauseMode1_dtor(this, flags)` (0x2216c8): the base vtable at
/// `+8`, then `UIState_dtor(this, flags)`.
pub fn ov_screen_pause_mode1_dtor(c: &mut Cpu) {
    guest(c, |c| ov_pause_dtor(c, 0x0022_16e4));
}

/// The shared body of the two pause destructors: a 0x10 frame holding `ra`,
/// the base vtable at `this + 8`, then the base `UIState` destructor (the
/// argument registers pass through unchanged).
fn ov_pause_dtor(c: &mut Cpu, ret_dtor: u32) -> Flow {
    c.cycles += 10;
    let f = Frame::open(c, 0x10);
    f.save_ra(c, 0);
    c.g[V1][0] = c.g[A0][0];
    c.g[V0][0] = sx(OV_PAUSE_BASE_VTABLE);
    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    call_ptr(c, OV_PAUSE_UI_STATE_DTOR, ret_dtor)?;
    f.restore_ra(c, 0);
    f.close(c);
    Ok(())
}

/// `OVScreenPauseMode2_ctor(this)` (0x2216f0): `UIState_ctor(this, 0)`, then
/// the BIGCHAL overlay's fields. Unlike the `_ctor_2` shape it passes flags 0
/// and stores zero (not the overlay id) at `+0x98`.
pub fn ov_screen_pause_mode2_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let f = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        f.save(c, S0, 0x10);
        f.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, OV_PAUSE_UI_STATE_CTOR, 0x0022_1708)?;

        let this = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0x98), 0);
        c.g[V1][0] = sx(OV_PAUSE_BASE_VTABLE);
        c.sw(this.wrapping_add(0x48), 0);
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = sx(OV_PAUSE_DATA_PAGE);
        c.sw(this.wrapping_add(0x4c), 0);
        c.g[V1][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0x54), 0);
        c.g[V1][0] = sx(OV_PAUSE_BIGCHAL_VTABLE);
        c.g[V0][0] = c.g[S0][0];
        f.restore_ra(c, 0);
        c.g[A0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2088));
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.sh(this.wrapping_add(0x58), c.g[A0][0]);
        c.sw(this.wrapping_add(0xc4), 0);
        f.restore(c, S0, 0x10);
        f.close(c);
        Ok(())
    });
}

/// `OVScreenPauseMode2_ctor_2(this, engine, overlayId)` (0x221758): the
/// BIGCHAL pause overlay's constructor, the same shape as Mode1's `_ctor_2`
/// with the BIGCHAL vtable.
pub fn ov_screen_pause_mode2_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        ov_pause_ctor_2(c, 0x0022_1774, OV_PAUSE_BIGCHAL_VTABLE)
    });
}

/// `OVScreenPauseMode2_dtor(this, flags)` (0x2217c8): the same body as
/// Mode1's destructor, returning from its base call to `0x2217e4`.
pub fn ov_screen_pause_mode2_dtor(c: &mut Cpu) {
    guest(c, |c| ov_pause_dtor(c, 0x0022_17e4));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine_rt;
    use crate::difftest::{check, scramble, Rng, DATA};

    /// The overlay object the constructors and destructors work on.
    const OV_THIS: u32 = DATA + 0x2000;

    /// A machine with the runtime and allocator, `this` at [`OV_THIS`], and a
    /// random chat halfword in the data page (the constructors copy it).
    fn ov_pause_machine(c: &mut Cpu, r: &mut Rng, seed: u32, entry: u32) {
        scramble(c, r);
        machine_rt(c, seed, entry as u64, 0);
        c.bus
            .write16(OV_PAUSE_DATA_PAGE + 0x2088, r.next_u32() as u16);
        c.g[A0][0] = sx(OV_THIS);
        c.pc = entry;
    }

    #[test]
    fn ov_screen_pause_mode1_ctor_2_matches() {
        check(
            ee::s05::f_00221658__generated,
            ov_screen_pause_mode1_ctor_2,
            40,
            &|c, seed| {
                let mut r = Rng(0x0022_1658 ^ (seed as u64) << 8);
                ov_pause_machine(c, &mut r, seed, 0x0022_1658);
                c.g[A1][0] = sx(seed % 2);
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn ov_screen_pause_mode2_ctor_2_matches() {
        check(
            ee::s05::f_00221758__generated,
            ov_screen_pause_mode2_ctor_2,
            40,
            &|c, seed| {
                let mut r = Rng(0x0022_1758 ^ (seed as u64) << 8);
                ov_pause_machine(c, &mut r, seed, 0x0022_1758);
                c.g[A1][0] = sx(seed % 2);
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn ov_screen_pause_mode2_ctor_matches() {
        check(
            ee::s05::f_002216f0__generated,
            ov_screen_pause_mode2_ctor,
            40,
            &|c, seed| {
                let mut r = Rng(0x0022_16f0 ^ (seed as u64) << 8);
                ov_pause_machine(c, &mut r, seed, 0x0022_16f0);
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn ov_screen_pause_mode1_dtor_matches() {
        check(
            ee::s05::f_002216c8__generated,
            ov_screen_pause_mode1_dtor,
            40,
            &|c, seed| {
                let mut r = Rng(0x0022_16c8 ^ (seed as u64) << 8);
                ov_pause_machine(c, &mut r, seed, 0x0022_16c8);
                c.g[A1][0] = 0;
            },
        );
    }

    #[test]
    fn ov_screen_pause_mode2_dtor_matches() {
        check(
            ee::s05::f_002217c8__generated,
            ov_screen_pause_mode2_dtor,
            40,
            &|c, seed| {
                let mut r = Rng(0x0022_17c8 ^ (seed as u64) << 8);
                ov_pause_machine(c, &mut r, seed, 0x0022_17c8);
                c.g[A1][0] = 0;
            },
        );
    }
}
