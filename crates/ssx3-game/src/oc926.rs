//! Staging module oc926 (wave wave254): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc926`.
//!
//! Not ported here: `cFEStateLobbyMenu_OnWidgetNotify` (0x1bdc18),
//! `cFEStateLobbyMenu_UpdatePlayerScrollArrows` (0x1be720) and
//! `cFEStateLogin_OnDisconnectPopupEvent` (0x1bf5a8): already hand-written and
//! registered in `game/fe/feasyncfile.rs` (with tests).

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
        0x001c_2418,
        c_fe_state_net_quick_messages_refresh_all,
        "cFEStateNetQuickMessages_RefreshAll",
    ),
    (
        0x001c_2a18,
        c_fe_state_net_quick_messages_store_edited_message,
        "cFEStateNetQuickMessages_StoreEditedMessage",
    ),
    (
        0x001c_3d68,
        c_fe_state_stats_update_scroll_bar,
        "cFEStateStats_UpdateScrollBar",
    ),
    (
        0x001c_49c8,
        c_fe_state_stats_fill_course_list,
        "cFEStateStats_FillCourseList",
    ),
    (
        0x001c_4bd8,
        c_fe_state_stats_post_input,
        "cFEStateStats_PostInput",
    ),
];

/// A gcc 2.9x virtual call whose vtable word is already in `vt`:
/// `A0 = lh(vt + slot)` (the `this` delta), `func = lw(vt + slot + 4)`,
/// `jalr func` with `A0 = obj + delta`, returning to `ret`.
fn oc926_call_slot(c: &mut Cpu, obj: usize, vt: usize, func: usize, slot: u32, ret: u32) -> Flow {
    c.g[A0][0] = Addr(r32(c, vt)).off(slot).lh(c);
    c.g[func][0] = Addr(r32(c, vt)).off(slot + 4).lw(c);
    let target = r32(c, func);
    c.g[A0][0] = sx(r32(c, obj).wrapping_add(r32(c, A0)));
    call_ptr(c, target, ret)
}

/// [`oc926_call_slot`] after loading `vt = *(obj + vt_off)`.
fn oc926_vcall(
    c: &mut Cpu,
    obj: usize,
    vt: usize,
    func: usize,
    vt_off: u32,
    slot: u32,
    ret: u32,
) -> Flow {
    c.g[vt][0] = Addr(r32(c, obj)).off(vt_off).lw(c);
    oc926_call_slot(c, obj, vt, func, slot, ret)
}

/// `void cFEStateNetQuickMessages_RefreshAll(self)` (0x1c2418): refresh the
/// ten message slots, `RefreshMessage(self, i)` for i = 0..9.
pub fn c_fe_state_net_quick_messages_refresh_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = 0;
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S0][0];
        loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x001c_2468, 0x001c_2440)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            let more = (c.g[S0][0] as i64) < 10;
            c.g[V0][0] = more as u64;
            c.g[A1][0] = c.g[S0][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001c_2438);
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateNetQuickMessages_StoreEditedMessage(self)` (0x1c2a18):
/// store the edited quick message `n = self->edit->index` (0..9). With a
/// message already stored for the slot (the sprintf-style call at 0x416810
/// returns non-zero) the text is copied; otherwise a "message too long"
/// style popup is shown (option text, text id 0x83c). Then the editor
/// object's virtual method at +8 runs and the slot is refreshed.
pub fn c_fe_state_net_quick_messages_store_edited_message(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        frame.save(c, S1, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(r32(c, S2)).off(0x69c).lw(c);
        c.g[S3][0] = Addr(r32(c, V1)).off(0x18).lw(c);
        c.g[V0][0] = ((c.g[S3][0] as u32) < 0xa) as u64;
        let slot_ok = c.g[V0][0] != 0;
        c.g[S0][0] = sx(r32(c, V1).wrapping_add(0x74));
        if slot_ok {
            c.cycles += 10;
            call_ptr(c, 0x0014_dc80, 0x001c_2a54)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x4;
            call_ptr(c, 0x0014_dd58, 0x001c_2a60)?;
            c.g[S4][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0041_6810, 0x001c_2a6c)?;
            if c.g[V0][0] != 0 {
                // The slot already holds a message: copy the text over it.
                c.g[A1][0] = Addr(r32(c, S2)).off(0x69c).lw(c);
                c.cycles += 7;
                c.g[A0][0] = 0x0053_0000;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x5617));
                c.g[V0][0] = sx(r32(c, S3) << 6);
                c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x74));
                call_ptr(c, 0x0041_66f4, 0x001c_2b00)?;
                c.g[V0][0] = 0x1;
            } else {
                c.cycles += 29;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x001a_9c50, 0x001c_2a7c)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = 0x0046_0000;
                c.g[V0][0] = 0x1;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x5c48));
                c.sw(r32(c, S0).wrapping_add(0xbc), c.g[V0][0]);
                c.g[S1][0] = sx(r32(c, S0).wrapping_add(0xbc));
                call_ptr(c, 0x0031_7670, 0x001c_2a98)?;
                c.g[V1][0] = 0x3;
                c.g[A0][0] = 0x0046_0000;
                c.sw(r32(c, S0).wrapping_add(0xc0), c.g[V1][0]);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1618));
                c.sw(r32(c, S0).wrapping_add(0xc4), c.g[V0][0]);
                call_ptr(c, 0x0031_7670, 0x001c_2ab0)?;
                c.g[A2][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 0;
                call_ptr(c, 0x001c_57c0, 0x001c_2ac0)?;
                c.g[V0][0] = 0x83c;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S2][0];
                c.sw(r32(c, S0).wrapping_add(0x150), c.g[V0][0]);
                call_ptr(c, 0x001a_9bc0, 0x001c_2ad4)?;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, 0x0014_f6e8, 0x001c_2ae0)?;
                c.g[V0][0] = 0x1;
            }
            c.cycles += 9;
            c.sw(r32(c, S2).wrapping_add(0x6d0), c.g[V0][0]);
            oc926_vcall(c, S4, V1, V0, 0xc, 0x8, 0x001c_2b1c)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S3][0];
            call_ptr(c, 0x001c_2468, 0x001c_2b28)?;
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateStats_UpdateScrollBar(self, pos, count)` (0x1c3d68): position
/// the scroll-bar thumb widget (`ScrollThumb`, found by name under
/// `self->screen`) on its track (`ScrollTrack`): the thumb's y becomes
/// `track.y + (track.h - 17.0) / n * pos + 10.0` with `n = count - 1` (count
/// when 1), then both widgets are laid out (slot 0x48 of the thumb, track).
/// Does nothing without a screen, for `pos < 0`, `count < pos` or
/// `count <= 0`.
pub fn c_fe_state_stats_update_scroll_bar(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S3, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x50);
        frame.save_ra(c, 0x30);
        c.g[V0][0] = Addr(r32(c, S1)).off(0x40).lw(c);
        c.g[S3][0] = c.g[A1][0];
        'epilogue: {
            'plus1: {
                if c.g[V0][0] == 0 {
                    break 'plus1;
                }
                c.cycles += 2;
                let neg = (c.g[S3][0] as i64) < 0;
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S3][0] as i64)) as u64;
                if neg {
                    break 'plus1;
                }
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x70);
                    break 'epilogue;
                }
                c.cycles += 2;
                if (c.g[S0][0] as i64) <= 0 {
                    frame.restore(c, S0, 0x70);
                    break 'epilogue;
                }
                c.cycles += 16;
                c.g[A0][0] = 0x0046_0000;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x5ce0));
                call_ptr(c, 0x0031_7670, 0x001c_3db8)?;
                c.g[A0][0] = Addr(r32(c, S1)).off(0x40).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001c_3dc4)?;
                c.g[A0][0] = 0x004a_0000;
                c.g[S2][0] = c.g[V0][0];
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1c18));
                call_ptr(c, 0x0031_7670, 0x001c_3dd4)?;
                c.g[A0][0] = Addr(r32(c, S1)).off(0x40).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001c_3de0)?;
                c.g[S1][0] = c.g[V0][0];
                if c.g[S1][0] == 0 {
                    frame.restore(c, S0, 0x70);
                    break 'epilogue;
                }
                c.cycles += 2;
                let no_track = c.g[S2][0] == 0;
                c.g[A1][0] = c.g[SP][0];
                if no_track {
                    break 'plus1;
                }
                c.cycles += 54;
                // track->vtbl[0xa0](track, &rect): rect lands on the stack.
                oc926_vcall(c, S2, V0, V1, 8, 0xa0, 0x001c_3e08)?;
                // rect copy (12 bytes at track+0x44) to sp+0x10
                c.ldl(2, r32(c, S2).wrapping_add(0x4b));
                c.ldr(2, r32(c, S2).wrapping_add(0x44));
                c.g[V1][0] = Addr(r32(c, S2)).off(0x4c).lw(c);
                c.sdl(2, sp(c).wrapping_add(0x17));
                c.sdr(2, sp(c).wrapping_add(0x10));
                c.sw(sp(c).wrapping_add(0x18), c.g[V1][0]);
                c.mtc1(S3, 1);
                c.cvt_s_w(1, 1);
                c.g[V1][0] = ((c.g[S0][0] as i64) < 2) as u64;
                c.g[V0][0] = sx(r32(c, S0).wrapping_sub(1));
                c.lwc1(2, sp(c).wrapping_add(4));
                if c.g[V1][0] == 0 {
                    c.g[S0][0] = c.g[V0][0];
                }
                c.g[AT][0] = 0xffff_ffff_c188_0000; // -17.0f
                c.mtc1(AT, 0);
                c.mtc1(S0, 3);
                c.cvt_s_w(3, 3);
                // thumb rect (12 bytes at thumb+0x44) to sp+0x20
                c.ldl(6, r32(c, S1).wrapping_add(0x4b));
                c.ldr(6, r32(c, S1).wrapping_add(0x44));
                c.g[V0][0] = Addr(r32(c, S1)).off(0x4c).lw(c);
                c.sdl(6, sp(c).wrapping_add(0x27));
                c.sdr(6, sp(c).wrapping_add(0x20));
                c.sw(sp(c).wrapping_add(0x28), c.g[V0][0]);
                c.g[A1][0] = 0x1;
                c.add_s(2, 2, 0);
                c.g[AT][0] = 0x41a0_0000; // 20.0f
                c.mtc1(AT, 4);
                c.lwc1(0, sp(c).wrapping_add(0x14));
                c.div_s(2, 2, 3);
                c.mul_s(1, 1, 2);
                c.add_s(0, 0, 1);
                c.add_s(0, 0, 4);
                c.swc1(0, sp(c).wrapping_add(0x24));
                // thumb rect back from sp+0x20 (with the new y)
                c.ldl(6, sp(c).wrapping_add(0x27));
                c.ldr(6, sp(c).wrapping_add(0x20));
                c.g[V0][0] = Addr(sp(c)).off(0x28).lw(c);
                c.sdl(6, r32(c, S1).wrapping_add(0x4b));
                c.sdr(6, r32(c, S1).wrapping_add(0x44));
                c.sw(r32(c, S1).wrapping_add(0x4c), c.g[V0][0]);
                oc926_vcall(c, S1, V0, V1, 8, 0x48, 0x001c_3eb4)?;
                c.g[V0][0] = Addr(r32(c, S2)).off(8).lw(c);
                c.g[A1][0] = 0x1;
                oc926_call_slot(c, S2, V0, V1, 0x48, 0x001c_3ecc)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x70);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateStats_FillCourseList(self)` (0x1c49c8): fill the course
/// list box (`self->screen`, widget named at 0x465c78) from the 29-entry
/// course table at 0x46d9b8 (16-byte entries `{ id, name, category, area }`):
/// the entries whose category is the list box's selected value and whose
/// area is `self->area` (+0x6e0) are added with the course name text; the
/// first one's id is kept at sp+0 and, when it is a valid course index,
/// the results page is reset and the selection updated before the results
/// are refreshed.
pub fn c_fe_state_stats_fill_course_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S2, 0x70);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x90);
        c.g[A0][0] = 0x0046_0000;
        frame.save(c, S1, 0x80);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save_ra(c, 0x20);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x5c78));
        call_ptr(c, 0x0031_7670, 0x001c_49fc)?;
        c.g[A0][0] = Addr(r32(c, S2)).off(0x40).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001c_4a08)?;
        c.g[S0][0] = c.g[V0][0];
        'epilogue: {
            if c.g[S0][0] == 0 {
                frame.restore(c, S0, 0x90);
                break 'epilogue;
            }
            c.cycles += 17;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0039_a738, 0x001c_4a1c)?;
            c.g[S6][0] = c.g[V0][0];
            call_ptr(c, 0x0014_dc80, 0x001c_4a24)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0014_dd58, 0x001c_4a30)?;
            c.g[A0][0] = 0x0046_0000;
            c.g[S5][0] = c.g[V0][0];
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x5c88));
            call_ptr(c, 0x0031_7670, 0x001c_4a40)?;
            c.g[A0][0] = Addr(r32(c, S2)).off(0x40).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001c_4a4c)?;
            c.g[S0][0] = c.g[V0][0];
            if c.g[S0][0] == 0 {
                frame.restore(c, S0, 0x90);
                break 'epilogue;
            }
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            c.g[S4][0] = 0;
            call_ptr(c, 0x0039_a670, 0x001c_4a64)?;
            c.g[S3][0] = 0x1c;
            c.g[V0][0] = 0x0047_0000;
            c.g[S1][0] = sx(r32(c, V0).wrapping_sub(0x2648));
            loop {
                c.cycles += 11;
                // entry -> sp+0x10 (16 bytes)
                c.ldl(2, r32(c, S1).wrapping_add(0x7));
                c.ldr(2, r32(c, S1));
                c.ldl(3, r32(c, S1).wrapping_add(0xf));
                c.ldr(3, r32(c, S1).wrapping_add(0x8));
                c.sdl(2, sp(c).wrapping_add(0x17));
                c.sdr(2, sp(c).wrapping_add(0x10));
                c.sdl(3, sp(c).wrapping_add(0x1f));
                c.sdr(3, sp(c).wrapping_add(0x18));
                c.g[V0][0] = Addr(sp(c)).off(0x18).lw(c);
                if c.g[V0][0] != c.g[S6][0] {
                    c.g[S3][0] = sx(r32(c, S3).wrapping_sub(1));
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = Addr(sp(c)).off(0x1c).lw(c);
                    c.g[V0][0] = Addr(r32(c, S2)).off(0x6e0).lw(c);
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[S3][0] = sx(r32(c, S3).wrapping_sub(1));
                    } else {
                        c.cycles += 9;
                        c.g[A1][0] = Addr(sp(c)).off(0x14).lw(c);
                        c.g[A0][0] = c.g[S5][0];
                        call_ptr(c, 0x0014_4c60, 0x001c_4ab8)?;
                        c.g[A2][0] = Addr(sp(c)).off(0x10).lw(c);
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0039_a3f8, 0x001c_4ac8)?;
                        let have_first = c.g[S4][0] != 0;
                        c.g[S3][0] = sx(r32(c, S3).wrapping_sub(1));
                        if !have_first {
                            // remember the first match at sp+0
                            c.cycles += 9;
                            c.ldl(2, sp(c).wrapping_add(0x17));
                            c.ldr(2, sp(c).wrapping_add(0x10));
                            c.ldl(3, sp(c).wrapping_add(0x1f));
                            c.ldr(3, sp(c).wrapping_add(0x18));
                            c.sdl(2, sp(c).wrapping_add(0x7));
                            c.sdr(2, sp(c));
                            c.sdl(3, sp(c).wrapping_add(0xf));
                            c.sdr(3, sp(c).wrapping_add(0x8));
                            c.g[S4][0] = 0x1;
                        }
                    }
                }
                c.cycles += 2;
                let more = (c.g[S3][0] as i64) >= 0;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001c_4a70);
                }
            }
            c.cycles += 3;
            c.g[A1][0] = Addr(sp(c)).lw(c);
            let negative = (c.g[A1][0] as i64) < 0;
            c.g[V0][0] = ((c.g[A1][0] as i64) < 29) as u64;
            let valid = !negative && {
                c.cycles += 2;
                c.g[V0][0] != 0
            };
            if valid {
                c.cycles += 14;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x001c_4b78, 0x001c_4b18)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                call_ptr(c, 0x0039_a7a8, 0x001c_4b24)?;
                c.g[A0][0] = 0x0046_0000;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x5d50));
                call_ptr(c, 0x0031_7670, 0x001c_4b30)?;
                c.g[A0][0] = Addr(r32(c, S2)).off(0x40).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001c_4b3c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                call_ptr(c, 0x0039_b960, 0x001c_4b48)?;
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x001c_3310, 0x001c_4b50)?;
            frame.restore(c, S0, 0x90);
        }
        c.cycles += 9;
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateStats_PostInput(self, input)` (0x1c4bd8): after the base
/// input handling (0x39e508), react to the input object's four button
/// queries (virtual slots 0x88, 0x90, 0xa0, 0x98): any of the first three
/// (or the fourth) true and the state `self->mode` (+0x6e4) below 3 moves
/// the column focus: slot 0x88 (previous column), else slot 0x90 (next
/// column).
pub fn c_fe_state_stats_post_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e508, 0x001c_4bf4)?;
        'epilogue: {
            // Which of the queries is true: slots 0x88, 0x90, 0xa0, 0x98.
            oc926_vcall(c, S0, V1, V0, 8, 0x88, 0x001c_4c08)?;
            let mut pressed = c.g[V0][0] != 0;
            if !pressed {
                c.cycles += 7;
                oc926_vcall(c, S0, V1, V0, 8, 0x90, 0x001c_4c24)?;
                pressed = c.g[V0][0] != 0;
            }
            if !pressed {
                c.cycles += 7;
                oc926_vcall(c, S0, V1, V0, 8, 0xa0, 0x001c_4c40)?;
                pressed = c.g[V0][0] != 0;
            }
            if !pressed {
                c.cycles += 7;
                oc926_vcall(c, S0, V1, V0, 8, 0x98, 0x001c_4c5c)?;
                if c.g[V0][0] == 0 {
                    frame.restore(c, S0, 0x20);
                    break 'epilogue;
                }
                c.cycles += 1;
            }
            c.g[V0][0] = Addr(r32(c, S1)).off(0x6e4).lw(c);
            c.cycles += 3;
            c.g[V0][0] = (c.g[V0][0] < 3) as u64;
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x20);
                break 'epilogue;
            }
            c.cycles += 7;
            oc926_vcall(c, S0, V1, V0, 8, 0x88, 0x001c_4c88)?;
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x001c_4e40, 0x001c_4c98)?;
                frame.restore(c, S0, 0x20);
                break 'epilogue;
            }
            c.g[V1][0] = Addr(r32(c, S0)).off(0x8).lw(c);
            c.cycles += 6;
            oc926_call_slot(c, S0, V1, V0, 0x90, 0x001c_4cb0)?;
            let none = c.g[V0][0] == 0;
            frame.restore(c, S0, 0x20);
            if !none {
                c.cycles += 3;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x001c_4d28, 0x001c_4cc0)?;
                frame.restore(c, S0, 0x20);
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{fakes, Rng, DATA};
    use std::sync::atomic::AtomicUsize;

    /// Fixture areas, clear of the fake log (DATA+0x100000), the pointers the
    /// fakes return (DATA..DATA+0x40000), the allocator and the BELibrary dummy.
    const OC926_BASE: u32 = DATA + 0x28_0000;
    /// Control word (magic: the gated fakes are armed) and, from
    /// [`OC926_RES`], per-call forced results (`word - 1`; 0 keeps the fake's).
    const OC926_CTRL: u32 = OC926_BASE;
    const OC926_MAGIC: u32 = 0x4f43_9260;
    const OC926_RES: u32 = OC926_CTRL + 0x1000;
    /// The state object (`self`).
    const OC926_SELF: u32 = OC926_BASE + 0x3000;
    /// Objects the state points at, 0x1000 apart.
    const OC926_OBJ: u32 = OC926_BASE + 0x4000;
    /// Vtables, 0x400 apart.
    const OC926_VT: u32 = OC926_BASE + 0x8000;
    const OC926_ZERO: u32 = OC926_BASE + 0x9000;
    /// The static course table `FillCourseList` reads.
    const OC926_COURSES: u32 = 0x0047_0000 - 0x2648;

    /// The fake callee: log the call like `fakes::fake_call`, then apply the
    /// forced result of this call index unless the call unwound.
    fn oc926_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let n = c.bus.read32(fakes::LOG_COUNT);
        fakes::fake_call(c, addr);
        if c.pc == ra {
            let word = c.bus.read32(OC926_RES + 4 * (n & 0x3ff));
            if word != 0 {
                c.g[V0][0] = word.wrapping_sub(1) as u64;
            }
        }
    }

    fn oc926_miss(c: &mut Cpu, target: u32) {
        oc926_call(c, target);
    }

    /// Gated fakes of the callees in their override slots.
    macro_rules! oc926_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC926_CTRL) != OC926_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc926_call(c, $addr);
                }
            )*
            fn oc926_install() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc926_fakes!(
        (
            oc926_fake_001c2468,
            OC926_PREV_1C2468,
            ee::s03::OVERRIDE_001C2468,
            ee::s03::f_001c2468__generated,
            0x001c_2468
        ),
        (
            oc926_fake_0014dc80,
            OC926_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc926_fake_0014dd58,
            OC926_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc926_fake_00416810,
            OC926_PREV_416810,
            ee::s15::OVERRIDE_00416810,
            ee::s15::f_00416810__generated,
            0x0041_6810
        ),
        (
            oc926_fake_004166f4,
            OC926_PREV_4166F4,
            ee::s15::OVERRIDE_004166F4,
            ee::s15::f_004166f4__generated,
            0x0041_66f4
        ),
        (
            oc926_fake_001a9c50,
            OC926_PREV_1A9C50,
            ee::s03::OVERRIDE_001A9C50,
            ee::s03::f_001a9c50__generated,
            0x001a_9c50
        ),
        (
            oc926_fake_00317670,
            OC926_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc926_fake_001c57c0,
            OC926_PREV_1C57C0,
            ee::s03::OVERRIDE_001C57C0,
            ee::s03::f_001c57c0__generated,
            0x001c_57c0
        ),
        (
            oc926_fake_001a9bc0,
            OC926_PREV_1A9BC0,
            ee::s03::OVERRIDE_001A9BC0,
            ee::s03::f_001a9bc0__generated,
            0x001a_9bc0
        ),
        (
            oc926_fake_0014f6e8,
            OC926_PREV_14F6E8,
            ee::s01::OVERRIDE_0014F6E8,
            ee::s01::f_0014f6e8__generated,
            0x0014_f6e8
        ),
        (
            oc926_fake_0039d860,
            OC926_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc926_fake_0039a738,
            OC926_PREV_39A738,
            ee::s13::OVERRIDE_0039A738,
            ee::s13::f_0039a738__generated,
            0x0039_a738
        ),
        (
            oc926_fake_00144c60,
            OC926_PREV_144C60,
            ee::s01::OVERRIDE_00144C60,
            ee::s01::f_00144c60__generated,
            0x0014_4c60
        ),
        (
            oc926_fake_0039a3f8,
            OC926_PREV_39A3F8,
            ee::s13::OVERRIDE_0039A3F8,
            ee::s13::f_0039a3f8__generated,
            0x0039_a3f8
        ),
        (
            oc926_fake_001c4b78,
            OC926_PREV_1C4B78,
            ee::s03::OVERRIDE_001C4B78,
            ee::s03::f_001c4b78__generated,
            0x001c_4b78
        ),
        (
            oc926_fake_0039a7a8,
            OC926_PREV_39A7A8,
            ee::s13::OVERRIDE_0039A7A8,
            ee::s13::f_0039a7a8__generated,
            0x0039_a7a8
        ),
        (
            oc926_fake_0039b960,
            OC926_PREV_39B960,
            ee::s13::OVERRIDE_0039B960,
            ee::s13::f_0039b960__generated,
            0x0039_b960
        ),
        (
            oc926_fake_001c3310,
            OC926_PREV_1C3310,
            ee::s03::OVERRIDE_001C3310,
            ee::s03::f_001c3310__generated,
            0x001c_3310
        ),
        (
            oc926_fake_0039a670,
            OC926_PREV_39A670,
            ee::s13::OVERRIDE_0039A670,
            ee::s13::f_0039a670__generated,
            0x0039_a670
        ),
        (
            oc926_fake_0039e508,
            OC926_PREV_39E508,
            ee::s13::OVERRIDE_0039E508,
            ee::s13::f_0039e508__generated,
            0x0039_e508
        ),
        (
            oc926_fake_001c4e40,
            OC926_PREV_1C4E40,
            ee::s03::OVERRIDE_001C4E40,
            ee::s03::f_001c4e40__generated,
            0x001c_4e40
        ),
        (
            oc926_fake_001c4d28,
            OC926_PREV_1C4D28,
            ee::s03::OVERRIDE_001C4D28,
            ee::s03::f_001c4d28__generated,
            0x001c_4d28
        ),
    );

    /// Scrambled machine with the runtime and the fakes armed, `pc` at the
    /// function's entry.
    fn oc926_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32, entry: u32) -> Rng {
        oc926_install();
        let r = fakes::machine_rt(c, seed, salt, calls);
        c.hooks.miss = oc926_miss;
        c.bus.write32(OC926_CTRL, OC926_MAGIC);
        c.pc = entry;
        r
    }

    /// Force the result of fake call number `n` (`word = result + 1`).
    fn oc926_force(c: &mut Cpu, n: u32, result: u32) {
        c.bus.write32(OC926_RES + 4 * n, result.wrapping_add(1));
    }

    /// An object with its vtable word at `obj + vt_off` and, for each slot
    /// offset, a random `this` adjustment and a fake method address.
    fn oc926_object(c: &mut Cpu, r: &mut Rng, obj: u32, vt: u32, vt_off: u32, slots: &[u32]) {
        c.bus.write32(obj + vt_off, vt);
        for (i, &off) in slots.iter().enumerate() {
            c.bus.write16(vt + off, (r.next_u32() & 0x7c) as u16);
            c.bus.write32(vt + off + 4, fakes::FAKE + 0x10 * i as u32);
        }
    }

    /// `RefreshAll`: ten `RefreshMessage` calls, some unwinding.
    #[test]
    fn c_fe_state_net_quick_messages_refresh_all_matches() {
        fakes::covered(
            ee::s03::f_001c2418__generated,
            c_fe_state_net_quick_messages_refresh_all,
            60,
            &|c, seed| {
                let _ = oc926_machine(c, seed, 0x926_2418, 12, 0x001c_2418);
                c.g[A0][0] = sx(OC926_SELF);
            },
        );
    }

    /// `StoreEditedMessage`: slot index at or above 10 (nothing), below 10
    /// with the slot already filled (`0x416810` non-zero) and empty (popup
    /// built), fakes that unwind anywhere.
    #[test]
    fn c_fe_state_net_quick_messages_store_edited_message_matches() {
        fakes::covered(
            ee::s03::f_001c2a18__generated,
            c_fe_state_net_quick_messages_store_edited_message,
            120,
            &|c, seed| {
                let mut r = oc926_machine(c, seed, 0x926_2a18, 12, 0x001c_2a18);
                let variant = seed % 4;
                let index = if variant == 0 {
                    10 + seed % 7
                } else {
                    seed % 10
                };
                c.bus.write32(OC926_SELF + 0x69c, OC926_OBJ);
                c.bus.write32(OC926_OBJ + 0x18, index);
                // call 1: BELibrary_GetInterface -> the editor object (vtable at +0xc)
                oc926_force(c, 1, OC926_OBJ + 0x1000);
                oc926_object(c, &mut r, OC926_OBJ + 0x1000, OC926_VT, 0xc, &[8]);
                // call 2: the slot already holds a message?
                oc926_force(c, 2, (variant % 2 == 1) as u32);
                // call 3: GetPopup (an object the function writes into)
                oc926_force(c, 3, OC926_OBJ + 0x2000);
                c.g[A0][0] = sx(OC926_SELF);
            },
        );
    }

    /// `UpdateScrollBar`: no screen, `pos < 0`, `count < pos`, `count <= 0`,
    /// count 1 and larger, and the track / thumb widgets found or missing.
    #[test]
    fn c_fe_state_stats_update_scroll_bar_matches() {
        const ARGS: [(u32, u32); 7] = [
            (u32::MAX, 5),
            (6, 5),
            (0, 0),
            (0, 1),
            (1, 1),
            (2, 5),
            (4, 8),
        ];
        const FLOATS: [u32; 5] = [
            0x0000_0000,
            0x3fc0_0000,
            0xc050_0000,
            0x42c8_0000,
            0x4930_d400,
        ];
        fakes::covered(
            ee::s03::f_001c3d68__generated,
            c_fe_state_stats_update_scroll_bar,
            168,
            &|c, seed| {
                let mut r = oc926_machine(c, seed, 0x926_3d68, 8, 0x001c_3d68);
                let (pos, count) = ARGS[seed as usize % ARGS.len()];
                let variant = seed / 7;
                c.bus.write32(
                    OC926_SELF + 0x40,
                    if variant % 8 == 7 { 0 } else { OC926_ZERO },
                );
                let track = OC926_OBJ;
                let thumb = OC926_OBJ + 0x1000;
                // call 1: the track, call 3: the thumb
                oc926_force(c, 1, if variant % 2 == 0 { track } else { 0 });
                oc926_force(c, 3, if variant / 2 % 2 == 0 { thumb } else { 0 });
                oc926_object(c, &mut r, track, OC926_VT, 8, &[0xa0, 0x48]);
                oc926_object(c, &mut r, thumb, OC926_VT + 0x400, 8, &[0x48]);
                for off in [0x44u32, 0x48, 0x4c] {
                    let f = FLOATS[r.next_u32() as usize % FLOATS.len()];
                    c.bus.write32(track + off, f);
                    c.bus.write32(thumb + off, r.next_u32());
                }
                // the float the track method leaves at sp+4 of the frame
                let f = FLOATS[r.next_u32() as usize % FLOATS.len()];
                c.bus.write32(crate::difftest::SP - 0x80 + 4, f);
                c.g[A0][0] = sx(OC926_SELF);
                c.g[A1][0] = sx(pos);
                c.g[A2][0] = sx(count);
            },
        );
    }

    /// `FillCourseList`: no list box, no second box, and the 29-entry table
    /// with matching / non-matching entries (first match kept, valid or
    /// invalid course id).
    #[test]
    fn c_fe_state_stats_fill_course_list_matches() {
        fakes::covered(
            ee::s03::f_001c49c8__generated,
            c_fe_state_stats_fill_course_list,
            220,
            &|c, seed| {
                let mut r = oc926_machine(c, seed, 0x926_49c8, 20, 0x001c_49c8);
                c.bus.write32(OC926_SELF + 0x40, OC926_ZERO);
                c.bus.write32(OC926_SELF + 0x6e0, 0x66);
                let widget = OC926_OBJ;
                // calls 1 and 6: UIScreen_FindObject; call 2: the selected value
                oc926_force(c, 1, if seed % 5 == 0 { 0 } else { widget });
                oc926_force(c, 2, 0x55);
                oc926_force(c, 6, if seed % 5 == 1 { 0 } else { widget });
                let none = seed % 11 == 0;
                for i in 0..29u32 {
                    let e = OC926_COURSES + 0x10 * i;
                    let id = (r.next_u32() % 45) as i32 - 8;
                    c.bus.write32(e, id as u32);
                    c.bus.write32(e + 4, r.next_u32());
                    let hit = !none && r.next_u32() % 3 == 0;
                    c.bus.write32(e + 8, if hit { 0x55 } else { 0x54 });
                    c.bus.write32(
                        e + 12,
                        if hit || r.next_u32() % 2 == 0 {
                            0x66
                        } else {
                            0x67
                        },
                    );
                }
                c.g[A0][0] = sx(OC926_SELF);
            },
        );
    }

    /// `PostInput`: every combination of the four button queries (per-call
    /// results from the seed bits) with `mode` below and above 3.
    #[test]
    fn c_fe_state_stats_post_input_matches() {
        fakes::covered(
            ee::s03::f_001c4bd8__generated,
            c_fe_state_stats_post_input,
            512,
            &|c, seed| {
                let mut r = oc926_machine(c, seed, 0x926_4bd8, 10, 0x001c_4bd8);
                let input = OC926_OBJ;
                oc926_object(c, &mut r, input, OC926_VT, 8, &[0x88, 0x90, 0xa0, 0x98]);
                for n in 0..8u32 {
                    // calls 1..8 are the queries (call 0 is the base input)
                    c.bus.write32(
                        OC926_RES + 4 * (n + 1),
                        if seed >> n & 1 == 1 { 2 } else { 1 },
                    );
                }
                c.bus
                    .write32(OC926_SELF + 0x6e4, if seed >> 8 & 1 == 1 { 2 } else { 5 });
                c.g[A0][0] = sx(OC926_SELF);
                c.g[A1][0] = sx(input);
            },
        );
    }
}
