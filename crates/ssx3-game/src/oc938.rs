//! Staging module oc938 (wave wave255): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc938`.

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
        0x001c_8930,
        c_fe_popup_confirm_set_widget_align,
        "cFEPopupConfirm_SetWidgetAlign",
    ),
    (
        0x001c_9b98,
        c_fe_popup_confirm_open_confirm_popup,
        "cFEPopupConfirm_OpenConfirmPopup",
    ),
];

/// `void cFEPopupConfirm_SetWidgetAlign(void *unused, cWidget *obj, uint8 state, float t)`
/// (0x1c8930; the peak3 fork calls it `cScreenPopup_changePivot`). The word at
/// `obj+0x14` holds the alignment in bits 13..18. When it already equals
/// `state` nothing changes. Otherwise the widget's size comes from its
/// vtable method 20, the new alignment is stored, and the x of the origin
/// (`obj+0x44`) moves by `t` (`size.x / 2` when `t` is zero): right for states
/// 17 and 18, left for 9 and 10. The origin and the alignment word go through
/// the stack frame, as the generated code copies them.
pub fn c_fe_popup_confirm_set_widget_align(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x50);
        c.g[S1][0] = c.g[A2][0] & 0xff;
        frame.save_ra(c, 0x20);
        let obj = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(obj.wrapping_add(0x14)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 13);
        c.g[V0][0] &= 0x3f;
        // mov.s f20, f12
        c.f[20] = c.f[12];
        if c.g[V0][0] != c.g[S1][0] {
            set_widget_align_move(c, frame, obj)?;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 20, 0x50);
        frame.close(c);
        Ok(())
    });
}

/// The changing branch of [`c_fe_popup_confirm_set_widget_align`]: calls the
/// widget's size method, stores the alignment word, scales `t` when it is
/// zero, moves the origin's x for the state, and copies the origin and the
/// z word back from the frame.
fn set_widget_align_move(c: &mut Cpu, frame: Frame, obj: u32) -> Flow {
    c.cycles += 23;
    let vt = Addr(obj.wrapping_add(8)).lw(c);
    c.g[V0][0] = vt;
    c.g[A1][0] = c.g[SP][0];
    c.g[A0][0] = Addr((vt as u32).wrapping_add(0xa0)).lh(c);
    c.g[V1][0] = Addr((vt as u32).wrapping_add(0xa4)).lw(c);
    let method = c.g[V1][0] as u32;
    c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
    // vtable method 20 (the widget's size into the frame at sp).
    call_ptr(c, method, 0x001c_8978)?;
    // Read the widget back from s0 after the call, as the generated code does.
    let obj = c.g[S0][0] as u32;

    // The alignment word: clear bits 13..18, insert the new state.
    c.g[A0][0] = Addr(obj.wrapping_add(0x14)).lw(c);
    c.g[V0][0] = 0xffff_ffff_fff8_0000 | 0x1fff;
    c.f[0] = 0; // mtc1 $zero, $f0
    c.g[V1][0] = c.g[S1][0] & 0x3f;
    c.g[A0][0] &= c.g[V0][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 13);
    c.c_eq_s(20, 0);
    c.g[A0][0] |= c.g[V1][0];

    // The origin (8 bytes at +0x44) and its z word, through the frame.
    c.ldl(V0, obj.wrapping_add(0x4b));
    c.ldr(V0, obj.wrapping_add(0x44));
    c.g[V1][0] = Addr(obj.wrapping_add(0x4c)).lw(c);
    let at_lo = frame.at(c, 0x17);
    c.sdl(V0, at_lo);
    let at_hi = frame.at(c, 0x10);
    c.sdr(V0, at_hi);
    let at_z = frame.at(c, 0x18);
    let z = c.g[V1][0];
    c.sw(at_z, z);

    // `t` is zero: use size.x / 2 (size.x is the first float of the frame).
    let word = c.g[A0][0];
    Addr(obj.wrapping_add(0x14)).sw(c, word);
    if c.fcc() {
        c.cycles += 4;
        let size_x = frame.at(c, 0);
        c.lwc1(1, size_x);
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(AT, 0);
        c.mul_s(20, 1, 0);
    }

    // Move the origin's x for the state (17/18 right, 9/10 left).
    c.cycles += 3;
    let state = c.g[S1][0] as i64;
    if state >= 9 {
        c.cycles += 2;
        let x_slot = frame.at(c, 0x10);
        c.lwc1(0, x_slot);
        let moved = if state < 11 {
            c.cycles += 1;
            c.sub_s(0, 0, 20);
            true
        } else {
            c.cycles += 3;
            if state < 19 {
                c.cycles += 2;
                if state >= 17 {
                    c.cycles += 2;
                    c.add_s(0, 0, 20);
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };
        if moved {
            c.cycles += 1;
            c.swc1(0, x_slot);
        }
    }

    // Copy the origin and z back into the widget.
    c.cycles += 6;
    let lo = frame.at(c, 0x17);
    c.ldl(V0, lo);
    let hi = frame.at(c, 0x10);
    c.ldr(V0, hi);
    let z_at = frame.at(c, 0x18);
    c.g[V1][0] = Addr(z_at).lw(c);
    c.sdl(V0, obj.wrapping_add(0x4b));
    c.sdr(V0, obj.wrapping_add(0x44));
    let z_back = c.g[V1][0];
    Addr(obj.wrapping_add(0x4c)).sw(c, z_back);
    Ok(())
}

/// `void cFEPopupConfirm_OpenConfirmPopup(cFEPopupConfirm *self, void *unused, int idx)`
/// (0x1c9b98; the peak3 fork's `cScreenPopup_createConfirmationPopup`). Allocates
/// a 0x360-byte popup and constructs it (`cFEPopupConfirm_BaseCtor`), keeps it
/// at `self+0x2ac`, sets its two option texts from the hashes of two strings,
/// places it at (30, 40, 0), copies entry `idx` (12 bytes per entry) into it,
/// and calls the popup's setup method.
pub fn c_fe_popup_confirm_open_confirm_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 77;
        let frame = Frame::open(c, 0x60);
        c.g[A3][0] = 0;
        frame.save(c, S3, 0x20);
        c.g[A2][0] = 0x100;
        frame.save(c, S1, 0x40);
        c.g[S3][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = sx(0x0046_0000u32.wrapping_add(0x6110));
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0x10);
        // popup = cMemMan_alloc(0x360, D_00466110, 0x100, 0)
        c.g[A0][0] = 0x360;
        call_ptr(c, 0x0031_7d70, 0x001c_9bd0)?;

        // popup = cFEPopupConfirm_BaseCtor(popup, self->0x10, self)
        let self_ = c.g[S1][0] as u32;
        c.g[A1][0] = Addr(self_.wrapping_add(0x10)).lw(c);
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        Callee {
            addr: 0x001c_58e8,
            f: ee::s03::f_001c58e8,
        }
        .call(c, 0x001c_9be0)?;
        c.g[S0][0] = c.g[V0][0];
        let self_ = c.g[S1][0] as u32;
        let popup = c.g[S0][0] as u32;

        let s0_value = c.g[S0][0];
        Addr(self_.wrapping_add(0x2ac)).sw(c, s0_value);
        // Option texts: CfgSetOptionTextId(slots, 0, GetHashValue32(D_0045DCC8))
        // and CfgSetOptionTextId(slots, 1, GetHashValue32(D_0045FCD8)).
        let slots = popup.wrapping_add(0xbc);
        c.g[V0][0] = 2;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_sub(0x2338));
        let count = c.g[V0][0];
        Addr(slots).sw(c, count);
        call_ptr(c, 0x0031_7670, 0x001c_9bfc)?;
        c.g[S2][0] = sx(slots);
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        Callee {
            addr: 0x001c_57c0,
            f: ee::s03::f_001c57c0,
        }
        .call(c, 0x001c_9c10)?;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_sub(0x328));
        call_ptr(c, 0x0031_7670, 0x001c_9c1c)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 1;
        Callee {
            addr: 0x001c_57c0,
            f: ee::s03::f_001c57c0,
        }
        .call(c, 0x001c_9c2c)?;
        let self_ = c.g[S1][0] as u32;
        let popup = c.g[S0][0] as u32;

        // The popup's origin: (30, 40, 0), stored as 12 bytes at +0x158.
        c.g[AT][0] = 0x41f0_0000;
        c.mtc1(AT, 1);
        c.g[A0][0] = 0xc;
        c.g[AT][0] = 0x4220_0000;
        c.mtc1(AT, 0);
        c.mult(0, A0, S3, A0); // a0 = idx * 12
        let s0_at = frame.at(c, 0);
        c.swc1(1, s0_at);
        let s4_at = frame.at(c, 4);
        c.swc1(0, s4_at);
        let s8_at = frame.at(c, 8);
        c.sw(s8_at, 0);
        let p_lo = frame.at(c, 7);
        c.ldl(V0, p_lo);
        let p_hi = frame.at(c, 0);
        c.ldr(V0, p_hi);
        let z_at = frame.at(c, 8);
        c.g[V1][0] = Addr(z_at).lw(c);
        c.sdl(V0, popup.wrapping_add(0x15f));
        c.sdr(V0, popup.wrapping_add(0x158));
        let z = c.g[V1][0];
        Addr(popup.wrapping_add(0x160)).sw(c, z);

        // The entry idx of self: its count and its two fields into the popup.
        c.g[A1][0] = sx(self_.wrapping_add(c.g[A0][0] as u32) as u32);
        c.g[V0][0] = Addr(self_.wrapping_add(0x314)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(self_));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(3));
        let v0 = c.g[V0][0];
        Addr(popup.wrapping_add(0x14c)).sw(c, v0);
        let entry = c.g[A1][0] as u32;
        c.g[V1][0] = Addr(entry.wrapping_add(0x2d8)).lw(c);
        let v1 = c.g[V1][0];
        Addr(popup.wrapping_add(0x168)).sw(c, v1);
        let key = c.g[A0][0] as u32;
        c.ldl(V0, key.wrapping_add(0x2d7));
        c.ldr(V0, key.wrapping_add(0x2d0));
        c.sdl(V0, popup.wrapping_add(0xc7));
        c.sdr(V0, popup.wrapping_add(0xc0));
        let idx = c.g[S3][0];
        Addr(self_.wrapping_add(0x324)).sw(c, idx);

        // Setup call: method 0x18 of the popup manager's vtable (object at self+0x40).
        c.g[V1][0] = Addr(self_.wrapping_add(0x40)).lw(c);
        c.g[A1][0] = Addr(self_.wrapping_add(0x2ac)).lw(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xd0)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x10)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0039_f290, 0x001c_9cb0)?;

        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, fill, Rng, DATA};

    #[test]
    fn c_fe_popup_confirm_set_widget_align_matches() {
        check(
            ee::s03::f_001c8930__generated,
            c_fe_popup_confirm_set_widget_align,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x001c_8930, 0);
                let obj = DATA + 0x20_0000;
                // Every state 0..23 (so 9/10, 17/18 and the no-move cases),
                // the alignment equal to it for half the seeds, and t zero
                // for a quarter of them.
                let state = seed % 24;
                let old = if seed % 2 == 0 {
                    state
                } else {
                    r.next_u32() % 64
                };
                let vt = fakes::vtable(c, &mut r, &[0xa0]);
                let word = (r.next_u32() & !(0x3f << 13)) | (old << 13);
                c.bus.write32(obj + 0x14, word);
                c.bus.write32(obj + 8, vt);
                for k in 0..3 {
                    c.bus.write32(obj + 0x44 + 4 * k, r.edge_f32());
                }
                c.g[A1][0] = obj as u64;
                c.g[A2][0] = (state as u64) | ((r.next_u32() as u64) << 8);
                c.f[12] = if seed % 4 == 0 { 0 } else { r.edge_f32() };
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_open_confirm_popup_matches() {
        check(
            ee::s03::f_001c9b98__generated,
            c_fe_popup_confirm_open_confirm_popup,
            60,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x001c_9b98, 0);
                let this = DATA + 0x20_0000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 3) as u64;
                c.g[A2][0] = r.next_u32() as u64;
                // Widget fields the code reads, pointing into the test data.
                fill(c, &mut r, this, 0x340 / 4);
                c.bus.write32(this + 0x40, this + 0x400);
                c.bus.write32(this + 0x400 + 0xd0, this + 0x800);
                c.bus.write32(this + 0x800 + 0x10, this + 0xa00);
                c.bus.write32(this + 0x314, r.next_u32() % 16);
                c.bus.write32(this + 0x10, 0);
            },
        );
    }
}
