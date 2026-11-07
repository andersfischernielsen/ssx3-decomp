//! `src/main/loadscreens2.cpp`: hand-written replacements (moved from the
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
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0023_2ae0, start_screen_ctor, "StartScreen_ctor")];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_CTOR;
}

// ---------------------------------------------------------------------------
// StartScreen + memory card.

/// `StartScreen* StartScreen_Load(self, arg)` (0x232ae0): the start screen's
/// constructor: `AppModule_ctor`, then the module's id/vtable and a cleared
/// field.
pub fn start_screen_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        callee::APP_MODULE_CTOR.call(c, 0x0023_2afc)?;
        let self_ = abi::r32(c, S0);
        c.g[V1][0] = 0x48_0000;
        Addr(self_.wrapping_add(0x18)).sw(c, c.g[S1][0]);
        c.g[V1][0] = sx(0x48_0000u32.wrapping_sub(0x2830));
        c.g[V0][0] = c.g[S0][0];
        Addr(self_).sw(c, c.g[V1][0]);
        Addr(self_.wrapping_add(0xc)).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
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
    use crate::difftest::GAME_GP;

    #[test]
    fn start_screen_ctor_matches() {
        check(
            ee::s06::f_00232ae0__generated,
            start_screen_ctor,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x232a_e0, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32() & 0xffff);
                c.bus.write32(GAME_GP - 0x51b8, r.next_u32());
            },
        );
    }
}
