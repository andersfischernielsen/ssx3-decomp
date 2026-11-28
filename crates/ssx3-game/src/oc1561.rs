//! Staging module oc1561 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1561`.

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
    0x002d_0ef8,
    menu_sys_point2d_item_draw,
    "MenuSysPoint2DItem_Draw",
)];

/// `void MenuSysPoint2DItem_Draw(MenuSysPoint2DItem* item, float* rect)`
/// (0x2d0ef8), a menu row that shows a 2D point curve.
///
/// Draws the owning menu's row background through its vtable (slot at
/// `menu + 0x12c`), prints the row label in the focus/idle colour (the selected
/// item of the page is the focused one), and, while the item shows its cursor
/// and is focused, the current point as "x<sep>y" below it. Then it draws a
/// 100-unit-wide graph on the right of the rect: a filled background quad
/// (`D_004D53C0`), the zero axis when the Y range straddles 0 (`D_004D5380`),
/// the polyline through the item's points (`D_004D53D0`) and the cursor marker
/// (`D_004D53E0`), each on its own depth layer of the draw module's top object
/// (`gp - 0x854 -> +0xe84`). The top object's depth field is restored at the
/// end; `v0` is the value stored back there.
///
/// Fields: item+0x00 menu, +0x14 label, +0x18 point count, +0x24 current index,
/// +0x28 points (x, y pairs), +0x2c/+0x30 X range, +0x34/+0x38 Y range,
/// +0x40 show-cursor flag.
pub fn menu_sys_point2d_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        'b_2d0fa0: {
            'b_2d0f9c: {
                c.cycles += 38;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x1e0));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x170));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x160));
                c.g[S3][0] = c.g[A0][0];
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x150));
                c.g[S4][0] = c.g[A1][0];
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x1b8));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x190));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x180));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x130), c.g[RA][0]);
                c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x1c8));
                c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                c.g[RA][0] = sx(0x002d0f40);
                c.g[A0][0] = c.lw((c.g[S3][0] as u32));
                call_ptr(c, 0x002cbf30, 0x002d0f40)?;
                c.g[A3][0] = c.lw((c.g[S3][0] as u32));
                c.g[V0][0] = c.g[S3][0] ^ c.g[V0][0];
                c.g[S5][0] = (c.g[V0][0] < 0x1u64) as u64;
                c.g[A1][0] = c.g[S4][0];
                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x12c));
                c.g[A2][0] = c.g[S5][0];
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x002d0f68);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x002d0f68)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32));
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x124));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
                c.cvt_s_w(1, 1);
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x34));
                c.g[RA][0] = sx(0x002d0f88);
                c.mul_s(21, 1, 0);
                call_ptr(c, 0x002cbf30, 0x002d0f88)?;
                let t = c.g[S3][0] != c.g[V0][0];
                c.g[V0][0] = 0x4d0000;
                if t {
                    break 'b_2d0f9c;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0x4d0000;
                    let t = true;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x53a0));
                    if t {
                        break 'b_2d0fa0;
                    } else {
                        break 'b_2d0f9c;
                    }
                }
            }
            c.cycles += 1;
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x53b0));
            break 'b_2d0fa0;
        }
        'b_2d1174: {
            'b_2d1168: {
                'b_2d1164: {
                    'b_2d1140: {
                        c.cycles += 50;
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 20);
                        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32));
                        c.g[S6][0] = c.g[V0][0];
                        c.swc1(20, (c.g[SP][0] as u32));
                        c.g[AT][0] = 0x40000000;
                        c.mtc1(1, 2);
                        c.lwc1(0, (c.g[V1][0] as u32));
                        c.g[AT][0] = 0x40c00000;
                        c.mtc1(1, 3);
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                        c.g[AT][0] = 0x41800000;
                        c.mtc1(1, 4);
                        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xc));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x124));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
                        c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.ldr(2, (c.g[SP][0] as u32));
                        c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0xf));
                        c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x47));
                        c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0x40));
                        c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0x4f));
                        c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x48));
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x14));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x17));
                        c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x2f));
                        c.sdr(2, (c.g[A0][0] as u32).wrapping_add(0x28));
                        c.lwc1(13, (c.g[S4][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32));
                        c.add_s(13, 13, 3);
                        c.lwc1(12, (c.g[S4][0] as u32));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
                        c.add_s(12, 12, 4);
                        c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                        c.sub_s(13, 13, 20);
                        c.g[RA][0] = sx(0x002d105c);
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
                        call_ptr(c, 0x00391cb0, 0x002d105c)?;
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                        let t = c.g[V0][0] == 0u64;

                        if t {
                            break 'b_2d1140;
                        } else {
                            c.cycles += 2;
                            let t = c.g[S5][0] == 0u64;
                            c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_2d1164;
                            } else {
                                c.cycles += 52;
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x24));
                                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x28));
                                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                c.g[A0][0] = c.g[S2][0];
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                c.g[RA][0] = sx(0x002d1090);
                                c.lwc1(12, (c.g[V0][0] as u32));
                                call_ptr(c, 0x002cdfd8, 0x002d1090)?;
                                c.g[A2][0] = 0x4a0000;
                                c.g[A1][0] = c.g[S2][0];
                                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x39f0));
                                c.g[RA][0] = sx(0x002d10a4);
                                c.g[A0][0] = c.g[S1][0];
                                call_ptr(c, 0x00318630, 0x002d10a4)?;
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x24));
                                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x28));
                                c.g[A0][0] = c.g[S0][0];
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                c.g[RA][0] = sx(0x002d10c4);
                                c.lwc1(12, (c.g[V0][0] as u32).wrapping_add(0x4));
                                call_ptr(c, 0x002cdfd8, 0x002d10c4)?;
                                c.g[A2][0] = c.g[S0][0];
                                c.g[A0][0] = c.g[S6][0];
                                c.g[RA][0] = sx(0x002d10d4);
                                c.g[A1][0] = c.g[S1][0];
                                call_ptr(c, 0x003185c8, 0x002d10d4)?;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x002d10e0);
                                c.g[A1][0] = 0x2;
                                call_ptr(c, 0x00318260, 0x002d10e0)?;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[RA][0] = sx(0x002d10ec);
                                c.g[A1][0] = 0x2;
                                call_ptr(c, 0x00318260, 0x002d10ec)?;
                                c.g[A0][0] = c.g[S2][0];
                                c.g[RA][0] = sx(0x002d10f8);
                                c.g[A1][0] = 0x2;
                                call_ptr(c, 0x00318260, 0x002d10f8)?;
                                c.lwc1(13, (c.g[S4][0] as u32).wrapping_add(0x4));
                                c.g[AT][0] = 0x41400000;
                                c.mtc1(1, 0);
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32));
                                c.add_s(13, 13, 0);
                                c.lwc1(1, (c.g[S4][0] as u32));
                                c.g[AT][0] = 0x42000000;
                                c.mtc1(1, 12);
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
                                c.sub_s(13, 13, 20);
                                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                c.add_s(12, 1, 12);
                                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
                                c.g[RA][0] = sx(0x002d1134);
                                c.add_s(13, 13, 21);
                                call_ptr(c, 0x00391cb0, 0x002d1134)?;
                                c.g[A0][0] = c.g[S6][0];
                                c.g[RA][0] = sx(0x002d1140);
                                c.g[A1][0] = 0x2;
                                call_ptr(c, 0x00318260, 0x002d1140)?;
                                break 'b_2d1140;
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S5][0] == 0u64;
                    c.lwc1(1, (c.g[S4][0] as u32).wrapping_add(0xc));
                    if t {
                        break 'b_2d1168;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                        let t = c.g[V0][0] == 0u64;

                        if t {
                            break 'b_2d1168;
                        } else {
                            c.cycles += 4;
                            c.g[AT][0] = 0x41c00000;
                            c.mtc1(1, 0);
                            let t = true;
                            c.sub_s(6, 1, 0);
                            if t {
                                break 'b_2d1174;
                            } else {
                                break 'b_2d1164;
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.lwc1(1, (c.g[S4][0] as u32).wrapping_add(0xc));
                break 'b_2d1168;
            }
            c.cycles += 3;
            c.g[AT][0] = 0x41400000;
            c.mtc1(1, 0);
            c.sub_s(6, 1, 0);
            break 'b_2d1174;
        }
        c.cycles += 29;
        c.lwc1(1, (c.g[S4][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = 0x3;
        c.lwc1(2, (c.g[S4][0] as u32).wrapping_add(0xc));
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.lwc1(0, (c.g[S4][0] as u32));
        c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
        c.sub_s(2, 2, 6);
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 3);
        c.add_s(0, 0, 1);
        c.g[AT][0] = 0x41800000;
        c.mtc1(1, 4);
        c.lwc1(1, (c.g[S4][0] as u32).wrapping_add(0x4));
        c.g[A2][0] = 0xffffffffffffffff;
        c.mul_s(2, 2, 3);
        c.g[AT][0] = 0x42c80000;
        c.mtc1(1, 5);
        c.sub_s(0, 0, 4);
        c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x1c));
        c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x18));
        c.add_s(1, 1, 2);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.sub_s(0, 0, 5);
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xe84));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.g[V0][0] & 0x3e0;
        c.g[S2][0] = sx((c.g[V0][0] as u32) >> 5);
        'l_2d11e8: loop {
            c.cycles += 7;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
            let t = c.g[A1][0] != c.g[A2][0];
            c.g[V1][0] = 0x4d0000;
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002d11e8);
                }
                continue 'l_2d11e8;
            } else {
                c.cycles += 27;
                c.g[AT][0] = 0x43000000;
                c.mtc1(1, 0);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x53c0));
                c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0x53c0));
                c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x2));
                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.mul_s(3, 3, 0);
                c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.mul_s(4, 4, 0);
                c.mul_s(1, 1, 0);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 5);
                c.mul_s(2, 2, 0);
                c.g[S4][0] = sx((c.g[S2][0] as u32) << 5);
                c.cvt_w_s(0, 3);
                c.mfc1(8, 0);
                c.g[V1][0] = c.g[S1][0];
                c.cvt_w_s(0, 1);
                c.mfc1(7, 0);
                c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc0));
                c.cvt_w_s(0, 2);
                c.mfc1(5, 0);
                c.cvt_w_s(0, 4);
                c.mfc1(4, 0);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[T0][0]);
                'l_2d1270: loop {
                    c.cycles += 8;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[A1][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                    c.swc1(5, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[T0][0]);
                        if event_due(c) {
                            deliver_event_here(c, 0x002d1270);
                        }
                        continue 'l_2d1270;
                    } else {
                        'b_2d131c: {
                            c.cycles += 34;
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                            c.g[A3][0] = 0xffffffffffffffff;
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 0);
                            c.g[A1][0] = c.g[S6][0];
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A2][0] = c.g[S5][0];
                            c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x11c));
                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x118));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x110), 0u64);
                            c.g[RA][0] = sx(0x002d12c8);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x114), 0u64);
                            call_ptr(c, 0x002cb350, 0x002d12c8)?;
                            c.lwc1(3, (c.g[S3][0] as u32).wrapping_add(0x30));
                            c.g[V1][0] = 0x1;
                            c.lwc1(7, (c.g[S3][0] as u32).wrapping_add(0x38));
                            c.lwc1(2, (c.g[S3][0] as u32).wrapping_add(0x2c));
                            c.lwc1(1, (c.g[S3][0] as u32).wrapping_add(0x34));
                            c.sub_s(4, 3, 7);
                            c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x1c));
                            c.sub_s(1, 1, 2);
                            c.lwc1(5, (c.g[SP][0] as u32).wrapping_add(0x18));
                            c.div_s(23, 0, 4);
                            c.lwc1(6, (c.g[SP][0] as u32).wrapping_add(0x10));
                            c.mtc1(0, 8);
                            c.div_s(22, 5, 1);
                            c.c_le_s(8, 3);
                            c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x14));
                            c.mul_s(0, 7, 23);
                            c.mul_s(2, 2, 22);
                            c.sub_s(21, 3, 0);
                            let t = c.fcc();
                            c.sub_s(24, 6, 2);
                            if t {
                                break 'b_2d131c;
                            } else {
                                c.cycles += 1;
                                c.g[V1][0] = 0u64;
                                break 'b_2d131c;
                            }
                        }
                        'b_2d1330: {
                            c.cycles += 4;
                            c.c_lt_s(8, 7);
                            let t = c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_2d1330;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = 0u64;
                                break 'b_2d1330;
                            }
                        }
                        'b_2d1444: {
                            c.cycles += 2;
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                            if t {
                                break 'b_2d1444;
                            } else {
                                c.cycles += 36;
                                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                c.g[A0][0] = ((c.g[V0][0] as i64) < 32i64) as u64;
                                c.g[V1][0] = 0x1f;
                                c.g[A3][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
                                if c.g[A0][0] == 0 {
                                    c.g[V0][0] = c.g[V1][0];
                                }
                                c.g[A1][0] = 0xfffffffffffffc1f;
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
                                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                c.g[V0][0] = c.g[V0][0] & 0x3e0;
                                c.g[A2][0] = 0x4d0000;
                                c.g[AT][0] = 0x43000000;
                                c.mtc1(1, 4);
                                c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
                                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5380));
                                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                                c.g[A1][0] = c.g[S1][0];
                                c.sw((c.g[A3][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                                c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                                c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0x5380));
                                c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x4));
                                c.lwc1(3, (c.g[A0][0] as u32).wrapping_add(0xc));
                                c.mul_s(2, 2, 4);
                                c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                                c.mul_s(1, 1, 4);
                                c.mul_s(3, 3, 4);
                                c.mul_s(0, 0, 4);
                                c.cvt_w_s(4, 2);
                                c.mfc1(7, 4);
                                c.cvt_w_s(2, 1);
                                c.mfc1(6, 2);
                                c.cvt_w_s(1, 0);
                                c.mfc1(4, 1);
                                c.cvt_w_s(0, 3);
                                c.mfc1(3, 0);
                                c.sw((c.g[A1][0] as u32).wrapping_add(0x1c), c.g[A3][0]);
                                'l_2d13c8: loop {
                                    c.cycles += 7;
                                    c.sw((c.g[A1][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                                    c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[A0][0]);
                                    c.sw((c.g[A1][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x30));
                                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[T0][0] as i64)) as u64;
                                    if c.g[V0][0] != 0u64 {
                                        c.sw((c.g[A1][0] as u32).wrapping_add(0x1c), c.g[A3][0]);
                                        if event_due(c) {
                                            deliver_event_here(c, 0x002d13c8);
                                        }
                                        continue 'l_2d13c8;
                                    } else {
                                        c.cycles += 24;
                                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                                        c.g[A1][0] = 0x2;
                                        c.g[AT][0] = 0x3f800000;
                                        c.mtc1(1, 2);
                                        c.g[A2][0] = c.g[S1][0];
                                        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x18));
                                        c.g[A3][0] = 0u64;
                                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x118), 0u64);
                                        c.add_s(0, 0, 1);
                                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x114));
                                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x11c));
                                        c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
                                        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x118), 0u64);
                                        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x248));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24c));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x002d1444);
                                        c.g[A0][0] =
                                            sx((c.g[T0][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x002d1444)?;
                                        break 'b_2d1444;
                                    }
                                }
                            }
                        }
                        c.cycles += 35;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                        c.g[A0][0] = ((c.g[S0][0] as i64) < 32i64) as u64;
                        c.g[V0][0] = 0x1f;
                        c.g[A3][0] = 0xfffffffffffffc1f;
                        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
                        if c.g[A0][0] != 0 {
                            c.g[V0][0] = c.g[S0][0];
                        }
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
                        c.g[A1][0] = 0x4d0000;
                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[V0][0] = c.g[V0][0] & 0x3e0;
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x53d0));
                        c.g[AT][0] = 0x43000000;
                        c.mtc1(1, 4);
                        c.g[V1][0] = c.g[V1][0] & c.g[A3][0];
                        c.g[T0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x60));
                        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                        c.g[A3][0] = c.g[S1][0];
                        c.sw((c.g[A2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                        c.lwc1(2, (c.g[A1][0] as u32).wrapping_add(0x53d0));
                        c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x4));
                        c.lwc1(3, (c.g[A0][0] as u32).wrapping_add(0xc));
                        c.mul_s(2, 2, 4);
                        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                        c.mul_s(1, 1, 4);
                        c.mul_s(3, 3, 4);
                        c.mul_s(0, 0, 4);
                        c.cvt_w_s(4, 2);
                        c.mfc1(6, 4);
                        c.cvt_w_s(2, 1);
                        c.mfc1(5, 2);
                        c.cvt_w_s(1, 0);
                        c.mfc1(4, 1);
                        c.cvt_w_s(0, 3);
                        c.mfc1(3, 0);
                        c.sw((c.g[A3][0] as u32).wrapping_add(0x1c), c.g[A2][0]);
                        'l_2d14d0: loop {
                            c.cycles += 7;
                            c.sw((c.g[A3][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                            c.sw((c.g[A3][0] as u32).wrapping_add(0x14), c.g[A0][0]);
                            c.sw((c.g[A3][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x30));
                            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[T0][0] as i64)) as u64;
                            if c.g[V0][0] != 0u64 {
                                c.sw((c.g[A3][0] as u32).wrapping_add(0x1c), c.g[A2][0]);
                                if event_due(c) {
                                    deliver_event_here(c, 0x002d14d0);
                                }
                                continue 'l_2d14d0;
                            } else {
                                'b_2d15b8: {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18));
                                    c.g[S0][0] = 0x1;
                                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                        break 'b_2d15b8;
                                    } else {
                                        c.cycles += 4;
                                        c.g[AT][0] = 0x3f800000;
                                        c.mtc1(1, 20);
                                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x28));
                                        'l_2d1510: loop {
                                            c.cycles += 41;
                                            c.g[V1][0] = sx((c.g[S0][0] as u32) << 3);
                                            c.g[T1][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
                                            c.g[A1][0] = 0x2;
                                            c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            c.g[A3][0] = 0u64;
                                            c.g[A2][0] = c.g[V0][0];
                                            c.lwc1(1, (c.g[V0][0] as u32).wrapping_sub(0x8));
                                            c.lwc1(2, (c.g[A2][0] as u32).wrapping_sub(0x4));
                                            c.g[V1][0] = c.g[V0][0];
                                            c.mul_s(1, 1, 22);
                                            c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0x4));
                                            c.mul_s(2, 2, 23);
                                            c.lwc1(0, (c.g[V1][0] as u32));
                                            c.mul_s(3, 3, 23);
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x118), 0u64);
                                            c.add_s(1, 1, 24);
                                            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x11c));
                                            c.add_s(2, 2, 21);
                                            c.g[A2][0] = c.g[S1][0];
                                            c.mul_s(0, 0, 22);
                                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x110));
                                            c.add_s(3, 3, 21);
                                            c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x114));
                                            c.add_s(0, 0, 24);
                                            c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x110));
                                            c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x114));
                                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x110));
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x118), 0u64);
                                            c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                            c.g[V1][0] =
                                                c.lw((c.g[T1][0] as u32).wrapping_add(0x10d8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0x248));
                                            c.g[T0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x24c));
                                            let t = c.g[T0][0] as u32;
                                            c.g[RA][0] = sx(0x002d15a4);
                                            c.g[A0][0] = sx((c.g[T1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x002d15a4)?;
                                            c.g[V0][0] =
                                                c.lw((c.g[S3][0] as u32).wrapping_add(0x18));
                                            c.g[V0][0] =
                                                ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[A0][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x28));
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x002d1510);
                                                }
                                                continue 'l_2d1510;
                                            } else {
                                                c.cycles += 1;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                                break 'b_2d15b8;
                                            }
                                        }
                                    }
                                }
                                'b_2d16e4: {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                    if t {
                                        break 'b_2d16e4;
                                    } else {
                                        c.cycles += 36;
                                        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x3));
                                        c.g[A0][0] = ((c.g[V0][0] as i64) < 32i64) as u64;
                                        c.g[V1][0] = 0x1f;
                                        c.g[A3][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
                                        if c.g[A0][0] == 0 {
                                            c.g[V0][0] = c.g[V1][0];
                                        }
                                        c.g[A1][0] = 0xfffffffffffffc1f;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
                                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                        c.g[V0][0] = c.g[V0][0] & 0x3e0;
                                        c.g[A2][0] = 0x4d0000;
                                        c.g[AT][0] = 0x43000000;
                                        c.mtc1(1, 4);
                                        c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
                                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x53e0));
                                        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                                        c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x120));
                                        c.sw((c.g[A3][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                                        c.g[T0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc0));
                                        c.g[V1][0] = c.g[S1][0];
                                        c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0x53e0));
                                        c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x4));
                                        c.lwc1(3, (c.g[A0][0] as u32).wrapping_add(0xc));
                                        c.mul_s(2, 2, 4);
                                        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                                        c.mul_s(1, 1, 4);
                                        c.mul_s(3, 3, 4);
                                        c.mul_s(0, 0, 4);
                                        c.cvt_w_s(4, 2);
                                        c.mfc1(7, 4);
                                        c.cvt_w_s(2, 1);
                                        c.mfc1(6, 2);
                                        c.cvt_w_s(1, 0);
                                        c.mfc1(5, 1);
                                        c.cvt_w_s(0, 3);
                                        c.mfc1(4, 0);
                                        c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[A3][0]);
                                        'l_2d1650: loop {
                                            c.cycles += 7;
                                            c.sw(
                                                (c.g[V1][0] as u32).wrapping_add(0x10),
                                                c.g[A2][0],
                                            );
                                            c.sw(
                                                (c.g[V1][0] as u32).wrapping_add(0x14),
                                                c.g[A1][0],
                                            );
                                            c.sw(
                                                (c.g[V1][0] as u32).wrapping_add(0x18),
                                                c.g[A0][0],
                                            );
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
                                            c.g[V0][0] =
                                                ((c.g[V1][0] as i64) < (c.g[T0][0] as i64)) as u64;
                                            if c.g[V0][0] != 0u64 {
                                                c.sw(
                                                    (c.g[V1][0] as u32).wrapping_add(0x1c),
                                                    c.g[A3][0],
                                                );
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x002d1650);
                                                }
                                                continue 'l_2d1650;
                                            } else {
                                                c.cycles += 30;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x24));
                                                c.g[A0][0] = c.g[S1][0];
                                                c.g[V1][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x28));
                                                c.g[A1][0] = c.g[S5][0];
                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                                c.g[AT][0] = 0x40000000;
                                                c.mtc1(1, 4);
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add((c.g[V1][0] as u32)));
                                                c.g[AT][0] = 0x40a00000;
                                                c.mtc1(1, 2);
                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                                                c.g[A2][0] = c.g[T1][0];
                                                c.lwc1(0, (c.g[V0][0] as u32));
                                                c.mul_s(1, 1, 23);
                                                c.g[AT][0] = 0x3f800000;
                                                c.mtc1(1, 3);
                                                c.mul_s(0, 0, 22);
                                                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x11c));
                                                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x12c));
                                                c.add_s(1, 1, 21);
                                                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x118));
                                                c.add_s(0, 0, 24);
                                                c.sw((c.g[SP][0] as u32).wrapping_add(0x120), 0u64);
                                                c.sw((c.g[SP][0] as u32).wrapping_add(0x124), 0u64);
                                                c.sub_s(1, 1, 4);
                                                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x128));
                                                c.sub_s(0, 0, 4);
                                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x114));
                                                c.g[RA][0] = sx(0x002d16e4);
                                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x110));
                                                call_ptr(c, 0x002cb350, 0x002d16e4)?;
                                                break 'b_2d16e4;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 22;
                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                c.g[A1][0] = 0xfffffffffffffc1f;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
                                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x190));
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x180));
                                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x170));
                                c.g[V0][0] = c.g[V0][0] | c.g[S4][0];
                                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x150));
                                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x160));
                                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x140));
                                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x130));
                                c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                                c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x1c8));
                                c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                                c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x1b8));
                                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                                c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                let t = c.g[RA][0] as u32;
                                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1e0));
                                c.pc = t;
                                return Ok(());
                            }
                        }
                    }
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the fakes (own RAM word and magic: the fakes sit in shared
    /// override slots, so they act only on machines that set this).
    const OC1561_CTRL: u32 = DATA + 0x20_0000;
    const OC1561_MAGIC: u32 = 0x1561_2d0e;
    const OC1561_ITEM: u32 = DATA + 0x21_0000;
    const OC1561_MENU: u32 = DATA + 0x22_0000;
    const OC1561_VT: u32 = DATA + 0x23_0000;
    const OC1561_FONT: u32 = DATA + 0x24_0000;
    const OC1561_STYLE: u32 = DATA + 0x25_0000;
    const OC1561_DRAW: u32 = DATA + 0x26_0000;
    const OC1561_TOP: u32 = DATA + 0x27_0000;
    const OC1561_REND: u32 = DATA + 0x28_0000;
    const OC1561_REND2: u32 = DATA + 0x29_0000;
    const OC1561_PTS: u32 = DATA + 0x2a_0000;
    const OC1561_RECT: u32 = DATA + 0x2b_0000;
    const OC1561_LABEL: u32 = DATA + 0x2c_0000;

    /// Gated fakes for the callees of the draw: with the gate word set they log
    /// the call (`fakes::fake_call`); otherwise they run the body they replaced.
    macro_rules! oc1561_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1561_CTRL) != OC1561_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            fn oc1561_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1561_fakes!(
        (
            oc1561_fake_2cbf30,
            OC1561_PREV_2CBF30,
            ee::s09::OVERRIDE_002CBF30,
            ee::s09::f_002cbf30__generated,
            0x002c_bf30
        ),
        (
            oc1561_fake_2cdfd8,
            OC1561_PREV_2CDFD8,
            ee::s09::OVERRIDE_002CDFD8,
            ee::s09::f_002cdfd8__generated,
            0x002c_dfd8
        ),
        (
            oc1561_fake_2cb350,
            OC1561_PREV_2CB350,
            ee::s09::OVERRIDE_002CB350,
            ee::s09::f_002cb350__generated,
            0x002c_b350
        ),
        (
            oc1561_fake_391cb0,
            OC1561_PREV_391CB0,
            ee::s12::OVERRIDE_00391CB0,
            ee::s12::f_00391cb0__generated,
            0x0039_1cb0
        ),
        (
            oc1561_fake_318630,
            OC1561_PREV_318630,
            ee::s10::OVERRIDE_00318630,
            ee::s10::f_00318630__generated,
            0x0031_8630
        ),
        (
            oc1561_fake_3185c8,
            OC1561_PREV_3185C8,
            ee::s10::OVERRIDE_003185C8,
            ee::s10::f_003185c8__generated,
            0x0031_85c8
        ),
        (
            oc1561_fake_318260,
            OC1561_PREV_318260,
            ee::s10::OVERRIDE_00318260,
            ee::s10::f_00318260__generated,
            0x0031_8260
        ),
    );

    fn oc1561_w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// One menu row with a point curve. Seed bits: 0 = show cursor, 1 = the item
    /// is the page's selected one, 2-3 = Y range kind (all positive, straddling
    /// zero, all negative, starting at zero); the point count is 1..=4.
    fn oc1561_setup(c: &mut Cpu, seed: u32) {
        oc1561_install_fakes();
        let mut r: Rng = machine(c, seed, 0x2d0ef8, 24);
        c.g[GP][0] = GAME_GP as u64;
        oc1561_w(c, OC1561_CTRL, OC1561_MAGIC);
        let focused = seed & 2 != 0;
        if focused {
            // Calls 0 and 2 are MenuSysPage_GetSelectedItem (1 is the menu's
            // draw-background vcall).
            oc1561_w(c, LOG_RESULTS, OC1561_ITEM);
            oc1561_w(c, LOG_RESULTS + 8, OC1561_ITEM);
        }
        c.g[A0][0] = sx(OC1561_ITEM);
        c.g[A1][0] = sx(OC1561_RECT);
        for i in 0..4 {
            oc1561_w(c, OC1561_RECT + 4 * i, r.f32(200.0));
        }
        let count = 1 + (seed >> 4) % 4;
        let cur = (seed >> 6) % count;
        let (min_y, max_y) = match (seed >> 2) % 4 {
            0 => (1.0f32, 9.0f32),
            1 => (-4.0, 6.0),
            2 => (-9.0, -1.0),
            _ => (0.0, 5.0),
        };
        oc1561_w(c, OC1561_ITEM, OC1561_MENU);
        oc1561_w(c, OC1561_ITEM + 0x14, OC1561_LABEL);
        oc1561_w(c, OC1561_ITEM + 0x18, count);
        oc1561_w(c, OC1561_ITEM + 0x24, cur);
        oc1561_w(c, OC1561_ITEM + 0x28, OC1561_PTS);
        oc1561_w(c, OC1561_ITEM + 0x2c, 0.0f32.to_bits());
        oc1561_w(c, OC1561_ITEM + 0x30, 10.0f32.to_bits());
        oc1561_w(c, OC1561_ITEM + 0x34, min_y.to_bits());
        oc1561_w(c, OC1561_ITEM + 0x38, max_y.to_bits());
        oc1561_w(c, OC1561_ITEM + 0x40, seed & 1);
        for i in 0..8 {
            oc1561_w(c, OC1561_PTS + 4 * i, r.f32(10.0));
        }
        // The menu: font at +0x124, vtable at +0x12c (slot 9 at +0x48 = delta,
        // +0x4c = function) pointing at a fake method.
        oc1561_w(c, OC1561_MENU + 0x124, OC1561_FONT);
        oc1561_w(c, OC1561_MENU + 0x12c, OC1561_VT);
        c.bus.write16(OC1561_VT + 0x48, 0x40);
        oc1561_w(c, OC1561_VT + 0x4c, FAKE + 0x10);
        oc1561_w(c, OC1561_FONT + 0x58, OC1561_STYLE);
        oc1561_w(c, OC1561_STYLE + 0x14, 14);
        oc1561_w(c, OC1561_STYLE + 0x34, 1.25f32.to_bits());
        // Draw module (gp - 0x854) -> top object; renderer (gp + 0x2a90).
        oc1561_w(c, GAME_GP - 0x854, OC1561_DRAW);
        oc1561_w(c, OC1561_DRAW + 0xe84, OC1561_TOP);
        oc1561_w(c, OC1561_TOP + 8, r.next_u32());
        oc1561_w(c, GAME_GP + 0x2a90, OC1561_REND);
        oc1561_w(c, OC1561_REND + 0x10d8, OC1561_REND2);
        c.bus.write16(OC1561_REND2 + 0x248, 0x20);
        oc1561_w(c, OC1561_REND2 + 0x24c, FAKE + 0x20);
        // Colour globals D_004D5380/53a0/53b0/53c0/53d0/53e0 (4 floats each).
        for base in [
            0x4d_5380u32,
            0x4d_53a0,
            0x4d_53b0,
            0x4d_53c0,
            0x4d_53d0,
            0x4d_53e0,
        ] {
            for i in 0..4 {
                oc1561_w(c, base + 4 * i, r.f32(1.2));
            }
        }
    }

    /// The draw against the generated body: the focused and idle label colours,
    /// the cursor text, the graph (axis only when the Y range straddles 0,
    /// polyline only with 2+ points, cursor marker) and callee unwinds.
    #[test]
    fn menu_sys_point2d_item_draw_matches() {
        fakes::check_reaching(
            ee::s09::f_002d0ef8__generated,
            menu_sys_point2d_item_draw,
            256,
            &[
                0x002d_0f40, // GetSelectedItem
                0x002d_0f68, // background vcall
                0x002d_105c, // label
                0x002d_1090, // FormatFloat x
                0x002d_10d4, // string concat
                0x002d_1134, // cursor text
                0x002d_12c8, // graph background quad
                0x002d_1444, // axis line
                0x002d_15a4, // polyline segment
                0x002d_16e4, // cursor marker quad
            ],
            &oc1561_setup,
        );
    }
}
