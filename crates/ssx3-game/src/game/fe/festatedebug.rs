//! `src/fe/festatedebug.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::feasyncfile::r;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0018_1090,
    fe_state_char_select_ctor,
    "cFEStateCharSelect_ctor",
)];

// ---------------------------------------------------------------------------
// FE state: character select.

/// `cFEStateCharSelect* cFEStateCharSelect_ctor(cFEStateCharSelect* s, void*
/// screen, int count)` (0x181090): install the character-select screen and
/// pre-fill every rider slot it owns.
pub fn fe_state_char_select_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x60);
        'outer: {
            frame.save(c, S1, 0x40);
            let this = r(c, A0);
            c.g[S1][0] = sx(this);
            c.g[A2][0] = sx((r(c, A2) as u32) << 24);
            frame.save(c, S2, 0x30);
            frame.save(c, S0, 0x50);
            c.g[S2][0] = sx((r(c, A2) as i32 >> 24) as u32);
            frame.save(c, S3, 0x20);
            frame.save(c, S4, 0x10);
            c.g[S0][0] = 0;
            frame.save_ra(c, 0);
            c.g[S4][0] = 0xff;
            call_ptr(c, 0x0039_e2a0, 0x0018_10c4)?;

            c.g[V0][0] = sx(0x0046_d000);
            c.g[A0][0] = 9;
            c.g[V1][0] = 1;
            Addr(p(r(c, S1), 8)).sw(c, c.g[V0][0]);
            Addr(p(r(c, S1), 0xc)).sw(c, c.g[A0][0]);
            Addr(p(r(c, S1), 0x54)).sw(c, c.g[V1][0]);
            Addr(p(r(c, S1), 0x44)).sb(c, c.g[S2][0]);
            Addr(p(r(c, S1), 0x48)).sw(c, 0);
            Addr(p(r(c, S1), 0x4c)).sw(c, 0);
            c.g[V0][0] = Addr(p(r(c, S1), 0x10)).lw(c);
            let none = (c.g[S2][0] as i64) <= 0;
            c.g[S3][0] = Addr(r(c, V0)).lw(c);
            if none {
                break 'outer;
            }
            loop {
                c.cycles += 9;
                c.g[A1][0] = sx((r(c, S0) as u32) << 24);
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = sx((r(c, A1) as i32 >> 24) as u32);
                c.g[RA][0] = sx(0x0018_110c);
                c.g[S0][0] = sx(r(c, S0).wrapping_add(1));
                call_ptr(c, 0x001a_1cd0, 0x0018_110c)?;
                c.g[V0][0] = !(0u64 | c.g[V0][0]);
                c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                let more = c.g[V1][0] != 0;
                c.g[S4][0] = c.g[S4][0] & c.g[V0][0];
                if !more {
                    break 'outer;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0018_10f8);
                }
            }
        }
        c.cycles += 10;
        Addr(p(r(c, S1), 0x15)).sb(c, c.g[S4][0]);
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::ee;
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_8 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_6 as W;

    #[test]
    fn fe_state_char_select_ctor_matches() {
        check(
            ee::s02::f_00181090__generated,
            fe_state_char_select_ctor,
            60,
            &|c, seed| {
                base(c, seed, 0x1810, 1);
                c.g[A0][0] = DATA as u64;
                c.g[A2][0] = (seed % 4) as u64;
                w(c, DATA + 0x10, DATA + 0x400);
                w(c, DATA + 0x400, DATA + 0x500);
            },
        );
    }
}
