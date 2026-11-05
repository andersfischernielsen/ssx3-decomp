//! Staging wave 203 (oc224): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

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
    (0x001f_31e0, load_indicator_draw, "LoadIndicator_Draw"),
    (
        0x0039_ecb0,
        ui_state_mgr_process_states,
        "UIStateMgr_ProcessStates",
    ),
    (
        0x0028_e100,
        ssx_audio_system_update_peak_music,
        "SSXAudioSystem_UpdatePeakMusic",
    ),
    (0x002f_4a08, lens_fx_draw_sun, "LensFX_DrawSun"),
];

// ---------------------------------------------------------------------------
// Shared register-level helpers.

/// `gp + off` (a global).
#[inline(always)]
fn oc224_gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `reg + off` (a field of the object in `reg`).
#[inline(always)]
fn oc224_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// `ldl/ldr reg, a` (an unaligned doubleword load).
#[inline(always)]
fn oc224_ld_u(c: &mut Cpu, reg: usize, a: u32) {
    c.ldl(reg, a.wrapping_add(7));
    c.ldr(reg, a);
}

/// `sdl/sdr reg, a` (an unaligned doubleword store).
#[inline(always)]
fn oc224_sd_u(c: &mut Cpu, reg: usize, a: u32) {
    c.sdl(reg, a.wrapping_add(7));
    c.sdr(reg, a);
}

// ---------------------------------------------------------------------------
// LoadIndicator_Draw (0x1f31e0).

/// The renderer singleton (`gp-0x854`): its current draw-state record
/// (0x14 bytes; pushed by copying it one record up) and its method table.
mod renderer {
    pub const GP: i32 = -0x854;
    pub const STATE: u32 = 0xe84;
    pub const VTABLE: u32 = 0x10d8;
    /// The current frame's render-context id (stored in the state's +0x10).
    pub const CONTEXT_ID: u32 = 0x1068;
}

/// The load indicator object: `{ wchar_t* text; int busy; float phase;
/// Font* font; }`.
mod load_indicator {
    pub const TEXT: u32 = 0x0;
    pub const BUSY: u32 = 0x4;
    pub const PHASE: u32 = 0x8;
    pub const FONT: u32 = 0xc;
}

/// `void LoadIndicator_Draw(LoadIndicator* li)` (0x1f31e0): while the
/// indicator is busy, pushes a 2D draw state (the default record at
/// 0x501420 with alpha/blend bits set), sets a full-screen viewport and a
/// 640x480 ortho projection, draws the "loading" text right-aligned at
/// x=620 (`Font_MeasureTextW`, `Font_DrawTextW`, scaled 3x the first time
/// then 2x), and a pulsing 18-pixel square left of it whose width follows
/// `|phase - 1|`; then pops the draw state.
pub fn load_indicator_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x60);
        frame.save_ra(c, 0x50);
        frame.save_fpr(c, 23, 0xa8);
        frame.save_fpr(c, 22, 0xa0);
        frame.save_fpr(c, 21, 0x98);
        frame.save_fpr(c, 20, 0x90);
        let this = r32(c, S0);
        c.g[V0][0] = c.lw(this.wrapping_add(load_indicator::BUSY));
        let busy = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw(oc224_gp(c, 0x1dc0));
        if busy {
            load_indicator_draw_busy(c, this)?;
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore_ra(c, 0x50);
        frame.restore_fpr(c, 23, 0xa8);
        frame.restore_fpr(c, 22, 0xa0);
        frame.restore_fpr(c, 21, 0x98);
        frame.restore_fpr(c, 20, 0x90);
        frame.close(c);
        Ok(())
    })
}

/// The busy body of [`load_indicator_draw`] (v0 = the "text scale set" flag
/// at gp+0x1dc0 on entry).
fn load_indicator_draw_busy(c: &mut Cpu, this: u32) -> crate::call::Flow {
    // First use: the text scale (gp+0x1db8/0x1dbc) starts at 3.
    c.cycles += 2;
    if c.g[V0][0] == 0 {
        c.cycles += 7;
        c.g[AT][0] = 0x4040_0000;
        c.mtc1(AT, 0);
        c.g[V0][0] = 1;
        let v = c.g[V0][0];
        c.sw(oc224_gp(c, 0x1dc0), v);
        c.swc1(0, oc224_gp(c, 0x1dbc));
        c.swc1(0, oc224_gp(c, 0x1db8));
    }
    c.lwc1(0, this.wrapping_add(load_indicator::PHASE));
    // f23 = |phase - 1|
    c.cycles += 8;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 1);
    c.mtc1(0, 2);
    c.sub_s(23, 0, 1);
    c.c_lt_s(23, 2);
    if c.fcc() {
        c.neg_s(23, 23);
    }

    // Push the draw state: copy the current record one up, advance.
    c.cycles += 89;
    c.g[A0][0] = c.lw(oc224_gp(c, renderer::GP));
    c.g[V0][0] = c.lw(oc224_at(c, A0, renderer::STATE));
    let st = r32(c, V0);
    oc224_ld_u(c, V1, st);
    oc224_ld_u(c, A1, st.wrapping_add(8));
    c.g[A2][0] = c.lw(st.wrapping_add(0x10));
    oc224_sd_u(c, V1, st.wrapping_add(0x14));
    oc224_sd_u(c, A1, st.wrapping_add(0x1c));
    let v = c.g[A2][0];
    c.sw(st.wrapping_add(0x24), v);
    c.g[V1][0] = c.lw(oc224_at(c, A0, renderer::STATE));
    c.g[A1][0] = c.lw(oc224_gp(c, renderer::GP));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x14));
    let v = c.g[V1][0];
    c.sw(oc224_at(c, A0, renderer::STATE), v);
    // renderer->vt[0xa8]() and vt[0xf8]()
    c.g[V0][0] = c.lw(oc224_at(c, A1, renderer::VTABLE));
    c.g[A0][0] = c.lh(oc224_at(c, V0, 0xa8));
    c.g[V1][0] = c.lw(oc224_at(c, V0, 0xac));
    let t = r32(c, V1);
    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x001f_32ac)?;
    c.g[A1][0] = c.lw(oc224_gp(c, renderer::GP));
    c.g[V0][0] = c.lw(oc224_at(c, A1, renderer::VTABLE));
    c.g[A0][0] = c.lh(oc224_at(c, V0, 0xf8));
    c.g[V1][0] = c.lw(oc224_at(c, V0, 0xfc));
    let t = r32(c, V1);
    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x001f_32c4)?;

    // The new state = the default 2D record (0x501420), with its flag
    // fields at +4 rewritten.
    c.g[V0][0] = c.lw(oc224_gp(c, renderer::GP));
    c.g[A0][0] = 0x0050_0000;
    c.g[A2][0] = 0xffff_ffff_fe7f_0000;
    c.g[T3][0] = 0x0100_0000;
    c.g[V1][0] = c.lw(oc224_at(c, V0, renderer::STATE));
    c.g[A2][0] |= 0xffff;
    c.g[T0][0] = 0xffff_ffff_ffcf_0000;
    c.g[T2][0] = 0xffff_ffff_fff0_0000;
    c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1420));
    let def = r32(c, A3);
    oc224_ld_u(c, T1, def);
    oc224_ld_u(c, V0, def.wrapping_add(8));
    c.g[A1][0] = c.lw(def.wrapping_add(0x10));
    let st = r32(c, V1);
    oc224_sd_u(c, T1, st);
    oc224_sd_u(c, V0, st.wrapping_add(8));
    let v = c.g[A1][0];
    c.sw(st.wrapping_add(0x10), v);
    c.g[T0][0] |= 0xffff;
    c.g[T2][0] |= 0xfff;
    c.g[T1][0] = 0x0001_0000;
    c.g[A3][0] = c.lw(oc224_gp(c, renderer::GP));
    c.g[T1][0] |= 0x4000;
    c.g[T4][0] = 0xffff_ffff_ffff_ff83;
    c.g[A1][0] = 0x0050_0000;
    c.g[A0][0] = c.lw(oc224_at(c, A3, renderer::STATE));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xe60));
    c.g[V0][0] = c.lw(oc224_at(c, A0, 4));
    c.g[V0][0] &= c.g[A2][0];
    c.g[V0][0] |= c.g[T3][0];
    let v = c.g[V0][0];
    c.sw(oc224_at(c, A0, 4), v);
    c.g[V1][0] = c.lw(oc224_at(c, A3, renderer::STATE));
    c.g[V0][0] = c.lw(oc224_at(c, V1, 4));
    c.g[V0][0] &= c.g[T0][0];
    let v = c.g[V0][0];
    c.sw(oc224_at(c, V1, 4), v);
    c.g[A0][0] = c.lw(oc224_at(c, A3, renderer::STATE));
    c.g[V0][0] = c.lw(oc224_at(c, A0, 4));
    c.g[V0][0] &= c.g[T2][0];
    c.g[V0][0] |= c.g[T1][0];
    let v = c.g[V0][0];
    c.sw(oc224_at(c, A0, 4), v);
    c.g[A2][0] = c.lw(oc224_at(c, A3, renderer::STATE));
    c.g[V0][0] = c.lw(oc224_at(c, A2, 4));
    c.g[V0][0] &= c.g[T4][0];
    c.g[V0][0] |= 0x14;
    let v = c.g[V0][0];
    c.sw(oc224_at(c, A2, 4), v);
    // renderer->vt[0x110](0x4ff1a0)
    c.g[V1][0] = c.lw(oc224_at(c, A3, renderer::VTABLE));
    c.g[A0][0] = c.lh(oc224_at(c, V1, 0x110));
    c.g[V0][0] = c.lw(oc224_at(c, V1, 0x114));
    let t = r32(c, V0);
    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x001f_3390)?;

    // Screen width (vt[0x50]) as a float in f20 (unsigned conversion).
    c.g[A0][0] = c.lw(oc224_gp(c, renderer::GP));
    c.g[V1][0] = c.lw(oc224_at(c, A0, renderer::VTABLE));
    c.g[V0][0] = c.lh(oc224_at(c, V1, 0xb8));
    c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add(0xbc));
    c.g[A1][0] = c.lh(oc224_at(c, V1, 0x50));
    c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = c.lw(oc224_at(c, V1, 0x54));
    let t = r32(c, V0);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    call_ptr(c, t, 0x001f_33b4)?;
    let negative = (c.g[V0][0] as i64) < 0;
    c.g[A1][0] = c.lw(oc224_gp(c, renderer::GP));
    if !negative {
        c.cycles += 4;
        c.mtc1(V0, 20);
        c.cvt_s_w(20, 20);
        c.g[V1][0] = c.lw(oc224_at(c, A1, renderer::VTABLE));
    } else {
        c.cycles += 7;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
        c.g[V0][0] &= 1;
        c.g[V0][0] |= c.g[V1][0];
        c.mtc1(V0, 20);
        c.cvt_s_w(20, 20);
        c.add_s(20, 20, 20);
        c.g[V1][0] = c.lw(oc224_at(c, A1, renderer::VTABLE));
    }
    // Screen height (vt[0x58]) in f15.
    c.cycles += 6;
    c.g[A0][0] = c.lh(oc224_at(c, V1, 0x58));
    c.g[V0][0] = c.lw(oc224_at(c, V1, 0x5c));
    let t = r32(c, V0);
    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x001f_33f8)?;
    let negative = (c.g[V0][0] as i64) < 0;
    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
    if !negative {
        c.cycles += 4;
        c.mtc1(V0, 15);
        c.cvt_s_w(15, 15);
    } else {
        c.cycles += 5;
        c.g[V0][0] &= 1;
        c.g[V0][0] |= c.g[V1][0];
        c.mtc1(V0, 15);
        c.cvt_s_w(15, 15);
        c.add_s(15, 15, 15);
    }

    // Viewport (0, 0, w, h): vt[0xb8](1).
    c.cycles += 157;
    c.mtc1(0, 12);
    c.g[A0][0] = c.g[S1][0];
    c.g[V0][0] = c.lw(r32(c, S2));
    c.mov_s(14, 20);
    c.mov_s(13, 12);
    let t = r32(c, V0);
    c.g[A1][0] = 1;
    call_ptr(c, t, 0x001f_3440)?;
    // Ortho 640x480: vt[0xd0](0, 0, 640, 480, 0, 1, 1, 0).
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 20);
    c.g[A3][0] = c.lw(oc224_gp(c, renderer::GP));
    c.g[A1][0] = 1;
    c.mtc1(0, 21);
    c.mov_s(17, 20);
    c.g[V0][0] = c.lw(oc224_at(c, A3, renderer::VTABLE));
    c.g[A2][0] = 0;
    c.g[AT][0] = 0x4420_0000;
    c.mtc1(AT, 14);
    c.mov_s(16, 21);
    c.g[A0][0] = c.lh(oc224_at(c, V0, 0xd0));
    c.mov_s(12, 21);
    c.g[AT][0] = 0x43f0_0000;
    c.mtc1(AT, 15);
    c.mov_s(13, 21);
    c.g[V1][0] = c.lw(oc224_at(c, V0, 0xd4));
    c.lwc1(22, oc224_gp(c, -0x5558));
    let t = r32(c, V1);
    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x001f_3490)?;

    // Font colour (1, 1, 1, 0) at +0x40, scaled size = size * k.
    let sp = sp(c);
    c.g[V0][0] = c.lw(this.wrapping_add(load_indicator::FONT));
    c.g[A2][0] = 0;
    c.swc1(20, sp);
    c.g[A3][0] = 0;
    c.swc1(20, sp.wrapping_add(4));
    c.swc1(20, sp.wrapping_add(8));
    c.swc1(21, sp.wrapping_add(0xc));
    oc224_ld_u(c, T0, sp);
    oc224_ld_u(c, T1, sp.wrapping_add(8));
    let font = r32(c, V0);
    oc224_sd_u(c, T0, font.wrapping_add(0x40));
    oc224_sd_u(c, T1, font.wrapping_add(0x48));
    c.lwc1(2, oc224_gp(c, -0x5554));
    c.g[V0][0] = c.lw(this.wrapping_add(load_indicator::FONT));
    let font = r32(c, V0);
    c.lwc1(0, font.wrapping_add(0x30));
    c.lwc1(1, font.wrapping_add(0x34));
    c.mul_s(0, 0, 2);
    c.mul_s(1, 1, 2);
    c.swc1(0, font.wrapping_add(0x38));
    c.swc1(1, font.wrapping_add(0x3c));
    // Shadow colour (1, 0, 0, 0) at +0x50; text scale from gp+0x1db8.
    c.swc1(20, sp);
    c.g[V1][0] = c.lw(this.wrapping_add(load_indicator::FONT));
    c.swc1(21, sp.wrapping_add(4));
    c.swc1(21, sp.wrapping_add(8));
    c.swc1(21, sp.wrapping_add(0xc));
    oc224_ld_u(c, V0, sp);
    oc224_ld_u(c, A0, sp.wrapping_add(8));
    let font = r32(c, V1);
    oc224_sd_u(c, V0, font.wrapping_add(0x50));
    oc224_sd_u(c, A0, font.wrapping_add(0x58));
    c.g[V0][0] = c.lw(this.wrapping_add(load_indicator::FONT));
    oc224_ld_u(c, T1, oc224_gp(c, 0x1db8));
    oc224_sd_u(c, T1, oc224_at(c, V0, 0x28));
    // width = Font_MeasureTextW(font, text)
    c.g[A0][0] = c.lw(this.wrapping_add(load_indicator::FONT));
    c.g[A1][0] = c.lw(this.wrapping_add(load_indicator::TEXT));
    c.lwc1(12, oc224_at(c, A0, 0x38));
    c.lwc1(13, oc224_at(c, A0, 0x3c));
    call_ptr(c, 0x0039_21f0, 0x001f_3548)?;
    // Font_DrawTextW at (620 - width, f22 - lines * height * k).
    c.g[V0][0] = c.lw(this.wrapping_add(load_indicator::FONT));
    c.lwc1(4, oc224_gp(c, -0x5550));
    c.lwc1(1, oc224_at(c, V0, 0x14));
    c.cvt_s_w(1, 1);
    c.lwc1(3, oc224_at(c, V0, 0x34));
    c.g[AT][0] = 0x441b_0000;
    c.mtc1(AT, 2);
    c.swc1(21, sp.wrapping_add(8));
    c.mul_s(1, 1, 3);
    c.swc1(20, sp.wrapping_add(0xc));
    c.sub_s(2, 2, 0);
    c.g[A0][0] = c.lw(oc224_gp(c, renderer::GP));
    c.mul_s(1, 1, 4);
    c.swc1(2, sp);
    c.sub_s(1, 22, 1);
    c.swc1(1, sp.wrapping_add(4));
    c.g[V1][0] = c.lw(oc224_at(c, A0, renderer::STATE));
    c.g[V0][0] = c.lw(oc224_at(c, V1, 8));
    c.g[V0][0] |= 0x3e0;
    let v = c.g[V0][0];
    c.sw(oc224_at(c, V1, 8), v);
    c.lwc1(12, sp);
    c.lwc1(13, sp.wrapping_add(4));
    c.g[A0][0] = c.lw(this.wrapping_add(load_indicator::FONT));
    c.g[A1][0] = c.lw(this.wrapping_add(load_indicator::TEXT));
    call_ptr(c, 0x0039_1e30, 0x001f_35ac)?;

    // The pulsing square: vt[0x270](pos, size, uv0, uv1, colour), with
    // the text scale back at 2.
    c.g[AT][0] = 0x4000_0000;
    c.mtc1(AT, 0);
    c.g[A1][0] = c.g[SP][0];
    c.g[V0][0] = c.lw(this.wrapping_add(load_indicator::FONT));
    c.g[T0][0] = sx(sp.wrapping_add(0x30));
    c.swc1(0, sp.wrapping_add(0x14));
    c.g[T1][0] = sx(sp.wrapping_add(0x40));
    c.swc1(0, sp.wrapping_add(0x10));
    c.g[A2][0] = sx(sp.wrapping_add(0x10));
    oc224_ld_u(c, A0, sp.wrapping_add(0x10));
    oc224_sd_u(c, A0, oc224_at(c, V0, 0x28));
    c.g[A3][0] = sx(sp.wrapping_add(0x20));
    c.g[AT][0] = 0x4190_0000;
    c.mtc1(AT, 3);
    c.lwc1(0, sp);
    c.g[AT][0] = 0x4188_0000;
    c.mtc1(AT, 1);
    c.mul_s(2, 23, 3);
    c.swc1(22, sp.wrapping_add(4));
    c.sub_s(0, 0, 1);
    c.swc1(21, sp.wrapping_add(0x24));
    c.swc1(2, sp.wrapping_add(0x10));
    c.swc1(3, sp.wrapping_add(0x14));
    c.swc1(0, sp);
    c.swc1(21, sp.wrapping_add(0x20));
    c.swc1(20, sp.wrapping_add(0x30));
    c.swc1(20, sp.wrapping_add(0x34));
    c.g[V1][0] = c.lw(oc224_gp(c, renderer::GP));
    c.g[A0][0] = c.lw(oc224_at(c, V1, renderer::STATE));
    c.g[V0][0] = c.lw(oc224_at(c, V1, renderer::CONTEXT_ID));
    let v = c.g[V0][0];
    c.sh(oc224_at(c, A0, 0x10), v);
    c.g[V1][0] = c.lw(oc224_gp(c, renderer::GP));
    c.g[T2][0] = c.lw(oc224_at(c, V1, renderer::VTABLE));
    c.g[A0][0] = c.lh(oc224_at(c, T2, 0x270));
    c.swc1(20, sp.wrapping_add(0x4c));
    c.swc1(20, sp.wrapping_add(0x40));
    c.swc1(20, sp.wrapping_add(0x44));
    c.swc1(20, sp.wrapping_add(0x48));
    c.g[V0][0] = c.lw(oc224_at(c, T2, 0x274));
    let t = r32(c, V0);
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x001f_3658)?;
    // vt[0x100]() and vt[0xb0]()
    c.g[A1][0] = c.lw(oc224_gp(c, renderer::GP));
    c.g[V0][0] = c.lw(oc224_at(c, A1, renderer::VTABLE));
    c.g[A0][0] = c.lh(oc224_at(c, V0, 0x100));
    c.g[V1][0] = c.lw(oc224_at(c, V0, 0x104));
    let t = r32(c, V1);
    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x001f_3670)?;
    c.g[A1][0] = c.lw(oc224_gp(c, renderer::GP));
    c.g[V0][0] = c.lw(oc224_at(c, A1, renderer::VTABLE));
    c.g[A0][0] = c.lh(oc224_at(c, V0, 0xb0));
    c.g[V1][0] = c.lw(oc224_at(c, V0, 0xb4));
    let t = r32(c, V1);
    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x001f_3688)?;
    // Pop the draw state.
    c.g[A0][0] = c.lw(oc224_gp(c, renderer::GP));
    c.g[V0][0] = c.lw(oc224_at(c, A0, renderer::STATE));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
    let v = c.g[V0][0];
    c.sw(oc224_at(c, A0, renderer::STATE), v);
    Ok(())
}

// ---------------------------------------------------------------------------
// UIStateMgr_ProcessStates (0x39ecb0).

/// A UI state (list node): `{ prev; next; vptr; id; ..; flags (+0x1c);
/// owner (+0x20) }`. Flags: bit 0 top-level, bit 3 shown, bit 4 input,
/// bit 5 active, bit 7 remove-on-close, bits 8..13 the phase, bit 14 keep
/// the owner hidden.
mod ui_state {
    pub const NEXT: u32 = 0x4;
    pub const VPTR: u32 = 0x8;
    pub const FLAGS: u32 = 0x1c;
    pub const OWNER: u32 = 0x20;
    /// `flags & ~(0x3f << 8)` (clear the phase).
    pub const PHASE_MASK: u64 = 0xffff_ffff_ffff_c0ff;
}

mod ui_callee {
    use super::Callee;
    use ssx3_ee as ee;
    pub const LIST_FIRST: Callee = Callee {
        addr: 0x0039_7840,
        f: ee::s13::UIList_First,
    };
    pub const NODE_IS_SENTINEL: Callee = Callee {
        addr: 0x0039_7718,
        f: ee::s13::UIListNode_IsSentinel,
    };
    pub const NODE_UNLINK: Callee = Callee {
        addr: 0x0039_7740,
        f: ee::s13::UIListNode_Unlink,
    };
    pub const UPDATE_SCREENS: Callee = Callee {
        addr: 0x0039_e868,
        f: ee::s13::UIState_UpdateScreens,
    };
    pub const SET_PARENT_CHAIN_VISIBLE: Callee = Callee {
        addr: 0x0039_f7b8,
        f: ee::s13::UIStateMgr_SetParentChainVisible,
    };
    pub const HAS_STATE_IN_PHASE: Callee = Callee {
        addr: 0x0039_eb60,
        f: ee::s13::UIStateMgr_HasStateInPhase,
    };
    pub const GET_ACTIVE_STATE: Callee = Callee {
        addr: 0x0039_fa88,
        f: ee::s13::UIStateMgr_GetActiveState,
    };
    pub const RESTART_STATE_IN_PHASE: Callee = Callee {
        addr: 0x0039_ea90,
        f: ee::s13::UIStateMgr_RestartStateInPhase,
    };
}

/// The loop head (resume point after an unwinding event).
const PROCESS_STATES_HEAD: u32 = 0x0039_ed00;

/// `int UIStateMgr_ProcessStates(UIStateMgr* mgr)` (0x39ecb0): steps every
/// active state of the manager's list through its open/close phases
/// (2 opening -> 4 -> 3 -> 5 open; 6 closing -> 7 closed -> 8), calling the
/// state's phase methods, updating its screens, showing/hiding the owner
/// chain and restarting a state when the last one closed. Returns
/// `!opened` if a state finished closing (else 0). Dispatcher-shaped: an
/// unwinding event at the loop back edge returns with `pc` at
/// [`PROCESS_STATES_HEAD`], where the trampoline re-enters it (loop state in
/// s0/s2/s6/s7 and the node's flags in v1).
pub fn ui_state_mgr_process_states(c: &mut Cpu) {
    guest(c, |c| {
        let mut run_body = match c.pc {
            0x0039_ecb0 => {
                c.cycles += 17;
                let frame = Frame::open(c, 0x90);
                frame.save(c, S2, 0x60);
                c.g[S2][0] = c.g[A0][0];
                frame.save(c, S0, 0x80);
                frame.save(c, S6, 0x20);
                frame.save(c, S7, 0x10);
                c.g[S6][0] = 0;
                frame.save(c, S1, 0x70);
                frame.save(c, S3, 0x50);
                frame.save(c, S4, 0x40);
                frame.save(c, S5, 0x30);
                frame.save_ra(c, 0);
                c.g[S7][0] = 0;
                ui_callee::LIST_FIRST.call(c, 0x0039_ece8)?;
                c.g[S0][0] = c.g[V0][0];
                if c.g[S0][0] == 0 {
                    return process_states_finish(c);
                }
                c.cycles += 2;
                false
            }
            PROCESS_STATES_HEAD => true,
            pc => {
                c.jump_out(pc, 0x0039_ecb0, 0x0039_f0fc);
                return Ok(());
            }
        };
        loop {
            if run_body {
                process_states_step(c)?;
            }
            run_body = true;
            // 0x39f0b4: next node, until the sentinel.
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            ui_callee::NODE_IS_SENTINEL.call(c, 0x0039_f0bc)?;
            if c.g[V0][0] != 0 {
                break;
            }
            c.g[V1][0] = c.lw(oc224_at(c, S0, ui_state::FLAGS));
            if crate::events::back_edge_unwinds(c, PROCESS_STATES_HEAD) {
                return Err(crate::call::Unwound);
            }
        }
        process_states_finish(c)
    })
}

/// 0x39f0c4: v0 = closed ? !opened : 0, restore, return.
fn process_states_finish(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 2;
    let closed = c.g[S6][0] != 0;
    c.g[V0][0] = 0;
    if closed {
        c.cycles += 1;
        c.g[V0][0] = c.g[S7][0] ^ 1;
    }
    c.cycles += 11;
    let frame = Frame { size: 0x90 };
    frame.restore(c, S0, 0x80);
    frame.restore(c, S1, 0x70);
    frame.restore(c, S2, 0x60);
    frame.restore(c, S3, 0x50);
    frame.restore(c, S4, 0x40);
    frame.restore(c, S5, 0x30);
    frame.restore(c, S6, 0x20);
    frame.restore(c, S7, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// Virtual call `vt[slot]` on the state in `obj_reg` (vptr at +8):
/// `a0 = obj + delta` (the other argument registers are already set).
#[inline(always)]
fn ui_vcall(
    c: &mut Cpu,
    obj_reg: usize,
    vt_reg: usize,
    fn_reg: usize,
    slot: u32,
    ret: u32,
) -> crate::call::Flow {
    c.g[vt_reg][0] = c.lw(oc224_at(c, obj_reg, ui_state::VPTR));
    c.g[A0][0] = c.lh(oc224_at(c, vt_reg, slot));
    c.g[fn_reg][0] = c.lw(oc224_at(c, vt_reg, slot + 4));
    let t = r32(c, fn_reg);
    c.g[A0][0] = sx((c.g[obj_reg][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, ret)
}

/// One node (0x39ed00, s0 = node, v1 = its flags); leaves s0 = next.
fn process_states_step(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 5;
    c.g[S5][0] = c.lw(oc224_at(c, S0, ui_state::NEXT));
    c.g[V0][0] = sx((c.g[V1][0] as u32 as i32 >> 5) as u32) & 1;
    c.g[A1][0] = c.g[V1][0];
    if c.g[V0][0] == 0 {
        return step_next(c);
    }
    // The phase, 2..7 through the jump table at 0x493fc0.
    c.cycles += 8;
    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8) & 0x3f;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(2));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
    c.g[A0][0] = sx((c.g[V0][0] as u32 as i32 >> 24) as u32);
    c.g[V1][0] = (c.g[A0][0] < 6) as u64;
    c.g[V0][0] = 0x0049_0000;
    if c.g[V1][0] == 0 {
        return step_next(c);
    }
    c.cycles += 6;
    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3fc0));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = c.lw(r32(c, V1));
    match r32(c, A0) {
        0x0039_ed4c => phase_opening(c),
        0x0039_ee14 => phase_open_input(c),
        0x0039_edc8 => {
            c.cycles += 1;
            c.g[V1][0] = c.lw(oc224_at(c, S0, ui_state::VPTR));
            phase_open_wait(c)
        }
        0x0039_ee78 => {
            // 5: open: vt[0x60](), update screens.
            c.cycles += 9;
            ui_vcall(c, S0, V0, V1, 0x60, 0x0039_ee8c)?;
            c.g[A0][0] = c.g[S0][0];
            ui_callee::UPDATE_SCREENS.call(c, 0x0039_ee94)?;
            c.g[S0][0] = c.g[S5][0];
            Ok(())
        }
        0x0039_ee9c => {
            // 6: closing: vt[0x50]() true -> phase 7; mark shown.
            c.cycles += 7;
            ui_vcall(c, S0, V1, V0, 0x50, 0x0039_eeb0)?;
            let done = c.g[V0][0] != 0;
            c.g[V1][0] = ui_state::PHASE_MASK;
            if done {
                c.cycles += 4;
                c.g[V0][0] = c.lw(oc224_at(c, S0, ui_state::FLAGS));
                c.g[V0][0] &= c.g[V1][0];
                c.g[V0][0] |= 0x700;
                let v = c.g[V0][0];
                c.sw(oc224_at(c, S0, ui_state::FLAGS), v);
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw(oc224_at(c, S0, ui_state::FLAGS));
            c.g[A0][0] = c.g[S0][0];
            c.g[V0][0] |= 0x10;
            store_flags_update(c)
        }
        0x0039_eee4 => phase_closed(c),
        pc => {
            c.jump_out(pc, 0x0039_ecb0, 0x0039_f0fc);
            Err(crate::call::Unwound)
        }
    }
}

/// 0x39f0b0: s0 = next.
fn step_next(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 1;
    c.g[S0][0] = c.g[S5][0];
    Ok(())
}

/// 0x39eed4: store v0 as the flags, update the screens, s0 = next.
fn store_flags_update(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 4;
    let v = c.g[V0][0];
    c.sw(oc224_at(c, S0, ui_state::FLAGS), v);
    ui_callee::UPDATE_SCREENS.call(c, 0x0039_eedc)?;
    c.g[S0][0] = c.g[S5][0];
    Ok(())
}

/// Phase 2 (0x39ed4c): opening: vt[0x20](), phase 4, update screens; a
/// top-level state shows its owner chain and notifies the owner
/// (vt[0x88](state, 0x15)); then as phase 4.
fn phase_opening(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 17;
    c.g[S7][0] = 1;
    ui_vcall(c, S0, V0, V1, 0x20, 0x0039_ed64)?;
    c.g[V0][0] = c.lw(oc224_at(c, S0, ui_state::FLAGS));
    c.g[V1][0] = ui_state::PHASE_MASK;
    c.g[A0][0] = c.g[S0][0];
    c.g[V0][0] &= c.g[V1][0];
    c.g[V0][0] |= 0x400;
    let v = c.g[V0][0];
    c.sw(oc224_at(c, S0, ui_state::FLAGS), v);
    ui_callee::UPDATE_SCREENS.call(c, 0x0039_ed80)?;
    c.g[V0][0] = c.lw(oc224_at(c, S0, ui_state::FLAGS)) & 1;
    if c.g[V0][0] == 0 {
        c.g[V1][0] = c.lw(oc224_at(c, S0, ui_state::VPTR));
        return phase_open_wait(c);
    }
    c.cycles += 3;
    c.g[S1][0] = c.lw(oc224_at(c, S0, ui_state::OWNER));
    c.g[A0][0] = c.g[S2][0];
    if c.g[S1][0] != 0 {
        c.cycles += 11;
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 1;
        c.g[A3][0] = 1;
        ui_callee::SET_PARENT_CHAIN_VISIBLE.call(c, 0x0039_edac)?;
        c.g[V0][0] = c.lw(oc224_at(c, S1, ui_state::VPTR));
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = 0x15;
        c.g[A0][0] = c.lh(oc224_at(c, V0, 0x88));
        c.g[V1][0] = c.lw(oc224_at(c, V0, 0x8c));
        let t = r32(c, V1);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0039_edc8)?;
    }
    c.cycles += 1;
    c.g[V1][0] = c.lw(oc224_at(c, S0, ui_state::VPTR));
    phase_open_wait(c)
}

/// Phase 4 (0x39edcc, v1 = vptr): vt[0x38](input) true -> phase 3; clear
/// the input bit; update screens.
fn phase_open_wait(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 9;
    c.g[A1][0] = c.lw(oc224_at(c, S0, ui_state::FLAGS));
    c.g[A0][0] = c.lh(oc224_at(c, V1, 0x38));
    c.g[A1][0] = sx((c.g[A1][0] as u32 as i32 >> 4) as u32);
    c.g[V0][0] = c.lw(oc224_at(c, V1, 0x3c));
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    let t = r32(c, V0);
    c.g[A1][0] &= 1;
    call_ptr(c, t, 0x0039_ede8)?;
    let done = c.g[V0][0] != 0;
    c.g[V1][0] = ui_state::PHASE_MASK;
    if done {
        c.cycles += 4;
        c.g[V0][0] = c.lw(oc224_at(c, S0, ui_state::FLAGS));
        c.g[V0][0] &= c.g[V1][0];
        c.g[V0][0] |= 0x300;
        let v = c.g[V0][0];
        c.sw(oc224_at(c, S0, ui_state::FLAGS), v);
    }
    c.cycles += 5;
    c.g[V0][0] = c.lw(oc224_at(c, S0, ui_state::FLAGS));
    c.g[V1][0] = 0xffff_ffff_ffff_ffef;
    c.g[A0][0] = c.g[S0][0];
    c.g[V0][0] &= c.g[V1][0];
    store_flags_update(c)
}

/// Phase 3 (0x39ee14, a1 = flags): with input, vt[0x38](1) and clear the
/// input bit; vt[0x30](0); phase 5; update screens.
fn phase_open_input(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[A1][0] as u32 as i32 >> 4) as u32) & 1;
    c.g[A1][0] = 1;
    if c.g[V0][0] != 0 {
        c.cycles += 9;
        ui_vcall(c, S0, V1, V0, 0x38, 0x0039_ee38)?;
        c.g[V1][0] = c.lw(oc224_at(c, S0, ui_state::FLAGS));
        c.g[V0][0] = 0xffff_ffff_ffff_ffef;
        c.g[V1][0] &= c.g[V0][0];
        let v = c.g[V1][0];
        c.sw(oc224_at(c, S0, ui_state::FLAGS), v);
    }
    c.cycles += 12;
    c.g[A1][0] = 0;
    ui_vcall(c, S0, V0, V1, 0x30, 0x0039_ee60)?;
    c.g[V0][0] = c.lw(oc224_at(c, S0, ui_state::FLAGS));
    c.g[V1][0] = ui_state::PHASE_MASK;
    c.g[A0][0] = c.g[S0][0];
    c.g[V0][0] &= c.g[V1][0];
    c.g[V0][0] |= 0x500;
    store_flags_update(c)
}

/// Phase 7 (0x39eee4): closed: vt[0x58](0), vt[0x28](); phase 8, inactive;
/// unlink if remove-on-close; a top-level state hides/re-shows its owner
/// chain (unless another state is still opening) and notifies the owner
/// (vt[0x88](state, 0x16)); a removed state gets vt[0x8](3) (delete), else
/// the shown bit is cleared; when nothing is active any more, restart a
/// state in phase 1 (else 8).
fn phase_closed(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 23;
    c.g[A1][0] = 0;
    c.g[S4][0] = 0;
    c.g[S6][0] = 1;
    ui_vcall(c, S0, V0, V1, 0x58, 0x0039_ef04)?;
    ui_vcall(c, S0, V0, V1, 0x28, 0x0039_ef18)?;
    c.g[V0][0] = c.lw(oc224_at(c, S0, ui_state::FLAGS));
    c.g[V1][0] = ui_state::PHASE_MASK;
    c.g[A0][0] = 0xffff_ffff_ffff_ffdf;
    c.g[V0][0] &= c.g[V1][0];
    c.g[V0][0] |= 0x800;
    c.g[V0][0] &= c.g[A0][0];
    c.g[V1][0] = sx((c.g[V0][0] as u32 as i32 >> 7) as u32) & 1;
    let remove = c.g[V1][0] != 0;
    let v = c.g[V0][0];
    c.sw(oc224_at(c, S0, ui_state::FLAGS), v);
    if remove {
        c.cycles += 2;
        c.g[A0][0] = c.g[S0][0];
        ui_callee::NODE_UNLINK.call(c, 0x0039_ef48)?;
    }
    c.cycles += 4;
    c.g[V0][0] = c.lw(oc224_at(c, S0, ui_state::FLAGS));
    c.g[S3][0] = c.g[V0][0] & 1;
    c.g[V0][0] = sx((c.g[V0][0] as u32 as i32 >> 14) as u32);
    if c.g[S3][0] != 0 {
        c.cycles += 4;
        c.g[S1][0] = c.lw(oc224_at(c, S0, ui_state::OWNER));
        c.g[V0][0] &= 1;
        c.g[S6][0] = 0;
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 1;
            ui_callee::HAS_STATE_IN_PHASE.call(c, 0x0039_ef74)?;
            c.g[S4][0] = c.g[V0][0];
        } else {
            c.cycles += 1;
            c.g[S4][0] = 0;
        }
        c.cycles += 2;
        if c.g[S4][0] == 0 {
            // No state opening: mark hidden-owner, step the owner's count.
            c.cycles += 4;
            c.g[V0][0] = c.lw(oc224_at(c, S0, ui_state::FLAGS)) | 0x80;
            let v = c.g[V0][0];
            c.sw(oc224_at(c, S0, ui_state::FLAGS), v);
            if c.g[S1][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = c.lw(oc224_at(c, S1, ui_state::FLAGS));
                c.g[V0][0] = c.g[A0][0] & 6;
                let counted = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[A0][0] as u32) >> 1);
                if counted {
                    c.cycles += 8;
                    c.g[V1][0] = 0xffff_ffff_ffff_fff9;
                    c.g[V0][0] &= 3;
                    c.g[V1][0] = c.g[A0][0] & c.g[V1][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    c.g[V0][0] &= 3;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                    c.g[V1][0] |= c.g[V0][0];
                    let v = c.g[V1][0];
                    c.sw(oc224_at(c, S1, ui_state::FLAGS), v);
                }
                c.cycles += 17;
                c.g[V1][0] = c.lw(oc224_at(c, S1, ui_state::FLAGS));
                c.g[A0][0] = 0xffff_ffff_ffff_ffdf;
                c.g[A3][0] = c.lw(oc224_at(c, S1, ui_state::VPTR));
                c.g[A1][0] = c.g[S0][0];
                c.g[V0][0] = c.g[V1][0] & 6;
                c.g[A2][0] = 0x16;
                c.g[V0][0] = (c.g[V0][0] < 1) as u64;
                c.g[V1][0] &= c.g[A0][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
                c.g[V1][0] |= c.g[V0][0];
                let v = c.g[V1][0];
                c.sw(oc224_at(c, S1, ui_state::FLAGS), v);
                c.g[A0][0] = c.lh(oc224_at(c, A3, 0x88));
                c.g[V0][0] = c.lw(oc224_at(c, A3, 0x8c));
                let t = r32(c, V0);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0039_f004)?;
            }
        } else {
            // 0x39f00c: another state opening: re-show the owner chain.
            c.cycles += 2;
            c.g[A1][0] = c.g[S1][0];
            if c.g[S1][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = 0;
                c.g[A3][0] = 1;
                ui_callee::SET_PARENT_CHAIN_VISIBLE.call(c, 0x0039_f024)?;
            }
        }
    }
    // 0x39f024
    c.cycles += 2;
    if c.g[S0][0] != 0 {
        c.cycles += 5;
        c.g[V1][0] = c.lw(oc224_at(c, S0, ui_state::FLAGS));
        c.g[V0][0] = sx((c.g[V1][0] as u32 as i32 >> 7) as u32) & 1;
        let remove = c.g[V0][0] != 0;
        c.g[V0][0] = 0xffff_ffff_ffff_fff7;
        if remove {
            c.cycles += 10;
            c.g[A0][0] = c.g[S0][0];
            ui_callee::NODE_UNLINK.call(c, 0x0039_f048)?;
            c.g[A1][0] = 3;
            ui_vcall(c, S0, V0, V1, 0x8, 0x0039_f060)?;
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.g[V1][0] & c.g[V0][0];
            let v = c.g[V0][0];
            c.sw(oc224_at(c, S0, ui_state::FLAGS), v);
        }
    }
    // 0x39f070
    c.cycles += 2;
    if c.g[S3][0] != 0 {
        c.cycles += 2;
        c.g[S0][0] = c.g[S5][0];
        if c.g[S4][0] == 0 {
            return Ok(());
        }
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S2][0];
    ui_callee::GET_ACTIVE_STATE.call(c, 0x0039_f088)?;
    c.g[S0][0] = c.g[S5][0];
    if c.g[V0][0] != 0 {
        return Ok(());
    }
    c.cycles += 5;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = 1;
    ui_callee::RESTART_STATE_IN_PHASE.call(c, 0x0039_f09c)?;
    c.g[S0][0] = c.g[S5][0];
    if c.g[V0][0] != 0 {
        return Ok(());
    }
    c.cycles += 3;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = 8;
    ui_callee::RESTART_STATE_IN_PHASE.call(c, 0x0039_f0b0)?;
    step_next(c)
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_UpdatePeakMusic (0x28e100).

/// The audio system's music fields.
mod audio_sys {
    /// The inline music player (bank monitor) at +0x118.
    pub const PLAYER: u32 = 0x118;
    /// The music bank's mixer object (its methods: +0x10 set gain, +0x20
    /// set volume curve).
    pub const MIXER: u32 = 0x5558;
    /// Peak music needs an update.
    pub const PEAK_DIRTY: u32 = 0x6274;
    /// What to do: 0 start, 2 change peak, 3/other resume, 4 stop.
    pub const PEAK_STATE: u32 = 0x6278;
    /// The pending track id (0x17 = none).
    pub const PEAK_TRACK: u32 = 0x6284;
    pub const NO_TRACK: u64 = 0x17;
}

mod audio_callee {
    use super::Callee;
    use ssx3_ee as ee;
    pub const IS_GAME_MUSIC_ENABLED: Callee = Callee {
        addr: 0x0028_d960,
        f: ee::s07::SSXAudioSystem_IsGameMusicEnabled,
    };
    pub const GET_BE_LIBRARY: Callee = Callee {
        addr: 0x0028_b1e8,
        f: ee::s07::SSXAudioSystem_GetBELibrary,
    };
    pub const GET_CHANNEL_GAIN_PTR: Callee = Callee {
        addr: 0x0028_7968,
        f: ee::s07::SSXAudioSystem_GetChannelGainPtr,
    };
    pub const SELECT_RANDOM_SONG: Callee = Callee {
        addr: 0x0028_d488,
        f: ee::s07::SSXAudioSystem_SelectRandomSong,
    };
    pub const PLAY_MUSIC: Callee = Callee {
        addr: 0x0028_cf98,
        f: ee::s07::SSXAudioSystem_PlayMusic,
    };
    /// `BE_GetInterface(lib, 0)` (s01).
    pub const BE_GET_INTERFACE: u32 = 0x0014_dd58;
    /// `BENewRaceInt_GetTrack` (s01): the current track record.
    pub const RACE_GET_TRACK: u32 = 0x0014_4bc0;
    /// `SSXAudioSystem_TrackToCategory(this, track)` (s08): 1, 2, 4 or 0.
    pub const TRACK_TO_CATEGORY: u32 = 0x002a_1e20;
    /// `SSXAudioSystem_IsTrackGroup14(this, track)` (s08).
    pub const IS_RANDOM_TRACK: u32 = 0x002a_4030;
    /// `MusicPlayer_SetCategory(player, n)` (s08).
    pub const PLAYER_SET_CATEGORY: u32 = 0x002b_49c0;
    /// `MusicPlayer_PathCmdA(player, n)` (s08).
    pub const PLAYER_PATH_CMD: u32 = 0x002b_3c28;
}

/// `void SSXAudioSystem_UpdatePeakMusic(SSXAudioSystem* this)` (0x28e100):
/// when the peak music is dirty, by the peak state: 0 (if game music is
/// on) load the bank for the pending track (or the current race track) and
/// apply its category's gain; 2 (if game music is on and a track is
/// pending) either play a random song (random-song tracks) or load the
/// track's bank, then clear the pending track; 4 stop the music; anything
/// else (if music is on) restart it (3 with path 0x24). Clears the dirty
/// flag.
pub fn ssx_audio_system_update_peak_music(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S4, 0x20);
        frame.save(c, S0, 0x60);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, S4);
        c.g[V0][0] = c.lw(this.wrapping_add(audio_sys::PEAK_DIRTY));
        let dirty = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x60);
        if dirty {
            update_peak_music_body(c, this)?;
            c.cycles += 1;
            frame.restore(c, S0, 0x60);
        }
        c.cycles += 8;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Clear the dirty flag (0x28e4e4).
fn peak_clear_dirty(c: &mut Cpu, this: u32) -> crate::call::Flow {
    c.cycles += 1;
    c.sw(this.wrapping_add(audio_sys::PEAK_DIRTY), 0);
    Ok(())
}

fn update_peak_music_body(c: &mut Cpu, this: u32) -> crate::call::Flow {
    c.cycles += 3;
    c.g[S0][0] = c.lw(this.wrapping_add(audio_sys::PEAK_STATE));
    c.g[V0][0] = 2;
    if c.g[S0][0] == 0 {
        return peak_start(c, this);
    }
    c.cycles += 2;
    let state = c.g[S0][0];
    c.g[V0][0] = 4;
    if state == 2 {
        return peak_change(c, this);
    }
    // 0x28e488: stop (4) or restart.
    c.cycles += 2;
    c.g[A0][0] = c.g[S4][0];
    if state == 4 {
        c.cycles += 2;
        c.g[A1][0] = 0;
    } else {
        c.cycles += 4;
        c.g[A0][0] = c.g[S4][0];
        audio_callee::IS_GAME_MUSIC_ENABLED.call(c, 0x0028_e4a0)?;
        let enabled = c.g[V0][0] != 0;
        c.g[V0][0] = 3;
        if !enabled {
            return peak_clear_dirty(c, this);
        }
        c.cycles += 3;
        c.g[V1][0] = c.lw(this.wrapping_add(audio_sys::PEAK_STATE));
        c.g[A0][0] = c.g[S4][0];
        if c.g[V1][0] != c.g[V0][0] {
            // PlayMusic(this, 0, 0, -1, 0)
            c.cycles += 5;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            c.g[A3][0] = u64::MAX;
            c.g[T0][0] = 0;
            audio_callee::PLAY_MUSIC.call(c, 0x0028_e4e4)?;
            return peak_clear_dirty(c, this);
        }
        c.cycles += 1;
        c.g[A1][0] = 0x24;
    }
    // 0x28e4b8: PlayMusic(this, a1, 0, -1, 0)
    c.cycles += 6;
    c.g[A2][0] = 0;
    c.g[A3][0] = u64::MAX;
    c.g[T0][0] = 0;
    audio_callee::PLAY_MUSIC.call(c, 0x0028_e4c8)?;
    c.sw(this.wrapping_add(audio_sys::PEAK_DIRTY), 0);
    Ok(())
}

/// State 0 (0x28e134): load the bank for the pending (or current race)
/// track.
fn peak_start(c: &mut Cpu, this: u32) -> crate::call::Flow {
    c.cycles += 4;
    audio_callee::IS_GAME_MUSIC_ENABLED.call(c, 0x0028_e144)?;
    let enabled = c.g[V0][0] != 0;
    c.g[S0][0] = audio_sys::NO_TRACK;
    if !enabled {
        return peak_clear_dirty(c, this);
    }
    c.cycles += 3;
    c.g[A1][0] = c.lw(this.wrapping_add(audio_sys::PEAK_TRACK));
    if c.g[A1][0] == c.g[S0][0] {
        // No pending track: the race's track.
        c.cycles += 12;
        audio_callee::GET_BE_LIBRARY.call(c, 0x0028_e160)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, audio_callee::BE_GET_INTERFACE, 0x0028_e16c)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, audio_callee::RACE_GET_TRACK, 0x0028_e174)?;
        c.g[A1][0] = c.lw(r32(c, V0));
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, audio_callee::TRACK_TO_CATEGORY, 0x0028_e180)?;
        c.g[V1][0] = c.g[V0][0];
    } else {
        c.cycles += 4;
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, audio_callee::TRACK_TO_CATEGORY, 0x0028_e190)?;
        let v = c.g[S0][0];
        c.sw(this.wrapping_add(audio_sys::PEAK_TRACK), v);
        c.g[V1][0] = c.g[V0][0];
    }
    // 0x28e198: by category (v1).
    c.cycles += 3;
    c.g[V0][0] = 2;
    if c.g[V1][0] == 1 {
        c.cycles += 23;
        peak_bank_gain(c, this, 1, [0x0028_e1d0, 0x0028_e1ec, 0x0028_e1f8])?;
    } else {
        c.cycles += 2;
        if c.g[V1][0] == c.g[V0][0] {
            c.cycles += 22;
            peak_bank_gain(c, this, 2, [0x0028_e230, 0x0028_e24c, 0x0028_e258])?;
        } else {
            c.cycles += 21;
            peak_bank_gain(c, this, 3, [0x0028_e284, 0x0028_e2a4, 0x0028_e2b0])?;
        }
    }
    c.cycles += 10;
    peak_bank_curve(c, [0x0028_e2c8, 0x0028_e2d4])?;
    c.sw(this.wrapping_add(audio_sys::PEAK_DIRTY), 0);
    Ok(())
}

/// State 2 (0x28e2dc): change to the pending peak track.
fn peak_change(c: &mut Cpu, this: u32) -> crate::call::Flow {
    c.cycles += 4;
    c.g[A0][0] = c.g[S4][0];
    audio_callee::IS_GAME_MUSIC_ENABLED.call(c, 0x0028_e2ec)?;
    let enabled = c.g[V0][0] != 0;
    c.g[V0][0] = audio_sys::NO_TRACK;
    if !enabled {
        return peak_clear_dirty(c, this);
    }
    c.cycles += 3;
    c.g[A1][0] = c.lw(this.wrapping_add(audio_sys::PEAK_TRACK));
    if c.g[A1][0] == c.g[V0][0] {
        c.sw(this.wrapping_add(audio_sys::PEAK_DIRTY), 0);
        return Ok(());
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S4][0];
    call_ptr(c, audio_callee::IS_RANDOM_TRACK, 0x0028_e308)?;
    if c.g[V0][0] != 0 {
        // A random song: SelectRandomSong, PlayMusic(this, 0x24, 1, -1, 0).
        c.cycles += 10;
        c.g[A0][0] = c.g[S4][0];
        audio_callee::SELECT_RANDOM_SONG.call(c, 0x0028_e318)?;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = 0x24;
        c.g[A2][0] = 1;
        c.g[A3][0] = u64::MAX;
        c.g[T0][0] = 0;
        audio_callee::PLAY_MUSIC.call(c, 0x0028_e330)?;
        c.g[V0][0] = audio_sys::NO_TRACK;
    } else {
        // Load the track's bank (0x28e338).
        c.g[A1][0] = c.lw(this.wrapping_add(audio_sys::PEAK_TRACK));
        c.cycles += 6;
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, audio_callee::TRACK_TO_CATEGORY, 0x0028_e340)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = 1;
        c.g[S2][0] = sx(this.wrapping_add(audio_sys::PLAYER));
        if c.g[V1][0] == 1 {
            c.cycles += 22;
            peak_bank_gain(c, this, 1, [0x0028_e378, 0x0028_e394, 0x0028_e3a0])?;
        } else {
            c.cycles += 2;
            if c.g[V1][0] == c.g[S0][0] {
                c.cycles += 22;
                peak_bank_gain(c, this, 2, [0x0028_e3d8, 0x0028_e3f4, 0x0028_e400])?;
            } else {
                c.cycles += 21;
                peak_bank_gain(c, this, 3, [0x0028_e42c, 0x0028_e44c, 0x0028_e458])?;
            }
        }
        c.cycles += 9;
        peak_bank_curve(c, [0x0028_e470, 0x0028_e47c])?;
        c.g[V0][0] = audio_sys::NO_TRACK;
    }
    // 0x28e480: no pending track any more.
    c.cycles += 2;
    let v = c.g[V0][0];
    c.sw(this.wrapping_add(audio_sys::PEAK_TRACK), v);
    peak_clear_dirty(c, this)
}

/// Apply category `kind`'s gain: `mixer->vt[0x10](player + d, &table[kind],
/// 0xc, GetChannelGainPtr(this, 1, 0), 0)`, then
/// `MusicPlayer_SetCategory(player, kind)`; v1 = the mixer. The third
/// variant sets s5 after the first call (as compiled).
fn peak_bank_gain(c: &mut Cpu, this: u32, kind: u32, rets: [u32; 3]) -> crate::call::Flow {
    c.g[S3][0] = c.lw(this.wrapping_add(audio_sys::MIXER));
    c.g[S2][0] = sx(this.wrapping_add(audio_sys::PLAYER));
    c.g[A0][0] = c.g[S4][0];
    c.g[S0][0] = c.lh(oc224_at(c, S3, 0x10));
    c.g[S1][0] = sx(0x004a_3630 + 8 * (kind - 1));
    c.g[A1][0] = 1;
    c.g[A2][0] = 0;
    c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
    if kind != 3 {
        c.g[S5][0] = c.g[S2][0];
    }
    audio_callee::GET_CHANNEL_GAIN_PTR.call(c, rets[0])?;
    if kind == 3 {
        c.g[S5][0] = c.g[S2][0];
    }
    c.g[V1][0] = c.lw(oc224_at(c, S3, 0x14));
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    c.g[A3][0] = c.g[V0][0];
    c.g[A2][0] = 0xc;
    let t = r32(c, V1);
    c.g[T0][0] = 0;
    call_ptr(c, t, rets[1])?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = kind as u64;
    call_ptr(c, audio_callee::PLAYER_SET_CATEGORY, rets[2])?;
    c.g[V1][0] = c.lw(this.wrapping_add(audio_sys::MIXER));
    Ok(())
}

/// `mixer(v1)->vt[0x20](player(s5) + d, 0xc)`, then
/// `MusicPlayer_PathCmdA(player, 0x7f)`.
fn peak_bank_curve(c: &mut Cpu, rets: [u32; 2]) -> crate::call::Flow {
    c.g[A1][0] = 0xc;
    c.g[A0][0] = c.lh(oc224_at(c, V1, 0x20));
    c.g[V0][0] = c.lw(oc224_at(c, V1, 0x24));
    let t = r32(c, V0);
    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, rets[0])?;
    c.g[A0][0] = c.g[S5][0];
    c.g[A1][0] = 0x7f;
    call_ptr(c, audio_callee::PLAYER_PATH_CMD, rets[1])
}

// ---------------------------------------------------------------------------
// LensFX_DrawSun (0x2f4a08).

mod lens_fx {
    /// Non-zero while the sun is on screen (halfword).
    pub const VISIBLE: u32 = 0x2c;
    /// The sun's screen position (x, y).
    pub const SUN_X: u32 = 0x1f0;
    pub const SUN_Y: u32 = 0x1f4;
    /// The flare intensity.
    pub const INTENSITY: u32 = 0x204;
}

mod sun_callee {
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
fn draw_sun_body(c: &mut Cpu, fx: u32) -> Result<bool, crate::call::Unwound> {
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{fill, Rng, DATA, GAME_GP};

    /// `gp + off` on the test machine.
    fn gpa(off: i32) -> u32 {
        GAME_GP.wrapping_add(off as u32)
    }

    /// A guest function that returns `*(u32*)a0` (`lw v0, 0(a0); jr ra`):
    /// a real vtable target for results a fake cannot give (negative
    /// 64-bit values).
    const GET_WORD: u32 = 0x0027_43c0;

    // ---------------------------------------------------- LoadIndicator_Draw

    const LI: u32 = DATA;
    const LI_FONT: u32 = DATA + 0x100;
    const LI_TEXT: u32 = DATA + 0x200;
    const REND: u32 = DATA + 0x1000;
    const REND_STATE: u32 = DATA + 0x3000;
    const REND_VT: u32 = DATA + 0x4000;

    /// Renderer methods the indicator (and `Font_DrawTextW`) call.
    const LI_SLOTS: [u32; 11] = [
        0xa8, 0xf8, 0x110, 0x50, 0x58, 0xb8, 0xd0, 0x270, 0x100, 0xb0, 0x288,
    ];

    /// The indicator at `LI` (busy most seeds, an empty wide string, a
    /// random font), the renderer at `gp-0x854` (and `gp+0x2a90`, the one
    /// the font code uses) with a random draw-state stack and a vtable of
    /// fakes. Some seeds route the width/height getters to a real getter
    /// returning negative values (the unsigned-conversion branches).
    fn load_indicator_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x001f_31e0, 12);
        c.bus.write32(gpa(-0x854), REND);
        c.bus.write32(gpa(0x2a90), REND);
        c.bus.write32(REND + 0xe84, REND_STATE + 0x14 * (seed % 3));
        fill(c, &mut r, REND_STATE, 0x30);
        c.bus.write32(REND + 0x1068, r.next_u32());
        c.bus.write32(REND + 0x10d8, REND_VT);
        for (i, &off) in LI_SLOTS.iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 & !3 & 0xfff) as u32;
            c.bus.write16(REND_VT + off, delta as u16);
            c.bus.write32(REND_VT + off + 4, FAKE + 0x10 * i as u32);
        }
        if seed % 4 == 1 {
            // width = *(REND+0), height = *(REND+4), both maybe negative
            c.bus.write16(REND_VT + 0x50, 0);
            c.bus.write32(REND_VT + 0x54, GET_WORD);
            c.bus.write16(REND_VT + 0x58, 4);
            c.bus.write32(REND_VT + 0x5c, GET_WORD);
            c.bus
                .write32(REND, if seed % 8 == 1 { 0x8000_0281 } else { 640 });
            c.bus
                .write32(REND + 4, if seed % 16 < 8 { 0xffff_fff1 } else { 448 });
        }
        // The indicator.
        c.bus.write32(LI + 0x0, LI_TEXT);
        c.bus.write16(LI_TEXT, 0);
        c.bus
            .write32(LI + 0x4, if seed % 5 == 0 { 0 } else { 1 + seed });
        c.bus.write32(LI + 0x8, r.f32(2.0));
        c.bus.write32(LI + 0xc, LI_FONT);
        fill(c, &mut r, LI_FONT, 0x20);
        for off in [0x18u32, 0x1c, 0x20, 0x24, 0x30, 0x34] {
            c.bus.write32(LI_FONT + off, r.f32(4.0));
        }
        c.bus.write32(LI_FONT + 0x14, r.next_u32() % 4);
        // The text scale (first use sets it to 3) and the .lit4 constants.
        c.bus.write32(gpa(0x1dc0), seed % 2);
        c.bus.write32(gpa(0x1db8), r.f32(3.0));
        c.bus.write32(gpa(0x1dbc), r.f32(3.0));
        for off in [-0x5558, -0x5554, -0x5550] {
            c.bus.write32(gpa(off), r.f32(500.0));
        }
        c.g[A0][0] = LI as u64;
        c.pc = 0x001f_31e0;
    }

    #[test]
    fn load_indicator_draw_matches() {
        covered(
            ee::s04::f_001f31e0__generated,
            load_indicator_draw,
            200,
            &load_indicator_setup,
        );
    }

    // ----------------------------------------------- UIStateMgr_ProcessStates

    /// The manager (state list at +0).
    const UI_MGR: u32 = DATA;
    /// A shared vtable: every slot 0..0x100 a distinct fake.
    const UI_VT: u32 = DATA + 0xf000;
    /// Owner states (not in the list) at DATA+0x1000.
    const UI_OWNERS: u32 = DATA + 0x1000;

    /// Link `states` into the list at `list` (anchor link at +4), ending at
    /// the self-linked `sentinel`. Sometimes an empty list is a null link.
    fn ui_list(c: &mut Cpu, r: &mut Rng, list: u32, states: &[u32], sentinel: u32) {
        if states.is_empty() && r.next_u32() % 4 == 0 {
            c.bus.write32(list + 4, 0);
            return;
        }
        let mut prev = list;
        for &s in states {
            c.bus.write32(prev + 4, s);
            c.bus.write32(s, prev);
            prev = s;
        }
        c.bus.write32(prev + 4, sentinel);
        c.bus.write32(sentinel, prev);
        c.bus.write32(sentinel + 4, sentinel);
    }

    /// A random state at `s`: vptr = [`UI_VT`], flags with a phase in 0..8
    /// and random bits, no screens, owner `owner`.
    fn ui_state(c: &mut Cpu, r: &mut Rng, s: u32, owner: u32) {
        c.bus.write32(s + 8, UI_VT);
        c.bus.write32(s + 0xc, r.next_u32() % 6);
        let phase = 2 + r.next_u32() % 7;
        c.bus
            .write32(s + 0x1c, (r.next_u32() & !0x3f00) | phase << 8);
        c.bus.write32(s + 0x20, owner);
        // An empty screen list (+0x24) for UIState_UpdateScreens.
        ui_list(c, r, s + 0x24, &[], s + 0x30);
    }

    /// 0..6 states in the manager's list (0x40 apart from DATA+0x100),
    /// half of them owned by a state outside the list, the shared vtable,
    /// and the phase jump table (.rodata 0x493fc0).
    fn process_states_setup(c: &mut Cpu, seed: u32, calls: u32) -> Rng {
        let mut r = machine(c, seed, 0x0039_ecb0, calls);
        for off in (0..0x100).step_by(8) {
            c.bus.write16(UI_VT + off, ((r.next_u32() % 16) * 4) as u16);
            c.bus.write32(UI_VT + off + 4, FAKE + 2 * off);
        }
        crate::difftest::put_words(
            c,
            0x0049_3fc0,
            &[
                0x0039_ed4c,
                0x0039_ee14,
                0x0039_edc8,
                0x0039_ee78,
                0x0039_ee9c,
                0x0039_eee4,
            ],
        );
        let n = r.next_u32() % 7;
        let states: Vec<u32> = (0..n).map(|i| DATA + 0x100 + 0x40 * i).collect();
        for (i, &s) in states.iter().enumerate() {
            let owner = if r.next_u32() % 2 == 0 {
                0
            } else {
                let o = UI_OWNERS + 0x40 * i as u32;
                ui_state(c, &mut r, o, 0);
                o
            };
            ui_state(c, &mut r, s, owner);
        }
        ui_list(c, &mut r, UI_MGR, &states, DATA + 0x6c0);
        // The manager's second list (+0x1c) is empty.
        ui_list(c, &mut r, UI_MGR + 0x1c, &[], DATA + 0xec0);
        c.g[A0][0] = UI_MGR as u64;
        c.pc = 0x0039_ecb0;
        r
    }

    #[test]
    fn ui_state_mgr_process_states_matches() {
        use crate::difftest::events::{check_events, schedule};
        let (generated, hand): (GuestFn, GuestFn) =
            (ee::s13::f_0039ecb0__generated, ui_state_mgr_process_states);
        covered(generated, hand, 300, &|c, seed| {
            process_states_setup(c, seed, 8);
        });
        check_events(generated, hand, 300, &|c, seed| {
            let mut r = process_states_setup(c, seed, 0);
            schedule(c, &mut r, seed, 0x0039_ecb0)
        });
    }

    // ------------------------------------------ SSXAudioSystem_UpdatePeakMusic

    const AUDIO: u32 = DATA;
    const AUDIO_WORLD: u32 = DATA + 0xc000;
    const AUDIO_MIXER: u32 = DATA + 0xc100;
    const AUDIO_IFACE: u32 = DATA + 0xc200;

    /// `SSXAudioSystem_TrackToCategory`'s jump table (.rodata 0x483010):
    /// the 22 tracks' categories (1, 2 or 4).
    const TRACK_CATEGORY_JUMPS: [u32; 22] = {
        const A: u32 = 0x002a_1e44;
        const B: u32 = 0x002a_1e4c;
        const C: u32 = 0x002a_1e54;
        [
            A, A, B, B, C, A, B, C, A, B, C, A, B, C, A, B, C, A, A, B, B, C,
        ]
    };

    /// The audio system at `AUDIO` with every peak state, music on/off and
    /// pending tracks (none, plain, random-song); the world (`gp-0x848`)
    /// says a replay is playing so `PlayMusic` returns at once; the race's
    /// track (via the BE interface, a fake) at 0x43d950; a mixer of fakes;
    /// the music player fixture `SelectRandomSong` needs (a two-slot bank
    /// with slot 1 occupied, as in `RestartPeakMusic`'s test).
    fn update_peak_music_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x0028_e100, 4);
        let a = AUDIO;
        c.bus.write32(gpa(-0x1f04), DATA + 0xc300);
        c.bus.write32(gpa(-0x848), AUDIO_WORLD);
        c.bus.write32(AUDIO_WORLD + 0x78, DATA + 0xc300);
        c.bus.write32(AUDIO_WORLD + 0x84, AUDIO_WORLD + 0x40);
        c.bus.write32(AUDIO_WORLD + 0x40 + 0x28, AUDIO_WORLD + 0x80);
        c.bus.write32(AUDIO_WORLD + 0x80, 1);
        // BE_GetInterface(lib, 0): gp-0x1efc indexes the table at 0x5308a8.
        c.bus.write32(gpa(-0x1efc), 0);
        c.bus.write32(0x0053_08a8, AUDIO_IFACE);
        c.bus.write32(AUDIO_IFACE + 4, AUDIO_IFACE + 0x10);
        c.bus.write16(AUDIO_IFACE + 0x10 + 8, 0);
        c.bus.write32(AUDIO_IFACE + 0x10 + 0xc, FAKE + 0x100);
        c.bus.write32(0x0053_5c08, 0);
        c.bus.write32(0x0043_d950, r.next_u32() % 0x18);
        crate::difftest::put_words(c, 0x0048_3010, &TRACK_CATEGORY_JUMPS);
        // The mixer (methods +0x10, +0x20).
        c.bus.write32(a + 0x5558, AUDIO_MIXER);
        for (i, off) in [0x10u32, 0x20].into_iter().enumerate() {
            c.bus
                .write16(AUDIO_MIXER + off, ((r.next_u32() % 16) * 4) as u16);
            c.bus.write32(AUDIO_MIXER + off + 4, FAKE + 0x10 * i as u32);
        }
        // The music player (inline at +0x118).
        c.bus.write32(a + 0x118, DATA + 0xa000);
        c.bus.write32(DATA + 0xa000, DATA + 0xa100);
        c.bus.write32(DATA + 0xa100 + 0x1d8, 0);
        c.bus.write32(a + 0x504, 2);
        c.bus.write32(a + 0x4fc, 2);
        c.bus.write32(a + 0x508, 0);
        c.bus.write64(a + 0x510, 2);
        c.bus.write32(a + 0x62a4, r.next_u32() % 2);
        // The peak music request.
        c.bus.write32(a + 0x6274, if seed % 7 == 0 { 0 } else { 1 });
        c.bus.write32(
            a + 0x6278,
            [0u32, 2, 3, 4, 1, 0, 2][(seed / 7) as usize % 7],
        );
        c.bus
            .write32(a + 0x608c, [3u32, 0, 2, 3, 4][(seed / 2) as usize % 5]);
        let track = match r.next_u32() % 4 {
            0 => 0x17,
            1 => 0xe + r.next_u32() % 3,
            _ => r.next_u32() % 0x18,
        };
        c.bus.write32(a + 0x6284, track);
        c.bus.write32(a + 0x6294, 0);
        c.g[A0][0] = a as u64;
        c.pc = 0x0028_e100;
    }

    #[test]
    fn ssx_audio_system_update_peak_music_matches() {
        covered(
            ee::s07::f_0028e100__generated,
            ssx_audio_system_update_peak_music,
            300,
            &update_peak_music_setup,
        );
    }

    // ------------------------------------------------------- LensFX_DrawSun

    const SUN_FX: u32 = DATA;
    const SUN_VP: u32 = DATA + 0x300;
    const SUN_HOLDER: u32 = DATA + 0x400;
    const SUN_VT: u32 = DATA + 0x600;
    const SUN_REND: u32 = DATA + 0x1000;
    const SUN_REND_VT: u32 = DATA + 0x3000;
    const SUN_REND_STATE: u32 = DATA + 0x3400;
    const SUN_RESULTS: u32 = DATA + 0x3800;

    /// The lens FX at `SUN_FX` (visible most seeds, no flares), a viewport,
    /// the world painter's sun for views 6 and 7 (methods are fakes whose
    /// forced results point at the colour, intensity, size and texture id
    /// at `SUN_RESULTS`: the alpha is positive or not, the size sometimes
    /// 1), the renderer (`gp+0x2a90` and `gp-0x854`) with a vtable of fakes.
    fn draw_sun_setup(c: &mut Cpu, seed: u32) {
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
        covered(ee::s09::f_002f4a08__generated, lens_fx_draw_sun, 200, &draw_sun_setup);
    }
}
