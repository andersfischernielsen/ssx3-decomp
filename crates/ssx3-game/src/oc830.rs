//! Staging module oc830 (wave wave249): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc830`.

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
        0x0018_5a70,
        cfe_options_credits_dtor,
        "cFEOptionsCredits_dtor",
    ),
    (
        0x0018_5a98,
        cfe_options_credits_init,
        "cFEOptionsCredits_Init",
    ),
    (
        0x0018_5ba0,
        cfe_options_credits_on_activate,
        "cFEOptionsCredits_OnActivate",
    ),
    (
        0x0018_5f40,
        cfe_options_credits_render,
        "cFEOptionsCredits_Render",
    ),
    (
        0x0018_6d60,
        cfe_state_mem_card_show_not_detected_msg,
        "cFEStateMemCard_ShowNotDetectedMsg",
    ),
    (
        0x0018_7360,
        cfe_state_mem_card_show_no_space_msg,
        "cFEStateMemCard_ShowNoSpaceMsg",
    ),
];

/// The address `off(reg)`.
#[inline(always)]
fn oc830_ea(c: &Cpu, r: usize, off: u32) -> u32 {
    (c.g[r][0] as u32).wrapping_add(off)
}

/// `lw reg, -off(gp)`.
#[inline(always)]
fn oc830_gp_lw(c: &mut Cpu, off: u32) -> u64 {
    let a = (c.g[GP][0] as u32).wrapping_sub(off);
    c.lw(a)
}

/// `addiu`-style `sx(reg - k)`.
#[inline(always)]
fn oc830_sx_sub(c: &Cpu, r: usize, k: u32) -> u64 {
    sx((c.g[r][0] as u32).wrapping_sub(k))
}

/// `cFEOptionsCredits::~cFEOptionsCredits` (0x185a70): restore the
/// `cFEState` vtable and run the `UIState` destructor.
pub fn cfe_options_credits_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x47_0000;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = oc830_sx_sub(c, V0, 0x2f30);
        let a = oc830_ea(c, V1, 0x8);
        c.sw(a, c.g[V0][0]);
        call_ptr(c, 0x0039_e390, 0x0018_5a8c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEOptionsCredits::Init` (0x185a98): create the credits screen from the
/// UI engine, start its thread, and look up the text-scroll widgets.
pub fn cfe_options_credits_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x46_0000;
        c.g[A0][0] = oc830_sx_sub(c, A0, 0x2480);
        let a = oc830_ea(c, S1, 0x10);
        c.g[S0][0] = c.lw(a);
        call_ptr(c, 0x0031_7670, 0x0018_5abc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0018_5ad0)?;
        let a = oc830_ea(c, S1, 0x40);
        c.sw(a, c.g[V0][0]);
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0018_5ae8)?;
        }
        // Widget 1: set bit 3 of its flags word.
        c.cycles += 9;
        c.g[A0][0] = 0x46_0000;
        c.g[A0][0] = oc830_sx_sub(c, A0, 0x2470);
        call_ptr(c, 0x0031_7670, 0x0018_5af4)?;
        let a = oc830_ea(c, S1, 0x40);
        c.g[A0][0] = c.lw(a);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0018_5b00)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[A0][0] = 0x46_0000;
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            let a = oc830_ea(c, V1, 0x90);
            c.g[V0][0] = c.lw(a);
            c.g[V0][0] |= 0x8;
            c.sw(a, c.g[V0][0]);
        }
        // Widgets 2 and 3: remember them at +0x58 and call their method at
        // vtable +0x48 (the same slot both times, with a1 = 0).
        c.cycles += 8;
        c.g[A0][0] = oc830_sx_sub(c, A0, 0x2460);
        call_ptr(c, 0x0031_7670, 0x0018_5b20)?;
        let a = oc830_ea(c, S1, 0x40);
        c.g[A0][0] = c.lw(a);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0018_5b2c)?;
        c.g[A2][0] = c.g[V0][0];
        let a = oc830_ea(c, S1, 0x58);
        c.sw(a, c.g[A2][0]);
        if c.g[A2][0] != 0 {
            oc830_init_vcall(c, 6, 0x0018_5b50)?;
        }
        c.cycles += 9;
        c.g[A0][0] = 0x46_0000;
        c.g[A0][0] = oc830_sx_sub(c, A0, 0x2450);
        call_ptr(c, 0x0031_7670, 0x0018_5b5c)?;
        let a = oc830_ea(c, S1, 0x40);
        c.g[A0][0] = c.lw(a);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0018_5b68)?;
        c.g[A2][0] = c.g[V0][0];
        let a = oc830_ea(c, S1, 0x58);
        c.sw(a, c.g[A2][0]);
        if c.g[A2][0] != 0 {
            oc830_init_vcall(c, 6, 0x0018_5b8c)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Init's `widget->vtable[0x48](widget + delta, 0)` call (a2 = widget).
fn oc830_init_vcall(c: &mut Cpu, cycles: u64, ret: u32) -> Flow {
    c.cycles += cycles;
    let a = oc830_ea(c, A2, 0x8);
    c.g[V0][0] = c.lw(a);
    c.g[A1][0] = 0;
    let a = oc830_ea(c, V0, 0x48);
    c.g[A0][0] = c.lh(a);
    let a = oc830_ea(c, V0, 0x4c);
    c.g[V1][0] = c.lw(a);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx(oc830_ea(c, A2, 0).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// One loop of OnActivate's text-id setup: `sprintf(buf, fmt, i)`, hash the
/// buffer and `SetTextId(scroll, hash)` for `i` up to `limit`. `a2` and `s1`
/// hold `i` on entry; `head` is the generated loop head (return addresses
/// are `head + 0x10/0x18/0x24`).
fn oc830_credits_ids(c: &mut Cpu, head: u32, buf: usize, limit: i64) -> Flow {
    loop {
        c.cycles += 12;
        c.g[A1][0] = oc830_sx_sub(c, S4, 0x2430);
        c.g[A0][0] = c.g[buf][0];
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
        call_ptr(c, 0x0041_7828, head + 0x10)?;
        c.g[A0][0] = c.g[buf][0];
        call_ptr(c, 0x0031_7670, head + 0x18)?;
        let a = oc830_ea(c, S2, 0x5c);
        c.g[A0][0] = c.lw(a);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x003a_1f18, head + 0x24)?;
        c.g[V0][0] = ((c.g[S1][0] as i64) < limit) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A2][0] = c.g[S1][0];
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `SetTextId(scroll, StrHash(<string literal at 0x460000 - off>))`.
fn oc830_credits_set_text(c: &mut Cpu, off: u32, ra_hash: u32, ra_set: u32) -> Flow {
    c.g[A0][0] = 0x46_0000;
    c.g[A0][0] = oc830_sx_sub(c, A0, off);
    call_ptr(c, 0x0031_7670, ra_hash)?;
    let a = oc830_ea(c, S2, 0x5c);
    c.g[A0][0] = c.lw(a);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x003a_1f18, ra_set)
}

/// `cFEOptionsCredits::OnActivate` (0x185ba0): fill the credits text-scroll
/// widget with the credit lines' text ids, then run `cFEState::OnActivate`.
pub fn cfe_options_credits_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, 18, 0x80);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 22, 0x40);
        c.g[A0][0] = 0x46_0000;
        frame.save(c, 16, 0xa0);
        frame.save(c, 17, 0x90);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, 19, 0x70);
        frame.save(c, 20, 0x60);
        frame.save(c, 21, 0x50);
        frame.save_ra(c, 0x30);
        c.g[A0][0] = oc830_sx_sub(c, A0, 0x2440);
        call_ptr(c, 0x0031_7670, 0x0018_5bd8)?;
        let a = oc830_ea(c, S2, 0x40);
        c.g[A0][0] = c.lw(a);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0018_5be4)?;
        c.g[A2][0] = c.g[V0][0];
        let a = oc830_ea(c, S2, 0x5c);
        c.sw(a, c.g[A2][0]);
        if c.g[A2][0] != 0 {
            c.cycles += 28;
            let a = oc830_ea(c, A2, 0x8);
            c.g[V1][0] = c.lw(a);
            c.g[A1][0] = c.g[SP][0];
            c.g[S1][0] = 0;
            c.g[S4][0] = 0x46_0000;
            let a = oc830_ea(c, V1, 0xa0);
            c.g[A0][0] = c.lh(a);
            let a = oc830_ea(c, V1, 0xa4);
            c.g[V0][0] = c.lw(a);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx(oc830_ea(c, A2, 0).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0018_5c10)?;
            c.g[AT][0] = 0x4416_0000;
            c.mtc1(1, 0);
            c.g[A1][0] = c.g[SP][0];
            let a = oc830_ea(c, S2, 0x5c);
            c.g[A2][0] = c.lw(a);
            let sp0 = c.g[SP][0] as u32;
            c.swc1(0, sp0);
            let a = oc830_ea(c, A2, 0x8);
            c.g[V0][0] = c.lw(a);
            let a = oc830_ea(c, V0, 0xa8);
            c.g[A0][0] = c.lh(a);
            let a = oc830_ea(c, V0, 0xac);
            c.g[V1][0] = c.lw(a);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx(oc830_ea(c, A2, 0).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0018_5c38)?;
            let a = oc830_ea(c, S2, 0x5c);
            c.g[A2][0] = c.lw(a);
            c.g[A1][0] = 0;
            oc830_on_activate_vcall(c, 0x0018_5c54)?;
            c.g[S3][0] = sx(oc830_ea(c, SP, 0x10));
            c.g[S0][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S1][0];
            oc830_credits_ids(c, 0x0018_5c60, S0, 7)?;
            c.cycles += 16;
            c.g[S1][0] = 0x7;
            c.g[S0][0] = c.g[S3][0];
            oc830_credits_set_text(c, 0x2420, 0x0018_5ca4, 0x0018_5cb0)?;
            oc830_credits_set_text(c, 0x2410, 0x0018_5cbc, 0x0018_5cc8)?;
            c.g[A2][0] = c.g[S1][0];
            oc830_credits_ids(c, 0x0018_5cd0, S0, 47)?;
            c.cycles += 4;
            c.g[S1][0] = 0x31;
            c.g[S0][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S1][0];
            oc830_credits_ids(c, 0x0018_5d10, S0, 75)?;
            c.cycles += 10;
            c.g[S1][0] = 0x4b;
            c.g[S0][0] = c.g[S3][0];
            oc830_credits_set_text(c, 0x2400, 0x0018_5d54, 0x0018_5d60)?;
            c.g[A2][0] = c.g[S1][0];
            oc830_credits_ids(c, 0x0018_5d68, S0, 81)?;
            c.cycles += 16;
            c.g[S1][0] = 0x5a;
            c.g[S0][0] = c.g[S3][0];
            oc830_credits_set_text(c, 0x23f0, 0x0018_5dac, 0x0018_5db8)?;
            oc830_credits_set_text(c, 0x23e0, 0x0018_5dc4, 0x0018_5dd0)?;
            c.g[A2][0] = c.g[S1][0];
            oc830_credits_ids(c, 0x0018_5dd8, S0, 146)?;
            // The tail: four single lines (ids 0x96..0x99), then the last run.
            c.cycles += 52;
            c.g[S0][0] = oc830_sx_sub(c, S4, 0x2430);
            c.g[S5][0] = c.g[S3][0];
            c.g[A0][0] = 0x46_0000;
            c.g[A0][0] = oc830_sx_sub(c, A0, 0x23d0);
            call_ptr(c, 0x0031_7670, 0x0018_5e1c)?;
            c.g[S1][0] = 0x9a;
            let a = oc830_ea(c, S2, 0x5c);
            c.g[A0][0] = c.lw(a);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_1f18, 0x0018_5e2c)?;
            oc830_credits_set_text(c, 0x23c0, 0x0018_5e38, 0x0018_5e44)?;
            for (id, r_fmt, r_hash, r_set) in [
                (0x96u64, 0x0018_5e54u32, 0x0018_5e5cu32, 0x0018_5e68u32),
                (0x97, 0x0018_5e78, 0x0018_5e80, 0x0018_5e8c),
                (0x98, 0x0018_5e9c, 0x0018_5ea4, 0x0018_5eb0),
                (0x99, 0x0018_5ec0, 0x0018_5ec8, 0x0018_5ed4),
            ] {
                c.g[A2][0] = id;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x0041_7828, r_fmt)?;
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x0031_7670, r_hash)?;
                let a = oc830_ea(c, S2, 0x5c);
                c.g[A0][0] = c.lw(a);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_1f18, r_set)?;
            }
            c.g[A2][0] = c.g[S1][0];
            oc830_credits_ids(c, 0x0018_5ed8, S5, 168)?;
        }
        c.cycles += 13;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S6][0];
        Callee {
            addr: 0x0018_6518,
            f: ee::s02::f_00186518,
        }
        .call(c, 0x0018_5f14)?;
        frame.restore(c, 16, 0xa0);
        frame.restore(c, 17, 0x90);
        frame.restore(c, 18, 0x80);
        frame.restore(c, 19, 0x70);
        frame.restore(c, 20, 0x60);
        frame.restore(c, 21, 0x50);
        frame.restore(c, 22, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// OnActivate's `widget->vtable[0x48](widget + delta, 0)` (a2 = widget, a1 = 0).
fn oc830_on_activate_vcall(c: &mut Cpu, ret: u32) -> Flow {
    let a = oc830_ea(c, A2, 0x8);
    c.g[V0][0] = c.lw(a);
    let a = oc830_ea(c, V0, 0x48);
    c.g[A0][0] = c.lh(a);
    let a = oc830_ea(c, V0, 0x4c);
    c.g[V1][0] = c.lw(a);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx(oc830_ea(c, A2, 0).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// Render's `widget->vtable[off](widget + delta, a1)` through `a2`: loads the
/// slot with v0 = vtable, a0 = delta, v1 = fn.
#[inline(always)]
fn oc830_render_vcall(c: &mut Cpu, obj: usize, vt: usize, fnr: usize, off: u32, ret: u32) -> Flow {
    let a = oc830_ea(c, obj, 0x8);
    c.g[vt][0] = c.lw(a);
    let a = oc830_ea(c, vt, off);
    c.g[A0][0] = c.lh(a);
    let a = oc830_ea(c, vt, off + 4);
    c.g[fnr][0] = c.lw(a);
    let target = c.g[fnr][0] as u32;
    c.g[A0][0] = sx(oc830_ea(c, obj, 0).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// `cFEOptionsCredits::Render` (0x185f40): lay out the visible credit lines
/// of the text-scroll widget (`+0x5c`) with the font's glyph metrics (`'~'`
/// moves the marker widget at `+0x58`, `'^'` flips the colour block), draw
/// each run, then wrap the scroll position and run `UIState::Render`.
pub fn cfe_options_credits_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x140);
        frame.save(c, 18, 0xf0);
        frame.save(c, 16, 0x110);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 17, 0x100);
        frame.save(c, 19, 0xe0);
        frame.save(c, 20, 0xd0);
        frame.save(c, 21, 0xc0);
        frame.save(c, 22, 0xb0);
        frame.save(c, 23, 0xa0);
        frame.save(c, 30, 0x90);
        frame.save_ra(c, 0x80);
        frame.save_fpr(c, 23, 0x138);
        frame.save_fpr(c, 22, 0x130);
        frame.save_fpr(c, 21, 0x128);
        frame.save_fpr(c, 20, 0x120);
        let a = oc830_ea(c, S2, 0x5c);
        c.g[A2][0] = c.lw(a);
        'draw: {
            if c.g[A2][0] == 0 {
                break 'draw;
            }
            // Copy the widget's two 12-byte rectangles to sp+0 and sp+0x10.
            c.cycles += 23;
            let a = oc830_ea(c, A2, 0x4b);
            c.ldl(2, a);
            let a = oc830_ea(c, A2, 0x44);
            c.ldr(2, a);
            let a = oc830_ea(c, A2, 0x4c);
            c.g[V1][0] = c.lw(a);
            let sp0 = c.g[SP][0] as u32;
            c.sdl(2, sp0.wrapping_add(0x7));
            c.sdr(2, sp0);
            c.sw(sp0.wrapping_add(0x8), c.g[V1][0]);
            let a = oc830_ea(c, A2, 0x57);
            c.ldl(7, a);
            let a = oc830_ea(c, A2, 0x50);
            c.ldr(7, a);
            let a = oc830_ea(c, A2, 0x58);
            c.g[T0][0] = c.lw(a);
            c.sdl(7, sp0.wrapping_add(0x17));
            c.sdr(7, sp0.wrapping_add(0x10));
            c.sw(sp0.wrapping_add(0x18), c.g[T0][0]);
            c.g[A1][0] = sx(sp0.wrapping_add(0x20));
            oc830_render_vcall(c, A2, V0, V1, 0xa0, 0x0018_5fd4)?;
            let a = oc830_ea(c, S2, 0x5c);
            c.g[A0][0] = c.lw(a);
            call_ptr(c, 0x003a_12d0, 0x0018_5fdc)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A1][0] = 0xffff_ffff_ffff_fc1f;
            if c.g[S1][0] == 0 {
                break 'draw;
            }
            // Scale the font metrics into a line count (s3) and the scroll
            // position into a first line (fp) and row offset (s4).
            c.cycles += 36;
            let a = oc830_ea(c, S1, 0x14);
            c.lwc1(2, a);
            c.cvt_s_w(2, 2);
            let a = oc830_ea(c, S1, 0x34);
            c.lwc1(0, a);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 1);
            let a = oc830_ea(c, S2, 0x5c);
            c.g[V0][0] = c.lw(a);
            c.g[S7][0] = 0;
            c.mul_s(2, 2, 0);
            c.swc1(1, sp0.wrapping_add(0x3c));
            c.swc1(1, sp0.wrapping_add(0x30));
            c.swc1(1, sp0.wrapping_add(0x34));
            c.swc1(1, sp0.wrapping_add(0x38));
            c.lwc1(0, sp0.wrapping_add(0x14));
            c.lwc1(1, sp0.wrapping_add(0x24));
            let a = oc830_ea(c, V0, 0x23);
            c.ldl(4, a);
            let a = oc830_ea(c, V0, 0x1c);
            c.ldr(4, a);
            let a = oc830_ea(c, V0, 0x2b);
            c.ldl(6, a);
            let a = oc830_ea(c, V0, 0x24);
            c.ldr(6, a);
            c.sdl(4, sp0.wrapping_add(0x47));
            c.sdr(4, sp0.wrapping_add(0x40));
            c.sdl(6, sp0.wrapping_add(0x4f));
            c.sdr(6, sp0.wrapping_add(0x48));
            c.mul_s(0, 0, 2);
            c.g[V1][0] = oc830_gp_lw(c, 0x854);
            let a = oc830_ea(c, V1, 0xe84);
            c.g[A0][0] = c.lw(a);
            c.cvt_w_s(2, 0);
            c.mfc1(2, 2);
            let a = oc830_ea(c, A0, 0x8);
            c.g[V1][0] = c.lw(a);
            c.g[V0][0] = sx(oc830_ea(c, V0, 0x3));
            c.g[V1][0] &= c.g[A1][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
            c.g[V1][0] |= 0x2a0;
            c.g[S3][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            c.sw(a, c.g[V1][0]);

            c.cycles += 13;
            c.mtc1(19, 0);
            c.cvt_s_w(0, 0);
            let a = oc830_ea(c, S2, 0x60);
            c.g[A0][0] = c.lw(a);
            c.div(0, 4, 19);
            c.div_s(1, 1, 0);
            c.cvt_w_s(0, 1);
            c.mfc1(2, 0);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
            c.g[V1][0] = c.hi[0];
            c.g[FP][0] = c.lo[0];
            c.g[S4][0] = c.g[V1][0];
            c.g[S6][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            if (c.g[S4][0] as i64) < 0 {
                c.cycles += 3;
                c.g[V0][0] = sx(0u32.wrapping_sub(c.g[A0][0] as u32));
                c.cycles += 3;
                c.div(0, 2, 19);
                c.g[V1][0] = c.hi[0];
                c.g[S4][0] = c.g[V1][0];
            }

            // Draw the lines.
            c.cycles += 2;
            let no_lines = (c.g[S6][0] as i64) <= 0;
            c.g[S0][0] = 0;
            if !no_lines {
                c.cycles += 5;
                c.g[AT][0] = 0x4420_0000;
                c.mtc1(1, 23);
                c.g[AT][0] = 0x3f00_0000;
                c.mtc1(1, 22);
                let a = (c.g[GP][0] as u32).wrapping_sub(0x59ec);
                c.lwc1(21, a);
                loop {
                    'tail: {
                        'next: {
                            c.cycles += 9;
                            let a = oc830_ea(c, S2, 0x5c);
                            c.g[A0][0] = c.lw(a);
                            c.g[A1][0] = sx(oc830_ea(c, S0, 0).wrapping_add(c.g[FP][0] as u32));
                            c.mtc1(19, 20);
                            c.cvt_s_w(20, 20);
                            c.g[S5][0] = sx(oc830_ea(c, S0, 0x1));
                            call_ptr(c, 0x003a_2068, 0x0018_60f8)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[V1][0] = 0x20;
                            if c.g[S0][0] == 0 {
                                break 'next;
                            }
                            // Skip leading spaces.
                            c.cycles += 5;
                            let a = oc830_ea(c, S0, 0);
                            c.g[V0][0] = c.lhu(a);
                            c.mtc1(20, 1);
                            c.cvt_s_w(1, 1);
                            if c.g[V0][0] != c.g[V1][0] {
                                let a = oc830_ea(c, S2, 0x60);
                                c.g[V0][0] = c.lw(a);
                            } else {
                                c.cycles += 2;
                                c.g[S0][0] = sx(oc830_ea(c, S0, 0x2));
                                loop {
                                    c.cycles += 7;
                                    let a = oc830_ea(c, S0, 0);
                                    c.g[V0][0] = c.lhu(a);
                                    if c.g[V0][0] == c.g[V1][0] {
                                        c.g[S0][0] = sx(oc830_ea(c, S0, 0x2));
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0018_6120);
                                        }
                                        continue;
                                    }
                                    c.cycles += 1;
                                    let a = oc830_ea(c, S2, 0x60);
                                    c.g[V0][0] = c.lw(a);
                                    break;
                                }
                            }
                            // Copy rect 0 to sp+0x50 and nudge its y by the
                            // sign of the scroll position.
                            c.cycles += 8;
                            c.ldl(4, sp0.wrapping_add(0x7));
                            c.ldr(4, sp0);
                            c.g[A1][0] = c.lw(sp0.wrapping_add(0x8));
                            c.sdl(4, sp0.wrapping_add(0x57));
                            c.sdr(4, sp0.wrapping_add(0x50));
                            c.sw(sp0.wrapping_add(0x58), c.g[A1][0]);
                            let le = (c.g[V0][0] as i64) <= 0;
                            c.lwc1(0, sp0.wrapping_add(0x54));
                            if le {
                                c.cycles += 1;
                                c.add_s(0, 0, 1);
                            } else {
                                c.cycles += 2;
                                c.sub_s(0, 0, 1);
                            }
                            // A '~' marker moves the marker widget at +0x58.
                            c.cycles += 5;
                            c.swc1(0, sp0.wrapping_add(0x54));
                            let a = oc830_ea(c, S0, 0);
                            c.g[V1][0] = c.lhu(a);
                            c.g[V0][0] = 0x7e;
                            let not_marker = c.g[V1][0] != c.g[V0][0];
                            c.lwc1(1, sp0.wrapping_add(0x10));
                            if !not_marker {
                                c.cycles += 3;
                                let a = oc830_ea(c, S2, 0x58);
                                c.g[A2][0] = c.lw(a);
                                c.g[A1][0] = sx(sp0.wrapping_add(0x60));
                                if c.g[A2][0] != 0 {
                                    c.cycles += 26;
                                    let a = oc830_ea(c, A2, 0x8);
                                    c.g[V1][0] = c.lw(a);
                                    c.g[S7][0] = 0x1;
                                    let a = oc830_ea(c, V1, 0xa0);
                                    c.g[A0][0] = c.lh(a);
                                    let a = oc830_ea(c, V1, 0xa4);
                                    c.g[V0][0] = c.lw(a);
                                    let target = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx(oc830_ea(c, A2, 0).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, target, 0x0018_61a4)?;
                                    c.lwc1(0, sp0.wrapping_add(0x60));
                                    c.g[A1][0] = 0x1;
                                    let a = oc830_ea(c, S2, 0x58);
                                    c.g[V0][0] = c.lw(a);
                                    c.sub_s(0, 23, 0);
                                    c.mul_s(0, 0, 22);
                                    c.swc1(0, sp0.wrapping_add(0x50));
                                    c.ldl(8, sp0.wrapping_add(0x57));
                                    c.ldr(8, sp0.wrapping_add(0x50));
                                    c.g[V1][0] = c.lw(sp0.wrapping_add(0x58));
                                    let a = oc830_ea(c, V0, 0x4b);
                                    c.sdl(8, a);
                                    let a = oc830_ea(c, V0, 0x44);
                                    c.sdr(8, a);
                                    let a = oc830_ea(c, V0, 0x4c);
                                    c.sw(a, c.g[V1][0]);
                                    let a = oc830_ea(c, S2, 0x58);
                                    c.g[A2][0] = c.lw(a);
                                    oc830_render_vcall(c, A2, V0, V1, 0x48, 0x0018_61ec)?;
                                    c.lwc1(0, sp0.wrapping_add(0x4));
                                    break 'tail;
                                }
                            }
                            // Position the run (a '^' skips the colour copy).
                            c.cycles += 7;
                            c.g[V1][0] = 0x5e;
                            c.lwc1(0, sp0.wrapping_add(0x14));
                            c.swc1(1, sp0.wrapping_add(0x60));
                            c.swc1(0, sp0.wrapping_add(0x64));
                            let a = oc830_ea(c, S0, 0);
                            c.g[V0][0] = c.lhu(a);
                            if c.g[V0][0] == c.g[V1][0] {
                                c.cycles += 10;
                                c.ldl(7, sp0.wrapping_add(0x37));
                                c.ldr(7, sp0.wrapping_add(0x30));
                                c.ldl(8, sp0.wrapping_add(0x3f));
                                c.ldr(8, sp0.wrapping_add(0x38));
                                let a = oc830_ea(c, S1, 0x47);
                                c.sdl(7, a);
                                let a = oc830_ea(c, S1, 0x40);
                                c.sdr(7, a);
                                let a = oc830_ea(c, S1, 0x4f);
                                c.sdl(8, a);
                                let a = oc830_ea(c, S1, 0x48);
                                c.sdr(8, a);
                                c.g[S0][0] = sx(oc830_ea(c, S0, 0x2));
                            } else {
                                c.cycles += 14;
                                c.ldl(4, sp0.wrapping_add(0x47));
                                c.ldr(4, sp0.wrapping_add(0x40));
                                c.ldl(5, sp0.wrapping_add(0x4f));
                                c.ldr(5, sp0.wrapping_add(0x48));
                                let a = oc830_ea(c, S1, 0x47);
                                c.sdl(4, a);
                                let a = oc830_ea(c, S1, 0x40);
                                c.sdr(4, a);
                                let a = oc830_ea(c, S1, 0x4f);
                                c.sdl(5, a);
                                let a = oc830_ea(c, S1, 0x48);
                                c.sdr(5, a);
                                c.lwc1(0, sp0.wrapping_add(0x60));
                                c.lwc1(1, sp0.wrapping_add(0x64));
                                c.sub_s(0, 0, 21);
                                c.sub_s(1, 1, 21);
                                c.swc1(0, sp0.wrapping_add(0x60));
                                c.swc1(1, sp0.wrapping_add(0x64));
                            }
                            // Measure and draw the run.
                            c.cycles += 27;
                            c.lwc1(12, sp0.wrapping_add(0x60));
                            c.g[A0][0] = c.g[S1][0];
                            c.lwc1(13, sp0.wrapping_add(0x64));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A2][0] = sx(sp0.wrapping_add(0x70));
                            c.g[A3][0] = 0;
                            call_ptr(c, 0x0039_21f0, 0x0018_628c)?;
                            c.lwc1(12, sp0.wrapping_add(0x78));
                            c.g[A0][0] = c.g[S1][0];
                            c.lwc1(1, sp0.wrapping_add(0x60));
                            c.g[A1][0] = c.g[S0][0];
                            c.sub_s(12, 23, 12);
                            c.lwc1(13, sp0.wrapping_add(0x54));
                            c.mul_s(12, 12, 22);
                            c.swc1(12, sp0.wrapping_add(0x50));
                            let a = oc830_ea(c, S1, 0x30);
                            c.lwc1(0, a);
                            let a = oc830_ea(c, S1, 0x34);
                            c.lwc1(2, a);
                            c.mul_s(0, 0, 1);
                            let a = oc830_ea(c, S1, 0x38);
                            c.swc1(0, a);
                            c.lwc1(1, sp0.wrapping_add(0x64));
                            c.mul_s(2, 2, 1);
                            let a = oc830_ea(c, S1, 0x3c);
                            c.swc1(2, a);
                            call_ptr(c, 0x0039_1e30, 0x0018_62cc)?;
                            let a = oc830_ea(c, S1, 0x37);
                            c.ldl(8, a);
                            let a = oc830_ea(c, S1, 0x30);
                            c.ldr(8, a);
                            let a = oc830_ea(c, S1, 0x3f);
                            c.sdl(8, a);
                            let a = oc830_ea(c, S1, 0x38);
                            c.sdr(8, a);
                        }
                        c.cycles += 1;
                        c.lwc1(0, sp0.wrapping_add(0x4));
                    }
                    c.cycles += 5;
                    c.g[S0][0] = c.g[S5][0];
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S6][0] as i64)) as u64;
                    c.add_s(0, 0, 20);
                    let more = c.g[V0][0] != 0;
                    c.swc1(0, sp0.wrapping_add(0x4));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0018_60e0);
                    }
                }
            }

            // Reset the marker widget when no '~' was seen.
            c.cycles += 2;
            if c.g[S7][0] != 0 {
                let a = oc830_ea(c, S2, 0x5c);
                c.g[V1][0] = c.lw(a);
            } else {
                c.cycles += 3;
                let a = oc830_ea(c, S2, 0x58);
                c.g[A2][0] = c.lw(a);
                c.g[A1][0] = 0;
                if c.g[A2][0] != 0 {
                    c.cycles += 5;
                    oc830_render_vcall(c, A2, V0, V1, 0x48, 0x0018_631c)?;
                }
                c.cycles += 1;
                let a = oc830_ea(c, S2, 0x5c);
                c.g[V1][0] = c.lw(a);
            }
            // Wrap the scroll position into the text height.
            c.cycles += 8;
            c.mult(0, 5, 22, 19);
            let a = oc830_ea(c, S2, 0x60);
            c.g[A0][0] = c.lw(a);
            let a = oc830_ea(c, V1, 0xc8);
            c.g[V0][0] = c.lhu(a);
            c.mult(0, 2, 2, 19);
            c.g[V1][0] = sx(0u32.wrapping_sub(c.g[A1][0] as u32));
            c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            let in_range = c.g[A0][0] == 0;
            c.g[V1][0] = sx(oc830_ea(c, V0, 0).wrapping_add(c.g[A1][0] as u32));
            if !in_range {
                c.cycles += 3;
                let a = oc830_ea(c, S2, 0x64);
                c.g[V0][0] = c.lw(a);
                if (c.g[V0][0] as i64) < 0 {
                    let a = oc830_ea(c, S2, 0x60);
                    c.sw(a, c.g[V1][0]);
                }
            }
            c.cycles += 4;
            let a = oc830_ea(c, S2, 0x60);
            c.g[V0][0] = c.lw(a);
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                let a = oc830_ea(c, S2, 0x64);
                c.g[V0][0] = c.lw(a);
                let stopped = (c.g[V0][0] as i64) <= 0;
                c.g[V0][0] = sx(0u32.wrapping_sub(c.g[A1][0] as u32));
                if !stopped {
                    c.cycles += 1;
                    let a = oc830_ea(c, S2, 0x60);
                    c.sw(a, c.g[V0][0]);
                }
            }
        }
        c.cycles += 18;
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0039_e6b8, 0x0018_6374)?;
        frame.restore(c, 16, 0x110);
        frame.restore(c, 17, 0x100);
        frame.restore(c, 18, 0xf0);
        frame.restore(c, 19, 0xe0);
        frame.restore(c, 20, 0xd0);
        frame.restore(c, 21, 0xc0);
        frame.restore(c, 22, 0xb0);
        frame.restore(c, 23, 0xa0);
        frame.restore(c, 30, 0x90);
        frame.restore_ra(c, 0x80);
        frame.restore_fpr(c, 23, 0x138);
        frame.restore_fpr(c, 22, 0x130);
        frame.restore_fpr(c, 21, 0x128);
        frame.restore_fpr(c, 20, 0x120);
        frame.close(c);
        Ok(())
    })
}
/// `cFEStateMemCard::ShowNotDetectedMsg` (0x186d60): format the "memory card
/// not detected" text for the current slot and show it with two buttons.
pub fn cfe_state_mem_card_show_not_detected_msg(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 120;
        let frame = Frame::open(c, 0x6b0);
        c.g[A0][0] = oc830_gp_lw(c, 0x848);
        frame.save(c, 16, 0x6a0);
        frame.save(c, 17, 0x690);
        frame.save(c, 18, 0x680);
        frame.save(c, 19, 0x670);
        frame.save_ra(c, 0x660);
        call_ptr(c, 0x0022_7f80, 0x0018_6d84)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = c.g[SP][0];
        let a = oc830_ea(c, S1, 0x428);
        c.g[A2][0] = c.lw(a);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0024_1dc8, 0x0018_6d98)?;
        call_ptr(c, 0x0014_dc80, 0x0018_6da0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x5;
        call_ptr(c, 0x0014_dd58, 0x0018_6dac)?;
        let a = oc830_ea(c, S1, 0x434);
        c.g[S0][0] = c.lw(a);
        c.g[S2][0] = c.lw(c.g[S0][0] as u32);
        let a = oc830_ea(c, S2, 0xb0);
        c.g[V0][0] = c.lh(a);
        c.g[S0][0] = sx(oc830_ea(c, S0, 0).wrapping_add(c.g[V0][0] as u32));
        call_ptr(c, 0x0015_2ba8, 0x0018_6dc0)?;
        let a = oc830_ea(c, S2, 0xb4);
        c.g[V1][0] = c.lw(a);
        c.g[A1][0] = c.g[V0][0];
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, target, 0x0018_6dd0)?;
        c.g[A1][0] = oc830_gp_lw(c, 0x848);
        c.g[S3][0] = c.g[V0][0];
        c.g[A0][0] = 0x46_0000;
        c.g[S2][0] = sx(oc830_ea(c, SP, 0x20));
        let a = oc830_ea(c, A1, 0x8c);
        c.g[S0][0] = c.lw(a);
        c.g[A0][0] = oc830_sx_sub(c, A0, 0x2358);
        let a = oc830_ea(c, S0, 0x4);
        c.g[S1][0] = c.lw(a);
        let a = oc830_ea(c, S1, 0x20);
        c.g[V0][0] = c.lh(a);
        c.g[S0][0] = sx(oc830_ea(c, S0, 0).wrapping_add(c.g[V0][0] as u32));
        call_ptr(c, 0x0031_7670, 0x0018_6df8)?;
        let a = oc830_ea(c, S1, 0x24);
        c.g[V1][0] = c.lw(a);
        c.g[A0][0] = c.g[S0][0];
        let target = c.g[V1][0] as u32;
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, target, 0x0018_6e08)?;
        c.g[A2][0] = c.g[SP][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A3][0] = c.g[S3][0];
        call_ptr(c, 0x002c_26d0, 0x0018_6e1c)?;
        c.g[T3][0] = oc830_gp_lw(c, 0x1c38);
        c.g[V0][0] = 0x9;
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        let a = oc830_ea(c, T3, 0x19c);
        c.sw(a, c.g[V0][0]);
        c.g[T2][0] = 0;
        let a = oc830_ea(c, T3, 0x8);
        c.g[V0][0] = c.lw(a);
        c.g[A3][0] = 0;
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = 0;
        let a = oc830_ea(c, V0, 0x120);
        c.g[A0][0] = c.lh(a);
        let a = oc830_ea(c, V0, 0x124);
        c.g[V1][0] = c.lw(a);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(oc830_ea(c, T3, 0).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0018_6e54)?;
        // Button 1 (a2 = 0), then button 2 (a2 = 1): text lookup + button add.
        c.g[A1][0] = oc830_gp_lw(c, 0x1c38);
        c.g[A0][0] = 0x46_0000;
        c.g[V0][0] = oc830_gp_lw(c, 0x848);
        c.g[A0][0] = oc830_sx_sub(c, A0, 0x2338);
        let a = oc830_ea(c, A1, 0x218);
        c.g[S1][0] = c.lw(a);
        let a = oc830_ea(c, V0, 0x8c);
        c.g[S0][0] = c.lw(a);
        let a = oc830_ea(c, S1, 0x8);
        c.g[S3][0] = c.lw(a);
        let a = oc830_ea(c, S0, 0x4);
        c.g[S2][0] = c.lw(a);
        let a = oc830_ea(c, S3, 0xd0);
        c.g[V0][0] = c.lh(a);
        let a = oc830_ea(c, S2, 0x20);
        c.g[V1][0] = c.lh(a);
        c.g[S1][0] = sx(oc830_ea(c, S1, 0).wrapping_add(c.g[V0][0] as u32));
        c.g[S0][0] = sx(oc830_ea(c, S0, 0).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, 0x0031_7670, 0x0018_6e88)?;
        let a = oc830_ea(c, S2, 0x24);
        c.g[V1][0] = c.lw(a);
        c.g[A0][0] = c.g[S0][0];
        let target = c.g[V1][0] as u32;
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, target, 0x0018_6e98)?;
        let a = oc830_ea(c, S3, 0xd4);
        c.g[A3][0] = c.lw(a);
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        let target = c.g[A3][0] as u32;
        c.g[A2][0] = 0;
        call_ptr(c, target, 0x0018_6eac)?;
        c.g[V1][0] = oc830_gp_lw(c, 0x1c38);
        c.g[A0][0] = 0x46_0000;
        c.g[V0][0] = oc830_gp_lw(c, 0x848);
        c.g[A0][0] = oc830_sx_sub(c, A0, 0x2328);
        let a = oc830_ea(c, V1, 0x218);
        c.g[S1][0] = c.lw(a);
        let a = oc830_ea(c, V0, 0x8c);
        c.g[S0][0] = c.lw(a);
        let a = oc830_ea(c, S1, 0x8);
        c.g[S3][0] = c.lw(a);
        let a = oc830_ea(c, S0, 0x4);
        c.g[S2][0] = c.lw(a);
        let a = oc830_ea(c, S3, 0xd0);
        c.g[V0][0] = c.lh(a);
        let a = oc830_ea(c, S2, 0x20);
        c.g[V1][0] = c.lh(a);
        c.g[S1][0] = sx(oc830_ea(c, S1, 0).wrapping_add(c.g[V0][0] as u32));
        c.g[S0][0] = sx(oc830_ea(c, S0, 0).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, 0x0031_7670, 0x0018_6ee0)?;
        let a = oc830_ea(c, S2, 0x24);
        c.g[V1][0] = c.lw(a);
        c.g[A0][0] = c.g[S0][0];
        let target = c.g[V1][0] as u32;
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, target, 0x0018_6ef0)?;
        let a = oc830_ea(c, S3, 0xd4);
        c.g[A3][0] = c.lw(a);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        let target = c.g[A3][0] as u32;
        c.g[A2][0] = 0x1;
        call_ptr(c, target, 0x0018_6f04)?;
        oc830_msg_show(c, 0x0018_6f24)?;
        frame.restore(c, 16, 0x6a0);
        frame.restore(c, 17, 0x690);
        frame.restore(c, 18, 0x680);
        frame.restore(c, 19, 0x670);
        frame.restore_ra(c, 0x660);
        frame.close(c);
        Ok(())
    })
}

/// The last call of both memory-card messages: `popup->vtable[0xc8](popup +
/// delta, 1)` for the popup at `*(gp-0x1c38) + 0x218`.
#[inline(always)]
fn oc830_msg_show(c: &mut Cpu, ret: u32) -> Flow {
    c.g[V1][0] = oc830_gp_lw(c, 0x1c38);
    c.g[A1][0] = 0x1;
    let a = oc830_ea(c, V1, 0x218);
    c.g[A2][0] = c.lw(a);
    let a = oc830_ea(c, A2, 0x8);
    c.g[V0][0] = c.lw(a);
    let a = oc830_ea(c, V0, 0xc8);
    c.g[A0][0] = c.lh(a);
    let a = oc830_ea(c, V0, 0xcc);
    c.g[V1][0] = c.lw(a);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx(oc830_ea(c, A2, 0).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// `cFEStateMemCard::ShowNoSpaceMsg` (0x187360): format the "not enough
/// space" text for the current slot and show it with one button.
pub fn cfe_state_mem_card_show_no_space_msg(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 98;
        let frame = Frame::open(c, 0x6b0);
        c.g[A0][0] = oc830_gp_lw(c, 0x848);
        frame.save(c, 16, 0x6a0);
        frame.save(c, 17, 0x690);
        frame.save(c, 18, 0x680);
        frame.save(c, 19, 0x670);
        frame.save_ra(c, 0x660);
        call_ptr(c, 0x0022_7f80, 0x0018_7384)?;
        c.g[S2][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0018_738c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x5;
        call_ptr(c, 0x0014_dd58, 0x0018_7398)?;
        c.g[V1][0] = oc830_gp_lw(c, 0x1c38);
        c.g[V0][0] = 0x14;
        let a = oc830_ea(c, V1, 0x19c);
        c.sw(a, c.g[V0][0]);
        let a = oc830_ea(c, S2, 0x434);
        c.g[S0][0] = c.lw(a);
        c.g[S1][0] = c.lw(c.g[S0][0] as u32);
        let a = oc830_ea(c, S1, 0xb0);
        c.g[V0][0] = c.lh(a);
        c.g[S0][0] = sx(oc830_ea(c, S0, 0).wrapping_add(c.g[V0][0] as u32));
        call_ptr(c, 0x0015_2ba8, 0x0018_73b8)?;
        let a = oc830_ea(c, S1, 0xb4);
        c.g[V1][0] = c.lw(a);
        c.g[A0][0] = c.g[S0][0];
        let target = c.g[V1][0] as u32;
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, target, 0x0018_73c8)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[S3][0] = sx(oc830_ea(c, SP, 0x640));
        let a = oc830_ea(c, A0, 0x428);
        c.g[A2][0] = c.lw(a);
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, 0x0024_1dc8, 0x0018_73e0)?;
        c.g[V0][0] = oc830_gp_lw(c, 0x848);
        c.g[A0][0] = 0x46_0000;
        c.g[A0][0] = oc830_sx_sub(c, A0, 0x2318);
        let a = oc830_ea(c, V0, 0x8c);
        c.g[S0][0] = c.lw(a);
        let a = oc830_ea(c, S0, 0x4);
        c.g[S1][0] = c.lw(a);
        let a = oc830_ea(c, S1, 0x20);
        c.g[V0][0] = c.lh(a);
        c.g[S0][0] = sx(oc830_ea(c, S0, 0).wrapping_add(c.g[V0][0] as u32));
        call_ptr(c, 0x0031_7670, 0x0018_7400)?;
        let a = oc830_ea(c, S1, 0x24);
        c.g[V1][0] = c.lw(a);
        c.g[A0][0] = c.g[S0][0];
        let target = c.g[V1][0] as u32;
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, target, 0x0018_7410)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S2][0];
        call_ptr(c, 0x002c_26d0, 0x0018_7424)?;
        c.g[T3][0] = oc830_gp_lw(c, 0x1c38);
        c.g[A1][0] = c.g[SP][0];
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        let a = oc830_ea(c, T3, 0x8);
        c.g[V0][0] = c.lw(a);
        c.g[T2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A2][0] = 0;
        let a = oc830_ea(c, V0, 0x120);
        c.g[A0][0] = c.lh(a);
        let a = oc830_ea(c, V0, 0x124);
        c.g[V1][0] = c.lw(a);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(oc830_ea(c, T3, 0).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0018_7454)?;
        c.g[A1][0] = oc830_gp_lw(c, 0x1c38);
        c.g[A0][0] = 0x46_0000;
        c.g[V0][0] = oc830_gp_lw(c, 0x848);
        c.g[A0][0] = oc830_sx_sub(c, A0, 0x2328);
        let a = oc830_ea(c, A1, 0x218);
        c.g[S1][0] = c.lw(a);
        let a = oc830_ea(c, V0, 0x8c);
        c.g[S0][0] = c.lw(a);
        let a = oc830_ea(c, S1, 0x8);
        c.g[S3][0] = c.lw(a);
        let a = oc830_ea(c, S0, 0x4);
        c.g[S2][0] = c.lw(a);
        let a = oc830_ea(c, S3, 0xd0);
        c.g[V0][0] = c.lh(a);
        let a = oc830_ea(c, S2, 0x20);
        c.g[V1][0] = c.lh(a);
        c.g[S1][0] = sx(oc830_ea(c, S1, 0).wrapping_add(c.g[V0][0] as u32));
        c.g[S0][0] = sx(oc830_ea(c, S0, 0).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, 0x0031_7670, 0x0018_7488)?;
        let a = oc830_ea(c, S2, 0x24);
        c.g[V1][0] = c.lw(a);
        c.g[A0][0] = c.g[S0][0];
        let target = c.g[V1][0] as u32;
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, target, 0x0018_7498)?;
        let a = oc830_ea(c, S3, 0xd4);
        c.g[A3][0] = c.lw(a);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        let target = c.g[A3][0] as u32;
        c.g[A2][0] = 0x1;
        call_ptr(c, target, 0x0018_74ac)?;
        oc830_msg_show(c, 0x0018_74cc)?;
        frame.restore(c, 16, 0x6a0);
        frame.restore(c, 17, 0x690);
        frame.restore(c, 18, 0x680);
        frame.restore(c, 19, 0x670);
        frame.restore_ra(c, 0x660);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, check_lohi, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    // Fixture objects live in DATA+0x200000 and up, clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return.
    pub(crate) const OC830_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC830_MAGIC: u32 = 0x4f43_3830;
    /// Per-callee forced result words, one per callee address.
    pub(crate) const OC830_MAIL: u32 = DATA + 0x30_0000;
    /// Fake-call counters, `0x100000` above the mailbox words.
    pub(crate) const OC830_STRIDE: u32 = 0xffff_fff0;
    /// The all-fake vtable: entry `{delta 0, fn FAKE}` at every multiple of 8.
    pub(crate) const OC830_V: u32 = DATA + 0x21_0000;
    pub(crate) const OC830_OBJ: u32 = DATA + 0x21_1000;
    pub(crate) const OC830_W: u32 = DATA + 0x21_2000;
    pub(crate) const OC830_STR: u32 = DATA + 0x22_6000;

    pub(crate) fn oc830_mail(addr: u32) -> u32 {
        OC830_MAIL + (addr & 0xf_fffc)
    }

    /// Make the fake for `addr` return `v` in `v0`.
    pub(crate) fn oc830_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc830_mail(addr), v.wrapping_add(3));
    }

    pub(crate) fn oc830_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(oc830_mail(addr), 1);
    }

    /// The fake callee: logs like `fakes::fake_call`, then returns its forced
    /// result (a pointer by default; `OC830_STRIDE`: string `n % 8`).
    pub(crate) fn oc830_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc830_mail(addr));
        let n = c.bus.read32(oc830_mail(addr) + 0x10_0000);
        c.bus.write32(oc830_mail(addr) + 0x10_0000, n + 1);
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = match mail {
                1 => 0,
                OC830_STRIDE => (OC830_STR + 0x40 * (n % 8)) as u64,
                m => (m - 3) as u64,
            };
        }
    }

    macro_rules! oc830_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC830_CTRL) != OC830_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc830_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc830_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc830_fakes!(
        (
            oc830_fake_39e390,
            OC830_PREV_39E390,
            ee::s13::OVERRIDE_0039E390,
            ee::s13::f_0039e390__generated,
            0x0039e390
        ),
        (
            oc830_fake_397cc0,
            OC830_PREV_397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x00397cc0
        ),
        (
            oc830_fake_39c870,
            OC830_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039c870
        ),
        (
            oc830_fake_39d860,
            OC830_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039d860
        ),
        (
            oc830_fake_3a12d0,
            OC830_PREV_3A12D0,
            ee::s13::OVERRIDE_003A12D0,
            ee::s13::f_003a12d0__generated,
            0x003a12d0
        ),
        (
            oc830_fake_3a2068,
            OC830_PREV_3A2068,
            ee::s13::OVERRIDE_003A2068,
            ee::s13::f_003a2068__generated,
            0x003a2068
        ),
        (
            oc830_fake_39e6b8,
            OC830_PREV_39E6B8,
            ee::s13::OVERRIDE_0039E6B8,
            ee::s13::f_0039e6b8__generated,
            0x0039e6b8
        ),
        (
            oc830_fake_3a1f18,
            OC830_PREV_3A1F18,
            ee::s13::OVERRIDE_003A1F18,
            ee::s13::f_003a1f18__generated,
            0x003a1f18
        ),
        (
            oc830_fake_317670,
            OC830_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x00317670
        ),
        (
            oc830_fake_3921f0,
            OC830_PREV_3921F0,
            ee::s12::OVERRIDE_003921F0,
            ee::s12::f_003921f0__generated,
            0x003921f0
        ),
        (
            oc830_fake_391e30,
            OC830_PREV_391E30,
            ee::s12::OVERRIDE_00391E30,
            ee::s12::f_00391e30__generated,
            0x00391e30
        ),
        (
            oc830_fake_417828,
            OC830_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x00417828
        ),
        (
            oc830_fake_186518,
            OC830_PREV_186518,
            ee::s02::OVERRIDE_00186518,
            ee::s02::f_00186518__generated,
            0x00186518
        ),
        (
            oc830_fake_227f80,
            OC830_PREV_227F80,
            ee::s05::OVERRIDE_00227F80,
            ee::s05::f_00227f80__generated,
            0x00227f80
        ),
        (
            oc830_fake_241dc8,
            OC830_PREV_241DC8,
            ee::s06::OVERRIDE_00241DC8,
            ee::s06::f_00241dc8__generated,
            0x00241dc8
        ),
        (
            oc830_fake_14dc80,
            OC830_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014dc80
        ),
        (
            oc830_fake_14dd58,
            OC830_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014dd58
        ),
        (
            oc830_fake_152ba8,
            OC830_PREV_152BA8,
            ee::s01::OVERRIDE_00152BA8,
            ee::s01::f_00152ba8__generated,
            0x00152ba8
        ),
        (
            oc830_fake_2c26d0,
            OC830_PREV_2C26D0,
            ee::s08::OVERRIDE_002C26D0,
            ee::s08::f_002c26d0__generated,
            0x002c26d0
        ),
    );

    /// Switch this machine's callee fakes on and build the shared vtable.
    pub(crate) fn oc830_arm(c: &mut Cpu) {
        c.bus.write32(OC830_CTRL, OC830_MAGIC);
        for off in (0..0x140).step_by(8) {
            c.bus.write16(OC830_V + off, 0);
            c.bus.write32(OC830_V + off + 4, fakes::FAKE);
        }
    }

    fn oc830_gp_set(c: &mut Cpu, off: u32, v: u32) {
        c.bus.write32(GAME_GP - off, v);
    }

    #[test]
    fn cfe_options_credits_dtor_matches() {
        oc830_install_fakes();
        check(
            ee::s02::f_00185a70__generated,
            cfe_options_credits_dtor,
            12,
            &|c, seed| {
                machine(c, seed, 0x18_5a70, 1);
                oc830_arm(c);
                c.pc = 0x0018_5a70;
                c.g[A0][0] = sx(OC830_OBJ);
            },
        );
    }

    #[test]
    fn cfe_options_credits_init_matches() {
        oc830_install_fakes();
        check(
            ee::s02::f_00185a98__generated,
            cfe_options_credits_init,
            48,
            &|c, seed| {
                machine(c, seed, 0x18_5a98, 8);
                oc830_arm(c);
                c.pc = 0x0018_5a98;
                c.g[A0][0] = sx(OC830_OBJ);
                c.bus.write32(OC830_OBJ + 0x10, DATA + 0x1234);
                if seed % 4 >= 2 {
                    oc830_force_zero(c, 0x0039_7cc0);
                }
                // FindObject: nothing, or a widget with the shared vtable.
                if seed % 2 == 1 {
                    oc830_force(c, 0x0039_d860, OC830_W);
                    c.bus.write32(OC830_W + 0x8, OC830_V);
                    c.bus.write32(OC830_W + 0x90, 0x10);
                } else {
                    oc830_force_zero(c, 0x0039_d860);
                }
            },
        );
    }

    #[test]
    fn cfe_options_credits_on_activate_matches() {
        oc830_install_fakes();
        check(
            ee::s02::f_00185ba0__generated,
            cfe_options_credits_on_activate,
            40,
            &|c, seed| {
                machine(c, seed, 0x18_5ba0, 200);
                oc830_arm(c);
                c.pc = 0x0018_5ba0;
                c.g[A0][0] = sx(OC830_OBJ);
                c.g[A1][0] = sx(DATA + 0x77);
                if seed % 4 == 0 {
                    oc830_force_zero(c, 0x0039_d860);
                } else {
                    oc830_force(c, 0x0039_d860, OC830_W);
                    c.bus.write32(OC830_W + 0x8, OC830_V);
                }
            },
        );
    }

    #[test]
    fn cfe_options_credits_render_matches() {
        oc830_install_fakes();
        const T: u32 = DATA + 0x22_1000;
        const W2: u32 = DATA + 0x22_2000;
        const F: u32 = DATA + 0x22_3000;
        const G: u32 = DATA + 0x22_4000;
        const GO: u32 = DATA + 0x22_5000;
        const POOL: [u16; 6] = [0x20, 0x41, 0x42, 0x7e, 0x5e, 0x20];
        check_lohi(
            ee::s02::f_00185f40__generated,
            cfe_options_credits_render,
            96,
            &|c, seed| {
                let mut r = machine(c, seed, 0x18_5f40, 60);
                oc830_arm(c);
                c.pc = 0x0018_5f40;
                let s = OC830_OBJ;
                c.g[A0][0] = sx(s);
                let sp0 = (c.g[SP][0] as u32).wrapping_sub(0x140);
                let no_text = seed % 11 == 0;
                c.bus.write32(s + 0x5c, if no_text { 0 } else { T });
                c.bus.write32(s + 0x58, if seed % 3 == 0 { 0 } else { W2 });
                c.bus
                    .write32(s + 0x60, (r.next_u32() % 200).wrapping_sub(100));
                c.bus
                    .write32(s + 0x64, [0xffff_fffd, 0, 5][seed as usize % 3]);
                // The text-scroll widget and its rectangles.
                c.bus.write32(T + 0x8, OC830_V);
                for off in [0x1c, 0x24, 0x44, 0x4c, 0x50, 0x58] {
                    c.bus.write32(T + off, r.next_u32());
                }
                c.bus.write32(
                    T + 0x54,
                    [0x3f80_0000, 0x4000_0000, 0x3f00_0000][seed as usize % 3],
                );
                c.bus.write16(T + 0xc8, (r.next_u32() % 50) as u16);
                c.bus.write32(W2 + 0x8, OC830_V);
                // The font: 1..4 lines of 1.0 height.
                c.bus.write32(F + 0x14, 1 + seed % 4);
                c.bus.write32(F + 0x34, 0x3f80_0000);
                for off in [0x30, 0x38, 0x3c, 0x40, 0x44, 0x48] {
                    c.bus.write32(F + off, r.next_u32());
                }
                if seed % 13 == 0 {
                    oc830_force_zero(c, 0x003a_12d0);
                } else {
                    oc830_force(c, 0x003a_12d0, F);
                }
                c.bus.write32(oc830_mail(0x003a_2068), OC830_STRIDE);
                // Globals: the glyph-flag word through gp-0x854, a float at gp-0x59ec.
                oc830_gp_set(c, 0x854, G);
                c.bus.write32(G + 0xe84, GO);
                c.bus.write32(GO + 0x8, 0x1234_5678);
                oc830_gp_set(c, 0x59ec, 0x3f00_0000);
                // Total height (float at sp+0x24): 6, 12, 30, -5 or 0.
                let h =
                    [0x40c0_0000u32, 0x4140_0000, 0x41f0_0000, 0xc0a0_0000, 0][seed as usize % 5];
                c.bus.write32(sp0 + 0x24, h);
                // Eight lines of text.
                for k in 0..8u32 {
                    for i in 0..6u32 {
                        let ch = POOL[(r.next_u32() % 6) as usize];
                        c.bus.write16(OC830_STR + 0x40 * k + 2 * i, ch);
                    }
                    c.bus.write16(OC830_STR + 0x40 * k + 12, 0);
                }
            },
        );
    }

    /// The memory-card message objects: `MCManager_Get` returns the manager,
    /// the world and the popup hang off gp, all with the shared vtable.
    fn oc830_memcard_world(c: &mut Cpu) {
        const M: u32 = DATA + 0x23_0000;
        const P: u32 = DATA + 0x23_1000;
        const WORLD: u32 = DATA + 0x23_2000;
        const X: u32 = DATA + 0x23_3000;
        const G2: u32 = DATA + 0x23_4000;
        const H: u32 = DATA + 0x23_5000;
        oc830_force(c, 0x0022_7f80, M);
        c.bus.write32(M + 0x428, 0x777);
        c.bus.write32(M + 0x434, P);
        c.bus.write32(P, OC830_V);
        oc830_gp_set(c, 0x848, WORLD);
        c.bus.write32(WORLD + 0x8c, X);
        c.bus.write32(X + 0x4, OC830_V);
        oc830_gp_set(c, 0x1c38, G2);
        c.bus.write32(G2 + 0x8, OC830_V);
        c.bus.write32(G2 + 0x218, H);
        c.bus.write32(H + 0x8, OC830_V);
    }

    #[test]
    fn cfe_state_mem_card_show_not_detected_msg_matches() {
        oc830_install_fakes();
        check(
            ee::s02::f_00186d60__generated,
            cfe_state_mem_card_show_not_detected_msg,
            32,
            &|c, seed| {
                machine(c, seed, 0x18_6d60, 14);
                oc830_arm(c);
                c.pc = 0x0018_6d60;
                oc830_memcard_world(c);
            },
        );
    }

    #[test]
    fn cfe_state_mem_card_show_no_space_msg_matches() {
        oc830_install_fakes();
        check(
            ee::s02::f_00187360__generated,
            cfe_state_mem_card_show_no_space_msg,
            32,
            &|c, seed| {
                machine(c, seed, 0x18_7360, 12);
                oc830_arm(c);
                c.pc = 0x0018_7360;
                oc830_memcard_world(c);
            },
        );
    }
}
