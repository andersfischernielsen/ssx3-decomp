//! Native rider anger billboard, using GraphicsMan's retail anger texture.

use crate::abi;
use crate::call::{call_ptr, guest, Frame};
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x002d_5048, anger_fx_render, "cAngerFX_Render")];

const MATERIAL_TOP: u32 = 0x0e84;
const GFX_INTERFACE: u32 = 0x10d8;
const ANGER_TEXTURE: u32 = 0x0fac;
const MATERIAL_TEMPLATE: u32 = 0x0050_1420;

fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// `void cAngerFX::Render()` (0x2d5048).
/// Select the local viewport's anger slot, position a billboard above the
/// rider's head, fade at long range, then submit one native four-vertex strip.
/// The retail icon is the texture handle at GraphicsMan+0xfac; imported world
/// material indices never select the exclamation texture.
pub fn anger_fx_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x1a0);
        frame.save(c, S0, 0x180);
        frame.save(c, S1, 0x170);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x160);
        frame.save_fpr(c, 21, 0x198);
        frame.save_fpr(c, 20, 0x190);
        c.g[A0][0] = c.lw(r(c, S0));
        c.g[V1][0] = c.lw(r(c, A0).wrapping_add(0x6c0));
        c.g[A0][0] = sx(r(c, A0).wrapping_add(0x6c0));
        c.g[A1][0] = c.lh(r(c, V1).wrapping_add(0x40));
        c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x44));
        let rider_hides_fx = r(c, V0);
        c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, A1)));
        call_ptr(c, rider_hides_fx, 0x002d_5080)?;

        let mut restore_s1 = true;
        'render: {
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x180);
                break 'render;
            }
            // Each local viewport has its own 0x2c-byte animation slot.
            c.cycles += 16;
            c.g[V1][0] = c.lw(r(c, GP).wrapping_sub(0x848));
            c.g[A0][0] = 0x2c;
            c.g[A1][0] = c.lw(r(c, S0).wrapping_add(4));
            c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x84));
            c.f[20] = 0;
            c.g[V1][0] = c.lw(r(c, V0).wrapping_add(0x84));
            c.g[AT][0] = 0x3f80_0000;
            c.f[21] = r(c, AT);
            c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x14));
            c.mult(0, V0, V0, A0);
            c.g[S1][0] = sx(r(c, A1).wrapping_add(r(c, V0)));
            c.lwc1(0, r(c, S1));
            abi::set_fcc(c, FpuF32(c.f[0]).eq_s(FpuF32(c.f[20])));
            if c.fcc() {
                frame.restore(c, S0, 0x180);
                break 'render;
            }

            // Transform the rider's head bone to view space for clipping.
            c.cycles += 32;
            c.g[A1][0] = c.lw(r(c, GP).wrapping_sub(0x854));
            c.g[V1][0] = c.lw(r(c, A1).wrapping_add(GFX_INTERFACE));
            c.g[A0][0] = c.lh(r(c, V1).wrapping_add(0x118));
            c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x11c));
            let view_matrix = r(c, V0);
            c.g[A0][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
            call_ptr(c, view_matrix, 0x002d_50e0)?;
            c.g[T4][0] = c.g[V0][0];
            c.g[V0][0] = c.lw(r(c, S0));
            c.g[A1][0] = c.lw(r(c, V0).wrapping_add(0x780));
            c.g[V1][0] = c.lw(r(c, V0).wrapping_add(0x8a8));
            c.g[A0][0] = c.lw(r(c, A1).wrapping_add(0x30));
            c.g[V1][0] = sx(r(c, V1) << 6);
            c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, V1)));
            c.lq(V0, r(c, A0).wrapping_add(0x30));
            c.sq(V0, r(c, SP));
            c.lqc2(8, r(c, SP));
            c.lqc2(4, r(c, T4));
            c.lqc2(5, r(c, T4).wrapping_add(0x10));
            c.lqc2(6, r(c, T4).wrapping_add(0x20));
            c.lqc2(7, r(c, T4).wrapping_add(0x30));
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.sqc2(12, r(c, SP).wrapping_add(0x20));
            c.lq(V0, r(c, SP).wrapping_add(0x20));
            c.sq(V0, r(c, SP).wrapping_add(0x10));
            c.lwc1(4, r(c, SP).wrapping_add(0x18));
            abi::set_fcc(c, FpuF32(c.f[4]).le(FpuF32(c.f[20])));
            let behind_camera = c.fcc();
            frame.restore(c, S0, 0x180);
            if behind_camera {
                break 'render;
            }
            c.cycles += 6;
            c.g[AT][0] = 0x44fa_0000;
            c.f[5] = r(c, AT);
            abi::set_fcc(c, FpuF32(c.f[5]).le(FpuF32(c.f[4])));
            if c.fcc() {
                frame.restore(c, S1, 0x170);
                restore_s1 = false;
                break 'render;
            }

            // Keep the original growth and distance fade operation order.
            c.cycles += 9;
            c.lwc1(1, r(c, GP).wrapping_sub(0x3d20));
            c.lwc1(2, r(c, GP).wrapping_sub(0x3d1c));
            c.f[1] = (FpuF32(c.f[4]) * FpuF32(c.f[1])).0;
            c.lwc1(3, r(c, S1).wrapping_add(0x14));
            c.lwc1(0, r(c, S1).wrapping_add(0x18));
            abi::set_fcc(c, FpuF32(c.f[2]).lt(FpuF32(c.f[4])));
            c.f[6] = (FpuF32(c.f[0]) + FpuF32(c.f[1])).0;
            let fades_at_distance = c.fcc();
            c.f[7] = (FpuF32(c.f[3]) + FpuF32(c.f[1])).0;
            if fades_at_distance {
                c.cycles += 3;
                c.f[1] = (FpuF32(c.f[5]) - FpuF32(c.f[4])).0;
                c.lwc1(0, r(c, GP).wrapping_sub(0x3d18));
                c.f[21] = (FpuF32(c.f[1]) * FpuF32(c.f[0])).0;
            }
            c.cycles += 252;
            // Camera right/up axes form the billboard basis.
            c.lwc1(0, r(c, T4));
            c.g[T1][0] = 0x500000;
            c.lwc1(1, r(c, T4).wrapping_add(0x10));
            c.g[A2][0] = 0xffffffffffbf0000;
            c.lwc1(2, r(c, T4).wrapping_add(0x20));
            c.g[A2][0] = c.g[A2][0] | 0xffff;
            c.swc1(0, r(c, SP).wrapping_add(0x20));
            c.g[T0][0] = 0x400000;
            c.swc1(2, r(c, SP).wrapping_add(0x28));
            c.g[T2][0] = 0xfffffffffffffffc;
            c.swc1(1, r(c, SP).wrapping_add(0x24));
            c.g[A3][0] = 0xffffffffe0000000;
            c.swc1(20, r(c, SP).wrapping_add(0x2c));
            c.g[A3][0] = c.g[A3][0] | 0x3ff;
            c.g[AT][0] = 0x420c0000;
            c.f[2] = r(c, AT);
            c.g[T3][0] = 0xfffffffffffffc1f;
            c.lwc1(1, r(c, T4).wrapping_add(0x4));
            c.g[T5][0] = 0x3;
            c.lwc1(0, r(c, T4).wrapping_add(0x14));
            c.g[T6][0] = sx(r(c, SP).wrapping_add(0x80));
            c.lwc1(3, r(c, T4).wrapping_add(0x24));
            c.swc1(1, r(c, SP).wrapping_add(0x30));
            c.g[T4][0] = 0xffffffffffffffff;
            c.swc1(0, r(c, SP).wrapping_add(0x34));
            c.swc1(3, r(c, SP).wrapping_add(0x38));
            c.swc1(20, r(c, SP).wrapping_add(0x3c));
            c.g[2][0] = sx(c.f[2]);
            c.lqc2(4, r(c, SP).wrapping_add(0x30));
            c.qmtc2(2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, r(c, SP).wrapping_add(0x60));
            c.g[AT][0] = 0x41900000;
            c.f[0] = r(c, AT);
            c.lq(2, r(c, SP).wrapping_add(0x60));
            c.f[0] = (FpuF32(c.f[6]) * FpuF32(c.f[0])).0;
            c.g[AT][0] = 0x41100000;
            c.f[1] = r(c, AT);
            c.sq(2, r(c, SP).wrapping_add(0x50));
            c.lqc2(3, r(c, SP));
            c.lqc2(4, r(c, SP).wrapping_add(0x50));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, r(c, SP).wrapping_add(0x60));
            c.f[1] = (FpuF32(c.f[7]) * FpuF32(c.f[1])).0;
            c.lq(2, r(c, SP).wrapping_add(0x60));
            c.sq(2, r(c, SP).wrapping_add(0x40));
            c.g[3][0] = sx(c.f[0]);
            c.lqc2(4, r(c, SP).wrapping_add(0x30));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, r(c, SP).wrapping_add(0x70));
            c.lq(2, r(c, SP).wrapping_add(0x70));
            c.sq(2, r(c, SP).wrapping_add(0x60));
            c.lqc2(3, r(c, SP).wrapping_add(0x40));
            c.lqc2(4, r(c, SP).wrapping_add(0x60));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, r(c, SP).wrapping_add(0x70));
            c.lq(2, r(c, SP).wrapping_add(0x70));
            c.sq(2, r(c, SP).wrapping_add(0x50));
            c.g[4][0] = sx(c.f[1]);
            c.lqc2(4, r(c, SP).wrapping_add(0x20));
            c.qmtc2(4, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, r(c, SP).wrapping_add(0x70));
            c.lq(3, r(c, SP).wrapping_add(0x70));
            c.g[A0][0] = c.lw(r(c, GP).wrapping_sub(0x854));
            c.sq(3, r(c, SP).wrapping_add(0x60));
            // Push the native material stack and install the retail anger material.
            c.g[V0][0] = c.lw(r(c, A0).wrapping_add(MATERIAL_TOP));
            c.ldl(3, r(c, V0).wrapping_add(0x7));
            c.ldr(3, r(c, V0));
            c.ldl(5, r(c, V0).wrapping_add(0xf));
            c.ldr(5, r(c, V0).wrapping_add(0x8));
            c.g[T7][0] = c.lw(r(c, V0).wrapping_add(0x10));
            c.sdl(3, r(c, V0).wrapping_add(0x1b));
            c.sdr(3, r(c, V0).wrapping_add(0x14));
            c.sdl(5, r(c, V0).wrapping_add(0x23));
            c.sdr(5, r(c, V0).wrapping_add(0x1c));
            c.sw(r(c, V0).wrapping_add(0x24), c.g[T7][0]);
            c.g[V1][0] = c.lw(r(c, A0).wrapping_add(MATERIAL_TOP));
            c.g[A1][0] = c.lw(r(c, GP).wrapping_sub(0x854));
            c.g[V1][0] = sx(r(c, V1).wrapping_add(0x14));
            c.sw(r(c, A0).wrapping_add(MATERIAL_TOP), c.g[V1][0]);
            c.g[V0][0] = c.lw(r(c, A1).wrapping_add(MATERIAL_TOP));
            c.g[T7][0] = sx(MATERIAL_TEMPLATE);
            c.ldl(3, r(c, T7).wrapping_add(0x7));
            c.ldr(3, r(c, T7));
            c.ldl(4, r(c, T7).wrapping_add(0xf));
            c.ldr(4, r(c, T7).wrapping_add(0x8));
            c.g[A1][0] = c.lw(r(c, T7).wrapping_add(0x10));
            c.sdl(3, r(c, V0).wrapping_add(0x7));
            c.sdr(3, r(c, V0));
            c.sdl(4, r(c, V0).wrapping_add(0xf));
            c.sdr(4, r(c, V0).wrapping_add(0x8));
            c.sw(r(c, V0).wrapping_add(0x10), c.g[A1][0]);
            c.g[A1][0] = c.lw(r(c, GP).wrapping_sub(0x854));
            c.g[V1][0] = c.lw(r(c, A1).wrapping_add(MATERIAL_TOP));
            c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x4));
            c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
            c.g[V0][0] = c.g[V0][0] | c.g[T0][0];
            c.sw(r(c, V1).wrapping_add(0x4), c.g[V0][0]);
            c.g[A0][0] = c.lw(r(c, A1).wrapping_add(MATERIAL_TOP));
            c.g[V0][0] = c.lw(r(c, A0).wrapping_add(0x4));
            c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
            c.g[V0][0] = c.g[V0][0] | 0x2;
            c.sw(r(c, A0).wrapping_add(0x4), c.g[V0][0]);
            c.g[V1][0] = c.lw(r(c, A1).wrapping_add(MATERIAL_TOP));
            c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x8));
            c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
            c.sw(r(c, V1).wrapping_add(0x8), c.g[V0][0]);
            c.g[A0][0] = c.lw(r(c, A1).wrapping_add(MATERIAL_TOP));
            c.g[V0][0] = c.lw(r(c, A0).wrapping_add(0x8));
            c.g[V0][0] = c.g[V0][0] & c.g[T3][0];
            c.g[V0][0] = c.g[V0][0] | 0xe0;
            c.sw(r(c, A0).wrapping_add(0x8), c.g[V0][0]);
            c.g[V1][0] = c.lw(r(c, A1).wrapping_add(MATERIAL_TOP));
            c.g[V0][0] = c.lw(r(c, A1).wrapping_add(ANGER_TEXTURE));
            let icon_graphics = r(c, A1);
            let icon_handle = r(c, V0);
            trace_icon(c, icon_graphics, icon_handle);
            c.sh(r(c, V1).wrapping_add(0x10), c.g[V0][0]);
            // Build four UV/colour/position vertices, retaining both halves of each GPR.
            c.ldl(16, r(c, S1).wrapping_add(0xb));
            c.ldr(16, r(c, S1).wrapping_add(0x4));
            c.ldl(2, r(c, S1).wrapping_add(0x13));
            c.ldr(2, r(c, S1).wrapping_add(0xc));
            c.sdl(16, r(c, SP).wrapping_add(0x77));
            c.sdr(16, r(c, SP).wrapping_add(0x70));
            c.sdl(2, r(c, SP).wrapping_add(0x7f));
            c.sdr(2, r(c, SP).wrapping_add(0x78));
            c.lwc1(0, r(c, SP).wrapping_add(0x70));
            c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[21])).0;
            c.swc1(0, r(c, SP).wrapping_add(0x70));
            c.lqc2(3, r(c, SP).wrapping_add(0x40));
            c.lqc2(4, r(c, SP).wrapping_add(0x60));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, r(c, SP).wrapping_add(0x150));
            c.g[AT][0] = 0x437f0000;
            c.f[4] = r(c, AT);
            c.lwc1(0, r(c, SP).wrapping_add(0x74));
            c.g[A2][0] = c.g[T6][0];
            c.g[AT][0] = 0x43000000;
            c.f[6] = r(c, AT);
            c.g[A1][0] = 0x4;
            c.lwc1(1, r(c, SP).wrapping_add(0x78));
            c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[4])).0;
            c.lwc1(2, r(c, SP).wrapping_add(0x7c));
            c.g[A3][0] = 0u64;
            c.lwc1(3, r(c, SP).wrapping_add(0x70));
            c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[4])).0;
            c.lq(2, r(c, SP).wrapping_add(0x150));
            c.f[2] = (FpuF32(c.f[2]) * FpuF32(c.f[4])).0;
            c.f[3] = (FpuF32(c.f[3]) * FpuF32(c.f[6])).0;
            c.g[AT][0] = 0x3f800000;
            c.f[5] = r(c, AT);
            c.f[7] = FpuF32(c.f[0]).to_i32() as u32;
            c.swc1(7, r(c, SP).wrapping_add(0x90));
            c.sq(2, r(c, SP).wrapping_add(0x140));
            c.sq(2, r(c, SP).wrapping_add(0xa0));
            c.f[0] = FpuF32(c.f[1]).to_i32() as u32;
            c.swc1(0, r(c, SP).wrapping_add(0x94));
            c.f[0] = FpuF32(c.f[2]).to_i32() as u32;
            c.swc1(0, r(c, SP).wrapping_add(0x98));
            c.swc1(5, r(c, SP).wrapping_add(0x88));
            c.f[0] = FpuF32(c.f[3]).to_i32() as u32;
            c.swc1(0, r(c, SP).wrapping_add(0x9c));
            c.sw(r(c, SP).wrapping_add(0x80), 0u64);
            c.swc1(5, r(c, SP).wrapping_add(0x84));
            c.lqc2(3, r(c, SP).wrapping_add(0x40));
            c.lqc2(4, r(c, SP).wrapping_add(0x60));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, r(c, SP).wrapping_add(0x150));
            c.lwc1(1, r(c, SP).wrapping_add(0x74));
            c.lwc1(2, r(c, SP).wrapping_add(0x78));
            c.lwc1(3, r(c, SP).wrapping_add(0x7c));
            c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[4])).0;
            c.lwc1(0, r(c, SP).wrapping_add(0x70));
            c.f[2] = (FpuF32(c.f[2]) * FpuF32(c.f[4])).0;
            c.f[3] = (FpuF32(c.f[3]) * FpuF32(c.f[4])).0;
            c.lq(2, r(c, SP).wrapping_add(0x150));
            c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[6])).0;
            c.swc1(5, r(c, SP).wrapping_add(0xb8));
            c.f[7] = FpuF32(c.f[1]).to_i32() as u32;
            c.swc1(7, r(c, SP).wrapping_add(0xc0));
            c.sq(2, r(c, SP).wrapping_add(0x140));
            c.f[1] = FpuF32(c.f[2]).to_i32() as u32;
            c.swc1(1, r(c, SP).wrapping_add(0xc4));
            c.f[1] = FpuF32(c.f[3]).to_i32() as u32;
            c.swc1(1, r(c, SP).wrapping_add(0xc8));
            c.f[1] = FpuF32(c.f[0]).to_i32() as u32;
            c.swc1(1, r(c, SP).wrapping_add(0xcc));
            c.sq(2, r(c, SP).wrapping_add(0xd0));
            c.swc1(5, r(c, SP).wrapping_add(0xb0));
            c.swc1(5, r(c, SP).wrapping_add(0xb4));
            c.lqc2(3, r(c, SP).wrapping_add(0x50));
            c.lqc2(4, r(c, SP).wrapping_add(0x60));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, r(c, SP).wrapping_add(0x150));
            c.lwc1(0, r(c, SP).wrapping_add(0x74));
            c.lwc1(1, r(c, SP).wrapping_add(0x78));
            c.lwc1(2, r(c, SP).wrapping_add(0x7c));
            c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[4])).0;
            c.lwc1(3, r(c, SP).wrapping_add(0x70));
            c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[4])).0;
            c.lq(2, r(c, SP).wrapping_add(0x150));
            c.f[2] = (FpuF32(c.f[2]) * FpuF32(c.f[4])).0;
            c.f[3] = (FpuF32(c.f[3]) * FpuF32(c.f[6])).0;
            c.swc1(5, r(c, SP).wrapping_add(0xe8));
            c.f[7] = FpuF32(c.f[0]).to_i32() as u32;
            c.swc1(7, r(c, SP).wrapping_add(0xf0));
            c.sq(2, r(c, SP).wrapping_add(0x140));
            c.sq(2, r(c, SP).wrapping_add(0x100));
            c.f[0] = FpuF32(c.f[1]).to_i32() as u32;
            c.swc1(0, r(c, SP).wrapping_add(0xf4));
            c.sw(r(c, SP).wrapping_add(0xe0), 0u64);
            c.f[0] = FpuF32(c.f[2]).to_i32() as u32;
            c.swc1(0, r(c, SP).wrapping_add(0xf8));
            c.sw(r(c, SP).wrapping_add(0xe4), 0u64);
            c.f[0] = FpuF32(c.f[3]).to_i32() as u32;
            c.swc1(0, r(c, SP).wrapping_add(0xfc));
            c.lqc2(3, r(c, SP).wrapping_add(0x50));
            c.lqc2(4, r(c, SP).wrapping_add(0x60));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, r(c, SP).wrapping_add(0x150));
            c.lwc1(1, r(c, SP).wrapping_add(0x74));
            c.lwc1(2, r(c, SP).wrapping_add(0x78));
            c.lwc1(3, r(c, SP).wrapping_add(0x7c));
            c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[4])).0;
            c.lwc1(0, r(c, SP).wrapping_add(0x70));
            c.f[2] = (FpuF32(c.f[2]) * FpuF32(c.f[4])).0;
            c.f[3] = (FpuF32(c.f[3]) * FpuF32(c.f[4])).0;
            c.lq(2, r(c, SP).wrapping_add(0x150));
            c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[6])).0;
            c.f[4] = FpuF32(c.f[1]).to_i32() as u32;
            c.swc1(4, r(c, SP).wrapping_add(0x120));
            c.sq(2, r(c, SP).wrapping_add(0x140));
            c.f[1] = FpuF32(c.f[2]).to_i32() as u32;
            c.swc1(1, r(c, SP).wrapping_add(0x124));
            c.f[1] = FpuF32(c.f[3]).to_i32() as u32;
            c.swc1(1, r(c, SP).wrapping_add(0x128));
            c.f[1] = FpuF32(c.f[0]).to_i32() as u32;
            c.swc1(1, r(c, SP).wrapping_add(0x12c));
            c.sq(2, r(c, SP).wrapping_add(0x130));
            c.swc1(5, r(c, SP).wrapping_add(0x110));
            c.swc1(5, r(c, SP).wrapping_add(0x118));
            c.sw(r(c, SP).wrapping_add(0x114), 0u64);
            c.g[T0][0] = c.lw(r(c, GP).wrapping_sub(0x854));
            c.g[V0][0] = c.lw(r(c, T0).wrapping_add(GFX_INTERFACE));
            c.g[A0][0] = c.lh(r(c, V0).wrapping_add(0x238));
            c.g[V1][0] = c.lw(r(c, V0).wrapping_add(0x23c));
            // Native graphics interface slot 71 submits the triangle strip.
            let draw_strip = r(c, V1);
            c.g[A0][0] = sx(r(c, T0).wrapping_add(r(c, A0)));
            call_ptr(c, draw_strip, 0x002d_556c)?;
            c.g[A0][0] = c.lw(r(c, GP).wrapping_sub(0x854));
            c.g[V0][0] = c.lw(r(c, A0).wrapping_add(MATERIAL_TOP));
            c.g[V0][0] = sx(r(c, V0).wrapping_sub(0x14));
            c.sw(r(c, A0).wrapping_add(MATERIAL_TOP), c.g[V0][0]);
            frame.restore(c, S0, 0x180);
        }
        if restore_s1 {
            c.cycles += 1;
            frame.restore(c, S1, 0x170);
        }
        c.cycles += 5;
        frame.restore_ra(c, 0x160);
        frame.restore_fpr(c, 21, 0x198);
        frame.restore_fpr(c, 20, 0x190);
        frame.close(c);
        Ok(())
    });
}

/// Opt-in asset diagnosis during a normal windowed game session.
fn trace_icon(c: &mut Cpu, graphics: u32, handle: u32) {
    static WRITTEN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    let Some(directory) = std::env::var_os("SSX3_ANGER_DUMP") else {
        return;
    };
    if WRITTEN.swap(true, std::sync::atomic::Ordering::Relaxed) {
        return;
    }
    let manager = c.bus.read32(graphics + 0x18f4);
    let info = if handle < 2000 {
        c.bus.read32(manager + 8 + handle * 4)
    } else {
        0
    };
    eprintln!("[anger icon] graphics={graphics:08x} handle={handle:08x} manager={manager:08x} info={info:08x}");
    if info == 0 {
        return;
    }
    let directory = std::path::PathBuf::from(directory);
    let _ = std::fs::create_dir_all(&directory);
    for (name, address, size) in [
        ("info.bin", info, 0x58),
        ("pixels.bin", c.bus.read32(info + 0x1c), 128 * 128),
        ("palette.bin", c.bus.read32(info + 0x20), 1024),
    ] {
        let bytes: Vec<u8> = (0..size).map(|i| c.bus.read8(address + i)).collect();
        let _ = std::fs::write(directory.join(name), bytes);
    }
}
