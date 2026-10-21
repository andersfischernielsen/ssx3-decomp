//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements, each with a differential test against the generated body.
//!
//! Both functions in this module are asm-only in the reference
//! decompilation, so their replacements are block-for-block translations of
//! the generated bodies: the same FPU/VU0 macro ops in the same order, the
//! same memory accesses, and the same cycle charges. Address arithmetic is
//! routed through [`at`]/[`gp`]; `Emitter_ComputeDescBounds` has no calls,
//! and `ChaseCameraAlgorithm_ApplyAirSwing` makes every call through the
//! function table with [`call_ptr`], as the original's `jalr` does.
//!
//! - **Emitters**: `Emitter_ComputeDescBounds` derives an emitter
//!   descriptor's world-space bounding box.
//! - **Camera**: `ChaseCameraAlgorithm_ApplyAirSwing` advances the air-swing
//!   camera impulse and the cubic spline that shapes it.

use crate::abi::r32;
use crate::call::call_ptr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0036_d500,
        emitter_compute_desc_bounds,
        "Emitter_ComputeDescBounds",
    ),
];

/// `reg + off` (32-bit address arithmetic).
#[inline(always)]
fn at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `void Emitter_ComputeDescBounds(const Desc* d, Bounds* out)` (0x36d500):
/// derive the emitter descriptor's world-space bounding box (min at `out+0`,
/// max at `out+0x10`) and its extent from the descriptor's vectors at
/// `+0x60`..`+0xef`, folding in the reference scale/offset vectors at
/// `gp-0x2814`.. and the existing `out` values.
pub fn emitter_compute_desc_bounds(c: &mut Cpu) {
    'b_36d52c: {
        c.cycles += 10;
        c.lwc1(0, at(c, A0, 0x14));
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0xb0));
        c.lwc1(1, at(c, A0, 0x1c));
        c.add_s(2, 0, 0);
        c.add_s(0, 1, 0);
        c.add_s(1, 1, 2);
        c.c_lt_s(1, 0);
        let t = !c.fcc();
        c.mov_s(3, 1);
        if t {
            break 'b_36d52c;
        } else {
            c.cycles += 1;
            c.mov_s(3, 0);
            break 'b_36d52c;
        }
    }
    'b_36d5d0: {
        c.cycles += 41;
        c.lqc2(3, at(c, A0, 0xb0));
        c.lqc2(4, at(c, A0, 0xc0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x20));
        c.g[AT][0] = 0x40000000;
        c.mtc1(1, 2);
        c.lq(2, at(c, SP, 0x20));
        c.sq(2, at(c, SP, 0x10));
        c.lqc2(3, at(c, SP, 0x10));
        c.lqc2(4, at(c, A0, 0xd0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x20));
        c.lq(2, at(c, SP, 0x20));
        c.sq(2, (c.g[SP][0] as u32));
        c.sq(2, at(c, A1, 0x10));
        c.sq(2, (c.g[A1][0] as u32));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xc0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x30));
        c.lq(2, at(c, SP, 0x30));
        c.sq(2, at(c, SP, 0x20));
        c.lqc2(3, at(c, A0, 0xb0));
        c.lqc2(4, at(c, SP, 0x20));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x30));
        c.lq(2, at(c, SP, 0x30));
        c.sq(2, at(c, SP, 0x10));
        c.lqc2(3, at(c, SP, 0x10));
        c.lqc2(4, at(c, A0, 0xd0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x30));
        c.lq(2, at(c, SP, 0x30));
        c.lwc1(0, (c.g[A1][0] as u32));
        c.sq(2, (c.g[SP][0] as u32));
        c.lwc1(1, (c.g[SP][0] as u32));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, (c.g[A1][0] as u32));
            break 'b_36d5d0;
        } else {
            break 'b_36d5d0;
        }
    }
    'b_36d5e8: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x4));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x4));
            break 'b_36d5e8;
        } else {
            break 'b_36d5e8;
        }
    }
    'b_36d600: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x8));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x8));
            break 'b_36d600;
        } else {
            break 'b_36d600;
        }
    }
    'b_36d618: {
        c.cycles += 6;
        c.lwc1(1, (c.g[SP][0] as u32));
        c.lwc1(0, at(c, A1, 0x10));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x10));
            break 'b_36d618;
        } else {
            break 'b_36d618;
        }
    }
    'b_36d630: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x14));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x14));
            break 'b_36d630;
        } else {
            break 'b_36d630;
        }
    }
    'b_36d648: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x18));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x18));
            break 'b_36d648;
        } else {
            break 'b_36d648;
        }
    }
    'b_36d6ac: {
        c.cycles += 25;
        c.lqc2(3, at(c, A0, 0xb0));
        c.lqc2(4, at(c, A0, 0xc0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x20));
        c.lq(2, at(c, SP, 0x20));
        c.sq(2, at(c, SP, 0x10));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xd0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x30));
        c.lq(2, at(c, SP, 0x30));
        c.sq(2, at(c, SP, 0x20));
        c.lqc2(3, at(c, SP, 0x10));
        c.lqc2(4, at(c, SP, 0x20));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x30));
        c.lq(2, at(c, SP, 0x30));
        c.lwc1(0, (c.g[A1][0] as u32));
        c.sq(2, (c.g[SP][0] as u32));
        c.lwc1(1, (c.g[SP][0] as u32));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, (c.g[A1][0] as u32));
            break 'b_36d6ac;
        } else {
            break 'b_36d6ac;
        }
    }
    'b_36d6c4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x4));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x4));
            break 'b_36d6c4;
        } else {
            break 'b_36d6c4;
        }
    }
    'b_36d6dc: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x8));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x8));
            break 'b_36d6dc;
        } else {
            break 'b_36d6dc;
        }
    }
    'b_36d6f4: {
        c.cycles += 6;
        c.lwc1(1, (c.g[SP][0] as u32));
        c.lwc1(0, at(c, A1, 0x10));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x10));
            break 'b_36d6f4;
        } else {
            break 'b_36d6f4;
        }
    }
    'b_36d70c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x14));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x14));
            break 'b_36d70c;
        } else {
            break 'b_36d70c;
        }
    }
    'b_36d724: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x18));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x18));
            break 'b_36d724;
        } else {
            break 'b_36d724;
        }
    }
    'b_36d7a4: {
        c.cycles += 32;
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xc0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x30));
        c.lq(2, at(c, SP, 0x30));
        c.sq(2, at(c, SP, 0x20));
        c.lqc2(3, at(c, A0, 0xb0));
        c.lqc2(4, at(c, SP, 0x20));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x30));
        c.lq(2, at(c, SP, 0x30));
        c.sq(2, at(c, SP, 0x10));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xd0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x40));
        c.lq(2, at(c, SP, 0x40));
        c.sq(2, at(c, SP, 0x30));
        c.lqc2(3, at(c, SP, 0x10));
        c.lqc2(4, at(c, SP, 0x30));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x40));
        c.lq(2, at(c, SP, 0x40));
        c.lwc1(0, (c.g[A1][0] as u32));
        c.sq(2, (c.g[SP][0] as u32));
        c.lwc1(1, (c.g[SP][0] as u32));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, (c.g[A1][0] as u32));
            break 'b_36d7a4;
        } else {
            break 'b_36d7a4;
        }
    }
    'b_36d7bc: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x4));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x4));
            break 'b_36d7bc;
        } else {
            break 'b_36d7bc;
        }
    }
    'b_36d7d4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x8));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x8));
            break 'b_36d7d4;
        } else {
            break 'b_36d7d4;
        }
    }
    'b_36d7ec: {
        c.cycles += 6;
        c.lwc1(1, (c.g[SP][0] as u32));
        c.lwc1(0, at(c, A1, 0x10));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x10));
            break 'b_36d7ec;
        } else {
            break 'b_36d7ec;
        }
    }
    'b_36d804: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x14));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x14));
            break 'b_36d804;
        } else {
            break 'b_36d804;
        }
    }
    'b_36d81c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x18));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x18));
            break 'b_36d81c;
        } else {
            break 'b_36d81c;
        }
    }
    'b_36d8bc: {
        c.cycles += 40;
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xc0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x40));
        c.lq(2, at(c, SP, 0x40));
        c.sq(2, at(c, SP, 0x30));
        c.lqc2(3, at(c, A0, 0xb0));
        c.lqc2(4, at(c, SP, 0x30));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x40));
        c.lq(2, at(c, SP, 0x40));
        c.sq(2, at(c, SP, 0x20));
        c.lqc2(3, at(c, SP, 0x20));
        c.lqc2(4, at(c, A0, 0xd0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x40));
        c.lq(2, at(c, SP, 0x40));
        c.lwc1(0, (c.g[A0][0] as u32));
        c.cvt_s_w(0, 0);
        c.sq(2, at(c, SP, 0x10));
        c.mfc1(3, 0);
        c.lqc2(4, at(c, A0, 0xe0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x50));
        c.lq(2, at(c, SP, 0x50));
        c.sq(2, at(c, SP, 0x40));
        c.lqc2(3, at(c, SP, 0x10));
        c.lqc2(4, at(c, SP, 0x40));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x50));
        c.lq(2, at(c, SP, 0x50));
        c.lwc1(0, (c.g[A1][0] as u32));
        c.sq(2, (c.g[SP][0] as u32));
        c.lwc1(1, (c.g[SP][0] as u32));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, (c.g[A1][0] as u32));
            break 'b_36d8bc;
        } else {
            break 'b_36d8bc;
        }
    }
    'b_36d8d4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x4));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x4));
            break 'b_36d8d4;
        } else {
            break 'b_36d8d4;
        }
    }
    'b_36d8ec: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x8));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x8));
            break 'b_36d8ec;
        } else {
            break 'b_36d8ec;
        }
    }
    'b_36d904: {
        c.cycles += 6;
        c.lwc1(1, (c.g[SP][0] as u32));
        c.lwc1(0, at(c, A1, 0x10));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x10));
            break 'b_36d904;
        } else {
            break 'b_36d904;
        }
    }
    'b_36d91c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x14));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x14));
            break 'b_36d91c;
        } else {
            break 'b_36d91c;
        }
    }
    'b_36d934: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x18));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x18));
            break 'b_36d934;
        } else {
            break 'b_36d934;
        }
    }
    'b_36d9d4: {
        c.cycles += 40;
        c.lqc2(3, at(c, A0, 0xb0));
        c.lqc2(4, at(c, A0, 0xc0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x30));
        c.lq(2, at(c, SP, 0x30));
        c.sq(2, at(c, SP, 0x20));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xd0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x40));
        c.lq(2, at(c, SP, 0x40));
        c.sq(2, at(c, SP, 0x30));
        c.lqc2(3, at(c, SP, 0x20));
        c.lqc2(4, at(c, SP, 0x30));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x40));
        c.lq(2, at(c, SP, 0x40));
        c.lwc1(0, (c.g[A0][0] as u32));
        c.cvt_s_w(0, 0);
        c.sq(2, at(c, SP, 0x10));
        c.mfc1(3, 0);
        c.lqc2(4, at(c, A0, 0xe0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x50));
        c.lq(2, at(c, SP, 0x50));
        c.sq(2, at(c, SP, 0x40));
        c.lqc2(3, at(c, SP, 0x10));
        c.lqc2(4, at(c, SP, 0x40));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x50));
        c.lq(2, at(c, SP, 0x50));
        c.lwc1(0, (c.g[A1][0] as u32));
        c.sq(2, (c.g[SP][0] as u32));
        c.lwc1(1, (c.g[SP][0] as u32));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, (c.g[A1][0] as u32));
            break 'b_36d9d4;
        } else {
            break 'b_36d9d4;
        }
    }
    'b_36d9ec: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x4));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x4));
            break 'b_36d9ec;
        } else {
            break 'b_36d9ec;
        }
    }
    'b_36da04: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x8));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x8));
            break 'b_36da04;
        } else {
            break 'b_36da04;
        }
    }
    'b_36da1c: {
        c.cycles += 6;
        c.lwc1(1, (c.g[SP][0] as u32));
        c.lwc1(0, at(c, A1, 0x10));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x10));
            break 'b_36da1c;
        } else {
            break 'b_36da1c;
        }
    }
    'b_36da34: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x14));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x14));
            break 'b_36da34;
        } else {
            break 'b_36da34;
        }
    }
    'b_36da4c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x18));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x18));
            break 'b_36da4c;
        } else {
            break 'b_36da4c;
        }
    }
    'b_36db08: {
        c.cycles += 47;
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xc0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x40));
        c.lq(2, at(c, SP, 0x40));
        c.sq(2, at(c, SP, 0x30));
        c.lqc2(3, at(c, A0, 0xb0));
        c.lqc2(4, at(c, SP, 0x30));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x40));
        c.lq(2, at(c, SP, 0x40));
        c.sq(2, at(c, SP, 0x20));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xd0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x50));
        c.lq(2, at(c, SP, 0x50));
        c.sq(2, at(c, SP, 0x40));
        c.lqc2(3, at(c, SP, 0x20));
        c.lqc2(4, at(c, SP, 0x40));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x50));
        c.lq(2, at(c, SP, 0x50));
        c.lwc1(0, (c.g[A0][0] as u32));
        c.cvt_s_w(0, 0);
        c.sq(2, at(c, SP, 0x10));
        c.mfc1(3, 0);
        c.lqc2(4, at(c, A0, 0xe0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x60));
        c.lq(2, at(c, SP, 0x60));
        c.sq(2, at(c, SP, 0x50));
        c.lqc2(3, at(c, SP, 0x10));
        c.lqc2(4, at(c, SP, 0x50));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x60));
        c.lq(2, at(c, SP, 0x60));
        c.sq(2, (c.g[SP][0] as u32));
        c.lwc1(1, (c.g[SP][0] as u32));
        c.lwc1(0, (c.g[A1][0] as u32));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, (c.g[A1][0] as u32));
            break 'b_36db08;
        } else {
            break 'b_36db08;
        }
    }
    'b_36db20: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x4));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x4));
            break 'b_36db20;
        } else {
            break 'b_36db20;
        }
    }
    'b_36db38: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x8));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x8));
            break 'b_36db38;
        } else {
            break 'b_36db38;
        }
    }
    'b_36db50: {
        c.cycles += 6;
        c.lwc1(1, (c.g[SP][0] as u32));
        c.lwc1(0, at(c, A1, 0x10));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x10));
            break 'b_36db50;
        } else {
            break 'b_36db50;
        }
    }
    'b_36db68: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x14));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x14));
            break 'b_36db68;
        } else {
            break 'b_36db68;
        }
    }
    'b_36db80: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x18));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x18));
            break 'b_36db80;
        } else {
            break 'b_36db80;
        }
    }
    'b_36dc5c: {
        c.cycles += 55;
        c.lq(3, at(c, A1, 0x10));
        c.lq(2, (c.g[A1][0] as u32));
        c.sq(3, at(c, SP, 0x10));
        c.sq(2, (c.g[SP][0] as u32));
        c.lqc2(3, at(c, A0, 0x70));
        c.lqc2(4, at(c, A0, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x70));
        c.lq(2, at(c, SP, 0x70));
        c.sq(2, at(c, SP, 0x60));
        c.lqc2(3, at(c, SP, 0x60));
        c.lqc2(4, at(c, A0, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x70));
        c.lq(2, at(c, SP, 0x70));
        c.sq(2, at(c, SP, 0x50));
        c.lqc2(3, at(c, SP, 0x50));
        c.lqc2(4, at(c, A0, 0xa0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x70));
        c.lq(2, at(c, SP, 0x70));
        c.sq(2, at(c, SP, 0x20));
        c.sq(2, at(c, SP, 0x40));
        c.sq(2, at(c, SP, 0x30));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0x80));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x70));
        c.lqc2(3, at(c, A0, 0x70));
        c.lqc2(4, at(c, SP, 0x70));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x60));
        c.lqc2(3, at(c, SP, 0x60));
        c.lqc2(4, at(c, A0, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x50));
        c.lqc2(3, at(c, SP, 0x50));
        c.lqc2(4, at(c, A0, 0xa0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.lwc1(0, at(c, SP, 0x20));
        c.sq(2, at(c, SP, 0x40));
        c.lwc1(1, at(c, SP, 0x40));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x20));
            break 'b_36dc5c;
        } else {
            break 'b_36dc5c;
        }
    }
    'b_36dc74: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x24));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x24));
            break 'b_36dc74;
        } else {
            break 'b_36dc74;
        }
    }
    'b_36dc8c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x28));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x28));
            break 'b_36dc8c;
        } else {
            break 'b_36dc8c;
        }
    }
    'b_36dca4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x40));
        c.lwc1(0, at(c, SP, 0x30));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x30));
            break 'b_36dca4;
        } else {
            break 'b_36dca4;
        }
    }
    'b_36dcbc: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x34));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x34));
            break 'b_36dcbc;
        } else {
            break 'b_36dcbc;
        }
    }
    'b_36dcd4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x38));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x38));
            break 'b_36dcd4;
        } else {
            break 'b_36dcd4;
        }
    }
    'b_36dd50: {
        c.cycles += 31;
        c.lqc2(3, at(c, A0, 0x70));
        c.lqc2(4, at(c, A0, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x70));
        c.lq(2, at(c, SP, 0x70));
        c.sq(2, at(c, SP, 0x60));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0x90));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x70));
        c.lqc2(3, at(c, SP, 0x60));
        c.lqc2(4, at(c, SP, 0x70));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x50));
        c.lqc2(3, at(c, SP, 0x50));
        c.lqc2(4, at(c, A0, 0xa0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.lwc1(0, at(c, SP, 0x20));
        c.sq(2, at(c, SP, 0x40));
        c.lwc1(1, at(c, SP, 0x40));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x20));
            break 'b_36dd50;
        } else {
            break 'b_36dd50;
        }
    }
    'b_36dd68: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x24));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x24));
            break 'b_36dd68;
        } else {
            break 'b_36dd68;
        }
    }
    'b_36dd80: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x28));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x28));
            break 'b_36dd80;
        } else {
            break 'b_36dd80;
        }
    }
    'b_36dd98: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x40));
        c.lwc1(0, at(c, SP, 0x30));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x30));
            break 'b_36dd98;
        } else {
            break 'b_36dd98;
        }
    }
    'b_36ddb0: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x34));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x34));
            break 'b_36ddb0;
        } else {
            break 'b_36ddb0;
        }
    }
    'b_36ddc8: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x38));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x38));
            break 'b_36ddc8;
        } else {
            break 'b_36ddc8;
        }
    }
    'b_36de44: {
        c.cycles += 31;
        c.lqc2(3, at(c, A0, 0x70));
        c.lqc2(4, at(c, A0, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x70));
        c.lq(2, at(c, SP, 0x70));
        c.sq(2, at(c, SP, 0x60));
        c.lqc2(3, at(c, SP, 0x60));
        c.lqc2(4, at(c, A0, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x70));
        c.lq(2, at(c, SP, 0x70));
        c.sq(2, at(c, SP, 0x50));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xa0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x70));
        c.lqc2(3, at(c, SP, 0x50));
        c.lqc2(4, at(c, SP, 0x70));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.lwc1(0, at(c, SP, 0x20));
        c.sq(2, at(c, SP, 0x40));
        c.lwc1(1, at(c, SP, 0x40));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x20));
            break 'b_36de44;
        } else {
            break 'b_36de44;
        }
    }
    'b_36de5c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x24));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x24));
            break 'b_36de5c;
        } else {
            break 'b_36de5c;
        }
    }
    'b_36de74: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x28));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x28));
            break 'b_36de74;
        } else {
            break 'b_36de74;
        }
    }
    'b_36de8c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x40));
        c.lwc1(0, at(c, SP, 0x30));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x30));
            break 'b_36de8c;
        } else {
            break 'b_36de8c;
        }
    }
    'b_36dea4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x34));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x34));
            break 'b_36dea4;
        } else {
            break 'b_36dea4;
        }
    }
    'b_36debc: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x38));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x38));
            break 'b_36debc;
        } else {
            break 'b_36debc;
        }
    }
    'b_36df54: {
        c.cycles += 38;
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0x80));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x70));
        c.lqc2(3, at(c, A0, 0x70));
        c.lqc2(4, at(c, SP, 0x70));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x60));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0x90));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x80));
        c.lqc2(3, at(c, SP, 0x60));
        c.lqc2(4, at(c, SP, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x50));
        c.lqc2(3, at(c, SP, 0x50));
        c.lqc2(4, at(c, A0, 0xa0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.lwc1(0, at(c, SP, 0x20));
        c.sq(2, at(c, SP, 0x40));
        c.lwc1(1, at(c, SP, 0x40));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x20));
            break 'b_36df54;
        } else {
            break 'b_36df54;
        }
    }
    'b_36df6c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x24));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x24));
            break 'b_36df6c;
        } else {
            break 'b_36df6c;
        }
    }
    'b_36df84: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x28));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x28));
            break 'b_36df84;
        } else {
            break 'b_36df84;
        }
    }
    'b_36df9c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x40));
        c.lwc1(0, at(c, SP, 0x30));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x30));
            break 'b_36df9c;
        } else {
            break 'b_36df9c;
        }
    }
    'b_36dfb4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x34));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x34));
            break 'b_36dfb4;
        } else {
            break 'b_36dfb4;
        }
    }
    'b_36dfcc: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x38));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x38));
            break 'b_36dfcc;
        } else {
            break 'b_36dfcc;
        }
    }
    'b_36e064: {
        c.cycles += 38;
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0x80));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x70));
        c.lqc2(3, at(c, A0, 0x70));
        c.lqc2(4, at(c, SP, 0x70));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x60));
        c.lqc2(3, at(c, SP, 0x60));
        c.lqc2(4, at(c, A0, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x50));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xa0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x80));
        c.lqc2(3, at(c, SP, 0x50));
        c.lqc2(4, at(c, SP, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.lwc1(0, at(c, SP, 0x20));
        c.sq(2, at(c, SP, 0x40));
        c.lwc1(1, at(c, SP, 0x40));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x20));
            break 'b_36e064;
        } else {
            break 'b_36e064;
        }
    }
    'b_36e07c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x24));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x24));
            break 'b_36e07c;
        } else {
            break 'b_36e07c;
        }
    }
    'b_36e094: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x28));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x28));
            break 'b_36e094;
        } else {
            break 'b_36e094;
        }
    }
    'b_36e0ac: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x40));
        c.lwc1(0, at(c, SP, 0x30));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x30));
            break 'b_36e0ac;
        } else {
            break 'b_36e0ac;
        }
    }
    'b_36e0c4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x34));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x34));
            break 'b_36e0c4;
        } else {
            break 'b_36e0c4;
        }
    }
    'b_36e0dc: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x38));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x38));
            break 'b_36e0dc;
        } else {
            break 'b_36e0dc;
        }
    }
    'b_36e174: {
        c.cycles += 38;
        c.lqc2(3, at(c, A0, 0x70));
        c.lqc2(4, at(c, A0, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x70));
        c.lq(2, at(c, SP, 0x70));
        c.sq(2, at(c, SP, 0x60));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0x90));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x70));
        c.lqc2(3, at(c, SP, 0x60));
        c.lqc2(4, at(c, SP, 0x70));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x50));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xa0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x80));
        c.lqc2(3, at(c, SP, 0x50));
        c.lqc2(4, at(c, SP, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.lwc1(0, at(c, SP, 0x20));
        c.sq(2, at(c, SP, 0x40));
        c.lwc1(1, at(c, SP, 0x40));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x20));
            break 'b_36e174;
        } else {
            break 'b_36e174;
        }
    }
    'b_36e18c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x24));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x24));
            break 'b_36e18c;
        } else {
            break 'b_36e18c;
        }
    }
    'b_36e1a4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x28));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x28));
            break 'b_36e1a4;
        } else {
            break 'b_36e1a4;
        }
    }
    'b_36e1bc: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x40));
        c.lwc1(0, at(c, SP, 0x30));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x30));
            break 'b_36e1bc;
        } else {
            break 'b_36e1bc;
        }
    }
    'b_36e1d4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x34));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x34));
            break 'b_36e1d4;
        } else {
            break 'b_36e1d4;
        }
    }
    'b_36e1ec: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x38));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x38));
            break 'b_36e1ec;
        } else {
            break 'b_36e1ec;
        }
    }
    'b_36e2a0: {
        c.cycles += 45;
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0x80));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x70));
        c.lqc2(3, at(c, A0, 0x70));
        c.lqc2(4, at(c, SP, 0x70));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x60));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0x90));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x80));
        c.lqc2(3, at(c, SP, 0x60));
        c.lqc2(4, at(c, SP, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x50));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xa0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0xa0));
        c.lq(2, at(c, SP, 0xa0));
        c.sq(2, at(c, SP, 0x90));
        c.lqc2(3, at(c, SP, 0x50));
        c.lqc2(4, at(c, SP, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0xa0));
        c.lq(2, at(c, SP, 0xa0));
        c.sq(2, at(c, SP, 0x40));
        c.lwc1(1, at(c, SP, 0x40));
        c.lwc1(0, at(c, SP, 0x20));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x20));
            break 'b_36e2a0;
        } else {
            break 'b_36e2a0;
        }
    }
    'b_36e2b8: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x24));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x24));
            break 'b_36e2b8;
        } else {
            break 'b_36e2b8;
        }
    }
    'b_36e2d0: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x28));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x28));
            break 'b_36e2d0;
        } else {
            break 'b_36e2d0;
        }
    }
    'b_36e2e8: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x40));
        c.lwc1(0, at(c, SP, 0x30));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x30));
            break 'b_36e2e8;
        } else {
            break 'b_36e2e8;
        }
    }
    'b_36e300: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x34));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x34));
            break 'b_36e300;
        } else {
            break 'b_36e300;
        }
    }
    'b_36e318: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x38));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x38));
            break 'b_36e318;
        } else {
            break 'b_36e318;
        }
    }
    'b_36e3e4: {
        c.cycles += 51;
        c.lqc2(3, at(c, A0, 0x60));
        c.lqc2(4, at(c, SP, 0x30));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x50));
        c.lwc1(1, gp(c, -0x2868));
        c.lq(2, at(c, SP, 0x50));
        c.lwc1(0, gp(c, -0x2864));
        c.min_s(1, 3, 1);
        c.sq(2, at(c, SP, 0x40));
        c.lqc2(3, at(c, A0, 0x60));
        c.lqc2(4, at(c, SP, 0x20));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x60));
        c.mul_s(0, 1, 0);
        c.lq(2, at(c, SP, 0x60));
        c.lwc1(2, gp(c, -0x2860));
        c.sq(2, at(c, SP, 0x50));
        c.mfc1(3, 3);
        c.lqc2(4, at(c, A0, 0x60));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x90));
        c.add_s(0, 0, 2);
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x80));
        c.mul_s(1, 1, 0);
        c.lqc2(3, (c.g[SP][0] as u32));
        c.lqc2(4, at(c, SP, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x70));
        c.mfc1(3, 1);
        c.lqc2(4, at(c, SP, 0x40));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0xa0));
        c.lq(2, at(c, SP, 0xa0));
        c.sq(2, at(c, SP, 0x90));
        c.lqc2(3, at(c, SP, 0x70));
        c.lqc2(4, at(c, SP, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0xa0));
        c.lq(2, at(c, SP, 0xa0));
        c.sq(2, at(c, SP, 0x60));
        c.lwc1(2, at(c, SP, 0x60));
        c.lwc1(0, (c.g[A1][0] as u32));
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.swc1(2, (c.g[A1][0] as u32));
            break 'b_36e3e4;
        } else {
            break 'b_36e3e4;
        }
    }
    'b_36e3fc: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x64));
        c.lwc1(0, at(c, A1, 0x4));
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x4));
            break 'b_36e3fc;
        } else {
            break 'b_36e3fc;
        }
    }
    'b_36e414: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x68));
        c.lwc1(0, at(c, A1, 0x8));
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x8));
            break 'b_36e414;
        } else {
            break 'b_36e414;
        }
    }
    'b_36e42c: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x60));
        c.lwc1(0, at(c, A1, 0x10));
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x10));
            break 'b_36e42c;
        } else {
            break 'b_36e42c;
        }
    }
    'b_36e444: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x64));
        c.lwc1(0, at(c, A1, 0x14));
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x14));
            break 'b_36e444;
        } else {
            break 'b_36e444;
        }
    }
    'b_36e45c: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x68));
        c.lwc1(0, at(c, A1, 0x18));
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x18));
            break 'b_36e45c;
        } else {
            break 'b_36e45c;
        }
    }
    'b_36e4dc: {
        c.cycles += 32;
        c.mfc1(3, 3);
        c.lqc2(4, at(c, A0, 0x60));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x80));
        c.lqc2(3, (c.g[SP][0] as u32));
        c.lqc2(4, at(c, SP, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x70));
        c.mfc1(3, 1);
        c.lqc2(4, at(c, SP, 0x50));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0xa0));
        c.lq(2, at(c, SP, 0xa0));
        c.sq(2, at(c, SP, 0x90));
        c.lqc2(3, at(c, SP, 0x70));
        c.lqc2(4, at(c, SP, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0xa0));
        c.lq(2, at(c, SP, 0xa0));
        c.lwc1(0, (c.g[A1][0] as u32));
        c.sq(2, at(c, SP, 0x60));
        c.lwc1(2, at(c, SP, 0x60));
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.swc1(2, (c.g[A1][0] as u32));
            break 'b_36e4dc;
        } else {
            break 'b_36e4dc;
        }
    }
    'b_36e4f4: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x64));
        c.lwc1(0, at(c, A1, 0x4));
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x4));
            break 'b_36e4f4;
        } else {
            break 'b_36e4f4;
        }
    }
    'b_36e50c: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x68));
        c.lwc1(0, at(c, A1, 0x8));
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x8));
            break 'b_36e50c;
        } else {
            break 'b_36e50c;
        }
    }
    'b_36e524: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x60));
        c.lwc1(0, at(c, A1, 0x10));
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x10));
            break 'b_36e524;
        } else {
            break 'b_36e524;
        }
    }
    'b_36e53c: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x64));
        c.lwc1(0, at(c, A1, 0x14));
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x14));
            break 'b_36e53c;
        } else {
            break 'b_36e53c;
        }
    }
    'b_36e554: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x68));
        c.lwc1(0, at(c, A1, 0x18));
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x18));
            break 'b_36e554;
        } else {
            break 'b_36e554;
        }
    }
    'b_36e5d4: {
        c.cycles += 32;
        c.mfc1(3, 3);
        c.lqc2(4, at(c, A0, 0x60));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x80));
        c.lqc2(3, at(c, SP, 0x10));
        c.lqc2(4, at(c, SP, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x70));
        c.mfc1(3, 1);
        c.lqc2(4, at(c, SP, 0x40));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0xa0));
        c.lq(2, at(c, SP, 0xa0));
        c.sq(2, at(c, SP, 0x90));
        c.lqc2(3, at(c, SP, 0x70));
        c.lqc2(4, at(c, SP, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0xa0));
        c.lq(2, at(c, SP, 0xa0));
        c.lwc1(0, (c.g[A1][0] as u32));
        c.sq(2, at(c, SP, 0x60));
        c.lwc1(2, at(c, SP, 0x60));
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.swc1(2, (c.g[A1][0] as u32));
            break 'b_36e5d4;
        } else {
            break 'b_36e5d4;
        }
    }
    'b_36e5ec: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x64));
        c.lwc1(0, at(c, A1, 0x4));
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x4));
            break 'b_36e5ec;
        } else {
            break 'b_36e5ec;
        }
    }
    'b_36e604: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x68));
        c.lwc1(0, at(c, A1, 0x8));
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x8));
            break 'b_36e604;
        } else {
            break 'b_36e604;
        }
    }
    'b_36e61c: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x60));
        c.lwc1(0, at(c, A1, 0x10));
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x10));
            break 'b_36e61c;
        } else {
            break 'b_36e61c;
        }
    }
    'b_36e634: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x64));
        c.lwc1(0, at(c, A1, 0x14));
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x14));
            break 'b_36e634;
        } else {
            break 'b_36e634;
        }
    }
    'b_36e64c: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x68));
        c.lwc1(0, at(c, A1, 0x18));
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x18));
            break 'b_36e64c;
        } else {
            break 'b_36e64c;
        }
    }
    'b_36e6cc: {
        c.cycles += 32;
        c.mfc1(3, 3);
        c.lqc2(4, at(c, A0, 0x60));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x80));
        c.lqc2(3, at(c, SP, 0x10));
        c.lqc2(4, at(c, SP, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x70));
        c.mfc1(3, 1);
        c.lqc2(4, at(c, SP, 0x50));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0xa0));
        c.lq(2, at(c, SP, 0xa0));
        c.sq(2, at(c, SP, 0x90));
        c.lqc2(3, at(c, SP, 0x70));
        c.lqc2(4, at(c, SP, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0xa0));
        c.lq(2, at(c, SP, 0xa0));
        c.lwc1(0, (c.g[A1][0] as u32));
        c.sq(2, at(c, SP, 0x60));
        c.lwc1(1, at(c, SP, 0x60));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, (c.g[A1][0] as u32));
            break 'b_36e6cc;
        } else {
            break 'b_36e6cc;
        }
    }
    'b_36e6e4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x64));
        c.lwc1(0, at(c, A1, 0x4));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x4));
            break 'b_36e6e4;
        } else {
            break 'b_36e6e4;
        }
    }
    'b_36e6fc: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x68));
        c.lwc1(0, at(c, A1, 0x8));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x8));
            break 'b_36e6fc;
        } else {
            break 'b_36e6fc;
        }
    }
    'b_36e714: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x60));
        c.lwc1(0, at(c, A1, 0x10));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x10));
            break 'b_36e714;
        } else {
            break 'b_36e714;
        }
    }
    'b_36e72c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x64));
        c.lwc1(0, at(c, A1, 0x14));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x14));
            break 'b_36e72c;
        } else {
            break 'b_36e72c;
        }
    }
    'b_36e744: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x68));
        c.lwc1(0, at(c, A1, 0x18));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x18));
            break 'b_36e744;
        } else {
            break 'b_36e744;
        }
    }
    'b_36ebd4: {
        c.cycles += 38;
        c.lwc1(3, at(c, A0, 0x60));
        c.lwc1(1, at(c, SP, 0x20));
        c.lwc1(4, at(c, A0, 0x64));
        c.sub_s(1, 1, 3);
        c.lwc1(5, at(c, A0, 0x68));
        c.lwc1(2, at(c, SP, 0x34));
        c.lwc1(0, at(c, SP, 0x38));
        c.div_s(1, 3, 1);
        c.lwc1(7, at(c, SP, 0x30));
        c.lwc1(9, at(c, SP, 0x24));
        c.sub_s(2, 2, 4);
        c.lwc1(8, at(c, SP, 0x28));
        c.sub_s(0, 0, 5);
        c.sub_s(7, 7, 3);
        c.lwc1(10, gp(c, -0x285c));
        c.sub_s(9, 9, 4);
        c.lwc1(6, gp(c, -0x2858));
        c.sub_s(8, 8, 5);
        c.div_s(2, 4, 2);
        c.div_s(0, 5, 0);
        c.div_s(3, 3, 7);
        c.div_s(4, 4, 9);
        c.div_s(5, 5, 8);
        c.sub_s(1, 1, 10);
        c.mtc1(0, 7);
        c.sub_s(2, 2, 10);
        c.mul_s(8, 1, 6);
        c.sub_s(3, 3, 10);
        c.sub_s(4, 4, 10);
        c.sub_s(5, 5, 10);
        c.sub_s(0, 0, 10);
        c.c_lt_s(7, 8);
        c.mul_s(3, 3, 6);
        c.mul_s(9, 0, 6);
        c.mul_s(4, 4, 6);
        c.mul_s(7, 2, 6);
        let t = !c.fcc();
        c.mul_s(5, 5, 6);
        if t {
            break 'b_36ebd4;
        } else {
            'b_36e874: {
                c.cycles += 38;
                c.lwc1(0, gp(c, -0x2854));
                c.lwc1(1, gp(c, -0x2850));
                c.min_s(8, 8, 0);
                c.mfc1(2, 8);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.mul_s(1, 8, 1);
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.add_s(1, 1, 10);
                c.lqc2(3, (c.g[SP][0] as u32));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.mul_s(2, 8, 1);
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36e874;
                } else {
                    break 'b_36e874;
                }
            }
            'b_36e88c: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36e88c;
                } else {
                    break 'b_36e88c;
                }
            }
            'b_36e8a4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36e8a4;
                } else {
                    break 'b_36e8a4;
                }
            }
            'b_36e8bc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36e8bc;
                } else {
                    break 'b_36e8bc;
                }
            }
            'b_36e8d4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36e8d4;
                } else {
                    break 'b_36e8d4;
                }
            }
            'b_36e8ec: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36e8ec;
                } else {
                    break 'b_36e8ec;
                }
            }
            'b_36e96c: {
                c.cycles += 32;
                c.mfc1(3, 8);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, (c.g[SP][0] as u32));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36e96c;
                } else {
                    break 'b_36e96c;
                }
            }
            'b_36e984: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36e984;
                } else {
                    break 'b_36e984;
                }
            }
            'b_36e99c: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36e99c;
                } else {
                    break 'b_36e99c;
                }
            }
            'b_36e9b4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36e9b4;
                } else {
                    break 'b_36e9b4;
                }
            }
            'b_36e9cc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36e9cc;
                } else {
                    break 'b_36e9cc;
                }
            }
            'b_36e9e4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36e9e4;
                } else {
                    break 'b_36e9e4;
                }
            }
            'b_36ea64: {
                c.cycles += 32;
                c.mfc1(3, 8);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36ea64;
                } else {
                    break 'b_36ea64;
                }
            }
            'b_36ea7c: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36ea7c;
                } else {
                    break 'b_36ea7c;
                }
            }
            'b_36ea94: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36ea94;
                } else {
                    break 'b_36ea94;
                }
            }
            'b_36eaac: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36eaac;
                } else {
                    break 'b_36eaac;
                }
            }
            'b_36eac4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36eac4;
                } else {
                    break 'b_36eac4;
                }
            }
            'b_36eadc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36eadc;
                } else {
                    break 'b_36eadc;
                }
            }
            'b_36eb5c: {
                c.cycles += 32;
                c.mfc1(3, 8);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36eb5c;
                } else {
                    break 'b_36eb5c;
                }
            }
            'b_36eb74: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36eb74;
                } else {
                    break 'b_36eb74;
                }
            }
            'b_36eb8c: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36eb8c;
                } else {
                    break 'b_36eb8c;
                }
            }
            'b_36eba4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36eba4;
                } else {
                    break 'b_36eba4;
                }
            }
            'b_36ebbc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36ebbc;
                } else {
                    break 'b_36ebbc;
                }
            }
            c.cycles += 6;
            c.lwc1(1, at(c, SP, 0x68));
            c.lwc1(0, at(c, A1, 0x18));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, at(c, A1, 0x18));
                break 'b_36ebd4;
            } else {
                break 'b_36ebd4;
            }
        }
    }
    'b_36efe4: {
        c.cycles += 4;
        c.mtc1(0, 0);
        c.c_lt_s(0, 3);
        let t = !c.fcc();

        if t {
            break 'b_36efe4;
        } else {
            'b_36ec80: {
                c.cycles += 39;
                c.lwc1(0, gp(c, -0x284c));
                c.lwc1(1, gp(c, -0x2848));
                c.min_s(3, 3, 0);
                c.lwc1(2, gp(c, -0x2844));
                c.mfc1(2, 3);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.mul_s(1, 3, 1);
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.add_s(1, 1, 2);
                c.lqc2(3, (c.g[SP][0] as u32));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.mul_s(2, 3, 1);
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36ec80;
                } else {
                    break 'b_36ec80;
                }
            }
            'b_36ec98: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36ec98;
                } else {
                    break 'b_36ec98;
                }
            }
            'b_36ecb0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36ecb0;
                } else {
                    break 'b_36ecb0;
                }
            }
            'b_36ecc8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36ecc8;
                } else {
                    break 'b_36ecc8;
                }
            }
            'b_36ece0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36ece0;
                } else {
                    break 'b_36ece0;
                }
            }
            'b_36ecf8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36ecf8;
                } else {
                    break 'b_36ecf8;
                }
            }
            'b_36ed78: {
                c.cycles += 32;
                c.mfc1(3, 3);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, (c.g[SP][0] as u32));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36ed78;
                } else {
                    break 'b_36ed78;
                }
            }
            'b_36ed90: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36ed90;
                } else {
                    break 'b_36ed90;
                }
            }
            'b_36eda8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36eda8;
                } else {
                    break 'b_36eda8;
                }
            }
            'b_36edc0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36edc0;
                } else {
                    break 'b_36edc0;
                }
            }
            'b_36edd8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36edd8;
                } else {
                    break 'b_36edd8;
                }
            }
            'b_36edf0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36edf0;
                } else {
                    break 'b_36edf0;
                }
            }
            'b_36ee70: {
                c.cycles += 32;
                c.mfc1(3, 3);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36ee70;
                } else {
                    break 'b_36ee70;
                }
            }
            'b_36ee88: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36ee88;
                } else {
                    break 'b_36ee88;
                }
            }
            'b_36eea0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36eea0;
                } else {
                    break 'b_36eea0;
                }
            }
            'b_36eeb8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36eeb8;
                } else {
                    break 'b_36eeb8;
                }
            }
            'b_36eed0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36eed0;
                } else {
                    break 'b_36eed0;
                }
            }
            'b_36eee8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36eee8;
                } else {
                    break 'b_36eee8;
                }
            }
            'b_36ef68: {
                c.cycles += 32;
                c.mfc1(3, 3);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36ef68;
                } else {
                    break 'b_36ef68;
                }
            }
            'b_36ef80: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36ef80;
                } else {
                    break 'b_36ef80;
                }
            }
            'b_36ef98: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36ef98;
                } else {
                    break 'b_36ef98;
                }
            }
            'b_36efb0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36efb0;
                } else {
                    break 'b_36efb0;
                }
            }
            'b_36efc8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36efc8;
                } else {
                    break 'b_36efc8;
                }
            }
            'b_36efe0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36efe0;
                } else {
                    break 'b_36efe0;
                }
            }
            c.cycles += 1;
            c.mtc1(0, 0);
            break 'b_36efe4;
        }
    }
    'b_36f3ec: {
        c.cycles += 3;
        c.c_lt_s(0, 4);
        let t = !c.fcc();

        if t {
            break 'b_36f3ec;
        } else {
            'b_36f08c: {
                c.cycles += 39;
                c.lwc1(0, gp(c, -0x2840));
                c.lwc1(1, gp(c, -0x283c));
                c.min_s(4, 4, 0);
                c.lwc1(2, gp(c, -0x2838));
                c.mfc1(2, 4);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.mul_s(1, 4, 1);
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.add_s(1, 1, 2);
                c.lqc2(3, (c.g[SP][0] as u32));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.mul_s(2, 4, 1);
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36f08c;
                } else {
                    break 'b_36f08c;
                }
            }
            'b_36f0a4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36f0a4;
                } else {
                    break 'b_36f0a4;
                }
            }
            'b_36f0bc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36f0bc;
                } else {
                    break 'b_36f0bc;
                }
            }
            'b_36f0d4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36f0d4;
                } else {
                    break 'b_36f0d4;
                }
            }
            'b_36f0ec: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36f0ec;
                } else {
                    break 'b_36f0ec;
                }
            }
            'b_36f104: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36f104;
                } else {
                    break 'b_36f104;
                }
            }
            'b_36f184: {
                c.cycles += 32;
                c.mfc1(3, 4);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, (c.g[SP][0] as u32));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36f184;
                } else {
                    break 'b_36f184;
                }
            }
            'b_36f19c: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36f19c;
                } else {
                    break 'b_36f19c;
                }
            }
            'b_36f1b4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36f1b4;
                } else {
                    break 'b_36f1b4;
                }
            }
            'b_36f1cc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36f1cc;
                } else {
                    break 'b_36f1cc;
                }
            }
            'b_36f1e4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36f1e4;
                } else {
                    break 'b_36f1e4;
                }
            }
            'b_36f1fc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36f1fc;
                } else {
                    break 'b_36f1fc;
                }
            }
            'b_36f27c: {
                c.cycles += 32;
                c.mfc1(3, 4);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36f27c;
                } else {
                    break 'b_36f27c;
                }
            }
            'b_36f294: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36f294;
                } else {
                    break 'b_36f294;
                }
            }
            'b_36f2ac: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36f2ac;
                } else {
                    break 'b_36f2ac;
                }
            }
            'b_36f2c4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36f2c4;
                } else {
                    break 'b_36f2c4;
                }
            }
            'b_36f2dc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36f2dc;
                } else {
                    break 'b_36f2dc;
                }
            }
            'b_36f2f4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36f2f4;
                } else {
                    break 'b_36f2f4;
                }
            }
            'b_36f374: {
                c.cycles += 32;
                c.mfc1(3, 4);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36f374;
                } else {
                    break 'b_36f374;
                }
            }
            'b_36f38c: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36f38c;
                } else {
                    break 'b_36f38c;
                }
            }
            'b_36f3a4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36f3a4;
                } else {
                    break 'b_36f3a4;
                }
            }
            'b_36f3bc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36f3bc;
                } else {
                    break 'b_36f3bc;
                }
            }
            'b_36f3d4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36f3d4;
                } else {
                    break 'b_36f3d4;
                }
            }
            c.cycles += 6;
            c.lwc1(1, at(c, SP, 0x68));
            c.lwc1(0, at(c, A1, 0x18));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, at(c, A1, 0x18));
                break 'b_36f3ec;
            } else {
                break 'b_36f3ec;
            }
        }
    }
    'b_36f7fc: {
        c.cycles += 4;
        c.mtc1(0, 0);
        c.c_lt_s(0, 7);
        let t = !c.fcc();

        if t {
            break 'b_36f7fc;
        } else {
            c.cycles += 17;
            c.lwc1(0, gp(c, -0x2834));
            c.lwc1(1, gp(c, -0x2830));
            c.min_s(7, 7, 0);
            c.lwc1(2, gp(c, -0x282c));
            c.mfc1(2, 7);
            c.lqc2(4, at(c, A0, 0x60));
            c.qmtc2(2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, at(c, SP, 0x90));
            c.mul_s(1, 7, 1);
            c.lq(2, at(c, SP, 0x90));
            c.sq(2, at(c, SP, 0x80));
            c.add_s(1, 1, 2);
            c.lqc2(3, (c.g[SP][0] as u32));
            c.lqc2(4, at(c, SP, 0x80));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, at(c, SP, 0x90));
            break 'b_36f7fc;
        }
    }
    'b_36fc04: {
        c.cycles += 3;
        c.c_lt_s(0, 5);
        let t = !c.fcc();

        if t {
            break 'b_36fc04;
        } else {
            'b_36f8a4: {
                c.cycles += 39;
                c.lwc1(0, gp(c, -0x2828));
                c.lwc1(1, gp(c, -0x2824));
                c.min_s(5, 5, 0);
                c.lwc1(2, gp(c, -0x2820));
                c.mfc1(2, 5);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.mul_s(1, 5, 1);
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.add_s(1, 1, 2);
                c.lqc2(3, (c.g[SP][0] as u32));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.mul_s(2, 5, 1);
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36f8a4;
                } else {
                    break 'b_36f8a4;
                }
            }
            'b_36f8bc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36f8bc;
                } else {
                    break 'b_36f8bc;
                }
            }
            'b_36f8d4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36f8d4;
                } else {
                    break 'b_36f8d4;
                }
            }
            'b_36f8ec: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36f8ec;
                } else {
                    break 'b_36f8ec;
                }
            }
            'b_36f904: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36f904;
                } else {
                    break 'b_36f904;
                }
            }
            'b_36f91c: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36f91c;
                } else {
                    break 'b_36f91c;
                }
            }
            'b_36f99c: {
                c.cycles += 32;
                c.mfc1(3, 5);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, (c.g[SP][0] as u32));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36f99c;
                } else {
                    break 'b_36f99c;
                }
            }
            'b_36f9b4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36f9b4;
                } else {
                    break 'b_36f9b4;
                }
            }
            'b_36f9cc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36f9cc;
                } else {
                    break 'b_36f9cc;
                }
            }
            'b_36f9e4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36f9e4;
                } else {
                    break 'b_36f9e4;
                }
            }
            'b_36f9fc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36f9fc;
                } else {
                    break 'b_36f9fc;
                }
            }
            'b_36fa14: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36fa14;
                } else {
                    break 'b_36fa14;
                }
            }
            'b_36fa94: {
                c.cycles += 32;
                c.mfc1(3, 5);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36fa94;
                } else {
                    break 'b_36fa94;
                }
            }
            'b_36faac: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36faac;
                } else {
                    break 'b_36faac;
                }
            }
            'b_36fac4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36fac4;
                } else {
                    break 'b_36fac4;
                }
            }
            'b_36fadc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36fadc;
                } else {
                    break 'b_36fadc;
                }
            }
            'b_36faf4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36faf4;
                } else {
                    break 'b_36faf4;
                }
            }
            'b_36fb0c: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36fb0c;
                } else {
                    break 'b_36fb0c;
                }
            }
            'b_36fb8c: {
                c.cycles += 32;
                c.mfc1(3, 5);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36fb8c;
                } else {
                    break 'b_36fb8c;
                }
            }
            'b_36fba4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36fba4;
                } else {
                    break 'b_36fba4;
                }
            }
            'b_36fbbc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36fbbc;
                } else {
                    break 'b_36fbbc;
                }
            }
            'b_36fbd4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36fbd4;
                } else {
                    break 'b_36fbd4;
                }
            }
            'b_36fbec: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36fbec;
                } else {
                    break 'b_36fbec;
                }
            }
            c.cycles += 6;
            c.lwc1(1, at(c, SP, 0x68));
            c.lwc1(0, at(c, A1, 0x18));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, at(c, A1, 0x18));
                break 'b_36fc04;
            } else {
                break 'b_36fc04;
            }
        }
    }
    'b_370010: {
        c.cycles += 4;
        c.mtc1(0, 0);
        c.c_lt_s(0, 9);
        let t = !c.fcc();

        if t {
            break 'b_370010;
        } else {
            'b_36fcb0: {
                c.cycles += 39;
                c.lwc1(0, gp(c, -0x281c));
                c.lwc1(1, gp(c, -0x2818));
                c.min_s(9, 9, 0);
                c.lwc1(2, gp(c, -0x2814));
                c.mfc1(2, 9);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.mul_s(1, 9, 1);
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.add_s(1, 1, 2);
                c.lqc2(3, (c.g[SP][0] as u32));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.mul_s(2, 9, 1);
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36fcb0;
                } else {
                    break 'b_36fcb0;
                }
            }
            'b_36fcc8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36fcc8;
                } else {
                    break 'b_36fcc8;
                }
            }
            'b_36fce0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36fce0;
                } else {
                    break 'b_36fce0;
                }
            }
            'b_36fcf8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36fcf8;
                } else {
                    break 'b_36fcf8;
                }
            }
            'b_36fd10: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36fd10;
                } else {
                    break 'b_36fd10;
                }
            }
            'b_36fd28: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36fd28;
                } else {
                    break 'b_36fd28;
                }
            }
            'b_36fda8: {
                c.cycles += 32;
                c.mfc1(3, 9);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, (c.g[SP][0] as u32));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36fda8;
                } else {
                    break 'b_36fda8;
                }
            }
            'b_36fdc0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36fdc0;
                } else {
                    break 'b_36fdc0;
                }
            }
            'b_36fdd8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36fdd8;
                } else {
                    break 'b_36fdd8;
                }
            }
            'b_36fdf0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36fdf0;
                } else {
                    break 'b_36fdf0;
                }
            }
            'b_36fe08: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36fe08;
                } else {
                    break 'b_36fe08;
                }
            }
            'b_36fe20: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36fe20;
                } else {
                    break 'b_36fe20;
                }
            }
            'b_36fea0: {
                c.cycles += 32;
                c.mfc1(3, 9);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36fea0;
                } else {
                    break 'b_36fea0;
                }
            }
            'b_36feb8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36feb8;
                } else {
                    break 'b_36feb8;
                }
            }
            'b_36fed0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36fed0;
                } else {
                    break 'b_36fed0;
                }
            }
            'b_36fee8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36fee8;
                } else {
                    break 'b_36fee8;
                }
            }
            'b_36ff00: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36ff00;
                } else {
                    break 'b_36ff00;
                }
            }
            'b_36ff18: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36ff18;
                } else {
                    break 'b_36ff18;
                }
            }
            'b_36ff98: {
                c.cycles += 32;
                c.mfc1(3, 9);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, (c.g[A1][0] as u32));
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, (c.g[A1][0] as u32));
                    break 'b_36ff98;
                } else {
                    break 'b_36ff98;
                }
            }
            'b_36ffb0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36ffb0;
                } else {
                    break 'b_36ffb0;
                }
            }
            'b_36ffc8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36ffc8;
                } else {
                    break 'b_36ffc8;
                }
            }
            'b_36ffe0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36ffe0;
                } else {
                    break 'b_36ffe0;
                }
            }
            'b_36fff8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36fff8;
                } else {
                    break 'b_36fff8;
                }
            }
            c.cycles += 6;
            c.lwc1(1, at(c, SP, 0x68));
            c.lwc1(0, at(c, A1, 0x18));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, at(c, A1, 0x18));
                break 'b_370010;
            } else {
                break 'b_370010;
            }
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0xb0));
    c.pc = t;
    return;
}

/// `void ChaseCameraAlgorithm_ApplyAirSwing(ChaseCameraAlgorithm* self,
/// Info* info, float amount /*f12*/)` (0x164878): the air-swing camera
/// impulse. Sets the swing flag (`gp-0x1e64`), advances the swing phase
/// (`+0x22c`/`+0x234`), drives the spline at `+0x334` (init, nine points,
/// evaluate) and rotates/offsets the camera's target vectors (`+0x110`,
/// `+0x120`), the smoothing state (`+0x130`..) and the orbit vectors.
/// Virtual calls go through the view object at `+0x30`.
pub fn chase_camera_algorithm_apply_air_swing(c: &mut Cpu) {
    'b_16531c: {
        'b_165318: {
            'b_16518c: {
                c.cycles += 21;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xf0));
                c.sw(gp(c, -0x1e64), 0u64);
                c.sq(17, at(c, SP, 0xc0));
                c.sq(18, at(c, SP, 0xb0));
                c.g[S1][0] = c.g[A0][0];
                c.swc1(20, at(c, SP, 0xe0));
                c.g[S2][0] = c.g[A1][0];
                c.sq(16, at(c, SP, 0xd0));
                c.mov_s(20, 12);
                c.sd(at(c, SP, 0xa0), c.g[RA][0]);
                c.swc1(21, at(c, SP, 0xe8));
                c.lqc2(3, at(c, S1, 0x20));
                c.lqc2(4, at(c, S1, 0x40));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x10));
                c.lq(2, at(c, SP, 0x10));
                c.lq(3, at(c, S1, 0x40));
                c.g[A0][0] = c.lw(at(c, S1, 0x2d0));
                c.sq(2, (c.g[SP][0] as u32));
                let t = c.g[A0][0] == 0u64;
                c.sq(3, at(c, SP, 0x10));
                if t {
                    break 'b_16518c;
                } else {
                    'b_164fb0: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, S1, 0x2d8));
                        if c.g[V0][0] != 0u64 {
                            c.lwc1(1, at(c, S1, 0x234));
                            break 'b_164fb0;
                        } else {
                            'b_164fa4: {
                                c.cycles += 12;
                                c.lwc1(0, at(c, S1, 0x22c));
                                c.lwc1(1, gp(c, -0x6920));
                                c.g[A1][0] = c.lw(at(c, S1, 0x30));
                                c.add_s(0, 0, 1);
                                c.swc1(0, at(c, S1, 0x22c));
                                c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                                c.g[A0][0] = c.lh(at(c, V1, 0x98));
                                c.g[V0][0] = c.lw(at(c, V1, 0x9c));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x00164900);
                                c.g[A0][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                if call_ptr(c, t, 0x00164900).is_err() {
                                    return;
                                }
                                if c.g[V0][0] == 0u64 {
                                    c.g[V0][0] = c.lw(at(c, S1, 0x2d8));
                                    break 'b_164fa4;
                                } else {
                                    'b_1649a4: {
                                        c.cycles += 31;
                                        c.lqc2(3, at(c, S1, 0x20));
                                        c.lqc2(4, at(c, S1, 0x40));
                                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                        c.sqc2(5, at(c, SP, 0x30));
                                        c.g[S0][0] = sx(at(c, S1, 0x230));
                                        c.lq(2, at(c, SP, 0x30));
                                        c.g[A1][0] = c.lw(at(c, S1, 0x30));
                                        c.sq(2, at(c, S1, 0x110));
                                        c.sq(2, at(c, SP, 0x20));
                                        c.lqc2(3, at(c, S1, 0x110));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.vu0().sqrt(4, 0);
                                        c.vu0().waitq();
                                        c.cfc2(3, 22);
                                        c.mtc1(3, 21);
                                        c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                                        c.g[A0][0] = c.lh(at(c, V0, 0x60));
                                        c.g[V1][0] = c.lw(at(c, V0, 0x64));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x00164968);
                                        c.g[A0][0] =
                                            sx((c.g[A1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        if call_ptr(c, t, 0x00164968).is_err() {
                                            return;
                                        }
                                        c.lwc1(1, at(c, S1, 0x22c));
                                        c.lwc1(2, gp(c, -0x691c));
                                        c.sub_s(0, 0, 1);
                                        c.c_le_s(2, 0);
                                        if !c.fcc() {
                                            c.mov_s(0, 2);
                                            break 'b_1649a4;
                                        } else {
                                            c.cycles += 8;
                                            c.g[A1][0] = c.lw(at(c, S1, 0x30));
                                            c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                                            c.g[A0][0] = c.lh(at(c, V0, 0x60));
                                            c.g[V1][0] = c.lw(at(c, V0, 0x64));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x0016499c);
                                            c.g[A0][0] = sx((c.g[A1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            if call_ptr(c, t, 0x0016499c).is_err() {
                                                return;
                                            }
                                            c.lwc1(1, at(c, S1, 0x22c));
                                            c.sub_s(0, 0, 1);
                                            break 'b_1649a4;
                                        }
                                    }
                                    'b_164a88: {
                                        c.cycles += 50;
                                        c.swc1(0, (c.g[S0][0] as u32));
                                        c.swc1(20, at(c, S1, 0x23c));
                                        c.sw(at(c, S1, 0x234), 0u64);
                                        c.g[A1][0] = c.lw(at(c, S1, 0x30));
                                        c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                                        c.g[A0][0] = c.lh(at(c, V1, 0x80));
                                        c.g[V0][0] = c.lw(at(c, V1, 0x84));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x001649c8);
                                        c.g[A0][0] =
                                            sx((c.g[A1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        if call_ptr(c, t, 0x001649c8).is_err() {
                                            return;
                                        }
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A2][0] = 0x500000;
                                        c.lqc2(4, (c.g[A2][0] as u32).wrapping_sub(0xea0));
                                        c.lqc2(5, (c.g[A0][0] as u32));
                                        c.vu0().opmula(4, 5);
                                        c.vu0().opmsub(6, 5, 4);
                                        c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                                        c.sqc2(6, at(c, SP, 0x30));
                                        c.mtc1(0, 3);
                                        c.lq(2, at(c, SP, 0x30));
                                        c.g[A1][0] = c.g[A0][0];
                                        c.sq(2, at(c, SP, 0x20));
                                        c.lqc2(3, at(c, SP, 0x20));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.vu0().sqrt(4, 0);
                                        c.vu0().waitq();
                                        c.cfc2(3, 22);
                                        c.mtc1(3, 0);
                                        c.mfc1(2, 0);
                                        c.qmtc2(2, 3);
                                        c.vu0().div(0, 3, 3, 0);
                                        c.lqc2(4, at(c, SP, 0x20));
                                        c.vu0().waitq();
                                        c.vu0().mul(0b1111, 5, 4, Src::Q);
                                        c.sqc2(5, at(c, SP, 0x20));
                                        c.lqc2(3, at(c, SP, 0x20));
                                        c.lqc2(5, at(c, S2, 0x20));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(3, 4);
                                        c.mtc1(3, 1);
                                        c.c_lt_s(1, 3);
                                        let t = !c.fcc();

                                        if t {
                                            break 'b_164a88;
                                        } else {
                                            c.cycles += 7;
                                            c.g[AT][0] = 0xffffffffbf800000;
                                            c.mtc1(1, 0);
                                            c.mfc1(2, 0);
                                            c.lqc2(4, at(c, SP, 0x20));
                                            c.qmtc2(2, 3);
                                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                            c.sqc2(5, at(c, SP, 0x20));
                                            break 'b_164a88;
                                        }
                                    }
                                    'b_164bb4: {
                                        c.cycles += 74;
                                        c.lqc2(3, at(c, SP, 0x20));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.vu0().sqrt(4, 0);
                                        c.vu0().waitq();
                                        c.cfc2(2, 22);
                                        c.mtc1(2, 0);
                                        c.g[AT][0] = 0x40000000;
                                        c.mtc1(1, 2);
                                        c.div_s(0, 21, 0);
                                        c.mfc1(2, 0);
                                        c.lqc2(4, at(c, SP, 0x20));
                                        c.qmtc2(2, 3);
                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                        c.sqc2(5, at(c, SP, 0x20));
                                        c.lqc2(3, (c.g[A0][0] as u32));
                                        c.lqc2(5, (c.g[A0][0] as u32));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(2, 4);
                                        c.mtc1(2, 0);
                                        c.lqc2(3, at(c, S1, 0x110));
                                        c.lqc2(5, (c.g[A0][0] as u32));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(3, 4);
                                        c.mtc1(3, 1);
                                        c.div_s(1, 1, 0);
                                        c.mfc1(3, 1);
                                        c.lqc2(4, (c.g[A0][0] as u32));
                                        c.qmtc2(3, 3);
                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                        c.sqc2(5, at(c, SP, 0x60));
                                        c.lq(2, at(c, SP, 0x60));
                                        c.sq(2, at(c, SP, 0x50));
                                        c.mfc1(3, 2);
                                        c.lqc2(4, at(c, SP, 0x50));
                                        c.qmtc2(3, 3);
                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                        c.sqc2(5, at(c, SP, 0x60));
                                        c.lq(2, at(c, SP, 0x60));
                                        c.sq(2, at(c, SP, 0x40));
                                        c.lqc2(3, at(c, S1, 0x110));
                                        c.lqc2(4, at(c, SP, 0x40));
                                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                        c.sqc2(5, at(c, SP, 0x60));
                                        c.lq(2, at(c, SP, 0x60));
                                        c.sq(2, at(c, S1, 0x120));
                                        c.sq(2, at(c, SP, 0x30));
                                        c.lwc1(0, at(c, S1, 0x128));
                                        c.neg_s(0, 0);
                                        c.swc1(0, at(c, S1, 0x128));
                                        c.lqc2(3, at(c, S1, 0x120));
                                        c.lqc2(5, at(c, S1, 0x120));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(2, 4);
                                        c.mtc1(2, 1);
                                        c.c_le_s(1, 3);
                                        let t = !c.fcc();
                                        c.lq(2, at(c, SP, 0x20));
                                        if t {
                                            break 'b_164bb4;
                                        } else {
                                            c.cycles += 1;
                                            c.sq(2, at(c, S1, 0x120));
                                            break 'b_164bb4;
                                        }
                                    }
                                    'b_164d98: {
                                        c.cycles += 80;
                                        c.lwc1(0, gp(c, -0x6918));
                                        c.mfc1(2, 0);
                                        c.lqc2(4, at(c, S1, 0x120));
                                        c.qmtc2(2, 3);
                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                        c.sqc2(5, at(c, S1, 0x120));
                                        c.lwc1(1, gp(c, -0x6914));
                                        c.mfc1(2, 1);
                                        c.lqc2(4, at(c, SP, 0x20));
                                        c.qmtc2(2, 3);
                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                        c.sqc2(5, at(c, SP, 0x40));
                                        c.lwc1(2, gp(c, -0x6910));
                                        c.lq(2, at(c, SP, 0x40));
                                        c.sq(2, at(c, SP, 0x30));
                                        c.lqc2(3, at(c, S1, 0x120));
                                        c.lqc2(4, at(c, SP, 0x30));
                                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                        c.sqc2(5, at(c, S1, 0x120));
                                        c.lqc2(3, at(c, S1, 0x120));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.vu0().sqrt(4, 0);
                                        c.vu0().waitq();
                                        c.cfc2(2, 22);
                                        c.mtc1(2, 0);
                                        c.div_s(0, 21, 0);
                                        c.mfc1(2, 0);
                                        c.lqc2(4, at(c, S1, 0x120));
                                        c.qmtc2(2, 3);
                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                        c.sqc2(5, at(c, S1, 0x120));
                                        c.lqc2(3, (c.g[A1][0] as u32));
                                        c.lqc2(5, (c.g[A1][0] as u32));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(2, 4);
                                        c.mtc1(2, 0);
                                        c.lqc2(3, at(c, S1, 0x120));
                                        c.lqc2(5, (c.g[A1][0] as u32));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(3, 4);
                                        c.mtc1(3, 1);
                                        c.div_s(1, 1, 0);
                                        c.mfc1(3, 1);
                                        c.lqc2(4, (c.g[A1][0] as u32));
                                        c.qmtc2(3, 3);
                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                        c.sqc2(5, at(c, SP, 0x40));
                                        c.lq(2, at(c, SP, 0x40));
                                        c.sq(2, at(c, SP, 0x30));
                                        c.sq(2, at(c, S1, 0x150));
                                        c.lqc2(3, at(c, S1, 0x120));
                                        c.lqc2(4, at(c, S1, 0x150));
                                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                        c.sqc2(5, at(c, SP, 0x40));
                                        c.lq(2, at(c, SP, 0x40));
                                        c.sq(2, at(c, SP, 0x30));
                                        c.lqc2(3, at(c, SP, 0x30));
                                        c.lqc2(5, at(c, SP, 0x30));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(3, 4);
                                        c.mtc1(3, 0);
                                        c.c_lt_s(0, 2);
                                        let t = !c.fcc();

                                        if t {
                                            break 'b_164d98;
                                        } else {
                                            c.cycles += 16;
                                            c.swc1(3, at(c, SP, 0x38));
                                            c.g[AT][0] = 0x3f800000;
                                            c.mtc1(1, 0);
                                            c.lqc2(3, at(c, SP, 0x30));
                                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                            c.vu0().sqrt(4, 0);
                                            c.vu0().waitq();
                                            c.cfc2(2, 22);
                                            c.mtc1(2, 2);
                                            c.c_lt_s(0, 2);
                                            let t = !c.fcc();

                                            if t {
                                                break 'b_164d98;
                                            } else {
                                                c.cycles += 25;
                                                c.g[AT][0] = 0x43480000;
                                                c.mtc1(1, 0);
                                                c.g[AT][0] = 0x42c80000;
                                                c.mtc1(1, 1);
                                                c.div_s(0, 0, 2);
                                                c.mfc1(2, 0);
                                                c.lqc2(4, at(c, SP, 0x30));
                                                c.qmtc2(2, 3);
                                                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                                c.sqc2(5, at(c, SP, 0x30));
                                                c.lqc2(3, at(c, S1, 0x120));
                                                c.lqc2(4, at(c, SP, 0x30));
                                                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                                c.sqc2(5, at(c, S1, 0x120));
                                                c.mfc1(3, 1);
                                                c.lqc2(4, (c.g[A2][0] as u32).wrapping_sub(0xea0));
                                                c.qmtc2(3, 3);
                                                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                                c.sqc2(5, at(c, SP, 0x50));
                                                c.lq(2, at(c, SP, 0x50));
                                                c.sq(2, at(c, SP, 0x40));
                                                c.lqc2(3, at(c, S1, 0x120));
                                                c.lqc2(4, at(c, SP, 0x40));
                                                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                                c.sqc2(5, at(c, S1, 0x120));
                                                break 'b_164d98;
                                            }
                                        }
                                    }
                                    'b_164ec8: {
                                        'b_164e6c: {
                                            c.cycles += 50;
                                            c.lqc2(4, at(c, S1, 0x110));
                                            c.lqc2(5, at(c, S1, 0x120));
                                            c.vu0().opmula(4, 5);
                                            c.vu0().opmsub(6, 5, 4);
                                            c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                                            c.sqc2(6, at(c, SP, 0x50));
                                            c.lwc1(2, gp(c, -0x690c));
                                            c.lq(2, at(c, SP, 0x50));
                                            c.sq(2, at(c, S1, 0x130));
                                            c.sq(2, at(c, SP, 0x40));
                                            c.lqc2(3, at(c, S1, 0x130));
                                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                            c.vu0().sqrt(4, 0);
                                            c.vu0().waitq();
                                            c.cfc2(3, 22);
                                            c.mtc1(3, 0);
                                            c.mfc1(2, 0);
                                            c.qmtc2(2, 3);
                                            c.vu0().div(0, 3, 3, 0);
                                            c.lqc2(4, at(c, S1, 0x130));
                                            c.vu0().waitq();
                                            c.vu0().mul(0b1111, 5, 4, Src::Q);
                                            c.sqc2(5, at(c, S1, 0x130));
                                            c.lqc2(3, at(c, S1, 0x110));
                                            c.lqc2(5, at(c, S1, 0x110));
                                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                            c.qmfc2(3, 4);
                                            c.mtc1(3, 1);
                                            c.lqc2(3, at(c, S1, 0x120));
                                            c.lqc2(5, at(c, S1, 0x120));
                                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                            c.qmfc2(2, 4);
                                            c.mtc1(2, 0);
                                            c.mul_s(1, 1, 0);
                                            c.sqrt_s(1, 1);
                                            c.c_lt_s(1, 2);
                                            let t = !c.fcc();

                                            if t {
                                                break 'b_164e6c;
                                            } else {
                                                c.cycles += 3;
                                                c.mtc1(0, 1);
                                                let t = true;

                                                if t {
                                                    break 'b_164ec8;
                                                } else {
                                                    break 'b_164e6c;
                                                }
                                            }
                                        }
                                        'b_164eb8: {
                                            c.cycles += 16;
                                            c.lqc2(3, at(c, S1, 0x110));
                                            c.lqc2(5, at(c, S1, 0x120));
                                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                            c.qmfc2(2, 4);
                                            c.mtc1(2, 0);
                                            c.g[AT][0] = 0xffffffffbf800000;
                                            c.mtc1(1, 2);
                                            c.div_s(12, 0, 1);
                                            c.c_le_s(2, 12);
                                            if !c.fcc() {
                                                c.mov_s(12, 2);
                                                break 'b_164eb8;
                                            } else {
                                                c.cycles += 3;
                                                c.g[AT][0] = 0x3f800000;
                                                c.mtc1(1, 0);
                                                c.min_s(12, 12, 0);
                                                break 'b_164eb8;
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x00164ec0);
                                        if call_ptr(c, 0x0031c128, 0x00164ec0).is_err() {
                                            return;
                                        }
                                        c.lwc1(1, gp(c, -0x6908));
                                        c.sub_s(1, 1, 0);
                                        break 'b_164ec8;
                                    }
                                    c.cycles += 55;
                                    c.mtc1(0, 20);
                                    c.g[S0][0] = sx(at(c, S1, 0x334));
                                    c.swc1(1, at(c, S1, 0x238));
                                    c.g[A1][0] = 0x9;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.mov_s(12, 20);
                                    c.g[RA][0] = sx(0x00164ee8);
                                    c.mov_s(13, 20);
                                    if call_ptr(c, 0x0031d738, 0x00164ee8).is_err() {
                                        return;
                                    }
                                    c.mov_s(12, 20);
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x00164ef8);
                                    c.mov_s(13, 12);
                                    if call_ptr(c, 0x0031d7e0, 0x00164ef8).is_err() {
                                        return;
                                    }
                                    c.g[AT][0] = 0x3e000000;
                                    c.mtc1(1, 12);
                                    c.lwc1(13, gp(c, -0x6904));
                                    c.g[RA][0] = sx(0x00164f0c);
                                    c.g[A0][0] = c.g[S0][0];
                                    if call_ptr(c, 0x0031d7e0, 0x00164f0c).is_err() {
                                        return;
                                    }
                                    c.g[AT][0] = 0x3e800000;
                                    c.mtc1(1, 12);
                                    c.lwc1(13, gp(c, -0x6900));
                                    c.g[RA][0] = sx(0x00164f20);
                                    c.g[A0][0] = c.g[S0][0];
                                    if call_ptr(c, 0x0031d7e0, 0x00164f20).is_err() {
                                        return;
                                    }
                                    c.g[AT][0] = 0x3ec00000;
                                    c.mtc1(1, 12);
                                    c.lwc1(13, gp(c, -0x68fc));
                                    c.g[RA][0] = sx(0x00164f34);
                                    c.g[A0][0] = c.g[S0][0];
                                    if call_ptr(c, 0x0031d7e0, 0x00164f34).is_err() {
                                        return;
                                    }
                                    c.g[AT][0] = 0x3f000000;
                                    c.mtc1(1, 12);
                                    c.lwc1(13, gp(c, -0x68f8));
                                    c.g[RA][0] = sx(0x00164f48);
                                    c.g[A0][0] = c.g[S0][0];
                                    if call_ptr(c, 0x0031d7e0, 0x00164f48).is_err() {
                                        return;
                                    }
                                    c.g[AT][0] = 0x3f200000;
                                    c.mtc1(1, 12);
                                    c.lwc1(13, gp(c, -0x68f4));
                                    c.g[RA][0] = sx(0x00164f5c);
                                    c.g[A0][0] = c.g[S0][0];
                                    if call_ptr(c, 0x0031d7e0, 0x00164f5c).is_err() {
                                        return;
                                    }
                                    c.g[AT][0] = 0x3f400000;
                                    c.mtc1(1, 12);
                                    c.lwc1(13, gp(c, -0x68f0));
                                    c.g[RA][0] = sx(0x00164f70);
                                    c.g[A0][0] = c.g[S0][0];
                                    if call_ptr(c, 0x0031d7e0, 0x00164f70).is_err() {
                                        return;
                                    }
                                    c.g[AT][0] = 0x3f600000;
                                    c.mtc1(1, 12);
                                    c.lwc1(13, gp(c, -0x68ec));
                                    c.g[RA][0] = sx(0x00164f84);
                                    c.g[A0][0] = c.g[S0][0];
                                    if call_ptr(c, 0x0031d7e0, 0x00164f84).is_err() {
                                        return;
                                    }
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 12);
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x00164f98);
                                    c.mov_s(13, 12);
                                    if call_ptr(c, 0x0031d7e0, 0x00164f98).is_err() {
                                        return;
                                    }
                                    c.g[V0][0] = 0x1;
                                    c.sw(at(c, S1, 0x2d8), c.g[V0][0]);
                                    c.g[V0][0] = c.lw(at(c, S1, 0x2d8));
                                    break 'b_164fa4;
                                }
                            }
                            c.cycles += 2;
                            if c.g[V0][0] == 0u64 {
                                c.g[V0][0] = c.lw(at(c, S1, 0x2d0));
                                break 'b_16531c;
                            } else {
                                c.cycles += 1;
                                c.lwc1(1, at(c, S1, 0x234));
                                break 'b_164fb0;
                            }
                        }
                    }
                    'b_1650a4: {
                        c.cycles += 7;
                        c.lwc1(0, gp(c, -0x68e8));
                        c.lwc1(2, at(c, S1, 0x230));
                        c.add_s(1, 1, 0);
                        c.c_lt_s(1, 2);
                        let t = !c.fcc();
                        c.swc1(1, at(c, S1, 0x234));
                        if t {
                            break 'b_1650a4;
                        } else {
                            c.cycles += 54;
                            c.div_s(12, 1, 2);
                            c.g[V1][0] = sx(at(c, S1, 0x334));
                            c.g[V0][0] = 0x1;
                            c.g[A0][0] = c.g[V1][0];
                            c.sw(gp(c, -0x1e64), c.g[V0][0]);
                            c.g[RA][0] = sx(0x00164fe8);
                            c.sw(at(c, GP, 0x1cf0), c.g[V1][0]);
                            if call_ptr(c, 0x0031dee0, 0x00164fe8).is_err() {
                                return;
                            }
                            c.lwc1(12, at(c, S1, 0x238));
                            c.mov_s(20, 0);
                            c.g[AT][0] = 0x3f000000;
                            c.mtc1(1, 0);
                            c.g[A0][0] = sx(at(c, SP, 0x90));
                            c.mul_s(12, 20, 12);
                            c.g[A1][0] = sx(at(c, SP, 0x94));
                            c.g[RA][0] = sx(0x0016500c);
                            c.mul_s(12, 12, 0);
                            if call_ptr(c, 0x0031be50, 0x0016500c).is_err() {
                                return;
                            }
                            c.lwc1(1, at(c, S1, 0x130));
                            c.lwc1(2, at(c, S1, 0x134));
                            c.lwc1(0, at(c, SP, 0x90));
                            c.lwc1(4, at(c, S1, 0x138));
                            c.mul_s(1, 0, 1);
                            c.lwc1(3, at(c, SP, 0x94));
                            c.mul_s(2, 0, 2);
                            c.mul_s(0, 0, 4);
                            c.swc1(3, at(c, SP, 0x3c));
                            c.swc1(1, at(c, SP, 0x30));
                            c.swc1(2, at(c, SP, 0x34));
                            c.swc1(0, at(c, SP, 0x38));
                            c.lq(2, at(c, SP, 0x30));
                            c.sq(2, at(c, SP, 0x20));
                            c.lqc2(4, at(c, SP, 0x20));
                            c.lqc2(5, at(c, S1, 0x110));
                            c.vu0().sub(0b0001, 8, 8, Src::Vec(8));
                            c.vu0().opmula(4, 5);
                            c.vu0().opmsub(6, 5, 4);
                            c.vu0().opmula(4, 6);
                            c.vu0().opmsub(7, 6, 4);
                            c.vu0().mula(0b1110, 5, Src::Bc(0, W));
                            c.vu0().madda(0b1110, 6, Src::Bc(4, W));
                            c.vu0().madda(0b1110, 6, Src::Bc(4, W));
                            c.vu0().madda(0b1110, 7, Src::Bc(0, W));
                            c.vu0().madd(0b1110, 8, 7, Src::Bc(0, W));
                            c.sqc2(8, at(c, SP, 0x40));
                            c.lq(2, at(c, SP, 0x40));
                            c.sq(2, at(c, S1, 0x140));
                            c.sq(2, at(c, SP, 0x30));
                            c.mfc1(3, 20);
                            c.lqc2(4, at(c, S1, 0x150));
                            c.qmtc2(3, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, at(c, SP, 0x40));
                            c.lq(2, at(c, SP, 0x40));
                            c.sq(2, at(c, S1, 0x160));
                            c.sq(2, at(c, SP, 0x30));
                            break 'b_1650a4;
                        }
                    }
                    'b_1650f8: {
                        c.cycles += 21;
                        c.lwc1(1, gp(c, -0x68e4));
                        c.lwc1(0, at(c, S1, 0x230));
                        c.lwc1(2, at(c, S1, 0x234));
                        c.mul_s(0, 0, 1);
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 4);
                        c.mtc1(0, 1);
                        c.g[AT][0] = 0x40400000;
                        c.mtc1(1, 3);
                        c.div_s(2, 2, 0);
                        c.min_s(2, 2, 4);
                        c.max_s(2, 2, 1);
                        c.mul_s(0, 2, 2);
                        c.add_s(1, 0, 0);
                        c.mul_s(0, 0, 3);
                        c.mul_s(1, 1, 2);
                        c.sub_s(0, 0, 1);
                        c.c_lt_s(4, 0);
                        if c.fcc() {
                            c.mov_s(0, 4);
                            break 'b_1650f8;
                        } else {
                            break 'b_1650f8;
                        }
                    }
                    c.cycles += 37;
                    c.mov_s(2, 0);
                    c.sub_s(0, 4, 2);
                    c.mfc1(3, 0);
                    c.lqc2(4, at(c, S1, 0x20));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, at(c, SP, 0x40));
                    c.lq(2, at(c, SP, 0x40));
                    c.sq(2, at(c, SP, 0x30));
                    c.mfc1(3, 2);
                    c.lqc2(4, at(c, S2, 0x10));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, at(c, SP, 0x50));
                    c.lq(2, at(c, SP, 0x50));
                    c.sq(2, at(c, SP, 0x40));
                    c.lqc2(3, at(c, SP, 0x30));
                    c.lqc2(4, at(c, SP, 0x40));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, at(c, SP, 0x50));
                    c.lq(2, at(c, SP, 0x50));
                    c.sq(2, at(c, S1, 0x20));
                    c.sq(2, at(c, SP, 0x20));
                    c.lqc2(3, at(c, S1, 0x20));
                    c.lqc2(4, at(c, S1, 0x140));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, at(c, SP, 0x40));
                    c.lq(2, at(c, SP, 0x40));
                    c.sq(2, at(c, SP, 0x30));
                    c.lqc2(3, at(c, SP, 0x30));
                    c.lqc2(4, at(c, S1, 0x160));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, at(c, SP, 0x40));
                    c.lq(2, at(c, SP, 0x40));
                    c.sq(2, at(c, S1, 0x40));
                    let t = true;
                    c.sq(2, at(c, SP, 0x20));
                    if t {
                        break 'b_165318;
                    } else {
                        break 'b_16518c;
                    }
                }
            }
            c.cycles += 6;
            c.lwc1(3, at(c, S1, 0x23c));
            c.mtc1(0, 1);
            c.c_lt_s(1, 3);
            if !c.fcc() {
                c.g[V0][0] = c.lw(at(c, S1, 0x2d0));
                break 'b_16531c;
            } else {
                'b_1651ec: {
                    c.cycles += 18;
                    c.lwc1(0, gp(c, -0x68e0));
                    c.g[AT][0] = 0x40400000;
                    c.mtc1(1, 2);
                    c.sub_s(0, 3, 0);
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 3);
                    c.max_s(0, 0, 1);
                    c.div_s(12, 0, 20);
                    c.swc1(0, at(c, S1, 0x23c));
                    c.mul_s(1, 12, 12);
                    c.add_s(0, 1, 1);
                    c.mul_s(1, 1, 2);
                    c.mul_s(0, 0, 12);
                    c.sub_s(0, 1, 0);
                    c.c_lt_s(3, 0);
                    if c.fcc() {
                        c.mov_s(0, 3);
                        break 'b_1651ec;
                    } else {
                        break 'b_1651ec;
                    }
                }
                c.cycles += 75;
                c.lq(2, at(c, S1, 0x20));
                c.mov_s(12, 0);
                c.sub_s(0, 3, 12);
                c.sq(2, at(c, SP, 0x20));
                c.mfc1(3, 0);
                c.lqc2(4, at(c, S1, 0x20));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x50));
                c.lq(2, at(c, SP, 0x50));
                c.sq(2, at(c, SP, 0x40));
                c.mfc1(3, 12);
                c.lqc2(4, at(c, S2, 0x10));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x60));
                c.lq(2, at(c, SP, 0x60));
                c.sq(2, at(c, SP, 0x50));
                c.lqc2(3, at(c, SP, 0x40));
                c.lqc2(4, at(c, SP, 0x50));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x60));
                c.lq(2, at(c, SP, 0x60));
                c.sq(2, at(c, S1, 0x20));
                c.sq(2, at(c, SP, 0x30));
                c.mfc1(3, 0);
                c.lqc2(4, at(c, S1, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x50));
                c.lq(2, at(c, SP, 0x50));
                c.sq(2, at(c, SP, 0x40));
                c.lqc2(3, at(c, S2, 0x10));
                c.lqc2(4, at(c, S1, 0x140));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x80));
                c.lq(2, at(c, SP, 0x80));
                c.sq(2, at(c, SP, 0x70));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, S1, 0x160));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x80));
                c.lq(2, at(c, SP, 0x80));
                c.sq(2, at(c, SP, 0x60));
                c.mfc1(3, 12);
                c.lqc2(4, at(c, SP, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x80));
                c.lq(2, at(c, SP, 0x80));
                c.sq(2, at(c, SP, 0x50));
                c.lqc2(3, at(c, SP, 0x40));
                c.lqc2(4, at(c, SP, 0x50));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x80));
                c.lq(2, at(c, SP, 0x80));
                c.sq(2, at(c, S1, 0x40));
                c.sq(2, at(c, SP, 0x30));
                c.lqc2(3, at(c, S1, 0x20));
                c.lqc2(4, at(c, S1, 0x40));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x40));
                c.lq(2, at(c, SP, 0x40));
                c.sq(2, at(c, S1, 0x170));
                c.sq(2, at(c, SP, 0x30));
                c.lqc2(3, at(c, S1, 0x20));
                c.lqc2(4, at(c, SP, 0x20));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x40));
                c.lq(2, at(c, SP, 0x40));
                c.lwc1(0, at(c, S1, 0x23c));
                c.sq(2, at(c, S1, 0x180));
                c.sq(2, at(c, SP, 0x30));
                c.swc1(0, at(c, S1, 0x240));
                c.swc1(0, at(c, S1, 0x244));
                break 'b_165318;
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.lw(at(c, S1, 0x2d0));
        break 'b_16531c;
    }
    'b_165448: {
        c.cycles += 2;
        let t = c.g[V0][0] == 0u64;

        if t {
            break 'b_165448;
        } else {
            c.cycles += 5;
            c.lwc1(4, at(c, S1, 0x240));
            c.mtc1(0, 3);
            c.c_lt_s(3, 4);
            let t = !c.fcc();

            if t {
                break 'b_165448;
            } else {
                c.cycles += 4;
                c.lwc1(1, at(c, S1, 0x244));
                c.c_lt_s(3, 1);
                let t = !c.fcc();

                if t {
                    break 'b_165448;
                } else {
                    'b_165390: {
                        c.cycles += 18;
                        c.lwc1(0, gp(c, -0x68dc));
                        c.g[AT][0] = 0x40400000;
                        c.mtc1(1, 2);
                        c.sub_s(0, 4, 0);
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 4);
                        c.max_s(0, 0, 3);
                        c.div_s(3, 0, 1);
                        c.swc1(0, at(c, S1, 0x240));
                        c.mul_s(1, 3, 3);
                        c.add_s(0, 1, 1);
                        c.mul_s(1, 1, 2);
                        c.mul_s(0, 0, 3);
                        c.sub_s(0, 1, 0);
                        c.c_lt_s(4, 0);
                        if c.fcc() {
                            c.mov_s(0, 4);
                            break 'b_165390;
                        } else {
                            break 'b_165390;
                        }
                    }
                    c.cycles += 46;
                    c.lqc2(3, at(c, S1, 0x20));
                    c.lqc2(4, at(c, S1, 0x40));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, at(c, SP, 0x30));
                    c.mov_s(3, 0);
                    c.lq(2, at(c, SP, 0x30));
                    c.sub_s(0, 4, 3);
                    c.sq(2, at(c, SP, 0x20));
                    c.mfc1(3, 3);
                    c.lqc2(4, at(c, S1, 0x170));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, at(c, SP, 0x50));
                    c.lq(2, at(c, SP, 0x50));
                    c.sq(2, at(c, SP, 0x40));
                    c.mfc1(3, 0);
                    c.lqc2(4, at(c, SP, 0x20));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, at(c, SP, 0x60));
                    c.lq(2, at(c, SP, 0x60));
                    c.sq(2, at(c, SP, 0x50));
                    c.lqc2(3, at(c, SP, 0x40));
                    c.lqc2(4, at(c, SP, 0x50));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, at(c, SP, 0x60));
                    c.lq(2, at(c, SP, 0x60));
                    c.sq(2, at(c, SP, 0x30));
                    c.mfc1(3, 3);
                    c.lqc2(4, at(c, S1, 0x180));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, at(c, SP, 0x50));
                    c.lq(2, at(c, SP, 0x50));
                    c.sq(2, at(c, SP, 0x40));
                    c.lqc2(3, at(c, S1, 0x20));
                    c.lqc2(4, at(c, SP, 0x40));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, at(c, S1, 0x20));
                    c.lqc2(3, at(c, S1, 0x20));
                    c.lqc2(4, at(c, SP, 0x30));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, at(c, SP, 0x50));
                    c.lq(2, at(c, SP, 0x50));
                    c.sq(2, at(c, S1, 0x40));
                    c.sq(2, at(c, SP, 0x40));
                    break 'b_165448;
                }
            }
        }
    }
    'b_165518: {
        c.cycles += 31;
        c.lqc2(3, at(c, S1, 0x20));
        c.lqc2(4, at(c, S1, 0x40));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x30));
        c.lwc1(1, gp(c, -0x68d8));
        c.lq(2, at(c, SP, 0x30));
        c.sq(2, at(c, SP, 0x20));
        c.lqc2(3, (c.g[SP][0] as u32));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(2, 22);
        c.mtc1(2, 0);
        c.lqc2(3, at(c, SP, 0x20));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(3, 22);
        c.mtc1(3, 2);
        c.mul_s(1, 0, 1);
        c.c_lt_s(2, 1);
        let t = !c.fcc();

        if t {
            break 'b_165518;
        } else {
            'b_165514: {
                c.cycles += 6;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.c_lt_s(0, 2);
                let t = !c.fcc();
                c.lq(2, at(c, SP, 0x10));
                if t {
                    break 'b_165514;
                } else {
                    c.cycles += 14;
                    c.div_s(0, 1, 2);
                    c.mfc1(2, 0);
                    c.lqc2(4, at(c, SP, 0x20));
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, at(c, SP, 0x20));
                    c.lqc2(3, at(c, S1, 0x20));
                    c.lqc2(4, at(c, SP, 0x20));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, at(c, SP, 0x40));
                    c.lq(2, at(c, SP, 0x40));
                    c.sq(2, at(c, S1, 0x40));
                    let t = true;
                    c.sq(2, at(c, SP, 0x30));
                    if t {
                        break 'b_165518;
                    } else {
                        break 'b_165514;
                    }
                }
            }
            c.cycles += 1;
            c.sq(2, at(c, S1, 0x40));
            break 'b_165518;
        }
    }
    c.cycles += 10;
    c.g[RA][0] = sx(0x00165520);
    c.g[A0][0] = c.g[S1][0];
    if call_ptr(c, 0x00168150, 0x00165520).is_err() {
        return;
    }
    c.lq(16, at(c, SP, 0xd0));
    c.lq(17, at(c, SP, 0xc0));
    c.lq(18, at(c, SP, 0xb0));
    c.g[RA][0] = c.ld(at(c, SP, 0xa0));
    c.lwc1(21, at(c, SP, 0xe8));
    c.lwc1(20, at(c, SP, 0xe0));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0xf0));
    c.pc = t;
    return;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, scramble, Rng, DATA, GAME_GP};
    use ssx3_ee_rt::cpu::{reg::*, sx};

    /// A pointer to the first descriptor byte is `a0`, the box being widened
    /// is `a1`; every float the pass reads (scalars at `+0x14`/`+0x1c`, the
    /// vectors at `+0x60`..`+0xef`, the box, the `gp-0x2868` reference
    /// vectors) is random, and the seeds reach both sides of the clamps.
    #[test]
    fn emitter_compute_desc_bounds_matches() {
        check(
            ssx3_ee::s12::f_0036d500__generated,
            emitter_compute_desc_bounds,
            300,
            &|c, seed| {
                let mut r = Rng(0x0036_d500 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x200);
                for i in 0..0x40u32 {
                    let v = if i % 3 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(100.0)
                    };
                    c.bus.write32(DATA + 4 * i, v);
                }
                for i in 0..8u32 {
                    c.bus.write32(DATA + 0x200 + 4 * i, r.f32(100.0));
                }
                let base = (GAME_GP as i32 - 0x2868) as u32;
                for i in 0..24u32 {
                    c.bus.write32(base + 4 * i, r.edge_f32());
                }
            },
        );
    }


}
