//! Staging module oc842 (wave wave249): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc842`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0018_e478,
        cfe_state_options_device_select_v33,
        "cFEStateOptionsDeviceSelect_v33",
    ),
    (
        0x0018_ea98,
        fe_screen_select_profile_ctor,
        "FEScreenSelectProfile_ctor",
    ),
    (
        0x0018_eb40,
        fe_select_profile_on_widget_action,
        "FEScreenSelectProfile_OnWidgetAction",
    ),
    (
        0x0018_eca0,
        cfe_state_profile_load_ctor,
        "cFEStateProfileLoad_ctor",
    ),
    (
        0x0018_edc8,
        cfe_state_profile_load_init,
        "cFEStateProfileLoad_Init",
    ),
    (
        0x0018_ee98,
        cfe_state_profile_load_render,
        "cFEStateProfileLoad_Render",
    ),
];

/// `reg + off` as a guest address.
#[inline(always)]
fn off842(reg: u64, off: u32) -> u32 {
    (reg as u32).wrapping_add(off)
}

/// `UIStateMgr_SwitchStateClosePopups` (0x39f4c0).
pub(crate) const OC842_SWITCH_CLOSE_POPUPS: u32 = 0x0039_f4c0;
/// `UIStateMgr_SwitchState` (0x39f400).
pub(crate) const OC842_SWITCH_STATE: u32 = 0x0039_f400;

/// `int cFEStateOptionsDeviceSelect_v33(this)` (0x18e478): once per screen
/// (guard word at `+0x1ac`), ask the owner's engine for the next state through
/// its vtable (slot `+0x28/+0x2c`, with a flag argument in a2: set when the
/// word at `+0x1bc` is 0, `+0x1e4` is non-zero and `+0x1b4` is non-zero, or
/// forced to 2 when `+0x230` is non-zero), and if it returns one, switch to it
/// with the owner's state manager (`owner + 0x18`). Returns 1; the guard is
/// stored as 1.
pub fn cfe_state_options_device_select_v33(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw(off842(c.g[S0][0], 0x1ac));
        let done = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if !done {
            c.cycles += 21;
            c.g[V1][0] = c.lw(off842(c.g[S0][0], 0x10));
            c.g[T2][0] = 2;
            c.g[A2][0] = c.lw(off842(c.g[S0][0], 0x1bc));
            c.g[A1][0] = c.g[S0][0];
            c.g[T1][0] = c.lw(c.g[V1][0] as u32);
            c.g[V0][0] = c.lw(off842(c.g[S0][0], 0x1e4));
            c.g[A2][0] ^= 1;
            c.g[T0][0] = c.lw(off842(c.g[T1][0], 4));
            c.g[A2][0] = (c.g[A2][0] < 1) as u64;
            c.g[V0][0] ^= 1;
            c.g[V1][0] = c.lw(off842(c.g[S0][0], 0x1b4));
            if c.g[V0][0] == 0 {
                c.g[A2][0] = 0;
            }
            c.g[A3][0] = c.lw(off842(c.g[S0][0], 0x230));
            c.g[A0][0] = c.lh(off842(c.g[T0][0], 0x28));
            if c.g[V1][0] == 0 {
                c.g[A2][0] = 0;
            }
            c.g[V0][0] = c.lw(off842(c.g[T0][0], 0x2c));
            if c.g[A3][0] != 0 {
                c.g[A2][0] = c.g[T2][0];
            }
            let method = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, method, 0x0018_e4e0)?;
            let none = c.g[V0][0] == 0;
            c.g[A1][0] = c.g[V0][0];
            if !none {
                c.cycles += 3;
                c.g[A0][0] = c.lw(off842(c.g[S0][0], 0x10));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                call_ptr(c, OC842_SWITCH_CLOSE_POPUPS, 0x0018_e4f4)?;
            }
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        c.cycles += 5;
        frame.restore_ra(c, 0);
        let guard = c.g[V0][0];
        c.sw(off842(c.g[S0][0], 0x1ac), guard);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `FEScreenSelectProfile::FEScreenSelectProfile(this)` (0x18ea98): the
/// `UIState` base constructor, then `this->+0xc = 8` and the screen's vtable
/// at `+8`. Returns `this`.
pub fn fe_screen_select_profile_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e2a0, 0x0018_eaac)?;
        c.g[V0][0] = 8;
        c.g[V1][0] = 0x0047_0000;
        c.sw(off842(c.g[S0][0], 0xc), c.g[V0][0]);
        c.g[V1][0] = sx(0x0047_0000u32.wrapping_sub(0x3ea8));
        c.sw(off842(c.g[S0][0], 0x8), c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void FEScreenSelectProfile_OnWidgetAction(self, menu, action)` (0x18eb40):
/// a widget action on a profile-select row. The widget (`menu`) is recoloured
/// through its vtable (`+0x58/+0x5c`): white for action 2, magenta for
/// action 1, yellow for action 5. Actions 5 and 6 then run the owner's
/// action method (`*(self+0x10)`'s engine vtable, `+0x20/+0x24` for 5 and
/// `+0x28/+0x2c` for 6, with `menu->+0x18`); a non-null result is pushed with
/// `UIStateMgr_SwitchState(owner + 0x18, result)`. Other actions do nothing.
pub fn fe_select_profile_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        let no_menu = c.g[S0][0] == 0;
        c.g[S1][0] = c.g[A0][0];
        // `'out`: the exits that already restored s0 (skip the `+1` block).
        'out: {
            // `'skip`: the exits that run the `+1` block, which restores s0.
            'skip: {
                if no_menu {
                    break 'skip;
                }
                c.cycles += 3;
                c.g[V0][0] = 2;
                let is2 = c.g[A2][0] == c.g[V0][0];
                c.g[V0][0] = (c.g[A2][0] < 3) as u64;
                if is2 {
                    // Action 2: white (1, 1, 1, 1).
                    c.cycles += 12;
                    c.g[V1][0] = c.lw(off842(c.g[S0][0], 0x8));
                    c.g[A1][0] = c.g[SP][0];
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 0);
                    c.g[A0][0] = c.lh(off842(c.g[V1][0], 0x58));
                    c.swc1(0, off842(c.g[SP][0], 0xc));
                    c.swc1(0, c.g[SP][0] as u32);
                    c.swc1(0, off842(c.g[SP][0], 4));
                    c.swc1(0, off842(c.g[SP][0], 8));
                    c.g[V0][0] = c.lw(off842(c.g[V1][0], 0x5c));
                    let method = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, method, 0x0018_ec8c)?;
                    break 'skip;
                }
                c.cycles += 2;
                let ge3 = c.g[V0][0] == 0;
                c.g[V0][0] = 1;
                if !ge3 {
                    c.cycles += 2;
                    if c.g[A2][0] == c.g[V0][0] {
                        // Action 1: magenta (1, 0, 1, 0).
                        c.g[V1][0] = c.lw(off842(c.g[S0][0], 0x8));
                        c.cycles += 13;
                        c.g[A1][0] = c.g[SP][0];
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(1, 0);
                        c.g[A0][0] = c.lh(off842(c.g[V1][0], 0x58));
                        c.swc1(0, off842(c.g[SP][0], 8));
                        c.swc1(0, c.g[SP][0] as u32);
                        c.sw(off842(c.g[SP][0], 4), 0);
                        c.sw(off842(c.g[SP][0], 0xc), 0);
                        c.g[V0][0] = c.lw(off842(c.g[V1][0], 0x5c));
                        let method = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, method, 0x0018_ec54)?;
                        frame.restore(c, S0, 0x30);
                        break 'out;
                    }
                    c.cycles += 2;
                    frame.restore(c, S0, 0x30);
                    break 'out;
                }
                c.cycles += 3;
                c.g[V0][0] = 5;
                if c.g[A2][0] == c.g[V0][0] {
                    // Action 5: yellow (1, 1, 0, 0), then the owner's method.
                    c.g[V1][0] = c.lw(off842(c.g[S0][0], 0x8));
                    c.cycles += 19;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 0);
                    c.g[A0][0] = c.lh(off842(c.g[V1][0], 0x58));
                    c.swc1(0, off842(c.g[SP][0], 4));
                    c.swc1(0, c.g[SP][0] as u32);
                    c.sw(off842(c.g[SP][0], 8), 0);
                    c.sw(off842(c.g[SP][0], 0xc), 0);
                    c.g[V0][0] = c.lw(off842(c.g[V1][0], 0x5c));
                    let method = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, method, 0x0018_ebcc)?;
                    c.g[A0][0] = c.lw(off842(c.g[S1][0], 0x10));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.lw(off842(c.g[S0][0], 0x18));
                    c.g[A3][0] = c.lw(c.g[A0][0] as u32);
                    c.g[V1][0] = c.lw(off842(c.g[A3][0], 4));
                    c.g[A0][0] = c.lh(off842(c.g[V1][0], 0x20));
                    c.g[V0][0] = c.lw(off842(c.g[V1][0], 0x24));
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 6;
                    if c.g[A2][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw(off842(c.g[S1][0], 0x10));
                    } else {
                        c.cycles += 2;
                        frame.restore(c, S0, 0x30);
                        break 'out;
                    }
                    // Action 6: just the owner's method.
                    c.cycles += 6;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.lw(off842(c.g[S0][0], 0x18));
                    c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V1][0] = c.lw(off842(c.g[A3][0], 4));
                    c.g[A0][0] = c.lh(off842(c.g[V1][0], 0x28));
                    c.g[V0][0] = c.lw(off842(c.g[V1][0], 0x2c));
                }
                // Shared tail: run the method, push a non-null result.
                c.cycles += 4;
                let method = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, method, 0x0018_ec0c)?;
                let none = c.g[V0][0] == 0;
                c.g[A1][0] = c.g[V0][0];
                if none {
                    break 'skip;
                }
                c.cycles += 5;
                c.g[A0][0] = c.lw(off842(c.g[S1][0], 0x10));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                call_ptr(c, OC842_SWITCH_STATE, 0x0018_ec20)?;
                frame.restore(c, S0, 0x30);
                break 'out;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateProfileLoad::cFEStateProfileLoad(this, a1, mode, flags)`
/// (0x18eca0): the memory-card state constructor (a2 = 0), then the vtable and
/// `+0xc = 0x23`; `mode` goes to the global at `gp-0x1b08`; the fields at
/// `+0x1cc/+0x230/+0x234/+0x27c/+0x22c` are cleared and `flags` is stored at
/// `+0x15`. When bit 0 of `flags` is clear, `+0x22c` counts the leading clear
/// bits (up to 3). Then the pad slots (2 or 1) are searched, with
/// `BELibrary_GetInterface(...)` and `GetPadIndexBySlot`, for the one equal to
/// `+0x22c`; the match goes to `+0x1cc`, and its low byte is copied to `+0x44`.
/// Returns `this`.
pub fn cfe_state_profile_load_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S2][0] = c.g[A3][0] & 0xff;
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        call_ptr(c, 0x001d_53b0, 0x0018_eccc)?;
        c.g[V1][0] = 0x0047_0000;
        c.g[V0][0] = 0x23;
        c.g[V1][0] = sx(0x0047_0000u32.wrapping_sub(0x4030));
        c.sw(off842(c.g[S1][0], 0xc), c.g[V0][0]);
        c.sw(off842(c.g[S1][0], 0x8), c.g[V1][0]);
        c.g[A0][0] = 0;
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1b08), c.g[S0][0]);
        c.g[V0][0] = c.g[S2][0] & 1;
        c.sw(off842(c.g[S1][0], 0x1cc), 0);
        c.sw(off842(c.g[S1][0], 0x230), 0);
        c.sw(off842(c.g[S1][0], 0x234), 0);
        c.sw(off842(c.g[S1][0], 0x27c), 0);
        c.sb(off842(c.g[S1][0], 0x15), c.g[S2][0]);
        let bit0 = c.g[V0][0] != 0;
        c.sw(off842(c.g[S1][0], 0x22c), 0);
        if !bit0 {
            c.cycles += 2;
            c.g[V0][0] = c.lw(off842(c.g[S1][0], 0x22c));
            'count: loop {
                c.cycles += 6;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                c.g[S2][0] = sx((c.g[S2][0] as u32) >> 1);
                c.g[V1][0] = ((c.g[A0][0] as i64) < 3) as u64;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                let end = c.g[V1][0] == 0;
                c.sw(off842(c.g[S1][0], 0x22c), c.g[V0][0]);
                if end {
                    break 'count;
                }
                c.cycles += 3;
                c.g[V0][0] = c.g[S2][0] & 1;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = c.lw(off842(c.g[S1][0], 0x22c));
                    if event_due(c) {
                        deliver_event_here(c, 0x0018_ed10);
                    }
                    continue 'count;
                }
                break 'count;
            }
        }
        c.cycles += 17;
        c.g[S2][0] = 1;
        call_ptr(c, 0x0014_dc80, 0x0018_ed3c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x0018_ed48)?;
        c.g[S0][0] = 0;
        c.g[S3][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0018_ed54)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0018_ed60)?;
        c.g[V1][0] = 0x0053_0000;
        c.g[A0][0] = 2;
        c.g[V1][0] = sx(0x0053_0000u32.wrapping_add(0x5bc8));
        c.g[V0][0] = c.lb(off842(c.g[V1][0], 0x49));
        c.g[V0][0] ^= 2;
        if c.g[V0][0] == 0 {
            c.g[S2][0] = c.g[A0][0];
        }
        'pads: loop {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
            let past = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S3][0];
            if !past {
                c.cycles += 5;
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0014_7318, 0x0018_ed8c)?;
                c.g[V1][0] = c.lw(off842(c.g[S1][0], 0x22c));
                if c.g[V0][0] != c.g[V1][0] {
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    if event_due(c) {
                        deliver_event_here(c, 0x0018_ed78);
                    }
                    continue 'pads;
                }
                c.cycles += 1;
                c.sw(off842(c.g[S1][0], 0x1cc), c.g[S0][0]);
            }
            break 'pads;
        }
        c.cycles += 10;
        c.g[V1][0] = c.lb(off842(c.g[S1][0], 0x1cc));
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x40);
        c.sb(off842(c.g[S1][0], 0x44), c.g[V1][0]);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateProfileLoad_Init(this)` (0x18edc8): create the profile screen
/// (`UIEngine_CreateScreen(owner, this, StrHash(name), 0)`, `owner = *(this+0x10)`),
/// keep it at `+0x40` and start its thread, set up the titles, then show the
/// title for the mode in `gp-0x1b08`: 1 new profile, 2 load, 3 save.
pub fn cfe_state_profile_load_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_sub(0x1db0));
        c.g[S0][0] = c.lw(off842(c.g[S1][0], 0x10));
        call_ptr(c, 0x0031_7670, 0x0018_edec)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0018_ee00)?;
        let no_screen = c.g[V0][0] == 0;
        c.sw(off842(c.g[S1][0], 0x40), c.g[V0][0]);
        if !no_screen {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0018_ee18)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0019_02c0, 0x0018_ee20)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1b08));
        c.g[V0][0] = 2;
        let mode2 = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 3) as u64;
        // 0: nothing shown (the `+1` block is skipped), 1: new, 2: load, 3: save.
        let mut show = 0;
        if mode2 {
            show = 2;
        } else {
            c.cycles += 2;
            let ge3 = c.g[V0][0] == 0;
            c.g[V0][0] = 1;
            if ge3 {
                c.cycles += 3;
                c.g[V0][0] = 3;
                let mode3 = c.g[V1][0] == c.g[V0][0];
                frame.restore(c, S0, 0x20);
                if mode3 {
                    show = 3;
                } else {
                    c.cycles += 2;
                    frame.restore(c, S1, 0x10);
                }
            } else {
                c.cycles += 2;
                let mode1 = c.g[V1][0] == c.g[V0][0];
                frame.restore(c, S0, 0x20);
                if mode1 {
                    show = 1;
                } else {
                    c.cycles += 2;
                    frame.restore(c, S1, 0x10);
                }
            }
        }
        if show != 0 {
            match show {
                1 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0019_0540, 0x0018_ee64)?;
                }
                2 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0019_0670, 0x0018_ee74)?;
                }
                _ => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0019_0768, 0x0018_ee84)?;
                }
            }
            frame.restore(c, S0, 0x20);
            c.cycles += 1;
            frame.restore(c, S1, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateProfileLoad_Render(this)` (0x18ee98): `UIState_Render(this)`.
pub fn cfe_state_profile_load_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0039_e6b8, 0x0018_eea8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, FAKE};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use crate::game::fe::festateoptions::{fe_install_fakes, FE_GATE, FE_GATE_MAGIC};
    use ssx3_ee as ee;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// This module's own gate word (the fakes below act only when it is set).
    const OC842_GATE: u32 = DATA + 0xf_0100;
    const OC842_MAGIC: u32 = 0x0842_beef;
    /// Value the `UIEngine_CreateScreen` fake returns in v0.
    const OC842_RET_CREATE: u32 = DATA + 0xf_0110;
    /// Value the `GetPadIndexBySlot` fake returns in v0.
    const OC842_RET_PAD: u32 = DATA + 0xf_0114;
    /// A real method that returns 0 (the owner has no next state).
    const OC842_RET_ZERO: u32 = 0x001a_8828;

    /// A gated fake of the callee at `$addr`: with the gate word set it logs the
    /// call (registers, cycles, target) and returns to `ra`; with the word at
    /// `$ret` set (nonzero address) v0 is then replaced by that word's value.
    /// With the gate off it runs the body the slot held before.
    macro_rules! oc842_fake {
        ($name:ident, $prev:ident, $addr:expr, $generated:path, $ret:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC842_GATE) != OC842_MAGIC {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $addr, ra);
                let slot: u32 = $ret;
                if slot != 0 {
                    let v = c.bus.read32(slot);
                    c.g[V0][0] = sx(v);
                }
            }
        };
    }

    oc842_fake!(
        oc842_fake_render,
        OC842_PREV_RENDER,
        0x0039_e6b8,
        ee::s13::f_0039e6b8__generated,
        0
    );
    oc842_fake!(
        oc842_fake_switch_close,
        OC842_PREV_SWITCH_CLOSE,
        0x0039_f4c0,
        ee::s13::f_0039f4c0__generated,
        0
    );
    oc842_fake!(
        oc842_fake_switch,
        OC842_PREV_SWITCH,
        0x0039_f400,
        ee::s13::f_0039f400__generated,
        0
    );
    oc842_fake!(
        oc842_fake_create_screen,
        OC842_PREV_CREATE,
        0x0039_7cc0,
        ee::s13::f_00397cc0__generated,
        OC842_RET_CREATE
    );
    oc842_fake!(
        oc842_fake_start_thread,
        OC842_PREV_START_THREAD,
        0x0039_c870,
        ee::s13::f_0039c870__generated,
        0
    );
    oc842_fake!(
        oc842_fake_get_be,
        OC842_PREV_GET_BE,
        0x0014_dc80,
        ee::s01::f_0014dc80__generated,
        0
    );
    oc842_fake!(
        oc842_fake_get_interface,
        OC842_PREV_GET_INTERFACE,
        0x0014_dd58,
        ee::s01::f_0014dd58__generated,
        0
    );
    oc842_fake!(
        oc842_fake_pad_index,
        OC842_PREV_PAD_INDEX,
        0x0014_7318,
        ee::s01::f_00147318__generated,
        OC842_RET_PAD
    );

    fn oc842_install() {
        fe_install_fakes();
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E6B8),
            oc842_fake_render,
            &OC842_PREV_RENDER,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039F4C0),
            oc842_fake_switch_close,
            &OC842_PREV_SWITCH_CLOSE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039F400),
            oc842_fake_switch,
            &OC842_PREV_SWITCH,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_00397CC0),
            oc842_fake_create_screen,
            &OC842_PREV_CREATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039C870),
            oc842_fake_start_thread,
            &OC842_PREV_START_THREAD,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc842_fake_get_be,
            &OC842_PREV_GET_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc842_fake_get_interface,
            &OC842_PREV_GET_INTERFACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00147318),
            oc842_fake_pad_index,
            &OC842_PREV_PAD_INDEX,
        );
    }

    /// Scrambled machine for one seed with both gates set and `pc` at `entry`.
    /// Each of the `calls` fake calls may unwind (one per seed at most).
    fn oc842_machine(c: &mut Cpu, seed: u32, entry: u32, salt: u64, calls: u32) -> Rng {
        oc842_install();
        let mut r = fakes::machine(c, seed, salt, calls);
        c.bus.write32(FE_GATE, FE_GATE_MAGIC);
        c.bus.write32(OC842_GATE, OC842_MAGIC);
        c.bus.write32(GAME_GP + 0xda0, r.next_u32());
        c.pc = entry;
        r
    }

    fn w842(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// A vtable slot pair at `vt + off`: halfword delta, method address.
    fn slot842(c: &mut Cpu, vt: u32, off: u32, delta: u16, method: u32) {
        c.bus.write16(vt + off, delta);
        c.bus.write32(vt + off + 4, method);
    }

    #[test]
    fn cfe_state_options_device_select_v33_matches() {
        check(
            ssx3_ee::s02::f_0018e478__generated,
            cfe_state_options_device_select_v33,
            80,
            &|c, seed| {
                let mut r = oc842_machine(c, seed, 0x0018_e478, 0x18e478, 3);
                let this = DATA + 0x2000;
                let owner = DATA + 0x3000;
                let engine = DATA + 0x4000;
                let vt = DATA + 0x5000;
                // The guard is already set on every fifth seed.
                w842(
                    c,
                    this + 0x1ac,
                    if seed % 5 == 0 { r.next_u32() | 1 } else { 0 },
                );
                w842(c, this + 0x10, owner);
                // The a2 flag inputs: each of the four words zero or not.
                for (i, off) in [0x1bcu32, 0x1e4, 0x1b4, 0x230].into_iter().enumerate() {
                    let v = if (seed >> (i + 1)) & 1 == 0 {
                        0
                    } else {
                        r.next_u32() | 1
                    };
                    w842(c, this + off, v);
                }
                if seed % 7 == 0 {
                    w842(c, this + 0x1bc, 1);
                }
                w842(c, owner, engine);
                w842(c, engine + 4, vt);
                slot842(
                    c,
                    vt,
                    0x28,
                    (r.next_u32() as i16 as i32 & !3) as u16,
                    if seed % 4 == 1 { OC842_RET_ZERO } else { FAKE },
                );
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn fe_screen_select_profile_ctor_matches() {
        check(
            ssx3_ee::s02::f_0018ea98__generated,
            fe_screen_select_profile_ctor,
            60,
            &|c, seed| {
                let mut r = oc842_machine(c, seed, 0x0018_ea98, 0x18ea98, 1);
                c.g[A0][0] = sx(DATA + (r.next_u32() & 0x7_fff0));
            },
        );
    }

    #[test]
    fn fe_select_profile_on_widget_action_matches() {
        check(
            ssx3_ee::s02::f_0018eb40__generated,
            fe_select_profile_on_widget_action,
            90,
            &|c, seed| {
                let mut r = oc842_machine(c, seed, 0x0018_eb40, 0x18eb40, 3);
                let this = DATA + 0x1000;
                let menu = DATA + 0x2000;
                let menu_vt = DATA + 0x3000;
                let owner = DATA + 0x4000;
                let engine = DATA + 0x5000;
                let engine_vt = DATA + 0x6000;
                let action = match seed % 9 {
                    0 => 2,
                    1 => 1,
                    2 => 5,
                    3 => 6,
                    4 => 0,
                    5 => 3,
                    6 => 4,
                    7 => 7,
                    _ => sx(0xffff_ffff),
                };
                w842(c, this + 0x10, owner);
                w842(c, owner, engine);
                w842(c, engine + 4, engine_vt);
                w842(c, menu + 8, menu_vt);
                w842(c, menu + 0x18, r.next_u32());
                // The widget's colour method (`+0x58/+0x5c`).
                slot842(
                    c,
                    menu_vt,
                    0x58,
                    (r.next_u32() as i16 as i32 & !3) as u16,
                    FAKE,
                );
                // The owner's action methods: actions 5 (`+0x20`) and 6 (`+0x28`);
                // one in three returns no state.
                let method = if seed % 3 == 0 {
                    OC842_RET_ZERO
                } else {
                    FAKE + 0x10
                };
                slot842(
                    c,
                    engine_vt,
                    0x20,
                    (r.next_u32() as i16 as i32 & !3) as u16,
                    method,
                );
                slot842(
                    c,
                    engine_vt,
                    0x28,
                    (r.next_u32() as i16 as i32 & !3) as u16,
                    method,
                );
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 11 == 10 { 0 } else { sx(menu) };
                c.g[A2][0] = action;
            },
        );
    }

    #[test]
    fn cfe_state_profile_load_ctor_matches() {
        check(
            ssx3_ee::s02::f_0018eca0__generated,
            cfe_state_profile_load_ctor,
            120,
            &|c, seed| {
                let mut r = oc842_machine(c, seed, 0x0018_eca0, 0x18eca0, 7);
                // `lb` of the pad-mode byte (0x535c11): 2 selects two pad slots.
                c.bus.write8(0x0053_5c11, if seed % 2 == 0 { 2 } else { 0 });
                // The pad index the fake returns: equal to the `+0x22c` count
                // (0..3) for some seeds.
                w842(c, OC842_RET_PAD, seed / 2 % 5);
                c.g[A0][0] = sx(DATA + (r.next_u32() & 0x7_fff0));
            },
        );
    }

    #[test]
    fn cfe_state_profile_load_init_matches() {
        use crate::game::fe::fememcardcallbacks::tests::hash_screen;
        check(
            ssx3_ee::s02::f_0018edc8__generated,
            cfe_state_profile_load_init,
            60,
            &|c, seed| {
                let _ = oc842_machine(c, seed, 0x0018_edc8, 0x18edc8, 14);
                let this = DATA + 0x1000;
                let owner = DATA + 0x2000;
                let screen = DATA + 0x3000;
                let node = DATA + 0x4000;
                let vt = DATA + 0x5000;
                w842(c, this + 0x10, owner);
                w842(c, OC842_RET_CREATE, if seed % 7 == 0 { 0 } else { screen });
                // One node answers every hash lookup (the names are zeroed
                // game strings); its vtable answers every slot with a fake.
                hash_screen(c, screen, node, 0x004a_1458);
                w842(c, node + 8, vt);
                for off in (0..0x100).step_by(8) {
                    slot842(c, vt, off, 0, FAKE);
                }
                // The title mode.
                w842(
                    c,
                    GAME_GP.wrapping_sub(0x1b08),
                    [1, 2, 3, 0, 4, 2, 1, 3][seed as usize % 8],
                );
                // (Registers last: the fixture ran guest code.)
                c.g[A0][0] = sx(this);
                c.pc = 0x0018_edc8;
            },
        );
    }

    #[test]
    fn cfe_state_profile_load_render_matches() {
        check(
            ssx3_ee::s02::f_0018ee98__generated,
            cfe_state_profile_load_render,
            40,
            &|c, seed| {
                let mut r = oc842_machine(c, seed, 0x0018_ee98, 0x18ee98, 1);
                c.g[A0][0] = sx(DATA + (r.next_u32() & 0x7_fff0));
            },
        );
    }
}
