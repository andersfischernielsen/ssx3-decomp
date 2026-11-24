//! `src/main/debugmenu.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::fereal::open;
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
}
