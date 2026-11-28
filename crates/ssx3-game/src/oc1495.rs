//! Staging module oc1495 (wave wave275): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1495`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002f_41a8,
        csplash_manager_reset_for_camera,
        "cSplashManager_ResetForCamera",
    ),
    (
        0x0016_c4f0,
        camera_trigger_man_read_bound_obj_action,
        "CameraTriggerMan_ReadBoundObjAction",
    ),
    (0x0031_d618, ccsi_cubic_spline_dtor, "cCSICubicSpline_dtor"),
    (0x003a_39dc, store_f1_tail_003a39e0, "f_003A39DC"),
    (
        0x0031_29e0,
        anim_controller_play_anim_direct,
        "AnimController_PlayAnimDirect",
    ),
    (
        0x002b_e258,
        wp_in_game_weather_set_defaults,
        "tWPInGameData_Weather_setDefaults",
    ),
    (0x003d_b9d8, loc_string_table_get_info, "LocStringTable_GetInfo"),
    (0x003d_2068, path_copy_event, "PATHI_copyevent"),
];

/// `cSplashManager_ResetForCamera(this, camera)` (0x2f41a8): for every view
/// whose player camera is `camera` (the `camera` argument in `a0`), resets that
/// view's splash manager: the manager is looked up through the view table at
/// `+0x84` of the world global (`gp-0x848`), and the object it returns gets a
/// virtual call through the function pointer at `+0x74` of its `+0xc` record.
pub fn csplash_manager_reset_for_camera(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
        let has_views = c.g[A0][0] != 0;
        c.g[S0][0] = 0;
        if has_views {
            c.cycles += 1;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            'views: loop {
                c.cycles += 7;
                c.g[A0][0] = sx((c.g[S0][0] as u32) << 2);
                c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                let other_camera = c.g[S1][0] != c.g[V1][0];
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if !other_camera {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[A1][0];
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0023_06a8, 0x002f_4200)?;
                    let no_splash = c.g[V0][0] == 0;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if !no_splash {
                        c.cycles += 8;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                        call_ptr(c, 0x0023_06a8, 0x002f_4214)?;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x70));
                        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x74));
                        let target = c.g[A1][0] as u32;
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, target, 0x002f_4228)?;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                }
                c.cycles += 7;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if !more {
                    break 'views;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_41d8);
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `CameraTriggerMan::ReadBoundObjAction` (0x16c4f0): returns 5 when
/// `CameraTrigger_ReadBoundObj` gives nothing, else 0.
pub fn camera_trigger_man_read_bound_obj_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        Callee {
            addr: 0x0017_1690,
            f: ee::s02::f_00171690,
        }
        .call(c, 0x0016_c500)?;
        c.g[V1][0] = 0x5;
        frame.restore_ra(c, 0);
        if c.g[V0][0] != 0 {
            c.g[V1][0] = 0;
        }
        c.g[V0][0] = c.g[V1][0];
        frame.close(c);
        Ok(())
    });
}

/// `cCSICubicSpline` destructor (0x31d618): frees the spline's storage, and
/// when bit 0 of `flags` is set also deletes the object itself.
pub fn ccsi_cubic_spline_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        Callee {
            addr: 0x0031_d790,
            f: ee::s10::f_0031d790,
        }
        .call(c, 0x0031_d634)?;
        c.g[S0][0] = c.g[S0][0] & 0x1;
        let delete_object = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if delete_object {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            Callee {
                addr: 0x0031_7e50,
                f: ee::s10::f_00317e50,
            }
            .call(c, 0x0031_d648)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The 4-byte stub at 0x3a39dc: stores `f1` at `t0 + 0x64`, then falls into
/// the code at 0x3a39e0 (a tail jump).
pub fn store_f1_tail_003a39e0(c: &mut Cpu) {
    c.cycles += 1;
    c.swc1(1, (c.g[T0][0] as u32).wrapping_add(0x64));
    c.pc = 0x003a_39e0;
    ee::s13::f_003a39e0(c);
}

/// `AnimController_PlayAnimDirect(this, anim, id, t)` (0x3129e0): looks up the
/// anim entry for `anim` (a second lookup when `t` is negative, reading the
/// blend time from it), then starts the anim through `AnimController_PlayAnimImpl`
/// and stores its result at `this + 4 * index`.
pub fn anim_controller_play_anim_direct(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S4, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x60);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, S3, 0x20);
        c.mov_s(20, 12);
        frame.save(c, S0, 0x50);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x0031_15c8,
            f: ee::s10::f_003115c8,
        }
        .call(c, 0x0031_2a18)?;
        c.mtc1(0, 0);
        c.c_lt_s(20, 0);
        let negative = c.fcc();
        c.g[S3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        if negative {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            Callee {
                addr: 0x0031_15c8,
                f: ee::s10::f_003115c8,
            }
            .call(c, 0x0031_2a34)?;
            c.lwc1(20, (c.g[V0][0] as u32).wrapping_add(0x10));
        }
        c.cycles += 25;
        c.g[V0][0] = 0x490000;
        c.g[S0][0] = sx((c.g[S3][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x27f8));
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A1][0] = sx((c.g[S3][0] as u32) << 3);
        c.g[T1][0] = c.lw(c.g[V0][0] as u32);
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S4][0];
        c.mov_s(12, 20);
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[T0][0] = 0x1;
        Callee {
            addr: 0x0031_1f00,
            f: ee::s10::f_00311f00,
        }
        .call(c, 0x0031_2a70)?;
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
        frame.restore(c, S1, 0x40);
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    });
}

/// `tWPInGameData_Weather_setDefaults(this)` (0x2be258): sets the weather
/// defaults in the record in `a0`: the floats 1.0 / 6.0 / 10.0 and the value of
/// the global at `gp-0x4290` at their offsets, and clears the integer fields.
pub fn wp_in_game_weather_set_defaults(c: &mut Cpu) {
    c.cycles += 28;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 0);
    c.g[AT][0] = 0x40c0_0000;
    c.mtc1(1, 1);
    c.g[AT][0] = 0x4120_0000;
    c.mtc1(1, 2);
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4290));
    let this = c.g[A0][0] as u32;
    c.swc1(1, this.wrapping_add(0x10));
    c.swc1(2, this.wrapping_add(0x30));
    c.swc1(3, this.wrapping_add(0x80));
    c.swc1(0, this.wrapping_add(0x98));
    c.sw(this, 0u64);
    c.sw(this.wrapping_add(0x8), 0u64);
    c.sw(this.wrapping_add(0x18), 0u64);
    c.sw(this.wrapping_add(0x20), 0u64);
    c.sw(this.wrapping_add(0x28), 0u64);
    c.swc1(0, this.wrapping_add(0x38));
    c.swc1(0, this.wrapping_add(0x40));
    c.sw(this.wrapping_add(0x48), 0u64);
    c.sw(this.wrapping_add(0x50), 0u64);
    c.sw(this.wrapping_add(0x58), 0u64);
    c.swc1(0, this.wrapping_add(0x60));
    c.swc1(0, this.wrapping_add(0x68));
    c.swc1(0, this.wrapping_add(0x70));
    c.swc1(0, this.wrapping_add(0x78));
    c.swc1(0, this.wrapping_add(0x88));
    c.swc1(0, this.wrapping_add(0x90));
    ret(c);
}

/// `LocStringTable_GetInfo(table, kind, record)` (0x3db9d8): reads one
/// packed info word of a localised string record. Kind 1 and kind 0 read two
/// bytes each, kind 2 a halfword, kind 3 a flag bit, other kinds 0.
pub fn loc_string_table_get_info(c: &mut Cpu) {
    'tail: {
        'main: {
            c.cycles += 3;
            c.g[V0][0] = 0x1;
            let kind_one = c.g[A1][0] == c.g[V0][0];
            c.g[A3][0] = c.g[A0][0];
            if kind_one {
                c.cycles += 3;
                c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0xd));
                c.g[V1][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0xc));
                break 'tail;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < 2) as u64;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                if c.g[A1][0] == 0 {
                    c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0xf));
                    c.cycles += 1;
                    c.g[V1][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0xe));
                    break 'tail;
                }
                c.cycles += 2;
                c.g[V0][0] = 0;
                ret(c);
                return;
            }
            c.g[V0][0] = 0x2;
            c.cycles += 2;
            if c.g[A1][0] == c.g[V0][0] {
                c.g[V0][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0xe));
                break 'main;
            }
            c.cycles += 3;
            c.g[V0][0] = 0x3;
            if c.g[A1][0] == c.g[V0][0] {
                c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x8));
                c.cycles += 2;
                c.g[V0][0] = c.g[V0][0] & 0x1;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
            ret(c);
            return;
        }
        // The record's packed word: bytes at +0x3, +0x2, +0x1, +0x0 of the
        // entry at `a3 + 4 * v0 + 0x10`, then the next entry's +0xc.
        c.cycles += 26;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x3));
        c.g[A0][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x2));
        c.g[A1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x1));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 24);
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 16);
        c.g[A2][0] = c.lbu(c.g[V0][0] as u32);
        c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 8);
        c.g[V1][0] = c.g[V1][0] | c.g[A1][0];
        c.g[V1][0] = c.g[V1][0] | c.g[A2][0];
        c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
        c.g[A2][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x3));
        c.g[V1][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x2));
        c.g[A1][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x1));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 16);
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 8);
        c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
        c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
        ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    ret(c);
}

/// `PATHI_copyevent(event)` (0x3d2068): walks the 16 path-event slots of the
/// game record (at `0x44f420`), adds up the sizes of the ones that fit before
/// `limit`, and copies the event's record onto the end of them (calling
/// `0x41605c`), or reports it through `PATH_DebugPrintf` when the flags say so.
/// Returns the new end offset, or 0 when nothing was copied.
pub fn path_copy_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = c.g[A0][0];
        frame.save(c, S0, 0x10);
        c.g[V0][0] = 0x450000;
        frame.save_ra(c, 0);
        c.g[T0][0] = 0;
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0xbe0));
        c.g[V1][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0xe));
        c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x91c));
        c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0xdc));
        c.g[V1][0] = c.g[V1][0] & 0x3f;
        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x11c));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
        c.g[T1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
        loop {
            'scan: {
                c.cycles += 4;
                c.g[V1][0] = c.lw(c.g[A3][0] as u32);
                c.g[V0][0] = (c.g[V1][0] < c.g[S0][0]) as u64;
                let before_end = c.g[V0][0] != 0;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                if before_end {
                    break 'scan;
                }
                c.cycles += 3;
                c.g[V0][0] = (c.g[V1][0] < c.g[A2][0]) as u64;
                if c.g[V0][0] == 0 {
                    break 'scan;
                }
                c.cycles += 5;
                c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0xe));
                c.g[V0][0] = c.g[V0][0] & 0x3f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            }
            c.cycles += 3;
            c.g[V0][0] = (c.g[T0][0] < 0x10) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003d_20a0);
            }
        }
        'fill: {
            'copy: {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[T1][0] as u32));
                c.g[V0][0] = (c.g[V0][0] < c.g[A2][0]) as u64;
                let fits = c.g[V0][0] != 0;
                c.g[V1][0] = 0x450000;
                if !fits {
                    break 'copy;
                }
                c.cycles += 5;
                c.g[A2][0] = c.g[T1][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0041_605c, 0x003d_20f8)?;
                c.g[V0][0] = c.g[S0][0];
                break 'fill;
            }
            'report: {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0xbc4));
                c.g[V0][0] = c.g[V0][0] & 0x120;
                let quiet = c.g[V0][0] == 0;
                c.g[A0][0] = 0x490000;
                if quiet {
                    break 'report;
                }
                c.cycles += 4;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[S0][0] as u32));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5b48));
                c.g[A1][0] = c.g[T1][0];
                Callee {
                    addr: 0x003d_1658,
                    f: ee::s14::f_003d1658,
                }
                .call(c, 0x003d_2120)?;
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate and mailbox words of the fakes below (one gate word per test,
    /// each in the machine's own RAM, beyond the fakes' call log).
    const OC1495_MAGIC: u32 = 0x1495_0001;
    const OC1495_BOUND_CTRL: u32 = DATA + 0x20_0000;
    const OC1495_DTOR_CTRL: u32 = DATA + 0x20_1000;
    const OC1495_ANIM_CTRL: u32 = DATA + 0x20_2000;
    const OC1495_PATH_CTRL: u32 = DATA + 0x20_3000;

    static OC1495_PREV_BOUND_OBJ: AtomicUsize = AtomicUsize::new(0);
    static OC1495_PREV_FREE: AtomicUsize = AtomicUsize::new(0);
    static OC1495_PREV_DELETE: AtomicUsize = AtomicUsize::new(0);
    static OC1495_PREV_GET_ENTRY: AtomicUsize = AtomicUsize::new(0);
    static OC1495_PREV_PLAY_IMPL: AtomicUsize = AtomicUsize::new(0);
    static OC1495_PREV_COPY_CALLEE: AtomicUsize = AtomicUsize::new(0);
    static OC1495_PREV_DEBUG: AtomicUsize = AtomicUsize::new(0);

    /// `CameraTrigger_ReadBoundObj`: returns the mailbox word in `v0`.
    fn oc1495_fake_bound_obj(c: &mut Cpu) {
        if c.bus.read32(OC1495_BOUND_CTRL) != OC1495_MAGIC {
            return fakes::fall_back(c, &OC1495_PREV_BOUND_OBJ, ee::s02::f_00171690__generated);
        }
        c.cycles += 3;
        c.g[V0][0] = c.bus.read32(OC1495_BOUND_CTRL + 4) as u64;
        c.pc = c.g[RA][0] as u32;
    }

    /// `cCSICubicSpline_Free` and `__builtin_delete`: logged no-op calls.
    fn oc1495_fake_free(c: &mut Cpu) {
        if c.bus.read32(OC1495_DTOR_CTRL) != OC1495_MAGIC {
            return fakes::fall_back(c, &OC1495_PREV_FREE, ee::s10::f_0031d790__generated);
        }
        fakes::fake_call(c, 0x0031_d790);
    }

    fn oc1495_fake_delete(c: &mut Cpu) {
        if c.bus.read32(OC1495_DTOR_CTRL) != OC1495_MAGIC {
            return fakes::fall_back(c, &OC1495_PREV_DELETE, ee::s10::f_00317e50__generated);
        }
        fakes::fake_call(c, 0x0031_7e50);
    }

    /// `AnimTable_GetEntry` returns the entry in the mailbox word at +4;
    /// `AnimController_PlayAnimImpl` returns the word at +8.
    fn oc1495_fake_get_entry(c: &mut Cpu) {
        if c.bus.read32(OC1495_ANIM_CTRL) != OC1495_MAGIC {
            return fakes::fall_back(c, &OC1495_PREV_GET_ENTRY, ee::s10::f_003115c8__generated);
        }
        c.cycles += 4;
        c.g[V0][0] = c.bus.read32(OC1495_ANIM_CTRL + 4) as u64;
        c.pc = c.g[RA][0] as u32;
    }

    fn oc1495_fake_play_impl(c: &mut Cpu) {
        if c.bus.read32(OC1495_ANIM_CTRL) != OC1495_MAGIC {
            return fakes::fall_back(c, &OC1495_PREV_PLAY_IMPL, ee::s10::f_00311f00__generated);
        }
        c.cycles += 6;
        c.g[V0][0] = c.bus.read32(OC1495_ANIM_CTRL + 8) as u64;
        c.pc = c.g[RA][0] as u32;
    }

    /// The `call_indirect` target `0x41605c` and `PATH_DebugPrintf`: logged.
    fn oc1495_fake_copy_callee(c: &mut Cpu) {
        if c.bus.read32(OC1495_PATH_CTRL) != OC1495_MAGIC {
            return fakes::fall_back(c, &OC1495_PREV_COPY_CALLEE, ee::s15::f_0041605c__generated);
        }
        fakes::fake_call(c, 0x0041_605c);
    }

    fn oc1495_fake_debug(c: &mut Cpu) {
        if c.bus.read32(OC1495_PATH_CTRL) != OC1495_MAGIC {
            return fakes::fall_back(c, &OC1495_PREV_DEBUG, ee::s14::f_003d1658__generated);
        }
        fakes::fake_call(c, 0x003d_1658);
    }

    fn oc1495_install_bound_obj() {
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_00171690),
            oc1495_fake_bound_obj,
            &OC1495_PREV_BOUND_OBJ,
        );
    }

    fn oc1495_install_dtor() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0031D790),
            oc1495_fake_free,
            &OC1495_PREV_FREE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc1495_fake_delete,
            &OC1495_PREV_DELETE,
        );
    }

    fn oc1495_install_anim() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003115C8),
            oc1495_fake_get_entry,
            &OC1495_PREV_GET_ENTRY,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00311F00),
            oc1495_fake_play_impl,
            &OC1495_PREV_PLAY_IMPL,
        );
    }

    fn oc1495_install_path() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0041605C),
            oc1495_fake_copy_callee,
            &OC1495_PREV_COPY_CALLEE,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003D1658),
            oc1495_fake_debug,
            &OC1495_PREV_DEBUG,
        );
    }

    /// `cSplashManager_ResetForCamera`: zero to three views, each with or
    /// without a splash manager, the camera matching view 0..2 or none.
    #[test]
    fn csplash_manager_reset_for_camera_matches() {
        check(
            ee::s09::f_002f41a8__generated,
            csplash_manager_reset_for_camera,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002f_41a8, 2);
                let world = DATA + 0x1000;
                let views = DATA + 0x2000;
                let arr = DATA + 0x3000;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, views);
                c.bus.write32(views + 0x84, arr);
                c.bus.write32(arr + 0x10, seed % 4);
                for i in 0..3u32 {
                    c.bus.write32(arr + 4 * i + 4, DATA + 0x6000 + 0x40 * i);
                    let splash = if (seed / 4 + i) % 2 == 0 {
                        0
                    } else {
                        DATA + 0x7000 + 0x100 * i
                    };
                    c.bus.write32(views + 4 * i + 0x68, splash);
                    let owner = DATA + 0x8000 + 0x100 * i;
                    c.bus.write32(DATA + 0x7000 + 0x100 * i + 0xc, owner);
                    c.bus.write16(owner + 0x70, (r.next_u32() % 64) as u16);
                    c.bus.write32(owner + 0x74, 0x00F0_0000 + 0x10 * i);
                }
                let k = seed % 3;
                c.g[A0][0] = if seed % 5 == 0 {
                    r.next_u32() as u64
                } else {
                    (DATA + 0x6000 + 0x40 * k) as u64
                };
            },
        );
    }

    /// `CameraTriggerMan_ReadBoundObjAction`: the bound object result is 0 or
    /// not, both give the same return value.
    #[test]
    fn camera_trigger_man_read_bound_obj_action_matches() {
        check(
            ee::s02::f_0016c4f0__generated,
            camera_trigger_man_read_bound_obj_action,
            300,
            &|c, seed| {
                oc1495_install_bound_obj();
                let mut r = machine(c, seed, 0x0016_c4f0, 0);
                c.bus.write32(OC1495_BOUND_CTRL, OC1495_MAGIC);
                c.bus.write32(
                    OC1495_BOUND_CTRL + 4,
                    if seed % 2 == 0 { 0 } else { r.next_u32() },
                );
            },
        );
    }

    /// `cCSICubicSpline_dtor`: the flag bit 0 both ways, with and without
    /// the object delete.
    #[test]
    fn ccsi_cubic_spline_dtor_matches() {
        check(
            ee::s10::f_0031d618__generated,
            ccsi_cubic_spline_dtor,
            300,
            &|c, seed| {
                oc1495_install_dtor();
                let mut r = machine(c, seed, 0x0031_d618, 0);
                c.bus.write32(OC1495_DTOR_CTRL, OC1495_MAGIC);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.g[A1][0] = r.next_u32() as u64;
                if seed % 2 == 0 {
                    c.g[A1][0] &= !1u64;
                } else {
                    c.g[A1][0] |= 1;
                }
            },
        );
    }

    /// The 4-byte stub: the store of `f1` and the dispatcher's tail (the
    /// index test fails at once, so it returns to `ra`).
    #[test]
    fn store_f1_tail_003a39e0_matches() {
        check(
            ee::s13::f_003a39dc__generated,
            store_f1_tail_003a39e0,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003a_39dc, 0);
                c.g[T0][0] = (DATA + 0x100) as u64;
                c.f[1] = if seed % 2 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(100.0)
                };
                c.g[T2][0] = 5;
                c.g[A1][0] = 3;
            },
        );
    }

    /// `AnimController_PlayAnimDirect`: a non-negative and a negative `t`
    /// (the second lookup), with the anim entries of every index 0..3.
    #[test]
    fn anim_controller_play_anim_direct_matches() {
        check(
            ee::s10::f_003129e0__generated,
            anim_controller_play_anim_direct,
            300,
            &|c, seed| {
                oc1495_install_anim();
                let mut r = machine(c, seed, 0x0031_29e0, 0);
                let this = DATA + 0x100;
                let entry = DATA + 0x400;
                c.bus.write32(OC1495_ANIM_CTRL, OC1495_MAGIC);
                c.bus.write32(OC1495_ANIM_CTRL + 4, entry);
                c.bus.write32(OC1495_ANIM_CTRL + 8, r.next_u32());
                c.bus.write32(entry + 0xc, seed % 4);
                c.bus.write32(entry + 0x10, r.f32(4.0));
                c.bus.write32(this + 0x50, DATA + 0x800);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x900) as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.f[12] = if seed % 3 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(4.0)
                };
            },
        );
    }

    /// `tWPInGameData_Weather_setDefaults`: straight-line, one fixture.
    #[test]
    fn wp_in_game_weather_set_defaults_matches() {
        check(
            ee::s08::f_002be258__generated,
            wp_in_game_weather_set_defaults,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002b_e258, 0);
                c.bus.write32(GAME_GP.wrapping_sub(0x4290), r.next_u32());
                c.g[A0][0] = (DATA + 0x1000) as u64;
            },
        );
    }

    /// `LocStringTable_GetInfo`: every kind (0, 1, 2, 3, other, negative).
    #[test]
    fn loc_string_table_get_info_matches() {
        check(
            ee::s14::f_003db9d8__generated,
            loc_string_table_get_info,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003d_b9d8, 0);
                let record = DATA + 0x1000;
                for i in 0..8u32 {
                    c.bus.write32(record + 4 * i, r.next_u32());
                }
                c.g[A0][0] = record as u64;
                c.g[A1][0] = match seed % 7 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => u64::MAX,
                    _ => 5,
                };
            },
        );
    }

    /// `PATHI_copyevent`: the scan over entries before, inside and past the
    /// limit; the copy (`0x41605c`) with and without room; the report path
    /// with its flags set or clear.
    #[test]
    fn path_copy_event_matches() {
        check(
            ee::s14::f_003d2068__generated,
            path_copy_event,
            300,
            &|c, seed| {
                oc1495_install_path();
                let mut r = machine(c, seed, 0x003d_2068, 2);
                c.bus.write32(OC1495_PATH_CTRL, OC1495_MAGIC);
                let base = DATA + 0x1000;
                c.bus.write32(0x0044_F420, base);
                for k in 0..16u32 {
                    let p = base + ((r.next_u32() % 0xe00) & !3);
                    c.bus.write32(base + 0xdc + 4 * k, p);
                    c.bus.write16(p + 0xe, r.next_u32() as u16);
                }
                c.bus.write16(DATA + 0x10e, r.next_u32() as u16);
                c.bus
                    .write32(0x0044_F43C, if seed % 2 == 0 { 0 } else { r.next_u32() });
                c.g[A0][0] = (DATA + 0x100) as u64;
            },
        );
    }
}
