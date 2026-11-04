//! Staging wave 201 (oc204): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size),
//! one function per writer. Each reproduces the generated body's registers,
//! cycles, memory and call order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0021_d1a0, hud_draw_meter_bar, "HUD_DrawMeterBar"),
    (0x002f_1150, rider_base_update_attack_fx, "RiderBase_UpdateAttackFX"),
    (0x002b_d698, twpigd_weather_blend, "tWPIGD_Weather_Blend"),
    (0x0010_1a28, focus_man_collect_node_objects, "FocusMan_CollectNodeObjects"),
];

// ---------------------------------------------------------------------------
// HUD_DrawMeterBar (0x21d1a0)

/// `HUD_DrawMeterBar`'s 0x1c0-byte frame: the element's layout and the
/// textured quad it draws with.
pub mod meter_bar {
    /// `float pos[2]` (HUD_GetElemPos).
    pub const POS: u32 = 0x00;
    /// `float x, y`: the aligned screen position. `y` walks up the bar as
    /// the full bar is drawn.
    pub const X: u32 = 0x10;
    pub const Y: u32 = 0x14;
    /// `float w, h` (HUD_GetElemSize, then scaled).
    pub const W: u32 = 0x20;
    pub const H: u32 = 0x24;
    /// `float sx, sy` (HUD_GetElemScale, then scaled; `sy` also by the
    /// segment scale: one segment is `23 * sy` high, the top cap `31 * sy`).
    pub const SCALE: u32 = 0x30;
    pub const SY: u32 = 0x34;
    /// `float {1, 1}`: the scale passed to HUD_AlignElemPos.
    pub const UNIT_SCALE: u32 = 0x40;
    /// The quad: four 0x30-byte vertices.
    pub const QUAD: u32 = 0x50;
    pub const VERTEX: u32 = 0x30;
    /// Vertex fields: texture `u, v`, `z = 1`, the colour as four ints,
    /// screen `x, y`, `0`, `1`.
    pub const V_U: u32 = 0x00;
    pub const V_V: u32 = 0x04;
    pub const V_Z: u32 = 0x08;
    pub const V_COLOR: u32 = 0x10;
    pub const V_X: u32 = 0x20;
    pub const V_Y: u32 = 0x24;
    pub const V_ZERO: u32 = 0x28;
    pub const V_ONE: u32 = 0x2c;
    /// HUD sprite fields: texture id (`lw`, stored as a short into the
    /// render state), v of the top edge, u left, u right, v of the bottom.
    pub const TEX_ID: u32 = 0x00;
    pub const TEX_V0: u32 = 0x0c;
    pub const TEX_U0: u32 = 0x10;
    pub const TEX_U1: u32 = 0x14;
    pub const TEX_V1: u32 = 0x18;
    /// The sprite table of the HUD object: 12 bytes per meter style
    /// (`top cap, segment, bottom cap` pointers).
    pub const SPRITES: u32 = 0x4fc;
    /// Renderer (`gp-0x854`) fields: the current render-state record and the
    /// vptr; render-state fields: blend bits (5..9 of word 2), texture id.
    pub const RENDERER: u32 = 0x854;
    pub const STATE_TOP: u32 = 0xe84;
    pub const VPTR: u32 = 0x10d8;
    pub const STATE_FLAGS: u32 = 0x08;
    pub const STATE_TEX: u32 = 0x10;
    /// Renderer method `DrawPrims(count, verts, 0)`.
    pub const DRAW_PRIMS: u32 = 0x238;
    /// Layout element fields: horizontal / vertical alignment and the blend
    /// mode byte.
    pub const ELEM_HALIGN: u32 = 0x20;
    pub const ELEM_VALIGN: u32 = 0x21;
    pub const ELEM_BLEND: u32 = 0x22;
    /// Segment and top cap heights in layout pixels.
    pub const SEGMENT_H: u32 = 0x41b8_0000; // 23.0
    pub const TOP_CAP_H: u32 = 0x41f8_0000; // 31.0
}

/// `f[n]` as an FPU float.
#[inline(always)]
fn mb_f(c: &Cpu, n: usize) -> crate::ps2f::FpuF32 {
    crate::ps2f::FpuF32(c.f[n])
}

/// `f[n] = v`.
#[inline(always)]
fn mb_set(c: &mut Cpu, n: usize, v: crate::ps2f::FpuF32) {
    c.f[n] = v.0;
}

/// `lwc1` from the frame.
#[inline(always)]
fn mb_ld(c: &mut Cpu, off: u32) -> crate::ps2f::FpuF32 {
    crate::guest::Addr(sp(c).wrapping_add(off)).read_fpu(c)
}

/// `swc1` to the frame.
#[inline(always)]
fn mb_st(c: &mut Cpu, off: u32, v: crate::ps2f::FpuF32) {
    crate::guest::Addr(sp(c).wrapping_add(off)).write_fpu(c, v)
}

/// `lwc1` of a sprite field.
#[inline(always)]
fn mb_tex(c: &mut Cpu, tex: u64, off: u32) -> crate::ps2f::FpuF32 {
    crate::guest::Addr((tex as u32).wrapping_add(off)).read_fpu(c)
}

/// The renderer (`lw gp-0x854`).
#[inline(always)]
fn mb_renderer(c: &mut Cpu) -> u64 {
    crate::guest::Addr(r32(c, GP).wrapping_sub(meter_bar::RENDERER)).lw(c)
}

/// The quad's texture coordinates from sprite `tex`, the top edge at
/// `v_top` (the sprite's v0, or clipped). Leaves `f0 = v1`, `f1 = u1`, as
/// the original's alternating loads do.
fn mb_put_uvs(c: &mut Cpu, tex: u64, v_top: crate::ps2f::FpuF32) {
    use meter_bar::*;
    let u0 = mb_tex(c, tex, TEX_U0);
    let u1 = mb_tex(c, tex, TEX_U1);
    let v1 = mb_tex(c, tex, TEX_V1);
    for (i, u, v) in [(0, u0, v_top), (1, u1, v_top), (2, u0, v1), (3, u1, v1)] {
        mb_st(c, QUAD + VERTEX * i + V_U, u);
        mb_st(c, QUAD + VERTEX * i + V_V, v);
    }
    mb_set(c, 0, v1);
    mb_set(c, 1, u1);
}

/// The quad's screen corners: `(x0, top)`, `(x1, top)`, `(x0, bottom)`,
/// `(x1, bottom)`.
fn mb_put_xy(
    c: &mut Cpu,
    x0: crate::ps2f::FpuF32,
    x1: crate::ps2f::FpuF32,
    top: crate::ps2f::FpuF32,
    bottom: crate::ps2f::FpuF32,
) {
    use meter_bar::*;
    for (i, x, y) in [(0, x0, top), (1, x1, top), (2, x0, bottom), (3, x1, bottom)] {
        mb_st(c, QUAD + VERTEX * i + V_X, x);
        mb_st(c, QUAD + VERTEX * i + V_Y, y);
    }
}

/// Bind sprite `tex`'s texture: `sh id, state+0x10`. The registers the
/// original uses differ per block: the caller names them (`state_reg` holds
/// the render-state record, `id_reg` the id).
fn mb_bind(c: &mut Cpu, renderer: u64, tex: u64, state_reg: usize, id_reg: usize) {
    use meter_bar::*;
    let id = crate::guest::Addr((tex as u32).wrapping_add(TEX_ID)).lw(c);
    c.g[id_reg][0] = id;
    let state = crate::guest::Addr((renderer as u32).wrapping_add(STATE_TOP)).lw(c);
    c.g[state_reg][0] = state;
    crate::guest::Addr((state as u32).wrapping_add(STATE_TEX)).sh(c, id);
}

/// `renderer->DrawPrims(4, quad, 0)` returning to `ret` (t0 = renderer,
/// v0 = vptr, v1 = method, a0 = adjusted this).
fn mb_draw_quad(c: &mut Cpu, ret_to: u32) -> crate::call::Flow {
    use meter_bar::*;
    c.g[A1][0] = 4;
    c.g[A2][0] = c.g[S3][0];
    c.g[A3][0] = 0;
    let renderer = mb_renderer(c);
    c.g[T0][0] = renderer;
    let vt = crate::guest::Addr((renderer as u32).wrapping_add(VPTR)).lw(c);
    c.g[V0][0] = vt;
    let m = crate::call::VSlot::read(c, vt, DRAW_PRIMS);
    c.g[V1][0] = m.func;
    c.g[A0][0] = m.this(renderer);
    call_ptr(c, m.func as u32, ret_to)
}

/// The partial bar's clipped cap: when the cap's top `top` (f`top_reg`)
/// reaches above the fill line (f21), its top edge moves down to the fill
/// line and its top `v` (f2) moves proportionally: `v0 + (v1 - v0) *
/// (fill - top) / height` (f3 = cap height).
fn mb_clip_cap(c: &mut Cpu, tex: u64, top_reg: usize) {
    use meter_bar::*;
    c.cycles += 7;
    let v1 = mb_tex(c, tex, TEX_V1);
    let fill = mb_f(c, 21);
    let cut = fill - mb_f(c, top_reg);
    mb_set(c, 1, cut);
    mb_set(c, top_reg, fill);
    let dv = ((v1 - mb_f(c, 2)) * cut).div(mb_f(c, 3));
    mb_set(c, 0, dv);
    mb_set(c, 2, mb_f(c, 2) + dv);
}

/// A partial bar's cap from sprite `tex` (in a0's block): v top in f2,
/// screen top in f`top_reg`, bottom in f4. 39 or 37 cycles are charged by
/// the caller.
fn mb_draw_partial_cap(c: &mut Cpu, tex: u64, top_reg: usize, ret_to: u32) -> crate::call::Flow {
    use meter_bar::*;
    let renderer = c.g[V1][0];
    mb_bind(c, renderer, tex, V0, A0);
    let x = mb_ld(c, X);
    let v_top = mb_f(c, 2);
    mb_put_uvs(c, tex, v_top);
    mb_set(c, 3, x);
    let x1 = x + mb_ld(c, W);
    mb_set(c, 2, x1);
    let (top, bottom) = (mb_f(c, top_reg), mb_f(c, 4));
    mb_put_xy(c, x, x1, top, bottom);
    mb_draw_quad(c, ret_to)
}

/// `void HUD_DrawMeterBar(HUDState* hud (a0), HUDElem* elems (a1), s8 elem
/// (a2), int style (a3), const MeterSeg* seg (t0: {float scale; int
/// count}), const float colour[4] (t1), int flags (t2), float rect_out[4]
/// or null (t3), float fill (f12), float sx (f13), float sy (f14))`
/// (0x21d1a0): draws a vertical meter (the boost bar) at a HUD layout
/// element. The element's position, size and scale (sy also times
/// `seg->scale`) are aligned on screen; the colour (times 128, as ints)
/// goes into the four vertices and the element's blend byte into the
/// render state. A full bar (`fill == 1`) is drawn bottom to top: the
/// bottom cap (sprite 2, 23 units high, its bottom at the element's
/// bottom), `count` segments (sprite 1, 23 units each) and the top cap
/// (sprite 0, 31 units). A partial bar fills from the bottom up to
/// `y + h * (1 - fill)`: the same pieces, the one that crosses the fill line
/// clipped (its top edge and top `v` moved down) and the drawing stopped
/// there. The drawn rectangle goes to `rect_out` when given. Frame 0x1c0
/// (s0-s7, f20-f22).
pub fn hud_draw_meter_bar(c: &mut Cpu) {
    use crate::ps2f::FpuF32;
    use meter_bar::*;
    guest(c, |c| {
        c.cycles += 82;
        let frame = Frame::open(c, 0x1c0);
        for (reg, off) in [
            (S0, 0x190),
            (S1, 0x180),
            (S2, 0x170),
            (S3, 0x160),
            (S4, 0x150),
            (S5, 0x140),
            (S6, 0x130),
            (S7, 0x120),
        ] {
            frame.save(c, reg, off);
        }
        frame.save_ra(c, 0x110);
        for (f, off) in [(22, 0x1b0), (21, 0x1a8), (20, 0x1a0)] {
            frame.save_fpr(c, f, off);
        }
        // The element (elems + elem * 0x24) and the style's sprite triple
        // (hud + 0x4fc + style * 12).
        c.g[V0][0] = 0xc;
        c.mult(0, A3, A3, V0);
        c.g[S0][0] = c.g[A2][0] as u8 as i8 as i64 as u64;
        c.g[V0][0] = 0x24;
        c.mult(0, V1, S0, V0);
        c.g[A3][0] = sx(r32(c, A3).wrapping_add(SPRITES));
        c.g[S2][0] = c.g[A0][0];
        c.g[S3][0] = c.g[T2][0];
        c.g[S0][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
        c.g[S6][0] = c.g[T0][0];
        c.g[S7][0] = sx(r32(c, S2).wrapping_add(r32(c, A3)));
        c.g[S5][0] = c.g[T1][0];
        c.g[S4][0] = c.g[T3][0];
        c.f[22] = c.f[12];
        c.f[21] = c.f[14];
        c.f[20] = c.f[13];
        let elem = r32(c, S0);
        let sprites = r32(c, S7);
        let seg = r32(c, S6);
        let colour = r32(c, S5);
        let rect_out = c.g[S4][0];

        // HUD_GetElemPos(elem, pos); HUD_GetElemSize(elem, size, null);
        // HUD_GetElemScale(elem, scale).
        c.g[A1][0] = sx(sp(c).wrapping_add(POS));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001e_91f8, 0x0021_d224)?;
        c.g[S1][0] = sx(sp(c).wrapping_add(W));
        c.g[A2][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x001e_9220, 0x0021_d238)?;
        c.g[A1][0] = sx(sp(c).wrapping_add(SCALE));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001e_9290, 0x0021_d244)?;

        // Scale by (sx, sy) and the segment scale; align with scale {1, 1}.
        let scale_x = mb_ld(c, SCALE) * mb_f(c, 20);
        let scale_y = mb_ld(c, SY) * mb_f(c, 21);
        let seg_scale = crate::guest::Addr(seg).read_fpu(c);
        let w = mb_ld(c, W) * scale_x;
        let h = mb_ld(c, H) * scale_y;
        let sy = scale_y * seg_scale;
        mb_set(c, 2, scale_x);
        mb_set(c, 0, scale_y);
        mb_set(c, 1, sy);
        mb_set(c, 3, w);
        mb_set(c, 4, h);
        c.g[AT][0] = 0x3f80_0000;
        mb_set(c, 5, FpuF32::ONE);
        mb_st(c, SCALE, scale_x);
        mb_st(c, SY, sy);
        mb_st(c, W, w);
        mb_st(c, H, h);
        mb_st(c, UNIT_SCALE, FpuF32::ONE);
        mb_st(c, UNIT_SCALE + 4, FpuF32::ONE);
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = sx(sp(c).wrapping_add(X));
        c.g[A2][0] = sx(sp(c).wrapping_add(POS));
        c.g[A3][0] = c.g[S1][0];
        c.g[T0][0] = sx(sp(c).wrapping_add(UNIT_SCALE));
        c.g[T1][0] = crate::guest::Addr(elem.wrapping_add(ELEM_HALIGN)).lb(c);
        c.g[T2][0] = crate::guest::Addr(elem.wrapping_add(ELEM_VALIGN)).lb(c);
        c.g[T3][0] = c.g[S3][0];
        call_ptr(c, 0x001f_10f8, 0x0021_d2b0)?;

        // The element's blend byte into bits 5..9 of the render state.
        let renderer = mb_renderer(c);
        c.g[V0][0] = renderer;
        c.g[A1][0] = 0xffff_ffff_ffff_fc1f;
        let blend = crate::guest::Addr(elem.wrapping_add(ELEM_BLEND)).lb(c);
        c.g[S3][0] = sx(sp(c).wrapping_add(QUAD));
        let state = crate::guest::Addr((renderer as u32).wrapping_add(STATE_TOP)).lw(c);
        c.g[A0][0] = state;
        c.g[A3][0] = u64::MAX;
        c.g[V1][0] = sx((blend as u32) << 5) & 0x3e0;
        c.g[A2][0] = 3;
        let flags_at = crate::guest::Addr((state as u32).wrapping_add(STATE_FLAGS));
        let flags = (flags_at.lw(c) & c.g[A1][0]) | c.g[V1][0];
        c.g[V0][0] = flags;
        flags_at.sw(c, flags);

        // A countdown with no effect (a2 = 3 .. -1).
        loop {
            c.cycles += 7;
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
            c.g[A0][0] = sx(sp(c).wrapping_add(QUAD + V_X));
            if c.g[A2][0] != c.g[A3][0] {
                if event_due(c) {
                    deliver_event_here(c, 0x0021_d2e8);
                }
                continue;
            }
            break;
        }

        // The colour as ints (times 128): t1, a3, a2, a1 = c[2], c[0],
        // c[1], c[3].
        c.cycles += 23;
        c.g[AT][0] = 0x4300_0000;
        let k = FpuF32(0x4300_0000);
        let col = crate::guest::Addr(colour);
        let c2 = col.off(8).read_fpu(c) * k;
        let c3 = col.off(12).read_fpu(c) * k;
        let c0 = col.read_fpu(c) * k;
        let c1 = col.off(4).read_fpu(c) * k;
        mb_set(c, 2, c1);
        mb_set(c, 3, c0);
        mb_set(c, 4, c3);
        c.g[V1][0] = c.g[S3][0];
        c.g[T0][0] = sx(r32(c, S3).wrapping_add(4 * VERTEX));
        c.g[AT][0] = 0x3f80_0000;
        mb_set(c, 5, FpuF32::ONE);
        c.g[T1][0] = sx(c2.to_i32() as u32);
        c.g[A3][0] = sx(c0.to_i32() as u32);
        c.g[A2][0] = sx(c1.to_i32() as u32);
        c.f[1] = c1.to_i32() as u32;
        c.g[A1][0] = sx(c3.to_i32() as u32);
        c.f[0] = c3.to_i32() as u32;

        // The four vertices' constant fields.
        loop {
            c.cycles += 11;
            let v = crate::guest::Addr(r32(c, V1));
            v.off(V_COLOR + 0xc).sw(c, c.g[A3][0]);
            v.off(V_COLOR).sw(c, c.g[A2][0]);
            v.off(V_COLOR + 4).sw(c, c.g[T1][0]);
            v.off(V_COLOR + 8).sw(c, c.g[A1][0]);
            v.off(V_Z).write_fpu(c, mb_f(c, 5));
            let tail = crate::guest::Addr(r32(c, A0));
            tail.off(V_ZERO - V_X).sw(c, 0);
            tail.off(V_ONE - V_X).write_fpu(c, mb_f(c, 5));
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(VERTEX));
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[T0][0] as i64)) as u64;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(VERTEX));
            if c.g[V0][0] != 0 {
                if event_due(c) {
                    deliver_event_here(c, 0x0021_d360);
                }
                continue;
            }
            break;
        }

        c.cycles += 6;
        c.g[AT][0] = 0x3f80_0000;
        mb_set(c, 0, FpuF32::ONE);
        let full = mb_f(c, 22).eq_s(FpuF32::ONE);
        crate::abi::set_fcc(c, full);
        if full {
            // ---- Full bar: bottom cap, segments, top cap, bottom to top.
            c.cycles += 2;
            let t = mb_ld(c, X);
            mb_set(c, 1, t);
            if rect_out != 0 {
                c.cycles += 7;
                let (x, y, w, h) = (mb_f(c, 1), mb_ld(c, Y), mb_ld(c, W), mb_ld(c, H));
                mb_set(c, 2, y);
                mb_set(c, 3, w);
                mb_set(c, 0, h);
                let out = crate::guest::Addr(rect_out as u32);
                out.write_fpu(c, x);
                out.off(12).write_fpu(c, h);
                out.off(4).write_fpu(c, y);
                out.off(8).write_fpu(c, w);
            }

            // Bottom cap (sprite 2): from the element's bottom up 23 * sy.
            c.cycles += 80;
            let tex = crate::guest::Addr(sprites.wrapping_add(8)).lw(c);
            let bottom = mb_ld(c, Y) + mb_ld(c, H);
            mb_st(c, Y, bottom);
            c.g[AT][0] = SEGMENT_H as u64;
            mb_set(c, 21, FpuF32(SEGMENT_H));
            c.g[S0][0] = 0;
            let renderer = mb_renderer(c);
            c.g[T0][0] = renderer;
            mb_bind(c, renderer, tex, V1, V0);
            let piece = mb_ld(c, SY) * mb_f(c, 21);
            mb_set(c, 20, piece);
            let x = mb_ld(c, X);
            let x1 = x + mb_ld(c, W);
            let top = bottom - piece;
            mb_set(c, 3, x);
            mb_set(c, 4, bottom);
            mb_set(c, 2, x1);
            mb_set(c, 5, top);
            let v_top = mb_tex(c, tex, TEX_V0);
            mb_put_uvs(c, tex, v_top);
            mb_put_xy(c, x, x1, top, bottom);
            mb_draw_quad(c, 0x0021_d498)?;

            // Segments (sprite 1), `count` of them.
            let y = mb_ld(c, Y) - mb_f(c, 20);
            mb_set(c, 0, y);
            mb_st(c, Y, y);
            let tex = crate::guest::Addr(sprites.wrapping_add(4)).lw(c);
            c.g[V1][0] = tex;
            let renderer = mb_renderer(c);
            c.g[A0][0] = renderer;
            mb_bind(c, renderer, tex, A1, V0);
            let t = mb_ld(c, SY);
            mb_set(c, 1, t);
            mb_set(c, 20, mb_f(c, 1) * mb_f(c, 21));
            c.g[V0][0] = crate::guest::Addr(seg.wrapping_add(4)).lw(c);
            let v_top = mb_tex(c, tex, TEX_V0);
            mb_put_uvs(c, tex, v_top);
            if (c.g[V0][0] as i64) > 0 {
                loop {
                    c.cycles += 29;
                    let bottom = mb_ld(c, Y);
                    let x = mb_ld(c, X);
                    let x1 = x + mb_ld(c, W);
                    let top = bottom - mb_f(c, 20);
                    mb_set(c, 2, bottom);
                    mb_set(c, 1, x);
                    mb_set(c, 0, x1);
                    mb_set(c, 3, top);
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    mb_put_xy(c, x, x1, top, bottom);
                    mb_draw_quad(c, 0x0021_d564)?;
                    let y = mb_ld(c, Y) - mb_f(c, 20);
                    mb_set(c, 0, y);
                    let count = crate::guest::Addr(seg.wrapping_add(4)).lw(c);
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (count as i64)) as u64;
                    mb_st(c, Y, y);
                    if c.g[V0][0] != 0 {
                        if event_due(c) {
                            deliver_event_here(c, 0x0021_d508);
                        }
                        continue;
                    }
                    break;
                }
            }

            // Top cap (sprite 0): 31 * sy high.
            c.cycles += 49;
            let tex = crate::guest::Addr(sprites).lw(c);
            c.g[V1][0] = tex;
            let renderer = mb_renderer(c);
            c.g[A0][0] = renderer;
            mb_bind(c, renderer, tex, V0, T0);
            c.g[AT][0] = TOP_CAP_H as u64;
            let piece = mb_ld(c, SY) * FpuF32(TOP_CAP_H);
            let x = mb_ld(c, X);
            let bottom = mb_ld(c, Y);
            let x1 = x + mb_ld(c, W);
            let top = bottom - piece;
            mb_set(c, 4, x);
            mb_set(c, 5, bottom);
            mb_set(c, 2, top);
            mb_set(c, 3, x1);
            let v_top = mb_tex(c, tex, TEX_V0);
            mb_put_uvs(c, tex, v_top);
            mb_put_xy(c, x, x1, top, bottom);
            mb_draw_quad(c, 0x0021_d638)?;
            frame.restore(c, S0, 0x190);
        } else {
            // ---- Partial bar: up to the fill line y + h * (1 - fill) (f21).
            mb_set(c, 0, FpuF32::ONE - mb_f(c, 22));
            c.cycles += 6;
            let h = mb_ld(c, H);
            let y = mb_ld(c, Y);
            mb_set(c, 1, h);
            mb_set(c, 2, y);
            let empty = h * mb_f(c, 0);
            mb_set(c, 0, empty);
            let bottom = y + h;
            mb_set(c, 4, bottom);
            let fill = y + empty;
            mb_set(c, 21, fill);
            if rect_out != 0 {
                c.cycles += 7;
                let filled = bottom - fill;
                let (x, w) = (mb_ld(c, X), mb_ld(c, W));
                mb_set(c, 0, filled);
                mb_set(c, 1, x);
                mb_set(c, 2, w);
                let out = crate::guest::Addr(rect_out as u32);
                out.write_fpu(c, x);
                out.off(12).write_fpu(c, filled);
                out.off(8).write_fpu(c, w);
                out.off(4).write_fpu(c, fill);
            }

            // Bottom cap (sprite 2), clipped at the fill line.
            c.cycles += 11;
            let sy = mb_ld(c, SY);
            mb_set(c, 0, sy);
            c.g[S0][0] = 1;
            c.g[AT][0] = SEGMENT_H as u64;
            mb_set(c, 22, FpuF32(SEGMENT_H));
            let tex = crate::guest::Addr(sprites.wrapping_add(8)).lw(c);
            c.g[T1][0] = tex;
            let piece = sy * mb_f(c, 22);
            mb_set(c, 3, piece);
            let top = mb_f(c, 4) - piece;
            mb_set(c, 20, top);
            let clipped = top.le(mb_f(c, 21));
            crate::abi::set_fcc(c, clipped);
            let t = mb_tex(c, tex, TEX_V0);
            mb_set(c, 2, t);
            if !clipped {
                c.cycles += 1;
                c.g[S0][0] = 0;
            }
            c.cycles += 2;
            c.g[V1][0] = mb_renderer(c);
            if clipped {
                mb_clip_cap(c, tex, 20);
            }
            c.cycles += 39;
            mb_draw_partial_cap(c, tex, 20, 0x0021_d75c)?;
            c.f[4] = c.f[20];
            if !clipped {
                // Segments (sprite 1) until one crosses the fill line.
                c.cycles += 26;
                let tex = crate::guest::Addr(sprites.wrapping_add(4)).lw(c);
                c.g[S0][0] = tex;
                c.g[S2][0] = 0;
                let renderer = mb_renderer(c);
                c.g[V0][0] = renderer;
                mb_bind(c, renderer, tex, V1, A0);
                let sy = mb_ld(c, SY);
                mb_set(c, 0, sy);
                mb_set(c, 22, sy * mb_f(c, 22));
                let v_top = mb_tex(c, tex, TEX_V0);
                mb_put_uvs(c, tex, v_top);
                c.g[V0][0] = crate::guest::Addr(seg.wrapping_add(4)).lw(c);
                let mut crossed = false;
                if (c.g[V0][0] as i64) > 0 {
                    c.cycles += 1;
                    mb_set(c, 20, mb_f(c, 4) - mb_f(c, 22));
                    loop {
                        c.cycles += 4;
                        let clipped = mb_f(c, 20).le(mb_f(c, 21));
                        crate::abi::set_fcc(c, clipped);
                        c.g[S1][0] = 1;
                        if !clipped {
                            c.cycles += 1;
                            c.g[S1][0] = 0;
                        }
                        c.cycles += 2;
                        let t = mb_ld(c, X);
                        mb_set(c, 1, t);
                        if clipped {
                            c.cycles += 11;
                            let tex = c.g[S0][0];
                            let v0 = mb_tex(c, tex, TEX_V0);
                            let fill = mb_f(c, 21);
                            let cut = fill - mb_f(c, 20);
                            mb_set(c, 2, cut);
                            mb_set(c, 20, fill);
                            let dv = ((mb_tex(c, tex, TEX_V1) - v0) * cut).div(mb_f(c, 22));
                            let v_top = v0 + dv;
                            mb_set(c, 0, v_top);
                            mb_st(c, QUAD + VERTEX + V_V, v_top);
                            mb_st(c, QUAD + V_V, v_top);
                            let t = mb_ld(c, X);
                            mb_set(c, 1, t);
                        }
                        c.cycles += 21;
                        let x = mb_f(c, 1);
                        let x1 = x + mb_ld(c, W);
                        mb_set(c, 0, x1);
                        let (top, bottom) = (mb_f(c, 20), mb_f(c, 4));
                        mb_put_xy(c, x, x1, top, bottom);
                        mb_draw_quad(c, 0x0021_d864)?;
                        c.f[4] = c.f[20];
                        if c.g[S1][0] != 0 {
                            crossed = true;
                            break;
                        }
                        c.cycles += 5;
                        let count = crate::guest::Addr(seg.wrapping_add(4)).lw(c);
                        c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                        c.g[V0][0] = ((c.g[S2][0] as i64) < (count as i64)) as u64;
                        if c.g[V0][0] != 0 {
                            mb_set(c, 20, mb_f(c, 4) - mb_f(c, 22));
                            if event_due(c) {
                                deliver_event_here(c, 0x0021_d7d0);
                            }
                            continue;
                        }
                        break;
                    }
                }
                if !crossed {
                    // Top cap (sprite 0), clipped at the fill line.
                    c.cycles += 11;
                    let sy = mb_ld(c, SY);
                    mb_set(c, 1, sy);
                    c.g[V0][0] = 1;
                    c.g[AT][0] = TOP_CAP_H as u64;
                    mb_set(c, 0, FpuF32(TOP_CAP_H));
                    let tex = crate::guest::Addr(sprites).lw(c);
                    c.g[S2][0] = tex;
                    let piece = sy * FpuF32(TOP_CAP_H);
                    mb_set(c, 3, piece);
                    let top = mb_f(c, 4) - piece;
                    mb_set(c, 5, top);
                    let clipped = top.le(mb_f(c, 21));
                    crate::abi::set_fcc(c, clipped);
                    let t = mb_tex(c, tex, TEX_V0);
                    mb_set(c, 2, t);
                    if !clipped {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                    }
                    c.cycles += 2;
                    c.g[V1][0] = mb_renderer(c);
                    if clipped {
                        mb_clip_cap(c, tex, 5);
                    }
                    c.cycles += 37;
                    mb_draw_partial_cap(c, tex, 5, 0x0021_d968)?;
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x190);
        }

        c.cycles += 13;
        for (reg, off) in [
            (S1, 0x180),
            (S2, 0x170),
            (S3, 0x160),
            (S4, 0x150),
            (S5, 0x140),
            (S6, 0x130),
            (S7, 0x120),
        ] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0x110);
        for (f, off) in [(22, 0x1b0), (21, 0x1a8), (20, 0x1a0)] {
            frame.restore_fpr(c, f, off);
        }
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderBase_UpdateAttackFX (0x2f1150)

/// `RiderBase_UpdateAttackFX`'s object (`self`, a0) and the rider fields it
/// reads.
pub mod attack_fx {
    /// `RiderBase* owner`.
    pub const OWNER: u32 = 0x00;
    /// `vec4 pos[4]`: the four spark positions.
    pub const POS: u32 = 0x10;
    /// `vec4 col[4]`: their colours `{1, c, c, c}`.
    pub const COL: u32 = 0x50;
    /// `float size[4]`.
    pub const SIZE: u32 = 0x90;
    /// `int active`.
    pub const ACTIVE: u32 = 0xa0;
    /// Rider: the attack strength (sparks while `> 0`).
    pub const R_STRENGTH: u32 = 0x350;
    /// Rider: facing flag (picks which hand's bone).
    pub const R_FLIP: u32 = 0x320;
    /// Rider: the model (`+0x30`: the bone matrix array, 64 bytes each).
    pub const R_MODEL: u32 = 0x780;
    pub const MODEL_BONES: u32 = 0x30;
    /// Rider: the anim controller.
    pub const R_ANIM: u32 = 0x784;
    /// Rider: the left / right hand bones.
    pub const R_BONE_A: u32 = 0x8b0;
    pub const R_BONE_B: u32 = 0x8b8;
    /// Anim states of the punches (track 0) and their track-1 variants.
    pub const PUNCH_A: u64 = 0x146;
    pub const PUNCH_B: u64 = 0x147;
    pub const PUNCH_A1: u64 = 0x143;
    pub const PUNCH_B1: u64 = 0x144;
    /// Anim table type of a track-1 sequence that carries the punch flags.
    pub const SEQ_TYPE_PUNCH: u64 = 0xd;
    /// `D_004FF120`: the matrix's replacement translation row.
    pub const ORIGIN_ROW: u32 = 0x004f_f120;
    /// `gp+0xa0c`: the effects random seed; `gp-0x3930`: colour scale 0.9.
    pub const RAND_SEED: u32 = 0xa0c;
    pub const COLOUR_SCALE: u32 = 0x3930;
}

/// One step of the effects LCG: `prod = seed (reg) * 0x18fcd` (mult into
/// `prod`), as the original interleaves it.
#[inline(always)]
fn afx_mult(c: &mut Cpu, prod: usize, seed: usize) {
    c.mult(0, prod, seed, A1);
}

/// `out = ((prod + 0xe9507c) & 0x7fffff) | 0x3f800000`: the new seed, also a
/// float in [1, 2).
#[inline(always)]
fn afx_bits(c: &mut Cpu, out: usize, prod: usize) {
    c.g[out][0] = (sx(r32(c, prod).wrapping_add(r32(c, T4))) & c.g[A0][0]) | c.g[T1][0];
}

/// Store the seed in `reg` and read it back as a float minus 1 (`[0, 1)`).
#[inline(always)]
fn afx_store_unit(c: &mut Cpu, reg: usize) -> crate::ps2f::FpuF32 {
    use crate::ps2f::FpuF32;
    let seed_at = crate::guest::Addr(r32(c, GP).wrapping_add(attack_fx::RAND_SEED));
    seed_at.sw(c, c.g[reg][0]);
    seed_at.read_fpu(c) - FpuF32(c.f[6])
}

/// `void RiderBase_UpdateAttackFX(AttackFX* self)` (0x2f1150): the sparks
/// of a rider's punch. Active while the owner's attack strength
/// (`+0x350`) is positive. The owner's anim controller decides the hand:
/// punch A (track 0 state 0x146, or track 1 state 0x143 with a type-0xd
/// sequence whose flag bit 0 is set and bit 1 clear) uses bone `+0x8b0`
/// (`+0x8b8` when `+0x320` is clear), punch B (0x147 / 0x144) the other;
/// anything else switches the effect off. When active, the hand's bone
/// translation plus `(8, 0, 0)` through the bone matrix (translation
/// replaced by `D_004FF120`) is the centre, and four sparks get a random
/// offset of `±3s` per axis, a size `15 .. 15 + 15s` and a colour `{1, c,
/// c, c}` with `c = 0.9 s rand` (`s` the strength). Frame 0xf0 (s0-s4).
pub fn rider_base_update_attack_fx(c: &mut Cpu) {
    use crate::guest::Addr;
    use crate::ps2f::FpuF32;
    use attack_fx::*;
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xf0);
        c.f[1] = 0;
        for (reg, off) in [(S0, 0xe0), (S1, 0xd0)] {
            frame.save(c, reg, off);
        }
        c.g[S0][0] = c.g[A0][0];
        for (reg, off) in [(S2, 0xc0), (S3, 0xb0), (S4, 0xa0)] {
            frame.save(c, reg, off);
        }
        frame.save_ra(c, 0x90);
        let this = Addr(r32(c, S0));
        let owner = this.lw(c);
        c.g[V0][0] = owner;
        let strength = Addr((owner as u32).wrapping_add(R_STRENGTH)).read_fpu(c);
        c.f[0] = strength.0;
        let on = FpuF32(0).lt(strength);
        crate::abi::set_fcc(c, on);
        c.g[V1][0] = 1;
        if !on {
            c.cycles += 1;
            c.g[V1][0] = 0;
        }
        c.cycles += 2;
        this.off(ACTIVE).sw(c, c.g[V1][0]);
        if !on {
            c.cycles += 1;
            frame.restore(c, S0, 0xe0);
        } else {
            // The anim states of tracks 0 and 1, and the punch flags of a
            // type-0xd track-1 sequence (bit 1 clear, bit 0 set).
            c.cycles += 18;
            c.g[A0][0] = Addr((owner as u32).wrapping_add(R_ANIM)).lw(c);
            c.g[A1][0] = 0;
            c.g[S1][0] = Addr((owner as u32).wrapping_add(R_BONE_A)).lw(c);
            c.g[S2][0] = 0;
            call_ptr(c, 0x0031_2aa0, 0x002f_11ac)?;
            c.g[S4][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            let owner = this.lw(c);
            c.g[V0][0] = owner;
            c.g[A0][0] = Addr((owner as u32).wrapping_add(R_ANIM)).lw(c);
            call_ptr(c, 0x0031_2aa0, 0x002f_11c0)?;
            c.g[S3][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            let owner = this.lw(c);
            c.g[V0][0] = owner;
            c.g[A0][0] = Addr((owner as u32).wrapping_add(R_ANIM)).lw(c);
            call_ptr(c, 0x0031_1ae8, 0x002f_11d4)?;
            c.g[V1][0] = SEQ_TYPE_PUNCH;
            let seq_type = c.g[V0][0];
            c.g[V0][0] = PUNCH_A;
            if seq_type == SEQ_TYPE_PUNCH {
                // Flag bit `bit` of the track-1 sequence (`+0xb0`, 64 bits).
                let test_flag = |c: &mut Cpu, bit: u64, ret1: u32, ret2: u32| -> crate::call::Flow {
                    c.cycles += 9;
                    let owner = this.lw(c);
                    c.g[V0][0] = owner;
                    c.g[A1][0] = 1;
                    c.g[A0][0] = Addr((owner as u32).wrapping_add(R_ANIM)).lw(c);
                    call_ptr(c, 0x0031_1b20, ret1)?;
                    c.g[A0][0] = sx(r32(c, V0).wrapping_add(0xb0));
                    c.g[A1][0] = bit;
                    call_ptr(c, 0x0014_46a0, ret2)
                };
                test_flag(c, 1, 0x002f_11f0, 0x002f_11fc)?;
                let bit1 = c.g[V0][0] != 0;
                c.g[V0][0] = PUNCH_A;
                if !bit1 {
                    test_flag(c, 0, 0x002f_1214, 0x002f_1220)?;
                    c.g[S2][0] = (c.g[V0][0] != 0) as u64;
                    c.g[V0][0] = PUNCH_A;
                }
            }
            let (track0, track1, flagged) = (c.g[S4][0], c.g[S3][0], c.g[S2][0] != 0);

            // Punch A?
            c.cycles += 2;
            let mut punch_a = track0 == PUNCH_A;
            c.g[V0][0] = PUNCH_A1;
            if !punch_a {
                c.cycles += 2;
                let t1_match = track1 == PUNCH_A1;
                c.g[V0][0] = PUNCH_B;
                if t1_match {
                    c.cycles += 2;
                    punch_a = flagged;
                }
            }
            if punch_a {
                c.cycles += 4;
                let owner = this.lw(c);
                c.g[A0][0] = owner;
                c.g[V0][0] = Addr((owner as u32).wrapping_add(R_FLIP)).lw(c);
                let bone = if c.g[V0][0] == 0 {
                    R_BONE_B
                } else {
                    c.cycles += 2;
                    R_BONE_A
                };
                c.g[S1][0] = Addr((owner as u32).wrapping_add(bone)).lw(c);
            } else {
                // Punch B?
                c.cycles += 2;
                let mut punch_b = track0 == PUNCH_B;
                c.g[V0][0] = PUNCH_B1;
                if !punch_b {
                    c.cycles += 2;
                    if track1 == PUNCH_B1 {
                        c.cycles += 2;
                        punch_b = flagged;
                    }
                }
                if punch_b {
                    c.cycles += 4;
                    let owner = this.lw(c);
                    c.g[A0][0] = owner;
                    c.g[V0][0] = Addr((owner as u32).wrapping_add(R_FLIP)).lw(c);
                    let bone = if c.g[V0][0] == 0 {
                        R_BONE_A
                    } else {
                        c.cycles += 1;
                        R_BONE_B
                    };
                    c.g[S1][0] = Addr((owner as u32).wrapping_add(bone)).lw(c);
                } else {
                    this.off(ACTIVE).sw(c, 0);
                }
            }

            c.cycles += 3;
            c.g[V0][0] = this.off(ACTIVE).lw(c);
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0xe0);
            } else {
                afx_emit_sparks(c);
                c.cycles += 1;
                frame.restore(c, S0, 0xe0);
            }
        }

        c.cycles += 7;
        for (reg, off) in [(S1, 0xd0), (S2, 0xc0), (S3, 0xb0), (S4, 0xa0)] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0x90);
        frame.close(c);
        Ok(())
    });
}

/// The spark positions, sizes and colours (s0 = self, s1 = bone): the
/// 66-cycle setup and the four 85-cycle iterations. Frame: `base` at
/// 0x00, the bone matrix copy at 0x10, the offset at 0x60, scratch
/// vectors at 0x50/0x70/0x80.
fn afx_emit_sparks(c: &mut Cpu) {
    use crate::guest::Addr;
    use crate::ps2f::FpuF32;
    use attack_fx::*;
    use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
    c.cycles += 66;
    let s = sp(c);
    let owner = Addr(r32(c, S0)).lw(c);
    c.g[A1][0] = owner;
    c.g[A0][0] = sx(r32(c, S1) << 6);
    c.g[A2][0] = sx(s.wrapping_add(0x10));
    // base = bones[bone].row3 (the bone's translation).
    let model = Addr((owner as u32).wrapping_add(R_MODEL)).lw(c);
    c.g[V1][0] = model;
    let bones = Addr((model as u32).wrapping_add(MODEL_BONES)).lw(c);
    c.g[V0][0] = sx((bones as u32).wrapping_add(r32(c, A0)));
    c.lq(V1, r32(c, V0).wrapping_add(0x30));
    c.sq(V1, s);
    // m = bones[bone], with row 3 = D_004FF120.
    let model = Addr((owner as u32).wrapping_add(R_MODEL)).lw(c);
    c.g[V0][0] = model;
    let bones = Addr((model as u32).wrapping_add(MODEL_BONES)).lw(c);
    c.g[V1][0] = bones;
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(bones as u32));
    let mat = r32(c, A0);
    let copy = r32(c, A2);
    for i in 0..4u32 {
        c.lqc2(1 + i as usize, mat.wrapping_add(0x10 * i));
    }
    for i in 0..4u32 {
        c.sqc2(1 + i as usize, copy.wrapping_add(0x10 * i));
    }
    c.g[V0][0] = sx(ORIGIN_ROW);
    c.g[AT][0] = 0x4100_0000;
    c.f[0] = 0x4100_0000; // 8.0
    // The LCG constants: a1 = 0x18fcd, t4 = 0xe9507c, a0 = 0x7fffff,
    // t1 = 1.0f.
    c.g[A1][0] = 0x1_8fcd;
    c.lq(V1, ORIGIN_ROW);
    c.g[T4][0] = 0xe9_507c;
    Addr(s.wrapping_add(0x70)).write_fpu(c, FpuF32(c.f[0]));
    c.g[A0][0] = 0x7f_ffff;
    c.sq(V1, s.wrapping_add(0x40));
    c.g[T1][0] = 0x3f80_0000;
    for off in [0x74, 0x78, 0x7c] {
        Addr(s.wrapping_add(off)).sw(c, 0);
    }
    // rnd = m * (8, 0, 0, 0); base += rnd.
    c.lqc2(8, s.wrapping_add(0x70));
    for i in 0..4u32 {
        c.lqc2(4 + i as usize, copy.wrapping_add(0x10 * i));
    }
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, s.wrapping_add(0x80));
    c.g[AT][0] = 0x4040_0000;
    c.f[9] = 0x4040_0000; // 3.0
    c.lq(V0, s.wrapping_add(0x80));
    c.g[T0][0] = sx(r32(c, S0).wrapping_add(SIZE));
    c.g[AT][0] = 0x4170_0000;
    c.f[7] = 0x4170_0000; // 15.0
    c.g[A3][0] = c.g[S0][0];
    c.sq(V0, s.wrapping_add(0x60));
    c.g[A2][0] = 3;
    c.lqc2(3, s);
    c.lqc2(4, s.wrapping_add(0x60));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s.wrapping_add(0x80));
    c.g[AT][0] = 0x3f80_0000;
    c.f[6] = 0x3f80_0000; // 1.0
    c.lq(V0, s.wrapping_add(0x80));
    c.f[8] = Addr(r32(c, GP).wrapping_sub(COLOUR_SCALE)).read_fpu(c).0;
    c.sq(V0, s);
    c.sq(V0, s.wrapping_add(0x50));

    // Four sparks: a2 counts 3..0, a3 walks self, t0 the sizes.
    loop {
        c.cycles += 85;
        let s = sp(c);
        let (one, fifteen, three) = (FpuF32(c.f[6]), FpuF32(c.f[7]), FpuF32(c.f[9]));
        c.g[V0][0] = Addr(r32(c, GP).wrapping_add(RAND_SEED)).lw(c);
        c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
        let owner = Addr(r32(c, S0)).lw(c);
        c.g[V1][0] = owner;
        afx_mult(c, T2, V0);
        let strength = Addr((owner as u32).wrapping_add(R_STRENGTH)).read_fpu(c);
        Addr(s.wrapping_add(0x6c)).sw(c, 0);
        // Offset range ±3s, size 15 .. 15 + 15s.
        let spread = strength * three;
        let lo = -spread;
        let hi_size = strength * fifteen + fifteen;
        let width = spread - lo;
        afx_bits(c, V0, T2);
        afx_mult(c, T2, V0);
        let rx = afx_store_unit(c, V0);
        afx_bits(c, V1, T2);
        afx_mult(c, T2, V1);
        let ry = afx_store_unit(c, V1);
        afx_bits(c, V0, T2);
        let ox = lo + width * rx;
        let rz = afx_store_unit(c, V0);
        let oy = lo + width * ry;
        Addr(s.wrapping_add(0x60)).write_fpu(c, ox);
        Addr(s.wrapping_add(0x64)).write_fpu(c, oy);
        let oz_part = width * rz;
        let oz = lo + oz_part;
        Addr(s.wrapping_add(0x68)).write_fpu(c, oz);
        // pos[i] = base + offset.
        c.lqc2(3, s);
        c.lqc2(4, s.wrapping_add(0x60));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, s.wrapping_add(0x70));
        c.g[V1][0] = Addr(r32(c, GP).wrapping_add(RAND_SEED)).lw(c);
        c.lq(V0, s.wrapping_add(0x70));
        afx_mult(c, T2, V1);
        c.sq(V0, r32(c, A3).wrapping_add(POS));
        c.sq(V0, s.wrapping_add(0x50));
        // size[i] = 15 + (hi - 15) * rand.
        afx_bits(c, V1, T2);
        let size_range = hi_size - fifteen;
        let rs = afx_store_unit(c, V1);
        afx_mult(c, V0, V1);
        afx_bits(c, V1, V0);
        let size = size_range * rs + fifteen;
        Addr(r32(c, T0)).write_fpu(c, size);
        c.g[T0][0] = sx(r32(c, T0).wrapping_add(4));
        // col[i] = {1, c, c, c}, c = 0.9 s rand.
        let owner = Addr(r32(c, S0)).lw(c);
        c.g[V0][0] = owner;
        let strength = Addr((owner as u32).wrapping_add(R_STRENGTH)).read_fpu(c);
        let scaled = strength * FpuF32(c.f[8]);
        Addr(s.wrapping_add(0x50)).write_fpu(c, one);
        let rc = afx_store_unit(c, V1);
        let tint = scaled * rc;
        for off in [0x5c, 0x54, 0x58] {
            Addr(s.wrapping_add(off)).write_fpu(c, tint);
        }
        c.ldl(V1, s.wrapping_add(0x57));
        c.ldr(V1, s.wrapping_add(0x50));
        c.ldl(T2, s.wrapping_add(0x5f));
        c.ldr(T2, s.wrapping_add(0x58));
        let a3 = r32(c, A3);
        c.sdl(V1, a3.wrapping_add(COL + 7));
        c.sdr(V1, a3.wrapping_add(COL));
        c.sdl(T2, a3.wrapping_add(COL + 0xf));
        c.sdr(T2, a3.wrapping_add(COL + 8));
        // The FPU registers as the original leaves them.
        c.f[0] = rc.0;
        c.f[1] = tint.0;
        c.f[2] = ox.0;
        c.f[3] = oz_part.0;
        c.f[4] = size.0;
        c.f[5] = oz.0;
        let more = (c.g[A2][0] as i64) >= 0;
        c.g[A3][0] = sx(a3.wrapping_add(0x10));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x002f_1398);
            }
            continue;
        }
        break;
    }
}

// ---------------------------------------------------------------------------
// tWPIGD_Weather_Blend (0x2bd698)

/// `tWPIGD_Weather`'s blend: the object (`self`, a0) and the keyframe it
/// blends toward (`target`, a1).
pub mod weather_blend {
    /// `self`: the vtable pointer; the vtable's hook (called before and
    /// after the blend) is `{i16 delta at +0x218, fn at +0x21c}`.
    pub const VPTR: u32 = 0x04;
    pub const VT_HOOK_DELTA: u32 = 0x218;
    pub const VT_HOOK: u32 = 0x21c;
    /// `self`: 19 pairs `{float current, float target}` from +0x08.
    pub const PAIRS: u32 = 0x08;
    /// `target`: the 19 keyframe values from +0x04.
    pub const VALUES: u32 = 0x04;
    /// Return addresses of the two hook calls.
    pub const RET_PRE: u32 = 0x002b_d6d0;
    pub const RET_POST: u32 = 0x002b_d950;
}

/// Call `self`'s vtable hook (`this + delta` in a0) returning to `ret`.
#[inline(always)]
fn wb_hook(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    use crate::guest::Addr;
    use weather_blend::*;
    let vt = r32(c, V1);
    c.g[A0][0] = Addr(vt.wrapping_add(VT_HOOK_DELTA)).lh(c);
    c.g[V0][0] = Addr(vt.wrapping_add(VT_HOOK)).lw(c);
    let target = r32(c, V0);
    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
    call_ptr(c, target, ret)
}

/// One scheduling group of the blend, in the compiled code's memory order:
/// pairs `first ..= first + olds.len()`. The first pair's `t * target` goes
/// through `acc`, its old value through f0; the other pairs' old values are
/// loaded up front into `olds` and pre-scaled by `1 - t` (f3). Then each
/// pair stores `t * target + (1 - t) * current` and copies the target.
/// With `tail`, the last pair's old value (in `olds`' last register) is
/// scaled into f3 and blended through f20 (the compiled code's last pair).
fn wb_group(c: &mut Cpu, first: u32, acc: usize, olds: &[usize], tail: bool) {
    use crate::guest::Addr;
    use crate::ps2f::FpuF32;
    use weather_blend::*;
    let this = Addr(r32(c, S0));
    let key = Addr(r32(c, S1));
    let cur = |n: u32| this.off(PAIRS + 8 * n);
    let val = |n: u32| key.off(VALUES + 4 * n);
    let t = FpuF32(c.f[20]);
    let k = FpuF32(c.f[3]);
    c.f[acc] = val(first).read_fpu(c).0;
    c.f[0] = cur(first).read_fpu(c).0;
    for (i, &r) in olds.iter().enumerate() {
        c.f[r] = cur(first + 1 + i as u32).read_fpu(c).0;
    }
    c.f[acc] = (t * FpuF32(c.f[acc])).0;
    c.f[0] = (k * FpuF32(c.f[0])).0;
    for (i, &r) in olds.iter().enumerate() {
        let dst = if tail && i + 1 == olds.len() { 3 } else { r };
        c.f[dst] = (k * FpuF32(c.f[r])).0;
    }
    c.f[acc] = (FpuF32(c.f[acc]) + FpuF32(c.f[0])).0;
    cur(first).write_fpu(c, FpuF32(c.f[acc]));
    c.f[1] = val(first).read_fpu(c).0;
    cur(first).off(4).write_fpu(c, FpuF32(c.f[1]));
    for (i, &r) in olds.iter().enumerate() {
        let n = first + 1 + i as u32;
        if tail && i + 1 == olds.len() {
            c.f[0] = val(n).read_fpu(c).0;
            c.f[20] = (t * FpuF32(c.f[0]) + FpuF32(c.f[3])).0;
            cur(n).write_fpu(c, FpuF32(c.f[20]));
            c.f[0] = val(n).read_fpu(c).0;
            cur(n).off(4).write_fpu(c, FpuF32(c.f[0]));
        } else {
            c.f[0] = val(n).read_fpu(c).0;
            c.f[0] = (t * FpuF32(c.f[0]) + FpuF32(c.f[r])).0;
            cur(n).write_fpu(c, FpuF32(c.f[0]));
            c.f[1] = val(n).read_fpu(c).0;
            cur(n).off(4).write_fpu(c, FpuF32(c.f[1]));
        }
    }
}

/// `void tWPIGD_Weather::Blend(const tWPIGD_Weather* target, float t)`
/// (0x2bd698): blends the weather's 19 settings toward a keyframe with the
/// eased weight `t^2`: each pair `{current, target}` (+0x08 + 8i) gets
/// `current = t^2 * key[i] + (1 - t^2) * current` and `target = key[i]`
/// (`key` from target+0x04). Calls its vtable hook (+0x218/+0x21c) before
/// and after. Frame 0x40 (s0, s1, f20); 180 cycles charged at entry.
pub fn twpigd_weather_blend(c: &mut Cpu) {
    use crate::guest::Addr;
    use crate::ps2f::FpuF32;
    use weather_blend::*;
    guest(c, |c| {
        c.cycles += 180;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x30);
        let t = FpuF32(c.f[12]);
        c.f[20] = t.0;
        c.g[V1][0] = Addr(r32(c, S0).wrapping_add(VPTR)).lw(c);
        c.f[20] = (t * t).0;
        wb_hook(c, RET_PRE)?;
        c.g[AT][0] = 0x3f80_0000;
        c.f[3] = (FpuF32::ONE - FpuF32(c.f[20])).0;
        // Pairs 0..=6 (+0x08..+0x38), 7..=13 (+0x40..+0x70), 14..=18
        // (+0x78..+0x98), as the compiler grouped them.
        wb_group(c, 0, 1, &[2, 4, 5, 6, 7, 8], false);
        wb_group(c, 7, 2, &[4, 5, 6, 7, 8, 9], false);
        wb_group(c, 14, 2, &[4, 5, 6, 1], true);
        c.g[V1][0] = Addr(r32(c, S0).wrapping_add(VPTR)).lw(c);
        wb_hook(c, RET_POST)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// FocusMan_CollectNodeObjects (0x101a28)

/// The focus manager's spatial tree and the object list it fills.
pub mod focus_collect {
    /// Tree node: eight child nodes (`kids[0..8]`, +0x00..+0x1c), then the
    /// head of the node's object chain.
    pub const KIDS: u32 = 0x00;
    pub const KID_COUNT: u32 = 8;
    pub const ITEMS: u32 = 0x20;
    /// Object link: the next object of the chain.
    pub const NEXT: u32 = 0x00;
    /// Out list: `int n; Obj* items[]`.
    pub const LIST_N: u32 = 0x00;
    pub const LIST_ITEMS: u32 = 0x04;
    /// `gp+0xce8` (`D_004A3DD8`): the world-script manager.
    pub const SCRIPT_MAN: u32 = 0xce8;
    /// `WScriptMan_IsScriptedInstance(man, obj)`.
    pub const IS_SCRIPTED: u32 = 0x0030_a310;
    /// Return addresses: the test call, then each child's recursion.
    pub const RET_TEST: u32 = 0x0010_1a68;
    pub const RET_KID: [u32; 8] = [
        0x0010_1ab0,
        0x0010_1ac4,
        0x0010_1ad8,
        0x0010_1aec,
        0x0010_1b00,
        0x0010_1b14,
        0x0010_1b28,
        0x0010_1b3c,
    ];
    /// The object loop's head (event checks at its back edge).
    pub const LOOP_HEAD: u32 = 0x0010_1a60;
}

/// `void FocusMan_CollectNodeObjects(ObjList* list (a0), FocusNode* node
/// (a1), int recurse (a2))` (0x101a28): appends every object of the node's
/// chain that the world-script manager (`gp+0xce8`) reports as a scripted
/// instance (`WScriptMan_IsScriptedInstance`) to `list` (`items[n++]`);
/// with `recurse`, then walks the eight child nodes the same way (always
/// recursing). Frame 0x60 (s0-s4).
pub fn focus_man_collect_node_objects(c: &mut Cpu) {
    use crate::guest::Addr;
    use focus_collect::*;
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save(c, S4, 0x10);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x50);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        c.g[S4][0] = c.g[A2][0];
        let list = r32(c, S1);
        let node = r32(c, S2);
        let script_man = Addr(r32(c, GP).wrapping_add(SCRIPT_MAN));

        // The node's own objects.
        c.g[S0][0] = Addr(node.wrapping_add(ITEMS)).lw(c);
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[S3][0] = sx(list.wrapping_add(LIST_ITEMS));
            c.g[A0][0] = script_man.lw(c);
            loop {
                c.cycles += 4;
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, IS_SCRIPTED, RET_TEST)?;
                let obj = r32(c, S0);
                if c.g[V0][0] != 0 {
                    c.cycles += 7;
                    let n = Addr(list.wrapping_add(LIST_N)).lw(c);
                    c.g[V1][0] = n;
                    let slot = r32(c, S3).wrapping_add((n as u32) << 2);
                    c.g[V0][0] = sx(slot);
                    c.g[V1][0] = sx((n as u32).wrapping_add(1));
                    Addr(slot).sw(c, c.g[S0][0]);
                    Addr(list.wrapping_add(LIST_N)).sw(c, c.g[V1][0]);
                }
                c.g[S0][0] = Addr(obj.wrapping_add(NEXT)).lw(c);
                c.cycles += 2;
                c.g[A0][0] = script_man.lw(c);
                if c.g[S0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, LOOP_HEAD);
                }
            }
        }

        // The children.
        c.cycles += 2;
        if c.g[S4][0] != 0 {
            let me = Callee {
                addr: 0x0010_1a28,
                f: ee::s00::f_00101a28,
            };
            for i in 0..KID_COUNT {
                c.cycles += 3;
                let kid = Addr(node.wrapping_add(KIDS + 4 * i)).lw(c);
                c.g[A1][0] = kid;
                c.g[A0][0] = c.g[S1][0];
                if kid != 0 {
                    c.cycles += 2;
                    c.g[A2][0] = 1;
                    me.call(c, RET_KID[i as usize])?;
                }
            }
            c.cycles += 1;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `HUD_DrawMeterBar`: a HUD object whose sprite table (+0x4fc, style
    /// 0..3) points at three of four sprites with random UVs; layout
    /// elements -1..4 (the element byte comes with junk upper bits) with
    /// small positions, heights 8..300 (sometimes negative: no texture, so
    /// 0), scales 0.25..2 and random align/blend bytes; segment scale
    /// 0.5..1.5 and count -1..5; colour, align flags, an out rect on 3 of 4
    /// seeds; fill (f12) exactly 1 (full bar) on every third seed, else
    /// 0..1.1 (the partial bar's caps and segments clipped or not, the
    /// segment loop ending at the fill line or at the count); sx/sy (f13,
    /// f14) 0.5..2. The renderer at gp-0x854 has a render-state record and
    /// a vtable whose `DrawPrims` (+0x238) is a fake that logs every
    /// register and sometimes unwinds. The HUD layout callees run for real.
    #[test]
    fn hud_draw_meter_bar_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        use crate::difftest::{DATA, GAME_GP};
        const HUD: u32 = DATA;
        const ELEMS: u32 = DATA + 0x1000;
        const SEG: u32 = DATA + 0x1400;
        const COLOUR: u32 = DATA + 0x1410;
        const RECT: u32 = DATA + 0x1420;
        const SPRITE: u32 = DATA + 0x1500;
        const RENDERER: u32 = DATA + 0x2000;
        const STATE: u32 = DATA + 0x4000;
        const VT: u32 = DATA + 0x4100;
        covered(ee::s05::f_0021d1a0__generated, hud_draw_meter_bar, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x21d1a0, 8);
            let unit = |r: &mut crate::difftest::Rng, lo: f32, steps: u32, step: f32| {
                (lo + (r.next_u32() % steps) as f32 * step).to_bits()
            };
            // Sprites: id, v0, u0, u1, v1.
            for k in 0..4 {
                let s = SPRITE + 0x20 * k;
                c.bus.write32(s, r.next_u32());
                for off in [0xc, 0x10, 0x14, 0x18] {
                    c.bus.write32(s + off, r.f32(1.0));
                }
            }
            for style in 0..4 {
                for i in 0..3 {
                    let p = SPRITE + 0x20 * (r.next_u32() % 4);
                    c.bus.write32(HUD + meter_bar::SPRITES + 0xc * style + 4 * i, p);
                }
            }
            // Layout elements -1..4.
            for i in 0..6u32 {
                let e = ELEMS - 0x24 + 0x24 * i;
                c.bus.write16(e, (r.next_u32() % 640) as u16);
                c.bus.write16(e + 2, (r.next_u32() % 448) as u16);
                c.bus.write16(e + 4, (1 + r.next_u32() % 64) as u16);
                let h = if r.next_u32() % 8 == 0 {
                    -((r.next_u32() % 16) as i16)
                } else {
                    (8 + r.next_u32() % 293) as i16
                };
                c.bus.write16(e + 6, h as u16);
                for off in [8, 0xc] {
                    let s = unit(&mut r, 0.25, 8, 0.25);
                    c.bus.write32(e + off, s);
                }
                for off in [0x10, 0x14, 0x18, 0x1c] {
                    c.bus.write32(e + off, r.f32(1.0));
                }
                c.bus.write8(e + 0x20, (r.next_u32() % 4) as u8);
                c.bus.write8(e + 0x21, (r.next_u32() % 4) as u8);
                c.bus.write8(e + 0x22, r.next_u32() as u8);
            }
            let seg_scale = unit(&mut r, 0.5, 5, 0.25);
            c.bus.write32(SEG, seg_scale);
            c.bus.write32(SEG + 4, ((r.next_u32() % 7) as i32 - 1) as u32);
            for i in 0..4 {
                c.bus.write32(COLOUR + 4 * i, r.f32(1.0));
            }
            // The renderer: render state and DrawPrims.
            c.bus.write32(GAME_GP - meter_bar::RENDERER, RENDERER);
            c.bus.write32(RENDERER + meter_bar::STATE_TOP, STATE);
            c.bus.write32(STATE + meter_bar::STATE_FLAGS, r.next_u32());
            c.bus.write32(RENDERER + meter_bar::VPTR, VT);
            c.bus.write16(VT + meter_bar::DRAW_PRIMS, ((r.next_u32() % 16) * 4) as u16);
            c.bus.write32(VT + meter_bar::DRAW_PRIMS + 4, FAKE);
            // Arguments.
            let idx = (r.next_u32() % 6) as i32 - 1;
            c.g[A0][0] = HUD as u64;
            c.g[A1][0] = ELEMS as u64;
            c.g[A2][0] = (r.next_u32() & !0xff) as u64 | (idx as u8) as u64;
            c.g[A3][0] = (r.next_u32() % 4) as u64;
            c.g[T0][0] = SEG as u64;
            c.g[T1][0] = COLOUR as u64;
            c.g[T2][0] = (r.next_u32() % 4) as u64;
            c.g[T3][0] = if seed % 4 == 0 { 0 } else { RECT as u64 };
            c.f[12] = if seed % 3 == 0 {
                0x3f80_0000
            } else {
                ((r.next_u32() % 1101) as f32 / 1000.0).to_bits()
            };
            c.f[13] = unit(&mut r, 0.5, 7, 0.25);
            c.f[14] = unit(&mut r, 0.5, 7, 0.25);
        });
    }

    /// `RiderBase_UpdateAttackFX`: the effect at DATA, its rider with an
    /// attack strength that is 0, -0, negative or denormal on one seed in
    /// six (the effect switches off at once) and else 0.05..2; the rider's
    /// anim controller (real callees) with track 0 in {punch A, punch B,
    /// punch A1, other} and track 1 in {A1, B1, 0x1b6 (no table entry),
    /// other}, the anim table type of those states 0xd on two seeds in
    /// three, the track-1 sequence's 64-bit flags random (bits 0 and 1 pick
    /// the flagged track-1 punches); the facing flag set or clear, bones
    /// 0..3 into four random matrices, `D_004FF120`, the 0.9 colour scale
    /// and a random seed. Covers both hands of both punches, the switched
    /// off fall-through and the four-spark loop.
    #[test]
    fn rider_base_update_attack_fx_matches() {
        use crate::difftest::{check_lohi, fill, scramble, Rng, DATA, GAME_GP, SP};
        use attack_fx::*;
        const FX: u32 = DATA;
        const RIDER: u32 = DATA + 0x1000;
        const CTL: u32 = DATA + 0x2000;
        const SEQS: u32 = DATA + 0x2100;
        const SEQ: u32 = DATA + 0x2200;
        const MODEL: u32 = DATA + 0x3000;
        const BONES: u32 = DATA + 0x3100;
        /// `AnimTable_GetEntry`'s table (0x1c bytes per state).
        const ANIM_TABLE: u32 = 0x0044_6990;
        check_lohi(ee::s09::f_002f1150__generated, rider_base_update_attack_fx, 300, &|c, seed| {
            let mut r = Rng(0x2f11_5000_1234 ^ (seed as u64) << 20);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            fill(c, &mut r, SP - 0x100, 0x40);
            fill(c, &mut r, FX, 0x30);
            c.bus.write32(FX + OWNER, RIDER);
            c.g[A0][0] = FX as u64;
            // The rider.
            let strength = if seed % 6 == 0 {
                [0, 0x8000_0000, 0xbf80_0000, 0x0000_0123][(r.next_u32() % 4) as usize]
            } else {
                (0.05 + (r.next_u32() % 196) as f32 * 0.01).to_bits()
            };
            c.bus.write32(RIDER + R_STRENGTH, strength);
            c.bus.write32(RIDER + R_FLIP, if r.next_u32() % 2 == 0 { 0 } else { r.next_u32() | 1 });
            c.bus.write32(RIDER + R_BONE_A, r.next_u32() % 4);
            c.bus.write32(RIDER + R_BONE_B, r.next_u32() % 4);
            c.bus.write32(RIDER + R_MODEL, MODEL);
            c.bus.write32(MODEL + MODEL_BONES, BONES);
            for i in 0..64 {
                c.bus.write32(BONES + 4 * i, r.f32(10.0));
            }
            for i in 0..4 {
                c.bus.write32(ORIGIN_ROW + 4 * i, r.f32(2.0));
            }
            c.bus.write32(GAME_GP - COLOUR_SCALE, 0.9f32.to_bits());
            c.bus.write32(GAME_GP + RAND_SEED, r.next_u32());
            // The anim controller: track states, the anim table, the
            // track-1 sequence's flags.
            c.bus.write32(RIDER + R_ANIM, CTL);
            let track0 = [PUNCH_A, PUNCH_B, PUNCH_A1, 0x10][(r.next_u32() % 4) as usize];
            let track1 = [PUNCH_A1, PUNCH_B1, 0x1b6, 0x20][(r.next_u32() % 4) as usize];
            c.bus.write32(CTL, track0 as u32);
            c.bus.write32(CTL + 4, track1 as u32);
            for state in [PUNCH_A1, PUNCH_B1, 0x20] {
                let ty = if r.next_u32() % 3 != 0 { SEQ_TYPE_PUNCH as u32 } else { r.next_u32() % 0x20 };
                c.bus.write32(ANIM_TABLE + 0x1c * state as u32, ty);
            }
            c.bus.write32(CTL + 0x50, SEQS);
            c.bus.write32(SEQS + 8 + 4, SEQ);
            let flags = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            c.bus.write64(SEQ + 0xb0, flags);
        });
    }

    /// `tWPIGD_Weather_Blend`: the weather at a random data address with
    /// 19 random `{current, target}` pairs (an edge-case float on one pair
    /// in four), a keyframe with random values (on one seed in seven it
    /// overlaps the weather's pairs, which checks the load/store order),
    /// t (f12) in [-1.2, 1.2) or an edge case on one seed in five, and a
    /// vtable whose hook is a fake (random this-delta) that unwinds on
    /// either of the two calls on some seeds.
    #[test]
    fn twpigd_weather_blend_matches() {
        use crate::difftest::fakes::{covered, machine, ptr, FAKE};
        use weather_blend::*;
        covered(ee::s08::f_002bd698__generated, twpigd_weather_blend, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x2bd698, 2);
            c.pc = 0x002b_d698;
            let this = ptr(&mut r) & !0xfff;
            for i in 0..38 {
                let v = if r.next_u32() % 4 == 0 { r.edge_f32() } else { r.f32(500.0) };
                c.bus.write32(this + PAIRS + 4 * i, v);
            }
            let key = if seed % 7 == 0 {
                this + 4 * (r.next_u32() % 16)
            } else {
                let k = this + 0x400;
                for i in 0..19 {
                    let v = if r.next_u32() % 8 == 0 { r.edge_f32() } else { r.f32(500.0) };
                    c.bus.write32(k + VALUES + 4 * i, v);
                }
                k
            };
            // The vtable clear of the blended data (the blend cannot
            // overwrite its hook slot before the second call).
            let vt = this + 0x800;
            c.bus.write16(vt + VT_HOOK_DELTA, (r.next_u32() as i16 & !3) as u16);
            c.bus.write32(vt + VT_HOOK, FAKE);
            c.bus.write32(this + VPTR, vt);
            c.g[A0][0] = this as u64;
            c.g[A1][0] = key as u64;
            c.f[12] = if seed % 5 == 0 { r.edge_f32() } else { r.f32(1.2) };
        });
    }

    /// `FocusMan_CollectNodeObjects`: a random tree of up to 10 nodes (kid
    /// slots null or a later node, so no cycles; the root sometimes with no
    /// objects), each with a chain of 0..4 objects. The real
    /// `WScriptMan_IsScriptedInstance` runs on its short paths: an object
    /// with a script (+0xc != 0) is scripted, one without and a type entry
    /// (`man+0x28c -> +4 -> +4`, 12 bytes per `obj+0x78`) other than 3 is
    /// not. `recurse` is 0 on every fourth seed (junk upper bits otherwise);
    /// the list starts with a random count 0..8. Recursion goes through the
    /// generated body on both sides.
    #[test]
    fn focus_man_collect_node_objects_matches() {
        use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
        use focus_collect::*;
        const LIST: u32 = DATA;
        const NODES: u32 = DATA + 0x1000;
        const OBJS: u32 = DATA + 0x2000;
        const MAN: u32 = DATA + 0x8000;
        const TYPES: u32 = DATA + 0x8400;
        check(ee::s00::f_00101a28__generated, focus_man_collect_node_objects, 300, &|c, seed| {
            let mut r = Rng(0x101a28 ^ (seed as u64) << 20);
            scramble(c, &mut r);
            c.pc = 0x0010_1a28;
            c.g[GP][0] = sx(GAME_GP);
            // Script manager: man+0x28c -> A, A+4 -> B, B+4 -> type table.
            c.bus.write32(GAME_GP + SCRIPT_MAN, MAN);
            c.bus.write32(MAN + 0x28c, MAN + 0x100);
            c.bus.write32(MAN + 0x104, MAN + 0x200);
            c.bus.write32(MAN + 0x204, TYPES);
            for t in 0..8u32 {
                let ty = if r.next_u32() % 2 == 0 { 0 } else { 1 + r.next_u32() % 2 };
                c.bus.write16(TYPES + 12 * t, ty as u16);
            }
            // List.
            c.bus.write32(LIST + LIST_N, r.next_u32() % 8);
            // Nodes and their object chains.
            let count = 1 + r.next_u32() % 10;
            let mut obj = OBJS;
            for k in 0..count {
                let n = NODES + 0x40 * k;
                for i in 0..KID_COUNT {
                    let later = k + 1 + r.next_u32() % 10;
                    let kid = if later < count && r.next_u32() % 3 == 0 {
                        NODES + 0x40 * later
                    } else {
                        0
                    };
                    c.bus.write32(n + KIDS + 4 * i, kid);
                }
                let chain = if k == 0 && seed % 5 == 0 { 0 } else { r.next_u32() % 5 };
                let mut link = n + ITEMS;
                for _ in 0..chain {
                    c.bus.write32(link, obj);
                    let script = if r.next_u32() % 2 == 0 { 0 } else { r.next_u32() | 1 };
                    c.bus.write32(obj + 0xc, script);
                    c.bus.write8(obj + 0x78, (r.next_u32() % 8) as u8);
                    link = obj + NEXT;
                    obj += 0x80;
                }
                c.bus.write32(link, 0);
            }
            c.g[A0][0] = sx(LIST);
            c.g[A1][0] = sx(NODES);
            c.g[A2][0] = if seed % 4 == 0 {
                0
            } else {
                (r.next_u32() as u64) << 32 | (1 + r.next_u32() % 4) as u64
            };
        });
    }
}
