//! Staging module oc395 (wave wave217): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc395`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::game::render::particle::{add32, at, fconst, fr, gp, le, p, setf, shl32};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0021_d9a0, hud_draw_control_hint, "HUD_DrawControlHint")];

// ---------------------------------------------------------------------------
// HUD_DrawControlHint (0x21d9a0)

/// The HUD root's per-player block (`players[idx]`, stride 0x8c): the rider
/// index at +0x48.
pub(crate) const HUD_CONTROL_HINT_PLAYER_STRIDE: u32 = 0x8c;
pub(crate) const HUD_CONTROL_HINT_PLAYER_RIDER: u32 = 0x48;
/// The icon text slots of the two control maps.
pub(crate) const HUD_CONTROL_HINT_ICON_TEXT: u32 = 0x6ac;
pub(crate) const HUD_CONTROL_HINT_ICON_TEXT_ALT: u32 = 0x7f8;
/// The HUD's sprite table (`sprites[0x20]`): 0x10 the gauge end piece,
/// 0x11 the middle, 0x12/0x13 the button cap (control map 0 / other).
pub(crate) const HUD_CONTROL_HINT_SPRITES: u32 = 0x494;
pub(crate) const HUD_CONTROL_HINT_END_SPRITE: u32 = 0x4d4;
pub(crate) const HUD_CONTROL_HINT_MID_SPRITE: u32 = 0x4d8;
/// The layout elements: the icon (+0x7bc) and the gauge (+0x798).
pub(crate) const HUD_CONTROL_HINT_ICON_ELEM: u32 = 0x7bc;
pub(crate) const HUD_CONTROL_HINT_GAUGE_ELEM: u32 = 0x798;
/// The fill colour (`D_004C8668`, an `sColour` of 4 floats).
pub(crate) const HUD_CONTROL_HINT_FILL_COLOUR: u32 = 0x004c_8668;
/// gp-relative: the fill margins' "initialised" flag and the margins.
pub(crate) const HUD_CONTROL_HINT_MARGIN_INIT: i32 = 0x1da8;
pub(crate) const HUD_CONTROL_HINT_MARGIN_X: i32 = 0x2a38;
pub(crate) const HUD_CONTROL_HINT_MARGIN_Y: i32 = 0x2a3c;

#[inline(always)]
fn hint_mul(c: &mut Cpu, d: usize, a: usize, b: usize) {
    let v = fr(c, a) * fr(c, b);
    setf(c, d, v);
}

#[inline(always)]
fn hint_add(c: &mut Cpu, d: usize, a: usize, b: usize) {
    let v = fr(c, a) + fr(c, b);
    setf(c, d, v);
}

#[inline(always)]
fn hint_sub(c: &mut Cpu, d: usize, a: usize, b: usize) {
    let v = fr(c, a) - fr(c, b);
    setf(c, d, v);
}

/// The renderer's slot-71 draw (`renderer->vt[0x238]`, 4 vertices at a2):
/// `t0 = renderer`, `v1 = vtable`, `a0 = renderer + delta`, `v0 = method`.
#[inline(always)]
fn hint_draw_quad(c: &mut Cpu, ret_to: u32) -> Flow {
    c.g[T0][0] = c.lw(gp(c, -0x854));
    c.g[V1][0] = c.lw(at(c, T0, 0x10d8));
    c.g[A0][0] = c.lh(at(c, V1, 0x238));
    c.g[V0][0] = c.lw(at(c, V1, 0x23c));
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = add32(c.g[T0][0], c.g[A0][0]);
    call_ptr(c, target, ret_to)
}

/// `HUD_DrawControlHint(float fill (f12), float alpha (f13), HudRoot* hud,
/// char* elems, sGaugeBar* bar, int idx, int icon, float scale[2])`
/// (0x21d9a0): a horizontal gauge with a control hint. With `icon`, the
/// button icon's text (`HUD_DrawIconText`, the control map of rider `idx`
/// picks the slot); then the button cap (sprite 0x12 or 0x13, by control
/// map), the end piece, `bar->count > 0` a middle piece, the mirrored end
/// piece, and when `fill > 0` the fill rectangle (`HUD_DrawRect`) inside
/// lazily initialised 3-pixel margins.
///
/// Stack: a (pos) +0, sc +0x10, t (size) +0x20, pos copy +0x30, p +0x40,
/// v +0x50, four vertices +0x60 (stride 0x30: uv q, colour g b a r,
/// position at +0x20), position temp +0x120/+0x130, fill size +0x140,
/// colour +0x150, unit scale +0x160. Frame 0x240.
pub fn hud_draw_control_hint(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0x240);
        let s = sp(c);
        frame.save(c, S0, 0x200);
        frame.save(c, S3, 0x1d0);
        c.g[S0][0] = c.g[T0][0];
        frame.save(c, S4, 0x1c0);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S5, 0x1b0);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S6, 0x1a0);
        c.g[S5][0] = c.g[T1][0];
        frame.save(c, FP, 0x180);
        c.g[S6][0] = c.g[A3][0];
        frame.save_fpr(c, 25, 0x238);
        c.g[FP][0] = c.g[A2][0];
        frame.save_fpr(c, 24, 0x230);
        c.f[25] = c.f[13];
        frame.save(c, S1, 0x1f0);
        c.f[24] = c.f[12];
        frame.save(c, S2, 0x1e0);
        frame.save(c, S7, 0x190);
        frame.save_ra(c, 0x170);
        frame.save_fpr(c, 23, 0x228);
        frame.save_fpr(c, 22, 0x220);
        frame.save_fpr(c, 21, 0x218);
        frame.save_fpr(c, 20, 0x210);
        // alt = control map of players[idx].rider (BE interface 1).
        call_ptr(c, 0x0014_dc80, 0x0021_da08)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x0021_da14)?;
        c.g[V1][0] = HUD_CONTROL_HINT_PLAYER_STRIDE as u64;
        c.g[A0][0] = c.g[V0][0];
        c.mult(0, V1, S6, V1);
        c.g[V1][0] = add32(c.g[S4][0], c.g[V1][0]);
        c.g[A1][0] = c.lw(at(c, V1, HUD_CONTROL_HINT_PLAYER_RIDER));
        call_ptr(c, 0x0014_7618, 0x0021_da2c)?;
        c.g[S7][0] = c.g[V0][0];
        c.g[S2][0] = sx(p(s, 0x10));
        c.g[S1][0] = sx(p(s, 0x20));
        if c.g[S0][0] == 0 {
            c.cycles += 1;
        } else {
            // The icon: position, scale, colour; its text in the slot of
            // the rider's control map.
            c.cycles += 12;
            c.g[S0][0] = add32(c.g[S3][0], HUD_CONTROL_HINT_ICON_ELEM as u64);
            c.g[A1][0] = c.g[SP][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x001e_91f8, 0x0021_da4c)?;
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x001e_9290, 0x0021_da58)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x001e_91a8, 0x0021_da64)?;
            c.g[T0][0] = c.lb(at(c, S3, 0x7dc));
            let (slot, copy_reg, cycles, ret_to) = if c.g[S7][0] == 0 {
                (HUD_CONTROL_HINT_ICON_TEXT, V0, 12, 0x0021_da94)
            } else {
                (HUD_CONTROL_HINT_ICON_TEXT_ALT, T4, 10, 0x0021_dac4)
            };
            c.cycles += cycles;
            c.g[A0][0] = add32(c.g[S4][0], slot as u64);
            c.g[T1][0] = c.lb(at(c, S3, 0x7dd));
            c.g[A1][0] = sx(p(s, 0x30));
            c.g[A2][0] = c.g[S2][0];
            c.g[copy_reg][0] = c.ld(s);
            let v = c.g[copy_reg][0];
            c.sd(p(s, 0x30), v);
            c.g[A3][0] = c.g[S1][0];
            call_ptr(c, 0x001e_95a0, ret_to)?;
            if c.g[S7][0] != 0 {
                c.cycles += 1;
            }
        }
        c.g[S0][0] = add32(c.g[S3][0], HUD_CONTROL_HINT_GAUGE_ELEM as u64);

        // The gauge element: a (pos), t (size), sc (scale * scale[]).
        c.cycles += 52;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001e_91f8, 0x0021_dad4)?;
        c.g[A2][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x001e_9220, 0x0021_dae4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x001e_9290, 0x0021_daf0)?;
        c.lwc1(0, at(c, S5, 4));
        c.g[T0][0] = sx(p(s, 0x50));
        c.lwc1(2, at(c, S5, 0));
        c.g[S2][0] = c.g[T0][0];
        c.lwc1(4, p(s, 0x10));
        c.g[A3][0] = c.g[S1][0];
        c.lwc1(3, p(s, 0x14));
        c.g[A0][0] = c.g[S4][0];
        hint_mul(c, 4, 4, 2);
        c.lwc1(1, p(s, 0x20));
        hint_mul(c, 3, 3, 0);
        fconst(c, 2, 0x3f80_0000);
        c.lwc1(0, p(s, 0x24));
        c.g[A1][0] = sx(p(s, 0x40));
        hint_mul(c, 1, 1, 4);
        c.g[T1][0] = c.lb(at(c, S3, 0x7b8));
        hint_mul(c, 0, 0, 3);
        c.g[T2][0] = c.lb(at(c, S3, 0x7b9));
        c.g[A2][0] = c.g[SP][0];
        c.g[T3][0] = c.g[S6][0];
        c.swc1(2, p(s, 0x54));
        c.swc1(0, p(s, 0x24));
        c.swc1(1, p(s, 0x20));
        c.swc1(4, p(s, 0x10));
        c.swc1(3, p(s, 0x14));
        c.swc1(2, p(s, 0x50));
        // p (+0x40) = the aligned position of size t, unit scale v.
        call_ptr(c, 0x001f_10f8, 0x0021_db60)?;
        // Draw row of the gauge (draw state flags bits 5..9).
        c.g[V0][0] = c.lw(gp(c, -0x854));
        c.g[A1][0] = 0xffff_ffff_ffff_fc1f;
        c.g[V1][0] = c.lb(at(c, S3, 0x7ba));
        c.g[S0][0] = sx(p(s, 0x60));
        c.g[A0][0] = c.lw(at(c, V0, 0xe84));
        c.g[T0][0] = add32(c.g[S4][0], HUD_CONTROL_HINT_SPRITES as u64);
        c.g[V1][0] = shl32(c.g[V1][0], 5);
        c.g[A3][0] = u64::MAX;
        c.g[V0][0] = c.lw(at(c, A0, 8));
        c.g[V1][0] &= 0x3e0;
        c.g[A2][0] = 3;
        c.g[V0][0] &= c.g[A1][0];
        c.g[V0][0] |= c.g[V1][0];
        let v = c.g[V0][0];
        c.sw(at(c, A0, 8), v);
        // The vertex array's (empty) constructor loop.
        loop {
            c.cycles += 7;
            c.g[A2][0] = add32(c.g[A2][0], u32::MAX as u64);
            c.g[A0][0] = 0x80;
            if c.g[A2][0] == c.g[A3][0] {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0021_db98);
            }
        }
        // Vertex colours (r = alpha * 128, g b a = 0x80) and q = 1.
        c.cycles += 11;
        fconst(c, 0, 0x4300_0000);
        fconst(c, 1, 0x3f80_0000);
        c.g[V1][0] = c.g[S0][0];
        hint_mul(c, 0, 25, 0);
        c.g[A2][0] = add32(c.g[S0][0], 0xc0);
        let red = fr(c, 0).to_i32() as u32;
        c.f[2] = red;
        c.g[A1][0] = sx(red);
        let v = c.g[A1][0];
        c.sw(at(c, V1, 0x1c), v);
        loop {
            c.cycles += 8;
            let grey = c.g[A0][0];
            c.sw(at(c, V1, 0x10), grey);
            c.sw(at(c, V1, 0x14), grey);
            c.sw(at(c, V1, 0x18), grey);
            c.swc1(1, at(c, V1, 8));
            c.g[V1][0] = add32(c.g[V1][0], 0x30);
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            let v = c.g[A1][0];
            c.sw(at(c, V1, 0x1c), v);
            if event_due(c) {
                deliver_event_here(c, 0x0021_dbe0);
            }
        }

        // The button cap: 20 * sc square, vertically centred on t.
        c.cycles += 160;
        c.lwc1(3, p(s, 0x14));
        c.g[V0][0] = 0x48;
        fconst(c, 5, 0x41a0_0000);
        c.g[V1][0] = 0x4c;
        c.lwc1(2, p(s, 0x24));
        if c.g[S7][0] == 0 {
            c.g[V1][0] = c.g[V0][0];
        }
        hint_mul(c, 3, 3, 5);
        fconst(c, 6, 0x3f00_0000);
        c.lwc1(0, p(s, 0x10));
        c.g[V1][0] = add32(c.g[T0][0], c.g[V1][0]);
        c.lwc1(4, p(s, 0x44));
        c.g[A1][0] = 4;
        hint_sub(c, 2, 2, 3);
        c.lwc1(1, p(s, 0x40));
        hint_mul(c, 0, 0, 5);
        c.g[A0][0] = c.lw(r32(c, V1));
        c.swc1(1, p(s, 0x120));
        c.g[A2][0] = c.g[S0][0];
        hint_mul(c, 2, 2, 6);
        c.swc1(3, p(s, 0x54));
        c.swc1(0, p(s, 0x50));
        c.g[A3][0] = 0;
        c.g[T0][0] = c.lw(gp(c, -0x854));
        hint_add(c, 4, 4, 2);
        fconst(c, 21, 0x3f80_0000);
        fconst(c, 23, 0x4040_0000);
        c.swc1(4, p(s, 0x124));
        c.g[V1][0] = c.lw(at(c, T0, 0xe84));
        c.g[V0][0] = c.lw(r32(c, A0));
        let v = c.g[V0][0];
        c.sh(at(c, V1, 0x10), v);
        // setUV(cap).
        c.lwc1(0, at(c, A0, 0x10));
        c.lwc1(3, p(s, 0x120));
        c.swc1(0, p(s, 0x60));
        c.lwc1(2, p(s, 0x124));
        c.lwc1(0, at(c, A0, 0xc));
        c.swc1(0, p(s, 0x64));
        c.lwc1(1, at(c, A0, 0x14));
        c.swc1(1, p(s, 0x90));
        c.lwc1(0, at(c, A0, 0xc));
        c.swc1(0, p(s, 0x94));
        c.lwc1(1, at(c, A0, 0x10));
        c.swc1(1, p(s, 0xc0));
        c.lwc1(0, at(c, A0, 0x18));
        c.swc1(0, p(s, 0xc4));
        c.lwc1(1, at(c, A0, 0x14));
        c.swc1(1, p(s, 0xf0));
        c.lwc1(0, at(c, A0, 0x18));
        // Positions through the temp at +0x130.
        c.swc1(3, p(s, 0x130));
        c.swc1(0, p(s, 0xf4));
        c.swc1(2, p(s, 0x134));
        c.lwc1(1, p(s, 0x50));
        c.sw(p(s, 0x138), 0);
        c.swc1(21, p(s, 0x13c));
        hint_add(c, 1, 3, 1);
        c.lwc1(0, p(s, 0x54));
        c.lq(V1, p(s, 0x130));
        c.swc1(1, p(s, 0x130));
        hint_add(c, 0, 2, 0);
        c.sw(p(s, 0x138), 0);
        c.sq(V1, p(s, 0x80));
        c.lq(V0, p(s, 0x130));
        c.swc1(3, p(s, 0x130));
        c.swc1(0, p(s, 0x134));
        c.sw(p(s, 0x138), 0);
        c.sq(V0, p(s, 0xb0));
        c.lq(V1, p(s, 0x130));
        c.swc1(1, p(s, 0x130));
        c.sw(p(s, 0x138), 0);
        c.sq(V1, p(s, 0xe0));
        c.lq(V0, p(s, 0x130));
        c.sq(V0, p(s, 0x110));
        hint_draw_quad(c, 0x0021_dd44)?;

        // The end piece: p.x += v.x + 3 sc.x, t.x -= the same; v = p;
        // segW (f22) = sc.x * bar->scale * 9.
        c.lwc1(0, p(s, 0x10));
        c.g[A1][0] = 4;
        c.lwc1(1, p(s, 0x50));
        c.g[A2][0] = c.g[S0][0];
        hint_mul(c, 0, 0, 23);
        c.lwc1(3, p(s, 0x40));
        c.lwc1(2, p(s, 0x20));
        c.g[A3][0] = 0;
        c.g[V1][0] = c.lw(at(c, S4, HUD_CONTROL_HINT_END_SPRITE));
        hint_add(c, 1, 1, 0);
        c.lwc1(4, r32(c, FP));
        c.g[T0][0] = c.lw(gp(c, -0x854));
        fconst(c, 0, 0x4110_0000);
        hint_sub(c, 2, 2, 1);
        hint_add(c, 3, 3, 1);
        hint_mul(c, 4, 4, 0);
        c.swc1(2, p(s, 0x20));
        c.swc1(3, p(s, 0x40));
        c.g[V0][0] = c.ld(p(s, 0x40));
        let v = c.g[V0][0];
        c.sd(p(s, 0x50), v);
        c.g[A0][0] = c.lw(at(c, T0, 0xe84));
        c.g[V0][0] = c.lw(r32(c, V1));
        c.lwc1(0, p(s, 0x10));
        let v = c.g[V0][0];
        c.sh(at(c, A0, 0x10), v);
        hint_mul(c, 22, 0, 4);
        c.lwc1(1, at(c, V1, 0x10));
        c.lwc1(3, p(s, 0x40));
        c.swc1(1, p(s, 0x60));
        c.lwc1(2, p(s, 0x44));
        hint_add(c, 4, 3, 22);
        c.lwc1(0, at(c, V1, 0xc));
        c.swc1(0, p(s, 0x64));
        c.lwc1(1, at(c, V1, 0x14));
        c.swc1(1, p(s, 0x90));
        c.lwc1(0, at(c, V1, 0xc));
        c.swc1(0, p(s, 0x94));
        c.lwc1(1, at(c, V1, 0x10));
        c.swc1(1, p(s, 0xc0));
        c.lwc1(0, at(c, V1, 0x18));
        c.swc1(0, p(s, 0xc4));
        c.lwc1(1, at(c, V1, 0x14));
        c.swc1(1, p(s, 0xf0));
        c.lwc1(0, at(c, V1, 0x18));
        c.swc1(0, p(s, 0xf4));
        c.swc1(3, p(s, 0x120));
        c.sw(p(s, 0x128), 0);
        c.swc1(2, p(s, 0x124));
        c.swc1(21, p(s, 0x12c));
        c.lwc1(0, p(s, 0x24));
        c.lq(V1, p(s, 0x120));
        c.swc1(4, p(s, 0x120));
        hint_add(c, 0, 2, 0);
        c.sw(p(s, 0x128), 0);
        c.sq(V1, p(s, 0x80));
        c.lq(V0, p(s, 0x120));
        c.swc1(3, p(s, 0x120));
        c.swc1(0, p(s, 0x124));
        c.sw(p(s, 0x128), 0);
        c.sq(V0, p(s, 0xb0));
        c.lq(V1, p(s, 0x120));
        c.swc1(4, p(s, 0x120));
        c.sw(p(s, 0x128), 0);
        c.sq(V1, p(s, 0xe0));
        c.lq(V0, p(s, 0x120));
        c.sq(V0, p(s, 0x110));
        hint_draw_quad(c, 0x0021_de6c)?;
        c.lwc1(0, p(s, 0x40));
        c.g[V0][0] = c.lw(at(c, FP, 4));
        hint_add(c, 0, 0, 22);
        c.swc1(0, p(s, 0x40));

        if (c.g[V0][0] as i64) > 0 {
            // The middle piece: width (f20) t.x - 2 segW.
            c.cycles += 60;
            c.g[V0][0] = c.lw(at(c, S4, HUD_CONTROL_HINT_MID_SPRITE));
            hint_add(c, 1, 22, 22);
            c.g[A0][0] = c.lw(gp(c, -0x854));
            c.g[A1][0] = 4;
            c.g[T0][0] = c.lw(r32(c, V0));
            c.g[A2][0] = c.g[S0][0];
            c.g[V1][0] = c.lw(at(c, A0, 0xe84));
            c.g[A3][0] = 0;
            let v = c.g[T0][0];
            c.sh(at(c, V1, 0x10), v);
            c.lwc1(0, at(c, V0, 0x10));
            c.lwc1(20, p(s, 0x20));
            c.swc1(0, p(s, 0x60));
            hint_sub(c, 20, 20, 1);
            c.lwc1(3, p(s, 0x40));
            c.lwc1(0, at(c, V0, 0xc));
            c.lwc1(2, p(s, 0x44));
            c.swc1(0, p(s, 0x64));
            hint_add(c, 4, 3, 20);
            c.lwc1(0, at(c, V0, 0x14));
            c.swc1(0, p(s, 0x90));
            c.lwc1(1, at(c, V0, 0xc));
            c.swc1(1, p(s, 0x94));
            c.lwc1(0, at(c, V0, 0x10));
            c.swc1(0, p(s, 0xc0));
            c.lwc1(1, at(c, V0, 0x18));
            c.swc1(1, p(s, 0xc4));
            c.lwc1(0, at(c, V0, 0x14));
            c.swc1(0, p(s, 0xf0));
            c.lwc1(0, p(s, 0x24));
            c.lwc1(1, at(c, V0, 0x18));
            c.swc1(3, p(s, 0x120));
            c.swc1(2, p(s, 0x124));
            c.sw(p(s, 0x128), 0);
            hint_add(c, 2, 2, 0);
            c.swc1(21, p(s, 0x12c));
            c.swc1(1, p(s, 0xf4));
            c.lq(V0, p(s, 0x120));
            c.swc1(4, p(s, 0x120));
            c.sw(p(s, 0x128), 0);
            c.sq(V0, p(s, 0x80));
            c.lq(V0, p(s, 0x120));
            c.swc1(3, p(s, 0x120));
            c.sw(p(s, 0x128), 0);
            c.swc1(2, p(s, 0x124));
            c.sq(V0, p(s, 0xb0));
            c.lq(V1, p(s, 0x120));
            c.swc1(4, p(s, 0x120));
            c.sw(p(s, 0x128), 0);
            c.sq(V1, p(s, 0xe0));
            c.lq(V0, p(s, 0x120));
            c.sq(V0, p(s, 0x110));
            hint_draw_quad(c, 0x0021_df64)?;
            c.lwc1(0, p(s, 0x40));
            hint_add(c, 0, 0, 20);
            c.swc1(0, p(s, 0x40));
        }

        // The mirrored end piece (setUVFlip).
        c.cycles += 61;
        c.g[V1][0] = c.lw(at(c, S4, HUD_CONTROL_HINT_END_SPRITE));
        c.g[A2][0] = c.g[S0][0];
        c.g[T0][0] = c.lw(gp(c, -0x854));
        c.g[A1][0] = 4;
        c.g[A0][0] = c.lw(r32(c, V1));
        c.g[A3][0] = 0;
        c.g[V0][0] = c.lw(at(c, T0, 0xe84));
        let v = c.g[A0][0];
        c.sh(at(c, V0, 0x10), v);
        c.lwc1(0, at(c, V1, 0x14));
        c.lwc1(3, p(s, 0x40));
        c.swc1(0, p(s, 0x60));
        c.lwc1(2, p(s, 0x44));
        hint_add(c, 4, 3, 22);
        c.lwc1(0, at(c, V1, 0xc));
        c.swc1(0, p(s, 0x64));
        c.lwc1(1, at(c, V1, 0x10));
        c.swc1(1, p(s, 0x90));
        c.lwc1(0, at(c, V1, 0xc));
        c.swc1(0, p(s, 0x94));
        c.lwc1(1, at(c, V1, 0x14));
        c.swc1(1, p(s, 0xc0));
        c.lwc1(0, at(c, V1, 0x18));
        c.swc1(0, p(s, 0xc4));
        c.lwc1(1, at(c, V1, 0x10));
        c.swc1(1, p(s, 0xf0));
        c.lwc1(0, at(c, V1, 0x18));
        c.swc1(3, p(s, 0x120));
        c.swc1(2, p(s, 0x124));
        c.sw(p(s, 0x128), 0);
        c.swc1(21, p(s, 0x12c));
        c.swc1(0, p(s, 0xf4));
        c.lq(V1, p(s, 0x120));
        c.swc1(4, p(s, 0x120));
        c.sw(p(s, 0x128), 0);
        c.sq(V1, p(s, 0x80));
        c.lq(V0, p(s, 0x120));
        c.sq(V0, p(s, 0xb0));
        c.lwc1(0, p(s, 0x24));
        c.swc1(3, p(s, 0x120));
        hint_add(c, 2, 2, 0);
        c.sw(p(s, 0x128), 0);
        c.swc1(2, p(s, 0x124));
        c.lq(V1, p(s, 0x120));
        c.swc1(4, p(s, 0x120));
        c.sw(p(s, 0x128), 0);
        c.sq(V1, p(s, 0xe0));
        c.lq(V0, p(s, 0x120));
        c.sq(V0, p(s, 0x110));
        hint_draw_quad(c, 0x0021_e048)?;
        c.lwc1(0, p(s, 0x40));
        setf(c, 1, FpuF32::ZERO);
        hint_add(c, 0, 0, 22);
        let fill = fr(c, 24);
        let empty = le(c, fill, FpuF32::ZERO);
        c.swc1(0, p(s, 0x40));

        if !empty {
            // The fill: margins (3, 3) set once, scaled by sc.
            c.cycles += 3;
            c.g[V0][0] = c.lw(gp(c, HUD_CONTROL_HINT_MARGIN_INIT));
            c.lwc1(2, p(s, 0x10));
            if c.g[V0][0] == 0 {
                c.cycles += 5;
                c.g[V0][0] = 1;
                c.swc1(23, gp(c, HUD_CONTROL_HINT_MARGIN_Y));
                let v = c.g[V0][0];
                c.sw(gp(c, HUD_CONTROL_HINT_MARGIN_INIT), v);
                c.swc1(23, gp(c, HUD_CONTROL_HINT_MARGIN_X));
                c.lwc1(2, p(s, 0x10));
            }
            c.cycles += 56;
            c.g[V0][0] = 0x004d_0000;
            c.lwc1(1, gp(c, HUD_CONTROL_HINT_MARGIN_X));
            c.g[T3][0] = 0xffff_ffff_ffff_fc1f;
            c.lwc1(4, p(s, 0x14));
            c.g[A0][0] = c.g[S4][0];
            hint_mul(c, 1, 1, 2);
            c.lwc1(0, gp(c, HUD_CONTROL_HINT_MARGIN_Y));
            c.lwc1(2, p(s, 0x20));
            c.g[A1][0] = c.g[S2][0];
            hint_mul(c, 0, 0, 4);
            c.lwc1(3, p(s, 0x24));
            hint_add(c, 4, 1, 1);
            c.swc1(1, p(s, 0x120));
            c.g[T2][0] = c.lw(gp(c, -0x854));
            c.g[A2][0] = sx(p(s, 0x140));
            hint_add(c, 1, 0, 0);
            c.swc1(0, p(s, 0x124));
            hint_sub(c, 2, 2, 4);
            c.g[A3][0] = sx(p(s, 0x160));
            c.g[V1][0] = c.lb(at(c, S3, 0x7ba));
            c.g[T0][0] = sx(p(s, 0x150));
            hint_sub(c, 3, 3, 1);
            hint_mul(c, 2, 2, 24);
            c.g[V1][0] = add32(c.g[V1][0], 1);
            c.g[V1][0] = shl32(c.g[V1][0], 5);
            c.swc1(3, p(s, 0x144));
            c.g[V1][0] &= 0x3e0;
            c.swc1(2, p(s, 0x140));
            c.g[T1][0] = sx(HUD_CONTROL_HINT_FILL_COLOUR);
            c.g[T4][0] = c.ld(HUD_CONTROL_HINT_FILL_COLOUR);
            c.g[T5][0] = c.ld(HUD_CONTROL_HINT_FILL_COLOUR + 8);
            let (lo, hi) = (c.g[T4][0], c.g[T5][0]);
            c.sd(p(s, 0x150), lo);
            c.sd(p(s, 0x158), hi);
            // colour.r *= alpha; v += margins.
            c.lwc1(0, p(s, 0x150));
            c.lwc1(1, p(s, 0x50));
            c.lwc1(2, p(s, 0x54));
            hint_mul(c, 0, 0, 25);
            c.lwc1(3, p(s, 0x120));
            c.lwc1(4, p(s, 0x124));
            hint_add(c, 1, 1, 3);
            c.swc1(0, p(s, 0x150));
            hint_add(c, 2, 2, 4);
            c.swc1(1, p(s, 0x50));
            c.swc1(2, p(s, 0x54));
            // Draw row + 1.
            c.g[T1][0] = c.lw(at(c, T2, 0xe84));
            c.g[V0][0] = c.lw(at(c, T1, 8));
            c.g[V0][0] &= c.g[T3][0];
            c.g[V0][0] |= c.g[V1][0];
            let v = c.g[V0][0];
            c.sw(at(c, T1, 8), v);
            c.swc1(21, p(s, 0x164));
            c.swc1(21, p(s, 0x160));
            call_ptr(c, 0x001f_1338, 0x0021_e164)?;
        }

        c.cycles += 18;
        frame.restore(c, S0, 0x200);
        frame.restore(c, S1, 0x1f0);
        frame.restore(c, S2, 0x1e0);
        frame.restore(c, S3, 0x1d0);
        frame.restore(c, S4, 0x1c0);
        frame.restore(c, S5, 0x1b0);
        frame.restore(c, S6, 0x1a0);
        frame.restore(c, S7, 0x190);
        frame.restore(c, FP, 0x180);
        frame.restore_ra(c, 0x170);
        frame.restore_fpr(c, 25, 0x238);
        frame.restore_fpr(c, 24, 0x230);
        frame.restore_fpr(c, 23, 0x228);
        frame.restore_fpr(c, 22, 0x220);
        frame.restore_fpr(c, 21, 0x218);
        frame.restore_fpr(c, 20, 0x210);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{DATA, GAME_GP};
    use crate::game::fe::ovstatehud::tests::{rider_ind_elem, rider_ind_f, rider_ind_uf};

    /// `HUD_DrawControlHint`: with and without the icon (control map 0 or
    /// another: both text slots, empty), both caps, middle piece on and off
    /// (`bar->count` <= 0 / > 0), fill <= 0 (no fill) or positive, the
    /// fill margins set or still to initialise. The BE interface method and
    /// the renderer's draw slot are fakes (some calls unwind).
    #[test]
    fn hud_draw_control_hint_matches() {
        covered(
            ee::s05::f_0021d9a0__generated,
            hud_draw_control_hint,
            250,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0021_d9a0, 6);
                let (hud, elems, bar, scale) = (DATA, DATA + 0x2000, DATA + 0x3000, DATA + 0x3010);
                // The HUD's sprites 0x10..0x13: texture id and UV rectangle.
                for k in 0x10..0x14u32 {
                    let sprite = DATA + 0x4000 + 0x40 * k;
                    c.bus
                        .write32(hud + HUD_CONTROL_HINT_SPRITES + 4 * k, sprite);
                    c.bus.write32(sprite, r.next_u32() % 64);
                    for j in 1..8 {
                        c.bus
                            .write32(sprite + 4 * j, rider_ind_f(rider_ind_uf(&mut r, 0.0, 64.0)));
                    }
                }
                // Icon text slots without text: HUD_DrawIconText returns at once.
                c.bus.write32(hud + HUD_CONTROL_HINT_ICON_TEXT + 0x18, 0);
                c.bus
                    .write32(hud + HUD_CONTROL_HINT_ICON_TEXT_ALT + 0x18, 0);
                // players[idx].rider -> player slot -> control map byte.
                let idx = seed % 2;
                let rider = r.next_u32() % 4;
                c.bus.write32(
                    hud + HUD_CONTROL_HINT_PLAYER_STRIDE * idx + HUD_CONTROL_HINT_PLAYER_RIDER,
                    rider,
                );
                let slot = r.next_u32() % 4;
                c.bus.write32(0x0053_05b0 + 4 * rider, slot);
                let map = if seed % 3 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 3
                };
                c.bus.write8(0x0053_4fe0 + 0x1c * slot + 0x13, map as u8);
                // The gauge and icon elements (with their align/row bytes).
                rider_ind_elem(c, &mut r, elems + HUD_CONTROL_HINT_GAUGE_ELEM);
                rider_ind_elem(c, &mut r, elems + HUD_CONTROL_HINT_ICON_ELEM);
                // The bar: segment scale and middle count.
                c.bus
                    .write32(bar, rider_ind_f(rider_ind_uf(&mut r, 0.2, 2.0)));
                let count = match seed % 4 {
                    0 => 0,
                    1 => (r.next_u32() % 5) as i32 - 5,
                    _ => 1 + (r.next_u32() % 8) as i32,
                };
                c.bus.write32(bar + 4, count as u32);
                for i in 0..2 {
                    c.bus
                        .write32(scale + 4 * i, rider_ind_f(rider_ind_uf(&mut r, 0.5, 2.0)));
                }
                // Fill colour and the margins (set, or still to initialise).
                for i in 0..4 {
                    c.bus.write32(
                        HUD_CONTROL_HINT_FILL_COLOUR + 4 * i,
                        rider_ind_f(rider_ind_uf(&mut r, 0.0, 1.0)),
                    );
                }
                let init = if seed % 5 == 0 { 0 } else { 1 };
                c.bus.write32(
                    GAME_GP.wrapping_add(HUD_CONTROL_HINT_MARGIN_INIT as u32),
                    init,
                );
                for off in [HUD_CONTROL_HINT_MARGIN_X, HUD_CONTROL_HINT_MARGIN_Y] {
                    c.bus.write32(
                        GAME_GP.wrapping_add(off as u32),
                        rider_ind_f(rider_ind_uf(&mut r, 0.0, 6.0)),
                    );
                }
                // Renderer (gp-0x854): draw state (+0xe84) and the vtable
                // (+0x10d8) whose slot 0x238 is a fake.
                let (renderer, state, vt) = (DATA + 0x6000, DATA + 0x7800, DATA + 0x7000);
                c.bus.write32(GAME_GP.wrapping_sub(0x854), renderer);
                c.bus.write32(renderer + 0xe84, state);
                c.bus.write32(state + 8, r.next_u32());
                c.bus.write32(renderer + 0x10d8, vt);
                let delta = (r.next_u32() as i16 as i32 % 64) & !3;
                c.bus.write16(vt + 0x238, delta as u16);
                c.bus.write32(vt + 0x23c, FAKE);
                // BELibrary and its interface (a fake method).
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
                c.bus.write32(GAME_GP.wrapping_sub(0x1efc), 0);
                c.bus.write32(0x0053_08a8, DATA + 0x5000);
                c.bus.write32(DATA + 0x5004, DATA + 0x5100);
                c.bus.write16(DATA + 0x5100 + 8, 0);
                c.bus.write32(DATA + 0x5100 + 0xc, FAKE);
                // Arguments.
                c.g[A0][0] = sx(hud);
                c.g[A1][0] = sx(elems);
                c.g[A2][0] = sx(bar);
                c.g[A3][0] = idx as u64;
                c.g[T0][0] = if seed % 3 == 1 {
                    0
                } else {
                    1 + (r.next_u32() % 4) as u64
                };
                c.g[T1][0] = sx(scale);
                let fill = match seed % 6 {
                    0 => 0.0,
                    1 => -rider_ind_uf(&mut r, 0.01, 1.0),
                    _ => rider_ind_uf(&mut r, 0.01, 1.0),
                };
                c.f[12] = rider_ind_f(fill);
                c.f[13] = rider_ind_f(rider_ind_uf(&mut r, 0.0, 1.0));
                c.pc = 0x0021_d9a0;
            },
        );
    }
}
