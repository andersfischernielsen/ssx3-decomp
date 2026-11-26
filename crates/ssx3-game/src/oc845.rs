//! Staging module oc845 (wave wave249): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc845`.

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
        0x0018_f168,
        oc845_profile_load_v39,
        "cFEStateProfileLoad_v39",
    ),
    (
        0x0018_f9d8,
        oc845_profile_load_refresh_slots,
        "cFEStateProfileLoad_RefreshSlots",
    ),
    (
        0x0018_fc70,
        oc845_profile_load_shutdown,
        "cFEStateProfileLoad_Shutdown",
    ),
    (
        0x0018_fc98,
        oc845_profile_load_begin_card_op,
        "cFEStateProfileLoad_BeginCardOp",
    ),
    (
        0x0019_02c0,
        oc845_profile_load_setup_titles,
        "cFEStateProfileLoad_SetupTitles",
    ),
    (
        0x0019_0540,
        oc845_profile_load_show_new_profile_title,
        "cFEStateProfileLoad_ShowNewProfileTitle",
    ),
];

/// `cFEStateProfileLoad_RefreshProfileList` (0x18f2b8), not yet replaced.
const OC845_REFRESH_PROFILE_LIST: Callee = Callee {
    addr: 0x0018_f2b8,
    f: ee::s02::f_0018f2b8,
};

/// A string in the `0x46xxxx` rodata block: `lui 0x46; addiu -off`.
#[inline(always)]
fn oc845_str46(off: u32) -> u32 {
    0x0046_0000u32.wrapping_sub(off)
}

/// A string in the `0x4axxxx` rodata block: `lui 0x4a; addiu off`.
#[inline(always)]
fn oc845_str4a(off: u32) -> u32 {
    0x004a_0000u32.wrapping_add(off)
}

/// Find a widget of the state's screen (`*(this + 0x40)`, `this` in register
/// `this`) by string name: hash the name (`0x317670`), look the hash up in the
/// screen (`0x39d860`, `cUIScreen_getObjectByHashName`). The widget is in `v0`.
fn oc845_find(c: &mut Cpu, name: u32, this: usize, ret_hash: u32, ret_find: u32) -> Flow {
    c.g[A0][0] = sx(name);
    call_ptr(c, 0x0031_7670, ret_hash)?;
    c.g[A0][0] = c.lw((c.g[this][0] as u32).wrapping_add(0x40));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, ret_find)
}

/// Call the virtual method at `{delta: i16, fn}` `off` of the widget in `v0`,
/// loading through `v1`/`a2` (the compiled form that keeps the widget in `v0`).
fn oc845_vcall_v0(c: &mut Cpu, a1: u64, off: u32, ret_to: u32) -> Flow {
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(8));
    c.g[A1][0] = a1;
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(off));
    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(off + 4));
    let target = c.g[A2][0] as u32;
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret_to)
}

/// Call the virtual method at `off` of the widget held in register `obj`,
/// loading through `v0`/`v1` (the compiled form that keeps the widget in a
/// saved register).
fn oc845_vcall_via(c: &mut Cpu, obj: usize, a1: u64, off: u32, ret_to: u32) -> Flow {
    c.g[V0][0] = c.lw((c.g[obj][0] as u32).wrapping_add(8));
    c.g[A1][0] = a1;
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(off));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(off + 4));
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[obj][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret_to)
}

/// `void cFEStateProfileLoad_v39(this, int, int on)` (0x18f168): select the
/// two option widgets (`on ^ 1` to one, set on the other) and remember the
/// selection at `this+0x224`; with the profile mode (`gp-0x1b08`) equal to 2
/// the first widget is skipped and the selection comes from `a1`.
pub fn oc845_profile_load_v39(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[A0][0] = 0x4a0000;
        frame.save(c, S2, 0x20);
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        oc845_find(c, oc845_str4a(0x15e0), S1, 0x0018_f198, 0x0018_f1a4)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1b08));
        c.g[V0][0] = 0x2;
        let mode_is_2 = c.g[V1][0] == c.g[V0][0];
        c.g[S0][0] ^= 0x1;
        if !mode_is_2 {
            c.cycles += 21;
            oc845_find(c, oc845_str4a(0x15d0), S1, 0x0018_f1c4, 0x0018_f1d0)?;
            let a1 = c.g[S0][0];
            oc845_vcall_v0(c, a1, 0x48, 0x0018_f1e8)?;
        } else {
            c.cycles += 21;
            c.g[A0][0] = 0x4a0000;
            c.g[S0][0] = c.g[S3][0] ^ 0x1;
            oc845_find(c, oc845_str4a(0x15d8), S1, 0x0018_f21c, 0x0018_f228)?;
            let a1 = c.g[S0][0];
            oc845_vcall_v0(c, a1, 0x48, 0x0018_f240)?;
        }
        c.sw((c.g[S1][0] as u32).wrapping_add(0x224), c.g[S0][0]);
        let a1 = c.g[S0][0];
        let ret_to = if !mode_is_2 { 0x0018_f204 } else { 0x0018_f25c };
        oc845_vcall_via(c, S2, a1, 0x48, ret_to)?;
        frame.restore(c, S0, 0x40);
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateProfileLoad_RefreshSlots(this)` (0x18f9d8): ask the
/// memory-card manager and the screen whether the card is usable; when it is
/// not (a check answers 0), relabel the six slot widgets ("slot N": text off,
/// image on); when both answer yes, refresh the profile list instead.
pub fn oc845_profile_load_refresh_slots(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S2, 0x60);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x80);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        frame.save(c, S1, 0x70);
        frame.save(c, S3, 0x50);
        frame.save_ra(c, 0x40);
        call_ptr(c, 0x0022_7f80, 0x0018_fa00)?;
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x150));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x154));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0018_fa18)?;
        let first_zero = c.g[V0][0] == 0;
        c.g[S1][0] = 0x1;
        // Both checks passed: refresh the profile list and leave.
        let mut refreshed = false;
        if !first_zero {
            c.cycles += 8;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
            c.g[V1][0] = c.lw(c.g[A1][0] as u32);
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1a8));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1ac));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0018_fa38)?;
            let second_zero = c.g[V0][0] == 0;
            c.g[S1][0] = 0x1;
            if !second_zero {
                c.cycles += 10;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = 0x3;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x118));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x11c));
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0018_fa58)?;
                c.g[A0][0] = c.g[S2][0];
                OC845_REFRESH_PROFILE_LIST.call(c, 0x0018_fa60)?;
                frame.restore(c, S0, 0x80);
                refreshed = true;
            }
        }
        if !refreshed {
            c.cycles += 2;
            c.g[S3][0] = 0x460000;
            c.g[A0][0] = c.g[SP][0];
            loop {
                c.cycles += 25;
                c.g[A2][0] = c.g[S1][0];
                c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1da0));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                call_ptr(c, 0x0041_7828, 0x0018_fa80)?;
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x0031_7670, 0x0018_fa88)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x0018_fa94)?;
                c.g[S0][0] = c.g[V0][0];
                oc845_vcall_via(c, S0, 0, 0xc8, 0x0018_fab0)?;
                oc845_vcall_via(c, S0, 1, 0x40, 0x0018_fac8)?;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 7i64) as u64;
                if c.g[V0][0] != 0 {
                    c.g[A0][0] = c.g[SP][0];
                    if event_due(c) {
                        deliver_event_here(c, 0x0018_fa70);
                    }
                    continue;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x80);
                break;
            }
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateProfileLoad_Shutdown(this)` (0x18fc70, `onDestroyScreen`):
/// shut the memory-card state base down and clear the "card op pending" word
/// at `this+0x1a4`.
pub fn oc845_profile_load_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x001d_58b8, 0x0018_fc84)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1a4), 0u64);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateProfileLoad_BeginCardOp(this)` (0x18fc98): when a card
/// operation is requested (`this+0x224`), start the manager's save
/// (`MCManager_BeginSave(mgr, 1, this+0x1cc)`), mark the state busy
/// (`+0x1a8`, `+0x1c0 = 3`, `+0x1dc`) and, the first time (`+0x210`, `+0x204`
/// both 0), open the status popup: a 0x640-byte buffer filled by
/// `MCManager_GetStatusText` and handed to the screen's vtable `+0x120`.
pub fn oc845_profile_load_begin_card_op(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x224));
        'epilogue: {
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x30);
                break 'epilogue;
            }
            'opened: {
                c.cycles += 13;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[S2][0] = 0x1;
                call_ptr(c, 0x0022_7f80, 0x0018_fcc8)?;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1cc));
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0023_cae8, 0x0018_fcd8)?;
                c.g[V0][0] = 0x3;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x210));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c0), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1a8), c.g[S2][0]);
                let popup_open = c.g[V1][0] != 0;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1dc), c.g[S2][0]);
                if popup_open {
                    break 'opened;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x204));
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x30);
                    break 'epilogue;
                }
                c.cycles += 28;
                c.g[V0][0] = 0x16;
                c.g[A3][0] = 0u64;
                c.g[A1][0] = sx(oc845_str46(0x1d60));
                c.g[A2][0] = 0x100;
                c.g[A0][0] = 0x640;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x19c), c.g[V0][0]);
                call_ptr(c, 0x0031_7e30, 0x0018_fd1c)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[S1][0] = c.g[V0][0];
                call_ptr(c, 0x0022_7f80, 0x0018_fd28)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = 0x320;
                call_ptr(c, 0x0024_1e18, 0x0018_fd38)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = 0x1;
                c.g[A3][0] = 0x1;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x120));
                c.g[T0][0] = 0u64;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
                c.g[T1][0] = 0u64;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                let target = c.g[V1][0] as u32;
                c.g[T2][0] = 0x1;
                call_ptr(c, target, 0x0018_fd64)?;
                let no_buffer = c.g[S1][0] == 0;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x210), c.g[S2][0]);
                if !no_buffer {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0031_7e98, 0x0018_fd74)?;
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateProfileLoad_SetupTitles(this)` (0x1902c0): the screen's
/// initial widget state: hide six title/slot widgets (`vtable+0x48(0)`), then
/// for slots 1..6 set text slot N from the format string and tag the widget
/// with `N-1` (`+0x18`), then show the two buttons (`vtable+0x38`).
pub fn oc845_profile_load_setup_titles(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 102;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        c.g[A0][0] = 0x460000;
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[A0][0] = sx(oc845_str46(0x1ce8));
        frame.save_ra(c, 0x10);
        c.g[S1][0] = 0x1;
        call_ptr(c, 0x0031_7670, 0x0019_02ec)?;
        c.g[S3][0] = 0x4a0000;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = c.g[V0][0];
        c.g[S2][0] = 0x460000;
        call_ptr(c, 0x0039_d860, 0x0019_0300)?;
        c.g[A2][0] = c.g[V0][0];
        oc845_vcall_via(c, A2, 0, 0x48, 0x0019_031c)?;
        // The other five widgets: (name, hash return, find return, call return).
        let hides: [(u32, u32, u32, u32); 5] = [
            (oc845_str46(0x1cd0), 0x0019_0328, 0x0019_0334, 0x0019_0350),
            (oc845_str46(0x1cb8), 0x0019_035c, 0x0019_0368, 0x0019_0384),
            (oc845_str46(0x1ca8), 0x0019_0390, 0x0019_039c, 0x0019_03b8),
            (oc845_str4a(0x15d0), 0x0019_03c4, 0x0019_03d0, 0x0019_03ec),
            (oc845_str4a(0x15d8), 0x0019_03f8, 0x0019_0404, 0x0019_0420),
        ];
        for (name, ret_hash, ret_find, ret_call) in hides {
            oc845_find(c, name, S0, ret_hash, ret_find)?;
            c.g[A2][0] = c.g[V0][0];
            oc845_vcall_via(c, A2, 0, 0x48, ret_call)?;
        }
        oc845_find(c, oc845_str4a(0x1458), S0, 0x0019_042c, 0x0019_0438)?;
        c.g[A2][0] = c.g[V0][0];
        oc845_vcall_via(c, A2, 0, 0x48, 0x0019_0454)?;
        c.g[A0][0] = c.g[SP][0];
        loop {
            c.cycles += 26;
            c.g[A2][0] = c.g[S1][0];
            c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x12c0));
            call_ptr(c, 0x0041_7828, 0x0019_0464)?;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0031_7670, 0x0019_046c)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0019_0478)?;
            c.g[A1][0] = c.g[SP][0];
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x003a_0c50, 0x0019_0484)?;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1da0));
            call_ptr(c, 0x0041_7828, 0x0019_0494)?;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0031_7670, 0x0019_049c)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0019_04a8)?;
            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
            c.g[V0][0] = ((c.g[S1][0] as i64) < 7i64) as u64;
            if c.g[V0][0] != 0 {
                c.g[A0][0] = c.g[SP][0];
                if event_due(c) {
                    deliver_event_here(c, 0x0019_0458);
                }
                continue;
            }
            break;
        }
        c.cycles += 31;
        oc845_find(c, oc845_str4a(0x1600), S0, 0x0019_04cc, 0x0019_04d8)?;
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = 0u64;
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
        let target = c.g[A2][0] as u32;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0019_04f0)?;
        oc845_find(c, oc845_str4a(0x1608), S0, 0x0019_04fc, 0x0019_0508)?;
        oc845_vcall_v0(c, 1, 0x38, 0x0019_0520)?;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateProfileLoad_ShowNewProfileTitle(this)` (0x190540): show the
/// "new profile" title state: enable the first widget, set two text widgets
/// to their text ids (`0x3a1000`), enable the next widget and the last two
/// buttons, then mark the state (`+0x220`, `+0x224` = 1).
pub fn oc845_profile_load_show_new_profile_title(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 75;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x460000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(oc845_str46(0x1ce8));
        call_ptr(c, 0x0031_7670, 0x0019_0560)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0019_056c)?;
        c.g[S0][0] = c.g[V0][0];
        oc845_vcall_via(c, S0, 1, 0x48, 0x0019_0588)?;
        c.g[A0][0] = sx(oc845_str46(0x1c98));
        call_ptr(c, 0x0031_7670, 0x0019_0594)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x003a_1000, 0x0019_05a0)?;
        c.g[A0][0] = sx(oc845_str46(0x1cb8));
        call_ptr(c, 0x0031_7670, 0x0019_05ac)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0019_05b8)?;
        c.g[S0][0] = c.g[V0][0];
        oc845_vcall_via(c, S0, 1, 0x48, 0x0019_05d4)?;
        c.g[A0][0] = sx(oc845_str46(0x1c88));
        call_ptr(c, 0x0031_7670, 0x0019_05e0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x003a_1000, 0x0019_05ec)?;
        oc845_find(c, oc845_str4a(0x15d0), S1, 0x0019_05f8, 0x0019_0604)?;
        oc845_vcall_v0(c, 1, 0x48, 0x0019_061c)?;
        oc845_find(c, oc845_str4a(0x1458), S1, 0x0019_0628, 0x0019_0634)?;
        oc845_vcall_v0(c, 1, 0x48, 0x0019_064c)?;
        c.g[V0][0] = 0x1;
        frame.restore(c, S0, 0x20);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x224), c.g[V0][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x220), c.g[V0][0]);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Gate word (the fakes only act when it holds the magic) and the
    /// "return 0" mailboxes (one word per fake address).
    const OC845_CTRL: u32 = DATA + 0x20_0000;
    const OC845_MAGIC: u32 = 0x4f43_8450;
    const OC845_ZERO: u32 = DATA + 0x20_1000;

    /// The state object, a widget, a shared vtable and a screen.
    const OC845_THIS: u32 = DATA + 0x8_0000;
    const OC845_NODE: u32 = DATA + 0x9_0000;
    const OC845_VT: u32 = DATA + 0xa_0000;
    const OC845_SCREEN: u32 = DATA + 0xb_0000;
    const OC845_MGR: u32 = DATA + 0xc_0000;
    const OC845_OBJ: u32 = DATA + 0xd_0000;
    const OC845_VT2: u32 = DATA + 0xe_0000;
    /// A real leaf that returns 0.
    const OC845_RET_ZERO: u32 = 0x001a_8828;

    /// The fake callee: logs the call like `fakes::fake_call`; a mailbox set to
    /// 1 makes it return 0.
    fn oc845_fake_call(c: &mut Cpu, addr: u32) {
        let zero = c.bus.read32(OC845_ZERO + (addr & 0xffc)) == 1;
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if zero && c.pc == ra {
            c.g[V0][0] = 0;
        }
    }

    macro_rules! oc845_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC845_CTRL) != OC845_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc845_fake_call(c, $addr);
                }
            )*
            fn oc845_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc845_fakes!(
        (
            oc845_fake_317670,
            OC845_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc845_fake_39d860,
            OC845_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc845_fake_227f80,
            OC845_PREV_227F80,
            ee::s05::OVERRIDE_00227F80,
            ee::s05::f_00227f80__generated,
            0x0022_7f80
        ),
        (
            oc845_fake_417828,
            OC845_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
        (
            oc845_fake_3a0c50,
            OC845_PREV_3A0C50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a_0c50
        ),
        (
            oc845_fake_3a1000,
            OC845_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
        (
            oc845_fake_23cae8,
            OC845_PREV_23CAE8,
            ee::s06::OVERRIDE_0023CAE8,
            ee::s06::f_0023cae8__generated,
            0x0023_cae8
        ),
        (
            oc845_fake_317e30,
            OC845_PREV_317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc845_fake_241e18,
            OC845_PREV_241E18,
            ee::s06::OVERRIDE_00241E18,
            ee::s06::f_00241e18__generated,
            0x0024_1e18
        ),
        (
            oc845_fake_317e98,
            OC845_PREV_317E98,
            ee::s10::OVERRIDE_00317E98,
            ee::s10::f_00317e98__generated,
            0x0031_7e98
        ),
        (
            oc845_fake_1d58b8,
            OC845_PREV_1D58B8,
            ee::s04::OVERRIDE_001D58B8,
            ee::s04::f_001d58b8__generated,
            0x001d_58b8
        ),
    );

    /// A vtable whose every `{delta 0, fn}` slot (8-byte stride) calls a fake.
    fn oc845_vtable(c: &mut Cpu, vt: u32) {
        for off in (0..0x200u32).step_by(8) {
            c.bus.write16(vt + off, 0);
            c.bus.write32(vt + off + 4, FAKE + 0x10 * ((off / 8) % 8));
        }
    }

    /// The machine with the gate set and the shared objects: the state at
    /// `OC845_THIS` (screen at `+0x40`, vtable at `+8`), and a widget whose
    /// vtable calls fakes; the first fake results are that widget.
    fn oc845_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc845_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC845_CTRL, OC845_MAGIC);
        oc845_vtable(c, OC845_VT);
        c.bus.write32(OC845_THIS + 8, OC845_VT);
        c.bus.write32(OC845_THIS + 0x40, OC845_SCREEN);
        c.bus.write32(OC845_NODE + 8, OC845_VT);
        c.g[GP][0] = GAME_GP as u64;
        r
    }

    /// The first eight fake calls return the widget.
    fn oc845_results_node(c: &mut Cpu) {
        for n in 0..8u32 {
            c.bus.write32(LOG_RESULTS + 4 * n, OC845_NODE);
        }
    }

    #[test]
    fn oc845_profile_load_v39_matches() {
        check(
            ee::s02::f_0018f168__generated,
            oc845_profile_load_v39,
            120,
            &|c, seed| {
                let mut r = oc845_machine(c, seed, 0x18_f168, 10);
                if seed % 3 != 0 {
                    oc845_results_node(c);
                }
                c.bus.write32(
                    GAME_GP.wrapping_sub(0x1b08),
                    if seed % 2 == 0 { 2 } else { r.next_u32() & 3 },
                );
                c.bus.write32(OC845_THIS + 0x224, r.next_u32());
                c.g[A0][0] = sx(OC845_THIS);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.pc = 0x0018_f168;
            },
        );
    }

    #[test]
    fn oc845_profile_load_refresh_slots_matches() {
        check(
            ee::s02::f_0018f9d8__generated,
            oc845_profile_load_refresh_slots,
            160,
            &|c, seed| {
                let mut r = oc845_machine(c, seed, 0x18_f9d8, 45);
                // Call 0 (the manager) returns the manager object.
                c.bus.write32(LOG_RESULTS, OC845_MGR);
                c.bus.write32(OC845_MGR + 0x434, OC845_OBJ);
                c.bus.write32(OC845_OBJ, OC845_VT2);
                oc845_vtable(c, OC845_VT2);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0xf_0000);
                // 0: the screen check answers 0; 1: the card check answers 0;
                // 2, 3: both answer yes and the profile list is refreshed.
                let case = seed % 4;
                if case == 0 {
                    c.bus.write32(OC845_VT + 0x154, OC845_RET_ZERO);
                }
                if case == 1 {
                    c.bus.write32(OC845_VT2 + 0x1ac, OC845_RET_ZERO);
                }
                c.g[A0][0] = sx(OC845_THIS);
                c.g[S1][0] = r.next_u32() as u64;
                c.pc = 0x0018_f9d8;
            },
        );
    }

    #[test]
    fn oc845_profile_load_shutdown_matches() {
        check(
            ee::s02::f_0018fc70__generated,
            oc845_profile_load_shutdown,
            40,
            &|c, seed| {
                let mut r = oc845_machine(c, seed, 0x18_fc70, 1);
                c.bus.write32(OC845_THIS + 0x1a4, r.next_u32());
                c.g[A0][0] = sx(OC845_THIS);
                c.pc = 0x0018_fc70;
            },
        );
    }

    #[test]
    fn oc845_profile_load_begin_card_op_matches() {
        check(
            ee::s02::f_0018fc98__generated,
            oc845_profile_load_begin_card_op,
            160,
            &|c, seed| {
                let mut r = oc845_machine(c, seed, 0x18_fc98, 7);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0xf_0000);
                // 0: no card op requested; 1: the popup is already open
                // (+0x210); 2: first time but +0x204 set; 3: no buffer
                // (allocator returns 0); 4..7: popup built and the buffer freed.
                let case = seed % 8;
                c.bus.write32(
                    OC845_THIS + 0x224,
                    if case == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(
                    OC845_THIS + 0x210,
                    if case == 1 { r.next_u32() | 1 } else { 0 },
                );
                c.bus.write32(
                    OC845_THIS + 0x204,
                    if case == 2 { r.next_u32() | 1 } else { 0 },
                );
                c.bus.write32(OC845_THIS + 0x1cc, r.next_u32());
                if case == 3 {
                    c.bus.write32(OC845_ZERO + (0x0031_7e30 & 0xffc), 1);
                }
                c.g[A0][0] = sx(OC845_THIS);
                c.pc = 0x0018_fc98;
            },
        );
    }

    #[test]
    fn oc845_profile_load_setup_titles_matches() {
        check(
            ee::s02::f_001902c0__generated,
            oc845_profile_load_setup_titles,
            100,
            &|c, seed| {
                let _r = oc845_machine(c, seed, 0x19_02c0, 70);
                if seed % 3 != 0 {
                    oc845_results_node(c);
                }
                c.g[A0][0] = sx(OC845_THIS);
                c.pc = 0x0019_02c0;
            },
        );
    }

    #[test]
    fn oc845_profile_load_show_new_profile_title_matches() {
        check(
            ee::s02::f_00190540__generated,
            oc845_profile_load_show_new_profile_title,
            80,
            &|c, seed| {
                let _r = oc845_machine(c, seed, 0x19_0540, 22);
                if seed % 3 != 0 {
                    oc845_results_node(c);
                }
                c.g[A0][0] = sx(OC845_THIS);
                c.pc = 0x0019_0540;
            },
        );
    }
}
