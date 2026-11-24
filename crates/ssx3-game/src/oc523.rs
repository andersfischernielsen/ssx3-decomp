//! Staging module oc523 (wave wave230): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc523`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0024_a228, screen_tint_menu_ctor, "ScreenTintMenu_ctor")];

/// The constructor's frame size.
const SCR_FRAME: u32 = 0x150;

/// The page's item objects, by frame slot: slot `k` (`sp + 4k`) holds
/// `this + SCR_ITEM_OFF[k]`. Slots 0..=33 are the 34 items that are built in
/// order; the eight gamma items live in s-registers (see the constructor).
const SCR_ITEM_OFF: [u32; 34] = [
    0x130, 0x14c, 0x164, 0x194, 0x1c4, 0x1f4, 0x228, 0x25c, 0x290, 0x2c4, 0x2f8, 0x32c, 0x360,
    0x394, 0x3c8, 0x3fc, 0x42c, 0x45c, 0x490, 0x4c0, 0x4f0, 0x520, 0x550, 0x584, 0x5b8, 0x5ec,
    0x620, 0x654, 0x688, 0x6bc, 0x6f0, 0x724, 0x758, 0x78c,
];

/// `this + off`, as the compiled code computes it from s0.
fn scr_field(c: &Cpu, off: u32) -> u64 {
    sx((c.g[S0][0] as u32).wrapping_add(off))
}

/// `gp + off` (a pointer into the globals).
fn scr_gp(c: &Cpu, off: u32) -> u64 {
    sx((c.g[GP][0] as u32).wrapping_add(off))
}

/// `this->item[k] = this + SCR_ITEM_OFF[k]`: the address goes to the frame
/// slot `sp + 4k` and stays in v0.
fn scr_store_slot(c: &mut Cpu, k: u32) {
    c.g[V0][0] = scr_field(c, SCR_ITEM_OFF[k as usize]);
    let v = c.g[V0][0];
    let a = sp(c).wrapping_add(4 * k);
    c.sw(a, v);
}

/// The item address in frame slot `k` (`lw sp + 4k`).
fn scr_slot(c: &mut Cpu, k: u32) -> u64 {
    let a = sp(c).wrapping_add(4 * k);
    c.lw(a)
}

/// `cIntMenuItem(item, index, &int at gp + gp_off, min, max, label)`
/// (0x2cd8f8): the item is `a0`, the range is `a3`..`t0`, the label `t1`.
fn scr_int(
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
fn scr_float(
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
fn scr_add(c: &mut Cpu, item: u64, ret: u32) -> Flow {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

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
}
