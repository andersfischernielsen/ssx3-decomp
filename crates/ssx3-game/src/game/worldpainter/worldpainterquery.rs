//! `src/worldpainter/worldpainterquery.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
pub(crate) use crate::game::fe::festaterewards2::r;
pub(crate) use crate::game::dirtysock::tags2::gp_3 as gp;
pub(crate) use crate::game::fe::festaterewards2::Ops;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002c_03a8, world_painter_query_reset_list, "WorldPainterQuery_ResetList"),
];


// ---------------------------------------------------------------------------
// World painter and string helpers.

/// `void cWorldPainterQuery_reset(cWorldPainterQuery* self)` (0x2C03A8): set
/// every queued query's result word to -99999.0f.
pub fn world_painter_query_reset_list(c: &mut Cpu) {
    c.cycles += 3;
    let head = c.lw_r_2(V1, gp(c, 0x790));
    if head != 0 {
        c.cycles += 3;
        c.ld_f(0, gp(c, -0x428c));
        c.lw_r_2(V0, head as u32);
        loop {
            c.cycles += 7;
            let result = r(c, V0);
            c.st_f(0, result);
            c.lw_r_2(V1, r(c, V1).wrapping_add(0x14));
            if c.g[V1][0] == 0 {
                break;
            }
            c.lw_r_2(V0, r(c, V1));
            if event_due(c) {
                deliver_event_here(c, 0x002c_03c0);
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::put_vec;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::fe::festaterewards2::tests::SEEDS;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::main::gameload::tests::fb;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
 // gp - 0x2a68: 1/30
    pub(crate) const GP_NEG99999: u32 = 0x49ee_64;


    // --------------------------------------------------- painter and strings

    #[test]
    fn world_painter_query_reset_list_matches() {
        check(ssx3_ee::s08::f_002c03a8__generated, world_painter_query_reset_list, SEEDS, &|c, seed| {
            machine(c, seed, 0x2c03_a8, 0);
            belib(c);
            w(c, GP_NEG99999, fb(-99999.0));
            match seed % 3 {
                0 => {}
                1 => {
                    w(c, GAME_GP + 0x790, DATA);
                    w(c, DATA, DATA + 0x1000);
                    w(c, DATA + 0x14, 0);
                }
                _ => {
                    w(c, GAME_GP + 0x790, DATA);
                    w(c, DATA, DATA + 0x1000);
                    w(c, DATA + 0x14, DATA + 0x20);
                    w(c, DATA + 0x20, DATA + 0x1010);
                    w(c, DATA + 0x34, 0);
                }
            }
        });
    }
}
