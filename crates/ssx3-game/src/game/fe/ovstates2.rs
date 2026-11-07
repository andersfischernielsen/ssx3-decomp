//! `src/fe/ovstates2.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::guest;
use crate::call::Frame;
pub(crate) use crate::game::fe::festaterewards2::r;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x001a_06b0, fe_flow_state_free, "FEFlowState_Free")];

// ---------------------------------------------------------------------------
// Callees (the generated name of each address, so a later hand-written
// replacement of a callee is reached through its override slot too).

pub(crate) mod callee {

    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_MAN_FREE;
}

/// `void FEFlowState_Free(FEFlowState* self)` (0x1A06B0): close the state,
/// freeing the buffer it owns at +0x4.
pub fn fe_flow_state_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        let owned = Addr(r(c, S0).wrapping_add(4)).lw(c);
        c.g[A0][0] = owned;
        if owned == 0 {
            Addr(r(c, S0)).sb(c, 0);
        } else {
            c.cycles += 4;
            callee::MEM_MAN_FREE.call(c, 0x001a_06d4)?;
            Addr(r(c, S0).wrapping_add(4)).sw(c, 0);
            Addr(r(c, S0)).sb(c, 0);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
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

    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::fe::festaterewards2::tests::SEEDS;

    #[test]
    fn fe_flow_state_free_matches() {
        check(
            ssx3_ee::s03::f_001a06b0__generated,
            fe_flow_state_free,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1a06_b0, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
                let owned = if seed % 2 == 0 { 0 } else { DATA + 0x800 };
                w(c, DATA + 4, owned);
            },
        );
    }
}
