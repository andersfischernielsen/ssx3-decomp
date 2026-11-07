//! `src/fe/ovstatelodge2.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
pub(crate) use crate::game::dirtysock::tags2::p;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::festatebonusmaterial::widget_range_notify;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x001f_3790, cfe_state_lodge_init, "cFEStateLodge_Init"),
    (
        0x001f_3cc8,
        fe_state_lodge_on_widget_notify,
        "cFEStateLodge_OnWidgetNotify",
    ),
    (
        0x001f_3ff8,
        c_fe_state_lodge_rider_detail_ctor,
        "cFEStateLodgeRiderDetail_ctor",
    ),
    (
        0x001f_4050,
        fe_state_lodge_rider_detail_init,
        "cFEStateLodgeRiderDetail_Init",
    ),
    (
        0x001f_4380,
        c_fe_state_lodge_rider_detail_on_render,
        "cFEStateLodgeRiderDetail_OnRender",
    ),
    (
        0x001f_43d8,
        fe_state_lodge_rider_detail_on_widget_notify,
        "cFEStateLodgeRiderDetail_OnWidgetNotify",
    ),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {

    pub(crate) use crate::game::dirtysock::tags2::callee::FE_MODULE_GET_PLAYER_PAD_MASK;

    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_3 as UI_STATE_CTOR;

    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE;

    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
}

/// `void* cFEStateLodgeRiderDetail_ctor(self)` (0x1f3ff8): run the UI state
/// ctor, install the class and copy the pad mask byte.
pub fn c_fe_state_lodge_rider_detail_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[RA][0] = sx(0x001f_400c);
        call_ptr(c, callee::UI_STATE_CTOR, 0x001f_400c)?;
        c.g[V0][0] = 0x28;
        c.g[V1][0] = 0x47_0000;
        Addr(p(r32(c, S0), 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x39d8));
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V1][0]);
        c.g[A1][0] = 0;
        Addr(p(r32(c, S0), 0x44)).sb(c, 0);
        c.g[V0][0] = Addr(r32(c, S0)).off(0x10).lw(c);
        c.g[RA][0] = sx(0x001f_4034);
        c.g[A0][0] = Addr(r32(c, V0)).lw(c);
        call_ptr(c, callee::FE_MODULE_GET_PLAYER_PAD_MASK, 0x001f_4034)?;
        Addr(p(r32(c, S0), 0x15)).sb(c, c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateLodgeRiderDetail_OnRender(self)` (0x1f4380): when the detail
/// screen is not open, find its widget by name and reveal it.
pub fn c_fe_state_lodge_rider_detail_on_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S0)).off(0x48).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 8;
            c.g[A0][0] = 0x47_0000;
            c.g[RA][0] = sx(0x001f_43a8);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1090));
            call_ptr(c, 0x0031_7670, 0x001f_43a8)?;
            c.g[A0][0] = Addr(r32(c, S0)).off(0x40).lw(c);
            c.g[RA][0] = sx(0x001f_43b4);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001f_43b4)?;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001f_43c4);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF, 0x001f_43c4)?;
            }
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateLodge_OnWidgetNotify(...)` (0x1F3CC8): ids 7..9 answer 0x100.
pub fn fe_state_lodge_on_widget_notify(c: &mut Cpu) {
    widget_range_notify(c, 7);
}

/// `int cFEStateLodgeRiderDetail_OnWidgetNotify(...)` (0x1F43D8): ids 8..9
/// answer 0x100.
pub fn fe_state_lodge_rider_detail_on_widget_notify(c: &mut Cpu) {
    widget_range_notify(c, 8);
}

/// `void cFEStateLodge_Init(self)` (0x1f3790): create the lodge screen, play
/// its first frame, select the lodge bank and start its music.
pub fn cfe_state_lodge_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        c.g[A0][0] = 0x4a0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2318));
        c.g[S0][0] = Addr(p(this as u32, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x001f_37b4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001f_37c8)?;
        let missing = c.g[V0][0] == 0;
        c.sw(p(c.g[S1][0] as u32, 0x40), c.g[V0][0]);
        if !missing {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_37e0)?;
        }
        c.cycles += 24;
        call_ptr(c, 0x0014_dc80, 0x001f_37e8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x001f_37f4)?;
        c.g[A1][0] = Addr(p(c.g[S1][0] as u32, 0x44)).lb(c);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_74c8, 0x001f_3800)?;
        c.sw(p(c.g[S1][0] as u32, 0x48), c.g[V0][0]);
        c.g[V0][0] = 0x440000;
        c.sw(p(c.g[V0][0] as u32, 0x1b14), 0);
        call_ptr(c, 0x0028_b180, 0x001f_3810)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0028_f140, 0x001f_381c)?;
        c.g[V0][0] = 0x12;
        c.sw(p(c.g[S1][0] as u32, 0x54), 0);
        c.sw(p(c.g[S1][0] as u32, 0x50), c.g[V0][0]);
        c.sw(p(c.g[S1][0] as u32, 0x4c), 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// FE state: lodge rider detail.

/// `void cFEStateLodgeRiderDetail_Init(cFEStateLodgeRiderDetail* s)` (0x1f4050):
/// create the "rider detail" screen from its name hash, play its first frame,
/// then install the screen's rider name and the current peak's description.
pub fn fe_state_lodge_rider_detail_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        'outer: {
            frame.save(c, S1, 0x10);
            let this = r(c, A0);
            c.g[S1][0] = sx(this);
            frame.save_ra(c, 0);
            frame.save(c, S0, 0x20);
            c.g[A0][0] = sx(0x0046_ef20);
            c.g[RA][0] = sx(0x001f_4074);
            c.g[S0][0] = Addr(p(this, 0x10)).lw(c);
            call_ptr(c, callee::FE_STRING_HASH, 0x001f_4074)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001f_4088);
            c.g[A3][0] = 0;
            call_ptr(c, 0x0039_7cc0, 0x001f_4088)?;
            let none = c.g[V0][0] == 0;
            Addr(p(r(c, S1), 0x40)).sw(c, c.g[V0][0]);
            if none {
                break 'outer;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x001f_40a0);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_40a0)?;
            'mid: {
                c.cycles += 20;
                c.g[V0][0] = 1;
                c.g[RA][0] = sx(0x001f_40ac);
                Addr(p(r(c, S1), 0x48)).sw(c, c.g[V0][0]);
                call_ptr(c, 0x0014_dc80, 0x001f_40ac)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001f_40b8);
                c.g[A1][0] = 1;
                call_ptr(c, callee::BE_GET_INTERFACE, 0x001f_40b8)?;
                c.g[A1][0] = Addr(p(r(c, S1), 0x44)).lb(c);
                c.g[RA][0] = sx(0x001f_40c4);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014_7488, 0x001f_40c4)?;
                c.g[RA][0] = sx(0x001f_40cc);
                c.g[S0][0] = c.g[V0][0];
                call_ptr(c, 0x0014_dc80, 0x001f_40cc)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001f_40d8);
                c.g[A1][0] = 0xd;
                call_ptr(c, callee::BE_GET_INTERFACE, 0x001f_40d8)?;
                c.g[A1][0] = Addr(p(r(c, S1), 0x44)).lb(c);
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001f_40e8);
                c.g[A2][0] = c.g[S0][0];
                call_ptr(c, 0x0015_77a0, 0x001f_40e8)?;
                if c.g[V0][0] == 0 {
                    Addr(p(r(c, S1), 0x48)).sw(c, 0);
                    break 'mid;
                }
                break 'mid;
            }
            break 'outer;
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
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

    pub(crate) use crate::game::be::bestate2::tests::belib;
    pub(crate) use crate::game::be::bestate2::tests::w;
    pub(crate) use crate::game::be::bestate2::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerlist::tests::wb;
    pub(crate) use crate::game::dirtysock::tags2::tests::alloc_ready;
    pub(crate) use crate::game::dirtysock::tags2::tests::clear;
    pub(crate) use crate::game::dirtysock::tags2::tests::common;

    pub(crate) use crate::game::dirtysock::tags2::tests::SEEDS_7 as SEEDS;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_4 as alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3_2 as OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ;

    use ssx3_ee as ee;

    #[test]
    fn c_fe_state_lodge_rider_detail_ctor_matches() {
        check(
            ee::s04::f_001f3ff8__generated,
            c_fe_state_lodge_rider_detail_ctor,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x1f3ff8, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x400);
                let module = DATA + 0x2000;
                clear(c, module, 0x40);
                c.bus.write32(this + 0x10, module);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn c_fe_state_lodge_rider_detail_on_render_matches() {
        check(
            ee::s04::f_001f4380__generated,
            c_fe_state_lodge_rider_detail_on_render,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x1f4380, 2);
                let this = DATA + 0x100;
                clear(c, this, 0x200);
                let screen = DATA + 0x2000;
                clear(c, screen, 0x200);
                c.bus
                    .write32(this + 0x48, if seed % 3 == 0 { 1 } else { 0 });
                c.bus.write32(this + 0x40, screen);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn fe_state_lodge_on_widget_notify_matches() {
        check(
            ssx3_ee::s04::f_001f3cc8__generated,
            fe_state_lodge_on_widget_notify,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x001f_3cc8, 0);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A2][0] = (seed % 12) as u64;
            },
        );
    }

    #[test]
    fn fe_state_lodge_rider_detail_on_widget_notify_matches() {
        check(
            ssx3_ee::s04::f_001f43d8__generated,
            fe_state_lodge_rider_detail_on_widget_notify,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x001f_43d8, 0);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A2][0] = (seed % 12) as u64;
            },
        );
    }

    /// A screen-owning state: engine at +0x10, a screen created or not.
    pub(crate) fn screen_init(c: &mut Cpu, seed: u32, salt: u64) {
        let _ = base(c, seed, salt, 2);
        c.g[A0][0] = OBJ as u64;
        w(c, OBJ + 0x10, OBJ2);
        w(c, OBJ + 0x40, 0);
        w(c, OBJ + 0x44, (seed % 4) as u32);
        w(c, OBJ2 + 0x10, OBJ3);
    }

    #[test]
    fn cfe_state_lodge_init_matches() {
        check(
            ee::s04::f_001f3790__generated,
            cfe_state_lodge_init,
            6,
            &|c, seed| screen_init(c, seed, 0x001f_3790),
        );
    }

    #[test]
    fn fe_state_lodge_rider_detail_init_matches() {
        check(
            ee::s04::f_001f4050__generated,
            fe_state_lodge_rider_detail_init,
            40,
            &|c, seed| {
                alloc_base(c, seed, 0x1f40, 5);
                c.g[A0][0] = DATA as u64;
                w(c, DATA + 0x10, DATA + 0x400);
                wb(c, DATA + 0x44, (seed % 4) as u8);
            },
        );
    }
}
