//! Staging module oc526 (wave wave231): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc526`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{self, call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0024_e388,
        camera_splash_menu_dtor,
        "CameraSplashMenu_dtor",
    ),
    (0x0024_e7c8, path_arrow_menu_dtor, "PathArrowMenu_dtor"),
    (
        0x0024f7e0,
        render_toggles_menu_on_activate,
        "RenderTogglesMenu_OnActivate",
    ),
    (
        0x0024f800,
        render_toggles_menu_on_deactivate,
        "RenderTogglesMenu_OnDeactivate",
    ),
    (
        0x0024f820,
        visual_effects_test_menu_on_activate,
        "VisualEffectsTestMenu_OnActivate",
    ),
    (
        0x0024f840,
        visual_effects_test_menu_on_deactivate,
        "VisualEffectsTestMenu_OnDeactivate",
    ),
    (0x0024f940, plants_menu_on_activate, "PlantsMenu_OnActivate"),
    (
        0x0024f960,
        plants_menu_on_deactivate,
        "PlantsMenu_OnDeactivate",
    ),
    (
        0x0024f980,
        light_glow_menu_on_activate,
        "LightGlowMenu_OnActivate",
    ),
    (
        0x0024f9a0,
        light_glow_menu_on_deactivate,
        "LightGlowMenu_OnDeactivate",
    ),
    (
        0x0024f9c0,
        depth_fog_menu_on_activate,
        "DepthFogMenu_OnActivate",
    ),
    (
        0x0024f9e0,
        depth_fog_menu_on_deactivate,
        "DepthFogMenu_OnDeactivate",
    ),
    (
        0x0024fa78,
        height_fog_menu_on_activate,
        "HeightFogMenu_OnActivate",
    ),
    (
        0x0024fa98,
        height_fog_menu_on_deactivate,
        "HeightFogMenu_OnDeactivate",
    ),
    (
        0x0024fb80,
        detail_system_menu_on_activate,
        "DetailSystemMenu_OnActivate",
    ),
    (
        0x0024fba0,
        detail_system_menu_on_deactivate,
        "DetailSystemMenu_OnDeactivate",
    ),
    (
        0x0024fc80,
        shell_renderer_menu_on_activate,
        "ShellRendererMenu_OnActivate",
    ),
    (
        0x0024fca0,
        shell_renderer_menu_on_deactivate,
        "ShellRendererMenu_OnDeactivate",
    ),
    (
        0x0024fcc0,
        screen_tint_menu_on_activate,
        "ScreenTintMenu_OnActivate",
    ),
    (
        0x0024fce0,
        screen_tint_menu_on_deactivate,
        "ScreenTintMenu_OnDeactivate",
    ),
];

/// `cMenuItem::~cMenuItem` (0x2ca280), the item destructor the menus chain.
const MENU_ITEM_DTOR_TARGET: u32 = 0x002c_a280;
/// `cMenu::~cMenu` (0x2caa80), the page destructor the menus end with.
const MENU_DTOR_TARGET: u32 = 0x002c_aa80;

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

fn menu_dtor_arg(c: &Cpu, this: u32, arg: MenuDtorArg) -> u64 {
    match arg {
        MenuDtorArg::This(off) => sx(this.wrapping_add(off)),
        MenuDtorArg::Reg(reg) => c.g[reg][0],
        MenuDtorArg::Imm(v) => v,
    }
}

/// Runs a destructor's steps for the object at `this`.
fn run_menu_dtor_steps(c: &mut Cpu, this: u32, steps: &[MenuDtorStep]) -> Flow {
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
const CAMERA_SPLASH_MENU_DTOR_STEPS: &[MenuDtorStep] = &[
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
const PATH_ARROW_MENU_DTOR_STEPS: &[MenuDtorStep] = &[
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;

    /// A menu-page thunk: `this` at `DATA`, random junk in every register.
    fn thunk_case(generated: GuestFn, hand: GuestFn, entry: u32) {
        check(generated, hand, 100, &|c, seed| {
            machine(c, seed, entry as u64, 0);
            c.g[A0][0] = DATA as u64;
            c.pc = entry;
        });
    }

    /// A destructor: `this` in the fixture area, flags with bit 0 clear so the
    /// items do not take the `operator delete` path.
    fn dtor_case(generated: GuestFn, hand: GuestFn, entry: u32, salt: u64) {
        check(generated, hand, 24, &|c, seed| {
            let mut r = machine(c, seed, salt, 0);
            c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x40);
            c.g[A1][0] = ((r.next_u32() as u64) << 1) | 0x2;
            c.pc = entry;
        });
    }

    #[test]
    fn camera_splash_menu_dtor_matches() {
        dtor_case(
            ee::s06::f_0024e388__generated,
            camera_splash_menu_dtor,
            0x0024_e388,
            0x24_e388,
        );
    }

    #[test]
    fn path_arrow_menu_dtor_matches() {
        dtor_case(
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
}
