//! Staging module oc496 (wave wave227): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc496`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0017_36e0,
    bounded_camera_algorithm_base_update,
    "BoundedCameraAlgorithmBase_Update",
)];

/// `base + off` with `base` a register's low word (the generated
/// `(c.g[R][0] as u32).wrapping_add(off)`).
#[inline(always)]
fn bcu_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}

/// `base - back` with `base` a register's low word (the generated
/// `wrapping_sub`), used for the gp-relative constants.
#[inline(always)]
fn bcu_back(c: &Cpu, reg: usize, back: u32) -> u32 {
    r32(c, reg).wrapping_sub(back)
}

/// `void BoundedCameraAlgorithmBase_Update(self)` (0x1736e0; the reference
/// names it `cTriggerAlgorithm_update`): the trigger camera's per-frame update.
///
/// The focus point is the rider's position (vtable slot 1) raised by
/// `HeightOffset` (`+0x6c`) along z and pushed by `LeadDistance` (`+0x74`)
/// along its front (slot 4); it becomes `CamTargetPos` (`+0x20`). Yaw and
/// pitch of camera-to-focus turn towards the front's yaw and pitch (wrapped),
/// the distance moves towards `IdealDist` (`+0x70`): at once when more than
/// 5000 away, else by a third of the error, or by error * k / 60 with
/// k = 0.0072 * |velocity| + 8 when k >= 20. `Position` (`+0x40`) is the focus
/// pulled back along the new direction (two `Math_SinCos`). `Heading` (`+0x50`)
/// and `Pitch` (`+0x54`) then turn towards that direction by
/// `HeadingLookRate` (`+0x64`) and `PitchLookRate` (`+0x68`) per 1/60 s.
///
/// The arithmetic is the generated body's VU0 and FPU sequence, with its
/// stack spills and cycle charges per block, so the port is exact at every
/// branch.
pub fn bounded_camera_algorithm_base_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 71;
        c.g[SP][0] = sx(bcu_back(c, SP, 0xe0));
        c.sq(16, bcu_at(c, SP, 0xa0));
        c.swc1(22, bcu_at(c, SP, 0xc0));
        c.g[S0][0] = c.g[A0][0];
        c.swc1(21, bcu_at(c, SP, 0xb8));
        c.g[A0][0] = c.g[SP][0];
        c.swc1(20, bcu_at(c, SP, 0xb0));
        c.sd(bcu_at(c, SP, 0x90), c.g[RA][0]);
        c.swc1(25, bcu_at(c, SP, 0xd8));
        c.swc1(24, bcu_at(c, SP, 0xd0));
        c.swc1(23, bcu_at(c, SP, 0xc8));
        c.mtc1(0, 22);

        // The rider's position (slot 1, +8), velocity (slot 3, +0x18) and
        // front (slot 4, +0x20) into the stack vectors at sp+0, +0x10, +0x20.
        c.g[A2][0] = c.lw(bcu_at(c, S0, 0x30));
        c.g[V0][0] = c.lw(r32(c, A2));
        c.g[A1][0] = c.lh(bcu_at(c, V0, 0x8));
        c.g[V1][0] = c.lw(bcu_at(c, V0, 0xc));
        let t = c.g[V1][0] as u32;
        c.g[A1][0] = sx(r32(c, A2).wrapping_add(r32(c, A1)));
        call_ptr(c, t, 0x0017_3728)?;

        c.g[A2][0] = c.lw(bcu_at(c, S0, 0x30));
        c.g[A0][0] = sx(bcu_at(c, SP, 0x10));
        c.g[V0][0] = c.lw(r32(c, A2));
        c.g[A1][0] = c.lh(bcu_at(c, V0, 0x18));
        c.g[V1][0] = c.lw(bcu_at(c, V0, 0x1c));
        let t = c.g[V1][0] as u32;
        c.g[A1][0] = sx(r32(c, A2).wrapping_add(r32(c, A1)));
        call_ptr(c, t, 0x0017_3744)?;

        c.g[A2][0] = c.lw(bcu_at(c, S0, 0x30));
        c.g[A0][0] = sx(bcu_at(c, SP, 0x20));
        c.g[V0][0] = c.lw(r32(c, A2));
        c.g[A1][0] = c.lh(bcu_at(c, V0, 0x20));
        c.g[V1][0] = c.lw(bcu_at(c, V0, 0x24));
        let t = c.g[V1][0] as u32;
        c.g[A1][0] = sx(r32(c, A2).wrapping_add(r32(c, A1)));
        call_ptr(c, t, 0x0017_3760)?;

        // focus = position + HeightOffset * z (the t4vect_ZAxis constant at
        // 0x4ff160), then focus += front * LeadDistance.
        c.g[V0][0] = 0x500000;
        c.lwc1(0, bcu_at(c, S0, 0x6c));
        c.g[V0][0] = sx(bcu_back(c, V0, 0xea0));
        c.mfc1(3, 0);
        c.lqc2(4, r32(c, V0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, bcu_at(c, SP, 0x40));
        c.lq(2, bcu_at(c, SP, 0x40));
        c.sq(2, bcu_at(c, SP, 0x30));
        c.lqc2(3, r32(c, SP));
        c.lqc2(4, bcu_at(c, SP, 0x30));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, r32(c, SP));
        c.lwc1(0, bcu_at(c, S0, 0x74));
        c.mfc1(3, 0);
        c.lqc2(4, bcu_at(c, SP, 0x20));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, bcu_at(c, SP, 0x40));
        c.lq(2, bcu_at(c, SP, 0x40));
        c.sq(2, bcu_at(c, SP, 0x30));
        c.lqc2(3, r32(c, SP));
        c.lqc2(4, bcu_at(c, SP, 0x30));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, r32(c, SP));
        // self->CamTargetPos = focus; toFocus = focus - Position (sp+0x30).
        c.lq(2, r32(c, SP));
        c.sq(2, bcu_at(c, S0, 0x20));
        c.lqc2(3, r32(c, SP));
        c.lqc2(4, bcu_at(c, S0, 0x40));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, bcu_at(c, SP, 0x40));
        c.lq(2, bcu_at(c, SP, 0x40));
        c.sq(2, bcu_at(c, SP, 0x30));

        // yawD = Atan2(toFocus.y, toFocus.x) (f23), with the x == 0 cases
        // (+-pi/2 or 0) handled before the atan call.
        'b_17386c: {
            'b_173868: {
                'b_173828: {
                    c.lwc1(20, bcu_at(c, SP, 0x30));
                    c.c_eq_s(20, 22);
                    let t = !c.fcc();
                    c.lwc1(21, bcu_at(c, SP, 0x34));
                    if t {
                        break 'b_173828;
                    } else {
                        c.cycles += 4;
                        c.c_eq_s(21, 22);
                        let t = c.fcc();
                        c.mov_s(23, 21);
                        if t {
                            break 'b_173868;
                        } else {
                            c.cycles += 4;
                            c.c_le_s(22, 21);
                            c.lwc1(23, bcu_back(c, GP, 0x63f4));
                            let t = !c.fcc();
                            c.lwc1(20, bcu_at(c, SP, 0x20));
                            if t {
                                break 'b_173868;
                            } else {
                                c.cycles += 3;
                                c.lwc1(23, bcu_back(c, GP, 0x63f0));
                                break 'b_17386c;
                            }
                        }
                    }
                }
                'b_173864: {
                    c.cycles += 7;
                    c.div_s(12, 21, 20);
                    call_ptr(c, 0x0031_c228, 0x0017_3834)?;
                    c.c_lt_s(20, 22);
                    let t = !c.fcc();
                    c.mov_s(12, 0);
                    if t {
                        break 'b_173864;
                    } else {
                        'b_17385c: {
                            c.cycles += 3;
                            c.c_lt_s(22, 21);
                            let t = !c.fcc();
                            if t {
                                break 'b_17385c;
                            } else {
                                c.cycles += 3;
                                c.lwc1(0, bcu_back(c, GP, 0x63ec));
                                c.add_s(12, 12, 0);
                                break 'b_173864;
                            }
                        }
                        c.cycles += 2;
                        c.lwc1(0, bcu_back(c, GP, 0x63e8));
                        c.sub_s(12, 12, 0);
                        break 'b_173864;
                    }
                }
                c.cycles += 1;
                c.mov_s(23, 12);
                break 'b_173868;
            }
            c.cycles += 1;
            c.lwc1(20, bcu_at(c, SP, 0x20));
            break 'b_17386c;
        }

        // yawC = Atan2(riderFront.y, riderFront.x) (f12).
        'b_1738e8: {
            'b_1738ac: {
                c.cycles += 5;
                c.mtc1(0, 22);
                c.c_eq_s(20, 22);
                let t = !c.fcc();
                c.lwc1(21, bcu_at(c, SP, 0x24));
                if t {
                    break 'b_1738ac;
                } else {
                    c.cycles += 4;
                    c.c_eq_s(21, 22);
                    let t = c.fcc();
                    c.mov_s(12, 21);
                    if t {
                        break 'b_1738e8;
                    } else {
                        c.cycles += 4;
                        c.c_le_s(22, 21);
                        c.lwc1(12, bcu_back(c, GP, 0x63e4));
                        let t = !c.fcc();
                        if t {
                            break 'b_1738e8;
                        } else {
                            c.cycles += 3;
                            c.lwc1(12, bcu_back(c, GP, 0x63e0));
                            break 'b_1738e8;
                        }
                    }
                }
            }
            c.cycles += 7;
            c.div_s(12, 21, 20);
            call_ptr(c, 0x0031_c228, 0x0017_38b8)?;
            c.c_lt_s(20, 22);
            let t = !c.fcc();
            c.mov_s(12, 0);
            if t {
                break 'b_1738e8;
            } else {
                'b_1738e0: {
                    c.cycles += 3;
                    c.c_lt_s(22, 21);
                    let t = !c.fcc();
                    if t {
                        break 'b_1738e0;
                    } else {
                        c.cycles += 3;
                        c.lwc1(0, bcu_back(c, GP, 0x63dc));
                        c.add_s(12, 12, 0);
                        break 'b_1738e8;
                    }
                }
                c.cycles += 2;
                c.lwc1(0, bcu_back(c, GP, 0x63d8));
                c.sub_s(12, 12, 0);
                break 'b_1738e8;
            }
        }

        // f12 = diff = Wrap(yawC) - yawD (f23), with Wrap(v) = v - floor(v / 2pi
        // + 1/2) * 2pi (the floor by cvt.w.s).
        'b_17391c: {
            c.cycles += 10;
            c.lwc1(2, bcu_back(c, GP, 0x63d4));
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 4);
            c.mul_s(0, 12, 2);
            c.add_s(0, 0, 4);
            c.cvt_w_s(1, 0);
            c.cvt_s_w(1, 1);
            c.c_lt_s(0, 1);
            let t = !c.fcc();
            if t {
                break 'b_17391c;
            } else {
                c.cycles += 3;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 0);
                c.sub_s(1, 1, 0);
                break 'b_17391c;
            }
        }
        'b_17395c: {
            c.cycles += 12;
            c.lwc1(3, bcu_back(c, GP, 0x63d0));
            c.mul_s(0, 1, 3);
            c.sub_s(0, 12, 0);
            c.sub_s(12, 0, 23);
            c.mul_s(1, 12, 2);
            c.add_s(1, 1, 4);
            c.cvt_w_s(2, 1);
            c.cvt_s_w(2, 2);
            c.c_lt_s(1, 2);
            if !c.fcc() {
                c.mul_s(0, 2, 3);
                break 'b_17395c;
            } else {
                c.cycles += 4;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 0);
                c.sub_s(2, 2, 0);
                c.mul_s(0, 2, 3);
                break 'b_17395c;
            }
        }

        // f25 = yawN = yawD + Wrap(diff) * HeadingAdjustRate / 60. The distance
        // f24 = len3 + dd when |dd| > 5000 (dd = IdealDist - len3); otherwise
        // see the k branch below.
        'b_173a24: {
            'b_1739bc: {
                c.cycles += 22;
                c.lwc1(4, bcu_at(c, S0, 0x5c));
                c.lwc1(3, bcu_at(c, S0, 0x70));
                c.lqc2(3, bcu_at(c, SP, 0x30));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().sqrt(4, 0);
                c.vu0().waitq();
                c.cfc2(2, 22);
                c.mtc1(2, 6);
                c.lwc1(7, bcu_back(c, GP, 0x63cc));
                c.sub_s(0, 12, 0);
                c.lwc1(2, bcu_back(c, GP, 0x63c8));
                c.sub_s(5, 3, 6);
                c.abs_s(1, 5);
                c.mul_s(0, 0, 4);
                c.c_lt_s(2, 1);
                c.mul_s(0, 0, 7);
                let t = !c.fcc();
                c.add_s(25, 23, 0);
                if t {
                    break 'b_1739bc;
                } else {
                    c.cycles += 2;
                    c.add_s(24, 6, 5);
                    break 'b_173a24;
                }
            }
            // The k branch of the distance: k = |riderVel| * 0.0072 + 8, and
            // x = dd * k / 60 when k >= 20, else dd / 3; f24 = len3 + x.
            'b_173a20: {
                'b_173a18: {
                    c.cycles += 20;
                    c.lwc1(2, bcu_back(c, GP, 0x63c4));
                    c.lqc2(3, bcu_at(c, SP, 0x10));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(2, 22);
                    c.mtc1(2, 0);
                    c.g[AT][0] = 0x4100_0000;
                    c.mtc1(1, 3);
                    c.mul_s(0, 0, 2);
                    c.g[AT][0] = 0x41a0_0000;
                    c.mtc1(1, 1);
                    c.add_s(0, 0, 3);
                    c.c_le_s(1, 0);
                    let t = !c.fcc();
                    if t {
                        break 'b_173a18;
                    } else {
                        c.cycles += 3;
                        c.mul_s(0, 5, 0);
                        c.mul_s(0, 0, 7);
                        break 'b_173a20;
                    }
                }
                c.cycles += 2;
                c.lwc1(0, bcu_back(c, GP, 0x63c0));
                c.mul_s(0, 5, 0);
                break 'b_173a20;
            }
            c.cycles += 1;
            c.add_s(24, 6, 0);
            break 'b_173a24;
        }

        // pitchD = Atan2(-toFocus.z, Len2(toFocus.x, toFocus.y)) (f23).
        'b_173ad0: {
            'b_173ac8: {
                'b_173a88: {
                    c.cycles += 14;
                    c.lwc1(0, bcu_at(c, SP, 0x30));
                    c.lwc1(1, bcu_at(c, SP, 0x34));
                    c.mul_s(2, 0, 0);
                    c.lwc1(3, bcu_at(c, SP, 0x38));
                    c.mul_s(4, 1, 1);
                    c.swc1(0, bcu_at(c, SP, 0x40));
                    c.mtc1(0, 22);
                    c.neg_s(21, 3);
                    c.swc1(1, bcu_at(c, SP, 0x44));
                    c.add_s(2, 2, 4);
                    c.sqrt_s(20, 2);
                    c.c_eq_s(20, 22);
                    let t = !c.fcc();
                    if t {
                        break 'b_173a88;
                    } else {
                        c.cycles += 4;
                        c.c_eq_s(21, 22);
                        let t = c.fcc();
                        c.mov_s(23, 21);
                        if t {
                            break 'b_173ac8;
                        } else {
                            c.cycles += 4;
                            c.c_le_s(22, 21);
                            c.lwc1(23, bcu_back(c, GP, 0x63bc));
                            let t = !c.fcc();
                            c.lwc1(0, bcu_at(c, SP, 0x20));
                            if t {
                                break 'b_173ac8;
                            } else {
                                c.cycles += 3;
                                c.lwc1(23, bcu_back(c, GP, 0x63b8));
                                c.lwc1(1, bcu_at(c, SP, 0x24));
                                break 'b_173ad0;
                            }
                        }
                    }
                }
                'b_173ac4: {
                    c.cycles += 7;
                    c.div_s(12, 21, 20);
                    call_ptr(c, 0x0031_c228, 0x0017_3a94)?;
                    c.c_lt_s(20, 22);
                    let t = !c.fcc();
                    c.mov_s(3, 0);
                    if t {
                        break 'b_173ac4;
                    } else {
                        'b_173abc: {
                            c.cycles += 3;
                            c.c_lt_s(22, 21);
                            let t = !c.fcc();
                            if t {
                                break 'b_173abc;
                            } else {
                                c.cycles += 3;
                                c.lwc1(0, bcu_back(c, GP, 0x63b4));
                                c.add_s(3, 3, 0);
                                break 'b_173ac4;
                            }
                        }
                        c.cycles += 2;
                        c.lwc1(0, bcu_back(c, GP, 0x63b0));
                        c.sub_s(3, 3, 0);
                        break 'b_173ac4;
                    }
                }
                c.cycles += 1;
                c.mov_s(23, 3);
                break 'b_173ac8;
            }
            c.cycles += 2;
            c.lwc1(0, bcu_at(c, SP, 0x20));
            c.lwc1(1, bcu_at(c, SP, 0x24));
            break 'b_173ad0;
        }

        // pitchC = Atan2(riderFront.z, Len2(riderFront.x, riderFront.y)) (f12).
        'b_173b6c: {
            'b_173b68: {
                'b_173b28: {
                    c.cycles += 11;
                    c.mul_s(2, 0, 0);
                    c.swc1(0, bcu_at(c, SP, 0x40));
                    c.mul_s(3, 1, 1);
                    c.swc1(1, bcu_at(c, SP, 0x44));
                    c.mtc1(0, 22);
                    c.lwc1(21, bcu_at(c, SP, 0x28));
                    c.add_s(2, 2, 3);
                    c.sqrt_s(20, 2);
                    c.c_eq_s(20, 22);
                    let t = !c.fcc();
                    if t {
                        break 'b_173b28;
                    } else {
                        c.cycles += 4;
                        c.c_eq_s(21, 22);
                        let t = c.fcc();
                        c.mov_s(12, 21);
                        if t {
                            break 'b_173b68;
                        } else {
                            c.cycles += 4;
                            c.c_le_s(22, 21);
                            c.lwc1(12, bcu_back(c, GP, 0x63ac));
                            let t = !c.fcc();
                            c.lwc1(1, bcu_at(c, S0, 0x58));
                            if t {
                                break 'b_173b6c;
                            } else {
                                c.cycles += 3;
                                c.lwc1(12, bcu_back(c, GP, 0x63a8));
                                break 'b_173b6c;
                            }
                        }
                    }
                }
                'b_173b64: {
                    c.cycles += 7;
                    c.div_s(12, 21, 20);
                    call_ptr(c, 0x0031_c228, 0x0017_3b34)?;
                    c.c_lt_s(20, 22);
                    let t = !c.fcc();
                    c.mov_s(1, 0);
                    if t {
                        break 'b_173b64;
                    } else {
                        'b_173b5c: {
                            c.cycles += 3;
                            c.c_lt_s(22, 21);
                            let t = !c.fcc();
                            if t {
                                break 'b_173b5c;
                            } else {
                                c.cycles += 3;
                                c.lwc1(0, bcu_back(c, GP, 0x63a4));
                                c.add_s(1, 1, 0);
                                break 'b_173b64;
                            }
                        }
                        c.cycles += 2;
                        c.lwc1(0, bcu_back(c, GP, 0x63a0));
                        c.sub_s(1, 1, 0);
                        break 'b_173b64;
                    }
                }
                c.cycles += 1;
                c.mov_s(12, 1);
                break 'b_173b68;
            }
            c.cycles += 1;
            c.lwc1(1, bcu_at(c, S0, 0x58));
            break 'b_173b6c;
        }

        // tp = pitchC * -PitchMultiplier + PitchOffset / 60 (f2), wrapped
        // (floor of the scaled value, as above).
        'b_173bb8: {
            c.cycles += 16;
            c.lwc1(3, bcu_back(c, GP, 0x639c));
            c.neg_s(1, 1);
            c.lwc1(2, bcu_at(c, S0, 0x78));
            c.lwc1(0, bcu_back(c, GP, 0x6398));
            c.mul_s(2, 2, 3);
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 4);
            c.mul_s(1, 12, 1);
            c.add_s(3, 1, 2);
            c.mul_s(0, 3, 0);
            c.add_s(0, 0, 4);
            c.cvt_w_s(2, 0);
            c.cvt_s_w(2, 2);
            c.c_lt_s(0, 2);
            let t = !c.fcc();
            if t {
                break 'b_173bb8;
            } else {
                c.cycles += 3;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 0);
                c.sub_s(2, 2, 0);
                break 'b_173bb8;
            }
        }

        // pitchN from pitchC and pitchD; two Math_SinCos calls (yawN in f25,
        // pitchN in f20) into the stack, then dir = (sc3 * sc1, sc3 * sc0,
        // -sc2, 0), n = focus - dir * dist (f24), and Position (+0x40) = n.
        'b_173cfc: {
            'b_173cf8: {
                'b_173cbc: {
                    c.cycles += 54;
                    c.lwc1(20, bcu_back(c, GP, 0x6394));
                    c.g[A0][0] = sx(bcu_at(c, SP, 0x80));
                    c.lwc1(1, bcu_at(c, S0, 0x60));
                    c.g[A1][0] = sx(bcu_at(c, SP, 0x84));
                    c.mul_s(20, 2, 20);
                    c.lwc1(0, bcu_back(c, GP, 0x6390));
                    c.mov_s(12, 25);
                    c.mtc1(0, 22);
                    c.sub_s(20, 3, 20);
                    c.sub_s(20, 20, 23);
                    c.mul_s(20, 20, 1);
                    c.mul_s(20, 20, 0);
                    c.add_s(20, 23, 20);
                    call_ptr(c, 0x0031_be50, 0x0017_3bf0)?;
                    c.g[A0][0] = sx(bcu_at(c, SP, 0x88));
                    c.g[A1][0] = sx(bcu_at(c, SP, 0x8c));
                    c.mov_s(12, 20);
                    call_ptr(c, 0x0031_be50, 0x0017_3c00)?;
                    c.lwc1(2, bcu_at(c, SP, 0x8c));
                    c.lwc1(0, bcu_at(c, SP, 0x84));
                    c.lwc1(3, bcu_at(c, SP, 0x80));
                    c.lwc1(1, bcu_at(c, SP, 0x88));
                    c.mul_s(0, 2, 0);
                    c.mul_s(2, 2, 3);
                    c.swc1(22, bcu_at(c, SP, 0x4c));
                    c.neg_s(1, 1);
                    c.swc1(0, bcu_at(c, SP, 0x40));
                    c.swc1(2, bcu_at(c, SP, 0x44));
                    c.swc1(1, bcu_at(c, SP, 0x48));
                    c.mfc1(3, 24);
                    c.lqc2(4, bcu_at(c, SP, 0x40));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, bcu_at(c, SP, 0x70));
                    c.lq(2, bcu_at(c, SP, 0x70));
                    c.sq(2, bcu_at(c, SP, 0x60));
                    c.lqc2(3, r32(c, SP));
                    c.lqc2(4, bcu_at(c, SP, 0x60));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, bcu_at(c, SP, 0x70));
                    c.lq(2, bcu_at(c, SP, 0x70));
                    c.sq(2, bcu_at(c, S0, 0x40));
                    c.sq(2, bcu_at(c, SP, 0x50));
                    // n = focus - Position (sp+0x60), then n at sp+0x50.
                    c.lqc2(3, r32(c, SP));
                    c.lqc2(4, bcu_at(c, S0, 0x40));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, bcu_at(c, SP, 0x60));
                    c.lq(2, bcu_at(c, SP, 0x60));
                    c.sq(2, bcu_at(c, SP, 0x50));
                    // yawE = Atan2(n.y, n.x) (f12).
                    c.lwc1(20, bcu_at(c, SP, 0x50));
                    c.c_eq_s(20, 22);
                    let t = !c.fcc();
                    c.lwc1(21, bcu_at(c, SP, 0x54));
                    if t {
                        break 'b_173cbc;
                    } else {
                        c.cycles += 4;
                        c.c_eq_s(21, 22);
                        let t = c.fcc();
                        c.mov_s(12, 21);
                        if t {
                            break 'b_173cf8;
                        } else {
                            c.cycles += 4;
                            c.c_le_s(22, 21);
                            c.lwc1(12, bcu_back(c, GP, 0x638c));
                            let t = !c.fcc();
                            c.lwc1(7, bcu_at(c, S0, 0x50));
                            if t {
                                break 'b_173cfc;
                            } else {
                                c.cycles += 3;
                                c.lwc1(12, bcu_back(c, GP, 0x6388));
                                break 'b_173cfc;
                            }
                        }
                    }
                }
                c.cycles += 7;
                c.div_s(12, 21, 20);
                call_ptr(c, 0x0031_c228, 0x0017_3cc8)?;
                c.c_lt_s(20, 22);
                let t = !c.fcc();
                c.mov_s(12, 0);
                if t {
                    break 'b_173cf8;
                } else {
                    'b_173cf0: {
                        c.cycles += 3;
                        c.c_lt_s(22, 21);
                        let t = !c.fcc();
                        if t {
                            break 'b_173cf0;
                        } else {
                            c.cycles += 3;
                            c.lwc1(0, bcu_back(c, GP, 0x6384));
                            c.add_s(12, 12, 0);
                            break 'b_173cf8;
                        }
                    }
                    c.cycles += 2;
                    c.lwc1(0, bcu_back(c, GP, 0x6380));
                    c.sub_s(12, 12, 0);
                    break 'b_173cf8;
                }
            }
            c.cycles += 1;
            c.lwc1(7, bcu_at(c, S0, 0x50));
            break 'b_173cfc;
        }

        // Heading wrap: Wrap(yawE - Heading) (f1).
        'b_173d34: {
            c.cycles += 11;
            c.lwc1(0, bcu_back(c, GP, 0x637c));
            c.sub_s(12, 12, 7);
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 1);
            c.mul_s(0, 12, 0);
            c.add_s(0, 0, 1);
            c.cvt_w_s(1, 0);
            c.cvt_s_w(1, 1);
            c.c_lt_s(0, 1);
            let t = !c.fcc();
            if t {
                break 'b_173d34;
            } else {
                c.cycles += 3;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 0);
                c.sub_s(1, 1, 0);
                break 'b_173d34;
            }
        }

        // Heading (+0x50) += Wrap(yawE - Heading) * HeadingLookRate / 60, and
        // pitchE = Atan2(n.z, Len2(n.x, n.y)) turns Pitch in the tail below.
        'b_173dfc: {
            'b_173df8: {
                'b_173df4: {
                    'b_173db4: {
                        c.cycles += 21;
                        c.lwc1(0, bcu_back(c, GP, 0x6378));
                        c.lwc1(5, bcu_at(c, S0, 0x64));
                        c.mul_s(0, 1, 0);
                        c.lwc1(2, bcu_at(c, SP, 0x54));
                        c.lwc1(1, bcu_at(c, SP, 0x50));
                        c.lwc1(6, bcu_back(c, GP, 0x6374));
                        c.mul_s(4, 2, 2);
                        c.sub_s(0, 12, 0);
                        c.swc1(1, bcu_at(c, SP, 0x60));
                        c.mul_s(3, 1, 1);
                        c.swc1(2, bcu_at(c, SP, 0x64));
                        c.mtc1(0, 22);
                        c.mul_s(0, 0, 5);
                        c.lwc1(21, bcu_at(c, SP, 0x58));
                        c.add_s(3, 3, 4);
                        c.sqrt_s(20, 3);
                        c.mul_s(0, 0, 6);
                        c.c_eq_s(20, 22);
                        c.add_s(0, 7, 0);
                        let t = !c.fcc();
                        c.swc1(0, bcu_at(c, S0, 0x50));
                        if t {
                            break 'b_173db4;
                        } else {
                            c.cycles += 4;
                            c.c_eq_s(21, 22);
                            let t = c.fcc();
                            c.mov_s(0, 21);
                            if t {
                                break 'b_173df4;
                            } else {
                                c.cycles += 4;
                                c.c_le_s(22, 21);
                                c.lwc1(0, bcu_back(c, GP, 0x6370));
                                let t = !c.fcc();
                                c.lwc1(1, bcu_at(c, S0, 0x54));
                                if t {
                                    break 'b_173df8;
                                } else {
                                    c.cycles += 3;
                                    c.lwc1(0, bcu_back(c, GP, 0x636c));
                                    c.lwc1(3, bcu_at(c, S0, 0x68));
                                    break 'b_173dfc;
                                }
                            }
                        }
                    }
                    'b_173df0: {
                        c.cycles += 7;
                        c.div_s(12, 21, 20);
                        call_ptr(c, 0x0031_c228, 0x0017_3dc0)?;
                        c.c_lt_s(20, 22);
                        let t = !c.fcc();
                        c.mov_s(12, 0);
                        if t {
                            break 'b_173df0;
                        } else {
                            'b_173de8: {
                                c.cycles += 3;
                                c.c_lt_s(22, 21);
                                let t = !c.fcc();
                                if t {
                                    break 'b_173de8;
                                } else {
                                    c.cycles += 3;
                                    c.lwc1(0, bcu_back(c, GP, 0x6368));
                                    c.add_s(12, 12, 0);
                                    break 'b_173df0;
                                }
                            }
                            c.cycles += 2;
                            c.lwc1(0, bcu_back(c, GP, 0x6364));
                            c.sub_s(12, 12, 0);
                            break 'b_173df0;
                        }
                    }
                    c.cycles += 1;
                    c.mov_s(0, 12);
                    break 'b_173df4;
                }
                c.cycles += 1;
                c.lwc1(1, bcu_at(c, S0, 0x54));
                break 'b_173df8;
            }
            c.cycles += 1;
            c.lwc1(3, bcu_at(c, S0, 0x68));
            break 'b_173dfc;
        }

        // Pitch (+0x54) += (pitchE - Pitch) * PitchLookRate / 60.
        c.cycles += 16;
        c.sub_s(0, 0, 1);
        c.lwc1(2, bcu_back(c, GP, 0x6360));
        c.g[RA][0] = c.ld(bcu_at(c, SP, 0x90));
        c.lwc1(25, bcu_at(c, SP, 0xd8));
        c.mul_s(0, 0, 3);
        c.lwc1(24, bcu_at(c, SP, 0xd0));
        c.lwc1(23, bcu_at(c, SP, 0xc8));
        c.lwc1(22, bcu_at(c, SP, 0xc0));
        c.mul_s(0, 0, 2);
        c.lwc1(21, bcu_at(c, SP, 0xb8));
        c.lwc1(20, bcu_at(c, SP, 0xb0));
        c.add_s(1, 1, 0);
        c.swc1(1, bcu_at(c, S0, 0x54));
        c.lq(16, bcu_at(c, SP, 0xa0));
        c.g[SP][0] = sx(bcu_at(c, SP, 0xe0));
        ret(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{elf_image, fakes, put_vec, Rng, DATA};

    /// The trigger camera (`self`), its rider target object, and the target's
    /// vtable (rider methods in slots 1, 3 and 4, at +8, +0x18 and +0x20).
    const BCU_SELF: u32 = DATA + 0x1000;
    const BCU_TARGET: u32 = DATA + 0x2000;
    const BCU_VT: u32 = DATA + 0x3000;

    /// A uniform value in -m..m, as float bits.
    fn bcu_sym(r: &mut Rng, m: f32) -> u32 {
        (((r.next_u32() as f32 / u32::MAX as f32) * 2.0 - 1.0) * m).to_bits()
    }

    #[test]
    fn bounded_camera_algorithm_base_update_matches() {
        fakes::covered(
            ee::s02::f_001736e0__generated,
            bounded_camera_algorithm_base_update,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = fakes::machine(c, seed, 0x0017_36e0, 3);
                c.pc = 0x0017_36e0;
                let unit = |r: &mut Rng| r.next_u32() as f32 / u32::MAX as f32;
                // The rider's vtable: its three method slots call fakes.
                c.bus.write32(BCU_TARGET, BCU_VT);
                for (k, off) in [0x8u32, 0x18, 0x20].into_iter().enumerate() {
                    c.bus.write16(BCU_VT + off, 0x10 * k as u16);
                    c.bus
                        .write32(BCU_VT + off + 4, fakes::FAKE + 0x10 * k as u32);
                }
                // The rider's position, velocity and front land in the
                // stack vectors (the fake methods write nothing): focus at
                // sp+0, velocity at +0x10, front at +0x20 of the new frame.
                let frame = crate::difftest::SP - 0xe0;
                let pos = [
                    bcu_sym(&mut r, 50.0),
                    bcu_sym(&mut r, 50.0),
                    bcu_sym(&mut r, 50.0),
                    0,
                ];
                let speed = if seed % 4 == 0 { 4000.0 } else { 30.0 };
                let vel = [
                    bcu_sym(&mut r, speed),
                    bcu_sym(&mut r, speed),
                    bcu_sym(&mut r, speed),
                    0,
                ];
                let mut front = [
                    bcu_sym(&mut r, 1.0),
                    bcu_sym(&mut r, 1.0),
                    bcu_sym(&mut r, 1.0),
                    0,
                ];
                // Zero front components reach the x == 0 and y == 0 atan cases.
                if seed % 6 == 0 {
                    front[0] = 0;
                }
                if seed % 12 == 0 {
                    front[1] = 0;
                }
                put_vec(c, frame, pos);
                put_vec(c, frame + 0x10, vel);
                put_vec(c, frame + 0x20, front);
                // The camera: with front.x = 0, the focus x is the position's
                // x, so Position.x equal to it makes toFocus.x zero.
                let me = BCU_SELF;
                c.bus.write32(me + 0x30, BCU_TARGET);
                let position = if seed % 6 == 0 {
                    [pos[0], bcu_sym(&mut r, 50.0), bcu_sym(&mut r, 50.0), 0]
                } else {
                    [
                        bcu_sym(&mut r, 50.0),
                        bcu_sym(&mut r, 50.0),
                        bcu_sym(&mut r, 50.0),
                        0,
                    ]
                };
                put_vec(c, me + 0x40, position);
                c.bus.write32(me + 0x50, bcu_sym(&mut r, 3.0)); // Heading
                c.bus.write32(me + 0x54, bcu_sym(&mut r, 1.0)); // Pitch
                c.bus.write32(me + 0x58, (unit(&mut r) * 2.0).to_bits()); // PitchMultiplier
                c.bus.write32(me + 0x5c, (unit(&mut r) * 120.0).to_bits()); // HeadingAdjustRate
                c.bus.write32(me + 0x60, (unit(&mut r) * 120.0).to_bits()); // PitchAdjustRate
                c.bus.write32(me + 0x64, (unit(&mut r) * 120.0).to_bits()); // HeadingLookRate
                c.bus.write32(me + 0x68, (unit(&mut r) * 120.0).to_bits()); // PitchLookRate
                c.bus.write32(me + 0x6c, bcu_sym(&mut r, 5.0)); // HeightOffset
                                                                // Distances far apart on a third of the seeds (the |dd| > 5000
                                                                // branch), near otherwise (the k-scaled step).
                let ideal = if seed % 3 == 0 {
                    10000.0 * unit(&mut r)
                } else {
                    30.0 * unit(&mut r)
                };
                c.bus.write32(me + 0x70, ideal.to_bits()); // IdealDist
                c.bus.write32(me + 0x74, bcu_sym(&mut r, 20.0)); // LeadDistance
                c.bus.write32(me + 0x78, bcu_sym(&mut r, 30.0)); // PitchOffset
                c.g[A0][0] = sx(me);
            },
        );
    }
}
