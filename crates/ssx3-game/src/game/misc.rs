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
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::ai::gp;
pub(crate) use crate::game::fe::splashmanager::gpo;
pub(crate) use crate::game::fe::splashmanager::stk;
pub(crate) use crate::game::visualfx::lensfx::at;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
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
    (
        0x0010_bd10,
        computer_actor_rider_select_target_rider,
        "ComputerActorRider_SelectTargetRider",
    ),
    (
        0x0010_def0,
        computer_actor_rider_compute_rubber_band,
        "ComputerActorRider_ComputeRubberBand",
    ),
    (0x002b_af90, wp_data_find_cell_index, "WPData_FindCellIndex"),
    (0x002d_5048, c_anger_fx_render, "cAngerFX_Render"),
    (
        0x002f_1510,
        rider_base_render_attack_fx,
        "RiderBase_RenderAttackFX",
    ),
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

pub(crate) mod rubber_band {
    use super::*;

    /// The computer actor's rubber-band parameters (`+0x18`): the catch-up
    /// and hold-back distances, the mode (0 off, 1 hold back, 2 catch up,
    /// 3 both), its own distance to the finish.
    pub const PARAMS: u32 = 0x18;

    pub const CATCH_UP_DIST: u32 = 0xdc;

    pub const HOLD_BACK_DIST: u32 = 0xe0;

    pub const MODE: u32 = 0xe4;

    pub const DIST_TO_FINISH: u32 = 0x4d0;

    /// The game-mode byte (2: no rubber band) at 0x535c11.
    pub const GAME_MODE: u32 = 0x0053_0000 + 0x5c11;

    /// `.lit4`: the minimum distance (50000), the catch-up and hold-back
    /// gains (1.1, 1/1.1).
    pub const GP_MIN_DIST: i32 = -0x7cc4;

    pub const GP_CATCH_UP_GAIN: i32 = -0x7cc0;

    pub const GP_HOLD_BACK_GAIN: i32 = -0x7cbc;

    pub const BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;

    pub const BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;

    pub const COMPUTER_ACTOR_RIDER_GET_HUMAN_RIDER: Callee = Callee {
        addr: 0x0010_deb0,
        f: ee::s00::f_0010deb0,
    };

    pub const RIDER_GET_DIST_TO_FINISH: Callee = Callee {
        addr: 0x0011_3128,
        f: ee::s00::f_00113128,
    };
}

pub(crate) mod target {

    /// The chosen target rider (-1: none).
    pub const TARGET: u32 = 0xe70;

    /// The actor's AI view, starting with the table of the six riders
    /// (0x24 each: `{ Rider* rider; ?; float distance; float heading; ...
    /// }`).
    pub const TABLE: u32 = 0x18;

    pub const ENTRY_SIZE: u32 = 0x24;

    pub const ENTRY_DIST: u32 = 0x8;

    pub const ENTRY_HEADING: u32 = 0xc;

    /// Also in the AI view: the actor's velocity and its distance to the
    /// finish.
    pub const RIDER_VELOCITY: u32 = 0x1b0;

    pub const RIDER_FINISH_DIST: u32 = 0x4c8;
}

// ---------------------------------------------------------------------------
// WPData_FindCellIndex (0x2baf90)

pub(crate) const QUAD_TREE_FIND_LEAF: Callee = Callee {
    addr: 0x002c_1cd8,
    f: ee::s08::QuadTree_FindLeaf,
};

/// `int WPData_FindCellIndex(WPData* wp, float x, float z)` (0x2baf90): the
/// world-painter cell at (x, z): the leaf of the data's quadtree
/// (`QuadTree_FindLeaf(wp->tree, x, z)`) and its cell index (`leaf + 4`).
pub fn wp_data_find_cell_index(c: &mut Cpu) {
    use crate::guest::Addr;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(r32(c, A0)).lw(c);
        QUAD_TREE_FIND_LEAF.call(c, 0x002b_afa0)?;
        c.g[V0][0] = Addr(r32(c, V0)).off(4).lw(c);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float ComputerActorRider_ComputeRubberBand(ComputerActorRider* this)`
/// (0x10def0): the computer rider's speed factor (f0) against the human
/// rider: 1 without a human, with the mode off, in game mode 2, or within
/// 50000 of the finish. Otherwise with `d` = the human's distance to the
/// finish minus its own: catching up (`d < catch_up`, mode 2/3) gives
/// `1.1 * (1 + (d - k) / k)`; holding back (`hold_back < d`, mode 1/3)
/// `(1/1.1) / (1 + (d - k2) / k2)`; else 1.
pub fn computer_actor_rider_compute_rubber_band(c: &mut Cpu) {
    use rubber_band::*;
    guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        c.cycles += 13;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, BE_LIBRARY_GET_INSTANCE, 0x0010_df04)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, BE_LIBRARY_GET_INTERFACE, 0x0010_df10)?;
        c.g[A0][0] = c.g[S0][0];
        COMPUTER_ACTOR_RIDER_GET_HUMAN_RIDER.call(c, 0x0010_df18)?;
        c.g[A2][0] = c.g[V0][0];
        let this = Addr(r32(c, S0));

        // Whether the rubber band applies at all.
        let mut applies = false;
        if c.g[A2][0] != 0 {
            c.cycles += 4;
            let params = this.off(PARAMS).lw(c);
            c.g[A1][0] = params;
            let mode = Addr(params as u32).off(MODE).lw(c);
            c.g[V0][0] = 0x53_0000;
            if mode != 0 {
                c.cycles += 4;
                c.g[A0][0] = 0x2;
                c.g[V1][0] = Addr(GAME_MODE).lb(c);
                if c.g[V1][0] != 0x2 {
                    c.cycles += 5;
                    let own = Addr(params as u32).off(DIST_TO_FINISH).read_fpu(c);
                    let min = Addr(gp(c, GP_MIN_DIST)).read_fpu(c);
                    abi::set_fpr(c, 1, own);
                    abi::set_fpr(c, 0, min);
                    applies = !abi::c_lt_s(c, own, min);
                }
            }
        }
        if !applies {
            c.cycles += 4;
            c.g[AT][0] = 0x3f80_0000;
            abi::set_fpr(c, 0, FpuF32::ONE);
            frame.restore(c, S0, 0x10);
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        // d = the human's distance to the finish minus ours.
        c.cycles += 10;
        c.g[A0][0] = c.g[A2][0];
        RIDER_GET_DIST_TO_FINISH.call(c, 0x0010_df70)?;
        let params = Addr(this.off(PARAMS).lw(c) as u32);
        c.g[V0][0] = sx(params.0);
        let own = params.off(DIST_TO_FINISH).read_fpu(c);
        let k = params.off(CATCH_UP_DIST).read_fpu(c);
        abi::set_fpr(c, 1, own);
        abi::set_fpr(c, 3, k);
        let d = FpuF32(c.f[0]) - own;
        abi::set_fpr(c, 0, d);
        let mut factor = None;
        if abi::c_lt_s(c, d, k) {
            c.cycles += 5;
            let mode = params.off(MODE).lw(c);
            let catch_up = sx((mode as u32).wrapping_sub(2)) < 2;
            c.g[V0][0] = catch_up as u64;
            if catch_up {
                c.cycles += 8;
                c.g[AT][0] = 0x3f80_0000;
                abi::set_fpr(c, 1, FpuF32::ONE);
                let gain = Addr(gp(c, GP_CATCH_UP_GAIN)).read_fpu(c);
                abi::set_fpr(c, 2, gain);
                factor = Some(((d - k).div(k) + FpuF32::ONE) * gain);
            }
        }
        let factor = match factor {
            Some(f) => f,
            None => {
                c.g[A0][0] = sx(params.0);
                c.cycles += 5;
                let k2 = params.off(HOLD_BACK_DIST).read_fpu(c);
                abi::set_fpr(c, 3, k2);
                let beyond = abi::c_lt_s(c, k2, d);
                c.g[V0][0] = 0x1;
                let mut hold_back = false;
                if beyond {
                    c.cycles += 3;
                    let mode = params.off(MODE).lw(c);
                    c.g[A0][0] = mode;
                    c.g[V0][0] = 0x3;
                    if mode == 1 {
                        hold_back = true;
                    } else {
                        c.cycles += 2;
                        hold_back = mode == 3;
                    }
                }
                if hold_back {
                    c.cycles += 9;
                    c.g[AT][0] = 0x3f80_0000;
                    abi::set_fpr(c, 1, FpuF32::ONE);
                    let gain = Addr(gp(c, GP_HOLD_BACK_GAIN)).read_fpu(c);
                    abi::set_fpr(c, 2, gain);
                    FpuF32::ONE.div((d - k2).div(k2) + FpuF32::ONE) * gain
                } else {
                    c.cycles += 2;
                    c.g[AT][0] = 0x3f80_0000;
                    FpuF32::ONE
                }
            }
        };
        abi::set_fpr(c, 0, factor);
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cAngerFX::Render()` (0x2d5048, `src/visualfx/angerfx.cpp`, asm
/// only in the reference): the red "anger" glow drawn over a rider.
///
/// `self+0` is the rider, `self+4` the per-racer effect records (0x2c
/// bytes: +0 intensity, +0x4..+0x10 colour, +0x14/+0x18 size), indexed by
/// the race's current slot (world +0x84 -> +0x84 -> +0x14). Nothing is
/// drawn when the rider's vtable +0x40 query says so, when the intensity
/// is 0, or when the rider's bone (`rider+0x780` skeleton, bone
/// `rider+0x8a8`) lies behind the camera or 2000 or more units away in view
/// space (renderer vtable +0x118 gives the view matrix). Past 1500 units
/// the glow fades (`.lit4` 0.002 per unit). The quad is built from the
/// view matrix's right/up axes scaled by the size and distance (`.lit4`
/// 0.003), its colour converted to 0..255 (alpha 0..128), written as one
/// 4-vertex sprite with a GS state packet pushed on the renderer's packet
/// stack (`gp-0x854` +0xe84) and drawn through renderer vtable +0x238.
///
/// The body keeps the original's block structure: every cycle charge, the
/// VU0 macro ops (`vmula`/`vmadda`/`vmadd`, `vmul`/`vadd`/`vsub`) on the
/// stack temporaries, and the scratch registers each block leaves.
pub fn c_anger_fx_render(c: &mut Cpu) {
    guest(c, |c| {
        'b_2d5584: {
            'b_2d5580: {
                c.cycles += 16;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x1a0));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
                c.g[S0][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x160), c.g[RA][0]);
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x198));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x190));
                c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                call_ptr(c, t, 0x002d5080)?;
                if c.g[V0][0] != 0u64 {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
                    break 'b_2d5580;
                } else {
                    c.cycles += 16;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = 0x2c;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                    c.mtc1(0, 20);
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 21);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    c.mult(0, 2, 2, 4);
                    c.g[S1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.lwc1(0, (c.g[S1][0] as u32));
                    c.c_eq_s(0, 20);
                    if c.fcc() {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
                        break 'b_2d5580;
                    } else {
                        c.cycles += 32;
                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x118));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x11c));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x002d50e0)?;
                        c.g[T4][0] = c.g[V0][0];
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x780));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8a8));
                        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x30));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.lq(2, (c.g[A0][0] as u32).wrapping_add(0x30));
                        c.sq(2, (c.g[SP][0] as u32));
                        c.lqc2(8, (c.g[SP][0] as u32));
                        c.lqc2(4, (c.g[T4][0] as u32));
                        c.lqc2(5, (c.g[T4][0] as u32).wrapping_add(0x10));
                        c.lqc2(6, (c.g[T4][0] as u32).wrapping_add(0x20));
                        c.lqc2(7, (c.g[T4][0] as u32).wrapping_add(0x30));
                        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                        c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x18));
                        c.c_le_s(4, 20);
                        let t = c.fcc();
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
                        if t {
                            break 'b_2d5580;
                        } else {
                            c.cycles += 6;
                            c.g[AT][0] = 0x44fa0000;
                            c.mtc1(1, 5);
                            c.c_le_s(5, 4);
                            if c.fcc() {
                                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
                                break 'b_2d5584;
                            } else {
                                'b_2d5190: {
                                    c.cycles += 9;
                                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3d20));
                                    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x3d1c));
                                    c.mul_s(1, 4, 1);
                                    c.lwc1(3, (c.g[S1][0] as u32).wrapping_add(0x14));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x18));
                                    c.c_lt_s(2, 4);
                                    c.add_s(6, 0, 1);
                                    let t = !c.fcc();
                                    c.add_s(7, 3, 1);
                                    if t {
                                        break 'b_2d5190;
                                    } else {
                                        c.cycles += 3;
                                        c.sub_s(1, 5, 4);
                                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3d18));
                                        c.mul_s(21, 1, 0);
                                        break 'b_2d5190;
                                    }
                                }
                                c.cycles += 252;
                                c.lwc1(0, (c.g[T4][0] as u32));
                                c.g[T1][0] = 0x500000;
                                c.lwc1(1, (c.g[T4][0] as u32).wrapping_add(0x10));
                                c.g[A2][0] = 0xffffffffffbf0000;
                                c.lwc1(2, (c.g[T4][0] as u32).wrapping_add(0x20));
                                c.g[A2][0] = c.g[A2][0] | 0xffff;
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                                c.g[T0][0] = 0x400000;
                                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x28));
                                c.g[T2][0] = 0xfffffffffffffffc;
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x24));
                                c.g[A3][0] = 0xffffffffe0000000;
                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x2c));
                                c.g[A3][0] = c.g[A3][0] | 0x3ff;
                                c.g[AT][0] = 0x420c0000;
                                c.mtc1(1, 2);
                                c.g[T3][0] = 0xfffffffffffffc1f;
                                c.lwc1(1, (c.g[T4][0] as u32).wrapping_add(0x4));
                                c.g[T5][0] = 0x3;
                                c.lwc1(0, (c.g[T4][0] as u32).wrapping_add(0x14));
                                c.g[T6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                                c.lwc1(3, (c.g[T4][0] as u32).wrapping_add(0x24));
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x30));
                                c.g[T4][0] = 0xffffffffffffffff;
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x34));
                                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x38));
                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x3c));
                                c.mfc1(2, 2);
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                                c.qmtc2(2, 3);
                                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
                                c.g[AT][0] = 0x41900000;
                                c.mtc1(1, 0);
                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                                c.mul_s(0, 6, 0);
                                c.g[AT][0] = 0x41100000;
                                c.mtc1(1, 1);
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                                c.lqc2(3, (c.g[SP][0] as u32));
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
                                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
                                c.mul_s(1, 7, 1);
                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                                c.mfc1(3, 0);
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                                c.qmtc2(3, 3);
                                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
                                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                                c.mfc1(4, 1);
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                                c.qmtc2(4, 3);
                                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                                c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xe84));
                                c.ldl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
                                c.ldr(3, (c.g[V0][0] as u32));
                                c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0xf));
                                c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x8));
                                c.g[T7][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x1b));
                                c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x14));
                                c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x23));
                                c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x1c));
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x24), c.g[T7][0]);
                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xe84));
                                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x14));
                                c.sw((c.g[A0][0] as u32).wrapping_add(0xe84), c.g[V1][0]);
                                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
                                c.g[T7][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1420));
                                c.ldl(3, (c.g[T7][0] as u32).wrapping_add(0x7));
                                c.ldr(3, (c.g[T7][0] as u32));
                                c.ldl(4, (c.g[T7][0] as u32).wrapping_add(0xf));
                                c.ldr(4, (c.g[T7][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = c.lw((c.g[T7][0] as u32).wrapping_add(0x10));
                                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
                                c.sdr(3, (c.g[V0][0] as u32));
                                c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0xf));
                                c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                                c.g[V0][0] = c.g[V0][0] | c.g[T0][0];
                                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
                                c.g[V0][0] = c.g[V0][0] | 0x2;
                                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                                c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
                                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                c.g[V0][0] = c.g[V0][0] & c.g[T3][0];
                                c.g[V0][0] = c.g[V0][0] | 0xe0;
                                c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
                                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xfac));
                                c.sh((c.g[V1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                                c.ldl(16, (c.g[S1][0] as u32).wrapping_add(0xb));
                                c.ldr(16, (c.g[S1][0] as u32).wrapping_add(0x4));
                                c.ldl(2, (c.g[S1][0] as u32).wrapping_add(0x13));
                                c.ldr(2, (c.g[S1][0] as u32).wrapping_add(0xc));
                                c.sdl(16, (c.g[SP][0] as u32).wrapping_add(0x77));
                                c.sdr(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7f));
                                c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x78));
                                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x70));
                                c.mul_s(0, 0, 21);
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x70));
                                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
                                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x150));
                                c.g[AT][0] = 0x437f0000;
                                c.mtc1(1, 4);
                                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x74));
                                c.g[A2][0] = c.g[T6][0];
                                c.g[AT][0] = 0x43000000;
                                c.mtc1(1, 6);
                                c.g[A1][0] = 0x4;
                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x78));
                                c.mul_s(0, 0, 4);
                                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x7c));
                                c.g[A3][0] = 0u64;
                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                                c.mul_s(1, 1, 4);
                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x150));
                                c.mul_s(2, 2, 4);
                                c.mul_s(3, 3, 6);
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 5);
                                c.cvt_w_s(7, 0);
                                c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x90));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x140));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                c.cvt_w_s(0, 1);
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x94));
                                c.cvt_w_s(0, 2);
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x98));
                                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x88));
                                c.cvt_w_s(0, 3);
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x9c));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x80), 0u64);
                                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x84));
                                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
                                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x150));
                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x74));
                                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x78));
                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x7c));
                                c.mul_s(1, 1, 4);
                                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x70));
                                c.mul_s(2, 2, 4);
                                c.mul_s(3, 3, 4);
                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x150));
                                c.mul_s(0, 0, 6);
                                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xb8));
                                c.cvt_w_s(7, 1);
                                c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x140));
                                c.cvt_w_s(1, 2);
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xc4));
                                c.cvt_w_s(1, 3);
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xc8));
                                c.cvt_w_s(1, 0);
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xcc));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
                                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xb4));
                                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
                                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x150));
                                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x74));
                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x78));
                                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x7c));
                                c.mul_s(0, 0, 4);
                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                                c.mul_s(1, 1, 4);
                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x150));
                                c.mul_s(2, 2, 4);
                                c.mul_s(3, 3, 6);
                                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xe8));
                                c.cvt_w_s(7, 0);
                                c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0xf0));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x140));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                                c.cvt_w_s(0, 1);
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xf4));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0xe0), 0u64);
                                c.cvt_w_s(0, 2);
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xf8));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0xe4), 0u64);
                                c.cvt_w_s(0, 3);
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xfc));
                                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
                                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x150));
                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x74));
                                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x78));
                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x7c));
                                c.mul_s(1, 1, 4);
                                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x70));
                                c.mul_s(2, 2, 4);
                                c.mul_s(3, 3, 4);
                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x150));
                                c.mul_s(0, 0, 6);
                                c.cvt_w_s(4, 1);
                                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x120));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x140));
                                c.cvt_w_s(1, 2);
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x124));
                                c.cvt_w_s(1, 3);
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x128));
                                c.cvt_w_s(1, 0);
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x12c));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x118));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x114), 0u64);
                                c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x238));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x23c));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[T0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x002d556c)?;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xe84));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
                                c.sw((c.g[A0][0] as u32).wrapping_add(0xe84), c.g[V0][0]);
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
                                break 'b_2d5580;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
            break 'b_2d5584;
        }
        c.cycles += 5;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x160));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x198));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x190));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1a0));
        c.pc = t;
        return Ok(());
    })
}

// -------------------------------------------------- RiderBase_RenderAttackFX

/// `void RiderBase_RenderAttackFX(AttackFX* this)` (0x2f1510): while the
/// attack effect is active (`this+0xa0`), draw its four sprites. Pushes a
/// GS state record (copies the current 20-byte record in the graphics
/// manager's packet, `gfx+0xe84`, `gfx = gp-0x854`), loads the template at
/// 0x501420 into it, sets the frame buffer (`gfx+0xfb0`) and patches its
/// blend/test fields; then per sprite `i` the display's `DrawSprite`
/// (virtual +0x270/+0x274) with position `this+0x10+16i`, size `(s, s)`
/// (`s = this+0x90+4i`), uv `(0, 0)..(1, 1)` and colour `this+0x50+16i`;
/// finally pops the record.
///
/// The register use (the mask constants in a2..t7, the GPR temporaries of
/// the record copies) and the blocks follow the original's.
pub fn rider_base_render_attack_fx(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0xb0);
        'b_2f173c: {
            c.cycles += 13;
            c.g[T8][0] = c.g[A0][0];
            frame.save(c, S0, 0x90);
            frame.save(c, S1, 0x80);
            frame.save(c, S2, 0x70);
            frame.save(c, S3, 0x60);
            frame.save(c, S4, 0x50);
            frame.save(c, S5, 0x40);
            frame.save_ra(c, 0x30);
            frame.save_fpr(c, 20, 0xa0);
            c.g[V0][0] = c.lw(at(c, T8, 0xa0));
            let t = c.g[V0][0] == 0u64;
            c.g[V1][0] = c.lw(gpo(c, -0x854));
            if t {
                break 'b_2f173c;
            } else {
                c.cycles += 99;
                c.g[A1][0] = 0x500000;
                c.g[A2][0] = 0xfffffffffe7f0000;
                c.g[T0][0] = 0xffffffffffcf0000;
                c.g[V0][0] = c.lw(at(c, V1, 0xe84));
                c.g[A2][0] = c.g[A2][0] | 0xffff;
                c.g[T0][0] = c.g[T0][0] | 0xffff;
                c.g[T1][0] = 0xfffffffffff00000;
                c.ldl(4, at(c, V0, 0x7));
                c.ldr(4, (c.g[V0][0] as u32));
                c.ldl(7, at(c, V0, 0xf));
                c.ldr(7, at(c, V0, 0x8));
                c.g[T2][0] = c.lw(at(c, V0, 0x10));
                c.sdl(4, at(c, V0, 0x1b));
                c.sdr(4, at(c, V0, 0x14));
                c.sdl(7, at(c, V0, 0x23));
                c.sdr(7, at(c, V0, 0x1c));
                c.sw(at(c, V0, 0x24), c.g[T2][0]);
                c.g[T1][0] = c.g[T1][0] | 0xfff;
                c.g[A3][0] = 0x10000;
                c.g[T4][0] = 0xfffffffffffffffc;
                c.g[V0][0] = c.lw(at(c, V1, 0xe84));
                c.g[A3][0] = c.g[A3][0] | 0x4000;
                c.g[A0][0] = c.lw(gpo(c, -0x854));
                c.g[T2][0] = 0xffffffffe0000000;
                c.g[V0][0] = sx(at(c, V0, 0x14));
                c.g[T2][0] = c.g[T2][0] | 0x3ff;
                c.sw(at(c, V1, 0xe84), c.g[V0][0]);
                c.g[T5][0] = 0xfffffffffffffc1f;
                c.g[T6][0] = 0xffffffffffffff83;
                c.g[T3][0] = 0xffffffffffbf0000;
                c.g[V0][0] = c.lw(at(c, A0, 0xe84));
                c.g[T3][0] = c.g[T3][0] | 0xffff;
                c.g[T7][0] = 0x400000;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 20);
                c.g[A0][0] = sx(at(c, A1, 0x1420));
                c.ldl(16, at(c, A0, 0x7));
                c.ldr(16, (c.g[A0][0] as u32));
                c.ldl(17, at(c, A0, 0xf));
                c.ldr(17, at(c, A0, 0x8));
                c.g[V1][0] = c.lw(at(c, A0, 0x10));
                c.sdl(16, at(c, V0, 0x7));
                c.sdr(16, (c.g[V0][0] as u32));
                c.sdl(17, at(c, V0, 0xf));
                c.sdr(17, at(c, V0, 0x8));
                c.sw(at(c, V0, 0x10), c.g[V1][0]);
                c.g[S2][0] = sx(at(c, T8, 0x10));
                c.g[S5][0] = sx(stk(c, 0x10));
                c.g[S4][0] = sx(stk(c, 0x20));
                c.g[A0][0] = c.lw(gpo(c, -0x854));
                c.g[S1][0] = sx(at(c, T8, 0x50));
                c.g[S0][0] = sx(at(c, T8, 0x90));
                c.g[S3][0] = 0x3;
                c.g[V1][0] = c.lw(at(c, A0, 0xe84));
                c.g[V0][0] = c.lw(at(c, A0, 0xfb0));
                c.sh(at(c, V1, 0x10), c.g[V0][0]);
                c.g[A1][0] = c.lw(gpo(c, -0x854));
                c.g[V1][0] = c.lw(at(c, A1, 0xe84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                c.g[V0][0] = c.g[V0][0] | 0xc;
                c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                c.g[A0][0] = c.lw(at(c, A1, 0xe84));
                c.g[V0][0] = c.lw(at(c, A0, 0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.sw(at(c, A0, 0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw(at(c, A1, 0xe84));
                c.g[V0][0] = c.lw(at(c, V1, 0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                c.sw(at(c, V1, 0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw(at(c, A1, 0xe84));
                c.g[V0][0] = c.lw(at(c, A0, 0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
                c.sw(at(c, A0, 0x4), c.g[V0][0]);
                c.g[A2][0] = c.lw(at(c, A1, 0xe84));
                c.g[V0][0] = c.lw(at(c, A2, 0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T4][0];
                c.g[V0][0] = c.g[V0][0] | 0x2;
                c.sw(at(c, A2, 0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw(at(c, A1, 0xe84));
                c.g[V0][0] = c.lw(at(c, V1, 0x8));
                c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
                c.sw(at(c, V1, 0x8), c.g[V0][0]);
                c.g[A0][0] = c.lw(at(c, A1, 0xe84));
                c.g[V0][0] = c.lw(at(c, A0, 0x8));
                c.g[V0][0] = c.g[V0][0] & c.g[T5][0];
                c.g[V0][0] = c.g[V0][0] | 0xe0;
                c.sw(at(c, A0, 0x8), c.g[V0][0]);
                c.g[V1][0] = c.lw(at(c, A1, 0xe84));
                c.g[V0][0] = c.lw(at(c, V1, 0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T6][0];
                c.g[V0][0] = c.g[V0][0] | 0x1c;
                c.sw(at(c, V1, 0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw(at(c, A1, 0xe84));
                c.g[V0][0] = c.lw(at(c, A0, 0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T3][0];
                c.g[V0][0] = c.g[V0][0] | c.g[T7][0];
                c.sw(at(c, A0, 0x4), c.g[V0][0]);
                c.g[V0][0] = c.lw(gpo(c, -0x854));
                'l_2f16d0: loop {
                    c.cycles += 23;
                    c.g[A1][0] = c.g[S2][0];
                    c.lwc1(0, (c.g[S0][0] as u32));
                    c.g[T1][0] = c.g[S1][0];
                    c.g[V1][0] = c.lw(at(c, V0, 0x10d8));
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A3][0] = c.g[S5][0];
                    c.g[T0][0] = c.g[S4][0];
                    c.g[A0][0] = c.lh(at(c, V1, 0x270));
                    c.g[S1][0] = sx(at(c, S1, 0x10));
                    c.swc1(0, stk(c, 0x4));
                    c.g[S0][0] = sx(at(c, S0, 0x4));
                    c.swc1(0, stk(c, 0));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.sw(stk(c, 0x10), 0u64);
                    c.g[S2][0] = sx(at(c, S2, 0x10));
                    c.sw(stk(c, 0x14), 0u64);
                    c.swc1(20, stk(c, 0x20)); // uv max (1, 1)
                    c.swc1(20, stk(c, 0x24));
                    c.g[V0][0] = c.lw(at(c, V1, 0x274));
                    let t = c.g[V0][0] as u32;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                    call_ptr(c, t, 0x002f1724)?; // display->DrawSprite
                    let t = (c.g[S3][0] as i64) >= 0;
                    c.g[V0][0] = c.lw(gpo(c, -0x854));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002f16d0);
                        }
                        continue 'l_2f16d0;
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw(gpo(c, -0x854));
                        c.g[V0][0] = c.lw(at(c, V1, 0xe84));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
                        c.sw(at(c, V1, 0xe84), c.g[V0][0]);
                        break 'b_2f173c;
                    }
                }
            }
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore_ra(c, 0x30);
        frame.restore_fpr(c, 20, 0xa0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ComputerActorRider_SelectTargetRider

/// `int AI_GetFrameCounter()` (0x1298c8).
pub(crate) const AI_GET_FRAME_COUNTER: Callee = Callee {
    addr: 0x0012_98c8,
    f: ee::s00::AI_GetFrameCounter,
};

/// `bool ComputerActorRider_IsRivalTarget(actor, index)` (0x10d9e8).
pub(crate) const IS_RIVAL_TARGET: Callee = Callee {
    addr: 0x0010_d9e8,
    f: ee::s00::ComputerActorRider_IsRivalTarget,
};

/// `float atanf(float)` (0x31c228) and `float cosf(float)` (0x31c040).
pub(crate) const ATANF: u32 = 0x0031_c228;

pub(crate) const COSF: u32 = 0x0031_c040;

/// The target-rider score of table entry `s1`/index `s2` (f22 = its
/// distance): distance weighted by how far its heading is from the actor's
/// travel direction (`|cos|` + 1 for rivals, 1 - cos for others); the best
/// (smallest) score becomes the target and the rival flag goes to `*s4`.
pub(crate) fn score_target(c: &mut Cpu) -> Flow {
    use target::*;
    let actor = r32(c, S0);
    let gp = r32(c, GP);
    let s = sp(c);
    c.cycles += 15;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S2][0];
    IS_RIVAL_TARGET.call(c, 0x0010_be44)?;
    c.mtc1(0, 23);
    c.g[S3][0] = c.g[V0][0];
    c.g[V0][0] = c.lw(actor.wrapping_add(TABLE));
    let rider = r32(c, V0);
    c.lq(V1, rider.wrapping_add(RIDER_VELOCITY));
    c.sq(V1, s);
    c.lq(V1, rider.wrapping_add(RIDER_VELOCITY));
    c.lwc1(21, s.wrapping_add(0x4));
    c.sq(V1, s.wrapping_add(0x10));
    c.lwc1(20, s.wrapping_add(0x10));
    // f12 = the travel angle atan2-style from (x f20, y f21).
    c.c_eq_s(20, 23);
    let mut reload = true;
    if !c.fcc() {
        c.cycles += 7;
        c.div_s(12, 21, 20);
        call_ptr(c, ATANF, 0x0010_beac)?;
        c.c_lt_s(20, 23);
        let negative_x = c.fcc();
        c.mov_s(12, 0);
        if negative_x {
            c.cycles += 4;
            c.c_lt_s(23, 21);
            if !c.fcc() {
                c.sub_s(0, 0, 25);
            } else {
                c.cycles += 1;
                c.add_s(0, 0, 25);
            }
            c.cycles += 1;
            c.mov_s(12, 0);
        }
    } else {
        c.cycles += 4;
        c.c_eq_s(21, 23);
        let zero = c.fcc();
        c.mov_s(12, 21);
        if !zero {
            // +-pi/2.
            c.cycles += 4;
            c.c_le_s(23, 21);
            c.lwc1(12, gp.wrapping_sub(0x7df4));
            let up = c.fcc();
            c.g[V0][0] = c.lw(actor.wrapping_add(TABLE));
            if up {
                c.cycles += 3;
                c.lwc1(12, gp.wrapping_sub(0x7df0));
            }
            reload = false;
        }
    }
    if reload {
        c.cycles += 1;
        c.g[V0][0] = c.lw(actor.wrapping_add(TABLE));
    }
    // The heading difference wrapped to a turn: d - round(d / 2pi) * 2pi.
    c.cycles += 14;
    c.lwc1(1, gp.wrapping_sub(0x7dec));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S1)));
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(AT, 2);
    c.lwc1(0, r32(c, V0).wrapping_add(ENTRY_HEADING));
    c.sub_s(0, 0, 12);
    c.mul_s(1, 0, 1);
    c.add_s(1, 1, 2);
    c.cvt_w_s(2, 1);
    c.cvt_s_w(2, 2);
    c.c_lt_s(1, 2);
    if c.fcc() {
        c.sub_s(2, 2, 24);
    }
    c.cycles += 6;
    c.lwc1(12, gp.wrapping_sub(0x7de8));
    c.mul_s(12, 2, 12);
    c.sub_s(12, 0, 12);
    call_ptr(c, COSF, 0x0010_bf20)?;
    if c.g[S3][0] == 0 {
        c.sub_s(0, 24, 0);
    } else {
        c.cycles += 2;
        c.abs_s(0, 0);
        c.add_s(0, 0, 24);
    }
    c.cycles += 5;
    c.mul_s(22, 22, 0);
    c.c_le_s(22, 26);
    if c.fcc() {
        c.cycles += 4;
        c.sw(actor.wrapping_add(TARGET), c.g[S2][0]);
        c.mov_s(26, 22);
        c.sw(r32(c, S4), c.g[S3][0]);
    }
    Ok(())
}

/// `bool ComputerActorRider_SelectTargetRider(ComputerActor* actor, int*
/// rival)` (0x10bd10): keep the current target rider while it is within
/// range and the actor within 600 units of the finish; otherwise, every 12th
/// AI frame and while within 540 units of the finish, pick the best-scoring
/// rider within 700 units. Returns whether there is a target.
pub fn computer_actor_rider_select_target_rider(c: &mut Cpu) {
    use target::*;
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S0, 0x70);
        frame.save(c, S4, 0x30);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save_ra(c, 0x20);
        for (f, off) in [
            (26, 0xb0),
            (25, 0xa8),
            (24, 0xa0),
            (23, 0x98),
            (22, 0x90),
            (21, 0x88),
            (20, 0x80),
        ] {
            frame.save_fpr(c, f, off);
        }
        c.g[S0][0] = c.g[A0][0];
        let actor = r32(c, S0);
        let gp = r32(c, GP);
        c.g[V1][0] = c.lw(actor.wrapping_add(TARGET));
        c.g[S4][0] = c.g[A1][0];
        'done: {
            if (c.g[V1][0] as i64) >= 0 {
                // Drop the target when it is beyond the range at gp-0x7e00
                // or the actor is more than 600 units from the finish.
                c.cycles += 10;
                c.g[V0][0] = ENTRY_SIZE as u64;
                c.g[A0][0] = c.lw(actor.wrapping_add(TABLE));
                c.mult(0, V0, V1, V0);
                c.lwc1(1, gp.wrapping_sub(0x7e00));
                c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                c.lwc1(0, r32(c, V0).wrapping_add(ENTRY_DIST));
                c.c_lt_s(1, 0);
                let mut drop = c.fcc();
                c.g[V0][0] = u64::MAX;
                if !drop {
                    c.cycles += 7;
                    c.lwc1(1, r32(c, A0).wrapping_add(RIDER_FINISH_DIST));
                    c.g[AT][0] = 0x4416_0000; // 600.0
                    c.mtc1(AT, 0);
                    c.c_lt_s(0, 1);
                    drop = c.fcc();
                }
                if drop {
                    c.cycles += 2;
                    c.sw(actor.wrapping_add(TARGET), c.g[V0][0]);
                }
                c.g[V0][0] = c.lw(actor.wrapping_add(TARGET));
                c.cycles += 2;
                if (c.g[V0][0] as i64) >= 0 {
                    c.g[V0][0] = c.lw(actor.wrapping_add(TARGET));
                    break 'done;
                }
            }
            // Choose only below 540 units to the finish, every 12th frame.
            c.cycles += 8;
            c.g[V0][0] = c.lw(actor.wrapping_add(TABLE));
            c.g[AT][0] = 0x4407_0000; // 540.0
            c.mtc1(AT, 1);
            c.lwc1(0, r32(c, V0).wrapping_add(RIDER_FINISH_DIST));
            c.c_lt_s(0, 1);
            if !c.fcc() {
                c.g[V0][0] = c.lw(actor.wrapping_add(TARGET));
                break 'done;
            }
            c.cycles += 6;
            AI_GET_FRAME_COUNTER.call(c, 0x0010_bdd4)?;
            c.g[V1][0] = 0xc;
            c.div(0, V0, V1);
            c.cycles += 3;
            c.g[A0][0] = c.hi[0];
            if c.g[A0][0] != 0 {
                c.g[V0][0] = c.lw(actor.wrapping_add(TARGET));
                break 'done;
            }
            c.cycles += 6;
            c.lwc1(26, gp.wrapping_sub(0x7dfc));
            c.g[S2][0] = 0;
            c.lwc1(25, gp.wrapping_sub(0x7df8));
            c.g[S1][0] = 0;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 24);
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.lw(actor.wrapping_add(TABLE));
                c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
                let entry = r32(c, V0);
                c.g[V1][0] = c.lw(entry);
                if c.g[V1][0] != 0 {
                    c.cycles += 7;
                    c.g[AT][0] = 0x442f_0000; // 700.0
                    c.mtc1(AT, 0);
                    c.lwc1(22, entry.wrapping_add(ENTRY_DIST));
                    c.c_lt_s(0, 22);
                    if !c.fcc() {
                        score_target(c)?;
                    }
                }
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S2][0] as i64) < 6) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(ENTRY_SIZE));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0010_be08);
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw(actor.wrapping_add(TARGET));
        }
        // v0 = target >= 0.
        c.cycles += 17;
        c.g[V0][0] = !c.g[V0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 31);
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore_ra(c, 0x20);
        for (f, off) in [
            (26, 0xb0),
            (25, 0xa8),
            (24, 0xa0),
            (23, 0x98),
            (22, 0x90),
            (21, 0x88),
            (20, 0x80),
        ] {
            frame.restore_fpr(c, f, off);
        }
        frame.close(c);
        Ok(())
    })
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
    use super::*;
    use super::*;
    use super::*;
    use super::*;
    use super::*;
    use crate::difftest::fakes::{ptr, vtable, LOG_RESULTS};
    use crate::difftest::{check_lohi, fresh, put_words};

    pub(crate) use crate::game::fe::ovstatetrophy::tests::slot;
    pub(crate) use crate::game::fe::ovstatetrophy::tests::RET0;
    pub(crate) use crate::game::fe::ovstatetrophy::tests::RET1;

    /// WPData_FindCellIndex on a real quadtree (QuadTree_FindLeaf): a root
    /// leaf, or a tree of up to three internal levels with random leaf
    /// cells; points inside the tree and outside it (negative or too large
    /// grid coordinates: no leaf).
    #[test]
    fn wp_data_find_cell_index_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::DATA;
        const WP: u32 = DATA;
        const TREE: u32 = DATA + 0x100;
        const NODES: u32 = DATA + 0x1000;
        crate::difftest::check(
            ee::s08::f_002baf90__generated,
            wp_data_find_cell_index,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2baf_90, 0);
                c.pc = 0x002b_af90;
                let f = |x: f32| x.to_bits();
                c.bus.write32(WP, TREE);
                // Grid: 0x8000 cells over 0..1024 world units.
                c.bus.write32(TREE, f(32.0));
                c.bus.write32(TREE + 4, f(0.0));
                c.bus.write32(TREE + 8, f(0.0));
                c.bus.write16(TREE + 0x14, 0);
                c.bus.write32(TREE + 0x20, NODES);
                // Nodes (8 bytes): internal = four child halfwords (index << 1,
                // bit 0 of the first set), leaf = even first halfword and its
                // cell index at +4.
                let depth = seed % 4;
                let mut next = 1u32;
                let mut level = vec![0u32];
                for d in 0..=depth {
                    let mut below = Vec::new();
                    for &n in &level {
                        let node = NODES + 8 * n;
                        if d < depth && r.next_u32() % 4 != 0 {
                            for k in 0..4u32 {
                                let child = next;
                                next += 1;
                                below.push(child);
                                c.bus
                                    .write16(node + 2 * k, (child << 1 | (k == 0) as u32) as u16);
                            }
                        } else {
                            c.bus.write16(node, (r.next_u32() as u16) & !1);
                            c.bus.write32(node + 4, r.next_u32() % 5000);
                        }
                    }
                    level = below;
                }
                let coord = |r: &mut crate::difftest::Rng| match r.next_u32() % 8 {
                    0 => -((r.next_u32() % 100) as f32) - 1.0,
                    1 => 1024.0 + (r.next_u32() % 100) as f32,
                    _ => (r.next_u32() % 1024) as f32 + 0.5,
                };
                c.f[12] = f(coord(&mut r));
                c.f[13] = f(coord(&mut r));
                c.g[A0][0] = sx(WP);
            },
        );
    }

    // -- 0x0010def0 ComputerActorRider_ComputeRubberBand ---------------------

    /// `ComputerActorRider_ComputeRubberBand`: no human (no slot, or an
    /// empty one), every mode, game mode 2, near the finish, catching up,
    /// holding back and in between; the back end's interface getter is a
    /// fake that may unwind.
    #[test]
    fn computer_actor_rider_compute_rubber_band_matches() {
        use crate::difftest::fakes::{machine, vtable};
        use crate::difftest::{check, put_words, DATA, GAME_GP};
        const THIS: u32 = DATA + 0x8_0000;
        const PARAMS: u32 = DATA + 0x8_1000;
        const APP: u32 = DATA + 0x8_2000;
        const MODULE: u32 = DATA + 0x8_2100;
        const PLAYERS: u32 = DATA + 0x8_2200;
        const HUMAN: u32 = DATA + 0x8_3000;
        const BE_LIBRARY: u32 = DATA + 0x8_4000;
        const BE_OBJ: u32 = DATA + 0x8_4100;
        const BE_INTERFACES: u32 = 0x0053_08a8;
        let f = |x: f32| x.to_bits();
        check(
            ssx3_ee::s00::f_0010def0__generated,
            computer_actor_rider_compute_rubber_band,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_def0, 2);
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                // The back end: the library exists, interface 0's getter is
                // a fake.
                w(c, GAME_GP - 0x1f04, BE_LIBRARY);
                w(c, GAME_GP - 0x1efc, 0);
                w(c, BE_INTERFACES, BE_OBJ);
                let vt = vtable(c, &mut r, &[8]);
                w(c, BE_OBJ + 4, vt);
                put_words(c, GAME_GP - 0x7cc4, &[f(50000.0), f(1.1), f(1.0 / 1.1)]);

                // The player slots: the human in slot 0..5 (or none, 6).
                let human = seed % 7;
                for i in 0..6 {
                    let (a, b) = if i == human {
                        (1 + r.next_u32() % 9, 1 + r.next_u32() % 9)
                    } else if r.next_u32() % 2 == 0 {
                        (0, r.next_u32() % 3)
                    } else {
                        (r.next_u32() % 3, 0)
                    };
                    w(c, PARAMS + 0x24 * i, a);
                    w(c, PARAMS + 0x24 * i + 4, b);
                }
                w(c, GAME_GP - 0x848, APP);
                w(c, APP + 0x84, MODULE);
                w(c, MODULE + 0xc, PLAYERS);
                if human < 6 {
                    let p = if seed % 11 == 0 { 0 } else { HUMAN };
                    w(c, PLAYERS + 4 * human + 0x28, p);
                }

                // Mode, game mode, distances.
                w(c, THIS + 0x18, PARAMS);
                w(c, PARAMS + 0xe4, (seed / 7) % 4);
                c.bus
                    .write8(0x0053_5c11, if seed % 13 == 0 { 2 } else { 1 });
                let own = if seed % 9 == 0 {
                    (r.next_u32() % 50000) as f32
                } else {
                    50000.0 + (r.next_u32() % 100000) as f32
                };
                w(c, PARAMS + 0x4d0, f(own));
                let k = 100.0 + (r.next_u32() % 5000) as f32;
                let k2 = 100.0 + (r.next_u32() % 5000) as f32;
                w(c, PARAMS + 0xdc, f(k));
                w(c, PARAMS + 0xe0, f(k2));
                let delta = (r.next_u32() % 20000) as f32 - 10000.0;
                w(c, HUMAN + 0x4d0, f(own + delta));

                c.g[A0][0] = sx(THIS);
                c.pc = 0x0010_def0;
            },
        );
    }

    pub(crate) mod anger {
        const B: u32 = 0x00b0_0000;
        pub const FX: u32 = B;
        pub const RIDER: u32 = B + 0x1000;
        pub const RIDER_VT: u32 = B + 0x2000;
        pub const SKEL: u32 = B + 0x2100;
        pub const BONES: u32 = B + 0x2200;
        pub const RECORDS: u32 = B + 0x3000;
        pub const WORLD: u32 = B + 0x4000;
        pub const RACE: u32 = B + 0x4100;
        pub const CUR: u32 = B + 0x4200;
        pub const RENDERER: u32 = B + 0x5000;
        pub const PACKET: u32 = B + 0x7000;
        pub const VIEW: u32 = B + 0x7800;
    }

    /// The effect, its rider (vtable slot +0x40 a real 0/1 function), a
    /// bone matrix and a view matrix placing the bone from behind the
    /// camera to past 2000 units, records with a zero or non-zero
    /// intensity, and the renderer (fakes for +0x118, which returns the
    /// view matrix, and +0x238).
    pub(crate) fn anger_fx_render_setup(c: &mut Cpu, seed: u32) {
        use anger::*;
        crate::difftest::elf_image(c);
        let mut r = machine(c, seed, 0x002d_5048, 2);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), WORLD);
        c.bus.write32(GAME_GP.wrapping_sub(0x854), RENDERER);
        c.g[A0][0] = FX as u64;
        c.bus.write32(FX, RIDER);
        c.bus.write32(FX + 4, RECORDS);
        // Rider vtable slot +0x40: non-zero (skip) for some seeds.
        c.bus.write32(RIDER + 0x6c0, RIDER_VT);
        c.bus.write16(RIDER_VT + 0x40, (r.next_u32() & 0x1c) as u16);
        c.bus
            .write32(RIDER_VT + 0x44, if seed % 9 == 0 { RET1 } else { RET0 });
        // Current racer slot and its record.
        c.bus.write32(WORLD + 0x84, RACE);
        c.bus.write32(RACE + 0x84, CUR);
        let slot = r.next_u32() % 4;
        c.bus.write32(CUR + 0x14, slot);
        for i in 0..4 {
            let rec = RECORDS + 0x2c * i;
            let intensity = if seed % 7 == 1 { 0 } else { r.f32(1.0) };
            c.bus.write32(rec, intensity);
            for w in 1..5 {
                c.bus.write32(rec + 4 * w, r.f32(1.0));
            }
            c.bus.write32(rec + 0x14, r.f32(200.0));
            c.bus.write32(rec + 0x18, r.f32(200.0));
        }
        // Bone matrix translation (+0x30) of bone `rider+0x8a8`.
        let bone = r.next_u32() % 4;
        c.bus.write32(RIDER + 0x780, SKEL);
        c.bus.write32(RIDER + 0x8a8, bone);
        c.bus.write32(SKEL + 0x30, BONES);
        for b in 0..4 {
            put_vec(
                c,
                BONES + 0x40 * b + 0x30,
                [r.f32(500.0), r.f32(500.0), r.f32(500.0), 1.0f32.to_bits()],
            );
        }
        // View matrix rows; the translation row's z spreads the view depth
        // over -1000 .. 3000 (behind, near, fading, too far).
        for row in 0..3 {
            put_vec(
                c,
                VIEW + 0x10 * row,
                [r.f32(1.0), r.f32(1.0), r.f32(0.5), 0],
            );
        }
        let depth = -1000.0 + (r.next_u32() % 4000) as f32;
        put_vec(
            c,
            VIEW + 0x30,
            [
                r.f32(100.0),
                r.f32(100.0),
                depth.to_bits(),
                1.0f32.to_bits(),
            ],
        );
        // Renderer: +0x118 returns the view matrix (first fake call).
        let rvt = vtable(c, &mut r, &[0x118, 0x238]);
        c.bus.write32(RENDERER + 0x10d8, rvt);
        c.bus.write32(RENDERER + 0xe84, PACKET);
        c.bus.write32(RENDERER + 0xfac, r.next_u32());
        fill(c, &mut r, PACKET, 0x10);
        c.bus.write32(LOG_RESULTS, VIEW);
        c.pc = 0x002d_5048;
    }

    /// Skipped by the rider query, zero intensity, behind the camera, too
    /// far, fading and full glow; unwinds at both renderer calls.
    #[test]
    fn c_anger_fx_render_matches() {
        covered(
            ee::s09::f_002d5048__generated,
            c_anger_fx_render,
            240,
            &anger_fx_render_setup,
        );
    }

    // -- 0x002f1510 RiderBase_RenderAttackFX ----------------------------------

    /// `RiderBase_RenderAttackFX`: inactive and active effects with random
    /// sprites, a random packet record and template, the display's
    /// `DrawSprite` a fake (some calls unwind).
    #[test]
    fn rider_base_render_attack_fx_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        use crate::difftest::{fill, DATA, GAME_GP};
        const THIS: u32 = DATA;
        const GFX: u32 = DATA + 0x1_0000;
        const DISP: u32 = DATA + 0x2_0000;
        const PKT: u32 = DATA + 0x3_0000;
        const TEMPLATE: u32 = 0x0050_1420;
        covered(
            ee::s09::f_002f1510__generated,
            rider_base_render_attack_fx,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f_1510, 4);
                let r = &mut r;
                c.pc = 0x002f_1510;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                w(c, GAME_GP.wrapping_sub(0x854), GFX);
                w(c, GFX + 0x10d8, DISP);
                c.bus
                    .write16(DISP + 0x270, (r.next_u32() as i16 & !3) as u16);
                w(c, DISP + 0x274, FAKE);
                w(c, GFX + 0xe84, PKT + 4 * (r.next_u32() % 8));
                w(c, GFX + 0xfb0, r.next_u32());
                fill(c, r, PKT, 0x40);
                fill(c, r, TEMPLATE, 5);
                fill(c, r, THIS, 0x30);
                for i in 0..4 {
                    w(c, THIS + 0x90 + 4 * i, r.f32(64.0));
                }
                w(
                    c,
                    THIS + 0xa0,
                    if seed % 5 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.g[A0][0] = sx(THIS);
            },
        );
    }

    /// ComputerActorRider_SelectTargetRider: a current target (none, near,
    /// far) and the actor's distance to the finish (below/above 540 and
    /// 600), the AI frame counter (a multiple of 12 or not), six table
    /// entries (empty, near, beyond 700), the rival flags and the actor's
    /// travel direction (zero x and/or y, either sign) so every heading
    /// branch and the real `atanf`/`cosf` run.
    #[test]
    fn computer_actor_rider_select_target_rider_matches() {
        check(
            ee::s00::f_0010bd10__generated,
            computer_actor_rider_select_target_rider,
            300,
            &|c, seed| {
                let mut r = Rng(0x10bd_10 ^ (seed as u64) << 24);
                scramble(c, &mut r);
                c.pc = 0x0010_bd10;
                c.g[GP][0] = GAME_GP as u64;
                let actor = DATA;
                let table = DATA + 0x400;
                let flag = DATA + 0x700;
                // AI_GetFrameCounter: [[[gp-0x848]+0x84]+0xc]+8.
                let (a, b, d) = (DATA + 0x5000, DATA + 0x5100, DATA + 0x5200);
                c.bus.write32(GAME_GP - 0x848, a);
                c.bus.write32(a + 0x84, b);
                c.bus.write32(b + 0xc, d);
                c.bus.write32(
                    d + 8,
                    if seed % 3 == 0 {
                        7
                    } else {
                        12 * (r.next_u32() % 50)
                    },
                );
                // Rivals: IsRivalTarget reads the actor's table +0xf0 / +0xf8.
                c.bus.write32(table + 0xf0, r.next_u32() % 2);
                c.bus.write32(table + 0xf8, r.next_u32() % 6);
                for (off, v) in [
                    (0x7e00u32, 500.0f32),
                    (0x7dfc, 1.0e9),
                    (0x7df8, std::f32::consts::PI),
                    (0x7df4, -std::f32::consts::FRAC_PI_2),
                    (0x7df0, std::f32::consts::FRAC_PI_2),
                    (0x7dec, 1.0 / (2.0 * std::f32::consts::PI)),
                    (0x7de8, 2.0 * std::f32::consts::PI),
                ] {
                    c.bus.write32(GAME_GP - off, v.to_bits());
                }
                c.bus.write32(actor + 0x18, table);
                c.bus
                    .write32(actor + 0xe70, [u32::MAX, 1, 3][(r.next_u32() % 3) as usize]);
                for k in 0..6u32 {
                    let e = table + 0x24 * k;
                    c.bus.write32(
                        e,
                        if r.next_u32() % 4 == 0 {
                            0
                        } else {
                            DATA + 0x900
                        },
                    );
                    let dist = [100.0f32, 650.0, 800.0][(r.next_u32() % 3) as usize];
                    c.bus.write32(e + 8, dist.to_bits());
                    c.bus.write32(e + 0xc, r.f32(4.0));
                }
                let finish = [300.0f32, 560.0, 700.0][(r.next_u32() % 3) as usize];
                c.bus.write32(table + 0x4c8, finish.to_bits());
                let comp = |r: &mut Rng| match r.next_u32() % 4 {
                    0 => 0,
                    _ => r.f32(10.0),
                };
                put_vec(
                    c,
                    table + 0x1b0,
                    [comp(&mut r), comp(&mut r), r.next_u32(), r.next_u32()],
                );
                c.g[A0][0] = sx(actor);
                c.g[A1][0] = sx(flag);
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
