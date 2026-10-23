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
}
