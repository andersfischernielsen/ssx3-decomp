//! Staging module oc652 (wave g32.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc652`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::game::main::debugmenu::{
    add_item, argb_item, gp_at, int_item, item_value, this_at, Arg, Item,
};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0024_98a0, depth_fog_menu_ctor, "DepthFogMenu_ctor"),
    (0x0024_9b28, height_fog_menu_ctor, "HeightFogMenu_ctor"),
    (0x0024_9c68, detail_system_menu_ctor, "DetailSystemMenu_ctor"),
];

/// `this + off` into v0, then stored at frame slot `slot` (`sp + 4 * slot`).
fn menu_ctor_stash(c: &mut Cpu, slot: u32, off: u32) {
    c.g[V0][0] = this_at(c, off);
    let addr = sp(c).wrapping_add(4 * slot);
    let v = c.g[V0][0];
    c.sw(addr, v);
}

/// Load f-register `f` with a bound: a copy of an f-register, or a literal
/// moved through `at` with `mtc1`.
fn menu_ctor_bound(c: &mut Cpu, f: usize, bound: Arg) {
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
fn menu_ctor_float(
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
fn menu_ctor_float_gp(
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
        argb_item(
            c,
            Item::Reg(S2),
            7,
            0x004a_2d50,
            0x0050_4720,
            0x0024_9a48,
        )?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;

    #[test]
    fn depth_fog_menu_ctor_matches() {
        check(ee::s06::f_002498a0__generated, depth_fog_menu_ctor, 60, &|c, seed| {
            machine(c, seed, 0x98a0, 0);
            c.g[A0][0] = DATA as u64;
            c.pc = 0x0024_98a0;
        });
    }

    #[test]
    fn height_fog_menu_ctor_matches() {
        check(ee::s06::f_00249b28__generated, height_fog_menu_ctor, 60, &|c, seed| {
            machine(c, seed, 0x9b28, 0);
            c.g[A0][0] = DATA as u64;
            c.pc = 0x0024_9b28;
        });
    }

    #[test]
    fn detail_system_menu_ctor_matches() {
        check(ee::s06::f_00249c68__generated, detail_system_menu_ctor, 60, &|c, seed| {
            machine(c, seed, 0x9c68, 0);
            c.g[A0][0] = DATA as u64;
            c.pc = 0x0024_9c68;
        });
    }
}
