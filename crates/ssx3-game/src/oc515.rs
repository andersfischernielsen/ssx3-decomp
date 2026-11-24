//! Staging module oc515 (wave left513b.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc515`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0022_c830, game_module_render_ui, "GameModule_RenderUI")];

/// `base + off` as a 32-bit guest address (`off` may be negative).
#[inline(always)]
fn render_ui_at(base: u64, off: i32) -> u32 {
    (base as u32).wrapping_add(off as u32)
}

/// `void GameModule_RenderUI(GameModule* self)` (0x22c830): the frame's UI pass.
///
/// For each view of the view manager (`self->ViewMan`: cameras at +4, count at
/// +0x10, current view at +0x14, views from +0x18) it makes the view current,
/// sets the viewport window, the perspective and the view matrix from the
/// camera, stores the layer in the state top, sets the render-state bits and
/// draws bucket 3. Then the 2D pass: an orthographic viewport, the debug font
/// scale and colour, the script engine, the UI engine, the load indicator and,
/// while active, the menu. It clears the screen, draws the frame with depth fog
/// off, runs the audio and world trigger updates and resets `mUpdateTick`.
///
/// Every call either returns to the address after it or the function returns at
/// once (host-thread unwind), so each call is followed by `?`.
pub fn game_module_render_ui(c: &mut Cpu) {
    guest(c, |c| {
        ssx3_ee_rt::prof::enter(c, 0x0022_c830);
        c.cycles += 20;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, S1, 0xa0);
        frame.save(c, S2, 0x90);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0xb0);
        frame.save(c, S3, 0x80);
        frame.save(c, S4, 0x70);
        frame.save(c, S5, 0x60);
        frame.save(c, S6, 0x50);
        frame.save(c, S7, 0x40);
        frame.save(c, FP, 0x30);
        frame.save_ra(c, 0x20);
        frame.save_fpr(c, 23, 0xd8);
        frame.save_fpr(c, 22, 0xd0);
        frame.save_fpr(c, 21, 0xc8);
        frame.save_fpr(c, 20, 0xc0);

        c.g[V0][0] = c.lw(render_ui_at(c.g[S1][0], 0x84));
        c.g[V1][0] = c.lw(render_ui_at(c.g[V0][0], 0x10));
        let no_views = c.g[V1][0] == 0;
        c.g[S2][0] = 0;
        if !no_views {
            c.cycles += 10;
            c.g[S5][0] = 0xffff_ffff_fe7f_ffff;
            c.g[S4][0] = 0xffff_ffff_ffcf_ffff;
            c.g[S3][0] = 0xffff_ffff_fff0_0fff;
            c.g[FP][0] = 0x30_0000;
            c.g[S6][0] = 0x18;
            c.g[S7][0] = 0xffff_ffff_ffff_fc1f;
            c.g[V0][0] = c.lw(render_ui_at(c.g[S1][0], 0x84));
            loop {
                // One view: make it current (cur = i), then the render state.
                c.cycles += 81;
                c.g[A2][0] = 0x1;
                c.g[T1][0] = c.lw(render_ui_at(c.g[GP][0], -0x854));
                c.sw(render_ui_at(c.g[V0][0], 0x14), c.g[S2][0]);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = c.lw(render_ui_at(c.g[S1][0], 0x84));
                c.g[A3][0] = c.lw(render_ui_at(c.g[T1][0], 0x10d8));
                c.g[V0][0] = c.lw(render_ui_at(c.g[V1][0], 0x14));
                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S6][0] as u32));
                c.g[A0][0] = c.lh(render_ui_at(c.g[A3][0], 0xc0));
                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[T0][0] = c.lw(render_ui_at(c.g[A3][0], 0xc4));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                let t = c.g[T0][0] as u32;
                c.g[S0][0] = c.lw(render_ui_at(c.g[V1][0], 0x4));
                call_ptr(c, t, 0x0022_c8e8)?;

                // SetViewportPerspective(cam->View, cam->Near, cam->Far).
                c.g[A1][0] = c.lw(render_ui_at(c.g[GP][0], -0x854));
                c.lwc1(12, c.g[S0][0] as u32);
                c.g[V0][0] = c.lw(render_ui_at(c.g[A1][0], 0x10d8));
                c.lwc1(13, render_ui_at(c.g[S0][0], 0x4));
                c.g[A0][0] = c.lh(render_ui_at(c.g[V0][0], 0xc8));
                c.g[V1][0] = c.lw(render_ui_at(c.g[V0][0], 0xcc));
                c.lwc1(14, render_ui_at(c.g[S0][0], 0x8));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0022_c90c)?;

                // SetView(&cam->CamMat).
                c.g[A2][0] = c.lw(render_ui_at(c.g[GP][0], -0x854));
                c.g[A1][0] = sx(render_ui_at(c.g[S0][0], 0x40));
                c.g[V0][0] = c.lw(render_ui_at(c.g[A2][0], 0x10d8));
                c.g[A0][0] = c.lh(render_ui_at(c.g[V0][0], 0x110));
                c.g[V1][0] = c.lw(render_ui_at(c.g[V0][0], 0x114));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0022_c928)?;

                // The layer into the state top, then the render-state bits.
                c.g[A2][0] = c.lw(render_ui_at(c.g[GP][0], -0x854));
                c.g[T1][0] = 0x80_0000;
                c.g[A0][0] = sx(render_ui_at(c.g[GP][0], 0x2898));
                c.g[A1][0] = 0x3;
                c.g[V0][0] = c.lw(render_ui_at(c.g[A2][0], 0x24));
                c.g[A3][0] = c.lw(render_ui_at(c.g[A2][0], 0xe84));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = c.lw(render_ui_at(c.g[A2][0], 0xf50));
                c.g[A2][0] = 0xffff_ffff_ffff_f07f;
                c.sh(render_ui_at(c.g[A3][0], 0x12), c.g[V1][0]);
                c.g[T0][0] = c.lw(render_ui_at(c.g[GP][0], -0x854));
                c.g[V1][0] = c.lw(render_ui_at(c.g[T0][0], 0xe84));
                c.g[V0][0] = c.lw(render_ui_at(c.g[V1][0], 0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.g[V0][0] = c.g[V0][0] | 0x380;
                c.sw(render_ui_at(c.g[V1][0], 0x4), c.g[V0][0]);
                c.g[V1][0] = 0xffff_ffff_ffff_ffcf;
                c.g[A2][0] = c.lw(render_ui_at(c.g[T0][0], 0xe84));
                c.g[V0][0] = c.lw(render_ui_at(c.g[A2][0], 0x0));
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                c.g[V0][0] = c.g[V0][0] | 0x30;
                c.sw(render_ui_at(c.g[A2][0], 0x0), c.g[V0][0]);
                c.g[V1][0] = c.lw(render_ui_at(c.g[T0][0], 0xe84));
                c.g[V0][0] = c.lw(render_ui_at(c.g[V1][0], 0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[S5][0];
                c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
                c.sw(render_ui_at(c.g[V1][0], 0x4), c.g[V0][0]);
                c.g[A2][0] = c.lw(render_ui_at(c.g[T0][0], 0xe84));
                c.g[V0][0] = c.lw(render_ui_at(c.g[A2][0], 0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[S4][0];
                c.g[V0][0] = c.g[V0][0] | c.g[FP][0];
                c.sw(render_ui_at(c.g[A2][0], 0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw(render_ui_at(c.g[T0][0], 0xe84));
                c.g[V0][0] = c.lw(render_ui_at(c.g[V1][0], 0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[S3][0];
                c.g[V0][0] = c.g[V0][0] | 0xd000;
                c.sw(render_ui_at(c.g[V1][0], 0x4), c.g[V0][0]);
                c.g[A2][0] = c.lw(render_ui_at(c.g[T0][0], 0xe84));
                c.g[V0][0] = c.lw(render_ui_at(c.g[A2][0], 0x8));
                c.g[V0][0] = c.g[V0][0] & c.g[S7][0];
                c.g[V0][0] = c.g[V0][0] | 0x80;
                c.sw(render_ui_at(c.g[A2][0], 0x8), c.g[V0][0]);
                // cBucketMan_renderBucket(gBucketMan, 3).
                call_ptr(c, 0x0035_50a0, 0x0022_c9d8)?;

                // while (++i < count): back edge with the event check.
                c.g[V1][0] = c.lw(render_ui_at(c.g[S1][0], 0x84));
                c.g[V0][0] = c.lw(render_ui_at(c.g[V1][0], 0x10));
                c.g[V0][0] = (c.g[S2][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V0][0] = c.lw(render_ui_at(c.g[S1][0], 0x84));
                if event_due(c) {
                    deliver_event_here(c, 0x0022_c8a8);
                }
            }
        }

        // The screen width query (vt slot +0x50 of the view state), into f20.
        c.cycles += 11;
        c.g[A0][0] = c.lw(render_ui_at(c.g[GP][0], -0x854));
        c.g[V1][0] = c.lw(render_ui_at(c.g[A0][0], 0x10d8));
        c.g[V0][0] = c.lh(render_ui_at(c.g[V1][0], 0xb8));
        c.g[S2][0] = sx(render_ui_at(c.g[V1][0], 0xbc));
        c.g[A1][0] = c.lh(render_ui_at(c.g[V1][0], 0x50));
        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = c.lw(render_ui_at(c.g[V1][0], 0x54));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, t, 0x0022_ca10)?;
        let neg = (c.g[V0][0] as i64) < 0;
        c.g[A1][0] = c.lw(render_ui_at(c.g[GP][0], -0x854));
        if neg {
            // Unsigned word to float: halve, keep the low bit, convert, double.
            c.cycles += 7;
            c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(V0, 20);
            c.cvt_s_w(20, 20);
            c.add_s(20, 20, 20);
            c.g[V1][0] = c.lw(render_ui_at(c.g[A1][0], 0x10d8));
        } else {
            c.cycles += 4;
            c.mtc1(V0, 20);
            c.cvt_s_w(20, 20);
            c.g[V1][0] = c.lw(render_ui_at(c.g[A1][0], 0x10d8));
        }

        // The screen height query (vt slot +0x58), into f15.
        c.cycles += 6;
        c.g[A0][0] = c.lh(render_ui_at(c.g[V1][0], 0x58));
        c.g[V0][0] = c.lw(render_ui_at(c.g[V1][0], 0x5c));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0022_ca54)?;
        let neg = (c.g[V0][0] as i64) < 0;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
        if neg {
            c.cycles += 5;
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(V0, 15);
            c.cvt_s_w(15, 15);
            c.add_s(15, 15, 15);
        } else {
            c.cycles += 4;
            c.mtc1(V0, 15);
            c.cvt_s_w(15, 15);
        }

        // The 2D pass: the font scale and colour, the debug font, the overlays.
        c.cycles += 87;
        c.mtc1(0, 12);
        c.mov_s(14, 20);
        c.g[V0][0] = c.lw(c.g[S2][0] as u32);
        c.g[A0][0] = c.g[S0][0];
        c.mov_s(20, 12);
        c.mov_s(13, 12);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 23);
        let t = c.g[V0][0] as u32;
        c.g[A1][0] = 0x1;
        call_ptr(c, t, 0x0022_caa8)?;
        c.lwc1(21, render_ui_at(c.g[GP][0], -0x51dc));
        c.g[A3][0] = c.lw(render_ui_at(c.g[GP][0], -0x854));
        c.g[A2][0] = 0;
        c.g[AT][0] = 0x4420_0000;
        c.mtc1(AT, 14);
        c.mov_s(16, 20);
        c.g[V0][0] = c.lw(render_ui_at(c.g[A3][0], 0x10d8));
        c.mov_s(17, 23);
        c.g[AT][0] = 0x43f0_0000;
        c.mtc1(AT, 15);
        c.mov_s(12, 20);
        c.g[A0][0] = c.lh(render_ui_at(c.g[V0][0], 0xd0));
        c.mov_s(13, 20);
        c.g[V1][0] = c.lw(render_ui_at(c.g[V0][0], 0xd4));
        c.g[A1][0] = 0;
        c.g[AT][0] = 0x4000_0000;
        c.mtc1(AT, 22);
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0022_caf4)?;
        c.g[A2][0] = c.lw(render_ui_at(c.g[GP][0], -0x854));
        c.g[A1][0] = sx(0x50_0000u32.wrapping_sub(0xe60));
        c.g[V0][0] = c.lw(render_ui_at(c.g[A2][0], 0x10d8));
        c.g[A0][0] = c.lh(render_ui_at(c.g[V0][0], 0x110));
        c.g[V1][0] = c.lw(render_ui_at(c.g[V0][0], 0x114));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0022_cb14)?;
        c.g[A1][0] = c.lw(render_ui_at(c.g[GP][0], -0x854));
        c.g[A0][0] = 0xffff_ffff_ffff_fc1f;
        c.g[A2][0] = 0xffff_ffff_ffff_ff83;
        c.g[V1][0] = c.lw(render_ui_at(c.g[A1][0], 0xe84));
        c.g[V0][0] = c.lw(render_ui_at(c.g[V1][0], 0x8));
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        c.g[V0][0] = c.g[V0][0] | 0x300;
        c.sw(render_ui_at(c.g[V1][0], 0x8), c.g[V0][0]);
        c.g[A0][0] = c.lw(render_ui_at(c.g[A1][0], 0xe84));
        c.g[V0][0] = c.lw(render_ui_at(c.g[A0][0], 0x4));
        c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
        c.g[V0][0] = c.g[V0][0] | 0x14;
        c.sw(render_ui_at(c.g[A0][0], 0x4), c.g[V0][0]);
        c.g[V1][0] = c.lw(render_ui_at(c.g[S1][0], 0x24));
        c.lwc1(0, render_ui_at(c.g[V1][0], 0x30));
        c.lwc1(1, render_ui_at(c.g[V1][0], 0x34));
        c.mul_s(0, 0, 21);
        c.mul_s(1, 1, 21);
        c.swc1(0, render_ui_at(c.g[V1][0], 0x38));
        c.swc1(1, render_ui_at(c.g[V1][0], 0x3c));
        c.swc1(22, frame.at(c, 0x0));
        c.g[V0][0] = c.lw(render_ui_at(c.g[S1][0], 0x24));
        c.swc1(22, frame.at(c, 0x4));
        c.ldl(4, frame.at(c, 0x7));
        c.ldr(4, frame.at(c, 0x0));
        c.sdl(4, render_ui_at(c.g[V0][0], 0x2f));
        c.sdr(4, render_ui_at(c.g[V0][0], 0x28));
        c.swc1(23, frame.at(c, 0x10));
        c.g[V0][0] = c.lw(render_ui_at(c.g[S1][0], 0x24));
        c.swc1(21, frame.at(c, 0x14));
        c.swc1(21, frame.at(c, 0x18));
        c.swc1(20, frame.at(c, 0x1c));
        c.ldl(4, frame.at(c, 0x17));
        c.ldr(4, frame.at(c, 0x10));
        c.ldl(5, frame.at(c, 0x1f));
        c.ldr(5, frame.at(c, 0x18));
        c.sdl(4, render_ui_at(c.g[V0][0], 0x47));
        c.sdr(4, render_ui_at(c.g[V0][0], 0x40));
        c.sdl(5, render_ui_at(c.g[V0][0], 0x4f));
        c.sdr(5, render_ui_at(c.g[V0][0], 0x48));
        // cSSXScriptEngine_Render(ScriptEngine), cUIEngine_render2D(UI),
        // cBXLoadIndicator_Render(LoadIndicator), then the menu while active.
        c.g[A0][0] = c.lw(render_ui_at(c.g[GP][0], -0x84c));
        call_ptr(c, 0x0027_89c0, 0x0022_cbbc)?;
        c.g[A0][0] = c.lw(render_ui_at(c.g[S1][0], 0x48));
        call_ptr(c, 0x0039_7df8, 0x0022_cbc4)?;
        c.g[A0][0] = c.lw(render_ui_at(c.g[S1][0], 0x94));
        call_ptr(c, 0x001f_31e0, 0x0022_cbcc)?;
        c.g[A0][0] = c.lw(render_ui_at(c.g[S1][0], 0xa4));
        call_ptr(c, 0x002c_c098, 0x0022_cbd4)?;
        let menu_active = c.g[V0][0] == 0u64;
        c.g[V0][0] = c.lw(render_ui_at(c.g[GP][0], -0x854));
        if !menu_active {
            c.cycles += 23;
            c.g[A0][0] = c.lw(render_ui_at(c.g[S1][0], 0xa4));
            call_ptr(c, 0x002c_c350, 0x0022_cbe4)?;
            c.g[V0][0] = c.lw(render_ui_at(c.g[S1][0], 0x24));
            c.swc1(22, frame.at(c, 0x4));
            c.swc1(22, frame.at(c, 0x0));
            c.ldl(4, frame.at(c, 0x7));
            c.ldr(4, frame.at(c, 0x0));
            c.sdl(4, render_ui_at(c.g[V0][0], 0x2f));
            c.sdr(4, render_ui_at(c.g[V0][0], 0x28));
            c.swc1(23, frame.at(c, 0x0));
            c.g[V0][0] = c.lw(render_ui_at(c.g[S1][0], 0x24));
            c.swc1(21, frame.at(c, 0x8));
            c.swc1(21, frame.at(c, 0x4));
            c.swc1(20, frame.at(c, 0xc));
            c.ldl(4, frame.at(c, 0x7));
            c.ldr(4, frame.at(c, 0x0));
            c.ldl(5, frame.at(c, 0xf));
            c.ldr(5, frame.at(c, 0x8));
            c.sdl(4, render_ui_at(c.g[V0][0], 0x47));
            c.sdr(4, render_ui_at(c.g[V0][0], 0x40));
            c.sdl(5, render_ui_at(c.g[V0][0], 0x4f));
            c.sdr(5, render_ui_at(c.g[V0][0], 0x48));
            c.g[V0][0] = c.lw(render_ui_at(c.g[GP][0], -0x854));
        }

        // The end of the frame: the clear colour, the draw with depth fog off,
        // then the audio and world trigger updates.
        c.cycles += 44;
        c.g[V1][0] = c.lw(render_ui_at(c.g[V0][0], 0x10d8));
        c.g[A0][0] = c.lh(render_ui_at(c.g[V1][0], 0x78));
        c.swc1(20, frame.at(c, 0x8));
        c.swc1(20, frame.at(c, 0x0));
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.swc1(20, frame.at(c, 0x4));
        c.g[V0][0] = c.lw(render_ui_at(c.g[V1][0], 0x7c));
        let t = c.g[V0][0] as u32;
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, t, 0x0022_cc5c)?;
        c.g[S0][0] = c.lw(render_ui_at(c.g[GP][0], 0x1234));
        c.sw(render_ui_at(c.g[GP][0], 0x1234), 0u64);
        c.g[A1][0] = c.lw(render_ui_at(c.g[GP][0], -0x854));
        c.g[V0][0] = c.lw(render_ui_at(c.g[A1][0], 0x10d8));
        c.g[A0][0] = c.lh(render_ui_at(c.g[V0][0], 0xa0));
        c.g[V1][0] = c.lw(render_ui_at(c.g[V0][0], 0xa4));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0022_cc7c)?;
        c.sw(render_ui_at(c.g[GP][0], 0x1234), c.g[S0][0]);
        c.g[A1][0] = 0x1;
        c.lwc1(0, render_ui_at(c.g[GP][0], -0x51d8));
        c.lwc1(12, render_ui_at(c.g[S1][0], 0xac));
        c.cvt_s_w(12, 12);
        c.g[A0][0] = c.lw(render_ui_at(c.g[GP][0], 0x410));
        c.mul_s(12, 12, 0);
        // cSSXAudio_Update(gSSXAudio, 1, mUpdateTick / 60).
        call_ptr(c, 0x0028_5bf8, 0x0022_cc9c)?;
        c.g[A0][0] = c.lw(render_ui_at(c.g[GP][0], 0x21e4));
        // cWorldTriggerManager_Update(gWorldTriggerManager).
        call_ptr(c, 0x002b_5d78, 0x0022_cca4)?;
        c.sw(render_ui_at(c.g[S1][0], 0xac), 0u64);

        frame.restore(c, S0, 0xb0);
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore(c, S6, 0x50);
        frame.restore(c, S7, 0x40);
        frame.restore(c, FP, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 23, 0xd8);
        frame.restore_fpr(c, 22, 0xd0);
        frame.restore_fpr(c, 21, 0xc8);
        frame.restore_fpr(c, 20, 0xc0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
use crate::difftest::fakes;
#[cfg(test)]
use crate::difftest::{fill, Rng, DATA, GAME_GP};
#[cfg(test)]
use std::ptr::addr_of_mut;
#[cfg(test)]
use std::sync::atomic::AtomicUsize;

/// Magic word in the machine's RAM that turns the callee fakes on.
#[cfg(test)]
const OC515_GATE: u32 = DATA + 0x20_0000;
#[cfg(test)]
const OC515_GATE_MAGIC: u32 = 0x0515_0515;
/// Per-call result overrides (call index n: a flag word at +0, the 64-bit
/// value at +8): the fakes return the value in v0 when the flag is set.
#[cfg(test)]
const OC515_RES: u32 = DATA + 0x20_1000;
#[cfg(test)]
const OC515_RES_N: u32 = 48;
#[cfg(test)]
const OC515_SEEDS: u32 = 200;

#[cfg(test)]
static OC515_PREV_RENDER_BUCKET: AtomicUsize = AtomicUsize::new(0);
#[cfg(test)]
static OC515_PREV_UI_RENDER: AtomicUsize = AtomicUsize::new(0);
#[cfg(test)]
static OC515_PREV_LOAD_INDICATOR: AtomicUsize = AtomicUsize::new(0);
#[cfg(test)]
static OC515_PREV_MENU_ACTIVE: AtomicUsize = AtomicUsize::new(0);
#[cfg(test)]
static OC515_PREV_MENU_RENDER: AtomicUsize = AtomicUsize::new(0);
#[cfg(test)]
static OC515_PREV_AUDIO_UPDATE: AtomicUsize = AtomicUsize::new(0);
#[cfg(test)]
static OC515_PREV_WORLD_TRIGGER: AtomicUsize = AtomicUsize::new(0);
#[cfg(test)]
static OC515_PREV_MOVIE_PAUSE: AtomicUsize = AtomicUsize::new(0);

/// A fake call to `target` returning to `ra`, with the per-index result override.
#[cfg(test)]
fn oc515_fake_done(c: &mut Cpu, target: u32, ra: u32) {
    let n = c.bus.read32(fakes::LOG_COUNT);
    fakes::fake(c, target, ra);
    if n < OC515_RES_N && c.pc == ra {
        let slot = OC515_RES + 16 * n;
        if c.bus.read32(slot) != 0 {
            c.g[V0][0] = c.bus.read64(slot + 8);
        }
    }
}

/// A callee fake, gated on the magic word; with the gate off the body the slot
/// held before (`prev`, or the generated `generated`) runs.
#[cfg(test)]
fn oc515_gated(c: &mut Cpu, target: u32, prev: &AtomicUsize, generated: GuestFn) {
    if c.bus.read32(OC515_GATE) != OC515_GATE_MAGIC {
        return fakes::fall_back(c, prev, generated);
    }
    let ra = c.g[RA][0] as u32;
    oc515_fake_done(c, target, ra);
}

/// The indirect-call hook of the fixture's machines: function pointers in the
/// view state point at fake addresses, which land here.
#[cfg(test)]
fn oc515_miss(c: &mut Cpu, target: u32) {
    let ra = c.g[RA][0] as u32;
    oc515_fake_done(c, target, ra);
}

#[cfg(test)]
fn oc515_fake_render_bucket(c: &mut Cpu) {
    oc515_gated(
        c,
        0x0035_50a0,
        &OC515_PREV_RENDER_BUCKET,
        ee::s11::f_003550a0__generated,
    )
}

#[cfg(test)]
fn oc515_fake_ui_render(c: &mut Cpu) {
    oc515_gated(
        c,
        0x0039_7df8,
        &OC515_PREV_UI_RENDER,
        ee::s13::f_00397df8__generated,
    )
}

#[cfg(test)]
fn oc515_fake_load_indicator(c: &mut Cpu) {
    oc515_gated(
        c,
        0x001f_31e0,
        &OC515_PREV_LOAD_INDICATOR,
        ee::s04::f_001f31e0__generated,
    )
}

#[cfg(test)]
fn oc515_fake_menu_active(c: &mut Cpu) {
    oc515_gated(
        c,
        0x002c_c098,
        &OC515_PREV_MENU_ACTIVE,
        ee::s09::f_002cc098__generated,
    )
}

#[cfg(test)]
fn oc515_fake_menu_render(c: &mut Cpu) {
    oc515_gated(
        c,
        0x002c_c350,
        &OC515_PREV_MENU_RENDER,
        ee::s09::f_002cc350__generated,
    )
}

#[cfg(test)]
fn oc515_fake_audio_update(c: &mut Cpu) {
    oc515_gated(
        c,
        0x0028_5bf8,
        &OC515_PREV_AUDIO_UPDATE,
        ee::s07::f_00285bf8__generated,
    )
}

#[cfg(test)]
fn oc515_fake_world_trigger(c: &mut Cpu) {
    oc515_gated(
        c,
        0x002b_5d78,
        &OC515_PREV_WORLD_TRIGGER,
        ee::s08::f_002b5d78__generated,
    )
}

#[cfg(test)]
fn oc515_fake_movie_pause(c: &mut Cpu) {
    oc515_gated(
        c,
        0x0027_89c0,
        &OC515_PREV_MOVIE_PAUSE,
        ee::s07::f_002789c0__generated,
    )
}

/// Hooks the fakes into the override slots of the callees (once per process).
#[cfg(test)]
fn oc515_install_fakes() {
    fakes::hook(
        addr_of_mut!(ee::s11::OVERRIDE_003550A0),
        oc515_fake_render_bucket,
        &OC515_PREV_RENDER_BUCKET,
    );
    fakes::hook(
        addr_of_mut!(ee::s13::OVERRIDE_00397DF8),
        oc515_fake_ui_render,
        &OC515_PREV_UI_RENDER,
    );
    fakes::hook(
        addr_of_mut!(ee::s04::OVERRIDE_001F31E0),
        oc515_fake_load_indicator,
        &OC515_PREV_LOAD_INDICATOR,
    );
    fakes::hook(
        addr_of_mut!(ee::s09::OVERRIDE_002CC098),
        oc515_fake_menu_active,
        &OC515_PREV_MENU_ACTIVE,
    );
    fakes::hook(
        addr_of_mut!(ee::s09::OVERRIDE_002CC350),
        oc515_fake_menu_render,
        &OC515_PREV_MENU_RENDER,
    );
    fakes::hook(
        addr_of_mut!(ee::s07::OVERRIDE_00285BF8),
        oc515_fake_audio_update,
        &OC515_PREV_AUDIO_UPDATE,
    );
    fakes::hook(
        addr_of_mut!(ee::s08::OVERRIDE_002B5D78),
        oc515_fake_world_trigger,
        &OC515_PREV_WORLD_TRIGGER,
    );
    fakes::hook(
        addr_of_mut!(ee::s07::OVERRIDE_002789C0),
        oc515_fake_movie_pause,
        &OC515_PREV_MOVIE_PAUSE,
    );
}

/// The machine for one seed: scrambled registers and fake hooks, the gate word,
/// per-call result overrides, the game globals (GrMan at gp-0x854 with its view
/// state at +0x10d8 and state top at +0xe84), a view manager with 0..3 views,
/// and the self object with its pointers. `a0` is the self object.
#[cfg(test)]
fn oc515_setup(c: &mut Cpu, seed: u32) -> Rng {
    oc515_install_fakes();
    let mut r = fakes::machine(c, seed, 515, 12);
    c.hooks.miss = oc515_miss;
    c.pc = 0x0022_c830;
    c.bus.write32(OC515_GATE, OC515_GATE_MAGIC);
    for n in 0..OC515_RES_N {
        let slot = OC515_RES + 16 * n;
        c.bus.write32(slot, (r.next_u32() % 4 != 0) as u32);
        let v = match r.next_u32() % 3 {
            0 => (r.next_u32() | 0x8000_0000) as i32 as i64 as u64,
            1 => (r.next_u32() & 0x7fff_ffff) as u64,
            _ => 0,
        };
        c.bus.write64(slot + 8, v);
    }

    let me = DATA + 0x1000;
    let view = DATA + 0x4000;
    let gr = DATA + 0x1_0000;
    let st = DATA + 0x1_2000;
    let top = DATA + 0x1_8000;

    // Game globals.
    c.bus.write32(GAME_GP - 0x854, gr);
    c.bus.write32(GAME_GP - 0x84c, DATA + 0x3200);
    c.bus.write32(GAME_GP + 0x410, DATA + 0x3300);
    c.bus.write32(GAME_GP + 0x21e4, DATA + 0x3400);
    c.bus.write32(GAME_GP - 0x51d8, r.f32(4.0));
    c.bus.write32(GAME_GP - 0x51dc, r.f32(4.0));
    c.bus.write32(GAME_GP + 0x1234, r.next_u32());

    // GrMan: view state and state top, with a small index at +0x24.
    c.bus.write32(gr + 0x10d8, st);
    c.bus.write32(gr + 0xe84, top);
    c.bus.write32(gr + 0x24, r.next_u32() % 4);
    fill(c, &mut r, top, 0x40);
    // View state: a {delta, function} pair per virtual slot, delta at +0xb8.
    for (k, off) in [0x50u32, 0x58, 0x78, 0xa0, 0xc0, 0xc8, 0xd0, 0x110]
        .into_iter()
        .enumerate()
    {
        c.bus.write16(st + off, ((r.next_u32() as i16) & !3) as u16);
        c.bus.write32(st + off + 4, fakes::FAKE + 0x10 * k as u32);
    }
    c.bus
        .write16(st + 0xb8, ((r.next_u32() as i16) & !3) as u16);

    // View manager: count at +0x10, cameras at +4.
    c.bus.write32(view + 0x10, seed % 4);
    for i in 0..3u32 {
        let cam = DATA + 0x5000 + 0x100 * i;
        c.bus.write32(view + 4 + 4 * i, cam);
        c.bus.write32(cam, r.f32(100.0));
        c.bus.write32(cam + 4, r.f32(100.0));
        c.bus.write32(cam + 8, r.f32(100.0));
    }

    // The self object.
    c.bus.write32(me + 0x84, view);
    c.bus.write32(me + 0x24, DATA + 0x2000);
    c.bus.write32(me + 0x48, DATA + 0x3000);
    c.bus.write32(me + 0x94, DATA + 0x3100);
    c.bus.write32(me + 0xa4, DATA + 0x3180);
    c.bus.write32(me + 0xac, r.next_u32() % 100);
    fill(c, &mut r, DATA + 0x2000, 0x20);
    c.g[A0][0] = sx(me);
    r
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::covered;

    #[test]
    fn game_module_render_ui_matches() {
        covered(
            ee::s05::f_0022c830__generated,
            game_module_render_ui,
            OC515_SEEDS,
            &|c, seed| {
                oc515_setup(c, seed);
            },
        );
    }
}
