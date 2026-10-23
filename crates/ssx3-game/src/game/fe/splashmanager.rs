//! `src/fe/splashmanager.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::game::ai::rider::per_tick_60;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::{W, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002f_39c8, c_splash_manager_reset, "cSplashManager_Reset"),
    (0x002f_39e0, c_splash_manager_update, "cSplashManager_Update"),
    (0x002f_2810, c_splash_manager_update_drop, "cSplashManager_UpdateDrop"),
    (0x002f_3030, c_splash_manager_update_crystal, "cSplashManager_UpdateCrystal"),
    (0x002f_4260, c_splash_manager_add_splash_from_point, "cSplashManager_AddSplashFromPoint"),
    (0x002f_4330, c_splash_manager_set_snow_amount, "cSplashManager_SetSnowAmount"),
];


// ---------------------------------------------------------------------------
// Splash manager.

/// `void cSplashManager_Reset(cSplashManager *this)` (0x2f39c8): clear the
/// counters and timers.
pub fn c_splash_manager_reset(c: &mut Cpu) {
    c.cycles += 6;
    let this = abi::arg(c, 0);
    Addr(this).off(0x1028).sw(c, 0);
    Addr(this).off(0x100c).sw(c, 0);
    Addr(this).off(0x1024).sw(c, 0);
    Addr(this).off(0x14).sw(c, 0);
    Addr(this).off(0x18).sw(c, 0);
    abi::ret(c);
}


/// `void cSplashManager_SetSnowAmount(cSplashManager *this, float amount)`
/// (0x2f4330): store the amount and, when it crosses the global threshold,
/// integrate it into the splash accumulation at +0x1024.
pub fn c_splash_manager_set_snow_amount(c: &mut Cpu) {
    c.cycles += 6;
    let this = abi::arg(c, 0);
    c.swc1(12, this.wrapping_add(0x1028));
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x1190));
    c.c_lt_s(0, 12);
    let below = !c.fcc();
    c.lwc1(2, (c.g[GP][0] as u32).wrapping_add(0x1194));
    if below {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 11;
    c.g[AT][0] = 0x4120_0000; // 10.0
    c.mtc1(AT, 1);
    c.lwc1(0, this.wrapping_add(0x1020));
    c.add_s(0, 0, 1);
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x38f4));
    c.lwc1(1, this.wrapping_add(0x1024));
    c.mul_s(2, 2, 3);
    c.mul_s(0, 12, 0);
    c.mul_s(0, 0, 2);
    // Tick rate: SnowFall_Update calls this once per tick, so the pending
    // lens splats grow by the per-60-Hz-tick increment * 60/rate (nothing
    // changes at 60).
    per_tick_60(c, 0);
    c.add_s(1, 1, 0);
    c.swc1(1, this.wrapping_add(0x1024));
    c.cycles += 2;
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Tick rate helpers for the lens particles.

/// `60 / rate` when the game does not run at 60 Hz (the share of a 60 Hz
/// frame one tick is), `None` at 60.
#[inline]
fn frame_share(c: &mut Cpu) -> Option<FpuF32> {
    let hz = crate::tick::rate(c);
    if hz == 60 {
        None
    } else {
        Some(FpuF32::from_f32(60.0 / hz as f32))
    }
}

/// `f{r} *= q` when not at 60 (an amount per 60 Hz frame as the amount for
/// this tick).
#[inline]
fn per_frame(c: &mut Cpu, r: usize, q: Option<FpuF32>) {
    if let Some(q) = q {
        c.f[r] = (FpuF32(c.f[r]) * q).0;
    }
}

/// `void cSplashManager_AddSplashFromPoint(cSplashManager* this, const
/// Vec4* point, float speed)` (0x2f4260): snow thrown up at `point` lands on
/// the lens: when the point is within `gp+0x1180` (falloff `1 - d /
/// gp+0x1180` positive), the lens takes snow (`+0x1028` above `gp+0x1188`)
/// and the speed (`speed * gp-0x38f8`) is above `gp+0x1184`, the pending
/// splash amount `+0x1024` grows by `falloff * clamp01((speed' - lo) / (120
/// - lo)) * gp+0x118c`.
///
/// Tick rate: called every tick while the board sprays, so the growth is per
/// 60 Hz frame: above 60 it is scaled by `60/rate` (the decay in
/// `cSplashManager_Update` is converted too). Nothing changes at 60.
pub fn c_splash_manager_add_splash_from_point(c: &mut Cpu) {
    c.cycles += 27;
    let this = abi::arg(c, 0);
    let point = abi::arg(c, 1);
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
    let s = sp(c);
    let g = gp(c);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 5);
    // d = |camera - point|.
    c.lqc2(3, this.wrapping_add(splash::LAST_CAMERA));
    c.lqc2(4, point);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s.wrapping_add(0x10));
    c.mtc1(ZERO, 3);
    c.lq(V0, s.wrapping_add(0x10));
    c.lwc1(1, g.wrapping_add(0x1180));
    c.sq(V0, s);
    c.lqc2(3, s);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(V1, 22);
    c.mtc1(V1, 0);
    c.div_s(0, 0, 1);
    c.sub_s(4, 5, 0);
    c.c_lt_s(3, 4);
    let near = c.fcc();
    c.lwc1(0, g.wrapping_add(0x1188));
    'out: {
        if !near {
            break 'out;
        }
        c.cycles += 5;
        c.lwc1(1, this.wrapping_add(0x1028));
        c.c_lt_s(0, 1);
        let snowing = c.fcc();
        c.lwc1(2, g.wrapping_add(0x1184));
        if !snowing {
            break 'out;
        }
        // The speed ramp.
        c.cycles += 11;
        c.lwc1(0, g.wrapping_sub(0x38f8));
        c.mul_s(0, 12, 0);
        c.g[AT][0] = 0x42f0_0000; // 120.0
        c.mtc1(AT, 1);
        c.sub_s(1, 1, 2);
        c.sub_s(0, 0, 2);
        c.div_s(0, 0, 1);
        c.c_le_s(3, 0);
        let positive = c.fcc();
        c.mov_s(6, 3);
        if positive {
            c.cycles += 1;
            c.min_s(6, 0, 5);
        }
        c.cycles += 6;
        c.mul_s(0, 4, 6);
        c.lwc1(2, g.wrapping_add(0x118c));
        c.lwc1(1, this.wrapping_add(0x1024));
        c.mul_s(0, 0, 2);
        let q = frame_share(c);
        per_frame(c, 0, q);
        c.add_s(1, 1, 0);
        c.swc1(1, this.wrapping_add(0x1024));
    }
    c.cycles += 2;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
    abi::ret(c);
}

/// The drop's unit direction from the screen centre (`gp+0x24c8`,
/// `gp+0x24cc`) to its position `(f4, f5)` into `sp+0`/`sp+4` (the offset
/// itself when it sits on the centre); `f6` = 0, `f7` = 1.
fn centre_direction_drop(c: &mut Cpu, s: u32, g: u32) {
    c.lwc1(0, g.wrapping_add(0x24c8));
    c.lwc1(1, g.wrapping_add(0x24cc));
    c.sub_s(4, 4, 0);
    c.sub_s(5, 5, 1);
    c.mul_s(0, 4, 4);
    c.swc1(4, s.wrapping_add(0x10));
    c.mul_s(1, 5, 5);
    c.swc1(5, s.wrapping_add(0x14));
    c.add_s(0, 0, 1);
    c.sqrt_s(0, 0);
    c.c_eq_s(0, 6);
    if c.fcc() {
        c.swc1(4, s);
        c.cycles += 1;
        c.swc1(5, s.wrapping_add(4));
    } else {
        c.cycles += 6;
        c.div_s(0, 7, 0);
        c.mul_s(1, 5, 0);
        c.mul_s(0, 4, 0);
        c.swc1(1, s.wrapping_add(4));
        c.swc1(0, s);
    }
}

/// `f1 = clamp01((speed - gp+0x1198) / (120 - gp+0x1198))` (0 when not
/// positive); leaves `f7 = 1`, `f8 = 0`.
fn speed_ramp(c: &mut Cpu, g: u32) {
    c.cycles += 13;
    c.lwc1(0, g.wrapping_add(0x1198));
    c.g[AT][0] = 0x42f0_0000; // 120.0
    c.mtc1(AT, 1);
    c.sub_s(2, 12, 0);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 7);
    c.sub_s(1, 1, 0);
    c.mtc1(ZERO, 8);
    c.div_s(2, 2, 1);
    c.c_le_s(8, 2);
    let positive = c.fcc();
    c.mov_s(1, 8);
    if positive {
        c.cycles += 1;
        c.min_s(1, 2, 7);
    }
}

/// The outward push, the growth and the speed cap shared by drops and
/// crystals (0x2f2964.. / 0x2f31e0..): velocity at `vel` (`+0`, `+4`),
/// sizes at `size` (`+0`, `+4`), push random range at `gp+push`, growth
/// range at `gp+grow`, cap at `gp+cap`. Each increment is scaled by `q`.
fn push_grow_cap(c: &mut Cpu, s: u32, g: u32, vel: u32, size: u32, push: u32, grow: u32, cap: u32, q: Option<FpuF32>) {
    c.cycles += 77;
    c.g[V0][0] = Addr(g.wrapping_add(0xa0c)).lw(c);
    c.g[A1][0] = 0x1_8fcd;
    c.g[V1][0] = 0xe9_507c;
    c.mult(0, V0, V0, A1);
    c.g[A0][0] = 0x7f_ffff;
    c.g[A2][0] = 0x3f80_0000;
    c.lwc1(2, s);
    c.lwc1(0, s.wrapping_add(4));
    c.g[T0][0] = sx(vel);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.mul_s(2, 2, 1);
    c.g[V0][0] &= c.g[A0][0];
    c.mul_s(0, 0, 1);
    c.g[V0][0] |= c.g[A2][0];
    c.lwc1(4, g.wrapping_add(push));
    Addr(g.wrapping_add(0xa0c)).sw(c, c.g[V0][0]);
    c.mult(0, T1, V0, A1);
    c.lwc1(3, g.wrapping_add(push + 4));
    c.lwc1(1, g.wrapping_add(0xa0c));
    c.sub_s(3, 3, 4);
    c.lwc1(6, vel);
    c.sub_s(1, 1, 7);
    c.lwc1(5, vel.wrapping_add(4));
    c.swc1(2, s.wrapping_add(0x20));
    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.swc1(0, s.wrapping_add(0x24));
    c.g[V0][0] &= c.g[A0][0];
    c.mul_s(3, 3, 1);
    c.g[V0][0] |= c.g[A2][0];
    c.add_s(4, 4, 3);
    c.mul_s(0, 0, 4);
    c.mul_s(2, 2, 4);
    // Tick rate: the push is a velocity change per 60 Hz frame.
    per_frame(c, 0, q);
    per_frame(c, 2, q);
    c.add_s(5, 5, 0);
    c.swc1(0, s.wrapping_add(0x14));
    c.add_s(6, 6, 2);
    c.swc1(2, s.wrapping_add(0x10));
    c.swc1(5, vel.wrapping_add(4));
    c.mul_s(4, 5, 5);
    c.swc1(6, vel);
    c.mul_s(1, 6, 6);
    Addr(g.wrapping_add(0xa0c)).sw(c, c.g[V0][0]);
    // Growth of both sizes.
    c.lwc1(3, g.wrapping_add(grow));
    c.lwc1(2, g.wrapping_add(0xa0c));
    c.add_s(1, 1, 4);
    c.lwc1(0, g.wrapping_add(grow + 4));
    c.sqrt_s(4, 1);
    c.sub_s(2, 2, 7);
    c.lwc1(1, size);
    c.sub_s(0, 0, 3);
    c.mul_s(0, 0, 2);
    c.add_s(3, 3, 0);
    per_frame(c, 3, q);
    c.add_s(1, 1, 3);
    c.swc1(1, size);
    c.g[V0][0] = Addr(g.wrapping_add(0xa0c)).lw(c);
    c.lwc1(3, g.wrapping_add(grow));
    c.mult(0, T1, V0, A1);
    c.lwc1(2, g.wrapping_add(grow + 4));
    c.sub_s(2, 2, 3);
    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] &= c.g[A0][0];
    c.g[V0][0] |= c.g[A2][0];
    Addr(g.wrapping_add(0xa0c)).sw(c, c.g[V0][0]);
    c.lwc1(0, g.wrapping_add(0xa0c));
    c.lwc1(1, size.wrapping_add(4));
    c.sub_s(0, 0, 7);
    c.mul_s(2, 2, 0);
    c.add_s(3, 3, 2);
    per_frame(c, 3, q);
    c.add_s(1, 1, 3);
    c.swc1(1, size.wrapping_add(4));
    // Speed cap (per 60 Hz frame; the velocity stays in those units).
    c.lwc1(2, g.wrapping_add(cap));
    c.c_lt_s(2, 4);
    if c.fcc() {
        c.cycles += 4;
        c.c_eq_s(4, 8);
        let still = c.fcc();
        c.g[V0][0] = c.g[T0][0];
        if still {
            c.cycles += 3;
            c.lwc1(0, r32(c, V0).wrapping_add(4));
            c.swc1(6, s.wrapping_add(0x30));
            c.swc1(0, s.wrapping_add(0x34));
        } else {
            c.cycles += 6;
            c.div_s(0, 7, 4);
            c.mul_s(1, 5, 0);
            c.mul_s(0, 6, 0);
            c.swc1(1, s.wrapping_add(0x34));
            c.swc1(0, s.wrapping_add(0x30));
        }
        c.cycles += 11;
        c.lwc1(0, s.wrapping_add(0x30));
        c.lwc1(1, s.wrapping_add(0x34));
        c.mul_s(0, 0, 2);
        c.mul_s(1, 1, 2);
        c.swc1(0, s.wrapping_add(0x10));
        c.swc1(1, s.wrapping_add(0x14));
        c.ldl(3, s.wrapping_add(0x17));
        c.ldr(3, s.wrapping_add(0x10));
        let v = r32(c, V0);
        c.sdl(3, v.wrapping_add(7));
        c.sdr(3, v);
    }
}

/// The crystal's screen test (0x2f3438): the position in `(f{x}, f{y})`,
/// the sizes at `size`; dead when either axis is fully off the 640x480
/// screen. Returns whether it is off screen, after charging the tests made.
fn off_screen(c: &mut Cpu, x: usize, y: usize, size: u32) -> bool {
    c.cycles += 7;
    c.lwc1(1, size);
    c.mtc1(ZERO, 2);
    c.add_s(0, x, 1);
    c.c_le_s(0, 2);
    if c.fcc() {
        return true;
    }
    c.cycles += 7;
    c.sub_s(1, x, 1);
    c.g[AT][0] = 0x4420_0000; // 640.0
    c.mtc1(AT, 0);
    c.c_le_s(0, 1);
    if c.fcc() {
        return true;
    }
    c.cycles += 6;
    c.lwc1(1, size.wrapping_add(4));
    c.add_s(0, y, 1);
    c.c_le_s(0, 2);
    if c.fcc() {
        return true;
    }
    c.cycles += 6;
    c.sub_s(1, y, 1);
    c.g[AT][0] = 0x43f0_0000; // 480.0
    c.mtc1(AT, 0);
    c.c_le_s(0, 1);
    if c.fcc() {
        c.cycles += 1;
        return true;
    }
    false
}

/// `void cSplashManager_UpdateDrop(Drop* d, float speed)` (0x2f2810): one
/// tick of a lens water drop (`+0` alive, `+4`/`+8` position, `+0xc`/`+0x10`
/// size, `+0x14`/`+0x18` velocity, `+0x1c` age, `+0x20` life). The age
/// advances by the step; the velocity takes a random kick (`gp+0x11b0..b4`
/// in x, `gp+0x11b8..bc` in y) and, at camera speed, a random push away from
/// the screen centre; the sizes grow at random (`gp+0x11c0..c4`); the speed
/// is capped at `gp+0x11d0`; the position moves by the velocity times a
/// random `lerp(gp+0x11d4, gp+0x11d8, rand)`. The drop dies at the end of
/// its life or off screen.
///
/// Tick rate: the velocity is in pixels per 60 Hz frame. Above 60 every
/// per-call increment (the kicks, the push, the growth) and the position
/// step are scaled by `60/rate`, so a drop moves, grows and falls in the
/// same wall time (the random draws stay per call: their mean effect per
/// second is the same, the jitter a little smoother). The age uses its
/// `.lit4` 1/60, which follows the rate. Nothing changes at 60.
pub fn c_splash_manager_update_drop(c: &mut Cpu) {
    let q = frame_share(c);
    let g = gp(c);
    c.cycles += 64;
    c.g[V0][0] = Addr(g.wrapping_add(0xa0c)).lw(c);
    c.g[A1][0] = 0x1_8fcd;
    c.g[A3][0] = c.g[A0][0];
    let d = r32(c, A3);
    c.mult(0, V0, V0, A1);
    c.lwc1(0, d.wrapping_add(0x1c));
    c.lwc1(1, g.wrapping_sub(0x3918));
    c.g[V1][0] = 0xe9_507c;
    c.g[A0][0] = 0x7f_ffff;
    c.add_s(0, 0, 1);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A2][0] = 0x3f80_0000;
    c.g[V0][0] &= c.g[A0][0];
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 7);
    c.swc1(0, d.wrapping_add(0x1c));
    c.g[V0][0] |= c.g[A2][0];
    Addr(g.wrapping_add(0xa0c)).sw(c, c.g[V0][0]);
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
    let s = sp(c);
    // Random kick in x.
    c.lwc1(3, g.wrapping_add(0x11b0));
    c.lwc1(1, g.wrapping_add(0xa0c));
    c.lwc1(0, g.wrapping_add(0x11b4));
    c.sub_s(1, 1, 7);
    c.lwc1(2, d.wrapping_add(0x14));
    c.sub_s(0, 0, 3);
    c.lwc1(4, d.wrapping_add(0x4));
    c.lwc1(5, d.wrapping_add(0x8));
    c.mtc1(ZERO, 6);
    c.mul_s(0, 0, 1);
    c.add_s(3, 3, 0);
    per_frame(c, 3, q);
    c.add_s(2, 2, 3);
    c.swc1(2, d.wrapping_add(0x14));
    // Random kick in y.
    c.g[V0][0] = Addr(g.wrapping_add(0xa0c)).lw(c);
    c.lwc1(3, g.wrapping_add(0x11b8));
    c.mult(0, T0, V0, A1);
    c.lwc1(2, g.wrapping_add(0x11bc));
    c.sub_s(2, 2, 3);
    c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] &= c.g[A0][0];
    c.g[V0][0] |= c.g[A2][0];
    Addr(g.wrapping_add(0xa0c)).sw(c, c.g[V0][0]);
    c.lwc1(0, g.wrapping_add(0xa0c));
    c.lwc1(1, d.wrapping_add(0x18));
    c.sub_s(0, 0, 7);
    c.mul_s(2, 2, 0);
    c.add_s(3, 3, 2);
    per_frame(c, 3, q);
    c.add_s(1, 1, 3);
    c.swc1(1, d.wrapping_add(0x18));
    centre_direction_drop(c, s, g);

    speed_ramp(c, g);
    push_grow_cap(c, s, g, d.wrapping_add(0x14), d.wrapping_add(0xc), 0x11c8, 0x11c0, 0x11d0, q);
    c.g[V0][0] = Addr(g.wrapping_add(0xa0c)).lw(c);

    // Move by velocity * lerp(lo, hi, rand) on each axis.
    c.cycles += 49;
    c.g[V1][0] = 0x1_8fcd;
    c.g[A0][0] = 0xe9_507c;
    c.mult(0, V0, V0, V1);
    c.g[A1][0] = 0x7f_ffff;
    c.g[A2][0] = 0x3f80_0000;
    c.lwc1(2, d.wrapping_add(0x14));
    c.lwc1(6, g.wrapping_add(0x11d4));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.lwc1(3, g.wrapping_add(0x11d8));
    c.g[V0][0] &= c.g[A1][0];
    c.mul_s(5, 2, 6);
    c.g[V0][0] |= c.g[A2][0];
    c.mul_s(2, 2, 3);
    c.mult(0, T0, V0, V1);
    Addr(g.wrapping_add(0xa0c)).sw(c, c.g[V0][0]);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 4);
    c.lwc1(1, g.wrapping_add(0xa0c));
    c.sub_s(2, 2, 5);
    c.lwc1(0, d.wrapping_add(0x18));
    c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.sub_s(1, 1, 4);
    c.g[V1][0] &= c.g[A1][0];
    c.mul_s(3, 0, 3);
    c.g[V1][0] |= c.g[A2][0];
    c.mul_s(0, 0, 6);
    Addr(g.wrapping_add(0xa0c)).sw(c, c.g[V1][0]);
    c.mul_s(2, 2, 1);
    c.lwc1(7, d.wrapping_add(0x4));
    c.lwc1(1, g.wrapping_add(0xa0c));
    c.sub_s(3, 3, 0);
    c.add_s(5, 5, 2);
    // Tick rate: the step is the velocity per 60 Hz frame.
    per_frame(c, 5, q);
    c.lwc1(8, d.wrapping_add(0x8));
    c.sub_s(1, 1, 4);
    c.lwc1(6, d.wrapping_add(0x1c));
    c.lwc1(4, d.wrapping_add(0x20));
    c.swc1(5, s.wrapping_add(0x10));
    c.mul_s(3, 3, 1);
    c.add_s(1, 7, 5);
    c.c_le_s(4, 6);
    c.add_s(0, 0, 3);
    per_frame(c, 0, q);
    c.swc1(1, d.wrapping_add(0x4));
    c.add_s(8, 8, 0);
    c.swc1(0, s.wrapping_add(0x14));
    let expired = c.fcc();
    c.swc1(8, d.wrapping_add(0x8));
    // Dead at the end of its life or off screen (x in f1, y in f8).
    let dead = if expired {
        c.cycles += 1;
        true
    } else {
        c.lwc1(7, d.wrapping_add(0xc));
        drop_off_screen(c, d)
    };
    if dead {
        Addr(d).sw(c, 0);
    }
    c.cycles += 2;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
    abi::ret(c);
}

/// The drop's screen test (0x2f2b9c): x in `f1`, y in `f8`, sizes `f7` (x)
/// and `d+0x10` (y).
fn drop_off_screen(c: &mut Cpu, d: u32) -> bool {
    c.cycles += 7;
    c.mtc1(ZERO, 2);
    c.add_s(0, 1, 7);
    c.c_le_s(0, 2);
    if c.fcc() {
        return true;
    }
    c.cycles += 7;
    c.sub_s(1, 1, 7);
    c.g[AT][0] = 0x4420_0000; // 640.0
    c.mtc1(AT, 0);
    c.c_le_s(0, 1);
    if c.fcc() {
        return true;
    }
    c.cycles += 6;
    c.lwc1(1, d.wrapping_add(0x10));
    c.add_s(0, 8, 1);
    c.c_le_s(0, 2);
    if c.fcc() {
        return true;
    }
    c.cycles += 6;
    c.sub_s(1, 8, 1);
    c.g[AT][0] = 0x43f0_0000; // 480.0
    c.mtc1(AT, 0);
    c.c_le_s(0, 1);
    if c.fcc() {
        c.cycles += 1;
        return true;
    }
    false
}

/// `void cSplashManager_UpdateCrystal(Crystal* k, float speed)` (0x2f3030):
/// one tick of a lens ice crystal (`+4` alive, `+8`/`+0xc` position,
/// `+0x10`/`+0x14` size, `+0x18`/`+0x1c` velocity, `+0x20` age, `+0x24`
/// life, `+0x40` time to the next random acceleration, `+0x44`/`+0x48` that
/// acceleration). The age advances and the timer runs down by the step; when
/// it runs out a new timer (`gp+0x120c..10`) and acceleration
/// (`gp+0x11fc..1208`) are drawn. The velocity takes the acceleration, the
/// push away from the screen centre at camera speed (`gp+0x121c..20`), the
/// sizes grow (`gp+0x1214..18`), the speed is capped at `gp+0x1224`, the
/// position moves by the velocity. The crystal dies at the end of its life
/// or off screen.
///
/// Tick rate: as the drops, every per-call increment and the position step
/// are scaled by `60/rate` above 60 (the timer and age use the `.lit4`
/// 1/60, which follows the rate, so the re-draws happen in wall time).
/// Nothing changes at 60.
pub fn c_splash_manager_update_crystal(c: &mut Cpu) {
    let q = frame_share(c);
    let g = gp(c);
    c.cycles += 12;
    c.g[A3][0] = c.g[A0][0];
    let k = r32(c, A3);
    c.lwc1(0, g.wrapping_sub(0x390c));
    c.lwc1(1, k.wrapping_add(0x40));
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
    let s = sp(c);
    c.lwc1(2, k.wrapping_add(0x20));
    c.sub_s(1, 1, 0);
    c.mtc1(ZERO, 5);
    c.add_s(2, 2, 0);
    c.c_lt_s(1, 5);
    c.swc1(1, k.wrapping_add(0x40));
    let redraw = c.fcc();
    c.swc1(2, k.wrapping_add(0x20));
    if redraw {
        // A new timer and acceleration.
        c.cycles += 49;
        c.g[V0][0] = Addr(g.wrapping_add(0xa0c)).lw(c);
        c.g[V1][0] = 0x1_8fcd;
        c.g[A0][0] = 0xe9_507c;
        c.mult(0, V0, V0, V1);
        c.g[A1][0] = 0x7f_ffff;
        c.g[A2][0] = 0x3f80_0000;
        c.lwc1(2, g.wrapping_add(0x120c));
        c.lwc1(1, g.wrapping_add(0x1210));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 3);
        c.g[V0][0] &= c.g[A1][0];
        c.sub_s(1, 1, 2);
        c.g[V0][0] |= c.g[A2][0];
        Addr(g.wrapping_add(0xa0c)).sw(c, c.g[V0][0]);
        c.mult(0, T0, V0, V1);
        c.lwc1(0, g.wrapping_add(0xa0c));
        c.sub_s(0, 0, 3);
        c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] &= c.g[A1][0];
        c.g[V0][0] |= c.g[A2][0];
        c.mul_s(1, 1, 0);
        c.mult(0, T0, V0, V1);
        c.add_s(2, 2, 1);
        c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] &= c.g[A1][0];
        c.swc1(2, k.wrapping_add(0x40));
        c.g[V1][0] |= c.g[A2][0];
        Addr(g.wrapping_add(0xa0c)).sw(c, c.g[V0][0]);
        c.lwc1(2, g.wrapping_add(0x11fc));
        c.lwc1(1, g.wrapping_add(0xa0c));
        c.lwc1(0, g.wrapping_add(0x1200));
        c.sub_s(1, 1, 3);
        c.sub_s(0, 0, 2);
        c.mul_s(0, 0, 1);
        c.add_s(2, 2, 0);
        c.swc1(2, k.wrapping_add(0x44));
        Addr(g.wrapping_add(0xa0c)).sw(c, c.g[V1][0]);
        c.lwc1(2, g.wrapping_add(0x1204));
        c.lwc1(1, g.wrapping_add(0x1208));
        c.lwc1(0, g.wrapping_add(0xa0c));
        c.sub_s(1, 1, 2);
        c.sub_s(0, 0, 3);
        c.mul_s(1, 1, 0);
        c.add_s(2, 2, 1);
        c.swc1(2, k.wrapping_add(0x48));
    }

    // velocity += acceleration; the direction from the screen centre.
    c.cycles += 24;
    c.lwc1(0, k.wrapping_add(0x18));
    c.lwc1(1, k.wrapping_add(0x1c));
    c.lwc1(2, k.wrapping_add(0x44));
    c.lwc1(3, k.wrapping_add(0x48));
    // Tick rate: the acceleration is per 60 Hz frame.
    per_frame(c, 2, q);
    per_frame(c, 3, q);
    c.add_s(0, 0, 2);
    c.lwc1(4, k.wrapping_add(0x8));
    c.add_s(1, 1, 3);
    c.lwc1(2, k.wrapping_add(0xc));
    c.swc1(0, k.wrapping_add(0x18));
    c.swc1(1, k.wrapping_add(0x1c));
    c.lwc1(0, g.wrapping_add(0x24c8));
    c.lwc1(1, g.wrapping_add(0x24cc));
    c.sub_s(4, 4, 0);
    c.sub_s(2, 2, 1);
    c.mul_s(0, 4, 4);
    c.swc1(4, s.wrapping_add(0x10));
    c.mul_s(1, 2, 2);
    c.swc1(2, s.wrapping_add(0x14));
    c.add_s(0, 0, 1);
    c.sqrt_s(1, 0);
    c.c_eq_s(1, 5);
    if c.fcc() {
        c.swc1(4, s);
        c.cycles += 1;
        c.swc1(2, s.wrapping_add(4));
    } else {
        c.cycles += 8;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.div_s(0, 0, 1);
        c.mul_s(1, 2, 0);
        c.mul_s(0, 4, 0);
        c.swc1(1, s.wrapping_add(4));
        c.swc1(0, s);
    }

    speed_ramp(c, g);
    push_grow_cap(c, s, g, k.wrapping_add(0x18), k.wrapping_add(0x10), 0x121c, 0x1214, 0x1224, q);
    c.lwc1(3, k.wrapping_add(0x8));

    // position += velocity.
    c.cycles += 12;
    c.lwc1(0, k.wrapping_add(0x18));
    per_frame(c, 0, q);
    c.lwc1(4, k.wrapping_add(0xc));
    c.add_s(3, 3, 0);
    c.lwc1(1, k.wrapping_add(0x1c));
    per_frame(c, 1, q);
    c.lwc1(2, k.wrapping_add(0x20));
    c.lwc1(0, k.wrapping_add(0x24));
    c.add_s(4, 4, 1);
    c.swc1(3, k.wrapping_add(0x8));
    c.c_le_s(0, 2);
    let expired = c.fcc();
    c.swc1(4, k.wrapping_add(0xc));
    let dead = if expired {
        c.cycles += 1;
        true
    } else {
        off_screen(c, 3, 4, k.wrapping_add(0x10))
    };
    if dead {
        Addr(k).off(4).sw(c, 0);
    }
    c.cycles += 2;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
    abi::ret(c);
}


/// `cSplashManager` fields used by the update.
mod splash {
    /// Index of the view (camera) the lens effect belongs to.
    pub const VIEW: u32 = 0x10;
    /// Live drops (`+0x1c`, 0x3c bytes each).
    pub const DROP_COUNT: u32 = 0x14;
    pub const DROPS: u32 = 0x1c;
    pub const DROP_SIZE: u32 = 0x3c;
    /// Live crystals (`+0x724`, 0x4c bytes each: `+0` drips so far, `+4`
    /// alive, `+8` position, `+0x10`/`+0x14` size).
    pub const CRYSTAL_COUNT: u32 = 0x18;
    pub const CRYSTALS: u32 = 0x724;
    pub const CRYSTAL_SIZE: u32 = 0x4c;
    /// Whether `LAST_CAMERA` holds a position yet.
    pub const HAVE_CAMERA: u32 = 0x100c;
    /// The camera position of the previous tick (a quadword).
    pub const LAST_CAMERA: u32 = 0x1010;
    /// The camera speed the drops and crystals react to.
    pub const SPEED: u32 = 0x1020;
}

/// Direct callees of the splash manager update.
mod callee {
    use super::{ee, Callee};
    pub const EMIT_PENDING: Callee = Callee { addr: 0x002f_3810, f: ee::s09::cSplashManager_EmitPending };
    pub const UPDATE_DROP: Callee = Callee { addr: 0x002f_2810, f: ee::s09::cSplashManager_UpdateDrop };
    pub const UPDATE_CRYSTAL: Callee = Callee { addr: 0x002f_3030, f: ee::s09::cSplashManager_UpdateCrystal };
    pub const ADD_DROP: Callee = Callee { addr: 0x002f_37a8, f: ee::s09::cSplashManager_AddDrop };
    /// `Rand_NextMain` (called through the function table).
    pub const RAND_NEXT_MAIN: u32 = 0x0031_77f0;
}

#[inline(always)]
fn gp(c: &Cpu) -> u32 {
    c.g[GP][0] as u32
}

/// `void cSplashManager_Update(cSplashManager *this)` (0x2f39e0): the lens
/// splash effect of one view. Measures the camera speed from its
/// displacement since the last tick (a jump of more than 1000 resets the
/// effect through the virtual at `+0x70`), emits the pending snow splats,
/// updates the drops (swap-removing dead ones), rolls the chance that one
/// random crystal drips (adds a drop, at most `gp+0x117c` per crystal), and
/// updates the crystals (swap-removing dead ones).
///
/// Tick rate: the speed is the displacement per tick times 60 (`.lit4`
/// 59.99999, per second): above 60 it is scaled by `rate/60`, so the speed
/// in units per second (and the > 1000 jump test) stays the same. The drip
/// chance is rolled once per 60 Hz frame of this manager
/// ([`crate::tick::frame60_due`]). Both are unchanged at 60.
pub fn c_splash_manager_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0xd0);
        c.g[V0][0] = Addr(gp(c).wrapping_add(0x115c)).lw(c);
        frame.save(c, S1, 0xa0);
        frame.save(c, S0, 0xb0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x90);
        frame.save(c, S3, 0x80);
        frame.save(c, S4, 0x70);
        frame.save(c, S5, 0x60);
        frame.save(c, S6, 0x50);
        frame.save(c, S7, 0x40);
        frame.save_ra(c, 0x30);
        frame.save_fpr(c, 20, 0xc0);
        if c.g[V0][0] != 0 {
            splash_update_body(c)?;
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xb0);
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore(c, S6, 0x50);
        frame.restore(c, S7, 0x40);
        frame.restore_ra(c, 0x30);
        frame.restore_fpr(c, 20, 0xc0);
        frame.close(c);
        Ok(())
    })
}

/// The update once the effect is enabled (`Ok` = go on to the epilogue).
fn splash_update_body(c: &mut Cpu) -> Flow {
    use splash::*;
    let this = r32(c, S1);
    // The view's camera: world->+0x84->+0x84 holds the camera list.
    c.cycles += 8;
    c.g[V0][0] = Addr(gp(c).wrapping_sub(0x848)).lw(c);
    c.g[A1][0] = Addr(this).off(VIEW).lw(c);
    c.g[V1][0] = Addr(r32(c, V0)).off(0x84).lw(c);
    c.g[A0][0] = Addr(r32(c, V1)).off(0x84).lw(c);
    c.g[V0][0] = Addr(r32(c, A0)).off(0x10).lw(c);
    c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
    let valid = c.g[V0][0] != 0;
    c.g[V1][0] = sx(r32(c, A1) << 2);
    if !valid {
        return Ok(());
    }

    // Camera speed from the displacement since the last tick.
    c.cycles += 6;
    c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
    c.g[A1][0] = Addr(this).off(HAVE_CAMERA).lw(c);
    c.g[A0][0] = Addr(r32(c, V1)).off(4).lw(c);
    c.lq(V0, r32(c, A0).wrapping_add(0x20));
    c.sq(V0, sp(c));
    if c.g[A1][0] == 0 {
        c.cycles += 3;
        c.g[V0][0] = 1;
        Addr(this).off(SPEED).sw(c, 0);
        Addr(this).off(HAVE_CAMERA).sw(c, c.g[V0][0]);
    } else {
        c.cycles += 22;
        c.lqc2(3, sp(c));
        c.lqc2(4, this.wrapping_add(LAST_CAMERA));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp(c).wrapping_add(0x20));
        c.lwc1(2, gp(c).wrapping_sub(0x3904));
        let hz = crate::tick::rate(c);
        if hz != 60 {
            // Per tick -> per second at the game's rate.
            c.f[2] = (FpuF32(c.f[2]) * FpuF32::from_f32(hz as f32 / 60.0)).0;
        }
        c.lq(V0, sp(c).wrapping_add(0x20));
        c.lwc1(1, gp(c).wrapping_sub(0x3900));
        c.sq(V0, sp(c).wrapping_add(0x10));
        c.lqc2(3, sp(c).wrapping_add(0x10));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(3, 22);
        c.mtc1(3, 0);
        c.mul_s(0, 0, 2);
        c.mul_s(0, 0, 1);
        c.swc1(0, this.wrapping_add(SPEED));
    }

    // A jump (teleport, camera cut): reset the effect.
    c.cycles += 7;
    c.lwc1(1, this.wrapping_add(SPEED));
    c.g[AT][0] = 0x447a_0000; // 1000.0
    c.mtc1(AT, 0);
    c.c_lt_s(0, 1);
    let jumped = c.fcc();
    c.lq(V0, sp(c));
    if jumped {
        c.cycles += 7;
        c.g[V0][0] = Addr(this).off(0xc).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x70).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x74).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x002f_3ae4)?;
        Addr(this).off(SPEED).sw(c, 0);
        c.lq(V0, sp(c));
    }

    // Emit the pending snow splats, then update the drops.
    c.cycles += 7;
    c.g[A0][0] = c.g[S1][0];
    c.g[S3][0] = 0;
    c.sq(V0, this.wrapping_add(LAST_CAMERA));
    callee::EMIT_PENDING.call(c, 0x002f_3afc)?;
    c.g[V0][0] = Addr(this).off(DROP_COUNT).lw(c);
    if (c.g[V0][0] as i64) > 0 {
        c.cycles += 4;
        c.g[S5][0] = DROP_SIZE as u64;
        c.g[S2][0] = sx(this.wrapping_add(DROPS));
        c.g[S4][0] = 0;
        c.lwc1(12, this.wrapping_add(SPEED));
        loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S2][0];
            c.g[S0][0] = c.g[S4][0];
            callee::UPDATE_DROP.call(c, 0x002f_3b24)?;
            c.g[V0][0] = Addr(r32(c, S2)).lw(c);
            if c.g[V0][0] == 0 {
                // Dead: move the last drop into this slot.
                c.g[V0][0] = Addr(this).off(DROP_COUNT).lw(c);
                c.cycles += 36;
                c.g[A0][0] = sx(r32(c, S4).wrapping_add(this));
                c.mult(0, 3, 2, 21);
                c.g[V0][0] = sx(r32(c, V1).wrapping_add(this));
                let (src, dst) = (r32(c, V0), r32(c, A0));
                c.ldl(5, src.wrapping_sub(0x19));
                c.ldr(5, src.wrapping_sub(0x20));
                c.ldl(6, src.wrapping_sub(0x11));
                c.ldr(6, src.wrapping_sub(0x18));
                c.ldl(7, src.wrapping_sub(0x9));
                c.ldr(7, src.wrapping_sub(0x10));
                c.ldl(8, src.wrapping_sub(0x1));
                c.ldr(8, src.wrapping_sub(0x8));
                c.sdl(5, dst.wrapping_add(0x23));
                c.sdr(5, dst.wrapping_add(0x1c));
                c.sdl(6, dst.wrapping_add(0x2b));
                c.sdr(6, dst.wrapping_add(0x24));
                c.sdl(7, dst.wrapping_add(0x33));
                c.sdr(7, dst.wrapping_add(0x2c));
                c.sdl(8, dst.wrapping_add(0x3b));
                c.sdr(8, dst.wrapping_add(0x34));
                c.ldl(5, src.wrapping_add(0x7));
                c.ldr(5, src);
                c.ldl(6, src.wrapping_add(0xf));
                c.ldr(6, src.wrapping_add(0x8));
                c.ldl(7, src.wrapping_add(0x17));
                c.ldr(7, src.wrapping_add(0x10));
                c.g[T0][0] = Addr(src).off(0x18).lw(c);
                c.sdl(5, dst.wrapping_add(0x43));
                c.sdr(5, dst.wrapping_add(0x3c));
                c.sdl(6, dst.wrapping_add(0x4b));
                c.sdr(6, dst.wrapping_add(0x44));
                c.sdl(7, dst.wrapping_add(0x53));
                c.sdr(7, dst.wrapping_add(0x4c));
                Addr(dst).off(0x54).sw(c, c.g[T0][0]);
                c.g[V1][0] = Addr(this).off(DROP_COUNT).lw(c);
                c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
                Addr(this).off(DROP_COUNT).sw(c, c.g[V1][0]);
            } else {
                c.cycles += 4;
                c.g[S4][0] = sx(r32(c, S0).wrapping_add(DROP_SIZE));
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(DROP_SIZE));
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
            }
            c.cycles += 4;
            c.g[V0][0] = Addr(this).off(DROP_COUNT).lw(c);
            c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.lwc1(12, this.wrapping_add(SPEED));
            if event_due(c) {
                deliver_event_here(c, 0x002f_3b18);
            }
        }
    }

    // Does a crystal drip this tick? chance = rand * count / gp+0x1168 <
    // gp+0x1170; then a random crystal.
    c.cycles += 23;
    c.g[S6][0] = u64::MAX;
    if crate::tick::frame60_due(c, this.wrapping_add(CRYSTALS), 0) {
        call_ptr(c, callee::RAND_NEXT_MAIN, 0x002f_3be8)?;
        c.g[V1][0] = 0x7f_ffff;
        c.g[A0][0] = 0x3f80_0000;
        c.g[A1][0] = Addr(this).off(CRYSTAL_COUNT).lw(c);
        c.g[V0][0] &= c.g[V1][0];
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.g[V0][0] |= c.g[A0][0];
        c.mtc1(A1, 2);
        c.cvt_s_w(2, 2);
        c.mtc1(V0, 1);
        c.lwc1(3, gp(c).wrapping_add(0x1170));
        c.sub_s(0, 1, 0);
        c.lwc1(1, gp(c).wrapping_add(0x1168));
        c.cvt_s_w(1, 1);
        c.mul_s(0, 0, 2);
        c.div_s(0, 0, 1);
        c.c_lt_s(0, 3);
        if c.fcc() {
            c.cycles += 2;
            if (c.g[A1][0] as i64) > 0 {
                c.cycles += 6;
                call_ptr(c, callee::RAND_NEXT_MAIN, 0x002f_3c4c)?;
                c.g[V1][0] = Addr(this).off(CRYSTAL_COUNT).lw(c);
                c.divu(0, 2, 3);
                c.cycles += 3;
                c.g[A0][0] = c.hi[0];
                c.g[S6][0] = c.hi[0];
            }
        }
    }
    c.g[V0][0] = Addr(this).off(CRYSTAL_COUNT).lw(c);

    // Update the crystals.
    c.cycles += 2;
    c.g[S3][0] = 0;
    if (c.g[V0][0] as i64) <= 0 {
        return Ok(());
    }
    c.cycles += 6;
    c.lwc1(20, gp(c).wrapping_sub(0x38fc));
    c.g[S7][0] = CRYSTAL_SIZE as u64;
    c.g[S4][0] = sx(this.wrapping_add(CRYSTALS + 8));
    c.g[S2][0] = sx(this.wrapping_add(CRYSTALS));
    c.g[S5][0] = 0;
    c.lwc1(12, this.wrapping_add(SPEED));
    loop {
        c.cycles += 6;
        c.g[A0][0] = c.g[S2][0];
        c.g[S0][0] = c.g[S5][0];
        callee::UPDATE_CRYSTAL.call(c, 0x002f_3c94)?;
        c.g[V0][0] = Addr(r32(c, S2)).off(4).lw(c);
        if c.g[V0][0] == 0 {
            c.g[V1][0] = Addr(this).off(CRYSTAL_COUNT).lw(c);
            remove_crystal(c, this);
        } else {
            c.cycles += 2;
            if c.g[S6][0] != c.g[S3][0] {
                c.g[S4][0] = sx(r32(c, S4).wrapping_add(CRYSTAL_SIZE));
            } else {
                crystal_drip(c, this)?;
                c.cycles += 1;
                c.g[S4][0] = sx(r32(c, S4).wrapping_add(CRYSTAL_SIZE));
            }
            c.cycles += 4;
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(CRYSTAL_SIZE));
            c.g[S5][0] = sx(r32(c, S5).wrapping_add(CRYSTAL_SIZE));
            c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
        }
        c.cycles += 4;
        c.g[V0][0] = Addr(this).off(CRYSTAL_COUNT).lw(c);
        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            return Ok(());
        }
        c.lwc1(12, this.wrapping_add(SPEED));
        if event_due(c) {
            deliver_event_here(c, 0x002f_3c88);
        }
    }
}

/// The chosen crystal drips: while under the cap, count the drip and, when
/// both its scaled sizes exceed `gp+0x11a4`, add a drop at its position.
fn crystal_drip(c: &mut Cpu, this: u32) -> Flow {
    c.cycles += 7;
    c.g[V1][0] = sx(this.wrapping_add(splash::CRYSTALS));
    c.g[V0][0] = Addr(gp(c).wrapping_add(0x117c)).lw(c);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, S0)));
    c.g[A0][0] = Addr(r32(c, V1)).lw(c);
    let under_cap = (c.g[A0][0] as i64) < (c.g[V0][0] as i64);
    c.g[V0][0] = sx(r32(c, A0).wrapping_add(1));
    if !under_cap {
        return Ok(());
    }
    c.cycles += 11;
    c.g[A2][0] = sx(sp(c).wrapping_add(0x10));
    Addr(r32(c, V1)).sw(c, c.g[V0][0]);
    let crystal = r32(c, S2);
    c.lwc1(0, crystal.wrapping_add(0x10));
    c.lwc1(1, crystal.wrapping_add(0x14));
    c.mul_s(0, 0, 20);
    c.lwc1(2, gp(c).wrapping_add(0x11a4));
    c.mul_s(1, 1, 20);
    c.c_lt_s(2, 0);
    c.swc1(0, sp(c).wrapping_add(0x10));
    let wide = c.fcc();
    c.swc1(1, sp(c).wrapping_add(0x14));
    if !wide {
        return Ok(());
    }
    c.cycles += 4;
    c.c_lt_s(2, 1);
    let tall = c.fcc();
    c.g[A0][0] = c.g[S1][0];
    if !tall {
        return Ok(());
    }
    c.cycles += 2;
    c.g[A1][0] = c.g[S4][0];
    callee::ADD_DROP.call(c, 0x002f_3d08)
}

/// Move the last crystal into the dead one's slot (`s5` = its offset, `v1` =
/// the count): 0x40 bytes in 32-byte steps (doubleword copies when both are
/// 8-aligned, else unaligned pairs), then the last 12 bytes.
fn remove_crystal(c: &mut Cpu, this: u32) {
    c.cycles += 9;
    c.g[V0][0] = sx(r32(c, S5).wrapping_add(this));
    c.g[A0][0] = sx(r32(c, V0).wrapping_add(splash::CRYSTALS));
    c.mult(0, 2, 3, 23);
    c.g[A1][0] = sx(r32(c, V0).wrapping_add(this));
    c.g[V1][0] = sx(r32(c, A1).wrapping_add(0x6d8));
    c.g[V0][0] = c.g[V1][0] | c.g[A0][0];
    c.g[V0][0] &= 7;
    let aligned = c.g[V0][0] == 0;
    c.g[V0][0] = sx(r32(c, A1).wrapping_add(0x718));
    if aligned {
        loop {
            c.cycles += 11;
            let (src, dst) = (r32(c, V1), r32(c, A0));
            c.g[A1][0] = c.ld(src);
            c.g[A2][0] = c.ld(src.wrapping_add(0x8));
            c.g[A3][0] = c.ld(src.wrapping_add(0x10));
            c.g[T0][0] = c.ld(src.wrapping_add(0x18));
            c.sd(dst, c.g[A1][0]);
            c.sd(dst.wrapping_add(0x8), c.g[A2][0]);
            c.sd(dst.wrapping_add(0x10), c.g[A3][0]);
            c.sd(dst.wrapping_add(0x18), c.g[T0][0]);
            c.g[V1][0] = sx(src.wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = sx(dst.wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002f_3d94);
            }
        }
    } else {
        loop {
            c.cycles += 19;
            let (src, dst) = (r32(c, V1), r32(c, A0));
            c.ldl(5, src.wrapping_add(0x7));
            c.ldr(5, src);
            c.ldl(6, src.wrapping_add(0xf));
            c.ldr(6, src.wrapping_add(0x8));
            c.ldl(7, src.wrapping_add(0x17));
            c.ldr(7, src.wrapping_add(0x10));
            c.ldl(8, src.wrapping_add(0x1f));
            c.ldr(8, src.wrapping_add(0x18));
            c.sdl(5, dst.wrapping_add(0x7));
            c.sdr(5, dst);
            c.sdl(6, dst.wrapping_add(0xf));
            c.sdr(6, dst.wrapping_add(0x8));
            c.sdl(7, dst.wrapping_add(0x17));
            c.sdr(7, dst.wrapping_add(0x10));
            c.sdl(8, dst.wrapping_add(0x1f));
            c.sdr(8, dst.wrapping_add(0x18));
            c.g[V1][0] = sx(src.wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = sx(dst.wrapping_add(0x20));
            if !more {
                c.cycles += 2;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002f_3d40);
            }
        }
    }
    c.cycles += 9;
    let (src, dst) = (r32(c, V1), r32(c, A0));
    c.ldl(2, src.wrapping_add(0x7));
    c.ldr(2, src);
    c.g[A1][0] = Addr(src).off(8).lw(c);
    c.sdl(2, dst.wrapping_add(0x7));
    c.sdr(2, dst);
    Addr(dst).off(8).sw(c, c.g[A1][0]);
    c.g[V0][0] = Addr(this).off(splash::CRYSTAL_COUNT).lw(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
    Addr(this).off(splash::CRYSTAL_COUNT).sw(c, c.g[V0][0]);
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
    use crate::difftest::elf_image;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;

    fn f(x: f32) -> u32 {
        x.to_bits()
    }

    /// `Rand_NextMain`'s state (`0x500000 - 0xfe8`, six words).
    const RAND_STATE: u32 = 0x004f_f018;

    /// A splash manager at `DATA` with its view's camera, `drops` drops and
    /// `crystals` crystals; the camera moved by `step` since the last tick.
    fn splash_fixture(c: &mut Cpu, r: &mut Rng, seed: u32, step: f32, drops: u32, crystals: u32) {
        let this = DATA;
        let (world, g, list, cam) = (DATA + 0x8000, DATA + 0x8100, DATA + 0x8200, DATA + 0x8300);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
        c.bus.write32(world + 0x84, g);
        c.bus.write32(g + 0x84, list);
        c.bus.write32(list + 0x10, 2);
        c.bus.write32(list + 4, cam);
        c.bus.write32(list + 8, cam + 0x40);
        let view = if seed % 9 == 8 { 2 + seed % 3 } else { seed % 2 };
        c.bus.write32(this + 0x10, view);
        for k in [cam, cam + 0x40] {
            for i in 0..4 {
                c.bus.write32(k + 0x20 + 4 * i, f(10.0 * (i + 1) as f32 + (seed % 13) as f32));
            }
        }
        let cp = if view == 1 { cam + 0x40 } else { cam };
        for i in 0..4 {
            let v = f32::from_bits(c.bus.read32(cp + 0x20 + 4 * i));
            c.bus.write32(this + 0x1010 + 4 * i, f(v - if i == 0 { step } else { 0.0 }));
        }
        let vt = vtable(c, r, &[0x70]);
        c.bus.write32(this + 0xc, vt);
        c.bus.write32(this + 0x1024, f((seed % 3) as f32 * 0.75));
        c.bus.write32(this + 0x14, drops);
        for i in 0..drops {
            let d = this + 0x1c + 0x3c * i;
            for k in 1..15 {
                c.bus.write32(d + 4 * k, r.f32(8.0));
            }
            c.bus.write32(d, ((seed + i) % 3 != 0) as u32);
        }
        c.bus.write32(this + 0x18, crystals);
        for j in 0..crystals {
            let k = this + 0x724 + 0x4c * j;
            for w in 2..19 {
                c.bus.write32(k + 4 * w, r.f32(4.0));
            }
            c.bus.write32(k, (seed + j) % 3);
            c.bus.write32(k + 4, ((seed + j) % 4 != 0) as u32);
            c.bus.write32(k + 0x10, f((r.next_u32() % 20) as f32));
            c.bus.write32(k + 0x14, f((r.next_u32() % 20) as f32));
        }
    }


    #[test]
    fn c_splash_manager_reset_matches() {
        check(ssx3_ee::s09::f_002f39c8, c_splash_manager_reset, 100, &|c, seed| {
            let mut r = Rng(0x1011 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x1028, r.next_u32());
        });
    }


    #[test]
    fn c_splash_manager_set_snow_amount_matches() {
        check(ssx3_ee::s09::f_002f4330, c_splash_manager_set_snow_amount, 300, &|c, seed| {
            let mut r = Rng(0x1012 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.f[12] = match seed % 5 {
                0 => 0,
                1 => 0x7f7f_ffff,
                2 => 0x8000_0000,
                3 => r.edge_f32(),
                _ => r.f32(1000.0),
            };
            c.bus.write32(GAME_GP + 0x1190, r.edge_f32());
            c.bus.write32(GAME_GP + 0x1194, r.edge_f32());
            c.bus.write32(GAME_GP - 0x38f4, r.edge_f32());
            c.bus.write32(DATA + 0x1020, r.edge_f32());
            c.bus.write32(DATA + 0x1024, r.edge_f32());
        });
    }


    #[test]
    fn c_splash_manager_update_matches() {
        check(ssx3_ee::s09::f_002f39e0__generated, c_splash_manager_update, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x2f39e0, 1);
            elf_image(c);
            if seed % 11 == 10 {
                c.bus.write32(GAME_GP + 0x115c, 0);
            }
            for i in 0..6 {
                c.bus.write32(RAND_STATE + 4 * i, r.next_u32());
            }
            c.bus.write32(DATA + 0x100c, (seed % 7 != 0) as u32);
            let step = match seed % 5 {
                0 => 900.0,
                1 => 0.0,
                _ => (seed % 17) as f32 * 0.25,
            };
            let drops = seed % 5;
            let crystals = (seed / 5) % 4;
            // (Crystal slots alternate between 8-aligned and not: both
            // copy loops of the swap-remove run.)
            splash_fixture(c, &mut r, seed, step, drops, crystals);
            c.g[A0][0] = DATA as u64;
        });
    }


    /// The lens snow accumulates per 60 Hz tick: one call at 60 and
    /// `rate/60` calls at 120 / 240 Hz add the same amount.
    #[test]
    fn c_splash_manager_set_snow_amount_follows_the_rate() {
        fn after(hz: u32) -> f32 {
            let mut c = crate::difftest::fresh();
            let c = &mut *c;
            c.g[GP][0] = GAME_GP as u64;
            crate::tick::set_for_test(c, hz);
            c.bus.write32(GAME_GP + 0x1190, f(1.5));
            c.bus.write32(GAME_GP + 0x1194, f(0.015));
            c.bus.write32(GAME_GP - 0x38f4, 0x3bfc_0fc1); // 1/130
            c.bus.write32(DATA + 0x1020, f(200.0));
            c.bus.write32(DATA + 0x1024, f(0.25));
            for _ in 0..hz / 60 {
                c.g[A0][0] = DATA as u64;
                c.f[12] = f(3.0);
                c_splash_manager_set_snow_amount(c);
            }
            f32::from_bits(c.bus.read32(DATA + 0x1024))
        }
        let base = after(60);
        assert!(base > 0.3);
        for hz in [120, 240] {
            let v = after(hz);
            assert!((v - base).abs() < 1e-5, "{hz} Hz: {v} vs {base} at 60");
        }
    }


    /// Above 60 Hz the camera speed is per second at the game's rate (the
    /// same camera motion per second gives the same speed), and the drip
    /// chance is rolled once per 60 Hz frame (one `Rand_NextMain` call per
    /// roll with no crystals).
    #[test]
    fn c_splash_manager_update_follows_the_rate() {
        /// (speed, rand calls) after `rate/60` updates with the camera moving
        /// 6 units per 60 Hz frame.
        fn after(hz: u32) -> (f32, u32) {
            let mut c = crate::difftest::fresh();
            let c = &mut *c;
            let mut r = machine(c, 4, 0x2f39e1, 0);
            elf_image(c);
            crate::tick::set_for_test(c, hz);
            let step = 6.0 * 60.0 / hz as f32;
            splash_fixture(c, &mut r, 0, step, 0, 0);
            c.bus.write32(DATA + 0x1024, 0);
            c.bus.write32(DATA + 0x100c, 1);
            let calls0 = c.bus.read32(RAND_STATE + 0x14);
            let cam = DATA + 0x8300;
            for _ in 0..hz / 60 {
                c.g[A0][0] = DATA as u64;
                c_splash_manager_update(c);
                assert_eq!(c.pc, crate::difftest::RA);
                // The camera moves on.
                let x = f32::from_bits(c.bus.read32(cam + 0x20));
                c.bus.write32(cam + 0x20, f(x + step));
            }
            let speed = f32::from_bits(c.bus.read32(DATA + 0x1020));
            (speed, c.bus.read32(RAND_STATE + 0x14).wrapping_sub(calls0))
        }
        let (s60, n60) = after(60);
        assert!(s60 > 1.0, "speed {s60}");
        assert_eq!(n60, 1);
        for hz in [120, 240] {
            let (s, n) = after(hz);
            assert!((s - s60).abs() < 1e-3 * s60, "{hz} Hz: speed {s} vs {s60} at 60");
            assert_eq!(n, 1, "{hz} Hz: {n} drip rolls in one 60 Hz frame");
        }
    }


    /// The screen centre the drops and crystals are pushed away from.
    fn centre(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0x24c8, f(320.0));
        c.bus.write32(GAME_GP + 0x24cc, f(240.0));
    }

    /// A float in `[lo, hi)`.
    fn uf(r: &mut Rng, lo: f32, hi: f32) -> u32 {
        f(lo + (r.next_u32() as f32 / u32::MAX as f32) * (hi - lo))
    }


    #[test]
    fn c_splash_manager_add_splash_from_point_matches() {
        check(ssx3_ee::s09::f_002f4260__generated, c_splash_manager_add_splash_from_point, 300, &|c, seed| {
            let mut r = Rng(0x2f4260 ^ seed as u64);
            scramble(c, &mut r);
            elf_image(c);
            c.g[GP][0] = GAME_GP as u64;
            let this = DATA;
            let point = DATA + 0x2000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = point as u64;
            let reach = f32::from_bits(c.bus.read32(GAME_GP + 0x1180));
            let d = match seed % 4 {
                0 => reach * 2.0,
                _ => reach * (seed % 7) as f32 / 8.0,
            };
            for i in 0..4 {
                c.bus.write32(this + 0x1010 + 4 * i, f(10.0 * i as f32));
                let off = if i == 0 { d } else { 0.0 };
                c.bus.write32(point + 4 * i, f(10.0 * i as f32 + off));
            }
            let thr = f32::from_bits(c.bus.read32(GAME_GP + 0x1188));
            c.bus.write32(this + 0x1028, f(if seed % 5 == 1 { thr - 1.0 } else { thr + 2.0 }));
            c.bus.write32(this + 0x1024, r.f32(3.0));
            // Speeds below, inside and above the ramp.
            c.f[12] = f((seed % 9) as f32 * 600.0);
        });
    }


    /// A lens drop at `DATA` for `cSplashManager_UpdateDrop`.
    fn drop_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let d = DATA;
        c.bus.write32(d, 1);
        let (x, y) = match seed % 8 {
            0 => (320.0, 240.0),
            1 => (-30.0, 100.0),
            2 => (660.0, 100.0),
            3 => (100.0, -30.0),
            4 => (100.0, 500.0),
            _ => (f32::from_bits(uf(r, 0.0, 640.0)), f32::from_bits(uf(r, 0.0, 480.0))),
        };
        c.bus.write32(d + 0x4, f(x));
        c.bus.write32(d + 0x8, f(y));
        c.bus.write32(d + 0xc, uf(r, 1.0, 20.0));
        c.bus.write32(d + 0x10, uf(r, 1.0, 20.0));
        let vmax = if seed % 3 == 0 { 9.0 } else { 3.0 };
        c.bus.write32(d + 0x14, r.f32(vmax));
        c.bus.write32(d + 0x18, r.f32(vmax));
        c.bus.write32(d + 0x1c, uf(r, 0.0, 2.0));
        c.bus.write32(d + 0x20, uf(r, 0.0, 2.0));
        for k in 9..15 {
            c.bus.write32(d + 4 * k, r.next_u32());
        }
    }


    #[test]
    fn c_splash_manager_update_drop_matches() {
        check(ssx3_ee::s09::f_002f2810__generated, c_splash_manager_update_drop, 300, &|c, seed| {
            let mut r = Rng(0x2f2810 ^ seed as u64);
            scramble(c, &mut r);
            elf_image(c);
            c.g[GP][0] = GAME_GP as u64;
            centre(c);
            c.bus.write32(GAME_GP + 0xa0c, r.next_u32());
            drop_fixture(c, &mut r, seed);
            c.g[A0][0] = DATA as u64;
            c.f[12] = f((seed % 7) as f32 * 30.0);
        });
    }


    /// A lens crystal at `DATA` for `cSplashManager_UpdateCrystal`.
    fn crystal_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let k = DATA;
        c.bus.write32(k, seed % 3);
        c.bus.write32(k + 4, 1);
        let (x, y) = match seed % 8 {
            0 => (320.0, 240.0),
            1 => (-30.0, 100.0),
            2 => (660.0, 100.0),
            3 => (100.0, -30.0),
            4 => (100.0, 500.0),
            _ => (f32::from_bits(uf(r, 0.0, 640.0)), f32::from_bits(uf(r, 0.0, 480.0))),
        };
        c.bus.write32(k + 0x8, f(x));
        c.bus.write32(k + 0xc, f(y));
        c.bus.write32(k + 0x10, uf(r, 1.0, 20.0));
        c.bus.write32(k + 0x14, uf(r, 1.0, 20.0));
        let vmax = if seed % 3 == 0 { 9.0 } else { 2.0 };
        c.bus.write32(k + 0x18, r.f32(vmax));
        c.bus.write32(k + 0x1c, r.f32(vmax));
        c.bus.write32(k + 0x20, uf(r, 0.0, 2.0));
        c.bus.write32(k + 0x24, uf(r, 0.0, 2.0));
        for w in 10..16 {
            c.bus.write32(k + 4 * w, r.next_u32());
        }
        // The timer to the next acceleration: running out or not.
        c.bus.write32(k + 0x40, if seed % 4 == 0 { f(0.001) } else { uf(r, 0.1, 2.0) });
        c.bus.write32(k + 0x44, r.f32(0.5));
        c.bus.write32(k + 0x48, r.f32(0.5));
    }


    #[test]
    fn c_splash_manager_update_crystal_matches() {
        check(ssx3_ee::s09::f_002f3030__generated, c_splash_manager_update_crystal, 300, &|c, seed| {
            let mut r = Rng(0x2f3030 ^ seed as u64);
            scramble(c, &mut r);
            elf_image(c);
            c.g[GP][0] = GAME_GP as u64;
            centre(c);
            c.bus.write32(GAME_GP + 0xa0c, r.next_u32());
            crystal_fixture(c, &mut r, seed);
            c.g[A0][0] = DATA as u64;
            c.f[12] = f((seed % 7) as f32 * 30.0);
        });
    }


    /// The lens splash builds per 60 Hz frame: one call at 60 and `rate/60`
    /// calls at 120 / 240 Hz add the same amount.
    #[test]
    fn c_splash_manager_add_splash_from_point_follows_the_rate() {
        fn after(hz: u32) -> f32 {
            let mut c = crate::difftest::fresh();
            let c = &mut *c;
            elf_image(c);
            c.g[GP][0] = GAME_GP as u64;
            crate::tick::set_for_test(c, hz);
            let (this, point) = (DATA, DATA + 0x2000);
            let reach = f32::from_bits(c.bus.read32(GAME_GP + 0x1180));
            let thr = f32::from_bits(c.bus.read32(GAME_GP + 0x1188));
            for i in 0..4 {
                c.bus.write32(this + 0x1010 + 4 * i, 0);
                c.bus.write32(point + 4 * i, if i == 0 { f(reach * 0.25) } else { 0 });
            }
            c.bus.write32(this + 0x1028, f(thr + 2.0));
            c.bus.write32(this + 0x1024, 0);
            for _ in 0..hz / 60 {
                c.g[A0][0] = this as u64;
                c.g[A1][0] = point as u64;
                c.f[12] = f(3500.0);
                c.g[RA][0] = crate::difftest::RA as u64;
                c_splash_manager_add_splash_from_point(c);
            }
            f32::from_bits(c.bus.read32(this + 0x1024))
        }
        let base = after(60);
        assert!(base > 0.0, "{base}");
        for hz in [120, 240] {
            let v = after(hz);
            assert!((v - base).abs() < 1e-5 * base.max(1.0), "{hz} Hz: {v} vs {base} at 60");
        }
    }


    /// (position, size, velocity) of a particle after one second of updates
    /// with fixed random ranges (each range one value), no push (camera at
    /// rest) and the `.lit4` step for the rate at `step_entry`.
    /// `(pos, size, vel)` are the particle's field offsets.
    fn one_second(
        hz: u32,
        update: fn(&mut Cpu),
        (pos, size, vel): (u32, u32, u32),
        step_entry: u32,
        setup: &dyn Fn(&mut Cpu),
    ) -> ([f32; 2], f32, [f32; 2]) {
        let mut c = crate::difftest::fresh();
        let c = &mut *c;
        elf_image(c);
        c.g[GP][0] = GAME_GP as u64;
        centre(c);
        crate::tick::set_for_test(c, hz);
        c.bus.write32(step_entry, crate::tick::step_bits(hz));
        setup(c);
        for _ in 0..hz {
            c.g[A0][0] = DATA as u64;
            c.f[12] = 0;
            c.g[RA][0] = crate::difftest::RA as u64;
            update(c);
            assert_eq!(c.pc, crate::difftest::RA);
        }
        let mut rd = |a: u32| f32::from_bits(c.bus.read32(DATA + a));
        ([rd(pos), rd(pos + 4)], rd(size), [rd(vel), rd(vel + 4)])
    }

    const DROP_FIELDS: (u32, u32, u32) = (0x4, 0xc, 0x14);
    const CRYSTAL_FIELDS: (u32, u32, u32) = (0x8, 0x10, 0x18);

    fn close(a: f32, b: f32, rel: f32) -> bool {
        (a - b).abs() <= rel * a.abs().max(b.abs()).max(1.0)
    }


    /// Drops kick, grow and move per 60 Hz frame: after one second at 120 /
    /// 240 Hz the velocity and size match 60 Hz, the position within the
    /// integration error.
    #[test]
    fn c_splash_manager_update_drop_follows_the_rate() {
        let setup = |c: &mut Cpu| {
            for (a, v) in [(0x11b0, 0.03f32), (0x11b4, 0.03), (0x11b8, 0.02), (0x11bc, 0.02), (0x11c0, 0.05), (0x11c4, 0.05), (0x11d4, 1.0), (0x11d8, 1.0)] {
                c.bus.write32(GAME_GP + a, f(v));
            }
            let d = DATA;
            c.bus.write32(d, 1);
            c.bus.write32(d + 0x4, f(100.0));
            c.bus.write32(d + 0x8, f(100.0));
            c.bus.write32(d + 0xc, f(5.0));
            c.bus.write32(d + 0x10, f(5.0));
            c.bus.write32(d + 0x14, f(1.0));
            c.bus.write32(d + 0x18, f(0.5));
            c.bus.write32(d + 0x1c, 0);
            c.bus.write32(d + 0x20, f(100.0));
        };
        let step = GAME_GP.wrapping_sub(0x3918);
        let (p60, s60, v60) = one_second(60, c_splash_manager_update_drop, DROP_FIELDS, step, &setup);
        assert!(p60[0] > 150.0, "{p60:?}");
        for hz in [120, 240] {
            let (p, s, v) = one_second(hz, c_splash_manager_update_drop, DROP_FIELDS, step, &setup);
            assert!(close(s, s60, 1e-4), "{hz} Hz: size {s} vs {s60}");
            for i in 0..2 {
                assert!(close(v[i], v60[i], 1e-4), "{hz} Hz: velocity {v:?} vs {v60:?}");
                assert!(close(p[i], p60[i], 0.01), "{hz} Hz: position {p:?} vs {p60:?}");
            }
        }
    }


    /// Crystals accelerate, grow and move per 60 Hz frame (as the drops).
    #[test]
    fn c_splash_manager_update_crystal_follows_the_rate() {
        let setup = |c: &mut Cpu| {
            for (a, v) in [(0x1214, 0.05f32), (0x1218, 0.05)] {
                c.bus.write32(GAME_GP + a, f(v));
            }
            let k = DATA;
            c.bus.write32(k + 4, 1);
            c.bus.write32(k + 0x8, f(100.0));
            c.bus.write32(k + 0xc, f(100.0));
            c.bus.write32(k + 0x10, f(5.0));
            c.bus.write32(k + 0x14, f(5.0));
            c.bus.write32(k + 0x18, f(1.0));
            c.bus.write32(k + 0x1c, f(0.5));
            c.bus.write32(k + 0x20, 0);
            c.bus.write32(k + 0x24, f(100.0));
            c.bus.write32(k + 0x40, f(100.0));
            c.bus.write32(k + 0x44, f(0.02));
            c.bus.write32(k + 0x48, f(0.03));
        };
        let step = GAME_GP.wrapping_sub(0x390c);
        let (p60, s60, v60) = one_second(60, c_splash_manager_update_crystal, CRYSTAL_FIELDS, step, &setup);
        assert!(p60[0] > 150.0, "{p60:?}");
        for hz in [120, 240] {
            let (p, s, v) = one_second(hz, c_splash_manager_update_crystal, CRYSTAL_FIELDS, step, &setup);
            assert!(close(s, s60, 1e-4), "{hz} Hz: size {s} vs {s60}");
            for i in 0..2 {
                assert!(close(v[i], v60[i], 1e-4), "{hz} Hz: velocity {v:?} vs {v60:?}");
                assert!(close(p[i], p60[i], 0.01), "{hz} Hz: position {p:?} vs {p60:?}");
            }
        }
    }
}
