//! Staging module oc525 (wave wave230): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc525`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0024_abb8, shadow_menu_ctor, "ShadowMenu_ctor"),
    (0x0024_ae90, snow_surface_menu_ctor, "SnowSurfaceMenu_ctor"),
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
    (0x0024_dab8, light_glow_menu_dtor, "LightGlowMenu_dtor"),
    (0x0024_dd70, screen_tint_menu_dtor, "ScreenTintMenu_dtor"),
    (0x0024_dfa0, board_trail_menu_dtor, "BoardTrailMenu_dtor"),
    (
        0x0024_e0f8,
        power_up_fx_toggles_menu_dtor,
        "PowerUpFXTogglesMenu_dtor",
    ),
];

/// `lw` of the frame slot `sp + off`: an item address the body kept there.
fn mctor_slot(c: &mut Cpu, frame: Frame, off: u32) -> u64 {
    let a = frame.at(c, off);
    c.lw(a)
}

/// `sw value, sp + off`: stash an item address in the frame.
fn mctor_stash(c: &mut Cpu, frame: Frame, off: u32, value: u64) {
    let a = frame.at(c, off);
    c.sw(a, value);
}

/// `this + off` as the 64-bit value a register holds (sign-extended).
fn mctor_addr(this: u64, off: u32) -> u64 {
    sx((this as u32).wrapping_add(off))
}

/// `cIntMenuItem(item, index, &gp[var], min, max, name)` (0x2cd8f8). Returns
/// to `ret`.
fn mctor_int(
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
fn mctor_float(c: &mut Cpu, item: u64, index: u64, var: u32, name: u32, ret: u32) -> Flow {
    c.g[A0][0] = item;
    c.g[A1][0] = index;
    c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(var));
    c.g[A3][0] = sx(name);
    call_ptr(c, 0x002c_dce8, ret)
}

/// `cMenu_addItem(this, item, -1)` (0x2cbcc8).
fn mctor_add(c: &mut Cpu, this: u64, item: u64, ret: u32) -> Flow {
    c.g[A1][0] = item;
    c.g[A0][0] = this;
    c.g[A2][0] = u64::MAX;
    call_ptr(c, 0x002c_bcc8, ret)
}

// Offsets of the shadow menu's items and vtable in the `cShadowMenu` object.
const SHADOW_VTBL: u32 = 0x12c;
const SHADOW_TITLE: u32 = 0x130;
const SHADOW_SPACE: u32 = 0x14c;
const SHADOW_ENABLE: u32 = 0x164;
const SHADOW_SHOW_POLYS: u32 = 0x194;
const SHADOW_DIR_X: u32 = 0x1c4;
const SHADOW_DIR_Y: u32 = 0x1f8;
const SHADOW_DIR_Z: u32 = 0x22c;
const SHADOW_LENGTH: u32 = 0x260;
const SHADOW_INTENSITY: u32 = 0x294;
const SHADOW_START_UV: u32 = 0x2c8;
const SHADOW_END_UV: u32 = 0x2fc;

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
fn vfx_item(c: &mut Cpu, item: u64, name: u64, extra: u64, ret: u32) -> Flow {
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
fn mdtor_item(c: &mut Cpu, item: u64, ret: u32) -> Flow {
    c.g[A0][0] = item;
    c.g[A1][0] = 2;
    call_ptr(c, 0x002c_a280, ret)
}

/// `cPowerUpFXTogglesMenu_dtor(this, flags)` (0x24e0f8): tears down the
/// seventeen items of the page in reverse build order, then the menu itself
/// (0x2caa80) with the same flags in a1. Returns to the caller.
pub fn power_up_fx_toggles_menu_dtor(c: &mut Cpu) {
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
pub fn board_trail_menu_dtor(c: &mut Cpu) {
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
pub fn light_glow_menu_dtor(c: &mut Cpu) {
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
pub fn screen_tint_menu_dtor(c: &mut Cpu) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, DATA};
    use ssx3_ee as ee;

    #[test]
    fn shadow_menu_ctor_matches() {
        check(ee::s06::f_0024abb8__generated, shadow_menu_ctor, 100, &|c, seed| {
            machine(c, seed, 0x0024_abb8, 0);
            c.g[A0][0] = DATA as u64;
            c.pc = 0x0024_abb8;
        });
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
        check(ee::s06::f_0024b6d0__generated, flags_menu_ctor, 100, &|c, seed| {
            machine(c, seed, 0x0024_b6d0, 0);
            c.g[A0][0] = DATA as u64;
            c.pc = 0x0024_b6d0;
        });
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
        check(ee::s06::f_0024cca0__generated, tube_menu_ctor, 100, &|c, seed| {
            machine(c, seed, 0x0024_cca0, 0);
            c.g[A0][0] = DATA as u64;
            c.pc = 0x0024_cca0;
        });
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
            light_glow_menu_dtor,
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
            screen_tint_menu_dtor,
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
            board_trail_menu_dtor,
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
            power_up_fx_toggles_menu_dtor,
            100,
            &|c, seed| {
                machine(c, seed, 0x0024_e0f8, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 0;
                c.pc = 0x0024_e0f8;
            },
        );
    }
}
