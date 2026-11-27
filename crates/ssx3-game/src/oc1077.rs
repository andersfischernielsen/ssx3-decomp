//! Staging module oc1077 (wave wave267): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1077`.

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
    0x003a_23b8,
    render2d_draw_tiled_sprite,
    "Render2D_DrawTiledSprite",
)];

/// Direct callees of [`render2d_draw_tiled_sprite`].
pub(crate) mod oc1077_callee {
    use super::{ee, Callee};

    /// `Render2D_DrawQuadUV(gfx, verts, rect, uv)`.
    pub(crate) const DRAW_QUAD_UV: Callee = Callee {
        addr: 0x0038_e5e8,
        f: ee::s12::Render2D_DrawQuadUV,
    };

    /// `Render2D_DrawQuadUVRot90(gfx, verts, rect, uv)`: same, UVs turned 90 degrees.
    pub(crate) const DRAW_QUAD_UV_ROT90: Callee = Callee {
        addr: 0x0038_e738,
        f: ee::s12::Render2D_DrawQuadUVRot90,
    };
}

/// `void Render2D_DrawTiledSprite(Sprite* o /*a0*/, Rect* dest /*a1*/, Colour* col /*a2*/,
/// u32 flags /*a3*/, float tx /*f12*/, float ty /*f13*/)` (0x3a23b8,
/// `cTextureElement::renderTiled`): draws the sprite's texture tiled `tx` times
/// across and `ty` times down `dest`. Sets the draw state's texture to `o->tex`,
/// builds the four colour vertices (a = 128 * alpha, r/g/b = 255 * channel) in a
/// stack template, then draws `ny = (int)ty` rows of `nx = (int)tx` whole tiles
/// (`w = dest.w / tx` wide, `h = dest.h / ty` high), a clipped column per row when
/// `tx` has a fraction, and a last clipped row when `ty` has one. `flags` bit 0
/// swaps U, bit 1 swaps V, bit 2 draws through the rot90 quad path (the UV
/// fractions then clip V instead of U). Does nothing when `tx <= 0` or `ty <= 0`.
///
/// Transcribed block for block from the generated body: every scratch register,
/// the callee-saved FPU/GPR saves and the stack scratch (rect at +0xe0, uv at
/// +0xf0, colour verts at +0x20) are reproduced.
#[allow(unused_labels, unused_assignments, unused_variables, unreachable_code)]
pub fn render2d_draw_tiled_sprite(c: &mut Cpu) {
    guest(c, |c| {
        'b_3a29a8: {
            'b_3a29a4: {
                c.cycles += 58;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x200));
                c.mtc1(0, 0);
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x150));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.g[S4][0] = c.g[SP][0];
                c.swc1(31, (c.g[SP][0] as u32).wrapping_add(0x1f8));
                c.g[S6][0] = c.g[A3][0];
                c.swc1(30, (c.g[SP][0] as u32).wrapping_add(0x1f0));
                c.swc1(29, (c.g[SP][0] as u32).wrapping_add(0x1e8));
                c.mov_s(30, 12);
                c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                c.c_le_s(30, 0);
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x1a8));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x190));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x180));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x170));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x160));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x120), c.g[RA][0]);
                c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0x1e0));
                c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0x1d8));
                c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x1c8));
                c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x1b8));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
                c.ldr(2, (c.g[A1][0] as u32));
                c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0xf));
                c.ldr(3, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(2, (c.g[SP][0] as u32));
                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
                c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.g[A0][0];
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
                c.ldr(3, (c.g[A2][0] as u32));
                c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0xf));
                c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x17));
                c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x1f));
                c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.g[A0][0] = c.lw((c.g[A1][0] as u32));
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.swc1(13, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xe84));
                c.sh((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                c.lwc1(1, (c.g[SP][0] as u32));
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x104));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x108));
                c.lwc1(22, (c.g[A1][0] as u32).wrapping_add(0xc));
                c.lwc1(31, (c.g[A1][0] as u32).wrapping_add(0x18));
                c.lwc1(21, (c.g[A1][0] as u32).wrapping_add(0x10));
                let t = c.fcc();
                c.lwc1(29, (c.g[A1][0] as u32).wrapping_add(0x14));
                if t {
                    break 'b_3a29a4;
                } else {
                    c.cycles += 5;
                    c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x100));
                    c.c_le_s(3, 0);
                    let t = c.fcc();
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x190));
                    if t {
                        break 'b_3a29a8;
                    } else {
                        'b_3a24d8: {
                            c.cycles += 7;
                            c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = c.g[S6][0] & 0x1;
                            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xc));
                            c.div_s(27, 0, 30);
                            c.div_s(28, 1, 3);
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = c.g[S6][0] & 0x2;
                            if t {
                                break 'b_3a24d8;
                            } else {
                                c.cycles += 2;
                                c.mov_s(21, 29);
                                c.lwc1(29, (c.g[A1][0] as u32).wrapping_add(0x10));
                                break 'b_3a24d8;
                            }
                        }
                        'b_3a24e8: {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;

                            if t {
                                break 'b_3a24e8;
                            } else {
                                c.cycles += 2;
                                c.mov_s(22, 31);
                                c.lwc1(31, (c.g[A1][0] as u32).wrapping_add(0xc));
                                break 'b_3a24e8;
                            }
                        }
                        c.cycles += 10;
                        c.cvt_w_s(0, 30);
                        c.mfc1(17, 0);
                        c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x108));
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x100));
                        c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[V1][0] = 0xffffffffffffffff;
                        c.cvt_w_s(0, 0);
                        c.mfc1(21, 0);
                        c.g[V0][0] = 0x3;
                        'l_3a2510: loop {
                            c.cycles += 7;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            let t = c.g[V0][0] != c.g[V1][0];
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc0));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x003a_2510);
                                }
                                continue 'l_3a2510;
                            } else {
                                c.cycles += 11;
                                c.mtc1(21, 1);
                                c.cvt_s_w(1, 1);
                                c.g[AT][0] = 0x43000000;
                                c.mtc1(1, 4);
                                c.g[AT][0] = 0x437f0000;
                                c.mtc1(1, 2);
                                c.g[V1][0] = c.g[S2][0];
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 3);
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x10c));
                                c.lwc1(0, (c.g[A2][0] as u32));
                                'l_3a2558: loop {
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
                                            deliver_event_here(c, 0x003a_2558);
                                        }
                                        continue 'l_3a2558;
                                    } else {
                                        'b_3a2798: {
                                            c.cycles += 2;
                                            if c.g[S5][0] == 0u64 {
                                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x10c));
                                                break 'b_3a2798;
                                            } else {
                                                'b_3a2794: {
                                                    c.cycles += 3;
                                                    c.lwc1(
                                                        25,
                                                        (c.g[SP][0] as u32).wrapping_add(0x108),
                                                    );
                                                    let t = (c.g[S5][0] as i64) <= 0;
                                                    c.g[V1][0] = 0u64;
                                                    if t {
                                                        break 'b_3a2794;
                                                    } else {
                                                        c.cycles += 1;
                                                        'l_3a25c0: loop {
                                                            'b_3a2788: {
                                                                'b_3a26ac: {
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = c.g[S6][0] & 0x4;
                                                                    let t = c.g[V0][0] == 0u64;
                                                                    c.lwc1(
                                                                        20,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x104),
                                                                    );
                                                                    if t {
                                                                        break 'b_3a26ac;
                                                                    } else {
                                                                        'b_3a2638: {
                                                                            c.cycles += 6;
                                                                            c.add_s(1, 25, 28);
                                                                            c.g[S3][0] =
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
                                                                                        0x110,
                                                                                    ),
                                                                            );
                                                                            if t {
                                                                                break 'b_3a2638;
                                                                            } else {
                                                                                c.cycles += 5;
                                                                                c.sub_s(26, 29, 21);
                                                                                c.g[S0][0] =
                                                                                    c.g[S1][0];
                                                                                c.sub_s(24, 31, 22);
                                                                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                                                                'l_3a25f8: loop {
                                                                                    c.cycles += 16;
                                                                                    c.g[A1][0] =
                                                                                        c.g[S2][0];
                                                                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x844));
                                                                                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
                                                                                    c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0xe4));
                                                                                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0xe8));
                                                                                    c.add_s(
                                                                                        20, 20, 27,
                                                                                    );
                                                                                    c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0xec));
                                                                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                                                                                    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xf4));
                                                                                    c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0xf8));
                                                                                    c.g[RA][0] = sx(
                                                                                        0x003a_2630,
                                                                                    );
                                                                                    c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0xfc));
                                                                                    oc1077_callee::DRAW_QUAD_UV_ROT90.call(c, 0x003a_2630)?;
                                                                                    if c.g[S0][0]
                                                                                        != 0u64
                                                                                    {
                                                                                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                                                                        if event_due(
                                                                                            c,
                                                                                        ) {
                                                                                            deliver_event_here(c, 0x003a_25f8);
                                                                                        }
                                                                                        continue 'l_3a25f8;
                                                                                    } else {
                                                                                        break 'b_3a2638;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 4;
                                                                        c.c_eq_s(23, 30);
                                                                        let t = c.fcc();
                                                                        c.g[V1][0] = c.g[S3][0];
                                                                        if t {
                                                                            break 'b_3a2788;
                                                                        } else {
                                                                            c.cycles += 25;
                                                                            c.sub_s(3, 30, 23);
                                                                            c.lwc1(
                                                                                1,
                                                                                (c.g[S4][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x8,
                                                                                    ),
                                                                            );
                                                                            c.sub_s(0, 31, 22);
                                                                            c.lwc1(
                                                                                2,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x104,
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
                                                                                        0xe0,
                                                                                    ),
                                                                            );
                                                                            c.mul_s(0, 0, 3);
                                                                            c.swc1(
                                                                                25,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xe4,
                                                                                    ),
                                                                            );
                                                                            c.sub_s(2, 29, 21);
                                                                            c.swc1(
                                                                                28,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xec,
                                                                                    ),
                                                                            );
                                                                            c.sub_s(1, 1, 20);
                                                                            c.swc1(
                                                                                21,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xf0,
                                                                                    ),
                                                                            );
                                                                            c.add_s(0, 22, 0);
                                                                            c.swc1(
                                                                                22,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xf4,
                                                                                    ),
                                                                            );
                                                                            c.swc1(
                                                                                2,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xf8,
                                                                                    ),
                                                                            );
                                                                            c.g[A2][0] =
                                                                                sx((c.g[SP][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0xe0,
                                                                                    ));
                                                                            c.swc1(
                                                                                1,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xe8,
                                                                                    ),
                                                                            );
                                                                            c.g[A3][0] =
                                                                                sx((c.g[SP][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0xf0,
                                                                                    ));
                                                                            c.sub_s(0, 0, 22);
                                                                            c.g[RA][0] =
                                                                                sx(0x003a_26a4);
                                                                            c.swc1(
                                                                                0,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xfc,
                                                                                    ),
                                                                            );
                                                                            oc1077_callee::DRAW_QUAD_UV_ROT90.call(c, 0x003a_26a4)?;
                                                                            let t = true;
                                                                            c.g[V1][0] = c.g[S3][0];
                                                                            if t {
                                                                                break 'b_3a2788;
                                                                            } else {
                                                                                break 'b_3a26ac;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_3a2718: {
                                                                    c.cycles += 6;
                                                                    c.add_s(3, 25, 28);
                                                                    c.g[S3][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x1));
                                                                    c.mtc1(17, 23);
                                                                    c.cvt_s_w(23, 23);
                                                                    let t =
                                                                        (c.g[S1][0] as i64) <= 0;
                                                                    c.swc1(
                                                                        3,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x110),
                                                                    );
                                                                    if t {
                                                                        break 'b_3a2718;
                                                                    } else {
                                                                        c.cycles += 5;
                                                                        c.sub_s(26, 29, 21);
                                                                        c.g[S0][0] = c.g[S1][0];
                                                                        c.sub_s(24, 31, 22);
                                                                        c.swc1(
                                                                            20,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xe0),
                                                                        );
                                                                        'l_3a26d8: loop {
                                                                            c.cycles += 16;
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
                                                                                        0xe0,
                                                                                    ));
                                                                            c.swc1(
                                                                                25,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xe4,
                                                                                    ),
                                                                            );
                                                                            c.g[A3][0] =
                                                                                sx((c.g[SP][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0xf0,
                                                                                    ));
                                                                            c.swc1(
                                                                                27,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xe8,
                                                                                    ),
                                                                            );
                                                                            c.add_s(20, 20, 27);
                                                                            c.swc1(
                                                                                28,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xec,
                                                                                    ),
                                                                            );
                                                                            c.g[S0][0] =
                                                                                sx((c.g[S0][0]
                                                                                    as u32)
                                                                                    .wrapping_sub(
                                                                                        0x1,
                                                                                    ));
                                                                            c.swc1(
                                                                                21,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xf0,
                                                                                    ),
                                                                            );
                                                                            c.swc1(
                                                                                22,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xf4,
                                                                                    ),
                                                                            );
                                                                            c.swc1(
                                                                                26,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xf8,
                                                                                    ),
                                                                            );
                                                                            c.g[RA][0] =
                                                                                sx(0x003a_2710);
                                                                            c.swc1(
                                                                                24,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xfc,
                                                                                    ),
                                                                            );
                                                                            oc1077_callee::DRAW_QUAD_UV
                                                                                .call(
                                                                                    c,
                                                                                    0x003a_2710,
                                                                                )?;
                                                                            if c.g[S0][0] != 0u64 {
                                                                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x003a_26d8);
                                                                                }
                                                                                continue 'l_3a26d8;
                                                                            } else {
                                                                                break 'b_3a2718;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 4;
                                                                c.c_eq_s(23, 30);
                                                                let t = c.fcc();
                                                                c.g[V1][0] = c.g[S3][0];
                                                                if t {
                                                                    break 'b_3a2788;
                                                                } else {
                                                                    c.cycles += 24;
                                                                    c.sub_s(3, 30, 23);
                                                                    c.lwc1(
                                                                        1,
                                                                        (c.g[S4][0] as u32)
                                                                            .wrapping_add(0x8),
                                                                    );
                                                                    c.sub_s(0, 29, 21);
                                                                    c.lwc1(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x104),
                                                                    );
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[GP][0] as u32)
                                                                            .wrapping_sub(0x844));
                                                                    c.g[A1][0] = c.g[S2][0];
                                                                    c.add_s(1, 2, 1);
                                                                    c.swc1(
                                                                        20,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0xe0),
                                                                    );
                                                                    c.mul_s(0, 0, 3);
                                                                    c.swc1(
                                                                        25,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0xe4),
                                                                    );
                                                                    c.sub_s(2, 31, 22);
                                                                    c.swc1(
                                                                        28,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0xec),
                                                                    );
                                                                    c.sub_s(1, 1, 20);
                                                                    c.swc1(
                                                                        21,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0xf0),
                                                                    );
                                                                    c.add_s(0, 21, 0);
                                                                    c.swc1(
                                                                        22,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0xf4),
                                                                    );
                                                                    c.swc1(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0xfc),
                                                                    );
                                                                    c.g[A2][0] = sx((c.g[SP][0]
                                                                        as u32)
                                                                        .wrapping_add(0xe0));
                                                                    c.swc1(
                                                                        1,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0xe8),
                                                                    );
                                                                    c.g[A3][0] = sx((c.g[SP][0]
                                                                        as u32)
                                                                        .wrapping_add(0xf0));
                                                                    c.sub_s(0, 0, 21);
                                                                    c.g[RA][0] = sx(0x003a_2784);
                                                                    c.swc1(
                                                                        0,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0xf8),
                                                                    );
                                                                    oc1077_callee::DRAW_QUAD_UV
                                                                        .call(c, 0x003a_2784)?;
                                                                    c.g[V1][0] = c.g[S3][0];
                                                                    break 'b_3a2788;
                                                                }
                                                            }
                                                            c.cycles += 3;
                                                            c.g[V0][0] = ((c.g[V1][0] as i64)
                                                                < (c.g[S5][0] as i64))
                                                                as u64;
                                                            let t = c.g[V0][0] != 0u64;
                                                            c.lwc1(
                                                                25,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x110),
                                                            );
                                                            if t {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c,
                                                                        0x003a_25c0,
                                                                    );
                                                                }
                                                                continue 'l_3a25c0;
                                                            } else {
                                                                break 'b_3a2794;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x10c));
                                                break 'b_3a2798;
                                            }
                                        }
                                        c.cycles += 5;
                                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x100));
                                        c.c_eq_s(3, 0);
                                        if c.fcc() {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x190));
                                            break 'b_3a29a8;
                                        } else {
                                            'b_3a28b4: {
                                                c.cycles += 3;
                                                c.g[V0][0] = c.g[S6][0] & 0x4;
                                                let t = c.g[V0][0] == 0u64;
                                                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x104));
                                                if t {
                                                    break 'b_3a28b4;
                                                } else {
                                                    'b_3a2840: {
                                                        c.cycles += 14;
                                                        c.sub_s(3, 0, 3);
                                                        c.g[AT][0] = 0x3f800000;
                                                        c.mtc1(1, 1);
                                                        c.sub_s(2, 29, 21);
                                                        c.lwc1(
                                                            0,
                                                            (c.g[S4][0] as u32).wrapping_add(0xc),
                                                        );
                                                        c.mtc1(17, 23);
                                                        c.cvt_s_w(23, 23);
                                                        c.sub_s(1, 1, 3);
                                                        c.lwc1(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x108),
                                                        );
                                                        c.mul_s(2, 2, 1);
                                                        c.add_s(0, 3, 0);
                                                        c.add_s(21, 21, 2);
                                                        let t = (c.g[S1][0] as i64) <= 0;
                                                        c.sub_s(24, 0, 25);
                                                        if t {
                                                            break 'b_3a2840;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.sub_s(28, 29, 21);
                                                            c.g[S0][0] = c.g[S1][0];
                                                            c.sub_s(26, 31, 22);
                                                            c.swc1(
                                                                20,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xe0),
                                                            );
                                                            'l_3a2800: loop {
                                                                c.cycles += 16;
                                                                c.g[A1][0] = c.g[S2][0];
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x844));
                                                                c.g[A2][0] = sx((c.g[SP][0]
                                                                    as u32)
                                                                    .wrapping_add(0xe0));
                                                                c.swc1(
                                                                    25,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0xe4),
                                                                );
                                                                c.g[A3][0] = sx((c.g[SP][0]
                                                                    as u32)
                                                                    .wrapping_add(0xf0));
                                                                c.swc1(
                                                                    27,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0xe8),
                                                                );
                                                                c.add_s(20, 20, 27);
                                                                c.swc1(
                                                                    24,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0xec),
                                                                );
                                                                c.g[S0][0] = sx((c.g[S0][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x1));
                                                                c.swc1(
                                                                    21,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0),
                                                                );
                                                                c.swc1(
                                                                    22,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf4),
                                                                );
                                                                c.swc1(
                                                                    28,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf8),
                                                                );
                                                                c.g[RA][0] = sx(0x003a_2838);
                                                                c.swc1(
                                                                    26,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0xfc),
                                                                );
                                                                oc1077_callee::DRAW_QUAD_UV_ROT90
                                                                    .call(c, 0x003a_2838)?;
                                                                if c.g[S0][0] != 0u64 {
                                                                    c.swc1(
                                                                        20,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0xe0),
                                                                    );
                                                                    if event_due(c) {
                                                                        deliver_event_here(
                                                                            c,
                                                                            0x003a_2800,
                                                                        );
                                                                    }
                                                                    continue 'l_3a2800;
                                                                } else {
                                                                    break 'b_3a2840;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 4;
                                                    c.c_eq_s(23, 30);
                                                    let t = c.fcc();
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x190),
                                                    );
                                                    if t {
                                                        break 'b_3a29a8;
                                                    } else {
                                                        c.cycles += 25;
                                                        c.sub_s(2, 30, 23);
                                                        c.lwc1(
                                                            1,
                                                            (c.g[S4][0] as u32).wrapping_add(0x8),
                                                        );
                                                        c.sub_s(0, 31, 22);
                                                        c.lwc1(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x104),
                                                        );
                                                        c.g[A0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x844));
                                                        c.g[A1][0] = c.g[S2][0];
                                                        c.add_s(1, 3, 1);
                                                        c.swc1(
                                                            20,
                                                            (c.g[SP][0] as u32).wrapping_add(0xe0),
                                                        );
                                                        c.mul_s(0, 0, 2);
                                                        c.swc1(
                                                            25,
                                                            (c.g[SP][0] as u32).wrapping_add(0xe4),
                                                        );
                                                        c.sub_s(3, 29, 21);
                                                        c.swc1(
                                                            24,
                                                            (c.g[SP][0] as u32).wrapping_add(0xec),
                                                        );
                                                        c.sub_s(1, 1, 20);
                                                        c.swc1(
                                                            21,
                                                            (c.g[SP][0] as u32).wrapping_add(0xf0),
                                                        );
                                                        c.add_s(0, 22, 0);
                                                        c.swc1(
                                                            22,
                                                            (c.g[SP][0] as u32).wrapping_add(0xf4),
                                                        );
                                                        c.swc1(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xf8),
                                                        );
                                                        c.g[A2][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0xe0));
                                                        c.swc1(
                                                            1,
                                                            (c.g[SP][0] as u32).wrapping_add(0xe8),
                                                        );
                                                        c.g[A3][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0xf0));
                                                        c.sub_s(0, 0, 22);
                                                        c.g[RA][0] = sx(0x003a_28ac);
                                                        c.swc1(
                                                            0,
                                                            (c.g[SP][0] as u32).wrapping_add(0xfc),
                                                        );
                                                        oc1077_callee::DRAW_QUAD_UV_ROT90
                                                            .call(c, 0x003a_28ac)?;
                                                        let t = true;
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x190),
                                                        );
                                                        if t {
                                                            break 'b_3a29a8;
                                                        } else {
                                                            break 'b_3a28b4;
                                                        }
                                                    }
                                                }
                                            }
                                            'b_3a2938: {
                                                c.cycles += 13;
                                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x100));
                                                c.mtc1(17, 23);
                                                c.cvt_s_w(23, 23);
                                                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x10c));
                                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x108));
                                                c.sub_s(0, 1, 2);
                                                c.sub_s(2, 31, 22);
                                                c.lwc1(1, (c.g[S4][0] as u32).wrapping_add(0xc));
                                                c.add_s(1, 3, 1);
                                                c.mul_s(2, 2, 0);
                                                c.sub_s(24, 1, 25);
                                                let t = (c.g[S1][0] as i64) <= 0;
                                                c.add_s(31, 22, 2);
                                                if t {
                                                    break 'b_3a2938;
                                                } else {
                                                    c.cycles += 4;
                                                    c.sub_s(28, 29, 21);
                                                    c.g[S0][0] = c.g[S1][0];
                                                    c.sub_s(26, 31, 22);
                                                    c.swc1(
                                                        20,
                                                        (c.g[SP][0] as u32).wrapping_add(0xe0),
                                                    );
                                                    'l_3a28f8: loop {
                                                        c.cycles += 16;
                                                        c.g[A1][0] = c.g[S2][0];
                                                        c.g[A0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x844));
                                                        c.g[A2][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0xe0));
                                                        c.swc1(
                                                            25,
                                                            (c.g[SP][0] as u32).wrapping_add(0xe4),
                                                        );
                                                        c.g[A3][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0xf0));
                                                        c.swc1(
                                                            27,
                                                            (c.g[SP][0] as u32).wrapping_add(0xe8),
                                                        );
                                                        c.add_s(20, 20, 27);
                                                        c.swc1(
                                                            24,
                                                            (c.g[SP][0] as u32).wrapping_add(0xec),
                                                        );
                                                        c.g[S0][0] =
                                                            sx((c.g[S0][0] as u32)
                                                                .wrapping_sub(0x1));
                                                        c.swc1(
                                                            21,
                                                            (c.g[SP][0] as u32).wrapping_add(0xf0),
                                                        );
                                                        c.swc1(
                                                            22,
                                                            (c.g[SP][0] as u32).wrapping_add(0xf4),
                                                        );
                                                        c.swc1(
                                                            28,
                                                            (c.g[SP][0] as u32).wrapping_add(0xf8),
                                                        );
                                                        c.g[RA][0] = sx(0x003a_2930);
                                                        c.swc1(
                                                            26,
                                                            (c.g[SP][0] as u32).wrapping_add(0xfc),
                                                        );
                                                        oc1077_callee::DRAW_QUAD_UV
                                                            .call(c, 0x003a_2930)?;
                                                        if c.g[S0][0] != 0u64 {
                                                            c.swc1(
                                                                20,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xe0),
                                                            );
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x003a_28f8);
                                                            }
                                                            continue 'l_3a28f8;
                                                        } else {
                                                            break 'b_3a2938;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 4;
                                            c.c_eq_s(23, 30);
                                            let t = c.fcc();
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x190));
                                            if t {
                                                break 'b_3a29a8;
                                            } else {
                                                c.cycles += 23;
                                                c.sub_s(2, 30, 23);
                                                c.lwc1(1, (c.g[S4][0] as u32).wrapping_add(0x8));
                                                c.sub_s(0, 29, 21);
                                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x104));
                                                c.g[A0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x844));
                                                c.g[A1][0] = c.g[S2][0];
                                                c.add_s(1, 3, 1);
                                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                                c.mul_s(0, 0, 2);
                                                c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0xe4));
                                                c.sub_s(3, 31, 22);
                                                c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0xec));
                                                c.sub_s(1, 1, 20);
                                                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
                                                c.add_s(0, 21, 0);
                                                c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xf4));
                                                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xfc));
                                                c.g[A2][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0xe0));
                                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
                                                c.g[A3][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                c.sub_s(0, 0, 21);
                                                c.g[RA][0] = sx(0x003a_29a4);
                                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xf8));
                                                oc1077_callee::DRAW_QUAD_UV.call(c, 0x003a_29a4)?;
                                                break 'b_3a29a4;
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
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x190));
            break 'b_3a29a8;
        }
        c.cycles += 21;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x180));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x170));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x150));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x120));
        c.lwc1(31, (c.g[SP][0] as u32).wrapping_add(0x1f8));
        c.lwc1(30, (c.g[SP][0] as u32).wrapping_add(0x1f0));
        c.lwc1(29, (c.g[SP][0] as u32).wrapping_add(0x1e8));
        c.lwc1(28, (c.g[SP][0] as u32).wrapping_add(0x1e0));
        c.lwc1(27, (c.g[SP][0] as u32).wrapping_add(0x1d8));
        c.lwc1(26, (c.g[SP][0] as u32).wrapping_add(0x1d0));
        c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x1c8));
        c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x1c0));
        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x1b8));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x1b0));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x1a8));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x1a0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x200));
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::{fill, scramble, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this test's quad fakes (0 = fall back to the real callee).
    const OC1077_CTRL: u32 = DATA + 0x34_0000;
    const OC1077_MAGIC: u32 = 0x1077_0c01;
    /// Call counter; the call whose index equals the word at `+8` unwinds.
    const OC1077_COUNT: u32 = OC1077_CTRL + 4;
    const OC1077_UNWIND_AT: u32 = OC1077_CTRL + 8;

    static OC1077_PREV_UV: AtomicUsize = AtomicUsize::new(0);
    static OC1077_PREV_UV_ROT90: AtomicUsize = AtomicUsize::new(0);

    fn oc1077_fake_body(c: &mut Cpu) {
        let n = c.bus.read32(OC1077_COUNT);
        c.bus.write32(OC1077_COUNT, n + 1);
        c.pc = if n == c.bus.read32(OC1077_UNWIND_AT) {
            0x0012_3450
        } else {
            c.g[RA][0] as u32
        };
    }

    fn oc1077_fake_uv(c: &mut Cpu) {
        if c.bus.read32(OC1077_CTRL) != OC1077_MAGIC {
            return fakes::fall_back(c, &OC1077_PREV_UV, ee::s12::f_0038e5e8__generated);
        }
        oc1077_fake_body(c);
    }

    fn oc1077_fake_uv_rot90(c: &mut Cpu) {
        if c.bus.read32(OC1077_CTRL) != OC1077_MAGIC {
            return fakes::fall_back(c, &OC1077_PREV_UV_ROT90, ee::s12::f_0038e738__generated);
        }
        oc1077_fake_body(c);
    }

    fn oc1077_hook() {
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038E5E8),
            oc1077_fake_uv,
            &OC1077_PREV_UV,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038E738),
            oc1077_fake_uv_rot90,
            &OC1077_PREV_UV_ROT90,
        );
    }

    const OC1077_O: u32 = DATA + 0x1234;
    const OC1077_R: u32 = DATA + 0x12_0000;
    const OC1077_C: u32 = DATA + 0x13_0000;
    const OC1077_GFX: u32 = DATA + 0x20_0000;
    const OC1077_TOP: u32 = DATA + 0x21_0000;

    /// Tile counts by seed: non-positive (early exits), under one, whole and
    /// fractional, so rows, clipped columns and the clipped last row all occur.
    const OC1077_TILES: [f32; 8] = [-1.0, 0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0];

    fn oc1077_setup(c: &mut Cpu, seed: u32, unwind_at: u32) {
        let mut r = machine(c, seed, 0x3a_23b8, 0);
        scramble(c, &mut r);
        c.bus.write32(OC1077_CTRL, OC1077_MAGIC);
        c.bus.write32(OC1077_COUNT, 0);
        c.bus.write32(OC1077_UNWIND_AT, unwind_at);
        c.pc = 0x003a_23b8;
        c.bus.write32(OC1077_O, (seed % 8) as u32);
        for off in [0xcu32, 0x10, 0x14, 0x18] {
            c.bus.write32(OC1077_O + off, r.f32(1.0));
        }
        for i in 0..4u32 {
            c.bus.write32(OC1077_R + 4 * i, r.f32(100.0));
            c.bus.write32(OC1077_C + 4 * i, r.f32(1.0));
        }
        c.g[A0][0] = sx(OC1077_O);
        c.g[A1][0] = sx(OC1077_R);
        c.g[A2][0] = sx(OC1077_C);
        c.g[A3][0] = (seed / 3 % 8) as u64;
        c.f[12] = OC1077_TILES[(seed / 5 % 8) as usize].to_bits();
        c.f[13] = OC1077_TILES[(seed / 40 % 8) as usize].to_bits();
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(GAME_GP - 0x854, OC1077_GFX);
        c.bus.write32(OC1077_GFX + 0xe84, OC1077_TOP);
        c.bus.write32(OC1077_TOP + 0x10, 0);
        c.bus.write32(GAME_GP - 0x844, OC1077_GFX + 0x100);
        fill(c, &mut r, OC1077_GFX + 0x200, 0x20);
        fill(c, &mut r, OC1077_TOP + 0x20, 0x20);
    }

    /// All flag combinations (seed / 3 % 8) against every tx and ty class
    /// (seed / 5 % 8, seed / 40 % 8), plain and rot90 quad calls.
    #[test]
    fn render2d_draw_tiled_sprite_matches() {
        oc1077_hook();
        check(
            ee::s13::f_003a23b8__generated,
            render2d_draw_tiled_sprite,
            512,
            &|c, seed| oc1077_setup(c, seed, u32::MAX),
        );
    }

    /// Every quad call in turn unwinds (the caller returns at once).
    #[test]
    fn render2d_draw_tiled_sprite_unwinds() {
        oc1077_hook();
        check(
            ee::s13::f_003a23b8__generated,
            render2d_draw_tiled_sprite,
            256,
            &|c, seed| oc1077_setup(c, seed * 7 + 1, seed % 6),
        );
    }

    #[test]
    fn render2d_draw_tiled_sprite_events() {
        oc1077_hook();
        driven!(gen_tiled, ee::s13::f_003a23b8__generated);
        driven!(hand_tiled, render2d_draw_tiled_sprite);
        check_events(gen_tiled, hand_tiled, 120, &|c, seed| {
            oc1077_setup(c, 270 + seed / 3 % 15, u32::MAX);
            schedule(
                c,
                &mut Rng(0x3a_23b8 ^ (seed as u64) << 8),
                seed,
                0x003a_23b8,
            )
        });
    }
}
