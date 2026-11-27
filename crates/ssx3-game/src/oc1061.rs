//! Staging module oc1061 (wave wave265): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1061`.

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
    0x0039_05e8,
    ps2_graphics_man_emit_color_filter_packet,
    "PS2GraphicsMan_EmitColorFilterPacket",
)];

/// `void PS2GraphicsMan_EmitColorFilterPacket(PS2GraphicsMan* self, int idx, u64** cursor)`
/// (0x3905e8): appends one GIF packet to the DMA cursor `*cursor` that draws a
/// full-screen colour filter for screen slice `idx`: a frame/zbuf/test setup,
/// the filter's texture-window strips (one sprite pair per 32 pixels of
/// width), an extra blend pass when any of the filter's RGB floats (at
/// `self + 0x6ca4 + idx * 0x18 + 0xc..0x14`) scales to a positive byte, and the
/// closing restore of the display frame, with the packet's DMA tag at the old
/// cursor patched with the final qword count. The frame buffer geometry is
/// asked from the `self->+0x10d8` object's vtable (+0x54, +0x5c).
///
/// Transcribed block for block from the generated body (every scratch register
/// and the stack frame are reproduced); the three `s`-register loops keep their
/// event checks.
pub fn ps2_graphics_man_emit_color_filter_packet(c: &mut Cpu) {
    guest(c, |c| {
        'b_390724: {
            'b_39071c: {
                'b_390718: {
                    c.cycles += 70;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
                    c.g[V1][0] = 0x18;
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.mult(0, 3, 5, 3);
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.g[S3][0] = c.g[A0][0];
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.g[A1][0] = sx((c.g[A1][0] as u32) << 3);
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca4));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[S4][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                    c.g[V1][0] = 0x1000;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
                    c.g[AT][0] = 0x42ff0000;
                    c.mtc1(1, 2);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x5a84));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x5a80));
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A0][0]);
                    c.g[A0][0] = c.g[V0][0];
                    c.lwc1(0, (c.g[S4][0] as u32).wrapping_add(0xc));
                    c.g[S0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x59b6));
                    c.mul_s(0, 0, 2);
                    c.g[S1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x59b4));
                    c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x59b0));
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[S0][0] as u32)));
                    c.lwc1(3, (c.g[S4][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[S1][0] as u32)));
                    c.lwc1(1, (c.g[S4][0] as u32).wrapping_add(0x14));
                    c.cvt_w_s(4, 0);
                    c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x54));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.mul_s(3, 3, 2);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 31);
                    c.mul_s(1, 1, 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A1][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x59b2));
                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 1) as u32);
                    c.cvt_w_s(0, 3);
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x58));
                    c.g[S5][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[A1][0]);
                    c.cvt_w_s(0, 1);
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x5c));
                    c.g[A1][0] = sx((c.g[A0][0] as u32) >> 31);
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1f));
                    c.g[A1][0] = 0xffffffffffffffff;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                    c.g[A1][0] = ((c.g[A1][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                    if c.g[A1][0] != 0 {
                        c.g[V0][0] = c.g[S1][0];
                    }
                    c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 1) as u32);
                    c.g[S6][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x54));
                    c.g[FP][0] = sx((c.g[V1][0] as u32) << 4);
                    c.g[S7][0] = sx((c.g[A0][0] as u32) << 4);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S5][0]);
                    c.sw((c.g[SP][0] as u32), c.g[A2][0]);
                    let t = (c.g[V0][0] as i64) > 0;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x60), 0u64);
                    if t {
                        break 'b_390718;
                    } else {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x58));
                        let t = (c.g[V1][0] as i64) > 0;
                        c.g[A1][0] = 0x1;
                        if t {
                            break 'b_39071c;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x5c));
                            let t = (c.g[A0][0] as i64) <= 0;
                            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x60));
                            if t {
                                break 'b_390724;
                            } else {
                                break 'b_390718;
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[A1][0] = 0x1;
                break 'b_39071c;
            }
            c.cycles += 2;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x60), c.g[A1][0]);
            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x60));
            break 'b_390724;
        }
        'b_3909c4: {
            c.cycles += 123;
            c.g[V1][0] = sx((c.g[S6][0] as u32) << 1);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3));
            c.g[A3][0] = c.lw((c.g[SP][0] as u32));
            c.mult(0, 3, 3, 6);
            c.g[V0][0] = sx((c.g[S6][0] as u32) << 2);
            c.g[S2][0] = c.lw((c.g[A3][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xa));
            c.g[A1][0] = 0x8000;
            c.g[A1][0] = c.g[A1][0] << 45;
            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = 0xe;
            c.g[V0][0] = c.g[V0][0].wrapping_add(c.g[V1][0]);
            c.sd((c.g[S2][0] as u32).wrapping_add(0x8), c.g[A0][0]);
            c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
            c.g[S0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[S0][0] as u32)));
            c.sd((c.g[S2][0] as u32), c.g[V0][0]);
            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10d8));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x0039077c)?;
            c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = 0x1000000;
            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
            c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[S1][0] as u32)));
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
            c.g[V0][0] = c.g[V0][0] << 32;
            c.g[A1][0] = c.g[A1][0] << 16;
            c.g[A0][0] = c.g[V1][0] | c.g[A0][0];
            c.g[A1][0] = c.g[A3][0] | c.g[A1][0];
            c.g[A0][0] = c.g[A0][0] << 32;
            c.g[A3][0] = c.g[A2][0] << 32;
            c.g[V0][0] = c.g[V0][0] >> 16;
            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A3][0] = c.g[A3][0] >> 32;
            c.g[A0][0] = c.g[A0][0] >> 32;
            c.g[A3][0] = c.g[A3][0] | c.g[V0][0];
            c.g[V1][0] = c.g[A2][0] << 32;
            c.g[T0][0] = c.g[S7][0] << 32;
            c.g[A1][0] = c.g[A1][0] | c.g[V1][0];
            c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            c.g[A2][0] = c.g[A2][0] << 48;
            c.g[V1][0] = 0x8000;
            c.g[V1][0] = c.g[V1][0] << 17;
            c.g[A0][0] = c.g[A0][0] | c.g[V1][0];
            c.g[A1][0] = c.g[A1][0] | c.g[A2][0];
            c.g[A2][0] = 0x30000;
            c.g[T0][0] = c.g[FP][0] | c.g[T0][0];
            c.g[V0][0] = 0x4c;
            c.g[V1][0] = 0x4e;
            c.g[T1][0] = 0x47;
            c.g[T2][0] = 0x18;
            c.g[T3][0] = 0x40;
            c.g[T4][0] = 0x5;
            c.g[T5][0] = 0x8;
            c.g[T6][0] = 0x14;
            c.sd((c.g[S2][0] as u32).wrapping_add(0x10), c.g[A3][0]);
            c.sd((c.g[S2][0] as u32).wrapping_add(0x30), c.g[A2][0]);
            c.sd((c.g[S2][0] as u32).wrapping_add(0x50), c.g[A1][0]);
            c.sd((c.g[S2][0] as u32).wrapping_add(0x18), c.g[V0][0]);
            c.sd((c.g[S2][0] as u32).wrapping_add(0x20), c.g[A0][0]);
            c.sd((c.g[S2][0] as u32).wrapping_add(0x28), c.g[V1][0]);
            c.sd((c.g[S2][0] as u32).wrapping_add(0x38), c.g[T1][0]);
            c.sd((c.g[S2][0] as u32).wrapping_add(0x40), c.g[T0][0]);
            c.sd((c.g[S2][0] as u32).wrapping_add(0x48), c.g[T2][0]);
            c.sd((c.g[S2][0] as u32).wrapping_add(0x58), c.g[T3][0]);
            c.sd((c.g[S2][0] as u32).wrapping_add(0x60), c.g[T4][0]);
            c.sd((c.g[S2][0] as u32).wrapping_add(0x68), c.g[T5][0]);
            c.sd((c.g[S2][0] as u32).wrapping_add(0x78), c.g[T6][0]);
            c.sd((c.g[S2][0] as u32).wrapping_add(0x70), 0u64);
            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10d8));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x0039085c)?;
            c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
            c.g[V0][0] = c.g[V0][0] << 32;
            c.g[A0][0] = 0xd500;
            c.g[A0][0] = c.g[A0][0] << 19;
            c.g[V1][0] = sx((c.g[A3][0] as u32) << 5);
            c.g[V0][0] = c.g[V0][0] >> 18;
            c.g[V1][0] = c.g[V1][0] << 32;
            c.g[A1][0] = 0x116;
            c.g[V1][0] = c.g[V1][0] >> 32;
            c.sd((c.g[S2][0] as u32).wrapping_add(0x90), c.g[A1][0]);
            c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
            c.sd((c.g[S2][0] as u32).wrapping_add(0x98), 0u64);
            c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
            c.g[V0][0] = 0x6;
            c.sd((c.g[S2][0] as u32).wrapping_add(0x80), c.g[V1][0]);
            c.g[A2][0] = 0x3f800000;
            c.g[A2][0] = c.g[A2][0] << 16;
            c.g[A2][0] = c.g[A2][0] | 0x8000;
            c.g[A2][0] = c.g[A2][0] << 16;
            c.sd((c.g[S2][0] as u32).wrapping_add(0x88), c.g[V0][0]);
            c.g[V1][0] = 0x1;
            c.g[AT][0] = 0x42ff0000;
            c.mtc1(1, 1);
            c.lwc1(2, (c.g[S4][0] as u32).wrapping_add(0x4));
            c.lwc1(3, (c.g[S4][0] as u32).wrapping_add(0x8));
            c.mul_s(2, 2, 1);
            c.lwc1(0, (c.g[S4][0] as u32));
            c.mul_s(3, 3, 1);
            c.sd((c.g[S2][0] as u32).wrapping_add(0xa8), c.g[V1][0]);
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
            c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
            c.sd((c.g[S2][0] as u32).wrapping_add(0xa0), c.g[V0][0]);
            let t = c.g[S6][0] == 0u64;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0xb0));
            if t {
                break 'b_3909c4;
            } else {
                c.cycles += 24;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[A3][0] = sx((c.g[S5][0] as u32) << 4);
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[T1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = sx((c.g[V0][0] as u32) << 4);
                c.g[T5][0] = 0x3;
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 4);
                c.g[T4][0] = 0xffff;
                c.g[T4][0] = c.g[T4][0] << 32;
                c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[V1][0] = sx((c.g[S0][0] as u32) << 4);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[T7][0] = c.g[V0][0] << 16;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x200));
                c.g[S4][0] = c.g[A0][0] << 16;
                c.g[S1][0] = c.g[A1][0] << 16;
                c.g[T6][0] = c.g[V1][0] << 16;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[FP][0] as u32)));
                c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[FP][0] as u32)));
                c.g[T3][0] = 0x5;
                c.g[T0][0] = c.g[S6][0];
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x208));
                'l_390970: loop {
                    c.cycles += 21;
                    c.g[V1][0] = c.g[A2][0] | c.g[S4][0];
                    c.g[A0][0] = c.g[T2][0] | c.g[S1][0];
                    c.g[V0][0] = c.g[T1][0] | c.g[T7][0];
                    c.g[A1][0] = c.g[A3][0] | c.g[T6][0];
                    c.g[V1][0] = c.g[V1][0] | c.g[T4][0];
                    c.g[A0][0] = c.g[A0][0] | c.g[T4][0];
                    c.sd((c.g[S2][0] as u32), c.g[V0][0]);
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x200));
                    c.sd((c.g[S2][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x200));
                    c.sd((c.g[S2][0] as u32).wrapping_add(0x20), c.g[A1][0]);
                    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x200));
                    c.sd((c.g[S2][0] as u32).wrapping_add(0x30), c.g[A0][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x200));
                    c.sd((c.g[S2][0] as u32).wrapping_add(0x8), c.g[T5][0]);
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                    c.sd((c.g[S2][0] as u32).wrapping_add(0x18), c.g[T3][0]);
                    c.sd((c.g[S2][0] as u32).wrapping_add(0x28), c.g[T5][0]);
                    c.sd((c.g[S2][0] as u32).wrapping_add(0x38), c.g[T3][0]);
                    let t = c.g[T0][0] != 0u64;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x40));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00390970);
                        }
                        continue 'l_390970;
                    } else {
                        break 'b_3909c4;
                    }
                }
            }
        }
        'b_390aa4: {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x60));
            let t = c.g[A0][0] == 0u64;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x58));
            if t {
                break 'b_390aa4;
            } else {
                c.cycles += 24;
                c.g[A0][0] = 0x3f800000;
                c.g[A0][0] = c.g[A0][0] << 16;
                c.g[A0][0] = c.g[A0][0] | 0x8000;
                c.g[A0][0] = c.g[A0][0] << 16;
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x54));
                c.g[A2][0] = 0x1;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x5c));
                c.g[V0][0] = c.g[V0][0] << 8;
                c.g[V0][0] = c.g[A1][0] | c.g[V0][0];
                c.g[V1][0] = c.g[V1][0] << 16;
                c.g[A1][0] = 0x48;
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.sd((c.g[S2][0] as u32), c.g[A1][0]);
                c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                c.g[V1][0] = 0x42;
                c.g[S5][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = 0x46;
                c.sd((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                c.sd((c.g[S2][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                c.sd((c.g[S2][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                c.sd((c.g[S2][0] as u32).wrapping_add(0x28), c.g[A2][0]);
                c.sd((c.g[S2][0] as u32).wrapping_add(0x18), 0u64);
                let t = c.g[S6][0] == 0u64;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x30));
                if t {
                    break 'b_390aa4;
                } else {
                    c.cycles += 16;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = sx((c.g[S0][0] as u32) << 4);
                    c.g[A0][0] = sx((c.g[S5][0] as u32) << 4);
                    c.g[V1][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[V0][0] = sx((c.g[A2][0] as u32) << 4);
                    c.g[T0][0] = c.g[V1][0] << 16;
                    c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A2][0] = c.g[S6][0];
                    c.g[S0][0] = c.g[V0][0] << 16;
                    c.g[T1][0] = 0xffff;
                    c.g[T1][0] = c.g[T1][0] << 32;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x200));
                    c.g[A3][0] = 0x5;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[FP][0] as u32)));
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[FP][0] as u32)));
                    'l_390a70: loop {
                        c.cycles += 13;
                        c.g[V0][0] = c.g[A0][0] | c.g[S0][0];
                        c.g[V1][0] = c.g[A1][0] | c.g[T0][0];
                        c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
                        c.g[V1][0] = c.g[V1][0] | c.g[T1][0];
                        c.sd((c.g[S2][0] as u32), c.g[V0][0]);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x200));
                        c.sd((c.g[S2][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x200));
                        c.sd((c.g[S2][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                        c.sd((c.g[S2][0] as u32).wrapping_add(0x18), c.g[A3][0]);
                        let t = c.g[A2][0] != 0u64;
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00390a70);
                            }
                            continue 'l_390a70;
                        } else {
                            break 'b_390aa4;
                        }
                    }
                }
            }
        }
        c.cycles += 95;
        c.g[A3][0] = c.lw((c.g[SP][0] as u32));
        c.g[A1][0] = 0x10000000;
        c.g[T0][0] = 0x10000000;
        c.g[T0][0] = c.g[T0][0] << 32;
        c.g[T0][0] = c.g[T0][0] | 0x8003;
        c.g[A2][0] = 0xe;
        c.g[A0][0] = c.lw((c.g[A3][0] as u32));
        c.g[S1][0] = 0x1000;
        c.g[A3][0] = 0x10000000;
        c.g[A3][0] = c.g[A3][0] | 0x4;
        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
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
        c.sd((c.g[S2][0] as u32), c.g[A3][0]);
        c.sd((c.g[S2][0] as u32).wrapping_add(0x10), c.g[T0][0]);
        c.sd((c.g[S2][0] as u32).wrapping_add(0x18), c.g[A2][0]);
        c.sd((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10d8));
        c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x5a84));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00390b20)?;
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
        c.g[S0][0] = c.g[S0][0] << 32;
        c.g[V0][0] = c.g[V0][0] << 32;
        c.g[S0][0] = c.g[S0][0] >> 32;
        c.g[V0][0] = c.g[V0][0] >> 16;
        c.g[V1][0] = 0x4c;
        c.g[S0][0] = c.g[S0][0] | c.g[V0][0];
        c.sd((c.g[S2][0] as u32).wrapping_add(0x28), c.g[V1][0]);
        c.sd((c.g[S2][0] as u32).wrapping_add(0x20), c.g[S0][0]);
        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00390b58)?;
        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10d8));
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
        c.g[S0][0] = sx((c.g[S0][0] as u32) >> 1);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 4);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00390b78)?;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
        c.g[A3][0] = 0x40;
        c.g[S1][0] = sx((c.g[S1][0] as u32) >> 1);
        c.g[V0][0] = 0x18;
        c.g[S1][0] = sx((c.g[S1][0] as u32) << 4);
        c.sd((c.g[S2][0] as u32).wrapping_add(0x38), c.g[V0][0]);
        c.g[S1][0] = c.g[S1][0] << 32;
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.g[S1][0] = c.g[S0][0] | c.g[S1][0];
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.sd((c.g[S2][0] as u32).wrapping_add(0x30), c.g[S1][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.g[V0][0] = c.lhu((c.g[S3][0] as u32).wrapping_add(0x5900));
        c.g[V1][0] = c.lhu((c.g[S3][0] as u32).wrapping_add(0x5904));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.g[A0][0] = c.lhu((c.g[S3][0] as u32).wrapping_add(0x5902));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[A2][0] = c.lhu((c.g[S3][0] as u32).wrapping_add(0x5906));
        c.g[V1][0] = c.lhu((c.g[S3][0] as u32).wrapping_add(0x5900));
        c.g[V0][0] = c.g[V0][0] << 16;
        c.g[A1][0] = c.lhu((c.g[S3][0] as u32).wrapping_add(0x5902));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.g[A1][0] = c.g[A1][0] << 32;
        c.g[A0][0] = c.g[A0][0] << 48;
        c.g[V1][0] = c.g[V1][0] | c.g[A1][0];
        c.sd((c.g[S2][0] as u32).wrapping_add(0x48), c.g[A3][0]);
        c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.sd((c.g[S2][0] as u32).wrapping_add(0x40), c.g[V1][0]);
        c.g[V1][0] = c.lw((c.g[SP][0] as u32));
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x50));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.sw((c.g[V1][0] as u32), c.g[S2][0]);
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{Rng, DATA, GAME_GP};

    /// The screen object (`self`).
    const OC1061_OBJ: u32 = DATA + 0x1000;
    /// The `self->+0x10d8` object's vtable.
    const OC1061_VT: u32 = DATA + 0x9000;
    /// The cursor cell (`u64**`).
    const OC1061_CURSOR: u32 = DATA + 0xa000;
    /// The DMA buffer.
    const OC1061_BUF: u32 = DATA + 0x2_0000;

    /// Runs the real packet builder against random geometry and filter colours:
    /// no colour (flag off), each of the three components alone positive,
    /// widths from 0 to 0x1ff (0..15 strips), slices 0..2, with the vtable
    /// calls faked (and unwinding on some seeds).
    #[test]
    fn ps2_graphics_man_emit_color_filter_packet_matches() {
        fakes::covered(
            ee::s12::f_003905e8__generated,
            ps2_graphics_man_emit_color_filter_packet,
            200,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x1061_0001, 5);
                let idx = seed % 3;
                let o = OC1061_OBJ;
                let slot = o + idx * 8;
                for off in [0x59b0u32, 0x59b2, 0x59b4, 0x59b6] {
                    let v = if seed % 7 == 0 && off == 0x59b4 {
                        0
                    } else if off >= 0x59b4 {
                        r.next_u32() & 0x1ff
                    } else {
                        r.next_u32() & 0xfff
                    };
                    c.bus.write16(slot + off, v as u16);
                }
                for off in [0x5900u32, 0x5902, 0x5904, 0x5906] {
                    c.bus.write16(o + off, (r.next_u32() & 0xfff) as u16);
                }
                c.bus.write32(o + 0x5a80, r.next_u32());
                c.bus.write32(o + 0x5a84, r.next_u32());
                c.bus.write32(o + 0x10d8, OC1061_VT);
                for off in [0x50u32, 0x58] {
                    c.bus.write16(OC1061_VT + off, (r.next_u32() & 0x7c) as u16);
                }
                c.bus.write32(OC1061_VT + 0x54, fakes::FAKE);
                c.bus.write32(OC1061_VT + 0x5c, fakes::FAKE + 0x10);
                // Filter colour: which components scale to a positive byte.
                let pos = [0.5f32, 0.004, 1.0, 0.0, -0.3];
                let mode = seed % 5;
                let col = |k: u32| -> u32 {
                    let p = match mode {
                        0 => 3,
                        1 => {
                            if k == 0 {
                                0
                            } else {
                                3
                            }
                        }
                        2 => {
                            if k == 1 {
                                2
                            } else {
                                4
                            }
                        }
                        3 => {
                            if k == 2 {
                                1
                            } else {
                                3
                            }
                        }
                        _ => (k as usize + seed as usize) % 5,
                    };
                    pos[p].to_bits()
                };
                let f = o + 0x6ca4 + idx * 0x18;
                for k in 0..6u32 {
                    c.bus
                        .write32(f + 4 * k, if k < 3 { r.f32(1.0) } else { col(k - 3) });
                }
                c.bus.write32(OC1061_CURSOR, OC1061_BUF);
                c.g[A0][0] = sx(o);
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = sx(OC1061_CURSOR);
                c.g[SP][0] = sx(DATA + 0x8_0000);
            },
        );
    }
}
