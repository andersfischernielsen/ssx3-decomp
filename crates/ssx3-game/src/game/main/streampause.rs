//! `src/main/streampause.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::main::gamemode::sp_2 as sp;
pub(crate) use crate::game::main::gamemode::sw_r;
use crate::guest::Addr;
#[cfg(test)]
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0024_3838, game_module_dtor, "GameModule_dtor"),
    (
        0x0024_2d10,
        generic_stream_pause_create,
        "GenericStreamPause_Create",
    ),
];

// ---------------------------------------------------------------------------
// Intra-shard callees the generator called by name (`jal`), plus the
// cross-shard ones it wrote as `call_indirect` (called with `call_ptr`).

pub(crate) mod callee {

    pub(crate) use crate::game::main::gamemode::callee::BUILTIN_DELETE;
}

/// `void GameModule_dtor(self, flags)` (0x243838): install the module and its
/// embedded object vtables and free the object when `flags & 1`.
pub fn game_module_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x10));
        c.g[V0][0] = 0x48_0000;
        c.g[V1][0] = 0x47_0000;
        c.sd(sp(c), c.g[RA][0]);
        c.g[V0][0] = sx(low(c, V0).wrapping_sub(0x2918));
        c.g[V1][0] = sx(low(c, V1).wrapping_sub(0x2690));
        sw_r(c, A0, 0xbc, c.g[V0][0]);
        c.g[A1][0] = c.g[A1][0] & 0x1;
        sw_r(c, A0, 0, c.g[V1][0]);
        for off in [
            0x1f8u32, 0x1e8, 0x1c4, 0x1a8, 0x198, 0x184, 0x164, 0x154, 0x144, 0x134, 0x124, 0x10c,
            0xfc, 0xec, 0xdc,
        ] {
            sw_r(c, A0, off, c.g[V0][0]);
        }
        if c.g[A1][0] == 0 {
            c.cycles += 3;
        } else {
            c.cycles += 2;
            call_ptr(c, callee::BUILTIN_DELETE.addr, 0x0024_38a4)?;
            c.cycles += 3;
        }
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

/// `operator_new`-style allocator (`EA_MemAllocNamed`, 0x317d70).
pub(crate) const OC437_MEM_ALLOC_NAMED: u32 = 0x0031_7d70;

/// `cStreamPause* GenericStreamPause_Create(void)` (0x242d10): allocates a
/// 4-byte object (`EA_MemAllocNamed(4, name, 0x20000000, 0)`) and stores its
/// vtable pointer in it. Returns the allocation in `v0`.
pub fn generic_stream_pause_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x48_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = 4;
        c.g[A1][0] = sx(0x48_0000u32.wrapping_sub(0x3ae0));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, OC437_MEM_ALLOC_NAMED, 0x0024_2d30)?;
        c.g[V1][0] = 0x48_0000;
        c.g[A0][0] = c.g[V0][0];
        c.g[V1][0] = sx(0x48_0000u32.wrapping_sub(0x2f18));
        frame.restore_ra(c, 0);
        c.sw(c.g[A0][0] as u32, c.g[V1][0]);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;

    use crate::difftest::DATA;

    pub(crate) use crate::game::main::gamemode::tests::be_library;

    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS;

    #[test]
    fn game_module_dtor_matches() {
        check(
            ee::s06::f_00243838__generated,
            game_module_dtor,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x243838, 2);
                be_library(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::oc437_covered;

    #[test]
    fn generic_stream_pause_create_matches() {
        oc437_covered(
            ee::s06::f_00242d10__generated,
            generic_stream_pause_create,
            0x0024_2d10,
            1,
            &|_, _, _| {},
        );
    }
}
