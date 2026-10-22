//! `src/fe/festatelegal.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi;
use crate::call::sp;
use crate::call::Flow;
pub(crate) use crate::game::fe::fepopup3::p_2 as p;
pub(crate) use crate::game::fe::fepopup3::sw;
pub(crate) use crate::game::fe::fepopup3::gp_2 as gp;
pub(crate) use crate::game::fe::feasyncfile::lw;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0018_7cb8, fe_screen_autosave_popup_update, "FEScreenAutosavePopup_Update"),
    (0x0018_87a0, cfe_state_options_ctor, "cFEStateOptions_ctor"),
];


// ---------------------------------------------------------------------------
// FE state construction / locale tables.

/// `cFEStateOptions_ctor` (0x1887a0): construct the options screen state:
/// the base `UIList` (`0x39e2a0`), vtable `0x46c7c0`, size `0x18`, and, when
/// `reset` is non-null, reset the flow-state selection to entry `0x18`.
pub fn cfe_state_options_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        call_ptr(c, 0x0039_e2a0, 0x0018_87bc)?;
        c.g[V1][0] = 0x0047_0000;
        c.g[V0][0] = 0x18;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3840));
        sw(c, p(c.g[S1][0], 0xc), c.g[V0][0]);
        let reset = c.g[S0][0] == 0;
        sw(c, p(c.g[S1][0], 8), c.g[V1][0]);
        if !reset {
            c.cycles += 4;
            c.g[A0][0] = sx(gp(c, 0x1d90));
            c.g[A1][0] = 0x18;
            c.g[A2][0] = 0;
            call_ptr(c, 0x001a_0708, 0x0018_87e4)?;
        }
        c.cycles += 6;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void FEScreenAutosavePopup_Update(FEScreenAutosavePopup* p)` (0x00187cb8):
/// tick the base state; every 240 ticks look up the autosave screen, start
/// its thread (or report failure) and close the top states.
pub fn fe_screen_autosave_popup_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.sq(S0, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e510, 0x0018_7ccc)?; // UIState_Update
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
        c.g[V1][0] = 0xf0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        let tick = c.g[V0][0] != c.g[V1][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x48), c.g[V0][0]);
        if !tick {
            c.cycles += 10;
            c.g[A0][0] = sx(0x0045_dc60);
            c.g[RA][0] = sx(0x0018_7cec);
            call_ptr(c, 0x0031_7670, 0x0018_7cec)?; // StrHash
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_c778, 0x0018_7cf8)?; // UIScreen_LookupValue
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = 0xffff;
            let missing = c.g[A1][0] == c.g[V0][0];
            c.g[A1][0] = c.g[A1][0] & 0xffff;
            if !missing {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                c.g[A2][0] = 1;
                call_ptr(c, 0x0039_c870, 0x0018_7d14)?; // UIScreen_StartThread
            }
            c.cycles += 4;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            c.g[A1][0] = 1;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x0039_f190, 0x0018_7d24)?; // UIStateMgr_CloseTopStates
        }
        c.cycles += 4;
        c.lq(S0, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        abi::ret(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::check;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fill;
    use ssx3_ee as ee;
    pub(crate) use crate::game::fe::fepopup3::tests::setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_3 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_4 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_6 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_3 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base;


    // ---------------------------------------------------------------- FE core

    #[test]
    fn cfe_state_options_ctor_matches() {
        check(
            ssx3_ee::s02::f_001887a0__generated,
            cfe_state_options_ctor,
            160,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1887a0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = if seed % 3 == 0 {
                    0
                } else {
                    (r.next_u32() | 1) as u64
                };
                c.bus.write32(0x004a_4e84, DATA + 0x800);
            },
        );
    }


    #[test]
    fn fe_screen_autosave_popup_update_matches() {
        check(
            ee::s02::f_00187cb8__generated,
            fe_screen_autosave_popup_update,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x187c, 3);
                let p = DATA;
                w(c, p + 0x48, if seed % 4 == 0 { 0xef } else { r.next_u32() });
                w(c, p + 0x40, if seed % 3 == 0 { 0 } else { DATA + 0x600 });
                w(c, p + 0x10, DATA + 0x700);
                fill(c, &mut r, p + 0x20, 8);
                c.g[A0][0] = sx(p);
            },
        );
    }
}
