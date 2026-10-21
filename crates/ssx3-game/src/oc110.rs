//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements, each with a differential test against the generated body.
//!
//! - **RiderModel_BlendAnimPose** (`0x0030f2b0`): one frame of animation-pose
//!   blending for a rider model. It walks the model's bone list, matches each
//!   bone to its animation channels (through the global channel table at
//!   `gp+0xd08`), samples the channel curves through
//!   `AnimDataBase_SampleChannel` (`0x00311318`) and blends the sampled pose
//!   into the per-bone pose arrays in the frame, then resolves the final
//!   bone matrices (VU0 rotations and quaternions included).
//!
//! The generated body is the authority for registers, cycles and call order;
//! the reference decompilation has only assembly for this function. The body
//! follows the original's block structure line for line, so every register the
//! generated code leaves behind (including the scratch values in `v0`/`v1` and
//! the VU0 VF/ACC/Q state) is reproduced; the scalar float maths uses the FPU
//! ops, and every VU0 operation is the original's macro op on `c.vu0()`.
//!
//! Frame slots named below (all `sp`-relative, frame size `0x4e00`):
//! `0x4c80` bone count (`a1`), `0x4c84` animation-state array (`a2`),
//! `0x4c88` mask argument (`a3`), `0x4c8c` channel table (`*(gp+0xd08)`),
//! `0x4c90` layer count (1 + the layers that blended this frame),
//! `0x4c94` reference blend value, `0x4c98`/`0x4c9c`/`0x4ca0`/`0x4ca4` loop
//! cursors, `0x4ca8` address of the per-layer weight buffer (`sp+0x848`),
//! `0x620`/`0x720` two `0x100`-byte scratch pose buffers, `0x820` a `0x20`-byte
//! flag buffer, `0x840`/`0x848` the per-layer blend masks.

use crate::call::{call_ptr, guest, sp, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0030_f2b0,
    rider_model_blend_anim_pose,
    "RiderModel_BlendAnimPose",
)];
pub fn rider_model_blend_anim_pose(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x4e00);
        'b_30f690: {
            c.cycles += 42;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xd08));
            frame.save(c, 21, 0x4d90);
            c.sw(frame.at(c, 0x4c84), c.g[A2][0]);
            c.g[S5][0] = c.g[A0][0];
            c.sw(frame.at(c, 0x4c80), c.g[A1][0]);
            c.g[A0][0] = sx(frame.at(c, 0x620));
            frame.save(c, 23, 0x4d70);
            c.g[A1][0] = 0u64;
            c.sw(frame.at(c, 0x4c8c), c.g[V0][0]);
            c.g[A2][0] = 0x100;
            frame.save(c, 16, 0x4de0);
            c.g[S7][0] = 0u64;
            frame.save(c, 17, 0x4dd0);
            frame.save(c, 18, 0x4dc0);
            frame.save(c, 19, 0x4db0);
            frame.save(c, 20, 0x4da0);
            frame.save(c, 22, 0x4d80);
            frame.save(c, 30, 0x4d60);
            frame.save_ra(c, 0x4d50);
            frame.save_fpr(c, 20, 0x4df0);
            c.g[RA][0] = sx(0x0030f30c);
            c.sw(frame.at(c, 0x4c88), c.g[A3][0]);
            call_ptr(c, 0x003e6448, 0x0030f30c)?;
            c.g[V1][0] = 0x1;
            c.g[A0][0] = sx(frame.at(c, 0x720));
            c.sw(frame.at(c, 0x4c90), c.g[V1][0]);
            c.g[A1][0] = 0u64;
            c.g[RA][0] = sx(0x0030f324);
            c.g[A2][0] = 0x100;
            call_ptr(c, 0x003e6448, 0x0030f324)?;
            c.g[A0][0] = sx(frame.at(c, 0x820));
            c.g[A2][0] = 0x20;
            c.g[RA][0] = sx(0x0030f334);
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x003e6448, 0x0030f334)?;
            c.g[V0][0] = 0xffffffffffffffff;
            c.sw(frame.at(c, 0x948), 0u64);
            c.sd(frame.at(c, 0x840), c.g[V0][0]);
            c.sw(frame.at(c, 0x94c), 0u64);
            c.g[A0][0] = c.lw(frame.at(c, 0x4c84));
            c.g[A2][0] = c.lw(frame.at(c, 0x4c80));
            c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
            let t = (c.g[A2][0] as i64) <= 0;
            c.sw(frame.at(c, 0x4c94), c.g[A0][0]);
            if t {
                break 'b_30f690;
            } else {
                c.cycles += 6;
                c.g[V0][0] = sx(frame.at(c, 0x848));
                c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x323c));
                c.sw(frame.at(c, 0x4ca8), c.g[V0][0]);
                c.g[T7][0] = 0x110;
                c.g[T6][0] = 0x110;
                c.g[V1][0] = c.lw(frame.at(c, 0x4c84));
                'l_30f370: loop {
                    'b_30f680: {
                        'b_30f67c: {
                            'b_30f3cc: {
                                c.cycles += 8;
                                c.g[V0][0] = 0x50;
                                c.lo[0] = c.g[V1][0];
                                c.madd(0, 11, 23, 2);
                                c.lwc1(0, (c.g[T3][0] as u32).wrapping_add(0xc));
                                c.c_lt_s(0, 20);
                                let t = !c.fcc();
                                c.g[A0][0] = c.lw(frame.at(c, 0x4c90));
                                if t {
                                    break 'b_30f3cc;
                                } else {
                                    c.cycles += 2;
                                    let t = (c.g[A0][0] as i64) <= 0;
                                    c.g[T9][0] = sx((c.g[S7][0] as u32).wrapping_add(0x1));
                                    if t {
                                        break 'b_30f67c;
                                    } else {
                                        c.cycles += 4;
                                        c.g[A2][0] = c.lw(frame.at(c, 0x4ca8));
                                        c.g[V0][0] = sx((c.g[S7][0] as u32) << 2);
                                        c.g[V1][0] = c.g[A0][0];
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[A2][0] as u32)));
                                        'l_30f3a8: loop {
                                            c.cycles += 7;
                                            c.sw((c.g[V0][0] as u32), 0u64);
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                            let t = c.g[V1][0] != 0u64;
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x110));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x0030f3a8);
                                                }
                                                continue 'l_30f3a8;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[V1][0] = c.lw(frame.at(c, 0x4c80));
                                                if t {
                                                    break 'b_30f680;
                                                } else {
                                                    break 'b_30f3cc;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            'b_30f438: {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x8));
                                c.g[V1][0] = c.lw(frame.at(c, 0x4c94));
                                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[T9][0] = sx((c.g[S7][0] as u32).wrapping_add(0x1));
                                if t {
                                    break 'b_30f438;
                                } else {
                                    'b_30f430: {
                                        c.cycles += 3;
                                        c.g[A0][0] = c.lw(frame.at(c, 0x4c90));
                                        let t = (c.g[A0][0] as i64) <= 0;
                                        c.g[V0][0] = sx(frame.at(c, 0x840));
                                        if t {
                                            break 'b_30f430;
                                        } else {
                                            c.cycles += 3;
                                            c.g[AT][0] = 0x3f800000;
                                            c.mtc1(1, 3);
                                            c.g[V1][0] = c.g[A0][0];
                                            'l_30f3f8: loop {
                                                'b_30f420: {
                                                    c.cycles += 9;
                                                    c.lwc1(
                                                        0,
                                                        (c.g[V0][0] as u32).wrapping_add(0x108),
                                                    );
                                                    c.lwc1(
                                                        2,
                                                        (c.g[V0][0] as u32).wrapping_add(0x10c),
                                                    );
                                                    c.sub_s(1, 3, 0);
                                                    c.mul_s(1, 1, 2);
                                                    c.add_s(0, 0, 1);
                                                    c.c_lt_s(3, 0);
                                                    let t = !c.fcc();
                                                    c.swc1(
                                                        0,
                                                        (c.g[V0][0] as u32).wrapping_add(0x108),
                                                    );
                                                    if t {
                                                        break 'b_30f420;
                                                    } else {
                                                        c.cycles += 1;
                                                        c.swc1(
                                                            3,
                                                            (c.g[V0][0] as u32).wrapping_add(0x108),
                                                        );
                                                        break 'b_30f420;
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.sw((c.g[V0][0] as u32).wrapping_add(0x10c), 0u64);
                                                c.g[V1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                                let t = c.g[V1][0] != 0u64;
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x110));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x0030f3f8);
                                                    }
                                                    continue 'l_30f3f8;
                                                } else {
                                                    break 'b_30f430;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    c.g[A2][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x8));
                                    c.sw(frame.at(c, 0x4c94), c.g[A2][0]);
                                    break 'b_30f438;
                                }
                            }
                            'b_30f4e4: {
                                c.cycles += 16;
                                c.g[A0][0] = c.lw((c.g[T3][0] as u32));
                                c.g[V0][0] = 0x14;
                                c.g[A2][0] = c.lw(frame.at(c, 0x4c8c));
                                c.g[T1][0] = 0u64;
                                c.g[V1][0] = c.g[A0][0] & 0xff;
                                c.g[T5][0] = c.ld((c.g[S5][0] as u32).wrapping_add(0x150));
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[A0][0] = sx((c.g[A0][0] as u32) >> 8);
                                c.g[V1][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                c.mult(0, 4, 4, 2);
                                c.g[V1][0] = c.lw((c.g[V1][0] as u32));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[A1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                                let t = c.g[A1][0] == 0u64;
                                c.g[T2][0] = c.ld((c.g[T3][0] as u32).wrapping_add(0x10));
                                if t {
                                    break 'b_30f4e4;
                                } else {
                                    c.cycles += 10;
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                                    c.g[A2][0] = c.g[A1][0];
                                    c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                    c.g[T4][0] = 0x58;
                                    c.g[A0][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0xa));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                                    c.g[T0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x14));
                                    c.g[A3][0] = sx((c.g[A0][0] as u32) << 3);
                                    c.g[A1][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    'l_30f4a0: loop {
                                        'b_30f4dc: {
                                            c.cycles += 4;
                                            c.g[A0][0] = c.lbu((c.g[A1][0] as u32));
                                            c.g[V0][0] =
                                                ((c.g[T0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[A2][0] =
                                                    sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                                                break 'b_30f4dc;
                                            } else {
                                                'b_30f4d8: {
                                                    c.cycles += 6;
                                                    c.g[V1][0] = c
                                                        .lw((c.g[S5][0] as u32).wrapping_add(0x1c));
                                                    c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add((c.g[V1][0] as u32)));
                                                    c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                                                    let t = (c.g[V0][0] as i64) < 0;
                                                    c.mult(0, 4, 2, 12);
                                                    if t {
                                                        break 'b_30f4d8;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S5][0] as u32)
                                                                .wrapping_add(0xc));
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add((c.g[A0][0] as u32)));
                                                        c.g[V1][0] = c
                                                            .ld((c.g[V0][0] as u32)
                                                                .wrapping_add(0x10));
                                                        c.g[T1][0] = c.g[T1][0] | c.g[V1][0];
                                                        break 'b_30f4d8;
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[A2][0] =
                                                    sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                                                break 'b_30f4dc;
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = c.g[A2][0] != 0u64;
                                        c.g[A1][0] =
                                            sx((c.g[A1][0] as u32)
                                                .wrapping_add((c.g[A3][0] as u32)));
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x0030f4a0);
                                            }
                                            continue 'l_30f4a0;
                                        } else {
                                            break 'b_30f4e4;
                                        }
                                    }
                                }
                            }
                            'b_30f4fc: {
                                c.cycles += 4;
                                c.g[T1][0] = c.g[T1][0] & c.g[T5][0];
                                c.g[V0][0] = c.lw(frame.at(c, 0x4c88));
                                let t = c.g[V0][0] == 0u64;
                                c.g[T1][0] = c.g[T1][0] & c.g[T2][0];
                                if t {
                                    break 'b_30f4fc;
                                } else {
                                    c.cycles += 2;
                                    c.g[V0][0] = c.ld((c.g[S5][0] as u32).wrapping_add(0x158));
                                    c.g[T1][0] = c.g[T1][0] & c.g[V0][0];
                                    break 'b_30f4fc;
                                }
                            }
                            c.cycles += 3;
                            c.g[V1][0] = c.lw(frame.at(c, 0x4c90));
                            let t = (c.g[V1][0] as i64) <= 0;
                            c.g[A0][0] = 0x110;
                            if t {
                                break 'b_30f67c;
                            } else {
                                c.cycles += 10;
                                c.g[FP][0] = sx(frame.at(c, 0x840));
                                c.mult(0, 7, 3, 4);
                                c.g[A2][0] = c.lw(frame.at(c, 0x4ca8));
                                c.g[S0][0] = c.g[FP][0];
                                c.g[T0][0] = sx(frame.at(c, 0x848));
                                c.g[T4][0] = c.g[T0][0];
                                c.g[T2][0] = 0u64;
                                c.g[FP][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add((c.g[T6][0] as u32)));
                                c.g[S1][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add((c.g[T7][0] as u32)));
                                c.g[S6][0] = c.g[V1][0];
                                'l_30f530: loop {
                                    'b_30f668: {
                                        'b_30f55c: {
                                            c.cycles += 5;
                                            c.g[V0][0] = c.ld((c.g[S0][0] as u32));
                                            c.g[V1][0] = !(0u64 | c.g[T1][0]);
                                            c.g[S2][0] = c.g[V0][0] & c.g[T1][0];
                                            let t = c.g[S2][0] != 0u64;
                                            c.g[V1][0] = c.g[V0][0] & c.g[V1][0];
                                            if t {
                                                break 'b_30f55c;
                                            } else {
                                                c.cycles += 6;
                                                c.g[V0][0] = sx((c.g[S7][0] as u32) << 2);
                                                c.sd((c.g[S0][0] as u32), c.g[V1][0]);
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add((c.g[T2][0] as u32)));
                                                c.g[V0][0] = sx((c.g[T0][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                let t = true;
                                                c.sw((c.g[V0][0] as u32), 0u64);
                                                if t {
                                                    break 'b_30f668;
                                                } else {
                                                    break 'b_30f55c;
                                                }
                                            }
                                        }
                                        'b_30f604: {
                                            c.cycles += 3;
                                            c.g[S4][0] = sx((c.g[S7][0] as u32) << 2);
                                            let t = c.g[V1][0] == 0u64;
                                            c.g[S3][0] = sx(frame.at(c, 0x848));
                                            if t {
                                                break 'b_30f604;
                                            } else {
                                                'b_30f5e4: {
                                                    c.cycles += 9;
                                                    c.sd((c.g[S1][0] as u32), c.g[V1][0]);
                                                    c.g[V0][0] = sx((c.g[S4][0] as u32)
                                                        .wrapping_add((c.g[A3][0] as u32)));
                                                    c.g[V0][0] = sx((c.g[T0][0] as u32)
                                                        .wrapping_add((c.g[V0][0] as u32)));
                                                    c.lwc1(
                                                        0,
                                                        (c.g[S0][0] as u32).wrapping_add(0x108),
                                                    );
                                                    c.swc1(
                                                        0,
                                                        (c.g[S1][0] as u32).wrapping_add(0x108),
                                                    );
                                                    c.lwc1(
                                                        1,
                                                        (c.g[S0][0] as u32).wrapping_add(0x10c),
                                                    );
                                                    c.swc1(
                                                        1,
                                                        (c.g[S1][0] as u32).wrapping_add(0x10c),
                                                    );
                                                    let t = c.g[S7][0] == 0u64;
                                                    c.sw((c.g[V0][0] as u32), 0u64);
                                                    if t {
                                                        break 'b_30f5e4;
                                                    } else {
                                                        c.cycles += 22;
                                                        c.g[A1][0] = c.g[T4][0];
                                                        frame.save(c, 7, 0x4cb0);
                                                        frame.save(c, 8, 0x4cc0);
                                                        c.g[A0][0] = c.g[FP][0];
                                                        frame.save(c, 9, 0x4cd0);
                                                        c.g[A2][0] = c.g[S4][0];
                                                        frame.save(c, 10, 0x4ce0);
                                                        frame.save(c, 11, 0x4cf0);
                                                        frame.save(c, 12, 0x4d00);
                                                        frame.save(c, 14, 0x4d20);
                                                        frame.save(c, 15, 0x4d30);
                                                        c.g[RA][0] = sx(0x0030f5c0);
                                                        frame.save(c, 25, 0x4d40);
                                                        call_ptr(c, 0x003e6574, 0x0030f5c0)?;
                                                        frame.restore(c, 25, 0x4d40);
                                                        frame.restore(c, 15, 0x4d30);
                                                        frame.restore(c, 14, 0x4d20);
                                                        frame.restore(c, 12, 0x4d00);
                                                        frame.restore(c, 11, 0x4cf0);
                                                        frame.restore(c, 10, 0x4ce0);
                                                        frame.restore(c, 9, 0x4cd0);
                                                        frame.restore(c, 8, 0x4cc0);
                                                        frame.restore(c, 7, 0x4cb0);
                                                        break 'b_30f5e4;
                                                    }
                                                }
                                                c.cycles += 8;
                                                c.g[V0][0] = c.lw(frame.at(c, 0x4c90));
                                                c.g[FP][0] =
                                                    sx((c.g[FP][0] as u32).wrapping_add(0x110));
                                                c.g[S1][0] =
                                                    sx((c.g[S1][0] as u32).wrapping_add(0x110));
                                                c.g[A3][0] =
                                                    sx((c.g[A3][0] as u32).wrapping_add(0x110));
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                c.g[T7][0] =
                                                    sx((c.g[T7][0] as u32).wrapping_add(0x110));
                                                c.sw(frame.at(c, 0x4c90), c.g[V0][0]);
                                                c.g[T6][0] =
                                                    sx((c.g[T6][0] as u32).wrapping_add(0x110));
                                                break 'b_30f604;
                                            }
                                        }
                                        'b_30f660: {
                                            'b_30f648: {
                                                c.cycles += 14;
                                                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x108));
                                                c.g[AT][0] = 0x3f800000;
                                                c.mtc1(1, 3);
                                                c.sd((c.g[S0][0] as u32), c.g[S2][0]);
                                                c.sub_s(0, 3, 0);
                                                c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x10c));
                                                c.lwc1(1, (c.g[T3][0] as u32).wrapping_add(0xc));
                                                c.add_s(2, 2, 1);
                                                c.mul_s(0, 0, 1);
                                                c.div_s(1, 0, 2);
                                                c.c_lt_s(1, 20);
                                                let t = !c.fcc();
                                                c.swc1(2, (c.g[S0][0] as u32).wrapping_add(0x10c));
                                                if t {
                                                    break 'b_30f648;
                                                } else {
                                                    c.cycles += 3;
                                                    c.mtc1(0, 1);
                                                    let t = true;
                                                    c.g[V0][0] = sx((c.g[S4][0] as u32)
                                                        .wrapping_add((c.g[T2][0] as u32)));
                                                    if t {
                                                        break 'b_30f660;
                                                    } else {
                                                        break 'b_30f648;
                                                    }
                                                }
                                            }
                                            'b_30f65c: {
                                                c.cycles += 5;
                                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3238));
                                                c.c_le_s(0, 1);
                                                if c.fcc() {
                                                    c.mov_s(1, 3);
                                                    break 'b_30f65c;
                                                } else {
                                                    break 'b_30f65c;
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[V0][0] = sx((c.g[S4][0] as u32)
                                                .wrapping_add((c.g[T2][0] as u32)));
                                            break 'b_30f660;
                                        }
                                        c.cycles += 2;
                                        c.g[V0][0] =
                                            sx((c.g[S3][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.swc1(1, (c.g[V0][0] as u32));
                                        break 'b_30f668;
                                    }
                                    c.cycles += 5;
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x110));
                                    c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x110));
                                    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x1));
                                    let t = c.g[S6][0] != 0u64;
                                    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x110));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0030f530);
                                        }
                                        continue 'l_30f530;
                                    } else {
                                        break 'b_30f67c;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V1][0] = c.lw(frame.at(c, 0x4c80));
                        break 'b_30f680;
                    }
                    c.cycles += 4;
                    c.g[S7][0] = c.g[T9][0];
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = c.lw(frame.at(c, 0x4c84));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0030f370);
                        }
                        continue 'l_30f370;
                    } else {
                        break 'b_30f690;
                    }
                }
            }
        }
        'b_30ffcc: {
            c.cycles += 3;
            c.g[A0][0] = c.lw(frame.at(c, 0x4c90));
            let t = (c.g[A0][0] as i64) <= 0;
            c.g[T9][0] = 0u64;
            if t {
                break 'b_30ffcc;
            } else {
                c.cycles += 1;
                c.g[A2][0] = sx((c.g[T9][0] as u32).wrapping_add(0x1));
                'l_30f6a0: loop {
                    'b_30ffb8: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw(frame.at(c, 0x4c80));
                        c.g[A1][0] = 0u64;
                        let t = (c.g[V0][0] as i64) <= 0;
                        c.sw(frame.at(c, 0x4ca0), c.g[A2][0]);
                        if t {
                            break 'b_30ffb8;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = 0x110;
                            c.mult(0, 3, 25, 2);
                            c.sw(frame.at(c, 0x4c98), c.g[V1][0]);
                            c.g[A0][0] = c.lw(frame.at(c, 0x4c98));
                            'l_30f6c0: loop {
                                'b_30ffa8: {
                                    c.cycles += 11;
                                    c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                                    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                    c.mtc1(0, 0);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    c.sw(frame.at(c, 0x4ca4), c.g[A2][0]);
                                    c.g[V1][0] = sx(sp(c).wrapping_add((c.g[V0][0] as u32)));
                                    c.lwc1(20, (c.g[V1][0] as u32).wrapping_add(0x848));
                                    c.c_eq_s(20, 0);
                                    if c.fcc() {
                                        c.g[A1][0] = c.lw(frame.at(c, 0x4ca4));
                                        break 'b_30ffa8;
                                    } else {
                                        'b_30ffa4: {
                                            c.cycles += 19;
                                            c.g[V1][0] = c.lw(frame.at(c, 0x4c84));
                                            c.g[V0][0] = 0x50;
                                            c.g[A0][0] = 0x14;
                                            c.g[S2][0] = 0u64;
                                            c.lo[0] = c.g[V1][0];
                                            c.madd(0, 19, 5, 2);
                                            c.g[V1][0] = c.lw((c.g[S3][0] as u32));
                                            c.g[V0][0] = c.g[V1][0] & 0xff;
                                            c.g[V1][0] = sx((c.g[V1][0] as u32) >> 8);
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                            c.mult(0, 3, 3, 4);
                                            c.g[A0][0] = c.lw(frame.at(c, 0x4c8c));
                                            c.g[V0][0] = sx((c.g[A0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                                            c.g[A1][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[A1][0] as u32)));
                                            c.g[V1][0] =
                                                c.lhu((c.g[V1][0] as u32).wrapping_add(0x8));
                                            let t = c.g[V1][0] == 0u64;
                                            c.sw(frame.at(c, 0x4c9c), c.g[V1][0]);
                                            if t {
                                                break 'b_30ffa4;
                                            } else {
                                                c.cycles += 2;
                                                c.g[A1][0] = c.lw((c.g[S3][0] as u32));
                                                'l_30f740: loop {
                                                    'b_30ff94: {
                                                        'b_30ff90: {
                                                            c.cycles += 22;
                                                            c.g[V1][0] = 0x14;
                                                            c.g[A2][0] = c.lw(frame.at(c, 0x4c8c));
                                                            c.g[V0][0] = c.g[A1][0] & 0xff;
                                                            c.g[A3][0] = c.lw((c.g[S5][0] as u32)
                                                                .wrapping_add(0x14));
                                                            c.g[A1][0] =
                                                                sx((c.g[A1][0] as u32) >> 8);
                                                            c.g[V0][0] =
                                                                sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[A2][0] as u32)
                                                                .wrapping_add((c.g[V0][0] as u32)));
                                                            c.mult(0, 5, 5, 3);
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                            c.g[A0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[A2][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0xc));
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add((c.g[A1][0] as u32)));
                                                            c.g[V1][0] = c.lhu(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0xa),
                                                            );
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.mult(0, 3, 18, 3);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add((c.g[V1][0] as u32)));
                                                            c.g[V0][0] =
                                                                sx((c.g[V0][0] as u32) << 3);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add((c.g[A2][0] as u32)));
                                                            c.g[V0][0] = c.lbu(
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.g[A3][0] = ((c.g[A3][0] as i64)
                                                                < (c.g[V0][0] as i64))
                                                                as u64;
                                                            let t = c.g[A3][0] != 0u64;
                                                            c.g[T5][0] = sx((c.g[S2][0] as u32)
                                                                .wrapping_add(0x1));
                                                            if t {
                                                                break 'b_30ff90;
                                                            } else {
                                                                c.cycles += 6;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[S5][0] as u32)
                                                                        .wrapping_add(0x1c));
                                                                c.g[V0][0] =
                                                                    sx((c.g[V0][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[V1][0] as u32),
                                                                    ));
                                                                c.g[V0][0] =
                                                                    c.lw((c.g[V0][0] as u32));
                                                                let t = (c.g[V0][0] as i64) < 0;
                                                                c.g[A0][0] = 0x58;
                                                                if t {
                                                                    break 'b_30ff90;
                                                                } else {
                                                                    c.cycles += 12;
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[S5][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    c.mult(0, 4, 2, 4);
                                                                    c.g[A2][0] =
                                                                        c.lw(frame.at(c, 0x4c98));
                                                                    c.g[V0][0] =
                                                                        sx(frame.at(c, 0x840));
                                                                    c.g[V1][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            (c.g[A2][0] as u32),
                                                                        ));
                                                                    c.g[FP][0] = c.g[V0][0];
                                                                    c.g[V1][0] =
                                                                        c.ld((c.g[V1][0] as u32));
                                                                    c.g[S6][0] = sx((c.g[A1][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            (c.g[A0][0] as u32),
                                                                        ));
                                                                    c.g[V0][0] = c
                                                                        .ld((c.g[S6][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    c.g[V0][0] =
                                                                        c.g[V0][0] & c.g[V1][0];
                                                                    if c.g[V0][0] == 0u64 {
                                                                        c.g[V1][0] =
                                                                            c.lw(
                                                                                frame.at(c, 0x4c9c)
                                                                            );
                                                                        break 'b_30ff94;
                                                                    } else {
                                                                        'b_30f984: {
                                                                            'b_30f980: {
                                                                                c.cycles += 3;
                                                                                c.g[S0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x4c));
                                                                                let t = c.g[S0][0]
                                                                                    == 0u64;
                                                                                c.g[S7][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xc));
                                                                                if t {
                                                                                    break 'b_30f980;
                                                                                } else {
                                                                                    c.cycles += 8;
                                                                                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x10));
                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S7][0] as u32)));
                                                                                    c.g[V0][0] = c
                                                                                        .g[V1][0]
                                                                                        >> (c.g
                                                                                            [V0]
                                                                                            [0]
                                                                                            & 63);
                                                                                    c.g[V0][0] = c
                                                                                        .g[V0][0]
                                                                                        & 0x1;
                                                                                    c.g[V0][0] = c
                                                                                        .g[V0][0]
                                                                                        << 32;
                                                                                    c.g[V0][0] = ((c
                                                                                        .g[V0][0]
                                                                                        as i64)
                                                                                        >> 32)
                                                                                        as u64;
                                                                                    if c.g[V0][0]
                                                                                        == 0u64
                                                                                    {
                                                                                        c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x44));
                                                                                        break 'b_30f984;
                                                                                    } else {
                                                                                        'b_30f848: {
                                                                                            'b_30f838: {
                                                                                                c.cycles += 6;
                                                                                                c.g[AT][0] = 0x3f800000;
                                                                                                c.mtc1(1, 0);
                                                                                                c.c_eq_s(20, 0);
                                                                                                let t = !c.fcc();
                                                                                                c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x8));
                                                                                                if t
                                                                                                {
                                                                                                    break 'b_30f838;
                                                                                                } else {
                                                                                                    c.cycles += 5;
                                                                                                    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x3c));
                                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                                                    c.g[S4][0] = c.g[V0][0];
                                                                                                    let t = true;
                                                                                                    c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                                    if t {
    break 'b_30f848;
    } else {
    break 'b_30f838;
    }
                                                                                                }
                                                                                            }
                                                                                            c.cycles += 4;
                                                                                            c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                                                                                            c.g[V0][0] = sx(frame.at(c, 0x20));
                                                                                            c.g[S4][0] = c.g[V1][0];
                                                                                            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                                            break 'b_30f848;
                                                                                        }
                                                                                        'b_30f914: {
                                                                                            'b_30f8d8: {
                                                                                                c.cycles += 3;
                                                                                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                                                                                let t = c.g[V0][0] == 0u64;
                                                                                                c.g[T5][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                                                                                if t
                                                                                                {
                                                                                                    break 'b_30f8d8;
                                                                                                } else {
                                                                                                    c.cycles += 17;
                                                                                                    c.g[V0][0] = c.lbu((c.g[S3][0] as u32));
                                                                                                    c.g[V1][0] = c.lw(frame.at(c, 0x4c8c));
                                                                                                    c.g[A1][0] = sx(frame.at(c, 0x220));
                                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                                                    c.g[A2][0] = c.g[S3][0];
                                                                                                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                                    c.g[A3][0] = c.g[S2][0];
                                                                                                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                                                                                                    c.g[T0][0] = 0u64;
                                                                                                    frame.save(c, 13, 0x4d10);
                                                                                                    c.g[T1][0] = c.g[S0][0];
                                                                                                    c.g[RA][0] = sx(0x0030f888);
                                                                                                    frame.save(c, 25, 0x4d40);
                                                                                                    call_ptr(c, 0x00311318, 0x0030f888)?;
                                                                                                    c.g[A1][0] = 0u64;
                                                                                                    frame.restore(c, 13, 0x4d10);
                                                                                                    let t = (c.g[S0][0] as i64) <= 0;
                                                                                                    frame.restore(c, 25, 0x4d40);
                                                                                                    if t {
    break 'b_30f914;
    } else {
    c.cycles += 2;
    c.g[A3][0] = sx(frame.at(c, 0x220));
    c.g[A2][0] = c.g[S1][0];
    'l_30f8a0: loop {
    c.cycles += 12;
    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x40));
    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[A0][0] = ((c.g[A1][0] as i64) < (c.g[S0][0] as i64)) as u64;
    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.lwc1(0, (c.g[V0][0] as u32));
    c.swc1(0, (c.g[A2][0] as u32));
    let t = c.g[A0][0] != 0u64;
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
    if t {
    if event_due(c) { deliver_event_here(c, 0x0030f8a0); } continue 'l_30f8a0;
    } else {
    c.cycles += 2;
    let t = true;

    if t {
    break 'b_30f914;
    } else {
    break 'b_30f8d8;
    }
    }
    }
    }
                                                                                                }
                                                                                            }
                                                                                            c.cycles += 15;
                                                                                            c.g[V0][0] = c.lbu((c.g[S3][0] as u32));
                                                                                            c.g[A0][0] = c.lw(frame.at(c, 0x4c8c));
                                                                                            c.g[A1][0] = c.g[S1][0];
                                                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                                            c.g[A2][0] = c.g[S3][0];
                                                                                            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                            c.g[A3][0] = c.g[S2][0];
                                                                                            c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                                                                                            c.g[T0][0] = 0u64;
                                                                                            frame.save(c, 13, 0x4d10);
                                                                                            c.g[T1][0] = c.g[S0][0];
                                                                                            c.g[RA][0] = sx(0x0030f90c);
                                                                                            frame.save(c, 25, 0x4d40);
                                                                                            call_ptr(c, 0x00311318, 0x0030f90c)?;
                                                                                            frame.restore(c, 25, 0x4d40);
                                                                                            frame.restore(c, 13, 0x4d10);
                                                                                            break 'b_30f914;
                                                                                        }
                                                                                        'b_30f940: {
                                                                                            c.cycles += 6;
                                                                                            c.g[AT][0] = 0x3f800000;
                                                                                            c.mtc1(
                                                                                                1,
                                                                                                1,
                                                                                            );
                                                                                            c.c_eq_s(20, 1);
                                                                                            let t = !c.fcc();
                                                                                            c.g[V1][0] = 0x1;
                                                                                            if t {
                                                                                                break 'b_30f940;
                                                                                            } else {
                                                                                                c.cycles += 5;
                                                                                                c.g[V0][0] = sx((c.g[S7][0] as u32) << 2);
                                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(sp(c)));
                                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x820));
                                                                                                let t = true;
                                                                                                c.sw((c.g[V0][0] as u32), c.g[V1][0]);
                                                                                                if t
                                                                                                {
                                                                                                    break 'b_30f980;
                                                                                                } else {
                                                                                                    break 'b_30f940;
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        c.cycles +=
                                                                                            4;
                                                                                        c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x3c));
                                                                                        c.sub_s(
                                                                                            2, 1,
                                                                                            20,
                                                                                        );
                                                                                        let t = (c.g
                                                                                            [S0][0]
                                                                                            as i64)
                                                                                            <= 0;
                                                                                        c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                        if t {
                                                                                            break 'b_30f980;
                                                                                        } else {
                                                                                            c.cycles += 2;
                                                                                            c.g[A1][0] = c.g[S1][0];
                                                                                            c.g[T1][0] = c.g[S0][0];
                                                                                            'l_30f958: loop {
                                                                                                c.cycles += 10;
                                                                                                c.lwc1(1, (c.g[V0][0] as u32));
                                                                                                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1));
                                                                                                c.lwc1(0, (c.g[A1][0] as u32));
                                                                                                c.mul_s(1, 2, 1);
                                                                                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                                                                                                c.mul_s(0, 20, 0);
                                                                                                c.add_s(1, 1, 0);
                                                                                                c.swc1(1, (c.g[V0][0] as u32));
                                                                                                let t = c.g[T1][0] != 0u64;
                                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                                                                                                if t
                                                                                                {
                                                                                                    if event_due(c) { deliver_event_here(c, 0x0030f958); }
                                                                                                    continue 'l_30f958;
                                                                                                } else {
                                                                                                    break 'b_30f980;
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            c.cycles += 1;
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [S6][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x44,
                                                                                ));
                                                                            break 'b_30f984;
                                                                        }
                                                                        'b_30f9c4: {
                                                                            c.cycles += 2;
                                                                            let t = (c.g[V0][0]
                                                                                as i64)
                                                                                <= 0;
                                                                            c.g[A3][0] = c.g[S2][0];
                                                                            if t {
                                                                                break 'b_30f9c4;
                                                                            } else {
                                                                                c.cycles += 14;
                                                                                c.g[V0][0] = c.lbu(
                                                                                    (c.g[S3][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[V1][0] =
                                                                                    c.lw(frame.at(
                                                                                        c, 0x4c8c,
                                                                                    ));
                                                                                c.g[A1][0] =
                                                                                    sx(frame.at(
                                                                                        c, 0x420,
                                                                                    ));
                                                                                c.g[V0][0] =
                                                                                    sx((c.g[V0][0]
                                                                                        as u32)
                                                                                        << 2);
                                                                                c.g[A2][0] =
                                                                                    c.g[S3][0];
                                                                                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                c.g[T0][0] = 0u64;
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[V0][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[T1][0] = 0x80;
                                                                                frame.save(
                                                                                    c, 13, 0x4d10,
                                                                                );
                                                                                c.g[RA][0] =
                                                                                    sx(0x0030f9bc);
                                                                                frame.save(
                                                                                    c, 25, 0x4d40,
                                                                                );
                                                                                call_ptr(
                                                                                    c, 0x00311318,
                                                                                    0x0030f9bc,
                                                                                )?;
                                                                                frame.restore(
                                                                                    c, 25, 0x4d40,
                                                                                );
                                                                                frame.restore(
                                                                                    c, 13, 0x4d10,
                                                                                );
                                                                                break 'b_30f9c4;
                                                                            }
                                                                        }
                                                                        c.cycles += 7;
                                                                        c.g[T2][0] = c
                                                                            .lw((c.g[S6][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        c.g[V0][0] = c.lw((c.g[S6]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x44));
                                                                        c.g[T0][0] = c.g[T2][0];
                                                                        c.g[V0][0] =
                                                                            sx((c.g[T0][0] as u32)
                                                                                .wrapping_add(
                                                                                    (c.g[V0][0]
                                                                                        as u32),
                                                                                ));
                                                                        c.g[V0][0] = ((c.g[T0][0]
                                                                            as i64)
                                                                            < (c.g[V0][0] as i64))
                                                                            as u64;
                                                                        let t = c.g[V0][0] == 0u64;
                                                                        c.g[A3][0] = c.lw((c.g[S6]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x38));
                                                                        if t {
                                                                            break 'b_30ff90;
                                                                        } else {
                                                                            c.cycles += 14;
                                                                            c.g[V0][0] = 0x110;
                                                                            c.g[V1][0] =
                                                                                sx((c.g[T0][0]
                                                                                    as u32)
                                                                                    << 2);
                                                                            c.mult(0, 4, 25, 2);
                                                                            c.g[A2][0] =
                                                                                sx((c.g[V1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        sp(c),
                                                                                    ));
                                                                            c.g[V1][0] = c.g[A2][0];
                                                                            c.g[AT][0] = 0x3f800000;
                                                                            c.mtc1(1, 6);
                                                                            c.g[T4][0] =
                                                                                sx((c.g[A2][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x720,
                                                                                    ));
                                                                            c.g[T1][0] =
                                                                                sx(frame
                                                                                    .at(c, 0x420));
                                                                            c.g[S0][0] =
                                                                                0xffffffffffffffff;
                                                                            c.g[S1][0] = 0x500000;
                                                                            c.g[T6][0] =
                                                                                sx((c.g[A0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        (c.g[FP][0]
                                                                                            as u32),
                                                                                    ));
                                                                            c.g[T7][0] = 0x1;
                                                                            c.g[T3][0] =
                                                                                sx((c.g[V1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x620,
                                                                                    ));
                                                                            'l_30fa18: loop {
                                                                                'b_30ff74: {
                                                                                    c.cycles += 5;
                                                                                    c.g[V0][0] = c
                                                                                        .ld(
                                                                                        (c.g[T6][0]
                                                                                            as u32),
                                                                                    );
                                                                                    c.g[V0][0] = c
                                                                                        .g[V0][0]
                                                                                        >> (c.g
                                                                                            [T0]
                                                                                            [0]
                                                                                            & 63);
                                                                                    c.g[V0][0] = c
                                                                                        .g[V0][0]
                                                                                        & 0x1;
                                                                                    if c.g[V0][0]
                                                                                        == 0u64
                                                                                    {
                                                                                        c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x44));
                                                                                        break 'b_30ff74;
                                                                                    } else {
                                                                                        'b_30fa5c: {
                                                                                            'b_30fa50: {
                                                                                                c.cycles += 3;
                                                                                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                                                                                let t = c.g[V0][0] == 0u64;
                                                                                                c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x20));
                                                                                                if t
                                                                                                {
                                                                                                    break 'b_30fa50;
                                                                                                } else {
                                                                                                    c.cycles += 6;
                                                                                                    c.g[V0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x16));
                                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T2][0] as u32)));
                                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                                                    c.g[A0][0] = c.g[V1][0];
                                                                                                    let t = true;
                                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                                                    if t {
    break 'b_30fa5c;
    } else {
    break 'b_30fa50;
    }
                                                                                                }
                                                                                            }
                                                                                            c.cycles += 3;
                                                                                            c.g[V0][0] = sx((c.g[T0][0] as u32) << 2);
                                                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                                            c.g[A0][0] = c.g[V1][0];
                                                                                            break 'b_30fa5c;
                                                                                        }
                                                                                        'b_30fc28: {
                                                                                            c.cycles += 8;
                                                                                            c.g[A1][0] = c.lw((c.g[V0][0] as u32));
                                                                                            c.g[V0][0] = sx((c.g[T2][0] as u32) << 2);
                                                                                            c.g[V1][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x14));
                                                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                                            c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                                                                                            c.g[V1][0] = c.g[V1][0] & 0x1;
                                                                                            let t = c.g[V1][0] == 0u64;
                                                                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
                                                                                            if t {
                                                                                                break 'b_30fc28;
                                                                                            } else {
                                                                                                'b_30fa9c: {
                                                                                                    'b_30fa98: {
                                                                                                        c.cycles += 4;
                                                                                                        c.c_eq_s(20, 6);
                                                                                                        let t = !c.fcc();
                                                                                                        c.g[V0][0] = sx((c.g[T0][0] as u32) << 4);
                                                                                                        if t {
    break 'b_30fa98;
    } else {
    c.cycles += 3;
    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x24));
    let t = true;
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    if t {
    break 'b_30fa9c;
    } else {
    break 'b_30fa98;
    }
    }
                                                                                                    }
                                                                                                    c.cycles += 1;
                                                                                                    c.g[A0][0] = c.g[SP][0];
                                                                                                    break 'b_30fa9c;
                                                                                                }
                                                                                                'b_30fb18: {
                                                                                                    c.cycles += 18;
                                                                                                    c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                                                                                                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                                                                                    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                                                    c.lwc1(5, (c.g[V0][0] as u32));
                                                                                                    c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                                                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2));
                                                                                                    c.swc1(5, (c.g[A0][0] as u32));
                                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                                                    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                                    c.lwc1(3, (c.g[V1][0] as u32));
                                                                                                    c.swc1(3, (c.g[A0][0] as u32).wrapping_add(0x4));
                                                                                                    c.lwc1(4, (c.g[V0][0] as u32));
                                                                                                    c.swc1(6, (c.g[A0][0] as u32).wrapping_add(0xc));
                                                                                                    c.swc1(4, (c.g[A0][0] as u32).wrapping_add(0x8));
                                                                                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                                                                                    let t = c.g[V0][0] == 0u64;
                                                                                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3));
                                                                                                    if t {
    break 'b_30fb18;
    } else {
    c.cycles += 13;
    c.lwc1(0, (c.g[A3][0] as u32).wrapping_add(0x40));
    c.lwc1(1, (c.g[A3][0] as u32).wrapping_add(0x44));
    c.lwc1(2, (c.g[A3][0] as u32).wrapping_add(0x48));
    c.mul_s(0, 5, 0);
    c.mul_s(1, 3, 1);
    c.mul_s(2, 4, 2);
    c.lwc1(3, (c.g[A3][0] as u32).wrapping_add(0x4c));
    frame.save_fpr(c, 0, 0x4c40);
    frame.save_fpr(c, 1, 0x4c44);
    frame.save_fpr(c, 2, 0x4c48);
    frame.save_fpr(c, 3, 0x4c4c);
    frame.restore(c, 2, 0x4c40);
    c.sq(2, (c.g[A0][0] as u32));
    break 'b_30fb18;
    }
                                                                                                }
                                                                                                'b_30fbe8: {
                                                                                                    c.cycles += 3;
                                                                                                    c.g[V0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x10));
                                                                                                    let t = c.g[V0][0] != c.g[S0][0];

                                                                                                    if t {
    break 'b_30fbe8;
    } else {
    'b_30fbdc: {
    'b_30fb74: {
    c.cycles += 5;
    c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0xc));
    c.mtc1(0, 0);
    c.c_eq_s(1, 0);
    let t = !c.fcc();

    if t {
    break 'b_30fb74;
    } else {
    c.cycles += 15;
    c.lqc2(4, (c.g[S3][0] as u32).wrapping_add(0x30));
    c.lqc2(5, (c.g[A0][0] as u32));
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
    c.sqc2(8, frame.at(c, 0x4c50));
    let t = true;
    frame.restore(c, 2, 0x4c50);
    if t {
    break 'b_30fbdc;
    } else {
    break 'b_30fb74;
    }
    }
    }
    c.cycles += 26;
    c.lqc2(3, (c.g[A0][0] as u32));
    c.lqc2(4, (c.g[S1][0] as u32).wrapping_sub(0xed0));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, frame.at(c, 0x4c70));
    frame.restore(c, 2, 0x4c70);
    frame.save(c, 2, 0x4c60);
    c.lqc2(4, (c.g[S3][0] as u32).wrapping_add(0x30));
    c.lqc2(5, frame.at(c, 0x4c60));
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
    c.sqc2(8, frame.at(c, 0x4c70));
    frame.restore(c, 2, 0x4c70);
    frame.save(c, 2, 0x4c50);
    c.lqc2(3, frame.at(c, 0x4c50));
    c.lqc2(4, (c.g[S3][0] as u32).wrapping_add(0x20));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, frame.at(c, 0x4c70));
    frame.restore(c, 2, 0x4c70);
    break 'b_30fbdc;
    }
    c.cycles += 3;
    frame.save(c, 2, 0x4c40);
    frame.restore(c, 2, 0x4c40);
    c.sq(2, (c.g[A0][0] as u32));
    break 'b_30fbe8;
    }
                                                                                                }
                                                                                                'b_30fc00: {
                                                                                                    c.cycles += 4;
                                                                                                    c.c_eq_s(20, 6);
                                                                                                    if !c.fcc() {
    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x24));
    break 'b_30fc00;
    } else {
    c.cycles += 2;
    let t = true;
    c.sw((c.g[T4][0] as u32), c.g[T7][0]);
    if t {
    break 'b_30fc28;
    } else {
    break 'b_30fc00;
    }
    }
                                                                                                }
                                                                                                c.cycles += 10;
                                                                                                c.g[V1][0] = sx((c.g[T0][0] as u32) << 4);
                                                                                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                                c.mfc1(2, 20);
                                                                                                c.lqc2(4, (c.g[V1][0] as u32));
                                                                                                c.qmtc2(2, 3);
                                                                                                c.lqc2(5, (c.g[A0][0] as u32));
                                                                                                c.vu0().mula(0b1111, 4, Src::Bc(0, W));
                                                                                                c.vu0().msuba(0b1111, 4, Src::Bc(3, X));
                                                                                                c.vu0().madd(0b1111, 4, 5, Src::Bc(3, X));
                                                                                                c.sqc2(4, (c.g[V1][0] as u32));
                                                                                                break 'b_30fc28;
                                                                                            }
                                                                                        }
                                                                                        c.cycles +=
                                                                                            4;
                                                                                        c.g[V0][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x14));
                                                                                        c.g[V0]
                                                                                            [0] = c
                                                                                            .g[V0]
                                                                                            [0]
                                                                                            & 0x2;
                                                                                        if c.g[V0]
                                                                                            [0]
                                                                                            == 0u64
                                                                                        {
                                                                                            c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x44));
                                                                                            break 'b_30ff74;
                                                                                        } else {
                                                                                            'b_30fc58: {
                                                                                                'b_30fc54: {
                                                                                                    c.cycles += 4;
                                                                                                    c.c_eq_s(20, 6);
                                                                                                    let t = !c.fcc();
                                                                                                    c.g[V0][0] = sx((c.g[T0][0] as u32) << 4);
                                                                                                    if t {
    break 'b_30fc54;
    } else {
    c.cycles += 3;
    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x28));
    let t = true;
    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    if t {
    break 'b_30fc58;
    } else {
    break 'b_30fc54;
    }
    }
                                                                                                }
                                                                                                c.cycles += 1;
                                                                                                c.g[A2][0] = sx(frame.at(c, 0x10));
                                                                                                break 'b_30fc58;
                                                                                            }
                                                                                            'b_30fe98: {
                                                                                                c.cycles += 85;
                                                                                                c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                                                                                                c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                                c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x8));
                                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                                                                                                c.lwc1(0, (c.g[V0][0] as u32));
                                                                                                frame.save_fpr(c, 1, 0x4c44);
                                                                                                frame.save_fpr(c, 2, 0x4c48);
                                                                                                frame.save_fpr(c, 0, 0x4c40);
                                                                                                c.sw(frame.at(c, 0x4c4c), 0u64);
                                                                                                c.lqc2(3, frame.at(c, 0x4c40));
                                                                                                c.vu0().add(0b1110, 4, 0, Src::Bc(0, W));
                                                                                                c.vu0().abs(0b1110, 6, 3);
                                                                                                c.vu0().ftoi(0, 0b1111, 8, 3);
                                                                                                c.vu0().add(0b1110, 7, 3, Src::Vec(6));
                                                                                                c.qmfc2(4, 8);
                                                                                                c.vu0().ftoi(0, 0b1110, 6, 4);
                                                                                                c.qmfc2(5, 7);
                                                                                                c.pceqw(5, 5, 0);
                                                                                                c.paddw(4, 4, 5);
                                                                                                c.psraw(5, 4, 1);
                                                                                                c.qmtc2(5, 7);
                                                                                                c.qmfc2(2, 6);
                                                                                                c.paddw(2, 2, 4);
                                                                                                c.vu0().itof(0, 0b1111, 7, 7);
                                                                                                c.psllw(4, 4, 15);
                                                                                                c.psllw(4, 4, 15);
                                                                                                c.pceqw(5, 4, 0);
                                                                                                c.vu0().suba(0b1110, 3, Src::Vec(7));
                                                                                                c.vu0().msub(0b1110, 3, 7, Src::Bc(0, W));
                                                                                                c.pcgtw(4, 4, 0);
                                                                                                c.por(4, 4, 5);
                                                                                                c.vu0().suba(0b1110, 3, Src::Bc(0, W));
                                                                                                c.vu0().msub(0b1110, 5, 4, Src::Bc(0, W));
                                                                                                c.psllw(2, 2, 15);
                                                                                                c.psllw(2, 2, 15);
                                                                                                c.vu0().mul(0b1110, 5, 5, Src::Vec(3));
                                                                                                c.vu0().sub(0b1110, 9, 0, Src::Vec(5));
                                                                                                c.vu0().mula(0b1110, 9, Src::Vec(5));
                                                                                                c.vu0().madd(0b1110, 10, 4, Src::Bc(0, W));
                                                                                                c.qmfc2(5, 9);
                                                                                                c.pand(5, 5, 4);
                                                                                                c.qmfc2(3, 5);
                                                                                                c.pnor(4, 4, 0);
                                                                                                c.pand(4, 4, 3);
                                                                                                c.por(5, 5, 4);
                                                                                                frame.save(c, 5, 0x4c50);
                                                                                                c.sqc2(10, frame.at(c, 0x4c60));
                                                                                                frame.restore_fpr(c, 0, 0x4c60);
                                                                                                c.sqrt_s(0, 0);
                                                                                                frame.save_fpr(c, 0, 0x4c60);
                                                                                                frame.restore_fpr(c, 0, 0x4c64);
                                                                                                c.sqrt_s(0, 0);
                                                                                                frame.save_fpr(c, 0, 0x4c64);
                                                                                                frame.restore_fpr(c, 0, 0x4c68);
                                                                                                c.sqrt_s(0, 0);
                                                                                                frame.save_fpr(c, 0, 0x4c68);
                                                                                                c.lqc2(3, frame.at(c, 0x4c60));
                                                                                                c.pceqw(5, 2, 0);
                                                                                                c.pcgtw(2, 2, 0);
                                                                                                c.por(2, 2, 5);
                                                                                                c.vu0().sub(0b1110, 4, 0, Src::Vec(3));
                                                                                                c.qmfc2(5, 3);
                                                                                                c.pand(5, 5, 2);
                                                                                                c.pnor(2, 2, 0);
                                                                                                c.qmfc2(3, 4);
                                                                                                c.pand(2, 2, 3);
                                                                                                c.por(5, 5, 2);
                                                                                                frame.save(c, 5, 0x4c60);
                                                                                                frame.restore_fpr(c, 4, 0x4c50);
                                                                                                frame.restore_fpr(c, 0, 0x4c54);
                                                                                                frame.restore_fpr(c, 1, 0x4c68);
                                                                                                c.mul_s(0, 4, 0);
                                                                                                frame.restore_fpr(c, 5, 0x4c58);
                                                                                                frame.restore_fpr(c, 2, 0x4c64);
                                                                                                frame.restore_fpr(c, 3, 0x4c60);
                                                                                                c.mul_s(1, 0, 1);
                                                                                                c.mul_s(4, 4, 2);
                                                                                                c.swc1(3, (c.g[A2][0] as u32));
                                                                                                c.mul_s(0, 0, 5);
                                                                                                c.swc1(1, (c.g[A2][0] as u32).wrapping_add(0x4));
                                                                                                c.swc1(4, (c.g[A2][0] as u32).wrapping_add(0x8));
                                                                                                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
                                                                                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                                                                                if c.g[V0][0] == 0u64 {
    c.g[V0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x10));
    break 'b_30fe98;
    } else {
    'b_30fde0: {
    'b_30fdcc: {
    c.cycles += 4;
    c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x1b));
    c.g[V0][0] = (c.g[V0][0] < 0x4u64) as u64;
    let t = c.g[V0][0] == 0u64;
    c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x1b));
    if t {
    break 'b_30fdcc;
    } else {
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    let t = true;
    c.lwc1(3, (c.g[V0][0] as u32));
    if t {
    break 'b_30fde0;
    } else {
    break 'b_30fdcc;
    }
    }
    }
    c.cycles += 5;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.lwc1(0, (c.g[V0][0] as u32));
    c.neg_s(3, 0);
    break 'b_30fde0;
    }
    'b_30fe14: {
    'b_30fe00: {
    c.cycles += 4;
    c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x18));
    c.g[V0][0] = (c.g[V0][0] < 0x4u64) as u64;
    let t = c.g[V0][0] == 0u64;
    c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x18));
    if t {
    break 'b_30fe00;
    } else {
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    let t = true;
    c.lwc1(2, (c.g[V0][0] as u32));
    if t {
    break 'b_30fe14;
    } else {
    break 'b_30fe00;
    }
    }
    }
    c.cycles += 5;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.lwc1(0, (c.g[V0][0] as u32));
    c.neg_s(2, 0);
    break 'b_30fe14;
    }
    'b_30fe48: {
    'b_30fe34: {
    c.cycles += 4;
    c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x19));
    c.g[V0][0] = (c.g[V0][0] < 0x4u64) as u64;
    let t = c.g[V0][0] == 0u64;
    c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x19));
    if t {
    break 'b_30fe34;
    } else {
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    let t = true;
    c.lwc1(1, (c.g[V0][0] as u32));
    if t {
    break 'b_30fe48;
    } else {
    break 'b_30fe34;
    }
    }
    }
    c.cycles += 5;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.lwc1(0, (c.g[V0][0] as u32));
    c.neg_s(1, 0);
    break 'b_30fe48;
    }
    'b_30fe7c: {
    'b_30fe68: {
    c.cycles += 4;
    c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x1a));
    c.g[V0][0] = (c.g[V0][0] < 0x4u64) as u64;
    let t = c.g[V0][0] == 0u64;
    c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x1a));
    if t {
    break 'b_30fe68;
    } else {
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    let t = true;
    c.lwc1(0, (c.g[V0][0] as u32));
    if t {
    break 'b_30fe7c;
    } else {
    break 'b_30fe68;
    }
    }
    }
    c.cycles += 5;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.lwc1(0, (c.g[V0][0] as u32));
    c.neg_s(0, 0);
    break 'b_30fe7c;
    }
    c.cycles += 7;
    frame.save_fpr(c, 3, 0x4c4c);
    frame.save_fpr(c, 2, 0x4c40);
    frame.save_fpr(c, 1, 0x4c44);
    frame.save_fpr(c, 0, 0x4c48);
    frame.restore(c, 2, 0x4c40);
    c.sq(2, (c.g[A2][0] as u32));
    c.g[V0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x10));
    break 'b_30fe98;
    }
                                                                                            }
                                                                                            'b_30fedc: {
                                                                                                c.cycles += 2;
                                                                                                let t = c.g[V0][0] != c.g[S0][0];

                                                                                                if t
                                                                                                {
                                                                                                    break 'b_30fedc;
                                                                                                } else {
                                                                                                    c.cycles += 15;
                                                                                                    c.lqc2(4, (c.g[S3][0] as u32).wrapping_add(0x30));
                                                                                                    c.lqc2(5, (c.g[A2][0] as u32));
                                                                                                    c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
                                                                                                    c.vu0().opmula(4, 5);
                                                                                                    c.vu0().opmsub(6, 5, 4);
                                                                                                    c.vu0().mula(0b1110, 4, Src::Bc(5, W));
                                                                                                    c.vu0().madda(0b1110, 5, Src::Bc(4, W));
                                                                                                    c.vu0().suba(0b0001, 7, Src::Bc(7, X));
                                                                                                    c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
                                                                                                    c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
                                                                                                    c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
                                                                                                    c.sqc2(8, frame.at(c, 0x4c50));
                                                                                                    frame.restore(c, 2, 0x4c50);
                                                                                                    frame.save(c, 2, 0x4c40);
                                                                                                    c.sq(2, (c.g[A2][0] as u32));
                                                                                                    break 'b_30fedc;
                                                                                                }
                                                                                            }
                                                                                            'b_30ff70: {
                                                                                                'b_30fef4: {
                                                                                                    c.cycles += 4;
                                                                                                    c.c_eq_s(20, 6);
                                                                                                    if !c.fcc() {
    c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x28));
    break 'b_30fef4;
    } else {
    c.cycles += 2;
    let t = true;
    c.sw((c.g[T3][0] as u32), c.g[T7][0]);
    if t {
    break 'b_30ff70;
    } else {
    break 'b_30fef4;
    }
    }
                                                                                                }
                                                                                                c.cycles += 31;
                                                                                                c.g[V0][0] = sx((c.g[T0][0] as u32) << 4);
                                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                                                                                c.mfc1(4, 20);
                                                                                                c.lqc2(4, (c.g[V0][0] as u32));
                                                                                                c.qmtc2(4, 3);
                                                                                                c.lqc2(5, (c.g[A2][0] as u32));
                                                                                                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                                                                c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
                                                                                                c.vu0().adda(0b1000, 7, Src::Bc(7, Y));
                                                                                                c.vu0().madda(0b1000, 6, Src::Bc(7, Z));
                                                                                                c.vu0().madd(0b1000, 7, 6, Src::Bc(7, W));
                                                                                                c.qmfc2(4, 7);
                                                                                                c.g[V1][0] = 0xffffffff80000000;
                                                                                                c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                                                                                                c.pextlw(3, 3, 3);
                                                                                                c.pextlw(3, 3, 3);
                                                                                                c.qmfc2(4, 5);
                                                                                                c.pxor(4, 4, 3);
                                                                                                c.qmtc2(4, 5);
                                                                                                c.vu0().mula(0b1111, 4, Src::Bc(0, W));
                                                                                                c.vu0().msuba(0b1111, 4, Src::Bc(3, X));
                                                                                                c.vu0().madd(0b1111, 3, 5, Src::Bc(3, X));
                                                                                                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                                                                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                                                                                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                                                                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                                                                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                                                                c.vu0().rsqrt(0, 3, 4, 0);
                                                                                                c.vu0().waitq();
                                                                                                c.vu0().mul(0b1111, 5, 3, Src::Q);
                                                                                                c.sqc2(5, (c.g[V0][0] as u32));
                                                                                                break 'b_30ff70;
                                                                                            }
                                                                                            c.cycles += 1;
                                                                                            c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x44));
                                                                                            break 'b_30ff74;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                c.cycles += 7;
                                                                                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                                                                                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x4));
                                                                                c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x4));
                                                                                c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                c.g[V0][0] = ((c.g
                                                                                    [T0][0]
                                                                                    as i64)
                                                                                    < (c.g[V0][0]
                                                                                        as i64))
                                                                                    as u64;
                                                                                let t = c.g[V0][0]
                                                                                    != 0u64;
                                                                                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x50));
                                                                                if t {
                                                                                    if event_due(c)
                                                                                    {
                                                                                        deliver_event_here(c, 0x0030fa18);
                                                                                    }
                                                                                    continue 'l_30fa18;
                                                                                } else {
                                                                                    break 'b_30ff90;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 1;
                                                        c.g[V1][0] = c.lw(frame.at(c, 0x4c9c));
                                                        break 'b_30ff94;
                                                    }
                                                    c.cycles += 4;
                                                    c.g[S2][0] = c.g[T5][0];
                                                    c.g[V0][0] = ((c.g[S2][0] as i64)
                                                        < (c.g[V1][0] as i64))
                                                        as u64;
                                                    if c.g[V0][0] != 0u64 {
                                                        c.g[A1][0] = c.lw((c.g[S3][0] as u32));
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x0030f740);
                                                        }
                                                        continue 'l_30f740;
                                                    } else {
                                                        break 'b_30ffa4;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[A1][0] = c.lw(frame.at(c, 0x4ca4));
                                        break 'b_30ffa8;
                                    }
                                }
                                c.cycles += 4;
                                c.g[A0][0] = c.lw(frame.at(c, 0x4c80));
                                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[A0][0] = c.lw(frame.at(c, 0x4c98));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0030f6c0);
                                    }
                                    continue 'l_30f6c0;
                                } else {
                                    break 'b_30ffb8;
                                }
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[T9][0] = c.lw(frame.at(c, 0x4ca0));
                    c.g[A2][0] = c.lw(frame.at(c, 0x4c90));
                    c.g[V0][0] = ((c.g[T9][0] as i64) < (c.g[A2][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A2][0] = sx((c.g[T9][0] as u32).wrapping_add(0x1));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0030f6a0);
                        }
                        continue 'l_30f6a0;
                    } else {
                        break 'b_30ffcc;
                    }
                }
            }
        }
        'b_3100ec: {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[T1][0] = 0u64;
            if t {
                break 'b_3100ec;
            } else {
                c.cycles += 2;
                c.g[S1][0] = 0x58;
                c.mult(0, 3, 9, 17);
                'l_30ffe0: loop {
                    'b_310090: {
                        c.cycles += 8;
                        c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xc));
                        c.g[T0][0] = 0u64;
                        c.g[S0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                        c.g[A2][0] = c.g[A1][0];
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                        let t = (c.g[V0][0] as i64) <= 0;
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x38));
                        if t {
                            break 'b_310090;
                        } else {
                            c.cycles += 6;
                            c.g[V0][0] = 0x58;
                            c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30));
                            c.mult(0, 10, 9, 2);
                            c.g[T4][0] = sx(frame.at(c, 0x720));
                            c.g[T3][0] = sx(frame.at(c, 0x620));
                            'l_310018: loop {
                                'b_310050: {
                                    c.cycles += 8;
                                    c.g[V0][0] =
                                        sx((c.g[T2][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                                    c.g[A2][0] =
                                        sx((c.g[T0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.g[A1][0] = sx((c.g[A2][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[T4][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                    let t = c.g[V1][0] != 0u64;
                                    c.g[V0][0] =
                                        sx((c.g[T3][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                    if t {
                                        break 'b_310050;
                                    } else {
                                        c.cycles += 6;
                                        c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x24));
                                        c.g[V1][0] = sx((c.g[A2][0] as u32) << 4);
                                        c.lq(4, (c.g[A3][0] as u32).wrapping_sub(0x10));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.sq(4, (c.g[V1][0] as u32));
                                        c.g[V0][0] =
                                            sx((c.g[T3][0] as u32)
                                                .wrapping_add((c.g[A1][0] as u32)));
                                        break 'b_310050;
                                    }
                                }
                                'b_310074: {
                                    c.cycles += 3;
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                    if c.g[V1][0] != 0u64 {
                                        c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xc));
                                        break 'b_310074;
                                    } else {
                                        c.cycles += 6;
                                        c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x28));
                                        c.g[V1][0] = sx((c.g[A2][0] as u32) << 4);
                                        c.lq(4, (c.g[A3][0] as u32));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.sq(4, (c.g[V1][0] as u32));
                                        c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xc));
                                        break 'b_310074;
                                    }
                                }
                                c.cycles += 7;
                                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x50));
                                c.g[V1][0] =
                                    sx((c.g[T2][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                                c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[A2][0] = c.g[A1][0];
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00310018);
                                    }
                                    continue 'l_310018;
                                } else {
                                    break 'b_310090;
                                }
                            }
                        }
                    }
                    'b_3100dc: {
                        'b_3100d8: {
                            c.cycles += 5;
                            c.mult(0, 2, 9, 17);
                            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                            let t = (c.g[V0][0] as i64) < 0;
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            if t {
                                break 'b_3100d8;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(sp(c)));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x820));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                if c.g[V1][0] != 0u64 {
                                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
                                    break 'b_3100dc;
                                } else {
                                    c.cycles += 8;
                                    c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A1][0] = 0u64;
                                    c.g[A2][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4c));
                                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x3c));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                                    c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
                                    c.g[RA][0] = sx(0x003100d8);
                                    c.g[A0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, 0x003e6448, 0x003100d8)?;
                                    break 'b_3100d8;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
                        break 'b_3100dc;
                    }
                    c.cycles += 4;
                    c.g[T1][0] = c.g[S0][0];
                    c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.mult(0, 3, 9, 17);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0030ffe0);
                        }
                        continue 'l_30ffe0;
                    } else {
                        break 'b_3100ec;
                    }
                }
            }
        }
        c.cycles += 13;
        frame.restore(c, 16, 0x4de0);
        frame.restore(c, 17, 0x4dd0);
        frame.restore(c, 18, 0x4dc0);
        frame.restore(c, 19, 0x4db0);
        frame.restore(c, 20, 0x4da0);
        frame.restore(c, 21, 0x4d90);
        frame.restore(c, 22, 0x4d80);
        frame.restore(c, 23, 0x4d70);
        frame.restore(c, 30, 0x4d60);
        frame.restore_ra(c, 0x4d50);
        frame.restore_fpr(c, 20, 0x4df0);
        frame.close(c);
        return Ok(());
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, put_vec, scramble, Rng, DATA, GAME_GP};

    // Fixture objects, all inside RAM below the test stack.
    const MODEL: u32 = DATA; // the rider model
    const STATES: u32 = DATA + 0x2000; // `sAnimState` per bone, 0x50 apart
    const BONES: u32 = DATA + 0x4000; // the model's bone list, 0x50 apart
    const CHANS: u32 = DATA + 0x5000; // global animation-channel table
    const WORDS: u32 = DATA + 0x6000; // model+0x1c
    const FLOATS: u32 = DATA + 0x6400; // model+0x20
    const VEC_A: u32 = DATA + 0x6800; // model+0x24
    const VEC_B: u32 = DATA + 0x6c00; // model+0x28
    const MATS: u32 = DATA + 0x7000; // model+0x3c
    const AUX: u32 = DATA + 0x7800; // *(bone+0x38)
    const IDX: u32 = DATA + 0x7c00; // *(bone+0x40)

    #[test]
    fn rider_model_blend_anim_pose_matches() {
        check(
            ssx3_ee::s10::f_0030f2b0__generated,
            rider_model_blend_anim_pose,
            250,
            &|c, seed| {
                let mut r = Rng(0x30f2_b0a5_1c3d ^ seed as u64);
                scramble(c, &mut r);

                c.g[GP][0] = GAME_GP as u64;
                // No animation channels are registered for this frame, so the
                // sampler's channel lookups take their short paths.
                c.bus.write32(GAME_GP + 0xd08, CHANS);
                for i in 0..16 {
                    c.bus.write32(CHANS + 4 * i, 0);
                }

                let bones = 1 + seed % 2;
                c.g[A0][0] = MODEL as u64;
                c.g[A1][0] = bones as u64;
                c.g[A2][0] = STATES as u64;
                c.g[A3][0] = (seed % 2) as u64;

                // The model: bone list, per-bone arrays and the two blend masks.
                c.bus.write32(MODEL + 0x8, bones);
                c.bus.write32(MODEL + 0xc, BONES);
                c.bus.write32(MODEL + 0x10, r.next_u32() % 32);
                c.bus.write32(MODEL + 0x14, r.next_u32() % 4);
                c.bus.write32(MODEL + 0x1c, WORDS);
                c.bus.write32(MODEL + 0x20, FLOATS);
                c.bus.write32(MODEL + 0x24, VEC_A);
                c.bus.write32(MODEL + 0x28, VEC_B);
                c.bus.write32(MODEL + 0x3c, MATS);
                let bits = if seed % 2 == 0 { 0u64 } else { u64::MAX };
                c.bus.write64(MODEL + 0x150, bits);
                c.bus
                    .write64(MODEL + 0x158, if seed % 3 == 0 { 0 } else { bits });
                for i in 0..16 {
                    c.bus.write32(WORDS + 4 * i, r.next_u32() % 8);
                    c.bus.write32(FLOATS + 4 * i, r.edge_f32());
                    put_vec(
                        c,
                        VEC_A + 16 * i,
                        [r.edge_f32(), r.edge_f32(), r.edge_f32(), 0x3f80_0000],
                    );
                    put_vec(
                        c,
                        VEC_B + 16 * i,
                        [r.edge_f32(), r.edge_f32(), r.edge_f32(), 0x3f80_0000],
                    );
                    put_vec(
                        c,
                        MATS + 16 * i,
                        [r.edge_f32(), r.edge_f32(), r.edge_f32(), 0x3f80_0000],
                    );
                }

                for b in 0..bones {
                    let bone = BONES + 0x50 * b;
                    c.bus.write32(bone + 0x4, r.next_u32() % 3);
                    c.bus.write32(bone + 0x8, r.next_u32() % 32);
                    c.bus.write32(bone + 0xc, r.next_u32() % 8);
                    c.bus.write32(bone + 0x10, r.next_u32() % 4);
                    c.bus.write32(bone + 0x38, AUX + 0x100 * b);
                    c.bus.write32(bone + 0x40, IDX + 0x40 * b);
                    // No channel samples for this bone: the sample path reaches
                    // `AnimDataBase_SampleChannel`, which needs a populated
                    // animation-cache/heap fixture (known runaway PC 0x31ef40).
                    c.bus.write32(bone + 0x44, 0);
                    c.bus.write32(bone + 0x4c, r.next_u32() % 3);
                    for k in 0..4 {
                        c.bus.write32(AUX + 0x100 * b + 4 * k, r.next_u32() % 4);
                        c.bus.write32(IDX + 0x40 * b + 4 * k, r.next_u32() % 8);
                    }

                    // The animation state this bone blends from.
                    let st = STATES + 0x50 * b;
                    for k in 0..20 {
                        c.bus.write32(st + 4 * k, r.next_u32() % 8);
                    }
                    // channel index (low byte) and byte offset (high 24 bits)
                    c.bus
                        .write32(st, (r.next_u32() & 0xff) | ((r.next_u32() % 0x40) << 8));
                    c.bus.write32(st + 0x8, b);
                    c.bus.write64(
                        st + 0x10,
                        if seed % 2 == 0 {
                            0
                        } else {
                            1u64 << (r.next_u32() % 64)
                        },
                    );
                    c.bus.write16(st + 0x14, (r.next_u32() % 4) as u16);
                    c.bus.write16(st + 0x16, (r.next_u32() % 4) as u16);
                    for k in 0x18..0x1c {
                        c.bus.write8(st + k, (r.next_u32() % 6) as u8);
                    }
                    for k in 0..4 {
                        c.bus.write32(st + 0x40 + 4 * k, r.edge_f32());
                    }
                }
            },
        );
    }
}
