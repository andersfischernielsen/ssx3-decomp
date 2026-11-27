//! Staging module oc1064 (wave wave265): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1064`.
//!
//! The five functions are emitters of GS packets and ribbons: long straight-line
//! register code with a few loops. The ports keep the generated register
//! allocation, cycle charges and call order block for block (the original is
//! one hand-scheduled body per function), with the callees called as the
//! generated body calls them.

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
        0x0036_b5e8,
        ps2_graphics_man_emit_depth_fog_flat_fill,
        "PS2GraphicsMan_EmitDepthFogFlatFill",
    ),
    (
        0x0036_c188,
        ps2_graphics_man_emit_blended_fill,
        "PS2GraphicsMan_EmitBlendedFill",
    ),
    (
        0x0036_c398,
        ps2_graphics_man_emit_bloom_composite,
        "PS2GraphicsMan_EmitBloomComposite",
    ),
    (0x0037_26c8, rail_tube_render, "RailTube_Render"),
    (
        0x0038_ca70,
        ps2_bezier_man_emit_patch_type1,
        "PS2BezierMan_EmitPatchType1",
    ),
];

/// `void PS2GraphicsMan_EmitDepthFogFlatFill(PS2GraphicsMan* gm, int idx, uint64** pp)`
/// (0x36b5e8): emit the GS packet (a CLEAR/fill sprite strip, one 32 pixel
/// column per sprite pair) that fills saved rectangle `idx` of the graphics
/// manager with its flat fog colour (`gm+idx*0x20+0x6c64`: r, g, b floats
/// scaled by 256), then the frame/z restore packet; the rectangle size comes
/// from the manager's virtual width/height queries (vtable slots 0x50/0x58).
/// The packet is built at `*pp` and `*pp` is advanced past it.
pub fn ps2_graphics_man_emit_depth_fog_flat_fill(c: &mut Cpu) {
    guest(c, |c| {
        'b_36b85c: {
            c.cycles += 133;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S3][0] = c.g[A1][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[V1][0] = sx((c.g[S3][0] as u32) << 3);
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S3][0] = sx((c.g[S3][0] as u32) << 5);
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S6][0] = c.g[A0][0];
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[A3][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[T0][0] = c.g[A3][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[V1][0] = c.g[A3][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6c64));
            c.g[S3][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[S3][0] as u32)));
            c.g[S0][0] = 0x1000;
            c.g[A1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10d8));
            c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x59b6));
            c.g[A0][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x50));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x54));
            c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.sw((c.g[SP][0] as u32), c.g[A2][0]);
            c.g[S1][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x59b4));
            c.g[S2][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x59b0));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0036b66c);
            c.g[S4][0] = c.lhu((c.g[T0][0] as u32).wrapping_add(0x59b2));
            call_ptr(c, t, 0x0036b66c)?;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
            c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10d8));
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
            c.g[FP][0] = sx((c.g[V0][0] as u32) << 4);
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0036b690);
            c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x0036b690)?;
            c.g[V1][0] = 0xffffffffffffffff;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1f));
            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[S1][0] as i64)) as u64;
            c.g[A1][0] = c.lw((c.g[SP][0] as u32));
            if c.g[V1][0] != 0 {
                c.g[A0][0] = c.g[S1][0];
            }
            c.g[S2][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x5a84));
            c.g[S7][0] = sx(((c.g[A0][0] as u32) as i32 >> 5) as u32);
            c.g[S5][0] = c.lw((c.g[A1][0] as u32));
            c.g[V1][0] = sx((c.g[S7][0] as u32) << 1);
            c.g[A1][0] = 0x8000;
            c.g[A1][0] = c.g[A1][0] << 45;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x7));
            c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x5a80));
            c.g[V1][0] = c.g[V1][0] | c.g[A1][0];
            c.g[A0][0] = 0xe;
            c.sd((c.g[S5][0] as u32), c.g[V1][0]);
            c.sd((c.g[S5][0] as u32).wrapping_add(0x8), c.g[A0][0]);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
            c.g[S0][0] = sx((c.g[S0][0] as u32) >> 1);
            c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10d8));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
            c.g[S0][0] = sx((c.g[S0][0] as u32) << 4);
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0036b6fc);
            c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x0036b6fc)?;
            c.g[V1][0] = 0x1000000;
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
            c.g[S1][0] = c.g[S1][0] | c.g[V1][0];
            c.g[S2][0] = c.g[S2][0] << 32;
            c.g[V0][0] = c.g[V0][0] << 32;
            c.g[S1][0] = c.g[S1][0] << 32;
            c.g[V0][0] = c.g[V0][0] >> 16;
            c.g[V1][0] = 0x8000;
            c.g[V1][0] = c.g[V1][0] << 17;
            c.g[T4][0] = c.g[S0][0];
            c.g[S2][0] = c.g[S2][0] >> 32;
            c.g[S1][0] = c.g[S1][0] >> 32;
            c.g[S2][0] = c.g[S2][0] | c.g[V0][0];
            c.g[S1][0] = c.g[S1][0] | c.g[V1][0];
            c.g[A0][0] = c.g[T4][0] << 32;
            c.g[A0][0] = c.g[FP][0] | c.g[A0][0];
            c.g[V1][0] = 0x4e;
            c.g[V0][0] = 0x4c;
            c.g[A1][0] = 0x70000;
            c.g[A2][0] = 0x47;
            c.g[A3][0] = 0x18;
            c.g[T0][0] = 0x11;
            c.g[T1][0] = 0x42;
            c.g[T2][0] = 0x46;
            c.sd((c.g[S5][0] as u32).wrapping_add(0x28), c.g[V1][0]);
            c.sd((c.g[S5][0] as u32).wrapping_add(0x40), c.g[A0][0]);
            c.g[T3][0] = 0x3f800000;
            c.g[T3][0] = c.g[T3][0] << 16;
            c.g[T3][0] = c.g[T3][0] | 0x8000;
            c.g[T3][0] = c.g[T3][0] << 16;
            c.sd((c.g[S5][0] as u32).wrapping_add(0x18), c.g[V0][0]);
            c.sd((c.g[S5][0] as u32).wrapping_add(0x30), c.g[A1][0]);
            c.g[V0][0] = 0x1;
            c.sd((c.g[S5][0] as u32).wrapping_add(0x10), c.g[S2][0]);
            c.sd((c.g[S5][0] as u32).wrapping_add(0x20), c.g[S1][0]);
            c.sd((c.g[S5][0] as u32).wrapping_add(0x38), c.g[A2][0]);
            c.sd((c.g[S5][0] as u32).wrapping_add(0x48), c.g[A3][0]);
            c.sd((c.g[S5][0] as u32).wrapping_add(0x50), c.g[T0][0]);
            c.sd((c.g[S5][0] as u32).wrapping_add(0x58), c.g[T1][0]);
            c.sd((c.g[S5][0] as u32).wrapping_add(0x60), c.g[T2][0]);
            c.sd((c.g[S5][0] as u32).wrapping_add(0x68), 0u64);
            c.g[AT][0] = 0x43800000;
            c.mtc1(1, 1);
            c.lwc1(2, (c.g[S3][0] as u32).wrapping_add(0x18));
            c.lwc1(3, (c.g[S3][0] as u32).wrapping_add(0x1c));
            c.mul_s(2, 2, 1);
            c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x14));
            c.mul_s(3, 3, 1);
            c.sd((c.g[S5][0] as u32).wrapping_add(0x78), c.g[V0][0]);
            c.mul_s(0, 0, 1);
            c.cvt_w_s(1, 2);
            c.mfc1(2, 1);
            c.cvt_w_s(1, 3);
            c.mfc1(3, 1);
            c.cvt_w_s(1, 0);
            c.mfc1(4, 1);
            c.g[V0][0] = c.g[V0][0] << 8;
            c.g[V0][0] = c.g[A0][0] | c.g[V0][0];
            c.g[V1][0] = c.g[V1][0] << 16;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] | c.g[T3][0];
            c.sd((c.g[S5][0] as u32).wrapping_add(0x70), c.g[V0][0]);
            let t = c.g[S7][0] == 0u64;
            c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x80));
            if t {
                break 'b_36b85c;
            } else {
                c.cycles += 11;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                c.g[T1][0] = c.g[T4][0] << 16;
                c.g[A0][0] = c.g[S7][0];
                c.g[A2][0] = c.g[FP][0];
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 4);
                c.g[T0][0] = 0xfffe;
                c.g[T0][0] = c.g[T0][0] << 31;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A3][0] = 0x5;
                c.g[S0][0] = c.g[V0][0] << 16;
                c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x200));
                'l_36b828: loop {
                    c.cycles += 13;
                    c.g[V0][0] = c.g[A2][0] | c.g[T1][0];
                    c.g[V1][0] = c.g[A1][0] | c.g[S0][0];
                    c.g[V0][0] = c.g[V0][0] | c.g[T0][0];
                    c.g[V1][0] = c.g[V1][0] | c.g[T0][0];
                    c.sd((c.g[S5][0] as u32), c.g[V0][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x200));
                    c.sd((c.g[S5][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x200));
                    c.sd((c.g[S5][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                    c.sd((c.g[S5][0] as u32).wrapping_add(0x18), c.g[A3][0]);
                    let t = c.g[A0][0] != 0u64;
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0036b828);
                        }
                        continue 'l_36b828;
                    } else {
                        break 'b_36b85c;
                    }
                }
            }
        }
        c.cycles += 95;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32));
        c.g[A3][0] = 0x10000000;
        c.g[A3][0] = c.g[A3][0] | 0x4;
        c.g[T0][0] = 0x10000000;
        c.g[T0][0] = c.g[T0][0] << 32;
        c.g[T0][0] = c.g[T0][0] | 0x8003;
        c.g[A2][0] = 0xe;
        c.g[A0][0] = c.lw((c.g[A1][0] as u32));
        c.g[S1][0] = 0x1000;
        c.g[A1][0] = 0x10000000;
        c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 31);
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
        c.sd((c.g[A0][0] as u32), c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[SP][0] as u32));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
        c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
        c.sd((c.g[S5][0] as u32), c.g[A3][0]);
        c.sd((c.g[S5][0] as u32).wrapping_add(0x10), c.g[T0][0]);
        c.sd((c.g[S5][0] as u32).wrapping_add(0x18), c.g[A2][0]);
        c.sd((c.g[S5][0] as u32).wrapping_add(0x8), 0u64);
        c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10d8));
        c.g[S0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x5a84));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0036b8d8);
        c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0036b8d8)?;
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
        c.g[S0][0] = c.g[S0][0] << 32;
        c.g[V0][0] = c.g[V0][0] << 32;
        c.g[S0][0] = c.g[S0][0] >> 32;
        c.g[V0][0] = c.g[V0][0] >> 16;
        c.g[V1][0] = 0x4c;
        c.g[S0][0] = c.g[S0][0] | c.g[V0][0];
        c.sd((c.g[S5][0] as u32).wrapping_add(0x28), c.g[V1][0]);
        c.sd((c.g[S5][0] as u32).wrapping_add(0x20), c.g[S0][0]);
        c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0036b910);
        c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0036b910)?;
        c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10d8));
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
        c.g[S0][0] = sx((c.g[S0][0] as u32) >> 1);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 4);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0036b930);
        c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0036b930)?;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
        c.g[A3][0] = 0x40;
        c.g[S1][0] = sx((c.g[S1][0] as u32) >> 1);
        c.g[V0][0] = 0x18;
        c.g[S1][0] = sx((c.g[S1][0] as u32) << 4);
        c.sd((c.g[S5][0] as u32).wrapping_add(0x38), c.g[V0][0]);
        c.g[S1][0] = c.g[S1][0] << 32;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S1][0] = c.g[S0][0] | c.g[S1][0];
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sd((c.g[S5][0] as u32).wrapping_add(0x30), c.g[S1][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[V0][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0x5900));
        c.g[V1][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0x5904));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.g[A0][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0x5902));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[A2][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0x5906));
        c.g[V1][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0x5900));
        c.g[V0][0] = c.g[V0][0] << 16;
        c.g[A1][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0x5902));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.g[A1][0] = c.g[A1][0] << 32;
        c.g[A0][0] = c.g[A0][0] << 48;
        c.g[V1][0] = c.g[V1][0] | c.g[A1][0];
        c.sd((c.g[S5][0] as u32).wrapping_add(0x48), c.g[A3][0]);
        c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sd((c.g[S5][0] as u32).wrapping_add(0x40), c.g[V1][0]);
        c.g[V1][0] = c.lw((c.g[SP][0] as u32));
        c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sw((c.g[V1][0] as u32), c.g[S5][0]);
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    });
}

/// `void PS2GraphicsMan_EmitBlendedFill(void* self, int x, int w, int h, int y,
/// int alpha, uint32* handle)` (0x36c188; argument names from the reference): build a GS packet in a scratchpad
/// stream block (`SprStream_BeginBlock` on the stream at gp+0x1384): a sprite
/// strip of `w/32` columns with the grey/alpha blend registers, then
/// `SprStream_EndBlockAligned` and store the next block handle in `*handle`.
pub fn ps2_graphics_man_emit_blended_fill(c: &mut Cpu) {
    guest(c, |c| {
        'b_36c1e8: {
            c.cycles += 24;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S0][0] = c.g[A2][0];
            c.g[S2][0] = 0xffffffffffffffff;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1f));
            c.g[S2][0] = ((c.g[S2][0] as i64) < (c.g[S0][0] as i64)) as u64;
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
            if c.g[S2][0] != 0 {
                c.g[V0][0] = c.g[S0][0];
            }
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S6][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.div(0, 16, 22);
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S3][0] = 0u64;
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S4][0] = c.g[A1][0];
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S1][0] = c.g[T0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[S5][0] = c.g[T1][0];
            if c.g[S6][0] == c.g[S3][0] {
                break 'b_36c1e8;
            } else {
                break 'b_36c1e8;
            }
        }
        'b_36c350: {
            c.cycles += 74;
            c.g[A2][0] = 0xffffffffffffffff;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[T2][0]);
            c.sw((c.g[SP][0] as u32), c.g[A3][0]);
            c.g[FP][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
            c.g[A3][0] = 0u64;
            c.g[A1][0] = c.lw((c.g[T2][0] as u32));
            c.g[A0][0] = c.g[FP][0];
            c.g[V0][0] = c.lo[0];
            c.g[RA][0] = sx(0x0036c210);
            c.g[S7][0] = sx((c.g[V0][0] as u32) << 4);
            Callee {
                addr: 0x0038f460,
                f: ee::s12::SprStream_BeginBlock,
            }
            .call(c, 0x0036c210)?;
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3f));
            c.g[A3][0] = sx((c.g[S6][0] as u32) << 1);
            if c.g[S2][0] != 0 {
                c.g[V1][0] = c.g[S0][0];
            }
            c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x7));
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 6) as u32);
            c.g[A0][0] = 0x10000000;
            c.g[S3][0] = ((c.g[S3][0] as i64) < (c.g[V1][0] as i64)) as u64;
            c.g[A1][0] = 0x1;
            c.g[T3][0] = c.g[V0][0];
            c.g[A2][0] = c.g[A2][0] | c.g[A0][0];
            if c.g[S3][0] != 0 {
                c.g[A1][0] = c.g[V1][0];
            }
            c.sd((c.g[T3][0] as u32), c.g[A2][0]);
            c.g[V1][0] = c.g[S1][0] << 8;
            c.g[S4][0] = sx(((c.g[S4][0] as u32) as i32 >> 5) as u32);
            c.g[A1][0] = c.g[A1][0] << 16;
            c.g[V0][0] = 0x8000;
            c.g[V0][0] = c.g[V0][0] << 45;
            c.g[A0][0] = 0xffffffffff000000;
            c.g[A0][0] = c.g[A0][0] << 16;
            c.g[A0][0] = c.g[A0][0] | 0x100;
            c.g[A0][0] = c.g[A0][0] << 16;
            c.g[A2][0] = 0xa1;
            c.g[V1][0] = c.g[S1][0] | c.g[V1][0];
            c.g[A1][0] = c.g[S4][0] | c.g[A1][0];
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x6));
            c.g[S5][0] = c.g[S5][0] << 32;
            c.sd((c.g[T3][0] as u32).wrapping_add(0x8), 0u64);
            c.g[A1][0] = c.g[A1][0] | c.g[A0][0];
            c.g[A3][0] = c.g[A3][0] | c.g[V0][0];
            c.g[S5][0] = c.g[S5][0] | c.g[A2][0];
            c.g[S1][0] = c.g[S1][0] << 16;
            c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = 0xe;
            c.g[A2][0] = 0x4c;
            c.g[V0][0] = 0x3f800000;
            c.g[V0][0] = c.g[V0][0] << 16;
            c.g[V0][0] = c.g[V0][0] | 0x8000;
            c.g[V0][0] = c.g[V0][0] << 16;
            c.g[T0][0] = 0x18;
            c.g[T1][0] = 0x42;
            c.g[T2][0] = 0x3f;
            c.g[V1][0] = c.g[V1][0] | c.g[S1][0];
            c.sd((c.g[T3][0] as u32).wrapping_add(0x8), c.g[A0][0]);
            c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
            c.sd((c.g[T3][0] as u32).wrapping_add(0x10), c.g[A1][0]);
            c.g[V0][0] = 0x146;
            c.sd((c.g[T3][0] as u32).wrapping_add(0x18), c.g[A2][0]);
            c.g[A0][0] = 0x1;
            c.sd((c.g[T3][0] as u32), c.g[A3][0]);
            c.g[A2][0] = 0u64;
            c.sd((c.g[T3][0] as u32).wrapping_add(0x28), c.g[T0][0]);
            c.g[A1][0] = c.g[S7][0];
            c.sd((c.g[T3][0] as u32).wrapping_add(0x30), c.g[S5][0]);
            c.sd((c.g[T3][0] as u32).wrapping_add(0x38), c.g[T1][0]);
            c.sd((c.g[T3][0] as u32).wrapping_add(0x48), c.g[T2][0]);
            c.sd((c.g[T3][0] as u32).wrapping_add(0x20), 0u64);
            c.sd((c.g[T3][0] as u32).wrapping_add(0x40), 0u64);
            c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x50));
            c.sd((c.g[T3][0] as u32), c.g[V0][0]);
            c.sd((c.g[T3][0] as u32).wrapping_add(0x10), c.g[V1][0]);
            c.sd((c.g[T3][0] as u32).wrapping_add(0x18), c.g[A0][0]);
            c.sd((c.g[T3][0] as u32).wrapping_add(0x8), 0u64);
            let t = (c.g[S6][0] as i64) <= 0;
            c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x20));
            if t {
                break 'b_36c350;
            } else {
                c.cycles += 6;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32));
                c.g[A3][0] = c.g[S6][0];
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 4);
                c.g[V1][0] = 0x5;
                c.g[A0][0] = c.g[V0][0] << 16;
                'l_36c328: loop {
                    c.cycles += 10;
                    c.g[V0][0] = c.g[A1][0] | c.g[A0][0];
                    c.sd((c.g[T3][0] as u32), c.g[A2][0]);
                    c.sd((c.g[T3][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[S7][0] as u32)));
                    c.sd((c.g[T3][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S7][0] as u32)));
                    c.sd((c.g[T3][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
                    let t = c.g[A3][0] != 0u64;
                    c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0036c328);
                        }
                        continue 'l_36c328;
                    } else {
                        break 'b_36c350;
                    }
                }
            }
        }
        c.cycles += 18;
        c.g[A0][0] = c.g[FP][0];
        c.g[A1][0] = c.g[T3][0];
        c.g[RA][0] = sx(0x0036c360);
        c.g[A2][0] = 0x4;
        Callee {
            addr: 0x0038f668,
            f: ee::s12::SprStream_EndBlockAligned,
        }
        .call(c, 0x0036c360)?;
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[V1][0] as u32), c.g[V0][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    });
}

/// `void PS2GraphicsMan_EmitBloomComposite(PS2GraphicsMan* gm, ..., uint32* handle)`
/// (0x36c398): like `EmitBlendedFill`, but a textured strip: a scratchpad
/// stream block with the frame/z/texture registers (the frame width comes from
/// the manager's virtual width query, vtable slot 0x50) and one textured
/// sprite per 32 pixel column; ends with `SprStream_EndBlockAligned` and
/// stores the next block handle in `*handle` (stack argument at +0xe8).
pub fn ps2_graphics_man_emit_bloom_composite(c: &mut Cpu) {
    guest(c, |c| {
        'b_36c3fc: {
            c.cycles += 25;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
            c.g[V0][0] = 0xffffffffffffffff;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S3][0] = c.g[T3][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S3][0] as i64)) as u64;
            c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1f));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
            if c.g[V0][0] != 0 {
                c.g[V1][0] = c.g[S3][0];
            }
            c.g[FP][0] = sx(((c.g[V1][0] as u32) as i32 >> 5) as u32);
            c.g[S2][0] = c.g[A2][0];
            c.div(0, 18, 30);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[S0][0] = c.g[T0][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S1][0] = c.g[A0][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[A2][0] = 0xffffffffffffffff;
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
            c.g[S7][0] = c.g[T1][0];
            if c.g[FP][0] == 0u64 {
                break 'b_36c3fc;
            } else {
                break 'b_36c3fc;
            }
        }
        'b_36c4e8: {
            'b_36c4d4: {
                c.cycles += 39;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A3][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V1][0]);
                c.g[A3][0] = 0u64;
                c.sw((c.g[SP][0] as u32), c.g[A1][0]);
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe8));
                c.g[S5][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5a80));
                c.g[A1][0] = c.lw((c.g[A0][0] as u32));
                c.g[A0][0] = c.g[V1][0];
                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[T2][0]);
                c.g[V0][0] = c.lo[0];
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                c.g[RA][0] = sx(0x0036c434);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                Callee {
                    addr: 0x0038f460,
                    f: ee::s12::SprStream_BeginBlock,
                }
                .call(c, 0x0036c434)?;
                c.g[A0][0] = sx((c.g[FP][0] as u32) << 2);
                c.g[S6][0] = c.g[V0][0];
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0xd));
                c.g[A1][0] = 0x10000000;
                c.g[V1][0] = c.g[V1][0] | c.g[A1][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = 0x8000;
                c.g[V0][0] = c.g[V0][0] << 45;
                c.sd((c.g[S6][0] as u32), c.g[V1][0]);
                c.sd((c.g[S6][0] as u32).wrapping_add(0x8), 0u64);
                c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = 0xe;
                c.sd((c.g[S6][0] as u32), c.g[A0][0]);
                c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 5) as u32);
                c.sd((c.g[S6][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[S4][0] = sx((c.g[S6][0] as u32).wrapping_add(0x10));
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10d8));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0036c48c);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x0036c48c)?;
                c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
                let t = c.g[V0][0] == 0u64;

                if t {
                    break 'b_36c4d4;
                } else {
                    c.cycles += 15;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10d8));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0036c4ac);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x0036c4ac)?;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
                    c.g[V1][0] = 0xffffffffff000000;
                    c.g[V1][0] = c.g[V1][0] << 16;
                    c.g[V1][0] = c.g[V1][0] | 0x100;
                    c.g[V1][0] = c.g[V1][0] << 16;
                    c.g[V0][0] = c.g[V0][0] << 32;
                    c.g[V0][0] = c.g[V0][0] >> 16;
                    c.g[V0][0] = c.g[S0][0] | c.g[V0][0];
                    let t = true;
                    c.g[S0][0] = c.g[V0][0] | c.g[V1][0];
                    if t {
                        break 'b_36c4e8;
                    } else {
                        break 'b_36c4d4;
                    }
                }
            }
            c.cycles += 5;
            c.g[V0][0] = 0xffffffffff000000;
            c.g[V0][0] = c.g[V0][0] << 16;
            c.g[V0][0] = c.g[V0][0] | 0x101;
            c.g[V0][0] = c.g[V0][0] << 16;
            c.g[S0][0] = c.g[S0][0] | c.g[V0][0];
            break 'b_36c4e8;
        }
        'b_36c6f4: {
            c.cycles += 100;
            c.g[V0][0] = 0xffffffffffffffff;
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x3f));
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S2][0] as i64)) as u64;
            c.g[T0][0] = 0x1;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = c.g[S2][0];
            }
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 6) as u32);
            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd8));
            c.g[V0][0] = ((0u64 as i64) < (c.g[V1][0] as i64)) as u64;
            c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[S3][0] as u32)));
            if c.g[V0][0] != 0 {
                c.g[T0][0] = c.g[V1][0];
            }
            c.g[T1][0] = c.g[A2][0] << 8;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd0));
            c.g[T1][0] = c.g[A2][0] | c.g[T1][0];
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[A0][0] = c.g[A0][0] << 16;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[V0][0]);
            c.g[A0][0] = c.g[S7][0] | c.g[A0][0];
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[A1][0] = 0x1000000;
            c.g[T0][0] = c.g[T0][0] << 14;
            c.g[T3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe0));
            c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            c.g[A1][0] = c.g[S5][0] | c.g[A1][0];
            c.g[V0][0] = c.g[A2][0] << 16;
            c.g[T2][0] = c.g[T2][0] << 34;
            c.g[T1][0] = c.g[T1][0] | c.g[V0][0];
            c.g[A2][0] = c.g[A2][0] << 24;
            c.g[V0][0] = c.g[V1][0] << 32;
            c.g[T1][0] = c.g[T1][0] | c.g[A2][0];
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
            c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
            c.g[V0][0] = c.lw((c.g[SP][0] as u32));
            c.g[A2][0] = 0xd502;
            c.g[A2][0] = c.g[A2][0] << 19;
            c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            c.g[V1][0] = 0xa;
            c.g[T0][0] = c.g[V0][0] | c.g[T0][0];
            c.g[T2][0] = c.g[T2][0] | c.g[V1][0];
            c.g[V0][0] = 0x8000;
            c.g[V0][0] = c.g[V0][0] << 17;
            c.g[V1][0] = 0x29;
            c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
            c.g[T0][0] = c.g[T0][0] | c.g[A2][0];
            c.g[T3][0] = c.g[T3][0] << 32;
            c.sd((c.g[S4][0] as u32), c.g[S0][0]);
            c.g[T5][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
            c.g[T3][0] = c.g[T3][0] | c.g[V1][0];
            c.g[V0][0] = 0x4c;
            c.g[A2][0] = 0x30000;
            c.sd((c.g[S6][0] as u32).wrapping_add(0x18), c.g[V0][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x20), c.g[A1][0]);
            c.g[A3][0] = c.g[A3][0] << 48;
            c.sd((c.g[S6][0] as u32).wrapping_add(0x30), c.g[A2][0]);
            c.g[T4][0] = 0xfe00;
            c.g[T4][0] = c.g[T4][0] << 46;
            c.g[V1][0] = 0x4e;
            c.g[T5][0] = c.g[T5][0] << 14;
            c.sd((c.g[S6][0] as u32).wrapping_add(0x28), c.g[V1][0]);
            c.g[A0][0] = c.g[A0][0] | c.g[A3][0];
            c.g[T5][0] = c.g[T5][0] | c.g[T2][0];
            c.g[T1][0] = c.g[T1][0] | c.g[T4][0];
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A3][0] = 0x47;
            c.g[T2][0] = 0x18;
            c.g[T4][0] = 0x42;
            c.g[T6][0] = 0x3f;
            c.g[T7][0] = 0x40;
            c.g[S0][0] = 0x60;
            c.g[S1][0] = 0x14;
            c.g[S2][0] = 0x6;
            c.g[S3][0] = 0x8;
            c.g[S4][0] = 0x156;
            c.g[S5][0] = 0x1;
            c.sd((c.g[S6][0] as u32).wrapping_add(0x38), c.g[A3][0]);
            c.g[T9][0] = sx((c.g[S7][0] as u32) << 4);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x48), c.g[T2][0]);
            c.g[S7][0] = 0x8;
            c.sd((c.g[S6][0] as u32).wrapping_add(0x50), c.g[T3][0]);
            c.g[T8][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8));
            c.sd((c.g[S6][0] as u32).wrapping_add(0x58), c.g[T4][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x68), c.g[T6][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x70), c.g[A0][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x78), c.g[T7][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x80), c.g[S0][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x88), c.g[S1][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x90), c.g[T0][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x98), c.g[S2][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0xa0), c.g[T5][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0xa8), c.g[S3][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0xb0), c.g[S4][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0xc0), c.g[T1][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0xc8), c.g[S5][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x40), 0u64);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x60), 0u64);
            c.sd((c.g[S6][0] as u32).wrapping_add(0xb8), 0u64);
            let t = (c.g[FP][0] as i64) <= 0;
            c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0xd0));
            if t {
                break 'b_36c6f4;
            } else {
                c.cycles += 12;
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                c.g[A2][0] = c.g[FP][0];
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[T1][0] = 0x3;
                c.g[V1][0] = sx((c.g[A1][0] as u32) << 4);
                c.g[T0][0] = 0x5;
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = sx((c.g[A0][0] as u32) << 4);
                c.g[A0][0] = sx((c.g[A1][0] as u32) << 4);
                c.g[T2][0] = c.g[V0][0] << 16;
                c.g[A3][0] = c.g[V1][0] << 16;
                c.g[T3][0] = c.g[A0][0] << 16;
                'l_36c6a8: loop {
                    c.cycles += 19;
                    c.g[A0][0] = c.g[T8][0] | c.g[A3][0];
                    c.g[V0][0] = c.g[T9][0] | c.g[T2][0];
                    c.sd((c.g[S6][0] as u32).wrapping_add(0x20), c.g[A0][0]);
                    c.g[V1][0] = sx((c.g[T9][0] as u32).wrapping_add(0x200));
                    c.sd((c.g[S6][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.g[A1][0] = c.g[V1][0] | c.g[T3][0];
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[T9][0] = c.g[V1][0];
                    c.sd((c.g[S6][0] as u32), c.g[S7][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                    c.sd((c.g[S6][0] as u32).wrapping_add(0x30), c.g[A1][0]);
                    c.g[T8][0] = sx((c.g[T8][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sd((c.g[S6][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sd((c.g[S6][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                    c.sd((c.g[S6][0] as u32).wrapping_add(0x28), c.g[T1][0]);
                    c.sd((c.g[S6][0] as u32).wrapping_add(0x38), c.g[T0][0]);
                    let t = c.g[A2][0] != 0u64;
                    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x40));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0036c6a8);
                        }
                        continue 'l_36c6a8;
                    } else {
                        break 'b_36c6f4;
                    }
                }
            }
        }
        c.cycles += 18;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
        c.g[A1][0] = c.g[S6][0];
        c.g[RA][0] = sx(0x0036c704);
        c.g[A2][0] = 0x4;
        Callee {
            addr: 0x0038f668,
            f: ee::s12::SprStream_EndBlockAligned,
        }
        .call(c, 0x0036c704)?;
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe8));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        c.sw((c.g[V1][0] as u32), c.g[V0][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
        c.pc = t;
        return Ok(());
    });
}

/// `void RailTube_Render(RailTube* t)` (0x3726c8): when the global enable
/// (gp+0x10e4) is set, build the two ribbon edges of the tube (the
/// normalised direction first to last point, crossed with the up vector at
/// 0x4ff160, scaled by the half widths at gp+0x10e8/0x10ec) into the
/// vertex buffer at 0x501440, push the render state, draw the ribbon through
/// the graphics manager's vtable slot 0x238, draw the centre line through slot
/// 0x248 and pop the state.
pub fn rail_tube_render(c: &mut Cpu) {
    guest(c, |c| {
        'b_372a88: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x10e4));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
            let t = c.g[V0][0] == 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[RA][0]);
            if t {
                break 'b_372a88;
            } else {
                'b_372910: {
                    c.cycles += 89;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = 0x500000;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = 0u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                    c.g[AT][0] = 0xffffffffbf800000;
                    c.mtc1(1, 1);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.lqc2(3, (c.g[V1][0] as u32));
                    c.lqc2(4, (c.g[A0][0] as u32));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().rsqrt(0, 3, 4, 0);
                    c.vu0().waitq();
                    c.vu0().mul(0b1111, 5, 3, Src::Q);
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(2, (c.g[SP][0] as u32));
                    c.lqc2(4, (c.g[SP][0] as u32));
                    c.lqc2(5, (c.g[A1][0] as u32).wrapping_sub(0xea0));
                    c.vu0().opmula(4, 5);
                    c.vu0().opmsub(6, 5, 4);
                    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                    c.sqc2(6, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().rsqrt(0, 3, 4, 0);
                    c.vu0().waitq();
                    c.vu0().mul(0b1111, 5, 3, Src::Q);
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x10e8));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.mfc1(3, 0);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.mfc1(3, 1);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x10ec));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.mfc1(3, 0);
                    c.lqc2(4, (c.g[A1][0] as u32).wrapping_sub(0xea0));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x10ec));
                    c.mfc1(3, 0);
                    c.lqc2(4, (c.g[A1][0] as u32).wrapping_sub(0xea0));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[V0][0] = 0x500000;
                    if t {
                        break 'b_372910;
                    } else {
                        c.cycles += 7;
                        c.g[T2][0] = 0x60;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1460));
                        c.g[T1][0] = 0x30;
                        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc0));
                        c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf0));
                        c.g[A3][0] = c.g[V0][0];
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        'l_372868: loop {
                            c.cycles += 42;
                            c.g[A1][0] = sx((c.g[A2][0] as u32) << 4);
                            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            c.lqc2(3, (c.g[V1][0] as u32));
                            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                            c.sq(2, (c.g[A3][0] as u32));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            c.lq(2, (c.g[V1][0] as u32));
                            c.sq(2, (c.g[A3][0] as u32).wrapping_add(0x30));
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x60));
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            c.mult(0, 2, 3, 10);
                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                            c.lq(2, (c.g[A0][0] as u32));
                            c.sq(2, (c.g[V1][0] as u32));
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                            c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.mult(0, 2, 4, 9);
                            c.lqc2(3, (c.g[A1][0] as u32));
                            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
                            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                            c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                            c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T3][0] as u32)));
                            c.sq(3, (c.g[A0][0] as u32));
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            if c.g[V0][0] != 0u64 {
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                if event_due(c) {
                                    deliver_event_here(c, 0x00372868);
                                }
                                continue 'l_372868;
                            } else {
                                break 'b_372910;
                            }
                        }
                    }
                }
                'b_372a54: {
                    c.cycles += 69;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[T1][0] = 0x60;
                    c.g[V1][0] = 0x500000;
                    c.g[S1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
                    c.mult(0, 5, 5, 9);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x14a0));
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x30));
                    c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x60));
                    c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60));
                    c.g[T2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A3][0] = 0u64;
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.lq(2, (c.g[A0][0] as u32));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sq(2, (c.g[A1][0] as u32));
                    c.lq(3, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.sq(3, (c.g[A1][0] as u32).wrapping_add(0x10));
                    c.lq(2, (c.g[A0][0] as u32).wrapping_add(0x20));
                    c.sq(2, (c.g[A1][0] as u32).wrapping_add(0x20));
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.mult(0, 4, 4, 9);
                    c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                    c.lq(2, (c.g[T0][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T2][0] as u32)));
                    c.sq(2, (c.g[A0][0] as u32));
                    c.lq(3, (c.g[T0][0] as u32).wrapping_add(0x10));
                    c.sq(3, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.lq(2, (c.g[T0][0] as u32).wrapping_add(0x20));
                    c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                    c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(4, (c.g[V1][0] as u32));
                    c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(5, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0x1b));
                    c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x14));
                    c.sdl(5, (c.g[V1][0] as u32).wrapping_add(0x23));
                    c.sdr(5, (c.g[V1][0] as u32).wrapping_add(0x1c));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x24), c.g[T0][0]);
                    c.g[V1][0] = 0x500000;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xe84), c.g[V0][0]);
                    c.g[T1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1420));
                    c.ldl(2, (c.g[T1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, (c.g[T1][0] as u32));
                    c.ldl(5, (c.g[T1][0] as u32).wrapping_add(0xf));
                    c.ldr(5, (c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x1b));
                    c.sdr(2, (c.g[A0][0] as u32).wrapping_add(0x14));
                    c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0x23));
                    c.sdr(5, (c.g[A0][0] as u32).wrapping_add(0x1c));
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x24), c.g[T0][0]);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10d8));
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x238));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x23c));
                    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00372a14);
                    c.g[A1][0] = c.g[A1][0] | 0x2;
                    call_ptr(c, t, 0x00372a14)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_372a54;
                    } else {
                        c.cycles += 1;
                        c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x20));
                        'l_372a28: loop {
                            c.cycles += 11;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[V1][0] = sx((c.g[A1][0] as u32) << 4);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.lq(2, (c.g[V1][0] as u32));
                            c.sq(2, (c.g[A2][0] as u32));
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x30));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00372a28);
                                }
                                continue 'l_372a28;
                            } else {
                                break 'b_372a54;
                            }
                        }
                    }
                }
                c.cycles += 13;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10d8));
                c.g[A2][0] = 0x500000;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1440));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x248));
                c.g[A3][0] = 0u64;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24c));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00372a7c);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00372a7c)?;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
                c.sw((c.g[S1][0] as u32).wrapping_add(0xe84), c.g[V0][0]);
                break 'b_372a88;
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

/// `void PS2BezierMan_EmitPatchType1(PS2BezierMan* m, int kind, int sub,
/// Patch* patch, uint32* handle)` (0x38ca70): copy the patch record into the
/// scratchpad (`SprStream_Upload`), decide from its flags and the render
/// toggles (gp+0x13e8/0x13ec) whether it is drawn at once (patch
/// header packet, ring entry, reflection and projector passes) or deferred to
/// the manager's defer list (+0x5364).
pub fn ps2_bezier_man_emit_patch_type1(c: &mut Cpu) {
    guest(c, |c| {
        'b_38cb18: {
            c.cycles += 41;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
            c.g[V1][0] = 0xc;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S1][0] = c.g[A2][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[S6][0] = c.g[A1][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.mult(0, 3, 17, 3);
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S2][0] = c.g[A0][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S0][0] = sx((c.g[S6][0] as u32) << 2);
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S0][0] as u32)));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = 0x1b0;
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S7][0] = c.g[T0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.g[FP][0] = c.g[A3][0];
            c.g[S5][0] = 0u64;
            c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
            c.g[A0][0] = c.lw((c.g[FP][0] as u32));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[SP][0] as u32), c.g[A2][0]);
            c.g[RA][0] = sx(0x0038cae0);
            c.g[A2][0] = 0x2;
            Callee {
                addr: 0x0038f708,
                f: ee::s12::SprStream_Upload,
            }
            .call(c, 0x0038cae0)?;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S0][0] as u32)));
            c.g[S4][0] = c.g[V0][0];
            c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xc));
            c.g[S1][0] = sx((c.g[S1][0] as u32) << 8);
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x44c));
            c.g[V1][0] = sx((c.g[S6][0] as u32) << 1);
            c.g[A0][0] = 0x600000;
            c.g[S3][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
            c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S1][0] as u32)));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
            let t = c.g[V0][0] == 0u64;
            c.g[T3][0] = 0u64;
            if t {
                break 'b_38cb18;
            } else {
                c.cycles += 1;
                c.g[T3][0] = (0u64 < c.g[V1][0]) as u64;
                break 'b_38cb18;
            }
        }
        'b_38cb2c: {
            c.cycles += 2;
            let t = c.g[T3][0] == 0u64;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x13e8));
            if t {
                break 'b_38cb2c;
            } else {
                c.cycles += 2;
                if c.g[V0][0] == 0u64 {
                    c.g[S5][0] = (0u64 < c.g[T3][0]) as u64;
                    break 'b_38cb2c;
                } else {
                    c.cycles += 1;
                    c.g[T3][0] = 0u64;
                    break 'b_38cb2c;
                }
            }
        }
        'b_38cb50: {
            c.cycles += 3;
            c.g[V0][0] = 0x2;
            let t = c.g[S6][0] != c.g[V0][0];
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
            if t {
                break 'b_38cb50;
            } else {
                c.cycles += 6;
                c.sq(11, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0038cb48);
                c.g[A1][0] = c.g[S4][0];
                Callee {
                    addr: 0x0038ca08,
                    f: ee::s12::PS2BezierMan_TestPatchBounds,
                }
                .call(c, 0x0038cb48)?;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.lq(11, (c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_38cb50;
            }
        }
        'b_38cb6c: {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[S1][0] = (0u64 < c.g[V0][0]) as u64;
            let t = c.g[S1][0] == 0u64;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x13ec));
            if t {
                break 'b_38cb6c;
            } else {
                c.cycles += 2;
                if c.g[V0][0] == 0u64 {
                    c.g[S5][0] = c.g[S5][0] | c.g[S1][0];
                    break 'b_38cb6c;
                } else {
                    c.cycles += 1;
                    c.g[S1][0] = 0u64;
                    break 'b_38cb6c;
                }
            }
        }
        'b_38cc04: {
            'b_38cc00: {
                'b_38cbf8: {
                    'b_38cbf4: {
                        'b_38cbe0: {
                            'b_38cbc8: {
                                c.cycles += 6;
                                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xc));
                                c.g[A1][0] = 0x180000;
                                c.g[A0][0] = 0x100000;
                                c.g[V1][0] = c.g[V0][0] & c.g[A1][0];
                                if c.g[V1][0] == c.g[A0][0] {
                                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe84));
                                    break 'b_38cbc8;
                                } else {
                                    'b_38cbb0: {
                                        'b_38cba0: {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[V0][0] = 0x80000;
                                            if t {
                                                break 'b_38cba0;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[V1][0] == c.g[V0][0];
                                                c.g[A0][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0xe84));
                                                if t {
                                                    break 'b_38cbb0;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[V1][0] = 0xfffffffffffffff3;
                                                    if t {
                                                        break 'b_38cbf8;
                                                    } else {
                                                        break 'b_38cba0;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        if c.g[V1][0] == c.g[A1][0] {
                                            c.g[V1][0] =
                                                c.lw((c.g[S3][0] as u32).wrapping_add(0xe84));
                                            break 'b_38cbe0;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[A0][0] =
                                                c.lw((c.g[S3][0] as u32).wrapping_add(0xe84));
                                            if t {
                                                break 'b_38cbf4;
                                            } else {
                                                break 'b_38cbb0;
                                            }
                                        }
                                    }
                                    c.cycles += 6;
                                    c.g[V1][0] = 0xfffffffffffffff3;
                                    c.g[S5][0] = 0x1;
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                    let t = true;
                                    c.g[V0][0] = c.g[V0][0] | 0x8;
                                    if t {
                                        break 'b_38cc00;
                                    } else {
                                        break 'b_38cbc8;
                                    }
                                }
                            }
                            c.cycles += 6;
                            c.g[V1][0] = 0xfffffffffffffff3;
                            c.g[S5][0] = 0x1;
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                            let t = true;
                            c.g[V0][0] = c.g[V0][0] | 0x4;
                            if t {
                                break 'b_38cc00;
                            } else {
                                break 'b_38cbe0;
                            }
                        }
                        c.cycles += 5;
                        c.g[S5][0] = 0x1;
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                        c.g[V0][0] = c.g[V0][0] | 0xc;
                        let t = true;
                        c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                        if t {
                            break 'b_38cc04;
                        } else {
                            break 'b_38cbf4;
                        }
                    }
                    c.cycles += 1;
                    c.g[V1][0] = 0xfffffffffffffff3;
                    break 'b_38cbf8;
                }
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                break 'b_38cc00;
            }
            c.cycles += 1;
            c.sw((c.g[A0][0] as u32), c.g[V0][0]);
            break 'b_38cc04;
        }
        'b_38cdf0: {
            'b_38cdec: {
                'b_38cd88: {
                    c.cycles += 2;
                    let t = c.g[S5][0] == 0u64;
                    c.g[A2][0] = c.g[S0][0] << 32;
                    if t {
                        break 'b_38cd88;
                    } else {
                        'b_38cd30: {
                            c.cycles += 41;
                            c.g[A0][0] = c.lh((c.g[S4][0] as u32).wrapping_add(0x1a0));
                            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe84));
                            c.g[V1][0] = 0x50000000;
                            c.g[A2][0] = c.g[A2][0] | c.g[V1][0];
                            c.g[T4][0] = 0x30000000;
                            c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                            c.g[T0][0] = 0x10000000;
                            c.g[T0][0] = c.g[T0][0] | 0x1;
                            c.g[V0][0] = 0x450000;
                            c.g[A1][0] = sx((c.g[S6][0] as u32) << 2);
                            c.g[A0][0] = c.lh((c.g[S4][0] as u32).wrapping_add(0x1a2));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4630));
                            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe84));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[T1][0] = 0x14000000;
                            c.g[A3][0] = 0xfff0000;
                            c.sh((c.g[V1][0] as u32).wrapping_add(0x12), c.g[A0][0]);
                            c.g[A3][0] = c.g[A3][0] | 0xffff;
                            c.g[T2][0] = c.lw((c.g[S7][0] as u32));
                            c.g[A0][0] = c.g[T2][0] | c.g[T4][0];
                            c.sd((c.g[A0][0] as u32), c.g[A2][0]);
                            c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                            c.sd((c.g[A0][0] as u32).wrapping_add(0x8), 0u64);
                            c.sd((c.g[A0][0] as u32).wrapping_add(0x18), 0u64);
                            c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x4));
                            c.g[S0][0] = c.lw((c.g[A1][0] as u32));
                            c.sd((c.g[A0][0] as u32).wrapping_add(0x20), 0u64);
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                            c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
                            c.g[V0][0] = c.g[V0][0] << 32;
                            c.sd((c.g[A0][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30));
                            c.g[T1][0] = c.g[A0][0] & c.g[A3][0];
                            c.sw((c.g[S7][0] as u32), c.g[T1][0]);
                            c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18f0));
                            c.g[V0][0] = c.lw((c.g[A3][0] as u32));
                            c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[T0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe84));
                            if t {
                                break 'b_38cd30;
                            } else {
                                c.cycles += 32;
                                c.g[A1][0] = 0x60000;
                                c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = c.g[A1][0] | 0x8000;
                                c.g[V1][0] = 0xffffffffffffffe0;
                                c.g[A1][0] =
                                    sx((c.g[A3][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
                                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc4));
                                c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x80));
                                c.g[A2][0] = c.g[A2][0] | c.g[T4][0];
                                c.g[V0][0] = c.g[V0][0] & 0x1f;
                                c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
                                c.sw((c.g[T0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                                c.g[V1][0] = c.lw((c.g[A3][0] as u32));
                                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc0));
                                c.g[V0][0] = sx((c.g[V1][0] as u32) << 7);
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                                c.sw((c.g[A3][0] as u32), c.g[V1][0]);
                                c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
                                c.ldr(3, (c.g[T0][0] as u32));
                                c.ldl(5, (c.g[T0][0] as u32).wrapping_add(0xf));
                                c.ldr(5, (c.g[T0][0] as u32).wrapping_add(0x8));
                                c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
                                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
                                c.sdr(3, (c.g[V0][0] as u32));
                                c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0xf));
                                c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x8));
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[T2][0]);
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                                c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
                                c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), 0u64);
                                break 'b_38cd30;
                            }
                        }
                        'b_38cd60: {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[S7][0] as u32));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                            let t = c.g[T3][0] == 0u64;
                            c.sw((c.g[S7][0] as u32), c.g[V0][0]);
                            if t {
                                break 'b_38cd60;
                            } else {
                                c.cycles += 8;
                                c.g[A1][0] = c.lw((c.g[SP][0] as u32));
                                c.g[A2][0] = c.g[S4][0];
                                c.g[A0][0] = c.g[S2][0];
                                c.g[A3][0] = c.g[S6][0];
                                c.g[T0][0] = c.g[S0][0];
                                c.g[T1][0] = c.g[FP][0];
                                c.g[RA][0] = sx(0x0038cd60);
                                c.g[T2][0] = c.g[S7][0];
                                Callee {
                                    addr: 0x0038d168,
                                    f: ee::s12::PS2BezierMan_EmitPatchMatrixPass,
                                }
                                .call(c, 0x0038cd60)?;
                                break 'b_38cd60;
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[S1][0] == 0u64;
                        c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                        if t {
                            break 'b_38cdec;
                        } else {
                            c.cycles += 8;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[A1][0] = c.g[S6][0];
                            c.g[A2][0] = c.g[S0][0];
                            c.g[A3][0] = c.g[FP][0];
                            c.g[RA][0] = sx(0x0038cd80);
                            c.g[T1][0] = c.g[S7][0];
                            Callee {
                                addr: 0x0038d448,
                                f: ee::s12::PS2BezierMan_EmitPatchProjectorPasses,
                            }
                            .call(c, 0x0038cd80)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                            if t {
                                break 'b_38cdf0;
                            } else {
                                break 'b_38cd88;
                            }
                        }
                    }
                }
                c.cycles += 25;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5364));
                c.g[A0][0] = c.lhu((c.g[S4][0] as u32).wrapping_add(0x1a0));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sh((c.g[V0][0] as u32).wrapping_add(0x5368), c.g[A0][0]);
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5364));
                c.g[A0][0] = c.lhu((c.g[S4][0] as u32).wrapping_add(0x1a2));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.sh((c.g[V1][0] as u32).wrapping_add(0x536a), c.g[A0][0]);
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5364));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x536c), c.g[S6][0]);
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5364));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x5370), c.g[S0][0]);
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5364));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x5374), c.g[FP][0]);
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5364));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                c.sw((c.g[S2][0] as u32).wrapping_add(0x5364), c.g[V1][0]);
                break 'b_38cdec;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
            break 'b_38cdf0;
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{check, fill, put_vec, scramble, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The scratchpad stream (gp+0x1384): two slots, buffers in RAM.
    const OC1064_STREAM: u32 = DATA + 0x7_0000;

    /// Point gp+0x1384 at a two-slot stream whose blocks live in RAM (as the
    /// `DeferMan_SortBuckets` fixture does); the real stream callees run.
    fn oc1064_stream(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(GAME_GP + 0x1384, OC1064_STREAM);
        let slot = r.next_u32() % 2;
        c.bus.write32(OC1064_STREAM + 4, 2);
        c.bus.write32(OC1064_STREAM + 8, slot);
        c.bus.write32(OC1064_STREAM + 0xc, slot);
        c.bus.write32(OC1064_STREAM + 0x10, 0x101);
        c.bus.write32(OC1064_STREAM + 0x14, DATA + 0x7_1000);
        c.bus.write32(OC1064_STREAM + 0x18, DATA + 0x7_3000);
        fill(c, r, DATA + 0x7_1000, 0x400);
        fill(c, r, DATA + 0x7_3000, 0x400);
    }

    /// A rectangle width in pixels: none, less than a column, or several.
    fn oc1064_width(r: &mut Rng, seed: u32) -> u32 {
        match seed % 6 {
            0 => 0,
            1 => 1 + r.next_u32() % 31,
            _ => r.next_u32() % 700,
        }
    }

    /// `PS2GraphicsMan_EmitDepthFogFlatFill`: a manager with one of four saved
    /// rectangles (no columns, part of one, several), the fog colour floats,
    /// the two virtual size queries as fakes (forced results; some calls
    /// unwind) and the packet cursor `*pp`.
    #[test]
    fn ps2_graphics_man_emit_depth_fog_flat_fill_matches() {
        const GM: u32 = DATA;
        const VT: u32 = DATA + 0x7000;
        const PP: u32 = DATA + 0x7800;
        const BUF: u32 = DATA + 0x8000;
        covered(
            ee::s12::f_0036b5e8__generated,
            ps2_graphics_man_emit_depth_fog_flat_fill,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0036_b5e8, 6);
                c.pc = 0x0036_b5e8;
                let idx = r.next_u32() % 4;
                c.bus.write32(GM + 0x10d8, VT);
                for slot in [0x50, 0x58] {
                    c.bus.write16(VT + slot, (r.next_u32() as u16) & 0xfc);
                }
                c.bus.write32(VT + 0x54, FAKE);
                c.bus.write32(VT + 0x5c, FAKE + 0x10);
                let rect = GM + idx * 8;
                c.bus.write16(rect + 0x59b0, r.next_u32() as u16 % 640);
                c.bus.write16(rect + 0x59b2, r.next_u32() as u16 % 640);
                let w = oc1064_width(&mut r, seed);
                c.bus.write16(rect + 0x59b4, w as u16);
                c.bus.write16(rect + 0x59b6, r.next_u32() as u16 % 480);
                c.bus.write32(GM + 0x5a80, r.next_u32());
                c.bus.write32(GM + 0x5a84, r.next_u32());
                let fog = GM + idx * 0x20 + 0x6c64;
                for off in [0x14, 0x18, 0x1c] {
                    let v = if seed % 5 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(2.0)
                    };
                    c.bus.write32(fog + off, v);
                }
                for k in 0..6 {
                    c.bus.write32(LOG_RESULTS + 4 * k, r.next_u32() % 0x1400);
                }
                c.bus.write32(PP, BUF);
                fill(c, &mut r, BUF, 0x20);
                c.g[A0][0] = sx(GM);
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = sx(PP);
            },
        );
    }

    /// `PS2GraphicsMan_EmitBlendedFill`: a stream block with random strip
    /// width (none, partial, several columns; a few negative), position,
    /// alpha and grey; the real stream callees run.
    #[test]
    fn ps2_graphics_man_emit_blended_fill_matches() {
        const HANDLE: u32 = DATA + 0x6800;
        check(
            ee::s12::f_0036c188__generated,
            ps2_graphics_man_emit_blended_fill,
            200,
            &|c, seed| {
                let mut r = Rng(0x0036_c188 ^ (seed as u64) << 13);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                oc1064_stream(c, &mut r);
                c.bus
                    .write32(HANDLE, DATA + 0x7_8000 + (r.next_u32() & 0xff0));
                let w = if seed % 11 == 7 {
                    (r.next_u32() % 100).wrapping_neg()
                } else {
                    oc1064_width(&mut r, seed)
                };
                c.g[A1][0] = (r.next_u32() % 640) as u64;
                c.g[A2][0] = sx(w);
                c.g[A3][0] = (r.next_u32() % 480) as u64;
                c.g[T0][0] = (r.next_u32() % 480) as u64;
                c.g[T1][0] = (r.next_u32() & 0xff) as u64;
                c.g[T2][0] = sx(HANDLE);
            },
        );
    }

    /// `PS2GraphicsMan_EmitBloomComposite`: the width query is the real leaf at
    /// 0x1406c0 (returns 0: the frame word without a width) or a fake with a
    /// forced result; arguments in registers and on the caller's stack.
    #[test]
    fn ps2_graphics_man_emit_bloom_composite_matches() {
        const GM: u32 = DATA;
        const VT: u32 = DATA + 0x6000;
        const HANDLE: u32 = DATA + 0x6800;
        const STACK: u32 = DATA + 0xa_0000;
        covered(
            ee::s12::f_0036c398__generated,
            ps2_graphics_man_emit_bloom_composite,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0036_c398, 2);
                c.pc = 0x0036_c398;
                oc1064_stream(c, &mut r);
                c.g[SP][0] = sx(STACK);
                c.bus.write32(GM + 0x10d8, VT);
                c.bus.write16(VT + 0x50, (r.next_u32() as u16) & 0xfc);
                c.bus
                    .write32(VT + 0x54, if seed % 3 == 0 { 0x0014_06c0 } else { FAKE });
                c.bus.write32(GM + 0x5a80, r.next_u32());
                for k in 0..2 {
                    c.bus
                        .write32(LOG_RESULTS + 4 * k, 0x40 | (r.next_u32() % 0x1000));
                }
                c.bus
                    .write32(HANDLE, DATA + 0x7_8000 + (r.next_u32() & 0xff0));
                for off in [0, 8, 0x10] {
                    c.bus.write32(STACK + off, r.next_u32());
                }
                c.bus.write32(STACK + 0x18, HANDLE);
                let w = if seed % 11 == 7 {
                    (r.next_u32() % 100).wrapping_neg()
                } else {
                    oc1064_width(&mut r, seed)
                };
                c.g[A0][0] = sx(GM);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = (r.next_u32() % 1024) as u64;
                c.g[A3][0] = (r.next_u32() % 480) as u64;
                c.g[T0][0] = (r.next_u32() % 640) as u64;
                c.g[T1][0] = (r.next_u32() % 480) as u64;
                c.g[T2][0] = (r.next_u32() % 480) as u64;
                c.g[T3][0] = sx(w);
            },
        );
    }

    /// `RailTube_Render`: tubes of 0..7 points (and an empty one, n = -1), the
    /// global enable off for some seeds, the two draw calls through the
    /// manager's vtable as fakes (some unwind); the unit vectors come from
    /// random points (VU0 maths on both sides).
    #[test]
    fn rail_tube_render_matches() {
        const TUBE: u32 = DATA + 0x1000;
        const PTS: u32 = DATA + 0x2000;
        const GM: u32 = DATA + 0x4000;
        const VT: u32 = DATA + 0x6000;
        const TOP: u32 = DATA + 0x6800;
        covered(
            ee::s12::f_003726c8__generated,
            rail_tube_render,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0037_26c8, 2);
                c.pc = 0x0037_26c8;
                let n = (r.next_u32() % 8) as i32 - 1;
                c.bus.write32(TUBE + 4, n as u32);
                c.bus.write32(TUBE + 8, PTS);
                for i in 0..8 {
                    let p = [r.f32(50.0), r.f32(50.0), r.f32(50.0), 0x3f80_0000];
                    put_vec(c, PTS + 0x10 * i, p);
                }
                c.bus.write32(
                    GAME_GP + 0x10e4,
                    if seed % 5 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(GAME_GP + 0x10e8, r.f32(3.0));
                c.bus.write32(GAME_GP + 0x10ec, r.f32(3.0));
                put_vec(
                    c,
                    0x004f_f160,
                    [r.f32(1.0), r.f32(1.0), r.f32(1.0), r.f32(1.0)],
                );
                fill(c, &mut r, 0x0050_1420, 8);
                c.bus.write32(GAME_GP + 0x2a90, GM);
                c.bus.write32(GM + 0xe84, TOP);
                c.bus.write32(GM + 0x10d8, VT);
                fill(c, &mut r, TOP, 0x10);
                for slot in [0x238, 0x248] {
                    c.bus.write16(VT + slot, (r.next_u32() as u16) & 0xfc);
                }
                c.bus.write32(VT + 0x23c, FAKE);
                c.bus.write32(VT + 0x24c, FAKE + 0x10);
                c.g[A0][0] = sx(TUBE);
            },
        );
    }

    // ------------------------------------------------ PS2BezierMan_EmitPatchType1

    const OC1064_CTRL: u32 = DATA + 0xf_0000;
    const OC1064_MAGIC: u32 = 0x1064_c0de;

    /// Fake `k` (0 = TestPatchBounds, 1 = EmitPatchMatrixPass, 2 =
    /// EmitPatchProjectorPasses): gate, call counter and forced result of
    /// call `n` live in the machine's RAM; the call logs its arguments.
    fn oc1064_gate(k: u32) -> u32 {
        OC1064_CTRL + 0x400 * k
    }

    fn oc1064_fake_call(c: &mut Cpu, k: u32) {
        let g = oc1064_gate(k);
        let n = c.bus.read32(g + 4);
        c.bus.write32(g + 4, n + 1);
        let args = [A0, A1, A2, A3, T0, T1, T2];
        for (i, reg) in args.into_iter().enumerate() {
            let v = c.g[reg][0] as u32;
            c.bus.write32(g + 0x200 + 0x20 * (n % 8) + 4 * i as u32, v);
        }
        let v = c.bus.read32(g + 0x100 + 4 * (n % 8));
        c.g[V0][0] = sx(v);
        c.cycles += 5;
        c.pc = c.g[RA][0] as u32;
    }

    static OC1064_PREV_BOUNDS: AtomicUsize = AtomicUsize::new(0);
    static OC1064_PREV_MATRIX: AtomicUsize = AtomicUsize::new(0);
    static OC1064_PREV_PROJECTOR: AtomicUsize = AtomicUsize::new(0);

    fn oc1064_fake_bounds(c: &mut Cpu) {
        if c.bus.read32(oc1064_gate(0)) != OC1064_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                &OC1064_PREV_BOUNDS,
                ee::s12::f_0038ca08__generated,
            );
        }
        oc1064_fake_call(c, 0)
    }

    fn oc1064_fake_matrix(c: &mut Cpu) {
        if c.bus.read32(oc1064_gate(1)) != OC1064_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                &OC1064_PREV_MATRIX,
                ee::s12::f_0038d168__generated,
            );
        }
        oc1064_fake_call(c, 1)
    }

    fn oc1064_fake_projector(c: &mut Cpu) {
        if c.bus.read32(oc1064_gate(2)) != OC1064_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                &OC1064_PREV_PROJECTOR,
                ee::s12::f_0038d448__generated,
            );
        }
        oc1064_fake_call(c, 2)
    }

    fn oc1064_hook_all() {
        use crate::difftest::fakes::hook;
        hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038CA08),
            oc1064_fake_bounds,
            &OC1064_PREV_BOUNDS,
        );
        hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038D168),
            oc1064_fake_matrix,
            &OC1064_PREV_MATRIX,
        );
        hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038D448),
            oc1064_fake_projector,
            &OC1064_PREV_PROJECTOR,
        );
    }

    /// `PS2BezierMan_EmitPatchType1`: the patch record is uploaded to the
    /// scratchpad by the real `SprStream_Upload`; its flags pick the four
    /// render-state cases (and the reflection/shadow passes, whose callees
    /// are gated fakes with random results), the gp toggles disable the
    /// passes, the ring (count below or at its 2600 limit) takes an entry;
    /// patches that are not drawn go to the defer list.
    #[test]
    fn ps2_bezier_man_emit_patch_type1_matches() {
        const MAN: u32 = DATA;
        const ARGS: u32 = DATA + 0x2_0000;
        const SRC: u32 = DATA + 0x2_1000;
        const HANDLE: u32 = DATA + 0x2_2000;
        const GM: u32 = DATA + 0x3_0000;
        const TOP: u32 = DATA + 0x3_4000;
        const TABLE: u32 = DATA + 0x3_8000;
        const RING: u32 = DATA + 0x4_0000;
        oc1064_hook_all();
        check(
            ee::s12::f_0038ca70__generated,
            ps2_bezier_man_emit_patch_type1,
            300,
            &|c, seed| {
                let mut r = Rng(0x0038_ca70 ^ (seed as u64) << 11);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let kind = r.next_u32() % 4;
                for (k, on) in [(0, true), (1, seed % 4 != 0), (2, seed % 3 != 0)] {
                    let g = oc1064_gate(k);
                    c.bus.write32(g, if on { OC1064_MAGIC } else { 0 });
                    c.bus.write32(g + 4, 0);
                    for n in 0..8 {
                        c.bus.write32(g + 0x100 + 4 * n, r.next_u32() % 3);
                    }
                }
                // Patch record in RAM; the scratchpad copy lands at gp+0x2a94.
                fill(c, &mut r, SRC, 0x6c);
                let flags = [0, 0x80000, 0x100000, 0x180000][(r.next_u32() % 4) as usize]
                    | [0, 0x200000, 0x400000, 0x600000][(r.next_u32() % 4) as usize];
                c.bus.write32(SRC + 0xc, flags);
                c.bus
                    .write32(GAME_GP + 0x2a94, 0x7000_0000 + (r.next_u32() % 0x30) * 0x10);
                c.bus.write32(ARGS, SRC);
                c.bus.write32(ARGS + 4, r.next_u32() % 4);
                // The manager: cache entries, texture bases, defer list.
                fill(c, &mut r, MAN + 0x28, 8);
                fill(c, &mut r, MAN + 0x44c, 8);
                c.bus.write32(MAN + 0x5364, r.next_u32() % 8);
                // Render state ring of the graphics manager.
                c.bus.write32(GAME_GP + 0x2a90, GM);
                c.bus.write32(GM + 0xe84, TOP);
                c.bus.write32(GM + 0x18f0, RING);
                fill(c, &mut r, TOP, 8);
                c.bus.write32(
                    RING,
                    if seed % 7 == 3 {
                        2600
                    } else {
                        r.next_u32() % 4
                    },
                );
                c.bus.write32(RING + 0x69cc0, r.next_u32());
                c.bus.write32(RING + 0x69cc4, r.next_u32());
                // Toggles and the per-kind tables.
                c.bus
                    .write32(GAME_GP + 0x13e8, if seed % 3 == 0 { 0 } else { 1 });
                c.bus
                    .write32(GAME_GP + 0x13ec, if seed % 4 == 1 { 0 } else { 1 });
                for k in 0..4 {
                    c.bus.write32(0x0044_b9d0 + 4 * k, TABLE + 0x100 * k);
                    fill(c, &mut r, TABLE + 0x100 * k, 0x40);
                }
                c.bus
                    .write32(HANDLE, DATA + 0x3_c000 + (r.next_u32() & 0xff0));
                c.g[A0][0] = sx(MAN);
                c.g[A1][0] = kind as u64;
                c.g[A2][0] = (r.next_u32() % 4) as u64;
                c.g[A3][0] = sx(ARGS);
                c.g[T0][0] = sx(HANDLE);
            },
        );
    }
}
