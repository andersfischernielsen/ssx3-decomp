//! Staging module oc472 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc472`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0024_bf60,
    splash_toggles_menu_ctor,
    "CameraSplashMenu_ctor",
)];

/// Offsets of the menu's items in the menu object, in the order the
/// constructor adds them (items 0..=50; items 51..=57 are kept in s4, s5,
/// s3, fp, s2, s7 and s6).
const ITEM_OFF: [u32; 51] = [
    0x130, 0x14c, 0x164, 0x194, 0x1c4, 0x1f4, 0x224, 0x258,
    0x28c, 0x2bc, 0x2ec, 0x31c, 0x350, 0x384, 0x3b8, 0x3ec,
    0x420, 0x454, 0x488, 0x4bc, 0x4f0, 0x524, 0x558, 0x58c,
    0x5c0, 0x5f4, 0x628, 0x65c, 0x690, 0x6c4, 0x6f8, 0x72c,
    0x760, 0x794, 0x7c8, 0x7fc, 0xe58, 0x14b4, 0x14e8, 0x151c,
    0x1550, 0x1584, 0x15b8, 0x15ec, 0x1620, 0x1654, 0x1688, 0x16bc,
    0x16f0, 0x1724, 0x1758,
];

/// Where an item's address is kept while the constructor runs: its frame
/// slot (`sp + 4 * i`) or a saved s-register.
#[derive(Clone, Copy)]
enum Item {
    Slot(usize),
    Reg(usize),
}

/// The second float argument (f13): a copy of an f-register, or a literal
/// loaded into `at` and moved with `mtc1`.
#[derive(Clone, Copy)]
enum Arg {
    Reg(usize),
    Lit(u64),
}

/// `this + off`, as the compiled code computes it from s0.
fn this_at(c: &Cpu, off: u32) -> u64 {
    sx((c.g[S0][0] as u32).wrapping_add(off))
}

/// `gp + off` (a pointer into the globals).
fn gp_at(c: &Cpu, off: u32) -> u64 {
    sx((c.g[GP][0] as u32).wrapping_add(off))
}

/// The frame slot that holds item `i`'s address.
fn slot_addr(c: &Cpu, i: usize) -> u32 {
    sp(c).wrapping_add(4 * i as u32)
}

/// `this->item[i] = this + ITEM_OFF[i]`: the address goes to the frame slot
/// and stays in v0.
fn store_item(c: &mut Cpu, i: usize) {
    c.g[V0][0] = this_at(c, ITEM_OFF[i]);
    let slot = slot_addr(c, i);
    let v = c.g[V0][0];
    Addr(slot).sw(c, v);
}

/// The value an item is passed as (its address).
fn item_value(c: &mut Cpu, item: Item) -> u64 {
    match item {
        Item::Slot(i) => {
            let slot = slot_addr(c, i);
            Addr(slot).lw(c)
        }
        Item::Reg(r) => c.g[r][0],
    }
}

/// `this->word[off] = v` (the menu object's own fields).
fn set_this_word(c: &mut Cpu, off: u32, v: u64) {
    let a = (c.g[S0][0] as u32).wrapping_add(off);
    Addr(a).sw(c, v);
}

/// `cIntMenuItem(item, index, &int at gp+value, min, max, label)` (0x2cd8f8).
fn int_item(
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
fn float_item(
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
fn argb_item(c: &mut Cpu, item: Item, index: u64, label: u64, colour: u64, ret: u32) -> Flow {
    let this = item_value(c, item);
    c.g[A0][0] = this;
    c.g[A1][0] = index;
    c.g[A2][0] = label;
    c.g[A3][0] = colour;
    call_ptr(c, 0x002d_02b0, ret)
}

/// `MenuSysPage_InsertItem(this, item, -1)` (0x2cbcc8): append the item.
fn add_item(c: &mut Cpu, item: Item, ret: u32) -> Flow {
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
        float_item(c, Item::Slot(6), 4, 0x116c, 0x47ef18, 20, Arg::Reg(21), None, 0x24c11c)?;
        // PercentageIceCrystalSpawnChance (item 7)
        store_item(c, 12);
        float_item(c, Item::Slot(7), 5, 0x1170, 0x47ef30, 20, Arg::Reg(21), Some((21, 0x41200000)), 0x24c14c)?;
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
        float_item(c, Item::Slot(11), 9, 0x1180, 0x47efa8, 20, Arg::Lit(0x44fa0000), None, 0x24c208)?;
        // LowestImpactIntensity (item 12)
        store_item(c, 20);
        float_item(c, Item::Slot(12), 10, 0x1184, 0x47efc8, 20, Arg::Lit(0x42f00000), None, 0x24c234)?;
        // MinimumSnowfallForImpacts (item 13)
        store_item(c, 21);
        store_item(c, 22);
        float_item(c, Item::Slot(13), 11, 0x1188, 0x47efe0, 20, Arg::Reg(21), None, 0x24c264)?;
        // ImpactMultiplier (item 14)
        store_item(c, 23);
        float_item(c, Item::Slot(14), 12, 0x118c, 0x47f000, 20, Arg::Reg(21), None, 0x24c28c)?;
        // LowestSnowfallAmount (item 15)
        store_item(c, 24);
        store_item(c, 25);
        float_item(c, Item::Slot(15), 13, 0x1190, 0x47f018, 20, Arg::Reg(21), None, 0x24c2bc)?;
        // SnowfallMultiplier (item 16)
        store_item(c, 26);
        store_item(c, 27);
        float_item(c, Item::Slot(16), 14, 0x1194, 0x47f030, 20, Arg::Reg(21), None, 0x24c2ec)?;
        // MinVelocityForPushoff (item 17)
        store_item(c, 28);
        store_item(c, 29);
        float_item(c, Item::Slot(17), 15, 0x1198, 0x47f048, 20, Arg::Reg(21), None, 0x24c31c)?;
        // DropMinLifeSpan (item 18)
        store_item(c, 30);
        store_item(c, 31);
        float_item(c, Item::Slot(18), 16, 0x119c, 0x47f068, 20, Arg::Reg(23), None, 0x24c34c)?;
        // DropMaxLifeSpan (item 19)
        store_item(c, 32);
        store_item(c, 33);
        float_item(c, Item::Slot(19), 17, 0x11a0, 0x47f080, 20, Arg::Reg(23), None, 0x24c37c)?;
        // DropMinSize (item 20)
        store_item(c, 34);
        float_item(c, Item::Slot(20), 18, 0x11a4, 0x47f098, 20, Arg::Reg(24), None, 0x24c3a4)?;
        // DropMaxSize (item 21)
        store_item(c, 35);
        store_item(c, 36);
        float_item(c, Item::Slot(21), 19, 0x11a8, 0x47f0a8, 20, Arg::Reg(24), None, 0x24c3d4)?;
        // DropMaxWidthHeightDifference (item 22)
        store_item(c, 37);
        store_item(c, 38);
        float_item(c, Item::Slot(22), 20, 0x11ac, 0x47f0b8, 20, Arg::Reg(21), None, 0x24c404)?;
        // DropMinVelocityJiggleX (item 23)
        store_item(c, 39);
        store_item(c, 40);
        float_item(c, Item::Slot(23), 21, 0x11b0, 0x47f0e0, 22, Arg::Reg(21), None, 0x24c434)?;
        // DropMaxVelocityJiggleX (item 24)
        store_item(c, 41);
        store_item(c, 42);
        float_item(c, Item::Slot(24), 22, 0x11b4, 0x47f100, 22, Arg::Reg(21), None, 0x24c464)?;
        // DropMinVelocityJiggleY (item 25)
        store_item(c, 43);
        store_item(c, 44);
        float_item(c, Item::Slot(25), 23, 0x11b8, 0x47f120, 22, Arg::Reg(21), None, 0x24c494)?;
        // DropMaxVelocityJiggleY (item 26)
        store_item(c, 45);
        store_item(c, 46);
        float_item(c, Item::Slot(26), 24, 0x11bc, 0x47f140, 22, Arg::Reg(21), None, 0x24c4c4)?;
        // DropMinSizeGrowth (item 27)
        store_item(c, 47);
        store_item(c, 48);
        float_item(c, Item::Slot(27), 25, 0x11c0, 0x47f160, 22, Arg::Reg(21), None, 0x24c4f4)?;
        // DropMaxSizeGrowth (item 28)
        store_item(c, 49);
        store_item(c, 50);
        float_item(c, Item::Slot(28), 26, 0x11c4, 0x47f178, 22, Arg::Reg(21), None, 0x24c524)?;
        // DropMinPushOffSpeed (item 29)
        float_item(c, Item::Slot(29), 27, 0x11c8, 0x47f190, 20, Arg::Reg(21), None, 0x24c544)?;
        // DropMaxPushOffSpeed (item 30)
        float_item(c, Item::Slot(30), 28, 0x11cc, 0x47f1a8, 20, Arg::Reg(21), None, 0x24c564)?;
        // DropMaxVelocity (item 31)
        float_item(c, Item::Slot(31), 29, 0x11d0, 0x47f1c0, 20, Arg::Reg(23), None, 0x24c584)?;
        // DropMinVelocityUsage (item 32)
        float_item(c, Item::Slot(32), 30, 0x11d4, 0x47f1d8, 20, Arg::Reg(21), None, 0x24c5a4)?;
        // DropMaxVelocityUsage (item 33)
        float_item(c, Item::Slot(33), 31, 0x11d8, 0x47f1f0, 20, Arg::Reg(21), None, 0x24c5c4)?;
        // DropMaxRotationChange (item 34)
        c.g[S1][0] = 0x47f208;
        float_item(c, Item::Slot(34), 32, 0x11dc, 0x47f208, 20, Arg::Reg(25), None, 0x24c5e8)?;
        // DropTintMin (item 35)
        argb_item(c, Item::Slot(35), 33, 0x47f230, 0x5046e0, 0x24c604)?;
        // DropTintMax (item 36)
        argb_item(c, Item::Slot(36), 34, 0x47f240, 0x5046f0, 0x24c620)?;
        // CrystalGroupSpacingX (item 37)
        float_item(c, Item::Slot(37), 35, 0x11e0, 0x47f250, 20, Arg::Reg(23), None, 0x24c640)?;
        // CrystalGroupSpacingY (item 38)
        float_item(c, Item::Slot(38), 36, 0x11e4, 0x47f268, 20, Arg::Reg(23), None, 0x24c660)?;
        // CrystalMinLifeSpan (item 39)
        float_item(c, Item::Slot(39), 37, 0x11e8, 0x47f280, 20, Arg::Reg(23), None, 0x24c680)?;
        // CrystalMaxLifeSpan (item 40)
        float_item(c, Item::Slot(40), 38, 0x11ec, 0x47f298, 20, Arg::Reg(23), None, 0x24c6a0)?;
        // CrystalMinSize (item 41)
        float_item(c, Item::Slot(41), 39, 0x11f0, 0x47f2b0, 20, Arg::Reg(24), None, 0x24c6c0)?;
        // CrystalMaxSize (item 42)
        float_item(c, Item::Slot(42), 40, 0x11f4, 0x47f2c8, 20, Arg::Reg(24), None, 0x24c6e0)?;
        // CrystalMaxWidthHeightDifference (item 43)
        float_item(c, Item::Slot(43), 41, 0x11f8, 0x47f2e0, 20, Arg::Reg(21), None, 0x24c700)?;
        // CrystalMinVelocityJiggleX (item 44)
        float_item(c, Item::Slot(44), 42, 0x11fc, 0x47f308, 22, Arg::Reg(21), None, 0x24c720)?;
        // CrystalMaxVelocityJiggleX (item 45)
        float_item(c, Item::Slot(45), 43, 0x1200, 0x47f328, 22, Arg::Reg(21), None, 0x24c740)?;
        // CrystalMinVelocityJiggleY (item 46)
        float_item(c, Item::Slot(46), 44, 0x1204, 0x47f348, 22, Arg::Reg(21), None, 0x24c760)?;
        // CrystalMaxVelocityJiggleY (item 47)
        float_item(c, Item::Slot(47), 45, 0x1208, 0x47f368, 22, Arg::Reg(21), None, 0x24c780)?;
        // CrystalMinJiggleTime (item 48)
        float_item(c, Item::Slot(48), 46, 0x120c, 0x47f388, 20, Arg::Reg(21), None, 0x24c7a0)?;
        // CrystalMaxJiggleTime (item 49)
        float_item(c, Item::Slot(49), 47, 0x1210, 0x47f3b8, 20, Arg::Reg(21), None, 0x24c7c0)?;
        // CrystalMinSizeGrowth (item 50)
        float_item(c, Item::Slot(50), 48, 0x1214, 0x47f3e8, 22, Arg::Reg(21), None, 0x24c7e0)?;
        // CrystalMaxSizeGrowth (item 51)
        float_item(c, Item::Reg(S4), 49, 0x1218, 0x47f400, 22, Arg::Reg(21), None, 0x24c800)?;
        // CrystalMinPushOffSpeed (item 52)
        float_item(c, Item::Reg(S5), 50, 0x121c, 0x47f418, 20, Arg::Reg(21), None, 0x24c820)?;
        // CrystalMaxPushOffSpeed (item 53)
        float_item(c, Item::Reg(S3), 51, 0x1220, 0x47f438, 20, Arg::Reg(21), None, 0x24c840)?;
        // CrystalMaxVelocity (item 54)
        float_item(c, Item::Reg(FP), 52, 0x1224, 0x47f458, 20, Arg::Reg(23), None, 0x24c860)?;
        // CrystalMaxRotationChange (item 55)
        float_item(c, Item::Reg(S2), 53, 0x1228, 0x47f208, 20, Arg::Reg(25), None, 0x24c87c)?;
        // CrystalTintMin (item 56)
        argb_item(c, Item::Reg(S7), 54, 0x47f478, 0x504700, 0x24c898)?;
        // CrystalTintMax (item 57)
        argb_item(c, Item::Reg(S6), 55, 0x47f490, 0x504710, 0x24c8b4)?;

        // Add every item to the page, title and space first.
        for i in 0..51 {
            add_item(c, Item::Slot(i), 0x0024_c8c4 + 0x10 * i as u32)?;
        }
        for (i, reg) in [(51u32, S4), (52, S5), (53, S3), (54, FP), (55, S2), (56, S7), (57, S6)] {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

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
}
