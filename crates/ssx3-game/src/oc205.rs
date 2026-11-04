//! Staging wave 201 (oc205): hand-written replacements for the hottest
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
    (0x0037_7458, ps2_graphics_man_set_ortho, "PS2GraphicsMan_SetOrtho"),
];

// ---------------------------------------------------------------------------
// PS2GraphicsMan_SetOrtho (0x377458)

/// PS2GraphicsMan fields used by `PS2GraphicsMan_SetOrtho`.
mod ortho {
    /// Display object: `lh +0x50`/`+0x58` this-deltas, methods at
    /// `+0x54` (screen width) and `+0x5c` (screen height).
    pub const DISPLAY: u32 = 0x10d8;
    pub const CUR_VIEW: u32 = 0x10dc;
    /// Drawing mode (0x31 selects the large depth range).
    pub const MODE: u32 = 0x5a40;
    /// `sPtView views[]`, 0x230 bytes each: viewport x, y, w, h at +0..+0xc,
    /// near/far at +0x10/+0x14, the ortho flag at +0x20.
    pub const VIEWS: u32 = 0x6d20;
    pub const VIEW_STRIDE: u32 = 0x230;
    pub const VIEW_NEAR: u32 = 0x10;
    pub const VIEW_FAR: u32 = 0x14;
    pub const VIEW_ORTHO: u32 = 0x20;
    /// Screen projection matrix (rows 0x6af0..0x6b2c) and viewport quads.
    pub const SCREEN_MAT: u32 = 0x6af0;
    pub const VIEWPORT_MIN: u32 = 0x6b70;
    pub const VIEWPORT_MAX: u32 = 0x6b80;
    /// Set when the view matrices are current (cleared: rebuild).
    pub const MATS_READY: u32 = 0x6b90;
    /// Clip-space matrix (0x57c0), projection (0x5860), clip quads (0x5840,
    /// 0x5850), viewport half sizes and offsets (0x58e0, 0x58f0).
    pub const LOCAL_MAT: u32 = 0x57c0;
    pub const PROJ_MAT: u32 = 0x5860;
    /// gp-relative `.lit4` constants.
    pub const GP_BIAS: u32 = 0x2794;
    pub const GP_BIAS_BIG: u32 = 0x2790;
    pub const GP_DEPTH_BIG: u32 = 0x278c;
    pub const GP_DEPTH_FAR_BIG: u32 = 0x2788;
    pub const GP_DEPTH: u32 = 0x2784;
    pub const GP_DEPTH_FAR: u32 = 0x2780;
    pub const GP_PIXEL_OFFSET: u32 = 0x277c;
    pub const GP_5910: u32 = 0x2778;
    pub const GP_5920: u32 = 0x2774;
    pub const GP_592C: u32 = 0x2770;
}

const SET_SCISSOR_CLAMPED: Callee = Callee {
    addr: 0x0037_6938,
    f: ee::s12::PS2GraphicsMan_SetScissorClamped,
};

/// The display's `this` adjusted by the halfword delta at `delta_off`, and
/// the method pointer at `delta_off + 4` (`a0`/`v0` as the call expects);
/// leaves the display pointer in v1.
fn set_ortho_display_method(c: &mut Cpu, gfx: u32, delta_off: u32) -> u32 {
    use crate::{abi, guest::Addr, ps2f::FpuF32};
    let disp = Addr(gfx).off(ortho::DISPLAY).lw(c);
    c.g[V1][0] = disp;
    let delta = Addr(disp as u32).off(delta_off).lh(c);
    c.g[A0][0] = delta;
    let method = Addr(disp as u32).off(delta_off + 4).lw(c);
    c.g[V0][0] = method;
    c.g[A0][0] = sx(gfx.wrapping_add(delta as u32));
    method as u32
}

/// gcc's `(float)(u32)` of a 64-bit result (calls 3 and 4): halve-or-1 and
/// double when the value reads negative. Leaves v0/v1 as the original.
fn set_ortho_u64_to_float(c: &mut Cpu) -> crate::ps2f::FpuF32 {
    use crate::{abi, guest::Addr, ps2f::FpuF32};
    let negative = (c.g[V0][0] as i64) < 0;
    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
    if negative {
        c.cycles += 5;
        c.g[V0][0] = (c.g[V0][0] & 1) | c.g[V1][0];
        let h = FpuF32::from_i32(c.g[V0][0] as u32 as i32);
        h + h
    } else {
        c.cycles += 4;
        FpuF32::from_i32(c.g[V0][0] as u32 as i32)
    }
}

/// `(float)(u32)(v << 3)` of a 32-bit result (calls 1 and 2): the shifted
/// value halved and doubled when its sign bit is set. Leaves v0 as the
/// original; returns the value and whether the halving path was taken.
fn set_ortho_times8_to_float(c: &mut Cpu) -> (crate::ps2f::FpuF32, bool) {
    use crate::{abi, guest::Addr, ps2f::FpuF32};
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
    if (c.g[V0][0] as i64) < 0 {
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
        let h = FpuF32::from_i32(c.g[V0][0] as u32 as i32);
        (h + h, true)
    } else {
        (FpuF32::from_i32(c.g[V0][0] as u32 as i32), false)
    }
}

/// `void PS2GraphicsMan_SetOrtho(PS2GraphicsMan* g, float left, float top,
/// float width, float height, float near, float far)` (0x377458): switch
/// the current view to an orthographic projection. Marks the view ortho with
/// its near/far, builds the screen matrix and viewport quads from the view
/// rectangle and the display's size (both centred on the GS 2048 offset),
/// builds the clip-space and projection matrices with the guard-band ratios,
/// mode-dependent depth constants, then clamps the scissor to the viewport
/// and resets the fog/texture constants at 0x5910..0x592c.
pub fn ps2_graphics_man_set_ortho(c: &mut Cpu) {
    use crate::{abi, guest::Addr, ps2f::FpuF32};
    use ortho::*;
    guest(c, |c| {
        c.cycles += 42;
        let frame = Frame::open(c, 0x90);
        let sp0 = sp(c);
        let left = FpuF32(c.f[12]);
        let top = FpuF32(c.f[13]);
        let width = FpuF32(c.f[14]);
        let height = FpuF32(c.f[15]);
        let near = FpuF32(c.f[16]);
        let far = FpuF32(c.f[17]);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x20);
        c.g[A1][0] = 0x31;
        for (f, off) in [
            (28, 0x70),
            (27, 0x68),
            (24, 0x50),
            (22, 0x40),
            (31, 0x88),
            (30, 0x80),
            (29, 0x78),
            (26, 0x60),
            (25, 0x58),
            (23, 0x48),
            (21, 0x38),
            (20, 0x30),
        ] {
            frame.save_fpr(c, f, off);
        }
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = VIEW_STRIDE as u64;
        abi::set_fpr(c, 27, width);
        abi::set_fpr(c, 24, near);
        abi::set_fpr(c, 22, far);
        abi::set_fpr(c, 28, height);
        let gfx = c.g[S0][0] as u32;

        // Mark the current view orthographic with its depth range.
        c.g[V1][0] = Addr(gfx).off(CUR_VIEW).lw(c);
        Addr(sp0).write_fpu(c, left);
        c.mult(0, V1, V1, A0);
        Addr(sp0).off(4).write_fpu(c, top);
        let view = gfx.wrapping_add((c.g[V1][0] as u32).wrapping_add(VIEWS));
        c.g[V1][0] = sx(view);
        let ortho_flag = c.g[V0][0];
        Addr(view).off(VIEW_ORTHO).sw(c, ortho_flag);
        Addr(view).off(VIEW_FAR).write_fpu(c, far);
        Addr(view).off(VIEW_NEAR).write_fpu(c, near);

        // The view rectangle and the depth bias for the mode.
        c.g[V0][0] = Addr(gfx).off(CUR_VIEW).lw(c);
        c.g[V1][0] = Addr(gfx).off(MODE).lw(c);
        c.mult(0, A2, V0, A0);
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(gfx));
        let rect = Addr((c.g[V0][0] as u32).wrapping_add(VIEWS));
        let vh = rect.off(0xc).read_fpu(c);
        let vx = rect.read_fpu(c);
        let vy = rect.off(4).read_fpu(c);
        let gp = c.g[GP][0] as u32;
        let mut bias = Addr(gp.wrapping_sub(GP_BIAS)).read_fpu(c);
        let vw = rect.off(8).read_fpu(c);
        if c.g[V1][0] == c.g[A1][0] {
            c.cycles += 1;
            bias = Addr(gp.wrapping_sub(GP_BIAS_BIG)).read_fpu(c);
        }
        abi::set_fpr(c, 23, vh);
        abi::set_fpr(c, 25, vx);
        abi::set_fpr(c, 26, vy);
        abi::set_fpr(c, 29, bias);
        abi::set_fpr(c, 21, vw);

        // Display width: x base = 32768 - 8 * width.
        c.cycles += 8;
        let method = set_ortho_display_method(c, gfx, 0x50);
        call_ptr(c, method, 0x0037_7518)?;
        let (x_off, _) = set_ortho_times8_to_float(c);
        c.cycles += 4;
        abi::set_fpr(c, 1, x_off);
        c.g[V1][0] = Addr(gfx).off(DISPLAY).lw(c);

        // Display height: y base = 32768 - 8 * height.
        c.cycles += 10;
        c.g[AT][0] = 0x4700_0000;
        let k32768 = FpuF32(0x4700_0000);
        abi::set_fpr(c, 0, k32768);
        let disp = c.g[V1][0] as u32;
        c.g[A0][0] = Addr(disp).off(0x58).lh(c);
        c.g[V0][0] = Addr(disp).off(0x5c).lw(c);
        let x_base = k32768 - x_off;
        abi::set_fpr(c, 20, x_base);
        c.g[A0][0] = sx(gfx.wrapping_add(c.g[A0][0] as u32));
        let method = c.g[V0][0] as u32;
        call_ptr(c, method, 0x0037_7560)?;
        let (y_off, halved) = set_ortho_times8_to_float(c);
        c.cycles += if halved { 3 } else { 4 };
        abi::set_fpr(c, 12, y_off);

        // Screen matrix and viewport quads.
        c.cycles += 67;
        let one = FpuF32::ONE;
        let sixteen = FpuF32(0x4180_0000);
        let depth = far - near;
        let x_scale = vw.div(width) * sixteen;
        let y_scale = vh.div(height) * sixteen;
        c.g[V1][0] = Addr(gfx).off(MODE).lw(c);
        Addr(sp0).off(0xc).write_fpu(c, depth);
        c.g[V0][0] = 0x31;
        let z_scale = (-bias).div(depth);
        let x0 = vx * sixteen;
        let y0 = vy * sixteen;
        let y_base = k32768 - y_off;
        let left_m = Addr(sp0).read_fpu(c);
        let top_m = Addr(sp0).off(4).read_fpu(c);
        let x_min = (x0 - x_scale * left_m) + x_base;
        let y_min = (y0 - y_scale * top_m) + y_base;
        let z_off = -far * z_scale;
        let x_max = (vx + vw) * sixteen + x_base;
        let y_max = (vy + vh) * sixteen + y_base;
        let z_far = z_scale * far + z_off;
        let z_near = z_scale * near + z_off;
        let vp_x = x0 + x_base;
        let vp_y = y0 + y_base;
        let m = Addr(gfx).off(SCREEN_MAT);
        m.write_fpu(c, x_scale);
        for o in [0x4, 0x8, 0xc, 0x10, 0x18, 0x1c, 0x20, 0x24, 0x2c] {
            m.off(o).sw(c, 0);
        }
        m.off(0x14).write_fpu(c, y_scale);
        m.off(0x28).write_fpu(c, z_scale);
        m.off(0x30).write_fpu(c, x_min);
        m.off(0x34).write_fpu(c, y_min);
        m.off(0x38).write_fpu(c, z_off);
        m.off(0x3c).write_fpu(c, one);
        let vmin = Addr(gfx).off(VIEWPORT_MIN);
        vmin.write_fpu(c, vp_x);
        vmin.off(4).write_fpu(c, vp_y);
        vmin.off(8).write_fpu(c, z_near);
        vmin.off(0xc).write_fpu(c, one);
        let vmax = Addr(gfx).off(VIEWPORT_MAX);
        vmax.write_fpu(c, x_max);
        vmax.off(4).write_fpu(c, y_max);
        vmax.off(8).write_fpu(c, z_far);
        vmax.off(0xc).write_fpu(c, one);
        Addr(gfx).off(MATS_READY).sw(c, 0);
        c.g[AT][0] = 0x4700_0000;
        for (r, v) in [
            (0, z_far),
            (1, z_near),
            (2, x_max),
            (3, y_max),
            (4, sixteen),
            (5, x_min),
            (6, y_min),
            (7, z_off),
            (8, y_base),
            (9, one),
            (10, vp_y),
            (11, vp_x),
        ] {
            abi::set_fpr(c, r, v);
        }
        // Depth constants for the mode (one on the stack, one kept in f31).
        let (depth_k, depth_far_k);
        if c.g[V1][0] != c.g[V0][0] {
            c.cycles += 3;
            depth_k = Addr(gp.wrapping_sub(GP_DEPTH)).read_fpu(c);
            depth_far_k = Addr(gp.wrapping_sub(GP_DEPTH_FAR)).read_fpu(c);
            abi::set_fpr(c, 3, depth_k);
        } else {
            c.cycles += 4;
            depth_k = Addr(gp.wrapping_sub(GP_DEPTH_BIG)).read_fpu(c);
            depth_far_k = Addr(gp.wrapping_sub(GP_DEPTH_FAR_BIG)).read_fpu(c);
            abi::set_fpr(c, 2, depth_k);
        }
        abi::set_fpr(c, 31, depth_far_k);
        Addr(sp0).off(8).write_fpu(c, depth_k);

        // Screen width in pixels; x centre of the viewport.
        c.cycles += 11;
        let two = FpuF32(0x4000_0000);
        c.g[AT][0] = 0x4000_0000;
        abi::set_fpr(c, 0, two);
        let method = set_ortho_display_method(c, gfx, 0x50);
        let inv_h2 = two.div(height);
        let inv_w2 = two.div(width);
        abi::set_fpr(c, 30, inv_h2);
        abi::set_fpr(c, 29, inv_w2);
        call_ptr(c, method, 0x0037_76d4)?;
        let screen_w = set_ortho_u64_to_float(c);
        abi::set_fpr(c, 1, screen_w);

        // Screen height in pixels; y centre of the viewport.
        c.cycles += 13;
        let half = FpuF32(0x3f00_0000);
        c.g[AT][0] = 0x3f00_0000;
        let half_screen_w = screen_w * half;
        let dx = vw * half - half_screen_w;
        let method = set_ortho_display_method(c, gfx, 0x58);
        let cx = dx + vx;
        abi::set_fpr(c, 1, half_screen_w);
        abi::set_fpr(c, 0, dx);
        abi::set_fpr(c, 20, cx);
        call_ptr(c, method, 0x0037_772c)?;
        let screen_h = set_ortho_u64_to_float(c);

        // Clip-space and projection matrices.
        c.cycles += 125;
        let minus_one = FpuF32(0xbf80_0000);
        let guard_w = FpuF32(0x4500_0000) - (cx.abs() + cx.abs());
        let half_vh = vh * half;
        let cy = (half_vh - screen_h * half) + vy;
        let guard_h = FpuF32(0x4500_0000) - (cy.abs() + cy.abs());
        let left_m = Addr(sp0).read_fpu(c);
        let top_m = Addr(sp0).off(4).read_fpu(c);
        let depth_m = Addr(sp0).off(0xc).read_fpu(c);
        let pixel = Addr(gp.wrapping_sub(GP_PIXEL_OFFSET)).read_fpu(c);
        let gx = vw.div(guard_w);
        let gy = vh.div(guard_h);
        let z_proj = (-(far + near)).div(depth_m);
        let off_x = (cx + pixel) - (left_m * vw).div(width);
        let off_y = (cy + pixel) - (top_m * vh).div(height);
        let two_over_depth = two.div(depth_m);
        let sx_clip = gx * inv_w2;
        let sy_clip = gy * inv_h2;
        let half_guard_w = guard_w * half;
        let half_guard_h = guard_h * half;
        let half_vw = vw * half;
        let depth_k_m = Addr(sp0).off(8).read_fpu(c);

        let p = Addr(gfx).off(PROJ_MAT);
        p.write_fpu(c, inv_w2);
        for o in [0x4, 0x8, 0xc, 0x10, 0x18, 0x1c, 0x20, 0x24, 0x2c] {
            p.off(o).sw(c, 0);
        }
        p.off(0x14).write_fpu(c, inv_h2);
        p.off(0x28).write_fpu(c, two_over_depth);
        p.off(0x30).write_fpu(c, minus_one);
        p.off(0x34).write_fpu(c, minus_one);
        p.off(0x38).write_fpu(c, z_proj);
        p.off(0x3c).write_fpu(c, one);
        // Viewport half sizes (0x58e0) and offsets (0x58f0) with the depths.
        let g = Addr(gfx);
        g.off(0x58e0).write_fpu(c, half_vw);
        g.off(0x58e4).write_fpu(c, half_vh);
        g.off(0x58e8).write_fpu(c, depth_k_m);
        g.off(0x58ec).sw(c, 0);
        g.off(0x58f0).write_fpu(c, off_x);
        g.off(0x58f4).write_fpu(c, off_y);
        g.off(0x58f8).write_fpu(c, depth_far_k);
        g.off(0x58fc).sw(c, 0);
        // Clip-space matrix (guard-band scaled ortho).
        let l = Addr(gfx).off(LOCAL_MAT);
        l.write_fpu(c, sx_clip);
        for o in [0x4, 0x8, 0xc, 0x10, 0x18, 0x1c, 0x20, 0x24, 0x2c] {
            l.off(o).sw(c, 0);
        }
        l.off(0x14).write_fpu(c, sy_clip);
        l.off(0x28).write_fpu(c, two_over_depth);
        l.off(0x30).write_fpu(c, -gx);
        l.off(0x34).write_fpu(c, -gy);
        l.off(0x38).write_fpu(c, z_proj);
        l.off(0x3c).write_fpu(c, one);
        // Clip quads: guard half sizes and offsets with the depths.
        g.off(0x5840).write_fpu(c, half_guard_w);
        g.off(0x5844).write_fpu(c, half_guard_h);
        g.off(0x5848).write_fpu(c, depth_k_m);
        g.off(0x584c).sw(c, 0);
        g.off(0x5850).write_fpu(c, off_x);
        g.off(0x5854).write_fpu(c, off_y);
        g.off(0x5858).write_fpu(c, depth_far_k);
        g.off(0x585c).sw(c, 0);

        // Registers at the call: SetScissorClamped(g, vx, vy, vw, vh).
        c.g[AT][0] = 0x4000_0000;
        for (r, v) in [
            (0, sy_clip),
            (1, off_y),
            (2, half_guard_w),
            (3, half_vw),
            (4, off_x),
            (5, half_guard_h),
            (6, depth_k_m),
            (7, sx_clip),
            (8, (left_m * vw).div(width)),
            (9, half),
            (10, -gy),
            (11, -gx),
            (12, vx),
            (13, vy),
            (14, vw),
            (15, vh),
            (16, z_proj),
            (17, two_over_depth),
            (18, one),
        ] {
            abi::set_fpr(c, r, v);
        }
        c.g[A0][0] = c.g[S0][0];
        SET_SCISSOR_CLAMPED.call(c, 0x0037_78ec)?;

        let gp = c.g[GP][0] as u32;
        let k5910 = Addr(gp.wrapping_sub(GP_5910)).read_fpu(c);
        let k5920 = Addr(gp.wrapping_sub(GP_5920)).read_fpu(c);
        let k592c = Addr(gp.wrapping_sub(GP_592C)).read_fpu(c);
        abi::set_fpr(c, 1, k5910);
        abi::set_fpr(c, 2, k5920);
        abi::set_fpr(c, 0, k592c);
        g.off(0x5910).write_fpu(c, k5910);
        g.off(0x592c).write_fpu(c, k592c);
        g.off(0x5920).write_fpu(c, k5920);
        g.off(0x5924).sw(c, 0);
        g.off(0x5928).sw(c, 0);

        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        for (f, off) in [
            (31, 0x88),
            (30, 0x80),
            (29, 0x78),
            (28, 0x70),
            (27, 0x68),
            (26, 0x60),
            (25, 0x58),
            (24, 0x50),
            (23, 0x48),
            (22, 0x40),
            (21, 0x38),
            (20, 0x30),
        ] {
            frame.restore_fpr(c, f, off);
        }
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // PS2GraphicsMan_SetOrtho ---------------------------------------------

    /// The fake-call hook with sign-extended results, so a forced result
    /// with bit 31 set reads negative as a 64-bit value (the `(float)(u32)`
    /// halving path of the third and fourth calls).
    fn set_ortho_fake(c: &mut Cpu, target: u32) {
        crate::difftest::fakes::fake_call(c, target);
        c.g[V0][0] = sx(c.g[V0][0] as u32);
    }

    /// `PS2GraphicsMan_SetOrtho`: a graphics manager at DATA whose display
    /// object has fake width/height methods (forced results reach the
    /// halving paths of all four conversions), both depth modes, both view
    /// slots, plausible and edge-case ortho rectangles; the real
    /// `SetScissorClamped` runs on the same display with every clamp mode.
    #[test]
    fn ps2_graphics_man_set_ortho_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
        use crate::difftest::{put_words, DATA, GAME_GP};
        fn f(x: f32) -> u32 {
            x.to_bits()
        }
        /// A float in [lo, hi).
        fn uf(r: &mut crate::difftest::Rng, lo: f32, hi: f32) -> u32 {
            f(lo + (r.next_u32() as f32 / u32::MAX as f32) * (hi - lo))
        }
        covered(
            ee::s12::f_00377458__generated,
            ps2_graphics_man_set_ortho,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x37_7458, 6);
                c.hooks.miss = set_ortho_fake;
                c.pc = 0x0037_7458;
                let gfx = DATA;
                // Display object: {delta 0, width()} at +0x50, {delta 0, height()} at +0x58.
                let display = DATA + 0x3000;
                c.bus.write16(display + 0x50, 0);
                c.bus.write32(display + 0x54, FAKE);
                c.bus.write16(display + 0x58, 0);
                c.bus.write32(display + 0x5c, FAKE + 0x10);
                c.bus.write32(gfx + ortho::DISPLAY, display);
                let idx = seed % 2;
                c.bus.write32(gfx + ortho::CUR_VIEW, idx);
                let mode = if seed % 3 == 0 { 0x31 } else { 0x10 + seed % 4 };
                c.bus.write32(gfx + ortho::MODE, mode);
                // The view rectangle (x, y, w, h).
                let view = gfx + ortho::VIEWS + idx * ortho::VIEW_STRIDE;
                let rect = [uf(&mut r, -64.0, 64.0), uf(&mut r, -64.0, 64.0), uf(&mut r, 1.0, 640.0), uf(&mut r, 1.0, 480.0)];
                put_words(c, view, &rect);
                // SetScissorClamped's clamp mode and ratios.
                let clamp = match seed % 4 {
                    0 => 0,
                    1 => 2,
                    2 => 3,
                    _ => 0xffff_ffff,
                };
                c.bus.write32(gfx + 0x6b94, clamp);
                c.bus.write32(gfx + 0x6b98, f(0.75));
                c.bus.write32(gfx + 0x6b9c, f(0.5));
                // gp-0x2794 .. gp-0x2770: bias, big-mode bias, depth pairs,
                // pixel offset, the three constants stored after the call.
                put_words(
                    c,
                    GAME_GP - ortho::GP_BIAS,
                    &[
                        f(0.5),
                        f(-0.25),
                        f(65535.0),
                        f(16777215.0),
                        f(255.0),
                        f(65535.0),
                        f(0.5),
                        f(1.0),
                        f(2.0),
                        f(3.0),
                    ],
                );
                // Forced display sizes: bit 28 (calls 1-2) and bit 31 (calls
                // 3-4) set take the halving paths; 0 keeps the fake's value.
                let forced: [u32; 4] = match seed % 4 {
                    1 => [0x1000_0140, 0x1000_00f0, 0x8000_0281, 0x8000_01e0],
                    2 => [0x1000_0280, 0, 0x8000_0141, 0],
                    3 => [0, 0x1000_0100, 0, 0x8000_0101],
                    _ => [0; 4],
                };
                put_words(c, LOG_RESULTS, &forced);
                // left, top, width, height, near, far.
                let args = if seed % 5 == 0 {
                    [0; 6].map(|_| r.edge_f32())
                } else {
                    [
                        uf(&mut r, -32.0, 32.0),
                        uf(&mut r, -32.0, 32.0),
                        uf(&mut r, 1.0, 640.0),
                        uf(&mut r, 1.0, 480.0),
                        uf(&mut r, -16.0, 0.0),
                        uf(&mut r, 1.0, 4096.0),
                    ]
                };
                for (i, a) in args.into_iter().enumerate() {
                    c.f[12 + i] = a;
                }
                c.g[A0][0] = sx(gfx);
            },
        );
    }
}
