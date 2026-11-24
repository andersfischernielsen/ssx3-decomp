//! Staging module oc448 (wave wave224): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc448`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0013_2a30,
    rider_handplant_sub_handle_input,
    "RiderHandplantSub_HandleInput",
)];

// The handplant sub-state (`self`) fields.

/// Seconds spent in the handplant (float, `EnterSec`).
const HPI_ENTER_SEC: u32 = 0x00;
/// The balance the rider leans to (float, `CurBalance`).
const HPI_BALANCE: u32 = 0x04;
/// `IsMissedHandplant`: a missed handplant gets no boost.
const HPI_MISSED: u32 = 0x08;
/// `IsRailSpline`: the handplant started on a rail spline.
const HPI_RAIL_SPLINE: u32 = 0x0c;
/// `HandplantType`: type 1 mirrors the balance sign.
const HPI_TYPE: u32 = 0x10;
/// `mRider`.
const HPI_RIDER: u32 = 0x14;

// The rider's fields.

/// `mRiderAnim`.
const HPI_R_ANIM: u32 = 0x784;
/// `mAirPredictor`.
const HPI_R_AIR: u32 = 0x788;
/// `mMetrix`.
const HPI_R_METRIX: u32 = 0x790;
/// `HandplantMode`: 1 landing, 2 boosted, 3 plain, 4..6 exit poses.
const HPI_R_MODE: u32 = 0x32c;
/// `mTimeScale` (float).
const HPI_R_TIME_SCALE: u32 = 0x300;
/// `IsRidingFakie`.
const HPI_R_FAKIE: u32 = 0x320;
/// `mPosition`.
const HPI_R_POSITION: u32 = 0x110;

/// Slewed value `k` is three words (current, rate, target) at `0x1f0 + 12k`:
/// slot 0 at 0x1f0, slot 2 at 0x208, slot 7 at 0x244, slot 8 at 0x250.
const fn hpi_slew(k: u32) -> u32 {
    0x1f0 + 0xc * k
}

/// Slewed value 7: the balance (its rate and target are written here).
const HPI_SLEW7_RATE: u32 = hpi_slew(7) + 4;
const HPI_SLEW7_TARGET: u32 = hpi_slew(7) + 8;
/// Slewed value 8: the lean (current, rate and target).
const HPI_SLEW8_CUR: u32 = hpi_slew(8);
const HPI_SLEW8_RATE: u32 = hpi_slew(8) + 4;
const HPI_SLEW8_TARGET: u32 = hpi_slew(8) + 8;

#[inline]
fn hpi_self(c: &Cpu) -> u32 {
    c.g[S0][0] as u32
}

/// `self->mRider`, read from the sub-state as the generated code does.
#[inline]
fn hpi_rider(c: &mut Cpu) -> u64 {
    let at = hpi_self(c).wrapping_add(HPI_RIDER);
    Addr(at).lw(c)
}

#[inline]
fn hpi_f(c: &Cpu, r: usize) -> FpuF32 {
    FpuF32(c.f[r])
}

#[inline]
fn hpi_set(c: &mut Cpu, r: usize, v: FpuF32) {
    c.f[r] = v.0;
}

/// `void RiderHandplantSub_HandleInput(RiderHandplantSub* self, const Input* in)`
/// (0x132a30): the per-frame update of the handplant sub-state. A reset in
/// progress ends it. Landing mode (1) waits for the landing animation, then
/// starts the boosted (2) or the plain (3) handplant. Boosted mode (2) steers
/// the balance toward the stick and then picks an exit pose (4, 5 or 6). In
/// the other modes the handplant ends when the rider is back on the ground or
/// the animation has finished.
pub fn rider_handplant_sub_handle_input(c: &mut Cpu) {
    guest(c, hpi_body)
}

fn hpi_body(c: &mut Cpu) -> Flow {
    c.cycles += 16;
    let frame = Frame::open(c, 0x50);
    c.g[A2][0] = 0;
    frame.save(c, S0, 0x30);
    frame.save(c, S1, 0x20);
    c.g[S0][0] = c.g[A0][0];
    frame.save(c, S2, 0x10);
    c.g[S1][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    frame.save_fpr(c, 20, 0x40);
    // `if (cRider_checkForReset(rider, (flags >> 12) & 1, 0)) return;`
    c.g[A1][0] = Addr(c.g[S1][0] as u32).lw(c);
    c.g[A0][0] = hpi_rider(c);
    c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 12) as u32);
    c.g[A1][0] &= 0x1;
    call_ptr(c, 0x0011_6120, 0x0013_2a68)?;

    // Exits that restore s0 (`lq 16`) leave through the epilogue.
    'b_132f7c: {
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x30);
            break 'b_132f7c;
        }
        'b_132f78: {
            // Landing mode (1): wait for the landing animation, then start.
            'b_132b28: {
                c.cycles += 5;
                c.g[A0][0] = hpi_rider(c);
                c.g[S2][0] = 1;
                c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(HPI_R_MODE)).lw(c);
                let t = c.g[V1][0] != c.g[S2][0];
                c.g[V0][0] = 2;
                if t {
                    break 'b_132b28;
                }
                c.cycles += 5;
                c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(HPI_R_ANIM)).lw(c);
                c.g[A1][0] = 2;
                call_ptr(c, 0x0031_2ae8, 0x0013_2a90)?;
                if c.g[V0][0] == 0 {
                    frame.restore(c, S0, 0x30);
                    break 'b_132f7c;
                }
                'b_132b10: {
                    'b_132ae4: {
                        c.cycles += 4;
                        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c) & 0x2000;
                        let t = c.g[V0][0] == 0;
                        c.g[A0][0] = hpi_rider(c);
                        if t {
                            break 'b_132ae4;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = Addr(hpi_self(c).wrapping_add(HPI_MISSED)).lw(c);
                        let t = c.g[V0][0] != 0;
                        if t {
                            break 'b_132ae4;
                        }
                        // Boosted handplant: mode 2, start event 1.
                        c.cycles += 12;
                        c.g[V1][0] = 2;
                        c.g[A1][0] = 1;
                        let v = c.g[V1][0];
                        Addr((c.g[A0][0] as u32).wrapping_add(HPI_R_MODE)).sw(c, v);
                        c.g[V0][0] = hpi_rider(c);
                        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(HPI_R_METRIX)).lw(c);
                        call_ptr(c, 0x0011_9bf0, 0x0013_2acc)?;
                        c.g[A0][0] = hpi_rider(c);
                        c.f[12] = c.f[0];
                        c.g[A1][0] = 1;
                        call_ptr(c, 0x0010_e098, 0x0013_2adc)?;
                        c.g[V0][0] = hpi_rider(c);
                        break 'b_132b10;
                    }
                    // Plain handplant: mode 3, start event 2.
                    c.cycles += 11;
                    c.g[V1][0] = 3;
                    c.g[A1][0] = 2;
                    let v = c.g[V1][0];
                    Addr((c.g[A0][0] as u32).wrapping_add(HPI_R_MODE)).sw(c, v);
                    c.g[V0][0] = hpi_rider(c);
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(HPI_R_METRIX)).lw(c);
                    call_ptr(c, 0x0011_9bf0, 0x0013_2afc)?;
                    c.g[A0][0] = hpi_rider(c);
                    c.f[12] = c.f[0];
                    c.g[A1][0] = 1;
                    call_ptr(c, 0x0010_e098, 0x0013_2b0c)?;
                    c.g[V0][0] = hpi_rider(c);
                }
                // HandplantPlayAnim(self, mode, type), then the balance is zeroed.
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = Addr(hpi_self(c).wrapping_add(HPI_TYPE)).lw(c);
                c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(HPI_R_MODE)).lw(c);
                Callee {
                    addr: 0x0013_2fb8,
                    f: ee::s01::f_00132fb8,
                }
                .call(c, 0x0013_2b20)?;
                Addr(hpi_self(c)).sw(c, 0);
                break 'b_132f78;
            }

            // Boosted mode (2): steer the balance, then pick the exit pose.
            'b_132da8: {
                c.cycles += 2;
                let t = c.g[V1][0] != c.g[V0][0];
                if t {
                    break 'b_132da8;
                }
                'b_132d00: {
                    c.cycles += 10;
                    c.lwc1(2, (c.g[A0][0] as u32).wrapping_add(HPI_R_TIME_SCALE));
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x7560));
                    c.lwc1(0, hpi_self(c));
                    let dt = hpi_f(c, 2) * hpi_f(c, 1);
                    hpi_set(c, 8, dt);
                    let enter = hpi_f(c, 0) + hpi_f(c, 8);
                    hpi_set(c, 0, enter);
                    c.swc1(0, hpi_self(c));
                    c.g[A1][0] = Addr(c.g[S1][0] as u32).lw(c);
                    c.g[V0][0] = c.g[A1][0] & 0x2000;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = Addr(hpi_self(c).wrapping_add(HPI_RAIL_SPLINE)).lw(c);
                        break 'b_132d00;
                    }
                    'b_132cfc: {
                        c.cycles += 8;
                        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(HPI_SLEW8_CUR));
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(1, 1);
                        c.abs_s(0, 0);
                        c.c_lt_s(0, 1);
                        let t = !c.fcc();
                        c.g[V0][0] = sx((c.g[A1][0] as u32) << 12);
                        if t {
                            break 'b_132cfc;
                        }
                        // The stick's balance target (flags bits 14..19).
                        c.cycles += 7;
                        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x755c));
                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 26) as u32);
                        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(HPI_R_FAKIE)).lw(c);
                        c.mtc1(2, 0);
                        c.cvt_s_w(0, 0);
                        let t = c.g[V1][0] == 0;
                        let des = hpi_f(c, 0) * hpi_f(c, 1);
                        hpi_set(c, 2, des);
                        if !t {
                            c.cycles += 1;
                            let neg = -hpi_f(c, 2);
                            hpi_set(c, 2, neg);
                        }
                        // Balance type 1 mirrors the target.
                        c.cycles += 3;
                        c.g[V0][0] = Addr(hpi_self(c).wrapping_add(HPI_TYPE)).lw(c);
                        if c.g[V0][0] == c.g[S2][0] {
                            let neg = -hpi_f(c, 2);
                            hpi_set(c, 2, neg);
                        }
                        // Slew the balance toward the target, then the next balance.
                        c.cycles += 14;
                        c.g[AT][0] = 0x40a0_0000;
                        c.mtc1(1, 0);
                        c.g[AT][0] = 0x4040_0000;
                        c.mtc1(1, 1);
                        let v = hpi_f(c, 8) * hpi_f(c, 0);
                        hpi_set(c, 0, v);
                        c.swc1(2, (c.g[A0][0] as u32).wrapping_add(HPI_SLEW7_TARGET));
                        let v = hpi_f(c, 8) * hpi_f(c, 1);
                        hpi_set(c, 4, v);
                        c.swc1(0, (c.g[A0][0] as u32).wrapping_add(HPI_SLEW7_RATE));
                        let v = hpi_f(c, 2) + hpi_f(c, 4);
                        hpi_set(c, 1, v);
                        c.lwc1(3, hpi_self(c).wrapping_add(HPI_BALANCE));
                        c.c_lt_s(1, 3);
                        if c.fcc() {
                            c.cycles += 2;
                            let v = hpi_f(c, 3) - hpi_f(c, 4);
                            hpi_set(c, 1, v);
                        } else {
                            let v = hpi_f(c, 2) - hpi_f(c, 4);
                            hpi_set(c, 0, v);
                            c.cycles += 4;
                            c.c_lt_s(3, 0);
                            let t = !c.fcc();
                            c.mov_s(1, 2);
                            if !t {
                                c.cycles += 1;
                                let v = hpi_f(c, 3) + hpi_f(c, 4);
                                hpi_set(c, 1, v);
                            }
                        }
                        // The lean target follows the sign of the new balance (f1).
                        'b_132c90: {
                            'b_132c24: {
                                c.cycles += 6;
                                c.mtc1(0, 3);
                                c.mov_s(7, 1);
                                c.c_lt_s(7, 3);
                                let t = !c.fcc();
                                c.swc1(1, hpi_self(c).wrapping_add(HPI_BALANCE));
                                if t {
                                    break 'b_132c24;
                                }
                                c.cycles += 5;
                                c.lwc1(2, hpi_self(c).wrapping_add(HPI_ENTER_SEC));
                                c.g[AT][0] = 0xffff_ffff_bf80_0000;
                                c.mtc1(1, 7);
                                c.g[A0][0] = hpi_rider(c);
                                break 'b_132c90;
                            }
                            'b_132c48: {
                                c.cycles += 4;
                                c.c_lt_s(3, 7);
                                if !c.fcc() {
                                    c.lwc1(0, hpi_self(c).wrapping_add(HPI_ENTER_SEC));
                                    break 'b_132c48;
                                }
                                c.cycles += 5;
                                c.lwc1(2, hpi_self(c).wrapping_add(HPI_ENTER_SEC));
                                c.g[AT][0] = 0x3f80_0000;
                                c.mtc1(1, 7);
                                c.g[A0][0] = hpi_rider(c);
                                break 'b_132c90;
                            }
                            // Balance exactly zero: the lean decides the sign.
                            c.cycles += 6;
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x7558));
                            c.g[A0][0] = hpi_rider(c);
                            c.c_lt_s(1, 0);
                            let t = c.fcc();
                            c.mov_s(2, 0);
                            if !t {
                                c.cycles += 4;
                                c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(HPI_SLEW8_CUR));
                                c.c_eq_s(0, 3);
                                if c.fcc() {
                                    break 'b_132c90;
                                }
                            }
                            c.cycles += 6;
                            c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(HPI_SLEW8_CUR));
                            c.g[AT][0] = 0x3f80_0000;
                            c.mtc1(1, 7);
                            c.c_lt_s(0, 3);
                            if c.fcc() {
                                c.cycles += 2;
                                c.g[AT][0] = 0xffff_ffff_bf80_0000;
                                c.mtc1(1, 7);
                            }
                        }
                        // The lean target and rate from the balance and the lean.
                        c.cycles += 27;
                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7554));
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(1, 3);
                        let v = hpi_f(c, 2) * hpi_f(c, 0);
                        hpi_set(c, 0, v);
                        c.g[AT][0] = 0x40b0_0000;
                        c.mtc1(1, 1);
                        c.g[AT][0] = 0x4080_0000;
                        c.mtc1(1, 6);
                        c.g[AT][0] = 0x3fc0_0000;
                        c.mtc1(1, 4);
                        let v = hpi_f(c, 0).min(hpi_f(c, 3));
                        hpi_set(c, 0, v);
                        c.lwc1(2, (c.g[A0][0] as u32).wrapping_add(HPI_SLEW8_CUR));
                        let v = hpi_f(c, 3) - hpi_f(c, 0);
                        hpi_set(c, 5, v);
                        let v = hpi_f(c, 2).abs();
                        hpi_set(c, 2, v);
                        let v = hpi_f(c, 0) * hpi_f(c, 1);
                        hpi_set(c, 1, v);
                        c.swc1(7, (c.g[A0][0] as u32).wrapping_add(HPI_SLEW8_TARGET));
                        let v = hpi_f(c, 0) * hpi_f(c, 6);
                        hpi_set(c, 0, v);
                        let v = hpi_f(c, 5) * hpi_f(c, 4);
                        hpi_set(c, 4, v);
                        let v = hpi_f(c, 3) - hpi_f(c, 2);
                        hpi_set(c, 3, v);
                        let v = hpi_f(c, 0) + hpi_f(c, 5);
                        hpi_set(c, 0, v);
                        let v = hpi_f(c, 1) + hpi_f(c, 4);
                        hpi_set(c, 1, v);
                        let v = hpi_f(c, 0) * hpi_f(c, 3);
                        hpi_set(c, 0, v);
                        let v = hpi_f(c, 1) * hpi_f(c, 2);
                        hpi_set(c, 1, v);
                        let v = hpi_f(c, 1) + hpi_f(c, 0);
                        hpi_set(c, 1, v);
                        let v = hpi_f(c, 1) * hpi_f(c, 8);
                        hpi_set(c, 1, v);
                        c.swc1(1, (c.g[A0][0] as u32).wrapping_add(HPI_SLEW8_RATE));
                        break 'b_132f78;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = Addr(hpi_self(c).wrapping_add(HPI_RAIL_SPLINE)).lw(c);
                }
                // Exit pose: 6 on a rail spline near level, 4 when leaning
                // far left, else 5.
                'b_132d44: {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0;
                    c.g[V1][0] = hpi_rider(c);
                    if !t {
                        c.cycles += 8;
                        c.g[AT][0] = 0x3e80_0000;
                        c.mtc1(1, 1);
                        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(HPI_SLEW8_CUR));
                        c.abs_s(0, 0);
                        c.c_lt_s(0, 1);
                        let t = c.fcc();
                        c.g[V0][0] = 6;
                        if t {
                            break 'b_132d44;
                        }
                    }
                    c.cycles += 6;
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x7550));
                    c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(HPI_SLEW8_CUR));
                    c.c_lt_s(0, 1);
                    c.g[V0][0] = 5;
                    if c.fcc() {
                        c.cycles += 1;
                        c.g[V0][0] = 4;
                    }
                }
                c.cycles += 25;
                let mode = c.g[V0][0];
                Addr((c.g[V1][0] as u32).wrapping_add(HPI_R_MODE)).sw(c, mode);
                c.g[A0][0] = hpi_rider(c);
                c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(HPI_R_POSITION));
                call_ptr(c, 0x0011_fa10, 0x0013_2d54)?;
                c.g[A0][0] = hpi_rider(c);
                call_ptr(c, 0x0011_e098, 0x0013_2d5c)?;
                // Zero the slewed values 0, 2 and 8 of the rider.
                c.g[V1][0] = hpi_rider(c);
                c.g[A0][0] = c.g[S0][0];
                Addr((c.g[V1][0] as u32).wrapping_add(hpi_slew(0) + 4)).sw(c, 0);
                Addr((c.g[V1][0] as u32).wrapping_add(hpi_slew(0) + 8)).sw(c, 0);
                Addr((c.g[V1][0] as u32).wrapping_add(hpi_slew(0))).sw(c, 0);
                c.g[V0][0] = hpi_rider(c);
                Addr((c.g[V0][0] as u32).wrapping_add(hpi_slew(2) + 4)).sw(c, 0);
                Addr((c.g[V0][0] as u32).wrapping_add(hpi_slew(2) + 8)).sw(c, 0);
                Addr((c.g[V0][0] as u32).wrapping_add(hpi_slew(2))).sw(c, 0);
                c.g[V1][0] = hpi_rider(c);
                Addr((c.g[V1][0] as u32).wrapping_add(HPI_SLEW8_RATE)).sw(c, 0);
                Addr((c.g[V1][0] as u32).wrapping_add(HPI_SLEW8_TARGET)).sw(c, 0);
                Addr((c.g[V1][0] as u32).wrapping_add(HPI_SLEW8_CUR)).sw(c, 0);
                c.g[V0][0] = hpi_rider(c);
                c.g[A2][0] = Addr((c.g[A0][0] as u32).wrapping_add(HPI_TYPE)).lw(c);
                c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(HPI_R_MODE)).lw(c);
                Callee {
                    addr: 0x0013_2fb8,
                    f: ee::s01::f_00132fb8,
                }
                .call(c, 0x0013_2da0)?;
                frame.restore(c, S0, 0x30);
                break 'b_132f7c;
            }

            // Not landing and not boosted: leave when grounded, else wait.
            'b_132de4: {
                c.cycles += 5;
                c.g[A0][0] = hpi_rider(c);
                call_ptr(c, 0x0011_fe98, 0x0013_2db0)?;
                c.g[V1][0] = 5;
                let t = c.g[V0][0] != c.g[V1][0];
                c.g[V0][0] = hpi_rider(c);
                if t {
                    break 'b_132de4;
                }
                c.cycles += 10;
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(HPI_R_METRIX)).lw(c);
                call_ptr(c, 0x0011_9c38, 0x0013_2dc4)?;
                c.g[A0][0] = hpi_rider(c);
                c.g[A1][0] = 1;
                c.f[12] = c.f[0];
                call_ptr(c, 0x0010_e098, 0x0013_2dd4)?;
                c.g[A0][0] = hpi_rider(c);
                c.g[A1][0] = 1;
                call_ptr(c, 0x0011_fe78, 0x0013_2de0)?;
                c.g[V0][0] = hpi_rider(c);
            }

            // Ground check and the air predictor's landing-rate clamp.
            'b_132f00: {
                'b_132e50: {
                    c.cycles += 8;
                    c.g[A1][0] = 2;
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(HPI_R_ANIM)).lw(c);
                    call_ptr(c, 0x0031_1b20, 0x0013_2df0)?;
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xb0));
                    c.g[A1][0] = 1;
                    Callee {
                        addr: 0x0014_46a0,
                        f: ee::s01::f_001446a0,
                    }
                    .call(c, 0x0013_2dfc)?;
                    if c.g[V0][0] == 0 {
                        break 'b_132e50;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = hpi_rider(c);
                    call_ptr(c, 0x0010_6848, 0x0013_2e0c)?;
                    if c.g[V0][0] != 0 {
                        frame.restore(c, S0, 0x30);
                        break 'b_132f7c;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = hpi_rider(c);
                    call_ptr(c, 0x0011_fe98, 0x0013_2e1c)?;
                    let t = c.g[V0][0] != 0;
                    c.g[V0][0] = hpi_rider(c);
                    if t {
                        break 'b_132f00;
                    }
                    c.cycles += 11;
                    c.g[A1][0] = 0x3d;
                    c.g[AT][0] = 0xffff_ffff_bf80_0000;
                    c.mtc1(1, 12);
                    c.g[A2][0] = 0;
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(HPI_R_ANIM)).lw(c);
                    call_ptr(c, 0x0031_28e8, 0x0013_2e3c)?;
                    c.g[A0][0] = hpi_rider(c);
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0011_fec8, 0x0013_2e48)?;
                    frame.restore(c, S0, 0x30);
                    break 'b_132f7c;
                }
                c.cycles += 5;
                c.g[A0][0] = hpi_rider(c);
                call_ptr(c, 0x0011_fe98, 0x0013_2e58)?;
                c.g[A0][0] = c.g[V0][0];
                if c.g[A0][0] != c.g[S2][0] {
                    c.g[V0][0] = hpi_rider(c);
                    break 'b_132f00;
                }
                'b_132e88: {
                    c.cycles += 5;
                    c.g[V1][0] = hpi_rider(c);
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(HPI_R_AIR)).lw(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xac)).lw(c);
                    let t = c.g[V1][0] == c.g[A0][0];
                    c.g[A1][0] = 0;
                    if !t {
                        c.cycles += 3;
                        c.g[V0][0] = 3;
                        let t = c.g[V1][0] != c.g[V0][0];
                        if t {
                            break 'b_132e88;
                        }
                    }
                    c.cycles += 1;
                    c.g[A1][0] = 1;
                }
                c.cycles += 2;
                if c.g[A1][0] == 0 {
                    c.g[V0][0] = hpi_rider(c);
                    break 'b_132f00;
                }
                c.cycles += 10;
                c.g[V1][0] = hpi_rider(c);
                c.mtc1(0, 2);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(HPI_R_AIR)).lw(c);
                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0xa0));
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x98));
                let v = hpi_f(c, 0) - hpi_f(c, 1);
                hpi_set(c, 20, v);
                c.c_lt_s(2, 20);
                if !c.fcc() {
                    c.g[V0][0] = hpi_rider(c);
                    break 'b_132f00;
                }
                c.cycles += 15;
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(HPI_R_ANIM)).lw(c);
                c.g[A1][0] = 2;
                call_ptr(c, 0x0031_1b20, 0x0013_2ec4)?;
                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x10));
                c.g[AT][0] = 0x3f00_0000;
                c.mtc1(1, 2);
                let v = hpi_f(c, 0) - hpi_f(c, 1);
                hpi_set(c, 0, v);
                c.g[AT][0] = 0x4000_0000;
                c.mtc1(1, 3);
                let v = hpi_f(c, 0).div(hpi_f(c, 20));
                hpi_set(c, 0, v);
                c.c_le_s(2, 0);
                let t = !c.fcc();
                c.mov_s(1, 2);
                if !t {
                    c.cycles += 1;
                    let v = hpi_f(c, 0).min(hpi_f(c, 3));
                    hpi_set(c, 1, v);
                }
                c.cycles += 2;
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x90));
                c.g[V0][0] = hpi_rider(c);
            }

            // Grounded or the animation has ended: leave the handplant.
            c.cycles += 5;
            c.g[A1][0] = 2;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(HPI_R_ANIM)).lw(c);
            call_ptr(c, 0x0031_2ae8, 0x0013_2f0c)?;
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x30);
                break 'b_132f7c;
            }
            'b_132f54: {
                c.cycles += 5;
                c.g[A0][0] = hpi_rider(c);
                call_ptr(c, 0x0011_fe98, 0x0013_2f1c)?;
                c.g[V1][0] = 1;
                let t = c.g[V0][0] != c.g[V1][0];
                c.g[V0][0] = hpi_rider(c);
                if t {
                    break 'b_132f54;
                }
                c.cycles += 11;
                c.g[A1][0] = 0x11f;
                c.g[AT][0] = 0xffff_ffff_bf80_0000;
                c.mtc1(1, 12);
                c.g[A2][0] = 0;
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(HPI_R_ANIM)).lw(c);
                call_ptr(c, 0x0031_28e8, 0x0013_2f40)?;
                c.g[A0][0] = hpi_rider(c);
                c.g[A1][0] = 4;
                call_ptr(c, 0x0011_fec8, 0x0013_2f4c)?;
                frame.restore(c, S0, 0x30);
                break 'b_132f7c;
            }
            c.cycles += 9;
            c.g[A1][0] = 0x3d;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 12);
            c.g[A2][0] = 0;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(HPI_R_ANIM)).lw(c);
            call_ptr(c, 0x0031_28e8, 0x0013_2f6c)?;
            c.g[A0][0] = hpi_rider(c);
            c.g[A1][0] = 0;
            call_ptr(c, 0x0011_fec8, 0x0013_2f78)?;
            break 'b_132f78;
        }
        // Falls through from every exit that is not a plain return.
        c.cycles += 1;
        frame.restore(c, S0, 0x30);
    }
    // Epilogue: `lq` of s1, s2 and f20, `ld ra`, `jr ra`.
    c.cycles += 6;
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 20, 0x40);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    /// Fixture layout: the sub-state, the input, the rider and its objects.
    const OC448_SELF: u32 = DATA + 0x100;
    const OC448_IN: u32 = DATA + 0x200;
    const OC448_RIDER: u32 = DATA + 0x1000;
    const OC448_ANIM: u32 = DATA + 0x5000;
    const OC448_METRIX: u32 = DATA + 0x6000;
    const OC448_AIR: u32 = DATA + 0x7000;
    const OC448_SEQ: u32 = DATA + 0x8000;
    /// The gate word and the scripted fake results (outside the fakes' log).
    const OC448_CTRL: u32 = DATA + 0x0020_0000;
    const OC448_MAGIC: u32 = 0x0448_0a30;

    /// The next scripted result of fake `idx` (the last one repeats).
    fn oc448_script(c: &mut Cpu, idx: u32) -> u32 {
        let count = OC448_CTRL + 0x80 + 4 * idx;
        let n = c.bus.read32(count);
        c.bus.write32(count, n + 1);
        c.bus.read32(OC448_CTRL + 0x100 + 0x20 * idx + 4 * n.min(7))
    }

    fn oc448_put_script(c: &mut Cpu, idx: u32, vals: [u32; 8]) {
        for (k, v) in vals.iter().enumerate() {
            c.bus
                .write32(OC448_CTRL + 0x100 + 0x20 * idx + 4 * k as u32, *v);
        }
    }

    /// Gated fakes for the callees with override slots. With the gate off
    /// they run what the slot held before.
    macro_rules! oc448_fakes {
        ($($fake:ident = $idx:expr, $shard:ident :: $slot:ident / $gen:ident, $set:expr;)*) => {
            $(
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC448_CTRL) != OC448_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), ee::$shard::$gen);
                    }
                    let v = oc448_script(c, $idx);
                    let set: fn(&mut Cpu, u32) = $set;
                    set(c, v);
                    ret(c);
                }
            )*
            fn oc448_install() {
                $(
                    fakes::hook(
                        std::ptr::addr_of_mut!(ee::$shard::$slot),
                        $fake,
                        fakes::prev_cell($fake),
                    );
                )*
            }
        };
    }

    oc448_fakes! {
        oc448_fake_check_reset = 0, s00::OVERRIDE_00116120 / f_00116120__generated, |c: &mut Cpu, v: u32| { c.g[V0][0] = sx(v); };
        oc448_fake_layer_finished = 1, s10::OVERRIDE_00312AE8 / f_00312ae8__generated, |c: &mut Cpu, v: u32| { c.g[V0][0] = sx(v); };
        oc448_fake_start = 2, s00::OVERRIDE_00119BF0 / f_00119bf0__generated, |c: &mut Cpu, v: u32| { c.f[0] = v; };
        oc448_fake_boost = 3, s00::OVERRIDE_0010E098 / f_0010e098__generated, |_c: &mut Cpu, _v: u32| {};
        oc448_fake_lean = 4, s00::OVERRIDE_0011FA10 / f_0011fa10__generated, |_c: &mut Cpu, _v: u32| {};
        oc448_fake_orient = 5, s00::OVERRIDE_0011E098 / f_0011e098__generated, |_c: &mut Cpu, _v: u32| {};
        oc448_fake_end = 6, s00::OVERRIDE_00119C38 / f_00119c38__generated, |c: &mut Cpu, v: u32| { c.f[0] = v; };
        oc448_fake_flags64 = 7, s01::OVERRIDE_001446A0 / f_001446a0__generated, |c: &mut Cpu, v: u32| { c.g[V0][0] = sx(v); };
        oc448_fake_rail = 8, s00::OVERRIDE_00106848 / f_00106848__generated, |c: &mut Cpu, v: u32| { c.g[V0][0] = sx(v); };
        oc448_fake_seq = 9, s10::OVERRIDE_00311B20 / f_00311b20__generated, |c: &mut Cpu, v: u32| { c.g[V0][0] = sx(v); };
        oc448_fake_main_state = 10, s00::OVERRIDE_0011FE98 / f_0011fe98__generated, |c: &mut Cpu, v: u32| { c.g[V0][0] = sx(v); };
        oc448_fake_play_anim = 11, s10::OVERRIDE_003128E8 / f_003128e8__generated, |_c: &mut Cpu, _v: u32| {};
        oc448_fake_sub_state = 12, s00::OVERRIDE_0011FEC8 / f_0011fec8__generated, |_c: &mut Cpu, _v: u32| {};
        oc448_fake_motion = 13, s00::OVERRIDE_0011FE78 / f_0011fe78__generated, |_c: &mut Cpu, _v: u32| {};
        oc448_fake_anim_table = 14, s10::OVERRIDE_00311B48 / f_00311b48__generated, |_c: &mut Cpu, _v: u32| {};
    }

    fn oc448_unit(r: &mut Rng) -> f32 {
        (r.next_u32() % 1000) as f32 / 1000.0
    }

    /// One seed: random registers, the sub-state, the input flags, the
    /// rider's fields, the game's float constants and the scripted results
    /// of the callees (reset, animation end, boost, ground, air, state).
    fn oc448_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x0448_0a30_0000_0001 ^ ((seed as u64) << 20));
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.g[A0][0] = OC448_SELF as u64;
        c.g[A1][0] = OC448_IN as u64;

        // The game's float constants next to gp.
        c.bus
            .write32(GAME_GP.wrapping_sub(0x7560), 0.016_666_668f32.to_bits());
        c.bus
            .write32(GAME_GP.wrapping_sub(0x755c), 0.032_258_064f32.to_bits());
        c.bus
            .write32(GAME_GP.wrapping_sub(0x7558), 0.501_338_4f32.to_bits());
        c.bus
            .write32(GAME_GP.wrapping_sub(0x7554), 0.199_991_3f32.to_bits());
        c.bus
            .write32(GAME_GP.wrapping_sub(0x7550), (-0.050_000_0f32).to_bits());

        // The sub-state.
        c.bus
            .write32(OC448_SELF + HPI_ENTER_SEC, (oc448_unit(&mut r) * 1.2).to_bits());
        c.bus.write32(OC448_SELF + HPI_BALANCE, r.f32(0.6));
        c.bus
            .write32(OC448_SELF + HPI_MISSED, u32::from(seed % 3 != 0));
        c.bus
            .write32(OC448_SELF + HPI_RAIL_SPLINE, (seed / 3) % 2);
        c.bus.write32(OC448_SELF + HPI_TYPE, (seed / 6) % 2);
        c.bus.write32(OC448_SELF + HPI_RIDER, OC448_RIDER);

        // The input flags: bit 13 (boost) on every other seed.
        let flags = if seed % 2 == 0 {
            r.next_u32() | 0x2000
        } else {
            r.next_u32() & !0x2000
        };
        c.bus.write32(OC448_IN, flags);

        // The rider.
        const MODES: [u32; 6] = [1, 2, 4, 5, 6, 3];
        c.bus
            .write32(OC448_RIDER + HPI_R_MODE, MODES[(seed % 6) as usize]);
        c.bus.write32(
            OC448_RIDER + HPI_R_TIME_SCALE,
            (0.5 + oc448_unit(&mut r) * 1.5).to_bits(),
        );
        c.bus.write32(OC448_RIDER + HPI_R_FAKIE, (seed / 2) % 2);
        c.bus
            .write32(OC448_RIDER + HPI_SLEW8_CUR, r.f32(1.6));
        c.bus.write32(OC448_RIDER + HPI_R_ANIM, OC448_ANIM);
        c.bus.write32(OC448_RIDER + HPI_R_AIR, OC448_AIR);
        c.bus.write32(OC448_RIDER + HPI_R_METRIX, OC448_METRIX);
        c.bus.write32(OC448_AIR + 0xac, (seed / 4) % 4);
        c.bus.write32(OC448_AIR + 0x98, r.f32(2.0));
        c.bus.write32(OC448_AIR + 0xa0, r.f32(2.0));
        c.bus
            .write32(OC448_SEQ + 0x8, oc448_unit(&mut r).to_bits());
        c.bus
            .write32(OC448_SEQ + 0x10, (1.0 + oc448_unit(&mut r)).to_bits());

        // The scripted callees.
        c.bus.write32(OC448_CTRL, OC448_MAGIC);
        oc448_put_script(c, 0, [u32::from(seed % 5 == 0), 0, 0, 0, 0, 0, 0, 0]);
        let bits: [u32; 8] = std::array::from_fn(|_| r.next_u32() & 1);
        oc448_put_script(c, 1, bits);
        let bits: [u32; 8] = std::array::from_fn(|_| r.next_u32() & 1);
        oc448_put_script(c, 7, bits);
        let starts: [u32; 8] = std::array::from_fn(|_| r.f32(2.0));
        oc448_put_script(c, 2, starts);
        let ends: [u32; 8] = std::array::from_fn(|_| r.f32(2.0));
        oc448_put_script(c, 6, ends);
        oc448_put_script(c, 8, [u32::from(seed % 4 == 0), 0, 0, 0, 0, 0, 0, 0]);
        oc448_put_script(c, 9, [OC448_SEQ; 8]);
        const STATES: [u32; 6] = [0, 1, 5, 3, 4, 2];
        let states: [u32; 8] = std::array::from_fn(|_| STATES[(r.next_u32() % 6) as usize]);
        oc448_put_script(c, 10, states);
    }

    #[test]
    fn rider_handplant_sub_handle_input_matches() {
        oc448_install();
        check(
            ee::s01::f_00132a30__generated,
            rider_handplant_sub_handle_input,
            300,
            &oc448_setup,
        );
    }
}
