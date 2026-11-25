//! Staging module oc573 (wave l567.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc573`.

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
    (0x0034_1388, boost_node_ctor, "cBoostNode_cBoostNode1"),
    (0x0034_1818, boost_node_apply_boost, "cBoostNode_applyBoost"),
];

// The cBoostNode constructor's callees and globals.
/// `cInstanceNode_cInstanceNode1` (the WorldObj constructor).
const BOOST_CTOR_WORLD_OBJ: Callee = Callee {
    addr: 0x0034_fb00,
    f: ee::s11::f_0034fb00,
};
/// Return address after the WorldObj constructor call.
const BOOST_CTOR_RET_WORLD: u32 = 0x0034_13a4;
/// `cBoostNode_vtbl`.
const BOOST_NODE_VTBL: u32 = 0x0049_14e0;
/// `cAppMan_TheAppMan` (a GP-relative pointer); its `TimerHz` int is at +0x10.
const GP_APP_MAN: u32 = 0x2a74;
const APP_MAN_TIMER_HZ: u32 = 0x10;
/// The float 27.7777786f (game units to speed), at gp-0x2b98.
const GP_SPEED_SCALE: u32 = 0x2b98;
/// `t4vect_Zero` and `t4vect_YAxis` (the zero and up vectors).
const T4VECT_ZERO: u32 = 0x004f_f120;
const T4VECT_Y_AXIS: u32 = 0x004f_f150;
/// `tMakeBoostData` (the definition the node is built from).
const DEF_DRAG: u32 = 0x0c;
const DEF_MAX_SPEED: u32 = 0x10;
const DEF_BOOST_TIME: u32 = 0x08;
const DEF_TYPE: u32 = 0x04;
const DEF_BOOST_X: u32 = 0x14;
const DEF_BOOST_Y: u32 = 0x18;
const DEF_BOOST_Z: u32 = 0x1c;
/// `cBoostNode` fields.
const BOOST_VPTR: u32 = 0x0c;
const BOOST_INSTANCE: u32 = 0x18;
const BOOST_TIME: u32 = 0x3c;
const BOOST_BOOST: u32 = 0x20;
const BOOST_MAX_SPEED: u32 = 0x30;
const BOOST_DRAG: u32 = 0x34;
const BOOST_BOOST_TIME: u32 = 0x38;
const BOOST_TYPE: u32 = 0x40;
/// The instance's `m_fmTransform` (a 4x4 matrix at +0x10).
const INSTANCE_MATRIX: u32 = 0x10;

/// `cBoostNode::cBoostNode(self, bucket, nodeType, instance, def)`: the
/// WorldObj part, then the boost vector (the definition's direction, or
/// the velocity-free up or zero vector when it has none; a negated,
/// transformed copy for type 2), the max speed and drag, and the time.
pub fn boost_node_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save_ra(c, 0x30);
        c.g[S1][0] = c.g[T0][0];
        BOOST_CTOR_WORLD_OBJ.call(c, BOOST_CTOR_RET_WORLD)?;
        let this = c.g[S0][0] as u32;
        let def = c.g[S1][0] as u32;

        'b_341488: {
            'b_34143c: {
                c.g[V0][0] = sx(BOOST_NODE_VTBL);
                c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(GP_APP_MAN)).lw(c);
                c.mtc1(0, 2);
                let vptr = c.g[V0][0];
                Addr(this.wrapping_add(BOOST_VPTR)).sw(c, vptr);
                c.g[A0][0] = Addr(def.wrapping_add(DEF_TYPE)).lh(c);
                let ty = c.g[A0][0];
                Addr(this.wrapping_add(BOOST_TYPE)).sw(c, ty);
                // BoostTime = (int)(def->BoostTime * (float)TimerHz).
                c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(APP_MAN_TIMER_HZ));
                c.cvt_s_w(1, 1);
                c.lwc1(0, def.wrapping_add(DEF_BOOST_TIME));
                c.mul_s(0, 0, 1);
                c.cvt_w_s(1, 0);
                c.swc1(1, this.wrapping_add(BOOST_BOOST_TIME));
                // All three BoostX/Y/Z zero: the up or zero vector.
                c.lwc1(1, def.wrapping_add(DEF_BOOST_X));
                c.c_eq_s(1, 2);
                if !c.fcc() {
                    c.lwc1(0, def.wrapping_add(DEF_BOOST_X));
                    break 'b_34143c;
                }
                c.cycles += 5;
                c.lwc1(0, def.wrapping_add(DEF_BOOST_Y));
                c.c_eq_s(0, 2);
                if !c.fcc() {
                    c.lwc1(0, def.wrapping_add(DEF_BOOST_X));
                    break 'b_34143c;
                }
                'b_341438: {
                    c.cycles += 5;
                    c.lwc1(0, def.wrapping_add(DEF_BOOST_Z));
                    c.c_eq_s(0, 2);
                    let nonzero = !c.fcc();
                    c.g[V0][0] = 0x2;
                    if nonzero {
                        break 'b_341438;
                    }
                    'b_34142c: {
                        'b_341428: {
                            // Type 2 boosts along the up vector, others along zero.
                            c.cycles += 2;
                            if c.g[A0][0] != c.g[V0][0] {
                                break 'b_341428;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = sx(T4VECT_Y_AXIS);
                            break 'b_34142c;
                        }
                        c.cycles += 1;
                        c.g[V0][0] = sx(T4VECT_ZERO);
                    }
                    c.cycles += 3;
                    c.lq(3, c.g[V0][0] as u32);
                    c.sq(3, this.wrapping_add(BOOST_BOOST));
                    break 'b_341488;
                }
                c.cycles += 1;
                c.lwc1(0, def.wrapping_add(DEF_BOOST_X));
            }
            // The non-zero vector: normalise (def dir) through VU0.
            c.cycles += 19;
            c.lwc1(1, def.wrapping_add(DEF_BOOST_Y));
            c.lwc1(2, def.wrapping_add(DEF_BOOST_Z));
            c.swc1(0, sp(c).wrapping_add(0x10));
            c.swc1(1, sp(c).wrapping_add(0x14));
            c.swc1(2, sp(c).wrapping_add(0x18));
            c.sw(sp(c).wrapping_add(0x1c), 0u64);
            c.lqc2(3, sp(c).wrapping_add(0x10));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().rsqrt(0, 3, 4, 0);
            c.vu0().waitq();
            c.vu0().mul(0b1111, 5, 3, Src::Q);
            c.sqc2(5, sp(c).wrapping_add(0x20));
            c.lq(2, sp(c).wrapping_add(0x20));
            c.sq(2, this.wrapping_add(BOOST_BOOST));
            c.sq(2, sp(c));
            break 'b_341488;
        }

        'b_3414b0: {
            c.cycles += 9;
            c.lwc1(0, def.wrapping_add(DEF_MAX_SPEED));
            c.g[V0][0] = 0;
            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(GP_SPEED_SCALE));
            c.g[V1][0] = Addr(this.wrapping_add(BOOST_TYPE)).lw(c);
            c.mul_s(0, 0, 1);
            c.swc1(0, this.wrapping_add(BOOST_MAX_SPEED));
            c.lwc1(1, def.wrapping_add(DEF_DRAG));
            let no_type = c.g[V1][0] == 0;
            c.swc1(1, this.wrapping_add(BOOST_DRAG));
            if !no_type {
                c.cycles += 1;
                c.g[V0][0] = Addr(this.wrapping_add(BOOST_BOOST_TIME)).lw(c);
            }
        }

        'b_341530: {
            c.cycles += 4;
            let time = c.g[V0][0];
            Addr(this.wrapping_add(BOOST_TIME)).sw(c, time);
            c.g[V0][0] = 0x2;
            let not_type2 = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = c.g[S0][0];
            if not_type2 {
                break 'b_341530;
            }
            // Type 2: the boost points the other way through the instance matrix.
            c.cycles += 28;
            c.g[V1][0] = Addr(this.wrapping_add(BOOST_INSTANCE)).lw(c);
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 0);
            c.mfc1(2, 0);
            c.lqc2(4, this.wrapping_add(BOOST_BOOST));
            c.qmtc2(2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, sp(c).wrapping_add(0x20));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(INSTANCE_MATRIX));
            c.lq(2, sp(c).wrapping_add(0x20));
            c.sq(2, sp(c).wrapping_add(0x10));
            c.lqc2(8, sp(c).wrapping_add(0x10));
            c.lqc2(4, c.g[V1][0] as u32);
            c.lqc2(5, (c.g[V1][0] as u32).wrapping_add(0x10));
            c.lqc2(6, (c.g[V1][0] as u32).wrapping_add(0x20));
            c.lqc2(7, (c.g[V1][0] as u32).wrapping_add(0x30));
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.sqc2(12, sp(c).wrapping_add(0x20));
            c.lwc1(0, this.wrapping_add(BOOST_MAX_SPEED));
            c.lq(2, sp(c).wrapping_add(0x20));
            c.neg_s(0, 0);
            c.sq(2, this.wrapping_add(BOOST_BOOST));
            c.sq(2, sp(c));
            c.swc1(0, this.wrapping_add(BOOST_MAX_SPEED));
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S0, 0x50);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    });
}

// cBoostNode_applyBoost callees.
/// `cObjectInterface_getUpdateTime`.
const APPLY_GET_UPDATE_TIME: u32 = 0x002d_1c70;
/// Return addresses of the calls in `cBoostNode_applyBoost`.
const APPLY_RET_VELOCITY: u32 = 0x0034_1844;
const APPLY_RET_UPDATE_TYPE2: u32 = 0x0034_1954;
const APPLY_RET_APPLY_TYPE2: u32 = 0x0034_1988;
const APPLY_RET_UPDATE: u32 = 0x0034_19e4;
const APPLY_RET_APPLY: u32 = 0x0034_1a18;
/// `obj->getVelocity()` and `obj->applyBoost(&vel)` are read from the
/// object's vtable: halfword delta at +0x10 / +0x20, function at +0x14 / +0x24.
const VT_GET_VELOCITY: u32 = 0x10;
const VT_APPLY_BOOST: u32 = 0x20;
/// `cBoostNodeState` fields (`&self->Boost`).
const BOOST_STATE_MAX_SPEED: u32 = 0x10;
const BOOST_STATE_DRAG: u32 = 0x14;

/// `void cBoostNode::applyBoost(cBeing* obj)`: the object's velocity along
/// the boost direction (the velocity itself when the direction is zero); for
/// type 2 a push when moving against it, else the room left to MaxSpeed when
/// positive. The push is scaled by drag and the update time and handed to
/// `obj->applyBoost`.
pub fn boost_node_apply_boost(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x60);
        c.g[S1][0] = c.g[A1][0];
        let obj = c.g[S1][0] as u32;
        let this = c.g[S0][0] as u32;

        'b_3418c4: {
            'b_3418c0: {
                // vel = obj->getVelocity(); dir = self->Boost
                c.g[V1][0] = Addr(obj).lw(c);
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(VT_GET_VELOCITY)).lh(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(VT_GET_VELOCITY + 4)).lw(c);
                let get_velocity = c.g[V0][0] as u32;
                c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, get_velocity, APPLY_RET_VELOCITY)?;
                let vel = c.g[V0][0] as u32;
                c.lq(3, this.wrapping_add(BOOST_BOOST));
                c.lq(4, vel);
                c.sq(3, sp(c).wrapping_add(0x10));
                c.mtc1(0, 1);
                c.lwc1(0, sp(c).wrapping_add(0x10));
                c.c_eq_s(0, 1);
                let dir_x_nonzero = !c.fcc();
                c.sq(4, sp(c));
                if dir_x_nonzero {
                    break 'b_3418c0;
                }
                c.cycles += 5;
                c.lwc1(0, sp(c).wrapping_add(0x14));
                c.c_eq_s(0, 1);
                let dir_y_nonzero = !c.fcc();
                c.lwc1(0, sp(c).wrapping_add(0x18));
                if dir_y_nonzero {
                    break 'b_3418c0;
                }
                c.cycles += 4;
                c.c_eq_s(0, 1);
                if !c.fcc() {
                    c.lwc1(0, this.wrapping_add(BOOST_MAX_SPEED));
                    break 'b_3418c4;
                }
                // dir is zero: dir = normalize(vel), through VU0.
                c.cycles += 13;
                c.lqc2(3, sp(c));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().rsqrt(0, 3, 4, 0);
                c.vu0().waitq();
                c.vu0().mul(0b1111, 5, 3, Src::Q);
                c.sqc2(5, sp(c).wrapping_add(0x30));
                c.lq(2, sp(c).wrapping_add(0x30));
                c.sq(2, sp(c).wrapping_add(0x10));
                c.sq(2, sp(c).wrapping_add(0x20));
            }
            c.cycles += 1;
            c.lwc1(0, this.wrapping_add(BOOST_MAX_SPEED));
        }

        'b_341a1c: {
            'b_341990: {
                // alongSpeed = dot(vel, dir); diff = MaxSpeed - alongSpeed.
                c.cycles += 14;
                c.lqc2(3, sp(c));
                c.lqc2(5, sp(c).wrapping_add(0x10));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(2, 4);
                c.mtc1(2, 2);
                let this = c.g[S0][0] as u32;
                c.g[V1][0] = Addr(this.wrapping_add(BOOST_TYPE)).lw(c);
                c.g[A0][0] = sx(this.wrapping_add(BOOST_BOOST));
                c.g[V0][0] = 0x2;
                let not_type2 = c.g[V1][0] != c.g[V0][0];
                c.sub_s(1, 0, 2);
                if not_type2 {
                    break 'b_341990;
                }
                // Type 2 pushes only while moving along the boost (dot >= 0).
                c.cycles += 5;
                c.mtc1(0, 0);
                c.c_le_s(0, 2);
                let against = !c.fcc();
                frame.restore(c, S0, 0x80);
                if against {
                    break 'b_341a1c;
                }
                c.cycles += 32;
                c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(BOOST_STATE_MAX_SPEED));
                c.mfc1(3, 0);
                c.lqc2(4, sp(c).wrapping_add(0x10));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, sp(c).wrapping_add(0x50));
                c.lq(2, sp(c).wrapping_add(0x50));
                c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(BOOST_STATE_DRAG));
                c.sq(2, sp(c).wrapping_add(0x40));
                c.mfc1(3, 0);
                c.lqc2(4, sp(c).wrapping_add(0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, sp(c).wrapping_add(0x50));
                c.lq(2, sp(c).wrapping_add(0x50));
                c.sq(2, sp(c).wrapping_add(0x30));
                call_ptr(c, APPLY_GET_UPDATE_TIME, APPLY_RET_UPDATE_TYPE2)?;
                c.mfc1(2, 0);
                c.lqc2(4, sp(c).wrapping_add(0x30));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, sp(c).wrapping_add(0x50));
                c.g[A1][0] = sx(sp(c).wrapping_add(0x20));
                c.lq(2, sp(c).wrapping_add(0x50));
                c.g[V1][0] = Addr(obj).lw(c);
                c.sq(2, sp(c).wrapping_add(0x20));
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(VT_APPLY_BOOST)).lh(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(VT_APPLY_BOOST + 4)).lw(c);
                let apply = c.g[V0][0] as u32;
                c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, apply, APPLY_RET_APPLY_TYPE2)?;
                frame.restore(c, S0, 0x80);
                break 'b_341a1c;
            }
            // Other types push the room left to MaxSpeed while it is positive.
            c.cycles += 5;
            c.mtc1(0, 0);
            c.c_lt_s(0, 1);
            let no_room = !c.fcc();
            frame.restore(c, S0, 0x80);
            if no_room {
                break 'b_341a1c;
            }
            c.cycles += 30;
            c.mfc1(3, 1);
            c.lqc2(4, sp(c).wrapping_add(0x10));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, sp(c).wrapping_add(0x50));
            c.lq(2, sp(c).wrapping_add(0x50));
            c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(BOOST_STATE_DRAG));
            c.sq(2, sp(c).wrapping_add(0x40));
            c.mfc1(3, 0);
            c.lqc2(4, sp(c).wrapping_add(0x40));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, sp(c).wrapping_add(0x50));
            c.lq(2, sp(c).wrapping_add(0x50));
            c.sq(2, sp(c).wrapping_add(0x30));
            call_ptr(c, APPLY_GET_UPDATE_TIME, APPLY_RET_UPDATE)?;
            c.mfc1(2, 0);
            c.lqc2(4, sp(c).wrapping_add(0x30));
            c.qmtc2(2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, sp(c).wrapping_add(0x50));
            c.g[A1][0] = sx(sp(c).wrapping_add(0x20));
            c.lq(2, sp(c).wrapping_add(0x50));
            c.g[V1][0] = Addr(obj).lw(c);
            c.sq(2, sp(c).wrapping_add(0x20));
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(VT_APPLY_BOOST)).lh(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(VT_APPLY_BOOST + 4)).lw(c);
            let apply = c.g[V0][0] as u32;
            c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, apply, APPLY_RET_APPLY)?;
            frame.restore(c, S0, 0x80);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x70);
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::{FAKE, LOG_RESULTS};
    use crate::difftest::{check, fill, put_vec, Rng, DATA, GAME_GP};
    use crate::game::fe::feasyncfile::tests::alloc_base_3;

    /// The `TimerHz` global's struct and the speed scale constant (gp-0x2b98).
    const APP_MAN: u32 = DATA + 0x7000;
    const SPEED_SCALE: f32 = 27.777_779;

    /// A direction in six patterns: zero, then one, two or three non-zero
    /// components (or all three) so every branch of the zero test runs.
    fn boost_dir(r: &mut Rng, mask: u32) -> [u32; 3] {
        [
            if matches!(mask, 1 | 4 | 5) {
                r.f32(1.0)
            } else {
                0
            },
            if matches!(mask, 2 | 4 | 5) {
                r.f32(1.0)
            } else {
                0
            },
            if matches!(mask, 3 | 4 | 5) {
                r.f32(1.0)
            } else {
                0
            },
        ]
    }

    #[test]
    fn boost_node_ctor_matches() {
        check(
            ee::s11::f_00341388__generated,
            boost_node_ctor,
            300,
            &|c, seed| {
                let mut r = alloc_base_3(c, seed, 0x34_1388, 0);
                c.pc = 0x0034_1388;
                c.g[GP][0] = GAME_GP as u64;
                let this = DATA + 0x4000;
                let def = DATA + 0x5000;
                let instance = DATA + 0x6000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x8000);
                c.g[A2][0] = (seed % 3) as u64;
                c.g[A3][0] = sx(instance);
                c.g[T0][0] = sx(def);
                fill(c, &mut r, this, 16);
                fill(c, &mut r, instance, 32);
                c.bus.write32(GAME_GP + GP_APP_MAN, APP_MAN);
                c.bus.write32(APP_MAN + APP_MAN_TIMER_HZ, 30 + seed % 40);
                c.bus
                    .write32(GAME_GP - GP_SPEED_SCALE, SPEED_SCALE.to_bits());
                put_vec(c, T4VECT_ZERO, [0, 0, 0, 0]);
                put_vec(c, T4VECT_Y_AXIS, [0, 1.0f32.to_bits(), 0, 0]);
                // Type 0, 1 or 2 (type 2 is the reversed, transformed boost).
                c.bus
                    .write16(def + DEF_TYPE, [0u16, 2, 1][seed as usize % 3]);
                c.bus.write32(def + DEF_BOOST_TIME, r.f32(20.0));
                c.bus.write32(def + DEF_DRAG, r.f32(2.0));
                c.bus.write32(def + DEF_MAX_SPEED, r.f32(50.0));
                let [x, y, z] = boost_dir(&mut r, seed % 6);
                c.bus.write32(def + DEF_BOOST_X, x);
                c.bus.write32(def + DEF_BOOST_Y, y);
                c.bus.write32(def + DEF_BOOST_Z, z);
            },
        );
    }

    #[test]
    fn boost_node_apply_boost_matches() {
        check(
            ee::s11::f_00341818__generated,
            boost_node_apply_boost,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34_1818, 0);
                c.pc = 0x0034_1818;
                let this = DATA + 0x4000;
                let obj = DATA + 0x5000;
                let vel = DATA + 0x4800;
                let vtbl = DATA + 0x6800;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(obj);
                fill(c, &mut r, this, 16);
                // self->Boost (the direction), MaxSpeed, Drag, Type.
                let [x, y, z] = boost_dir(&mut r, seed % 6);
                c.bus.write32(this + BOOST_BOOST, x);
                c.bus.write32(this + BOOST_BOOST + 4, y);
                c.bus.write32(this + BOOST_BOOST + 8, z);
                c.bus.write32(this + BOOST_BOOST + 12, 0);
                c.bus.write32(this + BOOST_MAX_SPEED, r.f32(30.0));
                c.bus.write32(this + BOOST_DRAG, r.f32(2.0));
                c.bus
                    .write32(this + BOOST_TYPE, [0u32, 2, 1][seed as usize % 3]);
                // The object's velocity (the first fake call returns its pointer),
                // and its vtable: getVelocity and applyBoost are fake targets.
                fill(c, &mut r, vel, 4);
                c.bus.write32(LOG_RESULTS, vel);
                c.bus.write32(obj, vtbl);
                c.bus.write16(vtbl + VT_GET_VELOCITY, 0);
                c.bus.write32(vtbl + VT_GET_VELOCITY + 4, FAKE);
                c.bus.write16(vtbl + VT_APPLY_BOOST, 8);
                c.bus.write32(vtbl + VT_APPLY_BOOST + 4, FAKE + 0x40);
            },
        );
    }
}
