//! Staging module oc1083 (wave wave267): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1083`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, VSlot};
use crate::events::{deliver_event_here, event_due};
use crate::game::ui::uiengine::{UI_LIST_FIRST, UI_LIST_NODE_IS_SENTINEL};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0039_e130, c_ui_slider_render, "cUISlider_Render"),
    (
        0x0039_e778,
        ui_state_hide_flagged_screens,
        "UIState_HideFlaggedScreens",
    ),
    (
        0x0039_e7f0,
        ui_state_show_flagged_screens,
        "UIState_ShowFlaggedScreens",
    ),
    (
        0x0039_fd80,
        ui_object_set_anim_progress,
        "UIObject_SetAnimProgress",
    ),
    (0x0039_fdd8, ui_object_is_anim_done, "UIObject_IsAnimDone"),
    (
        0x0039_ffa0,
        ui_object_start_event_thread,
        "UIObject_StartEventThread",
    ),
    (0x003a_10f0, ui_text_get_cached_size, "UIText_GetCachedSize"),
    (0x003a_11b8, c_ui_text_scroll_load, "cUITextScroll_Load"),
];

// Callees (each is the generated function's readable alias, so a replaced
// callee is reached through its override slot).
const OC1083_UIANIM_FRAME_COUNT: Callee = Callee {
    addr: 0x0039_7458,
    f: ee::s13::f_00397458,
};
const OC1083_UIANIM_IS_DONE: Callee = Callee {
    addr: 0x0039_7440,
    f: ee::s13::f_00397440,
};
const OC1083_UISCREEN_START_THREAD: Callee = Callee {
    addr: 0x0039_c870,
    f: ee::s13::f_0039c870,
};
const OC1083_UIOBJECT_LOAD: Callee = Callee {
    addr: 0x0039_fe00,
    f: ee::s13::f_0039fe00,
};
const OC1083_UITS_CLEAR_LINES: Callee = Callee {
    addr: 0x003a_18c0,
    f: ee::s13::f_003a18c0,
};
const OC1083_UITS_SET_TEXT_ID: Callee = Callee {
    addr: 0x003a_1f18,
    f: ee::s13::f_003a1f18,
};

/// `void cUISlider::render2D(cUISlider* self, float dx, float dy)` (0x39e130).
/// The slider keeps two vtable objects: the first end at `+0x88` and the
/// second at `+0x8c`. Both are drawn at the slider's position plus (dx, dy).
/// The second end is first moved along the first end's track: its x becomes
/// `pos.x + (s1.x - s2.x) / (count - 1) * step`, with `step` at `+0x74` and
/// `count` at `+0x78`, where s1 and s2 are the two ends' sizes.
pub fn c_ui_slider_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x50);
        frame.save_fpr(c, 21, 0x68);
        frame.save_fpr(c, 20, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x40);
        let this = c.g[S0][0] as u32;
        c.g[A1][0] = Addr(this).off(0x88).lw(c);
        if c.g[A1][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(this).off(0x8c).lw(c);
            if c.g[V0][0] != 0 {
                c_ui_slider_render_ends(c, this)?;
            }
        }
        frame.restore(c, S0, 0x50);
        c.cycles += 5;
        frame.restore_ra(c, 0x40);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// The body of [`c_ui_slider_render`] once both ends exist: render the first
/// end, size both ends, place the second end, render it.
fn c_ui_slider_render_ends(c: &mut Cpu, this: u32) -> Flow {
    let s = sp(c);
    let a = c.g[A1][0] as u32;
    c.cycles += 68;
    c.lwc1(20, this.wrapping_add(0x44));
    c.lwc1(21, this.wrapping_add(0x48));
    c.g[V0][0] = Addr(a).off(8).lw(c);
    c.add_s(20, 12, 20);
    c.add_s(21, 13, 21);
    let vt = c.g[V0][0];
    let end = VSlot::read(c, vt, 0x88);
    c.g[A0][0] = end.delta;
    c.g[V1][0] = end.func;
    c.mov_s(12, 20);
    c.g[A0][0] = end.this(c.g[A1][0]);
    let t = c.g[V1][0] as u32;
    c.mov_s(13, 21);
    call_ptr(c, t, 0x0039_e18c)?;

    // First end drawn. Copy its position (12 bytes at +0x44) to the stack
    // at sp+0, then size it into s1 (sp+0x10).
    let o = Addr(this).off(0x88).lw(c) as u32;
    c.g[A0][0] = o as u64;
    c.g[A1][0] = sx(s.wrapping_add(0x10));
    c.ldl(2, o.wrapping_add(0x4b));
    c.ldr(2, o.wrapping_add(0x44));
    c.g[V1][0] = Addr(o).off(0x4c).lw(c);
    c.sdl(2, s.wrapping_add(7));
    c.sdr(2, s);
    let v1 = c.g[V1][0];
    Addr(s).off(8).sw(c, v1);
    c.g[V0][0] = Addr(o).off(8).lw(c);
    let vt = c.g[V0][0];
    let size1 = VSlot::read(c, vt, 0xa0);
    c.g[V1][0] = size1.delta;
    c.g[A2][0] = size1.func;
    let t = c.g[A2][0] as u32;
    c.g[A0][0] = size1.this(c.g[A0][0]);
    call_ptr(c, t, 0x0039_e1c0)?;

    // Second end sized into s2 (sp+0x20).
    c.g[A3][0] = Addr(this).off(0x8c).lw(c);
    c.g[A1][0] = sx(s.wrapping_add(0x20));
    let b = c.g[A3][0] as u32;
    c.g[V0][0] = Addr(b).off(8).lw(c);
    let vt = c.g[V0][0];
    let size2 = VSlot::read(c, vt, 0xa0);
    c.g[A0][0] = size2.delta;
    c.g[V1][0] = size2.func;
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = size2.this(c.g[A3][0]);
    call_ptr(c, t, 0x0039_e1dc)?;

    // Place the second end: x = pos.x + (s1.x - s2.x) / (count - 1) * step.
    c.lwc1(2, this.wrapping_add(0x78));
    c.cvt_s_w(2, 2);
    c.lwc1(0, s.wrapping_add(0x20));
    c.lwc1(1, s.wrapping_add(0x10));
    c.lwc1(4, this.wrapping_add(0x74));
    c.cvt_s_w(4, 4);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 3);
    c.mov_s(12, 20);
    c.sub_s(1, 1, 0);
    c.g[V0][0] = Addr(this).off(0x8c).lw(c);
    c.sub_s(2, 2, 3);
    c.mov_s(13, 21);
    let b = c.g[V0][0] as u32;
    c.ldl(3, b.wrapping_add(0x4b));
    c.ldr(3, b.wrapping_add(0x44));
    c.g[A0][0] = Addr(b).off(0x4c).lw(c);
    c.sdl(3, s.wrapping_add(0x37));
    c.sdr(3, s.wrapping_add(0x30));
    let a0 = c.g[A0][0];
    Addr(s).off(0x38).sw(c, a0);
    c.div_s(1, 1, 2);
    c.lwc1(0, s);
    c.mul_s(1, 1, 4);
    c.add_s(0, 0, 1);
    c.swc1(0, s.wrapping_add(0x30));
    c.ldl(7, s.wrapping_add(0x37));
    c.ldr(7, s.wrapping_add(0x30));
    c.g[V1][0] = Addr(s).off(0x38).lw(c);
    c.sdl(7, b.wrapping_add(0x4b));
    c.sdr(7, b.wrapping_add(0x44));
    let v1 = c.g[V1][0];
    Addr(b).off(0x4c).sw(c, v1);

    // Render the second end.
    c.g[A1][0] = Addr(this).off(0x8c).lw(c);
    let b2 = c.g[A1][0];
    c.g[V0][0] = Addr(b2 as u32).off(8).lw(c);
    let vt = c.g[V0][0];
    let draw2 = VSlot::read(c, vt, 0x88);
    c.g[A0][0] = draw2.delta;
    c.g[V1][0] = draw2.func;
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = draw2.this(b2);
    call_ptr(c, t, 0x0039_e26c)?;
    Ok(())
}

/// The flagged-screen walk shared by the two `UIState_*FlaggedScreens`:
/// for each screen in the state's list (`+0x24`) whose flags (`+0x54`) have
/// bit 6 set, call its vtable slot 9 with `show` (0 hides, 1 shows).
/// `rets` are the return addresses of the list-first call, the slot call,
/// the sentinel test, and the loop head (the event resume address).
fn ui_state_flagged_screens(c: &mut Cpu, show: u64, rets: [u32; 4]) -> Flow {
    let [ret_first, ret_method, ret_sentinel, head] = rets;
    c.cycles += 8;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    let this = c.g[A0][0] as u32;
    c.g[A0][0] = sx(this.wrapping_add(0x24));
    UI_LIST_FIRST.call(c, ret_first)?;
    c.g[S0][0] = c.g[V0][0];
    if c.g[S0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x54).lw(c);
        loop {
            c.cycles += 4;
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32) & 1;
            let node = c.g[S0][0] as u32;
            c.g[A2][0] = sx(node.wrapping_add(0x40));
            if c.g[V0][0] != 0 {
                c.cycles += 6;
                c.g[V0][0] = Addr(node).off(0x48).lw(c);
                let vt = c.g[V0][0];
                c.g[A1][0] = show;
                let slot = VSlot::read(c, vt, 0x48);
                c.g[V1][0] = slot.func;
                c.g[A0][0] = slot.this(c.g[A2][0]);
                call_ptr(c, slot.func as u32, ret_method)?;
            }
            c.cycles += 5;
            let next = Addr(c.g[S0][0] as u32).off(4).lw(c);
            c.g[S0][0] = next;
            c.g[A0][0] = next;
            UI_LIST_NODE_IS_SENTINEL.call(c, ret_sentinel)?;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x54).lw(c);
                if event_due(c) {
                    deliver_event_here(c, head);
                }
                continue;
            }
            c.cycles += 1;
            break;
        }
    }
    frame.restore(c, S0, 0x10);
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void UIState_HideFlaggedScreens(cUIState* self)` (0x39e778): hide every
/// flagged screen of the state (its vtable slot 9 with show = 0).
pub fn ui_state_hide_flagged_screens(c: &mut Cpu) {
    guest(c, |c| {
        ui_state_flagged_screens(c, 0, [0x0039_e78c, 0x0039_e7c8, 0x0039_e7d4, 0x0039_e7a0])
    })
}

/// `void UIState_ShowFlaggedScreens(cUIState* self)` (0x39e7f0): show every
/// flagged screen of the state (its vtable slot 9 with show = 1).
pub fn ui_state_show_flagged_screens(c: &mut Cpu) {
    guest(c, |c| {
        ui_state_flagged_screens(c, 1, [0x0039_e804, 0x0039_e840, 0x0039_e84c, 0x0039_e818])
    })
}

/// `void UIObject_SetAnimProgress(UIObject* self, float f)` (0x39fd80): when
/// the object has an animation (`+0xc`), set the animation's short at `+0xa`
/// to its frame count times `f`, truncated.
pub fn ui_object_set_anim_progress(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_fpr(c, 20, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = c.g[S0][0] as u32;
        c.g[A0][0] = Addr(this).off(0xc).lw(c);
        c.mov_s(20, 12);
        if c.g[A0][0] != 0 {
            c.cycles += 9;
            OC1083_UIANIM_FRAME_COUNT.call(c, 0x0039_fda8)?;
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.g[V1][0] = Addr(this).off(0xc).lw(c);
            c.mul_s(0, 0, 20);
            c.cvt_w_s(1, 0);
            c.mfc1(2, 1);
            let v0 = c.g[V0][0];
            let anim = c.g[V1][0] as u32;
            Addr(anim).off(0xa).sh(c, v0);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `bool UIObject_IsAnimDone(UIObject* self)` (0x39fdd8): false without an
/// animation, otherwise the animation's done test.
pub fn ui_object_is_anim_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let obj = Addr((c.g[A0][0] as u32)).off(0xc).lw(c);
        c.g[A0][0] = obj;
        c.g[V0][0] = 0;
        if obj != 0 {
            c.cycles += 2;
            OC1083_UIANIM_IS_DONE.call(c, 0x0039_fdf4)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int UIObject_StartEventThread(UIObject* self, signed char idx)`
/// (0x39ffa0): when the object has an event table (`+0x3c`) whose entry
/// `idx` is not `0xffff` and it has a screen (`+0x5c`), start that screen's
/// frame event. Returns 1 when it started, 0 otherwise.
pub fn ui_object_start_event_thread(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
        let this = c.g[A2][0] as u32;
        c.g[V1][0] = Addr(this).off(0x3c).lw(c);
        c.g[V0][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
        let events = c.g[V1][0];
        if events == 0 {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 6;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
            c.g[A0][0] = 0xffff;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(events as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lhu(c);
            let no_entry = c.g[V1][0] == c.g[A0][0];
            c.g[V0][0] = 0;
            if !no_entry {
                c.cycles += 3;
                c.g[A0][0] = Addr(this).off(0x5c).lw(c);
                let no_screen = c.g[A0][0] == 0;
                c.g[A1][0] = c.g[V1][0];
                if !no_screen {
                    c.cycles += 4;
                    c.g[A2][0] = 1;
                    OC1083_UISCREEN_START_THREAD.call(c, 0x0039_ffe8)?;
                    c.g[V0][0] = 1;
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int UIText_GetCachedSize(UIText* self)` (0x3a10f0): the cached size at
/// `+0xa8`, after a refresh through the text's vtable slot 16 (at `+0x80`)
/// unless the cached flag (bit 2 of `+0x14`) is set and the text is laid out
/// (`+0xb4` non-zero).
pub fn ui_text_get_cached_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(this).off(0xb4).lw(c);
        if c.g[V0][0] == 0 {
            c.g[V0][0] = Addr(this).off(8).lw(c);
            ui_text_refresh(c, this)?;
        } else {
            c.cycles += 5;
            c.g[V0][0] = Addr(this).off(0x14).lw(c);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32) & 1;
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[V0][0] = Addr(this).off(8).lw(c);
                ui_text_refresh(c, this)?;
            }
        }
        c.g[V0][0] = Addr(this).off(0xa8).lw(c);
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The refresh call of [`ui_text_get_cached_size`]: vtable slot 16 at `v0`.
fn ui_text_refresh(c: &mut Cpu, this: u32) -> Flow {
    let _ = this;
    c.cycles += 5;
    let vt = c.g[V0][0];
    let slot = VSlot::read(c, vt, 0x80);
    c.g[A0][0] = slot.delta;
    c.g[V1][0] = slot.func;
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = slot.this(c.g[S0][0]);
    call_ptr(c, t, 0x003a_1134)?;
    Ok(())
}

/// `void cUITextScroll_Load(cUITextScroll* self, tUITextScroll* data)`
/// (0x3a11b8): load the base object, set the font and shadow bits of the
/// state word at `+0x74`, let the vtable hook (slot at `+0xb8`) see the
/// shadow flag, rebuild the text when the data has a string id, and copy the
/// shadow colour (scaled by 1/255) when bit 3 of the state word is set.
pub fn c_ui_text_scroll_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A0][0];
        let this = c.g[S1][0] as u32;
        let data = c.g[S0][0] as u32;
        OC1083_UIOBJECT_LOAD.call(c, 0x003a_11d4)?;

        c.g[V1][0] = Addr(data).off(0x20).lbu(c);
        c.g[A0][0] = 0xffff_ffff_ffff_fff9;
        c.g[V0][0] = Addr(this).off(0x74).lw(c);
        c.g[V1][0] = c.g[V1][0] & 3;
        c.g[A2][0] = Addr(this).off(8).lw(c);
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        let word = c.g[V0][0];
        Addr(this).off(0x74).sw(c, word);

        let vt = c.g[A2][0];
        let hook = VSlot::read(c, vt, 0xb8);
        c.g[A0][0] = hook.delta;
        c.g[A1][0] = Addr(data).off(0x21).lbu(c);
        c.g[V1][0] = hook.func;
        c.g[A0][0] = hook.this(c.g[S1][0]);
        c.g[A1][0] &= 1;
        call_ptr(c, hook.func as u32, 0x003a_1210)?;

        c.g[V0][0] = Addr(data).off(0x24).lw(c);
        if c.g[V0][0] == 0 {
            c.g[V0][0] = Addr(this).off(0x74).lw(c);
        } else {
            c.cycles += 6;
            c.g[A0][0] = c.g[S1][0];
            OC1083_UITS_CLEAR_LINES.call(c, 0x003a_1224)?;
            c.g[A1][0] = Addr(data).off(0x24).lw(c);
            c.g[A0][0] = c.g[S1][0];
            OC1083_UITS_SET_TEXT_ID.call(c, 0x003a_1230)?;
            c.g[V0][0] = Addr(this).off(0x74).lw(c);
        }

        // Shadow colour block.
        c.cycles += 4;
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32) & 1;
        if c.g[V0][0] != 0 {
            c.cycles += 30;
            c.g[V0][0] = Addr(data).off(0x2b).lbu(c);
            c.g[V1][0] = Addr(data).off(0x28).lbu(c);
            c.g[A0][0] = Addr(data).off(0x29).lbu(c);
            c.mtc1(2, 4);
            c.cvt_s_w(4, 4);
            c.g[A1][0] = Addr(data).off(0x2a).lbu(c);
            c.mtc1(3, 2);
            c.cvt_s_w(2, 2);
            c.mtc1(4, 3);
            c.cvt_s_w(3, 3);
            let gp = c.g[GP][0] as u32;
            c.lwc1(0, gp.wrapping_sub(0x24f4));
            c.mtc1(5, 1);
            c.cvt_s_w(1, 1);
            c.mul_s(4, 4, 0);
            c.mul_s(2, 2, 0);
            c.mul_s(3, 3, 0);
            c.mul_s(1, 1, 0);
            let s = sp(c);
            c.swc1(4, s.wrapping_add(0xc));
            c.swc1(2, s);
            c.swc1(3, s.wrapping_add(4));
            c.swc1(1, s.wrapping_add(8));
            c.ldl(2, s.wrapping_add(7));
            c.ldr(2, s);
            c.ldl(3, s.wrapping_add(0xf));
            c.ldr(3, s.wrapping_add(8));
            let s1 = c.g[S1][0] as u32;
            c.sdl(2, s1.wrapping_add(0x7f));
            c.sdr(2, s1.wrapping_add(0x78));
            c.sdl(3, s1.wrapping_add(0x87));
            c.sdr(3, s1.wrapping_add(0x80));
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    const SEEDS: u32 = 200;

    /// A vtable slot: `{ short delta; short; void (*fn)() }` at `off`.
    pub(crate) fn oc1083_put_slot(c: &mut Cpu, vt: u32, off: u32, delta: u32, func: u32) {
        c.bus.write16(vt + off, delta as u16);
        c.bus.write32(vt + off + 4, func);
    }

    /// A random small signed slot delta (a multiple of 4).
    pub(crate) fn oc1083_slot_delta(r: &mut Rng) -> u32 {
        (r.next_u32() as i16 as i32 & !3) as u32
    }

    /// Gate words of the callee fakes: each answers only while its word holds
    /// `OC1083_MAGIC`. Placed above the pointer area.
    const OC1083_LOAD_CTRL: u32 = DATA + 0x0021_0830;
    const OC1083_CLEAR_CTRL: u32 = DATA + 0x0021_0840;
    const OC1083_SETID_CTRL: u32 = DATA + 0x0021_0850;
    const OC1083_START_CTRL: u32 = DATA + 0x0021_0860;
    const OC1083_MAGIC: u32 = 0x1083_c0de;

    static OC1083_PREV_LOAD: AtomicUsize = AtomicUsize::new(0);
    static OC1083_PREV_CLEAR: AtomicUsize = AtomicUsize::new(0);
    static OC1083_PREV_SETID: AtomicUsize = AtomicUsize::new(0);
    static OC1083_PREV_START: AtomicUsize = AtomicUsize::new(0);

    fn oc1083_fake_object_load(c: &mut Cpu) {
        if c.bus.read32(OC1083_LOAD_CTRL) != OC1083_MAGIC {
            return fakes::fall_back(c, &OC1083_PREV_LOAD, ssx3_ee::s13::f_0039fe00__generated);
        }
        fakes::fake_call(c, 0x0039_fe00);
    }

    fn oc1083_fake_clear_lines(c: &mut Cpu) {
        if c.bus.read32(OC1083_CLEAR_CTRL) != OC1083_MAGIC {
            return fakes::fall_back(c, &OC1083_PREV_CLEAR, ssx3_ee::s13::f_003a18c0__generated);
        }
        fakes::fake_call(c, 0x003a_18c0);
    }

    fn oc1083_fake_set_text_id(c: &mut Cpu) {
        if c.bus.read32(OC1083_SETID_CTRL) != OC1083_MAGIC {
            return fakes::fall_back(c, &OC1083_PREV_SETID, ssx3_ee::s13::f_003a1f18__generated);
        }
        fakes::fake_call(c, 0x003a_1f18);
    }

    fn oc1083_fake_start_thread(c: &mut Cpu) {
        if c.bus.read32(OC1083_START_CTRL) != OC1083_MAGIC {
            return fakes::fall_back(c, &OC1083_PREV_START, ssx3_ee::s13::f_0039c870__generated);
        }
        fakes::fake_call(c, 0x0039_c870);
    }

    fn oc1083_install_fakes() {
        fakes::hook(
            addr_of_mut!(ssx3_ee::s13::OVERRIDE_0039FE00),
            oc1083_fake_object_load,
            &OC1083_PREV_LOAD,
        );
        fakes::hook(
            addr_of_mut!(ssx3_ee::s13::OVERRIDE_003A18C0),
            oc1083_fake_clear_lines,
            &OC1083_PREV_CLEAR,
        );
        fakes::hook(
            addr_of_mut!(ssx3_ee::s13::OVERRIDE_003A1F18),
            oc1083_fake_set_text_id,
            &OC1083_PREV_SETID,
        );
        fakes::hook(
            addr_of_mut!(ssx3_ee::s13::OVERRIDE_0039C870),
            oc1083_fake_start_thread,
            &OC1083_PREV_START,
        );
    }

    /// The slider's fixture: self at DATA+0x1000, ends `a` and `b` at
    /// DATA+0x2000 and DATA+0x3000 with vtables at +0x2200 and +0x3200.
    #[test]
    fn c_ui_slider_render_matches() {
        check(
            ssx3_ee::s13::f_0039e130__generated,
            c_ui_slider_render,
            SEEDS,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x39e1_3000, 4);
                let this = DATA + 0x1000;
                let a = DATA + 0x2000;
                let b = DATA + 0x3000;
                let va = DATA + 0x2200;
                let vb = DATA + 0x3200;
                c.bus
                    .write32(this + 0x88, if seed % 7 == 0 { 0 } else { a });
                c.bus
                    .write32(this + 0x8c, if seed % 7 == 1 { 0 } else { b });
                c.bus.write32(this + 0x44, r.edge_f32());
                c.bus.write32(this + 0x48, r.edge_f32());
                c.bus.write32(this + 0x74, r.next_u32() % 9);
                c.bus.write32(this + 0x78, 1 + r.next_u32() % 6);
                c.bus.write32(a + 8, va);
                c.bus.write32(b + 8, vb);
                for obj in [a, b] {
                    c.bus.write32(obj + 0x44, r.edge_f32());
                    c.bus.write32(obj + 0x48, r.edge_f32());
                    c.bus.write32(obj + 0x4c, r.edge_f32());
                }
                let d = oc1083_slot_delta(&mut r);
                oc1083_put_slot(c, va, 0x88, d, fakes::FAKE);
                let d = oc1083_slot_delta(&mut r);
                oc1083_put_slot(c, va, 0xa0, d, fakes::FAKE + 0x10);
                let d = oc1083_slot_delta(&mut r);
                oc1083_put_slot(c, vb, 0x88, d, fakes::FAKE + 0x20);
                let d = oc1083_slot_delta(&mut r);
                oc1083_put_slot(c, vb, 0xa0, d, fakes::FAKE + 0x30);
                c.g[A0][0] = this as u64;
                c.f[12] = r.edge_f32();
                c.f[13] = r.edge_f32();
            },
        );
    }

    /// The flagged-screen fixture: the state at DATA+0x1000, its list head at
    /// +0x24, `nodes` screens at DATA+0x2000 + 0x100*i, terminated by a
    /// sentinel node (its next points at itself).
    fn oc1083_flagged_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = fakes::machine(c, seed, salt, 3);
        let this = DATA + 0x1000;
        let nodes = seed % 5;
        let vt = DATA + 0x3800;
        let d = oc1083_slot_delta(&mut r);
        oc1083_put_slot(c, vt, 0x48, d, fakes::FAKE);
        let node_at = |i: u32| DATA + 0x2000 + 0x100 * i;
        let first = if nodes == 0 { node_at(0) } else { node_at(0) };
        c.bus.write32(this + 0x24 + 4, first);
        for i in 0..nodes {
            let n = node_at(i);
            c.bus.write32(n, 0);
            c.bus.write32(n + 4, node_at(i + 1));
            c.bus.write32(n + 0x48, vt);
            c.bus.write32(n + 0x54, r.next_u32());
        }
        let term = node_at(nodes);
        c.bus.write32(term, 0);
        c.bus.write32(term + 4, term);
        c.g[A0][0] = this as u64;
    }

    #[test]
    fn ui_state_hide_flagged_screens_matches() {
        check(
            ssx3_ee::s13::f_0039e778__generated,
            ui_state_hide_flagged_screens,
            SEEDS,
            &|c, seed| oc1083_flagged_setup(c, seed, 0x39e7_7800),
        );
    }

    #[test]
    fn ui_state_show_flagged_screens_matches() {
        check(
            ssx3_ee::s13::f_0039e7f0__generated,
            ui_state_show_flagged_screens,
            SEEDS,
            &|c, seed| oc1083_flagged_setup(c, seed, 0x39e7_f000),
        );
    }

    #[test]
    fn ui_object_set_anim_progress_matches() {
        check(
            ssx3_ee::s13::f_0039fd80__generated,
            ui_object_set_anim_progress,
            SEEDS,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x39fd_8000, 0);
                let this = DATA + 0x1000;
                let anim = DATA + 0x2000;
                let frames_obj = DATA + 0x2100;
                c.bus
                    .write32(this + 0xc, if seed % 4 == 0 { 0 } else { anim });
                c.bus.write32(anim, frames_obj);
                c.bus.write16(frames_obj + 2, r.next_u32() as u16);
                c.bus.write32(anim + 0xa, r.next_u32());
                c.g[S0][0] = 0;
                c.g[A0][0] = this as u64;
                c.f[12] = r.edge_f32();
            },
        );
    }

    #[test]
    fn ui_object_is_anim_done_matches() {
        check(
            ssx3_ee::s13::f_0039fdd8__generated,
            ui_object_is_anim_done,
            SEEDS,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x39fd_d800, 0);
                let this = DATA + 0x1000;
                let anim = DATA + 0x2000;
                let p = DATA + 0x2100;
                c.bus
                    .write32(this + 0xc, if seed % 3 == 0 { 0 } else { anim });
                c.bus.write32(anim, p);
                c.bus.write16(anim + 0xa, r.next_u32() as u16);
                c.bus.write16(p + 2, r.next_u32() as u16);
                c.g[A0][0] = this as u64;
            },
        );
    }

    #[test]
    fn ui_object_start_event_thread_matches() {
        oc1083_install_fakes();
        check(
            ssx3_ee::s13::f_0039ffa0__generated,
            ui_object_start_event_thread,
            SEEDS,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x39ff_a000, 2);
                oc1083_install_fakes();
                c.bus.write32(OC1083_START_CTRL, OC1083_MAGIC);
                let this = DATA + 0x1000;
                let events = DATA + 0x2000;
                let screen = DATA + 0x2400;
                c.bus
                    .write32(this + 0x3c, if seed % 4 == 0 { 0 } else { events });
                c.bus
                    .write32(this + 0x5c, if seed % 3 == 0 { 0 } else { screen });
                for i in 0..8u32 {
                    let v = if i % 3 == 0 { 0xffff } else { (i + 1) as u16 };
                    c.bus.write16(events + 2 * i, v);
                }
                let idx = (seed % 5) as u64;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (r.next_u32() as u64 & !0xff) | idx;
            },
        );
    }

    #[test]
    fn ui_text_get_cached_size_matches() {
        check(
            ssx3_ee::s13::f_003a10f0__generated,
            ui_text_get_cached_size,
            SEEDS,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3a10_f000, 2);
                let this = DATA + 0x1000;
                let vt = DATA + 0x3000;
                c.bus
                    .write32(this + 0xb4, if seed % 3 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(this + 0x14, r.next_u32());
                c.bus.write32(this + 8, vt);
                c.bus.write32(this + 0xa8, r.next_u32());
                let d = oc1083_slot_delta(&mut r);
                oc1083_put_slot(c, vt, 0x80, d, fakes::FAKE);
                c.g[A0][0] = this as u64;
            },
        );
    }

    #[test]
    fn c_ui_text_scroll_load_matches() {
        oc1083_install_fakes();
        check(
            ssx3_ee::s13::f_003a11b8__generated,
            c_ui_text_scroll_load,
            SEEDS,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3a11_b800, 4);
                oc1083_install_fakes();
                c.bus.write32(OC1083_LOAD_CTRL, OC1083_MAGIC);
                c.bus.write32(OC1083_CLEAR_CTRL, OC1083_MAGIC);
                c.bus.write32(OC1083_SETID_CTRL, OC1083_MAGIC);
                let this = DATA + 0x1000;
                let data = DATA + 0x2000;
                let vt = DATA + 0x3000;
                c.bus.write32(this + 8, vt);
                c.bus.write32(this + 0x74, r.next_u32());
                let d = oc1083_slot_delta(&mut r);
                oc1083_put_slot(c, vt, 0xb8, d, fakes::FAKE + 0x40);
                c.bus.write8(data + 0x20, r.next_u32() as u8);
                c.bus.write8(data + 0x21, r.next_u32() as u8);
                c.bus
                    .write32(data + 0x24, if seed % 2 == 0 { 0 } else { 7 });
                for off in 0x28..0x2c {
                    c.bus.write8(data + off, r.next_u32() as u8);
                }
                c.bus.write32(GAME_GP.wrapping_sub(0x24f4), 0x3b80_8081);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = data as u64;
            },
        );
    }
}
