//! Staging module oc1558 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1558`.

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
    (0x002c_d8a8, menu_sys_int_item_ctor_2, "MenuSysIntItem_ctor_2"),
    (0x002c_dc88, menu_sys_float_item_ctor, "MenuSysFloatItem_ctor"),
    (0x002c_dfd8, menu_sys_format_float, "MenuSys_FormatFloat"),
    (0x002c_e170, menu_sys_float_range_item_ctor, "MenuSysFloatRangeItem_ctor"),
    (
        0x002c_e1f0,
        menu_sys_float_range_item_handle_input,
        "MenuSysFloatRangeItem_HandleInput",
    ),
    (
        0x002c_e820,
        menu_sys_text_edit_item_measure,
        "MenuSysTextEditItem_Measure",
    ),
    (
        0x002c_ed78,
        menu_sys_color_channel_item_handle_input,
        "MenuSysColorChannelItem_HandleInput",
    ),
    (
        0x002c_f788,
        menu_sys_group_item_page_forward_v03,
        "MenuSysGroupItem_PageForwardV03",
    ),
];

/// `MenuSysIntItem* MenuSysIntItem_ctor_2(item, a1)` (0x002cd8a8): the base menu
/// item, then the int item's class vtable and its zeroed value fields.
pub fn menu_sys_int_item_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x002c_a258, 0x002c_d8bc)?; // MenuSysItem_ctor
        c.g[V1][0] = sx(0x0048_6bb0);
        c.g[V0][0] = c.g[S0][0];
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x14), 0);
        c.sw(this.wrapping_add(0x10), c.g[V1][0]);
        c.sw(this.wrapping_add(0x18), 0);
        c.sw(this.wrapping_add(0x1c), 0);
        c.sw(this.wrapping_add(0x20), 0);
        c.sw(this.wrapping_add(0x28), 0);
        c.sw(this.wrapping_add(0x24), 0);
        c.sw(this.wrapping_add(0x2c), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysFloatItem* MenuSysFloatItem_ctor(item, a1)` (0x002cdc88): the base
/// menu item (with the -1 data word), then the float item's vtable, unit scale
/// 1.0 and zeroed fields.
pub fn menu_sys_float_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = u64::MAX;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x002c_a258, 0x002c_dca0)?; // MenuSysItem_ctor
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[V1][0] = sx(0x0048_6b60);
        c.g[V0][0] = c.g[S0][0];
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x10), c.g[V1][0]);
        c.swc1(0, this.wrapping_add(0x24));
        c.sw(this.wrapping_add(0x14), 0);
        c.sw(this.wrapping_add(0x18), 0);
        c.sw(this.wrapping_add(0x1c), 0);
        c.sw(this.wrapping_add(0x20), 0);
        c.sw(this.wrapping_add(0x30), 0);
        c.sw(this.wrapping_add(0x2c), 0);
        c.sw(this.wrapping_add(0x28), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysFloatRangeItem* MenuSysFloatRangeItem_ctor(item, a1, a2, a3, f12, f13)`
/// (0x002ce170): the base menu item, then the range item's vtable, its two
/// int fields and the float range (f12 = low, f13 = high).
pub fn menu_sys_float_range_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A3][0];
        frame.save_fpr(c, 21, 0x48);
        c.g[S2][0] = c.g[A2][0];
        frame.save_fpr(c, 20, 0x40);
        c.mov_s(21, 13);
        frame.save_ra(c, 0);
        c.mov_s(20, 12);
        call_ptr(c, 0x002c_a258, 0x002c_e1a4)?; // MenuSysItem_ctor
        c.g[V1][0] = sx(0x0048_6b10);
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x14), c.g[S1][0]);
        c.sw(this.wrapping_add(0x18), c.g[S2][0]);
        c.swc1(20, this.wrapping_add(0x1c));
        c.g[V0][0] = c.g[S0][0];
        c.swc1(21, this.wrapping_add(0x20));
        c.sw(this.wrapping_add(0x10), c.g[V1][0]);
        c.sw(this.wrapping_add(0x28), 0);
        c.sw(this.wrapping_add(0x24), 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x48);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysGroupItem_PageForwardV03(item)` (0x002cf788): forward one page
/// through the group's vtable: the slot at +0x4c of the vtable at +0x10 of the
/// group (`item+0x130`), called with `this` moved by the slot's delta at +0x48.
pub fn menu_sys_group_item_page_forward_v03(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A3][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x130));
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002c_f7a8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSys_FormatFloat(char* buf, float v, ...)` (0x002cdfd8): formats a float item value with the fixed-point rounding table (0x40da10 helpers) and writes it through the string formatter.
#[allow(unused_assignments, unused_variables, unreachable_code, unused_mut, unused_labels)]
pub fn menu_sys_format_float(c: &mut Cpu) {
    guest(c, |c| {
    'b_2ce038: {
    c.cycles += 21;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x48));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.mov_s(21, 12);
    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
    c.g[RA][0] = sx(0x002cdffc);
    c.g[S1][0] = c.g[A0][0];
    call_ptr(c, 0x0040da10, 0x002cdffc)?;
    c.g[AT][0] = 0x41200000;
    c.mtc1(1, 12);
    c.g[RA][0] = sx(0x002ce00c);
    c.mov_s(20, 0);
    call_ptr(c, 0x0040da10, 0x002ce00c)?;
    c.div_s(20, 20, 0);
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3fc8));
    c.add_s(20, 20, 0);
    c.cvt_w_s(1, 20);
    c.cvt_s_w(1, 1);
    c.c_lt_s(20, 1);
    let t = !c.fcc();
    
    if t {
    break 'b_2ce038;
    } else {
    c.cycles += 3;
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 0);
    c.sub_s(1, 1, 0);
    break 'b_2ce038;
    }
    }
    'b_2ce0a8: {
    'b_2ce054: {
    c.cycles += 5;
    c.cvt_w_s(0, 1);
    c.mfc1(3, 0);
    let t = (c.g[V1][0] as i64) >= 0;
    c.g[V0][0] = 0x4a0000;
    if t {
    break 'b_2ce054;
    } else {
    c.cycles += 2;
    let t = true;
    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3998));
    if t {
    break 'b_2ce0a8;
    } else {
    break 'b_2ce054;
    }
    }
    }
    'b_2ce068: {
    c.cycles += 2;
    let t = (c.g[V1][0] as i64) > 0;
    c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
    if t {
    break 'b_2ce068;
    } else {
    c.cycles += 3;
    c.g[V0][0] = 0x4a0000;
    let t = true;
    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x39a0));
    if t {
    break 'b_2ce0a8;
    } else {
    break 'b_2ce068;
    }
    }
    }
    'b_2ce078: {
    c.cycles += 2;
    let t = c.g[V0][0] == 0u64;
    c.g[V0][0] = 0x4a0000;
    if t {
    break 'b_2ce078;
    } else {
    c.cycles += 2;
    let t = true;
    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x39a8));
    if t {
    break 'b_2ce0a8;
    } else {
    break 'b_2ce078;
    }
    }
    }
    'b_2ce08c: {
    c.cycles += 3;
    c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
    let t = c.g[V0][0] == 0u64;
    c.g[V0][0] = 0x4a0000;
    if t {
    break 'b_2ce08c;
    } else {
    c.cycles += 2;
    let t = true;
    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x39b0));
    if t {
    break 'b_2ce0a8;
    } else {
    break 'b_2ce08c;
    }
    }
    }
    'b_2ce0a0: {
    c.cycles += 3;
    c.g[V0][0] = ((c.g[V1][0] as i64) < 7i64) as u64;
    let t = c.g[V0][0] == 0u64;
    c.g[V0][0] = 0x4a0000;
    if t {
    break 'b_2ce0a0;
    } else {
    c.cycles += 2;
    let t = true;
    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x39b8));
    if t {
    break 'b_2ce0a8;
    } else {
    break 'b_2ce0a0;
    }
    }
    }
    c.cycles += 2;
    c.g[V0][0] = 0x4a0000;
    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x39c0));
    break 'b_2ce0a8;
    }
    c.cycles += 22;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xda0));
    c.mov_s(12, 21);
    c.g[RA][0] = sx(0x002ce0b8);
    c.sw((c.g[SP][0] as u32), c.g[V0][0]);
    call_ptr(c, 0x00413af8, 0x002ce0b8)?;
    c.g[A0][0] = c.g[SP][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[RA][0] = sx(0x002ce0c8);
    c.g[A1][0] = c.g[S0][0];
    call_ptr(c, 0x003194a8, 0x002ce0c8)?;
    c.g[A1][0] = c.g[SP][0];
    c.g[RA][0] = sx(0x002ce0d4);
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x00317fe8, 0x002ce0d4)?;
    c.g[A0][0] = c.g[SP][0];
    c.g[RA][0] = sx(0x002ce0e0);
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x00318260, 0x002ce0e0)?;
    c.g[V0][0] = c.g[S1][0];
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
    c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x48));
    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    c.pc = t; return Ok(());
    });
}

/// `int MenuSysFloatRangeItem_HandleInput(item)` (0x002ce1f0): base input, the disabled check, then the range slider step, clamp and value callback.
#[allow(unused_assignments, unused_variables, unreachable_code, unused_mut, unused_labels)]
pub fn menu_sys_float_range_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
    'b_2ce350: {
    'b_2ce34c: {
    c.cycles += 12;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[S0][0] = c.g[A0][0];
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[RA][0] = sx(0x002ce20c);
    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
    call_ptr(c, 0x002ca2b0, 0x002ce20c)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[RA][0] = sx(0x002ce218);
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x002ca3b0, 0x002ce218)?;
    let t = c.g[V0][0] != 0u64;
    c.g[V0][0] = c.g[S1][0];
    if t {
    break 'b_2ce34c;
    } else {
    c.cycles += 2;
    if c.g[S1][0] != 0u64 {
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    break 'b_2ce350;
    } else {
    'b_2ce27c: {
    'b_2ce264: {
    c.cycles += 10;
    c.g[RA][0] = sx(0x002ce230);
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x002ca428, 0x002ce230)?;
    c.mov_s(20, 0);
    c.g[RA][0] = sx(0x002ce23c);
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x002ca408, 0x002ce23c)?;
    c.sub_s(20, 20, 0);
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3fc4));
    c.c_le_s(20, 0);
    let t = c.fcc();
    
    if t {
    break 'b_2ce264;
    } else {
    c.cycles += 5;
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3fc0));
    c.c_le_s(0, 20);
    let t = !c.fcc();
    c.g[V1][0] = 0u64;
    if t {
    break 'b_2ce27c;
    } else {
    break 'b_2ce264;
    }
    }
    }
    c.cycles += 5;
    c.mtc1(0, 0);
    c.c_lt_s(20, 0);
    let t = !c.fcc();
    c.g[V1][0] = 0x1;
    if t {
    break 'b_2ce27c;
    } else {
    c.cycles += 1;
    c.g[V1][0] = 0xffffffffffffffff;
    break 'b_2ce27c;
    }
    }
    'b_2ce29c: {
    'b_2ce298: {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
    if c.g[V1][0] != c.g[V0][0] {
    c.sw((c.g[S0][0] as u32).wrapping_add(0x24), 0u64);
    break 'b_2ce298;
    } else {
    c.cycles += 4;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
    let t = true;
    c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
    if t {
    break 'b_2ce29c;
    } else {
    break 'b_2ce298;
    }
    }
    }
    c.cycles += 1;
    c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[V1][0]);
    break 'b_2ce29c;
    }
    c.cycles += 2;
    let t = c.g[V1][0] == 0u64;
    c.g[V0][0] = c.g[S1][0];
    if t {
    break 'b_2ce34c;
    } else {
    'b_2ce310: {
    c.cycles += 27;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x3fbc));
    c.mult(0, 2, 2, 2);
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 5);
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3fb8));
    c.lwc1(3, (c.g[S0][0] as u32).wrapping_add(0x20));
    c.lwc1(4, (c.g[S0][0] as u32).wrapping_add(0x1c));
    c.mtc1(2, 0);
    c.cvt_s_w(0, 0);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
    c.sub_s(3, 3, 4);
    c.lwc1(2, (c.g[V1][0] as u32));
    c.mul_s(0, 0, 6);
    c.max_s(0, 0, 5);
    c.mul_s(1, 0, 1);
    c.mul_s(1, 20, 1);
    c.mul_s(1, 1, 3);
    c.add_s(2, 2, 1);
    c.swc1(2, (c.g[V1][0] as u32));
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
    c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x1c));
    c.lwc1(0, (c.g[V0][0] as u32));
    c.c_lt_s(0, 1);
    if c.fcc() {
    c.swc1(1, (c.g[V0][0] as u32));
    break 'b_2ce310;
    } else {
    break 'b_2ce310;
    }
    }
    'b_2ce32c: {
    c.cycles += 7;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
    c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x20));
    c.lwc1(0, (c.g[V0][0] as u32));
    c.c_lt_s(1, 0);
    if c.fcc() {
    c.swc1(1, (c.g[V0][0] as u32));
    break 'b_2ce32c;
    } else {
    break 'b_2ce32c;
    }
    }
    c.cycles += 8;
    c.g[A2][0] = c.lw((c.g[S0][0] as u32));
    c.g[A1][0] = c.g[S0][0];
    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x12c));
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
    let t = c.g[V1][0] as u32;
    c.g[RA][0] = sx(0x002ce348);
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    call_ptr(c, t, 0x002ce348)?;
    c.g[V0][0] = c.g[S1][0];
    break 'b_2ce34c;
    }
    }
    }
    }
    c.cycles += 1;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    break 'b_2ce350;
    }
    c.cycles += 5;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = t; return Ok(());
    });
}

/// `void MenuSysTextEditItem_Measure(out, row)` (0x002ce820): measure the row, then the text-edit padding and the row width helper.
#[allow(unused_assignments, unused_variables, unreachable_code, unused_mut, unused_labels)]
pub fn menu_sys_text_edit_item_measure(c: &mut Cpu) {
    guest(c, |c| {
    'b_2ce898: {
    'b_2ce894: {
    c.cycles += 20;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[S2][0] = c.g[A1][0];
    c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
    c.g[S3][0] = c.g[A0][0];
    c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x18));
    c.g[RA][0] = sx(0x002ce84c);
    c.g[A0][0] = c.lw((c.g[S2][0] as u32));
    call_ptr(c, 0x002cbf30, 0x002ce84c)?;
    c.g[S0][0] = c.g[S2][0] ^ c.g[V0][0];
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = c.g[S1][0];
    c.g[RA][0] = sx(0x002ce860);
    c.g[S0][0] = (c.g[S0][0] < 0x1u64) as u64;
    call_ptr(c, 0x004166f4, 0x002ce860)?;
    c.g[RA][0] = sx(0x002ce868);
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x00416810, 0x002ce868)?;
    let t = c.g[S0][0] == 0u64;
    c.g[V1][0] = c.g[V0][0];
    if t {
    break 'b_2ce894;
    } else {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x28));
    let t = c.g[V0][0] != c.g[V1][0];
    c.g[A1][0] = c.g[S2][0];
    if t {
    break 'b_2ce898;
    } else {
    c.cycles += 3;
    c.g[V0][0] = ((c.g[V0][0] as i64) < 15i64) as u64;
    let t = c.g[V0][0] == 0u64;
    c.g[A1][0] = 0x4a0000;
    if t {
    break 'b_2ce894;
    } else {
    c.cycles += 3;
    c.g[A0][0] = c.g[SP][0];
    c.g[RA][0] = sx(0x002ce894);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x39d0));
    call_ptr(c, 0x004162d0, 0x002ce894)?;
    break 'b_2ce894;
    }
    }
    }
    }
    c.cycles += 1;
    c.g[A1][0] = c.g[S2][0];
    break 'b_2ce898;
    }
    c.cycles += 29;
    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
    c.g[A3][0] = 0u64;
    c.g[RA][0] = sx(0x002ce8ac);
    c.g[T0][0] = c.g[SP][0];
    call_ptr(c, 0x002ca4c8, 0x002ce8ac)?;
    c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x37));
    c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x3f));
    c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x38));
    c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x27));
    c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x2f));
    c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x28));
    c.g[V0][0] = c.g[S3][0];
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x27));
    c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x2f));
    c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x28));
    c.sdl(6, (c.g[S3][0] as u32).wrapping_add(0x7));
    c.sdr(6, (c.g[S3][0] as u32));
    c.sdl(3, (c.g[S3][0] as u32).wrapping_add(0xf));
    c.sdr(3, (c.g[S3][0] as u32).wrapping_add(0x8));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
    c.pc = t; return Ok(());
    });
}

/// `int MenuSysColorChannelItem_HandleInput(item)` (0x002ced78): base input, the disabled check, then the channel step, clamp and value callback.
#[allow(unused_assignments, unused_variables, unreachable_code, unused_mut, unused_labels)]
pub fn menu_sys_color_channel_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
    'b_2ceed4: {
    'b_2ceed0: {
    c.cycles += 12;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[S0][0] = c.g[A0][0];
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[RA][0] = sx(0x002ced94);
    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
    call_ptr(c, 0x002ca2b0, 0x002ced94)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[RA][0] = sx(0x002ceda0);
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x002ca3b0, 0x002ceda0)?;
    let t = c.g[V0][0] != 0u64;
    c.g[V0][0] = c.g[S1][0];
    if t {
    break 'b_2ceed0;
    } else {
    c.cycles += 2;
    if c.g[S1][0] != 0u64 {
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    break 'b_2ceed4;
    } else {
    'b_2cee04: {
    'b_2cedec: {
    c.cycles += 10;
    c.g[RA][0] = sx(0x002cedb8);
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x002ca428, 0x002cedb8)?;
    c.mov_s(20, 0);
    c.g[RA][0] = sx(0x002cedc4);
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x002ca408, 0x002cedc4)?;
    c.sub_s(20, 20, 0);
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3fb0));
    c.c_le_s(20, 0);
    let t = c.fcc();
    
    if t {
    break 'b_2cedec;
    } else {
    c.cycles += 5;
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3fac));
    c.c_le_s(0, 20);
    let t = !c.fcc();
    c.g[V1][0] = 0u64;
    if t {
    break 'b_2cee04;
    } else {
    break 'b_2cedec;
    }
    }
    }
    c.cycles += 5;
    c.mtc1(0, 0);
    c.c_lt_s(20, 0);
    let t = !c.fcc();
    c.g[V1][0] = 0x1;
    if t {
    break 'b_2cee04;
    } else {
    c.cycles += 1;
    c.g[V1][0] = 0xffffffffffffffff;
    break 'b_2cee04;
    }
    }
    'b_2cee24: {
    'b_2cee20: {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
    if c.g[V1][0] != c.g[V0][0] {
    c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), 0u64);
    break 'b_2cee20;
    } else {
    c.cycles += 4;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
    let t = true;
    c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
    if t {
    break 'b_2cee24;
    } else {
    break 'b_2cee20;
    }
    }
    }
    c.cycles += 1;
    c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[V1][0]);
    break 'b_2cee24;
    }
    'b_2ceecc: {
    c.cycles += 2;
    let t = c.g[V1][0] == 0u64;
    c.g[A1][0] = 0u64;
    if t {
    break 'b_2ceecc;
    } else {
    'b_2cee44: {
    c.cycles += 6;
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
    c.g[A0][0] = 0x120;
    c.g[A3][0] = 0x1;
    c.mult(0, 3, 3, 3);
    if c.g[A0][0] == c.g[A1][0] {
    break 'b_2cee44;
    } else {
    break 'b_2cee44;
    }
    }
    'b_2cee94: {
    c.cycles += 20;
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3fa8));
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
    c.div(0, 3, 4);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.mtc1(0, 3);
    c.lwc1(2, (c.g[A2][0] as u32));
    c.g[V1][0] = c.lo[0];
    c.g[A1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
    if c.g[A1][0] != 0 { c.g[A3][0] = c.g[V1][0]; }
    c.mtc1(7, 0);
    c.cvt_s_w(0, 0);
    c.mul_s(1, 0, 1);
    c.mul_s(1, 20, 1);
    c.add_s(2, 2, 1);
    c.c_lt_s(2, 3);
    if c.fcc() {
    c.mov_s(2, 3);
    break 'b_2cee94;
    } else {
    break 'b_2cee94;
    }
    }
    'b_2ceeac: {
    c.cycles += 6;
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 0);
    c.c_lt_s(0, 2);
    if c.fcc() {
    c.mov_s(2, 0);
    break 'b_2ceeac;
    } else {
    break 'b_2ceeac;
    }
    }
    c.cycles += 8;
    c.swc1(2, (c.g[A2][0] as u32));
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = c.lw((c.g[S0][0] as u32));
    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x12c));
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
    let t = c.g[V1][0] as u32;
    c.g[RA][0] = sx(0x002ceecc);
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    call_ptr(c, t, 0x002ceecc)?;
    break 'b_2ceecc;
    }
    }
    c.cycles += 1;
    c.g[V0][0] = c.g[S1][0];
    break 'b_2ceed0;
    }
    }
    }
    c.cycles += 1;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    break 'b_2ceed4;
    }
    c.cycles += 5;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = t; return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, elf_image, fakes, Rng, DATA};

    /// A menu item for the input handlers: its first word is the object, whose
    /// +0x12c word is a class table with the value callback (`delta` at +0x20,
    /// the function at +0x24, which the machine's miss hook answers), and the
    /// float and counter fields the handlers read.
    fn oc1558_input_item(c: &mut Cpu, r: &mut Rng, item: u32) {
        let obj = DATA + 0x3000;
        let table = DATA + 0x3100;
        let float_block = DATA + 0x3200;
        c.bus.write32(item, obj);
        c.bus.write32(obj + 0x12c, table);
        c.bus.write16(table + 0x20, (r.next_u32() as i16 as i32 & !3) as u16);
        c.bus.write32(table + 0x24, fakes::FAKE);
        c.bus.write32(item + 0x14, float_block);
        c.bus.write32(item + 0x18, float_block);
        for i in 0..4 {
            c.bus.write32(float_block + 4 * i, r.f32(10.0));
        }
        c.bus.write32(item + 0x1c, r.f32(10.0));
        c.bus.write32(item + 0x20, r.f32(10.0));
        c.bus.write32(item + 0x24, r.next_u32() % 4);
        c.bus.write32(item + 0x28, r.next_u32() % 4);
    }

    #[test]
    fn menu_sys_int_item_ctor_2_matches() {
        check(
            ee::s09::f_002cd8a8__generated,
            menu_sys_int_item_ctor_2,
            40,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x002c_d8a8, 1);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(DATA + 0x200);
            },
        );
    }

    #[test]
    fn menu_sys_float_item_ctor_matches() {
        check(
            ee::s09::f_002cdc88__generated,
            menu_sys_float_item_ctor,
            40,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x002c_dc88, 1);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(DATA + 0x200);
            },
        );
    }

    #[test]
    fn menu_sys_float_range_item_ctor_matches() {
        check(
            ee::s09::f_002ce170__generated,
            menu_sys_float_range_item_ctor,
            40,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x002c_e170, 1);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.f[12] = r.edge_f32();
                c.f[13] = r.edge_f32();
            },
        );
    }

    #[test]
    fn menu_sys_group_item_page_forward_v03_matches() {
        check(
            ee::s09::f_002cf788__generated,
            menu_sys_group_item_page_forward_v03,
            24,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x002c_f788, 1);
                let (item, group) = (DATA + 0x100, DATA + 0x200);
                // the group's vtable: delta at +0x48, the forwarding slot at +0x4c
                let table = fakes::vtable(c, &mut r, &[0x48]);
                c.bus.write32(group + 0x10, table);
                c.bus.write32(item + 0x130, group);
                c.g[A0][0] = sx(item);
            },
        );
    }

    #[test]
    fn menu_sys_format_float_matches() {
        check(
            ee::s09::f_002cdfd8__generated,
            menu_sys_format_float,
            40,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x002c_dfd8, 3);
                elf_image(c);
                c.g[A0][0] = sx(DATA + 0x100);
                c.f[12] = r.edge_f32();
            },
        );
    }

    #[test]
    fn menu_sys_float_range_item_handle_input_matches() {
        check(
            ee::s09::f_002ce1f0__generated,
            menu_sys_float_range_item_handle_input,
            40,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x002c_e1f0, 3);
                elf_image(c);
                let item = DATA + 0x100;
                oc1558_input_item(c, &mut r, item);
                c.g[A0][0] = sx(item);
                c.f[20] = r.edge_f32();
            },
        );
    }

    #[test]
    fn menu_sys_color_channel_item_handle_input_matches() {
        check(
            ee::s09::f_002ced78__generated,
            menu_sys_color_channel_item_handle_input,
            40,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x002c_ed78, 3);
                elf_image(c);
                let item = DATA + 0x100;
                oc1558_input_item(c, &mut r, item);
                c.g[A0][0] = sx(item);
                c.f[20] = r.edge_f32();
            },
        );
    }

    #[test]
    fn menu_sys_text_edit_item_measure_matches() {
        check(
            ee::s09::f_002ce820__generated,
            menu_sys_text_edit_item_measure,
            40,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x002c_e820, 3);
                elf_image(c);
                let (out, item, page) = (DATA + 0x100, DATA + 0x200, DATA + 0x300);
                c.bus.write32(item, page);
                c.bus.write32(item + 0x14, DATA + 0x400);
                c.bus.write32(item + 0x18, r.next_u32());
                c.bus.write32(item + 0x28, r.next_u32() % 20);
                c.g[A0][0] = sx(out);
                c.g[A1][0] = sx(item);
            },
        );
    }
}
