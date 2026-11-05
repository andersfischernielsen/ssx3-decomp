//! Staging wave 207 (oc260): hand-written replacements for the hottest
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

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0010_f998, ai_update_race_positions, "AI_UpdateRacePositions"),
    (0x002f_5148, world_light_man_light_instance_cb, "WorldLightMan_LightInstanceCB"),
    (0x001f_3188, load_indicator_update, "LoadIndicator_Update"),
    (0x0013_f8f8, obj_ptr_array_merge_sort, "ObjPtrArray_MergeSort"),
    (0x0038_6640, ps2_graphics_man_reset_view_effects, "PS2GraphicsMan_v115"),
    (0x002e_e978, world_painter_get_weather98, "WorldPainter_GetWeather98"),
    (0x002e_e5b8, world_painter_get_weather38, "WorldPainter_GetWeather38"),
    (0x0035_9698, spline_modifier_update, "SplineModifier_Update"),
    (0x003d_62c8, path_track_set_volume, "PATH_TrackSetVolume"),
    (0x0039_6128, tex_man_invalidate_bound, "TexMan_InvalidateBound"),
];

// ---------------------------------------------------------------------------
// 0x10f998 AI_UpdateRacePositions

/// Field offsets of the AI (race manager) and its riders used by
/// [`ai_update_race_positions`].
mod race_positions {
    use super::*;
    /// AI: the rider pointer array.
    pub const AI_RIDERS: u32 = 0x28;
    /// AI: how riders are ranked (0 off, 1 by distance to finish, 2 by score).
    pub const AI_RANK_MODE: u32 = 0x74;
    /// AI: number of riders.
    pub const AI_COUNT: u32 = 0x78;
    /// Rider: race position (0 = first).
    pub const RIDER_POSITION: u32 = 0xec;
    /// Rider: a float compared with 10.0 (the result is dead; only v0 keeps it).
    pub const RIDER_RACE_TIME: u32 = 0x470;
    /// Rider: the score object (its +0x198 is the score).
    pub const RIDER_SCORE_OBJ: u32 = 0x790;
    /// Rider: finished flag.
    pub const RIDER_FINISHED: u32 = 0x878;
    /// Score object: points.
    pub const SCORE_POINTS: u32 = 0x198;

    pub const ARE_ALL_RIDERS_LOADED: Callee = Callee {
        addr: 0x0012_a180,
        f: ee::s00::AI_AreAllRidersLoaded,
    };
    pub const GET_DIST_TO_FINISH: Callee = Callee {
        addr: 0x0011_3128,
        f: ee::s00::Rider_GetDistToFinish,
    };
    /// `EA_ShellSortPairs(n, keys, values)` (cross-shard).
    pub const SHELL_SORT_PAIRS: u32 = 0x003e_6328;
    /// `SSXAudioSystem_GetInstance()` (cross-shard).
    pub const AUDIO_GET_INSTANCE: u32 = 0x0028_b180;
    /// `SSXAudioSystem_OnRacePositionChange(audio, rider, behind, ahead,
    /// old_position, new_position)` (cross-shard).
    pub const AUDIO_ON_POSITION_CHANGE: u32 = 0x0029_9e28;

    /// `base + off` of a register value (32-bit address arithmetic).
    #[inline(always)]
    pub fn race_pos_at(c: &Cpu, reg: usize, off: u32) -> u32 {
        r32(c, reg).wrapping_add(off)
    }
}

/// `void AI_UpdateRacePositions(AI* ai)` (0x10f998): once every rider is
/// loaded, give each rider a sort key (rank mode 1: finished riders 0xffff,
/// others `-(int)(dist_to_finish + 20 * position)`; mode 2: finished riders
/// -65535, others their score; any other mode leaves the stack slot as it
/// is), shell-sort the rider indices by key, then store each rider's new
/// position, telling the audio system (with the riders just behind and
/// ahead) about every rider whose position changed.
pub fn ai_update_race_positions(c: &mut Cpu) {
    use race_positions::*;
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, S3, 0xa0);
        frame.save(c, S0, 0xd0);
        c.g[S3][0] = c.g[A0][0];
        for (reg, off) in [
            (S1, 0xc0),
            (S2, 0xb0),
            (S4, 0x90),
            (S5, 0x80),
            (S6, 0x70),
            (S7, 0x60),
            (FP, 0x50),
        ] {
            frame.save(c, reg, off);
        }
        frame.save_ra(c, 0x40);
        c.g[V0][0] = c.lw(race_pos_at(c, S3, AI_RANK_MODE));
        frame.restore(c, S0, 0xd0);
        'out: {
            if c.g[V0][0] == 0 {
                break 'out;
            }
            c.cycles += 4;
            ARE_ALL_RIDERS_LOADED.call(c, 0x0010_f9dc)?;
            c.g[S1][0] = 0;
            'done: {
                if c.g[V0][0] == 0 {
                    break 'done;
                }
                // Pass 1: indices at sp+0 (s5), keys at sp+0x20 (s2/s7).
                c.cycles += 4;
                c.g[V0][0] = c.lw(race_pos_at(c, S3, AI_COUNT));
                c.g[S7][0] = sx(sp(c).wrapping_add(0x20));
                c.g[A0][0] = c.g[V0][0];
                if (c.g[V0][0] as i64) > 0 {
                    c.cycles += 5;
                    c.g[S6][0] = 0x14;
                    c.g[S2][0] = c.g[S7][0];
                    c.g[S5][0] = c.g[SP][0];
                    c.g[FP][0] = 1;
                    c.g[S4][0] = sx(race_pos_at(c, S3, AI_RIDERS));
                    loop {
                        // 0x10fa08
                        c.cycles += 4;
                        c.g[V1][0] = c.lw(race_pos_at(c, S3, AI_RANK_MODE));
                        c.sw(r32(c, S5), c.g[S1][0]);
                        c.g[S0][0] = c.lw(r32(c, S4));
                        let mut keyed = true;
                        if c.g[V1][0] == c.g[FP][0] {
                            // Rank by distance to the finish.
                            c.cycles += 3;
                            let finished = c.lw(race_pos_at(c, S0, RIDER_FINISHED));
                            c.g[V0][0] = 0xffff;
                            if finished != 0 {
                                c.cycles += 2;
                                c.sw(r32(c, S2), c.g[V0][0]);
                            } else {
                                c.cycles += 7;
                                c.lwc1(1, race_pos_at(c, S0, RIDER_RACE_TIME));
                                c.g[AT][0] = 0x4120_0000; // 10.0
                                c.mtc1(AT, 0);
                                c.c_le_s(0, 1);
                                c.g[V0][0] = 1;
                                if !c.fcc() {
                                    c.cycles += 1;
                                    c.g[V0][0] = 0;
                                }
                                c.cycles += 12;
                                c.g[A0][0] = c.g[S0][0];
                                GET_DIST_TO_FINISH.call(c, 0x0010_fa68)?;
                                // key = -(int)(dist + position * 20)
                                c.g[V0][0] = c.lw(race_pos_at(c, S0, RIDER_POSITION));
                                c.mult(0, V0, V0, S6);
                                c.mtc1(V0, 1);
                                c.cvt_s_w(1, 1);
                                c.add_s(0, 0, 1);
                                c.cvt_w_s(1, 0);
                                c.mfc1(V0, 1);
                                c.g[V0][0] = sx(0u32.wrapping_sub(r32(c, V0)));
                                c.sw(r32(c, S2), c.g[V0][0]);
                            }
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 2;
                            if c.g[V1][0] == c.g[V0][0] {
                                // Rank by score.
                                c.g[V0][0] = c.lw(race_pos_at(c, S0, RIDER_FINISHED));
                                c.cycles += 2;
                                if c.g[V0][0] == 0 {
                                    c.g[V0][0] = c.lw(race_pos_at(c, S0, RIDER_SCORE_OBJ));
                                    c.cycles += 2;
                                    c.g[V1][0] = c.lw(race_pos_at(c, V0, SCORE_POINTS));
                                    c.sw(r32(c, S2), c.g[V1][0]);
                                } else {
                                    c.cycles += 4;
                                    c.g[V0][0] = 0xffff_ffff_ffff_0001;
                                    c.sw(r32(c, S2), c.g[V0][0]);
                                }
                            } else {
                                c.cycles += 2;
                                c.g[V0][0] = c.lw(race_pos_at(c, S3, AI_COUNT));
                                keyed = false;
                            }
                        }
                        if keyed {
                            c.cycles += 1;
                            c.g[V0][0] = c.lw(race_pos_at(c, S3, AI_COUNT));
                        }
                        // 0x10fab4
                        c.cycles += 7;
                        c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                        c.g[S2][0] = sx(r32(c, S2).wrapping_add(4));
                        c.g[S5][0] = sx(r32(c, S5).wrapping_add(4));
                        c.g[A0][0] = c.g[V0][0];
                        let more = (c.g[S1][0] as i64) < (c.g[V0][0] as i64);
                        c.g[V0][0] = more as u64;
                        c.g[S4][0] = sx(r32(c, S4).wrapping_add(4));
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0010_fa08);
                        }
                    }
                }
                // 0x10fad0: sort the indices by key.
                c.cycles += 6;
                c.g[A1][0] = c.g[S7][0];
                c.g[A2][0] = c.g[SP][0];
                call_ptr(c, SHELL_SORT_PAIRS, 0x0010_fadc)?;
                c.g[V0][0] = c.lw(race_pos_at(c, S3, AI_COUNT));
                c.g[S0][0] = 0;
                if (c.g[V0][0] as i64) <= 0 {
                    break 'done;
                }
                // Pass 2: s0 = new position, s1 = the rider array.
                c.cycles += 2;
                c.g[S1][0] = sx(race_pos_at(c, S3, AI_RIDERS));
                c.g[V1][0] = sx(r32(c, S0) << 2);
                loop {
                    // 0x10faf0
                    c.cycles += 9;
                    c.g[S2][0] = sx(sp(c).wrapping_add(r32(c, V1)));
                    c.g[S7][0] = c.g[V1][0];
                    c.g[V0][0] = c.lw(r32(c, S2));
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
                    c.g[V1][0] = c.lw(r32(c, V0));
                    c.g[A0][0] = c.lw(race_pos_at(c, V1, RIDER_POSITION));
                    let unchanged = c.g[A0][0] == c.g[S0][0];
                    c.g[S6][0] = sx(r32(c, S0).wrapping_add(1));
                    if !unchanged {
                        // s4 = the rider ahead, s5 = the rider behind.
                        c.cycles += 3;
                        c.g[S5][0] = 0;
                        c.g[S4][0] = 0;
                        if (c.g[S0][0] as i64) > 0 {
                            c.cycles += 7;
                            c.g[V0][0] = sx(r32(c, S0).wrapping_sub(1));
                            c.g[V0][0] = sx(r32(c, V0) << 2);
                            c.g[A0][0] = sx(sp(c).wrapping_add(r32(c, V0)));
                            c.g[V1][0] = c.lw(r32(c, A0));
                            c.g[V1][0] = sx(r32(c, V1) << 2);
                            c.g[V1][0] = sx(r32(c, S1).wrapping_add(r32(c, V1)));
                            c.g[S4][0] = c.lw(r32(c, V1));
                        }
                        c.cycles += 5;
                        c.g[V0][0] = c.lw(race_pos_at(c, S3, AI_COUNT));
                        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                        let not_last = (c.g[S0][0] as i64) < (c.g[V0][0] as i64);
                        c.g[V0][0] = sx(r32(c, S6) << 2);
                        if not_last {
                            c.cycles += 5;
                            c.g[A0][0] = sx(sp(c).wrapping_add(r32(c, V0)));
                            c.g[V1][0] = c.lw(r32(c, A0));
                            c.g[V1][0] = sx(r32(c, V1) << 2);
                            c.g[V1][0] = sx(r32(c, S1).wrapping_add(r32(c, V1)));
                            c.g[S5][0] = c.lw(r32(c, V1));
                        }
                        c.cycles += 12;
                        call_ptr(c, AUDIO_GET_INSTANCE, 0x0010_fb6c)?;
                        c.g[V1][0] = c.lw(r32(c, S2));
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A2][0] = c.g[S5][0];
                        c.g[A3][0] = c.g[S4][0];
                        c.g[V1][0] = sx(r32(c, V1) << 2);
                        c.g[T1][0] = c.g[S0][0];
                        c.g[V1][0] = sx(r32(c, S1).wrapping_add(r32(c, V1)));
                        c.g[A1][0] = c.lw(r32(c, V1));
                        c.g[T0][0] = c.lw(race_pos_at(c, A1, RIDER_POSITION));
                        call_ptr(c, AUDIO_ON_POSITION_CHANGE, 0x0010_fb94)?;
                    }
                    // 0x10fb94: riders[indices[s0]]->position = s0.
                    c.cycles += 11;
                    c.g[V1][0] = sx(sp(c).wrapping_add(r32(c, S7)));
                    c.g[V0][0] = c.lw(r32(c, V1));
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
                    c.g[V1][0] = c.lw(r32(c, V0));
                    c.sw(race_pos_at(c, V1, RIDER_POSITION), c.g[S0][0]);
                    c.g[S0][0] = c.g[S6][0];
                    c.g[V0][0] = c.lw(race_pos_at(c, S3, AI_COUNT));
                    let more = (c.g[S0][0] as i64) < (c.g[V0][0] as i64);
                    c.g[V0][0] = more as u64;
                    c.g[V1][0] = sx(r32(c, S0) << 2);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0010_faf0);
                    }
                }
            }
            // 0x10fbc0
            c.cycles += 1;
            frame.restore(c, S0, 0xd0);
        }
        c.cycles += 11;
        for (reg, off) in [
            (S1, 0xc0),
            (S2, 0xb0),
            (S3, 0xa0),
            (S4, 0x90),
            (S5, 0x80),
            (S6, 0x70),
            (S7, 0x60),
            (FP, 0x50),
        ] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x2f5148 WorldLightMan_LightInstanceCB

/// `WorldLightMan_GetInstanceLighting(self, inst, force)` (0x2f5400).
const LIGHT_INSTANCE_GET_LIGHTING: Callee = Callee {
    addr: 0x002f_5400,
    f: ee::s09::WorldLightMan_GetInstanceLighting,
};

/// `void WorldLightMan_LightInstanceCB(Instance* inst)` (0x2f5148): the
/// world's per-instance lighting callback: looks up the instance's lighting
/// in the game module's light manager (`world->game_module->light_man`,
/// +0x84 then +0x90), forcing a re-light check when bit 12 of the
/// instance's flags (+8) is set.
pub fn world_light_man_light_instance_cb(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        let world_slot = r32(c, GP).wrapping_sub(0x848);
        c.g[V1][0] = c.lw(world_slot);
        frame.save_ra(c, 0);
        let inst = r32(c, A0);
        c.g[A1][0] = c.g[A0][0];
        c.g[A2][0] = c.lw(inst.wrapping_add(0x8));
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x84));
        c.g[A2][0] = sx((r32(c, A2) as i32 >> 12) as u32);
        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0x90));
        c.g[A2][0] &= 1;
        LIGHT_INSTANCE_GET_LIGHTING.call(c, 0x002f_5170)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x1f3188 LoadIndicator_Update

/// `gp`-relative `.lit4` entry with the per-tick step (1/60 s).
const LOAD_INDICATOR_LIT_STEP: u32 = 0x555c;

/// `void LoadIndicator_Update(LoadIndicator* self)` (0x1f3188): while the
/// indicator is active (+4), advance its animation clock (+8) by one tick
/// and wrap it into (0, 2] by subtracting 2 while it exceeds 2.
pub fn load_indicator_update(c: &mut Cpu) {
    const ACTIVE: u32 = 0x4;
    const CLOCK: u32 = 0x8;
    c.cycles += 3;
    let this = r32(c, A0);
    c.g[V0][0] = c.lw(this.wrapping_add(ACTIVE));
    let active = c.g[V0][0] != 0;
    c.g[V0][0] = sx(this.wrapping_add(CLOCK));
    if active {
        c.cycles += 9;
        c.lwc1(1, this.wrapping_add(CLOCK));
        c.lwc1(0, r32(c, GP).wrapping_sub(LOAD_INDICATOR_LIT_STEP));
        c.g[AT][0] = 0x4000_0000; // 2.0
        c.mtc1(AT, 2);
        c.add_s(1, 1, 0);
        c.c_lt_s(2, 1);
        let over = c.fcc();
        c.swc1(1, this.wrapping_add(CLOCK));
        if over {
            c.cycles += 2;
            c.sub_s(1, 1, 2);
            loop {
                // 0x1f31c0
                c.cycles += 4;
                c.c_lt_s(2, 1);
                if !c.fcc() {
                    break;
                }
                c.sub_s(1, 1, 2);
                if event_due(c) {
                    deliver_event_here(c, 0x001f_31c0);
                }
            }
            c.cycles += 1;
            c.swc1(1, r32(c, V0));
        }
    }
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// 0x13f8f8 ObjPtrArray_MergeSort

/// The sort key of an object in [`obj_ptr_array_merge_sort`] (unsigned).
const MERGE_SORT_KEY: u32 = 0x78;

/// `ObjPtrArray_MergeSort` itself (the recursion).
const MERGE_SORT_SELF: Callee = Callee {
    addr: 0x0013_f8f8,
    f: ee::s01::ObjPtrArray_MergeSort,
};

/// `void ObjPtrArray_MergeSort(Obj** first, Obj** last, Obj** buf, cmp)`
/// (0x13f8f8): stable merge sort of an object-pointer array by the
/// unsigned key at +0x78, ascending, into `buf` (which must hold `n`
/// pointers). Two elements are ordered directly; otherwise the first half
/// (`(n + 1) / 2`) is sorted into the upper half of `buf`, the second half
/// into the start of the source, and both are merged into `buf`. `cmp` is
/// only passed along.
pub fn obj_ptr_array_merge_sort(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = 2;
        frame.save(c, S1, 0x80);
        frame.save(c, S7, 0x20);
        c.g[S1][0] = c.g[A0][0];
        c.g[S7][0] = c.g[A1][0];
        frame.save(c, S0, 0x90);
        frame.save(c, S3, 0x60);
        c.g[V1][0] = sx(r32(c, S7).wrapping_sub(r32(c, S1)));
        frame.save(c, S6, 0x30);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S2, 0x70);
        c.g[S0][0] = sx((r32(c, V1) as i32 >> 2) as u32);
        frame.save(c, S4, 0x50);
        c.g[S6][0] = c.g[A3][0];
        frame.save(c, S5, 0x40);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);
        let first = r32(c, S1);
        let buf = r32(c, S3);
        if c.g[S0][0] == c.g[V0][0] {
            // Two elements: order them directly into buf.
            c.cycles += 7;
            c.g[A1][0] = c.lw(first);
            c.g[A0][0] = c.lw(first.wrapping_add(4));
            c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(MERGE_SORT_KEY));
            c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(MERGE_SORT_KEY));
            c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
            if c.g[V0][0] == 0 {
                c.sw(buf, c.g[A0][0]);
                c.cycles += 3;
                c.g[V0][0] = c.lw(first);
                c.sw(buf.wrapping_add(4), c.g[V0][0]);
            } else {
                c.cycles += 4;
                c.sw(buf, c.g[A1][0]);
                c.g[V0][0] = c.lw(first.wrapping_add(4));
                c.sw(buf.wrapping_add(4), c.g[V0][0]);
            }
        } else {
            // s2 = mid = first + (n + 1) / 2, s4 = buf + n / 2, s5 = buf + n.
            c.cycles += 16;
            let n = r32(c, S0);
            c.g[V0][0] = sx(n.wrapping_add(1));
            c.g[V1][0] = sx(r32(c, V1) >> 31);
            c.g[FP][0] = c.g[V0][0];
            c.g[A0][0] = sx(r32(c, V0) >> 31);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.g[V1][0] = sx(n.wrapping_add(r32(c, V1)));
            c.g[V0][0] = sx((r32(c, V0) as i32 >> 1) as u32);
            c.g[V1][0] = sx((r32(c, V1) as i32 >> 1) as u32);
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[S2][0] = sx(first.wrapping_add(r32(c, V0)));
            c.g[S4][0] = sx(buf.wrapping_add(r32(c, V1)));
            c.g[V0][0] = sx(n << 2);
            let small = (c.g[S0][0] as i64) < 3;
            c.g[V1][0] = small as u64;
            c.g[S5][0] = sx(buf.wrapping_add(r32(c, V0)));
            if small {
                c.cycles += 3;
                c.g[V0][0] = c.lw(first);
                c.sw(r32(c, S4), c.g[V0][0]);
            } else {
                // First half into buf's upper half.
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S4][0];
                c.g[A3][0] = c.g[S6][0];
                MERGE_SORT_SELF.call(c, 0x0013_f9dc)?;
            }
            c.cycles += 3;
            let small = (c.g[S0][0] as i64) < 4;
            c.g[V0][0] = small as u64;
            c.g[A0][0] = c.g[S2][0];
            if small {
                c.cycles += 3;
                c.g[V0][0] = c.lw(r32(c, S2));
                c.sw(first, c.g[V0][0]);
            } else {
                // Second half into the start of the source.
                c.cycles += 4;
                c.g[A1][0] = c.g[S7][0];
                c.g[A3][0] = c.g[S6][0];
                c.g[A2][0] = c.g[S1][0];
                MERGE_SORT_SELF.call(c, 0x0013_fa04)?;
            }
            // Merge: a0..t1 (the source's n / 2), a3..t2 (buf's
            // (n + 1) / 2), out a2..s5.
            c.cycles += 15;
            c.g[V0][0] = sx(r32(c, FP) >> 31);
            c.g[V1][0] = sx(r32(c, S0) >> 31);
            c.g[V0][0] = sx(r32(c, FP).wrapping_add(r32(c, V0)));
            c.g[V1][0] = sx(r32(c, S0).wrapping_add(r32(c, V1)));
            c.g[V0][0] = sx((r32(c, V0) as i32 >> 1) as u32);
            c.g[V1][0] = sx((r32(c, V1) as i32 >> 1) as u32);
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[A3][0] = c.g[S4][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[A2][0] = c.g[S3][0];
            c.g[T2][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
            let any = c.g[A2][0] < c.g[S5][0];
            c.g[V0][0] = any as u64;
            c.g[T1][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
            if any {
                c.cycles += 2;
                c.g[V0][0] = (c.g[A0][0] < c.g[T1][0]) as u64;
                loop {
                    // 0x13fa48
                    c.cycles += 2;
                    let a_left = c.g[V0][0] != 0;
                    c.g[V0][0] = (c.g[A3][0] < c.g[T2][0]) as u64;
                    if a_left {
                        c.cycles += 2;
                        let b_left = c.g[V0][0] != 0;
                        c.g[T0][0] = c.lw(r32(c, A0));
                        let mut took_b = false;
                        if b_left {
                            c.cycles += 6;
                            c.g[A1][0] = c.lw(r32(c, A3));
                            c.g[V1][0] = c.lw(r32(c, T0).wrapping_add(MERGE_SORT_KEY));
                            c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(MERGE_SORT_KEY));
                            c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                            if c.g[V0][0] == 0 {
                                c.sw(r32(c, A2), c.g[T0][0]);
                            } else {
                                c.cycles += 3;
                                c.sw(r32(c, A2), c.g[A1][0]);
                                c.g[A3][0] = sx(r32(c, A3).wrapping_add(4));
                                took_b = true;
                            }
                        } else {
                            c.cycles += 1;
                            c.sw(r32(c, A2), c.g[T0][0]);
                        }
                        if !took_b {
                            c.cycles += 2;
                            c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
                        }
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(r32(c, A3));
                        c.g[A3][0] = sx(r32(c, A3).wrapping_add(4));
                        c.sw(r32(c, A2), c.g[V0][0]);
                    }
                    c.cycles += 4;
                    c.g[A2][0] = sx(r32(c, A2).wrapping_add(4));
                    let more = c.g[A2][0] < c.g[S5][0];
                    c.g[V0][0] = (c.g[A0][0] < c.g[T1][0]) as u64;
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0013_fa48);
                    }
                }
            }
        }
        c.cycles += 12;
        for (reg, off) in [
            (S0, 0x90),
            (S1, 0x80),
            (S2, 0x70),
            (S3, 0x60),
            (S4, 0x50),
            (S5, 0x40),
            (S6, 0x30),
            (S7, 0x20),
            (FP, 0x10),
        ] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x386640 PS2GraphicsMan_v115

const GFX_RESET_DEPTH_FOG: Callee = Callee {
    addr: 0x0036_aba0,
    f: ee::s12::PS2GraphicsMan_ResetDepthFogParams,
};
const GFX_RESET_COLOR_FILTER: Callee = Callee {
    addr: 0x0039_0458,
    f: ee::s12::PS2GraphicsMan_ResetColorFilter,
};
const GFX_RESET_BLOOM: Callee = Callee {
    addr: 0x0036_c740,
    f: ee::s12::PS2GraphicsMan_ResetBloomParams,
};

/// `void PS2GraphicsMan::v115(int view)` (0x386640, a vtable slot): reset
/// view `view`'s screen effects to their defaults: depth fog, colour
/// filter, then bloom parameters.
pub fn ps2_graphics_man_reset_view_effects(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        GFX_RESET_DEPTH_FOG.call(c, 0x0038_665c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        GFX_RESET_COLOR_FILTER.call(c, 0x0038_6668)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        GFX_RESET_BLOOM.call(c, 0x0038_6674)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x2ee978 WorldPainter_GetWeather98, 0x2ee5b8 WorldPainter_GetWeather38

/// The world painters' weather getters: `entry = &D_004FA370[index]`
/// (0xf0 bytes each), `obj = *entry->weather` (+0x20), call the weather
/// object's virtual method at v-table `vt_off` (`{delta: i16, fn}`, `this +
/// delta`) through the function table and return the float it points at.
fn world_painter_weather_float_getter(c: &mut Cpu, vt_off: u32, ret_addr: u32) {
    const WEATHER_SLOT: u32 = 0x20;
    guest(c, |c| {
        c.cycles += 18;
        c.g[V1][0] = 0xf0;
        let frame = Frame::open(c, 0x10);
        c.mult(0, A0, A0, V1); // a0 = index * 0xf0
        c.g[V0][0] = 0x50_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x50_0000u32.wrapping_sub(0x5c90));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
        c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(WEATHER_SLOT));
        c.g[A2][0] = c.lw(r32(c, V1));
        c.g[A1][0] = c.lw(r32(c, A2).wrapping_add(4));
        c.g[A0][0] = c.lh(r32(c, A1).wrapping_add(vt_off));
        c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(vt_off + 4));
        let method = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
        call_ptr(c, method, ret_addr)?;
        c.lwc1(0, r32(c, V0));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float WorldPainter_GetWeather98(int index)` (0x2ee978): weather
/// method at v-table +0x1e8.
pub fn world_painter_get_weather98(c: &mut Cpu) {
    world_painter_weather_float_getter(c, 0x1e8, 0x002e_e9b0);
}

/// `float WorldPainter_GetWeather38(int index)` (0x2ee5b8): weather
/// method at v-table +0x188.
pub fn world_painter_get_weather38(c: &mut Cpu) {
    world_painter_weather_float_getter(c, 0x188, 0x002e_e5f0);
}

// ---------------------------------------------------------------------------
// 0x359698 SplineModifier_Update

/// Fields of a spline modifier (an object moving along a spline).
mod spline_mod {
    /// End behaviour: 0 stop, 2 bounce (ping-pong), 4 stop, else wrap.
    pub const MODE: u32 = 0x30;
    /// Distance along the spline.
    pub const POS: u32 = 0x3c;
    /// Acceleration while the acceleration time runs.
    pub const ACCEL: u32 = 0x40;
    /// Remaining acceleration time (seconds).
    pub const ACCEL_TIME: u32 = 0x44;
    /// Speed (distance per second).
    pub const SPEED: u32 = 0x48;
    /// Stopped flag.
    pub const STOPPED: u32 = 0x50;
    /// Active flag.
    pub const ACTIVE: u32 = 0x54;
    /// Transform needs an update.
    pub const DIRTY: u32 = 0x58;
    /// The spline's length.
    pub const LENGTH: u32 = 0xe4;
    /// `gp`-relative literals: the 0.1 margin below the end (modes 0, 2, 4).
    pub const LIT_END_MARGIN_STOP: u32 = 0x2a5c;
    pub const LIT_END_MARGIN_BOUNCE: u32 = 0x2a58;
    pub const LIT_END_MARGIN_STOP4: u32 = 0x2a54;
    /// `World_GetSecondsPerTick()` (f0, cross-shard).
    pub const SECONDS_PER_TICK: u32 = 0x002d_1c70;
}

/// `void SplineModifier_Update(SplineModifier* self)` (0x359698): spend
/// one tick of the acceleration time (speeding up by `accel * dt`); then,
/// if active and not stopped, advance the position by `speed * dt` and
/// handle running off either end by the mode (stop and reverse, bounce,
/// or wrap around), and mark the transform dirty.
pub fn spline_modifier_update(c: &mut Cpu) {
    use spline_mod::*;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_fpr(c, 20, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = r32(c, S0);
        let gp = r32(c, GP);
        c.mtc1(0, 20); // f20 = 0.0
        c.lwc1(0, this + ACCEL_TIME);
        c.c_lt_s(20, 0);
        if c.fcc() {
            // Accelerating: accel_time -= dt; speed += accel * dt.
            c.cycles += 12;
            call_ptr(c, SECONDS_PER_TICK, 0x0035_96cc)?;
            c.lwc1(1, this + ACCEL_TIME);
            c.sub_s(1, 1, 0);
            c.swc1(1, this + ACCEL_TIME);
            call_ptr(c, SECONDS_PER_TICK, 0x0035_96dc)?;
            c.lwc1(1, this + ACCEL);
            c.lwc1(2, this + SPEED);
            c.mul_s(1, 1, 0);
            c.add_s(2, 2, 1);
            c.swc1(2, this + SPEED);
        }
        c.g[V0][0] = c.lw(this + ACTIVE);
        'out: {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                break 'out;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw(this + STOPPED);
            if c.g[V0][0] != 0 {
                break 'out;
            }
            // pos += speed * dt (f3 speed, f2 the new position).
            c.cycles += 10;
            call_ptr(c, SECONDS_PER_TICK, 0x0035_9710)?;
            c.lwc1(3, this + SPEED);
            c.lwc1(1, this + POS);
            c.mul_s(0, 3, 0);
            c.add_s(2, 1, 0);
            c.c_lt_s(2, 20);
            let before_start = c.fcc();
            c.swc1(2, this + POS);
            // `stop`: stop at the start, reversed; `store_pos`: f0 holds the
            // position to store.
            let mut stop = false;
            let mut store_pos = false;
            if before_start {
                c.cycles += 3;
                c.g[V1][0] = c.lw(this + MODE);
                c.g[V0][0] = 2;
                if c.g[V1][0] == 0 {
                    stop = true;
                } else {
                    c.cycles += 2;
                    let bounce = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 4;
                    if bounce {
                        // Bounce: speed = -speed, pos = -pos.
                        c.cycles += 5;
                        c.neg_s(0, 3);
                        c.neg_s(1, 2);
                        c.swc1(0, this + SPEED);
                        c.swc1(1, this + POS);
                    } else {
                        c.cycles += 2;
                        if c.g[V1][0] != c.g[V0][0] {
                            // Wrap: pos += length.
                            c.lwc1(0, this + LENGTH);
                            c.cycles += 2;
                            c.add_s(0, 2, 0);
                            store_pos = true;
                        } else {
                            stop = true;
                        }
                    }
                }
                if stop {
                    // Stop at the start, reversed.
                    c.cycles += 7;
                    c.neg_s(0, 3);
                    c.g[V0][0] = 1;
                    c.swc1(20, this + POS);
                    c.sw(this + STOPPED, c.g[V0][0]);
                    c.swc1(0, this + SPEED);
                    c.sw(this + ACTIVE, 0);
                }
            } else {
                c.cycles += 5;
                c.lwc1(4, this + LENGTH);
                c.c_le_s(4, 2);
                let past_end = c.fcc();
                c.g[V0][0] = 1;
                if past_end {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(this + MODE);
                    c.g[V0][0] = 2;
                    store_pos = true;
                    if c.g[V1][0] == 0 {
                        // Stop just before the end, reversed.
                        c.cycles += 8;
                        c.lwc1(0, gp.wrapping_sub(LIT_END_MARGIN_STOP));
                        c.neg_s(1, 3);
                        c.g[V0][0] = 1;
                        c.sw(this + ACTIVE, 0);
                        c.sub_s(0, 4, 0);
                        c.sw(this + STOPPED, c.g[V0][0]);
                        c.swc1(1, this + SPEED);
                    } else {
                        c.cycles += 2;
                        let bounce = c.g[V1][0] == c.g[V0][0];
                        c.g[V0][0] = 4;
                        if bounce {
                            // Bounce: speed = -speed, pos -= pos - (length - 0.1).
                            c.cycles += 7;
                            c.lwc1(0, gp.wrapping_sub(LIT_END_MARGIN_BOUNCE));
                            c.neg_s(1, 3);
                            c.sub_s(0, 4, 0);
                            c.swc1(1, this + SPEED);
                            c.sub_s(0, 2, 0);
                            c.sub_s(0, 2, 0);
                        } else {
                            c.cycles += 2;
                            if c.g[V1][0] != c.g[V0][0] {
                                // Wrap: pos -= length.
                                c.sub_s(0, 2, 4);
                            } else {
                                // Mode 4: stop just before the end, reversed.
                                c.cycles += 7;
                                c.lwc1(0, gp.wrapping_sub(LIT_END_MARGIN_STOP4));
                                c.neg_s(1, 3);
                                c.g[V0][0] = 1;
                                c.sw(this + ACTIVE, 0);
                                c.sub_s(0, 4, 0);
                                c.sw(this + STOPPED, c.g[V0][0]);
                                c.swc1(1, this + SPEED);
                            }
                        }
                    }
                }
            }
            if store_pos {
                c.cycles += 1;
                c.swc1(0, this + POS);
            }
            if store_pos || (before_start && !stop) {
                c.cycles += 1;
                c.g[V0][0] = 1;
            }
            c.cycles += 2;
            c.sw(this + DIRTY, c.g[V0][0]);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x3d62c8 PATH_TrackSetVolume

/// PATH track fields used by [`path_track_set_volume`].
mod path_track {
    /// Track volume (signed byte, percent).
    pub const VOLUME: u32 = 0x4;
    /// Status byte: negative while the track is playing.
    pub const STATUS: u32 = 0xe;
    /// The track's own volume scale (signed byte, percent).
    pub const SCALE: u32 = 0x34;
    /// The platform voice index.
    pub const PLAT: u32 = 0x54;
    /// `PATH_PlatSetVolume(plat, volume)` (cross-shard).
    pub const PLAT_SET_VOLUME: u32 = 0x002a_f6c0;
}

/// `void PATH_TrackSetVolume(PathTrack* track, int volume)` (0x3d62c8):
/// store the volume (as a signed byte) and, while the track is playing
/// (status byte negative), apply `scale * volume / 100` to its platform
/// voice.
pub fn path_track_set_volume(c: &mut Cpu) {
    use path_track::*;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx(r32(c, A1) << 24);
        frame.save_ra(c, 0);
        c.g[V1][0] = sx((r32(c, A1) as i32 >> 24) as u32);
        let track = r32(c, A0);
        c.sb(track.wrapping_add(VOLUME), c.g[V1][0]);
        c.g[V0][0] = c.lb(track.wrapping_add(STATUS));
        frame.restore_ra(c, 0);
        if (c.g[V0][0] as i64) < 0 {
            c.cycles += 4;
            c.g[A1][0] = c.lb(track.wrapping_add(SCALE));
            c.g[V0][0] = 100;
            c.cycles += 6;
            c.mult(0, A1, A1, V1);
            c.div(0, A1, V0);
            c.g[A1][0] = c.lo[0];
            c.g[A0][0] = c.lw(track.wrapping_add(PLAT));
            call_ptr(c, PLAT_SET_VOLUME, 0x003d_630c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x396128 TexMan_InvalidateBound

/// `void TexMan_InvalidateBound(TexMan* self)` (0x396128): forget which
/// textures are bound: both bound-texture slots (+0x1f4c, +0x1f50, cleared
/// from the top down) and the bound palette (+0x1f54) become -1.
pub fn tex_man_invalidate_bound(c: &mut Cpu) {
    const BOUND_SLOTS: u32 = 0x1f4c;
    const BOUND_PALETTE: u32 = 0x1f54;
    let this = r32(c, A0);
    c.cycles += 4;
    c.g[A1][0] = u64::MAX;
    c.g[V1][0] = 1;
    c.g[V0][0] = sx(this.wrapping_add(BOUND_SLOTS + 4));
    loop {
        // 0x396138
        c.cycles += 7;
        c.sw(r32(c, V0), c.g[A1][0]);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0039_6138);
        }
    }
    c.cycles += 3;
    c.g[V0][0] = u64::MAX;
    c.sw(this.wrapping_add(BOUND_PALETTE), c.g[V0][0]);
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// AI_UpdateRacePositions over 0..8 riders: rank mode 0 (early out),
    /// 1 (distance), 2 (score) or another value (keys left as stack
    /// junk); some riders not loaded (early out), finished or not; the
    /// stored positions sometimes already equal the new rank. The audio
    /// notification runs for real up to BELibrary_GetInterface, whose
    /// interface call is a fake (which may unwind), and stops at the game
    /// module's race-type check. Events are delivered at both loop heads.
    #[test]
    fn ai_update_race_positions_matches() {
        use crate::difftest::events::{check_events, schedule, Events};
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{Rng, GAME_GP};
        use race_positions::*;
        const AI: u32 = 0x00a0_0000;
        const RIDER0: u32 = 0x00a1_0000;
        const SCORE0: u32 = 0x00a2_0000;
        const IFACE: u32 = 0x00a3_0000;
        const IFACE_VT: u32 = 0x00a3_0100;
        const BELIB: u32 = 0x00a3_0200;
        const WORLD: u32 = 0x00a3_0400;
        const GAME_MODULE: u32 = 0x00a3_0800;
        const AUDIO: u32 = 0x00a3_1000;
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r: Rng = machine(c, seed, 0x0f99_8000, 6);
            let mode = if seed % 9 == 0 {
                0
            } else {
                [1, 1, 2, 2, 3][(r.next_u32() % 5) as usize]
            };
            let count = r.next_u32() % 9;
            c.bus.write32(AI + AI_RANK_MODE, mode);
            c.bus.write32(AI + AI_COUNT, count);
            let not_loaded = if seed % 11 == 5 { r.next_u32() % 8 } else { 99 };
            for i in 0..8u32 {
                let rider = RIDER0 + 0x1000 * i;
                let score = SCORE0 + 0x400 * i;
                c.bus.write32(AI + AI_RIDERS + 4 * i, rider);
                c.bus.write32(rider + 0x880, if i == not_loaded { 3 } else { 7 });
                c.bus.write32(
                    rider + RIDER_FINISHED,
                    if r.next_u32() % 3 == 0 { 1 + r.next_u32() % 5 } else { 0 },
                );
                c.bus.write32(rider + RIDER_RACE_TIME, r.f32(20.0));
                c.bus.write32(rider + 0x4d0, r.f32(3000.0));
                let pos = if r.next_u32() % 2 == 0 { i } else { r.next_u32() % 8 };
                c.bus.write32(rider + RIDER_POSITION, pos);
                c.bus.write32(rider + RIDER_SCORE_OBJ, score);
                c.bus.write32(score + SCORE_POINTS, r.next_u32() % 1000);
            }
            // SSXAudioSystem_GetInstance and OnRacePositionChange's path.
            c.bus.write32(GAME_GP + 0x410, AUDIO);
            c.bus.write32(GAME_GP - 0x1f04, BELIB);
            c.bus.write32(GAME_GP - 0x1efc, 0);
            c.bus.write32(0x0053_08a8, IFACE);
            c.bus.write32(IFACE + 4, IFACE_VT);
            c.bus.write16(IFACE_VT + 8, 0);
            c.bus.write32(IFACE_VT + 0xc, FAKE);
            c.bus.write8(0x0053_5c10, 0);
            c.bus.write32(GAME_GP - 0x848, WORLD);
            c.bus.write32(WORLD + 0x84, GAME_MODULE);
            c.bus.write32(GAME_MODULE + 0x214, 0);
            c.g[A0][0] = AI as u64;
            schedule(c, &mut r, seed, 0x0010_f998)
        };
        check_events(ee::s00::f_0010f998__generated, ai_update_race_positions, 300, &setup);
    }

    /// WorldLightMan_LightInstanceCB: the world, its game module and light
    /// manager; an instance whose lighting node is cached at the head of
    /// its hash chain, with no model and the node's own position (so the
    /// real GetInstanceLighting takes its short path whether or not the
    /// force bit (flags bit 12) is set). Flags are random otherwise.
    #[test]
    fn world_light_man_light_instance_cb_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{put_vec, GAME_GP};
        const WORLD: u32 = 0x00a0_0000;
        const GAME_MODULE: u32 = 0x00a0_1000;
        const LIGHT_MAN: u32 = 0x00a1_0000;
        const NODE: u32 = 0x00a2_0000;
        const FRAME_STATE: u32 = 0x00a3_0000;
        const INST: u32 = 0x00a4_0000;
        crate::difftest::check(
            ee::s09::f_002f5148__generated,
            world_light_man_light_instance_cb,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f51_4800, 0);
                c.pc = 0x002f_5148;
                c.bus.write32(GAME_GP - 0x848, WORLD);
                c.bus.write32(WORLD + 0x84, GAME_MODULE);
                c.bus.write32(GAME_MODULE + 0x90, LIGHT_MAN);
                c.bus.write32(GAME_GP + 0x2a74, FRAME_STATE);
                c.bus.write32(FRAME_STATE + 0x18, 100 + seed % 50);
                let key = r.next_u32();
                let slot = (key >> 8) & 0x1f;
                c.bus.write32(LIGHT_MAN + 0x3210 + 4 * slot, NODE);
                c.bus.write32(NODE, key);
                let pos = [r.f32(500.0), r.f32(500.0), r.f32(500.0), r.f32(1.0)];
                put_vec(c, NODE + 0xb0, pos);
                let flags = r.next_u32();
                let flags = if seed % 2 == 0 { flags | 0x1000 } else { flags & !0x1000 };
                c.bus.write32(INST + 0x8, flags);
                c.bus.write32(INST + 0xc, 0);
                put_vec(c, INST + 0x40, pos);
                c.bus.write32(INST + 0x78, key);
                c.g[A0][0] = INST as u64;
            },
        );
    }

    /// LoadIndicator_Update: inactive on one seed in four; the clock
    /// anywhere in [-3, 57) (no wrap, one or many subtractions), the step
    /// literal 1/60 or a random small value. Events are delivered in the
    /// wrap loop.
    #[test]
    fn load_indicator_update_matches() {
        use crate::difftest::events::{check_events, schedule, Events};
        use crate::difftest::fakes::machine;
        use crate::difftest::{DATA, GAME_GP};
        const SELF: u32 = DATA + 0x2_0000;
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 0x1f31_8800, 0);
            let active = if seed % 4 == 0 { 0 } else { 1 + r.next_u32() % 7 };
            c.bus.write32(SELF + 4, active);
            let clock = -3.0 + (r.next_u32() % 60_000) as f32 / 1000.0;
            c.bus.write32(SELF + 8, clock.to_bits());
            let step = if seed % 3 == 0 {
                r.f32(1.0)
            } else {
                (1.0f32 / 60.0).to_bits()
            };
            c.bus.write32(GAME_GP - LOAD_INDICATOR_LIT_STEP, step);
            c.g[A0][0] = SELF as u64;
            schedule(c, &mut r, seed, 0x001f_3188)
        };
        check_events(ee::s04::f_001f3188__generated, load_indicator_update, 300, &setup);
    }

    /// ObjPtrArray_MergeSort on -3..=24 pointers (two elements, the small
    /// halves, deep recursion, empty/negative counts), objects with keys
    /// from a small range (ties keep their order) or full 32-bit values
    /// (unsigned compare); the buffer after the array. Events are delivered
    /// in the merge loops (at any recursion depth).
    #[test]
    fn obj_ptr_array_merge_sort_matches() {
        use crate::difftest::events::{check_events, schedule, Events};
        use crate::difftest::fakes::machine;
        use crate::difftest::DATA;
        const ARRAY: u32 = DATA + 0x1_0000;
        const BUF: u32 = DATA + 0x1_1000;
        const OBJS: u32 = DATA + 0x2_0000;
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 0x13f8_f800, 0);
            let n: i32 = match seed % 10 {
                0 => 2,
                1 => (r.next_u32() % 4) as i32 - 3,
                _ => (r.next_u32() % 25) as i32,
            };
            let small_keys = seed % 3 != 0;
            for i in 0..32u32 {
                let obj = OBJS + 0x100 * i;
                c.bus.write32(ARRAY - 0x40 + 4 * i, OBJS + 0x100 * (r.next_u32() % 32));
                c.bus.write32(ARRAY + 4 * i, obj);
                c.bus.write32(BUF - 0x40 + 4 * i, r.next_u32());
                c.bus.write32(BUF + 4 * i, OBJS + 0x100 * (r.next_u32() % 32));
                let key = if small_keys { r.next_u32() % 6 } else { r.next_u32() };
                c.bus.write32(obj + MERGE_SORT_KEY, key);
            }
            c.g[A0][0] = ARRAY as u64;
            c.g[A1][0] = sx(ARRAY.wrapping_add((4 * n) as u32));
            c.g[A2][0] = BUF as u64;
            schedule(c, &mut r, seed, 0x0013_f8f8)
        };
        check_events(ee::s01::f_0013f8f8__generated, obj_ptr_array_merge_sort, 300, &setup);
    }

    /// PS2GraphicsMan_v115: the graphics manager at DATA, view 0..3, the
    /// three default tables (gp-relative literals and the depth-fog colour
    /// at 0x504720) random; the three real leaf resets run.
    #[test]
    fn ps2_graphics_man_reset_view_effects_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{fill, DATA, GAME_GP};
        crate::difftest::check(
            ee::s12::f_00386640__generated,
            ps2_graphics_man_reset_view_effects,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0038_6640, 0);
                c.pc = 0x0038_6640;
                fill(c, &mut r, GAME_GP + 0x1230, 0x40);
                fill(c, &mut r, GAME_GP + 0x14f0, 0x8);
                fill(c, &mut r, 0x0050_4720, 4);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (r.next_u32() % 4) as u64;
            },
        );
    }

    /// The weather-getter fixture: painter index 0..3, its weather holder,
    /// object and v-table at random pointers, the slot `{delta, FAKE}`;
    /// the fake (which may unwind) returns a pointer to random data.
    fn world_painter_weather_setup(c: &mut Cpu, seed: u32, entry_pc: u32, vt_off: u32) {
        use crate::difftest::fakes::{machine, ptr, FAKE};
        let mut r = machine(c, seed, entry_pc as u64, 1);
        c.pc = entry_pc;
        let index = seed % 4;
        c.g[A0][0] = index as u64;
        let entry = 0x004f_a370u32.wrapping_add(index * 0xf0);
        let holder = ptr(&mut r);
        let obj = ptr(&mut r);
        let vt = ptr(&mut r);
        c.bus.write32(entry + 0x20, holder);
        c.bus.write32(holder, obj);
        c.bus.write32(obj + 4, vt);
        let delta = (r.next_u32() as i16 as i32 & !3) as u32;
        c.bus.write16(vt + vt_off, delta as u16);
        c.bus.write32(vt + vt_off + 4, FAKE);
    }

    /// WorldPainter_GetWeather98.
    #[test]
    fn world_painter_get_weather98_matches() {
        crate::difftest::fakes::covered(
            ee::s09::f_002ee978__generated,
            world_painter_get_weather98,
            200,
            &|c, seed| world_painter_weather_setup(c, seed, 0x002e_e978, 0x1e8),
        );
    }

    /// WorldPainter_GetWeather38.
    #[test]
    fn world_painter_get_weather38_matches() {
        crate::difftest::fakes::covered(
            ee::s09::f_002ee5b8__generated,
            world_painter_get_weather38,
            200,
            &|c, seed| world_painter_weather_setup(c, seed, 0x002e_e5b8, 0x188),
        );
    }

    /// SplineModifier_Update: acceleration time positive or not, active /
    /// stopped flags (one seed in five each), modes 0..4, a speed and
    /// position that leave the spline at the start, at the end, or stay
    /// inside; the world's seconds-per-tick (1/60 or random) and the 0.1
    /// literals.
    #[test]
    fn spline_modifier_update_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{DATA, GAME_GP};
        use spline_mod::*;
        const SELF: u32 = DATA + 0x3_0000;
        const WORLD: u32 = DATA + 0x3_1000;
        crate::difftest::check(ee::s11::f_00359698__generated, spline_modifier_update, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x0035_9698, 0);
            c.pc = 0x0035_9698;
            let fl = |x: f32| x.to_bits();
            c.bus.write32(GAME_GP - 0x848, WORLD);
            let dt = if seed % 4 == 0 { r.f32(0.5) } else { fl(1.0 / 60.0) };
            c.bus.write32(WORLD + 0x14, dt);
            for lit in [LIT_END_MARGIN_STOP, LIT_END_MARGIN_BOUNCE, LIT_END_MARGIN_STOP4] {
                let v = if seed % 3 == 0 { r.f32(1.0) } else { fl(0.1) };
                c.bus.write32(GAME_GP - lit, v);
            }
            let accel_time = match seed % 4 {
                0 => 0,
                1 => fl(-0.5),
                _ => r.f32(2.0),
            };
            c.bus.write32(SELF + ACCEL_TIME, accel_time);
            c.bus.write32(SELF + ACCEL, r.f32(100.0));
            let length = 50.0 + (r.next_u32() % 100) as f32;
            c.bus.write32(SELF + LENGTH, fl(length));
            // Inside, just past the end, just before the start.
            let (pos, speed) = match (seed / 2) % 3 {
                0 => (length * 0.5, 30.0),
                1 => (length - 0.2, 600.0),
                _ => (0.2, -600.0),
            };
            let speed = if seed % 7 == 0 { -speed } else { speed };
            c.bus.write32(SELF + POS, fl(pos));
            c.bus.write32(SELF + SPEED, fl(speed));
            c.bus.write32(SELF + MODE, r.next_u32() % 5);
            c.bus.write32(SELF + ACTIVE, if seed % 5 == 1 { 0 } else { 1 + r.next_u32() % 3 });
            c.bus.write32(SELF + STOPPED, if seed % 5 == 2 { 1 } else { 0 });
            c.bus.write32(SELF + DIRTY, r.next_u32());
            c.g[A0][0] = SELF as u64;
        });
    }

    /// PATH_TrackSetVolume: random volume and scale bytes (both signs),
    /// the track playing (status byte negative) on half the seeds; the
    /// real PATH_PlatSetVolume takes its early out (no audio streams:
    /// the audio system's +0x6280 is 0). LO/HI compared too.
    #[test]
    fn path_track_set_volume_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{DATA, GAME_GP};
        use path_track::*;
        const TRACK: u32 = DATA + 0x4_0000;
        const PLAT0: u32 = DATA + 0x4_1000;
        const AUDIO: u32 = DATA + 0x5_0000;
        crate::difftest::check_lohi(ee::s14::f_003d62c8__generated, path_track_set_volume, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x003d_62c8, 0);
            c.pc = 0x003d_62c8;
            let status = r.next_u32() as u8;
            let status = if seed % 2 == 0 { status | 0x80 } else { status & 0x7f };
            c.bus.write8(TRACK + STATUS, status);
            c.bus.write8(TRACK + VOLUME, r.next_u32() as u8);
            c.bus.write8(TRACK + SCALE, r.next_u32() as u8);
            let plat = r.next_u32() % 8;
            c.bus.write32(TRACK + PLAT, plat);
            c.bus.write32(0x004d_3e98 + 4 * plat, PLAT0 + 0x100 * plat);
            c.bus.write32(GAME_GP + 0x410, AUDIO);
            c.bus.write32(AUDIO + 0x6280, 0);
            c.g[A0][0] = TRACK as u64;
            c.g[A1][0] = if seed % 5 == 0 { r.next_u32() as u64 } else { (r.next_u32() % 101) as u64 };
        });
    }

    /// TexMan_InvalidateBound: the texture manager at a random address with
    /// random bound slots. Events are delivered in the two-slot loop.
    #[test]
    fn tex_man_invalidate_bound_matches() {
        use crate::difftest::events::{check_events, schedule, Events};
        use crate::difftest::fakes::machine;
        use crate::difftest::{fill, DATA};
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 0x0039_6128, 0);
            let this = DATA + (r.next_u32() & 0x7_fff0);
            fill(c, &mut r, this + 0x1f40, 8);
            c.g[A0][0] = sx(this);
            let mode = schedule(c, &mut r, seed, 0x0039_6128);
            // The loop's only back edge comes 11 cycles in: make the event
            // seeds' event due by then.
            if mode != Events::Off {
                c.next_event = c.cycles + (r.next_u32() % 12) as u64;
            }
            mode
        };
        check_events(ee::s13::f_00396128__generated, tex_man_invalidate_bound, 200, &setup);
    }
}
