//! Staging module oc360 (wave wave216): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc360`.

#![allow(unused_imports)]

use crate::abi;
use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0013_aa48,
        rider_air_state_collide_walls,
        "RiderAirState_CollideWalls",
    ),
    (0x0030_9270, wscript_man_update, "WScriptMan_Update"),
    (
        0x0031_1f00,
        anim_controller_play_anim_impl,
        "AnimController_PlayAnimImpl",
    ),
    (
        0x0037_47a0,
        bezier_patch_cache_tessellate_patch,
        "BezierPatchCache_TessellatePatch",
    ),
    (
        0x0013_7138,
        rider_wipeout_state_post_update_root_pose,
        "RiderWipeoutState_PostUpdateRootPose",
    ),
    (0x0020_cfd0, game_ui_update, "GameUI_Update"),
    (
        0x0013_8640,
        rider_wipeout_state_post_update_ground,
        "RiderWipeoutState_PostUpdateGround",
    ),
    (0x003d_7ec8, spch_dequeue_channel, "SPCH_DequeueChannel"),
    (
        0x0010_d410,
        computer_actor_rider_score_path_branch,
        "ComputerActorRider_ScorePathBranch",
    ),
    (
        0x0012_d160,
        rider_wipeout_sub_update_phase2,
        "RiderWipeoutSub_UpdatePhase2",
    ),
];

// ---------------------------------------------------------------------------
// RiderAirState_CollideWalls (0x13aa48)

/// The air state's rider (`self + 4`).
pub(crate) const RACW_RIDER: u32 = 0x4;

/// Rider fields: hull centre, velocity, vtable (slot 0x40: IsHuman),
/// overlap list, wall hint, sphere tree.
pub(crate) const RACW_R_CENTRE: u32 = 0x180;

pub(crate) const RACW_R_VEL: u32 = 0x1e0;

pub(crate) const RACW_R_VTABLE: u32 = 0x6c0;

pub(crate) const RACW_R_OVERLAP: u32 = 0x860;

pub(crate) const RACW_R_WALL_HINT: u32 = 0x868;

pub(crate) const RACW_R_SPHERE_TREE: u32 = 0xaa0;

/// Stack frame: the hit (normal at +0x10, surface word at +0x4c), the hull
/// at +0x80, the impact record at +0x3e0 and the vector temporaries.
pub(crate) const RACW_HIT_NORMAL: u32 = 0x10;

pub(crate) const RACW_HULL: u32 = 0x80;

pub(crate) const RACW_SUB_STATE: u32 = 0x0011_fee8;

pub(crate) const RACW_COLL_HULL_CTOR: u32 = 0x0032_f650;

pub(crate) const RACW_FIND_CLOSEST_CONTACT: u32 = 0x0033_42d0;

pub(crate) const RACW_TRANSLATE: u32 = 0x0010_6538;

pub(crate) const RACW_BUILD_WORLD_MATRIX: u32 = 0x0011_e098;

pub(crate) const RACW_HANDLE_IMPACT: u32 = 0x0010_5d98;

pub(crate) const RACW_COLL_HULL_DTOR: u32 = 0x0032_f708;

/// `vf4.w = a . b` with the game's VU0 dot macro (`vf3` and `vf5` the
/// operands, `vf6` scratch).
fn racw_dot_vf3_vf5(c: &mut Cpu) {
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// `*dst = *v * f[fs]` through VU0 (`mfc1 v1; qmtc2 v1, vf3; vmul vf5,
/// vf4, vf3x`).
fn racw_scale(c: &mut Cpu, gpr: usize, fs: usize, v: u32, dst: u32) {
    c.mfc1(gpr, fs);
    c.lqc2(4, v);
    c.qmtc2(gpr, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, dst);
}

/// `int RiderAirState_CollideWalls(RiderAirState* self, Vec4* normal_out)`
/// (0x13aa48): unless the rider is in sub-state 9, sweep his collision hull
/// (centre `rider+0x180`) through the overlap list. On a contact at
/// distance `d >= 0`: translate the rider out along the hit normal by
/// `d * k`; if he is moving into the wall (`-n . vel >= 0`), push his
/// velocity back along the normal by `(-n . vel) + max(k2 * (-n . vel), k3)`
/// (a bounce), rebuild his world matrix, report the impact (hit position,
/// normal, the unit velocity direction, the surface word at hit+0x4c, the
/// speed into the wall), store the normal in `*normal_out` and return 1.
/// Otherwise 0. The hull is destroyed on every path past the sweep.
pub fn rider_air_state_collide_walls(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x490);
        let at = |c: &Cpu, o: u32| sp(c).wrapping_add(o);
        frame.save(c, S1, 0x460);
        frame.save(c, S4, 0x430);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x470);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S2, 0x450);
        frame.save(c, S3, 0x440);
        frame.save_ra(c, 0x420);
        frame.save_fpr(c, 21, 0x488);
        frame.save_fpr(c, 20, 0x480);
        c.g[A0][0] = Addr(r32(c, S1)).off(RACW_RIDER).lw(c);
        call_ptr(c, RACW_SUB_STATE, 0x0013_aa7c)?;
        c.g[V1][0] = 9;
        'done: {
            if c.g[V0][0] == c.g[V1][0] {
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'done;
            }
            c.g[S0][0] = Addr(r32(c, S1)).off(RACW_RIDER).lw(c);

            // CollHull hull(rider->sphere_tree, !rider->IsHuman())
            c.cycles += 29;
            c.g[S2][0] = sx(at(c, RACW_HULL));
            c.mtc1(0, 21);
            c.g[S3][0] = c.g[S2][0];
            let rider = r32(c, S0);
            c.g[V1][0] = Addr(rider).off(RACW_R_VTABLE).lw(c);
            c.g[A1][0] = sx(rider.wrapping_add(RACW_R_VTABLE));
            c.g[A0][0] = Addr(r32(c, V1)).off(0x40).lh(c);
            c.g[V0][0] = Addr(r32(c, V1)).off(0x44).lw(c);
            let method = r32(c, V0);
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            call_ptr(c, method, 0x0013_aab4)?;
            c.g[A1][0] = Addr(r32(c, S0)).off(RACW_R_SPHERE_TREE).lw(c);
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = c.g[V0][0] ^ 1;
            call_ptr(c, RACW_COLL_HULL_CTOR, 0x0013_aac4)?;

            // hull.centre = rider->centre; d = FindClosestContact(overlap,
            // &hull, &hit, wall_hint)
            c.g[V1][0] = Addr(r32(c, S1)).off(RACW_RIDER).lw(c);
            c.g[A0][0] = 1;
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[SP][0];
            let rider = r32(c, V1);
            c.lq(V0, rider.wrapping_add(RACW_R_CENTRE));
            Addr(at(c, 0x88)).sw(c, 1);
            c.sq(V0, at(c, 0x90));
            c.sq(V0, at(c, 0x3a0));
            c.g[A3][0] = Addr(rider).off(RACW_R_WALL_HINT).lw(c);
            c.g[A0][0] = Addr(rider).off(RACW_R_OVERLAP).lw(c);
            call_ptr(c, RACW_FIND_CLOSEST_CONTACT, 0x0013_aaf0)?;
            c.mov_s(1, 0);
            c.c_lt_s(1, 21);
            let missed = c.fcc();
            c.g[A0][0] = c.g[S3][0];
            let mut hit = false;
            if !missed {
                // Translate the rider out of the wall: n * k * d.
                c.cycles += 40;
                c.lwc1(0, r32(c, GP).wrapping_sub(0x7190));
                c.g[A1][0] = sx(at(c, 0x3a0));
                let s = sp(c);
                racw_scale(c, V1, 0, s + RACW_HIT_NORMAL, s + 0x3c0);
                c.lq(V0, at(c, 0x3c0));
                c.sq(V0, at(c, 0x3b0));
                racw_scale(c, V1, 1, s + 0x3b0, s + 0x3c0);
                c.lq(V0, at(c, 0x3c0));
                c.g[A0][0] = Addr(r32(c, S1)).off(RACW_RIDER).lw(c);
                c.sq(V0, at(c, 0x3a0));
                call_ptr(c, RACW_TRANSLATE, 0x0013_ab4c)?;

                // f20 = -n . vel
                c.g[AT][0] = 0xffff_ffff_bf80_0000;
                c.mtc1(AT, 0);
                let s = sp(c);
                racw_scale(c, V0, 0, s + RACW_HIT_NORMAL, s + 0x3c0);
                c.lq(V0, at(c, 0x3c0));
                c.g[A0][0] = Addr(r32(c, S1)).off(RACW_RIDER).lw(c);
                c.sq(V0, at(c, 0x3b0));
                c.lqc2(3, at(c, 0x3b0));
                c.lqc2(5, r32(c, A0).wrapping_add(RACW_R_VEL));
                racw_dot_vf3_vf5(c);
                c.qmfc2(V1, 4);
                c.mtc1(V1, 20);
                c.c_lt_s(20, 21);
                hit = !c.fcc();
                if !hit {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S3][0];
                }
            }
            if !hit {
                c.cycles += 4;
                c.g[A1][0] = 2;
                call_ptr(c, RACW_COLL_HULL_DTOR, 0x0013_abb0)?;
                c.g[V0][0] = 0;
                break 'done;
            }

            // dir = vel / |vel|; push = max(f20 * k2, k3)
            c.cycles += 18;
            c.lwc1(0, r32(c, GP).wrapping_sub(0x718c));
            c.lqc2(3, r32(c, A0).wrapping_add(RACW_R_VEL));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().rsqrt(0, 3, 4, 0);
            c.vu0().waitq();
            c.vu0().mul(0b1111, 5, 3, Src::Q);
            c.sqc2(5, at(c, 0x3c0));
            c.lwc1(1, r32(c, GP).wrapping_sub(0x7188));
            c.mul_s(0, 20, 0);
            c.lq(V0, at(c, 0x3c0));
            c.c_lt_s(0, 1);
            let below = c.fcc();
            c.sq(V0, at(c, 0x3b0));
            if below {
                c.cycles += 1;
                c.mov_s(0, 1);
            }

            // vel += n * (f20 + push)
            c.cycles += 32;
            c.add_s(0, 20, 0);
            let s = sp(c);
            racw_scale(c, V1, 0, s + RACW_HIT_NORMAL, s + 0x3d0);
            c.lq(V0, at(c, 0x3d0));
            c.g[V1][0] = Addr(r32(c, S1)).off(RACW_RIDER).lw(c);
            c.sq(V0, at(c, 0x3c0));
            let vel = r32(c, V1).wrapping_add(RACW_R_VEL);
            c.lqc2(3, vel);
            c.lqc2(4, at(c, 0x3c0));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, vel);
            c.g[A0][0] = Addr(r32(c, S1)).off(RACW_RIDER).lw(c);
            call_ptr(c, RACW_BUILD_WORLD_MATRIX, 0x0013_ac40)?;

            // Rider_HandleImpact(rider, {hit pos, dir, normal, f20},
            // hit.surface)
            c.lq(V0, at(c, 0x3b0));
            c.g[A1][0] = sx(at(c, 0x3e0));
            c.lq(A3, sp(c));
            c.lq(V1, at(c, 0x10));
            c.g[A0][0] = Addr(r32(c, S1)).off(RACW_RIDER).lw(c);
            c.g[A2][0] = Addr(at(c, 0x4c)).lw(c);
            c.sq(V0, at(c, 0x3f0));
            c.sq(A3, at(c, 0x3e0));
            c.sq(V1, at(c, 0x400));
            c.swc1(20, at(c, 0x410));
            call_ptr(c, RACW_HANDLE_IMPACT, 0x0013_ac6c)?;
            c.lq(V0, at(c, 0x10));
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 2;
            c.sq(V0, r32(c, S4));
            call_ptr(c, RACW_COLL_HULL_DTOR, 0x0013_ac80)?;
            c.g[V0][0] = 1;
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x470);
        frame.restore(c, S1, 0x460);
        frame.restore(c, S2, 0x450);
        frame.restore(c, S3, 0x440);
        frame.restore(c, S4, 0x430);
        frame.restore_ra(c, 0x420);
        frame.restore_fpr(c, 21, 0x488);
        frame.restore_fpr(c, 20, 0x480);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptMan_Update (0x309270)

/// Script manager fields: the five per-frame counters cleared on entry, the
/// pending-command list (`count` then 12-byte commands, copied whole: 0x184
/// bytes), the world reference, the current lookup result and process, the
/// process lists.
pub(crate) const WSU_COUNTERS: [u32; 5] = [0x4, 0x10, 0x1c, 0x28, 0x34];

pub(crate) const WSU_COMMANDS: u32 = 0x40;

pub(crate) const WSU_COMMANDS_END: u32 = 0x1c0;

pub(crate) const WSU_WORLD_REF: u32 = 0x28c;

pub(crate) const WSU_LOOKUP: u32 = 0x290;

pub(crate) const WSU_CURRENT: u32 = 0x2a4;

pub(crate) const WSU_RUNNING: u32 = 0x2b4;

pub(crate) const WSU_FIRST_RUNNING: u32 = 0x2b8;

pub(crate) const WSU_STARTING: u32 = 0x2bc;

/// A process's next link.
pub(crate) const WSU_PROC_NEXT: u32 = 0x18;

/// Command flags: dispatch to the process, forward to its task.
pub(crate) const WSU_DISPATCH: u64 = 0x20000;

pub(crate) const WSU_FORWARD: u64 = 0x10000;

pub(crate) const WSU_PROCESS_PENDING: Callee = Callee {
    addr: 0x0030_9118,
    f: ee::s10::WScriptMan_ProcessPendingRequest,
};

pub(crate) const WSU_POP_FRONT: Callee = Callee {
    addr: 0x0030_b208,
    f: ee::s10::WScriptProcessList_PopFront,
};

pub(crate) const WSU_PUSH_FRONT: Callee = Callee {
    addr: 0x0030_b260,
    f: ee::s10::WScriptProcessList_PushFront,
};

pub(crate) const WSU_START_PROCESS: Callee = Callee {
    addr: 0x0030_9750,
    f: ee::s10::WScriptMan_StartProcess,
};

pub(crate) const WSU_DISPATCH_EVENT: Callee = Callee {
    addr: 0x0030_78c0,
    f: ee::s10::WScriptProcess_DispatchEvent,
};

pub(crate) const WSU_FORWARD_EVENT: Callee = Callee {
    addr: 0x0030_7a10,
    f: ee::s10::WScriptProcess_ForwardEventToTask,
};

pub(crate) const WSU_PROCESS_UPDATE: Callee = Callee {
    addr: 0x0030_77f8,
    f: ee::s10::WScriptProcess_Update,
};

/// `void WScriptMan_Update(WScriptMan* self)` (0x309270): the script
/// manager's frame. Handle the pending request, clear the five counters,
/// move every starting process to the running list and start it, then take
/// the frame's command list (copied to the stack, the original cleared):
/// for each command, look up its world object when the command's type is 0
/// (`self+0x290`), and hand it to every running process (dispatch and/or
/// forward to its task, by the command's flags, with `self+0x2a4` = the
/// process). Finally update every running process.
///
/// Dispatcher-shaped: an event at a loop back edge can unwind, and the
/// function is re-entered at that loop head (0x3092b8, 0x3092f4, 0x30934c,
/// 0x309398, 0x309418, 0x309480) with its state in the registers.
pub fn wscript_man_update(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x220 };
        let s2 = |c: &Cpu| r32(c, S2);
        let mut pc = c.pc;
        loop {
            match pc {
                0x0030_9270 => {
                    c.cycles += 18;
                    let _ = Frame::open(c, 0x220);
                    frame.save(c, S2, 0x1f0);
                    frame.save(c, S0, 0x210);
                    frame.save(c, S1, 0x200);
                    frame.save(c, S3, 0x1e0);
                    frame.save(c, S4, 0x1d0);
                    frame.save(c, S5, 0x1c0);
                    frame.save(c, S6, 0x1b0);
                    frame.save(c, S7, 0x1a0);
                    frame.save_ra(c, 0x190);
                    c.g[S2][0] = c.g[A0][0];
                    WSU_PROCESS_PENDING.call(c, 0x0030_92a0)?;
                    for off in WSU_COUNTERS {
                        Addr(s2(c)).off(off).sw(c, 0);
                    }
                    pc = 0x0030_92cc;
                }
                // Start the next process taken off the starting list.
                0x0030_92b8 => {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S0][0];
                    WSU_PUSH_FRONT.call(c, 0x0030_92c0)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S2][0];
                    WSU_START_PROCESS.call(c, 0x0030_92cc)?;
                    pc = 0x0030_92cc;
                }
                0x0030_92cc => {
                    c.cycles += 5;
                    c.g[A0][0] = sx(s2(c).wrapping_add(WSU_STARTING));
                    WSU_POP_FRONT.call(c, 0x0030_92d4)?;
                    c.g[S0][0] = c.g[V0][0];
                    if c.g[S0][0] != 0 {
                        c.g[A0][0] = sx(s2(c).wrapping_add(WSU_RUNNING));
                        if back_edge_unwinds(c, 0x0030_92b8) {
                            return Ok(());
                        }
                        pc = 0x0030_92b8;
                        continue;
                    }
                    // Copy the command list to the stack (unaligned or
                    // doubleword copies, 0x20 bytes a step).
                    c.cycles += 4;
                    c.g[V1][0] = sx(s2(c).wrapping_add(WSU_COMMANDS));
                    c.g[V0][0] = c.g[V1][0] & 7;
                    let aligned = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[SP][0];
                    c.cycles += 1;
                    c.g[V0][0] = sx(s2(c).wrapping_add(WSU_COMMANDS_END));
                    pc = if aligned { 0x0030_934c } else { 0x0030_92f4 };
                }
                0x0030_92f4 => {
                    c.cycles += 19;
                    let (src, dst) = (r32(c, V1), r32(c, A0));
                    for (i, reg) in [A1, A2, A3, T0].into_iter().enumerate() {
                        let o = 8 * i as u32;
                        c.ldl(reg, src.wrapping_add(o + 7));
                        c.ldr(reg, src.wrapping_add(o));
                    }
                    for (i, reg) in [A1, A2, A3, T0].into_iter().enumerate() {
                        let o = 8 * i as u32;
                        c.sdl(reg, dst.wrapping_add(o + 7));
                        c.sdr(reg, dst.wrapping_add(o));
                    }
                    c.g[V1][0] = sx(src.wrapping_add(0x20));
                    c.g[A0][0] = sx(dst.wrapping_add(0x20));
                    if c.g[V1][0] != c.g[V0][0] {
                        if back_edge_unwinds(c, 0x0030_92f4) {
                            return Ok(());
                        }
                        continue;
                    }
                    c.cycles += 2;
                    pc = 0x0030_9378;
                }
                0x0030_934c => {
                    c.cycles += 11;
                    let (src, dst) = (r32(c, V1), r32(c, A0));
                    for (i, reg) in [A1, A2, A3, T0].into_iter().enumerate() {
                        c.g[reg][0] = c.ld(src.wrapping_add(8 * i as u32));
                    }
                    for (i, reg) in [A1, A2, A3, T0].into_iter().enumerate() {
                        c.sd(dst.wrapping_add(8 * i as u32), c.g[reg][0]);
                    }
                    c.g[V1][0] = sx(src.wrapping_add(0x20));
                    c.g[A0][0] = sx(dst.wrapping_add(0x20));
                    if c.g[V1][0] != c.g[V0][0] {
                        if back_edge_unwinds(c, 0x0030_934c) {
                            return Ok(());
                        }
                        continue;
                    }
                    pc = 0x0030_9378;
                }
                0x0030_9378 => {
                    // The last word, clear the list, and walk the commands.
                    c.cycles += 6;
                    c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                    let v = c.g[V0][0];
                    Addr(r32(c, A0)).sw(c, v);
                    Addr(s2(c)).off(WSU_COMMANDS).sw(c, 0);
                    c.g[V0][0] = Addr(sp(c)).lw(c);
                    c.g[A2][0] = 0;
                    if (c.g[V0][0] as i64) <= 0 {
                        pc = 0x0030_9470;
                        continue;
                    }
                    c.cycles += 2;
                    c.g[S7][0] = 0xc;
                    pc = 0x0030_9398;
                }
                // Command a2 (s3 = &commands[a2]).
                0x0030_9398 => {
                    c.cycles += 6;
                    c.mult(0, V0, A2, S7);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
                    c.g[S3][0] = sx(sp(c).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = Addr(r32(c, S3)).lhu(c);
                    if c.g[V1][0] == 0 {
                        wsu_lookup(c);
                    }
                    c.g[S0][0] = Addr(s2(c)).off(WSU_FIRST_RUNNING).lw(c);
                    c.cycles += 2;
                    c.g[S4][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                    if c.g[S0][0] == 0 {
                        pc = 0x0030_9458;
                        continue;
                    }
                    c.cycles += 4;
                    c.g[S6][0] = WSU_DISPATCH;
                    c.g[S5][0] = WSU_FORWARD;
                    c.g[S1][0] = c.g[S0][0];
                    pc = 0x0030_9418;
                }
                // Hand the command to process s1 (s0 = the next one).
                0x0030_9418 => {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(r32(c, S3)).lw(c);
                    c.g[S0][0] = Addr(r32(c, S0)).off(WSU_PROC_NEXT).lw(c);
                    c.g[V0][0] &= c.g[S6][0];
                    let v = c.g[S1][0];
                    Addr(s2(c)).off(WSU_CURRENT).sw(c, v);
                    if c.g[V0][0] != 0 {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = c.g[S3][0];
                        WSU_DISPATCH_EVENT.call(c, 0x0030_9438)?;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = Addr(r32(c, S3)).lw(c);
                    c.g[V0][0] &= c.g[S5][0];
                    c.g[A0][0] = c.g[S1][0];
                    if c.g[V0][0] != 0 {
                        c.cycles += 2;
                        c.g[A1][0] = c.g[S3][0];
                        WSU_FORWARD_EVENT.call(c, 0x0030_9450)?;
                    }
                    c.cycles += 2;
                    if c.g[S0][0] != 0 {
                        c.g[S1][0] = c.g[S0][0];
                        if back_edge_unwinds(c, 0x0030_9418) {
                            return Ok(());
                        }
                        continue;
                    }
                    pc = 0x0030_9458;
                }
                0x0030_9458 => {
                    c.cycles += 6;
                    c.g[V0][0] = Addr(sp(c)).lw(c);
                    c.g[A2][0] = c.g[S4][0];
                    Addr(s2(c)).off(WSU_LOOKUP).sw(c, 0);
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    Addr(s2(c)).off(WSU_CURRENT).sw(c, 0);
                    if c.g[V0][0] != 0 {
                        if back_edge_unwinds(c, 0x0030_9398) {
                            return Ok(());
                        }
                        pc = 0x0030_9398;
                        continue;
                    }
                    pc = 0x0030_9470;
                }
                0x0030_9470 => {
                    c.cycles += 3;
                    c.g[S0][0] = Addr(s2(c)).off(WSU_FIRST_RUNNING).lw(c);
                    if c.g[S0][0] == 0 {
                        Addr(s2(c)).off(WSU_CURRENT).sw(c, 0);
                        pc = 0x0030_949c;
                        continue;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S0][0];
                    pc = 0x0030_9480;
                }
                // Update process v0 (s0 = the next one).
                0x0030_9480 => {
                    c.cycles += 6;
                    c.g[S0][0] = Addr(r32(c, S0)).off(WSU_PROC_NEXT).lw(c);
                    c.g[A0][0] = c.g[V0][0];
                    let v = c.g[V0][0];
                    Addr(s2(c)).off(WSU_CURRENT).sw(c, v);
                    WSU_PROCESS_UPDATE.call(c, 0x0030_9490)?;
                    c.g[V0][0] = c.g[S0][0];
                    if c.g[S0][0] != 0 {
                        if back_edge_unwinds(c, 0x0030_9480) {
                            return Ok(());
                        }
                        continue;
                    }
                    c.cycles += 1;
                    Addr(s2(c)).off(WSU_CURRENT).sw(c, 0);
                    pc = 0x0030_949c;
                }
                0x0030_949c => {
                    c.cycles += 11;
                    frame.restore(c, S0, 0x210);
                    frame.restore(c, S1, 0x200);
                    frame.restore(c, S2, 0x1f0);
                    frame.restore(c, S3, 0x1e0);
                    frame.restore(c, S4, 0x1d0);
                    frame.restore(c, S5, 0x1c0);
                    frame.restore(c, S6, 0x1b0);
                    frame.restore(c, S7, 0x1a0);
                    frame.restore_ra(c, 0x190);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0030_9270, 0x0030_94c8);
                    return Ok(());
                }
            }
        }
    })
}

/// The world object a type-0 command names (`id & 0xff` selects a table
/// entry, `id >> 8` a word in it; `self+0x290` = that word's top 24 bits
/// times 4, or 0), as `WScriptMan_Update` inlines it. s3 = the command.
fn wsu_lookup(c: &mut Cpu) {
    c.cycles += 10;
    let s2 = r32(c, S2);
    c.g[V1][0] = Addr(s2).off(WSU_WORLD_REF).lw(c);
    c.g[A1][0] = Addr(r32(c, S3)).off(4).lw(c);
    c.g[A0][0] = Addr(r32(c, V1)).lw(c);
    c.g[V0][0] = c.g[A1][0] & 0xff;
    c.g[V1][0] = Addr(r32(c, A0)).off(8).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V1][0] = Addr(r32(c, V0)).lw(c);
    c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
    let mut found = false;
    if c.g[V1][0] != 0 {
        c.cycles += 7;
        c.g[A0][0] = Addr(r32(c, V1)).off(0x1c).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
        found = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    }
    if !found {
        c.cycles += 1;
        c.g[V0][0] = 0;
    }
    c.cycles += 2;
    let v = c.g[V0][0];
    Addr(s2).off(WSU_LOOKUP).sw(c, v);
}

// ---------------------------------------------------------------------------
// AnimController_PlayAnimImpl (0x311f00)

/// Controller fields copied into the new sequence: the bank word, the
/// time, the 32-byte transform, the 8-byte track key.
pub(crate) const PAI_CTRL_BANK: u32 = 0x18;

pub(crate) const PAI_CTRL_TIME: u32 = 0x1c;

pub(crate) const PAI_CTRL_KEY: u32 = 0x20;

pub(crate) const PAI_CTRL_XFORM: u32 = 0x30;

/// Sequence fields: first track's anim (+0) and id (+4), the transform
/// (+0x60), bank (+0x80), user word (+0x84), key (+0x88), time (+0x90),
/// weight (+0x94), target weight (+0x98), blend-in (+0xa4); three tracks
/// of 0x1c bytes from +0x4 (anim at +0x4, active at +0x18, time at +0x8,
/// speed at +0x14 of each).
pub(crate) const PAI_SEQ_XFORM: u32 = 0x60;

pub(crate) const PAI_SEQ_BANK: u32 = 0x80;

pub(crate) const PAI_SEQ_USER: u32 = 0x84;

pub(crate) const PAI_SEQ_KEY: u32 = 0x88;

pub(crate) const PAI_SEQ_TIME: u32 = 0x90;

pub(crate) const PAI_SEQ_WEIGHT: u32 = 0x94;

pub(crate) const PAI_SEQ_TARGET: u32 = 0x98;

pub(crate) const PAI_SEQ_BLEND_IN: u32 = 0xa4;

/// The animation banks (`gp+0xd08`): per bank (`anim & 0xff`) a header
/// with clips (0x14 bytes, `anim >> 8`) at +0x8 and event keys at +0x10.
pub(crate) const PAI_BANKS: u32 = 0xd08;

pub(crate) const PAI_CLEAR: Callee = Callee {
    addr: 0x0031_4668,
    f: ee::s10::AnimSeqList_Clear,
};

pub(crate) const PAI_GET_ENTRY: Callee = Callee {
    addr: 0x0031_15c8,
    f: ee::s10::AnimTable_GetEntry,
};

pub(crate) const PAI_PUSH_LOOPING: Callee = Callee {
    addr: 0x0031_4588,
    f: ee::s10::AnimSeqList_PushLoopingAnim,
};

pub(crate) const PAI_PUSH: Callee = Callee {
    addr: 0x0031_4518,
    f: ee::s10::AnimSeqList_PushAnim,
};

pub(crate) const PAI_ADD_EVENT_TRIGGER: Callee = Callee {
    addr: 0x0031_3ad8,
    f: ee::s10::AnimSequence_AddEventTrigger,
};

pub(crate) const PAI_ADD_TRIGGER: Callee = Callee {
    addr: 0x0031_3a70,
    f: ee::s10::AnimSequence_AddTrigger,
};

pub(crate) const PAI_SET_WEIGHT_FADE: Callee = Callee {
    addr: 0x0031_3a10,
    f: ee::s10::AnimSequence_SetWeightFade,
};

pub(crate) const PAI_GET_AT: Callee = Callee {
    addr: 0x0031_4760,
    f: ee::s10::AnimSeqList_GetAt,
};

pub(crate) const PAI_REMOVE_EVENT_TRIGGER: Callee = Callee {
    addr: 0x0031_3ba8,
    f: ee::s10::AnimSequence_RemoveEventTrigger,
};

pub(crate) const PAI_FADE_OUT: Callee = Callee {
    addr: 0x0031_3a20,
    f: ee::s10::AnimSequence_FadeOut,
};

pub(crate) const PAI_SET_TRACK_ANIM: Callee = Callee {
    addr: 0x0031_3c50,
    f: ee::s10::AnimSequence_SetTrackAnim,
};

pub(crate) const PAI_SET_TRACK_TIME: Callee = Callee {
    addr: 0x0031_3cf0,
    f: ee::s10::AnimSequence_SetTrackTime,
};

pub(crate) const PAI_SET_TRACK_WEIGHT: Callee = Callee {
    addr: 0x0031_3d28,
    f: ee::s10::AnimSequence_SetTrackWeight,
};

pub(crate) const PAI_REMOVE: Callee = Callee {
    addr: 0x0031_45f8,
    f: ee::s10::AnimSeqList_Remove,
};

/// `f1 = *(s0 + off)`, `f0 = *(s1 + off)` (or the v1/v0 bases), `c.eq.s`.
fn pai_eq(c: &mut Cpu, a: u32, b: u32) -> bool {
    c.lwc1(1, a);
    c.lwc1(0, b);
    c.c_eq_s(1, 0);
    c.fcc()
}

/// `int AnimController_PlayAnimImpl(AnimController* self, AnimSeqList*
/// list, int anim, int clip, int keep, int user, float blend)` (0x311f00):
/// start `anim` (clip `a3`) on the controller's sequence list. A zero
/// blend clears the list first. The new sequence (looping when the anim's
/// table entry type is 2; otherwise with the 0x3f event trigger unless
/// the type is non-zero and the entry's +8 word is 0) takes the
/// controller's bank, transform, key and time, weight 1, the entry's
/// blend-in, `user`, and one trigger per clip event key (frames / 30).
/// Alone on the list it fades in over the entry's fade time, if any.
/// Otherwise, with a blend, every older sequence loses its trigger and
/// fades out over `blend`; one with the same anim/id, key, bank and
/// transform (or of a type other than 0/1/2) is a match: unless `keep`,
/// the new sequence inherits its active tracks and weight (fading the
/// rest in) and the match is removed; else the new one fades in from 0.
/// Returns `anim`.
pub fn anim_controller_play_anim_impl(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0xa0);
        frame.save_fpr(c, 20, 0x90);
        frame.save_fpr(c, 21, 0x98);
        c.mtc1(0, 20);
        c.mov_s(21, 12);
        frame.save(c, S2, 0x60);
        c.c_eq_s(21, 20);
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S5, 0x30);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S6, 0x20);
        c.g[S5][0] = c.g[A2][0];
        frame.save(c, S7, 0x10);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, S0, 0x80);
        c.g[S7][0] = c.g[T0][0];
        frame.save(c, S1, 0x70);
        c.g[S4][0] = c.g[T1][0];
        let no_blend = c.fcc();
        frame.save_ra(c, 0);
        if no_blend {
            c.cycles += 2;
            c.g[A0][0] = c.g[S6][0];
            PAI_CLEAR.call(c, 0x0031_1f60)?;
        }

        // Push the new sequence (s1).
        c.cycles += 6;
        c.g[A0][0] = c.g[S5][0];
        PAI_GET_ENTRY.call(c, 0x0031_1f68)?;
        c.g[V1][0] = Addr(r32(c, V0)).off(4).lw(c);
        c.g[V0][0] = 2;
        let looping = c.g[V1][0] == c.g[V0][0];
        c.mov_s(12, 20);
        if looping {
            c.cycles += 6;
            c.g[A0][0] = c.g[S6][0];
            c.g[A1][0] = c.g[S5][0];
            c.g[A2][0] = c.g[S3][0];
            PAI_PUSH_LOOPING.call(c, 0x0031_1f88)?;
            c.g[S1][0] = c.g[V0][0];
        } else {
            c.cycles += 10;
            c.g[A0][0] = c.g[S6][0];
            c.g[A1][0] = c.g[S5][0];
            c.g[A2][0] = c.g[S3][0];
            PAI_PUSH.call(c, 0x0031_1fa0)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S5][0];
            PAI_GET_ENTRY.call(c, 0x0031_1fac)?;
            c.g[V1][0] = Addr(r32(c, V0)).off(4).lw(c);
            c.g[A0][0] = c.g[S1][0];
            let mut trigger = c.g[V1][0] == 0;
            if !trigger {
                c.cycles += 5;
                c.g[A0][0] = c.g[S5][0];
                PAI_GET_ENTRY.call(c, 0x0031_1fc0)?;
                c.g[V1][0] = Addr(r32(c, V0)).off(8).lw(c);
                c.g[A0][0] = c.g[S1][0];
                trigger = c.g[V1][0] != 0;
            }
            if trigger {
                c.cycles += 2;
                c.g[A1][0] = 0x3f;
                PAI_ADD_EVENT_TRIGGER.call(c, 0x0031_1fd4)?;
            }
        }

        // Copy the controller's state; weight = target = 1.
        c.cycles += 33;
        let (ctrl, seq) = (r32(c, S2), r32(c, S1));
        c.g[V1][0] = Addr(ctrl).off(PAI_CTRL_BANK).lw(c);
        c.g[A0][0] = c.g[S5][0];
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 1);
        c.g[S0][0] = 0;
        let v = c.g[V1][0];
        Addr(seq).off(PAI_SEQ_BANK).sw(c, v);
        c.lq(V0, ctrl.wrapping_add(PAI_CTRL_XFORM));
        c.sq(V0, seq.wrapping_add(PAI_SEQ_XFORM));
        c.lq(V1, ctrl.wrapping_add(PAI_CTRL_XFORM + 0x10));
        c.sq(V1, seq.wrapping_add(PAI_SEQ_XFORM + 0x10));
        c.g[V0][0] = c.ld(ctrl.wrapping_add(PAI_CTRL_KEY));
        c.sd(seq.wrapping_add(PAI_SEQ_KEY), c.g[V0][0]);
        c.lwc1(0, ctrl.wrapping_add(PAI_CTRL_TIME));
        c.swc1(1, seq.wrapping_add(PAI_SEQ_WEIGHT));
        c.swc1(0, seq.wrapping_add(PAI_SEQ_TIME));
        c.swc1(1, seq.wrapping_add(PAI_SEQ_TARGET));
        PAI_GET_ENTRY.call(c, 0x0031_2018)?;
        let seq = r32(c, S1);
        c.lwc1(0, r32(c, V0).wrapping_add(0x18));
        c.g[A1][0] = sx(r32(c, S3) >> 8);
        c.g[A0][0] = Addr(r32(c, GP)).off(PAI_BANKS).lw(c);
        c.g[V0][0] = c.g[S3][0] & 0xff;
        c.swc1(0, seq.wrapping_add(PAI_SEQ_BLEND_IN));
        c.g[A2][0] = sx(r32(c, V0) << 2);
        let v = c.g[S4][0];
        Addr(seq).off(PAI_SEQ_USER).sw(c, v);
        c.g[V0][0] = 0x14;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A2)));
        c.mult(0, A1, A1, V0);
        c.g[V1][0] = Addr(r32(c, A0)).lw(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(8).lw(c);
        c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
        c.g[S2][0] = Addr(r32(c, V0)).off(0x12).lhu(c);
        if c.g[S2][0] != 0 {
            // One trigger per event key of the clip (s2 keys).
            c.cycles += 4;
            c.lwc1(20, r32(c, GP).wrapping_sub(0x3234));
            c.g[S4][0] = c.g[A2][0];
            c.g[S3][0] = c.g[A1][0];
            c.g[V0][0] = Addr(r32(c, GP)).off(PAI_BANKS).lw(c);
            loop {
                c.cycles += 20;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S1][0];
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S4)));
                c.g[A2][0] = Addr(r32(c, V0)).lw(c);
                c.g[V1][0] = Addr(r32(c, A2)).off(8).lw(c);
                c.g[A3][0] = Addr(r32(c, A2)).off(0x10).lw(c);
                c.g[V1][0] = sx(r32(c, S3).wrapping_add(r32(c, V1)));
                c.g[V0][0] = Addr(r32(c, V1)).off(0x10).lhu(c);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S0)));
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A3)));
                c.g[V1][0] = Addr(r32(c, V0)).lhu(c);
                c.mtc1(V1, 12);
                c.cvt_s_w(12, 12);
                c.mul_s(12, 12, 20);
                PAI_ADD_TRIGGER.call(c, 0x0031_20ac)?;
                let more = (c.g[S0][0] as i64) < (c.g[S2][0] as i64);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                c.g[V0][0] = Addr(r32(c, GP)).off(PAI_BANKS).lw(c);
                if !more {
                    c.cycles += 1;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0031_2068);
                }
            }
        }
        c.g[V1][0] = Addr(r32(c, S6)).lw(c);

        c.cycles += 3;
        c.g[V0][0] = 1;
        if c.g[V1][0] == c.g[V0][0] {
            // Alone: fade in over the entry's fade time.
            c.cycles += 8;
            c.g[A0][0] = c.g[S5][0];
            PAI_GET_ENTRY.call(c, 0x0031_20d0)?;
            c.lwc1(13, r32(c, V0).wrapping_add(0x14));
            c.mtc1(0, 0);
            c.c_eq_s(13, 0);
            let none = c.fcc();
            c.g[V0][0] = c.g[S5][0];
            if !none {
                c.cycles += 8;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(AT, 12);
                c.g[A0][0] = c.g[S1][0];
                let seq = r32(c, S1);
                c.swc1(0, seq.wrapping_add(PAI_SEQ_WEIGHT));
                c.swc1(0, seq.wrapping_add(PAI_SEQ_TARGET));
                PAI_SET_WEIGHT_FADE.call(c, 0x0031_2100)?;
                c.g[V0][0] = c.g[S5][0];
            }
        } else {
            c.cycles += 5;
            c.mtc1(0, 0);
            c.c_eq_s(21, 0);
            let no_blend = c.fcc();
            c.g[V0][0] = c.g[S5][0];
            if !no_blend {
                pai_blend_out(c)?;
            }
        }

        c.cycles += 13;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x98);
        frame.restore_fpr(c, 20, 0x90);
        frame.close(c);
        Ok(())
    })
}

/// The blend part of `AnimController_PlayAnimImpl` (v1 = the list's
/// count, not 1; f21 = blend != 0): fade out the older sequences, find a
/// match (s4), and inherit from it or fade the new one in.
fn pai_blend_out(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 4;
    c.g[S2][0] = 1;
    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
    let any = c.g[V0][0] != 0;
    c.g[S4][0] = 0;
    if any {
        c.cycles += 3;
        c.g[S3][0] = 1;
        c.g[A0][0] = c.g[S6][0];
        loop {
            // other = list[s2]: drop its trigger, fade it out.
            c.cycles += 16;
            c.g[A1][0] = c.g[S2][0];
            PAI_GET_AT.call(c, 0x0031_2140)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = 0x3f;
            c.g[A0][0] = c.g[S0][0];
            PAI_REMOVE_EVENT_TRIGGER.call(c, 0x0031_2150)?;
            c.g[A0][0] = c.g[S0][0];
            c.mov_s(12, 21);
            PAI_FADE_OUT.call(c, 0x0031_215c)?;
            c.g[A0][0] = c.g[S5][0];
            PAI_GET_ENTRY.call(c, 0x0031_2164)?;
            let (other, seq) = (r32(c, S0), r32(c, S1));
            c.g[A0][0] = Addr(r32(c, V0)).off(4).lw(c);
            c.g[V1][0] = Addr(other).off(4).lw(c);
            c.g[V0][0] = Addr(seq).off(4).lw(c);
            // Same first-track id, or a type other than 0, 2 and 1.
            let mut ok = c.g[V1][0] == c.g[V0][0];
            c.g[A1][0] = 0;
            if !ok {
                c.cycles += 2;
                c.g[V0][0] = 2;
                ok = c.g[A0][0] != 0;
                if ok {
                    c.cycles += 2;
                    ok = c.g[A0][0] != c.g[V0][0];
                    if ok {
                        c.cycles += 2;
                        ok = c.g[A0][0] != c.g[S3][0];
                    }
                }
            }
            if ok {
                c.cycles += 1;
                c.g[A1][0] = 1;
            }
            c.cycles += 2;
            if c.g[A1][0] != 0 {
                pai_match(c);
            } else {
                c.g[V0][0] = Addr(r32(c, S6)).lw(c);
            }

            c.cycles += 4;
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S6][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0031_2138);
            }
        }
    }

    c.cycles += 2;
    let inherit = c.g[S4][0] != 0 && {
        c.cycles += 2;
        c.g[S7][0] == 0
    };
    if !inherit {
        // Fade the new sequence in from 0 over the blend.
        Addr(r32(c, S1)).off(PAI_SEQ_TARGET).sw(c, 0);
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        Addr(r32(c, S1)).off(PAI_SEQ_WEIGHT).sw(c, 0);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 12);
        c.mov_s(13, 21);
        PAI_SET_WEIGHT_FADE.call(c, 0x0031_2384)?;
        c.g[V0][0] = c.g[S5][0];
        return Ok(());
    }

    // Inherit the match's active tracks (anim, time, speed).
    c.cycles += 4;
    c.g[S2][0] = 0;
    c.g[S0][0] = sx(r32(c, S4).wrapping_add(0x14));
    c.g[S3][0] = sx(r32(c, S4).wrapping_add(0x18));
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, S3)).lw(c);
        c.g[A0][0] = c.g[S1][0];
        if c.g[V0][0] != 0 {
            c.cycles += 11;
            c.g[A2][0] = Addr(r32(c, S0).wrapping_sub(0x10)).lw(c);
            c.g[A1][0] = c.g[S2][0];
            PAI_SET_TRACK_ANIM.call(c, 0x0031_22f0)?;
            c.lwc1(12, r32(c, S0).wrapping_sub(0xc));
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S2][0];
            PAI_SET_TRACK_TIME.call(c, 0x0031_2300)?;
            c.lwc1(12, r32(c, S0));
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S2][0];
            PAI_SET_TRACK_WEIGHT.call(c, 0x0031_2310)?;
        }
        c.cycles += 5;
        c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x1c));
        c.g[V0][0] = ((c.g[S2][0] as i64) < 3) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S3][0] = sx(r32(c, S3).wrapping_add(0x1c));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0031_22d8);
        }
    }
    // weight = target = match weight; fade the rest in.
    c.cycles += 10;
    let (matched, seq) = (r32(c, S4), r32(c, S1));
    c.lwc1(0, matched.wrapping_add(PAI_SEQ_WEIGHT));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 12);
    c.swc1(0, seq.wrapping_add(PAI_SEQ_WEIGHT));
    c.swc1(0, seq.wrapping_add(PAI_SEQ_TARGET));
    c.lwc1(0, matched.wrapping_add(PAI_SEQ_WEIGHT));
    c.c_lt_s(0, 12);
    let partial = c.fcc();
    c.g[A0][0] = c.g[S1][0];
    if partial {
        c.cycles += 3;
        c.sub_s(13, 12, 0);
        c.mul_s(13, 21, 13);
        PAI_SET_WEIGHT_FADE.call(c, 0x0031_2358)?;
    }
    c.cycles += 5;
    c.g[A0][0] = c.g[S6][0];
    c.g[A1][0] = c.g[S4][0];
    PAI_REMOVE.call(c, 0x0031_2364)?;
    c.g[V0][0] = c.g[S5][0];
    Ok(())
}

/// The match test of one older sequence (s0) against the new one (s1)
/// after the type test passed: same anim word, key and bank, then the
/// transform (+0x70 row first, then +0x60): s4 = s0 when all equal. Ends
/// with v0 = the list's count.
fn pai_match(c: &mut Cpu) {
    let (other, seq) = (r32(c, S0), r32(c, S1));
    let fail = |c: &mut Cpu| c.g[V0][0] = Addr(r32(c, S6)).lw(c);
    c.cycles += 4;
    c.g[V1][0] = Addr(other).lw(c);
    c.g[V0][0] = Addr(seq).lw(c);
    if c.g[V1][0] != c.g[V0][0] {
        return fail(c);
    }
    c.cycles += 4;
    c.g[V1][0] = c.ld(other.wrapping_add(PAI_SEQ_KEY));
    c.g[V0][0] = c.ld(seq.wrapping_add(PAI_SEQ_KEY));
    if c.g[V1][0] != c.g[V0][0] {
        return fail(c);
    }
    c.cycles += 4;
    c.g[V1][0] = Addr(other).off(PAI_SEQ_BANK).lw(c);
    c.g[V0][0] = Addr(seq).off(PAI_SEQ_BANK).lw(c);
    if c.g[V1][0] != c.g[V0][0] {
        return fail(c);
    }

    // Second row of the transform (+0x70..+0x7c).
    c.cycles += 9;
    c.lwc1(1, other.wrapping_add(0x70));
    c.g[V1][0] = sx(other.wrapping_add(PAI_SEQ_XFORM));
    c.lwc1(0, seq.wrapping_add(0x70));
    c.g[V0][0] = sx(seq.wrapping_add(PAI_SEQ_XFORM));
    c.g[A0][0] = 0;
    c.c_eq_s(1, 0);
    let mut eq = c.fcc();
    c.g[A1][0] = 0;
    if eq {
        c.cycles += 5;
        eq = pai_eq(c, other.wrapping_add(0x74), seq.wrapping_add(0x74));
        if eq {
            c.cycles += 5;
            eq = pai_eq(c, other.wrapping_add(0x78), seq.wrapping_add(0x78));
            if eq {
                c.cycles += 6;
                if pai_eq(c, other.wrapping_add(0x7c), seq.wrapping_add(0x7c)) {
                    c.g[A1][0] = 1;
                }
            }
        }
    }

    c.cycles += 2;
    if c.g[A1][0] != 0 {
        // First row (+0x60..+0x6c): a0 = 1 when all equal.
        let (a, b) = (r32(c, V1), r32(c, V0));
        c.cycles += 6;
        let e0 = pai_eq(c, a, b);
        c.g[A1][0] = 0;
        // 0: to the "a0 = 1" step, 1: to the "a1 ? a0 : 0" step, 2: past it.
        let stage = if !e0 {
            0
        } else {
            c.cycles += 6;
            let e1 = pai_eq(c, a.wrapping_add(4), b.wrapping_add(4));
            c.g[A0][0] = c.g[S3][0];
            if !e1 {
                1
            } else {
                c.cycles += 6;
                if !pai_eq(c, a.wrapping_add(8), b.wrapping_add(8)) {
                    if c.g[A1][0] == 0 {
                        c.g[A0][0] = 0;
                    }
                    2
                } else {
                    c.cycles += 6;
                    if pai_eq(c, a.wrapping_add(0xc), b.wrapping_add(0xc)) {
                        c.g[A1][0] = 1;
                        1
                    } else {
                        0
                    }
                }
            }
        };
        if stage == 0 {
            c.cycles += 1;
            c.g[A0][0] = c.g[S3][0];
        }
        if stage <= 1 {
            c.cycles += 1;
            if c.g[A1][0] == 0 {
                c.g[A0][0] = 0;
            }
        }
        c.cycles += 1;
    }
    if c.g[A0][0] != 0 {
        c.g[S4][0] = c.g[S0][0];
    }
    c.cycles += 1;
    c.g[V0][0] = Addr(r32(c, S6)).lw(c);
}

// ---------------------------------------------------------------------------
// BezierPatchCache_TessellatePatch (0x3747a0)

/// Cache fields: per level (0-2) the free-slot stacks (+0x30) and counts
/// (+0x3c), the position slabs (+0x48, slots of 0xc0/0x1b0/0x300 bytes),
/// UV slabs (+0x54, 0x80/0x120/0x200), second-UV slabs (+0x60), the
/// entries (+0x18, 12 bytes: patch, flags, normal slot), the normal stacks
/// (+0x1cc), counts (+0x1d8) and slabs (+0x1b4).
pub(crate) const TPP_FREE: u32 = 0x30;

pub(crate) const TPP_FREE_COUNT: u32 = 0x3c;

pub(crate) const TPP_POS: [u32; 3] = [0x48, 0x4c, 0x50];

pub(crate) const TPP_UV: [u32; 3] = [0x54, 0x58, 0x5c];

pub(crate) const TPP_UV2: [u32; 3] = [0x60, 0x64, 0x68];

pub(crate) const TPP_ENTRIES: u32 = 0x18;

pub(crate) const TPP_NRM_FREE: u32 = 0x1cc;

pub(crate) const TPP_NRM_COUNT: u32 = 0x1d8;

pub(crate) const TPP_NRM: [u32; 3] = [0x1b4, 0x1b8, 0x1bc];

/// The patch's slot index per level (`patch + 0x1a6 + 2 * level`).
pub(crate) const TPP_PATCH_SLOT: u32 = 0x1a6;

pub(crate) const TPP_TESS_POSITIONS: Callee = Callee {
    addr: 0x0037_4180,
    f: ee::s12::BezierPatchCache_TessPositions,
};

pub(crate) const TPP_TESS_UVS: Callee = Callee {
    addr: 0x0037_4298,
    f: ee::s12::BezierPatchCache_TessUVs,
};

pub(crate) const TPP_TESS_UV2: Callee = Callee {
    addr: 0x0037_4440,
    f: ee::s12::BezierPatchCache_TessUV2,
};

pub(crate) const TPP_TESS_POSITIONS_ALT: Callee = Callee {
    addr: 0x0037_4518,
    f: ee::s12::BezierPatchCache_TessPositionsAlt,
};

/// `a2 = slab[level] + slot * {0xc0, 0x1b0, 0x300}[level]` (`mult` by
/// `slot_reg`), or 0 for a level outside 0-2; then `end` (the original's
/// delay-slot op). s0 = level, s1 = the cache; the caller charged the first
/// test.
fn tpp_select_positions(c: &mut Cpu, offs: [u32; 3], slot_reg: usize, end: fn(&mut Cpu)) {
    let level = c.g[S0][0];
    let cache = r32(c, S1);
    let (size, off) = if level == 1 {
        c.cycles += 3;
        (0x1b0, offs[1])
    } else {
        c.cycles += 3;
        c.g[V0][0] = ((level as i64) < 2) as u64;
        c.g[V0][0] = 2;
        c.cycles += 2;
        c.g[A2][0] = 0;
        if (level as i64) >= 2 {
            if level != 2 {
                c.cycles += 2;
                return end(c);
            }
            c.cycles += 2;
            (0x300, offs[2])
        } else {
            if level != 0 {
                c.cycles += 2;
                return end(c);
            }
            c.cycles += 3;
            (0xc0, offs[0])
        }
    };
    c.g[V0][0] = size;
    c.g[V1][0] = Addr(cache).off(off).lw(c);
    c.cycles += 3;
    c.mult(0, V0, slot_reg, V0);
    c.g[A2][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    end(c);
}

/// `dest = slab[level] + slot * {0x80, 0x120, 0x200}[level]` (level 1 by
/// `mult` with s6, the others by shifts of s4), or 0; then `end`.
fn tpp_select_uvs(c: &mut Cpu, dest: usize, offs: [u32; 3], end: fn(&mut Cpu)) {
    let level = c.g[S0][0];
    let cache = r32(c, S1);
    c.cycles += 2;
    c.g[V0][0] = ((level as i64) < 2) as u64;
    if level == 1 {
        c.cycles += 5;
        c.g[V0][0] = 0x120;
        c.g[V1][0] = Addr(cache).off(offs[1]).lw(c);
        c.mult(0, V0, S6, V0);
    } else {
        c.cycles += 2;
        let high = c.g[V0][0] == 0;
        c.g[V0][0] = 2;
        c.cycles += 2;
        c.g[dest][0] = 0;
        let shift = if high {
            if level != 2 {
                c.cycles += 2;
                return end(c);
            }
            c.cycles += 3;
            9
        } else {
            if level != 0 {
                c.cycles += 2;
                return end(c);
            }
            c.cycles += 4;
            7
        };
        let off = if shift == 9 { offs[2] } else { offs[0] };
        c.g[V1][0] = Addr(cache).off(off).lw(c);
        c.g[V0][0] = sx(r32(c, S4) << shift);
    }
    c.g[dest][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    c.cycles += 1;
    end(c);
}

fn tpp_end_v0_one(c: &mut Cpu) {
    c.g[V0][0] = 1;
}

fn tpp_end_a0_cache(c: &mut Cpu) {
    c.g[A0][0] = c.g[S1][0];
}

/// `int BezierPatchCache_TessellatePatch(BezierPatchCache* self, Patch*
/// patch, int level, int bit, int normals)` (0x3747a0): take a free slot
/// of `level` (-1 when none), tessellate the patch's positions, UVs and
/// second UVs into that slot's slabs, record the patch in the slot's entry
/// (flags |= 0xc | 1 << bit) and the slot in the patch; with `normals`,
/// also take a normal slot and tessellate the alternative positions into
/// it. Returns the slot.
pub fn bezier_patch_cache_tessellate_patch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        c.g[S0][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S5, 0x40);
        frame.save(c, S7, 0x20);
        let cache = r32(c, S1);
        c.g[V0][0] = sx(cache.wrapping_add(TPP_FREE_COUNT));
        frame.save(c, FP, 0x10);
        c.g[A0][0] = sx(r32(c, S0) << 2);
        frame.save(c, S2, 0x70);
        c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
        frame.save(c, S3, 0x60);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S4, 0x50);
        c.g[S7][0] = c.g[A3][0];
        frame.save(c, S6, 0x30);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, V1)).lw(c);
        c.g[FP][0] = c.g[T0][0];
        // slot = free[level][--free_count[level]], or -1.
        if (c.g[V0][0] as i64) <= 0 {
            c.cycles += 1;
            c.g[S4][0] = u64::MAX;
        } else {
            c.cycles += 8;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            c.g[A0][0] = sx(cache.wrapping_add(r32(c, A0)));
            let v = c.g[V0][0];
            Addr(r32(c, V1)).sw(c, v);
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V1][0] = Addr(r32(c, A0)).off(TPP_FREE).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            c.g[S4][0] = Addr(r32(c, V0)).lw(c);
        }

        // The slot's position (a2), UV (s3) and second-UV (s2) slabs.
        c.cycles += 3;
        c.g[V0][0] = 1;
        c.g[S6][0] = c.g[S4][0];
        tpp_select_positions(c, TPP_POS, S4, tpp_end_v0_one);
        tpp_select_uvs(c, S3, TPP_UV, tpp_end_v0_one);
        tpp_select_uvs(c, S2, TPP_UV2, tpp_end_a0_cache);

        c.cycles += 30;
        c.g[A1][0] = c.g[S5][0];
        c.g[A3][0] = c.g[S0][0];
        TPP_TESS_POSITIONS.call(c, 0x0037_4950)?;
        c.g[A2][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S5][0];
        c.g[A3][0] = c.g[S0][0];
        TPP_TESS_UVS.call(c, 0x0037_4964)?;
        c.g[A2][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S5][0];
        c.g[A3][0] = c.g[S0][0];
        TPP_TESS_UV2.call(c, 0x0037_4978)?;

        // entry = &entries[level][slot]: patch, flags |= 0xc | 1 << bit.
        let cache = r32(c, S1);
        c.g[V0][0] = 0xc;
        c.g[A2][0] = sx(r32(c, S0) << 2);
        c.mult(0, V0, S6, V0);
        c.g[A0][0] = sx(cache.wrapping_add(r32(c, A2)));
        c.g[A1][0] = Addr(r32(c, A0)).off(TPP_ENTRIES).lw(c);
        c.g[V1][0] = sx(r32(c, S0) << 1);
        c.g[V1][0] = sx(r32(c, S5).wrapping_add(r32(c, V1)));
        c.g[A3][0] = 1;
        c.g[A0][0] = sx(1u32 << (r32(c, S7) & 31));
        c.g[S3][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
        let entry = r32(c, S3);
        let patch = c.g[S5][0];
        Addr(entry).sw(c, patch);
        let slot = c.g[S4][0];
        Addr(r32(c, V1)).off(TPP_PATCH_SLOT).sh(c, slot);
        c.g[V0][0] = Addr(entry).off(4).lw(c);
        c.g[V0][0] |= 0xc;
        c.g[V0][0] |= c.g[A0][0];
        let flags = c.g[V0][0];
        Addr(entry).off(4).sw(c, flags);
        if c.g[FP][0] != 0 {
            // nslot = nrm_free[level][--nrm_count[level]]
            c.cycles += 11;
            let a2 = r32(c, A2);
            c.g[V1][0] = sx(cache.wrapping_add(TPP_NRM_COUNT));
            c.g[A0][0] = sx(cache.wrapping_add(a2));
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(a2));
            c.g[V0][0] = Addr(r32(c, V1)).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            let v = c.g[V0][0];
            Addr(r32(c, V1)).sw(c, v);
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V1][0] = Addr(r32(c, A0)).off(TPP_NRM_FREE).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            c.g[S2][0] = Addr(r32(c, V0)).lw(c);
            tpp_select_positions(c, TPP_NRM, S2, tpp_end_a0_cache);
            c.cycles += 4;
            c.g[A1][0] = c.g[S5][0];
            c.g[A3][0] = c.g[S0][0];
            TPP_TESS_POSITIONS_ALT.call(c, 0x0037_4a4c)?;
            let nslot = c.g[S2][0];
            Addr(r32(c, S3)).off(8).sw(c, nslot);
        }

        c.cycles += 13;
        c.g[V0][0] = c.g[S4][0];
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderWipeoutState_PostUpdateRootPose (0x137138)

/// Wipeout state fields: the body's velocity (+0x10) and angular velocity
/// (+0x20), the body (+0x40; active flag at +0x150, position at +0x130),
/// the sphere tree (+0x44), the swept segment (+0x50, +0x60).
pub(crate) const PURP_VEL: u32 = 0x10;

pub(crate) const PURP_ANG_VEL: u32 = 0x20;

pub(crate) const PURP_BODY: u32 = 0x40;

pub(crate) const PURP_TREE: u32 = 0x44;

pub(crate) const PURP_SEG: u32 = 0x50;

pub(crate) const PURP_BODY_POS: u32 = 0x130;

pub(crate) const PURP_BODY_ACTIVE: u32 = 0x150;

/// Stack frame: the hit (normal at +0x10, object at +0x50), the sphere at
/// +0x80 (its segment at +0xa0/+0xb0), vector temporaries, the sines and
/// cosines at +0x410.
pub(crate) const PURP_HIT_NORMAL: u32 = 0x10;

pub(crate) const PURP_HIT_OBJECT: u32 = 0x50;

pub(crate) const PURP_SPHERE: u32 = 0x80;

pub(crate) const PURP_BUILD_SPHERES: Callee = Callee {
    addr: 0x0013_7550,
    f: ee::s01::RiderWipeoutState_BuildBoardSpheres,
};

pub(crate) const PURP_COLL_HULL_CTOR: u32 = 0x0032_f650;

pub(crate) const PURP_FIND_CLOSEST_CONTACT: u32 = 0x0033_6850;

pub(crate) const PURP_TREE_TRANSLATE: u32 = 0x0032_9b40;

pub(crate) const PURP_RAND_RANGE: u32 = 0x0031_7830;

pub(crate) const PURP_SIN_COS: u32 = 0x0031_be50;

pub(crate) const PURP_COLL_HULL_DTOR: u32 = 0x0032_f708;

/// `hit.object->owner->OnPush(&hit, impulse)` (vtable slot 0x158), the
/// impulse `src * (world->int_at_0x10 * k)` built at `sp+tmp`, the
/// argument at `sp+arg`. a0 = hit.object.
fn purp_push_object(
    c: &mut Cpu,
    k_off: u32,
    src: u32,
    tmp: u32,
    arg: u32,
    ret: u32,
) -> crate::call::Flow {
    c.cycles += 3;
    c.g[V1][0] = Addr(r32(c, A0)).off(0xc).lw(c);
    c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
    if c.g[V1][0] == 0 {
        return Ok(());
    }
    c.cycles += 19;
    let s = sp(c);
    c.g[A1][0] = c.g[SP][0];
    c.g[A3][0] = Addr(r32(c, V1)).off(0xc).lw(c);
    c.g[A2][0] = sx(s.wrapping_add(arg));
    c.lwc1(0, r32(c, V0).wrapping_add(0x10));
    c.cvt_s_w(0, 0);
    c.lwc1(1, r32(c, GP).wrapping_sub(k_off));
    c.g[A0][0] = Addr(r32(c, A3)).off(0x158).lh(c);
    c.mul_s(0, 0, 1);
    c.g[A0][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
    racw_scale(c, V1, 0, src, s.wrapping_add(tmp));
    c.lq(V0, s.wrapping_add(tmp));
    c.sq(V0, s.wrapping_add(arg));
    c.g[V1][0] = Addr(r32(c, A3)).off(0x15c).lw(c);
    let method = r32(c, V1);
    call_ptr(c, method, ret)
}

/// `void RiderWipeoutState_PostUpdateRootPose(RiderWipeoutState* self)`
/// (0x137138): with the body active, rebuild the board spheres and test
/// them against the world. On a contact at distance `d >= 0`: push the
/// body, its sphere tree and swept segment out along the hit normal by
/// `d * k`. Hitting hard (`-n . vel >= k1`) or a wall (`n.z < k2`)
/// bounces: the velocity gains `n * (v + max(v / 2, k3))`, a hit object
/// is pushed by the impulse, the spin is damped and, if it has slowed below
/// k4, re-randomised (a random direction times a random 45-180 degrees).
/// Otherwise the body comes to rest (a hit object is pushed by the
/// velocity; velocity and spin take the zero vector at 0x4ff120).
pub fn rider_wipeout_state_post_update_root_pose(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x470);
        frame.save(c, S1, 0x440);
        frame.save(c, S0, 0x450);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x430);
        frame.save_ra(c, 0x420);
        frame.save_fpr(c, 21, 0x468);
        frame.save_fpr(c, 20, 0x460);
        let this = r32(c, S1);
        c.g[V0][0] = Addr(this).off(PURP_BODY).lw(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(PURP_BODY_ACTIVE).lw(c);
        frame.restore(c, S0, 0x450);
        'done: {
            if c.g[V1][0] == 0 {
                break 'done;
            }
            c.cycles += 27;
            PURP_BUILD_SPHERES.call(c, 0x0013_7170)?;
            let this = r32(c, S1);
            c.g[S0][0] = sx(sp(c).wrapping_add(PURP_SPHERE));
            c.g[A1][0] = Addr(this).off(PURP_TREE).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = 1;
            call_ptr(c, PURP_COLL_HULL_CTOR, 0x0013_7184)?;
            // self->seg = the sphere's segment; d = FindClosestContact(
            // world->collision, &sphere, &hit, 0)
            let (this, s) = (r32(c, S1), sp(c));
            c.g[S2][0] = c.g[S0][0];
            c.lq(A0, s + 0xb0);
            c.g[A1][0] = c.g[S0][0];
            c.lq(V1, s + 0xa0);
            c.g[A2][0] = c.g[SP][0];
            c.sq(A0, this.wrapping_add(PURP_SEG));
            c.g[A3][0] = 0;
            c.sq(V1, this.wrapping_add(PURP_SEG + 0x10));
            c.sq(A0, s + 0x3a0);
            c.sq(V1, s + 0x3b0);
            c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0x84).lw(c);
            c.g[A0][0] = Addr(r32(c, V1)).off(0x20).lw(c);
            call_ptr(c, PURP_FIND_CLOSEST_CONTACT, 0x0013_71bc)?;
            c.mov_s(1, 0);
            c.mtc1(0, 0);
            c.c_lt_s(1, 0);
            let missed = c.fcc();
            c.g[A0][0] = c.g[S2][0];
            if missed {
                c.cycles += 4;
                c.g[A1][0] = 2;
                call_ptr(c, PURP_COLL_HULL_DTOR, 0x0013_71dc)?;
                frame.restore(c, S0, 0x450);
                break 'done;
            }

            // push = n * k * d: the body, its tree and the segment move.
            c.cycles += 58;
            let s = sp(c);
            c.lwc1(0, r32(c, GP).wrapping_sub(0x72ac));
            c.g[A1][0] = sx(s + 0x3c0);
            racw_scale(c, V1, 0, s + PURP_HIT_NORMAL, s + 0x3e0);
            c.lq(V0, s + 0x3e0);
            c.sq(V0, s + 0x3d0);
            racw_scale(c, V1, 1, s + 0x3d0, s + 0x3e0);
            c.lq(V0, s + 0x3e0);
            c.g[V1][0] = Addr(r32(c, S1)).off(PURP_BODY).lw(c);
            c.sq(V0, s + 0x3c0);
            let pos = r32(c, V1).wrapping_add(PURP_BODY_POS);
            c.lqc2(3, pos);
            c.lqc2(4, s + 0x3c0);
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, pos);
            c.g[A0][0] = Addr(r32(c, S1)).off(PURP_TREE).lw(c);
            call_ptr(c, PURP_TREE_TRANSLATE, 0x0013_7240)?;
            let (this, s) = (r32(c, S1), sp(c));
            c.lq(V0, this.wrapping_add(PURP_SEG));
            c.lq(V1, this.wrapping_add(PURP_SEG + 0x10));
            c.sq(V0, s + 0x3a0);
            c.sq(V1, s + 0x3b0);
            for seg in [PURP_SEG, PURP_SEG + 0x10] {
                if seg != PURP_SEG {
                    c.g[AT][0] = 0xffff_ffff_bf80_0000;
                    c.mtc1(AT, 0);
                }
                c.lqc2(3, this.wrapping_add(seg));
                c.lqc2(4, s + 0x3c0);
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, this.wrapping_add(seg));
            }
            // f2 = -n . vel
            c.lwc1(1, r32(c, GP).wrapping_sub(0x72a8));
            racw_scale(c, V1, 0, s + PURP_HIT_NORMAL, s + 0x3e0);
            c.lq(V0, s + 0x3e0);
            c.sq(V0, s + 0x3d0);
            c.lqc2(3, s + 0x3d0);
            c.lqc2(5, this.wrapping_add(PURP_VEL));
            racw_dot_vf3_vf5(c);
            c.qmfc2(V1, 4);
            c.mtc1(V1, 2);
            c.c_le_s(1, 2);
            let mut bounce = c.fcc();
            c.lwc1(1, s + PURP_HIT_NORMAL + 8);
            if !bounce {
                c.cycles += 5;
                c.lwc1(0, r32(c, GP).wrapping_sub(0x72a4));
                c.c_lt_s(1, 0);
                bounce = c.fcc();
                c.g[A0][0] = Addr(s + PURP_HIT_OBJECT).lw(c);
            }

            if bounce {
                purp_bounce(c)?;
            } else {
                // Come to rest.
                c.cycles += 2;
                c.g[V0][0] = 0x50_0000;
                if c.g[A0][0] != 0 {
                    let vel = r32(c, S1).wrapping_add(PURP_VEL);
                    purp_push_object(c, 0x727c, vel, 0x3e0, 0x3d0, 0x0013_7508)?;
                    c.cycles += 1;
                    c.g[V0][0] = 0x50_0000;
                }
                c.cycles += 6;
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xee0));
                let this = r32(c, S1);
                c.lq(V1, r32(c, V0));
                c.sq(V1, this.wrapping_add(PURP_VEL));
                c.lq(A0, r32(c, V0));
                c.sq(A0, this.wrapping_add(PURP_ANG_VEL));
                c.g[A0][0] = c.g[S2][0];
            }

            c.cycles += 3;
            c.g[A1][0] = 2;
            call_ptr(c, PURP_COLL_HULL_DTOR, 0x0013_752c)?;
            frame.restore(c, S0, 0x450);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x440);
        frame.restore(c, S2, 0x430);
        frame.restore_ra(c, 0x420);
        frame.restore_fpr(c, 21, 0x468);
        frame.restore_fpr(c, 20, 0x460);
        frame.close(c);
        Ok(())
    })
}

/// The bounce of `RiderWipeoutState_PostUpdateRootPose` (f2 = v): the
/// impulse, the pushed hit object, the damped and maybe re-randomised
/// spin. Ends with a0 = the sphere (s2).
fn purp_bounce(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 8;
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(AT, 0);
    c.lwc1(1, r32(c, GP).wrapping_sub(0x72a0));
    c.mul_s(0, 2, 0);
    c.c_lt_s(0, 1);
    if c.fcc() {
        c.mov_s(0, 1);
    }
    // vel += n * (v + max(v / 2, k3))
    c.cycles += 15;
    c.add_s(0, 2, 0);
    let s = sp(c);
    racw_scale(c, V1, 0, s + PURP_HIT_NORMAL, s + 0x3e0);
    c.lq(V0, s + 0x3e0);
    c.sq(V0, s + 0x3d0);
    let vel = r32(c, S1).wrapping_add(PURP_VEL);
    c.lqc2(3, vel);
    c.lqc2(4, s + 0x3d0);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, vel);
    c.g[A0][0] = Addr(s + PURP_HIT_OBJECT).lw(c);
    if c.g[A0][0] != 0 {
        purp_push_object(c, 0x729c, s + 0x3d0, 0x3f0, 0x3e0, 0x0013_7394)?;
    }

    // spin *= k; if |spin| < k4: re-randomise it.
    c.cycles += 21;
    let spin = r32(c, S1).wrapping_add(PURP_ANG_VEL);
    c.lwc1(0, r32(c, GP).wrapping_sub(0x7298));
    racw_scale(c, V0, 0, spin, spin);
    c.lwc1(1, r32(c, GP).wrapping_sub(0x7294));
    c.lqc2(3, spin);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(V0, 22);
    c.mtc1(V0, 0);
    c.c_lt_s(0, 1);
    let slow = c.fcc();
    c.g[A0][0] = c.g[S2][0];
    if !slow {
        return Ok(());
    }
    c.cycles += 48;
    let gp = r32(c, GP);
    c.lwc1(12, gp.wrapping_sub(0x7290));
    c.lwc1(13, gp.wrapping_sub(0x728c));
    call_ptr(c, PURP_RAND_RANGE, 0x0013_73f8)?;
    let gp = r32(c, GP);
    c.lwc1(13, gp.wrapping_sub(0x7288));
    c.lwc1(12, gp.wrapping_sub(0x7284));
    c.mov_s(20, 0);
    call_ptr(c, PURP_RAND_RANGE, 0x0013_7408)?;
    c.mov_s(21, 0);
    let s = sp(c);
    c.g[A0][0] = sx(s + 0x410);
    c.g[A1][0] = sx(s + 0x414);
    c.mov_s(12, 20);
    call_ptr(c, PURP_SIN_COS, 0x0013_741c)?;
    c.mov_s(12, 21);
    let s = sp(c);
    c.g[A0][0] = sx(s + 0x418);
    c.g[A1][0] = sx(s + 0x41c);
    call_ptr(c, PURP_SIN_COS, 0x0013_742c)?;
    c.g[AT][0] = 0x4234_0000;
    c.mtc1(AT, 12);
    c.g[AT][0] = 0x4334_0000;
    c.mtc1(AT, 13);
    call_ptr(c, PURP_RAND_RANGE, 0x0013_7444)?;
    // spin += (cp * sy, cp * cy, sp, 0) * deg * (pi / 180)
    let s = sp(c);
    c.lwc1(1, s + 0x414);
    c.lwc1(2, s + 0x418);
    c.lwc1(4, s + 0x41c);
    c.mul_s(2, 1, 2);
    c.lwc1(5, s + 0x410);
    c.mul_s(1, 1, 4);
    c.lwc1(3, r32(c, GP).wrapping_sub(0x7280));
    c.swc1(5, s + 0x3f8);
    c.swc1(2, s + 0x3f0);
    c.mul_s(0, 0, 3);
    c.swc1(1, s + 0x3f4);
    Addr(s + 0x3fc).sw(c, 0);
    racw_scale(c, V1, 0, s + 0x3f0, s + 0x400);
    c.lq(V0, s + 0x400);
    c.sq(V0, s + 0x3e0);
    let spin = r32(c, S1).wrapping_add(PURP_ANG_VEL);
    c.lqc2(3, spin);
    c.lqc2(4, s + 0x3e0);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, spin);
    c.g[A0][0] = c.g[S2][0];
    Ok(())
}

// ---------------------------------------------------------------------------
// GameUI_Update (0x20cfd0)

/// The pending UI request (`gp-0x9f4`): 0 none, 1 menu result, 2 replay
/// finished, 3 close the PDA template, 4 back to the front end.
pub(crate) const GUU_REQUEST: u32 = 0x9f4;

/// The request jump table (5 words).
pub(crate) const GUU_TABLE: u32 = 0x0047_1ae0;

/// The world's game (`world+0x84`) fields: UI state manager (+0x48, its
/// stack at +0x18), replay (+0x28), flags (+0x200, word +8), mode (+0x214).
pub(crate) const GUU_GAME: u32 = 0x84;

pub(crate) const GUU_UI_MGR: u32 = 0x48;

pub(crate) const GUU_UI_STACK: u32 = 0x18;

pub(crate) const GUU_GET_ACTIVE_STATE: u32 = 0x0039_fa88;

pub(crate) const GUU_HANDLE_MENU_RESULT: Callee = Callee {
    addr: 0x0020_ccf8,
    f: ee::s05::GameUI_HandleMenuResult,
};

pub(crate) const GUU_REPLAY_FINISH: u32 = 0x0027_06f0;

pub(crate) const GUU_BE_GET_INTERFACE: u32 = 0x0014_dd58;

pub(crate) const GUU_CLOSE_ALL: u32 = 0x0039_f840;

pub(crate) const GUU_PDA_CLOSE: Callee = Callee {
    addr: 0x0020_d190,
    f: ee::s05::OVScreenPDATemplate_Close,
};

pub(crate) const GUU_REQUEST_STATE: u32 = 0x0023_1250;

/// `void GameUI_Update()` (0x20cfd0): act on the pending UI request.
/// 1: handle the menu result. 2: finish the replay, set the game's flag
/// word, and in mode 2 with the active UI state's id 8, touch BE
/// interface 7 and close every UI state if the flag at 0x534b30 is set.
/// 3: close the PDA template and every UI state. 4: request game state
/// 0xe (2, 0). Requests 2-4 are cleared first.
pub fn game_ui_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        let world = |c: &mut Cpu| Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
        c.g[V1][0] = world(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, V1)).off(GUU_GAME).lw(c);
        c.g[S1][0] = Addr(r32(c, V0)).off(GUU_UI_MGR).lw(c);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(GUU_UI_STACK));
        c.g[S0][0] = 0;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, GUU_GET_ACTIVE_STATE, 0x0020_d000)?;
            c.g[S0][0] = c.g[V0][0];
        }

        c.cycles += 4;
        let request = Addr(r32(c, GP).wrapping_sub(GUU_REQUEST));
        c.g[V1][0] = request.lw(c);
        c.g[V0][0] = (c.g[V1][0] < 5) as u64;
        let in_table = c.g[V0][0] != 0;
        c.g[V0][0] = 0x47_0000;
        // The paths that end at the shared exit (restores s0 there).
        let mut tail = true;
        if in_table {
            c.cycles += 6;
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(GUU_TABLE & 0xffff));
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A0][0] = Addr(r32(c, V1)).lw(c);
            let target = r32(c, A0);
            match target {
                0x0020_d02c => {
                    c.cycles += 4;
                    GUU_HANDLE_MENU_RESULT.call(c, 0x0020_d034)?;
                    frame.restore(c, S0, 0x20);
                    tail = false;
                }
                0x0020_d03c => tail = guu_replay_finished(c)?,
                0x0020_d0b8 => {
                    c.cycles += 9;
                    request.sw(c, 0);
                    GUU_PDA_CLOSE.call(c, 0x0020_d0c0)?;
                    c.g[V0][0] = world(c);
                    c.g[V1][0] = Addr(r32(c, V0)).off(GUU_GAME).lw(c);
                    c.g[A0][0] = Addr(r32(c, V1)).off(GUU_UI_MGR).lw(c);
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(GUU_UI_STACK));
                    call_ptr(c, GUU_CLOSE_ALL, 0x0020_d0d4)?;
                    frame.restore(c, S0, 0x20);
                    tail = false;
                }
                0x0020_d0dc => {
                    c.cycles += 7;
                    c.g[V0][0] = world(c);
                    c.g[A1][0] = 0xe;
                    request.sw(c, 0);
                    c.g[A2][0] = 2;
                    c.g[A0][0] = Addr(r32(c, V0)).off(GUU_GAME).lw(c);
                    c.g[A3][0] = 0;
                    call_ptr(c, GUU_REQUEST_STATE, 0x0020_d0f8)?;
                }
                0x0020_d0f8 => {}
                _ => {
                    c.jump_out(target, 0x0020_cfd0, 0x0020_d10c);
                    return Ok(());
                }
            }
        }
        if tail {
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Request 2 of `GameUI_Update` (s0 = the active UI state or 0, s1 = the
/// UI manager). Ok(true) when it ends at the shared exit (which restores
/// s0), Ok(false) when it restored s0 itself (where the original does,
/// once before the BE call).
fn guu_replay_finished(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
    let frame = Frame { size: 0x30 };
    c.cycles += 11;
    let gp = r32(c, GP);
    c.g[V1][0] = Addr(gp.wrapping_sub(0x848)).lw(c);
    Addr(gp.wrapping_sub(GUU_REQUEST)).sw(c, 0);
    c.g[V0][0] = Addr(r32(c, V1)).off(GUU_GAME).lw(c);
    c.g[A0][0] = Addr(r32(c, V0)).off(0x28).lw(c);
    call_ptr(c, GUU_REPLAY_FINISH, 0x0020_d050)?;
    c.g[A1][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
    c.g[A0][0] = 1;
    let world = r32(c, A1);
    c.g[V0][0] = Addr(world).off(GUU_GAME).lw(c);
    c.g[V1][0] = Addr(r32(c, V0)).off(0x200).lw(c);
    Addr(r32(c, V1)).off(8).sw(c, 1);
    if c.g[S0][0] == 0 {
        return Ok(true);
    }
    c.cycles += 5;
    c.g[V0][0] = Addr(world).off(GUU_GAME).lw(c);
    c.g[A0][0] = 2;
    c.g[V1][0] = Addr(r32(c, V0)).off(0x214).lw(c);
    if c.g[V1][0] != c.g[A0][0] {
        frame.restore(c, S0, 0x20);
        return Ok(false);
    }
    c.cycles += 4;
    c.g[V1][0] = Addr(r32(c, S0)).off(0x98).lw(c);
    c.g[V0][0] = 8;
    let id8 = c.g[V1][0] == c.g[V0][0];
    frame.restore(c, S0, 0x20);
    if !id8 {
        return Ok(false);
    }
    c.cycles += 7;
    c.g[A0][0] = Addr(world).off(0x78).lw(c);
    c.g[A1][0] = 7;
    call_ptr(c, GUU_BE_GET_INTERFACE, 0x0020_d098)?;
    c.g[V1][0] = 0x53_0000;
    c.g[V0][0] = Addr(0x0053_4b30).lw(c);
    frame.restore(c, S0, 0x20);
    if c.g[V0][0] == 0 {
        return Ok(false);
    }
    c.cycles += 4;
    c.g[A0][0] = sx(r32(c, S1).wrapping_add(GUU_UI_STACK));
    call_ptr(c, GUU_CLOSE_ALL, 0x0020_d0b0)?;
    frame.restore(c, S0, 0x20);
    Ok(false)
}

// ---------------------------------------------------------------------------
// RiderWipeoutState_PostUpdateGround (0x138640)

/// The wipeout state: landed flag (+0), pending flag (+4), the rider
/// (+0x40). Rider fields: velocity, ground normal, overlap list, wall
/// hint, sphere tree.
pub(crate) const PUG_RIDER: u32 = 0x40;

pub(crate) const PUG_R_VEL: u32 = 0x1e0;

pub(crate) const PUG_R_GROUND_NORMAL: u32 = 0x370;

pub(crate) const PUG_R_OVERLAP: u32 = 0x860;

pub(crate) const PUG_R_WALL_HINT: u32 = 0x868;

pub(crate) const PUG_R_SPHERE_TREE: u32 = 0xaa0;

/// Stack frame: the hit (normal +0x10, contact velocity +0x20, object
/// +0x50), the sphere at +0x80, vector temporaries from +0x3a0.
pub(crate) const PUG_HIT_NORMAL: u32 = 0x10;

pub(crate) const PUG_HIT_VEL: u32 = 0x20;

pub(crate) const PUG_HIT_OBJECT: u32 = 0x50;

pub(crate) const PUG_SPHERE: u32 = 0x80;

pub(crate) const PUG_COLL_HULL_CTOR: u32 = 0x0032_f650;

pub(crate) const PUG_FIND_CLOSEST_CONTACT: u32 = 0x0033_42d0;

pub(crate) const PUG_TRANSLATE: u32 = 0x0010_6538;

pub(crate) const PUG_UPDATE_BOUNDS: u32 = 0x0011_e150;

pub(crate) const PUG_WALL_COLLISION: u32 = 0x0010_5398;

pub(crate) const PUG_RIDER_COLLISIONS: u32 = 0x0010_7888;

pub(crate) const PUG_COLL_HULL_DTOR: u32 = 0x0032_f708;

/// `self->rider` (s1 = self).
fn pug_rider(c: &mut Cpu) -> u64 {
    Addr(r32(c, S1)).off(PUG_RIDER).lw(c)
}

/// `void RiderWipeoutState_PostUpdateGround(RiderWipeoutState* self)`
/// (0x138640): the sliding rider's sphere (along the ground normal) against
/// the world. A contact at `d >= 0` whose normal is not opposite the
/// ground (`ground . n > k`): a normal leaning into the ground is
/// flattened (the ground part removed, renormalised), a hit object's
/// owner is told (vtable slot 0x150), the rider moves out by `n * k2 * d`,
/// and moving into the contact (`-n . (vel - contact vel) >= 0`) bounces
/// him (`vel += n * (v + clamp(v / 2, lo, hi))`). Then bounds, wall and
/// rider collisions; a pending flag becomes the landed flag; a velocity
/// slower than 50 stops; the hull is destroyed.
pub fn rider_wipeout_state_post_update_ground(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x430);
        c.g[A2][0] = 1;
        frame.save(c, S0, 0x410);
        frame.save(c, S1, 0x400);
        c.g[S0][0] = sx(sp(c).wrapping_add(PUG_SPHERE));
        frame.save(c, S2, 0x3f0);
        c.g[S1][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x420);
        c.g[A0][0] = c.g[S0][0];
        frame.save_ra(c, 0x3e0);
        c.g[S2][0] = c.g[S0][0];
        c.g[V0][0] = pug_rider(c);
        c.g[A1][0] = Addr(r32(c, V0)).off(PUG_R_SPHERE_TREE).lw(c);
        call_ptr(c, PUG_COLL_HULL_CTOR, 0x0013_8678)?;
        // sphere.dir = ground normal; d = FindClosestContact(overlap,
        // &sphere, &hit, wall_hint)
        c.g[V1][0] = pug_rider(c);
        let (rider, s) = (r32(c, V1), sp(c));
        c.g[A0][0] = 1;
        Addr(s + 0x88).sw(c, 1);
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[SP][0];
        c.lq(V0, rider.wrapping_add(PUG_R_GROUND_NORMAL));
        c.sq(V0, s + 0x90);
        c.g[A3][0] = Addr(rider).off(PUG_R_WALL_HINT).lw(c);
        c.g[A0][0] = Addr(rider).off(PUG_R_OVERLAP).lw(c);
        call_ptr(c, PUG_FIND_CLOSEST_CONTACT, 0x0013_86a0)?;
        c.mtc1(0, 2);
        c.mov_s(20, 0);
        c.c_le_s(2, 20);
        if c.fcc() {
            pug_contact(c)?;
        }
        c.g[A0][0] = pug_rider(c);

        c.cycles += 10;
        c.g[A1][0] = 0;
        call_ptr(c, PUG_UPDATE_BOUNDS, 0x0013_88c0)?;
        c.g[A0][0] = pug_rider(c);
        c.g[A1][0] = 0;
        call_ptr(c, PUG_WALL_COLLISION, 0x0013_88cc)?;
        c.g[A0][0] = pug_rider(c);
        call_ptr(c, PUG_RIDER_COLLISIONS, 0x0013_88d4)?;
        let this = r32(c, S1);
        c.g[V0][0] = Addr(this).off(4).lw(c);
        let pending = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if pending {
            c.cycles += 2;
            Addr(this).off(4).sw(c, 0);
            Addr(this).sw(c, 1);
        }

        // Stop below 50.
        c.cycles += 17;
        c.g[A0][0] = pug_rider(c);
        let vel = r32(c, A0).wrapping_add(PUG_R_VEL);
        c.g[AT][0] = 0x4248_0000;
        c.mtc1(AT, 0);
        c.lqc2(3, vel);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(V0, 22);
        c.mtc1(V0, 1);
        c.c_lt_s(1, 0);
        let slow = c.fcc();
        c.g[V0][0] = 0x50_0000;
        if slow {
            c.cycles += 3;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xee0));
            c.lq(V1, r32(c, V0));
            c.sq(V1, vel);
        }

        c.cycles += 10;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 2;
        call_ptr(c, PUG_COLL_HULL_DTOR, 0x0013_8944)?;
        frame.restore(c, S0, 0x410);
        frame.restore(c, S1, 0x400);
        frame.restore(c, S2, 0x3f0);
        frame.restore_ra(c, 0x3e0);
        frame.restore_fpr(c, 20, 0x420);
        frame.close(c);
        Ok(())
    })
}

/// The contact of `RiderWipeoutState_PostUpdateGround` (f20 = d).
fn pug_contact(c: &mut Cpu) -> crate::call::Flow {
    // f1 = ground . n
    c.cycles += 15;
    c.g[V1][0] = pug_rider(c);
    let (rider, s) = (r32(c, V1), sp(c));
    c.lwc1(0, r32(c, GP).wrapping_sub(0x722c));
    c.lqc2(3, rider.wrapping_add(PUG_R_GROUND_NORMAL));
    c.lqc2(5, s + PUG_HIT_NORMAL);
    racw_dot_vf3_vf5(c);
    c.qmfc2(V0, 4);
    c.mtc1(V0, 1);
    c.c_lt_s(0, 1);
    if !c.fcc() {
        return Ok(());
    }
    c.cycles += 4;
    c.c_lt_s(1, 2);
    let into_ground = c.fcc();
    c.g[A0][0] = Addr(s + PUG_HIT_OBJECT).lw(c);
    if into_ground {
        // n = normalize(n - ground * (ground . n))
        c.cycles += 27;
        racw_scale(c, V0, 1, rider.wrapping_add(PUG_R_GROUND_NORMAL), s + 0x3d0);
        c.lq(V0, s + 0x3d0);
        c.sq(V0, s + 0x3c0);
        c.lqc2(3, s + PUG_HIT_NORMAL);
        c.lqc2(4, s + 0x3c0);
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, s + 0x3d0);
        c.lq(V0, s + 0x3d0);
        c.sq(V0, s + 0x3b0);
        c.lqc2(3, s + 0x3b0);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().rsqrt(0, 3, 4, 0);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 3, Src::Q);
        c.sqc2(5, s + 0x3d0);
        c.lq(V0, s + 0x3d0);
        c.sq(V0, s + PUG_HIT_NORMAL);
        c.sq(V0, s + 0x3a0);
        c.g[A0][0] = Addr(s + PUG_HIT_OBJECT).lw(c);
    }
    // hit.object->owner->OnContact(&hit) (vtable slot 0x150)
    c.cycles += 2;
    if c.g[A0][0] != 0 {
        c.cycles += 3;
        c.g[A2][0] = Addr(r32(c, A0)).off(0xc).lw(c);
        c.g[A1][0] = c.g[SP][0];
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            let owner = r32(c, A2);
            c.g[V0][0] = Addr(owner).off(0xc).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).off(0x150).lh(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0x154).lw(c);
            let method = r32(c, V1);
            c.g[A0][0] = sx(owner.wrapping_add(r32(c, A0)));
            call_ptr(c, method, 0x0013_8798)?;
        }
    }

    // Move out: n * k2 * d.
    c.cycles += 48;
    let s = sp(c);
    c.lwc1(0, r32(c, GP).wrapping_sub(0x7228));
    c.g[A1][0] = sx(s + 0x3a0);
    racw_scale(c, V1, 0, s + PUG_HIT_NORMAL, s + 0x3c0);
    c.lq(V0, s + 0x3c0);
    c.sq(V0, s + 0x3b0);
    racw_scale(c, V1, 20, s + 0x3b0, s + 0x3c0);
    c.lq(V0, s + 0x3c0);
    c.g[A0][0] = pug_rider(c);
    c.sq(V0, s + 0x3a0);
    call_ptr(c, PUG_TRANSLATE, 0x0013_87e0)?;

    // f3 = -n . (vel - contact vel)
    c.g[V0][0] = pug_rider(c);
    let (rider, s) = (r32(c, V0), sp(c));
    c.lqc2(3, rider.wrapping_add(PUG_R_VEL));
    c.lqc2(4, s + PUG_HIT_VEL);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s + 0x3c0);
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(AT, 0);
    c.lq(V0, s + 0x3c0);
    c.mtc1(0, 1);
    c.sq(V0, s + 0x3b0);
    racw_scale(c, V1, 0, s + PUG_HIT_NORMAL, s + 0x3d0);
    c.lq(V0, s + 0x3d0);
    c.sq(V0, s + 0x3c0);
    c.lqc2(3, s + 0x3c0);
    c.lqc2(5, s + 0x3b0);
    racw_dot_vf3_vf5(c);
    c.qmfc2(V1, 4);
    c.mtc1(V1, 3);
    c.c_le_s(1, 3);
    if !c.fcc() {
        return Ok(());
    }
    // vel += n * (v + clamp(v / 2, lo, hi))
    c.cycles += 9;
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(AT, 0);
    c.lwc1(1, r32(c, GP).wrapping_sub(0x7224));
    c.mul_s(2, 3, 0);
    c.lwc1(4, r32(c, GP).wrapping_sub(0x7220));
    c.c_le_s(1, 2);
    let above = c.fcc();
    c.mov_s(0, 1);
    if above {
        c.cycles += 1;
        c.min_s(0, 2, 4);
    }
    c.cycles += 14;
    c.add_s(0, 3, 0);
    racw_scale(c, V1, 0, s + PUG_HIT_NORMAL, s + 0x3d0);
    c.lq(V0, s + 0x3d0);
    c.g[V1][0] = pug_rider(c);
    c.sq(V0, s + 0x3c0);
    let vel = r32(c, V1).wrapping_add(PUG_R_VEL);
    c.lqc2(3, vel);
    c.lqc2(4, s + 0x3c0);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, vel);
    Ok(())
}

// ---------------------------------------------------------------------------
// SPCH_DequeueChannel (0x3d7ec8)

/// The speech queue (0x44fff8): per channel a pending word; request `i`'s
/// 4-byte key at `+0x74 + 0x60 i` and the next request's index (`lh`, -1
/// ends) at `+0x6e + 0x60 i`. The accept callback is the word at 0x44ff9c.
pub(crate) const SDC_QUEUE: u32 = 0x0044_fff8;

pub(crate) const SDC_REQUESTS: u32 = SDC_QUEUE + 0x74;

pub(crate) const SDC_NEXT: u32 = 0x6e;

pub(crate) const SDC_REQUEST_SIZE: u64 = 0x60;

pub(crate) const SDC_CALLBACKS: u32 = 0x0044_ff90;

pub(crate) const SDC_SELECT: Callee = Callee {
    addr: 0x003d_7a50,
    f: ee::s14::SPCH_SelectRequestForChannel,
};

pub(crate) const SDC_FREE_SLOT: Callee = Callee {
    addr: 0x003d_7c38,
    f: ee::s14::SPCH_FreeQueueSlot,
};

pub(crate) const SDC_PURGE: Callee = Callee {
    addr: 0x003d_7d58,
    f: ee::s14::SPCH_PurgeChannelRequests,
};

pub(crate) const SDC_RESET_SENTENCE: Callee = Callee {
    addr: 0x003d_9a40,
    f: ee::s14::SPCH_ResetSentence,
};

pub(crate) const SDC_BUILD_SENTENCE: Callee = Callee {
    addr: 0x003d_a418,
    f: ee::s14::SPCH_BuildSentenceWithFallback,
};

/// `int SPCH_DequeueChannel(int channel)` (0x3d7ec8): if the channel has
/// pending requests, pick requests until the accept callback (if any)
/// takes one (rejected ones are freed); none left returns 0. Then purge
/// the channel's other requests, reset the sentence and build it from the
/// accepted request and its followers (until one fails or the chain
/// ends), free the request and return the last build result.
pub fn spch_dequeue_channel(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = 0x45_0000;
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S3, 0x40);
        frame.save(c, S2, 0x30);
        c.g[A0][0] = sx(SDC_QUEUE);
        frame.save(c, RA, 0x90);
        c.g[V1][0] = sx(r32(c, S0) << 2);
        frame.save(c, S7, 0x80);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(SDC_QUEUE));
        frame.save(c, S6, 0x70);
        c.g[S3][0] = 0;
        frame.save(c, S5, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S1, 0x20);
        c.g[V0][0] = Addr(r32(c, V1)).lw(c);
        c.g[S2][0] = 0;
        'out: {
            if c.g[V0][0] == 0 {
                break 'out;
            }
            c.cycles += 5;
            c.g[V0][0] = 0x45_0000;
            c.g[S7][0] = sx(SDC_REQUESTS);
            c.g[S6][0] = sx(SDC_CALLBACKS);
            c.g[S5][0] = SDC_REQUEST_SIZE;
            c.g[S4][0] = 1;
            // Pick a request the callback accepts (s1).
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                SDC_SELECT.call(c, 0x003d_7f30)?;
                c.g[S1][0] = c.g[V0][0];
                let none = (c.g[S1][0] as i64) < 0;
                c.mult(0, V1, S1, S5);
                if none {
                    break 'out;
                }
                c.cycles += 8;
                c.g[A0][0] = 1;
                c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, S7)));
                c.g[V1][0] = Addr(r32(c, S6)).off(0xc).lw(c);
                let key = r32(c, V0);
                let s = sp(c);
                c.lwl(A1, key.wrapping_add(3));
                c.lwr(A1, key);
                c.swl(A1, s + 3);
                c.swr(A1, s);
                if c.g[V1][0] != 0 {
                    c.cycles += 3;
                    let accept = r32(c, V1);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, accept, 0x003d_7f64)?;
                    c.g[A0][0] = c.g[V0][0];
                }
                c.cycles += 2;
                if c.g[A0][0] == c.g[S4][0] {
                    c.cycles += 2;
                    c.g[S2][0] = 1;
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S1][0];
                    SDC_FREE_SLOT.call(c, 0x003d_7f80)?;
                }
                c.cycles += 2;
                if c.g[S2][0] != 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_7f28);
                }
            }

            // Build the sentence along the request chain.
            c.cycles += 8;
            c.g[A0][0] = c.g[S1][0];
            SDC_PURGE.call(c, 0x003d_7f90)?;
            c.g[S0][0] = c.g[S1][0];
            SDC_RESET_SENTENCE.call(c, 0x003d_7f98)?;
            c.g[S5][0] = SDC_REQUEST_SIZE;
            c.g[V0][0] = 0x45_0000;
            c.g[S4][0] = u64::MAX;
            c.g[S2][0] = sx(SDC_QUEUE);
            loop {
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                SDC_BUILD_SENTENCE.call(c, 0x003d_7fb0)?;
                c.mult(0, A0, S0, S5);
                c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, S2)));
                c.g[S0][0] = Addr(r32(c, V1)).off(SDC_NEXT).lh(c);
                c.g[S3][0] = c.g[V0][0];
                if c.g[S0][0] == c.g[S4][0] {
                    break;
                }
                c.cycles += 2;
                if c.g[S3][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_7fa8);
                }
            }
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            SDC_FREE_SLOT.call(c, 0x003d_7fd4)?;
        }
        c.cycles += 12;
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, RA, 0x90);
        frame.restore(c, S7, 0x80);
        frame.restore(c, S6, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ComputerActorRider_ScorePathBranch (0x10d410)

/// Path flags (`path + 0x38`): solo (bit 0), the three class bits (1-3),
/// level (bits 4-6: 0 = top speed limit, 3 = minimum speed), speed class
/// (bits 7-9; above 8 counts as 0).
pub(crate) const SPB_PATH_FLAGS: u32 = 0x38;

/// Computer fields: the rider (+0x18), the solo-path permission (+0xe04),
/// the random-bonus switch (+0xe08). Rider fields: position (+0x110),
/// velocity (+0x1e0), chasing flag (+0xf0) and target (+0xf8), the
/// rider's current path (+0xab8).
pub(crate) const SPB_RIDER: u32 = 0x18;

pub(crate) const SPB_SOLO_OK: u32 = 0xe04;

pub(crate) const SPB_RANDOM_BONUS: u32 = 0xe08;

pub(crate) const SPB_R_POS: u32 = 0x110;

pub(crate) const SPB_R_VEL: u32 = 0x1e0;

pub(crate) const SPB_R_CHASING: u32 = 0xf0;

pub(crate) const SPB_R_TARGET: u32 = 0xf8;

pub(crate) const SPB_R_PATH: u32 = 0xab8;

pub(crate) const SPB_CLASS_WEIGHT: Callee = Callee {
    addr: 0x0010_d870,
    f: ee::s00::ComputerActorRider_GetPathClassWeight,
};

pub(crate) const SPB_LOOK_AHEAD: Callee = Callee {
    addr: 0x0011_2588,
    f: ee::s00::Rider_GetAIPathLookAheadDist,
};

pub(crate) const SPB_CLOSEST_POINT: u32 = 0x0026_a428;

pub(crate) const SPB_COUNT_RIDERS: Callee = Callee {
    addr: 0x0010_fc30,
    f: ee::s00::AI_CountRidersOnPath,
};

pub(crate) const SPB_RAND: u32 = 0x0031_7810;

/// `|*v|` with the game's VU0 length macro (`vsqrt`, Q moved to GPR
/// `gpr`).
fn spb_length(c: &mut Cpu, v: u32, gpr: usize) {
    c.lqc2(3, v);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(gpr, 22);
}

/// The speed class's limit (20/40/60/80/100/120 for classes 1-6, else
/// 0) into v1, as the level-0 compare chain computes it (s0 = class).
fn spb_limit_top(c: &mut Cpu) {
    let class = c.g[S0][0];
    c.cycles += 3;
    c.g[V0][0] = 1;
    for (k, limit) in [(1u64, 0x14u64), (2, 0x28), (3, 0x3c), (4, 0x50)] {
        if k > 1 {
            c.cycles += 2;
        }
        c.g[V0][0] = k + 1;
        if class == k {
            c.cycles += 2;
            c.g[V1][0] = limit;
            return;
        }
    }
    c.cycles += 2;
    c.g[V1][0] = 0x64;
    if class == 5 {
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = class ^ 6;
    c.g[V1][0] = if c.g[V0][0] != 0 { 0 } else { 0x78 };
}

/// The same limits as the level-3 compare chain computes them (s0 =
/// class, s2 = 3, v0 = 1 on entry).
fn spb_limit_min(c: &mut Cpu) {
    let class = c.g[S0][0];
    c.cycles += 2;
    c.g[V0][0] = 2;
    if class == 1 {
        c.cycles += 2;
        c.g[V1][0] = 0x14;
        return;
    }
    c.cycles += 2;
    c.g[V1][0] = 0x28;
    if class == 2 {
        return;
    }
    for (k, limit) in [(c.g[S2][0], 0x3cu64), (4, 0x50)] {
        c.cycles += 2;
        c.g[V0][0] = if limit == 0x3c { 4 } else { 5 };
        if class == k {
            c.cycles += 2;
            c.g[V1][0] = limit;
            return;
        }
    }
    c.cycles += 2;
    c.g[V1][0] = 0x64;
    if class == 5 {
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = class ^ 6;
    c.g[V1][0] = if c.g[V0][0] != 0 { 0 } else { 0x78 };
}

/// `float ComputerActorRider_ScorePathBranch(ComputerActorRider* self,
/// Path* path)` (0x10d410): how much the computer rider wants to take
/// `path` at a branch. 0 when its class weight is 0, a solo path he may
/// not take, or his speed (`|vel| * k`) is above the path's top limit
/// (level 0) or below its minimum (level 3); -1 when the path is more
/// than 5000 away from where he will be; 100 when the rider he chases is
/// on it. Otherwise the sum of 7 terms over 22: closeness, the level / 3,
/// crowding (100 / 25 / 0 by the other riders on it), the class weight, a
/// solo bonus and a random bonus.
pub fn computer_actor_rider_score_path_branch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0xf0);
        c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
        frame.save(c, S0, 0xc0);
        frame.save(c, S1, 0xb0);
        frame.save(c, S2, 0xa0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x90);
        frame.save(c, S5, 0x70);
        c.g[S3][0] = c.g[A1][0];
        frame.save_fpr(c, 23, 0xe8);
        frame.save(c, S4, 0x80);
        frame.save_ra(c, 0x60);
        frame.save_fpr(c, 22, 0xe0);
        frame.save_fpr(c, 21, 0xd8);
        frame.save_fpr(c, 20, 0xd0);
        // Unpack the path flags.
        c.g[A3][0] = Addr(r32(c, S3)).off(SPB_PATH_FLAGS).lw(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x84).lw(c);
        let flags = r32(c, A3) as i32;
        c.g[V0][0] = sx((flags >> 7) as u32);
        c.g[A1][0] = sx((flags >> 3) as u32);
        c.g[S0][0] = c.g[V0][0] & 7;
        c.g[A2][0] = sx((flags >> 2) as u32);
        c.g[V0][0] = sx((flags >> 4) as u32);
        c.g[S5][0] = c.g[A3][0] & 1;
        c.g[S2][0] = c.g[V0][0] & 7;
        c.g[A3][0] = sx((flags >> 1) as u32);
        c.g[V0][0] = ((c.g[S0][0] as i64) < 9) as u64;
        c.g[S4][0] = Addr(r32(c, V1)).off(0xc).lw(c);
        if c.g[V0][0] == 0 {
            c.g[S0][0] = 0;
        }
        c.g[A1][0] &= 1;
        c.g[A2][0] &= 1;
        c.g[A3][0] &= 1;
        SPB_CLASS_WEIGHT.call(c, 0x0010_d490)?;
        c.mov_s(23, 0);
        c.mtc1(0, 0);
        c.c_le_s(23, 0);
        'exit_a: {
            if c.fcc() {
                frame.restore(c, S0, 0xc0);
                break 'exit_a;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S1)).off(SPB_SOLO_OK).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 2;
                if c.g[S5][0] != 0 {
                    frame.restore(c, S0, 0xc0);
                    break 'exit_a;
                }
                c.cycles += 1;
            }
            c.g[V0][0] = Addr(r32(c, S1)).off(SPB_RIDER).lw(c);

            // speed = |vel| * k against the class limit.
            c.cycles += 14;
            c.lwc1(1, r32(c, GP).wrapping_sub(0x7cfc));
            let vel = r32(c, V0).wrapping_add(SPB_R_VEL);
            spb_length(c, vel, V1);
            c.mtc1(V1, 0);
            c.g[A1][0] = c.g[V0][0];
            c.mul_s(1, 0, 1);
            let mut check = true;
            if c.g[S2][0] == 0 {
                spb_limit_top(c);
                c.cycles += 5;
                c.mtc1(V1, 0);
                c.cvt_s_w(0, 0);
                c.c_lt_s(0, 1);
            } else {
                c.cycles += 3;
                c.g[V0][0] = 3;
                let level3 = c.g[S2][0] == c.g[V0][0];
                c.g[V0][0] = 1;
                if level3 {
                    spb_limit_min(c);
                    c.cycles += 4;
                    c.mtc1(V1, 0);
                    c.cvt_s_w(0, 0);
                    c.c_lt_s(1, 0);
                } else {
                    check = false;
                }
            }
            if check {
                c.cycles += 2;
                let out = c.fcc();
                frame.restore(c, S0, 0xc0);
                if out {
                    c.cycles += 3;
                    c.mtc1(0, 0);
                    frame.restore(c, S1, 0xb0);
                    return spb_epilogue(c, frame);
                }
            }
            spb_score(c, frame)?;
        }
        c.cycles += 1;
        frame.restore(c, S1, 0xb0);
        spb_epilogue(c, frame)
    })
}

/// The shared epilogue of `ComputerActorRider_ScorePathBranch` (s0 and s1
/// already restored).
fn spb_epilogue(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 11;
    frame.restore(c, S2, 0xa0);
    frame.restore(c, S3, 0x90);
    frame.restore(c, S4, 0x80);
    frame.restore(c, S5, 0x70);
    frame.restore_ra(c, 0x60);
    frame.restore_fpr(c, 23, 0xe8);
    frame.restore_fpr(c, 22, 0xe0);
    frame.restore_fpr(c, 21, 0xd8);
    frame.restore_fpr(c, 20, 0xd0);
    frame.close(c);
    Ok(())
}

/// The scoring part of `ComputerActorRider_ScorePathBranch` (a1 = the
/// rider, f23 = the class weight, s2 = level, s3 = path, s4 = the AI,
/// s5 = solo). Ends with s0 restored and f0 = the score.
fn spb_score(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 49;
    let vel = r32(c, A1).wrapping_add(SPB_R_VEL);
    spb_length(c, vel, V0);
    c.mtc1(V0, 20);
    c.g[A0][0] = c.g[A1][0];
    c.g[AT][0] = 0x41b0_0000;
    c.mtc1(AT, 22);
    c.g[A1][0] = 1;
    SPB_LOOK_AHEAD.call(c, 0x0010_d61c)?;
    // ahead = pos + vel * (look_ahead / |vel|); d = distance to the path.
    c.g[V0][0] = Addr(r32(c, S1)).off(SPB_RIDER).lw(c);
    c.div_s(0, 0, 20);
    let s = sp(c);
    let vel = r32(c, V0).wrapping_add(SPB_R_VEL);
    racw_scale(c, V1, 0, vel, s + 0x40);
    c.g[A0][0] = c.g[S3][0];
    c.lq(V0, s + 0x40);
    c.g[A1][0] = sx(s + 0x20);
    c.g[V1][0] = Addr(r32(c, S1)).off(SPB_RIDER).lw(c);
    c.g[A2][0] = sx(s + 0x30);
    c.sq(V0, s + 0x30);
    c.g[A3][0] = sx(s + 0x50);
    c.lqc2(3, r32(c, V1).wrapping_add(SPB_R_POS));
    c.lqc2(4, s + 0x30);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s + 0x40);
    c.g[T0][0] = 1;
    c.lq(V0, s + 0x40);
    c.sq(V0, s + 0x20);
    call_ptr(c, SPB_CLOSEST_POINT, 0x0010_d674)?;
    // closeness = max(100 - d * d * k, 0)
    let s = sp(c);
    c.lwc1(2, s + 0x50);
    c.lwc1(1, r32(c, GP).wrapping_sub(0x7cf8));
    c.mul_s(0, 2, 2);
    c.g[AT][0] = 0x42c8_0000;
    c.mtc1(AT, 4);
    c.mtc1(0, 3);
    c.mul_s(0, 0, 1);
    c.sub_s(1, 4, 0);
    c.c_lt_s(1, 3);
    if c.fcc() {
        c.mov_s(1, 3);
    }
    c.cycles += 6;
    c.lwc1(0, r32(c, GP).wrapping_sub(0x7cf4));
    c.c_lt_s(0, 2);
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(AT, 0);
    let too_far = c.fcc();
    frame.restore(c, S0, 0xc0);
    if too_far {
        return Ok(());
    }
    // sc[0]; chasing a rider on this path: 100.
    c.cycles += 7;
    c.lwc1(0, r32(c, GP).wrapping_sub(0x7cf0));
    c.g[V1][0] = Addr(r32(c, S1)).off(SPB_RIDER).lw(c);
    c.mul_s(0, 1, 0);
    c.swc1(0, s);
    c.g[V0][0] = Addr(r32(c, V1)).off(SPB_R_CHASING).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 7;
        c.g[V0][0] = Addr(r32(c, V1)).off(SPB_R_TARGET).lw(c);
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
        c.g[V1][0] = Addr(r32(c, V0)).off(0x28).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).off(SPB_R_PATH).lw(c);
        let on_path = c.g[A0][0] == c.g[S3][0];
        c.mov_s(0, 4);
        if on_path {
            return Ok(());
        }
    }
    // sc[1] = 0, sc[2] = level * (1/22) * k; crowding.
    c.cycles += 23;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 2);
    c.mtc1(S2, 1);
    c.cvt_s_w(1, 1);
    c.lwc1(0, r32(c, GP).wrapping_sub(0x7cec));
    c.g[A0][0] = c.g[S4][0];
    c.div_s(20, 2, 22);
    c.g[A1][0] = c.g[S3][0];
    c.mtc1(0, 21);
    c.swc1(21, s + 4);
    c.mul_s(0, 20, 0);
    c.mul_s(1, 1, 0);
    c.swc1(1, s + 8);
    SPB_COUNT_RIDERS.call(c, 0x0010_d72c)?;
    c.g[A0][0] = Addr(r32(c, S1)).off(SPB_RIDER).lw(c);
    c.g[V1][0] = Addr(r32(c, A0)).off(SPB_R_PATH).lw(c);
    c.g[V1][0] ^= c.g[S3][0];
    c.g[V1][0] = (c.g[V1][0] < 1) as u64;
    c.g[V1][0] = sx(r32(c, V0).wrapping_sub(r32(c, V1)));
    c.g[AT][0] = 0x42c8_0000;
    c.mtc1(AT, 1);
    let others = c.g[V1][0] as i64;
    c.g[V0][0] = 1;
    if others > 0 {
        c.cycles += 2;
        c.mov_s(1, 21);
        if others == 1 {
            c.cycles += 2;
            c.g[AT][0] = 0x41c8_0000;
            c.mtc1(AT, 1);
        }
    }
    // sc[3] = crowding * 5/22, sc[4] = weight / 22, sc[5] = solo bonus.
    c.cycles += 11;
    let s = sp(c);
    c.g[AT][0] = 0x40a0_0000;
    c.mtc1(AT, 0);
    c.g[V0][0] = Addr(r32(c, S1)).off(SPB_SOLO_OK).lw(c);
    c.div_s(20, 0, 22);
    c.mul_s(1, 1, 20);
    c.div_s(20, 23, 22);
    c.swc1(1, s + 0xc);
    c.swc1(20, s + 0x10);
    c.div_s(20, 0, 22);
    let mut solo = c.g[V0][0] != 0;
    if solo {
        c.cycles += 2;
        solo = c.g[S5][0] != 0;
    }
    if solo {
        c.cycles += 4;
        c.g[AT][0] = 0x42c8_0000;
        c.mtc1(AT, 0);
        c.mul_s(0, 20, 0);
        c.swc1(0, s + 0x14);
    } else {
        Addr(s + 0x14).sw(c, 0);
    }
    // sc[6] = random % 100 * 2/22.
    c.cycles += 6;
    c.g[AT][0] = 0x4000_0000;
    c.mtc1(AT, 0);
    c.g[V0][0] = Addr(r32(c, S1)).off(SPB_RANDOM_BONUS).lw(c);
    c.div_s(20, 0, 22);
    if c.g[V0][0] == 0 {
        Addr(s + 0x18).sw(c, 0);
    } else {
        c.cycles += 6;
        call_ptr(c, SPB_RAND, 0x0010_d7c4)?;
        c.g[V1][0] = 0x64;
        c.divu(0, V0, V1);
        c.cycles += 3;
        c.g[V0][0] = c.hi[0];
        let negative = (c.g[V0][0] as i64) < 0;
        c.g[V1][0] = sx(r32(c, V0) >> 1);
        if negative {
            c.cycles += 6;
            c.g[V0][0] &= 1;
            c.g[V0][0] |= c.g[V1][0];
            c.mtc1(V0, 0);
            c.cvt_s_w(0, 0);
            c.add_s(0, 0, 0);
            c.mul_s(0, 0, 20);
        } else {
            c.cycles += 4;
            c.mtc1(V0, 0);
            c.cvt_s_w(0, 0);
            c.mul_s(0, 0, 20);
        }
        c.cycles += 1;
        c.swc1(0, sp(c) + 0x18);
    }
    // The sum of the 7 terms.
    c.cycles += 3;
    c.mtc1(0, 1);
    c.g[V0][0] = c.g[SP][0];
    c.g[V1][0] = 6;
    loop {
        c.cycles += 7;
        c.lwc1(0, r32(c, V0));
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
        let more = (c.g[V1][0] as i64) >= 0;
        c.add_s(1, 1, 0);
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0010_d818);
        }
    }
    c.cycles += 2;
    c.mov_s(0, 1);
    frame.restore(c, S0, 0xc0);
    Ok(())
}

// ---------------------------------------------------------------------------
// RiderWipeoutSub_UpdatePhase2 (0x12d160)

/// Wipeout sub-state fields: the state (+0), the bounce request (+0x54)
/// and its velocity (+0x60), the get-up progress (+0x70), the rider
/// (+0x80). Rider fields: velocity, the "is AI" flag (+0x150), the
/// surface (+0x438), the ground timer (+0x470), vtable (+0x6c0; slots 0x88
/// bounce, 0x90 anim speed), handler (+0x77c; +0x30 the anim set), anim
/// controller (+0x784), camera index (+0x870) and shake flag (+0x87c).
pub(crate) const UP2_STATE: u32 = 0x0;

pub(crate) const UP2_BOUNCE: u32 = 0x54;

pub(crate) const UP2_BOUNCE_VEL: u32 = 0x60;

pub(crate) const UP2_PROGRESS: u32 = 0x70;

pub(crate) const UP2_RIDER: u32 = 0x80;

pub(crate) const UP2_R_VEL: u32 = 0x1e0;

pub(crate) const UP2_R_IS_AI: u32 = 0x150;

pub(crate) const UP2_R_SURFACE: u32 = 0x438;

pub(crate) const UP2_R_GROUND_TIME: u32 = 0x470;

pub(crate) const UP2_R_VTABLE: u32 = 0x6c0;

pub(crate) const UP2_R_HANDLER: u32 = 0x77c;

pub(crate) const UP2_R_ANIM: u32 = 0x784;

pub(crate) const UP2_R_CAMERA: u32 = 0x870;

pub(crate) const UP2_R_SHAKE: u32 = 0x87c;

pub(crate) const UP2_ADD_SHAKE: u32 = 0x0015_e360;

pub(crate) const UP2_ANIM_SPEED: Callee = Callee {
    addr: 0x0012_e528,
    f: ee::s00::RiderWipeoutSub_UpdateAnimSpeed,
};

pub(crate) const UP2_START_GET_UP: Callee = Callee {
    addr: 0x0012_e010,
    f: ee::s00::RiderWipeoutSub_StartGetUp,
};

pub(crate) const UP2_LAYER_FINISHED: u32 = 0x0031_2ae8;

pub(crate) const UP2_LAYER_ENTRY: u32 = 0x0031_1ae8;

pub(crate) const UP2_CHECK_RESET: Callee = Callee {
    addr: 0x0011_6120,
    f: ee::s00::Rider_CheckResetPath,
};

pub(crate) const UP2_RECOVERY_BOOST: Callee = Callee {
    addr: 0x0010_f280,
    f: ee::s00::Rider_AddWipeoutRecoveryBoost,
};

pub(crate) const UP2_AUDIO: u32 = 0x0028_b180;

pub(crate) const UP2_GRUNT: u32 = 0x0029_f660;

pub(crate) const UP2_END_ANIM_A: Callee = Callee {
    addr: 0x0012_df48,
    f: ee::s00::RiderWipeoutSub_PlayEndAnimA,
};

pub(crate) const UP2_ANIM_SET_B: Callee = Callee {
    addr: 0x0012_dcb0,
    f: ee::s00::RiderWipeoutSub_SwitchToAnimSetB,
};

pub(crate) const UP2_STOP_LOOP: u32 = 0x0029_6e20;

pub(crate) const UP2_STOP_LOOP2: u32 = 0x0029_7438;

pub(crate) const UP2_ANIM_SET_A: Callee = Callee {
    addr: 0x0012_dd98,
    f: ee::s00::RiderWipeoutSub_SwitchToAnimSetA,
};

/// `self->rider` (s0 = self).
fn up2_rider(c: &mut Cpu) -> u64 {
    Addr(r32(c, S0)).off(UP2_RIDER).lw(c)
}

/// `rider->anim->GetLayerAnimEntry(2)` into v0 (a0 from the rider in
/// `rider_reg`).
fn up2_layer_entry(c: &mut Cpu, rider_reg: usize, ret: u32) -> crate::call::Flow {
    c.g[A0][0] = Addr(r32(c, rider_reg)).off(UP2_R_ANIM).lw(c);
    call_ptr(c, UP2_LAYER_ENTRY, ret)
}

/// `void RiderWipeoutSub_UpdatePhase2(RiderWipeoutSub* self)` (0x12d160):
/// the second wipeout phase. Shake the rider's camera by his speed (0..1
/// of 100 km/h), update the anim speed (half of it to the rider), and
/// serve a bounce request. An AI rider slower than k gets up (state 4).
/// Once the layer-2 anim is done (or is 0x1d / 0x19): a rider who is
/// fast enough to recover (progress >= 1 or on the ground long enough),
/// or a human slower than the limit (k2, k3 for anim 0x1c), recovers: on
/// surface 0x12 or as an AI, a fast one resets onto the path with a boost
/// and a slow one gets up (state 4); a human otherwise gets a boost (a
/// grunt for anim 0x1c) and the end anim (state 3). Else anim set 1
/// switches to set B (state 1), and a finished layer to set A. The
/// recover paths stop the two wipeout sound loops.
pub fn rider_wipeout_sub_update_phase2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save_fpr(c, 21, 0x38);
        frame.save_fpr(c, 20, 0x30);
        c.g[V0][0] = up2_rider(c);
        let vel = r32(c, V0).wrapping_add(UP2_R_VEL);
        spb_length(c, vel, A0);
        c.mtc1(A0, 20);
        c.g[V1][0] = Addr(r32(c, V0)).off(UP2_R_CAMERA).lw(c);
        if (c.g[V1][0] as i64) >= 0 {
            // t = clamp(speed * k / 100, 0, 1)
            c.cycles += 9;
            c.lwc1(0, r32(c, GP).wrapping_sub(0x77c0));
            c.mtc1(0, 12);
            c.mul_s(0, 20, 0);
            c.g[AT][0] = 0x42c8_0000;
            c.mtc1(AT, 1);
            c.c_le_s(12, 0);
            let positive = c.fcc();
            c.g[V1][0] = up2_rider(c);
            if positive {
                c.cycles += 5;
                c.c_le_s(0, 1);
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(AT, 12);
                if c.fcc() {
                    c.cycles += 1;
                    c.div_s(12, 0, 1);
                }
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, V1)).off(UP2_R_SHAKE).lw(c);
            let shake = c.g[V0][0] != 0;
            c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
            if shake {
                // world->game->cameras[rider->camera]->AddShake(4, t, 0)
                c.cycles += 9;
                c.g[A1][0] = 4;
                c.g[V1][0] = Addr(r32(c, V1)).off(UP2_R_CAMERA).lw(c);
                c.g[A0][0] = Addr(r32(c, V0)).off(0x84).lw(c);
                c.g[V1][0] = sx(r32(c, V1) << 2);
                c.mtc1(0, 13);
                c.g[V0][0] = Addr(r32(c, A0)).off(0x84).lw(c);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                c.g[A0][0] = Addr(r32(c, V0)).off(4).lw(c);
                call_ptr(c, UP2_ADD_SHAKE, 0x0012_d220)?;
            }
        }

        // rider->SetAnimSpeed(UpdateAnimSpeed() / 2) (vtable slot 0x90)
        c.cycles += 14;
        c.g[A0][0] = c.g[S0][0];
        UP2_ANIM_SPEED.call(c, 0x0012_d228)?;
        c.g[A1][0] = up2_rider(c);
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(AT, 12);
        c.g[V0][0] = Addr(r32(c, A1)).off(UP2_R_VTABLE).lw(c);
        c.mul_s(12, 0, 12);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x90).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x94).lw(c);
        let method = r32(c, V1);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x0012_d24c)?;
        'exit: {
            c.g[V0][0] = Addr(r32(c, S0)).off(UP2_BOUNCE).lw(c);
            if c.g[V0][0] != 0 {
                // rider->Bounce(|bounce velocity|) (vtable slot 0x88)
                c.cycles += 19;
                let this = r32(c, S0);
                Addr(this).off(UP2_BOUNCE).sw(c, 0);
                c.g[A1][0] = up2_rider(c);
                spb_length(c, this.wrapping_add(UP2_BOUNCE_VEL), V0);
                c.mtc1(V0, 12);
                c.g[V0][0] = Addr(r32(c, A1)).off(UP2_R_VTABLE).lw(c);
                c.g[A0][0] = Addr(r32(c, V0)).off(0x88).lh(c);
                c.g[V1][0] = Addr(r32(c, V0)).off(0x8c).lw(c);
                let method = r32(c, V1);
                c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                call_ptr(c, method, 0x0012_d29c)?;
                frame.restore(c, S0, 0x20);
                break 'exit;
            }
            c.g[V1][0] = up2_rider(c);
            let tail = up2_decide(c, frame)?;
            if tail {
                up2_stop_loops(c)?;
            }
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// The recover paths' tail: stop the two wipeout sound loops.
fn up2_stop_loops(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 12;
    call_ptr(c, UP2_AUDIO, 0x0012_d48c)?;
    c.g[A1][0] = up2_rider(c);
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, UP2_STOP_LOOP, 0x0012_d498)?;
    call_ptr(c, UP2_AUDIO, 0x0012_d4a0)?;
    c.g[A1][0] = up2_rider(c);
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, UP2_STOP_LOOP2, 0x0012_d4ac)
}

/// The decision part of `RiderWipeoutSub_UpdatePhase2` (v1 = the rider,
/// f20 = speed). Ok(true): continue with the sound-loop tail; Ok(false):
/// straight to the exit (s0 is restored by the caller either way).
fn up2_decide(c: &mut Cpu, _frame: Frame) -> Result<bool, crate::call::Unwound> {
    // An AI rider slower than k gets up.
    c.cycles += 4;
    c.g[V0][0] = Addr(r32(c, V1)).off(UP2_R_IS_AI).lw(c);
    c.g[V0][0] ^= 1;
    if c.g[V0][0] == 0 {
        c.cycles += 5;
        c.lwc1(0, r32(c, GP).wrapping_sub(0x77bc));
        c.c_lt_s(20, 0);
        if c.fcc() {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            UP2_START_GET_UP.call(c, 0x0012_d2d0)?;
            c.g[V0][0] = 4;
            Addr(r32(c, S0)).sw(c, 4);
            return Ok(true);
        }
    }
    c.g[V0][0] = up2_rider(c);

    // Wait for layer 2 (anims 0x1d and 0x19 go on at once).
    c.cycles += 5;
    c.g[A1][0] = 2;
    c.g[A0][0] = Addr(r32(c, V0)).off(UP2_R_ANIM).lw(c);
    call_ptr(c, UP2_LAYER_FINISHED, 0x0012_d2e8)?;
    let finished = c.g[V0][0] != 0;
    c.g[V0][0] = up2_rider(c);
    if !finished {
        c.cycles += 6;
        c.g[A1][0] = 2;
        up2_layer_entry(c, V0, 0x0012_d2fc)?;
        c.g[V1][0] = 0x1d;
        let go = c.g[V0][0] == c.g[V1][0];
        c.g[A1][0] = 2;
        if !go {
            c.cycles += 6;
            c.g[V0][0] = up2_rider(c);
            up2_layer_entry(c, V0, 0x0012_d314)?;
            c.g[V1][0] = 0x19;
            if c.g[V0][0] != c.g[V1][0] {
                return Ok(false);
            }
        }
        c.cycles += 1;
        c.g[V0][0] = up2_rider(c);
    }

    // The speed limit (k2, or k3 for anim 0x1c).
    c.cycles += 7;
    c.g[A1][0] = 2;
    c.lwc1(21, r32(c, GP).wrapping_sub(0x77b8));
    up2_layer_entry(c, V0, 0x0012_d334)?;
    c.g[V1][0] = 0x1c;
    let anim_1c = c.g[V0][0] == c.g[V1][0];
    c.lwc1(1, r32(c, S0).wrapping_add(UP2_PROGRESS));
    if anim_1c {
        c.cycles += 1;
        c.lwc1(21, r32(c, GP).wrapping_sub(0x77b4));
    }

    // a1 = fast: progress >= 1, or the ground timer >= 0.
    c.cycles += 7;
    c.g[A1][0] = 0;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 0);
    c.c_le_s(0, 1);
    let mut fast = c.fcc();
    c.g[A0][0] = up2_rider(c);
    let rider = r32(c, A0);
    if !fast {
        c.cycles += 6;
        c.lwc1(1, rider.wrapping_add(UP2_R_GROUND_TIME));
        c.mtc1(0, 0);
        c.c_le_s(0, 1);
        let on_ground = c.fcc();
        c.g[V0][0] = 1;
        if !on_ground {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 2;
        fast = c.g[V0][0] != 0;
    }
    if fast {
        c.cycles += 2;
        c.g[A1][0] = 1;
    }
    c.g[V0][0] = Addr(rider).off(UP2_R_IS_AI).lw(c);

    // Recover: a human slower than the limit, or fast.
    c.cycles += 3;
    c.g[V0][0] ^= 1;
    let mut recover = false;
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        c.c_lt_s(20, 21);
        if c.fcc() {
            c.g[V1][0] = Addr(rider).off(UP2_R_SURFACE).lw(c);
            recover = true;
        }
    }
    if !recover {
        c.cycles += 2;
        if c.g[A1][0] == 0 {
            // Anim set 1 switches to set B; a finished layer to set A.
            c.g[V0][0] = Addr(rider).off(UP2_R_HANDLER).lw(c);
            c.cycles += 4;
            c.g[V1][0] = 1;
            c.g[S1][0] = Addr(r32(c, V0)).off(0x30).lw(c);
            if c.g[S1][0] == c.g[V1][0] {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                UP2_ANIM_SET_B.call(c, 0x0012_d480)?;
                let set = c.g[S1][0];
                Addr(r32(c, S0)).sw(c, set);
                return Ok(true);
            }
            c.g[A0][0] = Addr(rider).off(UP2_R_ANIM).lw(c);
            c.cycles += 4;
            c.g[A1][0] = 2;
            call_ptr(c, UP2_LAYER_FINISHED, 0x0012_d4bc)?;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                UP2_ANIM_SET_A.call(c, 0x0012_d4cc)?;
            }
            return Ok(false);
        }
        c.cycles += 1;
        c.g[V1][0] = Addr(rider).off(UP2_R_SURFACE).lw(c);
    }

    // On surface 0x12 or as an AI: reset (fast) or get up (slow).
    c.cycles += 3;
    c.g[V0][0] = 0x12;
    let mut special = c.g[V1][0] == c.g[V0][0];
    if !special {
        c.cycles += 4;
        c.g[V0][0] = Addr(r32(c, A0)).off(UP2_R_IS_AI).lw(c);
        c.g[V0][0] ^= 1;
        special = c.g[V0][0] == 0;
    }
    if special {
        c.cycles += 2;
        let was_fast = c.g[A1][0] != 0;
        c.g[A1][0] = 0;
        if was_fast {
            c.cycles += 7;
            c.g[A2][0] = 3;
            UP2_CHECK_RESET.call(c, 0x0012_d3e0)?;
            c.g[A0][0] = up2_rider(c);
            c.g[A1][0] = 1;
            UP2_RECOVERY_BOOST.call(c, 0x0012_d3ec)?;
            return Ok(true);
        }
        c.cycles += 4;
        c.lwc1(0, r32(c, GP).wrapping_sub(0x77b0));
        c.c_lt_s(20, 0);
        if !c.fcc() {
            return Ok(true);
        }
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        UP2_START_GET_UP.call(c, 0x0012_d40c)?;
        c.g[V0][0] = 4;
    } else {
        // A human: boost (a grunt for anim 0x1c), the end anim, state 3.
        c.cycles += 9;
        UP2_RECOVERY_BOOST.call(c, 0x0012_d41c)?;
        c.g[V0][0] = up2_rider(c);
        c.g[A1][0] = 2;
        up2_layer_entry(c, V0, 0x0012_d42c)?;
        c.g[V1][0] = 0x1c;
        if c.g[V0][0] == c.g[V1][0] {
            c.cycles += 7;
            call_ptr(c, UP2_AUDIO, 0x0012_d440)?;
            c.g[A1][0] = up2_rider(c);
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = 1;
            c.g[A3][0] = 0;
            call_ptr(c, UP2_GRUNT, 0x0012_d454)?;
        }
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        UP2_END_ANIM_A.call(c, 0x0012_d45c)?;
        c.g[V0][0] = 3;
    }
    c.cycles += 2;
    let state = c.g[V0][0];
    Addr(r32(c, S0)).off(UP2_STATE).sw(c, state);
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine, FAKE};
    use crate::difftest::{check, elf_image, fill, put_vec, scramble, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    // ------------------------------------------- RiderAirState_CollideWalls

    static RACW_PREV_HANDLE_IMPACT: AtomicUsize = AtomicUsize::new(0);

    /// `Rider_HandleImpact` as a logged fake under the wipeout tests' gate
    /// (`PUA_CTRL`): their fakes already stand in for the hull, the sweep
    /// and `Rider_Translate`.
    fn racw_fake_handle_impact(c: &mut Cpu) {
        use crate::game::ai::motion::wipeoutmotion::tests::{pua_fake, PUA_CTRL, PUA_MAGIC};
        if c.bus.read32(PUA_CTRL) != PUA_MAGIC {
            return fakes::fall_back(c, &RACW_PREV_HANDLE_IMPACT, ee::s00::f_00105d98__generated);
        }
        pua_fake(c, RACW_HANDLE_IMPACT);
    }

    /// Sub-state 9 (nothing), no contact, a contact the rider moves away
    /// from, and a bounce (the push below and above its floor). The hull,
    /// the sweep, `Rider_Translate` and `Rider_HandleImpact` are logged
    /// fakes (one may unwind); `Rider_GetSubState` and
    /// `Rider_BuildWorldMatrix` run for real.
    #[test]
    fn rider_air_state_collide_walls_matches() {
        use crate::game::ai::motion::wipeoutmotion::tests::{
            pua_install_fakes, PUA_CTRL, PUA_HIT_A_TEMPLATE, PUA_MAGIC,
        };
        pua_install_fakes();
        {
            fakes::hook(
                std::ptr::addr_of_mut!(ee::s00::OVERRIDE_00105D98),
                racw_fake_handle_impact,
                &RACW_PREV_HANDLE_IMPACT,
            );
        }
        covered(
            ee::s01::f_0013aa48__generated,
            rider_air_state_collide_walls,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x0013_aa48, 6);
                c.pc = 0x0013_aa48;
                let state = DATA + 0x6_0000;
                let rider = DATA + 0x6_1000;
                let handler = DATA + 0x6_3000;
                let vt = DATA + 0x6_4000;
                let out = DATA + 0x6_5000;
                let pick = |r: &mut Rng, v: &[f32]| v[r.next_u32() as usize % v.len()].to_bits();
                let comps = [-12.0f32, -3.0, -1.0, -0.5, 0.0, 0.5, 1.0, 3.0, 12.0];
                let vec = |c: &mut Cpu, r: &mut Rng, at: u32| {
                    for i in 0..3 {
                        c.bus.write32(at + 4 * i, pick(r, &comps));
                    }
                    c.bus.write32(at + 12, pick(r, &[0.0, 1.0]));
                };
                c.bus.write32(PUA_CTRL, PUA_MAGIC);
                c.g[A0][0] = sx(state);
                c.g[A1][0] = sx(out);
                c.bus.write32(state + RACW_RIDER, rider);
                // Rider_GetSubState: rider->handler->sub_state.
                c.bus.write32(rider + 0x77c, handler);
                let sub = if seed % 6 == 0 { 9 } else { r.next_u32() % 12 };
                c.bus.write32(handler + 0xde4, sub);
                c.bus.write32(rider + RACW_R_VTABLE, vt);
                c.bus
                    .write16(vt + 0x40, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(vt + 0x44, FAKE);
                for off in [RACW_R_OVERLAP, RACW_R_WALL_HINT, RACW_R_SPHERE_TREE] {
                    c.bus.write32(rider + off, r.next_u32());
                }
                vec(c, &mut r, rider + RACW_R_CENTRE);
                vec(c, &mut r, rider + RACW_R_VEL);
                // Rider_BuildWorldMatrix's inputs (orientation, position).
                vec(c, &mut r, rider + 0x110);
                put_vec(
                    c,
                    rider + 0x120,
                    [r.f32(1.0), r.f32(1.0), r.f32(1.0), r.f32(1.0)],
                );
                // The contact distance and the hit.
                c.bus
                    .write32(PUA_CTRL + 4, pick(&mut r, &[-1.0, 0.0, 0.25, 2.0, 2.0]));
                fill(c, &mut r, PUA_HIT_A_TEMPLATE, 0x18);
                vec(c, &mut r, PUA_HIT_A_TEMPLATE + 0x10);
            },
        );
    }
    // ------------------------------------------- shared gated fakes

    /// Control block of this module's gated fakes: the magic word (without
    /// it they run the body they replaced), the `PopFront` queue (cursor at
    /// +0x10, results at +0x20, 0-terminated) and per-target results.
    pub(crate) const OC360_CTRL: u32 = DATA + 0x7_0000;

    pub(crate) const OC360_MAGIC: u32 = 0x0360_f00d;

    pub(crate) const OC360_POP_NEXT: u32 = OC360_CTRL + 0x10;

    pub(crate) const OC360_POP_QUEUE: u32 = OC360_CTRL + 0x20;

    /// What `AnimSeqList_PushAnim` returns, and the base of the sequences
    /// `AnimSeqList_GetAt` returns (0x100 apart).
    pub(crate) const OC360_NEW_SEQ: u32 = OC360_CTRL + 0x8;

    pub(crate) const OC360_SEQS: u32 = OC360_CTRL + 0xc;

    /// The contact distance and hit `Nash_FindClosestContact` returns.
    pub(crate) const OC360_DIST: u32 = OC360_CTRL + 0x40;

    pub(crate) const OC360_HIT: u32 = OC360_CTRL + 0x100;

    /// Scripted result queues: a cursor word, then 8 results (the last
    /// one repeats).
    pub(crate) const OC360_SELECT_QUEUE: u32 = OC360_CTRL + 0x200;

    pub(crate) const OC360_BUILD_QUEUE: u32 = OC360_CTRL + 0x240;

    pub(crate) fn oc360_next(c: &mut Cpu, queue: u32) -> u32 {
        let n = c.bus.read32(queue);
        c.bus.write32(queue, n + 1);
        c.bus.read32(queue + 4 + 4 * n.min(7))
    }

    /// What `AnimController_IsLayerAnimFinished` and
    /// `AnimController_GetLayerAnimEntry` return.
    pub(crate) const OC360_LAYER_FINISHED: u32 = OC360_CTRL + 0x4c;

    pub(crate) const OC360_LAYER_ENTRY: u32 = OC360_CTRL + 0x50;

    /// What `AI_CountRidersOnPath` returns.
    pub(crate) const OC360_COUNT: u32 = OC360_CTRL + 0x48;

    /// What `UIStateMgr_GetActiveState` returns.
    pub(crate) const OC360_ACTIVE_STATE: u32 = OC360_CTRL + 0x44;

    /// A logged fake (`fakes::fake`, which may unwind) with scripted
    /// answers.
    pub(crate) fn oc360_fake(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        let (a1, a2, a3) = (c.g[A1][0] as u32, c.g[A2][0] as u32, c.g[A3][0] as u32);
        fakes::fake(c, target, ra);
        if c.pc != ra {
            return;
        }
        match target {
            0x0030_b208 => {
                let n = c.bus.read32(OC360_POP_NEXT);
                c.bus.write32(OC360_POP_NEXT, n + 1);
                c.g[V0][0] = c.bus.read32(OC360_POP_QUEUE + 4 * n.min(7)) as u64;
            }
            // AnimSeqList_PushAnim / PushLoopingAnim: the new sequence.
            0x0031_4518 | 0x0031_4588 => c.g[V0][0] = c.bus.read32(OC360_NEW_SEQ) as u64,
            // AnimSeqList_GetAt(list, i): sequence i.
            0x0031_4760 => {
                c.g[V0][0] = (c.bus.read32(OC360_SEQS) + 0x100 * (a1 & 7)) as u64;
            }
            // SPCH_SelectRequestForChannel / SPCH_BuildSentenceWithFallback:
            // scripted queues (sign-extended).
            0x003d_7a50 => c.g[V0][0] = sx(oc360_next(c, OC360_SELECT_QUEUE)),
            0x003d_a418 => c.g[V0][0] = sx(oc360_next(c, OC360_BUILD_QUEUE)),
            // Path_GetClosestPoint(path, &pos, &out, &dist = a3, 1).
            0x0026_a428 => {
                let d = c.bus.read32(OC360_DIST);
                c.bus.write32(a3, d);
            }
            // AI_CountRidersOnPath: the scripted count.
            0x0010_fc30 => c.g[V0][0] = c.bus.read32(OC360_COUNT) as u64,
            // AnimController_IsLayerAnimFinished / GetLayerAnimEntry.
            0x0031_2ae8 => c.g[V0][0] = c.bus.read32(OC360_LAYER_FINISHED) as u64,
            0x0031_1ae8 => c.g[V0][0] = c.bus.read32(OC360_LAYER_ENTRY) as u64,
            // SSXAudioSystem_IsSpeechVolumeOn: off (the grunt returns at once).
            0x0029_f0a0 => c.g[V0][0] = 0,
            // UIStateMgr_GetActiveState: the scripted state (or 0).
            0x0039_fa88 => c.g[V0][0] = c.bus.read32(OC360_ACTIVE_STATE) as u64,
            // Nash_FindClosestContact(.., hit = a2, ..): the hit and the
            // distance.
            0x0033_6850 => {
                for i in 0..0x18 {
                    let w = c.bus.read32(OC360_HIT + 4 * i);
                    c.bus.write32(a2 + 4 * i, w);
                }
                c.f[0] = c.bus.read32(OC360_DIST);
            }
            _ => {}
        }
    }

    macro_rules! oc360_gated {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC360_CTRL) != OC360_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                oc360_fake(c, $addr);
            }
        };
    }

    oc360_gated!(
        oc360_fake_wsm_pending,
        OC360_PREV_WSM_PENDING,
        0x0030_9118,
        ee::s10::f_00309118__generated
    );

    oc360_gated!(
        oc360_fake_wsm_pop,
        OC360_PREV_WSM_POP,
        0x0030_b208,
        ee::s10::f_0030b208__generated
    );

    oc360_gated!(
        oc360_fake_wsm_push,
        OC360_PREV_WSM_PUSH,
        0x0030_b260,
        ee::s10::f_0030b260__generated
    );

    oc360_gated!(
        oc360_fake_wsm_start,
        OC360_PREV_WSM_START,
        0x0030_9750,
        ee::s10::f_00309750__generated
    );

    oc360_gated!(
        oc360_fake_wsp_forward,
        OC360_PREV_WSP_FORWARD,
        0x0030_7a10,
        ee::s10::f_00307a10__generated
    );

    oc360_gated!(
        oc360_fake_wsp_update,
        OC360_PREV_WSP_UPDATE,
        0x0030_77f8,
        ee::s10::f_003077f8__generated
    );

    fn oc360_install_wsm_fakes() {
        use std::ptr::addr_of_mut;
        {
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_00309118),
                oc360_fake_wsm_pending,
                &OC360_PREV_WSM_PENDING,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_0030B208),
                oc360_fake_wsm_pop,
                &OC360_PREV_WSM_POP,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_0030B260),
                oc360_fake_wsm_push,
                &OC360_PREV_WSM_PUSH,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_00309750),
                oc360_fake_wsm_start,
                &OC360_PREV_WSM_START,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_00307A10),
                oc360_fake_wsp_forward,
                &OC360_PREV_WSP_FORWARD,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_003077F8),
                oc360_fake_wsp_update,
                &OC360_PREV_WSP_UPDATE,
            );
        }
    }

    // ------------------------------------------- WScriptMan_Update

    /// The manager (8-aligned or not: the two copy loops), 0-3 starting
    /// processes, 0-3 running ones, up to 5 commands (type 0 with a world
    /// lookup that finds nothing, an empty word or a word; other types with
    /// the real `WScriptProcess_DispatchEvent` on fake methods; dispatch /
    /// forward flags). The other callees are logged fakes.
    fn wsu_setup(c: &mut Cpu, seed: u32, calls: u32) -> Rng {
        elf_image(c);
        let mut r = machine(c, seed, 0x0030_9270, calls);
        c.pc = 0x0030_9270;
        let base = DATA + 0x7_1000;
        let man = base + 4 * (seed % 2);
        let procs = base + 0x1000;
        let p10 = base + 0x2000;
        let wref = base + 0x3000;
        let table = base + 0x3100;
        let entries = base + 0x3200;
        c.bus.write32(OC360_CTRL, OC360_MAGIC);
        c.bus.write32(OC360_POP_NEXT, 0);
        let starting = r.next_u32() % 4;
        for i in 0..8 {
            let v = if i < starting { r.next_u32() | 0x10 } else { 0 };
            c.bus.write32(OC360_POP_QUEUE + 4 * i, v);
        }
        c.g[A0][0] = sx(man);
        fill(c, &mut r, man, 0xc0);
        // Commands.
        let count = match r.next_u32() % 6 {
            0 => 0,
            1 => 0u32.wrapping_sub(1 + r.next_u32() % 3),
            n => n + (r.next_u32() % 2),
        };
        c.bus.write32(man + WSU_COMMANDS, count);
        for i in 0..32 {
            let cmd = man + WSU_COMMANDS + 4 + 12 * i;
            let ty = match r.next_u32() % 3 {
                0 => 0,
                _ => r.next_u32() % 12,
            };
            let flags = (r.next_u32() % 4) << 16;
            c.bus.write32(cmd, flags | ty);
            c.bus
                .write32(cmd + 4, (r.next_u32() % 4) << 8 | r.next_u32() % 4);
            c.bus.write32(cmd + 8, r.next_u32());
        }
        // World lookup tables.
        c.bus.write32(man + WSU_WORLD_REF, wref);
        c.bus.write32(wref, wref + 0x10);
        c.bus.write32(wref + 0x10 + 8, table);
        for i in 0..4 {
            let e = entries + 0x40 * i;
            c.bus
                .write32(table + 4 * i, if r.next_u32() % 3 == 0 { 0 } else { e });
            c.bus.write32(e + 0x1c, e + 0x20);
            for j in 0..4 {
                let w = if r.next_u32() % 3 == 0 {
                    r.next_u32() & 0xff
                } else {
                    r.next_u32()
                };
                c.bus.write32(e + 0x20 + 4 * j, w);
            }
        }
        // Running processes.
        let running = r.next_u32() % 4;
        c.bus.write32(
            man + WSU_FIRST_RUNNING,
            if running == 0 { 0 } else { procs },
        );
        for i in 0..running {
            let p = procs + 0x100 * i;
            let next = if i + 1 < running { p + 0x100 } else { 0 };
            c.bus.write32(p + WSU_PROC_NEXT, next);
            c.bus.write32(p + 0x10, p10);
            let vt = fakes::vtable(
                c,
                &mut r,
                &[0x38, 0x40, 0x48, 0x50, 0x58, 0x60, 0x68, 0x70, 0x78, 0x80],
            );
            c.bus.write32(p + 0x5c, vt);
        }
        r
    }

    #[test]
    fn wscript_man_update_matches() {
        use crate::difftest::events::{check_events, schedule};
        oc360_install_wsm_fakes();
        covered(
            ee::s10::f_00309270__generated,
            wscript_man_update,
            300,
            &|c, seed| {
                wsu_setup(c, seed, 12);
            },
        );
        check_events(
            ee::s10::f_00309270__generated,
            wscript_man_update,
            300,
            &|c, seed| {
                let mut r = wsu_setup(c, seed, 0);
                schedule(c, &mut r, seed, 0x0030_9270)
            },
        );
    }
    // ------------------------------------------- AnimController_PlayAnimImpl

    macro_rules! oc360_fake_set {
        ($install:ident; $($name:ident $prev:ident $addr:literal $slot:ident $gen:ident;)*) => {
            $(oc360_gated!($name, $prev, $addr, ee::s10::$gen);)*
            fn $install() {
                use std::ptr::addr_of_mut;
                {
                    $(fakes::hook(addr_of_mut!(ee::s10::$slot), $name, &$prev);)*
                }
            }
        };
    }

    oc360_fake_set! {
        oc360_install_anim_fakes;
        oc360_fake_seq_clear OC360_PREV_SEQ_CLEAR 0x0031_4668 OVERRIDE_00314668 f_00314668__generated;
        oc360_fake_push_looping OC360_PREV_PUSH_LOOPING 0x0031_4588 OVERRIDE_00314588 f_00314588__generated;
        oc360_fake_push OC360_PREV_PUSH 0x0031_4518 OVERRIDE_00314518 f_00314518__generated;
        oc360_fake_add_event_trigger OC360_PREV_ADD_EVENT_TRIGGER 0x0031_3ad8 OVERRIDE_00313AD8 f_00313ad8__generated;
        oc360_fake_add_trigger OC360_PREV_ADD_TRIGGER 0x0031_3a70 OVERRIDE_00313A70 f_00313a70__generated;
        oc360_fake_weight_fade OC360_PREV_WEIGHT_FADE 0x0031_3a10 OVERRIDE_00313A10 f_00313a10__generated;
        oc360_fake_get_at OC360_PREV_GET_AT 0x0031_4760 OVERRIDE_00314760 f_00314760__generated;
        oc360_fake_remove_event_trigger OC360_PREV_REMOVE_EVENT_TRIGGER 0x0031_3ba8 OVERRIDE_00313BA8 f_00313ba8__generated;
        oc360_fake_fade_out OC360_PREV_FADE_OUT 0x0031_3a20 OVERRIDE_00313A20 f_00313a20__generated;
        oc360_fake_track_anim OC360_PREV_TRACK_ANIM 0x0031_3c50 OVERRIDE_00313C50 f_00313c50__generated;
        oc360_fake_track_time OC360_PREV_TRACK_TIME 0x0031_3cf0 OVERRIDE_00313CF0 f_00313cf0__generated;
        oc360_fake_track_weight OC360_PREV_TRACK_WEIGHT 0x0031_3d28 OVERRIDE_00313D28 f_00313d28__generated;
        oc360_fake_seq_remove OC360_PREV_SEQ_REMOVE 0x0031_45f8 OVERRIDE_003145F8 f_003145f8__generated;
    }

    /// The anim table `AnimTable_GetEntry` (a real leaf) indexes:
    /// `0x446990 + anim * 0x1c`.
    const PAI_TABLE: u32 = 0x0044_6990;

    /// Anims 0-5 with entry types 0/1/2/3 (+8 word zero or not, fade time
    /// zero or not), a zero or non-zero blend, `keep` 0 or 1, a list of
    /// 1-4 sequences whose older ones match the new one fully or differ in
    /// one field (anim, id, key, bank or one transform float), clips with
    /// 0-3 event keys. The list and sequence callees are logged fakes.
    fn pai_setup(c: &mut Cpu, seed: u32, calls: u32) -> Rng {
        elf_image(c);
        let mut r = machine(c, seed, 0x0031_1f00, calls);
        c.pc = 0x0031_1f00;
        let base = DATA + 0x7_8000;
        let ctrl = base;
        let list = base + 0x100;
        let new_seq = base + 0x200;
        let seqs = base + 0x400;
        let banks = base + 0x1000;
        let bank = base + 0x1100;
        let clips = base + 0x1200;
        let keys = base + 0x1400;
        c.bus.write32(OC360_CTRL, OC360_MAGIC);
        c.bus.write32(OC360_NEW_SEQ, new_seq);
        c.bus.write32(OC360_SEQS, seqs);
        let pick = |r: &mut Rng, v: &[f32]| v[r.next_u32() as usize % v.len()].to_bits();
        // Entries of anims 0-5.
        for a in 0..6 {
            let e = PAI_TABLE + 0x1c * a;
            fill(c, &mut r, e, 7);
            c.bus.write32(e + 4, r.next_u32() % 4);
            c.bus.write32(e + 8, r.next_u32() % 2);
            c.bus.write32(e + 0x14, pick(&mut r, &[0.0, 0.5]));
        }
        let anim = r.next_u32() % 6;
        // Controller.
        fill(c, &mut r, ctrl, 0x14);
        c.bus.write32(ctrl + PAI_CTRL_BANK, r.next_u32() % 2);
        c.bus
            .write32(ctrl + PAI_CTRL_TIME, pick(&mut r, &[0.0, 0.25]));
        for i in 0..8 {
            c.bus.write32(
                ctrl + PAI_CTRL_XFORM + 4 * i,
                pick(&mut r, &[0.0, 1.0, -2.0]),
            );
        }
        // Clip a3 = (clip << 8) | bank.
        let clip = r.next_u32() % 3;
        let clip_word = (clip << 8) | (r.next_u32() % 2);
        c.bus.write32(GAME_GP + PAI_BANKS, banks);
        c.bus.write32(banks, bank);
        c.bus.write32(banks + 4, bank);
        c.bus.write32(bank + 8, clips);
        c.bus.write32(bank + 0x10, keys);
        for k in 0..3 {
            c.bus
                .write16(clips + 0x14 * k + 0x10, (r.next_u32() % 4) as u16);
            c.bus
                .write16(clips + 0x14 * k + 0x12, (r.next_u32() % 4) as u16);
        }
        fill(c, &mut r, keys, 0x10);
        // The new sequence and the list.
        fill(c, &mut r, new_seq, 0x30);
        c.bus.write32(new_seq, r.next_u32() % 2);
        c.bus.write32(new_seq + 4, r.next_u32() % 2);
        let count = 1 + r.next_u32() % 4;
        c.bus.write32(list, count);
        for i in 0..4 {
            let o = seqs + 0x100 * i;
            fill(c, &mut r, o, 0x30);
            // A full match, then maybe one field off.
            let (w0, w1) = (c.bus.read32(new_seq), c.bus.read32(new_seq + 4));
            c.bus.write32(o, w0);
            c.bus.write32(o + 4, w1);
            let key = c.bus.read64(ctrl + PAI_CTRL_KEY);
            c.bus.write64(o + PAI_SEQ_KEY, key);
            let bank_word = c.bus.read32(ctrl + PAI_CTRL_BANK);
            c.bus.write32(o + PAI_SEQ_BANK, bank_word);
            for w in 0..8 {
                let v = c.bus.read32(ctrl + PAI_CTRL_XFORM + 4 * w);
                c.bus.write32(o + PAI_SEQ_XFORM + 4 * w, v);
            }
            match r.next_u32() % 16 {
                0 => c.bus.write32(o, 7),
                1 => c.bus.write32(o + 4, 7),
                2 => c.bus.write32(o + PAI_SEQ_KEY + 4 * (r.next_u32() % 2), 7),
                3 => c.bus.write32(o + PAI_SEQ_BANK, 7),
                4..=11 => {
                    let w = r.next_u32() % 8;
                    c.bus.write32(o + PAI_SEQ_XFORM + 4 * w, 0x4040_0000);
                }
                _ => {}
            }
            for k in 0..3 {
                c.bus.write32(o + 0x18 + 0x1c * k, r.next_u32() % 2);
            }
            c.bus
                .write32(o + PAI_SEQ_WEIGHT, pick(&mut r, &[0.5, 1.0, 2.0]));
        }
        c.g[A0][0] = sx(ctrl);
        c.g[A1][0] = sx(list);
        c.g[A2][0] = anim as u64;
        c.g[A3][0] = clip_word as u64;
        c.g[T0][0] = (r.next_u32() % 3 == 0) as u64;
        c.g[T1][0] = r.next_u32() as u64;
        c.f[12] = pick(&mut r, &[0.0, 0.0, 0.5, 0.25]);
        r
    }

    #[test]
    fn anim_controller_play_anim_impl_matches() {
        use crate::difftest::events::{check_events, schedule};
        oc360_install_anim_fakes();
        covered(
            ee::s10::f_00311f00__generated,
            anim_controller_play_anim_impl,
            300,
            &|c, seed| {
                pai_setup(c, seed, 16);
            },
        );
        check_events(
            ee::s10::f_00311f00__generated,
            anim_controller_play_anim_impl,
            300,
            &|c, seed| {
                let mut r = pai_setup(c, seed, 0);
                schedule(c, &mut r, seed, 0x0031_1f00)
            },
        );
    }
    // ------------------------------------------- BezierPatchCache_TessellatePatch

    macro_rules! oc360_fake_set12 {
        ($install:ident; $($name:ident $prev:ident $addr:literal $slot:ident $gen:ident;)*) => {
            $(oc360_gated!($name, $prev, $addr, ee::s12::$gen);)*
            fn $install() {
                use std::ptr::addr_of_mut;
                {
                    $(fakes::hook(addr_of_mut!(ee::s12::$slot), $name, &$prev);)*
                }
            }
        };
    }

    oc360_fake_set12! {
        oc360_install_tess_fakes;
        oc360_fake_tess_pos OC360_PREV_TESS_POS 0x0037_4180 OVERRIDE_00374180 f_00374180__generated;
        oc360_fake_tess_uv OC360_PREV_TESS_UV 0x0037_4298 OVERRIDE_00374298 f_00374298__generated;
        oc360_fake_tess_uv2 OC360_PREV_TESS_UV2 0x0037_4440 OVERRIDE_00374440 f_00374440__generated;
        oc360_fake_tess_alt OC360_PREV_TESS_ALT 0x0037_4518 OVERRIDE_00374518 f_00374518__generated;
    }

    /// Levels -1..3 (outside 0-2 every slab is 0), free stacks empty or
    /// not, normals on or off; the tessellators are logged fakes.
    #[test]
    fn bezier_patch_cache_tessellate_patch_matches() {
        oc360_install_tess_fakes();
        covered(
            ee::s12::f_003747a0__generated,
            bezier_patch_cache_tessellate_patch,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0037_47a0, 4);
                c.pc = 0x0037_47a0;
                let cache = DATA + 0x8_0000;
                let patch = DATA + 0x8_1000;
                let stacks = DATA + 0x8_2000;
                let entries = DATA + 0x8_3000;
                c.bus.write32(OC360_CTRL, OC360_MAGIC);
                fill(c, &mut r, cache, 0x80);
                fill(c, &mut r, patch, 0x70);
                let level = (seed % 5) as i32 - 1;
                for l in 0..3 {
                    let free = stacks + 0x100 * l;
                    let nfree = stacks + 0x400 + 0x100 * l;
                    c.bus.write32(cache + TPP_FREE + 4 * l, free);
                    c.bus.write32(cache + TPP_NRM_FREE + 4 * l, nfree);
                    let n = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 4
                    };
                    c.bus.write32(cache + TPP_FREE_COUNT + 4 * l, n);
                    c.bus
                        .write32(cache + TPP_NRM_COUNT + 4 * l, 1 + r.next_u32() % 4);
                    for k in 0..8 {
                        c.bus.write32(free + 4 * k, r.next_u32() % 6);
                        c.bus.write32(nfree + 4 * k, r.next_u32() % 6);
                    }
                    c.bus
                        .write32(cache + TPP_ENTRIES + 4 * l, entries + 0x100 * l);
                }
                fill(c, &mut r, entries, 0xc0);
                let level_word = level as u32;
                // A missing level reads entries[level] from the cache's
                // random words: point them somewhere harmless.
                // Levels -1 and 3 read their words next to the real ones.
                if !(0..3).contains(&level) {
                    let word = |off: u32| cache.wrapping_add(off).wrapping_add(4 * level_word);
                    c.bus.write32(word(TPP_ENTRIES), entries + 0x400);
                    c.bus.write32(word(TPP_NRM_COUNT), 2);
                    if level < 0 {
                        c.bus.write32(word(TPP_FREE_COUNT), 2);
                        c.bus.write32(word(TPP_FREE), stacks + 0x800);
                        c.bus.write32(word(TPP_NRM_FREE), stacks + 0x900);
                    }
                }
                c.g[A0][0] = sx(cache);
                c.g[A1][0] = sx(patch);
                c.g[A2][0] = sx(level_word);
                c.g[A3][0] = (r.next_u32() % 32) as u64;
                c.g[T0][0] = (r.next_u32() % 2) as u64;
            },
        );
    }
    // ------------------------------------------- RiderWipeoutState_PostUpdateRootPose

    oc360_gated!(
        oc360_fake_build_spheres,
        OC360_PREV_BUILD_SPHERES,
        0x0013_7550,
        ee::s01::f_00137550__generated
    );

    oc360_gated!(
        oc360_fake_nash_closest,
        OC360_PREV_NASH_CLOSEST,
        0x0033_6850,
        ee::s11::f_00336850__generated
    );

    oc360_gated!(
        oc360_fake_tree_translate,
        OC360_PREV_TREE_TRANSLATE,
        0x0032_9b40,
        ee::s10::f_00329b40__generated
    );

    /// The hull and its destructor are the wipeout tests' logged fakes
    /// (their gate set too); the sphere builder, the sweep and the tree
    /// translation are this module's.
    fn oc360_install_wipeout_fakes() {
        use std::ptr::addr_of_mut;
        crate::game::ai::motion::wipeoutmotion::tests::pua_install_fakes();
        {
            fakes::hook(
                addr_of_mut!(ee::s01::OVERRIDE_00137550),
                oc360_fake_build_spheres,
                &OC360_PREV_BUILD_SPHERES,
            );
            fakes::hook(
                addr_of_mut!(ee::s11::OVERRIDE_00336850),
                oc360_fake_nash_closest,
                &OC360_PREV_NASH_CLOSEST,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_00329B40),
                oc360_fake_tree_translate,
                &OC360_PREV_TREE_TRANSLATE,
            );
        }
    }

    /// The wipeout state fixture shared by the root-pose and ground tests:
    /// body, tree, velocity and spin, the world (collision at
    /// `+0x84 -> +0x20`, push scale at +0x10), the hit (normal, object
    /// none / ownerless / with an owner whose slot 0x158 is a fake) and
    /// the distance.
    fn purp_fixture(c: &mut Cpu, r: &mut Rng, this: u32) {
        use crate::game::ai::motion::wipeoutmotion::tests::{PUA_CTRL, PUA_MAGIC};
        let body = this + 0x200;
        let world = this + 0x400;
        let obj = this + 0x500;
        let owner = this + 0x600;
        let owner_vt = this + 0x700;
        let pick = |r: &mut Rng, v: &[f32]| v[r.next_u32() as usize % v.len()].to_bits();
        let comps = [-700.0f32, -12.0, -1.0, -0.5, 0.0, 0.5, 0.9, 1.0, 3.0, 700.0];
        let vec = |c: &mut Cpu, r: &mut Rng, at: u32| {
            for i in 0..3 {
                c.bus.write32(at + 4 * i, pick(r, &comps));
            }
            c.bus.write32(at + 12, pick(r, &[0.0, 1.0]));
        };
        c.bus.write32(OC360_CTRL, OC360_MAGIC);
        c.bus.write32(PUA_CTRL, PUA_MAGIC);
        fill(c, r, this, 0x20);
        c.bus.write32(this + PURP_BODY, body);
        c.bus.write32(this + PURP_TREE, r.next_u32());
        vec(c, r, this + PURP_VEL);
        // Spin: small (re-randomised) or large.
        let spin = [0.0f32, 1.0, 2.0, 20.0, -30.0];
        for i in 0..3 {
            c.bus.write32(this + PURP_ANG_VEL + 4 * i, pick(r, &spin));
        }
        c.bus.write32(this + PURP_ANG_VEL + 12, 0);
        vec(c, r, this + PURP_SEG);
        vec(c, r, this + PURP_SEG + 0x10);
        vec(c, r, body + PURP_BODY_POS);
        c.bus
            .write32(body + PURP_BODY_ACTIVE, (r.next_u32() % 6 != 0) as u32);
        c.bus.write32(GAME_GP - 0x848, world);
        c.bus.write32(world + 0x84, world + 0x80);
        c.bus.write32(world + 0xa0, r.next_u32());
        c.bus.write32(world + 0x10, r.next_u32() % 5);
        c.bus
            .write32(OC360_DIST, pick(r, &[-1.0, 0.0, 0.5, 2.0, 2.0]));
        fill(c, r, OC360_HIT, 0x18);
        vec(c, r, OC360_HIT + PURP_HIT_NORMAL);
        let object = match r.next_u32() % 3 {
            0 => 0,
            1 => {
                c.bus.write32(obj + 0xc, 0);
                obj
            }
            _ => {
                c.bus.write32(obj + 0xc, owner);
                obj
            }
        };
        c.bus.write32(OC360_HIT + PURP_HIT_OBJECT, object);
        c.bus.write32(owner + 0xc, owner_vt);
        c.bus
            .write16(owner_vt + 0x158, (r.next_u32() as i16 as i32 & !3) as u16);
        c.bus.write32(owner_vt + 0x15c, FAKE);
    }

    /// Inactive body, no contact, a contact that brings the body to rest
    /// (hit object none / ownerless / owned), and bounces (hard or off a
    /// wall) with the spin kept or re-randomised (the real
    /// `Rand_FloatRange` and `Math_SinCos`).
    #[test]
    fn rider_wipeout_state_post_update_root_pose_matches() {
        oc360_install_wipeout_fakes();
        covered(
            ee::s01::f_00137138__generated,
            rider_wipeout_state_post_update_root_pose,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x0013_7138, 6);
                c.pc = 0x0013_7138;
                let this = DATA + 0x9_0000;
                purp_fixture(c, &mut r, this);
                c.g[A0][0] = sx(this);
            },
        );
    }
    // ------------------------------------------- GameUI_Update

    oc360_gated!(
        oc360_fake_ui_active_state,
        OC360_PREV_UI_ACTIVE_STATE,
        0x0039_fa88,
        ee::s13::f_0039fa88__generated
    );

    oc360_gated!(
        oc360_fake_replay_finish,
        OC360_PREV_REPLAY_FINISH,
        0x0027_06f0,
        ee::s07::f_002706f0__generated
    );

    oc360_gated!(
        oc360_fake_be_get_interface,
        OC360_PREV_BE_GET_INTERFACE,
        0x0014_dd58,
        ee::s01::f_0014dd58__generated
    );

    oc360_gated!(
        oc360_fake_be_get_instance,
        OC360_PREV_BE_GET_INSTANCE,
        0x0014_dc80,
        ee::s01::f_0014dc80__generated
    );

    oc360_gated!(
        oc360_fake_ui_close_all,
        OC360_PREV_UI_CLOSE_ALL,
        0x0039_f840,
        ee::s13::f_0039f840__generated
    );

    oc360_gated!(
        oc360_fake_pda_close,
        OC360_PREV_PDA_CLOSE,
        0x0020_d190,
        ee::s05::f_0020d190__generated
    );

    oc360_gated!(
        oc360_fake_request_state,
        OC360_PREV_REQUEST_STATE,
        0x0023_1250,
        ee::s06::f_00231250__generated
    );

    fn oc360_install_ui_fakes() {
        use std::ptr::addr_of_mut;
        {
            fakes::hook(
                addr_of_mut!(ee::s13::OVERRIDE_0039FA88),
                oc360_fake_ui_active_state,
                &OC360_PREV_UI_ACTIVE_STATE,
            );
            fakes::hook(
                addr_of_mut!(ee::s07::OVERRIDE_002706F0),
                oc360_fake_replay_finish,
                &OC360_PREV_REPLAY_FINISH,
            );
            fakes::hook(
                addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
                oc360_fake_be_get_interface,
                &OC360_PREV_BE_GET_INTERFACE,
            );
            fakes::hook(
                addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
                oc360_fake_be_get_instance,
                &OC360_PREV_BE_GET_INSTANCE,
            );
            fakes::hook(
                addr_of_mut!(ee::s13::OVERRIDE_0039F840),
                oc360_fake_ui_close_all,
                &OC360_PREV_UI_CLOSE_ALL,
            );
            fakes::hook(
                addr_of_mut!(ee::s05::OVERRIDE_0020D190),
                oc360_fake_pda_close,
                &OC360_PREV_PDA_CLOSE,
            );
            fakes::hook(
                addr_of_mut!(ee::s06::OVERRIDE_00231250),
                oc360_fake_request_state,
                &OC360_PREV_REQUEST_STATE,
            );
        }
    }

    /// Requests 0-6 (5 and 6 are outside the table); the UI stack at 0
    /// (no state lookup) or not; an active state or none, its id 8 or
    /// not; game mode 2 or not; the close flag at 0x534b30 set or not.
    /// Request 1 runs the real `GameUI_HandleMenuResult` on its short path
    /// (the state's id outside 8..0x23).
    #[test]
    fn game_ui_update_matches() {
        oc360_install_ui_fakes();
        covered(
            ee::s05::f_0020cfd0__generated,
            game_ui_update,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x0020_cfd0, 4);
                c.pc = 0x0020_cfd0;
                let world = DATA + 0xa_0000;
                let game = world + 0x100;
                let mgr = world + 0x400;
                let flags = world + 0x500;
                let state = world + 0x600;
                let ptrs = world + 0x700;
                c.bus.write32(OC360_CTRL, OC360_MAGIC);
                fill(c, &mut r, world, 0x200);
                let request = seed % 7;
                c.bus.write32(GAME_GP - GUU_REQUEST, request);
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + GUU_GAME, game);
                c.bus.write32(world + 0xc0, ptrs);
                let mgr_word = if r.next_u32() % 5 == 0 {
                    0u32.wrapping_sub(GUU_UI_STACK)
                } else {
                    mgr
                };
                c.bus.write32(game + GUU_UI_MGR, mgr_word);
                c.bus.write32(game + 0x200, flags);
                c.bus.write32(game + 0x214, 1 + r.next_u32() % 2);
                let active = if request != 1 && r.next_u32() % 4 == 0 {
                    0
                } else {
                    state
                };
                c.bus.write32(OC360_ACTIVE_STATE, active);
                let id = if request == 1 {
                    [3, 0x40][r.next_u32() as usize % 2]
                } else {
                    [8, 8, 3][r.next_u32() as usize % 3]
                };
                c.bus.write32(state + 0x98, id);
                c.bus.write32(0x0053_4b30, r.next_u32() % 2);
            },
        );
    }
    // ------------------------------------------- RiderWipeoutState_PostUpdateGround

    /// No contact, a contact opposite the ground, one leaning into the
    /// ground (flattened) or not, a hit object none / ownerless / owned,
    /// moving into the contact (bounce, clamped low / high / between) or
    /// away; the pending flag; fast or slow (stopped). The callees are the
    /// wipeout tests' logged fakes (one may unwind).
    #[test]
    fn rider_wipeout_state_post_update_ground_matches() {
        use crate::game::ai::motion::wipeoutmotion::tests::{
            pua_install_fakes, PUA_CTRL, PUA_HIT_A_TEMPLATE, PUA_MAGIC,
        };
        pua_install_fakes();
        covered(
            ee::s01::f_00138640__generated,
            rider_wipeout_state_post_update_ground,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x0013_8640, 8);
                c.pc = 0x0013_8640;
                let this = DATA + 0xb_0000;
                let rider = this + 0x100;
                let obj = this + 0x1000;
                let owner = this + 0x1100;
                let owner_vt = this + 0x1200;
                let pick = |r: &mut Rng, v: &[f32]| v[r.next_u32() as usize % v.len()].to_bits();
                let comps = [
                    -700.0f32, -60.0, -1.0, -0.5, 0.0, 0.5, 1.0, 3.0, 60.0, 700.0,
                ];
                let vec = |c: &mut Cpu, r: &mut Rng, at: u32| {
                    for i in 0..3 {
                        c.bus.write32(at + 4 * i, pick(r, &comps));
                    }
                    c.bus.write32(at + 12, pick(r, &[0.0, 1.0]));
                };
                let unit = |c: &mut Cpu, r: &mut Rng, at: u32| {
                    let dirs: [[f32; 3]; 6] = [
                        [0.0, 0.0, 1.0],
                        [0.0, 0.0, -1.0],
                        [0.6, 0.0, 0.8],
                        [0.6, 0.0, -0.8],
                        [1.0, 0.0, 0.0],
                        [0.0, 0.8, 0.6],
                    ];
                    let d = dirs[r.next_u32() as usize % 6];
                    for i in 0..3 {
                        c.bus.write32(at + 4 * i as u32, d[i].to_bits());
                    }
                    c.bus.write32(at + 12, 0);
                };
                c.bus.write32(PUA_CTRL, PUA_MAGIC);
                c.g[A0][0] = sx(this);
                c.bus.write32(this, r.next_u32());
                c.bus.write32(this + 4, r.next_u32() % 2);
                c.bus.write32(this + PUG_RIDER, rider);
                for off in [PUG_R_OVERLAP, PUG_R_WALL_HINT, PUG_R_SPHERE_TREE] {
                    c.bus.write32(rider + off, r.next_u32());
                }
                vec(c, &mut r, rider + PUG_R_VEL);
                unit(c, &mut r, rider + PUG_R_GROUND_NORMAL);
                c.bus
                    .write32(PUA_CTRL + 4, pick(&mut r, &[-1.0, 0.0, 0.25, 2.0, 2.0]));
                fill(c, &mut r, PUA_HIT_A_TEMPLATE, 0x18);
                unit(c, &mut r, PUA_HIT_A_TEMPLATE + PUG_HIT_NORMAL);
                vec(c, &mut r, PUA_HIT_A_TEMPLATE + PUG_HIT_VEL);
                let object = match r.next_u32() % 3 {
                    0 => 0,
                    1 => {
                        c.bus.write32(obj + 0xc, 0);
                        obj
                    }
                    _ => {
                        c.bus.write32(obj + 0xc, owner);
                        obj
                    }
                };
                c.bus.write32(PUA_HIT_A_TEMPLATE + PUG_HIT_OBJECT, object);
                c.bus.write32(owner + 0xc, owner_vt);
                c.bus
                    .write16(owner_vt + 0x150, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(owner_vt + 0x154, FAKE);
            },
        );
    }
    // ------------------------------------------- SPCH_DequeueChannel

    macro_rules! oc360_fake_set14 {
        ($install:ident; $($name:ident $prev:ident $addr:literal $slot:ident $gen:ident;)*) => {
            $(oc360_gated!($name, $prev, $addr, ee::s14::$gen);)*
            fn $install() {
                use std::ptr::addr_of_mut;
                {
                    $(fakes::hook(addr_of_mut!(ee::s14::$slot), $name, &$prev);)*
                }
            }
        };
    }

    oc360_fake_set14! {
        oc360_install_spch_fakes;
        oc360_fake_spch_select OC360_PREV_SPCH_SELECT 0x003d_7a50 OVERRIDE_003D7A50 f_003d7a50__generated;
        oc360_fake_spch_free OC360_PREV_SPCH_FREE 0x003d_7c38 OVERRIDE_003D7C38 f_003d7c38__generated;
        oc360_fake_spch_purge OC360_PREV_SPCH_PURGE 0x003d_7d58 OVERRIDE_003D7D58 f_003d7d58__generated;
        oc360_fake_spch_reset OC360_PREV_SPCH_RESET 0x003d_9a40 OVERRIDE_003D9A40 f_003d9a40__generated;
        oc360_fake_spch_build OC360_PREV_SPCH_BUILD 0x003d_a418 OVERRIDE_003DA418 f_003da418__generated;
    }

    /// A channel without requests; requests picked until one is accepted
    /// (callback or none, which accepts) or none is left; sentence chains
    /// ending at -1 or at a failed build.
    fn sdc_setup(c: &mut Cpu, seed: u32, calls: u32) -> Rng {
        let mut r = machine(c, seed, 0x003d_7ec8, calls);
        c.pc = 0x003d_7ec8;
        c.bus.write32(OC360_CTRL, OC360_MAGIC);
        let channel = r.next_u32() % 4;
        for ch in 0..4 {
            let pending = if r.next_u32() % 5 == 0 { 0 } else { 1 };
            c.bus.write32(SDC_QUEUE + 4 * ch, pending);
        }
        // Requests 0-3: keys and next links.
        for i in 0..4u32 {
            let req = SDC_QUEUE + 0x60 * i;
            c.bus.write32(SDC_REQUESTS + 0x60 * i, r.next_u32());
            let next = match r.next_u32() % 3 {
                0 => 0xffff,
                _ => (r.next_u32() % 4) as u16 as u32,
            };
            c.bus.write16(req + SDC_NEXT, next as u16);
        }
        let callback = if r.next_u32() % 4 == 0 { 0 } else { FAKE };
        c.bus.write32(SDC_CALLBACKS + 0xc, callback);
        // Callback answers (forced fake results: 1 accepts).
        for i in 0..8 {
            let v = if r.next_u32() % 2 == 0 { 1 } else { 2 };
            c.bus.write32(fakes::LOG_RESULTS + 4 * i, v);
        }
        c.bus.write32(OC360_SELECT_QUEUE, 0);
        c.bus.write32(OC360_BUILD_QUEUE, 0);
        for i in 0..8 {
            let pick = if i == 7 || r.next_u32() % 6 == 0 {
                u32::MAX
            } else {
                r.next_u32() % 4
            };
            c.bus.write32(OC360_SELECT_QUEUE + 4 + 4 * i, pick);
            let ok = if i == 7 {
                0
            } else {
                (r.next_u32() % 3 != 0) as u32
            };
            c.bus.write32(OC360_BUILD_QUEUE + 4 + 4 * i, ok);
        }
        c.g[A0][0] = channel as u64;
        r
    }

    #[test]
    fn spch_dequeue_channel_matches() {
        use crate::difftest::events::{check_events, schedule};
        oc360_install_spch_fakes();
        covered(
            ee::s14::f_003d7ec8__generated,
            spch_dequeue_channel,
            300,
            &|c, seed| {
                sdc_setup(c, seed, 10);
            },
        );
        check_events(
            ee::s14::f_003d7ec8__generated,
            spch_dequeue_channel,
            300,
            &|c, seed| {
                let mut r = sdc_setup(c, seed, 0);
                schedule(c, &mut r, seed, 0x003d_7ec8)
            },
        );
    }
    // ------------------------------------------- ComputerActorRider_ScorePathBranch

    oc360_gated!(
        oc360_fake_closest_point,
        OC360_PREV_CLOSEST_POINT,
        0x0026_a428,
        ee::s07::f_0026a428__generated
    );

    oc360_gated!(
        oc360_fake_count_riders,
        OC360_PREV_COUNT_RIDERS,
        0x0010_fc30,
        ee::s00::f_0010fc30__generated
    );

    /// Every flag combination (class weight 0 or not, solo, levels 0-7,
    /// speed classes 0-7), speeds below and above the limits, the path
    /// near / far / beyond 5000, chasing a rider on the path or not, 0-3
    /// other riders on it, the solo and random bonuses. The real class
    /// weight, look-ahead and `Rand_NextGame`; the path query and the rider
    /// count are logged fakes.
    #[test]
    fn computer_actor_rider_score_path_branch_matches() {
        use std::ptr::addr_of_mut;
        {
            fakes::hook(
                addr_of_mut!(ee::s07::OVERRIDE_0026A428),
                oc360_fake_closest_point,
                &OC360_PREV_CLOSEST_POINT,
            );
            fakes::hook(
                addr_of_mut!(ee::s00::OVERRIDE_0010FC30),
                oc360_fake_count_riders,
                &OC360_PREV_COUNT_RIDERS,
            );
        }
        covered(
            ee::s00::f_0010d410__generated,
            computer_actor_rider_score_path_branch,
            400,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x0010_d410, 4);
                c.pc = 0x0010_d410;
                let this = DATA + 0xc_0000;
                let rider = this + 0x1000;
                let path = this + 0x2000;
                let other_path = this + 0x2100;
                let world = this + 0x3000;
                let game = this + 0x3100;
                let ai = this + 0x3200;
                let riders = this + 0x4000;
                c.bus.write32(OC360_CTRL, OC360_MAGIC);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(path);
                c.bus.write32(path + SPB_PATH_FLAGS, r.next_u32() & 0x3ff);
                c.bus.write16(this + 0xe00, (r.next_u32() % 4) as u16);
                c.bus.write32(this + SPB_SOLO_OK, r.next_u32() % 2);
                c.bus.write32(this + SPB_RANDOM_BONUS, r.next_u32() % 2);
                c.bus.write32(this + SPB_RIDER, rider);
                let speed =
                    [0.0f32, 300.0, 700.0, 1200.0, 2000.0, 5000.0][r.next_u32() as usize % 6];
                put_vec(c, rider + SPB_R_VEL, [speed.to_bits(), 0, 0, 0]);
                put_vec(
                    c,
                    rider + SPB_R_POS,
                    [r.f32(100.0), r.f32(100.0), r.f32(100.0), 0],
                );
                c.bus.write32(rider + SPB_R_CHASING, r.next_u32() % 2);
                let target = r.next_u32() % 4;
                c.bus.write32(rider + SPB_R_TARGET, target);
                let mine = if r.next_u32() % 2 == 0 {
                    path
                } else {
                    other_path
                };
                c.bus.write32(rider + SPB_R_PATH, mine);
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, game);
                c.bus.write32(game + 0xc, ai);
                for i in 0..4 {
                    let other = riders + 0x1000 * i;
                    c.bus.write32(ai + 0x28 + 4 * i, other);
                    let p = if r.next_u32() % 2 == 0 {
                        path
                    } else {
                        other_path
                    };
                    c.bus.write32(other + SPB_R_PATH, p);
                }
                let dist = [0.0f32, 50.0, 1000.0, 6000.0][r.next_u32() as usize % 4];
                c.bus.write32(OC360_DIST, dist.to_bits());
                c.bus.write32(OC360_COUNT, r.next_u32() % 4);
            },
        );
    }
    // ------------------------------------------- RiderWipeoutSub_UpdatePhase2

    macro_rules! oc360_fake_set_any {
        ($install:ident; $($name:ident $prev:ident $addr:literal $shard:ident $slot:ident $gen:ident;)*) => {
            $(oc360_gated!($name, $prev, $addr, ee::$shard::$gen);)*
            fn $install() {
                use std::ptr::addr_of_mut;
                {
                    $(fakes::hook(addr_of_mut!(ee::$shard::$slot), $name, &$prev);)*
                }
            }
        };
    }

    oc360_fake_set_any! {
        oc360_install_up2_fakes;
        oc360_fake_add_shake OC360_PREV_ADD_SHAKE 0x0015_e360 s01 OVERRIDE_0015E360 f_0015e360__generated;
        oc360_fake_anim_speed OC360_PREV_ANIM_SPEED 0x0012_e528 s00 OVERRIDE_0012E528 f_0012e528__generated;
        oc360_fake_check_reset OC360_PREV_CHECK_RESET 0x0011_6120 s00 OVERRIDE_00116120 f_00116120__generated;
        oc360_fake_recovery_boost OC360_PREV_RECOVERY_BOOST 0x0010_f280 s00 OVERRIDE_0010F280 f_0010f280__generated;
        oc360_fake_end_anim_a OC360_PREV_END_ANIM_A 0x0012_df48 s00 OVERRIDE_0012DF48 f_0012df48__generated;
        oc360_fake_anim_set_a OC360_PREV_ANIM_SET_A 0x0012_dd98 s00 OVERRIDE_0012DD98 f_0012dd98__generated;
        oc360_fake_layer_finished OC360_PREV_LAYER_FINISHED 0x0031_2ae8 s10 OVERRIDE_00312AE8 f_00312ae8__generated;
        oc360_fake_layer_entry OC360_PREV_LAYER_ENTRY 0x0031_1ae8 s10 OVERRIDE_00311AE8 f_00311ae8__generated;
        oc360_fake_anim_312660 OC360_PREV_ANIM_312660 0x0031_2660 s10 OVERRIDE_00312660 f_00312660__generated;
        oc360_fake_anim_30ecd8 OC360_PREV_ANIM_30ECD8 0x0030_ecd8 s10 OVERRIDE_0030ECD8 f_0030ecd8__generated;
        oc360_fake_anim_311bf0 OC360_PREV_ANIM_311BF0 0x0031_1bf0 s10 OVERRIDE_00311BF0 f_00311bf0__generated;
        oc360_fake_anim_3128e8 OC360_PREV_ANIM_3128E8 0x0031_28e8 s10 OVERRIDE_003128E8 f_003128e8__generated;
        oc360_fake_anim_312aa0 OC360_PREV_ANIM_312AA0 0x0031_2aa0 s10 OVERRIDE_00312AA0 f_00312aa0__generated;
        oc360_fake_speech_on OC360_PREV_SPEECH_ON 0x0029_f0a0 s08 OVERRIDE_0029F0A0 f_0029f0a0__generated;
        oc360_fake_stop_loop OC360_PREV_STOP_LOOP 0x0029_6e20 s08 OVERRIDE_00296E20 f_00296e20__generated;
        oc360_fake_stop_loop2 OC360_PREV_STOP_LOOP2 0x0029_7438 s08 OVERRIDE_00297438 f_00297438__generated;
    }

    /// The camera shake (index < 0, flag off / on, speed clamps), the
    /// bounce request, an AI getting up, the layer anim (finished, 0x1d,
    /// 0x19, 0x1c, other), fast (progress or ground timer) or slow, human
    /// or AI, surface 0x12, anim set 1 or not. The real
    /// `RiderWipeoutSub_StartGetUp` and `_SwitchToAnimSetB`, the real
    /// grunt (speech off) and audio instance; the other callees are logged
    /// fakes.
    #[test]
    fn rider_wipeout_sub_update_phase2_matches() {
        oc360_install_up2_fakes();
        covered(
            ee::s00::f_0012d160__generated,
            rider_wipeout_sub_update_phase2,
            400,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x0012_d160, 8);
                c.pc = 0x0012_d160;
                let this = DATA + 0xd_0000;
                let rider = this + 0x100;
                let vt = this + 0x1000;
                let handler = this + 0x1100;
                let world = this + 0x1200;
                let game = this + 0x1300;
                let cams = this + 0x1400;
                let skel = this + 0x1500;
                let arrays = this + 0x1800;
                let pick = |r: &mut Rng, v: &[f32]| v[r.next_u32() as usize % v.len()].to_bits();
                c.bus.write32(OC360_CTRL, OC360_MAGIC);
                c.g[A0][0] = sx(this);
                fill(c, &mut r, this, 0x40);
                fill(c, &mut r, rider, 0x400);
                c.bus
                    .write32(this + UP2_BOUNCE, (r.next_u32() % 5 == 0) as u32);
                c.bus
                    .write32(this + UP2_PROGRESS, pick(&mut r, &[0.0, 0.5, 1.0, 2.0]));
                c.bus.write32(this + UP2_RIDER, rider);
                let speed = pick(&mut r, &[0.0, 10.0, 100.0, 500.0, 1000.0, 2000.0, 4000.0]);
                put_vec(c, rider + UP2_R_VEL, [speed, 0, 0, 0]);
                c.bus.write32(rider + UP2_R_IS_AI, r.next_u32() % 3);
                c.bus.write32(
                    rider + UP2_R_SURFACE,
                    if r.next_u32() % 3 == 0 { 0x12 } else { 3 },
                );
                c.bus
                    .write32(rider + UP2_R_GROUND_TIME, pick(&mut r, &[-1.0, 0.0, 1.0]));
                c.bus.write32(rider + UP2_R_VTABLE, vt);
                for slot in [0x88u32, 0x90] {
                    c.bus
                        .write16(vt + slot, (r.next_u32() as i16 as i32 & !3) as u16);
                    c.bus.write32(vt + slot + 4, FAKE);
                }
                c.bus.write32(rider + UP2_R_HANDLER, handler);
                c.bus.write32(handler + 0x30, r.next_u32() % 2);
                let camera = if r.next_u32() % 4 == 0 {
                    u32::MAX
                } else {
                    r.next_u32() % 4
                };
                c.bus.write32(rider + UP2_R_CAMERA, camera);
                c.bus.write32(rider + UP2_R_SHAKE, r.next_u32() % 2);
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, game);
                c.bus.write32(game + 0x84, cams);
                // RiderWipeoutSub_StartGetUp's skeleton.
                c.bus.write32(rider + 0x780, skel);
                c.bus.write32(skel + 0x24, arrays);
                c.bus.write32(skel + 0x28, arrays + 0x100);
                c.bus.write32(rider + 0x89c, r.next_u32() % 4);
                fill(c, &mut r, arrays, 0x80);
                for i in 0..4 {
                    c.bus
                        .write32(skel + 0x140 + 4 * i, pick(&mut r, &[0.5, 1.0, 2.0]));
                }
                let finished = (r.next_u32() % 3 == 0) as u32;
                c.bus.write32(OC360_LAYER_FINISHED, finished);
                let entry = [0x19u32, 0x1c, 0x1d, 0x7][r.next_u32() as usize % 4];
                c.bus.write32(OC360_LAYER_ENTRY, entry);
            },
        );
    }
}
