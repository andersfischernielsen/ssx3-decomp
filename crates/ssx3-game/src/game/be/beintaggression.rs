//! `src/be/beintaggression.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
pub(crate) use crate::game::be::beintmission::lw;
pub(crate) use crate::game::be::beintmission::BE_GET_RIDER_CHARACTER;
pub(crate) use crate::game::be::beintmission::BE_GET_RIDER_PROFILE;
pub(crate) use crate::game::be::beintmission::BUILTIN_DELETE;
pub(crate) use crate::game::be::beintmission::RIDER_STRIDE;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0015_5ab0,
        be_aggression_int_get_rival_state,
        "BEAggressionInt_GetRivalState",
    ),
    (
        0x0015_c7e0,
        be_aggression_int_destroy_instance,
        "BEAggressionInt_DestroyInstance",
    ),
];

/// `void BEAggressionInt_DestroyInstance(void)` (0x15c7e0).
pub fn be_aggression_int_destroy_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1ea4));
        frame.save_ra(c, 0);
        call_ptr(c, BUILTIN_DELETE, 0x0015_c7f4)?;
        frame.restore_ra(c, 0);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1ea4), 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) const NEW_RACE_GET_INSTANCE: Callee = Callee {
    addr: 0x0014_48d8,
    f: ssx3_ee::s01::f_001448d8,
};

pub(crate) const F_00145750: Callee = Callee {
    addr: 0x0014_5750,
    f: ssx3_ee::s01::f_00145750,
};

/// `int BEAggressionInt_GetRivalState(BEAggressionInt *self, int rider,
/// int rival)` (0x155ab0): compare the rival's current race position with the
/// rider's and return 2/… from the aggression table, or 2 when they are equal.
pub fn be_aggression_int_get_rival_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        BE_GET_RIDER_PROFILE.call(c, 0x0015_5ad4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S2][0] = c.g[V0][0];
        BE_GET_RIDER_CHARACTER.call(c, 0x0015_5ae0)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[S0][0] = c.g[V0][0];
        BE_GET_RIDER_CHARACTER.call(c, 0x0015_5aec)?;
        c.g[S1][0] = c.g[V0][0];
        NEW_RACE_GET_INSTANCE.call(c, 0x0015_5af4)?;
        c.g[A0][0] = c.g[V0][0];
        F_00145750.call(c, 0x0015_5afc)?;
        let t = c.g[V0][0] == c.g[S0][0];
        c.g[A0][0] = RIDER_STRIDE as u64;
        if t {
            c.cycles += 1;
            c.g[V0][0] = 0x2;
        } else {
            c.cycles += 12;
            c.g[A1][0] = 0x9b50;
            c.mult(0, A0, S1, A0);
            c.mult(0, A1, S2, A1);
            c.g[V0][0] = sx((c.g[S0][0] as u32) << 1);
            c.g[V1][0] = 0x4a0000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[V1][0] = sx(0x4a0000u32.wrapping_add(0x6ca8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0xbc1));
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::fill;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    #[test]
    fn be_aggression_int_get_rival_state_matches() {
        check(
            ssx3_ee::s01::f_00155ab0,
            be_aggression_int_get_rival_state,
            30,
            &|c, seed| {
                let mut r = machine(c, seed, 0x5ab0, 0);
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = (seed % 8) as u64;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x400);
                fill(c, &mut r, DATA, 0x2000);
            },
        );
    }
}
