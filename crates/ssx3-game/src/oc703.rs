//! Staging module oc703 (wave wave239): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc703`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0030_3d88,
        wscript_fn_empty_helper,
        "WScriptFn_EmptyHelper",
    ),
    (
        0x0030_3430,
        wscript_fn_get_self_handle,
        "WScriptFn_GetSelfHandle",
    ),
    (
        0x0030_3490,
        wscript_fn_get_current_task_id,
        "WScriptFn_GetCurrentTaskId",
    ),
    (
        0x0030_3380,
        wscript_fn_get_rider_speed,
        "WScriptFn_GetRiderSpeed",
    ),
    (
        0x0030_32c0,
        wscript_fn_get_rider_speed_scaled,
        "WScriptFn_GetRiderSpeedScaled",
    ),
];

/// `WScriptFn_EmptyHelper` (0x303d88): does nothing, returns to its caller.
pub fn wscript_fn_empty_helper(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `WScriptFn_GetSelfHandle` (0x303430): stores the current task's `+0x78`
/// word into the calling script object's first word (or `-1` when the
/// script manager has no current process), marks the object as type 1 and
/// returns through `LunoVMRegister_AsInt` (0x226610).
pub fn wscript_fn_get_self_handle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.g[V0][0] = Addr(c.g[GP][0] as u32).off(0xce8).lw(c);
        let s = sp(c);
        c.sq(17, s.wrapping_add(0x10));
        c.sq(16, s.wrapping_add(0x20));
        let ra = c.g[RA][0];
        c.sd(s, ra);
        c.g[V0][0] = Addr(c.g[V0][0] as u32).off(0x290).lw(c);
        c.g[S1][0] = c.g[A0][0];
        // Both paths charge two cycles (the `-1` path after its block).
        c.cycles += 2;
        c.g[S0][0] = if c.g[V0][0] == 0 {
            u64::MAX
        } else {
            Addr(c.g[V0][0] as u32).off(0x78).lw(c)
        };
        // Tail: mark the object as type 1, then its int view.
        c.cycles += 11;
        let obj = c.g[S1][0] as u32;
        c.g[V0][0] = 1;
        c.g[A0][0] = c.g[S1][0];
        Addr(obj).off(0x8).sw(c, 1);
        call_ptr(c, 0x0022_6610, 0x0030_3470)?;
        let s0 = c.g[S0][0];
        let int_ptr = c.g[V0][0] as u32;
        Addr(int_ptr).sw(c, s0);
        c.g[V0][0] = c.g[S1][0];
        let s = sp(c);
        c.lq(16, s.wrapping_add(0x20));
        c.lq(17, s.wrapping_add(0x10));
        c.g[RA][0] = c.ld(s);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        ret(c);
        Ok(())
    });
}

/// `WScriptFn_GetCurrentTaskId` (0x303490): the current task id of the
/// calling script's process (`WScriptProcess_GetCurrentTaskId`, 0x3079d8),
/// or `-1` when the manager has no process. Stores it into the calling
/// object's first word, marks the object as type 3, and returns through
/// `LunoVMRegister_AsInt`'s table slot (0x226620).
pub fn wscript_fn_get_current_task_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.g[V0][0] = Addr(c.g[GP][0] as u32).off(0xce8).lw(c);
        let s = sp(c);
        c.sq(17, s.wrapping_add(0x10));
        c.sq(16, s.wrapping_add(0x20));
        c.g[S1][0] = c.g[A0][0];
        let ra = c.g[RA][0];
        c.sd(s, ra);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x2a4).lw(c);
        let mut s0 = u64::MAX;
        if c.g[A0][0] == 0 {
            c.cycles += 2;
        } else {
            c.cycles += 4;
            Callee {
                addr: 0x0030_79d8,
                f: ee::s10::f_003079d8,
            }
            .call(c, 0x0030_34bc)?;
            s0 = c.g[V0][0];
        }
        c.g[S0][0] = s0;
        // Tail: mark the object as type 3, then its int view (0x226620).
        c.cycles += 11;
        let obj = c.g[S1][0] as u32;
        c.g[V0][0] = 3;
        c.g[A0][0] = c.g[S1][0];
        Addr(obj).off(0x8).sw(c, 3);
        call_ptr(c, 0x0022_6620, 0x0030_34dc)?;
        let ptr = c.g[V0][0] as u32;
        Addr(ptr).sw(c, s0);
        c.g[V0][0] = c.g[S1][0];
        let s = sp(c);
        c.lq(16, s.wrapping_add(0x20));
        c.lq(17, s.wrapping_add(0x10));
        c.g[RA][0] = c.ld(s);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        ret(c);
        Ok(())
    });
}

/// The rider speed lookup both speed functions share (identical registers
/// in both, so one body): the world's rider table entry (`world+0x84`,
/// `+0xc` base, `+0x28` rider), the rider's `+0x6c0` block, whose fetch
/// function (`+0x14`, half-word offset `+0x10`) returns the vector address
/// in V0. `ret_to` is the return address of that fetch call.
fn rider_speed_fetch(c: &mut Cpu, ret_to: u32) -> Flow {
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    let s = sp(c);
    c.sq(16, s.wrapping_add(0x10));
    let ra = c.g[RA][0];
    c.sd(s, ra);
    c.g[S0][0] = c.g[A0][0];
    let s = sp(c);
    c.swc1(20, s.wrapping_add(0x20));
    c.g[A0][0] = Addr(c.g[GP][0] as u32).off(0xce8).lw(c);
    c.g[A1][0] = Addr(c.g[V1][0] as u32).off(0x84).lw(c);
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[V1][0] = Addr(c.g[A1][0] as u32).off(0xc).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x28).lw(c);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
    if c.g[A0][0] == 0 {
        c.g[V0][0] = 0;
    }
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6c0));
    if c.g[V0][0] == 0 {
        c.g[A0][0] = 0;
    }
    c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x6c0).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
    c.g[A1][0] = Addr(c.g[V1][0] as u32).off(0x10).lh(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x14).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    call_ptr(c, target, ret_to)?;
    Ok(())
}

/// `WScriptFn_GetRiderSpeed` (0x303380): the rider's velocity length as a
/// VU0 `sqrt` of its squared components, stored as a float into the
/// calling object's +0x8 word (type 2) and returned through
/// `LunoVMRegister_AsFloat` (0x226618).
pub fn wscript_fn_get_rider_speed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        rider_speed_fetch(c, 0x0030_33dc)?;
        c.lqc2(3, c.g[V0][0] as u32);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(3, 22);
        c.mtc1(3, 20);
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] = 0x2;
        let s0 = c.g[S0][0] as u32;
        Addr(s0).off(0x8).sw(c, 2);
        call_ptr(c, 0x0022_6618, 0x0030_3414)?;
        let out = c.g[V0][0] as u32;
        c.swc1(20, out);
        c.g[V0][0] = c.g[S0][0];
        let s = sp(c);
        c.g[RA][0] = c.ld(s);
        let s = sp(c);
        c.lwc1(20, s.wrapping_add(0x20));
        c.lq(16, s.wrapping_add(0x10));
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        ret(c);
        Ok(())
    });
}

/// `WScriptFn_GetRiderSpeedScaled` (0x3032c0): the rider's velocity length
/// (the same VU0 maths as `GetRiderSpeed`) times the game timer period,
/// stored as a float into the calling object's +0x8 word (type 2) and
/// returned through `LunoVMRegister_AsFloat` (0x226618).
pub fn wscript_fn_get_rider_speed_scaled(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 47;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        rider_speed_fetch(c, 0x0030_331c)?;
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        c.lqc2(3, c.g[V0][0] as u32);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(4, 22);
        c.mtc1(4, 20);
        c.g[V0][0] = 0x2;
        c.g[A0][0] = c.g[S0][0];
        let v1 = c.g[V1][0] as u32;
        c.lwc1(0, v1.wrapping_add(0x14));
        let s0 = c.g[S0][0] as u32;
        Addr(s0).off(0x8).sw(c, 2);
        c.mul_s(20, 20, 0);
        call_ptr(c, 0x0022_6618, 0x0030_3360)?;
        let s = sp(c);
        c.g[RA][0] = c.ld(s);
        let out = c.g[V0][0] as u32;
        c.swc1(20, out);
        c.g[V0][0] = c.g[S0][0];
        let s = sp(c);
        c.lwc1(20, s.wrapping_add(0x20));
        c.lq(16, s.wrapping_add(0x10));
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        ret(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, fill, Rng, DATA, GAME_GP};

    /// The script object every fixture passes as `this`.
    const OC703_OBJ: u32 = DATA + 0x4000;
    /// The script manager (`gp+0xce8` holds its address).
    const OC703_MGR: u32 = DATA + 0x1400;
    /// The current task object behind the manager's `+0x2a4` (WScriptProcess).
    const OC703_PROC: u32 = DATA + 0x2000;
    /// The `ProcessTask` table the process indexes.
    const OC703_TABLE: u32 = DATA + 0x2800;
    /// The world (`gp-0x848` holds its address) and its rider table.
    const OC703_WORLD: u32 = DATA + 0x1000;
    const OC703_RIDERS: u32 = DATA + 0x1100;
    const OC703_BASE: u32 = DATA + 0x1200;
    /// The rider entry block (`+0x6c0`) and the speed object behind it.
    const OC703_RIDER: u32 = DATA + 0x1600;
    const OC703_SPEED: u32 = DATA + 0x1800;

    /// The fixture of the two script-object functions: the manager with (or
    /// without) a current process.
    fn oc703_self_setup(c: &mut Cpu, seed: u32, entry: u32) {
        let mut r = fakes::machine(c, seed, 0x0303_0430, 0);
        c.pc = entry;
        c.g[A0][0] = sx(OC703_OBJ);
        c.bus.write32(GAME_GP + 0xce8, OC703_MGR);
        let cur = if seed % 4 == 0 { 0 } else { OC703_PROC };
        c.bus.write32(OC703_MGR + 0x290, cur);
        c.bus.write32(OC703_PROC + 0x78, r.next_u32());
        c.bus.write32(OC703_MGR + 0x2a4, cur);
        c.bus.write32(OC703_OBJ, r.next_u32());
    }

    /// The fixture for the two `GetRiderSpeed` functions: a rider entry
    /// whose fetch function is `LunoVMRegister_AsInt` (returns its argument),
    /// so the vector sits at the rider block plus a half-word offset.
    fn oc703_speed_setup(c: &mut Cpu, seed: u32, entry: u32) {
        let mut r = fakes::machine(c, seed, 0x0303_3380, 0);
        c.pc = entry;
        c.g[A0][0] = sx(OC703_OBJ);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC703_WORLD);
        c.bus.write32(OC703_WORLD + 0x84, OC703_RIDERS);
        c.bus.write32(OC703_WORLD + 0x14, r.f32(4.0));
        c.bus.write32(OC703_RIDERS + 0xc, OC703_BASE);
        c.bus.write32(GAME_GP + 0xce8, OC703_MGR);
        c.bus.write32(OC703_MGR, 0);
        let rider = if seed % 4 == 0 { 0 } else { OC703_RIDER };
        c.bus.write32(OC703_BASE + 0x28, rider);
        c.bus.write32(rider.wrapping_add(0x6c0), OC703_SPEED);
        c.bus.write32(OC703_SPEED + 0x10, (seed % 8) * 0x10);
        c.bus.write32(OC703_SPEED + 0x14, 0x0022_6610);
        if rider != 0 {
            fill(c, &mut r, rider + 0x6c0, 0x40);
        }
    }

    #[test]
    fn wscript_fn_empty_helper_matches() {
        check(
            ee::s10::f_00303d88__generated,
            wscript_fn_empty_helper,
            4,
            &|c, seed| {
                let _ = fakes::machine(c, seed, 0x0303_3d88, 0);
                c.pc = 0x0030_3d88;
            },
        );
    }

    #[test]
    fn wscript_fn_get_self_handle_matches() {
        check(
            ee::s10::f_00303430__generated,
            wscript_fn_get_self_handle,
            200,
            &|c, seed| {
                oc703_self_setup(c, seed, 0x0030_3430);
            },
        );
    }

    #[test]
    fn wscript_fn_get_current_task_id_matches() {
        check(
            ee::s10::f_00303490__generated,
            wscript_fn_get_current_task_id,
            200,
            &|c, seed| {
                let mut r = Rng(seed as u64 + 1);
                oc703_self_setup(c, seed, 0x0030_3490);
                // The process: its count (+4) and current index (+8, in or out
                // of range), with a task table behind it (+0x14).
                c.bus
                    .write32(OC703_PROC + 0x4, if seed % 3 == 0 { 1 } else { 4 });
                c.bus.write32(OC703_PROC + 0x8, (seed % 2) * 2);
                c.bus.write32(OC703_PROC + 0x14, OC703_TABLE);
                fill(c, &mut r, OC703_TABLE, 0x100);
            },
        );
    }

    #[test]
    fn wscript_fn_get_rider_speed_matches() {
        check(
            ee::s10::f_00303380__generated,
            wscript_fn_get_rider_speed,
            200,
            &|c, seed| {
                oc703_speed_setup(c, seed, 0x0030_3380);
            },
        );
    }

    #[test]
    fn wscript_fn_get_rider_speed_scaled_matches() {
        check(
            ee::s10::f_003032c0__generated,
            wscript_fn_get_rider_speed_scaled,
            200,
            &|c, seed| {
                oc703_speed_setup(c, seed, 0x0030_32c0);
            },
        );
    }
}
