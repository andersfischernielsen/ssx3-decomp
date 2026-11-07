//! `src/fe/festateload3.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
pub(crate) use crate::game::main::gamestate::gw;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at;
pub(crate) use crate::game::main::gamestate::p_2 as p;
#[cfg(test)]
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
#[cfg(test)]
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0023_4910,
    gamestatepreraceeload_check_intro_script,
    "GameStatePreRaceLoad_CheckIntroScript",
)];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_START;
}

/// `void GameStatePreRaceLoad_CheckIntroScript(self)` (0x234910): start the
/// intro script when the engine is in state 2 and mark `self+0x14`.
pub fn gamestatepreraceeload_check_intro_script(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        let engine = lw_at(c, gw(c, -0x84c));
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = engine;
        c.g[V0][0] = lw_at(c, p(engine as u32, 0x550));
        c.g[V0][0] ^= 2;
        let not_state = c.g[V0][0] != 0;
        c.g[S0][0] = c.g[A0][0];
        if !not_state {
            c.cycles += 5;
            c.g[A0][0] = c.g[V1][0];
            c.g[A1][0] = 1;
            c.g[A2][0] = 1;
            c.g[A3][0] = 1;
            call_ptr(c, callee::SCRIPT_ENGINE_LIST_START.addr, 0x0023_4944)?;
        }
        c.cycles += 5;
        c.g[A0][0] = lw_at(c, gw(c, -0x84c));
        c.g[V0][0] = lw_at(c, p(c.g[A0][0] as u32, 0x550));
        c.g[V0][0] ^= 2;
        if c.g[V0][0] != 0 {
            c.sw(p(r32(c, S0), 0x14), 0);
        } else {
            c.cycles += 5;
            c.g[A1][0] = 0;
            c.g[A2][0] = 1;
            c.g[A3][0] = 1;
            call_ptr(c, callee::SCRIPT_ENGINE_LIST_START.addr, 0x0023_4968)?;
            c.sw(p(r32(c, S0), 0x14), 0);
        }
        c.cycles += 5;
        c.g[V1][0] = 1;
        c.g[V0][0] = lw_at(c, gw(c, -0x84c));
        c.g[V0][0] = lw_at(c, p(c.g[V0][0] as u32, 0x550));
        if c.g[V0][0] == c.g[V1][0] {
            c.sw(p(r32(c, S0), 0x14), c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS;

    #[test]
    fn gamestatepreraceeload_check_intro_script_matches() {
        check(
            ee::s06::f_00234910__generated,
            gamestatepreraceeload_check_intro_script,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0x4910, 0);
                let engine = DATA + 0x800;
                c.bus.write32(GAME_GP - 0x84c, engine);
                c.bus.write32(engine + 0x550, (seed % 3) as u32);
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }
}
