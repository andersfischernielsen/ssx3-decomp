//! Staging module oc1078 (wave wave267): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1078`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `void Render2D_DrawTiledSpriteXform(Element *el, Rect *dst, Colour *col,
/// Mat *xform, float tx, float ty, int flags)` (0x3a2bc8; the reference calls it
/// `cTextureElement_renderRotTiled`): draws the texture element `el` tiled
/// `tx` by `ty` times into the destination rectangle `dst` (x, y, w, h), through
/// the 2D renderer. `a0` element (texture id at +0, uv at +0xc..+0x18), `a1`
/// destination rect, `a2` colour (copied to the stack), `a3` transform, `f12`
/// tx, `f13` ty, `t0` flags (1 flip u, 2 flip v, 4 rotated quads via
/// `Render2D_DrawQuadUVRot90Xform`, else `Render2D_DrawQuadUVXform`). Whole
/// tiles are `floor(tx)` columns by `floor(ty)` rows; the fractional rest of
/// the row and of the column is drawn as partial tiles with matching uv.
/// Kept as the generated block structure (labels are the generated block
/// addresses) so every register, FPU register and cycle matches.
#[allow(unreachable_code, unused_assignments)]
pub fn render2d_draw_tiled_sprite_xform(c: &mut Cpu) {
    guest(c, |c| {
        'b_3a3228: {
            'b_3a3224: {
                c.cycles += 78;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x240));
                c.mtc1(0, 0);
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x180));
                c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x160));
                c.g[S5][0] = c.g[SP][0];
                c.swc1(31, (c.g[SP][0] as u32).wrapping_add(0x238));
                c.g[S7][0] = c.g[T0][0];
                c.swc1(30, (c.g[SP][0] as u32).wrapping_add(0x230));
                c.swc1(29, (c.g[SP][0] as u32).wrapping_add(0x228));
                c.mov_s(30, 12);
                c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x1f0));
                c.c_le_s(30, 0);
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x1e8));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x190));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x170));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x150), c.g[RA][0]);
                c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0x220));
                c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0x218));
                c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0x210));
                c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x208));
                c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x200));
                c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x1f8));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x1e0));
                c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
                c.ldr(2, (c.g[A1][0] as u32));
                c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0xf));
                c.ldr(3, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(2, (c.g[SP][0] as u32));
                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
                c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.g[A0][0];
                c.ldl(2, (c.g[A2][0] as u32).wrapping_add(0x7));
                c.ldr(2, (c.g[A2][0] as u32));
                c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0xf));
                c.ldr(3, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0x17));
                c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x10));
                c.ldl(9, (c.g[A2][0] as u32).wrapping_add(0x1f));
                c.ldr(9, (c.g[A2][0] as u32).wrapping_add(0x18));
                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x17));
                c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x1f));
                c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x27));
                c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sdl(9, (c.g[SP][0] as u32).wrapping_add(0x2f));
                c.sdr(9, (c.g[SP][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x20));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                c.ldl(10, (c.g[A3][0] as u32).wrapping_add(0x7));
                c.ldr(10, (c.g[A3][0] as u32));
                c.ldl(3, (c.g[A3][0] as u32).wrapping_add(0xf));
                c.ldr(3, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.sdl(10, (c.g[SP][0] as u32).wrapping_add(0x47));
                c.sdr(10, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x4f));
                c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x48));
                c.swc1(13, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[A0][0] = c.lw((c.g[A1][0] as u32));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xe84));
                c.sh((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                c.lwc1(1, (c.g[SP][0] as u32));
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x134));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x138));
                c.lwc1(22, (c.g[A1][0] as u32).wrapping_add(0xc));
                c.lwc1(31, (c.g[A1][0] as u32).wrapping_add(0x18));
                c.lwc1(21, (c.g[A1][0] as u32).wrapping_add(0x10));
                let t = c.fcc();
                c.lwc1(29, (c.g[A1][0] as u32).wrapping_add(0x14));
                if t {
                    break 'b_3a3224;
                } else {
                    c.cycles += 5;
                    c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x130));
                    c.c_le_s(3, 0);
                    let t = c.fcc();
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                    if t {
                        break 'b_3a3228;
                    } else {
                        'b_3a2d38: {
                            c.cycles += 7;
                            c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = c.g[S7][0] & 0x1;
                            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xc));
                            c.div_s(27, 0, 30);
                            c.div_s(28, 1, 3);
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = c.g[S7][0] & 0x2;
                            if t {
                                break 'b_3a2d38;
                            } else {
                                c.cycles += 2;
                                c.mov_s(21, 29);
                                c.lwc1(29, (c.g[A1][0] as u32).wrapping_add(0x10));
                                break 'b_3a2d38;
                            }
                        }
                        'b_3a2d48: {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;

                            if t {
                                break 'b_3a2d48;
                            } else {
                                c.cycles += 2;
                                c.mov_s(22, 31);
                                c.lwc1(31, (c.g[A1][0] as u32).wrapping_add(0xc));
                                break 'b_3a2d48;
                            }
                        }
                        c.cycles += 10;
                        c.cvt_w_s(0, 30);
                        c.mfc1(17, 0);
                        c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x138));
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x130));
                        c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                        c.g[V1][0] = 0xffffffffffffffff;
                        c.cvt_w_s(0, 0);
                        c.mfc1(22, 0);
                        c.g[V0][0] = 0x3;
                        'l_3a2d70: loop {
                            c.cycles += 7;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            let t = c.g[V0][0] != c.g[V1][0];
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc0));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x003a2d70);
                                }
                                continue 'l_3a2d70;
                            } else {
                                c.cycles += 11;
                                c.mtc1(22, 1);
                                c.cvt_s_w(1, 1);
                                c.g[AT][0] = 0x43000000;
                                c.mtc1(1, 4);
                                c.g[AT][0] = 0x437f0000;
                                c.mtc1(1, 2);
                                c.g[V1][0] = c.g[S2][0];
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 3);
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x140));
                                c.lwc1(0, (c.g[A2][0] as u32));
                                'l_3a2db8: loop {
                                    c.cycles += 20;
                                    c.mul_s(0, 0, 4);
                                    c.cvt_w_s(1, 0);
                                    c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x1c));
                                    c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x4));
                                    c.mul_s(1, 1, 2);
                                    c.cvt_w_s(0, 1);
                                    c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x10));
                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.mul_s(0, 0, 2);
                                    c.cvt_w_s(1, 0);
                                    c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x14));
                                    c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0xc));
                                    c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x8));
                                    c.mul_s(1, 1, 2);
                                    c.cvt_w_s(0, 1);
                                    c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x18));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
                                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                                    if c.g[V0][0] != 0u64 {
                                        c.lwc1(0, (c.g[A2][0] as u32));
                                        if event_due(c) {
                                            deliver_event_here(c, 0x003a2db8);
                                        }
                                        continue 'l_3a2db8;
                                    } else {
                                        'b_3a3008: {
                                            c.cycles += 2;
                                            if c.g[S6][0] == 0u64 {
                                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x140));
                                                break 'b_3a3008;
                                            } else {
                                                'b_3a3004: {
                                                    c.cycles += 3;
                                                    c.lwc1(
                                                        25,
                                                        (c.g[SP][0] as u32).wrapping_add(0x138),
                                                    );
                                                    let t = (c.g[S6][0] as i64) <= 0;
                                                    c.g[V1][0] = 0u64;
                                                    if t {
                                                        break 'b_3a3004;
                                                    } else {
                                                        c.cycles += 1;
                                                        'l_3a2e20: loop {
                                                            'b_3a2ff8: {
                                                                'b_3a2f14: {
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = c.g[S7][0] & 0x4;
                                                                    let t = c.g[V0][0] == 0u64;
                                                                    c.lwc1(
                                                                        20,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x134),
                                                                    );
                                                                    if t {
                                                                        break 'b_3a2f14;
                                                                    } else {
                                                                        'b_3a2e9c: {
                                                                            c.cycles += 6;
                                                                            c.add_s(1, 25, 28);
                                                                            c.g[S4][0] =
                                                                                sx((c.g[V1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x1,
                                                                                    ));
                                                                            c.mtc1(17, 23);
                                                                            c.cvt_s_w(23, 23);
                                                                            let t = (c.g[S1][0]
                                                                                as i64)
                                                                                <= 0;
                                                                            c.swc1(
                                                                                1,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x13c,
                                                                                    ),
                                                                            );
                                                                            if t {
                                                                                break 'b_3a2e9c;
                                                                            } else {
                                                                                c.cycles += 5;
                                                                                c.sub_s(26, 29, 21);
                                                                                c.g[S0][0] =
                                                                                    c.g[S1][0];
                                                                                c.sub_s(24, 31, 22);
                                                                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x110));
                                                                                'l_3a2e58: loop {
                                                                                    c.cycles += 17;
                                                                                    c.g[A1][0] =
                                                                                        c.g[S2][0];
                                                                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x844));
                                                                                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
                                                                                    c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x114));
                                                                                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x120));
                                                                                    c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0x118));
                                                                                    c.g[T0][0] =
                                                                                        c.g[S3][0];
                                                                                    c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0x11c));
                                                                                    c.add_s(
                                                                                        20, 20, 27,
                                                                                    );
                                                                                    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x120));
                                                                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                                                                                    c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x124));
                                                                                    c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0x128));
                                                                                    c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x12c));
                                                                                    call_ptr(
                                                                                        c,
                                                                                        0x0038ea60,
                                                                                        0x003a2e94,
                                                                                    )?;
                                                                                    if c.g[S0][0]
                                                                                        != 0u64
                                                                                    {
                                                                                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x110));
                                                                                        if event_due(
                                                                                            c,
                                                                                        ) {
                                                                                            deliver_event_here(c, 0x003a2e58);
                                                                                        }
                                                                                        continue 'l_3a2e58;
                                                                                    } else {
                                                                                        break 'b_3a2e9c;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 4;
                                                                        c.c_eq_s(23, 30);
                                                                        let t = c.fcc();
                                                                        c.g[V1][0] = c.g[S4][0];
                                                                        if t {
                                                                            break 'b_3a2ff8;
                                                                        } else {
                                                                            c.cycles += 26;
                                                                            c.sub_s(3, 30, 23);
                                                                            c.lwc1(
                                                                                1,
                                                                                (c.g[S5][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x8,
                                                                                    ),
                                                                            );
                                                                            c.sub_s(0, 31, 22);
                                                                            c.lwc1(
                                                                                2,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x134,
                                                                                    ),
                                                                            );
                                                                            c.g[A0][0] = c.lw((c.g
                                                                                [GP][0]
                                                                                as u32)
                                                                                .wrapping_sub(
                                                                                    0x844,
                                                                                ));
                                                                            c.g[A1][0] = c.g[S2][0];
                                                                            c.add_s(1, 2, 1);
                                                                            c.swc1(
                                                                                20,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x110,
                                                                                    ),
                                                                            );
                                                                            c.mul_s(0, 0, 3);
                                                                            c.swc1(
                                                                                25,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x114,
                                                                                    ),
                                                                            );
                                                                            c.sub_s(2, 29, 21);
                                                                            c.swc1(
                                                                                28,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x11c,
                                                                                    ),
                                                                            );
                                                                            c.sub_s(1, 1, 20);
                                                                            c.swc1(
                                                                                21,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x120,
                                                                                    ),
                                                                            );
                                                                            c.add_s(0, 22, 0);
                                                                            c.swc1(
                                                                                22,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x124,
                                                                                    ),
                                                                            );
                                                                            c.swc1(
                                                                                2,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x128,
                                                                                    ),
                                                                            );
                                                                            c.g[A2][0] =
                                                                                sx((c.g[SP][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x110,
                                                                                    ));
                                                                            c.swc1(
                                                                                1,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x118,
                                                                                    ),
                                                                            );
                                                                            c.g[A3][0] =
                                                                                sx((c.g[SP][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x120,
                                                                                    ));
                                                                            c.sub_s(0, 0, 22);
                                                                            c.g[T0][0] = c.g[S3][0];
                                                                            c.swc1(
                                                                                0,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x12c,
                                                                                    ),
                                                                            );
                                                                            call_ptr(
                                                                                c, 0x0038ea60,
                                                                                0x003a2f0c,
                                                                            )?;
                                                                            let t = true;
                                                                            c.g[V1][0] = c.g[S4][0];
                                                                            if t {
                                                                                break 'b_3a2ff8;
                                                                            } else {
                                                                                break 'b_3a2f14;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_3a2f84: {
                                                                    c.cycles += 6;
                                                                    c.add_s(3, 25, 28);
                                                                    c.g[S4][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x1));
                                                                    c.mtc1(17, 23);
                                                                    c.cvt_s_w(23, 23);
                                                                    let t =
                                                                        (c.g[S1][0] as i64) <= 0;
                                                                    c.swc1(
                                                                        3,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x13c),
                                                                    );
                                                                    if t {
                                                                        break 'b_3a2f84;
                                                                    } else {
                                                                        c.cycles += 5;
                                                                        c.sub_s(26, 29, 21);
                                                                        c.g[S0][0] = c.g[S1][0];
                                                                        c.sub_s(24, 31, 22);
                                                                        c.swc1(
                                                                            20,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x110,
                                                                                ),
                                                                        );
                                                                        'l_3a2f40: loop {
                                                                            c.cycles += 17;
                                                                            c.g[A1][0] = c.g[S2][0];
                                                                            c.g[A0][0] = c.lw((c.g
                                                                                [GP][0]
                                                                                as u32)
                                                                                .wrapping_sub(
                                                                                    0x844,
                                                                                ));
                                                                            c.g[A2][0] =
                                                                                sx((c.g[SP][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x110,
                                                                                    ));
                                                                            c.swc1(
                                                                                25,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x114,
                                                                                    ),
                                                                            );
                                                                            c.g[A3][0] =
                                                                                sx((c.g[SP][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x120,
                                                                                    ));
                                                                            c.swc1(
                                                                                27,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x118,
                                                                                    ),
                                                                            );
                                                                            c.g[T0][0] = c.g[S3][0];
                                                                            c.swc1(
                                                                                28,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x11c,
                                                                                    ),
                                                                            );
                                                                            c.add_s(20, 20, 27);
                                                                            c.swc1(
                                                                                21,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x120,
                                                                                    ),
                                                                            );
                                                                            c.g[S0][0] =
                                                                                sx((c.g[S0][0]
                                                                                    as u32)
                                                                                    .wrapping_sub(
                                                                                        0x1,
                                                                                    ));
                                                                            c.swc1(
                                                                                22,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x124,
                                                                                    ),
                                                                            );
                                                                            c.swc1(
                                                                                26,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x128,
                                                                                    ),
                                                                            );
                                                                            c.swc1(
                                                                                24,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x12c,
                                                                                    ),
                                                                            );
                                                                            call_ptr(
                                                                                c, 0x0038e880,
                                                                                0x003a2f7c,
                                                                            )?;
                                                                            if c.g[S0][0] != 0u64 {
                                                                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x110));
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x003a2f40);
                                                                                }
                                                                                continue 'l_3a2f40;
                                                                            } else {
                                                                                break 'b_3a2f84;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 4;
                                                                c.c_eq_s(23, 30);
                                                                let t = c.fcc();
                                                                c.g[V1][0] = c.g[S4][0];
                                                                if t {
                                                                    break 'b_3a2ff8;
                                                                } else {
                                                                    c.cycles += 25;
                                                                    c.sub_s(3, 30, 23);
                                                                    c.lwc1(
                                                                        1,
                                                                        (c.g[S5][0] as u32)
                                                                            .wrapping_add(0x8),
                                                                    );
                                                                    c.sub_s(0, 29, 21);
                                                                    c.lwc1(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x134),
                                                                    );
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[GP][0] as u32)
                                                                            .wrapping_sub(0x844));
                                                                    c.g[A1][0] = c.g[S2][0];
                                                                    c.add_s(1, 2, 1);
                                                                    c.swc1(
                                                                        20,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x110),
                                                                    );
                                                                    c.mul_s(0, 0, 3);
                                                                    c.swc1(
                                                                        25,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x114),
                                                                    );
                                                                    c.sub_s(2, 31, 22);
                                                                    c.swc1(
                                                                        28,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x11c),
                                                                    );
                                                                    c.sub_s(1, 1, 20);
                                                                    c.swc1(
                                                                        21,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x120),
                                                                    );
                                                                    c.add_s(0, 21, 0);
                                                                    c.swc1(
                                                                        22,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x124),
                                                                    );
                                                                    c.swc1(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x12c),
                                                                    );
                                                                    c.g[A2][0] = sx((c.g[SP][0]
                                                                        as u32)
                                                                        .wrapping_add(0x110));
                                                                    c.swc1(
                                                                        1,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x118),
                                                                    );
                                                                    c.g[A3][0] = sx((c.g[SP][0]
                                                                        as u32)
                                                                        .wrapping_add(0x120));
                                                                    c.sub_s(0, 0, 21);
                                                                    c.g[T0][0] = c.g[S3][0];
                                                                    c.swc1(
                                                                        0,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x128),
                                                                    );
                                                                    call_ptr(
                                                                        c, 0x0038e880, 0x003a2ff4,
                                                                    )?;
                                                                    c.g[V1][0] = c.g[S4][0];
                                                                    break 'b_3a2ff8;
                                                                }
                                                            }
                                                            c.cycles += 3;
                                                            c.g[V0][0] = ((c.g[V1][0] as i64)
                                                                < (c.g[S6][0] as i64))
                                                                as u64;
                                                            let t = c.g[V0][0] != 0u64;
                                                            c.lwc1(
                                                                25,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x13c),
                                                            );
                                                            if t {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x003a2e20,
                                                                    );
                                                                }
                                                                continue 'l_3a2e20;
                                                            } else {
                                                                break 'b_3a3004;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x140));
                                                break 'b_3a3008;
                                            }
                                        }
                                        c.cycles += 5;
                                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x130));
                                        c.c_eq_s(3, 0);
                                        if c.fcc() {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                                            break 'b_3a3228;
                                        } else {
                                            'b_3a312c: {
                                                c.cycles += 3;
                                                c.g[V0][0] = c.g[S7][0] & 0x4;
                                                let t = c.g[V0][0] == 0u64;
                                                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x134));
                                                if t {
                                                    break 'b_3a312c;
                                                } else {
                                                    'b_3a30b4: {
                                                        c.cycles += 14;
                                                        c.sub_s(3, 0, 3);
                                                        c.g[AT][0] = 0x3f800000;
                                                        c.mtc1(1, 1);
                                                        c.sub_s(2, 29, 21);
                                                        c.lwc1(
                                                            0,
                                                            (c.g[S5][0] as u32).wrapping_add(0xc),
                                                        );
                                                        c.mtc1(17, 23);
                                                        c.cvt_s_w(23, 23);
                                                        c.sub_s(1, 1, 3);
                                                        c.lwc1(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x138),
                                                        );
                                                        c.mul_s(2, 2, 1);
                                                        c.add_s(0, 3, 0);
                                                        c.add_s(21, 21, 2);
                                                        let t = (c.g[S1][0] as i64) <= 0;
                                                        c.sub_s(24, 0, 25);
                                                        if t {
                                                            break 'b_3a30b4;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.sub_s(28, 29, 21);
                                                            c.g[S0][0] = c.g[S1][0];
                                                            c.sub_s(26, 31, 22);
                                                            c.swc1(
                                                                20,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x110),
                                                            );
                                                            'l_3a3070: loop {
                                                                c.cycles += 17;
                                                                c.g[A1][0] = c.g[S2][0];
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x844));
                                                                c.g[A2][0] = sx((c.g[SP][0]
                                                                    as u32)
                                                                    .wrapping_add(0x110));
                                                                c.swc1(
                                                                    25,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x114),
                                                                );
                                                                c.g[A3][0] = sx((c.g[SP][0]
                                                                    as u32)
                                                                    .wrapping_add(0x120));
                                                                c.swc1(
                                                                    27,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x118),
                                                                );
                                                                c.g[T0][0] = c.g[S3][0];
                                                                c.swc1(
                                                                    24,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x11c),
                                                                );
                                                                c.add_s(20, 20, 27);
                                                                c.swc1(
                                                                    21,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x120),
                                                                );
                                                                c.g[S0][0] = sx((c.g[S0][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x1));
                                                                c.swc1(
                                                                    22,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x124),
                                                                );
                                                                c.swc1(
                                                                    28,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x128),
                                                                );
                                                                c.swc1(
                                                                    26,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x12c),
                                                                );
                                                                call_ptr(
                                                                    c, 0x0038ea60, 0x003a30ac,
                                                                )?;
                                                                if c.g[S0][0] != 0u64 {
                                                                    c.swc1(
                                                                        20,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x110),
                                                                    );
                                                                    if event_due(c) {
                                                                        deliver_event_here(
                                                                            c, 0x003a3070,
                                                                        );
                                                                    }
                                                                    continue 'l_3a3070;
                                                                } else {
                                                                    break 'b_3a30b4;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 4;
                                                    c.c_eq_s(23, 30);
                                                    let t = c.fcc();
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x1d0),
                                                    );
                                                    if t {
                                                        break 'b_3a3228;
                                                    } else {
                                                        c.cycles += 26;
                                                        c.sub_s(2, 30, 23);
                                                        c.lwc1(
                                                            1,
                                                            (c.g[S5][0] as u32).wrapping_add(0x8),
                                                        );
                                                        c.sub_s(0, 31, 22);
                                                        c.lwc1(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x134),
                                                        );
                                                        c.g[A0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x844));
                                                        c.g[A1][0] = c.g[S2][0];
                                                        c.add_s(1, 3, 1);
                                                        c.swc1(
                                                            20,
                                                            (c.g[SP][0] as u32).wrapping_add(0x110),
                                                        );
                                                        c.mul_s(0, 0, 2);
                                                        c.swc1(
                                                            25,
                                                            (c.g[SP][0] as u32).wrapping_add(0x114),
                                                        );
                                                        c.sub_s(3, 29, 21);
                                                        c.swc1(
                                                            24,
                                                            (c.g[SP][0] as u32).wrapping_add(0x11c),
                                                        );
                                                        c.sub_s(1, 1, 20);
                                                        c.swc1(
                                                            21,
                                                            (c.g[SP][0] as u32).wrapping_add(0x120),
                                                        );
                                                        c.add_s(0, 22, 0);
                                                        c.swc1(
                                                            22,
                                                            (c.g[SP][0] as u32).wrapping_add(0x124),
                                                        );
                                                        c.swc1(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x128),
                                                        );
                                                        c.g[T0][0] = c.g[S3][0];
                                                        c.swc1(
                                                            1,
                                                            (c.g[SP][0] as u32).wrapping_add(0x118),
                                                        );
                                                        c.g[A2][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0x110));
                                                        c.sub_s(0, 0, 22);
                                                        c.g[A3][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0x120));
                                                        c.swc1(
                                                            0,
                                                            (c.g[SP][0] as u32).wrapping_add(0x12c),
                                                        );
                                                        call_ptr(c, 0x0038ea60, 0x003a3124)?;
                                                        let t = true;
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1d0),
                                                        );
                                                        if t {
                                                            break 'b_3a3228;
                                                        } else {
                                                            break 'b_3a312c;
                                                        }
                                                    }
                                                }
                                            }
                                            'b_3a31b4: {
                                                c.cycles += 13;
                                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x130));
                                                c.mtc1(17, 23);
                                                c.cvt_s_w(23, 23);
                                                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x140));
                                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x138));
                                                c.sub_s(0, 1, 2);
                                                c.sub_s(2, 31, 22);
                                                c.lwc1(1, (c.g[S5][0] as u32).wrapping_add(0xc));
                                                c.add_s(1, 3, 1);
                                                c.mul_s(2, 2, 0);
                                                c.sub_s(24, 1, 25);
                                                let t = (c.g[S1][0] as i64) <= 0;
                                                c.add_s(31, 22, 2);
                                                if t {
                                                    break 'b_3a31b4;
                                                } else {
                                                    c.cycles += 4;
                                                    c.sub_s(28, 29, 21);
                                                    c.g[S0][0] = c.g[S1][0];
                                                    c.sub_s(26, 31, 22);
                                                    c.swc1(
                                                        20,
                                                        (c.g[SP][0] as u32).wrapping_add(0x110),
                                                    );
                                                    'l_3a3170: loop {
                                                        c.cycles += 17;
                                                        c.g[A1][0] = c.g[S2][0];
                                                        c.g[A0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x844));
                                                        c.g[A2][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0x110));
                                                        c.swc1(
                                                            25,
                                                            (c.g[SP][0] as u32).wrapping_add(0x114),
                                                        );
                                                        c.g[A3][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0x120));
                                                        c.swc1(
                                                            27,
                                                            (c.g[SP][0] as u32).wrapping_add(0x118),
                                                        );
                                                        c.g[T0][0] = c.g[S3][0];
                                                        c.swc1(
                                                            24,
                                                            (c.g[SP][0] as u32).wrapping_add(0x11c),
                                                        );
                                                        c.add_s(20, 20, 27);
                                                        c.swc1(
                                                            21,
                                                            (c.g[SP][0] as u32).wrapping_add(0x120),
                                                        );
                                                        c.g[S0][0] =
                                                            sx((c.g[S0][0] as u32)
                                                                .wrapping_sub(0x1));
                                                        c.swc1(
                                                            22,
                                                            (c.g[SP][0] as u32).wrapping_add(0x124),
                                                        );
                                                        c.swc1(
                                                            28,
                                                            (c.g[SP][0] as u32).wrapping_add(0x128),
                                                        );
                                                        c.swc1(
                                                            26,
                                                            (c.g[SP][0] as u32).wrapping_add(0x12c),
                                                        );
                                                        call_ptr(c, 0x0038e880, 0x003a31ac)?;
                                                        if c.g[S0][0] != 0u64 {
                                                            c.swc1(
                                                                20,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x110),
                                                            );
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x003a3170);
                                                            }
                                                            continue 'l_3a3170;
                                                        } else {
                                                            break 'b_3a31b4;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 4;
                                            c.c_eq_s(23, 30);
                                            let t = c.fcc();
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                                            if t {
                                                break 'b_3a3228;
                                            } else {
                                                c.cycles += 24;
                                                c.sub_s(2, 30, 23);
                                                c.lwc1(1, (c.g[S5][0] as u32).wrapping_add(0x8));
                                                c.sub_s(0, 29, 21);
                                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x134));
                                                c.g[A0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x844));
                                                c.g[A1][0] = c.g[S2][0];
                                                c.add_s(1, 3, 1);
                                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x110));
                                                c.mul_s(0, 0, 2);
                                                c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x114));
                                                c.sub_s(3, 31, 22);
                                                c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x11c));
                                                c.sub_s(1, 1, 20);
                                                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x120));
                                                c.add_s(0, 21, 0);
                                                c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x124));
                                                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x12c));
                                                c.g[T0][0] = c.g[S3][0];
                                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x118));
                                                c.g[A2][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0x110));
                                                c.sub_s(0, 0, 21);
                                                c.g[A3][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0x120));
                                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x128));
                                                call_ptr(c, 0x0038e880, 0x003a3224)?;
                                                break 'b_3a3224;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1d0));
            break 'b_3a3228;
        }
        c.cycles += 22;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x1c0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x1b0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x1a0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x190));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x180));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x170));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x150));
        c.lwc1(31, (c.g[SP][0] as u32).wrapping_add(0x238));
        c.lwc1(30, (c.g[SP][0] as u32).wrapping_add(0x230));
        c.lwc1(29, (c.g[SP][0] as u32).wrapping_add(0x228));
        c.lwc1(28, (c.g[SP][0] as u32).wrapping_add(0x220));
        c.lwc1(27, (c.g[SP][0] as u32).wrapping_add(0x218));
        c.lwc1(26, (c.g[SP][0] as u32).wrapping_add(0x210));
        c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x208));
        c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x200));
        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x1f8));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x1f0));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x1e8));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x1e0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x240));
        c.pc = t;
        Ok(())
    })
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x003a_2bc8,
    render2d_draw_tiled_sprite_xform,
    "Render2D_DrawTiledSpriteXform",
)];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the oc1078 draw fakes (own RAM word, both sides get their own).
    pub(crate) const OC1078_CTRL: u32 = DATA + 0xe_0000;
    pub(crate) const OC1078_MAGIC: u32 = 0x1078_c0de;
    /// Number of logged draw calls, then the records.
    pub(crate) const OC1078_LOG_N: u32 = OC1078_CTRL + 4;
    pub(crate) const OC1078_LOG: u32 = OC1078_CTRL + 0x100;
    pub(crate) const OC1078_ELEM: u32 = DATA + 0x1000;
    pub(crate) const OC1078_DST: u32 = DATA + 0x1100;
    pub(crate) const OC1078_COL: u32 = DATA + 0x1200;
    pub(crate) const OC1078_XF: u32 = DATA + 0x1300;
    pub(crate) const OC1078_GRMAN: u32 = DATA + 0x2000;
    pub(crate) const OC1078_DRAWSTATE: u32 = DATA + 0x3000;

    static OC1078_PREV_QUAD: AtomicUsize = AtomicUsize::new(0);
    static OC1078_PREV_ROT: AtomicUsize = AtomicUsize::new(0);

    /// A gated fake draw call: logs the rect, uv and transform the caller built
    /// on its stack (the real callees read them), then the usual fake.
    fn oc1078_fake_draw(c: &mut Cpu, target: u32) {
        let n = c.bus.read32(OC1078_LOG_N);
        c.bus.write32(OC1078_LOG_N, n + 1);
        let p = OC1078_LOG + n * 0x80;
        c.bus.write32(p, target);
        let (a1, a2, a3, t0) = (
            c.g[A1][0] as u32,
            c.g[A2][0] as u32,
            c.g[A3][0] as u32,
            c.g[T0][0] as u32,
        );
        for i in 0..4 {
            let w = c.bus.read32(a2 + 4 * i);
            c.bus.write32(p + 0x10 + 4 * i, w);
            let w = c.bus.read32(a3 + 4 * i);
            c.bus.write32(p + 0x20 + 4 * i, w);
        }
        for i in 0..9 {
            let w = c.bus.read32(t0 + 4 * i);
            c.bus.write32(p + 0x30 + 4 * i, w);
        }
        for i in 0..8 {
            let w = c.bus.read32(a1 + 4 * i);
            c.bus.write32(p + 0x58 + 4 * i, w);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, target, ra);
    }

    fn oc1078_fake_quad(c: &mut Cpu) {
        if c.bus.read32(OC1078_CTRL) != OC1078_MAGIC {
            return fakes::fall_back(c, &OC1078_PREV_QUAD, ee::s12::f_0038e880__generated);
        }
        oc1078_fake_draw(c, 0x0038_e880)
    }

    fn oc1078_fake_rot(c: &mut Cpu) {
        if c.bus.read32(OC1078_CTRL) != OC1078_MAGIC {
            return fakes::fall_back(c, &OC1078_PREV_ROT, ee::s12::f_0038ea60__generated);
        }
        oc1078_fake_draw(c, 0x0038_ea60)
    }

    fn oc1078_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038E880),
            oc1078_fake_quad,
            &OC1078_PREV_QUAD,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038EA60),
            oc1078_fake_rot,
            &OC1078_PREV_ROT,
        );
    }

    /// `Render2D_DrawTiledSpriteXform`: tile counts from none (tx or ty <= 0, the
    /// two early exits) to 3.x by 3.x, whole and fractional, each flag
    /// combination (u flip, v flip, rotated quads), random colour, rect, uv.
    #[test]
    fn render2d_draw_tiled_sprite_xform_matches() {
        let counts: [f32; 9] = [-1.0, 0.0, 0.5, 1.0, 1.5, 2.0, 2.75, 3.0, 3.25];
        fakes::covered(
            ee::s13::f_003a2bc8__generated,
            render2d_draw_tiled_sprite_xform,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a2bc8, 12);
                oc1078_install_fakes();
                c.bus.write32(OC1078_CTRL, OC1078_MAGIC);
                c.bus.write32(OC1078_LOG_N, 0);
                let flt = |r: &mut Rng| {
                    if seed % 5 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(100.0)
                    }
                };
                c.bus.write32(GAME_GP.wrapping_sub(0x854), OC1078_GRMAN);
                c.bus.write32(OC1078_GRMAN + 0xe84, OC1078_DRAWSTATE);
                c.bus.write32(GAME_GP.wrapping_sub(0x844), r.next_u32());
                c.bus.write32(OC1078_ELEM, r.next_u32() & 0xffff);
                for off in [0xc, 0x10, 0x14, 0x18] {
                    let v = flt(&mut r);
                    c.bus.write32(OC1078_ELEM + off, v);
                }
                for i in 0..4 {
                    let v = flt(&mut r);
                    c.bus.write32(OC1078_DST + 4 * i, v);
                }
                for i in 0..9 {
                    let v = if i < 4 { r.f32(1.5) } else { r.next_u32() };
                    c.bus.write32(OC1078_COL + 4 * i, v);
                }
                for i in 0..4 {
                    let v = r.next_u32();
                    c.bus.write32(OC1078_XF + 4 * i, v);
                }
                c.g[A0][0] = ssx3_ee_rt::cpu::sx(OC1078_ELEM);
                c.g[A1][0] = ssx3_ee_rt::cpu::sx(OC1078_DST);
                c.g[A2][0] = ssx3_ee_rt::cpu::sx(OC1078_COL);
                c.g[A3][0] = ssx3_ee_rt::cpu::sx(OC1078_XF);
                c.g[T0][0] = (seed / 9 % 8) as u64 | (r.next_u32() as u64 & 0x1000_0000_0000_0000);
                c.f[12] = counts[(seed % 9) as usize].to_bits();
                c.f[13] = counts[((seed / 3 + seed / 27) % 9) as usize].to_bits();
                c.pc = 0x003a_2bc8;
            },
        );
    }
}
