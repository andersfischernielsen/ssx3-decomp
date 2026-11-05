//! Staging wave 204 (oc230): hand-written replacements for the hottest
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

/// `gp + off`.
#[inline(always)]
fn oc230_gp(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off)
}

/// `reg + off` (the low word of a GPR as a base address).
#[inline(always)]
fn oc230_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// The renderer's fields used by the screen tint.
mod oc230_renderer {
    /// Top of the draw-state stack (20-byte records).
    pub const STATE: u32 = 0xe84;
    /// Output mode (1: 640 wide; else 320, doubled when `INTERLACE`).
    pub const MODE: u32 = 0x270;
    pub const INTERLACE: u32 = 0x274;
    /// The vtable (gcc 2.9x `{delta, fn}` slots).
    pub const VTABLE: u32 = 0x10d8;
}

/// Renderer virtual call through `s0`'s vtable slot `slot` (`this` = s0 +
/// delta, the rest of the arguments already set), returning to `ret`.
fn oc230_render_vcall(c: &mut Cpu, slot: u32, ret: u32) -> crate::call::Flow {
    c.g[V0][0] = c.lw(oc230_at(c, S0, oc230_renderer::VTABLE));
    c.g[A0][0] = c.lh(oc230_at(c, V0, slot));
    c.g[V1][0] = c.lw(oc230_at(c, V0, slot + 4));
    let t = r32(c, V1);
    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
    call_ptr(c, t, ret)
}

/// Copy the 20-byte record at `src` to `dst` with the original's
/// unaligned `ldl/ldr`/`sdl/sdr` pairs through `lo`/`hi` and `t` (`lw`).
fn oc230_copy_state(c: &mut Cpu, src: u32, dst: u32, lo: usize, hi: usize, t: usize) {
    c.ldl(lo, src.wrapping_add(7));
    c.ldr(lo, src);
    c.ldl(hi, src.wrapping_add(0xf));
    c.ldr(hi, src.wrapping_add(8));
    c.g[t][0] = c.lw(src.wrapping_add(0x10));
    c.sdl(lo, dst.wrapping_add(7));
    c.sdr(lo, dst);
    c.sdl(hi, dst.wrapping_add(0xf));
    c.sdr(hi, dst.wrapping_add(8));
    let v = c.g[t][0];
    c.sw(dst.wrapping_add(0x10), v);
}

/// `void VfxScreenTint_Render(void)` (0x390f20): when the screen tint is
/// enabled (`gp+0x151c`) and its alpha (`gp+0x15ac`) is not zero, pushes a
/// copy of the renderer's draw state, begins the scene, sets the full-screen
/// viewport (640x480, or 320x480 offset by 320 when interlaced), resets the
/// state to the default record (0x501420) with alpha blending, draws a
/// screen-sized quad (four vertices, colour = tint rgb * alpha * 255, alpha
/// * 255) and pops the state.
pub fn vfx_screen_tint_render(c: &mut Cpu) {
    use oc230_renderer as rd;
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x110);
        c.g[V0][0] = c.lw(oc230_gp(c, 0x151c));
        frame.save(c, S1, 0xe0);
        frame.save(c, S0, 0xf0);
        c.g[S1][0] = 0;
        frame.save_ra(c, 0xd0);
        frame.save_fpr(c, 21, 0x108);
        frame.save_fpr(c, 20, 0x100);
        if c.g[V0][0] != 0 {
            // visible = alpha != 0
            c.cycles += 6;
            c.lwc1(1, oc230_gp(c, 0x15ac));
            c.mtc1(0, 0);
            c.c_eq_s(1, 0);
            if !c.fcc() {
                c.g[S1][0] = 1;
            }
        }
        c.cycles += 2;
        c.g[S0][0] = c.lw(oc230_gp(c, 0x2a90));
        if c.g[S1][0] != 0 {
            // Push the draw state (copy the top record one up).
            c.cycles += 33;
            c.g[V0][0] = c.lw(oc230_at(c, S0, rd::STATE));
            let top = r32(c, V0);
            oc230_copy_state(c, top, top.wrapping_add(0x14), V1, A0, A1);
            c.g[V1][0] = c.lw(oc230_at(c, S0, rd::STATE));
            c.g[V0][0] = c.lw(oc230_at(c, S0, rd::VTABLE));
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x14));
            let v = c.g[V1][0];
            c.sw(oc230_at(c, S0, rd::STATE), v);
            // BeginScene (0xa8), Begin2D (0xf8), slot 0x128.
            c.g[A0][0] = c.lh(oc230_at(c, V0, 0xa8));
            c.g[V1][0] = c.lw(oc230_at(c, V0, 0xac));
            let t = r32(c, V1);
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
            call_ptr(c, t, 0x0039_0fb0)?;
            oc230_render_vcall(c, 0xf8, 0x0039_0fc4)?;
            oc230_render_vcall(c, 0x128, 0x0039_0fd8)?;
            // Viewport width f20 and x offset f21.
            c.g[A0][0] = c.lw(oc230_at(c, S0, rd::MODE));
            c.g[V0][0] = 1;
            if c.g[A0][0] != c.g[V0][0] {
                c.g[V0][0] = c.lw(oc230_at(c, S0, rd::INTERLACE));
                c.cycles += 5;
                c.g[AT][0] = 0x43a0_0000; // 320
                c.mtc1(AT, 20);
                c.mtc1(0, 21);
                if c.g[V0][0] != 0 {
                    c.mov_s(21, 20);
                }
                c.cycles += 1;
            } else {
                c.cycles += 5;
                c.g[AT][0] = 0x4420_0000; // 640
                c.mtc1(AT, 20);
                c.mtc1(0, 21);
            }
            c.g[V0][0] = c.lw(oc230_at(c, S0, rd::VTABLE));

            // SetViewport(this, 0, 0, x, 0, w, 480, 0, 1) (0xd0).
            c.cycles += 104;
            c.g[A1][0] = 0;
            c.mtc1(0, 13);
            c.mov_s(14, 20);
            c.g[A0][0] = c.lh(oc230_at(c, V0, 0xd0));
            c.g[A2][0] = 0;
            c.g[V1][0] = c.lw(oc230_at(c, V0, 0xd4));
            c.mov_s(12, 21);
            c.g[AT][0] = 0x43f0_0000; // 480
            c.mtc1(AT, 15);
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 17);
            let t = r32(c, V1);
            c.mov_s(16, 13);
            call_ptr(c, t, 0x0039_104c)?;

            // Reset the top state to the default record, then its bits.
            c.g[V1][0] = c.lw(oc230_at(c, S0, rd::STATE));
            c.g[V0][0] = 0x50_0000;
            c.g[A2][0] = 0xffff_ffff_ffff_fc1f;
            c.g[A0][0] = 0xffff_ffff_ffbf_0000;
            c.g[A1][0] = sx(0x0050_1420);
            let top = r32(c, V1);
            oc230_copy_state(c, 0x0050_1420, top, A3, T0, T1);
            c.g[A0][0] |= 0xffff;
            c.g[T2][0] = 0x40_0000;
            c.g[A1][0] = 0xffff_ffff_fe7f_0000;
            c.g[V1][0] = c.lw(oc230_at(c, S0, rd::STATE));
            c.g[A1][0] |= 0xffff;
            c.g[T3][0] = 0x100_0000;
            c.g[A3][0] = 0xffff_ffff_ffcf_0000;
            c.g[V0][0] = c.lw(oc230_at(c, V1, 8));
            c.g[A3][0] |= 0xffff;
            c.g[T0][0] = 0xffff_ffff_fff0_0000;
            c.g[T1][0] = 0x1_0000;
            c.g[V0][0] &= c.g[A2][0];
            c.g[T0][0] |= 0xfff;
            c.g[V0][0] |= 0xa0;
            c.g[A2][0] = c.lw(oc230_gp(c, 0x15bc));
            let v = c.g[V0][0];
            c.sw(oc230_at(c, V1, 8), v);
            c.g[T1][0] |= 0x4000;
            c.g[T4][0] = 0xffff_ffff_ffff_ff83;
            c.g[A2][0] = sx(r32(c, A2) << 2);
            c.g[V1][0] = c.lw(oc230_at(c, S0, rd::STATE));
            c.g[A2][0] &= 0x7c;
            c.g[T5][0] = u64::MAX;
            c.add_s(14, 21, 20);
            c.g[V0][0] = c.lw(oc230_at(c, V1, 4));
            c.g[T7][0] = u64::MAX;
            c.g[T6][0] = 3;
            // word 4 of the top state: (w & mask) | bits, five times.
            c.g[V0][0] &= c.g[A0][0];
            c.g[V0][0] |= c.g[T2][0];
            let v = c.g[V0][0];
            c.sw(oc230_at(c, V1, 4), v);
            for (base, mask, bits) in [
                (A0, A1, Some(T3)),
                (V1, A3, None),
                (A0, T0, Some(T1)),
                (A1, T4, Some(A2)),
            ] {
                c.g[base][0] = c.lw(oc230_at(c, S0, rd::STATE));
                c.g[V0][0] = c.lw(oc230_at(c, base, 4));
                c.g[V0][0] &= c.g[mask][0];
                if let Some(b) = bits {
                    c.g[V0][0] |= c.g[b][0];
                }
                let v = c.g[V0][0];
                c.sw(oc230_at(c, base, 4), v);
            }
            c.g[V1][0] = c.lw(oc230_at(c, S0, rd::STATE));
            let v = c.g[T5][0];
            c.sh(oc230_at(c, V1, 0x10), v);

            // Quad corner positions (x, y, 0, 1) built in the scratch
            // quadword at sp+0xc0 and copied to the vertices' +0x20.
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 1);
            let sp = sp(c);
            c.swc1(21, sp.wrapping_add(0xc0));
            c.sw(sp.wrapping_add(0xc8), 0);
            c.sw(sp.wrapping_add(0xc4), 0);
            c.swc1(1, sp.wrapping_add(0xcc));
            c.g[AT][0] = 0x43f0_0000;
            c.mtc1(AT, 0);
            c.lq(V0, sp.wrapping_add(0xc0));
            c.swc1(14, sp.wrapping_add(0xc0));
            c.sw(sp.wrapping_add(0xc8), 0);
            c.sw(sp.wrapping_add(0xc4), 0);
            c.sq(V0, sp.wrapping_add(0x20));
            c.lq(V1, sp.wrapping_add(0xc0));
            c.swc1(21, sp.wrapping_add(0xc0));
            c.sw(sp.wrapping_add(0xc8), 0);
            c.swc1(0, sp.wrapping_add(0xc4));
            c.sq(V1, sp.wrapping_add(0x50));
            c.lq(A0, sp.wrapping_add(0xc0));
            c.swc1(14, sp.wrapping_add(0xc0));
            c.sw(sp.wrapping_add(0xc8), 0);
            c.sq(A0, sp.wrapping_add(0x80));
            c.lq(V0, sp.wrapping_add(0xc0));
            c.sq(V0, sp.wrapping_add(0xb0));
            if c.g[S1][0] != 0 {
                // Colour = tint (r, g, b) * alpha * 255, alpha * 255.
                c.cycles += 25;
                c.lwc1(4, oc230_gp(c, 0x15ac));
                c.mov_s(5, 1);
                c.lwc1(3, oc230_gp(c, 0x15b0));
                c.g[V1][0] = c.g[SP][0];
                c.lwc1(2, oc230_gp(c, 0x15b4));
                c.g[T0][0] = sx(sp.wrapping_add(0xc0));
                c.lwc1(1, oc230_gp(c, 0x15b8));
                c.mul_s(3, 4, 3);
                c.g[AT][0] = 0x437f_0000; // 255
                c.mtc1(AT, 0);
                c.mul_s(2, 4, 2);
                c.mul_s(1, 4, 1);
                c.mul_s(3, 3, 0);
                c.mul_s(4, 4, 0);
                c.mul_s(2, 2, 0);
                c.mul_s(1, 1, 0);
                c.cvt_w_s(0, 3);
                c.mfc1(A3, 0);
                c.cvt_w_s(0, 2);
                c.mfc1(A2, 0);
                c.cvt_w_s(0, 1);
                c.mfc1(A1, 0);
                c.cvt_w_s(0, 4);
                c.mfc1(A0, 0);
                let v = c.g[A3][0];
                c.sw(oc230_at(c, V1, 0x10), v);
                // Per vertex (0x30 apart): q = 1 at +8, colour at +0x10.
                loop {
                    c.cycles += 8;
                    let vert = r32(c, V1);
                    let (a2, a1, a0) = (c.g[A2][0], c.g[A1][0], c.g[A0][0]);
                    c.sw(vert.wrapping_add(0x14), a2);
                    c.sw(vert.wrapping_add(0x18), a1);
                    c.sw(vert.wrapping_add(0x1c), a0);
                    c.swc1(5, vert.wrapping_add(8));
                    c.g[V1][0] = sx(vert.wrapping_add(0x30));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[T0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    let v = c.g[A3][0];
                    c.sw(oc230_at(c, V1, 0x10), v);
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_1218);
                    }
                }
                // DrawPrimitive(this, 4, vertices, 0) (0x238).
                c.cycles += 8;
                c.g[V0][0] = c.lw(oc230_at(c, S0, rd::VTABLE));
                c.g[A1][0] = 4;
                c.g[A2][0] = c.g[SP][0];
                c.g[A3][0] = 0;
                c.g[A0][0] = c.lh(oc230_at(c, V0, 0x238));
                c.g[V1][0] = c.lw(oc230_at(c, V0, 0x23c));
                let t = r32(c, V1);
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                call_ptr(c, t, 0x0039_1258)?;
            }

            // End2D (0x100), EndScene (0xb0), pop the draw state.
            c.cycles += 13;
            oc230_render_vcall(c, 0x100, 0x0039_126c)?;
            oc230_render_vcall(c, 0xb0, 0x0039_1280)?;
            c.g[V0][0] = c.lw(oc230_at(c, S0, rd::STATE));
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x14));
            let v = c.g[V0][0];
            c.sw(oc230_at(c, S0, rd::STATE), v);
        }
        c.cycles += 7;
        frame.restore(c, S0, 0xf0);
        frame.restore(c, S1, 0xe0);
        frame.restore_ra(c, 0xd0);
        frame.restore_fpr(c, 21, 0x108);
        frame.restore_fpr(c, 20, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// The tail of a sphere-vs-segment test (f0 = dot(centre - start, edge)) in
/// the FPU registers given: `t = f0 / len`; v0 = 1 when `t >= 0`, the centre
/// is within the radius of the line (`d2 - t^2 <= r2`) and the entry point
/// `t - sqrt(r2 - (d2 - t^2))` is not past the end.
fn oc230_seg_tail(c: &mut Cpu, t: usize, len: usize, d2: usize, r2: usize, zero: usize) {
    c.div_s(t, 0, len);
    c.c_lt_s(t, zero);
    let behind = c.fcc();
    c.g[V0][0] = 0;
    if behind {
        return;
    }
    c.cycles += 5;
    c.mul_s(0, t, t);
    c.sub_s(0, d2, 0);
    c.c_lt_s(r2, 0);
    if c.fcc() {
        return;
    }
    c.cycles += 8;
    c.sub_s(0, r2, 0);
    c.g[V0][0] = 1;
    c.sqrt_s(0, 0);
    c.sub_s(0, t, 0);
    c.c_lt_s(len, 0);
    if c.fcc() {
        c.g[V0][0] = 0;
    }
}

/// The sphere-tree fields `RiderSphereTree_CollideTriangle2` reads.
mod oc230_tree {
    /// Root sphere centre (Vec4) and radius.
    pub const CENTER: u32 = 0x10;
    pub const RADIUS: u32 = 0x20;
    /// Bit i: kid i takes part.
    pub const MASK: u32 = 0x28;
    pub const COUNT: u32 = 0x2c;
    /// Kids: centre (Vec4) and radius at +0x10, 0x20 apart.
    pub const KIDS: u32 = 0x30;
}

/// The hit of `RiderSphereTree_CollideTriangle2`: push = n * (r - dist)
/// to `*out_push` (sp+0x80), the contact point (sp+0x10) to `*out_point`
/// (sp+0x84), push kept at sp+0x20. `kid` selects the kid loop's
/// registers (and its `best = dist`).
fn oc230_ct2_hit(c: &mut Cpu, kid: bool) {
    use ssx3_ee_rt::vu0::{Src, X};
    let s = sp(c);
    let (scale, push, point, ptr0) = if kid { (V0, V1, V0, A0) } else { (V1, A0, V1, A1) };
    c.sub_s(0, 21, 20);
    if kid {
        c.mov_s(23, 20);
    } else {
        c.g[V0][0] = 1;
    }
    c.mfc1(scale, 0);
    c.lqc2(4, s);
    c.qmtc2(scale, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, s.wrapping_add(0x30));
    c.lq(push, s.wrapping_add(0x30));
    c.g[ptr0][0] = c.lw(s.wrapping_add(0x80));
    c.lq(point, s.wrapping_add(0x10));
    c.sq(push, r32(c, ptr0));
    c.g[A1][0] = c.lw(s.wrapping_add(0x84));
    c.sq(point, r32(c, A1));
    c.sq(push, s.wrapping_add(0x20));
}

/// Does the root sphere touch the triangle (p, a, b)? v0 = 1/0. Its plane
/// distance is in f20, radius in f21, n at sp+0. Leaves s0 = the centre.
fn oc230_ct2_root_touch(c: &mut Cpu) -> crate::call::Flow {
    use crate::game::intersect::riderspheretree as rst;
    use ssx3_ee_rt::vu0::{Src, X};
    let s = sp(c);
    let center = r32(c, S3).wrapping_add(oc230_tree::CENTER);
    // Projected centre d = centre - n * dist (sp+0x10) inside?
    c.cycles += 22;
    c.mfc1(V0, 20);
    c.lqc2(4, s);
    c.qmtc2(V0, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, s.wrapping_add(0x30));
    c.g[S0][0] = c.g[A0][0];
    c.lq(V0, s.wrapping_add(0x30));
    c.g[A0][0] = c.g[S1][0];
    c.lwc1(12, oc230_gp(c, 0u32.wrapping_sub(0x2dd0)));
    c.g[A1][0] = c.g[S4][0];
    c.sq(V0, s.wrapping_add(0x20));
    c.g[A2][0] = c.g[S6][0];
    rst::vsub_spill(c, center, s.wrapping_add(0x20), s.wrapping_add(0x30), s.wrapping_add(0x10));
    c.g[A3][0] = sx(s.wrapping_add(0x10));
    rst::TRI_INTERSECT.call(c, 0x0032_ab68)?;
    let inside = c.g[V0][0] != 0;
    c.g[V0][0] = 1;
    if inside {
        return Ok(());
    }
    // Near p? (v = centre - p at sp+0x40, d2 = |v|^2 in f5, r^2 in f3)
    c.cycles += 20;
    let p = r32(c, S1);
    rst::vsub_spill(c, center, p, s.wrapping_add(0x50), s.wrapping_add(0x40));
    c.mul_s(3, 21, 21);
    rst::vdot(c, s.wrapping_add(0x40), s.wrapping_add(0x40), V1);
    c.mtc1(V1, 5);
    c.c_lt_s(5, 3);
    c.g[V0][0] = 1;
    if c.fcc() {
        return Ok(());
    }
    // Near edge p-a?
    c.cycles += 30;
    let a = r32(c, S4);
    rst::vsub_spill(c, a, p, s.wrapping_add(0x60), s.wrapping_add(0x50));
    rst::vlen(c, s.wrapping_add(0x50));
    c.mtc1(V1, 2);
    rst::vdot(c, s.wrapping_add(0x40), s.wrapping_add(0x50), V0);
    c.mtc1(V0, 0);
    oc230_seg_tail(c, 1, 2, 5, 3, 22);
    c.cycles += 2;
    let hit = c.g[V0][0] != 0;
    c.g[V0][0] = 1;
    if hit {
        return Ok(());
    }
    // Near edge p-b?
    c.cycles += 31;
    let b = r32(c, S6);
    rst::vsub_spill(c, b, p, s.wrapping_add(0x60), s.wrapping_add(0x50));
    c.mtc1(0, 1);
    rst::vlen(c, s.wrapping_add(0x50));
    c.mtc1(V1, 4);
    rst::vdot(c, s.wrapping_add(0x40), s.wrapping_add(0x50), V0);
    c.mtc1(V0, 0);
    oc230_seg_tail(c, 2, 4, 5, 3, 1);
    c.cycles += 2;
    let hit = c.g[V0][0] != 0;
    c.g[V0][0] = 1;
    if hit {
        return Ok(());
    }
    // Near edge a-b? (v2 = centre - a, d2 = |v2|^2)
    c.cycles += 46;
    let center = r32(c, S0);
    rst::vsub_spill(c, center, a, s.wrapping_add(0x60), s.wrapping_add(0x50));
    c.mtc1(0, 1);
    rst::vdot(c, s.wrapping_add(0x50), s.wrapping_add(0x50), V1);
    c.mtc1(V1, 5);
    rst::vsub_spill(c, b, a, s.wrapping_add(0x70), s.wrapping_add(0x60));
    rst::vlen(c, s.wrapping_add(0x60));
    c.mtc1(V1, 4);
    rst::vdot(c, s.wrapping_add(0x50), s.wrapping_add(0x60), V0);
    c.mtc1(V0, 0);
    oc230_seg_tail(c, 2, 4, 5, 3, 1);
    Ok(())
}

/// One kid of the tree (s0 = its centre, s7 = tree + 0x20 * index, s2 =
/// index, fp = 1): when it is in the mask, its plane distance is below its
/// radius and below the best so far (f23), and it touches the triangle, it
/// becomes the hit. Ends with v0 = the kid count.
fn oc230_ct2_kid(c: &mut Cpu) -> crate::call::Flow {
    use crate::game::intersect::riderspheretree as rst;
    use oc230_tree as tr;
    use ssx3_ee_rt::vu0::{Src, X};
    let s = sp(c);
    let tree = r32(c, S3);
    c.cycles += 5;
    c.g[V0][0] = c.lw(tree.wrapping_add(tr::MASK));
    c.g[V1][0] = sx(r32(c, FP) << (r32(c, S2) & 31));
    c.g[V0][0] &= c.g[V1][0];
    if c.g[V0][0] == 0 {
        c.g[V0][0] = c.lw(tree.wrapping_add(tr::COUNT));
        return Ok(());
    }
    c.cycles += 20;
    let kid = r32(c, S0);
    let p = r32(c, S1);
    rst::vsub_spill(c, kid, p, s.wrapping_add(0x30), s.wrapping_add(0x20));
    c.lwc1(21, kid.wrapping_add(0x10));
    rst::vdot(c, s, s.wrapping_add(0x20), V1);
    c.mtc1(V1, 20);
    c.c_lt_s(20, 21);
    if !c.fcc() {
        c.g[V0][0] = c.lw(tree.wrapping_add(tr::COUNT));
        return Ok(());
    }
    c.cycles += 4;
    c.c_lt_s(20, 23);
    if !c.fcc() {
        c.g[V0][0] = c.lw(tree.wrapping_add(tr::COUNT));
        return Ok(());
    }
    'touch: {
        // Projected centre inside?
        c.cycles += 23;
        c.mfc1(V0, 20);
        c.lqc2(4, s);
        c.qmtc2(V0, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, s.wrapping_add(0x40));
        c.lwc1(12, oc230_gp(c, 0u32.wrapping_sub(0x2dc8)));
        c.lq(V0, s.wrapping_add(0x40));
        c.g[S5][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S4][0];
        c.sq(V0, s.wrapping_add(0x30));
        c.g[A2][0] = c.g[S6][0];
        rst::vsub_spill(c, kid, s.wrapping_add(0x30), s.wrapping_add(0x40), s.wrapping_add(0x10));
        c.g[A3][0] = sx(s.wrapping_add(0x10));
        c.sq(V0, s.wrapping_add(0x20));
        rst::TRI_INTERSECT.call(c, 0x0032_af50)?;
        let inside = c.g[V0][0] != 0;
        c.g[V1][0] = 1;
        if inside {
            break 'touch;
        }
        // Near p?
        c.cycles += 20;
        let kid_at = r32(c, S7).wrapping_add(tr::KIDS);
        rst::vsub_spill(c, kid_at, p, s.wrapping_add(0x30), s.wrapping_add(0x20));
        c.mul_s(2, 21, 21);
        rst::vdot(c, s.wrapping_add(0x20), s.wrapping_add(0x20), V1);
        c.mtc1(V1, 4);
        c.c_lt_s(4, 2);
        c.g[V1][0] = 1;
        if c.fcc() {
            break 'touch;
        }
        // Near edge p-a, p-b?
        for end in [S4, S6] {
            c.cycles += 30;
            let e = r32(c, end);
            rst::vsub_spill(c, e, p, s.wrapping_add(0x40), s.wrapping_add(0x30));
            rst::vlen(c, s.wrapping_add(0x30));
            c.mtc1(V1, 3);
            rst::vdot(c, s.wrapping_add(0x20), s.wrapping_add(0x30), V0);
            c.mtc1(V0, 0);
            oc230_seg_tail(c, 1, 3, 4, 2, 22);
            c.cycles += 2;
            let hit = c.g[V0][0] != 0;
            c.g[V1][0] = 1;
            if hit {
                break 'touch;
            }
        }
        // Near edge a-b?
        c.cycles += 45;
        let (a, b) = (r32(c, S4), r32(c, S6));
        rst::vsub_spill(c, r32(c, S5), a, s.wrapping_add(0x40), s.wrapping_add(0x30));
        rst::vdot(c, s.wrapping_add(0x30), s.wrapping_add(0x30), V1);
        c.mtc1(V1, 4);
        rst::vsub_spill(c, b, a, s.wrapping_add(0x50), s.wrapping_add(0x40));
        rst::vlen(c, s.wrapping_add(0x40));
        c.mtc1(V1, 3);
        rst::vdot(c, s.wrapping_add(0x30), s.wrapping_add(0x40), V0);
        c.mtc1(V0, 0);
        c.div_s(1, 0, 3);
        c.c_lt_s(1, 22);
        let behind = c.fcc();
        c.g[V0][0] = 0;
        if !behind {
            c.cycles += 6;
            c.mul_s(0, 1, 1);
            c.sub_s(0, 4, 0);
            c.c_lt_s(2, 0);
            let off_line = c.fcc();
            c.g[V1][0] = c.g[FP][0];
            if off_line {
                c.cycles += 1;
                if c.g[V0][0] == 0 {
                    c.g[V1][0] = 0;
                }
                break 'touch;
            }
            c.cycles += 8;
            c.sub_s(0, 2, 0);
            c.g[V0][0] = 1;
            c.sqrt_s(0, 0);
            c.sub_s(0, 1, 0);
            c.c_lt_s(3, 0);
            if !c.fcc() {
                break 'touch;
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 1;
        c.g[V1][0] = c.g[FP][0];
        c.cycles += 1;
        if c.g[V0][0] == 0 {
            c.g[V1][0] = 0;
        }
    }
    c.cycles += 2;
    if c.g[V1][0] != 0 {
        c.cycles += 15;
        oc230_ct2_hit(c, true);
    }
    c.g[V0][0] = c.lw(tree.wrapping_add(tr::COUNT));
    Ok(())
}

/// `bool RiderSphereTree_CollideTriangle2(SphereTree* tree, Vec4* p, Vec4*
/// a, Vec4* b, Vec4* n, Vec4* out_push, Vec4* out_point)` (0x32aa28): the
/// rider's sphere tree against the triangle (p, a, b) with normal n (the
/// second triangle test, n flipped toward the sphere). The root sphere must
/// be within its radius of the plane and touch the triangle (projected
/// centre inside, near p, or near an edge). With no kids in the mask the
/// root is the hit; otherwise every kid in the mask is tried and the
/// nearest touching one is the hit. On a hit the push-out `n * (r - dist)`
/// and the contact point are written.
pub fn rider_sphere_tree_collide_triangle2(c: &mut Cpu) {
    use crate::game::intersect::riderspheretree as rst;
    use oc230_tree as tr;
    use ssx3_ee_rt::vu0::{Src, X};
    guest(c, |c| {
        c.cycles += 45;
        let frame = Frame::open(c, 0x150);
        let s = sp(c);
        c.lq(V0, r32(c, T0));
        frame.save(c, S1, 0x110);
        frame.save(c, S3, 0xf0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S4, 0xe0);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S6, 0xc0);
        c.g[A0][0] = sx(r32(c, S3).wrapping_add(tr::CENTER));
        c.sq(V0, s); // n
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, S0, 0x120);
        c.g[S6][0] = c.g[A3][0];
        frame.save(c, S2, 0x100);
        frame.save(c, S5, 0xd0);
        frame.save(c, S7, 0xb0);
        frame.save(c, FP, 0xa0);
        frame.save_ra(c, 0x90);
        frame.save_fpr(c, 23, 0x148);
        frame.save_fpr(c, 22, 0x140);
        frame.save_fpr(c, 21, 0x138);
        frame.save_fpr(c, 20, 0x130);
        // dist = n . (centre - p) in f20, radius in f21.
        let tree = r32(c, S3);
        rst::vsub_spill(c, tree.wrapping_add(tr::CENTER), r32(c, S1), s.wrapping_add(0x20), s.wrapping_add(0x10));
        let (push, point) = (c.g[T1][0], c.g[T2][0]);
        c.sw(s.wrapping_add(0x80), push);
        c.lwc1(21, tree.wrapping_add(tr::RADIUS));
        c.sw(s.wrapping_add(0x84), point);
        rst::vdot(c, s, s.wrapping_add(0x10), V1);
        c.mtc1(V1, 20);
        c.abs_s(0, 20);
        c.c_lt_s(21, 0);
        let far = c.fcc();
        c.g[V0][0] = 0;
        'done: {
            if far {
                break 'done;
            }
            // Below the plane: dist = -dist, n = -n.
            c.cycles += 4;
            c.mtc1(0, 22);
            c.c_lt_s(20, 22);
            if c.fcc() {
                c.cycles += 11;
                c.g[AT][0] = 0xffff_ffff_bf80_0000;
                c.mtc1(AT, 0);
                c.neg_s(20, 20);
                c.mfc1(V1, 0);
                c.lqc2(4, s);
                c.qmtc2(V1, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, s.wrapping_add(0x20));
                c.lq(V0, s.wrapping_add(0x20));
                c.sq(V0, s);
                c.sq(V0, s.wrapping_add(0x10));
            }
            oc230_ct2_root_touch(c)?;
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'done;
            }
            c.g[V0][0] = c.lw(tree.wrapping_add(tr::MASK));
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                // No kids: the root is the hit.
                c.cycles += 15;
                oc230_ct2_hit(c, false);
                break 'done;
            }
            c.g[V0][0] = c.lw(tree.wrapping_add(tr::COUNT));
            // The nearest touching kid (best in f23).
            c.cycles += 3;
            c.lwc1(23, oc230_gp(c, 0u32.wrapping_sub(0x2dcc)));
            let none = (c.g[V0][0] as i64) <= 0;
            c.g[S2][0] = 0;
            if !none {
                c.cycles += 4;
                c.mtc1(0, 22);
                c.g[FP][0] = 1;
                c.g[S7][0] = c.g[S3][0];
                c.g[S0][0] = sx(tree.wrapping_add(tr::KIDS));
                loop {
                    oc230_ct2_kid(c)?;
                    c.cycles += 5;
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                    c.g[S7][0] = sx(r32(c, S7).wrapping_add(0x20));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x20));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_ae88);
                    }
                }
            }
            // Hit when best moved off its start value.
            c.cycles += 5;
            c.lwc1(0, oc230_gp(c, 0u32.wrapping_sub(0x2dc4)));
            c.c_eq_s(23, 0);
            let untouched = c.fcc();
            c.g[V0][0] = 1;
            if untouched {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 16;
        frame.restore(c, S0, 0x120);
        frame.restore(c, S1, 0x110);
        frame.restore(c, S2, 0x100);
        frame.restore(c, S3, 0xf0);
        frame.restore(c, S4, 0xe0);
        frame.restore(c, S5, 0xd0);
        frame.restore(c, S6, 0xc0);
        frame.restore(c, S7, 0xb0);
        frame.restore(c, FP, 0xa0);
        frame.restore_ra(c, 0x90);
        frame.restore_fpr(c, 23, 0x148);
        frame.restore_fpr(c, 22, 0x140);
        frame.restore_fpr(c, 21, 0x138);
        frame.restore_fpr(c, 20, 0x130);
        frame.close(c);
        Ok(())
    })
}

/// `SpatialHash_Move`'s entry, its end, and the loop heads it is re-entered
/// at after an unwind (the bucket scan's "not empty" and tail loop, the
/// new bucket's clear loop).
const SHM_ENTRY: u32 = 0x0032_8808;
const SHM_END: u32 = 0x0032_8c20;
const SHM_NONEMPTY: u32 = 0x0032_8ac0;
const SHM_TAIL: u32 = 0x0032_8af0;
const SHM_CLEAR: u32 = 0x0032_8ba8;

/// The callees of `SpatialHash_Move`.
mod oc230_shm_callee {
    use crate::call::Callee;
    use ssx3_ee as ee;
    pub const REMOVE: Callee = Callee { addr: 0x0032_84b8, f: ee::s10::SpatialHash_Remove };
    pub const INSERT_AT_KEY: Callee = Callee { addr: 0x0032_8660, f: ee::s10::SpatialHash_InsertAtKey };
    pub const MOVE: Callee = Callee { addr: 0x0032_8808, f: ee::s10::SpatialHash_Move };
    pub const DELETE: Callee = Callee { addr: 0x0031_7e50, f: ee::s10::__builtin_delete };
    pub const ALLOC_NAMED: Callee = Callee { addr: 0x0031_7d70, f: ee::s10::EA_MemAllocNamed };
    /// `memset` (another shard: through the function table).
    pub const MEMSET: u32 = 0x003e_6448;
}

/// Where the emptiness scan of the old child's bucket array starts (or
/// resumes).
#[derive(Clone, Copy, PartialEq)]
enum Oc230Scan {
    /// The eight unrolled words (right after `SpatialHash_Remove`).
    Words,
    /// A non-zero word was found in the tail loop.
    NonEmpty,
    /// The loop over the last three words (a0 counter, v1 pointer).
    Tail,
}

/// `sx(a + b)` of two GPR low words.
#[inline(always)]
fn oc230_add(c: &Cpu, a: usize, b: usize) -> u64 {
    sx(r32(c, a).wrapping_add(r32(c, b)))
}

/// `sx(reg << n)`.
#[inline(always)]
fn oc230_sll(c: &Cpu, reg: usize, n: u32) -> u64 {
    sx(r32(c, reg) << n)
}

/// `sx(reg >> n)` (arithmetic, `srav`).
#[inline(always)]
fn oc230_sra(c: &Cpu, reg: usize, n: usize) -> u64 {
    sx(((r32(c, reg) as i32) >> (r32(c, n) & 31)) as u32)
}

/// `SpatialHash_Move` epilogue (s0 restored before).
fn oc230_shm_exit(c: &mut Cpu, frame: Frame) {
    c.cycles += 11;
    frame.restore(c, S1, 0xb0);
    frame.restore(c, S2, 0xa0);
    frame.restore(c, S3, 0x90);
    frame.restore(c, S4, 0x80);
    frame.restore(c, S5, 0x70);
    frame.restore(c, S6, 0x60);
    frame.restore(c, S7, 0x50);
    frame.restore(c, FP, 0x40);
    frame.restore_ra(c, 0x30);
    frame.close(c);
}

/// `void SpatialHash_Move(SpatialHash* h /*a0*/, int key /*a1*/, Link* e
/// /*a2*/, const Key* old /*a3*/, const Key* new /*t0*/, const Key* cell
/// /*t1*/)` (0x328808): moves entry e of an octree cell (level, x, y, z)
/// from the old key to the new one. New key at the cell's level: remove
/// and push e on the bucket list `h->buckets[key]` (+0x20). Old key at the
/// cell's level: unlink e from its bucket and insert it at the new key.
/// Otherwise both keys are in child cells: the same child recurses with
/// the child key (level - 1, coordinates 2 * cell + octant); different
/// children: remove from the old child (freeing its hash when all eleven
/// bucket words are empty), then insert into the new child, allocating
/// and clearing its hash (0x2c bytes) when absent. Dispatcher-shaped in the
/// original: re-entered at the scan and clear loop heads after an unwind.
pub fn spatial_hash_move(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0xd0 };
        match c.pc {
            SHM_ENTRY => oc230_shm_entry(c, frame),
            SHM_NONEMPTY => oc230_shm_after_remove(c, frame, Oc230Scan::NonEmpty),
            SHM_TAIL => oc230_shm_after_remove(c, frame, Oc230Scan::Tail),
            SHM_CLEAR => oc230_shm_clear_insert(c, frame),
            pc => {
                c.jump_out(pc, SHM_ENTRY, SHM_END);
                Ok(())
            }
        }
    })
}

fn oc230_shm_entry(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    use oc230_shm_callee as callee;
    c.cycles += 20;
    Frame::open(c, 0xd0);
    c.g[T3][0] = c.g[A3][0];
    frame.save(c, S0, 0xc0);
    frame.save(c, S2, 0xa0);
    c.g[S0][0] = c.g[T1][0];
    frame.save(c, S3, 0x90);
    c.g[S2][0] = c.g[A2][0];
    frame.save(c, S7, 0x50);
    c.g[S3][0] = c.g[A0][0];
    frame.save(c, FP, 0x40);
    frame.save(c, S1, 0xb0);
    c.g[FP][0] = c.g[T0][0];
    frame.save(c, S4, 0x80);
    frame.save(c, S5, 0x70);
    frame.save(c, S6, 0x60);
    frame.save_ra(c, 0x30);
    c.g[V1][0] = c.lw(r32(c, FP));
    c.g[T4][0] = c.lw(r32(c, S0));
    c.g[S7][0] = c.g[A1][0];
    let entry = r32(c, S2);

    if c.g[V1][0] == c.g[T4][0] {
        // The new key is at the cell's level: remove, push on the bucket.
        c.cycles += 8;
        c.g[T0][0] = c.g[S0][0];
        callee::REMOVE.call(c, 0x0032_8860)?;
        c.g[V0][0] = oc230_sll(c, S7, 2);
        c.g[A0][0] = oc230_add(c, V0, S3);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x20));
        let head = r32(c, A0);
        c.g[V0][0] = c.lw(head);
        if c.g[V0][0] != 0 {
            let v = c.g[S2][0];
            c.sw(oc230_at(c, V0, 4), v);
        }
        c.cycles += 5;
        c.g[V0][0] = c.lw(head);
        c.sw(entry.wrapping_add(4), 0);
        let v = c.g[V0][0];
        c.sw(entry, v);
        let v = c.g[S2][0];
        c.sw(head, v);
        c.cycles += 1;
        frame.restore(c, S0, 0xc0);
        oc230_shm_exit(c, frame);
        return Ok(());
    }

    c.cycles += 3;
    c.g[V0][0] = c.lw(r32(c, T3));
    let old_at_cell = c.g[V0][0] == c.g[T4][0];
    c.g[A2][0] = sx(r32(c, T4).wrapping_sub(r32(c, V0)));
    if old_at_cell {
        // The old key is at the cell's level: unlink from its bucket,
        // insert at the new key.
        c.cycles += 5;
        c.g[V0][0] = oc230_sll(c, S7, 2);
        c.g[A0][0] = c.lw(entry);
        c.g[V1][0] = oc230_add(c, V0, S3);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x20));
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = c.lw(entry.wrapping_add(4));
            let v = c.g[V0][0];
            c.sw(oc230_at(c, A0, 4), v);
        }
        c.cycles += 3;
        c.g[A0][0] = c.lw(entry.wrapping_add(4));
        c.g[V0][0] = c.lw(entry);
        let v = c.g[V0][0];
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            c.sw(r32(c, A0), v);
        } else {
            c.cycles += 1;
            c.sw(r32(c, V1), v);
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S7][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A3][0] = c.g[FP][0];
        c.g[T0][0] = c.g[S0][0];
        callee::INSERT_AT_KEY.call(c, 0x0032_88e4)?;
        frame.restore(c, S0, 0xc0);
        oc230_shm_exit(c, frame);
        return Ok(());
    }

    // Both keys below the cell: their child octants. Old: (t6, v0, t7) =
    // (old >> (level diff - 1)) - 2 * cell; new: (s4, s5, s6) likewise.
    c.cycles += 28;
    let (cell, old, new) = (r32(c, S0), r32(c, T3), r32(c, FP));
    c.g[A3][0] = sx(r32(c, T4).wrapping_sub(r32(c, V1)));
    c.g[V0][0] = c.lw(cell.wrapping_add(4));
    c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
    c.g[V1][0] = c.lw(old.wrapping_add(4));
    c.g[A3][0] = sx(r32(c, A3).wrapping_sub(1));
    c.g[T5][0] = oc230_sll(c, V0, 1);
    c.g[A0][0] = c.lw(cell.wrapping_add(8));
    c.g[V1][0] = oc230_sra(c, V1, A2);
    c.g[V0][0] = c.lw(old.wrapping_add(0xc));
    c.g[T6][0] = sx(r32(c, V1).wrapping_sub(r32(c, T5)));
    c.g[T2][0] = oc230_sll(c, A0, 1);
    c.g[V1][0] = c.lw(cell.wrapping_add(0xc));
    c.g[V0][0] = oc230_sra(c, V0, A2);
    c.g[T0][0] = c.lw(old.wrapping_add(8));
    c.g[T1][0] = oc230_sll(c, V1, 1);
    c.g[A1][0] = c.lw(new.wrapping_add(4));
    c.g[A0][0] = c.lw(new.wrapping_add(8));
    c.g[T7][0] = sx(r32(c, V0).wrapping_sub(r32(c, T1)));
    c.g[V1][0] = c.lw(new.wrapping_add(0xc));
    c.g[T0][0] = oc230_sra(c, T0, A2);
    c.g[A1][0] = oc230_sra(c, A1, A3);
    c.g[A0][0] = oc230_sra(c, A0, A3);
    c.g[V1][0] = oc230_sra(c, V1, A3);
    c.g[V0][0] = sx(r32(c, T0).wrapping_sub(r32(c, T2)));
    c.g[S4][0] = sx(r32(c, A1).wrapping_sub(r32(c, T5)));
    c.g[S5][0] = sx(r32(c, A0).wrapping_sub(r32(c, T2)));
    c.g[S6][0] = sx(r32(c, V1).wrapping_sub(r32(c, T1)));
    let same_child = if c.g[T6][0] != c.g[S4][0] {
        c.cycles += 1;
        false
    } else {
        c.cycles += 2;
        if c.g[V0][0] != c.g[S5][0] {
            false
        } else {
            c.cycles += 2;
            c.g[T7][0] == c.g[S6][0]
        }
    };
    if same_child {
        // Recurse into the child with the child key (sp+0).
        c.cycles += 23;
        let sp = sp(c);
        c.g[V0][0] = oc230_sll(c, S4, 4);
        c.g[V1][0] = oc230_sll(c, S5, 3);
        c.g[V0][0] = oc230_add(c, S3, V0);
        c.g[A3][0] = oc230_add(c, T1, S6);
        c.g[A0][0] = sx(r32(c, T4).wrapping_sub(1));
        c.g[A1][0] = oc230_add(c, T5, S4);
        c.g[A2][0] = oc230_add(c, T2, S5);
        c.g[V0][0] = oc230_add(c, V0, V1);
        let v = c.g[A1][0];
        c.sw(sp.wrapping_add(4), v);
        c.g[V1][0] = oc230_sll(c, S6, 2);
        let v = c.g[A2][0];
        c.sw(sp.wrapping_add(8), v);
        c.g[V0][0] = oc230_add(c, V0, V1);
        let v = c.g[A3][0];
        c.sw(sp.wrapping_add(0xc), v);
        c.g[A1][0] = c.g[S7][0];
        let v = c.g[A0][0];
        c.sw(sp, v);
        c.g[A2][0] = c.g[S2][0];
        c.g[A3][0] = c.g[T3][0];
        c.g[T0][0] = c.g[FP][0];
        c.g[A0][0] = c.lw(r32(c, V0));
        c.g[T1][0] = c.g[SP][0];
        callee::MOVE.call(c, 0x0032_89c0)?;
        frame.restore(c, S0, 0xc0);
        oc230_shm_exit(c, frame);
        return Ok(());
    }

    // Different children: remove from the old child (key at sp+0, its
    // hash slot in s1).
    c.g[A3][0] = c.lw(cell.wrapping_add(8));
    c.cycles += 30;
    let sp = sp(c);
    c.g[A2][0] = oc230_sll(c, T6, 4);
    c.g[A0][0] = c.lw(cell.wrapping_add(4));
    c.g[A2][0] = oc230_add(c, S3, A2);
    c.g[A1][0] = c.lw(cell.wrapping_add(0xc));
    c.g[A3][0] = oc230_sll(c, A3, 1);
    c.g[V1][0] = c.lw(cell);
    c.g[A3][0] = oc230_add(c, A3, V0);
    c.g[V0][0] = oc230_sll(c, V0, 3);
    c.g[A0][0] = oc230_sll(c, A0, 1);
    c.g[A1][0] = oc230_sll(c, A1, 1);
    c.g[A2][0] = oc230_add(c, A2, V0);
    c.g[A0][0] = oc230_add(c, A0, T6);
    c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
    c.g[A1][0] = oc230_add(c, A1, T7);
    let v = c.g[V1][0];
    c.sw(sp, v);
    c.g[V0][0] = oc230_sll(c, T7, 2);
    let v = c.g[A3][0];
    c.sw(sp.wrapping_add(8), v);
    let v = c.g[A1][0];
    c.sw(sp.wrapping_add(0xc), v);
    c.g[S1][0] = oc230_add(c, A2, V0);
    let v = c.g[A0][0];
    c.sw(sp.wrapping_add(4), v);
    c.g[A3][0] = c.g[T3][0];
    c.g[A1][0] = c.g[S7][0];
    c.g[A2][0] = c.g[S2][0];
    c.g[A0][0] = c.lw(r32(c, S1));
    c.g[T0][0] = c.g[SP][0];
    callee::REMOVE.call(c, 0x0032_8a34)?;
    oc230_shm_after_remove(c, frame, Oc230Scan::Words)
}

/// After the remove from the old child: is its hash (`*s1`, eleven words)
/// empty? (Also stores the new child's offsets at sp+0x20/0x24/0x28 and the
/// key pointer sp+0x10 at sp+0x2c.) Then free it if so and insert into the
/// new child.
fn oc230_shm_after_remove(c: &mut Cpu, frame: Frame, mut probe: Oc230Scan) -> crate::call::Flow {
    use oc230_shm_callee as callee;
    let sp = sp(c);
    // The new child's offsets (x * 16, y * 8, z * 4) and the key pointer.
    let offsets = |c: &mut Cpu, reg: usize| {
        c.g[A3][0] = oc230_sll(c, S5, 3);
        let v = c.g[reg][0];
        c.sw(sp.wrapping_add(0x20), v);
        let v = c.g[A3][0];
        c.sw(sp.wrapping_add(0x24), v);
        c.g[reg][0] = oc230_sll(c, S6, 2);
        c.g[A3][0] = sx(sp.wrapping_add(0x10));
        let v = c.g[reg][0];
        c.sw(sp.wrapping_add(0x28), v);
        let v = c.g[A3][0];
        c.sw(sp.wrapping_add(0x2c), v);
    };
    loop {
        match probe {
            Oc230Scan::Words => {
                c.g[V1][0] = c.lw(r32(c, S1));
                let words = r32(c, V1);
                let mut set = false;
                for k in 0..7u32 {
                    if k > 0 {
                        c.cycles += 3;
                    }
                    c.g[V0][0] = c.lw(words.wrapping_add(4 * k));
                    if c.g[V0][0] != 0 {
                        set = true;
                        break;
                    }
                }
                if !set {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(words.wrapping_add(0x1c));
                    let clear = c.g[V0][0] == 0;
                    c.g[V0][0] = oc230_sll(c, S4, 4);
                    if clear {
                        // The last three words in a loop.
                        c.cycles += 10;
                        offsets(c, V0);
                        c.g[V1][0] = sx(words.wrapping_add(0x20));
                        c.g[A0][0] = 0;
                        probe = Oc230Scan::Tail;
                        continue;
                    }
                    c.cycles += 1;
                }
                c.g[V1][0] = oc230_sll(c, S4, 4);
                c.cycles += 9;
                c.g[V0][0] = 0;
                offsets(c, V1);
                break;
            }
            Oc230Scan::NonEmpty => {
                c.cycles += 2;
                c.g[V0][0] = 0;
                break;
            }
            Oc230Scan::Tail => {
                c.cycles += 3;
                c.g[V0][0] = c.lw(r32(c, V1));
                let set = c.g[V0][0] != 0;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                if set {
                    if crate::events::back_edge_unwinds(c, SHM_NONEMPTY) {
                        return Ok(());
                    }
                    probe = Oc230Scan::NonEmpty;
                    continue;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A0][0] as i64) < 3) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
                if more {
                    if crate::events::back_edge_unwinds(c, SHM_TAIL) {
                        return Ok(());
                    }
                    continue;
                }
                c.cycles += 1;
                c.g[V0][0] = 1;
                break;
            }
        }
    }

    // Free the old child's hash when empty.
    c.cycles += 2;
    let empty = c.g[V0][0] != 0;
    c.g[V0][0] = c.lw(sp.wrapping_add(0x20));
    if empty {
        c.cycles += 4;
        c.g[A0][0] = c.lw(r32(c, S1));
        callee::DELETE.call(c, 0x0032_8b1c)?;
        c.sw(r32(c, S1), 0);
        c.g[V0][0] = c.lw(sp.wrapping_add(0x20));
    }

    // The new child's key (sp+0x10) and hash slot (s1).
    c.cycles += 23;
    let cell = r32(c, S0);
    c.g[V1][0] = c.lw(sp.wrapping_add(0x24));
    c.g[A2][0] = oc230_add(c, S3, V0);
    c.g[A1][0] = c.lw(cell.wrapping_add(0xc));
    c.g[A0][0] = c.lw(cell.wrapping_add(4));
    c.g[A2][0] = oc230_add(c, A2, V1);
    c.g[V0][0] = c.lw(cell.wrapping_add(8));
    c.g[A1][0] = oc230_sll(c, A1, 1);
    c.g[V1][0] = c.lw(cell);
    c.g[A0][0] = oc230_sll(c, A0, 1);
    c.g[V0][0] = oc230_sll(c, V0, 1);
    c.g[A0][0] = oc230_add(c, A0, S4);
    c.g[V0][0] = oc230_add(c, V0, S5);
    c.g[A1][0] = oc230_add(c, A1, S6);
    c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
    c.g[A3][0] = c.lw(sp.wrapping_add(0x28));
    let v = c.g[V1][0];
    c.sw(sp.wrapping_add(0x10), v);
    let v = c.g[A0][0];
    c.sw(sp.wrapping_add(0x14), v);
    c.g[S1][0] = oc230_add(c, A2, A3);
    let v = c.g[V0][0];
    c.sw(sp.wrapping_add(0x18), v);
    let v = c.g[A1][0];
    c.sw(sp.wrapping_add(0x1c), v);
    c.g[V0][0] = c.lw(r32(c, S1));
    if c.g[V0][0] != 0 {
        c.g[A0][0] = c.lw(r32(c, S1));
        return oc230_shm_insert(c, frame);
    }
    // No hash there yet: allocate one (0x2c bytes), clear it.
    c.cycles += 10;
    c.g[A1][0] = sx(0x004a_3fc0);
    c.g[A0][0] = 0x2c;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    callee::ALLOC_NAMED.call(c, 0x0032_8b98)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = u64::MAX;
    c.g[V0][0] = sx(r32(c, S0).wrapping_add(0x20));
    c.g[V1][0] = 2;
    oc230_shm_clear_insert(c, frame)
}

/// The new hash (s0): words 8..10 zeroed in a loop (v0 pointer, v1 count
/// down to a0 = -1), `memset(h, 0, 0x20)`, stored in the slot (s1); then
/// the insert.
fn oc230_shm_clear_insert(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    loop {
        c.cycles += 7;
        c.sw(r32(c, V0), 0);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
        let more = c.g[V1][0] != c.g[A0][0];
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
        if !more {
            break;
        }
        if crate::events::back_edge_unwinds(c, SHM_CLEAR) {
            return Ok(());
        }
    }
    c.cycles += 6;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x20;
    call_ptr(c, oc230_shm_callee::MEMSET, 0x0032_8bd4)?;
    let v = c.g[S0][0];
    c.sw(r32(c, S1), v);
    c.g[A0][0] = c.lw(r32(c, S1));
    oc230_shm_insert(c, frame)
}

/// `SpatialHash_InsertAtKey(*slot, key, e, new, child key)`, epilogue.
fn oc230_shm_insert(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 5;
    c.g[A1][0] = c.g[S7][0];
    c.g[T0][0] = c.lw(sp(c).wrapping_add(0x2c));
    c.g[A2][0] = c.g[S2][0];
    c.g[A3][0] = c.g[FP][0];
    oc230_shm_callee::INSERT_AT_KEY.call(c, 0x0032_8bf0)?;
    c.cycles += 1;
    frame.restore(c, S0, 0xc0);
    oc230_shm_exit(c, frame);
    Ok(())
}

/// `u32 SND_UDiv64by32(u64 x /*a0*/, u32 d /*a1*/)` (0x3c64a8): the
/// quotient of x / d by 32 steps of shift-and-subtract long division (x
/// goes through the stack to split it into lo/hi words; the remainder stays
/// in v1).
pub fn snd_udiv64by32(c: &mut Cpu) {
    c.cycles += 8;
    let frame = Frame::open(c, 0x10);
    let s = sp(c);
    c.g[A2][0] = 0; // quotient
    let x = c.g[A0][0];
    c.sd(s, x);
    c.g[A3][0] = 0x1f; // bit counter
    c.g[A0][0] = c.lw(s); // lo
    c.g[V1][0] = c.lw(s.wrapping_add(4)); // hi (the running remainder)
    c.g[V0][0] = sx(r32(c, A0) >> 31);
    loop {
        c.cycles += 6;
        // (hi:lo) <<= 1, q <<= 1
        c.g[V1][0] = sx((r32(c, V1) << 1).wrapping_add(r32(c, V0)));
        c.g[A2][0] = sx(r32(c, A2) << 1);
        c.g[V0][0] = (c.g[V1][0] < c.g[A1][0]) as u64;
        c.g[A0][0] = sx(r32(c, A0) << 1);
        if c.g[V0][0] == 0 {
            // hi >= d: subtract, set the quotient bit.
            c.cycles += 2;
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(r32(c, A1)));
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
        }
        c.cycles += 3;
        c.g[A3][0] = sx(r32(c, A3).wrapping_sub(1));
        let more = (c.g[A3][0] as i64) >= 0;
        c.g[V0][0] = sx(r32(c, A0) >> 31);
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_64c8);
        }
    }
    c.cycles += 3;
    c.g[V0][0] = c.g[A2][0];
    frame.close(c);
}

/// The `cCSICubicSpline` fields: word 0 packs the segment count (bits
/// 11..18) and the current segment (bits 19..26); the end time at +4 and
/// the segment array at +8 (0x14 bytes each: four coefficients, start time
/// at +0x10).
mod oc230_spline {
    pub const END_TIME: u32 = 0x4;
    pub const SEGMENTS: u32 = 0x8;
    pub const SEG_SIZE: u32 = 0x14;
    pub const SEG_START: u32 = 0x10;
    /// Word 0 with the current-segment field cleared.
    pub const CUR_CLEAR: u64 = 0xffff_ffff_f807_ffff;
    pub const EVAL_SEGMENT: crate::call::Callee = crate::call::Callee {
        addr: 0x0031_d5a8,
        f: ssx3_ee::s10::cCSICubicSpline_EvalSegment,
    };
}

/// `float cCSICubicSpline_Evaluate(cCSICubicSpline* s, float t)`
/// (0x31dee0): evaluates the segment holding t (`EvalSegment`, result in
/// f0). The current segment is tried first (start <= t <= next start, or
/// the end time for the last); else t before it: segment 0 when t is before
/// segment 1; t after it: the last segment when the current is the one
/// before it, the next one when t is before the one after; otherwise a
/// binary search over the start times. The found segment becomes current.
pub fn c_csi_cubic_spline_evaluate(c: &mut Cpu) {
    use oc230_spline as sp_;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let spline = r32(c, A2);
        c.g[V0][0] = c.lw(spline);
        c.g[V1][0] = sx(r32(c, V0) >> 11) & 0xff; // count
        c.g[V0][0] = sx(r32(c, V0) >> 19);
        c.g[A0][0] = c.g[V0][0] & 0xff; // current
        c.g[A3][0] = sx(r32(c, V1).wrapping_sub(1)); // last
        let is_last = c.g[A3][0] == c.g[A0][0];
        c.g[V0][0] = sp_::SEG_SIZE as u64;
        // a1 = the current segment's upper bound (the next start or the
        // end time), t2 = the segments.
        if !is_last {
            c.cycles += 5;
            c.g[V1][0] = c.lw(spline.wrapping_add(sp_::SEGMENTS));
            c.mult(0, V0, A0, V0);
            c.g[T2][0] = c.g[V1][0];
            c.g[V0][0] = oc230_add(c, V0, V1);
            c.g[A1][0] = sx(r32(c, V0).wrapping_add(sp_::SEG_SIZE + sp_::SEG_START));
        } else {
            c.cycles += 3;
            c.g[A1][0] = sx(spline.wrapping_add(sp_::END_TIME));
            c.g[T2][0] = c.lw(spline.wrapping_add(sp_::SEGMENTS));
        }
        let segs = r32(c, T2);

        // The current segment?
        c.cycles += 11;
        c.g[V0][0] = c.lw(spline);
        c.g[V1][0] = sp_::SEG_SIZE as u64;
        c.g[T1][0] = c.g[V0][0];
        c.g[V0][0] = sx(r32(c, V0) >> 19) & 0xff;
        c.mult(0, T0, V0, V1);
        c.g[A0][0] = oc230_add(c, T0, T2);
        let cur = r32(c, A0);
        c.lwc1(0, cur.wrapping_add(sp_::SEG_START));
        c.c_le_s(0, 12);
        if c.fcc() {
            c.cycles += 5;
            c.lwc1(0, r32(c, A1));
            c.c_le_s(12, 0);
            if c.fcc() {
                c.cycles += 4;
                sp_::EVAL_SEGMENT.call(c, 0x0031_df74)?;
                return oc230_spline_exit(c, frame);
            }
            c.lwc1(0, cur.wrapping_add(sp_::SEG_START));
        }

        c.cycles += 4;
        c.c_lt_s(12, 0);
        let before = c.fcc();
        c.g[V0][0] = sx(r32(c, T1) >> 19);
        let mut search = false;
        if before {
            // Before the current one: segment 0 when t < segment 1's start.
            c.cycles += 5;
            c.lwc1(0, segs.wrapping_add(sp_::SEG_SIZE + sp_::SEG_START));
            c.c_lt_s(12, 0);
            c.g[A0][0] = c.g[T2][0];
            if c.fcc() {
                c.cycles += 7;
                c.g[V0][0] = c.g[T1][0] & sp_::CUR_CLEAR;
                let v = c.g[V0][0];
                c.sw(spline, v);
                sp_::EVAL_SEGMENT.call(c, 0x0031_dfb4)?;
                return oc230_spline_exit(c, frame);
            }
            search = true;
        }
        if !search {
            // After it: the last segment when the current is the one before.
            c.cycles += 4;
            c.g[V1][0] = sx(r32(c, A3).wrapping_sub(1));
            c.g[A0][0] = c.g[V0][0] & 0xff;
            let next_is_last = c.g[A0][0] == c.g[V1][0];
            c.g[A1][0] = sp_::SEG_SIZE as u64;
            if next_is_last {
                c.cycles += 14;
                c.g[A0][0] = sp_::SEG_SIZE as u64;
                c.g[V0][0] = 0xffff_ffff_f807_0000;
                c.mult(0, A0, A3, A0);
                c.g[V1][0] = c.g[A3][0] & 0xff;
                c.g[V0][0] |= 0xffff;
                c.g[V1][0] = oc230_sll(c, V1, 19);
                c.g[V0][0] &= c.g[T1][0];
                c.g[A1][0] = c.lw(spline.wrapping_add(sp_::SEGMENTS));
                c.g[V0][0] |= c.g[V1][0];
                c.g[A0][0] = oc230_add(c, A1, A0);
                let v = c.g[V0][0];
                c.sw(spline, v);
                sp_::EVAL_SEGMENT.call(c, 0x0031_dffc)?;
                return oc230_spline_exit(c, frame);
            }
            // The next one when t is before the start of the one after it.
            c.cycles += 7;
            c.mult(0, V1, A0, A1);
            c.g[V0][0] = oc230_add(c, V1, T2);
            c.lwc1(0, r32(c, V0).wrapping_add(2 * sp_::SEG_SIZE + sp_::SEG_START));
            c.c_lt_s(12, 0);
            let in_next = c.fcc();
            c.g[V0][0] = sx(r32(c, A0).wrapping_add(1));
            if in_next {
                c.cycles += 14;
                c.g[V1][0] = 0xffff_ffff_f807_0000;
                c.g[V0][0] &= 0xff;
                c.g[V1][0] |= 0xffff;
                c.g[V0][0] = oc230_sll(c, V0, 19);
                c.g[V1][0] &= c.g[T1][0];
                c.g[V1][0] |= c.g[V0][0];
                c.g[A0][0] = sx(r32(c, V1) >> 19);
                let v = c.g[V1][0];
                c.sw(spline, v);
                c.g[A0][0] &= 0xff;
                c.mult(0, V0, A0, A1);
                c.g[A0][0] = oc230_add(c, V0, T2);
                sp_::EVAL_SEGMENT.call(c, 0x0031_e050)?;
                return oc230_spline_exit(c, frame);
            }
        }

        // Binary search: lo (t0) .. hi (a1) over the segment starts.
        c.cycles += 2;
        c.g[A1][0] = c.g[A3][0];
        c.g[T0][0] = 0;
        loop {
            c.cycles += 10;
            c.g[V1][0] = oc230_add(c, A1, T0);
            c.g[A0][0] = sp_::SEG_SIZE as u64;
            c.g[V0][0] = sx(r32(c, V1) >> 31);
            c.g[V1][0] = oc230_add(c, V1, V0);
            c.g[V1][0] = sx(((r32(c, V1) as i32) >> 1) as u32); // mid
            c.mult(0, V0, V1, A0);
            c.g[A0][0] = oc230_add(c, V0, T2);
            let seg = r32(c, A0);
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A3][0] as i64)) as u64;
            c.lwc1(1, seg.wrapping_add(sp_::SEG_START));
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.lwc1(0, seg.wrapping_add(sp_::SEG_SIZE + sp_::SEG_START));
            } else {
                c.cycles += 1;
                c.lwc1(0, spline.wrapping_add(sp_::END_TIME));
            }
            c.cycles += 4;
            c.c_le_s(1, 12);
            if !c.fcc() {
                // t before mid: hi = mid.
                c.g[A1][0] = c.g[V1][0];
            } else {
                c.cycles += 4;
                c.c_le_s(12, 0);
                let inside = c.fcc();
                c.g[T0][0] = sx(r32(c, V1).wrapping_add(1));
                if inside {
                    c.cycles += 9;
                    c.g[V0][0] = 0xffff_ffff_f807_0000;
                    c.g[V1][0] &= 0xff;
                    c.g[V0][0] |= 0xffff;
                    c.g[V1][0] = oc230_sll(c, V1, 19);
                    c.g[V0][0] &= c.g[T1][0];
                    c.g[V0][0] |= c.g[V1][0];
                    let v = c.g[V0][0];
                    c.sw(spline, v);
                    sp_::EVAL_SEGMENT.call(c, 0x0031_e0d4)?;
                    return oc230_spline_exit(c, frame);
                }
                // t after mid's end: lo = mid + 1.
            }
            if event_due(c) {
                deliver_event_here(c, 0x0031_e060);
            }
        }
    })
}

/// `cCSICubicSpline_Evaluate`'s exit after the segment call.
fn oc230_spline_exit(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    frame.restore_ra(c, 0);
    c.cycles += 2;
    frame.close(c);
    Ok(())
}

/// The libm callees of `tanf`.
mod oc230_tanf {
    use crate::call::Callee;
    pub const REM_PIO2F: Callee = Callee { addr: 0x0040_e878, f: ssx3_ee::s15::__ieee754_rem_pio2f };
    pub const KERNEL_TANF: Callee = Callee { addr: 0x0040_f5a8, f: ssx3_ee::s15::__kernel_tanf };
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

fn oc230_tanf_exit(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 3;
    frame.restore_ra(c, 0x10);
    frame.close(c);
    Ok(())
}

/// The callees of `DeferMan_SortBuckets`.
mod oc230_defer {
    use crate::call::Callee;
    pub const SPR_STREAM_BEGIN_BLOCK: Callee = Callee { addr: 0x0038_f460, f: ssx3_ee::s12::SprStream_BeginBlock };
    pub const SPR_STREAM_NEXT_INDEX: Callee = Callee { addr: 0x0038_f6a8, f: ssx3_ee::s12::SprStream_NextIndex };
    /// `memset` (another shard: through the function table).
    pub const MEMSET: u32 = 0x003e_6448;
    /// The item count and the items (key, value: 8 bytes each).
    pub const COUNT: u32 = 0x6_7ca4;
    pub const ITEMS: u32 = 0x6_7ca8;
    /// The second item buffer (global).
    pub const SCRATCH: u32 = 0x0050_5b70;
}

/// `void DeferMan_SortBuckets(DeferMan* m)` (0x364050): LSD radix sort of
/// the deferred draw items (8 bytes, sorted by the 32-bit key) by bytes:
/// four 257-entry histograms (one per key byte) in a scratchpad stream
/// block (gp+0x1384), counted in one pass and prefix-summed, then four
/// scatter passes ping-ponging between the items and the global buffer
/// (an even number, so the result ends in the items); then the stream
/// moves to its next block.
pub fn defer_man_sort_buckets(c: &mut Cpu) {
    use oc230_defer as d;
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x60);
        c.g[V1][0] = d::ITEMS as u64;
        frame.save(c, S4, 0x10);
        frame.save(c, S0, 0x50);
        c.g[A1][0] = 0x50_0000;
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        c.g[V0][0] = sx(r32(c, A0).wrapping_add(0x6_0000));
        frame.save_ra(c, 0);
        c.g[S1][0] = oc230_add(c, A0, V1); // src = the items
        c.g[S4][0] = c.lw(oc230_gp(c, 0x1384)); // the scratchpad stream
        c.g[S3][0] = sx(d::SCRATCH); // dst
        frame.save(c, S2, 0x30);
        c.g[A1][0] = 0;
        c.g[S2][0] = c.lw(r32(c, V0).wrapping_add(d::COUNT - 0x6_0000)); // n
        c.g[A0][0] = c.g[S4][0];
        c.g[A2][0] = u64::MAX;
        c.g[A3][0] = 0;
        d::SPR_STREAM_BEGIN_BLOCK.call(c, 0x0036_40a4)?;
        c.g[S0][0] = c.g[V0][0]; // the histograms
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0x1010;
        call_ptr(c, d::MEMSET, 0x0036_40b8)?;
        let hist = r32(c, S0);

        // Count the four key bytes (histogram k at +0x404 * k, entry b + 1).
        let empty = (c.g[S2][0] as i64) <= 0;
        c.g[T0][0] = c.g[S1][0];
        if !empty {
            c.cycles += 2;
            c.g[A3][0] = c.g[S2][0];
            loop {
                c.cycles += 28;
                let item = r32(c, T0);
                c.g[A0][0] = c.lw(item);
                c.g[A3][0] = sx(r32(c, A3).wrapping_sub(1));
                c.g[T0][0] = sx(item.wrapping_add(8));
                c.g[V1][0] = c.g[A0][0] & 0xff;
                c.g[A2][0] = sx(r32(c, A0) >> 6) & 0x3fc;
                c.g[V1][0] = oc230_sll(c, V1, 2);
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(hist));
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(hist));
                c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(4));
                c.g[A1][0] = sx(r32(c, A0) >> 14) & 0x3fc;
                c.g[A0][0] = sx(r32(c, A0) >> 24);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(hist));
                let v = c.g[V0][0];
                c.sw(r32(c, V1).wrapping_add(4), v);
                c.g[A0][0] = oc230_sll(c, A0, 2);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(hist));
                for (base, off, reg) in [(A2, 0x408u32, V0), (A1, 0x80c, V1), (A0, 0xc10, V0)] {
                    let at = r32(c, base).wrapping_add(off);
                    c.g[reg][0] = c.lw(at);
                    c.g[reg][0] = sx(r32(c, reg).wrapping_add(1));
                    let v = c.g[reg][0];
                    c.sw(at, v);
                }
                if c.g[A3][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0036_40c8);
                }
            }
        }

        // Prefix sums: hist[k][i] += hist[k][i - 1] for i = 1..255.
        c.cycles += 2;
        c.g[T2][0] = sx(hist.wrapping_add(4));
        c.g[T3][0] = 0xfe;
        loop {
            c.cycles += 19;
            let at = r32(c, T2);
            c.g[V1][0] = c.lw(at);
            c.g[T3][0] = sx(r32(c, T3).wrapping_sub(1));
            c.g[A0][0] = c.lw(at.wrapping_add(0x404));
            c.g[A1][0] = c.lw(at.wrapping_add(0x808));
            c.g[A2][0] = c.lw(at.wrapping_add(0xc0c));
            c.g[V0][0] = c.lw(at.wrapping_sub(4));
            c.g[A3][0] = c.lw(at.wrapping_add(0x400));
            c.g[T0][0] = c.lw(at.wrapping_add(0x804));
            c.g[V1][0] = oc230_add(c, V1, V0);
            c.g[T1][0] = c.lw(at.wrapping_add(0xc08));
            c.g[A0][0] = oc230_add(c, A0, A3);
            c.g[A1][0] = oc230_add(c, A1, T0);
            let v = c.g[V1][0];
            c.sw(at, v);
            c.g[A2][0] = oc230_add(c, A2, T1);
            let v = c.g[A0][0];
            c.sw(at.wrapping_add(0x404), v);
            let v = c.g[A1][0];
            c.sw(at.wrapping_add(0x808), v);
            let v = c.g[A2][0];
            c.sw(at.wrapping_add(0xc0c), v);
            let more = (c.g[T3][0] as i64) >= 0;
            c.g[T2][0] = sx(at.wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0036_4140);
            }
        }

        // Scatter by byte shift = 0, 8, 16, 24 (histogram shift / 8).
        c.cycles += 1;
        c.g[A3][0] = 0;
        loop {
            c.cycles += 7;
            c.g[V1][0] = oc230_sll(c, A3, 5);
            c.g[V0][0] = sx(((r32(c, A3) as i32) >> 3) as u32);
            c.g[V1][0] = oc230_add(c, V1, V0);
            c.g[T1][0] = sx(r32(c, A3).wrapping_add(8));
            c.g[V1][0] = oc230_sll(c, V1, 2);
            let empty = (c.g[S2][0] as i64) <= 0;
            c.g[T0][0] = sx(hist.wrapping_add(r32(c, V1)));
            if !empty {
                c.cycles += 3;
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.g[S2][0];
                loop {
                    // dst[h[(key >> shift) & 0xff]++] = *src++
                    c.cycles += 18;
                    let item = r32(c, A1);
                    c.g[V0][0] = c.lw(item);
                    c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
                    c.g[V0][0] = sx(r32(c, V0) >> (r32(c, A3) & 31)) & 0xff;
                    c.g[V0][0] = oc230_sll(c, V0, 2);
                    c.g[V0][0] = oc230_add(c, V0, T0);
                    let slot = r32(c, V0);
                    c.g[A0][0] = c.lw(slot);
                    c.g[V1][0] = c.g[A0][0];
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                    c.g[V1][0] = oc230_sll(c, V1, 3);
                    let v = c.g[A0][0];
                    c.sw(slot, v);
                    c.g[V1][0] = oc230_add(c, V1, S3);
                    let out = r32(c, V1);
                    c.ldl(V0, item.wrapping_add(7));
                    c.ldr(V0, item);
                    c.sdl(V0, out.wrapping_add(7));
                    c.sdr(V0, out);
                    let more = c.g[A2][0] != 0;
                    c.g[A1][0] = sx(item.wrapping_add(8));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0036_41b8);
                    }
                }
            }
            // Swap src and dst.
            c.cycles += 6;
            c.g[V1][0] = c.g[S3][0];
            c.g[A3][0] = c.g[T1][0];
            c.g[S3][0] = c.g[S1][0];
            c.g[V0][0] = ((c.g[A3][0] as i64) < 32) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S1][0] = c.g[V1][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0036_4190);
            }
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S4][0];
        d::SPR_STREAM_NEXT_INDEX.call(c, 0x0036_4220)?;
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

/// `Nash_CollideVolume` and the contact record layout (0x80 bytes: the
/// distance at +0x40, the object at +0x50 with its id at +0x78, the
/// instance at +0x54 with its id at +0x150).
mod oc230_nash {
    pub const COLLIDE_VOLUME: crate::call::Callee = crate::call::Callee {
        addr: 0x0033_5d78,
        f: ssx3_ee::s11::Nash_CollideVolume,
    };
    pub const HIT_SIZE: u32 = 0x80;
    pub const DIST: u32 = 0x40;
    pub const OBJECT: u32 = 0x50;
    pub const INSTANCE: u32 = 0x54;
    pub const OBJECT_ID: u32 = 0x78;
    pub const INSTANCE_ID: u32 = 0x150;
    /// The query's vtable and its reference-distance method.
    pub const QUERY_VTABLE: u32 = 0x50;
    pub const REF_DIST_SLOT: u32 = 0x18;
}

/// `float Nash_FindClosestContact(Nash* grid, Query* q, Contact* out, int
/// arg)` (0x336850): collects up to 64 contacts (`Nash_CollideVolume`
/// into a frame array, after the original's empty 64-step constructor
/// loop); none gives -1. Otherwise picks the contact whose distance is
/// closest to the query's reference distance (virtual 0x18); on a tie an
/// object contact beats an instance one, then the lower object id, or for
/// two instance contacts the lower instance id. Copies it to `out` and
/// returns its distance.
pub fn nash_find_closest_contact(c: &mut Cpu) {
    use oc230_nash as n;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x2050);
        c.g[V0][0] = 0x3f;
        frame.save(c, S0, 0x2040);
        c.g[V1][0] = u64::MAX;
        frame.save(c, S3, 0x2010);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x2030);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S2, 0x2020);
        frame.save_ra(c, 0x2000);
        // The contacts' (empty) constructor loop.
        loop {
            c.cycles += 7;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            let more = c.g[V0][0] != c.g[V1][0];
            c.g[T0][0] = c.g[A3][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0033_6878);
            }
        }
        c.cycles += 7;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[SP][0];
        c.g[A3][0] = 0x40;
        n::COLLIDE_VOLUME.call(c, 0x0033_68a4)?;
        c.g[S2][0] = c.g[V0][0];
        if c.g[S2][0] == 0 {
            c.cycles += 4;
            c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0
            c.mtc1(AT, 0);
            frame.restore(c, S0, 0x2040);
            return oc230_nash_exit(c, frame);
        }
        // The reference distance (f2).
        c.g[V0][0] = c.lw(oc230_at(c, S0, n::QUERY_VTABLE));
        c.cycles += 13;
        c.g[S1][0] = 0; // best
        c.g[A0][0] = c.lh(oc230_at(c, V0, n::REF_DIST_SLOT));
        c.g[V1][0] = c.lw(oc230_at(c, V0, n::REF_DIST_SLOT + 4));
        let t = r32(c, V1);
        c.g[A0][0] = oc230_add(c, S0, A0);
        call_ptr(c, t, 0x0033_68d4)?;
        let s = sp(c);
        c.mov_s(2, 0);
        c.g[A3][0] = 1;
        c.lwc1(0, s.wrapping_add(n::DIST));
        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[S2][0] as i64)) as u64;
        c.sub_s(0, 0, 2);
        c.abs_s(1, 0); // best |dist - ref|
        let more = c.g[V0][0] != 0;
        c.g[A2][0] = sx(s.wrapping_add(n::HIT_SIZE));
        if more {
            c.cycles += 1;
            loop {
                let hit = r32(c, A2);
                c.cycles += 6;
                c.lwc1(0, hit.wrapping_add(n::DIST));
                c.sub_s(0, 0, 2);
                c.abs_s(0, 0);
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.cycles += 3;
                    c.mov_s(1, 0);
                    c.g[S1][0] = c.g[A3][0];
                } else {
                    c.cycles += 4;
                    c.c_eq_s(0, 1);
                    let tie = c.fcc();
                    c.g[V0][0] = oc230_sll(c, S1, 7);
                    if tie {
                        // The tie break: is this contact better than the best?
                        c.cycles += 3;
                        c.g[A0][0] = c.lw(hit.wrapping_add(n::OBJECT));
                        c.g[V1][0] = oc230_add(c, V0, SP); // the best contact
                        let best = r32(c, V1);
                        if c.g[A0][0] != 0 {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(best.wrapping_add(n::OBJECT));
                            if c.g[V0][0] != 0 {
                                c.g[V1][0] = c.lw(oc230_at(c, A0, n::OBJECT_ID));
                                c.cycles += 3;
                                c.g[V0][0] = c.lw(oc230_at(c, V0, n::OBJECT_ID));
                                c.g[V0][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                            } else {
                                c.cycles += 2;
                                c.g[V0][0] = 1;
                            }
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(best.wrapping_add(n::OBJECT));
                            let best_is_object = c.g[V0][0] != 0;
                            c.g[V0][0] = 0;
                            if !best_is_object {
                                c.cycles += 5;
                                c.g[V1][0] = c.lw(best.wrapping_add(n::INSTANCE));
                                c.g[A0][0] = c.lw(hit.wrapping_add(n::INSTANCE));
                                c.g[A1][0] = c.lw(oc230_at(c, V1, n::INSTANCE_ID));
                                c.g[V0][0] = c.lw(oc230_at(c, A0, n::INSTANCE_ID));
                                c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
                            }
                        }
                        c.cycles += 1;
                        if c.g[V0][0] != 0 {
                            c.g[S1][0] = c.g[A3][0];
                        }
                    }
                }
                c.cycles += 4;
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[S2][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A2][0] = sx(hit.wrapping_add(n::HIT_SIZE));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0033_68f8);
                }
            }
        }
        // *out = contacts[best] (four quadwords), return its distance.
        c.cycles += 4;
        c.g[V0][0] = oc230_sll(c, S1, 7);
        c.g[V1][0] = c.g[S3][0];
        c.g[V0][0] = oc230_add(c, V0, SP);
        c.g[A0][0] = sx(r32(c, V0).wrapping_add(n::HIT_SIZE));
        loop {
            c.cycles += 7;
            let (from, to) = (r32(c, V0), r32(c, V1));
            c.lq(A1, from);
            c.lq(A2, from.wrapping_add(0x10));
            c.sq(A1, to);
            c.sq(A2, to.wrapping_add(0x10));
            c.g[V0][0] = sx(from.wrapping_add(0x20));
            let more = c.g[V0][0] != c.g[A0][0];
            c.g[V1][0] = sx(to.wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0033_699c);
            }
        }
        c.cycles += 2;
        c.lwc1(0, oc230_at(c, S3, n::DIST));
        frame.restore(c, S0, 0x2040);
        oc230_nash_exit(c, frame)
    })
}

fn oc230_nash_exit(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 6;
    frame.restore(c, S1, 0x2030);
    frame.restore(c, S2, 0x2020);
    frame.restore(c, S3, 0x2010);
    frame.restore_ra(c, 0x2000);
    frame.close(c);
    Ok(())
}

/// The callees and tuning globals of `cSplashManager_EmitPending`.
mod oc230_splash {
    use crate::call::Callee;
    pub const SPAWN_CRYSTALS: Callee = Callee { addr: 0x002f_3640, f: ssx3_ee::s09::cSplashManager_SpawnCrystals };
    pub const ADD_DROP: Callee = Callee { addr: 0x002f_37a8, f: ssx3_ee::s09::cSplashManager_AddDrop };
    /// `BXrand` (another shard: through the function table).
    pub const BX_RAND: u32 = 0x0031_77f0;
    /// The pending-emission accumulator (float) in the manager.
    pub const PENDING: u32 = 0x1024;
    /// gp offsets: the float-LCG seed, the crystal chance, the crystal
    /// count range [min, max), the drop size.
    pub const SEED: u32 = 0xa0c;
    pub const CRYSTAL_CHANCE: u32 = 0x116c;
    pub const CRYSTALS_MIN: u32 = 0x1174;
    pub const CRYSTALS_MAX: u32 = 0x1178;
    pub const DROP_SIZE: u32 = 0x11a8;
}

/// One step of the float LCG at gp+0xa0c: `seed = ((seed_reg * 0x18fcd +
/// 0xe9507c) & 0x7fffff) | 1.0` into `dst`, the product of the next step
/// into `next` (the original interleaves the multiplies).
fn oc230_splash_lcg(c: &mut Cpu, prod: usize, dst: usize) {
    c.g[dst][0] = oc230_add(c, prod, S2);
    c.g[dst][0] &= c.g[S1][0];
    c.g[dst][0] |= c.g[S5][0];
}

/// `void cSplashManager_EmitPending(cSplashManager* m)` (0x2f3810): takes
/// the whole part n of the pending-emission accumulator (+0x1024, keeping
/// the fraction) and emits until n is used up: a random screen position
/// (640 x 480, from the float LCG at gp+0xa0c), then with the crystal
/// chance a burst of `min + BXrand() % (max - min)` crystals (at least 1,
/// `SpawnCrystals`), else one drop of the configured size (`AddDrop`).
pub fn c_splash_manager_emit_pending(c: &mut Cpu) {
    use oc230_splash as sp_;
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S0, 0xa0);
        frame.save(c, S4, 0x60);
        frame.save(c, S1, 0x90);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S2, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save(c, S7, 0x30);
        frame.save_ra(c, 0x20);
        frame.save_fpr(c, 22, 0xc0);
        frame.save_fpr(c, 21, 0xb8);
        frame.save_fpr(c, 20, 0xb0);
        // n = (int)pending; pending -= n.
        let pending = r32(c, S4).wrapping_add(sp_::PENDING);
        c.lwc1(0, pending);
        c.cvt_w_s(1, 0);
        c.mfc1(S0, 1);
        c.mtc1(S0, 1);
        c.cvt_s_w(1, 1);
        c.sub_s(0, 0, 1);
        let none = (c.g[S0][0] as i64) <= 0;
        c.swc1(0, pending);
        if !none {
            c.cycles += 16;
            c.g[S3][0] = 0x1_8fcd; // LCG multiplier
            c.g[S2][0] = 0xe9_507c; // LCG increment
            c.g[S1][0] = 0x7f_ffff; // mantissa mask
            c.g[AT][0] = 0x4420_0000;
            c.mtc1(AT, 22); // 640
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 20); // 1
            c.g[AT][0] = 0x43f0_0000;
            c.mtc1(AT, 21); // 480
            c.g[S5][0] = 0x3f80_0000;
            c.g[S6][0] = 0;
            c.g[S7][0] = 1;
            c.g[V0][0] = c.lw(oc230_gp(c, sp_::SEED));
            loop {
                // pos = ((r1 - 1) * 640, (r2 - 1) * 480); roll r3 - 1.
                c.cycles += 29;
                let seed = oc230_gp(c, sp_::SEED);
                let s = sp(c);
                c.lwc1(2, oc230_gp(c, sp_::CRYSTAL_CHANCE));
                c.mult(0, V1, V0, S3);
                oc230_splash_lcg(c, V1, V0);
                c.mult(0, A0, V0, S3);
                let v = c.g[V0][0];
                c.sw(seed, v);
                c.lwc1(0, seed);
                oc230_splash_lcg(c, A0, V1);
                c.sub_s(0, 0, 20);
                c.mult(0, A0, V1, S3);
                let v = c.g[V1][0];
                c.sw(seed, v);
                c.mul_s(0, 0, 22);
                c.lwc1(1, seed);
                oc230_splash_lcg(c, A0, V0);
                c.swc1(0, s);
                c.sub_s(1, 1, 20);
                let v = c.g[V0][0];
                c.sw(seed, v);
                c.mul_s(1, 1, 21);
                c.lwc1(0, seed);
                c.sub_s(0, 0, 20);
                c.c_lt_s(0, 2);
                let crystals = c.fcc();
                c.swc1(1, s.wrapping_add(4));
                if crystals {
                    // k = max(min + BXrand() % (max - min), 1) crystals.
                    c.cycles += 10;
                    call_ptr(c, sp_::BX_RAND, 0x002f_3924)?;
                    c.g[A3][0] = c.lw(oc230_gp(c, sp_::CRYSTALS_MIN));
                    c.g[A0][0] = c.g[S4][0];
                    c.g[V1][0] = c.lw(oc230_gp(c, sp_::CRYSTALS_MAX));
                    c.g[A1][0] = c.g[SP][0];
                    c.g[V1][0] = sx(r32(c, V1).wrapping_sub(r32(c, A3)));
                    c.divu(0, V0, V1);
                    c.cycles += 9;
                    c.g[A2][0] = c.hi[0];
                    c.g[A3][0] = oc230_add(c, A3, A2);
                    c.g[V0][0] = ((c.g[S6][0] as i64) < (c.g[A3][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[A3][0] = c.g[S7][0];
                    }
                    c.g[A2][0] = c.g[A3][0];
                    c.g[S0][0] = sx(r32(c, S0).wrapping_sub(r32(c, A3)));
                    sp_::SPAWN_CRYSTALS.call(c, 0x002f_3960)?;
                } else {
                    // One drop of the configured size.
                    c.cycles += 8;
                    c.lwc1(0, oc230_gp(c, sp_::DROP_SIZE));
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A2][0] = sx(s.wrapping_add(0x10));
                    c.swc1(0, s.wrapping_add(0x14));
                    c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
                    c.swc1(0, s.wrapping_add(0x10));
                    sp_::ADD_DROP.call(c, 0x002f_3988)?;
                }
                c.cycles += 2;
                let more = (c.g[S0][0] as i64) > 0;
                c.g[V0][0] = c.lw(oc230_gp(c, sp_::SEED));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_38a8);
                }
            }
        }
        c.cycles += 14;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 22, 0xc0);
        frame.restore_fpr(c, 21, 0xb8);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    })
}

/// The rider fields `ComputerActorRider_FindSpeedMarker` reads.
mod oc230_cpu_rider {
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

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0039_0f20, vfx_screen_tint_render, "VfxScreenTint_Render"),
    (0x0032_aa28, rider_sphere_tree_collide_triangle2, "RiderSphereTree_CollideTriangle2"),
    (0x0032_8808, spatial_hash_move, "SpatialHash_Move"),
    (0x003c_64a8, snd_udiv64by32, "SND_UDiv64by32"),
    (0x0031_dee0, c_csi_cubic_spline_evaluate, "cCSICubicSpline_Evaluate"),
    (0x0040_d588, tanf, "tanf"),
    (0x0036_4050, defer_man_sort_buckets, "DeferMan_SortBuckets"),
    (0x0033_6850, nash_find_closest_contact, "Nash_FindClosestContact"),
    (0x002f_3810, c_splash_manager_emit_pending, "cSplashManager_EmitPending"),
    (0x0010_bb18, computer_actor_rider_find_speed_marker, "ComputerActorRider_FindSpeedMarker"),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{check, fill, put_words, scramble, Rng, DATA, GAME_GP};

    /// VfxScreenTint_Render: tint off / alpha zero (no draw), and the draw
    /// with both viewport modes (640, 320 with and without the interlace
    /// offset); the renderer's methods are fakes that may unwind.
    #[test]
    fn vfx_screen_tint_render_matches() {
        covered(ee::s12::f_00390f20__generated, vfx_screen_tint_render, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x0039_0f20, 7);
            c.pc = 0x0039_0f20;
            let gp = GAME_GP;
            c.bus.write32(gp + 0x151c, if seed % 5 == 0 { 0 } else { 1 });
            let alpha = match seed % 7 {
                0 => 0,
                1 => 0x8000_0000,
                _ => r.f32(1.0),
            };
            c.bus.write32(gp + 0x15ac, alpha);
            for off in [0x15b0, 0x15b4, 0x15b8] {
                let v = r.f32(1.0);
                c.bus.write32(gp + off, v);
            }
            let bits = r.next_u32();
            c.bus.write32(gp + 0x15bc, bits);
            let rend = DATA;
            c.bus.write32(gp + 0x2a90, rend);
            let stack = DATA + 0x2000 + (r.next_u32() % 16) * 4;
            c.bus.write32(rend + oc230_renderer::STATE, stack);
            fill(c, &mut r, stack, 16);
            fill(c, &mut r, 0x0050_1420, 5);
            let mode = if seed % 3 == 0 { 1 } else { r.next_u32() % 3 };
            c.bus.write32(rend + oc230_renderer::MODE, mode);
            c.bus.write32(rend + oc230_renderer::INTERLACE, seed / 2 % 2);
            let vt = DATA + 0x3000;
            c.bus.write32(rend + oc230_renderer::VTABLE, vt);
            for (i, slot) in [0xa8u32, 0xf8, 0x128, 0xd0, 0x238, 0x100, 0xb0].iter().enumerate() {
                c.bus.write16(vt + slot, (i as u16) * 4);
                c.bus.write32(vt + slot + 4, FAKE + 0x10 * i as u32);
            }
        });
    }

    /// RiderSphereTree_CollideTriangle2: the triangle p (0,0,0), a (4,0,0),
    /// b (0,4,0) with normal +z or -z; the root sphere and up to four kids
    /// at random places around it (above/below the plane, near the vertices
    /// and edges, or far), random masks. `VU0_TriangleIntersect` runs a stub
    /// microprogram at 0x6e0 (`vi2 = vi14`), so its answer is chosen per
    /// seed through vi14.
    #[test]
    fn rider_sphere_tree_collide_triangle2_matches() {
        use crate::difftest::put_vec;
        let f = |x: f32| x.to_bits();
        check(ee::s10::f_0032aa28__generated, rider_sphere_tree_collide_triangle2, 300, &|c, seed| {
            let mut r = Rng(0x32_aa28 ^ (seed as u64) << 20);
            scramble(c, &mut r);
            c.pc = 0x0032_aa28;
            c.g[GP][0] = GAME_GP as u64;
            ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                .hw
                .vu0
                .fallback = Some(ssx3_ps2::vu::reference::run_program);
            c.bus.write32(0x1100_0000 + 0x6e0, 0x8000_70b0); // IADD vi2, vi14, vi0
            c.bus.write32(0x1100_0000 + 0x6e4, 0x4000_02ff); // NOP, E
            c.g[14][0] = if seed % 4 == 0 { 1 } else { 0 };
            c.ctc2(14, 14);
            c.bus.write32(GAME_GP - 0x2dd0, f(0.0001));
            c.bus.write32(GAME_GP - 0x2dc8, f(0.0001));
            c.bus.write32(GAME_GP - 0x2dcc, f(1.0e10));
            c.bus.write32(GAME_GP - 0x2dc4, f(1.0e10));
            let tree = DATA;
            let (p, a, b, n) = (DATA + 0x400, DATA + 0x410, DATA + 0x420, DATA + 0x430);
            put_vec(c, p, [f(0.0), f(0.0), f(0.0), 0]);
            put_vec(c, a, [f(4.0), f(0.0), f(0.0), 0]);
            put_vec(c, b, [f(0.0), f(4.0), f(0.0), 0]);
            let nz = if seed % 5 == 1 { -1.0 } else { 1.0 };
            put_vec(c, n, [f(0.0), f(0.0), f(nz), 0]);
            let place = |c: &mut Cpu, r: &mut Rng, at: u32| {
                let q = match r.next_u32() % 7 {
                    0 => [-0.5, -0.5],
                    1 => [2.0, -0.6],
                    2 => [-0.6, 2.0],
                    3 => [2.3, 2.3],
                    4 => [1.0, 1.0],
                    5 => [4.4, 0.2],
                    _ => [9.0, 9.0],
                };
                let jitter = |r: &mut Rng| (r.next_u32() % 100) as f32 * 0.004 - 0.2;
                let z = (r.next_u32() % 100) as f32 * 0.03 - 1.5;
                put_vec(c, at, [f(q[0] + jitter(r)), f(q[1] + jitter(r)), f(z), 0]);
                c.bus.write32(at + 0x10, f(0.3 + (r.next_u32() % 100) as f32 * 0.012));
            };
            place(c, &mut r, tree + 0x10);
            let count = r.next_u32() % 5;
            for k in 0..count {
                place(c, &mut r, tree + 0x30 + 0x20 * k);
            }
            let mask = if seed % 3 == 0 { 0 } else { r.next_u32() & 0xf };
            c.bus.write32(tree + 0x28, mask);
            c.bus.write32(tree + 0x2c, count);
            c.g[A0][0] = sx(tree);
            c.g[A1][0] = sx(p);
            c.g[A2][0] = sx(a);
            c.g[A3][0] = sx(b);
            c.g[T0][0] = sx(n);
            c.g[T1][0] = sx(DATA + 0x500);
            c.g[T2][0] = sx(DATA + 0x510);
        });
    }

    /// SpatialHash_Move on a small octree: a cell at level 2 or 3, the
    /// entry linked (with or without neighbours) under the old key, the new
    /// key at the cell's level, below it in the same child (recursion) or in
    /// another child (old child freed when it empties, new child allocated
    /// when absent). Child hashes come from the real heap (`alloc_ready`);
    /// the real Remove/InsertAtKey run.
    #[test]
    fn spatial_hash_move_matches() {
        use crate::game::be::belibrary::tests::{alloc_ready, load};
        check(ee::s10::f_00328808__generated, spatial_hash_move, 200, &|c, seed| {
            let mut r = Rng(0x0032_8808 ^ (seed as u64) << 16);
            scramble(c, &mut r);
            alloc_ready(c);
            let alloc = |c: &mut Cpu| -> u32 {
                let p = load(c, 0x0031_7d70, ee::s10::EA_MemAllocNamed, &[0x2c, 0x004a_3fc0, 0x2000_0000, 0]) as u32;
                for i in 0..11 {
                    c.bus.write32(p + 4 * i, 0);
                }
                p
            };
            let level = 2 + (seed / 4 % 2) as i32;
            let cell: [i32; 3] = [0, 1, 2].map(|_| (r.next_u32() % 5) as i32 - 2);
            // A key at level l under the cell (octant bits random).
            let key_at = |r: &mut Rng, l: i32| -> [i32; 3] {
                let n = level - l;
                cell.map(|v| (v << n) | (r.next_u32() & ((1 << n) - 1)) as i32)
            };
            let chain = |c: &mut Cpu, h: u32, l: i32, k: &[i32; 3]| -> u32 {
                let mut node = h;
                for j in (l..level).rev() {
                    let o = k.map(|v| ((v >> (j - l)) & 1) as u32);
                    let slot = node + o[0] * 16 + o[1] * 8 + o[2] * 4;
                    let mut child = c.bus.read32(slot);
                    if child == 0 {
                        child = alloc(c);
                        c.bus.write32(slot, child);
                    }
                    node = child;
                }
                node
            };
            let h = DATA + 0x2000;
            for i in 0..11 {
                c.bus.write32(h + 4 * i, 0);
            }
            let key = r.next_u32() % 3;
            let (cell_k, old_k, new_k) = (DATA + 0x100, DATA + 0x110, DATA + 0x120);
            let deeper = |r: &mut Rng| level - 1 - (r.next_u32() % level as u32) as i32;
            let (old_l, new_l) = match seed % 4 {
                0 => (if r.next_u32() % 2 == 0 { level } else { deeper(&mut r) }, level),
                1 => (level, deeper(&mut r)),
                _ => (deeper(&mut r), deeper(&mut r)),
            };
            let old = key_at(&mut r, old_l);
            let mut new = key_at(&mut r, new_l);
            if seed % 4 == 2 {
                // Same top octant as the old key.
                let (no, oo) = (level - 1 - new_l, level - 1 - old_l);
                for a in 0..3 {
                    let bit = (old[a] >> oo) & 1;
                    new[a] = (new[a] & !(1 << no)) | (bit << no);
                }
            }
            for (at, l, k) in [(cell_k, level, cell), (old_k, old_l, old), (new_k, new_l, new)] {
                c.bus.write32(at, l as u32);
                for a in 0..3 {
                    c.bus.write32(at + 4 + 4 * a as u32, k[a] as u32);
                }
            }
            // The entry (and maybe neighbours) in the old key's bucket.
            let bottom = chain(c, h, old_l, &old);
            let bucket = bottom + 0x20 + 4 * key;
            let (e, prev, next) = (DATA + 0x200, DATA + 0x210, DATA + 0x220);
            let with_prev = r.next_u32() % 3 == 0;
            let with_next = r.next_u32() % 3 == 0;
            put_words(c, e, &[if with_next { next } else { 0 }, if with_prev { prev } else { 0 }]);
            put_words(c, prev, &[e, 0]);
            put_words(c, next, &[0, e]);
            c.bus.write32(bucket, if with_prev { prev } else { e });
            // Other live words in the old bottom hash, sometimes.
            if r.next_u32() % 3 == 0 {
                let w = r.next_u32() % 11;
                if bottom + 4 * w != bucket && c.bus.read32(bottom + 4 * w) == 0 {
                    c.bus.write32(bottom + 4 * w, DATA + 0x7000);
                }
            }
            // The new key's chain already there, sometimes.
            if r.next_u32() % 2 == 0 && new_l < level {
                chain(c, h, new_l, &new);
            }
            c.g[A0][0] = sx(h);
            c.g[A1][0] = key as u64;
            c.g[A2][0] = sx(e);
            c.g[A3][0] = sx(old_k);
            c.g[T0][0] = sx(new_k);
            c.g[T1][0] = sx(cell_k);
            c.pc = SHM_ENTRY;
        });
    }

    /// SND_UDiv64by32: random 64-bit dividends against small, large,
    /// sign-extended (upper half set) and zero divisors.
    #[test]
    fn snd_udiv64by32_matches() {
        check(ee::s13::f_003c64a8__generated, snd_udiv64by32, 300, &|c, seed| {
            let mut r = Rng(0x003c_64a8 ^ (seed as u64) << 12);
            scramble(c, &mut r);
            c.pc = 0x003c_64a8;
            let x = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            c.g[A0][0] = match seed % 4 {
                0 => x >> 20,
                _ => x,
            };
            c.g[A1][0] = match seed % 5 {
                0 => 0,
                1 => (r.next_u32() % 1000) as u64 + 1,
                2 => sx(r.next_u32() | 0x8000_0000),
                3 => r.next_u32() as u64,
                _ => c.g[A0][0] >> 32 | 1,
            };
        });
    }

    /// cCSICubicSpline_Evaluate: 1..8 segments with increasing start
    /// times, a random current segment, t at a segment start, before the
    /// first, at the end, inside the current one or anywhere in range
    /// (the real EvalSegment runs).
    #[test]
    fn c_csi_cubic_spline_evaluate_matches() {
        check(ee::s10::f_0031dee0__generated, c_csi_cubic_spline_evaluate, 300, &|c, seed| {
            let mut r = Rng(0x0031_dee0 ^ (seed as u64) << 12);
            scramble(c, &mut r);
            c.pc = 0x0031_dee0;
            let f = |x: f32| x.to_bits();
            let (spline, segs) = (DATA, DATA + 0x100);
            let n = 1 + r.next_u32() % 8;
            let cur = r.next_u32() % n;
            let junk = r.next_u32() & 0xf800_07ff;
            c.bus.write32(spline, junk | n << 11 | cur << 19);
            let mut starts = Vec::new();
            for i in 0..n {
                let t0 = i as f32 + (r.next_u32() % 50) as f32 * 0.01;
                starts.push(t0);
                let seg = segs + 0x14 * i;
                for k in 0..4 {
                    let v = r.f32(4.0);
                    c.bus.write32(seg + 4 * k, v);
                }
                c.bus.write32(seg + 0x10, f(t0));
            }
            let end = n as f32 + 0.25;
            c.bus.write32(spline + 4, f(end));
            c.bus.write32(segs + 0x14 * n + 0x10, f(end + 5.0));
            c.bus.write32(segs + 0x14 * (n + 1) + 0x10, f(end + 6.0));
            c.bus.write32(spline + 8, segs);
            let t = match seed % 6 {
                0 => starts[(r.next_u32() % n) as usize],
                1 => starts[0] - 0.5,
                2 => end,
                3 => starts[cur as usize] + 0.3,
                _ => starts[0] + (end - starts[0]) * (r.next_u32() % 1000) as f32 / 1000.0,
            };
            c.f[12] = f(t);
            c.g[A0][0] = sx(spline);
        });
    }

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

    /// DeferMan_SortBuckets: 0..64 items with random keys (sometimes few
    /// distinct bytes), the scratchpad stream at gp+0x1384 with two slots
    /// whose blocks are in RAM; the real stream callees and memset run.
    #[test]
    fn defer_man_sort_buckets_matches() {
        check(ee::s12::f_00364050__generated, defer_man_sort_buckets, 200, &|c, seed| {
            let mut r = Rng(0x0036_4050 ^ (seed as u64) << 12);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.pc = 0x0036_4050;
            let (man, stream) = (DATA, DATA + 0x7_0000);
            c.bus.write32(GAME_GP + 0x1384, stream);
            let slot = r.next_u32() % 2;
            c.bus.write32(stream + 4, 2);
            c.bus.write32(stream + 8, slot);
            c.bus.write32(stream + 0xc, slot);
            c.bus.write32(stream + 0x10, 0x101);
            c.bus.write32(stream + 0x14, DATA + 0x7_1000);
            c.bus.write32(stream + 0x18, DATA + 0x7_3000);
            fill(c, &mut r, DATA + 0x7_1000, 0x404);
            fill(c, &mut r, DATA + 0x7_3000, 0x404);
            let n = match seed % 5 {
                0 => 0,
                1 => 1,
                _ => r.next_u32() % 65,
            };
            c.bus.write32(man + oc230_defer::COUNT, n);
            for i in 0..n {
                let key = match seed % 3 {
                    0 => r.next_u32(),
                    1 => r.next_u32() & 0x0103_0107,
                    _ => r.next_u32() % 300,
                };
                c.bus.write32(man + oc230_defer::ITEMS + 8 * i, key);
                let v = r.next_u32();
                c.bus.write32(man + oc230_defer::ITEMS + 8 * i + 4, v);
            }
            fill(c, &mut r, oc230_defer::SCRATCH, 2 * 64);
            c.g[A0][0] = sx(man);
        });
    }

    /// Nash_FindClosestContact over the real `Nash_CollideVolume`: eight
    /// level-0 cells at random places around a query volume (box around
    /// the origin), each tree holding a chain of up to four instances
    /// whose forward-difference entry makes the volume's (fake) triangle
    /// test run, so 0..n contacts with fake distances come back; the
    /// query's reference distance (virtual 0x18) is a fake too. Also
    /// checks that some seeds found contacts.
    #[test]
    fn nash_find_closest_contact_matches() {
        use crate::game::intersect::worldsphtree::tests::{
            put_cache, put_instance, put_volume, setup_machine, VT_VOL,
        };
        let f = |x: f32| x.to_bits();
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = setup_machine(c, seed, 0x0033_6850, 6);
            c.pc = 0x0033_6850;
            let (belib, world) = (DATA + 0x3_8000, DATA + 0x3_9000);
            c.bus.write32(GAME_GP - 0x1f04, belib);
            c.bus.write32(GAME_GP - 0x848, world);
            c.bus.write32(world + 0x84, world + 0x100);
            c.bus.write32(world + 0x100 + 0x7c, 0);
            c.bus.write32(GAME_GP - 0x2dfc, 0);
            for k in 0..16 {
                c.bus.write32(GAME_GP - 0x2d10 + 4 * k, f(0.2));
            }
            let (grid, q, cache, out) = (DATA, DATA + 0x400, DATA + 0xc00, DATA + 0x6000);
            put_volume(c, &mut r, q);
            c.bus.write16(VT_VOL + 0x18, 0);
            c.bus.write32(VT_VOL + 0x1c, FAKE + 0x70);
            put_cache(c, cache);
            c.bus.write32(grid + 0xa4, cache);
            // Instance chains (inst+0 = next).
            let mut chains = Vec::new();
            for k in 0..4u32 {
                let count = (seed + k) % 5;
                let mut next = 0;
                for i in 0..count {
                    let inst = DATA + 0x8000 + 0x800 * k + 0x200 * i;
                    put_instance(c, &mut r, inst, seed + i + k);
                    c.bus.write32(inst, next);
                    next = inst;
                }
                chains.push(next);
            }
            for k in 0..8u32 {
                let cell = grid + 0x14 * k;
                let xyz = [0u32, 1, 5, u32::MAX][(r.next_u32() % 4) as usize];
                c.bus.write32(cell, 0);
                for a in 1..4 {
                    c.bus.write32(cell + 4 * a, xyz);
                }
                let tree = DATA + 0x1000 + 0x100 * k;
                for i in 0..10 {
                    c.bus.write32(tree + 4 * i, 0);
                }
                c.bus.write32(tree + 0x24, chains[(r.next_u32() % 4) as usize]);
                c.bus.write32(cell + 0x10, if (seed + k) % 6 != 0 { tree } else { 0 });
            }
            c.g[A0][0] = sx(grid);
            c.g[A1][0] = sx(q);
            c.g[A2][0] = sx(out);
            c.g[A3][0] = r.next_u32() as u64 % 8;
        };
        check(ee::s11::f_00336850__generated, nash_find_closest_contact, 200, &setup);
        let found = (0..60)
            .filter(|&seed| {
                let mut c = crate::difftest::fresh();
                setup(&mut *c, seed);
                crate::difftest::run_generated(&mut *c, ee::s11::f_00336850__generated);
                c.f[0] != 0xbf80_0000
            })
            .count();
        assert!(found >= 6, "contacts found in only {found} of 60 seeds");
    }

    /// cSplashManager_EmitPending: pending amounts below 1 (nothing), up
    /// to ~10 emissions, a crystal chance of 0, 1 or in between, crystal
    /// ranges including an empty one (k forced to 1) and small drop/crystal
    /// capacities so the real AddDrop/SpawnCrystals (and their init calls,
    /// over the ELF image's data) sometimes store and sometimes skip.
    #[test]
    fn c_splash_manager_emit_pending_matches() {
        check(ee::s09::f_002f3810__generated, c_splash_manager_emit_pending, 200, &|c, seed| {
            let mut r = Rng(0x002f_3810 ^ (seed as u64) << 12);
            crate::difftest::elf_image(c);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.pc = 0x002f_3810;
            let gp = GAME_GP;
            c.bus.write32(gp - 0x1f04, DATA + 0x3_0000);
            c.bus.write32(gp - 0x848, DATA + 0x3_1000);
            let m = DATA;
            for i in 0..0x500 {
                c.bus.write32(m + 4 * i, 0);
            }
            let pending = match seed % 5 {
                0 => 0.75f32,
                1 => -1.5,
                _ => (r.next_u32() % 1000) as f32 * 0.011,
            };
            c.bus.write32(m + oc230_splash::PENDING, pending.to_bits());
            c.bus.write32(m + 0x14, r.next_u32() % 3);
            c.bus.write32(m + 0x18, r.next_u32() % 3);
            c.bus.write32(gp + 0x1164, r.next_u32() % 7);
            c.bus.write32(gp + 0x1168, r.next_u32() % 7);
            let seed_bits = 0x3f80_0000 | (r.next_u32() & 0x7f_ffff);
            c.bus.write32(gp + oc230_splash::SEED, seed_bits);
            let chance = match seed % 3 {
                0 => 0.0f32,
                1 => 1.0,
                _ => 0.4,
            };
            c.bus.write32(gp + oc230_splash::CRYSTAL_CHANCE, chance.to_bits());
            let lo = r.next_u32() % 3;
            let hi = if seed % 4 == 0 { lo } else { lo + 1 + r.next_u32() % 3 };
            c.bus.write32(gp + oc230_splash::CRYSTALS_MIN, lo);
            c.bus.write32(gp + oc230_splash::CRYSTALS_MAX, hi);
            c.bus.write32(gp + oc230_splash::DROP_SIZE, r.f32(4.0));
            c.g[A0][0] = sx(m);
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
        covered(ee::s00::f_0010bb18__generated, computer_actor_rider_find_speed_marker, 300, &|c, seed| {
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
            let n = if seed % 6 == 0 { 1 } else { 1 + r.next_u32() % 12 };
            c.bus.write32(LOG_RESULTS, n);
            let buf = STACK - 0x100;
            let marker = if seed % 4 == 0 { 99 } else { r.next_u32() % 12 };
            for i in 0..12u32 {
                let rec = buf + 0x10 * i;
                let ty = if i == marker { 0x11 } else { r.next_u32() % 0x20 & !0x11 | 1 };
                c.bus.write32(rec, ty);
                c.bus.write32(rec + 4, r.next_u32() % 200);
                let lim = if r.next_u32() % 2 == 0 { dist } else { r.f32(1000.0) };
                c.bus.write32(rec + 8, lim);
            }
            c.g[A0][0] = sx(actor);
            c.g[A1][0] = sx(DATA + 0x2000);
            c.g[A2][0] = sx(DATA + 0x2010);
        });
    }
}
