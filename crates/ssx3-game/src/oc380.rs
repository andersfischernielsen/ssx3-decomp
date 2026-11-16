//! Staging module oc380 (wave wave217): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc380`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::game::render::particle::{add32, at, fconst, fr, gp, ldf, lt, p, setf, shl32};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0022_04a0, hud_draw_fading_text, "HUD_DrawFadingText")];

// ---------------------------------------------------------------------------
// HUD_DrawFadingText.

/// Fields `HUD_DrawFadingText` reads (the HUD in a0, the element table in a1).
pub(crate) mod fading_text {
    /// The text box's element in the element table (a1 + 0x870, 0x24 bytes;
    /// +0x20 horizontal alignment, +0x21 vertical, +0x22 draw row).
    pub const ELEM: u32 = 0x870;
    /// Fade-in progress (0..1; 1 = fully in).
    pub const FADE: u32 = 0x194;
    /// Text alpha (scales the colour's first component).
    pub const ALPHA: u32 = 0x198;
    /// The lines: a string each, 0x68 apart, with width (+0x60) and height
    /// (+0x64).
    pub const LINES: u32 = 0x19c;
    pub const LINE_WIDTH: u32 = 0x1fc;
    pub const LINE_HEIGHT: u32 = 0x200;
    pub const LINE_STRIDE: u32 = 0x68;
    pub const LINE_COUNT: u32 = 0x3a4;
    /// Lines past this index are drawn dimmed.
    pub const LIT_LINES: u32 = 0x3a8;
    /// The box size (w, h).
    pub const BOX_SIZE: u32 = 0x3ac;
    /// The text objects: B for the later lines, A for the first.
    pub const TEXT_B: u32 = 0x428;
    pub const TEXT_A: u32 = 0x42c;
    /// The sprite table; the three box pieces are at +0x50.
    pub const SPRITES: u32 = 0x494;
    /// Line colours (16 bytes each, static data).
    pub const COLOUR_FIRST: u32 = 0x004c_86a8;
    pub const COLOUR_LIT: u32 = 0x004c_86c8;
    pub const COLOUR_DIM: u32 = 0x004c_86e8;
}

/// `ldl/ldr reg, a` (an unaligned doubleword load).
#[inline(always)]
fn fading_text_ld(c: &mut Cpu, reg: usize, a: u32) {
    c.ldl(reg, a.wrapping_add(7));
    c.ldr(reg, a);
}

/// `sdl/sdr reg, a` (an unaligned doubleword store).
#[inline(always)]
fn fading_text_sd(c: &mut Cpu, reg: usize, a: u32) {
    c.sdl(reg, a.wrapping_add(7));
    c.sdr(reg, a);
}

/// Copy a line colour (16 bytes at `src`) to sp+0x10 through `ra`/`rb`,
/// then `f0` = its first component.
#[inline(always)]
fn fading_text_colour(c: &mut Cpu, src: u32, ra: usize, rb: usize) {
    let sp_ = sp(c);
    fading_text_ld(c, ra, src);
    fading_text_ld(c, rb, src.wrapping_add(8));
    fading_text_sd(c, ra, p(sp_, 0x10));
    fading_text_sd(c, rb, p(sp_, 0x18));
    ldf(c, 0, p(sp_, 0x10));
}

/// `void HUD_DrawFadingText(HUD* hud, HUDElem* elems, int fromSide)`
/// (0x2204a0): a text box that slides in while `hud->fade` (+0x194) rises
/// to 1 (from the bottom of the screen, or from the left when `fromSide`).
/// Lays the box out from its element (+0x870 in the element table), draws
/// the three box pieces (sprites at +0x4e4) as 4-vertex strips through the
/// renderer's vtable (+0x238), then each line (+0x19c, 0x68 apart) with
/// `Font_DrawText`, left/centre/right aligned: the first line in its own
/// colour with text object A, lit lines (up to +0x3a8) and dimmed lines in
/// theirs with text object B. Leaves both text objects' offsets at (2, 2).
pub fn hud_draw_fading_text(c: &mut Cpu) {
    guest(c, |c| {
        use fading_text::*;
        c.cycles += 63;
        let frame = Frame::open(c, 0x200);
        c.g[A3][0] = c.lw(gp(c, -0x854));
        frame.save(c, S0, 0x1d0);
        frame.save(c, S1, 0x1c0);
        frame.save(c, S2, 0x1b0);
        frame.save(c, S3, 0x1a0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x190);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S7, 0x160);
        c.g[S4][0] = c.g[A2][0];
        frame.save_fpr(c, 23, 0x1f8);
        c.g[S1][0] = add32(c.g[S3][0], ELEM as u64);
        frame.save_fpr(c, 20, 0x1e0);
        frame.save(c, S5, 0x180);
        frame.save(c, S6, 0x170);
        frame.save_ra(c, 0x150);
        frame.save_fpr(c, 22, 0x1f0);
        frame.save_fpr(c, 21, 0x1e8);
        fconst(c, 20, 0x3f80_0000);
        // The screen: (G->vtable[0x150])(G + delta), its height at +8.
        c.g[V1][0] = c.lw(at(c, A3, 0x10d8));
        c.g[A0][0] = c.lh(at(c, V1, 0x150));
        c.g[V0][0] = c.lw(at(c, V1, 0x154));
        let target = r32(c, V0);
        c.g[A0][0] = add32(c.g[A3][0], c.g[A0][0]);
        call_ptr(c, target, 0x0022_0508)?;
        let sp_ = sp(c);
        let screen = ldf(c, 0, at(c, V0, 8));
        c.g[S0][0] = sx(p(sp_, 0x20));
        c.swc1(20, sp_);
        c.g[A1][0] = c.g[S0][0];
        // f23 = 1 / screen scale
        let inv = fr(c, 20).div(screen);
        setf(c, 23, inv);
        c.swc1(20, p(sp_, 4));
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001e_91f8, 0x0022_0528)?; // HUD_GetElemPos(elem, sp+0x20)
        c.g[A1][0] = sx(p(sp_, 0x30));
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001e_9290, 0x0022_0534)?; // HUD_GetElemScale(elem, sp+0x30)
        let sy = ldf(c, 0, p(sp_, 0x34));
        c.g[A2][0] = c.g[S0][0];
        c.g[S7][0] = c.lb(at(c, S3, ELEM + 0x20));
        c.g[A0][0] = c.g[S2][0];
        let sy = sy * fr(c, 23);
        setf(c, 0, sy);
        c.g[S0][0] = c.lb(at(c, S3, ELEM + 0x22));
        c.g[T2][0] = c.lb(at(c, S3, ELEM + 0x21));
        c.g[A1][0] = sx(p(sp_, 0x50));
        c.g[A3][0] = sx(p(sp_, 0x40));
        c.g[T0][0] = c.g[SP][0];
        c.swc1(0, p(sp_, 0x34));
        c.g[T1][0] = c.g[S7][0];
        // Box size to sp+0x40, its height scaled.
        let size_at = at(c, S2, BOX_SIZE);
        fading_text_ld(c, V0, size_at);
        fading_text_sd(c, V0, p(sp_, 0x40));
        c.g[T3][0] = 0;
        let h = ldf(c, 0, p(sp_, 0x44));
        let h = h * fr(c, 23);
        setf(c, 0, h);
        c.swc1(0, p(sp_, 0x44));
        // HUD_AlignElemPos(hud, sp+0x50, pos, size, (1,1), alignX, alignY, 0)
        call_ptr(c, 0x001f_10f8, 0x0022_0588)?;
        // Slide in: (1 - fade)^2 of the way from the screen bottom (510) or
        // from left of the screen edge.
        let fade = ldf(c, 0, at(c, S2, FADE));
        let one = fr(c, 20);
        let fading = lt(c, fade, one);
        c.g[A0][0] = add32(c.g[S3][0], ELEM as u64);
        if fading {
            c.cycles += 3;
            let k = one - fade;
            setf(c, 3, k);
            let k = k * k;
            setf(c, 3, k);
            if c.g[S4][0] == 0 {
                c.cycles += 8;
                let y = ldf(c, 1, p(sp_, 0x54));
                let bottom = fconst(c, 0, 0x43ff_0000);
                let d = bottom - y;
                setf(c, 0, d);
                let d = k * d;
                setf(c, 0, d);
                setf(c, 1, y + d);
                c.swc1(1, p(sp_, 0x54));
                c.g[A0][0] = add32(c.g[S3][0], ELEM as u64);
            } else {
                c.cycles += 10;
                let x = ldf(c, 0, p(sp_, 0x50));
                let margin = fconst(c, 1, 0x41f0_0000);
                let w = ldf(c, 2, p(sp_, 0x40));
                let d = x + margin;
                setf(c, 1, d);
                let d = d + w;
                setf(c, 1, d);
                let d = k * d;
                setf(c, 1, d);
                setf(c, 0, x - d);
                c.swc1(0, p(sp_, 0x50));
            }
        }
        c.cycles += 10;
        c.g[A1][0] = sx(p(sp_, 0x10));
        call_ptr(c, 0x001e_91a8, 0x0022_05f8)?; // HUD_GetElemColor(elem, sp+0x10)
        c.g[S5][0] = add32(c.g[S0][0], 2);
        c.g[S4][0] = sx(p(sp_, 0x60));
        c.g[A1][0] = shl32(c.g[S0][0], 5);
        c.g[T0][0] = sx(p(sp_, 0x120));
        c.g[A0][0] = add32(c.g[S2][0], SPRITES as u64);
        c.g[V1][0] = u64::MAX;
        c.g[V0][0] = 3;
        // (an emptied loop: 3 down to -1)
        loop {
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            c.g[A2][0] = 0x80;
            if c.g[V0][0] == c.g[V1][0] {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0022_0618);
            }
        }
        // Four vertices at sp+0x60 (0x30 apart): colour (0x80, 0x80, 0x80,
        // 0x80), q = 1.
        c.cycles += 5;
        fconst(c, 0, 0x3f80_0000);
        c.g[V1][0] = c.g[S4][0];
        c.g[A3][0] = add32(c.g[S4][0], 0xc0);
        let a2 = c.g[A2][0];
        c.sw(at(c, V1, 0x1c), a2);
        loop {
            c.cycles += 8;
            let v = r32(c, V1);
            c.sw(p(v, 0x10), a2);
            c.sw(p(v, 0x14), a2);
            c.sw(p(v, 0x18), a2);
            c.swc1(0, p(v, 8));
            c.g[V1][0] = add32(c.g[V1][0], 0x30);
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A3][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.sw(at(c, V1, 0x1c), a2);
            if event_due(c) {
                deliver_event_here(c, 0x0022_0648);
            }
        }
        // Piece widths (42, w - 56, 14) at sp+0x120; the box position at
        // sp+0x130; f22 = its bottom, f21 = 1.
        c.cycles += 38;
        let w = ldf(c, 0, p(sp_, 0x40));
        c.g[S0][0] = add32(c.g[A0][0], 0x50);
        let edges = fconst(c, 3, 0x4260_0000);
        c.g[S1][0] = c.g[T0][0];
        fconst(c, 1, 0x4160_0000);
        c.g[A2][0] = 0xffff_ffff_ffff_fc1f;
        setf(c, 0, w - edges);
        fconst(c, 2, 0x4228_0000);
        c.swc1(1, p(sp_, 0x138));
        c.g[A1][0] &= 0x3e0;
        c.swc1(2, p(sp_, 0x130));
        c.g[S3][0] = 2;
        c.swc1(0, p(sp_, 0x134));
        fading_text_ld(c, V0, p(sp_, 0x130));
        c.g[V1][0] = c.lw(p(sp_, 0x138));
        fading_text_sd(c, V0, p(sp_, 0x120));
        let v = c.g[V1][0];
        c.sw(p(sp_, 0x128), v);
        c.g[A0][0] = c.lw(gp(c, -0x854));
        let y = ldf(c, 1, p(sp_, 0x54));
        let h = ldf(c, 0, p(sp_, 0x44));
        fading_text_ld(c, T0, p(sp_, 0x50));
        fading_text_sd(c, T0, p(sp_, 0x130));
        setf(c, 22, y + h);
        fconst(c, 21, 0x3f80_0000);
        // Draw row: bits 5..9 of the draw state's flags = elem row.
        c.g[V1][0] = c.lw(at(c, A0, 0xe84));
        c.g[V0][0] = c.lw(at(c, V1, 8));
        c.g[V0][0] &= c.g[A2][0];
        c.g[V0][0] |= c.g[A1][0];
        let v = c.g[V0][0];
        c.sw(at(c, V1, 8), v);
        // The three box pieces, left to right.
        loop {
            c.cycles += 58;
            let off = ldf(c, 20, r32(c, S1));
            c.g[A1][0] = 4;
            let x = ldf(c, 0, p(sp_, 0x130));
            c.g[A2][0] = c.g[S4][0];
            ldf(c, 1, p(sp_, 0x134));
            c.g[A3][0] = 0;
            c.swc1(0, p(sp_, 0x140));
            setf(c, 20, x + off);
            c.sw(p(sp_, 0x148), 0);
            c.g[S1][0] = add32(c.g[S1][0], 4);
            c.swc1(1, p(sp_, 0x144));
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(1));
            c.swc1(21, p(sp_, 0x14c));
            c.g[A0][0] = c.lw(r32(c, S0));
            // Vertex positions: (x, y), (x2, y), (x, bottom), (x2, bottom).
            c.lq(V1, p(sp_, 0x140));
            c.g[S0][0] = add32(c.g[S0][0], 4);
            c.swc1(20, p(sp_, 0x140));
            c.sw(p(sp_, 0x148), 0);
            c.sq(V1, p(sp_, 0x80));
            c.lq(V0, p(sp_, 0x140));
            c.swc1(0, p(sp_, 0x140));
            c.sw(p(sp_, 0x148), 0);
            c.swc1(22, p(sp_, 0x144));
            c.sq(V0, p(sp_, 0xb0));
            c.lq(V1, p(sp_, 0x140));
            c.swc1(20, p(sp_, 0x140));
            c.sw(p(sp_, 0x148), 0);
            c.sq(V1, p(sp_, 0xe0));
            c.lq(V0, p(sp_, 0x140));
            c.sq(V0, p(sp_, 0x110));
            // Texture coordinates from the sprite (u0 +0x10, v0 +0xc, u1
            // +0x14, v1 +0x18).
            let s = r32(c, A0);
            c.lwc1(0, p(s, 0x10));
            c.swc1(0, p(sp_, 0x60));
            c.lwc1(1, p(s, 0xc));
            c.swc1(1, p(sp_, 0x64));
            c.lwc1(0, p(s, 0x14));
            c.swc1(0, p(sp_, 0x90));
            c.lwc1(1, p(s, 0xc));
            c.g[T0][0] = c.lw(gp(c, -0x854));
            c.swc1(1, p(sp_, 0x94));
            c.lwc1(0, p(s, 0x10));
            c.swc1(0, p(sp_, 0xc0));
            c.lwc1(1, p(s, 0x18));
            c.swc1(1, p(sp_, 0xc4));
            c.lwc1(0, p(s, 0x14));
            c.swc1(0, p(sp_, 0xf0));
            c.lwc1(1, p(s, 0x18));
            c.swc1(1, p(sp_, 0xf4));
            // The texture into the draw state, then
            // (G->vtable[0x238])(G + delta, 4, verts, 0).
            c.g[V1][0] = c.lw(s);
            c.g[V0][0] = c.lw(at(c, T0, 0xe84));
            let v = c.g[V1][0];
            c.sh(at(c, V0, 0x10), v);
            c.g[T0][0] = c.lw(gp(c, -0x854));
            c.g[V0][0] = c.lw(at(c, T0, 0x10d8));
            c.g[A0][0] = c.lh(at(c, V0, 0x238));
            c.g[V1][0] = c.lw(at(c, V0, 0x23c));
            let target = r32(c, V1);
            c.g[A0][0] = add32(c.g[T0][0], c.g[A0][0]);
            call_ptr(c, target, 0x0022_07e0)?;
            let more = (c.g[S3][0] as i64) >= 0;
            c.swc1(20, p(sp_, 0x130));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0022_0700);
            }
        }
        // Text scales: A = size * elem scale, B = size * elem scale * k;
        // both offsets (0, 0); draw row + 2; y += gap.
        c.cycles += 52;
        c.sw(p(sp_, 0x60), 0);
        c.g[A1][0] = shl32(c.g[S5][0], 5);
        c.sw(p(sp_, 0x64), 0);
        c.g[A2][0] = 0xffff_ffff_ffff_fc1f;
        let scale_x = ldf(c, 0, p(sp_, 0x30));
        c.g[A1][0] &= 0x3e0;
        c.g[V0][0] = c.lw(at(c, S2, TEXT_A));
        c.g[S4][0] = 0;
        let k = ldf(c, 4, gp(c, -0x530c));
        let tw = ldf(c, 1, at(c, V0, 0x30));
        let th = ldf(c, 2, at(c, V0, 0x34));
        setf(c, 1, tw * scale_x);
        let gap = ldf(c, 5, gp(c, -0x5308));
        c.swc1(1, at(c, V0, 0x38));
        let scale_y = ldf(c, 0, p(sp_, 0x34));
        setf(c, 2, th * scale_y);
        c.swc1(2, at(c, V0, 0x3c));
        let scale_x = ldf(c, 3, p(sp_, 0x30));
        let scale_y = ldf(c, 1, p(sp_, 0x34));
        let kx = scale_x * k;
        setf(c, 3, kx);
        c.g[V1][0] = c.lw(at(c, S2, TEXT_B));
        setf(c, 1, scale_y * k);
        c.swc1(3, p(sp_, 0x70));
        c.swc1(1, p(sp_, 0x74));
        let tw = ldf(c, 0, at(c, V1, 0x30));
        let th = ldf(c, 2, at(c, V1, 0x34));
        setf(c, 0, tw * kx);
        c.swc1(0, at(c, V1, 0x38));
        let ky = ldf(c, 1, p(sp_, 0x74));
        setf(c, 2, th * ky);
        c.swc1(2, at(c, V1, 0x3c));
        c.g[V0][0] = c.lw(at(c, S2, TEXT_A));
        fading_text_ld(c, V1, p(sp_, 0x60));
        let dst = at(c, V0, 0x28);
        fading_text_sd(c, V1, dst);
        c.g[V1][0] = c.lw(at(c, S2, TEXT_B));
        fading_text_ld(c, T4, p(sp_, 0x60));
        let dst = at(c, V1, 0x28);
        fading_text_sd(c, T4, dst);
        c.g[A0][0] = c.lw(gp(c, -0x854));
        c.g[V1][0] = c.lw(at(c, A0, 0xe84));
        c.g[V0][0] = c.lw(at(c, V1, 8));
        c.g[V0][0] &= c.g[A2][0];
        c.g[V0][0] |= c.g[A1][0];
        let v = c.g[V0][0];
        c.sw(at(c, V1, 8), v);
        let y = ldf(c, 0, p(sp_, 0x54));
        c.g[V0][0] = c.lw(at(c, S2, LINE_COUNT));
        setf(c, 0, y + gap);
        let no_lines = (c.g[V0][0] as i64) <= 0;
        c.swc1(0, p(sp_, 0x54));
        if !no_lines {
            c.cycles += 8;
            c.g[S3][0] = add32(c.g[S2][0], LINES as u64);
            fconst(c, 20, 0x41a0_0000);
            c.g[S1][0] = c.g[S3][0];
            c.g[S0][0] = add32(c.g[S2][0], LINE_WIDTH as u64);
            c.g[S6][0] = 0;
            c.g[S5][0] = c.g[S1][0];
            loop {
                // The line's colour and text object.
                c.cycles += 4;
                c.g[V0][0] = c.lw(at(c, S2, LIT_LINES));
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                c.g[A0][0] = c.lw(at(c, S2, TEXT_B));
                if c.g[V0][0] != 0 {
                    c.cycles += 12;
                    c.g[V0][0] = 0x004d_0000;
                    c.g[T0][0] = sx(COLOUR_DIM);
                    fading_text_colour(c, COLOUR_DIM, A1, A2);
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0x004d_0000;
                    if c.g[S4][0] == 0 {
                        c.cycles += 13;
                        c.g[V0][0] = 0x004d_0000;
                        c.g[A0][0] = c.lw(at(c, S2, TEXT_A));
                        c.g[A1][0] = sx(COLOUR_FIRST);
                        fading_text_colour(c, COLOUR_FIRST, T4, T5);
                    } else {
                        c.cycles += 10;
                        c.g[T4][0] = sx(COLOUR_LIT);
                        fading_text_colour(c, COLOUR_LIT, A2, A3);
                    }
                }
                // colour.r *= alpha; text->colour = colour
                c.cycles += 13;
                let alpha = ldf(c, 1, at(c, S2, ALPHA));
                let r = fr(c, 0) * alpha;
                setf(c, 0, r);
                c.swc1(0, p(sp_, 0x10));
                fading_text_ld(c, T5, p(sp_, 0x10));
                fading_text_ld(c, V0, p(sp_, 0x18));
                let dst = at(c, A0, 0x40);
                fading_text_sd(c, T5, dst);
                fading_text_sd(c, V0, p(dst, 8));
                let align = c.g[S7][0] as i64;
                let x = ldf(c, 12, p(sp_, 0x50));
                if align < 0 {
                    // Left: x + 20.
                    c.cycles += 6;
                    c.g[A1][0] = c.g[S1][0];
                    ldf(c, 13, p(sp_, 0x54));
                    let m = fr(c, 20);
                    setf(c, 12, x + m);
                    call_ptr(c, 0x0039_1cb0, 0x0022_09c0)?; // Font_DrawText
                } else {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S5][0];
                    if align > 0 {
                        // Right: x - 20 - width.
                        c.cycles += 8;
                        let x = ldf(c, 12, p(sp_, 0x50));
                        let width = ldf(c, 0, r32(c, S0));
                        let m = fr(c, 20);
                        let x = x - m;
                        setf(c, 12, x);
                        ldf(c, 13, p(sp_, 0x54));
                        setf(c, 12, x - width);
                        call_ptr(c, 0x0039_1cb0, 0x0022_09e8)?;
                    } else {
                        // Centred: x + (box w - width) * 0.5.
                        c.cycles += 12;
                        let bw = ldf(c, 12, p(sp_, 0x40));
                        c.g[A1][0] = c.g[S3][0];
                        let width = ldf(c, 0, r32(c, S0));
                        let half = fconst(c, 2, 0x3f00_0000);
                        let d = bw - width;
                        setf(c, 12, d);
                        let x = ldf(c, 1, p(sp_, 0x50));
                        ldf(c, 13, p(sp_, 0x54));
                        let d = d * half;
                        setf(c, 12, d);
                        setf(c, 12, x + d);
                        call_ptr(c, 0x0039_1cb0, 0x0022_0a1c)?;
                    }
                }
                c.g[V0][0] = add32(c.g[S2][0], c.g[S6][0]);
                // Next line: y += height / screen scale.
                c.cycles += 14;
                let y = ldf(c, 1, p(sp_, 0x54));
                let lh = ldf(c, 0, at(c, V0, LINE_HEIGHT));
                c.g[S4][0] = add32(c.g[S4][0], 1);
                c.g[V0][0] = c.lw(at(c, S2, LINE_COUNT));
                c.g[S3][0] = add32(c.g[S3][0], LINE_STRIDE as u64);
                let lh = lh * fr(c, 23);
                setf(c, 0, lh);
                c.g[S0][0] = add32(c.g[S0][0], LINE_STRIDE as u64);
                c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[S6][0] = add32(c.g[S6][0], LINE_STRIDE as u64);
                c.g[S5][0] = add32(c.g[S5][0], LINE_STRIDE as u64);
                c.g[S1][0] = add32(c.g[S1][0], LINE_STRIDE as u64);
                setf(c, 1, y + lh);
                let more = c.g[V0][0] != 0;
                c.swc1(1, p(sp_, 0x54));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0022_08d8);
                }
            }
        }
        // Both text objects' offsets back to (2, 2).
        c.cycles += 29;
        fconst(c, 0, 0x4000_0000);
        c.g[V1][0] = c.lw(at(c, S2, TEXT_A));
        c.swc1(0, p(sp_, 0x74));
        c.swc1(0, p(sp_, 0x70));
        fading_text_ld(c, A2, p(sp_, 0x70));
        let dst = at(c, V1, 0x28);
        fading_text_sd(c, A2, dst);
        frame.restore(c, S0, 0x1d0);
        c.g[V0][0] = c.lw(at(c, S2, TEXT_B));
        frame.restore(c, S1, 0x1c0);
        fading_text_ld(c, T5, p(sp_, 0x70));
        let dst = at(c, V0, 0x28);
        fading_text_sd(c, T5, dst);
        frame.restore(c, S2, 0x1b0);
        frame.restore(c, S3, 0x1a0);
        frame.restore(c, S4, 0x190);
        frame.restore(c, S5, 0x180);
        frame.restore(c, S6, 0x170);
        frame.restore(c, S7, 0x160);
        frame.restore_ra(c, 0x150);
        frame.restore_fpr(c, 23, 0x1f8);
        frame.restore_fpr(c, 22, 0x1f0);
        frame.restore_fpr(c, 21, 0x1e8);
        frame.restore_fpr(c, 20, 0x1e0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE, LOG_RESULTS};
    use crate::difftest::{check, DATA, GAME_GP};
    use crate::game::render::particle::tests::{f, uf};

    /// The fading text box against the generated body: the slide-in from
    /// the bottom and from the side, all three alignments, 0-4 lines with
    /// the first, lit and dimmed colours, the renderer and `Font_DrawText`'s
    /// vtable calls faked (some unwinding).
    #[test]
    fn hud_draw_fading_text_matches() {
        use fading_text::*;
        check(
            ee::s05::f_002204a0__generated,
            hud_draw_fading_text,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2204a0, 8);
                c.pc = 0x0022_04a0;
                let hud = DATA;
                let elems = DATA + 0x1000;
                let text_a = DATA + 0x2000;
                let text_b = DATA + 0x2100;
                let screen = DATA + 0x2200;
                let draw = DATA + 0x2300;
                let sprites = DATA + 0x2400;
                let g = DATA + 0x3000;
                let vt = DATA + 0x4000;
                // The game object G (gp-0x854, also Font_DrawText's gp+0x2a90):
                // the renderer vtable (+0x10d8) and the draw state (+0xe84).
                c.bus.write32(GAME_GP.wrapping_sub(0x854), g);
                c.bus.write32(GAME_GP.wrapping_add(0x2a90), g);
                c.bus.write32(g + 0x10d8, vt);
                c.bus.write32(g + 0xe84, draw);
                c.bus.write32(draw + 8, r.next_u32());
                for (i, off) in [0x150u32, 0x238, 0x280].into_iter().enumerate() {
                    let delta = (r.next_u32() as i16 as i32 & !3 & 0xff) as u16;
                    c.bus.write16(vt + off, delta);
                    c.bus.write32(vt + off + 4, FAKE + 0x10 * i as u32);
                }
                // The first call returns the screen (scale at +8).
                c.bus.write32(LOG_RESULTS, screen);
                c.bus.write32(screen + 8, f(uf(&mut r, 0.5, 2.0)));
                // Static colours and scale constants.
                for base in [COLOUR_FIRST, COLOUR_LIT, COLOUR_DIM] {
                    for k in 0..4 {
                        c.bus.write32(base + 4 * k, f(uf(&mut r, 0.0, 1.0)));
                    }
                }
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x530c), f(uf(&mut r, 0.5, 1.5)));
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x5308), f(uf(&mut r, 0.0, 10.0)));
                // The element: position, size, scale, colour, alignment, row.
                let e = elems + ELEM;
                c.bus.write16(e, (r.next_u32() % 640) as u16);
                c.bus.write16(e + 2, (r.next_u32() % 448) as u16);
                c.bus.write16(e + 4, (r.next_u32() % 300) as u16);
                c.bus.write16(e + 6, (r.next_u32() % 200) as u16);
                c.bus.write32(e + 8, f(uf(&mut r, 0.5, 2.0)));
                c.bus.write32(e + 0xc, f(uf(&mut r, 0.5, 2.0)));
                for k in 0..4 {
                    c.bus.write32(e + 0x10 + 4 * k, f(uf(&mut r, 0.0, 1.0)));
                }
                c.bus.write8(e + 0x20, ((seed / 2 % 3) as i8 - 1) as u8);
                c.bus.write8(e + 0x21, ((r.next_u32() % 3) as i8 - 1) as u8);
                c.bus.write8(e + 0x22, (r.next_u32() % 8) as u8);
                // The HUD.
                let fade = if seed % 3 == 0 {
                    uf(&mut r, 1.0, 2.0)
                } else {
                    uf(&mut r, 0.0, 1.0)
                };
                c.bus.write32(hud + FADE, f(fade));
                c.bus.write32(hud + ALPHA, f(uf(&mut r, 0.0, 1.0)));
                let lines = if seed % 7 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 4
                };
                c.bus.write32(hud + LINE_COUNT, lines);
                c.bus
                    .write32(hud + LIT_LINES, (r.next_u32() % 5).wrapping_sub(1));
                c.bus.write32(hud + BOX_SIZE, f(uf(&mut r, 60.0, 300.0)));
                c.bus
                    .write32(hud + BOX_SIZE + 4, f(uf(&mut r, 20.0, 200.0)));
                for i in 0..4 {
                    let l = hud + LINES + LINE_STRIDE * i;
                    c.bus.write8(l, b'A' + i as u8);
                    c.bus.write8(l + 1, 0);
                    c.bus.write32(
                        hud + LINE_WIDTH + LINE_STRIDE * i,
                        f(uf(&mut r, 0.0, 200.0)),
                    );
                    c.bus.write32(
                        hud + LINE_HEIGHT + LINE_STRIDE * i,
                        f(uf(&mut r, 5.0, 30.0)),
                    );
                }
                c.bus.write32(hud + TEXT_B, text_b);
                c.bus.write32(hud + TEXT_A, text_a);
                for t in [text_a, text_b] {
                    for k in 0..10 {
                        c.bus.write32(t + 0x18 + 4 * k, f(uf(&mut r, 0.0, 2.0)));
                    }
                    c.bus.write32(t + 0x60, r.next_u32());
                }
                for i in 0..3 {
                    let s = sprites + 0x40 * i;
                    c.bus.write32(hud + SPRITES + 0x50 + 4 * i, s);
                    c.bus.write32(s, r.next_u32());
                    for k in 0..4 {
                        c.bus.write32(s + 0xc + 4 * k, f(uf(&mut r, 0.0, 1.0)));
                    }
                }
                c.g[A0][0] = sx(hud);
                c.g[A1][0] = sx(elems);
                c.g[A2][0] = if seed % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64 | 1
                };
            },
        );
    }
}
