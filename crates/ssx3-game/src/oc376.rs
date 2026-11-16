//! Staging module oc376 (wave wave217): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc376`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0039_8a78, ui_icon_render, "UIIcon_Render")];

// ---------------------------------------------------------------------------
// UIIcon_Render (0x398a78)
// ---------------------------------------------------------------------------

/// `GrMan` (the UI render context) at `gp - 0x854`.
pub(crate) const UI_ICON_RENDER_GRMAN: u32 = 0x854;
/// GrMan: the current render-state record (0x14 bytes each, a stack).
pub(crate) const UI_ICON_RENDER_CTX_TOP: u32 = 0xe84;
/// GrMan: its vtable (slot at +0x238: draw sprite quad (n, verts, 0)).
pub(crate) const UI_ICON_RENDER_CTX_VT: u32 = 0x10d8;
/// Icon fields.
pub(crate) const UI_ICON_RENDER_VT: u32 = 0x8;
pub(crate) const UI_ICON_RENDER_BLEND: u32 = 0x15;
pub(crate) const UI_ICON_RENDER_COLOR: u32 = 0x1c;
pub(crate) const UI_ICON_RENDER_ROT: u32 = 0x2c;
pub(crate) const UI_ICON_RENDER_POS: u32 = 0x44;
pub(crate) const UI_ICON_RENDER_SCALE: u32 = 0x50;
pub(crate) const UI_ICON_RENDER_FLAGS: u32 = 0x74;
pub(crate) const UI_ICON_RENDER_TEX_ID: u32 = 0x78;
pub(crate) const UI_ICON_RENDER_TEX: u32 = 0x7c;
/// The 3x3 base matrix the rotations start from (`D_004FF170`).
pub(crate) const UI_ICON_RENDER_BASE_MATRIX: u32 = 0x004f_f170;

/// `sp + off` (the frame: four 0x30-byte vertices at 0, the position at
/// 0xc0, the size at 0xd0, the scale at 0xe0, the matrix at 0xf0).
#[inline(always)]
fn ui_icon_render_sp(c: &Cpu, off: u32) -> u32 {
    sp(c).wrapping_add(off)
}

/// `s0 + off` (the icon).
#[inline(always)]
fn ui_icon_render_s0(c: &Cpu, off: u32) -> u32 {
    (c.g[S0][0] as u32).wrapping_add(off)
}

/// One of the three optional axis rotations of [`ui_icon_render`]
/// (`Rotate_8A78`): the per-axis stack slots and scratch registers the
/// compiled code uses.
struct UiIconRenderAxis {
    /// Which component of the unit axis at sp+0x120 is 1.
    axis: u32,
    /// Math_SinCos outputs.
    sin: u32,
    cos: u32,
    /// The rotation matrix r.
    rot: u32,
    /// The copy of m.
    copy: u32,
    /// GPRs carrying m's four doublewords, and the one for its last word.
    gprs: [usize; 4],
    word: usize,
    /// Math_SinCos's return address.
    ret: u32,
}

/// m = r * m for a rotation of angle `f12` about one unit axis (the block
/// after the `c.eq.s f12, f27` test; `f27` = 0.0). Charges the block's 138
/// cycles.
fn ui_icon_render_rotate(c: &mut Cpu, a: &UiIconRenderAxis) -> Flow {
    let (sin, cos, rot, copy, word) = (a.sin, a.cos, a.rot, a.copy, a.word);
    let [g0, g1, g2, g3] = a.gprs;
    c.cycles += 138;
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 20); // f20 = 1.0
    c.g[A0][0] = sx(ui_icon_render_sp(c, sin));
    c.g[A1][0] = sx(ui_icon_render_sp(c, cos));
    for i in 0..3 {
        let f = if i == a.axis { 20 } else { 27 };
        c.swc1(f, ui_icon_render_sp(c, 0x120 + 4 * i));
    }
    call_ptr(c, 0x0031_be50, a.ret)?; // Math_SinCos(&sin, &cos, angle)

    // r = the rotation matrix, then copy m, then m = r * copy.
    c.lwc1(10, ui_icon_render_sp(c, cos));
    c.lwc1(0, ui_icon_render_sp(c, 0x120));
    c.sub_s(20, 20, 10);
    c.lwc1(2, ui_icon_render_sp(c, 0x128));
    c.lwc1(4, ui_icon_render_sp(c, 0x124));
    c.lwc1(6, ui_icon_render_sp(c, sin));
    c.mul_s(1, 20, 2);
    c.mul_s(3, 20, 0);
    c.mul_s(20, 20, 4);
    c.mul_s(11, 6, 2);
    c.mul_s(12, 6, 0);
    c.mul_s(8, 1, 2);
    c.mul_s(9, 3, 2);
    c.mul_s(5, 1, 0);
    c.mul_s(7, 3, 0);
    c.mul_s(2, 20, 2);
    c.mul_s(0, 20, 0);
    c.mul_s(6, 6, 4);
    c.mul_s(1, 1, 4);
    c.mul_s(3, 3, 4);
    c.mul_s(20, 20, 4);
    c.add_s(5, 5, 6);
    c.sub_s(0, 0, 11);
    c.sub_s(1, 1, 12);
    c.add_s(8, 8, 10);
    c.swc1(5, ui_icon_render_sp(c, rot + 0x8));
    c.add_s(7, 7, 10);
    c.swc1(0, ui_icon_render_sp(c, rot + 0x4));
    c.add_s(3, 3, 11);
    c.swc1(1, ui_icon_render_sp(c, rot + 0x14));
    c.sub_s(9, 9, 6);
    c.swc1(8, ui_icon_render_sp(c, rot + 0x20));
    c.add_s(20, 20, 10);
    c.swc1(7, ui_icon_render_sp(c, rot));
    c.add_s(2, 2, 12);
    c.swc1(3, ui_icon_render_sp(c, rot + 0xc));
    c.swc1(9, ui_icon_render_sp(c, rot + 0x18));
    c.swc1(20, ui_icon_render_sp(c, rot + 0x10));
    c.swc1(2, ui_icon_render_sp(c, rot + 0x1c));
    c.ldl(g0, ui_icon_render_sp(c, 0xf7));
    c.ldr(g0, ui_icon_render_sp(c, 0xf0));
    c.ldl(g1, ui_icon_render_sp(c, 0xff));
    c.ldr(g1, ui_icon_render_sp(c, 0xf8));
    c.ldl(g2, ui_icon_render_sp(c, 0x107));
    c.ldr(g2, ui_icon_render_sp(c, 0x100));
    c.ldl(g3, ui_icon_render_sp(c, 0x10f));
    c.ldr(g3, ui_icon_render_sp(c, 0x108));
    c.sdl(g0, ui_icon_render_sp(c, copy + 0x7));
    c.sdr(g0, ui_icon_render_sp(c, copy));
    c.sdl(g1, ui_icon_render_sp(c, copy + 0xf));
    c.sdr(g1, ui_icon_render_sp(c, copy + 0x8));
    c.sdl(g2, ui_icon_render_sp(c, copy + 0x17));
    c.sdr(g2, ui_icon_render_sp(c, copy + 0x10));
    c.sdl(g3, ui_icon_render_sp(c, copy + 0x1f));
    c.sdr(g3, ui_icon_render_sp(c, copy + 0x18));
    c.g[word][0] = c.lw(ui_icon_render_sp(c, 0x110));
    c.sw(ui_icon_render_sp(c, copy + 0x20), c.g[word][0]);
    c.lwc1(0, ui_icon_render_sp(c, copy));
    c.lwc1(7, ui_icon_render_sp(c, rot));
    c.lwc1(2, ui_icon_render_sp(c, copy + 0xc));
    c.lwc1(16, ui_icon_render_sp(c, rot + 0x4));
    c.mul_s(9, 0, 7);
    c.lwc1(19, ui_icon_render_sp(c, rot + 0xc));
    c.lwc1(20, ui_icon_render_sp(c, rot + 0x10));
    c.mul_s(12, 2, 16);
    c.lwc1(6, ui_icon_render_sp(c, rot + 0x18));
    c.mul_s(8, 0, 19);
    c.lwc1(15, ui_icon_render_sp(c, rot + 0x1c));
    c.mul_s(5, 2, 20);
    c.mul_s(0, 0, 6);
    c.lwc1(3, ui_icon_render_sp(c, copy + 0x4));
    c.mul_s(2, 2, 15);
    c.lwc1(10, ui_icon_render_sp(c, copy + 0x10));
    c.lwc1(1, ui_icon_render_sp(c, copy + 0x18));
    c.mul_s(4, 3, 7);
    c.lwc1(14, ui_icon_render_sp(c, rot + 0x8));
    c.mul_s(17, 10, 16);
    c.lwc1(11, ui_icon_render_sp(c, rot + 0x20));
    c.add_s(0, 0, 2);
    c.lwc1(18, ui_icon_render_sp(c, rot + 0x14));
    c.mul_s(2, 1, 14);
    c.mul_s(13, 1, 11);
    c.add_s(9, 9, 12);
    c.lwc1(12, ui_icon_render_sp(c, copy + 0x1c));
    c.add_s(8, 8, 5);
    c.mul_s(1, 1, 18);
    c.mul_s(5, 12, 14);
    c.add_s(4, 4, 17);
    c.add_s(9, 9, 2);
    c.add_s(8, 8, 1);
    c.add_s(0, 0, 13);
    c.add_s(4, 4, 5);
    c.swc1(9, ui_icon_render_sp(c, 0xf0));
    c.swc1(8, ui_icon_render_sp(c, 0xfc));
    c.mul_s(2, 10, 15);
    c.swc1(0, ui_icon_render_sp(c, 0x108));
    c.mul_s(5, 3, 6);
    c.swc1(4, ui_icon_render_sp(c, 0xf4));
    c.mul_s(3, 3, 19);
    c.lwc1(1, ui_icon_render_sp(c, copy + 0x8));
    c.mul_s(10, 10, 20);
    c.lwc1(0, ui_icon_render_sp(c, copy + 0x14));
    c.add_s(5, 5, 2);
    c.mul_s(6, 1, 6);
    c.lwc1(2, ui_icon_render_sp(c, copy + 0x20));
    c.mul_s(15, 0, 15);
    c.mul_s(7, 1, 7);
    c.mul_s(16, 0, 16);
    c.mul_s(0, 0, 20);
    c.mul_s(1, 1, 19);
    c.mul_s(14, 2, 14);
    c.add_s(3, 3, 10);
    c.add_s(1, 1, 0);
    c.mul_s(0, 2, 11);
    c.mul_s(11, 12, 11);
    c.add_s(7, 7, 16);
    c.add_s(6, 6, 15);
    c.mul_s(2, 2, 18);
    c.mul_s(12, 12, 18);
    c.add_s(5, 5, 11);
    c.add_s(7, 7, 14);
    c.add_s(3, 3, 12);
    c.add_s(1, 1, 2);
    c.swc1(5, ui_icon_render_sp(c, 0x10c));
    c.add_s(6, 6, 0);
    c.swc1(7, ui_icon_render_sp(c, 0xf8));
    c.swc1(3, ui_icon_render_sp(c, 0x100));
    c.swc1(1, ui_icon_render_sp(c, 0x104));
    c.swc1(6, ui_icon_render_sp(c, 0x110));
    Ok(())
}

/// `void UIIcon_Render(cUIIcon* self, float x, float y)` (0x398a78,
/// `cUIIcon_render2D`): draws the icon as one textured quad at (x, y).
/// Picks the texture (the texture page entry's uv rectangle, else the whole
/// texture `texId`, else untextured), mirrors the uvs (flag 2) and turns
/// them by quarter turns (flags bits 2-3), sizes the quad (the size getter
/// times the scale, aligned by `UIObject_ApplyAlignment`), rotates it about
/// the icon's position by the x/y/z angles, sets the colour (alpha, r, g,
/// b times 128), pushes a render state with the blend mode from +0x15 and
/// calls the context's sprite drawer with the four vertices.
pub fn ui_icon_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x2a0);
        let gp = c.g[GP][0] as u32;
        c.g[A1][0] = c.lw(gp.wrapping_sub(UI_ICON_RENDER_GRMAN));
        frame.save(c, S0, 0x240);
        c.g[V0][0] = 0x3;
        frame.save(c, S1, 0x230);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x220);
        c.g[S1][0] = sx(ui_icon_render_sp(c, 0xd0)); // &size
        frame.save_fpr(c, 21, 0x258);
        c.g[S2][0] = sx(ui_icon_render_sp(c, 0xc0)); // &pos
        frame.save_fpr(c, 20, 0x250);
        c.mov_s(21, 13); // y
        frame.save_ra(c, 0x210);
        c.mov_s(20, 12); // x
        for (f, off) in [
            (28, 0x290),
            (27, 0x288),
            (26, 0x280),
            (25, 0x278),
            (24, 0x270),
            (23, 0x268),
            (22, 0x260),
        ] {
            frame.save_fpr(c, f, off);
        }
        c.g[V1][0] = u64::MAX;
        c.g[A0][0] = c.lw(ui_icon_render_s0(c, UI_ICON_RENDER_TEX));

        // The texture and the vertices' uvs (vertex i at sp + 0x30*i: u, v).
        if c.g[A0][0] != 0 {
            // A texture page entry: id, then the uv rectangle (v0 0xc, u0
            // 0x10, u1 0x14, v1 0x18).
            c.cycles += 21;
            c.g[A0][0] = c.lw(c.g[A0][0] as u32);
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(UI_ICON_RENDER_CTX_TOP));
            c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
            c.g[V1][0] = c.lw(ui_icon_render_s0(c, UI_ICON_RENDER_TEX));
            for (f, src, dst) in [
                (0, 0x10, 0x0),
                (1, 0xc, 0x4),
                (0, 0x14, 0x30),
                (1, 0xc, 0x34),
                (0, 0x10, 0x60),
                (1, 0x18, 0x64),
                (0, 0x14, 0x90),
                (1, 0x18, 0x94),
            ] {
                c.lwc1(f, (c.g[V1][0] as u32).wrapping_add(src));
                c.swc1(f, ui_icon_render_sp(c, dst));
            }
        } else {
            c.g[V1][0] = c.lw(ui_icon_render_s0(c, UI_ICON_RENDER_TEX_ID));
            c.cycles += 2;
            if (c.g[V1][0] as i64) <= 0 {
                // Untextured.
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(UI_ICON_RENDER_CTX_TOP));
                c.cycles += 2;
                c.g[V0][0] = u64::MAX;
                c.sh((c.g[V1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            } else {
                // The whole texture: uvs (0,0) (1,0) (0,1) (1,1).
                c.cycles += 13;
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(UI_ICON_RENDER_CTX_TOP));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.swc1(0, ui_icon_render_sp(c, 0x94));
                c.sw(ui_icon_render_sp(c, 0x0), 0);
                c.sw(ui_icon_render_sp(c, 0x4), 0);
                c.swc1(0, ui_icon_render_sp(c, 0x30));
                c.sw(ui_icon_render_sp(c, 0x34), 0);
                c.sw(ui_icon_render_sp(c, 0x60), 0);
                c.swc1(0, ui_icon_render_sp(c, 0x64));
                c.swc1(0, ui_icon_render_sp(c, 0x90));
            }
        }

        // Flag 2: mirror (swap u0/u1, v0/v1, u2/u3, v2/v3).
        c.cycles += 4;
        c.g[V0][0] = c.lw(ui_icon_render_s0(c, UI_ICON_RENDER_FLAGS)) & 0x2;
        c.g[A0][0] = c.g[SP][0];
        if c.g[V0][0] != 0 {
            c.cycles += 11;
            let swap = Callee {
                addr: 0x0039_8a60,
                f: ee::s13::UIIcon_SwapWords,
            };
            c.g[A1][0] = sx(ui_icon_render_sp(c, 0x30));
            swap.call(c, 0x0039_8b8c)?;
            c.g[A0][0] = c.g[SP][0] | 0x4;
            c.g[A1][0] = sx(ui_icon_render_sp(c, 0x34));
            swap.call(c, 0x0039_8b98)?;
            c.g[A0][0] = sx(ui_icon_render_sp(c, 0x60));
            c.g[A1][0] = sx(ui_icon_render_sp(c, 0x90));
            swap.call(c, 0x0039_8ba4)?;
            c.g[A0][0] = sx(ui_icon_render_sp(c, 0x64));
            c.g[A1][0] = sx(ui_icon_render_sp(c, 0x94));
            swap.call(c, 0x0039_8bb0)?;
        }

        // Flags bits 2-3: quarter turns of the uvs (0 <- 2 <- 3 <- 1 <- 0),
        // the counter kept in the top byte of v1.
        c.cycles += 5;
        c.g[V0][0] = sx(c.lw(ui_icon_render_s0(c, UI_ICON_RENDER_FLAGS)) as u32 >> 2) & 0x3;
        c.lwc1(8, ui_icon_render_sp(c, 0x0));
        if c.g[V0][0] != 0 {
            c.cycles += 11;
            c.g[A0][0] = c.g[V0][0];
            c.lwc1(7, ui_icon_render_sp(c, 0x60));
            c.g[V1][0] = 0x1000000;
            c.lwc1(6, ui_icon_render_sp(c, 0x90));
            c.g[A1][0] = 0x1000000;
            c.lwc1(5, ui_icon_render_sp(c, 0x30));
            c.lwc1(4, ui_icon_render_sp(c, 0x4));
            c.lwc1(3, ui_icon_render_sp(c, 0x64));
            c.lwc1(2, ui_icon_render_sp(c, 0x94));
            c.lwc1(1, ui_icon_render_sp(c, 0x34));
            loop {
                c.cycles += 15;
                c.mov_s(0, 8);
                c.mov_s(8, 7);
                c.mov_s(7, 6);
                c.mov_s(6, 5);
                let i = (c.g[V1][0] as u32 as i32) >> 24;
                c.g[V0][0] = ((sx(i as u32) as i64) < (c.g[A0][0] as i64)) as u64;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.mov_s(5, 0);
                c.mov_s(0, 4);
                c.mov_s(4, 3);
                c.mov_s(3, 2);
                c.mov_s(2, 1);
                c.mov_s(1, 0);
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_8bf0);
                }
            }
            c.cycles += 8;
            for (f, off) in [
                (1, 0x34),
                (2, 0x94),
                (3, 0x64),
                (4, 0x4),
                (5, 0x30),
                (6, 0x90),
                (7, 0x60),
                (8, 0x0),
            ] {
                c.swc1(f, ui_icon_render_sp(c, off));
            }
        }

        // pos = self->pos, scale = self->scale, size = getSize() * scale,
        // aligned; pos += (x, y); m = the base matrix.
        c.cycles += 61;
        c.ldl(2, ui_icon_render_s0(c, UI_ICON_RENDER_POS + 7));
        c.ldr(2, ui_icon_render_s0(c, UI_ICON_RENDER_POS));
        c.g[V1][0] = c.lw(ui_icon_render_s0(c, UI_ICON_RENDER_POS + 8));
        c.sdl(2, ui_icon_render_sp(c, 0xc7));
        c.sdr(2, ui_icon_render_sp(c, 0xc0));
        c.sw(ui_icon_render_sp(c, 0xc8), c.g[V1][0]);
        c.g[V1][0] = c.lw(ui_icon_render_s0(c, UI_ICON_RENDER_VT));
        c.g[A1][0] = c.g[S1][0];
        c.ldl(6, ui_icon_render_s0(c, UI_ICON_RENDER_SCALE + 7));
        c.ldr(6, ui_icon_render_s0(c, UI_ICON_RENDER_SCALE));
        c.g[A3][0] = c.lw(ui_icon_render_s0(c, UI_ICON_RENDER_SCALE + 8));
        c.sdl(6, ui_icon_render_sp(c, 0xe7));
        c.sdr(6, ui_icon_render_sp(c, 0xe0));
        c.sw(ui_icon_render_sp(c, 0xe8), c.g[A3][0]);
        c.mtc1(0, 27);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let get_size = c.g[V0][0] as u32;
        call_ptr(c, get_size, 0x0039_8c98)?; // self->getSize(&size)
        c.lwc1(2, ui_icon_render_sp(c, 0xd0));
        c.g[A1][0] = c.g[S2][0];
        c.lwc1(0, ui_icon_render_sp(c, 0xd4));
        c.g[A0][0] = c.g[S0][0];
        c.lwc1(1, ui_icon_render_sp(c, 0xe4));
        c.g[A2][0] = c.g[S1][0];
        c.lwc1(3, ui_icon_render_sp(c, 0xe0));
        c.mul_s(0, 0, 1);
        c.mul_s(2, 2, 3);
        c.swc1(0, ui_icon_render_sp(c, 0xd4));
        c.swc1(2, ui_icon_render_sp(c, 0xd0));
        Callee {
            addr: 0x0039_feb8,
            f: ee::s13::UIObject_ApplyAlignment,
        }
        .call(c, 0x0039_8cc8)?; // UIObject_ApplyAlignment(self, &pos, &size)
        c.lwc1(0, ui_icon_render_sp(c, 0xc0));
        c.g[V0][0] = 0x500000;
        c.lwc1(1, ui_icon_render_sp(c, 0xc4));
        c.lwc1(12, ui_icon_render_s0(c, UI_ICON_RENDER_ROT));
        c.add_s(0, 0, 20);
        c.add_s(1, 1, 21);
        c.c_eq_s(12, 27);
        c.swc1(0, ui_icon_render_sp(c, 0xc0));
        c.swc1(1, ui_icon_render_sp(c, 0xc4));
        c.g[A1][0] = sx(UI_ICON_RENDER_BASE_MATRIX); // 0x500000 - 0xe90
        c.ldl(11, (c.g[A1][0] as u32).wrapping_add(0x7));
        c.ldr(11, c.g[A1][0] as u32);
        c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0xf));
        c.ldr(3, (c.g[A1][0] as u32).wrapping_add(0x8));
        c.ldl(4, (c.g[A1][0] as u32).wrapping_add(0x17));
        c.ldr(4, (c.g[A1][0] as u32).wrapping_add(0x10));
        c.sdl(11, ui_icon_render_sp(c, 0xf7));
        c.sdr(11, ui_icon_render_sp(c, 0xf0));
        c.sdl(3, ui_icon_render_sp(c, 0xff));
        c.sdr(3, ui_icon_render_sp(c, 0xf8));
        c.sdl(4, ui_icon_render_sp(c, 0x107));
        c.sdr(4, ui_icon_render_sp(c, 0x100));
        c.ldl(11, (c.g[A1][0] as u32).wrapping_add(0x1f));
        c.ldr(11, (c.g[A1][0] as u32).wrapping_add(0x18));
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x20));
        c.sdl(11, ui_icon_render_sp(c, 0x10f));
        c.sdr(11, ui_icon_render_sp(c, 0x108));
        c.sw(ui_icon_render_sp(c, 0x110), c.g[V1][0]);

        // The x, y and z rotations (each only for a non-zero angle).
        if !c.fcc() {
            ui_icon_render_rotate(
                c,
                &UiIconRenderAxis {
                    axis: 0,
                    sin: 0x1f0,
                    cos: 0x1f4,
                    rot: 0x130,
                    copy: 0x160,
                    gprs: [A2, A3, T0, T1],
                    word: A2,
                    ret: 0x0039_8d60,
                },
            )?;
        }
        c.lwc1(12, ui_icon_render_s0(c, UI_ICON_RENDER_ROT + 4));
        c.cycles += 4;
        c.c_eq_s(12, 27);
        if !c.fcc() {
            ui_icon_render_rotate(
                c,
                &UiIconRenderAxis {
                    axis: 1,
                    sin: 0x1f8,
                    cos: 0x1fc,
                    rot: 0x190,
                    copy: 0x1c0,
                    gprs: [T3, V0, V1, A0],
                    word: T3,
                    ret: 0x0039_8f98,
                },
            )?;
        }
        c.lwc1(12, ui_icon_render_s0(c, UI_ICON_RENDER_ROT + 8));
        c.cycles += 4;
        c.c_eq_s(12, 27);
        if !c.fcc() {
            ui_icon_render_rotate(
                c,
                &UiIconRenderAxis {
                    axis: 2,
                    sin: 0x200,
                    cos: 0x204,
                    rot: 0x130,
                    copy: 0x160,
                    gprs: [A1, A2, A3, T0],
                    word: A1,
                    ret: 0x0039_91d0,
                },
            )?;
        }
        c.lwc1(26, ui_icon_render_s0(c, UI_ICON_RENDER_POS));

        // The corners (pos - self->pos and pos + size - self->pos) through
        // the upper two rows of m, w = 1, then + self->pos with VU0.
        c.cycles += 146;
        c.g[A0][0] = sx(ui_icon_render_sp(c, 0x20));
        c.lwc1(9, ui_icon_render_sp(c, 0xc0));
        c.g[A3][0] = 0x3;
        c.lwc1(25, ui_icon_render_s0(c, UI_ICON_RENDER_POS + 4));
        c.lwc1(10, ui_icon_render_sp(c, 0xc4));
        c.sub_s(5, 9, 26);
        c.lwc1(0, ui_icon_render_sp(c, 0xd0));
        c.sub_s(8, 10, 25);
        c.lwc1(4, ui_icon_render_sp(c, 0xf0));
        c.add_s(9, 9, 0);
        c.lwc1(19, ui_icon_render_sp(c, 0xfc));
        c.lwc1(2, ui_icon_render_sp(c, 0xf4));
        c.mul_s(23, 4, 5);
        c.lwc1(17, ui_icon_render_sp(c, 0x100));
        c.mul_s(14, 19, 8);
        c.lwc1(1, ui_icon_render_sp(c, 0xf8));
        c.sub_s(20, 9, 26);
        c.lwc1(15, ui_icon_render_sp(c, 0x104));
        c.mul_s(22, 2, 5);
        c.lwc1(0, ui_icon_render_sp(c, 0xd4));
        c.mul_s(12, 17, 8);
        c.mul_s(21, 1, 5);
        c.lwc1(13, ui_icon_render_sp(c, 0xc8));
        c.mul_s(11, 15, 8);
        c.lwc1(3, ui_icon_render_sp(c, 0xd8));
        c.add_s(10, 10, 0);
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 18);
        c.add_s(0, 23, 14);
        c.lwc1(24, ui_icon_render_s0(c, UI_ICON_RENDER_POS + 8));
        c.add_s(6, 22, 12);
        c.swc1(18, ui_icon_render_sp(c, 0x16c));
        c.sub_s(16, 10, 25);
        c.swc1(5, ui_icon_render_sp(c, 0x120));
        c.add_s(7, 21, 11);
        c.swc1(0, ui_icon_render_sp(c, 0x160));
        c.mul_s(4, 4, 20);
        c.swc1(6, ui_icon_render_sp(c, 0x164));
        c.mul_s(2, 2, 20);
        c.swc1(8, ui_icon_render_sp(c, 0x124));
        c.mul_s(1, 1, 20);
        c.swc1(7, ui_icon_render_sp(c, 0x168));
        c.add_s(3, 13, 3);
        c.swc1(9, ui_icon_render_sp(c, 0x140));
        c.mul_s(19, 19, 16);
        c.lq(2, ui_icon_render_sp(c, 0x160));
        c.add_s(14, 4, 14);
        c.swc1(10, ui_icon_render_sp(c, 0x144));
        c.add_s(12, 2, 12);
        c.sq(2, ui_icon_render_sp(c, 0x20));
        c.add_s(11, 1, 11);
        c.swc1(3, ui_icon_render_sp(c, 0x148));
        c.mul_s(17, 17, 16);
        c.swc1(14, ui_icon_render_sp(c, 0x160));
        c.mul_s(15, 15, 16);
        c.swc1(12, ui_icon_render_sp(c, 0x164));
        c.sub_s(0, 3, 24);
        c.swc1(11, ui_icon_render_sp(c, 0x168));
        c.sub_s(13, 13, 24);
        c.swc1(20, ui_icon_render_sp(c, 0x130));
        c.add_s(23, 23, 19);
        c.lq(2, ui_icon_render_sp(c, 0x160));
        c.add_s(22, 22, 17);
        c.swc1(0, ui_icon_render_sp(c, 0x138));
        c.add_s(21, 21, 15);
        c.swc1(13, ui_icon_render_sp(c, 0x128));
        c.swc1(16, ui_icon_render_sp(c, 0x134));
        c.add_s(4, 4, 19);
        c.swc1(23, ui_icon_render_sp(c, 0x160));
        c.add_s(2, 2, 17);
        c.swc1(22, ui_icon_render_sp(c, 0x164));
        c.add_s(1, 1, 15);
        c.swc1(21, ui_icon_render_sp(c, 0x168));
        c.mov_s(28, 18);
        c.sq(2, ui_icon_render_sp(c, 0x50));
        c.lq(3, ui_icon_render_sp(c, 0x160));
        c.swc1(4, ui_icon_render_sp(c, 0x160));
        c.swc1(2, ui_icon_render_sp(c, 0x164));
        c.swc1(1, ui_icon_render_sp(c, 0x168));
        c.sq(3, ui_icon_render_sp(c, 0x80));
        c.lq(2, ui_icon_render_sp(c, 0x160));
        c.swc1(20, ui_icon_render_sp(c, 0x150));
        c.sq(2, ui_icon_render_sp(c, 0x140));
        c.sq(2, ui_icon_render_sp(c, 0xb0));
        c.swc1(26, ui_icon_render_sp(c, 0x140));
        c.swc1(25, ui_icon_render_sp(c, 0x144));
        c.swc1(24, ui_icon_render_sp(c, 0x148));
        c.swc1(16, ui_icon_render_sp(c, 0x154));
        c.swc1(27, ui_icon_render_sp(c, 0x158));
        c.swc1(18, ui_icon_render_sp(c, 0x15c));
        c.swc1(27, ui_icon_render_sp(c, 0x14c));
        c.lqc2(3, ui_icon_render_sp(c, 0x20));
        c.lqc2(4, ui_icon_render_sp(c, 0x140));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, ui_icon_render_sp(c, 0x20));
        c.g[AT][0] = 0x43000000;
        c.mtc1(1, 4); // 128.0
        for vert in [0x50, 0x80, 0xb0] {
            c.lwc1(0, ui_icon_render_s0(c, UI_ICON_RENDER_POS));
            c.lwc1(1, ui_icon_render_s0(c, UI_ICON_RENDER_POS + 4));
            c.lwc1(2, ui_icon_render_s0(c, UI_ICON_RENDER_POS + 8));
            c.swc1(0, ui_icon_render_sp(c, 0x140));
            c.swc1(1, ui_icon_render_sp(c, 0x144));
            c.swc1(2, ui_icon_render_sp(c, 0x148));
            c.swc1(27, ui_icon_render_sp(c, 0x14c));
            c.lqc2(3, ui_icon_render_sp(c, vert));
            c.lqc2(4, ui_icon_render_sp(c, 0x140));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, ui_icon_render_sp(c, vert));
        }

        // The colour (a, r, g, b) * 128 as integers.
        c.lwc1(1, ui_icon_render_s0(c, UI_ICON_RENDER_COLOR));
        c.lwc1(2, ui_icon_render_s0(c, UI_ICON_RENDER_COLOR + 4));
        c.lwc1(3, ui_icon_render_s0(c, UI_ICON_RENDER_COLOR + 8));
        c.mul_s(1, 1, 4);
        c.lwc1(0, ui_icon_render_s0(c, UI_ICON_RENDER_COLOR + 0xc));
        c.mul_s(2, 2, 4);
        c.mul_s(3, 3, 4);
        c.mul_s(0, 0, 4);
        c.cvt_w_s(4, 1);
        c.mfc1(6, 4);
        c.cvt_w_s(1, 2);
        c.mfc1(5, 1);
        c.cvt_w_s(1, 3);
        c.mfc1(3, 1);
        c.cvt_w_s(1, 0);
        c.mfc1(2, 1);

        // Each vertex: pos.z = 0, colour, q = 1.
        loop {
            c.cycles += 9;
            let v = c.g[A0][0] as u32; // &verts[i].pos
            c.sw(v.wrapping_add(0x8), 0);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
            c.sw(v.wrapping_sub(0x4), c.g[A2][0]);
            c.sw(v.wrapping_sub(0x10), c.g[A1][0]);
            c.sw(v.wrapping_sub(0xc), c.g[V1][0]);
            c.sw(v.wrapping_sub(0x8), c.g[V0][0]);
            c.swc1(28, v.wrapping_sub(0x18));
            c.g[A0][0] = sx(v.wrapping_add(0x30));
            if (c.g[A3][0] as i64) < 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0039_9620);
            }
        }

        // Push the render state (blend mode (blend & 0x1f) + 9 into bits
        // 5-9 of +8, flags |= 0xc), draw the quad, pop.
        c.cycles += 58;
        let gp = c.g[GP][0] as u32;
        c.g[T0][0] = c.lw(gp.wrapping_sub(UI_ICON_RENDER_GRMAN));
        c.g[T2][0] = 0xfffffffffffffc1f;
        c.g[A1][0] = 0x4;
        c.g[A2][0] = c.g[SP][0];
        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(UI_ICON_RENDER_CTX_TOP));
        c.g[A3][0] = 0;
        c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
        c.ldr(2, c.g[V1][0] as u32);
        c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
        c.ldr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[T1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
        c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x1b));
        c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0x14));
        c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0x23));
        c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x1c));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x24), c.g[T1][0]);
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(UI_ICON_RENDER_CTX_TOP));
        c.g[T1][0] = c.lw(gp.wrapping_sub(UI_ICON_RENDER_GRMAN));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
        c.sw(
            (c.g[T0][0] as u32).wrapping_add(UI_ICON_RENDER_CTX_TOP),
            c.g[V0][0],
        );
        c.g[A0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(UI_ICON_RENDER_CTX_TOP));
        c.g[V0][0] = c.lw(c.g[A0][0] as u32) | 0xc;
        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
        c.g[V1][0] = c.lbu(ui_icon_render_s0(c, UI_ICON_RENDER_BLEND));
        c.g[A0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(UI_ICON_RENDER_CTX_TOP));
        c.g[V1][0] &= 0x1f;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x9));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
        c.g[V0][0] &= c.g[T2][0];
        c.g[V1][0] &= 0x3e0;
        c.g[V0][0] |= c.g[V1][0];
        c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[T1][0] as u32).wrapping_add(UI_ICON_RENDER_CTX_VT));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x238));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x23c));
        let draw = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, draw, 0x0039_96e0)?; // ctx->drawSprites(4, verts, 0)

        // Pop the render state, epilogue.
        let gp = c.g[GP][0] as u32;
        c.g[V1][0] = c.lw(gp.wrapping_sub(UI_ICON_RENDER_GRMAN));
        frame.restore(c, S0, 0x240);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(UI_ICON_RENDER_CTX_TOP));
        frame.restore(c, S1, 0x230);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
        frame.restore(c, S2, 0x220);
        frame.restore_ra(c, 0x210);
        for (f, off) in [
            (28, 0x290),
            (27, 0x288),
            (26, 0x280),
            (25, 0x278),
            (24, 0x270),
            (23, 0x268),
            (22, 0x260),
            (21, 0x258),
            (20, 0x250),
        ] {
            frame.restore_fpr(c, f, off);
        }
        c.sw(
            (c.g[V1][0] as u32).wrapping_add(UI_ICON_RENDER_CTX_TOP),
            c.g[V0][0],
        );
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fill, put_words, scramble, Rng, DATA, GAME_GP};

    /// Fixture layout: the icon, GrMan, its render-state stack and vtable,
    /// the icon's vtable and a texture page entry.
    const UI_ICON_RENDER_T_ICON: u32 = DATA;
    const UI_ICON_RENDER_T_CTX: u32 = DATA + 0x1000;
    const UI_ICON_RENDER_T_STATES: u32 = DATA + 0x3000;
    const UI_ICON_RENDER_T_CTX_VT: u32 = DATA + 0x4000;
    const UI_ICON_RENDER_T_ICON_VT: u32 = DATA + 0x4800;
    const UI_ICON_RENDER_T_TEX: u32 = DATA + 0x5000;

    /// Every path: textured by a page entry / by texture id / untextured,
    /// mirrored or not, 0-3 quarter turns, each axis rotation on or off.
    /// The size getter is the real `UIObject_GetVec3At60` (0x3a4c90), the
    /// drawer the real leaf at 0x1406c0.
    #[test]
    fn ui_icon_render_matches() {
        check(
            ee::s13::f_00398a78__generated,
            ui_icon_render,
            300,
            &|c, seed| {
                let mut r = Rng(0x0398_a780 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                scramble(c, &mut r);
                c.pc = 0x0039_8a78;
                c.g[GP][0] = GAME_GP as u64;
                // Math_SinCos's .lit4 constants.
                put_words(
                    c,
                    0x004a_0230,
                    &[
                        0x3f22f983, 0x3fc90fdb, 0x3638ef1f, 0xb9500d03, 0x3c088889, 0xbe2aaaab,
                    ],
                );
                let icon = UI_ICON_RENDER_T_ICON;
                c.g[A0][0] = sx(icon);
                c.f[12] = r.f32(400.0);
                c.f[13] = r.f32(400.0);

                // GrMan: the render-state stack (two records) and the drawer.
                let ctx = UI_ICON_RENDER_T_CTX;
                c.bus.write32(GAME_GP - UI_ICON_RENDER_GRMAN, ctx);
                c.bus
                    .write32(ctx + UI_ICON_RENDER_CTX_TOP, UI_ICON_RENDER_T_STATES);
                fill(c, &mut r, UI_ICON_RENDER_T_STATES, 10);
                c.bus
                    .write32(ctx + UI_ICON_RENDER_CTX_VT, UI_ICON_RENDER_T_CTX_VT);
                c.bus.write16(UI_ICON_RENDER_T_CTX_VT + 0x238, 0);
                c.bus.write32(UI_ICON_RENDER_T_CTX_VT + 0x23c, 0x0014_06c0);

                // The icon: vtable slot 20 = the size getter (size at +0x60).
                c.bus
                    .write32(icon + UI_ICON_RENDER_VT, UI_ICON_RENDER_T_ICON_VT);
                c.bus.write16(UI_ICON_RENDER_T_ICON_VT + 0xa0, 0);
                c.bus.write32(UI_ICON_RENDER_T_ICON_VT + 0xa4, 0x003a_4c90);
                c.bus.write32(icon + 0x14, r.next_u32()); // alignment bits, blend byte
                for i in 0..4 {
                    c.bus
                        .write32(icon + UI_ICON_RENDER_COLOR + 4 * i, r.f32(1.2));
                }
                for i in 0..3 {
                    let angle = if (seed >> (2 + i)) & 1 == 0 {
                        0
                    } else {
                        r.f32(7.0)
                    };
                    c.bus.write32(icon + UI_ICON_RENDER_ROT + 4 * i, angle);
                    c.bus
                        .write32(icon + UI_ICON_RENDER_POS + 4 * i, r.f32(300.0));
                    c.bus
                        .write32(icon + UI_ICON_RENDER_SCALE + 4 * i, r.f32(3.0));
                    c.bus.write32(icon + 0x60 + 4 * i, r.f32(100.0));
                }
                c.bus.write32(icon + UI_ICON_RENDER_FLAGS, r.next_u32());
                let tex_id = match seed % 3 {
                    0 => r.next_u32() & 0x7fff,
                    1 => 0,
                    _ => 0x8000_0000 | r.next_u32(),
                };
                c.bus.write32(icon + UI_ICON_RENDER_TEX_ID, tex_id);
                let tex = if seed % 4 == 0 {
                    UI_ICON_RENDER_T_TEX
                } else {
                    0
                };
                c.bus.write32(icon + UI_ICON_RENDER_TEX, tex);
                c.bus.write32(UI_ICON_RENDER_T_TEX, r.next_u32() & 0xffff);
                for off in [0xc, 0x10, 0x14, 0x18] {
                    c.bus.write32(UI_ICON_RENDER_T_TEX + off, r.f32(1.0));
                }

                // The base matrix: identity or random.
                for i in 0..9 {
                    let v = if seed % 5 < 2 {
                        if i % 4 == 0 {
                            0x3f80_0000
                        } else {
                            0
                        }
                    } else {
                        r.f32(2.0)
                    };
                    c.bus.write32(UI_ICON_RENDER_BASE_MATRIX + 4 * i, v);
                }
            },
        );
    }
}
