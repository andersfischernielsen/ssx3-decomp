//! Staging module oc832 (wave wave249): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc832`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x00187580,
        cfe_state_mem_card_on_child_event,
        "cFEStateMemCard_OnChildEvent",
    ),
    (
        0x00187920,
        cfe_state_mem_card_start_system_load,
        "cFEStateMemCard_StartSystemLoad",
    ),
    (
        0x00187c18,
        fe_screen_autosave_popup_ctor,
        "FEScreenAutosavePopup_ctor",
    ),
    (
        0x00187c50,
        fe_screen_autosave_popup_init,
        "FEScreenAutosavePopup_Init",
    ),
    (
        0x00188870,
        cfe_state_options_on_widget_created,
        "cFEStateOptions_OnWidgetCreated",
    ),
    (
        0x00188980,
        cfe_state_options_on_widget_action,
        "cFEStateOptions_OnWidgetAction",
    ),
];

/// `lw off(reg)` (register `reg` holds the base).
#[inline(always)]
fn oc832_lw(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    c.lw(a)
}

/// `lh off(reg)`.
#[inline(always)]
fn oc832_lh(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    c.lh(a)
}

/// `sw value, off(reg)`.
#[inline(always)]
fn oc832_sw(c: &mut Cpu, reg: usize, off: u32, value: u64) {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    c.sw(a, value);
}

/// `lw -off(gp)`.
#[inline(always)]
fn oc832_gp_lw(c: &mut Cpu, off: u32) -> u64 {
    let a = (c.g[GP][0] as u32).wrapping_sub(off);
    c.lw(a)
}

/// `sw value, -off(gp)`.
#[inline(always)]
fn oc832_gp_sw(c: &mut Cpu, off: u32, value: u64) {
    let a = (c.g[GP][0] as u32).wrapping_sub(off);
    c.sw(a, value);
}

/// `void cFEStateMemCard_OnChildEvent(self, child, key)` (0x187580): the memory
/// card state's popup callback. After the base handler, state 0x15 starts the
/// system load or the profile search, or finishes; state 0xa and key 0xf finish;
/// states 0x4/0x14 handle the confirm keys (0x10 sets the retry flag, otherwise
/// the screen's vtable call and a rescan).
pub fn cfe_state_mem_card_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x001d_8700, 0x0018_759c)?;
        c.g[V1][0] = oc832_lw(c, S1, 0x19c);
        c.g[V0][0] = 0x15;
        let other_state = c.g[V1][0] != c.g[V0][0];
        c.g[V0][0] = 0xa;
        'done: {
            'rescan_or_set_flag: {
                'finish: {
                    'other: {
                        if other_state {
                            break 'other;
                        }
                        // State 0x15: start the load or search, or finish.
                        c.cycles += 12;
                        c.g[A0][0] = oc832_gp_lw(c, 0x848);
                        call_ptr(c, 0x0022_7f80, 0x0018_75b4)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A1][0] = 0;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0023_fb18, 0x0018_75c4)?;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0023_fae0, 0x0018_75cc)?;
                        c.g[V1][0] = oc832_lw(c, S1, 0x238);
                        c.g[V0][0] = c.g[V1][0] & 2;
                        let flagged = c.g[V0][0] != 0;
                        c.g[V0][0] = c.g[V1][0] ^ 1;
                        if !flagged {
                            c.cycles += 4;
                            call_ptr(c, 0x0018_7920, 0x0018_75e4)?;
                            frame.restore(c, S0, 0x20);
                            break 'done;
                        }
                        c.cycles += 3;
                        c.g[V0][0] &= 1;
                        if c.g[V0][0] == 0 {
                            break 'finish;
                        }
                        c.cycles += 4;
                        call_ptr(c, 0x0018_77b0, 0x0018_7600)?;
                        frame.restore(c, S0, 0x20);
                        break 'done;
                    }
                    // Other states: state 0xa with key 0xf, or state 4, finish.
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V1][0] = oc832_lw(c, S1, 0x19c);
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0xf;
                        if c.g[S0][0] == c.g[V0][0] {
                            break 'finish;
                        }
                        c.cycles += 1;
                        c.g[V1][0] = oc832_lw(c, S1, 0x19c);
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0x4;
                    let not_state4 = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xf));
                    if not_state4 {
                        break 'rescan_or_set_flag;
                    }
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0018_7318, 0x0018_7634)?;
                frame.restore(c, S0, 0x20);
                break 'done;
            }
            // Keys 0xf and 0x10 in state 0x14.
            c.cycles += 3;
            c.g[V0][0] = (c.g[V0][0] < 2u64) as u64;
            let wrong_key = c.g[V0][0] == 0;
            c.g[V0][0] = 0x14;
            if !wrong_key {
                c.cycles += 2;
                if c.g[V1][0] != c.g[V0][0] {
                    frame.restore(c, S0, 0x20);
                    break 'done;
                }
                c.cycles += 3;
                c.g[V0][0] = c.g[S0][0] ^ 0x10;
                let key_is_10 = c.g[V0][0] == 0;
                c.g[V1][0] = 1;
                if !key_is_10 {
                    c.cycles += 13;
                    c.g[V0][0] = oc832_lw(c, S1, 0x8);
                    c.g[A0][0] = oc832_lh(c, V0, 0x148);
                    c.g[V1][0] = oc832_lw(c, V0, 0x14c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0018_7670)?;
                    c.g[A0][0] = oc832_gp_lw(c, 0x848);
                    call_ptr(c, 0x0022_7f80, 0x0018_7678)?;
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0023_d618, 0x0018_7680)?;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0018_7270, 0x0018_7688)?;
                    frame.restore(c, S0, 0x20);
                    break 'done;
                }
                c.cycles += 1;
                oc832_sw(c, S1, 0x1ec, c.g[V1][0]);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateMemCard_StartSystemLoad(void)` (0x187920): set the memcard
/// state (`gp-0x1c34` = 4, state object `+0x1c0` = 5, `+0x1bc` = 2), start the
/// manager's system-file read of the current size and type, play animation 9 of
/// the "load" widget on the screen, call `SetLoadPressed(state, 1)` and set
/// `+0x1a8`.
pub fn cfe_state_mem_card_start_system_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = oc832_gp_lw(c, 0x1c38);
        c.g[V0][0] = 4;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[V1][0] = 5;
        frame.save_ra(c, 0);
        c.g[S1][0] = 2;
        oc832_gp_sw(c, 0x1c34, c.g[V0][0]);
        oc832_sw(c, A0, 0x1c0, c.g[V1][0]);
        call_ptr(c, 0x0014_dc80, 0x0018_794c)?;
        c.g[A1][0] = 5;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x0018_7958)?;
        c.g[A0][0] = oc832_gp_lw(c, 0x848);
        call_ptr(c, 0x0022_7f80, 0x0018_7960)?;
        c.g[V1][0] = oc832_gp_lw(c, 0x1c38);
        c.g[S0][0] = c.g[V0][0];
        oc832_sw(c, V1, 0x1bc, c.g[S1][0]);
        oc832_sw(c, V1, 0x1c4, 0);
        call_ptr(c, 0x0015_2948, 0x0018_7974)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0023_caa0, 0x0018_7980)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0023_c8f0, 0x0018_798c)?;
        c.g[V0][0] = oc832_lw(c, S0, 0x748);
        c.g[A1][0] = 0x35;
        c.g[A0][0] = oc832_lh(c, V0, 0x8);
        c.g[V1][0] = oc832_lw(c, V0, 0xc);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0018_79a4)?;
        c.g[S0][0] = oc832_gp_lw(c, 0x1c38);
        c.g[A0][0] = sx(0x004a_0000u32.wrapping_add(0x14c0));
        oc832_sw(c, S0, 0x1bc, c.g[S1][0]);
        call_ptr(c, 0x0031_7670, 0x0018_79b8)?;
        c.g[A0][0] = oc832_lw(c, S0, 0x40);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0018_79c4)?;
        c.g[S1][0] = c.g[V0][0];
        let no_widget = c.g[S1][0] == 0;
        c.g[V0][0] = oc832_gp_lw(c, 0x1c38);
        'played: {
            if no_widget {
                break 'played;
            }
            c.cycles += 10;
            c.g[A0][0] = sx(0x0046_0000u32.wrapping_sub(0x2300));
            c.g[S0][0] = oc832_lw(c, V0, 0x10);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
            call_ptr(c, 0x0031_7670, 0x0018_79e4)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_7140, 0x0018_79f0)?;
            let no_anim = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S1][0];
            if no_anim {
                break 'played;
            }
            c.cycles += 5;
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = 9;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0;
            call_ptr(c, 0x0039_fcc8, 0x0018_7a0c)?;
        }
        c.cycles += 11;
        c.g[A0][0] = oc832_gp_lw(c, 0x1c38);
        c.g[A1][0] = 1;
        call_ptr(c, 0x001d_9308, 0x0018_7a18)?;
        c.g[V1][0] = oc832_gp_lw(c, 0x1c38);
        c.g[V0][0] = 1;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        oc832_sw(c, V1, 0x1a8, c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

/// `FEScreenAutosavePopup* FEScreenAutosavePopup_ctor(this)` (0x187c18): base
/// `cUIState` constructor, then vtable `0x46b4a0`, screen (`+0x48`, the frame
/// counter) cleared; returns `this`.
pub fn fe_screen_autosave_popup_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e318, 0x0018_7c2c)?;
        c.g[V1][0] = 0x0047_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4b60));
        oc832_sw(c, S0, 0x48, 0);
        oc832_sw(c, S0, 0x8, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void FEScreenAutosavePopup_Init(this)` (0x187c50): add the autosave screen
/// to the engine (`this+0x10`) by hash name; keep it at `+0x40` and play its
/// frame 0; clear the frame counter (`+0x48`).
pub fn fe_screen_autosave_popup_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x22f0));
        c.g[S0][0] = oc832_lw(c, S1, 0x10);
        call_ptr(c, 0x0031_7670, 0x0018_7c74)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0018_7c88)?;
        let no_screen = c.g[V0][0] == 0;
        oc832_sw(c, S1, 0x40, c.g[V0][0]);
        if !no_screen {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0018_7ca0)?;
        }
        c.cycles += 6;
        oc832_sw(c, S1, 0x48, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The option widgets `cFEStateOptions_OnWidgetCreated` knows: (offset of the
/// name's string from 0x4a0000, value stored at `self+0x18`, return address of
/// the hash call).
const OC832_OPTION_WIDGETS: [(u32, u64, u32); 8] = [
    (0x14f8, 0x1c, 0x0018_8894),
    (0x1500, 0x1d, 0x0018_88b0),
    (0x1508, 0x1f, 0x0018_88cc),
    (0x1510, 0x1e, 0x0018_88e8),
    (0x1518, 0x20, 0x0018_8904),
    (0x1520, 0x24, 0x0018_8920),
    (0x1528, 0x24, 0x0018_893c),
    (0x1530, 0x22, 0x0018_8958),
];

/// `void cFEStateOptions_OnWidgetCreated(self, widget)` (0x188870): compare the
/// widget's hash name (`+0x38`) with the eight option-widget names in turn and
/// store the matching kind (0x1c, 0x1d, 0x1f, 0x1e, 0x20, 0x24, 0x24, 0x22) at
/// `self+0x18`.
pub fn cfe_state_options_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0x004a_0000;
        frame.save(c, S1, 0x10);
        c.g[A0][0] = sx(0x004a_0000u32.wrapping_add(OC832_OPTION_WIDGETS[0].0));
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = oc832_lw(c, S1, 0x38);
        call_ptr(c, 0x0031_7670, OC832_OPTION_WIDGETS[0].2)?;
        let mut kind = None;
        let first_matches = c.g[S0][0] == c.g[V0][0];
        c.g[A0][0] = 0x004a_0000;
        if first_matches {
            c.cycles += 2;
            kind = Some(OC832_OPTION_WIDGETS[0].1);
        } else {
            for (i, &(off, value, ret)) in OC832_OPTION_WIDGETS.iter().enumerate().skip(1) {
                c.cycles += 5;
                c.g[S0][0] = oc832_lw(c, S1, 0x38);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(off));
                call_ptr(c, 0x0031_7670, ret)?;
                if i == OC832_OPTION_WIDGETS.len() - 1 {
                    let no_match = c.g[S0][0] != c.g[V0][0];
                    frame.restore(c, S0, 0x20);
                    if !no_match {
                        c.cycles += 1;
                        kind = Some(value);
                    }
                } else if c.g[S0][0] != c.g[V0][0] {
                    c.g[A0][0] = 0x004a_0000;
                } else {
                    c.cycles += 2;
                    kind = Some(value);
                    break;
                }
            }
        }
        if let Some(value) = kind {
            c.g[V0][0] = value;
            c.cycles += 2;
            oc832_sw(c, S1, 0x18, c.g[V0][0]);
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared "unsaved options?" test of `cFEStateOptions_OnWidgetAction`
/// (cases 5 and 6): `true` if the popup must be opened instead (no pending
/// `gp-0x1c10` flag and one of the four change flags `gp-0x1c20`, `-0x1c1c`,
/// `-0x1c14`, `-0x1c18` set). On `false`, `v0` holds `self+0x1c`.
/// The case-5 copy (`case5`) spends one more cycle on its fall-through.
fn oc832_options_changed(c: &mut Cpu, case5: bool) -> bool {
    c.cycles += 3;
    c.g[V0][0] = oc832_gp_lw(c, 0x1c10);
    if c.g[V0][0] != 0 {
        c.g[V0][0] = oc832_lw(c, S1, 0x1c);
        return false;
    }
    c.cycles += 3;
    c.g[V0][0] = oc832_gp_lw(c, 0x1c20);
    let t = c.g[V0][0] != 0;
    c.g[V0][0] = oc832_gp_lw(c, 0x1c1c);
    if t {
        return true;
    }
    c.cycles += 2;
    let t = c.g[V0][0] != 0;
    c.g[V0][0] = oc832_gp_lw(c, 0x1c14);
    if t {
        return true;
    }
    c.cycles += 2;
    let t = c.g[V0][0] != 0;
    c.g[V0][0] = oc832_gp_lw(c, 0x1c18);
    if t {
        return true;
    }
    c.cycles += 2;
    if case5 {
        // The case-5 copy tests the last flag into a register first.
        let t = c.g[V0][0] != 0;
        if t {
            return true;
        }
        c.cycles += 1;
        c.g[V0][0] = oc832_lw(c, S1, 0x1c);
        return false;
    }
    if c.g[V0][0] == 0 {
        c.g[V0][0] = oc832_lw(c, S1, 0x1c);
        false
    } else {
        true
    }
}

/// The "transition out" of the options screen: clear `gp-0x1c10`, set bit 0x80
/// of `self+0x1c`, look up the "TransitionOut" frame label of the screen
/// (`self+0x40`). Returns whether the screen has such a frame (the caller plays it).
/// `rets` are the return addresses of the hash call and the label call.
fn oc832_options_find_transition(c: &mut Cpu, rets: [u32; 2]) -> Result<bool, Unwound> {
    c.cycles += 13;
    c.g[A0][0] = 0x0046_0000;
    oc832_gp_sw(c, 0x1c10, 0);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x23a0));
    c.g[V0][0] |= 0x80;
    oc832_sw(c, S1, 0x1c, c.g[V0][0]);
    call_ptr(c, 0x0031_7670, rets[0])?;
    c.g[A0][0] = oc832_lw(c, S1, 0x40);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_c778, rets[1])?;
    c.g[A1][0] = c.g[V0][0];
    c.g[V0][0] = 0xffff;
    let none = c.g[A1][0] == c.g[V0][0];
    c.g[A1][0] &= 0xffff;
    Ok(!none)
}

/// `void cFEStateOptions_OnWidgetAction(self, widget, event)` (0x188980): case 5
/// (button): the keyboard widget opens a cKeyboardPopup; the back widget asks
/// to save or plays "TransitionOut" and otherwise asks the engine's factory
/// (vtable slot 4) for a screen to push. Case 6 (back): the same save prompt or
/// transition. Case 13 (keyboard closed): apply the cheat code typed and play
/// the success (6) or failure (4) sound.
///
/// Returns through the common tail: `exit` 0 runs all of it, 1 skips the first
/// `cycles += 1` + `lq s0`, 2 only the epilogue.
pub fn cfe_state_options_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x6;
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A1][0];
        let is_back = c.g[A2][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        let exit: u8;
        if is_back {
            exit = oc832_options_back(c, &frame)?;
        } else {
            c.cycles += 3;
            c.g[V0][0] = (c.g[A2][0] < 0x7u64) as u64;
            let high = c.g[V0][0] == 0;
            c.g[V0][0] = 0x5;
            if !high {
                c.cycles += 2;
                let is_button = c.g[A2][0] == c.g[V0][0];
                frame.restore(c, S0, 0x30);
                if is_button {
                    exit = oc832_options_button(c, &frame)?;
                } else {
                    c.cycles += 2;
                    frame.restore(c, S1, 0x20);
                    exit = 2;
                }
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0xd;
                let is_closed = c.g[A2][0] == c.g[V0][0];
                frame.restore(c, S0, 0x30);
                if is_closed {
                    exit = oc832_options_keyboard_closed(c)?;
                } else {
                    c.cycles += 2;
                    frame.restore(c, S1, 0x20);
                    exit = 2;
                }
            }
        }
        if exit == 0 {
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        if exit <= 1 {
            c.cycles += 1;
            frame.restore(c, S1, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Opens the "save options?" popup (`0x188e58`); returns the tail level 1.
fn oc832_options_open_prompt(c: &mut Cpu, frame: &Frame) -> Result<u8, Unwound> {
    c.cycles += 4;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x0018_8e58, 0x0018_8b8c)?;
    frame.restore(c, S0, 0x30);
    Ok(1)
}

/// Event 6 of `cFEStateOptions_OnWidgetAction`: back out of the options screen.
fn oc832_options_back(c: &mut Cpu, frame: &Frame) -> Result<u8, Unwound> {
    if oc832_options_changed(c, false) {
        return oc832_options_open_prompt(c, frame);
    }
    if !oc832_options_find_transition(c, [0x0018_8bac, 0x0018_8bb8])? {
        return Ok(0);
    }
    c.cycles += 5;
    c.g[A0][0] = oc832_lw(c, S1, 0x40);
    c.g[A2][0] = 1;
    call_ptr(c, 0x0039_c870, 0x0018_8bd4)?;
    frame.restore(c, S0, 0x30);
    Ok(1)
}

/// Event 5 of `cFEStateOptions_OnWidgetAction`: a button was pressed.
fn oc832_options_button(c: &mut Cpu, frame: &Frame) -> Result<u8, Unwound> {
    c.cycles += 6;
    c.g[A0][0] = 0x004a_0000;
    c.g[S0][0] = oc832_lw(c, S2, 0x38);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1520));
    call_ptr(c, 0x0031_7670, 0x0018_89e4)?;
    if c.g[S0][0] == c.g[V0][0] {
        // The keyboard widget: open a keyboard popup for the name.
        c.cycles += 32;
        c.g[A1][0] = 0x0046_0000;
        c.g[A0][0] = 0x444;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2728));
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0018_8a04)?;
        c.g[T0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x15));
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = oc832_lw(c, S1, 0x10);
        c.g[A3][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x001c_b030, 0x0018_8a1c)?;
        c.g[S0][0] = 0x004a_0000;
        c.g[A0][0] = c.g[V0][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1408));
        oc832_sw(c, S1, 0x50, c.g[V0][0]);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x001c_b418, 0x0018_8a34)?;
        c.g[A0][0] = oc832_lw(c, S1, 0x50);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x001c_cf00, 0x0018_8a40)?;
        c.g[A0][0] = oc832_lw(c, S1, 0x50);
        c.g[A1][0] = 0x4b;
        c.g[A2][0] = 1;
        c.g[A3][0] = 2;
        call_ptr(c, 0x001c_e3c8, 0x0018_8a54)?;
        c.g[A0][0] = oc832_lw(c, S1, 0x10);
        c.g[A1][0] = oc832_lw(c, S1, 0x50);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0039_f290, 0x0018_8a64)?;
        frame.restore(c, S0, 0x30);
        return Ok(1);
    }
    c.g[A0][0] = 0x004a_0000;
    c.cycles += 5;
    c.g[S0][0] = oc832_lw(c, S2, 0x38);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1530));
    call_ptr(c, 0x0031_7670, 0x0018_8a78)?;
    if c.g[S0][0] != c.g[V0][0] {
        c.g[V0][0] = oc832_lw(c, S1, 0x10);
    } else {
        // The back widget.
        if oc832_options_changed(c, true) {
            return oc832_options_open_prompt(c, frame);
        }
        if oc832_options_find_transition(c, [0x0018_8acc, 0x0018_8ad8])? {
            c.cycles += 3;
            c.g[A0][0] = oc832_lw(c, S1, 0x40);
            c.g[A2][0] = 1;
            call_ptr(c, 0x0039_c870, 0x0018_8af4)?;
        }
        c.cycles += 3;
        c.g[V0][0] = oc832_lw(c, S1, 0x48);
        let no_save_screen = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x30);
        if no_save_screen {
            return Ok(1);
        }
        c.cycles += 4;
        c.g[V0][0] = oc832_lw(c, S1, 0x10);
        c.g[V1][0] = oc832_lw(c, V0, 0);
        let no_custom = c.g[V1][0] == 0;
        c.g[V0][0] = 0x18;
        if !no_custom {
            c.cycles += 1;
            oc832_sw(c, V1, 0xc, c.g[V0][0]);
        }
        c.cycles += 1;
        c.g[V0][0] = oc832_lw(c, S1, 0x10);
    }
    // Ask the engine's factory (vtable slot 4) for the next screen and push it.
    c.cycles += 10;
    c.g[A1][0] = c.g[S1][0];
    c.g[A2][0] = oc832_lw(c, S2, 0x18);
    c.g[A3][0] = oc832_lw(c, V0, 0);
    c.g[V1][0] = oc832_lw(c, A3, 0x4);
    c.g[A0][0] = oc832_lh(c, V1, 0x20);
    c.g[V0][0] = oc832_lw(c, V1, 0x24);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x0018_8b38)?;
    let no_screen = c.g[V0][0] == 0;
    c.g[A1][0] = c.g[V0][0];
    if no_screen {
        return Ok(0);
    }
    c.cycles += 5;
    c.g[A0][0] = oc832_lw(c, S1, 0x10);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
    call_ptr(c, 0x0039_f400, 0x0018_8b4c)?;
    frame.restore(c, S0, 0x30);
    Ok(1)
}

/// Event 13 of `cFEStateOptions_OnWidgetAction`: the keyboard popup closed;
/// apply the cheat code (`0x187d38` on the popup's text), play sound 6 if it
/// worked, else 4, on the engine's sound object (or the null sound at
/// `gp+0x2968`), and forget the popup.
fn oc832_options_keyboard_closed(c: &mut Cpu) -> Result<u8, Unwound> {
    c.cycles += 10;
    c.g[A0][0] = oc832_lw(c, S1, 0x50);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x74));
    call_ptr(c, 0x0018_7d38, 0x0018_8be8)?;
    c.g[V1][0] = oc832_lw(c, S1, 0x10);
    c.g[A0][0] = c.g[V0][0];
    c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2968));
    c.g[V0][0] = oc832_lw(c, V1, 0x14);
    if c.g[V0][0] != 0 {
        c.g[A2][0] = c.g[V0][0];
    }
    let no_sound = c.g[A2][0] == 0;
    c.g[V0][0] = 1;
    if !no_sound {
        c.cycles += 2;
        let failed = c.g[A0][0] != c.g[V0][0];
        c.g[V0][0] = oc832_lw(c, A2, 0);
        if !failed {
            c.cycles += 7;
            c.g[A1][0] = 6;
            c.g[A0][0] = oc832_lh(c, V0, 0x10);
            c.g[V1][0] = oc832_lw(c, V0, 0x14);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0018_8c20)?;
            oc832_sw(c, S1, 0x50, 0);
            return Ok(0);
        }
        c.cycles += 5;
        c.g[A1][0] = 4;
        c.g[A0][0] = oc832_lh(c, V0, 0x10);
        c.g[V1][0] = oc832_lw(c, V0, 0x14);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0018_8c3c)?;
    }
    c.cycles += 1;
    oc832_sw(c, S1, 0x50, 0);
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{check, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// The gate word of the fakes (a machine without it runs the real callees).
    const OC832_CTRL: u32 = DATA + 0x30_0000;
    const OC832_MAGIC: u32 = 0x4f43_8320;
    /// Per-callee forced results: one word per fake at `OC832_FORCE + (addr & 0xffffc)`.
    const OC832_FORCE: u32 = DATA + 0x30_1000;
    const OC832_OBJ: u32 = DATA + 0x8_0000;
    const OC832_WIDGET: u32 = DATA + 0x8_1000;
    const OC832_ENGINE: u32 = DATA + 0x8_2000;
    const OC832_CUSTOM: u32 = DATA + 0x8_3000;
    const OC832_VT: u32 = DATA + 0x8_4000;
    const OC832_KEYBOARD: u32 = DATA + 0x8_5000;
    const OC832_SOUND: u32 = DATA + 0x8_6000;
    const OC832_SOUND_VT: u32 = DATA + 0x8_7000;
    const OC832_WORLD: u32 = DATA + 0x8_8000;
    /// The hash every widget name test compares against.
    const OC832_HASH: u32 = 0x1234;

    fn oc832_w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// Make the fake for `addr` return 0.
    fn oc832_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(OC832_FORCE + (addr & 0xf_fffc), 1);
    }

    /// Make the fake for `addr` return `v` (stored +3).
    fn oc832_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus
            .write32(OC832_FORCE + (addr & 0xf_fffc), v.wrapping_add(3));
    }

    /// The fake callee: logs like `fakes::fake_call`, then returns the forced
    /// result of its mailbox (0 = the fake's pointer) unless it unwound.
    fn oc832_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(OC832_FORCE + (addr & 0xf_fffc));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = if mail == 1 { 0 } else { (mail - 3) as u64 };
        }
    }

    macro_rules! oc832_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC832_CTRL) != OC832_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc832_fake_call(c, $addr);
                }
            )*
            fn oc832_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc832_fakes!(
        (
            oc832_fake_1d8700,
            OC832_PREV_1D8700,
            ee::s04::OVERRIDE_001D8700,
            ee::s04::f_001d8700__generated,
            0x001d_8700
        ),
        (
            oc832_fake_227f80,
            OC832_PREV_227F80,
            ee::s05::OVERRIDE_00227F80,
            ee::s05::f_00227f80__generated,
            0x0022_7f80
        ),
        (
            oc832_fake_23fb18,
            OC832_PREV_23FB18,
            ee::s06::OVERRIDE_0023FB18,
            ee::s06::f_0023fb18__generated,
            0x0023_fb18
        ),
        (
            oc832_fake_23fae0,
            OC832_PREV_23FAE0,
            ee::s06::OVERRIDE_0023FAE0,
            ee::s06::f_0023fae0__generated,
            0x0023_fae0
        ),
        (
            oc832_fake_1877b0,
            OC832_PREV_1877B0,
            ee::s02::OVERRIDE_001877B0,
            ee::s02::f_001877b0__generated,
            0x0018_77b0
        ),
        (
            oc832_fake_187318,
            OC832_PREV_187318,
            ee::s02::OVERRIDE_00187318,
            ee::s02::f_00187318__generated,
            0x0018_7318
        ),
        (
            oc832_fake_187270,
            OC832_PREV_187270,
            ee::s02::OVERRIDE_00187270,
            ee::s02::f_00187270__generated,
            0x0018_7270
        ),
        (
            oc832_fake_23d618,
            OC832_PREV_23D618,
            ee::s06::OVERRIDE_0023D618,
            ee::s06::f_0023d618__generated,
            0x0023_d618
        ),
        (
            oc832_fake_14dc80,
            OC832_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc832_fake_14dd58,
            OC832_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc832_fake_152948,
            OC832_PREV_152948,
            ee::s01::OVERRIDE_00152948,
            ee::s01::f_00152948__generated,
            0x0015_2948
        ),
        (
            oc832_fake_23caa0,
            OC832_PREV_23CAA0,
            ee::s06::OVERRIDE_0023CAA0,
            ee::s06::f_0023caa0__generated,
            0x0023_caa0
        ),
        (
            oc832_fake_23c8f0,
            OC832_PREV_23C8F0,
            ee::s06::OVERRIDE_0023C8F0,
            ee::s06::f_0023c8f0__generated,
            0x0023_c8f0
        ),
        (
            oc832_fake_317670,
            OC832_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc832_fake_39d860,
            OC832_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc832_fake_397140,
            OC832_PREV_397140,
            ee::s13::OVERRIDE_00397140,
            ee::s13::f_00397140__generated,
            0x0039_7140
        ),
        (
            oc832_fake_39fcc8,
            OC832_PREV_39FCC8,
            ee::s13::OVERRIDE_0039FCC8,
            ee::s13::f_0039fcc8__generated,
            0x0039_fcc8
        ),
        (
            oc832_fake_1d9308,
            OC832_PREV_1D9308,
            ee::s04::OVERRIDE_001D9308,
            ee::s04::f_001d9308__generated,
            0x001d_9308
        ),
        (
            oc832_fake_39e318,
            OC832_PREV_39E318,
            ee::s13::OVERRIDE_0039E318,
            ee::s13::f_0039e318__generated,
            0x0039_e318
        ),
        (
            oc832_fake_397cc0,
            OC832_PREV_397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x0039_7cc0
        ),
        (
            oc832_fake_39c870,
            OC832_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc832_fake_1cb418,
            OC832_PREV_1CB418,
            ee::s04::OVERRIDE_001CB418,
            ee::s04::f_001cb418__generated,
            0x001c_b418
        ),
        (
            oc832_fake_1ccf00,
            OC832_PREV_1CCF00,
            ee::s04::OVERRIDE_001CCF00,
            ee::s04::f_001ccf00__generated,
            0x001c_cf00
        ),
        (
            oc832_fake_1ce3c8,
            OC832_PREV_1CE3C8,
            ee::s04::OVERRIDE_001CE3C8,
            ee::s04::f_001ce3c8__generated,
            0x001c_e3c8
        ),
        (
            oc832_fake_188e58,
            OC832_PREV_188E58,
            ee::s02::OVERRIDE_00188E58,
            ee::s02::f_00188e58__generated,
            0x0018_8e58
        ),
        (
            oc832_fake_39f400,
            OC832_PREV_39F400,
            ee::s13::OVERRIDE_0039F400,
            ee::s13::f_0039f400__generated,
            0x0039_f400
        ),
        (
            oc832_fake_317d70,
            OC832_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc832_fake_39c778,
            OC832_PREV_39C778,
            ee::s13::OVERRIDE_0039C778,
            ee::s13::f_0039c778__generated,
            0x0039_c778
        ),
        (
            oc832_fake_3180b8,
            OC832_PREV_3180B8,
            ee::s10::OVERRIDE_003180B8,
            ee::s10::f_003180b8__generated,
            0x0031_80b8
        ),
        (
            oc832_fake_4166f4,
            OC832_PREV_4166F4,
            ee::s15::OVERRIDE_004166F4,
            ee::s15::f_004166f4__generated,
            0x0041_66f4
        ),
        (
            oc832_fake_2c2868,
            OC832_PREV_2C2868,
            ee::s08::OVERRIDE_002C2868,
            ee::s08::f_002c2868__generated,
            0x002c_2868
        ),
        (
            oc832_fake_158618,
            OC832_PREV_158618,
            ee::s01::OVERRIDE_00158618,
            ee::s01::f_00158618__generated,
            0x0015_8618
        ),
    );

    /// Scrambled machine with the fakes' gate and the world at `gp-0x848`.
    fn oc832_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) {
        oc832_install_fakes();
        let _ = machine(c, seed, salt, calls);
        c.bus.write32(OC832_CTRL, OC832_MAGIC);
        oc832_w(c, GAME_GP.wrapping_sub(0x848), OC832_WORLD);
    }

    /// `cFEStateMemCard_OnChildEvent`: every (state, key, flags) combination.
    #[test]
    fn cfe_state_mem_card_on_child_event_matches() {
        check(
            ee::s02::f_00187580__generated,
            cfe_state_mem_card_on_child_event,
            60,
            &|c, seed| {
                oc832_machine(c, seed, 0x187580, 8);
                c.pc = 0x0018_7580;
                let state = [0x15u32, 0xa, 0x4, 0x14, 7][(seed % 5) as usize];
                let key = [0xfu32, 0x10, 5][(seed / 5 % 3) as usize];
                let flags = seed / 15 % 4;
                c.g[A0][0] = sx(OC832_OBJ);
                c.g[A2][0] = key as u64;
                oc832_w(c, OC832_OBJ + 0x19c, state);
                oc832_w(c, OC832_OBJ + 0x238, flags);
                oc832_w(c, OC832_OBJ + 8, OC832_VT);
                c.bus.write16(OC832_VT + 0x148, 0x10 * (seed % 3) as u16);
                oc832_w(c, OC832_VT + 0x14c, FAKE + 0x10);
            },
        );
    }

    /// `cFEStateMemCard_StartSystemLoad`: with and without the load widget and
    /// its animation.
    #[test]
    fn cfe_state_mem_card_start_system_load_matches() {
        check(
            ee::s02::f_00187920__generated,
            cfe_state_mem_card_start_system_load,
            12,
            &|c, seed| {
                oc832_machine(c, seed, 0x187920, 13);
                c.pc = 0x0018_7920;
                oc832_w(c, GAME_GP.wrapping_sub(0x1c38), OC832_OBJ);
                oc832_w(c, OC832_OBJ + 0x10, OC832_ENGINE);
                oc832_w(c, OC832_OBJ + 0x40, OC832_WIDGET);
                match seed % 3 {
                    1 => oc832_force_zero(c, 0x0039_d860),
                    2 => oc832_force_zero(c, 0x0039_7140),
                    _ => {}
                }
            },
        );
    }

    /// `FEScreenAutosavePopup_ctor`.
    #[test]
    fn fe_screen_autosave_popup_ctor_matches() {
        check(
            ee::s02::f_00187c18__generated,
            fe_screen_autosave_popup_ctor,
            4,
            &|c, seed| {
                oc832_machine(c, seed, 0x187c18, 1);
                c.pc = 0x0018_7c18;
                c.g[A0][0] = sx(OC832_OBJ);
                oc832_w(c, OC832_OBJ + 0x48, 77);
            },
        );
    }

    /// `FEScreenAutosavePopup_Init`: with and without the screen.
    #[test]
    fn fe_screen_autosave_popup_init_matches() {
        check(
            ee::s02::f_00187c50__generated,
            fe_screen_autosave_popup_init,
            8,
            &|c, seed| {
                oc832_machine(c, seed, 0x187c50, 3);
                c.pc = 0x0018_7c50;
                c.g[A0][0] = sx(OC832_OBJ);
                oc832_w(c, OC832_OBJ + 0x10, OC832_ENGINE);
                oc832_w(c, OC832_OBJ + 0x48, 77);
                if seed % 2 == 1 {
                    oc832_force_zero(c, 0x0039_7cc0);
                }
            },
        );
    }

    /// `cFEStateOptions_OnWidgetCreated`: the widget name matches each of the
    /// eight option names in turn, or none.
    #[test]
    fn cfe_state_options_on_widget_created_matches() {
        check(
            ee::s02::f_00188870__generated,
            cfe_state_options_on_widget_created,
            18,
            &|c, seed| {
                oc832_machine(c, seed, 0x188870, 8);
                c.pc = 0x0018_8870;
                c.g[A1][0] = sx(OC832_WIDGET);
                oc832_w(c, OC832_WIDGET + 0x38, OC832_HASH);
                oc832_w(c, OC832_OBJ + 0x18, 0x77);
                let m = seed % 9;
                if m < 8 {
                    oc832_w(c, LOG_RESULTS + 4 * m, OC832_HASH);
                }
            },
        );
    }

    /// The screen, widget and engine `cFEStateOptions_OnWidgetAction` reads.
    fn oc832_action_fixture(c: &mut Cpu, key: u32) {
        c.pc = 0x0018_8980;
        c.g[A0][0] = sx(OC832_OBJ);
        c.g[A1][0] = sx(OC832_WIDGET);
        c.g[A2][0] = key as u64;
        oc832_w(c, OC832_OBJ + 0x10, OC832_ENGINE);
        oc832_w(c, OC832_OBJ + 0x1c, 3);
        oc832_w(c, OC832_OBJ + 0x40, 0x7777);
        oc832_w(c, OC832_WIDGET + 0x38, OC832_HASH);
        oc832_w(c, OC832_WIDGET + 0x18, 0x55);
        oc832_w(c, OC832_ENGINE, OC832_CUSTOM);
        oc832_w(c, OC832_CUSTOM + 4, OC832_VT);
        c.bus.write16(OC832_VT + 0x20, 0x10);
        oc832_w(c, OC832_VT + 0x24, FAKE + 0x20);
    }

    /// Set the "options changed" flag number `which` (1: `gp-0x1c10`, 2..5: the
    /// four change flags).
    fn oc832_option_flag(c: &mut Cpu, which: u32) {
        let off = [0, 0x1c10, 0x1c20, 0x1c1c, 0x1c14, 0x1c18][which as usize];
        if off != 0 {
            oc832_w(c, GAME_GP.wrapping_sub(off), 1);
        }
    }

    /// `cFEStateOptions_OnWidgetAction`, event 6 (back).
    #[test]
    fn cfe_state_options_on_widget_action_back_matches() {
        check(
            ee::s02::f_00188980__generated,
            cfe_state_options_on_widget_action,
            24,
            &|c, seed| {
                oc832_machine(c, seed, 0x188980, 4);
                oc832_action_fixture(c, 6);
                oc832_option_flag(c, seed % 6);
                if seed / 6 % 2 == 1 {
                    oc832_w(c, LOG_RESULTS + 4, 0xffff);
                }
            },
        );
    }

    /// `cFEStateOptions_OnWidgetAction`, events that do nothing.
    #[test]
    fn cfe_state_options_on_widget_action_other_matches() {
        check(
            ee::s02::f_00188980__generated,
            cfe_state_options_on_widget_action,
            10,
            &|c, seed| {
                oc832_machine(c, seed, 0x188980, 1);
                let key = [0u32, 1, 2, 3, 4, 7, 8, 0xc, 0xe, 0x20][(seed % 10) as usize];
                oc832_action_fixture(c, key);
            },
        );
    }

    /// `cFEStateOptions_OnWidgetAction`, event 5 on the back widget or on a
    /// widget that asks the engine's factory for a screen.
    #[test]
    fn cfe_state_options_on_widget_action_button_matches() {
        check(
            ee::s02::f_00188980__generated,
            cfe_state_options_on_widget_action,
            192,
            &|c, seed| {
                oc832_machine(c, seed, 0x188981, 6);
                oc832_action_fixture(c, 5);
                let back_widget = seed % 2 == 1;
                oc832_option_flag(c, seed / 2 % 6);
                if seed / 12 % 2 == 1 {
                    oc832_w(c, OC832_OBJ + 0x48, 1);
                }
                if seed / 24 % 2 == 1 {
                    oc832_w(c, OC832_ENGINE, 0);
                }
                if seed / 48 % 2 == 1 {
                    // A real leaf returning 0: the factory has no screen.
                    oc832_w(c, OC832_VT + 0x24, 0x0014_06c0);
                }
                if seed / 96 % 2 == 1 {
                    oc832_w(c, LOG_RESULTS + 12, 0xffff);
                }
                if back_widget {
                    oc832_w(c, LOG_RESULTS + 4, OC832_HASH);
                }
            },
        );
    }

    /// `cFEStateOptions_OnWidgetAction`, event 5 on the keyboard widget. Its
    /// callees `cKeyboardPopup_cKeyboardPopup` (0x1cb030) and
    /// `cUIStateStack_pushPopup` (0x39f290) have no override slot, so they run
    /// for real.
    #[test]
    fn cfe_state_options_on_widget_action_keyboard_matches() {
        check(
            ee::s02::f_00188980__generated,
            cfe_state_options_on_widget_action,
            4,
            &|c, seed| {
                oc832_machine(c, seed, 0x188982, 8);
                oc832_action_fixture(c, 5);
                oc832_w(c, OC832_OBJ + 0x14, 0x0000_ab00);
                oc832_w(c, LOG_RESULTS, OC832_HASH);
            },
        );
    }

    /// `cFEStateOptions_OnWidgetAction`, event 13: the typed cheat code worked
    /// (the cheat hash is forced) or not; the sound object is the engine's or
    /// the null sound.
    #[test]
    fn cfe_state_options_on_widget_action_keyboard_closed_matches() {
        check(
            ee::s02::f_00188980__generated,
            cfe_state_options_on_widget_action,
            8,
            &|c, seed| {
                oc832_machine(c, seed, 0x188983, 24);
                oc832_action_fixture(c, 13);
                oc832_w(c, OC832_OBJ + 0x50, OC832_KEYBOARD);
                if seed & 1 != 0 {
                    oc832_force(c, 0x0031_7670, 0x7_e68d);
                }
                oc832_w(c, GAME_GP.wrapping_add(0x2968), OC832_SOUND_VT);
                c.bus.write16(OC832_SOUND_VT + 0x10, 8);
                oc832_w(c, OC832_SOUND_VT + 0x14, FAKE + 0x30);
                if seed & 2 != 0 {
                    oc832_w(c, OC832_ENGINE + 0x14, OC832_SOUND);
                    oc832_w(c, OC832_SOUND, OC832_SOUND_VT);
                }
            },
        );
    }
}
