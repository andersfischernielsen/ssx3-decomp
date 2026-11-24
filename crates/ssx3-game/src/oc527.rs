//! Staging module oc527 (wave wave231): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc527`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0024_fdc0,
        shadow_menu_on_activate,
        "ShadowMenu_OnActivate",
    ),
    (
        0x0024_fde0,
        shadow_menu_on_deactivate,
        "ShadowMenu_OnDeactivate",
    ),
    (
        0x0024_fea8,
        snow_surface_menu_on_activate,
        "SnowSurfaceMenu_OnActivate",
    ),
    (
        0x0024_fec8,
        snow_surface_menu_on_deactivate,
        "SnowSurfaceMenu_OnDeactivate",
    ),
    (
        0x0024_fee8,
        board_trail_menu_on_activate,
        "BoardTrailMenu_OnActivate",
    ),
    (
        0x0024_ff08,
        board_trail_menu_on_deactivate,
        "BoardTrailMenu_OnDeactivate",
    ),
    (0x0024_ff90, flags_menu_on_activate, "FlagsMenu_OnActivate"),
    (
        0x0024_ffb0,
        flags_menu_on_deactivate,
        "FlagsMenu_OnDeactivate",
    ),
    (
        0x0024_ffd0,
        power_up_fx_toggles_menu_on_activate,
        "PowerUpFXTogglesMenu_OnActivate",
    ),
    (
        0x0024_fff0,
        power_up_fx_toggles_menu_on_deactivate,
        "PowerUpFXTogglesMenu_OnDeactivate",
    ),
    (
        0x0025_0010,
        snowfall_toggles_menu_on_activate,
        "SnowfallTogglesMenu_OnActivate",
    ),
    (
        0x0025_0030,
        snowfall_toggles_menu_on_deactivate,
        "SnowfallTogglesMenu_OnDeactivate",
    ),
    (
        0x0025_0050,
        camera_splash_menu_on_activate,
        "CameraSplashMenu_OnActivate",
    ),
    (
        0x0025_0070,
        camera_splash_menu_on_deactivate,
        "CameraSplashMenu_OnDeactivate",
    ),
    (0x0025_0128, tube_menu_on_activate, "TubeMenu_OnActivate"),
    (
        0x0025_0148,
        tube_menu_on_deactivate,
        "TubeMenu_OnDeactivate",
    ),
    (
        0x0025_0168,
        path_arrow_menu_on_activate,
        "PathArrowMenu_OnActivate",
    ),
    (
        0x0025_0188,
        path_arrow_menu_on_deactivate,
        "PathArrowMenu_OnDeactivate",
    ),
    (
        0x0025_01a8,
        visual_effects_menus_global_ctors,
        "VisualEffectsMenus_GlobalCtors",
    ),
    (
        0x0025_01c8,
        visual_effects_menus_global_dtors,
        "VisualEffectsMenus_GlobalDtors",
    ),
];

/// `VisualEffectsMenus_StaticInitDestroy(int initialize, uint priority)`
/// (0x24e8d8), the static menu table the global ctor and dtor call.
pub(crate) const OC527_VISUAL_EFFECTS_STATIC_INIT_DESTROY: Callee = Callee {
    addr: 0x0024_e8d8,
    f: ee::s06::f_0024e8d8,
};

/// `void ShadowMenu_OnActivate(void)` (0x24fdc0): `MenuSysPage_OnActivate(this)`.
pub fn shadow_menu_on_activate(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x002c_bf60), 0x0024_fdd0)
}

/// `void ShadowMenu_OnDeactivate(void)` (0x24fde0): `MenuSysPage_OnDeactivate(this)`.
pub fn shadow_menu_on_deactivate(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x002c_bfd0), 0x0024_fdf0)
}

/// `void SnowSurfaceMenu_OnActivate(void)` (0x24fea8): `MenuSysPage_OnActivate(this)`.
pub fn snow_surface_menu_on_activate(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x002c_bf60), 0x0024_feb8)
}

/// `void SnowSurfaceMenu_OnDeactivate(void)` (0x24fec8): `MenuSysPage_OnDeactivate(this)`.
pub fn snow_surface_menu_on_deactivate(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x002c_bfd0), 0x0024_fed8)
}

/// `void BoardTrailMenu_OnActivate(void)` (0x24fee8): `MenuSysPage_OnActivate(this)`.
pub fn board_trail_menu_on_activate(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x002c_bf60), 0x0024_fef8)
}

/// `void BoardTrailMenu_OnDeactivate(void)` (0x24ff08): `MenuSysPage_OnDeactivate(this)`.
pub fn board_trail_menu_on_deactivate(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x002c_bfd0), 0x0024_ff18)
}

/// `void FlagsMenu_OnActivate(void)` (0x24ff90): `MenuSysPage_OnActivate(this)`.
pub fn flags_menu_on_activate(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x002c_bf60), 0x0024_ffa0)
}

/// `void FlagsMenu_OnDeactivate(void)` (0x24ffb0): `MenuSysPage_OnDeactivate(this)`.
pub fn flags_menu_on_deactivate(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x002c_bfd0), 0x0024_ffc0)
}

/// `void PowerUpFXTogglesMenu_OnActivate(void)` (0x24ffd0): `MenuSysPage_OnActivate(this)`.
pub fn power_up_fx_toggles_menu_on_activate(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x002c_bf60), 0x0024_ffe0)
}

/// `void PowerUpFXTogglesMenu_OnDeactivate(void)` (0x24fff0): `MenuSysPage_OnDeactivate(this)`.
pub fn power_up_fx_toggles_menu_on_deactivate(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x002c_bfd0), 0x0025_0000)
}

/// `void SnowfallTogglesMenu_OnActivate(void)` (0x250010): `MenuSysPage_OnActivate(this)`.
pub fn snowfall_toggles_menu_on_activate(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x002c_bf60), 0x0025_0020)
}

/// `void SnowfallTogglesMenu_OnDeactivate(void)` (0x250030): `MenuSysPage_OnDeactivate(this)`.
pub fn snowfall_toggles_menu_on_deactivate(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x002c_bfd0), 0x0025_0040)
}

/// `void CameraSplashMenu_OnActivate(void)` (0x250050): `MenuSysPage_OnActivate(this)`.
pub fn camera_splash_menu_on_activate(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x002c_bf60), 0x0025_0060)
}

/// `void CameraSplashMenu_OnDeactivate(void)` (0x250070): `MenuSysPage_OnDeactivate(this)`.
pub fn camera_splash_menu_on_deactivate(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x002c_bfd0), 0x0025_0080)
}

/// `void TubeMenu_OnActivate(void)` (0x250128): `MenuSysPage_OnActivate(this)`.
pub fn tube_menu_on_activate(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x002c_bf60), 0x0025_0138)
}

/// `void TubeMenu_OnDeactivate(void)` (0x250148): `MenuSysPage_OnDeactivate(this)`.
pub fn tube_menu_on_deactivate(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x002c_bfd0), 0x0025_0158)
}

/// `void PathArrowMenu_OnActivate(void)` (0x250168): `MenuSysPage_OnActivate(this)`.
pub fn path_arrow_menu_on_activate(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x002c_bf60), 0x0025_0178)
}

/// `void PathArrowMenu_OnDeactivate(void)` (0x250188): `MenuSysPage_OnDeactivate(this)`.
pub fn path_arrow_menu_on_deactivate(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x002c_bfd0), 0x0025_0198)
}

/// `void VisualEffectsMenus_GlobalCtors(void)` (0x2501a8): construct the
/// static menus (`VisualEffectsMenus_StaticInitDestroy(1, 0xffff)`).
pub fn visual_effects_menus_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        OC527_VISUAL_EFFECTS_STATIC_INIT_DESTROY.call(c, 0x0025_01bc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void VisualEffectsMenus_GlobalDtors(void)` (0x2501c8): destroy the
/// static menus (`VisualEffectsMenus_StaticInitDestroy(0, 0xffff)`).
pub fn visual_effects_menus_global_dtors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        OC527_VISUAL_EFFECTS_STATIC_INIT_DESTROY.call(c, 0x0025_01dc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::{fill, DATA};

    const SEEDS: u32 = 100;

    /// A menu page wrapper's fixture: `a0` points at random page data.
    fn page_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        c.g[A0][0] = DATA as u64;
        fill(c, &mut r, DATA, 0x100 / 4);
    }

    #[test]
    fn shadow_menu_on_activate_matches() {
        check(
            ee::s06::f_0024fdc0__generated,
            shadow_menu_on_activate,
            SEEDS,
            &|c, seed| page_setup(c, seed, 0x0024_fdc0),
        );
    }

    #[test]
    fn shadow_menu_on_deactivate_matches() {
        check(
            ee::s06::f_0024fde0__generated,
            shadow_menu_on_deactivate,
            SEEDS,
            &|c, seed| page_setup(c, seed, 0x0024_fde0),
        );
    }

    #[test]
    fn snow_surface_menu_on_activate_matches() {
        check(
            ee::s06::f_0024fea8__generated,
            snow_surface_menu_on_activate,
            SEEDS,
            &|c, seed| page_setup(c, seed, 0x0024_fea8),
        );
    }

    #[test]
    fn snow_surface_menu_on_deactivate_matches() {
        check(
            ee::s06::f_0024fec8__generated,
            snow_surface_menu_on_deactivate,
            SEEDS,
            &|c, seed| page_setup(c, seed, 0x0024_fec8),
        );
    }

    #[test]
    fn board_trail_menu_on_activate_matches() {
        check(
            ee::s06::f_0024fee8__generated,
            board_trail_menu_on_activate,
            SEEDS,
            &|c, seed| page_setup(c, seed, 0x0024_fee8),
        );
    }

    #[test]
    fn board_trail_menu_on_deactivate_matches() {
        check(
            ee::s06::f_0024ff08__generated,
            board_trail_menu_on_deactivate,
            SEEDS,
            &|c, seed| page_setup(c, seed, 0x0024_ff08),
        );
    }

    #[test]
    fn flags_menu_on_activate_matches() {
        check(
            ee::s06::f_0024ff90__generated,
            flags_menu_on_activate,
            SEEDS,
            &|c, seed| page_setup(c, seed, 0x0024_ff90),
        );
    }

    #[test]
    fn flags_menu_on_deactivate_matches() {
        check(
            ee::s06::f_0024ffb0__generated,
            flags_menu_on_deactivate,
            SEEDS,
            &|c, seed| page_setup(c, seed, 0x0024_ffb0),
        );
    }

    #[test]
    fn power_up_fx_toggles_menu_on_activate_matches() {
        check(
            ee::s06::f_0024ffd0__generated,
            power_up_fx_toggles_menu_on_activate,
            SEEDS,
            &|c, seed| page_setup(c, seed, 0x0024_ffd0),
        );
    }

    #[test]
    fn power_up_fx_toggles_menu_on_deactivate_matches() {
        check(
            ee::s06::f_0024fff0__generated,
            power_up_fx_toggles_menu_on_deactivate,
            SEEDS,
            &|c, seed| page_setup(c, seed, 0x0024_fff0),
        );
    }

    #[test]
    fn snowfall_toggles_menu_on_activate_matches() {
        check(
            ee::s06::f_00250010__generated,
            snowfall_toggles_menu_on_activate,
            SEEDS,
            &|c, seed| page_setup(c, seed, 0x0025_0010),
        );
    }

    #[test]
    fn snowfall_toggles_menu_on_deactivate_matches() {
        check(
            ee::s06::f_00250030__generated,
            snowfall_toggles_menu_on_deactivate,
            SEEDS,
            &|c, seed| page_setup(c, seed, 0x0025_0030),
        );
    }

    #[test]
    fn camera_splash_menu_on_activate_matches() {
        check(
            ee::s06::f_00250050__generated,
            camera_splash_menu_on_activate,
            SEEDS,
            &|c, seed| page_setup(c, seed, 0x0025_0050),
        );
    }

    #[test]
    fn camera_splash_menu_on_deactivate_matches() {
        check(
            ee::s06::f_00250070__generated,
            camera_splash_menu_on_deactivate,
            SEEDS,
            &|c, seed| page_setup(c, seed, 0x0025_0070),
        );
    }

    #[test]
    fn tube_menu_on_activate_matches() {
        check(
            ee::s06::f_00250128__generated,
            tube_menu_on_activate,
            SEEDS,
            &|c, seed| page_setup(c, seed, 0x0025_0128),
        );
    }

    #[test]
    fn tube_menu_on_deactivate_matches() {
        check(
            ee::s06::f_00250148__generated,
            tube_menu_on_deactivate,
            SEEDS,
            &|c, seed| page_setup(c, seed, 0x0025_0148),
        );
    }

    #[test]
    fn path_arrow_menu_on_activate_matches() {
        check(
            ee::s06::f_00250168__generated,
            path_arrow_menu_on_activate,
            SEEDS,
            &|c, seed| page_setup(c, seed, 0x0025_0168),
        );
    }

    #[test]
    fn path_arrow_menu_on_deactivate_matches() {
        check(
            ee::s06::f_00250188__generated,
            path_arrow_menu_on_deactivate,
            SEEDS,
            &|c, seed| page_setup(c, seed, 0x0025_0188),
        );
    }

    #[test]
    fn visual_effects_menus_global_ctors_matches() {
        check(
            ee::s06::f_002501a8__generated,
            visual_effects_menus_global_ctors,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x0025_01a8, 0);
            },
        );
    }

    #[test]
    fn visual_effects_menus_global_dtors_matches() {
        check(
            ee::s06::f_002501c8__generated,
            visual_effects_menus_global_dtors,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x0025_01c8, 0);
            },
        );
    }
}
