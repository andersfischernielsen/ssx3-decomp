//! Native Rider_LaunchIntoAir (0x00114298).
//!
//! A0 is Rider*, F12 is jump charge. Negative charge requests the default
//! impulse and resets the owner's jump stamp; nonnegative charge is squared,
//! scaled by entry speed and clamped. This routine also performs slope
//! shaping, contact separation, trick/camera notifications and speed limiting.
//! It does not switch the rider's main or sub state. Callers that supply an
//! authored lift velocity must arrange the Ground -> Air transition separately.
//!
//! The VU helpers below retain the original operations and scratch registers;
//! host float normalization changes takeoff results on the PS2 edge cases.
//! Authority: generated/ee/s00/src/lib.rs, f_00114298. The C++ declarations
//! confirm (Rider*, float); no readable C++ body was available for this entry.

use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

use crate::call::{call_ptr, guest, Flow, Frame};

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0011_4298, rider_launch_into_air, "Rider_LaunchIntoAir")];

/// Scalar dot product of vf3 and vf{rhs}, including the original W lane.
/// Leaves vf4.x as the result, vf6.x as 1, and the same accumulator contents.
#[inline]
fn vu_dot(c: &mut Cpu, rhs: u8) {
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(rhs));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

#[inline]
fn vu_normalize(c: &mut Cpu, input: u32, output: u32) {
    c.lqc2(3, input);
    vu_dot(c, 3);
    c.vu0().rsqrt(0, 3, 4, 0);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 5, 3, Src::Q);
    c.sqc2(5, output);
}

#[inline]
fn vu_length(c: &mut Cpu, input: u32, scratch: usize, result: usize) {
    c.lqc2(3, input);
    vu_dot(c, 3);
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(scratch, 22);
    c.mtc1(scratch, result);
}

#[inline]
fn vu_scale(c: &mut Cpu, input: u32, scalar: usize, output: u32) {
    c.lqc2(4, input);
    c.qmtc2(scalar, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, output);
}

#[inline]
fn vu_add(c: &mut Cpu, left: u32, right: u32, output: u32) {
    c.lqc2(3, left);
    c.lqc2(4, right);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, output);
}

#[inline]
fn vu_sub(c: &mut Cpu, left: u32, right: u32, output: u32) {
    c.lqc2(3, left);
    c.lqc2(4, right);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, output);
}

/// Guest ABI entry. F12 is a charge scalar, not a desired velocity.
pub fn rider_launch_into_air(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x130);
        let sp = c.g[SP][0] as u32;
        let rider = c.g[A0][0] as u32;
        let gp = c.g[GP][0] as u32;

        c.mtc1(0, 0);
        frame.save(c, S0, 0xe0);
        frame.save(c, S1, 0xd0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 26, 0x120);
        c.g[S1][0] = 0;
        frame.save_ra(c, 0xc0);
        c.mov_s(26, 12);
        frame.save_fpr(c, 27, 0x128);
        c.c_lt_s(26, 0);
        frame.save_fpr(c, 25, 0x118);
        frame.save_fpr(c, 24, 0x110);
        frame.save_fpr(c, 23, 0x108);
        frame.save_fpr(c, 22, 0x100);
        frame.save_fpr(c, 21, 0xf8);
        frame.save_fpr(c, 20, 0xf0);

        // f23 is entry speed; the original velocity remains in slot 0 for
        // the later cancellation check. The FPU comparison survives VU math.
        vu_length(c, rider.wrapping_add(0x1e0), V0, 23);
        c.lq(V0, rider.wrapping_add(0x1e0));
        let default_jump = c.fcc();
        c.sq(V0, sp);
        if default_jump {
            c.cycles += 5;
            c.g[V1][0] = c.lw(rider.wrapping_add(0x77c));
            c.g[V0][0] = u64::MAX;
            c.lwc1(21, gp.wrapping_sub(0x7bf0));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
        } else {
            c.cycles += 5;
            c.lwc1(0, gp.wrapping_sub(0x7bec));
            c.c_lt_s(23, 0);
            let low_speed = c.fcc();
            c.mul_s(21, 26, 26);
            if low_speed {
                c.cycles += 5;
                c.lwc1(0, gp.wrapping_sub(0x7be8));
                c.lwc1(1, gp.wrapping_sub(0x7be4));
                c.mul_s(0, 23, 0);
                c.add_s(0, 0, 1);
            } else {
                c.cycles += 1;
                c.lwc1(0, gp.wrapping_sub(0x7be0));
            }
            c.cycles += 7;
            c.mul_s(21, 21, 0);
            c.lwc1(0, gp.wrapping_sub(0x7bdc));
            c.lwc1(1, gp.wrapping_sub(0x7bd8));
            c.c_le_s(0, 21);
            if c.fcc() {
                c.min_s(0, 21, 1);
            }
            c.cycles += 1;
            c.mov_s(21, 0);
        }

        apply_jump_impulse(c, rider, sp, gp)?;
        notify_and_correct_velocity(c, rider, sp, gp)?;
        separate_contact_and_limit_speed(c, rider, sp, gp);
        apply_landing_decay(c, rider, gp)?;
        record_takeoff_quantity(c, rider, sp);

        c.cycles += 5;
        c.mtc1(0, 0);
        c.c_lt_s(0, 26);
        let charged_jump = c.fcc();
        c.g[V0][0] = 1;
        if !charged_jump {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 14;
        c.sw(rider.wrapping_add(0x5a8), c.g[V0][0]);
        frame.restore(c, S0, 0xe0);
        frame.restore(c, S1, 0xd0);
        frame.restore_ra(c, 0xc0);
        frame.restore_fpr(c, 27, 0x128);
        frame.restore_fpr(c, 26, 0x120);
        frame.restore_fpr(c, 25, 0x118);
        frame.restore_fpr(c, 24, 0x110);
        frame.restore_fpr(c, 23, 0x108);
        frame.restore_fpr(c, 22, 0x100);
        frame.restore_fpr(c, 21, 0xf8);
        frame.restore_fpr(c, 20, 0xf0);
        frame.close(c);
        Ok(())
    });
}

/// Rail exits launch along the rider up axis. Other jumps blend the contact
/// normal and slope tangent, preserving the original slope-dependent shaping.
fn apply_jump_impulse(c: &mut Cpu, rider: u32, sp: u32, gp: u32) -> Flow {
    'impulse_applied: {
        'ground_launch: {
            c.cycles += 12;
            c.lwc1(12, gp.wrapping_sub(0x7bd4));
            c.g[RA][0] = sx(0x0011437c);
            call_ptr(c, 0x0031c040, 0x0011437c)?;
            c.lwc1(12, gp.wrapping_sub(0x7bd0));
            c.g[RA][0] = sx(0x00114388);
            c.mov_s(20, 0);
            call_ptr(c, 0x0031c040, 0x00114388)?;
            c.mov_s(27, 0);
            c.g[RA][0] = sx(0x00114394);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0011fe98, 0x00114394)?;
            c.g[V1][0] = 0x4;
            if c.g[V0][0] != c.g[V1][0] {
                c.lwc1(2, rider.wrapping_add(0x378));
                break 'ground_launch;
            } else {
                c.cycles += 15;
                c.lq(2, rider.wrapping_add(0x1c0));
                c.sq(2, sp.wrapping_add(0x20));
                c.mfc1(3, 21);
                vu_scale(c, sp.wrapping_add(0x20), 3, sp.wrapping_add(0x30));
                c.lq(2, sp.wrapping_add(0x30));
                c.sq(2, sp.wrapping_add(0x10));
                vu_add(
                    c,
                    rider.wrapping_add(0x1e0),
                    sp.wrapping_add(0x10),
                    rider.wrapping_add(0x1e0),
                );
                c.g[V0][0] = c.lw(gp.wrapping_sub(0x848));
                break 'impulse_applied;
            }
        }
        'reshape_tangent: {
            'add_simple_impulse: {
                c.cycles += 37;
                c.sub_s(1, 27, 20);
                c.lwc1(0, gp.wrapping_sub(0x7bcc));
                c.mfc1(2, 0);
                vu_scale(c, rider.wrapping_add(0x3a0), 2, sp.wrapping_add(0x40));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 24);
                c.lq(2, sp.wrapping_add(0x40));
                c.sub_s(2, 2, 20);
                c.div_s(1, 24, 1);
                c.sq(2, sp.wrapping_add(0x30));
                vu_add(
                    c,
                    rider.wrapping_add(0x370),
                    sp.wrapping_add(0x30),
                    sp.wrapping_add(0x40),
                );
                c.mtc1(0, 22);
                c.lq(2, sp.wrapping_add(0x40));
                c.sq(2, sp.wrapping_add(0x20));
                vu_normalize(c, sp.wrapping_add(0x20), sp.wrapping_add(0x40));
                c.mul_s(25, 2, 1);
                c.lwc1(0, rider.wrapping_add(0x3a8));
                c.lq(2, sp.wrapping_add(0x40));
                c.c_lt_s(0, 22);
                let t = c.fcc();
                c.sq(2, sp.wrapping_add(0x10));
                if t {
                    break 'add_simple_impulse;
                } else {
                    c.cycles += 3;
                    c.c_lt_s(25, 22);
                    let t = c.fcc();

                    if t {
                        break 'add_simple_impulse;
                    } else {
                        c.cycles += 5;
                        c.lwc1(0, rider.wrapping_add(0x1e8));
                        c.c_lt_s(0, 22);
                        if !c.fcc() {
                            c.lwc1(1, rider.wrapping_add(0x370));
                            break 'reshape_tangent;
                        } else {
                            break 'add_simple_impulse;
                        }
                    }
                }
            }
            c.cycles += 13;
            c.mfc1(3, 21);
            vu_scale(c, sp.wrapping_add(0x10), 3, sp.wrapping_add(0x30));
            c.lq(2, sp.wrapping_add(0x30));
            c.sq(2, sp.wrapping_add(0x20));
            vu_add(
                c,
                rider.wrapping_add(0x1e0),
                sp.wrapping_add(0x20),
                rider.wrapping_add(0x1e0),
            );
            c.g[V0][0] = c.lw(gp.wrapping_sub(0x848));
            break 'impulse_applied;
        }
        'slope_blend_ready: {
            c.cycles += 101;
            c.mov_s(12, 22);
            c.lwc1(0, rider.wrapping_add(0x374));
            c.swc1(1, sp.wrapping_add(0x30));
            c.swc1(0, sp.wrapping_add(0x34));
            c.swc1(22, sp.wrapping_add(0x38));
            c.swc1(22, sp.wrapping_add(0x3c));
            vu_normalize(c, sp.wrapping_add(0x30), sp.wrapping_add(0x40));
            c.lq(2, sp.wrapping_add(0x40));
            c.sq(2, sp.wrapping_add(0x20));
            c.lqc2(3, rider.wrapping_add(0x3a0));
            c.lqc2(5, sp.wrapping_add(0x20));
            vu_dot(c, 5);
            c.qmfc2(3, 4);
            c.mtc1(3, 0);
            c.mfc1(4, 0);
            vu_scale(c, sp.wrapping_add(0x20), 4, sp.wrapping_add(0x60));
            c.lq(2, sp.wrapping_add(0x60));
            c.sq(2, sp.wrapping_add(0x50));
            vu_sub(
                c,
                rider.wrapping_add(0x3a0),
                sp.wrapping_add(0x50),
                sp.wrapping_add(0x60),
            );
            c.lq(2, sp.wrapping_add(0x60));
            c.sq(2, sp.wrapping_add(0x40));
            vu_normalize(c, sp.wrapping_add(0x40), sp.wrapping_add(0x60));
            c.lq(2, sp.wrapping_add(0x60));
            c.sq(2, sp.wrapping_add(0x30));
            c.lqc2(3, rider.wrapping_add(0x1e0));
            c.lqc2(5, sp.wrapping_add(0x20));
            vu_dot(c, 5);
            c.qmfc2(3, 4);
            c.mtc1(3, 0);
            c.mfc1(4, 0);
            vu_scale(c, sp.wrapping_add(0x20), 4, sp.wrapping_add(0x70));
            c.lq(2, sp.wrapping_add(0x70));
            c.sq(2, sp.wrapping_add(0x60));
            vu_sub(
                c,
                rider.wrapping_add(0x1e0),
                sp.wrapping_add(0x60),
                sp.wrapping_add(0x70),
            );
            c.lq(2, sp.wrapping_add(0x70));
            c.sq(2, sp.wrapping_add(0x50));
            vu_normalize(c, sp.wrapping_add(0x50), sp.wrapping_add(0x70));
            c.lq(2, sp.wrapping_add(0x70));
            c.g[RA][0] = sx(0x00114620);
            c.sq(2, sp.wrapping_add(0x40));
            call_ptr(c, 0x0031bf60, 0x00114620)?;
            c.lwc1(12, gp.wrapping_sub(0x7bc8));
            c.g[RA][0] = sx(0x0011462c);
            c.mov_s(20, 0);
            call_ptr(c, 0x0031bf60, 0x0011462c)?;
            c.sub_s(0, 0, 20);
            c.lwc1(1, sp.wrapping_add(0x38));
            c.lwc1(4, gp.wrapping_sub(0x7bc4));
            c.sub_s(1, 1, 20);
            c.div_s(0, 24, 0);
            c.mul_s(1, 25, 1);
            c.mul_s(3, 1, 0);
            c.c_le_s(22, 3);
            let t = !c.fcc();
            c.mov_s(2, 22);
            if t {
                break 'slope_blend_ready;
            } else {
                c.cycles += 1;
                c.min_s(2, 3, 4);
                break 'slope_blend_ready;
            }
        }
        c.cycles += 81;
        c.mov_s(3, 2);
        c.g[S1][0] = 0x1;
        c.sub_s(0, 24, 3);
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 2);
        c.mfc1(2, 0);
        vu_scale(c, sp.wrapping_add(0x10), 2, sp.wrapping_add(0x80));
        c.mul_s(1, 3, 23);
        c.lq(2, sp.wrapping_add(0x80));
        c.sq(2, sp.wrapping_add(0x70));
        c.mfc1(3, 3);
        vu_scale(c, sp.wrapping_add(0x30), 3, sp.wrapping_add(0x90));
        c.lq(2, sp.wrapping_add(0x90));
        c.sq(2, sp.wrapping_add(0x80));
        vu_add(
            c,
            sp.wrapping_add(0x70),
            sp.wrapping_add(0x80),
            sp.wrapping_add(0x90),
        );
        c.lq(2, sp.wrapping_add(0x90));
        c.sq(2, sp.wrapping_add(0x60));
        vu_normalize(c, sp.wrapping_add(0x60), sp.wrapping_add(0x90));
        c.lq(2, sp.wrapping_add(0x90));
        c.sq(2, sp.wrapping_add(0x10));
        c.sq(2, sp.wrapping_add(0x50));
        c.mfc1(3, 0);
        vu_scale(c, rider.wrapping_add(0x1e0), 3, sp.wrapping_add(0x80));
        c.lq(2, sp.wrapping_add(0x80));
        c.sq(2, sp.wrapping_add(0x70));
        c.mfc1(3, 1);
        vu_scale(c, sp.wrapping_add(0x40), 3, sp.wrapping_add(0x90));
        c.lq(2, sp.wrapping_add(0x90));
        c.sq(2, sp.wrapping_add(0x80));
        vu_add(
            c,
            sp.wrapping_add(0x70),
            sp.wrapping_add(0x80),
            sp.wrapping_add(0x90),
        );
        c.lq(2, sp.wrapping_add(0x90));
        c.sq(2, sp.wrapping_add(0x60));
        c.mfc1(3, 21);
        vu_scale(c, sp.wrapping_add(0x10), 3, sp.wrapping_add(0xb0));
        c.lq(2, sp.wrapping_add(0xb0));
        c.sq(2, sp.wrapping_add(0xa0));
        c.mfc1(3, 2);
        vu_scale(c, sp.wrapping_add(0xa0), 3, sp.wrapping_add(0xb0));
        c.lq(2, sp.wrapping_add(0xb0));
        c.sq(2, sp.wrapping_add(0x90));
        vu_add(
            c,
            sp.wrapping_add(0x60),
            sp.wrapping_add(0x90),
            sp.wrapping_add(0xb0),
        );
        c.lq(2, sp.wrapping_add(0xb0));
        c.sq(2, rider.wrapping_add(0x1e0));
        c.sq(2, sp.wrapping_add(0x50));
        c.g[V0][0] = c.lw(gp.wrapping_sub(0x848));
        break 'impulse_applied;
    }
    Ok(())
}

/// Notify camera/trick systems, award jump boost, then correct a launch
/// impulse that would remove too much horizontal speed on a steep surface.
fn notify_and_correct_velocity(c: &mut Cpu, rider: u32, sp: u32, gp: u32) -> Flow {
    'velocity_loss_corrected: {
        c.cycles += 26;
        c.g[A2][0] = c.lw(rider.wrapping_add(0x790));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
        c.g[A1][0] = c.lw(rider.wrapping_add(0x86c));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xfc));
        c.g[RA][0] = sx(0x001147b8);
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
        call_ptr(c, 0x002707e0, 0x001147b8)?;
        c.g[RA][0] = sx(0x001147c0);
        c.g[A0][0] = c.lw(gp.wrapping_add(0xce8));
        call_ptr(c, 0x00309848, 0x001147c0)?;
        c.g[V0][0] = c.lw(rider.wrapping_add(0x324));
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = c.lw(rider.wrapping_add(0x320));
        c.g[A0][0] = c.lw(rider.wrapping_add(0x790));
        c.g[A1][0] = c.g[A1][0] ^ c.g[V0][0];
        c.g[RA][0] = sx(0x001147dc);
        c.g[A1][0] = (0u64 < c.g[A1][0]) as u64;
        call_ptr(c, 0x00119e38, 0x001147dc)?;
        c.mov_s(12, 0);
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x001147ec);
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0010e098, 0x001147ec)?;
        c.lwc1(0, gp.wrapping_sub(0x7bc0));
        c.lwc1(1, rider.wrapping_add(0x378));
        c.add_s(0, 27, 0);
        c.c_lt_s(0, 1);
        if !c.fcc() {
            c.lq(2, rider.wrapping_add(0x380));
            break 'velocity_loss_corrected;
        } else {
            c.cycles += 24;
            vu_sub(c, rider.wrapping_add(0x1e0), sp, sp.wrapping_add(0x20));
            c.lq(2, sp);
            c.lq(3, sp.wrapping_add(0x20));
            c.sq(2, sp.wrapping_add(0x20));
            c.sw(sp.wrapping_add(0x28), 0u64);
            c.sq(3, sp.wrapping_add(0x10));
            c.lwc1(4, sp.wrapping_add(0x28));
            c.sw(sp.wrapping_add(0x18), 0u64);
            c.lqc2(3, sp.wrapping_add(0x20));
            c.lqc2(5, sp.wrapping_add(0x10));
            vu_dot(c, 5);
            c.qmfc2(2, 4);
            c.mtc1(2, 0);
            c.c_lt_s(0, 4);
            if !c.fcc() {
                c.lq(2, rider.wrapping_add(0x380));
                break 'velocity_loss_corrected;
            } else {
                'speed_blend_ready: {
                    'below_half_max: {
                        c.cycles += 9;
                        c.lwc1(0, rider.wrapping_add(0x2e4));
                        c.g[AT][0] = 0x3f000000;
                        c.mtc1(1, 1);
                        c.lwc1(2, gp.wrapping_sub(0x7bbc));
                        c.mul_s(3, 0, 1);
                        c.c_le_s(3, 23);
                        let t = !c.fcc();
                        c.mul_s(0, 0, 2);
                        if t {
                            break 'below_half_max;
                        } else {
                            'above_blend_range: {
                                c.cycles += 3;
                                c.c_le_s(23, 0);
                                let t = !c.fcc();

                                if t {
                                    break 'above_blend_range;
                                } else {
                                    c.cycles += 5;
                                    c.sub_s(1, 0, 3);
                                    c.sub_s(0, 23, 3);
                                    c.div_s(0, 0, 1);

                                    break 'speed_blend_ready;
                                }
                            }
                            c.cycles += 4;
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 0);

                            break 'speed_blend_ready;
                        }
                    }
                    c.cycles += 1;
                    c.mov_s(0, 4);
                    break 'speed_blend_ready;
                }
                c.cycles += 12;
                c.mfc1(3, 0);
                vu_scale(c, sp.wrapping_add(0x10), 3, sp.wrapping_add(0x40));
                c.lq(2, sp.wrapping_add(0x40));
                c.sq(2, sp.wrapping_add(0x30));
                vu_sub(
                    c,
                    rider.wrapping_add(0x1e0),
                    sp.wrapping_add(0x30),
                    rider.wrapping_add(0x1e0),
                );
                c.lq(2, rider.wrapping_add(0x380));
                break 'velocity_loss_corrected;
            }
        }
    }
    Ok(())
}

/// Keep the contact escape direction at rider+0x3c0, separate the rider
/// from the surface where needed, and clamp the resulting speed.
fn separate_contact_and_limit_speed(c: &mut Cpu, rider: u32, sp: u32, gp: u32) {
    'lateral_contact_ready: {
        'fallback_contact: {
            'normalize_contact: {
                c.cycles += 10;
                c.lwc1(2, rider.wrapping_add(0x388));
                c.sq(2, sp.wrapping_add(0x10));
                c.sw(sp.wrapping_add(0x18), 0u64);
                c.lwc1(0, sp.wrapping_add(0x10));
                c.lwc1(1, sp.wrapping_add(0x18));
                c.abs_s(0, 0);
                c.c_lt_s(1, 0);
                let t = c.fcc();
                c.lwc1(0, sp.wrapping_add(0x14));
                if t {
                    break 'normalize_contact;
                } else {
                    c.cycles += 5;
                    c.abs_s(0, 0);
                    c.c_lt_s(1, 0);
                    let t = !c.fcc();
                    c.g[V0][0] = 0x500000;
                    if t {
                        break 'fallback_contact;
                    } else {
                        break 'normalize_contact;
                    }
                }
            }
            c.cycles += 19;
            vu_length(c, sp.wrapping_add(0x10), 2, 0);
            c.mfc1(3, 0);
            c.qmtc2(3, 3);
            c.vu0().div(0, 3, 3, 0);
            c.lqc2(4, sp.wrapping_add(0x10));
            c.vu0().waitq();
            c.vu0().mul(0b1111, 5, 4, Src::Q);
            c.sqc2(5, sp.wrapping_add(0x10));

            break 'lateral_contact_ready;
        }
        c.cycles += 3;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xec0));
        c.lq(3, (c.g[V0][0] as u32));
        c.sq(3, sp.wrapping_add(0x10));
        break 'lateral_contact_ready;
    }
    'contact_adjusted: {
        'load_speed_limit: {
            c.cycles += 6;
            c.lwc1(0, gp.wrapping_sub(0x7bb8));
            c.lq(2, sp.wrapping_add(0x10));
            c.c_lt_s(2, 0);
            let t = !c.fcc();
            c.sq(2, rider.wrapping_add(0x3c0));
            if t {
                break 'load_speed_limit;
            } else {
                c.cycles += 5;
                c.lwc1(0, gp.wrapping_sub(0x7bb4));
                c.c_lt_s(0, 2);
                if !c.fcc() {
                    c.lwc1(1, rider.wrapping_add(0x2e4));
                    break 'contact_adjusted;
                } else {
                    c.cycles += 47;
                    c.lqc2(3, rider.wrapping_add(0x1e0));
                    c.lqc2(5, sp.wrapping_add(0x10));
                    vu_dot(c, 5);
                    c.qmfc2(2, 4);
                    c.mtc1(2, 0);
                    c.lqc2(3, sp.wrapping_add(0x10));
                    c.lqc2(5, sp.wrapping_add(0x10));
                    vu_dot(c, 5);
                    c.qmfc2(3, 4);
                    c.mtc1(3, 1);
                    c.div_s(0, 0, 1);
                    c.g[AT][0] = 0x40800000;
                    c.mtc1(1, 2);
                    c.mfc1(3, 0);
                    vu_scale(c, sp.wrapping_add(0x10), 3, sp.wrapping_add(0x30));
                    c.lq(2, sp.wrapping_add(0x30));
                    c.sq(2, sp.wrapping_add(0x20));
                    vu_sub(
                        c,
                        rider.wrapping_add(0x1e0),
                        sp.wrapping_add(0x20),
                        rider.wrapping_add(0x1e0),
                    );
                    c.mfc1(3, 2);
                    vu_scale(c, sp.wrapping_add(0x10), 3, sp.wrapping_add(0x40));
                    c.lq(2, sp.wrapping_add(0x40));
                    c.sq(2, sp.wrapping_add(0x30));
                    vu_add(
                        c,
                        rider.wrapping_add(0x110),
                        sp.wrapping_add(0x30),
                        rider.wrapping_add(0x110),
                    );
                    c.g[V0][0] = c.lh(rider.wrapping_add(0x2d4));
                    c.g[V0][0] = c.g[V0][0] & 0x20;
                    if c.g[V0][0] != 0u64 {
                        c.lwc1(1, rider.wrapping_add(0x2e4));
                        break 'contact_adjusted;
                    } else {
                        c.cycles += 6;
                        c.lwc1(0, gp.wrapping_sub(0x7bb0));
                        c.mfc1(2, 0);
                        vu_scale(c, rider.wrapping_add(0x1e0), 2, rider.wrapping_add(0x1e0));
                        break 'load_speed_limit;
                    }
                }
            }
        }
        c.cycles += 1;
        c.lwc1(1, rider.wrapping_add(0x2e4));
        break 'contact_adjusted;
    }
    'speed_limited: {
        c.cycles += 13;
        vu_length(c, rider.wrapping_add(0x1e0), 2, 0);
        c.c_lt_s(1, 0);
        let t = !c.fcc();

        if t {
            break 'speed_limited;
        } else {
            c.cycles += 6;
            c.div_s(0, 1, 0);
            c.mfc1(2, 0);
            vu_scale(c, rider.wrapping_add(0x1e0), 2, rider.wrapping_add(0x1e0));
            break 'speed_limited;
        }
    }
}

/// Apply the original frame-counter-dependent jump velocity attenuation.
/// Default launches and the three excluded rider modes bypass attenuation.
fn apply_landing_decay(c: &mut Cpu, rider: u32, gp: u32) -> Flow {
    'landing_decay_applied: {
        c.cycles += 5;
        c.mtc1(0, 0);
        c.c_le_s(0, 26);
        let t = !c.fcc();
        c.g[V0][0] = 0x500000;
        if t {
            break 'landing_decay_applied;
        } else {
            c.cycles += 5;
            c.g[V0][0] = c.lw(rider.wrapping_add(0x434));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xb));
            c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = 0x500000;
            if t {
                break 'landing_decay_applied;
            } else {
                'decay_factor_ready: {
                    'interpolate_decay: {
                        c.cycles += 14;
                        c.g[RA][0] = sx(0x00114b04);
                        call_ptr(c, 0x001298c8, 0x00114b04)?;
                        c.g[A0][0] = c.lw(rider.wrapping_add(0x77c));
                        c.lwc1(1, gp.wrapping_sub(0x7bac));
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
                        c.lwc1(2, gp.wrapping_sub(0x7ba8));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                        c.mtc1(2, 0);
                        c.cvt_s_w(0, 0);
                        c.mul_s(0, 0, 1);
                        c.c_le_s(0, 2);
                        if !c.fcc() {
                            c.sub_s(1, 0, 2);
                            break 'interpolate_decay;
                        } else {
                            c.cycles += 3;
                            c.lwc1(0, gp.wrapping_sub(0x7ba4));

                            break 'decay_factor_ready;
                        }
                    }
                    c.cycles += 9;
                    c.lwc1(3, gp.wrapping_sub(0x7ba0));
                    c.lwc1(2, gp.wrapping_sub(0x7b9c));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 0);
                    c.mul_s(1, 1, 2);
                    c.min_s(1, 1, 0);
                    c.sub_s(0, 0, 1);
                    c.mul_s(0, 0, 3);
                    c.add_s(0, 1, 0);
                    break 'decay_factor_ready;
                }
                c.cycles += 7;
                c.mfc1(2, 0);
                vu_scale(c, rider.wrapping_add(0x1e0), 2, rider.wrapping_add(0x1e0));
                c.g[V0][0] = 0x500000;
                c.mtc1(0, 0);
                break 'landing_decay_applied;
            }
        }
    }
    Ok(())
}

/// Record the signed takeoff component relative to the ground normal.
/// The original uses two cross products rather than a host trig expression.
fn record_takeoff_quantity(c: &mut Cpu, rider: u32, sp: u32) {
    'takeoff_metric_ready: {
        c.cycles += 23;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xea0));
        c.lqc2(4, (c.g[V0][0] as u32));
        c.lqc2(5, rider.wrapping_add(0x1e0));
        c.vu0().opmula(4, 5);
        c.vu0().opmsub(6, 5, 4);
        c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
        c.sqc2(6, sp.wrapping_add(0x30));
        c.lq(2, sp.wrapping_add(0x30));
        c.sq(2, sp.wrapping_add(0x20));
        vu_length(c, sp.wrapping_add(0x20), 3, 1);
        c.c_lt_s(0, 1);
        if !c.fcc() {
            c.swc1(0, rider.wrapping_add(0x5a4));
            break 'takeoff_metric_ready;
        } else {
            c.cycles += 37;
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 0);
            c.div_s(0, 0, 1);
            c.mfc1(2, 0);
            vu_scale(c, sp.wrapping_add(0x20), 2, sp.wrapping_add(0x20));
            c.lqc2(4, sp.wrapping_add(0x20));
            c.lqc2(5, rider.wrapping_add(0x370));
            c.vu0().opmula(4, 5);
            c.vu0().opmsub(6, 5, 4);
            c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
            c.sqc2(6, sp.wrapping_add(0x40));
            c.lq(2, sp.wrapping_add(0x40));
            c.lqc2(3, rider.wrapping_add(0x1e0));
            c.lqc2(5, rider.wrapping_add(0x370));
            vu_dot(c, 5);
            c.qmfc2(3, 4);
            c.mtc1(3, 0);
            c.sq(2, sp.wrapping_add(0x30));
            vu_length(c, sp.wrapping_add(0x30), 3, 1);
            c.mul_s(0, 0, 1);
            c.swc1(0, rider.wrapping_add(0x5a4));
            break 'takeoff_metric_ready;
        }
    }
}
