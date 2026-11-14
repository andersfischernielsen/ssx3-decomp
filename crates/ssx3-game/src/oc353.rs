//! Staging module oc353 (wave wave216): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc353`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::game::fe::ovstatehud::{ld_d, sd_d};
use crate::game::render::particle::{
    add32, at, eq, fconst, fr, gp, ldf, le, p, setf, shl32, sra32,
};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0021_e1b0,
    hud_draw_rider_indicator,
    "HUD_DrawRiderIndicator",
)];

/// The HUD element table (0x24 bytes per element) the rider indicator
/// uses: element `n` (the rider's indicator style) for the arrow, 6 for
/// the side piece, 7 the marker, 8 the badge.
pub(crate) const HUD_RIDER_INDICATOR_ELEMS: u32 = 0x0047_7f30;

/// The 512-entry sine table the bounce of style 0 reads.
pub(crate) const HUD_RIDER_INDICATOR_SINE: u32 = 0x0050_4fb8;

/// The badge texture number (1-based index into the HUD's textures at +0x444).
pub(crate) const HUD_RIDER_INDICATOR_BADGE: u32 = 0x0053_5bf8;

/// `HUD_DrawStretchedPanel` (0x0021e7e0), a direct call in the original.
pub(crate) const HUD_RIDER_INDICATOR_PANEL: Callee = Callee {
    addr: 0x0021_e7e0,
    f: ee::s05::f_0021e7e0,
};

/// `void HUD_DrawRiderIndicator(HUD *hud, HudObj *obj, Rider *rider, int
/// align)` (0x21e1b0): the indicator over another rider. Looks up the
/// rider's style (`world->race->slots[rider->index]->style`, +0xec), takes
/// that style's element colour, and while the rider's timer (+0x48) is
/// running either bounces the arrow (style 0: a damped sine) or fades its
/// red. Draws the marker (element 7) and the badge (element 8, texture
/// from the global badge number), then the arrow (`style`) and its side
/// piece (element 6, style clamped to 3), scaled by the bounce. While the
/// pulse (+0x4c) is non-negative, both are drawn again as stretched,
/// pulsing panels one row lower.
pub fn hud_draw_rider_indicator(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 46;
        let frame = Frame::open(c, 0x1c0);
        let sp = sp(c);
        let one = fconst(c, 0, 0x3f80_0000);
        let v = c.g[A2][0];
        c.sw(p(sp, 0xe0), v);
        c.g[T0][0] = 0x24;
        frame.save(c, S5, 0x140);
        c.g[A2][0] = 0x0047_0000;
        frame.save(c, S6, 0x130);
        c.g[S5][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x1a0);
        c.g[A2][0] = sx(HUD_RIDER_INDICATOR_ELEMS);
        // Unit scale (sp+0) and bounce factor k = 1 (f20).
        c.swc1(0, sp);
        c.g[S6][0] = c.g[A1][0];
        c.swc1(0, p(sp, 4));
        setf(c, 20, one);
        frame.save(c, S0, 0x190);
        c.g[A1][0] = sx(p(sp, 0x10));
        frame.save(c, S1, 0x180);
        frame.save(c, S2, 0x170);
        frame.save(c, S3, 0x160);
        frame.save(c, S4, 0x150);
        frame.save(c, S7, 0x120);
        frame.save(c, FP, 0x110);
        frame.save_ra(c, 0x100);
        frame.save_fpr(c, 22, 0x1b0);
        frame.save_fpr(c, 21, 0x1a8);
        // fp = the rider's race slot; colour of element `style` to sp+0x10.
        c.g[V0][0] = c.lw(gp(c, -0x848));
        c.g[T1][0] = c.lw(p(sp, 0xe0));
        c.g[A0][0] = c.lw(at(c, V0, 0x84));
        c.g[V1][0] = c.lw(r32(c, T1));
        c.g[V0][0] = c.lw(at(c, A0, 0xc));
        let v = c.g[A3][0];
        c.sw(p(sp, 0xe4), v);
        c.g[V1][0] = shl32(c.g[V1][0], 2);
        c.g[V0][0] = add32(c.g[V0][0], c.g[V1][0]);
        c.g[FP][0] = c.lw(at(c, V0, 0x28));
        c.g[A0][0] = c.lw(at(c, FP, 0xec));
        c.mult(0, A0, A0, T0);
        c.g[A0][0] = add32(c.g[A0][0], c.g[A2][0]);
        call_ptr(c, 0x001e_91a8, 0x0021_e24c)?;
        // The timer: style 0 bounces (k), the others fade the red.
        c.g[T2][0] = c.lw(p(sp, 0xe0));
        setf(c, 0, FpuF32::ZERO);
        let t = ldf(c, 4, at(c, T2, 0x48));
        let running = le(c, FpuF32::ZERO, t);
        if running {
            c.cycles += 3;
            c.g[V0][0] = c.lw(at(c, FP, 0xec));
            let red = ldf(c, 1, p(sp, 0x10));
            if c.g[V0][0] == 0 {
                c.cycles += 19;
                let k_angle = ldf(c, 0, gp(c, -0x5368));
                c.g[V1][0] = sx(HUD_RIDER_INDICATOR_SINE);
                let k_index = ldf(c, 2, gp(c, -0x5364));
                let a = t * k_angle;
                setf(c, 0, a);
                let amp = ldf(c, 3, gp(c, -0x5360));
                let tt = t * t;
                setf(c, 1, tt);
                let a = a * k_index;
                setf(c, 0, a);
                let fall = fr(c, 20) - tt;
                setf(c, 1, fall);
                let i = a.to_i32() as u32;
                c.f[2] = i;
                c.g[V0][0] = sx(i);
                c.g[V0][0] &= 0x1ff;
                c.g[V0][0] = shl32(c.g[V0][0], 2);
                c.g[V0][0] = add32(c.g[V0][0], c.g[V1][0]);
                let s = ldf(c, 0, r32(c, V0));
                let s = s * amp;
                setf(c, 0, s);
                let s = s * fall;
                setf(c, 0, s);
                let k = s + fr(c, 20);
                setf(c, 20, k);
            } else {
                c.cycles += 6;
                let half = fconst(c, 2, 0x3f00_0000);
                let m = t * half;
                setf(c, 0, m);
                let m = m + half;
                setf(c, 0, m);
                let red = red * m;
                setf(c, 1, red);
                c.swc1(1, p(sp, 0x10));
            }
            c.cycles += 1;
        }
        c.g[V0][0] = sx(p(sp, 0x20));

        c.cycles += 141;
        // Element scale (sp+0x20) and position (sp+0x30) of the object.
        c.g[S0][0] = add32(c.g[S6][0], 0x360);
        let v = c.g[V0][0];
        c.sw(p(sp, 0xe8), v);
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[S3][0] = add32(c.g[S5][0], 0x444);
        call_ptr(c, 0x001e_9290, 0x0021_e2f4)?;
        fconst(c, 22, 0x3f80_0000);
        c.g[S4][0] = sx(p(sp, 0x30));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S4][0];
        call_ptr(c, 0x001e_91f8, 0x0021_e30c)?;
        // size1 (sp+0x40): element 6, scaled.
        c.g[S0][0] = sx(HUD_RIDER_INDICATOR_ELEMS + 0xd8);
        c.g[V1][0] = sx(p(sp, 0x40));
        let v = c.g[V1][0];
        c.sw(p(sp, 0xec), v);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V1][0];
        c.g[A2][0] = 0;
        call_ptr(c, 0x001e_9220, 0x0021_e32c)?;
        let w1 = ldf(c, 0, p(sp, 0x40));
        c.g[A0][0] = sx(p(sp, 0x50));
        let h1 = ldf(c, 1, p(sp, 0x44));
        c.g[V1][0] = sx(r32(c, S0).wrapping_sub(0xd8));
        let scale_x = ldf(c, 2, p(sp, 0x20));
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x48));
        let scale_y = ldf(c, 3, p(sp, 0x24));
        let w1 = w1 * scale_x;
        setf(c, 0, w1);
        let v = c.g[A0][0];
        c.sw(p(sp, 0xf0), v);
        let h1 = h1 * scale_y;
        setf(c, 1, h1);
        c.g[A0][0] = 0x24;
        c.g[A1][0] = c.lw(p(sp, 0xf0));
        c.swc1(0, p(sp, 0x40));
        c.swc1(1, p(sp, 0x44));
        // size2 (sp+0x50): element `style` with its texture.
        c.g[V0][0] = c.lw(at(c, FP, 0xec));
        c.mult(0, A0, V0, A0);
        c.g[V0][0] = shl32(c.g[V0][0], 2);
        c.g[V0][0] = add32(c.g[S3][0], c.g[V0][0]);
        c.g[A2][0] = c.lw(r32(c, V0));
        c.g[A0][0] = add32(c.g[A0][0], c.g[V1][0]);
        call_ptr(c, 0x001e_9220, 0x0021_e380)?;
        let w2 = ldf(c, 1, p(sp, 0x50));
        c.g[A1][0] = sx(p(sp, 0x60));
        let scale_x = ldf(c, 0, p(sp, 0x20));
        c.g[S7][0] = sx(p(sp, 0x70));
        let h2 = ldf(c, 2, p(sp, 0x54));
        c.g[A3][0] = c.g[A1][0];
        let w2 = w2 * scale_x;
        setf(c, 1, w2);
        let scale_y = ldf(c, 3, p(sp, 0x24));
        let w1 = ldf(c, 0, p(sp, 0x40));
        c.g[T0][0] = c.g[SP][0];
        let v = c.g[A1][0];
        c.sw(p(sp, 0xf4), v);
        let h2 = h2 * scale_y;
        setf(c, 2, h2);
        let dim_w = w2 + w1;
        setf(c, 0, dim_w);
        // Align the whole indicator (dim at sp+0x60) to p (sp+0x70).
        c.g[T3][0] = c.lw(p(sp, 0xe4));
        c.g[T2][0] = c.lb(at(c, S6, 0x381));
        c.g[A2][0] = c.g[S4][0];
        c.g[T1][0] = c.lb(at(c, S6, 0x380));
        c.g[A0][0] = c.g[S5][0];
        c.swc1(1, p(sp, 0x50));
        c.g[A1][0] = c.g[S7][0];
        c.swc1(2, p(sp, 0x64));
        c.swc1(0, p(sp, 0x60));
        c.swc1(2, p(sp, 0x54));
        call_ptr(c, 0x001f_10f8, 0x0021_e3e0)?;
        // p += (w2, h1); draw row = obj row; marker colour (element 7).
        let px = ldf(c, 0, p(sp, 0x70));
        c.g[S2][0] = sx(p(sp, 0x80));
        let py = ldf(c, 1, p(sp, 0x74));
        c.g[A0][0] = sx(HUD_RIDER_INDICATOR_ELEMS + 0xfc);
        let w2 = ldf(c, 2, p(sp, 0x50));
        c.g[A1][0] = c.g[S2][0];
        let h1 = ldf(c, 3, p(sp, 0x44));
        let px = px + w2;
        setf(c, 0, px);
        c.g[V1][0] = c.lb(at(c, S6, 0x382));
        let py = py + h1;
        setf(c, 1, py);
        c.g[A3][0] = c.lw(gp(c, -0x854));
        c.g[V1][0] = shl32(c.g[V1][0], 5);
        c.swc1(0, p(sp, 0x70));
        c.g[V1][0] &= 0x3e0;
        c.swc1(1, p(sp, 0x74));
        c.g[A2][0] = c.lw(at(c, A3, 0xe84));
        c.g[A3][0] = 0xffff_ffff_ffff_fc1f;
        c.g[V0][0] = c.lw(at(c, A2, 8));
        c.g[V0][0] &= c.g[A3][0];
        c.g[V0][0] |= c.g[V1][0];
        let v = c.g[V0][0];
        c.sw(at(c, A2, 8), v);
        call_ptr(c, 0x001e_91a8, 0x0021_e43c)?;
        // The marker: element 7's size (sp+0x90), scaled, texture +0x46c.
        c.g[S1][0] = sx(p(sp, 0x90));
        c.g[A0][0] = sx(HUD_RIDER_INDICATOR_ELEMS + 0xfc);
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 0;
        call_ptr(c, 0x001e_9220, 0x0021_e454)?;
        let w3 = ldf(c, 1, p(sp, 0x90));
        c.g[T0][0] = c.g[SP][0];
        let h3 = ldf(c, 0, p(sp, 0x94));
        c.g[A3][0] = c.g[S1][0];
        let scale_x = ldf(c, 2, p(sp, 0x20));
        c.g[T1][0] = c.g[S2][0];
        let scale_y = ldf(c, 3, p(sp, 0x24));
        c.g[A2][0] = c.g[S7][0];
        let w3 = w3 * scale_x;
        setf(c, 1, w3);
        c.g[A1][0] = c.lw(at(c, S5, 0x46c));
        let h3 = h3 * scale_y;
        setf(c, 0, h3);
        c.g[A0][0] = c.g[S5][0];
        c.swc1(1, p(sp, 0x90));
        c.swc1(0, p(sp, 0x94));
        call_ptr(c, 0x001f_1190, 0x0021_e490)?;
        // The badge: p.x += w3, element 8's colour and size.
        let px = ldf(c, 0, p(sp, 0x70));
        c.g[A0][0] = c.g[S0][0];
        let w3 = ldf(c, 1, p(sp, 0x90));
        c.g[A1][0] = c.g[S2][0];
        let px = px + w3;
        setf(c, 0, px);
        c.swc1(0, p(sp, 0x70));
        call_ptr(c, 0x001e_91a8, 0x0021_e4ac)?;
        c.g[A2][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x001e_9220, 0x0021_e4bc)?;
        call_ptr(c, 0x0014_dc80, 0x0021_e4c4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0021_e4d0)?;
        c.g[V1][0] = 0x0053_0000;
        c.g[T0][0] = c.lw(p(sp, 0xe8));
        c.g[V0][0] = c.lw(HUD_RIDER_INDICATOR_BADGE);
        c.g[A3][0] = c.g[S1][0];
        c.g[T1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S5][0];
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        c.g[A2][0] = c.g[S7][0];
        c.g[V0][0] = shl32(c.g[V0][0], 2);
        c.g[V0][0] = add32(c.g[S3][0], c.g[V0][0]);
        c.g[A1][0] = c.lw(r32(c, V0));
        call_ptr(c, 0x001f_1190, 0x0021_e500)?;
        // Scale the arrow sizes by the bounce.
        let k = fr(c, 20);
        let unit = fr(c, 22);
        if !eq(c, k, unit) {
            c.cycles += 19;
            let w2 = ldf(c, 2, p(sp, 0x50));
            let h2 = ldf(c, 4, p(sp, 0x54));
            let w1 = ldf(c, 5, p(sp, 0x40));
            setf(c, 2, w2 * k);
            let h1 = ldf(c, 3, p(sp, 0x44));
            setf(c, 4, h2 * k);
            let dim_w = ldf(c, 1, p(sp, 0x60));
            setf(c, 5, w1 * k);
            let dim_h = ldf(c, 0, p(sp, 0x64));
            setf(c, 3, h1 * k);
            setf(c, 1, dim_w * k);
            c.swc1(2, p(sp, 0x50));
            setf(c, 0, dim_h * k);
            c.swc1(4, p(sp, 0x54));
            c.swc1(5, p(sp, 0x40));
            c.swc1(3, p(sp, 0x44));
            c.swc1(1, p(sp, 0x60));
            c.swc1(0, p(sp, 0x64));
        }
        c.g[T2][0] = c.lb(at(c, S6, 0x381));

        c.cycles += 57;
        // Align again (unit scale), then the arrow (element `style`).
        c.g[T0][0] = c.g[SP][0];
        c.g[A3][0] = c.lw(p(sp, 0xf4));
        c.g[A1][0] = c.g[S7][0];
        c.g[T3][0] = c.lw(p(sp, 0xe4));
        c.g[A2][0] = c.g[S4][0];
        c.g[T1][0] = c.lb(at(c, S6, 0x380));
        c.g[A0][0] = c.g[S5][0];
        call_ptr(c, 0x001f_10f8, 0x0021_e57c)?;
        c.g[V1][0] = c.lw(gp(c, -0x854));
        c.g[T2][0] = 0xffff_ffff_ffff_fc1f;
        c.g[V0][0] = c.lb(at(c, S6, 0x382));
        c.g[A2][0] = c.g[S7][0];
        c.g[A1][0] = c.lw(at(c, V1, 0xe84));
        c.g[T0][0] = c.g[SP][0];
        c.g[V0][0] = shl32(c.g[V0][0], 5);
        c.g[A3][0] = c.lw(p(sp, 0xf0));
        c.g[V1][0] = c.lw(at(c, A1, 8));
        c.g[V0][0] &= 0x3e0;
        c.g[A0][0] = c.g[S5][0];
        c.g[T1][0] = sx(p(sp, 0x10));
        c.g[V1][0] &= c.g[T2][0];
        c.g[V1][0] |= c.g[V0][0];
        let v = c.g[V1][0];
        c.sw(at(c, A1, 8), v);
        c.g[V0][0] = c.lw(at(c, FP, 0xec));
        c.g[V0][0] = shl32(c.g[V0][0], 2);
        c.g[V0][0] = add32(c.g[S3][0], c.g[V0][0]);
        c.g[A1][0] = c.lw(r32(c, V0));
        call_ptr(c, 0x001f_1190, 0x0021_e5cc)?;
        // The side piece at (p.x + w2, p.y): texture +0x45c + 4 * min(style, 3).
        c.g[V0][0] = c.lw(at(c, FP, 0xec));
        c.g[A1][0] = 3;
        ld_d(c, V1, p(sp, 0x70));
        sd_d(c, V1, p(sp, 0xa0));
        c.g[T0][0] = c.g[SP][0];
        c.g[V1][0] = ((c.g[V0][0] as i64) < 4) as u64;
        c.g[A3][0] = c.lw(p(sp, 0xec));
        if c.g[V1][0] == 0 {
            c.g[V0][0] = c.g[A1][0];
        }
        let x = ldf(c, 0, p(sp, 0xa0));
        let w2 = ldf(c, 1, p(sp, 0x50));
        c.g[V0][0] = shl32(c.g[V0][0], 24);
        c.g[V0][0] = sra32(c.g[V0][0], 22);
        c.g[A0][0] = c.g[S5][0];
        setf(c, 0, x + w2);
        c.g[V0][0] = add32(c.g[S5][0], c.g[V0][0]);
        c.g[A1][0] = c.lw(at(c, V0, 0x45c));
        c.g[A2][0] = sx(p(sp, 0xa0));
        c.g[T1][0] = sx(p(sp, 0x10));
        c.swc1(0, p(sp, 0xa0));
        call_ptr(c, 0x001f_1190, 0x0021_e624)?;
        // The pulse: both pieces again as stretched panels, one row lower.
        c.g[T0][0] = c.lw(p(sp, 0xe0));
        setf(c, 0, FpuF32::ZERO);
        let pulse = ldf(c, 1, at(c, T0, 0x4c));
        let pulsing = le(c, FpuF32::ZERO, pulse);
        c.g[V1][0] = c.lw(gp(c, -0x854));
        if pulsing {
            c.cycles += 52;
            c.g[T1][0] = 0xffff_ffff_ffff_fc1f;
            c.g[V0][0] = c.lb(at(c, S6, 0x382));
            c.g[S0][0] = sx(p(sp, 0xb0));
            c.g[T0][0] = c.lw(at(c, V1, 0xe84));
            c.g[A0][0] = c.g[S5][0];
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            let stretch = ldf(c, 20, gp(c, -0x535c));
            c.g[V1][0] = c.lw(at(c, T0, 8));
            c.g[V0][0] = shl32(c.g[V0][0], 5);
            c.g[V0][0] &= 0x3e0;
            let half = fconst(c, 21, 0x3f00_0000);
            c.g[V1][0] &= c.g[T1][0];
            c.g[A1][0] = sx(p(sp, 0x10));
            c.g[V1][0] |= c.g[V0][0];
            c.g[A2][0] = c.g[S0][0];
            let v = c.g[V1][0];
            c.sw(at(c, T0, 8), v);
            c.g[A3][0] = sx(p(sp, 0xc0));
            ld_d(c, T2, p(sp, 0x70));
            sd_d(c, T2, p(sp, 0xb0));
            setf(c, 13, stretch);
            setf(c, 14, stretch);
            setf(c, 15, half);
            c.g[T1][0] = c.lw(p(sp, 0xe0));
            ldf(c, 12, at(c, T1, 0x4c));
            ld_d(c, T2, p(sp, 0x50));
            sd_d(c, T2, p(sp, 0xc0));
            let unit = fr(c, 22);
            setf(c, 16, unit);
            HUD_RIDER_INDICATOR_PANEL.call(c, 0x0021_e6c4)?;
            c.g[T1][0] = c.lw(p(sp, 0xe0));
            let stretch = fr(c, 20);
            setf(c, 13, stretch);
            c.g[A1][0] = sx(p(sp, 0x10));
            c.g[A2][0] = c.g[S0][0];
            ldf(c, 12, at(c, T1, 0x4c));
            let half = fr(c, 21);
            setf(c, 15, half);
            ld_d(c, T2, p(sp, 0xa0));
            sd_d(c, T2, p(sp, 0xb0));
            let unit = fr(c, 22);
            setf(c, 16, unit);
            c.g[A0][0] = c.g[S5][0];
            c.g[A3][0] = sx(p(sp, 0xd0));
            ld_d(c, T0, p(sp, 0x40));
            sd_d(c, T0, p(sp, 0xd0));
            setf(c, 14, stretch);
            HUD_RIDER_INDICATOR_PANEL.call(c, 0x0021_e710)?;
        }

        c.cycles += 15;
        frame.restore(c, S0, 0x190);
        frame.restore(c, S1, 0x180);
        frame.restore(c, S2, 0x170);
        frame.restore(c, S3, 0x160);
        frame.restore(c, S4, 0x150);
        frame.restore(c, S5, 0x140);
        frame.restore(c, S6, 0x130);
        frame.restore(c, S7, 0x120);
        frame.restore(c, FP, 0x110);
        frame.restore_ra(c, 0x100);
        frame.restore_fpr(c, 22, 0x1b0);
        frame.restore_fpr(c, 21, 0x1a8);
        frame.restore_fpr(c, 20, 0x1a0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{Rng, DATA, GAME_GP};

    fn rider_ind_f(x: f32) -> u32 {
        x.to_bits()
    }

    fn rider_ind_uf(r: &mut Rng, lo: f32, hi: f32) -> f32 {
        lo + (r.next_u32() as f32 / u32::MAX as f32) * (hi - lo)
    }

    /// A HUD element at `e`: position, size (sometimes <= 0: the texture's
    /// size is used), scale, colour, alignment and row.
    fn rider_ind_elem(c: &mut Cpu, r: &mut Rng, e: u32) {
        c.bus.write16(e, (r.next_u32() % 640) as u16);
        c.bus.write16(e + 2, (r.next_u32() % 448) as u16);
        c.bus
            .write16(e + 4, ((r.next_u32() % 80) as i16 - 12) as u16);
        c.bus
            .write16(e + 6, ((r.next_u32() % 80) as i16 - 12) as u16);
        c.bus.write32(e + 8, rider_ind_f(rider_ind_uf(r, 0.5, 2.0)));
        c.bus
            .write32(e + 0xc, rider_ind_f(rider_ind_uf(r, 0.5, 2.0)));
        for k in 0..4 {
            c.bus
                .write32(e + 0x10 + 4 * k, rider_ind_f(rider_ind_uf(r, 0.0, 1.0)));
        }
        c.bus.write8(e + 0x20, (r.next_u32() % 3) as u8);
        c.bus.write8(e + 0x21, (r.next_u32() % 3) as u8);
        c.bus.write8(e + 0x22, (r.next_u32() % 8) as u8);
    }

    /// Every path: the timer off, the style-0 bounce, the red fade; styles
    /// below and above 4 (the side piece's clamp); pulse on and off; some
    /// textures missing (`HUD_DrawSprite` returns early); fakes unwinding.
    #[test]
    fn hud_draw_rider_indicator_matches() {
        check(
            ee::s05::f_0021e1b0__generated,
            hud_draw_rider_indicator,
            250,
            &|c, seed| {
                let mut r = machine(c, seed, 0x21e1_b0, 12);
                let (hud, obj, rider) = (DATA, DATA + 0x1000, DATA + 0x1800);
                let style = seed % 6;
                let index = r.next_u32() % 4;
                // The HUD's textures (+0x444..+0x46c) and panel fields.
                for k in 0..11u32 {
                    let tex = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        DATA + 0x4000 + 0x40 * k
                    };
                    c.bus.write32(hud + 0x444 + 4 * k, tex);
                    if tex != 0 {
                        c.bus.write32(tex, r.next_u32() % 64);
                        for j in 1..8 {
                            c.bus
                                .write32(tex + 4 * j, rider_ind_f(rider_ind_uf(&mut r, 0.0, 64.0)));
                        }
                    }
                }
                c.bus.write32(hud + 0x474, r.next_u32());
                for k in 0..4 {
                    c.bus.write32(
                        hud + 0x480 + 4 * k,
                        rider_ind_f(rider_ind_uf(&mut r, 0.0, 1.0)),
                    );
                }
                // The object's element at +0x360.
                rider_ind_elem(c, &mut r, obj + 0x360);
                // The rider: slot index, timer, pulse.
                c.bus.write32(rider, index);
                let timer = if seed % 7 == 3 {
                    -rider_ind_uf(&mut r, 0.01, 1.0)
                } else {
                    rider_ind_uf(&mut r, 0.0, 1.5)
                };
                c.bus.write32(rider + 0x48, rider_ind_f(timer));
                c.bus
                    .write32(rider + 0x4c, rider_ind_f(rider_ind_uf(&mut r, -0.6, 2.0)));
                // world -> race (+0x84) -> slots (+0xc) -> slot (+0x28 + 4i) -> style.
                let world = DATA + 0x3000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, DATA + 0x3100);
                c.bus.write32(DATA + 0x3100 + 0xc, DATA + 0x3200);
                c.bus
                    .write32(DATA + 0x3200 + 0x28 + 4 * index, DATA + 0x3400);
                c.bus.write32(DATA + 0x3400 + 0xec, style);
                // Draw state (gp-0x854 -> +0xe84) with its flags word.
                c.bus.write32(GAME_GP.wrapping_sub(0x854), DATA + 0x6000);
                c.bus.write32(DATA + 0x6000 + 0xe84, DATA + 0x7800);
                c.bus.write32(DATA + 0x7800 + 8, r.next_u32());
                // BELibrary and its interface 0 (a fake method).
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
                c.bus.write32(GAME_GP.wrapping_sub(0x1efc), 0);
                c.bus.write32(0x0053_08a8, DATA + 0x5000);
                c.bus.write32(DATA + 0x5004, DATA + 0x5100);
                c.bus.write16(DATA + 0x5100 + 8, 0);
                c.bus.write32(DATA + 0x5100 + 0xc, FAKE);
                c.bus
                    .write32(HUD_RIDER_INDICATOR_BADGE, 1 + r.next_u32() % 6);
                // Bounce constants and the sine table.
                for (off, v) in [
                    (0x5368, 12.566_371f32),
                    (0x5364, 81.487_33),
                    (0x5360, 0.199_636_31),
                    (0x535c, 1.810_175),
                ] {
                    c.bus.write32(GAME_GP.wrapping_sub(off), rider_ind_f(v));
                }
                for i in 0..512u32 {
                    c.bus.write32(
                        HUD_RIDER_INDICATOR_SINE + 4 * i,
                        rider_ind_f(rider_ind_uf(&mut r, -1.0, 1.0)),
                    );
                }
                for i in 0..9u32 {
                    rider_ind_elem(c, &mut r, HUD_RIDER_INDICATOR_ELEMS + 0x24 * i);
                }
                c.g[A0][0] = sx(hud);
                c.g[A1][0] = sx(obj);
                c.g[A2][0] = sx(rider);
                c.g[A3][0] = (r.next_u32() % 4) as u64;
                c.pc = 0x0021_e1b0;
            },
        );
    }
}
