//! Staging wave 201/202 (oc217): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

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
        0x0020_eda0,
        c_ov_state_hud1p_draw_progress_meter,
        "cOVStateHUD1P_DrawProgressMeter",
    ),
    (0x002d_5048, c_anger_fx_render, "cAngerFX_Render"),
    (0x0033_4888, collide_volume_object, "Collide_VolumeObject"),
    (0x003c_cf90, snd_eaxa_decode, "SND_EaxaDecode"),
    (
        0x0011_4130,
        rider_handle_boost_input,
        "Rider_HandleBoostInput",
    ),
    (0x0012_10b0, rider_update_impacts, "Rider_UpdateImpacts"),
    (0x002b_d5a8, twpigd_lighting_blend, "tWPIGD_Lighting_Blend"),
    (0x0028_aaf8, snd_load_queue_update, "SndLoadQueue_Update"),
    (
        0x0032_9590,
        collide_sphere_vs_box_edges,
        "Collide_SphereVsBoxEdges",
    ),
    (
        0x003b_7c40,
        sndstrm_get_request_status,
        "SNDSTRM_GetRequestStatus",
    ),
];

/// `void cOVStateHUD1P::DrawProgressMeter(Rider* player)` (0x20eda0,
/// `src/fe/ovstatetrophy.cpp`, asm only in the reference): the race
/// progress meter of the 1P HUD.
///
/// Skipped when the HUD is hidden (`gp-0x990`), the meter is off
/// (`gp-0x994`), there is no player or no meter definition (`gp-0x9a0`).
/// Otherwise it asks the player for its racer slot (rider vtable +0x38),
/// slews each racer's displayed position (table 0x4c8bc8, six 0x18-byte
/// entries: +0x8 course distance, +0x10 shown position, +0x14 visible)
/// toward its distance (clamped to +-2000, stepped by the `.lit4` speeds
/// `gp-0x54ec`/`gp-0x54e8`), opens a sprite batch on the renderer
/// (`gp-0x854`, vtable at +0x10d8; GS packet pointer at +0xe84), draws the
/// percentage text (`sprintf("%d%%")` + `Font_DrawTextCentered` with the
/// HUD font at `gp-0x98c`), then the bar, every racer marker that falls in
/// the visible window (player distance -5000 .. +5000), the course markers
/// listed at `gp-0x998` (count at meter def +0x8), closes the batch and
/// remembers the player slot at `self+0x34`.
///
/// The body keeps the original's block structure (each block's cycle
/// charge, the scratch registers it leaves, the `.lit4` reads through
/// `gp`), with the calls through the call protocol and the loop back edges'
/// event checks through [`event_due`]/[`deliver_event_here`].
pub fn c_ov_state_hud1p_draw_progress_meter(c: &mut Cpu) {
    guest(c, |c| {
        'b_20faf8: {
            'b_20faf4: {
                c.cycles += 21;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x1d0));
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x990));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
                c.g[S1][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x160));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x150));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.sd((c.g[SP][0] as u32).wrapping_add(0xf0), c.g[RA][0]);
                c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x1b8));
                c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x1a8));
                c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x198));
                let t = c.g[V0][0] != 0u64;
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x190));
                if t {
                    break 'b_20faf4;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x994));
                    let t = c.g[V0][0] == 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
                    if t {
                        break 'b_20faf8;
                    } else {
                        c.cycles += 2;
                        let t = c.g[A1][0] == 0u64;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x998));
                        if t {
                            break 'b_20faf8;
                        } else {
                            c.cycles += 3;
                            c.g[S7][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x9a0));
                            let t = c.g[S7][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[V0][0]);
                            if t {
                                break 'b_20faf8;
                            } else {
                                c.cycles += 33;
                                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x6c0));
                                c.g[S0][0] = 0x4c0000;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c0));
                                c.g[S5][0] = 0x4c0000;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                                let t = c.g[V0][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x0020ee34)?;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[S6][0] = c.g[V0][0];
                                c.g[V1][0] = 0x18;
                                c.g[V0][0] = 0x4d0000;
                                c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                c.mult(0, 3, 22, 3);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7438));
                                c.g[AT][0] = 0xffffffffc4fa0000;
                                c.mtc1(1, 7);
                                c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                                c.g[FP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                                c.g[AT][0] = 0x44fa0000;
                                c.mtc1(1, 6);
                                c.g[A3][0] = 0x1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0xe4), c.g[A1][0]);
                                c.g[A0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                c.g[V1][0] = c.g[A0][0];
                                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x54ec));
                                c.lwc1(20, (c.g[V1][0] as u32).wrapping_add(0x8));
                                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe4));
                                c.g[A1][0] = 0u64;
                                c.lwc1(4, (c.g[A0][0] as u32).wrapping_add(0xc));
                                c.g[S3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x78));
                                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
                                'l_20ee98: loop {
                                    'b_20eea4: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
                                        if c.g[V0][0] != c.g[S6][0] {
                                            c.sw((c.g[V1][0] as u32), 0u64);
                                            break 'b_20eea4;
                                        } else {
                                            break 'b_20eea4;
                                        }
                                    }
                                    'b_20ef30: {
                                        c.cycles += 3;
                                        c.g[V0][0] =
                                            ((c.g[A1][0] as i64) < (c.g[S3][0] as i64)) as u64;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                            break 'b_20ef30;
                                        } else {
                                            'b_20eeec: {
                                                'b_20eed4: {
                                                    c.cycles += 5;
                                                    c.lwc1(
                                                        0,
                                                        (c.g[V1][0] as u32).wrapping_sub(0x8),
                                                    );
                                                    c.sub_s(0, 0, 4);
                                                    c.c_lt_s(0, 7);
                                                    let t = !c.fcc();

                                                    if t {
                                                        break 'b_20eed4;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[AT][0] = 0xffffffffc4fa0000;
                                                        c.mtc1(1, 0);
                                                        let t = true;
                                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                                        if t {
                                                            break 'b_20eeec;
                                                        } else {
                                                            break 'b_20eed4;
                                                        }
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.c_lt_s(6, 0);
                                                let t = !c.fcc();
                                                c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                                if t {
                                                    break 'b_20eeec;
                                                } else {
                                                    c.cycles += 2;
                                                    c.g[AT][0] = 0x44fa0000;
                                                    c.mtc1(1, 0);
                                                    break 'b_20eeec;
                                                }
                                            }
                                            'b_20ef28: {
                                                c.cycles += 2;
                                                if c.g[V0][0] == 0u64 {
                                                    c.swc1(0, (c.g[A2][0] as u32));
                                                    break 'b_20ef28;
                                                } else {
                                                    'b_20ef24: {
                                                        'b_20ef14: {
                                                            c.cycles += 6;
                                                            c.lwc1(2, (c.g[A2][0] as u32));
                                                            c.lwc1(
                                                                3,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x54e8),
                                                            );
                                                            c.sub_s(1, 0, 2);
                                                            c.c_lt_s(3, 1);
                                                            let t = !c.fcc();

                                                            if t {
                                                                break 'b_20ef14;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.add_s(0, 2, 3);
                                                                if t {
                                                                    break 'b_20ef24;
                                                                } else {
                                                                    break 'b_20ef14;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.c_lt_s(1, 5);
                                                        if c.fcc() {
                                                            c.sub_s(0, 2, 3);
                                                            break 'b_20ef24;
                                                        } else {
                                                            break 'b_20ef24;
                                                        }
                                                    }
                                                    c.cycles += 1;
                                                    c.swc1(0, (c.g[A2][0] as u32));
                                                    break 'b_20ef28;
                                                }
                                            }
                                            c.cycles += 2;
                                            c.sw((c.g[V1][0] as u32), c.g[A3][0]);
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                            break 'b_20ef30;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x18));
                                    c.g[V0][0] = ((c.g[A1][0] as i64) < 6i64) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x18));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0020ee98);
                                        }
                                        continue 'l_20ee98;
                                    } else {
                                        c.cycles += 2;
                                        c.g[V0][0] = 0x3;
                                        c.g[V1][0] = 0xffffffffffffffff;
                                        'l_20ef48: loop {
                                            c.cycles += 7;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                            let t = c.g[V0][0] != c.g[V1][0];
                                            c.g[A0][0] = 0x80;
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x0020ef48);
                                                }
                                                continue 'l_20ef48;
                                            } else {
                                                c.cycles += 5;
                                                c.g[AT][0] = 0x3f800000;
                                                c.mtc1(1, 0);
                                                c.g[V1][0] = c.g[SP][0];
                                                c.g[A1][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                                                c.sw(
                                                    (c.g[V1][0] as u32).wrapping_add(0x10),
                                                    c.g[A0][0],
                                                );
                                                'l_20ef78: loop {
                                                    c.cycles += 8;
                                                    c.sw(
                                                        (c.g[V1][0] as u32).wrapping_add(0x14),
                                                        c.g[A0][0],
                                                    );
                                                    c.sw(
                                                        (c.g[V1][0] as u32).wrapping_add(0x18),
                                                        c.g[A0][0],
                                                    );
                                                    c.sw(
                                                        (c.g[V1][0] as u32).wrapping_add(0x1c),
                                                        c.g[A0][0],
                                                    );
                                                    c.swc1(
                                                        0,
                                                        (c.g[V1][0] as u32).wrapping_add(0x8),
                                                    );
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x30));
                                                    c.g[V0][0] = ((c.g[V1][0] as i64)
                                                        < (c.g[A1][0] as i64))
                                                        as u64;
                                                    if c.g[V0][0] != 0u64 {
                                                        c.sw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x10),
                                                            c.g[A0][0],
                                                        );
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x0020ef78);
                                                        }
                                                        continue 'l_20ef78;
                                                    } else {
                                                        'b_20f114: {
                                                            c.cycles += 86;
                                                            c.g[A1][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                            c.g[V0][0] = c.lw((c.g[A1][0] as u32)
                                                                .wrapping_add(0x10d8));
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0xf8));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0xfc));
                                                            let t = c.g[V1][0] as u32;
                                                            c.g[A0][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add((c.g[A0][0] as u32)));
                                                            call_ptr(c, t, 0x0020efb0)?;
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                            c.g[T0][0] = 0xfffffffffe7f0000;
                                                            c.g[T0][0] = c.g[T0][0] | 0xffff;
                                                            c.g[T4][0] = 0x1000000;
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[T1][0] = 0xffffffffffcf0000;
                                                            c.g[T1][0] = c.g[T1][0] | 0xffff;
                                                            c.g[T3][0] = 0xfffffffffff00000;
                                                            c.ldl(
                                                                5,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(5, (c.g[V0][0] as u32));
                                                            c.ldl(
                                                                6,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                6,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[A3][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.sdl(
                                                                5,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x1b),
                                                            );
                                                            c.sdr(
                                                                5,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x14),
                                                            );
                                                            c.sdl(
                                                                6,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x23),
                                                            );
                                                            c.sdr(
                                                                6,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x1c),
                                                            );
                                                            c.sw(
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x24),
                                                                c.g[A3][0],
                                                            );
                                                            c.g[T3][0] = c.g[T3][0] | 0xfff;
                                                            c.g[T2][0] = 0x10000;
                                                            c.g[T5][0] = 0xffffffffffffff83;
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[T2][0] = c.g[T2][0] | 0x4000;
                                                            c.g[A2][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                            c.g[A1][0] = 0x500000;
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x14));
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_sub(0xe60));
                                                            c.sw(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0xe84),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[V0][0] = 0x500000;
                                                            c.g[V1][0] = c.lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[T6][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x1420));
                                                            c.ldl(
                                                                4,
                                                                (c.g[T6][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(4, (c.g[T6][0] as u32));
                                                            c.ldl(
                                                                6,
                                                                (c.g[T6][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                6,
                                                                (c.g[T6][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[A3][0] = c.lw((c.g[T6][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.sdl(
                                                                4,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(4, (c.g[V1][0] as u32));
                                                            c.sdl(
                                                                6,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                6,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sw(
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x10),
                                                                c.g[A3][0],
                                                            );
                                                            c.g[A3][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                            c.g[A0][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                                                            c.g[V0][0] = c.g[V0][0] | c.g[T4][0];
                                                            c.sw(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x4),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[V1][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                                                            c.sw(
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x4),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[A0][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.g[V0][0] = c.g[V0][0] & c.g[T3][0];
                                                            c.g[V0][0] = c.g[V0][0] | c.g[T2][0];
                                                            c.sw(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x4),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[A2][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[V0][0] = c.lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.g[V0][0] = c.g[V0][0] & c.g[T5][0];
                                                            c.g[V0][0] = c.g[V0][0] | 0x14;
                                                            c.sw(
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x4),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[V1][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0x10d8));
                                                            c.g[A0][0] = c.lh((c.g[V1][0] as u32)
                                                                .wrapping_add(0x110));
                                                            c.g[A2][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x114));
                                                            let t = c.g[A2][0] as u32;
                                                            c.g[A0][0] = sx((c.g[A3][0] as u32)
                                                                .wrapping_add((c.g[A0][0] as u32)));
                                                            call_ptr(c, t, 0x0020f0b8)?;
                                                            c.g[V0][0] = 0x18;
                                                            c.lwc1(
                                                                0,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x54e4),
                                                            );
                                                            c.mult(0, 2, 22, 2);
                                                            c.g[A0][0] = 0x4d0000;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_sub(0x7438));
                                                            c.sub_s(25, 20, 0);
                                                            c.lwc1(
                                                                1,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x54e0),
                                                            );
                                                            c.lwc1(
                                                                2,
                                                                (c.g[S7][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            c.g[V0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add((c.g[V0][0] as u32)));
                                                            c.g[V1][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_add(0x1dc4));
                                                            c.lwc1(
                                                                0,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.div_s(26, 0, 2);
                                                            let t = c.g[V1][0] != 0u64;
                                                            c.add_s(24, 25, 1);
                                                            if t {
                                                                break 'b_20f114;
                                                            } else {
                                                                c.cycles += 9;
                                                                c.g[AT][0] = 0x3f800000;
                                                                c.mtc1(1, 0);
                                                                c.g[V0][0] = sx((c.g[S0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x6868));
                                                                c.g[V1][0] = 0x1;
                                                                c.swc1(
                                                                    0,
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0xc),
                                                                );
                                                                c.sw(
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_add(0x1dc4),
                                                                    c.g[V1][0],
                                                                );
                                                                c.swc1(
                                                                    0,
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x6868),
                                                                );
                                                                c.swc1(
                                                                    0,
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x4),
                                                                );
                                                                c.swc1(
                                                                    0,
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x8),
                                                                );
                                                                break 'b_20f114;
                                                            }
                                                        }
                                                        'b_20f148: {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_add(0x1dc8));
                                                            let t = c.g[V0][0] != 0u64;
                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_add(0x1dd8));
                                                            if t {
                                                                break 'b_20f148;
                                                            } else {
                                                                c.cycles += 10;
                                                                c.g[V0][0] = sx((c.g[S5][0]
                                                                    as u32)
                                                                    .wrapping_add(0x6878));
                                                                c.g[AT][0] = 0x3f800000;
                                                                c.mtc1(1, 0);
                                                                c.sw(
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0xc),
                                                                    0u64,
                                                                );
                                                                c.g[V1][0] = 0x1;
                                                                c.swc1(
                                                                    0,
                                                                    (c.g[S5][0] as u32)
                                                                        .wrapping_add(0x6878),
                                                                );
                                                                c.sw(
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_add(0x1dc8),
                                                                    c.g[V1][0],
                                                                );
                                                                c.sw(
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x4),
                                                                    0u64,
                                                                );
                                                                c.sw(
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x8),
                                                                    0u64,
                                                                );
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_add(0x1dd8));
                                                                break 'b_20f148;
                                                            }
                                                        }
                                                        'b_20f164: {
                                                            c.cycles += 2;
                                                            let t = c.g[V0][0] != 0u64;
                                                            c.g[V0][0] = 0x1;
                                                            if t {
                                                                break 'b_20f164;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.g[AT][0] = 0x40000000;
                                                                c.mtc1(1, 0);
                                                                c.sw(
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_add(0x1dd8),
                                                                    c.g[V0][0],
                                                                );
                                                                c.swc1(
                                                                    0,
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_add(0x1dd4),
                                                                );
                                                                c.swc1(
                                                                    0,
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_add(0x1dd0),
                                                                );
                                                                break 'b_20f164;
                                                            }
                                                        }
                                                        'b_20f3f0: {
                                                            c.cycles += 67;
                                                            c.g[AT][0] = 0x42c80000;
                                                            c.mtc1(1, 0);
                                                            c.g[V0][0] = 0x4d0000;
                                                            c.g[AT][0] = 0x3f000000;
                                                            c.mtc1(1, 1);
                                                            c.g[A1][0] = 0x4a0000;
                                                            c.mul_s(0, 26, 0);
                                                            c.g[A0][0] = c.g[FP][0];
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x2690));
                                                            c.g[S4][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x7368));
                                                            c.add_s(0, 0, 1);
                                                            c.cvt_w_s(1, 0);
                                                            c.mfc1(6, 1);
                                                            c.g[S2][0] = 0u64;
                                                            call_ptr(c, 0x00417828, 0x0020f1a0)?;
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x98c));
                                                            c.g[A3][0] = 0xfffffffffffffc1f;
                                                            c.lwc1(
                                                                2,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x54dc),
                                                            );
                                                            c.g[A1][0] = c.g[FP][0];
                                                            c.g[T0][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x6868));
                                                            c.ldl(
                                                                2,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(2, (c.g[T0][0] as u32));
                                                            c.ldl(
                                                                3,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                3,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                2,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x47),
                                                            );
                                                            c.sdr(
                                                                2,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x40),
                                                            );
                                                            c.sdl(
                                                                3,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x4f),
                                                            );
                                                            c.sdr(
                                                                3,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x48),
                                                            );
                                                            c.g[V1][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x98c));
                                                            c.lwc1(
                                                                0,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x30),
                                                            );
                                                            c.lwc1(
                                                                1,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x34),
                                                            );
                                                            c.mul_s(0, 0, 2);
                                                            c.mul_s(1, 1, 2);
                                                            c.swc1(
                                                                0,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x38),
                                                            );
                                                            c.swc1(
                                                                1,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x3c),
                                                            );
                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x98c));
                                                            c.g[A2][0] = sx((c.g[S5][0] as u32)
                                                                .wrapping_add(0x6878));
                                                            c.ldl(
                                                                14,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(14, (c.g[A2][0] as u32));
                                                            c.ldl(
                                                                3,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                3,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                14,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x57),
                                                            );
                                                            c.sdr(
                                                                14,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x50),
                                                            );
                                                            c.sdl(
                                                                3,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x5f),
                                                            );
                                                            c.sdr(
                                                                3,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x58),
                                                            );
                                                            c.g[V1][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x98c));
                                                            c.ldl(
                                                                8,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_add(0x1dd7),
                                                            );
                                                            c.ldr(
                                                                8,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_add(0x1dd0),
                                                            );
                                                            c.sdl(
                                                                8,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x2f),
                                                            );
                                                            c.sdr(
                                                                8,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x28),
                                                            );
                                                            c.g[A2][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x98c));
                                                            c.g[V1][0] = c.lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
                                                            c.g[V0][0] = c.g[V0][0] | 0x140;
                                                            c.sw(
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                                c.g[V0][0],
                                                            );
                                                            c.lwc1(12, (c.g[S1][0] as u32));
                                                            c.lwc1(
                                                                13,
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            call_ptr(c, 0x00392908, 0x0020f254)?;
                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x98c));
                                                            c.lwc1(
                                                                0,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x30),
                                                            );
                                                            c.lwc1(
                                                                1,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x34),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x38),
                                                            );
                                                            c.swc1(
                                                                1,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x3c),
                                                            );
                                                            let t = (c.g[S3][0] as i64) <= 0;
                                                            c.g[S0][0] = c.lw((c.g[S4][0] as u32)
                                                                .wrapping_add(0xc));
                                                            if t {
                                                                break 'b_20f3f0;
                                                            } else {
                                                                c.cycles += 6;
                                                                c.g[AT][0] = 0x3f800000;
                                                                c.mtc1(1, 20);
                                                                c.g[S5][0] = 0x4d0000;
                                                                c.g[S5][0] = sx((c.g[S5][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x7438));
                                                                c.g[FP][0] = 0xfffffffffffffc1f;
                                                                c.g[V0][0] = 0x18;
                                                                'l_20f288: loop {
                                                                    'b_20f3e4: {
                                                                        'b_20f2b8: {
                                                                            c.cycles += 7;
                                                                            c.mult(0, 2, 18, 2);
                                                                            c.g[V1][0] =
                                                                                sx((c.g[S5][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        (c.g[V0][0]
                                                                                            as u32),
                                                                                    ));
                                                                            c.lwc1(
                                                                                1,
                                                                                (c.g[V1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x8,
                                                                                    ),
                                                                            );
                                                                            c.c_le_s(25, 1);
                                                                            let t = !c.fcc();
                                                                            c.g[A1][0] = 0x4d0000;
                                                                            if t {
                                                                                break 'b_20f2b8;
                                                                            } else {
                                                                                c.cycles += 4;
                                                                                c.c_le_s(1, 24);
                                                                                if c.fcc() {
                                                                                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                                                                    break 'b_20f3e4;
                                                                                } else {
                                                                                    c.cycles += 1;
                                                                                    c.g[A1][0] =
                                                                                        0x4d0000;
                                                                                    break 'b_20f2b8;
                                                                                }
                                                                            }
                                                                        }
                                                                        'b_20f30c: {
                                                                            c.cycles += 18;
                                                                            c.lwc1(
                                                                                4,
                                                                                (c.g[GP][0] as u32)
                                                                                    .wrapping_sub(
                                                                                        0x54d8,
                                                                                    ),
                                                                            );
                                                                            c.g[A1][0] =
                                                                                sx((c.g[A1][0]
                                                                                    as u32)
                                                                                    .wrapping_sub(
                                                                                        0x7428,
                                                                                    ));
                                                                            c.c_lt_s(24, 1);
                                                                            c.g[V0][0] =
                                                                                sx((c.g[V0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        (c.g[A1][0]
                                                                                            as u32),
                                                                                    ));
                                                                            c.lwc1(
                                                                                5,
                                                                                (c.g[S0][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x18,
                                                                                    ),
                                                                            );
                                                                            c.lwc1(
                                                                                0,
                                                                                (c.g[V0][0] as u32),
                                                                            );
                                                                            c.lwc1(
                                                                                3,
                                                                                (c.g[S1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x10,
                                                                                    ),
                                                                            );
                                                                            c.mov_s(8, 5);
                                                                            c.neg_s(0, 0);
                                                                            c.lwc1(
                                                                                2,
                                                                                (c.g[S4][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x4,
                                                                                    ),
                                                                            );
                                                                            c.g[AT][0] = 0x3f000000;
                                                                            c.mtc1(1, 1);
                                                                            c.lwc1(
                                                                                7,
                                                                                (c.g[S0][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xc,
                                                                                    ),
                                                                            );
                                                                            c.mul_s(0, 0, 4);
                                                                            c.lwc1(
                                                                                6,
                                                                                (c.g[S1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xc,
                                                                                    ),
                                                                            );
                                                                            c.mul_s(2, 2, 1);
                                                                            let t = !c.fcc();
                                                                            c.add_s(4, 3, 0);
                                                                            if t {
                                                                                break 'b_20f30c;
                                                                            } else {
                                                                                c.cycles += 3;
                                                                                c.mov_s(5, 7);
                                                                                c.lwc1(6, (c.g[S1][0] as u32).wrapping_add(0x8));
                                                                                c.mov_s(7, 8);
                                                                                break 'b_20f30c;
                                                                            }
                                                                        }
                                                                        c.cycles += 54;
                                                                        c.lwc1(
                                                                            0,
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(0x10),
                                                                        );
                                                                        c.sub_s(3, 4, 2);
                                                                        c.swc1(
                                                                            5,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x4),
                                                                        );
                                                                        c.add_s(4, 4, 2);
                                                                        c.swc1(
                                                                            0,
                                                                            (c.g[SP][0] as u32),
                                                                        );
                                                                        c.g[A1][0] = 0x4;
                                                                        c.g[A2][0] = c.g[SP][0];
                                                                        c.g[A3][0] = 0u64;
                                                                        c.lwc1(
                                                                            1,
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(0x14),
                                                                        );
                                                                        c.lwc1(
                                                                            2,
                                                                            (c.g[S4][0] as u32)
                                                                                .wrapping_add(0x8),
                                                                        );
                                                                        c.swc1(
                                                                            5,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x34),
                                                                        );
                                                                        c.swc1(
                                                                            1,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x30),
                                                                        );
                                                                        c.add_s(2, 6, 2);
                                                                        c.lwc1(
                                                                            0,
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(0x10),
                                                                        );
                                                                        c.swc1(
                                                                            7,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x64),
                                                                        );
                                                                        c.swc1(
                                                                            0,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x60),
                                                                        );
                                                                        c.lwc1(
                                                                            1,
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(0x14),
                                                                        );
                                                                        c.swc1(
                                                                            3,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd0),
                                                                        );
                                                                        c.sw(
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd8),
                                                                            0u64,
                                                                        );
                                                                        c.swc1(
                                                                            6,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd4),
                                                                        );
                                                                        c.swc1(
                                                                            20,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xdc),
                                                                        );
                                                                        c.swc1(
                                                                            1,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x90),
                                                                        );
                                                                        c.lq(
                                                                            4,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd0),
                                                                        );
                                                                        c.swc1(
                                                                            4,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd0),
                                                                        );
                                                                        c.sw(
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd8),
                                                                            0u64,
                                                                        );
                                                                        c.swc1(
                                                                            7,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x94),
                                                                        );
                                                                        c.lq(
                                                                            3,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd0),
                                                                        );
                                                                        c.swc1(
                                                                            3,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd0),
                                                                        );
                                                                        c.swc1(
                                                                            2,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd4),
                                                                        );
                                                                        c.sw(
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd8),
                                                                            0u64,
                                                                        );
                                                                        c.sq(
                                                                            3,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x50),
                                                                        );
                                                                        c.lq(
                                                                            2,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd0),
                                                                        );
                                                                        c.swc1(
                                                                            4,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd0),
                                                                        );
                                                                        c.sq(
                                                                            2,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x80),
                                                                        );
                                                                        c.sq(
                                                                            4,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x20),
                                                                        );
                                                                        c.sw(
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd8),
                                                                            0u64,
                                                                        );
                                                                        c.lq(
                                                                            2,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd0),
                                                                        );
                                                                        c.g[T0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_sub(0x854));
                                                                        c.sq(
                                                                            2,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xb0),
                                                                        );
                                                                        c.g[A0][0] = c.lw((c.g[T0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0xe84));
                                                                        c.g[V0][0] = c
                                                                            .lw((c.g[A0][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        c.g[V0][0] =
                                                                            c.g[V0][0] & c.g[FP][0];
                                                                        c.g[V0][0] =
                                                                            c.g[V0][0] | 0x140;
                                                                        c.sw(
                                                                            (c.g[A0][0] as u32)
                                                                                .wrapping_add(0x8),
                                                                            c.g[V0][0],
                                                                        );
                                                                        c.g[V1][0] = c.lw((c.g[T0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0xe84));
                                                                        c.g[V0][0] =
                                                                            c.lw(
                                                                                (c.g[S0][0] as u32)
                                                                            );
                                                                        c.sh(
                                                                            (c.g[V1][0] as u32)
                                                                                .wrapping_add(0x10),
                                                                            c.g[V0][0],
                                                                        );
                                                                        c.g[T0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_sub(0x854));
                                                                        c.g[V0][0] = c.lw((c.g[T0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10d8));
                                                                        c.g[A0][0] = c.lh((c.g[V0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x238));
                                                                        c.g[V1][0] = c.lw((c.g[V0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x23c));
                                                                        let t = c.g[V1][0] as u32;
                                                                        c.g[A0][0] =
                                                                            sx((c.g[T0][0] as u32)
                                                                                .wrapping_add(
                                                                                    (c.g[A0][0]
                                                                                        as u32),
                                                                                ));
                                                                        call_ptr(c, t, 0x0020f3e0)?;
                                                                        c.g[S2][0] =
                                                                            sx((c.g[S2][0] as u32)
                                                                                .wrapping_add(0x1));
                                                                        break 'b_20f3e4;
                                                                    }
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = ((c.g[S2][0]
                                                                        as i64)
                                                                        < (c.g[S3][0] as i64))
                                                                        as u64;
                                                                    let t = c.g[V0][0] != 0u64;
                                                                    c.g[V0][0] = 0x18;
                                                                    if t {
                                                                        if event_due(c) {
                                                                            deliver_event_here(
                                                                                c, 0x0020f288,
                                                                            );
                                                                        }
                                                                        continue 'l_20f288;
                                                                    } else {
                                                                        break 'b_20f3f0;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        'b_20f420: {
                                                            c.cycles += 8;
                                                            c.lwc1(
                                                                0,
                                                                (c.g[S7][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            c.g[A2][0] = 0x4d0000;
                                                            c.g[V0][0] = sx((c.g[A2][0] as u32)
                                                                .wrapping_sub(0x73a8));
                                                            c.lwc1(
                                                                21,
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x18),
                                                            );
                                                            c.c_lt_s(0, 24);
                                                            c.g[T2][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0xc));
                                                            let t = !c.fcc();
                                                            c.lwc1(
                                                                22,
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x14),
                                                            );
                                                            if t {
                                                                break 'b_20f420;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.sub_s(0, 24, 0);
                                                                c.lwc1(
                                                                    1,
                                                                    (c.g[S1][0] as u32)
                                                                        .wrapping_add(0x1c),
                                                                );
                                                                c.mul_s(0, 0, 1);
                                                                c.add_s(22, 22, 0);
                                                                break 'b_20f420;
                                                            }
                                                        }
                                                        'b_20f444: {
                                                            c.cycles += 5;
                                                            c.mtc1(0, 23);
                                                            c.c_lt_s(25, 23);
                                                            if !c.fcc() {
                                                                c.lwc1(
                                                                    0,
                                                                    (c.g[T2][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                );
                                                                break 'b_20f444;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.lwc1(
                                                                    0,
                                                                    (c.g[S1][0] as u32)
                                                                        .wrapping_add(0x1c),
                                                                );
                                                                c.mul_s(0, 25, 0);
                                                                c.add_s(21, 21, 0);
                                                                c.lwc1(
                                                                    0,
                                                                    (c.g[T2][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                );
                                                                break 'b_20f444;
                                                            }
                                                        }
                                                        c.cycles += 83;
                                                        c.g[T1][0] = 0xfffffffffffffc1f;
                                                        c.lwc1(
                                                            2,
                                                            (c.g[S1][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.g[A1][0] = 0x4;
                                                        c.swc1(0, (c.g[SP][0] as u32));
                                                        c.g[A2][0] = c.g[SP][0];
                                                        c.lwc1(
                                                            1,
                                                            (c.g[GP][0] as u32)
                                                                .wrapping_sub(0x54d4),
                                                        );
                                                        c.g[A3][0] = 0u64;
                                                        c.lwc1(
                                                            0,
                                                            (c.g[T2][0] as u32).wrapping_add(0xc),
                                                        );
                                                        c.sub_s(3, 2, 1);
                                                        c.g[AT][0] = 0x3f800000;
                                                        c.mtc1(1, 20);
                                                        c.swc1(
                                                            0,
                                                            (c.g[SP][0] as u32).wrapping_add(0x4),
                                                        );
                                                        c.add_s(2, 2, 1);
                                                        c.lwc1(
                                                            0,
                                                            (c.g[T2][0] as u32).wrapping_add(0x14),
                                                        );
                                                        c.swc1(
                                                            0,
                                                            (c.g[SP][0] as u32).wrapping_add(0x30),
                                                        );
                                                        c.lwc1(
                                                            1,
                                                            (c.g[T2][0] as u32).wrapping_add(0xc),
                                                        );
                                                        c.swc1(
                                                            1,
                                                            (c.g[SP][0] as u32).wrapping_add(0x34),
                                                        );
                                                        c.lwc1(
                                                            0,
                                                            (c.g[T2][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.swc1(
                                                            0,
                                                            (c.g[SP][0] as u32).wrapping_add(0x60),
                                                        );
                                                        c.lwc1(
                                                            1,
                                                            (c.g[T2][0] as u32).wrapping_add(0x18),
                                                        );
                                                        c.swc1(
                                                            1,
                                                            (c.g[SP][0] as u32).wrapping_add(0x64),
                                                        );
                                                        c.lwc1(
                                                            0,
                                                            (c.g[T2][0] as u32).wrapping_add(0x14),
                                                        );
                                                        c.swc1(
                                                            0,
                                                            (c.g[SP][0] as u32).wrapping_add(0x90),
                                                        );
                                                        c.lwc1(
                                                            1,
                                                            (c.g[T2][0] as u32).wrapping_add(0x18),
                                                        );
                                                        c.swc1(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.swc1(
                                                            21,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd4),
                                                        );
                                                        c.swc1(
                                                            23,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd8),
                                                        );
                                                        c.swc1(
                                                            20,
                                                            (c.g[SP][0] as u32).wrapping_add(0xdc),
                                                        );
                                                        c.swc1(
                                                            1,
                                                            (c.g[SP][0] as u32).wrapping_add(0x94),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.swc1(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.lq(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.swc1(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.swc1(
                                                            22,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd4),
                                                        );
                                                        c.sq(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x50),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x80),
                                                        );
                                                        c.swc1(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.g[T0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xb0),
                                                        );
                                                        c.g[A0][0] = c
                                                            .lw((c.g[T0][0] as u32)
                                                                .wrapping_add(0xe84));
                                                        c.g[V0][0] = c
                                                            .lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x8));
                                                        c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                                                        c.g[V0][0] = c.g[V0][0] | 0x120;
                                                        c.sw(
                                                            (c.g[A0][0] as u32).wrapping_add(0x8),
                                                            c.g[V0][0],
                                                        );
                                                        c.g[V0][0] = c
                                                            .lw((c.g[T0][0] as u32)
                                                                .wrapping_add(0xe84));
                                                        c.g[V1][0] = c.lw((c.g[T2][0] as u32));
                                                        c.sh(
                                                            (c.g[V0][0] as u32).wrapping_add(0x10),
                                                            c.g[V1][0],
                                                        );
                                                        c.g[T0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                        c.g[V0][0] = c.lw((c.g[T0][0] as u32)
                                                            .wrapping_add(0x10d8));
                                                        c.g[A0][0] = c
                                                            .lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x238));
                                                        c.g[V1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x23c));
                                                        let t = c.g[V1][0] as u32;
                                                        c.g[A0][0] = sx((c.g[T0][0] as u32)
                                                            .wrapping_add((c.g[A0][0] as u32)));
                                                        call_ptr(c, t, 0x0020f524)?;
                                                        c.lwc1(
                                                            2,
                                                            (c.g[S1][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.sub_s(1, 22, 21);
                                                        c.lwc1(
                                                            0,
                                                            (c.g[GP][0] as u32)
                                                                .wrapping_sub(0x54d0),
                                                        );
                                                        c.g[A2][0] = 0x80;
                                                        c.swc1(
                                                            21,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd4),
                                                        );
                                                        c.g[A1][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0xc0));
                                                        c.sub_s(3, 2, 0);
                                                        c.swc1(
                                                            23,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd8),
                                                        );
                                                        c.swc1(
                                                            20,
                                                            (c.g[SP][0] as u32).wrapping_add(0xdc),
                                                        );
                                                        c.add_s(2, 2, 0);
                                                        c.mul_s(1, 1, 26);
                                                        c.g[A0][0] = c.g[SP][0];
                                                        c.swc1(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.lq(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.add_s(1, 21, 1);
                                                        c.swc1(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.sq(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.swc1(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.swc1(
                                                            1,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd4),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x50),
                                                        );
                                                        c.lq(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.swc1(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.sq(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x80),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xb0),
                                                        );
                                                        c.sw(
                                                            (c.g[A0][0] as u32).wrapping_add(0x10),
                                                            c.g[A2][0],
                                                        );
                                                        'l_20f590: loop {
                                                            c.cycles += 7;
                                                            c.sw(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x14),
                                                                0u64,
                                                            );
                                                            c.sw(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x18),
                                                                0u64,
                                                            );
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x30));
                                                            c.g[V0][0] = ((c.g[A0][0] as i64)
                                                                < (c.g[A1][0] as i64))
                                                                as u64;
                                                            if c.g[V0][0] != 0u64 {
                                                                c.sw(
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                    c.g[A2][0],
                                                                );
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x0020f590,
                                                                    );
                                                                }
                                                                continue 'l_20f590;
                                                            } else {
                                                                c.cycles += 23;
                                                                c.g[T0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x854));
                                                                c.g[V1][0] = 0xfffffffffffffc1f;
                                                                c.g[T1][0] = 0xffffffffffffffff;
                                                                c.g[A1][0] = 0x4;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[T0][0] as u32)
                                                                        .wrapping_add(0xe84));
                                                                c.g[A2][0] = c.g[SP][0];
                                                                c.g[A3][0] = 0u64;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[A0][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                c.g[V0][0] =
                                                                    c.g[V0][0] & c.g[V1][0];
                                                                c.g[V0][0] = c.g[V0][0] | 0x140;
                                                                c.sw(
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x8),
                                                                    c.g[V0][0],
                                                                );
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[T0][0] as u32)
                                                                        .wrapping_add(0xe84));
                                                                c.sh(
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                    c.g[T1][0],
                                                                );
                                                                c.g[T0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x854));
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[T0][0] as u32)
                                                                        .wrapping_add(0x10d8));
                                                                c.g[A0][0] = c
                                                                    .lh((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x238));
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x23c));
                                                                let t = c.g[V1][0] as u32;
                                                                c.g[A0][0] = sx((c.g[T0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[A0][0] as u32),
                                                                    ));
                                                                call_ptr(c, t, 0x0020f5f8)?;
                                                                c.g[A0][0] = 0x80;
                                                                c.g[A1][0] = sx((c.g[SP][0]
                                                                    as u32)
                                                                    .wrapping_add(0xc0));
                                                                c.g[V1][0] = c.g[SP][0];
                                                                c.sw(
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                    c.g[A0][0],
                                                                );
                                                                'l_20f608: loop {
                                                                    c.cycles += 7;
                                                                    c.sw(
                                                                        (c.g[V1][0] as u32)
                                                                            .wrapping_add(0x14),
                                                                        c.g[A0][0],
                                                                    );
                                                                    c.sw(
                                                                        (c.g[V1][0] as u32)
                                                                            .wrapping_add(0x18),
                                                                        c.g[A0][0],
                                                                    );
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x30));
                                                                    c.g[V0][0] = ((c.g[V1][0]
                                                                        as i64)
                                                                        < (c.g[A1][0] as i64))
                                                                        as u64;
                                                                    if c.g[V0][0] != 0u64 {
                                                                        c.sw(
                                                                            (c.g[V1][0] as u32)
                                                                                .wrapping_add(0x10),
                                                                            c.g[A0][0],
                                                                        );
                                                                        if event_due(c) {
                                                                            deliver_event_here(
                                                                                c, 0x0020f608,
                                                                            );
                                                                        }
                                                                        continue 'l_20f608;
                                                                    } else {
                                                                        'b_20f8a4: {
                                                                            c.cycles += 8;
                                                                            c.lwc1(
                                                                                1,
                                                                                (c.g[S1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x1c,
                                                                                    ),
                                                                            );
                                                                            c.g[S0][0] = 0u64;
                                                                            c.g[AT][0] = 0x41a00000;
                                                                            c.mtc1(1, 0);
                                                                            c.div_s(0, 0, 1);
                                                                            c.add_s(24, 24, 0);
                                                                            let t = (c.g[S3][0]
                                                                                as i64)
                                                                                <= 0;
                                                                            c.sub_s(23, 25, 0);
                                                                            if t {
                                                                                break 'b_20f8a4;
                                                                            } else {
                                                                                c.cycles += 9;
                                                                                c.g[V0][0] =
                                                                                    0x4d0000;
                                                                                c.g[AT][0] =
                                                                                    0x3f000000;
                                                                                c.mtc1(1, 26);
                                                                                c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7438));
                                                                                c.g[AT][0] =
                                                                                    0x3f800000;
                                                                                c.mtc1(1, 22);
                                                                                c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x10));
                                                                                c.g[V0][0] = 0x18;
                                                                                'l_20f668: loop {
                                                                                    'b_20f898: {
                                                                                        c.cycles +=
                                                                                            7;
                                                                                        c.mult(
                                                                                            0, 4,
                                                                                            16, 2,
                                                                                        );
                                                                                        c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                                        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                                                                                        c.c_lt_s(
                                                                                            0, 23,
                                                                                        );
                                                                                        if c.fcc() {
                                                                                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                                                            break 'b_20f898;
                                                                                        } else {
                                                                                            c.cycles += 4;
                                                                                            c.c_lt_s(24, 0);
                                                                                            if c.fcc() {
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
break 'b_20f898;
} else {
'b_20f718: {
c.cycles += 10;
c.sub_s(0, 0, 25);
c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x1c));
c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x18));
c.g[T1][0] = 0xc;
c.lwc1(20, (c.g[S1][0] as u32).wrapping_add(0x10));
c.g[V0][0] = 0x1;
c.mul_s(0, 0, 2);
c.mov_s(4, 1);
let t = c.g[S6][0] == c.g[S0][0];
c.sub_s(21, 1, 0);
if t {
break 'b_20f718;
} else {
'b_20f70c: {
c.cycles += 17;
c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe4));
c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x54cc));
c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.lwc1(0, (c.g[V1][0] as u32));
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
c.mul_s(0, 0, 1);
c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
c.g[V1][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x40));
c.sub_s(20, 20, 0);
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x44));
let t = c.g[V0][0] as u32;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
call_ptr(c, t, 0x0020f6f8)?;
let t = c.g[V0][0] == 0u64;
c.g[T1][0] = 0xd;
if t {
break 'b_20f70c;
} else {
c.cycles += 3;
c.g[V0][0] = 0x3;
let t = true;
c.lwc1(4, (c.g[S1][0] as u32).wrapping_add(0x18));
if t {
break 'b_20f718;
} else {
break 'b_20f70c;
}
}
}
c.cycles += 3;
c.lwc1(4, (c.g[S1][0] as u32).wrapping_add(0x18));
c.g[T1][0] = 0xe;
c.g[V0][0] = 0x2;
break 'b_20f718;
}
}
'b_20f894: {
c.cycles += 12;
c.g[T0][0] = 0x4d0000;
c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x73a8));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
c.g[T3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
c.mul_s(2, 0, 26);
c.sub_s(6, 21, 2);
c.c_le_s(4, 6);
let t = c.fcc();
c.add_s(7, 21, 2);
if t {
break 'b_20f894;
} else {
c.cycles += 5;
c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x14));
c.c_le_s(7, 1);
if c.fcc() {
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
break 'b_20f898;
} else {
'b_20f7bc: {
'b_20f7b8: {
'b_20f78c: {
c.cycles += 4;
c.c_lt_s(6, 1);
c.lwc1(3, (c.g[T3][0] as u32).wrapping_add(0xc));
let t = !c.fcc();
c.lwc1(5, (c.g[T3][0] as u32).wrapping_add(0x18));
if t {
break 'b_20f78c;
} else {
c.cycles += 8;
c.mov_s(6, 1);
c.sub_s(0, 3, 5);
c.sub_s(1, 7, 1);
c.add_s(2, 2, 2);
c.mul_s(0, 0, 1);
c.div_s(0, 0, 2);
let t = true;
c.add_s(3, 5, 0);
if t {
break 'b_20f7b8;
} else {
break 'b_20f78c;
}
}
}
c.cycles += 4;
c.c_lt_s(4, 7);
if !c.fcc() {
c.lwc1(0, (c.g[T3][0] as u32).wrapping_add(0x10));
break 'b_20f7bc;
} else {
c.cycles += 7;
c.sub_s(0, 5, 3);
c.mov_s(7, 4);
c.sub_s(1, 4, 6);
c.add_s(2, 2, 2);
c.mul_s(0, 0, 1);
c.div_s(0, 0, 2);
c.add_s(5, 3, 0);
break 'b_20f7b8;
}
}
c.cycles += 1;
c.lwc1(0, (c.g[T3][0] as u32).wrapping_add(0x10));
break 'b_20f7bc;
}
c.cycles += 54;
c.g[T2][0] = 0xfffffffffffffc1f;
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x4));
c.g[T1][0] = sx((c.g[T1][0] as u32) << 5);
c.swc1(0, (c.g[SP][0] as u32));
c.g[A1][0] = 0x4;
c.g[A2][0] = c.g[SP][0];
c.g[A3][0] = 0u64;
c.lwc1(0, (c.g[T3][0] as u32).wrapping_add(0x14));
c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x4));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x34));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
c.mul_s(2, 2, 26);
c.lwc1(1, (c.g[T3][0] as u32).wrapping_add(0x10));
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x64));
c.sub_s(3, 20, 2);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x60));
c.add_s(2, 20, 2);
c.lwc1(0, (c.g[T3][0] as u32).wrapping_add(0x14));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.sw((c.g[SP][0] as u32).wrapping_add(0xd8), 0u64);
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0xd4));
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xdc));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x90));
c.lq(4, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.sw((c.g[SP][0] as u32).wrapping_add(0xd8), 0u64);
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x94));
c.lq(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0xd4));
c.sw((c.g[SP][0] as u32).wrapping_add(0xd8), 0u64);
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sw((c.g[SP][0] as u32).wrapping_add(0xd8), 0u64);
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
c.g[V1][0] = c.lw((c.g[T3][0] as u32));
c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x238));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x23c));
let t = c.g[V1][0] as u32;
c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0020f894)?;
break 'b_20f894;
}
}
}
c.cycles += 1;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
break 'b_20f898;
}
                                                                                        }
                                                                                    }
                                                                                    c.cycles += 3;
                                                                                    c.g[V0][0] = ((c
                                                                                        .g[S0][0]
                                                                                        as i64)
                                                                                        < (c.g[S3]
                                                                                            [0]
                                                                                            as i64))
                                                                                        as u64;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        != 0u64;
                                                                                    c.g[V0][0] =
                                                                                        0x18;
                                                                                    if t {
                                                                                        if event_due(
                                                                                            c,
                                                                                        ) {
                                                                                            deliver_event_here(c, 0x0020f668);
                                                                                        }
                                                                                        continue 'l_20f668;
                                                                                    } else {
                                                                                        break 'b_20f8a4;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        'b_20facc: {
                                                                            c.cycles += 3;
                                                                            c.g[A1][0] = c.lw((c.g
                                                                                [S7][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                            let t =
                                                                                c.g[A1][0] == 0u64;
                                                                            c.g[V1][0] = 0x530000;
                                                                            if t {
                                                                                break 'b_20facc;
                                                                            } else {
                                                                                'b_20f8d4: {
                                                                                    c.cycles += 5;
                                                                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8d0));
                                                                                    c.g[V0][0] = sx(
                                                                                        ((c.g[V0][0]
                                                                                            as u32)
                                                                                            as i32
                                                                                            >> 9)
                                                                                            as u32,
                                                                                    );
                                                                                    c.g[V0][0] = c
                                                                                        .g[V0][0]
                                                                                        & 0x1;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        == 0u64;
                                                                                    c.g[S3][0] =
                                                                                        0x1;
                                                                                    if t {
                                                                                        break 'b_20f8d4;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            4;
                                                                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
                                                                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x88));
                                                                                        if c.g[A0]
                                                                                            [0]
                                                                                            != 0
                                                                                        {
                                                                                            c.g[S3][0] = 0u64;
                                                                                        }
                                                                                        break 'b_20f8d4;
                                                                                    }
                                                                                }
                                                                                c.cycles += 2;
                                                                                let t = (c.g[A1][0]
                                                                                    as i64)
                                                                                    <= 0;
                                                                                c.g[S2][0] = 0u64;
                                                                                if t {
                                                                                    break 'b_20facc;
                                                                                } else {
                                                                                    c.cycles += 9;
                                                                                    c.g[V0][0] =
                                                                                        0x4d0000;
                                                                                    c.g[AT][0] =
                                                                                        0x3f000000;
                                                                                    c.mtc1(1, 21);
                                                                                    c.g[AT][0] =
                                                                                        0x3f800000;
                                                                                    c.mtc1(1, 20);
                                                                                    c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7358));
                                                                                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe0));
                                                                                    c.g[S4][0] = 0xfffffffffffffc1f;
                                                                                    'l_20f900: loop {
                                                                                        'b_20fabc: {
                                                                                            c.cycles += 5;
                                                                                            c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x4));
                                                                                            c.c_lt_s(1, 23);
                                                                                            if c.fcc() {
c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x8));
break 'b_20fabc;
} else {
c.cycles += 4;
c.c_lt_s(24, 1);
if c.fcc() {
c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x8));
break 'b_20fabc;
} else {
'b_20f940: {
c.cycles += 4;
c.g[V1][0] = c.lw((c.g[S0][0] as u32));
c.g[V0][0] = 0x1;
if c.g[V1][0] != c.g[V0][0] {
c.sub_s(1, 1, 25);
break 'b_20f940;
} else {
c.cycles += 2;
if c.g[S3][0] == 0u64 {
c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x8));
break 'b_20fabc;
} else {
c.cycles += 1;
c.sub_s(1, 1, 25);
break 'b_20f940;
}
}
}
'b_20fab8: {
c.cycles += 16;
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x1c));
c.g[V0][0] = sx((c.g[V1][0] as u32) << 4);
c.lwc1(4, (c.g[S1][0] as u32).wrapping_add(0x18));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S5][0] as u32)));
c.mul_s(1, 1, 0);
c.lwc1(7, (c.g[V0][0] as u32).wrapping_add(0x8));
c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
c.mul_s(2, 7, 21);
c.g[T1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
c.sub_s(1, 4, 1);
c.mul_s(9, 0, 21);
c.sub_s(6, 1, 2);
c.c_le_s(4, 6);
let t = c.fcc();
c.add_s(8, 1, 2);
if t {
break 'b_20fab8;
} else {
c.cycles += 5;
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x14));
c.c_le_s(8, 0);
if c.fcc() {
c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x8));
break 'b_20fabc;
} else {
'b_20f9ec: {
'b_20f9e8: {
'b_20f9c0: {
c.cycles += 4;
c.c_lt_s(6, 0);
c.lwc1(3, (c.g[T1][0] as u32).wrapping_add(0xc));
let t = !c.fcc();
c.lwc1(5, (c.g[T1][0] as u32).wrapping_add(0x18));
if t {
break 'b_20f9c0;
} else {
c.cycles += 7;
c.mov_s(6, 0);
c.sub_s(1, 3, 5);
c.sub_s(0, 8, 0);
c.mul_s(1, 1, 0);
c.div_s(1, 1, 7);
let t = true;
c.add_s(3, 5, 1);
if t {
break 'b_20f9e8;
} else {
break 'b_20f9c0;
}
}
}
c.cycles += 4;
c.c_lt_s(4, 8);
if !c.fcc() {
c.lwc1(0, (c.g[T1][0] as u32).wrapping_add(0x10));
break 'b_20f9ec;
} else {
c.cycles += 6;
c.sub_s(1, 5, 3);
c.mov_s(8, 4);
c.sub_s(0, 4, 6);
c.mul_s(1, 1, 0);
c.div_s(1, 1, 7);
c.add_s(5, 3, 1);
break 'b_20f9e8;
}
}
c.cycles += 1;
c.lwc1(0, (c.g[T1][0] as u32).wrapping_add(0x10));
break 'b_20f9ec;
}
c.cycles += 51;
c.g[A1][0] = 0x4;
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A2][0] = c.g[SP][0];
c.swc1(0, (c.g[SP][0] as u32));
c.g[A3][0] = 0u64;
c.lwc1(0, (c.g[T1][0] as u32).wrapping_add(0x14));
c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x10));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x34));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sub_s(3, 2, 9);
c.add_s(2, 2, 9);
c.lwc1(0, (c.g[T1][0] as u32).wrapping_add(0x10));
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x64));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lwc1(1, (c.g[T1][0] as u32).wrapping_add(0x14));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.sw((c.g[SP][0] as u32).wrapping_add(0xd8), 0u64);
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0xd4));
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xdc));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x90));
c.lq(4, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.sw((c.g[SP][0] as u32).wrapping_add(0xd8), 0u64);
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x94));
c.lq(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0xd4));
c.sw((c.g[SP][0] as u32).wrapping_add(0xd8), 0u64);
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sw((c.g[SP][0] as u32).wrapping_add(0xd8), 0u64);
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.g[V0][0] & c.g[S4][0];
c.g[V0][0] = c.g[V0][0] | 0x160;
c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
c.g[V1][0] = c.lw((c.g[T1][0] as u32));
c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x238));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x23c));
let t = c.g[V1][0] as u32;
c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0020fab8)?;
break 'b_20fab8;
}
}
}
c.cycles += 1;
c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x8));
break 'b_20fabc;
}
}
                                                                                        }
                                                                                        c.cycles +=
                                                                                            4;
                                                                                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                                                                        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                                                                        let t = c.g
                                                                                            [V0][0]
                                                                                            != 0u64;
                                                                                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                                                                                        if t {
                                                                                            if event_due(c) { deliver_event_here(c, 0x0020f900); }
                                                                                            continue 'l_20f900;
                                                                                        } else {
                                                                                            break 'b_20facc;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 10;
                                                                        c.g[A0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_sub(0x854));
                                                                        c.g[V0][0] = c.lw((c.g[A0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0xe84));
                                                                        c.g[A1][0] = c.lw((c.g[A0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10d8));
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_sub(
                                                                                    0x14,
                                                                                ));
                                                                        c.sw(
                                                                            (c.g[A0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0xe84,
                                                                                ),
                                                                            c.g[V0][0],
                                                                        );
                                                                        c.g[V1][0] = c.lh((c.g[A1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x100));
                                                                        c.g[V0][0] = c.lw((c.g[A1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x104));
                                                                        let t = c.g[V0][0] as u32;
                                                                        c.g[A0][0] =
                                                                            sx((c.g[A0][0] as u32)
                                                                                .wrapping_add(
                                                                                    (c.g[V1][0]
                                                                                        as u32),
                                                                                ));
                                                                        call_ptr(c, t, 0x0020faf0)?;
                                                                        c.sw(
                                                                            (c.g[S1][0] as u32)
                                                                                .wrapping_add(0x34),
                                                                            c.g[S6][0],
                                                                        );
                                                                        break 'b_20faf4;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
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
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
            break 'b_20faf8;
        }
        c.cycles += 18;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x150));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lwc1(26, (c.g[SP][0] as u32).wrapping_add(0x1c0));
        c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x1b8));
        c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x1b0));
        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x1a8));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x1a0));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x198));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x190));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1d0));
        c.pc = t;
        return Ok(());
    })
}

/// `void cAngerFX::Render()` (0x2d5048, `src/visualfx/angerfx.cpp`, asm
/// only in the reference): the red "anger" glow drawn over a rider.
///
/// `self+0` is the rider, `self+4` the per-racer effect records (0x2c
/// bytes: +0 intensity, +0x4..+0x10 colour, +0x14/+0x18 size), indexed by
/// the race's current slot (world +0x84 -> +0x84 -> +0x14). Nothing is
/// drawn when the rider's vtable +0x40 query says so, when the intensity
/// is 0, or when the rider's bone (`rider+0x780` skeleton, bone
/// `rider+0x8a8`) lies behind the camera or 2000 or more units away in view
/// space (renderer vtable +0x118 gives the view matrix). Past 1500 units
/// the glow fades (`.lit4` 0.002 per unit). The quad is built from the
/// view matrix's right/up axes scaled by the size and distance (`.lit4`
/// 0.003), its colour converted to 0..255 (alpha 0..128), written as one
/// 4-vertex sprite with a GS state packet pushed on the renderer's packet
/// stack (`gp-0x854` +0xe84) and drawn through renderer vtable +0x238.
///
/// The body keeps the original's block structure: every cycle charge, the
/// VU0 macro ops (`vmula`/`vmadda`/`vmadd`, `vmul`/`vadd`/`vsub`) on the
/// stack temporaries, and the scratch registers each block leaves.
pub fn c_anger_fx_render(c: &mut Cpu) {
    guest(c, |c| {
        'b_2d5584: {
            'b_2d5580: {
                c.cycles += 16;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x1a0));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
                c.g[S0][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x160), c.g[RA][0]);
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x198));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x190));
                c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                call_ptr(c, t, 0x002d5080)?;
                if c.g[V0][0] != 0u64 {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
                    break 'b_2d5580;
                } else {
                    c.cycles += 16;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = 0x2c;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                    c.mtc1(0, 20);
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 21);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    c.mult(0, 2, 2, 4);
                    c.g[S1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.lwc1(0, (c.g[S1][0] as u32));
                    c.c_eq_s(0, 20);
                    if c.fcc() {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
                        break 'b_2d5580;
                    } else {
                        c.cycles += 32;
                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x118));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x11c));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x002d50e0)?;
                        c.g[T4][0] = c.g[V0][0];
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x780));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8a8));
                        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x30));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.lq(2, (c.g[A0][0] as u32).wrapping_add(0x30));
                        c.sq(2, (c.g[SP][0] as u32));
                        c.lqc2(8, (c.g[SP][0] as u32));
                        c.lqc2(4, (c.g[T4][0] as u32));
                        c.lqc2(5, (c.g[T4][0] as u32).wrapping_add(0x10));
                        c.lqc2(6, (c.g[T4][0] as u32).wrapping_add(0x20));
                        c.lqc2(7, (c.g[T4][0] as u32).wrapping_add(0x30));
                        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                        c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x18));
                        c.c_le_s(4, 20);
                        let t = c.fcc();
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
                        if t {
                            break 'b_2d5580;
                        } else {
                            c.cycles += 6;
                            c.g[AT][0] = 0x44fa0000;
                            c.mtc1(1, 5);
                            c.c_le_s(5, 4);
                            if c.fcc() {
                                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
                                break 'b_2d5584;
                            } else {
                                'b_2d5190: {
                                    c.cycles += 9;
                                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3d20));
                                    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x3d1c));
                                    c.mul_s(1, 4, 1);
                                    c.lwc1(3, (c.g[S1][0] as u32).wrapping_add(0x14));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x18));
                                    c.c_lt_s(2, 4);
                                    c.add_s(6, 0, 1);
                                    let t = !c.fcc();
                                    c.add_s(7, 3, 1);
                                    if t {
                                        break 'b_2d5190;
                                    } else {
                                        c.cycles += 3;
                                        c.sub_s(1, 5, 4);
                                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3d18));
                                        c.mul_s(21, 1, 0);
                                        break 'b_2d5190;
                                    }
                                }
                                c.cycles += 252;
                                c.lwc1(0, (c.g[T4][0] as u32));
                                c.g[T1][0] = 0x500000;
                                c.lwc1(1, (c.g[T4][0] as u32).wrapping_add(0x10));
                                c.g[A2][0] = 0xffffffffffbf0000;
                                c.lwc1(2, (c.g[T4][0] as u32).wrapping_add(0x20));
                                c.g[A2][0] = c.g[A2][0] | 0xffff;
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                                c.g[T0][0] = 0x400000;
                                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x28));
                                c.g[T2][0] = 0xfffffffffffffffc;
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x24));
                                c.g[A3][0] = 0xffffffffe0000000;
                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x2c));
                                c.g[A3][0] = c.g[A3][0] | 0x3ff;
                                c.g[AT][0] = 0x420c0000;
                                c.mtc1(1, 2);
                                c.g[T3][0] = 0xfffffffffffffc1f;
                                c.lwc1(1, (c.g[T4][0] as u32).wrapping_add(0x4));
                                c.g[T5][0] = 0x3;
                                c.lwc1(0, (c.g[T4][0] as u32).wrapping_add(0x14));
                                c.g[T6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                                c.lwc1(3, (c.g[T4][0] as u32).wrapping_add(0x24));
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x30));
                                c.g[T4][0] = 0xffffffffffffffff;
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x34));
                                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x38));
                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x3c));
                                c.mfc1(2, 2);
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                                c.qmtc2(2, 3);
                                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
                                c.g[AT][0] = 0x41900000;
                                c.mtc1(1, 0);
                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                                c.mul_s(0, 6, 0);
                                c.g[AT][0] = 0x41100000;
                                c.mtc1(1, 1);
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                                c.lqc2(3, (c.g[SP][0] as u32));
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
                                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
                                c.mul_s(1, 7, 1);
                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                                c.mfc1(3, 0);
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                                c.qmtc2(3, 3);
                                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
                                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                                c.mfc1(4, 1);
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                                c.qmtc2(4, 3);
                                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                                c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xe84));
                                c.ldl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
                                c.ldr(3, (c.g[V0][0] as u32));
                                c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0xf));
                                c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x8));
                                c.g[T7][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x1b));
                                c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x14));
                                c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x23));
                                c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x1c));
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x24), c.g[T7][0]);
                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xe84));
                                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x14));
                                c.sw((c.g[A0][0] as u32).wrapping_add(0xe84), c.g[V1][0]);
                                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
                                c.g[T7][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1420));
                                c.ldl(3, (c.g[T7][0] as u32).wrapping_add(0x7));
                                c.ldr(3, (c.g[T7][0] as u32));
                                c.ldl(4, (c.g[T7][0] as u32).wrapping_add(0xf));
                                c.ldr(4, (c.g[T7][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = c.lw((c.g[T7][0] as u32).wrapping_add(0x10));
                                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
                                c.sdr(3, (c.g[V0][0] as u32));
                                c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0xf));
                                c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                                c.g[V0][0] = c.g[V0][0] | c.g[T0][0];
                                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
                                c.g[V0][0] = c.g[V0][0] | 0x2;
                                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                                c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
                                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                c.g[V0][0] = c.g[V0][0] & c.g[T3][0];
                                c.g[V0][0] = c.g[V0][0] | 0xe0;
                                c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
                                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xfac));
                                c.sh((c.g[V1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                                c.ldl(16, (c.g[S1][0] as u32).wrapping_add(0xb));
                                c.ldr(16, (c.g[S1][0] as u32).wrapping_add(0x4));
                                c.ldl(2, (c.g[S1][0] as u32).wrapping_add(0x13));
                                c.ldr(2, (c.g[S1][0] as u32).wrapping_add(0xc));
                                c.sdl(16, (c.g[SP][0] as u32).wrapping_add(0x77));
                                c.sdr(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7f));
                                c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x78));
                                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x70));
                                c.mul_s(0, 0, 21);
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x70));
                                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
                                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x150));
                                c.g[AT][0] = 0x437f0000;
                                c.mtc1(1, 4);
                                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x74));
                                c.g[A2][0] = c.g[T6][0];
                                c.g[AT][0] = 0x43000000;
                                c.mtc1(1, 6);
                                c.g[A1][0] = 0x4;
                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x78));
                                c.mul_s(0, 0, 4);
                                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x7c));
                                c.g[A3][0] = 0u64;
                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                                c.mul_s(1, 1, 4);
                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x150));
                                c.mul_s(2, 2, 4);
                                c.mul_s(3, 3, 6);
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 5);
                                c.cvt_w_s(7, 0);
                                c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x90));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x140));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                c.cvt_w_s(0, 1);
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x94));
                                c.cvt_w_s(0, 2);
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x98));
                                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x88));
                                c.cvt_w_s(0, 3);
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x9c));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x80), 0u64);
                                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x84));
                                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
                                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x150));
                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x74));
                                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x78));
                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x7c));
                                c.mul_s(1, 1, 4);
                                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x70));
                                c.mul_s(2, 2, 4);
                                c.mul_s(3, 3, 4);
                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x150));
                                c.mul_s(0, 0, 6);
                                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xb8));
                                c.cvt_w_s(7, 1);
                                c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x140));
                                c.cvt_w_s(1, 2);
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xc4));
                                c.cvt_w_s(1, 3);
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xc8));
                                c.cvt_w_s(1, 0);
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xcc));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
                                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xb4));
                                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
                                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x150));
                                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x74));
                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x78));
                                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x7c));
                                c.mul_s(0, 0, 4);
                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                                c.mul_s(1, 1, 4);
                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x150));
                                c.mul_s(2, 2, 4);
                                c.mul_s(3, 3, 6);
                                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xe8));
                                c.cvt_w_s(7, 0);
                                c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0xf0));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x140));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                                c.cvt_w_s(0, 1);
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xf4));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0xe0), 0u64);
                                c.cvt_w_s(0, 2);
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xf8));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0xe4), 0u64);
                                c.cvt_w_s(0, 3);
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xfc));
                                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
                                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x150));
                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x74));
                                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x78));
                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x7c));
                                c.mul_s(1, 1, 4);
                                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x70));
                                c.mul_s(2, 2, 4);
                                c.mul_s(3, 3, 4);
                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x150));
                                c.mul_s(0, 0, 6);
                                c.cvt_w_s(4, 1);
                                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x120));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x140));
                                c.cvt_w_s(1, 2);
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x124));
                                c.cvt_w_s(1, 3);
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x128));
                                c.cvt_w_s(1, 0);
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x12c));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x118));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x114), 0u64);
                                c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x238));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x23c));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[T0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x002d556c)?;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xe84));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
                                c.sw((c.g[A0][0] as u32).wrapping_add(0xe84), c.g[V0][0]);
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
                                break 'b_2d5580;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
            break 'b_2d5584;
        }
        c.cycles += 5;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x160));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x198));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x190));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1a0));
        c.pc = t;
        return Ok(());
    })
}

/// `int Collide_VolumeObject(Query* q, WorldNode* node, Contact* out,
/// int cap, int mode)` (0x334888, `src/intersect/worldsphtree.cpp`, asm
/// only in the reference): collides a query volume with one world object
/// and appends the contacts (0x80 bytes each) to `out`; returns how many.
///
/// An object instance with its own collision (`node+0xc`, vtable +0x130)
/// handles the query itself (vtable +0x138). Otherwise only collision
/// types 1..3 (`*node+0x88`) collide. The instance supplies either its
/// bone matrices (vtable +0xd0 / +0xe0) or one matrix (+0xc8; default
/// `node+0x10`). For each collision part (`node+0x80`: +4 count, +8 parts
/// of 0x10 bytes: bone, shape, matrix) the part's matrix is scaled by
/// `node+0x84` and composed with the bone or object matrix (VU0
/// `vmula`/`vmadda`/`vmadd` 4x4 products), loaded into the query
/// (`q+0x50` vtable +0x8), tested (type 2: +0x38; otherwise +0x48 and then
/// the part's sphere tree, type 3 via +0x30, or its mesh leaf through
/// `Collide_MeshLeafTriangles`), the query closed (+0x10), and every
/// contact found is copied out with its node, part, surface (`+0x88`
/// part records, +0xa) and default fields filled in.
///
/// The body keeps the original's block structure (every cycle charge, the
/// 24-matrix stack array, the scratch registers each block leaves), with
/// the calls through the call protocol and the loop back edges' event
/// checks through [`event_due`]/[`deliver_event_here`].
pub fn collide_volume_object(c: &mut Cpu) {
    guest(c, |c| {
        'b_3350f0: {
            'b_334920: {
                c.cycles += 20;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x27d0));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x27b0));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x27a0));
                c.g[S0][0] = c.g[A3][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x2790));
                c.g[S1][0] = c.g[A1][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x2780));
                c.g[S2][0] = c.g[A0][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x2770));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x2760));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x2750));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x2740));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x2730));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x2720), c.g[RA][0]);
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x27c8));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x27c0));
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x26f0), c.g[A2][0]);
                let t = c.g[A1][0] == 0u64;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x26f4), c.g[T0][0]);
                if t {
                    break 'b_334920;
                } else {
                    c.cycles += 7;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x130));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x134));
                    let t = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x003348ec)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S2][0];
                    if t {
                        break 'b_334920;
                    } else {
                        c.cycles += 11;
                        c.g[T1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x26f0));
                        c.g[A3][0] = c.g[S0][0];
                        c.g[V1][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0xc));
                        c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x26f4));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x138));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x13c));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x00334918)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x27b0));
                        if t {
                            break 'b_3350f0;
                        } else {
                            break 'b_334920;
                        }
                    }
                }
            }
            'b_3350ec: {
                'b_334944: {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x88));
                    c.g[V1][0] = 0x3;
                    c.g[FP][0] = c.lw((c.g[V0][0] as u32));
                    let t = c.g[FP][0] == c.g[V1][0];
                    c.g[V0][0] = 0x1;
                    if t {
                        break 'b_334944;
                    } else {
                        c.cycles += 2;
                        let t = c.g[FP][0] == c.g[V0][0];
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_334944;
                        } else {
                            c.cycles += 2;
                            let t = c.g[FP][0] != c.g[V0][0];
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_3350ec;
                            } else {
                                break 'b_334944;
                            }
                        }
                    }
                }
                c.cycles += 5;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = 0x17;
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = 0xffffffffffffffff;
                'l_334958: loop {
                    c.cycles += 7;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    let t = c.g[V1][0] != c.g[A0][0];
                    c.g[S3][0] = c.g[SP][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00334958);
                        }
                        continue 'l_334958;
                    } else {
                        'b_3349d8: {
                            c.cycles += 4;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x26f8), c.g[V0][0]);
                            c.g[V0][0] = 0x1;
                            let t = c.g[S0][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x26fc), c.g[V0][0]);
                            if t {
                                break 'b_3349d8;
                            } else {
                                'b_3349c0: {
                                    c.cycles += 7;
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xd0));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xd4));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x00334998)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A1][0] = c.g[S1][0];
                                    if t {
                                        break 'b_3349c0;
                                    } else {
                                        c.cycles += 8;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x26fc), 0u64);
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xe0));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe4));
                                        let t = c.g[V0][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x003349b8)?;
                                        let t = true;
                                        c.g[S3][0] = c.g[V0][0];
                                        if t {
                                            break 'b_3349d8;
                                        } else {
                                            break 'b_3349c0;
                                        }
                                    }
                                }
                                c.cycles += 6;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc8));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xcc));
                                let t = c.g[V0][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x003349d4)?;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x26f8), c.g[V0][0]);
                                break 'b_3349d8;
                            }
                        }
                        'b_3350e8: {
                            c.cycles += 16;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
                            c.g[S4][0] = c.g[S3][0];
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 12);
                            c.g[S5][0] = 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x2700), c.g[V1][0]);
                            c.g[S6][0] = 0u64;
                            c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x84));
                            c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x608));
                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x600));
                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x604));
                            c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0x60c));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                            let t = (c.g[V0][0] as i64) <= 0;
                            c.g[S7][0] = 0u64;
                            if t {
                                break 'b_3350e8;
                            } else {
                                c.cycles += 10;
                                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x610));
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x6b0));
                                c.g[T6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x650));
                                c.g[AT][0] = 0xffffffffbf800000;
                                c.mtc1(1, 21);
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x2704), c.g[A0][0]);
                                c.mov_s(20, 12);
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x2708), c.g[A1][0]);
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x270c), c.g[T6][0]);
                                'l_334a40: loop {
                                    'b_334ce4: {
                                        'b_334ce0: {
                                            c.cycles += 3;
                                            c.g[T7][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x26fc));
                                            let t = c.g[T7][0] == 0u64;
                                            c.g[V0][0] = 0xffffffffffffffff;
                                            if t {
                                                break 'b_334ce0;
                                            } else {
                                                'b_334ba0: {
                                                    c.cycles += 3;
                                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                                                    let t = c.g[A1][0] == c.g[V0][0];
                                                    c.g[A1][0] = sx((c.g[A1][0] as u32) << 6);
                                                    if t {
                                                        break 'b_334ba0;
                                                    } else {
                                                        c.cycles += 82;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0xc));
                                                        c.lwc1(
                                                            7,
                                                            (c.g[SP][0] as u32).wrapping_add(0x600),
                                                        );
                                                        c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                            .wrapping_add((c.g[SP][0] as u32)));
                                                        c.lq(2, (c.g[A0][0] as u32));
                                                        c.lwc1(
                                                            6,
                                                            (c.g[SP][0] as u32).wrapping_add(0x604),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x690),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x650),
                                                        );
                                                        c.lwc1(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x608),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[A0][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.lwc1(
                                                            4,
                                                            (c.g[SP][0] as u32).wrapping_add(0x60c),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x690),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x660),
                                                        );
                                                        c.lq(
                                                            3,
                                                            (c.g[A0][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.sq(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x690),
                                                        );
                                                        c.sq(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x670),
                                                        );
                                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32)
                                                            .wrapping_add(0x2708));
                                                        c.lq(
                                                            2,
                                                            (c.g[A0][0] as u32).wrapping_add(0x30),
                                                        );
                                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32)
                                                            .wrapping_add(0x270c));
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x6a0),
                                                        );
                                                        c.lwc1(
                                                            1,
                                                            (c.g[SP][0] as u32).wrapping_add(0x6a0),
                                                        );
                                                        c.lwc1(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x6a4),
                                                        );
                                                        c.lwc1(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x6a8),
                                                        );
                                                        c.mul_s(1, 1, 7);
                                                        c.lwc1(
                                                            0,
                                                            (c.g[SP][0] as u32).wrapping_add(0x6ac),
                                                        );
                                                        c.mul_s(2, 2, 6);
                                                        c.mul_s(3, 3, 5);
                                                        c.mul_s(0, 0, 4);
                                                        c.swc1(
                                                            1,
                                                            (c.g[SP][0] as u32).wrapping_add(0x690),
                                                        );
                                                        c.swc1(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x694),
                                                        );
                                                        c.swc1(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x698),
                                                        );
                                                        c.swc1(
                                                            0,
                                                            (c.g[SP][0] as u32).wrapping_add(0x69c),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x690),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x680),
                                                        );
                                                        c.lqc2(4, (c.g[A1][0] as u32));
                                                        c.lqc2(
                                                            5,
                                                            (c.g[A1][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.lqc2(
                                                            6,
                                                            (c.g[A1][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.lqc2(
                                                            7,
                                                            (c.g[A1][0] as u32).wrapping_add(0x30),
                                                        );
                                                        c.lqc2(8, (c.g[A0][0] as u32));
                                                        c.lqc2(
                                                            9,
                                                            (c.g[A0][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.lqc2(
                                                            10,
                                                            (c.g[A0][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.lqc2(
                                                            11,
                                                            (c.g[A0][0] as u32).wrapping_add(0x30),
                                                        );
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
                                                        c.sqc2(12, (c.g[V1][0] as u32));
                                                        c.sqc2(
                                                            13,
                                                            (c.g[V1][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.sqc2(
                                                            14,
                                                            (c.g[V1][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.sqc2(
                                                            15,
                                                            (c.g[V1][0] as u32).wrapping_add(0x30),
                                                        );
                                                        c.g[A1][0] = c.lw((c.g[SP][0] as u32)
                                                            .wrapping_add(0x2704));
                                                        c.lqc2(1, (c.g[V1][0] as u32));
                                                        c.lqc2(
                                                            2,
                                                            (c.g[V1][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.lqc2(
                                                            3,
                                                            (c.g[V1][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.lqc2(
                                                            4,
                                                            (c.g[V1][0] as u32).wrapping_add(0x30),
                                                        );
                                                        c.sqc2(1, (c.g[A1][0] as u32));
                                                        c.sqc2(
                                                            2,
                                                            (c.g[A1][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.sqc2(
                                                            3,
                                                            (c.g[A1][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.sqc2(
                                                            4,
                                                            (c.g[A1][0] as u32).wrapping_add(0x30),
                                                        );
                                                        c.g[V0][0] = sx((c.g[S5][0] as u32) << 6);
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add((c.g[SP][0] as u32)));
                                                        c.lqc2(1, (c.g[A1][0] as u32));
                                                        c.lqc2(
                                                            2,
                                                            (c.g[A1][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.lqc2(
                                                            3,
                                                            (c.g[A1][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.lqc2(
                                                            4,
                                                            (c.g[A1][0] as u32).wrapping_add(0x30),
                                                        );
                                                        c.sqc2(1, (c.g[V0][0] as u32));
                                                        c.sqc2(
                                                            2,
                                                            (c.g[V0][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.sqc2(
                                                            3,
                                                            (c.g[V0][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.sqc2(
                                                            4,
                                                            (c.g[V0][0] as u32).wrapping_add(0x30),
                                                        );
                                                        let t = true;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x4));
                                                        if t {
                                                            break 'b_334ce4;
                                                        } else {
                                                            break 'b_334ba0;
                                                        }
                                                    }
                                                }
                                                c.cycles += 80;
                                                c.g[A0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                c.lwc1(6, (c.g[SP][0] as u32).wrapping_add(0x600));
                                                c.lq(2, (c.g[A0][0] as u32));
                                                c.lwc1(7, (c.g[SP][0] as u32).wrapping_add(0x604));
                                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x690));
                                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x650));
                                                c.lwc1(5, (c.g[SP][0] as u32).wrapping_add(0x608));
                                                c.lq(2, (c.g[A0][0] as u32).wrapping_add(0x10));
                                                c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x60c));
                                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x690));
                                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x660));
                                                c.g[T6][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x2708));
                                                c.lq(3, (c.g[A0][0] as u32).wrapping_add(0x20));
                                                c.g[T7][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x26f8));
                                                c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x690));
                                                c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x670));
                                                c.lq(2, (c.g[A0][0] as u32).wrapping_add(0x30));
                                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x6a0));
                                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x6a0));
                                                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x6a4));
                                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x6a8));
                                                c.mul_s(1, 1, 6);
                                                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x6ac));
                                                c.mul_s(2, 2, 7);
                                                c.mul_s(3, 3, 5);
                                                c.mul_s(0, 0, 4);
                                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x690));
                                                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x694));
                                                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x698));
                                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x69c));
                                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x690));
                                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x680));
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x270c));
                                                c.lqc2(4, (c.g[T7][0] as u32));
                                                c.lqc2(5, (c.g[T7][0] as u32).wrapping_add(0x10));
                                                c.lqc2(6, (c.g[T7][0] as u32).wrapping_add(0x20));
                                                c.lqc2(7, (c.g[T7][0] as u32).wrapping_add(0x30));
                                                c.lqc2(8, (c.g[V0][0] as u32));
                                                c.lqc2(9, (c.g[V0][0] as u32).wrapping_add(0x10));
                                                c.lqc2(10, (c.g[V0][0] as u32).wrapping_add(0x20));
                                                c.lqc2(11, (c.g[V0][0] as u32).wrapping_add(0x30));
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
                                                c.sqc2(12, (c.g[T6][0] as u32));
                                                c.sqc2(13, (c.g[T6][0] as u32).wrapping_add(0x10));
                                                c.sqc2(14, (c.g[T6][0] as u32).wrapping_add(0x20));
                                                c.sqc2(15, (c.g[T6][0] as u32).wrapping_add(0x30));
                                                c.g[V1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x2704));
                                                c.lqc2(1, (c.g[T6][0] as u32));
                                                c.lqc2(2, (c.g[T6][0] as u32).wrapping_add(0x10));
                                                c.lqc2(3, (c.g[T6][0] as u32).wrapping_add(0x20));
                                                c.lqc2(4, (c.g[T6][0] as u32).wrapping_add(0x30));
                                                c.sqc2(1, (c.g[V1][0] as u32));
                                                c.sqc2(2, (c.g[V1][0] as u32).wrapping_add(0x10));
                                                c.sqc2(3, (c.g[V1][0] as u32).wrapping_add(0x20));
                                                c.sqc2(4, (c.g[V1][0] as u32).wrapping_add(0x30));
                                                c.g[V0][0] = sx((c.g[S5][0] as u32) << 6);
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add((c.g[SP][0] as u32)));
                                                c.lqc2(1, (c.g[V1][0] as u32));
                                                c.lqc2(2, (c.g[V1][0] as u32).wrapping_add(0x10));
                                                c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x20));
                                                c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x30));
                                                c.sqc2(1, (c.g[V0][0] as u32));
                                                c.sqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
                                                c.sqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
                                                c.sqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
                                                break 'b_334ce0;
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                        break 'b_334ce4;
                                    }
                                    'b_3350cc: {
                                        c.cycles += 7;
                                        c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                                        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                                        c.g[T6][0] = sx((c.g[S4][0] as u32).wrapping_add(0x40));
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x2714), c.g[A0][0]);
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x2718), c.g[A1][0]);
                                        let t = c.g[V0][0] == 0u64;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x271c), c.g[T6][0]);
                                        if t {
                                            break 'b_3350cc;
                                        } else {
                                            'b_334d74: {
                                                'b_334d70: {
                                                    'b_334d64: {
                                                        'b_334d38: {
                                                            c.cycles += 4;
                                                            c.g[T7][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x26f4));
                                                            c.g[V0][0] = 0x2;
                                                            if c.g[T7][0] != c.g[V0][0] {
                                                                c.g[T6][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x26f4));
                                                                break 'b_334d38;
                                                            } else {
                                                                c.cycles += 8;
                                                                c.g[V1][0] = 0xc;
                                                                c.g[A1][0] = 0xffffffffffffffff;
                                                                c.mult(0, 2, 21, 3);
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x88));
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[V1][0] as u32),
                                                                    ));
                                                                c.g[A0][0] = c
                                                                    .lh((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x1a));
                                                                let t = c.g[A0][0] != c.g[A1][0];
                                                                c.g[V1][0] = sx((c.g[S5][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1));
                                                                if t {
                                                                    break 'b_334d74;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    let t = true;
                                                                    c.g[V1][0] = sx((c.g[S6][0]
                                                                        as u32)
                                                                        .wrapping_add(0x1));
                                                                    if t {
                                                                        break 'b_334d64;
                                                                    } else {
                                                                        break 'b_334d38;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 3;
                                                        c.g[V0][0] = 0x1;
                                                        let t = c.g[T6][0] != c.g[V0][0];
                                                        c.g[V1][0] =
                                                            sx((c.g[S5][0] as u32)
                                                                .wrapping_add(0x1));
                                                        if t {
                                                            break 'b_334d74;
                                                        } else {
                                                            c.cycles += 8;
                                                            c.g[T7][0] = 0xc;
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x88));
                                                            c.mult(0, 2, 21, 15);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add((c.g[V1][0] as u32)));
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x1a));
                                                            c.g[V0][0] = 0xffffffffffffffff;
                                                            let t = c.g[A0][0] == c.g[V0][0];
                                                            c.g[V1][0] = sx((c.g[S6][0] as u32)
                                                                .wrapping_add(0x1));
                                                            if t {
                                                                break 'b_334d70;
                                                            } else {
                                                                break 'b_334d64;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 3;
                                                    c.g[V0][0] = c.g[FP][0] ^ 0x2;
                                                    let t = true;
                                                    if c.g[V0][0] != 0 {
                                                        c.g[S6][0] = c.g[V1][0];
                                                    }
                                                    if t {
                                                        break 'b_3350cc;
                                                    } else {
                                                        break 'b_334d70;
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[V1][0] =
                                                    sx((c.g[S5][0] as u32).wrapping_add(0x1));
                                                break 'b_334d74;
                                            }
                                            'b_334f50: {
                                                'b_334eac: {
                                                    c.cycles += 68;
                                                    c.g[A1][0] =
                                                        sx((c.g[S0][0] as u32).wrapping_add(0x10));
                                                    c.g[T6][0] =
                                                        sx((c.g[S4][0] as u32).wrapping_add(0x40));
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x2714),
                                                        c.g[V1][0],
                                                    );
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x2718),
                                                        c.g[A1][0],
                                                    );
                                                    c.g[V0][0] = 0x3f;
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x271c),
                                                        c.g[T6][0],
                                                    );
                                                    c.g[A0][0] =
                                                        sx((c.g[SP][0] as u32).wrapping_add(0x610));
                                                    c.g[A2][0] =
                                                        sx((c.g[SP][0] as u32).wrapping_add(0x650));
                                                    c.g[V1][0] = 0xffffffffffffffff;
                                                    c.lqc2(1, (c.g[S4][0] as u32));
                                                    c.lqc2(
                                                        2,
                                                        (c.g[S4][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.lqc2(
                                                        3,
                                                        (c.g[S4][0] as u32).wrapping_add(0x20),
                                                    );
                                                    c.lqc2(
                                                        4,
                                                        (c.g[S4][0] as u32).wrapping_add(0x30),
                                                    );
                                                    c.sqc2(1, (c.g[A0][0] as u32));
                                                    c.sqc2(
                                                        2,
                                                        (c.g[A0][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.sqc2(
                                                        3,
                                                        (c.g[A0][0] as u32).wrapping_add(0x20),
                                                    );
                                                    c.sqc2(
                                                        4,
                                                        (c.g[A0][0] as u32).wrapping_add(0x30),
                                                    );
                                                    c.lwc1(
                                                        3,
                                                        (c.g[SP][0] as u32).wrapping_add(0x614),
                                                    );
                                                    c.g[A1][0] = c.g[A0][0];
                                                    c.lwc1(
                                                        2,
                                                        (c.g[SP][0] as u32).wrapping_add(0x620),
                                                    );
                                                    c.g[S3][0] = 0u64;
                                                    c.swc1(
                                                        3,
                                                        (c.g[SP][0] as u32).wrapping_add(0x620),
                                                    );
                                                    c.lwc1(
                                                        3,
                                                        (c.g[SP][0] as u32).wrapping_add(0x618),
                                                    );
                                                    c.lwc1(
                                                        1,
                                                        (c.g[SP][0] as u32).wrapping_add(0x630),
                                                    );
                                                    c.swc1(
                                                        3,
                                                        (c.g[SP][0] as u32).wrapping_add(0x630),
                                                    );
                                                    c.lq(
                                                        2,
                                                        (c.g[SP][0] as u32).wrapping_add(0x640),
                                                    );
                                                    c.lwc1(
                                                        3,
                                                        (c.g[SP][0] as u32).wrapping_add(0x628),
                                                    );
                                                    c.lwc1(
                                                        0,
                                                        (c.g[SP][0] as u32).wrapping_add(0x634),
                                                    );
                                                    c.sq(
                                                        2,
                                                        (c.g[SP][0] as u32).wrapping_add(0x660),
                                                    );
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x2710),
                                                        0u64,
                                                    );
                                                    c.swc1(
                                                        2,
                                                        (c.g[SP][0] as u32).wrapping_add(0x614),
                                                    );
                                                    c.swc1(
                                                        1,
                                                        (c.g[SP][0] as u32).wrapping_add(0x618),
                                                    );
                                                    c.swc1(
                                                        0,
                                                        (c.g[SP][0] as u32).wrapping_add(0x628),
                                                    );
                                                    c.swc1(
                                                        3,
                                                        (c.g[SP][0] as u32).wrapping_add(0x634),
                                                    );
                                                    c.mfc1(3, 21);
                                                    c.lqc2(
                                                        4,
                                                        (c.g[SP][0] as u32).wrapping_add(0x660),
                                                    );
                                                    c.qmtc2(3, 3);
                                                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                                    c.sqc2(
                                                        5,
                                                        (c.g[SP][0] as u32).wrapping_add(0x670),
                                                    );
                                                    c.lq(
                                                        2,
                                                        (c.g[SP][0] as u32).wrapping_add(0x670),
                                                    );
                                                    c.sq(
                                                        2,
                                                        (c.g[SP][0] as u32).wrapping_add(0x650),
                                                    );
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x65c),
                                                        0u64,
                                                    );
                                                    c.lqc2(
                                                        8,
                                                        (c.g[SP][0] as u32).wrapping_add(0x650),
                                                    );
                                                    c.lqc2(4, (c.g[A0][0] as u32));
                                                    c.lqc2(
                                                        5,
                                                        (c.g[A0][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.lqc2(
                                                        6,
                                                        (c.g[A0][0] as u32).wrapping_add(0x20),
                                                    );
                                                    c.lqc2(
                                                        7,
                                                        (c.g[A0][0] as u32).wrapping_add(0x30),
                                                    );
                                                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                                                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                                                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                                                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                                                    c.sqc2(
                                                        12,
                                                        (c.g[A2][0] as u32).wrapping_add(0x20),
                                                    );
                                                    c.lq(
                                                        2,
                                                        (c.g[SP][0] as u32).wrapping_add(0x670),
                                                    );
                                                    c.lwc1(
                                                        12,
                                                        (c.g[S1][0] as u32).wrapping_add(0x84),
                                                    );
                                                    c.sq(
                                                        2,
                                                        (c.g[SP][0] as u32).wrapping_add(0x650),
                                                    );
                                                    c.sq(
                                                        2,
                                                        (c.g[SP][0] as u32).wrapping_add(0x660),
                                                    );
                                                    c.div_s(12, 20, 12);
                                                    c.swc1(
                                                        20,
                                                        (c.g[A2][0] as u32).wrapping_add(0xc),
                                                    );
                                                    c.g[A2][0] = c
                                                        .lw((c.g[S2][0] as u32).wrapping_add(0x50));
                                                    c.lq(
                                                        2,
                                                        (c.g[SP][0] as u32).wrapping_add(0x650),
                                                    );
                                                    c.sq(
                                                        2,
                                                        (c.g[SP][0] as u32).wrapping_add(0x640),
                                                    );
                                                    c.g[A0][0] =
                                                        c.lh((c.g[A2][0] as u32).wrapping_add(0x8));
                                                    c.g[V1][0] =
                                                        c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    call_ptr(c, t, 0x00334e78)?;
                                                    c.g[V0][0] = 0x2;
                                                    let t = c.g[FP][0] != c.g[V0][0];
                                                    c.g[V1][0] = c
                                                        .lw((c.g[S2][0] as u32).wrapping_add(0x50));
                                                    if t {
                                                        break 'b_334eac;
                                                    } else {
                                                        c.cycles += 10;
                                                        c.g[A3][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0x6f0));
                                                        c.g[A1][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x4));
                                                        c.g[T0][0] = 0x40;
                                                        c.g[A0][0] = c
                                                            .lh((c.g[V1][0] as u32)
                                                                .wrapping_add(0x38));
                                                        c.g[V0][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x3c));
                                                        c.g[A2][0] =
                                                            sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0xc));
                                                        let t = c.g[V0][0] as u32;
                                                        c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                            .wrapping_add((c.g[A0][0] as u32)));
                                                        call_ptr(c, t, 0x00334ea4)?;
                                                        let t = true;
                                                        c.g[S3][0] = c.g[V0][0];
                                                        if t {
                                                            break 'b_334f50;
                                                        } else {
                                                            break 'b_334eac;
                                                        }
                                                    }
                                                }
                                                'b_334f4c: {
                                                    c.cycles += 8;
                                                    c.g[A1][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V1][0] as u32).wrapping_add(0x48));
                                                    c.g[V0][0] = c
                                                        .lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                                    c.g[A2][0] =
                                                        sx((c.g[A1][0] as u32).wrapping_add(0xc));
                                                    let t = c.g[V0][0] as u32;
                                                    c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    call_ptr(c, t, 0x00334ec4)?;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x3;
                                                    if t {
                                                        break 'b_334f4c;
                                                    } else {
                                                        'b_334f14: {
                                                            c.cycles += 2;
                                                            let t = c.g[FP][0] != c.g[V0][0];
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_add(0x16c8));
                                                            if t {
                                                                break 'b_334f14;
                                                            } else {
                                                                c.cycles += 16;
                                                                c.g[A1][0] = c.g[S1][0];
                                                                call_ptr(
                                                                    c, 0x003a6cd8, 0x00334edc,
                                                                )?;
                                                                c.g[A0][0] = c.g[V0][0];
                                                                c.g[A1][0] = c.g[S6][0];
                                                                call_ptr(
                                                                    c, 0x003a6ce8, 0x00334ee8,
                                                                )?;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[S2][0] as u32)
                                                                        .wrapping_add(0x50));
                                                                c.g[A2][0] = sx((c.g[SP][0]
                                                                    as u32)
                                                                    .wrapping_add(0x6f0));
                                                                c.sw(
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x2710),
                                                                    c.g[V0][0],
                                                                );
                                                                c.g[A3][0] = 0x40;
                                                                c.g[A1][0] = c.g[V0][0];
                                                                c.g[A0][0] = c
                                                                    .lh((c.g[V1][0] as u32)
                                                                        .wrapping_add(0x30));
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[V1][0] as u32)
                                                                        .wrapping_add(0x34));
                                                                let t = c.g[V0][0] as u32;
                                                                c.g[A0][0] = sx((c.g[S2][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[A0][0] as u32),
                                                                    ));
                                                                call_ptr(c, t, 0x00334f0c)?;
                                                                let t = true;
                                                                c.g[S3][0] = c.g[V0][0];
                                                                if t {
                                                                    break 'b_334f4c;
                                                                } else {
                                                                    break 'b_334f14;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 14;
                                                        c.g[A1][0] = c.g[S1][0];
                                                        call_ptr(c, 0x003a6cc8, 0x00334f1c)?;
                                                        c.g[A0][0] = c.g[V0][0];
                                                        c.g[A1][0] = c.g[S6][0];
                                                        call_ptr(c, 0x003a6d00, 0x00334f28)?;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x4));
                                                        c.g[A1][0] = c.g[V0][0];
                                                        c.g[A0][0] = c.g[S2][0];
                                                        c.g[A3][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0x6f0));
                                                        c.g[A2][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x18));
                                                        c.g[T0][0] = 0x40;
                                                        c.g[A2][0] = c.g[A2][0] & 0x1;
                                                        call_ptr(c, 0x0032c0f8, 0x00334f48)?;
                                                        c.g[S3][0] = c.g[V0][0];
                                                        break 'b_334f4c;
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[S6][0] =
                                                    sx((c.g[S6][0] as u32).wrapping_add(0x1));
                                                break 'b_334f50;
                                            }
                                            c.cycles += 7;
                                            c.g[V0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                                            let t = c.g[V1][0] as u32;
                                            c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x00334f64)?;
                                            let t = (c.g[S3][0] as i64) <= 0;
                                            c.g[A1][0] = 0u64;
                                            if t {
                                                break 'b_3350cc;
                                            } else {
                                                c.cycles += 7;
                                                c.g[T7][0] = 0xc;
                                                c.g[V0][0] = 0x500000;
                                                c.mult(0, 3, 21, 15);
                                                c.g[T3][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_sub(0xee0));
                                                c.g[T5][0] = 0x1;
                                                c.g[T2][0] = 0xffffffffffffffff;
                                                c.g[T4][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x10));
                                                'l_334f88: loop {
                                                    c.cycles += 23;
                                                    c.g[V0][0] = sx((c.g[A1][0] as u32) << 7);
                                                    c.g[T0][0] =
                                                        sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add((c.g[SP][0] as u32)));
                                                    c.g[A0][0] = sx((c.g[S7][0] as u32) << 7);
                                                    c.lqc2(
                                                        8,
                                                        (c.g[V0][0] as u32).wrapping_add(0x700),
                                                    );
                                                    c.lqc2(4, (c.g[S4][0] as u32));
                                                    c.lqc2(
                                                        5,
                                                        (c.g[S4][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.lqc2(
                                                        6,
                                                        (c.g[S4][0] as u32).wrapping_add(0x20),
                                                    );
                                                    c.lqc2(
                                                        7,
                                                        (c.g[S4][0] as u32).wrapping_add(0x30),
                                                    );
                                                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                                                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                                                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                                                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                                                    c.sqc2(
                                                        12,
                                                        (c.g[SP][0] as u32).wrapping_add(0x660),
                                                    );
                                                    c.g[A3][0] = c.g[A0][0];
                                                    c.lq(
                                                        3,
                                                        (c.g[SP][0] as u32).wrapping_add(0x660),
                                                    );
                                                    c.g[A2][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_add(0x770));
                                                    c.g[A1][0] = c
                                                        .lw((c.g[SP][0] as u32)
                                                            .wrapping_add(0x26f0));
                                                    c.g[T1][0] =
                                                        sx((c.g[S7][0] as u32).wrapping_add(0x1));
                                                    c.sq(
                                                        3,
                                                        (c.g[SP][0] as u32).wrapping_add(0x650),
                                                    );
                                                    c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                        .wrapping_add((c.g[A1][0] as u32)));
                                                    c.sq(
                                                        3,
                                                        (c.g[V0][0] as u32).wrapping_add(0x700),
                                                    );
                                                    c.g[A1][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_add(0x6f0));
                                                    'l_334fe4: loop {
                                                        c.cycles += 7;
                                                        c.lq(14, (c.g[A1][0] as u32));
                                                        c.lq(
                                                            15,
                                                            (c.g[A1][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.sq(14, (c.g[A0][0] as u32));
                                                        c.sq(
                                                            15,
                                                            (c.g[A0][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.g[A1][0] =
                                                            sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x20));
                                                        let t = c.g[A1][0] != c.g[A2][0];
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x20));
                                                        if t {
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x00334fe4);
                                                            }
                                                            continue 'l_334fe4;
                                                        } else {
                                                            'b_3350bc: {
                                                                c.cycles += 47;
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x26f0));
                                                                c.lwc1(
                                                                    0,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x600),
                                                                );
                                                                c.g[A0][0] = sx((c.g[A3][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[A1][0] as u32),
                                                                    ));
                                                                c.lwc1(
                                                                    1,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x608),
                                                                );
                                                                c.lwc1(2, (c.g[A0][0] as u32));
                                                                c.lwc1(
                                                                    6,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x60c),
                                                                );
                                                                c.mul_s(2, 2, 0);
                                                                c.lwc1(
                                                                    4,
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x4),
                                                                );
                                                                c.lwc1(
                                                                    0,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x604),
                                                                );
                                                                c.lwc1(
                                                                    5,
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x8),
                                                                );
                                                                c.lwc1(
                                                                    3,
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0xc),
                                                                );
                                                                c.mul_s(4, 4, 0);
                                                                c.mul_s(5, 5, 1);
                                                                c.swc1(
                                                                    2,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x660),
                                                                );
                                                                c.mul_s(3, 3, 6);
                                                                c.swc1(
                                                                    4,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x664),
                                                                );
                                                                c.swc1(
                                                                    5,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x668),
                                                                );
                                                                c.swc1(
                                                                    3,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x66c),
                                                                );
                                                                c.lqc2(
                                                                    8,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x660),
                                                                );
                                                                c.lqc2(4, (c.g[S4][0] as u32));
                                                                c.lqc2(
                                                                    5,
                                                                    (c.g[S4][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                );
                                                                c.lqc2(
                                                                    6,
                                                                    (c.g[S4][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                c.lqc2(
                                                                    7,
                                                                    (c.g[S4][0] as u32)
                                                                        .wrapping_add(0x30),
                                                                );
                                                                c.vu0().mula(
                                                                    0b1111,
                                                                    4,
                                                                    Src::Bc(8, X),
                                                                );
                                                                c.vu0().madda(
                                                                    0b1111,
                                                                    5,
                                                                    Src::Bc(8, Y),
                                                                );
                                                                c.vu0().madda(
                                                                    0b1111,
                                                                    6,
                                                                    Src::Bc(8, Z),
                                                                );
                                                                c.vu0().madd(
                                                                    0b1111,
                                                                    12,
                                                                    7,
                                                                    Src::Bc(8, W),
                                                                );
                                                                c.sqc2(
                                                                    12,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x670),
                                                                );
                                                                c.lq(
                                                                    3,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x670),
                                                                );
                                                                c.sw(
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x44),
                                                                    c.g[T5][0],
                                                                );
                                                                c.sq(3, (c.g[A0][0] as u32));
                                                                c.sw(
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x50),
                                                                    c.g[S1][0],
                                                                );
                                                                c.sw(
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x5c),
                                                                    c.g[S5][0],
                                                                );
                                                                c.sw(
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x4c),
                                                                    c.g[T2][0],
                                                                );
                                                                c.g[T6][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x2710));
                                                                c.sw(
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x48),
                                                                    c.g[T6][0],
                                                                );
                                                                c.sq(
                                                                    3,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x650),
                                                                );
                                                                c.lq(2, (c.g[T3][0] as u32));
                                                                c.sq(
                                                                    2,
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                c.lq(3, (c.g[T3][0] as u32));
                                                                c.sw(
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x54),
                                                                    0u64,
                                                                );
                                                                c.sq(
                                                                    3,
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x30),
                                                                );
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x88));
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[T4][0] as u32),
                                                                    ));
                                                                c.g[V0][0] = c
                                                                    .lh((c.g[V0][0] as u32)
                                                                        .wrapping_add(0xa));
                                                                if c.g[V0][0] != c.g[T2][0] {
                                                                    c.sw(
                                                                        (c.g[A0][0] as u32)
                                                                            .wrapping_add(0x4c),
                                                                        c.g[V0][0],
                                                                    );
                                                                    break 'b_3350bc;
                                                                } else {
                                                                    break 'b_3350bc;
                                                                }
                                                            }
                                                            c.cycles += 4;
                                                            c.g[A1][0] = c.g[T0][0];
                                                            c.g[V0][0] = ((c.g[A1][0] as i64)
                                                                < (c.g[S3][0] as i64))
                                                                as u64;
                                                            let t = c.g[V0][0] != 0u64;
                                                            c.g[S7][0] = c.g[T1][0];
                                                            if t {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x00334f88,
                                                                    );
                                                                }
                                                                continue 'l_334f88;
                                                            } else {
                                                                break 'b_3350cc;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 7;
                                    c.g[T7][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2700));
                                    c.g[S5][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2714));
                                    c.g[V0][0] = c.lw((c.g[T7][0] as u32).wrapping_add(0x4));
                                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2718));
                                    c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[S4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x271c));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00334a40);
                                        }
                                        continue 'l_334a40;
                                    } else {
                                        break 'b_3350e8;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.g[S7][0];
                        break 'b_3350ec;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x27b0));
            break 'b_3350f0;
        }
        c.cycles += 13;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x27a0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x2790));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x2780));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x2770));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x2760));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x2750));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x2740));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x2730));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x2720));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x27c8));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x27c0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x27d0));
        c.pc = t;
        return Ok(());
    })
}

/// EAXA (EA ADPCM) decoder state, `SND_EaxaDecode`'s first argument.
mod eaxa {
    /// Decoded samples still waiting in the block buffer (stored negated
    /// after a partial block).
    pub const PENDING: u32 = 0x4;
    /// Samples left in the stream.
    pub const REMAINING: u32 = 0x8;
    /// Read pointer into the block buffer (floats).
    pub const READ: u32 = 0x10;
    /// One decoded block (28 floats).
    pub const BUF: u32 = 0x24;
    /// The sample count that `SND_EaxaDecodeBlocks` takes (by address).
    pub const COUNT: u32 = 0x94;
    /// Output pointer (floats).
    pub const OUT: u32 = 0xa4;
    /// Samples per EAXA block.
    pub const BLOCK: u64 = 28;
}

const SND_EAXA_DECODE_BLOCKS: Callee = Callee {
    addr: 0x003c_ca08,
    f: ee::s14::SND_EaxaDecodeBlocks,
};

/// `int SND_EaxaDecode(EaxaState* s, float** pout, int n)` (0x3ccf90,
/// `src/ealib/seg_2B4578.c`): decodes up to `n` samples of an EA ADPCM
/// stream to `*pout`; returns how many were written.
///
/// First the samples left over from the last partial block are copied
/// out, then whole 28-sample blocks are decoded straight to the output
/// (`SND_EaxaDecodeBlocks`), and a remainder is decoded as one block into
/// `s+0x24` and its first samples copied; the rest stays pending
/// (`s+0x4` negated, read pointer `s+0x10`). The stream's remaining count
/// (`s+0x8`) goes down by every sample delivered; when it runs out nothing
/// stays pending.
///
/// The state pointer lives in `s0`, the delivered count in `s3` and the
/// counters in `s1`/`s2` as in the original (`SND_EaxaDecodeBlocks` saves
/// them on its stack), and every block's cycle charge and scratch register
/// is reproduced.
pub fn snd_eaxa_decode(c: &mut Cpu) {
    use eaxa::*;
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S3, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        c.g[S3][0] = 0;
        frame.save(c, S2, 0x30);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        let s = r32(c, S0);
        let out = Addr(r32(c, A1)).lw(c);
        c.g[V0][0] = out;
        let remaining = Addr(s + REMAINING).lw(c);
        c.g[V1][0] = remaining;
        Addr(s + OUT).sw(c, out);
        if remaining == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            // m = min(n, remaining) (a1).
            c.cycles += 5;
            c.g[A1][0] = remaining;
            let n_lt = ((c.g[A2][0] as i64) < (remaining as i64)) as u64;
            c.g[V0][0] = n_lt;
            let pending = Addr(s + PENDING).lw(c);
            c.g[V1][0] = pending;
            if n_lt != 0 {
                c.g[A1][0] = c.g[A2][0];
            }
            if pending != 0 {
                // k = min(m, pending) (a0, s1): copy k pending samples.
                c.cycles += 5;
                let m_lt = ((c.g[A1][0] as i64) < (pending as i64)) as u64;
                c.g[V0][0] = m_lt;
                c.g[A0][0] = if m_lt != 0 { c.g[A1][0] } else { pending };
                c.g[S1][0] = c.g[A0][0];
                if (c.g[A0][0] as i64) > 0 {
                    c.cycles += 1;
                    loop {
                        c.cycles += 12;
                        let s = r32(c, S0);
                        let rd = Addr(s + READ).lw(c) as u32;
                        c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
                        let dst = Addr(s + OUT).lw(c) as u32;
                        c.lwc1(0, rd);
                        c.swc1(0, dst);
                        let dst = Addr(s + OUT).lw(c) as u32;
                        let rd = Addr(s + READ).lw(c) as u32;
                        c.g[V0][0] = sx(dst.wrapping_add(4));
                        c.g[V1][0] = sx(rd.wrapping_add(4));
                        Addr(s + OUT).sw(c, c.g[V0][0]);
                        Addr(s + READ).sw(c, c.g[V1][0]);
                        if c.g[S1][0] == 0 {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003c_cff8);
                        }
                    }
                }
                c.cycles += 8;
                let s = r32(c, S0);
                let k = r32(c, A0);
                let remaining = Addr(s + REMAINING).lw(c) as u32;
                c.g[S3][0] = c.g[A0][0];
                let pending = Addr(s + PENDING).lw(c) as u32;
                c.g[A1][0] = sx(r32(c, A1).wrapping_sub(k));
                c.g[V1][0] = sx(remaining.wrapping_sub(k));
                c.g[V0][0] = sx(pending.wrapping_sub(k));
                Addr(s + REMAINING).sw(c, c.g[V1][0]);
                Addr(s + PENDING).sw(c, c.g[V0][0]);
            }
            // Whole blocks: s1 = m / 28 * 28, s2 = the remainder.
            c.cycles += 4;
            c.g[V0][0] = BLOCK;
            c.div(0, A1, V0);
            c.cycles += 5;
            c.g[V1][0] = c.lo[0];
            c.mult(0, S1, V1, V0);
            c.g[S2][0] = sx(r32(c, A1).wrapping_sub(r32(c, S1)));
            let s = r32(c, S0);
            Addr(s + COUNT).sw(c, c.g[S1][0]);
            if (c.g[S1][0] as i64) > 0 {
                c.cycles += 6;
                c.g[A0][0] = sx(s + COUNT);
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(r32(c, S1)));
                SND_EAXA_DECODE_BLOCKS.call(c, 0x003c_d078)?;
                let s = r32(c, S0);
                let remaining = Addr(s + REMAINING).lw(c) as u32;
                c.g[V0][0] = sx(remaining.wrapping_sub(r32(c, S1)));
                Addr(s + REMAINING).sw(c, c.g[V0][0]);
            }
            // The remainder: decode one block into the buffer, copy its
            // first s2 samples to the output, leave the rest pending.
            c.cycles += 2;
            let s = r32(c, S0);
            c.g[V0][0] = sx(s + BUF);
            if (c.g[S2][0] as i64) > 0 {
                c.cycles += 20;
                c.g[S4][0] = Addr(s + OUT).lw(c);
                Addr(s + COUNT).sw(c, c.g[S2][0]);
                c.g[A0][0] = sx(s + COUNT);
                Addr(s + OUT).sw(c, c.g[V0][0]);
                c.g[S1][0] = 0;
                SND_EAXA_DECODE_BLOCKS.call(c, 0x003c_d0a4)?;
                let s = r32(c, S0);
                let decoded = Addr(s + COUNT).lw(c) as u32;
                c.g[A0][0] = sx(decoded);
                c.g[A2][0] = sx(r32(c, S3).wrapping_add(r32(c, S2)));
                let out = Addr(s + OUT).lw(c) as u32;
                c.g[A1][0] = sx(out);
                let remaining = Addr(s + REMAINING).lw(c) as u32;
                c.g[V1][0] = sx(out.wrapping_add(decoded << 2));
                c.g[A0][0] = sx(0u32.wrapping_sub(decoded));
                c.g[V0][0] = sx(remaining.wrapping_sub(r32(c, S2)));
                c.g[A1][0] = sx(out.wrapping_sub(0x70));
                Addr(s + REMAINING).sw(c, c.g[V0][0]);
                Addr(s + READ).sw(c, c.g[V1][0]);
                Addr(s + PENDING).sw(c, c.g[A0][0]);
                Addr(s + OUT).sw(c, c.g[A1][0]);
                c.cycles += 1;
                c.g[A1][0] = c.g[S4][0];
                loop {
                    c.cycles += 9;
                    let s = r32(c, S0);
                    let block = Addr(s + OUT).lw(c) as u32;
                    c.g[V1][0] = sx(block);
                    let i = r32(c, S1);
                    c.g[S1][0] = sx(i.wrapping_add(1));
                    c.g[V0][0] = sx((i << 2).wrapping_add(block));
                    let more = (c.g[S1][0] as i64) < (c.g[S2][0] as i64);
                    c.g[A0][0] = more as u64;
                    c.lwc1(0, r32(c, V0));
                    c.swc1(0, r32(c, A1));
                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(4));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_d0e0);
                    }
                }
                c.cycles += 1;
                c.g[S3][0] = c.g[A2][0];
            }
            // Stream finished: nothing stays pending.
            c.cycles += 3;
            let s = r32(c, S0);
            c.g[V0][0] = Addr(s + REMAINING).lw(c);
            if (c.g[V0][0] as i64) <= 0 {
                Addr(s + PENDING).sw(c, 0);
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Rider boost fields (`Rider`).
mod rider_boost {
    /// Boost meter, 0..1.
    pub const METER: u32 = 0x2f8;
    /// Boost strength while the button is held (0, 0.25, 0.625 or 1; 0
    /// when not boosting).
    pub const LEVEL: u32 = 0x2fc;
    /// The meter drains only while this float is 0.
    pub const DRAIN_HOLD: u32 = 0x2e8;
    /// The meter drains only while this is set ...
    pub const DRAIN_ON: u32 = 0x304;
    /// ... and this count is below 10.
    pub const DRAIN_COUNT: u32 = 0x2f4;
    /// The rider's controller record (+0xd20: cheat-code input state).
    pub const PAD: u32 = 0x77c;
    /// `.lit4`: level thresholds 2/3 and 1/3, and the drain per tick.
    pub const GP_TWO_THIRDS: u32 = 0x7bfc;
    pub const GP_ONE_THIRD: u32 = 0x7bf8;
    pub const GP_DRAIN: u32 = 0x7bf4;
}

const RECORD_CHEAT_CODE_INPUT: u32 = 0x002f_6ac8;
const AUDIO_GET_INSTANCE: u32 = 0x0028_b180;
const AUDIO_START_BOOST_SOUND: u32 = 0x0029_8d90;
const AUDIO_PLAY_BOOST_EMPTY_SOUND: u32 = 0x0029_9368;
const AUDIO_STOP_BOOST_SOUND: u32 = 0x0029_92d8;

/// `void Rider_HandleBoostInput(Rider* r, int held, int pressed)`
/// (0x114130): the boost button.
///
/// A press is recorded for the cheat-code reader
/// (`RiderBase_RecordCheatCodeInput(pad+0xd20, 0)`). While boost is held
/// with meter left (`r+0x2f8 > 0`), the first tick picks the boost level
/// from the meter (> 2/3: 1, > 1/3: 0.625, else 0.25) and starts the boost
/// sound; the meter then drains by a fixed step per tick when
/// `r+0x2e8 == 0`, `r+0x304` is set and `r+0x2f4 < 10`. Held with an empty
/// meter, a press plays the "empty" sound. Otherwise (and after an empty
/// press) a running boost sound is stopped and the level cleared.
///
/// Registers as the original: the rider in `s0`, `held` in `s2`,
/// `pressed` in `s1`, the FPU temporaries f0-f2 and f20, every block's
/// cycle charge.
pub fn rider_handle_boost_input(c: &mut Cpu) {
    use rider_boost::*;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x40);
        if c.g[S1][0] != 0 {
            c.cycles += 4;
            let pad = Addr(r32(c, S0) + PAD).lw(c) as u32;
            c.g[A1][0] = 0;
            c.g[A0][0] = sx(pad.wrapping_add(0xd20));
            call_ptr(c, RECORD_CHEAT_CODE_INPUT, 0x0011_4168)?;
        }
        let gp = r32(c, GP);
        // Whether the function leaves through the shared tail (one more
        // cycle) or straight from the "held with meter" branch.
        let through_tail = 'level: {
            c.cycles += 2;
            if c.g[S2][0] != 0 {
                c.cycles += 5;
                let rider = r32(c, S0);
                c.lwc1(1, rider + METER);
                c.mtc1(0, 2);
                c.c_lt_s(2, 1);
                if c.fcc() {
                    // Held with meter left.
                    c.cycles += 5;
                    c.lwc1(0, rider + LEVEL);
                    c.c_eq_s(0, 2);
                    if c.fcc() {
                        // First tick: pick the level (f0) from the meter (f1).
                        c.cycles += 6;
                        c.lwc1(0, gp.wrapping_sub(GP_TWO_THIRDS));
                        c.c_lt_s(0, 1);
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(AT, 0);
                        if !c.fcc() {
                            c.cycles += 6;
                            c.lwc1(0, gp.wrapping_sub(GP_ONE_THIRD));
                            c.c_lt_s(0, 1);
                            c.g[AT][0] = 0x3e80_0000;
                            c.mtc1(AT, 0);
                            if c.fcc() {
                                c.cycles += 2;
                                c.g[AT][0] = 0x3f20_0000;
                                c.mtc1(AT, 0);
                            }
                        }
                        c.cycles += 7;
                        c.swc1(0, rider + LEVEL);
                        call_ptr(c, AUDIO_GET_INSTANCE, 0x0011_41d8)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A2][0] = c.g[S1][0];
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, AUDIO_START_BOOST_SOUND, 0x0011_41e8)?;
                    }
                    let rider = r32(c, S0);
                    c.lwc1(1, rider + DRAIN_HOLD);
                    c.cycles += 5;
                    c.mtc1(0, 0);
                    c.c_eq_s(1, 0);
                    if !c.fcc() {
                        break 'level false;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = Addr(rider + DRAIN_ON).lw(c);
                    if c.g[V0][0] == 0 {
                        break 'level false;
                    }
                    c.cycles += 4;
                    let count = Addr(rider + DRAIN_COUNT).lw(c);
                    c.g[V0][0] = ((count as i64) < 10) as u64;
                    if c.g[V0][0] == 0 {
                        break 'level false;
                    }
                    c.cycles += 5;
                    c.lwc1(0, rider + METER);
                    c.lwc1(1, gp.wrapping_sub(GP_DRAIN));
                    c.sub_s(0, 0, 1);
                    c.swc1(0, rider + METER);
                    break 'level true;
                }
                // Held with an empty meter: a press plays the empty sound.
                c.cycles += 2;
                if c.g[S1][0] != 0 {
                    c.cycles += 6;
                    call_ptr(c, AUDIO_GET_INSTANCE, 0x0011_4240)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, AUDIO_PLAY_BOOST_EMPTY_SOUND, 0x0011_424c)?;
                }
            }
            // Not boosting: stop a running boost sound, clear the level.
            let rider = r32(c, S0);
            c.lwc1(0, rider + LEVEL);
            c.cycles += 5;
            c.mtc1(0, 20);
            c.c_lt_s(20, 0);
            if c.fcc() {
                c.cycles += 6;
                call_ptr(c, AUDIO_GET_INSTANCE, 0x0011_426c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, AUDIO_STOP_BOOST_SOUND, 0x0011_4278)?;
            }
            let rider = r32(c, S0);
            c.swc1(20, rider + LEVEL);
            true
        };
        if through_tail {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 6;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// Rider fields read by `Rider_UpdateImpacts`.
mod impacts {
    /// Impulse vector (decays by 0.98 a tick).
    pub const IMPULSE: u32 = 0x3e0;
    /// Impact accumulators (decay by 0.956 / 0.9783 a tick).
    pub const ACCUM: u32 = 0x3f0;
    pub const ACCUM2: u32 = 0x3f4;
    /// The `RiderBase` owner (+0x30: handler state).
    pub const OWNER: u32 = 0x77c;
    /// Air predictor: +0xac mode, +0x98 / +0xa0 heights.
    pub const AIR: u32 = 0x788;
    /// `.lit4` decay factors.
    pub const GP_IMPULSE_DECAY: u32 = 0x791c;
    pub const GP_ACCUM_DECAY: u32 = 0x7918;
    pub const GP_ACCUM2_DECAY: u32 = 0x7914;
}

const RIDER_CHECK_RESET_PATH: Callee = Callee {
    addr: 0x0011_6120,
    f: ee::s00::Rider_CheckResetPath,
};
const RIDER_GET_MAIN_STATE: Callee = Callee {
    addr: 0x0011_fe98,
    f: ee::s00::Rider_GetMainState,
};
const RIDER_UPDATE_BOOST_TIMERS: Callee = Callee {
    addr: 0x0012_00d0,
    f: ee::s00::Rider_UpdateBoostTimers,
};
const RIDER_UPDATE_LEG_IK_BLEND: Callee = Callee {
    addr: 0x0012_0d90,
    f: ee::s00::Rider_UpdateLegIKBlend,
};

/// `void Rider_UpdateImpacts(Rider* r)` (0x1210b0): decays the impulse
/// vector (VU0 `vmul` by 0.98) and the two impact accumulators, then puts
/// the rider back on the course (`Rider_CheckResetPath(r, 0, 2)`) after a
/// hard impact (`r+0x3f0 > 4.5`), or, when riding (main state 1, or 2 with
/// the handler in state 1), after a fall of more than 45 units in air
/// modes 1 and 3 or a second accumulator above 5. Always ends with the
/// boost timers and the leg IK blend.
///
/// The rider stays in `s0` and the constant 1 in `s1` as in the original;
/// the FPU temporaries, VF3-VF5 and every block's cycle charge are kept.
pub fn rider_update_impacts(c: &mut Cpu) {
    use impacts::*;
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        let gp = r32(c, GP);
        c.lwc1(0, gp.wrapping_sub(GP_IMPULSE_DECAY));
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let rider = r32(c, S0);
        // impulse *= 0.98 (VU0, the factor broadcast from VF3.x).
        c.mfc1(V0, 0);
        c.lqc2(4, rider + IMPULSE);
        c.qmtc2(V0, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, rider + IMPULSE);
        // The accumulators decay; a hard impact (> 4.5) resets.
        c.lwc1(3, gp.wrapping_sub(GP_ACCUM_DECAY));
        c.lwc1(0, rider + ACCUM);
        c.lwc1(1, rider + ACCUM2);
        c.mul_s(0, 0, 3);
        c.lwc1(2, gp.wrapping_sub(GP_ACCUM2_DECAY));
        c.g[AT][0] = 0x4090_0000;
        c.mtc1(AT, 3);
        c.mul_s(1, 1, 2);
        c.c_lt_s(3, 0);
        c.swc1(0, rider + ACCUM);
        c.swc1(1, rider + ACCUM2);
        'reset: {
            if c.fcc() {
                c.cycles += 5;
                c.g[A1][0] = 0;
                c.g[A2][0] = 2;
                RIDER_CHECK_RESET_PATH.call(c, 0x0012_1118)?;
                break 'reset;
            }
            // Riding: main state 1, or 2 with the handler in state 1.
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[S1][0] = 1;
            RIDER_GET_MAIN_STATE.call(c, 0x0012_112c)?;
            if c.g[V0][0] != c.g[S1][0] {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                RIDER_GET_MAIN_STATE.call(c, 0x0012_113c)?;
                c.g[V1][0] = 2;
                if c.g[V0][0] != c.g[V1][0] {
                    break 'reset;
                }
                c.cycles += 4;
                let owner = Addr(r32(c, S0) + OWNER).lw(c);
                c.g[V0][0] = owner;
                c.g[V1][0] = Addr((owner as u32).wrapping_add(0x30)).lw(c);
                if c.g[V1][0] != c.g[S1][0] {
                    break 'reset;
                }
                c.cycles += 1;
            }
            let rider = r32(c, S0);
            let air = Addr(rider + AIR).lw(c);
            c.g[V0][0] = air;
            // Air modes 1 and 3 count falls (a1).
            c.cycles += 4;
            c.g[A1][0] = 0;
            let mode = Addr((air as u32).wrapping_add(0xac)).lw(c);
            c.g[V1][0] = mode;
            c.g[A0][0] = air;
            if mode == c.g[S1][0] {
                c.cycles += 1;
                c.g[A1][0] = 1;
            } else {
                c.cycles += 3;
                c.g[V0][0] = 3;
                if mode == 3 {
                    c.cycles += 1;
                    c.g[A1][0] = 1;
                }
            }
            'fell: {
                c.cycles += 2;
                if c.g[A1][0] != 0 {
                    // A fall of more than 45 units.
                    c.cycles += 9;
                    let air = r32(c, A0);
                    c.lwc1(0, air + 0xa0);
                    c.lwc1(1, air + 0x98);
                    c.g[AT][0] = 0x4234_0000;
                    c.mtc1(AT, 2);
                    c.sub_s(1, 1, 0);
                    c.c_lt_s(2, 1);
                    c.g[A0][0] = c.g[S0][0];
                    if c.fcc() {
                        break 'fell;
                    }
                    c.cycles += 1;
                }
                // Or the second accumulator above 5.
                c.lwc1(1, rider + ACCUM2);
                c.cycles += 6;
                c.g[AT][0] = 0x40a0_0000;
                c.mtc1(AT, 0);
                c.c_lt_s(0, 1);
                c.g[A0][0] = c.g[S0][0];
                if !c.fcc() {
                    break 'reset;
                }
            }
            c.cycles += 3;
            c.g[A1][0] = 0;
            c.g[A2][0] = 2;
            RIDER_CHECK_RESET_PATH.call(c, 0x0012_11d0)?;
        }
        c.cycles += 9;
        c.g[A0][0] = c.g[S0][0];
        RIDER_UPDATE_BOOST_TIMERS.call(c, 0x0012_11d8)?;
        c.g[A0][0] = c.g[S0][0];
        RIDER_UPDATE_LEG_IK_BLEND.call(c, 0x0012_11e0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void tWPIGD_Lighting::Blend(const tWPIGD_Lighting* p, float t)`
/// (0x2bd5a8): blends the world painter's lighting toward `p` with weight
/// `w = t*t`.
///
/// Notifies the owner (vtable at `self+4`, slot +0x218) before and after;
/// copies the four 16-byte light vectors (`p+0x4.. -> self+0x8..`,
/// unaligned doubleword pairs), and for the two scalar parameters keeps
/// the blended value and the target:
/// `self+0x48 = w*p[0x24] + (1-w)*self[0x48]`, `self+0x4c = p[0x24]`, and
/// the same for `p+0x28` into `self+0x50`/`self+0x54`.
///
/// `self` stays in `s0`, `p` in `s1` and `w` in f20 as in the original;
/// the FPU temporaries f0-f3 and the 60-cycle block are kept.
pub fn twpigd_lighting_blend(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 60;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x30);
        c.mov_s(20, 12);
        c.mul_s(20, 20, 20);
        let this = r32(c, S0);
        let vt = Addr(this + 4).lw(c) as u32;
        c.g[V1][0] = sx(vt);
        lighting_notify(c, this, vt, 0x002b_d5e0)?;
        // Four light vectors (v0 carries each doubleword).
        let (this, p) = (r32(c, S0), r32(c, S1));
        for (dst, src) in [(0x8u32, 0x4u32), (0x18, 0xc), (0x28, 0x14), (0x38, 0x1c)] {
            if dst == 0x38 {
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(AT, 2);
            }
            c.ldl(V0, p + src + 7);
            c.ldr(V0, p + src);
            c.sdl(V0, this + dst + 7);
            c.sdr(V0, this + dst);
        }
        // f2 = 1 - w; the two blended parameters.
        c.sub_s(2, 2, 20);
        c.lwc1(1, p + 0x24);
        c.lwc1(0, this + 0x48);
        c.mul_s(1, 20, 1);
        c.lwc1(3, this + 0x50);
        c.mul_s(0, 2, 0);
        let vt = Addr(this + 4).lw(c) as u32;
        c.g[V1][0] = sx(vt);
        c.mul_s(2, 2, 3);
        c.add_s(1, 1, 0);
        c.swc1(1, this + 0x48);
        c.lwc1(0, p + 0x24);
        c.swc1(0, this + 0x4c);
        c.lwc1(1, p + 0x28);
        c.mul_s(20, 20, 1);
        c.add_s(20, 20, 2);
        c.swc1(20, this + 0x50);
        c.lwc1(0, p + 0x28);
        c.swc1(0, this + 0x54);
        lighting_notify(c, this, vt, 0x002b_d680)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// The owner notification: `this->vt[+0x218]` (`a0 = this + delta`).
fn lighting_notify(c: &mut Cpu, this: u32, vt: u32, ret: u32) -> crate::call::Flow {
    const NOTIFY: u32 = 0x218;
    let slot = crate::call::VSlot::read(c, sx(vt), NOTIFY);
    c.g[V0][0] = slot.func;
    c.g[A0][0] = slot.this(sx(this));
    call_ptr(c, slot.func as u32, ret)
}

/// `void SndLoadQueue::Update()` (0x28aaf8, `src/sound/asyncsys.cpp`): polls
/// the sound load queue's current asynchronous file read.
///
/// Nothing to do unless a load is running (`self+0x4`). The kind of the
/// current request (ring at `+0x1ac`; rings are `{size, data, head, tail}`,
/// the others at `+0x18c` ids, `+0x19c` name slots of 64 bytes at `+0xc`,
/// `+0x1bc` parameters) picks what happens when `AsyncFile_IsDone(handle)`
/// reports completion: kinds 0 and 2 load a sound bank
/// (`AsyncFile_GetResult`, `SND_BankAdd`; a bank that is not resident is
/// removed, copied to memory the owner allocates through vtable `+0x18`
/// (`EA_MemCopy`) and added again; a resident one is moved there with
/// `SND_BankGetSize`/`SND_BankMove`), kind 1 hands the raw data over; the
/// owner is told through vtable `+0x28` (failures with -1 / 2), the four
/// rings advance and the next request starts (`SndLoadQueue_StartNext`).
/// A failed read (`IsDone < 0`) ends the request
/// (`SndLoadQueue_FinishCurrent(self, 0)`).
///
/// The body keeps the original's block structure (every cycle charge, the
/// ring reads written out at each use, the scratch registers each block
/// leaves), with the calls through the call protocol.
pub fn snd_load_queue_update(c: &mut Cpu) {
    guest(c, |c| {
        'b_28b0d4: {
            c.cycles += 10;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_28b0d4;
            } else {
                'b_28ab4c: {
                    'b_28ab38: {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1b4));
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1b8));
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1ac));
                        if t {
                            break 'b_28ab38;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[S1][0] = 0xffffffffffffffff;
                            if t {
                                break 'b_28ab4c;
                            } else {
                                break 'b_28ab38;
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[S1][0] = c.lw((c.g[V0][0] as u32));
                    break 'b_28ab4c;
                }
                'b_28b0c8: {
                    'b_28b028: {
                        'b_28b01c: {
                            'b_28aeb8: {
                                c.cycles += 3;
                                c.g[V0][0] = 0x1;
                                let t = c.g[S1][0] == c.g[V0][0];
                                c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                                if t {
                                    break 'b_28aeb8;
                                } else {
                                    'b_28ab78: {
                                        'b_28ab70: {
                                            c.cycles += 2;
                                            if c.g[V0][0] == 0u64 {
                                                c.g[V0][0] = 0x2;
                                                break 'b_28ab70;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[S1][0] == 0u64;

                                                if t {
                                                    break 'b_28ab78;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x50),
                                                    );
                                                    if t {
                                                        break 'b_28b0d4;
                                                    } else {
                                                        break 'b_28ab70;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        if c.g[S1][0] != c.g[V0][0] {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                            break 'b_28b0d4;
                                        } else {
                                            break 'b_28ab78;
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                                    call_ptr(c, 0x003df980, 0x0028ab80)?;
                                    c.g[V1][0] = c.g[V0][0];
                                    if c.g[V1][0] == 0u64 {
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                        break 'b_28b0d4;
                                    } else {
                                        'b_28aea8: {
                                            'b_28aea4: {
                                                c.cycles += 2;
                                                let t = (c.g[V1][0] as i64) <= 0;
                                                c.g[V0][0] = 0x1;
                                                if t {
                                                    break 'b_28aea4;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V1][0] != c.g[V0][0];
                                                    c.g[A0][0] = c.g[S0][0];
                                                    if t {
                                                        break 'b_28aea8;
                                                    } else {
                                                        'b_28ae1c: {
                                                            c.cycles += 9;
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                                                            c.g[A1][0] = c.g[SP][0];
                                                            c.sw((c.g[SP][0] as u32), 0u64);
                                                            c.g[A2][0] = c.g[SP][0] | 0x4;
                                                            c.sw(
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x4),
                                                                0u64,
                                                            );
                                                            call_ptr(c, 0x003df9d8, 0x0028abb4)?;
                                                            c.g[V1][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x4));
                                                            let t = (c.g[V1][0] as i64) <= 0;
                                                            c.g[A1][0] = c.lw((c.g[SP][0] as u32));
                                                            if t {
                                                                break 'b_28ae1c;
                                                            } else {
                                                                'b_28ae0c: {
                                                                    'b_28acf8: {
                                                                        c.cycles += 5;
                                                                        c.g[A0][0] = c.g[S0][0];
                                                                        call_ptr(
                                                                            c, 0x003b6098,
                                                                            0x0028abc8,
                                                                        )?;
                                                                        c.g[V1][0] = 0x7;
                                                                        let t = c.g[V0][0]
                                                                            == c.g[V1][0];
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[S0][0] as u32)
                                                                            );
                                                                        if t {
                                                                            break 'b_28acf8;
                                                                        } else {
                                                                            'b_28ac14: {
                                                                                'b_28ac00: {
                                                                                    c.cycles += 9;
                                                                                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18c));
                                                                                    call_ptr(
                                                                                        c,
                                                                                        0x003b6300,
                                                                                        0x0028abdc,
                                                                                    )?;
                                                                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1d4));
                                                                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x194));
                                                                                    c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
                                                                                    c.g[T1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1c));
                                                                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x198));
                                                                                    let t = c.g[A1]
                                                                                        [0]
                                                                                        != c.g[A0]
                                                                                            [0];
                                                                                    c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                    if t {
                                                                                        break 'b_28ac00;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;
                                                                                        let t =
                                                                                            true;
                                                                                        c.g[A1][0] = 0xffffffffffffffff;
                                                                                        if t {
                                                                                            break 'b_28ac14;
                                                                                        } else {
                                                                                            break 'b_28ac00;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                c.cycles += 5;
                                                                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                                                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                                                                c.g[V0][0] =
                                                                                    sx((c.g[V0][0]
                                                                                        as u32)
                                                                                        << 2);
                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                                c.g[A1][0] = c.lw(
                                                                                    (c.g[V0][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_28ac14;
                                                                            }
                                                                            'b_28ac44: {
                                                                                'b_28ac30: {
                                                                                    c.cycles += 5;
                                                                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c4));
                                                                                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1bc));
                                                                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c8));
                                                                                    let t = c.g[V1]
                                                                                        [0]
                                                                                        != c.g[V0]
                                                                                            [0];
                                                                                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                                                                    if t {
                                                                                        break 'b_28ac30;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;
                                                                                        let t =
                                                                                            true;
                                                                                        c.g[A3][0] = 0xffffffffffffffff;
                                                                                        if t {
                                                                                            break 'b_28ac44;
                                                                                        } else {
                                                                                            break 'b_28ac30;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                c.cycles += 5;
                                                                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                                                                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                                                                                c.g[V0][0] =
                                                                                    sx((c.g[V0][0]
                                                                                        as u32)
                                                                                        << 2);
                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                                c.g[A3][0] = c.lw(
                                                                                    (c.g[V0][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_28ac44;
                                                                            }
                                                                            'b_28aca8: {
                                                                                'b_28ac94: {
                                                                                    c.cycles += 18;
                                                                                    c.g[V0][0] = c
                                                                                        .lw(
                                                                                        (c.g[T1][0]
                                                                                            as u32),
                                                                                    );
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        as u32;
                                                                                    c.g[A0][0] =
                                                                                        c.g[T0][0];
                                                                                    call_ptr(
                                                                                        c, t,
                                                                                        0x0028ac50,
                                                                                    )?;
                                                                                    c.g[S2][0] =
                                                                                        c.g[V0][0];
                                                                                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                                                                    c.g[A1][0] = c
                                                                                        .lw(
                                                                                        (c.g[SP][0]
                                                                                            as u32),
                                                                                    );
                                                                                    c.g[A0][0] =
                                                                                        c.g[S2][0];
                                                                                    call_ptr(
                                                                                        c,
                                                                                        0x003e6574,
                                                                                        0x0028ac64,
                                                                                    )?;
                                                                                    c.g[A0][0] =
                                                                                        c.g[S0][0];
                                                                                    c.g[A1][0] =
                                                                                        c.g[S2][0];
                                                                                    call_ptr(
                                                                                        c,
                                                                                        0x003b6098,
                                                                                        0x0028ac70,
                                                                                    )?;
                                                                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1d4));
                                                                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x194));
                                                                                    c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                                                                                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2c));
                                                                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x198));
                                                                                    let t = c.g[A1]
                                                                                        [0]
                                                                                        != c.g[A0]
                                                                                            [0];
                                                                                    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                    if t {
                                                                                        break 'b_28ac94;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;
                                                                                        let t =
                                                                                            true;
                                                                                        c.g[A1][0] = 0xffffffffffffffff;
                                                                                        if t {
                                                                                            break 'b_28aca8;
                                                                                        } else {
                                                                                            break 'b_28ac94;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                c.cycles += 5;
                                                                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                                                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                                                                c.g[V0][0] =
                                                                                    sx((c.g[V0][0]
                                                                                        as u32)
                                                                                        << 2);
                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                                c.g[A1][0] = c.lw(
                                                                                    (c.g[V0][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_28aca8;
                                                                            }
                                                                            'b_28acd4: {
                                                                                'b_28acc0: {
                                                                                    c.cycles += 4;
                                                                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1a4));
                                                                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1a8));
                                                                                    let t = c.g[V1]
                                                                                        [0]
                                                                                        != c.g[V0]
                                                                                            [0];
                                                                                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x19c));
                                                                                    if t {
                                                                                        break 'b_28acc0;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;
                                                                                        let t =
                                                                                            true;
                                                                                        c.g[V0][0] = 0xffffffffffffffff;
                                                                                        if t {
                                                                                            break 'b_28acd4;
                                                                                        } else {
                                                                                            break 'b_28acc0;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                c.cycles += 5;
                                                                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                                                                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                                                                                c.g[V0][0] =
                                                                                    sx((c.g[V0][0]
                                                                                        as u32)
                                                                                        << 2);
                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                                c.g[V0][0] = c.lw(
                                                                                    (c.g[V0][0]
                                                                                        as u32),
                                                                                );
                                                                                break 'b_28acd4;
                                                                            }
                                                                            c.cycles += 9;
                                                                            c.g[T0][0] =
                                                                                sx((c.g[V0][0]
                                                                                    as u32)
                                                                                    << 6);
                                                                            c.g[A0][0] = c.g[A3][0];
                                                                            c.g[V0][0] = c
                                                                                .lw((c.g[A2][0]
                                                                                    as u32));
                                                                            c.g[T0][0] =
                                                                                sx((c.g[T0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0xc,
                                                                                    ));
                                                                            c.g[T2][0] = c.lw((c.g
                                                                                [SP][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                            c.g[T0][0] =
                                                                                sx((c.g[S0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        (c.g[T0][0]
                                                                                            as u32),
                                                                                    ));
                                                                            c.g[A2][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            let t = true;
                                                                            c.g[T1][0] = c.g[S2][0];
                                                                            if t {
                                                                                break 'b_28ae0c;
                                                                            } else {
                                                                                break 'b_28acf8;
                                                                            }
                                                                        }
                                                                    }
                                                                    'b_28ad3c: {
                                                                        'b_28ad28: {
                                                                            c.cycles += 10;
                                                                            c.g[S1][0] =
                                                                                sx((c.g[S0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x18c,
                                                                                    ));
                                                                            call_ptr(
                                                                                c, 0x003b6528,
                                                                                0x0028ad00,
                                                                            )?;
                                                                            c.g[S3][0] = c.g[V0][0];
                                                                            c.g[A1][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x194,
                                                                                ));
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x1d4,
                                                                                ));
                                                                            c.g[A0][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x198,
                                                                                ));
                                                                            c.g[V1][0] = c.lh((c.g
                                                                                [V0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x18,
                                                                                ));
                                                                            c.g[T0][0] =
                                                                                sx((c.g[V0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x1c,
                                                                                    ));
                                                                            let t = c.g[A1][0]
                                                                                != c.g[A0][0];
                                                                            c.g[A2][0] =
                                                                                sx((c.g[S0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        (c.g[V1][0]
                                                                                            as u32),
                                                                                    ));
                                                                            if t {
                                                                                break 'b_28ad28;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                let t = true;
                                                                                c.g[A1][0] = 0xffffffffffffffff;
                                                                                if t {
                                                                                    break 'b_28ad3c;
                                                                                } else {
                                                                                    break 'b_28ad28;
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 5;
                                                                        c.g[V0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        c.g[V1][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    (c.g[V1][0]
                                                                                        as u32),
                                                                                ));
                                                                        c.g[A1][0] =
                                                                            c.lw(
                                                                                (c.g[V0][0] as u32)
                                                                            );
                                                                        break 'b_28ad3c;
                                                                    }
                                                                    'b_28ad68: {
                                                                        'b_28ad54: {
                                                                            c.cycles += 4;
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x1c4,
                                                                                ));
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x1c8,
                                                                                ));
                                                                            let t = c.g[V1][0]
                                                                                != c.g[V0][0];
                                                                            c.g[A0][0] =
                                                                                sx((c.g[S0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x1bc,
                                                                                    ));
                                                                            if t {
                                                                                break 'b_28ad54;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                let t = true;
                                                                                c.g[A3][0] = 0xffffffffffffffff;
                                                                                if t {
                                                                                    break 'b_28ad68;
                                                                                } else {
                                                                                    break 'b_28ad54;
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 5;
                                                                        c.g[V0][0] = c
                                                                            .lw((c.g[A0][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        c.g[V1][0] = c
                                                                            .lw((c.g[A0][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    (c.g[V1][0]
                                                                                        as u32),
                                                                                ));
                                                                        c.g[A3][0] =
                                                                            c.lw(
                                                                                (c.g[V0][0] as u32)
                                                                            );
                                                                        break 'b_28ad68;
                                                                    }
                                                                    'b_28adc0: {
                                                                        'b_28adac: {
                                                                            c.cycles += 15;
                                                                            c.g[A0][0] = c.g[A2][0];
                                                                            c.g[V0][0] = c
                                                                                .lw((c.g[T0][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[V0][0] as u32;
                                                                            c.g[A2][0] = c.g[S3][0];
                                                                            call_ptr(
                                                                                c, t, 0x0028ad78,
                                                                            )?;
                                                                            c.g[S2][0] = c.g[V0][0];
                                                                            c.g[A1][0] = c
                                                                                .lw((c.g[S0][0]
                                                                                    as u32));
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            call_ptr(
                                                                                c, 0x003b6548,
                                                                                0x0028ad88,
                                                                            )?;
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x1d4,
                                                                                ));
                                                                            c.g[A1][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x194,
                                                                                ));
                                                                            c.g[V0][0] = c.lh((c.g
                                                                                [V1][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x28,
                                                                                ));
                                                                            c.g[A2][0] =
                                                                                sx((c.g[V1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x2c,
                                                                                    ));
                                                                            c.g[A0][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x198,
                                                                                ));
                                                                            let t = c.g[A1][0]
                                                                                != c.g[A0][0];
                                                                            c.g[A3][0] =
                                                                                sx((c.g[S0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        (c.g[V0][0]
                                                                                            as u32),
                                                                                    ));
                                                                            if t {
                                                                                break 'b_28adac;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                let t = true;
                                                                                c.g[A1][0] = 0xffffffffffffffff;
                                                                                if t {
                                                                                    break 'b_28adc0;
                                                                                } else {
                                                                                    break 'b_28adac;
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 5;
                                                                        c.g[V0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        c.g[V1][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    (c.g[V1][0]
                                                                                        as u32),
                                                                                ));
                                                                        c.g[A1][0] =
                                                                            c.lw(
                                                                                (c.g[V0][0] as u32)
                                                                            );
                                                                        break 'b_28adc0;
                                                                    }
                                                                    'b_28adec: {
                                                                        'b_28add8: {
                                                                            c.cycles += 4;
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x1a4,
                                                                                ));
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x1a8,
                                                                                ));
                                                                            let t = c.g[V1][0]
                                                                                != c.g[V0][0];
                                                                            c.g[A0][0] =
                                                                                sx((c.g[S0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x19c,
                                                                                    ));
                                                                            if t {
                                                                                break 'b_28add8;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                let t = true;
                                                                                c.g[V0][0] = 0xffffffffffffffff;
                                                                                if t {
                                                                                    break 'b_28adec;
                                                                                } else {
                                                                                    break 'b_28add8;
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 5;
                                                                        c.g[V0][0] = c
                                                                            .lw((c.g[A0][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        c.g[V1][0] = c
                                                                            .lw((c.g[A0][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    (c.g[V1][0]
                                                                                        as u32),
                                                                                ));
                                                                        c.g[V0][0] =
                                                                            c.lw(
                                                                                (c.g[V0][0] as u32)
                                                                            );
                                                                        break 'b_28adec;
                                                                    }
                                                                    c.cycles += 8;
                                                                    c.g[T0][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        << 6);
                                                                    c.g[A0][0] = c.g[A3][0];
                                                                    c.g[V0][0] =
                                                                        c.lw((c.g[A2][0] as u32));
                                                                    c.g[T0][0] = sx((c.g[T0][0]
                                                                        as u32)
                                                                        .wrapping_add(0xc));
                                                                    c.g[A2][0] =
                                                                        c.lw((c.g[S0][0] as u32));
                                                                    c.g[T0][0] = sx((c.g[S0][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            (c.g[T0][0] as u32),
                                                                        ));
                                                                    c.g[T1][0] = c.g[S2][0];
                                                                    c.g[T2][0] = c.g[S3][0];
                                                                    break 'b_28ae0c;
                                                                }
                                                                c.cycles += 4;
                                                                let t = c.g[V0][0] as u32;
                                                                c.g[A3][0] = 0u64;
                                                                call_ptr(c, t, 0x0028ae14)?;
                                                                let t = true;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x194));
                                                                if t {
                                                                    break 'b_28b028;
                                                                } else {
                                                                    break 'b_28ae1c;
                                                                }
                                                            }
                                                        }
                                                        'b_28ae58: {
                                                            'b_28ae44: {
                                                                c.cycles += 8;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x1d4));
                                                                c.g[A2][0] = sx((c.g[S0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x18c));
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x194));
                                                                c.g[V0][0] = c
                                                                    .lh((c.g[A0][0] as u32)
                                                                        .wrapping_add(0x28));
                                                                c.g[A3][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x2c));
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x198));
                                                                let t = c.g[A1][0] != c.g[V1][0];
                                                                c.g[A0][0] = sx((c.g[S0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[V0][0] as u32),
                                                                    ));
                                                                if t {
                                                                    break 'b_28ae44;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    let t = true;
                                                                    c.g[A1][0] = 0xffffffffffffffff;
                                                                    if t {
                                                                        break 'b_28ae58;
                                                                    } else {
                                                                        break 'b_28ae44;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 5;
                                                            c.g[V0][0] = c.lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0xc));
                                                            c.g[V1][0] = c.lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.g[V0][0] =
                                                                sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add((c.g[V1][0] as u32)));
                                                            c.g[A1][0] = c.lw((c.g[V0][0] as u32));
                                                            break 'b_28ae58;
                                                        }
                                                        'b_28ae84: {
                                                            'b_28ae70: {
                                                                c.cycles += 4;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x1a4));
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x1a8));
                                                                let t = c.g[V1][0] != c.g[V0][0];
                                                                c.g[A2][0] = sx((c.g[S0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x19c));
                                                                if t {
                                                                    break 'b_28ae70;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    let t = true;
                                                                    c.g[V0][0] = 0xffffffffffffffff;
                                                                    if t {
                                                                        break 'b_28ae84;
                                                                    } else {
                                                                        break 'b_28ae70;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 5;
                                                            c.g[V0][0] = c.lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0xc));
                                                            c.g[V1][0] = c.lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.g[V0][0] =
                                                                sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add((c.g[V1][0] as u32)));
                                                            c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                                                            break 'b_28ae84;
                                                        }
                                                        c.cycles += 8;
                                                        c.g[T0][0] = sx((c.g[V0][0] as u32) << 6);
                                                        c.g[A2][0] = 0xffffffffffffffff;
                                                        c.g[V0][0] = c.lw((c.g[A3][0] as u32));
                                                        c.g[T0][0] =
                                                            sx((c.g[T0][0] as u32)
                                                                .wrapping_add(0xc));
                                                        c.g[T0][0] = sx((c.g[S0][0] as u32)
                                                            .wrapping_add((c.g[T0][0] as u32)));
                                                        c.g[A3][0] = 0x2;
                                                        let t = true;
                                                        c.g[T1][0] = 0u64;
                                                        if t {
                                                            break 'b_28b01c;
                                                        } else {
                                                            break 'b_28aea4;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[A0][0] = c.g[S0][0];
                                            break 'b_28aea8;
                                        }
                                        c.cycles += 4;
                                        c.g[A1][0] = 0u64;
                                        Callee {
                                            addr: 0x00289df0,
                                            f: ee::s07::SndLoadQueue_FinishCurrent,
                                        }
                                        .call(c, 0x0028aeb0)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                        if t {
                                            break 'b_28b0d4;
                                        } else {
                                            break 'b_28aeb8;
                                        }
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                            call_ptr(c, 0x003df980, 0x0028aec0)?;
                            if c.g[V0][0] == 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                break 'b_28b0d4;
                            } else {
                                c.cycles += 2;
                                let t = (c.g[V0][0] as i64) <= 0;
                                c.g[A0][0] = c.g[S0][0];
                                if t {
                                    break 'b_28b0c8;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] != c.g[S1][0];
                                    c.g[A1][0] = c.g[SP][0] | 0x8;
                                    if t {
                                        break 'b_28b0c8;
                                    } else {
                                        'b_28af98: {
                                            c.cycles += 8;
                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                                            c.g[A2][0] = c.g[SP][0] | 0xc;
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0u64);
                                            call_ptr(c, 0x003df9d8, 0x0028aeec)?;
                                            c.g[V1][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                                            let t = (c.g[V1][0] as i64) <= 0;
                                            c.g[A1][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                            if t {
                                                break 'b_28af98;
                                            } else {
                                                'b_28af3c: {
                                                    'b_28af28: {
                                                        c.cycles += 10;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        call_ptr(c, 0x003b6098, 0x0028af00)?;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1d4));
                                                        c.g[A2][0] =
                                                            sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x18c));
                                                        c.g[A1][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x194));
                                                        c.g[V1][0] = c
                                                            .lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x28));
                                                        c.g[T3][0] =
                                                            sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x2c));
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x198));
                                                        let t = c.g[A1][0] != c.g[A0][0];
                                                        c.g[A3][0] = sx((c.g[S0][0] as u32)
                                                            .wrapping_add((c.g[V1][0] as u32)));
                                                        if t {
                                                            break 'b_28af28;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[A1][0] = 0xffffffffffffffff;
                                                            if t {
                                                                break 'b_28af3c;
                                                            } else {
                                                                break 'b_28af28;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 5;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                                                    c.g[V1][0] =
                                                        c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add((c.g[V1][0] as u32)));
                                                    c.g[A1][0] = c.lw((c.g[V0][0] as u32));
                                                    break 'b_28af3c;
                                                }
                                                'b_28af68: {
                                                    'b_28af54: {
                                                        c.cycles += 4;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1a8));
                                                        let t = c.g[V1][0] != c.g[V0][0];
                                                        c.g[A0][0] =
                                                            sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x19c));
                                                        if t {
                                                            break 'b_28af54;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[V0][0] = 0xffffffffffffffff;
                                                            if t {
                                                                break 'b_28af68;
                                                            } else {
                                                                break 'b_28af54;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 5;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                                                    c.g[V1][0] =
                                                        c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add((c.g[V1][0] as u32)));
                                                    c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                                                    break 'b_28af68;
                                                }
                                                c.cycles += 12;
                                                c.g[T0][0] = sx((c.g[V0][0] as u32) << 6);
                                                c.g[A0][0] = c.g[A3][0];
                                                c.g[T0][0] =
                                                    sx((c.g[T0][0] as u32).wrapping_add(0xc));
                                                c.g[T1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                                c.g[T2][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                                                c.g[T0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add((c.g[T0][0] as u32)));
                                                c.g[V0][0] = c.lw((c.g[T3][0] as u32));
                                                c.g[A3][0] = 0u64;
                                                let t = c.g[V0][0] as u32;
                                                c.g[A2][0] = c.lw((c.g[S0][0] as u32));
                                                call_ptr(c, t, 0x0028af90)?;
                                                let t = true;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x194));
                                                if t {
                                                    break 'b_28b028;
                                                } else {
                                                    break 'b_28af98;
                                                }
                                            }
                                        }
                                        'b_28afd4: {
                                            'b_28afc0: {
                                                c.cycles += 8;
                                                c.g[A0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x1d4));
                                                c.g[A2][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x18c));
                                                c.g[A1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x194));
                                                c.g[V0][0] =
                                                    c.lh((c.g[A0][0] as u32).wrapping_add(0x28));
                                                c.g[A3][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x2c));
                                                c.g[V1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x198));
                                                let t = c.g[A1][0] != c.g[V1][0];
                                                c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                if t {
                                                    break 'b_28afc0;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[A1][0] = 0xffffffffffffffff;
                                                    if t {
                                                        break 'b_28afd4;
                                                    } else {
                                                        break 'b_28afc0;
                                                    }
                                                }
                                            }
                                            c.cycles += 5;
                                            c.g[V0][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                                            c.g[V1][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                            c.g[A1][0] = c.lw((c.g[V0][0] as u32));
                                            break 'b_28afd4;
                                        }
                                        'b_28b000: {
                                            'b_28afec: {
                                                c.cycles += 4;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x1a4));
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x1a8));
                                                let t = c.g[V1][0] != c.g[V0][0];
                                                c.g[A2][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x19c));
                                                if t {
                                                    break 'b_28afec;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[V0][0] = 0xffffffffffffffff;
                                                    if t {
                                                        break 'b_28b000;
                                                    } else {
                                                        break 'b_28afec;
                                                    }
                                                }
                                            }
                                            c.cycles += 5;
                                            c.g[V0][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                                            c.g[V1][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                            c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                                            break 'b_28b000;
                                        }
                                        c.cycles += 7;
                                        c.g[T0][0] = sx((c.g[V0][0] as u32) << 6);
                                        c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0xc));
                                        c.g[V0][0] = c.lw((c.g[A3][0] as u32));
                                        c.g[T0][0] =
                                            sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[T0][0] as u32)));
                                        c.g[A2][0] = 0xffffffffffffffff;
                                        c.g[A3][0] = 0x2;
                                        break 'b_28b01c;
                                    }
                                }
                            }
                        }
                        c.cycles += 3;
                        let t = c.g[V0][0] as u32;
                        c.g[T2][0] = 0u64;
                        call_ptr(c, t, 0x0028b024)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x194));
                        break 'b_28b028;
                    }
                    'b_28b04c: {
                        c.cycles += 3;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x198));
                        let t = c.g[V0][0] == c.g[A0][0];
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
                        if t {
                            break 'b_28b04c;
                        } else {
                            c.cycles += 6;
                            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18c));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x198), c.g[V0][0]);
                            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            if c.g[V0][0] == 0u64 {
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x198), 0u64);
                                break 'b_28b04c;
                            } else {
                                break 'b_28b04c;
                            }
                        }
                    }
                    'b_28b070: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1a4));
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1a8));
                        let t = c.g[V0][0] == c.g[A0][0];
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        if t {
                            break 'b_28b070;
                        } else {
                            c.cycles += 5;
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x19c));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x1a8), c.g[V0][0]);
                            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            if c.g[V0][0] == 0u64 {
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x1a8), 0u64);
                                break 'b_28b070;
                            } else {
                                break 'b_28b070;
                            }
                        }
                    }
                    'b_28b094: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1b4));
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1b8));
                        let t = c.g[V0][0] == c.g[A0][0];
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        if t {
                            break 'b_28b094;
                        } else {
                            c.cycles += 5;
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1ac));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x1b8), c.g[V0][0]);
                            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            if c.g[V0][0] == 0u64 {
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x1b8), 0u64);
                                break 'b_28b094;
                            } else {
                                break 'b_28b094;
                            }
                        }
                    }
                    'b_28b0b8: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c4));
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c8));
                        let t = c.g[V0][0] == c.g[A0][0];
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        if t {
                            break 'b_28b0b8;
                        } else {
                            c.cycles += 5;
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1bc));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x1c8), c.g[V0][0]);
                            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            if c.g[V0][0] == 0u64 {
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c8), 0u64);
                                break 'b_28b0b8;
                            } else {
                                break 'b_28b0b8;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x0028a298,
                        f: ee::s07::SndLoadQueue_StartNext,
                    }
                    .call(c, 0x0028b0c0)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    if t {
                        break 'b_28b0d4;
                    } else {
                        break 'b_28b0c8;
                    }
                }
                c.cycles += 3;
                c.g[A1][0] = 0u64;
                Callee {
                    addr: 0x00289df0,
                    f: ee::s07::SndLoadQueue_FinishCurrent,
                }
                .call(c, 0x0028b0d0)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_28b0d4;
            }
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    })
}

/// `bool Collide_SphereVsBoxEdges(const Vec4* min, const Vec4* max, const
/// Vec4* centre, Vec4* point, Vec4* normal, float* dist, float r)`
/// (0x329590, asm only in the reference): a sphere against an axis-aligned
/// box's faces.
///
/// For each axis in turn (x, y, z), when the centre lies inside the box's
/// extent on the other two axes and the sphere reaches the face pair
/// (`centre - min + r >= 0` and `max - centre + r >= 0`), the nearer face
/// is taken if it beats the best distance so far (`*dist`, starting at the
/// `.lit4` 1e30): `point` is the centre snapped onto that face and
/// `normal` the axis unit vector (table at 0x4ff140) for the max face, or
/// its negation (VU0 `vmul` by -1) for the min face. Returns whether any
/// face was hit (`*dist` no longer 1e30).
///
/// A leaf; the body keeps the original's block structure (every cycle
/// charge, the FPU temporaries f0-f5, VF3-VF5, v1 and the 0x20-byte stack
/// scratch the negated normal goes through).
pub fn collide_sphere_vs_box_edges(c: &mut Cpu) {
    'b_3296b8: {
        'b_3296b4: {
            c.cycles += 8;
            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x2df0));
            c.swc1(5, (c.g[T1][0] as u32));
            c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x4));
            c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
            c.c_le_s(0, 1);
            let t = !c.fcc();
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            if t {
                break 'b_3296b4;
            } else {
                c.cycles += 5;
                c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.c_le_s(1, 0);
                if !c.fcc() {
                    c.lwc1(1, (c.g[A2][0] as u32));
                    break 'b_3296b8;
                } else {
                    c.cycles += 6;
                    c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x8));
                    c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.c_le_s(0, 1);
                    if !c.fcc() {
                        c.lwc1(1, (c.g[A2][0] as u32));
                        break 'b_3296b8;
                    } else {
                        c.cycles += 5;
                        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                        c.c_le_s(1, 0);
                        if !c.fcc() {
                            c.lwc1(1, (c.g[A2][0] as u32));
                            break 'b_3296b8;
                        } else {
                            c.cycles += 11;
                            c.lwc1(0, (c.g[A2][0] as u32));
                            c.lwc1(1, (c.g[A0][0] as u32));
                            c.lwc1(2, (c.g[A1][0] as u32));
                            c.sub_s(1, 0, 1);
                            c.mtc1(0, 4);
                            c.sub_s(2, 2, 0);
                            c.add_s(3, 1, 12);
                            c.c_le_s(4, 3);
                            let t = !c.fcc();
                            c.add_s(2, 2, 12);
                            if t {
                                break 'b_3296b4;
                            } else {
                                c.cycles += 4;
                                c.c_le_s(4, 2);
                                if !c.fcc() {
                                    c.lwc1(1, (c.g[A2][0] as u32));
                                    break 'b_3296b8;
                                } else {
                                    'b_329684: {
                                        c.cycles += 4;
                                        c.c_lt_s(3, 5);
                                        if !c.fcc() {
                                            c.lwc1(0, (c.g[T1][0] as u32));
                                            break 'b_329684;
                                        } else {
                                            c.cycles += 18;
                                            c.lq(2, (c.g[A2][0] as u32));
                                            c.g[AT][0] = 0xffffffffbf800000;
                                            c.mtc1(1, 1);
                                            c.sq(2, (c.g[A3][0] as u32));
                                            c.g[V0][0] = 0x500000;
                                            c.lwc1(0, (c.g[A0][0] as u32));
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_sub(0xec0));
                                            c.swc1(0, (c.g[A3][0] as u32));
                                            c.swc1(3, (c.g[T1][0] as u32));
                                            c.mfc1(3, 1);
                                            c.lqc2(4, (c.g[V0][0] as u32));
                                            c.qmtc2(3, 3);
                                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
                                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                                            c.sq(2, (c.g[T0][0] as u32));
                                            c.sq(2, (c.g[SP][0] as u32));
                                            c.lwc1(0, (c.g[T1][0] as u32));
                                            break 'b_329684;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.c_lt_s(2, 0);
                                    let t = !c.fcc();
                                    c.g[V1][0] = 0x500000;
                                    if t {
                                        break 'b_3296b4;
                                    } else {
                                        c.cycles += 8;
                                        c.lq(2, (c.g[A2][0] as u32));
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xec0));
                                        c.sq(2, (c.g[A3][0] as u32));
                                        c.lwc1(0, (c.g[A1][0] as u32));
                                        c.swc1(0, (c.g[A3][0] as u32));
                                        c.swc1(2, (c.g[T1][0] as u32));
                                        c.lq(2, (c.g[V1][0] as u32));
                                        c.sq(2, (c.g[T0][0] as u32));
                                        break 'b_3296b4;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 1;
        c.lwc1(1, (c.g[A2][0] as u32));
        break 'b_3296b8;
    }
    'b_3297d4: {
        c.cycles += 5;
        c.lwc1(0, (c.g[A0][0] as u32));
        c.c_le_s(0, 1);
        if !c.fcc() {
            c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x4));
            break 'b_3297d4;
        } else {
            c.cycles += 5;
            c.lwc1(0, (c.g[A1][0] as u32));
            c.c_le_s(1, 0);
            if !c.fcc() {
                c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x4));
                break 'b_3297d4;
            } else {
                c.cycles += 6;
                c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.c_le_s(0, 1);
                if !c.fcc() {
                    c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x4));
                    break 'b_3297d4;
                } else {
                    c.cycles += 5;
                    c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                    c.c_le_s(1, 0);
                    if !c.fcc() {
                        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x4));
                        break 'b_3297d4;
                    } else {
                        'b_3297d0: {
                            c.cycles += 11;
                            c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                            c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x4));
                            c.lwc1(2, (c.g[A1][0] as u32).wrapping_add(0x4));
                            c.sub_s(1, 0, 1);
                            c.mtc1(0, 4);
                            c.sub_s(2, 2, 0);
                            c.add_s(3, 1, 12);
                            c.c_le_s(4, 3);
                            let t = !c.fcc();
                            c.add_s(2, 2, 12);
                            if t {
                                break 'b_3297d0;
                            } else {
                                c.cycles += 4;
                                c.c_le_s(4, 2);
                                if !c.fcc() {
                                    c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x4));
                                    break 'b_3297d4;
                                } else {
                                    'b_3297a0: {
                                        c.cycles += 4;
                                        c.lwc1(0, (c.g[T1][0] as u32));
                                        c.c_lt_s(3, 0);
                                        let t = !c.fcc();

                                        if t {
                                            break 'b_3297a0;
                                        } else {
                                            c.cycles += 18;
                                            c.lq(2, (c.g[A2][0] as u32));
                                            c.g[AT][0] = 0xffffffffbf800000;
                                            c.mtc1(1, 1);
                                            c.sq(2, (c.g[A3][0] as u32));
                                            c.g[V0][0] = 0x500000;
                                            c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_sub(0xeb0));
                                            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
                                            c.swc1(3, (c.g[T1][0] as u32));
                                            c.mfc1(3, 1);
                                            c.lqc2(4, (c.g[V0][0] as u32));
                                            c.qmtc2(3, 3);
                                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
                                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                                            c.sq(2, (c.g[T0][0] as u32));
                                            c.sq(2, (c.g[SP][0] as u32));
                                            c.lwc1(0, (c.g[T1][0] as u32));
                                            break 'b_3297a0;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.c_lt_s(2, 0);
                                    let t = !c.fcc();
                                    c.g[V1][0] = 0x500000;
                                    if t {
                                        break 'b_3297d0;
                                    } else {
                                        c.cycles += 8;
                                        c.lq(2, (c.g[A2][0] as u32));
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xeb0));
                                        c.sq(2, (c.g[A3][0] as u32));
                                        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
                                        c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
                                        c.swc1(2, (c.g[T1][0] as u32));
                                        c.lq(2, (c.g[V1][0] as u32));
                                        c.sq(2, (c.g[T0][0] as u32));
                                        break 'b_3297d0;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x4));
                        break 'b_3297d4;
                    }
                }
            }
        }
    }
    'b_3298f0: {
        c.cycles += 5;
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
        c.c_le_s(0, 1);
        if !c.fcc() {
            c.lwc1(1, (c.g[T1][0] as u32));
            break 'b_3298f0;
        } else {
            c.cycles += 5;
            c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
            c.c_le_s(1, 0);
            if !c.fcc() {
                c.lwc1(1, (c.g[T1][0] as u32));
                break 'b_3298f0;
            } else {
                c.cycles += 6;
                c.lwc1(1, (c.g[A2][0] as u32));
                c.lwc1(0, (c.g[A0][0] as u32));
                c.c_le_s(0, 1);
                if !c.fcc() {
                    c.lwc1(1, (c.g[T1][0] as u32));
                    break 'b_3298f0;
                } else {
                    c.cycles += 5;
                    c.lwc1(0, (c.g[A1][0] as u32));
                    c.c_le_s(1, 0);
                    if !c.fcc() {
                        c.lwc1(1, (c.g[T1][0] as u32));
                        break 'b_3298f0;
                    } else {
                        'b_3298ec: {
                            c.cycles += 11;
                            c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                            c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x8));
                            c.lwc1(2, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.sub_s(1, 0, 1);
                            c.mtc1(0, 4);
                            c.sub_s(2, 2, 0);
                            c.add_s(3, 1, 12);
                            c.c_le_s(4, 3);
                            let t = !c.fcc();
                            c.add_s(2, 2, 12);
                            if t {
                                break 'b_3298ec;
                            } else {
                                c.cycles += 4;
                                c.c_le_s(4, 2);
                                if !c.fcc() {
                                    c.lwc1(1, (c.g[T1][0] as u32));
                                    break 'b_3298f0;
                                } else {
                                    'b_3298bc: {
                                        c.cycles += 4;
                                        c.lwc1(0, (c.g[T1][0] as u32));
                                        c.c_lt_s(3, 0);
                                        let t = !c.fcc();

                                        if t {
                                            break 'b_3298bc;
                                        } else {
                                            c.cycles += 18;
                                            c.lq(2, (c.g[A2][0] as u32));
                                            c.g[AT][0] = 0xffffffffbf800000;
                                            c.mtc1(1, 1);
                                            c.sq(2, (c.g[A3][0] as u32));
                                            c.g[V0][0] = 0x500000;
                                            c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_sub(0xea0));
                                            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                                            c.swc1(3, (c.g[T1][0] as u32));
                                            c.mfc1(3, 1);
                                            c.lqc2(4, (c.g[V0][0] as u32));
                                            c.qmtc2(3, 3);
                                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
                                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                                            c.sq(2, (c.g[T0][0] as u32));
                                            c.sq(2, (c.g[SP][0] as u32));
                                            c.lwc1(0, (c.g[T1][0] as u32));
                                            break 'b_3298bc;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.c_lt_s(2, 0);
                                    let t = !c.fcc();
                                    c.g[V1][0] = 0x500000;
                                    if t {
                                        break 'b_3298ec;
                                    } else {
                                        c.cycles += 8;
                                        c.lq(2, (c.g[A2][0] as u32));
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xea0));
                                        c.sq(2, (c.g[A3][0] as u32));
                                        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                                        c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                                        c.swc1(2, (c.g[T1][0] as u32));
                                        c.lq(2, (c.g[V1][0] as u32));
                                        c.sq(2, (c.g[T0][0] as u32));
                                        break 'b_3298ec;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.lwc1(1, (c.g[T1][0] as u32));
                        break 'b_3298f0;
                    }
                }
            }
        }
    }
    'b_329908: {
        c.cycles += 5;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x2dec));
        c.c_eq_s(1, 0);
        let t = !c.fcc();
        c.g[V0][0] = 0x1;
        if t {
            break 'b_329908;
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_329908;
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    c.pc = t;
    return;
}

const SNDSTRM_GET_STREAM: Callee = Callee {
    addr: 0x003b_6a38,
    f: ee::s13::SNDSTRM_GetStream,
};
const SNDSTRM_FIND_REQUEST: Callee = Callee {
    addr: 0x003b_7a70,
    f: ee::s13::SNDSTRM_FindRequest,
};
const SND_UMUL32X32: Callee = Callee {
    addr: 0x003c_6380,
    f: ee::s13::SND_UMul32x32,
};
const SND_UDIV64BY32: Callee = Callee {
    addr: 0x003c_64a8,
    f: ee::s13::SND_UDiv64by32,
};

/// `int SNDSTRM_GetRequestStatus(int handle, SndStrmStatus* st)`
/// (0x3b7c40): fills `st` (four words, cleared first) for a stream request.
///
/// A negative handle, or one whose stream (low byte, `SNDSTRM_GetStream`)
/// does not exist, returns -8. Otherwise returns 0 with `st[0]` the
/// state: 3 when the request is unknown (`SNDSTRM_FindRequest`), 0 when it
/// is not active (`req+0x24`), 2 when it is the stream's current request
/// (`stream+0x11c`) and 1 when it is queued. For active requests `st[1]`
/// and `st[2]` are the start position (`req+0x14`) and the length
/// (`req+0x18 - req+0x14`) in milliseconds at the stream's rate (current:
/// `stream+0x14`, queued: `stream+0x18`; 64-bit `x * 1000 / rate`), and
/// `st[3]` is `req+0x1c * 1000 / rate`.
///
/// Registers as the original: the handle and then the request in `s0`,
/// the stream and then the rate in `s1`, `st` in `s2`, the 64-bit products
/// passed through the stack, LO/HI from the final `mult`/`divu`.
pub fn sndstrm_get_request_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        let st = r32(c, S2);
        for i in 0..4 {
            Addr(st + 4 * i).sw(c, 0);
        }
        'done: {
            'bad: {
                if (c.g[S0][0] as i64) < 0 {
                    break 'bad;
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0] & 0xff;
                SNDSTRM_GET_STREAM.call(c, 0x003b_7c78)?;
                c.g[S1][0] = c.g[V0][0];
                if c.g[S1][0] == 0 {
                    break 'bad;
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                SNDSTRM_FIND_REQUEST.call(c, 0x003b_7c94)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = 3;
                let st = r32(c, S2);
                'state: {
                    if c.g[S0][0] == 0 {
                        // Unknown request: state 3.
                        c.cycles += 1;
                        Addr(st).sw(c, c.g[V0][0]);
                        break 'state;
                    }
                    c.cycles += 3;
                    let req = r32(c, S0);
                    c.g[V0][0] = Addr(req + 0x24).lbu(c);
                    if c.g[V0][0] == 0 {
                        // Not active: state 0.
                        c.cycles += 2;
                        Addr(st).sw(c, 0);
                        break 'state;
                    }
                    let stream = r32(c, S1);
                    c.g[V0][0] = Addr(stream + 0x11c).lw(c);
                    c.cycles += 2;
                    if c.g[V0][0] == c.g[S0][0] {
                        // The current request: state 2, the stream's rate.
                        c.cycles += 4;
                        c.g[V0][0] = 2;
                        Addr(st).sw(c, 2);
                        c.g[S1][0] = Addr(stream + 0x14).lhu(c);
                    } else {
                        // Queued: state 1, the next rate.
                        c.g[V0][0] = 1;
                        c.cycles += 2;
                        Addr(st).sw(c, 1);
                        c.g[S1][0] = Addr(stream + 0x18).lhu(c);
                    }
                    // Start and length in milliseconds.
                    c.cycles += 29;
                    c.g[A0][0] = Addr(req + 0x14).lw(c);
                    c.g[A1][0] = 1000;
                    SND_UMUL32X32.call(c, 0x003b_7ce0)?;
                    ms_from_product(c);
                    SND_UDIV64BY32.call(c, 0x003b_7d00)?;
                    let (st, req) = (r32(c, S2), r32(c, S0));
                    Addr(st + 4).sw(c, c.g[V0][0]);
                    c.g[A1][0] = 1000;
                    let end = Addr(req + 0x18).lw(c) as u32;
                    c.g[V0][0] = sx(end);
                    let start = Addr(req + 0x14).lw(c) as u32;
                    c.g[A0][0] = sx(end.wrapping_sub(start));
                    SND_UMUL32X32.call(c, 0x003b_7d18)?;
                    ms_from_product(c);
                    SND_UDIV64BY32.call(c, 0x003b_7d38)?;
                    let (st, req) = (r32(c, S2), r32(c, S0));
                    Addr(st + 8).sw(c, c.g[V0][0]);
                    c.g[V1][0] = 1000;
                    // st[3] = req+0x1c * 1000 / rate (32-bit).
                    c.cycles += 6;
                    c.g[V0][0] = Addr(req + 0x1c).lw(c);
                    c.mult(0, V0, V0, V1);
                    c.divu(0, V0, S1);
                    c.g[V0][0] = c.lo[0];
                    Addr(st + 0xc).sw(c, c.g[V0][0]);
                }
                c.cycles += 1;
                c.g[V0][0] = 0;
                break 'done;
            }
            c.cycles += 2;
            c.g[V0][0] = (-8i64) as u64;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The 64-bit product in v0 split into words on the stack (`dsll32`/
/// `dsrl32`, two `sw`), reloaded as one doubleword into a0 with the rate
/// (s1) in a1: `SND_UDiv64by32`'s arguments.
fn ms_from_product(c: &mut Cpu) {
    let product = c.g[V0][0];
    c.g[V1][0] = product & 0xffff_ffff;
    c.g[V0][0] = product >> 32;
    let at = sp(c);
    Addr(at).sw(c, c.g[V1][0]);
    Addr(at + 4).sw(c, c.g[V0][0]);
    c.g[A0][0] = Addr(at).ld(c);
    c.g[A1][0] = c.g[S1][0];
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, ptr, vtable, FAKE, LOG_RESULTS};
    use crate::difftest::{
        check, check_lohi, fill, fresh, put_vec, put_words, scramble, Rng, DATA, GAME_GP,
    };

    // ---- cOVStateHUD1P_DrawProgressMeter (0x20eda0) ----

    /// The HUD state (`self`), the player rider and the other racers, the
    /// world chain (`gp-0x848` -> +0x84 race manager -> +0xc race: +0x78
    /// racer count, +0x28.. racer pointers; world +0xc0 -> mode), the
    /// renderer (`gp-0x854`: vtable at +0x10d8, packet pointer at +0xe84),
    /// the HUD font (`gp-0x98c`), the meter definition (`gp-0x9a0`) and the
    /// course markers (`gp-0x998`).
    mod meter {
        /// Away from the fakes' random pointers (DATA..+1 MiB) and log.
        const B: u32 = 0x00b0_0000;
        pub const HUD: u32 = B;
        pub const PLAYER: u32 = B + 0x1000;
        pub const WORLD: u32 = B + 0x3000;
        pub const RACE_MAN: u32 = B + 0x3400;
        pub const RACE: u32 = B + 0x3800;
        pub const MODE: u32 = B + 0x3c00;
        pub const RENDERER: u32 = B + 0x4000;
        pub const PACKET: u32 = B + 0x7000;
        pub const FONT: u32 = B + 0x8000;
        pub const METER: u32 = B + 0x9000;
        pub const MARKERS: u32 = B + 0xa000;
        pub const STYLES: u32 = B + 0xb000;
        pub const RACERS: u32 = B + 0xc000;
        /// Per-racer progress entries (6 x 0x18) and the colour/style table
        /// (16-byte entries with a pointer at +0xc) in .bss.
        pub const PROGRESS: u32 = 0x004c_8bc8;
        pub const STYLE_TABLE: u32 = 0x004c_8c58;
        /// The two smoothing vectors (0x4c6868, 0x4c6878) and the bit-9 flag word.
        pub const VECS: u32 = 0x004c_6868;
        pub const FLAGS: u32 = 0x0053_08d0;
    }

    fn gp_at(off: i32) -> u32 {
        GAME_GP.wrapping_add(off as u32)
    }

    fn draw_progress_meter_setup(c: &mut Cpu, seed: u32) {
        use meter::*;
        crate::difftest::elf_image(c);
        let mut r = machine(c, seed, 0x0020_eda0, 24);
        // Early outs: HUD hidden, meter off, no player, no meter definition.
        let early = seed % 10;
        c.bus.write32(gp_at(-0x990), (early == 0) as u32);
        c.bus.write32(gp_at(-0x994), (early != 1) as u32);
        c.bus.write32(gp_at(-0x998), MARKERS);
        c.bus
            .write32(gp_at(-0x9a0), if early == 2 { 0 } else { METER });
        c.bus.write32(gp_at(-0x98c), FONT);
        c.bus.write32(gp_at(-0x848), WORLD);
        c.bus.write32(gp_at(-0x854), RENDERER);
        for off in [0x1dc4, 0x1dc8, 0x1dd8] {
            c.bus.write32(gp_at(off), r.next_u32() & 1);
        }
        c.g[A0][0] = HUD as u64;
        c.g[A1][0] = if early == 3 { 0 } else { PLAYER as u64 };
        fill(c, &mut r, HUD, 0x10);
        for i in 0..8 {
            c.bus.write32(HUD + 4 * i, r.f32(400.0));
        }
        c.bus.write32(HUD + 0x34, r.next_u32() % 4);
        // Rider vtable: slot 0x38 (the player's racer slot, a fake).
        let vt = vtable(c, &mut r, &[0x38, 0x40]);
        c.bus.write32(PLAYER + 0x6c0, vt);
        // Slot 0x40 (asked of every other racer) is a real 0/1 function.
        c.bus
            .write32(vt + 0x44, if seed % 3 == 0 { RET1 } else { RET0 });
        let slot = r.next_u32() % 6;
        c.bus.write32(LOG_RESULTS, slot);
        // World -> race manager -> race; mode flag (+0x88).
        c.bus.write32(WORLD + 0x84, RACE_MAN);
        c.bus.write32(WORLD + 0xc0, MODE);
        c.bus.write32(MODE + 0x88, r.next_u32() & 1);
        c.bus.write32(RACE_MAN + 0xc, RACE);
        c.bus.write32(RACE + 0x78, r.next_u32() % 8);
        for i in 0..8 {
            let racer = RACERS + 0x800 * i;
            c.bus.write32(RACE + 0x28 + 4 * i, racer);
            c.bus.write32(racer + 0x6c0, vt);
        }
        // Racer progress: distances around the player's so some fall
        // inside the +-5000 window and some outside.
        for i in 0..6 {
            let e = PROGRESS + 0x18 * i;
            for w in 0..5 {
                c.bus.write32(e + 4 * w, r.f32(9000.0));
            }
            c.bus.write32(e + 0x14, r.next_u32() & 1);
        }
        // Style table entries {f32, f32, f32, ptr}; the pointed-to records
        // hold a colour word and a few floats.
        for i in 0..10 {
            let e = STYLE_TABLE + 0x10 * i;
            for w in 0..3 {
                c.bus.write32(e + 4 * w, r.f32(300.0));
            }
            let rec = STYLES + 0x40 * i;
            c.bus.write32(e + 0xc, rec);
            c.bus.write32(rec, r.next_u32());
            for w in 1..8 {
                c.bus.write32(rec + 4 * w, r.f32(300.0));
            }
        }
        for w in 0..8 {
            c.bus.write32(VECS + 4 * w, r.f32(2.0));
        }
        c.bus.write32(FLAGS, r.next_u32());
        // Meter definition: +0x8 marker count, +0x10 course length.
        fill(c, &mut r, METER, 8);
        c.bus.write32(METER + 0x8, r.next_u32() % 4);
        c.bus.write32(
            METER + 0x10,
            (5000.0 + (r.next_u32() % 20000) as f32).to_bits(),
        );
        for i in 0..4 {
            c.bus.write32(MARKERS + 8 * i, r.next_u32() % 3);
            c.bus.write32(MARKERS + 8 * i + 4, r.f32(9000.0));
        }
        // Renderer: vtable slots begin/state/draw/end, GS packet buffer.
        let rvt = vtable(c, &mut r, &[0xf8, 0x100, 0x110, 0x238]);
        c.bus.write32(RENDERER + 0x10d8, rvt);
        c.bus.write32(RENDERER + 0xe84, PACKET);
        fill(c, &mut r, PACKET, 0x10);
        c.pc = 0x0020_eda0;
    }

    /// Every early out, and the full meter with 0..7 racers, markers in and
    /// out of the window, and unwinds at the renderer calls.
    #[test]
    fn c_ov_state_hud1p_draw_progress_meter_matches() {
        covered(
            ee::s05::f_0020eda0__generated,
            c_ov_state_hud1p_draw_progress_meter,
            200,
            &draw_progress_meter_setup,
        );
    }

    // ---- cAngerFX_Render (0x2d5048) ----

    /// Guest functions that just return 0 / 1 (`GameProcess_v09`,
    /// `WorldObj_v16`), for vtable slots whose result picks a branch.
    const RET0: u32 = 0x0036_0758;
    const RET1: u32 = 0x0036_0910;

    mod anger {
        const B: u32 = 0x00b0_0000;
        pub const FX: u32 = B;
        pub const RIDER: u32 = B + 0x1000;
        pub const RIDER_VT: u32 = B + 0x2000;
        pub const SKEL: u32 = B + 0x2100;
        pub const BONES: u32 = B + 0x2200;
        pub const RECORDS: u32 = B + 0x3000;
        pub const WORLD: u32 = B + 0x4000;
        pub const RACE: u32 = B + 0x4100;
        pub const CUR: u32 = B + 0x4200;
        pub const RENDERER: u32 = B + 0x5000;
        pub const PACKET: u32 = B + 0x7000;
        pub const VIEW: u32 = B + 0x7800;
    }

    /// The effect, its rider (vtable slot +0x40 a real 0/1 function), a
    /// bone matrix and a view matrix placing the bone from behind the
    /// camera to past 2000 units, records with a zero or non-zero
    /// intensity, and the renderer (fakes for +0x118, which returns the
    /// view matrix, and +0x238).
    fn anger_fx_render_setup(c: &mut Cpu, seed: u32) {
        use anger::*;
        crate::difftest::elf_image(c);
        let mut r = machine(c, seed, 0x002d_5048, 2);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), WORLD);
        c.bus.write32(GAME_GP.wrapping_sub(0x854), RENDERER);
        c.g[A0][0] = FX as u64;
        c.bus.write32(FX, RIDER);
        c.bus.write32(FX + 4, RECORDS);
        // Rider vtable slot +0x40: non-zero (skip) for some seeds.
        c.bus.write32(RIDER + 0x6c0, RIDER_VT);
        c.bus.write16(RIDER_VT + 0x40, (r.next_u32() & 0x1c) as u16);
        c.bus
            .write32(RIDER_VT + 0x44, if seed % 9 == 0 { RET1 } else { RET0 });
        // Current racer slot and its record.
        c.bus.write32(WORLD + 0x84, RACE);
        c.bus.write32(RACE + 0x84, CUR);
        let slot = r.next_u32() % 4;
        c.bus.write32(CUR + 0x14, slot);
        for i in 0..4 {
            let rec = RECORDS + 0x2c * i;
            let intensity = if seed % 7 == 1 { 0 } else { r.f32(1.0) };
            c.bus.write32(rec, intensity);
            for w in 1..5 {
                c.bus.write32(rec + 4 * w, r.f32(1.0));
            }
            c.bus.write32(rec + 0x14, r.f32(200.0));
            c.bus.write32(rec + 0x18, r.f32(200.0));
        }
        // Bone matrix translation (+0x30) of bone `rider+0x8a8`.
        let bone = r.next_u32() % 4;
        c.bus.write32(RIDER + 0x780, SKEL);
        c.bus.write32(RIDER + 0x8a8, bone);
        c.bus.write32(SKEL + 0x30, BONES);
        for b in 0..4 {
            put_vec(
                c,
                BONES + 0x40 * b + 0x30,
                [r.f32(500.0), r.f32(500.0), r.f32(500.0), 1.0f32.to_bits()],
            );
        }
        // View matrix rows; the translation row's z spreads the view depth
        // over -1000 .. 3000 (behind, near, fading, too far).
        for row in 0..3 {
            put_vec(
                c,
                VIEW + 0x10 * row,
                [r.f32(1.0), r.f32(1.0), r.f32(0.5), 0],
            );
        }
        let depth = -1000.0 + (r.next_u32() % 4000) as f32;
        put_vec(
            c,
            VIEW + 0x30,
            [
                r.f32(100.0),
                r.f32(100.0),
                depth.to_bits(),
                1.0f32.to_bits(),
            ],
        );
        // Renderer: +0x118 returns the view matrix (first fake call).
        let rvt = vtable(c, &mut r, &[0x118, 0x238]);
        c.bus.write32(RENDERER + 0x10d8, rvt);
        c.bus.write32(RENDERER + 0xe84, PACKET);
        c.bus.write32(RENDERER + 0xfac, r.next_u32());
        fill(c, &mut r, PACKET, 0x10);
        c.bus.write32(LOG_RESULTS, VIEW);
        c.pc = 0x002d_5048;
    }

    /// Skipped by the rider query, zero intensity, behind the camera, too
    /// far, fading and full glow; unwinds at both renderer calls.
    #[test]
    fn c_anger_fx_render_matches() {
        covered(
            ee::s09::f_002d5048__generated,
            c_anger_fx_render,
            240,
            &anger_fx_render_setup,
        );
    }

    // ---- Collide_VolumeObject (0x334888) ----

    mod volume {
        const B: u32 = 0x00b0_0000;
        pub const QUERY: u32 = B;
        pub const QUERY_VT: u32 = B + 0x100;
        pub const NODE: u32 = B + 0x1000;
        pub const INST: u32 = B + 0x1400;
        pub const INST_VT: u32 = B + 0x1500;
        pub const TYPEREC: u32 = B + 0x2000;
        pub const SECTION: u32 = B + 0x2400;
        pub const TABLE: u32 = B + 0x2500;
        pub const LIST: u32 = B + 0x3000;
        pub const PARTS: u32 = B + 0x3100;
        pub const SHAPES: u32 = B + 0x3400;
        pub const MATRICES: u32 = B + 0x4000;
        pub const OUT: u32 = B + 0x8000;
        pub const WOBJ: u32 = B + 0x6000;
    }

    /// Write vtable slot `off` = `{0, f}`.
    fn slot(c: &mut Cpu, vt: u32, off: u32, f: u32) {
        c.bus.write16(vt + off, 0);
        c.bus.write32(vt + off + 4, f);
    }

    /// A query whose tests (+0x30/+0x38/+0x48) are real 0/1 functions (one
    /// contact at most per part) and whose load/close (+0x8/+0x10) are
    /// fakes; a node with or without an instance (own-collision, animated
    /// or single-matrix), collision type 0..4, 0..3 parts with bone or no
    /// bone (-1), with or without a shape; the world object (`gp+0x16c8`)
    /// whose section tables lead to empty meshes (`Collide_MeshLeafTriangles`
    /// returns at once).
    fn collide_volume_object_setup(c: &mut Cpu, seed: u32) {
        use volume::*;
        let mut r = machine(c, seed, 0x0033_4888, 8);
        let pick = |r: &mut Rng| if r.next_u32() % 3 == 0 { RET0 } else { RET1 };
        c.bus.write32(QUERY + 0x50, QUERY_VT);
        slot(c, QUERY_VT, 0x8, FAKE);
        slot(c, QUERY_VT, 0x10, FAKE + 0x10);
        for off in [0x30, 0x38, 0x48] {
            let f = pick(&mut r);
            slot(c, QUERY_VT, off, f);
        }
        // Instance: own collision (+0x130), animated (+0xd0).
        c.bus
            .write32(NODE + 0xc, if seed % 4 == 0 { 0 } else { INST });
        c.bus.write32(INST + 0xc, INST_VT);
        slot(c, INST_VT, 0x130, if seed % 5 == 1 { RET1 } else { RET0 });
        slot(c, INST_VT, 0x138, FAKE + 0x20);
        let animated = pick(&mut r);
        slot(c, INST_VT, 0xd0, animated);
        slot(c, INST_VT, 0xe0, FAKE + 0x30);
        slot(c, INST_VT, 0xc8, FAKE + 0x40);
        for i in 0..16 {
            c.bus.write32(NODE + 0x10 + 4 * i, r.f32(4.0));
        }
        // Collision type and per-part surface records (+0x1a: -1 or not).
        c.bus.write32(NODE + 0x88, TYPEREC);
        c.bus.write32(TYPEREC, r.next_u32() % 5);
        c.bus.write32(TYPEREC + 0xc, SECTION);
        for i in 0..8 {
            let h = if r.next_u32() & 1 == 0 {
                0xffff
            } else {
                r.next_u32() & 0x7fff
            };
            c.bus.write16(TYPEREC + 0x1a + 0xc * i, h as u16);
            c.bus.write16(TYPEREC + 0x16 + 0xc * i, r.next_u32() as u16);
        }
        // Section data -> table of (empty) meshes / sphere trees.
        c.bus.write32(SECTION + 0x8, TABLE);
        c.bus.write32(GAME_GP + 0x16c8, WOBJ);
        // Parts.
        c.bus.write32(NODE + 0x80, LIST);
        c.bus.write32(NODE + 0x84, r.f32(2.0));
        c.bus.write32(LIST + 4, r.next_u32() % 4);
        c.bus.write32(LIST + 8, PARTS);
        for i in 0..4 {
            let part = PARTS + 0x10 * i;
            let bone = if r.next_u32() % 3 == 0 {
                u32::MAX
            } else {
                r.next_u32() % 24
            };
            c.bus.write32(part, bone);
            let shape = SHAPES + 0x40 * i;
            c.bus
                .write32(part + 4, if r.next_u32() % 4 == 0 { 0 } else { shape });
            c.bus.write32(part + 0xc, MATRICES + 0x40 * i);
            fill(c, &mut r, shape, 0x10);
            for w in 0..16 {
                c.bus.write32(MATRICES + 0x40 * i + 4 * w, r.f32(4.0));
            }
        }
        c.g[A0][0] = QUERY as u64;
        c.g[A1][0] = NODE as u64;
        c.g[A2][0] = OUT as u64;
        c.g[A3][0] = 0x40;
        c.g[T0][0] = (r.next_u32() % 4) as u64;
        c.pc = 0x0033_4888;
    }

    /// Own-collision instances, rejected types, single and animated
    /// matrices, parts with and without a bone or shape, every query test
    /// result, contacts copied out; unwinds at the fake calls.
    #[test]
    fn collide_volume_object_matches() {
        covered(
            ee::s11::f_00334888__generated,
            collide_volume_object,
            240,
            &collide_volume_object_setup,
        );
    }

    // ---- SND_EaxaDecode (0x3ccf90) ----

    /// A decoder state with nothing left, or with 0..300 samples left;
    /// pending samples none, some, or negative (a partial block); a request
    /// of 0..200 samples (whole blocks and a remainder); random ADPCM bytes
    /// for the real `SND_EaxaDecodeBlocks` (coefficient table from the ELF).
    fn snd_eaxa_decode_setup(c: &mut Cpu, seed: u32) {
        const S: u32 = 0x00b0_0000;
        const POUT: u32 = S + 0x1000;
        const OUT: u32 = S + 0x1100;
        const READ: u32 = S + 0x2000;
        const SRC: u32 = S + 0x3000;
        crate::difftest::elf_image(c);
        let mut r = Rng(0x3ccf_9000 ^ seed as u64);
        scramble(c, &mut r);
        let remaining = if seed % 6 == 0 { 0 } else { r.next_u32() % 300 };
        let pending = match seed % 4 {
            0 => 0,
            1 => 0u32.wrapping_sub(r.next_u32() % 28),
            _ => 1 + r.next_u32() % 27,
        };
        c.bus.write32(S + 0x4, pending);
        c.bus.write32(S + 0x8, remaining);
        c.bus.write32(S + 0x10, READ);
        c.bus.write32(S + 0x98, r.f32(1.0));
        c.bus.write32(S + 0x9c, r.f32(1.0));
        c.bus.write32(S + 0xa0, SRC);
        fill(c, &mut r, READ, 32);
        fill(c, &mut r, SRC, 0x100);
        c.bus.write32(POUT, OUT);
        c.g[A0][0] = S as u64;
        c.g[A1][0] = POUT as u64;
        c.g[A2][0] = (r.next_u32() % 200) as u64;
        c.pc = 0x003c_cf90;
    }

    #[test]
    fn snd_eaxa_decode_matches() {
        check_lohi(
            ee::s14::f_003ccf90__generated,
            snd_eaxa_decode,
            300,
            &snd_eaxa_decode_setup,
        );
    }

    // ---- Rider_HandleBoostInput (0x114130) ----

    /// A rider with a meter that is empty, negative or 0..1 (every level
    /// band), a level of 0 (first tick) or running, every drain condition,
    /// and `held`/`pressed` both ways. The audio calls are real and take
    /// their short path (no world in game, not a human rider).
    fn rider_handle_boost_input_setup(c: &mut Cpu, seed: u32) {
        const RIDER: u32 = 0x00b0_0000;
        const PAD: u32 = RIDER + 0x1000;
        const WORLD: u32 = RIDER + 0x2000;
        const AUDIO: u32 = RIDER + 0x3000;
        let mut r = Rng(0x0011_4130 ^ (seed as u64) << 4);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        put_words(
            c,
            GAME_GP - 0x7bfc,
            &[0x3f2a_aaab, 0x3eaa_aaab, 0x3ac5_7fcc],
        );
        c.bus.write32(GAME_GP - 0x848, WORLD);
        c.bus.write32(GAME_GP + 0x410, AUDIO);
        c.bus.write32(RIDER + 0x77c, PAD);
        c.bus.write32(RIDER + 0x874, 0);
        let meter = match seed % 6 {
            0 => 0,
            1 => (-0.5f32).to_bits(),
            2 => 0.2f32.to_bits(),
            3 => 0.5f32.to_bits(),
            _ => 0.9f32.to_bits(),
        };
        c.bus.write32(RIDER + 0x2f8, meter);
        let level = if (seed / 6) % 2 == 0 {
            0
        } else {
            0.625f32.to_bits()
        };
        c.bus.write32(RIDER + 0x2fc, level);
        let hold = if r.next_u32() % 3 == 0 { r.f32(1.0) } else { 0 };
        c.bus.write32(RIDER + 0x2e8, hold);
        c.bus.write32(RIDER + 0x304, r.next_u32() & 1);
        c.bus.write32(RIDER + 0x2f4, r.next_u32() % 16);
        c.g[A0][0] = RIDER as u64;
        c.g[A1][0] = ((seed / 12) % 2) as u64;
        c.g[A2][0] = ((seed / 24) % 2) as u64;
        c.pc = 0x0011_4130;
    }

    #[test]
    fn rider_handle_boost_input_matches() {
        check(
            ee::s00::f_00114130__generated,
            rider_handle_boost_input,
            288,
            &rider_handle_boost_input_setup,
        );
    }

    // ---- Rider_UpdateImpacts (0x1210b0) ----

    /// A zeroed object from the fakes' pointer pool.
    fn zobj(r: &mut Rng) -> u32 {
        ptr(r)
    }

    /// The game image plus back-end singletons and libraries whose
    /// interfaces are fakes (as the rider-state tests build it), so the
    /// real reset path's sound/replay calls stop at a fake.
    fn game_world(c: &mut Cpu, r: &mut Rng) {
        crate::difftest::elf_image(c);
        for off in [0x1f04, 0x1f28] {
            let singleton = zobj(r);
            c.bus.write32(GAME_GP - off, singleton);
        }
        c.bus.write32(GAME_GP - 0x1efc, 0);
        for i in 0..8 {
            let lib = zobj(r);
            c.bus.write32(0x0053_08a8 + 4 * i, lib);
            let vt = vtable(c, r, &[8]);
            c.bus.write32(lib + 4, vt);
        }
    }

    /// A rider with impact accumulators below and above the reset limits,
    /// an owner in main state 1, 2 (handler state 1 or not) or a state past
    /// the state tables, an air predictor in every mode with falls below
    /// and above 45 units; the boost-timer and leg-IK callees run for real
    /// (their vtable slots are fakes, the animation table comes from the
    /// game image).
    fn rider_update_impacts_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x0012_10b0, 8);
        game_world(c, &mut r);
        let rider = zobj(&mut r);
        c.g[A0][0] = rider as u64;
        put_vec(
            c,
            rider + 0x3e0,
            [r.f32(10.0), r.f32(10.0), r.f32(10.0), r.f32(1.0)],
        );
        let big = |x: bool| (if x { 10.0f32 } else { 1.0f32 }).to_bits();
        c.bus.write32(rider + 0x3f0, big(seed % 5 == 0));
        c.bus.write32(rider + 0x3f4, big(seed % 3 == 0));
        // Owner: main state 1, 2 or out of range; handler state 1 or not.
        let owner = zobj(&mut r);
        c.bus.write32(rider + 0x77c, owner);
        let main = match (seed / 5) % 4 {
            0 => 1,
            1 | 2 => 2,
            _ => 0xe + r.next_u32() % 8,
        };
        c.bus.write32(owner + 0xde0, main);
        c.bus.write32(owner + 0xde4, 0xe + r.next_u32() % 8);
        c.bus.write32(owner + 0x30, (seed / 20) % 2);
        // Air predictor: modes 0..4, falls of 0..90 units.
        let air = zobj(&mut r);
        c.bus.write32(rider + 0x788, air);
        c.bus.write32(air + 0xac, r.next_u32() % 5);
        c.bus.write32(air + 0xa0, 0);
        c.bus
            .write32(air + 0x98, ((r.next_u32() % 90) as f32).to_bits());
        // Boost timers and leg IK: vtable at +0x6c0, frame time, animation.
        let vt = vtable(c, &mut r, &[0x48]);
        c.bus.write32(rider + 0x6c0, vt);
        c.bus.write32(rider + 0x300, 0.016_666_668f32.to_bits());
        for off in [0x2f0u32, 0x2f8, 0x2fc, 0x318] {
            c.bus.write32(rider + off, r.f32(2.0));
        }
        c.bus.write32(rider + 0x2f4, r.next_u32() % 12);
        c.bus.write32(rider + 0x304, r.next_u32() % 5);
        let anim = zobj(&mut r);
        c.bus.write32(rider + 0x784, anim);
        c.bus.write32(anim + 8, (r.next_u32() % 4) * 4);
        c.pc = 0x0012_10b0;
    }

    #[test]
    fn rider_update_impacts_matches() {
        check(
            ee::s00::f_001210b0__generated,
            rider_update_impacts,
            240,
            &rider_update_impacts_setup,
        );
    }

    // ---- tWPIGD_Lighting_Blend (0x2bd5a8) ----

    /// Lighting blocks with random vectors and parameters (sometimes
    /// unaligned by 4, as the doubleword pairs allow), a blend weight from
    /// the edge cases, and the owner notification a fake (unwinding at
    /// either call).
    fn twpigd_lighting_blend_setup(c: &mut Cpu, seed: u32) {
        const B: u32 = 0x00b0_0000;
        let mut r = machine(c, seed, 0x002b_d5a8, 2);
        let this = B + 4 * (seed % 2);
        let p = B + 0x100 + 4 * ((seed / 2) % 2);
        for i in 0..0x18 {
            c.bus.write32(this + 4 * i, r.f32(4.0));
            c.bus.write32(p + 4 * i, r.f32(4.0));
        }
        let vt = vtable(c, &mut r, &[0x218]);
        c.bus.write32(this + 4, vt);
        c.g[A0][0] = this as u64;
        c.g[A1][0] = p as u64;
        c.f[12] = if seed % 3 == 0 {
            r.edge_f32()
        } else {
            r.f32(1.0)
        };
        c.pc = 0x002b_d5a8;
    }

    #[test]
    fn twpigd_lighting_blend_matches() {
        covered(
            ee::s08::f_002bd5a8__generated,
            twpigd_lighting_blend,
            200,
            &twpigd_lighting_blend_setup,
        );
    }

    // ---- SndLoadQueue_Update (0x28aaf8) ----

    mod load_queue {
        const B: u32 = 0x00b0_0000;
        pub const QUEUE: u32 = B;
        pub const RINGS: u32 = B + 0x400;
        pub const FILES: u32 = B + 0x800;
        pub const BUF: u32 = B + 0x1000;
        /// `AsyncFile` handle table: count (0x519bf0) and table pointer
        /// (0x450c34, 0x30-byte entries: +0 handle, +0x8 size, +0x10
        /// error, +0x1c pending request, +0x2c end).
        pub const FILE_COUNT: u32 = 0x0051_9bf0;
        pub const FILE_TABLE: u32 = 0x0045_0c34;
    }

    /// A queue that is idle, or running a request of kind 0..3 or none;
    /// rings holding one entry or empty; a handle that is unknown (the read
    /// failed), still pending, or done with no data or a small zeroed
    /// buffer; the owner's allocate/notify slots are fakes. The mutex and
    /// thread syscalls land in the fakes too. One-entry rings keep
    /// `SndLoadQueue_StartNext` from opening another file.
    fn snd_load_queue_update_setup(c: &mut Cpu, seed: u32) {
        use load_queue::*;
        let mut r = machine(c, seed, 0x0028_aaf8, 6);
        let slot = r.next_u32() % 4;
        let handle = 0x100 | slot;
        c.bus
            .write32(QUEUE, if seed % 7 == 0 { 0x1ff } else { handle });
        c.bus.write32(QUEUE + 4, (seed % 9 != 0) as u32);
        // Rings: +0x18c ids, +0x19c name slots, +0x1ac kinds, +0x1bc params.
        for (i, off) in [0x18cu32, 0x19c, 0x1ac, 0x1bc].into_iter().enumerate() {
            let data = RINGS + 0x40 * i as u32;
            let empty = match off {
                0x1ac => seed % 11 == 0,
                0x18c => false,
                _ => r.next_u32() % 4 == 0,
            };
            c.bus.write32(QUEUE + off, 4);
            c.bus.write32(QUEUE + off + 4, data);
            let tail = r.next_u32() % 4;
            c.bus
                .write32(QUEUE + off + 8, if empty { tail } else { (tail + 1) % 4 });
            c.bus.write32(QUEUE + off + 0xc, tail);
            let value = match off {
                0x1ac => (seed / 2) % 4,
                0x19c => r.next_u32() % 4,
                _ => r.next_u32() % 64,
            };
            c.bus.write32(data + 4 * tail, value);
        }
        fill(c, &mut r, QUEUE + 0xc, 0x40);
        let vt = vtable(c, &mut r, &[0x18, 0x28]);
        c.bus.write32(QUEUE + 0x1d4, vt);
        // The file: pending, or done with 0 or 0x40 bytes.
        c.bus.write32(FILE_COUNT, 4);
        c.bus.write32(FILE_TABLE, FILES);
        let entry = FILES + 0x30 * slot;
        c.bus.write32(entry, handle);
        let size = if (seed / 3) % 4 == 0 { 0x40 } else { 0 };
        c.bus.write32(entry + 0x8, size);
        c.bus.write32(entry + 0x10, 0);
        c.bus.write32(entry + 0x1c, ((seed / 5) % 4 == 0) as u32);
        c.bus.write32(entry + 0x2c, BUF + size);
        c.g[A0][0] = QUEUE as u64;
        c.pc = 0x0028_aaf8;
    }

    #[test]
    fn snd_load_queue_update_matches() {
        check(
            ee::s07::f_0028aaf8__generated,
            snd_load_queue_update,
            300,
            &snd_load_queue_update_setup,
        );
    }

    // ---- Collide_SphereVsBoxEdges (0x329590) ----

    /// Boxes around the origin, centres inside and outside them on every
    /// axis, radii 0..5 (some exactly on a face), the axis unit vectors
    /// in .bss and the 1e30 `.lit4` limits.
    fn collide_sphere_vs_box_edges_setup(c: &mut Cpu, seed: u32) {
        const B: u32 = 0x00b0_0000;
        let mut r = Rng(0x0032_9590 ^ (seed as u64) << 3);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        put_words(c, GAME_GP - 0x2df0, &[0x7149_f2ca, 0x7149_f2ca]);
        let one = 1.0f32.to_bits();
        put_vec(c, 0x004f_f140, [one, 0, 0, 0]);
        put_vec(c, 0x004f_f150, [0, one, 0, 0]);
        put_vec(c, 0x004f_f160, [0, 0, one, 0]);
        let coord = |r: &mut Rng, lo: f32, hi: f32| {
            (lo + (r.next_u32() % 1000) as f32 * (hi - lo) / 1000.0).to_bits()
        };
        for i in 0..3 {
            c.bus.write32(B + 4 * i, coord(&mut r, -10.0, 0.0));
            c.bus.write32(B + 0x10 + 4 * i, coord(&mut r, 0.0, 10.0));
            // Mostly inside the box so the face tests are reached.
            let centre = if r.next_u32() % 4 == 0 {
                coord(&mut r, -15.0, 15.0)
            } else {
                coord(&mut r, -9.0, 9.0)
            };
            c.bus.write32(B + 0x20 + 4 * i, centre);
        }
        c.bus.write32(B + 0x2c, one);
        fill(c, &mut r, B + 0x30, 0x10);
        c.g[A0][0] = B as u64;
        c.g[A1][0] = (B + 0x10) as u64;
        c.g[A2][0] = (B + 0x20) as u64;
        c.g[A3][0] = (B + 0x30) as u64;
        c.g[T0][0] = (B + 0x40) as u64;
        c.g[T1][0] = (B + 0x50) as u64;
        c.f[12] = if seed % 10 == 0 {
            0
        } else {
            coord(&mut r, 0.0, 5.0)
        };
        c.pc = 0x0032_9590;
    }

    #[test]
    fn collide_sphere_vs_box_edges_matches() {
        check(
            ee::s10::f_00329590__generated,
            collide_sphere_vs_box_edges,
            300,
            &collide_sphere_vs_box_edges_setup,
        );
    }

    // ---- SNDSTRM_GetRequestStatus (0x3b7c40) ----

    /// Four stream slots (0x50a0a0, count byte 0x50a92e), some empty; each
    /// stream with a two-request list (+0x11c) and non-zero rates
    /// (+0x14/+0x18); handles that are negative, name a missing stream,
    /// an unknown request, or the current or queued request, active or
    /// not, with random positions.
    fn sndstrm_get_request_status_setup(c: &mut Cpu, seed: u32) {
        const B: u32 = 0x00b0_0000;
        const STREAM_COUNT: u32 = 0x0050_a92e;
        const STREAMS: u32 = 0x0050_a0a0;
        let mut r = Rng(0x003b_7c40 ^ (seed as u64) << 5);
        scramble(c, &mut r);
        c.bus.write8(STREAM_COUNT, 4);
        for i in 0..4u32 {
            let stream = B + 0x1000 * i;
            c.bus
                .write32(STREAMS + 4 * i, if i == 3 { 0 } else { stream });
            c.bus
                .write16(stream + 0x14, (1 + r.next_u32() % 48000) as u16);
            c.bus
                .write16(stream + 0x18, (1 + r.next_u32() % 48000) as u16);
            // Two requests: handles i | 0x100 and i | 0x200.
            let (first, second) = (stream + 0x200, stream + 0x300);
            c.bus.write32(stream + 0x11c, first);
            c.bus.write32(first, second);
            c.bus.write32(second, 0);
            for (req, h) in [(first, 0x100u32), (second, 0x200)] {
                c.bus.write32(req + 0xc, i | h);
                c.bus.write8(req + 0x24, (r.next_u32() % 3 != 0) as u8);
                c.bus.write32(req + 0x14, r.next_u32() % 0x10_0000);
                c.bus.write32(req + 0x18, r.next_u32() % 0x10_0000);
                c.bus.write32(req + 0x1c, r.next_u32() % 0x1_0000);
            }
        }
        let stream = r.next_u32() % 4;
        let handle = match seed % 5 {
            0 => 0x8000_0000 | r.next_u32(),
            1 => stream | 0x300,
            2 => stream | 0x100,
            _ => stream | 0x200,
        };
        c.g[A0][0] = handle as i32 as i64 as u64;
        c.g[A1][0] = (B + 0x8000) as u64;
        fill(c, &mut r, B + 0x8000, 4);
        c.pc = 0x003b_7c40;
    }

    #[test]
    fn sndstrm_get_request_status_matches() {
        check_lohi(
            ee::s13::f_003b7c40__generated,
            sndstrm_get_request_status,
            300,
            &sndstrm_get_request_status_setup,
        );
    }
}
