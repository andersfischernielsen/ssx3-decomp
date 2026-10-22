//! `src/visualfx/crowdrender2d.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002d_b9c8, c_crowd_anim2d_ctor, "cCrowdAnim2D_ctor"),
];


// ---------------------------------------------------------------------------
// 2D crowd / fog particles.

/// `cCrowdAnim2D *cCrowdAnim2D_ctor(cCrowdAnim2D *this, cCrowdAnim2DData *data)`
/// (0x2db9c8): initialise the animation record, storing the data pointer at
/// +0x1c and resolving the shared tables.
pub fn c_crowd_anim2d_ctor(c: &mut Cpu) {
    c.cycles += 16;
    let this = abi::arg(c, 0);
    c.g[V0][0] = 0x4;
    Addr(this).off(0x1c).sw(c, c.g[A1][0]);
    Addr(this).off(0x8).sw(c, 0x4);
    c.g[A1][0] = 0x44_0000;
    Addr(this).off(0x4).sw(c, 0);
    c.g[A3][0] = 0x3;
    Addr(this).off(0xc).sw(c, 0);
    c.g[V0][0] = c.g[A0][0];
    c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a74)).lw(c);
    c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x5dd0)).lw(c);
    Addr(this).sw(c, 0);
    Addr(this).off(0x10).sw(c, c.g[V1][0]);
    c.g[A1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x1c)).lw(c);
    Addr(this).off(0x14).sw(c, c.g[A3][0]);
    Addr(this).off(0x18).sw(c, c.g[A1][0]);
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
    fn c_crowd_anim2d_ctor_matches() {
        check(ssx3_ee::s09::f_002db9c8, c_crowd_anim2d_ctor, 200, &|c, seed| {
            let mut r = Rng(0x1004 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            let table = DATA + 0x500;
            c.bus.write32(GAME_GP + 0x2a74, table);
            c.bus.write32(table + 0x1c, r.next_u32());
            c.bus.write32(0x44_5dd0, r.next_u32());
        });
    }
}
