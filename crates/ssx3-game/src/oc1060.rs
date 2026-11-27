//! Staging module oc1060 (wave wave265): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1060`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0038_3a10,
    ps2_graphics_man_clip_line_segment,
    "PS2GraphicsMan_ClipLineSegment",
)];

/// Offsets of the clip box in the graphics manager (`a0`): `min` xyzw at
/// +0x6b70, `max` xyzw at +0x6b80.
#[cfg(test)]
pub(crate) const CLIPLINE_MIN: u32 = 0x6b70;
#[cfg(test)]
pub(crate) const CLIPLINE_MAX: u32 = 0x6b80;

/// `bool PS2GraphicsMan_ClipLineSegment(PS2GraphicsMan* self, Vtx* verts,
/// Pt* points, int* codes)` (0x383a10): clips a projected line segment (two
/// 0x30-byte vertices, two xyzw points, two 6-bit outcodes) to the clip box.
/// Stages in order: near (0x20), far (0x10), bottom (8), top (4) with a reject
/// test (the two outcodes share a bit) after each, then left (2) and right (1).
/// A stage moves the outside end onto the plane and interpolates u/v and the
/// four colour words. Returns false (`v0 = 0`) when rejected, true otherwise.
/// The scratch registers left behind (t0-t3, a0/a2 in the last stage, f0-f7,
/// lo/hi) are those of the generated body, so the stages keep its order.
pub fn ps2_graphics_man_clip_line_segment(c: &mut Cpu) {
    c.cycles += 6;
    c.g[T0][0] = c.lw(c.g[A3][0] as u32);
    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
    c.g[T1][0] = c.g[T0][0] | c.g[V1][0];
    let rejected = clipline_near(c) || clipline_far(c) || clipline_y_min(c) || clipline_y_max(c);
    if !rejected {
        clipline_x_min(c);
        clipline_x_max(c);
        c.cycles += 2;
        let ra = c.g[RA][0] as u32;
        c.g[V0][0] = 0x1;
        c.pc = ra;
        return;
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = 0;
    c.pc = ra;
}

/// The four colour channels of a clipped end (`verts[k].r..a`, `u32` words at
/// +0x10..+0x1c): `(float)v0 + frac * (float)(v1 - v0)` truncated, the unsigned
/// word turned into a float the compiler's way (halve, keep the low bit,
/// double when bit 31 is set). `fr` is the FPU register holding the factor,
/// `cr` the register of the last channel's `(float)v0`. The scratch registers it
/// leaves differ per stage, so the instruction order is kept exactly.
#[inline(never)]
fn clipline_colours(c: &mut Cpu, fr: usize, cr: usize) {
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
    let neg = (c.g[V0][0] as i64) < 0;
    c.g[T2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
    if neg {
        c.cycles += 8;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
        c.g[V0][0] = c.g[V0][0] & 0x1;
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.mtc1(2, 1);
        c.cvt_s_w(1, 1);
        c.add_s(1, 1, 1);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x40));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
    } else {
        c.cycles += 4;
        c.mtc1(2, 1);
        c.cvt_s_w(1, 1);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x40));
    }
    c.cycles += 3;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
    let neg = (c.g[V0][0] as i64) < 0;
    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
    if neg {
        c.cycles += 6;
        c.g[V0][0] = c.g[V0][0] & 0x1;
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.add_s(0, 0, 0);
        c.g[V0][0] = 0x30;
    } else {
        c.cycles += 4;
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.g[V0][0] = 0x30;
    }
    c.cycles += 10;
    c.mul_s(0, fr, 0);
    c.mult(0, 3, 9, 2);
    c.add_s(0, 1, 0);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
    c.cvt_w_s(1, 0);
    c.swc1(1, (c.g[T2][0] as u32));
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
    let neg = (c.g[V0][0] as i64) < 0;
    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
    if neg {
        c.cycles += 7;
        c.g[V0][0] = c.g[V0][0] & 0x1;
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.mtc1(2, 1);
        c.cvt_s_w(1, 1);
        c.add_s(1, 1, 1);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x44));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
    } else {
        c.cycles += 4;
        c.mtc1(2, 1);
        c.cvt_s_w(1, 1);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x44));
    }
    c.cycles += 3;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
    let neg = (c.g[V0][0] as i64) < 0;
    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
    if neg {
        c.cycles += 6;
        c.g[V0][0] = c.g[V0][0] & 0x1;
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.add_s(0, 0, 0);
        c.g[V0][0] = 0x30;
    } else {
        c.cycles += 4;
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.g[V0][0] = 0x30;
    }
    c.cycles += 10;
    c.mul_s(0, fr, 0);
    c.mult(0, 3, 9, 2);
    c.add_s(0, 1, 0);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x18));
    c.cvt_w_s(1, 0);
    c.swc1(1, (c.g[T0][0] as u32));
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
    let neg = (c.g[V0][0] as i64) < 0;
    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
    if neg {
        c.cycles += 7;
        c.g[V0][0] = c.g[V0][0] & 0x1;
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.mtc1(2, 1);
        c.cvt_s_w(1, 1);
        c.add_s(1, 1, 1);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x48));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
    } else {
        c.cycles += 4;
        c.mtc1(2, 1);
        c.cvt_s_w(1, 1);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x48));
    }
    c.cycles += 3;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
    let neg = (c.g[V0][0] as i64) < 0;
    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
    if neg {
        c.cycles += 6;
        c.g[V0][0] = c.g[V0][0] & 0x1;
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.add_s(0, 0, 0);
        c.mul_s(0, fr, 0);
    } else {
        c.cycles += 4;
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.mul_s(0, fr, 0);
    }
    c.cycles += 10;
    c.g[V0][0] = 0x30;
    c.mult(0, 3, 9, 2);
    c.add_s(0, 1, 0);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.cvt_w_s(1, 0);
    c.swc1(1, (c.g[T2][0] as u32));
    c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1c));
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
    let neg = (c.g[V0][0] as i64) < 0;
    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
    if neg {
        c.cycles += 7;
        c.g[V0][0] = c.g[V0][0] & 0x1;
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.mtc1(2, cr);
        c.cvt_s_w(cr, cr);
        c.add_s(cr, cr, cr);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4c));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
    } else {
        c.cycles += 4;
        c.mtc1(2, cr);
        c.cvt_s_w(cr, cr);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4c));
    }
    c.cycles += 3;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
    let neg = (c.g[V0][0] as i64) < 0;
    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
    if neg {
        c.cycles += 6;
        c.g[V0][0] = c.g[V0][0] & 0x1;
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.add_s(0, 0, 0);
        c.mul_s(0, fr, 0);
    } else {
        c.cycles += 4;
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.mul_s(0, fr, 0);
    }
}

/// Stage of `PS2GraphicsMan_ClipLineSegment`: near plane (outcode bit 0x20): the end that is behind `clip_min.w` moves onto the plane; z/w come from the plane, x/y/u/v/colour are interpolated with the perspective factor `frac * w1 / plane`. Re-derives that end's outcode and rejects when the two outcodes now share a bit.
#[inline(never)]
fn clipline_near(c: &mut Cpu) -> bool {
    c.g[V0][0] = c.g[T1][0] & 0x20;
    let t = c.g[V0][0] == 0u64;
    c.g[V1][0] = 0x30;
    if !t {
        c.cycles += 47;
        c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
        c.g[T1][0] = sx(((c.g[T0][0] as u32) as i32 >> 5) as u32);
        c.lwc1(7, (c.g[A0][0] as u32).wrapping_add(0x6b7c));
        c.g[T1][0] = c.g[T1][0] ^ 0x1;
        c.lwc1(6, (c.g[A2][0] as u32).wrapping_add(0x1c));
        c.g[T1][0] = c.g[T1][0] & 0x1;
        c.sub_s(0, 2, 7);
        c.lwc1(5, (c.g[A2][0] as u32));
        c.sub_s(2, 2, 6);
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x10));
        c.lwc1(4, (c.g[A2][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = sx((c.g[T1][0] as u32) << 4);
        c.sub_s(1, 5, 1);
        c.lwc1(3, (c.g[A2][0] as u32).wrapping_add(0x4));
        c.div_s(0, 0, 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.sub_s(3, 3, 4);
        c.mult(0, 8, 9, 3);
        c.mul_s(1, 0, 1);
        c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.mul_s(3, 0, 3);
        c.mul_s(0, 0, 6);
        c.sub_s(5, 5, 1);
        c.div_s(4, 0, 7);
        c.swc1(5, (c.g[V0][0] as u32));
        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
        c.sub_s(0, 0, 3);
        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
        c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x6b78));
        c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b7c));
        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
        c.lwc1(1, (c.g[A1][0] as u32));
        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x30));
        c.sub_s(0, 0, 1);
        c.mul_s(0, 4, 0);
        c.add_s(1, 1, 0);
        c.swc1(1, (c.g[V1][0] as u32));
        c.lwc1(2, (c.g[A1][0] as u32).wrapping_add(0x4));
        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x34));
        c.sub_s(0, 0, 2);
        c.mul_s(0, 4, 0);
        c.add_s(2, 2, 0);
        c.swc1(2, (c.g[V1][0] as u32).wrapping_add(0x4));
        clipline_colours(c, 4, 1);
        c.cycles += 14;
        c.g[V0][0] = sx((c.g[T1][0] as u32) << 2);
        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        c.g[V1][0] = sx((c.g[T1][0] as u32) << 4);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.add_s(0, 1, 0);
        c.cvt_w_s(1, 0);
        c.swc1(1, (c.g[T3][0] as u32));
        c.sw((c.g[T0][0] as u32), 0u64);
        c.lwc1(1, (c.g[V1][0] as u32));
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b70));
        c.c_lt_s(1, 0);
        let t = !c.fcc();
        c.g[V0][0] = 0x2;
        if !t {
            c.cycles += 2;
            c.sw((c.g[T0][0] as u32), c.g[V0][0]);
            c.lwc1(1, (c.g[V1][0] as u32));
        }
        c.cycles += 5;
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b80));
        c.c_lt_s(0, 1);
        if !c.fcc() {
            c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[T0][0] as u32));
            c.g[V0][0] = c.g[V0][0] | 0x1;
            c.sw((c.g[T0][0] as u32), c.g[V0][0]);
            c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
        }
        c.cycles += 5;
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b74));
        c.c_lt_s(1, 0);
        if !c.fcc() {
            c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[T0][0] as u32));
            c.g[V0][0] = c.g[V0][0] | 0x8;
            c.sw((c.g[T0][0] as u32), c.g[V0][0]);
            c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
        }
        c.cycles += 5;
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b84));
        c.c_lt_s(0, 1);
        if !c.fcc() {
            c.g[T0][0] = c.lw((c.g[A3][0] as u32));
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[T0][0] as u32));
            c.g[V0][0] = c.g[V0][0] | 0x4;
            c.sw((c.g[T0][0] as u32), c.g[V0][0]);
            c.g[T0][0] = c.lw((c.g[A3][0] as u32));
        }
        c.cycles += 4;
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.g[T0][0] & c.g[V1][0];
        let t = c.g[V0][0] == 0u64;
        c.g[T1][0] = c.g[T0][0] | c.g[V1][0];
        if !t {
            return true;
        }
    }
    false
}

/// Stage of `PS2GraphicsMan_ClipLineSegment`: far plane (outcode bit 0x10): as the near plane, against `clip_max.w`.
#[inline(never)]
fn clipline_far(c: &mut Cpu) -> bool {
    c.cycles += 3;
    c.g[V0][0] = c.g[T1][0] & 0x10;
    let t = c.g[V0][0] == 0u64;
    c.g[V1][0] = 0x30;
    if !t {
        c.cycles += 48;
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0xc));
        c.lwc1(6, (c.g[A0][0] as u32).wrapping_add(0x6b8c));
        c.lwc1(5, (c.g[A2][0] as u32).wrapping_add(0x1c));
        c.sub_s(0, 1, 6);
        c.lwc1(4, (c.g[A2][0] as u32));
        c.sub_s(1, 1, 5);
        c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0x10));
        c.lwc1(3, (c.g[A2][0] as u32).wrapping_add(0x4));
        c.sub_s(2, 4, 2);
        c.g[V0][0] = c.lw((c.g[A3][0] as u32));
        c.div_s(0, 0, 1);
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x14));
        c.g[T1][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
        c.g[T1][0] = c.g[T1][0] ^ 0x1;
        c.sub_s(3, 3, 1);
        c.g[T1][0] = c.g[T1][0] & 0x1;
        c.g[V0][0] = sx((c.g[T1][0] as u32) << 4);
        c.mult(0, 8, 9, 3);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.mul_s(2, 0, 2);
        c.mul_s(3, 0, 3);
        c.mul_s(0, 0, 5);
        c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.sub_s(4, 4, 2);
        c.div_s(5, 0, 6);
        c.swc1(4, (c.g[V0][0] as u32));
        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
        c.sub_s(0, 0, 3);
        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
        c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x6b88));
        c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b8c));
        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
        c.lwc1(1, (c.g[A1][0] as u32));
        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x30));
        c.sub_s(0, 0, 1);
        c.mul_s(0, 5, 0);
        c.add_s(1, 1, 0);
        c.swc1(1, (c.g[V1][0] as u32));
        c.lwc1(2, (c.g[A1][0] as u32).wrapping_add(0x4));
        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x34));
        c.sub_s(0, 0, 2);
        c.mul_s(0, 5, 0);
        c.add_s(2, 2, 0);
        c.swc1(2, (c.g[V1][0] as u32).wrapping_add(0x4));
        clipline_colours(c, 5, 1);
        c.cycles += 14;
        c.g[V0][0] = sx((c.g[T1][0] as u32) << 2);
        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        c.g[V1][0] = sx((c.g[T1][0] as u32) << 4);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.add_s(0, 1, 0);
        c.cvt_w_s(1, 0);
        c.swc1(1, (c.g[T3][0] as u32));
        c.sw((c.g[T0][0] as u32), 0u64);
        c.lwc1(1, (c.g[V1][0] as u32));
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b70));
        c.c_lt_s(1, 0);
        let t = !c.fcc();
        c.g[V0][0] = 0x2;
        if !t {
            c.cycles += 2;
            c.sw((c.g[T0][0] as u32), c.g[V0][0]);
            c.lwc1(1, (c.g[V1][0] as u32));
        }
        c.cycles += 5;
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b80));
        c.c_lt_s(0, 1);
        if !c.fcc() {
            c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[T0][0] as u32));
            c.g[V0][0] = c.g[V0][0] | 0x1;
            c.sw((c.g[T0][0] as u32), c.g[V0][0]);
            c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
        }
        c.cycles += 5;
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b74));
        c.c_lt_s(1, 0);
        if !c.fcc() {
            c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[T0][0] as u32));
            c.g[V0][0] = c.g[V0][0] | 0x8;
            c.sw((c.g[T0][0] as u32), c.g[V0][0]);
            c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
        }
        c.cycles += 5;
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b84));
        c.c_lt_s(0, 1);
        if !c.fcc() {
            c.g[T0][0] = c.lw((c.g[A3][0] as u32));
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[T0][0] as u32));
            c.g[V0][0] = c.g[V0][0] | 0x4;
            c.sw((c.g[T0][0] as u32), c.g[V0][0]);
            c.g[T0][0] = c.lw((c.g[A3][0] as u32));
        }
        c.cycles += 4;
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.g[T0][0] & c.g[V1][0];
        let t = c.g[V0][0] != 0u64;
        c.g[T1][0] = c.g[T0][0] | c.g[V1][0];
        if t {
            return true;
        }
    }
    false
}

/// Stage of `PS2GraphicsMan_ClipLineSegment`: bottom edge (outcode bit 8): the end below `clip_min.y` moves onto the edge; x/z/w/u/v/colour interpolate linearly. Rejects on a shared outcode bit (x outcodes re-derived).
#[inline(never)]
fn clipline_y_min(c: &mut Cpu) -> bool {
    c.cycles += 3;
    c.g[V0][0] = c.g[T1][0] & 0x8;
    let t = c.g[V0][0] == 0u64;
    c.g[V1][0] = 0x30;
    if !t {
        c.cycles += 51;
        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
        c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x6b74));
        c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0x14));
        c.sub_s(1, 0, 1);
        c.lwc1(5, (c.g[A2][0] as u32));
        c.sub_s(0, 0, 2);
        c.lwc1(3, (c.g[A2][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[A3][0] as u32));
        c.sub_s(3, 5, 3);
        c.lwc1(4, (c.g[A2][0] as u32).wrapping_add(0x8));
        c.div_s(6, 1, 0);
        c.g[T1][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
        c.g[T1][0] = c.g[T1][0] ^ 0x1;
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x18));
        c.g[T1][0] = c.g[T1][0] & 0x1;
        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = sx((c.g[T1][0] as u32) << 4);
        c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.sub_s(4, 4, 1);
        c.sub_s(2, 2, 0);
        c.mult(0, 8, 9, 3);
        c.mul_s(3, 6, 3);
        c.mul_s(4, 6, 4);
        c.mul_s(2, 6, 2);
        c.sub_s(5, 5, 3);
        c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.swc1(5, (c.g[V0][0] as u32));
        c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x6b74));
        c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
        c.sub_s(0, 0, 4);
        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0xc));
        c.sub_s(1, 1, 2);
        c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xc));
        c.lwc1(2, (c.g[A1][0] as u32));
        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x30));
        c.sub_s(0, 0, 2);
        c.mul_s(0, 6, 0);
        c.add_s(2, 2, 0);
        c.swc1(2, (c.g[V1][0] as u32));
        c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x4));
        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x34));
        c.sub_s(0, 0, 1);
        c.mul_s(0, 6, 0);
        c.add_s(1, 1, 0);
        c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
        clipline_colours(c, 6, 5);
        c.cycles += 14;
        c.g[V0][0] = sx((c.g[T1][0] as u32) << 2);
        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        c.g[V1][0] = sx((c.g[T1][0] as u32) << 4);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.add_s(0, 5, 0);
        c.cvt_w_s(1, 0);
        c.swc1(1, (c.g[T3][0] as u32));
        c.sw((c.g[T0][0] as u32), 0u64);
        c.lwc1(1, (c.g[V1][0] as u32));
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b70));
        c.c_lt_s(1, 0);
        let t = !c.fcc();
        c.g[V0][0] = 0x2;
        if !t {
            c.cycles += 2;
            c.sw((c.g[T0][0] as u32), c.g[V0][0]);
            c.lwc1(1, (c.g[V1][0] as u32));
        }
        c.cycles += 5;
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b80));
        c.c_lt_s(0, 1);
        if !c.fcc() {
            c.g[T0][0] = c.lw((c.g[A3][0] as u32));
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[T0][0] as u32));
            c.g[V0][0] = c.g[V0][0] | 0x1;
            c.sw((c.g[T0][0] as u32), c.g[V0][0]);
            c.g[T0][0] = c.lw((c.g[A3][0] as u32));
        }
        c.cycles += 4;
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.g[T0][0] & c.g[V1][0];
        let t = c.g[V0][0] != 0u64;
        c.g[T1][0] = c.g[T0][0] | c.g[V1][0];
        if t {
            return true;
        }
    }
    false
}

/// Stage of `PS2GraphicsMan_ClipLineSegment`: top edge (outcode bit 4): as the bottom edge, against `clip_max.y`.
#[inline(never)]
fn clipline_y_max(c: &mut Cpu) -> bool {
    c.cycles += 3;
    c.g[V0][0] = c.g[T1][0] & 0x4;
    let t = c.g[V0][0] == 0u64;
    c.g[V1][0] = 0x30;
    if !t {
        c.cycles += 51;
        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
        c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x6b84));
        c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0x14));
        c.sub_s(1, 0, 1);
        c.lwc1(5, (c.g[A2][0] as u32));
        c.sub_s(0, 0, 2);
        c.lwc1(3, (c.g[A2][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[A3][0] as u32));
        c.sub_s(3, 5, 3);
        c.lwc1(4, (c.g[A2][0] as u32).wrapping_add(0x8));
        c.div_s(6, 1, 0);
        c.g[T1][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
        c.g[T1][0] = c.g[T1][0] ^ 0x1;
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x18));
        c.g[T1][0] = c.g[T1][0] & 0x1;
        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = sx((c.g[T1][0] as u32) << 4);
        c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.sub_s(4, 4, 1);
        c.sub_s(2, 2, 0);
        c.mult(0, 8, 9, 3);
        c.mul_s(3, 6, 3);
        c.mul_s(4, 6, 4);
        c.mul_s(2, 6, 2);
        c.sub_s(5, 5, 3);
        c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.swc1(5, (c.g[V0][0] as u32));
        c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x6b84));
        c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
        c.sub_s(0, 0, 4);
        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0xc));
        c.sub_s(1, 1, 2);
        c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xc));
        c.lwc1(2, (c.g[A1][0] as u32));
        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x30));
        c.sub_s(0, 0, 2);
        c.mul_s(0, 6, 0);
        c.add_s(2, 2, 0);
        c.swc1(2, (c.g[V1][0] as u32));
        c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x4));
        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x34));
        c.sub_s(0, 0, 1);
        c.mul_s(0, 6, 0);
        c.add_s(1, 1, 0);
        c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
        clipline_colours(c, 6, 5);
        c.cycles += 14;
        c.g[V0][0] = sx((c.g[T1][0] as u32) << 2);
        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        c.g[V1][0] = sx((c.g[T1][0] as u32) << 4);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.add_s(0, 5, 0);
        c.cvt_w_s(1, 0);
        c.swc1(1, (c.g[T3][0] as u32));
        c.sw((c.g[T0][0] as u32), 0u64);
        c.lwc1(1, (c.g[V1][0] as u32));
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b70));
        c.c_lt_s(1, 0);
        let t = !c.fcc();
        c.g[V0][0] = 0x2;
        if !t {
            c.cycles += 2;
            c.sw((c.g[T0][0] as u32), c.g[V0][0]);
            c.lwc1(1, (c.g[V1][0] as u32));
        }
        c.cycles += 5;
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x6b80));
        c.c_lt_s(0, 1);
        if !c.fcc() {
            c.g[T0][0] = c.lw((c.g[A3][0] as u32));
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[T0][0] as u32));
            c.g[V0][0] = c.g[V0][0] | 0x1;
            c.sw((c.g[T0][0] as u32), c.g[V0][0]);
            c.g[T0][0] = c.lw((c.g[A3][0] as u32));
        }
        c.cycles += 4;
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.g[T0][0] & c.g[V1][0];
        let t = c.g[V0][0] != 0u64;
        c.g[T1][0] = c.g[T0][0] | c.g[V1][0];
        if t {
            return true;
        }
    }
    false
}

/// Stage of `PS2GraphicsMan_ClipLineSegment`: left edge (outcode bit 2): clip against `clip_min.x`; the moved end's outcode becomes 0 (no reject test: it is the last but one stage).
#[inline(never)]
fn clipline_x_min(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[T1][0] & 0x2;
    let t = c.g[V0][0] == 0u64;
    c.g[V1][0] = 0x30;
    if !t {
        c.cycles += 51;
        c.lwc1(0, (c.g[A2][0] as u32));
        c.lwc1(6, (c.g[A0][0] as u32).wrapping_add(0x6b70));
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x10));
        c.sub_s(2, 0, 6);
        c.g[V0][0] = c.lw((c.g[A3][0] as u32));
        c.sub_s(0, 0, 1);
        c.lwc1(3, (c.g[A2][0] as u32).wrapping_add(0x4));
        c.g[T0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x14));
        c.g[T0][0] = c.g[T0][0] ^ 0x1;
        c.lwc1(4, (c.g[A2][0] as u32).wrapping_add(0x8));
        c.div_s(7, 2, 0);
        c.g[T0][0] = c.g[T0][0] & 0x1;
        c.sub_s(3, 3, 1);
        c.g[V0][0] = sx((c.g[T0][0] as u32) << 4);
        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x18));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
        c.mult(0, 10, 8, 3);
        c.lwc1(5, (c.g[A2][0] as u32).wrapping_add(0x1c));
        c.sub_s(4, 4, 0);
        c.swc1(6, (c.g[V0][0] as u32));
        c.mul_s(3, 7, 3);
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x4));
        c.mul_s(4, 7, 4);
        c.sub_s(2, 2, 5);
        c.g[V1][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.sub_s(1, 1, 3);
        c.mul_s(2, 7, 2);
        c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
        c.sub_s(0, 0, 4);
        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0xc));
        c.sub_s(1, 1, 2);
        c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xc));
        c.lwc1(2, (c.g[A1][0] as u32));
        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x30));
        c.sub_s(0, 0, 2);
        c.mul_s(0, 7, 0);
        c.add_s(2, 2, 0);
        c.swc1(2, (c.g[V1][0] as u32));
        c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x4));
        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x34));
        c.sub_s(0, 0, 1);
        c.mul_s(0, 7, 0);
        c.add_s(1, 1, 0);
        c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
        let neg = (c.g[V0][0] as i64) < 0;
        c.g[T3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
        if neg {
            c.cycles += 8;
            c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.add_s(1, 1, 1);
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x40));
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
        } else {
            c.cycles += 4;
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x40));
        }
        c.cycles += 3;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
        let neg = (c.g[V0][0] as i64) < 0;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
        if neg {
            c.cycles += 6;
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.add_s(0, 0, 0);
            c.g[V0][0] = 0x30;
        } else {
            c.cycles += 4;
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.g[V0][0] = 0x30;
        }
        c.cycles += 10;
        c.mul_s(0, 7, 0);
        c.mult(0, 3, 8, 2);
        c.add_s(0, 1, 0);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
        c.cvt_w_s(1, 0);
        c.swc1(1, (c.g[T3][0] as u32));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
        let neg = (c.g[V0][0] as i64) < 0;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
        if neg {
            c.cycles += 7;
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.add_s(1, 1, 1);
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x44));
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
        } else {
            c.cycles += 4;
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x44));
        }
        c.cycles += 3;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
        let neg = (c.g[V0][0] as i64) < 0;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
        if neg {
            c.cycles += 6;
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.add_s(0, 0, 0);
            c.g[V0][0] = 0x30;
        } else {
            c.cycles += 4;
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.g[V0][0] = 0x30;
        }
        c.cycles += 10;
        c.mul_s(0, 7, 0);
        c.mult(0, 3, 8, 2);
        c.add_s(0, 1, 0);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x18));
        c.cvt_w_s(1, 0);
        c.swc1(1, (c.g[T2][0] as u32));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
        let neg = (c.g[V0][0] as i64) < 0;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
        if neg {
            c.cycles += 7;
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.add_s(1, 1, 1);
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x48));
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
        } else {
            c.cycles += 4;
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x48));
        }
        c.cycles += 3;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
        let neg = (c.g[V0][0] as i64) < 0;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
        if neg {
            c.cycles += 6;
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.add_s(0, 0, 0);
            c.g[V0][0] = 0x30;
        } else {
            c.cycles += 4;
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.g[V0][0] = 0x30;
        }
        c.cycles += 10;
        c.mul_s(0, 7, 0);
        c.mult(0, 3, 8, 2);
        c.add_s(0, 1, 0);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1c));
        c.cvt_w_s(1, 0);
        c.swc1(1, (c.g[T3][0] as u32));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
        let neg = (c.g[V0][0] as i64) < 0;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
        if neg {
            c.cycles += 7;
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.add_s(1, 1, 1);
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4c));
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
        } else {
            c.cycles += 4;
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4c));
        }
        c.cycles += 3;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
        let neg = (c.g[V0][0] as i64) < 0;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
        if neg {
            c.cycles += 6;
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.add_s(0, 0, 0);
            c.mul_s(0, 7, 0);
        } else {
            c.cycles += 4;
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.mul_s(0, 7, 0);
        }
        c.cycles += 6;
        c.g[V0][0] = sx((c.g[T0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        c.add_s(0, 1, 0);
        c.cvt_w_s(1, 0);
        c.swc1(1, (c.g[T2][0] as u32));
        c.sw((c.g[V0][0] as u32), 0u64);
    }
}

/// Stage of `PS2GraphicsMan_ClipLineSegment`: right edge (outcode bit 1): clip against `clip_max.x`; the moved end's outcode becomes 0.
#[inline(never)]
fn clipline_x_max(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[T1][0] & 0x1;
    let t = c.g[V0][0] == 0u64;
    c.g[V1][0] = 0x30;
    if !t {
        c.cycles += 50;
        c.lwc1(6, (c.g[A0][0] as u32).wrapping_add(0x6b80));
        c.lwc1(0, (c.g[A2][0] as u32));
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x10));
        c.sub_s(2, 0, 6);
        c.g[V0][0] = c.lw((c.g[A3][0] as u32));
        c.sub_s(0, 0, 1);
        c.lwc1(3, (c.g[A2][0] as u32).wrapping_add(0x4));
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x14));
        c.g[A0][0] = c.g[V0][0] ^ 0x1;
        c.g[A0][0] = c.g[A0][0] & 0x1;
        c.lwc1(4, (c.g[A2][0] as u32).wrapping_add(0x8));
        c.div_s(7, 2, 0);
        c.g[V0][0] = sx((c.g[A0][0] as u32) << 4);
        c.sub_s(3, 3, 1);
        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x18));
        c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.lwc1(5, (c.g[A2][0] as u32).wrapping_add(0x1c));
        c.sub_s(4, 4, 0);
        c.swc1(6, (c.g[V0][0] as u32));
        c.mult(0, 8, 4, 3);
        c.sub_s(2, 2, 5);
        c.mul_s(3, 7, 3);
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x4));
        c.mul_s(4, 7, 4);
        c.mul_s(2, 7, 2);
        c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.sub_s(1, 1, 3);
        c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
        c.sub_s(0, 0, 4);
        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0xc));
        c.sub_s(1, 1, 2);
        c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xc));
        c.lwc1(2, (c.g[A1][0] as u32));
        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x30));
        c.sub_s(0, 0, 2);
        c.mul_s(0, 7, 0);
        c.add_s(2, 2, 0);
        c.swc1(2, (c.g[V1][0] as u32));
        c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x4));
        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x34));
        c.sub_s(0, 0, 1);
        c.mul_s(0, 7, 0);
        c.add_s(1, 1, 0);
        c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
        let neg = (c.g[V0][0] as i64) < 0;
        c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
        if neg {
            c.cycles += 8;
            c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.add_s(1, 1, 1);
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x40));
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
        } else {
            c.cycles += 4;
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x40));
        }
        c.cycles += 3;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
        let neg = (c.g[V0][0] as i64) < 0;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
        if neg {
            c.cycles += 6;
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.add_s(0, 0, 0);
            c.mul_s(0, 7, 0);
        } else {
            c.cycles += 4;
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.mul_s(0, 7, 0);
        }
        c.cycles += 10;
        c.g[V0][0] = 0x30;
        c.mult(0, 3, 4, 2);
        c.add_s(0, 1, 0);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.cvt_w_s(1, 0);
        c.swc1(1, (c.g[T0][0] as u32));
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
        let neg = (c.g[V0][0] as i64) < 0;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
        if neg {
            c.cycles += 7;
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.add_s(1, 1, 1);
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x44));
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
        } else {
            c.cycles += 4;
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x44));
        }
        c.cycles += 3;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
        let neg = (c.g[V0][0] as i64) < 0;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
        if neg {
            c.cycles += 6;
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.add_s(0, 0, 0);
            c.mul_s(0, 7, 0);
        } else {
            c.cycles += 4;
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.mul_s(0, 7, 0);
        }
        c.cycles += 10;
        c.g[V0][0] = 0x30;
        c.mult(0, 3, 4, 2);
        c.add_s(0, 1, 0);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.cvt_w_s(1, 0);
        c.swc1(1, (c.g[A2][0] as u32));
        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x18));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
        let neg = (c.g[V0][0] as i64) < 0;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
        if neg {
            c.cycles += 7;
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.add_s(1, 1, 1);
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x48));
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
        } else {
            c.cycles += 4;
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x48));
        }
        c.cycles += 3;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
        let neg = (c.g[V0][0] as i64) < 0;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
        if neg {
            c.cycles += 6;
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.add_s(0, 0, 0);
            c.mul_s(0, 7, 0);
        } else {
            c.cycles += 4;
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.mul_s(0, 7, 0);
        }
        c.cycles += 10;
        c.g[V0][0] = 0x30;
        c.mult(0, 3, 4, 2);
        c.add_s(0, 1, 0);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.cvt_w_s(1, 0);
        c.swc1(1, (c.g[T0][0] as u32));
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
        let neg = (c.g[V0][0] as i64) < 0;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
        if neg {
            c.cycles += 6;
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 5);
            c.cvt_s_w(5, 5);
            c.add_s(5, 5, 5);
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
        } else {
            c.cycles += 4;
            c.mtc1(2, 5);
            c.cvt_s_w(5, 5);
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
        }
        c.cycles += 4;
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4c));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
        let neg = (c.g[V0][0] as i64) < 0;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
        if neg {
            c.cycles += 6;
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.add_s(0, 0, 0);
            c.mul_s(0, 7, 0);
        } else {
            c.cycles += 4;
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.mul_s(0, 7, 0);
        }
        c.cycles += 6;
        c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        c.add_s(0, 5, 0);
        c.cvt_w_s(1, 0);
        c.swc1(1, (c.g[A2][0] as u32));
        c.sw((c.g[V0][0] as u32), 0u64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA};

    /// `PS2GraphicsMan_ClipLineSegment`: points spread over every outcode
    /// (near/far by w buckets, x/y outside the unit box), outcodes derived
    /// from the points (or random in a fifth of the seeds), colour words with
    /// and without bit 31 (both unsigned-to-float paths).
    #[test]
    fn ps2_graphics_man_clip_line_segment_matches() {
        check(
            ee::s12::f_00383a10__generated,
            ps2_graphics_man_clip_line_segment,
            600,
            &|c, seed| {
                let mut r =
                    Rng(0x3a10_0383 ^ (seed as u64 + 1).wrapping_mul(0x9e37_79b9_7f4a_7c15));
                scramble(c, &mut r);
                let gm = DATA + 0x1000;
                let verts = DATA + 0x100;
                let pts = DATA + 0x200;
                let codes = DATA + 0x300;
                c.g[A0][0] = sx(gm);
                c.g[A1][0] = sx(verts);
                c.g[A2][0] = sx(pts);
                c.g[A3][0] = sx(codes);
                // clip box: min xyzw, max xyzw
                let min = [-1.0f32, -1.0, 0.0, 0.5];
                let max = [1.0f32, 1.0, 1.0, 100.0];
                for i in 0..4 {
                    c.bus
                        .write32(gm + CLIPLINE_MIN + 4 * i as u32, min[i].to_bits());
                    c.bus
                        .write32(gm + CLIPLINE_MAX + 4 * i as u32, max[i].to_bits());
                }
                let w_base = [0.1f32, 0.4, 5.0, 50.0, 120.0, 200.0, 0.3, 0.7];
                let mut p = [[0.0f32; 4]; 2];
                for k in 0..2 {
                    let x = f32::from_bits(r.f32(2.5));
                    let y = f32::from_bits(r.f32(2.5));
                    let z = f32::from_bits(r.f32(1.5));
                    let wb = w_base[(r.next_u32() % 8) as usize];
                    let w = wb + f32::from_bits(r.f32(0.2)).abs();
                    // some seeds keep an end inside the box in x or y
                    let (x, y) = match (seed / 7 + k as u32) % 4 {
                        0 => (x * 0.3, y),
                        1 => (x, y * 0.3),
                        _ => (x, y),
                    };
                    p[k] = [x, y, z, w];
                    for (j, v) in p[k].iter().enumerate() {
                        c.bus
                            .write32(pts + 0x10 * k as u32 + 4 * j as u32, v.to_bits());
                    }
                }
                let mut oc = [0u32; 2];
                for k in 0..2 {
                    let q = p[k];
                    if q[3] < min[3] {
                        oc[k] |= 0x20;
                    }
                    if q[3] > max[3] {
                        oc[k] |= 0x10;
                    }
                    if q[1] < min[1] {
                        oc[k] |= 8;
                    }
                    if q[1] > max[1] {
                        oc[k] |= 4;
                    }
                    if q[0] < min[0] {
                        oc[k] |= 2;
                    }
                    if q[0] > max[0] {
                        oc[k] |= 1;
                    }
                }
                if seed % 5 == 4 {
                    oc = [r.next_u32() & 0x3f, r.next_u32() & 0x3f];
                }
                c.bus.write32(codes, oc[0]);
                c.bus.write32(codes + 4, oc[1]);
                // two 0x30-byte vertices: u, v, colour words at +0x10..+0x1c
                for k in 0..2u32 {
                    let v = verts + 0x30 * k;
                    for off in (0..0x30).step_by(4) {
                        let word = if (0x10..0x20).contains(&off) {
                            let x = r.next_u32();
                            if r.next_u32() & 1 == 0 {
                                x | 0x8000_0000
                            } else {
                                x & 0x7fff_ffff
                            }
                        } else {
                            r.f32(4.0)
                        };
                        c.bus.write32(v + off, word);
                    }
                }
            },
        );
    }
}
