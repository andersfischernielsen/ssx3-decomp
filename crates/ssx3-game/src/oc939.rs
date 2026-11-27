//! Staging module oc939 (wave wave255): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc939`.

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
        0x001c_a0f0,
        fepopup_confirm_set_option_enabled,
        "cFEPopupConfirm_SetOptionEnabled",
    ),
    (
        0x001c_a578,
        fepopup_confirm_render,
        "cFEPopupConfirm_Render",
    ),
    (0x001c_b030, keyboard_popup_ctor, "cKeyboardPopup_ctor"),
    (
        0x001c_b2c8,
        keyboard_popup_on_widget_created,
        "cKeyboardPopup_OnWidgetCreated",
    ),
    (
        0x001c_bb70,
        keyboard_popup_move_up_from_top_row,
        "cKeyboardPopup_MoveUpFromTopRow",
    ),
];

/// `cKeyboardPopup::IsKeyEnabled(self, key, &val)` (0x1ce560).
const KB_IS_KEY_ENABLED: Callee = Callee {
    addr: 0x001c_e560,
    f: ee::s04::f_001ce560,
};

/// `cKeyboardPopup::FindEnabledInColumnFromBottom(self, -1)` (0x1cc0e0).
const KB_FIND_ENABLED_FROM_BOTTOM: Callee = Callee {
    addr: 0x001c_c0e0,
    f: ee::s04::f_001cc0e0,
};

/// `cKeyboardPopup::BuildKeys(self)` (0x1ce2a8).
const KB_BUILD_KEYS: Callee = Callee {
    addr: 0x001c_e2a8,
    f: ee::s04::f_001ce2a8,
};

/// `cFEPopupConfirm::SetOptionEnabled(self, idx, a)` (0x1ca0f0): the option
/// object at `self + 0x90 + 4 * idx`, when there is one, gets its vtable's
/// slot 8 called with `a` and its slot 25 with 1. Each method is reached
/// through its vtable entry (`lh` delta added to the object, `lw` function).
pub fn fepopup_confirm_set_option_enabled(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let f = Frame::open(c, 0x20);
        c.g[A1][0] = sx(r32(c, A1) << 2);
        f.save_ra(c, 0);
        let slot = r32(c, A0).wrapping_add(r32(c, A1));
        c.g[A0][0] = sx(slot);
        f.save(c, S0, 0x10);
        c.g[S0][0] = Addr(slot.wrapping_add(0x90)).lw(c);
        let present = c.g[S0][0] != 0;
        c.g[A1][0] = c.g[A2][0];
        if present {
            c.cycles += 11;
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(8)).lw(c);
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x40)).lh(c);
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x44)).lw(c);
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
            let target = r32(c, V1);
            call_ptr(c, target, 0x001c_a124)?;
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(8)).lw(c);
            c.g[A1][0] = 1;
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0xc8)).lh(c);
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0xcc)).lw(c);
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
            let target = r32(c, V1);
            call_ptr(c, target, 0x001c_a13c)?;
        }
        c.cycles += 4;
        f.restore(c, S0, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `cFEPopupConfirm::Render` (0x1ca578): forwards to `UIState_Render`.
pub fn fepopup_confirm_render(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x0039_e6b8), 0x001c_a588);
}

/// `cKeyboardPopup::cKeyboardPopup(self, engine, owner, a3, ctrl)` (0x1cb030),
/// the constructor: the `cUIState` base (0x39e318), then the popup's fields
/// (+0x15 the control byte, +0x70 `a3`, +0x424 the text object reset by
/// 0x3180b8, the vtable, the defaults), then `BuildKeys`.
pub fn keyboard_popup_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 50;
        let f = Frame::open(c, 0x50);
        f.save(c, S3, 0x10);
        c.g[S3][0] = c.g[A0][0];
        f.save(c, S1, 0x30);
        f.save(c, S2, 0x20);
        c.g[S1][0] = c.g[T0][0] & 0xff;
        f.save(c, S0, 0x40);
        f.save_ra(c, 0);
        c.g[S2][0] = c.g[A3][0];
        call_ptr(c, 0x0039_e318, 0x001c_b058)?;
        c.g[S0][0] = 1;
        c.g[A2][0] = Addr(r32(c, GP).wrapping_add(0xda0)).lw(c);
        c.g[V0][0] = sx(0x0046_bec8);
        c.g[V1][0] = 0x1d;
        c.g[A1][0] = 0x3f;
        let this = r32(c, S3);
        {
            let a = this.wrapping_add(0x15);
            let v = c.g[S1][0];
            Addr(a).sb(c, v);
        }
        {
            let a = this.wrapping_add(0x70);
            let v = c.g[S2][0];
            Addr(a).sw(c, v);
        }
        c.g[A0][0] = sx(this.wrapping_add(0x424));
        Addr(this.wrapping_add(0x418)).sw(c, 0);
        Addr(this.wrapping_add(0x41c)).sw(c, 0);
        {
            let a = this.wrapping_add(0x420);
            let v = c.g[S0][0];
            Addr(a).sw(c, v);
        }
        Addr(this.wrapping_add(0x64)).sw(c, 0);
        Addr(this.wrapping_add(0x6c)).sw(c, 0);
        {
            let a = this.wrapping_add(0x68);
            let v = c.g[S0][0];
            Addr(a).sw(c, v);
        }
        Addr(this.wrapping_add(0x74)).sb(c, 0);
        Addr(this.wrapping_add(0x134)).sw(c, 0);
        Addr(this.wrapping_add(0x138)).sw(c, 0);
        Addr(this.wrapping_add(0x13c)).sw(c, 0);
        Addr(this.wrapping_add(0x54)).sw(c, 0);
        Addr(this.wrapping_add(0x58)).sw(c, 0);
        {
            let a = this.wrapping_add(0x8);
            let v = c.g[V0][0];
            Addr(a).sw(c, v);
        }
        {
            let a = this.wrapping_add(0x140);
            let v = c.g[V1][0];
            Addr(a).sw(c, v);
        }
        {
            let a = this.wrapping_add(0x424);
            let v = c.g[A2][0];
            Addr(a).sw(c, v);
        }
        {
            let a = this.wrapping_add(0x60);
            let v = c.g[A1][0];
            Addr(a).sw(c, v);
        }
        call_ptr(c, 0x0031_80b8, 0x001c_b0c0)?;
        let this = r32(c, S3);
        c.g[A0][0] = Addr(r32(c, GP).wrapping_add(0x300)).lw(c);
        c.g[V0][0] = 2;
        c.g[V1][0] = 0x2b;
        {
            let a = this.wrapping_add(0x144);
            let v = c.g[V0][0];
            Addr(a).sw(c, v);
        }
        {
            let a = this.wrapping_add(0x434);
            let v = c.g[V1][0];
            Addr(a).sw(c, v);
        }
        {
            let a = this.wrapping_add(0x43c);
            let v = c.g[S0][0];
            Addr(a).sw(c, v);
        }
        Addr(this.wrapping_add(0x428)).sw(c, 0);
        Addr(this.wrapping_add(0x42c)).sw(c, 0);
        Addr(this.wrapping_add(0x430)).sw(c, 0);
        Addr(this.wrapping_add(0x438)).sw(c, 0);
        Addr(this.wrapping_add(0x48)).sw(c, 0);
        Addr(this.wrapping_add(0x4c)).sw(c, 0);
        let has_gp300 = c.g[A0][0] != 0;
        Addr(this.wrapping_add(0x50)).sw(c, 0);
        if has_gp300 {
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x88)).lw(c);
            c.g[V0][0] = sx(r32(c, V0) >> 1);
            c.g[V0][0] &= 0x1;
            {
                let a = this.wrapping_add(0x68);
                let v = c.g[V0][0];
                Addr(a).sw(c, v);
            }
        }
        c.cycles += 12;
        c.g[A0][0] = c.g[S3][0];
        KB_BUILD_KEYS.call(c, 0x001c_b110)?;
        let this = r32(c, S3);
        c.g[V0][0] = c.g[S3][0];
        Addr(this.wrapping_add(0x440)).sw(c, 0);
        Addr(this.wrapping_add(0x5c)).sw(c, 0);
        f.restore(c, S0, 0x40);
        f.restore(c, S1, 0x30);
        f.restore(c, S2, 0x20);
        f.restore(c, S3, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// One lookup of `cKeyboardPopup::onWidgetCreate` (0x317670 on the text
/// object at `a0 + add`, compared with the widget's +0x38 word). Returns
/// whether they match.
fn widget_created_lookup(c: &mut Cpu, add: u32, ret_addr: u32) -> Result<bool, Unwound> {
    c.cycles += 5;
    c.g[S0][0] = Addr(r32(c, S1).wrapping_add(0x38)).lw(c);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(add));
    call_ptr(c, 0x0031_7670, ret_addr)?;
    Ok(c.g[S0][0] == c.g[V0][0])
}

/// The matched arm of the first lookup in `cKeyboardPopup::onWidgetCreate`:
/// two notifications through the state's vtable, then `0x1cb360` when the
/// popup is open. Returns `true` when it already restored `s0` and goes
/// straight to the epilogue.
fn widget_created_matched(c: &mut Cpu, f: &Frame) -> Result<bool, Unwound> {
    c.cycles += 10;
    c.g[V0][0] = Addr(r32(c, S1).wrapping_add(8)).lw(c);
    c.g[A1][0] = 0;
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x48)).lh(c);
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x4c)).lw(c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
    call_ptr(c, target, 0x001c_b314)?;
    c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0x1680)).lw(c);
    c.g[V0][0] ^= 0x1;
    let skip_second = c.g[V0][0] != 0;
    c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
    if !skip_second {
        c.cycles += 7;
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(8)).lw(c);
        c.g[A1][0] = 1;
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x48)).lh(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x4c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x001c_b33c)?;
        c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
    }
    c.cycles += 3;
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x84)).lw(c);
    let closed = c.g[V1][0] == 0;
    c.g[A1][0] = 0;
    if closed {
        return Ok(false);
    }
    c.cycles += 7;
    c.g[V0][0] = Addr(r32(c, S1).wrapping_add(8)).lw(c);
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x48)).lh(c);
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x4c)).lw(c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
    call_ptr(c, target, 0x001c_b360)?;
    f.restore(c, S0, 0x30);
    Ok(true)
}

/// `cKeyboardPopup::onWidgetCreate(self, widget)` (0x1cb2c8): when the text
/// object of the widget matches one of the three lookups, the matching
/// field of the popup (+0x48, +0x50 or +0x4c) takes the widget.
pub fn keyboard_popup_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let f = Frame::open(c, 0x40);
        f.save(c, S2, 0x10);
        f.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A0][0];
        f.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        f.save(c, S0, 0x30);
        c.g[S0][0] = Addr(r32(c, S1).wrapping_add(0x38)).lw(c);
        c.g[A0][0] = sx(0x0046_6398);
        call_ptr(c, 0x0031_7670, 0x001c_b2f4)?;
        let direct_exit = if c.g[S0][0] == c.g[V0][0] {
            widget_created_matched(c, &f)?
        } else {
            c.g[A0][0] = 0x0046_0000;
            if widget_created_lookup(c, 0x63a8, 0x001c_b374)? {
                c.cycles += 2;
                {
                    let a = r32(c, S2).wrapping_add(0x48);
                    let v = c.g[S1][0];
                    Addr(a).sw(c, v);
                }
            } else {
                c.g[A0][0] = 0x0046_0000;
                if widget_created_lookup(c, 0x63b8, 0x001c_b390)? {
                    c.cycles += 2;
                    {
                        let a = r32(c, S2).wrapping_add(0x50);
                        let v = c.g[S1][0];
                        Addr(a).sw(c, v);
                    }
                } else {
                    c.g[A0][0] = 0x0046_0000;
                    if widget_created_lookup(c, 0x63c8, 0x001c_b3ac)? {
                        {
                            let a = r32(c, S2).wrapping_add(0x4c);
                            let v = c.g[S1][0];
                            Addr(a).sw(c, v);
                        }
                    }
                }
            }
            false
        };
        if !direct_exit {
            c.cycles += 1;
            f.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        f.restore(c, S1, 0x20);
        f.restore(c, S2, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// The `else` arm of one quick-key state of `handleMovementUpFromQuickKeyRow`:
/// `IsKeyEnabled(self, key, &val)` (the key is `ret_addr`'s call). When the
/// key is enabled the state becomes `key`; otherwise `val` decides whether
/// the popup's flag at +0x144 changes, and the enabled key in the column from
/// the bottom is found. Either way the result is 1.
fn move_up_top_key(c: &mut Cpu, key: u32, ret_addr: u32) -> Flow {
    c.g[A1][0] = key as u64;
    c.g[A2][0] = c.g[SP][0];
    KB_IS_KEY_ENABLED.call(c, ret_addr)?;
    let enabled = c.g[V0][0] != 0;
    c.g[V0][0] = key as u64;
    if enabled {
        c.cycles += 2;
        {
            let a = r32(c, S0).wrapping_add(0x140);
            let v = c.g[V0][0];
            Addr(a).sw(c, v);
        }
    } else {
        c.cycles += 3;
        c.g[V0][0] = Addr(sp(c)).lw(c);
        let val_set = c.g[V0][0] != 0;
        c.g[V0][0] = (c.g[S1][0] < 1) as u64;
        if val_set {
            c.cycles += 1;
            {
                let a = r32(c, S0).wrapping_add(0x144);
                let v = c.g[V0][0];
                Addr(a).sw(c, v);
            }
        }
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = u64::MAX;
        KB_FIND_ENABLED_FROM_BOTTOM.call(c, 0x001c_bc24)?;
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    Ok(())
}

/// `cKeyboardPopup::handleMovementUpFromQuickKeyRow` (0x1cbb70): on the
/// quick-key row (popup states 5-12, one key each) the cursor moves to that
/// row's key when it is enabled, else to the enabled key in the column from
/// the bottom. Returns 1 when it handled the move, 0 for any other state.
pub fn keyboard_popup_move_up_from_top_row(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let f = Frame::open(c, 0x40);
        f.save(c, S0, 0x30);
        f.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        f.save_ra(c, 0x10);
        c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0x140)).lw(c);
        c.g[V0][0] = (sx(r32(c, V1).wrapping_sub(5)) < 4) as u64;
        c.g[S1][0] = c.g[A1][0];
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            move_up_top_key(c, 0x4b, 0x001c_bba4)?;
        } else {
            c.cycles += 4;
            c.g[V0][0] = (sx(r32(c, V1).wrapping_sub(9)) < 2) as u64;
            c.g[A0][0] = c.g[S0][0];
            if c.g[V0][0] != 0 {
                c.cycles += 5;
                move_up_top_key(c, 0x4c, 0x001c_bbd0)?;
            } else {
                c.cycles += 4;
                c.g[V0][0] = (sx(r32(c, V1).wrapping_sub(0xb)) < 2) as u64;
                c.g[A1][0] = 0x4f;
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                } else {
                    c.cycles += 4;
                    move_up_top_key(c, 0x4f, 0x001c_bbf8)?;
                }
            }
        }
        c.cycles += 5;
        f.restore(c, S0, 0x30);
        f.restore(c, S1, 0x20);
        f.restore_ra(c, 0x10);
        f.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, machine_rt, FAKE};
    use crate::difftest::{check, Rng, DATA};

    /// The `vt[off]` method of a fixture vtable: `{ delta, fn }`.
    pub(crate) fn put_method(c: &mut Cpu, vt: u32, off: u32, delta: i16, func: u32) {
        c.bus.write16(vt + off, delta as u16);
        c.bus.write32(vt + off + 4, func);
    }

    #[test]
    fn fepopup_confirm_set_option_enabled_matches() {
        check(
            ee::s03::f_001ca0f0__generated,
            fepopup_confirm_set_option_enabled,
            200,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x1ca0, 2);
                let idx = seed % 4;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = r.next_u32() as u64;
                let obj = if seed % 5 == 0 { 0 } else { DATA + 0x1000 };
                c.bus.write32(DATA + 0x90 + 4 * idx, obj);
                let vt = DATA + 0x2000;
                c.bus.write32(DATA + 0x1008, vt);
                put_method(c, vt, 0x40, (r.next_u32() as i16) & !3, FAKE);
                put_method(c, vt, 0xc8, (r.next_u32() as i16) & !3, FAKE + 0x10);
            },
        );
    }

    #[test]
    fn fepopup_confirm_render_matches() {
        check(
            ee::s03::f_001ca578__generated,
            fepopup_confirm_render,
            100,
            &|c, seed| {
                let _ = machine_rt(c, seed, 0x1ca5, 0);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn keyboard_popup_ctor_matches() {
        check(
            ee::s04::f_001cb030__generated,
            keyboard_popup_ctor,
            200,
            &|c, seed| {
                let mut r: Rng = machine_rt(c, seed, 0x1cb0, 0);
                let this = DATA + 0x2000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x3000);
                c.g[A2][0] = sx(DATA + 0x3800);
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = r.next_u32() as u64;
                let gp = crate::difftest::GAME_GP;
                c.bus
                    .write32(gp + 0x300, if seed % 3 == 0 { 0 } else { DATA + 0x5000 });
                c.bus.write32(DATA + 0x5000 + 0x88, r.next_u32());
                c.bus.write32(gp + 0xda0, r.next_u32());
            },
        );
    }

    #[test]
    fn keyboard_popup_on_widget_created_matches() {
        check(
            ee::s04::f_001cb2c8__generated,
            keyboard_popup_on_widget_created,
            200,
            &|c, seed| {
                let mut r: Rng = machine_rt(c, seed, 0x1cb2, 0);
                c.g[A0][0] = sx(DATA + 0x2000);
                c.g[A1][0] = sx(DATA + 0x3000);
                let widget = DATA + 0x3000;
                c.bus.write32(widget + 0x38, r.next_u32());
                c.bus.write32(widget + 0x8, DATA + 0x4000);
                put_method(c, DATA + 0x4000, 0x48, 0, FAKE + 0x20);
                c.bus
                    .write32(crate::difftest::GAME_GP - 0x848, DATA + 0x6000);
                c.bus
                    .write32(DATA + 0x6000 + 0x84, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(crate::difftest::GAME_GP - 0x1680, seed % 2);
            },
        );
    }

    #[test]
    fn keyboard_popup_move_up_from_top_row_matches() {
        check(
            ee::s04::f_001cbb70__generated,
            keyboard_popup_move_up_from_top_row,
            200,
            &|c, seed| {
                let _ = machine_rt(c, seed, 0x1cbb, 0);
                let this = DATA + 0x2000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
                c.bus.write32(this + 0x140, seed % 16);
            },
        );
    }
}
