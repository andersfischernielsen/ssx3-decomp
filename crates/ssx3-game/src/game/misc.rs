//! `src/misc.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::addu;
use crate::abi::r32;
use crate::abi::reg;
use crate::abi::ret;
use crate::abi::set_reg;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::ai::gp;
pub(crate) use crate::game::fe::ovstatehud::oc224_gp;
pub(crate) use crate::game::fe::ovstatehud::oc224_ld_u;
pub(crate) use crate::game::fe::ovstatehud::oc224_sd_u;
pub(crate) use crate::game::fe::splashmanager::gpo;
pub(crate) use crate::game::fe::splashmanager::oc230_gp;
pub(crate) use crate::game::fe::splashmanager::stk;
pub(crate) use crate::game::intersect::aifwddiff::oc230_add;
pub(crate) use crate::game::intersect::aifwddiff::oc230_at;
pub(crate) use crate::game::intersect::aifwddiff::oc230_sll;
pub(crate) use crate::game::main::gameload::vcall;
pub(crate) use crate::game::main::gameload::R2D_CTX_GP;
pub(crate) use crate::game::main::gameload::R2D_STATE_TOP;
pub(crate) use crate::game::ui::uistate::oc224_at;
use crate::game::visualfx::boardwakefx as bw;
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
    (0x0011_3e80, rider_set_slew0_target, "Rider_SetSlew0Target"),
    (0x0012_20d8, rider_build_light_set, "Rider_BuildLightSet"),
    (0x0021_e7a8, hud_align_y, "HUD_AlignY"),
    (0x0022_00c0, hud_draw_speedometer, "HUD_DrawSpeedometer"),
    (0x002f_4a08, lens_fx_draw_sun, "LensFX_DrawSun"),
    (0x003d_4f10, path_update_tracks, "PATH_UpdateTracks"),
    (0x003e_12e0, stream_file_get_chunk, "StreamFile_GetChunk"),
    (0x0042_c078, d_intr, "DIntr"),
    (
        0x0010_aa70,
        computer_actor_rider_think_prewind,
        "ComputerActorRider_ThinkPrewind",
    ),
    (
        0x0010_b0e8,
        computer_actor_rider_update_spin_plan,
        "ComputerActorRider_UpdateSpinPlan",
    ),
    (
        0x0010_bb18,
        computer_actor_rider_find_speed_marker,
        "ComputerActorRider_FindSpeedMarker",
    ),
    (
        0x0011_4cc0,
        rider_turn_around_if_backward,
        "Rider_TurnAroundIfBackward",
    ),
    (
        0x0011_61d0,
        rider_try_enter_board_press,
        "Rider_TryEnterBoardPress",
    ),
    (
        0x002d_bf98,
        ps2_fog_particle_man_render,
        "cPS2FogParticleMan_Render",
    ),
    (
        0x002f_0a98,
        ps2_visual_effects_man_update,
        "PS2VisualEffectsMan_Update",
    ),
    (
        0x002f_0cd0,
        ps2_visual_effects_man_render,
        "PS2VisualEffectsMan_Render",
    ),
    (0x003b_96b8, sndmidi_service, "SNDMIDI_Service"),
    (0x003d_5128, path_update, "PATH_Update"),
    (0x003f_f9a8, libpad_get_dma_buffer, "libpad_GetDmaBuffer"),
    (0x003f_fa58, sce_pad_read, "scePadRead"),
    (0x0040_d588, tanf, "tanf"),
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

pub(crate) mod rider_lights {

    /// The rider's vtable (`+0x38` slot: the AI/view index getter).
    pub const VTABLE: u32 = 0x6c0;

    pub const AI_INDEX_SLOT: u32 = 0x38;

    /// `RiderHandler*` (its `+0xd30` holds the extra directional lights).
    pub const HANDLER: u32 = 0x77c;

    /// Eight dynamic lights affecting the rider (null for none).
    pub const DYNAMIC_LIGHTS: u32 = 0x794;

    /// The rider's irradiance (light set).
    pub const LIGHTS: u32 = 0x7c0;

    /// Handler offset of the directional-light block: `{x, y, z, n}` and
    /// `n` lights (`+0x10` points, 12 bytes each; `+0x50` directions, 16
    /// bytes each).
    pub const HANDLER_LIGHTS: u32 = 0xd30;
}

pub(crate) mod renderer {
    pub(crate) use crate::game::fe::ovstatehud::renderer::CONTEXT_ID;
    pub(crate) use crate::game::fe::ovstatehud::renderer::GP;
    pub(crate) use crate::game::fe::ovstatehud::renderer::STATE;
    pub(crate) use crate::game::fe::ovstatehud::renderer::VTABLE;
}

pub(crate) mod lens_fx {

    /// Non-zero while the sun is on screen (halfword).
    pub const VISIBLE: u32 = 0x2c;

    /// The sun's screen position (x, y).
    pub const SUN_X: u32 = 0x1f0;

    pub const SUN_Y: u32 = 0x1f4;

    /// The flare intensity.
    pub const INTENSITY: u32 = 0x204;
}

pub(crate) mod sun_callee {
    use super::Callee;
    use ssx3_ee as ee;

    pub const RED: Callee = Callee {
        addr: 0x002e_e100,
        f: ee::s09::WorldPainter_GetSunRed,
    };

    pub const GREEN: Callee = Callee {
        addr: 0x002e_e148,
        f: ee::s09::WorldPainter_GetSunGreen,
    };

    pub const BLUE: Callee = Callee {
        addr: 0x002e_e190,
        f: ee::s09::WorldPainter_GetSunBlue,
    };

    pub const INTENSITY: Callee = Callee {
        addr: 0x002e_e1d8,
        f: ee::s09::WorldPainter_GetSunIntensity,
    };

    pub const SIZE: Callee = Callee {
        addr: 0x002e_e2b0,
        f: ee::s09::WorldPainter_GetSunSize,
    };

    pub const TEXTURE: Callee = Callee {
        addr: 0x002e_e220,
        f: ee::s09::WorldPainter_GetSunTexture,
    };

    pub const DRAW_FLARE_SET: Callee = Callee {
        addr: 0x002f_4690,
        f: ee::s09::LensFX_DrawFlareSet,
    };
}

pub(crate) mod path {

    /// `PathState*`: tick lengths at +0xc (normal) and +0x10 (streamed),
    /// the 24 track slots at +0x58.
    pub const STATE: u32 = 0x0044_f420;

    /// Mode byte: `'B'` = streamed (block) mode.
    pub const MODE: u32 = 0x0044_f434;

    /// Current PATH time.
    pub const TIME: u32 = 0x0044_f438;

    pub const TRACKS: u32 = 24;

    pub const SLOT0: u32 = 0x58;
}

pub(crate) mod track {

    pub const BUSY: u32 = 0x3;

    pub const VOLUME_FADE: u32 = 0xd;

    pub const PLAY_STATE: u32 = 0xe;

    pub const AUX_FADE: u32 = 0x15;

    pub const FADE3: u32 = 0x1d;

    pub const LEAD: u32 = 0x24;

    pub const STREAMED: u32 = 0x2f;

    pub const ACTIVE: u32 = 0x30;

    pub const NODE: u32 = 0x36;

    pub const DEADLINE: u32 = 0x48;

    pub const STOP_AT: u32 = 0x4c;

    pub const HANDLE: u32 = 0x54;
}

pub(crate) mod speedo {

    /// The HUD settings block copied to the stack (0x288 bytes, 8-aligned).
    pub const SETTINGS: u32 = 0x0053_5610;

    pub const SETTINGS_SIZE: u32 = 0x280;

    /// Settings word 0: bit 19 clear = the converted units (factor and
    /// second label).
    pub const UNITS_FLAG: u64 = 0x8_0000;

    /// The rider's velocity provider (an object with a vtable).
    pub const RIDER_VELOCITY: u32 = 0x6c0;

    pub const SLOT_GET_VELOCITY: u32 = 0x10;

    /// The speedometer's HUD element (element 25 of 0x24 bytes).
    pub const ELEM: u32 = 0x384;

    pub const ELEM_ALIGN_X: u32 = 0x3a4;

    pub const ELEM_ALIGN_Y: u32 = 0x3a5;

    /// The HUD's text object.
    pub const HUD_TEXT: u32 = 0x42c;

    /// Text object fields: colour (8 bytes at 0x40), base scale, scale.
    pub const TEXT_COLOR: u32 = 0x40;

    pub const TEXT_BASE_SCALE: u32 = 0x30;

    pub const TEXT_SCALE: u32 = 0x38;

    pub const TEXT_LINE: u32 = 0x14;

    /// The two unit labels (flag set / clear) and the number format.
    pub const LABEL_A: u32 = 0x004a_2308;

    pub const LABEL_B: u32 = 0x004a_2310;

    pub const NUMBER_FORMAT: u32 = 0x004a_2138;

    /// `gp -` constants: speed scale, units conversion factor, the two digit-scale
    /// factors (fewer than three digits or not), the label offset.
    pub const GP_NEG_SPEED_SCALE: u32 = 0x5320;

    pub const GP_NEG_UNITS_FACTOR: u32 = 0x531c;

    pub const GP_NEG_SCALE_SHORT: u32 = 0x5318;

    pub const GP_NEG_SCALE_LONG: u32 = 0x5314;

    pub const GP_NEG_LABEL_OFFSET: u32 = 0x5310;

    // Callees (all called through the function table).
    pub const BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;

    pub const BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;

    pub const HUD_GET_ELEM_POS: u32 = 0x001e_91f8;

    pub const HUD_GET_ELEM_SCALE: u32 = 0x001e_9290;

    pub const HUD_GET_ELEM_COLOR: u32 = 0x001e_91a8;

    pub const FONT_MEASURE_TEXT: u32 = 0x0039_1fb0;

    pub const HUD_ALIGN_ELEM_POS: u32 = 0x001f_10f8;

    pub const FONT_DRAW_TEXT: u32 = 0x0039_1cb0;

    pub const SPRINTF: u32 = 0x0041_7828;

    pub const STRLEN: u32 = 0x0041_6810;

    pub const FONT_DRAW_TEXT_CENTERED: u32 = 0x0039_2908;

    pub const COPY_HEAD: u32 = 0x0022_01e0;
}

pub(crate) mod chunk {
    use super::ee;
    use super::Callee;

    /// Stream manager (`*handle`): the ring start (`+0x30`) and its mutex
    /// (`+4`).
    pub const MAN_MUTEX: u32 = 0x4;

    pub const MAN_RING: u32 = 0x30;

    /// Stream handle: its tag (`+4`), the bytes still queued (`+8`), the
    /// next chunk (`+0xc`).
    pub const H_TAG: u32 = 0x4;

    pub const H_QUEUED: u32 = 0x8;

    pub const H_NEXT: u32 = 0xc;

    /// Chunk header: `+0` -1 = wrap to the ring start; `+4` tag (top byte)
    /// and size (low 24 bits).
    pub const CH_WRAP: u32 = 0x0;

    pub const CH_TAG_SIZE: u32 = 0x4;

    pub const SIZE_MASK: u64 = 0xff_ffff;

    pub const TAG_MASK: u64 = 0xffff_ffff_ff00_0000;

    pub const HEAD: u32 = 0x003e_13a0;

    pub const STREAM_FILE_VALIDATE: Callee = Callee {
        addr: 0x003d_fbd0,
        f: ee::s14::StreamFile_Validate,
    };

    pub const EA_MUTEX_LOCK: Callee = Callee {
        addr: 0x003e_5700,
        f: ee::s14::EAMutex_Lock,
    };

    pub const EA_MUTEX_UNLOCK: Callee = Callee {
        addr: 0x003e_5760,
        f: ee::s14::EAMutex_Unlock,
    };
}

// ---------------------------------------------------------------------------
// HUD_AlignY (0x0021e7a8)

/// `float HUD_AlignY(void* self, int align, float pos, float size, float
/// scale)` (0x0021e7a8): the top edge of a HUD element aligned at `pos`.
/// `align == 0` centres it (`pos - scale*size*0.5`), `align > 0` bottom
/// aligns it (`pos - scale*size`), `align < 0` keeps `pos` (top aligned).
/// Result in f0 (and f12 holds it too).
pub fn hud_align_y(c: &mut Cpu) {
    use crate::ps2f::FpuF32;
    let align = c.g[A1][0];
    let pos = FpuF32(c.f[12]);
    let size = FpuF32(c.f[13]);
    let scale = FpuF32(c.f[14]);
    c.cycles += 2;
    let mut top = pos;
    if align == 0 {
        c.cycles += 5;
        const HALF: u32 = 0x3f00_0000;
        c.g[AT][0] = HALF as u64;
        c.f[1] = HALF;
        let offset = (scale * size) * FpuF32(HALF);
        c.f[0] = offset.0;
        c.cycles += 1;
        top = pos - offset;
    } else {
        c.cycles += 2;
        if (align as i64) > 0 {
            c.cycles += 1;
            let offset = scale * size;
            c.f[0] = offset.0;
            c.cycles += 1;
            top = pos - offset;
        }
    }
    c.f[12] = top.0;
    c.cycles += 2;
    c.f[0] = top.0;
    ret(c);
}

/// `int rider->GetAIIndex()` through the rider's vtable, returning to `ret`.
pub(crate) fn rider_ai_index(c: &mut Cpu, rider: u32, ret: u32) -> crate::call::Flow {
    use rider_lights::*;
    c.g[V1][0] = c.lw(rider.wrapping_add(VTABLE));
    c.g[A1][0] = sx(rider.wrapping_add(VTABLE));
    c.g[A0][0] = c.lh(bw::p(r32(c, V1), AI_INDEX_SLOT));
    c.g[V0][0] = c.lw(bw::p(r32(c, V1), AI_INDEX_SLOT + 4));
    let method = r32(c, V0);
    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
    call_ptr(c, method, ret)
}

/// `void Rider_BuildLightSet(cRider* r)` (0x1220d8): rebuild the rider's
/// irradiance at `+0x7c0`: clear it, copy the view's ambient set
/// (`WorldPainter_GetViewIrradiance(index)`, 0xa0 bytes), add the
/// camera-relative light at the hips (`WorldPainter_GetLighting30`), each of
/// the eight dynamic lights, and, if the rider handler has them, its
/// directional block: the offset vector, then one directional light per
/// entry.
pub fn rider_build_light_set(c: &mut Cpu) {
    use rider_lights::*;
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S2, 0x60);
        frame.save(c, S0, 0x80);
        c.g[S2][0] = c.g[A0][0];
        let rider = r32(c, S2);
        c.g[S0][0] = sx(rider.wrapping_add(LIGHTS));
        frame.save(c, S1, 0x70);
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save_ra(c, 0x20);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0038_9260, 0x0012_2108)?; // Irradiance_Clear
        rider_ai_index(c, rider, 0x0012_2120)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x002e_dfb8, 0x0012_2128)?; // WorldPainter_GetViewIrradiance
                                                // Copy the view's irradiance (0xa0 bytes, two quadwords a step).
        c.g[V1][0] = sx(r32(c, V0).wrapping_add(0xa0));
        loop {
            c.cycles += 7;
            c.lq(A0, r32(c, V0));
            c.lq(A1, bw::p(r32(c, V0), 0x10));
            c.sq(A0, r32(c, S0));
            c.sq(A1, bw::p(r32(c, S0), 0x10));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x20));
            let more = c.g[V0][0] != c.g[V1][0];
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0012_212c);
            }
        }
        // The camera-relative light at the hips.
        c.cycles += 20;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = c.g[S2][0];
        Callee {
            addr: 0x0011_ff48,
            f: ee::s00::Rider_GetHipsPosition,
        }
        .call(c, 0x0012_2154)?;
        c.g[S0][0] = sx(rider.wrapping_add(LIGHTS));
        c.g[S4][0] = c.g[S0][0];
        c.g[S1][0] = sx(rider.wrapping_add(DYNAMIC_LIGHTS));
        c.g[S3][0] = 7;
        rider_ai_index(c, rider, 0x0012_217c)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x002e_efa8, 0x0012_2184)?; // WorldPainter_GetLighting30
        c.mov_s(12, 0);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x0038_9cb8, 0x0012_2194)?; // Irradiance_AddCameraRelativeLight
                                                // The dynamic lights.
        loop {
            c.cycles += 3;
            c.g[A2][0] = c.lw(r32(c, S1));
            let none = c.g[A2][0] == 0;
            c.g[A0][0] = c.g[S4][0];
            if !none {
                c.cycles += 2;
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, 0x0038_a6a8, 0x0012_21ac)?; // Irradiance_AddDynamicLight
            }
            c.cycles += 3;
            c.g[S3][0] = sx(r32(c, S3).wrapping_sub(1));
            let more = (c.g[S3][0] as i64) >= 0;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0012_2198);
            }
        }
        // The handler's directional block.
        c.cycles += 6;
        c.g[V1][0] = c.lw(rider.wrapping_add(HANDLER));
        c.g[S1][0] = 0;
        c.g[V0][0] = sx(r32(c, V1).wrapping_add(HANDLER_LIGHTS));
        if c.g[V1][0] != 0 {
            c.g[S1][0] = c.g[V0][0];
        }
        let block = r32(c, S1);
        c.g[A1][0] = sx(bw::p(sp(c), 0x10));
        if block != 0 {
            c.cycles += 13;
            c.lwc1(0, block);
            c.lwc1(1, bw::p(block, 0x4));
            c.g[S5][0] = c.g[A1][0];
            c.lwc1(2, bw::p(block, 0x8));
            c.g[A0][0] = c.g[S4][0];
            c.swc1(0, bw::p(sp(c), 0x10));
            c.g[S3][0] = 0;
            c.swc1(1, bw::p(sp(c), 0x14));
            c.swc1(2, bw::p(sp(c), 0x18));
            call_ptr(c, 0x0038_9558, 0x0012_21f8)?; // Vec3_AddTo
            c.g[V0][0] = c.lw(bw::p(block, 0xc));
            c.g[S0][0] = sx(bw::p(block, 0x18));
            if c.g[V0][0] as i64 > 0 {
                c.cycles += 3;
                c.g[S2][0] = sx(bw::p(block, 0x50));
                c.lwc1(0, r32(c, S0).wrapping_sub(0x8));
                loop {
                    c.cycles += 16;
                    c.g[A1][0] = c.g[S2][0];
                    c.lwc1(1, r32(c, S0).wrapping_sub(0x4));
                    c.g[A0][0] = c.g[S4][0];
                    c.lwc1(2, r32(c, S0));
                    c.g[A2][0] = c.g[S5][0];
                    c.swc1(0, bw::p(sp(c), 0x10));
                    c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
                    c.swc1(1, bw::p(sp(c), 0x14));
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(0xc));
                    c.swc1(2, bw::p(sp(c), 0x18));
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(0x10));
                    call_ptr(c, 0x0038_9520, 0x0012_2240)?; // Irradiance_AddDirectional
                    c.g[V0][0] = c.lw(bw::p(block, 0xc));
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.lwc1(0, r32(c, S0).wrapping_sub(0x8));
                    if event_due(c) {
                        deliver_event_here(c, 0x0012_2210);
                    }
                }
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// DIntr (0x42c078).

/// COP0 Status bit EIE (interrupts enabled).
pub(crate) const STATUS_EIE: u64 = 0x1_0000;

/// `int DIntr(void)` (0x42c078): disable interrupts; returns 1 if they were
/// enabled. The `di` is repeated until Status.EIE reads back clear.
pub fn d_intr(c: &mut Cpu) {
    c.cycles += 5;
    c.mfc0(V1, 12);
    c.g[V0][0] = STATUS_EIE;
    c.g[V1][0] &= c.g[V0][0];
    let was_on = c.g[V1][0] != 0;
    c.g[A0][0] = was_on as u64;
    if !was_on {
        c.cycles += 3;
        c.g[A0][0] = 0;
        c.g[V0][0] = 0;
        ret(c);
        return;
    }
    c.cycles += 1;
    loop {
        c.cycles += 7;
        c.di();
        c.mfc0(V0, 12);
        c.g[V1][0] = STATUS_EIE;
        c.g[V0][0] &= c.g[V1][0];
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0042_c090);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

/// `void LensFX_DrawSun(LensFX* fx, Viewport* vp, int second_view)`
/// (0x2f4a08): while the sun is visible, reads the view's sun colour,
/// intensity, size (1 means 320) and texture from the world painter
/// (view 6 or 7); if the alpha `intensity * fx->intensity` is positive,
/// pushes an additive 2D draw state on the renderer (`gp+0x2a90`), sets the
/// viewport, binds the texture, draws the sun sprite at the sun's screen
/// position, draws the flare set (`LensFX_DrawFlareSet`), and pops the
/// state.
pub fn lens_fx_draw_sun(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x100);
        frame.save(c, S4, 0x90);
        frame.save(c, S5, 0x80);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S0, 0xd0);
        frame.save(c, S1, 0xc0);
        frame.save(c, S2, 0xb0);
        frame.save(c, S3, 0xa0);
        frame.save(c, S6, 0x70);
        frame.save(c, S7, 0x60);
        frame.save_ra(c, 0x50);
        frame.save_fpr(c, 22, 0xf0);
        frame.save_fpr(c, 21, 0xe8);
        frame.save_fpr(c, 20, 0xe0);
        let fx = r32(c, S4);
        c.g[V0][0] = c.lhu(fx.wrapping_add(lens_fx::VISIBLE));
        c.g[S5][0] = c.g[A1][0];
        if c.g[V0][0] != 0 {
            let drawn = draw_sun_body(c, fx)?;
            if drawn {
                c.cycles += 1;
                frame.restore(c, S0, 0xd0);
            }
        } else {
            c.cycles += 1;
            frame.restore(c, S0, 0xd0);
        }
        c.cycles += 13;
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore(c, S5, 0x80);
        frame.restore(c, S6, 0x70);
        frame.restore(c, S7, 0x60);
        frame.restore_ra(c, 0x50);
        frame.restore_fpr(c, 22, 0xf0);
        frame.restore_fpr(c, 21, 0xe8);
        frame.restore_fpr(c, 20, 0xe0);
        frame.close(c);
        Ok(())
    })
}

/// The visible body: false when the alpha was not positive (s0 already
/// restored), true after drawing.
pub(crate) fn draw_sun_body(c: &mut Cpu, fx: u32) -> Result<bool, crate::call::Unwound> {
    // Colour (1, r, g, b) at sp+0x10, copied to sp+0.
    c.cycles += 42;
    c.g[V0][0] = 6;
    c.g[S0][0] = 7;
    if c.g[A2][0] == 0 {
        c.g[S0][0] = c.g[V0][0];
    }
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 22);
    c.g[A0][0] = c.g[S0][0];
    sun_callee::RED.call(c, 0x002f_4a68)?;
    c.mov_s(21, 0);
    c.g[A0][0] = c.g[S0][0];
    sun_callee::GREEN.call(c, 0x002f_4a74)?;
    c.mov_s(20, 0);
    c.g[A0][0] = c.g[S0][0];
    sun_callee::BLUE.call(c, 0x002f_4a80)?;
    let sp = sp(c);
    c.swc1(20, sp.wrapping_add(0x18));
    c.g[S7][0] = sx(sp.wrapping_add(0x10));
    c.swc1(21, sp.wrapping_add(0x14));
    c.swc1(0, sp.wrapping_add(0x1c));
    c.swc1(22, sp.wrapping_add(0x10));
    oc224_ld_u(c, V0, sp.wrapping_add(0x10));
    oc224_ld_u(c, V1, sp.wrapping_add(0x18));
    oc224_sd_u(c, V0, sp);
    oc224_sd_u(c, V1, sp.wrapping_add(8));
    // alpha *= fx->intensity * sun intensity
    c.g[A0][0] = c.g[S0][0];
    sun_callee::INTENSITY.call(c, 0x002f_4abc)?;
    c.lwc1(2, fx.wrapping_add(lens_fx::INTENSITY));
    c.g[A0][0] = c.g[S0][0];
    c.lwc1(1, sp);
    c.mul_s(2, 2, 0);
    c.mul_s(1, 1, 2);
    c.swc1(1, sp);
    sun_callee::SIZE.call(c, 0x002f_4ad8)?;
    c.mov_s(20, 0);
    c.g[A0][0] = c.g[S0][0];
    sun_callee::TEXTURE.call(c, 0x002f_4ae4)?;
    c.c_eq_s(20, 22);
    let unit = c.fcc();
    c.g[S6][0] = c.g[V0][0];
    if unit {
        c.cycles += 2;
        c.g[AT][0] = 0x43a0_0000;
        c.mtc1(AT, 20);
    }
    c.cycles += 6;
    c.lwc1(0, sp);
    c.mtc1(0, 21);
    c.c_le_s(0, 21);
    let invisible = c.fcc();
    c.lq(S0, sp.wrapping_add(0xd0));
    if invisible {
        return Ok(false);
    }

    // Push the draw state (a copy one up), then the default 2D record.
    c.cycles += 155;
    c.g[S0][0] = c.lw(oc224_gp(c, 0x2a90));
    c.g[A0][0] = 0x0050_0000;
    c.g[V0][0] = c.lw(oc224_at(c, S0, renderer::STATE));
    let st = r32(c, V0);
    oc224_ld_u(c, A3, st);
    oc224_ld_u(c, V1, st.wrapping_add(8));
    c.g[A1][0] = c.lw(st.wrapping_add(0x10));
    oc224_sd_u(c, A3, st.wrapping_add(0x14));
    oc224_sd_u(c, V1, st.wrapping_add(0x1c));
    let v = c.g[A1][0];
    c.sw(st.wrapping_add(0x24), v);
    c.g[V1][0] = c.lw(oc224_at(c, S0, renderer::STATE));
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x14));
    let v = c.g[V0][0];
    c.sw(oc224_at(c, S0, renderer::STATE), v);
    c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1420));
    let def = r32(c, A2);
    oc224_ld_u(c, A3, def);
    oc224_ld_u(c, V0, def.wrapping_add(8));
    c.g[A1][0] = c.lw(def.wrapping_add(0x10));
    let st = r32(c, V1);
    oc224_sd_u(c, A3, st.wrapping_add(0x14));
    oc224_sd_u(c, V0, st.wrapping_add(0x1c));
    let v = c.g[A1][0];
    c.sw(st.wrapping_add(0x24), v);
    // BeginScene (vt[0xa8]).
    c.g[V0][0] = c.lw(oc224_at(c, S0, renderer::VTABLE));
    c.g[A0][0] = c.lh(oc224_at(c, V0, 0xa8));
    c.g[V1][0] = c.lw(oc224_at(c, V0, 0xac));
    let t = r32(c, V1);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x002f_4b94)?;
    // SetViewport(vp, 1, 0, 0, 1) (vt[0xd0]).
    let vp = r32(c, S5);
    c.g[V0][0] = c.lw(oc224_at(c, S0, renderer::VTABLE));
    c.mov_s(16, 21);
    c.lwc1(12, vp);
    c.mov_s(17, 22);
    c.g[A0][0] = c.lh(oc224_at(c, V0, 0xd0));
    c.g[A2][0] = 0;
    c.lwc1(13, vp.wrapping_add(4));
    c.g[A1][0] = 1;
    c.lwc1(14, vp.wrapping_add(8));
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = c.lw(oc224_at(c, V0, 0xd4));
    let t = r32(c, V1);
    c.lwc1(15, vp.wrapping_add(0xc));
    call_ptr(c, t, 0x002f_4bc8)?;
    // Begin2D (vt[0xf8]), SetTexture(0x4ff1a0) (vt[0x110]).
    c.g[V0][0] = c.lw(oc224_at(c, S0, renderer::VTABLE));
    c.g[A0][0] = c.lh(oc224_at(c, V0, 0xf8));
    c.g[V1][0] = c.lw(oc224_at(c, V0, 0xfc));
    let t = r32(c, V1);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x002f_4bdc)?;
    c.g[A2][0] = c.lw(oc224_at(c, S0, renderer::VTABLE));
    c.g[A1][0] = 0x0050_0000;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xe60));
    c.g[A0][0] = c.lh(oc224_at(c, A2, 0x110));
    c.g[V0][0] = c.lw(oc224_at(c, A2, 0x114));
    let t = r32(c, V0);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x002f_4bf8)?;

    // The state's blend/test bits.
    c.g[A2][0] = c.lw(oc224_at(c, S0, renderer::STATE));
    c.g[V1][0] = 0xffff_ffff_ffbf_0000;
    c.g[V1][0] |= 0xffff;
    c.g[A1][0] = 0x0040_0000;
    c.g[V0][0] = c.lw(oc224_at(c, A2, 4));
    c.g[A0][0] = 0xffff_ffff_fe7f_0000;
    c.g[A0][0] |= 0xffff;
    c.g[T7][0] = 0x0100_0000;
    c.g[V0][0] &= c.g[V1][0];
    c.g[T3][0] = 0xffff_ffff_ffcf_0000;
    c.g[V0][0] |= c.g[A1][0];
    c.g[T3][0] |= 0xffff;
    let v = c.g[V0][0];
    c.sw(oc224_at(c, A2, 4), v);
    c.g[T4][0] = 0xffff_ffff_fff0_0000;
    c.g[T4][0] |= 0xfff;
    c.g[T5][0] = 0x0001_0000;
    c.g[V1][0] = c.lw(oc224_at(c, S0, renderer::STATE));
    c.g[T5][0] |= 0x4000;
    c.g[S1][0] = 0xffff_ffff_ffff_fffc;
    c.g[T6][0] = 0xffff_ffff_e000_0000;
    c.g[V0][0] = c.lw(oc224_at(c, V1, 4));
    c.g[T6][0] |= 0x3ff;
    c.g[S2][0] = 0xffff_ffff_ffff_ff83;
    c.g[S3][0] = 0xffff_ffff_ffff_fc1f;
    c.g[V0][0] &= c.g[A0][0];
    c.g[T2][0] = sx((c.g[S6][0] as u32).wrapping_add(0x33));
    c.g[V0][0] |= c.g[T7][0];
    c.g[T2][0] = sx((c.g[T2][0] as u32) << 2);
    let v = c.g[V0][0];
    c.sw(oc224_at(c, V1, 4), v);
    c.g[T2][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[T2][0] as u32));
    c.g[A3][0] = sx(sp.wrapping_add(0x30));
    c.g[T0][0] = sx(sp.wrapping_add(0x40));
    c.g[V1][0] = c.lw(oc224_at(c, S0, renderer::STATE));
    c.g[T1][0] = c.g[SP][0];
    c.g[A2][0] = sx(sp.wrapping_add(0x20));
    c.g[A1][0] = c.g[S7][0];
    c.g[V0][0] = c.lw(oc224_at(c, V1, 4));
    c.g[V0][0] &= c.g[T3][0];
    let v = c.g[V0][0];
    c.sw(oc224_at(c, V1, 4), v);
    c.g[A0][0] = c.lw(oc224_at(c, S0, renderer::STATE));
    c.g[V0][0] = c.lw(oc224_at(c, A0, 4));
    c.g[V0][0] &= c.g[T4][0];
    c.g[V0][0] |= c.g[T5][0];
    let v = c.g[V0][0];
    c.sw(oc224_at(c, A0, 4), v);
    c.g[V1][0] = c.lw(oc224_at(c, S0, renderer::STATE));
    c.g[V0][0] = c.lw(oc224_at(c, V1, 4));
    c.g[V0][0] &= c.g[S1][0];
    let v = c.g[V0][0];
    c.sw(oc224_at(c, V1, 4), v);
    c.g[A0][0] = c.lw(oc224_at(c, S0, renderer::STATE));
    c.g[V0][0] = c.lw(oc224_at(c, A0, 8));
    c.g[V0][0] &= c.g[T6][0];
    let v = c.g[V0][0];
    c.sw(oc224_at(c, A0, 8), v);
    c.g[V1][0] = c.lw(oc224_at(c, S0, renderer::STATE));
    c.g[V0][0] = c.lw(oc224_at(c, V1, 4));
    c.g[V0][0] &= c.g[S2][0];
    c.g[V0][0] |= 0x1c;
    let v = c.g[V0][0];
    c.sw(oc224_at(c, V1, 4), v);
    c.g[A0][0] = c.lw(oc224_at(c, S0, renderer::STATE));
    c.g[V0][0] = c.lw(oc224_at(c, A0, 8));
    c.g[V0][0] &= c.g[S3][0];
    c.g[V0][0] |= 0x100;
    let v = c.g[V0][0];
    c.sw(oc224_at(c, A0, 8), v);
    // The texture id (renderer table at +0x101c) into the state.
    c.g[V1][0] = c.lw(oc224_at(c, T2, 0xf50));
    c.g[V0][0] = c.lw(oc224_at(c, S0, renderer::STATE));
    let v = c.g[V1][0];
    c.sh(oc224_at(c, V0, 0x10), v);
    // DrawSprite((x, y, 0, 1), (size, size), (0, 0), (1, 1), colour).
    c.swc1(21, sp.wrapping_add(0x18));
    c.lwc1(0, fx.wrapping_add(lens_fx::SUN_X));
    c.lwc1(1, fx.wrapping_add(lens_fx::SUN_Y));
    c.swc1(0, sp.wrapping_add(0x10));
    c.swc1(1, sp.wrapping_add(0x14));
    c.swc1(22, sp.wrapping_add(0x1c));
    c.g[V1][0] = c.lw(oc224_at(c, S0, renderer::VTABLE));
    c.g[A0][0] = c.lh(oc224_at(c, V1, 0x270));
    c.swc1(20, sp.wrapping_add(0x20));
    c.swc1(20, sp.wrapping_add(0x24));
    c.swc1(21, sp.wrapping_add(0x34));
    c.swc1(22, sp.wrapping_add(0x44));
    c.swc1(21, sp.wrapping_add(0x30));
    c.swc1(22, sp.wrapping_add(0x40));
    c.g[V0][0] = c.lw(oc224_at(c, V1, 0x274));
    let t = r32(c, V0);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x002f_4d40)?;
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = c.g[S5][0];
    sun_callee::DRAW_FLARE_SET.call(c, 0x002f_4d4c)?;
    // Pop the state; End2D (vt[0x100]), EndScene (vt[0xb0]).
    c.g[V0][0] = c.lw(oc224_at(c, S0, renderer::STATE));
    c.g[A1][0] = c.lw(oc224_at(c, S0, renderer::VTABLE));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
    let v = c.g[V0][0];
    c.sw(oc224_at(c, S0, renderer::STATE), v);
    c.g[A0][0] = c.lh(oc224_at(c, A1, 0x100));
    c.g[V1][0] = c.lw(oc224_at(c, A1, 0x104));
    let t = r32(c, V1);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x002f_4d6c)?;
    c.g[V0][0] = c.lw(oc224_at(c, S0, renderer::VTABLE));
    c.g[A0][0] = c.lh(oc224_at(c, V0, 0xb0));
    c.g[V1][0] = c.lw(oc224_at(c, V0, 0xb4));
    let t = r32(c, V1);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x002f_4d80)?;
    Ok(true)
}

// ---------------------------------------------------------------------------
// Rider_SetSlew0Target (0x113e80)

/// Rider: +0x1e0 velocity, slew 0 (lean) value +0x1f0, rate +0x1f4,
/// target +0x1f8, +0x438 the main state.
pub(crate) const RIDER_VELOCITY: u32 = 0x1e0;

pub(crate) const RIDER_SLEW0: u32 = 0x1f0;

pub(crate) const RIDER_SLEW0_RATE: u32 = 0x1f4;

pub(crate) const RIDER_SLEW0_TARGET: u32 = 0x1f8;

pub(crate) const RIDER_MAIN_STATE: u32 = 0x438;

/// gp-relative constants: full-lean speed (694.54), rate per unit of
/// distance (12.0), minimum and maximum rate (0.1, 14.0), the rate factor
/// in states 2, 3 and 13 (0.756), seconds per tick (1/60).
pub(crate) const GP_SLEW0_FULL_SPEED: u32 = 0x7c40;

pub(crate) const GP_SLEW0_RATE_SCALE: u32 = 0x7c3c;

pub(crate) const GP_SLEW0_RATE_MIN: u32 = 0x7c38;

pub(crate) const GP_SLEW0_RATE_MAX: u32 = 0x7c34;

pub(crate) const GP_SLEW0_STATE_FACTOR: u32 = 0x7c30;

pub(crate) const GP_SLEW0_DT: u32 = 0x7c2c;

/// `void Rider_SetSlew0Target(Rider* rider, float target)` (0x113e80): set
/// the lean slew target, scaled down below full-lean speed, and its rate:
/// the distance to the current value times 12, clamped to [0.1, 14] (the
/// lower bound wins when above it; reduced in states 2, 3 and 13), per tick.
pub fn rider_set_slew0_target(c: &mut Cpu) {
    let rider = Addr(abi::arg(c, 0));
    let gp = c.g[GP][0] as u32;
    let lit = |o: u32| Addr(gp.wrapping_sub(o));
    c.cycles += 18;
    let full = lit(GP_SLEW0_FULL_SPEED).read_fpu(c);
    abi::set_fpr(c, 1, full);
    // |velocity| (VU0 macro mode, as the original).
    c.lqc2(3, rider.off(RIDER_VELOCITY).0);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(V0, 22);
    let speed = FpuF32(c.g[V0][0] as u32);
    c.g[AT][0] = 0x3f80_0000; // 1.0
    let one = FpuF32(0x3f80_0000);
    abi::set_fpr(c, 2, one);
    let ratio = speed.div(full);
    abi::set_fpr(c, 0, ratio);
    let slow = ratio.lt(one);
    abi::set_fcc(c, slow);
    let mut target = abi::farg(c, 0);
    if slow {
        target = target * ratio;
        abi::set_fpr(c, 12, target);
    }
    c.cycles += 11;
    let cur = rider.off(RIDER_SLEW0).read_fpu(c);
    let scale = lit(GP_SLEW0_RATE_SCALE).read_fpu(c);
    let lo = lit(GP_SLEW0_RATE_MIN).read_fpu(c);
    let hi = lit(GP_SLEW0_RATE_MAX).read_fpu(c);
    abi::set_fpr(c, 3, hi);
    abi::set_fpr(c, 1, lo);
    let d = (target - cur).abs() * scale;
    abi::set_fpr(c, 0, d);
    let above = lo.le(d);
    abi::set_fcc(c, above);
    let mut rate = lo;
    if above {
        c.cycles += 1;
        rate = d.min(hi);
    }
    abi::set_fpr(c, 2, rate);
    c.cycles += 5;
    c.g[V1][0] = rider.off(RIDER_MAIN_STATE).lw(c);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(2));
    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
    let mut slowed = c.g[V0][0] != 0;
    if !slowed {
        c.cycles += 3;
        c.g[V0][0] = 0xd;
        slowed = c.g[V1][0] == c.g[V0][0];
    }
    if slowed {
        c.cycles += 2;
        let k = lit(GP_SLEW0_STATE_FACTOR).read_fpu(c);
        abi::set_fpr(c, 0, k);
        rate = rate * k;
    }
    abi::set_fpr(c, 1, rate);
    c.cycles += 5;
    let dt = lit(GP_SLEW0_DT).read_fpu(c);
    rider.off(RIDER_SLEW0_TARGET).write_fpu(c, target);
    let per_tick = rate * dt;
    abi::set_fpr(c, 0, per_tick);
    rider.off(RIDER_SLEW0_RATE).write_fpu(c, per_tick);
    abi::ret(c);
}

pub(crate) const PATH_SERVICE_QUEUES: Callee = Callee {
    addr: 0x003d_1e80,
    f: ee::s14::PATH_ServiceQueues,
};

pub(crate) const PATH_VOLUME_FADE: Callee = Callee {
    addr: 0x003d_5a98,
    f: ee::s14::PATH_TrackUpdateVolumeFade,
};

pub(crate) const PATH_AUX_FADE: Callee = Callee {
    addr: 0x003d_5cc0,
    f: ee::s14::PATH_TrackUpdateAuxSendFade,
};

pub(crate) const PATH_FADE3: Callee = Callee {
    addr: 0x003d_5ee8,
    f: ee::s14::PATH_TrackUpdateFade3,
};

pub(crate) const PATH_QUEUED_COUNT: Callee = Callee {
    addr: 0x003d_4950,
    f: ee::s14::PATH_TrackGetQueuedCount,
};

pub(crate) const PATH_TRACK_ADVANCE: Callee = Callee {
    addr: 0x003d_3d20,
    f: ee::s14::PATH_TrackAdvance,
};

/// `void PATH_UpdateTracks(void)` (0x3d4f10): the per-tick service of the
/// PATH interactive-music player. Unless in streamed (`'B'`) mode, service
/// the event queues first. For each of the 24 track slots that is in use
/// and idle: run its volume / aux-send / third fades while it is active;
/// then, for the active tracks of the current mode, stop a track whose stop
/// time has come, or (if its platform stream is playing) advance it to the
/// next node once its queue runs low. Finally (normal mode) service the
/// queues until they are empty.
pub fn path_update_tracks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[V1][0] = 0x45_0000;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = Addr(path::MODE).lbu(c);
        frame.save(c, S4, 0x50);
        c.g[V0][0] ^= 0x42;
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        c.g[S4][0] = (c.g[V0][0] == 0) as u64;
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);
        // s6 = tick length, s7 = half of it, fp = 0x450000 (globals base).
        let streamed = c.g[S4][0] != 0;
        c.cycles += if streamed { 5 } else { 4 };
        c.g[V0][0] = 0x45_0000;
        c.g[V1][0] = Addr(path::STATE).lw(c);
        c.g[FP][0] = c.g[V0][0];
        let tick_off = if streamed { 0x10 } else { 0xc };
        c.g[S6][0] = Addr(r32(c, V1).wrapping_add(tick_off)).lw(c);

        c.cycles += 2;
        c.g[S3][0] = 0;
        if !streamed {
            c.cycles += 2;
            PATH_SERVICE_QUEUES.call(c, 0x003d_4f84)?;
        }
        c.cycles += 3;
        c.g[S7][0] = sx(r32(c, S6) >> 1);
        c.g[V0][0] = Addr(path::STATE).lw(c);
        loop {
            c.cycles += 5;
            c.g[V1][0] = sx(r32(c, S3) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            c.g[S0][0] = Addr(r32(c, V0).wrapping_add(path::SLOT0)).lw(c);
            if c.g[S0][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, S0).wrapping_add(track::BUSY)).lbu(c);
                if c.g[V0][0] == 0 {
                    path_update_track(c)?;
                }
            }
            c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
            c.cycles += 3;
            let more = c.g[S3][0] < path::TRACKS as u64;
            c.g[V0][0] = Addr(path::STATE).lw(c);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003d_4f90);
            }
        }

        // Drain the queues (s0 is already restored for the callee's frame).
        c.cycles += 1;
        loop {
            c.cycles += 2;
            frame.restore(c, S0, 0x90);
            if c.g[S4][0] != 0 {
                break;
            }
            c.cycles += 4;
            PATH_SERVICE_QUEUES.call(c, 0x003d_50f0)?;
            frame.restore(c, S0, 0x90);
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003d_50e0);
            }
        }
        c.cycles += 11;
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// One idle track (s0, slot s3) of `PATH_UpdateTracks`.
pub(crate) fn path_update_track(c: &mut Cpu) -> Flow {
    use track::*;
    let t = r32(c, S0);
    c.cycles += 8;
    c.g[V0][0] = 0x45_0000;
    c.g[A1][0] = Addr(t + LEAD).lw(c);
    c.g[V1][0] = Addr(path::TIME).lw(c);
    c.g[S5][0] = c.g[V0][0];
    c.g[A0][0] = Addr(t + NODE).lh(c);
    c.g[S2][0] = 0;
    // s1 = time + lead: the end of what is queued.
    c.g[S1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
    if (c.g[A0][0] as i64) >= 0 {
        c.cycles += 2;
        c.g[V0][0] = Addr(t + ACTIVE).lw(c);
        c.g[S2][0] = (c.g[V0][0] != 0) as u64;
    }
    c.cycles += 2;
    if c.g[S2][0] != 0 {
        c.cycles += 3;
        let fades = [
            (VOLUME_FADE, PATH_VOLUME_FADE, 0x003d_4ff4),
            (AUX_FADE, PATH_AUX_FADE, 0x003d_5008),
            (FADE3, PATH_FADE3, 0x003d_501c),
        ];
        for (i, (field, fade, ret)) in fades.into_iter().enumerate() {
            if i > 0 {
                c.cycles += 2;
            }
            c.g[V0][0] = Addr(t + field).lb(c);
            if (c.g[V0][0] as i64) >= 0 {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                fade.call(c, ret)?;
            }
        }
    }
    c.g[V0][0] = Addr(t + STREAMED).lbu(c);
    c.cycles += 3;
    c.g[V0][0] = (c.g[V0][0] != 0) as u64;
    if c.g[V0][0] != c.g[S4][0] {
        return Ok(());
    }
    c.cycles += 2;
    if c.g[S2][0] == 0 {
        return Ok(());
    }

    c.cycles += 3;
    c.g[V1][0] = Addr(t + STOP_AT).lw(c);
    c.g[V0][0] = Addr(path::TIME).lw(c);
    if c.g[V1][0] != 0 {
        // Stop pending: stop the stream once time + tick/2 reaches it.
        c.cycles += 4;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S7)));
        c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
        if c.g[V0][0] != 0 {
            return Ok(());
        }
        c.cycles += 5;
        c.g[A0][0] = Addr(t + HANDLE).lw(c);
        c.g[A1][0] = 0;
        call_ptr(c, 0x002a_e9f8, 0x003d_505c)?;
        Addr(t + STOP_AT).sw(c, 0);
        c.cycles += 1;
        return Ok(());
    }
    // Stream playing?
    c.cycles += 4;
    c.g[A0][0] = Addr(t + HANDLE).lw(c);
    call_ptr(c, 0x002a_ebd0, 0x003d_506c)?;
    if c.g[V0][0] == 0 {
        return Ok(());
    }
    c.cycles += 6;
    c.g[A0][0] = c.g[S0][0];
    PATH_QUEUED_COUNT.call(c, 0x003d_507c)?;
    c.g[V1][0] = c.g[V0][0];
    c.g[V0][0] = Addr(t + DEADLINE).lw(c);
    c.g[S1][0] = sx(r32(c, S1).wrapping_add(r32(c, S6)));
    c.cycles += 2;
    // Not yet: the queue reaches past the next tick (deadline), or holds
    // more than the lead.
    if c.g[V0][0] != 0 {
        c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
    } else {
        c.g[V0][0] = Addr(t + LEAD).lw(c);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    }
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        return Ok(());
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(t + PLAY_STATE).lb(c);
    if c.g[V0][0] == 0 {
        c.cycles += 4;
        c.g[A0][0] = Addr(t + HANDLE).lw(c);
        call_ptr(c, 0x002a_f428, 0x003d_50b8)?;
        let started = c.g[V0][0] != 0;
        c.g[V0][0] = u64::MAX;
        if !started {
            c.cycles += 1;
            Addr(t + PLAY_STATE).sb(c, u64::MAX);
        }
    }
    c.cycles += 2;
    c.g[A0][0] = c.g[S3][0];
    PATH_TRACK_ADVANCE.call(c, 0x003d_50cc)?;
    c.cycles += 1;
    Ok(())
}

/// `void HUD_DrawSpeedometer(Hud* hud, Elem* elems, Rider* rider, int
/// arg)` (0x2200c0): the speed readout. Copies the HUD settings to the
/// stack, takes the length of the rider's velocity (VU0 `sqrt`) times the
/// speed scale (and the units factor when settings bit 19 is clear),
/// draws the unit label at the speedometer element (measured, aligned with
/// `HUD_AlignElemPos`), then the number (`sprintf("%d", speed + 0.5)`)
/// centred above it, scaled down for three or more digits.
///
/// The settings block is at a constant 8-aligned address, so only the
/// compiled code's aligned copy loop is reachable (its `ldl/ldr` twin is
/// dead code).
pub fn hud_draw_speedometer(c: &mut Cpu) {
    use speedo::*;
    guest(c, |c| {
        let p = |a: u32, o: u32| a.wrapping_add(o);
        c.cycles += 25;
        let frame = Frame::open(c, 0x3d0);
        frame.save(c, S1, 0x3a0);
        frame.save(c, S2, 0x390);
        let rider = r32(c, A2);
        set_reg(c, S1, reg(c, A2));
        frame.save(c, S4, 0x370);
        let hud = r32(c, A0);
        set_reg(c, S2, reg(c, A0));
        let sp0 = sp(c);
        Addr(sp0 + 0x310).sw(c, reg(c, A3));
        let elems = r32(c, A1);
        set_reg(c, S4, reg(c, A1));
        frame.save(c, S0, 0x3b0);
        frame.save(c, S3, 0x380);
        frame.save(c, S5, 0x360);
        frame.save(c, S6, 0x350);
        frame.save(c, S7, 0x340);
        frame.save(c, FP, 0x330);
        frame.save_ra(c, 0x320);
        frame.save_fpr(c, 20, 0x3c0);
        call_ptr(c, BE_LIBRARY_GET_INSTANCE, 0x0022_0104)?;
        set_reg(c, A0, reg(c, V0));
        set_reg(c, A1, 4);
        call_ptr(c, BE_LIBRARY_GET_INTERFACE, 0x0022_0110)?;

        // Copy the settings to sp+0 (32 bytes per step).
        set_reg(c, V1, 0x53_0000);
        set_reg(c, A3, sx(SETTINGS));
        set_reg(c, V0, 0); // SETTINGS & 7
        set_reg(c, A0, sx(sp0));
        c.cycles += 13;
        let (label_pos, label_size) = (sp0 + 0x2d0, sp0 + 0x2e0);
        set_reg(c, T2, sx(label_pos));
        set_reg(c, T3, sx(label_size));
        Addr(sp0 + 0x314).sw(c, sx(label_pos));
        let end = SETTINGS + SETTINGS_SIZE;
        set_reg(c, V0, sx(end));
        Addr(sp0 + 0x318).sw(c, sx(label_size));
        set_reg(c, A1, 0x4a_0000);
        set_reg(c, T0, 0x4a_0000);
        let velocity = p(rider, RIDER_VELOCITY);
        set_reg(c, A2, sx(velocity));
        let elem = p(elems, ELEM);
        set_reg(c, S0, sx(elem));
        let (pos, scale, color, number) = (sp0 + 0x290, sp0 + 0x2a0, sp0 + 0x2b0, sp0 + 0x2f0);
        set_reg(c, S5, sx(pos));
        set_reg(c, S6, sx(scale));
        set_reg(c, S7, sx(color));
        set_reg(c, S3, sx(number));
        let mut from = SETTINGS;
        let mut to = sp0;
        loop {
            c.cycles += 11;
            for (k, r) in [V1, T1, T2, T3].into_iter().enumerate() {
                let v = c.ld(from + 8 * k as u32);
                set_reg(c, r, v);
            }
            for (k, r) in [V1, T1, T2, T3].into_iter().enumerate() {
                c.sd(to + 8 * k as u32, reg(c, r));
            }
            from += 0x20;
            to += 0x20;
            set_reg(c, A3, sx(from));
            set_reg(c, A0, sx(to));
            if from == end {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, COPY_HEAD);
            }
        }
        c.cycles += 1;
        let vt = Addr(velocity).lw(c);
        set_reg(c, V1, vt);

        // The tail doubleword; then the velocity vector.
        c.cycles += 26;
        set_reg(c, FP, sx(LABEL_B));
        c.ldl(V0, from + 7);
        c.ldr(V0, from);
        c.sdl(V0, to + 7);
        set_reg(c, S1, sx(LABEL_A));
        c.sdr(V0, to);
        let slot = VSlot::read(c, vt, SLOT_GET_VELOCITY);
        set_reg(c, A0, slot.delta);
        set_reg(c, V0, slot.func);
        set_reg(c, A0, slot.this(sx(velocity)));
        call_ptr(c, slot.func as u32, 0x0022_0238)?;
        // |v| with VU0 (vf6.w = 1), into f0, times the speed scale.
        c.lqc2(3, r32(c, V0));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(V1, 22);
        c.mtc1(V1, 0);
        let gp = r32(c, GP);
        c.lwc1(1, gp.wrapping_sub(GP_NEG_SPEED_SCALE));
        let units = Addr(sp0).lw(c) & UNITS_FLAG;
        set_reg(c, V1, UNITS_FLAG);
        set_reg(c, V0, units);
        c.mul_s(20, 0, 1);
        let mut label = LABEL_A;
        if units == 0 {
            c.cycles += 3;
            c.lwc1(0, gp.wrapping_sub(GP_NEG_UNITS_FACTOR));
            set_reg(c, S1, sx(LABEL_B));
            label = LABEL_B;
            c.mul_s(20, 20, 0);
        }

        // The label: element position, scale and colour; measure; align; draw.
        c.cycles += 71;
        set_reg(c, A0, sx(elem));
        set_reg(c, A1, sx(pos));
        call_ptr(c, HUD_GET_ELEM_POS, 0x0022_0290)?;
        set_reg(c, A1, sx(scale));
        set_reg(c, A0, sx(elem));
        call_ptr(c, HUD_GET_ELEM_SCALE, 0x0022_029c)?;
        set_reg(c, A0, sx(elem));
        set_reg(c, A1, sx(color));
        call_ptr(c, HUD_GET_ELEM_COLOR, 0x0022_02a8)?;
        let text = Addr(p(hud, HUD_TEXT)).lw(c) as u32;
        set_reg(c, V1, sx(text));
        set_reg(c, A2, sx(sp0 + 0x2c0));
        set_reg(c, A1, sx(label));
        set_reg(c, A3, 0);
        c.ldl(T3, color + 7);
        c.ldr(T3, color);
        c.ldl(V0, color + 0xf);
        c.ldr(V0, color + 8);
        c.sdl(T3, p(text, TEXT_COLOR + 7));
        c.sdr(T3, p(text, TEXT_COLOR));
        c.sdl(V0, p(text, TEXT_COLOR + 0xf));
        c.sdr(V0, p(text, TEXT_COLOR + 8));
        let text = Addr(p(hud, HUD_TEXT)).lw(c) as u32;
        set_reg(c, V0, sx(text));
        c.lwc1(1, scale);
        c.lwc1(0, p(text, TEXT_BASE_SCALE));
        c.lwc1(2, p(text, TEXT_BASE_SCALE + 4));
        c.mul_s(0, 0, 1);
        c.swc1(0, p(text, TEXT_SCALE));
        c.lwc1(1, scale + 4);
        c.mul_s(2, 2, 1);
        c.swc1(2, p(text, TEXT_SCALE + 4));
        let text = Addr(p(hud, HUD_TEXT)).lw(c) as u32;
        set_reg(c, A0, sx(text));
        c.lwc1(12, p(text, TEXT_SCALE));
        c.lwc1(13, p(text, TEXT_SCALE + 4));
        call_ptr(c, FONT_MEASURE_TEXT, 0x0022_030c)?;
        c.lwc1(1, sp0 + 0x2c8);
        set_reg(c, T0, sx(number));
        c.lwc1(2, sp0 + 0x2cc);
        set_reg(c, A2, sx(pos));
        set_reg(c, AT, 0x3f80_0000);
        c.mtc1(AT, 0);
        set_reg(c, A0, sx(hud));
        let ay = Addr(p(elems, ELEM_ALIGN_Y)).lb(c);
        set_reg(c, T2, ay);
        let ax = Addr(p(elems, ELEM_ALIGN_X)).lb(c);
        set_reg(c, T1, ax);
        let v = Addr(sp0 + 0x314).lw(c);
        set_reg(c, A1, v);
        let v = Addr(sp0 + 0x310).lw(c);
        set_reg(c, T3, v);
        let v = Addr(sp0 + 0x318).lw(c);
        set_reg(c, A3, v);
        c.swc1(1, label_size);
        c.swc1(0, number + 4);
        c.swc1(0, number);
        c.swc1(2, label_size + 4);
        call_ptr(c, HUD_ALIGN_ELEM_POS, 0x0022_0350)?;
        c.lwc1(12, label_pos);
        set_reg(c, A1, sx(label));
        c.lwc1(13, label_pos + 4);
        let v = Addr(p(hud, HUD_TEXT)).lw(c);
        set_reg(c, A0, v);
        call_ptr(c, FONT_DRAW_TEXT, 0x0022_0364)?;

        // The number: sprintf("%d", (int)(speed + 0.5)).
        set_reg(c, AT, 0x3f00_0000);
        c.mtc1(AT, 0);
        set_reg(c, T1, 0x4a_0000);
        set_reg(c, A1, sx(NUMBER_FORMAT));
        c.add_s(0, 20, 0);
        c.cvt_w_s(1, 0);
        c.mfc1(A2, 1);
        set_reg(c, A0, sx(number));
        call_ptr(c, SPRINTF, 0x0022_0388)?;
        set_reg(c, A0, sx(number));
        call_ptr(c, STRLEN, 0x0022_0390)?;
        let short = reg(c, V0) < 3;
        set_reg(c, V0, short as u64);
        c.lwc1(0, gp.wrapping_sub(GP_NEG_SCALE_SHORT));
        c.lwc1(1, scale);
        if !short {
            c.cycles += 1;
            c.lwc1(0, gp.wrapping_sub(GP_NEG_SCALE_LONG));
        }
        c.cycles += 62;
        c.lwc1(2, scale + 4);
        c.mul_s(1, 1, 0);
        c.mul_s(2, 2, 0);
        c.swc1(1, scale);
        c.swc1(2, scale + 4);
        let text = Addr(p(hud, HUD_TEXT)).lw(c) as u32;
        set_reg(c, V0, sx(text));
        set_reg(c, A1, sx(number));
        c.lwc1(0, scale);
        c.lwc1(1, p(text, TEXT_BASE_SCALE));
        c.lwc1(2, p(text, TEXT_BASE_SCALE + 4));
        c.mul_s(1, 1, 0);
        set_reg(c, AT, 0x3f00_0000);
        c.mtc1(AT, 4);
        c.lwc1(5, gp.wrapping_sub(GP_NEG_LABEL_OFFSET));
        c.swc1(1, p(text, TEXT_SCALE));
        c.lwc1(0, scale + 4);
        c.mul_s(2, 2, 0);
        c.swc1(2, p(text, TEXT_SCALE + 4));
        // x = label x + width / 2; y = label y - (line * scale.y * sy / 2 + off).
        c.lwc1(0, label_size);
        c.lwc1(12, label_pos);
        c.mul_s(0, 0, 4);
        let text = Addr(p(hud, HUD_TEXT)).lw(c) as u32;
        set_reg(c, V0, sx(text));
        c.lwc1(3, scale + 4);
        c.lwc1(2, label_pos + 4);
        set_reg(c, A0, sx(text));
        c.add_s(12, 12, 0);
        c.swc1(12, label_pos);
        c.lwc1(0, p(text, TEXT_LINE));
        c.cvt_s_w(0, 0);
        c.lwc1(1, p(text, TEXT_BASE_SCALE + 4));
        c.mul_s(0, 0, 1);
        c.mul_s(0, 0, 3);
        c.mul_s(0, 0, 4);
        c.add_s(0, 0, 5);
        c.sub_s(2, 2, 0);
        c.mov_s(13, 2);
        c.swc1(2, label_pos + 4);
        call_ptr(c, FONT_DRAW_TEXT_CENTERED, 0x0022_043c)?;
        // Restore the text scale (base * 1).
        set_reg(c, AT, 0x3f80_0000);
        c.mtc1(AT, 0);
        let text = Addr(p(hud, HUD_TEXT)).lw(c) as u32;
        set_reg(c, V0, sx(text));
        let unit = sp0 + 0x300;
        c.swc1(0, unit + 4);
        c.swc1(0, unit);
        c.lwc1(2, p(text, TEXT_BASE_SCALE));
        c.lwc1(0, p(text, TEXT_BASE_SCALE + 4));
        c.swc1(2, p(text, TEXT_SCALE));
        c.lwc1(1, unit + 4);
        c.mul_s(0, 0, 1);
        frame.restore(c, S0, 0x3b0);
        frame.restore(c, S1, 0x3a0);
        frame.restore(c, S2, 0x390);
        frame.restore(c, S3, 0x380);
        frame.restore(c, S4, 0x370);
        frame.restore(c, S5, 0x360);
        frame.restore(c, S6, 0x350);
        frame.restore(c, S7, 0x340);
        frame.restore(c, FP, 0x330);
        frame.restore_ra(c, 0x320);
        frame.restore_fpr(c, 20, 0x3c0);
        c.swc1(0, p(text, TEXT_SCALE + 4));
        frame.close(c);
        Ok(())
    })
}

/// `u8* StreamFile_GetChunk(StreamFile* handle)` (0x3e12e0): takes the
/// handle's next queued chunk. Null for an invalid handle or an empty
/// queue. Otherwise clears the chunk's tag byte (keeping its size),
/// subtracts the size from the queued count under the manager's mutex and,
/// when data remains, walks the ring (chunks by size, a -1 word wraps to the
/// ring start) to the next chunk carrying this handle's tag; returns the
/// chunk taken.
pub fn stream_file_get_chunk(c: &mut Cpu) {
    use chunk::*;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        let sp0 = sp(c);
        set_reg(c, A1, sx(sp0));
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0x10);
        set_reg(c, A2, sx(sp0) | 4);
        STREAM_FILE_VALIDATE.call(c, 0x003e_1300)?;
        let invalid = reg(c, V0) != 0;
        set_reg(c, V0, 0);
        'out: {
            if invalid {
                break 'out;
            }
            c.cycles += 4;
            let h = Addr(sp0 + 4).lw(c) as u32;
            set_reg(c, V1, sx(h));
            let queued = Addr(h + H_QUEUED).lw(c);
            set_reg(c, V0, queued);
            if queued == 0 {
                c.cycles += 2;
                set_reg(c, V0, 0);
                break 'out;
            }
            let chunk = Addr(h + H_NEXT).lw(c) as u32;
            set_reg(c, S2, sx(chunk));
            c.cycles += 17;
            set_reg(c, V0, SIZE_MASK);
            let ts = Addr(chunk + CH_TAG_SIZE).lw(c);
            set_reg(c, V1, ts);
            let size = ts & SIZE_MASK;
            set_reg(c, S1, size);
            Addr(chunk + CH_TAG_SIZE).sw(c, size);
            let man = Addr(sp0).lw(c);
            set_reg(c, A0, addu(man, MAN_MUTEX as u64));
            EA_MUTEX_LOCK.call(c, 0x003e_1340)?;
            let h = Addr(sp0 + 4).lw(c);
            set_reg(c, V0, h);
            let h = h as u32;
            let left = sx((Addr(h + H_QUEUED).lw(c) as u32).wrapping_sub(size as u32));
            set_reg(c, S0, left);
            Addr(h + H_QUEUED).sw(c, left);
            let man = Addr(sp0).lw(c);
            set_reg(c, A0, addu(man, MAN_MUTEX as u64));
            EA_MUTEX_UNLOCK.call(c, 0x003e_135c)?;
            let mut at = sx(chunk.wrapping_add(size as u32));
            set_reg(c, A2, at);
            if (left as i64) > 0 {
                c.cycles += 8;
                let h = Addr(sp0 + 4).lw(c);
                set_reg(c, A1, h);
                let tag_word = Addr((at as u32).wrapping_add(CH_TAG_SIZE)).lw(c) & TAG_MASK;
                set_reg(c, V0, tag_word);
                set_reg(c, A0, TAG_MASK);
                let tag = sx((Addr((h as u32).wrapping_add(H_TAG)).lw(c) as u32) << 24);
                set_reg(c, V1, tag);
                set_reg(c, T1, h);
                if tag_word != tag {
                    c.cycles += 7;
                    set_reg(c, A0, 0xff_0000);
                    let man = Addr(sp0).lw(c);
                    set_reg(c, A1, man);
                    set_reg(c, T0, u64::MAX);
                    set_reg(c, A0, SIZE_MASK);
                    set_reg(c, A3, TAG_MASK);
                    let mut first = Addr(at as u32 + CH_WRAP).lw(c);
                    set_reg(c, V0, first);
                    loop {
                        c.cycles += 2;
                        if first != u64::MAX {
                            let ts = Addr((at as u32).wrapping_add(CH_TAG_SIZE)).lw(c);
                            set_reg(c, V0, ts);
                            c.cycles += 2;
                            let size = ts & SIZE_MASK;
                            set_reg(c, S1, size);
                            at = addu(at, size);
                        } else {
                            c.cycles += 2;
                            at = Addr((man as u32).wrapping_add(MAN_RING)).lw(c);
                        }
                        set_reg(c, A2, at);
                        c.cycles += 4;
                        let tag_word = Addr((at as u32).wrapping_add(CH_TAG_SIZE)).lw(c) & TAG_MASK;
                        set_reg(c, V0, tag_word);
                        if tag_word == tag {
                            break;
                        }
                        first = Addr(at as u32 + CH_WRAP).lw(c);
                        set_reg(c, V0, first);
                        if event_due(c) {
                            deliver_event_here(c, HEAD);
                        }
                    }
                }
                c.cycles += 1;
                Addr((h as u32).wrapping_add(H_NEXT)).sw(c, at);
            }
            c.cycles += 1;
            set_reg(c, V0, sx(chunk));
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod oc230_tanf {
    use crate::call::Callee;

    pub const REM_PIO2F: Callee = Callee {
        addr: 0x0040_e878,
        f: ssx3_ee::s15::__ieee754_rem_pio2f,
    };

    pub const KERNEL_TANF: Callee = Callee {
        addr: 0x0040_f5a8,
        f: ssx3_ee::s15::__kernel_tanf,
    };
}

pub(crate) mod oc230_cpu_rider {

    /// The actor's rider.
    pub const RIDER: u32 = 0x18;

    /// The rider's track query object (vtable at +0x34, query slot 8).
    pub const TRACK_QUERY: u32 = 0xab8;

    pub const QUERY_VTABLE: u32 = 0x34;

    pub const QUERY_SLOT: u32 = 0x8;

    /// The rider's track position and distance.
    pub const TRACK_POS: u32 = 0x4c0;

    pub const TRACK_DIST: u32 = 0x4c4;

    /// Marker records (0x10 bytes: type, value, distance).
    pub const SPEED_MARKER: u64 = 0x11;
}

pub(crate) mod turn_around {

    /// `+0x120` orientation quaternion.
    pub const ROT: u32 = 0x120;

    /// `+0x1c0` world matrix row z (the up axis).
    pub const WORLD_UP: u32 = 0x1c0;

    /// `+0x1e0` velocity.
    pub const VEL: u32 = 0x1e0;

    /// `+0x214` current brake slew value.
    pub const BRAKE: u32 = 0x214;

    /// `+0x3a0` ground forward direction.
    pub const GROUND_FORWARD: u32 = 0x3a0;

    /// `gp-0x7b98`: how far backward (velocity along the ground forward
    /// axis) the rider must move before turning.
    pub const GP_BACKWARD_LIMIT: u32 = 0x7b98;
}

pub(crate) mod prewind {
    use super::ee;
    use super::Callee;

    /// `ComputerActorRider`: its rider; the control block (+0xdf0: speed
    /// threshold, +0x48/+0x4c analog values); the scripted-start flag and
    /// its two points.
    pub const RIDER: u32 = 0x18;

    pub const CONTROL: u32 = 0xdf0;

    pub const SCRIPTED: u32 = 0xe1c;

    pub const SCRIPT_FROM: u32 = 0xe50;

    pub const SCRIPT_TO: u32 = 0xe60;

    /// `Rider`: position, look-ahead point, velocity, track distance and
    /// lateral offset, the track.
    pub const POS: u32 = 0x110;

    pub const VELOCITY: u32 = 0x1e0;

    pub const AHEAD: u32 = 0x490;

    pub const TRACK_DIST: u32 = 0x4c0;

    pub const TRACK_SIDE: u32 = 0x4c4;

    pub const TRACK: u32 = 0xab8;

    /// Track: vtable at +0x34, `Query(out, max, from, to)` at slot 8.
    pub const TRACK_VT: u32 = 0x34;

    pub const SLOT_QUERY: u32 = 0x8;

    /// A track event of this type ends the prewind.
    pub const EVENT_PREWIND_END: u64 = 0x10;

    /// gp-0x7e30: subtracted from the speed threshold.
    pub const GP_SPEED_MARGIN: u32 = 0x7e30;

    /// `Path_GetPointAtDistance(out, track, in_range, t)` (cross-shard).
    pub const PATH_GET_POINT: u32 = 0x0026_ab20;

    pub const COMPUTE_STEER: Callee = Callee {
        addr: 0x0010_0348,
        f: ee::s00::ComputerActorRider_ComputeSteer,
    };
}

/// `float tanf(float x)` (0x40d588, fdlibm): |x| <= pi/4 goes straight to
/// `__kernel_tanf(x, 0, 1)`; inf/NaN give `x - x`; otherwise x is reduced
/// by `__ieee754_rem_pio2f` (y[0], y[1] at sp+0) and the kernel gets
/// `(y0, y1, 1 - 2 * (n & 1))`.
pub fn tanf(c: &mut Cpu) {
    use oc230_tanf as callee;
    guest(c, |c| {
        c.cycles += 11;
        c.mfc1(V0, 12);
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = c.g[V0][0];
        c.g[V1][0] = 0x7fff_ffff;
        c.g[A0][0] &= c.g[V1][0]; // |x| bits
        c.g[V0][0] = (0x3f49_0fda_i64 < c.g[A0][0] as i64) as u64; // > pi/4
        frame.save_ra(c, 0x10);
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.mtc1(0, 13);
            c.g[A0][0] = 1;
        } else {
            c.cycles += 5;
            c.g[V0][0] = (0x7f7f_ffff_i64 < c.g[A0][0] as i64) as u64; // inf/NaN
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.sub_s(0, 12, 12);
                return oc230_tanf_exit(c, frame);
            }
            c.cycles += 8;
            c.g[A0][0] = c.g[SP][0];
            callee::REM_PIO2F.call(c, 0x0040_d5e4)?;
            let s = sp(c);
            c.g[V0][0] &= 1;
            c.g[A0][0] = 1;
            c.g[V0][0] = oc230_sll(c, V0, 1);
            c.lwc1(12, s);
            c.lwc1(13, s.wrapping_add(4));
            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(r32(c, V0)));
        }
        c.cycles += 2;
        callee::KERNEL_TANF.call(c, 0x0040_d604)?;
        oc230_tanf_exit(c, frame)
    })
}

pub(crate) fn oc230_tanf_exit(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 3;
    frame.restore_ra(c, 0x10);
    frame.close(c);
    Ok(())
}

/// `bool ComputerActorRider_FindSpeedMarker(ComputerActorRider* a, float*
/// speed, int* reached)` (0x10bb18): asks the rider's track query for up
/// to 12 markers between its position and 600 units ahead (into a frame
/// buffer); at the first speed marker (type 0x11) writes its speed (value
/// * gp-0x7e04, km/h to units) and whether its distance has been reached,
/// and returns true; false when there is none.
pub fn computer_actor_rider_find_speed_marker(c: &mut Cpu) {
    use oc230_cpu_rider as cr;
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x100);
        c.g[AT][0] = 0x4416_0000; // 600
        c.mtc1(AT, 0);
        frame.save(c, S0, 0xf0);
        frame.save(c, S1, 0xe0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0xd0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0xc0);
        c.g[S2][0] = c.g[A2][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0xc;
        c.g[V0][0] = c.lw(oc230_at(c, S0, cr::RIDER));
        let rider = r32(c, V0);
        c.g[A3][0] = c.lw(rider.wrapping_add(cr::TRACK_QUERY));
        c.lwc1(13, rider.wrapping_add(cr::TRACK_DIST));
        c.g[V1][0] = c.lw(oc230_at(c, A3, cr::QUERY_VTABLE));
        c.lwc1(12, rider.wrapping_add(cr::TRACK_POS));
        c.add_s(13, 13, 0);
        c.g[A0][0] = c.lh(oc230_at(c, V1, cr::QUERY_SLOT));
        c.g[V0][0] = c.lw(oc230_at(c, V1, cr::QUERY_SLOT + 4));
        let t = r32(c, V0);
        c.g[A0][0] = oc230_add(c, A3, A0);
        call_ptr(c, t, 0x0010_bb70)?;
        c.g[A1][0] = c.g[V0][0]; // count
        let none = (c.g[A1][0] as i64) <= 0;
        c.g[A0][0] = 0;
        let mut found = false;
        if !none {
            c.cycles += 3;
            c.lwc1(2, oc230_gp(c, 0u32.wrapping_sub(0x7e04)));
            c.g[A2][0] = cr::SPEED_MARKER;
            c.g[V1][0] = c.g[SP][0];
            loop {
                c.cycles += 3;
                let rec = r32(c, V1);
                c.g[V0][0] = c.lw(rec);
                if c.g[V0][0] == c.g[A2][0] {
                    // speed = value * scale; reached = distance <= rider's.
                    c.cycles += 11;
                    c.lwc1(0, rec.wrapping_add(4));
                    c.cvt_s_w(0, 0);
                    c.lwc1(1, rec.wrapping_add(8));
                    c.mul_s(0, 0, 2);
                    c.swc1(0, r32(c, S1));
                    c.g[V0][0] = c.lw(oc230_at(c, S0, cr::RIDER));
                    c.lwc1(0, r32(c, V0).wrapping_add(cr::TRACK_DIST));
                    c.c_le_s(1, 0);
                    let reached = c.fcc();
                    c.g[V1][0] = 1;
                    if !reached {
                        c.cycles += 1;
                        c.g[V1][0] = 0;
                    }
                    c.cycles += 3;
                    let v = c.g[V1][0];
                    c.sw(r32(c, S2), v);
                    c.g[V0][0] = 1;
                    found = true;
                    break;
                }
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = sx(rec.wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0010_bb88);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0xf0);
        frame.restore(c, S1, 0xe0);
        frame.restore(c, S2, 0xd0);
        frame.restore_ra(c, 0xc0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Render-state stack helpers (2D render context, `*(gp - 0x854)`)
// ---------------------------------------------------------------------------

/// The render context (`lw gp-0x854`).
pub(crate) fn r2d_ctx(c: &mut Cpu) -> u64 {
    let gp = c.g[GP][0] as u32;
    Addr(gp.wrapping_sub(R2D_CTX_GP)).lw(c)
}

/// `top[1] = top[0]` (0x14 bytes: two unaligned doublewords through `d0`,
/// `d1`, then a word through `w`); `top` is the current state.
pub(crate) fn r2d_copy_up(c: &mut Cpu, top: u32, d0: usize, d1: usize, w: usize) {
    c.ldl(d0, top + 0x7);
    c.ldr(d0, top);
    c.ldl(d1, top + 0xf);
    c.ldr(d1, top + 0x8);
    c.g[w][0] = Addr(top).off(0x10).lw(c);
    c.sdl(d0, top + 0x1b);
    c.sdr(d0, top + 0x14);
    c.sdl(d1, top + 0x23);
    c.sdr(d1, top + 0x1c);
    let v = c.g[w][0];
    Addr(top).off(0x24).sw(c, v);
}

/// `reg = ctx->top; v0 = reg->word[off]; v0 = (v0 & and) | or; store`.
pub(crate) fn r2d_bits(c: &mut Cpu, ctx: u32, reg: usize, off: u32, and: u64, or: Option<u64>) {
    c.g[reg][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
    let top = r32(c, reg);
    c.g[V0][0] = Addr(top).off(off).lw(c);
    c.g[V0][0] &= and;
    if let Some(or) = or {
        c.g[V0][0] |= or;
    }
    let v = c.g[V0][0];
    Addr(top).off(off).sw(c, v);
}

// ---------------------------------------------------------------------------
// cPS2FogParticleMan_Render (0x2dbf98)
// ---------------------------------------------------------------------------

/// The fog particle manager: queued particle count (+0), vtable (+4; slot
/// 0x18 draws them).
pub(crate) const FOG_COUNT: u32 = 0x0;

pub(crate) const FOG_VT: u32 = 0x4;

pub(crate) const VT_FOG_DRAW: u32 = 0x18;

/// The sort buffer (8-byte {particle, depth} pairs) and its comparator
/// (`cPS2FogParticleMan_CompareDesc`).
pub(crate) const FOG_SORT_BUF: u32 = 0x0053_7ac0;

pub(crate) const FOG_COMPARE: u32 = 0x002d_c168;

/// `void cPS2FogParticleMan_Render(cPS2FogParticleMan* m)` (0x2dbf98): if
/// particles were queued, sort them back to front (`qsort`), push the 2D
/// render state and set it up for fog sprites (alpha blending, depth test
/// and write settings, the context's fog colour index, full mask), let the
/// manager draw them, pop the state and empty the queue.
pub fn ps2_fog_particle_man_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        let man = r32(c, S3);
        c.g[A1][0] = Addr(man).off(FOG_COUNT).lw(c);
        c.g[A3][0] = 0x2e_0000;
        if c.g[A1][0] != 0 {
            c.cycles += 98;
            // qsort(buf, count, 8, CompareDesc)
            c.g[A0][0] = sx(FOG_SORT_BUF);
            c.g[A3][0] = sx(FOG_COMPARE);
            c.g[A2][0] = 8;
            call_ptr(c, 0x0041_8ef8, 0x002d_bfd4)?;

            // Push the render state.
            c.g[V1][0] = r2d_ctx(c);
            let ctx = r32(c, V1);
            c.g[T5][0] = 0xffff_ffff_ffff_fc1f;
            c.g[T6][0] = 0xffff_ffff_ffff_fffc;
            c.g[V0][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
            c.g[A3][0] = 0xffff_ffff_e000_03ff;
            c.g[T7][0] = u64::MAX;
            let top = r32(c, V0);
            r2d_copy_up(c, top, A0, A1, T0);
            c.g[A2][0] = 0xf_fc00;
            c.g[S0][0] = 0xffff_ffff_ffff_ff83;
            c.g[T1][0] = 0xffff_ffff_ffbf_ffff;
            c.g[V0][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
            c.g[A1][0] = r2d_ctx(c);
            c.g[S1][0] = 0x40_0000;
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x14));
            c.g[T0][0] = 0xffff_ffff_fe7f_ffff;
            let v = c.g[V0][0];
            Addr(ctx).off(R2D_STATE_TOP).sw(c, v);
            c.g[T2][0] = 0xffff_ffff_ffcf_ffff;
            c.g[S2][0] = 0x30_0000;

            // Fog sprite state.
            let ctx = r32(c, A1);
            c.g[A0][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
            c.g[T4][0] = Addr(ctx).off(0xf60).lw(c);
            c.g[T3][0] = 0xffff_ffff_fff0_0fff;
            let top = r32(c, A0);
            c.g[V0][0] = Addr(top).off(0x8).lw(c);
            c.g[V0][0] &= c.g[T5][0];
            c.g[V0][0] |= 0xe0;
            let v = c.g[V0][0];
            Addr(top).off(0x8).sw(c, v);
            let (t6, a3, a2) = (c.g[T6][0], c.g[A3][0], c.g[A2][0]);
            r2d_bits(c, ctx, V1, 0x4, t6, Some(0x2));
            r2d_bits(c, ctx, A0, 0x8, a3, Some(a2));
            c.g[V1][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
            let fog = c.g[T4][0];
            Addr(r32(c, V1)).off(0x10).sh(c, fog);
            c.g[A1][0] = r2d_ctx(c);
            let ctx = r32(c, A1);
            c.g[A0][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
            let top = r32(c, A0);
            c.g[V0][0] = Addr(top).lw(c);
            c.g[V0][0] |= 0xc;
            let v = c.g[V0][0];
            Addr(top).sw(c, v);
            c.g[V1][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
            let mask = c.g[T7][0];
            Addr(r32(c, V1)).off(0x12).sh(c, mask);
            c.g[A2][0] = r2d_ctx(c);
            let ctx = r32(c, A2);
            let (s0, t1, s1, t0, t2, s2, t3) = (
                c.g[S0][0], c.g[T1][0], c.g[S1][0], c.g[T0][0], c.g[T2][0], c.g[S2][0], c.g[T3][0],
            );
            r2d_bits(c, ctx, V1, 0x4, s0, Some(0x14));
            r2d_bits(c, ctx, A0, 0x4, t1, Some(s1));
            r2d_bits(c, ctx, V1, 0x4, t0, None);
            r2d_bits(c, ctx, A1, 0x4, t2, Some(s2));
            r2d_bits(c, ctx, A0, 0x4, t3, None);

            // m->Draw()
            c.g[V1][0] = Addr(man).off(FOG_VT).lw(c);
            let (obj, vt) = (c.g[S3][0], c.g[V1][0]);
            vcall(c, obj, vt, VT_FOG_DRAW, V0, 0x002d_c134)?;

            // Pop the state; empty the queue.
            c.g[V1][0] = r2d_ctx(c);
            let ctx = r32(c, V1);
            c.g[V0][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x14));
            let v = c.g[V0][0];
            Addr(ctx).off(R2D_STATE_TOP).sw(c, v);
            Addr(man).off(FOG_COUNT).sw(c, 0);
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PS2VisualEffectsMan_Render (0x2f0cd0)
// ---------------------------------------------------------------------------

/// The visual effects manager's effect systems (17 pointers from +0).
pub(crate) const VFX_SYSTEMS: u32 = 17;

/// Effect system vtable slot: render.
pub(crate) const VT_VFX_RENDER: u32 = 0x18;

/// `void PS2VisualEffectsMan_Render(PS2VisualEffectsMan* m)` (0x2f0cd0):
/// render each of the 17 effect systems in order (`sys->Render()`).
pub fn ps2_visual_effects_man_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 110;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let man = r32(c, S0);
        for k in 0..VFX_SYSTEMS {
            c.g[A1][0] = Addr(man).off(4 * k).lw(c);
            let sys = c.g[A1][0];
            c.g[V0][0] = Addr(sys as u32).lw(c);
            let vt = c.g[V0][0];
            vcall(c, sys, vt, VT_VFX_RENDER, V1, 0x002f_0cf8 + 0x18 * k)?;
        }
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) const RIDER_BUILD_WORLD_MATRIX: Callee = Callee {
    addr: 0x0011_e098,
    f: ee::s00::Rider_BuildWorldMatrix,
};

pub(crate) const RIDER_TOGGLE_STANCE: Callee = Callee {
    addr: 0x0011_5168,
    f: ee::s00::Rider_ToggleStance,
};

/// `bool Rider_TurnAroundIfBackward(Rider* rider)` (0x114cc0): when the
/// rider is not braking and moves backward along the ground
/// (`vel · groundForward < -limit`), turn half way round the up axis (the
/// orientation becomes `(up, 0) ⊗ rot`, a 180° rotation), rebuild the world
/// matrix and switch stance. Returns whether it turned.
pub fn rider_turn_around_if_backward(c: &mut Cpu) {
    use turn_around::*;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x70);
        c.mtc1(0, 2);
        frame.save(c, S0, 0x60);
        frame.save_ra(c, 0x50);
        let rider = c.g[A0][0];
        c.g[S0][0] = rider;
        let r = rider as u32;
        c.lwc1(0, r.wrapping_add(BRAKE));
        c.c_eq_s(0, 2);
        let not_braking = c.fcc();
        c.g[V0][0] = 0;
        if not_braking {
            c.cycles += 15;
            // f1 = vel · groundForward (VU0 dot product, w lane).
            let gp = c.g[GP][0] as u32;
            c.lwc1(0, gp.wrapping_sub(GP_BACKWARD_LIMIT));
            c.lqc2(3, r.wrapping_add(VEL));
            c.lqc2(5, r.wrapping_add(GROUND_FORWARD));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(V0, 4);
            c.mtc1(V0, 1);
            c.neg_s(0, 0);
            c.c_lt_s(0, 1);
            if c.fcc() {
                // Not backward enough.
                c.g[V0][0] = 0;
            } else {
                c.cycles += 32;
                // The up axis as a pure quaternion (w = 0 from f2), built
                // through the stack as the original does.
                c.lq(V0, r.wrapping_add(WORLD_UP));
                c.g[A0][0] = rider;
                let s = sp(c);
                c.swc1(2, s.wrapping_add(0x1c));
                c.sq(V0, s.wrapping_add(0x30));
                c.lwc1(0, s.wrapping_add(0x30));
                c.lwc1(1, s.wrapping_add(0x34));
                c.lwc1(2, s.wrapping_add(0x38));
                for base in [0x10u32, 0x20] {
                    c.swc1(0, s.wrapping_add(base));
                    c.swc1(1, s.wrapping_add(base + 4));
                    c.swc1(2, s.wrapping_add(base + 8));
                }
                // rot = up ⊗ rot (VU0 quaternion product).
                c.lqc2(4, s.wrapping_add(0x10));
                c.lqc2(5, r.wrapping_add(ROT));
                c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
                c.vu0().opmula(4, 5);
                c.vu0().opmsub(6, 5, 4);
                c.vu0().mula(0b1110, 4, Src::Bc(5, W));
                c.vu0().madda(0b1110, 5, Src::Bc(4, W));
                c.vu0().suba(0b0001, 7, Src::Bc(7, X));
                c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
                c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
                c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
                c.sqc2(8, s.wrapping_add(0x40));
                c.lq(V0, s.wrapping_add(0x40));
                c.sq(V0, r.wrapping_add(ROT));
                c.sq(V0, s);
                RIDER_BUILD_WORLD_MATRIX.call(c, 0x0011_4d98)?;
                c.g[A0][0] = rider;
                RIDER_TOGGLE_STANCE.call(c, 0x0011_4da0)?;
                c.g[V0][0] = 1;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------- PS2VisualEffectsMan_Update

/// `PS2VisualEffectsMan_ApplyWorldPainter(self, index)`.
pub(crate) const VFX_APPLY_WORLD_PAINTER: Callee = Callee {
    addr: 0x002f_00a0,
    f: ee::s09::PS2VisualEffectsMan_ApplyWorldPainter,
};

/// `void PS2VisualEffectsMan_Update(PS2VisualEffectsMan* self)` (0x2f0a98):
/// reset the 17 effect objects (`self->objs[0..17]`, virtual slot `+0x10`
/// each), then for every player view `i` (the count at
/// `world->app(+0x84)->game(+0x84)->players(+0x10)`) apply the world
/// painter's parameters for the view's painter index (`gp+0xab8[i]`) and
/// reset the drawer's view slot `i` (virtual `+0x398` of `ctx+0x10d8`,
/// `ctx` at `gp-0x854`).
pub fn ps2_visual_effects_man_update(c: &mut Cpu) {
    /// `gp-0x848`: the world; `gp-0x854`: the render context.
    const WORLD_GP: u32 = 0x848;
    const CTX_GP: u32 = 0x854;
    /// `gp+0xab8`: the painter index of each view.
    const VIEW_PAINTER_GP: u32 = 0xab8;
    /// Loop head (0x2f0c70), for event delivery.
    const HEAD: u32 = 0x002f_0c70;
    /// The number of player views.
    fn view_count(c: &mut Cpu) -> u64 {
        c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(WORLD_GP));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x84));
        c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x84));
        c.lw(r32(c, A0).wrapping_add(0x10))
    }
    guest(c, |c| {
        c.cycles += 115;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[S2][0] = 0;
        frame.save_ra(c, 0);
        let this = r32(c, S0);
        for k in 0..17u32 {
            // objs[k]->Reset() (virtual +0x10).
            let obj = c.lw(this.wrapping_add(4 * k));
            c.g[A1][0] = obj;
            c.g[V0][0] = c.lw(obj as u32);
            c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x10));
            c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x14));
            let target = r32(c, V1);
            c.g[A0][0] = sx((obj as u32).wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x002f_0acc + 0x18 * k)?;
        }
        let count = view_count(c);
        c.g[V0][0] = count;
        if (count as i64) > 0 {
            c.cycles += 3;
            c.g[S1][0] = sx(r32(c, GP).wrapping_add(VIEW_PAINTER_GP));
            c.g[A1][0] = c.lw(r32(c, S1));
            loop {
                c.cycles += 18;
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                VFX_APPLY_WORLD_PAINTER.call(c, 0x002f_0c7c)?;
                // ctx->drawer->ResetView(i) (virtual +0x398).
                let ctx = c.lw(r32(c, GP).wrapping_sub(CTX_GP));
                c.g[A2][0] = ctx;
                c.g[A1][0] = c.g[S2][0];
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                c.g[V0][0] = c.lw((ctx as u32).wrapping_add(0x10d8));
                c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x398));
                c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x39c));
                let target = r32(c, V1);
                c.g[A0][0] = sx((ctx as u32).wrapping_add(r32(c, A0)));
                call_ptr(c, target, 0x002f_0c9c)?;
                let count = view_count(c);
                let more = (c.g[S2][0] as i64) < (count as i64);
                c.g[V0][0] = more as u64;
                if !more {
                    c.cycles += 1;
                    break;
                }
                c.g[A1][0] = c.lw(r32(c, S1));
                if event_due(c) {
                    deliver_event_here(c, HEAD);
                }
            }
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// --------------------------------------- ComputerActorRider_UpdateSpinPlan

/// `RiderSpinSub_GetFlipProgress(spin, &a, &b)` and
/// `RiderSpinSub_GetSpinProgress(spin, &c, &d)`: the heights still needed
/// to finish the current flip / spin (and with a margin).
pub(crate) const RIDER_SPIN_GET_FLIP_PROGRESS: u32 = 0x0013_5cb0;

pub(crate) const RIDER_SPIN_GET_SPIN_PROGRESS: u32 = 0x0013_5db0;

/// `void ComputerActorRider_UpdateSpinPlan(ComputerActorRider* ai, float
/// height)` (0x10b0e8): drop the AI's planned flip (`+0xe38`) and spin
/// (`+0xe3c`) when there is no longer height for them. Both go when the AI
/// must land (`+0xe20`) and is lower than 1. Otherwise, while planning
/// (`+0xe10`): the flip goes when `height + k0` is below the flip's need,
/// or below its margin while the need is under `k1`; the spin likewise
/// (with `k2`, `k3`), and dropping the spin drops the flip too.
pub fn computer_actor_rider_update_spin_plan(c: &mut Cpu) {
    /// AI fields.
    const RIDER: u32 = 0x18;
    const PLANNING: u32 = 0xe10;
    const MUST_LAND: u32 = 0xe20;
    const FLIP: u32 = 0xe38;
    const SPIN: u32 = 0xe3c;
    /// `rider+0x77c`: the rider state; its spin sub-state at +0x230.
    const RIDER_STATE: u32 = 0x77c;
    const SPIN_SUB: u32 = 0x230;
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        c.g[A1][0] = c.g[SP][0];
        frame.save_fpr(c, 20, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        c.f[20] = c.f[12];
        let ai = r32(c, S0);
        c.g[V0][0] = c.lw(ai.wrapping_add(RIDER));
        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(RIDER_STATE));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(SPIN_SUB));
        call_ptr(c, RIDER_SPIN_GET_FLIP_PROGRESS, 0x0010_b118)?;
        c.g[V0][0] = c.lw(ai.wrapping_add(RIDER));
        c.g[A1][0] = c.g[SP][0] | 0x8;
        c.g[A2][0] = c.g[SP][0] | 0xc;
        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(RIDER_STATE));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(SPIN_SUB));
        call_ptr(c, RIDER_SPIN_GET_SPIN_PROGRESS, 0x0010_b130)?;
        let sp0 = sp(c);
        c.g[V0][0] = c.lw(ai.wrapping_add(MUST_LAND));
        c.g[V1][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 6;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 0);
            c.c_lt_s(20, 0);
            if c.fcc() {
                c.g[V1][0] = 1;
            }
        }
        c.cycles += 2;
        if c.g[V1][0] != 0 {
            // Too low to finish anything: drop both.
            c.cycles += 3;
            c.sw(ai.wrapping_add(FLIP), 0);
            c.sw(ai.wrapping_add(SPIN), 0);
            c.cycles += 1;
        } else {
            c.g[V0][0] = c.lw(ai.wrapping_add(PLANNING));
            c.cycles += 2;
            if c.g[V0][0] != 0 && plan_checks(c, ai, sp0) {
                c.cycles += 1;
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    });

    /// The flip and spin checks (0x10b170..0x10b23c); true when they end
    /// through the shared exit (0x10b23c, one more cycle).
    fn plan_checks(c: &mut Cpu, ai: u32, sp0: u32) -> bool {
        let gp = r32(c, GP);
        // The flip: needs a (sp+0), b (sp+4).
        c.cycles += 6;
        c.lwc1(0, ai.wrapping_add(FLIP));
        c.mtc1(0, 3);
        c.c_eq_s(0, 3);
        if !c.fcc() {
            c.cycles += 7;
            c.lwc1(0, gp.wrapping_sub(0x7e20));
            c.lwc1(2, sp0);
            c.add_s(1, 20, 0);
            c.c_lt_s(1, 2);
            if c.fcc() {
                c.swc1(3, ai.wrapping_add(FLIP));
                c.cycles += 1;
            } else {
                c.cycles += 5;
                c.lwc1(0, sp0 + 0x4);
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.cycles += 6;
                    c.lwc1(0, gp.wrapping_sub(0x7e1c));
                    c.sub_s(0, 2, 0);
                    c.c_lt_s(0, 3);
                    if c.fcc() {
                        c.cycles += 1;
                        c.swc1(3, ai.wrapping_add(FLIP));
                        c.cycles += 1;
                    }
                }
            }
        }
        c.lwc1(0, ai.wrapping_add(SPIN));

        // The spin: needs c (sp+8), d (sp+0xc).
        c.cycles += 5;
        c.mtc1(0, 1);
        c.c_eq_s(0, 1);
        if c.fcc() {
            return false;
        }
        c.cycles += 7;
        c.lwc1(0, gp.wrapping_sub(0x7e18));
        c.lwc1(2, sp0 + 0x8);
        c.add_s(12, 20, 0);
        c.c_lt_s(12, 2);
        if !c.fcc() {
            c.cycles += 5;
            c.lwc1(0, sp0 + 0xc);
            c.c_lt_s(12, 0);
            if !c.fcc() {
                return false;
            }
            c.cycles += 6;
            c.lwc1(0, gp.wrapping_sub(0x7e14));
            c.sub_s(0, 2, 0);
            c.c_lt_s(0, 1);
            if !c.fcc() {
                return false;
            }
            c.cycles += 1;
        }
        c.swc1(1, ai.wrapping_add(FLIP));
        c.cycles += 1;
        c.swc1(1, ai.wrapping_add(SPIN));
        true
    }
}

/// `gp - off` (the small-data constants).
#[inline(always)]
pub(crate) fn gp_lit(c: &Cpu, off: u32) -> u32 {
    r32(c, GP).wrapping_sub(off)
}

// ---------------------------------------------------------------------------
// __ieee754_logf (0x40e550, newlib e_logf.c).

/// `at = bits; mtc1 at, f` (a float literal through `at`).
#[inline(always)]
pub(crate) fn lit(c: &mut Cpu, f: usize, bits: u32) {
    c.g[AT][0] = sx(bits);
    c.mtc1(AT, f);
}

// ---------------------------------------------------------------------------
// ComputerActorRider_ThinkPrewind (0x10aa70, src/seg/seg_1218.cpp).

/// `VF4.x = |VF3|` (the compiled VU0 length: dot product, sqrt into Q).
pub(crate) fn vu0_length_vf3(c: &mut Cpu) {
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
}

/// `void ComputerActorRider_ThinkPrewind(ComputerActorRider* self, Pad*
/// pad)` (0x10aa70): the CPU rider's input before the start. Queries the
/// track events in [dist, dist + 50]; presses bit 14 while slower than the
/// threshold; on a scripted start presses bit 13 once the rider is within
/// 50 of the goal distance and stops there. Otherwise a prewind-end event
/// (type 0x10) presses bit 13 and steers (pad word 1, 6 bits, from
/// `ComputeSteer` towards the event's point relative to half the
/// look-ahead), and the two analog values go to pad bits 15..20 and 21..26
/// (times 31). Frame 0x160 (the event buffer at +0, s0-s3, f20).
pub fn computer_actor_rider_think_prewind(c: &mut Cpu) {
    use prewind::*;
    guest(c, |c| {
        c.cycles += 43;
        let frame = Frame::open(c, 0x160);
        c.g[A2][0] = 0xc;
        frame.save(c, S1, 0x130);
        frame.save(c, S2, 0x120);
        frame.save(c, S0, 0x140);
        frame.save(c, S3, 0x110);
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x150);
        c.g[A1][0] = c.g[SP][0];
        frame.save_ra(c, 0x100);
        let me = r32(c, S2);
        let pad = r32(c, S1);
        lit(c, 20, 0x4248_0000); // 50.0
        c.g[V0][0] = c.lw(me.wrapping_add(RIDER));
        c.g[S0][0] = c.lw(at(c, V0, TRACK));
        c.lwc1(13, at(c, V0, TRACK_SIDE));
        c.g[V1][0] = c.lw(at(c, S0, TRACK_VT));
        c.lwc1(12, at(c, V0, TRACK_DIST));
        c.add_s(13, 13, 20);
        vcall_a0_s0(c, SLOT_QUERY, 0x0010_aacc)?;
        // Slower than the threshold: bit 14.
        c.lwc1(1, me.wrapping_add(CONTROL));
        c.g[A3][0] = c.g[V0][0];
        c.lwc1(2, gp_lit(c, GP_SPEED_MARGIN));
        c.g[T0][0] = u64::MAX;
        c.g[V0][0] = c.lw(me.wrapping_add(RIDER));
        c.sub_s(1, 1, 2);
        c.lqc2(3, at(c, V0, VELOCITY));
        vu0_length_vf3(c);
        c.cfc2(V1, 22);
        c.mtc1(V1, 0);
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.cycles += 4;
            c.g[V0][0] = c.lw(pad) | 0x4000;
            c.sw(pad, c.g[V0][0]);
        }
        c.g[V0][0] = c.lw(me.wrapping_add(SCRIPTED));
        c.cycles += 2;
        c.g[A2][0] = 0;
        if c.g[V0][0] != 0 {
            // 0x10ab34: |pos - from| < |to - from| - 50: bit 13.
            c.cycles += 38;
            let s = sp(c);
            c.lqc2(3, me.wrapping_add(SCRIPT_TO));
            c.lqc2(4, me.wrapping_add(SCRIPT_FROM));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, s.wrapping_add(0xd0));
            c.lq(2, s.wrapping_add(0xd0));
            c.g[V1][0] = c.lw(me.wrapping_add(RIDER));
            c.sq(2, s.wrapping_add(0xc0));
            c.lqc2(3, at(c, V1, POS));
            c.lqc2(4, me.wrapping_add(SCRIPT_FROM));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, s.wrapping_add(0xe0));
            c.lq(2, s.wrapping_add(0xe0));
            c.sq(2, s.wrapping_add(0xd0));
            c.lqc2(3, s.wrapping_add(0xc0));
            vu0_length_vf3(c);
            c.cfc2(V1, 22);
            c.mtc1(V1, 0);
            c.lqc2(3, s.wrapping_add(0xd0));
            vu0_length_vf3(c);
            c.cfc2(V0, 22);
            c.mtc1(V0, 1);
            c.sub_s(0, 0, 20);
            c.c_lt_s(1, 0);
            let close = c.fcc();
            frame.restore(c, S0, 0x140);
            if close {
                c.cycles += 4;
                c.g[V0][0] = c.lw(pad) | 0x2000;
                c.sw(pad, c.g[V0][0]);
            }
        } else {
            // 0x10abdc: look for a prewind-end event (t0 = its index).
            c.cycles += 2;
            c.g[S3][0] = sx(me.wrapping_add(CONTROL));
            if (c.g[A3][0] as i64) > 0 {
                c.cycles += 4;
                c.g[V1][0] = c.lw(sp(c));
                c.g[V0][0] = EVENT_PREWIND_END;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                let mut found = c.g[V1][0] == EVENT_PREWIND_END;
                if found {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(pad);
                    c.g[T0][0] = 0;
                } else {
                    // 0x10ac00
                    loop {
                        c.cycles += 3;
                        let more = (c.g[A2][0] as i64) < (c.g[A3][0] as i64);
                        c.g[V0][0] = more as u64;
                        c.g[A0][0] = sx(r32(c, A2) << 4);
                        if !more {
                            break;
                        }
                        c.cycles += 5;
                        c.g[A1][0] = EVENT_PREWIND_END;
                        c.g[V0][0] = sx(sp(c).wrapping_add(r32(c, A0)));
                        c.g[V1][0] = c.lw(r32(c, V0));
                        if c.g[V1][0] == c.g[A1][0] {
                            c.cycles += 2;
                            c.g[V0][0] = c.lw(pad);
                            c.g[T0][0] = c.g[A2][0];
                            found = true;
                            break;
                        }
                        c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                        if event_due(c) {
                            deliver_event_here(c, 0x0010_ac00);
                        }
                    }
                }
                if found {
                    c.cycles += 2;
                    c.g[V0][0] |= 0x2000;
                    c.sw(pad, c.g[V0][0]);
                }
            }
            // 0x10ac30
            c.cycles += 3;
            c.g[V0][0] = u64::MAX;
            c.g[A0][0] = sx(r32(c, T0) << 4);
            if c.g[T0][0] != u64::MAX {
                // Steer towards the event point (relative to half the
                // look-ahead).
                c.cycles += 45;
                let s = sp(c);
                c.g[V0][0] = c.lw(me.wrapping_add(RIDER));
                c.lqc2(3, at(c, V0, AHEAD));
                c.lqc2(4, at(c, V0, POS));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, s.wrapping_add(0xd0));
                lit(c, 0, 0x3f00_0000);
                c.lq(2, s.wrapping_add(0xd0));
                c.g[V1][0] = sx(s.wrapping_add(0xc).wrapping_add(r32(c, A0)));
                c.g[A1][0] = c.g[S0][0];
                c.sq(2, s.wrapping_add(0xc0));
                c.g[S0][0] = sx(s.wrapping_add(0xe0));
                c.mfc1(V0, 0);
                c.lqc2(4, s.wrapping_add(0xc0));
                c.qmtc2(V0, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, s.wrapping_add(0xe0));
                c.g[A0][0] = sx(s.wrapping_add(0xd0));
                c.lq(2, s.wrapping_add(0xe0));
                c.g[A2][0] = 0;
                c.sq(2, s.wrapping_add(0xc0));
                c.sq(2, s.wrapping_add(0xd0));
                c.lwc1(12, r32(c, V1));
                call_ptr(c, PATH_GET_POINT, 0x0010_aca0)?;
                let s = sp(c);
                c.lqc2(3, s.wrapping_add(0xd0));
                c.lqc2(4, s.wrapping_add(0xc0));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, s.wrapping_add(0xf0));
                c.g[A0][0] = c.g[S2][0];
                c.lq(2, s.wrapping_add(0xf0));
                c.g[A1][0] = c.g[S0][0];
                c.sq(2, s.wrapping_add(0xe0));
                COMPUTE_STEER.call(c, 0x0010_acc4)?;
                lit(c, 1, 0x41f8_0000); // 31.0
                c.g[A0][0] = 0xffff_ffff_ffff_ffc0;
                c.g[V1][0] = c.lw(pad.wrapping_add(4));
                c.mul_s(0, 0, 1);
                c.g[V1][0] &= c.g[A0][0];
                c.cvt_w_s(1, 0);
                c.mfc1(V0, 1);
                c.g[V0][0] &= 0x3f;
                c.g[V1][0] |= c.g[V0][0];
                c.sw(pad.wrapping_add(4), c.g[V1][0]);
            }
            // 0x10acf0: the analog values (6 bits each, times 31).
            c.cycles += 26;
            let ctl = r32(c, S3);
            c.lwc1(0, ctl.wrapping_add(0x48));
            c.g[A0][0] = 0xffff_ffff_ffe0_7fff;
            lit(c, 1, 0x41f8_0000);
            c.g[V1][0] = c.lw(pad);
            c.g[A1][0] = 0xffff_ffff_f81f_ffff;
            c.mul_s(0, 0, 1);
            c.g[V1][0] &= c.g[A0][0];
            c.cvt_w_s(2, 0);
            c.mfc1(V0, 2);
            c.g[V0][0] &= 0x3f;
            c.g[V0][0] = sx(r32(c, V0) << 15);
            c.g[V1][0] |= c.g[V0][0];
            c.sw(pad, c.g[V1][0]);
            c.g[V1][0] &= c.g[A1][0];
            c.lwc1(0, ctl.wrapping_add(0x4c));
            c.mul_s(0, 0, 1);
            c.cvt_w_s(1, 0);
            c.mfc1(V0, 1);
            c.g[V0][0] &= 0x3f;
            c.g[V0][0] = sx(r32(c, V0) << 21);
            c.g[V1][0] |= c.g[V0][0];
            c.sw(pad, c.g[V1][0]);
            frame.restore(c, S0, 0x140);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x130);
        frame.restore(c, S2, 0x120);
        frame.restore(c, S3, 0x110);
        frame.restore_ra(c, 0x100);
        frame.restore_fpr(c, 20, 0x150);
        frame.close(c);
        Ok(())
    })
}

/// `s0->vtable(v1)[slot]` with `a0 = s0 + delta`, the method in v0.
pub(crate) fn vcall_a0_s0(c: &mut Cpu, slot: u32, ret_to: u32) -> crate::call::Flow {
    c.g[A0][0] = c.lh(at(c, V1, slot));
    c.g[V0][0] = c.lw(at(c, V1, slot + 4));
    let method = r32(c, V0);
    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
    call_ptr(c, method, ret_to)
}

// ------------------------------------------------------------ PATH_Update

/// `void PATH_Update(char mode)` (0x3d5128): the PATH music player's tick.
/// Runs only when PATH is not locked (byte 0x44f42d), is initialised (byte
/// 0x44f42c) and no player is current (0x44f430 == 0). It then stores the
/// mode byte, marks "busy" (current = -1), samples the time in ms, refreshes
/// the platform streams, and for each of the four players (0x517610, 0x928
/// bytes each) that is active (+0x34) makes it current (state pointer
/// 0x44f420, current = its handle at +0x30) and, with a handle, selects its
/// master track and updates its tracks. Finally it refreshes the platform
/// streams with reset and clears current and state.
pub fn path_update(c: &mut Cpu) {
    use crate::game::misc::path::{MODE, STATE, TIME};
    /// Non-zero: PATH is locked, skip the tick.
    const LOCKED: u32 = 0x0044_f42d;
    /// Zero: PATH is not initialised.
    const READY: u32 = 0x0044_f42c;
    /// The current player's handle (-1 while updating, 0 when idle).
    const CURRENT: u32 = 0x0044_f430;
    /// The four players.
    const PLAYERS: u32 = 0x0051_7610;
    const PLAYER_SIZE: u32 = 0x928;
    const PLAYER_COUNT: u64 = 4;
    /// Player fields: its handle and its "active" word.
    const HANDLE: u32 = 0x30;
    const ACTIVE: u32 = 0x34;
    const PATH_SELECT_MASTER_TRACK: Callee = Callee {
        addr: 0x003d_4b00,
        f: ee::s14::PATH_SelectMasterTrack,
    };
    const PATH_UPDATE_TRACKS: Callee = Callee {
        addr: 0x003d_4f10,
        f: ee::s14::PATH_UpdateTracks,
    };
    const PATH_PLAT_GET_TIME_MS: u32 = 0x002a_f370;
    const PATH_PLAT_REFRESH_STATUS: u32 = 0x002a_efa8;
    /// The player loop head.
    const LOOP: u32 = 0x003d_51d8;

    guest(c, |c| {
        c.cycles += 15;
        c.g[V1][0] = 0x45_0000;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = Addr(LOCKED).lbu(c);
        c.g[A0][0] &= 0xff;
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);

        let mut run = c.g[V0][0] == 0;
        if run {
            c.cycles += 4;
            c.g[V0][0] = 0x45_0000;
            c.g[V1][0] = Addr(READY).lbu(c);
            run = c.g[V1][0] != 0;
            c.g[V1][0] = 0x45_0000;
        }
        if run {
            c.cycles += 3;
            c.g[V0][0] = Addr(CURRENT).lw(c);
            c.g[S1][0] = c.g[V1][0];
            run = c.g[V0][0] == 0;
        }
        if run {
            // Busy: current = -1; sample the time.
            c.cycles += 22;
            c.g[V0][0] = u64::MAX;
            c.g[V1][0] = 0x45_0000;
            c.sb(MODE, c.g[A0][0]);
            c.g[S2][0] = 0xffff_ffff_ffff_0000;
            c.sw(CURRENT, c.g[V0][0]);
            call_ptr(c, PATH_PLAT_GET_TIME_MS, 0x003d_519c)?;
            c.g[S0][0] = 0;
            c.g[V1][0] = 0x45_0000;
            c.g[A0][0] = 0;
            c.sw(TIME, c.g[V0][0]);
            c.g[S4][0] = 0x45_0000;
            call_ptr(c, PATH_PLAT_REFRESH_STATUS, 0x003d_51b4)?;

            // s0 = player index, a0 = its offset (s0 * 0x928, via mult).
            c.g[FP][0] = PLAYER_SIZE as u64;
            c.g[V0][0] = 0x51_0000;
            c.g[S2][0] |= 0xffff;
            c.g[S3][0] = sx(PLAYERS);
            c.g[S7][0] = u64::MAX;
            c.g[S6][0] = sx(PLAYERS + ACTIVE);
            c.g[S5][0] = sx(PLAYERS + HANDLE);
            c.mult(0, A0, S0, FP);
            loop {
                c.cycles += 4;
                let off = c.g[A0][0] as u32;
                c.g[V0][0] = sx(off.wrapping_add(r32(c, S6)));
                c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                let active = c.g[V1][0] != 0;
                c.g[V0][0] = Addr(CURRENT).lw(c);
                if active {
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        c.sw(CURRENT, c.g[S2][0]);
                    }
                    c.cycles += 3;
                    let in_range = c.g[S0][0] < PLAYER_COUNT;
                    c.g[V0][0] = sx(off.wrapping_add(r32(c, S5)));
                    if in_range {
                        // Make it current: state = the player, current = handle.
                        c.cycles += 6;
                        c.g[V1][0] = sx(off.wrapping_add(r32(c, S3)));
                        c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                        c.sw(STATE, c.g[V1][0]);
                        c.g[V0][0] = c.g[A0][0];
                        c.sw(CURRENT, c.g[A0][0]);
                    } else {
                        c.cycles += 2;
                        if c.g[S0][0] != c.g[S7][0] {
                            c.sw(CURRENT, 0);
                        }
                        c.cycles += 1;
                        c.g[V0][0] = Addr(CURRENT).lw(c);
                    }
                    c.cycles += 2;
                    let handle = c.g[V0][0] != 0;
                    c.g[V0][0] = sx(r32(c, S0).wrapping_add(1));
                    if handle {
                        c.cycles += 4;
                        PATH_SELECT_MASTER_TRACK.call(c, 0x003d_5230)?;
                        PATH_UPDATE_TRACKS.call(c, 0x003d_5238)?;
                        c.cycles += 1;
                        c.g[V0][0] = sx(r32(c, S0).wrapping_add(1));
                    }
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = sx(r32(c, S0).wrapping_add(1));
                }
                c.cycles += 4;
                c.g[S0][0] = c.g[V0][0] & 0xff;
                c.g[V1][0] = (c.g[S0][0] < PLAYER_COUNT) as u64;
                c.mult(0, A0, S0, FP);
                if c.g[V1][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, LOOP);
                }
            }
            // Refresh with reset; idle again.
            c.cycles += 4;
            c.g[A0][0] = 1;
            call_ptr(c, PATH_PLAT_REFRESH_STATUS, 0x003d_5254)?;
            c.sw(CURRENT, 0);
            c.sw(STATE, 0);
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------ Rider_TryEnterBoardPress

/// `bool Rider_TryEnterBoardPress(Rider* r, float dir)` (0x1161d0): start a
/// board press (nose press for `dir > 0`, tail press for `dir < 0`). Fails
/// (returns 0) for `dir == 0`, while the press sub-state (+0x328) is 3 or 4,
/// or, outside the main state 0, when the board's forward axis (+0x1e0) and
/// the velocity (+0x3a0) point apart (VU0 dot product < 0). Otherwise plays
/// animation 0x18 / 0x20 on the rider's anim controller (+0x784) with blend
/// -1, stores the press side (1 nose, 2 tail) at +0x330, enters sub-state
/// BOARD_PRESS and returns 1.
pub fn rider_try_enter_board_press(c: &mut Cpu) {
    /// Press sub-state (3 and 4 block a new press).
    const PRESS_STATE: u32 = 0x328;
    /// Press side: 1 nose, 2 tail.
    const PRESS_SIDE: u32 = 0x330;
    const FORWARD: u32 = 0x1e0;
    const VELOCITY: u32 = 0x3a0;
    const ANIM_CTL: u32 = 0x784;
    const ANIM_NOSE_PRESS: u64 = 0x18;
    const ANIM_TAIL_PRESS: u64 = 0x20;
    const BOARD_PRESS: u64 = 1;
    const ANIM_CONTROLLER_PLAY_ANIM: u32 = 0x0031_28e8;
    const RIDER_GET_MAIN_STATE: Callee = Callee {
        addr: 0x0011_fe98,
        f: ee::s00::Rider_GetMainState,
    };
    const RIDER_SET_SUB_STATE: Callee = Callee {
        addr: 0x0011_fec8,
        f: ee::s00::Rider_SetSubState,
    };
    /// -1.0f: the anim blend argument.
    const MINUS_ONE: u32 = 0xbf80_0000;

    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save_fpr(c, 21, 0x28);
        frame.save_fpr(c, 20, 0x20);
        c.f[21] = 0;
        let dir = FpuF32(c.f[12]);
        c.f[20] = dir.0;
        frame.save(c, S0, 0x10);
        let no_dir = dir.eq_s(FpuF32::ZERO);
        abi::set_fcc(c, no_dir);
        frame.save_ra(c, 0);
        let rider = c.g[A0][0];
        c.g[S0][0] = rider;
        c.g[V0][0] = 0;
        let r = Addr(rider as u32);
        'out: {
            if no_dir {
                break 'out;
            }
            c.cycles += 5;
            let state = r.off(PRESS_STATE).lw(c);
            let blocked = (state as u32).wrapping_sub(3) < 2;
            c.g[V0][0] = 0;
            if blocked {
                break 'out;
            }
            c.cycles += 4;
            c.g[A0][0] = rider;
            RIDER_GET_MAIN_STATE.call(c, 0x0011_6218)?;
            if c.g[V0][0] == 0 {
                // forward . velocity (xyz) via VU0, in f0.
                c.cycles += 13;
                c.lqc2(3, r.off(FORWARD).0);
                c.lqc2(5, r.off(VELOCITY).0);
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(V0, 4);
                c.f[0] = c.g[V0][0] as u32;
                let apart = abi::c_lt_s(c, FpuF32(c.f[0]), FpuF32(c.f[21]));
                c.g[V0][0] = 0;
                if apart {
                    break 'out;
                }
            }
            c.cycles += 5;
            c.f[0] = 0;
            let nose = abi::c_lt_s(c, FpuF32::ZERO, dir);
            c.g[A0][0] = r.off(ANIM_CTL).lw(c);
            let (anim, side, ret) = if nose {
                c.cycles += 7;
                (ANIM_NOSE_PRESS, 1, 0x0011_627c)
            } else {
                c.cycles += 6;
                (ANIM_TAIL_PRESS, 2, 0x0011_6298)
            };
            c.g[A1][0] = anim;
            c.g[AT][0] = sx(MINUS_ONE);
            c.f[12] = MINUS_ONE;
            c.g[A2][0] = 0;
            call_ptr(c, ANIM_CONTROLLER_PLAY_ANIM, ret)?;
            c.g[V1][0] = side;
            c.cycles += 5;
            r.off(PRESS_SIDE).sw(c, side);
            c.g[A0][0] = rider;
            c.g[A1][0] = BOARD_PRESS;
            RIDER_SET_SUB_STATE.call(c, 0x0011_62ac)?;
            c.g[V0][0] = 1;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x28);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

// --------------------------------------------------------- SNDMIDI_Service

/// `void SNDMIDI_Service(void)` (0x3b96b8): the MIDI sequencer tick. For each
/// of the 16 MIDI slots (0x50a120, 0x70 bytes each) that is playing (state
/// byte +0x28 == 3) and whose volume fade (+0x18, `SNDMIDI_UpdateFade`) did
/// not stop it: advance the tick counter (+0x0) and, while the scaled time
/// `ticks * tempo% (+0x24) / 100` has reached the next event's time (+0x4)
/// and the slot is still playing, process one event and count it.
pub fn sndmidi_service(c: &mut Cpu) {
    const SLOTS: u32 = 0x0050_a120;
    const SLOT_SIZE: u64 = 0x70;
    const SLOT_COUNT: i64 = 16;
    /// Slot fields.
    const TICKS: u32 = 0x0;
    const NEXT_EVENT: u32 = 0x4;
    const FADE: u32 = 0x18;
    const TEMPO: u32 = 0x24;
    const STATE: u32 = 0x28;
    const PLAYING: u64 = 3;
    const UPDATE_FADE: Callee = Callee {
        addr: 0x003b_8de8,
        f: ee::s13::SNDMIDI_UpdateFade,
    };
    const PROCESS_EVENT: Callee = Callee {
        addr: 0x003b_94b0,
        f: ee::s13::SNDMIDI_ProcessEvent,
    };

    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = 0x51_0000;
        frame.save(c, S2, 0x70);
        frame.save(c, S5, 0x40);
        c.g[S2][0] = 0; // slot index
        frame.save(c, S6, 0x30);
        c.g[S5][0] = 100; // percent divisor
        frame.save(c, S7, 0x20);
        c.g[S6][0] = PLAYING;
        frame.save(c, FP, 0x10);
        c.g[S7][0] = SLOT_SIZE;
        frame.save(c, S0, 0x90);
        c.g[FP][0] = sx(SLOTS);
        frame.save(c, S1, 0x80);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save_ra(c, 0);
        c.mult(0, V0, S2, S7);
        loop {
            c.cycles += 4;
            let slot = (c.g[V0][0] as u32).wrapping_add(r32(c, FP));
            c.g[S0][0] = sx(slot);
            let e = Addr(slot);
            c.g[S3][0] = e.off(STATE).lb(c);
            c.g[S4][0] = sx(r32(c, S2).wrapping_add(1));
            // false: the path that reaches the slot loop's end directly.
            let mut via_tail = true;
            'slot: {
                if c.g[S3][0] != c.g[S6][0] {
                    break 'slot;
                }
                c.cycles += 3;
                c.g[V0][0] = e.off(FADE).lw(c);
                if c.g[V0][0] != 0 {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S2][0];
                    UPDATE_FADE.call(c, 0x003b_9724)?;
                    if c.g[V0][0] != 0 {
                        // Stopped by its fade.
                        via_tail = false;
                        break 'slot;
                    }
                    c.cycles += 1;
                }
                c.g[V0][0] = e.off(TICKS).lw(c);
                // (divide-by-zero check of the divisor)
                c.cycles += 2;
                c.cycles += 10;
                c.g[A1][0] = e.off(TEMPO).lhu(c);
                c.g[S4][0] = sx(r32(c, S2).wrapping_add(1));
                c.g[A0][0] = sx(r32(c, V0).wrapping_add(1));
                c.g[V1][0] = e.off(NEXT_EVENT).lw(c);
                c.mult(0, V0, V0, A1);
                c.divu(0, V0, S5);
                c.g[S1][0] = c.lo[0]; // scaled time
                c.g[V1][0] = (c.g[S1][0] < c.g[V1][0]) as u64;
                let early = c.g[V1][0] != 0;
                e.off(TICKS).sw(c, c.g[A0][0]);
                if early {
                    break 'slot;
                }
                c.cycles += 3;
                c.g[V0][0] = e.off(STATE).lb(c);
                if c.g[V0][0] != c.g[S3][0] {
                    via_tail = false;
                    break 'slot;
                }
                c.cycles += 1;
                c.g[S3][0] = PLAYING;
                loop {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S2][0];
                    PROCESS_EVENT.call(c, 0x003b_9778)?;
                    c.g[V0][0] = e.off(NEXT_EVENT).lw(c);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                    c.g[V1][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                    let done = c.g[V1][0] != 0;
                    e.off(NEXT_EVENT).sw(c, c.g[V0][0]);
                    if done {
                        break;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = e.off(STATE).lb(c);
                    if c.g[V0][0] != c.g[S3][0] {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003b_9770);
                    }
                }
            }
            if via_tail {
                c.cycles += 1;
            }
            c.g[S2][0] = c.g[S4][0];
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < SLOT_COUNT) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.mult(0, V0, S2, S7);
            if event_due(c) {
                deliver_event_here(c, 0x003b_9700);
            }
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `sx(a + b)` on the low words (`addu`).
#[inline(always)]
pub(crate) fn add32(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

// ---------------------------------------------------------------------------
// libpad.

/// libpad's port table: 0x70 bytes per port, 0x1c per slot; `+0` the DMA
/// buffer (two 0x80-byte halves), `+0x10` non-zero while the port is open.
pub(crate) const PAD_PORTS: u32 = 0x0052_6250;

pub(crate) const PAD_PORT_SIZE: u64 = 0x70;

pub(crate) const PAD_SLOT_SIZE: u64 = 0x1c;

pub(crate) const PAD_OPEN: u32 = 0x10;

/// Offset of the data length word in a pad DMA buffer half.
pub(crate) const PAD_DATA_LEN: u32 = 0x60;

pub(crate) const D_INTR: Callee = Callee {
    addr: 0x0042_c078,
    f: ee::s15::f_0042c078,
};

pub(crate) const E_INTR: Callee = Callee {
    addr: 0x0042_c0c0,
    f: ee::s15::f_0042c0c0,
};

pub(crate) const MEMCPY: Callee = Callee {
    addr: 0x0041_605c,
    f: ee::s15::f_0041605c,
};

pub(crate) const SYNC_DCACHE: Callee = Callee {
    addr: 0x0042_4698,
    f: ee::s15::f_00424698,
};

pub(crate) const LIBPAD_GET_DMA_BUFFER: Callee = Callee {
    addr: 0x003f_f9a8,
    f: ee::s15::f_003ff9a8,
};

/// `int scePadRead(int port, int slot, u8* data)` (0x3ffa58): 0 when the port
/// is closed; otherwise, with interrupts off, copy the current 0x80-byte DMA
/// half to the stack, copy its first `len` (`+0x60`) bytes to `data`, turn
/// interrupts back on if they were, and return `len`.
pub fn sce_pad_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xe0);
        c.g[V1][0] = PAD_PORT_SIZE;
        c.sd(frame.at(c, 0x90), c.g[S1][0]);
        c.g[V0][0] = 0x52_0000;
        c.sd(frame.at(c, 0xa0), c.g[S2][0]);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        c.g[A0][0] = PAD_SLOT_SIZE;
        c.mult(1, V1, S1, V1);
        c.mult(0, A0, S2, A0);
        c.sd(frame.at(c, 0xc0), c.g[S4][0]);
        c.g[V0][0] = sx(PAD_PORTS);
        frame.save_ra(c, 0xd0);
        c.sd(frame.at(c, 0xb0), c.g[S3][0]);
        c.g[A0][0] = add32(c.g[A0][0], c.g[V1][0]);
        c.sd(frame.at(c, 0x80), c.g[S0][0]);
        c.g[V0][0] = add32(c.g[V0][0], c.g[A0][0]);
        c.g[V1][0] = Addr(r32(c, V0) + PAD_OPEN).lw(c);
        c.g[S4][0] = c.g[A2][0];
        if c.g[V1][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 11;
            D_INTR.call(c, 0x003f_fab8)?;
            c.g[S3][0] = c.g[V0][0];
            c.g[S0][0] = c.g[SP][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S2][0];
            LIBPAD_GET_DMA_BUFFER.call(c, 0x003f_facc)?;
            // Copy the 0x80-byte half to the stack, 32 bytes at a time.
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = c.g[V1][0] & 7;
            let aligned = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[SP][0];
            c.cycles += 1;
            c.g[V0][0] = add32(c.g[V1][0], 0x80);
            if aligned {
                loop {
                    c.cycles += 13;
                    let (src, dst) = (r32(c, V1), r32(c, A0));
                    for (k, reg) in [A1, A2, A3, T0].into_iter().enumerate() {
                        c.g[reg][0] = c.ld(src + 8 * k as u32);
                    }
                    for (k, reg) in [A1, A2, A3, T0].into_iter().enumerate() {
                        c.sd(dst + 8 * k as u32, c.g[reg][0]);
                    }
                    c.g[V1][0] = add32(c.g[V1][0], 0x20);
                    c.g[A0][0] = add32(c.g[A0][0], 0x20);
                    if c.g[V1][0] == c.g[V0][0] {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003f_fb40);
                    }
                }
                c.cycles += 1;
            } else {
                loop {
                    c.cycles += 21;
                    let (src, dst) = (r32(c, V1), r32(c, A0));
                    for (k, reg) in [A1, A2, A3, T0].into_iter().enumerate() {
                        let at = src + 8 * k as u32;
                        c.ldl(reg, at + 7);
                        c.ldr(reg, at);
                    }
                    for (k, reg) in [A1, A2, A3, T0].into_iter().enumerate() {
                        let at = dst + 8 * k as u32;
                        c.sdl(reg, at + 7);
                        c.sdr(reg, at);
                    }
                    c.g[V1][0] = add32(c.g[V1][0], 0x20);
                    c.g[A0][0] = add32(c.g[A0][0], 0x20);
                    if c.g[V1][0] == c.g[V0][0] {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003f_fae0);
                    }
                }
                c.cycles += 2;
            }
            let buf = r32(c, S0);
            c.g[A2][0] = Addr(buf + PAD_DATA_LEN).lw(c);
            c.cycles += 6;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S0][0];
            MEMCPY.call(c, 0x003f_fb84)?;
            c.g[V1][0] = 1;
            if c.g[S3][0] == c.g[V1][0] {
                c.cycles += 3;
                E_INTR.call(c, 0x003f_fb98)?;
            }
            c.g[V0][0] = Addr(buf + PAD_DATA_LEN).lw(c);
        }
        c.cycles += 8;
        frame.restore_ra(c, 0xd0);
        c.g[S4][0] = c.ld(frame.at(c, 0xc0));
        c.g[S3][0] = c.ld(frame.at(c, 0xb0));
        c.g[S2][0] = c.ld(frame.at(c, 0xa0));
        c.g[S1][0] = c.ld(frame.at(c, 0x90));
        c.g[S0][0] = c.ld(frame.at(c, 0x80));
        frame.close(c);
        Ok(())
    })
}

/// `u8* libpad_GetDmaBuffer(int port, int slot)` (0x3ff9a8): the port's
/// 0x100-byte DMA buffer (written back from the data cache first), and of
/// its two 0x80-byte halves the one with the larger frame counter (`+0x58`
/// against `+0xd8`).
pub fn libpad_get_dma_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        c.g[V0][0] = PAD_SLOT_SIZE;
        c.g[V1][0] = PAD_PORT_SIZE;
        c.mult(0, A1, A1, V0);
        c.mult(1, A0, A0, V1);
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x52_0000;
        frame.save_ra(c, 0x10);
        c.g[V0][0] = sx(PAD_PORTS);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[A1][0] = add32(c.g[A1][0], c.g[A0][0]);
        c.g[A1][0] = add32(c.g[A1][0], c.g[V0][0]);
        c.g[S0][0] = Addr(r32(c, A1)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = add32(c.g[S0][0], 0xff);
        SYNC_DCACHE.call(c, 0x003f_f9e4)?;
        let buf = r32(c, S0);
        c.g[V0][0] = Addr(buf + 0x58).lw(c);
        c.g[V1][0] = Addr(buf + 0xd8).lw(c);
        frame.restore_ra(c, 0x10);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        c.g[V0][0] = sx(r32(c, V0) << 7);
        c.g[V0][0] = add32(c.g[S0][0], c.g[V0][0]);
        c.g[S0][0] = c.ld(frame.at(c, 0));
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
    use super::*;
    use super::*;
    use super::*;
    use super::*;
    use super::*;
    use super::*;
    use crate::difftest::check_paths;
    pub(crate) use crate::game::fe::ovstatehud::tests::gpa;
    use crate::game::visualfx::boardwakefx::tests::{gp_at, put_spray_slot};

    /// Centred, bottom and top alignment (align 0, positive, negative with
    /// the upper half set) over edge-case floats.
    #[test]
    fn hud_align_y_matches() {
        use crate::difftest::{scramble, Rng};
        check(
            ee::s05::f_0021e7a8__generated,
            hud_align_y,
            300,
            &|c, seed| {
                let mut r = Rng(0x21e7a8 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.g[A1][0] = match seed % 4 {
                    0 => 0,
                    1 => 1 + (r.next_u32() & 0xff) as u64,
                    2 => sx(0xffff_fffe),
                    _ => 0x1_0000_0000,
                };
                c.f[12] = if seed % 3 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(500.0)
                };
                c.f[13] = if seed % 5 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(100.0)
                };
                c.f[14] = if seed % 7 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(4.0)
                };
                c.pc = 0x0021_e7a8;
            },
        );
    }

    /// A dynamic light for `Irradiance_AddDynamicLight` (kind, position,
    /// colour, cone and falloff fields).
    pub(crate) fn dynamic_light(c: &mut Cpu, r: &mut Rng, light: u32, kind: u32) {
        c.bus.write32(light + 0x10, kind);
        for k in 0..3 {
            c.bus.write32(light + 4 * k, r.f32(20.0));
        }
        c.bus.write32(
            light + 0xc,
            crate::game::visualfx::boardwakefx::tests::f(1.0),
        );
        c.bus.write32(
            light + 0x14,
            crate::game::visualfx::boardwakefx::tests::f(f32::from_bits(r.f32(1.5)) + 1.5),
        );
        c.bus.write32(light + 0x18, r.f32(1.0));
        c.bus.write32(
            light + 0x1c,
            crate::game::visualfx::boardwakefx::tests::f(f32::from_bits(r.f32(1.0)) + 1.0),
        );
        for k in 0..6 {
            c.bus.write32(light + 0x20 + 4 * k, r.f32(1.0));
        }
        fill(c, r, light + 0x38, 2);
        for k in 0..8 {
            c.bus.write32(light + 0x40 + 4 * k, r.f32(30.0));
        }
        let outer = f32::from_bits(r.f32(5.0)) + 5.0;
        c.bus.write32(
            light + 0x60,
            crate::game::visualfx::boardwakefx::tests::f(outer),
        );
        c.bus.write32(
            light + 0x5c,
            crate::game::visualfx::boardwakefx::tests::f(outer + 10.0),
        );
        c.bus.write8(light + 0x64, (r.next_u32() % 5) as u8);
        c.bus.write8(light + 0x65, (r.next_u32() % 6) as u8);
    }

    /// `Rider_BuildLightSet` with the real irradiance callees: the view
    /// irradiance copy, the camera-relative light (camera view matrix through
    /// a fake), 0-8 dynamic lights, and the handler's directional block
    /// (absent, empty, or 1-3 lights). The rider's AI-index getter and the
    /// lighting/camera methods are fakes (some seeds unwind at them).
    #[test]
    fn rider_build_light_set_matches() {
        use rider_lights::*;
        check_lohi(
            ee::s00::f_001220d8__generated,
            rider_build_light_set,
            200,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x1220, 4);
                let (rider, vt, hips, bones, handler) = (
                    DATA,
                    DATA + 0x2000,
                    DATA + 0x2100,
                    DATA + 0x2200,
                    DATA + 0x4000,
                );
                let (camera, cam_vt, view) = (DATA + 0x5000, DATA + 0x6000, DATA + 0x5100);
                fill(c, &mut r, rider + LIGHTS, 0x28);
                c.bus.write32(rider + VTABLE, vt);
                c.bus
                    .write16(vt + AI_INDEX_SLOT, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(vt + AI_INDEX_SLOT + 4, FAKE);
                // The view index from both getter calls (forced fake results).
                let view_index = 1 + seed % 3;
                c.bus.write32(LOG_RESULTS, view_index);
                c.bus.write32(LOG_RESULTS + 4, 1 + (seed / 3) % 3);
                c.bus.write32(LOG_RESULTS + 12, view);
                for i in 1..4u32 {
                    fill(c, &mut r, 0x004f_a3c0 + i * 0xf0, 0x28);
                    let (cell, obj, ovt) = (
                        DATA + 0x7000 + i * 0x40,
                        DATA + 0x7400 + i * 0x40,
                        DATA + 0x7800 + i * 0x200,
                    );
                    c.bus.write32(0x004f_a370 + i * 0xf0 + 0x1c, cell);
                    c.bus.write32(cell, obj);
                    c.bus.write32(obj + 4, ovt);
                    c.bus.write16(ovt + 0x150, (r.next_u32() & 0x7c) as u16);
                    c.bus.write32(ovt + 0x154, FAKE + 0x10);
                }
                // Hips: rider+0x780 -> skeleton, +0x2c bones (32 bytes), +0x89c index.
                c.bus.write32(rider + 0x780, hips);
                c.bus.write32(hips + 0x2c, bones);
                c.bus.write32(rider + 0x89c, r.next_u32() % 4);
                for k in 0..32 {
                    c.bus.write32(bones + 4 * k, r.f32(50.0));
                }
                // The camera (gp+0x2a90) and its view matrix.
                c.bus.write32(GAME_GP + 0x2a90, camera);
                c.bus.write32(camera + 0x10d8, cam_vt);
                c.bus.write16(cam_vt + 0x118, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(cam_vt + 0x11c, FAKE + 0x20);
                for k in 0..16 {
                    c.bus.write32(view + 4 * k, r.f32(2.0));
                }
                // Dynamic lights: none, or one of two lights per slot.
                let (l1, l2) = (DATA + 0x3000, DATA + 0x3100);
                dynamic_light(c, &mut r, l1, [1u32, 2, 0, 3][(seed % 4) as usize]);
                dynamic_light(c, &mut r, l2, [2u32, 1, 3, 0][(seed / 4 % 4) as usize]);
                for k in 0..8u32 {
                    let l = match (r.next_u32() % 4, seed % 5) {
                        (_, 0) => 0,
                        (0, _) => l1,
                        (1, _) => l2,
                        _ => 0,
                    };
                    c.bus.write32(rider + DYNAMIC_LIGHTS + 4 * k, l);
                }
                // The handler's directional block.
                c.bus
                    .write32(rider + HANDLER, if seed % 4 == 1 { 0 } else { handler });
                let block = handler + HANDLER_LIGHTS;
                for k in 0..3 {
                    c.bus.write32(block + 4 * k, r.f32(5.0));
                }
                c.bus.write32(block + 0xc, (seed / 2) % 4);
                for k in 0..12 {
                    c.bus.write32(block + 0x10 + 4 * k, r.f32(5.0));
                    c.bus.write32(block + 0x50 + 4 * k, r.f32(1.0));
                }
                c.bus.write32(block + 0x80, r.f32(1.0));
                c.g[A0][0] = sx(rider);
                c.pc = 0x0012_20d8;
            },
        );
    }

    /// `DIntr` with interrupts on and off (random other Status bits).
    #[test]
    fn d_intr_matches() {
        check(ee::s15::f_0042c078__generated, d_intr, 64, &|c, seed| {
            let mut r = Rng(0x42c0_7800 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            let status = r.next_u32() & !(STATUS_EIE as u32);
            c.cop0[12] = if seed % 2 == 0 {
                status
            } else {
                status | STATUS_EIE as u32
            };
            c.pc = 0x0042_c078;
        });
    }

    // ------------------------------------------------------- LensFX_DrawSun

    pub(crate) const SUN_FX: u32 = DATA;

    pub(crate) const SUN_VP: u32 = DATA + 0x300;

    pub(crate) const SUN_HOLDER: u32 = DATA + 0x400;

    pub(crate) const SUN_VT: u32 = DATA + 0x600;

    pub(crate) const SUN_REND: u32 = DATA + 0x1000;

    pub(crate) const SUN_REND_VT: u32 = DATA + 0x3000;

    pub(crate) const SUN_REND_STATE: u32 = DATA + 0x3400;

    pub(crate) const SUN_RESULTS: u32 = DATA + 0x3800;

    /// The lens FX at `SUN_FX` (visible most seeds, no flares), a viewport,
    /// the world painter's sun for views 6 and 7 (methods are fakes whose
    /// forced results point at the colour, intensity, size and texture id
    /// at `SUN_RESULTS`: the alpha is positive or not, the size sometimes
    /// 1), the renderer (`gp+0x2a90` and `gp-0x854`) with a vtable of fakes.
    pub(crate) fn draw_sun_setup(c: &mut Cpu, seed: u32) {
        use crate::difftest::fakes::LOG_RESULTS;
        let mut r = machine(c, seed, 0x002f_4a08, 12);
        for view in [6u32, 7] {
            let entry = 0x004f_a370 + view * 0xf0;
            c.bus.write32(entry + 0x14, SUN_HOLDER);
        }
        c.bus.write32(SUN_HOLDER, SUN_HOLDER + 0x40);
        c.bus.write32(SUN_HOLDER + 0x40 + 4, SUN_VT);
        for (i, off) in [0xe8u32, 0xf0, 0xf8, 0x100, 0x118, 0x108]
            .into_iter()
            .enumerate()
        {
            c.bus
                .write16(SUN_VT + off, ((r.next_u32() % 16) * 4) as u16);
            c.bus.write32(SUN_VT + off + 4, FAKE + 0x10 * i as u32);
        }
        // Results of the six getters (red, green, blue, intensity, size,
        // texture).
        for i in 0..6u32 {
            c.bus.write32(LOG_RESULTS + 4 * i, SUN_RESULTS + 0x10 * i);
        }
        for i in 0..3u32 {
            c.bus.write32(SUN_RESULTS + 0x10 * i, r.f32(1.0));
        }
        let intensity = if seed % 4 == 3 { 0 } else { r.f32(2.0) };
        c.bus.write32(SUN_RESULTS + 0x30, intensity);
        let size = if seed % 3 == 0 {
            0x3f80_0000
        } else {
            r.f32(400.0)
        };
        c.bus.write32(SUN_RESULTS + 0x40, size);
        c.bus.write32(SUN_RESULTS + 0x50, r.next_u32() % 8);
        // The renderer: every method a fake, a random state stack.
        for rend in [gpa(0x2a90), gpa(-0x854)] {
            c.bus.write32(rend, SUN_REND);
        }
        c.bus.write32(SUN_REND + 0x10d8, SUN_REND_VT);
        for off in (0..0x300).step_by(8) {
            c.bus
                .write16(SUN_REND_VT + off, ((r.next_u32() % 16) * 4) as u16);
            c.bus.write32(SUN_REND_VT + off + 4, FAKE + 0x100 + 2 * off);
        }
        c.bus
            .write32(SUN_REND + 0xe84, SUN_REND_STATE + 4 * (seed % 4));
        fill(c, &mut r, SUN_REND_STATE, 0x20);
        fill(c, &mut r, SUN_REND + 0xf50 + 0xcc, 8);
        // The lens FX and the viewport.
        c.bus
            .write16(SUN_FX + 0x2c, if seed % 9 == 0 { 0 } else { 1 });
        c.bus.write32(SUN_FX + 0x1f0, r.f32(600.0));
        c.bus.write32(SUN_FX + 0x1f4, r.f32(400.0));
        c.bus.write32(SUN_FX + 0x200, 0);
        c.bus.write32(SUN_FX + 0x204, r.f32(1.0));
        for i in 0..4 {
            c.bus.write32(SUN_VP + 4 * i, r.f32(640.0));
        }
        c.g[A0][0] = SUN_FX as u64;
        c.g[A1][0] = SUN_VP as u64;
        c.g[A2][0] = if seed % 2 == 0 {
            0
        } else {
            r.next_u32() as u64
        };
        c.pc = 0x002f_4a08;
    }

    #[test]
    fn lens_fx_draw_sun_matches() {
        covered(
            ee::s09::f_002f4a08__generated,
            lens_fx_draw_sun,
            200,
            &draw_sun_setup,
        );
    }

    /// Rider_SetSlew0Target: below and above full-lean speed, rate below,
    /// inside and above its bounds, states 2/3/13 and others.
    #[test]
    fn rider_set_slew0_target_matches() {
        check(
            ee::s00::f_00113e80__generated,
            rider_set_slew0_target,
            300,
            &|c, seed| {
                let mut r = Rng(0x113e_8000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let rider = DATA;
                c.g[A0][0] = sx(rider);
                let v = if seed % 3 == 0 { 900.0 } else { 300.0 };
                for k in 0..4 {
                    c.bus.write32(rider + RIDER_VELOCITY + 4 * k, r.f32(v));
                }
                c.f[12] = if seed % 7 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(2.0)
                };
                c.bus.write32(rider + RIDER_SLEW0, r.f32(2.0));
                put_words(
                    c,
                    GAME_GP - GP_SLEW0_FULL_SPEED,
                    &[
                        694.538_8f32.to_bits(),
                        12.002_403f32.to_bits(),
                        0.1f32.to_bits(),
                        14.001_209f32.to_bits(),
                        0.756_134f32.to_bits(),
                        (1.0f32 / 60.0).to_bits(),
                    ],
                );
                let state = [2, 3, 13, 0, 1, 4, 12, 14][(r.next_u32() % 8) as usize];
                c.bus.write32(rider + RIDER_MAIN_STATE, state);
            },
        );
    }

    // --- PATH_UpdateTracks -------------------------------------------------

    /// The PATH player in normal or streamed mode with a mix of empty, busy,
    /// inactive, fading, stopping and playing tracks. Platform handles: 0 has
    /// no stream (the fades' platform calls take their short path), 1..3 are
    /// streams that are stopped / playing. Every playing track still has
    /// enough queued (deadline or lead), so PATH_TrackAdvance (a large tree
    /// of its own) is not entered.
    pub(crate) fn path_fixture(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x003d_4f10 ^ (seed as u64) << 12);
        crate::difftest::scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
        let state = DATA + 0x100;
        let streamed = seed % 2 == 0;
        c.bus
            .write8(0x0044_f434, if streamed { b'B' } else { b'A' });
        w(c, 0x0044_f420, state);
        let time = 0x1000 + r.next_u32() % 0x1_0000;
        w(c, 0x0044_f438, time);
        let ticks = [r.next_u32() % 0x1000, r.next_u32() % 0x1000];
        w(c, state + 0xc, ticks[0]);
        w(c, state + 0x10, ticks[1]);
        let step = ticks[streamed as usize];
        w(c, state + 0x14, r.next_u32());
        w(c, state + 0x18, r.next_u32());
        w(c, state + 0x91c, 0); // no queues to service
        w(c, GAME_GP + 0x410, DATA + 0xf000); // audio system: no PATH streams
                                              // Platform stream table: 0 none, 1 stopped, 2/3 playing.
        let table = 0x004d_3e98;
        w(c, table, 0);
        for h in 1..4u32 {
            let p = DATA + 0xe000 + 0x100 * h;
            w(c, table + 4 * h, p);
            c.bus.write8(p + 0x64, 1);
            c.bus.write8(p + 0x65, 0);
            w(c, p + 0x58, [0, 0, 1, 7][h as usize]);
            w(c, p + 0xc, 1);
        }
        for i in 0..24u32 {
            let t = DATA + 0x2000 + 0x100 * i;
            let used = r.next_u32() % 5 != 0;
            w(c, state + 0x58 + 4 * i, if used { t } else { 0 });
            crate::difftest::fill(c, &mut r, t, 0x40);
            c.bus.write8(t + 0x3, (r.next_u32() % 6 == 0) as u8);
            c.bus
                .write16(t + 0x36, if r.next_u32() % 4 == 0 { 0xffff } else { 3 });
            w(c, t + 0x30, (r.next_u32() % 5 != 0) as u32);
            let mut fading = false;
            for f in [0xd, 0x15, 0x1d] {
                let on = r.next_u32() % 4 == 0;
                fading |= on;
                c.bus.write8(t + f, if on { 0 } else { 0xff });
            }
            let mine = (r.next_u32() % 4 != 0) == streamed;
            c.bus.write8(t + 0x2f, mine as u8);
            let handle = if fading { 0 } else { 1 + r.next_u32() % 3 };
            w(c, t + 0x54, handle);
            let stop = if handle != 0 && r.next_u32() % 3 == 0 {
                time + r.next_u32() % (step + 2)
            } else {
                0
            };
            w(c, t + 0x4c, stop);
            w(c, t + 0x20, 0); // nothing queued
            let lead = (r.next_u32() % 0x200) as i32 - 0x100;
            let deadline = if lead < 0 && r.next_u32() % 2 == 0 {
                0
            } else {
                (time as i32 + lead) as u32 + step + 1 + r.next_u32() % 0x100
            };
            w(c, t + 0x24, lead as u32);
            w(c, t + 0x48, deadline);
        }
        c.pc = 0x003d_4f10;
    }

    #[test]
    fn path_update_tracks_matches() {
        crate::difftest::check(
            ee::s14::f_003d4f10__generated,
            path_update_tracks,
            200,
            &|c, seed| path_fixture(c, seed),
        );
    }

    /// A float in [lo, hi) as its bit pattern.
    pub(crate) fn uf(r: &mut Rng, lo: f32, hi: f32) -> u32 {
        (lo + (r.next_u32() as f32 / u32::MAX as f32) * (hi - lo)).to_bits()
    }

    pub(crate) fn put_cstr(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    /// HUD_DrawSpeedometer: the BELibrary dummy and interface (a fake
    /// method), the rider's velocity method a fake returning a random vector,
    /// random settings (units bit both ways), HUD elements and text object as
    /// in the position-panel test, speeds giving one to three digits; real
    /// HUD/font/sprintf/strlen callees.
    #[test]
    fn hud_draw_speedometer_matches() {
        use super::speedo::*;
        covered(
            ee::s05::f_002200c0__generated,
            hud_draw_speedometer,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x22_00c0, 2);
                c.pc = 0x0022_00c0;
                let gp = GAME_GP;
                let (hud, elems, rider, text) = (DATA, DATA + 0x1000, DATA + 0x1c00, DATA + 0x800);
                // BELibrary singleton and interface slot 0 with a fake method.
                c.bus.write32(gp - 0x1f04, DATA + 0x30_0000);
                c.bus.write32(gp - 0x1efc, 0);
                let lib_if = DATA + 0x3000;
                c.bus.write32(0x0053_08a8, lib_if);
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(lib_if + 4, vt);
                let vt2 = vtable(c, &mut r, &[SLOT_GET_VELOCITY]);
                c.bus.write32(rider + RIDER_VELOCITY, vt2);
                let vel = DATA + 0x2000;
                c.bus.write32(LOG_RESULTS + 4, vel);
                for k in 0..4 {
                    let v = uf(&mut r, -70.0, 70.0);
                    c.bus.write32(vel + 4 * k, v);
                }
                // The settings block (word 0: the units flag).
                fill(c, &mut r, SETTINGS, 0x288 / 4);
                let w0 = r.next_u32() & !(UNITS_FLAG as u32);
                c.bus.write32(
                    SETTINGS,
                    w0 | if seed % 2 == 0 { UNITS_FLAG as u32 } else { 0 },
                );
                put_cstr(c, NUMBER_FORMAT, b"%d");
                put_cstr(c, LABEL_A, b"MPH");
                put_cstr(c, LABEL_B, b"KPH");
                for off in [
                    GP_NEG_SPEED_SCALE,
                    GP_NEG_UNITS_FACTOR,
                    GP_NEG_SCALE_SHORT,
                    GP_NEG_SCALE_LONG,
                    GP_NEG_LABEL_OFFSET,
                ] {
                    let v = uf(&mut r, 0.2, 3.0);
                    c.bus.write32(gp - off, v);
                }
                // The HUD, its text object and elements.
                c.bus.write32(hud + HUD_TEXT, text);
                c.bus.write32(text + TEXT_BASE_SCALE, uf(&mut r, 1.0, 20.0));
                c.bus
                    .write32(text + TEXT_BASE_SCALE + 4, uf(&mut r, 1.0, 20.0));
                c.bus.write32(text + TEXT_LINE, r.next_u32() % 32);
                for i in 0..48u32 {
                    let e = elems + 0x24 * i;
                    c.bus.write16(e, (r.next_u32() % 640) as u16);
                    c.bus.write16(e + 2, (r.next_u32() % 448) as u16);
                    c.bus.write16(e + 4, (r.next_u32() % 64) as u16);
                    c.bus.write16(e + 6, (r.next_u32() % 64) as u16);
                    for k in 0..4 {
                        let v = uf(&mut r, 0.0, 1.0);
                        c.bus.write32(e + 0x10 + 4 * k, v);
                    }
                    c.bus.write8(e + 0x20, (r.next_u32() % 3) as u8);
                    c.bus.write8(e + 0x21, (r.next_u32() % 3) as u8);
                    c.bus.write8(e + 0x22, (r.next_u32() % 8) as u8);
                }
                // Font_DrawText's globals (as in the position-panel test).
                c.bus.write32(gp - 0xf28, DATA + 0x1800);
                fill(c, &mut r, DATA + 0x1800, 4);
                c.bus.write32(gp - 0x854, DATA + 0x1900);
                c.bus.write32(DATA + 0x1900 + 0xe84, DATA + 0x1a00);
                c.bus.write32(DATA + 0x1a00 + 8, r.next_u32());
                c.g[A0][0] = sx(hud);
                c.g[A1][0] = sx(elems);
                c.g[A2][0] = sx(rider);
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    /// StreamFile_GetChunk: null / bad-magic / valid handles, empty or
    /// non-empty queues, a ring of 2-6 chunks plus a wrap marker with this
    /// handle's tag somewhere other than the chunk taken; the mutex's kernel
    /// calls are fakes (some unwind).
    #[test]
    fn stream_file_get_chunk_matches() {
        use super::chunk::*;
        covered(
            ee::s14::f_003e12e0__generated,
            stream_file_get_chunk,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e_12e0, 4);
                c.pc = 0x003e_12e0;
                let (h, man, ring) = (DATA, DATA + 0x100, DATA + 0x1000);
                c.g[A0][0] = if seed % 13 == 0 { 0 } else { sx(h) };
                c.bus.write32(h, man);
                c.bus.write32(
                    man,
                    if seed % 11 == 0 {
                        r.next_u32()
                    } else {
                        0x4d52_5453
                    },
                );
                // The mutex at man+4: owner, depth, semaphore.
                c.bus.write32(man + 8, r.next_u32() % 4);
                c.bus.write32(man + 0xc, r.next_u32() % 3);
                c.bus.write32(man + 0x10, r.next_u32() % 64);
                c.bus.write32(man + MAN_RING, ring);
                let tag = 1 + r.next_u32() % 255;
                c.bus.write32(h + H_TAG, tag);
                let k = 2 + r.next_u32() % 5;
                let take = r.next_u32() % k;
                let mut hit = (take + 1 + r.next_u32() % (k - 1)) % k;
                if hit == take {
                    hit = (take + 1) % k;
                }
                let mut at = ring;
                let mut take_at = ring;
                let mut take_size = 0;
                for i in 0..k {
                    let size = 0x10 * (1 + r.next_u32() % 8);
                    let t = if i == hit || (i != take && r.next_u32() % 3 == 0) {
                        tag
                    } else {
                        (tag + 1 + r.next_u32() % 254) % 256
                    };
                    let w0 = r.next_u32() & 0x7fff_ffff;
                    c.bus.write32(at, w0);
                    c.bus.write32(at + CH_TAG_SIZE, t << 24 | size);
                    if i == take {
                        take_at = at;
                        take_size = size;
                    }
                    at += size;
                }
                c.bus.write32(at, u32::MAX);
                c.bus.write32(
                    at + CH_TAG_SIZE,
                    r.next_u32() & 0x00ff_ffff | 0x0100_0000 * ((tag + 7) % 256),
                );
                c.bus.write32(h + H_NEXT, take_at);
                let queued = match seed % 4 {
                    0 => 0,
                    1 => take_size,
                    2 => take_size + 0x10 * (1 + r.next_u32() % 16),
                    _ => r.next_u32() % 0x100,
                };
                c.bus.write32(h + H_QUEUED, queued);
            },
        );
    }
    use super::*;
    use super::*;
    use super::*;
    use super::*;
    use super::*;
    use super::*;
    use super::*;
    use crate::difftest::events::driven;
    use crate::difftest::events::Events;
    use crate::difftest::require_paths;
    use crate::difftest::SP as STACK_TOP;
    pub(crate) use crate::game::fe::ovstatehud::tests::obj;

    /// tanf: small (|x| <= pi/4), medium and huge arguments (reduced by
    /// the real `__ieee754_rem_pio2f`, which reads its tables from the ELF
    /// image), and inf/NaN bit patterns; the real `__kernel_tanf` runs.
    #[test]
    fn tanf_matches() {
        check(ee::s15::f_0040d588__generated, tanf, 200, &|c, seed| {
            let mut r = Rng(0x0040_d588 ^ (seed as u64) << 12);
            crate::difftest::elf_image(c);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.pc = 0x0040_d588;
            let sign = (r.next_u32() & 1) << 31;
            c.f[12] = sign
                | match seed % 6 {
                    0 => r.f32(0.78) & 0x7fff_ffff,
                    1 => 0x3f49_0fda + r.next_u32() % 3,
                    2 => (1.0f32 + (r.next_u32() % 1000) as f32 * 0.1).to_bits(),
                    3 => (1.0e4f32 + (r.next_u32() % 100000) as f32 * 1.0e3).to_bits(),
                    4 => 0x7f80_0000 | r.next_u32() % 4 * (r.next_u32() & 0x7f_ffff),
                    _ => r.edge_f32() & 0x7fff_ffff,
                };
        });
    }

    /// ComputerActorRider_FindSpeedMarker: the track query is a fake that
    /// may unwind and returns a forced count (1..12); the marker
    /// records it "wrote" (the frame buffer at sp) are set up in the stack
    /// area beforehand, with a speed marker at a random index or none.
    #[test]
    fn computer_actor_rider_find_speed_marker_matches() {
        use crate::difftest::fakes::LOG_RESULTS;
        use crate::difftest::SP as STACK;
        covered(
            ee::s00::f_0010bb18__generated,
            computer_actor_rider_find_speed_marker,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_bb18, 1);
                c.pc = 0x0010_bb18;
                let (actor, rider, query, vt) = (DATA, DATA + 0x100, DATA + 0x1000, DATA + 0x1100);
                c.bus.write32(actor + oc230_cpu_rider::RIDER, rider);
                c.bus.write32(rider + oc230_cpu_rider::TRACK_QUERY, query);
                let (pos, dist) = (r.f32(1000.0), r.f32(1000.0));
                c.bus.write32(rider + oc230_cpu_rider::TRACK_POS, pos);
                c.bus.write32(rider + oc230_cpu_rider::TRACK_DIST, dist);
                c.bus.write32(query + oc230_cpu_rider::QUERY_VTABLE, vt);
                c.bus.write16(vt + 8, (r.next_u32() % 8 * 4) as u16);
                c.bus.write32(vt + 0xc, FAKE);
                c.bus.write32(GAME_GP - 0x7e04, 27.777_78f32.to_bits());
                // (A non-positive count cannot be forced: the fake returns the
                // forced word zero-extended, and 0 means "not forced".)
                let n = if seed % 6 == 0 {
                    1
                } else {
                    1 + r.next_u32() % 12
                };
                c.bus.write32(LOG_RESULTS, n);
                let buf = STACK - 0x100;
                let marker = if seed % 4 == 0 { 99 } else { r.next_u32() % 12 };
                for i in 0..12u32 {
                    let rec = buf + 0x10 * i;
                    let ty = if i == marker {
                        0x11
                    } else {
                        r.next_u32() % 0x20 & !0x11 | 1
                    };
                    c.bus.write32(rec, ty);
                    c.bus.write32(rec + 4, r.next_u32() % 200);
                    let lim = if r.next_u32() % 2 == 0 {
                        dist
                    } else {
                        r.f32(1000.0)
                    };
                    c.bus.write32(rec + 8, lim);
                }
                c.g[A0][0] = sx(actor);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = sx(DATA + 0x2010);
            },
        );
    }

    /// Empty queue (nothing happens) and 1-6 queued particles sorted by the
    /// real qsort and comparator; random render-state stack contents; the
    /// manager's draw call is a fake (some unwind).
    #[test]
    fn ps2_fog_particle_man_render_matches() {
        covered(
            ee::s09::f_002dbf98__generated,
            ps2_fog_particle_man_render,
            300,
            &|c, seed| {
                c.pc = 0x002d_bf98;
                let mut r = machine(c, seed, 0x2dbf98, 1);
                let gp = c.g[GP][0] as u32;
                let man = DATA;
                let ctx = DATA + 0x1000;
                let stack = DATA + 0x2004;
                let n = seed % 7;
                c.bus.write32(man + FOG_COUNT, n);
                c.bus.write32(man + FOG_VT, DATA + 0x5000);
                crate::game::main::gamemode::tests::slot(
                    c,
                    DATA + 0x5000,
                    VT_FOG_DRAW,
                    (r.next_u32() as i16) & !3,
                    FAKE,
                );
                crate::difftest::fill(c, &mut r, FOG_SORT_BUF, 2 * n);
                c.bus.write32(gp.wrapping_sub(R2D_CTX_GP), ctx);
                c.bus
                    .write32(ctx + R2D_STATE_TOP, stack + 0x14 * (seed % 3));
                c.bus.write32(ctx + 0xf60, r.next_u32());
                crate::difftest::fill(c, &mut r, stack, 0x14);
                c.g[A0][0] = sx(man);
            },
        );
    }

    /// 17 effect systems with fake render methods (some unwind).
    #[test]
    fn ps2_visual_effects_man_render_matches() {
        covered(
            ee::s09::f_002f0cd0__generated,
            ps2_visual_effects_man_render,
            300,
            &|c, seed| {
                c.pc = 0x002f_0cd0;
                let mut r = machine(c, seed, 0x2f0cd0, VFX_SYSTEMS);
                let man = DATA;
                for k in 0..VFX_SYSTEMS {
                    let sys = DATA + 0x100 + 0x40 * k;
                    let vt = DATA + 0x1000 + 0x40 * k;
                    c.bus.write32(man + 4 * k, sys);
                    c.bus.write32(sys, vt);
                    crate::game::main::gamemode::tests::slot(
                        c,
                        vt,
                        VT_VFX_RENDER,
                        (r.next_u32() as i16) & !3,
                        FAKE + 0x10 * k,
                    );
                }
                c.g[A0][0] = sx(man);
            },
        );
    }

    /// Rider_TurnAroundIfBackward: braking (no turn), moving forward (no
    /// turn) and moving backward (turn: rebuild the world matrix and toggle
    /// the stance through the real callees).
    #[test]
    fn rider_turn_around_if_backward_matches() {
        use super::turn_around::*;
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x4cc0, 0);
            elf_image(c);
            c.pc = 0x0011_4cc0;
            let rider = obj(&mut r);
            c.g[A0][0] = rider as u64;
            // Whole rider area the callees touch: random floats.
            for off in (0..0x800).step_by(4) {
                c.bus.write32(rider + off, r.f32(1.5));
            }
            let brake = match seed % 4 {
                0 => r.f32(1.0),
                1 => 0x8000_0000,
                _ => 0,
            };
            c.bus.write32(rider + BRAKE, brake);
            let vel_scale = if seed % 3 == 0 { 1.0 } else { -1.0 };
            for i in 0..4 {
                let v = r.f32(1.0);
                let fwd = r.f32(1.0);
                c.bus.write32(rider + GROUND_FORWARD + 4 * i, fwd);
                // Velocity roughly along (or against) the ground forward.
                let vf = f32::from_bits(fwd) * 3000.0 * vel_scale + f32::from_bits(v);
                c.bus.write32(rider + VEL + 4 * i, vf.to_bits());
                c.bus.write32(rider + 0x110 + 4 * i, r.f32(1e5));
            }
            let owner = obj(&mut r);
            c.bus.write32(rider + 0x77c, owner);
            c.bus.write32(rider + 0x784, owner);
        };
        check(
            ee::s00::f_00114cc0__generated,
            rider_turn_around_if_backward,
            300,
            &setup,
        );
        // Braking (10+4), not backward (10+15+4), turned (and more).
        require_paths(ee::s00::f_00114cc0__generated, 300, &[14, 29], &setup);
        let mut turned = 0;
        for seed in 0..300 {
            let mut c = fresh();
            setup(&mut c, seed);
            ee::s00::f_00114cc0__generated(&mut c);
            turned += (c.g[V0][0] == 1) as u32;
        }
        assert!(turned > 20, "turned {turned}");
    }

    // ---------------------------------------------- PS2VisualEffectsMan_Update

    /// `PS2VisualEffectsMan_Update`: the 17 resets and the drawer's view
    /// resets are fakes (one may unwind); 0..3 views, each through the real
    /// `PS2VisualEffectsMan_ApplyWorldPainter` (its fixture from
    /// `renderstateman`'s test: painter descriptors whose methods are
    /// `MenuSys_IsActive`).
    #[test]
    fn ps2_visual_effects_man_update_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{check, DATA, GAME_GP};
        check(
            ee::s09::f_002f0a98__generated,
            ps2_visual_effects_man_update,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f0a, 20);
                c.pc = 0x002f_0a98;
                let g = GAME_GP;
                let (vt, ctx, players) = (DATA + 0x6000, DATA + 0x7000, DATA + 0x2300);
                // The 17 effect objects share one vtable whose Reset is a fake.
                for k in 0..17u32 {
                    let obj = DATA + 0x5000 + 0x40 * k;
                    c.bus.write32(DATA + 4 * k, obj);
                    c.bus.write32(obj, vt);
                }
                c.bus.write16(vt + 0x10, (r.next_u32() & 0x3c) as u16);
                c.bus.write32(vt + 0x14, FAKE);
                c.bus.write32(g - 0x854, ctx);
                c.bus.write32(ctx + 0x10d8, vt);
                c.bus.write16(vt + 0x398, (r.next_u32() & 0x3c) as u16);
                c.bus.write32(vt + 0x39c, FAKE + 0x10);
                // world -> app (+0x84) -> players (+0x84; the view count at
                // +0x10) and game module (+0x28) -> handle.
                c.bus.write32(g - 0x848, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000 + 0x84, DATA + 0x2100);
                c.bus.write32(DATA + 0x2100 + 0x84, players);
                c.bus.write32(
                    players + 0x10,
                    match seed % 5 {
                        4 => (-(r.next_u32() as i32 % 3).abs()) as u32,
                        k => k,
                    },
                );
                for i in 0..4u32 {
                    c.bus.write32(g + 0xab8 + 4 * i, r.next_u32() % 4);
                }
                c.bus.write32(DATA + 0x2100 + 0x28, DATA + 0x2200);
                c.bus.write32(DATA + 0x2200, r.next_u32() & 1);
                c.bus.write32(g + 0x2a90, DATA + 0x4000);
                c.bus.write32(g + 0xa70, 0);
                for off in (0..0x40u32).step_by(4) {
                    c.bus.write32(0x004f_a370 + off, DATA + 0x1000);
                }
                c.bus.write32(0x004f_a370 + 0x24, r.edge_f32());
                c.bus.write32(DATA + 0x1000, DATA + 0x1100);
                c.bus.write32(DATA + 0x1104, DATA + 0x1200);
                c.bus.write32(DATA + 0x1100, 0);
                for slot in (0x30..0x1b8u32).step_by(8) {
                    c.bus.write16(DATA + 0x1200 + slot, 0);
                    c.bus.write32(DATA + 0x1200 + slot + 4, 0x002c_c098);
                }
                c.bus.write32(g + 0x1238, (seed % 3 == 0) as u32);
                c.bus.write32(g + 0x14ec, (seed % 7 == 0) as u32);
                c.bus.write32(g + 0x12dc, (seed % 4 == 1) as u32);
                c.bus.write32(g + 0xa6c, r.next_u32());
                for off in [
                    0x1244u32, 0x1248, 0x123c, 0x1240, 0x151c, 0x1524, 0x12e4, 0x12e8, 0x12ec,
                    0x12f0, 0x12f4, 0x1300, 0x1304, 0x14f4, 0x14f8, 0x14fc, 0x1500, 0x1504, 0x1508,
                ] {
                    c.bus.write32(g + off, r.next_u32());
                }
                for off in (0..0x260u32).step_by(4) {
                    c.bus.write32(DATA + 0x4000 + off, r.next_u32());
                }
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    // --------------------------------------- ComputerActorRider_UpdateSpinPlan

    /// `ComputerActorRider_UpdateSpinPlan`: the real flip/spin progress
    /// queries (their attribute factor through a fake BE interface, which
    /// may unwind), must-land on/off, planning on/off, planned flip/spin
    /// zero or not, heights and `.lit4` margins chosen to reach each drop.
    #[test]
    fn computer_actor_rider_update_spin_plan_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{check, DATA, GAME_GP};
        const AI: u32 = DATA + 0x9_0000;
        const RIDER: u32 = DATA + 0x9_1000;
        const STATE: u32 = DATA + 0x9_3000;
        const MODEL: u32 = DATA + 0x9_4000;
        const BE_LIBRARY: u32 = DATA + 0x8_6000;
        const BE_IFACE: u32 = DATA + 0x8_7000;
        const BE_VTABLE: u32 = DATA + 0x8_7100;
        check(
            ee::s00::f_0010b0e8__generated,
            computer_actor_rider_update_spin_plan,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_b0e8, 2);
                c.pc = 0x0010_b0e8;
                let pick =
                    |r: &mut crate::difftest::Rng, v: &[u32]| v[(r.next_u32() as usize) % v.len()];
                let f = |x: f32| x.to_bits();
                c.bus.write32(GAME_GP - 0x1f04, BE_LIBRARY);
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, BE_IFACE);
                c.bus.write32(BE_IFACE + 4, BE_VTABLE);
                c.bus.write16(BE_VTABLE + 8, 0);
                c.bus.write32(BE_VTABLE + 0xc, FAKE + 0x80);
                // The progress queries' own `.lit4` constants.
                for off in [0x7330u32, 0x732c, 0x7328, 0x7324, 0x7320, 0x731c] {
                    c.bus.write32(GAME_GP - off, f(0.5));
                }
                // The plan's margins.
                for off in [0x7e14u32, 0x7e18, 0x7e1c, 0x7e20] {
                    c.bus.write32(
                        GAME_GP - off,
                        pick(&mut r, &[f(0.0), f(0.5), f(2.0), f(-1.0)]),
                    );
                }
                c.bus.write32(AI + 0x18, RIDER);
                c.bus.write32(RIDER + 0x77c, STATE);
                let x = STATE + 0x230;
                for off in [0x10u32, 0x14, 0x18, 0x1c, 0x38, 0x3c] {
                    c.bus.write32(x + off, r.f32(4.0));
                }
                c.bus.write32(x + 0x58, MODEL);
                c.bus.write32(MODEL + 0x2ec, r.f32(2.0));
                c.bus.write32(MODEL + 0x86c, r.next_u32() % 4);
                c.bus.write32(MODEL + 0xb34, 1 + r.next_u32() % 3);
                c.bus.write32(AI + 0xe20, pick(&mut r, &[0, 0, 1]));
                c.bus.write32(AI + 0xe10, pick(&mut r, &[0, 1, 1, 1]));
                c.bus
                    .write32(AI + 0xe38, pick(&mut r, &[0, f(1.0), f(1.0)]));
                c.bus
                    .write32(AI + 0xe3c, pick(&mut r, &[0, f(1.0), f(1.0)]));
                c.f[12] = pick(&mut r, &[f(0.5), f(2.0), f(-3.0), f(10.0)]);
                c.g[A0][0] = sx(AI);
            },
        );
    }

    // ------------------------------------------------------------ HUD_DrawTime

    /// A positive float in [lo, hi) as bits.
    pub(crate) fn uf_2(r: &mut Rng, lo: f32, hi: f32) -> u32 {
        (lo + (r.next_u32() as f32 / u32::MAX as f32) * (hi - lo)).to_bits()
    }

    // ------------------------------------------- ComputerActorRider_ThinkPrewind

    /// ComputerActorRider_ThinkPrewind: the track query a fake returning
    /// 1..12 events (written on the stack where the buffer lies), with a
    /// prewind-end event first, later or not at all; slower or faster than
    /// the threshold; the scripted start (inside / outside the 50 margin);
    /// a real Path_GetPointAtDistance on a small track and a real
    /// ComputeSteer whose two virtual methods are fakes (may unwind).
    #[test]
    fn computer_actor_rider_think_prewind_matches() {
        use crate::difftest::fakes::{covered, vtable};
        use crate::difftest::{elf_image, SP};
        use prewind::*;
        const CAR: u32 = 0x00a0_0000;
        const PAD: u32 = 0x00a0_8000;
        const RIDER_AT: u32 = 0x00a1_0000;
        const TRK: u32 = 0x00a2_0000;
        const SEGS: u32 = 0x00a2_1000;
        const VECS: u32 = 0x00a3_0000;
        covered(
            ee::s00::f_0010aa70__generated,
            computer_actor_rider_think_prewind,
            200,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x10_aa70, 3);
                c.pc = 0x0010_aa70;
                let f = |x: f32| x.to_bits();
                fill(c, &mut r, CAR + CONTROL, 0x20);
                c.bus.write32(CAR + CONTROL, f((r.next_u32() % 60) as f32));
                c.bus.write32(CAR + CONTROL + 0x48, uf_2(&mut r, 0.0, 1.0));
                c.bus.write32(CAR + CONTROL + 0x4c, uf_2(&mut r, 0.0, 1.0));
                c.bus.write32(CAR + RIDER, RIDER_AT);
                c.bus.write32(CAR + SCRIPTED, (seed % 4 == 0) as u32);
                for off in [SCRIPT_FROM, SCRIPT_TO] {
                    put_vec(c, CAR + off, [r.f32(200.0), r.f32(200.0), r.f32(200.0), 0]);
                }
                fill(c, &mut r, PAD, 2);
                // The rider.
                for off in [POS, VELOCITY, AHEAD] {
                    put_vec(
                        c,
                        RIDER_AT + off,
                        [r.f32(60.0), r.f32(60.0), r.f32(60.0), 0],
                    );
                }
                c.bus
                    .write32(RIDER_AT + TRACK_DIST, uf_2(&mut r, 0.0, 200.0));
                c.bus.write32(RIDER_AT + TRACK_SIDE, r.f32(20.0));
                c.bus.write32(RIDER_AT + TRACK, TRK);
                let mvt = vtable(c, &mut r, &[0x10, 0x28]);
                c.bus.write32(RIDER_AT + 0x6c0, mvt);
                // The track: Query (fake) and the path segments.
                let tvt = vtable(c, &mut r, &[SLOT_QUERY]);
                c.bus.write32(TRK + TRACK_VT, tvt);
                c.bus.write32(TRK + 8, r.next_u32() % 4);
                for k in 0..3 {
                    c.bus.write32(TRK + 0xc + 4 * k, r.f32(100.0));
                }
                c.bus.write32(TRK + 0x18, SEGS);
                for i in 0..4 {
                    put_vec(
                        c,
                        SEGS + 16 * i,
                        [r.f32(2.0), r.f32(2.0), r.f32(2.0), uf_2(&mut r, 10.0, 60.0)],
                    );
                }
                // Query's result and the events it "wrote".
                let n = 1 + r.next_u32() % 12;
                c.bus.write32(LOG_RESULTS, n);
                let sel = match seed % 3 {
                    0 => 0,
                    1 => r.next_u32() % n,
                    _ => u32::MAX,
                };
                let buf = SP - 0x160;
                for i in 0..12 {
                    let e = buf + 16 * i;
                    fill(c, &mut r, e, 4);
                    let ty = if i == sel { 0x10 } else { r.next_u32() % 0x10 };
                    c.bus.write32(e, ty);
                    c.bus.write32(e + 0xc, uf_2(&mut r, 0.0, 250.0));
                }
                // ComputeSteer's axis and motion vectors.
                put_vec(c, VECS, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
                put_vec(c, VECS + 0x10, [0, 0, 0, 0]);
                c.bus.write32(LOG_RESULTS + 4, VECS);
                c.bus.write32(LOG_RESULTS + 8, VECS + 0x10);
                c.g[A0][0] = CAR as u64;
                c.g[A1][0] = PAD as u64;
            },
        );
    }

    /// `PATH_Update` with its real callees: locked / uninitialised / busy
    /// early outs, and the full tick over four players that are inactive,
    /// active without a handle, or active with a handle (an empty track
    /// list, so `PATH_SelectMasterTrack` and `PATH_UpdateTracks` take their
    /// short paths; no platform streams).
    #[test]
    fn path_update_matches() {
        check(
            ee::s14::f_003d5128__generated,
            path_update,
            120,
            &|c, seed| {
                let mut r = Rng(0x003d_5128 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = crate::difftest::GAME_GP as u64;
                c.pc = 0x003d_5128;
                let gate = seed % 6;
                c.bus.write8(
                    0x0044_f42d,
                    (gate == 0) as u8 * (1 + r.next_u32() as u8 % 3),
                );
                c.bus.write8(0x0044_f42c, (gate != 1) as u8);
                c.bus
                    .write32(0x0044_f430, if gate == 2 { r.next_u32() | 1 } else { 0 });
                c.bus.write32(0x0045_0dcc, r.next_u32()); // timer count
                c.bus.write32(0x0045_0dc8, 1 + r.next_u32() % 0x1000); // timer period
                for i in 0..4u32 {
                    let p = 0x0051_7610 + 0x928 * i;
                    let kind = r.next_u32() % 3;
                    c.bus
                        .write32(p + 0x34, (kind != 0) as u32 * (1 + r.next_u32() % 5));
                    c.bus
                        .write32(p + 0x30, if kind == 2 { DATA + 0x100 * i } else { 0 });
                }
                c.g[A0][0] = r.next_u32() as u64 | (r.next_u32() as u64) << 32;
                if seed % 2 == 0 {
                    c.g[A0][0] = (c.g[A0][0] & !0xff) | b'B' as u64;
                }
            },
        );
    }

    /// `Rider_TryEnterBoardPress` with its real callees: dir 0 / positive /
    /// negative / edge floats, press states 3, 4 and others, main state 0
    /// (the VU0 alignment test, both signs) or not. `AnimController_PlayAnim`
    /// reaches its virtual call through a test vtable at a fake forced to
    /// return 0x207 (its early exit; sometimes it unwinds); `SetSubState`'s
    /// owner has a state id past the state tables.
    #[test]
    fn rider_try_enter_board_press_matches() {
        check(
            ee::s00::f_001161d0__generated,
            rider_try_enter_board_press,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1161d0, 1);
                c.pc = 0x0011_61d0;
                let rider = DATA;
                let owner = DATA + 0x1000;
                let ctl = DATA + 0x2000;
                let vt = DATA + 0x3000;
                c.g[A0][0] = rider as u64;
                c.f[12] = match seed % 5 {
                    0 => 0,
                    1 => 0x8000_0000,
                    2 => r.f32(4.0) & !0x8000_0000,
                    3 => r.f32(4.0) | 0x8000_0000,
                    _ => r.edge_f32(),
                };
                let press = match (seed / 5) % 4 {
                    0 => 3,
                    1 => 4,
                    2 => r.next_u32(),
                    _ => 0,
                };
                c.bus.write32(rider + 0x328, press);
                c.bus.write32(rider + 0x77c, owner);
                let main = if (seed / 20) % 2 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 9
                };
                c.bus.write32(owner + 0xde0, main);
                c.bus.write32(owner + 0xde4, 0xe + r.next_u32() % 8);
                for k in 0..4 {
                    c.bus.write32(rider + 0x1e0 + 4 * k, r.f32(3.0));
                    c.bus.write32(rider + 0x3a0 + 4 * k, r.f32(30.0));
                }
                c.bus.write32(rider + 0x784, ctl);
                c.bus.write32(ctl + 0x58, vt);
                c.bus.write16(vt + 0x20, 0x10);
                c.bus.write32(vt + 0x24, fakes::FAKE);
                c.bus.write32(fakes::LOG_RESULTS, 0x207);
            },
        );
    }

    /// `SNDMIDI_Service` with its real callees over 16 slots: playing or
    /// not; no fade, a fade that keeps playing (no voices of the slot, so
    /// no volume calls) or one that ends (stop with an invalid handle,
    /// returns at once); scaled time before, at or 1..3 events past the next
    /// event. Events are program changes behind a non-zero delta counter
    /// (+0x27), so `SNDMIDI_ProcessEvent` only counts down.
    #[test]
    fn sndmidi_service_matches() {
        check(
            ee::s13::f_003b96b8__generated,
            sndmidi_service,
            100,
            &|c, seed| {
                let mut r = Rng(0x003b_96b8 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = crate::difftest::GAME_GP as u64;
                c.pc = 0x003b_96b8;
                // Voice table: no voice belongs to any slot.
                for v in 0..24 {
                    c.bus.write8(0x0050_a820 + 8 * v + 4, 0xff);
                }
                let events = DATA;
                put_words(c, events, &[0x0005_c000]);
                for i in 0..16u32 {
                    let e = 0x0050_a120 + 0x70 * i;
                    let k = r.next_u32();
                    let playing = k % 5 != 0;
                    c.bus
                        .write8(e + 0x28, if playing { 3 } else { (k % 3) as u8 });
                    match (k >> 4) % 3 {
                        0 => c.bus.write32(e + 0x18, 0),
                        1 => {
                            // Fading up, still below the target.
                            c.bus.write32(e + 0x18, 5);
                            c.bus.write32(e + 0x14, 10);
                            c.bus.write32(e + 0x1c, 100);
                        }
                        _ => {
                            // Fading out to -1: stop.
                            c.bus.write32(e + 0x18, (-5i32) as u32);
                            c.bus.write32(e + 0x14, 2);
                            c.bus.write32(e + 0x1c, u32::MAX);
                            c.bus.write32(e + 0x10, u32::MAX);
                        }
                    }
                    let ticks = r.next_u32() % 1000;
                    let tempo = r.next_u32() % 200;
                    c.bus.write32(e, ticks);
                    c.bus.write16(e + 0x24, tempo as u16);
                    let scaled = ticks * tempo / 100;
                    let next = match (k >> 8) % 5 {
                        0 => scaled + 1 + r.next_u32() % 10,
                        n => scaled.wrapping_sub(n - 1),
                    };
                    c.bus.write32(e + 0x4, next);
                    c.bus.write32(e + 0x8, events);
                    c.bus.write8(e + 0x27, 0xff);
                }
            },
        );
    }

    /// `scePadRead` on closed and open ports, with an 8-byte aligned and a
    /// 4-byte aligned DMA buffer (the two copy loops), either half current,
    /// interrupts on or off, data lengths 0..0x20.
    #[test]
    fn sce_pad_read_matches() {
        check(
            ee::s15::f_003ffa58__generated,
            sce_pad_read,
            120,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x3f_fa58, 0);
                c.cop0[12] = if seed % 2 == 0 { 0 } else { 0x1_0000 };
                let port = (seed / 2) % 2;
                let slot = (seed / 4) % 4;
                let at = PAD_PORTS + 0x70 * port + 0x1c * slot;
                let buf = DATA + 0x1000 + if seed % 3 == 0 { 4 } else { 0 };
                c.bus.write32(at, buf);
                c.bus
                    .write32(at + PAD_OPEN, if seed % 7 == 0 { 0 } else { 1 });
                for k in 0..0x40 {
                    c.bus.write32(buf + 4 * k, r.next_u32());
                }
                // The current half: the one with the larger frame counter.
                c.bus.write32(buf + 0x58, r.next_u32() % 4);
                c.bus.write32(buf + 0xd8, r.next_u32() % 4);
                c.bus.write32(buf + PAD_DATA_LEN, r.next_u32() % 0x21);
                c.bus
                    .write32(buf + 0x80 + PAD_DATA_LEN, r.next_u32() % 0x21);
                c.g[A0][0] = port as u64;
                c.g[A1][0] = slot as u64;
                c.g[A2][0] = sx(DATA + 0x3000);
                c.pc = 0x003f_fa58;
            },
        );
    }

    /// `libpad_GetDmaBuffer` for every port/slot, either half newer,
    /// interrupts on or off (the cache sync's two paths).
    #[test]
    fn libpad_get_dma_buffer_matches() {
        check(
            ee::s15::f_003ff9a8__generated,
            libpad_get_dma_buffer,
            120,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x3f_f9a8, 0);
                c.cop0[12] = if seed % 2 == 0 { 0 } else { 0x1_0000 };
                let port = (seed / 2) % 2;
                let slot = (seed / 4) % 4;
                let buf = DATA + 0x1000 + 0x100 * (r.next_u32() % 8);
                c.bus.write32(PAD_PORTS + 0x70 * port + 0x1c * slot, buf);
                c.bus.write32(buf + 0x58, r.next_u32() % 4);
                c.bus.write32(buf + 0xd8, r.next_u32() % 4);
                c.g[A0][0] = port as u64;
                c.g[A1][0] = slot as u64;
                c.pc = 0x003f_f9a8;
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
