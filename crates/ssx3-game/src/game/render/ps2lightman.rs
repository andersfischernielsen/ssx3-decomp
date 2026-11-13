//! `src/render/ps2lightman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::guest;
use crate::call::Frame;
use crate::guest::Addr;
#[cfg(test)]
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0038_ae10, light_man_ctor, "LightMan_ctor"),
    (0x0038_ae28, light_man_dtor, "LightMan_dtor"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::dirtysock::tagsunk::callee::BUILTIN_DELETE_2 as BUILTIN_DELETE;
}

// ---------------------------------------------------------------------------
// Light manager

/// `LightMan_ctor(LightMan *m)` (0x38ae10): install the vtable and register
/// the manager in the global at `gp + 0x2a9c`.
pub fn light_man_ctor(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = 0x49_0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx(0x49_0000u32.wrapping_add(0x36d8));
    Addr(abi::arg(c, 0)).sw(c, c.g[V1][0]);
    Addr((c.g[GP][0] as u32).wrapping_add(0x2a9c)).sw(c, c.g[A0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Light manager / UI lists.

/// `LightMan::~LightMan()` (0x38ae28): install the light manager vtable, clear
/// the global pointer and (when bit 0 of `flags` is set) delete the object.
pub fn light_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        c.g[V0][0] = 0x0049_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x36d8));
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        let vtable = c.g[V0][0];
        Addr(this).sw(c, vtable);
        c.g[A1][0] &= 1;
        let delete = c.g[A1][0] != 0;
        Addr((c.g[GP][0] as u32).wrapping_add(0x2a9c)).sw(c, 0);
        if delete {
            c.cycles += 2;
            callee::BUILTIN_DELETE.call(c, 0x0038_ae50)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::alloc_machine;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::zero;

    pub(crate) use crate::game::fe::festaterewards::tests::heap_block;

    #[test]
    fn light_man_ctor_matches() {
        check(ssx3_ee::s12::f_0038ae10, light_man_ctor, 200, &|c, seed| {
            let mut r = Rng(0x38ae_1000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = (DATA as u64) | ((r.next_u32() as u64) << 32);
            c.bus.write32(GAME_GP + 0x2a9c, r.next_u32());
        });
    }

    // ------------------------------------------------------ light / UI lists

    #[test]
    fn light_man_dtor_matches() {
        check(
            ee::s12::f_0038ae28__generated,
            light_man_dtor,
            60,
            &|c, seed| {
                let mut r = alloc_machine(c, seed, 0x38ae_28);
                let obj = heap_block(c, 0x140);
                zero(c, obj, 0x140);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx((seed & 3) as u32);
                c.bus.write32(GAME_GP + 0x2a9c, r.next_u32());
            },
        );
    }
}
