//! Native asynchronous map-picture binding and fade for Event Select.

use crate::call::{call_ptr, guest, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0020_2828, event_select_update, "EventSelect_Update")];

const IMAGE_SLOT: u32 = 0x2d8;
const IMAGE_VISIBLE: u32 = 0x2dc;
const IMAGE_ALPHA: u32 = 0x2e0;
const IMAGE_OBJECT: u32 = 0x2e4;
const UI_STATE: u32 = 0x2ec;
const IMAGE_DISABLED: u32 = 0x2f0;
const SHOW_OBJECT: u32 = 0x0039_e948;
const HIDE_OBJECT: u32 = 0x0039_e8b8;

fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

fn state_object(c: &mut Cpu, name: u32, show: bool, back: u32) -> Flow {
    c.g[A0][0] = c.lw(at(c, S0, UI_STATE));
    c.g[A1][0] = sx(name);
    call_ptr(c, if show { SHOW_OBJECT } else { HIDE_OBJECT }, back)
}

/// `void EventSelect_Update(EventSelect* self)` (0x202828).
/// Fade and show an already-bound map picture. Unless explicitly disabled,
/// bind a completed FEAsyncManager slot to the UIPic texture for next frame.
/// The picture handle is independent of the selected peak's table index.
pub fn event_select_update(c: &mut Cpu) {
    let this = c.g[A0][0] as u32;
    let back = c.g[RA][0] as u32;
    if crate::custom_tracks::menu_preview().is_some() {
        c.bus.write32(this + IMAGE_DISABLED, 0);
    }
    original_event_select_update(c);
    if c.pc == back && crate::custom_tracks::menu_preview().is_some() {
        guest(c, |c| crate::bonus_menu_mode::refresh_catalog_preview(c, this, back));
    }
}

fn original_event_select_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S0, 0x90);
        frame.save(c, S5, 0x40);
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save_ra(c, 0x30);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0020_2850)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0020_285c)?;
        c.g[S5][0] = c.g[V0][0];
        c.g[V0][0] = c.lw(at(c, S0, IMAGE_VISIBLE));
        if c.g[V0][0] != 0 {
            c.cycles += 7;
            c.lwc1(1, at(c, S0, IMAGE_ALPHA));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 0);
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.cycles += 4;
                c.lwc1(0, r(c, GP).wrapping_sub(0x5530));
                c.add_s(0, 1, 0);
                c.swc1(0, at(c, S0, IMAGE_ALPHA));
            }
            c.g[A0][0] = c.lw(at(c, S0, IMAGE_OBJECT));
            c.cycles += 26;
            c.g[A1][0] = c.g[SP][0];
            c.lwc1(0, at(c, S0, IMAGE_ALPHA));
            // Preserve the original unaligned colour-copy register effects.
            c.ldl(V0, at(c, A0, 0x23));
            c.ldr(V0, at(c, A0, 0x1c));
            c.ldl(V1, at(c, A0, 0x2b));
            c.ldr(V1, at(c, A0, 0x24));
            c.sdl(V0, at(c, SP, 7));
            c.sdr(V0, r(c, SP));
            c.sdl(V1, at(c, SP, 0xf));
            c.sdr(V1, at(c, SP, 8));
            c.swc1(0, r(c, SP));
            c.g[V0][0] = c.lw(at(c, A0, 8));
            c.g[V1][0] = c.lh(at(c, V0, 0x58));
            c.g[A2][0] = c.lw(at(c, V0, 0x5c));
            let target = r(c, A2);
            c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, V1)));
            call_ptr(c, target, 0x0020_28d8)?;
            c.g[A3][0] = c.lw(at(c, S0, IMAGE_OBJECT));
            c.g[A1][0] = 1;
            c.g[V0][0] = c.lw(at(c, A3, 8));
            c.g[A0][0] = c.lh(at(c, V0, 0x48));
            c.g[V1][0] = c.lw(at(c, V0, 0x4c));
            let target = r(c, V1);
            c.g[A0][0] = sx(r(c, A3).wrapping_add(r(c, A0)));
            call_ptr(c, target, 0x0020_28f4)?;
            c.g[V0][0] = c.lw(r(c, GP).wrapping_sub(0xb54));
            if c.g[V0][0] == 0 {
                c.cycles += 14;
                c.g[A1][0] = 0x0047_0000;
                state_object(c, 0x0047_0588, true, 0x0020_2910)?;
                c.g[A1][0] = 0x0047_0000;
                state_object(c, 0x0047_0598, true, 0x0020_2920)?;
                c.g[A1][0] = 0x0047_0000;
                state_object(c, 0x0047_05a8, true, 0x0020_2930)?;
            } else {
                c.g[A1][0] = 0x0047_0000;
                c.cycles += 5;
                state_object(c, 0x0047_07a0, true, 0x0020_2944)?;
            }
        } else {
            c.sw(at(c, S0, IMAGE_ALPHA), 0);
            c.cycles += 10;
            c.g[A1][0] = 0;
            c.g[A2][0] = c.lw(at(c, S0, IMAGE_OBJECT));
            c.g[V0][0] = c.lw(at(c, A2, 8));
            c.g[A0][0] = c.lh(at(c, V0, 0x48));
            c.g[V1][0] = c.lw(at(c, V0, 0x4c));
            let target = r(c, V1);
            c.g[A0][0] = sx(r(c, A2).wrapping_add(r(c, A0)));
            call_ptr(c, target, 0x0020_2968)?;
            c.g[V0][0] = c.lw(r(c, GP).wrapping_sub(0xb54));
            if c.g[V0][0] == 0 {
                c.cycles += 14;
                c.g[A1][0] = 0x0047_0000;
                state_object(c, 0x0047_0588, false, 0x0020_2984)?;
                c.g[A1][0] = 0x0047_0000;
                state_object(c, 0x0047_0598, false, 0x0020_2994)?;
                c.g[A1][0] = 0x0047_0000;
                state_object(c, 0x0047_05a8, false, 0x0020_29a4)?;
            } else {
                c.g[A1][0] = 0x0047_0000;
                c.cycles += 4;
                state_object(c, 0x0047_07a0, false, 0x0020_29b8)?;
            }
        }
        c.g[V0][0] = c.lw(at(c, S0, IMAGE_DISABLED));
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            c.g[S1][0] = 0;
            c.g[S2][0] = sx(at(c, SP, 0x10));
            c.g[S4][0] = 0x0047_0000;
            c.g[A2][0] = sx(r(c, S1).wrapping_add(1));
            loop {
                c.cycles += 7;
                c.g[A0][0] = c.g[S2][0];
                c.g[S3][0] = c.g[A2][0];
                c.g[A1][0] = sx(at(c, S4, 0x7b0));
                call_ptr(c, 0x0041_7828, 0x0020_29e8)?;
                c.g[V1][0] = c.lw(at(c, S0, IMAGE_VISIBLE));
                let hidden = c.g[V1][0] == 0;
                c.g[V0][0] = c.lw(r(c, GP).wrapping_sub(0xb54));
                let mut show = false;
                if hidden {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw(at(c, S0, UI_STATE));
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        c.g[A0][0] = c.lw(at(c, S0, UI_STATE));
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S5][0];
                        call_ptr(c, 0x0014_4bc0, 0x0020_2a04)?;
                        c.g[V1][0] = c.lw(at(c, V0, 0x54));
                        if c.g[S1][0] != c.g[V1][0] {
                            c.g[A0][0] = c.lw(at(c, S0, UI_STATE));
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(r(c, S0));
                            let nonzero = c.g[V0][0] != 0;
                            c.g[A0][0] = c.lw(at(c, S0, UI_STATE));
                            if !nonzero {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw(r(c, GP).wrapping_sub(0xb5c));
                                c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                                show = c.g[V0][0] == 0;
                            }
                        }
                    }
                }
                if show {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S2][0];
                    call_ptr(c, SHOW_OBJECT, 0x0020_2a34)?;
                    c.g[S1][0] = c.g[S3][0];
                } else {
                    c.cycles += 3;
                    c.g[A1][0] = c.g[S2][0];
                    call_ptr(c, HIDE_OBJECT, 0x0020_2a48)?;
                    c.g[S1][0] = c.g[S3][0];
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 3) as u64;
                let again = c.g[V0][0] != 0;
                c.g[A2][0] = sx(r(c, S1).wrapping_add(1));
                if !again {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0020_29d8);
                }
            }
            c.cycles += 8;
            c.g[V0][0] = c.lw(r(c, GP).wrapping_sub(0x848));
            c.g[A1][0] = c.lw(at(c, S0, IMAGE_SLOT));
            c.g[S1][0] = c.lw(at(c, V0, 0x11c));
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x001a_39c0, 0x0020_2a6c)?;
            c.g[V1][0] = 3;
            if c.g[V0][0] == c.g[V1][0] {
                c.cycles += 3;
                c.g[V0][0] = c.lw(at(c, S0, IMAGE_VISIBLE));
                if c.g[V0][0] == 0 {
                    c.cycles += 3;
                    c.g[A2][0] = c.lw(at(c, S0, IMAGE_OBJECT));
                    let missing = c.g[A2][0] == 0;
                    c.g[V1][0] = 0x11c;
                    if !missing {
                        c.cycles += 9;
                        c.g[V0][0] = c.lw(at(c, S0, IMAGE_SLOT));
                        c.g[A1][0] = 1;
                        c.mult(0, A0, V0, V1);
                        c.g[V0][0] = sx(r(c, A0).wrapping_add(r(c, S1)));
                        c.g[A0][0] = c.lw(at(c, V0, 0x110));
                        c.sw(at(c, A2, 0x7c), 0);
                        c.sw(at(c, A2, 0x78), c.g[A0][0]);
                        c.sw(at(c, S0, IMAGE_VISIBLE), c.g[A1][0]);
                        c.sw(at(c, S0, IMAGE_ALPHA), 0);
                    }
                    c.cycles += 1;
                }
            }
        }
        frame.restore(c, S0, 0x90);
        c.cycles += 8;
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    });
}
