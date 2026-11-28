//! Staging module oc1532 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1532`.
//!
//! (0x002c1a00 `QuadTree_InsertRecursive` was already installed in
//! `game::worldpainter::quadtree`: skipped.)

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002c_a4c8, menu_sys_item_measure_row, "MenuSysItem_MeasureRow"),
    (0x002c_a768, menu_sys_item_draw_row_text, "MenuSysItem_DrawRowText"),
    (0x002c_ad58, menu_sys_page_draw, "MenuSysPage_Draw"),
    (0x002c_b498, menu_sys_page_draw_frame, "MenuSysPage_DrawFrame"),
];

/// `MenuSysItem_MeasureRow(out: *mut [f32;4], item, a2, a3, t0)` (0x2ca4c8): measure
/// one menu row made of up to three strings (`a2` left, `a3` middle, `t0` right) with the
/// text helper `0x391fb0`; writes the 16-byte rect to `*a0` and returns `a0` in `v0`.
#[allow(unused_assignments, unused_variables, unreachable_code, unused_mut)]
pub fn menu_sys_item_measure_row(c: &mut Cpu) {
    guest(c, |c| {
        'b_2ca6ec: {
        'b_2ca6e8: {
        'b_2ca6dc: {
        'b_2ca654: {
        'b_2ca64c: {
        c.cycles += 34;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
        c.mtc1(0, 0);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S0][0] = c.g[A1][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S1][0] = c.g[T0][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S2][0] = c.g[A3][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S3][0] = c.g[A2][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
        c.g[S4][0] = c.g[A0][0];
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x88));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[A1][0] = c.lw((c.g[S0][0] as u32));
        c.swc1(0, (c.g[SP][0] as u32));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x124));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
        c.lwc1(4, (c.g[V1][0] as u32).wrapping_add(0x34));
        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x14));
        c.cvt_s_w(0, 0);
        c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x3c));
        c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x38));
        c.div_s(2, 2, 4);
        c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0x30));
        c.mul_s(0, 0, 4);
        c.div_s(1, 1, 3);
        c.mul_s(0, 0, 2);
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x14));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[S3][0] == 0u64;
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
        if t {
        break 'b_2ca64c;
        } else {
        'b_2ca61c: {
        c.cycles += 2;
        let t = c.g[S2][0] == 0u64;
        
        if t {
        break 'b_2ca61c;
        } else {
        c.cycles += 2;
        let t = c.g[S1][0] == 0u64;
        c.g[A2][0] = 0u64;
        if t {
        break 'b_2ca61c;
        } else {
        'b_2ca5ec: {
        'b_2ca5e4: {
        c.cycles += 31;
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x124));
        c.g[A1][0] = c.g[S2][0];
        c.g[A3][0] = 0u64;
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
        c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
        c.g[RA][0] = sx(0x002ca57c);
        c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
        call_ptr(c, 0x00391fb0, 0x002ca57c)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32));
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
        c.mov_s(21, 0);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
        c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
        c.g[RA][0] = sx(0x002ca5a4);
        c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
        call_ptr(c, 0x00391fb0, 0x002ca5a4)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32));
        c.mov_s(20, 0);
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 0u64;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
        c.g[A3][0] = 0u64;
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
        c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
        c.g[RA][0] = sx(0x002ca5cc);
        c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
        call_ptr(c, 0x00391fb0, 0x002ca5cc)?;
        c.c_lt_s(0, 20);
        let t = !c.fcc();
        c.g[A1][0] = c.g[S3][0];
        if t {
        break 'b_2ca5e4;
        } else {
        c.cycles += 2;
        let t = true;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32));
        if t {
        break 'b_2ca5ec;
        } else {
        break 'b_2ca5e4;
        }
        }
        }
        c.cycles += 2;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32));
        c.g[A1][0] = c.g[S1][0];
        break 'b_2ca5ec;
        }
        c.cycles += 12;
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
        c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
        c.g[RA][0] = sx(0x002ca608);
        c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
        call_ptr(c, 0x00391fb0, 0x002ca608)?;
        c.add_s(0, 0, 0);
        c.g[AT][0] = 0x42000000;
        c.mtc1(1, 1);
        let t = true;
        c.add_s(0, 21, 0);
        if t {
        break 'b_2ca6dc;
        } else {
        break 'b_2ca61c;
        }
        }
        }
        }
        c.cycles += 2;
        let t = c.g[S3][0] == 0u64;
        c.g[A1][0] = c.g[S3][0];
        if t {
        break 'b_2ca64c;
        } else {
        c.cycles += 10;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32));
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
        c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
        c.g[RA][0] = sx(0x002ca644);
        c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
        call_ptr(c, 0x00391fb0, 0x002ca644)?;
        let t = true;
        c.mov_s(20, 0);
        if t {
        break 'b_2ca654;
        } else {
        break 'b_2ca64c;
        }
        }
        }
        }
        c.cycles += 2;
        c.mtc1(0, 0);
        c.mov_s(20, 0);
        break 'b_2ca654;
        }
        'b_2ca680: {
        c.cycles += 2;
        let t = c.g[S2][0] == 0u64;
        c.g[A1][0] = c.g[S2][0];
        if t {
        break 'b_2ca680;
        } else {
        c.cycles += 9;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32));
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
        c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
        c.g[RA][0] = sx(0x002ca67c);
        c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
        call_ptr(c, 0x00391fb0, 0x002ca67c)?;
        c.add_s(20, 20, 0);
        break 'b_2ca680;
        }
        }
        'b_2ca6ac: {
        c.cycles += 2;
        let t = c.g[S1][0] == 0u64;
        c.g[A1][0] = c.g[S1][0];
        if t {
        break 'b_2ca6ac;
        } else {
        c.cycles += 9;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32));
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
        c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
        c.g[RA][0] = sx(0x002ca6a8);
        c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
        call_ptr(c, 0x00391fb0, 0x002ca6a8)?;
        c.add_s(20, 20, 0);
        break 'b_2ca6ac;
        }
        }
        'b_2ca6d4: {
        'b_2ca6c4: {
        c.cycles += 2;
        let t = c.g[S3][0] == 0u64;
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x8));
        if t {
        break 'b_2ca6c4;
        } else {
        c.cycles += 2;
        let t = c.g[S2][0] != 0u64;
        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
        if t {
        break 'b_2ca6d4;
        } else {
        c.cycles += 2;
        let t = c.g[S1][0] != 0u64;
        
        if t {
        break 'b_2ca6d4;
        } else {
        break 'b_2ca6c4;
        }
        }
        }
        }
        c.cycles += 2;
        let t = c.g[S2][0] == 0u64;
        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
        if t {
        break 'b_2ca6e8;
        } else {
        c.cycles += 2;
        let t = c.g[S1][0] == 0u64;
        c.g[V0][0] = c.g[S4][0];
        if t {
        break 'b_2ca6ec;
        } else {
        break 'b_2ca6d4;
        }
        }
        }
        c.cycles += 2;
        c.g[AT][0] = 0x41800000;
        c.mtc1(1, 1);
        break 'b_2ca6dc;
        }
        c.cycles += 3;
        c.add_s(0, 0, 1);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
        break 'b_2ca6e8;
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[S4][0];
        break 'b_2ca6ec;
        }
        c.cycles += 30;
        c.g[AT][0] = 0x42c80000;
        c.mtc1(1, 2);
        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xc));
        c.g[AT][0] = 0x41400000;
        c.mtc1(1, 4);
        c.max_s(0, 0, 2);
        c.g[AT][0] = 0x42000000;
        c.mtc1(1, 3);
        c.add_s(1, 1, 4);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.add_s(0, 0, 3);
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xc));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.ldr(3, (c.g[SP][0] as u32));
        c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0xf));
        c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.sdl(3, (c.g[S4][0] as u32).wrapping_add(0x7));
        c.sdr(3, (c.g[S4][0] as u32));
        c.sdl(4, (c.g[S4][0] as u32).wrapping_add(0xf));
        c.sdr(4, (c.g[S4][0] as u32).wrapping_add(0x8));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x88));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x80));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t; return Ok(());
    })
}

/// `MenuSysItem_DrawRowText` (0x2ca768): draw the up to three strings of a menu row
/// with the text helpers `0x391cb0` / `0x391fb0`.
#[allow(unused_assignments, unused_variables, unreachable_code, unused_mut)]
pub fn menu_sys_item_draw_row_text(c: &mut Cpu) {
    guest(c, |c| {
        'b_2ca858: {
        c.cycles += 45;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S0][0] = c.g[T2][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S1][0] = c.g[A1][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S2][0] = c.g[A0][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S4][0] = c.g[A2][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
        c.g[A2][0] = c.g[T1][0];
        c.g[S3][0] = c.g[A3][0];
        c.g[S5][0] = c.g[T0][0];
        c.g[T3][0] = c.lw((c.g[S2][0] as u32));
        c.g[V1][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x12c));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x002ca7bc);
        c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x002ca7bc)?;
        c.g[A0][0] = c.lw((c.g[S2][0] as u32));
        c.g[AT][0] = 0x40000000;
        c.mtc1(1, 0);
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x124));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
        c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x7));
        c.ldr(2, (c.g[S0][0] as u32));
        c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0xf));
        c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x8));
        c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x47));
        c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0x40));
        c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0x4f));
        c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x48));
        c.g[V0][0] = c.lw((c.g[S2][0] as u32));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.swc1(0, (c.g[SP][0] as u32));
        c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.ldr(2, (c.g[SP][0] as u32));
        c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x2f));
        c.sdr(2, (c.g[A0][0] as u32).wrapping_add(0x28));
        let t = c.g[S4][0] == 0u64;
        c.g[A1][0] = c.g[S4][0];
        if t {
        break 'b_2ca858;
        } else {
        c.cycles += 15;
        c.g[AT][0] = 0x40c00000;
        c.mtc1(1, 0);
        c.lwc1(13, (c.g[S1][0] as u32).wrapping_add(0x4));
        c.g[V1][0] = c.lw((c.g[S2][0] as u32));
        c.add_s(13, 13, 0);
        c.lwc1(1, (c.g[S1][0] as u32));
        c.g[AT][0] = 0x41800000;
        c.mtc1(1, 12);
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
        c.add_s(12, 1, 12);
        c.sub_s(13, 13, 0);
        c.g[RA][0] = sx(0x002ca858);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
        call_ptr(c, 0x00391cb0, 0x002ca858)?;
        break 'b_2ca858;
        }
        }
        'b_2ca8cc: {
        c.cycles += 2;
        let t = c.g[S3][0] == 0u64;
        c.g[A1][0] = c.g[S3][0];
        if t {
        break 'b_2ca8cc;
        } else {
        c.cycles += 27;
        c.g[V1][0] = c.lw((c.g[S2][0] as u32));
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
        c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
        c.g[RA][0] = sx(0x002ca880);
        c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
        call_ptr(c, 0x00391fb0, 0x002ca880)?;
        c.lwc1(12, (c.g[S1][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = c.g[S3][0];
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 3);
        c.sub_s(12, 12, 0);
        c.lwc1(13, (c.g[S1][0] as u32).wrapping_add(0x4));
        c.g[AT][0] = 0x40c00000;
        c.mtc1(1, 1);
        c.g[V1][0] = c.lw((c.g[S2][0] as u32));
        c.mul_s(12, 12, 3);
        c.lwc1(2, (c.g[S1][0] as u32));
        c.add_s(13, 13, 1);
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
        c.add_s(12, 2, 12);
        c.sub_s(13, 13, 0);
        c.g[RA][0] = sx(0x002ca8cc);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
        call_ptr(c, 0x00391cb0, 0x002ca8cc)?;
        break 'b_2ca8cc;
        }
        }
        'b_2ca940: {
        c.cycles += 2;
        let t = c.g[S5][0] == 0u64;
        c.g[A1][0] = c.g[S5][0];
        if t {
        break 'b_2ca940;
        } else {
        c.cycles += 27;
        c.g[V1][0] = c.lw((c.g[S2][0] as u32));
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
        c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
        c.g[RA][0] = sx(0x002ca8f4);
        c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
        call_ptr(c, 0x00391fb0, 0x002ca8f4)?;
        c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = c.g[S5][0];
        c.lwc1(13, (c.g[S1][0] as u32).wrapping_add(0x4));
        c.sub_s(1, 1, 0);
        c.lwc1(12, (c.g[S1][0] as u32));
        c.g[AT][0] = 0x40c00000;
        c.mtc1(1, 2);
        c.g[V1][0] = c.lw((c.g[S2][0] as u32));
        c.add_s(12, 12, 1);
        c.g[AT][0] = 0x41800000;
        c.mtc1(1, 3);
        c.add_s(13, 13, 2);
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
        c.sub_s(12, 12, 3);
        c.sub_s(13, 13, 0);
        c.g[RA][0] = sx(0x002ca940);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
        call_ptr(c, 0x00391cb0, 0x002ca940)?;
        break 'b_2ca940;
        }
        }
        c.cycles += 18;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0u64);
        c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x17));
        c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x2f));
        c.sdr(2, (c.g[A0][0] as u32).wrapping_add(0x28));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t; return Ok(());
    })
}

/// `MenuSysPage_Draw` (0x2cad58): draw a menu page: the visible items through their
/// vtable draw slots, then the scroll arrows.
#[allow(unused_assignments, unused_variables, unreachable_code, unused_mut)]
pub fn menu_sys_page_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x4e0));
        c.g[V0][0] = 0x45;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x4a0));
        c.g[V1][0] = 0xffffffffffffffff;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x4c0));
        c.g[S2][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x4b0));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x490));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x480), c.g[RA][0]);
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4d0));
        'l_2cad80: loop {
        c.cycles += 7;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        let t = c.g[V0][0] != c.g[V1][0];
        c.g[S3][0] = 0u64;
        if t {
        if event_due(c) { deliver_event_here(c, 0x002cad80); } continue 'l_2cad80;
        } else {
        'b_2cae60: {
        c.cycles += 7;
        c.mtc1(0, 0);
        c.g[V0][0] = c.lw((c.g[S2][0] as u32));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
        let t = (c.g[V0][0] as i64) <= 0;
        c.swc1(0, (c.g[SP][0] as u32));
        if t {
        break 'b_2cae60;
        } else {
        c.cycles += 4;
        c.mov_s(20, 0);
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x18));
        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
        'l_2cadc8: loop {
        'b_2cae4c: {
        'b_2cae48: {
        'b_2cae40: {
        c.cycles += 4;
        c.g[RA][0] = sx(0x002cadd0);
        c.g[A0][0] = c.lw((c.g[S1][0] as u32));
        Callee { addr: 0x002ca378, f: ssx3_ee::s09::f_002ca378 }.call(c, 0x002cadd0)?;
        let t = c.g[V0][0] == 0u64;
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x470));
        if t {
        break 'b_2cae40;
        } else {
        c.cycles += 24;
        c.g[A2][0] = c.lw((c.g[S1][0] as u32));
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x002cadf0);
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        call_ptr(c, t, 0x002cadf0)?;
        c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x477));
        c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x470));
        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x47f));
        c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x478));
        c.sdl(2, (c.g[S0][0] as u32).wrapping_sub(0x1));
        c.sdr(2, (c.g[S0][0] as u32).wrapping_sub(0x8));
        c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x7));
        c.sdr(3, (c.g[S0][0] as u32));
        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x4));
        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.add_s(0, 0, 1);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
        c.lwc1(0, (c.g[S0][0] as u32));
        c.c_lt_s(2, 0);
        if c.fcc() {
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
        break 'b_2cae48;
        } else {
        c.cycles += 2;
        let t = true;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32));
        if t {
        break 'b_2cae4c;
        } else {
        break 'b_2cae40;
        }
        }
        }
        }
        c.cycles += 2;
        c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x4));
        c.swc1(20, (c.g[S0][0] as u32));
        break 'b_2cae48;
        }
        c.cycles += 1;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32));
        break 'b_2cae4c;
        }
        c.cycles += 5;
        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let t = c.g[V0][0] != 0u64;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
        if t {
        if event_due(c) { deliver_event_here(c, 0x002cadc8); } continue 'l_2cadc8;
        } else {
        break 'b_2cae60;
        }
        }
        }
        }
        'b_2cafc8: {
        'b_2cafa8: {
        c.cycles += 19;
        c.g[V1][0] = 0x1e0;
        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.g[AT][0] = 0x44200000;
        c.mtc1(1, 0);
        c.mtc1(3, 1);
        c.cvt_s_w(1, 1);
        c.g[AT][0] = 0x42d40000;
        c.mtc1(1, 3);
        c.sub_s(0, 0, 2);
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 4);
        c.sub_s(1, 1, 3);
        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x124));
        c.mul_s(0, 0, 4);
        c.c_lt_s(1, 2);
        let t = !c.fcc();
        c.swc1(0, (c.g[SP][0] as u32));
        if t {
        break 'b_2cafa8;
        } else {
        'b_2caec0: {
        c.cycles += 5;
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] != 0u64 {
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        break 'b_2caec0;
        } else {
        break 'b_2caec0;
        }
        }
        'b_2caf04: {
        c.cycles += 7;
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
        c.g[A2][0] = c.g[V1][0];
        c.mtc1(0, 1);
        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let t = c.g[V0][0] != 0u64;
        c.g[T0][0] = c.g[A0][0];
        if t {
        break 'b_2caf04;
        } else {
        c.cycles += 3;
        c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = sx((c.g[A2][0] as u32) << 4);
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        'l_2caee8: loop {
        c.cycles += 7;
        c.lwc1(0, (c.g[A1][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let t = c.g[V0][0] == 0u64;
        c.add_s(1, 1, 0);
        if t {
        if event_due(c) { deliver_event_here(c, 0x002caee8); } continue 'l_2caee8;
        } else {
        break 'b_2caf04;
        }
        }
        }
        }
        'b_2caf98: {
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[T0][0] as i64)) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0x1e0;
        if t {
        break 'b_2caf98;
        } else {
        c.cycles += 12;
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x124));
        c.g[AT][0] = 0x42d40000;
        c.mtc1(1, 2);
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x64));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.sub_s(0, 0, 2);
        c.c_lt_s(0, 1);
        let t = !c.fcc();
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1c));
        if t {
        break 'b_2caf98;
        } else {
        c.cycles += 4;
        c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
        c.g[A3][0] = 0x1e0;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
        'l_2caf50: loop {
        c.cycles += 8;
        c.g[V1][0] = sx((c.g[V0][0] as u32) << 4);
        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.lwc1(0, (c.g[V1][0] as u32));
        c.g[A0][0] = ((c.g[V0][0] as i64) < (c.g[A2][0] as i64)) as u64;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        let t = c.g[A0][0] == 0u64;
        c.sub_s(1, 1, 0);
        if t {
        break 'b_2caf98;
        } else {
        c.cycles += 10;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x124));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64));
        c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
        c.mtc1(3, 0);
        c.cvt_s_w(0, 0);
        c.sub_s(0, 0, 2);
        c.c_lt_s(0, 1);
        if c.fcc() {
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
        if event_due(c) { deliver_event_here(c, 0x002caf50); } continue 'l_2caf50;
        } else {
        break 'b_2caf98;
        }
        }
        }
        }
        }
        }
        c.cycles += 4;
        c.g[AT][0] = 0x42540000;
        c.mtc1(1, 0);
        let t = true;
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
        if t {
        break 'b_2cafc8;
        } else {
        break 'b_2cafa8;
        }
        }
        }
        c.cycles += 8;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64));
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.sub_s(0, 0, 2);
        c.mul_s(0, 0, 4);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
        break 'b_2cafc8;
        }
        'b_2cb0d0: {
        'b_2cb0cc: {
        c.cycles += 7;
        c.g[RA][0] = sx(0x002cafd0);
        c.g[A0][0] = c.g[S2][0];
        Callee { addr: 0x002cb180, f: ssx3_ee::s09::f_002cb180 }.call(c, 0x002cafd0)?;
        c.g[S3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[S2][0] as u32));
        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
        if t {
        break 'b_2cb0cc;
        } else {
        c.cycles += 3;
        c.g[S0][0] = sx((c.g[S3][0] as u32) << 4);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc));
        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
        'l_2caff0: loop {
        'b_2cb0b8: {
        c.cycles += 4;
        c.g[RA][0] = sx(0x002caff8);
        c.g[A0][0] = c.lw((c.g[S1][0] as u32));
        Callee { addr: 0x002ca378, f: ssx3_ee::s09::f_002ca378 }.call(c, 0x002caff8)?;
        if c.g[V0][0] == 0u64 {
        c.g[V0][0] = c.lw((c.g[S2][0] as u32));
        break 'b_2cb0b8;
        } else {
        'b_2cb050: {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
        let t = c.g[S3][0] == c.g[V0][0];
        c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1c));
        if t {
        break 'b_2cb050;
        } else {
        c.cycles += 17;
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x124));
        c.g[V0][0] = 0x1e0;
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[S0][0] as u32)));
        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x64));
        c.lwc1(3, (c.g[A1][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
        c.g[AT][0] = 0x42540000;
        c.mtc1(1, 2);
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.add_s(1, 1, 3);
        c.sub_s(0, 0, 2);
        c.c_lt_s(0, 1);
        if c.fcc() {
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
        break 'b_2cb0d0;
        } else {
        break 'b_2cb050;
        }
        }
        }
        c.cycles += 26;
        c.lwc1(0, (c.g[SP][0] as u32));
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x124));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
        c.swc1(0, (c.g[A1][0] as u32));
        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add((c.g[S0][0] as u32)));
        c.g[A0][0] = c.g[V1][0];
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[S0][0] as u32)));
        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x64));
        c.cvt_s_w(1, 1);
        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[A2][0] = c.lw((c.g[S1][0] as u32));
        c.add_s(1, 1, 0);
        c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x14));
        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x18));
        c.lwc1(1, (c.g[A3][0] as u32));
        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.add_s(0, 0, 1);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x002cb0b4);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x002cb0b4)?;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32));
        break 'b_2cb0b8;
        }
        }
        c.cycles += 5;
        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let t = c.g[V0][0] != 0u64;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
        if t {
        if event_due(c) { deliver_event_here(c, 0x002caff0); } continue 'l_2caff0;
        } else {
        break 'b_2cb0cc;
        }
        }
        }
        }
        c.cycles += 1;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
        break 'b_2cb0d0;
        }
        'b_2cb11c: {
        c.cycles += 2;
        let t = c.g[V0][0] == 0u64;
        c.g[A0][0] = c.g[S2][0];
        if t {
        break 'b_2cb11c;
        } else {
        c.cycles += 17;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x124));
        c.g[AT][0] = 0x42000000;
        c.mtc1(1, 4);
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x470));
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x64));
        c.cvt_s_w(0, 0);
        c.g[AT][0] = 0x41b00000;
        c.mtc1(1, 3);
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4000));
        c.g[AT][0] = 0x41700000;
        c.mtc1(1, 2);
        c.add_s(0, 0, 4);
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x470));
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x478));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x47c));
        c.g[RA][0] = sx(0x002cb11c);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x474));
        Callee { addr: 0x002cb8c8, f: ssx3_ee::s09::f_002cb8c8 }.call(c, 0x002cb11c)?;
        break 'b_2cb11c;
        }
        }
        'b_2cb158: {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32));
        let t = c.g[S3][0] == c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        if t {
        break 'b_2cb158;
        } else {
        c.cycles += 12;
        c.g[AT][0] = 0x41b00000;
        c.mtc1(1, 3);
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3ffc));
        c.g[AT][0] = 0x41700000;
        c.mtc1(1, 2);
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3ff8));
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x470));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x470));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x474));
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x478));
        c.g[RA][0] = sx(0x002cb158);
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x47c));
        Callee { addr: 0x002cbac8, f: ssx3_ee::s09::f_002cbac8 }.call(c, 0x002cb158)?;
        break 'b_2cb158;
        }
        }
        c.cycles += 10;
        c.g[RA][0] = sx(0x002cb160);
        c.g[A0][0] = c.g[S2][0];
        Callee { addr: 0x002cb2f8, f: ssx3_ee::s09::f_002cb2f8 }.call(c, 0x002cb160)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x4c0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x4b0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x4a0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x490));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x480));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x4d0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4e0));
        c.pc = t; return Ok(());
        }
        }
    })
}

/// `MenuSysPage_DrawFrame` (0x2cb498): build the quads of a menu page frame
/// (`MenuSys_BuildQuad`).
#[allow(unused_assignments, unused_variables, unreachable_code, unused_mut)]
pub fn menu_sys_page_draw_frame(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x1a0));
        c.g[T4][0] = c.g[A0][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[V0][0] = 0x3;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.g[S3][0] = c.g[A1][0];
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.g[V1][0] = 0xffffffffffffffff;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.sd((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[RA][0]);
        c.swc1(29, (c.g[SP][0] as u32).wrapping_add(0x198));
        c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0x190));
        c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0x188));
        c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0x180));
        c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x178));
        c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x170));
        c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x168));
        c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x158));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x150));
        'l_2cb4f8: loop {
        c.cycles += 7;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        let t = c.g[V0][0] != c.g[V1][0];
        c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        if t {
        if event_due(c) { deliver_event_here(c, 0x002cb4f8); } continue 'l_2cb4f8;
        } else {
        c.cycles += 23;
        c.g[AT][0] = 0x43000000;
        c.mtc1(1, 1);
        c.g[V1][0] = c.g[SP][0];
        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
        c.lwc1(4, (c.g[A2][0] as u32).wrapping_add(0xc));
        c.lwc1(2, (c.g[A2][0] as u32));
        c.mul_s(0, 0, 1);
        c.lwc1(3, (c.g[A2][0] as u32).wrapping_add(0x4));
        c.mul_s(4, 4, 1);
        c.mul_s(2, 2, 1);
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 5);
        c.mul_s(3, 3, 1);
        c.cvt_w_s(1, 0);
        c.mfc1(6, 1);
        c.cvt_w_s(1, 2);
        c.mfc1(7, 1);
        c.cvt_w_s(1, 3);
        c.mfc1(5, 1);
        c.cvt_w_s(0, 4);
        c.mfc1(4, 0);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[A3][0]);
        'l_2cb570: loop {
        c.cycles += 8;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A1][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[A2][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[A0][0]);
        c.swc1(5, (c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[T0][0] as i64)) as u64;
        if c.g[V0][0] != 0u64 {
        c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[A3][0]);
        if event_due(c) { deliver_event_here(c, 0x002cb570); } continue 'l_2cb570;
        } else {
        c.cycles += 187;
        c.g[T1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[S1][0] = 0xffffffffffffffff;
        c.g[S2][0] = 0xfffffffffffffc1f;
        c.g[T3][0] = 0xffffffffffffff83;
        c.g[T2][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0xe84));
        c.g[A0][0] = c.g[SP][0];
        c.g[AT][0] = 0x40a00000;
        c.mtc1(1, 20);
        c.g[A1][0] = c.g[S4][0];
        c.g[A3][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x8));
        c.g[A2][0] = c.g[S5][0];
        c.g[AT][0] = 0x3e800000;
        c.mtc1(1, 21);
        c.g[S0][0] = c.g[A3][0] & 0x3e0;
        c.g[AT][0] = 0x41200000;
        c.mtc1(1, 2);
        c.g[S0][0] = sx((c.g[S0][0] as u32) >> 5);
        c.g[A3][0] = c.g[A3][0] & c.g[S2][0];
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x2));
        c.g[AT][0] = 0x3f400000;
        c.mtc1(1, 26);
        c.g[V1][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 27);
        if c.g[V1][0] == 0 { c.g[V0][0] = 0u64; }
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
        c.g[V0][0] = c.g[V0][0] & 0x3e0;
        c.g[A3][0] = c.g[A3][0] | c.g[V0][0];
        c.sw((c.g[T2][0] as u32).wrapping_add(0x8), c.g[A3][0]);
        c.g[T0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0xe84));
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.g[V0][0] & c.g[T3][0];
        c.g[V0][0] = c.g[V0][0] | 0x14;
        c.sw((c.g[T0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0x124));
        c.g[A3][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0xe84));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x60));
        c.sh((c.g[A3][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc8));
        c.lwc1(1, (c.g[S3][0] as u32).wrapping_add(0x4));
        c.lwc1(0, (c.g[S3][0] as u32));
        c.lwc1(24, (c.g[S3][0] as u32).wrapping_add(0x8));
        c.add_s(29, 1, 20);
        c.lwc1(25, (c.g[S3][0] as u32).wrapping_add(0xc));
        c.add_s(28, 0, 20);
        c.add_s(23, 0, 24);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.add_s(22, 1, 25);
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xcc));
        c.sub_s(25, 25, 2);
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xd8));
        c.sub_s(24, 24, 2);
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xdc));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xc4));
        c.sub_s(23, 23, 20);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xd0), 0u64);
        c.sub_s(22, 22, 20);
        c.g[RA][0] = sx(0x002cb67c);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xd4), 0u64);
        Callee { addr: 0x002cb350, f: ssx3_ee::s09::f_002cb350 }.call(c, 0x002cb67c)?;
        c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = c.g[SP][0];
        c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.g[A1][0] = c.g[S4][0];
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc4));
        c.g[A2][0] = c.g[S5][0];
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc8));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xcc));
        c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xd8));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xdc));
        c.g[RA][0] = sx(0x002cb6b0);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xd4), 0u64);
        Callee { addr: 0x002cb350, f: ssx3_ee::s09::f_002cb350 }.call(c, 0x002cb6b0)?;
        c.g[A0][0] = c.g[SP][0];
        c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xc4));
        c.g[A1][0] = c.g[S4][0];
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc8));
        c.g[A2][0] = c.g[S5][0];
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xcc));
        c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0xd4));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xd8));
        c.g[RA][0] = sx(0x002cb6e0);
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xdc));
        Callee { addr: 0x002cb350, f: ssx3_ee::s09::f_002cb350 }.call(c, 0x002cb6e0)?;
        c.lwc1(0, (c.g[S3][0] as u32));
        c.g[A0][0] = c.g[SP][0];
        c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xc4));
        c.g[A1][0] = c.g[S4][0];
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.g[A2][0] = c.g[S5][0];
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc8));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xcc));
        c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0xd4));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xd8));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xdc));
        c.g[RA][0] = sx(0x002cb714);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xd0), 0u64);
        Callee { addr: 0x002cb350, f: ssx3_ee::s09::f_002cb350 }.call(c, 0x002cb714)?;
        c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = c.g[SP][0];
        c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.g[A1][0] = c.g[S4][0];
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc4));
        c.g[A2][0] = c.g[S5][0];
        c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0xc8));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xcc));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0xd8));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xdc));
        c.g[RA][0] = sx(0x002cb748);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xd4), 0u64);
        Callee { addr: 0x002cb350, f: ssx3_ee::s09::f_002cb350 }.call(c, 0x002cb748)?;
        c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xc4));
        c.g[A0][0] = c.g[SP][0];
        c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.g[A1][0] = c.g[S4][0];
        c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0xc8));
        c.g[A2][0] = c.g[S5][0];
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xcc));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0xd4));
        c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0xd8));
        c.g[RA][0] = sx(0x002cb778);
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xdc));
        Callee { addr: 0x002cb350, f: ssx3_ee::s09::f_002cb350 }.call(c, 0x002cb778)?;
        c.lwc1(0, (c.g[S3][0] as u32));
        c.g[A0][0] = c.g[SP][0];
        c.swc1(29, (c.g[SP][0] as u32).wrapping_add(0xc4));
        c.g[A1][0] = c.g[S4][0];
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc8));
        c.g[A2][0] = c.g[S5][0];
        c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0xcc));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xd4));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xd8));
        c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0xdc));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.g[RA][0] = sx(0x002cb7ac);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xd0), 0u64);
        Callee { addr: 0x002cb350, f: ssx3_ee::s09::f_002cb350 }.call(c, 0x002cb7ac)?;
        c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.g[A0][0] = c.g[SP][0];
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc8));
        c.g[A1][0] = c.g[S4][0];
        c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[A2][0] = c.g[S5][0];
        c.swc1(29, (c.g[SP][0] as u32).wrapping_add(0xc4));
        c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0xcc));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xd4));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xd8));
        c.g[RA][0] = sx(0x002cb7dc);
        c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0xdc));
        Callee { addr: 0x002cb350, f: ssx3_ee::s09::f_002cb350 }.call(c, 0x002cb7dc)?;
        c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.g[A1][0] = c.g[S4][0];
        c.swc1(29, (c.g[SP][0] as u32).wrapping_add(0xc4));
        c.g[A2][0] = c.g[S5][0];
        c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0xc8));
        c.g[A0][0] = c.g[SP][0];
        c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0xcc));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xd4));
        c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0xdc));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[RA][0] = sx(0x002cb80c);
        c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0xd8));
        Callee { addr: 0x002cb350, f: ssx3_ee::s09::f_002cb350 }.call(c, 0x002cb80c)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[S1][0] = ((c.g[S1][0] as i64) < (c.g[S0][0] as i64)) as u64;
        if c.g[S1][0] == 0 { c.g[S0][0] = 0u64; }
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xe84));
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 5);
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.g[V0][0] = c.g[V0][0] & c.g[S2][0];
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[V0][0] = c.g[V0][0] | c.g[S0][0];
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lwc1(29, (c.g[SP][0] as u32).wrapping_add(0x198));
        c.lwc1(28, (c.g[SP][0] as u32).wrapping_add(0x190));
        c.lwc1(27, (c.g[SP][0] as u32).wrapping_add(0x188));
        c.lwc1(26, (c.g[SP][0] as u32).wrapping_add(0x180));
        c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x178));
        c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x170));
        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x168));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x158));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x150));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1a0));
        c.pc = t; return Ok(());
        }
        }
        }
        }
    })
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, check_reaching, machine, vtable};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of this module's text-helper fakes: the magic word at `OC1532_CTRL`.
    /// The helpers `0x391cb0` / `0x391fb0` are real functions, so without the
    /// fakes their calls would run the generated body and never be logged.
    const OC1532_CTRL: u32 = DATA + 0xf_f000;
    const OC1532_MAGIC: u32 = 0x1532_c0de;
    static OC1532_PREV_391CB0: AtomicUsize = AtomicUsize::new(0);
    static OC1532_PREV_391FB0: AtomicUsize = AtomicUsize::new(0);

    fn oc1532_fake_391cb0(c: &mut Cpu) {
        if c.bus.read32(OC1532_CTRL) != OC1532_MAGIC {
            return fakes::fall_back(c, &OC1532_PREV_391CB0, ee::s12::f_00391cb0__generated);
        }
        fakes::fake_call(c, 0x0039_1cb0)
    }

    fn oc1532_fake_391fb0(c: &mut Cpu) {
        if c.bus.read32(OC1532_CTRL) != OC1532_MAGIC {
            return fakes::fall_back(c, &OC1532_PREV_391FB0, ee::s12::f_00391fb0__generated);
        }
        fakes::fake_call(c, 0x0039_1fb0)
    }

    fn oc1532_install_fakes(c: &mut Cpu) {
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00391CB0),
            oc1532_fake_391cb0,
            &OC1532_PREV_391CB0,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00391FB0),
            oc1532_fake_391fb0,
            &OC1532_PREV_391FB0,
        );
        c.bus.write32(OC1532_CTRL, OC1532_MAGIC);
    }

    /// Base of the fixture structures (clear of the fakes' random pointers
    /// and of their call log).
    const OC1532_BASE: u32 = DATA + 0x20_0000;

    fn oc1532_w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// Menu -> vtable -> font chain every row function walks:
    /// `item -> menu; menu+0x124 -> vt; vt+0x58 -> font`.
    fn oc1532_font_chain(c: &mut Cpu, r: &mut Rng, item: u32) -> (u32, u32) {
        let menu = OC1532_BASE + 0x300;
        let vt = OC1532_BASE + 0x400;
        let font = OC1532_BASE + 0x500;
        oc1532_w(c, item, menu);
        oc1532_w(c, menu + 0x124, vt);
        oc1532_w(c, vt + 0x58, font);
        oc1532_w(c, font + 0x14, 1 + r.next_u32() % 5);
        oc1532_w(c, font + 0x30, 0x3f80_0000);
        oc1532_w(c, font + 0x34, 0x4000_0000);
        oc1532_w(c, font + 0x38, r.f32(40.0));
        oc1532_w(c, font + 0x3c, r.f32(40.0));
        (menu, font)
    }

    /// Row measure: the three optional strings (`seed % 8` picks which exist)
    /// each go through the text helper; the fake helper's random float
    /// results decide the max and the layout.
    #[test]
    fn menu_sys_item_measure_row_matches() {
        check_reaching(
            ee::s09::f_002ca4c8__generated,
            menu_sys_item_measure_row,
            240,
            &[0x002c_a57c, 0x002c_a5a4, 0x002c_a5cc, 0x002c_a608, 0x002c_a644, 0x002c_a67c, 0x002c_a6a8],
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ca4c8, 8);
                oc1532_install_fakes(c);
                let item = OC1532_BASE + 0x200;
                oc1532_font_chain(c, &mut r, item);
                let s = OC1532_BASE + 0x600;
                let pick = |bit: u32| if (seed >> bit) & 1 != 0 { s } else { 0 };
                c.g[A0][0] = sx(OC1532_BASE + 0x100);
                c.g[A1][0] = sx(item);
                c.g[A2][0] = sx(pick(0));
                c.g[A3][0] = sx(pick(1));
                c.g[T0][0] = sx(pick(2));
            },
        );
    }

    /// Row text draw: sets the font colour/scale, then draws the left,
    /// centre and right strings (`seed % 8`) through the helpers.
    #[test]
    fn menu_sys_item_draw_row_text_matches() {
        check_reaching(
            ee::s09::f_002ca768__generated,
            menu_sys_item_draw_row_text,
            240,
            &[0x002c_a7bc, 0x002c_a858, 0x002c_a880, 0x002c_a8cc, 0x002c_a8f4, 0x002c_a940],
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ca768, 8);
                oc1532_install_fakes(c);
                let item = OC1532_BASE + 0x200;
                let (menu, _font) = oc1532_font_chain(c, &mut r, item);
                let vt = vtable(c, &mut r, &[0x40]);
                oc1532_w(c, menu + 0x12c, vt);
                let rect = OC1532_BASE + 0x600;
                let color = OC1532_BASE + 0x680;
                for i in 0..4 {
                    oc1532_w(c, rect + 4 * i, r.f32(300.0));
                    oc1532_w(c, color + 4 * i, r.next_u32());
                }
                let s = OC1532_BASE + 0x700;
                let pick = |bit: u32| if (seed >> bit) & 1 != 0 { s } else { 0 };
                c.g[A0][0] = sx(item);
                c.g[A1][0] = sx(rect);
                c.g[A2][0] = sx(pick(0));
                c.g[A3][0] = sx(pick(1));
                c.g[T0][0] = sx(pick(2));
                c.g[T2][0] = sx(color);
            },
        );
    }

    /// Page draw: lay out the visible items of a page (`count` items at
    /// `+0xc`, scroll index at `+8`), draw them through their vtable and add
    /// the scroll arrows.
    #[test]
    fn menu_sys_page_draw_matches() {
        check_reaching(
            ee::s09::f_002cad58__generated,
            menu_sys_page_draw,
            300,
            &[0x002c_adf0, 0x002c_b0b4],
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cad58, 6);
                let page = OC1532_BASE + 0x200;
                let menu = OC1532_BASE + 0x300;
                let count = seed % 5;
                oc1532_w(c, page, count);
                let first = if count > 0 { (seed / 5) % count } else { 0 };
                oc1532_w(c, page + 4, first);
                oc1532_w(c, page + 8, if count > 0 { (seed / 25) % (count + 1) } else { 0 });
                oc1532_w(c, page + 0x124, menu);
                oc1532_w(c, menu + 0x64, (seed * 37) % 500);
                oc1532_w(c, menu + 0x60, (seed * 11) % 50);
                let ivt = vtable(c, &mut r, &[0x20, 0x28]);
                for i in 0..4u32 {
                    let item = OC1532_BASE + 0x1000 + 0x100 * i;
                    oc1532_w(c, page + 0xc + 4 * i, item);
                    // Item +4 bit 0 is its visible flag.
                    oc1532_w(c, item + 4, ((seed >> (i + 3)) ^ i) & 1);
                    oc1532_w(c, item + 0x10, ivt);
                }
                // The measure slot is a fake, so the item rect it "returns"
                // is whatever the stack holds: fill it with floats.
                let sp0 = c.g[SP][0] as u32;
                for i in 0..4u32 {
                    oc1532_w(c, sp0.wrapping_sub(0x70) + 4 * i, r.f32(200.0));
                }
                c.g[A0][0] = sx(page);
            },
        );
    }

    /// Frame draw: nine `MenuSys_BuildQuad` pieces (corners, edges, centre)
    /// of a menu box from a rect and a colour.
    #[test]
    fn menu_sys_page_draw_frame_matches() {
        check(
            ee::s09::f_002cb498__generated,
            menu_sys_page_draw_frame,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cb498, 0);
                let page = OC1532_BASE + 0x200;
                let menu = OC1532_BASE + 0x300;
                let grman = OC1532_BASE + 0x400;
                let state = OC1532_BASE + 0x500;
                let slot = OC1532_BASE + 0x6000;
                let rect = OC1532_BASE + 0x700;
                let color = OC1532_BASE + 0x740;
                oc1532_w(c, page + 0x124, menu);
                oc1532_w(c, menu + 0x60, r.next_u32() % 64);
                oc1532_w(c, GAME_GP.wrapping_sub(0x854), grman);
                oc1532_w(c, grman + 0xe84, state);
                oc1532_w(c, state + 0x4, r.next_u32());
                oc1532_w(c, state + 0x8, r.next_u32());
                // BuildQuad's submit slot: *(grman+0x10d8)+0x23c, a fake target.
                oc1532_w(c, grman + 0x10d8, slot);
                oc1532_w(c, slot + 0x238, 0x100);
                oc1532_w(c, slot + 0x23c, 0x0012_3450);
                for i in 0..4 {
                    oc1532_w(c, rect + 4 * i, r.f32(300.0));
                    oc1532_w(c, color + 4 * i, r.f32(if seed % 3 == 0 { 3.0 } else { 0.9 }));
                }
                c.g[A0][0] = sx(page);
                c.g[A1][0] = sx(rect);
                c.g[A2][0] = sx(color);
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }
}
