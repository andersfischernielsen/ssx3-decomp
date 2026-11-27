//! Staging module oc1044 (wave wave264): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1044`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002d_8a00,
        avalanche_debug_draw_instance,
        "Avalanche_DebugDrawInstance",
    ),
    (0x002d_92d0, avalanche_add_node, "Avalanche_AddNode"),
];

/// `reg + off` as a 32-bit address.
#[inline(always)]
fn oc1044_at(c: &Cpu, r: usize, off: u32) -> u32 {
    (c.g[r][0] as u32).wrapping_add(off)
}

/// `reg - off` as a 32-bit address.
#[inline(always)]
fn oc1044_back(c: &Cpu, r: usize, off: u32) -> u32 {
    (c.g[r][0] as u32).wrapping_sub(off)
}

/// `ra + rb` (32-bit, for `addu`).
#[inline(always)]
fn oc1044_add32(c: &Cpu, a: usize, b: usize) -> u32 {
    (c.g[a][0] as u32).wrapping_add(c.g[b][0] as u32)
}

/// Node field: the instance record the node draws (`tAvalancheObject*`).
pub(crate) const AVNODE_OBJ: u32 = 0x2e0;
/// Instance record: pointer to the 10-byte line records.
pub(crate) const AVOBJ_RECS: u32 = 0xfc;
/// Instance record: number of records (float, converted with `cvt.w.s`).
pub(crate) const AVOBJ_COUNT: u32 = 0x100;
/// Instance record: origin of the first line (a vector, 16 bytes).
pub(crate) const AVOBJ_POS: u32 = 0xe0;
/// Debug-draw flag (`gp + 0x968`).
pub(crate) const AVDEBUG_FLAG: u32 = 0x968;
/// Global render state (`gp - 0x854`).
pub(crate) const AVRS_GLOBAL: u32 = 0x854;

/// `void Avalanche_DebugDrawInstance(Node* node)` (0x2d8a00): when the debug
/// flag is set, draws every line record of the node's instance as a small
/// oriented marker. Each record's offset/axis/colour bytes are turned into a
/// rotation matrix about the (normalised) axis, the matrix is multiplied with
/// the running transform and stored with the colour into a table at
/// 0x4d5b70 (0x30 bytes per record); finally the render state's draw method
/// (vtable slot 0x248) is called with `(count, table, 0)`. The render-state
/// stack pointer (`*(world + 0xe84)`) is pushed with a copy of the matrix at
/// 0x4ff1a0 before and popped after.
pub fn avalanche_debug_draw_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x1d0);
        frame.save(c, 16, 0x1b0);
        frame.save(c, 17, 0x1a0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x190);
        frame.save(c, 19, 0x180);
        frame.save(c, 20, 0x170);
        frame.save(c, 21, 0x160);
        frame.save(c, 22, 0x150);
        frame.save(c, 23, 0x140);
        frame.save(c, 30, 0x130);
        frame.save_ra(c, 0x120);
        frame.save_fpr(c, 21, 0x1c8);
        frame.save_fpr(c, 20, 0x1c0);
        c.g[V0][0] = c.lw(oc1044_at(c, S0, AVNODE_OBJ));
        let no_obj = c.g[V0][0] == 0;
        c.g[V0][0] = 0x500000;
        'done: {
            if no_obj {
                break 'done;
            }
            'pushed: {
                c.cycles += 34;
                // Copy the 4x4 matrix at 0x4ff1a0 to the stack.
                c.g[V0][0] = sx(oc1044_back(c, V0, 0xe60));
                c.lqc2(1, oc1044_at(c, V0, 0));
                c.lqc2(2, oc1044_at(c, V0, 0x10));
                c.lqc2(3, oc1044_at(c, V0, 0x20));
                c.lqc2(4, oc1044_at(c, V0, 0x30));
                c.sqc2(1, oc1044_at(c, SP, 0));
                c.sqc2(2, oc1044_at(c, SP, 0x10));
                c.sqc2(3, oc1044_at(c, SP, 0x20));
                c.sqc2(4, oc1044_at(c, SP, 0x30));
                // Push a 0x14-byte render-state record (copy of the top one).
                c.g[A0][0] = c.lw(oc1044_back(c, GP, AVRS_GLOBAL));
                c.g[A3][0] = 0xffff_ffff_ffff_ff83;
                c.g[V0][0] = c.lw(oc1044_at(c, A0, 0xe84));
                c.ldl(3, oc1044_at(c, V0, 0x7));
                c.ldr(3, oc1044_at(c, V0, 0));
                c.ldl(5, oc1044_at(c, V0, 0xf));
                c.ldr(5, oc1044_at(c, V0, 0x8));
                c.g[A2][0] = c.lw(oc1044_at(c, V0, 0x10));
                c.sdl(3, oc1044_at(c, V0, 0x1b));
                c.sdr(3, oc1044_at(c, V0, 0x14));
                c.sdl(5, oc1044_at(c, V0, 0x23));
                c.sdr(5, oc1044_at(c, V0, 0x1c));
                c.sw(oc1044_at(c, V0, 0x24), c.g[A2][0]);
                c.g[V1][0] = c.lw(oc1044_at(c, A0, 0xe84));
                c.g[A2][0] = c.lw(oc1044_back(c, GP, AVRS_GLOBAL));
                c.g[V1][0] = sx(oc1044_at(c, V1, 0x14));
                c.sw(oc1044_at(c, A0, 0xe84), c.g[V1][0]);
                // Set bit 2 / clear the 0x7c bits (mask -125) in the new top's flags.
                c.g[A1][0] = c.lw(oc1044_at(c, A2, 0xe84));
                c.g[V0][0] = c.lw(oc1044_at(c, A1, 0x4));
                c.g[V0][0] &= c.g[A3][0];
                c.g[V0][0] |= 0x4;
                c.sw(oc1044_at(c, A1, 0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw(oc1044_at(c, GP, AVDEBUG_FLAG));
                let flag_off = c.g[V1][0] == 0;
                c.g[S2][0] = 0;
                if flag_off {
                    break 'pushed;
                }
                'lines: {
                    c.cycles += 9;
                    c.g[V1][0] = c.lw(oc1044_at(c, S0, AVNODE_OBJ));
                    c.lq(2, oc1044_at(c, V1, AVOBJ_POS));
                    c.sq(2, oc1044_at(c, SP, 0x40));
                    c.lwc1(0, oc1044_at(c, V1, AVOBJ_COUNT));
                    c.cvt_w_s(1, 0);
                    c.mfc1(2, 1);
                    let none = (c.g[V0][0] as i64) <= 0;
                    c.g[FP][0] = 0x4d0000;
                    if none {
                        break 'lines;
                    }
                    c.cycles += 10;
                    c.g[V0][0] = 0x500000;
                    c.g[S5][0] = sx(oc1044_at(c, FP, 0x5b70));
                    c.lwc1(21, oc1044_back(c, GP, 0x3cd0));
                    c.g[S7][0] = sx(oc1044_back(c, V0, 0xed0));
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 20);
                    c.g[S6][0] = sx(oc1044_at(c, S5, 0x20));
                    c.g[S4][0] = c.g[SP][0];
                    c.g[S3][0] = sx(oc1044_at(c, SP, 0x80));
                    c.g[S1][0] = 0;
                    loop {
                        // -- one record: position step, axis, angle --
                        c.cycles += 52;
                        c.g[V1][0] = c.lw(oc1044_at(c, S0, AVNODE_OBJ));
                        c.mtc1(0, 3);
                        c.g[V0][0] = c.lw(oc1044_at(c, V1, AVOBJ_RECS));
                        c.g[V0][0] = sx(oc1044_add32(c, S1, V0));
                        c.g[V1][0] = c.lb(oc1044_at(c, V0, 0x2));
                        c.g[A0][0] = c.lb(oc1044_at(c, V0, 0));
                        c.g[A1][0] = c.lb(oc1044_at(c, V0, 0x1));
                        c.mtc1(3, 1);
                        c.cvt_s_w(1, 1);
                        c.mtc1(4, 2);
                        c.cvt_s_w(2, 2);
                        c.swc1(3, oc1044_at(c, SP, 0x5c));
                        c.mtc1(5, 0);
                        c.cvt_s_w(0, 0);
                        c.add_s(1, 1, 1);
                        c.add_s(2, 2, 2);
                        c.add_s(0, 0, 0);
                        c.swc1(1, oc1044_at(c, SP, 0x58));
                        c.swc1(2, oc1044_at(c, SP, 0x50));
                        c.swc1(0, oc1044_at(c, SP, 0x54));
                        c.lqc2(3, oc1044_at(c, SP, 0x40));
                        c.lqc2(4, oc1044_at(c, SP, 0x50));
                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, oc1044_at(c, SP, 0x40));
                        c.g[V0][0] = c.lw(oc1044_at(c, S0, AVNODE_OBJ));
                        c.g[V1][0] = c.lw(oc1044_at(c, V0, AVOBJ_RECS));
                        c.g[V1][0] = sx(oc1044_add32(c, S1, V1));
                        c.g[V0][0] = c.lb(oc1044_at(c, V1, 0x6));
                        c.g[A1][0] = c.lb(oc1044_at(c, V1, 0x7));
                        c.mtc1(2, 2);
                        c.cvt_s_w(2, 2);
                        c.g[A0][0] = c.lb(oc1044_at(c, V1, 0x8));
                        c.mtc1(5, 0);
                        c.cvt_s_w(0, 0);
                        c.mtc1(4, 1);
                        c.cvt_s_w(1, 1);
                        c.mul_s(5, 2, 21);
                        c.mul_s(6, 0, 21);
                        c.mul_s(4, 1, 21);
                        c.mul_s(0, 5, 5);
                        c.swc1(5, oc1044_at(c, SP, 0x70));
                        c.mul_s(2, 6, 6);
                        c.swc1(6, oc1044_at(c, SP, 0x74));
                        c.mul_s(1, 4, 4);
                        c.swc1(4, oc1044_at(c, SP, 0x78));
                        c.add_s(0, 0, 2);
                        c.add_s(0, 0, 1);
                        c.sqrt_s(0, 0);
                        c.c_eq_s(0, 3);
                        if c.fcc() {
                            // zero-length axis: stored unnormalised
                            c.swc1(5, oc1044_at(c, SP, 0x60));
                            c.cycles += 2;
                            c.swc1(6, oc1044_at(c, SP, 0x64));
                            c.swc1(4, oc1044_at(c, SP, 0x68));
                        } else {
                            c.cycles += 8;
                            c.div_s(0, 20, 0);
                            c.mul_s(1, 4, 0);
                            c.mul_s(2, 5, 0);
                            c.mul_s(0, 6, 0);
                            c.swc1(1, oc1044_at(c, SP, 0x68));
                            c.swc1(2, oc1044_at(c, SP, 0x60));
                            c.swc1(0, oc1044_at(c, SP, 0x64));
                        }
                        // -- rotation about the axis by the angle byte --
                        c.cycles += 136;
                        c.lwc1(0, oc1044_at(c, SP, 0x60));
                        c.lwc1(1, oc1044_at(c, SP, 0x64));
                        c.lwc1(2, oc1044_at(c, SP, 0x68));
                        c.swc1(0, oc1044_at(c, SP, 0x50));
                        c.swc1(1, oc1044_at(c, SP, 0x54));
                        c.swc1(2, oc1044_at(c, SP, 0x58));
                        c.sw(oc1044_at(c, SP, 0x5c), 0);
                        c.lwc1(0, oc1044_back(c, GP, 0x3ccc));
                        c.g[V1][0] = c.lw(oc1044_at(c, S0, AVNODE_OBJ));
                        c.g[V0][0] = c.lw(oc1044_at(c, V1, AVOBJ_RECS));
                        c.g[V0][0] = sx(oc1044_add32(c, S1, V0));
                        c.g[V1][0] = c.lbu(oc1044_at(c, V0, 0x9));
                        c.mtc1(3, 12);
                        c.cvt_s_w(12, 12);
                        c.mul_s(12, 12, 0);
                        c.lqc2(1, oc1044_at(c, S4, 0));
                        c.lqc2(2, oc1044_at(c, S4, 0x10));
                        c.lqc2(3, oc1044_at(c, S4, 0x20));
                        c.lqc2(4, oc1044_at(c, S4, 0x30));
                        c.sqc2(1, oc1044_at(c, S3, 0));
                        c.sqc2(2, oc1044_at(c, S3, 0x10));
                        c.sqc2(3, oc1044_at(c, S3, 0x20));
                        c.sqc2(4, oc1044_at(c, S3, 0x30));
                        c.lq(2, oc1044_at(c, SP, 0xb0));
                        c.g[A0][0] = sx(oc1044_at(c, SP, 0x110));
                        c.lq(3, oc1044_at(c, S7, 0));
                        c.g[A1][0] = sx(oc1044_at(c, SP, 0x114));
                        c.sq(2, oc1044_at(c, SP, 0xc0));
                        c.sq(3, oc1044_at(c, SP, 0xb0));
                        // Math_SinCos(angle, &sin, &cos)
                        call_ptr(c, 0x0031_be50, 0x002d_8c88)?;
                        c.lwc1(11, oc1044_at(c, SP, 0x114));
                        c.lwc1(0, oc1044_at(c, SP, 0x50));
                        c.sub_s(1, 20, 11);
                        c.lwc1(3, oc1044_at(c, SP, 0x58));
                        c.lwc1(7, oc1044_at(c, SP, 0x54));
                        c.lwc1(8, oc1044_at(c, SP, 0x110));
                        c.mul_s(2, 1, 3);
                        c.lq(2, oc1044_at(c, SP, 0xc0));
                        c.mul_s(4, 1, 0);
                        c.sw(oc1044_at(c, SP, 0x30), 0);
                        c.mul_s(1, 1, 7);
                        c.sw(oc1044_at(c, SP, 0x34), 0);
                        c.mul_s(13, 8, 3);
                        c.sw(oc1044_at(c, SP, 0x38), 0);
                        c.mul_s(12, 8, 0);
                        c.swc1(20, oc1044_at(c, SP, 0x3c));
                        c.mul_s(9, 2, 3);
                        c.sq(2, oc1044_at(c, SP, 0x30));
                        c.mul_s(10, 4, 3);
                        c.sw(oc1044_at(c, SP, 0xc), 0);
                        c.mul_s(5, 2, 0);
                        c.sw(oc1044_at(c, SP, 0x1c), 0);
                        c.mul_s(6, 4, 0);
                        c.sw(oc1044_at(c, SP, 0x2c), 0);
                        c.mul_s(3, 1, 3);
                        c.mul_s(0, 1, 0);
                        c.mul_s(8, 8, 7);
                        c.mul_s(2, 2, 7);
                        c.mul_s(4, 4, 7);
                        c.mul_s(1, 1, 7);
                        c.add_s(5, 5, 8);
                        c.sub_s(0, 0, 13);
                        c.sub_s(2, 2, 12);
                        c.add_s(9, 9, 11);
                        c.swc1(5, oc1044_at(c, SP, 0x8));
                        c.add_s(6, 6, 11);
                        c.swc1(0, oc1044_at(c, SP, 0x4));
                        c.add_s(4, 4, 13);
                        c.swc1(2, oc1044_at(c, SP, 0x18));
                        c.sub_s(10, 10, 8);
                        c.swc1(9, oc1044_at(c, SP, 0x28));
                        c.add_s(1, 1, 11);
                        c.swc1(6, oc1044_at(c, SP, 0));
                        c.add_s(3, 3, 12);
                        c.swc1(4, oc1044_at(c, SP, 0x10));
                        c.swc1(10, oc1044_at(c, SP, 0x20));
                        c.swc1(1, oc1044_at(c, SP, 0x14));
                        c.swc1(3, oc1044_at(c, SP, 0x24));
                        // running = running * rotation (VU0 4x4 multiply)
                        c.lqc2(4, oc1044_at(c, S4, 0));
                        c.lqc2(5, oc1044_at(c, S4, 0x10));
                        c.lqc2(6, oc1044_at(c, S4, 0x20));
                        c.lqc2(7, oc1044_at(c, S4, 0x30));
                        c.lqc2(8, oc1044_at(c, S3, 0));
                        c.lqc2(9, oc1044_at(c, S3, 0x10));
                        c.lqc2(10, oc1044_at(c, S3, 0x20));
                        c.lqc2(11, oc1044_at(c, S3, 0x30));
                        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                        c.vu0().mula(0b1111, 4, Src::Bc(9, X));
                        c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
                        c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
                        c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
                        c.vu0().mula(0b1111, 4, Src::Bc(10, X));
                        c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
                        c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
                        c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
                        c.vu0().mula(0b1111, 4, Src::Bc(11, X));
                        c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
                        c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
                        c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
                        c.sqc2(12, oc1044_at(c, S4, 0));
                        c.sqc2(13, oc1044_at(c, S4, 0x10));
                        c.sqc2(14, oc1044_at(c, S4, 0x20));
                        c.sqc2(15, oc1044_at(c, S4, 0x30));
                        // -- table entry: matrix row + colour bytes --
                        c.lq(7, oc1044_at(c, SP, 0x40));
                        c.g[A1][0] = 0x30;
                        c.g[V0][0] = c.lw(oc1044_at(c, S0, AVNODE_OBJ));
                        c.mult(0, 5, 18, 5);
                        c.sq(7, oc1044_at(c, SP, 0x30));
                        c.g[S2][0] = sx(oc1044_at(c, S2, 1));
                        c.g[V1][0] = c.lw(oc1044_at(c, V0, AVOBJ_RECS));
                        c.g[A2][0] = sx(oc1044_add32(c, A1, S5));
                        c.g[V1][0] = sx(oc1044_add32(c, S1, V1));
                        c.g[A1][0] = sx(oc1044_add32(c, A1, S6));
                        c.g[V0][0] = c.lbu(oc1044_at(c, V1, 0x3));
                        c.sw(oc1044_at(c, A2, 0x10), c.g[V0][0]);
                        c.g[V1][0] = c.lw(oc1044_at(c, S0, AVNODE_OBJ));
                        c.g[V0][0] = c.lw(oc1044_at(c, V1, AVOBJ_RECS));
                        c.g[V0][0] = sx(oc1044_add32(c, S1, V0));
                        c.g[V1][0] = c.lbu(oc1044_at(c, V0, 0x4));
                        c.sw(oc1044_at(c, A2, 0x14), c.g[V1][0]);
                        c.g[A0][0] = c.lw(oc1044_at(c, S0, AVNODE_OBJ));
                        c.g[V0][0] = c.lw(oc1044_at(c, A0, AVOBJ_RECS));
                        c.g[V0][0] = sx(oc1044_add32(c, S1, V0));
                        c.g[V1][0] = c.lbu(oc1044_at(c, V0, 0x5));
                        c.sw(oc1044_at(c, A2, 0x18), c.g[V1][0]);
                        c.sq(7, oc1044_at(c, A1, 0));
                        c.g[V1][0] = c.lw(oc1044_at(c, S0, AVNODE_OBJ));
                        c.lwc1(0, oc1044_at(c, V1, AVOBJ_COUNT));
                        c.cvt_w_s(1, 0);
                        c.mfc1(2, 1);
                        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[S1][0] = sx(oc1044_at(c, S1, 0xa));
                        if !more {
                            break 'lines;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x002d_8b18);
                        }
                    }
                }
                // Submit: vtable slot 0x248 of the render state's draw object.
                c.cycles += 12;
                c.g[V1][0] = c.lw(oc1044_at(c, S0, AVNODE_OBJ));
                c.g[A2][0] = sx(oc1044_at(c, FP, 0x5b70));
                c.g[T0][0] = c.lw(oc1044_back(c, GP, AVRS_GLOBAL));
                c.g[A3][0] = 0;
                c.lwc1(0, oc1044_at(c, V1, AVOBJ_COUNT));
                c.g[V0][0] = c.lw(oc1044_at(c, T0, 0x10d8));
                c.cvt_w_s(1, 0);
                c.mfc1(5, 1);
                c.g[A0][0] = c.lh(oc1044_at(c, V0, 0x248));
                c.g[V1][0] = c.lw(oc1044_at(c, V0, 0x24c));
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx(oc1044_add32(c, T0, A0));
                call_ptr(c, target, 0x002d_8e60)?;
            }
            // Pop the render-state record.
            c.cycles += 4;
            c.g[V1][0] = c.lw(oc1044_back(c, GP, AVRS_GLOBAL));
            c.g[V0][0] = c.lw(oc1044_at(c, V1, 0xe84));
            c.g[V0][0] = sx(oc1044_back(c, V0, 0x14));
            c.sw(oc1044_at(c, V1, 0xe84), c.g[V0][0]);
        }
        c.cycles += 14;
        frame.restore(c, 16, 0x1b0);
        frame.restore(c, 17, 0x1a0);
        frame.restore(c, 18, 0x190);
        frame.restore(c, 19, 0x180);
        frame.restore(c, 20, 0x170);
        frame.restore(c, 21, 0x160);
        frame.restore(c, 22, 0x150);
        frame.restore(c, 23, 0x140);
        frame.restore(c, 30, 0x130);
        frame.restore_ra(c, 0x120);
        frame.restore_fpr(c, 21, 0x1c8);
        frame.restore_fpr(c, 20, 0x1c0);
        frame.close(c);
        Ok(())
    })
}

/// Node-list head (`gp + 0x9c0`): singly linked, `next` at +8.
pub(crate) const AVALANCHE_NODE_LIST: u32 = 0x9c0;
/// Free-record counter (`gp + 0x9d0`): index of the next record slot.
pub(crate) const AVALANCHE_REC_COUNT: u32 = 0x9d0;
/// Flag cleared by `Avalanche_AddNode` (`gp + 0x9cc`).
pub(crate) const AVALANCHE_DIRTY: u32 = 0x9cc;

/// `void Avalanche_AddNode(int id, tInstanceRecord* inst, int kind,
/// float x, float y, float z, float w)` (0x2d92d0): finds (or allocates, 0x120
/// bytes, linked at the head of the node list) the node for `id`, then
/// allocates a 0x210-byte record, links it in front of the node's record
/// list and fills it: the four floats (scaled by 32 for x, y, z), the owner
/// `inst`, a mode halfword derived from `kind` (2 -> 0, 5 -> 2, else 1), and
/// the largest-extent ratio of the instance's bounding box, then registers the
/// record in the global table and bumps the counter.
pub fn avalanche_add_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x90);
        frame.save(c, 16, 0x60);
        frame.save(c, 17, 0x50);
        frame.save(c, 18, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, 19, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 20, 0x20);
        c.g[S3][0] = 0x480000;
        frame.save(c, 21, 0x10);
        c.g[S4][0] = c.g[A2][0];
        frame.save_fpr(c, 23, 0x88);
        c.g[S5][0] = 0x540000;
        frame.save_fpr(c, 22, 0x80);
        c.mov_s(23, 13);
        frame.save_fpr(c, 21, 0x78);
        c.mov_s(22, 12);
        frame.save_fpr(c, 20, 0x70);
        c.mov_s(21, 15);
        c.g[S0][0] = c.lw(oc1044_at(c, GP, AVALANCHE_NODE_LIST));
        c.mov_s(20, 14);
        frame.save_ra(c, 0);

        // Look the node up by id.
        let mut found = false;
        loop {
            c.cycles += 2;
            let end = c.g[S0][0] == 0;
            c.g[A1][0] = 0x480000;
            if end {
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            if c.g[V0][0] != c.g[S2][0] {
                c.g[S0][0] = c.lw(oc1044_at(c, S0, 0x8));
                if event_due(c) {
                    deliver_event_here(c, 0x002d_9328);
                }
                continue;
            }
            // Same id: s0 is non-null here, so the node is found.
            c.cycles += 2;
            c.g[A0][0] = 0x120;
            found = true;
            break;
        }
        if !found {
            // New node: EA_MemAllocNamed(0x210, name, 0, 0), linked at the head.
            c.cycles += 14;
            c.g[A0][0] = 0x210;
            c.g[A1][0] = sx(oc1044_at(c, A1, 0x7568));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x002d_935c)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V0][0] = c.lw(oc1044_at(c, GP, AVALANCHE_NODE_LIST));
            c.sw(c.g[S0][0] as u32, c.g[S2][0]);
            c.sw(oc1044_at(c, S0, 0x8), c.g[V0][0]);
            c.sh(oc1044_at(c, S0, 0xc), 0);
            c.sh(oc1044_at(c, S0, 0xe), 0);
            c.sw(oc1044_at(c, S0, 0x4), 0);
            c.sw(oc1044_at(c, GP, AVALANCHE_NODE_LIST), c.g[S0][0]);
            c.g[A0][0] = 0x120;
        }

        // The new record: EA_MemAllocNamed(0x120, name, 0, 0).
        c.cycles += 33;
        c.g[A1][0] = sx(oc1044_at(c, S3, 0x7578));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x002d_9390)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[AT][0] = 0x41f0_0000; // 30.0
        c.mtc1(1, 3);
        c.g[V0][0] = c.lw(oc1044_at(c, S0, 0x4));
        c.g[V1][0] = sx(oc1044_at(c, A0, 0x110));
        c.mul_s(6, 20, 3);
        c.sw(oc1044_at(c, A0, 0xf4), c.g[V0][0]);
        c.mul_s(5, 22, 3);
        c.sw(oc1044_at(c, S0, 0x4), c.g[A0][0]);
        c.mul_s(3, 23, 3);
        c.swc1(21, oc1044_at(c, A0, 0x10c));
        c.lwc1(2, oc1044_at(c, S1, 0x60));
        c.lwc1(0, oc1044_at(c, S1, 0x6c));
        c.lwc1(1, oc1044_at(c, S1, 0x70));
        c.lwc1(4, oc1044_at(c, S1, 0x64));
        c.sub_s(0, 0, 2);
        c.abs_s(7, 0);
        c.lwc1(2, oc1044_at(c, S1, 0x74));
        c.sub_s(1, 1, 4);
        c.lwc1(0, oc1044_at(c, S1, 0x68));
        c.abs_s(1, 1);
        c.swc1(5, oc1044_at(c, A0, 0x100));
        c.c_lt_s(7, 1);
        c.swc1(3, oc1044_at(c, A0, 0x104));
        c.sub_s(2, 2, 0);
        c.swc1(6, oc1044_at(c, A0, 0x108));
        c.abs_s(0, 2);
        let dx_not_less_dy = !c.fcc();
        c.sw(oc1044_at(c, A0, 0xf8), c.g[S1][0]);

        // Pick the ratio of the extents by which axis is the largest:
        // 'x_over_y: |dx| < |dy|; each arm ends loading f2/f0 for the ratio.
        #[derive(PartialEq)]
        enum Next {
            LoadX,  // b_2d9430
            RatioZ, // b_2d9470
        }
        let next;
        if dx_not_less_dy {
            c.cycles += 4;
            c.c_lt_s(1, 0);
            if !c.fcc() {
                c.lwc1(0, oc1044_at(c, S1, 0x68));
                next = Next::RatioZ;
            } else {
                c.cycles += 1;
                c.lwc1(2, oc1044_at(c, S1, 0x60));
                next = Next::LoadX;
            }
        } else {
            c.cycles += 4;
            c.c_lt_s(7, 0);
            if c.fcc() {
                c.lwc1(2, oc1044_at(c, S1, 0x60));
                next = Next::LoadX;
            } else {
                c.cycles += 2;
                c.lwc1(0, oc1044_at(c, S1, 0x68));
                next = Next::RatioZ;
            }
        }
        if next == Next::LoadX {
            // b_2d9430 / b_2d9464
            c.cycles += 10;
            c.lwc1(1, oc1044_at(c, S1, 0x6c));
            c.lwc1(3, oc1044_at(c, S1, 0x64));
            c.lwc1(0, oc1044_at(c, S1, 0x70));
            c.sub_s(1, 1, 2);
            c.abs_s(2, 1);
            c.sub_s(0, 0, 3);
            c.abs_s(1, 0);
            c.c_lt_s(2, 1);
            c.cycles += 3;
            if !c.fcc() {
                c.lwc1(0, oc1044_back(c, GP, 0x3cc4));
                c.mul_s(1, 1, 0);
            } else {
                c.lwc1(0, oc1044_back(c, GP, 0x3cc8));
                c.mul_s(1, 2, 0);
            }
        } else {
            c.cycles += 5;
            c.lwc1(1, oc1044_at(c, S1, 0x74));
            c.lwc1(2, oc1044_back(c, GP, 0x3cc0));
            c.sub_s(1, 1, 0);
            c.abs_s(1, 1);
            c.mul_s(1, 1, 2);
        }

        // Store the ratio, clear fields, mode halfword from `kind`.
        c.cycles += 5;
        c.swc1(1, c.g[V1][0] as u32);
        c.g[V1][0] = 0x2;
        c.sw(oc1044_at(c, A0, 0xfc), 0);
        let kind_is_2 = c.g[S4][0] == c.g[V1][0];
        c.sb(oc1044_at(c, A0, 0xf3), 0);
        if kind_is_2 {
            c.cycles += 2;
            c.sh(oc1044_at(c, A0, 0xf0), 0);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x5;
            let kind_not_5 = c.g[S4][0] != c.g[V0][0];
            c.g[V0][0] = 0x1;
            if kind_not_5 {
                c.cycles += 1;
                c.sh(oc1044_at(c, A0, 0xf0), c.g[V0][0]);
            } else {
                c.cycles += 2;
                c.sh(oc1044_at(c, A0, 0xf0), c.g[V1][0]);
            }
        }

        // Register the record in the global table and bump the counter.
        c.cycles += 32;
        c.g[A1][0] = c.lw(oc1044_at(c, GP, AVALANCHE_REC_COUNT));
        c.g[V1][0] = sx(oc1044_back(c, S5, 0x7bb0));
        c.sb(oc1044_at(c, A0, 0xf2), 0);
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
        c.sw(oc1044_at(c, A0, 0x114), 0);
        c.g[A2][0] = c.lw(oc1044_at(c, A0, 0xf8));
        c.g[V0][0] = sx(oc1044_add32(c, V0, V1));
        c.sw(c.g[V0][0] as u32, c.g[A0][0]);
        c.g[V1][0] = sx((c.g[A1][0] as u32) << 8);
        c.g[V1][0] |= 0x2;
        c.g[A1][0] = sx(oc1044_at(c, A1, 0x1));
        c.g[V0][0] = c.lbu(oc1044_at(c, A2, 0x92));
        frame.restore(c, 16, 0x60);
        c.sh(oc1044_at(c, A2, 0x92), c.g[V0][0]);
        frame.restore(c, 17, 0x50);
        c.g[A0][0] = c.lw(oc1044_at(c, A0, 0xf8));
        frame.restore(c, 18, 0x40);
        c.g[V0][0] = c.lhu(oc1044_at(c, A0, 0x92));
        frame.restore(c, 19, 0x30);
        c.g[V0][0] |= c.g[V1][0];
        frame.restore(c, 20, 0x20);
        c.sh(oc1044_at(c, A0, 0x92), c.g[V0][0]);
        frame.restore(c, 21, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 23, 0x88);
        frame.restore_fpr(c, 22, 0x80);
        frame.restore_fpr(c, 21, 0x78);
        frame.restore_fpr(c, 20, 0x70);
        c.sw(oc1044_at(c, GP, AVALANCHE_REC_COUNT), c.g[A1][0]);
        c.sw(oc1044_at(c, GP, AVALANCHE_DIRTY), 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, machine_rt};
    use crate::difftest::{check, elf_image, Rng, DATA};

    /// Write a random float (edge cases or a plain range) at `a`.
    fn oc1044_rand_f32(c: &mut Cpu, r: &mut Rng, a: u32) {
        let v = if r.next_u32() % 5 == 0 {
            r.edge_f32()
        } else {
            r.f32(100.0)
        };
        c.bus.write32(a, v);
    }

    /// Debug-draw fixture: world with a render-state stack, a node whose
    /// instance holds `count` (0..3) line records, the vtable draw method a
    /// fake. Seeds also reach: no instance, debug flag off, count <= 0,
    /// zero-length axes (bytes 6..8 all 0).
    fn oc1044_debug_draw_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x0c10_4401, 1);
        elf_image(c);
        let node = DATA + 0x1000;
        let obj = DATA + 0x2000;
        let recs = DATA + 0x2800;
        let world = DATA + 0x3000;
        let stack = DATA + 0x3400;
        // The draw object: halfword `this` delta at +0x248, method at +0x24c.
        let draw = DATA + 0x3800;
        let delta = (r.next_u32() as i16 as i32 & !3) as u32;
        c.bus.write16(draw + 0x248, delta as u16);
        c.bus.write32(draw + 0x24c, fakes::FAKE);
        c.g[A0][0] = sx(node);
        c.bus
            .write32(node + AVNODE_OBJ, if seed % 8 == 0 { 0 } else { obj });
        let gp = c.g[GP][0] as u32;
        c.bus.write32(gp.wrapping_sub(AVRS_GLOBAL), world);
        c.bus.write32(world + 0xe84, stack);
        c.bus.write32(world + 0x10d8, draw);
        for k in 0..0x30 / 4 {
            let v = r.next_u32();
            c.bus.write32(stack + 4 * k, v);
        }
        c.bus.write32(
            gp + AVDEBUG_FLAG,
            if seed % 8 == 1 { 0 } else { r.next_u32() | 1 },
        );
        let count: i32 = match seed % 5 {
            0 => 0,
            1 => -1,
            n => n as i32 - 1, // 1, 2, 3
        };
        c.bus.write32(obj + AVOBJ_COUNT, (count as f32).to_bits());
        c.bus.write32(obj + AVOBJ_RECS, recs);
        for k in 0..4 {
            let v = r.next_u32();
            c.bus.write32(obj + AVOBJ_POS + 4 * k, v);
        }
        for k in 0..8 {
            let mut v = r.next_u32();
            if seed % 3 == 0 && k >= 1 {
                v &= 0x0000_ffff; // zero some bytes (zero-length axes)
            }
            c.bus.write32(recs + 4 * k, v);
        }
        // 1/127 and 2*pi/256-ish constants at gp-0x3cd0 / gp-0x3ccc.
        c.bus
            .write32(gp.wrapping_sub(0x3cd0), 0.007_874_016_f32.to_bits());
        c.bus
            .write32(gp.wrapping_sub(0x3ccc), 0.008_213_315_f32.to_bits());
        // the 0x4ff1a0 matrix and the 0x4ff130 row
        for k in 0..16 {
            let a = 0x4ff1a0 + 4 * k;
            oc1044_rand_f32(c, &mut r, a);
        }
        for k in 0..4 {
            oc1044_rand_f32(c, &mut r, 0x4ff130 + 4 * k);
        }
    }

    /// `Avalanche_DebugDrawInstance` against the generated body.
    #[test]
    fn avalanche_debug_draw_instance_matches() {
        fakes::covered(
            ee::s09::f_002d8a00__generated,
            avalanche_debug_draw_instance,
            200,
            &oc1044_debug_draw_setup,
        );
    }

    /// `Avalanche_AddNode`: node found first / second, absent, empty list;
    /// all `kind` values; every ordering of the instance's extents.
    #[test]
    fn avalanche_add_node_matches() {
        check(
            ee::s09::f_002d92d0__generated,
            avalanche_add_node,
            300,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x0c10_4402, 0);
                elf_image(c);
                let gp = c.g[GP][0] as u32;
                let id = 0x40 + seed % 5;
                let inst = DATA + 0x1000;
                let n1 = DATA + 0x2000;
                let n2 = DATA + 0x2100;
                c.g[A0][0] = sx(id);
                c.g[A1][0] = sx(inst);
                c.g[A2][0] = [2u64, 5, 1, 3][(seed / 3 % 4) as usize];
                for f in 12..16 {
                    c.f[f] = if r.next_u32() % 4 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(50.0)
                    };
                }
                for k in 0..8 {
                    oc1044_rand_f32(c, &mut r, inst + 0x60 + 4 * k);
                }
                c.bus.write32(inst + 0x90, r.next_u32());
                match seed % 4 {
                    0 => c.bus.write32(gp + AVALANCHE_NODE_LIST, 0),
                    1 => {
                        // first node matches
                        c.bus.write32(gp + AVALANCHE_NODE_LIST, n1);
                        c.bus.write32(n1, id);
                        c.bus.write32(n1 + 8, n2);
                        c.bus.write32(n1 + 4, DATA + 0x2200);
                    }
                    2 => {
                        // second node matches
                        c.bus.write32(gp + AVALANCHE_NODE_LIST, n1);
                        c.bus.write32(n1, id + 1);
                        c.bus.write32(n1 + 8, n2);
                        c.bus.write32(n2, id);
                        c.bus.write32(n2 + 8, 0);
                    }
                    _ => {
                        // no match: allocate a new node
                        c.bus.write32(gp + AVALANCHE_NODE_LIST, n1);
                        c.bus.write32(n1, id + 1);
                        c.bus.write32(n1 + 8, 0);
                    }
                }
                c.bus.write32(gp + AVALANCHE_REC_COUNT, seed % 40);
                c.bus.write32(gp + AVALANCHE_DIRTY, r.next_u32());
                for off in [0x3cc8u32, 0x3cc4, 0x3cc0] {
                    oc1044_rand_f32(c, &mut r, gp.wrapping_sub(off));
                }
            },
        );
    }
}
