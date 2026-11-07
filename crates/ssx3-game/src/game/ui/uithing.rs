//! `src/ui/uithing.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
pub(crate) use crate::game::camera::cameraalgorithms::lh;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::fepopup3::gp_6 as gp;
pub(crate) use crate::game::fe::ovstates::at as at_2;
pub(crate) use crate::game::fe::ovstates::close;
pub(crate) use crate::game::fe::ovstates::open;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0039_fe00, ui_object_load, "UIObject_Load"),
    (
        0x0039_ff78,
        uiobject_get_event_thread,
        "UIObject_GetEventThread",
    ),
    (0x003a_0000, ui_object_update_anim, "UIObject_UpdateAnim"),
    (0x003a_0318, uiobject_get_user_data, "UIObject_GetUserData"),
    (0x003a_0330, uiobject_set_user_data, "UIObject_SetUserData"),
    (0x003a_0348, ui_text_ctor, "UIText_ctor"),
    (0x003a_03f0, uitext_load, "UIText_Load"),
    (0x003a_04f0, uitext_get_font, "UIText_GetFont"),
    (0x003a_0528, ui_text_refresh_text, "UIText_RefreshText"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_3 as UI_OBJECT_CTOR;

    pub(crate) use crate::game::world::wscriptcache::callee::UI_ANIM_ADVANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_ANIM_APPLY;
}

// ---------------------------------------------------------------------------
// UI object user data / event thread

/// `int UIObject_GetEventThread(UIObject *obj, int index)` (0x39ff78): look up
/// the signed byte index in the 16-bit table at `obj + 0x3c`, or 0xffff.
pub fn uiobject_get_event_thread(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
    c.g[V0][0] = Addr(abi::arg(c, 0)).off(0x3c).lw(c);
    let present = c.g[V0][0] != 0;
    c.g[V1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
    if present {
        c.cycles += 4;
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lhu(c);
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0xffff;
    abi::ret(c);
}

/// `uint UIObject_GetUserData(UIObject *obj, int slot)` (0x3a0318): the word at
/// `obj + slot*4 + 0x6c`.
pub fn uiobject_get_user_data(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
    c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 22) as u32);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x6c).lw(c);
    abi::ret(c);
}

/// `void UIObject_SetUserData(UIObject *obj, int slot, uint value)`
/// (0x3a0330): store `value` at `obj + slot*4 + 0x6c`.
pub fn uiobject_set_user_data(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
    c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 22) as u32);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    Addr(c.g[A0][0] as u32).off(0x6c).sw(c, c.g[A2][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// UI text

/// `void *UIText_GetFont(UIText *text)` (0x3a04f0): the font record selected
/// by the low two bits at +0x74.
pub fn uitext_get_font(c: &mut Cpu) {
    c.cycles += 5;
    let text = abi::arg(c, 0);
    c.g[V0][0] = Addr(text).off(0x5c).lw(c);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xd0).lw(c);
    c.g[A1][0] = Addr(c.g[V1][0] as u32).off(0x10).lw(c);
    if c.g[A1][0] != 0 {
        c.g[V1][0] = Addr(text).off(0x74).lbu(c);
        c.cycles += 7;
        c.g[V0][0] = 0xc;
        c.g[A0][0] = Addr(c.g[A1][0] as u32).off(0x8).lw(c);
        c.g[V1][0] = c.g[V1][0] & 0x3;
        c.mult(0, 3, 3, 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xc).lw(c);
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// UI animation updates.

/// Shared body of `UIObject_UpdateAnim` / `UIVector_UpdateAnim`: apply the
/// event to the animation at `+0xc` (only when present), then advance it.
pub(crate) fn ui_anim_update(c: &mut Cpu, apply_ret: u32, advance_ret: u32) -> Flow {
    c.cycles += 7;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(0xc));
    let none = c.g[A0][0] == 0;
    c.g[A3][0] = c.g[A1][0] & 0xffff;
    if !none {
        c.cycles += 6;
        c.g[A2][0] = c.lbu(r32(c, S0).wrapping_add(0x10));
        c.g[A1][0] = c.g[S0][0];
        callee::UI_ANIM_APPLY.call(c, apply_ret)?;
        c.g[A1][0] = c.lbu(r32(c, S0).wrapping_add(0x10));
        c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(0xc));
        callee::UI_ANIM_ADVANCE.call(c, advance_ret)?;
    }
    c.cycles += 4;
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void UIObject_UpdateAnim(UIWidget* w, ushort event)` (0x3a0000): feed the
/// event to the widget's animation and advance it.
pub fn ui_object_update_anim(c: &mut Cpu) {
    guest(c, |c| ui_anim_update(c, 0x003a_0028, 0x003a_0034))
}

// ---------------------------------------------------------------------------
// UI text constructor.

/// `UIText* UIText_ctor(UIText* self)` (0x3a0348): run the UI object
/// constructor and install the two text vtables and the default flags.
pub fn ui_text_ctor(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        c.cycles += 42;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_OBJECT_CTOR.call(c, 0x003a_035c)?;
        c.g[V1][0] = Addr(at(c.g[S0][0], 0x74)).lbu(c);
        c.g[V0][0] = sx(0x0049_0000);
        c.g[V0][0] = sx(0x0049_4588);
        Addr(at(c.g[S0][0], 0xa8)).sw(c, 0);
        c.g[V1][0] |= 4;
        Addr(at(c.g[S0][0], 8)).sw(c, c.g[V0][0]);
        c.g[V1][0] &= 0x87;
        Addr(at(c.g[S0][0], 0xac)).sw(c, 0);
        Addr(at(c.g[S0][0], 0x74)).sb(c, c.g[V1][0]);
        c.g[A0][0] = sx(0x004c_0000);
        Addr(at(c.g[S0][0], 0xb0)).sw(c, 0);
        c.g[V0][0] = c.g[S0][0];
        Addr(at(c.g[S0][0], 0xb4)).sw(c, 0);
        Addr(at(c.g[S0][0], 0xb8)).sh(c, 0);
        Addr(at(c.g[S0][0], 0xba)).sh(c, 0);
        c.g[A3][0] = sx(0x004c_66c8);
        c.ldl(3, at(c.g[A3][0], 7));
        c.ldr(3, c.g[A3][0] as u32);
        c.ldl(5, at(c.g[A3][0], 0xf));
        c.ldr(5, at(c.g[A3][0], 8));
        c.sdl(3, at(c.g[S0][0], 0x8f));
        c.sdr(3, at(c.g[S0][0], 0x88));
        c.sdl(5, at(c.g[S0][0], 0x97));
        c.sdr(5, at(c.g[S0][0], 0x90));
        c.g[A3][0] = sx(0x004c_66c8);
        c.ldl(3, at(c.g[A3][0], 7));
        c.ldr(3, c.g[A3][0] as u32);
        c.ldl(5, at(c.g[A3][0], 0xf));
        c.ldr(5, at(c.g[A3][0], 8));
        c.sdl(3, at(c.g[S0][0], 0x9f));
        c.sdr(3, at(c.g[S0][0], 0x98));
        c.sdl(5, at(c.g[S0][0], 0xa7));
        c.sdr(5, at(c.g[S0][0], 0xa0));
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI objects.

/// `void* UIObject_Load(UIObject* self, const UIObjectDef* def)` (0x39fe00):
/// take the id (+0x38), the data pointer from `def+4`, merge the packed
/// bit 0..4 / 6 flag of `self+0x14` with the `def+8` bits (5 bits into bit 8,
/// bit 6 into bit 0, bit 7 into bit 7), then forward bits 9..10 of `def+8` to
/// the object's `onCreate` slot.
pub fn ui_object_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 46;
        open(c, 0x30);
        c.sq(16, at_2(c.g[SP][0] as u32, 0x20));
        c.sq(17, at_2(c.g[SP][0] as u32, 0x10));
        let this = c.g[A0][0];
        let def = c.g[A1][0];
        c.g[S0][0] = this;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S1][0] = def;
        c.g[A0][0] = 0xffff_ffff_ffff_e0ff;
        c.g[A1][0] = 0xffff_ffff_ffff_fffe;
        c.g[V0][0] = Addr(def as u32).off(4).lw(c);
        c.g[V1][0] = Addr(this as u32).off(0x14).lw(c);
        Addr(this as u32).off(0x38).sw(c, c.g[V0][0]);
        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
        c.g[A2][0] = Addr(this as u32).off(8).lw(c);
        c.g[V0][0] = Addr(def as u32).off(8).lbu(c);
        c.g[V0][0] = c.g[V0][0] & 0x1f;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
        Addr(this as u32).off(0x14).sw(c, c.g[V1][0]);
        c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
        c.g[V0][0] = Addr(def as u32).off(8).lw(c);
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
        c.g[V0][0] = c.g[V0][0] & 1;
        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
        Addr(this as u32).off(0x14).sw(c, c.g[V1][0]);
        c.g[A1][0] = Addr(def as u32).off(8).lw(c);
        c.g[A0][0] = Addr(c.g[A2][0] as u32).off(0x48).lh(c);
        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x4c).lw(c);
        c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 9) as u32);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((this as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[A1][0] = c.g[A1][0] & 1;
        call_ptr(c, target, 0x0039_fe7c)?;
        c.g[V1][0] = Addr(def as u32).off(8).lbu(c);
        c.g[A0][0] = 0xffff_ffff_ffff_ff7f;
        c.g[V0][0] = Addr(this as u32).off(0x14).lw(c);
        c.g[S1][0] = sx((def as u32).wrapping_add(0xc));
        c.g[V1][0] = c.g[V1][0] >> 7;
        Addr(this as u32).off(0x3c).sw(c, c.g[S1][0]);
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 7);
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.lq(17, at_2(c.g[SP][0] as u32, 0x10));
        Addr(this as u32).off(0x14).sw(c, c.g[V0][0]);
        c.lq(16, at_2(c.g[SP][0] as u32, 0x20));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x30);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `UIText_RefreshText` (0x003a0528).
//
// `void UIText::RefreshText(UIText* self)`: resolve the current string and
// either re-wrap it or pass it to the wide-string setter.

pub fn ui_text_refresh_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0] as u32;
        c.g[S0][0] = sx(this);
        c.g[A1][0] = lw(c, this, 0xb0);
        let string = c.g[A1][0];
        // The two branches that return early (no text or no owner) share this
        // body; the rest of the function only runs for a live string.
        if string != 0 {
            c.cycles += 6;
            c.g[V0][0] = lw(c, this, 0x5c);
            c.g[V1][0] = lw(c, c.g[V0][0] as u32, 0xd0);
            c.g[A0][0] = lw(c, c.g[V1][0] as u32, 0x10);
            c.g[A2][0] = lw(c, c.g[A0][0] as u32, 0x10);
            if c.g[A2][0] != 0 {
                c.cycles += 8;
                c.g[V1][0] = lw(c, c.g[A2][0] as u32, 4);
                c.g[A0][0] = lh(c, c.g[V1][0] as u32, 0x20);
                c.g[V0][0] = lw(c, c.g[V1][0] as u32, 0x24);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x003a_0570)?;
                c.g[A1][0] = c.g[V0][0];
                if c.g[A1][0] == 0 {
                    c.cycles += 3;
                    c.g[V0][0] = lw(c, this, 8);
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = lw(c, this, 0x14);
                    c.g[V0][0] = sx((((c.g[V0][0] as u32) as i32) >> 7) as u32);
                    c.g[V0][0] = c.g[V0][0] & 1;
                    let wrapped = c.g[V0][0] != 0;
                    let wide = if wrapped {
                        true
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = lw(c, this, 0x74);
                        c.g[V0][0] = sx((((c.g[V0][0] as u32) as i32) >> 6) as u32);
                        c.g[V0][0] = c.g[V0][0] & 1;
                        c.g[V0][0] != 0
                    };
                    if wide {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x003a_0d00, 0x003a_05ac)?;
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x003a_0eb0, 0x003a_05bc)?;
                    }
                    c.g[V0][0] = lw(c, this, 8);
                }
                // Common tail: notify the owner through its virtual slot.
                c.cycles += 6;
                c.g[A1][0] = 0x1;
                c.g[A0][0] = lh(c, c.g[V0][0] as u32, 0x28);
                c.g[V1][0] = lw(c, c.g[V0][0] as u32, 0x2c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x003a_05d4)?;
            }
        }
        frame.restore(c, 16, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void func_003A03F0(UIText* self, sUIData* data)` (0x3a03f0): the text
/// element's load: the shared object load, alignment and visibility, the text
/// id from `data+0x24`, and with the wrap flag the four texel-size floats from
/// the byte channels at `data+0x28`.
pub fn uitext_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x40);
        let data = c.g[A1][0];
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        c.g[S0][0] = data;
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x003a_040c);
        c.g[S1][0] = c.g[A0][0];
        c.pc = 0x0039_fe00;
        ee::s13::UIObject_Load(c);
        if c.pc != 0x003a_040c {
            return Err(Unwound);
        }
        c.g[V1][0] = c.lbu(p(c.g[S0][0] as u32, 0x20));
        c.g[A0][0] = 0xffff_ffff_ffff_fffc;
        c.g[V0][0] = c.lw(p(c.g[S1][0] as u32, 0x74));
        c.g[V1][0] = c.g[V1][0] & 0x3;
        c.g[A2][0] = c.lw(p(c.g[S1][0] as u32, 8));
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.sw(p(c.g[S1][0] as u32, 0x74), c.g[V0][0]);
        c.g[A0][0] = c.lh(p(c.g[A2][0] as u32, 0xb8));
        c.g[A1][0] = c.lbu(p(c.g[S0][0] as u32, 0x22));
        c.g[V0][0] = c.lw(p(c.g[A2][0] as u32, 0xbc));
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let func = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x003a_0444);
        c.g[A1][0] = c.g[A1][0] & 0x1;
        call_ptr(c, func, 0x003a_0444)?;
        c.g[A1][0] = c.lw(p(c.g[S0][0] as u32, 0x24));
        c.g[RA][0] = sx(0x003a_0450);
        c.g[A0][0] = c.g[S1][0];
        c.pc = 0x003a_1000;
        ee::s13::UIText_SetTextId(c);
        if c.pc != 0x003a_0450 {
            return Err(Unwound);
        }
        c.g[V0][0] = c.lw(p(c.g[S1][0] as u32, 0x74));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
        c.g[V0][0] = c.g[V0][0] & 0x1;
        if c.g[V0][0] == 0 {
            c.lq(16, p(c.g[SP][0] as u32, 0x30));
        } else {
            c.cycles += 30;
            c.g[V0][0] = c.lbu(p(c.g[S0][0] as u32, 0x2b));
            c.g[V1][0] = c.lbu(p(c.g[S0][0] as u32, 0x28));
            c.g[A0][0] = c.lbu(p(c.g[S0][0] as u32, 0x29));
            c.mtc1(2, 4);
            c.cvt_s_w(4, 4);
            c.g[A1][0] = c.lbu(p(c.g[S0][0] as u32, 0x2a));
            c.mtc1(3, 2);
            c.cvt_s_w(2, 2);
            c.mtc1(4, 3);
            c.cvt_s_w(3, 3);
            c.lwc1(0, gp(c, -0x24f8)); // texel scale
            c.mtc1(5, 1);
            c.cvt_s_w(1, 1);
            c.mul_s(4, 4, 0);
            c.mul_s(2, 2, 0);
            c.mul_s(3, 3, 0);
            c.mul_s(1, 1, 0);
            c.swc1(4, p(c.g[SP][0] as u32, 0xc));
            c.swc1(2, c.g[SP][0] as u32);
            c.swc1(3, p(c.g[SP][0] as u32, 4));
            c.swc1(1, p(c.g[SP][0] as u32, 8));
            c.ldl(2, p(c.g[SP][0] as u32, 7));
            c.ldr(2, c.g[SP][0] as u32);
            c.ldl(3, p(c.g[SP][0] as u32, 0xf));
            c.ldr(3, p(c.g[SP][0] as u32, 8));
            c.sdl(2, p(c.g[S1][0] as u32, 0x7f));
            c.sdr(2, p(c.g[S1][0] as u32, 0x78));
            c.sdl(3, p(c.g[S1][0] as u32, 0x87));
            c.sdr(3, p(c.g[S1][0] as u32, 0x80));
            c.lq(16, p(c.g[SP][0] as u32, 0x30));
        }
        c.cycles += 4;
        c.lq(17, p(c.g[SP][0] as u32, 0x20));
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0x10));
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::FAKE;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    pub(crate) use crate::game::be::beintplayer::tests::strict;
    pub(crate) use crate::game::be::beintplayer::tests::SEEDS_7 as SEEDS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base;

    pub(crate) use crate::game::fe::feasyncfile::tests::wb;

    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS;

    pub(crate) use crate::game::ui::uiengine::tests::ui_fixture;
    pub(crate) use crate::game::ui::uitext::tests::get_font_setup;
    use ssx3_ee as ee;

    #[test]
    fn uiobject_get_event_thread_matches() {
        check(
            ssx3_ee::s13::f_0039ff78,
            uiobject_get_event_thread,
            200,
            &|c, seed| {
                let mut r = Rng(0x39ff_7800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | (seed % 8) as u64;
                let table = DATA + 0x400;
                if seed % 3 == 0 {
                    c.bus.write32(DATA + 0x3c, 0);
                } else {
                    c.bus.write32(DATA + 0x3c, table);
                    for k in 0..20 {
                        c.bus.write16(table + k * 2, r.next_u32() as u16);
                    }
                }
            },
        );
    }

    #[test]
    fn uiobject_get_user_data_matches() {
        check(
            ssx3_ee::s13::f_003a0318,
            uiobject_get_user_data,
            200,
            &|c, seed| {
                let mut r = Rng(0x3a03_1800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (DATA as u64) | ((r.next_u32() as u64) << 32);
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | (seed % 8) as u64;
                for k in 0..8 {
                    c.bus.write32(DATA + k * 4 + 0x6c, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn uiobject_set_user_data_matches() {
        check(
            ssx3_ee::s13::f_003a0330,
            uiobject_set_user_data,
            200,
            &|c, seed| {
                let mut r = Rng(0x3a03_3000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | (seed % 8) as u64;
                c.g[A2][0] = (r.next_u32() as u64) | (r.next_u32() as u64) << 32;
                for k in 0..8 {
                    c.bus.write32(DATA + k * 4 + 0x6c, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn uitext_get_font_matches() {
        check_lohi(
            ssx3_ee::s13::f_003a04f0,
            uitext_get_font,
            200,
            &|c, seed| get_font_setup(c, seed, false),
        );
    }

    /// A widget at `DATA+0x1000` with (sometimes) an all-zero animation object
    /// at `+0xc`; the animation's `+0` stays null so the evaluator is skipped.
    pub(crate) fn ui_anim_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        let obj = DATA + 0x1000;
        let anim = DATA + 0x2000;
        c.bus.write8(obj + 0x10, (r.next_u32() % 0x40) as u8);
        if seed % 2 == 0 {
            c.bus.write32(obj + 0xc, anim);
            c.bus.write16(anim + 0xa, (r.next_u32() % 0x40) as u16);
            c.bus.write32(anim + 8, 0);
        }
        c.g[A0][0] = sx(obj);
        c.g[A1][0] = (r.next_u32() & 0xffff) as u64;
    }

    #[test]
    fn ui_object_update_anim_matches() {
        check(
            ee::s13::f_003a0000__generated,
            ui_object_update_anim,
            40,
            &|c, seed| {
                ui_anim_setup(c, seed, 0x3a_0000);
            },
        );
    }

    // ---------------------------------------------------------- UIText ctor

    #[test]
    fn ui_text_ctor_matches() {
        check(
            ee::s13::f_003a0348__generated,
            ui_text_ctor,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x3a03_48, 4);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                wb(c, obj + 0x74, (seed % 0x100) as u8);
            },
        );
    }

    // ------------------------------------------------------------------ UI.

    #[test]
    fn ui_object_load_matches() {
        check(
            ee::s13::f_0039fe00__generated,
            ui_object_load,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39_fe00, 1);
                let this = DATA;
                let def = DATA + 0x2000;
                let vt = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(def);
                c.bus.write32(this + 8, vt);
                c.bus.write16(vt + 0x48, 0);
                c.bus.write32(vt + 0x4c, FAKE);
                c.bus.write32(def + 4, r.next_u32());
                c.bus.write32(def + 8, r.next_u32());
                c.bus.write32(this + 0x14, r.next_u32());
            },
        );
    }

    #[test]
    fn ui_text_refresh_text_matches() {
        check(
            ee::s13::f_003a0528__generated,
            ui_text_refresh_text,
            100,
            &|c, seed| {
                let mut r = Rng(0x003a_0528 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let this = DATA;
                c.bus
                    .write32(this + 0xb0, if seed % 3 == 0 { 0 } else { 1 });
                let a = DATA + 0x100;
                let b = DATA + 0x200;
                let d = DATA + 0x300;
                c.bus.write32(this + 0x5c, a);
                c.bus.write32(a + 0xd0, b);
                c.bus.write32(b + 0x10, d);
                c.bus.write32(d + 0x10, 0); // hit the "no owner" early return
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn uitext_load_matches() {
        strict(
            ee::s13::f_003a03f0__generated,
            uitext_load,
            SEEDS_2,
            &|c, seed| {
                let (this, data) =
                    ui_fixture(c, seed, 0x03f0, &[0x28, 0x2c, 0x48, 0x4c, 0xb8, 0xbc]);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(data);
            },
        );
    }
}
