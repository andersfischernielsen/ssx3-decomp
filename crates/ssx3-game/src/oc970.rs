//! Staging module oc970 (wave wave257): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc970`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

use crate::abi;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001f_4a90,
        oc970_on_widget_action,
        "cFEStateBuyAttrib_OnWidgetAction",
    ),
    (
        0x001f_4ea8,
        oc970_update_level_text,
        "cFEStateBuyAttrib_UpdateLevelText",
    ),
    (
        0x001f_4fc0,
        oc970_update_cost_text,
        "cFEStateBuyAttrib_UpdateCostText",
    ),
    (
        0x001f_50b0,
        oc970_update_bars,
        "cFEStateBuyAttrib_UpdateBars",
    ),
    (
        0x001f_5240,
        oc970_update_total_cost,
        "cFEStateBuyAttrib_UpdateTotalCost",
    ),
];

/// `base + off` as a 32-bit guest address (the `off(reg)` operand).
fn oc970_at(c: &Cpu, r: usize, off: u32) -> u32 {
    (c.g[r][0] as u32).wrapping_add(off)
}

// Format strings and callees of the buy-attrib screens (the generated bodies'
// addresses): sprintf 0x417828, StrHash 0x317670, UIScreen_FindObject 0x39d860,
// UIText_SetAsciiString 0x3a0c50, int to string 0x198af0.

/// `void cFEStateBuyAttrib_UpdateTotalCost(cFEStateBuyAttrib* self)`
/// (0x1f5240): the total-cost text gets the item's cost (`self+0xa0`) as a
/// string, if the screen has a text object for it.
pub fn oc970_update_total_cost(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(0x0046_f090);
        call_ptr(c, 0x0031_7670, 0x001f_5260)?;
        c.g[A0][0] = Addr(oc970_at(c, S1, 0x48)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001f_526c)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 6;
            c.g[A0][0] = Addr(oc970_at(c, S1, 0xa0)).lw(c);
            call_ptr(c, 0x0019_8af0, 0x001f_5280)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_0c50, 0x001f_528c)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateBuyAttrib_UpdateLevelText(cFEStateBuyAttrib* self)`
/// (0x1f4ea8): for each of the seven attribute rows, the level is
/// `points / 5` (kept at its maximum so far), the rest `points - 5 * level`
/// is shown as `rem * 2` next to it, in the text object `"<row name>"`.
pub fn oc970_update_level_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S5, 0x70);
        frame.save(c, S2, 0xa0);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, S3, 0x90);
        c.g[S2][0] = sx(oc970_at(c, S5, 0x68));
        frame.save(c, S4, 0x80);
        c.g[S3][0] = 0;
        frame.save(c, S6, 0x60);
        c.g[S4][0] = sx(sp(c).wrapping_add(0x20));
        frame.save(c, S7, 0x50);
        c.g[S6][0] = 5;
        frame.save(c, FP, 0x40);
        c.g[S7][0] = 0x4a_0000;
        frame.save(c, S0, 0xc0);
        c.g[FP][0] = 0x4a_0000;
        frame.save(c, S1, 0xb0);
        frame.save_ra(c, 0x30);
        c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_sub(0x1c)).lw(c);
        'lp: loop {
            c.cycles += 2;
            // Level and remainder of row i; the level only ever grows.
            c.cycles += 11;
            c.g[A2][0] = c.g[S3][0];
            c.g[A0][0] = c.g[SP][0];
            c.div(0, V0, S6);
            c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x2338));
            c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
            c.g[S1][0] = c.lo[0];
            c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S1][0] as i64)) as u64;
            let t = c.g[V0][0] == 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S1][0] as u32));
            if !t {
                c.cycles += 1;
                let level = c.g[S1][0];
                Addr(c.g[S2][0] as u32).sw(c, level);
            }
            // Name the row, look its text up and set it.
            c.cycles += 15;
            c.g[S0][0] = Addr((c.g[S2][0] as u32).wrapping_sub(0x1c)).lw(c);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            call_ptr(c, 0x0041_7828, 0x001f_4f40)?;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0031_7670, 0x001f_4f48)?;
            c.g[A0][0] = Addr(oc970_at(c, S5, 0x48)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001f_4f54)?;
            c.g[A3][0] = sx((c.g[S0][0] as u32) << 1);
            c.g[A2][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S4][0];
            c.g[S0][0] = c.g[V0][0];
            let found = c.g[S0][0] != 0;
            c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x2340));
            if found {
                c.cycles += 5;
                call_ptr(c, 0x0041_7828, 0x001f_4f74)?;
                c.g[A1][0] = c.g[S4][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x003a_0c50, 0x001f_4f80)?;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S3][0] as i64) < 7) as u64;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_sub(0x1c)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x001f_4ef8);
                }
                continue 'lp;
            }
            break;
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
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

/// `void cFEStateBuyAttrib_UpdateCostText(cFEStateBuyAttrib* self)`
/// (0x1f4fc0): for each of the seven rows, the text shows the row's level
/// minus one through the BE interface, and for levels below 11 the row's
/// vtable entry is called with 0 (the level-maxed marker at 11 and up).
pub fn oc970_update_cost_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S3, 0x50);
        frame.save(c, S1, 0x70);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S2, 0x60);
        c.g[S1][0] = sx(oc970_at(c, S3, 0x68));
        frame.save(c, S4, 0x40);
        c.g[S2][0] = 0;
        frame.save(c, S5, 0x30);
        frame.save(c, S0, 0x80);
        frame.save_ra(c, 0x20);
        c.g[S5][0] = 0x4a_0000;
        call_ptr(c, 0x0014_dc80, 0x001f_4ff4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xb;
        call_ptr(c, 0x0014_dd58, 0x001f_5000)?;
        c.g[S4][0] = c.g[V0][0];
        'lp: loop {
            c.cycles += 13;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x2348));
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            call_ptr(c, 0x0041_7828, 0x001f_501c)?;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0031_7670, 0x001f_5024)?;
            c.g[A0][0] = Addr(oc970_at(c, S3, 0x48)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001f_5030)?;
            c.g[S0][0] = c.g[V0][0];
            let missing = c.g[S0][0] == 0;
            c.g[A0][0] = c.g[S4][0];
            if !missing {
                c.cycles += 12;
                c.g[A1][0] = Addr(c.g[S1][0] as u32).lw(c);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                call_ptr(c, 0x0015_0e50, 0x001f_5048)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0019_8af0, 0x001f_5050)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x003a_0c50, 0x001f_505c)?;
                c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < 11) as u64;
                let below_max = c.g[V0][0] != 0;
                c.g[A1][0] = 0;
                if !below_max {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(oc970_at(c, S0, 8)).lw(c);
                    c.g[A0][0] = Addr(oc970_at(c, V0, 0x48)).lh(c);
                    c.g[V1][0] = Addr(oc970_at(c, V0, 0x4c)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x001f_5080)?;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 7) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001f_5008);
                }
                continue 'lp;
            }
            break;
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateBuyAttrib_UpdateBars(cFEStateBuyAttrib* self)` (0x1f50b0):
/// positions the seven points bars (`pts[i] * 3.3636` wide, height 12) and
/// the add-on bars (`(add[i] + pts[i]) * 3.3636`), each a text-screen object
/// whose vtable takes a vector at +0xa8/+0x48 and a visibility flag.
pub fn oc970_update_bars(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S2, 0x80);
        frame.save_fpr(c, 21, 0xb8);
        c.g[S2][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0xb0);
        frame.save(c, S1, 0x90);
        frame.save(c, S3, 0x70);
        c.g[S1][0] = sx(oc970_at(c, S2, 0x4c));
        frame.save(c, S4, 0x60);
        c.g[S3][0] = 0;
        frame.save(c, S5, 0x50);
        c.g[S4][0] = 0;
        frame.save(c, S6, 0x40);
        c.g[S5][0] = 0x4a_0000;
        frame.save(c, S0, 0xa0);
        c.g[S6][0] = 0x47_0000;
        frame.save_ra(c, 0x30);
        c.g[AT][0] = 0x4140_0000;
        abi::set_fpr(c, 21, FpuF32(c.g[AT][0] as u32));
        let scale = Addr((c.g[GP][0] as u32).wrapping_sub(0x554c)).read_fpu(c);
        abi::set_fpr(c, 20, scale);
        'lp: loop {
            // The points bar of row i.
            c.cycles += 12;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0xf80));
            c.g[A2][0] = c.g[S3][0];
            call_ptr(c, 0x0041_7828, 0x001f_5110)?;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0031_7670, 0x001f_5118)?;
            c.g[A0][0] = Addr(oc970_at(c, S2, 0x48)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001f_5124)?;
            c.g[S0][0] = c.g[V0][0];
            if c.g[S0][0] == 0 {
                c.g[A0][0] = c.g[SP][0];
            } else {
                c.cycles += 19;
                let pts = Addr(c.g[S1][0] as u32).read_fpu(c);
                abi::set_fpr(c, 0, FpuF32::from_i32(pts.0 as i32));
                c.g[A1][0] = sx(sp(c).wrapping_add(0x20));
                frame.save_fpr(c, 21, 0x24);
                Addr(sp(c).wrapping_add(0x28)).sw(c, 0);
                let width = FpuF32(c.f[0]) * FpuF32(c.f[20]);
                abi::set_fpr(c, 0, width);
                Addr(sp(c).wrapping_add(0x20)).write_fpu(c, FpuF32(c.f[0]));
                c.g[V0][0] = Addr(oc970_at(c, S0, 8)).lw(c);
                c.g[A0][0] = Addr(oc970_at(c, V0, 0xa8)).lh(c);
                c.g[V1][0] = Addr(oc970_at(c, V0, 0xac)).lw(c);
                let set_vec = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, set_vec, 0x001f_5160)?;
                c.g[V0][0] = Addr(oc970_at(c, S0, 8)).lw(c);
                c.g[A1][0] = 1;
                c.g[A0][0] = Addr(oc970_at(c, V0, 0x48)).lh(c);
                c.g[V1][0] = Addr(oc970_at(c, V0, 0x4c)).lw(c);
                let set_flag = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, set_flag, 0x001f_5178)?;
                c.g[A0][0] = c.g[SP][0];
            }
            // The add-on bar of row i: (add[i] + pts[i]) wide.
            c.cycles += 11;
            c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x2350));
            c.g[A2][0] = c.g[S3][0];
            call_ptr(c, 0x0041_7828, 0x001f_5188)?;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0031_7670, 0x001f_5190)?;
            c.g[A0][0] = Addr(oc970_at(c, S2, 0x48)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001f_519c)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S4][0] as u32));
            if c.g[S0][0] != 0 {
                c.cycles += 21;
                c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
                c.g[V0][0] = Addr(oc970_at(c, V1, 0x84)).lw(c);
                c.g[A1][0] = sx(sp(c).wrapping_add(0x20));
                frame.save_fpr(c, 21, 0x24);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                Addr(sp(c).wrapping_add(0x28)).sw(c, 0);
                abi::set_fpr(c, 0, FpuF32(c.g[V0][0] as u32));
                let as_float = FpuF32::from_i32(c.f[0] as i32);
                abi::set_fpr(c, 0, as_float);
                let width = FpuF32(c.f[0]) * FpuF32(c.f[20]);
                abi::set_fpr(c, 0, width);
                Addr(sp(c).wrapping_add(0x20)).write_fpu(c, FpuF32(c.f[0]));
                c.g[V0][0] = Addr(oc970_at(c, S0, 8)).lw(c);
                c.g[A0][0] = Addr(oc970_at(c, V0, 0xa8)).lh(c);
                c.g[V1][0] = Addr(oc970_at(c, V0, 0xac)).lw(c);
                let set_vec = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, set_vec, 0x001f_51e4)?;
                c.g[V0][0] = Addr(oc970_at(c, S0, 8)).lw(c);
                c.g[A1][0] = 1;
                c.g[A0][0] = Addr(oc970_at(c, V0, 0x48)).lh(c);
                c.g[V1][0] = Addr(oc970_at(c, V0, 0x4c)).lw(c);
                let set_flag = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, set_flag, 0x001f_51fc)?;
            }
            c.cycles += 5;
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
            c.g[V0][0] = ((c.g[S3][0] as i64) < 7) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001f_5100);
                }
                continue 'lp;
            }
            break;
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore_ra(c, 0x30);
        frame.restore_fpr(c, 21, 0xb8);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateBuyAttrib_OnWidgetAction(cFEStateBuyAttrib* self, void* obj,
/// int msg)` (0x1f4a90). A null `obj` does nothing. `msg` 5 (next) opens the
/// buy popup when the bank covers the total cost and the total is positive,
/// else plays the engine's sound. `msg` 6 (prev) clears the buy prompt and
/// hides its group, or leaves the screen for the previous state. Other
/// messages return at once.
pub fn oc970_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = c.g[A1][0];
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        let obj_null = c.g[V1][0] == 0;
        frame.save_ra(c, 0);
        'c20: {
            'c1c: {
                'c18: {
                    if obj_null {
                        break 'c18;
                    }
                    'd: {
                        'e: {
                            'f: {
                                'g: {
                                    c.cycles += 3;
                                    c.g[V0][0] = 5;
                                    let is_next = c.g[A2][0] == c.g[V0][0];
                                    c.g[V0][0] = 6;
                                    if is_next {
                                        break 'g;
                                    }
                                    c.cycles += 2;
                                    let is_prev = c.g[A2][0] == c.g[V0][0];
                                    frame.restore(c, S0, 0x20);
                                    if is_prev {
                                        break 'f;
                                    }
                                    c.cycles += 2;
                                    frame.restore(c, S1, 0x10);
                                    break 'c20;
                                }
                                // msg 5: the bank against the total cost.
                                'h: {
                                    c.cycles += 5;
                                    c.g[V0][0] = Addr(oc970_at(c, S1, 0xa4)).lw(c);
                                    c.g[V1][0] = Addr(oc970_at(c, S1, 0xa0)).lw(c);
                                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                    if c.g[V0][0] != 0 {
                                        c.g[V0][0] = Addr(oc970_at(c, S1, 0x10)).lw(c);
                                        break 'h;
                                    }
                                    'i: {
                                        c.cycles += 2;
                                        let no_total = (c.g[V1][0] as i64) <= 0;
                                        c.g[A1][0] = 0x47_0000;
                                        if no_total {
                                            break 'i;
                                        }
                                        // Open the buy popup on the state stack.
                                        c.cycles += 22;
                                        c.g[A0][0] = 0x70;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xfb0));
                                        c.g[A2][0] = 0x100;
                                        c.g[A3][0] = 0;
                                        call_ptr(c, 0x0031_7d70, 0x001f_4af8)?;
                                        c.g[A1][0] = Addr(oc970_at(c, S1, 0x10)).lw(c);
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A3][0] = Addr(oc970_at(c, S1, 0x44)).lb(c);
                                        c.g[A2][0] = c.g[S1][0];
                                        call_ptr(c, 0x001c_aba8, 0x001f_4b0c)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A2][0] = Addr(oc970_at(c, S1, 0xa0)).lw(c);
                                        c.g[A3][0] = Addr(oc970_at(c, S1, 0xa4)).lw(c);
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0;
                                        call_ptr(c, 0x001c_afc0, 0x001f_4b24)?;
                                        c.g[A0][0] = Addr(oc970_at(c, S1, 0x10)).lw(c);
                                        c.g[A1][0] = c.g[S0][0];
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                        call_ptr(c, 0x0039_f290, 0x001f_4b34)?;
                                        frame.restore(c, S0, 0x20);
                                        break 'c1c;
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = Addr(oc970_at(c, S1, 0x10)).lw(c);
                                }
                                // The sound of the engine (msg 5 without the popup).
                                c.cycles += 5;
                                c.g[V1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2968));
                                c.g[A2][0] = Addr(oc970_at(c, V0, 0x14)).lw(c);
                                if c.g[A2][0] == 0 {
                                    c.g[A2][0] = c.g[V1][0];
                                }
                                let no_sound = c.g[A2][0] == 0;
                                c.g[A1][0] = 4;
                                if no_sound {
                                    break 'c18;
                                }
                                c.cycles += 4;
                                c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                                c.g[A0][0] = Addr(oc970_at(c, V0, 0x10)).lh(c);
                                c.g[V1][0] = Addr(oc970_at(c, V0, 0x14)).lw(c);
                                break 'e;
                            }
                            // msg 6: clear the buy prompt, or leave the screen.
                            c.cycles += 3;
                            c.g[V0][0] = Addr(oc970_at(c, S1, 0xa8)).lw(c);
                            let no_prompt = c.g[V0][0] == 0;
                            c.g[A1][0] = 0x47_0000;
                            if no_prompt {
                                break 'd;
                            }
                            c.cycles += 9;
                            Addr(oc970_at(c, S1, 0xa8)).sw(c, 0);
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xfa0));
                            call_ptr(c, 0x0039_e8b8, 0x001f_4b80)?;
                            c.g[A2][0] = Addr(oc970_at(c, S1, 0xac)).lw(c);
                            c.g[A1][0] = 1;
                            c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(8)).lw(c);
                            c.g[A0][0] = Addr(oc970_at(c, V0, 0x38)).lh(c);
                            c.g[V1][0] = Addr(oc970_at(c, V0, 0x3c)).lw(c);
                        }
                        // Call the sound's (or the attrib menu's) vtable entry.
                        c.cycles += 4;
                        let entry = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, entry, 0x001f_4b9c)?;
                        frame.restore(c, S0, 0x20);
                        break 'c1c;
                    }
                    // msg 6 with no buy prompt: go back to the previous state.
                    c.cycles += 12;
                    c.g[V0][0] = Addr(oc970_at(c, S1, 0x10)).lw(c);
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = Addr(oc970_at(c, V1, 0x18)).lw(c);
                    c.g[A3][0] = Addr(c.g[V0][0] as u32).lw(c);
                    c.g[V1][0] = Addr((c.g[A3][0] as u32).wrapping_add(4)).lw(c);
                    c.g[A0][0] = Addr(oc970_at(c, V1, 0x28)).lh(c);
                    c.g[V0][0] = Addr(oc970_at(c, V1, 0x2c)).lw(c);
                    let prev_entry = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, prev_entry, 0x001f_4bc8)?;
                    c.g[S0][0] = c.g[V0][0];
                    if c.g[S0][0] == 0 {
                        frame.restore(c, S0, 0x20);
                        break 'c1c;
                    }
                    // A frame with the transition-out label plays first.
                    'c08: {
                        c.cycles += 10;
                        c.g[A0][0] = 0x47_0000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fb0));
                        call_ptr(c, 0x0031_7670, 0x001f_4be0)?;
                        c.g[A0][0] = Addr(oc970_at(c, S1, 0x48)).lw(c);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039_c778, 0x001f_4bec)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V0][0] = 0xffff;
                        let no_frame = c.g[A1][0] == c.g[V0][0];
                        c.g[A1][0] = c.g[A1][0] & 0xffff;
                        if no_frame {
                            break 'c08;
                        }
                        c.cycles += 3;
                        c.g[A0][0] = Addr(oc970_at(c, S1, 0x48)).lw(c);
                        c.g[A2][0] = 1;
                        call_ptr(c, 0x0039_c870, 0x001f_4c08)?;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = Addr(oc970_at(c, S1, 0x10)).lw(c);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039_f400, 0x001f_4c18)?;
                    break 'c18;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            }
            c.cycles += 1;
            frame.restore(c, S1, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::elf_image;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine_rt;
    use crate::difftest::DATA;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    // Gated fakes for the callees that allocate or walk game data on zeroed
    // test memory: each has its own gate word, magic and forced-result word in
    // the machine's RAM (DATA+0x270000..), past the other fixture regions. The
    // result word is read as is, so a forced 0 is possible.
    const OC970_FIND_CTRL: u32 = DATA + 0x27_0000;
    const OC970_FIND_RESULT: u32 = DATA + 0x27_0004;
    const OC970_FIND_MAGIC: u32 = 0x0970_0001;
    const OC970_LOOKUP_CTRL: u32 = DATA + 0x27_0010;
    const OC970_LOOKUP_RESULT: u32 = DATA + 0x27_0014;
    const OC970_LOOKUP_MAGIC: u32 = 0x0970_0002;
    const OC970_PLAY_CTRL: u32 = DATA + 0x27_0020;
    const OC970_PLAY_RESULT: u32 = DATA + 0x27_0024;
    const OC970_PLAY_MAGIC: u32 = 0x0970_0003;
    const OC970_BE_CTRL: u32 = DATA + 0x27_0030;
    const OC970_BE_RESULT: u32 = DATA + 0x27_0034;
    const OC970_BE_MAGIC: u32 = 0x0970_0004;
    const OC970_IFACE_CTRL: u32 = DATA + 0x27_0040;
    const OC970_IFACE_RESULT: u32 = DATA + 0x27_0044;
    const OC970_IFACE_MAGIC: u32 = 0x0970_0005;
    const OC970_LEVEL_CTRL: u32 = DATA + 0x27_0050;
    const OC970_LEVEL_RESULT: u32 = DATA + 0x27_0054;
    const OC970_LEVEL_MAGIC: u32 = 0x0970_0006;
    const OC970_MONEY_CTRL: u32 = DATA + 0x27_0060;
    const OC970_MONEY_RESULT: u32 = DATA + 0x27_0064;
    const OC970_MONEY_MAGIC: u32 = 0x0970_0007;
    const OC970_SETTEXT_CTRL: u32 = DATA + 0x27_0070;
    const OC970_SETTEXT_RESULT: u32 = DATA + 0x27_0074;
    const OC970_SETTEXT_MAGIC: u32 = 0x0970_0008;

    /// One gated fake: with its gate word set it returns at once with V0 = the
    /// forced result word (0 included) and charges a fixed cost; with the gate
    /// off it runs the body the slot held before (`fall_back`).
    macro_rules! oc970_fake {
        ($fake:ident, $prev:ident, $ctrl:expr, $magic:expr, $result:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32($ctrl) != $magic {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                c.g[V0][0] = c.bus.read32($result) as u64;
                c.cycles += 5;
                c.pc = c.g[RA][0] as u32;
            }
        };
    }

    oc970_fake!(
        oc970_fake_find,
        OC970_PREV_FIND,
        OC970_FIND_CTRL,
        OC970_FIND_MAGIC,
        OC970_FIND_RESULT,
        ee::s13::f_0039d860__generated
    );
    oc970_fake!(
        oc970_fake_lookup,
        OC970_PREV_LOOKUP,
        OC970_LOOKUP_CTRL,
        OC970_LOOKUP_MAGIC,
        OC970_LOOKUP_RESULT,
        ee::s13::f_0039c778__generated
    );
    oc970_fake!(
        oc970_fake_play,
        OC970_PREV_PLAY,
        OC970_PLAY_CTRL,
        OC970_PLAY_MAGIC,
        OC970_PLAY_RESULT,
        ee::s13::f_0039c870__generated
    );
    oc970_fake!(
        oc970_fake_be,
        OC970_PREV_BE,
        OC970_BE_CTRL,
        OC970_BE_MAGIC,
        OC970_BE_RESULT,
        ee::s01::f_0014dc80__generated
    );
    oc970_fake!(
        oc970_fake_iface,
        OC970_PREV_IFACE,
        OC970_IFACE_CTRL,
        OC970_IFACE_MAGIC,
        OC970_IFACE_RESULT,
        ee::s01::f_0014dd58__generated
    );
    oc970_fake!(
        oc970_fake_level,
        OC970_PREV_LEVEL,
        OC970_LEVEL_CTRL,
        OC970_LEVEL_MAGIC,
        OC970_LEVEL_RESULT,
        ee::s01::f_00150e50__generated
    );
    oc970_fake!(
        oc970_fake_money,
        OC970_PREV_MONEY,
        OC970_MONEY_CTRL,
        OC970_MONEY_MAGIC,
        OC970_MONEY_RESULT,
        ee::s03::f_00198af0__generated
    );
    oc970_fake!(
        oc970_fake_settext,
        OC970_PREV_SETTEXT,
        OC970_SETTEXT_CTRL,
        OC970_SETTEXT_MAGIC,
        OC970_SETTEXT_RESULT,
        ee::s13::f_003a0c50__generated
    );

    fn oc970_install() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039D860),
            oc970_fake_find,
            &OC970_PREV_FIND,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039C778),
            oc970_fake_lookup,
            &OC970_PREV_LOOKUP,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039C870),
            oc970_fake_play,
            &OC970_PREV_PLAY,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc970_fake_be,
            &OC970_PREV_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc970_fake_iface,
            &OC970_PREV_IFACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00150E50),
            oc970_fake_level,
            &OC970_PREV_LEVEL,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_00198AF0),
            oc970_fake_money,
            &OC970_PREV_MONEY,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A0C50),
            oc970_fake_settext,
            &OC970_PREV_SETTEXT,
        );
    }

    /// Arm the gated fakes for one fixture. `forced` is, in order: the object
    /// UIScreen_FindObject returns, the value UIScreen_LookupValue returns, the
    /// BE object (cBE_getBE), the BE interface, the attribute cost
    /// (BEEconInt_GetAttributeCost) and the money string (FE_FormatMoney).
    /// A zero result is forced as zero.
    fn oc970_arm(c: &mut Cpu, forced: [u32; 6]) {
        let [find, lookup, be, iface, level, money] = forced;
        c.bus.write32(OC970_FIND_CTRL, OC970_FIND_MAGIC);
        c.bus.write32(OC970_FIND_RESULT, find);
        c.bus.write32(OC970_LOOKUP_CTRL, OC970_LOOKUP_MAGIC);
        c.bus.write32(OC970_LOOKUP_RESULT, lookup);
        c.bus.write32(OC970_PLAY_CTRL, OC970_PLAY_MAGIC);
        c.bus.write32(OC970_PLAY_RESULT, 0);
        c.bus.write32(OC970_BE_CTRL, OC970_BE_MAGIC);
        c.bus.write32(OC970_BE_RESULT, be);
        c.bus.write32(OC970_IFACE_CTRL, OC970_IFACE_MAGIC);
        c.bus.write32(OC970_IFACE_RESULT, iface);
        c.bus.write32(OC970_LEVEL_CTRL, OC970_LEVEL_MAGIC);
        c.bus.write32(OC970_LEVEL_RESULT, level);
        c.bus.write32(OC970_MONEY_CTRL, OC970_MONEY_MAGIC);
        c.bus.write32(OC970_MONEY_RESULT, money);
        c.bus.write32(OC970_SETTEXT_CTRL, OC970_SETTEXT_MAGIC);
        c.bus.write32(OC970_SETTEXT_RESULT, 0);
    }

    #[test]
    fn oc970_update_total_cost_matches() {
        oc970_install();
        check(
            ee::s04::f_001f5240__generated,
            oc970_update_total_cost,
            40,
            &|c, seed| {
                machine_rt(c, seed, 0x001f_5240, 3);
                elf_image(c);
                let this = DATA;
                c.bus.write32(this + 0x48, DATA + 0x1000);
                c.bus.write32(this + 0xa0, seed * 37);
                let text = if seed % 2 == 0 { 0 } else { DATA + 0x3000 };
                let money = if seed % 3 == 0 { 0 } else { DATA + 0x5000 };
                oc970_arm(c, [text, 0, 0, 0, 0, money]);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn oc970_update_level_text_matches() {
        oc970_install();
        check(
            ee::s04::f_001f4ea8__generated,
            oc970_update_level_text,
            40,
            &|c, seed| {
                machine_rt(c, seed, 0x001f_4ea8, 4);
                elf_image(c);
                let this = DATA;
                for i in 0..7u32 {
                    c.bus.write32(this + 0x4c + 4 * i, (seed + 3 * i) % 23);
                    c.bus.write32(this + 0x68 + 4 * i, (seed + i) % 4);
                }
                c.bus.write32(this + 0x48, DATA + 0x1000);
                let text = if seed % 3 == 0 { 0 } else { DATA + 0x3000 };
                oc970_arm(c, [text, 0, 0, 0, 0, 0]);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn oc970_update_cost_text_matches() {
        oc970_install();
        check(
            ee::s04::f_001f4fc0__generated,
            oc970_update_cost_text,
            40,
            &|c, seed| {
                machine_rt(c, seed, 0x001f_4fc0, 4);
                elf_image(c);
                let this = DATA;
                for i in 0..7u32 {
                    c.bus.write32(this + 0x68 + 4 * i, (seed + 5 * i) % 14);
                }
                c.bus.write32(this + 0x48, DATA + 0x1000);
                let text = if seed % 3 == 0 { 0 } else { DATA + 0x3000 };
                let be = if seed % 5 == 0 { 0 } else { DATA + 0x6000 };
                let level = if seed % 4 == 0 { 0 } else { seed % 20 };
                oc970_arm(c, [text, 0, be, be, level, DATA + 0x5000]);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn oc970_update_bars_matches() {
        oc970_install();
        check(
            ee::s04::f_001f50b0__generated,
            oc970_update_bars,
            40,
            &|c, seed| {
                machine_rt(c, seed, 0x001f_50b0, 4);
                elf_image(c);
                let this = DATA;
                for i in 0..7u32 {
                    c.bus.write32(this + 0x4c + 4 * i, (seed + 2 * i) % 30);
                    c.bus.write32(this + 0x84 + 4 * i, (seed + i) % 9);
                }
                c.bus.write32(this + 0x48, DATA + 0x1000);
                let text = if seed % 4 == 0 { 0 } else { DATA + 0x3000 };
                oc970_arm(c, [text, 0, 0, 0, 0, 0]);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn oc970_on_widget_action_matches() {
        oc970_install();
        check(
            ee::s04::f_001f4a90__generated,
            oc970_on_widget_action,
            60,
            &|c, seed| {
                machine_rt(c, seed, 0x001f_4a90, 4);
                elf_image(c);
                let this = DATA;
                let obj = if seed % 7 == 0 { 0 } else { DATA + 0x1000 };
                let engine = DATA + 0x2000;
                let sound = DATA + 0x2400;
                c.bus.write32(this + 0x10, engine);
                c.bus
                    .write32(engine + 0x14, if seed % 3 == 0 { 0 } else { sound });
                c.bus.write32(this + 0xa0, 10 + seed % 4);
                c.bus.write32(this + 0xa4, seed % 15);
                c.bus.write32(this + 0xa8, seed % 2);
                c.bus.write32(this + 0xac, DATA + 0x2800);
                c.bus.write32(this + 0x48, DATA + 0x1000);
                let text = if seed % 2 == 0 { 0 } else { DATA + 0x3000 };
                oc970_arm(c, [text, 0, 0, 0, 0, 0]);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(obj);
                c.g[A2][0] = [5u64, 6, 7][(seed % 3) as usize];
            },
        );
    }
}
