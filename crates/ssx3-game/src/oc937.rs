//! Staging module oc937 (wave wave255): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc937`.

#![allow(unused_imports)]

use crate::abi;
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001c_58e8,
        c_fe_popup_confirm_base_ctor,
        "cFEPopupConfirm_BaseCtor",
    ),
    (0x001c_5b68, c_fe_popup_confirm_v05, "cFEPopupConfirm_v05"),
    (
        0x001c_5e58,
        c_fe_popup_confirm_set_content_visible,
        "cFEPopupConfirm_SetContentVisible",
    ),
    (
        0x001c_7620,
        c_fe_popup_confirm_compute_background_scale,
        "cFEPopupConfirm_ComputeBackgroundScale",
    ),
    (
        0x001c_7eb0,
        c_fe_popup_confirm_layout_text_entry_box,
        "cFEPopupConfirm_LayoutTextEntryBox",
    ),
    (
        0x001c_80e8,
        c_fe_popup_confirm_desc_to_string,
        "cFEPopupConfirm_DescToString",
    ),
    (
        0x001c_8320,
        c_fe_popup_confirm_measure_desc,
        "cFEPopupConfirm_MeasureDesc",
    ),
    (
        0x001c_8678,
        c_fe_popup_confirm_layout_title,
        "cFEPopupConfirm_LayoutTitle",
    ),
];

/// `base + off` as a guest address (32-bit wrap, as the generated code does).
#[inline]
fn at_937(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// `cFEPopupConfirm_BaseCtor(this)` (0x1c58e8): the popup's base constructor.
/// Runs the UI-state constructor on `this`, the config constructor on
/// `this+0xbc`, memsets the five confirmation elements at `this+0x2d0`, and
/// clears the pointers, flags and the default colours.
pub fn c_fe_popup_confirm_base_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = sx(at_937(c.g[S0][0], 0x2d0));
        call_ptr(c, 0x0039_e318, 0x001c_5908)?;
        c.g[S2][0] = 4;
        c.g[V1][0] = sx(0x0047_0000u32.wrapping_sub(0x3428));
        c.g[A0][0] = sx(at_937(c.g[S0][0], 0xbc));
        let v1 = c.g[V1][0];
        let this = at_937(c.g[S0][0], 0);
        Addr(this.wrapping_add(8)).sw(c, v1);
        call_ptr(c, 0x001c_55f8, 0x001c_5920)?;
        let this = at_937(c.g[S0][0], 0);
        c.g[V1][0] = Addr(at_937(c.g[GP][0], 0xda0)).lw(c);
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        let v0 = c.g[V0][0];
        Addr(this.wrapping_add(0x2c4)).sw(c, v0);
        let v1 = c.g[V1][0];
        Addr(this.wrapping_add(0x2b4)).sw(c, v1);
        Addr(this.wrapping_add(0x2b0)).sw(c, v1);
        for off in [0x2a8u32, 0x2b8, 0x2bc, 0x30c, 0x2c0, 0x314] {
            Addr(this.wrapping_add(off)).sw(c, 0);
        }
        c.g[A0][0] = c.g[S1][0];
        loop {
            c.cycles += 7;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0xc;
            c.g[S1][0] = sx(at_937(c.g[S1][0], 0xc));
            call_ptr(c, 0x0041_6210, 0x001c_5960)?;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            let more = (c.g[S2][0] as i64) >= 0;
            c.g[A0][0] = c.g[S1][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001c_5950);
            }
        }
        c.cycles += 23;
        let this = at_937(c.g[S0][0], 0);
        c.g[V0][0] = 1;
        Addr(this.wrapping_add(0x48)).sw(c, 0);
        let v0 = c.g[V0][0];
        Addr(this.wrapping_add(0x310)).sw(c, v0);
        c.g[V1][0] = 4;
        Addr(this.wrapping_add(0x4c)).sw(c, 0);
        c.g[V0][0] = sx(this.wrapping_add(0xa0));
        for off in [
            0x6cu32, 0x70, 0x74, 0x78, 0x7c, 0x80, 0x88, 0x84, 0x8c, 0x64, 0x50, 0x54, 0x58, 0x5c,
            0x60, 0x68,
        ] {
            Addr(this.wrapping_add(off)).sw(c, 0);
        }
        loop {
            c.cycles += 7;
            Addr(c.g[V0][0] as u32).sw(c, 0);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001c_59c8);
            }
        }
        c.cycles += 43;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[A1][0] = 0x3f;
        Addr(this.wrapping_add(0x31c)).sw(c, 0);
        c.g[V0][0] = c.g[S0][0];
        let spv = sp(c);
        c.swc1(0, spv.wrapping_add(8));
        c.swc1(0, spv);
        c.swc1(0, spv.wrapping_add(4));
        frame.restore(c, S1, 0x30);
        c.ldl(V1, spv.wrapping_add(7));
        c.ldr(V1, spv);
        c.g[A0][0] = Addr(spv.wrapping_add(8)).lw(c);
        c.sdl(V1, this.wrapping_add(0x27f));
        c.sdr(V1, this.wrapping_add(0x278));
        let a0 = c.g[A0][0];
        Addr(this.wrapping_add(0x280)).sw(c, a0);
        Addr(this.wrapping_add(0x2c8)).sw(c, 0);
        c.g[V1][0] = 0x4c_0000;
        Addr(this.wrapping_add(0x2cc)).sw(c, 0);
        c.g[A3][0] = sx(at_937(c.g[V1][0], 0x6788));
        let a3 = at_937(c.g[A3][0], 0);
        c.ldl(T0, a3.wrapping_add(7));
        c.ldr(T0, a3);
        c.ldl(A0, a3.wrapping_add(0xf));
        c.ldr(A0, a3.wrapping_add(8));
        c.sdl(T0, this.wrapping_add(0x333));
        c.sdr(T0, this.wrapping_add(0x32c));
        c.sdl(A0, this.wrapping_add(0x33b));
        c.sdr(A0, this.wrapping_add(0x334));
        c.g[A0][0] = 0x4c_0000;
        frame.restore(c, S2, 0x20);
        c.g[A3][0] = sx(at_937(c.g[A0][0], 0x6798));
        let a3 = at_937(c.g[A3][0], 0);
        c.ldl(T0, a3.wrapping_add(7));
        c.ldr(T0, a3);
        c.ldl(V1, a3.wrapping_add(0xf));
        c.ldr(V1, a3.wrapping_add(8));
        c.sdl(T0, this.wrapping_add(0x343));
        c.sdr(T0, this.wrapping_add(0x33c));
        c.sdl(V1, this.wrapping_add(0x34b));
        c.sdr(V1, this.wrapping_add(0x344));
        let a1 = c.g[A1][0];
        Addr(this.wrapping_add(0x328)).sw(c, a1);
        frame.restore(c, S0, 0x40);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cFEPopupConfirm_v05(this)` (0x1c5b68): clears the word at `this+0x2b8`
/// and forwards to the base-class slot at 0x39e4a0.
pub fn c_fe_popup_confirm_v05(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let v0 = at_937(c.g[V0][0], 0x2b8);
        Addr(v0).sw(c, 0);
        call_ptr(c, 0x0039_e4a0, 0x001c_5b7c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// One block of `cFEPopupConfirm_SetContentVisible`: if the child at `this+field`
/// exists, call its slot 9 (`vt[9](child, visible)`) returning to `ret`.
fn set_content_visible_child_937(c: &mut Cpu, field: u32, ret: u32, head: u64) -> Flow {
    c.cycles += head;
    let this = at_937(c.g[S0][0], 0);
    c.g[A2][0] = Addr(this.wrapping_add(field)).lw(c);
    c.g[A1][0] = c.g[S1][0];
    if c.g[A2][0] == 0 {
        return Ok(());
    }
    c.cycles += 5;
    let child = c.g[A2][0] as u32;
    c.g[V0][0] = Addr(child.wrapping_add(8)).lw(c);
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
    let vt = c.g[V0][0] as u32;
    c.g[V1][0] = Addr(vt.wrapping_add(0x4c)).lw(c);
    c.g[A0][0] = sx(child.wrapping_add(c.g[A0][0] as u32));
    let target = c.g[V1][0] as u32;
    call_ptr(c, target, ret)
}

/// `cFEPopupConfirm_SetContentVisible(this, visible)` (0x1c5e58): passes the
/// visibility flag to the five child popups' slot-9 method.
pub fn c_fe_popup_confirm_set_content_visible(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        set_content_visible_child_937(c, 0x70, 0x001c_5e8c, 0)?;
        set_content_visible_child_937(c, 0x74, 0x001c_5eac, 3)?;
        set_content_visible_child_937(c, 0x78, 0x001c_5ecc, 3)?;
        set_content_visible_child_937(c, 0x7c, 0x001c_5eec, 3)?;
        set_content_visible_child_937(c, 0x88, 0x001c_5f0c, 3)?;
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEPopupConfirm_ComputeBackgroundScale(this, pos)` (0x1c7620): sets the
/// background scale for the popup's background type (online, blue, black).
pub fn c_fe_popup_confirm_compute_background_scale(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = Addr(at_937(c.g[S0][0], 0x31c)).lw(c);
        let src = at_937(c.g[A1][0], 0);
        c.ldl(A0, src.wrapping_add(7));
        c.ldr(A0, src);
        c.g[A2][0] = Addr(src.wrapping_add(8)).lw(c);
        let spv = sp(c);
        c.sdl(A0, spv.wrapping_add(7));
        c.sdr(A0, spv);
        let a2 = c.g[A2][0];
        Addr(spv.wrapping_add(8)).sw(c, a2);
        let t = c.g[V1][0] == c.g[V0][0];
        if t {
            // Type 1 (blue): the 1.05 bump through the float helpers.
            return cbs_blue_937(c, &frame);
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 2;
            c.cycles += 2;
            let is_two = c.g[V1][0] == c.g[V0][0];
            if is_two {
                // Black: scale 1.0.
                cbs_black_937(c);
                return cbs_b_tail_937(c, &frame);
            }
            c.cycles += 2;
            frame.restore(c, S0, 0x20);
            return cbs_epilogue_937(c, &frame);
        }
        c.cycles += 2;
        let is_zero = c.g[V1][0] == 0;
        c.lwc1(1, spv);
        if is_zero {
            // Online: scale from the position and the white background size.
            c.cycles += 14;
            c.lwc1(0, spv.wrapping_add(4));
            c.g[AT][0] = 0x4170_0000;
            c.mtc1(1, 2);
            c.g[AT][0] = 0x4120_0000;
            c.mtc1(1, 3);
            let v = fpr_937(c, 1) + fpr_937(c, 2);
            abi::set_fpr(c, 1, v);
            c.lwc1(4, at_937(c.g[S0][0], 0x1f4));
            let v = fpr_937(c, 0) + fpr_937(c, 3);
            abi::set_fpr(c, 0, v);
            c.lwc1(2, at_937(c.g[S0][0], 0x1f8));
            let v = fpr_937(c, 1).div(fpr_937(c, 4));
            abi::set_fpr(c, 1, v);
            let v = fpr_937(c, 0).div(fpr_937(c, 2));
            abi::set_fpr(c, 0, v);
            c.swc1(1, at_937(c.g[S0][0], 0x278));
            c.swc1(0, at_937(c.g[S0][0], 0x27c));
            return cbs_b_tail_937(c, &frame);
        }
        c.cycles += 2;
        frame.restore(c, S0, 0x20);
        cbs_epilogue_937(c, &frame)
    })
}

/// The `ComputeBackgroundScale` type-1 (blue) block: the scale through the
/// fixed-point helpers at 0x413af8, 0x411fd8 and 0x4127a8.
fn cbs_blue_937(c: &mut Cpu, frame: &Frame) -> Flow {
    c.cycles += 23;
    c.lwc1(12, sp(c));
    c.g[AT][0] = 0x41a0_0000;
    c.mtc1(1, 0);
    c.lwc1(1, at_937(c.g[S0][0], 0x218));
    let v = fpr_937(c, 12) + fpr_937(c, 0);
    abi::set_fpr(c, 12, v);
    let v = fpr_937(c, 12).div(fpr_937(c, 1));
    abi::set_fpr(c, 12, v);
    call_ptr(c, 0x0041_3af8, 0x001c_76d8)?;
    c.g[AT][0] = 0x0046_0000;
    c.g[A1][0] = Addr(at_937(c.g[AT][0], 0x6090)).ld(c);
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0041_1fd8, 0x001c_76e8)?;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0041_27a8, 0x001c_76f0)?;
    c.lwc1(1, sp(c).wrapping_add(4));
    c.g[AT][0] = 0x4170_0000;
    c.mtc1(1, 2);
    c.lwc1(3, at_937(c.g[S0][0], 0x21c));
    let v = fpr_937(c, 1) + fpr_937(c, 2);
    abi::set_fpr(c, 1, v);
    c.swc1(0, at_937(c.g[S0][0], 0x278));
    let v = fpr_937(c, 1).div(fpr_937(c, 3));
    abi::set_fpr(c, 1, v);
    c.swc1(1, at_937(c.g[S0][0], 0x27c));
    cbs_b_tail_937(c, frame)
}

/// The black (type 2) block: scale 1.0 in both axes.
fn cbs_black_937(c: &mut Cpu) {
    c.cycles += 4;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 0);
    c.swc1(0, at_937(c.g[S0][0], 0x27c));
    c.swc1(0, at_937(c.g[S0][0], 0x278));
}

/// The `B` tail: `cycles += 1; lq16`, then the epilogue.
fn cbs_b_tail_937(c: &mut Cpu, frame: &Frame) -> Flow {
    c.cycles += 1;
    frame.restore(c, S0, 0x20);
    cbs_epilogue_937(c, frame)
}

fn cbs_epilogue_937(c: &mut Cpu, frame: &Frame) -> Flow {
    c.cycles += 3;
    frame.restore_ra(c, 0x10);
    frame.close(c);
    Ok(())
}

/// The float at FPU register `r`.
#[inline]
fn fpr_937(c: &Cpu, r: usize) -> FpuF32 {
    FpuF32(c.f[r])
}

/// `cFEPopupConfirm_LayoutTextEntryBox(this)` (0x1c7eb0): places the text entry
/// box and its background from the body text and the two child widgets.
pub fn c_fe_popup_confirm_layout_text_entry_box(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 62;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        c.g[S0][0] = sx(at_937(sp(c) as u64, 0x30));
        frame.save_ra(c, 0x60);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S0][0];
        let this = at_937(c.g[S1][0], 0);
        c.g[A2][0] = Addr(this.wrapping_add(0x80)).lw(c);
        c.g[V0][0] = Addr(this.wrapping_add(0x6c)).lw(c);
        c.g[V1][0] = Addr(this.wrapping_add(0x84)).lw(c);
        let a2 = at_937(c.g[A2][0], 0);
        c.ldl(A0, a2.wrapping_add(0x4b));
        c.ldr(A0, a2.wrapping_add(0x44));
        c.g[A3][0] = Addr(a2.wrapping_add(0x4c)).lw(c);
        let spv = sp(c);
        c.sdl(A0, spv.wrapping_add(7));
        c.sdr(A0, spv);
        let a3 = c.g[A3][0];
        Addr(spv.wrapping_add(8)).sw(c, a3);
        let v0 = at_937(c.g[V0][0], 0);
        c.ldl(A0, v0.wrapping_add(0x4b));
        c.ldr(A0, v0.wrapping_add(0x44));
        c.g[A3][0] = Addr(v0.wrapping_add(0x4c)).lw(c);
        c.sdl(A0, spv.wrapping_add(0x27));
        c.sdr(A0, spv.wrapping_add(0x20));
        let a3 = c.g[A3][0];
        Addr(spv.wrapping_add(0x28)).sw(c, a3);
        let v1 = at_937(c.g[V1][0], 0);
        c.ldl(V0, v1.wrapping_add(0x4b));
        c.ldr(V0, v1.wrapping_add(0x44));
        c.g[A0][0] = Addr(v1.wrapping_add(0x4c)).lw(c);
        c.sdl(V0, spv.wrapping_add(0x17));
        c.sdr(V0, spv.wrapping_add(0x10));
        let a0 = c.g[A0][0];
        Addr(spv.wrapping_add(0x18)).sw(c, a0);
        c.g[V0][0] = Addr(a2.wrapping_add(8)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xa0)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xa4)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(a2.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001c_7f34)?;
        c.lwc1(2, spv.wrapping_add(0x10));
        c.g[A1][0] = c.g[S0][0];
        c.lwc1(1, spv);
        c.lwc1(0, at_937(c.g[S1][0], 0x260));
        let v = fpr_937(c, 1) - fpr_937(c, 2);
        abi::set_fpr(c, 1, v);
        c.g[AT][0] = 0x4120_0000;
        c.mtc1(1, 3);
        c.g[A2][0] = Addr(at_937(c.g[S1][0], 0x80)).lw(c);
        let v = fpr_937(c, 0) - fpr_937(c, 1);
        abi::set_fpr(c, 0, v);
        let v = fpr_937(c, 0) - fpr_937(c, 3);
        abi::set_fpr(c, 0, v);
        c.swc1(0, spv.wrapping_add(0x30));
        let a2 = at_937(c.g[A2][0], 0);
        c.g[V0][0] = Addr(a2.wrapping_add(8)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xa8)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xac)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(a2.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001c_7f74)?;
        c.g[AT][0] = 0x40c0_0000;
        c.mtc1(1, 6);
        c.lwc1(0, spv.wrapping_add(0x30));
        c.lwc1(2, at_937(c.g[S1][0], 0x20c));
        let v = fpr_937(c, 0) + fpr_937(c, 6);
        abi::set_fpr(c, 0, v);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 1);
        c.g[V0][0] = Addr(at_937(c.g[S1][0], 0x8c)).lw(c);
        c.swc1(1, spv.wrapping_add(0x48));
        let v = fpr_937(c, 0).div(fpr_937(c, 2));
        abi::set_fpr(c, 0, v);
        c.swc1(1, spv.wrapping_add(0x44));
        let is_zero = c.g[V0][0] == 0;
        c.swc1(0, spv.wrapping_add(0x40));
        if is_zero {
            return layout_text_entry_epilogue_937(c, &frame);
        }
        c.cycles += 37;
        c.lwc1(2, at_937(c.g[S1][0], 0x210));
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 4);
        c.ldl(T1, spv.wrapping_add(7));
        c.ldr(T1, spv);
        c.g[V1][0] = Addr(spv.wrapping_add(8)).lw(c);
        c.sdl(T1, spv.wrapping_add(0x57));
        c.sdr(T1, spv.wrapping_add(0x50));
        Addr(spv.wrapping_add(0x58)).sw(c, c.g[V1][0]);
        let v = fpr_937(c, 2) * fpr_937(c, 4);
        abi::set_fpr(c, 2, v);
        c.g[AT][0] = 0x4040_0000;
        c.mtc1(1, 5);
        c.lwc1(0, spv.wrapping_add(0x54));
        c.lwc1(1, spv.wrapping_add(0x30));
        let v = fpr_937(c, 0) + fpr_937(c, 2);
        abi::set_fpr(c, 0, v);
        c.lwc1(3, spv.wrapping_add(0x24));
        let v = fpr_937(c, 1) + fpr_937(c, 6);
        abi::set_fpr(c, 1, v);
        c.lwc1(2, spv.wrapping_add(0x50));
        let v = fpr_937(c, 0) + fpr_937(c, 3);
        abi::set_fpr(c, 0, v);
        let v = fpr_937(c, 1) * fpr_937(c, 4);
        abi::set_fpr(c, 1, v);
        let v = fpr_937(c, 0) - fpr_937(c, 5);
        abi::set_fpr(c, 0, v);
        let v = fpr_937(c, 2) + fpr_937(c, 1);
        abi::set_fpr(c, 2, v);
        c.swc1(0, spv.wrapping_add(0x54));
        c.swc1(2, spv.wrapping_add(0x50));
        c.ldl(A2, spv.wrapping_add(0x57));
        c.ldr(A2, spv.wrapping_add(0x50));
        c.g[A3][0] = Addr(spv.wrapping_add(0x58)).lw(c);
        let dst = at_937(c.g[V0][0], 0);
        c.sdl(A2, dst.wrapping_add(0x4b));
        c.sdr(A2, dst.wrapping_add(0x44));
        Addr(dst.wrapping_add(0x4c)).sw(c, c.g[A3][0]);
        c.g[V0][0] = Addr(at_937(c.g[S1][0], 0x8c)).lw(c);
        c.ldl(V1, spv.wrapping_add(0x47));
        c.ldr(V1, spv.wrapping_add(0x40));
        c.g[A0][0] = Addr(spv.wrapping_add(0x48)).lw(c);
        let dst = at_937(c.g[V0][0], 0);
        c.sdl(V1, dst.wrapping_add(0x57));
        c.sdr(V1, dst.wrapping_add(0x50));
        Addr(dst.wrapping_add(0x58)).sw(c, c.g[A0][0]);
        layout_text_entry_epilogue_937(c, &frame)
    })
}

/// Epilogue of `LayoutTextEntryBox`: `cycles += 5`, restore s0/s1/ra, return.
fn layout_text_entry_epilogue_937(c: &mut Cpu, frame: &Frame) -> Flow {
    c.cycles += 5;
    frame.restore(c, S0, 0x80);
    frame.restore(c, S1, 0x70);
    frame.restore_ra(c, 0x60);
    frame.close(c);
    Ok(())
}

/// `cFEPopupConfirm_DescToString(this, dst, desc)` (0x1c80e8): the popup's
/// description element (kind 0 empty, 1 unicode string, 2 char string, 3 a
/// localised string or a printf of its id) copied into the string `dst`.
pub fn c_fe_popup_confirm_desc_to_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = 1;
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        frame.save_ra(c, 0x40);
        c.g[V1][0] = Addr(c.g[S2][0] as u32).lw(c);
        let t = c.g[V1][0] == c.g[V0][0];
        c.g[S3][0] = c.g[A1][0];
        if t {
            return desc_case1_937(c, frame);
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            let is_zero = c.g[V1][0] == 0;
            frame.restore(c, S0, 0x80);
            if is_zero {
                return desc_case0_937(c, frame);
            }
            c.cycles += 2;
            frame.restore(c, S1, 0x70);
            return desc_epilogue_937(c, frame);
        }
        c.g[V0][0] = 2;
        c.cycles += 2;
        let is_two = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 3;
        if is_two {
            return desc_case2_937(c, frame);
        }
        c.cycles += 2;
        let is_three = c.g[V1][0] == c.g[V0][0];
        frame.restore(c, S0, 0x80);
        if is_three {
            return desc_case3_937(c, frame);
        }
        c.cycles += 2;
        frame.restore(c, S1, 0x70);
        desc_epilogue_937(c, frame)
    })
}

/// The `L1` tail of DescToString: `cycles += 1; lq17`, then the epilogue.
fn desc_l1_tail_937(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    frame.restore(c, S1, 0x70);
    desc_epilogue_937(c, frame)
}

/// The epilogue of DescToString: `cycles += 5`, restore s2/s3/ra, return.
fn desc_epilogue_937(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 5;
    frame.restore(c, S2, 0x60);
    frame.restore(c, S3, 0x50);
    frame.restore_ra(c, 0x40);
    frame.close(c);
    Ok(())
}

/// Kind 0: the empty string.
fn desc_case0_937(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 5;
    c.g[A1][0] = sx(0x004a_1408);
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, 0x0031_84e8, 0x001c_823c)?;
    desc_l1_tail_937(c, frame)
}

/// Kind 2: a char string, copied straight into `dst`.
fn desc_case2_937(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 5;
    c.g[A1][0] = Addr(at_937(c.g[S2][0], 4)).lw(c);
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, 0x0031_84e8, 0x001c_8180)?;
    frame.restore(c, S0, 0x80);
    desc_l1_tail_937(c, frame)
}

/// Kind 1: a unicode string converted into a new buffer.
fn desc_case1_937(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 11;
    c.g[A0][0] = Addr(at_937(c.g[S2][0], 4)).lw(c);
    call_ptr(c, 0x003d_bca0, 0x001c_8150)?;
    c.g[A1][0] = 0x0046_0000;
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6098));
    c.g[A2][0] = 0x100;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7e30, 0x001c_8168)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = Addr(at_937(c.g[S2][0], 4)).lw(c);
    desc_copy_tail_937(c, frame)
}

/// Kind 3: a localised string through the string table (`vt[4]` of the
/// world's object); when there is none, a printf of its id.
fn desc_case3_937(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 11;
    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[A1][0] = Addr(at_937(c.g[S2][0], 4)).lw(c);
    c.g[A2][0] = Addr(at_937(c.g[V0][0], 0x8c)).lw(c);
    c.g[V1][0] = Addr(at_937(c.g[A2][0], 4)).lw(c);
    c.g[A0][0] = Addr(at_937(c.g[V1][0], 0x20)).lh(c);
    c.g[V0][0] = Addr(at_937(c.g[V1][0], 0x24)).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x001c_81a8)?;
    c.g[S1][0] = c.g[V0][0];
    if c.g[S1][0] == 0 {
        c.g[A2][0] = Addr(at_937(c.g[S2][0], 4)).lw(c);
        // printf the id into a stack buffer, then copy it.
        c.cycles += 9;
        c.g[A1][0] = 0x0046_0000;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x60a8));
        call_ptr(c, 0x0041_7828, 0x001c_8218)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x0031_84e8, 0x001c_8224)?;
        frame.restore(c, S0, 0x80);
        return desc_l1_tail_937(c, frame);
    }
    c.cycles += 10;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x003d_bca0, 0x001c_81bc)?;
    c.g[A1][0] = 0x0046_0000;
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6098));
    c.g[A2][0] = 0x100;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7e30, 0x001c_81d4)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S1][0];
    desc_copy_tail_937(c, frame)
}

/// The shared tail of kinds 1 and 3: copy the source into the new buffer
/// (`S0`), set `dst` from it, free the buffer, and return.
fn desc_copy_tail_937(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 7;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x002c_2580, 0x001c_81e4)?;
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = c.g[S0][0];
    call_ptr(c, 0x0031_84e8, 0x001c_81f0)?;
    if c.g[S0][0] != 0 {
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0031_7e98, 0x001c_8200)?;
    }
    frame.restore(c, S0, 0x80);
    desc_l1_tail_937(c, frame)
}

/// `cFEPopupConfirm_MeasureDesc(self, text, desc, out)` (0x1c8320): scales the
/// text's size vector and measures the description into `out` (width, height).
pub fn c_fe_popup_confirm_measure_desc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x60);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x90);
        c.g[A0][0] = c.g[S1][0];
        frame.save_ra(c, 0x50);
        let text = c.g[S1][0] as u32;
        c.ldl(V0, text.wrapping_add(0x57));
        c.ldr(V0, text.wrapping_add(0x50));
        c.g[V1][0] = Addr(text.wrapping_add(0x58)).lw(c);
        let spv = sp(c);
        c.sdl(V0, spv.wrapping_add(7));
        c.sdr(V0, spv);
        let v1 = c.g[V1][0];
        Addr(spv.wrapping_add(8)).sw(c, v1);
        c.g[RA][0] = sx(0x001c_8364);
        c.g[S3][0] = c.g[A3][0];
        call_ptr(c, 0x003a_04f0, 0x001c_8364)?;
        let g = c.g[V0][0] as u32;
        c.lwc1(2, spv);
        c.g[V1][0] = 1;
        c.lwc1(3, spv.wrapping_add(4));
        c.lwc1(0, g.wrapping_add(0x30));
        c.lwc1(1, g.wrapping_add(0x34));
        let v = fpr_937(c, 0) * fpr_937(c, 2);
        abi::set_fpr(c, 0, v);
        let v = fpr_937(c, 1) * fpr_937(c, 3);
        abi::set_fpr(c, 1, v);
        c.swc1(0, g.wrapping_add(0x38));
        c.swc1(1, g.wrapping_add(0x3c));
        c.g[A0][0] = Addr(c.g[S2][0] as u32).lw(c);
        let t = c.g[A0][0] == c.g[V1][0];
        c.g[V0][0] = ((c.g[A0][0] as i64) < 2) as u64;
        if t {
            return measure_case1_937(c, &frame);
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 2;
        } else {
            c.cycles += 2;
            let is_zero = c.g[A0][0] == 0;
            frame.restore(c, S0, 0x90);
            if is_zero {
                return measure_case0_937(c, &frame);
            }
            c.cycles += 2;
            frame.restore(c, S1, 0x80);
            return measure_epilogue_937(c, &frame);
        }
        c.cycles += 2;
        let is_two = c.g[A0][0] == c.g[V0][0];
        c.g[V0][0] = 3;
        if is_two {
            return measure_case2_937(c, &frame);
        }
        c.cycles += 2;
        let is_three = c.g[A0][0] == c.g[V0][0];
        frame.restore(c, S0, 0x90);
        if is_three {
            return measure_case3_937(c, &frame);
        }
        c.cycles += 2;
        frame.restore(c, S1, 0x80);
        measure_epilogue_937(c, &frame)
    })
}

/// The measure epilogue: `cycles += 5`, restore s2/s3/ra, return.
fn measure_epilogue_937(c: &mut Cpu, frame: &Frame) -> Flow {
    c.cycles += 5;
    frame.restore(c, S2, 0x70);
    frame.restore(c, S3, 0x60);
    frame.restore_ra(c, 0x50);
    frame.close(c);
    Ok(())
}

/// The `L1` tail of MeasureDesc: `cycles += 1; lq17`, then the epilogue.
fn measure_l1_tail_937(c: &mut Cpu, frame: &Frame) -> Flow {
    c.cycles += 1;
    frame.restore(c, S1, 0x80);
    measure_epilogue_937(c, frame)
}

/// Kind 0: an empty description measures 0 x 0.
fn measure_case0_937(c: &mut Cpu, frame: &Frame) -> Flow {
    c.cycles += 2;
    Addr(at_937(c.g[S3][0], 0xc)).sw(c, 0);
    Addr(at_937(c.g[S3][0], 8)).sw(c, 0);
    measure_l1_tail_937(c, frame)
}

/// Kind 1: a unicode string drawn by the text helper at 0x3921f0.
fn measure_case1_937(c: &mut Cpu, frame: &Frame) -> Flow {
    c.cycles += 5;
    c.g[RA][0] = sx(0x001c_83cc);
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x003a_04f0, 0x001c_83cc)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = Addr(at_937(c.g[S2][0], 4)).lw(c);
    measure_draw1_937(c, frame)
}

/// The kind-1 draw (0x3921f0) and its tail.
fn measure_draw1_937(c: &mut Cpu, frame: &Frame) -> Flow {
    c.cycles += 7;
    c.lwc1(12, at_937(c.g[A0][0], 0x38));
    c.g[A2][0] = c.g[S3][0];
    c.lwc1(13, at_937(c.g[A0][0], 0x3c));
    c.g[A3][0] = 0;
    call_ptr(c, 0x0039_21f0, 0x001c_843c)?;
    frame.restore(c, S0, 0x90);
    measure_l1_tail_937(c, frame)
}

/// Kind 2: a char string drawn by the text helper at 0x391fb0.
fn measure_case2_937(c: &mut Cpu, frame: &Frame) -> Flow {
    c.cycles += 5;
    c.g[RA][0] = sx(0x001c_83e0);
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x003a_04f0, 0x001c_83e0)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = Addr(at_937(c.g[S2][0], 4)).lw(c);
    measure_draw2_937(c, frame)
}

/// The kind-2 draw (0x391fb0) and its tail.
fn measure_draw2_937(c: &mut Cpu, frame: &Frame) -> Flow {
    c.cycles += 7;
    c.lwc1(12, at_937(c.g[A0][0], 0x38));
    c.g[A2][0] = c.g[S3][0];
    c.lwc1(13, at_937(c.g[A0][0], 0x3c));
    c.g[A3][0] = 0;
    call_ptr(c, 0x0039_1fb0, 0x001c_847c)?;
    frame.restore(c, S0, 0x90);
    measure_l1_tail_937(c, frame)
}

/// Kind 3: a localised string through the string table; a printf of the id
/// when the table has none.
fn measure_case3_937(c: &mut Cpu, frame: &Frame) -> Flow {
    c.cycles += 11;
    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[A1][0] = Addr(at_937(c.g[S2][0], 4)).lw(c);
    c.g[A2][0] = Addr(at_937(c.g[V0][0], 0x8c)).lw(c);
    c.g[V1][0] = Addr(at_937(c.g[A2][0], 4)).lw(c);
    c.g[A0][0] = Addr(at_937(c.g[V1][0], 0x20)).lh(c);
    c.g[V0][0] = Addr(at_937(c.g[V1][0], 0x24)).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[RA][0] = sx(0x001c_840c);
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x001c_840c)?;
    c.g[S0][0] = c.g[V0][0];
    if c.g[S0][0] == 0 {
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        return measure_sprintf_937(c, frame);
    }
    c.cycles += 4;
    c.g[RA][0] = sx(0x001c_8420);
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x003a_04f0, 0x001c_8420)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0];
    measure_draw1_937(c, frame)
}

/// The printf path of kind 3: format the id into the stack buffer at `S0`,
/// then draw it.
fn measure_sprintf_937(c: &mut Cpu, frame: &Frame) -> Flow {
    c.cycles += 9;
    c.g[A2][0] = Addr(at_937(c.g[S2][0], 4)).lw(c);
    c.g[A1][0] = 0x0046_0000;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x60a8));
    call_ptr(c, 0x0041_7828, 0x001c_8458)?;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x003a_04f0, 0x001c_8460)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0];
    measure_draw2_937(c, frame)
}

/// `cFEPopupConfirm_LayoutTitle(this, info)` (0x1c8678): sets the title text
/// from the info element, then lays it out and positions it when the popup
/// has a second line.
pub fn c_fe_popup_confirm_layout_title(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[A3][0] = Addr(at_937(c.g[S0][0], 0x7c)).lw(c);
        if c.g[A3][0] == 0 {
            frame.restore(c, S0, 0x30);
            return layout_title_tail_937(c, &frame);
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(at_937(c.g[A1][0], 0xc)).lw(c);
        c.g[S1][0] = sx(at_937(c.g[A1][0], 0xc));
        if c.g[V0][0] == 0 {
            // No info: the text gets an empty value (vt[9](text, 0)).
            c.cycles += 6;
            c.g[V0][0] = Addr(at_937(c.g[A3][0], 8)).lw(c);
            c.g[A1][0] = 0;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x001c_8740);
            c.g[A0][0] = sx(at_937(c.g[A3][0], 0).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001c_8740)?;
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
            return layout_title_tail_937(c, &frame);
        }
        c.cycles += 26;
        c.g[A1][0] = c.g[A3][0];
        c.g[A2][0] = c.g[S1][0];
        call_ptr(c, 0x001c_8050, 0x001c_86b0)?;
        c.g[A2][0] = Addr(at_937(c.g[S0][0], 0x7c)).lw(c);
        c.g[A1][0] = 1;
        c.g[V0][0] = Addr(at_937(c.g[A2][0], 8)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(at_937(c.g[A2][0], 0).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001c_86cc)?;
        c.g[A1][0] = Addr(at_937(c.g[S0][0], 0x7c)).lw(c);
        c.g[A3][0] = c.g[SP][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001c_8320, 0x001c_86e0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(at_937(c.g[S0][0], 0x260));
        c.g[A2][0] = c.g[SP][0];
        call_ptr(c, 0x001c_9b68, 0x001c_86f0)?;
        let s0 = at_937(c.g[S0][0], 0);
        c.lwc1(0, s0.wrapping_add(0x264));
        c.g[AT][0] = 0x40a0_0000;
        c.mtc1(1, 1);
        c.g[V0][0] = Addr(s0.wrapping_add(0x2cc)).lw(c);
        let v = fpr_937(c, 0) + fpr_937(c, 1);
        abi::set_fpr(c, 0, v);
        let is_zero = c.g[V0][0] == 0;
        c.swc1(0, s0.wrapping_add(0x264));
        if is_zero {
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
            return layout_title_tail_937(c, &frame);
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S0][0];
        c.mtc1(0, 12);
        c.g[A1][0] = Addr(at_937(c.g[A0][0], 0x7c)).lw(c);
        c.g[RA][0] = sx(0x001c_8720);
        c.g[A2][0] = 0x11;
        call_ptr(c, 0x001c_8930, 0x001c_8720)?;
        frame.restore(c, S0, 0x30);
        layout_title_tail_937(c, &frame)
    })
}

/// Epilogue of `LayoutTitle`: `cycles += 4`, restore s1/ra, return.
fn layout_title_tail_937(c: &mut Cpu, frame: &Frame) -> Flow {
    c.cycles += 4;
    frame.restore(c, S1, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{fall_back, hook};
    use crate::difftest::{check, elf_image, fakes, fill, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The popup object (0x360 bytes) in the fixture area.
    const POPUP_937: u32 = DATA + 0x20_0000;

    /// Gate word of the LayoutTitle fakes, in the machine's own RAM.
    const LAYOUT_TITLE_GATE_937: u32 = DATA + 0x2e_0000;
    const LAYOUT_TITLE_MAGIC_937: u32 = 0x9370_0c0d;

    static PREV_SET_TEXT_937: AtomicUsize = AtomicUsize::new(0);
    static PREV_ACCUMULATE_937: AtomicUsize = AtomicUsize::new(0);
    static PREV_SET_WIDGET_ALIGN_937: AtomicUsize = AtomicUsize::new(0);

    /// SetTextFromDesc (0x1c8050) with the gate on: returns at once. With the
    /// gate off it runs the body found in the slot when it was hooked.
    fn layout_title_set_text_fake_937(c: &mut Cpu) {
        if c.bus.read32(LAYOUT_TITLE_GATE_937) != LAYOUT_TITLE_MAGIC_937 {
            return fall_back(c, &PREV_SET_TEXT_937, ee::s03::f_001c8050__generated);
        }
        abi::ret(c);
    }

    /// AccumulateSize (0x1c9b68) with the gate on: returns at once.
    fn layout_title_accumulate_fake_937(c: &mut Cpu) {
        if c.bus.read32(LAYOUT_TITLE_GATE_937) != LAYOUT_TITLE_MAGIC_937 {
            return fall_back(c, &PREV_ACCUMULATE_937, ee::s03::f_001c9b68__generated);
        }
        abi::ret(c);
    }

    /// SetWidgetAlign (0x1c8930) with the gate on: returns at once.
    fn layout_title_set_widget_align_fake_937(c: &mut Cpu) {
        if c.bus.read32(LAYOUT_TITLE_GATE_937) != LAYOUT_TITLE_MAGIC_937 {
            return fall_back(
                c,
                &PREV_SET_WIDGET_ALIGN_937,
                ee::s03::f_001c8930__generated,
            );
        }
        abi::ret(c);
    }

    /// Puts the three gated fakes in their override slots (once per process).
    fn layout_title_install_937() {
        hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C8050),
            layout_title_set_text_fake_937,
            &PREV_SET_TEXT_937,
        );
        hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C9B68),
            layout_title_accumulate_fake_937,
            &PREV_ACCUMULATE_937,
        );
        hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C8930),
            layout_title_set_widget_align_fake_937,
            &PREV_SET_WIDGET_ALIGN_937,
        );
    }

    #[test]
    fn c_fe_popup_confirm_base_ctor_matches() {
        check(
            ee::s03::f_001c58e8__generated,
            c_fe_popup_confirm_base_ctor,
            6,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x1c58e8, 0);
                elf_image(c);
                fill(c, &mut r, POPUP_937, 0xd8);
                c.bus.write32(GAME_GP.wrapping_add(0xda0), r.next_u32());
                c.g[A0][0] = POPUP_937 as u64;
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_v05_matches() {
        check(
            ee::s03::f_001c5b68__generated,
            c_fe_popup_confirm_v05,
            8,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x1c5b68, 0);
                fill(c, &mut r, POPUP_937, 0xd8);
                c.g[A0][0] = POPUP_937 as u64;
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_set_content_visible_matches() {
        check(
            ee::s03::f_001c5e58__generated,
            c_fe_popup_confirm_set_content_visible,
            16,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x1c5e58, 5);
                let vt = fakes::vtable(c, &mut r, &[0x48]);
                for (k, field) in [0x70u32, 0x74, 0x78, 0x7c, 0x88].iter().enumerate() {
                    let child = POPUP_937 + 0x1000 + 0x40 * k as u32;
                    c.bus.write32(child + 8, vt);
                    let present = (seed >> k) & 1 == 0;
                    c.bus
                        .write32(POPUP_937 + field, if present { child } else { 0 });
                }
                c.g[A0][0] = POPUP_937 as u64;
                c.g[A1][0] = (seed & 1) as u64;
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_compute_background_scale_matches() {
        check(
            ee::s03::f_001c7620__generated,
            c_fe_popup_confirm_compute_background_scale,
            24,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x1c7620, 0);
                fill(c, &mut r, POPUP_937, 0xd8);
                let ty = match seed % 5 {
                    4 => u32::MAX,
                    k => k,
                };
                c.bus.write32(POPUP_937 + 0x31c, ty);
                c.bus.write32(POPUP_937 + 0x218, r.f32(4.0));
                c.bus.write32(POPUP_937 + 0x21c, r.f32(4.0));
                c.bus.write32(POPUP_937 + 0x1f4, r.f32(4.0));
                c.bus.write32(POPUP_937 + 0x1f8, r.f32(4.0));
                let pos = POPUP_937 + 0x2000;
                c.bus.write32(pos, r.f32(100.0));
                c.bus.write32(pos + 4, r.f32(100.0));
                c.bus.write32(pos + 8, r.next_u32());
                c.g[A0][0] = POPUP_937 as u64;
                c.g[A1][0] = pos as u64;
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_layout_text_entry_box_matches() {
        check(
            ee::s03::f_001c7eb0__generated,
            c_fe_popup_confirm_layout_text_entry_box,
            16,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x1c7eb0, 0);
                let vt = fakes::vtable(c, &mut r, &[0xa0, 0xa8]);
                fill(c, &mut r, POPUP_937, 0xd8);
                let o80 = POPUP_937 + 0x1000;
                let o6c = POPUP_937 + 0x1100;
                let o84 = POPUP_937 + 0x1200;
                let o8c = POPUP_937 + 0x1300;
                for o in [o80, o6c, o84, o8c] {
                    fill(c, &mut r, o, 0x40);
                }
                c.bus.write32(o80 + 8, vt);
                c.bus.write32(POPUP_937 + 0x80, o80);
                c.bus.write32(POPUP_937 + 0x6c, o6c);
                c.bus.write32(POPUP_937 + 0x84, o84);
                c.bus
                    .write32(POPUP_937 + 0x8c, if seed % 2 == 0 { 0 } else { o8c });
                c.bus.write32(o8c + 0x4, 0);
                c.g[A0][0] = POPUP_937 as u64;
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_desc_to_string_matches() {
        check(
            ee::s03::f_001c80e8__generated,
            c_fe_popup_confirm_desc_to_string,
            20,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x1c80e8, 2);
                elf_image(c);
                let desc = POPUP_937 + 0x2000;
                let text = POPUP_937 + 0x2100;
                let dst = POPUP_937 + 0x2200;
                let ty = match seed % 5 {
                    4 => 9,
                    k => k,
                };
                c.bus.write32(desc, ty);
                c.bus.write32(desc + 4, text);
                c.bus.write16(text, 0x41);
                c.bus.write16(text + 2, 0x42);
                c.bus.write16(text + 4, 0);
                c.bus.write8(text + 8, 0);
                let vt = fakes::vtable(c, &mut r, &[0x20]);
                let world = POPUP_937 + 0x3000;
                let obj = POPUP_937 + 0x3100;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x8c, obj);
                c.bus.write32(obj + 4, vt);
                c.g[A0][0] = POPUP_937 as u64;
                c.g[A1][0] = dst as u64;
                c.g[A2][0] = desc as u64;
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_measure_desc_matches() {
        check(
            ee::s03::f_001c8320__generated,
            c_fe_popup_confirm_measure_desc,
            16,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x1c8320, 2);
                elf_image(c);
                let text = POPUP_937 + 0x3000;
                fill(c, &mut r, text, 0x40);
                let desc = POPUP_937 + 0x2000;
                let ty = match seed % 5 {
                    4 => 9,
                    k => k,
                };
                c.bus.write32(desc, ty);
                c.bus.write32(desc + 4, POPUP_937 + 0x2100);
                let vt = fakes::vtable(c, &mut r, &[0x20]);
                let world = POPUP_937 + 0x4000;
                let obj = POPUP_937 + 0x4100;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x8c, obj);
                c.bus.write32(obj + 4, vt);
                c.g[A0][0] = POPUP_937 as u64;
                c.g[A1][0] = text as u64;
                c.g[A2][0] = desc as u64;
                c.g[A3][0] = (POPUP_937 + 0x2800) as u64;
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_layout_title_matches() {
        check(
            ee::s03::f_001c8678__generated,
            c_fe_popup_confirm_layout_title,
            16,
            &|c, seed| {
                layout_title_install_937();
                let mut r = fakes::machine_rt(c, seed, 0x1c8678, 3);
                c.bus.write32(LAYOUT_TITLE_GATE_937, LAYOUT_TITLE_MAGIC_937);
                elf_image(c);
                fill(c, &mut r, POPUP_937, 0xd8);
                let text = POPUP_937 + 0x3000;
                fill(c, &mut r, text, 0x40);
                let vt = fakes::vtable(c, &mut r, &[0x48]);
                c.bus.write32(text + 8, vt);
                c.bus
                    .write32(POPUP_937 + 0x7c, if seed % 7 == 0 { 0 } else { text });
                c.bus
                    .write32(POPUP_937 + 0x2cc, if seed % 3 == 0 { 0 } else { 1 });
                let info = POPUP_937 + 0x2000;
                c.bus
                    .write32(info + 0xc, [0u32, 1, 2, 3][(seed % 4) as usize]);
                c.bus.write32(info + 0x10, POPUP_937 + 0x2100);
                c.g[A0][0] = POPUP_937 as u64;
                c.g[A1][0] = info as u64;
            },
        );
    }
}
