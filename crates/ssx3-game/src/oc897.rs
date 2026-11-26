//! Staging module oc897 (wave wave253): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc897`.

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
        0x001c_0460,
        cfe_online_agreement_ctor_3,
        "cFEStateOnlineAgreement_ctor_3",
    ),
    (
        0x001c_0b20,
        cfe_online_agreement_transition_in,
        "cFEStateOnlineAgreement_TransitionIn",
    ),
    (
        0x001c_0e88,
        cfe_online_agreement_v31,
        "cFEStateOnlineAgreement_v31",
    ),
    (
        0x001c_1488,
        cfe_state_net_quickplay_popup_v25,
        "cFEStateNetQuickPlayPopup_v25",
    ),
];

/// `cFEStateOnlineAgreement` constructor without a hash name (0x1c0460): runs
/// the net-state base constructor `NetFEState_ctor(self, 0, 0)` (0x1a8500),
/// then stores the class vtable word 0x46ac38 at `+0x8` and clears the two
/// pointers at `+0x6d0` and `+0x6d8`.
pub fn cfe_online_agreement_ctor_3(c: &mut Cpu) {
    guest(c, ctor_3_body)
}

fn ctor_3_body(c: &mut Cpu) -> Flow {
    ssx3_ee_rt::prof::enter(c, 0x001c_0460);
    c.cycles += 17;
    let frame = Frame::open(c, 0x20);
    c.g[A1][0] = 0;
    frame.save(c, S0, 0x10);
    c.g[A2][0] = 0;
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    Callee {
        addr: 0x001a_8500,
        f: ee::s03::f_001a8500,
    }
    .call(c, 0x001c_047c)?;
    c.g[V0][0] = c.g[S0][0];
    c.g[V1][0] = sx(0x0047_0000u32.wrapping_sub(0x53c8));
    let this = r32(c, S0);
    c.sw(this.wrapping_add(0x6d0), 0);
    c.sw(this.wrapping_add(0x8), c.g[V1][0]);
    c.sw(this.wrapping_add(0x6d8), 0);
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `cFEStateOnlineAgreement_v31(self)` (0x1c0e88): forwards to
/// `NetFEState_HandleNetEvent(self)` (0x1a8e40) with the same argument.
pub fn cfe_online_agreement_v31(c: &mut Cpu) {
    guest(c, v31_body)
}

fn v31_body(c: &mut Cpu) -> Flow {
    ssx3_ee_rt::prof::enter(c, 0x001c_0e88);
    c.cycles += 7;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    Callee {
        addr: 0x001a_8e40,
        f: ee::s03::f_001a8e40,
    }
    .call(c, 0x001c_0e98)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `cFEStateNetQuickPlayPopup_v25(self, ...)` (0x1c1488): one call on the
/// popup (`0x1da528(self, 4)`), the popup's text at `+0x54` from string
/// 0x465900, then four string-table fills (`0x1da510`/`0x1da530`) with the
/// lines 0x465918, 0x45fe60, 0x45fe40, 0x462c50 and 0x4614e8. Each text
/// lookup is the `0x317670` string call; the arguments a1/a2/a3 of each fill
/// are the registers the generated body leaves.
pub fn cfe_state_net_quickplay_popup_v25(c: &mut Cpu) {
    guest(c, quickplay_v25_body)
}

fn quickplay_v25_body(c: &mut Cpu) -> Flow {
    ssx3_ee_rt::prof::enter(c, 0x001c_1488);
    c.cycles += 54;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    c.g[S0][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[A1][0] = 4;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x001d_a528, 0x001c_14a4)?;
    c.g[A0][0] = sx(0x0046_5900);
    call_ptr(c, 0x0031_7670, 0x001c_14b0)?;
    c.sw(r32(c, S0).wrapping_add(0x54), c.g[V0][0]);
    c.g[A0][0] = sx(0x0046_5918);
    call_ptr(c, 0x0031_7670, 0x001c_14c0)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x001d_a530, 0x001c_14d0)?;
    c.g[A0][0] = sx(0x0045_fe60);
    call_ptr(c, 0x0031_7670, 0x001c_14dc)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    c.g[A3][0] = 3;
    call_ptr(c, 0x001d_a510, 0x001c_14f0)?;
    c.g[A0][0] = sx(0x0045_fe40);
    call_ptr(c, 0x0031_7670, 0x001c_14fc)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = 1;
    c.g[A3][0] = 4;
    call_ptr(c, 0x001d_a510, 0x001c_1510)?;
    c.g[A0][0] = sx(0x0046_2c50);
    call_ptr(c, 0x0031_7670, 0x001c_151c)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = 2;
    c.g[A3][0] = 0;
    call_ptr(c, 0x001d_a510, 0x001c_1530)?;
    c.g[A0][0] = sx(0x0046_14e8);
    call_ptr(c, 0x0031_7670, 0x001c_153c)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = 3;
    c.g[A3][0] = 1;
    call_ptr(c, 0x001d_a510, 0x001c_1550)?;
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `cFEStateOnlineAgreement_TransitionIn(self)` (0x1c0b20): fetches the
/// transition's owner objects and text, sets the two state words at `+0x7c`
/// and `+0x80`, probes three text lines (`0x416810` length below 0x19 means
/// the flag at `+0x8c`/`+0xa5` is set through `0x4166f4`, else cleared), then
/// starts the screen's first animation (`0x3a1310`) and calls its
/// per-frame hook. Returns 1.
pub fn cfe_online_agreement_transition_in(c: &mut Cpu) {
    guest(c, transition_in_body)
}

/// One text probe of `TransitionIn`: `0x416810(text)` returns a length; below
/// 0x19 the flag at `self+flag` goes through `0x4166f4`, else the flag byte is
/// cleared. Both paths charge two cycles (the probe block charges its own).
fn transition_in_probe(c: &mut Cpu, text: u32, flag: u32, probe_ret: u32, flag_ret: u32) -> Flow {
    c.g[A0][0] = sx(text);
    call_ptr(c, 0x0041_6810, probe_ret)?;
    c.g[V0][0] = (c.g[V0][0] < 0x19) as u64;
    c.g[A1][0] = sx(text);
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[A0][0] = sx(r32(c, S2).wrapping_add(flag));
        call_ptr(c, 0x0041_66f4, flag_ret)?;
    } else {
        c.cycles += 2;
        c.sb(r32(c, S2).wrapping_add(flag), 0);
    }
    Ok(())
}

fn transition_in_body(c: &mut Cpu) -> Flow {
    ssx3_ee_rt::prof::enter(c, 0x001c_0b20);
    c.cycles += 51;
    let frame = Frame::open(c, 0x70);
    frame.save(c, S0, 0x60);
    frame.save(c, S1, 0x50);
    frame.save(c, S2, 0x40);
    frame.save(c, S3, 0x30);
    frame.save(c, S4, 0x20);
    c.g[S3][0] = c.g[A0][0];
    frame.save(c, S5, 0x10);
    frame.save_ra(c, 0);

    c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(0x8));
    c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x100));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x104));
    let target = r32(c, V1);
    c.g[A0][0] = sx(r32(c, S3).wrapping_add(r32(c, A0)));
    call_ptr(c, target, 0x001c_0b58)?;

    c.g[A0][0] = sx(0x0046_19b8);
    call_ptr(c, 0x0031_7670, 0x001c_0b64)?;
    c.g[A0][0] = c.lw(r32(c, S3).wrapping_add(0x40));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, 0x001c_0b70)?;
    c.g[S2][0] = c.g[V0][0];
    c.g[A0][0] = sx(0x0046_19c8);
    call_ptr(c, 0x0031_7670, 0x001c_0b80)?;
    c.g[A0][0] = c.lw(r32(c, S3).wrapping_add(0x40));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, 0x001c_0b8c)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[A0][0] = sx(0x0046_19d8);
    call_ptr(c, 0x0031_7670, 0x001c_0b9c)?;
    c.g[A0][0] = c.lw(r32(c, S3).wrapping_add(0x40));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, 0x001c_0ba8)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = sx(0x0046_19e8);
    call_ptr(c, 0x0031_7670, 0x001c_0bb8)?;
    c.g[A0][0] = c.lw(r32(c, S3).wrapping_add(0x40));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, 0x001c_0bc4)?;
    c.g[S5][0] = c.g[V0][0];
    c.sw(r32(c, S2).wrapping_add(0x7c), c.g[S1][0]);
    c.sw(r32(c, S2).wrapping_add(0x80), c.g[S0][0]);
    c.g[S4][0] = sx(0x0046_5720);
    transition_in_probe(c, 0x0046_5720, 0x8c, 0x001c_0be0, 0x001c_0bfc)?;

    // Second probe: S0 = the text at 0x465730, S1 = the table base 0x460000.
    c.cycles += 8;
    c.g[V0][0] = 0x0046_0000;
    c.g[S0][0] = sx(0x0046_5730);
    c.g[S1][0] = c.g[V0][0];
    transition_in_probe(c, 0x0046_5730, 0xa5, 0x001c_0c10, 0x001c_0c2c)?;

    // Third probe: the same text through S1.
    c.cycles += 6;
    c.g[S0][0] = sx(r32(c, S1).wrapping_add(0x5730));
    transition_in_probe(c, 0x0046_5730, 0xa5, 0x001c_0c38, 0x001c_0c54)?;

    c.cycles += 20;
    c.sw(r32(c, S2).wrapping_add(0x84), c.g[S5][0]);
    c.g[A0][0] = sx(0x0046_5710);
    call_ptr(c, 0x0031_7670, 0x001c_0c64)?;
    c.g[A0][0] = c.lw(r32(c, S3).wrapping_add(0x40));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, 0x001c_0c70)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = 1;
    c.sw(r32(c, S0).wrapping_add(0xb8), c.g[S2][0]);
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x003a_1310, 0x001c_0c84)?;
    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x8));
    c.g[A1][0] = 1;
    c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x38));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x3c));
    let hook = r32(c, V1);
    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
    call_ptr(c, hook, 0x001c_0c9c)?;
    let s2_is_zero = c.g[S2][0] == 0;
    c.g[A1][0] = 1;
    if !s2_is_zero {
        c.cycles += 5;
        c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(0x8));
        c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x48));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x4c));
        let hook2 = r32(c, V1);
        c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
        call_ptr(c, hook2, 0x001c_0cb8)?;
    }

    c.cycles += 10;
    frame.restore(c, S0, 0x60);
    c.g[V0][0] = 1;
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, DATA};

    #[test]
    fn cfe_online_agreement_ctor_3_matches() {
        check(
            ee::s03::f_001c0460__generated,
            cfe_online_agreement_ctor_3,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1c0460, 0);
                c.pc = 0x001c_0460;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = r.next_u32() as u64;
                for i in 0..0x80u32 {
                    c.bus.write32(DATA + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn cfe_online_agreement_v31_matches() {
        check(
            ee::s03::f_001c0e88__generated,
            cfe_online_agreement_v31,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1c0e88, 0);
                c.pc = 0x001c_0e88;
                c.g[A0][0] = sx(DATA);
                for i in 0..0x80u32 {
                    c.bus.write32(DATA + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn cfe_state_net_quickplay_popup_v25_matches() {
        check(
            ee::s03::f_001c1488__generated,
            cfe_state_net_quickplay_popup_v25,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1c1488, 0);
                c.pc = 0x001c_1488;
                c.g[A1][0] = sx(DATA);
                for i in 0..0x80u32 {
                    c.bus.write32(DATA + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn cfe_online_agreement_transition_in_matches() {
        check(
            ee::s03::f_001c0b20__generated,
            cfe_online_agreement_transition_in,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1c0b20, 0);
                c.pc = 0x001c_0b20;
                c.g[A0][0] = sx(DATA);
                // self+0x8 -> a view whose +0x100/+0x104 and +0x38/+0x3c hold
                // the hook offsets and targets; self+0x40 -> the owner object.
                let view = DATA + 0x1000;
                c.bus.write32(DATA + 0x8, view);
                c.bus.write16(view + 0x100, 0);
                c.bus.write32(view + 0x104, 0);
                c.bus.write16(view + 0x38, 0);
                c.bus.write32(view + 0x3c, 0);
                c.bus.write16(view + 0x48, 0);
                c.bus.write32(view + 0x4c, 0);
                c.bus.write32(DATA + 0x40, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000 + 0xb8, 0);
                c.bus.write32(DATA + 0x2000 + 0x8, view);
                c.bus.write32(DATA + 0x2000 + 0x10, r.next_u32());
                // The flag bytes the probes write: random, so both paths of
                // each probe show up across seeds.
                for i in 0..0x40u32 {
                    c.bus.write8(DATA + 0x2100 + i, r.next_u32() as u8);
                }
            },
        );
    }
}
