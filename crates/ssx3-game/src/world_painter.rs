//! Work package E3 world painter, irradiance, lighting
//! (docs/investigation/phase2-candidates-3.md).
//!
//! Hand-written, typed replacements for the irradiance accumulators, the
//! world-painter getters, the two `tWPIGD_*_Blend` gradient blends and
//! `WorldPainter_BlendViewColours`, each proven byte-identical against the
//! generated body by a differential test.
//!
//! VU0 macro code is reproduced with [`VuF32`] in the original operation
//! order (a broadcast `MUL` scales every lane by one field, an `ADD` is
//! lane-by-lane), and the VF registers it used as scratch are written back
//! with [`vu0_scratch`]. FPU code uses [`FpuF32`].

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::{vu0_scratch, FpuF32, Vec4, VuF32};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this package: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0038_abf8, irradiance_get_entry, "Irradiance_GetEntry"),
    (0x0038_9260, irradiance_clear, "Irradiance_Clear"),
    (0x0038_9810, irradiance_scale_in_place, "Irradiance_ScaleInPlace"),
    (0x0038_9730, irradiance_add, "Irradiance_Add"),
    (0x0038_9520, irradiance_add_directional, "Irradiance_AddDirectional"),
    (0x0038_ac50, irradiance_find_index, "Irradiance_FindIndex"),
    (0x002b_cea8, twpigd_camera_blend, "tWPIGD_Camera_Blend"),
    (0x002b_cf38, twpigd_fog_blend, "tWPIGD_Fog_Blend"),
    (0x002e_d1d0, world_painter_blend_view_colours, "WorldPainter_BlendViewColours"),
    (0x002e_e400, world_painter_get_sky_box08, "WorldPainter_GetSkyBox08"),
    (0x002e_eed0, world_painter_get_light_glow28, "WorldPainter_GetLightGlow28"),
    (0x002e_e220, world_painter_get_sun_texture, "WorldPainter_GetSunTexture"),
    (0x002e_e3b8, world_painter_get_camera08, "WorldPainter_GetCamera08"),
    (0x002e_e570, world_painter_get_weather20, "WorldPainter_GetWeather20"),
    (0x002e_e7c8, world_painter_get_weather68, "WorldPainter_GetWeather68"),
    (0x002e_e810, world_painter_get_weather70, "WorldPainter_GetWeather70"),
    (0x002e_e858, world_painter_get_weather78, "WorldPainter_GetWeather78"),
    (0x002e_e9c0, world_painter_get_fog08, "WorldPainter_GetFog08"),
    (0x002e_ea08, world_painter_get_fog10, "WorldPainter_GetFog10"),
];

const ADD_DIRECTIONAL_SCALED: Callee = Callee {
    addr: 0x0038_9308,
    f: ee::s12::Irradiance_AddDirectionalScaled,
};

// ---------------------------------------------------------------------------
// Irradiance.

/// `Irradiance* Irradiance_GetEntry(Irradiance *irr, int index)` (0x38abf8):
/// the record at `base + min(index, count-1) * 0xa0`; `count` is at +4, the
/// base pointer at +8. The `mult` also leaves LO/HI as compiled code does.
pub fn irradiance_get_entry(c: &mut Cpu) {
    c.cycles += 9;
    let this = Addr(abi::arg(c, 0));
    c.g[A2][0] = this.off(4).lw(c);
    c.g[A3][0] = 0xa0;
    c.g[V0][0] = this.off(8).lw(c);
    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
    c.g[A0][0] = ((c.g[A1][0] as i64) < (c.g[A2][0] as i64)) as u64;
    if c.g[A0][0] != 0 {
        c.g[V1][0] = c.g[A1][0];
    }
    c.mult(0, V1, V1, A3);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    abi::ret(c);
}

/// `void Irradiance_Clear(Irradiance *irr)` (0x389260): zero the ten records
/// (0xa0 bytes) at `irr`, one quad per iteration, with an event check on the
/// back edge.
pub fn irradiance_clear(c: &mut Cpu) {
    const HEAD: u32 = 0x0038_9268;
    c.cycles += 2;
    c.f[0] = 0; // mtc1 zero
    c.g[V0][0] = 0x9;
    loop {
        c.cycles += 7;
        Addr(r32(c, A0).wrapping_add(0xc)).write_fpu(c, FpuF32::ZERO);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        Addr(r32(c, A0).wrapping_add(8)).write_fpu(c, FpuF32::ZERO);
        Addr(r32(c, A0).wrapping_add(4)).write_fpu(c, FpuF32::ZERO);
        Addr(r32(c, A0)).write_fpu(c, FpuF32::ZERO);
        let more = (c.g[V0][0] as i64) >= 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
        if more {
            if event_due(c) {
                deliver_event_here(c, HEAD);
            }
        } else {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
    }
}

/// `void Irradiance_ScaleInPlace(Irradiance *irr, float scale)` (0x389810):
/// multiply each of the ten quads at `irr` by the broadcast scalar `f12`.
/// The scalar travels `mfc1 → qmtc2 vf3 → MUL(Bc vf3.X)`.
pub fn irradiance_scale_in_place(c: &mut Cpu) {
    const HEAD: u32 = 0x0038_9818;
    c.cycles += 2;
    c.g[V1][0] = c.g[A0][0];
    c.g[A1][0] = 0x9;
    loop {
        c.cycles += 8;
        c.mfc1(V0, 12);
        c.lqc2(4, r32(c, V1));
        c.qmtc2(V0, 3);
        let v3 = c.bus.plat.vf(3);
        let v4 = c.bus.plat.vf(4);
        let s = VuF32(v3[0]);
        let scaled = Vec4 {
            x: VuF32(v4[0]) * s,
            y: VuF32(v4[1]) * s,
            z: VuF32(v4[2]) * s,
            w: VuF32(v4[3]) * s,
        };
        vu0_scratch::set_vf(c, 5, 0b1111, scaled);
        c.sqc2(5, r32(c, V1));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        let more = (c.g[A1][0] as i64) >= 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
        if more {
            if event_due(c) {
                deliver_event_here(c, HEAD);
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.g[A0][0];
            abi::ret(c);
            return;
        }
    }
}

/// `void Irradiance_Add(Irradiance *dst, const Irradiance *a, const
/// Irradiance *b)` (0x389730): add ten quads lane-by-lane (`vf3 + vf4`),
/// staging each result on the stack, then copy the 0xa0 bytes to `dst`.
pub fn irradiance_add(c: &mut Cpu) {
    const HEAD1: u32 = 0x0038_9740;
    const HEAD2: u32 = 0x0038_9778;
    c.cycles += 4;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
    c.g[A3][0] = 0x9;
    c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
    c.g[V1][0] = c.g[SP][0];
    loop {
        c.cycles += 12;
        c.lqc2(3, r32(c, A1));
        c.lqc2(4, r32(c, A2));
        let v3 = c.bus.plat.vf(3);
        let v4 = c.bus.plat.vf(4);
        let sum = Vec4 {
            x: VuF32(v3[0]) + VuF32(v4[0]),
            y: VuF32(v3[1]) + VuF32(v4[1]),
            z: VuF32(v3[2]) + VuF32(v4[2]),
            w: VuF32(v3[3]) + VuF32(v4[3]),
        };
        vu0_scratch::set_vf(c, 5, 0b1111, sum);
        c.sqc2(5, r32(c, SP).wrapping_add(0xb0));
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
        c.lq(2, r32(c, SP).wrapping_add(0xb0));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
        c.sq(2, r32(c, SP).wrapping_add(0xa0));
        c.sq(2, r32(c, V1));
        let more = (c.g[A3][0] as i64) >= 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
        if more {
            if event_due(c) {
                deliver_event_here(c, HEAD1);
            }
            continue;
        }
        c.cycles += 2;
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = c.g[SP][0];
        loop {
            c.cycles += 7;
            c.lq(5, r32(c, V0));
            c.lq(6, r32(c, V0).wrapping_add(0x10));
            c.sq(5, r32(c, V1));
            c.sq(6, r32(c, V1).wrapping_add(0x10));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
            let more = c.g[V0][0] != c.g[T0][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
            if more {
                if event_due(c) {
                    deliver_event_here(c, HEAD2);
                }
                continue;
            }
            c.cycles += 3;
            c.g[V0][0] = c.g[A0][0];
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
            abi::ret(c);
            return;
        }
    }
}

/// `void Irradiance_AddDirectional(Irradiance *irr, Direction *dir, void *p)`
/// (0x389520): forward to `Irradiance_AddDirectionalScaled(irr, p, 1.0,
/// dir->x, dir->y, dir->z)`.
pub fn irradiance_add_directional(c: &mut Cpu) {
    const RET: u32 = 0x0038_9548;
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A2][0];
        c.g[AT][0] = 0x3f80_0000;
        c.f[12] = 0x3f80_0000; // mtc1 1, f12 (scale 1.0)
        c.f[15] = Addr(r32(c, V0).wrapping_add(8)).read_u32(c);
        c.f[13] = Addr(r32(c, V0)).read_u32(c);
        c.g[RA][0] = sx(RET);
        c.f[14] = Addr(r32(c, V0).wrapping_add(4)).read_u32(c);
        ADD_DIRECTIONAL_SCALED.call(c, RET)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int Irradiance_FindIndex(IrradianceSet *set, const char *name)`
/// (0x38ac50): linear scan of the entries (`stride 8`, pointer at +0xc,
/// count at +4), `strcmp` each against `name`; returns the matching index,
/// else 0 (also when the count is non-positive).
pub fn irradiance_find_index(c: &mut Cpu) {
    const RET: u32 = 0x0038_ac90;
    const HEAD: u32 = 0x0038_ac80;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S1)).off(4).lw(c);
        c.g[S2][0] = c.g[A1][0];
        if (c.g[V0][0] as i64) <= 0 {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 2;
            c.g[V0][0] = Addr(r32(c, S1)).off(0xc).lw(c);
            loop {
                c.cycles += 6;
                c.g[A1][0] = sx((c.g[S0][0] as u32) << 3);
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(RET);
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                call_ptr(c, 0x0041_65a8, RET)?; // strcmp
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = Addr(r32(c, S1)).off(4).lw(c);
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S0][0];
                    break;
                }
                c.cycles += 4;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = Addr(r32(c, S1)).off(0xc).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, HEAD);
                    }
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    break;
                }
            }
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

// ---------------------------------------------------------------------------
// tWPIGD blends.

/// `void tWPIGD_Camera_Blend(tWPIGD *self, const BlendSrc *src, float t)`
/// (0x2bcea8): `vtable->get(self)` (twice), then a quadratic blend of the
/// camera colour/alpha fields with the source.
pub fn twpigd_camera_blend(c: &mut Cpu) {
    const RET1: u32 = 0x002b_cee0;
    const RET2: u32 = 0x002b_cf20;
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x30);
        c.f[20] = c.f[12]; // mov_s f20, f12
        c.g[V1][0] = Addr(r32(c, S0)).off(4).lw(c);
        c.f[20] = (FpuF32(c.f[20]) * FpuF32(c.f[20])).0;
        c.g[A0][0] = Addr(r32(c, V1)).off(0x218).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x21c).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(RET1);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
        call_ptr(c, target, RET1)?;

        let t2 = FpuF32(c.f[20]);
        c.g[AT][0] = 0x3f80_0000;
        c.f[0] = FpuF32::ONE.0; // mtc1 1, f0 (1.0)
        c.f[1] = Addr(r32(c, S1)).off(4).read_u32(c);
        c.f[0] = (FpuF32::ONE - t2).0; // 1.0 - t^2
        c.f[2] = Addr(r32(c, S0)).off(8).read_u32(c);
        c.f[20] = (t2 * FpuF32(c.f[1])).0;
        c.g[V1][0] = Addr(r32(c, S0)).off(4).lw(c);
        c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[2])).0;
        c.f[20] = (FpuF32(c.f[20]) + FpuF32(c.f[0])).0;
        Addr(r32(c, S0)).off(8).write_u32(c, c.f[20]);
        c.f[0] = Addr(r32(c, S1)).off(4).read_u32(c);
        Addr(r32(c, S0)).off(0xc).write_u32(c, c.f[0]);
        c.g[A0][0] = Addr(r32(c, V1)).off(0x218).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x21c).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(RET2);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
        call_ptr(c, target, RET2)?;

        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void tWPIGD_Fog_Blend(tWPIGD *self, const BlendSrc *src, float t)`
/// (0x2bcf38): `vtable->get(self)` (twice), then blend the fog colour ramp
/// (six RGBA pairs at self+8..+0x34) with the source by `1-t^2` / `t^2`.
pub fn twpigd_fog_blend(c: &mut Cpu) {
    const RET1: u32 = 0x002b_cf70;
    const RET2: u32 = 0x002b_d050;
    guest(c, |c| {
        c.cycles += 76;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x30);
        c.f[20] = c.f[12];
        c.g[V1][0] = Addr(r32(c, S0)).off(4).lw(c);
        c.f[20] = (FpuF32(c.f[20]) * FpuF32(c.f[20])).0;
        c.g[A0][0] = Addr(r32(c, V1)).off(0x218).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x21c).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(RET1);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
        call_ptr(c, target, RET1)?;

        let f20 = FpuF32(c.f[20]);
        c.g[AT][0] = 0x3f80_0000;
        c.f[2] = FpuF32::ONE.0; // mtc1 1, f2
        c.f[1] = Addr(r32(c, S1)).off(4).read_u32(c);
        c.f[2] = (FpuF32::ONE - f20).0;
        c.f[0] = Addr(r32(c, S0)).off(8).read_u32(c);
        c.f[1] = (f20 * FpuF32(c.f[1])).0;
        c.f[3] = Addr(r32(c, S0)).off(0x10).read_u32(c);
        c.f[4] = Addr(r32(c, S0)).off(0x18).read_u32(c);
        c.f[0] = (FpuF32(c.f[2]) * FpuF32(c.f[0])).0;
        c.f[6] = Addr(r32(c, S0)).off(0x20).read_u32(c);
        c.f[3] = (FpuF32(c.f[2]) * FpuF32(c.f[3])).0;
        c.f[7] = Addr(r32(c, S0)).off(0x28).read_u32(c);
        c.f[4] = (FpuF32(c.f[2]) * FpuF32(c.f[4])).0;
        c.f[5] = Addr(r32(c, S0)).off(0x30).read_u32(c);
        c.f[1] = (FpuF32(c.f[1]) + FpuF32(c.f[0])).0;
        c.g[V1][0] = Addr(r32(c, S0)).off(4).lw(c);
        c.f[6] = (FpuF32(c.f[2]) * FpuF32(c.f[6])).0;
        c.f[7] = (FpuF32(c.f[2]) * FpuF32(c.f[7])).0;
        Addr(r32(c, S0)).off(8).write_u32(c, c.f[1]);
        c.f[2] = (FpuF32(c.f[2]) * FpuF32(c.f[5])).0;
        c.f[1] = Addr(r32(c, S1)).off(4).read_u32(c);
        Addr(r32(c, S0)).off(0xc).write_u32(c, c.f[1]);
        c.f[0] = Addr(r32(c, S1)).off(8).read_u32(c);
        c.f[0] = (f20 * FpuF32(c.f[0])).0;
        c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[3])).0;
        Addr(r32(c, S0)).off(0x10).write_u32(c, c.f[0]);
        c.f[1] = Addr(r32(c, S1)).off(8).read_u32(c);
        Addr(r32(c, S0)).off(0x14).write_u32(c, c.f[1]);
        c.f[0] = Addr(r32(c, S1)).off(0xc).read_u32(c);
        c.f[0] = (f20 * FpuF32(c.f[0])).0;
        c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[4])).0;
        Addr(r32(c, S0)).off(0x18).write_u32(c, c.f[0]);
        c.f[1] = Addr(r32(c, S1)).off(0xc).read_u32(c);
        Addr(r32(c, S0)).off(0x1c).write_u32(c, c.f[1]);
        c.f[0] = Addr(r32(c, S1)).off(0x10).read_u32(c);
        c.f[0] = (f20 * FpuF32(c.f[0])).0;
        c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[6])).0;
        Addr(r32(c, S0)).off(0x20).write_u32(c, c.f[0]);
        c.f[1] = Addr(r32(c, S1)).off(0x10).read_u32(c);
        Addr(r32(c, S0)).off(0x24).write_u32(c, c.f[1]);
        c.f[0] = Addr(r32(c, S1)).off(0x14).read_u32(c);
        c.f[0] = (f20 * FpuF32(c.f[0])).0;
        c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[7])).0;
        Addr(r32(c, S0)).off(0x28).write_u32(c, c.f[0]);
        c.f[1] = Addr(r32(c, S1)).off(0x14).read_u32(c);
        Addr(r32(c, S0)).off(0x2c).write_u32(c, c.f[1]);
        c.f[0] = Addr(r32(c, S1)).off(0x18).read_u32(c);
        c.f[20] = (f20 * FpuF32(c.f[0])).0;
        c.f[20] = (FpuF32(c.f[20]) + FpuF32(c.f[2])).0;
        Addr(r32(c, S0)).off(0x30).write_u32(c, c.f[20]);
        c.f[0] = Addr(r32(c, S1)).off(0x18).read_u32(c);
        Addr(r32(c, S0)).off(0x34).write_u32(c, c.f[0]);

        c.g[A0][0] = Addr(r32(c, V1)).off(0x218).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x21c).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(RET2);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
        call_ptr(c, target, RET2)?;

        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// World painter.

/// `void WorldPainter_BlendViewColours(ViewColours *out, const float *a,
/// const float *b, int immediate)` (0x2ed1d0): blend two 4-float view-colour
/// blocks (`(1-k)*a + k*b`) into `out+0x28` and `out+0x38`, where `k` comes
/// from a global constant unless `immediate` is set (then 1.0). The
/// `ldl/ldr`+`sdl/sdr` pairs are byte-identical unaligned 8-byte copies.
pub fn world_painter_blend_view_colours(c: &mut Cpu) {
    c.cycles += 3;
    c.lwc1(9, r32(c, GP).wrapping_sub(0x3978));
    let zero = c.g[A3][0] == 0;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
    if !zero {
        c.cycles += 2;
        c.g[AT][0] = 0x3f80_0000;
        c.f[9] = 0x3f80_0000; // mtc1 1, f9
    }

    c.cycles += 85;
    c.g[AT][0] = 0x3f80_0000;
    c.f[0] = 0x3f80_0000; // 1.0
    c.lwc1(5, r32(c, A0).wrapping_add(0x28));
    c.f[0] = (FpuF32::ONE - FpuF32(c.f[9])).0;
    c.lwc1(6, r32(c, A0).wrapping_add(0x2c));
    c.lwc1(3, r32(c, A0).wrapping_add(0x30));
    c.lwc1(4, r32(c, A0).wrapping_add(0x34));
    c.lwc1(8, r32(c, A1).wrapping_add(0xc));
    c.f[5] = (FpuF32(c.f[5]) * FpuF32(c.f[0])).0;
    c.lwc1(1, r32(c, A1));
    c.f[6] = (FpuF32(c.f[6]) * FpuF32(c.f[0])).0;
    c.lwc1(2, r32(c, A1).wrapping_add(4));
    c.f[3] = (FpuF32(c.f[3]) * FpuF32(c.f[0])).0;
    c.lwc1(7, r32(c, A1).wrapping_add(8));
    c.f[4] = (FpuF32(c.f[4]) * FpuF32(c.f[0])).0;
    c.f[8] = (FpuF32(c.f[8]) * FpuF32(c.f[9])).0;
    Addr(r32(c, SP).wrapping_add(0x10)).write_u32(c, c.f[5]);
    c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[9])).0;
    Addr(r32(c, SP).wrapping_add(0x14)).write_u32(c, c.f[6]);
    c.f[2] = (FpuF32(c.f[2]) * FpuF32(c.f[9])).0;
    Addr(r32(c, SP).wrapping_add(0x18)).write_u32(c, c.f[3]);
    c.f[7] = (FpuF32(c.f[7]) * FpuF32(c.f[9])).0;
    Addr(r32(c, SP).wrapping_add(0x1c)).write_u32(c, c.f[4]);
    c.f[5] = (FpuF32(c.f[5]) + FpuF32(c.f[1])).0;
    Addr(r32(c, SP).wrapping_add(0x20)).write_u32(c, c.f[1]);
    c.f[6] = (FpuF32(c.f[6]) + FpuF32(c.f[2])).0;
    Addr(r32(c, SP).wrapping_add(0x24)).write_u32(c, c.f[2]);
    c.f[3] = (FpuF32(c.f[3]) + FpuF32(c.f[7])).0;
    Addr(r32(c, SP).wrapping_add(0x28)).write_u32(c, c.f[7]);
    c.f[4] = (FpuF32(c.f[4]) + FpuF32(c.f[8])).0;
    Addr(r32(c, SP)).write_u32(c, c.f[5]);
    Addr(r32(c, SP).wrapping_add(4)).write_u32(c, c.f[6]);
    Addr(r32(c, SP).wrapping_add(8)).write_u32(c, c.f[3]);
    Addr(r32(c, SP).wrapping_add(0xc)).write_u32(c, c.f[4]);
    Addr(r32(c, SP).wrapping_add(0x2c)).write_u32(c, c.f[8]);
    let g2 = Addr(r32(c, SP)).read_u64_unaligned(c);
    let g3 = Addr(r32(c, SP).wrapping_add(8)).read_u64_unaligned(c);
    c.g[V0][0] = g2;
    c.g[V1][0] = g3;
    Addr(r32(c, A0).wrapping_add(0x28)).write_u64_unaligned(c, g2);
    Addr(r32(c, A0).wrapping_add(0x30)).write_u64_unaligned(c, g3);

    // Second block: out+0x38 and out+0x40 from `b`.
    c.lwc1(4, r32(c, A0).wrapping_add(0x38));
    c.lwc1(5, r32(c, A0).wrapping_add(0x3c));
    c.lwc1(3, r32(c, A0).wrapping_add(0x40));
    c.f[4] = (FpuF32(c.f[4]) * FpuF32(c.f[0])).0;
    c.lwc1(2, r32(c, A0).wrapping_add(0x44));
    c.f[5] = (FpuF32(c.f[5]) * FpuF32(c.f[0])).0;
    c.lwc1(8, r32(c, A2).wrapping_add(0xc));
    c.f[3] = (FpuF32(c.f[3]) * FpuF32(c.f[0])).0;
    c.lwc1(1, r32(c, A2));
    c.f[2] = (FpuF32(c.f[2]) * FpuF32(c.f[0])).0;
    c.lwc1(7, r32(c, A2).wrapping_add(4));
    c.f[8] = (FpuF32(c.f[8]) * FpuF32(c.f[9])).0;
    c.lwc1(6, r32(c, A2).wrapping_add(8));
    c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[9])).0;
    c.f[7] = (FpuF32(c.f[7]) * FpuF32(c.f[9])).0;
    Addr(r32(c, SP).wrapping_add(0x10)).write_u32(c, c.f[4]);
    c.f[6] = (FpuF32(c.f[6]) * FpuF32(c.f[9])).0;
    Addr(r32(c, SP).wrapping_add(0x14)).write_u32(c, c.f[5]);
    Addr(r32(c, SP).wrapping_add(0x18)).write_u32(c, c.f[3]);
    c.f[4] = (FpuF32(c.f[4]) + FpuF32(c.f[1])).0;
    Addr(r32(c, SP).wrapping_add(0x1c)).write_u32(c, c.f[2]);
    c.f[5] = (FpuF32(c.f[5]) + FpuF32(c.f[7])).0;
    c.f[3] = (FpuF32(c.f[3]) + FpuF32(c.f[6])).0;
    Addr(r32(c, SP).wrapping_add(0x20)).write_u32(c, c.f[1]);
    c.f[2] = (FpuF32(c.f[2]) + FpuF32(c.f[8])).0;
    Addr(r32(c, SP)).write_u32(c, c.f[4]);
    Addr(r32(c, SP).wrapping_add(4)).write_u32(c, c.f[5]);
    Addr(r32(c, SP).wrapping_add(8)).write_u32(c, c.f[3]);
    Addr(r32(c, SP).wrapping_add(0xc)).write_u32(c, c.f[2]);
    Addr(r32(c, SP).wrapping_add(0x24)).write_u32(c, c.f[7]);
    Addr(r32(c, SP).wrapping_add(0x28)).write_u32(c, c.f[6]);
    Addr(r32(c, SP).wrapping_add(0x2c)).write_u32(c, c.f[8]);
    let g2 = Addr(r32(c, SP)).read_u64_unaligned(c);
    let g3 = Addr(r32(c, SP).wrapping_add(8)).read_u64_unaligned(c);
    c.g[V0][0] = g2;
    c.g[V1][0] = g3;
    Addr(r32(c, A0).wrapping_add(0x38)).write_u64_unaligned(c, g2);
    Addr(r32(c, A0).wrapping_add(0x40)).write_u64_unaligned(c, g3);

    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    abi::ret(c);
}

/// The result kind of the world-painter virtual getters.
enum GetterResult {
    /// The method returns a pointer whose first word is returned (`lw`).
    Word,
    /// The method returns a pointer whose first float is returned (`lwc1`).
    Float,
}

/// The 0x2ee*** family all do the same thing: index a global table
/// (`&DAT_004fa380 + index * 0xf0`), look up `holder = table[index].field`,
/// `obj = *holder`, `vtable = obj->vtable`, then call
/// `vtable[vt_off/4](obj + delta)` through the function table and return
/// what it points at.
fn world_painter_virtual_getter(
    c: &mut Cpu,
    table_off: u32,
    vt_off: u32,
    ret: u32,
    result: GetterResult,
) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[V1][0] = 0xf0;
        let frame = Frame::open(c, 0x10);
        c.mult(0, A0, A0, V1); // a0 = index * 0xf0
        c.g[V0][0] = 0x50_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x50_0000u32.wrapping_sub(0x5c90));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
        c.g[V1][0] = Addr(r32(c, A0)).off(table_off).lw(c);
        c.g[A2][0] = Addr(r32(c, V1)).lw(c);
        c.g[A1][0] = Addr(r32(c, A2)).off(4).lw(c);
        c.g[A0][0] = Addr(r32(c, A1)).off(vt_off).lh(c);
        c.g[V0][0] = Addr(r32(c, A1)).off(vt_off + 4).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(ret);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
        call_ptr(c, target, ret)?;
        let p = r32(c, V0);
        match result {
            GetterResult::Float => c.lwc1(0, p),
            GetterResult::Word => c.g[V0][0] = Addr(p).lw(c),
        }
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float WorldPainter_GetSkyBox08(int index)` (0x2ee400).
pub fn world_painter_get_sky_box08(c: &mut Cpu) {
    world_painter_virtual_getter(c, 0x10, 0xd0, 0x002e_e438, GetterResult::Float);
}

/// `float WorldPainter_GetLightGlow28(int index)` (0x2eeed0).
pub fn world_painter_get_light_glow28(c: &mut Cpu) {
    world_painter_virtual_getter(c, 0x8, 0x88, 0x002e_ef08, GetterResult::Float);
}

/// `int WorldPainter_GetSunTexture(int index)` (0x2ee220).
pub fn world_painter_get_sun_texture(c: &mut Cpu) {
    world_painter_virtual_getter(c, 0x14, 0x108, 0x002e_e258, GetterResult::Word);
}

/// `float WorldPainter_GetCamera08(int index)` (0x2ee3b8).
pub fn world_painter_get_camera08(c: &mut Cpu) {
    world_painter_virtual_getter(c, 0x0, 0x30, 0x002e_e3f0, GetterResult::Float);
}

/// `float WorldPainter_GetWeather20(int index)` (0x2ee570).
pub fn world_painter_get_weather20(c: &mut Cpu) {
    world_painter_virtual_getter(c, 0x20, 0x170, 0x002e_e5a8, GetterResult::Float);
}

/// `float WorldPainter_GetWeather68(int index)` (0x2ee7c8).
pub fn world_painter_get_weather68(c: &mut Cpu) {
    world_painter_virtual_getter(c, 0x20, 0x1b8, 0x002e_e800, GetterResult::Float);
}

/// `float WorldPainter_GetWeather70(int index)` (0x2ee810).
pub fn world_painter_get_weather70(c: &mut Cpu) {
    world_painter_virtual_getter(c, 0x20, 0x1c0, 0x002e_e848, GetterResult::Float);
}

/// `float WorldPainter_GetWeather78(int index)` (0x2ee858).
pub fn world_painter_get_weather78(c: &mut Cpu) {
    world_painter_virtual_getter(c, 0x20, 0x1c8, 0x002e_e890, GetterResult::Float);
}

/// `float WorldPainter_GetFog08(int index)` (0x2ee9c0).
pub fn world_painter_get_fog08(c: &mut Cpu) {
    world_painter_virtual_getter(c, 0x4, 0x38, 0x002e_e9f8, GetterResult::Float);
}

/// `float WorldPainter_GetFog10(int index)` (0x2eea08).
pub fn world_painter_get_fog10(c: &mut Cpu) {
    world_painter_virtual_getter(c, 0x4, 0x40, 0x002e_ea40, GetterResult::Float);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, FAKE};
    use crate::difftest::{check, check_lohi, scramble, Rng, DATA, GAME_GP};

    // --------------------------------------------------------------- irradiance

    #[test]
    fn irradiance_get_entry_matches() {
        check_lohi(ee::s12::f_0038abf8, irradiance_get_entry, 400, &|c, seed| {
            let mut r = Rng(0x0e37_0001 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let index = match seed % 3 {
                0 => 0u64,
                1 => (seed / 3 % 5) as u64,
                _ => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
            };
            c.g[A1][0] = index;
            let count = if seed % 7 == 0 { -1i32 } else { (seed % 4) as i32 };
            c.bus.write32(DATA + 4, count as u32);
            c.bus.write32(DATA + 8, r.next_u32());
        });
    }

    #[test]
    fn irradiance_clear_matches() {
        check(ee::s12::f_00389260, irradiance_clear, 100, &|c, seed| {
            let mut r = Rng(0x0e37_0002 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = (DATA + 0x1000) as u64;
            for i in 0..0x40 {
                c.bus.write32(DATA + 0x1000 + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn irradiance_scale_in_place_matches() {
        check(ee::s12::f_00389810, irradiance_scale_in_place, 200, &|c, seed| {
            let mut r = Rng(0x0e37_0003 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = (DATA + 0x1000) as u64;
            c.f[12] = r.edge_f32();
            for i in 0..0x30 {
                c.bus.write32(DATA + 0x1000 + i * 4, r.edge_f32());
            }
        });
    }

    #[test]
    fn irradiance_add_matches() {
        check(ee::s12::f_00389730, irradiance_add, 200, &|c, seed| {
            let mut r = Rng(0x0e37_0004 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = (DATA + 0x1000) as u64;
            c.g[A1][0] = (DATA + 0x2000) as u64;
            c.g[A2][0] = (DATA + 0x3000) as u64;
            for i in 0..0x40 {
                c.bus.write32(DATA + 0x2000 + i * 4, r.edge_f32());
                c.bus.write32(DATA + 0x3000 + i * 4, r.edge_f32());
            }
        });
    }

    #[test]
    fn irradiance_add_directional_matches() {
        check(ee::s12::f_00389520, irradiance_add_directional, 60, &|c, seed| {
            let mut r = Rng(0x0e37_0005 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = (DATA + 0x1000) as u64;
            c.g[A1][0] = (DATA + 0x2000) as u64;
            c.g[A2][0] = (DATA + 0x4000) as u64;
            for i in 0..0x100 {
                c.bus.write32(DATA + 0x1000 + i * 4, r.next_u32());
            }
            for i in 0..0x100 {
                c.bus.write32(DATA + 0x2000 + i * 4, r.edge_f32());
            }
            for i in 0..0x100 {
                c.bus.write32(DATA + 0x4000 + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn irradiance_find_index_matches() {
        check(ee::s12::f_0038ac50, irradiance_find_index, 100, &|c, seed| {
            let mut r = Rng(0x0e37_0006 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let set = DATA + 0x100;
            c.g[A0][0] = set as u64;
            c.g[A1][0] = (DATA + 0x300) as u64;
            let base = DATA + 0x200;
            let count = if seed % 11 == 0 { 0 } else { 3 };
            c.bus.write32(set + 4, count);
            c.bus.write32(set + 0xc, base);
            for (i, name) in [b"aaa\0".as_slice(), b"bbb\0", b"ccc\0"].iter().enumerate() {
                for (j, b) in name.iter().enumerate() {
                    c.bus.write8(base + (i * 8) as u32 + j as u32, *b);
                }
            }
            let name: &[u8] = if seed % 2 == 0 { b"bbb\0" } else { b"zzz\0" };
            for (j, b) in name.iter().enumerate() {
                c.bus.write8(DATA + 0x300 + j as u32, *b);
            }
        });
    }

    // ------------------------------------------------------------- tWPIGD blends

    fn blend_setup(c: &mut Cpu, seed: u32, s0_off: u32, s1_off: u32) {
        let mut r = fakes::machine(c, seed, 0x0e37_00b1, 2);
        let selfp = DATA + 0x1000;
        let src = DATA + 0x5000;
        c.g[A0][0] = selfp as u64;
        c.g[A1][0] = src as u64;
        c.f[12] = r.edge_f32();
        // `self` and `src` fields the blend reads/writes.
        for i in 0..(s0_off / 4 + 8) {
            c.bus.write32(selfp + i * 4, r.edge_f32());
        }
        for i in 0..(s1_off / 4 + 8) {
            c.bus.write32(src + i * 4, r.edge_f32());
        }
        let vt = DATA + 0x3000;
        c.bus.write32(selfp + 4, vt);
        let delta = (r.next_u32() as i16 as i32 & !3) as u32;
        c.bus.write16(vt + 0x218, delta as u16);
        c.bus.write32(vt + 0x21c, FAKE);
    }

    #[test]
    fn twpigd_camera_blend_matches() {
        fakes::covered(ee::s08::f_002bcea8, twpigd_camera_blend, 200, &|c, seed| {
            blend_setup(c, seed, 0xf, 0xf);
        });
    }

    #[test]
    fn twpigd_fog_blend_matches() {
        fakes::covered(ee::s08::f_002bcf38, twpigd_fog_blend, 200, &|c, seed| {
            blend_setup(c, seed, 0x37, 0x1b);
        });
    }

    // -------------------------------------------------------- blend view colours

    #[test]
    fn world_painter_blend_view_colours_matches() {
        check(ee::s09::f_002ed1d0, world_painter_blend_view_colours, 200, &|c, seed| {
            let mut r = Rng(0x0e37_0007 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = (DATA + 0x1000) as u64;
            c.g[A1][0] = (DATA + 0x2000) as u64;
            c.g[A2][0] = (DATA + 0x3000) as u64;
            c.g[A3][0] = if seed % 2 == 0 { 0 } else { r.next_u32() as u64 };
            c.bus.write32(GAME_GP.wrapping_sub(0x3978), r.edge_f32()); // global k
            for i in 0..0x20 {
                c.bus.write32(DATA + 0x1000 + i * 4, r.edge_f32());
                c.bus.write32(DATA + 0x2000 + i * 4, r.edge_f32());
                c.bus.write32(DATA + 0x3000 + i * 4, r.edge_f32());
            }
        });
    }

    // ------------------------------------------------------------- getters

    fn getter_setup(c: &mut Cpu, seed: u32, table_off: u32, vt_off: u32) {
        let mut r = fakes::machine(c, seed, 0x0e37_00aa, 1);
        let index = seed % 4;
        c.g[A0][0] = index as u64;
        let entry = 0x004f_a370u32.wrapping_add(index.wrapping_mul(0xf0));
        let holder = fakes::ptr(&mut r);
        let obj = fakes::ptr(&mut r);
        let vt = fakes::ptr(&mut r);
        c.bus.write32(entry.wrapping_add(table_off), holder);
        c.bus.write32(holder, obj);
        c.bus.write32(obj.wrapping_add(4), vt);
        let delta = (r.next_u32() as i16 as i32 & !3) as u32;
        c.bus.write16(vt.wrapping_add(vt_off), delta as u16);
        c.bus.write32(vt.wrapping_add(vt_off).wrapping_add(4), FAKE);
    }

    #[test]
    fn world_painter_get_sky_box08_matches() {
        fakes::covered(ee::s09::f_002ee400, world_painter_get_sky_box08, 200, &|c, seed| {
            getter_setup(c, seed, 0x10, 0xd0);
        });
    }

    #[test]
    fn world_painter_get_light_glow28_matches() {
        fakes::covered(ee::s09::f_002eeed0, world_painter_get_light_glow28, 200, &|c, seed| {
            getter_setup(c, seed, 0x8, 0x88);
        });
    }

    #[test]
    fn world_painter_get_sun_texture_matches() {
        fakes::covered(ee::s09::f_002ee220, world_painter_get_sun_texture, 200, &|c, seed| {
            getter_setup(c, seed, 0x14, 0x108);
        });
    }

    #[test]
    fn world_painter_get_camera08_matches() {
        fakes::covered(ee::s09::f_002ee3b8, world_painter_get_camera08, 200, &|c, seed| {
            getter_setup(c, seed, 0x0, 0x30);
        });
    }

    #[test]
    fn world_painter_get_weather20_matches() {
        fakes::covered(ee::s09::f_002ee570, world_painter_get_weather20, 200, &|c, seed| {
            getter_setup(c, seed, 0x20, 0x170);
        });
    }

    #[test]
    fn world_painter_get_weather68_matches() {
        fakes::covered(ee::s09::f_002ee7c8, world_painter_get_weather68, 200, &|c, seed| {
            getter_setup(c, seed, 0x20, 0x1b8);
        });
    }

    #[test]
    fn world_painter_get_weather70_matches() {
        fakes::covered(ee::s09::f_002ee810, world_painter_get_weather70, 200, &|c, seed| {
            getter_setup(c, seed, 0x20, 0x1c0);
        });
    }

    #[test]
    fn world_painter_get_weather78_matches() {
        fakes::covered(ee::s09::f_002ee858, world_painter_get_weather78, 200, &|c, seed| {
            getter_setup(c, seed, 0x20, 0x1c8);
        });
    }

    #[test]
    fn world_painter_get_fog08_matches() {
        fakes::covered(ee::s09::f_002ee9c0, world_painter_get_fog08, 200, &|c, seed| {
            getter_setup(c, seed, 0x4, 0x38);
        });
    }

    #[test]
    fn world_painter_get_fog10_matches() {
        fakes::covered(ee::s09::f_002eea08, world_painter_get_fog10, 200, &|c, seed| {
            getter_setup(c, seed, 0x4, 0x40);
        });
    }
}
