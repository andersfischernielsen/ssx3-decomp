//! `src/render/render2d.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0038_e0a8,
    render2d_transform_point,
    "Render2D_TransformPoint",
)];

// ---------------------------------------------------------------------------
// 2D rendering

/// `void Render2D_TransformPoint(Vec4 *out, const Vec2 *p, const Mat4 *m)`
/// (0x38e0a8): transform a 2D point by `m` into `out` (w = 1).
pub fn render2d_transform_point(c: &mut Cpu) {
    c.cycles += 28;
    let a0 = c.g[A0][0] as u32;
    let a1 = c.g[A1][0] as u32;
    let a2 = c.g[A2][0] as u32;
    c.f[6] = Addr(a1).off(0x4).read_fpu(c).0;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.f[2] = Addr(a2).off(0xc).read_fpu(c).0;
    c.g[V0][0] = c.g[A0][0];
    c.f[0] = Addr(a2).off(0x4).read_fpu(c).0;
    c.f[3] = Addr(a2).off(0x10).read_fpu(c).0;
    c.f[2] = (FpuF32(c.f[2]) * FpuF32(c.f[6])).0;
    c.f[4] = Addr(a1).read_fpu(c).0;
    c.f[1] = Addr(a2).read_fpu(c).0;
    c.f[3] = (FpuF32(c.f[3]) * FpuF32(c.f[6])).0;
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[4])).0;
    c.f[7] = Addr(a2).off(0x1c).read_fpu(c).0;
    c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[4])).0;
    c.f[5] = Addr(a2).off(0x18).read_fpu(c).0;
    c.g[AT][0] = 0x3f80_0000;
    c.f[4] = c.g[1][0] as u32;
    c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[3])).0;
    let sp = c.g[SP][0] as u32;
    Addr(sp).off(0x8).sw(c, 0);
    c.f[1] = (FpuF32(c.f[1]) + FpuF32(c.f[2])).0;
    Addr(sp).off(0xc).write_fpu(c, FpuF32(c.f[4]));
    c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[7])).0;
    c.f[1] = (FpuF32(c.f[1]) + FpuF32(c.f[5])).0;
    Addr(sp).off(0x4).write_fpu(c, FpuF32(c.f[0]));
    Addr(sp).write_fpu(c, FpuF32(c.f[1]));
    c.lq(3, sp);
    c.sq(3, a0);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    abi::ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    #[test]
    fn render2d_transform_point_matches() {
        check(
            ssx3_ee::s12::f_0038e0a8,
            render2d_transform_point,
            200,
            &|c, seed| {
                let mut r = Rng(0x38e0_a800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.g[A2][0] = (DATA + 0x200) as u64;
                for k in 0..2 {
                    c.bus.write32(DATA + 0x100 + k * 4, r.edge_f32());
                }
                for off in [0, 4, 0xc, 0x10, 0x18, 0x1c] {
                    c.bus.write32(DATA + 0x200 + off, r.edge_f32());
                }
            },
        );
    }
}
