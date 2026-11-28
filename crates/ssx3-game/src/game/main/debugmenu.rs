//! `src/main/debugmenu.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::tail_wrapper;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Target;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::replay::playbackman::oc683_off;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0024_d710,
        render_toggles_menu_dtor,
        "RenderTogglesMenu_dtor",
    ),
    (
        0x0024_d998,
        visual_effects_test_menu_dtor,
        "VisualEffectsTestMenu_dtor",
    ),
    (0x0024_dc68, depth_fog_menu_dtor, "DepthFogMenu_dtor"),
    (
        0x0024_e200,
        snowfall_toggles_menu_dtor,
        "SnowfallTogglesMenu_dtor",
    ),
    (
        0x0024_f6c0,
        visual_effects_main_menu_dtor,
        "VisualEffectsMainMenu_dtor",
    ),
    (0x0024_f860, plants_menu_dtor, "PlantsMenu_dtor"),
    (0x0024_fa00, height_fog_menu_dtor, "HeightFogMenu_dtor"),
    (
        0x0024_fab8,
        detail_system_menu_dtor,
        "DetailSystemMenu_dtor",
    ),
    (
        0x0024_fbc0,
        shell_renderer_menu_dtor,
        "ShellRendererMenu_dtor",
    ),
    (0x0024_fd00, shadow_menu_dtor, "ShadowMenu_dtor"),
    (0x0024_fe00, snow_surface_menu_dtor, "SnowSurfaceMenu_dtor"),
    (0x0024_ff28, flags_menu_dtor, "FlagsMenu_dtor"),
    (0x0025_0090, tube_menu_dtor, "TubeMenu_dtor"),
    (
        0x0025_09d8,
        aivisual_effects_main_menu_ctor,
        "AIVisualEffectsMainMenu_ctor",
    ),
    (
        0x0025_0a60,
        aivisual_effects_main_menu_on_activate,
        "AIVisualEffectsMainMenu_OnActivate",
    ),
    (
        0x0025_0a80,
        aivisual_effects_main_menu_on_deactivate,
        "AIVisualEffectsMainMenu_OnDeactivate",
    ),
    (
        0x0025_15f0,
        avalanche_menu_on_activate,
        "AvalancheMenu_OnActivate",
    ),
    (
        0x0025_1610,
        avalanche_menu_on_deactivate,
        "AvalancheMenu_OnDeactivate",
    ),
    (
        0x0025_1630,
        aivisual_effects_main_menu_dtor,
        "AIVisualEffectsMainMenu_dtor",
    ),
    (
        0x0025_1690,
        aivisual_effects_menus_global_ctors,
        "AIVisualEffectsMenus_GlobalCtors",
    ),
    (
        0x0025_16b0,
        aivisual_effects_menus_global_dtors,
        "AIVisualEffectsMenus_GlobalDtors",
    ),
    (0x0025_16d0, mem_init_block_header, "MEM_InitBlockHeader"),
    (
        0x0024_7e60,
        render_toggles_menu_ctor,
        "RenderTogglesMenu_ctor",
    ),
    (
        0x0024_bf60,
        splash_toggles_menu_ctor,
        "CameraSplashMenu_ctor",
    ),
    (
        0x0024_e8d8,
        visual_effects_menus_static_init_destroy,
        "VisualEffectsMenus_StaticInitDestroy",
    ),
    (0x0024_9148, light_glow_menu_ctor, "LightGlowMenu_ctor"),
    (0x0024_a228, screen_tint_menu_ctor, "ScreenTintMenu_ctor"),
    (0x0024_abb8, shadow_menu_ctor, "ShadowMenu_ctor"),
    (0x0024_ae90, snow_surface_menu_ctor, "SnowSurfaceMenu_ctor"),
    (0x0024_b0e8, board_trail_menu_ctor, "BoardTrailMenu_ctor"),
    (0x0024_b6d0, flags_menu_ctor, "FlagsMenu_ctor"),
    (
        0x0024_b7d0,
        power_up_fx_toggles_menu_ctor,
        "PowerUpFXTogglesMenu_ctor",
    ),
    (
        0x0024_bbe8,
        snowfall_toggles_menu_ctor,
        "SnowfallTogglesMenu_ctor",
    ),
    (0x0024_cca0, tube_menu_ctor, "TubeMenu_ctor"),
    (0x0024_ceb0, path_arrow_menu_ctor, "PathArrowMenu_ctor"),
    (
        0x0024_d2f0,
        visual_effects_main_menu_ctor,
        "VisualEffectsMainMenu_ctor",
    ),
    (
        0x0024_d690,
        visual_effects_main_menu_on_activate,
        "VisualEffectsMainMenu_OnActivate",
    ),
    (
        0x0024_d6b0,
        visual_effects_main_menu_on_deactivate,
        "VisualEffectsMainMenu_OnDeactivate",
    ),
    (0x0024_dab8, light_glow_menu_dtor_2, "LightGlowMenu_dtor"),
    (0x0024_dd70, screen_tint_menu_dtor_2, "ScreenTintMenu_dtor"),
    (0x0024_dfa0, board_trail_menu_dtor_2, "BoardTrailMenu_dtor"),
    (
        0x0024_e0f8,
        power_up_fx_toggles_menu_dtor_2,
        "PowerUpFXTogglesMenu_dtor",
    ),
    (
        0x0024_e388,
        camera_splash_menu_dtor,
        "CameraSplashMenu_dtor",
    ),
    (0x0024_e7c8, path_arrow_menu_dtor, "PathArrowMenu_dtor"),
    (
        0x0024_f7e0,
        render_toggles_menu_on_activate,
        "RenderTogglesMenu_OnActivate",
    ),
    (
        0x0024_f800,
        render_toggles_menu_on_deactivate,
        "RenderTogglesMenu_OnDeactivate",
    ),
    (
        0x0024_f820,
        visual_effects_test_menu_on_activate,
        "VisualEffectsTestMenu_OnActivate",
    ),
    (
        0x0024_f840,
        visual_effects_test_menu_on_deactivate,
        "VisualEffectsTestMenu_OnDeactivate",
    ),
    (
        0x0024_f940,
        plants_menu_on_activate,
        "PlantsMenu_OnActivate",
    ),
    (
        0x0024_f960,
        plants_menu_on_deactivate,
        "PlantsMenu_OnDeactivate",
    ),
    (
        0x0024_f980,
        light_glow_menu_on_activate,
        "LightGlowMenu_OnActivate",
    ),
    (
        0x0024_f9a0,
        light_glow_menu_on_deactivate,
        "LightGlowMenu_OnDeactivate",
    ),
    (
        0x0024_f9c0,
        depth_fog_menu_on_activate,
        "DepthFogMenu_OnActivate",
    ),
    (
        0x0024_f9e0,
        depth_fog_menu_on_deactivate,
        "DepthFogMenu_OnDeactivate",
    ),
    (
        0x0024_fa78,
        height_fog_menu_on_activate,
        "HeightFogMenu_OnActivate",
    ),
    (
        0x0024_fa98,
        height_fog_menu_on_deactivate,
        "HeightFogMenu_OnDeactivate",
    ),
    (
        0x0024_fb80,
        detail_system_menu_on_activate,
        "DetailSystemMenu_OnActivate",
    ),
    (
        0x0024_fba0,
        detail_system_menu_on_deactivate,
        "DetailSystemMenu_OnDeactivate",
    ),
    (
        0x0024_fc80,
        shell_renderer_menu_on_activate,
        "ShellRendererMenu_OnActivate",
    ),
    (
        0x0024_fca0,
        shell_renderer_menu_on_deactivate,
        "ShellRendererMenu_OnDeactivate",
    ),
    (
        0x0024_fcc0,
        screen_tint_menu_on_activate,
        "ScreenTintMenu_OnActivate",
    ),
    (
        0x0024_fce0,
        screen_tint_menu_on_deactivate,
        "ScreenTintMenu_OnDeactivate",
    ),
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
    (0x0025_01e8, avalanche_menu_ctor, "AvalancheMenu_ctor"),
    (0x0025_0ae0, avalanche_menu_ctor_2, "AvalancheMenu_dtor"),
    (
        0x0025_0cb0,
        aivisual_effects_menus_static_init_destroy,
        "AIVisualEffectsMenus_StaticInitDestroy",
    ),
    (
        0x0024_8970,
        visual_effects_test_menu_ctor,
        "VisualEffectsTestMenu_ctor",
    ),
    (0x0024_8dc8, plants_menu_ctor, "PlantsMenu_ctor"),
    (0x0024_98a0, depth_fog_menu_ctor, "DepthFogMenu_ctor"),
    (0x0024_9b28, height_fog_menu_ctor, "HeightFogMenu_ctor"),
    (
        0x0024_9c68,
        detail_system_menu_ctor,
        "DetailSystemMenu_ctor",
    ),
    (
        0x0024_9f80,
        shell_renderer_menu_ctor_2,
        "ShellRendererMenu_ctor",
    ),
];

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod addr {

    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_DTOR_2;
}

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {

    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_BASE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_ITEM_DTOR;
}

// ---------------------------------------------------------------------------
// Memory heap.

/// `void MEM_InitBlockHeader(MEMBlock *block, uint16 magic, uint32 prev,
/// uint32 next, uint32 size)` (0x2516d0): fill the block header at `block`.
pub fn mem_init_block_header(c: &mut Cpu) {
    c.cycles += 8;
    let block = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0x424d;
    block.off(0xc).sw(c, c.g[T1][0]);
    block.sh(c, c.g[V0][0]);
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
    block.off(0x2).sh(c, c.g[T0][0]);
    block.off(0x8).sw(c, c.g[T2][0]);
    block.off(0x4).sw(c, c.g[A2][0]);
    abi::ret(c);
}

pub(crate) const AIVISUAL_EFFECTS_MENUS_STATIC_INIT_DESTROY: Callee = Callee {
    addr: 0x0025_0cb0,
    f: ee::s06::f_00250cb0,
};

/// `AIVisualEffectsMainMenu *AIVisualEffectsMainMenu_ctor(AIVisualEffectsMainMenu *this)`
/// (0x2509d8): construct the page, a label item, a spacer item and a submenu
/// item and insert the submenu into the page.
pub fn aivisual_effects_main_menu_ctor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x002c_aa58, 0x0025_09f0)?; // MenuSysPage_ctor(this)
        let s0 = c.g[S0][0];
        c.g[S1][0] = sx((s0 as u32).wrapping_add(0x164));
        c.g[V1][0] = 0x48_0000;
        c.g[A1][0] = 0x48_0000;
        c.g[V1][0] = sx(0x48_0000u32.wrapping_sub(0x18));
        c.g[A0][0] = sx((s0 as u32).wrapping_add(0x130));
        c.sw((s0 as u32).wrapping_add(0x12c), c.g[V1][0]);
        c.g[A1][0] = sx(0x48_0000u32.wrapping_sub(0x50));
        call_ptr(c, 0x002c_d2a0, 0x0025_0a10)?; // MenuSysLabelItem_ctor
        let s0 = c.g[S0][0];
        c.g[A0][0] = sx((s0 as u32).wrapping_add(0x14c));
        c.g[A1][0] = 0x28;
        call_ptr(c, 0x002c_d350, 0x0025_0a1c)?; // MenuSysSpacerItem_ctor
        c.g[A1][0] = 0x4d_0000;
        c.g[A2][0] = 0x48_0000;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = sx(0x4d_0000u32.wrapping_add(0x1c88));
        c.g[A2][0] = sx(0x48_0000u32.wrapping_sub(0x30));
        call_ptr(c, 0x002c_ccc8, 0x0025_0a34)?; // MenuSysSubmenuItem_ctor
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x0025_0a44)?; // MenuSysPage_InsertItem
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AIVisualEffectsMainMenu_OnActivate(AIVisualEffectsMainMenu *this)`
/// (0x250a60): `MenuSysPage_OnActivate(this)`.
pub fn aivisual_effects_main_menu_on_activate(c: &mut Cpu) {
    call::tail_wrapper(c, None, call::Target::Table(0x002c_bf60), 0x0025_0a70)
}

/// `void AIVisualEffectsMainMenu_OnDeactivate(AIVisualEffectsMainMenu *this)`
/// (0x250a80): `MenuSysPage_OnDeactivate(this)`.
pub fn aivisual_effects_main_menu_on_deactivate(c: &mut Cpu) {
    call::tail_wrapper(c, None, call::Target::Table(0x002c_bfd0), 0x0025_0a90)
}

/// `void AIVisualEffectsMainMenu_dtor(AIVisualEffectsMainMenu *this, uint flags)`
/// (0x251630): destruct the submenu, spacer, label and page, then the base.
pub fn aivisual_effects_main_menu_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x164));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002c_a280, 0x0025_1654)?; // MenuSysItem_dtor
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002c_a280, 0x0025_1660)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x130));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002c_a280, 0x0025_166c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x002c_aa80, 0x0025_1678)?; // MenuSysPage_dtor
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AIVisualEffectsMenus_GlobalCtors(void)` (0x251690): construct the
/// static menus (`AIVisualEffectsMenus_StaticInitDestroy(1, 0xffff)`).
pub fn aivisual_effects_menus_global_ctors(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        AIVISUAL_EFFECTS_MENUS_STATIC_INIT_DESTROY.call(c, 0x0025_16a4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AIVisualEffectsMenus_GlobalDtors(void)` (0x2516b0): destroy the
/// static menus (`AIVisualEffectsMenus_StaticInitDestroy(0, 0xffff)`).
pub fn aivisual_effects_menus_global_dtors(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        AIVISUAL_EFFECTS_MENUS_STATIC_INIT_DESTROY.call(c, 0x0025_16c4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Menu hooks and thunks.

/// `void AvalancheMenu_OnActivate(AvalancheMenu *self)` (0x2515f0):
/// `MenuSysPage_OnActivate(self)`.
pub fn avalanche_menu_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x002c_bf60, 0x0025_1600)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AvalancheMenu_OnDeactivate(AvalancheMenu *self)` (0x251610):
/// `MenuSysPage_OnDeactivate(self)`.
pub fn avalanche_menu_on_deactivate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x002c_bfd0, 0x0025_1620)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `a + off`.
#[inline(always)]
pub(crate) fn at(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

// ---------------------------------------------------------------------------
// Destructors.

/// `void FlagsMenu_dtor(FlagsMenu* self, int flags)` (0x24ff28): destroy the
/// four sub-menus (each as a `MenuSysItem`) and then the page base (with
/// `flags`). `flags & 1` asks the page base to free `self`.
pub fn flags_menu_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        open(c, 0x30);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        let this = c.g[A0][0] as u32;
        c.g[S0][0] = sx(this);
        c.g[S1][0] = c.g[A1][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        // The four sub-menus, each destroyed as its base `MenuSysItem`.
        for (off, ret) in [
            (0x194u32, 0x0024_ff4cu32),
            (0x164, 0x0024_ff58),
            (0x14c, 0x0024_ff64),
            (0x130, 0x0024_ff70),
        ] {
            c.g[A0][0] = sx(at(this, off));
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x002c_a280, ret)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x002c_aa80, 0x0024_ff7c)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x30);
        Ok(())
    })
}

/// `HeightFogMenu_dtor` (0x24FA00): destroy the height fog menu's five
/// sub-menus and then the menu base.
pub fn height_fog_menu_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        for (off, ret) in [
            (0x1c8u32, 0x0024_fa24u32),
            (0x194, 0x0024_fa30),
            (0x164, 0x0024_fa3c),
            (0x14c, 0x0024_fa48),
            (0x130, 0x0024_fa54),
        ] {
            c.g[A0][0] = low(c, S0).wrapping_add(off as u64);
            c.g[A1][0] = 2;
            call_ptr(c, addr::MENU_DTOR_2, ret)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, addr::MENU_DTOR, 0x0024_fa60)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `TubeMenu_dtor` (0x250090): destroy the menu's eight child objects and
/// then the base menu.
pub fn tube_menu_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        for (off, ret) in [
            (0x25c, 0x0025_00b4),
            (0x22c, 0x0025_00c0),
            (0x1fc, 0x0025_00cc),
            (0x1c8, 0x0025_00d8),
            (0x194, 0x0025_00e4),
            (0x164, 0x0025_00f0),
            (0x14c, 0x0025_00fc),
            (0x130, 0x0025_0108),
        ] {
            c.g[A0][0] = sx(this.wrapping_add(off));
            c.g[RA][0] = sx(ret);
            c.g[A1][0] = 2;
            call_ptr(c, callee::TUBE_MENU_ITEM_DTOR, ret)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0025_0114);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, callee::TUBE_MENU_BASE_DTOR, 0x0025_0114)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SnowSurfaceMenu_dtor(self, flags)` (0x24fe00): destroy the menu's
/// nine sub-objects and the base object.
pub fn snow_surface_menu_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 41;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        for (off, ret) in [
            (0x294u32, 0x0024_fe24u32),
            (0x260, 0x0024_fe30),
            (0x22c, 0x0024_fe3c),
            (0x1f8, 0x0024_fe48),
            (0x1c4, 0x0024_fe54),
            (0x194, 0x0024_fe60),
            (0x164, 0x0024_fe6c),
            (0x14c, 0x0024_fe78),
            (0x130, 0x0024_fe84),
        ] {
            c.g[A0][0] = sx(p(r32(c, S0), off));
            c.g[A1][0] = 2;
            call_ptr(c, 0x002c_a280, ret)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x002c_aa80, 0x0024fe90)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Menu destructors.

/// `void ShellRendererMenu_dtor(ShellRendererMenu* self, int flags)`
/// (0x24fbc0): destroy the eleven sub-menus as `MenuSysItem`s, then the page
/// base (which frees `self` when `flags & 1`).
pub fn shell_renderer_menu_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 47;
        open(c, 0x30);
        c.sq(16, at(c.g[SP][0] as u32, 0x20));
        c.sq(17, at(c.g[SP][0] as u32, 0x10));
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[S1][0] = c.g[A1][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        for (off, ret) in [
            (0x2ecu32, 0x0024_fbe4u32),
            (0x2b8, 0x0024_fbf0),
            (0x284, 0x0024_fbfc),
            (0x254, 0x0024_fc08),
            (0x224, 0x0024_fc14),
            (0x1f4, 0x0024_fc20),
            (0x1c4, 0x0024_fc2c),
            (0x194, 0x0024_fc38),
            (0x164, 0x0024_fc44),
            (0x14c, 0x0024_fc50),
            (0x130, 0x0024_fc5c),
        ] {
            c.g[A0][0] = sx(at(this as u32, off));
            c.g[A1][0] = 2;
            call_ptr(c, 0x002c_a280, ret)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x002c_aa80, 0x0024_fc68)?;
        c.lq(16, at(c.g[SP][0] as u32, 0x20));
        c.lq(17, at(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x30);
        Ok(())
    })
}

/// `void ShadowMenu_dtor(ShadowMenu* self, int flags)` (0x24fd00): destroy the
/// eleven sub-menus as `MenuSysItem`s, then the page base.
pub fn shadow_menu_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 47;
        open(c, 0x30);
        c.sq(16, at(c.g[SP][0] as u32, 0x20));
        c.sq(17, at(c.g[SP][0] as u32, 0x10));
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[S1][0] = c.g[A1][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        for (off, ret) in [
            (0x2fcu32, 0x0024_fd24u32),
            (0x2c8, 0x0024_fd30),
            (0x294, 0x0024_fd3c),
            (0x260, 0x0024_fd48),
            (0x22c, 0x0024_fd54),
            (0x1f8, 0x0024_fd60),
            (0x1c4, 0x0024_fd6c),
            (0x194, 0x0024_fd78),
            (0x164, 0x0024_fd84),
            (0x14c, 0x0024_fd90),
            (0x130, 0x0024_fd9c),
        ] {
            c.g[A0][0] = sx(at(this as u32, off));
            c.g[A1][0] = 2;
            call_ptr(c, 0x002c_a280, ret)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x002c_aa80, 0x0024_fda8)?;
        c.lq(16, at(c.g[SP][0] as u32, 0x20));
        c.lq(17, at(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x30);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end lifecycle.

/// `void DetailSystemMenu_dtor(DetailSystemMenu *self, int flags)`
/// (0x24fab8): destroy the twelve sub-objects at +0x334..+0x130 (each with
/// `flags = 2`) and then the menu itself.
pub fn detail_system_menu_dtor(c: &mut Cpu) {
    // (offset, return address) pairs, in call order.
    const DESTROYED: [(u32, u32); 12] = [
        (0x334, 0x0024_fadc),
        (0x300, 0x0024_fae8),
        (0x2cc, 0x0024_faf4),
        (0x298, 0x0024_fb00),
        (0x264, 0x0024_fb0c),
        (0x230, 0x0024_fb18),
        (0x1fc, 0x0024_fb24),
        (0x1c8, 0x0024_fb30),
        (0x194, 0x0024_fb3c),
        (0x164, 0x0024_fb48),
        (0x14c, 0x0024_fb54),
        (0x130, 0x0024_fb60),
    ];
    guest(c, |c| {
        c.cycles += 50;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        for (off, ret) in DESTROYED {
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(off));
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x002c_a280, ret)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x002c_aa80, 0x0024_fb6c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PlantsMenu_dtor (0x000024f860).

/// `PlantsMenu_dtor`: destroy the plants menu: free its child objects and the base screen.
pub fn plants_menu_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 56;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x398));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002ca280, 0x0024f884)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x364));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002ca280, 0x0024f890)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x330));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002ca280, 0x0024f89c)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2fc));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002ca280, 0x0024f8a8)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2c8));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002ca280, 0x0024f8b4)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x298));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002ca280, 0x0024f8c0)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x264));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002ca280, 0x0024f8cc)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x230));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002ca280, 0x0024f8d8)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1fc));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002ca280, 0x0024f8e4)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c8));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002ca280, 0x0024f8f0)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x194));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002ca280, 0x0024f8fc)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x164));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002ca280, 0x0024f908)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002ca280, 0x0024f914)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x130));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002ca280, 0x0024f920)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x002caa80, 0x0024f92c)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    });
}

/// `DepthFogMenu_dtor(self, flags)` (0x24dc68): destroy the depth-fog menu's
/// items and pages, then the menu base.
pub fn depth_fog_menu_dtor(c: &mut Cpu) {
    const ITEM_DTOR: u32 = 0x002c_a280;
    const PAGE_DTOR: u32 = 0x002c_aa80;
    const MENU_DTOR: u32 = 0x002c_aa80;

    guest(c, |c| {
        c.cycles += 65;
        let frame = Frame::open(c, 0x40);
        let this = abi::arg(c, 0);
        frame.save(c, 16, 0x30);
        frame.save(c, 18, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, 17, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(this.wrapping_add(0x2c4));
        const ITEMS: [u32; 16] = [
            0x904, 0x8e0, 0x8bc, 0x898, 0x874, 0x2dc, 0x2c4, 0x290, 0x25c, 0x22c, 0x1f8, 0x1c4,
            0x194, 0x164, 0x14c, 0x130,
        ];
        for (i, off) in ITEMS.iter().enumerate() {
            c.g[A0][0] = sx(this.wrapping_add(*off));
            c.g[A1][0] = 2;
            call_ptr(
                c,
                if *off == 0x2dc { PAGE_DTOR } else { ITEM_DTOR },
                0x0024_dc94 + 0xc * i as u32,
            )?;
        }
        // The first entry is a page (MenuSysPage_dtor); the base dtor follows.
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, MENU_DTOR, 0x0024_dd54)?;
        c.lq(16, frame.at(c, 0x30));
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `PowerUpFXTogglesMenu_dtor(self, flags)` (0x24e0f8): destroy the
/// power-up FX toggles menu's items, then the menu base.
pub fn power_up_fx_toggles_menu_dtor(c: &mut Cpu) {
    const ITEM_DTOR: u32 = 0x002c_a280;
    const MENU_DTOR: u32 = 0x002c_aa80;

    guest(c, |c| {
        c.cycles += 65;
        let frame = Frame::open(c, 0x30);
        let this = abi::arg(c, 0);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        const ITEMS: [u32; 16] = [
            0x418, 0x3e8, 0x3b8, 0x388, 0x358, 0x328, 0x2f8, 0x2c8, 0x294, 0x260, 0x22c, 0x1f8,
            0x1c4, 0x194, 0x164, 0x14c,
        ];
        for (i, off) in ITEMS.iter().enumerate() {
            c.g[A0][0] = sx(this.wrapping_add(*off));
            c.g[A1][0] = 2;
            call_ptr(c, ITEM_DTOR, 0x0024_e11c + 0xc * i as u32)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, MENU_DTOR, 0x0024_e1e8)?;
        c.lq(16, frame.at(c, 0x20));
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int MPEG2_Get_Bits(int n)` — read `n` bits from the MPEG-2 bit reader.
pub(crate) const MPEG2_GET_BITS: u32 = 0x003b_1300;

/// `void MenuSysItem_dtor(MenuSysItem*, int flags)` and the page destructor.
pub(crate) const MENU_SYS_ITEM_DTOR: u32 = 0x002c_a280;

pub(crate) const MENU_SYS_PAGE_DTOR: u32 = 0x002c_aa80;

/// The MPEG-2 decoder's bit reader state pointer (`0x510000 - 0x5f78`).
pub(crate) const MPEG2_BIT_STATE: u32 = 0x0050_a088;

// ---------------------------------------------------------------------------
// MPEG-2 picture coding extension.

/// `void MPEG2_picture_coding_extension(void)` (0x3b20b8): read the 21 fields
/// of the picture coding extension into the decoder state. The bit widths and
/// return addresses are the original's call schedule.
pub(crate) const PCE_FIRST: [(u32, u32); 16] = [
    (4, 0x003b_20d4),
    (4, 0x003b_20e8),
    (4, 0x003b_20f4),
    (4, 0x003b_2100),
    (2, 0x003b_210c),
    (2, 0x003b_211c),
    (1, 0x003b_212c),
    (1, 0x003b_213c),
    (1, 0x003b_214c),
    (1, 0x003b_215c),
    (1, 0x003b_216c),
    (1, 0x003b_217c),
    (1, 0x003b_218c),
    (1, 0x003b_219c),
    (1, 0x003b_21ac),
    (1, 0x003b_21bc),
];

pub(crate) const PCE_LAST: [(u32, u32); 5] = [
    (1, 0x003b_21d0),
    (3, 0x003b_21e0),
    (1, 0x003b_21f0),
    (7, 0x003b_2200),
    (8, 0x003b_2210),
];

pub fn mpeg2_picture_coding_extension(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 68;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 4;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = 0x0051_0000;
        // The state block starts at 0x510000-0x6a90 and grows by four bytes
        // per field; two fields live in the bit reader's sub-object.
        c.g[S0][0] = 0x0051_0000;
        c.g[S0][0] = sx(0x0051_0000u32.wrapping_sub(0x6a90));
        let base = r32(c, S0);

        // Store field `i` where the original does: fields 10 and 12 live in
        // the bit reader's sub-object, the rest walk the state block.
        fn store(c: &mut Cpu, base: u32, i: usize, v: u64) {
            let dst = match i {
                9 => (Addr(MPEG2_BIT_STATE).lw(c) as u32).wrapping_add(0x428),
                11 => (Addr(MPEG2_BIT_STATE).lw(c) as u32).wrapping_add(0x42c),
                _ => base.wrapping_add(4 * (i - usize::from(i > 9) - usize::from(i > 11)) as u32),
            };
            Addr(dst).sw(c, v);
        }

        for (i, &(width, ret)) in PCE_FIRST.iter().enumerate() {
            match i {
                4..=9 | 11 | 13..=15 => c.g[V1][0] = 0x0051_0000,
                10 | 12 => c.g[V1][0] = Addr(MPEG2_BIT_STATE).lw(c),
                _ => {}
            }
            c.g[A0][0] = width as u64;
            call_ptr(c, MPEG2_GET_BITS, ret)?;
            let v = c.g[V0][0];
            store(c, base, i, v);
        }

        // The sixteenth field gates the rest of the extension.
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            frame.restore(c, S0, 0x20);
            frame.restore(c, S1, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        c.cycles += 20;
        for (i, &(width, ret)) in PCE_LAST.iter().enumerate() {
            c.g[V1][0] = 0x0051_0000;
            c.g[A0][0] = width as u64;
            call_ptr(c, MPEG2_GET_BITS, ret)?;
            Addr(base.wrapping_add(0x38 + 4 * i as u32)).sw(c, c.g[V0][0]);
        }

        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Options-menu page destructors.
//
// Every one of these destroys a fixed set of `MenuSysItem`s (with the
// non-deleting flag value 2) and then the page itself.

/// Destroy the embedded item at `this + off` (`MenuSysItem_dtor(x, 2)`).
pub(crate) fn item_dtor(c: &mut Cpu, this: u32, off: u32, ret: u32) -> Flow {
    c.g[A0][0] = sx(this.wrapping_add(off));
    c.g[A1][0] = 2;
    call_ptr(c, MENU_SYS_ITEM_DTOR, ret)
}

/// The shared body: `offsets[i]` is destroyed at `first_ret + 0xc*i`, then
/// `MenuSysPage_dtor(this, a1)` at the next return address.
pub(crate) fn menu_page_dtor(c: &mut Cpu, offsets: &[u32], first_ret: u32) {
    guest(c, |c| {
        c.cycles += 71;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = abi::arg64(c, 0);
        c.g[S1][0] = abi::arg64(c, 1);
        frame.save_ra(c, 0);

        let this = r32(c, S0);
        for (i, &off) in offsets.iter().enumerate() {
            item_dtor(c, this, off, first_ret + 0xc * i as u32)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(
            c,
            MENU_SYS_PAGE_DTOR,
            first_ret + 0xc * offsets.len() as u32,
        )?;

        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `VisualEffectsTestMenu_dtor` (0x24d998).
pub fn visual_effects_test_menu_dtor(c: &mut Cpu) {
    menu_page_dtor(
        c,
        &[
            0x464, 0x434, 0x404, 0x3d4, 0x3a4, 0x374, 0x344, 0x314, 0x2e4, 0x2b4, 0x284, 0x254,
            0x224, 0x1f4, 0x1c4, 0x194, 0x164, 0x14c, 0x130,
        ],
        0x0024_d9bc,
    );
}

/// `VisualEffectsMainMenu_dtor` (0x24f6c0).
pub fn visual_effects_main_menu_dtor(c: &mut Cpu) {
    menu_page_dtor(
        c,
        &[
            0x324, 0x308, 0x2ec, 0x2d0, 0x2b4, 0x298, 0x27c, 0x260, 0x244, 0x228, 0x20c, 0x1f0,
            0x1d4, 0x1b8, 0x19c, 0x180, 0x164, 0x14c, 0x130,
        ],
        0x0024_f6e4,
    );
}

/// `BoardTrailMenu_dtor` (0x24dfa0).
pub fn board_trail_menu_dtor(c: &mut Cpu) {
    menu_page_dtor(
        c,
        &[
            0x59c, 0x56c, 0x53c, 0x508, 0x4d4, 0x4a0, 0x46c, 0x438, 0x404, 0x3d0, 0x39c, 0x368,
            0x334, 0x300, 0x2cc, 0x298, 0x264, 0x230, 0x1fc, 0x1c8, 0x194, 0x164, 0x14c, 0x130,
        ],
        0x0024_dfc4,
    );
}

/// `LightGlowMenu_dtor` (0x24dab8).
pub fn light_glow_menu_dtor(c: &mut Cpu) {
    menu_page_dtor(
        c,
        &[
            0x700, 0x6cc, 0x698, 0x664, 0x630, 0x600, 0x5cc, 0x598, 0x564, 0x530, 0x4fc, 0x4c8,
            0x494, 0x460, 0x42c, 0x3fc, 0x3c8, 0x394, 0x360, 0x32c, 0x2f8, 0x2c4, 0x290, 0x25c,
            0x228, 0x1f4, 0x1c4, 0x194, 0x164, 0x14c, 0x130,
        ],
        0x0024_dadc,
    );
}

/// `AvalancheMenu_dtor` (0x250ae0).
pub fn avalanche_menu_dtor(c: &mut Cpu) {
    menu_page_dtor(
        c,
        &[
            0x77c, 0x748, 0x714, 0x6e0, 0x6b0, 0x67c, 0x648, 0x614, 0x5e0, 0x5ac, 0x578, 0x544,
            0x514, 0x4e0, 0x4ac, 0x478, 0x444, 0x410, 0x3dc, 0x3ac, 0x37c, 0x34c, 0x31c, 0x2e8,
            0x2b8, 0x288, 0x258, 0x228, 0x1f8, 0x1c4, 0x194, 0x164, 0x14c, 0x130,
        ],
        0x0025_0b04,
    );
}

/// `ScreenTintMenu_dtor` (0x24dd70).
pub fn screen_tint_menu_dtor(c: &mut Cpu) {
    menu_page_dtor(
        c,
        &[
            0x928, 0x8f4, 0x8c0, 0x88c, 0x858, 0x824, 0x7f0, 0x7c0, 0x78c, 0x758, 0x724, 0x6f0,
            0x6bc, 0x688, 0x654, 0x620, 0x5ec, 0x5b8, 0x584, 0x550, 0x520, 0x4f0, 0x4c0, 0x490,
            0x45c, 0x42c, 0x3fc, 0x3c8, 0x394, 0x360, 0x32c, 0x2f8, 0x2c4, 0x290, 0x25c, 0x228,
            0x1f4, 0x1c4, 0x194, 0x164, 0x14c, 0x130,
        ],
        0x0024_dd94,
    );
}

/// The `SnowfallTogglesMenu_dtor` steps: offset from `this`, and whether the
/// field is an embedded page rather than an item.
pub(crate) const SNOWFALL_STEPS: [(u32, bool); 26] = [
    (0xfec, false),
    (0xfd0, false),
    (0xfac, false),
    (0xf88, false),
    (0xf64, false),
    (0xf40, false),
    (0x9a8, true),
    (0x990, false),
    (0x974, false),
    (0x950, false),
    (0x92c, false),
    (0x908, false),
    (0x8e4, false),
    (0x34c, true),
    (0x334, false),
    (0x300, false),
    (0x2cc, false),
    (0x298, false),
    (0x264, false),
    (0x230, false),
    (0x1fc, false),
    (0x1c8, false),
    (0x194, false),
    (0x164, false),
    (0x14c, false),
    (0x130, false),
];

/// `SnowfallTogglesMenu_dtor` (0x24e200): 24 item destructors interleaved with
/// two embedded pages at `+0x9a8` and `+0x34c`, then the page itself.
pub fn snowfall_toggles_menu_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 98;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = abi::arg64(c, 0);
        c.g[S3][0] = abi::arg64(c, 1);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);

        let this = r32(c, S0);
        c.g[S1][0] = sx(this.wrapping_add(0x990));
        c.g[S2][0] = sx(this.wrapping_add(0x334));

        for (i, &(off, page)) in SNOWFALL_STEPS.iter().enumerate() {
            let ret = if i < 3 {
                0x0024_e22c + 0x10 * i as u32
            } else {
                0x0024_e258 + 0xc * (i as u32 - 3)
            };
            c.g[A0][0] = sx(this.wrapping_add(off));
            c.g[A1][0] = 2;
            let target = if page {
                MENU_SYS_PAGE_DTOR
            } else {
                MENU_SYS_ITEM_DTOR
            };
            call_ptr(c, target, ret)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, MENU_SYS_PAGE_DTOR, 0x0024_e36c)?;

        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `RenderTogglesMenu_dtor` (0x0024d710): destroy the debug render-toggles
/// menu: run the destructor (flags 2) of its 49 `MenuSysItem` sub-objects
/// -- laid out from `+0xa18` down to `+0x130`, 0x30 bytes apart with a few
/// 0x34 gaps -- and then of the menu object itself.
pub fn render_toggles_menu_dtor(c: &mut Cpu) {
    /// The 49 item sub-objects, in destruction order.
    const ITEMS: [u32; 49] = [
        0xa18, 0x9e8, 0x9b8, 0x988, 0x958, 0x924, 0x8f0, 0x8bc, 0x888, 0x858, 0x828, 0x7f8, 0x7c4,
        0x794, 0x764, 0x734, 0x704, 0x6d4, 0x6a4, 0x674, 0x644, 0x614, 0x5e4, 0x5b4, 0x584, 0x554,
        0x524, 0x4f4, 0x4c4, 0x494, 0x464, 0x434, 0x404, 0x3d4, 0x3a4, 0x374, 0x344, 0x314, 0x2e4,
        0x2b4, 0x284, 0x254, 0x224, 0x1f4, 0x1c4, 0x194, 0x164, 0x14c, 0x130,
    ];
    guest(c, |c| {
        c.cycles += 161;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let menu = c.g[S0][0];
        for (i, off) in ITEMS.iter().enumerate() {
            c.g[A0][0] = sx((menu as u32).wrapping_add(*off));
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x002c_a280, 0x0024_d734 + 0xc * i as u32)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x002c_aa80, 0x0024_d980)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Offsets of the menu's items in the menu object, in the order the
/// constructor adds them (items 0..=50; items 51..=57 are kept in s4, s5,
/// s3, fp, s2, s7 and s6).
pub(crate) const ITEM_OFF: [u32; 51] = [
    0x130, 0x14c, 0x164, 0x194, 0x1c4, 0x1f4, 0x224, 0x258, 0x28c, 0x2bc, 0x2ec, 0x31c, 0x350,
    0x384, 0x3b8, 0x3ec, 0x420, 0x454, 0x488, 0x4bc, 0x4f0, 0x524, 0x558, 0x58c, 0x5c0, 0x5f4,
    0x628, 0x65c, 0x690, 0x6c4, 0x6f8, 0x72c, 0x760, 0x794, 0x7c8, 0x7fc, 0xe58, 0x14b4, 0x14e8,
    0x151c, 0x1550, 0x1584, 0x15b8, 0x15ec, 0x1620, 0x1654, 0x1688, 0x16bc, 0x16f0, 0x1724, 0x1758,
];

/// Where an item's address is kept while the constructor runs: its frame
/// slot (`sp + 4 * i`) or a saved s-register.
#[derive(Clone, Copy)]
pub(crate) enum Item {
    Slot(usize),
    Reg(usize),
}

/// The second float argument (f13): a copy of an f-register, or a literal
/// loaded into `at` and moved with `mtc1`.
#[derive(Clone, Copy)]
pub(crate) enum Arg {
    Reg(usize),
    Lit(u64),
}

/// `this + off`, as the compiled code computes it from s0.
pub(crate) fn this_at(c: &Cpu, off: u32) -> u64 {
    sx((c.g[S0][0] as u32).wrapping_add(off))
}

/// `gp + off` (a pointer into the globals).
pub(crate) fn gp_at(c: &Cpu, off: u32) -> u64 {
    sx((c.g[GP][0] as u32).wrapping_add(off))
}

/// The frame slot that holds item `i`'s address.
pub(crate) fn slot_addr(c: &Cpu, i: usize) -> u32 {
    sp(c).wrapping_add(4 * i as u32)
}

/// `this->item[i] = this + ITEM_OFF[i]`: the address goes to the frame slot
/// and stays in v0.
pub(crate) fn store_item(c: &mut Cpu, i: usize) {
    c.g[V0][0] = this_at(c, ITEM_OFF[i]);
    let slot = slot_addr(c, i);
    let v = c.g[V0][0];
    Addr(slot).sw(c, v);
}

/// The value an item is passed as (its address).
pub(crate) fn item_value(c: &mut Cpu, item: Item) -> u64 {
    match item {
        Item::Slot(i) => {
            let slot = slot_addr(c, i);
            Addr(slot).lw(c)
        }
        Item::Reg(r) => c.g[r][0],
    }
}

/// `this->word[off] = v` (the menu object's own fields).
pub(crate) fn set_this_word(c: &mut Cpu, off: u32, v: u64) {
    let a = (c.g[S0][0] as u32).wrapping_add(off);
    Addr(a).sw(c, v);
}

/// `cIntMenuItem(item, index, &int at gp+value, min, max, label)` (0x2cd8f8).
pub(crate) fn int_item(
    c: &mut Cpu,
    item: Item,
    index: u64,
    value: u32,
    min: u64,
    max: u64,
    label: u64,
    ret: u32,
) -> Flow {
    let this = item_value(c, item);
    c.g[A0][0] = this;
    c.g[A1][0] = index;
    c.g[A2][0] = gp_at(c, value);
    c.g[A3][0] = min;
    c.g[T0][0] = max;
    c.g[T1][0] = label;
    call_ptr(c, 0x002c_d8f8, ret)
}

/// `cFloatMenuItem_ctor(item, index, &float at gp+value, label, min, max)`
/// (0x2cdce8): min in f12, max in f13. `post` sets one more f-register
/// after both arguments are read.
pub(crate) fn float_item(
    c: &mut Cpu,
    item: Item,
    index: u64,
    value: u32,
    label: u64,
    min: usize,
    max: Arg,
    post: Option<(usize, u64)>,
    ret: u32,
) -> Flow {
    let this = item_value(c, item);
    c.g[A0][0] = this;
    c.g[A1][0] = index;
    c.mov_s(12, min);
    match max {
        Arg::Reg(f) => c.mov_s(13, f),
        Arg::Lit(bits) => {
            c.g[AT][0] = bits;
            c.mtc1(1, 13);
        }
    }
    if let Some((f, bits)) = post {
        c.g[AT][0] = bits;
        c.mtc1(1, f);
    }
    c.g[A2][0] = gp_at(c, value);
    c.g[A3][0] = label;
    call_ptr(c, 0x002c_dce8, ret)
}

/// `cARGBMenuItem(item, index, label, &colour)` (0x2d02b0).
pub(crate) fn argb_item(
    c: &mut Cpu,
    item: Item,
    index: u64,
    label: u64,
    colour: u64,
    ret: u32,
) -> Flow {
    let this = item_value(c, item);
    c.g[A0][0] = this;
    c.g[A1][0] = index;
    c.g[A2][0] = label;
    c.g[A3][0] = colour;
    call_ptr(c, 0x002d_02b0, ret)
}

/// `MenuSysPage_InsertItem(this, item, -1)` (0x2cbcc8): append the item.
pub(crate) fn add_item(c: &mut Cpu, item: Item, ret: u32) -> Flow {
    let v = item_value(c, item);
    c.g[A1][0] = v;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = u64::MAX;
    call_ptr(c, 0x002c_bcc8, ret)
}

/// `cSplashTogglesMenu::cSplashTogglesMenu(this)` (0x24bf60; the symbol is
/// `CameraSplashMenu_ctor`): the debug menu page for the camera splash. It
/// sets the menu base up (`MenuSysPage_ctor`), then the title (a null item
/// labelled "Camera Splash Menu") and a space, then the 56 items of the
/// splash: int toggles and counts, float ranges, and the drop and crystal
/// tints. Every item is added to the page with index -1. Returns `this`.
pub fn splash_toggles_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 848;
        let frame = Frame::open(c, 0x1a0);
        frame.save(c, S0, 0x160);
        frame.save(c, S1, 0x150);
        frame.save(c, S2, 0x140);
        frame.save(c, S3, 0x130);
        frame.save(c, S4, 0x120);
        frame.save(c, S5, 0x110);
        frame.save(c, S6, 0x100);
        frame.save(c, S7, 0xf0);
        frame.save(c, FP, 0xe0);
        frame.save_ra(c, 0xd0);
        frame.save_fpr(c, 25, 0x198);
        frame.save_fpr(c, 24, 0x190);
        frame.save_fpr(c, 23, 0x188);
        frame.save_fpr(c, 22, 0x180);
        frame.save_fpr(c, 21, 0x178);
        frame.save_fpr(c, 20, 0x170);

        c.g[S0][0] = c.g[A0][0];
        c.g[S3][0] = this_at(c, 0x17f4);
        c.g[S4][0] = this_at(c, 0x178c);
        c.g[S5][0] = this_at(c, 0x17c0);
        c.g[S2][0] = this_at(c, 0x185c);
        c.g[S7][0] = this_at(c, 0x1890);
        c.g[FP][0] = this_at(c, 0x1828);
        c.g[S6][0] = this_at(c, 0x1eec);

        // The menu base.
        store_item(c, 0);
        call_ptr(c, 0x002c_aa58, 0x0024_bfd0)?;

        // The title: the float constants of the page, the vtable, then
        // cNullMenuItem(title, "Camera Splash Menu").
        c.mtc1(0, 20);
        store_item(c, 1);
        c.g[A1][0] = 0x0047_eee0;
        c.g[V0][0] = 0x0047_f7b8;
        let vtable = c.g[V0][0];
        set_this_word(c, 0x12c, vtable);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 21);
        c.g[AT][0] = 0x42c8_0000;
        c.mtc1(1, 23);
        c.g[AT][0] = 0x4220_0000;
        c.mtc1(1, 24);
        c.g[AT][0] = 0xffff_ffff_c120_0000;
        c.mtc1(1, 22);
        let title = item_value(c, Item::Slot(0));
        c.g[A0][0] = title;
        call_ptr(c, 0x002c_d2a0, 0x0024_c018)?;

        // The space item, 40 high (cSpaceMenuItem).
        c.g[AT][0] = 0x4434_0000;
        c.mtc1(1, 25);
        let space = item_value(c, Item::Slot(1));
        c.g[A0][0] = space;
        store_item(c, 2);
        c.g[A1][0] = 0x28;
        call_ptr(c, 0x002c_d350, 0x0024_c034)?;

        // The 56 items, in order (each block builds one item).
        // Enable (item 2)
        store_item(c, 3);
        store_item(c, 4);
        int_item(c, Item::Slot(2), 0, 0x115c, 0, 1, 0x4a2d30, 0x24c064)?;
        // Render (item 3)
        store_item(c, 5);
        int_item(c, Item::Slot(3), 1, 0x1160, 0, 1, 0x4a2da0, 0x24c08c)?;
        // NumDrops (item 4)
        store_item(c, 6);
        store_item(c, 7);
        int_item(c, Item::Slot(4), 2, 0x1164, 1, 30, 0x47eef8, 0x24c0bc)?;
        // NumIceCrystals (item 5)
        store_item(c, 8);
        store_item(c, 9);
        int_item(c, Item::Slot(5), 3, 0x1168, 1, 30, 0x47ef08, 0x24c0ec)?;
        // PercentageIceCrystal (item 6)
        store_item(c, 10);
        store_item(c, 11);
        float_item(
            c,
            Item::Slot(6),
            4,
            0x116c,
            0x47ef18,
            20,
            Arg::Reg(21),
            None,
            0x24c11c,
        )?;
        // PercentageIceCrystalSpawnChance (item 7)
        store_item(c, 12);
        float_item(
            c,
            Item::Slot(7),
            5,
            0x1170,
            0x47ef30,
            20,
            Arg::Reg(21),
            Some((21, 0x41200000)),
            0x24c14c,
        )?;
        // IceCrystalMinGrouping (item 8)
        store_item(c, 13);
        store_item(c, 14);
        int_item(c, Item::Slot(8), 6, 0x1174, 1, 10, 0x47ef50, 0x24c17c)?;
        // IceCrystalMaxGrouping (item 9)
        store_item(c, 15);
        store_item(c, 16);
        int_item(c, Item::Slot(9), 7, 0x1178, 1, 10, 0x47ef70, 0x24c1ac)?;
        // MaxSpawnPerCrystal (item 10)
        store_item(c, 17);
        int_item(c, Item::Slot(10), 8, 0x117c, 1, 10, 0x47ef90, 0x24c1d4)?;
        // FarthestImpactDistance (item 11)
        store_item(c, 18);
        store_item(c, 19);
        float_item(
            c,
            Item::Slot(11),
            9,
            0x1180,
            0x47efa8,
            20,
            Arg::Lit(0x44fa0000),
            None,
            0x24c208,
        )?;
        // LowestImpactIntensity (item 12)
        store_item(c, 20);
        float_item(
            c,
            Item::Slot(12),
            10,
            0x1184,
            0x47efc8,
            20,
            Arg::Lit(0x42f00000),
            None,
            0x24c234,
        )?;
        // MinimumSnowfallForImpacts (item 13)
        store_item(c, 21);
        store_item(c, 22);
        float_item(
            c,
            Item::Slot(13),
            11,
            0x1188,
            0x47efe0,
            20,
            Arg::Reg(21),
            None,
            0x24c264,
        )?;
        // ImpactMultiplier (item 14)
        store_item(c, 23);
        float_item(
            c,
            Item::Slot(14),
            12,
            0x118c,
            0x47f000,
            20,
            Arg::Reg(21),
            None,
            0x24c28c,
        )?;
        // LowestSnowfallAmount (item 15)
        store_item(c, 24);
        store_item(c, 25);
        float_item(
            c,
            Item::Slot(15),
            13,
            0x1190,
            0x47f018,
            20,
            Arg::Reg(21),
            None,
            0x24c2bc,
        )?;
        // SnowfallMultiplier (item 16)
        store_item(c, 26);
        store_item(c, 27);
        float_item(
            c,
            Item::Slot(16),
            14,
            0x1194,
            0x47f030,
            20,
            Arg::Reg(21),
            None,
            0x24c2ec,
        )?;
        // MinVelocityForPushoff (item 17)
        store_item(c, 28);
        store_item(c, 29);
        float_item(
            c,
            Item::Slot(17),
            15,
            0x1198,
            0x47f048,
            20,
            Arg::Reg(21),
            None,
            0x24c31c,
        )?;
        // DropMinLifeSpan (item 18)
        store_item(c, 30);
        store_item(c, 31);
        float_item(
            c,
            Item::Slot(18),
            16,
            0x119c,
            0x47f068,
            20,
            Arg::Reg(23),
            None,
            0x24c34c,
        )?;
        // DropMaxLifeSpan (item 19)
        store_item(c, 32);
        store_item(c, 33);
        float_item(
            c,
            Item::Slot(19),
            17,
            0x11a0,
            0x47f080,
            20,
            Arg::Reg(23),
            None,
            0x24c37c,
        )?;
        // DropMinSize (item 20)
        store_item(c, 34);
        float_item(
            c,
            Item::Slot(20),
            18,
            0x11a4,
            0x47f098,
            20,
            Arg::Reg(24),
            None,
            0x24c3a4,
        )?;
        // DropMaxSize (item 21)
        store_item(c, 35);
        store_item(c, 36);
        float_item(
            c,
            Item::Slot(21),
            19,
            0x11a8,
            0x47f0a8,
            20,
            Arg::Reg(24),
            None,
            0x24c3d4,
        )?;
        // DropMaxWidthHeightDifference (item 22)
        store_item(c, 37);
        store_item(c, 38);
        float_item(
            c,
            Item::Slot(22),
            20,
            0x11ac,
            0x47f0b8,
            20,
            Arg::Reg(21),
            None,
            0x24c404,
        )?;
        // DropMinVelocityJiggleX (item 23)
        store_item(c, 39);
        store_item(c, 40);
        float_item(
            c,
            Item::Slot(23),
            21,
            0x11b0,
            0x47f0e0,
            22,
            Arg::Reg(21),
            None,
            0x24c434,
        )?;
        // DropMaxVelocityJiggleX (item 24)
        store_item(c, 41);
        store_item(c, 42);
        float_item(
            c,
            Item::Slot(24),
            22,
            0x11b4,
            0x47f100,
            22,
            Arg::Reg(21),
            None,
            0x24c464,
        )?;
        // DropMinVelocityJiggleY (item 25)
        store_item(c, 43);
        store_item(c, 44);
        float_item(
            c,
            Item::Slot(25),
            23,
            0x11b8,
            0x47f120,
            22,
            Arg::Reg(21),
            None,
            0x24c494,
        )?;
        // DropMaxVelocityJiggleY (item 26)
        store_item(c, 45);
        store_item(c, 46);
        float_item(
            c,
            Item::Slot(26),
            24,
            0x11bc,
            0x47f140,
            22,
            Arg::Reg(21),
            None,
            0x24c4c4,
        )?;
        // DropMinSizeGrowth (item 27)
        store_item(c, 47);
        store_item(c, 48);
        float_item(
            c,
            Item::Slot(27),
            25,
            0x11c0,
            0x47f160,
            22,
            Arg::Reg(21),
            None,
            0x24c4f4,
        )?;
        // DropMaxSizeGrowth (item 28)
        store_item(c, 49);
        store_item(c, 50);
        float_item(
            c,
            Item::Slot(28),
            26,
            0x11c4,
            0x47f178,
            22,
            Arg::Reg(21),
            None,
            0x24c524,
        )?;
        // DropMinPushOffSpeed (item 29)
        float_item(
            c,
            Item::Slot(29),
            27,
            0x11c8,
            0x47f190,
            20,
            Arg::Reg(21),
            None,
            0x24c544,
        )?;
        // DropMaxPushOffSpeed (item 30)
        float_item(
            c,
            Item::Slot(30),
            28,
            0x11cc,
            0x47f1a8,
            20,
            Arg::Reg(21),
            None,
            0x24c564,
        )?;
        // DropMaxVelocity (item 31)
        float_item(
            c,
            Item::Slot(31),
            29,
            0x11d0,
            0x47f1c0,
            20,
            Arg::Reg(23),
            None,
            0x24c584,
        )?;
        // DropMinVelocityUsage (item 32)
        float_item(
            c,
            Item::Slot(32),
            30,
            0x11d4,
            0x47f1d8,
            20,
            Arg::Reg(21),
            None,
            0x24c5a4,
        )?;
        // DropMaxVelocityUsage (item 33)
        float_item(
            c,
            Item::Slot(33),
            31,
            0x11d8,
            0x47f1f0,
            20,
            Arg::Reg(21),
            None,
            0x24c5c4,
        )?;
        // DropMaxRotationChange (item 34)
        c.g[S1][0] = 0x47f208;
        float_item(
            c,
            Item::Slot(34),
            32,
            0x11dc,
            0x47f208,
            20,
            Arg::Reg(25),
            None,
            0x24c5e8,
        )?;
        // DropTintMin (item 35)
        argb_item(c, Item::Slot(35), 33, 0x47f230, 0x5046e0, 0x24c604)?;
        // DropTintMax (item 36)
        argb_item(c, Item::Slot(36), 34, 0x47f240, 0x5046f0, 0x24c620)?;
        // CrystalGroupSpacingX (item 37)
        float_item(
            c,
            Item::Slot(37),
            35,
            0x11e0,
            0x47f250,
            20,
            Arg::Reg(23),
            None,
            0x24c640,
        )?;
        // CrystalGroupSpacingY (item 38)
        float_item(
            c,
            Item::Slot(38),
            36,
            0x11e4,
            0x47f268,
            20,
            Arg::Reg(23),
            None,
            0x24c660,
        )?;
        // CrystalMinLifeSpan (item 39)
        float_item(
            c,
            Item::Slot(39),
            37,
            0x11e8,
            0x47f280,
            20,
            Arg::Reg(23),
            None,
            0x24c680,
        )?;
        // CrystalMaxLifeSpan (item 40)
        float_item(
            c,
            Item::Slot(40),
            38,
            0x11ec,
            0x47f298,
            20,
            Arg::Reg(23),
            None,
            0x24c6a0,
        )?;
        // CrystalMinSize (item 41)
        float_item(
            c,
            Item::Slot(41),
            39,
            0x11f0,
            0x47f2b0,
            20,
            Arg::Reg(24),
            None,
            0x24c6c0,
        )?;
        // CrystalMaxSize (item 42)
        float_item(
            c,
            Item::Slot(42),
            40,
            0x11f4,
            0x47f2c8,
            20,
            Arg::Reg(24),
            None,
            0x24c6e0,
        )?;
        // CrystalMaxWidthHeightDifference (item 43)
        float_item(
            c,
            Item::Slot(43),
            41,
            0x11f8,
            0x47f2e0,
            20,
            Arg::Reg(21),
            None,
            0x24c700,
        )?;
        // CrystalMinVelocityJiggleX (item 44)
        float_item(
            c,
            Item::Slot(44),
            42,
            0x11fc,
            0x47f308,
            22,
            Arg::Reg(21),
            None,
            0x24c720,
        )?;
        // CrystalMaxVelocityJiggleX (item 45)
        float_item(
            c,
            Item::Slot(45),
            43,
            0x1200,
            0x47f328,
            22,
            Arg::Reg(21),
            None,
            0x24c740,
        )?;
        // CrystalMinVelocityJiggleY (item 46)
        float_item(
            c,
            Item::Slot(46),
            44,
            0x1204,
            0x47f348,
            22,
            Arg::Reg(21),
            None,
            0x24c760,
        )?;
        // CrystalMaxVelocityJiggleY (item 47)
        float_item(
            c,
            Item::Slot(47),
            45,
            0x1208,
            0x47f368,
            22,
            Arg::Reg(21),
            None,
            0x24c780,
        )?;
        // CrystalMinJiggleTime (item 48)
        float_item(
            c,
            Item::Slot(48),
            46,
            0x120c,
            0x47f388,
            20,
            Arg::Reg(21),
            None,
            0x24c7a0,
        )?;
        // CrystalMaxJiggleTime (item 49)
        float_item(
            c,
            Item::Slot(49),
            47,
            0x1210,
            0x47f3b8,
            20,
            Arg::Reg(21),
            None,
            0x24c7c0,
        )?;
        // CrystalMinSizeGrowth (item 50)
        float_item(
            c,
            Item::Slot(50),
            48,
            0x1214,
            0x47f3e8,
            22,
            Arg::Reg(21),
            None,
            0x24c7e0,
        )?;
        // CrystalMaxSizeGrowth (item 51)
        float_item(
            c,
            Item::Reg(S4),
            49,
            0x1218,
            0x47f400,
            22,
            Arg::Reg(21),
            None,
            0x24c800,
        )?;
        // CrystalMinPushOffSpeed (item 52)
        float_item(
            c,
            Item::Reg(S5),
            50,
            0x121c,
            0x47f418,
            20,
            Arg::Reg(21),
            None,
            0x24c820,
        )?;
        // CrystalMaxPushOffSpeed (item 53)
        float_item(
            c,
            Item::Reg(S3),
            51,
            0x1220,
            0x47f438,
            20,
            Arg::Reg(21),
            None,
            0x24c840,
        )?;
        // CrystalMaxVelocity (item 54)
        float_item(
            c,
            Item::Reg(FP),
            52,
            0x1224,
            0x47f458,
            20,
            Arg::Reg(23),
            None,
            0x24c860,
        )?;
        // CrystalMaxRotationChange (item 55)
        float_item(
            c,
            Item::Reg(S2),
            53,
            0x1228,
            0x47f208,
            20,
            Arg::Reg(25),
            None,
            0x24c87c,
        )?;
        // CrystalTintMin (item 56)
        argb_item(c, Item::Reg(S7), 54, 0x47f478, 0x504700, 0x24c898)?;
        // CrystalTintMax (item 57)
        argb_item(c, Item::Reg(S6), 55, 0x47f490, 0x504710, 0x24c8b4)?;

        // Add every item to the page, title and space first.
        for i in 0..51 {
            add_item(c, Item::Slot(i), 0x0024_c8c4 + 0x10 * i as u32)?;
        }
        for (i, reg) in [
            (51u32, S4),
            (52, S5),
            (53, S3),
            (54, FP),
            (55, S2),
            (56, S7),
            (57, S6),
        ] {
            add_item(c, Item::Reg(reg), 0x0024_c8c4 + 0x10 * i)?;
        }

        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x150);
        frame.restore(c, S0, 0x160);
        frame.restore(c, S2, 0x140);
        frame.restore(c, S3, 0x130);
        frame.restore(c, S4, 0x120);
        frame.restore(c, S5, 0x110);
        frame.restore(c, S6, 0x100);
        frame.restore(c, S7, 0xf0);
        frame.restore(c, FP, 0xe0);
        frame.restore_ra(c, 0xd0);
        frame.restore_fpr(c, 25, 0x198);
        frame.restore_fpr(c, 24, 0x190);
        frame.restore_fpr(c, 23, 0x188);
        frame.restore_fpr(c, 22, 0x180);
        frame.restore_fpr(c, 21, 0x178);
        frame.restore_fpr(c, 20, 0x170);
        frame.close(c);
        Ok(())
    })
}

/// Offsets of the title (null) item and the spacer item in the menu.
pub(crate) const RTM_TITLE: u32 = 0x130;

pub(crate) const RTM_SPACE: u32 = 0x14c;

/// The argument of a toggle constructor call.
#[derive(Clone, Copy)]
pub(crate) enum RtmCall {
    /// `cIntMenuItem(item, index, &var, 0, max, name)` (0x2cd8f8).
    Int { max: u32, name: u32 },
    /// `cFloatMenuItem_ctor(item, index, &var, min, max, name)` (0x2cdce8):
    /// the bounds in f12/f13 (`Some` bit patterns of two floats), or the
    /// saved f21/f20 (`None`, -10.0 and 10.0), and the name in a3.
    Float {
        bounds: Option<(u32, u32)>,
        name: u32,
    },
}

/// One toggle item, in the order the constructor builds them. `stash` lists
/// the item addresses written to frame slots just before the call, as
/// `(slot, item offset)` with the slot at `sp + 4 * slot`.
#[derive(Clone, Copy)]
pub(crate) struct RtmToggle {
    item: u32,
    index: u32,
    var: u32,
    ret_addr: u32,
    stash: &'static [(u32, u32)],
    call: RtmCall,
}

/// The 47 toggle constructor calls (calls 3..=49 of the generated body).
/// `var` is the gp offset of the toggle's variable.
pub(crate) static RTM_TOGGLES: [RtmToggle; 47] = [
    RtmToggle {
        item: 0x164,
        index: 0,
        var: 0x13d0,
        ret_addr: 0x0024_7f3c,
        stash: &[(3, 0x194), (4, 0x1c4)],
        call: RtmCall::Int {
            max: 0x3c,
            name: 0x0047_dc10,
        },
    },
    RtmToggle {
        item: 0x194,
        index: 1,
        var: 0x13d4,
        ret_addr: 0x0024_7f6c,
        stash: &[(5, 0x1f4), (6, 0x224)],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_dc30,
        },
    },
    RtmToggle {
        item: 0x1c4,
        index: 2,
        var: 0x13d8,
        ret_addr: 0x0024_7f9c,
        stash: &[(7, 0x254), (8, 0x284)],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_dc48,
        },
    },
    RtmToggle {
        item: 0x1f4,
        index: 3,
        var: 0x13dc,
        ret_addr: 0x0024_7fcc,
        stash: &[(9, 0x2b4), (10, 0x2e4)],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_dc58,
        },
    },
    RtmToggle {
        item: 0x224,
        index: 4,
        var: 0x13e0,
        ret_addr: 0x0024_7ffc,
        stash: &[(11, 0x314), (12, 0x344)],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_dc70,
        },
    },
    RtmToggle {
        item: 0x254,
        index: 5,
        var: 0x13e4,
        ret_addr: 0x0024_802c,
        stash: &[(13, 0x374), (14, 0x3a4)],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_dc88,
        },
    },
    RtmToggle {
        item: 0x284,
        index: 6,
        var: 0x13e8,
        ret_addr: 0x0024_805c,
        stash: &[(15, 0x3d4), (16, 0x404)],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_dc98,
        },
    },
    RtmToggle {
        item: 0x2b4,
        index: 7,
        var: 0x13ec,
        ret_addr: 0x0024_808c,
        stash: &[(17, 0x434), (18, 0x464)],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_dcb8,
        },
    },
    RtmToggle {
        item: 0x2e4,
        index: 8,
        var: 0x13f0,
        ret_addr: 0x0024_80bc,
        stash: &[(19, 0x494), (20, 0x4c4)],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_dcd0,
        },
    },
    RtmToggle {
        item: 0x314,
        index: 9,
        var: 0x13f4,
        ret_addr: 0x0024_80ec,
        stash: &[(21, 0x4f4), (22, 0x524)],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_dce8,
        },
    },
    RtmToggle {
        item: 0x344,
        index: 10,
        var: 0x13f8,
        ret_addr: 0x0024_811c,
        stash: &[(23, 0x554), (24, 0x584)],
        call: RtmCall::Int {
            max: 3,
            name: 0x0047_dd00,
        },
    },
    RtmToggle {
        item: 0x374,
        index: 11,
        var: 0x13fc,
        ret_addr: 0x0024_814c,
        stash: &[(25, 0x5b4), (26, 0x5e4)],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_dd18,
        },
    },
    RtmToggle {
        item: 0x3a4,
        index: 12,
        var: 0x1400,
        ret_addr: 0x0024_817c,
        stash: &[(27, 0x614), (28, 0x644)],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_dd38,
        },
    },
    RtmToggle {
        item: 0x3d4,
        index: 13,
        var: 0x1404,
        ret_addr: 0x0024_81ac,
        stash: &[(29, 0x674), (30, 0x6a4)],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_dd48,
        },
    },
    RtmToggle {
        item: 0x404,
        index: 14,
        var: 0x1408,
        ret_addr: 0x0024_81dc,
        stash: &[(31, 0x6d4), (32, 0x704)],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_dd60,
        },
    },
    RtmToggle {
        item: 0x434,
        index: 15,
        var: 0x140c,
        ret_addr: 0x0024_820c,
        stash: &[(33, 0x734), (34, 0x764)],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_dd78,
        },
    },
    RtmToggle {
        item: 0x464,
        index: 16,
        var: 0x1410,
        ret_addr: 0x0024_823c,
        stash: &[(35, 0x794), (36, 0x7c4)],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_dd98,
        },
    },
    RtmToggle {
        item: 0x494,
        index: 17,
        var: 0x1414,
        ret_addr: 0x0024_826c,
        stash: &[(37, 0x7f8), (38, 0x828)],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_ddb8,
        },
    },
    RtmToggle {
        item: 0x4c4,
        index: 18,
        var: 0x1418,
        ret_addr: 0x0024_8294,
        stash: &[(39, 0x858)],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_ddc8,
        },
    },
    RtmToggle {
        item: 0x4f4,
        index: 19,
        var: 0x141c,
        ret_addr: 0x0024_82b4,
        stash: &[],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_dde0,
        },
    },
    RtmToggle {
        item: 0x524,
        index: 20,
        var: 0x1420,
        ret_addr: 0x0024_82dc,
        stash: &[(40, 0x8f0)],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_ddf8,
        },
    },
    RtmToggle {
        item: 0x554,
        index: 21,
        var: 0x1424,
        ret_addr: 0x0024_82fc,
        stash: &[],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_de10,
        },
    },
    RtmToggle {
        item: 0x584,
        index: 22,
        var: 0x1428,
        ret_addr: 0x0024_831c,
        stash: &[],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_de20,
        },
    },
    RtmToggle {
        item: 0x5b4,
        index: 23,
        var: 0x142c,
        ret_addr: 0x0024_833c,
        stash: &[],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_de38,
        },
    },
    RtmToggle {
        item: 0x5e4,
        index: 24,
        var: 0x1430,
        ret_addr: 0x0024_835c,
        stash: &[],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_de50,
        },
    },
    RtmToggle {
        item: 0x614,
        index: 25,
        var: 0x1434,
        ret_addr: 0x0024_837c,
        stash: &[],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_de70,
        },
    },
    RtmToggle {
        item: 0x644,
        index: 26,
        var: 0x1438,
        ret_addr: 0x0024_839c,
        stash: &[],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_de88,
        },
    },
    RtmToggle {
        item: 0x674,
        index: 27,
        var: 0x143c,
        ret_addr: 0x0024_83bc,
        stash: &[],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_dea0,
        },
    },
    RtmToggle {
        item: 0x6a4,
        index: 28,
        var: 0x1440,
        ret_addr: 0x0024_83dc,
        stash: &[],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_deb0,
        },
    },
    RtmToggle {
        item: 0x6d4,
        index: 29,
        var: 0x1444,
        ret_addr: 0x0024_83fc,
        stash: &[],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_ded0,
        },
    },
    RtmToggle {
        item: 0x704,
        index: 30,
        var: 0x1448,
        ret_addr: 0x0024_841c,
        stash: &[],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_dee8,
        },
    },
    RtmToggle {
        item: 0x734,
        index: 31,
        var: 0x144c,
        ret_addr: 0x0024_843c,
        stash: &[],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_df00,
        },
    },
    RtmToggle {
        item: 0x764,
        index: 32,
        var: 0x1450,
        ret_addr: 0x0024_845c,
        stash: &[],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_df28,
        },
    },
    RtmToggle {
        item: 0x794,
        index: 33,
        var: 0x1454,
        ret_addr: 0x0024_847c,
        stash: &[],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_df40,
        },
    },
    RtmToggle {
        item: 0x7c4,
        index: 34,
        var: 0x1458,
        ret_addr: 0x0024_84a4,
        stash: &[],
        call: RtmCall::Float {
            bounds: Some((0x3f80_0000, 0x44fa_0000)),
            name: 0x0047_df50,
        },
    },
    RtmToggle {
        item: 0x7f8,
        index: 35,
        var: 0x145c,
        ret_addr: 0x0024_84c4,
        stash: &[],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_df68,
        },
    },
    RtmToggle {
        item: 0x828,
        index: 36,
        var: 0x1460,
        ret_addr: 0x0024_84e4,
        stash: &[],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_df80,
        },
    },
    RtmToggle {
        item: 0x858,
        index: 37,
        var: 0x1464,
        ret_addr: 0x0024_8504,
        stash: &[],
        call: RtmCall::Int {
            max: 0x14,
            name: 0x0047_dfa0,
        },
    },
    RtmToggle {
        item: 0x888,
        index: 38,
        var: 0x1468,
        ret_addr: 0x0024_8524,
        stash: &[],
        call: RtmCall::Float {
            bounds: None,
            name: 0x004a_2cd0,
        },
    },
    RtmToggle {
        item: 0x8bc,
        index: 39,
        var: 0x146c,
        ret_addr: 0x0024_8544,
        stash: &[],
        call: RtmCall::Float {
            bounds: None,
            name: 0x004a_2cd8,
        },
    },
    RtmToggle {
        item: 0x8f0,
        index: 40,
        var: 0x1470,
        ret_addr: 0x0024_8564,
        stash: &[],
        call: RtmCall::Float {
            bounds: None,
            name: 0x004a_2ce0,
        },
    },
    RtmToggle {
        item: 0x924,
        index: 41,
        var: 0x1474,
        ret_addr: 0x0024_8584,
        stash: &[],
        call: RtmCall::Float {
            bounds: None,
            name: 0x004a_2ce8,
        },
    },
    RtmToggle {
        item: 0x958,
        index: 42,
        var: 0x1478,
        ret_addr: 0x0024_85a4,
        stash: &[],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_dfb0,
        },
    },
    RtmToggle {
        item: 0x988,
        index: 43,
        var: 0x147c,
        ret_addr: 0x0024_85c4,
        stash: &[],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_dfc8,
        },
    },
    RtmToggle {
        item: 0x9b8,
        index: 44,
        var: 0x1480,
        ret_addr: 0x0024_85e4,
        stash: &[],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_dfe8,
        },
    },
    RtmToggle {
        item: 0x9e8,
        index: 45,
        var: 0x1484,
        ret_addr: 0x0024_8604,
        stash: &[],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_e000,
        },
    },
    RtmToggle {
        item: 0xa18,
        index: 46,
        var: 0x1488,
        ret_addr: 0x0024_8624,
        stash: &[],
        call: RtmCall::Int {
            max: 1,
            name: 0x0047_e020,
        },
    },
];

/// `this + off` as the 64-bit register value the generated code computes.
pub(crate) fn rtm_item(this: u64, off: u32) -> u64 {
    sx((this as u32).wrapping_add(off))
}

/// `V0 = this + item; sw V0, sp + 4 * slot`: an item address stashed in the frame.
pub(crate) fn rtm_stash(c: &mut Cpu, frame: Frame, slot: u32, item: u32) {
    c.g[V0][0] = rtm_item(c.g[S0][0], item);
    let addr = frame.at(c, slot * 4);
    let v = c.g[V0][0];
    c.sw(addr, v);
}

/// `cIntMenuItem(item, index, &var, 0, max, name)`.
pub(crate) fn rtm_int(c: &mut Cpu, t: &RtmToggle, max: u32, name: u32) -> Flow {
    c.g[A0][0] = rtm_item(c.g[S0][0], t.item);
    c.g[A1][0] = t.index as u64;
    c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(t.var));
    c.g[A3][0] = 0;
    c.g[T0][0] = max as u64;
    c.g[T1][0] = sx(name);
    call_ptr(c, 0x002c_d8f8, t.ret_addr)
}

/// `cFloatMenuItem_ctor(item, index, &var, min, max, name)`: the bounds are
/// already in f12/f13; T0 and T1 keep their values, as in the generated body.
pub(crate) fn rtm_float(c: &mut Cpu, t: &RtmToggle, name: u32) -> Flow {
    c.g[A0][0] = rtm_item(c.g[S0][0], t.item);
    c.g[A1][0] = t.index as u64;
    c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(t.var));
    c.g[A3][0] = sx(name);
    call_ptr(c, 0x002c_dce8, t.ret_addr)
}

/// `MenuSysPage_InsertItem(this, item, -1)`.
pub(crate) fn rtm_insert(c: &mut Cpu, item: u32, ret_addr: u32) -> Flow {
    c.g[A1][0] = rtm_item(c.g[S0][0], item);
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = u64::MAX;
    call_ptr(c, 0x002c_bcc8, ret_addr)
}

/// `RenderTogglesMenu_ctor(RenderTogglesMenu *this)` (0x247e60): build the
/// debug render-toggles menu in `this`. It constructs the page
/// (`MenuSysPage_ctor`), the title and spacer items, then one `MenuSysItem`
/// per toggle (int, or float with bounds), each toggle's variable at a gp
/// offset, and finally inserts the 49 items into the page in order. Returns
/// `this` in v0.
///
/// The generated body writes the item addresses into frame slots (`sp + 4n`)
/// ahead of the constructors that read them back, and keeps the rest in
/// s1..s7/fp; the port writes the same slots before the same calls, so an
/// unwinding callee leaves the same stack.
pub fn render_toggles_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 708;
        let frame = Frame::open(c, 0x160);
        frame.save(c, S0, 0x140);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x130);
        frame.save(c, S2, 0x120);
        frame.save(c, S3, 0x110);
        frame.save(c, S4, 0x100);
        c.g[S2][0] = rtm_item(c.g[S0][0], 0x888);
        frame.save(c, S5, 0xf0);
        c.g[S3][0] = rtm_item(c.g[S0][0], 0x8bc);
        frame.save(c, S6, 0xe0);
        c.g[S5][0] = rtm_item(c.g[S0][0], 0x924);
        frame.save(c, S7, 0xd0);
        c.g[S4][0] = rtm_item(c.g[S0][0], 0x958);
        frame.save(c, FP, 0xc0);
        c.g[S7][0] = rtm_item(c.g[S0][0], 0x988);
        frame.save_ra(c, 0xb0);
        c.g[FP][0] = rtm_item(c.g[S0][0], 0x9b8);
        frame.save_fpr(c, 21, 0x158);
        c.g[S1][0] = rtm_item(c.g[S0][0], 0x9e8);
        frame.save_fpr(c, 20, 0x150);
        c.g[S6][0] = rtm_item(c.g[S0][0], 0xa18);

        // The page: MenuSysPage_ctor(this).
        rtm_stash(c, frame, 0, RTM_TITLE);
        call_ptr(c, 0x002c_aa58, 0x0024_7ec4)?;

        // Title item (null item, name "Render Toggles Menu") and vtable.
        c.g[AT][0] = 0xffff_ffff_c120_0000;
        c.mtc1(1, 21); // f21 = -10.0
        rtm_stash(c, frame, 1, RTM_SPACE);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x2408));
        c.g[AT][0] = 0x4120_0000;
        c.mtc1(1, 20); // f20 = 10.0
        c.g[V0][0] = sx(0x0048_0000u32.wrapping_sub(0x320)); // cRenderTogglesMenu_vtbl
        let vtbl_addr = (c.g[S0][0] as u32).wrapping_add(0x12c);
        let vtbl = c.g[V0][0];
        c.sw(vtbl_addr, vtbl);
        c.g[A0][0] = rtm_item(c.g[S0][0], RTM_TITLE);
        call_ptr(c, 0x002c_d2a0, 0x0024_7ef8)?;

        // Spacer item.
        c.g[A0][0] = rtm_item(c.g[S0][0], RTM_SPACE);
        rtm_stash(c, frame, 2, 0x164);
        c.g[A1][0] = 0x28;
        call_ptr(c, 0x002c_d350, 0x0024_7f0c)?;

        // The toggles, each with the frame slots the generated body fills
        // before its call.
        for t in RTM_TOGGLES.iter() {
            for &(slot, item) in t.stash {
                rtm_stash(c, frame, slot, item);
            }
            match t.call {
                RtmCall::Int { max, name } => {
                    rtm_int(c, t, max, name)?;
                }
                RtmCall::Float { bounds, name } => {
                    match bounds {
                        Some((lo, hi)) => {
                            c.g[AT][0] = lo as u64;
                            c.mtc1(1, 12);
                            c.g[AT][0] = hi as u64;
                            c.mtc1(1, 13);
                        }
                        None => {
                            c.mov_s(12, 21);
                            c.mov_s(13, 20);
                        }
                    }
                    rtm_float(c, t, name)?;
                }
            }
        }

        // Insert the 49 items into the page, in order.
        let order = core::iter::once(RTM_TITLE)
            .chain(core::iter::once(RTM_SPACE))
            .chain(RTM_TOGGLES.iter().map(|t| t.item));
        for (i, item) in order.enumerate() {
            rtm_insert(c, item, 0x0024_8634 + 0x10 * i as u32)?;
        }

        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x140);
        frame.restore(c, S1, 0x130);
        frame.restore(c, S2, 0x120);
        frame.restore(c, S3, 0x110);
        frame.restore(c, S4, 0x100);
        frame.restore(c, S5, 0xf0);
        frame.restore(c, S6, 0xe0);
        frame.restore(c, S7, 0xd0);
        frame.restore(c, FP, 0xc0);
        frame.restore_ra(c, 0xb0);
        frame.restore_fpr(c, 21, 0x158);
        frame.restore_fpr(c, 20, 0x150);
        frame.close(c);
        Ok(())
    })
}

/// `VisualEffectsMenus_StaticInitDestroy` (0x0024e8d8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn visual_effects_menus_static_init_destroy(c: &mut Cpu) {
    // no resume points: the generated body ignores `c.pc` and always starts at the
    // entry, so every pc runs the body below (the guard is not needed).
    guest(c, |c| {
        'b_24f694: {
            'b_24f690: {
                c.cycles += 13;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                c.g[V0][0] = 0xffff;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[A1][0] != c.g[V0][0];
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                if t {
                    break 'b_24f690;
                } else {
                    'b_24f270: {
                        c.cycles += 2;
                        let t = c.g[A0][0] == 0u64;
                        c.g[T5][0] = 0x4d0000;
                        if t {
                            break 'b_24f270;
                        } else {
                            c.cycles += 599;
                            c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x4fd4));
                            c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x4fd0));
                            c.g[T6][0] = 0x4d0000;
                            c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x4fcc));
                            c.g[T7][0] = 0x4d0000;
                            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4fc8));
                            c.g[V0][0] = sx((c.g[T7][0] as u32).wrapping_sub(0x6a70));
                            c.swc1(9, (c.g[T5][0] as u32).wrapping_sub(0x6a90));
                            c.g[S1][0] = 0x4d0000;
                            c.swc1(10, (c.g[T6][0] as u32).wrapping_sub(0x6a80));
                            c.g[S2][0] = 0x4d0000;
                            c.swc1(11, (c.g[T7][0] as u32).wrapping_sub(0x6a70));
                            c.g[S3][0] = 0x4d0000;
                            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x4fc4));
                            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6a40));
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4fc0));
                            c.g[S5][0] = 0x4d0000;
                            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4fbc));
                            c.g[S4][0] = 0x4d0000;
                            c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0xc));
                            c.g[T3][0] = 0x4d0000;
                            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4fb8));
                            c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x6aa0));
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 0);
                            c.g[A3][0] = sx((c.g[T5][0] as u32).wrapping_sub(0x6a90));
                            c.swc1(8, (c.g[S1][0] as u32).wrapping_sub(0x6a60));
                            c.g[T1][0] = sx((c.g[T6][0] as u32).wrapping_sub(0x6a80));
                            c.swc1(4, (c.g[S2][0] as u32).wrapping_sub(0x6a50));
                            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x6a60));
                            c.swc1(5, (c.g[S3][0] as u32).wrapping_sub(0x6a40));
                            c.g[T0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x6a50));
                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4fb4));
                            c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x6a30));
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4fb0));
                            c.g[T4][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x6a20));
                            c.swc1(2, (c.g[A0][0] as u32).wrapping_add(0xc));
                            c.g[T9][0] = 0x4d0000;
                            c.g[AT][0] = 0x3f000000;
                            c.mtc1(1, 6);
                            c.g[S1][0] = 0x4d0000;
                            c.swc1(0, (c.g[S5][0] as u32).wrapping_sub(0x6aa0));
                            c.g[T2][0] = sx((c.g[T9][0] as u32).wrapping_sub(0x6a10));
                            c.swc1(3, (c.g[S4][0] as u32).wrapping_sub(0x6a30));
                            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x69d0));
                            c.swc1(1, (c.g[T3][0] as u32).wrapping_sub(0x6a20));
                            c.g[S6][0] = 0x4d0000;
                            c.swc1(6, (c.g[A1][0] as u32).wrapping_add(0xc));
                            c.g[S2][0] = 0x4d0000;
                            c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x69c0));
                            c.lwc1(17, (c.g[GP][0] as u32).wrapping_sub(0x4fac));
                            c.g[T7][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x69e0));
                            c.lwc1(18, (c.g[GP][0] as u32).wrapping_sub(0x4fa8));
                            c.g[S3][0] = 0x4d0000;
                            c.sw((c.g[T1][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x69b0));
                            c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x4fa4));
                            c.g[FP][0] = 0x4d0000;
                            c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x4fa0));
                            c.g[S3][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x69a0));
                            c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x4));
                            c.g[T8][0] = 0x4d0000;
                            c.swc1(18, (c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[FP][0] = 0x4d0000;
                            c.swc1(13, (c.g[V1][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = 0x4d0000;
                            c.swc1(14, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6990));
                            c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x4f9c));
                            c.g[T5][0] = sx((c.g[T8][0] as u32).wrapping_sub(0x6a00));
                            c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x4f98));
                            c.g[S7][0] = 0x4d0000;
                            c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[T6][0] = sx((c.g[S7][0] as u32).wrapping_sub(0x69f0));
                            c.sw((c.g[T0][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[V1][0] = 0x4d0000;
                            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x4f94));
                            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6950));
                            c.swc1(15, (c.g[A0][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6930));
                            c.swc1(16, (c.g[A0][0] as u32).wrapping_add(0x8));
                            c.g[FP][0] = 0x4d0000;
                            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(12, (c.g[A2][0] as u32).wrapping_add(0x4));
                            c.g[S4][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6940));
                            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.g[T3][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6920));
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x4));
                            c.g[A3][0] = 0x4d0000;
                            c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6900));
                            c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x4));
                            c.g[T1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x68d0));
                            c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x8));
                            c.g[A3][0] = 0x4d0000;
                            c.lwc1(19, (c.g[GP][0] as u32).wrapping_sub(0x4f90));
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4f8c));
                            c.g[A2][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x68f0));
                            c.swc1(19, (c.g[T4][0] as u32).wrapping_add(0x4));
                            c.g[FP][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x68b0));
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4f88));
                            c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x68a0));
                            c.swc1(2, (c.g[T9][0] as u32).wrapping_sub(0x6a10));
                            c.g[A0][0] = 0x4d0000;
                            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x4f84));
                            c.swc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4f80));
                            c.swc1(8, (c.g[T7][0] as u32).wrapping_add(0xc));
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4f7c));
                            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4f78));
                            c.swc1(3, (c.g[T4][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
                            c.swc1(4, (c.g[T2][0] as u32).wrapping_add(0x4));
                            c.swc1(5, (c.g[T2][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[T8][0] as u32).wrapping_sub(0x6a00));
                            c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0xc));
                            c.swc1(0, (c.g[S7][0] as u32).wrapping_sub(0x69f0));
                            c.sw((c.g[T6][0] as u32).wrapping_add(0xc), 0u64);
                            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4f74));
                            c.swc1(0, (c.g[S6][0] as u32).wrapping_sub(0x69e0));
                            c.swc1(7, (c.g[T7][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x69d0));
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0xc));
                            c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6840));
                            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4f70));
                            c.g[A0][0] = 0x4d0000;
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4f6c));
                            c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x69c0));
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0xc));
                            c.g[T4][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6820));
                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4f68));
                            c.g[A0][0] = 0x4d0000;
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4f64));
                            c.g[T2][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6810));
                            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4f60));
                            c.g[A0][0] = 0x4d0000;
                            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4f5c));
                            c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x69b0));
                            c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x69a0));
                            c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[T6][0] as u32).wrapping_add(0x4), 0u64);
                            c.sw((c.g[T6][0] as u32).wrapping_add(0x8), 0u64);
                            c.swc1(8, (c.g[T7][0] as u32).wrapping_add(0x4));
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x67e0));
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x4), 0u64);
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[A0][0] = 0x4d0000;
                            c.g[S2][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x67c0));
                            c.sw((c.g[S3][0] as u32).wrapping_add(0x4), 0u64);
                            c.g[A0][0] = 0x4d0000;
                            c.sw((c.g[S3][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x67b0));
                            c.sw((c.g[S3][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[A0][0] = 0x4d0000;
                            c.g[S3][0] = 0x4d0000;
                            c.g[T7][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x67a0));
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6960));
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(6, (c.g[S3][0] as u32).wrapping_add(0xc));
                            c.g[T6][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6790));
                            c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
                            c.g[A0][0] = 0x4d0000;
                            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x4f58));
                            c.g[T5][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6780));
                            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4f54));
                            c.g[A0][0] = 0x4d0000;
                            c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x4f50));
                            c.swc1(1, (c.g[A0][0] as u32).wrapping_sub(0x6930));
                            c.g[A0][0] = 0x4d0000;
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4f4c));
                            c.g[S3][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6770));
                            c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x4f48));
                            c.g[A0][0] = 0x4d0000;
                            c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x4f44));
                            c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x6990));
                            c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0xc));
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x6980));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6980));
                            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x4f40));
                            c.swc1(6, (c.g[A0][0] as u32).wrapping_add(0xc));
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x6970));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6970));
                            c.swc1(6, (c.g[A0][0] as u32).wrapping_add(0xc));
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x6960));
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x6950));
                            c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6720));
                            c.g[V0][0] = 0x4d0000;
                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4f3c));
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x6940));
                            c.swc1(5, (c.g[S4][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = 0x4d0000;
                            c.sw((c.g[S4][0] as u32).wrapping_add(0xc), 0u64);
                            c.swc1(7, (c.g[V1][0] as u32).wrapping_add(0x4));
                            c.swc1(8, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[V1][0] = 0x4d0000;
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4f38));
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x6920));
                            c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0xc));
                            c.g[V1][0] = 0x4d0000;
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x6910));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6980));
                            c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = 0x4d0000;
                            c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6970));
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.swc1(6, (c.g[V1][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = 0x4d0000;
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
                            c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6960));
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = 0x4d0000;
                            c.swc1(6, (c.g[V1][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6910));
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = 0x4d0000;
                            c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x4));
                            c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                            c.swc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
                            c.swc1(1, (c.g[T1][0] as u32).wrapping_add(0xc));
                            c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
                            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4f34));
                            c.g[V0][0] = 0x4d0000;
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x6900));
                            c.swc1(4, (c.g[A1][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = 0x4d0000;
                            c.swc1(5, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[A1][0] as u32).wrapping_add(0xc), 0u64);
                            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x4f30));
                            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4f2c));
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x68f0));
                            c.swc1(6, (c.g[A2][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = c.g[V1][0];
                            c.swc1(7, (c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x68e0));
                            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x4f28));
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x68e0));
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
                            c.g[V1][0] = 0x4d0000;
                            c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x68d0));
                            c.g[V0][0] = 0x4d0000;
                            c.swc1(9, (c.g[T1][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = c.g[V0][0];
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x68c0));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x68c0));
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = 0x4d0000;
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x68b0));
                            c.g[V1][0] = 0x4d0000;
                            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = 0x4d0000;
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x68a0));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x68e0));
                            c.swc1(11, (c.g[A3][0] as u32).wrapping_add(0x8));
                            c.g[V1][0] = 0x4d0000;
                            c.swc1(10, (c.g[A3][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x68c0));
                            c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[A3][0] = 0x4d0000;
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4f24));
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x6890));
                            c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0x4));
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6890));
                            c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = 0x4d0000;
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6830));
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = 0x4d0000;
                            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
                            c.g[FP][0] = 0x4d0000;
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0xc));
                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4f20));
                            c.g[A3][0] = c.g[V1][0];
                            c.swc1(4, (c.g[FP][0] as u32).wrapping_sub(0x6850));
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6880));
                            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4f1c));
                            c.g[FP][0] = 0x4d0000;
                            c.swc1(3, (c.g[T0][0] as u32).wrapping_add(0xc));
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4f18));
                            c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
                            c.swc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = c.g[FP][0];
                            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4f14));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6870));
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x6880));
                            c.swc1(5, (c.g[A3][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = 0x4d0000;
                            c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
                            c.swc1(0, (c.g[FP][0] as u32).wrapping_sub(0x6870));
                            c.g[A3][0] = c.g[V1][0];
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6860));
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x6860));
                            c.g[FP][0] = 0x4d0000;
                            c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6850));
                            c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[V0][0] = 0x4d0000;
                            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x4f10));
                            c.g[V1][0] = 0x4d0000;
                            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4f0c));
                            c.g[A3][0] = c.g[V1][0];
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x6840));
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6830));
                            c.swc1(6, (c.g[T0][0] as u32).wrapping_add(0x4));
                            c.g[FP][0] = 0x4d0000;
                            c.swc1(7, (c.g[T0][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = 0x4d0000;
                            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x4f08));
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x6830));
                            c.swc1(8, (c.g[A3][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = 0x4d0000;
                            c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x4f04));
                            c.g[A3][0] = 0x4d0000;
                            c.swc1(0, (c.g[FP][0] as u32).wrapping_sub(0x6820));
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6880));
                            c.swc1(9, (c.g[T4][0] as u32).wrapping_add(0x8));
                            c.g[FP][0] = 0x4d0000;
                            c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x4f00));
                            c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6870));
                            c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x6810));
                            c.swc1(10, (c.g[T2][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = 0x4d0000;
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x6800));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6860));
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                            c.g[V1][0] = 0x4d0000;
                            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6850));
                            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
                            c.g[A3][0] = 0x4d0000;
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6830));
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[FP][0] = 0x4d0000;
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
                            c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6800));
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[V0][0] = 0x4d0000;
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x67f0));
                            c.swc1(0, (c.g[T4][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = 0x4d0000;
                            c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x8));
                            c.g[A3][0] = 0x4d0000;
                            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
                            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4efc));
                            c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0xc));
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4ef8));
                            c.swc1(2, (c.g[S0][0] as u32).wrapping_add(0xc));
                            c.swc1(1, (c.g[T7][0] as u32).wrapping_add(0xc));
                            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0xc));
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x67f0));
                            c.g[FP][0] = 0x4d0000;
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x67e0));
                            c.g[V0][0] = c.g[FP][0];
                            c.sw((c.g[S1][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x67d0));
                            c.swc1(0, (c.g[FP][0] as u32).wrapping_sub(0x67d0));
                            c.g[V1][0] = 0x4d0000;
                            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[A3][0] = 0x4d0000;
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x67c0));
                            c.g[FP][0] = 0x4d0000;
                            c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[V0][0] = 0x4d0000;
                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4ef4));
                            c.g[V1][0] = 0x4d0000;
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x67b0));
                            c.swc1(3, (c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[A3][0] = 0x4d0000;
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4ef0));
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6800));
                            c.swc1(0, (c.g[FP][0] as u32).wrapping_sub(0x67a0));
                            c.swc1(4, (c.g[T7][0] as u32).wrapping_add(0x8));
                            c.g[FP][0] = 0x4d0000;
                            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4eec));
                            c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x67f0));
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x6790));
                            c.swc1(5, (c.g[T6][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = 0x4d0000;
                            c.sw((c.g[T6][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x67d0));
                            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x4ee8));
                            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4ee4));
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x6780));
                            c.swc1(6, (c.g[T5][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = 0x1;
                            c.swc1(7, (c.g[T5][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                            c.swc1(12, (c.g[FP][0] as u32).wrapping_add(0x4));
                            c.g[A3][0] = 0x2;
                            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[FP][0] = 0x4d0000;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6750));
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                            c.g[S1][0] = 0x3;
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                            c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x4));
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[S2][0] = 0x4d0000;
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x4));
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x6760));
                            c.swc1(0, (c.g[T6][0] as u32).wrapping_add(0x4));
                            c.sw((c.g[T5][0] as u32).wrapping_add(0xc), 0u64);
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x1f08), c.g[V1][0]);
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x1f00), 0u64);
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x1f10), c.g[A3][0]);
                            c.g[V0][0] = c.g[V1][0] & 0xff;
                            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4ee0));
                            c.g[V0][0] = c.g[V0][0] | 0x100;
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x1f18), c.g[S1][0]);
                            c.g[V1][0] = c.g[A3][0] & 0xff;
                            c.g[A2][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x1f00));
                            c.g[A3][0] = 0x4d0000;
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4edc));
                            c.g[A1][0] = c.g[S1][0] & 0xff;
                            c.swc1(2, (c.g[S2][0] as u32).wrapping_add(0xc));
                            c.g[S1][0] = 0x4d0000;
                            c.swc1(1, (c.g[FP][0] as u32).wrapping_add(0xc));
                            c.g[V1][0] = c.g[V1][0] | 0x200;
                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4ed8));
                            c.g[A1][0] = c.g[A1][0] | 0x300;
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4ed4));
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x6770));
                            c.swc1(3, (c.g[S3][0] as u32).wrapping_add(0x4));
                            c.g[A3][0] = 0x4d0000;
                            c.swc1(4, (c.g[S3][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[S3][0] as u32).wrapping_add(0xc), 0u64);
                            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4ed0));
                            c.g[S3][0] = 0x4d0000;
                            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x4ecc));
                            c.swc1(0, (c.g[S1][0] as u32).wrapping_sub(0x6760));
                            c.swc1(5, (c.g[S2][0] as u32).wrapping_add(0x4));
                            c.g[S1][0] = c.g[A3][0];
                            c.swc1(6, (c.g[S2][0] as u32).wrapping_add(0x8));
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x6730));
                            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4ec8));
                            c.g[S2][0] = 0x4d0000;
                            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x4ec4));
                            c.swc1(0, (c.g[S2][0] as u32).wrapping_sub(0x6750));
                            c.swc1(7, (c.g[FP][0] as u32).wrapping_add(0x4));
                            c.g[S2][0] = 0x4;
                            c.swc1(8, (c.g[FP][0] as u32).wrapping_add(0x8));
                            c.g[FP][0] = c.g[S3][0];
                            c.swc1(0, (c.g[S3][0] as u32).wrapping_sub(0x6740));
                            c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6740));
                            c.g[S3][0] = 0x5;
                            c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x6730));
                            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[A3][0] = 0x9;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0xc), 0u64);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1f00), c.g[A2][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1f08), c.g[V0][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1f10), c.g[V1][0]);
                            c.g[V0][0] = 0x7;
                            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
                            c.g[V1][0] = 0x8;
                            c.sw((c.g[FP][0] as u32).wrapping_add(0x4), 0u64);
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[FP][0] = 0x6;
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1efc), 0u64);
                            c.g[S1][0] = 0xa;
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1ef8), 0u64);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1f18), c.g[A1][0]);
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x1f20), c.g[S2][0]);
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x1f28), c.g[S3][0]);
                            c.g[S2][0] = 0xb;
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x1f30), c.g[FP][0]);
                            c.g[S3][0] = 0xc;
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x1f38), c.g[V0][0]);
                            c.g[FP][0] = 0xd;
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x1f40), c.g[V1][0]);
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x1f48), c.g[A3][0]);
                            c.g[T2][0] = c.g[V0][0] & 0xff;
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x1f50), c.g[S1][0]);
                            c.g[T0][0] = c.g[V1][0] & 0xff;
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x1f58), c.g[S2][0]);
                            c.g[A3][0] = c.g[A3][0] & 0xff;
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x1f60), c.g[S3][0]);
                            c.g[A2][0] = c.g[S1][0] & 0xff;
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x1f68), c.g[FP][0]);
                            c.g[A1][0] = c.g[S2][0] & 0xff;
                            c.g[T1][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x1f20));
                            c.g[V1][0] = c.g[S3][0] & 0xff;
                            c.g[T3][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x1f28));
                            c.g[V0][0] = c.g[FP][0] & 0xff;
                            c.g[T4][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x1f30));
                            c.g[T1][0] = c.g[T1][0] | 0x400;
                            c.g[T3][0] = c.g[T3][0] | 0x500;
                            c.g[T2][0] = c.g[T2][0] | 0x700;
                            c.g[T4][0] = c.g[T4][0] | 0x600;
                            c.g[T0][0] = c.g[T0][0] | 0x800;
                            c.g[A3][0] = c.g[A3][0] | 0x900;
                            c.g[A2][0] = c.g[A2][0] | 0xa00;
                            c.g[A1][0] = c.g[A1][0] | 0xb00;
                            c.g[V1][0] = c.g[V1][0] | 0xc00;
                            c.g[V0][0] = c.g[V0][0] | 0xd00;
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1f20), c.g[T1][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1f28), c.g[T3][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1f30), c.g[T4][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1f38), c.g[T2][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1f40), c.g[T0][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1f48), c.g[A3][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1f50), c.g[A2][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1f58), c.g[A1][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1f60), c.g[V1][0]);
                            c.g[RA][0] = sx(0x0024f1a8);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1f68), c.g[V0][0]);
                            Callee {
                                addr: 0x00247e60,
                                f: ee::s06::RenderTogglesMenu_ctor,
                            }
                            .call(c, 0x0024f1a8)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f1b4);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5cd8));
                            Callee {
                                addr: 0x00248970,
                                f: ee::s06::VisualEffectsTestMenu_ctor,
                            }
                            .call(c, 0x0024f1b4)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f1c0);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5840));
                            Callee {
                                addr: 0x00249148,
                                f: ee::s06::LightGlowMenu_ctor,
                            }
                            .call(c, 0x0024f1c0)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f1cc);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5108));
                            Callee {
                                addr: 0x00248dc8,
                                f: ee::s06::PlantsMenu_ctor,
                            }
                            .call(c, 0x0024f1cc)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f1d8);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4d38));
                            Callee {
                                addr: 0x002498a0,
                                f: ee::s06::DepthFogMenu_ctor,
                            }
                            .call(c, 0x0024f1d8)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f1e4);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4418));
                            Callee {
                                addr: 0x00249b28,
                                f: ee::s06::HeightFogMenu_ctor,
                            }
                            .call(c, 0x0024f1e4)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f1f0);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4218));
                            Callee {
                                addr: 0x00249c68,
                                f: ee::s06::DetailSystemMenu_ctor,
                            }
                            .call(c, 0x0024f1f0)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f1fc);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3eb0));
                            Callee {
                                addr: 0x0024a228,
                                f: ee::s06::ScreenTintMenu_ctor,
                            }
                            .call(c, 0x0024f1fc)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f208);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3550));
                            Callee {
                                addr: 0x0024abb8,
                                f: ee::s06::ShadowMenu_ctor,
                            }
                            .call(c, 0x0024f208)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f214);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3220));
                            Callee {
                                addr: 0x0024ae90,
                                f: ee::s06::SnowSurfaceMenu_ctor,
                            }
                            .call(c, 0x0024f214)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f220);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2f58));
                            Callee {
                                addr: 0x0024b0e8,
                                f: ee::s06::BoardTrailMenu_ctor,
                            }
                            .call(c, 0x0024f220)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f22c);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2988));
                            Callee {
                                addr: 0x0024b6d0,
                                f: ee::s06::FlagsMenu_ctor,
                            }
                            .call(c, 0x0024f22c)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f238);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x27c0));
                            Callee {
                                addr: 0x0024b7d0,
                                f: ee::s06::PowerUpFXTogglesMenu_ctor,
                            }
                            .call(c, 0x0024f238)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f244);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2378));
                            Callee {
                                addr: 0x0024bbe8,
                                f: ee::s06::SnowfallTogglesMenu_ctor,
                            }
                            .call(c, 0x0024f244)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f250);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1358));
                            Callee {
                                addr: 0x0024bf60,
                                f: ee::s06::CameraSplashMenu_ctor,
                            }
                            .call(c, 0x0024f250)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f25c);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x11f0));
                            Callee {
                                addr: 0x0024cca0,
                                f: ee::s06::TubeMenu_ctor,
                            }
                            .call(c, 0x0024f25c)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f268);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1480));
                            Callee {
                                addr: 0x0024ceb0,
                                f: ee::s06::PathArrowMenu_ctor,
                            }
                            .call(c, 0x0024f268)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                            if t {
                                break 'b_24f694;
                            } else {
                                break 'b_24f270;
                            }
                        }
                    }
                    c.cycles += 264;
                    c.g[A0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[RA][0] = sx(0x0024f280);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1480));
                    Callee {
                        addr: 0x0024e7c8,
                        f: ee::s06::PathArrowMenu_dtor,
                    }
                    .call(c, 0x0024f280)?;
                    c.g[S0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x11f0));
                    c.g[RA][0] = sx(0x0024f294);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x25c));
                    call_ptr(c, 0x002ca280, 0x0024f294)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x22c));
                    c.g[RA][0] = sx(0x0024f2a0);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f2a0)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1fc));
                    c.g[RA][0] = sx(0x0024f2ac);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f2ac)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c8));
                    c.g[RA][0] = sx(0x0024f2b8);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f2b8)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x194));
                    c.g[RA][0] = sx(0x0024f2c4);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f2c4)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x164));
                    c.g[RA][0] = sx(0x0024f2d0);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f2d0)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c));
                    c.g[RA][0] = sx(0x0024f2dc);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f2dc)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x130));
                    c.g[RA][0] = sx(0x0024f2e8);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f2e8)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0024f2f4);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002caa80, 0x0024f2f4)?;
                    c.g[A0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[RA][0] = sx(0x0024f304);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1358));
                    Callee {
                        addr: 0x0024e388,
                        f: ee::s06::CameraSplashMenu_dtor,
                    }
                    .call(c, 0x0024f304)?;
                    c.g[A0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[RA][0] = sx(0x0024f314);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2378));
                    Callee {
                        addr: 0x0024e200,
                        f: ee::s06::SnowfallTogglesMenu_dtor,
                    }
                    .call(c, 0x0024f314)?;
                    c.g[A0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[RA][0] = sx(0x0024f324);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x27c0));
                    Callee {
                        addr: 0x0024e0f8,
                        f: ee::s06::PowerUpFXTogglesMenu_dtor,
                    }
                    .call(c, 0x0024f324)?;
                    c.g[S0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x2988));
                    c.g[RA][0] = sx(0x0024f338);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x194));
                    call_ptr(c, 0x002ca280, 0x0024f338)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x164));
                    c.g[RA][0] = sx(0x0024f344);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f344)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c));
                    c.g[RA][0] = sx(0x0024f350);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f350)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x130));
                    c.g[RA][0] = sx(0x0024f35c);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f35c)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0024f368);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002caa80, 0x0024f368)?;
                    c.g[A0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[RA][0] = sx(0x0024f378);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2f58));
                    Callee {
                        addr: 0x0024dfa0,
                        f: ee::s06::BoardTrailMenu_dtor,
                    }
                    .call(c, 0x0024f378)?;
                    c.g[S0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x3220));
                    c.g[RA][0] = sx(0x0024f38c);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x294));
                    call_ptr(c, 0x002ca280, 0x0024f38c)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x260));
                    c.g[RA][0] = sx(0x0024f398);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f398)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x22c));
                    c.g[RA][0] = sx(0x0024f3a4);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f3a4)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1f8));
                    c.g[RA][0] = sx(0x0024f3b0);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f3b0)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c4));
                    c.g[RA][0] = sx(0x0024f3bc);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f3bc)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x194));
                    c.g[RA][0] = sx(0x0024f3c8);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f3c8)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x164));
                    c.g[RA][0] = sx(0x0024f3d4);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f3d4)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c));
                    c.g[RA][0] = sx(0x0024f3e0);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f3e0)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x130));
                    c.g[RA][0] = sx(0x0024f3ec);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f3ec)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0024f3f8);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002caa80, 0x0024f3f8)?;
                    c.g[S0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x3550));
                    c.g[RA][0] = sx(0x0024f40c);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2fc));
                    call_ptr(c, 0x002ca280, 0x0024f40c)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2c8));
                    c.g[RA][0] = sx(0x0024f418);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f418)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x294));
                    c.g[RA][0] = sx(0x0024f424);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f424)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x260));
                    c.g[RA][0] = sx(0x0024f430);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f430)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x22c));
                    c.g[RA][0] = sx(0x0024f43c);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f43c)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1f8));
                    c.g[RA][0] = sx(0x0024f448);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f448)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c4));
                    c.g[RA][0] = sx(0x0024f454);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f454)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x194));
                    c.g[RA][0] = sx(0x0024f460);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f460)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x164));
                    c.g[RA][0] = sx(0x0024f46c);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f46c)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c));
                    c.g[RA][0] = sx(0x0024f478);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f478)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x130));
                    c.g[RA][0] = sx(0x0024f484);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f484)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0024f490);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002caa80, 0x0024f490)?;
                    c.g[A0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[RA][0] = sx(0x0024f4a0);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3eb0));
                    Callee {
                        addr: 0x0024dd70,
                        f: ee::s06::ScreenTintMenu_dtor,
                    }
                    .call(c, 0x0024f4a0)?;
                    c.g[S0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x4218));
                    c.g[RA][0] = sx(0x0024f4b4);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x334));
                    call_ptr(c, 0x002ca280, 0x0024f4b4)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x300));
                    c.g[RA][0] = sx(0x0024f4c0);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f4c0)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2cc));
                    c.g[RA][0] = sx(0x0024f4cc);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f4cc)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x298));
                    c.g[RA][0] = sx(0x0024f4d8);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f4d8)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x264));
                    c.g[RA][0] = sx(0x0024f4e4);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f4e4)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x230));
                    c.g[RA][0] = sx(0x0024f4f0);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f4f0)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1fc));
                    c.g[RA][0] = sx(0x0024f4fc);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f4fc)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c8));
                    c.g[RA][0] = sx(0x0024f508);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f508)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x194));
                    c.g[RA][0] = sx(0x0024f514);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f514)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x164));
                    c.g[RA][0] = sx(0x0024f520);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f520)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c));
                    c.g[RA][0] = sx(0x0024f52c);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f52c)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x130));
                    c.g[RA][0] = sx(0x0024f538);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f538)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0024f544);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002caa80, 0x0024f544)?;
                    c.g[S0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x4418));
                    c.g[RA][0] = sx(0x0024f558);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c8));
                    call_ptr(c, 0x002ca280, 0x0024f558)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x194));
                    c.g[RA][0] = sx(0x0024f564);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f564)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x164));
                    c.g[RA][0] = sx(0x0024f570);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f570)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c));
                    c.g[RA][0] = sx(0x0024f57c);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f57c)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x130));
                    c.g[RA][0] = sx(0x0024f588);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f588)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0024f594);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002caa80, 0x0024f594)?;
                    c.g[A0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[RA][0] = sx(0x0024f5a4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4d38));
                    Callee {
                        addr: 0x0024dc68,
                        f: ee::s06::DepthFogMenu_dtor,
                    }
                    .call(c, 0x0024f5a4)?;
                    c.g[S0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x5108));
                    c.g[RA][0] = sx(0x0024f5b8);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x398));
                    call_ptr(c, 0x002ca280, 0x0024f5b8)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x364));
                    c.g[RA][0] = sx(0x0024f5c4);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f5c4)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x330));
                    c.g[RA][0] = sx(0x0024f5d0);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f5d0)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2fc));
                    c.g[RA][0] = sx(0x0024f5dc);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f5dc)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2c8));
                    c.g[RA][0] = sx(0x0024f5e8);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f5e8)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x298));
                    c.g[RA][0] = sx(0x0024f5f4);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f5f4)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x264));
                    c.g[RA][0] = sx(0x0024f600);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f600)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x230));
                    c.g[RA][0] = sx(0x0024f60c);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f60c)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1fc));
                    c.g[RA][0] = sx(0x0024f618);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f618)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c8));
                    c.g[RA][0] = sx(0x0024f624);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f624)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x194));
                    c.g[RA][0] = sx(0x0024f630);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f630)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x164));
                    c.g[RA][0] = sx(0x0024f63c);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f63c)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c));
                    c.g[RA][0] = sx(0x0024f648);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f648)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x130));
                    c.g[RA][0] = sx(0x0024f654);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f654)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0024f660);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002caa80, 0x0024f660)?;
                    c.g[A0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[RA][0] = sx(0x0024f670);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5840));
                    Callee {
                        addr: 0x0024dab8,
                        f: ee::s06::LightGlowMenu_dtor,
                    }
                    .call(c, 0x0024f670)?;
                    c.g[A0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[RA][0] = sx(0x0024f680);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5cd8));
                    Callee {
                        addr: 0x0024d998,
                        f: ee::s06::VisualEffectsTestMenu_dtor,
                    }
                    .call(c, 0x0024f680)?;
                    c.g[A0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[RA][0] = sx(0x0024f690);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6720));
                    Callee {
                        addr: 0x0024d710,
                        f: ee::s06::RenderTogglesMenu_dtor,
                    }
                    .call(c, 0x0024f690)?;
                    break 'b_24f690;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            break 'b_24f694;
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        return Ok(());
    });
}

/// Offsets of the menu's items and vtable in the `cLightGlowMenu` object.
pub(crate) const LGM_VTBL: u32 = 0x12c;

pub(crate) const LGM_TITLE: u32 = 0x130;

pub(crate) const LGM_SPACE: u32 = 0x14c;

pub(crate) const LGM_ENABLE: u32 = 0x164;

pub(crate) const LGM_OVERRIDE_WP: u32 = 0x194;

pub(crate) const LGM_PS2_CAPTURE_SIZE: u32 = 0x1c4;

pub(crate) const LGM_PS2_MIN_INTENSITY: u32 = 0x1f4;

pub(crate) const LGM_PS2_MOD_INTENSITY: u32 = 0x228;

pub(crate) const LGM_PS2_COPY_INTENSITY: u32 = 0x25c;

pub(crate) const LGM_PS2_FRAME_SRC: u32 = 0x290;

pub(crate) const LGM_PS2_FRAME_DST: u32 = 0x2c4;

pub(crate) const LGM_PS2_TEX0: u32 = 0x2f8;

pub(crate) const LGM_PS2_TEX1: u32 = 0x32c;

pub(crate) const LGM_PS2_TEX2: u32 = 0x360;

pub(crate) const LGM_PS2_TEX3: u32 = 0x394;

pub(crate) const LGM_PS2_JITTER_TEX: u32 = 0x3c8;

pub(crate) const LGM_PS2_SHOW_TEXTURES: u32 = 0x3fc;

pub(crate) const LGM_SIZE: u32 = 0x42c;

pub(crate) const LGM_CUTOFF: u32 = 0x460;

pub(crate) const LGM_COLOUR_POWER: u32 = 0x494;

pub(crate) const LGM_COLOUR_SCALE: u32 = 0x4c8;

pub(crate) const LGM_COLOUR_AMBIENT: u32 = 0x4fc;

pub(crate) const LGM_SIZE_POWER: u32 = 0x530;

pub(crate) const LGM_SIZE_SCALE: u32 = 0x564;

pub(crate) const LGM_SIZE_AMBIENT: u32 = 0x598;

pub(crate) const LGM_FADE_RATE: u32 = 0x5cc;

pub(crate) const LGM_CLEAR_ALPHA: u32 = 0x600;

pub(crate) const LGM_INTENSITY_R: u32 = 0x630;

pub(crate) const LGM_INTENSITY_G: u32 = 0x664;

pub(crate) const LGM_INTENSITY_B: u32 = 0x698;

pub(crate) const LGM_INTENSITY_A: u32 = 0x6cc;

pub(crate) const LGM_INTENSITY_SCALE: u32 = 0x700;

/// The saved float registers the constructor fills once and reuses as the
/// bounds of its items: f20 = 0.0, f21 = 2.0, f22 = 1.0, f23 = 64.0,
/// f24 = 4.0, f25 = 8.0.
pub(crate) const LGM_F_ZERO: usize = 20;

pub(crate) const LGM_F_TWO: usize = 21;

pub(crate) const LGM_F_ONE: usize = 22;

pub(crate) const LGM_F_SIXTY_FOUR: usize = 23;

pub(crate) const LGM_F_FOUR: usize = 24;

pub(crate) const LGM_F_EIGHT: usize = 25;

/// Where the generated body reads an item address from: a frame slot the
/// previous step stored, or a callee-saved register that holds it.
#[derive(Clone, Copy)]
pub(crate) enum LgmSrc {
    Slot(u32),
    Reg(usize),
}

/// `this + off` as the 64-bit value the generated code keeps in a register.
pub(crate) fn lgm_item(this: u64, off: u32) -> u64 {
    sx((this as u32).wrapping_add(off))
}

/// `V0 = this + off; sw V0, sp + slot`: an item address kept in the frame.
pub(crate) fn lgm_stash(c: &mut Cpu, frame: Frame, slot: u32, off: u32) {
    c.g[V0][0] = lgm_item(c.g[S0][0], off);
    let addr = frame.at(c, slot);
    let v = c.g[V0][0];
    c.sw(addr, v);
}

/// The item address a call reads from `src`.
pub(crate) fn lgm_value(c: &mut Cpu, frame: Frame, src: LgmSrc) -> u64 {
    match src {
        LgmSrc::Slot(off) => {
            let addr = frame.at(c, off);
            c.lw(addr)
        }
        LgmSrc::Reg(r) => c.g[r][0],
    }
}

/// `cIntMenuItem(item, index, &var, min, max, name)` (0x2cd8f8): the item
/// address in a0, the gp variable at `var`, min in a3, max in t0, the name
/// in t1. Returns to `ret_addr`.
pub(crate) fn lgm_int(
    c: &mut Cpu,
    item: u64,
    index: u32,
    var: u32,
    min: u64,
    max: u64,
    name: u32,
    ret_addr: u32,
) -> Flow {
    c.g[A0][0] = item;
    c.g[A1][0] = index as u64;
    c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(var));
    c.g[A3][0] = min;
    c.g[T0][0] = max;
    c.g[T1][0] = sx(name);
    call_ptr(c, 0x002c_d8f8, ret_addr)
}

/// `cFloatMenuItem_ctor(item, index, &var, min, max, name)` (0x2cdce8): the
/// minimum is f20 (0.0); the maximum is the saved float register `max`, or
/// f13 as the caller already loaded it (`None`). The name goes in a3. t0 and
/// t1 keep their values, as in the generated body.
pub(crate) fn lgm_float(
    c: &mut Cpu,
    item: u64,
    index: u32,
    var: u32,
    name: u32,
    max: Option<usize>,
    ret_addr: u32,
) -> Flow {
    c.g[A0][0] = item;
    c.mov_s(12, LGM_F_ZERO);
    if let Some(max) = max {
        c.mov_s(13, max);
    }
    c.g[A3][0] = sx(name);
    c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(var));
    c.g[A1][0] = index as u64;
    call_ptr(c, 0x002c_dce8, ret_addr)
}

/// `MenuSysPage_InsertItem(this, item, -1)` (0x2cbcc8).
pub(crate) fn lgm_add(c: &mut Cpu, item: u64, ret_addr: u32) -> Flow {
    c.g[A1][0] = item;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = u64::MAX;
    call_ptr(c, 0x002c_bcc8, ret_addr)
}

/// `cLightGlowMenu_cLightGlowMenu(this)` (0x249148): the constructor of the
/// light-glow debug menu. It builds the page (`MenuSysPage_ctor`), the
/// title (a null item) and the spacer, then the switches (`cIntMenuItem`)
/// and the scales (`cFloatMenuItem_ctor`), each bound to its gp variable.
/// It sets the vtable and inserts the 31 items into the page in order.
/// Returns `this` in v0.
///
/// The generated body passes each item's address to the constructor that
/// follows it through a frame slot (`sp + 4 * n`), and keeps the item
/// addresses that are used late in s1..s7/fp. The port writes the same
/// slots before the same calls, so an unwinding callee leaves the same
/// stack. Its straight-line sequence has no branches; the only exits are
/// the unwind returns from the calls.
pub fn light_glow_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 470;
        let frame = Frame::open(c, 0x130);
        frame.save(c, S0, 0xf0);
        frame.save(c, S1, 0xe0);
        frame.save(c, S2, 0xd0);
        frame.save(c, S3, 0xc0);
        frame.save(c, S4, 0xb0);
        frame.save(c, S5, 0xa0);
        frame.save(c, S6, 0x90);
        frame.save(c, S7, 0x80);
        frame.save(c, FP, 0x70);
        frame.save_ra(c, 0x60);
        frame.save_fpr(c, 25, 0x128);
        frame.save_fpr(c, 24, 0x120);
        frame.save_fpr(c, 23, 0x118);
        frame.save_fpr(c, 22, 0x110);
        frame.save_fpr(c, 21, 0x108);
        frame.save_fpr(c, 20, 0x100);

        // The callee-saved item addresses used by the late calls.
        c.g[S0][0] = c.g[A0][0];
        c.g[S2][0] = lgm_item(c.g[S0][0], LGM_CLEAR_ALPHA);
        c.g[S3][0] = lgm_item(c.g[S0][0], LGM_INTENSITY_G);
        c.g[S4][0] = lgm_item(c.g[S0][0], LGM_INTENSITY_B);
        c.g[S6][0] = lgm_item(c.g[S0][0], LGM_SIZE_AMBIENT);
        c.g[S7][0] = lgm_item(c.g[S0][0], LGM_INTENSITY_R);
        c.g[FP][0] = lgm_item(c.g[S0][0], LGM_SIZE_POWER);
        c.g[S1][0] = lgm_item(c.g[S0][0], LGM_INTENSITY_A);
        c.g[S5][0] = lgm_item(c.g[S0][0], LGM_INTENSITY_SCALE);

        // The page: MenuSysPage_ctor(this), with the title item in slot 0.
        lgm_stash(c, frame, 0, LGM_TITLE);
        call_ptr(c, 0x002c_aa58, 0x0024_91bc)?;

        // Title: a null item named "Light Glow Menu"; then the vtable.
        c.mtc1(0, LGM_F_ZERO); // f20 = 0.0
        lgm_stash(c, frame, 4, LGM_SPACE);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x1dd8));
        c.g[V0][0] = 0x0048_0000;
        c.g[AT][0] = 0x4000_0000;
        c.mtc1(1, LGM_F_TWO); // f21 = 2.0
        c.g[V0][0] = sx(0x0048_0000u32.wrapping_sub(0x428)); // cLightGlowMenu_vtbl
        c.g[AT][0] = 0x4100_0000;
        c.mtc1(1, LGM_F_EIGHT); // f25 = 8.0
        let vtbl_addr = (c.g[S0][0] as u32).wrapping_add(LGM_VTBL);
        let vtbl = c.g[V0][0];
        c.sw(vtbl_addr, vtbl);
        c.g[AT][0] = 0x4280_0000;
        c.mtc1(1, LGM_F_SIXTY_FOUR); // f23 = 64.0
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, LGM_F_ONE); // f22 = 1.0
        let title = lgm_value(c, frame, LgmSrc::Slot(0));
        c.g[A0][0] = title;
        call_ptr(c, 0x002c_d2a0, 0x0024_9204)?; // cNullMenuItem(title, name)

        // Spacer: a space item of height 40.
        c.g[AT][0] = 0x4080_0000;
        c.mtc1(1, LGM_F_FOUR); // f24 = 4.0
        let space = lgm_value(c, frame, LgmSrc::Slot(4));
        c.g[A0][0] = space;
        lgm_stash(c, frame, 8, LGM_ENABLE);
        c.g[A1][0] = 0x28;
        call_ptr(c, 0x002c_d350, 0x0024_9220)?; // cSpaceMenuItem(space, 40)

        // Enable (int 0..1).
        let item = lgm_value(c, frame, LgmSrc::Slot(8));
        lgm_stash(c, frame, 0xc, LGM_OVERRIDE_WP);
        lgm_stash(c, frame, 0x10, LGM_PS2_CAPTURE_SIZE);
        lgm_int(c, item, 0, 0x12d8, 0, 1, 0x004a_2d30, 0x0024_9250)?;

        // Override world painter (int 0..1).
        let item = lgm_value(c, frame, LgmSrc::Slot(0xc));
        lgm_stash(c, frame, 0x14, LGM_PS2_MIN_INTENSITY);
        lgm_int(c, item, 1, 0x12dc, 0, 1, 0x0047_e238, 0x0024_9278)?;

        // PS2 capture size (int 4..8).
        let item = lgm_value(c, frame, LgmSrc::Slot(0x10));
        lgm_stash(c, frame, 0x18, LGM_PS2_MOD_INTENSITY);
        lgm_int(c, item, 2, 0x12e0, 4, 8, 0x0047_e250, 0x0024_92a0)?;

        // PS2 minimum intensity cutoff (0..2).
        let item = lgm_value(c, frame, LgmSrc::Slot(0x14));
        lgm_stash(c, frame, 0x1c, LGM_PS2_COPY_INTENSITY);
        lgm_stash(c, frame, 0x20, LGM_PS2_FRAME_SRC);
        lgm_float(
            c,
            item,
            3,
            0x12e4,
            0x0047_e268,
            Some(LGM_F_TWO),
            0x0024_92d0,
        )?;

        // PS2 post-cutoff scale (0..2).
        let item = lgm_value(c, frame, LgmSrc::Slot(0x18));
        lgm_stash(c, frame, 0x24, LGM_PS2_FRAME_DST);
        lgm_stash(c, frame, 0x28, LGM_PS2_TEX0);
        lgm_float(
            c,
            item,
            4,
            0x12e8,
            0x0047_e288,
            Some(LGM_F_TWO),
            0x0024_9300,
        )?;

        // PS2 copy intensity (0..8).
        let item = lgm_value(c, frame, LgmSrc::Slot(0x1c));
        lgm_stash(c, frame, 0x2c, LGM_PS2_TEX1);
        lgm_stash(c, frame, 0x30, LGM_PS2_TEX2);
        lgm_float(
            c,
            item,
            5,
            0x12ec,
            0x0047_e2a0,
            Some(LGM_F_EIGHT),
            0x0024_9330,
        )?;

        // PS2 frame source and blend intensities (0..2).
        let item = lgm_value(c, frame, LgmSrc::Slot(0x20));
        lgm_stash(c, frame, 0x34, LGM_PS2_TEX3);
        lgm_stash(c, frame, 0x38, LGM_PS2_JITTER_TEX);
        lgm_float(
            c,
            item,
            6,
            0x12f0,
            0x0047_e2b8,
            Some(LGM_F_TWO),
            0x0024_9360,
        )?;

        // PS2 frame blend intensity (0..2).
        let item = lgm_value(c, frame, LgmSrc::Slot(0x24));
        lgm_stash(c, frame, 0x3c, LGM_PS2_SHOW_TEXTURES);
        lgm_float(
            c,
            item,
            7,
            0x12f4,
            0x0047_e2d8,
            Some(LGM_F_TWO),
            0x0024_9388,
        )?;

        // PS2 blend textures 0..3 (0..2).
        let item = lgm_value(c, frame, LgmSrc::Slot(0x28));
        lgm_stash(c, frame, 0x40, LGM_SIZE);
        lgm_float(
            c,
            item,
            8,
            0x12f8,
            0x0047_e2f8,
            Some(LGM_F_TWO),
            0x0024_93b0,
        )?;
        let item = lgm_value(c, frame, LgmSrc::Slot(0x2c));
        lgm_stash(c, frame, 0x44, LGM_CUTOFF);
        lgm_float(
            c,
            item,
            9,
            0x12fc,
            0x0047_e310,
            Some(LGM_F_TWO),
            0x0024_93d8,
        )?;
        let item = lgm_value(c, frame, LgmSrc::Slot(0x30));
        lgm_stash(c, frame, 0x48, LGM_COLOUR_POWER);
        lgm_stash(c, frame, 0x4c, LGM_COLOUR_SCALE);
        lgm_float(
            c,
            item,
            10,
            0x1300,
            0x0047_e328,
            Some(LGM_F_TWO),
            0x0024_9408,
        )?;
        let item = lgm_value(c, frame, LgmSrc::Slot(0x34));
        lgm_stash(c, frame, 0x50, LGM_COLOUR_AMBIENT);
        lgm_float(
            c,
            item,
            11,
            0x1304,
            0x0047_e340,
            Some(LGM_F_TWO),
            0x0024_9430,
        )?;

        // PS2 texture jitter (0..8).
        let item = lgm_value(c, frame, LgmSrc::Slot(0x38));
        lgm_stash(c, frame, 0x54, LGM_SIZE_SCALE);
        lgm_float(
            c,
            item,
            12,
            0x1308,
            0x0047_e358,
            Some(LGM_F_EIGHT),
            0x0024_9458,
        )?;

        // PS2 show textures (int 0..1).
        let item = lgm_value(c, frame, LgmSrc::Slot(0x3c));
        lgm_stash(c, frame, 0x58, LGM_FADE_RATE);
        lgm_int(c, item, 13, 0x130c, 0, 1, 0x0047_e370, 0x0024_9480)?;

        // Size, cutoff, colour and size scales (floats).
        let item = lgm_value(c, frame, LgmSrc::Slot(0x40));
        lgm_float(
            c,
            item,
            14,
            0x1310,
            0x004a_2d38,
            Some(LGM_F_SIXTY_FOUR),
            0x0024_94a0,
        )?;
        let item = lgm_value(c, frame, LgmSrc::Slot(0x44));
        lgm_float(
            c,
            item,
            15,
            0x1314,
            0x004a_2d40,
            Some(LGM_F_ONE),
            0x0024_94c0,
        )?;
        let item = lgm_value(c, frame, LgmSrc::Slot(0x48));
        lgm_float(
            c,
            item,
            16,
            0x1318,
            0x0047_e388,
            Some(LGM_F_FOUR),
            0x0024_94e0,
        )?;
        let item = lgm_value(c, frame, LgmSrc::Slot(0x4c));
        lgm_float(
            c,
            item,
            17,
            0x131c,
            0x0047_e398,
            Some(LGM_F_SIXTY_FOUR),
            0x0024_9500,
        )?;
        // Colour ambient (0..16): the maximum is loaded into f13 here.
        let item = lgm_value(c, frame, LgmSrc::Slot(0x50));
        c.g[AT][0] = 0x4180_0000;
        c.mtc1(1, 13); // f13 = 16.0
        lgm_float(c, item, 18, 0x1320, 0x0047_e3a8, None, 0x0024_9524)?;
        let item = c.g[FP][0];
        lgm_float(
            c,
            item,
            19,
            0x1324,
            0x0047_e3b8,
            Some(LGM_F_FOUR),
            0x0024_9544,
        )?;
        let item = lgm_value(c, frame, LgmSrc::Slot(0x54));
        lgm_float(
            c,
            item,
            20,
            0x1328,
            0x0047_e3c8,
            Some(LGM_F_SIXTY_FOUR),
            0x0024_9564,
        )?;
        let item = c.g[S6][0];
        lgm_float(
            c,
            item,
            21,
            0x132c,
            0x0047_e3d8,
            Some(LGM_F_SIXTY_FOUR),
            0x0024_9584,
        )?;
        let item = lgm_value(c, frame, LgmSrc::Slot(0x58));
        lgm_float(
            c,
            item,
            22,
            0x1330,
            0x0047_e3e8,
            Some(LGM_F_ONE),
            0x0024_95a4,
        )?;

        // Clear (background) alpha (int 0..255).
        let item = c.g[S2][0];
        lgm_int(c, item, 23, 0x1334, 0, 0xff, 0x0047_e3f8, 0x0024_95c4)?;

        // Intensity red, green, blue, alpha and scale (floats 0..1).
        let item = c.g[S7][0];
        lgm_float(
            c,
            item,
            24,
            0x1338,
            0x0047_e410,
            Some(LGM_F_ONE),
            0x0024_95e4,
        )?;
        let item = c.g[S3][0];
        lgm_float(
            c,
            item,
            25,
            0x133c,
            0x0047_e420,
            Some(LGM_F_ONE),
            0x0024_9604,
        )?;
        let item = c.g[S4][0];
        lgm_float(
            c,
            item,
            26,
            0x1340,
            0x0047_e430,
            Some(LGM_F_ONE),
            0x0024_9624,
        )?;
        let item = c.g[S1][0];
        lgm_float(
            c,
            item,
            27,
            0x1344,
            0x0047_e440,
            Some(LGM_F_ONE),
            0x0024_9644,
        )?;
        let item = c.g[S5][0];
        lgm_float(
            c,
            item,
            28,
            0x1348,
            0x0047_e450,
            Some(LGM_F_ONE),
            0x0024_9664,
        )?;

        // Insert the 31 items into the page, in the order the constructor
        // made them: title, spacer, then the toggles and scales.
        let order = [
            LgmSrc::Slot(0x00),
            LgmSrc::Slot(0x04),
            LgmSrc::Slot(0x08),
            LgmSrc::Slot(0x0c),
            LgmSrc::Slot(0x10),
            LgmSrc::Slot(0x14),
            LgmSrc::Slot(0x18),
            LgmSrc::Slot(0x1c),
            LgmSrc::Slot(0x20),
            LgmSrc::Slot(0x24),
            LgmSrc::Slot(0x28),
            LgmSrc::Slot(0x2c),
            LgmSrc::Slot(0x30),
            LgmSrc::Slot(0x34),
            LgmSrc::Slot(0x38),
            LgmSrc::Slot(0x3c),
            LgmSrc::Slot(0x40),
            LgmSrc::Slot(0x44),
            LgmSrc::Slot(0x48),
            LgmSrc::Slot(0x4c),
            LgmSrc::Slot(0x50),
            LgmSrc::Reg(FP),
            LgmSrc::Slot(0x54),
            LgmSrc::Reg(S6),
            LgmSrc::Slot(0x58),
            LgmSrc::Reg(S2),
            LgmSrc::Reg(S7),
            LgmSrc::Reg(S3),
            LgmSrc::Reg(S4),
            LgmSrc::Reg(S1),
            LgmSrc::Reg(S5),
        ];
        for (i, src) in order.into_iter().enumerate() {
            let item = lgm_value(c, frame, src);
            lgm_add(c, item, 0x0024_9674 + 0x10 * i as u32)?;
        }

        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0xe0);
        frame.restore(c, S0, 0xf0);
        frame.restore(c, S2, 0xd0);
        frame.restore(c, S3, 0xc0);
        frame.restore(c, S4, 0xb0);
        frame.restore(c, S5, 0xa0);
        frame.restore(c, S6, 0x90);
        frame.restore(c, S7, 0x80);
        frame.restore(c, FP, 0x70);
        frame.restore_ra(c, 0x60);
        frame.restore_fpr(c, 25, 0x128);
        frame.restore_fpr(c, 24, 0x120);
        frame.restore_fpr(c, 23, 0x118);
        frame.restore_fpr(c, 22, 0x110);
        frame.restore_fpr(c, 21, 0x108);
        frame.restore_fpr(c, 20, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// The constructor's frame size.
pub(crate) const SCR_FRAME: u32 = 0x150;

/// The page's item objects, by frame slot: slot `k` (`sp + 4k`) holds
/// `this + SCR_ITEM_OFF[k]`. Slots 0..=33 are the 34 items that are built in
/// order; the eight gamma items live in s-registers (see the constructor).
pub(crate) const SCR_ITEM_OFF: [u32; 34] = [
    0x130, 0x14c, 0x164, 0x194, 0x1c4, 0x1f4, 0x228, 0x25c, 0x290, 0x2c4, 0x2f8, 0x32c, 0x360,
    0x394, 0x3c8, 0x3fc, 0x42c, 0x45c, 0x490, 0x4c0, 0x4f0, 0x520, 0x550, 0x584, 0x5b8, 0x5ec,
    0x620, 0x654, 0x688, 0x6bc, 0x6f0, 0x724, 0x758, 0x78c,
];

/// `this + off`, as the compiled code computes it from s0.
pub(crate) fn scr_field(c: &Cpu, off: u32) -> u64 {
    sx((c.g[S0][0] as u32).wrapping_add(off))
}

/// `gp + off` (a pointer into the globals).
pub(crate) fn scr_gp(c: &Cpu, off: u32) -> u64 {
    sx((c.g[GP][0] as u32).wrapping_add(off))
}

/// `this->item[k] = this + SCR_ITEM_OFF[k]`: the address goes to the frame
/// slot `sp + 4k` and stays in v0.
pub(crate) fn scr_store_slot(c: &mut Cpu, k: u32) {
    c.g[V0][0] = scr_field(c, SCR_ITEM_OFF[k as usize]);
    let v = c.g[V0][0];
    let a = sp(c).wrapping_add(4 * k);
    c.sw(a, v);
}

/// The item address in frame slot `k` (`lw sp + 4k`).
pub(crate) fn scr_slot(c: &mut Cpu, k: u32) -> u64 {
    let a = sp(c).wrapping_add(4 * k);
    c.lw(a)
}

/// `cIntMenuItem(item, index, &int at gp + gp_off, min, max, label)`
/// (0x2cd8f8): the item is `a0`, the range is `a3`..`t0`, the label `t1`.
pub(crate) fn scr_int(
    c: &mut Cpu,
    item: u64,
    index: u64,
    gp_off: u32,
    min: u64,
    max: u64,
    label: u64,
    ret: u32,
) -> Flow {
    c.g[A0][0] = item;
    c.g[A1][0] = index;
    c.g[A2][0] = scr_gp(c, gp_off);
    c.g[A3][0] = min;
    c.g[T0][0] = max;
    c.g[T1][0] = label;
    call_ptr(c, 0x002c_d8f8, ret)
}

/// `cFloatMenuItem_ctor(item, index, &float at gp + gp_off, label, 0.0, max)`
/// (0x2cdce8): min is 0.0 in f20, max is in the f-register `max_f`.
pub(crate) fn scr_float(
    c: &mut Cpu,
    item: u64,
    index: u64,
    gp_off: u32,
    label: u64,
    max_f: usize,
    ret: u32,
) -> Flow {
    c.g[A0][0] = item;
    c.g[A1][0] = index;
    c.mov_s(12, 20);
    c.mov_s(13, max_f);
    c.g[A2][0] = scr_gp(c, gp_off);
    c.g[A3][0] = label;
    call_ptr(c, 0x002c_dce8, ret)
}

/// `MenuSysPage_InsertItem(this, item, -1)` (0x2cbcc8): append the item.
pub(crate) fn scr_add(c: &mut Cpu, item: u64, ret: u32) -> Flow {
    c.g[A1][0] = item;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = u64::MAX;
    call_ptr(c, 0x002c_bcc8, ret)
}

/// `ScreenTintMenu *ScreenTintMenu_ctor(ScreenTintMenu *this)` (0x24a228):
/// the debug menu page of the screen tint. It sets the menu base up
/// (`MenuSysPage_ctor`), the title label and a spacer, then the 42 items of
/// the tint: int toggles, float ranges and tint colours, the lightning flash
/// and the gamma. Each item is built with its label and range, then every
/// item is appended to the page with index -1. Returns `this`.
pub fn screen_tint_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 611;
        let frame = Frame::open(c, SCR_FRAME);
        frame.save(c, S0, 0x120);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x110);
        frame.save(c, S2, 0x100);
        frame.save(c, S3, 0xf0);
        frame.save(c, S4, 0xe0);
        frame.save(c, S5, 0xd0);
        frame.save(c, S6, 0xc0);
        frame.save(c, S7, 0xb0);
        frame.save(c, FP, 0xa0);
        frame.save_ra(c, 0x90);
        frame.save_fpr(c, 22, 0x140);
        frame.save_fpr(c, 21, 0x138);
        frame.save_fpr(c, 20, 0x130);
        // The gamma items (kept in s-registers across the calls).
        c.g[S1][0] = scr_field(c, 0x88c);
        c.g[S4][0] = scr_field(c, 0x7f0);
        c.g[S5][0] = scr_field(c, 0x858);
        c.g[S6][0] = scr_field(c, 0x7c0);
        c.g[S7][0] = scr_field(c, 0x8c0);
        c.g[FP][0] = scr_field(c, 0x824);
        c.g[S2][0] = scr_field(c, 0x8f4);
        c.g[S3][0] = scr_field(c, 0x928);

        // The menu base: MenuSysPage_ctor(this); a0 is still this.
        scr_store_slot(c, 0);
        call_ptr(c, 0x002c_aa58, 0x0024_a290)?;

        // The title (cNullMenuItem, label 0x47e680), with the page's float
        // constants 0.0, 2.0 and 1.0 in f20..f22 and the page vtable.
        c.mtc1(0, 20);
        scr_store_slot(c, 1);
        c.g[A1][0] = 0x47_e680;
        c.g[V0][0] = 0x47_fa20;
        c.g[AT][0] = 0x4000_0000;
        c.mtc1(1, 21);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 22);
        let vtable = c.g[V0][0];
        let vtable_at = (c.g[S0][0] as u32).wrapping_add(0x12c);
        c.sw(vtable_at, vtable);
        c.g[A0][0] = scr_slot(c, 0);
        call_ptr(c, 0x002c_d2a0, 0x0024_a2c8)?;

        // The space item (cSpaceMenuItem, 40 high).
        c.g[A0][0] = scr_slot(c, 1);
        scr_store_slot(c, 2);
        c.g[A1][0] = 0x28;
        call_ptr(c, 0x002c_d350, 0x0024_a2dc)?;

        // Enable, override world painter and display values: int toggles.
        let item = scr_slot(c, 2);
        scr_store_slot(c, 3);
        scr_store_slot(c, 4);
        scr_int(c, item, 0, 0x14e8, 0, 1, 0x4a_2d30, 0x0024_a30c)?;
        let item = scr_slot(c, 3);
        scr_store_slot(c, 5);
        scr_int(c, item, 1, 0x14ec, 0, 1, 0x47_e238, 0x0024_a334)?;
        let item = scr_slot(c, 4);
        scr_store_slot(c, 6);
        scr_store_slot(c, 7);
        scr_int(c, item, 2, 0x14f0, 0, 1, 0x47_e698, 0x0024_a364)?;

        // The tint colours, fills and the XN tint: float ranges.
        let item = scr_slot(c, 5);
        scr_store_slot(c, 8);
        scr_float(c, item, 3, 0x14f4, 0x47_e6a8, 21, 0x0024_a38c)?;
        let item = scr_slot(c, 6);
        scr_store_slot(c, 9);
        scr_store_slot(c, 10);
        scr_float(c, item, 4, 0x14f8, 0x47_e6b8, 21, 0x0024_a3bc)?;
        let item = scr_slot(c, 7);
        scr_store_slot(c, 11);
        scr_store_slot(c, 12);
        scr_float(c, item, 5, 0x14fc, 0x47_e6c8, 21, 0x0024_a3ec)?;
        let item = scr_slot(c, 8);
        scr_store_slot(c, 13);
        scr_store_slot(c, 14);
        scr_float(c, item, 6, 0x1500, 0x47_e6d8, 22, 0x0024_a41c)?;
        let item = scr_slot(c, 9);
        scr_store_slot(c, 15);
        scr_store_slot(c, 16);
        scr_float(c, item, 7, 0x1504, 0x47_e6e8, 22, 0x0024_a44c)?;
        let item = scr_slot(c, 10);
        scr_store_slot(c, 17);
        scr_store_slot(c, 18);
        scr_float(c, item, 8, 0x1508, 0x47_e6f8, 22, 0x0024_a47c)?;
        let item = scr_slot(c, 11);
        scr_store_slot(c, 19);
        scr_store_slot(c, 20);
        scr_float(c, item, 9, 0x150c, 0x47_e708, 21, 0x0024_a4ac)?;
        let item = scr_slot(c, 12);
        scr_store_slot(c, 21);
        scr_store_slot(c, 22);
        scr_float(c, item, 10, 0x1510, 0x47_e718, 21, 0x0024_a4dc)?;
        let item = scr_slot(c, 13);
        scr_store_slot(c, 23);
        scr_store_slot(c, 24);
        scr_float(c, item, 11, 0x1514, 0x47_e728, 21, 0x0024_a50c)?;
        let item = scr_slot(c, 14);
        scr_store_slot(c, 25);
        scr_store_slot(c, 26);
        scr_float(c, item, 12, 0x1518, 0x4a_2d58, 21, 0x0024_a53c)?;

        // The lightning flash: toggles, timings and tints.
        let item = scr_slot(c, 15);
        scr_store_slot(c, 27);
        scr_store_slot(c, 28);
        scr_int(c, item, 13, 0x151c, 0, 1, 0x47_e738, 0x0024_a56c)?;
        let item = scr_slot(c, 16);
        scr_store_slot(c, 29);
        scr_store_slot(c, 30);
        scr_int(c, item, 14, 0x1520, u64::MAX, 1000, 0x47_e750, 0x0024_a59c)?;
        let item = scr_slot(c, 17);
        scr_store_slot(c, 31);
        scr_store_slot(c, 32);
        scr_float(c, item, 15, 0x1524, 0x47_e760, 22, 0x0024_a5cc)?;
        let item = scr_slot(c, 18);
        scr_store_slot(c, 33);
        scr_int(c, item, 16, 0x1528, 0, 600, 0x47_e778, 0x0024_a5f4)?;
        let item = scr_slot(c, 19);
        scr_int(c, item, 17, 0x152c, 0, 600, 0x47_e788, 0x0024_a614)?;
        let item = scr_slot(c, 20);
        scr_int(c, item, 18, 0x1530, 0, 600, 0x47_e798, 0x0024_a634)?;
        let item = scr_slot(c, 21);
        scr_int(c, item, 19, 0x1534, 0, 600, 0x47_e7b0, 0x0024_a654)?;
        let item = scr_slot(c, 22);
        scr_float(c, item, 20, 0x1538, 0x47_e7c0, 21, 0x0024_a674)?;
        let item = scr_slot(c, 23);
        scr_float(c, item, 21, 0x153c, 0x47_e7d0, 21, 0x0024_a694)?;
        let item = scr_slot(c, 24);
        scr_float(c, item, 22, 0x1540, 0x47_e7e0, 21, 0x0024_a6b4)?;
        let item = scr_slot(c, 25);
        scr_float(c, item, 23, 0x1544, 0x47_e7f0, 21, 0x0024_a6d4)?;
        let item = scr_slot(c, 26);
        scr_float(c, item, 24, 0x1548, 0x47_e800, 21, 0x0024_a6f4)?;
        let item = scr_slot(c, 27);
        scr_float(c, item, 25, 0x154c, 0x47_e818, 21, 0x0024_a714)?;
        let item = scr_slot(c, 28);
        scr_float(c, item, 26, 0x1550, 0x47_e828, 21, 0x0024_a734)?;
        let item = scr_slot(c, 29);
        scr_float(c, item, 27, 0x1554, 0x47_e840, 21, 0x0024_a754)?;
        let item = scr_slot(c, 30);
        scr_float(c, item, 28, 0x1558, 0x47_e858, 21, 0x0024_a774)?;
        let item = scr_slot(c, 31);
        scr_float(c, item, 29, 0x155c, 0x47_e870, 21, 0x0024_a794)?;
        let item = scr_slot(c, 32);
        scr_float(c, item, 30, 0x1560, 0x47_e888, 21, 0x0024_a7b4)?;
        let item = scr_slot(c, 33);
        scr_float(c, item, 31, 0x1564, 0x47_e8a0, 21, 0x0024_a7d4)?;

        // The gamma: enable (int), then power and the tints and fills.
        let item = c.g[S6][0];
        scr_int(c, item, 0x20, 0x1568, 0, 1, 0x47_e8b8, 0x0024_a7f4)?;
        let item = c.g[S4][0];
        scr_float(c, item, 0x21, 0x156c, 0x47_e8c8, 21, 0x0024_a814)?;
        let item = c.g[FP][0];
        scr_float(c, item, 0x22, 0x1570, 0x47_e8d8, 21, 0x0024_a834)?;
        let item = c.g[S5][0];
        scr_float(c, item, 0x23, 0x1574, 0x47_e8e8, 21, 0x0024_a854)?;
        let item = c.g[S1][0];
        scr_float(c, item, 0x24, 0x1578, 0x47_e900, 21, 0x0024_a874)?;
        let item = c.g[S7][0];
        scr_float(c, item, 0x25, 0x157c, 0x47_e910, 22, 0x0024_a894)?;
        let item = c.g[S2][0];
        scr_float(c, item, 0x26, 0x1580, 0x47_e920, 22, 0x0024_a8b4)?;
        let item = c.g[S3][0];
        scr_float(c, item, 0x27, 0x1584, 0x47_e938, 22, 0x0024_a8d4)?;

        // Append all 42 items to the page, in slot order then the gamma.
        for k in 0..34u32 {
            let item = scr_slot(c, k);
            scr_add(c, item, 0x0024_a8e4 + 0x10 * k)?;
        }
        for (i, reg) in [S6, S4, FP, S5, S1, S7, S2, S3].into_iter().enumerate() {
            let item = c.g[reg][0];
            scr_add(c, item, 0x0024_a8e4 + 0x10 * (34 + i as u32))?;
        }

        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x110);
        frame.restore(c, S0, 0x120);
        frame.restore(c, S2, 0x100);
        frame.restore(c, S3, 0xf0);
        frame.restore(c, S4, 0xe0);
        frame.restore(c, S5, 0xd0);
        frame.restore(c, S6, 0xc0);
        frame.restore(c, S7, 0xb0);
        frame.restore(c, FP, 0xa0);
        frame.restore_ra(c, 0x90);
        frame.restore_fpr(c, 22, 0x140);
        frame.restore_fpr(c, 21, 0x138);
        frame.restore_fpr(c, 20, 0x130);
        frame.close(c);
        Ok(())
    })
}

/// Offsets of the menu's vtable and items in the `cBoardTrailMenu` object.
pub(crate) const BTM_VTBL: u32 = 0x12c;

pub(crate) const BTM_TITLE: u32 = 0x130;

pub(crate) const BTM_SPACE: u32 = 0x14c;

pub(crate) const BTM_ENABLE: u32 = 0x164;

pub(crate) const BTM_INNER_WIDTH: u32 = 0x194;

pub(crate) const BTM_OUTER_WIDTH: u32 = 0x1c8;

pub(crate) const BTM_HEIGHT: u32 = 0x1fc;

pub(crate) const BTM_DEPTH_TO_HEIGHT: u32 = 0x230;

pub(crate) const BTM_PACKED_DEPTH: u32 = 0x264;

pub(crate) const BTM_LOOSE_DEPTH: u32 = 0x298;

pub(crate) const BTM_POWDER_DEPTH: u32 = 0x2cc;

pub(crate) const BTM_DEEP_POWDER_DEPTH: u32 = 0x300;

pub(crate) const BTM_TOP_BIAS_FRONT: u32 = 0x334;

pub(crate) const BTM_TOP_BIAS_BEHIND: u32 = 0x368;

pub(crate) const BTM_TOP_BIAS_CLEAR: u32 = 0x39c;

pub(crate) const BTM_INNER_JITTER: u32 = 0x3d0;

pub(crate) const BTM_BASE_U: u32 = 0x404;

pub(crate) const BTM_INNER_U: u32 = 0x438;

pub(crate) const BTM_OUTER_U: u32 = 0x46c;

pub(crate) const BTM_OUTER_NORMAL: u32 = 0x4a0;

pub(crate) const BTM_INNER_NORMAL: u32 = 0x4d4;

pub(crate) const BTM_BASE_NORMAL: u32 = 0x508;

pub(crate) const BTM_FADE_SEGMENTS: u32 = 0x53c;

pub(crate) const BTM_DRAW_FIXUP: u32 = 0x56c;

pub(crate) const BTM_SLICE_CAPS: u32 = 0x59c;

/// Where the generated body reads an item address from when it adds the
/// items to the page: a frame slot, or a callee-saved register.
#[derive(Clone, Copy)]
pub(crate) enum BtmSrc {
    Slot(u32),
    Reg(usize),
}

/// `this + off` as the 64-bit value the generated code keeps in a register.
pub(crate) fn btm_at(this: u64, off: u32) -> u64 {
    sx((this as u32).wrapping_add(off))
}

/// `V0 = this + off; sw V0, sp + slot`: an item address kept in the frame.
pub(crate) fn btm_stash(c: &mut Cpu, frame: Frame, slot: u32, off: u32) {
    c.g[V0][0] = btm_at(c.g[S0][0], off);
    let addr = frame.at(c, slot);
    let v = c.g[V0][0];
    c.sw(addr, v);
}

/// The item address a call reads from `src`.
pub(crate) fn btm_value(c: &mut Cpu, frame: Frame, src: BtmSrc) -> u64 {
    match src {
        BtmSrc::Slot(off) => {
            let addr = frame.at(c, off);
            c.lw(addr)
        }
        BtmSrc::Reg(r) => c.g[r][0],
    }
}

/// `cIntMenuItem(item, index, &var, min, max, name)` (0x2cd8f8): the item
/// address in a0, the index in a1, the gp variable at `var` in a2, min in
/// a3, max in t0, the name in t1. Returns to `ret_addr`.
pub(crate) fn btm_int(
    c: &mut Cpu,
    item: u64,
    index: u32,
    var: u32,
    min: u64,
    max: u64,
    name: u64,
    ret_addr: u32,
) -> Flow {
    c.g[A0][0] = item;
    c.g[A1][0] = index as u64;
    c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(var));
    c.g[A3][0] = min;
    c.g[T0][0] = max;
    c.g[T1][0] = name;
    call_ptr(c, 0x002c_d8f8, ret_addr)
}

/// `cFloatMenuItem_ctor(item, index, &var, min, max, name)` (0x2cdce8): the
/// caller has set the bounds in f12 and f13; the name goes in a3.
pub(crate) fn btm_float(
    c: &mut Cpu,
    item: u64,
    index: u32,
    var: u32,
    name: u64,
    ret_addr: u32,
) -> Flow {
    c.g[A0][0] = item;
    c.g[A3][0] = name;
    c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(var));
    c.g[A1][0] = index as u64;
    call_ptr(c, 0x002c_dce8, ret_addr)
}

/// `MenuSysPage_InsertItem(this, item, -1)` (0x2cbcc8).
pub(crate) fn btm_add(c: &mut Cpu, item: u64, ret_addr: u32) -> Flow {
    c.g[A1][0] = item;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = u64::MAX;
    call_ptr(c, 0x002c_bcc8, ret_addr)
}

/// `cBoardTrailMenu_cBoardTrailMenu(this)` (0x24b0e8): the constructor of the
/// board trail debug menu. It builds the page (`MenuSysPage_ctor`), the
/// title (a null item named "Board Trail Menu") and the spacer, then the
/// switches (`cIntMenuItem`) and the scales (`cFloatMenuItem_ctor`), each
/// bound to its gp variable. It sets the vtable and inserts the 24 items
/// into the page in order. Returns `this` in v0.
///
/// The generated body keeps the item addresses it passes from one
/// constructor to the next in frame slots (`sp + 4 * n`) and the ones used
/// late in s0..s7/fp. The port writes the same slots before the same calls,
/// so an unwinding callee leaves the same stack. Its only exits are the
/// unwind returns from the calls.
pub fn board_trail_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 377;
        let frame = Frame::open(c, 0x110);
        frame.save(c, S0, 0xd0);
        frame.save(c, S1, 0xc0);
        frame.save(c, S2, 0xb0);
        frame.save(c, S3, 0xa0);
        frame.save(c, S4, 0x90);
        frame.save(c, S5, 0x80);
        frame.save(c, S6, 0x70);
        frame.save(c, S7, 0x60);
        frame.save(c, FP, 0x50);
        frame.save_ra(c, 0x40);
        frame.save_fpr(c, 25, 0x108);
        frame.save_fpr(c, 24, 0x100);
        frame.save_fpr(c, 23, 0xf8);
        frame.save_fpr(c, 22, 0xf0);
        frame.save_fpr(c, 21, 0xe8);
        frame.save_fpr(c, 20, 0xe0);

        // The callee-saved item addresses the later calls read.
        c.g[S0][0] = c.g[A0][0];
        c.g[S3][0] = btm_at(c.g[S0][0], BTM_INNER_NORMAL);
        c.g[S2][0] = btm_at(c.g[S0][0], BTM_FADE_SEGMENTS);
        c.g[S5][0] = btm_at(c.g[S0][0], BTM_OUTER_U);
        c.g[S6][0] = btm_at(c.g[S0][0], BTM_BASE_NORMAL);
        c.g[S7][0] = btm_at(c.g[S0][0], BTM_BASE_U);
        c.g[FP][0] = btm_at(c.g[S0][0], BTM_OUTER_NORMAL);
        c.g[S4][0] = btm_at(c.g[S0][0], BTM_DRAW_FIXUP);
        c.g[S1][0] = btm_at(c.g[S0][0], BTM_SLICE_CAPS);

        // The page: MenuSysPage_ctor(this), with the title item in slot 0.
        btm_stash(c, frame, 0x00, BTM_TITLE);
        call_ptr(c, 0x002c_aa58, 0x0024_b15c)?;

        // The scale bounds: f23 = -100, f22 = 100, f21 = -1000, f20 = 1000,
        // f25 = 0. The vtable goes in at +0x12c.
        c.g[AT][0] = 0xffff_ffff_c2c8_0000;
        c.mtc1(1, 23);
        btm_stash(c, frame, 0x04, BTM_SPACE);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x15e8));
        c.g[V0][0] = sx(0x0048_0000u32.wrapping_sub(0x6e8));
        c.g[AT][0] = 0x42c8_0000;
        c.mtc1(1, 22);
        c.g[AT][0] = 0xffff_ffff_c47a_0000;
        c.mtc1(1, 21);
        let vtbl_addr = (c.g[S0][0] as u32).wrapping_add(BTM_VTBL);
        let vtbl = c.g[V0][0];
        c.sw(vtbl_addr, vtbl);
        c.g[AT][0] = 0x447a_0000;
        c.mtc1(1, 20);
        c.mtc1(0, 25);

        // Title: a null item named "Board Trail Menu".
        let title = btm_value(c, frame, BtmSrc::Slot(0x00));
        c.g[A0][0] = title;
        call_ptr(c, 0x002c_d2a0, 0x0024_b1a4)?;

        // Spacer: a space item of height 40 (f24 = 1.0).
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 24);
        let space = btm_value(c, frame, BtmSrc::Slot(0x04));
        c.g[A0][0] = space;
        btm_stash(c, frame, 0x08, BTM_ENABLE);
        c.g[A1][0] = 0x28;
        call_ptr(c, 0x002c_d350, 0x0024_b1c0)?;

        // Enable (int 0..1).
        let item = btm_value(c, frame, BtmSrc::Slot(0x08));
        btm_stash(c, frame, 0x0c, BTM_INNER_WIDTH);
        btm_int(
            c,
            item,
            0,
            0x148c,
            0,
            1,
            sx(0x004a_0000u32.wrapping_add(0x2d30)),
            0x0024_b1e8,
        )?;

        // Inner width scale (float -100..100).
        let item = btm_value(c, frame, BtmSrc::Slot(0x0c));
        c.mov_s(12, 23);
        c.mov_s(13, 22);
        btm_float(
            c,
            item,
            1,
            0x1490,
            sx(0x0048_0000u32.wrapping_sub(0x15d0)),
            0x0024_b208,
        )?;

        // Outer width scale (float -1000..1000).
        btm_stash(c, frame, 0x10, BTM_OUTER_WIDTH);
        let item = c.g[V0][0];
        btm_stash(c, frame, 0x14, BTM_HEIGHT);
        c.mov_s(12, 21);
        c.mov_s(13, 20);
        btm_float(
            c,
            item,
            2,
            0x1494,
            sx(0x0048_0000u32.wrapping_sub(0x15b8)),
            0x0024_b238,
        )?;

        // Height scale (float -20..20: set directly into f12 and f13).
        let item = btm_value(c, frame, BtmSrc::Slot(0x14));
        c.g[AT][0] = 0xffff_ffff_c1a0_0000;
        c.mtc1(1, 12);
        c.g[AT][0] = 0x41a0_0000;
        c.mtc1(1, 13);
        btm_float(
            c,
            item,
            3,
            0x1498,
            sx(0x0048_0000u32.wrapping_sub(0x15a0)),
            0x0024_b260,
        )?;

        // Depth to height scale (float 0..1), then the packed depth scale.
        btm_stash(c, frame, 0x18, BTM_DEPTH_TO_HEIGHT);
        let item = c.g[V0][0];
        btm_stash(c, frame, 0x1c, BTM_PACKED_DEPTH);
        c.mov_s(12, 25);
        c.mov_s(13, 24);
        btm_float(
            c,
            item,
            4,
            0x149c,
            sx(0x0048_0000u32.wrapping_sub(0x1590)),
            0x0024_b290,
        )?;

        // Packed depth scale (float -100..100), then the loose depth slot.
        let item = btm_value(c, frame, BtmSrc::Slot(0x1c));
        btm_stash(c, frame, 0x20, BTM_LOOSE_DEPTH);
        btm_stash(c, frame, 0x24, BTM_POWDER_DEPTH);
        c.mov_s(12, 23);
        c.mov_s(13, 22);
        btm_float(
            c,
            item,
            5,
            0x14a0,
            sx(0x0048_0000u32.wrapping_sub(0x1578)),
            0x0024_b2c0,
        )?;

        // Loose depth scale (float -100..100; f23 becomes -10 after the
        // bounds are read).
        let item = btm_value(c, frame, BtmSrc::Slot(0x20));
        btm_stash(c, frame, 0x28, BTM_DEEP_POWDER_DEPTH);
        c.mov_s(12, 23);
        btm_stash(c, frame, 0x2c, BTM_TOP_BIAS_FRONT);
        c.mov_s(13, 22);
        c.g[AT][0] = 0xffff_ffff_c120_0000;
        c.mtc1(1, 23);
        btm_float(
            c,
            item,
            6,
            0x14a4,
            sx(0x0048_0000u32.wrapping_sub(0x1560)),
            0x0024_b2f8,
        )?;

        // Powder depth scale (float -1000..1000; f22 becomes 10).
        c.g[AT][0] = 0x4120_0000;
        c.mtc1(1, 22);
        let item = btm_value(c, frame, BtmSrc::Slot(0x24));
        btm_stash(c, frame, 0x30, BTM_TOP_BIAS_BEHIND);
        btm_stash(c, frame, 0x34, BTM_TOP_BIAS_CLEAR);
        c.mov_s(12, 21);
        c.mov_s(13, 20);
        btm_float(
            c,
            item,
            7,
            0x14a8,
            sx(0x0048_0000u32.wrapping_sub(0x1548)),
            0x0024_b330,
        )?;

        // Deep powder depth scale (float -1000..1000).
        let item = btm_value(c, frame, BtmSrc::Slot(0x28));
        c.mov_s(12, 21);
        c.mov_s(13, 20);
        btm_float(
            c,
            item,
            8,
            0x14ac,
            sx(0x0048_0000u32.wrapping_sub(0x1530)),
            0x0024_b350,
        )?;

        // Top bias front (float -1000..1000), then the jitter slot.
        let item = btm_value(c, frame, BtmSrc::Slot(0x2c));
        btm_stash(c, frame, 0x38, BTM_INNER_JITTER);
        c.mov_s(12, 21);
        c.mov_s(13, 20);
        btm_float(
            c,
            item,
            9,
            0x14b0,
            sx(0x0048_0000u32.wrapping_sub(0x1518)),
            0x0024_b378,
        )?;

        // Top bias behind (float -1000..1000), then the inner U slot.
        let item = btm_value(c, frame, BtmSrc::Slot(0x30));
        btm_stash(c, frame, 0x3c, BTM_INNER_U);
        c.mov_s(12, 21);
        c.mov_s(13, 20);
        btm_float(
            c,
            item,
            10,
            0x14b4,
            sx(0x0048_0000u32.wrapping_sub(0x1508)),
            0x0024_b3a0,
        )?;

        // Top bias clear (float: the bounds are loaded from gp).
        let item = btm_value(c, frame, BtmSrc::Slot(0x34));
        let gp = c.g[GP][0] as u32;
        c.lwc1(12, gp.wrapping_sub(0x4fe0));
        c.lwc1(13, gp.wrapping_sub(0x4fdc));
        btm_float(
            c,
            item,
            11,
            0x14b8,
            sx(0x0048_0000u32.wrapping_sub(0x14f8)),
            0x0024_b3c0,
        )?;

        // Inner position jitter (float -10..10).
        let item = btm_value(c, frame, BtmSrc::Slot(0x38));
        c.mov_s(12, 23);
        c.mov_s(13, 22);
        btm_float(
            c,
            item,
            12,
            0x14bc,
            sx(0x0048_0000u32.wrapping_sub(0x14e8)),
            0x0024_b3e0,
        )?;

        // Base U, inner U, outer U (float 0..1).
        c.mov_s(12, 25);
        c.mov_s(13, 24);
        let item = c.g[S7][0];
        btm_float(
            c,
            item,
            13,
            0x14c0,
            sx(0x004a_0000u32.wrapping_add(0x2d88)),
            0x0024_b400,
        )?;
        let item = btm_value(c, frame, BtmSrc::Slot(0x3c));
        c.mov_s(12, 25);
        c.mov_s(13, 24);
        btm_float(
            c,
            item,
            14,
            0x14c4,
            sx(0x004a_0000u32.wrapping_add(0x2d90)),
            0x0024_b420,
        )?;
        c.mov_s(12, 25);
        c.mov_s(13, 24);
        let item = c.g[S5][0];
        btm_float(
            c,
            item,
            15,
            0x14c8,
            sx(0x004a_0000u32.wrapping_add(0x2d98)),
            0x0024_b440,
        )?;

        // Outer normal offset, inner normal offset, base normal offset
        // (float -10..10).
        c.mov_s(12, 23);
        c.mov_s(13, 22);
        let item = c.g[FP][0];
        btm_float(
            c,
            item,
            16,
            0x14cc,
            sx(0x0048_0000u32.wrapping_sub(0x14d0)),
            0x0024_b460,
        )?;
        c.mov_s(12, 23);
        c.mov_s(13, 22);
        let item = c.g[S3][0];
        btm_float(
            c,
            item,
            17,
            0x14d0,
            sx(0x0048_0000u32.wrapping_sub(0x14b8)),
            0x0024_b480,
        )?;
        c.mov_s(12, 23);
        c.mov_s(13, 22);
        let item = c.g[S6][0];
        btm_float(
            c,
            item,
            18,
            0x14d4,
            sx(0x0048_0000u32.wrapping_sub(0x14a0)),
            0x0024_b4a0,
        )?;

        // Fade segments (int 0..0x40), draw fixup layer (int 0..1), use
        // slice caps (int 0..1).
        let item = c.g[S2][0];
        btm_int(
            c,
            item,
            19,
            0x14d8,
            0,
            0x40,
            sx(0x0048_0000u32.wrapping_sub(0x1488)),
            0x0024_b4c0,
        )?;
        let item = c.g[S4][0];
        btm_int(
            c,
            item,
            20,
            0x14dc,
            0,
            1,
            sx(0x0048_0000u32.wrapping_sub(0x1470)),
            0x0024_b4e0,
        )?;
        let item = c.g[S1][0];
        btm_int(
            c,
            item,
            21,
            0x14e0,
            0,
            1,
            sx(0x0048_0000u32.wrapping_sub(0x1458)),
            0x0024_b500,
        )?;

        // Insert the 24 items into the page, in the order the constructor
        // made them: title, spacer, the toggles and scales, then the rest.
        let order = [
            BtmSrc::Slot(0x00),
            BtmSrc::Slot(0x04),
            BtmSrc::Slot(0x08),
            BtmSrc::Slot(0x0c),
            BtmSrc::Slot(0x10),
            BtmSrc::Slot(0x14),
            BtmSrc::Slot(0x18),
            BtmSrc::Slot(0x1c),
            BtmSrc::Slot(0x20),
            BtmSrc::Slot(0x24),
            BtmSrc::Slot(0x28),
            BtmSrc::Slot(0x2c),
            BtmSrc::Slot(0x30),
            BtmSrc::Slot(0x34),
            BtmSrc::Slot(0x38),
            BtmSrc::Reg(S7),
            BtmSrc::Slot(0x3c),
            BtmSrc::Reg(S5),
            BtmSrc::Reg(FP),
            BtmSrc::Reg(S3),
            BtmSrc::Reg(S6),
            BtmSrc::Reg(S2),
            BtmSrc::Reg(S4),
            BtmSrc::Reg(S1),
        ];
        for (i, src) in order.into_iter().enumerate() {
            let item = btm_value(c, frame, src);
            btm_add(c, item, 0x0024_b510 + 0x10 * i as u32)?;
        }

        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S0, 0xd0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore(c, S5, 0x80);
        frame.restore(c, S6, 0x70);
        frame.restore(c, S7, 0x60);
        frame.restore(c, FP, 0x50);
        frame.restore_ra(c, 0x40);
        frame.restore_fpr(c, 25, 0x108);
        frame.restore_fpr(c, 24, 0x100);
        frame.restore_fpr(c, 23, 0xf8);
        frame.restore_fpr(c, 22, 0xf0);
        frame.restore_fpr(c, 21, 0xe8);
        frame.restore_fpr(c, 20, 0xe0);
        frame.close(c);
        Ok(())
    })
}

/// `lw` of the frame slot `sp + off`: an item address the body kept there.
pub(crate) fn mctor_slot(c: &mut Cpu, frame: Frame, off: u32) -> u64 {
    let a = frame.at(c, off);
    c.lw(a)
}

/// `sw value, sp + off`: stash an item address in the frame.
pub(crate) fn mctor_stash(c: &mut Cpu, frame: Frame, off: u32, value: u64) {
    let a = frame.at(c, off);
    c.sw(a, value);
}

/// `this + off` as the 64-bit value a register holds (sign-extended).
pub(crate) fn mctor_addr(this: u64, off: u32) -> u64 {
    sx((this as u32).wrapping_add(off))
}

/// `cIntMenuItem(item, index, &gp[var], min, max, name)` (0x2cd8f8). Returns
/// to `ret`.
pub(crate) fn mctor_int(
    c: &mut Cpu,
    item: u64,
    index: u64,
    var: u32,
    min: u64,
    max: u64,
    name: u32,
    ret: u32,
) -> Flow {
    c.g[A0][0] = item;
    c.g[A1][0] = index;
    c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(var));
    c.g[A3][0] = min;
    c.g[T0][0] = max;
    c.g[T1][0] = sx(name);
    call_ptr(c, 0x002c_d8f8, ret)
}

/// `cFloatMenuItem_ctor(item, index, &gp[var], f12, f13, name)` (0x2cdce8).
/// The caller loads the minimum and maximum into f12 and f13 first.
pub(crate) fn mctor_float(
    c: &mut Cpu,
    item: u64,
    index: u64,
    var: u32,
    name: u32,
    ret: u32,
) -> Flow {
    c.g[A0][0] = item;
    c.g[A1][0] = index;
    c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(var));
    c.g[A3][0] = sx(name);
    call_ptr(c, 0x002c_dce8, ret)
}

/// `cMenu_addItem(this, item, -1)` (0x2cbcc8).
pub(crate) fn mctor_add(c: &mut Cpu, this: u64, item: u64, ret: u32) -> Flow {
    c.g[A1][0] = item;
    c.g[A0][0] = this;
    c.g[A2][0] = u64::MAX;
    call_ptr(c, 0x002c_bcc8, ret)
}

// Offsets of the shadow menu's items and vtable in the `cShadowMenu` object.
pub(crate) const SHADOW_VTBL: u32 = 0x12c;

pub(crate) const SHADOW_TITLE: u32 = 0x130;

pub(crate) const SHADOW_SPACE: u32 = 0x14c;

pub(crate) const SHADOW_ENABLE: u32 = 0x164;

pub(crate) const SHADOW_SHOW_POLYS: u32 = 0x194;

pub(crate) const SHADOW_DIR_X: u32 = 0x1c4;

pub(crate) const SHADOW_DIR_Y: u32 = 0x1f8;

pub(crate) const SHADOW_DIR_Z: u32 = 0x22c;

pub(crate) const SHADOW_LENGTH: u32 = 0x260;

pub(crate) const SHADOW_INTENSITY: u32 = 0x294;

pub(crate) const SHADOW_START_UV: u32 = 0x2c8;

pub(crate) const SHADOW_END_UV: u32 = 0x2fc;

/// `cShadowMenu_cShadowMenu(this)` (0x24abb8): the shadow debug menu. It
/// builds the page (`cMenu_cMenu`), the title and a spacer, then the enable
/// and show-polys switches and the eight shadow scales (`cIntMenuItem`,
/// `cFloatMenuItem_ctor`), each bound to its gp variable. It sets the vtable
/// and inserts the 11 items into the page in order. Returns `this` in v0.
pub fn shadow_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 182;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        frame.save(c, S2, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save(c, S7, 0x30);
        frame.save(c, FP, 0x20);
        frame.save_ra(c, 0x10);
        frame.save_fpr(c, 23, 0xc8);
        frame.save_fpr(c, 22, 0xc0);
        frame.save_fpr(c, 21, 0xb8);
        frame.save_fpr(c, 20, 0xb0);

        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0];
        c.g[V0][0] = mctor_addr(this, SHADOW_TITLE);
        c.g[S3][0] = mctor_addr(this, SHADOW_LENGTH);
        c.g[S4][0] = mctor_addr(this, SHADOW_DIR_Y);
        c.g[S5][0] = mctor_addr(this, SHADOW_INTENSITY);
        c.g[S6][0] = mctor_addr(this, SHADOW_DIR_Z);
        c.g[S7][0] = mctor_addr(this, SHADOW_DIR_X);
        c.g[FP][0] = mctor_addr(this, SHADOW_ENABLE);
        c.g[S1][0] = mctor_addr(this, SHADOW_START_UV);
        c.g[S2][0] = mctor_addr(this, SHADOW_END_UV);

        // The page: cMenu_cMenu(this). The title item's address goes in slot 0.
        let slot0 = frame.at(c, 0);
        let title = c.g[V0][0];
        c.sw(slot0, title);
        call_ptr(c, 0x002c_aa58, 0x0024_ac24)?;

        // Vtable, and the float constants the scales use: f21 = 1.0,
        // f20 = -1.0, f23 = -64.0, f22 = 64.0.
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 21);
        c.g[V0][0] = mctor_addr(this, SHADOW_SPACE);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x16b8)); // "Shadow Menu"
        let space = c.g[V0][0];
        let slot4 = frame.at(c, 4);
        c.sw(slot4, space);
        c.g[V0][0] = sx(0x0048_0000u32.wrapping_sub(0x638)); // cShadowMenu_vtbl
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 20);
        c.g[AT][0] = 0xffff_ffff_c280_0000;
        c.mtc1(1, 23);
        let vtbl = c.g[V0][0];
        c.sw((this as u32).wrapping_add(SHADOW_VTBL), vtbl);
        c.g[AT][0] = 0x4280_0000;
        c.mtc1(1, 22);

        // Title: a null item named "Shadow Menu".
        let a = frame.at(c, 0);
        c.g[A0][0] = c.lw(a);
        call_ptr(c, 0x002c_d2a0, 0x0024_ac68)?;

        // Spacer: a space item of height 40.
        let a = frame.at(c, 4);
        c.g[A0][0] = c.lw(a);
        c.g[A1][0] = 0x28;
        call_ptr(c, 0x002c_d350, 0x0024_ac74)?;

        // Enable (int 0..1) and show shadow polys (int 0..1).
        let show_polys = mctor_addr(this, SHADOW_SHOW_POLYS);
        c.g[V0][0] = show_polys;
        let slot8 = frame.at(c, 8);
        c.sw(slot8, show_polys);
        let enable = c.g[FP][0];
        mctor_int(c, enable, 0, 0x10c0, 0, 1, 0x004a_2d30, 0x0024_ac9c)?;
        let a = frame.at(c, 8);
        let item = c.lw(a);
        mctor_int(c, item, 1, 0x10c4, 0, 1, 0x0047_e958, 0x0024_acbc)?;

        // Shadow direction X, Y, Z in -1..1.
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        let item = c.g[S7][0];
        mctor_float(c, item, 2, 0x10c8, 0x0047_e970, 0x0024_acdc)?;
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        let item = c.g[S4][0];
        mctor_float(c, item, 3, 0x10cc, 0x0047_e980, 0x0024_acfc)?;
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        let item = c.g[S6][0];
        mctor_float(c, item, 4, 0x10d0, 0x0047_e990, 0x0024_ad1c)?;

        // Length in 0.1..100000 (the bounds are gp-relative constants).
        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x4fe8));
        c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x4fe4));
        let item = c.g[S3][0];
        mctor_float(c, item, 5, 0x10d4, 0x004a_2d60, 0x0024_ad3c)?;

        // Intensity in 0..1.
        c.mtc1(0, 12);
        c.mov_s(13, 21);
        let item = c.g[S5][0];
        mctor_float(c, item, 6, 0x10d8, 0x0047_e9a0, 0x0024_ad5c)?;

        // Start and end UV in -64..64.
        c.mov_s(12, 23);
        c.mov_s(13, 22);
        let item = c.g[S1][0];
        mctor_float(c, item, 7, 0x10dc, 0x0047_e9b0, 0x0024_ad7c)?;
        c.mov_s(12, 23);
        c.mov_s(13, 22);
        let item = c.g[S2][0];
        mctor_float(c, item, 8, 0x10e0, 0x004a_2d68, 0x0024_ad9c)?;

        // Insert the 11 items into the page, in the order they were made.
        let a = frame.at(c, 0);
        let item = c.lw(a);
        mctor_add(c, this, item, 0x0024_adac)?;
        let a = frame.at(c, 4);
        let item = c.lw(a);
        mctor_add(c, this, item, 0x0024_adbc)?;
        let item = c.g[FP][0];
        mctor_add(c, this, item, 0x0024_adcc)?;
        let a = frame.at(c, 8);
        let item = c.lw(a);
        mctor_add(c, this, item, 0x0024_addc)?;
        let item = c.g[S7][0];
        mctor_add(c, this, item, 0x0024_adec)?;
        let item = c.g[S4][0];
        mctor_add(c, this, item, 0x0024_adfc)?;
        let item = c.g[S6][0];
        mctor_add(c, this, item, 0x0024_ae0c)?;
        let item = c.g[S3][0];
        mctor_add(c, this, item, 0x0024_ae1c)?;
        let item = c.g[S5][0];
        mctor_add(c, this, item, 0x0024_ae2c)?;
        let item = c.g[S1][0];
        mctor_add(c, this, item, 0x0024_ae3c)?;
        let item = c.g[S2][0];
        mctor_add(c, this, item, 0x0024_ae4c)?;

        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x90);
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 23, 0xc8);
        frame.restore_fpr(c, 22, 0xc0);
        frame.restore_fpr(c, 21, 0xb8);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    })
}

/// `cSnowSurfaceMenu_cSnowSurfaceMenu(this)` (0x24ae90): the snow-surface
/// debug menu. It builds the page (`cMenu_cMenu`), a title and a spacer, then
/// the two switches and the five surface values (`cIntMenuItem`,
/// `cFloatMenuItem_ctor`), sets the vtable and inserts the nine items in
/// order. Returns `this` in v0.
pub fn snow_surface_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 149;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        frame.save(c, S2, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save(c, S7, 0x30);
        frame.save(c, FP, 0x20);
        frame.save_ra(c, 0x10);
        frame.save_fpr(c, 22, 0xc0);
        frame.save_fpr(c, 21, 0xb8);
        frame.save_fpr(c, 20, 0xb0);

        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0];
        c.g[S2][0] = mctor_addr(this, 0x260);
        c.g[S3][0] = mctor_addr(this, 0x1f8);
        c.g[S4][0] = mctor_addr(this, 0x22c);
        c.g[S5][0] = mctor_addr(this, 0x1c4);
        c.g[S6][0] = mctor_addr(this, 0x164);
        c.g[S7][0] = mctor_addr(this, 0x194);
        c.g[FP][0] = mctor_addr(this, 0x130);
        c.g[S1][0] = mctor_addr(this, 0x294);

        // The page: cMenu_cMenu(this).
        call_ptr(c, 0x002c_aa58, 0x0024_aef0)?;

        // f21 = 0.0 (the scales' minimum), f20 = 32.0, f22 = 3.0.
        c.mtc1(0, 21);
        c.g[V0][0] = mctor_addr(this, 0x14c);
        let space = c.g[V0][0];
        let slot0 = frame.at(c, 0);
        c.sw(slot0, space);
        c.g[A0][0] = c.g[FP][0];
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x1640)); // "Snow Surface Menu"
        c.g[V0][0] = sx(0x0048_0000u32.wrapping_sub(0x690)); // cSnowSurfaceMenu_vtbl
        c.g[AT][0] = 0x4200_0000;
        c.mtc1(1, 20);
        let vtbl = c.g[V0][0];
        c.sw((this as u32).wrapping_add(0x12c), vtbl);
        call_ptr(c, 0x002c_d2a0, 0x0024_af20)?;

        c.g[AT][0] = 0x4040_0000;
        c.mtc1(1, 22);
        let a = frame.at(c, 0);
        c.g[A0][0] = c.lw(a);
        c.g[A1][0] = 0x28;
        call_ptr(c, 0x002c_d350, 0x0024_af34)?;

        // The switch items and the scales (floats 0..32, 0..3).
        let item = c.g[S6][0];
        mctor_int(c, item, 0, 0x15ec, 0, 1, 0x004a_2d30, 0x0024_af54)?;
        let item = c.g[S7][0];
        mctor_int(c, item, 1, 0x15f0, 0, 1, 0x0047_e9d8, 0x0024_af74)?;
        let item = c.g[S5][0];
        c.mov_s(12, 21);
        c.mov_s(13, 20);
        mctor_float(c, item, 2, 0x15f4, 0x004a_2d70, 0x0024_af94)?;
        let item = c.g[S3][0];
        c.mov_s(12, 21);
        c.mov_s(13, 20);
        mctor_float(c, item, 3, 0x15f8, 0x004a_2d78, 0x0024_afb4)?;
        let item = c.g[S4][0];
        c.mov_s(13, 20);
        c.mov_s(12, 21);
        mctor_float(c, item, 4, 0x15fc, 0x004a_2d80, 0x0024_afd4)?;
        let item = c.g[S2][0];
        c.mov_s(12, 21);
        c.mov_s(13, 22);
        mctor_float(c, item, 5, 0x1600, 0x0047_e9f0, 0x0024_aff4)?;
        let item = c.g[S1][0];
        c.mov_s(12, 21);
        c.mov_s(13, 22);
        mctor_float(c, item, 6, 0x1604, 0x0047_ea00, 0x0024_b014)?;

        // Insert the nine items into the page: title, spacer, then the rest.
        let item = c.g[FP][0];
        mctor_add(c, this, item, 0x0024_b024)?;
        let a = frame.at(c, 0);
        let item = c.lw(a);
        mctor_add(c, this, item, 0x0024_b034)?;
        let item = c.g[S6][0];
        mctor_add(c, this, item, 0x0024_b044)?;
        let item = c.g[S7][0];
        mctor_add(c, this, item, 0x0024_b054)?;
        let item = c.g[S5][0];
        mctor_add(c, this, item, 0x0024_b064)?;
        let item = c.g[S3][0];
        mctor_add(c, this, item, 0x0024_b074)?;
        let item = c.g[S4][0];
        mctor_add(c, this, item, 0x0024_b084)?;
        let item = c.g[S2][0];
        mctor_add(c, this, item, 0x0024_b094)?;
        let item = c.g[S1][0];
        mctor_add(c, this, item, 0x0024_b0a4)?;

        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x90);
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 22, 0xc0);
        frame.restore_fpr(c, 21, 0xb8);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    })
}

/// `cFlagsMenu_cFlagsMenu(this)` (0x24b6d0): the flags debug menu. It builds
/// the page (`cMenu_cMenu`), a title and a spacer, then two switches
/// (`cIntMenuItem`), sets the vtable and inserts the four items in order.
/// Returns `this` in v0.
pub fn flags_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 64;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0x0);

        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0];
        c.g[S1][0] = mctor_addr(this, 0x164);
        c.g[S3][0] = mctor_addr(this, 0x130);
        c.g[S4][0] = mctor_addr(this, 0x14c);

        // The page: cMenu_cMenu(this).
        call_ptr(c, 0x002c_aa58, 0x0024_b700)?;

        c.g[S2][0] = mctor_addr(this, 0x194);
        c.g[V0][0] = sx(0x0048_0000u32.wrapping_sub(0x740)); // cFlagsMenu_vtbl
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x1448)); // "Flags Menu"
        let vtbl = c.g[V0][0];
        c.sw((this as u32).wrapping_add(0x12c), vtbl);
        call_ptr(c, 0x002c_d2a0, 0x0024_b720)?;

        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = 0x28;
        call_ptr(c, 0x002c_d350, 0x0024_b72c)?;

        // Two switches: index 0 and 1, 0..1.
        let item = c.g[S1][0];
        mctor_int(c, item, 0, 0x1150, 0, 1, 0x0047_ebc8, 0x0024_b74c)?;
        let item = c.g[S2][0];
        mctor_int(c, item, 1, 0x1154, 0, 1, 0x0047_ebd8, 0x0024_b76c)?;

        // Insert the four items into the page: title, spacer, switches.
        let item = c.g[S3][0];
        mctor_add(c, this, item, 0x0024_b77c)?;
        let item = c.g[S4][0];
        mctor_add(c, this, item, 0x0024_b78c)?;
        let item = c.g[S1][0];
        mctor_add(c, this, item, 0x0024_b79c)?;
        let item = c.g[S2][0];
        mctor_add(c, this, item, 0x0024_b7ac)?;

        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x40);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0x0);
        frame.close(c);
        Ok(())
    })
}

/// `cPowerUpFXTogglesMenu_cPowerUpFXTogglesMenu(this)` (0x24b7d0): the
/// power-up effects debug menu. It builds the page (`cMenu_cMenu`), a title
/// and a spacer, then fifteen switches and scales (`cIntMenuItem`,
/// `cFloatMenuItem_ctor`), each bound to its gp variable, sets the vtable and
/// inserts the 17 items in order. Returns `this` in v0.
pub fn power_up_fx_toggles_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 262;
        let frame = Frame::open(c, 0xf0);
        frame.save(c, S0, 0xc0);
        frame.save(c, S1, 0xb0);
        frame.save(c, S2, 0xa0);
        frame.save(c, S3, 0x90);
        frame.save(c, S4, 0x80);
        frame.save(c, S5, 0x70);
        frame.save(c, S6, 0x60);
        frame.save(c, S7, 0x50);
        frame.save(c, FP, 0x40);
        frame.save_ra(c, 0x30);
        frame.save_fpr(c, 22, 0xe0);
        frame.save_fpr(c, 21, 0xd8);
        frame.save_fpr(c, 20, 0xd0);

        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0];
        c.g[V0][0] = mctor_addr(this, 0x130);
        c.g[S3][0] = mctor_addr(this, 0x358);
        c.g[S2][0] = mctor_addr(this, 0x3b8);
        c.g[S5][0] = mctor_addr(this, 0x2f8);
        c.g[S6][0] = mctor_addr(this, 0x388);
        c.g[S7][0] = mctor_addr(this, 0x294);
        c.g[FP][0] = mctor_addr(this, 0x328);
        c.g[S4][0] = mctor_addr(this, 0x3e8);
        c.g[S1][0] = mctor_addr(this, 0x418);

        // The page: cMenu_cMenu(this), with the title item in slot 0.
        let title = c.g[V0][0];
        mctor_stash(c, frame, 0, title);
        call_ptr(c, 0x002c_aa58, 0x0024_b838)?;

        // f20 = 0.0, f21 = 60.0, f22 = 120.0; the vtable.
        c.mtc1(0, 20);
        c.g[V0][0] = mctor_addr(this, 0x14c);
        let space = c.g[V0][0];
        mctor_stash(c, frame, 4, space);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x1418)); // "Power-Up FX Toggles"
        c.g[V0][0] = sx(0x0048_0000u32.wrapping_sub(0x798)); // vtbl
        c.g[AT][0] = 0x4270_0000;
        c.mtc1(1, 21);
        c.g[AT][0] = 0x42f0_0000;
        c.mtc1(1, 22);
        let vtbl = c.g[V0][0];
        c.sw((this as u32).wrapping_add(0x12c), vtbl);

        // Title (a null item), then the spacer (a space item of height 40).
        c.g[A0][0] = mctor_slot(c, frame, 0);
        call_ptr(c, 0x002c_d2a0, 0x0024_b870)?;
        c.g[A0][0] = mctor_slot(c, frame, 4);
        c.g[V0][0] = mctor_addr(this, 0x164);
        let enable = c.g[V0][0];
        mctor_stash(c, frame, 8, enable);
        c.g[A1][0] = 0x28;
        call_ptr(c, 0x002c_d350, 0x0024_b884)?;

        // Index 0: the 0x164 switch (0..1). The next items' addresses go in
        // their frame slots before the call, as the body does.
        let item = mctor_slot(c, frame, 8);
        c.g[V0][0] = mctor_addr(this, 0x194);
        let show = c.g[V0][0];
        mctor_stash(c, frame, 0xc, show);
        c.g[V0][0] = mctor_addr(this, 0x1c4);
        let scale_a = c.g[V0][0];
        mctor_stash(c, frame, 0x10, scale_a);
        mctor_int(c, item, 0, 0x1610, 0, 1, 0x0047_ec00, 0x0024_b8b4)?;

        // Index 1: the 0x194 switch (0..1).
        let item = mctor_slot(c, frame, 0xc);
        c.g[V0][0] = mctor_addr(this, 0x1f8);
        let scale_b = c.g[V0][0];
        mctor_stash(c, frame, 0x14, scale_b);
        mctor_int(c, item, 1, 0x1614, 0, 1, 0x0047_ec20, 0x0024_b8dc)?;

        // Index 2: the 0x1c4 scale (0.0..1.0): f13 = 1.0 from AT.
        let item = mctor_slot(c, frame, 0x10);
        c.g[V0][0] = mctor_addr(this, 0x22c);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 13);
        let scale_c = c.g[V0][0];
        mctor_stash(c, frame, 0x18, scale_c);
        c.mov_s(12, 20);
        mctor_float(c, item, 2, 0x1618, 0x0047_ec40, 0x0024_b908)?;

        // Index 3: the 0x1f8 scale (0.0..60.0).
        let item = mctor_slot(c, frame, 0x14);
        c.g[V0][0] = mctor_addr(this, 0x260);
        let scale_d = c.g[V0][0];
        mctor_stash(c, frame, 0x1c, scale_d);
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        mctor_float(c, item, 3, 0x161c, 0x0047_ec58, 0x0024_b930)?;

        // Index 4: the 0x22c scale (0.0..60.0).
        let item = mctor_slot(c, frame, 0x18);
        c.g[V0][0] = mctor_addr(this, 0x2c8);
        let scale_e = c.g[V0][0];
        mctor_stash(c, frame, 0x20, scale_e);
        c.mov_s(13, 21);
        c.mov_s(12, 20);
        mctor_float(c, item, 4, 0x1620, 0x0047_ec78, 0x0024_b958)?;

        // Index 5: the 0x260 scale (0.0..120.0).
        let item = mctor_slot(c, frame, 0x1c);
        c.mov_s(12, 20);
        c.mov_s(13, 22);
        mctor_float(c, item, 5, 0x1624, 0x0047_ec98, 0x0024_b978)?;

        // Index 6: the S7 scale (0.0..120.0).
        c.mov_s(12, 20);
        c.mov_s(13, 22);
        let item = c.g[S7][0];
        mctor_float(c, item, 6, 0x1628, 0x0047_ecc0, 0x0024_b998)?;

        // Index 7: the 0x2c8 value (0..100).
        let item = mctor_slot(c, frame, 0x20);
        mctor_int(c, item, 7, 0x162c, 0, 100, 0x0047_ece8, 0x0024_b9b8)?;

        // Indices 8..14: the remaining switches (0..1).
        let item = c.g[S5][0];
        mctor_int(c, item, 8, 0x1630, 0, 1, 0x0047_ed08, 0x0024_b9d8)?;
        let item = c.g[FP][0];
        mctor_int(c, item, 9, 0x1634, 0, 1, 0x0047_ed20, 0x0024_b9f8)?;
        let item = c.g[S3][0];
        mctor_int(c, item, 10, 0x1638, 0, 1, 0x0047_ed38, 0x0024_ba18)?;
        let item = c.g[S6][0];
        mctor_int(c, item, 11, 0x163c, 0, 1, 0x0047_ed48, 0x0024_ba38)?;
        let item = c.g[S2][0];
        mctor_int(c, item, 12, 0x1640, 0, 1, 0x0047_ed60, 0x0024_ba58)?;
        let item = c.g[S4][0];
        mctor_int(c, item, 13, 0x1644, 0, 1, 0x0047_ed78, 0x0024_ba78)?;
        let item = c.g[S1][0];
        mctor_int(c, item, 14, 0x1648, 0, 1, 0x0047_ed88, 0x0024_ba98)?;

        // Insert the 17 items into the page, in the order they were made.
        let item = mctor_slot(c, frame, 0);
        mctor_add(c, this, item, 0x0024_baa8)?;
        let item = mctor_slot(c, frame, 4);
        mctor_add(c, this, item, 0x0024_bab8)?;
        let item = mctor_slot(c, frame, 8);
        mctor_add(c, this, item, 0x0024_bac8)?;
        let item = mctor_slot(c, frame, 0xc);
        mctor_add(c, this, item, 0x0024_bad8)?;
        let item = mctor_slot(c, frame, 0x10);
        mctor_add(c, this, item, 0x0024_bae8)?;
        let item = mctor_slot(c, frame, 0x14);
        mctor_add(c, this, item, 0x0024_baf8)?;
        let item = mctor_slot(c, frame, 0x18);
        mctor_add(c, this, item, 0x0024_bb08)?;
        let item = mctor_slot(c, frame, 0x1c);
        mctor_add(c, this, item, 0x0024_bb18)?;
        let item = c.g[S7][0];
        mctor_add(c, this, item, 0x0024_bb28)?;
        let item = mctor_slot(c, frame, 0x20);
        mctor_add(c, this, item, 0x0024_bb38)?;
        let item = c.g[S5][0];
        mctor_add(c, this, item, 0x0024_bb48)?;
        let item = c.g[FP][0];
        mctor_add(c, this, item, 0x0024_bb58)?;
        let item = c.g[S3][0];
        mctor_add(c, this, item, 0x0024_bb68)?;
        let item = c.g[S6][0];
        mctor_add(c, this, item, 0x0024_bb78)?;
        let item = c.g[S2][0];
        mctor_add(c, this, item, 0x0024_bb88)?;
        let item = c.g[S4][0];
        mctor_add(c, this, item, 0x0024_bb98)?;
        let item = c.g[S1][0];
        mctor_add(c, this, item, 0x0024_bba8)?;

        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore(c, S4, 0x80);
        frame.restore(c, S5, 0x70);
        frame.restore(c, S6, 0x60);
        frame.restore(c, S7, 0x50);
        frame.restore(c, FP, 0x40);
        frame.restore_ra(c, 0x30);
        frame.restore_fpr(c, 22, 0xe0);
        frame.restore_fpr(c, 21, 0xd8);
        frame.restore_fpr(c, 20, 0xd0);
        frame.close(c);
        Ok(())
    })
}

/// `cSnowfallTogglesMenu_cSnowfallTogglesMenu(this)` (0x24bbe8): the snowfall
/// debug menu. It builds the page (`cMenu_cMenu`), a title and a spacer, then
/// the switches and scales (`cIntMenuItem`, `cFloatMenuItem_ctor`, and two
/// items from 0x2d02b0 that take their names in a2/a3), sets the vtable and
/// inserts the 14 items in order. Returns `this` in v0.
pub fn snowfall_toggles_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 222;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, S0, 0xb0);
        frame.save(c, S1, 0xa0);
        frame.save(c, S2, 0x90);
        frame.save(c, S3, 0x80);
        frame.save(c, S4, 0x70);
        frame.save(c, S5, 0x60);
        frame.save(c, S6, 0x50);
        frame.save(c, S7, 0x40);
        frame.save(c, FP, 0x30);
        frame.save_ra(c, 0x20);
        frame.save_fpr(c, 22, 0xd0);
        frame.save_fpr(c, 21, 0xc8);
        frame.save_fpr(c, 20, 0xc0);

        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0];
        c.g[V0][0] = mctor_addr(this, 0x130);
        c.g[S2][0] = mctor_addr(this, 0x300);
        c.g[S4][0] = mctor_addr(this, 0x298);
        c.g[S5][0] = mctor_addr(this, 0x334);
        c.g[S6][0] = mctor_addr(this, 0x2cc);
        c.g[S7][0] = mctor_addr(this, 0x264);
        c.g[FP][0] = mctor_addr(this, 0x1fc);
        c.g[S1][0] = mctor_addr(this, 0x990);
        c.g[S3][0] = mctor_addr(this, 0xfec);

        // The page: cMenu_cMenu(this), with the title item in slot 0.
        let title = c.g[V0][0];
        mctor_stash(c, frame, 0, title);
        call_ptr(c, 0x002c_aa58, 0x0024_bc50)?;

        // f20 = 0.0, f21 = 1.0, f22 = 100.0; the vtable.
        c.mtc1(0, 20);
        c.g[V0][0] = mctor_addr(this, 0x14c);
        let space = c.g[V0][0];
        mctor_stash(c, frame, 4, space);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x1260)); // "Snowfall Toggles"
        c.g[V0][0] = sx(0x0048_0000u32.wrapping_sub(0x7f0)); // vtbl
        c.g[AT][0] = 0x42c8_0000;
        c.mtc1(1, 22);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 21);
        let vtbl = c.g[V0][0];
        c.sw((this as u32).wrapping_add(0x12c), vtbl);

        // Title (a null item), then the spacer (a space item of height 40).
        c.g[A0][0] = mctor_slot(c, frame, 0);
        call_ptr(c, 0x002c_d2a0, 0x0024_bc88)?;
        c.g[A0][0] = mctor_slot(c, frame, 4);
        c.g[V0][0] = mctor_addr(this, 0x164);
        let enable = c.g[V0][0];
        mctor_stash(c, frame, 8, enable);
        c.g[A1][0] = 0x28;
        call_ptr(c, 0x002c_d350, 0x0024_bc9c)?;

        // Index 0: the 0x164 switch (0..1); the 0x194 item's slot is set.
        let item = mctor_slot(c, frame, 8);
        c.g[V0][0] = mctor_addr(this, 0x194);
        let show = c.g[V0][0];
        mctor_stash(c, frame, 0xc, show);
        mctor_int(c, item, 0, 0x164c, 0, 1, 0x0047_edb8, 0x0024_bcc4)?;

        // Index 1: the 0x194 scale (0.0..4.0).
        let item = mctor_slot(c, frame, 0xc);
        c.g[V0][0] = mctor_addr(this, 0x1c8);
        c.g[AT][0] = 0x4080_0000;
        c.mtc1(1, 13);
        let scale_a = c.g[V0][0];
        mctor_stash(c, frame, 0x10, scale_a);
        c.mov_s(12, 20);
        mctor_float(c, item, 1, 0x1650, 0x0047_edd0, 0x0024_bcf0)?;

        // Index 2: the 0x1c8 scale (0.0..100.0).
        let item = mctor_slot(c, frame, 0x10);
        c.g[V0][0] = mctor_addr(this, 0x230);
        let scale_b = c.g[V0][0];
        mctor_stash(c, frame, 0x14, scale_b);
        c.mov_s(12, 20);
        c.mov_s(13, 22);
        mctor_float(c, item, 2, 0x1654, 0x0047_ede8, 0x0024_bd18)?;

        // Index 3: the FP scale (0.0..10.0, minimum loaded from gp).
        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x4fd8));
        c.g[AT][0] = 0x4120_0000;
        c.mtc1(1, 13);
        let item = c.g[FP][0];
        mctor_float(c, item, 3, 0x1658, 0x0047_ee00, 0x0024_bd3c)?;

        // Index 4: the 0x230 scale (0.0..360.0).
        let item = mctor_slot(c, frame, 0x14);
        c.g[AT][0] = 0x43b4_0000;
        c.mtc1(1, 13);
        c.mov_s(12, 20);
        mctor_float(c, item, 4, 0x165c, 0x0047_ee18, 0x0024_bd60)?;

        // Index 5: the S7 scale (0.0..1.0).
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        let item = c.g[S7][0];
        mctor_float(c, item, 5, 0x1660, 0x0047_ee30, 0x0024_bd80)?;

        // Index 6: the S4 scale (-2.0..5.0).
        c.g[AT][0] = 0xffff_ffff_c000_0000;
        c.mtc1(1, 12);
        c.g[AT][0] = 0x40a0_0000;
        c.mtc1(1, 13);
        let item = c.g[S4][0];
        mctor_float(c, item, 6, 0x1664, 0x0047_ee48, 0x0024_bda8)?;

        // Index 7: the S6 scale (0.0..1.0).
        c.mov_s(13, 21);
        c.mov_s(12, 20);
        let item = c.g[S6][0];
        mctor_float(c, item, 7, 0x1668, 0x0047_ee60, 0x0024_bdc8)?;

        // Index 8: the S2 scale (0.0..100.0).
        c.mov_s(12, 20);
        c.mov_s(13, 22);
        let item = c.g[S2][0];
        mctor_float(c, item, 8, 0x166c, 0x0047_ee78, 0x0024_bde8)?;

        // Items 9 and 10: 0x2d02b0 takes its names in a2 and a3.
        let item = c.g[S5][0];
        c.g[A2][0] = sx(0x0048_0000u32.wrapping_sub(0x1168));
        c.g[A3][0] = sx(0x0050_0000u32.wrapping_add(0x59b8));
        c.g[A0][0] = item;
        c.g[A1][0] = 0x9;
        call_ptr(c, 0x002d_02b0, 0x0024_be04)?;
        let item = c.g[S1][0];
        c.g[A2][0] = sx(0x0048_0000u32.wrapping_sub(0x1150));
        c.g[A3][0] = sx(0x0050_0000u32.wrapping_add(0x59c8));
        c.g[A0][0] = item;
        c.g[A1][0] = 0xa;
        call_ptr(c, 0x002d_02b0, 0x0024_be20)?;

        // Index 11: the S3 switch (0..1).
        let item = c.g[S3][0];
        mctor_int(c, item, 0xb, 0x1670, 0, 1, 0x0047_eec8, 0x0024_be40)?;

        // Insert the 14 items into the page, in the order they were made.
        let item = mctor_slot(c, frame, 0);
        mctor_add(c, this, item, 0x0024_be50)?;
        let item = mctor_slot(c, frame, 4);
        mctor_add(c, this, item, 0x0024_be60)?;
        let item = mctor_slot(c, frame, 8);
        mctor_add(c, this, item, 0x0024_be70)?;
        let item = mctor_slot(c, frame, 0xc);
        mctor_add(c, this, item, 0x0024_be80)?;
        let item = mctor_slot(c, frame, 0x10);
        mctor_add(c, this, item, 0x0024_be90)?;
        let item = c.g[FP][0];
        mctor_add(c, this, item, 0x0024_bea0)?;
        let item = mctor_slot(c, frame, 0x14);
        mctor_add(c, this, item, 0x0024_beb0)?;
        let item = c.g[S7][0];
        mctor_add(c, this, item, 0x0024_bec0)?;
        let item = c.g[S4][0];
        mctor_add(c, this, item, 0x0024_bed0)?;
        let item = c.g[S6][0];
        mctor_add(c, this, item, 0x0024_bee0)?;
        let item = c.g[S2][0];
        mctor_add(c, this, item, 0x0024_bef0)?;
        let item = c.g[S5][0];
        mctor_add(c, this, item, 0x0024_bf00)?;
        let item = c.g[S1][0];
        mctor_add(c, this, item, 0x0024_bf10)?;
        let item = c.g[S3][0];
        mctor_add(c, this, item, 0x0024_bf20)?;

        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S0, 0xb0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore(c, S6, 0x50);
        frame.restore(c, S7, 0x40);
        frame.restore(c, FP, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 22, 0xd0);
        frame.restore_fpr(c, 21, 0xc8);
        frame.restore_fpr(c, 20, 0xc0);
        frame.close(c);
        Ok(())
    })
}

/// `cTubeMenu_cTubeMenu(this)` (0x24cca0): the tube debug menu. It builds the
/// page (`cMenu_cMenu`), a title and a spacer, then five values (`cIntMenuItem`,
/// `cFloatMenuItem_ctor`), sets the vtable and inserts the eight items in
/// order. Returns `this` in v0.
pub fn tube_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 131;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0x0);
        frame.save_fpr(c, 21, 0xa8);
        frame.save_fpr(c, 20, 0xa0);

        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0];
        c.g[S2][0] = mctor_addr(this, 0x1fc);
        c.g[S3][0] = mctor_addr(this, 0x194);
        c.g[S4][0] = mctor_addr(this, 0x22c);
        c.g[S5][0] = mctor_addr(this, 0x1c8);
        c.g[S6][0] = mctor_addr(this, 0x164);
        c.g[S7][0] = mctor_addr(this, 0x130);
        c.g[FP][0] = mctor_addr(this, 0x14c);
        c.g[S1][0] = mctor_addr(this, 0x25c);

        // The page: cMenu_cMenu(this).
        call_ptr(c, 0x002c_aa58, 0x0024_ccfc)?;

        // f21 = 0.0, f20 = 1000.0; the vtable; the title (a null item).
        c.mtc1(0, 21);
        c.g[V0][0] = sx(0x0048_0000u32.wrapping_sub(0x8a0)); // vtbl
        c.g[A0][0] = c.g[S7][0];
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0xb58)); // "Tube Menu"
        let vtbl = c.g[V0][0];
        c.sw((this as u32).wrapping_add(0x12c), vtbl);
        call_ptr(c, 0x002c_d2a0, 0x0024_cd1c)?;

        // The spacer: a space item of height 40.
        c.g[AT][0] = 0x447a_0000;
        c.mtc1(1, 20);
        c.g[A0][0] = c.g[FP][0];
        c.g[A1][0] = 0x28;
        call_ptr(c, 0x002c_d350, 0x0024_cd30)?;

        // Index 0: the 0x164 switch (0..1). Indices 1..2: 0..1000 scales.
        let item = c.g[S6][0];
        mctor_int(c, item, 0, 0x10e4, 0, 1, 0x004a_2d30, 0x0024_cd50)?;
        let item = c.g[S3][0];
        c.mov_s(12, 21);
        c.mov_s(13, 20);
        mctor_float(c, item, 1, 0x10e8, 0x0047_f4b8, 0x0024_cd70)?;
        let item = c.g[S5][0];
        c.mov_s(12, 21);
        c.mov_s(13, 20);
        mctor_float(c, item, 2, 0x10ec, 0x0047_f4c8, 0x0024_cd90)?;

        // Indices 3..5: switches with a 0..255 range.
        let item = c.g[S2][0];
        mctor_int(c, item, 3, 0x10f0, 0, 0xff, 0x004a_2da8, 0x0024_cdb0)?;
        let item = c.g[S4][0];
        mctor_int(c, item, 4, 0x10f4, 0, 0xff, 0x004a_2db0, 0x0024_cdd0)?;
        let item = c.g[S1][0];
        mctor_int(c, item, 5, 0x10f8, 0, 0xff, 0x004a_2db8, 0x0024_cdf0)?;

        // Insert the eight items into the page: title, spacer, then the rest.
        let item = c.g[S7][0];
        mctor_add(c, this, item, 0x0024_ce00)?;
        let item = c.g[FP][0];
        mctor_add(c, this, item, 0x0024_ce10)?;
        let item = c.g[S6][0];
        mctor_add(c, this, item, 0x0024_ce20)?;
        let item = c.g[S3][0];
        mctor_add(c, this, item, 0x0024_ce30)?;
        let item = c.g[S5][0];
        mctor_add(c, this, item, 0x0024_ce40)?;
        let item = c.g[S2][0];
        mctor_add(c, this, item, 0x0024_ce50)?;
        let item = c.g[S4][0];
        mctor_add(c, this, item, 0x0024_ce60)?;
        let item = c.g[S1][0];
        mctor_add(c, this, item, 0x0024_ce70)?;

        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x80);
        frame.restore(c, S0, 0x90);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0x0);
        frame.restore_fpr(c, 21, 0xa8);
        frame.restore_fpr(c, 20, 0xa0);
        frame.close(c);
        Ok(())
    })
}

/// `cPathArrowMenu_cPathArrowMenu(this)` (0x24ceb0): the path-arrow debug
/// menu. It builds the page (`cMenu_cMenu`), a title, a spacer and the
/// switches and scales (`cIntMenuItem`, `cFloatMenuItem_ctor`), each bound to
/// its gp variable, sets the vtable and inserts the 18 items in order.
/// Returns `this` in v0.
pub fn path_arrow_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 272;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, S0, 0xc0);
        frame.save(c, S1, 0xb0);
        frame.save(c, S2, 0xa0);
        frame.save(c, S3, 0x90);
        frame.save(c, S4, 0x80);
        frame.save(c, S5, 0x70);
        frame.save(c, S6, 0x60);
        frame.save(c, S7, 0x50);
        frame.save(c, FP, 0x40);
        frame.save_ra(c, 0x30);
        frame.save_fpr(c, 21, 0xd8);
        frame.save_fpr(c, 20, 0xd0);

        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0];
        c.g[V0][0] = mctor_addr(this, 0x130);
        c.g[S3][0] = mctor_addr(this, 0x38c);
        c.g[S2][0] = mctor_addr(this, 0x3f4);
        c.g[S5][0] = mctor_addr(this, 0x32c);
        c.g[S6][0] = mctor_addr(this, 0x3c0);
        c.g[S7][0] = mctor_addr(this, 0x2cc);
        c.g[FP][0] = mctor_addr(this, 0x35c);
        c.g[S4][0] = mctor_addr(this, 0x428);
        c.g[S1][0] = mctor_addr(this, 0x458);

        // The page: cMenu_cMenu(this), with the title item in slot 0.
        let title = c.g[V0][0];
        mctor_stash(c, frame, 0, title);
        call_ptr(c, 0x002c_aa58, 0x0024_cf14)?;

        // f21 = 1000.0, f20 = 0.0; the vtable; the title (a null item).
        c.g[AT][0] = 0x447a_0000;
        c.mtc1(1, 21);
        c.g[V0][0] = mctor_addr(this, 0x14c);
        let space = c.g[V0][0];
        mctor_stash(c, frame, 4, space);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0xb28)); // "Path Arrow Menu"
        c.g[V0][0] = 0x0048_0000;
        c.mtc1(0, 20);
        c.g[V0][0] = sx(0x0048_0000u32.wrapping_sub(0x8f8)); // vtbl
        let vtbl = c.g[V0][0];
        c.sw((this as u32).wrapping_add(0x12c), vtbl);
        c.g[A0][0] = mctor_slot(c, frame, 0);
        call_ptr(c, 0x002c_d2a0, 0x0024_cf44)?;

        // The spacer: a space item of height 40.
        c.g[A0][0] = mctor_slot(c, frame, 4);
        c.g[V0][0] = mctor_addr(this, 0x164);
        let enable = c.g[V0][0];
        mctor_stash(c, frame, 8, enable);
        c.g[A1][0] = 0x28;
        call_ptr(c, 0x002c_d350, 0x0024_cf58)?;

        // Index 0: the 0x164 switch (0..1).
        let item = mctor_slot(c, frame, 8);
        c.g[V0][0] = mctor_addr(this, 0x194);
        let scale_a = c.g[V0][0];
        mctor_stash(c, frame, 0xc, scale_a);
        mctor_int(c, item, 0, 0x10fc, 0, 1, 0x004a_2d30, 0x0024_cf80)?;

        // Index 1: the 0x194 scale (0.0..1000.0).
        let item = mctor_slot(c, frame, 0xc);
        c.g[V0][0] = mctor_addr(this, 0x1c8);
        let scale_b = c.g[V0][0];
        mctor_stash(c, frame, 0x10, scale_b);
        c.g[V0][0] = mctor_addr(this, 0x1fc);
        c.g[A3][0] = sx(0x0048_0000u32.wrapping_sub(0xb18)); // name
        let scale_c = c.g[V0][0];
        mctor_stash(c, frame, 0x14, scale_c);
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        mctor_float(c, item, 1, 0x1100, 0x0047_f4e8, 0x0024_cfb0)?;

        // Index 2: the 0x1c8 scale (0.0..1000.0).
        let item = mctor_slot(c, frame, 0x10);
        c.g[V0][0] = mctor_addr(this, 0x230);
        let scale_d = c.g[V0][0];
        mctor_stash(c, frame, 0x18, scale_d);
        c.g[V0][0] = mctor_addr(this, 0x264);
        let scale_e = c.g[V0][0];
        mctor_stash(c, frame, 0x1c, scale_e);
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        mctor_float(c, item, 2, 0x1104, 0x0047_f4f8, 0x0024_cfe0)?;

        // Index 3: the 0x1fc scale (0.0..1000.0).
        let item = mctor_slot(c, frame, 0x14);
        c.g[V0][0] = mctor_addr(this, 0x298);
        let scale_f = c.g[V0][0];
        mctor_stash(c, frame, 0x20, scale_f);
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        mctor_float(c, item, 3, 0x1108, 0x0047_f508, 0x0024_d008)?;

        // Index 4: the 0x230 scale (0.0..1000.0).
        let item = mctor_slot(c, frame, 0x18);
        c.g[V0][0] = mctor_addr(this, 0x2fc);
        let scale_g = c.g[V0][0];
        mctor_stash(c, frame, 0x24, scale_g);
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        mctor_float(c, item, 4, 0x110c, 0x0047_f518, 0x0024_d030)?;

        // Index 5: the 0x264 scale (0.0..1000.0).
        let item = mctor_slot(c, frame, 0x1c);
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        mctor_float(c, item, 5, 0x1110, 0x0047_f528, 0x0024_d050)?;

        // Index 6: the 0x298 scale (0.0..1000.0, name in gp data).
        let item = mctor_slot(c, frame, 0x20);
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        mctor_float(c, item, 6, 0x1114, 0x004a_2dc0, 0x0024_d070)?;

        // Indices 7..10: switches with a 0..255 range.
        let item = c.g[S7][0];
        mctor_int(c, item, 7, 0x1118, 0, 0xff, 0x004a_2da8, 0x0024_d090)?;
        let item = mctor_slot(c, frame, 0x24);
        mctor_int(c, item, 8, 0x111c, 0, 0xff, 0x004a_2db0, 0x0024_d0b0)?;
        let item = c.g[S5][0];
        mctor_int(c, item, 9, 0x1120, 0, 0xff, 0x004a_2db8, 0x0024_d0d0)?;
        let item = c.g[FP][0];
        mctor_int(c, item, 10, 0x1124, 0, 0xff, 0x004a_2dc8, 0x0024_d0f0)?;

        // Index 11: the S3 scale (0.0..1000.0).
        let item = c.g[S3][0];
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        mctor_float(c, item, 11, 0x1128, 0x0047_f538, 0x0024_d110)?;

        // Index 12: the S6 scale (0.0..1000.0).
        let item = c.g[S6][0];
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        mctor_float(c, item, 12, 0x112c, 0x0047_f548, 0x0024_d130)?;

        // Index 13: the S2 scale (50.0..1000.0).
        c.g[AT][0] = 0x4248_0000;
        c.mtc1(1, 12);
        let item = c.g[S2][0];
        c.mov_s(13, 21);
        mctor_float(c, item, 13, 0x1130, 0x0047_f558, 0x0024_d154)?;

        // Index 14: the S4 value (0..1000); index 15: the S1 value (1..18000).
        let item = c.g[S4][0];
        mctor_int(c, item, 14, 0x1134, 0, 0x3e8, 0x004a_2dd0, 0x0024_d174)?;
        let item = c.g[S1][0];
        mctor_int(c, item, 15, 0x1138, 1, 0x4650, 0x0047_f568, 0x0024_d194)?;

        // Insert the 18 items into the page, in the order they were made.
        let item = mctor_slot(c, frame, 0);
        mctor_add(c, this, item, 0x0024_d1a4)?;
        let item = mctor_slot(c, frame, 4);
        mctor_add(c, this, item, 0x0024_d1b4)?;
        let item = mctor_slot(c, frame, 8);
        mctor_add(c, this, item, 0x0024_d1c4)?;
        let item = mctor_slot(c, frame, 0xc);
        mctor_add(c, this, item, 0x0024_d1d4)?;
        let item = mctor_slot(c, frame, 0x10);
        mctor_add(c, this, item, 0x0024_d1e4)?;
        let item = mctor_slot(c, frame, 0x14);
        mctor_add(c, this, item, 0x0024_d1f4)?;
        let item = mctor_slot(c, frame, 0x18);
        mctor_add(c, this, item, 0x0024_d204)?;
        let item = mctor_slot(c, frame, 0x1c);
        mctor_add(c, this, item, 0x0024_d214)?;
        let item = mctor_slot(c, frame, 0x20);
        mctor_add(c, this, item, 0x0024_d224)?;
        let item = c.g[S7][0];
        mctor_add(c, this, item, 0x0024_d234)?;
        let item = mctor_slot(c, frame, 0x24);
        mctor_add(c, this, item, 0x0024_d244)?;
        let item = c.g[S5][0];
        mctor_add(c, this, item, 0x0024_d254)?;
        let item = c.g[FP][0];
        mctor_add(c, this, item, 0x0024_d264)?;
        let item = c.g[S3][0];
        mctor_add(c, this, item, 0x0024_d274)?;
        let item = c.g[S6][0];
        mctor_add(c, this, item, 0x0024_d284)?;
        let item = c.g[S2][0];
        mctor_add(c, this, item, 0x0024_d294)?;
        let item = c.g[S4][0];
        mctor_add(c, this, item, 0x0024_d2a4)?;
        let item = c.g[S1][0];
        mctor_add(c, this, item, 0x0024_d2b4)?;

        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore(c, S4, 0x80);
        frame.restore(c, S5, 0x70);
        frame.restore(c, S6, 0x60);
        frame.restore(c, S7, 0x50);
        frame.restore(c, FP, 0x40);
        frame.restore_ra(c, 0x30);
        frame.restore_fpr(c, 21, 0xd8);
        frame.restore_fpr(c, 20, 0xd0);
        frame.close(c);
        Ok(())
    })
}

/// `VisualEffects` item constructor (0x2cccc8): the item in a0, its name in
/// a1 and a second name word in a2. Returns to `ret`.
pub(crate) fn vfx_item(c: &mut Cpu, item: u64, name: u64, extra: u64, ret: u32) -> Flow {
    c.g[A0][0] = item;
    c.g[A1][0] = name;
    c.g[A2][0] = extra;
    call_ptr(c, 0x002c_ccc8, ret)
}

/// `cVisualEffectsMainMenu_cVisualEffectsMainMenu(this)` (0x24d2f0): the
/// visual-effects main menu. It builds the page (`cMenu_cMenu`), a title and
/// a spacer, then 16 sub-menu entries (`0x2cccc8`), sets the vtable and
/// inserts the 17 items in order. Returns `this` in v0.
pub fn visual_effects_main_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 232;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S0, 0xc0);
        frame.save(c, S1, 0xb0);
        frame.save(c, S2, 0xa0);
        frame.save(c, S3, 0x90);
        frame.save(c, S4, 0x80);
        frame.save(c, S5, 0x70);
        frame.save(c, S6, 0x60);
        frame.save(c, S7, 0x50);
        frame.save(c, FP, 0x40);
        frame.save_ra(c, 0x30);

        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0];
        c.g[V0][0] = mctor_addr(this, 0x164);
        c.g[S1][0] = mctor_addr(this, 0x1d4);
        c.g[S2][0] = mctor_addr(this, 0x20c);
        c.g[S3][0] = mctor_addr(this, 0x244);
        c.g[S4][0] = mctor_addr(this, 0x27c);
        c.g[S5][0] = mctor_addr(this, 0x2b4);
        c.g[FP][0] = mctor_addr(this, 0x19c);

        // The first item (0x164) is kept in slot 0 for the insertion at the end.
        let first = c.g[V0][0];
        mctor_stash(c, frame, 0, first);
        c.g[S6][0] = mctor_addr(this, 0x2ec);
        call_ptr(c, 0x002c_aa58, 0x0024_d348)?;
        c.g[S7][0] = mctor_addr(this, 0x324);

        // The spacer (a null-named item at 0x130 is the title), then the vtable.
        c.g[V0][0] = mctor_addr(this, 0x180);
        let second = c.g[V0][0];
        mctor_stash(c, frame, 4, second);
        c.g[A0][0] = mctor_addr(this, 0x130);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0xa88)); // title name
        c.g[V0][0] = sx(0x0048_0000u32.wrapping_sub(0x950)); // vtbl
        let vtbl = c.g[V0][0];
        c.sw((this as u32).wrapping_add(0x12c), vtbl);
        call_ptr(c, 0x002c_d2a0, 0x0024_d370)?;
        c.g[A0][0] = mctor_addr(this, 0x14c);
        c.g[A1][0] = 0x28;
        call_ptr(c, 0x002c_d350, 0x0024_d37c)?;

        // Sub-menu entries: each a 0x2cccc8 item with its two name words.
        let item = mctor_slot(c, frame, 0);
        c.g[V0][0] = mctor_addr(this, 0x1b8);
        let e_b8 = c.g[V0][0];
        mctor_stash(c, frame, 8, e_b8);
        vfx_item(
            c,
            item,
            sx(0x004d_0000u32.wrapping_sub(0x6720)),
            sx(0x0048_0000u32.wrapping_sub(0xa68)),
            0x0024_d39c,
        )?;

        let item = mctor_slot(c, frame, 4);
        c.g[V0][0] = mctor_addr(this, 0x1f0);
        let e_f0 = c.g[V0][0];
        mctor_stash(c, frame, 0xc, e_f0);
        vfx_item(
            c,
            item,
            sx(0x004d_0000u32.wrapping_sub(0x5cd8)),
            sx(0x0048_0000u32.wrapping_sub(0xa58)),
            0x0024_d3bc,
        )?;

        c.g[V0][0] = mctor_addr(this, 0x228);
        let e_228 = c.g[V0][0];
        mctor_stash(c, frame, 0x10, e_228);
        let item = c.g[FP][0];
        vfx_item(
            c,
            item,
            sx(0x004d_0000u32.wrapping_sub(0x5840)),
            sx(0x0048_0000u32.wrapping_sub(0xa40)),
            0x0024_d3dc,
        )?;

        let item = mctor_slot(c, frame, 8);
        c.g[V0][0] = mctor_addr(this, 0x260);
        let e_260 = c.g[V0][0];
        mctor_stash(c, frame, 0x14, e_260);
        vfx_item(
            c,
            item,
            sx(0x004d_0000u32.wrapping_sub(0x5108)),
            sx(0x004a_0000u32.wrapping_add(0x2dd8)),
            0x0024_d3fc,
        )?;

        c.g[V0][0] = mctor_addr(this, 0x298);
        let e_298 = c.g[V0][0];
        mctor_stash(c, frame, 0x18, e_298);
        let item = c.g[S1][0];
        vfx_item(
            c,
            item,
            sx(0x004d_0000u32.wrapping_sub(0x4d38)),
            sx(0x0048_0000u32.wrapping_sub(0xa30)),
            0x0024_d41c,
        )?;

        let item = mctor_slot(c, frame, 0xc);
        c.g[V0][0] = mctor_addr(this, 0x2d0);
        let e_2d0 = c.g[V0][0];
        mctor_stash(c, frame, 0x1c, e_2d0);
        vfx_item(
            c,
            item,
            sx(0x004d_0000u32.wrapping_sub(0x4418)),
            sx(0x0048_0000u32.wrapping_sub(0xa20)),
            0x0024_d43c,
        )?;

        c.g[V0][0] = mctor_addr(this, 0x308);
        let e_308 = c.g[V0][0];
        mctor_stash(c, frame, 0x20, e_308);
        let item = c.g[S2][0];
        vfx_item(
            c,
            item,
            sx(0x004d_0000u32.wrapping_sub(0x4218)),
            sx(0x0048_0000u32.wrapping_sub(0xa10)),
            0x0024_d45c,
        )?;

        let item = mctor_slot(c, frame, 0x10);
        vfx_item(
            c,
            item,
            sx(0x004d_0000u32.wrapping_sub(0x3eb0)),
            sx(0x0048_0000u32.wrapping_sub(0xa00)),
            0x0024_d474,
        )?;
        let item = c.g[S3][0];
        vfx_item(
            c,
            item,
            sx(0x004d_0000u32.wrapping_sub(0x3550)),
            sx(0x004a_0000u32.wrapping_add(0x2de0)),
            0x0024_d48c,
        )?;
        let item = mctor_slot(c, frame, 0x14);
        vfx_item(
            c,
            item,
            sx(0x004d_0000u32.wrapping_sub(0x3220)),
            sx(0x0048_0000u32.wrapping_sub(0x9f0)),
            0x0024_d4a4,
        )?;
        let item = c.g[S4][0];
        vfx_item(
            c,
            item,
            sx(0x004d_0000u32.wrapping_sub(0x2f58)),
            sx(0x0048_0000u32.wrapping_sub(0x9e0)),
            0x0024_d4bc,
        )?;
        let item = mctor_slot(c, frame, 0x18);
        vfx_item(
            c,
            item,
            sx(0x004d_0000u32.wrapping_sub(0x2988)),
            sx(0x004a_0000u32.wrapping_add(0x2de8)),
            0x0024_d4d4,
        )?;
        let item = c.g[S5][0];
        vfx_item(
            c,
            item,
            sx(0x004d_0000u32.wrapping_sub(0x27c0)),
            sx(0x0048_0000u32.wrapping_sub(0x9d0)),
            0x0024_d4ec,
        )?;
        let item = mctor_slot(c, frame, 0x1c);
        vfx_item(
            c,
            item,
            sx(0x004d_0000u32.wrapping_sub(0x2378)),
            sx(0x0048_0000u32.wrapping_sub(0x9b0)),
            0x0024_d504,
        )?;
        let item = c.g[S6][0];
        vfx_item(
            c,
            item,
            sx(0x004d_0000u32.wrapping_sub(0x1358)),
            sx(0x0048_0000u32.wrapping_sub(0x998)),
            0x0024_d51c,
        )?;
        let item = mctor_slot(c, frame, 0x20);
        vfx_item(
            c,
            item,
            sx(0x004d_0000u32.wrapping_add(0x11f0)),
            sx(0x0048_0000u32.wrapping_sub(0x978)),
            0x0024_d534,
        )?;
        let item = c.g[S7][0];
        vfx_item(
            c,
            item,
            sx(0x004d_0000u32.wrapping_add(0x1480)),
            sx(0x0048_0000u32.wrapping_sub(0x968)),
            0x0024_d54c,
        )?;

        // Insert the 17 items into the page: the first entry, the spacer, then
        // the sub-menus and the rest, in the order they were made.
        let item = mctor_slot(c, frame, 0);
        mctor_add(c, this, item, 0x0024_d55c)?;
        let item = mctor_slot(c, frame, 4);
        mctor_add(c, this, item, 0x0024_d56c)?;
        let item = c.g[FP][0];
        mctor_add(c, this, item, 0x0024_d57c)?;
        let item = mctor_slot(c, frame, 8);
        mctor_add(c, this, item, 0x0024_d58c)?;
        let item = c.g[S1][0];
        mctor_add(c, this, item, 0x0024_d59c)?;
        let item = mctor_slot(c, frame, 0xc);
        mctor_add(c, this, item, 0x0024_d5ac)?;
        let item = c.g[S2][0];
        mctor_add(c, this, item, 0x0024_d5bc)?;
        let item = mctor_slot(c, frame, 0x10);
        mctor_add(c, this, item, 0x0024_d5cc)?;
        let item = c.g[S3][0];
        mctor_add(c, this, item, 0x0024_d5dc)?;
        let item = mctor_slot(c, frame, 0x14);
        mctor_add(c, this, item, 0x0024_d5ec)?;
        let item = c.g[S4][0];
        mctor_add(c, this, item, 0x0024_d5fc)?;
        let item = mctor_slot(c, frame, 0x18);
        mctor_add(c, this, item, 0x0024_d60c)?;
        let item = c.g[S5][0];
        mctor_add(c, this, item, 0x0024_d61c)?;
        let item = mctor_slot(c, frame, 0x1c);
        mctor_add(c, this, item, 0x0024_d62c)?;
        let item = c.g[S6][0];
        mctor_add(c, this, item, 0x0024_d63c)?;
        let item = mctor_slot(c, frame, 0x20);
        mctor_add(c, this, item, 0x0024_d64c)?;
        let item = c.g[S7][0];
        mctor_add(c, this, item, 0x0024_d65c)?;

        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore(c, S4, 0x80);
        frame.restore(c, S5, 0x70);
        frame.restore(c, S6, 0x60);
        frame.restore(c, S7, 0x50);
        frame.restore(c, FP, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `cVisualEffectsMainMenu_OnActivate(this)` (0x24d690): forwards to 0x2cbf60
/// with `this` unchanged (a 16-byte frame holding `ra`).
pub fn visual_effects_main_menu_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x002c_bf60, 0x0024_d6a0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cVisualEffectsMainMenu_OnDeactivate(this)` (0x24d6b0): forwards to
/// 0x2cbfd0 with `this` unchanged (a 16-byte frame holding `ra`).
pub fn visual_effects_main_menu_on_deactivate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x002c_bfd0, 0x0024_d6c0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Tear down one menu item (0x2ca280) with `a1 = 2`, returning to `ret`.
pub(crate) fn mdtor_item(c: &mut Cpu, item: u64, ret: u32) -> Flow {
    c.g[A0][0] = item;
    c.g[A1][0] = 2;
    call_ptr(c, 0x002c_a280, ret)
}

/// `cPowerUpFXTogglesMenu_dtor(this, flags)` (0x24e0f8): tears down the
/// seventeen items of the page in reverse build order, then the menu itself
/// (0x2caa80) with the same flags in a1. Returns to the caller.
pub fn power_up_fx_toggles_menu_dtor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 65;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0x0);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        let this = c.g[S0][0];
        let items: [(u32, u32); 17] = [
            (0x418, 0x0024_e11c),
            (0x3e8, 0x0024_e128),
            (0x3b8, 0x0024_e134),
            (0x388, 0x0024_e140),
            (0x358, 0x0024_e14c),
            (0x328, 0x0024_e158),
            (0x2f8, 0x0024_e164),
            (0x2c8, 0x0024_e170),
            (0x294, 0x0024_e17c),
            (0x260, 0x0024_e188),
            (0x22c, 0x0024_e194),
            (0x1f8, 0x0024_e1a0),
            (0x1c4, 0x0024_e1ac),
            (0x194, 0x0024_e1b8),
            (0x164, 0x0024_e1c4),
            (0x14c, 0x0024_e1d0),
            (0x130, 0x0024_e1dc),
        ];
        for (off, ret) in items {
            mdtor_item(c, mctor_addr(this, off), ret)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x002c_aa80, 0x0024_e1e8)?;
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x0);
        frame.close(c);
        Ok(())
    })
}

/// `cBoardTrailMenu_dtor(this, flags)` (0x24dfa0): tears down the 24 items
/// of the page in reverse build order, then the menu itself (0x2caa80).
pub fn board_trail_menu_dtor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 86;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0x0);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        let this = c.g[S0][0];
        let items: [(u32, u32); 24] = [
            (0x59c, 0x0024_dfc4),
            (0x56c, 0x0024_dfd0),
            (0x53c, 0x0024_dfdc),
            (0x508, 0x0024_dfe8),
            (0x4d4, 0x0024_dff4),
            (0x4a0, 0x0024_e000),
            (0x46c, 0x0024_e00c),
            (0x438, 0x0024_e018),
            (0x404, 0x0024_e024),
            (0x3d0, 0x0024_e030),
            (0x39c, 0x0024_e03c),
            (0x368, 0x0024_e048),
            (0x334, 0x0024_e054),
            (0x300, 0x0024_e060),
            (0x2cc, 0x0024_e06c),
            (0x298, 0x0024_e078),
            (0x264, 0x0024_e084),
            (0x230, 0x0024_e090),
            (0x1fc, 0x0024_e09c),
            (0x1c8, 0x0024_e0a8),
            (0x194, 0x0024_e0b4),
            (0x164, 0x0024_e0c0),
            (0x14c, 0x0024_e0cc),
            (0x130, 0x0024_e0d8),
        ];
        for (off, ret) in items {
            mdtor_item(c, mctor_addr(this, off), ret)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x002c_aa80, 0x0024_e0e4)?;
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x0);
        frame.close(c);
        Ok(())
    })
}

/// `cLightGlowMenu_dtor(this, flags)` (0x24dab8): tears down the 31 items of
/// the page in reverse build order, then the menu itself (0x2caa80).
pub fn light_glow_menu_dtor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 107;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0x0);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        let this = c.g[S0][0];
        let items: [(u32, u32); 31] = [
            (0x700, 0x0024_dadc),
            (0x6cc, 0x0024_dae8),
            (0x698, 0x0024_daf4),
            (0x664, 0x0024_db00),
            (0x630, 0x0024_db0c),
            (0x600, 0x0024_db18),
            (0x5cc, 0x0024_db24),
            (0x598, 0x0024_db30),
            (0x564, 0x0024_db3c),
            (0x530, 0x0024_db48),
            (0x4fc, 0x0024_db54),
            (0x4c8, 0x0024_db60),
            (0x494, 0x0024_db6c),
            (0x460, 0x0024_db78),
            (0x42c, 0x0024_db84),
            (0x3fc, 0x0024_db90),
            (0x3c8, 0x0024_db9c),
            (0x394, 0x0024_dba8),
            (0x360, 0x0024_dbb4),
            (0x32c, 0x0024_dbc0),
            (0x2f8, 0x0024_dbcc),
            (0x2c4, 0x0024_dbd8),
            (0x290, 0x0024_dbe4),
            (0x25c, 0x0024_dbf0),
            (0x228, 0x0024_dbfc),
            (0x1f4, 0x0024_dc08),
            (0x1c4, 0x0024_dc14),
            (0x194, 0x0024_dc20),
            (0x164, 0x0024_dc2c),
            (0x14c, 0x0024_dc38),
            (0x130, 0x0024_dc44),
        ];
        for (off, ret) in items {
            mdtor_item(c, mctor_addr(this, off), ret)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x002c_aa80, 0x0024_dc50)?;
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x0);
        frame.close(c);
        Ok(())
    })
}

/// `cScreenTintMenu_dtor(this, flags)` (0x24dd70): tears down the 42 items
/// of the page in reverse build order, then the menu itself (0x2caa80).
pub fn screen_tint_menu_dtor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 140;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0x0);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        let this = c.g[S0][0];
        let items: [(u32, u32); 42] = [
            (0x928, 0x0024_dd94),
            (0x8f4, 0x0024_dda0),
            (0x8c0, 0x0024_ddac),
            (0x88c, 0x0024_ddb8),
            (0x858, 0x0024_ddc4),
            (0x824, 0x0024_ddd0),
            (0x7f0, 0x0024_dddc),
            (0x7c0, 0x0024_dde8),
            (0x78c, 0x0024_ddf4),
            (0x758, 0x0024_de00),
            (0x724, 0x0024_de0c),
            (0x6f0, 0x0024_de18),
            (0x6bc, 0x0024_de24),
            (0x688, 0x0024_de30),
            (0x654, 0x0024_de3c),
            (0x620, 0x0024_de48),
            (0x5ec, 0x0024_de54),
            (0x5b8, 0x0024_de60),
            (0x584, 0x0024_de6c),
            (0x550, 0x0024_de78),
            (0x520, 0x0024_de84),
            (0x4f0, 0x0024_de90),
            (0x4c0, 0x0024_de9c),
            (0x490, 0x0024_dea8),
            (0x45c, 0x0024_deb4),
            (0x42c, 0x0024_dec0),
            (0x3fc, 0x0024_decc),
            (0x3c8, 0x0024_ded8),
            (0x394, 0x0024_dee4),
            (0x360, 0x0024_def0),
            (0x32c, 0x0024_defc),
            (0x2f8, 0x0024_df08),
            (0x2c4, 0x0024_df14),
            (0x290, 0x0024_df20),
            (0x25c, 0x0024_df2c),
            (0x228, 0x0024_df38),
            (0x1f4, 0x0024_df44),
            (0x1c4, 0x0024_df50),
            (0x194, 0x0024_df5c),
            (0x164, 0x0024_df68),
            (0x14c, 0x0024_df74),
            (0x130, 0x0024_df80),
        ];
        for (off, ret) in items {
            mdtor_item(c, mctor_addr(this, off), ret)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x002c_aa80, 0x0024_df8c)?;
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x0);
        frame.close(c);
        Ok(())
    })
}

/// `cMenuItem::~cMenuItem` (0x2ca280), the item destructor the menus chain.
pub(crate) const MENU_ITEM_DTOR_TARGET: u32 = 0x002c_a280;

/// `cMenu::~cMenu` (0x2caa80), the page destructor the menus end with.
pub(crate) const MENU_DTOR_TARGET: u32 = 0x002c_aa80;

/// An argument of one call in a destructor's straight-line body.
#[derive(Clone, Copy)]
pub(crate) enum MenuDtorArg {
    /// `this + off`, sign-extended (`addiu a0, s0, off`).
    This(u32),
    /// The value a saved register holds (`s0`..`s5`).
    Reg(usize),
    /// A constant.
    Imm(u64),
}

/// One step of a destructor body: a register set between two calls, or a
/// call (`a0`, `a1`, the function-table target and the return address).
/// `a1 == None` leaves `a1` as the previous callee left it, as the generated
/// body does.
#[derive(Clone, Copy)]
pub(crate) enum MenuDtorStep {
    /// `s<reg> = this + off`.
    Set(usize, u32),
    Call(MenuDtorArg, Option<MenuDtorArg>, u32, u32),
}

pub(crate) fn menu_dtor_arg(c: &Cpu, this: u32, arg: MenuDtorArg) -> u64 {
    match arg {
        MenuDtorArg::This(off) => sx(this.wrapping_add(off)),
        MenuDtorArg::Reg(reg) => c.g[reg][0],
        MenuDtorArg::Imm(v) => v,
    }
}

/// Runs a destructor's steps for the object at `this`.
pub(crate) fn run_menu_dtor_steps(c: &mut Cpu, this: u32, steps: &[MenuDtorStep]) -> Flow {
    for step in steps {
        match *step {
            MenuDtorStep::Set(reg, off) => c.g[reg][0] = sx(this.wrapping_add(off)),
            MenuDtorStep::Call(a0, a1, target, ret_addr) => {
                c.g[A0][0] = menu_dtor_arg(c, this, a0);
                if let Some(a1) = a1 {
                    c.g[A1][0] = menu_dtor_arg(c, this, a1);
                }
                call_ptr(c, target, ret_addr)?;
            }
        }
    }
    Ok(())
}

/// The steps of `camera_splash_menu_dtor`, in the generated order.
pub(crate) const CAMERA_SPLASH_MENU_DTOR_STEPS: &[MenuDtorStep] = &[
    MenuDtorStep::Set(S1, 0x1eec),
    MenuDtorStep::Set(S2, 0x1890),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x252c),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e3c4,
    ),
    MenuDtorStep::Set(S3, 0xe58),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x2508),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e3d4,
    ),
    MenuDtorStep::Set(S4, 0x7fc),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x24e4),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e3e4,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x24c0),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e3f0,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x249c),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e3fc,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x1f04),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_DTOR_TARGET,
        0x0024e408,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::Reg(S1),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e414,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x1ed0),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e420,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x1eac),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e42c,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x1e88),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e438,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x1e64),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e444,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x1e40),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e450,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x18a8),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_DTOR_TARGET,
        0x0024e45c,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::Reg(S2),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e468,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x185c),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e474,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x1828),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e480,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x17f4),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e48c,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x17c0),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e498,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x178c),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e4a4,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x1758),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e4b0,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x1724),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e4bc,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x16f0),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e4c8,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x16bc),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e4d4,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x1688),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e4e0,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x1654),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e4ec,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x1620),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e4f8,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x15ec),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e504,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x15b8),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e510,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x1584),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e51c,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x1550),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e528,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x151c),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e534,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x14e8),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e540,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x14b4),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e54c,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x1498),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e558,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x1474),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e564,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x1450),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e570,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x142c),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e57c,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x1408),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e588,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0xe70),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_DTOR_TARGET,
        0x0024e594,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::Reg(S3),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e5a0,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0xe3c),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e5ac,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0xe18),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e5b8,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0xdf4),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e5c4,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0xdd0),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e5d0,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0xdac),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e5dc,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x814),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_DTOR_TARGET,
        0x0024e5e8,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::Reg(S4),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e5f4,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x7c8),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e600,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x794),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e60c,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x760),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e618,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x72c),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e624,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x6f8),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e630,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x6c4),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e63c,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x690),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e648,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x65c),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e654,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x628),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e660,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x5f4),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e66c,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x5c0),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e678,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x58c),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e684,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x558),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e690,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x524),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e69c,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x4f0),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e6a8,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x4bc),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e6b4,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x488),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e6c0,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x454),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e6cc,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x420),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e6d8,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x3ec),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e6e4,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x3b8),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e6f0,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x384),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e6fc,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x350),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e708,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x31c),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e714,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x2ec),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e720,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x2bc),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e72c,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x28c),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e738,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x258),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e744,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x224),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e750,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x1f4),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e75c,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x1c4),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e768,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x194),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e774,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x164),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e780,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x14c),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e78c,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x130),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e798,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::Reg(S0),
        Some(MenuDtorArg::Reg(S5)),
        MENU_DTOR_TARGET,
        0x0024e7a4,
    ),
];

/// The steps of `path_arrow_menu_dtor`, in the generated order.
pub(crate) const PATH_ARROW_MENU_DTOR_STEPS: &[MenuDtorStep] = &[
    MenuDtorStep::Call(
        MenuDtorArg::This(0x458),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e7ec,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x428),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e7f8,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x3f4),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e804,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x3c0),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e810,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x38c),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e81c,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x35c),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e828,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x32c),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e834,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x2fc),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e840,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x2cc),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e84c,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x298),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e858,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x264),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e864,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x230),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e870,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x1fc),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e87c,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x1c8),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e888,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x194),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e894,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x164),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e8a0,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x14c),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e8ac,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::This(0x130),
        Some(MenuDtorArg::Imm(0x2)),
        MENU_ITEM_DTOR_TARGET,
        0x0024e8b8,
    ),
    MenuDtorStep::Call(
        MenuDtorArg::Reg(S0),
        Some(MenuDtorArg::Reg(S1)),
        MENU_DTOR_TARGET,
        0x0024e8c4,
    ),
];

/// `void CameraSplashMenu_dtor(CameraSplashMenu *this, int flags)` (0x24e388):
/// destroys the crystal, drop and snowfall option items (flags 2 each) and
/// the page itself with the caller's flags.
pub fn camera_splash_menu_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 272;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S5, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save_ra(c, 0);
        let this = r32(c, S0);
        run_menu_dtor_steps(c, this, CAMERA_SPLASH_MENU_DTOR_STEPS)?;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PathArrowMenu_dtor(PathArrowMenu *this, int flags)` (0x24e7c8):
/// destroys the path-arrow option items (flags 2 each), then the page with
/// the caller's flags.
pub fn path_arrow_menu_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 68;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let this = r32(c, S0);
        run_menu_dtor_steps(c, this, PATH_ARROW_MENU_DTOR_STEPS)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RenderTogglesMenu_OnActivate(void)` (0x24f7e0): `MenuSysPage_OnActivate(this)`, a 7-cycle forwarding wrapper
/// (`this` passes through in `a0`).
pub fn render_toggles_menu_on_activate(c: &mut Cpu) {
    call::tail_wrapper(c, None, call::Target::Table(0x002cbf60), 0x0024f7f0)
}

/// `void RenderTogglesMenu_OnDeactivate(void)` (0x24f800): `MenuSysPage_OnDeactivate(this)`, a 7-cycle forwarding wrapper
/// (`this` passes through in `a0`).
pub fn render_toggles_menu_on_deactivate(c: &mut Cpu) {
    call::tail_wrapper(c, None, call::Target::Table(0x002cbfd0), 0x0024f810)
}

/// `void VisualEffectsTestMenu_OnActivate(void)` (0x24f820): `MenuSysPage_OnActivate(this)`, a 7-cycle forwarding wrapper
/// (`this` passes through in `a0`).
pub fn visual_effects_test_menu_on_activate(c: &mut Cpu) {
    call::tail_wrapper(c, None, call::Target::Table(0x002cbf60), 0x0024f830)
}

/// `void VisualEffectsTestMenu_OnDeactivate(void)` (0x24f840): `MenuSysPage_OnDeactivate(this)`, a 7-cycle forwarding wrapper
/// (`this` passes through in `a0`).
pub fn visual_effects_test_menu_on_deactivate(c: &mut Cpu) {
    call::tail_wrapper(c, None, call::Target::Table(0x002cbfd0), 0x0024f850)
}

/// `void PlantsMenu_OnActivate(void)` (0x24f940): `MenuSysPage_OnActivate(this)`, a 7-cycle forwarding wrapper
/// (`this` passes through in `a0`).
pub fn plants_menu_on_activate(c: &mut Cpu) {
    call::tail_wrapper(c, None, call::Target::Table(0x002cbf60), 0x0024f950)
}

/// `void PlantsMenu_OnDeactivate(void)` (0x24f960): `MenuSysPage_OnDeactivate(this)`, a 7-cycle forwarding wrapper
/// (`this` passes through in `a0`).
pub fn plants_menu_on_deactivate(c: &mut Cpu) {
    call::tail_wrapper(c, None, call::Target::Table(0x002cbfd0), 0x0024f970)
}

/// `void LightGlowMenu_OnActivate(void)` (0x24f980): `MenuSysPage_OnActivate(this)`, a 7-cycle forwarding wrapper
/// (`this` passes through in `a0`).
pub fn light_glow_menu_on_activate(c: &mut Cpu) {
    call::tail_wrapper(c, None, call::Target::Table(0x002cbf60), 0x0024f990)
}

/// `void LightGlowMenu_OnDeactivate(void)` (0x24f9a0): `MenuSysPage_OnDeactivate(this)`, a 7-cycle forwarding wrapper
/// (`this` passes through in `a0`).
pub fn light_glow_menu_on_deactivate(c: &mut Cpu) {
    call::tail_wrapper(c, None, call::Target::Table(0x002cbfd0), 0x0024f9b0)
}

/// `void DepthFogMenu_OnActivate(void)` (0x24f9c0): `MenuSysPage_OnActivate(this)`, a 7-cycle forwarding wrapper
/// (`this` passes through in `a0`).
pub fn depth_fog_menu_on_activate(c: &mut Cpu) {
    call::tail_wrapper(c, None, call::Target::Table(0x002cbf60), 0x0024f9d0)
}

/// `void DepthFogMenu_OnDeactivate(void)` (0x24f9e0): `MenuSysPage_OnDeactivate(this)`, a 7-cycle forwarding wrapper
/// (`this` passes through in `a0`).
pub fn depth_fog_menu_on_deactivate(c: &mut Cpu) {
    call::tail_wrapper(c, None, call::Target::Table(0x002cbfd0), 0x0024f9f0)
}

/// `void HeightFogMenu_OnActivate(void)` (0x24fa78): `MenuSysPage_OnActivate(this)`, a 7-cycle forwarding wrapper
/// (`this` passes through in `a0`).
pub fn height_fog_menu_on_activate(c: &mut Cpu) {
    call::tail_wrapper(c, None, call::Target::Table(0x002cbf60), 0x0024fa88)
}

/// `void HeightFogMenu_OnDeactivate(void)` (0x24fa98): `MenuSysPage_OnDeactivate(this)`, a 7-cycle forwarding wrapper
/// (`this` passes through in `a0`).
pub fn height_fog_menu_on_deactivate(c: &mut Cpu) {
    call::tail_wrapper(c, None, call::Target::Table(0x002cbfd0), 0x0024faa8)
}

/// `void DetailSystemMenu_OnActivate(void)` (0x24fb80): `MenuSysPage_OnActivate(this)`, a 7-cycle forwarding wrapper
/// (`this` passes through in `a0`).
pub fn detail_system_menu_on_activate(c: &mut Cpu) {
    call::tail_wrapper(c, None, call::Target::Table(0x002cbf60), 0x0024fb90)
}

/// `void DetailSystemMenu_OnDeactivate(void)` (0x24fba0): `MenuSysPage_OnDeactivate(this)`, a 7-cycle forwarding wrapper
/// (`this` passes through in `a0`).
pub fn detail_system_menu_on_deactivate(c: &mut Cpu) {
    call::tail_wrapper(c, None, call::Target::Table(0x002cbfd0), 0x0024fbb0)
}

/// `void ShellRendererMenu_OnActivate(void)` (0x24fc80): `MenuSysPage_OnActivate(this)`, a 7-cycle forwarding wrapper
/// (`this` passes through in `a0`).
pub fn shell_renderer_menu_on_activate(c: &mut Cpu) {
    call::tail_wrapper(c, None, call::Target::Table(0x002cbf60), 0x0024fc90)
}

/// `void ShellRendererMenu_OnDeactivate(void)` (0x24fca0): `MenuSysPage_OnDeactivate(this)`, a 7-cycle forwarding wrapper
/// (`this` passes through in `a0`).
pub fn shell_renderer_menu_on_deactivate(c: &mut Cpu) {
    call::tail_wrapper(c, None, call::Target::Table(0x002cbfd0), 0x0024fcb0)
}

/// `void ScreenTintMenu_OnActivate(void)` (0x24fcc0): `MenuSysPage_OnActivate(this)`, a 7-cycle forwarding wrapper
/// (`this` passes through in `a0`).
pub fn screen_tint_menu_on_activate(c: &mut Cpu) {
    call::tail_wrapper(c, None, call::Target::Table(0x002cbf60), 0x0024fcd0)
}

/// `void ScreenTintMenu_OnDeactivate(void)` (0x24fce0): `MenuSysPage_OnDeactivate(this)`, a 7-cycle forwarding wrapper
/// (`this` passes through in `a0`).
pub fn screen_tint_menu_on_deactivate(c: &mut Cpu) {
    call::tail_wrapper(c, None, call::Target::Table(0x002cbfd0), 0x0024fcf0)
}

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

/// `addiu rt, rs, imm`: `rs + imm`, sign-extended to 64 bits.
pub(crate) fn avalanche_ctor_addiu(c: &Cpu, rs: usize, imm: i32) -> u64 {
    sx((c.g[rs][0] as u32).wrapping_add(imm as u32))
}

/// The 32-bit address `rs + off`.
pub(crate) fn avalanche_ctor_at(c: &Cpu, rs: usize, off: u32) -> u32 {
    (c.g[rs][0] as u32).wrapping_add(off)
}

/// `cAvalancheMenu::cAvalancheMenu(this)` (0x2501e8): the menu's base
/// constructor, its vtable, then the title, the spacer and one item per
/// tuning value (int or float, each with its variable and range), and
/// finally every item added to the page in order. Returns `this`.
/// Stack: 0x130 bytes; the item addresses are kept in 0x4-byte slots at
/// `sp+0x0..` (title at slot 0, then one slot per item).
pub fn avalanche_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 508;
        let frame = Frame::open(c, 0x130);
        frame.save(c, S0, 0x100);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0xf0);
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 304);
        frame.save(c, S2, 0xe0);
        frame.save(c, S3, 0xd0);
        frame.save(c, S4, 0xc0);
        c.g[S2][0] = avalanche_ctor_addiu(c, S0, 1812);
        frame.save(c, S5, 0xb0);
        c.g[S4][0] = avalanche_ctor_addiu(c, S0, 1760);
        frame.save(c, S6, 0xa0);
        c.g[S5][0] = avalanche_ctor_addiu(c, S0, 1556);
        frame.save(c, S7, 0x90);
        c.g[S6][0] = avalanche_ctor_addiu(c, S0, 1660);
        frame.save(c, FP, 0x80);
        c.g[S7][0] = avalanche_ctor_addiu(c, S0, 1712);
        frame.save_ra(c, 0x70);
        c.g[FP][0] = avalanche_ctor_addiu(c, S0, 1452);
        frame.save_fpr(c, 23, 0x128);
        c.g[S3][0] = avalanche_ctor_addiu(c, S0, 1864);
        frame.save_fpr(c, 22, 0x120);
        c.g[S1][0] = avalanche_ctor_addiu(c, S0, 1916);
        frame.save_fpr(c, 21, 0x118);
        frame.save_fpr(c, 20, 0x110);
        c.sw(frame.at(c, 0), c.g[V0][0]);
        call_ptr(c, 0x002caa58, 0x00250254)?; // cMenu_cMenu (MenuSysPage_ctor)
        c.mtc1(0, 20);
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 332);
        c.g[A1][0] = 0x480000;
        c.sw(frame.at(c, 0x4), c.g[V0][0]);
        c.g[A1][0] = avalanche_ctor_addiu(c, A1, -640);
        c.g[V0][0] = 0x480000;
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 21);
        c.g[V0][0] = avalanche_ctor_addiu(c, V0, 64);
        c.g[AT][0] = 0x447a0000;
        c.mtc1(1, 22);
        c.sw(avalanche_ctor_at(c, S0, 0x12c), c.g[V0][0]);
        c.g[AT][0] = 0x41200000;
        c.mtc1(1, 23);
        c.g[A0][0] = c.lw(frame.at(c, 0));
        call_ptr(c, 0x002cd2a0, 0x00250294)?; // cNullMenuItem ctor: TitleItem
        c.g[A0][0] = c.lw(frame.at(c, 0x4));
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 356);
        c.sw(frame.at(c, 0x8), c.g[V0][0]);
        c.g[A1][0] = 0x28;
        call_ptr(c, 0x002cd350, 0x002502a8)?; // cSpaceMenuItem ctor: SpaceItem
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 404);
        c.g[A0][0] = c.lw(frame.at(c, 0x8));
        c.sw(frame.at(c, 0xc), c.g[V0][0]);
        c.g[T1][0] = 0x4a0000;
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 452);
        c.g[T1][0] = avalanche_ctor_addiu(c, T1, 11760);
        c.sw(frame.at(c, 0x10), c.g[V0][0]);
        c.g[A1][0] = 0u64;
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2368);
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0x1;
        call_ptr(c, 0x002cd8f8, 0x002502d8)?; // cIntMenuItem ctor: EnableItem
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 504);
        c.g[A0][0] = c.lw(frame.at(c, 0xc));
        c.sw(frame.at(c, 0x14), c.g[V0][0]);
        c.g[T1][0] = 0x4a0000;
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 552);
        c.g[T1][0] = avalanche_ctor_addiu(c, T1, 11768);
        c.sw(frame.at(c, 0x18), c.g[V0][0]);
        c.g[T0][0] = 0x1;
        c.g[A3][0] = 0xffffffffffffffff;
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2372);
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x002cd8f8, 0x00250308)?; // cIntMenuItem ctor: RecalcItem
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 600);
        c.g[A0][0] = c.lw(frame.at(c, 0x10));
        c.sw(frame.at(c, 0x1c), c.g[V0][0]);
        c.g[A3][0] = 0x480000;
        c.g[AT][0] = 0x40800000;
        c.mtc1(1, 13);
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 648);
        c.sw(frame.at(c, 0x20), c.g[V0][0]);
        c.mov_s(12, 20);
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -624);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2376);
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002cdce8, 0x0025033c)?; // cFloatMenuItem ctor: BrightnessItem
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 696);
        c.g[A0][0] = c.lw(frame.at(c, 0x14));
        c.sw(frame.at(c, 0x24), c.g[V0][0]);
        c.g[T1][0] = 0x480000;
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 744);
        c.g[T1][0] = avalanche_ctor_addiu(c, T1, -608);
        c.sw(frame.at(c, 0x28), c.g[V0][0]);
        c.g[A1][0] = 0x3;
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2380);
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0x1;
        call_ptr(c, 0x002cd8f8, 0x0025036c)?; // cIntMenuItem ctor: UseLightMapColourItem
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 796);
        c.g[A0][0] = c.lw(frame.at(c, 0x18));
        c.sw(frame.at(c, 0x2c), c.g[V0][0]);
        c.g[T1][0] = 0x480000;
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 844);
        c.g[T1][0] = avalanche_ctor_addiu(c, T1, -584);
        c.sw(frame.at(c, 0x30), c.g[V0][0]);
        c.g[A1][0] = 0x4;
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2384);
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0x1;
        call_ptr(c, 0x002cd8f8, 0x0025039c)?; // cIntMenuItem ctor: UseEndColourItem
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 892);
        c.g[A0][0] = c.lw(frame.at(c, 0x1c));
        c.sw(frame.at(c, 0x34), c.g[V0][0]);
        c.g[T1][0] = 0x480000;
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 940);
        c.g[T1][0] = avalanche_ctor_addiu(c, T1, -568);
        c.sw(frame.at(c, 0x38), c.g[V0][0]);
        c.g[A1][0] = 0x5;
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2388);
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0x1;
        call_ptr(c, 0x002cd8f8, 0x002503cc)?; // cIntMenuItem ctor: SquareLerpItem
        c.g[A0][0] = c.lw(frame.at(c, 0x20));
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 988);
        c.g[T1][0] = 0x4a0000;
        c.sw(frame.at(c, 0x3c), c.g[V0][0]);
        c.g[T1][0] = avalanche_ctor_addiu(c, T1, 11776);
        c.g[A1][0] = 0x6;
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2392);
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0x1;
        call_ptr(c, 0x002cd8f8, 0x002503f4)?; // cIntMenuItem ctor: SaveItem
        c.g[A0][0] = c.lw(frame.at(c, 0x24));
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 1040);
        c.g[T1][0] = 0x4a0000;
        c.sw(frame.at(c, 0x40), c.g[V0][0]);
        c.g[T1][0] = avalanche_ctor_addiu(c, T1, 11784);
        c.g[T0][0] = 0x1;
        c.g[A3][0] = 0u64;
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2396);
        c.g[A1][0] = 0x7;
        call_ptr(c, 0x002cd8f8, 0x0025041c)?; // cIntMenuItem ctor: LoadItem
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 1092);
        c.g[A0][0] = c.lw(frame.at(c, 0x28));
        c.sw(frame.at(c, 0x44), c.g[V0][0]);
        c.g[A3][0] = 0x480000;
        c.g[AT][0] = 0x42f00000;
        c.mtc1(1, 13);
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 1144);
        c.sw(frame.at(c, 0x48), c.g[V0][0]);
        c.mov_s(12, 20);
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -544);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2400);
        c.g[A1][0] = 0x8;
        call_ptr(c, 0x002cdce8, 0x00250450)?; // cFloatMenuItem ctor: MinImpactVelocityItem
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 1196);
        c.g[A0][0] = c.lw(frame.at(c, 0x2c));
        c.sw(frame.at(c, 0x4c), c.g[V0][0]);
        c.g[T1][0] = 0x480000;
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 1248);
        c.g[T1][0] = avalanche_ctor_addiu(c, T1, -520);
        c.sw(frame.at(c, 0x50), c.g[V0][0]);
        c.g[A1][0] = 0x9;
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2404);
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0x1;
        call_ptr(c, 0x002cd8f8, 0x00250480)?; // cIntMenuItem ctor: RenderEmittersItem
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 1300);
        c.g[A0][0] = c.lw(frame.at(c, 0x30));
        c.sw(frame.at(c, 0x54), c.g[V0][0]);
        c.g[T1][0] = 0x480000;
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 1348);
        c.g[T1][0] = avalanche_ctor_addiu(c, T1, -504);
        c.sw(frame.at(c, 0x58), c.g[V0][0]);
        c.g[A1][0] = 0xa;
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2408);
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0x3f;
        call_ptr(c, 0x002cd8f8, 0x002504b0)?; // cIntMenuItem ctor: RenderLinesItem
        c.g[A0][0] = c.lw(frame.at(c, 0x34));
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 1400);
        c.g[T1][0] = 0x480000;
        c.sw(frame.at(c, 0x5c), c.g[V0][0]);
        c.g[T1][0] = avalanche_ctor_addiu(c, T1, -488);
        c.g[A1][0] = 0xb;
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2412);
        c.g[A3][0] = 0xffffffffffffffff;
        c.g[T0][0] = 0x1;
        call_ptr(c, 0x002cd8f8, 0x002504d8)?; // cIntMenuItem ctor: TriggerItem
        c.g[A0][0] = c.lw(frame.at(c, 0x38));
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 1504);
        c.g[T1][0] = 0x4a0000;
        c.sw(frame.at(c, 0x60), c.g[V0][0]);
        c.g[T1][0] = avalanche_ctor_addiu(c, T1, 11792);
        c.g[T0][0] = 0x1;
        c.g[A3][0] = 0xffffffffffffffff;
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2416);
        c.g[A1][0] = 0xc;
        call_ptr(c, 0x002cd8f8, 0x00250500)?; // cIntMenuItem ctor: UpdateItem
        c.g[A0][0] = c.lw(frame.at(c, 0x3c));
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 1608);
        c.g[A3][0] = 0x480000;
        c.sw(frame.at(c, 0x64), c.g[V0][0]);
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -464);
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2420);
        c.g[A1][0] = 0xd;
        call_ptr(c, 0x002cdce8, 0x00250528)?; // cFloatMenuItem ctor: UpdateTimeStepItem
        c.g[A0][0] = c.lw(frame.at(c, 0x40));
        c.g[A3][0] = 0x480000;
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -440);
        c.mov_s(12, 20);
        c.mov_s(13, 22);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2424);
        c.g[A1][0] = 0xe;
        call_ptr(c, 0x002cdce8, 0x00250548)?; // cFloatMenuItem ctor: RotationalVelocityScaleItem
        c.g[A0][0] = c.lw(frame.at(c, 0x44));
        c.g[A3][0] = 0x480000;
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -408);
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2428);
        c.g[A1][0] = 0xf;
        call_ptr(c, 0x002cdce8, 0x00250568)?; // cFloatMenuItem ctor: RandomBounceChanceItem
        c.g[A0][0] = c.lw(frame.at(c, 0x48));
        c.g[A3][0] = 0x480000;
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -384);
        c.mov_s(12, 20);
        c.mov_s(13, 22);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2432);
        c.g[A1][0] = 0x10;
        call_ptr(c, 0x002cdce8, 0x00250588)?; // cFloatMenuItem ctor: RandomBounceVelocityScaleXYItem
        c.g[A0][0] = c.lw(frame.at(c, 0x4c));
        c.g[A3][0] = 0x480000;
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -352);
        c.mov_s(12, 20);
        c.mov_s(13, 22);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2436);
        c.g[A1][0] = 0x11;
        call_ptr(c, 0x002cdce8, 0x002505a8)?; // cFloatMenuItem ctor: RandomBounceVelocityScaleZItem
        c.g[A0][0] = c.lw(frame.at(c, 0x50));
        c.g[A3][0] = 0x480000;
        c.mov_s(12, 21);
        c.mov_s(13, 22);
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -320);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2440);
        c.g[A1][0] = 0x12;
        call_ptr(c, 0x002cdce8, 0x002505c8)?; // cFloatMenuItem ctor: RandomBounceVelocityScaleItem
        c.g[A0][0] = c.lw(frame.at(c, 0x54));
        c.g[T1][0] = 0x480000;
        c.g[T1][0] = avalanche_ctor_addiu(c, T1, -288);
        c.g[T0][0] = 0x3e8;
        c.g[A3][0] = 0u64;
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2444);
        c.g[A1][0] = 0x13;
        call_ptr(c, 0x002cd8f8, 0x002505e8)?; // cIntMenuItem ctor: AvalancheIdItem
        c.g[A0][0] = c.lw(frame.at(c, 0x58));
        c.g[A3][0] = 0x4a0000;
        c.g[AT][0] = 0x42fe0000;
        c.mtc1(1, 13);
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, 11800);
        c.mov_s(12, 20);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2448);
        c.g[A1][0] = 0x14;
        call_ptr(c, 0x002cdce8, 0x0025060c)?; // cFloatMenuItem ctor: GravityItem
        c.g[A0][0] = c.lw(frame.at(c, 0x5c));
        c.g[A3][0] = 0x480000;
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -272);
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2452);
        c.g[A1][0] = 0x15;
        call_ptr(c, 0x002cdce8, 0x0025062c)?; // cFloatMenuItem ctor: VelocityDampingItem
        c.g[A3][0] = 0x480000;
        c.g[A0][0] = c.g[FP][0];
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -248);
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2456);
        c.g[A1][0] = 0x16;
        call_ptr(c, 0x002cdce8, 0x0025064c)?; // cFloatMenuItem ctor: NormalVelocityScaleItem
        c.g[A0][0] = c.lw(frame.at(c, 0x60));
        c.g[A3][0] = 0x480000;
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -224);
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2460);
        c.g[A1][0] = 0x17;
        call_ptr(c, 0x002cdce8, 0x0025066c)?; // cFloatMenuItem ctor: ForwardVelocityScaleItem
        c.g[A3][0] = 0x480000;
        c.g[A0][0] = c.g[S5][0];
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -200);
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2464);
        c.g[A1][0] = 0x18;
        call_ptr(c, 0x002cdce8, 0x0025068c)?; // cFloatMenuItem ctor: NormalToForwardScaleItem
        c.g[A0][0] = c.lw(frame.at(c, 0x64));
        c.g[A3][0] = 0x480000;
        c.g[AT][0] = 0x42ee0000;
        c.mtc1(1, 13);
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -176);
        c.mov_s(12, 21);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2468);
        c.g[A1][0] = 0x19;
        call_ptr(c, 0x002cdce8, 0x002506b0)?; // cFloatMenuItem ctor: MaxVelocityItem
        c.g[A3][0] = 0x4a0000;
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A0][0] = c.g[S6][0];
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, 11808);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2472);
        c.g[A1][0] = 0x1a;
        call_ptr(c, 0x002cdce8, 0x002506d0)?; // cFloatMenuItem ctor: DampingItem
        c.g[T1][0] = 0x480000;
        c.g[T0][0] = 0x400;
        c.g[T1][0] = avalanche_ctor_addiu(c, T1, -160);
        c.g[A0][0] = c.g[S7][0];
        c.g[A3][0] = 0x1;
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2476);
        c.g[A1][0] = 0x1b;
        call_ptr(c, 0x002cd8f8, 0x002506f0)?; // cIntMenuItem ctor: NumParticlesItem
        c.g[A3][0] = 0x480000;
        c.mov_s(12, 21);
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -144);
        c.g[A0][0] = c.g[S4][0];
        c.mov_s(13, 22);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2480);
        c.g[A1][0] = 0x1c;
        call_ptr(c, 0x002cdce8, 0x00250710)?; // cFloatMenuItem ctor: DefaultSizeItem
        c.g[A3][0] = 0x480000;
        c.g[A0][0] = c.g[S2][0];
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -128);
        c.mov_s(12, 20);
        c.mov_s(13, 23);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2484);
        c.g[A1][0] = 0x1d;
        call_ptr(c, 0x002cdce8, 0x00250730)?; // cFloatMenuItem ctor: PatchColourScaleItem
        c.g[A3][0] = 0x480000;
        c.mov_s(12, 20);
        c.mov_s(13, 23);
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -104);
        c.g[A0][0] = c.g[S3][0];
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2488);
        c.g[A1][0] = 0x1e;
        call_ptr(c, 0x002cdce8, 0x00250750)?; // cFloatMenuItem ctor: PatchColourScale2Item
        c.g[AT][0] = 0xffffffffbf800000;
        c.mtc1(1, 12);
        c.g[A3][0] = 0x4a0000;
        c.mov_s(13, 22);
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, 11816);
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2492);
        c.g[A1][0] = 0x1f;
        call_ptr(c, 0x002cdce8, 0x00250774)?; // cFloatMenuItem ctor: SeedItem
        c.g[A1][0] = c.lw(frame.at(c, 0));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250784)?; // cMenu_addItem: TitleItem
        c.g[A1][0] = c.lw(frame.at(c, 0x4));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250794)?; // cMenu_addItem: SpaceItem
        c.g[A1][0] = c.lw(frame.at(c, 0x8));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002507a4)?; // cMenu_addItem: EnableItem
        c.g[A1][0] = c.lw(frame.at(c, 0xc));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002507b4)?; // cMenu_addItem: RecalcItem
        c.g[A1][0] = c.lw(frame.at(c, 0x10));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002507c4)?; // cMenu_addItem: BrightnessItem
        c.g[A1][0] = c.lw(frame.at(c, 0x14));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002507d4)?; // cMenu_addItem: UseLightMapColourItem
        c.g[A1][0] = c.lw(frame.at(c, 0x18));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002507e4)?; // cMenu_addItem: UseEndColourItem
        c.g[A1][0] = c.lw(frame.at(c, 0x1c));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002507f4)?; // cMenu_addItem: SquareLerpItem
        c.g[A1][0] = c.lw(frame.at(c, 0x20));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250804)?; // cMenu_addItem: SaveItem
        c.g[A1][0] = c.lw(frame.at(c, 0x24));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250814)?; // cMenu_addItem: LoadItem
        c.g[A1][0] = c.lw(frame.at(c, 0x28));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250824)?; // cMenu_addItem: MinImpactVelocityItem
        c.g[A1][0] = c.lw(frame.at(c, 0x2c));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250834)?; // cMenu_addItem: RenderEmittersItem
        c.g[A1][0] = c.lw(frame.at(c, 0x30));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250844)?; // cMenu_addItem: RenderLinesItem
        c.g[A1][0] = c.lw(frame.at(c, 0x34));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250854)?; // cMenu_addItem: TriggerItem
        c.g[A1][0] = c.lw(frame.at(c, 0x38));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250864)?; // cMenu_addItem: UpdateItem
        c.g[A1][0] = c.lw(frame.at(c, 0x3c));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250874)?; // cMenu_addItem: UpdateTimeStepItem
        c.g[A1][0] = c.lw(frame.at(c, 0x40));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250884)?; // cMenu_addItem: RotationalVelocityScaleItem
        c.g[A1][0] = c.lw(frame.at(c, 0x44));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250894)?; // cMenu_addItem: RandomBounceChanceItem
        c.g[A1][0] = c.lw(frame.at(c, 0x48));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002508a4)?; // cMenu_addItem: RandomBounceVelocityScaleXYItem
        c.g[A1][0] = c.lw(frame.at(c, 0x4c));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002508b4)?; // cMenu_addItem: RandomBounceVelocityScaleZItem
        c.g[A1][0] = c.lw(frame.at(c, 0x50));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002508c4)?; // cMenu_addItem: RandomBounceVelocityScaleItem
        c.g[A1][0] = c.lw(frame.at(c, 0x54));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002508d4)?; // cMenu_addItem: AvalancheIdItem
        c.g[A1][0] = c.lw(frame.at(c, 0x58));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002508e4)?; // cMenu_addItem: GravityItem
        c.g[A1][0] = c.lw(frame.at(c, 0x5c));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002508f4)?; // cMenu_addItem: VelocityDampingItem
        c.g[A1][0] = c.g[FP][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250904)?; // cMenu_addItem: NormalVelocityScaleItem
        c.g[A1][0] = c.lw(frame.at(c, 0x60));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250914)?; // cMenu_addItem: ForwardVelocityScaleItem
        c.g[A1][0] = c.g[S5][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250924)?; // cMenu_addItem: NormalToForwardScaleItem
        c.g[A1][0] = c.lw(frame.at(c, 0x64));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250934)?; // cMenu_addItem: MaxVelocityItem
        c.g[A1][0] = c.g[S6][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250944)?; // cMenu_addItem: DampingItem
        c.g[A1][0] = c.g[S7][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250954)?; // cMenu_addItem: NumParticlesItem
        c.g[A1][0] = c.g[S4][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250964)?; // cMenu_addItem: DefaultSizeItem
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250974)?; // cMenu_addItem: PatchColourScaleItem
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250984)?; // cMenu_addItem: PatchColourScale2Item
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250994)?; // cMenu_addItem: SeedItem
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0xf0);
        frame.restore(c, S0, 0x100);
        frame.restore(c, S2, 0xe0);
        frame.restore(c, S3, 0xd0);
        frame.restore(c, S4, 0xc0);
        frame.restore(c, S5, 0xb0);
        frame.restore(c, S6, 0xa0);
        frame.restore(c, S7, 0x90);
        frame.restore(c, FP, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 23, 0x128);
        frame.restore_fpr(c, 22, 0x120);
        frame.restore_fpr(c, 21, 0x118);
        frame.restore_fpr(c, 20, 0x110);
        frame.close(c);
        Ok(())
    });
}

/// `AIVisualEffectsMenus_StaticInitDestroy(int prio, int init)` (0x250cb0, the
/// `__sti__aivisualeffectsmenu_cpp` static initialiser/destructor of the
/// visual-effects debug menu). Only `prio == 0xffff` does anything: with
/// `init != 0` it fills the sV4 colour table (0x4d1968..0x4d1c78) and the
/// sSlot array (0x4a5060..0x4a50d0), then constructs the static
/// `AvalancheMenu` at 0x4d1c88; with `init == 0` it destroys that menu. The
/// frame is 0xa0 bytes; s16-s23 and s30 are saved.
///
/// The table fills are 254 stores into fixed addresses (`gp`-relative for the
/// flags at 0x1f70..0x1fe0), transcribed from the generated body in order, so
/// the final value of every register and the cycle charge stay as the original
/// leaves them.
pub fn aivisual_effects_menus_static_init_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = 0xffff;
        frame.save(c, 16, 0x90);
        frame.save(c, 17, 0x80);
        frame.save(c, 18, 0x70);
        frame.save(c, 19, 0x60);
        frame.save(c, 20, 0x50);
        frame.save(c, 21, 0x40);
        frame.save(c, 22, 0x30);
        frame.save(c, 23, 0x20);
        frame.save(c, 30, 0x10);
        let prio_not_ffff = c.g[A1][0] != c.g[V0][0];
        frame.save_ra(c, 0);
        if prio_not_ffff {
            c.cycles += 1;
            frame.restore(c, 16, 0x90);
        } else {
            c.cycles += 2;
            let a0_zero = c.g[A0][0] == 0u64;
            c.g[T5][0] = 0x4d0000;
            if !a0_zero {
                // init != 0: the table fills, then the AvalancheMenu constructor.
                c.cycles += 561;
                c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x4ec0));
                c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x4ebc));
                c.g[T6][0] = 0x4d0000;
                c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x4eb8));
                c.g[T7][0] = 0x4d0000;
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4eb4));
                c.g[V0][0] = sx((c.g[T7][0] as u32).wrapping_add(0x1938));
                c.swc1(9, (c.g[T5][0] as u32).wrapping_add(0x1918));
                c.g[S1][0] = 0x4d0000;
                c.swc1(10, (c.g[T6][0] as u32).wrapping_add(0x1928));
                c.g[S2][0] = 0x4d0000;
                c.swc1(11, (c.g[T7][0] as u32).wrapping_add(0x1938));
                c.g[S3][0] = 0x4d0000;
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x4eb0));
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1968));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4eac));
                c.g[S5][0] = 0x4d0000;
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4ea8));
                c.g[S4][0] = 0x4d0000;
                c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[T3][0] = 0x4d0000;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4ea4));
                c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1908));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.g[A3][0] = sx((c.g[T5][0] as u32).wrapping_add(0x1918));
                c.swc1(8, (c.g[S1][0] as u32).wrapping_add(0x1948));
                c.g[T1][0] = sx((c.g[T6][0] as u32).wrapping_add(0x1928));
                c.swc1(4, (c.g[S2][0] as u32).wrapping_add(0x1958));
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1948));
                c.swc1(5, (c.g[S3][0] as u32).wrapping_add(0x1968));
                c.g[T0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1958));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4ea0));
                c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1978));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4e9c));
                c.g[T4][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1988));
                c.swc1(2, (c.g[A0][0] as u32).wrapping_add(0xc));
                c.g[S2][0] = 0x4d0000;
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 6);
                c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x19e8));
                c.swc1(3, (c.g[S4][0] as u32).wrapping_add(0x1978));
                c.g[T9][0] = 0x4d0000;
                c.swc1(1, (c.g[T3][0] as u32).wrapping_add(0x1988));
                c.g[S3][0] = 0x4d0000;
                c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x1908));
                c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x19f8));
                c.swc1(6, (c.g[A1][0] as u32).wrapping_add(0xc));
                c.g[T2][0] = sx((c.g[T9][0] as u32).wrapping_add(0x1998));
                c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
                c.g[S6][0] = 0x4d0000;
                c.lwc1(17, (c.g[GP][0] as u32).wrapping_sub(0x4e98));
                c.g[FP][0] = 0x4d0000;
                c.lwc1(18, (c.g[GP][0] as u32).wrapping_sub(0x4e94));
                c.g[S3][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1a08));
                c.sw((c.g[T1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[T7][0] = sx((c.g[S6][0] as u32).wrapping_add(0x19c8));
                c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x4e90));
                c.g[FP][0] = 0x4d0000;
                c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x4e8c));
                c.g[T8][0] = 0x4d0000;
                c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[T5][0] = sx((c.g[T8][0] as u32).wrapping_add(0x19a8));
                c.swc1(18, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[S7][0] = 0x4d0000;
                c.swc1(13, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = 0x4d0000;
                c.swc1(14, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1a18));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[T6][0] = sx((c.g[S7][0] as u32).wrapping_add(0x19b8));
                c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x4e88));
                c.g[V1][0] = 0x4d0000;
                c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x4e84));
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1a58));
                c.sw((c.g[T0][0] as u32).wrapping_add(0xc), 0u64);
                c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1a78));
                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x4e80));
                c.g[FP][0] = 0x4d0000;
                c.swc1(15, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[S0][0] = 0x4d0000;
                c.swc1(16, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.swc1(12, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x4d0000;
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
                c.g[S4][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a68));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x4d0000;
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[T3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a88));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x4d0000;
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x4));
                c.g[A3][0] = 0x4d0000;
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1aa8));
                c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x4));
                c.g[T1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1ad8));
                c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[A3][0] = 0x4d0000;
                c.lwc1(19, (c.g[GP][0] as u32).wrapping_sub(0x4e7c));
                c.g[A0][0] = 0x4d0000;
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4e78));
                c.g[A2][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1ab8));
                c.swc1(19, (c.g[T4][0] as u32).wrapping_add(0x4));
                c.g[FP][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1af8));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4e74));
                c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1b08));
                c.swc1(2, (c.g[T9][0] as u32).wrapping_add(0x1998));
                c.g[A0][0] = 0x4d0000;
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x4e70));
                c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1b68));
                c.swc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.g[S0][0];
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4e6c));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x19d8));
                c.swc1(8, (c.g[T7][0] as u32).wrapping_add(0xc));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4e68));
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4e64));
                c.swc1(3, (c.g[T4][0] as u32).wrapping_add(0x8));
                c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
                c.swc1(4, (c.g[T2][0] as u32).wrapping_add(0x4));
                c.swc1(5, (c.g[T2][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[T8][0] as u32).wrapping_add(0x19a8));
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0xc));
                c.swc1(0, (c.g[S7][0] as u32).wrapping_add(0x19b8));
                c.sw((c.g[T6][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4e60));
                c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x19c8));
                c.swc1(7, (c.g[T7][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x19d8));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = 0x4d0000;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4e5c));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x19e8));
                c.g[A0][0] = 0x4d0000;
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[T4][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1b88));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4e58));
                c.g[A0][0] = 0x4d0000;
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4e54));
                c.g[T2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1b98));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4e50));
                c.g[A0][0] = 0x4d0000;
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4e4c));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x19f8));
                c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
                c.g[A0][0] = 0x4d0000;
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x1a08));
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.g[S0][0];
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x19d8));
                c.sw((c.g[T6][0] as u32).wrapping_add(0x4), 0u64);
                c.sw((c.g[T6][0] as u32).wrapping_add(0x8), 0u64);
                c.swc1(8, (c.g[T7][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x4d0000;
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1bc8));
                c.sw((c.g[S2][0] as u32).wrapping_add(0x4), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
                c.g[A0][0] = 0x4d0000;
                c.g[S2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1be8));
                c.sw((c.g[S3][0] as u32).wrapping_add(0x4), 0u64);
                c.g[A0][0] = 0x4d0000;
                c.sw((c.g[S3][0] as u32).wrapping_add(0x8), 0u64);
                c.g[T7][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c08));
                c.sw((c.g[S3][0] as u32).wrapping_add(0xc), 0u64);
                c.g[A0][0] = 0x4d0000;
                c.g[S3][0] = 0x4d0000;
                c.g[T6][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c18));
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1a48));
                c.g[A0][0] = 0x4d0000;
                c.swc1(6, (c.g[S3][0] as u32).wrapping_add(0xc));
                c.g[T5][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c28));
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = 0x4d0000;
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4e48));
                c.swc1(1, (c.g[A0][0] as u32).wrapping_add(0x1a78));
                c.g[A0][0] = 0x4d0000;
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x4e44));
                c.g[S3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c38));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4e40));
                c.g[A0][0] = 0x4d0000;
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4e3c));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x1a18));
                c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = 0x4d0000;
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x1a28));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a28));
                c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x4e38));
                c.swc1(6, (c.g[A0][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = 0x4d0000;
                c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x4e34));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x1a38));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a38));
                c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x4e30));
                c.swc1(6, (c.g[A0][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = 0x4d0000;
                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x4e2c));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x1a48));
                c.g[A0][0] = 0x4d0000;
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x1a58));
                c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x4d0000;
                c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c88));
                c.g[V0][0] = 0x4d0000;
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4e28));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x1a68));
                c.swc1(5, (c.g[S4][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = 0x4d0000;
                c.sw((c.g[S4][0] as u32).wrapping_add(0xc), 0u64);
                c.swc1(7, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.swc1(8, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[V1][0] = 0x4d0000;
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4e24));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x1a88));
                c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = 0x4d0000;
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x1a98));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1a28));
                c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = 0x4d0000;
                c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1a38));
                c.swc1(6, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = 0x4d0000;
                c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1a48));
                c.swc1(6, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = 0x4d0000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1a98));
                c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = 0x4d0000;
                c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.swc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
                c.swc1(1, (c.g[T1][0] as u32).wrapping_add(0xc));
                c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4e20));
                c.g[V0][0] = 0x4d0000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x1aa8));
                c.swc1(4, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = 0x4d0000;
                c.swc1(5, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.sw((c.g[A1][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x4e1c));
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4e18));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x1ab8));
                c.swc1(6, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V1][0];
                c.swc1(7, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1ac8));
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x4e14));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x1ac8));
                c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = 0x4d0000;
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x1ad8));
                c.g[V0][0] = 0x4d0000;
                c.swc1(9, (c.g[T1][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = c.g[V0][0];
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x1ae8));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1ae8));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = 0x4d0000;
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x1af8));
                c.g[V1][0] = 0x4d0000;
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = 0x4d0000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x1b08));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1ac8));
                c.swc1(10, (c.g[A3][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = 0x4d0000;
                c.swc1(11, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1ae8));
                c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
                c.g[A3][0] = 0x4d0000;
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4e10));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x1b18));
                c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1b18));
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = 0x4d0000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1b78));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = 0x4d0000;
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
                c.g[FP][0] = 0x4d0000;
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0xc));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4e0c));
                c.g[A3][0] = c.g[V1][0];
                c.swc1(4, (c.g[FP][0] as u32).wrapping_add(0x1b58));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1b28));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4e08));
                c.g[FP][0] = 0x4d0000;
                c.swc1(3, (c.g[T0][0] as u32).wrapping_add(0xc));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4e04));
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.swc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = c.g[FP][0];
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4e00));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1b38));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x1b28));
                c.swc1(5, (c.g[A3][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = 0x4d0000;
                c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x1b38));
                c.g[A3][0] = c.g[V1][0];
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1b48));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x1b48));
                c.g[FP][0] = 0x4d0000;
                c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1b58));
                c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
                c.g[V0][0] = 0x4d0000;
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x4dfc));
                c.g[V1][0] = 0x4d0000;
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4df8));
                c.g[A3][0] = c.g[V1][0];
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x1b68));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1b78));
                c.swc1(6, (c.g[T0][0] as u32).wrapping_add(0x4));
                c.g[FP][0] = 0x4d0000;
                c.swc1(7, (c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = 0x4d0000;
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x4df4));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x1b78));
                c.swc1(8, (c.g[A3][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = 0x4d0000;
                c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x4df0));
                c.g[A3][0] = 0x4d0000;
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x1b88));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1b28));
                c.swc1(9, (c.g[T4][0] as u32).wrapping_add(0x8));
                c.g[FP][0] = 0x4d0000;
                c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1b38));
                c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x4dec));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x1b98));
                c.swc1(10, (c.g[T2][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = 0x4d0000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x1ba8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1b48));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = 0x4d0000;
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1b58));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
                c.g[A3][0] = 0x4d0000;
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1b78));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                c.g[FP][0] = 0x4d0000;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1ba8));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                c.g[V0][0] = 0x4d0000;
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1bb8));
                c.swc1(0, (c.g[T4][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = 0x4d0000;
                c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1bf8));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
                c.g[A3][0] = 0x4d0000;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4de8));
                c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4de4));
                c.g[V0][0] = 0x4d0000;
                c.swc1(2, (c.g[V1][0] as u32).wrapping_add(0xc));
                c.swc1(1, (c.g[T7][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = c.g[V0][0];
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1bd8));
                c.g[FP][0] = 0x4d0000;
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x1bb8));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x1bc8));
                c.g[A3][0] = 0x4d0000;
                c.sw((c.g[S1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[FP][0] = 0x4d0000;
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x1bd8));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[V0][0] = c.g[FP][0];
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x1be8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1bf8));
                c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
                c.g[V1][0] = 0x4d0000;
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4de0));
                c.g[A3][0] = 0x4d0000;
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x1bf8));
                c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[FP][0] = 0x4d0000;
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4ddc));
                c.g[V0][0] = 0x4d0000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x1c08));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1ba8));
                c.swc1(4, (c.g[T7][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = 0x4d0000;
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4dd8));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1bb8));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x1c18));
                c.swc1(5, (c.g[T6][0] as u32).wrapping_add(0x8));
                c.g[A3][0] = 0x4d0000;
                c.sw((c.g[T6][0] as u32).wrapping_add(0xc), 0u64);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1bd8));
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x4dd4));
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4dd0));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x1c28));
                c.swc1(6, (c.g[T5][0] as u32).wrapping_add(0x4));
                c.g[FP][0] = 0x1;
                c.swc1(7, (c.g[T5][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.swc1(12, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = 0x2;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = 0x3;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
                c.sw((c.g[A3][0] as u32).wrapping_add(0x8), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
                c.g[A3][0] = 0x4d0000;
                c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x4));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1c48));
                c.g[S2][0] = 0x4d0000;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4dcc));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1bf8));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4dc8));
                c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x4));
                c.g[S2][0] = 0x4d0000;
                c.swc1(0, (c.g[T6][0] as u32).wrapping_add(0x4));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1c58));
                c.sw((c.g[T5][0] as u32).wrapping_add(0xc), 0u64);
                c.sb((c.g[GP][0] as u32).wrapping_add(0x1f88), c.g[V0][0]);
                c.sb((c.g[GP][0] as u32).wrapping_add(0x1f90), c.g[V1][0]);
                c.sb((c.g[GP][0] as u32).wrapping_add(0x1f78), 0u64);
                c.sb((c.g[GP][0] as u32).wrapping_add(0x1f80), c.g[FP][0]);
                c.g[V1][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x1f88));
                c.g[A2][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x1f78));
                c.g[V0][0] = c.g[FP][0] & 0xff;
                c.g[A1][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x1f90));
                c.g[FP][0] = 0x4d0000;
                c.swc1(2, (c.g[A3][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = c.g[V0][0] | 0x100;
                c.swc1(1, (c.g[S2][0] as u32).wrapping_add(0xc));
                c.g[A3][0] = 0x4d0000;
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4dc4));
                c.g[S2][0] = c.g[A3][0];
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4dc0));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1c48));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x1c38));
                c.g[V1][0] = c.g[V1][0] | 0x200;
                c.swc1(3, (c.g[S3][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = c.g[A1][0] | 0x300;
                c.swc1(4, (c.g[S3][0] as u32).wrapping_add(0x8));
                c.sw((c.g[S3][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4dbc));
                c.g[S3][0] = 0x4d0000;
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x4db8));
                c.g[FP][0] = c.g[S3][0];
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x1c48));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1c58));
                c.swc1(5, (c.g[S2][0] as u32).wrapping_add(0x4));
                c.g[A3][0] = 0x4d0000;
                c.swc1(6, (c.g[S2][0] as u32).wrapping_add(0x8));
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4db4));
                c.g[S2][0] = c.g[A3][0];
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x4db0));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1c68));
                c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x1c58));
                c.swc1(7, (c.g[FP][0] as u32).wrapping_add(0x4));
                c.g[S3][0] = 0x4d0000;
                c.swc1(8, (c.g[FP][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x1c68));
                c.g[FP][0] = c.g[S3][0];
                c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1c78));
                c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x1c78));
                c.g[A3][0] = 0x6;
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
                c.g[S3][0] = 0x8;
                c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x1f78), c.g[A2][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x1f80), c.g[V0][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x1f88), c.g[V1][0]);
                c.g[V0][0] = 0x4;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x4), 0u64);
                c.g[V1][0] = 0x5;
                c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x8));
                c.sw((c.g[FP][0] as u32).wrapping_add(0x8), 0u64);
                c.g[S2][0] = 0x7;
                c.sw((c.g[GP][0] as u32).wrapping_add(0x1f74), 0u64);
                c.g[FP][0] = 0x9;
                c.sw((c.g[GP][0] as u32).wrapping_add(0x1f70), 0u64);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x1f90), c.g[A1][0]);
                c.sb((c.g[GP][0] as u32).wrapping_add(0x1f98), c.g[V0][0]);
                c.sb((c.g[GP][0] as u32).wrapping_add(0x1fa0), c.g[V1][0]);
                c.g[V0][0] = 0xa;
                c.sb((c.g[GP][0] as u32).wrapping_add(0x1fa8), c.g[A3][0]);
                c.g[V1][0] = 0xb;
                c.sb((c.g[GP][0] as u32).wrapping_add(0x1fb0), c.g[S2][0]);
                c.g[A3][0] = 0xc;
                c.sb((c.g[GP][0] as u32).wrapping_add(0x1fd0), c.g[V1][0]);
                c.g[S2][0] = 0xd;
                c.sb((c.g[GP][0] as u32).wrapping_add(0x1fc8), c.g[V0][0]);
                c.sb((c.g[GP][0] as u32).wrapping_add(0x1fd8), c.g[A3][0]);
                c.g[A1][0] = c.g[V1][0] & 0xff;
                c.sb((c.g[GP][0] as u32).wrapping_add(0x1fb8), c.g[S3][0]);
                c.g[A2][0] = c.g[V0][0] & 0xff;
                c.sb((c.g[GP][0] as u32).wrapping_add(0x1fc0), c.g[FP][0]);
                c.g[A2][0] = c.g[A2][0] | 0xa00;
                c.sb((c.g[GP][0] as u32).wrapping_add(0x1fe0), c.g[S2][0]);
                c.g[T0][0] = c.g[S3][0] & 0xff;
                c.g[T1][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x1f98));
                c.g[A3][0] = c.g[FP][0] & 0xff;
                c.g[T3][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x1fa0));
                c.g[V0][0] = c.g[S2][0] & 0xff;
                c.g[T4][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x1fa8));
                c.g[T1][0] = c.g[T1][0] | 0x400;
                c.g[T2][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x1fb0));
                c.g[T3][0] = c.g[T3][0] | 0x500;
                c.g[V1][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x1fd8));
                c.g[T4][0] = c.g[T4][0] | 0x600;
                c.g[T2][0] = c.g[T2][0] | 0x700;
                c.g[T0][0] = c.g[T0][0] | 0x800;
                c.g[A3][0] = c.g[A3][0] | 0x900;
                c.g[A1][0] = c.g[A1][0] | 0xb00;
                c.g[V1][0] = c.g[V1][0] | 0xc00;
                c.g[V0][0] = c.g[V0][0] | 0xd00;
                c.sw((c.g[GP][0] as u32).wrapping_add(0x1f98), c.g[T1][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x1fa0), c.g[T3][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x1fa8), c.g[T4][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x1fb0), c.g[T2][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x1fb8), c.g[T0][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x1fc0), c.g[A3][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x1fc8), c.g[A2][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x1fd0), c.g[A1][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x1fd8), c.g[V1][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x1fe0), c.g[V0][0]);
                Callee {
                    addr: 0x0025_01e8,
                    f: ee::s06::f_002501e8,
                }
                .call(c, 0x0025_15a8)?;
                frame.restore(c, 16, 0x90);
            } else {
                // init == 0: the AvalancheMenu destructor, with a2 = 2.
                c.cycles += 4;
                c.g[A0][0] = 0x4d0000;
                c.g[A1][0] = 0x2;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c88));
                Callee {
                    addr: 0x0025_0ae0,
                    f: ee::s06::f_00250ae0,
                }
                .call(c, 0x0025_15c0)?;
                c.cycles += 1;
                frame.restore(c, 16, 0x90);
            }
        }
        // epilogue
        c.cycles += 11;
        frame.restore(c, 17, 0x80);
        frame.restore(c, 18, 0x70);
        frame.restore(c, 19, 0x60);
        frame.restore(c, 20, 0x50);
        frame.restore(c, 21, 0x40);
        frame.restore(c, 22, 0x30);
        frame.restore(c, 23, 0x20);
        frame.restore(c, 30, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Offsets (from the menu object) of the 34 `cMenuItem` members, in the order
/// the constructor builds them.
pub(crate) const AVALANCHE_MENU_ITEM_OFFSETS: [u32; 34] = [
    0x77c, 0x748, 0x714, 0x6e0, 0x6b0, 0x67c, 0x648, 0x614, 0x5e0, 0x5ac, 0x578, 0x544, 0x514,
    0x4e0, 0x4ac, 0x478, 0x444, 0x410, 0x3dc, 0x3ac, 0x37c, 0x34c, 0x31c, 0x2e8, 0x2b8, 0x288,
    0x258, 0x228, 0x1f8, 0x1c4, 0x194, 0x164, 0x14c, 0x130,
];

/// `cMenuItem::cMenuItem(this, 2)` (0x2ca280).
pub(crate) const MENU_ITEM_CTOR: u32 = 0x002c_a280;

/// `cMenu::cMenu(this, flags)` (0x2caa80).
pub(crate) const MENU_CTOR: u32 = 0x002c_aa80;

/// `cAvalancheMenu::cAvalancheMenu(this, flags)` (0x250ae0). The constructor
/// (the list calls it a destructor) builds the menu's 34 `cMenuItem`s (flag 2)
/// at their fixed offsets, then runs the base `cMenu` constructor on the whole
/// object. Item `i` returns to `0x250b04 + 12 * i`; the base constructor returns
/// to `0x250c9c`. A callee that unwinds returns at once, as the compiled code
/// does.
pub fn avalanche_menu_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 116;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        for (i, &off) in AVALANCHE_MENU_ITEM_OFFSETS.iter().enumerate() {
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(off));
            c.g[A1][0] = 2;
            call_ptr(c, MENU_ITEM_CTOR, 0x0025_0b04 + 12 * i as u32)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, MENU_CTOR, 0x0025_0c9c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `VisualEffectsTestMenu_ctor(this)` (0x248970): the debug menu for the
/// visual-effects test. Runs the `cMenu` base constructor, sets the vtable,
/// builds the title and spacer items, then the 17 int items (HUD toggles,
/// the current element, position, size, scale, colour, layer and the world
/// painter enable and layer id), and appends all 19 items to the menu.
/// Returns `this`. Transcribed from the generated body; each call goes through
/// `call_ptr` so an unwinding callee returns at once, as the original does.
pub fn visual_effects_test_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 278;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x130));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3a4));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x404));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x344));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S6][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3d4));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S7][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2e4));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
        c.g[FP][0] = sx((c.g[S0][0] as u32).wrapping_add(0x374));
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x434));
        call_ptr(c, 0x002caa58, 0x002489c8)?;
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x464));
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c));
        c.g[A1][0] = 0x480000;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1fc8));
        c.g[V0][0] = 0x480000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x378));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x12c), c.g[V0][0]);
        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
        call_ptr(c, 0x002cd2a0, 0x002489f0)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x164));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[A1][0] = 0x28;
        call_ptr(c, 0x002cd350, 0x00248a04)?;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x194));
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.g[T1][0] = 0x480000;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c4));
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1fa8));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.g[A1][0] = 0u64;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1388));
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0x1;
        call_ptr(c, 0x002cd8f8, 0x00248a34)?;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
        c.g[T1][0] = 0x480000;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x224));
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1f90));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V0][0]);
        c.g[A1][0] = 0x1;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x138c));
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0x1;
        call_ptr(c, 0x002cd8f8, 0x00248a64)?;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x254));
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
        c.g[T1][0] = 0x480000;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x284));
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1f78));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
        c.g[A1][0] = 0x2;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1390));
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0x1;
        call_ptr(c, 0x002cd8f8, 0x00248a94)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2b4));
        c.g[T1][0] = 0x480000;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V0][0]);
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1f68));
        c.g[A1][0] = 0x3;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1394));
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0x3e8;
        call_ptr(c, 0x002cd8f8, 0x00248abc)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x314));
        c.g[T1][0] = 0x480000;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[V0][0]);
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1f50));
        c.g[A1][0] = 0x4;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1398));
        c.g[A3][0] = 0xfffffffffffffd80;
        c.g[T0][0] = 0x280;
        call_ptr(c, 0x002cd8f8, 0x00248ae4)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
        c.g[T1][0] = 0x480000;
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1f40));
        c.g[A1][0] = 0x5;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x139c));
        c.g[A3][0] = 0xfffffffffffffe20;
        c.g[T0][0] = 0x1e0;
        call_ptr(c, 0x002cd8f8, 0x00248b04)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[T1][0] = 0x4a0000;
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2cf0));
        c.g[A1][0] = 0x6;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x13a0));
        c.g[A3][0] = 0xfffffffffffffd80;
        c.g[T0][0] = 0x280;
        call_ptr(c, 0x002cd8f8, 0x00248b24)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
        c.g[T1][0] = 0x4a0000;
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2cf8));
        c.g[A1][0] = 0x7;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x13a4));
        c.g[A3][0] = 0xfffffffffffffd80;
        c.g[T0][0] = 0x280;
        call_ptr(c, 0x002cd8f8, 0x00248b44)?;
        c.g[T1][0] = 0x4a0000;
        c.g[A0][0] = c.g[S7][0];
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2d00));
        c.g[A1][0] = 0x8;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x13a8));
        c.g[A3][0] = 0xffffffffffffffff;
        c.g[T0][0] = 0x3e8;
        call_ptr(c, 0x002cd8f8, 0x00248b64)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
        c.g[T1][0] = 0x4a0000;
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2d08));
        c.g[A1][0] = 0x9;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x13ac));
        c.g[A3][0] = 0xffffffffffffffff;
        c.g[T0][0] = 0x3e8;
        call_ptr(c, 0x002cd8f8, 0x00248b84)?;
        c.g[T1][0] = 0x4a0000;
        c.g[A0][0] = c.g[S5][0];
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2d10));
        c.g[A1][0] = 0xa;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x13b0));
        c.g[A3][0] = 0xffffffffffffff01;
        c.g[T0][0] = 0xff;
        call_ptr(c, 0x002cd8f8, 0x00248ba4)?;
        c.g[T1][0] = 0x4a0000;
        c.g[A0][0] = c.g[FP][0];
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2d18));
        c.g[A1][0] = 0xb;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x13b4));
        c.g[A3][0] = 0xffffffffffffff01;
        c.g[T0][0] = 0xff;
        call_ptr(c, 0x002cd8f8, 0x00248bc4)?;
        c.g[T1][0] = 0x4a0000;
        c.g[A0][0] = c.g[S3][0];
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2d20));
        c.g[A1][0] = 0xc;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x13b8));
        c.g[A3][0] = 0xffffffffffffff01;
        c.g[T0][0] = 0xff;
        call_ptr(c, 0x002cd8f8, 0x00248be4)?;
        c.g[T1][0] = 0x4a0000;
        c.g[A0][0] = c.g[S6][0];
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2d28));
        c.g[A1][0] = 0xd;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x13bc));
        c.g[A3][0] = 0xffffffffffffff01;
        c.g[T0][0] = 0xff;
        call_ptr(c, 0x002cd8f8, 0x00248c04)?;
        c.g[T1][0] = 0x480000;
        c.g[A0][0] = c.g[S2][0];
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1f30));
        c.g[A1][0] = 0xe;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x13c0));
        c.g[A3][0] = 0xfffffffffffffff0;
        c.g[T0][0] = 0x10;
        call_ptr(c, 0x002cd8f8, 0x00248c24)?;
        c.g[T1][0] = 0x480000;
        c.g[A0][0] = c.g[S4][0];
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1f20));
        c.g[A1][0] = 0xf;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x13c4));
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0x1;
        call_ptr(c, 0x002cd8f8, 0x00248c44)?;
        c.g[T1][0] = 0x480000;
        c.g[A3][0] = 0u64;
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1f00));
        c.g[T0][0] = 0x64;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x13c8));
        c.g[A1][0] = 0x10;
        call_ptr(c, 0x002cd8f8, 0x00248c64)?;
        c.g[A1][0] = c.lw(c.g[SP][0] as u32);
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248c74)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248c84)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248c94)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248ca4)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248cb4)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248cc4)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248cd4)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248ce4)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248cf4)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248d04)?;
        c.g[A1][0] = c.g[S7][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248d14)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248d24)?;
        c.g[A1][0] = c.g[S5][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248d34)?;
        c.g[A1][0] = c.g[FP][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248d44)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248d54)?;
        c.g[A1][0] = c.g[S6][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248d64)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248d74)?;
        c.g[A1][0] = c.g[S4][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248d84)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248d94)?;
        c.g[V0][0] = c.g[S0][0];
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
        c.pc = t;
        Ok(())
    });
}

/// `PlantsMenu_ctor(this)` (0x248dc8): the debug menu for the plant sway and
/// branch parameters. Runs the `cMenu` base constructor, sets the vtable,
/// builds the title and spacer items, the enable int item and the float
/// items (sway, branch and rotation amplitudes and frequencies, direction),
/// then appends all 14 items to the menu. Returns `this`. Transcribed from
/// the generated body; each call goes through `call_ptr`.
pub fn plants_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 223;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xe0));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x130));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2fc));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x330));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2c8));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S6][0] = sx((c.g[S0][0] as u32).wrapping_add(0x264));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S7][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1fc));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
        c.g[FP][0] = sx((c.g[S0][0] as u32).wrapping_add(0x298));
        c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x364));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xc8));
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x398));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        call_ptr(c, 0x002caa58, 0x00248e30)?;
        c.mtc1(0, 20);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c));
        c.g[A1][0] = 0x480000;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1ee8));
        c.g[V0][0] = 0x480000;
        c.g[AT][0] = 0x40000000;
        c.mtc1(1, 21);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3d0));
        c.lwc1(22, (c.g[GP][0] as u32).wrapping_sub(0x5004));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x12c), c.g[V0][0]);
        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
        call_ptr(c, 0x002cd2a0, 0x00248e64)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x164));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[A1][0] = 0x28;
        call_ptr(c, 0x002cd350, 0x00248e78)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x194));
        c.g[T1][0] = 0x4a0000;
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2d30));
        c.g[T0][0] = 0x2;
        c.g[A3][0] = 0u64;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1354));
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x002cd8f8, 0x00248ea0)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c8));
        c.g[A3][0] = 0x480000;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1ed8));
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1358));
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x002cdce8, 0x00248ec8)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x230));
        c.g[AT][0] = 0xffffffffc0a00000;
        c.mtc1(1, 12);
        c.g[A3][0] = 0x480000;
        c.g[AT][0] = 0x40a00000;
        c.mtc1(1, 13);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1ec8));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x135c));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002cdce8, 0x00248ef8)?;
        c.g[A3][0] = 0x480000;
        c.g[A0][0] = c.g[S7][0];
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1eb8));
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1360));
        c.g[A1][0] = 0x3;
        call_ptr(c, 0x002cdce8, 0x00248f18)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
        c.g[A3][0] = 0x480000;
        c.mov_s(13, 21);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1ea0));
        c.mov_s(12, 20);
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1364));
        c.g[AT][0] = 0x43b40000;
        c.mtc1(1, 21);
        c.g[A1][0] = 0x4;
        call_ptr(c, 0x002cdce8, 0x00248f40)?;
        c.g[AT][0] = 0x447a0000;
        c.mtc1(1, 13);
        c.g[A3][0] = 0x480000;
        c.mov_s(12, 20);
        c.g[A0][0] = c.g[S6][0];
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1e88));
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1368));
        c.g[A1][0] = 0x5;
        call_ptr(c, 0x002cdce8, 0x00248f64)?;
        c.g[T1][0] = 0x480000;
        c.g[T0][0] = 0x1770;
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1e78));
        c.g[A0][0] = c.g[FP][0];
        c.g[A3][0] = 0u64;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x136c));
        c.g[A1][0] = 0x6;
        call_ptr(c, 0x002cd8f8, 0x00248f84)?;
        c.g[A3][0] = 0x480000;
        c.g[A0][0] = c.g[S5][0];
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1e58));
        c.mov_s(12, 20);
        c.mov_s(13, 22);
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1370));
        c.g[A1][0] = 0x7;
        call_ptr(c, 0x002cdce8, 0x00248fa4)?;
        c.g[A3][0] = 0x480000;
        c.g[A0][0] = c.g[S3][0];
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1e40));
        c.mov_s(12, 20);
        c.mov_s(13, 22);
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1374));
        c.g[A1][0] = 0x8;
        call_ptr(c, 0x002cdce8, 0x00248fc4)?;
        c.g[A3][0] = 0x480000;
        c.mov_s(13, 22);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1e20));
        c.g[A0][0] = c.g[S4][0];
        c.mov_s(12, 20);
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1378));
        c.g[A1][0] = 0x9;
        call_ptr(c, 0x002cdce8, 0x00248fe4)?;
        c.g[A3][0] = 0x480000;
        c.g[A0][0] = c.g[S2][0];
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1e00));
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x137c));
        c.g[A1][0] = 0xa;
        call_ptr(c, 0x002cdce8, 0x00249004)?;
        c.g[A3][0] = 0x480000;
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1df0));
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1380));
        c.g[A1][0] = 0xb;
        call_ptr(c, 0x002cdce8, 0x00249024)?;
        c.g[A1][0] = c.lw(c.g[SP][0] as u32);
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00249034)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00249044)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00249054)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00249064)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00249074)?;
        c.g[A1][0] = c.g[S7][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00249084)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00249094)?;
        c.g[A1][0] = c.g[S6][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002490a4)?;
        c.g[A1][0] = c.g[FP][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002490b4)?;
        c.g[A1][0] = c.g[S5][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002490c4)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002490d4)?;
        c.g[A1][0] = c.g[S4][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002490e4)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002490f4)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00249104)?;
        c.g[V0][0] = c.g[S0][0];
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0xc8));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
        c.pc = t;
        Ok(())
    });
}

/// `this + off` into v0, then stored at frame slot `slot` (`sp + 4 * slot`).
pub(crate) fn menu_ctor_stash(c: &mut Cpu, slot: u32, off: u32) {
    c.g[V0][0] = this_at(c, off);
    let addr = sp(c).wrapping_add(4 * slot);
    let v = c.g[V0][0];
    c.sw(addr, v);
}

/// Load f-register `f` with a bound: a copy of an f-register, or a literal
/// moved through `at` with `mtc1`.
pub(crate) fn menu_ctor_bound(c: &mut Cpu, f: usize, bound: Arg) {
    match bound {
        Arg::Reg(src) => c.mov_s(f, src),
        Arg::Lit(bits) => {
            c.g[AT][0] = bits;
            c.mtc1(1, f);
        }
    }
}

/// `cFloatMenuItem_ctor(item, index, &float at gp+value, label, min, max)`
/// (0x2cdce8) with f12/f13 already set by the caller.
pub(crate) fn menu_ctor_float(
    c: &mut Cpu,
    item: Item,
    index: u64,
    value: u32,
    label: u64,
    ret: u32,
) -> Flow {
    let this = item_value(c, item);
    c.g[A0][0] = this;
    c.g[A1][0] = index;
    c.g[A2][0] = gp_at(c, value);
    c.g[A3][0] = label;
    call_ptr(c, 0x002c_dce8, ret)
}

/// `cFloatMenuItem_ctor` with both bounds loaded from gp-relative floats
/// (`lwc1 f12, gp-min; lwc1 f13, gp-max`).
pub(crate) fn menu_ctor_float_gp(
    c: &mut Cpu,
    item: Item,
    index: u64,
    value: u32,
    label: u64,
    min_off: u32,
    max_off: u32,
    ret: u32,
) -> Flow {
    c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(min_off));
    c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(max_off));
    menu_ctor_float(c, item, index, value, label, ret)
}

/// `DepthFogMenu_ctor(DepthFogMenu *this)` (0x2498a0): the depth-fog debug
/// menu page. It builds the menu base (`cMenu_cMenu`, 0x2caa58), sets the
/// vtable, then the title (a null item, "Depth Fog Menu") and a spacer, then
/// the eight items of the fog: the enable and override-world-painter ints,
/// the near/far planes and the density and skybox density scale floats, the
/// exponent int, and the colour. Every item is appended to the page with
/// index -1. Returns `this` in v0.
///
/// The generated body keeps the title and space addresses in frame slots
/// (`sp + 0`, `sp + 4`), the rest in s1..s7/fp, so the port writes the same
/// slots before the same calls.
pub fn depth_fog_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 162;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S0, 0xa0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x90);
        c.g[V0][0] = this_at(c, 0x130); // TitleItem
        frame.save(c, S2, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S4, 0x60);
        c.g[S3][0] = this_at(c, 0x25c); // DensityItem
        frame.save(c, S5, 0x50);
        c.g[S4][0] = this_at(c, 0x1f8); // FarPlaneItem
        frame.save(c, S6, 0x40);
        c.g[S5][0] = this_at(c, 0x22c); // ExponentItem
        frame.save(c, S7, 0x30);
        c.g[S6][0] = this_at(c, 0x1c4); // NearPlaneItem
        frame.save(c, FP, 0x20);
        c.g[S7][0] = this_at(c, 0x164); // EnableItem
        frame.save_ra(c, 0x10);
        c.g[FP][0] = this_at(c, 0x194); // OverrideWPItem
        frame.save_fpr(c, 22, 0xc0);
        c.g[S1][0] = this_at(c, 0x290); // XBoxDensityScaleItem
        frame.save_fpr(c, 21, 0xb8);
        c.g[S2][0] = this_at(c, 0x2c4); // ColourItem
        frame.save_fpr(c, 20, 0xb0);

        // cMenu_cMenu(this), with the title address still in v0 and slot 0.
        let slot0 = frame.at(c, 0);
        let v0 = c.g[V0][0];
        c.sw(slot0, v0);
        call_ptr(c, 0x002c_aa58, 0x0024_9908)?;

        // The fog's vtable and the title and space items.
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 21); // f21 = 1.0
        menu_ctor_stash(c, 1, 0x14c); // SpaceItem into slot 1
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x1ba0)); // "Depth Fog Menu"
        c.g[V0][0] = 0x0048_0000;
        c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x5000));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x480)); // cDepthFogMenu_vtbl
        c.mtc1(0, 22); // f22 = 0.0
        let vtbl_addr = (c.g[S0][0] as u32).wrapping_add(0x12c);
        let vtbl = c.g[V0][0];
        c.sw(vtbl_addr, vtbl);
        c.g[A0][0] = item_value(c, Item::Slot(0));
        call_ptr(c, 0x002c_d2a0, 0x0024_993c)?;

        // Spacer item.
        c.g[A0][0] = item_value(c, Item::Slot(1));
        c.g[A1][0] = 0x28;
        call_ptr(c, 0x002c_d350, 0x0024_9948)?;

        // Enable (int, 0..1), OverrideWP (int, 0..1).
        int_item(
            c,
            Item::Reg(S7),
            0,
            0x1234,
            0,
            1,
            sx(0x004a_0000u32.wrapping_add(0x2d30)), // "Enable"
            0x0024_9968,
        )?;
        int_item(
            c,
            Item::Reg(FP),
            1,
            0x1238,
            0,
            1,
            sx(0x0048_0000u32.wrapping_sub(0x1dc8)), // "Override World Painter"
            0x0024_9988,
        )?;

        // NearPlane (1.0 .. 100000.0), FarPlane (1.0 .. 100000.0).
        menu_ctor_bound(c, 12, Arg::Reg(21));
        menu_ctor_bound(c, 13, Arg::Reg(20));
        menu_ctor_float(
            c,
            Item::Reg(S6),
            2,
            0x123c,
            sx(0x0048_0000u32.wrapping_sub(0x1b90)),
            0x0024_99a8,
        )?;
        menu_ctor_bound(c, 13, Arg::Reg(20));
        menu_ctor_bound(c, 12, Arg::Reg(21));
        menu_ctor_float(
            c,
            Item::Reg(S4),
            3,
            0x1240,
            sx(0x0048_0000u32.wrapping_sub(0x1b80)),
            0x0024_99c8,
        )?;

        // Exponent (int, 0..2).
        int_item(
            c,
            Item::Reg(S5),
            4,
            0x1244,
            0,
            2,
            sx(0x0048_0000u32.wrapping_sub(0x1b70)),
            0x0024_99e8,
        )?;

        // Density (0.0 .. 100.0).
        menu_ctor_bound(c, 13, Arg::Lit(0x42c8_0000));
        menu_ctor_bound(c, 12, Arg::Reg(22));
        menu_ctor_float(c, Item::Reg(S3), 5, 0x1248, 0x004a_2d48, 0x0024_9a0c)?;

        // XBoxDensityScale (0.0 .. 1.0).
        menu_ctor_bound(c, 12, Arg::Reg(22));
        menu_ctor_bound(c, 13, Arg::Reg(21));
        menu_ctor_float(
            c,
            Item::Reg(S1),
            6,
            0x124c,
            sx(0x0048_0000u32.wrapping_sub(0x1b60)),
            0x0024_9a2c,
        )?;

        // ColourItem (argb): label, then colour.
        argb_item(c, Item::Reg(S2), 7, 0x004a_2d50, 0x0050_4720, 0x0024_9a48)?;

        // Insert the ten items into the page, in order.
        add_item(c, Item::Slot(0), 0x0024_9a58)?;
        add_item(c, Item::Slot(1), 0x0024_9a68)?;
        add_item(c, Item::Reg(S7), 0x0024_9a78)?;
        add_item(c, Item::Reg(FP), 0x0024_9a88)?;
        add_item(c, Item::Reg(S6), 0x0024_9a98)?;
        add_item(c, Item::Reg(S4), 0x0024_9aa8)?;
        add_item(c, Item::Reg(S5), 0x0024_9ab8)?;
        add_item(c, Item::Reg(S3), 0x0024_9ac8)?;
        add_item(c, Item::Reg(S1), 0x0024_9ad8)?;
        add_item(c, Item::Reg(S2), 0x0024_9ae8)?;

        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x90);
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 22, 0xc0);
        frame.restore_fpr(c, 21, 0xb8);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    })
}

/// `HeightFogMenu_ctor(HeightFogMenu *this)` (0x249b28): the height-fog
/// debug menu page. It builds the menu base (`cMenu_cMenu`), sets the
/// vtable, the title ("Height Fog Menu") and a spacer, then the enable int
/// and the start-height and thickness floats, and appends the five items to
/// the page with index -1. Returns `this` in v0.
pub fn height_fog_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 79;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        c.g[S2][0] = this_at(c, 0x164); // EnableItem
        frame.save(c, S4, 0x20);
        c.g[S3][0] = this_at(c, 0x194); // StartHeightItem
        frame.save(c, S5, 0x10);
        c.g[S4][0] = this_at(c, 0x130); // TitleItem
        frame.save_ra(c, 0);
        c.g[S5][0] = this_at(c, 0x14c); // SpaceItem

        // cMenu_cMenu(this).
        call_ptr(c, 0x002c_aa58, 0x0024_9b60)?;
        c.g[S1][0] = this_at(c, 0x1c8); // ThicknessItem

        // The fog's vtable and the title item.
        c.g[V0][0] = sx(0x0048_0000u32.wrapping_sub(0x4d8)); // cHeightFogMenu_vtbl
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x1b40)); // "Height Fog Menu"
        let vtbl_addr = (c.g[S0][0] as u32).wrapping_add(0x12c);
        let vtbl = c.g[V0][0];
        c.sw(vtbl_addr, vtbl);
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, 0x002c_d2a0, 0x0024_9b80)?;

        // Spacer item.
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = 0x28;
        call_ptr(c, 0x002c_d350, 0x0024_9b8c)?;

        // Enable (int, 0..1).
        int_item(
            c,
            Item::Reg(S2),
            0,
            0x12c4,
            0,
            1,
            sx(0x004a_0000u32.wrapping_add(0x2d30)), // "Enable"
            0x0024_9bac,
        )?;

        // StartHeight (-100000.0 .. 100000.0).
        menu_ctor_float_gp(
            c,
            Item::Reg(S3),
            1,
            0x12c8,
            sx(0x0048_0000u32.wrapping_sub(0x1b30)),
            0x4ffc,
            0x4ff8,
            0x0024_9bcc,
        )?;

        // Thickness (0.0001 .. 10000.0).
        menu_ctor_float_gp(
            c,
            Item::Reg(S1),
            2,
            0x12cc,
            sx(0x0048_0000u32.wrapping_sub(0x1b20)),
            0x4ff4,
            0x4ff0,
            0x0024_9bec,
        )?;

        // Insert the five items into the page, in order.
        add_item(c, Item::Reg(S4), 0x0024_9bfc)?;
        add_item(c, Item::Reg(S5), 0x0024_9c0c)?;
        add_item(c, Item::Reg(S2), 0x0024_9c1c)?;
        add_item(c, Item::Reg(S3), 0x0024_9c2c)?;
        add_item(c, Item::Reg(S1), 0x0024_9c3c)?;

        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x50);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `DetailSystemMenu_ctor(DetailSystemMenu *this)` (0x249c68): the detail
/// (sparkle) debug menu page. It builds the menu base (`cMenu_cMenu`), sets
/// the vtable, the title ("Detail System Menu") and a spacer, then the
/// enable int and the eleven sparkle floats (PS2 and XBox counts, sizes and
/// screen-size bounds), and appends the twelve items to the page with index
/// -1. Returns `this` in v0.
///
/// The title, space, enable and spin-scale addresses live in frame slots
/// (`sp + 0, 4, 8, 0xc`), as in the generated body.
pub fn detail_system_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 198;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S0, 0xa0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x90);
        c.g[V0][0] = this_at(c, 0x130); // TitleItem
        frame.save(c, S2, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S4, 0x60);
        c.g[S3][0] = this_at(c, 0x264); // PS2MaxSparkleSizeItem
        frame.save(c, S5, 0x50);
        c.g[S2][0] = this_at(c, 0x2cc); // XBoxSizeItem
        frame.save(c, S6, 0x40);
        c.g[S5][0] = this_at(c, 0x298); // XBoxNumSparklesItem
        frame.save(c, S7, 0x30);
        c.g[S6][0] = this_at(c, 0x230); // PS2MinSparkleSizeItem
        frame.save(c, FP, 0x20);
        c.g[S7][0] = this_at(c, 0x1fc); // PS2SparkleSizeItem
        frame.save_ra(c, 0x10);
        c.g[FP][0] = this_at(c, 0x1c8); // PS2SparkleCountItem
        frame.save_fpr(c, 23, 0xc8);
        c.g[S4][0] = this_at(c, 0x300); // XBoxMinSizeItem
        frame.save_fpr(c, 22, 0xc0);
        c.g[S1][0] = this_at(c, 0x334); // XBoxMaxSizeItem
        frame.save_fpr(c, 21, 0xb8);
        frame.save_fpr(c, 20, 0xb0);

        // cMenu_cMenu(this), with the title address in v0 and slot 0.
        let slot0 = frame.at(c, 0);
        let v0 = c.g[V0][0];
        c.sw(slot0, v0);
        call_ptr(c, 0x002c_aa58, 0x0024_9cd4)?;

        // Constants: f23 = 1.0, f20 = 0.0, f22 = 500.0, f21 = 100.0.
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 23);
        menu_ctor_stash(c, 1, 0x14c); // SpaceItem into slot 1
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x1b10)); // "Detail System Menu"
        c.g[V0][0] = sx(0x0048_0000u32.wrapping_sub(0x530)); // cDetailSystemMenu_vtbl
        c.mtc1(0, 20);
        c.g[AT][0] = 0x43fa_0000;
        c.mtc1(1, 22);
        let vtbl_addr = (c.g[S0][0] as u32).wrapping_add(0x12c);
        let vtbl = c.g[V0][0];
        c.sw(vtbl_addr, vtbl);
        c.g[AT][0] = 0x42c8_0000;
        c.mtc1(1, 21);
        c.g[A0][0] = item_value(c, Item::Slot(0));
        call_ptr(c, 0x002c_d2a0, 0x0024_9d14)?;

        // Spacer item; the enable address goes to slot 2 before the call.
        c.g[A0][0] = item_value(c, Item::Slot(1));
        menu_ctor_stash(c, 2, 0x164); // EnableItem
        c.g[A1][0] = 0x28;
        call_ptr(c, 0x002c_d350, 0x0024_9d28)?;

        // Enable (int, 0..1); the spin-scale address goes to slot 3 first.
        menu_ctor_stash(c, 3, 0x194); // SpinScaleItem
        int_item(
            c,
            Item::Slot(2),
            0,
            0x1250,
            0,
            1,
            sx(0x004a_0000u32.wrapping_add(0x2d30)), // "Enable"
            0x0024_9d50,
        )?;

        // SpinScale (-1.0 .. 1.0).
        menu_ctor_bound(c, 12, Arg::Lit(0xffff_ffff_bf80_0000));
        menu_ctor_bound(c, 13, Arg::Reg(23));
        menu_ctor_float(
            c,
            Item::Slot(3),
            1,
            0x1254,
            sx(0x0048_0000u32.wrapping_sub(0x1af8)),
            0x0024_9d74,
        )?;

        // PS2SparkleCount (0.0 .. 20.0).
        menu_ctor_bound(c, 13, Arg::Lit(0x41a0_0000));
        menu_ctor_bound(c, 12, Arg::Reg(20));
        menu_ctor_float(
            c,
            Item::Reg(FP),
            2,
            0x1258,
            sx(0x0048_0000u32.wrapping_sub(0x1ae0)),
            0x0024_9d98,
        )?;

        // PS2SparkleSize (0.0 .. 500.0).
        menu_ctor_bound(c, 12, Arg::Reg(20));
        menu_ctor_bound(c, 13, Arg::Reg(22));
        menu_ctor_float(
            c,
            Item::Reg(S7),
            3,
            0x125c,
            sx(0x0048_0000u32.wrapping_sub(0x1ac8)),
            0x0024_9db8,
        )?;

        // PS2MinSparkleSize (0.0 .. 100.0).
        menu_ctor_bound(c, 12, Arg::Reg(20));
        menu_ctor_bound(c, 13, Arg::Reg(21));
        menu_ctor_float(
            c,
            Item::Reg(S6),
            4,
            0x1260,
            sx(0x0048_0000u32.wrapping_sub(0x1ab0)),
            0x0024_9dd8,
        )?;

        // PS2MaxSparkleSize (0.0 .. 100.0).
        menu_ctor_bound(c, 12, Arg::Reg(20));
        menu_ctor_bound(c, 13, Arg::Reg(21));
        menu_ctor_float(
            c,
            Item::Reg(S3),
            5,
            0x1264,
            sx(0x0048_0000u32.wrapping_sub(0x1a90)),
            0x0024_9df8,
        )?;

        // XBoxNumSparkles (1.0 .. 300.0).
        menu_ctor_bound(c, 13, Arg::Lit(0x4396_0000));
        menu_ctor_bound(c, 12, Arg::Reg(23));
        menu_ctor_float(
            c,
            Item::Reg(S5),
            6,
            0x1268,
            sx(0x0048_0000u32.wrapping_sub(0x1a70)),
            0x0024_9e1c,
        )?;

        // XBoxSize (0.0 .. 500.0).
        menu_ctor_bound(c, 13, Arg::Reg(22));
        menu_ctor_bound(c, 12, Arg::Reg(20));
        menu_ctor_float(
            c,
            Item::Reg(S2),
            7,
            0x126c,
            sx(0x0048_0000u32.wrapping_sub(0x1a58)),
            0x0024_9e3c,
        )?;

        // XBoxMinSize (0.0 .. 100.0).
        menu_ctor_bound(c, 12, Arg::Reg(20));
        menu_ctor_bound(c, 13, Arg::Reg(21));
        menu_ctor_float(
            c,
            Item::Reg(S4),
            8,
            0x1270,
            sx(0x0048_0000u32.wrapping_sub(0x1a40)),
            0x0024_9e5c,
        )?;

        // XBoxMaxSize (0.0 .. 100.0).
        menu_ctor_bound(c, 12, Arg::Reg(20));
        menu_ctor_bound(c, 13, Arg::Reg(21));
        menu_ctor_float(
            c,
            Item::Reg(S1),
            9,
            0x1274,
            sx(0x0048_0000u32.wrapping_sub(0x1a28)),
            0x0024_9e7c,
        )?;

        // Insert the twelve items into the page, in order.
        add_item(c, Item::Slot(0), 0x0024_9e8c)?;
        add_item(c, Item::Slot(1), 0x0024_9e9c)?;
        add_item(c, Item::Slot(2), 0x0024_9eac)?;
        add_item(c, Item::Slot(3), 0x0024_9ebc)?;
        add_item(c, Item::Reg(FP), 0x0024_9ecc)?;
        add_item(c, Item::Reg(S7), 0x0024_9edc)?;
        add_item(c, Item::Reg(S6), 0x0024_9eec)?;
        add_item(c, Item::Reg(S3), 0x0024_9efc)?;
        add_item(c, Item::Reg(S5), 0x0024_9f0c)?;
        add_item(c, Item::Reg(S2), 0x0024_9f1c)?;
        add_item(c, Item::Reg(S4), 0x0024_9f2c)?;
        add_item(c, Item::Reg(S1), 0x0024_9f3c)?;

        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x90);
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 23, 0xc8);
        frame.restore_fpr(c, 22, 0xc0);
        frame.restore_fpr(c, 21, 0xb8);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    })
}

/// `void ShellRendererMenu_ctor(cShellRendererMenu *self)` (0x249f80): the
/// base menu, then the shell renderer's items: nine groups of `IntMenuItem`
/// constructors (`cIntMenuItem_cIntMenuItem`, two of them with float
/// arguments), each added to the menu with `cMenu_addItem` (slot `index`, or
/// -1 for the end). The item objects sit at `self + 0x130 .. 0x2ec`.
/// Frame 0xb0: saves ra and s0-s7, fp.
pub fn shell_renderer_menu_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 169;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, 16, 0xa0);
        frame.save(c, 17, 0x90);
        frame.save(c, 18, 0x80);
        frame.save(c, 19, 0x70);
        frame.save(c, 20, 0x60);
        frame.save(c, 21, 0x50);
        frame.save(c, 22, 0x40);
        frame.save(c, 23, 0x30);
        frame.save(c, 30, 0x20);
        frame.save_ra(c, 0x10);

        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = sx(oc683_off(c.g[S0][0], 0x130));
        c.g[S3][0] = sx(oc683_off(c.g[S0][0], 0x224));
        c.g[S2][0] = sx(oc683_off(c.g[S0][0], 0x284));
        c.g[S5][0] = sx(oc683_off(c.g[S0][0], 0x1c4));
        c.g[S6][0] = sx(oc683_off(c.g[S0][0], 0x254));
        c.g[S7][0] = sx(oc683_off(c.g[S0][0], 0x1f4));
        c.g[FP][0] = sx(oc683_off(c.g[S0][0], 0x164));
        let slot0 = frame.at(c, 0);
        c.sw(slot0, c.g[V0][0]);
        c.g[S4][0] = sx(oc683_off(c.g[S0][0], 0x2b8));
        // cMenu_cMenu(self): the base menu constructor (A0 is still self).
        call_ptr(c, 0x002c_aa58, 0x0024_9fd8)?;

        c.g[S1][0] = sx(oc683_off(c.g[S0][0], 0x2ec));
        c.g[V0][0] = sx(oc683_off(c.g[S0][0], 0x14c));
        c.g[A1][0] = 0x0048_0000;
        let slot4 = frame.at(c, 4);
        c.sw(slot4, c.g[V0][0]);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x1a10));
        c.g[V0][0] = sx(0x0048_0000u32.wrapping_sub(0x588));
        let a12c = oc683_off(c.g[S0][0], 0x12c);
        c.sw(a12c, c.g[V0][0]);
        c.g[A0][0] = c.lw(slot0);
        call_ptr(c, 0x002c_d2a0, 0x0024_a000)?;
        c.g[A0][0] = c.lw(slot4);
        c.g[A1][0] = 0x28;
        call_ptr(c, 0x002c_d350, 0x0024_a00c)?;

        c.g[V0][0] = sx(oc683_off(c.g[S0][0], 0x194));
        c.g[T1][0] = 0x004a_0000;
        let slot8 = frame.at(c, 8);
        c.sw(slot8, c.g[V0][0]);
        c.g[A0][0] = c.g[FP][0];
        c.g[T1][0] = sx(0x004a_0000u32.wrapping_add(0x2d30));
        c.g[A1][0] = 0;
        c.g[A2][0] = sx(oc683_off(c.g[GP][0], 0x15c8));
        c.g[A3][0] = 0;
        c.g[T0][0] = 1;
        call_ptr(c, 0x002c_d8f8, 0x0024_a034)?;

        c.g[A0][0] = c.lw(slot8);
        c.g[T1][0] = sx(0x0048_0000u32.wrapping_sub(0x19f8));
        c.g[A1][0] = 1;
        c.g[A2][0] = sx(oc683_off(c.g[GP][0], 0x15cc));
        c.g[A3][0] = 0;
        c.g[T0][0] = 0xc8;
        call_ptr(c, 0x002c_d8f8, 0x0024_a054)?;

        c.g[T1][0] = sx(0x0048_0000u32.wrapping_sub(0x19e0));
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = 2;
        c.g[A2][0] = sx(oc683_off(c.g[GP][0], 0x15d0));
        c.g[A3][0] = 0;
        c.g[T0][0] = 0xc8;
        call_ptr(c, 0x002c_d8f8, 0x0024_a074)?;

        c.g[T1][0] = sx(0x0048_0000u32.wrapping_sub(0x1b90));
        c.g[A0][0] = c.g[S7][0];
        c.g[A1][0] = 3;
        c.g[A2][0] = sx(oc683_off(c.g[GP][0], 0x15d4));
        c.g[A3][0] = 1;
        c.g[T0][0] = 0x2710;
        call_ptr(c, 0x002c_d8f8, 0x0024_a094)?;

        c.g[T1][0] = sx(0x0048_0000u32.wrapping_sub(0x1b80));
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 4;
        c.g[A2][0] = sx(oc683_off(c.g[GP][0], 0x15d8));
        c.g[A3][0] = 1;
        c.g[T0][0] = 0x2710;
        call_ptr(c, 0x002c_d8f8, 0x0024_a0b4)?;

        c.g[T0][0] = 0x10000;
        c.g[T1][0] = sx(0x0048_0000u32.wrapping_sub(0x19c8));
        c.g[T0][0] |= 0x86a0;
        c.g[A0][0] = c.g[S6][0];
        c.g[A3][0] = 1;
        c.g[A2][0] = sx(oc683_off(c.g[GP][0], 0x15dc));
        c.g[A1][0] = 5;
        call_ptr(c, 0x002c_d8f8, 0x0024_a0d8)?;

        // Float arguments: f12 = *(gp - 0x4fec); f13 from the AT constant.
        c.lwc1(12, oc683_off(c.g[GP][0], -0x4fec));
        c.g[A3][0] = 0x0048_0000;
        c.g[AT][0] = 0x4000_0000;
        c.mtc1(1, 13);
        c.g[A3][0] = sx(0x0048_0000u32.wrapping_sub(0x19b8));
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = sx(oc683_off(c.g[GP][0], 0x15e0));
        c.g[A1][0] = 6;
        call_ptr(c, 0x002c_dce8, 0x0024_a0fc)?;

        c.mtc1(0, 12);
        c.g[A3][0] = 0x0048_0000;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 13);
        c.g[A0][0] = c.g[S4][0];
        c.g[A3][0] = sx(0x0048_0000u32.wrapping_sub(0x19a8));
        c.g[A2][0] = sx(oc683_off(c.g[GP][0], 0x15e4));
        c.g[A1][0] = 7;
        call_ptr(c, 0x002c_dce8, 0x0024_a120)?;

        c.g[T1][0] = sx(0x0048_0000u32.wrapping_sub(0x1990));
        c.g[A3][0] = 1;
        c.g[T0][0] = 0x40;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = sx(oc683_off(c.g[GP][0], 0x15e8));
        c.g[A1][0] = 8;
        call_ptr(c, 0x002c_d8f8, 0x0024_a140)?;

        // cMenu_addItem(self, item, -1) for the nine items, in order.
        c.g[A1][0] = c.lw(slot0);
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x0024_a150)?;
        c.g[A1][0] = c.lw(slot4);
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x0024_a160)?;
        c.g[A1][0] = c.g[FP][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x0024_a170)?;
        c.g[A1][0] = c.lw(slot8);
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x0024_a180)?;
        c.g[A1][0] = c.g[S5][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x0024_a190)?;
        c.g[A1][0] = c.g[S7][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x0024_a1a0)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x0024_a1b0)?;
        c.g[A1][0] = c.g[S6][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x0024_a1c0)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x0024_a1d0)?;
        c.g[A1][0] = c.g[S4][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x0024_a1e0)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x0024_a1f0)?;

        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, 17, 0x90);
        frame.restore(c, 16, 0xa0);
        frame.restore(c, 18, 0x80);
        frame.restore(c, 19, 0x70);
        frame.restore(c, 20, 0x60);
        frame.restore(c, 21, 0x50);
        frame.restore(c, 22, 0x40);
        frame.restore(c, 23, 0x30);
        frame.restore(c, 30, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;

    use crate::difftest::fakes::LOG_RESULTS;

    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;

    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_18 as base_5;

    pub(crate) use crate::game::fe::feasyncfile::tests::diff;

    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;

    pub(crate) use crate::game::fe::feasyncfile::tests::strict_2 as strict;

    pub(crate) use crate::game::fe::feasyncfile::tests::w;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3 as OBJ;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_17 as SEEDS_4;

    pub(crate) use crate::game::fe::fereal::tests::base;
    pub(crate) use crate::game::input::inputmap::tests::alloc;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_2;
    pub(crate) use crate::game::world::worldcache::tests::check_all;
    pub(crate) use crate::game::world::wscriptcache::tests::SEEDS;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn mem_init_block_header_matches() {
        check(
            ssx3_ee::s06::f_002516d0,
            mem_init_block_header,
            100,
            &|c, seed| {
                let mut r = Rng(0x2516_d000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.g[T0][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.g[T1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.g[T2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                for off in [0u32, 2, 4, 8, 0xc] {
                    c.bus.write32(DATA + off, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn aivisual_effects_main_menu_ctor_matches() {
        check(
            ee::s06::f_002509d8,
            aivisual_effects_main_menu_ctor,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x09d8, 0);
                c.g[A0][0] = DATA as u64;
                crate::difftest::fill(c, &mut r, DATA, 0x200 / 4);
            },
        );
    }

    #[test]
    fn aivisual_effects_main_menu_dtor_matches() {
        check(
            ee::s06::f_00251630,
            aivisual_effects_main_menu_dtor,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1630, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                crate::difftest::fill(c, &mut r, DATA, 0x200 / 4);
            },
        );
    }

    #[test]
    fn aivisual_effects_main_menu_activate_match() {
        for (gen, hand, salt) in [
            (
                ee::s06::f_00250a60 as fn(&mut Cpu),
                aivisual_effects_main_menu_on_activate as fn(&mut Cpu),
                0x0a60u64,
            ),
            (
                ee::s06::f_00250a80,
                aivisual_effects_main_menu_on_deactivate,
                0x0a80,
            ),
        ] {
            check(gen, hand, SEEDS, &|c, seed| {
                let mut r = machine(c, seed, salt, 0);
                c.g[A0][0] = DATA as u64;
                crate::difftest::fill(c, &mut r, DATA, 0x100 / 4);
            });
        }
    }

    #[test]
    fn aivisual_effects_menus_globals_match() {
        for (gen, hand, salt) in [
            (
                ee::s06::f_00251690 as fn(&mut Cpu),
                aivisual_effects_menus_global_ctors as fn(&mut Cpu),
                0x1690u64,
            ),
            (
                ee::s06::f_002516b0,
                aivisual_effects_menus_global_dtors,
                0x16b0,
            ),
        ] {
            check(gen, hand, SEEDS, &|c, seed| {
                let _ = machine(c, seed, salt, 0);
            });
        }
    }

    #[test]
    fn avalanche_menu_hooks_match() {
        for (gen, hand, salt) in [
            (
                ssx3_ee::s06::f_002515f0 as fn(&mut Cpu),
                avalanche_menu_on_activate as fn(&mut Cpu),
                0x515,
            ),
            (
                ssx3_ee::s06::f_00251610,
                avalanche_menu_on_deactivate,
                0x610,
            ),
        ] {
            check(gen, hand, 60, &|c, seed| {
                let mut r = machine(c, seed, salt, 1);
                c.g[A0][0] = DATA as u64;
                let _ = &mut r;
            });
        }
    }

    // -----------------------------------------------------------------------

    #[test]
    fn flags_menu_dtor_matches() {
        check(
            ee::s06::f_0024ff28__generated,
            flags_menu_dtor,
            40,
            &|c, seed| {
                base(c, seed, 0x24_ff28);
                c.g[A0][0] = sx(DATA);
                // Even flags: the base destructor must not free the test object
                // (the sub-objects are test data, not heap blocks).
                c.g[A1][0] = [0u64, 2, 4][seed as usize % 3];
                for off in [0x130u32, 0x14c, 0x164, 0x194] {
                    c.bus.write32(DATA + off + 0x10, 0x1111);
                    c.bus.write32(DATA + off + 0x12c, 0x2222);
                }
            },
        );
    }

    #[test]
    fn height_fog_menu_dtor_matches() {
        check(
            ee::s06::f_0024fa00__generated,
            height_fog_menu_dtor,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x24_fa00, 4);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x0024_fa00;
            },
        );
    }

    #[test]
    fn tube_menu_dtor_matches() {
        check(
            ee::s06::f_00250090__generated,
            tube_menu_dtor,
            SEEDS_3,
            &alloc_base_setup(|c, seed, _r| {
                let this = alloc(c, 0x400);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed & 1) as u64;
            }),
        );
    }

    #[test]
    fn snow_surface_menu_dtor_matches() {
        check(
            ee::s06::f_0024fe00__generated,
            snow_surface_menu_dtor,
            8,
            &|c, seed| {
                let _ = base_3(c, seed, 0x24fe00, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(c, OBJ + 0x130, 0);
                w(c, OBJ + 0x164, 0);
            },
        );
    }

    // -------------------------------------------------------------- Menu dtor.

    #[test]
    fn shell_renderer_menu_dtor_matches() {
        check(
            ee::s06::f_0024fbc0__generated,
            shell_renderer_menu_dtor,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x24_fbc0, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = [0u64, 2, 4][seed as usize % 3];
                for off in [
                    0x130u32, 0x14c, 0x164, 0x194, 0x1c4, 0x1f4, 0x224, 0x254, 0x284, 0x2b8, 0x2ec,
                ] {
                    c.bus.write32(DATA + off + 0x10, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn shadow_menu_dtor_matches() {
        check(
            ee::s06::f_0024fd00__generated,
            shadow_menu_dtor,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x24_fd00, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = [0u64, 2, 4][seed as usize % 3];
                for off in [
                    0x130u32, 0x14c, 0x164, 0x194, 0x1c4, 0x1f8, 0x22c, 0x260, 0x294, 0x2c8, 0x2fc,
                ] {
                    c.bus.write32(DATA + off + 0x10, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn detail_system_menu_dtor_matches() {
        check(
            ee::s06::f_0024fab8__generated,
            detail_system_menu_dtor,
            60,
            &|c, seed| {
                let _r = base_4(c, seed, 0x24fa);
                c.g[A0][0] = DATA as u64;
                // Bit 0 of the flags would free the (test) object.
                c.g[A1][0] = 2;
            },
        );
    }

    #[test]
    fn plants_menu_dtor_matches() {
        diff(
            ee::s06::f_0024f860__generated,
            plants_menu_dtor,
            30,
            &|c, seed| {
                let _r = fe_base(c, seed, 0x24_f860);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 3) as u64;
                for off in (0x130..0x3a0).step_by(4) {
                    w(c, DATA + off, 0);
                }
            },
        );
    }

    #[test]
    fn depth_fog_menu_dtor_matches() {
        strict(
            ssx3_ee::s06::f_0024dc68__generated,
            depth_fog_menu_dtor,
            SEEDS_4,
            &|c, seed| {
                let _ = base_5(c, seed, 0xdc68);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(if seed % 2 == 0 { 0 } else { 2 });
            },
        );
    }

    /// The allocator's mutex (`MEM_MUTEX`) and the fake result that keeps the
    /// delete path short.
    pub(crate) const MEM_MUTEX: u32 = 0x0053_8b00;

    pub(crate) fn alloc_setup(c: &mut Cpu, r: &mut Rng) {
        w(c, MEM_MUTEX + 4, 0x42);
        w(c, LOG_RESULTS, 0x42);
        w(c, MEM_MUTEX + 8, r.next_u32() % 4);
    }

    /// The deleting-destructor fixture: a random page pointer, both values of
    /// the delete flag and a short `operator delete` path.
    pub(crate) fn dtor_case(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 40, &|c, seed| {
            let mut r = machine(c, seed, salt, 2);
            alloc_setup(c, &mut r);
            c.g[A0][0] = ptr(&mut r) as u64;
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    #[test]
    fn visual_effects_test_menu_dtor_matches() {
        dtor_case(
            ee::s06::f_0024d998__generated,
            visual_effects_test_menu_dtor,
            0x24_d998,
        );
    }

    #[test]
    fn visual_effects_main_menu_dtor_matches() {
        dtor_case(
            ee::s06::f_0024f6c0__generated,
            visual_effects_main_menu_dtor,
            0x24_f6c0,
        );
    }

    #[test]
    fn snowfall_toggles_menu_dtor_matches() {
        dtor_case(
            ee::s06::f_0024e200__generated,
            snowfall_toggles_menu_dtor,
            0x24_e200,
        );
    }

    #[test]
    fn render_toggles_menu_dtor_matches() {
        check_all(
            ee::s06::f_0024d710__generated,
            render_toggles_menu_dtor,
            24,
            &|c, seed| {
                let mut r = machine(c, seed, 0x24d710, 0);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x40);
                // Destructor flags with bit 0 clear, so the items do not take the
                // `operator delete` branch.
                c.g[A1][0] = ((r.next_u32() as u64) << 1) | 0x2;
            },
        );
    }

    use crate::difftest::GAME_GP;

    #[test]
    fn splash_toggles_menu_ctor_matches() {
        check(
            ee::s06::f_0024bf60__generated,
            splash_toggles_menu_ctor,
            40,
            &|c, seed| {
                let mut r = Rng(0x0024_bf60 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn render_toggles_menu_ctor_matches() {
        check(
            ee::s06::f_00247e60__generated,
            render_toggles_menu_ctor,
            100,
            &|c, seed| {
                machine(c, seed, 0x7e60, 0);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    use crate::difftest::fresh;
    use crate::difftest::run_generated;

    /// `init` (a0) zero: the dtor chain; non-zero: the ctor chain; `prio`
    /// (a1) 0xFFFF runs the body at all.
    pub(crate) fn setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 0xe8d8, 0);
        c.g[A0][0] = (seed % 2) as u64;
        c.g[A1][0] = if seed % 5 == 0 { 0 } else { 0xFFFF };
        c.g[A2][0] = DATA as u64;
        c.pc = 0x0024_e8d8;
    }

    #[test]
    fn visual_effects_menus_static_init_destroy_paths() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..200 {
            let mut c = fresh();
            setup(&mut c, seed);
            let b = c.cycles;
            run_generated(&mut c, ee::s06::f_0024e8d8__generated);
            seen.insert(c.cycles - b);
        }
        assert!(seen.len() >= 3, "paths: {seen:?}");
    }

    #[test]
    fn visual_effects_menus_static_init_destroy_matches() {
        check(
            ee::s06::f_0024e8d8__generated,
            visual_effects_menus_static_init_destroy,
            200,
            &setup,
        );
    }

    #[test]
    fn light_glow_menu_ctor_matches() {
        check(
            ee::s06::f_00249148__generated,
            light_glow_menu_ctor,
            100,
            &|c, seed| {
                machine(c, seed, 0x0024_9148, 0);
                c.g[A0][0] = DATA as u64;
                c.pc = 0x0024_9148;
            },
        );
    }

    #[test]
    fn screen_tint_menu_ctor_matches() {
        check(
            ee::s06::f_0024a228__generated,
            screen_tint_menu_ctor,
            40,
            &|c, seed| {
                let mut r = Rng(0x0024_a228 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.g[A0][0] = sx(DATA);
                c.pc = 0x0024_a228;
            },
        );
    }

    #[test]
    fn board_trail_menu_ctor_matches() {
        check(
            ee::s06::f_0024b0e8__generated,
            board_trail_menu_ctor,
            100,
            &|c, seed| {
                machine(c, seed, 0x0024_b0e8, 0);
                c.g[A0][0] = DATA as u64;
                c.pc = 0x0024_b0e8;
            },
        );
    }

    #[test]
    fn shadow_menu_ctor_matches() {
        check(
            ee::s06::f_0024abb8__generated,
            shadow_menu_ctor,
            100,
            &|c, seed| {
                machine(c, seed, 0x0024_abb8, 0);
                c.g[A0][0] = DATA as u64;
                c.pc = 0x0024_abb8;
            },
        );
    }

    #[test]
    fn snow_surface_menu_ctor_matches() {
        check(
            ee::s06::f_0024ae90__generated,
            snow_surface_menu_ctor,
            100,
            &|c, seed| {
                machine(c, seed, 0x0024_ae90, 0);
                c.g[A0][0] = DATA as u64;
                c.pc = 0x0024_ae90;
            },
        );
    }

    #[test]
    fn flags_menu_ctor_matches() {
        check(
            ee::s06::f_0024b6d0__generated,
            flags_menu_ctor,
            100,
            &|c, seed| {
                machine(c, seed, 0x0024_b6d0, 0);
                c.g[A0][0] = DATA as u64;
                c.pc = 0x0024_b6d0;
            },
        );
    }

    #[test]
    fn power_up_fx_toggles_menu_ctor_matches() {
        check(
            ee::s06::f_0024b7d0__generated,
            power_up_fx_toggles_menu_ctor,
            100,
            &|c, seed| {
                machine(c, seed, 0x0024_b7d0, 0);
                c.g[A0][0] = DATA as u64;
                c.pc = 0x0024_b7d0;
            },
        );
    }

    #[test]
    fn snowfall_toggles_menu_ctor_matches() {
        check(
            ee::s06::f_0024bbe8__generated,
            snowfall_toggles_menu_ctor,
            100,
            &|c, seed| {
                machine(c, seed, 0x0024_bbe8, 0);
                c.g[A0][0] = DATA as u64;
                c.pc = 0x0024_bbe8;
            },
        );
    }

    #[test]
    fn tube_menu_ctor_matches() {
        check(
            ee::s06::f_0024cca0__generated,
            tube_menu_ctor,
            100,
            &|c, seed| {
                machine(c, seed, 0x0024_cca0, 0);
                c.g[A0][0] = DATA as u64;
                c.pc = 0x0024_cca0;
            },
        );
    }

    #[test]
    fn path_arrow_menu_ctor_matches() {
        check(
            ee::s06::f_0024ceb0__generated,
            path_arrow_menu_ctor,
            100,
            &|c, seed| {
                machine(c, seed, 0x0024_ceb0, 0);
                c.g[A0][0] = DATA as u64;
                c.pc = 0x0024_ceb0;
            },
        );
    }

    #[test]
    fn visual_effects_main_menu_ctor_matches() {
        check(
            ee::s06::f_0024d2f0__generated,
            visual_effects_main_menu_ctor,
            100,
            &|c, seed| {
                machine(c, seed, 0x0024_d2f0, 0);
                c.g[A0][0] = DATA as u64;
                c.pc = 0x0024_d2f0;
            },
        );
    }

    #[test]
    fn visual_effects_main_menu_on_activate_matches() {
        check(
            ee::s06::f_0024d690__generated,
            visual_effects_main_menu_on_activate,
            100,
            &|c, seed| {
                machine(c, seed, 0x0024_d690, 0);
                c.g[A0][0] = DATA as u64;
                c.pc = 0x0024_d690;
            },
        );
    }

    #[test]
    fn visual_effects_main_menu_on_deactivate_matches() {
        check(
            ee::s06::f_0024d6b0__generated,
            visual_effects_main_menu_on_deactivate,
            100,
            &|c, seed| {
                machine(c, seed, 0x0024_d6b0, 0);
                c.g[A0][0] = DATA as u64;
                c.pc = 0x0024_d6b0;
            },
        );
    }

    #[test]
    fn light_glow_menu_dtor_matches() {
        check(
            ee::s06::f_0024dab8__generated,
            light_glow_menu_dtor_2,
            100,
            &|c, seed| {
                machine(c, seed, 0x0024_dab8, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 0;
                c.pc = 0x0024_dab8;
            },
        );
    }

    #[test]
    fn screen_tint_menu_dtor_matches() {
        check(
            ee::s06::f_0024dd70__generated,
            screen_tint_menu_dtor_2,
            100,
            &|c, seed| {
                machine(c, seed, 0x0024_dd70, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 0;
                c.pc = 0x0024_dd70;
            },
        );
    }

    #[test]
    fn board_trail_menu_dtor_matches() {
        check(
            ee::s06::f_0024dfa0__generated,
            board_trail_menu_dtor_2,
            100,
            &|c, seed| {
                machine(c, seed, 0x0024_dfa0, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 0;
                c.pc = 0x0024_dfa0;
            },
        );
    }

    #[test]
    fn power_up_fx_toggles_menu_dtor_matches() {
        check(
            ee::s06::f_0024e0f8__generated,
            power_up_fx_toggles_menu_dtor_2,
            100,
            &|c, seed| {
                machine(c, seed, 0x0024_e0f8, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 0;
                c.pc = 0x0024_e0f8;
            },
        );
    }

    use crate::difftest::fill;

    use crate::difftest::fakes;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// A menu-page thunk: `this` at `DATA`, random junk in every register.
    pub(crate) fn thunk_case(generated: GuestFn, hand: GuestFn, entry: u32) {
        check(generated, hand, 100, &|c, seed| {
            machine(c, seed, entry as u64, 0);
            c.g[A0][0] = DATA as u64;
            c.pc = entry;
        });
    }

    /// A destructor: `this` in the fixture area, flags with bit 0 clear so the
    /// items do not take the `operator delete` path.
    pub(crate) fn dtor_case_2(generated: GuestFn, hand: GuestFn, entry: u32, salt: u64) {
        check(generated, hand, 24, &|c, seed| {
            let mut r = machine(c, seed, salt, 0);
            c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x40);
            c.g[A1][0] = ((r.next_u32() as u64) << 1) | 0x2;
            c.pc = entry;
        });
    }

    #[test]
    fn camera_splash_menu_dtor_matches() {
        dtor_case_2(
            ee::s06::f_0024e388__generated,
            camera_splash_menu_dtor,
            0x0024_e388,
            0x24_e388,
        );
    }

    #[test]
    fn path_arrow_menu_dtor_matches() {
        dtor_case_2(
            ee::s06::f_0024e7c8__generated,
            path_arrow_menu_dtor,
            0x0024_e7c8,
            0x24_e7c8,
        );
    }

    #[test]
    fn render_toggles_menu_on_activate_matches() {
        thunk_case(
            ee::s06::f_0024f7e0__generated,
            render_toggles_menu_on_activate,
            0x0024f7e0,
        );
    }

    #[test]
    fn render_toggles_menu_on_deactivate_matches() {
        thunk_case(
            ee::s06::f_0024f800__generated,
            render_toggles_menu_on_deactivate,
            0x0024f800,
        );
    }

    #[test]
    fn visual_effects_test_menu_on_activate_matches() {
        thunk_case(
            ee::s06::f_0024f820__generated,
            visual_effects_test_menu_on_activate,
            0x0024f820,
        );
    }

    #[test]
    fn visual_effects_test_menu_on_deactivate_matches() {
        thunk_case(
            ee::s06::f_0024f840__generated,
            visual_effects_test_menu_on_deactivate,
            0x0024f840,
        );
    }

    #[test]
    fn plants_menu_on_activate_matches() {
        thunk_case(
            ee::s06::f_0024f940__generated,
            plants_menu_on_activate,
            0x0024f940,
        );
    }

    #[test]
    fn plants_menu_on_deactivate_matches() {
        thunk_case(
            ee::s06::f_0024f960__generated,
            plants_menu_on_deactivate,
            0x0024f960,
        );
    }

    #[test]
    fn light_glow_menu_on_activate_matches() {
        thunk_case(
            ee::s06::f_0024f980__generated,
            light_glow_menu_on_activate,
            0x0024f980,
        );
    }

    #[test]
    fn light_glow_menu_on_deactivate_matches() {
        thunk_case(
            ee::s06::f_0024f9a0__generated,
            light_glow_menu_on_deactivate,
            0x0024f9a0,
        );
    }

    #[test]
    fn depth_fog_menu_on_activate_matches() {
        thunk_case(
            ee::s06::f_0024f9c0__generated,
            depth_fog_menu_on_activate,
            0x0024f9c0,
        );
    }

    #[test]
    fn depth_fog_menu_on_deactivate_matches() {
        thunk_case(
            ee::s06::f_0024f9e0__generated,
            depth_fog_menu_on_deactivate,
            0x0024f9e0,
        );
    }

    #[test]
    fn height_fog_menu_on_activate_matches() {
        thunk_case(
            ee::s06::f_0024fa78__generated,
            height_fog_menu_on_activate,
            0x0024fa78,
        );
    }

    #[test]
    fn height_fog_menu_on_deactivate_matches() {
        thunk_case(
            ee::s06::f_0024fa98__generated,
            height_fog_menu_on_deactivate,
            0x0024fa98,
        );
    }

    #[test]
    fn detail_system_menu_on_activate_matches() {
        thunk_case(
            ee::s06::f_0024fb80__generated,
            detail_system_menu_on_activate,
            0x0024fb80,
        );
    }

    #[test]
    fn detail_system_menu_on_deactivate_matches() {
        thunk_case(
            ee::s06::f_0024fba0__generated,
            detail_system_menu_on_deactivate,
            0x0024fba0,
        );
    }

    #[test]
    fn shell_renderer_menu_on_activate_matches() {
        thunk_case(
            ee::s06::f_0024fc80__generated,
            shell_renderer_menu_on_activate,
            0x0024fc80,
        );
    }

    #[test]
    fn shell_renderer_menu_on_deactivate_matches() {
        thunk_case(
            ee::s06::f_0024fca0__generated,
            shell_renderer_menu_on_deactivate,
            0x0024fca0,
        );
    }

    #[test]
    fn screen_tint_menu_on_activate_matches() {
        thunk_case(
            ee::s06::f_0024fcc0__generated,
            screen_tint_menu_on_activate,
            0x0024fcc0,
        );
    }

    #[test]
    fn screen_tint_menu_on_deactivate_matches() {
        thunk_case(
            ee::s06::f_0024fce0__generated,
            screen_tint_menu_on_deactivate,
            0x0024fce0,
        );
    }

    pub(crate) const SEEDS_5: u32 = 100;

    /// A menu page wrapper's fixture: `a0` points at random page data.
    pub(crate) fn page_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        c.g[A0][0] = DATA as u64;
        fill(c, &mut r, DATA, 0x100 / 4);
    }

    #[test]
    fn shadow_menu_on_activate_matches() {
        check(
            ee::s06::f_0024fdc0__generated,
            shadow_menu_on_activate,
            SEEDS_5,
            &|c, seed| page_setup(c, seed, 0x0024_fdc0),
        );
    }

    #[test]
    fn shadow_menu_on_deactivate_matches() {
        check(
            ee::s06::f_0024fde0__generated,
            shadow_menu_on_deactivate,
            SEEDS_5,
            &|c, seed| page_setup(c, seed, 0x0024_fde0),
        );
    }

    #[test]
    fn snow_surface_menu_on_activate_matches() {
        check(
            ee::s06::f_0024fea8__generated,
            snow_surface_menu_on_activate,
            SEEDS_5,
            &|c, seed| page_setup(c, seed, 0x0024_fea8),
        );
    }

    #[test]
    fn snow_surface_menu_on_deactivate_matches() {
        check(
            ee::s06::f_0024fec8__generated,
            snow_surface_menu_on_deactivate,
            SEEDS_5,
            &|c, seed| page_setup(c, seed, 0x0024_fec8),
        );
    }

    #[test]
    fn board_trail_menu_on_activate_matches() {
        check(
            ee::s06::f_0024fee8__generated,
            board_trail_menu_on_activate,
            SEEDS_5,
            &|c, seed| page_setup(c, seed, 0x0024_fee8),
        );
    }

    #[test]
    fn board_trail_menu_on_deactivate_matches() {
        check(
            ee::s06::f_0024ff08__generated,
            board_trail_menu_on_deactivate,
            SEEDS_5,
            &|c, seed| page_setup(c, seed, 0x0024_ff08),
        );
    }

    #[test]
    fn flags_menu_on_activate_matches() {
        check(
            ee::s06::f_0024ff90__generated,
            flags_menu_on_activate,
            SEEDS_5,
            &|c, seed| page_setup(c, seed, 0x0024_ff90),
        );
    }

    #[test]
    fn flags_menu_on_deactivate_matches() {
        check(
            ee::s06::f_0024ffb0__generated,
            flags_menu_on_deactivate,
            SEEDS_5,
            &|c, seed| page_setup(c, seed, 0x0024_ffb0),
        );
    }

    #[test]
    fn power_up_fx_toggles_menu_on_activate_matches() {
        check(
            ee::s06::f_0024ffd0__generated,
            power_up_fx_toggles_menu_on_activate,
            SEEDS_5,
            &|c, seed| page_setup(c, seed, 0x0024_ffd0),
        );
    }

    #[test]
    fn power_up_fx_toggles_menu_on_deactivate_matches() {
        check(
            ee::s06::f_0024fff0__generated,
            power_up_fx_toggles_menu_on_deactivate,
            SEEDS_5,
            &|c, seed| page_setup(c, seed, 0x0024_fff0),
        );
    }

    #[test]
    fn snowfall_toggles_menu_on_activate_matches() {
        check(
            ee::s06::f_00250010__generated,
            snowfall_toggles_menu_on_activate,
            SEEDS_5,
            &|c, seed| page_setup(c, seed, 0x0025_0010),
        );
    }

    #[test]
    fn snowfall_toggles_menu_on_deactivate_matches() {
        check(
            ee::s06::f_00250030__generated,
            snowfall_toggles_menu_on_deactivate,
            SEEDS_5,
            &|c, seed| page_setup(c, seed, 0x0025_0030),
        );
    }

    #[test]
    fn camera_splash_menu_on_activate_matches() {
        check(
            ee::s06::f_00250050__generated,
            camera_splash_menu_on_activate,
            SEEDS_5,
            &|c, seed| page_setup(c, seed, 0x0025_0050),
        );
    }

    #[test]
    fn camera_splash_menu_on_deactivate_matches() {
        check(
            ee::s06::f_00250070__generated,
            camera_splash_menu_on_deactivate,
            SEEDS_5,
            &|c, seed| page_setup(c, seed, 0x0025_0070),
        );
    }

    #[test]
    fn tube_menu_on_activate_matches() {
        check(
            ee::s06::f_00250128__generated,
            tube_menu_on_activate,
            SEEDS_5,
            &|c, seed| page_setup(c, seed, 0x0025_0128),
        );
    }

    #[test]
    fn tube_menu_on_deactivate_matches() {
        check(
            ee::s06::f_00250148__generated,
            tube_menu_on_deactivate,
            SEEDS_5,
            &|c, seed| page_setup(c, seed, 0x0025_0148),
        );
    }

    #[test]
    fn path_arrow_menu_on_activate_matches() {
        check(
            ee::s06::f_00250168__generated,
            path_arrow_menu_on_activate,
            SEEDS_5,
            &|c, seed| page_setup(c, seed, 0x0025_0168),
        );
    }

    #[test]
    fn path_arrow_menu_on_deactivate_matches() {
        check(
            ee::s06::f_00250188__generated,
            path_arrow_menu_on_deactivate,
            SEEDS_5,
            &|c, seed| page_setup(c, seed, 0x0025_0188),
        );
    }

    #[test]
    fn visual_effects_menus_global_ctors_matches() {
        check(
            ee::s06::f_002501a8__generated,
            visual_effects_menus_global_ctors,
            SEEDS_5,
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
            SEEDS_5,
            &|c, seed| {
                let _ = machine(c, seed, 0x0025_01c8, 0);
            },
        );
    }

    /// The menu lives at `DATA`; the ctor writes the page fields and every
    /// item, the rest of the object is random junk.
    #[test]
    fn avalanche_menu_ctor_matches() {
        check(
            ssx3_ee::s06::f_002501e8__generated,
            avalanche_menu_ctor,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2501_e8, 0);
                fill(c, &mut r, DATA, 0x240);
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
            },
        );
    }

    /// Covers the constructor path (prio 0xffff, a0 != 0), the destructor path
    /// (a0 == 0) and the early return for another priority.
    #[test]
    fn aivisual_effects_menus_static_init_destroy_matches() {
        check(
            ee::s06::f_00250cb0__generated,
            aivisual_effects_menus_static_init_destroy,
            60,
            &|c, seed| {
                let mut r = Rng(0x250c_b0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0025_0cb0;
                c.g[A0][0] = sx(if seed % 3 == 0 { 0 } else { DATA });
                c.g[A1][0] = sx(if seed % 4 == 0 { r.next_u32() } else { 0xffff });
            },
        );
    }

    /// Gate word for the menu constructor's callee fakes (test machines only).
    pub(crate) const AVALANCHE_MENU_GATE: u32 = DATA + 0x200000;

    pub(crate) static OC531_PREV_MENU_ITEM: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC531_PREV_MENU: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn oc531_fake_menu_item(c: &mut Cpu) {
        if c.bus.read32(AVALANCHE_MENU_GATE) != 0x0531_0001 {
            return fakes::fall_back(c, &OC531_PREV_MENU_ITEM, ee::s09::f_002ca280__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, MENU_ITEM_CTOR, ra);
    }

    pub(crate) fn oc531_fake_menu(c: &mut Cpu) {
        if c.bus.read32(AVALANCHE_MENU_GATE) != 0x0531_0001 {
            return fakes::fall_back(c, &OC531_PREV_MENU, ee::s09::f_002caa80__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, MENU_CTOR, ra);
    }

    #[test]
    fn avalanche_menu_ctor_matches_2() {
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA280),
            oc531_fake_menu_item,
            &OC531_PREV_MENU_ITEM,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CAA80),
            oc531_fake_menu,
            &OC531_PREV_MENU,
        );
        check(
            ee::s06::f_00250ae0__generated,
            avalanche_menu_ctor_2,
            200,
            &|c, seed| {
                let mut r = Rng(0x0025_0ae0 ^ seed as u64);
                scramble(c, &mut r);
                c.bus.write32(AVALANCHE_MENU_GATE, 0x0531_0001);
                c.g[A0][0] = sx(DATA + 0x1000 * (seed % 4));
                c.g[A1][0] = r.next_u32() as u64;
                c.pc = 0x0025_0ae0;
            },
        );
    }

    /// The menu object both constructors fill (zeroed memory; the constructors
    /// write every field they read back).
    pub(crate) const OC651_MENU: u32 = DATA + 0x1000;

    /// A machine with `this` in a0 and the entry in `pc`.
    pub(crate) fn oc651_menu_setup(c: &mut Cpu, seed: u32, entry: u32) {
        let _ = fakes::machine(c, seed, 0x651_0001, 0);
        c.pc = entry;
        c.g[A0][0] = OC651_MENU as u64;
    }

    #[test]
    fn visual_effects_test_menu_ctor_matches() {
        check(
            ssx3_ee::s06::f_00248970__generated,
            visual_effects_test_menu_ctor,
            200,
            &|c, seed| oc651_menu_setup(c, seed, 0x0024_8970),
        );
    }

    #[test]
    fn plants_menu_ctor_matches() {
        check(
            ssx3_ee::s06::f_00248dc8__generated,
            plants_menu_ctor,
            200,
            &|c, seed| oc651_menu_setup(c, seed, 0x0024_8dc8),
        );
    }

    #[test]
    fn depth_fog_menu_ctor_matches() {
        check(
            ee::s06::f_002498a0__generated,
            depth_fog_menu_ctor,
            60,
            &|c, seed| {
                machine(c, seed, 0x98a0, 0);
                c.g[A0][0] = DATA as u64;
                c.pc = 0x0024_98a0;
            },
        );
    }

    #[test]
    fn height_fog_menu_ctor_matches() {
        check(
            ee::s06::f_00249b28__generated,
            height_fog_menu_ctor,
            60,
            &|c, seed| {
                machine(c, seed, 0x9b28, 0);
                c.g[A0][0] = DATA as u64;
                c.pc = 0x0024_9b28;
            },
        );
    }

    #[test]
    fn detail_system_menu_ctor_matches() {
        check(
            ee::s06::f_00249c68__generated,
            detail_system_menu_ctor,
            60,
            &|c, seed| {
                machine(c, seed, 0x9c68, 0);
                c.g[A0][0] = DATA as u64;
                c.pc = 0x0024_9c68;
            },
        );
    }

    /// `ShellRendererMenu_ctor(self)`: the base menu and nine items, built by
    /// the real menu constructors (as for the sibling `ScreenTintMenu_ctor`).
    #[test]
    fn shell_renderer_menu_ctor_matches() {
        check(
            ee::s06::f_00249f80__generated,
            shell_renderer_menu_ctor_2,
            40,
            &|c, seed| {
                let mut r = Rng(0x0024_9f80 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.g[A0][0] = sx(DATA);
                c.pc = 0x0024_9f80;
            },
        );
    }
}
