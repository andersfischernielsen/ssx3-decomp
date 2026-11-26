//! Staging module oc862 (wave wave250): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc862`.

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
    (
        0x001a_3e30,
        cfe_state_net_messenger_v32,
        "cFEStateNetMessenger_v32",
    ),
    (
        0x001a_4040,
        cfe_state_net_messenger_v25,
        "cFEStateNetMessenger_v25",
    ),
    (
        0x001a_41c0,
        cfe_state_net_messenger_on_widget_created,
        "cFEStateNetMessenger_OnWidgetCreated",
    ),
    (
        0x001a_4300,
        cfe_state_net_messenger_on_widget_notify,
        "cFEStateNetMessenger_OnWidgetNotify",
    ),
    (
        0x001a_4810,
        cfe_state_net_messenger_find_user,
        "cFEStateNetMessenger_FindUser",
    ),
];

/// `lw off(gp)` (sign-extended).
fn oc862_gp_lw(c: &mut Cpu, off: u32) -> u64 {
    let a = r32(c, GP).wrapping_add(off);
    c.lw(a)
}

/// `a0 = 0x460000 + off; jal 0x317670` (text id of a string in .rodata,
/// returning to `ret`).
fn oc862_text_id(c: &mut Cpu, off: u32, ret: u32) -> crate::call::Flow {
    c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(off));
    call_ptr(c, 0x0031_7670, ret)
}

/// `cFEStateNetMessenger_v32(self)` (0x1a3e30): make sure the buddy manager
/// (`gp+0x238`) is up (`0x260f80` when it is not yet set, then `0x261460`
/// and, if that fails, `0x2613c8`), then call `0x262768(mgr, 1, 10, 0, 0, 0)`.
/// 4 (+2) + 4 (+3) + 2 (+3) + 9 cycles plus the callees.
pub fn cfe_state_net_messenger_v32(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = oc862_gp_lw(c, 0x238);
        frame.save_ra(c, 0);
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            call_ptr(c, 0x0026_0f80, 0x001a_3e48)?;
        }
        c.cycles += 4;
        c.g[A0][0] = oc862_gp_lw(c, 0x238);
        call_ptr(c, 0x0026_1460, 0x001a_3e50)?;
        let ok = c.g[V0][0] != 0;
        c.g[V0][0] = oc862_gp_lw(c, 0x238);
        if !ok {
            c.cycles += 3;
            c.g[A0][0] = oc862_gp_lw(c, 0x238);
            call_ptr(c, 0x0026_13c8, 0x001a_3e60)?;
            c.g[V0][0] = oc862_gp_lw(c, 0x238);
        }
        c.cycles += 2;
        let have = c.g[V0][0] != 0;
        c.g[A0][0] = oc862_gp_lw(c, 0x238);
        if !have {
            c.cycles += 3;
            c.g[A0][0] = oc862_gp_lw(c, 0u32.wrapping_sub(0xc8));
            call_ptr(c, 0x0025_b800, 0x001a_3e74)?;
            c.g[A0][0] = oc862_gp_lw(c, 0x238);
        }
        c.cycles += 9;
        c.g[A1][0] = 1;
        c.g[A2][0] = 0xa;
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        call_ptr(c, 0x0026_2768, 0x001a_3e90)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// One `widget(a0 = s0, a1, a2 = v0 [, a3])` call of
/// [`cfe_state_net_messenger_v25`]: `v0` is the text id just looked up.
fn oc862_v25_widget(
    c: &mut Cpu,
    a1: u32,
    a3: Option<u32>,
    target: u32,
    ret: u32,
) -> crate::call::Flow {
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = a1 as u64;
    if let Some(a3) = a3 {
        c.g[A3][0] = a3 as u64;
    }
    call_ptr(c, target, ret)
}

/// `cFEStateNetMessenger_v25(self, widget)` (0x1a4040): set up the messenger's
/// screen widget `a1`: type 5 (0x1da528), the id stored at +0x54, three text
/// rows (0x1da530, rows 0..2) and five option rows (0x1da510, rows 0..4)
/// with text ids looked up by hash. 76 cycles plus the callees.
pub fn cfe_state_net_messenger_v25(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 76;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 5;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001d_a528, 0x001a_405c)?;
        oc862_text_id(c, 0x1458, 0x001a_4068)?;
        let s0 = r32(c, S0);
        c.sw(s0.wrapping_add(0x54), c.g[V0][0]);
        oc862_text_id(c, 0x1470, 0x001a_4078)?;
        oc862_v25_widget(c, 0, None, 0x001d_a530, 0x001a_4088)?;
        oc862_text_id(c, 0x1488, 0x001a_4094)?;
        oc862_v25_widget(c, 1, None, 0x001d_a530, 0x001a_40a4)?;
        oc862_text_id(c, 0x14a0, 0x001a_40b0)?;
        oc862_v25_widget(c, 2, None, 0x001d_a530, 0x001a_40c0)?;
        oc862_text_id(c, 0u32.wrapping_sub(0x1a0), 0x001a_40cc)?;
        oc862_v25_widget(c, 0, Some(3), 0x001d_a510, 0x001a_40e0)?;
        oc862_text_id(c, 0x14b8, 0x001a_40ec)?;
        oc862_v25_widget(c, 1, Some(0), 0x001d_a510, 0x001a_4100)?;
        oc862_text_id(c, 0x14d0, 0x001a_410c)?;
        oc862_v25_widget(c, 2, Some(2), 0x001d_a510, 0x001a_4120)?;
        oc862_text_id(c, 0x14e8, 0x001a_412c)?;
        oc862_v25_widget(c, 3, Some(1), 0x001d_a510, 0x001a_4140)?;
        oc862_text_id(c, 0x1500, 0x001a_414c)?;
        oc862_v25_widget(c, 4, Some(6), 0x001d_a510, 0x001a_4160)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Call the widget's vtable slot 0x48 (`this + delta`, method) with `a1`.
fn oc862_created_vslot(c: &mut Cpu, a1: u32, ret: u32) -> crate::call::Flow {
    let s1 = r32(c, S1);
    c.g[V0][0] = c.lw(s1.wrapping_add(8));
    c.g[A1][0] = a1 as u64;
    let vt = r32(c, V0);
    c.g[A0][0] = c.lh(vt.wrapping_add(0x48));
    c.g[V1][0] = c.lw(vt.wrapping_add(0x4c));
    let target = r32(c, V1);
    c.g[A0][0] = sx(s1.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// `a1 = v0; a0 = s1; jal 0x3a1000` (set the widget text to the id in `v0`).
fn oc862_created_set_text(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x003a_1000, ret)
}

/// Store the new flags (`s1+0x14`) and restore `s0`.
fn oc862_created_store_flags(c: &mut Cpu, frame: Frame) {
    c.cycles += 2;
    let s1 = r32(c, S1);
    c.sw(s1.wrapping_add(0x14), c.g[V0][0]);
    frame.restore(c, S0, 0x20);
}

/// `cFEStateNetMessenger_OnWidgetCreated(self, widget)` (0x1a41c0): by the
/// widget's id (`+0x38`, compared with the text ids of five names) set its
/// text (ids 0/1: the text of the same name, 3a1000), show or hide it through
/// its vtable (slot 0x48) and set (0x80 / 0x1) or leave its flags (+0x14).
pub fn cfe_state_net_messenger_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        let s1 = r32(c, S1);
        c.g[S0][0] = c.lw(s1.wrapping_add(0x38));
        oc862_text_id(c, 0x1520, 0x001a_41e4)?;
        let mut text = false;
        if c.g[S0][0] == c.g[V0][0] {
            c.cycles += 3;
            c.g[A0][0] = sx(0x0046_1458);
            text = true;
        } else {
            c.cycles += 5;
            c.g[S0][0] = c.lw(s1.wrapping_add(0x38));
            oc862_text_id(c, 0x1530, 0x001a_4204)?;
            if c.g[S0][0] == c.g[V0][0] {
                c.cycles += 2;
                c.g[A0][0] = sx(0x0046_1540);
                text = true;
            }
        }
        if text {
            // 0x4214: look the text up, set it, show the widget.
            c.cycles += 13;
            call_ptr(c, 0x0031_7670, 0x001a_421c)?;
            oc862_created_set_text(c, 0x001a_4228)?;
            oc862_created_vslot(c, 1, 0x001a_4240)?;
            frame.restore(c, S0, 0x20);
        } else {
            // 0x4248.
            c.cycles += 5;
            c.g[S0][0] = c.lw(s1.wrapping_add(0x38));
            oc862_text_id(c, 0x1558, 0x001a_4254)?;
            if c.g[S0][0] == c.g[V0][0] {
                c.cycles += 9;
                oc862_created_vslot(c, 1, 0x001a_4274)?;
                c.g[V0][0] = c.lw(s1.wrapping_add(0x14));
                c.g[V0][0] |= 0x80;
                oc862_created_store_flags(c, frame);
            } else {
                c.cycles += 5;
                c.g[S0][0] = c.lw(s1.wrapping_add(0x38));
                oc862_text_id(c, 0x1570, 0x001a_428c)?;
                if c.g[S0][0] == c.g[V0][0] {
                    c.cycles += 14;
                    oc862_created_vslot(c, 0, 0x001a_42ac)?;
                    oc862_text_id(c, 0x1580, 0x001a_42b8)?;
                    oc862_created_set_text(c, 0x001a_42c4)?;
                    frame.restore(c, S0, 0x20);
                } else {
                    c.cycles += 5;
                    c.g[S0][0] = c.lw(s1.wrapping_add(0x38));
                    oc862_text_id(c, 0x1590, 0x001a_42d8)?;
                    let differ = c.g[S0][0] != c.g[V0][0];
                    frame.restore(c, S0, 0x20);
                    if !differ {
                        c.cycles += 2;
                        c.g[V0][0] = c.lw(s1.wrapping_add(0x14));
                        c.g[V0][0] |= 0x1;
                        oc862_created_store_flags(c, frame);
                    }
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateNetMessenger_OnWidgetNotify(self, widget, kind, value)`
/// (0x1a4300): kind 4 (selection changed) stores `value` at +0x6f4,
/// refreshes the player list and returns 0; any other kind returns 0x101.
pub fn cfe_state_net_messenger_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 4;
        let is_select = c.g[A2][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        if is_select {
            c.cycles += 3;
            let this = r32(c, A0);
            c.sw(this.wrapping_add(0x6f4), c.g[A3][0]);
            Callee {
                addr: 0x001a_6978,
                f: ee::s03::f_001a6978,
            }
            .call(c, 0x001a_4320)?;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0x101;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateNetMessenger_FindUser(self, user)` (0x1a4810): when the user
/// (0x416810) has fewer than 3 entries open the buddy popup with option text
/// (kind 3, id 0x806); otherwise show the other popup (id 0x807, clearing
/// +0x164), then pass the user to 0x3184e8(self+0x6d4) and 0x25e6d0(gp-0xc8).
pub fn cfe_state_net_messenger_find_user(c: &mut Cpu) {
    guest(c, |c| {
        let get_popup = Callee {
            addr: 0x001a_9c50,
            f: ee::s03::f_001a9c50,
        };
        let show_popup = Callee {
            addr: 0x001a_9bc0,
            f: ee::s03::f_001a9bc0,
        };
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A1][0];
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0041_6810, 0x001a_4838)?;
        let few = (c.g[V0][0] as i64) < 3;
        if few {
            c.cycles += 27;
            c.g[A0][0] = c.g[S3][0];
            c.g[S2][0] = 3;
            get_popup.call(c, 0x001a_4850)?;
            c.g[S0][0] = c.g[V0][0];
            let popup = r32(c, S0);
            c.g[V0][0] = 1;
            c.sw(popup.wrapping_add(0xbc), c.g[V0][0]);
            c.g[S1][0] = sx(popup.wrapping_add(0xbc));
            oc862_text_id(c, 0x15f8, 0x001a_486c)?;
            c.sw(popup.wrapping_add(0xc0), c.g[S2][0]);
            c.sw(popup.wrapping_add(0xc4), c.g[V0][0]);
            oc862_text_id(c, 0x1618, 0x001a_4880)?;
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0;
            Callee {
                addr: 0x001c_57c0,
                f: ee::s03::f_001c57c0,
            }
            .call(c, 0x001a_4890)?;
            c.g[V0][0] = 0x806;
            c.sw(popup.wrapping_add(0x14c), c.g[S2][0]);
            c.sw(popup.wrapping_add(0x150), c.g[V0][0]);
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S0][0];
            show_popup.call(c, 0x001a_48a8)?;
            frame.restore(c, S0, 0x40);
        } else {
            c.cycles += 24;
            c.g[A0][0] = c.g[S3][0];
            get_popup.call(c, 0x001a_48b8)?;
            c.g[S0][0] = c.g[V0][0];
            let popup = r32(c, S0);
            c.sw(popup.wrapping_add(0xbc), 0);
            oc862_text_id(c, 0x1628, 0x001a_48cc)?;
            c.g[A2][0] = 3;
            c.g[V1][0] = 0x807;
            c.sw(popup.wrapping_add(0x164), 0);
            c.g[A1][0] = c.g[S0][0];
            c.sw(popup.wrapping_add(0xc4), c.g[V0][0]);
            c.g[A0][0] = c.g[S3][0];
            c.sw(popup.wrapping_add(0x150), c.g[V1][0]);
            c.sw(popup.wrapping_add(0x14c), c.g[A2][0]);
            c.sw(popup.wrapping_add(0xc0), c.g[A2][0]);
            show_popup.call(c, 0x001a_48f4)?;
            c.g[A0][0] = sx(r32(c, S3).wrapping_add(0x6d4));
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0031_84e8, 0x001a_4900)?;
            c.g[A0][0] = oc862_gp_lw(c, 0u32.wrapping_sub(0xc8));
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0025_e6d0, 0x001a_490c)?;
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 6;
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
    use crate::difftest::check;
    use crate::difftest::fakes::{covered, fake_call, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{DATA, GAME_GP};

    /// Forced-result mailboxes of the fake callees, one word per target.
    const OC862_MAIL: u32 = DATA + 0x30_0000;

    /// The miss hook: the fake call, then the result forced for the target
    /// (1 = zero, n + 2 = n); the plain fake cannot return zero.
    fn oc862_miss(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(OC862_MAIL + (target & 0xf_fffc));
        fake_call(c, target);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = if mail == 1 { 0 } else { (mail - 2) as u64 };
        }
    }

    /// Make the fake for `target` return zero (`v == 0`) or `v`.
    fn oc862_force(c: &mut Cpu, target: u32, v: u32) {
        let w = if v == 0 { 1 } else { v + 2 };
        c.bus.write32(OC862_MAIL + (target & 0xf_fffc), w);
    }

    #[test]
    fn cfe_state_net_messenger_v32_matches() {
        covered(
            ssx3_ee::s03::f_001a3e30__generated,
            cfe_state_net_messenger_v32,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x1a3e30, 5);
                c.hooks.miss = oc862_miss;
                // The manager pointer is zero for half the seeds (the "not set
                // up" call and the final 0x25b800 call), the 0x261460 result is
                // zero for the other half (the 0x2613c8 call).
                c.bus.write32(
                    GAME_GP + 0x238,
                    if seed & 1 == 0 { 0 } else { DATA + 0x100 },
                );
                c.bus.write32(GAME_GP.wrapping_sub(0xc8), DATA + 0x200);
                oc862_force(c, 0x0026_1460, if seed & 2 == 0 { 0 } else { 5 });
            },
        );
    }

    #[test]
    fn cfe_state_net_messenger_v25_matches() {
        check(
            ssx3_ee::s03::f_001a4040__generated,
            cfe_state_net_messenger_v25,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x1a4040, 14);
                c.g[A1][0] = sx(DATA + 0x400);
            },
        );
    }

    #[test]
    fn cfe_state_net_messenger_on_widget_created_matches() {
        check(
            ssx3_ee::s03::f_001a41c0__generated,
            cfe_state_net_messenger_on_widget_created,
            72,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1a41c0, 7);
                let widget = DATA + 0x400;
                let vt = DATA + 0x800;
                const ID: u32 = 0x0012_3450;
                // Which of the five name lookups (calls 0..4) equals the widget's
                // id; 5 = none.
                let k = seed % 6;
                c.bus.write32(widget + 0x38, ID);
                c.bus.write32(widget + 8, vt);
                c.bus.write32(widget + 0x14, r.next_u32());
                c.bus
                    .write16(vt + 0x48, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(vt + 0x4c, FAKE);
                for i in 0..8u32 {
                    c.bus.write32(
                        LOG_RESULTS + 4 * i,
                        if i == k { ID } else { 0x1111_1100 + i },
                    );
                }
                c.g[A1][0] = sx(widget);
            },
        );
    }

    #[test]
    fn cfe_state_net_messenger_on_widget_notify_matches() {
        check(
            ssx3_ee::s03::f_001a4300__generated,
            cfe_state_net_messenger_on_widget_notify,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1a4300, 0);
                // +0x40 == 0: the real RefreshPlayerList returns at once.
                c.g[A0][0] = sx(DATA);
                c.g[A2][0] = if seed % 2 == 0 {
                    4
                } else {
                    3 + (seed as u64 % 5) * 7
                };
                c.g[A3][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn cfe_state_net_messenger_find_user_matches() {
        check(
            ssx3_ee::s03::f_001a4810__generated,
            cfe_state_net_messenger_find_user,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x1a4810, 40);
                let this = DATA;
                let popup = DATA + 0x2000;
                let screen = DATA + 0x3000;
                let (mid, leaf) = (DATA + 0x3400, DATA + 0x3800);
                // The popup exists (GetPopup only resets its config); the screen
                // chain of ShowPopup is valid and the popup's +0x2b8 is zero
                // (no rebuild).
                c.bus.write32(this + 0x6a0, popup);
                c.bus.write32(this + 0x40, screen);
                c.bus.write32(screen + 0xd0, mid);
                c.bus.write32(mid + 0x10, leaf);
                // The first call (0x416810) is below 3 for even seeds.
                c.bus.write32(
                    LOG_RESULTS,
                    if seed % 2 == 0 {
                        1 + (seed % 4) / 2
                    } else {
                        3 + seed % 5
                    },
                );
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x600);
            },
        );
    }
}
