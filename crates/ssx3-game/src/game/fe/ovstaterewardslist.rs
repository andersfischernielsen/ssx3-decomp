//! `src/fe/ovstaterewardslist.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Flow;
use crate::call::Frame;
pub(crate) use crate::game::dirtysock::tagsunk::at_3 as at;
pub(crate) use crate::game::dirtysock::tagsunk::gp_5 as gp;
pub(crate) use crate::game::dirtysock::tagsunk::sw;
pub(crate) use crate::game::fe::feasyncfile::at_2;
pub(crate) use crate::game::main::ssxappdtor::p as p_2;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0020_0218,
        ov_screen_rewards_list_popup_init,
        "OVScreenRewardsListPopup_Init",
    ),
    (0x0020_06b8, event_select_ctor, "EventSelect_ctor"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::BE_GET_INTERFACE_RIDER;
}

/// `EventSelect* EventSelect_ctor(EventSelect* self)` (0x2006b8): when the
/// reward screen is in play, cache its selected index for the widget layer.
pub fn event_select_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        c.g[V0][0] = c.lw(gp(c, -0xb5c));
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = (c.g[V0][0] < 2u64) as u64;
        frame.save_ra(c, 0);
        if c.g[V0][0] == 0 {
            c.cycles += 9;
            call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE, 0x0020_06d4)?;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x0020_06e0)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::BE_GET_INTERFACE_RIDER, 0x0020_06e8)?;
            c.g[V1][0] = c.lw(at(c, V0, 0x54));
            sw(c, gp(c, -0xb4c), 0, c.g[V1][0]);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void OVScreenRewardsListPopup_Init(OVScreenRewardsListPopup* self)`
/// (0x200218): create the rewards-list popup screen and start its thread.
pub fn ov_screen_rewards_list_popup_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x4704_b8;
        c.g[S0][0] = Addr(at_2(c, S1, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x0020_023c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0020_0250)?;
        let t = c.g[V0][0] == 0;
        Addr(at_2(c, S1, 0x40)).sw(c, c.g[V0][0]);
        if !t {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0020_0268)?;
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0020_a380, 0x0020_0270)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared body of `OVScreenTrophy_OnWidgetAction` (0x20e9a0) and
/// `OVScreenRewardsListPopup_OnWidgetAction` (0x200288): the same screen
/// widget-action handler compiled twice. `ret0` is the return address of its
/// first nested call; the others follow at fixed offsets.
pub(crate) fn ov_screen_widget_action(c: &mut Cpu, ret0: u32) -> Flow {
    c.cycles += 7;
    let frame = Frame::open(c, 0x40);
    let widget = c.g[A1][0];
    frame.save(c, 16, 0x30);
    frame.save(c, 17, 0x20);
    c.g[S0][0] = widget;
    frame.save_ra(c, 0x10);
    let no_widget = c.g[S0][0] == 0;
    c.g[S1][0] = c.g[A0][0];
    if !no_widget {
        c.cycles += 3;
        c.g[V0][0] = 0x5;
        let action5 = c.g[A2][0] == c.g[V0][0];
        c.g[V0][0] = 0x6;
        if action5 {
            // a "clicked" action: tell the widget (slot 0x58) the new state
            c.cycles += 16;
            c.g[V1][0] = c.lw(p_2(c.g[S0][0] as u32, 8));
            c.g[A1][0] = c.g[SP][0];
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.g[A0][0] = c.lh(p_2(c.g[V1][0] as u32, 0x58));
            c.swc1(0, p_2(c.g[SP][0] as u32, 4));
            c.swc1(0, c.g[SP][0] as u32);
            c.sw(p_2(c.g[SP][0] as u32, 8), 0);
            c.sw(p_2(c.g[SP][0] as u32, 0xc), 0);
            c.g[V0][0] = c.lw(p_2(c.g[V1][0] as u32, 0x5c));
            let func = c.g[V0][0] as u32;
            c.g[RA][0] = sx(ret0);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, func, ret0)?;
            c.g[V1][0] = c.lw(p_2(c.g[S0][0] as u32, 0x18));
            c.g[V0][0] = 0x1;
            let not_first = c.g[V1][0] != c.g[V0][0];
            c.g[A1][0] = 0;
            if !not_first {
                c.cycles += 10;
                c.g[V0][0] = c.lw(p_2(c.g[S1][0] as u32, 0x10));
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = 0x1;
                c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                c.g[V1][0] = c.lw(p_2(c.g[A3][0] as u32, 4));
                c.g[A0][0] = c.lh(p_2(c.g[V1][0] as u32, 0x20));
                c.g[V0][0] = c.lw(p_2(c.g[V1][0] as u32, 0x24));
                let func = c.g[V0][0] as u32;
                c.g[RA][0] = sx(ret0 + 0x34);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, func, ret0 + 0x34)?;
                c.g[A1][0] = c.g[V0][0];
            }
            c.cycles += 5;
            c.g[A0][0] = c.lw(p_2(c.g[S1][0] as u32, 0x10));
            c.g[RA][0] = sx(ret0 + 0x44);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x0039_f400, ret0 + 0x44)?;
            c.lq(16, p_2(c.g[SP][0] as u32, 0x30));
        } else {
            c.cycles += 2;
            if c.g[A2][0] == c.g[V0][0] {
                // the "second" action: hand the widget's state to the screen
                c.g[V0][0] = c.lw(p_2(c.g[S1][0] as u32, 0x10));
                c.cycles += 10;
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.lw(p_2(c.g[S0][0] as u32, 0x18));
                c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                c.g[V1][0] = c.lw(p_2(c.g[A3][0] as u32, 4));
                c.g[A0][0] = c.lh(p_2(c.g[V1][0] as u32, 0x28));
                c.g[V0][0] = c.lw(p_2(c.g[V1][0] as u32, 0x2c));
                let func = c.g[V0][0] as u32;
                c.g[RA][0] = sx(ret0 + 0x6c);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, func, ret0 + 0x6c)?;
                let unchanged = c.g[V0][0] == 0;
                c.g[A1][0] = c.g[V0][0];
                if !unchanged {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw(p_2(c.g[S1][0] as u32, 0x10));
                    c.g[RA][0] = sx(ret0 + 0x80);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039_f400, ret0 + 0x80)?;
                }
            } else {
                c.cycles += 2;
                c.lq(16, p_2(c.g[SP][0] as u32, 0x30));
            }
        }
    }
    c.cycles += 1;
    c.lq(16, p_2(c.g[SP][0] as u32, 0x30));
    c.cycles += 4;
    c.lq(17, p_2(c.g[SP][0] as u32, 0x20));
    c.g[RA][0] = c.ld(p_2(c.g[SP][0] as u32, 0x10));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = ra;
    Ok(())
}

/// `void func_00200288(OVScreen*, Widget* w, int action)` (0x200288): the
/// rewards-list popup's widget-action handler (the same code).
pub fn ovscreen_rewards_list_popup_on_widget_action(c: &mut Cpu) {
    guest(c, |c| ov_screen_widget_action(c, 0x0020_02f0))
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;

    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::be_library_2 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base;

    pub(crate) use crate::game::fe::feasyncfile::tests::W;
    use ssx3_ee as ee;

    #[test]
    fn event_select_ctor_matches() {
        check(
            ssx3_ee::s05::f_002006b8__generated,
            event_select_ctor,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x2006b8, 0);
                be_library(c);
                c.bus.write32(
                    GAME_GP.wrapping_sub(0xb5c),
                    if seed % 2 == 0 { 0 } else { 2 },
                );
            },
        );
    }

    #[test]
    fn ov_screen_rewards_list_popup_init_matches() {
        check(
            ee::s05::f_00200218__generated,
            ov_screen_rewards_list_popup_init,
            30,
            &|c, seed| {
                alloc_base(c, seed, 0x2002_18);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(W + 0x84, 0);
            },
        );
    }
}
