//! Staging module oc711 (wave wave240): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc711`.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
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
        0x001e_91d0,
        hud_get_elem_color_rotated,
        "HUD_GetElemColorRotated",
    ),
    (0x002a_7ea8, ini_parser_read_int, "IniParser_ReadInt"),
    (
        0x0029_48a0,
        ssx_audio_clear_rider_air_flags,
        "SSXAudioSystem_ClearRiderAirFlags",
    ),
    (
        0x0039_ff50,
        ui_object_get_state_data,
        "UIObject_GetStateData",
    ),
    (0x0039_a928, ui_menu_set_focus, "UIMenu_SetFocus"),
    (
        0x0020_e900,
        ov_state_overlay_update,
        "cOVStateOverlay_Update",
    ),
];

/// `IniParser::SkipWhitespace` (0x2a78f8).
const OC711_INI_SKIP_WS: Callee = Callee {
    addr: 0x002a_78f8,
    f: ee::s08::f_002a78f8,
};
/// `IniParser::ReadToken` (0x2a7da0).
const OC711_INI_READ_TOKEN: Callee = Callee {
    addr: 0x002a_7da0,
    f: ee::s08::f_002a7da0,
};
/// `UIMenu::setSelectedByIndex` (0x39b960).
const OC711_UI_MENU_SET_SELECTION: Callee = Callee {
    addr: 0x0039_b960,
    f: ee::s13::f_0039b960,
};
/// `UIMenu::findFirstSelectable` (0x39b7b0).
const OC711_UI_MENU_FIND_FIRST: Callee = Callee {
    addr: 0x0039_b7b0,
    f: ee::s13::f_0039b7b0,
};
/// `UIObject::getEventThread` (0x39ff78).
const OC711_UI_OBJECT_EVENT_THREAD: Callee = Callee {
    addr: 0x0039_ff78,
    f: ee::s13::f_0039ff78,
};
/// `UIScreen::startThread` (0x39c870).
const OC711_UI_SCREEN_START_THREAD: Callee = Callee {
    addr: 0x0039_c870,
    f: ee::s13::f_0039c870,
};

/// `lw off(base)`, sign-extended.
#[inline]
fn oc711_lw(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr((base as u32).wrapping_add(off)).lw(c)
}

/// `lh off(base)`, sign-extended.
#[inline]
fn oc711_lh(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr((base as u32).wrapping_add(off)).lh(c)
}

/// `lbu off(base)`.
#[inline]
fn oc711_lbu(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr((base as u32).wrapping_add(off)).lbu(c)
}

/// `void HUD_GetElemColorRotated(const float* src, float* dst)`
/// (0x1e91d0, `tHUDDef_getColour1`): copies four floats of a colour record
/// with a rotation: `dst+0xc = src[4]`, `dst[0] = src[5]`, `dst[1] = src[6]`,
/// `dst[2] = src[7]`. f0 and f1 are left holding the last two values loaded.
pub fn hud_get_elem_color_rotated(c: &mut Cpu) {
    c.cycles += 9;
    let src = Addr(abi::arg(c, 0));
    let dst = Addr(abi::arg(c, 1));
    let f = src.off(0x10).read_fpu(c);
    abi::set_fpr(c, 0, f);
    dst.off(0xc).write_fpu(c, f);
    let f = src.off(0x14).read_fpu(c);
    abi::set_fpr(c, 1, f);
    dst.write_fpu(c, f);
    let f = src.off(0x18).read_fpu(c);
    abi::set_fpr(c, 0, f);
    dst.off(0x4).write_fpu(c, f);
    let f = src.off(0x1c).read_fpu(c);
    abi::set_fpr(c, 1, f);
    dst.off(0x8).write_fpu(c, f);
    abi::ret(c);
}

/// `int IniParser_ReadInt(IniParser* self)` (0x2a7ea8, `FileParser_GetInt`):
/// skips blanks, reads the next token into the buffer at `self+0xc`, and
/// returns the integer `sscanf(buffer, fmt, &value)` stored (fmt at 0x4a3738).
pub fn ini_parser_read_int(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        OC711_INI_SKIP_WS.call(c, 0x002a_7ebc)?;
        c.g[A0][0] = c.g[S0][0];
        OC711_INI_READ_TOKEN.call(c, 0x002a_7ec4)?;
        c.g[A1][0] = 0x004a_0000;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3738));
        c.g[A2][0] = c.g[SP][0];
        call_ptr(c, 0x0041_78b0, 0x002a_7ed8)?;
        c.g[V0][0] = Addr(sp(c)).lw(c);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `void SSXAudio::ClearRiderAirFlags(self)` (0x2948a0, `cSSXAudio_ResetLift`):
/// clears the six `mNeedLift[i]` words (i = 5 down to 0) at `self+0x59e8`.
/// The loop's back edge is an event check (head 0x2948a8).
pub fn ssx_audio_clear_rider_air_flags(c: &mut Cpu) {
    c.cycles += 2;
    c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(0x59fc));
    c.g[V0][0] = 0x5;
    loop {
        c.cycles += 7;
        Addr(c.g[A0][0] as u32).sw(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        let more = (c.g[V0][0] as i64) >= 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0029_48a8);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `undefined UIObject_GetStateData(obj)` (0x39ff50, `cUIThing_getEngine`):
/// the state word at `*(*(obj+0x5c)+0xd0)+0x10`.
pub fn ui_object_get_state_data(c: &mut Cpu) {
    c.cycles += 4;
    let obj = c.g[A0][0];
    c.g[V0][0] = oc711_lw(c, obj, 0x5c);
    let v0 = c.g[V0][0];
    c.g[V1][0] = oc711_lw(c, v0, 0xd0);
    let v1 = c.g[V1][0];
    c.g[V0][0] = oc711_lw(c, v1, 0x10);
    abi::ret(c);
}

/// `void cUIMenu::setFocus(bool on)` (0x39a928, `UIMenu_SetFocus`): stores
/// `on` as bit 4 of the flags at `+0x14`, then, when focused, re-selects the
/// stored index (or the first highlightable item) and starts the selected
/// item's screen and event thread; when unfocused it does the same for the
/// current selection's event. The blocks below follow the generated body's
/// exits: every path ends in the common `lq s0` and epilogue.
pub fn ui_menu_set_focus(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = (0u64 < c.g[A1][0]) as u64;
        frame.save(c, S0, 0x10);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = 0xffff_ffff_ffff_ffef;
        let me = c.g[S0][0];
        c.g[V0][0] = oc711_lw(c, me, 0x14);
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        let on = c.g[A1][0] != 0;
        let flags = c.g[V0][0];
        Addr((me as u32).wrapping_add(0x14)).sw(c, flags);
        if on {
            ui_menu_focus_on(c)?;
        } else {
            ui_menu_focus_off(c)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The focused branch of `UIMenu_SetFocus` (generated blocks `'b_39a978`,
/// `'b_39a9ac`, `'b_39a9f0` and the tail at 0x39aa08).
fn ui_menu_focus_on(c: &mut Cpu) -> Flow {
    let s0 = c.g[S0][0];
    c.cycles += 4;
    c.g[V1][0] = oc711_lbu(c, s0, 0xa4);
    c.g[V0][0] = 0xff;
    let stored_none = c.g[V1][0] == c.g[V0][0];
    c.g[A0][0] = s0;
    if !stored_none {
        // The stored index is valid: select it and leave.
        c.cycles += 4;
        let a0 = c.g[A0][0];
        c.g[A1][0] = oc711_lbu(c, a0, 0xa4);
        OC711_UI_MENU_SET_SELECTION.call(c, 0x0039_a970)?;
        return Ok(());
    }
    // No stored index: the current item, else the first highlightable one.
    c.cycles += 3;
    c.g[A0][0] = oc711_lw(c, s0, 0xa0);
    if c.g[A0][0] != 0 {
        let a0 = c.g[A0][0];
        c.g[V0][0] = oc711_lw(c, a0, 0x14);
    } else {
        c.cycles += 10;
        c.g[A1][0] = 0;
        c.g[A0][0] = s0;
        OC711_UI_MENU_FIND_FIRST.call(c, 0x0039_a990)?;
        c.g[V1][0] = c.g[V0][0] ^ 0xff;
        c.g[A1][0] = 0;
        c.g[A0][0] = s0;
        if c.g[V1][0] != 0 {
            c.g[A1][0] = c.g[V0][0];
        }
        OC711_UI_MENU_SET_SELECTION.call(c, 0x0039_a9a4)?;
        return Ok(());
    }
    c.cycles += 4;
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
    c.g[V0][0] = c.g[V0][0] & 0x1;
    if c.g[V0][0] != 0 {
        return Ok(());
    }
    // The selected item has no keyframer event yet: start its screen.
    c.cycles += 6;
    c.g[A1][0] = 0;
    OC711_UI_OBJECT_EVENT_THREAD.call(c, 0x0039_a9c4)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[V0][0] = 0xffff;
    if c.g[A1][0] == c.g[V0][0] {
        c.g[A2][0] = oc711_lw(c, s0, 0xa0);
    } else {
        c.cycles += 4;
        c.g[V0][0] = oc711_lw(c, s0, 0xa0);
        let v0 = c.g[V0][0];
        c.g[A0][0] = oc711_lw(c, v0, 0x5c);
        if c.g[A0][0] == 0 {
            c.g[A2][0] = oc711_lw(c, s0, 0xa0);
        } else {
            c.cycles += 3;
            c.g[A2][0] = 1;
            OC711_UI_SCREEN_START_THREAD.call(c, 0x0039_a9ec)?;
            c.g[A2][0] = oc711_lw(c, s0, 0xa0);
        }
    }
    // Tail: the item's `vtable[0x3c]` method and the screen's `vtable[0x9c]`.
    c.cycles += 17;
    c.g[A1][0] = 1;
    let a2 = c.g[A2][0];
    c.g[V0][0] = oc711_lw(c, a2, 8);
    let v0 = c.g[V0][0];
    c.g[A0][0] = oc711_lh(c, v0, 0x38);
    c.g[V1][0] = oc711_lw(c, v0, 0x3c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((a2 as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x0039_aa08)?;
    c.g[A0][0] = oc711_lw(c, s0, 0x5c);
    c.g[A2][0] = 1;
    c.g[A1][0] = oc711_lw(c, s0, 0xa0);
    let a0 = c.g[A0][0];
    c.g[A3][0] = oc711_lw(c, a0, 0xd0);
    let a3 = c.g[A3][0];
    c.g[V0][0] = oc711_lw(c, a3, 8);
    let v0 = c.g[V0][0];
    c.g[A0][0] = oc711_lh(c, v0, 0x98);
    c.g[V1][0] = oc711_lw(c, v0, 0x9c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((a3 as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x0039_aa2c)?;
    Ok(())
}

/// The unfocused branch of `UIMenu_SetFocus` (generated block at 0x39aa48
/// onwards): when a selection exists, its event thread is started and both
/// vtable methods are called with the flag value 2.
fn ui_menu_focus_off(c: &mut Cpu) -> Flow {
    let s0 = c.g[S0][0];
    c.cycles += 3;
    c.g[A0][0] = oc711_lw(c, s0, 0xa0);
    if c.g[A0][0] == 0 {
        return Ok(());
    }
    c.cycles += 6;
    c.g[A1][0] = 1;
    OC711_UI_OBJECT_EVENT_THREAD.call(c, 0x0039_aa48)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[V0][0] = 0xffff;
    if c.g[A1][0] == c.g[V0][0] {
        c.g[A2][0] = oc711_lw(c, s0, 0xa0);
    } else {
        c.cycles += 4;
        c.g[V0][0] = oc711_lw(c, s0, 0xa0);
        let v0 = c.g[V0][0];
        c.g[A0][0] = oc711_lw(c, v0, 0x5c);
        if c.g[A0][0] == 0 {
            c.g[A2][0] = oc711_lw(c, s0, 0xa0);
        } else {
            c.cycles += 3;
            c.g[A2][0] = 1;
            OC711_UI_SCREEN_START_THREAD.call(c, 0x0039_aa70)?;
            c.g[A2][0] = oc711_lw(c, s0, 0xa0);
        }
    }
    c.cycles += 16;
    c.g[A1][0] = 0;
    let a2 = c.g[A2][0];
    c.g[V0][0] = oc711_lw(c, a2, 8);
    let v0 = c.g[V0][0];
    c.g[A0][0] = oc711_lh(c, v0, 0x38);
    c.g[V1][0] = oc711_lw(c, v0, 0x3c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((a2 as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x0039_aa8c)?;
    c.g[A0][0] = oc711_lw(c, s0, 0x5c);
    c.g[A2][0] = 2;
    c.g[A1][0] = oc711_lw(c, s0, 0xa0);
    let a0 = c.g[A0][0];
    c.g[A3][0] = oc711_lw(c, a0, 0xd0);
    let a3 = c.g[A3][0];
    c.g[V0][0] = oc711_lw(c, a3, 8);
    let v0 = c.g[V0][0];
    c.g[A0][0] = oc711_lh(c, v0, 0x98);
    c.g[V1][0] = oc711_lw(c, v0, 0x9c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((a3 as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x0039_aab0)?;
    Ok(())
}

/// `void cOVState::onUpdate(self)` (0x20e900, `cOVStateOverlay_Update`):
/// pauses the game (`cGame_activatePauseMenu(SSXApp->Game, 1)`), then runs
/// the state's `cUIState::onUpdate(self)`.
pub fn ov_state_overlay_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        let gp = c.g[GP][0] as u32;
        c.g[V0][0] = Addr(gp.wrapping_sub(0x848)).lw(c);
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let v0 = c.g[V0][0];
        c.g[A0][0] = oc711_lw(c, v0, 0x84);
        call_ptr(c, 0x0023_1840, 0x0020_e920)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_e510, 0x0020_e928)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::events::{check_events, schedule};
    use crate::difftest::fakes;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    /// Writes a pointer-sized word at `at`.
    fn put(c: &mut Cpu, at: u32, v: u32) {
        c.bus.write32(at, v);
    }

    #[test]
    fn hud_get_elem_color_rotated_matches() {
        check(
            ee::s04::f_001e91d0__generated,
            hud_get_elem_color_rotated,
            200,
            &|c, seed| {
                let mut r = Rng(0x001e_91d0 ^ seed as u64);
                scramble(c, &mut r);
                let src = DATA + 0x100;
                let dst = DATA + 0x200 + (seed % 4) * 0x10;
                for k in 0..8 {
                    put(c, src + 4 * k, r.next_u32());
                }
                c.g[A0][0] = sx(src);
                c.g[A1][0] = sx(dst);
            },
        );
    }

    #[test]
    fn ini_parser_read_int_matches() {
        check(
            ee::s08::f_002a7ea8__generated,
            ini_parser_read_int,
            200,
            &|c, seed| {
                let mut r = Rng(0x002a_7ea8 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA + 0x100;
                let text = DATA + 0x400;
                put(c, this + 8, text);
                let mut s = Vec::new();
                for _ in 0..(r.next_u32() % 4) {
                    s.push(b' ');
                }
                for _ in 0..(1 + r.next_u32() % 6) {
                    s.push(b'0' + (r.next_u32() % 10) as u8);
                }
                s.push(b' ');
                for (i, &b) in s.iter().enumerate() {
                    c.bus.write8(text + i as u32, b);
                }
                c.bus.write8(text + s.len() as u32, 0);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn ssx_audio_clear_rider_air_flags_matches() {
        check_events(
            ee::s07::f_002948a0__generated,
            ssx_audio_clear_rider_air_flags,
            120,
            &|c, seed| {
                let mut r = Rng(0x0029_48a0 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA + 0x100;
                for k in 0..8 {
                    put(c, this + 0x59e8 + 4 * k, r.next_u32());
                }
                c.g[A0][0] = sx(this);
                schedule(c, &mut r, seed, 0x0029_48a0)
            },
        );
    }

    #[test]
    fn ui_object_get_state_data_matches() {
        check(
            ee::s13::f_0039ff50__generated,
            ui_object_get_state_data,
            200,
            &|c, seed| {
                let mut r = Rng(0x0039_ff50 ^ seed as u64);
                scramble(c, &mut r);
                let obj = DATA + 0x100;
                let state = DATA + 0x200;
                let data = DATA + 0x300;
                put(c, obj + 0x5c, state);
                put(c, state + 0xd0, data);
                put(c, data + 0x10, r.next_u32());
                c.g[A0][0] = sx(obj);
            },
        );
    }

    /// The menu's focus change: the flag word, the stored index (0xff or a
    /// real one), a selection or none, the screen, and vtable methods that
    /// land on the fake callee (`FAKE`, which logs and returns).
    #[test]
    fn ui_menu_set_focus_matches() {
        check(
            ee::s13::f_0039a928__generated,
            ui_menu_set_focus,
            200,
            &|c, seed| {
                let mut r = Rng(0x0039_a928 ^ seed as u64);
                scramble(c, &mut r);
                let me = DATA + 0x100;
                let sel = DATA + 0x300;
                let screen = DATA + 0x600;
                let vt = DATA + 0x500;
                let obj = DATA + 0x700;
                let vt2 = DATA + 0xa00;
                let a3 = DATA + 0x900;
                put(c, me + 0x14, r.next_u32());
                c.bus.write8(
                    me + 0xa4,
                    if seed % 3 == 0 {
                        0xff
                    } else {
                        (r.next_u32() % 8) as u8
                    },
                );
                put(c, me + 0xa0, if seed % 4 == 0 { 0 } else { sel });
                put(c, me + 0x5c, obj);
                put(c, sel + 0x5c, if seed % 5 == 0 { 0 } else { screen });
                put(c, sel + 0x14, r.next_u32());
                put(c, sel + 8, vt);
                put(c, vt + 0x3c, FAKE);
                put(c, obj + 0xd0, a3);
                put(c, a3 + 8, vt2);
                put(c, vt2 + 0x9c, FAKE);
                c.g[A0][0] = sx(me);
                c.g[A1][0] = if seed % 2 == 0 { 1 } else { 0 };
            },
        );
    }

    /// The pause overlay: the game pointer through `gp-0x848`, then the
    /// two calls (both return to their `ra`).
    /// RAM word that arms the two fakes below (set only by this test's
    /// fixture, so other tests' machines still run the real callees).
    const OC711_FX_CTRL: u32 = DATA + 0x20_0000;
    const OC711_FX_MAGIC: u32 = 0x4f43_3711;

    /// Fake for `cGame_activatePauseMenu` (0x231840): logged when armed,
    /// otherwise the generated body.
    fn oc711_fake_pause(c: &mut Cpu) {
        if c.bus.read32(OC711_FX_CTRL) != OC711_FX_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc711_fake_pause),
                ee::s06::f_00231840__generated,
            );
        }
        fakes::fake_call(c, 0x0023_1840);
    }

    /// Fake for `cUIState_onUpdate` (0x39e510), same gate.
    fn oc711_fake_state_update(c: &mut Cpu) {
        if c.bus.read32(OC711_FX_CTRL) != OC711_FX_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc711_fake_state_update),
                ee::s13::f_0039e510__generated,
            );
        }
        fakes::fake_call(c, 0x0039_e510);
    }

    #[test]
    fn ov_state_overlay_update_matches() {
        // Both callees reach the real game code, which runs away on this
        // fixture; the armed fakes log the calls instead (the diff still
        // compares the arguments and the register dance).
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s06::OVERRIDE_00231840),
            oc711_fake_pause,
            fakes::prev_cell(oc711_fake_pause),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_0039E510),
            oc711_fake_state_update,
            fakes::prev_cell(oc711_fake_state_update),
        );
        check(
            ee::s05::f_0020e900__generated,
            ov_state_overlay_update,
            200,
            &|c, seed| {
                let mut r = Rng(0x0020_e900 ^ seed as u64);
                scramble(c, &mut r);
                c.bus.write32(OC711_FX_CTRL, OC711_FX_MAGIC);
                c.g[GP][0] = sx(GAME_GP);
                let world = DATA + 0x500;
                let game = DATA + 0x600;
                put(c, GAME_GP.wrapping_sub(0x848), world);
                put(c, world + 0x84, game);
                put(c, game + 4, r.next_u32());
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 2) * 0x40);
            },
        );
    }
}
