//! Staging wave 206 (oc252): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use crate::abi;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0026_d178, replay_cache_update, "ReplayCache_Update"),
    (0x0010_6828, rider_update_sphere_tree, "Rider_UpdateSphereTree"),
    (
        0x0034_5ef8,
        dynamic_particle_modifier_transform_points,
        "DynamicParticleModifier_TransformPoints",
    ),
    (0x0041_5fc8, memcmp, "memcmp"),
    (
        0x003a_96e0,
        world_cache_streamer_get_view_clearance,
        "cWorldCache_StreamerGetViewClearance",
    ),
    (0x0010_3cc8, rider_anim_blend_seq2, "RiderAnim_BlendSeq2"),
    (0x0036_1098, mod_obj_get_matrices, "ModObj_GetMatrices"),
    (0x0032_f8f0, coll_hull_collide_triangle, "CollHull_CollideTriangle"),
    (0x0031_aa58, ps2_exec_man_wait_frame, "PS2ExecMan_WaitFrame"),
    (0x0011_98d8, rider_metrix_set_flip_amount, "RiderMetrix_SetFlipAmount"),
];

// ---------------------------------------------------------------------------
// ReplayCache_Update (0x26d178)

/// World pointer (`gp - 0x848`).
const GP_WORLD: u32 = 0x848;
/// World: the race manager (`+0x84`); race manager: the game state (`+0x28`).
const WORLD_RACE: u32 = 0x84;
const RACE_STATE: u32 = 0x28;
/// Game state: three "paused/busy" flags that stop recording.
const STATE_BUSY_A: u32 = 0x60c;
const STATE_BUSY_B: u32 = 0x608;
const STATE_BUSY_C: u32 = 0x610;
/// Replay cache: the chunk buffer (null: no replay).
const CACHE_CHUNKS: u32 = 0xc;
/// The switch's jump table on the game mode (15 entries).
const UPDATE_JUMP_TABLE: u32 = 0x0048_15b0;
const UPDATE_RECORD: u32 = 0x0026_d1d0;
const UPDATE_IDLE: u32 = 0x0026_d204;
const UPDATE_READ: u32 = 0x0026_d20c;
const UPDATE_DONE: u32 = 0x0026_d218;

const REPLAY_CACHE_RECORD_FRAME: Callee = Callee {
    addr: 0x0026_d2b0,
    f: ee::s07::ReplayCache_RecordFrame,
};
const REPLAY_CACHE_READ_FRAME: Callee = Callee {
    addr: 0x0026_d420,
    f: ee::s07::ReplayCache_ReadFrame,
};

/// `int ReplayCache_Update(ReplayCache* cache, void* frame)` (0x26d178):
/// once per tick, by game mode (the state word at world->race->state):
/// mode 0 records `frame` unless one of the state's busy flags is set
/// (then returns 1), modes 10-12/14 return 0, modes 1/2/4 read the next
/// recorded frame into `frame` (its result), the rest return 1. A cache
/// without a buffer returns 0.
pub fn replay_cache_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let cache = r32(c, A2);
        c.g[V0][0] = Addr(cache.wrapping_add(CACHE_CHUNKS)).lw(c);
        let gp = r32(c, GP);
        c.g[A0][0] = Addr(gp.wrapping_sub(GP_WORLD)).lw(c);
        'out: {
            if c.g[V0][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'out;
            }
            c.cycles += 7;
            c.g[A3][0] = 1;
            let world = r32(c, A0);
            c.g[V0][0] = Addr(world.wrapping_add(WORLD_RACE)).lw(c);
            let race = r32(c, V0);
            c.g[V1][0] = Addr(race.wrapping_add(RACE_STATE)).lw(c);
            let state = r32(c, V1);
            c.g[V1][0] = Addr(state).lw(c);
            c.g[V0][0] = (c.g[V1][0] < 0xf) as u64;
            c.g[T0][0] = c.g[A0][0];
            'done: {
                if c.g[V0][0] == 0 {
                    break 'done;
                }
                c.cycles += 7;
                c.g[V0][0] = sx(UPDATE_JUMP_TABLE);
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(UPDATE_JUMP_TABLE));
                let slot = r32(c, V1);
                c.g[A0][0] = Addr(slot).lw(c);
                let target = r32(c, A0);
                let mut case = target;
                if case == UPDATE_RECORD {
                    c.cycles += 5;
                    let world = r32(c, T0);
                    c.g[V0][0] = Addr(world.wrapping_add(WORLD_RACE)).lw(c);
                    let race = r32(c, V0);
                    c.g[A0][0] = Addr(race.wrapping_add(RACE_STATE)).lw(c);
                    let state = r32(c, A0);
                    c.g[V1][0] = Addr(state.wrapping_add(STATE_BUSY_A)).lw(c);
                    c.g[V0][0] = c.g[A3][0];
                    if c.g[V1][0] != 0 {
                        break 'out;
                    }
                    for busy in [STATE_BUSY_B, STATE_BUSY_C] {
                        c.cycles += 3;
                        let flag = Addr(state.wrapping_add(busy)).lw(c);
                        c.g[V0][0] = c.g[A3][0];
                        if flag != 0 {
                            break 'out;
                        }
                    }
                    c.cycles += 2;
                    c.g[A0][0] = c.g[A2][0];
                    REPLAY_CACHE_RECORD_FRAME.call(c, UPDATE_IDLE)?;
                    case = UPDATE_IDLE;
                }
                match case {
                    UPDATE_IDLE => {
                        c.cycles += 2;
                        c.g[A3][0] = 0;
                    }
                    UPDATE_READ => {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[A2][0];
                        REPLAY_CACHE_READ_FRAME.call(c, 0x0026_d214)?;
                        c.g[A3][0] = c.g[V0][0];
                    }
                    UPDATE_DONE => {}
                    _ => {
                        c.jump_out(target, 0x0026_d178, 0x0026_d228);
                        return Ok(());
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[A3][0];
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider_UpdateSphereTree (0x106828)

/// Rider: its collision sphere tree.
const RIDER_SPHERE_TREE: u32 = 0xaa0;
/// `RiderSphereTree_UpdateFromBones` (called through the function table).
const RIDER_SPHERE_TREE_UPDATE_FROM_BONES: u32 = 0x0032_9ae0;

/// `void Rider_UpdateSphereTree(Rider* rider)` (0x106828): move the rider's
/// collision spheres onto its current bone positions
/// (`RiderSphereTree_UpdateFromBones(rider->sphereTree)`).
pub fn rider_update_sphere_tree(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        Some(RIDER_SPHERE_TREE),
        crate::call::Target::Table(RIDER_SPHERE_TREE_UPDATE_FROM_BONES),
        0x0010_6838,
    );
}

// ---------------------------------------------------------------------------
// DynamicParticleModifier_TransformPoints (0x345ef8)

/// Modifier: the instance it is attached to, and its node (bone) index.
const DPM_INSTANCE: u32 = 0xc;
const DPM_NODE: u32 = 0x10;
/// Modifier: the two emitter points in node space, and in world space.
const DPM_LOCAL_A: u32 = 0x20;
const DPM_WORLD_A: u32 = 0x30;
const DPM_WORLD_B: u32 = 0x40;
const DPM_LOCAL_B: u32 = 0x50;

const INSTANCE_GET_NODE_MATRIX: Callee = Callee {
    addr: 0x0034_fed8,
    f: ee::s11::Instance_GetNodeMatrix,
};

/// `void DynamicParticleModifier_TransformPoints(DynamicParticleModifier* m)`
/// (0x345ef8): fetch the world matrix of the modifier's node on its
/// instance into a stack matrix, and transform the two node-space points
/// (`+0x20`, `+0x50`) into world space (`+0x30`, `+0x40`) with VU0
/// `mula/madda/madd` (x, y, z, w columns).
pub fn dynamic_particle_modifier_transform_points(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        c.g[A2][0] = c.g[SP][0];
        frame.save_ra(c, 0x60);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.g[A0][0] = Addr(this.wrapping_add(DPM_INSTANCE)).lw(c);
        c.g[A1][0] = Addr(this.wrapping_add(DPM_NODE)).lw(c);
        INSTANCE_GET_NODE_MATRIX.call(c, 0x0034_5f18)?;
        // One point: vf12 = M * p, through the stack slot at +0x50.
        let transform = |c: &mut Cpu, local: u32| {
            let this = r32(c, S0);
            c.lqc2(8, this.wrapping_add(local));
            c.lqc2(4, frame.at(c, 0));
            c.lqc2(5, frame.at(c, 0x10));
            c.lqc2(6, frame.at(c, 0x20));
            c.lqc2(7, frame.at(c, 0x30));
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.sqc2(12, frame.at(c, 0x50));
        };
        transform(c, DPM_LOCAL_A);
        frame.restore_ra(c, 0x60);
        c.lq(2, frame.at(c, 0x50));
        let this = r32(c, S0);
        c.sq(2, this.wrapping_add(DPM_WORLD_A));
        c.sq(2, frame.at(c, 0x40));
        transform(c, DPM_LOCAL_B);
        c.lq(2, frame.at(c, 0x50));
        let this = r32(c, S0);
        c.sq(2, this.wrapping_add(DPM_WORLD_B));
        frame.restore(c, S0, 0x70);
        c.sq(2, frame.at(c, 0x40));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// memcmp (0x415fc8)

/// `int memcmp(const void* a, const void* b, size_t n)` (0x415fc8): when
/// `n >= 16` and both pointers are 16-byte aligned, compare quadwords
/// (`lq`, `pxor`, fold the halves) while they match; the rest (or the
/// first differing quadword) byte by byte. Returns the difference of the
/// first differing bytes (unsigned), or 0.
pub fn memcmp(c: &mut Cpu) {
    const QUAD_LOOP: u32 = 0x0041_5fe0;
    const BYTE_LOOP: u32 = 0x0041_6030;
    c.cycles += 3;
    let short = c.g[A2][0] < 0x10;
    c.g[V0][0] = c.g[A0][0] | c.g[A1][0];
    'bytes: {
        if short {
            break 'bytes;
        }
        c.cycles += 3;
        c.g[V0][0] &= 0xf;
        if c.g[V0][0] != 0 {
            break 'bytes;
        }
        loop {
            c.cycles += 11;
            let a = r32(c, A0);
            let b = r32(c, A1);
            c.lq(3, a);
            c.g[A3][0] = (c.g[A2][0] < 0x20) as u64;
            c.lq(2, b);
            c.g[A0][0] = sx(a.wrapping_add(0x10));
            c.pxor(T0, V0, V1);
            c.g[V0][0] = sx(b.wrapping_add(0x10));
            c.pcpyud(T2, T0, A3);
            c.g[T1][0] = c.g[T2][0] | c.g[T0][0];
            if c.g[T1][0] != 0 {
                // A difference in this quadword: find it byte by byte.
                c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x10));
                break 'bytes;
            }
            c.g[A1][0] = c.g[V0][0];
            c.cycles += 2;
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(0x10));
            if c.g[A3][0] != 0 {
                break 'bytes;
            }
            if event_due(c) {
                deliver_event_here(c, QUAD_LOOP);
            }
        }
    }
    c.cycles += 5;
    c.g[V0][0] = u64::MAX;
    c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
    if c.g[A2][0] != u64::MAX {
        c.cycles += 2;
        c.g[A3][0] = u64::MAX;
        loop {
            c.cycles += 4;
            let a = r32(c, A0);
            let b = r32(c, A1);
            c.g[V1][0] = Addr(a).lbu(c);
            c.g[V0][0] = Addr(b).lbu(c);
            c.g[A0][0] = sx(a.wrapping_add(1));
            if c.g[V1][0] != c.g[V0][0] {
                c.cycles += 2;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                ret(c);
                return;
            }
            c.cycles += 3;
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
            c.g[A1][0] = sx(b.wrapping_add(1));
            if c.g[A2][0] == c.g[A3][0] {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, BYTE_LOOP);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

// ---------------------------------------------------------------------------
// cWorldCache_StreamerGetViewClearance (0x3a96e0)

/// World cache: per-view streamer records (0x50 bytes each): in use, the
/// view position quad, and the view's clearance (fade) value.
const STREAM_VIEW_SIZE: u64 = 0x50;
const STREAM_VIEW_USED: u32 = 0x240;
const STREAM_VIEW_POS: u32 = 0x250;
const STREAM_VIEW_CLEARANCE: u32 = 0x284;

const WORLD_CACHE_STREAMER_UPDATE_VIEW: Callee = Callee {
    addr: 0x003a_9d60,
    f: ee::s13::cWorldCache_StreamerUpdateView,
};

/// `int cWorldCache_StreamerGetViewClearance(cWorldCache* cache, int view,
/// Vec4* out_pos, float* out_t)` (0x3a96e0): for a used view record,
/// update the view, copy its position to `out_pos` and `clearance - 0.5` to
/// `out_t`; returns whether that is positive (0 for an unused record).
pub fn world_cache_streamer_get_view_clearance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[V0][0] = STREAM_VIEW_SIZE;
        let frame = Frame::open(c, 0x40);
        c.mult(0, V1, A1, V0);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
        let view = r32(c, S0);
        c.g[V0][0] = Addr(view.wrapping_add(STREAM_VIEW_USED)).lw(c);
        c.g[S2][0] = c.g[A3][0];
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 14;
            WORLD_CACHE_STREAMER_UPDATE_VIEW.call(c, 0x003a_9720)?;
            let view = r32(c, S0);
            c.lq(V0, view.wrapping_add(STREAM_VIEW_POS));
            c.g[V1][0] = 1;
            c.g[AT][0] = 0x3f00_0000; // 0.5
            c.mtc1(AT, 2);
            c.sq(V0, r32(c, S1));
            c.mtc1(ZERO, 1);
            c.lwc1(0, view.wrapping_add(STREAM_VIEW_CLEARANCE));
            c.sub_s(0, 0, 2);
            c.c_lt_s(1, 0);
            let positive = c.fcc();
            c.swc1(0, r32(c, S2));
            if !positive {
                c.cycles += 1;
                c.g[V1][0] = 0;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[V1][0];
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderAnim_BlendSeq2 (0x103cc8)

/// `gp+0xd8c`: the rider animation bank (animation ids at +0x1030).
const ANIM_BANK_GP: u32 = 0xd8c;
const BANK_IDS: u32 = 0x1030;
/// `gp-0x7f6c`: 1/60, ticks to seconds (`.lit4`).
const SEQ2_TICK_GP: u32 = 0x7f6c;
/// AnimSequence: track 0's time and length, track 1's speed.
const SEQ_TIME0: u32 = 0x8;
const SEQ_LENGTH0: u32 = 0x10;
const SEQ_SPEED1: u32 = 0x2c;
/// AnimSequence methods (through the function table).
const ANIM_SEQ_SET_TRACK_ANIM: u32 = 0x0031_3c50;
const ANIM_SEQ_SET_TRACK_TIME: u32 = 0x0031_3cf0;
const ANIM_SEQ_SET_TRACK_WEIGHT: u32 = 0x0031_3d28;
const ANIM_SEQ_SET_TRACK_LOOP: u32 = 0x0031_3d40;
const ANIM_SEQ_ADVANCE_TRACK: u32 = 0x0031_35b0;
const ANIM_SEQ_UPDATE_WEIGHT_FADE: u32 = 0x0031_3800;
const ANIM_SEQ_LIST_REMOVE: u32 = 0x0031_45f8;

/// `int RiderAnim_BlendSeq2(Rider* r, AnimList* list, AnimSequence* seq,
/// int key_neg, int key_mid, float dt, float v, int key_pos)` (0x103cc8):
/// a two-track blend: track 0 plays `key_mid`, track 1 plays `key_pos` for
/// `v > 0` or `key_neg` otherwise, weighted `1 - |v|` and `|v|`; track 0
/// loops and advances by `dt` seconds, track 1 follows its phase at its own
/// speed. When the weight fade finishes the sequence leaves `list`
/// (returns 1), else 0. (`a3` key_neg, `t0` key_mid, `t1` key_pos, `f12`
/// dt, `f13` v.)
pub fn rider_anim_blend_seq2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x60);
        let gp = r32(c, GP);
        c.g[V0][0] = Addr(gp.wrapping_add(ANIM_BANK_GP)).lw(c);
        frame.save(c, S0, 0x40);
        c.g[T0][0] = sx(r32(c, T0) << 2);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        c.g[T0][0] = sx(r32(c, T0).wrapping_add(r32(c, V0)));
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A3][0];
        frame.save_fpr(c, 21, 0x58);
        c.g[S3][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x50);
        c.f[21] = c.f[12];
        frame.save_ra(c, 0);
        c.f[20] = c.f[13];
        c.g[S1][0] = c.g[T1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = Addr(r32(c, T0).wrapping_add(BANK_IDS)).lw(c);
        c.g[A1][0] = 0;
        call_ptr(c, ANIM_SEQ_SET_TRACK_ANIM, 0x0010_3d1c)?;

        // Track 1: key_pos for v > 0, else key_neg with v negated.
        c.f[0] = 0;
        let v = FpuF32(c.f[20]);
        let positive = abi::c_lt_s(c, FpuF32::ZERO, v);
        let gp = r32(c, GP);
        c.g[V1][0] = Addr(gp.wrapping_add(ANIM_BANK_GP)).lw(c);
        let ret = if positive {
            c.cycles += 8;
            c.g[V0][0] = sx(r32(c, S1) << 2);
            0x0010_3d48
        } else {
            c.cycles += 7;
            c.g[V0][0] = sx(r32(c, S2) << 2);
            c.f[20] = (-FpuF32(c.f[20])).0;
            0x0010_3d6c
        };
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        c.g[A1][0] = 1;
        c.g[A2][0] = Addr(r32(c, V0).wrapping_add(BANK_IDS)).lw(c);
        call_ptr(c, ANIM_SEQ_SET_TRACK_ANIM, ret)?;

        // Weights 1 - w and w, track 0 loops.
        c.cycles += 33;
        c.g[AT][0] = 0x3f80_0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.f[12] = (FpuF32::ONE - FpuF32(c.f[20])).0;
        call_ptr(c, ANIM_SEQ_SET_TRACK_WEIGHT, 0x0010_3d84)?;
        c.f[12] = c.f[20];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        call_ptr(c, ANIM_SEQ_SET_TRACK_WEIGHT, 0x0010_3d94)?;
        c.g[A2][0] = 1;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        call_ptr(c, ANIM_SEQ_SET_TRACK_LOOP, 0x0010_3da4)?;

        // Advance track 0 by dt seconds.
        let gp = r32(c, GP);
        let tick = Addr(gp.wrapping_sub(SEQ2_TICK_GP)).read_fpu(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        let step = FpuF32(c.f[21]) * tick;
        c.f[20] = step.0;
        c.f[12] = step.0;
        call_ptr(c, ANIM_SEQ_ADVANCE_TRACK, 0x0010_3dbc)?;

        // Track 1 follows track 0's phase at its own speed.
        let seq = r32(c, S0);
        let time0 = Addr(seq.wrapping_add(SEQ_TIME0)).read_fpu(c);
        c.g[A0][0] = c.g[S0][0];
        let length0 = Addr(seq.wrapping_add(SEQ_LENGTH0)).read_fpu(c);
        c.f[1] = length0.0;
        c.g[A1][0] = 1;
        let speed1 = Addr(seq.wrapping_add(SEQ_SPEED1)).read_fpu(c);
        c.f[0] = speed1.0;
        c.f[12] = (time0.div(length0) * speed1).0;
        call_ptr(c, ANIM_SEQ_SET_TRACK_TIME, 0x0010_3ddc)?;
        c.f[12] = c.f[20];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, ANIM_SEQ_UPDATE_WEIGHT_FADE, 0x0010_3de8)?;
        let faded = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S3][0];
        if faded {
            c.cycles += 3;
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, ANIM_SEQ_LIST_REMOVE, 0x0010_3e00)?;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x58);
        frame.restore_fpr(c, 20, 0x50);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ModObj_GetMatrices (0x361098)

/// ModObj: the embedded transform modifier (`+0x14`, its vtable at `+0x20`),
/// the flags (`+0x26`, bit 0: the modifier is active) and the matrix array.
const MODOBJ_MODIFIER: u32 = 0x14;
const MODOBJ_MODIFIER_VT: u32 = 0x20;
const MODOBJ_FLAGS: u32 = 0x26;
const MODOBJ_MATRICES: u32 = 0x44;
/// The modifier's "update matrices" vtable slot.
const MODIFIER_UPDATE_SLOT: u32 = 0x198;

/// `Matrix* ModObj_GetMatrices(ModObj* self)` (0x361098): the object's
/// matrix array (`+0x44`), first letting the active transform modifier
/// update it (virtual slot 0x198 on the embedded modifier at `+0x14`).
pub fn mod_obj_get_matrices(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.g[V0][0] = Addr(this.wrapping_add(MODOBJ_FLAGS)).lhu(c) & 1;
        c.g[A1][0] = sx(this.wrapping_add(MODOBJ_MODIFIER));
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = Addr(this.wrapping_add(MODOBJ_MODIFIER_VT)).lw(c);
            let vt = r32(c, V0);
            c.g[A0][0] = Addr(vt.wrapping_add(MODIFIER_UPDATE_SLOT)).lh(c);
            c.g[V1][0] = Addr(vt.wrapping_add(MODIFIER_UPDATE_SLOT + 4)).lw(c);
            let method = r32(c, V1);
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            call_ptr(c, method, 0x0036_10cc)?;
        }
        c.cycles += 5;
        let this = r32(c, S0);
        c.g[V0][0] = Addr(this.wrapping_add(MODOBJ_MATRICES)).lw(c);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// CollHull_CollideTriangle (0x32f8f0)

/// The hull's sphere tree versus a triangle (`0x32a1c0`, through the table).
const SPHERE_TREE_COLLIDE_TRIANGLE: u32 = 0x0032_a1c0;

/// `int CollHull_CollideTriangle(CollHull* hull, const Vec4* p, const Vec4* a,
/// const Vec4* b, const Vec4* n, Vec4* point, float* out_len, Vec4* out_dir)`
/// (0x32f8f0): collide the hull's sphere tree (`+0x60`) with the triangle;
/// on contact store the push vector's length in `out_len` and its
/// direction in `out_dir` (VU0 sqrt/div). Returns the contact flag. The
/// same shape as `CollHull_CollideTriangle2Sided`, with the one-sided test.
pub fn coll_hull_collide_triangle(c: &mut Cpu) {
    guest(c, |c| {
        crate::game::intersect::riderspheretree::coll_sphere_tree_common(
            c,
            SPHERE_TREE_COLLIDE_TRIANGLE,
            0x0032_f918,
        )
    })
}

// ---------------------------------------------------------------------------
// PS2ExecMan_WaitFrame (0x31aa58)

/// PS2ExecMan: vtable, the frame semaphore id, the "waiting" flag.
const EXEC_VT: u32 = 0x8;
const EXEC_FRAME_SEMA: u32 = 0x18;
const EXEC_WAITING: u32 = 0x1c;
/// Virtual slots called before and after the wait.
const EXEC_PRE_WAIT_SLOT: u32 = 0x58;
const EXEC_POST_WAIT_SLOT: u32 = 0x60;
/// `WaitSema` and `PollSema` (EE kernel wrappers, through the table).
const WAIT_SEMA: u32 = 0x0042_3de0;
const POLL_SEMA: u32 = 0x0042_3df0;

/// `void PS2ExecMan_WaitFrame(PS2ExecMan* self)` (0x31aa58): mark the manager
/// waiting, call its pre-wait hook (virtual slot 0x58), block on the frame
/// semaphore, clear the flag, drain the semaphore (`PollSema` until it no
/// longer returns the id), then call the post-wait hook (slot 0x60).
pub fn ps2_exec_man_wait_frame(c: &mut Cpu) {
    const DRAIN_LOOP: u32 = 0x0031_aa90;
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        Addr(this.wrapping_add(EXEC_WAITING)).sw(c, 1);
        virtual_call(c, EXEC_PRE_WAIT_SLOT, 0x0031_aa84)?;
        let this = r32(c, S0);
        c.g[A0][0] = Addr(this.wrapping_add(EXEC_FRAME_SEMA)).lw(c);
        call_ptr(c, WAIT_SEMA, 0x0031_aa8c)?;
        let this = r32(c, S0);
        Addr(this.wrapping_add(EXEC_WAITING)).sw(c, 0);
        loop {
            c.cycles += 5;
            let this = r32(c, S0);
            c.g[A0][0] = Addr(this.wrapping_add(EXEC_FRAME_SEMA)).lw(c);
            call_ptr(c, POLL_SEMA, 0x0031_aa98)?;
            let this = r32(c, S0);
            c.g[V1][0] = Addr(this.wrapping_add(EXEC_FRAME_SEMA)).lw(c);
            if c.g[V0][0] != c.g[V1][0] {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, DRAIN_LOOP);
            }
        }
        c.cycles += 9;
        // (`v0` holds the vtable here, `v1` the method.)
        let this = r32(c, S0);
        c.g[V0][0] = Addr(this.wrapping_add(EXEC_VT)).lw(c);
        let vt = r32(c, V0);
        c.g[A0][0] = Addr(vt.wrapping_add(EXEC_POST_WAIT_SLOT)).lh(c);
        c.g[V1][0] = Addr(vt.wrapping_add(EXEC_POST_WAIT_SLOT + 4)).lw(c);
        let method = r32(c, V1);
        c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x0031_aab8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The pre-wait virtual call: `v1` holds the vtable, `v0` the method.
fn virtual_call(c: &mut Cpu, slot: u32, ret: u32) -> crate::call::Flow {
    let this = r32(c, S0);
    c.g[V1][0] = Addr(this.wrapping_add(EXEC_VT)).lw(c);
    let vt = r32(c, V1);
    c.g[A0][0] = Addr(vt.wrapping_add(slot)).lh(c);
    c.g[V0][0] = Addr(vt.wrapping_add(slot + 4)).lw(c);
    let method = r32(c, V0);
    c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
    call_ptr(c, method, ret)
}

// ---------------------------------------------------------------------------
// RiderMetrix_SetFlipAmount (0x1198d8)

/// RiderMetrix: the trick's accumulated rotation total and the flip amount.
const METRIX_ROTATION_TOTAL: u32 = 0x14;
const METRIX_FLIP_AMOUNT: u32 = 0x38;
/// `gp-0x7a10`: the flip weight in the rotation total (0.0286..., `.lit4`).
const FLIP_WEIGHT_GP: u32 = 0x7a10;

/// `float RiderMetrix_SetFlipAmount(RiderMetrix* m, float flip)` (0x1198d8):
/// replace the flip amount, swapping its weighted magnitude in the
/// rotation total (`total - |old| * k + |flip| * k`, not below 0). Returns
/// 0.0 in f0.
pub fn rider_metrix_set_flip_amount(c: &mut Cpu) {
    c.cycles += 15;
    let m = Addr(abi::arg(c, 0));
    let gp = r32(c, GP);
    let k = Addr(gp.wrapping_sub(FLIP_WEIGHT_GP)).read_fpu(c);
    c.f[2] = k.0;
    let flip = FpuF32(c.f[12]);
    let old = m.off(METRIX_FLIP_AMOUNT).read_fpu(c);
    let total = m.off(METRIX_ROTATION_TOTAL).read_fpu(c);
    let removed = old.abs() * k;
    m.off(METRIX_FLIP_AMOUNT).write_fpu(c, flip);
    let added = flip.abs() * k;
    let less = total - removed;
    let new_total = (less + added).max(FpuF32::ZERO);
    c.f[3] = less.0;
    c.f[2] = 0;
    c.f[0] = 0;
    c.f[1] = new_total.0;
    m.off(METRIX_ROTATION_TOTAL).write_fpu(c, new_total);
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::fakes::machine;
    use crate::difftest::{check, fill, put_words, Rng, DATA, GAME_GP};

    /// `ReplayCache_Update`: every game mode 0..14 and some out of range,
    /// the busy flags (none or one of three) in record mode, a cache with or
    /// without a buffer; the real `ReplayCache_RecordFrame` (with `memcmp`
    /// and `ReplayCache_PushFrame`) and `ReplayCache_ReadFrame` run.
    #[test]
    fn replay_cache_update_matches() {
        const CACHE: u32 = DATA;
        const WORLD: u32 = DATA + 0x100;
        const RACE: u32 = DATA + 0x200;
        const STATE: u32 = DATA + 0x400;
        const CHUNKS: u32 = DATA + 0x1000;
        const FRAME: u32 = DATA + 0x2000;
        check(
            ee::s07::f_0026d178__generated,
            replay_cache_update,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x26d178, 0);
                c.pc = 0x0026_d178;
                put_words(
                    c,
                    UPDATE_JUMP_TABLE,
                    &[
                        0x26d1d0, 0x26d20c, 0x26d20c, 0x26d218, 0x26d20c, 0x26d218, 0x26d218,
                        0x26d218, 0x26d218, 0x26d218, 0x26d204, 0x26d204, 0x26d204, 0x26d218,
                        0x26d204,
                    ],
                );
                c.bus.write32(GAME_GP - GP_WORLD, WORLD);
                c.bus.write32(WORLD + WORLD_RACE, RACE);
                c.bus.write32(RACE + RACE_STATE, STATE);
                let mode = match seed % 20 {
                    15 => 15 + r.next_u32() % 100,
                    16 => 0x8000_0000 | r.next_u32(),
                    17..=19 => 0,
                    m => m,
                };
                c.bus.write32(STATE, mode);
                for (i, off) in [STATE_BUSY_A, STATE_BUSY_B, STATE_BUSY_C].into_iter().enumerate() {
                    let set = (seed / 20) % 4 == i as u32 + 1;
                    c.bus.write32(STATE + off, if set { r.next_u32() | 1 } else { 0 });
                }
                // The cache: count, read position, repeat left, chunks.
                fill(c, &mut r, CHUNKS, 32);
                let count = 1 + r.next_u32() % 12;
                c.bus.write32(CACHE, count);
                c.bus.write32(CACHE + 4, r.next_u32() % (count + 1));
                c.bus.write32(CACHE + 8, r.next_u32() % 4);
                let chunks = if seed % 23 == 0 { 0 } else { CHUNKS };
                c.bus.write32(CACHE + CACHE_CHUNKS, chunks);
                let last = CHUNKS + 8 * (count - 1);
                let mut frame = c.bus.read64(last) & !0xfff;
                if seed % 3 == 0 {
                    frame ^= 1u64 << (12 + r.next_u32() % 52);
                }
                c.bus.write64(FRAME, frame);
                c.g[A0][0] = sx(CACHE);
                c.g[A1][0] = sx(FRAME);
            },
        );
    }

    /// `Rider_UpdateSphereTree`: a sphere tree of 0..5 spheres (and a
    /// negative count) with random bone indices; the real
    /// `RiderSphereTree_UpdateFromBones` copies the bone positions.
    #[test]
    fn rider_update_sphere_tree_matches() {
        const RIDER: u32 = DATA;
        const TREE: u32 = DATA + 0x1000;
        const BONES: u32 = DATA + 0x2000;
        check(
            ee::s00::f_00106828__generated,
            rider_update_sphere_tree,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x106828, 0);
                c.pc = 0x0010_6828;
                fill(c, &mut r, BONES, 8 * 16);
                fill(c, &mut r, TREE, 0x100);
                c.bus.write32(RIDER + RIDER_SPHERE_TREE, TREE);
                c.bus.write32(TREE, BONES);
                let count = match seed % 7 {
                    6 => 0xffff_fff0 | r.next_u32() % 16,
                    n => n as u32,
                };
                c.bus.write32(TREE + 0x2c, count);
                c.bus.write32(TREE + 0x24, r.next_u32() % 16);
                for i in 0..6 {
                    c.bus.write32(TREE + 0x44 + 0x20 * i, r.next_u32() % 16);
                }
                c.g[A0][0] = sx(RIDER);
            },
        );
    }


    /// `DynamicParticleModifier_TransformPoints`: an instance without an
    /// owner, node -1 (the world matrix) or a node with 0..2 parents (the
    /// real `Instance_GetNodeMatrix` chains them), random points.
    #[test]
    fn dynamic_particle_modifier_transform_points_matches() {
        const INST: u32 = DATA;
        const SKEL: u32 = DATA + 0x400;
        const NODES: u32 = DATA + 0x500;
        const MATS: u32 = DATA + 0x1000;
        const MODIFIER: u32 = DATA + 0x2000;
        check(
            ee::s11::f_00345ef8__generated,
            dynamic_particle_modifier_transform_points,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x345ef8, 0);
                c.pc = 0x0034_5ef8;
                for k in 0..0x10 {
                    c.bus.write32(INST + 0x10 + 4 * k, r.f32(4.0));
                }
                c.bus.write32(INST + 0xc, 0);
                c.bus.write32(INST + 0x84, r.f32(2.0));
                c.bus.write32(INST + 0x80, SKEL);
                c.bus.write32(SKEL + 8, NODES);
                for (n, parent) in [(0u32, u32::MAX), (1, 0), (2, 1), (3, 0)] {
                    c.bus.write32(NODES + 0x10 * n, parent);
                    c.bus.write32(NODES + 0x10 * n + 0xc, MATS + 0x40 * n);
                }
                for k in 0..0x40 {
                    c.bus.write32(MATS + 4 * k, r.f32(4.0));
                }
                fill(c, &mut r, MODIFIER, 0x18);
                for k in 0..4 {
                    c.bus.write32(MODIFIER + DPM_LOCAL_A + 4 * k, r.f32(10.0));
                    c.bus.write32(MODIFIER + DPM_LOCAL_B + 4 * k, r.edge_f32());
                }
                let node = match seed % 5 {
                    0 => u32::MAX,
                    k => k - 1,
                };
                c.bus.write32(MODIFIER + DPM_INSTANCE, INST);
                c.bus.write32(MODIFIER + DPM_NODE, node);
                c.g[A0][0] = sx(MODIFIER);
            },
        );
    }


    /// `memcmp`: lengths 0..80, aligned or misaligned pointers, equal
    /// buffers or one differing byte anywhere (inside or past `n`).
    #[test]
    fn memcmp_matches() {
        const A: u32 = DATA + 0x100;
        const B: u32 = DATA + 0x400;
        check(ee::s15::f_00415fc8__generated, memcmp, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x415fc8, 0);
            let (oa, ob) = match seed % 4 {
                0 | 1 => (0, 0),
                2 => (r.next_u32() % 16, 0),
                _ => (0x10 * (r.next_u32() % 2), r.next_u32() % 16),
            };
            let n = match seed % 5 {
                0 => r.next_u32() % 16,
                _ => r.next_u32() % 80,
            };
            for i in 0..0x60 {
                let v = r.next_u32();
                c.bus.write32(A + 4 * i, v);
                c.bus.write32(B + 4 * i, v);
            }
            // Shift B so that A+oa and B+ob hold the same bytes.
            for i in 0..0x100u32 {
                let v = c.bus.read8(A + oa + i);
                c.bus.write8(B + ob + i, v);
            }
            if seed % 3 != 0 {
                let at = r.next_u32() % (n + 4);
                let v = c.bus.read8(B + ob + at);
                c.bus.write8(B + ob + at, v ^ (1 + r.next_u32() % 255) as u8);
            }
            c.g[A0][0] = sx(A + oa);
            c.g[A1][0] = sx(B + ob);
            c.g[A2][0] = n as u64;
        });
    }


    /// `cWorldCache_StreamerGetViewClearance`: view 0 or 1, used or not,
    /// random position and edge-case clearance values; the real
    /// `cWorldCache_StreamerUpdateView` runs on the empty cache.
    #[test]
    fn world_cache_streamer_get_view_clearance_matches() {
        check(
            ee::s13::f_003a96e0__generated,
            world_cache_streamer_get_view_clearance,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a96e0, 0);
                crate::game::fe::feasyncfile::tests::env(c);
                c.pc = 0x003a_96e0;
                let this = DATA + 0x400;
                let idx = seed % 2;
                let e = this + 0x50 * idx;
                c.bus.write32(e + STREAM_VIEW_USED, (seed % 3 != 0) as u32);
                for k in 0..4u32 {
                    c.bus.write32(e + STREAM_VIEW_POS + 4 * k, r.next_u32());
                }
                let t = match seed % 4 {
                    0 => 0x3f00_0000,
                    _ => r.edge_f32(),
                };
                c.bus.write32(e + STREAM_VIEW_CLEARANCE, t);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = sx(DATA + 0x2000);
                c.g[A3][0] = sx(DATA + 0x2100);
            },
        );
    }


    /// `RiderAnim_BlendSeq2`: random keys into an id bank, v around 0
    /// (positive, zero, negative, edge cases), random sequence times and
    /// track states; the real AnimSequence methods (from the game image)
    /// run, including the list removal when the fade finishes.
    #[test]
    fn rider_anim_blend_seq2_matches() {
        use crate::difftest::elf_image;
        use crate::difftest::fakes::ptr;
        check(
            ee::s00::f_00103cc8__generated,
            rider_anim_blend_seq2,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_3cc8, 0);
                elf_image(c);
                c.pc = 0x0010_3cc8;
                let bank = ptr(&mut r);
                c.bus.write32(GAME_GP + ANIM_BANK_GP, bank);
                for k in 0..8 {
                    c.bus.write32(bank + BANK_IDS + 4 * k, 0x100 + k * 0x101);
                }
                for reg in [A3, T0, T1] {
                    c.g[reg][0] = (r.next_u32() % 8) as u64;
                }
                let seq = ptr(&mut r);
                let list = ptr(&mut r);
                c.g[A0][0] = sx(ptr(&mut r));
                c.g[A1][0] = sx(list);
                c.g[A2][0] = sx(seq);
                for off in [0x8, 0xc, 0x10, 0x2c, 0x90, 0x94, 0x98, 0x9c, 0xa4, 0xa8] {
                    let v = match r.next_u32() % 4 {
                        0 => 0,
                        1 => r.edge_f32(),
                        _ => r.f32(3.0),
                    };
                    c.bus.write32(seq + off, v);
                }
                c.bus.write32(seq + 0x4, 0x100 + (r.next_u32() % 8) * 0x101);
                c.bus.write32(seq + 0x1c, r.next_u32() & 1);
                c.bus.write32(seq + 0xa0, r.next_u32() & 1);
                let mut at = list + 4;
                for _ in 0..seed % 3 {
                    let other = ptr(&mut r) + 0x8;
                    c.bus.write32(at, other);
                    at = other + 200;
                }
                let vs = [-1.0f32, -0.5, -0.25, 0.0, 0.25, 0.5, 1.0];
                c.f[13] = match seed % 9 {
                    7 => r.edge_f32(),
                    8 => r.f32(1.0),
                    i => vs[i as usize].to_bits(),
                };
                c.f[12] = if seed % 4 == 0 { r.edge_f32() } else { r.f32(3.0) };
            },
        );
    }


    /// `ModObj_GetMatrices`: modifier active or not; the modifier's update
    /// slot is a fake (logging the call, sometimes unwinding).
    #[test]
    fn mod_obj_get_matrices_matches() {
        use crate::difftest::fakes::vtable;
        check(
            ee::s12::f_00361098__generated,
            mod_obj_get_matrices,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0036_1098, 1);
                c.pc = 0x0036_1098;
                c.g[A0][0] = sx(DATA);
                let flags = (r.next_u32() & 0xfffe) as u16 | (seed % 2) as u16;
                c.bus.write16(DATA + MODOBJ_FLAGS, flags);
                c.bus.write32(DATA + MODOBJ_MATRICES, DATA + 0x3000);
                let vt = vtable(c, &mut r, &[MODIFIER_UPDATE_SLOT]);
                c.bus.write32(DATA + MODOBJ_MODIFIER_VT, vt);
            },
        );
    }


    /// `CollHull_CollideTriangle`: a single sphere (no child mask) over a
    /// triangle in the y = 0 plane, above/inside it (contact: push length
    /// and direction written) or too far or outside (no contact); the real
    /// sphere-tree test runs with the game's constants.
    #[test]
    fn coll_hull_collide_triangle_matches() {
        use crate::difftest::{elf_image, put_vec};
        const HULL: u32 = DATA;
        const TREE: u32 = DATA + 0x100;
        const P: u32 = DATA + 0x200;
        const A: u32 = DATA + 0x210;
        const B: u32 = DATA + 0x220;
        const N: u32 = DATA + 0x230;
        const POINT: u32 = DATA + 0x300;
        const LEN: u32 = DATA + 0x400;
        const DIR: u32 = DATA + 0x500;
        let f = |x: f32| x.to_bits();
        check(
            ee::s11::f_0032f8f0__generated,
            coll_hull_collide_triangle,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0032_f8f0, 0);
                elf_image(c);
                c.pc = 0x0032_f8f0;
                for i in 0..0x40 {
                    c.bus.write32(TREE + 4 * i, 0);
                }
                c.bus.write32(HULL + 0x60, TREE);
                let (x, z) = match seed % 4 {
                    3 => (8.0, 8.0), // outside the triangle
                    _ => (1.0 + (r.next_u32() % 4) as f32, 1.0 + (r.next_u32() % 4) as f32),
                };
                let y = match seed % 5 {
                    0 => 3.0, // above the sphere's reach
                    1 => -0.5,
                    _ => (r.next_u32() % 100) as f32 / 120.0,
                };
                put_vec(c, TREE + 0x10, [f(x), f(y), f(z), f(1.0)]);
                c.bus.write32(TREE + 0x20, f(1.0));
                put_vec(c, P, [0, 0, 0, f(1.0)]);
                put_vec(c, A, [f(10.0), 0, 0, f(1.0)]);
                put_vec(c, B, [0, 0, f(10.0), f(1.0)]);
                put_vec(c, N, [0, f(1.0), 0, 0]);
                c.g[A0][0] = sx(HULL);
                c.g[A1][0] = sx(P);
                c.g[A2][0] = sx(A);
                c.g[A3][0] = sx(B);
                c.g[T0][0] = sx(N);
                c.g[T1][0] = sx(POINT);
                c.g[T2][0] = sx(DIR);
                c.g[T3][0] = sx(LEN);
            },
        );
    }


    /// `PS2ExecMan_WaitFrame`: fake pre/post hooks (v-table), the real
    /// `WaitSema`/`PollSema` wrappers on the fake syscall; `PollSema`
    /// answers the semaphore id 0..3 times before something else; any call
    /// may unwind.
    #[test]
    fn ps2_exec_man_wait_frame_matches() {
        use crate::difftest::fakes::{vtable, LOG_RESULTS};
        const SEMA: u32 = 0x17;
        check(
            ee::s10::f_0031aa58__generated,
            ps2_exec_man_wait_frame,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_aa58, 6);
                c.pc = 0x0031_aa58;
                let this = DATA;
                let vt = vtable(c, &mut r, &[EXEC_PRE_WAIT_SLOT, EXEC_POST_WAIT_SLOT]);
                c.bus.write32(this + EXEC_VT, vt);
                c.bus.write32(this + EXEC_FRAME_SEMA, SEMA);
                c.bus.write32(this + EXEC_WAITING, r.next_u32());
                // Calls: 0 pre hook, 1 WaitSema, 2.. PollSema, then the post hook.
                let polls = seed % 4;
                for k in 0..polls {
                    c.bus.write32(LOG_RESULTS + 4 * (2 + k), SEMA);
                }
                c.g[A0][0] = sx(this);
            },
        );
    }


    /// `RiderMetrix_SetFlipAmount`: random and edge-case old/new flip
    /// amounts and totals (including totals that would go negative), the
    /// game's weight constant or an edge case.
    #[test]
    fn rider_metrix_set_flip_amount_matches() {
        check(
            ee::s00::f_001198d8__generated,
            rider_metrix_set_flip_amount,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0011_98d8, 0);
                c.pc = 0x0011_98d8;
                let k = if seed % 5 == 0 { r.edge_f32() } else { 0x3cea_aef8 };
                c.bus.write32(GAME_GP - FLIP_WEIGHT_GP, k);
                let pick = |r: &mut Rng, i: u32| match i % 3 {
                    0 => r.edge_f32(),
                    _ => r.f32(400.0),
                };
                c.bus.write32(DATA + METRIX_FLIP_AMOUNT, pick(&mut r, seed));
                c.bus.write32(DATA + METRIX_ROTATION_TOTAL, pick(&mut r, seed / 3));
                c.f[12] = pick(&mut r, seed / 9);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

}
