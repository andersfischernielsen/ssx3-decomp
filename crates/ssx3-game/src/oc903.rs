//! Staging module oc903 (wave wave253): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc903`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001c_5058,
        c_fe_state_net_news_ctor2,
        "cFEStateNetNews_ctor_2",
    ),
    (
        0x001c_5178,
        c_fe_state_net_news_update,
        "cFEStateNetNews_Update",
    ),
    (
        0x001c_5198,
        c_fe_state_net_news_transition_in,
        "cFEStateNetNews_TransitionIn",
    ),
    (
        0x001c_5368,
        c_fe_state_net_news_on_widget_action,
        "cFEStateNetNews_OnWidgetAction",
    ),
    (
        0x001c_5630,
        c_fe_popup_confirm_cfg_reset,
        "cFEPopupConfirm_CfgReset",
    ),
];

/// `cFEStateNetNews` constructor variant (0x1c5058): run `NetFEState_ctor`
/// with two zero arguments, then install the news vtable at `+8`. Returns
/// `this`.
pub fn c_fe_state_net_news_ctor2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x001a_8500, 0x001c_5074)?;
        c.g[V1][0] = sx(0x47_0000u32.wrapping_sub(0x7e98));
        c.g[V0][0] = c.g[S0][0];
        let this = r32(c, S0);
        let vtable = c.g[V1][0];
        Addr(this.wrapping_add(8)).sw(c, vtable);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateNetNews_Update` (0x1c5178): forward the frame update to
/// `NetFEState_Update`.
pub fn c_fe_state_net_news_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x001a_8770, 0x001c_5188)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cUIScreen_getObjectByHashName(self->mpScreen, GetHashValue32(name))`
/// (0x1c5198's lookups): `name` is the string's address, the two return
/// addresses are the guest call sites of the hash and the lookup.
pub(crate) fn oc903_news_find(c: &mut Cpu, name: u32, hash_ret: u32, find_ret: u32) -> Flow {
    c.g[A0][0] = sx(name);
    call_ptr(c, 0x0031_7670, hash_ret)?;
    c.g[A0][0] = Addr(r32(c, S3).wrapping_add(0x40)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, find_ret)
}

/// `if (strlen(src) > 24) dst[0] = 0; else strcpy(dst, src);` for the arrow
/// animation names. `reg` keeps the source address (the generated S4 or S0).
pub(crate) fn oc903_news_arrow(
    c: &mut Cpu,
    reg: usize,
    src: u32,
    dst: u32,
    len_ret: u32,
    copy_ret: u32,
) -> Flow {
    c.g[V0][0] = 0x46_0000;
    c.g[reg][0] = sx(src);
    c.g[A0][0] = c.g[reg][0];
    call_ptr(c, 0x0041_6810, len_ret)?;
    let short = c.g[V0][0] < 0x19;
    c.g[V0][0] = short as u64;
    c.g[A1][0] = c.g[reg][0];
    c.cycles += 2;
    if short {
        c.g[A0][0] = sx(dst);
        call_ptr(c, 0x0041_66f4, copy_ret)
    } else {
        Addr(dst).sb(c, 0);
        Ok(())
    }
}

/// The `on` branch of the news gain transition (0x1c5198): look up the
/// scroll bar and its arrows, text scroller and hide-on-empty widget, wire
/// them together, copy the arrow animation names, focus the scroller and
/// show the scroll bar.
pub(crate) fn oc903_news_on_gain(c: &mut Cpu) -> Flow {
    c.cycles += 37;
    oc903_news_find(c, 0x0046_5f28, 0x001c_51cc, 0x001c_51d8)?;
    c.g[S2][0] = c.g[V0][0];
    oc903_news_find(c, 0x0046_19c8, 0x001c_51e8, 0x001c_51f4)?;
    c.g[S1][0] = c.g[V0][0];
    oc903_news_find(c, 0x0046_19d8, 0x001c_5204, 0x001c_5210)?;
    c.g[S0][0] = c.g[V0][0];
    oc903_news_find(c, 0x0046_19e8, 0x001c_5220, 0x001c_522c)?;
    c.g[S5][0] = c.g[V0][0];
    let s2 = r32(c, S2);
    let up = c.g[S1][0];
    Addr(s2.wrapping_add(0x7c)).sw(c, up);
    let down = c.g[S0][0];
    Addr(s2.wrapping_add(0x80)).sw(c, down);
    let s2 = r32(c, S2);
    oc903_news_arrow(
        c,
        S4,
        0x0046_5720,
        s2.wrapping_add(0x8c),
        0x001c_5248,
        0x001c_5264,
    )?;
    c.cycles += 7;
    let s2 = r32(c, S2);
    oc903_news_arrow(
        c,
        S0,
        0x0046_5730,
        s2.wrapping_add(0xa5),
        0x001c_5274,
        0x001c_5290,
    )?;
    c.cycles += 16;
    let s2 = r32(c, S2);
    let hide = c.g[S5][0];
    Addr(s2.wrapping_add(0x84)).sw(c, hide);
    oc903_news_find(c, 0x0046_5f38, 0x001c_52a0, 0x001c_52ac)?;
    let x = r32(c, V0);
    let scroll = c.g[S2][0];
    Addr(x.wrapping_add(0xb8)).sw(c, scroll);
    // TextScroll::setFocus(1) through the vtable: this = x + delta, A2 = fn.
    c.g[A1][0] = 1;
    c.g[V1][0] = Addr(x.wrapping_add(8)).lw(c);
    let v1 = c.g[V1][0] as u32;
    c.g[A0][0] = Addr(v1.wrapping_add(0x38)).lh(c);
    c.g[A2][0] = Addr(v1.wrapping_add(0x3c)).lw(c);
    let focus = c.g[A2][0] as u32;
    c.g[A0][0] = sx(x.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, focus, 0x001c_52c8)?;
    c.g[A1][0] = 1;
    if c.g[S2][0] == 0 {
        return Ok(());
    }
    // ScrollBar::setVisible(1) through its own vtable.
    c.cycles += 5;
    let s2 = r32(c, S2);
    let bar_vt = Addr(s2.wrapping_add(8)).lw(c);
    c.g[V0][0] = bar_vt;
    let vt = r32(c, V0);
    c.g[A0][0] = Addr(vt.wrapping_add(0x48)).lh(c);
    c.g[V1][0] = Addr(vt.wrapping_add(0x4c)).lw(c);
    let visible = c.g[V1][0] as u32;
    c.g[A0][0] = sx(s2.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, visible, 0x001c_52e4)
}

/// `cFEStateNetNews_TransitionIn` (0x1c5198), the `cFEStateNetNews_onGainTransition`
/// of the reference: `on` wires the news widgets (see `oc903_news_on_gain`).
/// Returns 0.
pub fn c_fe_state_net_news_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S3, 0x30);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        if c.g[A1][0] != 0 {
            oc903_news_on_gain(c)?;
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x60);
        c.g[V0][0] = 0;
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

/// The A2 == 6 branch of `cFEStateNetNews_OnWidgetAction`: allocate a
/// `cFEStateNetNews_SSXWorld` (1744 bytes), construct it with the message
/// block at `+0x10`, and make it the next state.
pub(crate) fn oc903_news_six(c: &mut Cpu) -> Flow {
    c.cycles += 14;
    c.g[A1][0] = sx(0x0046_11a8);
    c.g[A0][0] = 0x6d0;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7d70, 0x001c_53e4)?;
    let s0 = r32(c, S0);
    c.g[A1][0] = Addr(s0.wrapping_add(0x10)).lw(c);
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x001c_1c50, 0x001c_53f0)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x001a_8a40, 0x001c_53fc)
}

/// The dispatch of `cFEStateNetNews_OnWidgetAction` for a message (A1 != 0)
/// on action `A2`. Returns true when the call goes on to the base
/// `NetFEState_OnWidgetAction`, false when the function is done.
pub(crate) fn oc903_news_action(c: &mut Cpu) -> Result<bool, Unwound> {
    c.cycles += 3;
    c.g[V0][0] = 6;
    let is_six = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = (c.g[A2][0] < 7) as u64;
    if is_six {
        oc903_news_six(c)?;
        return Ok(false);
    }
    c.cycles += 2;
    let at_least_seven = c.g[V0][0] == 0;
    c.g[V0][0] = 5;
    if !at_least_seven {
        c.cycles += 2;
        if c.g[A2][0] == c.g[V0][0] {
            return Ok(false);
        }
        c.cycles += 2;
        return Ok(true);
    }
    c.cycles += 3;
    c.g[V0][0] = 8;
    if c.g[A2][0] != c.g[V0][0] {
        return Ok(true);
    }
    c.cycles += 4;
    let s0 = r32(c, S0);
    c.g[V0][0] = Addr(s0.wrapping_add(0x1c)).lw(c) & 6;
    if c.g[V0][0] != 0 {
        return Ok(false);
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x001a_9b58, 0x001c_53c4)?;
    Ok(false)
}

/// `cFEStateNetNews_OnWidgetAction` (0x1c5368), the `onWidgetEvent` of the
/// reference: a message (A1 != 0) with action A2 either opens the messenger
/// or starts the SSX world state, and anything else goes on to the base
/// `NetFEState_OnWidgetAction`. No return value.
pub fn c_fe_state_net_news_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let has_msg = c.g[A1][0] != 0;
        c.g[S0][0] = c.g[A0][0];
        let tail = if !has_msg {
            true
        } else if oc903_news_action(c)? {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x001a_8918, 0x001c_540c)?;
            true
        } else {
            false
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

/// `cFEPopupConfirm_CfgReset` (0x1c5630). The reference calls it
/// `cPopupInfo_clear`: it clears the popup's four message strings (8 bytes
/// each), five option/confirm rows (8 and 12 bytes) and the scalar fields to
/// their defaults (`-1` default option, centred text, deferred events off).
pub fn c_fe_popup_confirm_cfg_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x40);
        c.g[A2][0] = 8;
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        let s0 = r32(c, S0);
        c.g[A0][0] = sx(s0.wrapping_add(4));
        frame.save(c, S3, 0x10);
        c.g[S2][0] = sx(s0.wrapping_add(0x54));
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(s0.wrapping_add(0x24));
        call_ptr(c, 0x0041_6210, 0x001c_5664)?;
        c.g[S3][0] = 4;
        let s0 = r32(c, S0);
        c.g[A0][0] = sx(s0.wrapping_add(0xc));
        c.g[A1][0] = 0;
        c.g[A2][0] = 8;
        call_ptr(c, 0x0041_6210, 0x001c_5678)?;
        let s0 = r32(c, S0);
        c.g[A0][0] = sx(s0.wrapping_add(0x14));
        c.g[A1][0] = 0;
        c.g[A2][0] = 8;
        call_ptr(c, 0x0041_6210, 0x001c_5688)?;
        let s0 = r32(c, S0);
        c.g[A0][0] = sx(s0.wrapping_add(0x1c));
        c.g[A1][0] = 0;
        c.g[A2][0] = 8;
        call_ptr(c, 0x0041_6210, 0x001c_5698)?;
        c.g[A0][0] = c.g[S1][0];
        loop {
            c.cycles += 12;
            c.g[A1][0] = 0;
            c.g[A2][0] = 8;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(8));
            call_ptr(c, 0x0041_6210, 0x001c_56b0)?;
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(1));
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0xc;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
            call_ptr(c, 0x0041_6210, 0x001c_56c8)?;
            let more = (c.g[S3][0] as i64) >= 0;
            c.g[A0][0] = c.g[S1][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001c_56a0);
            }
        }
        c.cycles += 32;
        let s0 = r32(c, S0);
        c.g[A0][0] = sx(s0.wrapping_add(0x4c));
        c.g[A1][0] = 0;
        c.g[A2][0] = 8;
        call_ptr(c, 0x0041_6210, 0x001c_56e0)?;
        let s0 = r32(c, S0);
        c.g[V1][0] = 1;
        Addr(s0.wrapping_add(0x98)).sw(c, 0);
        let one = c.g[V1][0];
        Addr(s0.wrapping_add(0xbc)).sw(c, one);
        c.g[V0][0] = 0x50_0000;
        Addr(s0.wrapping_add(0xa8)).sw(c, one);
        c.g[A0][0] = u64::MAX;
        Addr(s0.wrapping_add(0x94)).sw(c, 0);
        Addr(s0.wrapping_add(0xb8)).sw(c, 0);
        Addr(s0.wrapping_add(0xb0)).sw(c, 0);
        Addr(s0).sw(c, 0);
        Addr(s0.wrapping_add(0x90)).sw(c, 0);
        // Unaligned copy of the 8-byte default block at 0x4ff0d8 into the
        // popup (ldl/ldr into v1, sdl/sdr back at +0x9c..+0xa3).
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xf28));
        let a3 = c.g[A3][0] as u32;
        c.ldl(3, a3.wrapping_add(0x7));
        c.ldr(3, a3);
        c.g[A1][0] = Addr(a3.wrapping_add(8)).lw(c);
        c.sdl(3, s0.wrapping_add(0xa3));
        c.sdr(3, s0.wrapping_add(0x9c));
        let a1 = c.g[A1][0];
        Addr(s0.wrapping_add(0xa4)).sw(c, a1);
        let a0 = c.g[A0][0];
        Addr(s0.wrapping_add(0xac)).sw(c, a0);
        Addr(s0.wrapping_add(0xb4)).sw(c, 0);
        Addr(s0.wrapping_add(0xc0)).sw(c, 0);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Test-only fixture addresses (inside `DATA..DATA+0x400000`, clear of the
    /// random `ptr()` vtables and of the fakes' call log at `0x900000`).
    pub(crate) const OC903_GATE: u32 = DATA + 0x30_0000;
    pub(crate) const OC903_MAGIC: u32 = 0x0903_0903;
    pub(crate) const OC903_THIS: u32 = DATA + 0x20_0000;
    pub(crate) const OC903_OBJ: u32 = DATA + 0x31_0000;
    pub(crate) const OC903_SELF: u32 = DATA + 0x4000;

    /// The base-state globals the constructors read (the BELibrary dummy at
    /// `gp-0x1f04` and the world at `gp-0x848`, as in `camera::trigger`).
    pub(crate) fn oc903_world(c: &mut Cpu) {
        let be = DATA + 0x3_0000;
        let world = DATA + 0x8_0000;
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), be);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
        let game = world + 0x1000;
        c.bus.write32(world + 0x84, game);
        let race = world + 0x2000;
        c.bus.write32(game + 0xc, race);
        c.bus.write32(race + 0x7c, 0);
    }

    /// `cUIScreen_getObjectByHashName`: while the gate is set, return the
    /// next object of the fixture's list; otherwise the generated body.
    pub(crate) fn oc903_find(c: &mut Cpu) {
        if c.bus.read32(OC903_GATE) != OC903_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc903_find),
                ee::s13::f_0039d860__generated,
            );
        }
        let n = c.bus.read32(OC903_GATE + 4);
        c.bus.write32(OC903_GATE + 4, n + 1);
        c.g[V0][0] = c.bus.read32(OC903_GATE + 0x10 + 4 * n) as u64;
        c.pc = c.g[RA][0] as u32;
    }

    /// A gated stub: with the gate set, return at once to `ra` (the callee
    /// is not under test); otherwise run the body the slot held.
    pub(crate) fn oc903_gated(c: &mut Cpu, fake: fn(&mut Cpu), generated: fn(&mut Cpu)) {
        if c.bus.read32(OC903_GATE) != OC903_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(fake), generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    pub(crate) fn oc903_stub_widget(c: &mut Cpu) {
        oc903_gated(c, oc903_stub_widget, ee::s03::f_001a8918__generated)
    }

    pub(crate) fn oc903_stub_messenger(c: &mut Cpu) {
        oc903_gated(c, oc903_stub_messenger, ee::s03::f_001a9b58__generated)
    }

    pub(crate) fn oc903_stub_alloc(c: &mut Cpu) {
        oc903_gated(c, oc903_stub_alloc, ee::s10::f_00317d70__generated)
    }

    pub(crate) fn oc903_stub_ctor(c: &mut Cpu) {
        oc903_gated(c, oc903_stub_ctor, ee::s03::f_001c1c50__generated)
    }

    /// Install the gated fakes once per process (`fakes::hook` dedups).
    pub(crate) fn oc903_hooks() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039D860),
            oc903_find,
            fakes::prev_cell(oc903_find),
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A8918),
            oc903_stub_widget,
            fakes::prev_cell(oc903_stub_widget),
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A9B58),
            oc903_stub_messenger,
            fakes::prev_cell(oc903_stub_messenger),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc903_stub_alloc,
            fakes::prev_cell(oc903_stub_alloc),
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C1C50),
            oc903_stub_ctor,
            fakes::prev_cell(oc903_stub_ctor),
        );
    }

    /// `len` letters and a terminating zero at `at` (the arrow names).
    pub(crate) fn oc903_string(c: &mut Cpu, at: u32, len: u32) {
        for i in 0..len {
            c.bus.write8(at + i, b'a');
        }
        c.bus.write8(at + len, 0);
    }

    /// Fixture for the transition: five widgets behind one vtable (slots
    /// `+0x38` setFocus and `+0x48` setVisible, fake targets), the screen
    /// pointer, the arrow names (long or short by seed) and the `on` flag.
    pub(crate) fn oc903_transition_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x5198, 0);
        oc903_hooks();
        let vt = fakes::vtable(c, &mut r, &[0x38, 0x48]);
        c.bus.write32(OC903_GATE, OC903_MAGIC);
        c.bus.write32(OC903_GATE + 4, 0);
        for i in 0..5u32 {
            let obj = OC903_OBJ + 0x100 * i;
            c.bus.write32(obj + 8, vt);
            c.bus.write32(OC903_GATE + 0x10 + 4 * i, obj);
        }
        c.bus.write32(OC903_THIS + 0x40, DATA + 0x2_0000);
        c.g[A0][0] = sx(OC903_THIS);
        c.g[A1][0] = (seed % 5 != 0) as u64;
        oc903_string(c, 0x0046_5720, if seed % 2 == 0 { 30 } else { 7 });
        oc903_string(c, 0x0046_5730, if seed % 3 == 0 { 26 } else { 4 });
    }

    /// Fixture for the widget action: the message flag, the action (all of
    /// 0..9 by seed) and the messenger flags at `+0x1c`.
    pub(crate) fn oc903_widget_setup(c: &mut Cpu, seed: u32) {
        let _r = machine(c, seed, 0x5368, 0);
        oc903_hooks();
        c.bus.write32(OC903_GATE, OC903_MAGIC);
        c.bus
            .write32(OC903_SELF + 0x1c, if seed % 2 == 0 { 0 } else { 4 });
        c.g[A0][0] = sx(OC903_SELF);
        c.g[A1][0] = (seed % 7 != 0) as u64;
        c.g[A2][0] = (seed % 10) as u64;
    }

    #[test]
    fn c_fe_state_net_news_ctor2_matches() {
        check(
            ee::s03::f_001c5058__generated,
            c_fe_state_net_news_ctor2,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x5058, 1);
                oc903_world(c);
                c.g[A0][0] = sx(OC903_SELF);
            },
        );
    }

    #[test]
    fn c_fe_state_net_news_update_matches() {
        check(
            ee::s03::f_001c5178__generated,
            c_fe_state_net_news_update,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x5178, 0);
                oc903_world(c);
                c.g[A0][0] = sx(OC903_SELF);
            },
        );
    }

    #[test]
    fn c_fe_state_net_news_transition_in_matches() {
        check(
            ee::s03::f_001c5198__generated,
            c_fe_state_net_news_transition_in,
            120,
            &|c, seed| oc903_transition_setup(c, seed),
        );
    }

    #[test]
    fn c_fe_state_net_news_on_widget_action_matches() {
        check(
            ee::s03::f_001c5368__generated,
            c_fe_state_net_news_on_widget_action,
            120,
            &|c, seed| oc903_widget_setup(c, seed),
        );
    }

    #[test]
    fn c_fe_popup_confirm_cfg_reset_matches() {
        check(
            ee::s03::f_001c5630__generated,
            c_fe_popup_confirm_cfg_reset,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x5630, 0);
                c.g[A0][0] = sx(OC903_SELF + 0x200 * (seed % 4));
            },
        );
    }
}
