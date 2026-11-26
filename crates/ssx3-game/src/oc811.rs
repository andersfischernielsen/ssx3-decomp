//! Staging module oc811 (wave wave247): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc811`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0038_acd0, irradiance_load_811, "Irradiance_Load"),
    (0x0039_7ad8, ui_ascii_to_wide, "UI_AsciiToWide"),
    (
        0x0039_8568,
        ui_font_set_load_fonts_811,
        "UIFontSet_LoadFonts",
    ),
    (
        0x0039_f8c8,
        ui_state_mgr_destroy_all,
        "UIStateMgr_DestroyAll",
    ),
];

/// `void cIrradianceDataBase_Load(db)` (`Irradiance_Load`, 0x0038acd0): load the
/// irradiance file, allocate the index table (`count * 8`) and the sample array
/// (`count * 0xa0`), copy both from the file image and release the file buffer.
/// The normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn irradiance_load_811(c: &mut Cpu) {
    if c.pc != 0x0038acd0 {
        return ee::s12::f_0038acd0__generated(c);
    }
    guest(c, |c| {
        'b_38ad90: {
            c.cycles += 45;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.g[A1][0] = 0x100;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S0][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[A0][0] = 0x490000;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[RA][0] = sx(0x0038ad00);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2e68));
            call_ptr(c, 0x003e18c8, 0x0038ad00)?;
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            c.g[S2][0] = 0xa0;
            c.g[A1][0] = 0x490000;
            c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2e80));
            c.g[A2][0] = 0u64;
            c.g[A3][0] = 0u64;
            c.g[S1][0] = sx((c.g[V1][0] as u32) << 3);
            c.mult(0, 18, 3, 18);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
            c.g[RA][0] = sx(0x0038ad34);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x00317e30, 0x0038ad34)?;
            c.g[S4][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S1][0] as u32));
            c.g[A1][0] = 0x490000;
            c.g[A3][0] = 0u64;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2e90));
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = 0x2000000;
            c.g[RA][0] = sx(0x0038ad54);
            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            call_ptr(c, 0x00317e30, 0x0038ad54)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[A1][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x0038ad68);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            call_ptr(c, 0x0041605c, 0x0038ad68)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            c.g[A1][0] = c.g[S4][0];
            c.g[RA][0] = sx(0x0038ad78);
            c.g[A2][0] = c.g[S2][0];
            call_ptr(c, 0x0041605c, 0x0038ad78)?;
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            let t = c.g[A0][0] == 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            if t {
                break 'b_38ad90;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x0038ad8c);
                call_ptr(c, 0x00317e98, 0x0038ad8c)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_38ad90;
            }
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

/// `UI_AsciiToWide(wide, ascii)` (0x00397ad8): widen a NUL-terminated byte string
/// into 16-bit units (sign-extending each byte), then store the terminator.
/// The normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ui_ascii_to_wide(c: &mut Cpu) {
    if c.pc != 0x00397ad8 {
        return ee::s13::f_00397ad8__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x00397ad8;
        loop {
            match pc {
                0x00397ad8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lb(c.g[A1][0] as u32);
                    if t {
                        pc = 0x00397af8;
                        continue;
                    }
                    pc = 0x00397ae0;
                    continue;
                }
                0x00397ae0 => {
                    c.cycles += 6;
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 24);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                    c.sh(c.g[A0][0] as u32, c.g[V0][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
                    c.g[V0][0] = c.lb(c.g[A1][0] as u32);
                    pc = 0x00397af8;
                    continue;
                }
                0x00397af8 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = c.lbu(c.g[A1][0] as u32);
                    if t {
                        if back_edge_unwinds(c, 0x00397ae0) {
                            return Ok(());
                        }
                        pc = 0x00397ae0;
                        continue;
                    }
                    pc = 0x00397b00;
                    continue;
                }
                0x00397b00 => {
                    c.cycles += 2;
                    let t = c.g[RA][0] as u32;
                    c.sh(c.g[A0][0] as u32, 0u64);
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00397ad8, 0x00397b08);
                    return Ok(());
                }
            }
        }
    });
}

/// `UIFontSet_LoadFonts(self)` (0x00398568): for each of the signed byte count
/// (`+0x34`) font entries (12 bytes from `+4`, name pointer at `+4` of the entry)
/// create a font object (`EA_MemAllocNamed`, 0x84 bytes), construct it from the
/// name (`Font_ctor`) and store it at `entry + 0xc`.
/// The normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ui_font_set_load_fonts_811(c: &mut Cpu) {
    if c.pc != 0x00398568 {
        return ee::s13::f_00398568__generated(c);
    }
    guest(c, |c| {
        'b_3985f4: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S3][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V0][0] = c.lb((c.g[S3][0] as u32).wrapping_add(0x34));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S1][0] = 0u64;
            if t {
                break 'b_3985f4;
            } else {
                c.cycles += 4;
                c.g[S5][0] = 0x4a0000;
                c.g[S4][0] = 0xc;
                c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
                c.g[S0][0] = sx((c.g[S1][0] as u32) << 24);
                'l_3985a8: loop {
                    c.cycles += 19;
                    c.g[A0][0] = 0x84;
                    c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 24) as u32);
                    c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4780));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x003985c0);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x003985c0)?;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x003985d8);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x003915e8, 0x003985d8)?;
                    c.mult(0, 3, 16, 20);
                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S3][0] as u32));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    c.g[V0][0] = c.lb((c.g[S3][0] as u32).wrapping_add(0x34));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S1][0] as u32) << 24);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003985a8);
                        }
                        continue 'l_3985a8;
                    } else {
                        break 'b_3985f4;
                    }
                }
            }
        }
        c.cycles += 9;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

/// `cUIStateStack_destroyStates(self)` (`UIStateMgr_DestroyAll`, 0x0039f8c8): for the
/// main list and the explicit-state list (`+0x1c`), every state without the
/// explicit bit (bit 6 of `+0x1c`) gets virtual slot 5 (`onDestroyScreen`) and
/// is deleted through slot 1 with 3.
/// The normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ui_state_mgr_destroy_all(c: &mut Cpu) {
    if c.pc != 0x0039f8c8 {
        return ee::s13::f_0039f8c8__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x0039f8c8;
        loop {
            match pc {
                0x0039f8c8 => {
                    c.cycles += 10;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[RA][0] = sx(0x0039f8e4);
                    c.g[S2][0] = c.g[A0][0];
                    Callee {
                        addr: 0x00397840,
                        f: ee::s13::UIList_First,
                    }
                    .call(c, 0x0039f8e4)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;

                    if t {
                        pc = 0x0039f94c;
                        continue;
                    }
                    pc = 0x0039f8f0;
                    continue;
                }
                0x0039f8f0 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x0039f93c;
                        continue;
                    }
                    pc = 0x0039f8f8;
                    continue;
                }
                0x0039f8f8 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x0039f938;
                        continue;
                    }
                    pc = 0x0039f908;
                    continue;
                }
                0x0039f908 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0039f91c);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0039f91c)?;
                    let t = c.g[S0][0] == 0u64;
                    c.g[A1][0] = 0x3;
                    if t {
                        pc = 0x0039f938;
                        continue;
                    }
                    pc = 0x0039f924;
                    continue;
                }
                0x0039f924 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0039f938);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0039f938)?;
                    pc = 0x0039f938;
                    continue;
                }
                0x0039f938 => {
                    c.cycles += 1;
                    c.g[S0][0] = c.g[S1][0];
                    pc = 0x0039f93c;
                    continue;
                }
                0x0039f93c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0039f944);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00397718,
                        f: ee::s13::UIListNode_IsSentinel,
                    }
                    .call(c, 0x0039f944)?;

                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                        if back_edge_unwinds(c, 0x0039f8f8) {
                            return Ok(());
                        }
                        pc = 0x0039f8f8;
                        continue;
                    }
                    pc = 0x0039f94c;
                    continue;
                }
                0x0039f94c => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x0039f954);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1c));
                    Callee {
                        addr: 0x00397840,
                        f: ee::s13::UIList_First,
                    }
                    .call(c, 0x0039f954)?;
                    c.g[S0][0] = c.g[V0][0];

                    if c.g[S0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        pc = 0x0039f9c0;
                        continue;
                    }
                    pc = 0x0039f960;
                    continue;
                }
                0x0039f960 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x0039f9ac;
                        continue;
                    }
                    pc = 0x0039f968;
                    continue;
                }
                0x0039f968 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x0039f9a8;
                        continue;
                    }
                    pc = 0x0039f978;
                    continue;
                }
                0x0039f978 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0039f98c);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0039f98c)?;
                    let t = c.g[S0][0] == 0u64;
                    c.g[A1][0] = 0x3;
                    if t {
                        pc = 0x0039f9a8;
                        continue;
                    }
                    pc = 0x0039f994;
                    continue;
                }
                0x0039f994 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0039f9a8);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0039f9a8)?;
                    pc = 0x0039f9a8;
                    continue;
                }
                0x0039f9a8 => {
                    c.cycles += 1;
                    c.g[S0][0] = c.g[S1][0];
                    pc = 0x0039f9ac;
                    continue;
                }
                0x0039f9ac => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0039f9b4);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00397718,
                        f: ee::s13::UIListNode_IsSentinel,
                    }
                    .call(c, 0x0039f9b4)?;

                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                        if back_edge_unwinds(c, 0x0039f968) {
                            return Ok(());
                        }
                        pc = 0x0039f968;
                        continue;
                    }
                    pc = 0x0039f9bc;
                    continue;
                }
                0x0039f9bc => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    pc = 0x0039f9c0;
                    continue;
                }
                0x0039f9c0 => {
                    c.cycles += 5;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0039f8c8, 0x0039f9d4);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes::{self, machine, LOG_RESULTS};
    use crate::difftest::{check, Rng, DATA};
    use std::sync::atomic::AtomicUsize;

    /// Gate of the oc811 fakes (a magic word in the machine's RAM) and the
    /// mailbox of the file loader fake (0: the fake's pointer, 1: return 0).
    const OC811_CTRL: u32 = DATA + 0x20_0000;
    const OC811_MAIL: u32 = OC811_CTRL + 4;
    const OC811_MAGIC: u32 = 0x4f43_3831;

    /// The fake callee: logs like `fakes::fake_call`; the file loader fake can
    /// be told to return 0.
    fn oc811_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(OC811_MAIL);
        fakes::fake_call(c, addr);
        if addr == 0x003e_18c8 && c.pc == ra && mail == 1 {
            c.g[V0][0] = 0;
        }
    }

    macro_rules! oc811_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC811_CTRL) != OC811_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc811_fake_call(c, $addr);
                }
            )*
            fn oc811_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc811_fakes!(
        (
            oc811_fake_file_load,
            OC811_PREV_FILE_LOAD,
            ee::s14::OVERRIDE_003E18C8,
            ee::s14::f_003e18c8__generated,
            0x003e_18c8
        ),
        (
            oc811_fake_new_named,
            OC811_PREV_NEW_NAMED,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc811_fake_mem_free,
            OC811_PREV_MEM_FREE,
            ee::s10::OVERRIDE_00317E98,
            ee::s10::f_00317e98__generated,
            0x0031_7e98
        ),
        (
            oc811_fake_memcpy,
            OC811_PREV_MEMCPY,
            ee::s15::OVERRIDE_0041605C,
            ee::s15::f_0041605c__generated,
            0x0041_605c
        ),
        (
            oc811_fake_alloc_named,
            OC811_PREV_ALLOC_NAMED,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc811_fake_font_ctor,
            OC811_PREV_FONT_CTOR,
            ee::s12::OVERRIDE_003915E8,
            ee::s12::f_003915e8__generated,
            0x0039_15e8
        ),
    );

    /// Irradiance_Load: the loader, both allocations, both copies and the
    /// release of the file buffer (skipped when the loader returns 0), with
    /// the callees faked; fake call n unwinds on some seeds.
    #[test]
    fn irradiance_load_matches() {
        oc811_install_fakes();
        check(
            ee::s12::f_0038acd0__generated,
            irradiance_load_811,
            48,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0038_acd0, 6);
                c.pc = 0x0038_acd0;
                c.bus.write32(OC811_CTRL, OC811_MAGIC);
                c.bus.write32(OC811_MAIL, (seed % 4 == 0) as u32);
                let file = DATA + 0x1000;
                c.bus.write32(file, r.next_u32() % 7);
                c.bus.write32(LOG_RESULTS, file);
                c.g[A0][0] = DATA as u64;
                for w in 0..4 {
                    c.bus.write32(DATA + 4 * w, r.next_u32());
                }
            },
        );
    }

    /// UI_AsciiToWide on random byte strings (including the empty string and
    /// bytes with the high bit set), with events due inside the loop.
    #[test]
    fn ui_ascii_to_wide_matches() {
        driven!(gen_ascii, ee::s13::f_00397ad8__generated);
        driven!(hand_ascii, ui_ascii_to_wide);
        check_events(gen_ascii, hand_ascii, 96, &|c, seed| {
            let mut r = Rng(0x3979_d8 ^ (seed as u64) << 8);
            let src = DATA + 0x100;
            let dst = DATA + 0x400;
            let len = match seed % 5 {
                0 => 0,
                _ => (r.next_u32() % 12) as u32,
            };
            for i in 0..len {
                let b = (r.next_u32() as u8).max(1);
                c.bus.write8(src + i, b);
            }
            c.bus.write8(src + len, 0);
            for i in 0..0x40 {
                c.bus.write8(dst + i, 0x5a);
            }
            c.g[A0][0] = dst as u64;
            c.g[A1][0] = src as u64;
            c.g[RA][0] = crate::difftest::RA as u64;
            c.g[V1][0] = r.next_u32() as u64;
            schedule(c, &mut r, seed, 0x0039_7ad8)
        });
    }

    /// Build the font set: `count` entries of 12 bytes from `+4`, each with a
    /// random name pointer at `+4` of the entry.
    fn oc811_font_set(c: &mut Cpu, r: &mut Rng, count: i8) -> u32 {
        let this = DATA;
        c.bus.write8(this + 0x34, count as u8);
        for i in 0..4 {
            let e = this + 4 + 12 * i;
            c.bus.write32(e, r.next_u32());
            c.bus.write32(e + 4, DATA + 0x800 + (r.next_u32() & 0xff0));
            c.bus.write32(e + 8, r.next_u32());
        }
        c.g[A0][0] = this as u64;
        this
    }

    /// UIFontSet_LoadFonts: with the allocator and the font constructor
    /// faked (some seeds unwind inside a call), then with events due at the
    /// loop's back edge.
    #[test]
    fn ui_font_set_load_fonts_matches() {
        oc811_install_fakes();
        check(
            ee::s13::f_00398568__generated,
            ui_font_set_load_fonts_811,
            48,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0039_8568, 8);
                c.pc = 0x0039_8568;
                c.bus.write32(OC811_CTRL, OC811_MAGIC);
                oc811_font_set(c, &mut r, [-2, 0, 1, 2, 3, 4][seed as usize % 6]);
            },
        );
        driven!(gen_fonts, ee::s13::f_00398568__generated);
        driven!(hand_fonts, ui_font_set_load_fonts_811);
        check_events(gen_fonts, hand_fonts, 48, &|c, seed| {
            let mut r = machine(c, seed, 0x0039_8569, 0);
            c.bus.write32(OC811_CTRL, OC811_MAGIC);
            oc811_font_set(c, &mut r, 2 + (seed % 3) as i8);
            c.g[RA][0] = crate::difftest::RA as u64;
            schedule(c, &mut r, seed, 0x0039_8568)
        });
    }

    /// One state list: `n` states from `nodes` (0x40 apart), then the
    /// sentinel (next and prev pointing at itself); `head + 4` links the first.
    /// Each state gets the shared vtable and a random explicit bit (bit 6 of
    /// `+0x1c`).
    fn oc811_state_list(c: &mut Cpu, r: &mut Rng, head: u32, nodes: u32, n: u32, vt: u32) {
        let sentinel = nodes + 0x40 * n;
        for i in 0..n {
            let node = nodes + 0x40 * i;
            c.bus.write32(node, head);
            c.bus.write32(node + 4, node + 0x40);
            c.bus.write32(node + 8, vt);
            c.bus.write32(node + 0x1c, r.next_u32());
        }
        c.bus.write32(sentinel, sentinel);
        c.bus.write32(sentinel + 4, sentinel);
        c.bus.write32(head + 4, nodes);
    }

    fn oc811_destroy_setup(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let this = DATA + 0x100;
        let vt = DATA + 0x800;
        for (i, off) in [0x28u32, 0x8].into_iter().enumerate() {
            c.bus
                .write16(vt + off, (r.next_u32() as i16 as i32 & !3) as u16);
            c.bus.write32(vt + off + 4, fakes::FAKE + 0x10 * i as u32);
        }
        oc811_state_list(c, r, this, DATA + 0x1000, seed % 4, vt);
        oc811_state_list(c, r, this + 0x1c, DATA + 0x2000, (seed / 4) % 4, vt);
        c.g[A0][0] = this as u64;
    }

    /// UIStateMgr_DestroyAll: both lists, states with and without the explicit
    /// bit (virtual slots 5 and 1 called through fakes, some seeds unwind),
    /// then events due at the back edges (resumes run the generated body).
    #[test]
    fn ui_state_mgr_destroy_all_matches() {
        check(
            ee::s13::f_0039f8c8__generated,
            ui_state_mgr_destroy_all,
            96,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0039_f8c8, 10);
                c.pc = 0x0039_f8c8;
                oc811_destroy_setup(c, &mut r, seed);
            },
        );
        driven!(gen_destroy, ee::s13::f_0039f8c8__generated);
        driven!(hand_destroy, ui_state_mgr_destroy_all);
        check_events(gen_destroy, hand_destroy, 96, &|c, seed| {
            let mut r = machine(c, seed, 0x0039_f8c9, 0);
            oc811_destroy_setup(c, &mut r, seed);
            c.g[RA][0] = crate::difftest::RA as u64;
            schedule(c, &mut r, seed, 0x0039_f8c8)
        });
    }
}
