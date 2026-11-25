//! Staging module oc557 (wave wave232): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc557`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0030_96f0,
        wscript_man_unlink_process,
        "WScriptMan_UnlinkProcess",
    ),
    (
        0x0030_9d20,
        wscript_man_call_func_on_instance,
        "WScriptMan_CallFuncOnInstance",
    ),
    (
        0x0030_a660,
        wscript_man_has_obj_script5,
        "WScriptMan_HasObjScript5",
    ),
    (
        0x0030_a728,
        wscript_man_process_command,
        "WScriptMan_ProcessCommand",
    ),
    (
        0x0030_ac98,
        wscript_man_call_process_func,
        "WScriptMan_CallProcessFunc",
    ),
    (
        0x0030_ada8,
        wscript_man_call_task_func,
        "WScriptMan_CallTaskFunc",
    ),
    (
        0x0030_af08,
        wscript_man_get_event_arg,
        "WScriptMan_GetEventArg",
    ),
    (
        0x0030_b350,
        wscript_process_list_find_by_name,
        "WScriptProcessList_FindByName",
    ),
    (
        0x0030_b3f0,
        wscript_process_list_find_mission_by_name,
        "WScriptProcessList_FindMissionByName",
    ),
    (
        0x0030_b458,
        wscript_man_find_mission_by_id,
        "WScriptMan_FindMissionById",
    ),
    (
        0x0030_b4c0,
        wscript_man_set_pending_request_a,
        "WScriptMan_SetPendingRequestA",
    ),
    (
        0x0030_b4d8,
        wscript_man_set_pending_request_b,
        "WScriptMan_SetPendingRequestB",
    ),
    (
        0x0030_b4f0,
        wscript_man_set_pending_request_c,
        "WScriptMan_SetPendingRequestC",
    ),
    (
        0x0030_b508,
        wscript_man_set_hud_height_target,
        "WScriptMan_SetHudHeightTarget",
    ),
    (
        0x0030_b608,
        wscript_man_request_mission_abort,
        "WScriptMan_RequestMissionAbort",
    ),
    (
        0x0030_b758,
        wscript_man_request_mission_end,
        "WScriptMan_RequestMissionEnd",
    ),
    (
        0x0030_b898,
        wscript_man_is_process_active,
        "WScriptMan_IsProcessActive",
    ),
];

/// `void WScriptMan_UnlinkProcess(WScriptMan* self, WScriptProcess* node)`
/// (0x3096f0): take `node` out of the first of the active, idle and add lists
/// that holds it.
pub fn wscript_man_unlink_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x2b8));
        call_ptr(c, 0x0030_b2d0, 0x0030_9710)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 5;
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x2b4));
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0030_b2d0, 0x0030_9724)?;
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x20);
            } else {
                c.cycles += 4;
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x2bc));
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x0030_b2d0, 0x0030_9738)?;
                frame.restore(c, S0, 0x20);
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptMan_CallFuncOnInstance(WScriptMan* self, int id, u8* handle,
/// int value)` (0x309d20): run a world script function with `self->Inst`
/// (+0x290) set to `value`, through a fresh Luno register and table, then
/// restore the instance.
pub fn wscript_man_call_func_on_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x30);
        c.g[S4][0] = Addr(r32(c, S0).wrapping_add(0x290)).lw(c);
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x28c)).lw(c);
        let value = c.g[A2][0];
        Addr(r32(c, S0).wrapping_add(0x290)).sw(c, value);
        call_ptr(c, 0x003a_6d18, 0x0030_9d54)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = 0;
        call_ptr(c, 0x0022_6600, 0x0030_9d64)?;
        c.g[V0][0] = Addr(r32(c, S1)).lbu(c);
        c.g[S3][0] = sx(sp(c).wrapping_add(0x20));
        Addr(sp(c).wrapping_add(0x18)).sw(c, 0);
        c.g[A0][0] = c.g[S3][0];
        Addr(sp(c).wrapping_add(0x10)).sw(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
        c.g[S2][0] = Addr(r32(c, V0).wrapping_add(0x3cc)).lw(c);
        call_ptr(c, 0x0022_24a8, 0x0030_9d88)?;
        c.g[S1][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A0][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x0022_24b8, 0x0030_9da0)?;
        let inst_saved = c.g[S4][0];
        Addr(r32(c, S0).wrapping_add(0x290)).sw(c, inst_saved);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x0022_5b90, 0x0030_9db0)?;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `bool WScriptMan_HasObjScript5(WScriptMan* self, int id)` (0x30a660): true
/// when the world's object script lookup (kind 5) finds an entry.
pub fn wscript_man_has_obj_script5(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = 5;
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(r32(c, A0).wrapping_add(0x28c)).lw(c);
        call_ptr(c, 0x003a_6b78, 0x0030_a674)?;
        c.g[V0][0] = !c.g[V0][0];
        frame.restore_ra(c, 0);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    })
}

/// The lookup at the top of each `WScriptMan_ProcessCommand` case:
/// `obj = FindById(self + list, id)`, except that `id == -1` keeps the current
/// process (`A1`).
fn oc557_command_lookup(c: &mut Cpu, list: u32, ret_addr: u32) -> Flow {
    c.cycles += 4;
    c.g[V0][0] = u64::MAX;
    if c.g[V1][0] != c.g[V0][0] {
        c.cycles += 4;
        c.g[A1][0] = c.g[V1][0];
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(list));
        call_ptr(c, 0x0030_b320, ret_addr)?;
        c.g[A1][0] = c.g[V0][0];
    }
    Ok(())
}

/// `void WScriptMan_ProcessCommand(WScriptMan* self, u32 id, int kind)`
/// (0x30a728): kind 1 starts an idle process, 0 stops an active one, 2 ends
/// its task (`NextTask`), 18 terminates it. `id == -1` acts on the current
/// process.
pub fn wscript_man_process_command(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        c.g[V0][0] = 1;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let kind = c.g[A2][0];
        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(0x2a4)).lw(c);
        if kind == 1 {
            // Start: the idle list, then WScriptMan_StartProcess.
            oc557_command_lookup(c, 0x2b4, 0x0030_a798)?;
            c.cycles += 2;
            if c.g[A1][0] == 0 {
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0030_9750, 0x0030_a7ac)?;
                frame.restore(c, S0, 0x10);
            }
        } else {
            c.cycles += 3;
            if (kind as i64) < 2 {
                c.g[V0][0] = 1;
                c.cycles += 2;
                if kind == 0 {
                    // Stop: the active list, then WScriptMan_StopProcess.
                    oc557_command_lookup(c, 0x2b8, 0x0030_a7d0)?;
                    c.cycles += 2;
                    if c.g[A1][0] == 0 {
                        frame.restore(c, S0, 0x10);
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0030_9798, 0x0030_a7e4)?;
                        frame.restore(c, S0, 0x10);
                    }
                } else {
                    c.cycles += 2;
                    frame.restore(c, S0, 0x10);
                }
            } else {
                c.g[V0][0] = 2;
                c.cycles += 2;
                let two = kind == 2;
                c.g[V0][0] = 0x12;
                if two {
                    // End task: the active list, then WScriptProcess_NextTask.
                    oc557_command_lookup(c, 0x2b8, 0x0030_a808)?;
                    c.cycles += 2;
                    frame.restore(c, S0, 0x10);
                    if c.g[A1][0] != 0 {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[A1][0];
                        call_ptr(c, 0x0030_7e10, 0x0030_a81c)?;
                        frame.restore(c, S0, 0x10);
                    }
                } else {
                    c.cycles += 2;
                    if kind == 0x12 {
                        // Terminate: the active list, then WScriptProcess_Terminate.
                        oc557_command_lookup(c, 0x2b8, 0x0030_a840)?;
                        c.cycles += 2;
                        frame.restore(c, S0, 0x10);
                        if c.g[A1][0] != 0 {
                            c.cycles += 3;
                            c.g[A0][0] = c.g[A1][0];
                            call_ptr(c, 0x0030_7d78, 0x0030_a854)?;
                            frame.restore(c, S0, 0x10);
                        }
                    } else {
                        c.cycles += 2;
                        frame.restore(c, S0, 0x10);
                    }
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Shared body of `WScriptMan_CallProcessFunc` (0x30ac98) and
/// `WScriptMan_CallTaskFunc` (0x30ada8): run a Luno function with `obj` as
/// the current process (or task) at `self + cur_off`, on the owner's local
/// table; a missing table is created for the call and destroyed after it.
/// `task` reads the table through the task's process (+0x3c).
fn oc557_run_script(c: &mut Cpu, cur_off: u32, task: bool, prologue: u64, rets: [u32; 8]) -> Flow {
    let [r_setraw, r_pass, r_call, r_alloc, r_ctor, r_call2, r_dtor1, r_end] = rets;
    c.cycles += prologue;
    let frame = Frame::open(c, 0xa0);
    frame.save(c, S1, 0x80);
    frame.save(c, S2, 0x70);
    frame.save(c, S3, 0x60);
    frame.save(c, S0, 0x90);
    c.g[S3][0] = c.g[A0][0];
    frame.save(c, S4, 0x50);
    c.g[A0][0] = c.g[SP][0];
    frame.save_ra(c, 0x30);
    frame.save(c, S5, 0x40);
    c.g[S5][0] = Addr(r32(c, S3).wrapping_add(cur_off)).lw(c);
    let obj = c.g[A1][0];
    Addr(r32(c, S3).wrapping_add(cur_off)).sw(c, obj);
    c.g[A1][0] = c.g[A2][0];
    c.g[S4][0] = Addr(r32(c, S3).wrapping_add(0x290)).lw(c);
    c.g[A2][0] = 0;
    call_ptr(c, 0x0022_6600, r_setraw)?;
    c.g[S2][0] = sx(sp(c).wrapping_add(0x20));
    Addr(sp(c).wrapping_add(0x18)).sw(c, 0);
    Addr(sp(c).wrapping_add(0x10)).sw(c, 0);
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x0022_24a8, r_pass)?;
    c.g[V1][0] = Addr(r32(c, S3).wrapping_add(cur_off)).lw(c);
    if task {
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x3c)).lw(c);
        c.g[S1][0] = Addr(r32(c, V0).wrapping_add(0x1c)).lw(c);
    } else {
        c.g[S1][0] = Addr(r32(c, V1).wrapping_add(0x1c)).lw(c);
    }
    let has_table = c.g[S1][0] != 0;
    c.g[V0][0] = sx(sp(c).wrapping_add(0x10));
    if has_table {
        c.cycles += 8;
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[A3][0] = c.g[V0][0];
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, 0x0022_24b8, r_call)?;
        let inst = c.g[S4][0];
        Addr(r32(c, S3).wrapping_add(0x290)).sw(c, inst);
    } else {
        // No table: allocate one, run the function on it, destroy it.
        c.cycles += 18;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[A1][0] = sx(0x0049_0000u32.wrapping_sub(0x6728));
        c.g[A0][0] = 4;
        call_ptr(c, 0x0031_7d70, r_alloc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x8;
        call_ptr(c, 0x0022_4da0, r_ctor)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S0][0];
        call_ptr(c, 0x0022_24b8, r_call2)?;
        c.g[A0][0] = c.g[S1][0];
        if c.g[S1][0] != 0 {
            c.g[A1][0] = 3;
            call_ptr(c, 0x0022_4df0, r_dtor1)?;
        }
        c.cycles += 1;
        let inst = c.g[S4][0];
        Addr(r32(c, S3).wrapping_add(0x290)).sw(c, inst);
    }
    c.cycles += 13;
    c.g[A0][0] = c.g[S0][0];
    let saved = c.g[S5][0];
    Addr(r32(c, S3).wrapping_add(cur_off)).sw(c, saved);
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x0022_5b90, r_end)?;
    frame.restore(c, S0, 0x90);
    frame.restore(c, S1, 0x80);
    frame.restore(c, S2, 0x70);
    frame.restore(c, S3, 0x60);
    frame.restore(c, S4, 0x50);
    frame.restore(c, S5, 0x40);
    frame.restore_ra(c, 0x30);
    frame.close(c);
    Ok(())
}

/// `void WScriptMan_CallProcessFunc(WScriptMan* self, WScriptProcess* obj,
/// u8* data)` (0x30ac98): `oc557_run_script` with the current process.
pub fn wscript_man_call_process_func(c: &mut Cpu) {
    guest(c, |c| {
        oc557_run_script(
            c,
            0x2a4,
            false,
            25,
            [
                0x0030_acd8,
                0x0030_acec,
                0x0030_ad14,
                0x0030_ad34,
                0x0030_ad40,
                0x0030_ad5c,
                0x0030_ad6c,
                0x0030_ad80,
            ],
        )
    })
}

/// `void WScriptMan_CallTaskFunc(WScriptMan* self, WScriptTask* obj, u8* data)`
/// (0x30ada8): `oc557_run_script` with the current task.
pub fn wscript_man_call_task_func(c: &mut Cpu) {
    guest(c, |c| {
        oc557_run_script(
            c,
            0x2a8,
            true,
            26,
            [
                0x0030_ade8,
                0x0030_adfc,
                0x0030_ae28,
                0x0030_ae48,
                0x0030_ae54,
                0x0030_ae70,
                0x0030_ae80,
                0x0030_ae94,
            ],
        )
    })
}

/// One `WScriptMan_GetEventArg` case reached through the table: `V1 =
/// *(self + 0x29c)`, `V0 = lw(V1 + off)`, then `>> shift` (32-bit,
/// sign-extended) and `& mask`.
fn oc557_event_field(c: &mut Cpu, cycles: u64, off: u32, shift: u32, mask: u64) -> u64 {
    c.cycles += cycles;
    c.g[V1][0] = Addr(r32(c, A1).wrapping_add(0x29c)).lw(c);
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(off)).lw(c);
    if shift != 0 {
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> shift);
    }
    c.g[V0][0] & mask
}

/// `int WScriptMan_GetEventArg(WScriptMan* self, int which)` (0x30af08): the
/// field `which` (0..15) of the current trick's flag record, selected through
/// the game's table of bit-field extractors (one case per entry). An unknown
/// entry, `which >= 16` or no record gives 0.
pub fn wscript_man_get_event_arg(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        c.g[A1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, A1).wrapping_add(0x29c)).lw(c);
        let no_record = c.g[V0][0] == 0;
        c.g[V0][0] = (c.g[A2][0] < 0x10) as u64;
        if !no_record {
            c.cycles += 2;
            let in_range = c.g[V0][0] != 0;
            c.g[V0][0] = 0x0049_0000;
            if in_range {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x6630));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.g[A0][0] = Addr(r32(c, V1)).lw(c);
                let target = c.g[A0][0] as u32;
                let handled: Option<u64> = match target {
                    0x0030_af38 => Some(oc557_event_field(c, 4, 0, 0, 0x7)),
                    0x0030_af48 => Some(oc557_event_field(c, 5, 0, 3, 0xf)),
                    0x0030_af5c => Some(oc557_event_field(c, 5, 0, 7, 0x7)),
                    0x0030_af70 => Some(oc557_event_field(c, 5, 0, 10, 0x3)),
                    0x0030_af84 => Some(oc557_event_field(c, 5, 0, 12, 0xf)),
                    0x0030_af98 => {
                        c.cycles += 4;
                        c.g[V1][0] = Addr(r32(c, A1).wrapping_add(0x29c)).lw(c);
                        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x2)).lhu(c);
                        Some(c.g[V0][0] & 0x7)
                    }
                    0x0030_afa8 => Some(oc557_event_field(c, 5, 0, 19, 0x7)),
                    0x0030_afbc => Some(oc557_event_field(c, 5, 0, 22, 0x3f)),
                    0x0030_afd0 => Some(oc557_event_field(c, 4, 0, 28, u64::MAX)),
                    0x0030_afe0 => Some(oc557_event_field(c, 4, 0x4, 0, 0x7)),
                    0x0030_aff0 => Some(oc557_event_field(c, 5, 0x4, 3, 0x7f)),
                    0x0030_b004 => Some(oc557_event_field(c, 5, 0x4, 10, 0x1)),
                    0x0030_b018 => Some(oc557_event_field(c, 5, 0x4, 18, 0x3)),
                    0x0030_b02c => {
                        c.cycles += 4;
                        c.g[V1][0] = Addr(r32(c, A1).wrapping_add(0x29c)).lw(c);
                        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x7)).lbu(c);
                        Some(c.g[V0][0] & 0x7)
                    }
                    0x0030_b03c => Some(oc557_event_field(c, 5, 0x4, 20, 0xf)),
                    _ => None,
                };
                match handled {
                    Some(v) => {
                        c.g[V0][0] = v;
                        ret(c);
                        return Ok(());
                    }
                    None if target == 0x0030_b050 => {}
                    None => {
                        c.jump_out(target, 0x0030_af08, 0x0030_b058);
                        return Ok(());
                    }
                }
            }
        }
        c.cycles += 2;
        c.g[V0][0] = 0;
        ret(c);
        Ok(())
    })
}

/// `WScriptProcess* WScriptProcessList_FindByName(WScriptProcessList* self,
/// const char* name)` (0x30b350): find by the name's hash.
pub fn wscript_process_list_find_by_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.g[A1][0];
        call_ptr(c, 0x0031_7670, 0x0030_b368)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0030_b320, 0x0030_b374)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `WScriptProcess* WScriptProcessList_FindMissionByName(self, name)`
/// (0x30b3f0): as `WScriptProcessList_FindByName`, over the mission lookup.
pub fn wscript_process_list_find_mission_by_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.g[A1][0];
        call_ptr(c, 0x0031_7670, 0x0030_b408)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0030_b388, 0x0030_b414)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `WScriptProcess* WScriptMan_FindMissionById(WScriptMan* self, int id)`
/// (0x30b458): the mission `id` in the active list, else the idle list, else
/// the add list.
pub fn wscript_man_find_mission_by_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x2b8));
        call_ptr(c, 0x0030_b388, 0x0030_b478)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 5;
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x2b4));
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0030_b388, 0x0030_b48c)?;
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x20);
            } else {
                c.cycles += 4;
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x2bc));
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x0030_b388, 0x0030_b4a0)?;
                frame.restore(c, S0, 0x20);
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptMan_SetPendingRequestA(WScriptMan* self, int a, int b)`
/// (0x30b4c0): queue a request (flag at +4, values at +8 and +0xc).
pub fn wscript_man_set_pending_request_a(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = 1;
    let this = r32(c, A0);
    let b = c.g[A2][0];
    Addr(this.wrapping_add(0xc)).sw(c, b);
    Addr(this.wrapping_add(0x4)).sw(c, 1);
    let a = c.g[A1][0];
    Addr(this.wrapping_add(0x8)).sw(c, a);
    ret(c);
}

/// `void WScriptMan_SetPendingRequestB(WScriptMan* self, int a, int b)`
/// (0x30b4d8): as request A, with the fields at +0x10..+0x18.
pub fn wscript_man_set_pending_request_b(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = 1;
    let this = r32(c, A0);
    let b = c.g[A2][0];
    Addr(this.wrapping_add(0x18)).sw(c, b);
    Addr(this.wrapping_add(0x10)).sw(c, 1);
    let a = c.g[A1][0];
    Addr(this.wrapping_add(0x14)).sw(c, a);
    ret(c);
}

/// `void WScriptMan_SetPendingRequestC(WScriptMan* self, int a, int b)`
/// (0x30b4f0): as request A, with the fields at +0x1c..+0x24.
pub fn wscript_man_set_pending_request_c(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = 1;
    let this = r32(c, A0);
    let b = c.g[A2][0];
    Addr(this.wrapping_add(0x24)).sw(c, b);
    Addr(this.wrapping_add(0x1c)).sw(c, 1);
    let a = c.g[A1][0];
    Addr(this.wrapping_add(0x20)).sw(c, a);
    ret(c);
}

/// `void WScriptMan_SetHudHeightTarget(WScriptMan* self, int target, Hud* hud)`
/// (0x30b508): set the height target flag and value, and copy the hud's
/// word at +0x78.
pub fn wscript_man_set_hud_height_target(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 1;
    let this = r32(c, A0);
    let target = c.g[A1][0];
    Addr(this.wrapping_add(0x30)).sw(c, target);
    Addr(this.wrapping_add(0x28)).sw(c, 1);
    c.g[V0][0] = Addr(r32(c, A2).wrapping_add(0x78)).lw(c);
    c.sw(r32(c, A0).wrapping_add(0x2c), c.g[V0][0]);
    ret(c);
}

/// The playback-manager calls shared by the mission request functions: the
/// world's playback manager (`*(gp - 0x848) + 0x84`) and replay.
const OC557_GP_WORLD: u32 = 0x848;

/// `int WScriptMan_RequestMissionPlayback(WScriptMan* self)` (0x30b590), named
/// `previewLocalMission` in the reference: stop playback and replay, set up
/// the local mission's preview, and on success queue the playback command.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn wscript_man_request_mission_playback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = Addr(r32(c, GP).wrapping_sub(OC557_GP_WORLD)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x84)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x34)).lw(c);
        call_ptr(c, 0x0026_cdf8, 0x0030_b5b0)?;
        c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(OC557_GP_WORLD)).lw(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x84)).lw(c);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x28)).lw(c);
        call_ptr(c, 0x0026_f228, 0x0030_b5c0)?;
        c.g[V1][0] = Addr(r32(c, GP).wrapping_sub(OC557_GP_WORLD)).lw(c);
        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(0x2a0)).lw(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x84)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x34)).lw(c);
        call_ptr(c, 0x0026_caf8, 0x0030_b5d4)?;
        if c.g[V0][0] != 0 {
            c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0x2a0)).lw(c);
            c.cycles += 4;
            c.g[V0][0] = 0x5;
            c.sw(r32(c, S0).wrapping_add(0x2ac), c.g[V0][0]);
            c.sw(r32(c, S0).wrapping_add(0x2b0), c.g[V1][0]);
            c.g[V0][0] = 0x1;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptMan_RequestMissionAbort(WScriptMan* self)` (0x30b608): stop
/// the playback, clear its saved world state, and queue the abort command.
pub fn wscript_man_request_mission_abort(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = Addr(r32(c, GP).wrapping_sub(OC557_GP_WORLD)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x84)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x34)).lw(c);
        call_ptr(c, 0x0026_cdf8, 0x0030_b628)?;
        c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(OC557_GP_WORLD)).lw(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x84)).lw(c);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x34)).lw(c);
        call_ptr(c, 0x0026_cbb0, 0x0030_b638)?;
        c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0x2a0)).lw(c);
        c.g[V0][0] = 0x6;
        c.sw(r32(c, S0).wrapping_add(0x2ac), c.g[V0][0]);
        c.sw(r32(c, S0).wrapping_add(0x2b0), c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptMan_RequestMissionEnd(WScriptMan* self)` (0x30b758): stop and
/// clear the playback, reset and advance the replay, force the rider reset,
/// and queue the end command with the current mission id.
pub fn wscript_man_request_mission_end(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = Addr(r32(c, GP).wrapping_sub(OC557_GP_WORLD)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x84)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x34)).lw(c);
        call_ptr(c, 0x0026_cdf8, 0x0030_b778)?;
        c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(OC557_GP_WORLD)).lw(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x84)).lw(c);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x34)).lw(c);
        call_ptr(c, 0x0026_cbb0, 0x0030_b788)?;
        c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(OC557_GP_WORLD)).lw(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x84)).lw(c);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x28)).lw(c);
        call_ptr(c, 0x0026_f228, 0x0030_b798)?;
        c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(OC557_GP_WORLD)).lw(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x84)).lw(c);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x28)).lw(c);
        call_ptr(c, 0x0026_f7b8, 0x0030_b7a8)?;
        c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(OC557_GP_WORLD)).lw(c);
        c.g[V1][0] = Addr(r32(c, S0)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x84)).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x28)).lw(c);
        call_ptr(c, 0x0012_35f8, 0x0030_b7c8)?;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x2a0)).lw(c);
        c.g[V0][0] = u64::MAX;
        c.g[V1][0] = 0x4;
        c.sw(r32(c, S0).wrapping_add(0x2a0), c.g[V0][0]);
        c.sw(r32(c, S0).wrapping_add(0x2ac), c.g[V1][0]);
        let mission = c.g[A0][0];
        Addr(r32(c, S0).wrapping_add(0x2b0)).sw(c, mission);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool WScriptMan_IsProcessActive(WScriptMan* self, WScriptProcess* proc)`
/// (0x30b898): whether the process's id is in the active list.
pub fn wscript_man_is_process_active(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x2b8));
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(r32(c, A1).wrapping_add(0x20)).lw(c);
        call_ptr(c, 0x0030_b320, 0x0030_b8ac)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, fill, scramble, Rng, DATA, GAME_GP};

    /// A process list at `head` with one node per id: nodes at `base + 0x40 * i`,
    /// id at +0x20, next at +0x18. Returns the node addresses.
    pub(crate) fn oc557_chain(c: &mut Cpu, head: u32, base: u32, ids: &[u32]) -> Vec<u32> {
        let nodes: Vec<u32> = (0..ids.len() as u32).map(|i| base + 0x40 * i).collect();
        for (i, &node) in nodes.iter().enumerate() {
            c.bus.write32(node + 0x20, ids[i]);
            let next = nodes.get(i + 1).copied().unwrap_or(0);
            c.bus.write32(node + 0x18, next);
        }
        c.bus.write32(head, nodes.first().copied().unwrap_or(0));
        nodes
    }

    /// The world pointer at `gp - 0x848` with its playback block (+0x84),
    /// whose +0x34 is the playback manager and +0x28 the replay.
    pub(crate) fn oc557_world(c: &mut Cpu) {
        let world = DATA + 0x2000;
        let block = DATA + 0x2100;
        c.bus.write32(GAME_GP - OC557_GP_WORLD, world);
        c.bus.write32(world + 0x84, block);
        c.bus.write32(block + 0x34, DATA + 0x2200);
        c.bus.write32(block + 0x28, DATA + 0x2300);
    }

    #[test]
    fn wscript_man_unlink_process_matches() {
        check(
            ee::s10::f_003096f0__generated,
            wscript_man_unlink_process,
            240,
            &|c, seed| {
                let _ = machine(c, seed, 0x3096_f0, 0);
                c.pc = 0x0030_96f0;
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                let act = oc557_chain(c, this + 0x2b8, DATA + 0x1000, &[1, 2]);
                let idle = oc557_chain(c, this + 0x2b4, DATA + 0x1200, &[3]);
                let _add = oc557_chain(c, this + 0x2bc, DATA + 0x1400, &[4, 5]);
                let victim = match seed % 6 {
                    0 => act.first().copied(),
                    1 => act.get(1).copied(),
                    2 => idle.first().copied(),
                    3 => _add.get(1).copied(),
                    4 => None,
                    _ => act.first().copied(),
                };
                c.g[A1][0] = sx(victim.unwrap_or(DATA + 0x1800));
            },
        );
    }

    #[test]
    fn wscript_man_call_func_on_instance_matches() {
        check(
            ee::s10::f_00309d20__generated,
            wscript_man_call_func_on_instance,
            160,
            &|c, seed| {
                let _ = machine(c, seed, 0x309d20, 0);
                c.pc = 0x0030_9d20;
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                let handle = DATA + 0x900;
                c.bus.write8(handle, (seed % 4) as u8);
                c.g[A1][0] = sx(handle);
                c.g[A2][0] = sx(DATA + 0x700 + seed % 3);
                c.bus.write32(this + 0x28c, DATA + 0x400);
                c.bus.write32(this + 0x290, DATA + 0x500);
            },
        );
    }

    #[test]
    fn wscript_man_has_obj_script5_matches() {
        check(
            ee::s10::f_0030a660__generated,
            wscript_man_has_obj_script5,
            160,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30a660, 0);
                c.pc = 0x0030_a660;
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(seed);
                c.bus.write32(this + 0x28c, DATA + 0x400);
                fill(c, &mut r, DATA + 0x400, 0x40);
            },
        );
    }

    #[test]
    fn wscript_man_process_command_matches() {
        check(
            ee::s10::f_0030a728__generated,
            wscript_man_process_command,
            240,
            &|c, seed| {
                let _ = machine(c, seed, 0x30a728, 0);
                c.pc = 0x0030_a728;
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                let id: u32 = if seed % 3 == 0 {
                    0xffff_ffff
                } else {
                    1 + seed % 3
                };
                c.g[A1][0] = sx(id);
                let kinds: [u64; 8] = [0, 1, 2, 18, u64::MAX, 3, 5, 1];
                c.g[A2][0] = kinds[(seed % 8) as usize];
                oc557_chain(c, this + 0x2b4, DATA + 0x1200, &[2, 3]);
                oc557_chain(c, this + 0x2b8, DATA + 0x1000, &[1, 2]);
                let obj = if seed % 4 == 0 { 0 } else { DATA + 0x1800 };
                c.bus.write32(this + 0x2a4, obj);
            },
        );
    }

    /// Fixture for the two script-call functions: the current owner slot at
    /// `this + cur_off`, the object's table pointer and the saved instance.
    fn oc557_script_setup(c: &mut Cpu, seed: u32, salt: u64, cur_off: u32, task: bool) {
        let mut r = machine(c, seed, salt, 0);
        let this = DATA + 0x100;
        c.g[A0][0] = sx(this);
        let obj = DATA + 0x1800;
        c.g[A1][0] = sx(obj);
        c.g[A2][0] = sx(DATA + 0x1900);
        c.bus.write32(this + 0x290, r.next_u32());
        c.bus.write32(this + cur_off, r.next_u32());
        let table = if seed % 2 == 0 { 0 } else { DATA + 0x1a00 };
        if task {
            let process = DATA + 0x1c00;
            c.bus.write32(obj + 0x3c, process);
            c.bus.write32(process + 0x1c, table);
        } else {
            c.bus.write32(obj + 0x1c, table);
        }
        if seed % 2 == 0 {
            crate::game::bx::bxstring::tests::alloc_ready(c);
        }
    }

    #[test]
    fn wscript_man_call_process_func_matches() {
        check(
            ee::s10::f_0030ac98__generated,
            wscript_man_call_process_func,
            160,
            &|c, seed| {
                oc557_script_setup(c, seed, 0x30ac98, 0x2a4, false);
                c.pc = 0x0030_ac98;
            },
        );
    }

    #[test]
    fn wscript_man_call_task_func_matches() {
        check(
            ee::s10::f_0030ada8__generated,
            wscript_man_call_task_func,
            160,
            &|c, seed| {
                oc557_script_setup(c, seed, 0x30ada8, 0x2a8, true);
                c.pc = 0x0030_ada8;
            },
        );
    }

    #[test]
    fn wscript_man_get_event_arg_matches() {
        // The table of bit-field extractors at 0x490000 - 0x6630, one entry
        // per `which` (0..15), each naming one case of the port.
        const CASES: [u32; 16] = [
            0x0030_af38,
            0x0030_af48,
            0x0030_af5c,
            0x0030_af70,
            0x0030_af84,
            0x0030_af98,
            0x0030_afa8,
            0x0030_afbc,
            0x0030_afd0,
            0x0030_afe0,
            0x0030_aff0,
            0x0030_b004,
            0x0030_b018,
            0x0030_b02c,
            0x0030_b03c,
            0x0030_b050,
        ];
        check(
            ee::s10::f_0030af08__generated,
            wscript_man_get_event_arg,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30af08, 0);
                c.pc = 0x0030_af08;
                let table = 0x0049_0000u32.wrapping_sub(0x6630);
                for (i, &case) in CASES.iter().enumerate() {
                    c.bus.write32(table + 4 * i as u32, case);
                }
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A2][0] = (seed % 17) as u64;
                let record = if seed % 5 == 0 { 0 } else { DATA + 0x400 };
                c.bus.write32(this + 0x29c, record);
                fill(c, &mut r, DATA + 0x400, 0x10);
            },
        );
    }

    #[test]
    fn wscript_process_list_find_by_name_matches() {
        check(
            ee::s10::f_0030b350__generated,
            wscript_process_list_find_by_name,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30b350, 0);
                c.pc = 0x0030_b350;
                let this = DATA + 0x100;
                oc557_chain(c, this, DATA + 0x1000, &[7, 8]);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x2000);
                fill(c, &mut r, DATA + 0x2000, 4);
            },
        );
    }

    #[test]
    fn wscript_process_list_find_mission_by_name_matches() {
        check(
            ee::s10::f_0030b3f0__generated,
            wscript_process_list_find_mission_by_name,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30b3f0, 0);
                c.pc = 0x0030_b3f0;
                let this = DATA + 0x100;
                oc557_chain(c, this, DATA + 0x1000, &[7, 8]);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x2000);
                fill(c, &mut r, DATA + 0x2000, 4);
            },
        );
    }

    #[test]
    fn wscript_man_find_mission_by_id_matches() {
        check(
            ee::s10::f_0030b458__generated,
            wscript_man_find_mission_by_id,
            160,
            &|c, seed| {
                let _ = machine(c, seed, 0x30b458, 0);
                c.pc = 0x0030_b458;
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(7 + seed % 2);
                oc557_chain(c, this + 0x2b8, DATA + 0x1000, &[7]);
                oc557_chain(c, this + 0x2b4, DATA + 0x1200, &[8]);
                oc557_chain(c, this + 0x2bc, DATA + 0x1400, &[9]);
            },
        );
    }

    #[test]
    fn wscript_man_set_pending_request_a_matches() {
        check(
            ee::s10::f_0030b4c0__generated,
            wscript_man_set_pending_request_a,
            120,
            &|c, seed| {
                let _ = machine(c, seed, 0x30b4c0, 0);
                c.pc = 0x0030_b4c0;
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(seed);
                c.g[A2][0] = sx(seed.wrapping_mul(3));
            },
        );
    }

    #[test]
    fn wscript_man_set_pending_request_b_matches() {
        check(
            ee::s10::f_0030b4d8__generated,
            wscript_man_set_pending_request_b,
            120,
            &|c, seed| {
                let _ = machine(c, seed, 0x30b4d8, 0);
                c.pc = 0x0030_b4d8;
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(seed);
                c.g[A2][0] = sx(seed.wrapping_mul(5));
            },
        );
    }

    #[test]
    fn wscript_man_set_pending_request_c_matches() {
        check(
            ee::s10::f_0030b4f0__generated,
            wscript_man_set_pending_request_c,
            120,
            &|c, seed| {
                let _ = machine(c, seed, 0x30b4f0, 0);
                c.pc = 0x0030_b4f0;
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(seed);
                c.g[A2][0] = sx(seed.wrapping_mul(7));
            },
        );
    }

    #[test]
    fn wscript_man_set_hud_height_target_matches() {
        check(
            ee::s10::f_0030b508__generated,
            wscript_man_set_hud_height_target,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30b508, 0);
                c.pc = 0x0030_b508;
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(DATA + 0x200);
                fill(c, &mut r, DATA + 0x200, 0x20);
            },
        );
    }

    #[test]
    fn wscript_man_request_mission_abort_matches() {
        check(
            ee::s10::f_0030b608__generated,
            wscript_man_request_mission_abort,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30b608, 0);
                c.pc = 0x0030_b608;
                oc557_world(c);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x2a0, r.next_u32());
            },
        );
    }

    #[test]
    fn wscript_man_request_mission_end_matches() {
        check(
            ee::s10::f_0030b758__generated,
            wscript_man_request_mission_end,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30b758, 0);
                c.pc = 0x0030_b758;
                oc557_world(c);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x2a0, r.next_u32());
                c.bus.write32(this, seed % 3);
                c.bus.write32(DATA + 0x2400 + 0xc, DATA + 0x2500);
                c.bus.write32(DATA + 0x2500, r.next_u32());
            },
        );
    }

    #[test]
    fn wscript_man_is_process_active_matches() {
        check(
            ee::s10::f_0030b898__generated,
            wscript_man_is_process_active,
            160,
            &|c, seed| {
                let _ = machine(c, seed, 0x30b898, 0);
                c.pc = 0x0030_b898;
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                oc557_chain(c, this + 0x2b8, DATA + 0x1000, &[2, 5, 6]);
                let proc_ = DATA + 0x1800;
                c.bus
                    .write32(proc_ + 0x20, if seed % 2 == 0 { 5 } else { 9 });
                c.g[A1][0] = sx(proc_);
            },
        );
    }
}
