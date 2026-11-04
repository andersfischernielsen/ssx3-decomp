//! `src/misc.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::visualfx::lensfx::at;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002f_4db8,
        lens_fx_update_sun_position,
        "LensFX_UpdateSunPosition",
    ),
    (0x0021_d1a0, hud_draw_meter_bar, "HUD_DrawMeterBar"),
    (0x0010_8a48, rider_find_rail, "Rider_FindRail"),
    (0x0021_e750, hud_align_x, "HUD_AlignX"),
];

// ---------------------------------------------------------------------------
// Misc newlib: allocator glue and the cache-range loops.

pub(crate) const MALLOC_R: Callee = Callee {
    addr: 0x0041_dbf8,
    f: ssx3_ee::s15::f_0041dbf8,
};

pub(crate) const MEMSET: Callee = Callee {
    addr: 0x0041_6210,
    f: ssx3_ee::s15::f_00416210,
};

/// `struct glue *_sfmoreglue(struct _reent *r, int n)` (0x41ff98): allocate
/// `n * 0x58 + 0xc` for a `FILE` glue record and zero its data area.
pub fn sfmoreglue(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x58;
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        c.mult(0, S2, S1, V0); // s2 = n * 0x58, LO/HI
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
        MALLOC_R.call(c, 0x0041_ffc0)?;
        c.g[S0][0] = c.g[V0][0];
        let null = c.g[S0][0] == 0;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
        if null {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 9;
            Addr(c.g[S0][0] as u32).off(0x4).sw(c, c.g[S1][0]);
            Addr(c.g[S0][0] as u32).sw(c, 0);
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S2][0];
            Addr(c.g[S0][0] as u32).off(0x8).sw(c, c.g[V0][0]);
            c.g[A1][0] = 0;
            MEMSET.call(c, 0x0041_ffe8)?;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void *_calloc_r(struct _reent *r, size_t n, size_t size)` (0x4224a0):
/// `malloc(n * size)` then zero it (inline for small blocks, `memset`
/// otherwise).
pub fn calloc_r(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 10;
        c.mult(0, 0, A1, A2); // LO/HI = n * size
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lo[0];
        MALLOC_R.call(c, 0x0042_24bc)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_sub(4)).lw(c);
            c.cycles += 7;
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[V0][0] |= 0xfffc;
            c.g[V1][0] &= c.g[V0][0];
            c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(4));
            c.g[V0][0] = (c.g[A2][0] < 0x25) as u64;
            let large = c.g[V0][0] == 0;
            c.g[V0][0] = (c.g[A2][0] < 0x14) as u64;
            if large {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                MEMSET.call(c, 0x0042_2548)?;
            } else {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[A0][0] = c.g[S0][0];
                } else {
                    c.cycles += 5;
                    Addr(c.g[S0][0] as u32).sw(c, 0);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = (c.g[A2][0] < 0x1c) as u64;
                    let t = c.g[V0][0] != 0;
                    Addr(c.g[S0][0] as u32).off(0x4).sw(c, 0);
                    if !t {
                        c.cycles += 5;
                        c.g[V0][0] = (c.g[A2][0] < 0x24) as u64;
                        Addr(c.g[S0][0] as u32).off(0x8).sw(c, 0);
                        Addr(c.g[S0][0] as u32).off(0xc).sw(c, 0);
                        let t = c.g[V0][0] != 0;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                        if !t {
                            c.cycles += 3;
                            Addr(c.g[S0][0] as u32).off(0x10).sw(c, 0);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                            Addr(c.g[S0][0] as u32).off(0x14).sw(c, 0);
                        }
                    }
                }
                c.cycles += 5;
                Addr(c.g[A0][0] as u32).sw(c, 0);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                Addr(c.g[A0][0] as u32).off(0x4).sw(c, 0);
                Addr(c.g[A0][0] as u32).sw(c, 0);
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod meter_bar {

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
    pub const SEGMENT_H: u32 = 0x41b8_0000;
    // 23.0
    pub const TOP_CAP_H: u32 = 0x41f8_0000;
}

/// `f[n]` as an FPU float.
#[inline(always)]
pub(crate) fn mb_f(c: &Cpu, n: usize) -> crate::ps2f::FpuF32 {
    crate::ps2f::FpuF32(c.f[n])
}

/// `f[n] = v`.
#[inline(always)]
pub(crate) fn mb_set(c: &mut Cpu, n: usize, v: crate::ps2f::FpuF32) {
    c.f[n] = v.0;
}

/// `lwc1` from the frame.
#[inline(always)]
pub(crate) fn mb_ld(c: &mut Cpu, off: u32) -> crate::ps2f::FpuF32 {
    crate::guest::Addr(sp(c).wrapping_add(off)).read_fpu(c)
}

/// `swc1` to the frame.
#[inline(always)]
pub(crate) fn mb_st(c: &mut Cpu, off: u32, v: crate::ps2f::FpuF32) {
    crate::guest::Addr(sp(c).wrapping_add(off)).write_fpu(c, v)
}

/// `lwc1` of a sprite field.
#[inline(always)]
pub(crate) fn mb_tex(c: &mut Cpu, tex: u64, off: u32) -> crate::ps2f::FpuF32 {
    crate::guest::Addr((tex as u32).wrapping_add(off)).read_fpu(c)
}

/// The renderer (`lw gp-0x854`).
#[inline(always)]
pub(crate) fn mb_renderer(c: &mut Cpu) -> u64 {
    crate::guest::Addr(r32(c, GP).wrapping_sub(meter_bar::RENDERER)).lw(c)
}

/// The quad's texture coordinates from sprite `tex`, the top edge at
/// `v_top` (the sprite's v0, or clipped). Leaves `f0 = v1`, `f1 = u1`, as
/// the original's alternating loads do.
pub(crate) fn mb_put_uvs(c: &mut Cpu, tex: u64, v_top: crate::ps2f::FpuF32) {
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
pub(crate) fn mb_put_xy(
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
pub(crate) fn mb_bind(c: &mut Cpu, renderer: u64, tex: u64, state_reg: usize, id_reg: usize) {
    use meter_bar::*;
    let id = crate::guest::Addr((tex as u32).wrapping_add(TEX_ID)).lw(c);
    c.g[id_reg][0] = id;
    let state = crate::guest::Addr((renderer as u32).wrapping_add(STATE_TOP)).lw(c);
    c.g[state_reg][0] = state;
    crate::guest::Addr((state as u32).wrapping_add(STATE_TEX)).sh(c, id);
}

/// `renderer->DrawPrims(4, quad, 0)` returning to `ret` (t0 = renderer,
/// v0 = vptr, v1 = method, a0 = adjusted this).
pub(crate) fn mb_draw_quad(c: &mut Cpu, ret_to: u32) -> crate::call::Flow {
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
pub(crate) fn mb_clip_cap(c: &mut Cpu, tex: u64, top_reg: usize) {
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
pub(crate) fn mb_draw_partial_cap(
    c: &mut Cpu,
    tex: u64,
    top_reg: usize,
    ret_to: u32,
) -> crate::call::Flow {
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

pub(crate) mod rail_rider {

    /// Velocity (`vec4`).
    pub const VELOCITY: u32 = 0x1e0;

    /// The rider's path (`+0x2c`: nodes, 32 bytes each).
    pub const PATH: u32 = 0x780;

    /// The animation controller.
    pub const ANIM: u32 = 0x784;

    /// The overlap list of objects near the rider.
    pub const OVERLAP: u32 = 0x860;

    /// Current path node index.
    pub const NODE: u32 = 0x8a0;

    /// Offset from the path node to the rider's probe point (`vec4`).
    pub const PROBE_OFFSET: u32 = 0x9d0;
}

// ---------------------------------------------------------------------------
// Rider_FindRail (0x108a48, seg_1218.cpp).

/// `Rider_GetMainState(rider)`.
pub(crate) const RIDER_GET_MAIN_STATE: Callee = Callee {
    addr: 0x0011_fe98,
    f: ee::s00::Rider_GetMainState,
};

/// `Rider_ProbeRail(rider, pos, pos + 0x10)`.
pub(crate) const RIDER_PROBE_RAIL: Callee = Callee {
    addr: 0x0010_86b8,
    f: ee::s00::Rider_ProbeRail,
};

/// `OverlapList_FindNearestRail(list, pos, out, mask, radius)` (cross-shard).
pub(crate) const OVERLAP_LIST_FIND_NEAREST_RAIL: u32 = 0x0033_4680;

/// `AnimController_GetLayerAnimEntry(ctrl, layer)` (cross-shard).
pub(crate) const ANIM_GET_LAYER_ANIM_ENTRY: u32 = 0x0031_1ae8;

/// `AnimController_GetLayerSequence(ctrl, layer)` (cross-shard).
pub(crate) const ANIM_GET_LAYER_SEQUENCE: u32 = 0x0031_1b20;

/// `Flags64_Test(flags, bit)` (cross-shard).
pub(crate) const FLAGS64_TEST: u32 = 0x0014_46a0;

/// `int Rider_FindRail(Rider* self, sVec4 out[2])` (0x108a48): in the main
/// states 0/1, look for a rail near the rider's probe point (path node +
/// offset, radius 300). Rejects a rail behind the rider (closer than -0.2 x
/// speed along the velocity) and, when the animation on layer 2 is a grab
/// (kind 0x12) or a trick (0x13/0x14), a rail the sequence's flags rule out;
/// otherwise returns `Rider_ProbeRail(self, out, out + 1)`. Returns 0 when
/// there is none.
pub fn rider_find_rail(c: &mut Cpu) {
    use rail_rider::*;
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        c.g[S1][0] = c.g[A1][0];
        let me = c.g[S0][0] as u32;
        let lw = |c: &mut Cpu, a: u32| c.lw(a);
        RIDER_GET_MAIN_STATE.call(c, 0x0010_8a64)?;
        'out: {
            if c.g[V0][0] != 0 {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                RIDER_GET_MAIN_STATE.call(c, 0x0010_8a74)?;
                c.g[V1][0] = 1;
                let other = c.g[V0][0] != 1;
                c.g[V0][0] = 0;
                if other {
                    break 'out;
                }
                c.cycles += 1;
            }
            c.g[A0][0] = lw(c, me.wrapping_add(PATH));

            // 0x108a84: probe point = path node + offset; nearest rail.
            c.cycles += 19;
            c.g[A1][0] = c.g[SP][0];
            c.g[V0][0] = lw(c, me.wrapping_add(NODE));
            c.g[A2][0] = c.g[S1][0];
            c.g[V1][0] = lw(c, (c.g[A0][0] as u32).wrapping_add(0x2c));
            c.g[A3][0] = 1;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
            c.g[AT][0] = 0x4396_0000; // 300.0
            c.mtc1(1, 12);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.lqc2(3, c.g[V1][0] as u32);
            c.lqc2(4, me.wrapping_add(PROBE_OFFSET));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, sp(c).wrapping_add(0x10));
            c.lq(2, sp(c).wrapping_add(0x10));
            c.g[A0][0] = lw(c, me.wrapping_add(OVERLAP));
            c.sq(2, sp(c));
            call_ptr(c, OVERLAP_LIST_FIND_NEAREST_RAIL, 0x0010_8ac8)?;
            let none = c.g[V0][0] == 0;
            c.g[V0][0] = 0;
            if none {
                break 'out;
            }

            // 0x108ad0: |velocity| (VU0 length).
            c.cycles += 15;
            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7e38));
            c.lqc2(3, me.wrapping_add(VELOCITY));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(2, 22);
            c.mtc1(2, 2);
            c.c_lt_s(0, 2);
            if c.fcc() {
                // 0x108b0c: moving: reject a rail behind the rider.
                c.cycles += 21;
                let out = c.g[S1][0] as u32;
                c.lqc2(3, out);
                c.lqc2(4, sp(c));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp(c).wrapping_add(0x20));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x7e34));
                c.lq(2, sp(c).wrapping_add(0x20));
                c.mul_s(1, 2, 1);
                c.sq(2, sp(c).wrapping_add(0x10));
                c.lqc2(3, sp(c).wrapping_add(0x10));
                c.lqc2(5, me.wrapping_add(VELOCITY));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(3, 4);
                c.mtc1(3, 0);
                c.c_lt_s(0, 1);
                let behind = c.fcc();
                c.g[V0][0] = 0;
                if behind {
                    break 'out;
                }
                c.cycles += 1;
            }
            c.g[A0][0] = lw(c, me.wrapping_add(ANIM));

            // 0x108b64: the animation on layer 2 may forbid the rail.
            c.cycles += 5;
            c.g[A1][0] = 2;
            call_ptr(c, ANIM_GET_LAYER_ANIM_ENTRY, 0x0010_8b6c)?;
            c.g[V1][0] = 0x12;
            let grab = c.g[V0][0] != 0x12;
            c.g[A0][0] = lw(c, me.wrapping_add(ANIM));
            let probe;
            if !grab {
                // Kind 0x12: allowed when flag 2 is set or flag 0 is clear.
                c.cycles += 7;
                c.g[A1][0] = 2;
                call_ptr(c, ANIM_GET_LAYER_SEQUENCE, 0x0010_8b80)?;
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xb0));
                c.g[A1][0] = 2;
                call_ptr(c, FLAGS64_TEST, 0x0010_8b8c)?;
                let flag2 = c.g[V0][0] != 0;
                c.g[A1][0] = c.g[S1][0];
                if flag2 {
                    probe = true;
                } else {
                    c.cycles += 8;
                    c.g[A0][0] = lw(c, me.wrapping_add(ANIM));
                    c.g[A1][0] = 2;
                    call_ptr(c, ANIM_GET_LAYER_SEQUENCE, 0x0010_8ba0)?;
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xb0));
                    c.g[A1][0] = 0;
                    call_ptr(c, FLAGS64_TEST, 0x0010_8bac)?;
                    probe = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[S1][0];
                }
            } else {
                // 0x108bbc: kinds 0x13/0x14 need flag 2.
                c.cycles += 5;
                c.g[A1][0] = 2;
                call_ptr(c, ANIM_GET_LAYER_ANIM_ENTRY, 0x0010_8bc4)?;
                c.g[V1][0] = 0x13;
                let is_13 = c.g[V0][0] == 0x13;
                c.g[A0][0] = lw(c, me.wrapping_add(ANIM));
                let mut check = true;
                if !is_13 {
                    c.cycles += 5;
                    c.g[A1][0] = 2;
                    call_ptr(c, ANIM_GET_LAYER_ANIM_ENTRY, 0x0010_8bd8)?;
                    c.g[V1][0] = 0x14;
                    check = c.g[V0][0] == 0x14;
                    c.g[A1][0] = c.g[S1][0];
                    if check {
                        c.cycles += 1;
                        c.g[A0][0] = lw(c, me.wrapping_add(ANIM));
                    }
                }
                if check {
                    // 0x108be8
                    c.cycles += 7;
                    c.g[A1][0] = 2;
                    call_ptr(c, ANIM_GET_LAYER_SEQUENCE, 0x0010_8bf0)?;
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xb0));
                    c.g[A1][0] = 2;
                    call_ptr(c, FLAGS64_TEST, 0x0010_8bfc)?;
                    probe = c.g[V0][0] != 0;
                    c.g[A1][0] = c.g[S1][0];
                    // The branch back to 0x108bb4 is a back edge: an event
                    // may be due there (and may unwind; re-entry at 0x108bb4
                    // runs the generated code).
                    if !probe && crate::events::back_edge_unwinds(c, 0x0010_8bb4) {
                        return Err(crate::call::Unwound);
                    }
                } else {
                    probe = true;
                }
            }
            if probe {
                // 0x108c04
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
                RIDER_PROBE_RAIL.call(c, 0x0010_8c10)?;
            } else {
                // 0x108bb4
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
        }
        // 0x108c10
        c.cycles += 5;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// HUD_AlignX (0x21e750, ovstatehud.cpp).

/// The HUD's virtual screen width (640.0).
pub(crate) const HUD_SCREEN_WIDTH: u32 = 0x4420_0000;

/// 0.5.
pub(crate) const HALF: u32 = 0x3f00_0000;

/// `float HUD_AlignX(float x, int align, int flags, float width, float
/// scale)` (0x21e750): the left edge of an element of `width * scale` at
/// `x`. Flag 1 mirrors the layout (`x = 640 - x`, and the alignment flips
/// unless flag 2 is set); alignment 0 centres the element on `x`, positive
/// alignment right-aligns it, negative leaves `x` as the left edge.
pub fn hud_align_x(c: &mut Cpu) {
    use crate::ps2f::FpuF32;
    c.cycles += 3;
    let align = c.g[A1][0];
    let flags = c.g[A2][0];
    let mut x = FpuF32(c.f[12]);
    c.g[V0][0] = flags & 1;
    c.g[V1][0] = align;
    if flags & 1 != 0 {
        c.cycles += 6;
        c.g[AT][0] = HUD_SCREEN_WIDTH as u64;
        c.f[0] = HUD_SCREEN_WIDTH;
        c.g[V0][0] = flags & 2;
        c.g[V1][0] = sx(0u32.wrapping_sub(align as u32));
        x = FpuF32(HUD_SCREEN_WIDTH) - x;
        if flags & 2 != 0 {
            c.g[V1][0] = align;
        }
    }
    c.cycles += 2;
    let v = c.g[V1][0];
    let size = FpuF32(c.f[14]) * FpuF32(c.f[13]);
    let shift = if v == 0 {
        // Centred: half the size.
        c.cycles += 5;
        c.g[AT][0] = HALF as u64;
        c.f[1] = HALF;
        Some(size * FpuF32(HALF))
    } else {
        c.cycles += 2;
        if (v as i64) > 0 {
            c.cycles += 1;
            Some(size)
        } else {
            None
        }
    };
    if let Some(shift) = shift {
        c.cycles += 1;
        c.f[0] = shift.0;
        x = x - shift;
    }
    c.cycles += 2;
    c.f[12] = x.0;
    c.f[0] = x.0;
    ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::difftest::check;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_COUNT;
    use crate::difftest::fill;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::main::ssxapp::tests::f;
    pub(crate) use crate::game::visualfx::lensfx::tests::painter_sun;
    use ssx3_ee_rt::cpu::reg::*;
    use ssx3_ee_rt::cpu::sx;
    use ssx3_ee_rt::Cpu;

    // ----------------------------------------------------------------- sun

    /// Bit n set: fake call n returns 0 (for the off-screen test).
    pub(crate) const ZERO_RESULTS: u32 = fakes::LOG + 0x34;

    pub(crate) fn fake_maybe_zero(c: &mut Cpu, target: u32) {
        fake_call(c, target);
        let n = c.bus.read32(LOG_COUNT).wrapping_sub(1);
        if n < 32 && c.bus.read32(ZERO_RESULTS) >> n & 1 != 0 {
            c.g[V0][0] = 0;
        }
    }

    /// `LensFX_UpdateSunPosition` input. Fake calls: 0 the sun texture
    /// (-1 for a quarter of the seeds), 1 the azimuth, 2 the elevation
    /// (each 0.0 for some seeds: the `.lit4` default angles), 3 the
    /// off-screen test (0 = on screen for half the seeds), 4 the
    /// projection. `Math_SinCos` runs for real.
    pub(crate) fn sun_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x4c46_5853, 5);
        c.hooks.miss = fake_maybe_zero;
        let fx = DATA + 0x1000;
        let cam = DATA + 0x2000;
        let gfx = DATA + 0x3000;
        let results = DATA + 0x100;
        // Painter views 6 and 7 share one sun object.
        for view in [6, 7] {
            painter_sun(
                c,
                &mut r,
                view,
                DATA + 0x200,
                DATA + 0x6000,
                &[0xd8, 0xe0, 0x108],
            );
        }
        // The projection interface.
        c.bus.write32(GAME_GP + 0x2a90, gfx);
        c.bus.write32(gfx + 0x10d8, DATA + 0x7000);
        for (i, off) in [0x2d8u32, 0x2e0].into_iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 % 64) & !3;
            c.bus.write16(DATA + 0x7000 + off, delta as u16);
            c.bus
                .write32(DATA + 0x7000 + off + 4, FAKE + 0x40 + 0x10 * i as u32);
        }
        // Forced fake results: pointers to the texture / azimuth /
        // elevation words.
        let texture = if seed % 4 == 0 {
            u32::MAX
        } else {
            r.next_u32() % 8
        };
        c.bus.write32(results, texture);
        let az = if seed % 3 == 0 { 0 } else { f(&mut r, 360.0) };
        let el = if seed % 5 < 2 { 0 } else { f(&mut r, 90.0) };
        c.bus.write32(results + 4, az);
        c.bus.write32(results + 8, el);
        for (n, p) in [results, results + 4, results + 8].into_iter().enumerate() {
            c.bus.write32(fakes::LOG_RESULTS + 4 * n as u32, p);
        }
        c.bus
            .write32(ZERO_RESULTS, if seed % 2 == 0 { 1 << 3 } else { 0 });
        // `.lit4`: degrees to radians, the default angles.
        c.bus
            .write32(GAME_GP - 0x38c0, (std::f32::consts::PI / 180.0).to_bits());
        c.bus.write32(GAME_GP - 0x38bc, f(&mut r, 3.0));
        c.bus.write32(GAME_GP - 0x38b8, f(&mut r, 1.5));
        // The camera: viewport, view rows, translation, distance.
        for i in 0..4 {
            c.bus.write32(cam + 4 * i, f(&mut r, 640.0));
        }
        for i in 4..0x16 {
            c.bus.write32(cam + 4 * i, f(&mut r, 2.0));
        }
        c.bus.write32(cam + 0x40, f(&mut r, 1000.0));
        c.bus.write32(cam + 0x44, f(&mut r, 1000.0));
        c.bus.write32(cam + 0x48, f(&mut r, 1000.0));
        c.bus.write32(cam + 0x58, f(&mut r, 3000.0));
        fill(c, &mut r, fx, 16);
        fill(c, &mut r, fx + 0x1f0, 8);
        c.g[A0][0] = sx(fx);
        c.g[A1][0] = sx(cam);
        c.g[A2][0] = if seed % 7 < 3 {
            0
        } else {
            r.next_u32() as u64 | 1
        };
    }

    #[test]
    fn lens_fx_update_sun_position_matches() {
        covered(
            ssx3_ee::s09::f_002f4db8__generated,
            super::lens_fx_update_sun_position,
            300,
            &|c, seed| sun_setup(c, seed),
        );
    }
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
        covered(
            ee::s05::f_0021d1a0__generated,
            hud_draw_meter_bar,
            300,
            &|c, seed| {
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
                        c.bus
                            .write32(HUD + meter_bar::SPRITES + 0xc * style + 4 * i, p);
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
                c.bus
                    .write32(SEG + 4, ((r.next_u32() % 7) as i32 - 1) as u32);
                for i in 0..4 {
                    c.bus.write32(COLOUR + 4 * i, r.f32(1.0));
                }
                // The renderer: render state and DrawPrims.
                c.bus.write32(GAME_GP - meter_bar::RENDERER, RENDERER);
                c.bus.write32(RENDERER + meter_bar::STATE_TOP, STATE);
                c.bus.write32(STATE + meter_bar::STATE_FLAGS, r.next_u32());
                c.bus.write32(RENDERER + meter_bar::VPTR, VT);
                c.bus
                    .write16(VT + meter_bar::DRAW_PRIMS, ((r.next_u32() % 16) * 4) as u16);
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
            },
        );
    }
    use super::*;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::{elf_image, put_vec};

    // ------------------------------------------------------------------
    // ChaseCameraAlgorithm_ApplyAirSwing (0x164878).

    /// The chase-camera state, its swing info, the camera target ("rig",
    /// first word its vtable), the vtable and the rider the target points at.
    /// All above the fake results (`DATA..DATA+0x40000`), the heap arena
    /// (`DATA+0x40000..+0x80000`) and the fake log / world (`DATA+0x10_0000`).
    pub(crate) const AS_ME: u32 = DATA + 0x20_0000;

    pub(crate) const AS_INFO: u32 = DATA + 0x20_1000;

    pub(crate) const AS_RIG: u32 = DATA + 0x20_2000;

    pub(crate) const AS_VT: u32 = DATA + 0x20_2800;

    pub(crate) const AS_RIDER: u32 = DATA + 0x20_3000;

    /// Fields of the state (`ChaseCameraAlgorithm`).
    pub(crate) const AS_EYE_TARGET: u32 = 0x20;

    pub(crate) const AS_EYE: u32 = 0x40;

    pub(crate) const AS_TARGET: u32 = 0x30;

    pub(crate) const AS_SWING_CLOCK: u32 = 0x22c;

    pub(crate) const AS_DURATION: u32 = 0x230;

    pub(crate) const AS_TIME: u32 = 0x234;

    pub(crate) const AS_BLEND_OUT: u32 = 0x23c;

    pub(crate) const AS_BLEND_A: u32 = 0x240;

    pub(crate) const AS_BLEND_B: u32 = 0x244;

    pub(crate) const AS_AIRBORNE: u32 = 0x2d0;

    pub(crate) const AS_SWINGING: u32 = 0x2d8;

    pub(crate) const AS_SPLINE: u32 = 0x334;

    /// A real leaf that returns 0 (`v0 = 0; jr ra`), for the "no swing" answer
    /// of vtable slot 0x98 (the fakes never return 0).
    pub(crate) const RETURN_ZERO: u32 = 0x0014_06c0;

    /// `cCSICubicSpline_InitWithStartSlope` / `cCSICubicSpline_AddPoint`.
    pub(crate) const SPLINE_INIT: u32 = 0x0031_d738;

    pub(crate) const SPLINE_ADD: u32 = 0x0031_d7e0;

    /// Call a real guest function from the setup with `f12`/`f13` set.
    pub(crate) fn setup_call(
        c: &mut Cpu,
        addr: u32,
        f: GuestFn,
        a0: u32,
        a1: u64,
        f12: f32,
        f13: f32,
    ) {
        c.f[12] = f12.to_bits();
        c.f[13] = f13.to_bits();
        crate::game::camera::camera::tests::run(c, addr, f, &[sx(a0), a1]);
    }

    /// One air-swing frame: airborne or not, swing running or about to start
    /// (rig says yes / no), the spline initialised in the call itself or
    /// built beforehand (a live 9-point spline on the test heap), blend-out
    /// and the final minimum-distance clamp; every rig query is a logging fake
    /// (some unwind), `CheckDistanceReset` and the spline/maths are real.
    #[test]
    fn chase_camera_algorithm_apply_air_swing_matches() {
        covered(
            ee::s01::f_00164878__generated,
            crate::game::camera::camera::chase_camera_apply_air_swing,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = crate::game::camera::camera::tests::alloc_base(c, seed, 0x0016_4878);
                // Fake calls: 0x98 (n=0), 0x60 (1, 2), 0x80 (3), reset slot 8.
                let unwind = if seed % 3 == 0 {
                    r.next_u32() % 5
                } else {
                    u32::MAX
                };
                c.bus.write32(LOG_UNWIND_AT, unwind);
                for i in 0..0x360 / 4 {
                    c.bus.write32(AS_ME + 4 * i, 0);
                }
                // The rig: vtable, rider pointer (CheckDistanceReset's reset).
                c.bus.write32(AS_RIG, AS_VT);
                c.bus.write32(AS_RIG + 4, AS_RIDER);
                for (i, slot) in [0x08u32, 0x60, 0x80, 0x98].into_iter().enumerate() {
                    let delta = (r.next_u32() % 0x40) & !3;
                    c.bus.write16(AS_VT + slot, delta as u16);
                    c.bus.write32(AS_VT + slot + 4, FAKE + 0x10 * i as u32);
                }
                if seed % 5 == 0 {
                    c.bus.write32(AS_VT + 0x98 + 4, RETURN_ZERO);
                }
                put_vec(c, AS_RIDER + 0x1b0, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
                c.bus.write32(AS_ME + AS_TARGET, AS_RIG);
                // World up (D_004FF160) and random vectors where the fake
                // results point (below the pool allocator's tables at
                // DATA+0x20000; above them the fakes read allocator words).
                put_vec(c, 0x004f_f160, [0, 0, 1.0f32.to_bits(), 0]);
                for i in 0..0x2_0000 / 4 {
                    let v = if i % 4 == 3 { 0 } else { r.f32(1.0) };
                    c.bus.write32(DATA + 4 * i, v);
                }
                let big = if seed % 7 == 0 { 30.0 } else { 600.0 };
                put_vec(
                    c,
                    AS_ME + AS_EYE_TARGET,
                    [r.f32(big), r.f32(big), r.f32(big), 0],
                );
                put_vec(c, AS_ME + AS_EYE, [r.f32(big), r.f32(big), r.f32(big), 0]);
                for off in [0x110u32, 0x120, 0x130, 0x140, 0x150, 0x160, 0x170, 0x180] {
                    put_vec(
                        c,
                        AS_ME + off,
                        [r.f32(300.0), r.f32(300.0), r.f32(300.0), 0],
                    );
                }
                put_vec(
                    c,
                    AS_INFO + 0x10,
                    [r.f32(600.0), r.f32(600.0), r.f32(600.0), 0],
                );
                put_vec(c, AS_INFO + 0x20, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
                let airborne = seed % 4 != 0;
                let swinging = seed % 3 == 1;
                c.bus.write32(AS_ME + AS_AIRBORNE, airborne as u32);
                c.bus.write32(AS_ME + AS_SWINGING, swinging as u32);
                c.bus.write32(AS_ME + AS_SWING_CLOCK, r.f32(1.0));
                c.bus.write32(
                    AS_ME + AS_DURATION,
                    (2.3 + (r.next_u32() % 100) as f32 / 50.0).to_bits(),
                );
                let time = (r.next_u32() % 400) as f32 / 100.0;
                c.bus.write32(AS_ME + AS_TIME, time.to_bits());
                c.bus.write32(AS_ME + 0x238, r.f32(1.5));
                let blend = |r: &mut Rng, zero: bool| {
                    if zero {
                        0
                    } else {
                        ((r.next_u32() % 100) as f32 / 100.0).to_bits()
                    }
                };
                let z = seed % 6 == 2;
                let v = blend(&mut r, z);
                c.bus.write32(AS_ME + AS_BLEND_OUT, v);
                let v = blend(&mut r, seed % 5 == 3);
                c.bus.write32(AS_ME + AS_BLEND_A, v);
                let v = blend(&mut r, seed % 7 == 4);
                c.bus.write32(AS_ME + AS_BLEND_B, v);
                if swinging {
                    // A live spline, as the start of the swing leaves it.
                    let spl = AS_ME + AS_SPLINE;
                    setup_call(c, SPLINE_INIT, ee::s10::f_0031d738, spl, 9, 0.0, 0.0);
                    let mut y = 0.0f32;
                    for i in 0..9 {
                        y += (r.next_u32() % 100) as f32 / 800.0;
                        setup_call(
                            c,
                            SPLINE_ADD,
                            ee::s10::f_0031d7e0,
                            spl,
                            0,
                            i as f32 / 8.0,
                            y,
                        );
                    }
                }
                // Arguments: (this, info, blend-out time).
                c.g[A0][0] = sx(AS_ME);
                c.g[A1][0] = sx(AS_INFO);
                c.f[12] = ((r.next_u32() % 100) as f32 / 100.0 + 0.1).to_bits();
                c.g[RA][0] = crate::difftest::RA as u64;
                c.pc = 0x0016_4878;
            },
        );
    }

    // ------------------------------------------------------------------
    // Rider_FindRail (0x108a48).

    /// The rider, its state object, path, nodes, overlap list with one plain
    /// rail object (bounds, matrix, flags), animation controller and
    /// sequence list; the output pair of vectors.
    pub(crate) const FR_ME: u32 = DATA + 0x1000;

    pub(crate) const FR_STATE: u32 = DATA + 0x2000;

    pub(crate) const FR_PATH: u32 = DATA + 0x3000;

    pub(crate) const FR_NODES: u32 = DATA + 0x3100;

    pub(crate) const FR_LIST: u32 = DATA + 0x4000;

    pub(crate) const FR_RAIL: u32 = DATA + 0x5000;

    pub(crate) const FR_RAIL_FLAGS: u32 = DATA + 0x5200;

    pub(crate) const FR_ANIM: u32 = DATA + 0x6000;

    pub(crate) const FR_SEQS: u32 = DATA + 0x6100;

    pub(crate) const FR_SEQ: u32 = DATA + 0x6200;

    pub(crate) const FR_OUT: u32 = DATA + 0x7000;

    /// `AnimTable` (0x446990, 0x1c bytes per animation): the test writes the
    /// kind of animation 7.
    pub(crate) const ANIM_TABLE: u32 = 0x0044_6990;

    /// Main state 0/1/other; one rail in reach or none (or masked out); the
    /// rider still or moving (rail ahead / behind); layer-2 animation none,
    /// grab (0x12), trick (0x13/0x14) or other, with every combination of
    /// sequence flags 0 and 2. All callees are real (bounded on this data).
    #[test]
    fn rider_find_rail_matches() {
        use crate::difftest::fakes::machine;
        crate::difftest::check(
            ee::s00::f_00108a48__generated,
            rider_find_rail,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x0010_8a48, 0);
                let me = FR_ME;
                c.bus.write32(me + 0x77c, FR_STATE);
                c.bus
                    .write32(FR_STATE + 0xde0, [0, 1, 0, 1, 2][(seed % 5) as usize]);
                // Path node + probe offset.
                c.bus.write32(me + rail_rider::PATH, FR_PATH);
                c.bus.write32(FR_PATH + 0x2c, FR_NODES);
                let node = r.next_u32() % 4;
                c.bus.write32(me + rail_rider::NODE, node);
                for i in 0..4 {
                    put_vec(
                        c,
                        FR_NODES + 0x20 * i,
                        [r.f32(100.0), r.f32(100.0), r.f32(100.0), 0],
                    );
                }
                put_vec(
                    c,
                    me + rail_rider::PROBE_OFFSET,
                    [r.f32(5.0), r.f32(5.0), r.f32(5.0), 0],
                );
                let vel = if seed % 5 == 3 {
                    [0, 0, 0, 0]
                } else {
                    [r.f32(50.0), r.f32(50.0), r.f32(50.0), 0]
                };
                put_vec(c, me + rail_rider::VELOCITY, vel);
                c.bus.write32(me + 0x25c, r.f32(10.0));
                c.bus.write32(me + 0x330, r.next_u32() & 1);
                // The overlap list: none, or one plain rail (type 1) whose flags
                // pass the mask (1) or not.
                c.bus.write32(me + rail_rider::OVERLAP, FR_LIST);
                c.bus.write32(FR_LIST + 0x210, (seed % 7 != 0) as u32);
                c.bus.write32(FR_LIST + 0x214, FR_RAIL);
                c.bus.write32(FR_RAIL + 8, 1);
                c.bus.write32(FR_RAIL + 0x68, FR_RAIL_FLAGS);
                c.bus.write32(FR_RAIL_FLAGS + 0x1c, (seed % 11 != 5) as u32);
                c.bus.write32(FR_RAIL_FLAGS + 0x28, r.next_u32());
                for off in [0x6c, 0x70, 0x74] {
                    c.bus.write32(FR_RAIL + off, (-1.0e4f32).to_bits());
                }
                for off in [0x78, 0x7c, 0x80] {
                    c.bus.write32(FR_RAIL + off, 1.0e4f32.to_bits());
                }
                for i in 0..4 {
                    let w = if i == 3 { 1.0f32.to_bits() } else { 0 };
                    put_vec(
                        c,
                        FR_RAIL + 0x10 + 0x10 * i,
                        [r.f32(100.0), r.f32(100.0), r.f32(100.0), w],
                    );
                }
                // Layer-2 animation (0x1b6 = none) and its sequence flags.
                c.bus.write32(me + rail_rider::ANIM, FR_ANIM);
                c.bus
                    .write32(FR_ANIM + 8, if seed % 6 == 0 { 0x1b6 } else { 7 });
                let kind = [0x12u32, 0x13, 0x14, 0x12, 5][(r.next_u32() % 5) as usize];
                c.bus.write32(ANIM_TABLE + 7 * 0x1c, kind);
                c.bus.write32(FR_ANIM + 0x50, FR_SEQS);
                c.bus.write32(FR_SEQS + 0x10 + 4, FR_SEQ);
                c.bus.write64(FR_SEQ + 0xb0, (r.next_u32() & 0b101) as u64);
                put_vec(c, FR_OUT, [r.f32(100.0), r.f32(100.0), r.f32(100.0), 0]);
                c.g[A0][0] = sx(me);
                c.g[A1][0] = sx(FR_OUT);
                c.pc = 0x0010_8a48;
            },
        );
    }

    // ------------------------------------------------------------------
    // HUD_AlignX (0x21e750).

    /// Alignments -2..2, all four flag combinations, random and edge floats.
    #[test]
    fn hud_align_x_matches() {
        use crate::difftest::{scramble, Rng};
        crate::difftest::check(
            ee::s05::f_0021e750__generated,
            hud_align_x,
            300,
            &|c, seed| {
                let mut r = Rng(0x21e7_50 ^ (seed as u64) << 20);
                scramble(c, &mut r);
                c.g[A1][0] = sx((seed % 5) as i32 as u32).wrapping_sub(2);
                c.g[A2][0] = ((seed / 5) % 4) as u64 | (r.next_u32() as u64 & !3) << 8;
                for f in [12, 13, 14] {
                    c.f[f] = if seed % 6 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(400.0)
                    };
                }
                c.pc = 0x0021_e750;
            },
        );
    }
}

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::main::ssxapp::callee::NASH_CLASSIFY_EFFECTS;
    pub(crate) use crate::game::main::ssxapp::callee::NASH_CLASSIFY_INSTANCES;
    pub(crate) use crate::game::main::ssxapp::callee::NASH_CLASSIFY_PATCHES;
    pub(crate) use crate::game::main::ssxapp::callee::NASH_COLLECT_EFFECTS;
    pub(crate) use crate::game::main::ssxapp::callee::NASH_COLLECT_INSTANCES;
    pub(crate) use crate::game::main::ssxapp::callee::NASH_COLLECT_PATCHES;
    pub(crate) use crate::game::main::ssxapp::callee::NASH_CULL_NODE;
    pub(crate) use crate::game::main::ssxapp::callee::NASH_EMPTY_HOOK;
    pub(crate) use crate::game::main::ssxapp::callee::VU0_UPLOAD_MATRIX_PAIR;
    pub(crate) use crate::game::main::ssxapp::callee::WORLD_PAINTER_GET_SUN_AZIMUTH;
    pub(crate) use crate::game::main::ssxapp::callee::WORLD_PAINTER_GET_SUN_ELEVATION;
    pub(crate) use crate::game::main::ssxapp::callee::WORLD_PAINTER_GET_SUN_TEXTURE;
}

pub(crate) mod table_callee {
    pub(crate) use crate::game::main::ssxapp::table_callee::MATH_SIN_COS;
    pub(crate) use crate::game::main::ssxapp::table_callee::OCCLUSION_MAN_BUILD_VISIBLE;
}

pub(crate) mod lens {

    /// Screen rectangle of the sun sprite: x, y, w, h (floats).
    pub const RECT: u32 = 0x00;

    /// The clamped copy of the rectangle drawn (x, y, w, h).
    pub const CLAMPED: u32 = 0x10;

    /// Sprite depth / size copied from +0x1f8.
    pub const DEPTH: u32 = 0x24;

    /// Visible flags (two halfwords).
    pub const VISIBLE_A: u32 = 0x2c;

    pub const VISIBLE_B: u32 = 0x2e;

    /// The projected sun position (quad).
    pub const SCREEN: u32 = 0x1f0;

    /// Occlusion state cleared when the sun is off screen.
    pub const OCCLUSION: u32 = 0x204;
}

/// gp offset of the graphics object used for projection (`+0x10d8` its
/// camera interface vtable).
pub(crate) const LENS_GRAPHICS_GP: u32 = 0x2a90;

/// `.lit4` entries (gp-relative, negative): the painter-angle scale
/// (degrees to radians) and the default azimuth / elevation (radians).
pub(crate) const LIT_ANGLE_SCALE: u32 = 0x38c0;

pub(crate) const LIT_DEFAULT_AZIMUTH: u32 = 0x38bc;

pub(crate) const LIT_DEFAULT_ELEVATION: u32 = 0x38b8;

/// `void LensFX_UpdateSunPosition(LensFX* fx, Camera* cam, bool alt)`
/// (0x2f4db8): position the sun sprite. Painter view `alt ? 7 : 6`; a sun
/// texture of -1 hides the sun. Otherwise the sun direction comes from the
/// painter's azimuth / elevation (scaled to radians; the `.lit4` defaults
/// when both are 0) through `Math_SinCos`, is put `cam+0x58 - 500` units
/// from the camera position (camera row 3 transformed by the view rows),
/// and projected (camera interface slot 91 = off screen test; slot 92 =
/// project). On screen: both visible flags 1, the projected point at
/// +0x1f0, the 16x16 rect centred on it at +0, and its copy at +0x10 with
/// x / y clamped to `[0, viewport edge - 16]`; off screen: flags and the
/// occlusion state cleared.
pub fn lens_fx_update_sun_position(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S2, 0x80);
        frame.save(c, S1, 0x90);
        c.g[S2][0] = c.g[A0][0]; // fx
        frame.save(c, S3, 0x70);
        c.g[S1][0] = c.g[A1][0]; // camera
        frame.save(c, S4, 0x60);
        c.g[S3][0] = c.g[A2][0]; // alt view
        frame.save(c, S0, 0xa0);
        c.g[A0][0] = 6;
        frame.save_ra(c, 0x50);
        frame.save_fpr(c, 22, 0xc0);
        frame.save_fpr(c, 21, 0xb8);
        frame.save_fpr(c, 20, 0xb0);
        c.g[S4][0] = c.lw(r32(c, GP).wrapping_add(LENS_GRAPHICS_GP));
        callee::WORLD_PAINTER_GET_SUN_TEXTURE.call(c, 0x002f_4df8)?;
        c.g[V1][0] = u64::MAX;
        if c.g[V0][0] == c.g[V1][0] {
            // No sun.
            c.cycles += 3;
            c.sh(at(c, S2, lens::VISIBLE_B), 0);
            c.sh(at(c, S2, lens::VISIBLE_A), 0);
            sun_epilogue(c, frame);
            return Ok(());
        }
        c.lwc1(1, at(c, S1, 0x58));
        // The painter angles.
        c.cycles += 18;
        c.g[V0][0] = 6;
        c.g[AT][0] = 0x43fa_0000; // 500.0
        c.mtc1(AT, 0);
        c.g[S0][0] = if c.g[S3][0] == 0 { c.g[V0][0] } else { 7 };
        c.lwc1(20, r32(c, GP).wrapping_sub(LIT_ANGLE_SCALE));
        c.sub_s(22, 1, 0); // distance = cam+0x58 - 500
        c.g[A0][0] = c.g[S0][0];
        callee::WORLD_PAINTER_GET_SUN_AZIMUTH.call(c, 0x002f_4e34)?;
        c.mul_s(21, 0, 20); // azimuth
        c.g[A0][0] = c.g[S0][0];
        callee::WORLD_PAINTER_GET_SUN_ELEVATION.call(c, 0x002f_4e40)?;
        c.mul_s(20, 0, 20); // elevation
        c.mtc1(0, 0);
        c.c_eq_s(21, 0);
        let azimuth_set = !c.fcc();
        c.mov_s(12, 21);
        let painter_angles = azimuth_set || {
            c.cycles += 4;
            c.c_eq_s(20, 0);
            !c.fcc()
        };
        if azimuth_set {
            c.cycles += 1;
        }
        c.g[A0][0] = sx(sp(c).wrapping_add(0x40)); // &sincos(azimuth)
        if painter_angles {
            c.cycles += 7;
            c.g[A1][0] = sx(sp(c).wrapping_add(0x44));
            call_ptr(c, table_callee::MATH_SIN_COS, 0x002f_4e98)?;
            c.mov_s(12, 20);
            c.g[A0][0] = sx(sp(c).wrapping_add(0x48));
            c.g[A1][0] = sx(sp(c).wrapping_add(0x4c));
            call_ptr(c, table_callee::MATH_SIN_COS, 0x002f_4ea8)?;
        } else {
            c.cycles += 9;
            c.lwc1(12, r32(c, GP).wrapping_sub(LIT_DEFAULT_AZIMUTH));
            c.g[A1][0] = sx(sp(c).wrapping_add(0x44));
            call_ptr(c, table_callee::MATH_SIN_COS, 0x002f_4e74)?;
            c.lwc1(12, r32(c, GP).wrapping_sub(LIT_DEFAULT_ELEVATION));
            c.g[A0][0] = sx(sp(c).wrapping_add(0x48));
            c.g[A1][0] = sx(sp(c).wrapping_add(0x4c));
            call_ptr(c, table_callee::MATH_SIN_COS, 0x002f_4e84)?;
        }
        c.lq(V0, at(c, S1, 0x40));
        sun_world_position(c);
        // Off screen?
        let vt = c.lw(at(c, S4, 0x10d8));
        c.g[V1][0] = vt;
        c.g[A0][0] = c.lh((vt as u32).wrapping_add(0x2d8));
        c.g[V0][0] = c.lw((vt as u32).wrapping_add(0x2dc));
        let method = r32(c, V0);
        c.g[A0][0] = sx(r32(c, S4).wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x002f_5024)?;
        let off_screen = c.g[V0][0] != 0;
        c.g[V1][0] = 1;
        if off_screen {
            c.cycles += 4;
            c.sw(at(c, S2, lens::OCCLUSION), 0);
            c.sh(at(c, S2, lens::VISIBLE_A), 0);
            c.sh(at(c, S2, lens::VISIBLE_B), 0);
            sun_epilogue(c, frame);
            return Ok(());
        }
        // Project the sun to the screen.
        c.cycles += 41;
        c.g[A0][0] = sx(sp(c).wrapping_add(0x20));
        c.sh(at(c, S2, lens::VISIBLE_B), c.g[V1][0]);
        c.g[A2][0] = c.g[S0][0];
        c.sh(at(c, S2, lens::VISIBLE_A), c.g[V1][0]);
        let vt = c.lw(at(c, S4, 0x10d8));
        c.g[V0][0] = vt;
        c.g[A1][0] = c.lh((vt as u32).wrapping_add(0x2e0));
        c.g[V1][0] = c.lw((vt as u32).wrapping_add(0x2e4));
        let method = r32(c, V1);
        c.g[A1][0] = sx(r32(c, S4).wrapping_add(r32(c, A1)));
        call_ptr(c, method, 0x002f_5060)?;
        sun_screen_rect(c);
        sun_epilogue(c, frame);
        Ok(())
    })
}

/// The sun's world position (v0 = camera row 3 at entry): the camera
/// position `-(row 3)` through the camera rows (FPU) at sp+0, the
/// direction `(r44 r4c, r40 r4c, r48, 0)` from the two `Math_SinCos`
/// result pairs at sp+0x40..0x4c, and `sp+0x10 = position + direction *
/// distance (f22)` (VU0 macro ops). Leaves s0 = a1 = sp+0x10.
pub(crate) fn sun_world_position(c: &mut Cpu) {
    c.cycles += 96;
    let sp = sp(c);
    let cam = r32(c, S1);
    let at_sp = |o: u32| sp.wrapping_add(o);
    let at_cam = |o: u32| cam.wrapping_add(o);
    c.g[S0][0] = sx(at_sp(0x10));
    c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0
    c.mtc1(AT, 0);
    c.g[A1][0] = c.g[S0][0];
    // Negate the camera translation row: (-1) * row 3.
    c.sq(V0, at_sp(0x20));
    c.mfc1(V1, 0);
    c.lqc2(4, at_sp(0x20));
    c.qmtc2(V1, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, at_sp(0x30));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 14); // f14 = 1.0
    c.lq(V0, at_sp(0x30));
    // Transform it by the camera rotation rows (FPU).
    c.lwc1(6, at_cam(0x10));
    c.sq(V0, sp);
    c.lwc1(12, at_cam(0x14));
    c.lwc1(3, at_cam(0x20));
    c.lwc1(9, at_cam(0x24));
    c.lwc1(2, at_cam(0x30));
    c.lwc1(1, at_cam(0x34));
    c.lwc1(0, sp);
    c.lwc1(5, at_sp(0x4));
    c.lwc1(7, at_cam(0x40));
    c.mul_s(6, 0, 6);
    c.lwc1(8, at_cam(0x44));
    c.mul_s(12, 5, 12);
    c.mul_s(9, 5, 9);
    c.lwc1(10, at_cam(0x18));
    c.mul_s(1, 5, 1);
    c.lwc1(11, at_cam(0x28));
    c.mul_s(3, 0, 3);
    c.lwc1(4, at_sp(0x8));
    c.mul_s(2, 0, 2);
    c.lwc1(13, at_cam(0x38));
    c.mul_s(5, 5, 8);
    c.mul_s(0, 0, 7);
    c.lwc1(8, at_cam(0x48));
    c.add_s(3, 3, 9);
    c.lwc1(7, at_cam(0x2c));
    c.add_s(2, 2, 1);
    c.lwc1(9, at_cam(0x1c));
    c.add_s(0, 0, 5);
    c.lwc1(1, at_sp(0xc));
    c.mul_s(8, 4, 8);
    c.lwc1(5, at_cam(0x4c));
    c.mul_s(10, 4, 10);
    c.mul_s(11, 4, 11);
    c.add_s(6, 6, 12);
    c.mul_s(4, 4, 13);
    c.lwc1(12, at_cam(0x3c));
    c.mul_s(5, 1, 5);
    c.sq(V0, at_sp(0x10));
    c.mul_s(9, 1, 9);
    c.mul_s(7, 1, 7);
    c.mul_s(1, 1, 12);
    c.add_s(6, 6, 10);
    c.add_s(3, 3, 11);
    c.add_s(2, 2, 4);
    c.add_s(0, 0, 8);
    c.add_s(3, 3, 7);
    c.add_s(2, 2, 1);
    c.add_s(0, 0, 5);
    c.add_s(6, 6, 9);
    c.swc1(3, at_sp(0x14));
    c.swc1(2, at_sp(0x18));
    c.swc1(0, at_sp(0x1c));
    c.swc1(6, at_sp(0x10));
    c.lq(V0, at_sp(0x10));
    c.sq(V0, sp); // the camera position
                  // The direction from the Math_SinCos results.
    c.lwc1(3, at_sp(0x4c));
    c.lwc1(0, at_sp(0x44));
    c.lwc1(1, at_sp(0x40));
    c.mul_s(0, 0, 3);
    c.lwc1(2, at_sp(0x48));
    c.mul_s(1, 1, 3);
    c.swc1(14, at_sp(0xc));
    c.swc1(2, at_sp(0x18));
    c.swc1(0, at_sp(0x10));
    c.swc1(1, at_sp(0x14));
    c.sw(at_sp(0x1c), 0);
    // position = camera + direction * distance (VU0).
    c.mfc1(V0, 22);
    c.lqc2(4, at_sp(0x10));
    c.qmtc2(V0, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, at_sp(0x10));
    c.lqc2(3, at_sp(0x10));
    c.lqc2(4, sp);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at_sp(0x10));
}

/// After the projection (the screen point returned at sp+0x20): store it at
/// +0x1f0, centre a 16x16 rect on it at +0, copy the rect to +0x10 and
/// clamp its x to `[0, cam[0] + cam[2] - 16]` and its y to `[0, cam[0] +
/// cam[3] - 16]` (negative becomes 0; the viewport words at cam+0..0xc),
/// copy the depth from +0x1f8 to +0x24.
pub(crate) fn sun_screen_rect(c: &mut Cpu) {
    let fx = r32(c, S2);
    let cam = r32(c, S1);
    c.lq(V0, sp(c).wrapping_add(0x20));
    c.g[AT][0] = 0x4100_0000; // 8.0
    c.mtc1(AT, 2);
    c.sq(V0, fx.wrapping_add(lens::SCREEN));
    c.g[AT][0] = 0x4180_0000; // 16.0
    c.mtc1(AT, 4);
    c.lwc1(0, fx.wrapping_add(lens::SCREEN));
    c.lwc1(1, fx.wrapping_add(lens::SCREEN + 4));
    c.sub_s(0, 0, 2);
    c.swc1(4, fx.wrapping_add(lens::RECT + 8));
    c.sub_s(1, 1, 2);
    c.swc1(4, fx.wrapping_add(lens::RECT + 0xc));
    c.mtc1(0, 3); // f3 = 0
    c.swc1(0, fx.wrapping_add(lens::RECT));
    c.swc1(1, fx.wrapping_add(lens::RECT + 4));
    c.mov_s(5, 3);
    // Copy the rect (unaligned doubleword pairs).
    c.ldl(V0, fx.wrapping_add(lens::RECT + 7));
    c.ldr(V0, fx.wrapping_add(lens::RECT));
    c.ldl(V1, fx.wrapping_add(lens::RECT + 0xf));
    c.ldr(V1, fx.wrapping_add(lens::RECT + 8));
    c.sdl(V0, fx.wrapping_add(lens::CLAMPED + 7));
    c.sdr(V0, fx.wrapping_add(lens::CLAMPED));
    c.sdl(V1, fx.wrapping_add(lens::CLAMPED + 0xf));
    c.sdr(V1, fx.wrapping_add(lens::CLAMPED + 8));
    // Clamp x.
    c.lwc1(0, cam);
    c.lwc1(1, cam.wrapping_add(8));
    c.lwc1(2, fx.wrapping_add(lens::CLAMPED));
    c.add_s(0, 0, 1);
    c.c_le_s(3, 2);
    let inside = c.fcc();
    c.sub_s(0, 0, 4);
    if inside {
        c.cycles += 1;
        c.min_s(5, 2, 0);
    }
    // Clamp y.
    c.cycles += 9;
    c.swc1(5, fx.wrapping_add(lens::CLAMPED));
    c.lwc1(2, fx.wrapping_add(lens::CLAMPED + 4));
    c.mov_s(5, 3);
    c.lwc1(1, cam.wrapping_add(0xc));
    c.lwc1(0, cam);
    c.c_le_s(3, 2);
    c.add_s(0, 0, 1);
    let inside = c.fcc();
    c.sub_s(0, 0, 4);
    if inside {
        c.cycles += 1;
        c.min_s(5, 2, 0);
    }
    c.cycles += 3;
    c.lwc1(0, fx.wrapping_add(0x1f8));
    c.swc1(5, fx.wrapping_add(lens::CLAMPED + 4));
    c.swc1(0, fx.wrapping_add(lens::DEPTH));
}

/// `LensFX_UpdateSunPosition`'s shared epilogue.
pub(crate) fn sun_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 11;
    frame.restore(c, S0, 0xa0);
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore(c, S3, 0x70);
    frame.restore(c, S4, 0x60);
    frame.restore_ra(c, 0x50);
    frame.restore_fpr(c, 22, 0xc0);
    frame.restore_fpr(c, 21, 0xb8);
    frame.restore_fpr(c, 20, 0xb0);
    frame.close(c);
}
