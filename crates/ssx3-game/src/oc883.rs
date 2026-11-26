//! Staging module oc883 (wave wave252): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc883`.

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
        0x001b_4198,
        cfe_state_net_config_set_state6,
        "cFEStateNetConfig_SetState6",
    ),
    (
        0x001b_5068,
        cfe_state_net_config_set_list_visible,
        "cFEStateNetConfig_SetListVisible",
    ),
];

/// `void cFEStateNetConfig_SetState6(self)` (0x1b4198): stores state 6 in the
/// config state word at `+0x708`.
pub fn cfe_state_net_config_set_state6(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x6;
    let state = c.g[V0][0];
    Addr((c.g[A0][0] as u32).wrapping_add(0x708)).sw(c, state);
    ret(c);
}

/// A widget field of `self` (`+off` from the callee-saved `s0`), read as the
/// generated `lw` does.
fn set_list_visible_field(c: &mut Cpu, off: u32) -> u64 {
    Addr((c.g[S0][0] as u32).wrapping_add(off)).lw(c)
}

/// One virtual call on a widget, in the order the generated body issues it:
/// `a2 = widget`, `a1 = arg`, `v0 = [a2+8]` (the vtable), `a0 = lh(v0+slot)`,
/// `v1 = lw(v0+slot+4)`, then `a0 = a2 + a0` and the call through `v1`.
fn set_list_visible_vcall(c: &mut Cpu, widget: u64, arg: u64, slot: u32, back: u32) -> Flow {
    c.g[A2][0] = widget;
    c.g[A1][0] = arg;
    c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x8)).lw(c);
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(slot)).lh(c);
    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(slot + 4)).lw(c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, back)
}

/// The `configCount <= 0` arm: empty and hide both list boxes, hide the
/// config rectangle, show the "no configuration" text in the static colour,
/// and select menu row 0.
fn set_list_visible_empty(c: &mut Cpu) -> Flow {
    c.cycles += 66;
    c.g[A0][0] = set_list_visible_field(c, 0x6e4);
    call_ptr(c, 0x0039_a670, 0x001b_5084)?;
    c.g[A0][0] = set_list_visible_field(c, 0x6e8);
    call_ptr(c, 0x0039_a670, 0x001b_508c)?;
    let w = set_list_visible_field(c, 0x6e8);
    set_list_visible_vcall(c, w, 1, 0x40, 0x001b_50a8)?;
    let w = set_list_visible_field(c, 0x6e4);
    set_list_visible_vcall(c, w, 1, 0x40, 0x001b_50c4)?;
    let w = set_list_visible_field(c, 0x6ec);
    set_list_visible_vcall(c, w, 1, 0x40, 0x001b_50e0)?;
    let w = set_list_visible_field(c, 0x6e8);
    set_list_visible_vcall(c, w, 0, 0x48, 0x001b_50fc)?;
    let w = set_list_visible_field(c, 0x6e4);
    set_list_visible_vcall(c, w, 0, 0x48, 0x001b_5118)?;
    let w = set_list_visible_field(c, 0x6f8);
    set_list_visible_vcall(c, w, 0, 0x48, 0x001b_5134)?;
    let w = set_list_visible_field(c, 0x6fc);
    set_list_visible_vcall(c, w, 1, 0x48, 0x001b_5150)?;
    let w = set_list_visible_field(c, 0x6fc);
    set_list_visible_vcall(c, w, 0x004c_66c8, 0x58, 0x001b_5170)?;
    c.g[A0][0] = set_list_visible_field(c, 0x6f4);
    c.g[A1][0] = 0;
    call_ptr(c, 0x0039_b960, 0x001b_517c)?;
    Ok(())
}

/// The `configCount > 0` arm: show the lists, the config rectangle and the
/// pair settings (not static), hide the "no configuration" text, and select
/// menu row 1.
fn set_list_visible_with_configs(c: &mut Cpu) -> Flow {
    c.cycles += 53;
    let w = set_list_visible_field(c, 0x6ec);
    set_list_visible_vcall(c, w, 0, 0x40, 0x001b_51a0)?;
    let w = set_list_visible_field(c, 0x6e8);
    set_list_visible_vcall(c, w, 0, 0x40, 0x001b_51bc)?;
    let w = set_list_visible_field(c, 0x6e4);
    set_list_visible_vcall(c, w, 0, 0x40, 0x001b_51d8)?;
    let w = set_list_visible_field(c, 0x6e8);
    set_list_visible_vcall(c, w, 1, 0x48, 0x001b_51f4)?;
    let w = set_list_visible_field(c, 0x6e4);
    set_list_visible_vcall(c, w, 1, 0x48, 0x001b_5210)?;
    let w = set_list_visible_field(c, 0x6f8);
    set_list_visible_vcall(c, w, 1, 0x48, 0x001b_522c)?;
    let w = set_list_visible_field(c, 0x6fc);
    set_list_visible_vcall(c, w, 0, 0x48, 0x001b_5248)?;
    c.g[A0][0] = set_list_visible_field(c, 0x6f4);
    c.g[A1][0] = 1;
    call_ptr(c, 0x0039_b960, 0x001b_5254)?;
    Ok(())
}

/// `void cFEStateNetConfig_SetListVisible(self, configCount)` (0x1b5068): the
/// config page's list boxes, rectangle and "no configuration" text follow
/// `configCount` (`<= 0`: empty and hidden, menu row 0; `> 0`: shown, menu row
/// 1). Widgets are read from `self`: `+0x6e4` LB adapters, `+0x6e8` LB
/// settings, `+0x6ec` pair settings, `+0x6f4` config menu, `+0x6f8` config
/// rectangle, `+0x6fc` "no configuration" text.
pub fn cfe_state_net_config_set_list_visible(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let configs = (c.g[A1][0] as i64) > 0;
        c.g[S0][0] = c.g[A0][0];
        if configs {
            set_list_visible_with_configs(c)?;
        } else {
            set_list_visible_empty(c)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::{fake_call, fall_back, hook, machine, FAKE};
    use crate::difftest::DATA;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word in the machine's RAM: the test fakes below answer only when
    /// it is set, so other tests running the same callees see the real body.
    pub(crate) const OC883_CTRL: u32 = DATA + 0x0f_0000;
    pub(crate) const OC883_MAGIC: u32 = 0x0883_0883;

    pub(crate) static OC883_PREV_CLEAR: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC883_PREV_SELECT: AtomicUsize = AtomicUsize::new(0);

    /// `UIListBox_Clear` (0x39a670): logged, not run, while the gate is on.
    pub(crate) fn oc883_fake_clear(c: &mut Cpu) {
        if c.bus.read32(OC883_CTRL) != OC883_MAGIC {
            return fall_back(c, &OC883_PREV_CLEAR, ee::s13::f_0039a670__generated);
        }
        fake_call(c, 0x0039_a670);
    }

    /// `UIMenu_SetSelection` (0x39b960): logged, not run, while the gate is on.
    pub(crate) fn oc883_fake_select(c: &mut Cpu) {
        if c.bus.read32(OC883_CTRL) != OC883_MAGIC {
            return fall_back(c, &OC883_PREV_SELECT, ee::s13::f_0039b960__generated);
        }
        fake_call(c, 0x0039_b960);
    }

    #[test]
    fn cfe_state_net_config_set_state6_matches() {
        check(
            ee::s03::f_001b4198__generated,
            cfe_state_net_config_set_state6,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1b41, 0);
                c.pc = 0x001b_4198;
                let this = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.bus.write32(this + 0x708, r.next_u32());
            },
        );
    }

    #[test]
    fn cfe_state_net_config_set_list_visible_matches() {
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039A670),
            oc883_fake_clear,
            &OC883_PREV_CLEAR,
        );
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039B960),
            oc883_fake_select,
            &OC883_PREV_SELECT,
        );
        check(
            ee::s03::f_001b5068__generated,
            cfe_state_net_config_set_list_visible,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1b50, 12);
                c.bus.write32(OC883_CTRL, OC883_MAGIC);
                c.pc = 0x001b_5068;
                let this = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = match seed % 3 {
                    0 => 0,
                    1 => sx(0xffff_ffff),
                    _ => 1 + (seed / 3 % 3) as u64,
                };
                // One vtable with the three slots the body calls (setStatic,
                // setVisible, setColour), each a random thunk delta and a
                // fake method.
                let vt = DATA + 0x3800;
                for (i, off) in [0x40u32, 0x48, 0x58].into_iter().enumerate() {
                    let delta = (r.next_u32() as i16) & !3;
                    c.bus.write16(vt + off, delta as u16);
                    c.bus.write32(vt + off + 4, FAKE + 0x10 * i as u32);
                }
                // The six widgets at self+0x6e4..+0x6fc, each with the vtable at +8.
                for (i, field) in [0x6e4u32, 0x6e8, 0x6ec, 0x6f4, 0x6f8, 0x6fc]
                    .into_iter()
                    .enumerate()
                {
                    let w = DATA + 0x3000 + 0x40 * i as u32;
                    c.bus.write32(this + field, w);
                    c.bus.write32(w + 8, vt);
                }
            },
        );
    }
}
