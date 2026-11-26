//! Staging module oc896 (wave wave253): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc896`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Flow, Frame, Target, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001b_f6c0,
        cfe_state_login_on_child_event,
        "cFEStateLogin_OnChildEvent",
    ),
    (0x001b_fae0, cfe_state_login_v25, "cFEStateLogin_v25"),
    (
        0x001b_fca0,
        cfe_state_online_main_menu_update,
        "cFEStateOnlineMainMenu_Update",
    ),
    (
        0x001b_fcc0,
        cfe_state_online_main_menu_on_widget_created,
        "cFEStateOnlineMainMenu_OnWidgetCreated",
    ),
    (
        0x001b_fd98,
        cfe_state_online_main_menu_v25,
        "cFEStateOnlineMainMenu_v25",
    ),
    (
        0x001c_03d0,
        cfe_state_online_agreement_ctor_2,
        "cFEStateOnlineAgreement_ctor_2",
    ),
];

// ---------------------------------------------------------------------------
// cFEStateLogin: popup event routing.

/// `cFEStateLogin::OnChildEvent(this, popup)` (0x1bf6c0): routes a popup's
/// event by its kind (`popup+0x18`): 0x82c to the account-memory popup
/// (0x1bf438), 0x848 to the disconnect popup (0x1bf5a8), 0x80e to the popup's
/// own virtual handler (vtable at `this+8`, slot 0x108 with its `this`
/// delta), and anything else to `NetFEState::OnChildEvent` (0x1a97b8).
pub fn cfe_state_login_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = 0x82c;
        c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x18)).lw(c);
        c.g[A3][0] = c.g[A0][0];
        let this = c.g[A3][0];
        let kind = c.g[V1][0];
        let routed = if kind == 0x82c {
            cfe_login_child_account_popup(c, this)
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x80e;
            if kind >= 0x82d {
                c.cycles += 3;
                c.g[V0][0] = 0x848;
                if kind == 0x848 {
                    cfe_login_child_disconnect_popup(c, this)
                } else {
                    c.cycles += 2;
                    cfe_login_child_net(c, this)
                }
            } else {
                c.cycles += 2;
                if kind == 0x80e {
                    cfe_login_child_virtual(c, this)
                } else {
                    c.cycles += 2;
                    cfe_login_child_net(c, this)
                }
            }
        };
        routed?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// Kind 0x82c: `cFEStateLogin_OnAccountMemPopupEvent(this)` (0x1bf438).
fn cfe_login_child_account_popup(c: &mut Cpu, this: u64) -> Flow {
    c.cycles += 4;
    c.g[A0][0] = this;
    call_ptr(c, 0x001b_f438, 0x001b_f728)
}

/// Kind 0x848: `cFEStateLogin_OnDisconnectPopupEvent(this)` (0x1bf5a8).
fn cfe_login_child_disconnect_popup(c: &mut Cpu, this: u64) -> Flow {
    c.cycles += 4;
    c.g[A0][0] = this;
    call_ptr(c, 0x001b_f5a8, 0x001b_f738)
}

/// Default kinds: `NetFEState_OnChildEvent(this)` (0x1a97b8).
fn cfe_login_child_net(c: &mut Cpu, this: u64) -> Flow {
    c.cycles += 3;
    c.g[A0][0] = this;
    call_ptr(c, 0x001a_97b8, 0x001b_f748)
}

/// Kind 0x80e: the popup's own handler, through the vtable at `this+8`
/// (`this` adjusted by the slot's delta, slot function at +0x10c).
fn cfe_login_child_virtual(c: &mut Cpu, this: u64) -> Flow {
    c.g[V0][0] = Addr((this as u32).wrapping_add(0x8)).lw(c);
    c.cycles += 6;
    let vt = c.g[V0][0] as u32;
    c.g[A0][0] = Addr(vt.wrapping_add(0x108)).lh(c);
    c.g[V1][0] = Addr(vt.wrapping_add(0x10c)).lw(c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((this as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x001b_f718)
}

// ---------------------------------------------------------------------------
// cFEStateLogin / cFEStateOnlineMainMenu: help popup setup (`v25`).

/// `GetHashValue32(text)` (0x317670) of the string at `text`; the hash is
/// left in v0.
fn cfe_help_text_hash(c: &mut Cpu, text: u32, ret: u32) -> Flow {
    c.g[A0][0] = sx(text);
    call_ptr(c, 0x0031_7670, ret)
}

/// `popup->setDescription(0, GetHashValue32(text))` (0x1da530), the popup
/// being `s0`.
fn cfe_help_set_description(c: &mut Cpu, text: u32, hash_ret: u32, set_ret: u32) -> Flow {
    cfe_help_text_hash(c, text, hash_ret)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x001d_a530, set_ret)
}

/// `popup->setOption(index, GetHashValue32(text), value)` (0x1da510), the
/// popup being `s0`.
fn cfe_help_set_option(
    c: &mut Cpu,
    index: u64,
    text: u32,
    value: u64,
    hash_ret: u32,
    set_ret: u32,
) -> Flow {
    cfe_help_text_hash(c, text, hash_ret)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = index;
    c.g[A3][0] = value;
    call_ptr(c, 0x001d_a510, set_ret)
}

/// `cFEStateLogin_v25(popup)` (0x1bfae0): fills the login help popup (select
/// login type): option count 4, screen name, description, four options.
pub fn cfe_state_login_v25(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 54;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 4;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001d_a528, 0x001b_fafc)?; // setNumHelpOptions(popup, 4)
        cfe_help_text_hash(c, 0x0046_53a8, 0x001b_fb08)?;
        let name = c.g[V0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x54), name); // muiScreenName
        cfe_help_set_description(c, 0x0046_53c8, 0x001b_fb18, 0x001b_fb28)?;
        cfe_help_set_option(c, 0, 0x0045_fe60, 3, 0x001b_fb34, 0x001b_fb48)?;
        cfe_help_set_option(c, 1, 0x0046_2c50, 0, 0x001b_fb54, 0x001b_fb68)?;
        cfe_help_set_option(c, 2, 0x0046_14e8, 1, 0x001b_fb74, 0x001b_fb88)?;
        cfe_help_set_option(c, 3, 0x0046_4910, 2, 0x001b_fb94, 0x001b_fba8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateOnlineMainMenu_v25(popup)` (0x1bfd98): the online main menu's
/// help popup: option count 4, screen name, description, four options.
pub fn cfe_state_online_main_menu_v25(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 54;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 4;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001d_a528, 0x001b_fdb4)?; // setNumHelpOptions(popup, 4)
        cfe_help_text_hash(c, 0x0046_5480, 0x001b_fdc0)?;
        let name = c.g[V0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x54), name); // muiScreenName
        cfe_help_set_description(c, 0x0046_54a0, 0x001b_fdd0, 0x001b_fde0)?;
        cfe_help_set_option(c, 0, 0x0045_fe60, 3, 0x001b_fdec, 0x001b_fe00)?;
        cfe_help_set_option(c, 1, 0x0046_2c50, 0, 0x001b_fe0c, 0x001b_fe20)?;
        cfe_help_set_option(c, 2, 0x0046_1458, 2, 0x001b_fe2c, 0x001b_fe40)?;
        cfe_help_set_option(c, 3, 0x0046_54c0, 1, 0x001b_fe4c, 0x001b_fe60)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cFEStateOnlineMainMenu: update and widget-created routing.

/// `cFEStateOnlineMainMenu_Update(this)` (0x1bfca0): the base state's update,
/// tail-called with a 0x10 frame.
pub fn cfe_state_online_main_menu_update(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x001a_8770), 0x001b_fcb0);
}

/// How the widget-created probe chain ends: store `v0` at `widget+0x18`
/// (after one extra cycle), store 0 there, or leave the widget alone.
#[derive(Clone, Copy)]
enum MainMenuWidgetExit {
    Store,
    Zero,
    Done,
}

/// One probe of the widget's name hash (`widget+0x38`) against the string at
/// `text` (`GetHashValue32`, 0x317670): `cycles` first, then load
/// `s0 = widget->hash`, `a0 = text`, call; the hash is left in v0.
fn main_menu_widget_probe(c: &mut Cpu, cycles: u64, text: u32, ret: u32) -> Flow {
    c.cycles += cycles;
    c.g[S0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x38)).lw(c);
    c.g[A0][0] = sx(text);
    call_ptr(c, 0x0031_7670, ret)
}

/// The probes after the first one: the widget's kind is the first name that
/// matches. The last probe restores `s0` whether it matched or not.
fn main_menu_widget_rest(c: &mut Cpu, frame: Frame) -> Result<MainMenuWidgetExit, Unwound> {
    main_menu_widget_probe(c, 5, 0x0046_5420, 0x001b_fd00)?;
    if c.g[S0][0] == c.g[V0][0] {
        c.cycles += 2;
        c.g[V0][0] = 2;
        return Ok(MainMenuWidgetExit::Store);
    }
    main_menu_widget_probe(c, 5, 0x0046_5430, 0x001b_fd1c)?;
    if c.g[S0][0] == c.g[V0][0] {
        c.cycles += 2;
        c.g[V0][0] = 3;
        return Ok(MainMenuWidgetExit::Store);
    }
    main_menu_widget_probe(c, 5, 0x0046_5440, 0x001b_fd38)?;
    if c.g[S0][0] == c.g[V0][0] {
        c.cycles += 2;
        return Ok(MainMenuWidgetExit::Zero);
    }
    main_menu_widget_probe(c, 5, 0x0046_5458, 0x001b_fd54)?;
    if c.g[S0][0] == c.g[V0][0] {
        c.cycles += 2;
        c.g[V0][0] = 4;
        return Ok(MainMenuWidgetExit::Store);
    }
    main_menu_widget_probe(c, 5, 0x0046_5470, 0x001b_fd70)?;
    let differs = c.g[S0][0] != c.g[V0][0];
    frame.restore(c, S0, 0x20);
    if differs {
        return Ok(MainMenuWidgetExit::Done);
    }
    c.cycles += 1;
    c.g[V0][0] = 5;
    Ok(MainMenuWidgetExit::Store)
}

/// `cFEStateOnlineMainMenu_OnWidgetCreated(this, widget)` (0x1bfcc0): sets
/// `widget+0x18` to the kind of the main-menu entry from the widget's name
/// hash (1: quick matchup, 2: lobby, 3: stats, 0: setup character, 4: online
/// agreement, 5: log off). Unknown names leave the field alone.
pub fn cfe_state_online_main_menu_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        main_menu_widget_probe(c, 0, 0x0046_5408, 0x001b_fce4)?; // TextQuickMatchup
        let matched = c.g[S0][0] == c.g[V0][0];
        c.g[A0][0] = 0x0046_0000;
        let exit = if matched {
            c.cycles += 2;
            c.g[V0][0] = 1;
            MainMenuWidgetExit::Store
        } else {
            main_menu_widget_rest(c, frame)?
        };
        let widget = c.g[S1][0] as u32;
        match exit {
            MainMenuWidgetExit::Store => {
                c.cycles += 1;
                let kind = c.g[V0][0];
                c.sw(widget.wrapping_add(0x18), kind);
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            }
            MainMenuWidgetExit::Zero => {
                c.sw(widget.wrapping_add(0x18), 0);
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            }
            MainMenuWidgetExit::Done => {}
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cFEStateOnlineAgreement: constructor.

/// `cFEStateOnlineAgreement_ctor_2(this)` (0x1c03d0): the base `NetFEState`
/// constructor, then the agreement's kind (0x43), vtable (0x46ac38) and an
/// empty TOS line list.
pub fn cfe_state_online_agreement_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x001a_8500, 0x001c_03e4)?; // NetFEState_ctor
        let this = c.g[S0][0] as u32;
        c.g[V0][0] = 0x43;
        c.g[V1][0] = 0x0047_0000;
        let kind = c.g[V0][0];
        c.sw(this.wrapping_add(0xc), kind); // mHashName
        c.g[V1][0] = sx(0x0047_0000u32.wrapping_sub(0x53c8));
        let vtable = c.g[V1][0];
        c.sw(this.wrapping_add(0x8), vtable); // vtable
        c.g[V0][0] = c.g[S0][0];
        c.sw(this.wrapping_add(0x6d0), 0); // mpTOSLines
        c.sw(this.wrapping_add(0x6d8), 0); // miNextTOSLine
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{fall_back, hook, machine, vtable};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word in the machine's RAM: the update fake acts only when it is set.
    const OC896_GATE: u32 = DATA + 0x0f_0000;
    const OC896_MAGIC: u32 = 0x0896_0896;
    static OC896_UPDATE_PREV: AtomicUsize = AtomicUsize::new(0);

    /// Gated stand-in for `NetFEState_Update` (the override slot of 0x1a8770):
    /// records its `this` and returns; with the gate off it runs the body it
    /// replaced.
    fn oc896_fake_net_update(c: &mut Cpu) {
        if c.bus.read32(OC896_GATE) != OC896_MAGIC {
            return fall_back(c, &OC896_UPDATE_PREV, ee::s03::f_001a8770__generated);
        }
        c.bus.write32(OC896_GATE + 4, c.g[A0][0] as u32);
        c.pc = c.g[RA][0] as u32;
    }

    /// Scrambled machine with the BELibrary dummy and a clear world pointer.
    fn oc896_setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = machine(c, seed, salt, 0);
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x1000);
        c.bus.write32(GAME_GP.wrapping_sub(0x1efc), 0);
        r
    }

    /// Writes a NUL-terminated string at `at`.
    fn oc896_put_text(c: &mut Cpu, at: u32, text: &str) {
        for (i, b) in text.bytes().enumerate() {
            c.bus.write8(at + i as u32, b);
        }
        c.bus.write8(at + text.len() as u32, 0);
    }

    /// `GetHashValue32` of the string at `text`, computed with the generated
    /// body (the same on both sides of a comparison).
    fn oc896_string_hash(c: &mut Cpu, text: u32) -> u32 {
        c.g[A0][0] = sx(text);
        c.g[RA][0] = sx(crate::difftest::RA);
        ee::s10::f_00317670__generated(c);
        c.g[V0][0] as u32
    }

    #[test]
    fn cfe_state_login_on_child_event_matches() {
        check(
            ee::s03::f_001bf6c0__generated,
            cfe_state_login_on_child_event,
            120,
            &|c, seed| {
                let mut r = oc896_setup(c, seed, 0x1bf6c0);
                let obj = DATA + 0x2000;
                let ev = DATA + 0x2800;
                let vt = vtable(c, &mut r, &[0x108]);
                c.bus.write32(obj + 8, vt);
                let kind = match seed % 5 {
                    0 => 0x82c,
                    1 => 0x80e,
                    2 => 0x848,
                    3 => 0x800,
                    _ => 0x900,
                };
                c.bus.write32(ev + 0x18, kind);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx(ev);
            },
        );
    }

    #[test]
    fn cfe_state_login_v25_matches() {
        check(
            ee::s03::f_001bfae0__generated,
            cfe_state_login_v25,
            60,
            &|c, seed| {
                oc896_setup(c, seed, 0x1bfae0);
                c.g[A0][0] = sx(DATA + 0x3000);
                c.g[A1][0] = sx(DATA + 0x2000);
            },
        );
    }

    #[test]
    fn cfe_state_online_main_menu_v25_matches() {
        check(
            ee::s03::f_001bfd98__generated,
            cfe_state_online_main_menu_v25,
            60,
            &|c, seed| {
                oc896_setup(c, seed, 0x1bfd98);
                c.g[A0][0] = sx(DATA + 0x3000);
                c.g[A1][0] = sx(DATA + 0x2000);
            },
        );
    }

    #[test]
    fn cfe_state_online_main_menu_update_matches() {
        hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A8770),
            oc896_fake_net_update,
            &OC896_UPDATE_PREV,
        );
        check(
            ee::s03::f_001bfca0__generated,
            cfe_state_online_main_menu_update,
            60,
            &|c, seed| {
                oc896_setup(c, seed, 0x1bfca0);
                c.bus.write32(OC896_GATE, OC896_MAGIC);
                c.g[A0][0] = sx(DATA + 0x2000);
            },
        );
    }

    #[test]
    fn cfe_state_online_main_menu_on_widget_created_matches() {
        const TEXTS: [u32; 6] = [
            0x0046_5408,
            0x0046_5420,
            0x0046_5430,
            0x0046_5440,
            0x0046_5458,
            0x0046_5470,
        ];
        check(
            ee::s03::f_001bfcc0__generated,
            cfe_state_online_main_menu_on_widget_created,
            140,
            &|c, seed| {
                oc896_setup(c, seed, 0x1bfcc0);
                let mut hashes = [0u32; 6];
                for (i, &at) in TEXTS.iter().enumerate() {
                    oc896_put_text(c, at, &format!("TEXT{i}"));
                    hashes[i] = oc896_string_hash(c, at);
                }
                let widget = DATA + 0x2000;
                // seed % 7 picks the probe that matches (6: none does).
                let key = match seed % 7 {
                    k @ 0..=5 => hashes[k as usize],
                    _ => 0xdead_beef,
                };
                c.bus.write32(widget + 0x38, key);
                c.g[A1][0] = sx(widget);
            },
        );
    }

    #[test]
    fn cfe_state_online_agreement_ctor_2_matches() {
        check(
            ee::s03::f_001c03d0__generated,
            cfe_state_online_agreement_ctor_2,
            60,
            &|c, seed| {
                let mut r = oc896_setup(c, seed, 0x1c03d0);
                c.g[A0][0] = sx(DATA + 0x2000);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }
}
