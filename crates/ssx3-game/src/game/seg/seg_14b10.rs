//! `src/seg/seg_14b10.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0011_3b10, ai_set_race_state, "AI_SetRaceState")];

// ---------------------------------------------------------------------------
// AI manager: race-state selection.

pub(crate) const AI_SELECT_RACE_STATE: Callee = Callee {
    addr: 0x0011_3b48,
    f: ssx3_ee::s00::f_00113b48,
};

/// `void AI_SetRaceState(AI *ai, int state)` (0x113b10): when the state
/// changes, remember the old one (+4), snapshot the path pointer (+0x98 into
/// +0x9c) and reselect.
pub fn ai_set_race_state(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        let ai = c.g[A0][0];
        c.g[V1][0] = ai;
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr(ai as u32).lw(c);
        if c.g[A1][0] == c.g[A2][0] {
            c.g[RA][0] = c.ld(frame.at(c, 0));
        } else {
            c.cycles += 5;
            c.g[V0][0] = Addr(ai as u32).off(0x98).lw(c);
            Addr(ai as u32).off(0x4).sw(c, c.g[A2][0]);
            Addr(ai as u32).off(0x9c).sw(c, c.g[V0][0]);
            AI_SELECT_RACE_STATE.call(c, 0x0011_3b38)?;
            c.g[RA][0] = c.ld(frame.at(c, 0));
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fill;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::ai::ai::tests::race_state_table;
    pub(crate) use crate::game::lib::kernel::tests::SEEDS;

    #[test]
    fn ai_set_race_state_matches() {
        check(
            ssx3_ee::s00::f_00113b10,
            ai_set_race_state,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 74, 0);
                race_state_table(c, seed);
                let ai = DATA + 0x400;
                let state = (seed % 10) as u32;
                let cur = if seed % 4 == 0 {
                    state
                } else {
                    r.next_u32() % 10
                };
                c.bus.write32(ai, cur);
                c.bus.write32(ai + 0x98, r.next_u32());
                c.g[A0][0] = ai as u64;
                c.g[A1][0] = state as u64;
            },
        );
    }
}
