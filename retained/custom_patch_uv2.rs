//! Native interpolation of a terrain patch's lightmap UV rectangle.

use crate::abi;
use crate::call::Frame;
use crate::events::{deliver_event_here, event_due};
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0037_4440,
    tessellate_lightmap_uvs,
    "BezierPatchCache_TessUV2",
)];

const GRID_COUNTS: u32 = 0x19c;
const LIGHTMAP_ORIGIN: u32 = 0x10;
const LIGHTMAP_DELTA: u32 = 0x18;

fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// `TessUV2(cache, patch, Vec2* out, lod)` (0x374440).
/// For grid element `[row][column]`, U advances with the outer row and V
/// with the inner column. The rectangle is used as supplied: this routine
/// adds no half texel, gutter, axis swap or texture-size correction.
pub fn tessellate_lightmap_uvs(c: &mut Cpu) {
    c.cycles += 26;
    c.g[A3][0] = sx(r(c, A3) << 2);
    let frame = Frame::open(c, 0x30);
    c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, A3)));
    c.g[AT][0] = 0x3f80_0000;
    c.f[2] = r(c, AT);
    c.g[A0][0] = c.lw(r(c, A0).wrapping_add(GRID_COUNTS));
    c.ldl(V0, r(c, A1).wrapping_add(LIGHTMAP_ORIGIN + 7));
    c.ldr(V0, r(c, A1).wrapping_add(LIGHTMAP_ORIGIN));
    c.sdl(V0, r(c, SP).wrapping_add(7));
    c.sdr(V0, r(c, SP));
    c.g[V0][0] = sx(r(c, A0).wrapping_sub(1));
    c.ldl(T1, r(c, A1).wrapping_add(LIGHTMAP_DELTA + 7));
    c.ldr(T1, r(c, A1).wrapping_add(LIGHTMAP_DELTA));
    c.sdl(T1, r(c, SP).wrapping_add(0x17));
    c.sdr(T1, r(c, SP).wrapping_add(0x10));
    c.f[3] = FpuF32::from_i32(r(c, V0) as i32).0;
    c.g[V1][0] = 0;
    c.lwc1(1, r(c, SP).wrapping_add(0x10));
    c.lwc1(0, r(c, SP).wrapping_add(0x14));
    c.f[2] = FpuF32(c.f[2]).div(FpuF32(c.f[3])).0;
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[2])).0;
    c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[2])).0;
    c.swc1(0, r(c, SP).wrapping_add(0x14));
    let empty_grid = (c.g[A0][0] as i64) <= 0;
    c.swc1(1, r(c, SP).wrapping_add(0x10));
    if !empty_grid {
        loop {
            c.cycles += 7;
            c.g[V0][0] = sx(r(c, V1).wrapping_add(1));
            c.ldl(T1, r(c, SP).wrapping_add(7));
            c.ldr(T1, r(c, SP));
            c.sdl(T1, r(c, SP).wrapping_add(0x27));
            c.sdr(T1, r(c, SP).wrapping_add(0x20));
            let empty_row = (c.g[A0][0] as i64) <= 0;
            c.g[V1][0] = c.g[A0][0];
            if !empty_row {
                c.cycles += 1;
                loop {
                    c.cycles += 11;
                    c.lwc1(0, r(c, SP).wrapping_add(0x24));
                    c.g[V1][0] = sx(r(c, V1).wrapping_sub(1));
                    c.lwc1(1, r(c, SP).wrapping_add(0x14));
                    c.ldl(A3, r(c, SP).wrapping_add(0x27));
                    c.ldr(A3, r(c, SP).wrapping_add(0x20));
                    c.sdl(A3, r(c, A2).wrapping_add(7));
                    c.sdr(A3, r(c, A2));
                    c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[1])).0;
                    c.g[A2][0] = sx(r(c, A2).wrapping_add(8));
                    let more_columns = c.g[V1][0] != 0;
                    c.swc1(0, r(c, SP).wrapping_add(0x24));
                    if !more_columns {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0037_44c8);
                    }
                }
            }
            c.cycles += 7;
            c.lwc1(0, r(c, SP));
            c.g[V1][0] = c.g[V0][0];
            c.lwc1(1, r(c, SP).wrapping_add(0x10));
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[1])).0;
            let more_rows = c.g[V0][0] != 0;
            c.swc1(0, r(c, SP));
            if !more_rows {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0037_44a8);
            }
        }
    }
    c.cycles += 2;
    frame.close(c);
    abi::ret(c);
}
