//! Staging module oc524 (wave wave230): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc524`.

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
    &[(0x0024_b0e8, board_trail_menu_ctor, "BoardTrailMenu_ctor")];

/// Offsets of the menu's vtable and items in the `cBoardTrailMenu` object.
const BTM_VTBL: u32 = 0x12c;
const BTM_TITLE: u32 = 0x130;
const BTM_SPACE: u32 = 0x14c;
const BTM_ENABLE: u32 = 0x164;
const BTM_INNER_WIDTH: u32 = 0x194;
const BTM_OUTER_WIDTH: u32 = 0x1c8;
const BTM_HEIGHT: u32 = 0x1fc;
const BTM_DEPTH_TO_HEIGHT: u32 = 0x230;
const BTM_PACKED_DEPTH: u32 = 0x264;
const BTM_LOOSE_DEPTH: u32 = 0x298;
const BTM_POWDER_DEPTH: u32 = 0x2cc;
const BTM_DEEP_POWDER_DEPTH: u32 = 0x300;
const BTM_TOP_BIAS_FRONT: u32 = 0x334;
const BTM_TOP_BIAS_BEHIND: u32 = 0x368;
const BTM_TOP_BIAS_CLEAR: u32 = 0x39c;
const BTM_INNER_JITTER: u32 = 0x3d0;
const BTM_BASE_U: u32 = 0x404;
const BTM_INNER_U: u32 = 0x438;
const BTM_OUTER_U: u32 = 0x46c;
const BTM_OUTER_NORMAL: u32 = 0x4a0;
const BTM_INNER_NORMAL: u32 = 0x4d4;
const BTM_BASE_NORMAL: u32 = 0x508;
const BTM_FADE_SEGMENTS: u32 = 0x53c;
const BTM_DRAW_FIXUP: u32 = 0x56c;
const BTM_SLICE_CAPS: u32 = 0x59c;

/// Where the generated body reads an item address from when it adds the
/// items to the page: a frame slot, or a callee-saved register.
#[derive(Clone, Copy)]
enum BtmSrc {
    Slot(u32),
    Reg(usize),
}

/// `this + off` as the 64-bit value the generated code keeps in a register.
fn btm_at(this: u64, off: u32) -> u64 {
    sx((this as u32).wrapping_add(off))
}

/// `V0 = this + off; sw V0, sp + slot`: an item address kept in the frame.
fn btm_stash(c: &mut Cpu, frame: Frame, slot: u32, off: u32) {
    c.g[V0][0] = btm_at(c.g[S0][0], off);
    let addr = frame.at(c, slot);
    let v = c.g[V0][0];
    c.sw(addr, v);
}

/// The item address a call reads from `src`.
fn btm_value(c: &mut Cpu, frame: Frame, src: BtmSrc) -> u64 {
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
fn btm_int(
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
fn btm_float(c: &mut Cpu, item: u64, index: u32, var: u32, name: u64, ret_addr: u32) -> Flow {
    c.g[A0][0] = item;
    c.g[A3][0] = name;
    c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(var));
    c.g[A1][0] = index as u64;
    call_ptr(c, 0x002c_dce8, ret_addr)
}

/// `MenuSysPage_InsertItem(this, item, -1)` (0x2cbcc8).
fn btm_add(c: &mut Cpu, item: u64, ret_addr: u32) -> Flow {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, DATA};
    use ssx3_ee as ee;

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
}
