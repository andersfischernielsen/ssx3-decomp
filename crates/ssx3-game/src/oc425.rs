//! Staging module oc425 (wave wave219): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc425`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0013_c650,
        rider_ground_state_ctor,
        "RiderGroundState_ctor",
    ),
    (
        0x0013_6980,
        rider_wipeout_state_ctor,
        "RiderWipeoutState_ctor",
    ),
    (0x0014_2388, rider_wipeout_sub_ctor, "RiderWipeoutSub_ctor"),
    (
        0x0014_1480,
        rider_handplant_state_ctor,
        "RiderHandplantState_ctor",
    ),
    (
        0x0023_7cf8,
        game_mode_man_get_instance,
        "GameModeMan_GetInstance",
    ),
    (
        0x0011_bd60,
        rider_add_bolt_names_to_load_list,
        "Rider_AddBoltNamesToLoadList",
    ),
    (0x0014_dc10, bolts_get_char_table3, "Bolts_GetCharTable3"),
    (0x0014_dc50, bolts_get_char_table4, "Bolts_GetCharTable4"),
    (
        0x0028_2be0,
        script_engine_get_seq_process_id,
        "ScriptEngine_GetSeqProcessId",
    ),
    (
        0x0022_d478,
        section_man_request_activate,
        "SectionMan_RequestActivate",
    ),
    (
        0x0027_7598,
        script_list_pop_entries,
        "ScriptList_PopEntries",
    ),
    (
        0x0013_26c8,
        rider_railslide_sub_play_enter_anim,
        "RiderRailslideSub_PlayEnterAnim",
    ),
    (
        0x0012_3e30,
        rider_on_script_sound_event,
        "Rider_OnScriptSoundEvent",
    ),
    (
        0x0035_a458,
        multi_spline_modifier_clone_instances,
        "MultiSplineModifier_CloneInstances",
    ),
    (
        0x0017_30d0,
        camera_trigger_read_box_trigger_data,
        "CameraTrigger_ReadBoxTriggerData",
    ),
    (0x0027_6388, script_list_start, "ScriptList_Start"),
    (
        0x0028_1ac8,
        script_view_obj_bind_cam_slot,
        "ScriptViewObj_BindCamSlot",
    ),
    (0x0035_bd70, rail_modifier_dtor, "RailModifier_v01"),
    (0x0033_69d8, nash_probe_vertical, "Nash_ProbeVertical"),
    // ==== OVERRIDES ====
];

// ---------------------------------------------------------------------------
// Rider-derived state constructors (RiderGroundState_ctor, RiderWipeoutState_ctor,
// RiderWipeoutSub_ctor, RiderHandplantState_ctor): gcc 2.9x constructors of
// objects with a virtual `cRider` base.

/// `cRider::cRider(cRider* this)`.
const RIDER_CTOR_ADDR: u32 = 0x0011_b698;

/// What differs between the four constructors: where the `cRider` lives in
/// the object, the class' own vtables and the return/loop addresses.
#[derive(Clone, Copy)]
struct RiderCtorLayout {
    /// The embedded `cRider` is at `this + rider_at` (inChrg only).
    rider_at: u32,
    /// `this + rider_ptr` holds the pointer to it (the virtual-base link).
    rider_ptr: u32,
    /// The class' script-object vtable is at `0x460000 - script_vt`.
    script_vt: u32,
    /// The class' rider vtable is at `0x460000 - rider_vt`.
    rider_vt: u32,
    /// Return address of the `cRider::cRider` call.
    ret: u32,
    /// Loop heads of the two vtable copy loops (event checks).
    copy1: u32,
    copy2: u32,
}

/// The shared part of the four constructors: open the 0x130-byte frame,
/// construct the `cRider` base when `inChrg`, point its three vtables
/// (+0x6e8 script object, +0x6d0 script rider 0x459b90, +0x6c0 rider) at the
/// class' own, and (as a base subobject) copy the script-object vtable (0x48
/// bytes) to `sp` and the rider vtable (0xa8 bytes) to `sp+0x50`, adjusting
/// their `this` deltas by the subobject offset (gcc 2.9x thunks).
fn rider_ctor_vtables(c: &mut Cpu, l: &RiderCtorLayout) -> Result<Frame, Unwound> {
    c.cycles += 7;
    let frame = Frame::open(c, 0x130);
    frame.save(c, S0, 0x120);
    frame.save(c, S1, 0x110);
    c.g[S0][0] = c.g[A0][0];
    c.g[S1][0] = c.g[A1][0];
    frame.save_ra(c, 0x100);
    if c.g[S1][0] != 0 {
        c.cycles += 4;
        let me = r32(c, S0);
        c.g[V0][0] = sx(me.wrapping_add(l.rider_at));
        c.g[A0][0] = c.g[V0][0];
        c.sw(me.wrapping_add(l.rider_ptr), c.g[V0][0]);
        call_ptr(c, RIDER_CTOR_ADDR, l.ret)?;
    }
    c.cycles += 15;
    let me = r32(c, S0);
    c.g[V1][0] = c.lw(me.wrapping_add(l.rider_ptr));
    c.g[V0][0] = 0x46_0000;
    c.g[A2][0] = sx(0x46_0000u32.wrapping_sub(l.script_vt));
    c.g[A0][0] = 0x46_0000;
    c.sw(r32(c, V1).wrapping_add(0x6e8), c.g[A2][0]);
    c.g[A0][0] = sx(0x46_0000u32.wrapping_sub(0x6470));
    c.g[V1][0] = 0x46_0000;
    c.g[A3][0] = c.g[V0][0];
    c.g[A1][0] = c.lw(me.wrapping_add(l.rider_ptr));
    c.g[T2][0] = c.g[V1][0];
    c.g[V1][0] = sx(0x46_0000u32.wrapping_sub(l.rider_vt));
    c.sw(r32(c, A1).wrapping_add(0x6d0), c.g[A0][0]);
    c.g[V0][0] = c.lw(me.wrapping_add(l.rider_ptr));
    c.sw(r32(c, V0).wrapping_add(0x6c0), c.g[V1][0]);
    if c.g[S1][0] == 0 {
        // A base subobject: the script-object vtable (0x48 bytes) to sp.
        c.cycles += 4;
        c.g[A1][0] = c.g[A2][0];
        c.g[V1][0] = c.g[SP][0];
        c.g[V0][0] = sx(r32(c, A1).wrapping_add(0x40));
        c.g[T0][0] = sx(sp(c).wrapping_add(0x50));
        loop {
            c.cycles += 11;
            let src = r32(c, A1);
            c.g[A0][0] = c.ld(src);
            c.g[A2][0] = c.ld(src.wrapping_add(0x8));
            c.g[T1][0] = c.ld(src.wrapping_add(0x10));
            c.g[T3][0] = c.ld(src.wrapping_add(0x18));
            let dst = r32(c, V1);
            c.sd(dst, c.g[A0][0]);
            c.sd(dst.wrapping_add(0x8), c.g[A2][0]);
            c.sd(dst.wrapping_add(0x10), c.g[T1][0]);
            c.sd(dst.wrapping_add(0x18), c.g[T3][0]);
            c.g[A1][0] = sx(src.wrapping_add(0x20));
            let more = c.g[A1][0] != c.g[V0][0];
            c.g[V1][0] = sx(dst.wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, l.copy1);
            }
        }
        c.cycles += 14;
        let me = r32(c, S0);
        c.g[V0][0] = c.lw(me.wrapping_add(l.rider_ptr));
        c.g[A0][0] = sx(r32(c, A3).wrapping_sub(l.script_vt));
        c.g[A2][0] = c.ld(r32(c, A1));
        c.sd(r32(c, V1), c.g[A2][0]);
        c.sw(r32(c, V0).wrapping_add(0x6e8), c.g[SP][0]);
        c.g[A2][0] = sx(r32(c, T2).wrapping_sub(l.rider_vt));
        c.g[V1][0] = c.lhu(r32(c, A0).wrapping_add(0x8));
        c.g[T1][0] = sx(r32(c, A2).wrapping_add(0xa0));
        c.g[V0][0] = c.lw(me.wrapping_add(l.rider_ptr));
        c.g[A1][0] = c.g[T0][0];
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(l.rider_at));
        c.g[A3][0] = sx(r32(c, S0).wrapping_sub(r32(c, V0)));
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A3)));
        c.sh(sp(c).wrapping_add(0x8), c.g[V1][0]);
        // The rider vtable (0xa8 bytes) to sp+0x50.
        loop {
            c.cycles += 11;
            let src = r32(c, A2);
            c.g[V0][0] = c.ld(src);
            c.g[V1][0] = c.ld(src.wrapping_add(0x8));
            c.g[A0][0] = c.ld(src.wrapping_add(0x10));
            c.g[T3][0] = c.ld(src.wrapping_add(0x18));
            let dst = r32(c, A1);
            c.sd(dst, c.g[V0][0]);
            c.sd(dst.wrapping_add(0x8), c.g[V1][0]);
            c.sd(dst.wrapping_add(0x10), c.g[A0][0]);
            c.sd(dst.wrapping_add(0x18), c.g[T3][0]);
            c.g[A2][0] = sx(src.wrapping_add(0x20));
            let more = c.g[A2][0] != c.g[T1][0];
            c.g[A1][0] = sx(dst.wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, l.copy2);
            }
        }
        c.cycles += 10;
        c.g[V0][0] = sx(r32(c, T2).wrapping_sub(l.rider_vt));
        c.g[A0][0] = c.lw(me.wrapping_add(l.rider_ptr));
        c.g[V1][0] = c.lhu(r32(c, V0).wrapping_add(0x8));
        c.g[V0][0] = c.ld(r32(c, A2));
        c.g[T1][0] = c.ld(r32(c, A2).wrapping_add(0x8));
        c.sd(r32(c, A1), c.g[V0][0]);
        c.sd(r32(c, A1).wrapping_add(0x8), c.g[T1][0]);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A3)));
        c.sw(r32(c, A0).wrapping_add(0x6c0), c.g[T0][0]);
        c.sh(sp(c).wrapping_add(0x58), c.g[V1][0]);
    }
    Ok(frame)
}

const GROUND_STATE_LAYOUT: RiderCtorLayout = RiderCtorLayout {
    rider_at: 0x20,
    rider_ptr: 0x18,
    script_vt: 0x6568,
    rider_vt: 0x6520,
    ret: 0x0013_c67c,
    copy1: 0x0013_c6c8,
    copy2: 0x0013_c72c,
};

const WIPEOUT_STATE_LAYOUT: RiderCtorLayout = RiderCtorLayout {
    rider_at: 0x70,
    rider_ptr: 0x40,
    script_vt: 0x6850,
    rider_vt: 0x6808,
    ret: 0x0013_69ac,
    copy1: 0x0013_69f8,
    copy2: 0x0013_6a5c,
};

const WIPEOUT_SUB_LAYOUT: RiderCtorLayout = RiderCtorLayout {
    rider_at: 0x90,
    rider_ptr: 0x80,
    script_vt: 0x73f0,
    rider_vt: 0x73a8,
    ret: 0x0014_23b4,
    copy1: 0x0014_2400,
    copy2: 0x0014_2464,
};

const HANDPLANT_STATE_LAYOUT: RiderCtorLayout = RiderCtorLayout {
    rider_at: 0xb0,
    rider_ptr: 0xa0,
    script_vt: 0x6a40,
    rider_vt: 0x69f8,
    ret: 0x0014_14ac,
    copy1: 0x0014_14f8,
    copy2: 0x0014_155c,
};

/// `RiderGroundState* RiderGroundState_ctor(this, bool inChrg)` (0x13c650,
/// `cGroundMotion::cGroundMotion`): the rider base (`this+0x20`, linked at
/// `this+0x18`) and its vtables, then `AITickOnLoseFocus`/`AITickOnGainFocus`
/// (`+0x10`/`+0x14`) and the rider's `+0x31c` cleared; returns `this`.
pub fn rider_ground_state_ctor(c: &mut Cpu) {
    guest(c, |c| {
        let frame = rider_ctor_vtables(c, &GROUND_STATE_LAYOUT)?;
        c.cycles += 10;
        c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x18));
        c.g[V0][0] = c.g[S0][0];
        c.sw(r32(c, S0).wrapping_add(0x14), 0);
        c.sw(r32(c, S0).wrapping_add(0x10), 0);
        frame.restore(c, S0, 0x120);
        frame.restore(c, S1, 0x110);
        frame.restore_ra(c, 0x100);
        c.sw(r32(c, V1).wrapping_add(0x31c), 0);
        frame.close(c);
        Ok(())
    })
}

/// `RiderWipeoutState* RiderWipeoutState_ctor(this, bool inChrg)` (0x136980,
/// `cWipeoutMotion::cWipeoutMotion`): the rider base (`this+0x70`, linked at
/// `this+0x40`) and its vtables, `this+0x44` cleared; returns `this`.
pub fn rider_wipeout_state_ctor(c: &mut Cpu) {
    guest(c, |c| {
        let frame = rider_ctor_vtables(c, &WIPEOUT_STATE_LAYOUT)?;
        c.cycles += 7;
        c.g[V0][0] = c.g[S0][0];
        c.sw(r32(c, S0).wrapping_add(0x44), 0);
        frame.restore(c, S0, 0x120);
        frame.restore(c, S1, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `RiderWipeoutSub* RiderWipeoutSub_ctor(this, bool inChrg)` (0x142388,
/// `cWipeoutControl::cWipeoutControl`): the rider base (`this+0x90`, linked
/// at `this+0x80`) and its vtables; returns `this`.
pub fn rider_wipeout_sub_ctor(c: &mut Cpu) {
    guest(c, |c| {
        let frame = rider_ctor_vtables(c, &WIPEOUT_SUB_LAYOUT)?;
        c.cycles += 6;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x110);
        frame.restore(c, S0, 0x120);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `RiderHandplantState* RiderHandplantState_ctor(this, bool inChrg)`
/// (0x141480, `cHandplantMotion::cHandplantMotion`): the rider base
/// (`this+0xb0`, linked at `this+0xa0`) and its vtables; returns `this`.
pub fn rider_handplant_state_ctor(c: &mut Cpu) {
    guest(c, |c| {
        let frame = rider_ctor_vtables(c, &HANDPLANT_STATE_LAYOUT)?;
        c.cycles += 6;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x110);
        frame.restore(c, S0, 0x120);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}
/// `cGameModeMan* cGameModeMan::getGM()` (0x237cf8): the game-mode manager singleton, created on first use.
/// On the first call: allocate the 0xa0-byte `GameModeMan` (instance at `gp-0x484`),
/// clear the 10-entry handler table at 0x536668 and fill its empty entries with the mode
/// handlers (a `new` of 0x7c, 0xc or 0x8 bytes is tried up to three times; 0xd0, 4 and 0x2c
/// byte ones once), each with its vtable (0x47cfa0, 0x47cf38, 0x47ce00, 0x47cd30, 0x47cd98,
/// 0x47ccc8, 0x47cc60, 0x47ced0, 0x47ce68). Then publish the manager (`gp-0x480`) and set
/// `iCurrentGameEvent` (+8) to -1 and +0, +4, +0x80 to 0. Returns the instance.
pub fn game_mode_man_get_instance(c: &mut Cpu) {
    guest(c, |c| {
        'b_2380c4: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x484));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            let t = c.g[V0][0] != 0u64;
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            if t {
                break 'b_2380c4;
            } else {
                'b_237dc8: {
                    c.cycles += 16;
                    c.g[A1][0] = 0x480000;
                    c.g[A0][0] = 0xa0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3f58));
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0u64;
                    c.g[S0][0] = 0x530000;
                    call_ptr(c, 0x00317d70, 0x00237d34)?;
                    c.g[S3][0] = c.g[S0][0];
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x484), c.g[V0][0]);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6668));
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0x28;
                    call_ptr(c, 0x00416210, 0x00237d4c)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6668));
                    let t = c.g[V1][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6668));
                    if t {
                        break 'b_237dc8;
                    } else {
                        'b_237dc4: {
                            c.cycles += 12;
                            c.g[V0][0] = 0x480000;
                            c.g[A0][0] = 0x7c;
                            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3f48));
                            c.g[A2][0] = 0u64;
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x00237d74)?;
                            c.g[V1][0] = 0x480000;
                            c.sw((c.g[S3][0] as u32).wrapping_add(0x6668), c.g[V0][0]);
                            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3060));
                            let t = c.g[V0][0] != 0u64;
                            c.sw((c.g[V0][0] as u32), c.g[S0][0]);
                            if t {
                                break 'b_237dc4;
                            } else {
                                c.cycles += 8;
                                c.g[A0][0] = 0x7c;
                                c.g[A1][0] = c.g[S1][0];
                                c.g[A2][0] = 0u64;
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317d70, 0x00237d9c)?;
                                c.sw((c.g[V0][0] as u32), c.g[S0][0]);
                                let t = c.g[V0][0] != 0u64;
                                c.sw((c.g[S3][0] as u32).wrapping_add(0x6668), c.g[V0][0]);
                                if t {
                                    break 'b_237dc4;
                                } else {
                                    c.cycles += 7;
                                    c.g[A1][0] = c.g[S1][0];
                                    c.g[A0][0] = 0x7c;
                                    c.g[A2][0] = 0u64;
                                    c.g[A3][0] = 0u64;
                                    call_ptr(c, 0x00317d70, 0x00237dbc)?;
                                    c.sw((c.g[V0][0] as u32), c.g[S0][0]);
                                    c.sw((c.g[S3][0] as u32).wrapping_add(0x6668), c.g[V0][0]);
                                    break 'b_237dc4;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6668));
                        break 'b_237dc8;
                    }
                }
                'b_237e00: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        break 'b_237e00;
                    } else {
                        c.cycles += 11;
                        c.g[A1][0] = 0x480000;
                        c.g[A0][0] = 0xd0;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3f48));
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00317d70, 0x00237dec)?;
                        c.g[V1][0] = 0x480000;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x30c8));
                        c.sw((c.g[V0][0] as u32), c.g[V1][0]);
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        break 'b_237e00;
                    }
                }
                'b_237e34: {
                    c.cycles += 2;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        break 'b_237e34;
                    } else {
                        c.cycles += 11;
                        c.g[A1][0] = 0x480000;
                        c.g[A0][0] = 0x4;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3f48));
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00317d70, 0x00237e20)?;
                        c.g[V1][0] = 0x480000;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3200));
                        c.sw((c.g[V0][0] as u32), c.g[V1][0]);
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        break 'b_237e34;
                    }
                }
                'b_237e68: {
                    c.cycles += 2;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        break 'b_237e68;
                    } else {
                        c.cycles += 11;
                        c.g[A1][0] = 0x480000;
                        c.g[A0][0] = 0x4;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3f48));
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00317d70, 0x00237e54)?;
                        c.g[V1][0] = 0x480000;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x32d0));
                        c.sw((c.g[V0][0] as u32), c.g[V1][0]);
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        break 'b_237e68;
                    }
                }
                'b_237ee0: {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6668));
                    if t {
                        break 'b_237ee0;
                    } else {
                        'b_237edc: {
                            c.cycles += 12;
                            c.g[V0][0] = 0x480000;
                            c.g[A0][0] = 0xc;
                            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3f48));
                            c.g[A2][0] = 0u64;
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x00237e8c)?;
                            c.g[V1][0] = 0x480000;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                            c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3268));
                            let t = c.g[V0][0] != 0u64;
                            c.sw((c.g[V0][0] as u32), c.g[S2][0]);
                            if t {
                                break 'b_237edc;
                            } else {
                                c.cycles += 8;
                                c.g[A0][0] = 0xc;
                                c.g[A1][0] = c.g[S1][0];
                                c.g[A2][0] = 0u64;
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317d70, 0x00237eb4)?;
                                c.sw((c.g[V0][0] as u32), c.g[S2][0]);
                                let t = c.g[V0][0] != 0u64;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                                if t {
                                    break 'b_237edc;
                                } else {
                                    c.cycles += 7;
                                    c.g[A1][0] = c.g[S1][0];
                                    c.g[A0][0] = 0xc;
                                    c.g[A2][0] = 0u64;
                                    c.g[A3][0] = 0u64;
                                    call_ptr(c, 0x00317d70, 0x00237ed4)?;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                                    c.sw((c.g[V0][0] as u32), c.g[S2][0]);
                                    break 'b_237edc;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6668));
                        break 'b_237ee0;
                    }
                }
                'b_237f18: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                        break 'b_237f18;
                    } else {
                        c.cycles += 11;
                        c.g[A1][0] = 0x480000;
                        c.g[A0][0] = 0x4;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3f48));
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00317d70, 0x00237f04)?;
                        c.g[V1][0] = 0x480000;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3338));
                        c.sw((c.g[V0][0] as u32), c.g[V1][0]);
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                        break 'b_237f18;
                    }
                }
                'b_237f90: {
                    c.cycles += 2;
                    if c.g[V0][0] != 0u64 {
                        c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6668));
                        break 'b_237f90;
                    } else {
                        'b_237f8c: {
                            c.cycles += 12;
                            c.g[V0][0] = 0x480000;
                            c.g[A0][0] = 0xc;
                            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3f48));
                            c.g[A2][0] = 0u64;
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x00237f3c)?;
                            c.g[V1][0] = 0x480000;
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                            c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x33a0));
                            let t = c.g[V0][0] != 0u64;
                            c.sw((c.g[V0][0] as u32), c.g[S1][0]);
                            if t {
                                break 'b_237f8c;
                            } else {
                                c.cycles += 8;
                                c.g[A0][0] = 0xc;
                                c.g[A1][0] = c.g[S0][0];
                                c.g[A2][0] = 0u64;
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317d70, 0x00237f64)?;
                                c.sw((c.g[V0][0] as u32), c.g[S1][0]);
                                let t = c.g[V0][0] != 0u64;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                                if t {
                                    break 'b_237f8c;
                                } else {
                                    c.cycles += 7;
                                    c.g[A1][0] = c.g[S0][0];
                                    c.g[A0][0] = 0xc;
                                    c.g[A2][0] = 0u64;
                                    c.g[A3][0] = 0u64;
                                    call_ptr(c, 0x00317d70, 0x00237f84)?;
                                    c.sw((c.g[S2][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                                    c.sw((c.g[V0][0] as u32), c.g[S1][0]);
                                    break 'b_237f8c;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6668));
                        break 'b_237f90;
                    }
                }
                'b_237fc8: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
                        break 'b_237fc8;
                    } else {
                        c.cycles += 11;
                        c.g[A1][0] = 0x480000;
                        c.g[A0][0] = 0x2c;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3f48));
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00317d70, 0x00237fb4)?;
                        c.g[V1][0] = 0x480000;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3130));
                        c.sw((c.g[V0][0] as u32), c.g[V1][0]);
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
                        break 'b_237fc8;
                    }
                }
                'b_238040: {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6668));
                    if t {
                        break 'b_238040;
                    } else {
                        'b_23803c: {
                            c.cycles += 12;
                            c.g[V0][0] = 0x480000;
                            c.g[A0][0] = 0x8;
                            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3f48));
                            c.g[A2][0] = 0u64;
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x00237fec)?;
                            c.g[V1][0] = 0x480000;
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                            c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3198));
                            let t = c.g[V0][0] != 0u64;
                            c.sw((c.g[V0][0] as u32), c.g[S1][0]);
                            if t {
                                break 'b_23803c;
                            } else {
                                c.cycles += 8;
                                c.g[A0][0] = 0x8;
                                c.g[A1][0] = c.g[S0][0];
                                c.g[A2][0] = 0u64;
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317d70, 0x00238014)?;
                                c.sw((c.g[V0][0] as u32), c.g[S1][0]);
                                let t = c.g[V0][0] != 0u64;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                                if t {
                                    break 'b_23803c;
                                } else {
                                    c.cycles += 7;
                                    c.g[A1][0] = c.g[S0][0];
                                    c.g[A0][0] = 0x8;
                                    c.g[A2][0] = 0u64;
                                    c.g[A3][0] = 0u64;
                                    call_ptr(c, 0x00317d70, 0x00238034)?;
                                    c.sw((c.g[S2][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                                    c.sw((c.g[V0][0] as u32), c.g[S1][0]);
                                    break 'b_23803c;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6668));
                        break 'b_238040;
                    }
                }
                'b_238078: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                        break 'b_238078;
                    } else {
                        c.cycles += 11;
                        c.g[A1][0] = 0x480000;
                        c.g[A0][0] = 0x4;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3f48));
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00317d70, 0x00238064)?;
                        c.g[V1][0] = 0x480000;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x32d0));
                        c.sw((c.g[V0][0] as u32), c.g[V1][0]);
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                        break 'b_238078;
                    }
                }
                'b_2380ac: {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x484));
                    if t {
                        break 'b_2380ac;
                    } else {
                        c.cycles += 11;
                        c.g[A1][0] = 0x480000;
                        c.g[A0][0] = 0x4;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3f48));
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00317d70, 0x00238098)?;
                        c.g[V1][0] = 0x480000;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3338));
                        c.sw((c.g[V0][0] as u32), c.g[V1][0]);
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x484));
                        break 'b_2380ac;
                    }
                }
                c.cycles += 6;
                c.g[V1][0] = 0xffffffffffffffff;
                c.sw((c.g[GP][0] as u32).wrapping_sub(0x480), c.g[V0][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x80), 0u64);
                c.sw((c.g[V0][0] as u32), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                break 'b_2380c4;
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x484));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}
/// Offset of the anim-name table pointer in a `Rider` (`self->mLoadList`).
const RIDER_LOAD_LIST: u32 = 0x894;
/// Entries in the table (`count`, at +0x2940); each entry is 0x108 bytes: a
/// 0x100 byte name, then two words (`-1` and 0).
const ANIM_NAME_COUNT: u32 = 0x2940;
const ANIM_NAME_ENTRY: u32 = 0x108;

/// `void Rider_AddBoltNamesToLoadList(Rider* self, sBoltEntry** list, int n)`
/// (0x11bd60): for each of the `n` list entries, take the animation name at
/// `entry+0x28`; unless it is null, empty or already in `self->mLoadList`
/// (compared with `strcmp`), append it as a new 0x108-byte entry (`strcpy`,
/// `+0x100 = -1`, `+0x104 = 0`, count + 1). Then sort the table with
/// `qsort(table, count, 0x108, 0x11b678)`.
pub fn rider_add_bolt_names_to_load_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xa0);
        c.g[V1][0] = 0;
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        c.g[S6][0] = c.g[A2][0];
        frame.save(c, S0, 0x90);
        c.g[S7][0] = c.g[A0][0];
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);
        if (c.g[S6][0] as i64) > 0 {
            c.cycles += 1;
            c.g[FP][0] = u64::MAX;
            loop {
                'tail: {
                    'add: {
                        c.cycles += 6;
                        c.g[V0][0] = c.lw(r32(c, A1));
                        c.g[S4][0] = sx(r32(c, V1).wrapping_add(1));
                        c.g[S5][0] = sx(r32(c, A1).wrapping_add(4));
                        c.g[S2][0] = c.lw(r32(c, V0).wrapping_add(0x28));
                        let no_name = c.g[S2][0] == 0;
                        c.g[S3][0] = c.lw(r32(c, S7).wrapping_add(RIDER_LOAD_LIST));
                        if no_name {
                            break 'add;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lb(r32(c, S2));
                        let empty = c.g[V0][0] == 0;
                        c.g[V1][0] = c.g[S4][0];
                        if empty {
                            break 'tail;
                        }
                        c.cycles += 4;
                        c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(ANIM_NAME_COUNT));
                        c.g[S1][0] = 0;
                        c.g[V1][0] = c.g[V0][0];
                        if (c.g[V0][0] as i64) > 0 {
                            // Look for the name among the existing entries.
                            c.cycles += 1;
                            c.g[S0][0] = c.g[S3][0];
                            loop {
                                c.cycles += 5;
                                c.g[A0][0] = c.g[S2][0];
                                c.g[A1][0] = c.g[S0][0];
                                call_ptr(c, 0x0041_65a8, 0x0011_bde4)?;
                                let found = c.g[V0][0] == 0;
                                c.g[V1][0] = c.g[S4][0];
                                if found {
                                    break 'tail;
                                }
                                c.cycles += 6;
                                c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(ANIM_NAME_COUNT));
                                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                                c.g[V1][0] = c.g[V0][0];
                                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let more = c.g[V0][0] != 0;
                                c.g[S0][0] = sx(r32(c, S0).wrapping_add(ANIM_NAME_ENTRY));
                                if !more {
                                    break;
                                }
                                if event_due(c) {
                                    deliver_event_here(c, 0x0011_bdd8);
                                }
                            }
                        }
                        // Not found: append `name` as entry `count`.
                        c.cycles += 11;
                        c.g[V0][0] = ANIM_NAME_ENTRY as u64;
                        c.g[A1][0] = c.g[S2][0];
                        c.mult(0, 2, 3, 2);
                        c.g[S0][0] = sx(r32(c, V0).wrapping_add(r32(c, S3)));
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0041_66f4, 0x0011_be1c)?;
                        c.sw(r32(c, S0).wrapping_add(0x104), 0);
                        c.sw(r32(c, S0).wrapping_add(0x100), c.g[FP][0]);
                        c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(ANIM_NAME_COUNT));
                        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                        c.sw(r32(c, S3).wrapping_add(ANIM_NAME_COUNT), c.g[V0][0]);
                    }
                    c.cycles += 1;
                    c.g[V1][0] = c.g[S4][0];
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S6][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = c.g[S5][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0011_bda0);
                }
            }
        }
        // Sort the table by name (comparison function 0x11b678).
        c.cycles += 18;
        c.g[A0][0] = c.lw(r32(c, S7).wrapping_add(RIDER_LOAD_LIST));
        c.g[A3][0] = 0x12_0000;
        c.g[A3][0] = sx(r32(c, A3).wrapping_sub(0x4988));
        c.g[A2][0] = ANIM_NAME_ENTRY as u64;
        c.g[A1][0] = c.lw(r32(c, A0).wrapping_add(ANIM_NAME_COUNT));
        call_ptr(c, 0x0041_8ef8, 0x0011_be58)?;
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
/// The two `Bolts` character tables (`Bolts_GetCharTable3` / `4`): entry `i`
/// of the table of `self` (`count[i]` at `self + 4*i + count`, the first
/// entry `index[i]` at `self + 4*i + index`, the table at `self + table`):
/// the address of `table[index[i]]` (8-byte entries), or 0 when `count[i]`
/// is not positive.
fn bolts_get_char_table(c: &mut Cpu, count: u32, index: u32, table: u32) {
    c.cycles += 5;
    c.g[A1][0] = sx(r32(c, A1) << 2);
    c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(count));
    let positive = (c.g[V1][0] as i64) > 0;
    c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
    if positive {
        c.cycles += 5;
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(index));
        c.g[A0][0] = c.lw(r32(c, A0).wrapping_add(table));
        c.g[V0][0] = sx(r32(c, V0) << 3);
        c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0;
    }
    ret(c);
}

/// `sBoltEntry* Bolts_GetCharTable3(Bolts* self, int i)` (0x14dc10): the
/// table at `+0xc` (counts at `+0x2fc`, indexes at `+0x374`).
pub fn bolts_get_char_table3(c: &mut Cpu) {
    bolts_get_char_table(c, 0x2fc, 0x374, 0xc);
}

/// `sBoltEntry* Bolts_GetCharTable4(Bolts* self, int i)` (0x14dc50): the
/// table at `+0x10` (counts at `+0x3ec`, indexes at `+0x464`).
pub fn bolts_get_char_table4(c: &mut Cpu) {
    bolts_get_char_table(c, 0x3ec, 0x464, 0x10);
}

/// `int ScriptEngine_GetSeqProcessId(void* unused, ScriptSeq* seq)`
/// (0x282be0): `seq->process->id` (`*(*(seq + 0xc) + 0x1c)`).
pub fn script_engine_get_seq_process_id(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(0xc));
    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x1c));
    ret(c);
}
/// `cSectionMan_activateSection`'s state change (`section->state = state`).
fn section_request_set_state(c: &mut Cpu, state: u64) {
    c.cycles += 6;
    c.g[V0][0] = sx(r32(c, A2).wrapping_add(0x2168));
    c.g[V1][0] = sx(r32(c, S0) << 4);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    c.g[A0][0] = state;
    c.sw(r32(c, V0).wrapping_add(0x8), c.g[A0][0]);
}

/// `void SectionMan_RequestActivate(SectionMan* self, int i)` (0x22d478,
/// `cSectionMan::activateSection`): step the load state (`+8`) of section `i`
/// of the section map (0x442168, 16-byte entries; unless its world id at
/// `+4` is negative). A jump table (0x47b3d0, indexed by the
/// state, 0..8) picks: states 0, 3 and 7 become 4; 6 becomes 8; 1 and 5 become
/// 2 and the world view activates the section
/// (`cWorldCache_StreamerActivateSection(world + 0x10, section id)`); anything
/// else (2, 4, 8, out of range) stays. If it changed, queue it
/// (`SectionMan_QueueSection(self, i)`). The port accepts the resume entries
/// of the generated `__entries` twin through its fallback.
pub fn section_man_request_activate(c: &mut Cpu) {
    if c.pc != 0x0022_d478 {
        // The other entries (resume points after a callee unwound): the
        // structured form's `__entries` twin; the unwind form's body
        // dispatches on `c.pc` itself.
        #[cfg(not(feature = "ee-unwind"))]
        return ee::s05::f_0022d478__entries(c);
        #[cfg(feature = "ee-unwind")]
        return ee::s05::f_0022d478__generated(c);
    }
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x44_0000;
        frame.save(c, S0, 0x20);
        c.g[A2][0] = c.g[V0][0];
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[A3][0] = sx(r32(c, V0).wrapping_add(0x2168));
        c.g[A1][0] = sx(r32(c, S0) << 4);
        c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, A1)));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x4));
        let unused = (c.g[V1][0] as i64) < 0;
        c.g[S1][0] = c.g[A0][0];
        if !unused {
            c.cycles += 5;
            c.g[V1][0] = c.g[V0][0];
            c.g[V1][0] = c.lw(r32(c, V1).wrapping_add(0x8));
            c.g[V0][0] = (c.g[V1][0] < 0x9u64) as u64;
            let out_of_range = c.g[V0][0] == 0;
            c.g[A1][0] = 0;
            // `changed` (a1) stays 0 unless a state arm below sets it.
            if !out_of_range {
                c.cycles += 7;
                c.g[V0][0] = 0x48_0000;
                c.g[V1][0] = sx(r32(c, V1) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x4c30));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.g[A0][0] = c.lw(r32(c, V1));
                let case = r32(c, A0);
                let changed = match case {
                    0x0022_d4dc | 0x0022_d4f4 | 0x0022_d50c => {
                        section_request_set_state(c, 4);
                        true
                    }
                    0x0022_d524 => {
                        section_request_set_state(c, 8);
                        true
                    }
                    0x0022_d53c => {
                        c.cycles += 12;
                        c.g[V1][0] = sx(r32(c, A2).wrapping_add(0x2168));
                        c.g[A1][0] = sx(r32(c, S0) << 4);
                        c.g[A0][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
                        c.g[V0][0] = 0x2;
                        c.sw(r32(c, A0).wrapping_add(0x8), c.g[V0][0]);
                        c.g[V1][0] = c.g[A0][0];
                        c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x1b0));
                        c.g[A1][0] = c.lw(r32(c, V1).wrapping_add(0x4));
                        c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0x10));
                        c.g[A0][0] = c.lw(r32(c, V0));
                        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
                        call_ptr(c, 0x003a_99d8, 0x0022_d56c)?;
                        true
                    }
                    0x0022_d570 => false,
                    _ => {
                        c.jump_out(case, 0x0022_d478, 0x0022_d594);
                        return Ok(());
                    }
                };
                if changed {
                    c.cycles += 1;
                    c.g[A1][0] = 0x1;
                }
            }
            c.cycles += 2;
            let unchanged = c.g[A1][0] == 0;
            c.g[A0][0] = c.g[S1][0];
            if !unchanged {
                c.cycles += 2;
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0022_e298, 0x0022_d580)?;
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
/// `void ScriptList_PopEntries(ScriptList* self, int n)` (0x277598, `func_00277598`): handle up to `n` queued script commands.
/// While `n` is not zero and the command queue (`self+0xc`) is not empty: copy the front command (0x1c bytes, id at +0xc); if its id is not negative and it is not a movie entry (`IsMovieEntry`) stop the script request `ScriptEngine(*self)->0x282540(id)`; for a movie entry play the entry's sound (`ent[6]` when the video engine at `*self+0x500` has an id, else `ent[5]`) through the audio system's virtual `+4` entry when `*self+0x504` is set, then reset the video engine. Pop the command and go on.
pub fn script_list_pop_entries(c: &mut Cpu) {
    guest(c, |c| {
        'b_2776c4: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S1][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S2][0] = c.g[A1][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            let t = c.g[S2][0] == 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            if t {
                break 'b_2776c4;
            } else {
                c.cycles += 1;
                c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
                'l_2775c0: loop {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x00283da0, 0x002775c8)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[S3][0];
                    if t {
                        break 'b_2776c4;
                    } else {
                        'b_2776b0: {
                            'b_2776ac: {
                                c.cycles += 19;
                                c.g[A1][0] = 0u64;
                                call_ptr(c, 0x00283d70, 0x002775d8)?;
                                c.ldl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
                                c.ldr(3, (c.g[V0][0] as u32));
                                c.ldl(4, (c.g[V0][0] as u32).wrapping_add(0xf));
                                c.ldr(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                                c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x17));
                                c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x10));
                                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
                                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                                c.sdr(3, (c.g[SP][0] as u32));
                                c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0xf));
                                c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x8));
                                c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0x17));
                                c.sdr(5, (c.g[SP][0] as u32).wrapping_add(0x10));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A2][0]);
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                                let t = (c.g[V0][0] as i64) < 0;
                                c.g[A0][0] = c.g[S1][0];
                                if t {
                                    break 'b_2776ac;
                                } else {
                                    'b_277640: {
                                        c.cycles += 4;
                                        c.g[A1][0] = c.g[SP][0];
                                        call_ptr(c, 0x00277dd8, 0x00277624)?;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                                            break 'b_277640;
                                        } else {
                                            c.cycles += 5;
                                            c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                                            c.g[A1][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                                            call_ptr(c, 0x00282540, 0x00277638)?;
                                            let t = true;
                                            c.g[A0][0] = c.g[S3][0];
                                            if t {
                                                break 'b_2776b0;
                                            } else {
                                                break 'b_277640;
                                            }
                                        }
                                    }
                                    'b_2776a4: {
                                        c.cycles += 3;
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x504));
                                        if c.g[V1][0] == 0u64 {
                                            c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                                            break 'b_2776a4;
                                        } else {
                                            'b_277674: {
                                                c.cycles += 9;
                                                c.g[A0][0] = c.g[S1][0];
                                                c.g[A1][0] = c.g[SP][0];
                                                call_ptr(c, 0x00277de8, 0x00277658)?;
                                                c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                                                c.g[S0][0] = c.g[V0][0];
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x500));
                                                call_ptr(c, 0x00283bb8, 0x00277668)?;
                                                if c.g[V0][0] == 0u64 {
                                                    c.g[S0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                    break 'b_277674;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[S0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                                    break 'b_277674;
                                                }
                                            }
                                            c.cycles += 2;
                                            if (c.g[S0][0] as i64) < 0 {
                                                c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                                                break 'b_2776a4;
                                            } else {
                                                c.cycles += 10;
                                                call_ptr(c, 0x0028b180, 0x00277684)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x5558));
                                                c.g[A1][0] = c.g[S0][0];
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x118));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                                                c.g[A2][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                                                let t = c.g[A2][0] as u32;
                                                c.g[A0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x002776a0)?;
                                                c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                                                break 'b_2776a4;
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x500));
                                    call_ptr(c, 0x00283818, 0x002776ac)?;
                                    break 'b_2776ac;
                                }
                            }
                            c.cycles += 1;
                            c.g[A0][0] = c.g[S3][0];
                            break 'b_2776b0;
                        }
                        c.cycles += 5;
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x00283dc0, 0x002776b8)?;
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                        let t = c.g[S2][0] != 0u64;

                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x002775c0);
                            }
                            continue 'l_2775c0;
                        } else {
                            break 'b_2776c4;
                        }
                    }
                }
            }
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}
/// `void RiderRailslideSub_PlayEnterAnim(RailslideSub* self, int alt)`
/// (0x1326c8): play the rail-grind enter animation on the rider's animation
/// controller (`rider+0x784`) with blend -1.0 (`AnimController_PlayAnim(anim,
/// id, 0, -1.0f)`), the id picked by the rail style (`rider+0x328`) and `alt`:
/// style 4: 20 (alt: 69); style 3: 19 (alt: 70); otherwise 18 (alt: 68).
pub fn rider_railslide_sub_play_enter_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x4;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(0x8));
        c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x328));
        let style4 = c.g[A0][0] == c.g[V0][0];
        c.g[V0][0] = 0x3;
        // The id and the register state of the compiled branches, up to the call.
        let play_18 = if style4 || c.g[A0][0] == c.g[V0][0] {
            // Styles 4 and 3 differ in two ids.
            let (alt_id, id) = if style4 { (0x45, 0x14) } else { (0x46, 0x13) };
            // (Style 3 got here through one more compare.)
            c.cycles += if style4 { 2 } else { 4 };
            let main = c.g[A1][0] == 0;
            c.g[A1][0] = alt_id;
            if main {
                c.cycles += 3;
                c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x784));
                c.g[A1][0] = id;
            } else {
                c.cycles += 2;
                c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x784));
            }
            false
        } else {
            c.cycles += 2 + 2;
            let main = c.g[A1][0] == 0;
            c.g[A1][0] = 0x44;
            if !main {
                c.cycles += 1;
                c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x784));
            }
            main
        };
        if play_18 {
            c.cycles += 7;
            c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x784));
            c.g[A1][0] = 0x12;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 12);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0031_28e8, 0x0013_2760)?;
        } else {
            c.cycles += 6;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 12);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0031_28e8, 0x0013_2740)?;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}
/// `void Rider_OnScriptSoundEvent(Rider* self, ScriptRider* id, int, uint flags)` (0x123e30): for scripted rider animation events.
/// If `flags & 0x10`, read the script rider's event list (`ScriptRider_ReadFields(id, &info)`: count at +0x10, events at +0x14, 8-byte {id, arg*} entries) and for each event other than 102: ids below 100 are played by the audio system (`OnScriptRiderSpeech2(audio, id, self)`), ids from 100 up are subject-rider speech (`OnScriptRiderSpeech01(audio, id - 100, self, GetSubjectRider(engine, *arg, id->+8))`). Either sets `self+0xac8` to 1.
pub fn rider_on_script_sound_event(c: &mut Cpu) {
    guest(c, |c| {
        'b_123f10: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.g[A3][0] = c.g[A3][0] & 0x10;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S3][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S4][0] = c.g[A1][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
            let t = c.g[A3][0] == 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
            if t {
                break 'b_123f10;
            } else {
                c.cycles += 6;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, 0x0027c9b0, 0x00123e70)?;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[S2][0] = 0u64;
                if t {
                    break 'b_123f10;
                } else {
                    c.cycles += 3;
                    c.g[S6][0] = 0x66;
                    c.g[S5][0] = 0x1;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                    'l_123e88: loop {
                        'b_123efc: {
                            c.cycles += 5;
                            c.g[V0][0] = sx((c.g[S2][0] as u32) << 3);
                            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[S1][0] = c.lw((c.g[S0][0] as u32));
                            let t = c.g[S1][0] == c.g[S6][0];
                            c.g[V0][0] = ((c.g[S1][0] as i64) < 100i64) as u64;
                            if t {
                                break 'b_123efc;
                            } else {
                                'b_123ec4: {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;

                                    if t {
                                        break 'b_123ec4;
                                    } else {
                                        c.cycles += 8;
                                        call_ptr(c, 0x0028b180, 0x00123eac)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A1][0] = c.g[S1][0];
                                        c.g[A2][0] = c.g[S3][0];
                                        call_ptr(c, 0x002a1b58, 0x00123ebc)?;
                                        let t = true;
                                        c.sw((c.g[S3][0] as u32).wrapping_add(0xac8), c.g[S5][0]);
                                        if t {
                                            break 'b_123efc;
                                        } else {
                                            break 'b_123ec4;
                                        }
                                    }
                                }
                                c.cycles += 14;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x64));
                                call_ptr(c, 0x0028b180, 0x00123ecc)?;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A1][0] = c.lb((c.g[V1][0] as u32));
                                c.g[A2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
                                call_ptr(c, 0x00279f18, 0x00123ee4)?;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A1][0] = c.g[S1][0];
                                c.g[A3][0] = c.g[V0][0];
                                c.g[A2][0] = c.g[S3][0];
                                call_ptr(c, 0x002a1b88, 0x00123ef8)?;
                                c.sw((c.g[S3][0] as u32).wrapping_add(0xac8), c.g[S5][0]);
                                break 'b_123efc;
                            }
                        }
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00123e88);
                            }
                            continue 'l_123e88;
                        } else {
                            break 'b_123f10;
                        }
                    }
                }
            }
        }
        c.cycles += 10;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        Ok(())
    })
}
/// `void MultiSplineModifier_CloneInstances(cMultiSplineModifier* self)` (0x35a458): make every node instance a copy of the first.
/// Copy the 0xa0-byte `self->Instance` (+0x40) over node 0 (`NodeInst`, +0x44, an array of instance pointers) keeping node 0's field +0x78, then for nodes 1 to `NumModels - 1` (+4) `InstancesMan_CopyInstance(instancesMan, node, Instance)` and set the flags (+8): clear 0x40 and 0x100, set 0x20.
pub fn multi_spline_modifier_clone_instances(c: &mut Cpu) {
    guest(c, |c| {
        'b_35a530: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.g[A2][0] = 0xa0;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S1][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S2][0] = 0x1;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x44));
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32));
            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x003e6574, 0x0035a498)?;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x44));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x78), c.g[S0][0]);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let t = c.g[V0][0] == 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            if t {
                break 'b_35a530;
            } else {
                c.cycles += 5;
                c.g[S4][0] = 0xffffffffffff0000;
                c.g[S3][0] = 0xffffffffffff0000;
                c.g[S4][0] = c.g[S4][0] | 0xffbf;
                c.g[S3][0] = c.g[S3][0] | 0xfeff;
                'l_35a4c8: loop {
                    c.cycles += 26;
                    c.g[S0][0] = sx((c.g[S2][0] as u32) << 2);
                    call_ptr(c, 0x002d1ce0, 0x0035a4d0)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x44));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[A1][0] = c.lw((c.g[V1][0] as u32));
                    call_ptr(c, 0x00351270, 0x0035a4ec)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x44));
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.g[V1][0] & c.g[S4][0];
                    c.g[V1][0] = c.g[V1][0] | 0x20;
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x44));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.g[V0][0] & c.g[S3][0];
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let t = c.g[V1][0] != 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0035a4c8);
                        }
                        continue 'l_35a4c8;
                    } else {
                        break 'b_35a530;
                    }
                }
            }
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        Ok(())
    })
}
/// `int CameraTrigger_ReadBoxTriggerData(Reader* reader, CameraTrigger* obj)` (0x1730d0): read a box trigger from a stream and return the number of bytes read.
/// Mark the trigger loaded (`obj+0x28 = 1`), then read its five fields through the object's accessor virtuals (vtable at `obj+0x24`, slots +8, +0x28, +0x10, +0x18, +0x20): the center and axis vectors (`t3Vector` reads, 0x2d25a0) and the three float sizes (`Serialize_ReadU32`). Returns the summed sizes.
pub fn camera_trigger_read_box_trigger_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 60;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.g[V0][0] = 0x1;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = c.g[A1][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S2][0] = c.g[A0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00173108)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d25a0, 0x00173114)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0017312c)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d25a0, 0x00173138)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00173150)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d2510, 0x0017315c)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00173174)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d2510, 0x00173180)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00173198)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d2510, 0x001731a4)?;
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}
/// `int ScriptList_Start(ScriptList* self, int fade, int notify)` (0x276388, `tNISList_Play`): start the front script (or movie) of a loaded NIS list.
/// If the list is still loading (state +0xa4 == 1) poll it; if it is then not ready (state != 2) return the state. Otherwise set the state to 3, tell the HUD state (`UIStateStack_FindState` of the string hash 0x4a34a0) to hide through its virtual +0xc0, and take the front command (`ScriptCmdQueue_At(self+0xc, 0)`). A script command plays its script (`PlayScriptInstance(engine, self+8, id, 0, flags >> 3 & 1)`); when `self+4` is set it also stops the running transition and, with `fade`, starts the transition of the script data (the in, hold and out times in frames at +0xa, +0xc and +0xe of its info: `MakeTransitionObject` twice and `cTransitionMan_StartTransition`, or a screen grab for kind 7), pauses the process when a fade ran (`+0xac`, `+0xb0` = 1) and starts the cut scene transition (a fade of at least 0.5 s); finally `notify` calls the engine virtual +0x30 (slot 6). A movie command forces the cut scene mode (when `self+4`) and starts the video (`PlayVideo`, `UpdateVideo` with `notify`). Returns the state again.
pub fn script_list_start(c: &mut Cpu) {
    guest(c, |c| {
        'b_2763dc: {
            c.cycles += 18;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.g[V0][0] = 0x1;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S1][0] = c.g[A0][0];
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S2][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xa4));
            let t = c.g[V1][0] != c.g[V0][0];
            c.g[FP][0] = c.g[A2][0];
            if t {
                break 'b_2763dc;
            } else {
                c.cycles += 3;
                call_ptr(c, 0x00277778, 0x002763d8)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xa4));
                break 'b_2763dc;
            }
        }
        'b_27667c: {
            c.cycles += 3;
            c.g[V0][0] = 0x2;
            let t = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = c.g[V1][0];
            if t {
                break 'b_27667c;
            } else {
                'b_27644c: {
                    c.cycles += 19;
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x00283d70, 0x002763f4)?;
                    c.g[S6][0] = c.g[V0][0];
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[V0][0] = 0x3;
                    c.g[A0][0] = 0x4a0000;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xa4), c.g[V0][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x34a0));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x00317670, 0x0027641c)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039f9d8, 0x00276428)?;
                    c.g[A3][0] = c.g[V0][0];
                    let t = c.g[A3][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_27644c;
                    } else {
                        c.cycles += 6;
                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                        c.g[A2][0] = 0u64;
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x0027644c)?;
                        break 'b_27644c;
                    }
                }
                'b_276644: {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S6][0];
                    call_ptr(c, 0x00277dd8, 0x00276458)?;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                        break 'b_276644;
                    } else {
                        'b_276618: {
                            c.cycles += 11;
                            c.g[T0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x8));
                            c.g[A3][0] = 0u64;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                            c.g[T0][0] = sx(((c.g[T0][0] as u32) as i32 >> 3) as u32);
                            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A2][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xc));
                            c.g[T0][0] = c.g[T0][0] & 0x1;
                            call_ptr(c, 0x00279720, 0x00276480)?;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                            let t = c.g[V1][0] == 0u64;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            if t {
                                break 'b_276618;
                            } else {
                                'b_2764b4: {
                                    c.cycles += 7;
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                    call_ptr(c, 0x00230698, 0x00276498)?;
                                    c.g[S3][0] = c.g[V0][0];
                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x44));
                                    if c.g[V0][0] == 0u64 {
                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                                        break 'b_2764b4;
                                    } else {
                                        c.cycles += 3;
                                        c.g[A0][0] = c.g[S3][0];
                                        call_ptr(c, 0x002e4578, 0x002764b0)?;
                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                                        break 'b_2764b4;
                                    }
                                }
                                'b_2764e0: {
                                    c.cycles += 9;
                                    c.g[S4][0] = 0u64;
                                    c.g[A1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xc));
                                    c.mtc1(0, 20);
                                    c.g[S5][0] = 0u64;
                                    call_ptr(c, 0x00282bf0, 0x002764c8)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    call_ptr(c, 0x0027c070, 0x002764d0)?;
                                    let t = c.g[S2][0] == 0u64;
                                    c.g[S0][0] = c.g[V0][0];
                                    if t {
                                        break 'b_2764e0;
                                    } else {
                                        c.cycles += 2;
                                        c.g[S4][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0xa));
                                        c.g[S5][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0xc));
                                        break 'b_2764e0;
                                    }
                                }
                                'b_2765ec: {
                                    c.cycles += 6;
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0xb8), 0u64);
                                    c.g[S7][0] =
                                        sx((c.g[S4][0] as u32).wrapping_add((c.g[S5][0] as u32)));
                                    c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0xe));
                                    c.g[V0][0] =
                                        sx((c.g[S7][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    let t = (c.g[V0][0] as i64) <= 0;
                                    c.g[V0][0] = 0x7;
                                    if t {
                                        break 'b_2765ec;
                                    } else {
                                        'b_2765c8: {
                                            'b_2765c4: {
                                                'b_276584: {
                                                    c.cycles += 3;
                                                    c.g[A1][0] =
                                                        c.lb((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    let t = c.g[A1][0] == c.g[V0][0];

                                                    if t {
                                                        break 'b_276584;
                                                    } else {
                                                        c.cycles += 6;
                                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                                                        c.g[A2][0] = c
                                                            .lb((c.g[S0][0] as u32)
                                                                .wrapping_add(0x9));
                                                        call_ptr(c, 0x0027a4a0, 0x00276510)?;
                                                        c.g[S2][0] = c.g[V0][0];
                                                        if c.g[S2][0] == 0u64 {
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0xb8));
                                                            break 'b_2765c8;
                                                        } else {
                                                            c.cycles += 26;
                                                            c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                                                            c.g[A1][0] = c.lb((c.g[S0][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[A2][0] = c.lb((c.g[S0][0] as u32)
                                                                .wrapping_add(0x9));
                                                            call_ptr(c, 0x0027a4a0, 0x0027652c)?;
                                                            c.g[V1][0] = c.lh((c.g[S0][0] as u32)
                                                                .wrapping_add(0xe));
                                                            c.mtc1(20, 1);
                                                            c.cvt_s_w(1, 1);
                                                            c.lwc1(
                                                                0,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x4804),
                                                            );
                                                            c.mtc1(21, 13);
                                                            c.cvt_s_w(13, 13);
                                                            c.mtc1(3, 14);
                                                            c.cvt_s_w(14, 14);
                                                            c.g[A0][0] = c.g[S3][0];
                                                            c.mul_s(1, 1, 0);
                                                            c.g[A2][0] = c.g[S2][0];
                                                            c.mul_s(13, 13, 0);
                                                            c.g[A3][0] = c.g[V0][0];
                                                            c.mul_s(14, 14, 0);
                                                            c.g[A1][0] = 0x2;
                                                            c.mov_s(12, 1);
                                                            c.g[T0][0] = 0u64;
                                                            c.g[T1][0] = 0u64;
                                                            c.mov_s(20, 1);
                                                            call_ptr(c, 0x002e4370, 0x0027657c)?;
                                                            let t = true;
                                                            c.sw(
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0xb8),
                                                                c.g[S7][0],
                                                            );
                                                            if t {
                                                                break 'b_2765c4;
                                                            } else {
                                                                break 'b_276584;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = c.g[S4][0] == 0u64;
                                                c.g[A1][0] = 0x7;
                                                if t {
                                                    break 'b_2765c4;
                                                } else {
                                                    c.cycles += 14;
                                                    c.g[A2][0] =
                                                        c.lb((c.g[S0][0] as u32).wrapping_add(0x9));
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x84c)
                                                        );
                                                    call_ptr(c, 0x0027a4a0, 0x00276598)?;
                                                    c.mtc1(20, 0);
                                                    c.cvt_s_w(0, 0);
                                                    c.lwc1(
                                                        1,
                                                        (c.g[GP][0] as u32).wrapping_sub(0x4800),
                                                    );
                                                    c.g[A0][0] = c.g[S3][0];
                                                    c.g[A2][0] = c.g[V0][0];
                                                    c.g[A1][0] = 0x2;
                                                    c.g[A3][0] = 0u64;
                                                    c.mul_s(0, 0, 1);
                                                    c.mov_s(12, 0);
                                                    c.mov_s(20, 0);
                                                    call_ptr(c, 0x002e4540, 0x002765c4)?;
                                                    break 'b_2765c4;
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0xb8));
                                            break 'b_2765c8;
                                        }
                                        c.cycles += 2;
                                        let t = (c.g[V0][0] as i64) <= 0;

                                        if t {
                                            break 'b_2765ec;
                                        } else {
                                            c.cycles += 7;
                                            c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                                            c.g[FP][0] = 0u64;
                                            c.g[A1][0] =
                                                c.lw((c.g[S6][0] as u32).wrapping_add(0xc));
                                            call_ptr(c, 0x002826d8, 0x002765e0)?;
                                            c.g[V0][0] = 0x1;
                                            c.sw(
                                                (c.g[S1][0] as u32).wrapping_add(0xb0),
                                                c.g[V0][0],
                                            );
                                            c.sw(
                                                (c.g[S1][0] as u32).wrapping_add(0xac),
                                                c.g[V0][0],
                                            );
                                            break 'b_2765ec;
                                        }
                                    }
                                }
                                'b_276608: {
                                    c.cycles += 5;
                                    c.mtc1(0, 0);
                                    c.c_le_s(20, 0);
                                    let t = !c.fcc();
                                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    if t {
                                        break 'b_276608;
                                    } else {
                                        c.cycles += 2;
                                        c.g[AT][0] = 0x3f000000;
                                        c.mtc1(1, 20);
                                        break 'b_276608;
                                    }
                                }
                                c.cycles += 4;
                                c.mov_s(12, 20);
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64));
                                call_ptr(c, 0x002ea6b8, 0x00276618)?;
                                break 'b_276618;
                            }
                        }
                        c.cycles += 2;
                        if c.g[FP][0] == 0u64 {
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xa4));
                            break 'b_27667c;
                        } else {
                            c.cycles += 9;
                            c.g[A2][0] = c.lw((c.g[S1][0] as u32));
                            c.g[A1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xc));
                            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x2a8));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x30));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x34));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x0027663c)?;
                            let t = true;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xa4));
                            if t {
                                break 'b_27667c;
                            } else {
                                break 'b_276644;
                            }
                        }
                    }
                }
                'b_276658: {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        break 'b_276658;
                    } else {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x64));
                        call_ptr(c, 0x002ea860, 0x00276658)?;
                        break 'b_276658;
                    }
                }
                c.cycles += 5;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x500));
                call_ptr(c, 0x002839a8, 0x00276664)?;
                if c.g[FP][0] == 0u64 {
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xa4));
                    break 'b_27667c;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x500));
                    call_ptr(c, 0x00283aa0, 0x00276678)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xa4));
                    break 'b_27667c;
                }
            }
        }
        c.cycles += 13;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        Ok(())
    })
}
/// `void ScriptEngine_EvalChoiceParams(void*, int* out, int a2, int a3, int a4)` (0x27bdb8; no C++ reference, Ghidra cannot decompile it): fill `out[0..21]`.
/// The engine's tables are `E = (*(gp-0x848)+0x84)->+0xc` with counts at +0x7c, +0x80, +0x84 and +0x78. Each `out[i]` is cleared and then, by a jump table at 0x481ec0 indexed by `i`, set from a rider slot (a2, a3, a4 select rider records at `E+4*idx+0x28`, the table entries at +0x40, +0x48 and +0x5c, or +0x28 plus the count checks) through `Rider_GetCharacter` and the character tables at 0x481bb0: `out[i] = charTable[Rider_GetCharacter(rider)]`. Slots out of range stay 0. Resume entries go through the generated `__entries` twin.
// NOT INSTALLED (no OVERRIDES entry, no test): no C++ reference, Ghidra cannot decompile it,
// and the differential fixture ran away at 0x27be10 without a cause found.
#[allow(dead_code)]
pub fn script_engine_eval_choice_params(c: &mut Cpu) {
    if c.pc != 0x0027_bdb8 {
        // The other entries (resume points after a callee unwound): the
        // structured form's `__entries` twin; the unwind form's body
        // dispatches on `c.pc` itself.
        #[cfg(not(feature = "ee-unwind"))]
        return ee::s07::f_0027bdb8__entries(c);
        #[cfg(feature = "ee-unwind")]
        return ee::s07::f_0027bdb8__generated(c);
    }
    guest(c, |c| {
        c.cycles += 22;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S2][0] = c.g[A1][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S3][0] = 0u64;
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S4][0] = c.g[T0][0];
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S5][0] = c.g[A3][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S6][0] = c.g[A2][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        c.g[FP][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
        c.g[S7][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
        c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
        'l_27be10: loop {
            'b_27bf10: {
                c.cycles += 5;
                c.g[V1][0] = (c.g[S3][0] < 0x15u64) as u64;
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                let t = c.g[V1][0] == 0u64;
                c.sw((c.g[V0][0] as u32), 0u64);
                if t {
                    break 'b_27bf10;
                } else {
                    'b_27beec: {
                        'b_27bee8: {
                            'b_27bed0: {
                                'b_27beac: {
                                    'b_27be8c: {
                                        'b_27be6c: {
                                            'b_27be5c: {
                                                'b_27be4c: {
                                                    c.cycles += 6;
                                                    c.g[V0][0] = 0x480000;
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x1ec0));
                                                    c.g[V0][0] = sx((c.g[S0][0] as u32)
                                                        .wrapping_add((c.g[V0][0] as u32)));
                                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                    let t = c.g[V1][0] as u32;
                                                    match t {
                                                        0x0027be3c => {
                                                            c.cycles += 2;
                                                            let t = (c.g[S6][0] as i64) < 0;
                                                            c.g[V0][0] =
                                                                sx((c.g[S6][0] as u32) << 2);
                                                            if t {
                                                                break 'b_27bf10;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.g[V0][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[V0][0] as u32),
                                                                    ));
                                                                if t {
                                                                    break 'b_27bee8;
                                                                } else {
                                                                    break 'b_27be4c;
                                                                }
                                                            }
                                                        }
                                                        0x0027be4c => {
                                                            break 'b_27be4c;
                                                        }
                                                        0x0027be5c => {
                                                            break 'b_27be5c;
                                                        }
                                                        0x0027be6c => {
                                                            break 'b_27be6c;
                                                        }
                                                        0x0027be8c => {
                                                            break 'b_27be8c;
                                                        }
                                                        0x0027beac => {
                                                            break 'b_27beac;
                                                        }
                                                        0x0027bed0 => {
                                                            break 'b_27bed0;
                                                        }
                                                        _ => {
                                                            c.jump_out(t, 0x0027bdb8, 0x0027bf50);
                                                            return Ok(());
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = (c.g[S5][0] as i64) < 0;
                                                c.g[V0][0] = sx((c.g[S5][0] as u32) << 2);
                                                if t {
                                                    break 'b_27bf10;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[V0][0] = sx((c.g[S1][0] as u32)
                                                        .wrapping_add((c.g[V0][0] as u32)));
                                                    if t {
                                                        break 'b_27bee8;
                                                    } else {
                                                        break 'b_27be5c;
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            let t = (c.g[S4][0] as i64) < 0;
                                            c.g[V0][0] = sx((c.g[S4][0] as u32) << 2);
                                            if t {
                                                break 'b_27bf10;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[V0][0] = sx((c.g[S1][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                if t {
                                                    break 'b_27bee8;
                                                } else {
                                                    break 'b_27be6c;
                                                }
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x3));
                                        c.g[V0][0] =
                                            ((c.g[V1][0] as i64) < (c.g[FP][0] as i64)) as u64;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                                        if t {
                                            break 'b_27bf10;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] = sx((c.g[S1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x40));
                                            let t = true;
                                            c.g[A0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                                            if t {
                                                break 'b_27beec;
                                            } else {
                                                break 'b_27be8c;
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x5));
                                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S7][0] as i64)) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                                    if t {
                                        break 'b_27bf10;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V0][0] =
                                            sx((c.g[S1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x48));
                                        let t = true;
                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                                        if t {
                                            break 'b_27beec;
                                        } else {
                                            break 'b_27beac;
                                        }
                                    }
                                }
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                                c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0xa));
                                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                                if t {
                                    break 'b_27bf10;
                                } else {
                                    c.cycles += 4;
                                    c.g[V0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                    let t = true;
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                                    if t {
                                        break 'b_27beec;
                                    } else {
                                        break 'b_27bed0;
                                    }
                                }
                            }
                            c.cycles += 5;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                            c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0xf));
                            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                            if t {
                                break 'b_27bf10;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                break 'b_27bee8;
                            }
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                        break 'b_27beec;
                    }
                    c.cycles += 9;
                    call_ptr(c, 0x00123128, 0x0027bef4)?;
                    c.g[A0][0] = 0x480000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1bb0));
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                    c.sw((c.g[A1][0] as u32), c.g[V1][0]);
                    break 'b_27bf10;
                }
            }
            c.cycles += 4;
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[S3][0] as i64) < 21i64) as u64;
            if c.g[V0][0] != 0u64 {
                c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
                if event_due(c) {
                    deliver_event_here(c, 0x0027be10);
                }
                continue 'l_27be10;
            } else {
                c.cycles += 12;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                c.pc = t;
            }
        }
        Ok(())
    })
}
/// `void ScriptViewObj_BindCamSlot(SseEngine* self, int ch, int anim)` (0x281ac8): bind script camera animation `anim` to a camera rig for channel `ch`.
/// The channel's rig slot is `self->chans[ch]` (+0xdc, words); when unset (negative) pick the first of the two rigs (+0xd0) that exists and is not busy
/// (`rig+8 <= 0`), unless `anim == 9` (unbind: `SetBinding(rig, 0)` and the channel back to -1). A rig that does not already play `anim` gets the first
/// of the nine animation entries (`ScriptBinding_IsBound`) with that id (`ScriptObjCam_SetBinding(rig, entry)`). Written as the dispatcher the generated
/// body is (resume entries included).
pub fn script_view_obj_bind_cam_slot(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x00281ac8 => {
                c.cycles += 16;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S6][0] = c.g[A1][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S3][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[V0][0] = sx((c.g[S6][0] as u32) << 2);
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xdc));
                let t = (c.g[S2][0] as i64) >= 0;
                c.g[S4][0] = c.g[A2][0];
                if t {
                    pc = 0x00281b70;
                    continue;
                }
                pc = 0x00281b08;
                continue;
            }
            0x00281b08 => {
                c.cycles += 3;
                c.g[V0][0] = 0x9;
                let t = c.g[S4][0] == c.g[V0][0];
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                if t {
                    pc = 0x00281c18;
                    continue;
                }
                pc = 0x00281b14;
                continue;
            }
            0x00281b14 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xd0));
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = 0u64;
                if t {
                    pc = 0x00281b38;
                    continue;
                }
                pc = 0x00281b20;
                continue;
            }
            0x00281b20 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                let t = (c.g[V0][0] as i64) > 0;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                if t {
                    pc = 0x00281b3c;
                    continue;
                }
                pc = 0x00281b2c;
                continue;
            }
            0x00281b2c => {
                c.cycles += 2;
                let t = true;
                c.g[S2][0] = 0u64;
                if t {
                    pc = 0x00281b68;
                    continue;
                }
                pc = 0x00281b34;
                continue;
            }
            0x00281b34 => {
                c.cycles += 1;
                pc = 0x00281b38;
                continue;
            }
            0x00281b38 => {
                c.cycles += 1;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                pc = 0x00281b3c;
                continue;
            }
            0x00281b3c => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                if t {
                    pc = 0x00281b68;
                    continue;
                }
                pc = 0x00281b48;
                continue;
            }
            0x00281b48 => {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));

                if c.g[V0][0] == 0u64 {
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    if c.cycles >= c.next_event {
                        c.pc = 0x00281b3c;
                        if c.event() {
                            return;
                        }
                    }
                    pc = 0x00281b3c;
                    continue;
                }
                pc = 0x00281b58;
                continue;
            }
            0x00281b58 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));

                if (c.g[V0][0] as i64) > 0 {
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    if c.cycles >= c.next_event {
                        c.pc = 0x00281b3c;
                        if c.event() {
                            return;
                        }
                    }
                    pc = 0x00281b3c;
                    continue;
                }
                pc = 0x00281b64;
                continue;
            }
            0x00281b64 => {
                c.cycles += 1;
                c.g[S2][0] = c.g[V1][0];
                pc = 0x00281b68;
                continue;
            }
            0x00281b68 => {
                c.cycles += 2;
                let t = (c.g[S2][0] as i64) < 0;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                if t {
                    pc = 0x00281c18;
                    continue;
                }
                pc = 0x00281b70;
                continue;
            }
            0x00281b70 => {
                c.cycles += 3;
                c.g[V0][0] = 0x9;
                let t = c.g[S4][0] != c.g[V0][0];
                c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                if t {
                    pc = 0x00281ba0;
                    continue;
                }
                pc = 0x00281b7c;
                continue;
            }
            0x00281b7c => {
                c.cycles += 9;
                c.g[A1][0] = 0u64;
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[RA][0] = sx(0x00281b8c);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                c.pc = 0x0027d688;
                call_indirect(c, 0x0027d688);
                if c.pc != 0x00281b8c {
                    return;
                }
                c.g[V0][0] = sx((c.g[S6][0] as u32) << 2);
                c.g[V1][0] = 0xffffffffffffffff;
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                let t = true;
                c.sw((c.g[V0][0] as u32).wrapping_add(0xdc), c.g[V1][0]);
                if t {
                    pc = 0x00281c14;
                    continue;
                }
                pc = 0x00281ba0;
                continue;
            }
            0x00281ba0 => {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(0xd0));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
                let t = c.g[V0][0] == 0u64;
                c.g[S5][0] = c.g[V1][0];
                if t {
                    pc = 0x00281bc4;
                    continue;
                }
                pc = 0x00281bb8;
                continue;
            }
            0x00281bb8 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                let t = c.g[V0][0] == c.g[S4][0];
                c.g[V0][0] = sx((c.g[S6][0] as u32) << 2);
                if t {
                    pc = 0x00281c0c;
                    continue;
                }
                pc = 0x00281bc4;
                continue;
            }
            0x00281bc4 => {
                c.cycles += 3;
                c.g[S1][0] = 0u64;
                let t = true;
                c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1c));
                if t {
                    pc = 0x00281bd4;
                    continue;
                }
                pc = 0x00281bd0;
                continue;
            }
            0x00281bd0 => {
                c.cycles += 1;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                pc = 0x00281bd4;
                continue;
            }
            0x00281bd4 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 9i64) as u64;

                if c.g[V0][0] == 0u64 {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                    pc = 0x00281c18;
                    continue;
                }
                pc = 0x00281be0;
                continue;
            }
            0x00281be0 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00281be8);
                c.g[A0][0] = c.g[S0][0];
                c.pc = 0x00282f80;
                call_indirect(c, 0x00282f80);
                if c.pc != 0x00281be8 {
                    return;
                }

                if c.g[V0][0] == 0u64 {
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
                    if c.cycles >= c.next_event {
                        c.pc = 0x00281bd0;
                        if c.event() {
                            return;
                        }
                    }
                    pc = 0x00281bd0;
                    continue;
                }
                pc = 0x00281bf0;
                continue;
            }
            0x00281bf0 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));

                if c.g[V0][0] != c.g[S4][0] {
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
                    if c.cycles >= c.next_event {
                        c.pc = 0x00281bd0;
                        if c.event() {
                            return;
                        }
                    }
                    pc = 0x00281bd0;
                    continue;
                }
                pc = 0x00281bfc;
                continue;
            }
            0x00281bfc => {
                c.cycles += 4;
                c.g[A0][0] = c.lw((c.g[S5][0] as u32));
                c.g[RA][0] = sx(0x00281c08);
                c.g[A1][0] = c.g[S0][0];
                c.pc = 0x0027d688;
                call_indirect(c, 0x0027d688);
                if c.pc != 0x00281c08 {
                    return;
                }
                c.g[V0][0] = sx((c.g[S6][0] as u32) << 2);
                pc = 0x00281c0c;
                continue;
            }
            0x00281c0c => {
                c.cycles += 2;
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sw((c.g[V0][0] as u32).wrapping_add(0xdc), c.g[S2][0]);
                pc = 0x00281c14;
                continue;
            }
            0x00281c14 => {
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                pc = 0x00281c18;
                continue;
            }
            0x00281c18 => {
                c.cycles += 9;
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                c.pc = t;
                return;
            }
            _ => {
                c.jump_out(pc, 0x00281ac8, 0x00281c3c);
                return;
            }
        }
    }
}
/// `void RailModifier::~RailModifier(RailModifier* self, int flags)` (0x35bd70, `RailModifier_v01`): the rail modifier destructor.
/// Sets its vtable (+8 = 0x4911d0), removes the node (`self+0x44`) from the Nash spatial hash (`SpatialKey_FromAABB` of its box at +0x10, `SpatialHash_Remove` from the
/// one of the eight hashes the key signs pick, freeing the hash storage when it became empty), frees the node, re-inserts every segment box of its spline (found in
/// the world's spline table by the id at +0x30) into the hash, restores the `RailLink` vtable (+8 = 0x4912..) and unlinks (`RailModifier_ListUnlink`); with `flags & 1` frees itself.
/// Written as the dispatcher the generated body is (resume entries included).
pub fn rail_modifier_dtor(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x0035bd70 => {
                c.cycles += 48;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
                c.g[V0][0] = 0x490000;
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x11d0));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.g[S3][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S2][0] = c.g[A1][0];
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.g[RA][0] = sx(0x0035bdb4);
                c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.pc = 0x002d1be0;
                call_indirect(c, 0x002d1be0);
                if c.pc != 0x0035bdb4 {
                    return;
                }
                c.g[S0][0] = c.g[V0][0];
                c.g[S1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x44));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                c.g[RA][0] = sx(0x0035bdd4);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[V0][0]);
                c.pc = 0x00328f28;
                call_indirect(c, 0x00328f28);
                if c.pc != 0x0035bdd4 {
                    return;
                }
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                c.g[A3][0] = 0xffffffffffffffff;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = c.g[S0][0];
                }
                c.g[A0][0] = ((c.g[A3][0] as i64) < (c.g[A0][0] as i64)) as u64;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x28));
                c.g[A3][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
                if c.g[A0][0] != 0 {
                    c.g[V0][0] = c.g[V1][0];
                }
                c.g[A2][0] = c.g[S1][0];
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
                c.g[A1][0] = 0x2;
                if c.g[A3][0] != 0 {
                    c.g[S0][0] = c.g[V0][0];
                }
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[A3][0] = c.g[SP][0];
                c.g[RA][0] = sx(0x0035be20);
                c.g[T0][0] = c.g[S0][0];
                c.pc = 0x003284b8;
                call_indirect(c, 0x003284b8);
                if c.pc != 0x0035be20 {
                    return;
                }
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                let t = c.g[V0][0] != 0u64;
                c.g[S6][0] = c.g[S2][0] & 0x1;
                if t {
                    pc = 0x0035be84;
                    continue;
                }
                pc = 0x0035be30;
                continue;
            }
            0x0035be30 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0u64;
                if t {
                    pc = 0x0035be88;
                    continue;
                }
                pc = 0x0035be3c;
                continue;
            }
            0x0035be3c => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0u64;
                if t {
                    pc = 0x0035be88;
                    continue;
                }
                pc = 0x0035be48;
                continue;
            }
            0x0035be48 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0u64;
                if t {
                    pc = 0x0035be88;
                    continue;
                }
                pc = 0x0035be54;
                continue;
            }
            0x0035be54 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0u64;
                if t {
                    pc = 0x0035be88;
                    continue;
                }
                pc = 0x0035be60;
                continue;
            }
            0x0035be60 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0u64;
                if t {
                    pc = 0x0035be88;
                    continue;
                }
                pc = 0x0035be6c;
                continue;
            }
            0x0035be6c => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0u64;
                if t {
                    pc = 0x0035be88;
                    continue;
                }
                pc = 0x0035be78;
                continue;
            }
            0x0035be78 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                if t {
                    pc = 0x0035be98;
                    continue;
                }
                pc = 0x0035be84;
                continue;
            }
            0x0035be84 => {
                c.cycles += 1;
                c.g[V0][0] = 0u64;
                pc = 0x0035be88;
                continue;
            }
            0x0035be88 => {
                c.cycles += 2;
                let t = true;
                c.g[S7][0] = 0x490000;
                if t {
                    pc = 0x0035bebc;
                    continue;
                }
                pc = 0x0035be90;
                continue;
            }
            0x0035be90 => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0u64;
                if t {
                    pc = 0x0035bebc;
                    continue;
                }
                pc = 0x0035be98;
                continue;
            }
            0x0035be98 => {
                c.cycles += 2;
                c.g[A0][0] = 0u64;
                c.g[S7][0] = 0x490000;
                pc = 0x0035bea0;
                continue;
            }
            0x0035bea0 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                let t = c.g[V0][0] != 0u64;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                if t {
                    if c.cycles >= c.next_event {
                        c.pc = 0x0035be90;
                        if c.event() {
                            return;
                        }
                    }
                    pc = 0x0035be90;
                    continue;
                }
                pc = 0x0035beac;
                continue;
            }
            0x0035beac => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A0][0] as i64) < 3i64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                if t {
                    if c.cycles >= c.next_event {
                        c.pc = 0x0035bea0;
                        if c.event() {
                            return;
                        }
                    }
                    pc = 0x0035bea0;
                    continue;
                }
                pc = 0x0035beb8;
                continue;
            }
            0x0035beb8 => {
                c.cycles += 1;
                c.g[V0][0] = 0x1;
                pc = 0x0035bebc;
                continue;
            }
            0x0035bebc => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;

                if t {
                    pc = 0x0035bed0;
                    continue;
                }
                pc = 0x0035bec4;
                continue;
            }
            0x0035bec4 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x0035becc);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                c.pc = 0x00317e50;
                call_indirect(c, 0x00317e50);
                if c.pc != 0x0035becc {
                    return;
                }
                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
                pc = 0x0035bed0;
                continue;
            }
            0x0035bed0 => {
                c.cycles += 13;
                c.g[RA][0] = sx(0x0035bed8);
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x44));
                c.pc = 0x00317e50;
                call_indirect(c, 0x00317e50);
                if c.pc != 0x0035bed8 {
                    return;
                }
                c.g[RA][0] = sx(0x0035bee0);
                c.pc = 0x002d1bd8;
                call_indirect(c, 0x002d1bd8);
                if c.pc != 0x0035bee0 {
                    return;
                }
                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.g[A1][0] & 0xff;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32));

                if c.g[V1][0] != 0u64 {
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                    pc = 0x0035bf0c;
                    continue;
                }
                pc = 0x0035bf04;
                continue;
            }
            0x0035bf04 => {
                c.cycles += 2;
                let t = true;
                c.g[S5][0] = 0u64;
                if t {
                    pc = 0x0035bf30;
                    continue;
                }
                pc = 0x0035bf0c;
                continue;
            }
            0x0035bf0c => {
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if t {
                    pc = 0x0035bf2c;
                    continue;
                }
                pc = 0x0035bf28;
                continue;
            }
            0x0035bf28 => {
                c.cycles += 1;
                c.g[V0][0] = 0u64;
                pc = 0x0035bf2c;
                continue;
            }
            0x0035bf2c => {
                c.cycles += 1;
                c.g[S5][0] = c.g[V0][0];
                pc = 0x0035bf30;
                continue;
            }
            0x0035bf30 => {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x20));
                c.g[S2][0] = 0u64;
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[S1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x24));
                if t {
                    pc = 0x0035bff0;
                    continue;
                }
                pc = 0x0035bf40;
                continue;
            }
            0x0035bf40 => {
                c.cycles += 4;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 20);
                c.g[S4][0] = 0xffffffffffffffff;
                pc = 0x0035bf50;
                continue;
            }
            0x0035bf50 => {
                c.cycles += 40;
                c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x6c));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x70));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
                c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x74));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x1c));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x78));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x7c));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x24));
                c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x80));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x2c));
                c.g[RA][0] = sx(0x0035bf90);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x28));
                c.pc = 0x002d1be0;
                call_indirect(c, 0x002d1be0);
                if c.pc != 0x0035bf90 {
                    return;
                }
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[SP][0];
                c.g[RA][0] = sx(0x0035bfa0);
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.pc = 0x00328f28;
                call_indirect(c, 0x00328f28);
                if c.pc != 0x0035bfa0 {
                    return;
                }
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = c.g[S1][0];
                c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = c.g[S0][0];
                }
                c.g[A1][0] = ((c.g[S4][0] as i64) < (c.g[A1][0] as i64)) as u64;
                c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x28));
                c.g[T0][0] = ((c.g[S4][0] as i64) < (c.g[T0][0] as i64)) as u64;
                if c.g[A1][0] != 0 {
                    c.g[A0][0] = c.g[V1][0];
                }
                c.g[A3][0] = c.g[SP][0];
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
                c.g[A1][0] = 0x2;
                c.g[RA][0] = sx(0x0035bfe0);
                if c.g[T0][0] == 0 {
                    c.g[A0][0] = c.g[V0][0];
                }
                c.pc = 0x00328c20;
                call_indirect(c, 0x00328c20);
                if c.pc != 0x0035bfe0 {
                    return;
                }
                c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[S1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
                if t {
                    if c.cycles >= c.next_event {
                        c.pc = 0x0035bf50;
                        if c.event() {
                            return;
                        }
                    }
                    pc = 0x0035bf50;
                    continue;
                }
                pc = 0x0035bff0;
                continue;
            }
            0x0035bff0 => {
                c.cycles += 6;
                c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add(0x1200));
                c.g[A0][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x0035c000);
                c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.pc = 0x0035b6d0;
                call_indirect(c, 0x0035b6d0);
                if c.pc != 0x0035c000 {
                    return;
                }
                let t = c.g[S6][0] == 0u64;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                if t {
                    pc = 0x0035c014;
                    continue;
                }
                pc = 0x0035c008;
                continue;
            }
            0x0035c008 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x0035c010);
                c.g[A0][0] = c.g[S3][0];
                c.pc = 0x00317e50;
                call_indirect(c, 0x00317e50);
                if c.pc != 0x0035c010 {
                    return;
                }
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                pc = 0x0035c014;
                continue;
            }
            0x0035c014 => {
                c.cycles += 11;
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
                c.pc = t;
                return;
            }
            _ => {
                c.jump_out(pc, 0x0035bd70, 0x0035c040);
                return;
            }
        }
    }
}
/// `int Nash_ProbeVertical(NashCells* self, float* pos, float* outP, float* outN)` (0x3369d8, `func_003369D8`): find the ground hit under a position.
/// Build a vertical segment through `pos` (z - 100000 to z + 100000, `CollSegment_ctor` with parameter t = clamp((z + 20 - lo) / (hi - lo), 0, 1)), collide it with each of the 8 cells of `self` (20-byte cells, nodes at +0x10, +0x24, ...; `Cell_CollideColumnRecursive` collects up to 64 hits of 0x80 bytes). Returns 0 without hits; else picks the hit whose z is nearest to `pos[2]` (ties broken by the hit's two object pointers at +0x50 and +0x54, the one with the lower field at +0x78 / +0x150 order), copies its point and normal (two quadwords at +0 and +0x10) to `outP` and `outN` and returns 1. Other entries are served by the generated `__entries` twin.
pub fn nash_probe_vertical(c: &mut Cpu) {
    if c.pc != 0x0033_69d8 {
        // The other entries (resume points after a callee unwound): the
        // structured form's `__entries` twin; the unwind form's body
        // dispatches on `c.pc` itself.
        #[cfg(not(feature = "ee-unwind"))]
        return ee::s11::f_003369d8__entries(c);
        #[cfg(feature = "ee-unwind")]
        return ee::s11::f_003369d8__generated(c);
    }
    guest(c, |c| {
        'b_336a5c: {
            c.cycles += 32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x2190));
            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x2d5c));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x2180));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x2160));
            c.g[S0][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x2170));
            c.g[S2][0] = c.g[A1][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x2150));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x2140));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x2130));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x2120));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x2110));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x2100));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20f0), c.g[RA][0]);
            c.g[AT][0] = 0x41a00000;
            c.mtc1(1, 3);
            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x8));
            c.mtc1(0, 2);
            c.sub_s(4, 0, 1);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x20e0), c.g[A2][0]);
            c.add_s(6, 0, 1);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x20e4), c.g[A3][0]);
            c.add_s(0, 0, 3);
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 5);
            c.sub_s(1, 6, 4);
            c.sub_s(0, 0, 4);
            c.div_s(0, 0, 1);
            c.c_le_s(2, 0);
            let t = !c.fcc();
            c.mov_s(12, 2);
            if t {
                break 'b_336a5c;
            } else {
                c.cycles += 1;
                c.min_s(12, 0, 5);
                break 'b_336a5c;
            }
        }
        'b_336b18: {
            c.cycles += 41;
            c.lwc1(0, (c.g[S2][0] as u32));
            c.g[A0][0] = c.g[SP][0];
            c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x4));
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
            c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0xb8));
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.g[A3][0] = 0x2;
            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xc4));
            c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
            c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0xc8));
            c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x28));
            c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xcc));
            c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3c));
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[S6][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xb4));
            c.g[S7][0] = sx((c.g[S0][0] as u32).wrapping_add(0x64));
            c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xbc));
            c.g[FP][0] = sx((c.g[S0][0] as u32).wrapping_add(0x78));
            call_ptr(c, 0x0032e100, 0x00336ab0)?;
            c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8c));
            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x20e8), c.g[A2][0]);
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20d0));
            c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20d4));
            c.g[V1][0] = 0xffffffffffffffff;
            c.g[V0][0] = 0x3f;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
            c.g[V0][0] = 0x40;
            c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            c.sw((c.g[SP][0] as u32).wrapping_add(0xb4), c.g[A0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0xb8), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0xbc), c.g[A1][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0xc4), c.g[V1][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0xb0), c.g[SP][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0xc0), 0u64);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x20d0), 0u64);
            let t = c.g[T0][0] == 0u64;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x20d4), c.g[S2][0]);
            if t {
                break 'b_336b18;
            } else {
                c.cycles += 6;
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20d8));
                c.g[A2][0] = c.g[A1][0];
                c.g[A3][0] = c.g[S1][0];
                c.g[T1][0] = c.g[S0][0];
                call_ptr(c, 0x0033dbe8, 0x00336b18)?;
                break 'b_336b18;
            }
        }
        'b_336b3c: {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
            let t = c.g[V0][0] == 0u64;
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
            if t {
                break 'b_336b3c;
            } else {
                c.cycles += 6;
                c.g[T0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                c.g[T1][0] = c.g[S3][0];
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20d9));
                c.g[A2][0] = c.g[A1][0];
                c.g[A3][0] = c.g[S1][0];
                call_ptr(c, 0x0033dbe8, 0x00336b3c)?;
                break 'b_336b3c;
            }
        }
        'b_336b60: {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
            let t = c.g[V0][0] == 0u64;
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
            if t {
                break 'b_336b60;
            } else {
                c.cycles += 6;
                c.g[T0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                c.g[T1][0] = c.g[S4][0];
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20da));
                c.g[A2][0] = c.g[A1][0];
                c.g[A3][0] = c.g[S1][0];
                call_ptr(c, 0x0033dbe8, 0x00336b60)?;
                break 'b_336b60;
            }
        }
        'b_336b84: {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
            let t = c.g[V0][0] == 0u64;
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
            if t {
                break 'b_336b84;
            } else {
                c.cycles += 6;
                c.g[T0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x10));
                c.g[T1][0] = c.g[S5][0];
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20db));
                c.g[A2][0] = c.g[A1][0];
                c.g[A3][0] = c.g[S1][0];
                call_ptr(c, 0x0033dbe8, 0x00336b84)?;
                break 'b_336b84;
            }
        }
        'b_336ba8: {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x60));
            let t = c.g[V0][0] == 0u64;
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
            if t {
                break 'b_336ba8;
            } else {
                c.cycles += 6;
                c.g[T0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
                c.g[T1][0] = c.g[S6][0];
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20dc));
                c.g[A2][0] = c.g[A1][0];
                c.g[A3][0] = c.g[S1][0];
                call_ptr(c, 0x0033dbe8, 0x00336ba8)?;
                break 'b_336ba8;
            }
        }
        'b_336bcc: {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
            let t = c.g[V0][0] == 0u64;
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
            if t {
                break 'b_336bcc;
            } else {
                c.cycles += 6;
                c.g[T0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x10));
                c.g[T1][0] = c.g[S7][0];
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20dd));
                c.g[A2][0] = c.g[A1][0];
                c.g[A3][0] = c.g[S1][0];
                call_ptr(c, 0x0033dbe8, 0x00336bcc)?;
                break 'b_336bcc;
            }
        }
        'b_336bf0: {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x88));
            let t = c.g[V0][0] == 0u64;
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
            if t {
                break 'b_336bf0;
            } else {
                c.cycles += 6;
                c.g[T0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x10));
                c.g[T1][0] = c.g[FP][0];
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20de));
                c.g[A2][0] = c.g[A1][0];
                c.g[A3][0] = c.g[S1][0];
                call_ptr(c, 0x0033dbe8, 0x00336bf0)?;
                break 'b_336bf0;
            }
        }
        'b_336c18: {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x9c));
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20e8));
            if t {
                break 'b_336c18;
            } else {
                c.cycles += 7;
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                c.g[A3][0] = c.g[S1][0];
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20df));
                c.g[T0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                c.g[T1][0] = c.g[V0][0];
                c.g[A2][0] = c.g[A1][0];
                call_ptr(c, 0x0033dbe8, 0x00336c18)?;
                break 'b_336c18;
            }
        }
        'b_336d0c: {
            'b_336c2c: {
                c.cycles += 3;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20d0));
                if c.g[V1][0] != 0u64 {
                    c.lwc1(2, (c.g[S2][0] as u32).wrapping_add(0x8));
                    break 'b_336c2c;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_336d0c;
                    } else {
                        break 'b_336c2c;
                    }
                }
            }
            'b_336ce8: {
                c.cycles += 8;
                c.g[A3][0] = 0x1;
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xd8));
                c.g[T0][0] = 0u64;
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V1][0] as i64)) as u64;
                c.sub_s(0, 0, 2);
                c.abs_s(1, 0);
                let t = c.g[V0][0] == 0u64;
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x150));
                if t {
                    break 'b_336ce8;
                } else {
                    c.cycles += 1;
                    c.g[T1][0] = c.g[V1][0];
                    'l_336c50: loop {
                        'b_336cd8: {
                            'b_336c74: {
                                c.cycles += 6;
                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                                c.sub_s(0, 0, 2);
                                c.abs_s(0, 0);
                                c.c_lt_s(0, 1);
                                let t = !c.fcc();

                                if t {
                                    break 'b_336c74;
                                } else {
                                    c.cycles += 3;
                                    c.mov_s(1, 0);
                                    let t = true;
                                    c.g[T0][0] = c.g[A3][0];
                                    if t {
                                        break 'b_336cd8;
                                    } else {
                                        break 'b_336c74;
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.c_eq_s(0, 1);
                            let t = !c.fcc();
                            c.g[V0][0] = sx((c.g[T0][0] as u32) << 7);
                            if t {
                                break 'b_336cd8;
                            } else {
                                'b_336cd4: {
                                    'b_336cb4: {
                                        c.cycles += 4;
                                        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x50));
                                        c.g[V1][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[SP][0] as u32)));
                                        let t = c.g[A0][0] == 0u64;
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xd0));
                                        if t {
                                            break 'b_336cb4;
                                        } else {
                                            'b_336ca8: {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x50));
                                                if c.g[V0][0] != 0u64 {
                                                    c.g[V1][0] = c
                                                        .lw((c.g[A0][0] as u32).wrapping_add(0x78));
                                                    break 'b_336ca8;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[V0][0] = 0x1;
                                                    if t {
                                                        break 'b_336cd4;
                                                    } else {
                                                        break 'b_336ca8;
                                                    }
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                                            let t = true;
                                            c.g[V0][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                                            if t {
                                                break 'b_336cd4;
                                            } else {
                                                break 'b_336cb4;
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x50));
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = 0u64;
                                    if t {
                                        break 'b_336cd4;
                                    } else {
                                        c.cycles += 5;
                                        c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
                                        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x54));
                                        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x150));
                                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x150));
                                        c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
                                        break 'b_336cd4;
                                    }
                                }
                                c.cycles += 1;
                                if c.g[V0][0] != 0 {
                                    c.g[T0][0] = c.g[A3][0];
                                }
                                break 'b_336cd8;
                            }
                        }
                        c.cycles += 4;
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[T1][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x80));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00336c50);
                            }
                            continue 'l_336c50;
                        } else {
                            break 'b_336ce8;
                        }
                    }
                }
            }
            c.cycles += 9;
            c.g[V1][0] = sx((c.g[T0][0] as u32) << 7);
            c.g[V0][0] = 0x1;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[SP][0] as u32)));
            c.lq(4, (c.g[V1][0] as u32).wrapping_add(0xd0));
            c.lq(5, (c.g[V1][0] as u32).wrapping_add(0xe0));
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20e0));
            c.sq(4, (c.g[V1][0] as u32));
            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20e4));
            c.sq(5, (c.g[A2][0] as u32));
            break 'b_336d0c;
        }
        c.cycles += 12;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2180));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x2170));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x2160));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x2150));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x2140));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x2130));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x2120));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x2110));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x2100));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20f0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2190));
        c.pc = t;
        Ok(())
    })
}
// ==== BODIES ====

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule, Events};
    use crate::difftest::{check, fill, Rng, DATA};
    use crate::game::ai::computer::tests::{oc344_install, oc344_machine};

    /// A rider-state constructor fixture: the object at `DATA+0x1000` (the
    /// base's rider pointer already set as a base subobject has it), the
    /// vtable area filled, `cRider::cRider` cut short at its first callee
    /// (`Rider_InitData`, a gated fake that pops its frame, see computer.rs).
    fn oc425_ctor_fixture(
        c: &mut Cpu,
        seed: u32,
        layout: &RiderCtorLayout,
        entry: u32,
        in_chrg: bool,
    ) -> Rng {
        oc344_install();
        let mut r = oc344_machine(c, seed, entry as u64, 0);
        let me = DATA + 0x1000;
        let rider = DATA + 0x4000;
        fill(c, &mut r, me, 0x80);
        fill(c, &mut r, rider + 0x6a0, 0x40);
        fill(c, &mut r, 0x0045_8600, (0x0045_9c00 - 0x0045_8600) / 4);
        c.bus.write32(me + layout.rider_ptr, rider);
        c.g[A0][0] = sx(me);
        c.g[A1][0] = if in_chrg {
            (r.next_u32() as u64) << 32 | (r.next_u32() | 1) as u64
        } else {
            0
        };
        c.pc = entry;
        r
    }

    macro_rules! oc425_ctor_tests {
        ($matches:ident, $events:ident, $dgen:ident, $dhand:ident,
         $gen:path, $hand:path, $layout:expr, $entry:expr) => {
            driven!($dgen, $gen);
            driven!($dhand, $hand);

            /// Most-derived (the `cRider` base is built) and base subobject
            /// (both vtable copies), against the generated constructor.
            #[test]
            fn $matches() {
                check($gen, $hand, 120, &|c, seed| {
                    oc425_ctor_fixture(c, seed, &$layout, $entry, seed % 2 == 1);
                });
            }

            /// The copy loops' event checks, delivered (and unwinding).
            #[test]
            fn $events() {
                check_events($dgen, $dhand, 45, &|c, seed| {
                    let mut r = oc425_ctor_fixture(c, seed, &$layout, $entry, seed % 7 == 6);
                    schedule(c, &mut r, seed, $entry)
                });
            }
        };
    }

    oc425_ctor_tests!(
        rider_ground_state_ctor_matches,
        rider_ground_state_ctor_events_match,
        rgs_generated_driven,
        rgs_hand_driven,
        ee::s01::f_0013c650__generated,
        rider_ground_state_ctor,
        GROUND_STATE_LAYOUT,
        0x0013_c650
    );

    oc425_ctor_tests!(
        rider_wipeout_state_ctor_matches,
        rider_wipeout_state_ctor_events_match,
        rws_generated_driven,
        rws_hand_driven,
        ee::s01::f_00136980__generated,
        rider_wipeout_state_ctor,
        WIPEOUT_STATE_LAYOUT,
        0x0013_6980
    );

    oc425_ctor_tests!(
        rider_wipeout_sub_ctor_matches,
        rider_wipeout_sub_ctor_events_match,
        rwb_generated_driven,
        rwb_hand_driven,
        ee::s01::f_00142388__generated,
        rider_wipeout_sub_ctor,
        WIPEOUT_SUB_LAYOUT,
        0x0014_2388
    );

    oc425_ctor_tests!(
        rider_handplant_state_ctor_matches,
        rider_handplant_state_ctor_events_match,
        rhs_generated_driven,
        rhs_hand_driven,
        ee::s01::f_00141480__generated,
        rider_handplant_state_ctor,
        HANDPLANT_STATE_LAYOUT,
        0x0014_1480
    );
    use crate::difftest::fakes::{self, covered, machine};

    /// Gate of this module's callee fakes (in each test machine's own RAM).
    const OC425_CTRL: u32 = DATA + 0x0f_0400;
    const OC425_MAGIC: u32 = 0x4f43_3425;
    /// Forced results by fake call index: 16-byte records `{flags, v0, f0}`
    /// (flags: 1 = set v0 (sign-extended), 2 = set f0), 64 calls.
    const OC425_FORCED: u32 = OC425_CTRL + 0x100;

    /// Forced results by callee: 16-byte records `{target, flags, v0, f0}`
    /// (flags as above), 14 records, used when no result is forced for the
    /// call index.
    const OC425_TARGETS: u32 = OC425_CTRL + 0x20;

    /// The logging fake (it records every register the caller set up, so the
    /// RAM comparison checks the call), then the results the fixture forced
    /// for this call index or callee.
    fn oc425_fake_body(c: &mut Cpu, target: u32) {
        let n = c.bus.read32(fakes::LOG_COUNT);
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, target, ra);
        if c.pc != ra {
            return;
        }
        let mut e = 0;
        if n < 64 && c.bus.read32(OC425_FORCED + 0x10 * n) != 0 {
            e = OC425_FORCED + 0x10 * n;
        } else {
            for k in 0..14 {
                let at = OC425_TARGETS + 0x10 * k;
                if c.bus.read32(at) == target {
                    e = at + 4;
                    break;
                }
            }
        }
        if e == 0 {
            return;
        }
        let flags = c.bus.read32(e);
        if flags & 1 != 0 {
            c.g[V0][0] = sx(c.bus.read32(e + 4));
        }
        if flags & 2 != 0 {
            c.f[0] = c.bus.read32(e + 8);
        }
    }

    /// Force the result of fake call number `n` (v0 and/or f0).
    fn oc425_force(c: &mut Cpu, n: u32, v0: Option<u32>, f0: Option<u32>) {
        let e = OC425_FORCED + 0x10 * n;
        c.bus
            .write32(e, v0.is_some() as u32 | (f0.is_some() as u32) << 1);
        c.bus.write32(e + 4, v0.unwrap_or(0));
        c.bus.write32(e + 8, f0.unwrap_or(0));
    }

    /// Force the result of every fake call to `target` (v0 and/or f0).
    fn oc425_force_target(c: &mut Cpu, target: u32, v0: Option<u32>, f0: Option<u32>) {
        for k in 0..14 {
            let at = OC425_TARGETS + 0x10 * k;
            if c.bus.read32(at) == 0 {
                c.bus.write32(at, target);
                c.bus
                    .write32(at + 4, v0.is_some() as u32 | (f0.is_some() as u32) << 1);
                c.bus.write32(at + 8, v0.unwrap_or(0));
                c.bus.write32(at + 12, f0.unwrap_or(0));
                return;
            }
        }
        panic!("too many forced targets");
    }

    /// Scrambled machine with the fake hooks and this module's gate set.
    fn oc425_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC425_CTRL, OC425_MAGIC);
        r
    }

    /// `ScriptRider_ReadFields(rider, out)`: the fixture's list of script
    /// events (count at CTRL+0x10, pointer at CTRL+0x14) goes to `out+0x10`
    /// and `out+0x14`.
    fn oc425_pre_read_fields(c: &mut Cpu) {
        let out = c.g[A1][0] as u32;
        let count = c.bus.read32(OC425_CTRL + 0x10);
        let events = c.bus.read32(OC425_CTRL + 0x14);
        c.bus.write32(out + 0x10, count);
        c.bus.write32(out + 0x14, events);
    }

    /// `SpatialKey_FromAABB(out, box)`: the fixture's key signs (CTRL+4, +8,
    /// +0xc) go to `out+4`, `out+8` and `out+0xc`.
    fn oc425_pre_key(c: &mut Cpu) {
        let out = c.g[A0][0] as u32;
        for k in 1..4 {
            let w = c.bus.read32(OC425_CTRL + 4 * k);
            c.bus.write32(out + 4 * k, w);
        }
    }

    /// Gated fakes for callees that have override slots, and the function
    /// that puts them in the slots (once per process; with the gate clear
    /// they run what the slots held before).
    macro_rules! oc425_fakes {
        ($($fake:ident = $shard:ident :: $slot:ident / $gen:ident @ $addr:expr $(, pre $pre:ident)?;)*) => {
            $(
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC425_CTRL) != OC425_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), ee::$shard::$gen);
                    }
                    $( $pre(c); )?
                    oc425_fake_body(c, $addr);
                }
            )*
            fn oc425_install() {
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

    oc425_fakes! {
        fake_ea_mem_alloc_named = s10::OVERRIDE_00317D70 / f_00317d70__generated @ 0x0031_7d70;
        fake_memset = s15::OVERRIDE_00416210 / f_00416210__generated @ 0x0041_6210;
        fake_strcmp = s15::OVERRIDE_004165A8 / f_004165a8__generated @ 0x0041_65a8;
        fake_strcpy = s15::OVERRIDE_004166F4 / f_004166f4__generated @ 0x0041_66f4;
        fake_qsort = s15::OVERRIDE_00418EF8 / f_00418ef8__generated @ 0x0041_8ef8;
        fake_section_man_queue_section = s05::OVERRIDE_0022E298 / f_0022e298__generated @ 0x0022_e298;
        fake_world_cache_activate_section = s13::OVERRIDE_003A99D8 / f_003a99d8__generated @ 0x003a_99d8;
        fake_cmd_queue_is_empty = s07::OVERRIDE_00283DA0 / f_00283da0__generated @ 0x0028_3da0;
        fake_cmd_queue_at = s07::OVERRIDE_00283D70 / f_00283d70__generated @ 0x0028_3d70;
        fake_cmd_queue_pop = s07::OVERRIDE_00283DC0 / f_00283dc0__generated @ 0x0028_3dc0;
        fake_script_list_is_movie_entry = s07::OVERRIDE_00277DD8 / f_00277dd8__generated @ 0x0027_7dd8;
        fake_script_movie_get_id = s07::OVERRIDE_00283BB8 / f_00283bb8__generated @ 0x0028_3bb8;
        fake_script_movie_stop = s07::OVERRIDE_00283818 / f_00283818__generated @ 0x0028_3818;
        fake_audio_system_get_instance = s07::OVERRIDE_0028B180 / f_0028b180__generated @ 0x0028_b180;
        fake_anim_controller_play_anim = s10::OVERRIDE_003128E8 / f_003128e8__generated @ 0x0031_28e8;
        fake_script_rider_read_fields = s07::OVERRIDE_0027C9B0 / f_0027c9b0__generated @ 0x0027_c9b0, pre oc425_pre_read_fields;
        fake_audio_rider_speech2 = s08::OVERRIDE_002A1B58 / f_002a1b58__generated @ 0x002a_1b58;
        fake_audio_rider_speech01 = s08::OVERRIDE_002A1B88 / f_002a1b88__generated @ 0x002a_1b88;
        fake_ea_mem_copy = s14::OVERRIDE_003E6574 / f_003e6574__generated @ 0x003e_6574;
        fake_world_get_instances_man = s09::OVERRIDE_002D1CE0 / f_002d1ce0__generated @ 0x002d_1ce0;
        fake_serialize_read_u32 = s09::OVERRIDE_002D2510 / f_002d2510__generated @ 0x002d_2510;
        fake_str_hash = s10::OVERRIDE_00317670 / f_00317670__generated @ 0x0031_7670;
        fake_ui_state_find_state = s13::OVERRIDE_0039F9D8 / f_0039f9d8__generated @ 0x0039_f9d8;
        fake_script_play_instance = s07::OVERRIDE_00279720 / f_00279720__generated @ 0x0027_9720;
        fake_game_module_get_transition_man = s06::OVERRIDE_00230698 / f_00230698__generated @ 0x0023_0698;
        fake_transition_man_stop = s09::OVERRIDE_002E4578 / f_002e4578__generated @ 0x002e_4578;
        fake_script_get_process_in_seq = s07::OVERRIDE_00282BF0 / f_00282bf0__generated @ 0x0028_2bf0;
        fake_transition_man_start_captured = s09::OVERRIDE_002E4540 / f_002e4540__generated @ 0x002e_4540;
        fake_script_pause_process = s07::OVERRIDE_002826D8 / f_002826d8__generated @ 0x0028_26d8;
        fake_cutscene_start_in = s09::OVERRIDE_002EA6B8 / f_002ea6b8__generated @ 0x002e_a6b8;
        fake_cutscene_force_on = s09::OVERRIDE_002EA860 / f_002ea860__generated @ 0x002e_a860;
        fake_script_movie_update = s07::OVERRIDE_00283AA0 / f_00283aa0__generated @ 0x0028_3aa0;
        fake_transition_alloc_data = s09::OVERRIDE_002E4CE8 / f_002e4ce8__generated @ 0x002e_4ce8;
        fake_transition_wipe_ctor = s09::OVERRIDE_002EBBA8 / f_002ebba8__generated @ 0x002e_bba8;
        fake_transition_random2_ctor = s09::OVERRIDE_002EBE20 / f_002ebe20__generated @ 0x002e_be20;
        fake_transition_effect_ctor = s09::OVERRIDE_002E4D70 / f_002e4d70__generated @ 0x002e_4d70;
        fake_script_list_poll_loading = s07::OVERRIDE_00277778 / f_00277778__generated @ 0x0027_7778;
        fake_rider_get_character = s00::OVERRIDE_00123128 / f_00123128__generated @ 0x0012_3128;
        fake_script_obj_cam_set_binding = s07::OVERRIDE_0027D688 / f_0027d688__generated @ 0x0027_d688;
        fake_world_get_nash = s09::OVERRIDE_002D1BE0 / f_002d1be0__generated @ 0x002d_1be0;
        fake_world_get_instance = s09::OVERRIDE_002D1BD8 / f_002d1bd8__generated @ 0x002d_1bd8;
        fake_spatial_key_from_aabb = s10::OVERRIDE_00328F28 / f_00328f28__generated @ 0x0032_8f28, pre oc425_pre_key;
        fake_spatial_hash_remove = s10::OVERRIDE_003284B8 / f_003284b8__generated @ 0x0032_84b8;
        fake_spatial_hash_insert = s10::OVERRIDE_00328C20 / f_00328c20__generated @ 0x0032_8c20;
        fake_builtin_delete = s10::OVERRIDE_00317E50 / f_00317e50__generated @ 0x0031_7e50;
        fake_rail_list_unlink = s11::OVERRIDE_0035B6D0 / f_0035b6d0__generated @ 0x0035_b6d0;
        fake_coll_segment_ctor = s11::OVERRIDE_0032E100 / f_0032e100__generated @ 0x0032_e100;
        fake_cell_collide_column = s11::OVERRIDE_0033DBE8 / f_0033dbe8__generated @ 0x0033_dbe8, pre oc425_pre_collide;
        // ==== FAKES ====
    }
    /// `cGameModeMan::getGM`: the instance exists already (a seventh of the
    /// seeds), else every handler slot empty or taken, and allocations that
    /// fail (the retries of the 0x7c, 0xc and 0x8 byte handlers). Allocator
    /// and `memset` are fakes (some unwinding).
    #[test]
    fn game_mode_man_get_instance_matches() {
        oc425_install();
        covered(
            ee::s06::f_00237cf8__generated,
            game_mode_man_get_instance,
            300,
            &|c, seed| {
                let mut r = oc425_machine(c, seed, 0x0023_7cf8, 14);
                c.pc = 0x0023_7cf8;
                let gp = c.g[GP][0] as u32;
                let have = seed % 7 == 6;
                c.bus
                    .write32(gp - 0x484, if have { DATA + 0x5000 } else { 0 });
                c.bus.write32(gp - 0x480, r.next_u32());
                for slot in [0u32, 1, 2, 4, 5, 6, 7, 8, 9] {
                    let taken = r.next_u32() % 3 == 0;
                    c.bus.write32(
                        0x0053_6668 + 4 * slot,
                        if taken {
                            DATA + 0x6000 + 0x10 * slot
                        } else {
                            0
                        },
                    );
                }
                for k in 0..30 {
                    if r.next_u32() % 4 == 0 {
                        oc425_force(c, k, Some(0), None);
                    }
                }
            },
        );
    }
    /// `Rider_AddBoltNamesToLoadList`: lists of 0..4 entries (also `n <= 0`)
    /// whose names are null, empty or real, a table with 0..3 entries, and a
    /// `strcmp` that finds the name (forced 0) or not. `strcmp`, `strcpy` and
    /// `qsort` are fakes (some unwinding).
    #[test]
    fn rider_add_bolt_names_to_load_list_matches() {
        oc425_install();
        covered(
            ee::s00::f_0011bd60__generated,
            rider_add_bolt_names_to_load_list,
            300,
            &|c, seed| {
                let mut r = oc425_machine(c, seed, 0x0011_bd60, 8);
                c.pc = 0x0011_bd60;
                let rider = DATA + 0x1000;
                let table = DATA + 0x3000;
                let list = DATA + 0x2000;
                let n: i32 = if seed % 11 == 0 {
                    -(r.next_u32() as i32 & 1)
                } else {
                    1 + (r.next_u32() % 4) as i32
                };
                c.bus.write32(rider + RIDER_LOAD_LIST, table);
                c.bus.write32(table + ANIM_NAME_COUNT, r.next_u32() % 4);
                for i in 0..4u32 {
                    let entry = DATA + 0x2100 + 0x40 * i;
                    let name = DATA + 0x4000 + 0x20 * i;
                    let kind = r.next_u32() % 4;
                    c.bus.write32(list + 4 * i, entry);
                    c.bus
                        .write32(entry + 0x28, if kind == 0 { 0 } else { name });
                    c.bus
                        .write8(name, if kind == 1 { 0 } else { b'a' + i as u8 });
                }
                c.g[A0][0] = sx(rider);
                c.g[A1][0] = sx(list);
                c.g[A2][0] = sx(n as u32);
                for k in 0..24 {
                    if r.next_u32() % 3 == 0 {
                        oc425_force(c, k, Some(0), None);
                    }
                }
            },
        );
    }
    use crate::difftest::scramble;

    /// One of the `Bolts` character table getters: index below 8 (with junk
    /// in the high half of a1), counts zero, positive and negative.
    fn oc425_bolts_table_check(
        generated: GuestFn,
        hand: GuestFn,
        entry: u32,
        count: u32,
        index: u32,
        table: u32,
    ) {
        check(generated, hand, 300, &|c, seed| {
            let mut r = Rng(0xb017_0000 ^ seed as u64);
            scramble(c, &mut r);
            c.pc = entry;
            let me = DATA + 0x1000;
            let i = r.next_u32() % 8;
            c.g[A0][0] = sx(me);
            c.g[A1][0] = (r.next_u32() as u64) << 32 | i as u64;
            let n = match seed % 4 {
                0 => 0,
                1 => 1 + r.next_u32() % 5,
                2 => -(1 + (r.next_u32() % 5) as i32) as u32,
                _ => r.next_u32(),
            };
            c.bus.write32(me + 4 * i + count, n);
            c.bus.write32(me + 4 * i + index, r.next_u32() % 100);
            c.bus
                .write32(me + table, DATA + 0x2000 + 8 * (r.next_u32() % 16));
        });
    }

    /// `Bolts_GetCharTable3`.
    #[test]
    fn bolts_get_char_table3_matches() {
        oc425_bolts_table_check(
            ee::s01::f_0014dc10__generated,
            bolts_get_char_table3,
            0x0014_dc10,
            0x2fc,
            0x374,
            0xc,
        );
    }

    /// `Bolts_GetCharTable4`.
    #[test]
    fn bolts_get_char_table4_matches() {
        oc425_bolts_table_check(
            ee::s01::f_0014dc50__generated,
            bolts_get_char_table4,
            0x0014_dc50,
            0x3ec,
            0x464,
            0x10,
        );
    }

    /// `ScriptEngine_GetSeqProcessId`: the two pointer loads.
    #[test]
    fn script_engine_get_seq_process_id_matches() {
        check(
            ee::s07::f_00282be0__generated,
            script_engine_get_seq_process_id,
            200,
            &|c, seed| {
                let mut r = Rng(0x5e9_0000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0028_2be0;
                let seq = DATA + 0x1000;
                let process = DATA + 0x2000;
                c.g[A1][0] = sx(seq);
                c.bus.write32(seq + 0xc, process);
                c.bus.write32(process + 0x1c, r.next_u32());
            },
        );
    }
    /// `SectionMan_RequestActivate`: every load state (0..9, and out of range
    /// values), sections with a negative world id, the real jump table (and a
    /// bogus entry for the jump-out), with the world cache's activation and
    /// the queueing faked (some unwinding).
    #[test]
    fn section_man_request_activate_matches() {
        use crate::difftest::put_words;
        oc425_install();
        covered(
            ee::s05::f_0022d478__generated,
            section_man_request_activate,
            300,
            &|c, seed| {
                let mut r = oc425_machine(c, seed, 0x0022_d478, 2);
                c.pc = 0x0022_d478;
                let me = DATA + 0x1000;
                let i = r.next_u32() % 3;
                c.g[A0][0] = sx(me);
                c.g[A1][0] = sx(i);
                c.bus.write32(me + 0x1b0, DATA + 0x2000);
                c.bus.write32(DATA + 0x2010, DATA + 0x3000);
                c.bus.write32(DATA + 0x3000, r.next_u32());
                let section = 0x0044_2168 + 0x10 * i;
                c.bus.write32(
                    section + 4,
                    if seed % 5 == 4 {
                        u32::MAX
                    } else {
                        r.next_u32() % 40
                    },
                );
                let states = [0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, u32::MAX];
                let state = states[(seed % 12) as usize];
                c.bus.write32(section + 8, state);
                let table = [
                    0x22d4dc, 0x22d53c, 0x22d570, 0x22d50c, 0x22d570, 0x22d53c, 0x22d524, 0x22d4f4,
                    0x22d570,
                ];
                put_words(c, 0x0047_b3d0, &table);
                if seed % 17 == 16 && state < 9 {
                    // A table entry the function does not know: jump out.
                    c.bus.write32(0x0047_b3d0 + 4 * state, 0x0022_d5a0);
                }
            },
        );
    }
    /// `ScriptList_PopEntries`: queues of 1..3 commands (empty queue, movie and
    /// non-movie commands, negative ids), with and without the video engine's
    /// sound (negative and positive sound ids, the audio vtable call). Queue,
    /// movie and audio helpers are fakes (some unwinding); `KillProcess` and
    /// `GetMovieInfo` run for real on an empty request slot and a dummy BE.
    #[test]
    fn script_list_pop_entries_matches() {
        oc425_install();
        covered(
            ee::s07::f_00277598__generated,
            script_list_pop_entries,
            300,
            &|c, seed| {
                let mut r = oc425_machine(c, seed, 0x0027_7598, 10);
                crate::game::ai::ai::tests::belibrary(c, &mut r);
                c.pc = 0x0027_7598;
                let me = DATA + 0x10000;
                let engine = DATA + 0x11000;
                let cmd = DATA + 0x12000;
                let audio = DATA + 0x13000;
                c.bus.write32(me, engine);
                c.bus.write32(engine + 0x504, (seed % 3 != 0) as u32);
                // The front command: id at +0xc, key (first word) below 16.
                c.bus.write32(cmd, r.next_u32() % 16);
                c.bus.write32(
                    cmd + 0xc,
                    if seed % 6 == 5 {
                        u32::MAX
                    } else {
                        r.next_u32() % 8
                    },
                );
                for w in 1..7 {
                    if w != 3 {
                        c.bus.write32(cmd + 4 * w, r.next_u32());
                    }
                }
                // The movie table entries (both option variants): sound ids.
                crate::difftest::fill(c, &mut r, 0x0048_23dc, 0x100);
                // The audio system: a vtable with a (fake) method at +0x24.
                let vt = crate::difftest::fakes::vtable(c, &mut r, &[0x20]);
                c.bus.write32(audio + 0x5558, vt);
                oc425_force_target(c, 0x0028_3da0, Some((seed % 5 == 0) as u32), None);
                oc425_force_target(c, 0x0028_3d70, Some(cmd), None);
                oc425_force_target(c, 0x0027_7dd8, Some((seed % 2) as u32), None);
                oc425_force_target(c, 0x0028_3bb8, Some((seed % 4 < 2) as u32), None);
                oc425_force_target(c, 0x0028_b180, Some(audio), None);
                c.g[A0][0] = sx(me);
                c.g[A1][0] = sx(1 + seed % 3);
            },
        );
    }
    /// `RiderRailslideSub_PlayEnterAnim`: rail styles 4, 3 and others, with and
    /// without `alt`; `AnimController_PlayAnim` is a fake (some unwinding).
    #[test]
    fn rider_railslide_sub_play_enter_anim_matches() {
        oc425_install();
        covered(
            ee::s01::f_001326c8__generated,
            rider_railslide_sub_play_enter_anim,
            200,
            &|c, seed| {
                let mut r = oc425_machine(c, seed, 0x0013_26c8, 1);
                c.pc = 0x0013_26c8;
                let me = DATA + 0x1000;
                let rider = DATA + 0x2000;
                c.bus.write32(me + 8, rider);
                c.bus
                    .write32(rider + 0x328, [4, 3, 0, 5, 1, 4][(seed % 6) as usize]);
                c.bus.write32(rider + 0x784, r.next_u32());
                c.g[A0][0] = sx(me);
                c.g[A1][0] = if (seed / 6) % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64 | 1
                };
            },
        );
    }
    /// `Rider_OnScriptSoundEvent`: flag 0x10 clear, no events, and lists of 1..4
    /// events with every kind of id (102 skipped, below 100, 100 and up,
    /// negative). `ScriptRider_ReadFields` (writing the fixture's list) and the
    /// audio calls are fakes (some unwinding); the subject-rider lookup runs
    /// for real with out-of-range slots.
    #[test]
    fn rider_on_script_sound_event_matches() {
        oc425_install();
        covered(
            ee::s00::f_00123e30__generated,
            rider_on_script_sound_event,
            300,
            &|c, seed| {
                let mut r = oc425_machine(c, seed, 0x0012_3e30, 8);
                c.pc = 0x0012_3e30;
                let gp = c.g[GP][0] as u32;
                let me = DATA + 0x1000;
                let id = DATA + 0x2000;
                let events = DATA + 0x3000;
                c.bus.write32(gp - 0x84c, DATA + 0x4000);
                c.bus.write32(id + 8, r.next_u32());
                let count: i32 = match seed % 7 {
                    0 => 0,
                    1 => -1,
                    k => (k % 4) as i32 + 1,
                };
                c.bus.write32(OC425_CTRL + 0x10, count as u32);
                c.bus.write32(OC425_CTRL + 0x14, events);
                let ids = [102, 0, 5, 99, 100, 101, 150, -3i32 as u32];
                for i in 0..4 {
                    let arg = DATA + 0x5000 + 4 * i;
                    c.bus
                        .write32(events + 8 * i, ids[(r.next_u32() % 8) as usize]);
                    c.bus.write32(events + 8 * i + 4, arg);
                    c.bus.write8(arg, 0x20 + (r.next_u32() % 0x60) as u8);
                }
                c.g[A0][0] = sx(me);
                c.g[A1][0] = sx(id);
                c.g[A3][0] = if seed % 5 == 0 {
                    r.next_u32() as u64 & !0x10
                } else {
                    r.next_u32() as u64 | 0x10
                };
            },
        );
    }
    /// `MultiSplineModifier_CloneInstances`: 0..4 models (and a negative
    /// count); the node instances hold random flags. `EA_MemCopy` and the
    /// instances manager are fakes (some unwinding); `CopyInstance` runs for
    /// real.
    #[test]
    fn multi_spline_modifier_clone_instances_matches() {
        oc425_install();
        covered(
            ee::s11::f_0035a458__generated,
            multi_spline_modifier_clone_instances,
            200,
            &|c, seed| {
                let mut r = oc425_machine(c, seed, 0x0035_a458, 8);
                c.pc = 0x0035_a458;
                let me = DATA + 0x1000;
                let nodes = DATA + 0x3000;
                crate::difftest::fill(c, &mut r, DATA + 0x4000, 0x400);
                c.bus
                    .write32(me + 4, [0, 1, 2, 3, 4, u32::MAX][(seed % 6) as usize]);
                c.bus.write32(me + 0x40, DATA + 0x2000);
                c.bus.write32(me + 0x44, nodes);
                for i in 0..5 {
                    c.bus.write32(nodes + 4 * i, DATA + 0x4000 + 0x100 * i);
                }
                c.g[A0][0] = sx(me);
            },
        );
    }
    /// `CameraTrigger_ReadBoxTriggerData`: the object's five accessor virtuals
    /// (fake methods), the vector readers (real, over faked `ReadU32`) and
    /// `ReadU32` itself are called in order (some unwinding).
    #[test]
    fn camera_trigger_read_box_trigger_data_matches() {
        oc425_install();
        covered(
            ee::s02::f_001730d0__generated,
            camera_trigger_read_box_trigger_data,
            120,
            &|c, seed| {
                let mut r = oc425_machine(c, seed, 0x0017_30d0, 14);
                c.pc = 0x0017_30d0;
                let reader = DATA + 0x1000;
                let obj = DATA + 0x2000;
                let vt = crate::difftest::fakes::vtable(c, &mut r, &[0x8, 0x28, 0x10, 0x18, 0x20]);
                c.bus.write32(obj + 0x24, vt);
                c.bus.write32(obj + 0x28, r.next_u32());
                c.g[A0][0] = sx(reader);
                c.g[A1][0] = sx(obj);
            },
        );
    }
    /// `ScriptList_Start`: loading, ready and other list states; script and
    /// movie commands; with and without the transition (`self+4`), the fade
    /// flag and the notify flag; every transition kind (0..12, with the
    /// `MakeTransitionObject` of the engine running for real over faked
    /// allocation and constructors) and in/hold/out times (zero, positive,
    /// negative). Everything else it calls is a fake (some unwinding).
    #[test]
    fn script_list_start_matches() {
        oc425_install();
        covered(ee::s07::f_00276388__generated, script_list_start, 300, &|c, seed| {
            let mut r = oc425_machine(c, seed, 0x0027_6388, 20);
            c.pc = 0x0027_6388;
            let gp = c.g[GP][0] as u32;
            let me = DATA + 0x1000;
            let engine = DATA + 0x2000;
            let item = DATA + 0x3000;
            let world = DATA + 0x4000;
            let game = DATA + 0x5000;
            let hud = DATA + 0x6000;
            let tman = DATA + 0x7000;
            let proc = DATA + 0x8000;
            let info = DATA + 0x9000;
            c.bus.write32(gp - 0x848, world);
            c.bus.write32(gp - 0x84c, engine);
            c.bus.write32(gp - 0x4804, 0x3c88_8889);
            c.bus.write32(gp - 0x4800, 0x3c88_8889);
            c.bus.write32(world + 0x84, game);
            c.bus.write32(game + 0x48, r.next_u32());
            c.bus.write32(game + 0x64, r.next_u32());
            c.bus.write32(me, engine);
            c.bus.write32(me + 4, (seed % 3 != 0) as u32);
            c.bus.write32(me + 8, r.next_u32());
            c.bus
                .write32(me + 0xa4, [0, 1, 2, 2, 2, 2, 2, 3][(seed % 8) as usize]);
            let vt = crate::difftest::fakes::vtable(c, &mut r, &[0x30]);
            c.bus.write32(engine + 0x2a8, vt);
            // The video engine: no player (returns 0) or already active (1).
            c.bus.write32(engine + 0x500, (seed % 4 < 2) as u32);
            c.bus.write32(engine + 0x504, r.next_u32() | 1);
            c.bus.write32(item + 8, r.next_u32());
            c.bus.write32(item + 0xc, r.next_u32());
            let hud_vt = crate::difftest::fakes::vtable(c, &mut r, &[0xc0]);
            c.bus.write32(hud + 8, hud_vt);
            c.bus.write32(tman + 0x44, (seed % 5 < 2) as u32);
            c.bus.write32(proc + 0x14, info);
            let times = [0u16, 6, 30, 0xfffc];
            c.bus.write8(info + 8, (r.next_u32() % 13) as u8);
            c.bus.write8(info + 9, (r.next_u32() % 3) as u8);
            c.bus
                .write16(info + 0xa, times[(r.next_u32() % 4) as usize]);
            c.bus
                .write16(info + 0xc, times[(r.next_u32() % 4) as usize]);
            c.bus
                .write16(info + 0xe, times[(r.next_u32() % 4) as usize]);
            oc425_force_target(
                c,
                0x0039_f9d8,
                Some(if seed % 3 == 0 { 0 } else { hud }),
                None,
            );
            oc425_force_target(c, 0x0028_3d70, Some(item), None);
            oc425_force_target(c, 0x0027_7dd8, Some((seed % 4 == 3) as u32), None);
            oc425_force_target(c, 0x0023_0698, Some(tman), None);
            oc425_force_target(c, 0x0028_2bf0, Some(proc), None);
            c.g[A0][0] = sx(me);
            c.g[A1][0] = if seed % 7 < 3 {
                0
            } else {
                r.next_u32() as u64 | 1
            };
            c.g[A2][0] = if seed % 5 < 2 {
                0
            } else {
                r.next_u32() as u64 | 1
            };
        });
    }
    /// `ScriptViewObj_BindCamSlot`: bound and unbound channels, free and busy
    /// or missing rigs, the unbind id 9, rigs already playing the animation,
    /// and animation tables where the id is found or not (the real
    /// `ScriptBinding_IsBound`; `SetBinding` is a fake, some unwinding).
    #[test]
    fn script_view_obj_bind_cam_slot_matches() {
        oc425_install();
        covered(
            ee::s07::f_00281ac8__generated,
            script_view_obj_bind_cam_slot,
            400,
            &|c, seed| {
                let mut r = oc425_machine(c, seed, 0x0028_1ac8, 3);
                c.pc = 0x0028_1ac8;
                let me = DATA + 0x1000;
                let ch = r.next_u32() % 2;
                let anim = [3u32, 4, 5, 9][(r.next_u32() % 4) as usize];
                c.bus.write32(
                    me + 0xdc + 4 * ch,
                    [u32::MAX, 0, 1][(r.next_u32() % 3) as usize],
                );
                for i in 0..2u32 {
                    let rig = DATA + 0x2000 + 0x100 * i;
                    c.bus.write32(
                        me + 0xd0 + 4 * i,
                        if r.next_u32() % 5 == 0 { 0 } else { rig },
                    );
                    c.bus.write32(rig + 8, r.next_u32() % 3);
                    let cur = DATA + 0x3000 + 0x100 * i;
                    c.bus
                        .write32(rig + 0x14, if r.next_u32() % 3 == 0 { 0 } else { cur });
                    c.bus.write32(cur + 0xc, 3 + r.next_u32() % 3);
                }
                for j in 0..9u32 {
                    let e = me + 0x1c + 0x14 * j;
                    c.bus.write32(e + 8, r.next_u32() % 2);
                    c.bus.write32(e + 0xc, 3 + r.next_u32() % 3);
                }
                c.g[A0][0] = sx(me);
                c.g[A1][0] = sx(ch);
                c.g[A2][0] = sx(anim);
            },
        );
    }
    /// `RailModifier_v01` (the destructor): the node's spatial-hash slot empty
    /// or still used, splines with 0..3 segments (or none: no table entry),
    /// `flags & 1` set or clear, with the Nash, the instance table, the key
    /// builder, hash remove/insert, delete and list unlink as fakes (some
    /// unwinding); the key signs come from the fixture.
    #[test]
    fn rail_modifier_dtor_matches() {
        oc425_install();
        covered(ee::s11::f_0035bd70__generated, rail_modifier_dtor, 300, &|c, seed| {
            let mut r = oc425_machine(c, seed, 0x0035_bd70, 16);
            c.pc = 0x0035_bd70;
            let me = DATA + 0x1000;
            let nash = DATA + 0x3000;
            let world = DATA + 0xe800;
            let p1 = DATA + 0xe000;
            let table = DATA + 0xb000;
            let ent = DATA + 0xc000;
            let arr = DATA + 0xd000;
            let spline = DATA + 0xa000;
            c.bus.write32(OC425_CTRL + 4, r.next_u32());
            c.bus.write32(OC425_CTRL + 8, r.next_u32());
            c.bus.write32(OC425_CTRL + 12, r.next_u32());
            // The node's hash slots: pointers to eight-word blocks.
            for k in 0..8u32 {
                let block = DATA + 0x8000 + 0x100 * k;
                c.bus.write32(nash + 0x10 + 0x14 * k, block);
                for w in 0..8u32 {
                    let used = r.next_u32() % 4 == 0;
                    c.bus
                        .write32(block + 4 * w, if used { r.next_u32() } else { 0 });
                }
            }
            c.bus.write32(nash + 0xa0, r.next_u32());
            c.bus.write32(me + 0x44, DATA + 0x2000);
            c.bus.write32(me + 0x30, 0x201);
            c.bus.write32(world, p1);
            c.bus.write32(p1 + 8, table);
            c.bus
                .write32(table + 4, if seed % 6 == 5 { 0 } else { ent });
            c.bus.write32(ent + 0x44, arr);
            c.bus.write32(
                arr + 8,
                if seed % 7 == 6 {
                    0
                } else {
                    (spline >> 2) << 8 | 1
                },
            );
            c.bus.write32(spline + 0x20, r.next_u32() % 4);
            c.bus.write32(spline + 0x24, DATA + 0x10000);
            for k in 0..3u32 {
                let seg = DATA + 0x10000 + 0x100 * k;
                crate::difftest::fill(c, &mut r, seg + 0x6c, 6);
                c.bus.write32(seg + 0x64, DATA + 0x10000 + 0x100 * (k + 1));
            }
            oc425_force_target(c, 0x002d_1be0, Some(nash), None);
            oc425_force_target(c, 0x002d_1bd8, Some(world), None);
            c.g[A0][0] = sx(me);
            c.g[A1][0] = if seed % 2 == 0 {
                r.next_u32() as u64 & !1
            } else {
                r.next_u32() as u64 | 1
            };
        });
    }
    /// `Nash_ProbeVertical`'s hit collector: `Cell_CollideColumnRecursive(...,
    /// query, ...)` appends 0..1 hit records (point and normal vectors, the two
    /// object pointers at +0x50/+0x54) to the query's hit list and count,
    /// with z values taken from a small set so that ties happen.
    fn oc425_pre_collide(c: &mut Cpu) {
        let n = c.bus.read32(fakes::LOG_COUNT);
        let h = fakes::mix((c.bus.read32(OC425_CTRL + 0x10) as u64) << 8 ^ n as u64);
        if h & 3 == 0 {
            return;
        }
        let query = c.g[A1][0] as u32;
        let hits = c.bus.read32(query + 4);
        let count_ptr = c.bus.read32(query + 0xc);
        let count = c.bus.read32(count_ptr);
        if count >= 8 {
            return;
        }
        let rec = hits + 0x80 * count;
        let z = [-1.0f32, 0.0, 0.0, 1.0, 2.0][((h >> 8) % 5) as usize];
        let base = f32::from_bits(c.bus.read32(OC425_CTRL + 0x14));
        let obj_a = DATA + 0x2400 + 0x40 * ((h >> 16) % 4) as u32;
        let obj_b = DATA + 0x2500 + 0x40 * ((h >> 20) % 4) as u32;
        for (i, v) in [3.0f32, 4.0, base + z, 1.0, 0.0, 1.0, 0.0, 0.0]
            .into_iter()
            .enumerate()
        {
            c.bus.write32(rec + 4 * i as u32, v.to_bits());
        }
        c.bus
            .write32(rec + 0x50, if h >> 24 & 1 == 0 { 0 } else { obj_a });
        c.bus.write32(rec + 0x54, obj_b);
        c.bus.write32(count_ptr, count + 1);
    }

    /// `Nash_ProbeVertical`: cells with and without nodes, hit lists of 0..8
    /// hits (random z near the probe, ties broken by the object fields), the
    /// segment builder and the collisions as fakes (the collisions add hits;
    /// some unwind).
    #[test]
    fn nash_probe_vertical_matches() {
        oc425_install();
        covered(ee::s11::f_003369d8__generated, nash_probe_vertical, 300, &|c, seed| {
            let mut r = oc425_machine(c, seed, 0x0033_69d8, 9);
            c.pc = 0x0033_69d8;
            let gp = c.g[GP][0] as u32;
            let me = DATA + 0x1000;
            let pos = DATA + 0x2000;
            c.bus.write32(gp - 0x2d5c, 100000.0f32.to_bits());
            let z = r.f32(100.0);
            for (i, v) in [r.f32(100.0), r.f32(100.0), z].into_iter().enumerate() {
                c.bus.write32(pos + 4 * i as u32, v);
            }
            c.bus.write32(OC425_CTRL + 0x10, r.next_u32());
            c.bus.write32(OC425_CTRL + 0x14, z);
            crate::difftest::fill(c, &mut r, DATA + 0x2400, 0x100);
            for k in 0..8u32 {
                let node = if r.next_u32() % 3 == 0 {
                    0
                } else {
                    DATA + 0x2800
                };
                c.bus.write32(me + 0x10 + 0x14 * k, node);
            }
            c.bus.write32(me + 0xa4, r.next_u32());
            c.g[A0][0] = sx(me);
            c.g[A1][0] = sx(pos);
            c.g[A2][0] = sx(DATA + 0x3000);
            c.g[A3][0] = sx(DATA + 0x3040);
        });
    }
    // ==== TESTS ====
}
