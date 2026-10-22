//! `src/fe/splashmanager.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002f_39c8, c_splash_manager_reset, "cSplashManager_Reset"),
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
    c.add_s(1, 1, 0);
    c.swc1(1, this.wrapping_add(0x1024));
    c.cycles += 2;
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
}
