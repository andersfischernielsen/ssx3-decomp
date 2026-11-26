//! Staging module oc837 (wave wave249): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc837`.

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
        0x0018_a298,
        c_fe_state_options_sound_init,
        "cFEStateOptionsSound_Init",
    ),
    (
        0x0018_a3d8,
        c_fe_state_options_sound_on_child_event,
        "cFEStateOptionsSound_OnChildEvent",
    ),
    (
        0x0018_a870,
        c_fe_state_options_sound_on_activate,
        "cFEStateOptionsSound_OnActivate",
    ),
    (
        0x0018_bef8,
        c_fe_state_options_sound_set_speaker_mode,
        "cFEStateOptionsSound_SetSpeakerMode",
    ),
    (
        0x0018_c198,
        c_fe_state_options_hud_on_activate,
        "cFEStateOptionsHUD_OnActivate",
    ),
    (
        0x0018_c2a0,
        c_fe_state_options_hud_on_widget_action,
        "cFEStateOptionsHUD_OnWidgetAction",
    ),
];

/// `cFEStateOptionsSound_OnActivate(self)` (0x18a870): hand the activation to
/// the base state (`UIState_OnActivate`).
pub fn c_fe_state_options_sound_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0039_e4c0, 0x0018_a880)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateOptionsHUD_OnActivate(self)` (0x18c198): hand the activation to
/// the base state (`UIState_OnActivate`).
pub fn c_fe_state_options_hud_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0039_e4c0, 0x0018_c1a8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// The bodies below keep the generated block structure (labelled blocks and the
// struct-copy loops, whose unaligned halves are never taken at the fixed
// config address 0x535610).

/// `cFEStateOptionsSound_Init(self)` (0x18a298): create the advanced sound settings screen, start it, refresh the controls and show or hide the on/off groups and the extra item from the surround flag.
pub fn c_fe_state_options_sound_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_18a2ec: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S1][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[A0][0] = 0x460000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2040));
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x0018a2c0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x0018a2d4)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_18a2ec;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x0018a2ec)?;
                break 'b_18a2ec;
            }
        }
        'b_18a338: {
            c.cycles += 14;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0018b380, 0x0018a2f4)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
            c.g[A0][0] = 0x4a0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1580));
            c.g[V0][0] = c.g[V0][0] ^ 0x1;
            c.g[S2][0] = (c.g[V0][0] < 0x1u64) as u64;
            call_ptr(c, 0x00317670, 0x0018a30c)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x0018a318)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S2][0];
            if t {
                break 'b_18a338;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0018a338)?;
                break 'b_18a338;
            }
        }
        'b_18a370: {
            c.cycles += 9;
            c.g[A0][0] = 0x4a0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1588));
            call_ptr(c, 0x00317670, 0x0018a344)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x0018a350)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S2][0] ^ 0x1;
            if t {
                break 'b_18a370;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0018a370)?;
                break 'b_18a370;
            }
        }
        'b_18a3c0: {
            c.cycles += 9;
            c.g[A0][0] = 0x4a0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1528));
            call_ptr(c, 0x00317670, 0x0018a37c)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x0018a388)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = c.g[S2][0];
            if t {
                break 'b_18a3c0;
            } else {
                c.cycles += 11;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0018a3a8)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.g[S2][0] ^ 0x1;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0018a3c0)?;
                break 'b_18a3c0;
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `cFEStateOptionsSound_OnChildEvent(self, popup, event)` (0x18a3d8): on the reset-options popup answer, restore the default sound options (speaker mode, volumes, DJ speech, arcade SFX).
pub fn c_fe_state_options_sound_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        'b_18a858: {
            'b_18a854: {
                c.cycles += 17;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x2e0));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x2d0));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                c.g[S0][0] = c.g[A2][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x2a0));
                c.g[S1][0] = c.g[A1][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x2b0));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x290), c.g[RA][0]);
                c.g[S3][0] = c.g[A0][0];
                call_ptr(c, 0x0014dc80, 0x0018a400)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x4;
                call_ptr(c, 0x0014dd58, 0x0018a40c)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[V0][0] = 0x15;
                let t = c.g[S0][0] == c.g[V0][0];
                c.g[V0][0] = 0x16;
                if t {
                    break 'b_18a854;
                } else {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2d0));
                    if t {
                        break 'b_18a858;
                    } else {
                        c.cycles += 6;
                        c.g[A0][0] = 0x460000;
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x22e0));
                        call_ptr(c, 0x00317670, 0x0018a434)?;
                        let t = c.g[S0][0] != c.g[V0][0];
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2d0));
                        if t {
                            break 'b_18a858;
                        } else {
                            'b_18a850: {
                                c.cycles += 8;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = 0x2;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xb8));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xbc));
                                let t = c.g[V0][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x0018a454)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_18a850;
                                } else {
                                    'b_18a524: {
                                        'b_18a4f4: {
                                            c.cycles += 16;
                                            c.g[A0][0] = c.g[S2][0];
                                            c.g[S0][0] = 0x530000;
                                            call_ptr(c, 0x0014f458, 0x0018a468)?;
                                            c.g[S1][0] = c.g[S0][0];
                                            c.g[A1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x5610));
                                            c.g[A0][0] = c.g[S3][0];
                                            c.g[S0][0] =
                                                sx((c.g[S0][0] as u32).wrapping_add(0x5610));
                                            c.g[A1][0] = sx((c.g[A1][0] as u32) >> 12);
                                            c.g[A1][0] = c.g[A1][0] & 0x3;
                                            call_ptr(c, 0x0018bef8, 0x0018a484)?;
                                            call_ptr(c, 0x0028b180, 0x0018a48c)?;
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[V0][0] = c.g[S0][0] & 0x7;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V1][0] = c.g[SP][0];
                                            if t {
                                                break 'b_18a4f4;
                                            } else {
                                                c.cycles += 1;
                                                c.g[V0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x280));
                                                'l_18a4a0: loop {
                                                    c.cycles += 19;
                                                    c.ldl(5, (c.g[S0][0] as u32).wrapping_add(0x7));
                                                    c.ldr(5, c.g[S0][0] as u32);
                                                    c.ldl(6, (c.g[S0][0] as u32).wrapping_add(0xf));
                                                    c.ldr(6, (c.g[S0][0] as u32).wrapping_add(0x8));
                                                    c.ldl(
                                                        7,
                                                        (c.g[S0][0] as u32).wrapping_add(0x17),
                                                    );
                                                    c.ldr(
                                                        7,
                                                        (c.g[S0][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.ldl(
                                                        8,
                                                        (c.g[S0][0] as u32).wrapping_add(0x1f),
                                                    );
                                                    c.ldr(
                                                        8,
                                                        (c.g[S0][0] as u32).wrapping_add(0x18),
                                                    );
                                                    c.sdl(5, (c.g[V1][0] as u32).wrapping_add(0x7));
                                                    c.sdr(5, c.g[V1][0] as u32);
                                                    c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                                                    c.sdr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                                                    c.sdl(
                                                        7,
                                                        (c.g[V1][0] as u32).wrapping_add(0x17),
                                                    );
                                                    c.sdr(
                                                        7,
                                                        (c.g[V1][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.sdl(
                                                        8,
                                                        (c.g[V1][0] as u32).wrapping_add(0x1f),
                                                    );
                                                    c.sdr(
                                                        8,
                                                        (c.g[V1][0] as u32).wrapping_add(0x18),
                                                    );
                                                    c.g[S0][0] =
                                                        sx((c.g[S0][0] as u32).wrapping_add(0x20));
                                                    let t = c.g[S0][0] != c.g[V0][0];
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x0018a4a0);
                                                        }
                                                        continue 'l_18a4a0;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;

                                                        if t {
                                                            break 'b_18a524;
                                                        } else {
                                                            break 'b_18a4f4;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x280));
                                        'l_18a4f8: loop {
                                            c.cycles += 11;
                                            c.g[T1][0] = c.ld(c.g[S0][0] as u32);
                                            c.g[A1][0] =
                                                c.ld((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[A2][0] =
                                                c.ld((c.g[S0][0] as u32).wrapping_add(0x10));
                                            c.g[A3][0] =
                                                c.ld((c.g[S0][0] as u32).wrapping_add(0x18));
                                            c.sd(c.g[V1][0] as u32, c.g[T1][0]);
                                            c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                                            c.sd(
                                                (c.g[V1][0] as u32).wrapping_add(0x10),
                                                c.g[A2][0],
                                            );
                                            c.sd(
                                                (c.g[V1][0] as u32).wrapping_add(0x18),
                                                c.g[A3][0],
                                            );
                                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20));
                                            let t = c.g[S0][0] != c.g[V0][0];
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x0018a4f8);
                                                }
                                                continue 'l_18a4f8;
                                            } else {
                                                break 'b_18a524;
                                            }
                                        }
                                    }
                                    'b_18a5e4: {
                                        'b_18a5b4: {
                                            c.cycles += 14;
                                            c.ldl(8, (c.g[S0][0] as u32).wrapping_add(0x7));
                                            c.ldr(8, c.g[S0][0] as u32);
                                            c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
                                            c.sdr(8, c.g[V1][0] as u32);
                                            c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                                            c.g[A1][0] = c.g[A1][0] & 0xf;
                                            call_ptr(c, 0x00287410, 0x0018a540)?;
                                            call_ptr(c, 0x0028b180, 0x0018a548)?;
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[A1][0] =
                                                sx((c.g[S1][0] as u32).wrapping_add(0x5610));
                                            c.g[V0][0] = c.g[A1][0] & 0x7;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V1][0] = c.g[SP][0];
                                            if t {
                                                break 'b_18a5b4;
                                            } else {
                                                c.cycles += 1;
                                                c.g[V0][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x280));
                                                'l_18a560: loop {
                                                    c.cycles += 19;
                                                    c.ldl(6, (c.g[A1][0] as u32).wrapping_add(0x7));
                                                    c.ldr(6, c.g[A1][0] as u32);
                                                    c.ldl(7, (c.g[A1][0] as u32).wrapping_add(0xf));
                                                    c.ldr(7, (c.g[A1][0] as u32).wrapping_add(0x8));
                                                    c.ldl(
                                                        8,
                                                        (c.g[A1][0] as u32).wrapping_add(0x17),
                                                    );
                                                    c.ldr(
                                                        8,
                                                        (c.g[A1][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.ldl(
                                                        9,
                                                        (c.g[A1][0] as u32).wrapping_add(0x1f),
                                                    );
                                                    c.ldr(
                                                        9,
                                                        (c.g[A1][0] as u32).wrapping_add(0x18),
                                                    );
                                                    c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0x7));
                                                    c.sdr(6, c.g[V1][0] as u32);
                                                    c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0xf));
                                                    c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x8));
                                                    c.sdl(
                                                        8,
                                                        (c.g[V1][0] as u32).wrapping_add(0x17),
                                                    );
                                                    c.sdr(
                                                        8,
                                                        (c.g[V1][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.sdl(
                                                        9,
                                                        (c.g[V1][0] as u32).wrapping_add(0x1f),
                                                    );
                                                    c.sdr(
                                                        9,
                                                        (c.g[V1][0] as u32).wrapping_add(0x18),
                                                    );
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32).wrapping_add(0x20));
                                                    let t = c.g[A1][0] != c.g[V0][0];
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x0018a560);
                                                        }
                                                        continue 'l_18a560;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;

                                                        if t {
                                                            break 'b_18a5e4;
                                                        } else {
                                                            break 'b_18a5b4;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                                        'l_18a5b8: loop {
                                            c.cycles += 11;
                                            c.g[A2][0] = c.ld(c.g[A1][0] as u32);
                                            c.g[A3][0] =
                                                c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                                            c.g[T0][0] =
                                                c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                                            c.g[T1][0] =
                                                c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                                            c.sd(c.g[V1][0] as u32, c.g[A2][0]);
                                            c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                                            c.sd(
                                                (c.g[V1][0] as u32).wrapping_add(0x10),
                                                c.g[T0][0],
                                            );
                                            c.sd(
                                                (c.g[V1][0] as u32).wrapping_add(0x18),
                                                c.g[T1][0],
                                            );
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                                            let t = c.g[A1][0] != c.g[V0][0];
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x0018a5b8);
                                                }
                                                continue 'l_18a5b8;
                                            } else {
                                                break 'b_18a5e4;
                                            }
                                        }
                                    }
                                    'b_18a6a8: {
                                        'b_18a678: {
                                            c.cycles += 15;
                                            c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
                                            c.ldr(2, c.g[A1][0] as u32);
                                            c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                                            c.sdr(2, c.g[V1][0] as u32);
                                            c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                                            c.g[A1][0] = sx((c.g[A1][0] as u32) >> 8);
                                            c.g[A1][0] = c.g[A1][0] & 0xf;
                                            call_ptr(c, 0x00287488, 0x0018a604)?;
                                            call_ptr(c, 0x0028b180, 0x0018a60c)?;
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[A1][0] =
                                                sx((c.g[S1][0] as u32).wrapping_add(0x5610));
                                            c.g[V0][0] = c.g[A1][0] & 0x7;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V1][0] = c.g[SP][0];
                                            if t {
                                                break 'b_18a678;
                                            } else {
                                                c.cycles += 1;
                                                c.g[V0][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x280));
                                                'l_18a624: loop {
                                                    c.cycles += 19;
                                                    c.ldl(9, (c.g[A1][0] as u32).wrapping_add(0x7));
                                                    c.ldr(9, c.g[A1][0] as u32);
                                                    c.ldl(6, (c.g[A1][0] as u32).wrapping_add(0xf));
                                                    c.ldr(6, (c.g[A1][0] as u32).wrapping_add(0x8));
                                                    c.ldl(
                                                        7,
                                                        (c.g[A1][0] as u32).wrapping_add(0x17),
                                                    );
                                                    c.ldr(
                                                        7,
                                                        (c.g[A1][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.ldl(
                                                        8,
                                                        (c.g[A1][0] as u32).wrapping_add(0x1f),
                                                    );
                                                    c.ldr(
                                                        8,
                                                        (c.g[A1][0] as u32).wrapping_add(0x18),
                                                    );
                                                    c.sdl(9, (c.g[V1][0] as u32).wrapping_add(0x7));
                                                    c.sdr(9, c.g[V1][0] as u32);
                                                    c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                                                    c.sdr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                                                    c.sdl(
                                                        7,
                                                        (c.g[V1][0] as u32).wrapping_add(0x17),
                                                    );
                                                    c.sdr(
                                                        7,
                                                        (c.g[V1][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.sdl(
                                                        8,
                                                        (c.g[V1][0] as u32).wrapping_add(0x1f),
                                                    );
                                                    c.sdr(
                                                        8,
                                                        (c.g[V1][0] as u32).wrapping_add(0x18),
                                                    );
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32).wrapping_add(0x20));
                                                    let t = c.g[A1][0] != c.g[V0][0];
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x0018a624);
                                                        }
                                                        continue 'l_18a624;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;

                                                        if t {
                                                            break 'b_18a6a8;
                                                        } else {
                                                            break 'b_18a678;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                                        'l_18a67c: loop {
                                            c.cycles += 11;
                                            c.g[T1][0] = c.ld(c.g[A1][0] as u32);
                                            c.g[A2][0] =
                                                c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                                            c.g[A3][0] =
                                                c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                                            c.g[T0][0] =
                                                c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                                            c.sd(c.g[V1][0] as u32, c.g[T1][0]);
                                            c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                                            c.sd(
                                                (c.g[V1][0] as u32).wrapping_add(0x10),
                                                c.g[A3][0],
                                            );
                                            c.sd(
                                                (c.g[V1][0] as u32).wrapping_add(0x18),
                                                c.g[T0][0],
                                            );
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                                            let t = c.g[A1][0] != c.g[V0][0];
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x0018a67c);
                                                }
                                                continue 'l_18a67c;
                                            } else {
                                                break 'b_18a6a8;
                                            }
                                        }
                                    }
                                    'b_18a768: {
                                        'b_18a738: {
                                            c.cycles += 14;
                                            c.ldl(9, (c.g[A1][0] as u32).wrapping_add(0x7));
                                            c.ldr(9, c.g[A1][0] as u32);
                                            c.sdl(9, (c.g[V1][0] as u32).wrapping_add(0x7));
                                            c.sdr(9, c.g[V1][0] as u32);
                                            c.g[A1][0] = c.lbu(c.g[SP][0] as u32);
                                            c.g[A1][0] = c.g[A1][0] >> 4;
                                            call_ptr(c, 0x00287520, 0x0018a6c4)?;
                                            call_ptr(c, 0x0028b180, 0x0018a6cc)?;
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[A1][0] =
                                                sx((c.g[S1][0] as u32).wrapping_add(0x5610));
                                            c.g[V0][0] = c.g[A1][0] & 0x7;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V1][0] = c.g[SP][0];
                                            if t {
                                                break 'b_18a738;
                                            } else {
                                                c.cycles += 1;
                                                c.g[V0][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x280));
                                                'l_18a6e4: loop {
                                                    c.cycles += 19;
                                                    c.ldl(8, (c.g[A1][0] as u32).wrapping_add(0x7));
                                                    c.ldr(8, c.g[A1][0] as u32);
                                                    c.ldl(9, (c.g[A1][0] as u32).wrapping_add(0xf));
                                                    c.ldr(9, (c.g[A1][0] as u32).wrapping_add(0x8));
                                                    c.ldl(
                                                        6,
                                                        (c.g[A1][0] as u32).wrapping_add(0x17),
                                                    );
                                                    c.ldr(
                                                        6,
                                                        (c.g[A1][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.ldl(
                                                        7,
                                                        (c.g[A1][0] as u32).wrapping_add(0x1f),
                                                    );
                                                    c.ldr(
                                                        7,
                                                        (c.g[A1][0] as u32).wrapping_add(0x18),
                                                    );
                                                    c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
                                                    c.sdr(8, c.g[V1][0] as u32);
                                                    c.sdl(9, (c.g[V1][0] as u32).wrapping_add(0xf));
                                                    c.sdr(9, (c.g[V1][0] as u32).wrapping_add(0x8));
                                                    c.sdl(
                                                        6,
                                                        (c.g[V1][0] as u32).wrapping_add(0x17),
                                                    );
                                                    c.sdr(
                                                        6,
                                                        (c.g[V1][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.sdl(
                                                        7,
                                                        (c.g[V1][0] as u32).wrapping_add(0x1f),
                                                    );
                                                    c.sdr(
                                                        7,
                                                        (c.g[V1][0] as u32).wrapping_add(0x18),
                                                    );
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32).wrapping_add(0x20));
                                                    let t = c.g[A1][0] != c.g[V0][0];
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x0018a6e4);
                                                        }
                                                        continue 'l_18a6e4;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;

                                                        if t {
                                                            break 'b_18a768;
                                                        } else {
                                                            break 'b_18a738;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                                        'l_18a73c: loop {
                                            c.cycles += 11;
                                            c.g[T0][0] = c.ld(c.g[A1][0] as u32);
                                            c.g[T1][0] =
                                                c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                                            c.g[A2][0] =
                                                c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                                            c.g[A3][0] =
                                                c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                                            c.sd(c.g[V1][0] as u32, c.g[T0][0]);
                                            c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                                            c.sd(
                                                (c.g[V1][0] as u32).wrapping_add(0x10),
                                                c.g[A2][0],
                                            );
                                            c.sd(
                                                (c.g[V1][0] as u32).wrapping_add(0x18),
                                                c.g[A3][0],
                                            );
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                                            let t = c.g[A1][0] != c.g[V0][0];
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x0018a73c);
                                                }
                                                continue 'l_18a73c;
                                            } else {
                                                break 'b_18a768;
                                            }
                                        }
                                    }
                                    'b_18a828: {
                                        'b_18a7f8: {
                                            c.cycles += 14;
                                            c.ldl(8, (c.g[A1][0] as u32).wrapping_add(0x7));
                                            c.ldr(8, c.g[A1][0] as u32);
                                            c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
                                            c.sdr(8, c.g[V1][0] as u32);
                                            c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                                            c.g[A1][0] =
                                                sx(((c.g[A1][0] as u32) as i32 >> 17) as u32);
                                            c.g[A1][0] = c.g[A1][0] & 0x1;
                                            call_ptr(c, 0x00287558, 0x0018a788)?;
                                            c.g[S0][0] =
                                                sx((c.g[S1][0] as u32).wrapping_add(0x5610));
                                            call_ptr(c, 0x0028b180, 0x0018a790)?;
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[V0][0] = c.g[S0][0] & 0x7;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V1][0] = c.g[SP][0];
                                            if t {
                                                break 'b_18a7f8;
                                            } else {
                                                c.cycles += 1;
                                                c.g[V0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x280));
                                                'l_18a7a4: loop {
                                                    c.cycles += 19;
                                                    c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x7));
                                                    c.ldr(7, c.g[S0][0] as u32);
                                                    c.ldl(8, (c.g[S0][0] as u32).wrapping_add(0xf));
                                                    c.ldr(8, (c.g[S0][0] as u32).wrapping_add(0x8));
                                                    c.ldl(
                                                        9,
                                                        (c.g[S0][0] as u32).wrapping_add(0x17),
                                                    );
                                                    c.ldr(
                                                        9,
                                                        (c.g[S0][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.ldl(
                                                        5,
                                                        (c.g[S0][0] as u32).wrapping_add(0x1f),
                                                    );
                                                    c.ldr(
                                                        5,
                                                        (c.g[S0][0] as u32).wrapping_add(0x18),
                                                    );
                                                    c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                                                    c.sdr(7, c.g[V1][0] as u32);
                                                    c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0xf));
                                                    c.sdr(8, (c.g[V1][0] as u32).wrapping_add(0x8));
                                                    c.sdl(
                                                        9,
                                                        (c.g[V1][0] as u32).wrapping_add(0x17),
                                                    );
                                                    c.sdr(
                                                        9,
                                                        (c.g[V1][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.sdl(
                                                        5,
                                                        (c.g[V1][0] as u32).wrapping_add(0x1f),
                                                    );
                                                    c.sdr(
                                                        5,
                                                        (c.g[V1][0] as u32).wrapping_add(0x18),
                                                    );
                                                    c.g[S0][0] =
                                                        sx((c.g[S0][0] as u32).wrapping_add(0x20));
                                                    let t = c.g[S0][0] != c.g[V0][0];
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x0018a7a4);
                                                        }
                                                        continue 'l_18a7a4;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;

                                                        if t {
                                                            break 'b_18a828;
                                                        } else {
                                                            break 'b_18a7f8;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x280));
                                        'l_18a7fc: loop {
                                            c.cycles += 11;
                                            c.g[A2][0] = c.ld(c.g[S0][0] as u32);
                                            c.g[A3][0] =
                                                c.ld((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[T0][0] =
                                                c.ld((c.g[S0][0] as u32).wrapping_add(0x10));
                                            c.g[T1][0] =
                                                c.ld((c.g[S0][0] as u32).wrapping_add(0x18));
                                            c.sd(c.g[V1][0] as u32, c.g[A2][0]);
                                            c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                                            c.sd(
                                                (c.g[V1][0] as u32).wrapping_add(0x10),
                                                c.g[T0][0],
                                            );
                                            c.sd(
                                                (c.g[V1][0] as u32).wrapping_add(0x18),
                                                c.g[T1][0],
                                            );
                                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20));
                                            let t = c.g[S0][0] != c.g[V0][0];
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x0018a7fc);
                                                }
                                                continue 'l_18a7fc;
                                            } else {
                                                break 'b_18a828;
                                            }
                                        }
                                    }
                                    c.cycles += 10;
                                    c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x7));
                                    c.ldr(2, c.g[S0][0] as u32);
                                    c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                                    c.sdr(2, c.g[V1][0] as u32);
                                    c.g[A1][0] = c.lh((c.g[SP][0] as u32).wrapping_add(0x2));
                                    c.g[A1][0] = c.g[A1][0] & 0x1;
                                    call_ptr(c, 0x002875d0, 0x0018a844)?;
                                    c.g[A0][0] = c.g[S3][0];
                                    call_ptr(c, 0x0018b380, 0x0018a84c)?;
                                    c.g[V0][0] = 0x1;
                                    break 'b_18a850;
                                }
                            }
                            c.cycles += 1;
                            c.sw((c.g[GP][0] as u32).wrapping_sub(0x1c1c), c.g[V0][0]);
                            break 'b_18a854;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2d0));
            break 'b_18a858;
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x2c0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x2b0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x2a0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x290));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2e0));
        c.pc = t;
        Ok(())
    })
}

/// `cFEStateOptionsSound_SetSpeakerMode(self, mode)` (0x18bef8): map the speaker mode (0 and default 1, 1 -> 2, 2 -> 0, 3 -> 3) to the audio mode; when the music system accepts it, restart the audio engine with that mode and restore the volumes and flags.
pub fn c_fe_state_options_sound_set_speaker_mode(c: &mut Cpu) {
    guest(c, |c| {
        'b_18bf60: {
            'b_18bf5c: {
                'b_18bf54: {
                    'b_18bf4c: {
                        'b_18bf44: {
                            c.cycles += 11;
                            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                            c.g[V0][0] = 0x1;
                            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                            c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x60));
                            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x58));
                            let t = c.g[A1][0] == c.g[V0][0];
                            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                            if t {
                                break 'b_18bf44;
                            } else {
                                c.cycles += 2;
                                let t = c.g[A1][0] == 0u64;
                                c.g[V0][0] = 0x2;
                                if t {
                                    break 'b_18bf5c;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[A1][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x3;
                                    if t {
                                        break 'b_18bf4c;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[A1][0] == c.g[V0][0];
                                        c.g[S3][0] = 0x1;
                                        if t {
                                            break 'b_18bf54;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;

                                            if t {
                                                break 'b_18bf60;
                                            } else {
                                                break 'b_18bf44;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = true;
                        c.g[S3][0] = 0x2;
                        if t {
                            break 'b_18bf60;
                        } else {
                            break 'b_18bf4c;
                        }
                    }
                    c.cycles += 2;
                    let t = true;
                    c.g[S3][0] = 0u64;
                    if t {
                        break 'b_18bf60;
                    } else {
                        break 'b_18bf54;
                    }
                }
                c.cycles += 2;
                let t = true;
                c.g[S3][0] = 0x3;
                if t {
                    break 'b_18bf60;
                } else {
                    break 'b_18bf5c;
                }
            }
            c.cycles += 1;
            c.g[S3][0] = 0x1;
            break 'b_18bf60;
        }
        'b_18c0c0: {
            c.cycles += 14;
            call_ptr(c, 0x0028b180, 0x0018bf68)?;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x118));
            c.g[A1][0] = c.g[S3][0];
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0018bf8c)?;
            c.g[V1][0] = 0x1;
            let t = c.g[V0][0] != c.g[V1][0];
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            if t {
                break 'b_18c0c0;
            } else {
                c.cycles += 74;
                call_ptr(c, 0x0028b180, 0x0018bfa0)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x00287670, 0x0018bfa8)?;
                c.mtc1(2, 20);
                c.cvt_s_w(20, 20);
                call_ptr(c, 0x0028b180, 0x0018bfb8)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x002876a0, 0x0018bfc0)?;
                c.mtc1(2, 21);
                c.cvt_s_w(21, 21);
                call_ptr(c, 0x0028b180, 0x0018bfd0)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x002876d0, 0x0018bfd8)?;
                c.mtc1(2, 22);
                c.cvt_s_w(22, 22);
                call_ptr(c, 0x0028b180, 0x0018bfe8)?;
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x62b8));
                call_ptr(c, 0x0028b180, 0x0018bff0)?;
                c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x62b4));
                call_ptr(c, 0x0028b180, 0x0018bff8)?;
                c.g[S2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x534));
                call_ptr(c, 0x0028b180, 0x0018c000)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x00286200, 0x0018c008)?;
                call_ptr(c, 0x00284c28, 0x0018c010)?;
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x00284bb0, 0x0018c018)?;
                call_ptr(c, 0x0028b180, 0x0018c020)?;
                c.cvt_w_s(0, 20);
                c.mfc1(5, 0);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x00287410, 0x0018c030)?;
                call_ptr(c, 0x0028b180, 0x0018c038)?;
                c.cvt_w_s(0, 21);
                c.mfc1(5, 0);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x00287488, 0x0018c048)?;
                call_ptr(c, 0x0028b180, 0x0018c050)?;
                c.cvt_w_s(0, 22);
                c.mfc1(5, 0);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x00287520, 0x0018c060)?;
                call_ptr(c, 0x0028b180, 0x0018c068)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00287558, 0x0018c074)?;
                call_ptr(c, 0x0028b180, 0x0018c07c)?;
                c.g[A1][0] = c.g[S1][0];
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x002875d0, 0x0018c088)?;
                call_ptr(c, 0x0028b180, 0x0018c090)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x00285fb0, 0x0018c098)?;
                call_ptr(c, 0x0028b180, 0x0018c0a0)?;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5558));
                c.g[A1][0] = c.g[S2][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x118));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                let t = c.g[A2][0] as u32;
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0018c0bc)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                break 'b_18c0c0;
            }
        }
        c.cycles += 9;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x58));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}

/// `cFEStateOptionsHUD_OnWidgetAction(self, widget, action)` (0x18c2a0): handle the HUD options widget actions: open the reset popup (7), go to the previous state (6), show the widget (1), hide it for the three listed names (2).
pub fn c_fe_state_options_hud_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        'b_18c460: {
            'b_18c45c: {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = c.g[A2][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S1][0] = c.g[A1][0];
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                let t = c.g[S1][0] == 0u64;
                c.g[S2][0] = c.g[A0][0];
                if t {
                    break 'b_18c45c;
                } else {
                    'b_18c3fc: {
                        c.cycles += 9;
                        call_ptr(c, 0x0014dc80, 0x0018c2cc)?;
                        c.g[A1][0] = 0x4;
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x0014dd58, 0x0018c2d8)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V0][0] = 0x2;
                        let t = c.g[S0][0] == c.g[V0][0];
                        c.g[V0][0] = (c.g[S0][0] < 0x3u64) as u64;
                        if t {
                            break 'b_18c3fc;
                        } else {
                            'b_18c3bc: {
                                'b_18c300: {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_18c300;
                                    } else {
                                        c.cycles += 2;
                                        if c.g[S0][0] == c.g[V0][0] {
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                            break 'b_18c3bc;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                            if t {
                                                break 'b_18c460;
                                            } else {
                                                break 'b_18c300;
                                            }
                                        }
                                    }
                                }
                                'b_18c368: {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x6;
                                    let t = c.g[S0][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x7;
                                    if t {
                                        break 'b_18c368;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[S0][0] != c.g[V0][0];
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        if t {
                                            break 'b_18c460;
                                        } else {
                                            c.cycles += 21;
                                            c.g[A1][0] = 0x460000;
                                            c.g[A0][0] = 0x4c;
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_sub(0x2298));
                                            c.g[A2][0] = 0x100;
                                            c.g[A3][0] = 0u64;
                                            call_ptr(c, 0x00317d70, 0x0018c32c)?;
                                            c.g[S0][0] = c.g[V0][0];
                                            c.g[A1][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A2][0] = c.g[S2][0];
                                            call_ptr(c, 0x0039e318, 0x0018c340)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                            c.g[V0][0] = 0x470000;
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_sub(0x4c30));
                                            c.g[A1][0] = c.g[S0][0];
                                            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                            c.sb((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                                            call_ptr(c, 0x0039f290, 0x0018c360)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                            if t {
                                                break 'b_18c460;
                                            } else {
                                                break 'b_18c368;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 16;
                                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x0018c37c)?;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                c.g[A1][0] = c.g[S2][0];
                                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                                let t = c.g[V0][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x0018c3a0)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A1][0] = c.g[V0][0];
                                if t {
                                    break 'b_18c45c;
                                } else {
                                    c.cycles += 5;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                    call_ptr(c, 0x0039f400, 0x0018c3b4)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                    if t {
                                        break 'b_18c460;
                                    } else {
                                        break 'b_18c3bc;
                                    }
                                }
                            }
                            c.cycles += 16;
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0018c3d0)?;
                            c.sw((c.g[GP][0] as u32).wrapping_sub(0x1c18), c.g[S0][0]);
                            c.g[A1][0] = 0x530000;
                            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x5610));
                            c.g[V0][0] = 0x3fff0000;
                            c.g[V1][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x18));
                            c.g[V0][0] = c.g[V0][0] | 0xffff;
                            c.g[A0][0] = c.g[A0][0] & c.g[V0][0];
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 30);
                            c.g[A0][0] = c.g[A0][0] | c.g[V1][0];
                            let t = true;
                            c.sw((c.g[A1][0] as u32).wrapping_add(0x5610), c.g[A0][0]);
                            if t {
                                break 'b_18c45c;
                            } else {
                                break 'b_18c3fc;
                            }
                        }
                    }
                    'b_18c448: {
                        c.cycles += 6;
                        c.g[A0][0] = 0x4a0000;
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14f8));
                        call_ptr(c, 0x00317670, 0x0018c40c)?;
                        if c.g[S0][0] == c.g[V0][0] {
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            break 'b_18c448;
                        } else {
                            c.cycles += 6;
                            c.g[A0][0] = 0x4a0000;
                            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1500));
                            call_ptr(c, 0x00317670, 0x0018c424)?;
                            if c.g[S0][0] == c.g[V0][0] {
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                break 'b_18c448;
                            } else {
                                c.cycles += 6;
                                c.g[A0][0] = 0x4a0000;
                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1508));
                                call_ptr(c, 0x00317670, 0x0018c43c)?;
                                let t = c.g[S0][0] != c.g[V0][0];
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                if t {
                                    break 'b_18c460;
                                } else {
                                    c.cycles += 1;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_18c448;
                                }
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0018c45c)?;
                    break 'b_18c45c;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_18c460;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's fakes, and its magic value.
    const CTRL: u32 = DATA + 0x20_0000;
    const MAGIC: u32 = 0x4f43_8337;
    /// Per-callee result queues: 0x40 bytes per callee, `[count, e0, e1, ..]`.
    const MAIL: u32 = DATA + 0x30_0000;
    /// Fixture objects (above the range of the fakes' result pointers).
    const FIX: u32 = DATA + 0x8_0000;
    /// A faked function used as a virtual method with a controllable result.
    const VMETHOD: u32 = 0x0014_f600;
    /// The saved options struct the sound handler copies (0x290 bytes).
    const CFG: u32 = 0x0053_5610;
    /// Queue entries: 0 = the fake's own pointer, 1 = null, 2 = a0, `val(v)` = v.
    const NULL: u32 = 1;

    fn val(v: u32) -> u32 {
        v.wrapping_add(3)
    }

    /// The faked callees, in queue order.
    const ADDRS: &[u32] = &[
        0x00317670, 0x00317d70, 0x00397cc0, 0x0039c870, 0x0039d860, 0x0039e318, 0x0039f400,
        0x0039e4c0, 0x0014dc80, 0x0014dd58, 0x0014f458, 0x0014f600, 0x0028b180, 0x00287410,
        0x00287488, 0x00287520, 0x00287558, 0x002875d0, 0x00287670, 0x002876a0, 0x002876d0,
        0x00286200, 0x00284c28, 0x00284bb0, 0x00285fb0,
    ];

    fn mail(addr: u32) -> u32 {
        MAIL + 0x40 * ADDRS.iter().position(|&a| a == addr).unwrap() as u32
    }

    /// Queue the results of the next calls of the faked function `addr`.
    fn queue(c: &mut Cpu, addr: u32, entries: &[u32]) {
        for (i, &e) in entries.iter().enumerate() {
            c.bus.write32(mail(addr) + 4 + 4 * i as u32, e);
        }
    }

    fn w(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    fn w16(c: &mut Cpu, addr: u32, v: u16) {
        c.bus.write16(addr, v);
    }

    /// A gated fake callee: logs the call like `fakes::fake_call`, then
    /// returns the next queued result. `RefreshControls` (0x18b380, not
    /// replaced here) finds none of its screen objects.
    fn oc837_fake_call(c: &mut Cpu, addr: u32) {
        let a0 = c.g[A0][0];
        let ra = c.g[RA][0] as u32;
        let refresh = addr == 0x0039_d860 && (0x0018_b380..0x0018_bef8).contains(&ra);
        let mut entry = 0;
        if !refresh {
            let q = mail(addr);
            let n = c.bus.read32(q);
            c.bus.write32(q, n.wrapping_add(1));
            if n < 15 {
                entry = c.bus.read32(q + 4 + 4 * n);
            }
        }
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        if refresh {
            c.g[V0][0] = 0;
            return;
        }
        match entry {
            0 => {}
            1 => c.g[V0][0] = 0,
            2 => c.g[V0][0] = a0,
            m => c.g[V0][0] = m.wrapping_sub(3) as u64,
        }
    }

    macro_rules! oc837_fake {
        ($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(CTRL) != MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                oc837_fake_call(c, $addr);
            }
        };
    }

    oc837_fake!(
        oc837_fake_317670,
        OC837_PREV_317670,
        ee::s10::OVERRIDE_00317670,
        ee::s10::f_00317670__generated,
        0x00317670
    );
    oc837_fake!(
        oc837_fake_317d70,
        OC837_PREV_317D70,
        ee::s10::OVERRIDE_00317D70,
        ee::s10::f_00317d70__generated,
        0x00317d70
    );
    oc837_fake!(
        oc837_fake_397cc0,
        OC837_PREV_397CC0,
        ee::s13::OVERRIDE_00397CC0,
        ee::s13::f_00397cc0__generated,
        0x00397cc0
    );
    oc837_fake!(
        oc837_fake_39c870,
        OC837_PREV_39C870,
        ee::s13::OVERRIDE_0039C870,
        ee::s13::f_0039c870__generated,
        0x0039c870
    );
    oc837_fake!(
        oc837_fake_39d860,
        OC837_PREV_39D860,
        ee::s13::OVERRIDE_0039D860,
        ee::s13::f_0039d860__generated,
        0x0039d860
    );
    oc837_fake!(
        oc837_fake_39e318,
        OC837_PREV_39E318,
        ee::s13::OVERRIDE_0039E318,
        ee::s13::f_0039e318__generated,
        0x0039e318
    );
    oc837_fake!(
        oc837_fake_39f400,
        OC837_PREV_39F400,
        ee::s13::OVERRIDE_0039F400,
        ee::s13::f_0039f400__generated,
        0x0039f400
    );
    oc837_fake!(
        oc837_fake_39e4c0,
        OC837_PREV_39E4C0,
        ee::s13::OVERRIDE_0039E4C0,
        ee::s13::f_0039e4c0__generated,
        0x0039e4c0
    );
    oc837_fake!(
        oc837_fake_14dc80,
        OC837_PREV_14DC80,
        ee::s01::OVERRIDE_0014DC80,
        ee::s01::f_0014dc80__generated,
        0x0014dc80
    );
    oc837_fake!(
        oc837_fake_14dd58,
        OC837_PREV_14DD58,
        ee::s01::OVERRIDE_0014DD58,
        ee::s01::f_0014dd58__generated,
        0x0014dd58
    );
    oc837_fake!(
        oc837_fake_14f458,
        OC837_PREV_14F458,
        ee::s01::OVERRIDE_0014F458,
        ee::s01::f_0014f458__generated,
        0x0014f458
    );
    oc837_fake!(
        oc837_fake_14f600,
        OC837_PREV_14F600,
        ee::s01::OVERRIDE_0014F600,
        ee::s01::f_0014f600__generated,
        0x0014f600
    );
    oc837_fake!(
        oc837_fake_28b180,
        OC837_PREV_28B180,
        ee::s07::OVERRIDE_0028B180,
        ee::s07::f_0028b180__generated,
        0x0028b180
    );
    oc837_fake!(
        oc837_fake_287410,
        OC837_PREV_287410,
        ee::s07::OVERRIDE_00287410,
        ee::s07::f_00287410__generated,
        0x00287410
    );
    oc837_fake!(
        oc837_fake_287488,
        OC837_PREV_287488,
        ee::s07::OVERRIDE_00287488,
        ee::s07::f_00287488__generated,
        0x00287488
    );
    oc837_fake!(
        oc837_fake_287520,
        OC837_PREV_287520,
        ee::s07::OVERRIDE_00287520,
        ee::s07::f_00287520__generated,
        0x00287520
    );
    oc837_fake!(
        oc837_fake_287558,
        OC837_PREV_287558,
        ee::s07::OVERRIDE_00287558,
        ee::s07::f_00287558__generated,
        0x00287558
    );
    oc837_fake!(
        oc837_fake_2875d0,
        OC837_PREV_2875D0,
        ee::s07::OVERRIDE_002875D0,
        ee::s07::f_002875d0__generated,
        0x002875d0
    );
    oc837_fake!(
        oc837_fake_287670,
        OC837_PREV_287670,
        ee::s07::OVERRIDE_00287670,
        ee::s07::f_00287670__generated,
        0x00287670
    );
    oc837_fake!(
        oc837_fake_2876a0,
        OC837_PREV_2876A0,
        ee::s07::OVERRIDE_002876A0,
        ee::s07::f_002876a0__generated,
        0x002876a0
    );
    oc837_fake!(
        oc837_fake_2876d0,
        OC837_PREV_2876D0,
        ee::s07::OVERRIDE_002876D0,
        ee::s07::f_002876d0__generated,
        0x002876d0
    );
    oc837_fake!(
        oc837_fake_286200,
        OC837_PREV_286200,
        ee::s07::OVERRIDE_00286200,
        ee::s07::f_00286200__generated,
        0x00286200
    );
    oc837_fake!(
        oc837_fake_284c28,
        OC837_PREV_284C28,
        ee::s07::OVERRIDE_00284C28,
        ee::s07::f_00284c28__generated,
        0x00284c28
    );
    oc837_fake!(
        oc837_fake_284bb0,
        OC837_PREV_284BB0,
        ee::s07::OVERRIDE_00284BB0,
        ee::s07::f_00284bb0__generated,
        0x00284bb0
    );
    oc837_fake!(
        oc837_fake_285fb0,
        OC837_PREV_285FB0,
        ee::s07::OVERRIDE_00285FB0,
        ee::s07::f_00285fb0__generated,
        0x00285fb0
    );

    fn oc837_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317670),
            oc837_fake_317670,
            &OC837_PREV_317670,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc837_fake_317d70,
            &OC837_PREV_317D70,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_00397CC0),
            oc837_fake_397cc0,
            &OC837_PREV_397CC0,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039C870),
            oc837_fake_39c870,
            &OC837_PREV_39C870,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039D860),
            oc837_fake_39d860,
            &OC837_PREV_39D860,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E318),
            oc837_fake_39e318,
            &OC837_PREV_39E318,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039F400),
            oc837_fake_39f400,
            &OC837_PREV_39F400,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E4C0),
            oc837_fake_39e4c0,
            &OC837_PREV_39E4C0,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc837_fake_14dc80,
            &OC837_PREV_14DC80,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc837_fake_14dd58,
            &OC837_PREV_14DD58,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014F458),
            oc837_fake_14f458,
            &OC837_PREV_14F458,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014F600),
            oc837_fake_14f600,
            &OC837_PREV_14F600,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0028B180),
            oc837_fake_28b180,
            &OC837_PREV_28B180,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00287410),
            oc837_fake_287410,
            &OC837_PREV_287410,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00287488),
            oc837_fake_287488,
            &OC837_PREV_287488,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00287520),
            oc837_fake_287520,
            &OC837_PREV_287520,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00287558),
            oc837_fake_287558,
            &OC837_PREV_287558,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002875D0),
            oc837_fake_2875d0,
            &OC837_PREV_2875D0,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00287670),
            oc837_fake_287670,
            &OC837_PREV_287670,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002876A0),
            oc837_fake_2876a0,
            &OC837_PREV_2876A0,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002876D0),
            oc837_fake_2876d0,
            &OC837_PREV_2876D0,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00286200),
            oc837_fake_286200,
            &OC837_PREV_286200,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00284C28),
            oc837_fake_284c28,
            &OC837_PREV_284C28,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00284BB0),
            oc837_fake_284bb0,
            &OC837_PREV_284BB0,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00285FB0),
            oc837_fake_285fb0,
            &OC837_PREV_285FB0,
        );
    }

    /// The machine for one seed: scrambled registers, the fakes hooked and
    /// gated on.
    fn setup(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc837_install_fakes();
        let r = machine(c, seed, salt, calls);
        w(c, CTRL, MAGIC);
        r
    }

    /// An object whose vtable (at `+8`) has its slots at `+0x40` and `+0x48`
    /// set to harmless fake methods.
    fn vobject(c: &mut Cpu, obj: u32, vt: u32) {
        w(c, obj + 8, vt);
        w16(c, vt + 0x40, 0);
        w(c, vt + 0x44, FAKE);
        w16(c, vt + 0x48, 0);
        w(c, vt + 0x4c, FAKE + 0x10);
    }

    #[test]
    fn c_fe_state_options_sound_init_matches() {
        check(
            ee::s02::f_0018a298__generated,
            c_fe_state_options_sound_init,
            48,
            &|c, seed| {
                let _r = setup(c, seed, 0x18a298, 6);
                let this = FIX;
                let obj = FIX + 0x800;
                c.pc = 0x0018_a298;
                c.g[A0][0] = this as u64;
                w(c, this + 0x10, FIX + 0x400);
                w(
                    c,
                    GAME_GP.wrapping_sub(0x1680),
                    [0, 1, 2][(seed / 3 % 3) as usize],
                );
                vobject(c, obj, FIX + 0xc00);
                // The screen may be missing; each of the three groups too.
                queue(c, 0x0039_7cc0, &[if seed % 3 == 0 { NULL } else { 0 }]);
                let pick = |bit: u32| {
                    if (seed >> bit) & 1 == 0 {
                        NULL
                    } else {
                        val(obj)
                    }
                };
                queue(c, 0x0039_d860, &[pick(2), pick(3), pick(4)]);
            },
        );
    }

    #[test]
    fn c_fe_state_options_sound_on_child_event_matches() {
        check(
            ee::s02::f_0018a3d8__generated,
            c_fe_state_options_sound_on_child_event,
            96,
            &|c, seed| {
                let mut r = setup(c, seed, 0x18a3d8, 8);
                let this = FIX;
                let popup = FIX + 0x400;
                let pvt = FIX + 0x800;
                let hv = r.next_u32() | 0x1000_0000;
                c.pc = 0x0018_a3d8;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = popup as u64;
                c.g[A2][0] = match seed % 4 {
                    0 => 0x15,
                    1 => 0x17,
                    _ => 0x16,
                };
                w(c, popup + 0xc, hv);
                w(c, popup + 8, pvt);
                w16(c, pvt + 0xb8, 0);
                w(c, pvt + 0xbc, VMETHOD);
                // The popup's name hashes to the "reset options" id or not.
                let hit = (seed / 4) % 3 != 0;
                queue(c, 0x0031_7670, &[if hit { val(hv) } else { 0 }]);
                // The popup's answer method: answered "no" (0) or "yes".
                queue(
                    c,
                    VMETHOD,
                    &[if (seed / 12) % 2 == 0 { NULL } else { val(1) }],
                );
                for i in 0..0xa4 {
                    w(c, CFG + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn c_fe_state_options_sound_on_activate_matches() {
        check(
            ee::s02::f_0018a870__generated,
            c_fe_state_options_sound_on_activate,
            12,
            &|c, seed| {
                let _r = setup(c, seed, 0x18a870, 1);
                c.pc = 0x0018_a870;
                c.g[A0][0] = FIX as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_options_sound_set_speaker_mode_matches() {
        check(
            ee::s02::f_0018bef8__generated,
            c_fe_state_options_sound_set_speaker_mode,
            60,
            &|c, seed| {
                let _r = setup(c, seed, 0x18bef8, 6);
                c.pc = 0x0018_bef8;
                c.g[A0][0] = FIX as u64;
                c.g[A1][0] = [0u64, 1, 2, 3, 7, u64::MAX][(seed % 6) as usize];
                // 28b180 returns the audio object; its music system's method
                // (slot 2 of the object at +0x1d8) answers "accepted" or not.
                let (p, vt1, o1, o2, v) = (
                    FIX + 0x400,
                    FIX + 0x800,
                    FIX + 0xc00,
                    FIX + 0x1000,
                    FIX + 0x1400,
                );
                w(c, p + 0x118, vt1);
                w(c, vt1, o1);
                w(c, o1 + 0x1d8, o2);
                w(c, o2 + 4, v);
                w16(c, v + 0x10, 0);
                w(c, v + 0x14, VMETHOD);
                queue(c, 0x0028_b180, &[val(p)]);
                queue(
                    c,
                    VMETHOD,
                    &[if (seed / 6) % 2 == 0 { val(1) } else { NULL }],
                );
            },
        );
    }

    #[test]
    fn c_fe_state_options_hud_on_activate_matches() {
        check(
            ee::s02::f_0018c198__generated,
            c_fe_state_options_hud_on_activate,
            12,
            &|c, seed| {
                let _r = setup(c, seed, 0x18c198, 1);
                c.pc = 0x0018_c198;
                c.g[A0][0] = FIX as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_options_hud_on_widget_action_matches() {
        check(
            ee::s02::f_0018c2a0__generated,
            c_fe_state_options_hud_on_widget_action,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x18c2a0, 8);
                let this = FIX;
                let (eng, eng_vt, v1) = (FIX + 0x400, FIX + 0x800, FIX + 0xc00);
                let (widget, wvt) = (FIX + 0x1000, FIX + 0x1400);
                let (iface, ivt) = (FIX + 0x1800, FIX + 0x1c00);
                let hv = r.next_u32() | 0x1000_0000;
                let sub = seed / 10;
                c.pc = 0x0018_c2a0;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = if seed % 25 == 24 { 0 } else { widget as u64 };
                c.g[A2][0] = [2u64, 2, 2, 2, 6, 6, 7, 1, 0, 3][(seed % 10) as usize];
                // The engine's custom interface: its "previous state" method.
                w(c, this + 0x10, eng);
                w(c, eng, eng_vt);
                w(c, eng_vt + 4, v1);
                w16(c, v1 + 0x28, 0);
                w(c, v1 + 0x2c, VMETHOD);
                // The widget: show/hide slot, name hash, misc byte.
                w(c, widget + 8, wvt);
                w16(c, wvt + 0x48, 0);
                w(c, wvt + 0x4c, FAKE + 0x10);
                w(c, widget + 0x18, r.next_u32());
                w(c, widget + 0x38, hv);
                // The BE interface: a submit slot.
                w(c, iface + 0xc, ivt);
                w16(c, ivt + 8, 0);
                w(c, ivt + 0xc, FAKE + 0x20);
                queue(c, 0x0014_dd58, &[val(iface)]);
                // Which of the three name compares matches (or none).
                let mut h = [0u32; 3];
                if sub % 4 < 3 {
                    h[(sub % 4) as usize] = val(hv);
                }
                queue(c, 0x0031_7670, &h);
                queue(c, VMETHOD, &[if sub % 2 == 0 { NULL } else { val(1) }]);
                w(c, CFG, r.next_u32());
            },
        );
    }
}
