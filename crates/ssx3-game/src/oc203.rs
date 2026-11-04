//! Staging wave 201 (oc203): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size),
//! one function per writer. Each reproduces the generated body's registers,
//! cycles, memory and call order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002d_f920, c_dynamic_spray_fx_update, "cDynamicSprayFX_Update"),
    (0x002f_6518, rider_base_update_cheat_code, "RiderBase_UpdateCheatCode"),
    (0x0041_1c38, unpack_d, "__unpack_d"),
    (0x003c_d878, snd_utk_decode_frame, "SND_UtkDecodeFrame"),
];

// ------------------------------------------------- cDynamicSprayFX_Update

/// `cDynamicSprayFX` fields (the board spray of one rider; `a0`).
mod spray_fx {
    /// The rider object (`cRider*`).
    pub const OBJ: u32 = 0x00;
    /// The ten particle emitters (0x210 bytes each).
    pub const EMITTERS: u32 = 0x28;
    /// Matrix rows 1, 2, 0 of the spray bone (copied each update).
    pub const ROW1: u32 = 0x30;
    pub const ROW2: u32 = 0x40;
    pub const ROW0: u32 = 0x50;
    /// Non-zero: the emitters still have to be set up.
    pub const NEEDS_SETUP: u32 = 0x60;
    /// Last update's speed.
    pub const PREV_SPEED: u32 = 0x74;
    /// Non-zero once the bones were looked up.
    pub const BONES_FOUND: u32 = 0x80;
    /// Surface colour of the snow under the board (rgb * 2, alpha 1).
    pub const COLOUR: u32 = 0x90;
    /// Spray bone position (matrix row 3).
    pub const POS: u32 = 0xa0;
    /// Whether the spray is active this update.
    pub const ACTIVE: u32 = 0xb0;
    /// Signed sideways speed.
    pub const SIDE_SPEED: u32 = 0xb4;
    /// Speed (length of the physics velocity) and its absolute value.
    pub const SPEED: u32 = 0xb8;
    pub const ABS_SPEED: u32 = 0xbc;
    /// Direction of travel (normalised when the speed is positive).
    pub const DIR: u32 = 0xc0;
    /// This rider's race data (game + 0x44, 0xb0 bytes per rider).
    pub const RACE_DATA: u32 = 0xd0;
    /// The rider's main state, as of this update.
    pub const MAIN_STATE: u32 = 0xd4;
    /// The rider's emitter-enable state the emitters were last set for.
    pub const ENABLE_STATE: u32 = 0x128;
    /// Which side the spray goes to (-1, 0 or +1).
    pub const SIDE: u32 = 0x12c;
}

/// `cRider` fields read by the spray update.
mod spray_rider {
    pub const VELOCITY: u32 = 0x1e0;
    pub const FORWARD_SPEED: u32 = 0x1f0;
    pub const SLIP: u32 = 0x214;
    /// Non-zero: goofy stance (mirrors the spray side).
    pub const GOOFY: u32 = 0x320;
    /// Non-zero: in a trick (the side follows the trick animation).
    pub const IN_TRICK: u32 = 0x330;
    pub const BOARD_DIR: u32 = 0x370;
    pub const INDEX: u32 = 0x438;
    /// The physics sub-object (with its own vtable).
    pub const PHYSICS: u32 = 0x6c0;
    pub const MODEL: u32 = 0x780;
    pub const ANIM: u32 = 0x784;
    pub const ENABLE_STATE: u32 = 0x898;
    pub const SPRAY_BONE: u32 = 0x8a4;
    pub const GRINDING: u32 = 0xac4;
    pub const GRIND_OFF: u32 = 0xad0;
    pub const GRIND_RAIL: u32 = 0xafc;
    pub const GRIND_SPRAY: u32 = 0xb00;
}

/// Callees of `cDynamicSprayFX_Update` (direct calls through the shard).
mod spray_callee {
    use crate::call::Callee;
    use ssx3_ee as ee;

    pub const SETUP_EMITTERS: Callee = Callee {
        addr: 0x002d_f448,
        f: ee::s09::cDynamicSprayFX_SetupEmitters,
    };
    pub const FIND_BONES: Callee = Callee {
        addr: 0x002d_f4d0,
        f: ee::s09::cDynamicSprayFX_FindBones,
    };
    pub const QUEUE_BURST: Callee = Callee {
        addr: 0x002e_23e0,
        f: ee::s09::cDynamicSprayFX_QueueBurst,
    };
    pub const UPDATE_EMITTER_ENABLES: Callee = Callee {
        addr: 0x002e_2550,
        f: ee::s09::cDynamicSprayFX_UpdateEmitterEnables,
    };
    /// The per-emitter updates in call order: (enable flag offset in the
    /// emitter array, callee, return address).
    pub const EMITTERS: [(u32, Callee, u32); 8] = [
        (0x384, Callee { addr: 0x002e_02b8, f: ee::s09::cDynamicSprayFX_UpdateEmitter1 }, 0x002d_fdc4),
        (0x7a4, Callee { addr: 0x002d_fe88, f: ee::s09::cDynamicSprayFX_UpdateEmitter3 }, 0x002d_fddc),
        (0x174, Callee { addr: 0x002e_0ee8, f: ee::s09::cDynamicSprayFX_UpdateEmitter0 }, 0x002d_fdf4),
        (0x9b4, Callee { addr: 0x002e_1120, f: ee::s09::cDynamicSprayFX_UpdateEmitter4 }, 0x002d_fe0c),
        (0xbc4, Callee { addr: 0x002e_1598, f: ee::s09::cDynamicSprayFX_UpdateEmitter5 }, 0x002d_fe24),
        (0xfe4, Callee { addr: 0x002e_1a80, f: ee::s09::cDynamicSprayFX_UpdateEmitter7 }, 0x002d_fe3c),
        (0x1404, Callee { addr: 0x002e_2260, f: ee::s09::cDynamicSprayFX_UpdateEmitter9 }, 0x002d_fe54),
        (0x11f4, Callee { addr: 0x002e_1f70, f: ee::s09::cDynamicSprayFX_UpdateEmitter8 }, 0x002d_fe6c),
    ];
    /// `AnimController_GetLayerAnimId(anim, layer)`.
    pub const ANIM_LAYER_ID: u32 = 0x0031_2aa0;
    /// `Rider_GetMainState(rider)` / `Rider_GetSubState(rider)`.
    pub const RIDER_MAIN_STATE: u32 = 0x0011_fe98;
    pub const RIDER_SUB_STATE: u32 = 0x0011_fee8;
}

/// `s0 + off` (the spray), `base + off` for any other object.
#[inline(always)]
fn spray_at(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// `void cDynamicSprayFX_Update(cDynamicSprayFX* self)` (0x2df920): the
/// per-tick update of a rider's board spray. Sets the emitters and bones up
/// on first use, picks the spray side (from the trick animation and the
/// stance), measures the rider's sideways and forward speed and direction,
/// decides whether the spray is active (not in the air states, not in a
/// sub-state 9, grinding only on a spraying rail), copies the spray bone's
/// matrix, queues a burst on a sudden slow-down (or a rail landing), re-arms
/// the emitters when the rider's enable state changed, samples the snow
/// colour under the board, and runs every enabled emitter's update.
pub fn c_dynamic_spray_fx_update(c: &mut Cpu) {
    use spray_callee as callee;
    use ssx3_ee_rt::vu0::{Src, X};
    guest(c, |c| {
        // Prologue: s0 = self; s0, s1, ra and f20 saved.
        c.cycles += 9;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        frame.save_fpr(c, 20, 0x60);
        let this = c.g[S0][0];
        c.g[V0][0] = c.lw(spray_at(this, spray_fx::NEEDS_SETUP));
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            callee::SETUP_EMITTERS.call(c, 0x002d_f94c)?;
        }
        c.g[V0][0] = c.lw(spray_at(this, spray_fx::BONES_FOUND));

        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[A0][0] = this;
            callee::FIND_BONES.call(c, 0x002d_f960)?;
        }
        c.sw(spray_at(this, spray_fx::SIDE), 0);

        // The spray side: during a trick, from the trick animation (anims
        // 23-28, 37 and 38 spray to the stance side, all others to the
        // other side); mirrored for goofy riders.
        c.cycles += 4;
        c.g[V1][0] = c.lw(spray_at(this, spray_fx::OBJ));
        c.g[V0][0] = c.lw(spray_at(c.g[V1][0], spray_rider::IN_TRICK));
        if c.g[V0][0] != 0 {
            c.cycles += 8;
            c.g[A0][0] = c.lw(spray_at(c.g[V1][0], spray_rider::ANIM));
            c.g[A1][0] = 0x2;
            call_ptr(c, callee::ANIM_LAYER_ID, 0x002d_f980)?;
            let anim = c.g[V0][0];
            c.g[V1][0] = anim;
            c.g[V0][0] = (sx((anim as u32).wrapping_sub(0x17)) < 0x6) as u64;
            let stance_side = if c.g[V0][0] != 0 {
                // Anims 23-28.
                true
            } else {
                // Anims 37 and 38 (v0 is left 0x26 either way).
                c.cycles += 3;
                c.g[V0][0] = 0x26;
                let hit = if anim == 0x25 {
                    true
                } else {
                    c.cycles += 2;
                    anim == 0x26
                };
                if hit {
                    c.cycles += 1;
                }
                hit
            };
            c.g[V1][0] = c.lw(spray_at(this, spray_fx::OBJ));
            // +1.0 or -1.0, the other one for goofy riders.
            let (regular, goofy, goofy_cost): (u64, u64, u64) = if stance_side {
                (0x3f80_0000, sx(0xbf80_0000), 4)
            } else {
                (sx(0xbf80_0000), 0x3f80_0000, 3)
            };
            c.cycles += 5;
            c.g[AT][0] = regular;
            c.mtc1(AT, 0);
            c.g[V0][0] = c.lw(spray_at(c.g[V1][0], spray_rider::GOOFY));
            if c.g[V0][0] != 0 {
                c.cycles += goofy_cost;
                c.g[AT][0] = goofy;
                c.mtc1(AT, 0);
            }
            c.swc1(0, spray_at(this, spray_fx::SIDE));
            c.cycles += 1;
            c.g[V1][0] = c.lw(spray_at(this, spray_fx::OBJ));
        }

        // Sideways speed: -forward * (1 - |slip|), mirrored for goofy.
        c.cycles += 12;
        let obj = c.g[V1][0];
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 2);
        c.lwc1(0, spray_at(obj, spray_rider::SLIP));
        c.lwc1(1, spray_at(obj, spray_rider::FORWARD_SPEED));
        c.abs_s(0, 0);
        c.sub_s(2, 2, 0);
        c.neg_s(1, 1);
        c.mul_s(0, 1, 2);
        c.swc1(0, spray_at(this, spray_fx::SIDE_SPEED));
        c.g[V0][0] = c.lw(spray_at(obj, spray_rider::GOOFY));
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.neg_s(0, 0);
            c.swc1(0, spray_at(this, spray_fx::SIDE_SPEED));
        }
        c.g[A0][0] = c.lw(spray_at(this, spray_fx::OBJ));

        // Speed = |physics velocity| (virtual call on the physics
        // sub-object), direction = the rider's velocity, normalised.
        c.cycles += 26;
        let obj = c.g[A0][0];
        c.g[V1][0] = c.lw(spray_at(obj, spray_rider::PHYSICS));
        c.g[A0][0] = sx(spray_at(obj, spray_rider::PHYSICS));
        c.g[A1][0] = c.lh(spray_at(c.g[V1][0], 0x10));
        c.g[V0][0] = c.lw(spray_at(c.g[V1][0], 0x14));
        let method = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, method, 0x002d_fa48)?;
        c.lqc2(3, c.g[V0][0] as u32);
        spray_vu0_length(c);
        c.cfc2(A0, 22);
        c.mtc1(A0, 0);
        c.g[V1][0] = c.lw(spray_at(this, spray_fx::OBJ));
        c.abs_s(1, 0);
        c.swc1(0, spray_at(this, spray_fx::SPEED));
        c.swc1(1, spray_at(this, spray_fx::ABS_SPEED));
        c.mtc1(0, 2);
        c.lq(V0, spray_at(c.g[V1][0], spray_rider::VELOCITY));
        c.c_lt_s(2, 0);
        let moving = c.fcc();
        c.sq(V0, spray_at(this, spray_fx::DIR));
        if moving {
            c.cycles += 17;
            c.lqc2(3, spray_at(this, spray_fx::DIR));
            spray_vu0_length(c);
            c.cfc2(V0, 22);
            c.mtc1(V0, 0);
            c.mfc1(V1, 0);
            c.qmtc2(V1, 3);
            c.vu0().div(0, 3, 3, 0);
            c.lqc2(4, spray_at(this, spray_fx::DIR));
            c.vu0().waitq();
            c.vu0().mul(0b1111, 5, 4, Src::Q);
            c.sqc2(5, spray_at(this, spray_fx::DIR));
        }

        // Active unless the main state is 1, 2 or 4, the sub-state is 9, or
        // the rider grinds a rail that does not spray.
        c.cycles += 9;
        c.g[S1][0] = 0x1;
        c.g[A0][0] = c.lw(spray_at(this, spray_fx::OBJ));
        c.sw(spray_at(this, spray_fx::ACTIVE), c.g[S1][0]);
        call_ptr(c, callee::RIDER_MAIN_STATE, 0x002d_faec)?;
        let main_state = c.g[V0][0];
        c.g[V1][0] = main_state;
        c.g[V0][0] = (sx((main_state as u32).wrapping_sub(0x1)) < 0x2) as u64;
        c.sw(spray_at(this, spray_fx::MAIN_STATE), main_state);
        // None: inactive through the shared store (+1); Some(true): store
        // here; Some(false): stays active.
        let inactive: Option<bool> = if c.g[V0][0] != 0 {
            None
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x4;
            if main_state == 0x4 {
                Some(true)
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.lw(spray_at(this, spray_fx::OBJ));
                call_ptr(c, callee::RIDER_SUB_STATE, 0x002d_fb14)?;
                c.g[V1][0] = 0x9;
                if c.g[V0][0] == 0x9 {
                    Some(true)
                } else {
                    c.cycles += 4;
                    let obj = c.lw(spray_at(this, spray_fx::OBJ));
                    c.g[A0][0] = obj;
                    c.g[V0][0] = c.lw(spray_at(obj, spray_rider::GRINDING));
                    if c.g[V0][0] == 0 {
                        Some(false)
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(spray_at(obj, spray_rider::GRIND_OFF));
                        c.g[V1][0] = 0;
                        if c.g[V0][0] == 0 {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(spray_at(obj, spray_rider::GRIND_RAIL));
                            c.g[V1][0] = if c.g[V0][0] == 0 { 0 } else { c.g[S1][0] };
                        }
                        c.cycles += 2;
                        if c.g[V1][0] == 0 {
                            Some(true)
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(spray_at(obj, spray_rider::GRIND_SPRAY));
                            if c.g[V0][0] != 0 {
                                Some(false)
                            } else {
                                None
                            }
                        }
                    }
                }
            }
        };
        match inactive {
            Some(false) => {}
            Some(true) => {
                c.sw(spray_at(this, spray_fx::ACTIVE), 0);
                c.cycles += 1;
            }
            None => {
                c.cycles += 1;
                c.sw(spray_at(this, spray_fx::ACTIVE), 0);
                c.cycles += 1;
            }
        }
        c.g[T0][0] = c.lw(spray_at(this, spray_fx::OBJ));

        // Copy the spray bone's matrix rows (model matrices + bone * 64),
        // find this rider's race data, and the slow-down rate
        // (prev - speed) * 60 (f20).
        c.cycles += 49;
        let obj = c.g[T0][0];
        c.g[A3][0] = 0xb0;
        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[T1][0] = 0;
        c.g[A0][0] = c.lw(spray_at(obj, spray_rider::MODEL));
        c.g[V0][0] = c.lw(spray_at(obj, spray_rider::SPRAY_BONE));
        c.g[V1][0] = c.lw(spray_at(c.g[A0][0], 0x30));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
        c.lwc1(1, spray_at(this, spray_fx::PREV_SPEED));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.lwc1(2, spray_at(this, spray_fx::SPEED));
        c.lq(V0, c.g[V1][0] as u32);
        c.sub_s(1, 1, 2);
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3b88));
        c.sq(V0, spray_at(this, spray_fx::ROW0));
        c.sq(V0, sp(c));
        c.mul_s(20, 1, 0);
        c.g[V0][0] = c.lw(spray_at(obj, spray_rider::MODEL));
        c.g[A0][0] = c.lw(spray_at(obj, spray_rider::SPRAY_BONE));
        c.g[V1][0] = c.lw(spray_at(c.g[V0][0], 0x30));
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 6);
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3b84));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.lq(V0, spray_at(c.g[V1][0], 0x10));
        c.c_lt_s(0, 20);
        c.sq(V0, spray_at(this, spray_fx::ROW1));
        c.sq(V0, sp(c));
        for (row, dst) in [(0x20u32, spray_fx::ROW2), (0x30, spray_fx::POS)] {
            c.g[A1][0] = c.lw(spray_at(obj, spray_rider::MODEL));
            c.g[V1][0] = c.lw(spray_at(obj, spray_rider::SPRAY_BONE));
            c.g[A0][0] = c.lw(spray_at(c.g[A1][0], 0x30));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.lq(V0, spray_at(c.g[A0][0], row));
            c.sq(V0, spray_at(this, dst));
            c.sq(V0, sp(c));
        }
        c.g[V1][0] = c.lw(spray_at(obj, spray_rider::INDEX));
        c.g[A0][0] = c.lw(spray_at(c.g[A2][0], 0x84));
        c.mult(0, V1, V1, A3);
        c.g[V0][0] = c.lw(spray_at(c.g[A0][0], 0x44));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        let fast_slowdown = c.fcc();
        c.sw(spray_at(this, spray_fx::RACE_DATA), c.g[V0][0]);
        // burst (t1): a hard slow-down from a high speed, not in sub-state 9.
        let mut skip_reload_cost = false;
        if fast_slowdown {
            c.cycles += 4;
            c.c_lt_s(0, 2);
            if !c.fcc() {
                skip_reload_cost = true;
            } else {
                c.cycles += 4;
                c.g[A0][0] = obj;
                call_ptr(c, callee::RIDER_SUB_STATE, 0x002d_fc40)?;
                c.g[V0][0] ^= 0x9;
                c.g[T1][0] = (0 < c.g[V0][0]) as u64;
            }
        }
        if !skip_reload_cost {
            c.cycles += 1;
        }
        c.g[V1][0] = c.lw(spray_at(this, spray_fx::OBJ));

        // On a spraying rail the full rate counts and any slow-down of
        // 27.8 or more bursts; otherwise half the rate (f12).
        c.cycles += 6;
        let obj = c.g[V1][0];
        c.g[A0][0] = 0;
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(AT, 0);
        c.g[V0][0] = c.lw(spray_at(obj, spray_rider::GRINDING));
        c.mul_s(12, 20, 0);
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = c.lw(spray_at(obj, spray_rider::GRIND_OFF));
            if c.g[V0][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = c.lw(spray_at(obj, spray_rider::GRIND_RAIL));
                c.g[A0][0] = (0 < c.g[V0][0]) as u64;
            }
        }
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 5;
            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3b80));
            c.c_le_s(0, 20);
            let burst = c.fcc();
            c.g[T1][0] = 0x1;
            if !burst {
                c.cycles += 1;
                c.g[T1][0] = 0;
            }
            c.cycles += 2;
            c.mov_s(12, 20);
        }
        c.g[V0][0] = c.lw(spray_at(this, spray_fx::ACTIVE));

        // Queue the burst at pos + board direction * 10.
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            let burst = c.g[T1][0] != 0;
            c.g[A0][0] = this;
            if burst {
                c.cycles += 22;
                c.g[V0][0] = c.lw(spray_at(this, spray_fx::OBJ));
                c.g[AT][0] = 0x4120_0000;
                c.mtc1(AT, 0);
                c.g[A1][0] = c.g[SP][0];
                c.mfc1(V1, 0);
                c.lqc2(4, spray_at(c.g[V0][0], spray_rider::BOARD_DIR));
                c.qmtc2(V1, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x20));
                c.g[T0][0] = 0x1;
                c.lq(V0, frame.at(c, 0x20));
                c.sq(V0, frame.at(c, 0x10));
                c.lqc2(3, spray_at(this, spray_fx::POS));
                c.lqc2(4, frame.at(c, 0x10));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x20));
                c.lq(V0, frame.at(c, 0x20));
                c.g[V1][0] = c.lw(spray_at(this, spray_fx::OBJ));
                c.sq(V0, sp(c));
                c.g[A2][0] = sx(spray_at(c.g[V1][0], spray_rider::BOARD_DIR));
                c.g[A3][0] = c.lw(spray_at(c.g[V1][0], spray_rider::INDEX));
                callee::QUEUE_BURST.call(c, 0x002d_fd08)?;
            }
            c.cycles += 1;
        }
        c.lwc1(0, spray_at(this, spray_fx::SPEED));

        // Remember the speed; re-arm the emitters when the rider's enable
        // state changed.
        c.cycles += 6;
        c.g[A0][0] = c.lw(spray_at(this, spray_fx::OBJ));
        c.swc1(0, spray_at(this, spray_fx::PREV_SPEED));
        c.g[V1][0] = c.lw(spray_at(this, spray_fx::ENABLE_STATE));
        c.g[V0][0] = c.lw(spray_at(c.g[A0][0], spray_rider::ENABLE_STATE));
        if c.g[V1][0] != c.g[V0][0] {
            c.cycles += 3;
            c.g[A0][0] = this;
            callee::UPDATE_EMITTER_ENABLES.call(c, 0x002d_fd2c)?;
        }
        c.g[A0][0] = c.lw(spray_at(this, spray_fx::OBJ));

        // Snow colour: the physics sub-object's surface index (virtual
        // call) selects an entry of the surface table (0xf0 bytes each);
        // colour = (g, b, a, r) * 2 with alpha 1.
        c.cycles += 35;
        let obj = c.g[A0][0];
        c.g[V1][0] = c.lw(spray_at(obj, spray_rider::PHYSICS));
        c.g[A0][0] = sx(spray_at(obj, spray_rider::PHYSICS));
        c.g[A1][0] = c.lh(spray_at(c.g[V1][0], 0x38));
        c.g[V0][0] = c.lw(spray_at(c.g[V1][0], 0x3c));
        let method = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, method, 0x002d_fd48)?;
        c.g[A0][0] = 0xf0;
        c.g[V1][0] = 0x50_0000;
        c.mult(0, V0, V0, A0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5c68));
        c.g[AT][0] = 0x4000_0000;
        c.mtc1(AT, 4);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 5);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        let entry = c.g[V0][0];
        c.lwc1(3, spray_at(entry, 0x0));
        c.lwc1(0, spray_at(entry, 0x4));
        c.lwc1(1, spray_at(entry, 0x8));
        c.lwc1(2, spray_at(entry, 0xc));
        c.swc1(0, sp(c));
        c.swc1(1, frame.at(c, 0x4));
        c.swc1(2, frame.at(c, 0x8));
        c.swc1(3, frame.at(c, 0xc));
        c.lq(V0, sp(c));
        c.sq(V0, spray_at(this, spray_fx::COLOUR));
        c.mfc1(V1, 4);
        c.lqc2(4, spray_at(this, spray_fx::COLOUR));
        c.qmtc2(V1, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, spray_at(this, spray_fx::COLOUR));
        c.swc1(5, spray_at(this, spray_fx::COLOUR + 0xc));

        // Run every enabled emitter (flag at emitters + 0x174 + n * 0x210).
        let [first, second, middle @ .., last] = callee::EMITTERS;
        c.g[V0][0] = c.lw(spray_at(this, spray_fx::EMITTERS));
        c.g[V1][0] = c.lw(spray_at(c.g[V0][0], first.0));
        if c.g[V1][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = this;
            first.1.call(c, first.2)?;
            c.g[V0][0] = c.lw(spray_at(this, spray_fx::EMITTERS));
        }
        c.g[V1][0] = c.lw(spray_at(c.g[V0][0], second.0));
        c.cycles += 2;
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = this;
            second.1.call(c, second.2)?;
        }
        c.g[V0][0] = c.lw(spray_at(this, spray_fx::EMITTERS));
        for (flag, update, ret) in middle {
            c.cycles += 3;
            c.g[V1][0] = c.lw(spray_at(c.g[V0][0], flag));
            if c.g[V1][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = this;
                update.call(c, ret)?;
            }
            c.g[V0][0] = c.lw(spray_at(this, spray_fx::EMITTERS));
        }
        c.cycles += 3;
        c.g[V1][0] = c.lw(spray_at(c.g[V0][0], last.0));
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = this;
            last.1.call(c, last.2)?;
        }

        // Epilogue.
        frame.restore(c, S0, 0x50);
        c.cycles += 5;
        frame.restore(c, S1, 0x40);
        frame.restore_ra(c, 0x30);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// VU0 `|vf3.xyz|` into Q (vf4 = vf3 * vf3, vf6.w = vf0.w, ACC sum, sqrt).
#[inline(always)]
fn spray_vu0_length(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
}

// ---------------------------------------------- RiderBase_UpdateCheatCode

/// The rider's cheat-code state (`a0`; `self->rider` first).
mod cheat {
    /// The rider (`cRider*`).
    pub const RIDER: u32 = 0x00;
    /// Signed byte: 1 = the cheat is on, 0 = off, -1 = wrong input.
    pub const STATE: u32 = 0x04;
    /// Event and progress bits (bit 1: the cheat's variant flag).
    pub const FLAGS: u32 = 0x07;
}

/// `cRider` fields the cheat-code update drives.
mod cheat_rider {
    pub const VELOCITY: u32 = 0x1e0;
    /// Three rates set to 60 while the cheat is on (0 when off).
    pub const RATE_A: u32 = 0x2e8;
    pub const RATE_B: u32 = 0x2ec;
    pub const RATE_C: u32 = 0x2f0;
    /// The movement mode (11 = cheat mode).
    pub const MODE: u32 = 0x2f4;
    /// 1.0 while the cheat is on.
    pub const CHEAT_SCALE: u32 = 0x2f8;
    /// Must be positive for the speed/steer assist to run.
    pub const ASSIST: u32 = 0x2fc;
    /// 0 while the cheat is on, 1 when off.
    pub const NORMAL_PHYSICS: u32 = 0x304;
    /// The cheat's variant flag (1 when off).
    pub const VARIANT: u32 = 0x308;
    pub const MAX_SPEED: u32 = 0x30c;
    /// How much of the speed follows the steer direction (0 = none).
    pub const STEER: u32 = 0x310;
    /// How much of the speed is pulled up to the maximum.
    pub const BLEND: u32 = 0x314;
    /// The steer direction.
    pub const STEER_DIR: u32 = 0x3a0;
    /// The physics sub-object (with its own vtable).
    pub const PHYSICS: u32 = 0x6c0;
}

/// Callees and globals of `RiderBase_UpdateCheatCode`.
mod cheat_callee {
    use crate::call::Callee;
    use ssx3_ee as ee;

    pub const CLEAR: Callee = Callee {
        addr: 0x002f_64f0,
        f: ee::s09::RiderBase_ClearCheatCode,
    };
    pub const MATCH: Callee = Callee {
        addr: 0x002f_6910,
        f: ee::s09::RiderBase_MatchCheatCode,
    };
    /// `BELibrary_GetInterface` (through the function table).
    pub const BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;
    /// gp+: how many clears are still due.
    pub const CLEARS_DUE_GP: u32 = 0xad0;
    /// gp-: the world.
    pub const WORLD_GP: u32 = 0x848;
    /// gp-: .lit4 0.001 (minimum steer-direction length) and 0.0001
    /// (minimum velocity length).
    pub const MIN_DIR_LEN_GP: u32 = 0x3898;
    pub const MIN_VEL_LEN_GP: u32 = 0x3894;
    /// Non-zero: cheats switched off globally.
    pub const CHEATS_OFF: u32 = 0x0053_4b30;
    /// The zero vector the velocity is reset to.
    pub const ZERO_VEC: u32 = 0x004f_f130;
}

/// `base + off` for a guest pointer held in a register.
#[inline(always)]
fn cheat_at(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// VU0 `|vf3.xyz|` into Q.
#[inline(always)]
fn cheat_vu0_length(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
}

/// VU0 `vf5 = vf3 / |vf3.xyz|` (all four lanes).
#[inline(always)]
fn cheat_vu0_normalise(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().rsqrt(0, 3, 4, 0);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 5, 3, Src::Q);
}

/// `void RiderBase_UpdateCheatCode(CheatCode* self)` (0x2f6518): the
/// per-tick update of a rider's cheat code. Does nothing while the physics
/// sub-object's virtual query (slot 0x48) says so; otherwise runs a pending
/// clear, matches the input against the code, and switches the rider into
/// cheat mode 11 (rates 60, scale 1, variant flag from the state bits) or
/// back to mode 0. In cheat mode with a positive assist, pulls the speed up
/// toward the maximum (`max * blend + speed * (1 - blend)`), then turns the
/// velocity toward the steer direction (`dir * speed * steer + vel *
/// speed * (1 - steer)`, both normalised).
pub fn rider_base_update_cheat_code(c: &mut Cpu) {
    guest(c, |c| {
        // Prologue: s0 = self; s0 and ra saved.
        c.cycles += 13;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save_ra(c, 0x60);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0];

        // The physics sub-object's virtual query (vtable slot 0x48).
        c.g[A0][0] = c.lw(cheat_at(this, cheat::RIDER));
        let rider = c.g[A0][0];
        c.g[V1][0] = c.lw(cheat_at(rider, cheat_rider::PHYSICS));
        c.g[A0][0] = sx(cheat_at(rider, cheat_rider::PHYSICS));
        c.g[A1][0] = c.lh(cheat_at(c.g[V1][0], 0x48));
        c.g[V0][0] = c.lw(cheat_at(c.g[V1][0], 0x4c));
        let method = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, method, 0x002f_6544)?;
        if c.g[V0][0] == 0 {
            update_cheat_code_body(c, frame, this)?;
        }

        // Epilogue.
        frame.restore(c, S0, 0x70);
        c.cycles += 3;
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// The body of `RiderBase_UpdateCheatCode` after the virtual query said 0;
/// `Ok` on every normal exit (all go to the epilogue).
fn update_cheat_code_body(c: &mut Cpu, frame: Frame, this: u64) -> crate::call::Flow {
    use cheat_callee as callee;
    use cheat_rider as rider_f;
    use ssx3_ee_rt::vu0::{Src, X};

    // A pending clear (while at least two are due).
    c.cycles += 4;
    let gp = c.g[GP][0] as u32;
    c.g[V0][0] = c.lw(gp.wrapping_add(callee::CLEARS_DUE_GP));
    c.g[V0][0] = ((c.g[V0][0] as i64) < 2) as u64;
    if c.g[V0][0] == 0 {
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        callee::CLEAR.call(c, 0x002f_6564)?;
        let at = (c.g[GP][0] as u32).wrapping_add(callee::CLEARS_DUE_GP);
        c.g[V0][0] = c.lw(at);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        c.sw(at, c.g[V0][0]);
    }

    // Match the input, query the back end (result unused), and honour the
    // global cheats-off switch.
    c.cycles += 10;
    c.g[A0][0] = c.g[S0][0];
    callee::MATCH.call(c, 0x002f_6578)?;
    let gp = c.g[GP][0] as u32;
    c.g[V0][0] = c.lw(gp.wrapping_sub(callee::WORLD_GP));
    c.g[A1][0] = 0x7;
    c.g[A0][0] = c.lw(cheat_at(c.g[V0][0], 0x78));
    call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x002f_6588)?;
    c.g[V1][0] = 0x53_0000;
    c.g[V0][0] = c.lw(callee::CHEATS_OFF);
    if c.g[V0][0] != 0 {
        c.sb(cheat_at(this, cheat::STATE), 0);
    }

    // Enter or leave cheat mode.
    c.cycles += 4;
    c.g[V0][0] = c.lb(cheat_at(this, cheat::STATE));
    c.g[A1][0] = 0x1;
    if c.g[V0][0] != c.g[A1][0] {
        // Off: variant flag 1; leave mode 11 (if in it) for mode 0.
        c.g[V0][0] = c.lw(cheat_at(this, cheat::RIDER));
        c.cycles += 6;
        c.g[V1][0] = 0xb;
        c.sw(cheat_at(c.g[V0][0], rider_f::VARIANT), c.g[A1][0]);
        c.g[A0][0] = c.lw(cheat_at(this, cheat::RIDER));
        c.g[V0][0] = c.lw(cheat_at(c.g[A0][0], rider_f::MODE));
        if c.g[V0][0] != c.g[V1][0] {
            return Ok(());
        }
        c.cycles += 11;
        c.sw(cheat_at(c.g[A0][0], rider_f::MODE), 0);
        c.g[V0][0] = c.lw(cheat_at(this, cheat::RIDER));
        c.sw(cheat_at(c.g[V0][0], rider_f::CHEAT_SCALE), 0);
        c.g[V1][0] = c.lw(cheat_at(this, cheat::RIDER));
        c.sw(cheat_at(c.g[V1][0], rider_f::RATE_C), 0);
        c.g[V0][0] = c.lw(cheat_at(this, cheat::RIDER));
        c.sw(cheat_at(c.g[V0][0], rider_f::RATE_B), 0);
        c.g[V1][0] = c.lw(cheat_at(this, cheat::RIDER));
        c.sw(cheat_at(c.g[V1][0], rider_f::RATE_A), 0);
        c.g[V0][0] = c.lw(cheat_at(this, cheat::RIDER));
        c.sw(cheat_at(c.g[V0][0], rider_f::NORMAL_PHYSICS), c.g[A1][0]);
    } else {
        // On: mode 11, scale 1.0, rates 60.0, variant from flag bit 1.
        c.cycles += 23;
        c.g[V1][0] = c.lw(cheat_at(this, cheat::RIDER));
        c.g[V0][0] = 0xb;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 1);
        c.sw(cheat_at(c.g[V1][0], rider_f::MODE), c.g[V0][0]);
        c.g[AT][0] = 0x4270_0000;
        c.mtc1(AT, 0);
        c.g[V0][0] = c.lbu(cheat_at(this, cheat::FLAGS));
        c.g[A0][0] = c.lw(cheat_at(this, cheat::RIDER));
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
        c.g[V0][0] &= 0x1;
        c.sw(cheat_at(c.g[A0][0], rider_f::VARIANT), c.g[V0][0]);
        c.g[V1][0] = c.lw(cheat_at(this, cheat::RIDER));
        c.swc1(1, cheat_at(c.g[V1][0], rider_f::CHEAT_SCALE));
        c.g[V0][0] = c.lw(cheat_at(this, cheat::RIDER));
        c.swc1(0, cheat_at(c.g[V0][0], rider_f::RATE_C));
        c.g[V1][0] = c.lw(cheat_at(this, cheat::RIDER));
        c.swc1(0, cheat_at(c.g[V1][0], rider_f::RATE_B));
        c.g[V0][0] = c.lw(cheat_at(this, cheat::RIDER));
        c.swc1(0, cheat_at(c.g[V0][0], rider_f::RATE_A));
        c.g[V1][0] = c.lw(cheat_at(this, cheat::RIDER));
        c.sw(cheat_at(c.g[V1][0], rider_f::NORMAL_PHYSICS), 0);
    }

    // The assist runs only in cheat mode with a positive assist value.
    c.cycles += 5;
    c.g[A0][0] = c.lw(cheat_at(this, cheat::RIDER));
    c.g[V1][0] = 0xb;
    c.g[V0][0] = c.lw(cheat_at(c.g[A0][0], rider_f::MODE));
    if c.g[V0][0] != c.g[V1][0] {
        return Ok(());
    }
    c.cycles += 6;
    let rider = c.g[A0][0];
    c.lwc1(0, cheat_at(rider, rider_f::ASSIST));
    c.mtc1(0, 1);
    c.c_lt_s(1, 0);
    if !c.fcc() {
        return Ok(());
    }

    // speed = |vel|; below the maximum (and moving): vel = norm(vel) *
    // (max * blend + speed * (1 - blend)), speed = |vel|.
    c.cycles += 14;
    c.lwc1(0, cheat_at(rider, rider_f::MAX_SPEED));
    c.lqc2(3, cheat_at(rider, rider_f::VELOCITY));
    cheat_vu0_length(c);
    c.cfc2(V0, 22);
    c.mtc1(V0, 3);
    c.c_lt_s(3, 0);
    if c.fcc() {
        c.cycles += 3;
        c.c_eq_s(3, 1);
        if !c.fcc() {
            c.cycles += 39;
            c.lqc2(3, cheat_at(rider, rider_f::VELOCITY));
            cheat_vu0_normalise(c);
            c.sqc2(5, frame.at(c, 0x10));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 1);
            c.lq(V0, frame.at(c, 0x10));
            c.sq(V0, frame.at(c, 0x0));
            c.sq(V0, cheat_at(rider, rider_f::VELOCITY));
            c.g[V1][0] = c.lw(cheat_at(this, cheat::RIDER));
            let r = c.g[V1][0];
            c.lwc1(2, cheat_at(r, rider_f::BLEND));
            c.lwc1(0, cheat_at(r, rider_f::MAX_SPEED));
            c.sub_s(1, 1, 2);
            c.mul_s(0, 0, 2);
            c.mul_s(1, 3, 1);
            c.add_s(0, 0, 1);
            c.mfc1(A0, 0);
            c.lqc2(4, cheat_at(r, rider_f::VELOCITY));
            c.qmtc2(A0, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, cheat_at(r, rider_f::VELOCITY));
            c.g[V0][0] = c.lw(cheat_at(this, cheat::RIDER));
            c.lqc2(3, cheat_at(c.g[V0][0], rider_f::VELOCITY));
            cheat_vu0_length(c);
            c.cfc2(V1, 22);
            c.mtc1(V1, 3);
            c.g[A0][0] = c.lw(cheat_at(this, cheat::RIDER));
        }
    }

    // Steering: only with a non-zero steer, a steer direction longer than
    // 0.001 and a velocity longer than 0.0001.
    c.cycles += 6;
    c.mtc1(0, 2);
    let rider = c.g[A0][0];
    c.lwc1(0, cheat_at(rider, rider_f::STEER));
    c.c_eq_s(0, 2);
    if c.fcc() {
        return Ok(());
    }
    c.cycles += 15;
    let gp = c.g[GP][0] as u32;
    c.lwc1(0, gp.wrapping_sub(callee::MIN_DIR_LEN_GP));
    c.lqc2(3, cheat_at(rider, rider_f::STEER_DIR));
    cheat_vu0_length(c);
    c.cfc2(V0, 22);
    c.mtc1(V0, 1);
    c.c_lt_s(0, 1);
    if !c.fcc() {
        return Ok(());
    }
    c.cycles += 15;
    c.lwc1(0, gp.wrapping_sub(callee::MIN_VEL_LEN_GP));
    c.lqc2(3, cheat_at(rider, rider_f::VELOCITY));
    cheat_vu0_length(c);
    c.cfc2(V0, 22);
    c.mtc1(V0, 1);
    c.c_lt_s(0, 1);
    if !c.fcc() {
        return Ok(());
    }

    // vel = norm(dir) * (speed * steer) + norm(vel) * (speed * (1 - steer)).
    c.cycles += 61;
    c.lqc2(3, cheat_at(rider, rider_f::STEER_DIR));
    cheat_vu0_normalise(c);
    c.sqc2(5, frame.at(c, 0x30));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 1);
    c.lq(V0, frame.at(c, 0x30));
    c.g[V1][0] = c.lw(cheat_at(this, cheat::RIDER));
    c.sq(V0, frame.at(c, 0x20));
    c.lwc1(0, cheat_at(c.g[V1][0], rider_f::STEER));
    c.mul_s(0, 3, 0);
    c.mfc1(V1, 0);
    c.lqc2(4, frame.at(c, 0x20));
    c.qmtc2(V1, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, frame.at(c, 0x30));
    c.lq(V0, frame.at(c, 0x30));
    c.g[V1][0] = c.lw(cheat_at(this, cheat::RIDER));
    c.sq(V0, frame.at(c, 0x10));
    c.lqc2(3, cheat_at(c.g[V1][0], rider_f::VELOCITY));
    cheat_vu0_normalise(c);
    c.sqc2(5, frame.at(c, 0x50));
    c.lq(V0, frame.at(c, 0x50));
    c.g[V1][0] = c.lw(cheat_at(this, cheat::RIDER));
    c.sq(V0, frame.at(c, 0x40));
    c.lwc1(0, cheat_at(c.g[V1][0], rider_f::STEER));
    c.sub_s(1, 1, 0);
    c.mul_s(1, 3, 1);
    c.mfc1(V1, 1);
    c.lqc2(4, frame.at(c, 0x40));
    c.qmtc2(V1, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, frame.at(c, 0x50));
    c.lq(V0, frame.at(c, 0x50));
    c.sq(V0, frame.at(c, 0x30));
    c.lqc2(3, frame.at(c, 0x10));
    c.lqc2(4, frame.at(c, 0x30));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, frame.at(c, 0x50));
    c.lq(V0, frame.at(c, 0x50));
    c.sq(V0, frame.at(c, 0x0));
    c.sq(V0, cheat_at(rider, rider_f::VELOCITY));

    // The original's `x < 0 && x >= 0` guard (never both for a compare
    // without NaN): reset the velocity to the zero vector.
    c.g[A0][0] = c.lw(cheat_at(this, cheat::RIDER));
    let rider = c.g[A0][0];
    c.lwc1(0, cheat_at(rider, rider_f::VELOCITY));
    c.c_lt_s(0, 2);
    if !c.fcc() {
        return Ok(());
    }
    c.cycles += 4;
    c.c_le_s(2, 0);
    c.g[V0][0] = 0x50_0000;
    if !c.fcc() {
        return Ok(());
    }
    c.cycles += 4;
    c.g[V0][0] = sx(callee::ZERO_VEC);
    c.lq(A3, callee::ZERO_VEC);
    c.sq(A3, cheat_at(rider, rider_f::VELOCITY));
    Ok(())
}

// ------------------------------------------------------------- __unpack_d

/// libgcc's soft-float `fp_number_type` (the unpacked double; `a1`).
mod fp_number {
    /// Class: 0 signalling NaN, 1 quiet NaN, 2 zero, 3 normal, 4 infinity.
    pub const CLASS: u32 = 0x00;
    /// Sign bit (0 or 1).
    pub const SIGN: u32 = 0x04;
    /// Unbiased exponent.
    pub const NORMAL_EXP: u32 = 0x08;
    /// Fraction (u64); for normals the mantissa shifted left by 8 with the
    /// implicit one at bit 60.
    pub const FRACTION: u32 = 0x10;

    pub const CLASS_SNAN: u64 = 0;
    pub const CLASS_QNAN: u64 = 1;
    pub const CLASS_ZERO: u64 = 2;
    pub const CLASS_NUMBER: u64 = 3;
    pub const CLASS_INFINITY: u64 = 4;
}

/// `void __unpack_d(const FLO_union_type* src, fp_number_type* dst)`
/// (0x411c38, libgcc soft-float): splits the double at `a0` into sign,
/// exponent and fraction. This build has no denormals: a zero exponent field
/// is always class "zero" (fraction and exponent left unwritten). Exponent
/// 0x7ff is infinity (zero fraction) or a NaN (quiet when fraction bit 51 is
/// set); NaN and infinity leave the exponent field unwritten, infinity also
/// the fraction. Normals get the unbiased exponent and the mantissa
/// `<< 8` with the implicit one at bit 60.
pub fn unpack_d(c: &mut Cpu) {
    use crate::guest::Addr;
    use fp_number as fp;

    c.cycles += 10;
    let bits = Addr(c.g[A0][0] as u32).ld(c);
    let dst = Addr(c.g[A1][0] as u32);
    let fraction = bits & (u64::MAX >> 12);
    let exp = (bits >> 52) & 0x7ff;
    dst.off(fp::SIGN).sw(c, bits >> 63);
    c.g[A0][0] = exp;
    c.g[V1][0] = fraction;
    if exp == 0 {
        c.cycles += 3;
        c.g[V0][0] = fp::CLASS_ZERO;
        dst.off(fp::CLASS).sw(c, fp::CLASS_ZERO);
    } else if exp != 0x7ff {
        c.cycles += 11;
        let mantissa = (fraction << 8) | (1u64 << 60);
        let unbiased = sx((exp as u32).wrapping_sub(0x3ff));
        c.g[V1][0] = mantissa;
        c.g[A0][0] = unbiased;
        c.g[V0][0] = fp::CLASS_NUMBER;
        dst.off(fp::FRACTION).sd(c, mantissa);
        dst.off(fp::NORMAL_EXP).sw(c, unbiased);
        dst.off(fp::CLASS).sw(c, fp::CLASS_NUMBER);
    } else if fraction == 0 {
        c.cycles += 6;
        c.g[V0][0] = fp::CLASS_INFINITY;
        dst.off(fp::CLASS).sw(c, fp::CLASS_INFINITY);
    } else {
        c.cycles += 9;
        // v0 holds 1 on both paths (the quiet class, also the store value).
        c.g[V0][0] = 1;
        if fraction & (1u64 << 51) != 0 {
            c.cycles += 2;
            dst.off(fp::CLASS).sw(c, fp::CLASS_QNAN);
        } else {
            c.cycles += 1;
            dst.off(fp::CLASS).sw(c, fp::CLASS_SNAN);
        }
        c.cycles += 2;
        dst.off(fp::FRACTION).sd(c, fraction);
    }
    ret(c);
}

// ----------------------------------------------------- SND_UtkDecodeFrame

/// `SND_Utk` decoder state (`a0`; the bit reader `{ptr, buf, count}` is at
/// its start).
mod utk_frame {
    /// Non-zero: reduced bandwidth (only every other excitation sample is
    /// coded; the others are zeroed or interpolated).
    pub const REDUCED_BW: u32 = 0x0c;
    /// First RC index that makes the frame unvoiced (excitation decoded with
    /// the pulse model instead of the Huffman model).
    pub const VOICED_THRESHOLD: u32 = 0x10;
    /// `float[64]`: fixed-codebook gains.
    pub const FIXED_GAINS: u32 = 0x14;
    /// `float[12]`: reflection coefficients, stepped over the frame.
    pub const RC: u32 = 0x114;
    /// `float[324]`: adaptive codebook (the last 324 decoded samples).
    pub const ADAPT_CB: u32 = 0x174;
    /// `float[432]`: the frame's excitation (4 subframes of 108 samples).
    pub const DECODED: u32 = 0x684;
    /// `DECODED[108..]`: the 324 samples kept as the next adaptive codebook
    /// (copied 32 bytes at a time up to `KEEP_END`, then 16 more).
    pub const KEEP: u32 = 0x834;
    pub const KEEP_END: u32 = 0xd34;

    /// `D_0044E8E8`: the reflection-coefficient quantiser table (indices
    /// 0..63 for RC 0..3, 16..47 for RC 4..11).
    pub const RC_TABLE: u32 = 0x0044_e8e8;

    /// Frame (0x2e0) slots: the excitation scratch, the 12 RC deltas and
    /// the spilled pointers.
    pub const SIZE: u32 = 0x2e0;
    pub const EXC: u32 = 0x10;
    pub const DELTA: u32 = 0x1e0;
    pub const KEEP_PTR: u32 = 0x210;
    pub const RC_PTR: u32 = 0x214;
    pub const GAINS_PTR: u32 = 0x218;
    pub const DECODED_PTR: u32 = 0x21c;
    pub const KEEP_END_PTR: u32 = 0x220;
}

mod utk_frame_callee {
    use crate::call::Callee;
    use ssx3_ee as ee;

    pub const READ_BITS: Callee = Callee {
        addr: 0x003c_d1f8,
        f: ee::s14::SND_UtkReadBits,
    };
    pub const DECODE_EXCITATION: Callee = Callee {
        addr: 0x003c_d2b0,
        f: ee::s14::SND_UtkDecodeExcitation,
    };
    pub const INTERPOLATE: Callee = Callee {
        addr: 0x003c_d518,
        f: ee::s14::SND_UtkInterpolate,
    };
    pub const SYNTHESIZE: Callee = Callee {
        addr: 0x003c_d690,
        f: ee::s14::SND_UtkSynthesize,
    };
}

/// `addiu`/`addu` on a register value: 32-bit add, sign-extended.
#[inline(always)]
fn utk_add(v: u64, k: u32) -> u64 {
    sx((v as u32).wrapping_add(k))
}

/// `void SND_UtkDecodeFrame(SND_Utk* u)` (0x3cd878): decode one 432-sample
/// EA MicroTalk speech frame.
///
/// ```text
/// delta[j] = (RC_TABLE[bits(6 or 5)] - rc[j]) / 4         j = 0..11
/// voiced   = first index < voiced_threshold
/// 4 subframes i: lag = (i+2)*108 - bits(8); pitch = bits(4) / 15;
///   fixed = gains[bits(6)]; excitation (full, or every other sample with
///   the rest zeroed or interpolated and fixed halved);
///   decoded[i*108 + j] = pitch * adapt_cb[lag + j] + fixed * exc[j]
/// adapt_cb = decoded[108..432]
/// 4 times: rc += delta; synthesize 12 samples (the last time 33 blocks)
/// ```
///
/// Every register, every callee-saved value the callees spill and every
/// back-edge event check follows the generated body.
pub fn snd_utk_decode_frame(c: &mut Cpu) {
    guest(c, utk_decode_frame_body)
}

fn utk_decode_frame_body(c: &mut Cpu) -> crate::call::Flow {
    use crate::ps2f::FpuF32;
    use utk_frame as f;
    use utk_frame_callee as callee;

    let fmul = |a: u32, b: u32| (FpuF32(a) * FpuF32(b)).0;
    let fadd = |a: u32, b: u32| (FpuF32(a) + FpuF32(b)).0;
    let fsub = |a: u32, b: u32| (FpuF32(a) - FpuF32(b)).0;
    const QUARTER: u32 = 0x3e80_0000;

    // --- Prologue, first RC index.
    c.cycles += 38;
    let frame = Frame::open(c, f::SIZE);
    let u = c.g[A0][0];
    frame.save(c, S3, 0x290);
    frame.save(c, S0, 0x2c0);
    frame.save(c, S1, 0x2b0);
    frame.save(c, S2, 0x2a0);
    frame.save(c, S4, 0x280);
    frame.save(c, S6, 0x260);
    frame.save(c, FP, 0x240);
    frame.save_fpr(c, 20, 0x2d0);
    frame.save(c, S5, 0x270);
    frame.save(c, S7, 0x250);
    frame.save_ra(c, 0x230);
    frame.save_fpr(c, 21, 0x2d8);
    c.g[A1][0] = 6;
    c.g[S3][0] = u;
    let rc = utk_add(u, f::RC);
    c.g[V0][0] = rc;
    c.g[S2][0] = utk_add(u, f::RC + 4);
    c.g[S0][0] = 2;
    c.sw(frame.at(c, f::RC_PTR), rc);
    callee::READ_BITS.call(c, 0x003c_d8c8)?;

    // delta[0]; the first index also decides `voiced`.
    let first = c.g[V0][0];
    let table = sx(f::RC_TABLE);
    c.g[A0][0] = sx(((first as u32) << 2).wrapping_add(f::RC_TABLE));
    c.lwc1(2, utk_add(u, f::RC) as u32);
    c.g[AT][0] = QUARTER as u64;
    c.f[1] = QUARTER;
    c.lwc1(0, c.g[A0][0] as u32);
    c.g[S4][0] = table;
    let threshold = c.lw(utk_add(u, f::VOICED_THRESHOLD) as u32);
    c.g[V1][0] = threshold;
    c.g[FP][0] = sx(frame.at(c, f::DELTA));
    c.f[0] = fsub(c.f[0], c.f[2]);
    c.g[S1][0] = sx(frame.at(c, f::DELTA + 4));
    c.f[20] = c.f[1];
    c.g[S6][0] = ((first as i64) < (threshold as i64)) as u64;
    c.f[0] = fmul(c.f[0], c.f[1]);
    c.swc1(0, frame.at(c, f::DELTA));

    // delta[1..3]: 6-bit indices (s0 counts 2..0, s2 walks rc, s1 delta).
    loop {
        c.cycles += 14;
        c.g[A0][0] = u;
        c.g[A1][0] = 6;
        callee::READ_BITS.call(c, 0x003c_d91c)?;
        c.g[S0][0] = utk_add(c.g[S0][0], 0xffff_ffff);
        let entry = sx(((c.g[V0][0] as u32) << 2).wrapping_add(c.g[S4][0] as u32));
        c.g[V0][0] = entry;
        let rc_j = c.g[S2][0];
        c.lwc1(1, rc_j as u32);
        c.g[S2][0] = utk_add(rc_j, 4);
        c.lwc1(0, entry as u32);
        c.f[0] = fmul(fsub(c.f[0], c.f[1]), c.f[20]);
        let delta_j = c.g[S1][0];
        c.swc1(0, delta_j as u32);
        c.g[S1][0] = utk_add(delta_j, 4);
        if (c.g[S0][0] as i64) < 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_d910);
        }
    }

    // delta[4..11]: 5-bit indices into the table's second half.
    c.cycles += 18;
    c.g[V0][0] = 0x45_0000;
    c.g[A0][0] = utk_add(u, f::FIXED_GAINS);
    c.g[S4][0] = table;
    c.g[A1][0] = utk_add(u, f::DECODED);
    c.g[A2][0] = utk_add(u, f::KEEP);
    c.g[A3][0] = utk_add(u, f::KEEP_END);
    c.g[V0][0] = c.lw(frame.at(c, f::RC_PTR));
    c.g[S7][0] = utk_add(u, f::ADAPT_CB);
    c.g[AT][0] = QUARTER as u64;
    c.f[20] = QUARTER;
    c.g[S2][0] = utk_add(c.g[FP][0], 0x10);
    c.sw(frame.at(c, f::GAINS_PTR), c.g[A0][0]);
    c.g[S1][0] = utk_add(c.g[V0][0], 0x10);
    c.sw(frame.at(c, f::DECODED_PTR), c.g[A1][0]);
    c.g[S0][0] = 7;
    c.sw(frame.at(c, f::KEEP_PTR), c.g[A2][0]);
    c.sw(frame.at(c, f::KEEP_END_PTR), c.g[A3][0]);
    loop {
        c.cycles += 15;
        c.g[A0][0] = u;
        c.g[A1][0] = 5;
        callee::READ_BITS.call(c, 0x003c_d99c)?;
        c.g[S0][0] = utk_add(c.g[S0][0], 0xffff_ffff);
        let rc_j = c.g[S1][0];
        c.lwc1(1, rc_j as u32);
        c.g[S1][0] = utk_add(rc_j, 4);
        let entry = sx(
            ((c.g[V0][0] as u32).wrapping_add(0x10) << 2).wrapping_add(c.g[S4][0] as u32),
        );
        c.g[V0][0] = entry;
        c.lwc1(0, entry as u32);
        c.f[0] = fmul(fsub(c.f[0], c.f[1]), c.f[20]);
        let delta_j = c.g[S2][0];
        c.swc1(0, delta_j as u32);
        c.g[S2][0] = utk_add(delta_j, 4);
        if (c.g[S0][0] as i64) < 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_d990);
        }
    }

    // --- Four subframes (s4 = i, s1 = i + 1 once decoded).
    c.cycles += 3;
    c.g[S4][0] = 0;
    c.g[A1][0] = 8;
    loop {
        // Pitch lag (s5 = (i + 2) * 108 - bits(8)) and pitch gain bits(4) / 15
        // (converted as unsigned: a value with the top bit set is halved,
        // converted and doubled).
        c.cycles += 11;
        c.g[A0][0] = u;
        callee::READ_BITS.call(c, 0x003c_d9e0)?;
        c.g[V1][0] = utk_add(c.g[S4][0], 2);
        c.g[A1][0] = 0x6c;
        c.mult(0, V1, V1, A1);
        c.g[A0][0] = u;
        c.g[A1][0] = 4;
        c.g[S5][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        callee::READ_BITS.call(c, 0x003c_d9fc)?;
        let gain_bits = c.g[V0][0];
        c.g[V1][0] = sx((gain_bits as u32) >> 1);
        if (gain_bits as i64) < 0 {
            c.cycles += 5;
            let halved = (gain_bits & 1) | c.g[V1][0];
            c.g[V0][0] = halved;
            let v = FpuF32::from_i32(halved as u32 as i32);
            c.f[1] = (v + v).0;
        } else {
            c.cycles += 4;
            c.f[1] = FpuF32::from_i32(gain_bits as u32 as i32).0;
        }

        // Fixed gain gains[bits(6)] in f20, pitch gain (* 1/15) in f21.
        c.cycles += 13;
        c.g[AT][0] = 0x3d88_8889;
        c.f[0] = 0x3d88_8889;
        c.g[A0][0] = u;
        c.g[A1][0] = 6;
        c.f[21] = fmul(c.f[1], c.f[0]);
        callee::READ_BITS.call(c, 0x003c_da44)?;
        let gains = c.lw(frame.at(c, f::GAINS_PTR));
        c.g[A2][0] = gains;
        let gain_at = sx(((c.g[V0][0] as u32) << 2).wrapping_add(gains as u32));
        c.g[V0][0] = gain_at;
        let reduced = c.lw(utk_add(u, f::REDUCED_BW) as u32);
        c.g[V1][0] = reduced;
        c.lwc1(20, gain_at as u32);

        if reduced == 0 {
            // Full excitation at EXC + 4.
            c.cycles += 7;
            c.g[A0][0] = u;
            c.g[A1][0] = c.g[S6][0];
            c.g[A2][0] = sx(frame.at(c, f::EXC + 4));
            c.g[A3][0] = 1;
            callee::DECODE_EXCITATION.call(c, 0x003c_da70)?;
            c.g[S1][0] = utk_add(c.g[S4][0], 1);
            c.cycles += 1;
        } else {
            // Every other sample (phase `align`), then zero or interpolate
            // the others.
            c.cycles += 17;
            c.g[A0][0] = u;
            c.g[A1][0] = 1;
            callee::READ_BITS.call(c, 0x003c_da84)?;
            let align = c.g[V0][0];
            c.g[S2][0] = align;
            c.g[A0][0] = u;
            c.g[A1][0] = 1;
            c.g[S1][0] = sx((align as u32) << 2);
            callee::READ_BITS.call(c, 0x003c_da98)?;
            let out = utk_add(c.g[S1][0], 0x14);
            c.g[S0][0] = c.g[V0][0];
            c.g[A2][0] = sx(sp(c).wrapping_add(out as u32));
            c.g[A0][0] = u;
            c.g[A1][0] = c.g[S6][0];
            c.g[A3][0] = 2;
            callee::DECODE_EXCITATION.call(c, 0x003c_dab4)?;
            c.g[V1][0] = utk_add(c.g[S2][0], 0xffff_fffa);
            if c.g[S0][0] != 0 {
                // Zero the uncoded samples (54, walking down).
                c.cycles += 5;
                c.g[S1][0] = utk_add(c.g[S4][0], 1);
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[S0][0] = 0x6a;
                c.g[V0][0] = utk_add(sx(sp(c).wrapping_sub(c.g[V1][0] as u32)), 0x1a8);
                loop {
                    c.cycles += 7;
                    c.sw(c.g[V0][0] as u32, 0);
                    c.g[S0][0] = utk_add(c.g[S0][0], 0xffff_fffe);
                    c.g[V0][0] = utk_add(c.g[V0][0], 0xffff_fff8);
                    if (c.g[S0][0] as i64) < 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_dad0);
                    }
                }
                c.cycles += 2;
            } else {
                // Clear 5 samples on each side, interpolate, halve the gain.
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[S0][0] = 4;
                c.g[S1][0] = utk_add(c.g[S4][0], 1);
                c.g[V0][0] = sx(frame.at(c, f::EXC));
                loop {
                    c.cycles += 7;
                    c.sw(c.g[V0][0] as u32, 0);
                    c.g[S0][0] = utk_add(c.g[S0][0], 0xffff_ffff);
                    c.g[V0][0] = utk_add(c.g[V0][0], 0xffff_fffc);
                    if (c.g[S0][0] as i64) < 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_db08);
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = sx(frame.at(c, 0x1c4));
                c.g[S0][0] = 4;
                loop {
                    c.cycles += 7;
                    c.sw(c.g[V0][0] as u32, 0);
                    c.g[S0][0] = utk_add(c.g[S0][0], 0xffff_ffff);
                    c.g[V0][0] = utk_add(c.g[V0][0], 4);
                    if (c.g[S0][0] as i64) < 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_db30);
                    }
                }
                c.cycles += 6;
                let back = utk_add(c.g[A0][0], 0xffff_ffe8);
                c.g[A0][0] = sx(sp(c).wrapping_sub(back as u32));
                callee::INTERPOLATE.call(c, 0x003c_db58)?;
                c.g[AT][0] = 0x3f00_0000;
                c.f[0] = 0x3f00_0000;
                c.f[20] = fmul(c.f[20], c.f[0]);
                c.cycles += 1;
            }
        }
        c.g[A3][0] = 0x6c;

        // decoded[i*108 + j] = pitch * adapt_cb[lag + j] + fixed * exc[j].
        c.cycles += 8;
        c.g[V0][0] = sx((c.g[S5][0] as u32) << 2);
        c.mult(0, V1, S4, A3);
        let decoded = c.lw(frame.at(c, f::DECODED_PTR));
        c.g[A2][0] = decoded;
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S7][0] as u32));
        c.g[A0][0] = sx(frame.at(c, f::EXC + 4));
        c.g[S0][0] = 0x6b;
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(decoded as u32));
        loop {
            c.cycles += 11;
            let past = c.g[A1][0];
            let exc = c.g[A0][0];
            c.lwc1(0, past as u32);
            c.g[S0][0] = utk_add(c.g[S0][0], 0xffff_ffff);
            c.lwc1(1, exc as u32);
            c.g[A1][0] = utk_add(past, 4);
            c.f[0] = fmul(c.f[21], c.f[0]);
            c.g[A0][0] = utk_add(exc, 4);
            c.f[1] = fmul(c.f[20], c.f[1]);
            c.f[0] = fadd(c.f[0], c.f[1]);
            let out = c.g[V0][0];
            c.swc1(0, out as u32);
            c.g[V0][0] = utk_add(out, 4);
            if (c.g[S0][0] as i64) < 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_db88);
            }
        }
        c.cycles += 4;
        c.g[S4][0] = c.g[S1][0];
        let more = (c.g[S4][0] as i64) < 4;
        c.g[V0][0] = more as u64;
        c.g[A1][0] = 8;
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_d9d8);
        }
    }

    // --- adapt_cb = decoded[108..432] (0x510 bytes; doubleword copy when
    // both are 8-aligned, else ldl/ldr + sdl/sdr).
    c.cycles += 6;
    c.g[V1][0] = c.lw(frame.at(c, f::KEEP_PTR));
    c.g[A0][0] = c.g[S7][0];
    c.g[V0][0] = (c.g[V1][0] | c.g[A0][0]) & 7;
    if c.g[V0][0] == 0 {
        loop {
            c.cycles += 12;
            let (src, dst) = (c.g[V1][0] as u32, c.g[A0][0] as u32);
            c.g[V0][0] = c.ld(src);
            c.g[A1][0] = c.ld(src.wrapping_add(8));
            c.g[A2][0] = c.ld(src.wrapping_add(0x10));
            c.g[A3][0] = c.ld(src.wrapping_add(0x18));
            c.sd(dst, c.g[V0][0]);
            c.sd(dst.wrapping_add(8), c.g[A1][0]);
            c.sd(dst.wrapping_add(0x10), c.g[A2][0]);
            c.sd(dst.wrapping_add(0x18), c.g[A3][0]);
            c.g[V0][0] = c.lw(frame.at(c, f::KEEP_END_PTR));
            c.g[V1][0] = utk_add(c.g[V1][0], 0x20);
            c.g[A0][0] = utk_add(c.g[A0][0], 0x20);
            if c.g[V1][0] == c.g[V0][0] {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_dc34);
            }
        }
    } else {
        loop {
            c.cycles += 20;
            let (src, dst) = (c.g[V1][0] as u32, c.g[A0][0] as u32);
            for (reg, off) in [(A3, 0u32), (V0, 8), (A1, 0x10), (A2, 0x18)] {
                c.ldl(reg, src.wrapping_add(off + 7));
                c.ldr(reg, src.wrapping_add(off));
            }
            for (reg, off) in [(A3, 0u32), (V0, 8), (A1, 0x10), (A2, 0x18)] {
                c.sdl(reg, dst.wrapping_add(off + 7));
                c.sdr(reg, dst.wrapping_add(off));
            }
            c.g[A3][0] = c.lw(frame.at(c, f::KEEP_END_PTR));
            c.g[V1][0] = utk_add(c.g[V1][0], 0x20);
            c.g[A0][0] = utk_add(c.g[A0][0], 0x20);
            if c.g[V1][0] == c.g[A3][0] {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_dbdc);
            }
        }
        c.cycles += 2;
    }
    c.cycles += 11;
    let (src, dst) = (c.g[V1][0] as u32, c.g[A0][0] as u32);
    for (reg, off) in [(A1, 0u32), (A2, 8)] {
        c.ldl(reg, src.wrapping_add(off + 7));
        c.ldr(reg, src.wrapping_add(off));
    }
    for (reg, off) in [(A1, 0u32), (A2, 8)] {
        c.sdl(reg, dst.wrapping_add(off + 7));
        c.sdr(reg, dst.wrapping_add(off));
    }

    // --- Four steps: rc += delta, then synthesize from sample `offset`.
    c.g[S0][0] = 0xb;
    c.g[V0][0] = c.lw(frame.at(c, f::RC_PTR));
    c.g[V1][0] = c.g[FP][0];
    const STEPS: [(u32, u64, u64, u32); 4] = [
        (0x003c_dc90, 0x00, 1, 0x003c_dcc4),
        (0x003c_dcd0, 0x0c, 1, 0x003c_dd04),
        (0x003c_dd10, 0x18, 1, 0x003c_dd44),
        (0x003c_dd50, 0x24, 0x21, 0x003c_dd80),
    ];
    for (k, &(head, offset, blocks, ret_to)) in STEPS.iter().enumerate() {
        loop {
            c.cycles += 8;
            let (rc_j, delta_j) = (c.g[V0][0], c.g[V1][0]);
            c.lwc1(0, rc_j as u32);
            c.g[S0][0] = utk_add(c.g[S0][0], 0xffff_ffff);
            c.lwc1(1, delta_j as u32);
            c.g[V1][0] = utk_add(delta_j, 4);
            c.f[0] = fadd(c.f[0], c.f[1]);
            c.swc1(0, rc_j as u32);
            c.g[V0][0] = utk_add(rc_j, 4);
            if (c.g[S0][0] as i64) < 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, head);
            }
        }
        let last = k == STEPS.len() - 1;
        c.cycles += if last { 18 } else { 8 };
        c.g[A0][0] = u;
        c.g[A1][0] = offset;
        c.g[A2][0] = blocks;
        if !last {
            c.g[S0][0] = 0xb;
        }
        callee::SYNTHESIZE.call(c, ret_to)?;
        if !last {
            c.g[V0][0] = c.lw(frame.at(c, f::RC_PTR));
            c.g[V1][0] = c.g[FP][0];
        }
    }

    // --- Epilogue.
    frame.restore(c, S0, 0x2c0);
    frame.restore(c, S1, 0x2b0);
    frame.restore(c, S2, 0x2a0);
    frame.restore(c, S3, 0x290);
    frame.restore(c, S4, 0x280);
    frame.restore(c, S5, 0x270);
    frame.restore(c, S6, 0x260);
    frame.restore(c, S7, 0x250);
    frame.restore(c, FP, 0x240);
    frame.restore_ra(c, 0x230);
    frame.restore_fpr(c, 21, 0x2d8);
    frame.restore_fpr(c, 20, 0x2d0);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------- cDynamicSprayFX_Update

    /// `cDynamicSprayFX_Update` against the generated body. Real callees run
    /// on short paths: the emitter setup calls a real `return 0` method per
    /// emitter, the bone lookup finds no bones, the rider-state and anim
    /// getters are tiny leaves, the burst and enable updates are leaves, and
    /// emitter 5 (switched on by the fixture or by the enable update) takes
    /// its spawn path on two prepared emitter slots. The two virtual calls on
    /// the physics sub-object are fakes (speed vector, surface index) and may
    /// unwind. The other seven emitter updates stay off (their flags are 0
    /// and the enable update, reached only with enable states 2/3, leaves
    /// them off).
    #[test]
    fn c_dynamic_spray_fx_update_matches() {
        use crate::difftest::fakes::{self, FAKE, LOG_RESULTS};
        use crate::difftest::{put_vec, DATA};
        use crate::game::visualfx::boardwakefx::tests::{f, gp_at, put_spray_slot, RETURN_ZERO};

        const FX: u32 = DATA;
        const OBJ: u32 = DATA + 0x1000;
        const EM: u32 = DATA + 0x2000;
        const VT_EMITTER: u32 = DATA + 0x3800;
        const CURVES: u32 = DATA + 0x3900;
        const VT_PHYSICS: u32 = DATA + 0x3a00;
        const MODEL: u32 = DATA + 0x3b00;
        const BONE_TABLE: u32 = DATA + 0x3b80;
        const BONE_GROUPS: u32 = DATA + 0x3c00;
        const MATRICES: u32 = DATA + 0x4000;
        const ANIM: u32 = DATA + 0x4200;
        const STATES: u32 = DATA + 0x4300;
        const BONES: u32 = DATA + 0x5200;
        const SPEED_VEC: u32 = DATA + 0x5300;
        const WORLD: u32 = DATA + 0x5400;
        const GAME: u32 = DATA + 0x5500;
        const RACE: u32 = DATA + 0x5600;
        const PLAYERS: u32 = DATA + 0x5900;
        /// The surface colour table (0xf0 bytes per surface).
        const SURFACES: u32 = 0x004f_a398;

        fakes::covered(
            ee::s09::f_002df920__generated,
            c_dynamic_spray_fx_update,
            250,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x2df9_20, 2);
                let bit = |r: &mut crate::difftest::Rng| r.next_u32() & 1;

                // .lit4 constants: 60, the fast slow-down rate and speed
                // (555.6), the rail burst rate (27.8); emitter 5's.
                for (off, v) in [
                    (-0x3b88, 59.999996f32),
                    (-0x3b84, 555.55554),
                    (-0x3b80, 27.777779),
                    (-0x3abc, 27.777778),
                    (-0x3ab8, 277.77777),
                    (-0x3ab4, 3333.3335),
                    (-0x3ab0, 0.3),
                    (-0x3aac, 0.4),
                    (-0x3aa8, 0.8),
                    (-0x3aa4, 0.2),
                    (-0x3aa0, 1.0 / 60.0),
                    (-0x3a9c, 83.333336),
                    (-0x3a98, 0.4),
                    (-0x3a94, 0.5),
                ] {
                    c.bus.write32(gp_at(off), f(v));
                }
                c.bus.write32(gp_at(0xa0c), r.next_u32());
                c.bus.write32(gp_at(-0x848), WORLD);
                c.bus.write32(WORLD + 0x84, GAME);
                c.bus.write32(GAME + 0x44, RACE);
                c.bus.write32(GAME + 0x84, PLAYERS);
                c.bus.write32(PLAYERS + 0x10, 0);

                // The spray.
                for k in 0..0x4c {
                    c.bus.write32(FX + 4 * k, r.edge_f32());
                }
                c.bus.write32(FX + spray_fx::OBJ, OBJ);
                c.bus.write32(FX + spray_fx::EMITTERS, EM);
                c.bus.write32(FX + 0x2c, CURVES);
                c.bus.write32(FX + spray_fx::NEEDS_SETUP, (seed % 5 == 0) as u32);
                c.bus.write32(FX + 0x7c, BONES);
                c.bus.write32(FX + spray_fx::BONES_FOUND, (seed % 6 != 0) as u32);
                c.bus.write32(FX + 0x124, bit(&mut r));

                // The speed vector the physics sub-object returns, and the
                // previous speed (slow-down rate = (prev - speed) * 60).
                let (vec, prev) = match seed % 4 {
                    0 => ([0; 4], r.f32(1500.0)),
                    1 => {
                        let x = 600.0 + (r.next_u32() % 400) as f32;
                        let dv = [20.0f32, 5.0, 0.3, -5.0][((seed / 4) % 4) as usize];
                        ([f(x), 0, 0, r.edge_f32()], f(x + dv))
                    }
                    2 => (
                        [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                        r.edge_f32(),
                    ),
                    _ => (
                        [r.f32(300.0), r.f32(300.0), r.f32(300.0), r.f32(1.0)],
                        f(f32::from_bits(r.f32(800.0)).abs()),
                    ),
                };
                put_vec(c, SPEED_VEC, vec);
                c.bus.write32(FX + spray_fx::PREV_SPEED, prev);
                c.bus.write32(LOG_RESULTS, SPEED_VEC);
                let surface = 1 + seed % 7;
                c.bus.write32(LOG_RESULTS + 4, surface);
                for k in 0..4 {
                    c.bus.write32(SURFACES + surface * 0xf0 + 4 * k, r.f32(1.0));
                }

                // The rider.
                for k in 0..0x2c4 {
                    c.bus.write32(OBJ + 4 * k, r.edge_f32());
                }
                c.bus.write32(OBJ + spray_rider::IN_TRICK, (seed % 3 != 0) as u32);
                c.bus.write32(OBJ + spray_rider::ANIM, ANIM);
                let anims = [23u32, 25, 28, 37, 38, 22, 29, 36, 39, 0];
                c.bus.write32(ANIM + 8, anims[(r.next_u32() % 10) as usize]);
                c.bus.write32(OBJ + spray_rider::GOOFY, bit(&mut r));
                c.bus.write32(OBJ + spray_rider::SLIP, r.f32(1.5));
                c.bus.write32(OBJ + spray_rider::FORWARD_SPEED, r.f32(500.0));
                for k in 0..4 {
                    c.bus.write32(OBJ + spray_rider::VELOCITY + 4 * k, r.edge_f32());
                    c.bus.write32(OBJ + spray_rider::BOARD_DIR + 4 * k, r.f32(1.0));
                }
                if seed % 9 == 0 {
                    put_vec(c, OBJ + spray_rider::VELOCITY, [0; 4]);
                }
                c.bus.write32(OBJ + spray_rider::INDEX, seed % 4);
                c.bus.write32(OBJ + spray_rider::PHYSICS, VT_PHYSICS);
                c.bus.write16(VT_PHYSICS + 0x10, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(VT_PHYSICS + 0x14, FAKE);
                c.bus.write16(VT_PHYSICS + 0x38, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(VT_PHYSICS + 0x3c, FAKE + 0x10);
                c.bus.write32(OBJ + 0x77c, STATES);
                c.bus.write32(STATES + 0xde0, seed % 7);
                c.bus.write32(STATES + 0xde4, if seed % 11 == 0 { 9 } else { 3 });
                for off in [
                    spray_rider::GRINDING,
                    spray_rider::GRIND_OFF,
                    spray_rider::GRIND_RAIL,
                    spray_rider::GRIND_SPRAY,
                ] {
                    c.bus.write32(OBJ + off, bit(&mut r));
                }
                // The enable state: unchanged, or changed to 2/3 (which
                // leaves only emitter 5 on).
                let state = 2 + bit(&mut r);
                c.bus.write32(OBJ + spray_rider::ENABLE_STATE, state);
                let known = if seed % 4 == 3 { state ^ 1 } else { state };
                c.bus.write32(FX + spray_fx::ENABLE_STATE, known);

                // The model: matrices for the spray bone; no named bones.
                c.bus.write32(OBJ + spray_rider::MODEL, MODEL);
                c.bus.write32(OBJ + spray_rider::SPRAY_BONE, seed % 4);
                c.bus.write32(MODEL + 0x30, MATRICES);
                c.bus.write32(MODEL + 0x1c, BONE_TABLE);
                c.bus.write32(MODEL + 0xc, BONE_GROUPS);
                c.bus.write32(BONE_TABLE, 0);
                c.bus.write32(BONE_GROUPS + 0x44, 0);
                for k in 0..0x40 {
                    c.bus.write32(MATRICES + 4 * k, r.edge_f32());
                }

                // The ten emitters: a `return 0` setup method each; slots 5
                // and 6 ready for emitter 5's spawns; only emitter 5's flag
                // may be on.
                c.bus.write16(VT_EMITTER + 0x18, 0);
                c.bus.write32(VT_EMITTER + 0x1c, RETURN_ZERO);
                for k in 0..10 {
                    let e = EM + k * 0x210;
                    c.bus.write32(e + 0x1f8, VT_EMITTER);
                    c.bus.write32(e + 0x58, r.f32(2.0));
                }
                put_spray_slot(c, &mut r, EM + 5 * 0x210, DATA + 0x8000);
                put_spray_slot(c, &mut r, EM + 6 * 0x210, DATA + 0x8100);
                for (flag, _, _) in spray_callee::EMITTERS {
                    c.bus.write32(EM + flag, 0);
                }
                c.bus.write32(EM + 0xbc4, (seed % 5 == 1) as u32);

                c.g[A0][0] = sx(FX);
            },
        );
    }

    // ---------------------------------------- RiderBase_UpdateCheatCode

    /// `RiderBase_UpdateCheatCode` against the generated body. The physics
    /// sub-object's query is either a real `return 0` (the update runs) or
    /// a fake (non-zero: nothing to do, or an unwind); the clear, the match
    /// and `BELibrary_GetInterface` run for real (the interface's method is
    /// a fake). The state byte covers on/off/wrong; the rider covers cheat
    /// mode or not, the assist sign, speeds below/above the maximum (and
    /// zero), zero steer, short steer directions and short velocities. The
    /// final `x < 0 && x >= 0` reset is unreachable without NaNs.
    #[test]
    fn rider_base_update_cheat_code_matches() {
        use crate::difftest::fakes::{self, FAKE};
        use crate::difftest::{put_vec, DATA, GAME_GP};
        use crate::game::visualfx::boardwakefx::tests::{f, gp_at, RETURN_ZERO};

        const SELF: u32 = DATA;
        const RIDER: u32 = DATA + 0x1000;
        const PHYS_VT: u32 = DATA + 0x2000;
        const LIB: u32 = DATA + 0x2100;
        const LIB_VT: u32 = DATA + 0x2200;
        const WORLD: u32 = DATA + 0x2300;
        /// `g_beLibrary[]` and its index (gp-0x1efc).
        const BE_LIBRARY: u32 = 0x0053_08a8;

        fakes::covered(
            ee::s09::f_002f6518__generated,
            rider_base_update_cheat_code,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x2f_6518, 2);

                // .lit4 minimum lengths, the clear counter, the world, the
                // global cheats-off switch.
                c.bus.write32(gp_at(-0x3898), f(0.001));
                c.bus.write32(gp_at(-0x3894), f(0.0001));
                let due = [0u32, 1, 2, 3, 8][(seed % 5) as usize];
                c.bus.write32(gp_at(0xad0), due);
                c.bus.write32(gp_at(-0x848), WORLD);
                c.bus.write32(WORLD + 0x78, r.next_u32());
                c.bus.write32(cheat_callee::CHEATS_OFF, (seed % 7 == 0) as u32);

                // BELibrary_GetInterface: library 0, its method a fake.
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(BE_LIBRARY, LIB);
                c.bus.write32(LIB + 4, LIB_VT);
                c.bus.write16(LIB_VT + 8, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(LIB_VT + 0xc, FAKE + 0x10);

                // The cheat state: on, off or wrong; flags without events
                // mostly (the match then leaves the state alone).
                c.bus.write32(SELF + cheat::RIDER, RIDER);
                let state = [1u8, 1, 1, 0, 0xff, 2][(r.next_u32() % 6) as usize];
                c.bus.write8(SELF + cheat::STATE, state);
                c.bus.write8(SELF + 5, (r.next_u32() % 9) as u8);
                c.bus.write8(SELF + 6, (r.next_u32() % 4) as u8);
                let mut flags = r.next_u32() as u8;
                if seed % 3 != 0 {
                    flags &= 0x8f;
                }
                c.bus.write8(SELF + cheat::FLAGS, flags);

                // The rider: the physics query (0 from a real function,
                // or a fake), then the cheat-mode fields.
                for k in 0..0x80 {
                    c.bus.write32(RIDER + 0x1e0 + 4 * k, r.edge_f32());
                }
                c.bus.write32(RIDER + cheat_rider::PHYSICS, PHYS_VT);
                c.bus.write16(PHYS_VT + 0x48, (r.next_u32() & 0x7c) as u16);
                let query = if seed % 6 == 5 { FAKE } else { RETURN_ZERO };
                c.bus.write32(PHYS_VT + 0x4c, query);
                let mode = if seed % 8 == 3 { r.next_u32() % 12 } else { 11 };
                c.bus.write32(RIDER + cheat_rider::MODE, mode);
                let assist = match seed % 9 {
                    0 => f(-1.0),
                    1 => 0,
                    2 => r.edge_f32(),
                    _ => f(1.0),
                };
                c.bus.write32(RIDER + cheat_rider::ASSIST, assist);
                let abs = |x: u32| f(f32::from_bits(x).abs());
                let vel = match (seed / 2) % 6 {
                    0 => [0, 0, 0, r.next_u32()],
                    1 => [f(0.00002), f(-0.00003), f(0.00001), r.next_u32()],
                    2 => [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                    _ => [r.f32(100.0), r.f32(100.0), r.f32(100.0), r.f32(1.0)],
                };
                put_vec(c, RIDER + cheat_rider::VELOCITY, vel);
                let max = match (seed / 3) % 4 {
                    0 => r.edge_f32(),
                    1 => 0,
                    _ => abs(r.f32(250.0)),
                };
                c.bus.write32(RIDER + cheat_rider::MAX_SPEED, max);
                c.bus.write32(RIDER + cheat_rider::BLEND, r.f32(1.0));
                let steer = match seed % 7 {
                    1 => 0,
                    2 => r.edge_f32(),
                    _ => r.f32(1.0),
                };
                c.bus.write32(RIDER + cheat_rider::STEER, steer);
                let dir = match (seed / 4) % 5 {
                    0 => [f(0.0002), 0, f(0.0001), r.next_u32()],
                    1 => [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                    _ => [r.f32(2.0), r.f32(2.0), r.f32(2.0), r.f32(1.0)],
                };
                put_vec(c, RIDER + cheat_rider::STEER_DIR, dir);

                c.g[A0][0] = sx(SELF);
            },
        );
    }

    // ------------------------------------------------------------ __unpack_d

    /// `__unpack_d` against the generated body: random doubles plus each
    /// class (zero exponent, normal, infinity, quiet and signalling NaN) with
    /// both signs; the destination is pre-filled with junk so unwritten
    /// fields show.
    #[test]
    fn unpack_d_matches() {
        use crate::difftest::{check, scramble, Rng, DATA};

        const SRC: u32 = DATA;
        const DST: u32 = DATA + 0x40;
        check(ee::s15::f_00411c38__generated, unpack_d, 300, &|c, seed| {
            let mut r = Rng(0x0411_c38d ^ seed as u64);
            scramble(c, &mut r);
            let sign = (r.next_u32() as u64 & 1) << 63;
            let frac = ((r.next_u32() as u64) << 32 | r.next_u32() as u64) & (u64::MAX >> 12);
            let rnd_exp = (r.next_u32() as u64) & 0x7ff;
            let bits = match seed % 8 {
                0 => sign | frac,                                // zero exponent (with fraction)
                1 => sign,                                       // +-0
                2 => sign | (0x7ff << 52),                       // +-inf
                3 => sign | (0x7ff << 52) | (1 << 51) | frac,    // quiet NaN
                4 => sign | (0x7ff << 52) | ((frac & !(1 << 51)) | 1), // signalling NaN
                5 => sign | (0x3ff << 52) | frac,                // exponent 0
                _ => sign | (rnd_exp << 52) | frac,              // anything
            };
            for i in 0..8 {
                c.bus.write32(DST + i * 4, r.next_u32());
            }
            c.bus.write32(SRC, bits as u32);
            c.bus.write32(SRC + 4, (bits >> 32) as u32);
            c.g[A0][0] = sx(SRC);
            c.g[A1][0] = sx(DST);
        });
    }

    // ----------------------------------------------- SND_UtkDecodeFrame

    /// `SND_UtkDecodeFrame` against the generated body, with the real
    /// callees (bit reader, excitation decoder, interpolation, synthesis)
    /// decoding random speech bits from the game's tables. Seeds alternate
    /// the state's 8-byte alignment (the adaptive-codebook copy's
    /// doubleword loop or its ldl/ldr loop) and full vs reduced bandwidth;
    /// the random bits pick voiced/unvoiced frames and, in reduced frames,
    /// both the zeroing and the interpolation of the uncoded samples. Every
    /// seventh seed uses edge-case floats in the decoder state.
    #[test]
    fn snd_utk_decode_frame_matches() {
        use crate::difftest::{check, elf_image, fill, scramble, Rng, DATA};
        use utk_frame as f;

        /// `float[12]` synthesis history after the reflection coefficients.
        const SYNTH: u32 = 0x144;

        check(ee::s14::f_003cd878__generated, snd_utk_decode_frame, 120, &|c, seed| {
            let mut r = Rng(0x003c_d878 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            elf_image(c);
            c.pc = 0x003c_d878;
            // 4 mod 8 makes both copy pointers 8-aligned.
            let u = DATA + 0x1000 + 4 * (seed % 2);
            let input = DATA + 0x8000;
            fill(c, &mut r, input, 0x800);
            // Bit reader {ptr, buf, count}.
            c.bus.write32(u, input + r.next_u32() % 8);
            c.bus.write32(u + 4, r.next_u32());
            c.bus.write32(u + 8, 8 + r.next_u32() % 25);
            let reduced = if (seed / 2) % 2 == 0 { 0 } else { r.next_u32() | 1 };
            c.bus.write32(u + f::REDUCED_BW, reduced);
            c.bus.write32(u + f::VOICED_THRESHOLD, r.next_u32() % 72);
            let edge = seed % 7 == 0;
            let flt = |r: &mut Rng, range: f32| if edge { r.edge_f32() } else { r.f32(range) };
            for i in 0..64 {
                let v = flt(&mut r, 2.0);
                c.bus.write32(u + f::FIXED_GAINS + 4 * i, v);
            }
            for i in 0..12 {
                let v = flt(&mut r, 0.9);
                c.bus.write32(u + f::RC + 4 * i, v);
                let h = flt(&mut r, 100.0);
                c.bus.write32(u + SYNTH + 4 * i, h);
            }
            for i in 0..324 {
                let v = flt(&mut r, 1000.0);
                c.bus.write32(u + f::ADAPT_CB + 4 * i, v);
            }
            fill(c, &mut r, u + f::DECODED, 432);
            c.g[A0][0] = sx(u);
        });
    }
}
