//! `src/fe/ovstates4.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::festaterewards::p;
pub(crate) use crate::game::fe::festaterewards::r;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x001f_8df8,
    ovscreen_pause_freeride_play_icon_anim,
    "OVScreenPauseFreeride_PlayIconAnim",
)];

// ---------------------------------------------------------------------------
// Pause menu.

/// `void OVScreenPauseFreeride_PlayIconAnim(self, idx, on)` (0x1f8df8): format
/// the icon animation's label, look its frame up on the screen and either
/// start it (remembering the playing frame) or retarget the running
/// animation.
pub fn ovscreen_pause_freeride_play_icon_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save_ra(c, 0x40);
        let on = c.g[A2][0] != 0;
        c.g[S0][0] = c.g[A0][0];

        'outer: {
            if !on {
                c.cycles += 18;
                c.g[V0][0] = 0x440000;
                c.g[V1][0] = sx(r(c, A1) << 2);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(0x1c30));
                c.g[A1][0] = 0x4a0000;
                c.g[V1][0] = sx(p(r(c, V1), r(c, V0)));
                c.g[A0][0] = c.g[SP][0];
                c.g[A2][0] = Addr(r(c, V1)).lw(c);
                c.g[A1][0] = sx(r(c, A1).wrapping_add(0x2490));
                call_ptr(c, 0x0041_7828, 0x001f_8e98)?;
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x0031_7670, 0x001f_8ea0)?;
                c.g[A0][0] = Addr(r(c, S0)).off(0xc0).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_c778, 0x001f_8eac)?;
                c.g[V1][0] = c.g[V0][0];
                c.g[A0][0] = 0xffff;
                if c.g[V1][0] == c.g[A0][0] {
                    frame.restore(c, 16, 0x50);
                    break 'outer;
                }
                c.cycles += 5;
                c.g[V0][0] = Addr(r(c, S0)).off(0xc4).lw(c);
                c.sh(Addr(r(c, V0)).off(0xc).0, c.g[V1][0]);
                c.g[V1][0] = Addr(r(c, S0)).off(0xc4).lw(c);
                c.sh(Addr(r(c, V1)).off(0xe).0, c.g[A0][0]);
                c.sw(Addr(r(c, S0)).off(0xc4).0, 0);
                c.cycles += 1;
                frame.restore(c, 16, 0x50);
                break 'outer;
            }

            c.cycles += 18;
            c.g[V0][0] = 0x440000;
            c.g[V1][0] = sx(r(c, A1) << 2);
            c.g[V0][0] = sx(r(c, V0).wrapping_add(0x1c30));
            c.g[A1][0] = 0x4a0000;
            c.g[V1][0] = sx(p(r(c, V1), r(c, V0)));
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = Addr(r(c, V1)).lw(c);
            c.g[A1][0] = sx(r(c, A1).wrapping_add(0x2488));
            call_ptr(c, 0x0041_7828, 0x001f_8e30)?;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0031_7670, 0x001f_8e38)?;
            c.g[A0][0] = Addr(r(c, S0)).off(0xc0).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_c778, 0x001f_8e44)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = 0xffff;
            if c.g[A1][0] == c.g[V0][0] {
                frame.restore(c, 16, 0x50);
                break 'outer;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(r(c, S0)).off(0xc4).lw(c);
            if c.g[V0][0] != 0 {
                frame.restore(c, 16, 0x50);
                break 'outer;
            }
            c.cycles += 5;
            c.g[A0][0] = Addr(r(c, S0)).off(0xc0).lw(c);
            c.g[A2][0] = 1;
            call_ptr(c, 0x0039_c870, 0x001f_8e6c)?;
            c.sw(Addr(r(c, S0)).off(0xc4).0, c.g[V0][0]);
            c.cycles += 1;
            frame.restore(c, 16, 0x50);
        }

        c.cycles += 3;
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::seed_count;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::fe::festaterewards::tests::base_9 as base;
    pub(crate) use crate::game::fe::festaterewards::tests::hook;
    pub(crate) use crate::game::fe::festaterewards::tests::strict;
    pub(crate) use crate::game::fe::festaterewards::tests::w;
    pub(crate) use crate::game::fe::festaterewards::tests::BE_7 as BE;
    pub(crate) use crate::game::fe::festaterewards::tests::HOOKS;
    pub(crate) use crate::game::fe::festaterewards::tests::KEEP_F0;
    pub(crate) use crate::game::fe::festaterewards::tests::W;

    // ----------------------------------------------------------- pause menu

    #[test]
    fn ovscreen_pause_freeride_play_icon_anim_matches() {
        strict(
            ee::s04::f_001f8df8__generated,
            ovscreen_pause_freeride_play_icon_anim,
            90,
            &|c, seed| {
                let _ = base(c, seed, 0x1f8d, 6);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = ((seed / 4) % 2) as u64;
                w(c, DATA + 0xc0, DATA + 0x800);
                w(c, DATA + 0xc4, if seed % 3 == 0 { 0 } else { DATA + 0x900 });
                w(c, DATA + 0x800 + 0x38, DATA + 0xa00);
                w(c, DATA + 0xa00, 3);
                w(c, DATA + 0x900 + 0x8, 0);
            },
        );
    }
}
