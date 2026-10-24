//! `src/fe/ovstates6.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::gp_3 as gp;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0020_d190,
    ov_screen_pda_template_close,
    "OVScreenPDATemplate_Close",
)];

/// `void OVScreenPDATemplate_Close()` (0x20d190): unlink and delete the PDA
/// template (`gp-0x9e4`), then clear the global.
pub fn ov_screen_pda_template_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(gp(c, -0x9e4)).lw(c);
        let none = c.g[A0][0] == 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 5;
            call_ptr(c, 0x0039_7740, 0x0020_d1a8)?;
            c.g[A2][0] = Addr(gp(c, -0x9e4)).lw(c);
            let gone = c.g[A2][0] == 0;
            c.g[A1][0] = 3;
            if !gone {
                c.cycles += 5;
                let template = abi::r32(c, A2);
                c.g[V0][0] = Addr(template.wrapping_add(8)).lw(c);
                let vtable = c.g[V0][0] as u32;
                c.g[A0][0] = Addr(vtable.wrapping_add(8)).lh(c);
                c.g[V1][0] = Addr(vtable.wrapping_add(0xc)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx(template.wrapping_add(abi::r32(c, A0)));
                call_ptr(c, target, 0x0020_d1c8)?;
            }
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        Addr(gp(c, -0x9e4)).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot;

    #[test]
    fn ov_screen_pda_template_close_matches() {
        check(
            ee::s05::f_0020d190__generated,
            ov_screen_pda_template_close,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x20d1_90, 2);
                let template = if seed % 3 == 0 { 0 } else { DATA + 0x100 };
                c.bus.write32(GAME_GP - 0x9e4, template);
                if template != 0 {
                    let vt = DATA + 0x800;
                    c.bus.write32(template + 8, vt);
                    slot(c, &mut r, vt, 8, FAKE + 0x10);
                }
            },
        );
    }
}
