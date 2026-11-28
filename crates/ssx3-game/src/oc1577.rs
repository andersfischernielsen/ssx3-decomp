//! Staging module oc1577 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1577`.

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
    (0x0030_7308, wscript_mission_v18, "WScriptMission_v18"),
    (0x0030_74c0, wscript_mission_v19, "WScriptMission_v19"),
    (0x0030_7568, wscript_mission_v20, "WScriptMission_v20"),
    (0x0030_7c40, wscript_process_v04, "WScriptProcess_v04"),
    (0x0031_6a48, block_man_init, "BlockMan_Init"),
    (0x0031_6d28, app_dtor, "App_dtor"),
    (0x0031_7308, app_set_exec_callback, "App_SetExecCallback"),
];

/// `WScriptMission_v18` (0x307308; reference `cWScriptMission_missionPass`).
/// One pass over a mission object in `a0`: its process at `+0x10`, a child
/// count at `+0x20`, a callback at `+0x6c`. Runs the pass calls, then the
/// mission's setup and deactivation scripts.
pub fn wscript_mission_v18(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        'head: {
            c.g[S0][0] = 0;
            call_ptr(c, 0x0014_dc80, 0x0030_7328)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0xa;
            call_ptr(c, 0x0014_dd58, 0x0030_7334)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[A1][0] = Addr(c.g[S1][0] as u32).off(0x20).lw(c);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0015_3d78, 0x0030_7344)?;
            let t = c.g[V0][0] != 0;
            c.g[A2][0] = 1;
            if t {
                break 'head;
            }
            c.cycles += 15;
            c.g[A1][0] = Addr(c.g[S1][0] as u32).off(0x20).lw(c);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0015_4160, 0x0030_7358)?;
            c.g[A1][0] = Addr(c.g[S1][0] as u32).off(0x20).lw(c);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0015_3fa0, 0x0030_7364)?;
            c.g[S0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_dc80, 0x0030_736c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0xb;
            call_ptr(c, 0x0014_dd58, 0x0030_7378)?;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0015_11b0, 0x0030_7384)?;
            c.g[S0][0] = c.g[V0][0];
        }
        'tail: {
            'pass: {
                c.cycles += 26;
                call_ptr(c, 0x0014_dc80, 0x0030_7390)?;
                c.g[A1][0] = 0;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014_dd58, 0x0030_739c)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014_4c98, 0x0030_73a4)?;
                c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                c.g[A3][0] = Addr(c.g[S1][0] as u32).off(0x10).lw(c);
                c.g[A2][0] = c.g[S0][0];
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x84).lw(c);
                c.g[V0][0] = Addr(c.g[A3][0] as u32).lw(c);
                c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0xc).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x28).lw(c);
                call_ptr(c, 0x0010_f2d8, 0x0030_73d0)?;
                c.g[A1][0] = Addr(c.g[S1][0] as u32).off(0x20).lw(c);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0015_3fd8, 0x0030_73dc)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = u64::MAX;
                let t = c.g[S0][0] == c.g[V0][0];
                c.g[A2][0] = 0;
                if t {
                    break 'pass;
                }
                'child: {
                    c.cycles += 6;
                    c.g[A1][0] = Addr(c.g[S1][0] as u32).off(0x20).lw(c);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x0015_41d0, 0x0030_73fc)?;
                    c.g[A2][0] = Addr(c.g[S1][0] as u32).off(0x6c).lw(c);
                    let t = c.g[A2][0] == 0;
                    c.g[A0][0] = c.g[S2][0];
                    if t {
                        break 'child;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0x10).lw(c);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0030_ac98, 0x0030_7414)?;
                    c.g[A0][0] = c.g[S2][0];
                }
                c.cycles += 4;
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0015_3e28, 0x0030_7420)?;
                let t = c.g[V0][0] == 0;
                c.g[A1][0] = c.g[S0][0];
                if t {
                    break 'tail;
                }
                c.cycles += 13;
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = 0;
                call_ptr(c, 0x0015_4010, 0x0030_7434)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = 1;
                call_ptr(c, 0x0015_4080, 0x0030_7444)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = 1;
                call_ptr(c, 0x0015_41d0, 0x0030_7454)?;
                break 'tail;
            }
            c.cycles += 3;
            c.g[A2][0] = Addr(c.g[S1][0] as u32).off(0x6c).lw(c);
            let t = c.g[A2][0] == 0;
            if t {
                break 'tail;
            }
            c.cycles += 3;
            c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0x10).lw(c);
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0030_ac98, 0x0030_7474)?;
        }
        c.cycles += 19;
        c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0x10).lw(c);
        call_ptr(c, 0x0030_8db8, 0x0030_747c)?;
        c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0x10).lw(c);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0030_9b00, 0x0030_7488)?;
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x28).lw(c);
        call_ptr(c, 0x0026_f228, 0x0030_7498)?;
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x28).lw(c);
        call_ptr(c, 0x0026_f7b8, 0x0030_74a8)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `WScriptMission_v19` (0x3074c0): the short mission step. Asks the process
/// (`+0x10`, count at `+0x20`) for a value, runs its callback when that value
/// is zero, and terminates the process when its first word (state) is 2.
pub fn wscript_mission_v19(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0030_74d8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xa;
        call_ptr(c, 0x0014_dd58, 0x0030_74e4)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x20).lw(c);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0015_3e80, 0x0030_74f4)?;
        'pass: {
            if c.g[V0][0] != 0 {
                c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0x70).lw(c);
                break 'pass;
            }
            c.cycles += 5;
            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x20).lw(c);
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = 1;
            call_ptr(c, 0x0015_40f0, 0x0030_750c)?;
            c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0x70).lw(c);
        }
        'mid: {
            'mid_a: {
                c.cycles += 2;
                if c.g[A2][0] == 0 {
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x5c).lw(c);
                    break 'mid_a;
                }
                c.cycles += 5;
                c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x10).lw(c);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0030_ac98, 0x0030_7524)?;
                c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
                break 'mid;
            }
            c.cycles += 5;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0xa0).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xa4).lw(c);
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0030_753c)?;
            c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        }
        'end: {
            c.cycles += 3;
            c.g[V0][0] = 0x2;
            if c.g[V1][0] != c.g[V0][0] {
                frame.restore(c, S0, 0x20);
                break 'end;
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0030_7d78, 0x0030_7554)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `WScriptMission_v20` (0x307568): forwards the mission's process callback
/// (`+0x10`) to the process manager when the mission has one (`+0x74`).
pub fn wscript_mission_v20(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr(c.g[A1][0] as u32).off(0x74).lw(c);
        let t = c.g[A2][0] == 0;
        frame.restore_ra(c, 0);
        if !t {
            c.cycles += 3;
            c.g[A0][0] = Addr(c.g[A1][0] as u32).off(0x10).lw(c);
            call_ptr(c, 0x0030_ac98, 0x0030_7588)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `WScriptProcess_v04` (0x307c40): the process's activation step. Frees the
/// old state (`+0x1c`), allocates the new one, and either runs the process
/// callback (`+0x10`) or activates the process and calls the table entry of
/// the child at `+0x8`.
pub fn wscript_process_v04(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x1c).lw(c);
        'init: {
            if c.g[A0][0] == 0 {
                c.g[A1][0] = 0x490000;
                break 'init;
            }
            c.cycles += 3;
            c.g[A1][0] = 0x3;
            call_ptr(c, 0x0022_4df0, 0x0030_7c64)?;
            c.g[A1][0] = 0x490000;
        }
        'alloc: {
            c.cycles += 14;
            c.g[A2][0] = 0x2000_0000;
            c.g[A0][0] = 0x4;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6778));
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x0030_7c7c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x8;
            call_ptr(c, 0x0022_4da0, 0x0030_7c88)?;
            c.g[V1][0] = 0x1;
            c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0x28).lw(c);
            let state = c.g[V0][0];
            Addr(c.g[S0][0] as u32).off(0x1c).sw(c, state);
            let one = c.g[V1][0];
            Addr(c.g[S0][0] as u32).sw(c, one);
            let t = c.g[A2][0] == 0;
            Addr(c.g[S0][0] as u32).off(0x8).sw(c, 0);
            if t {
                break 'alloc;
            }
            c.cycles += 3;
            c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x10).lw(c);
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0030_ac98, 0x0030_7cac)?;
        }
        'run: {
            c.cycles += 8;
            c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x10).lw(c);
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0030_9a60, 0x0030_7cb8)?;
            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x8).lw(c);
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x10);
                break 'run;
            }
            c.cycles += 10;
            c.g[A0][0] = 0x44;
            c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x14).lw(c);
            c.mult(0, 4, 5, 4);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x40).lw(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x20).lh(c);
            c.g[A1][0] = Addr(c.g[V0][0] as u32).off(0x24).lw(c);
            let target = c.g[A1][0] as u32;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            call_ptr(c, target, 0x0030_7cf0)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `BlockMan_Init` (0x316a48): sets up a block manager from a list of
/// `(size word, size)` entries (`a2`, `a1` entries): aligns each size, sums
/// them into the manager's total at `+0x4`, allocates the storage, and fills
/// the per-block headers.
pub fn block_man_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S3, 0x40);
        frame.save(c, S5, 0x20);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S4, 0x30);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S6, 0x10);
        frame.save(c, S0, 0x70);
        c.g[S6][0] = c.g[A2][0];
        frame.save(c, S1, 0x60);
        c.g[A1][0] = sx((c.g[S5][0] as u32) << 4);
        frame.save(c, S2, 0x50);
        frame.save_ra(c, 0);
        let a3 = c.g[A3][0];
        Addr(c.g[S3][0] as u32).off(0x10).sw(c, a3);
        call_ptr(c, 0x0031_6a20, 0x0031_6a84)?;
        c.g[S4][0] = c.g[V0][0];
        let t = (c.g[S5][0] as i64) <= 0;
        let s4 = c.g[S4][0];
        Addr(c.g[S3][0] as u32).off(0x4).sw(c, s4);
        'init: {
            if t {
                break 'init;
            }
            c.cycles += 2;
            c.g[S0][0] = c.g[S6][0];
            c.g[S1][0] = c.g[S5][0];
            loop {
                c.cycles += 11;
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
                c.g[A0][0] = c.g[S3][0];
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                call_ptr(c, 0x0031_6a20, 0x0031_6aa8)?;
                c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[S3][0] as u32).off(0x4).lw(c);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                c.mult(0, 3, 3, 2);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                let t = c.g[S1][0] != 0;
                let total = c.g[A0][0];
                Addr(c.g[S3][0] as u32).off(0x4).sw(c, total);
                if !t {
                    break 'init;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0031_6a98);
                }
            }
        }
        'bb4: {
            c.cycles += 12;
            c.g[A0][0] = Addr(c.g[S3][0] as u32).off(0x4).lw(c);
            c.g[A1][0] = 0x490000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x27c0));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            c.g[S2][0] = 0;
            call_ptr(c, 0x0031_7e30, 0x0031_6ae0)?;
            let s5 = c.g[S5][0];
            Addr(c.g[S3][0] as u32).off(0xc).sw(c, s5);
            c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
            let v0 = c.g[V0][0];
            Addr(c.g[S3][0] as u32).sw(c, v0);
            let t = (c.g[S5][0] as i64) <= 0;
            Addr(c.g[S3][0] as u32).off(0x8).sw(c, v0);
            if t {
                break 'bb4;
            }
            c.cycles += 1;
            loop {
                'slot: {
                    c.cycles += 26;
                    c.g[S0][0] = sx((c.g[S2][0] as u32) << 3);
                    c.g[A0][0] = c.g[S3][0];
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S6][0] as u32));
                    c.g[A1][0] = Addr(c.g[S1][0] as u32).off(0x4).lw(c);
                    call_ptr(c, 0x0031_6a20, 0x0031_6b0c)?;
                    c.g[V1][0] = Addr(c.g[S3][0] as u32).off(0x8).lw(c);
                    c.g[A0][0] = sx((c.g[S2][0] as u32) << 4);
                    c.g[T0][0] = c.g[V0][0];
                    c.g[T2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    let t0 = c.g[T0][0];
                    Addr(c.g[V1][0] as u32).off(0x8).sw(c, t0);
                    c.mult(0, 9, 2, 8);
                    c.g[V0][0] = Addr(c.g[S3][0] as u32).off(0x8).lw(c);
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let s4 = c.g[S4][0];
                    Addr(c.g[V0][0] as u32).off(0xc).sw(c, s4);
                    c.g[V1][0] = Addr(c.g[S3][0] as u32).off(0x8).lw(c);
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    Addr(c.g[V1][0] as u32).sw(c, s4);
                    c.g[V0][0] = Addr(c.g[S3][0] as u32).off(0x8).lw(c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let t1 = c.g[T1][0];
                    Addr(c.g[A0][0] as u32).off(0x4).sw(c, t1);
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[A2][0] = 0;
                    if t {
                        break 'slot;
                    }
                    c.cycles += 4;
                    c.g[A3][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[T0][0];
                    c.g[A0][0] = c.g[S4][0];
                    c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    loop {
                        c.cycles += 7;
                        let fill = c.g[V1][0];
                        Addr(c.g[A0][0] as u32).sw(c, fill);
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[A3][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        if !t {
                            break 'slot;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0031_6b70);
                        }
                    }
                }
                c.cycles += 10;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S6][0] as u32));
                c.g[S2][0] = c.g[T2][0];
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[A0][0] = ((c.g[S2][0] as i64) < (c.g[S5][0] as i64)) as u64;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                c.mult(0, 2, 8, 3);
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[T1][0] as u32));
                let t = c.g[A0][0] != 0;
                Addr(c.g[V1][0] as u32).sw(c, 0);
                if !t {
                    break 'bb4;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0031_6af8);
                }
            }
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `App_dtor` (0x316d28): clears the app's exec-callback word and, when the
/// low bit of `a1` (the delete flag) is set, frees the object with the
/// runtime's delete.
pub fn app_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = 0x490000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2418));
        let v0 = c.g[V0][0];
        Addr(c.g[A0][0] as u32).off(0x5c).sw(c, v0);
        c.g[A1][0] = c.g[A1][0] & 0x1;
        let t = c.g[A1][0] == 0;
        Addr((c.g[GP][0] as u32).wrapping_add(0x2a74)).sw(c, 0);
        if !t {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e50, 0x0031_6d50)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `App_SetExecCallback` (0x317308): passes the app's callback word (`+0x8`)
/// to the PS2 exec manager.
pub fn app_set_exec_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(c.g[A0][0] as u32).off(0x8).lw(c);
        call_ptr(c, 0x0031_7478, 0x0031_7318)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine_rt};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;
    use crate::difftest::{check, elf_image, Rng, DATA};

    const OC1577_SEEDS: u32 = 150;
    /// Gate word of the `WScriptMan_CallProcessFunc` fake (0 = the real body).
    const OC1577_CTRL: u32 = DATA + 0x9000;
    const OC1577_MAGIC: u32 = 0x1577_0a01;
    static OC1577_PREV_CALL: AtomicUsize = AtomicUsize::new(0);

    /// `WScriptMan_CallProcessFunc` would run the Luno VM on a zeroed script
    /// (runaway in `LunoVM_Execute`): the fixtures call a logging fake instead.
    fn oc1577_fake_call_process_func(c: &mut Cpu) {
        if c.bus.read32(OC1577_CTRL) != OC1577_MAGIC {
            return fakes::fall_back(
                c,
                &OC1577_PREV_CALL,
                ee::s10::f_0030ac98__generated,
            );
        }
        fakes::fake_call(c, 0x0030_ac98);
    }

    /// Hook the fake (once per process) and open its gate in this machine.
    fn oc1577_gate(c: &mut Cpu) {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0030AC98),
            oc1577_fake_call_process_func,
            &OC1577_PREV_CALL,
        );
        c.bus.write32(OC1577_CTRL, OC1577_MAGIC);
    }
    const OC1577_MISSION: u32 = DATA + 0x1000;
    const OC1577_PROC: u32 = DATA + 0x2000;
    const OC1577_WORLD: u32 = DATA + 0x3000;
    const OC1577_WSTRUCT: u32 = DATA + 0x3100;
    const OC1577_TABLE: u32 = DATA + 0x3300;
    const OC1577_VT: u32 = DATA + 0x3400;
    const OC1577_PROCESS: u32 = DATA + 0x4000;
    const OC1577_TABLE_P: u32 = DATA + 0x4300;
    const OC1577_ELEM: u32 = DATA + 0x4400;
    const OC1577_BM: u32 = DATA + 0x5000;
    const OC1577_LIST: u32 = DATA + 0x5100;
    const OC1577_APP: u32 = DATA + 0x6000;
    const OC1577_EXEC: u32 = DATA + 0x7000;

    /// The mission fixture of the three WScriptMission ports: the mission, its
    /// process, the world lookup chain (gp-0x848 -> world -> +0x84 -> +0xc
    /// table), and a vtable whose `+0xa4` is the leaf at 0x1406c0 (returns 0).
    fn oc1577_mission(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        elf_image(c);
        let mut r = machine_rt(c, seed, salt, 0);
        oc1577_gate(c);
        let gp = c.g[GP][0] as u32;
        let m = OC1577_MISSION;
        c.g[A0][0] = sx(m);
        c.bus.write32(gp.wrapping_sub(0x848), OC1577_WORLD);
        c.bus.write32(OC1577_WORLD + 0x84, OC1577_WSTRUCT);
        c.bus.write32(OC1577_WSTRUCT + 0xc, OC1577_TABLE);
        c.bus.write32(OC1577_PROC, r.next_u32() % 4);
        c.bus.write32(m + 0x10, OC1577_PROC);
        c.bus.write32(m + 0x20, r.next_u32() % 3);
        c.bus.write32(m + 0x5c, OC1577_VT);
        c.bus.write32(m + 0x6c, (seed % 3 != 0) as u32);
        c.bus.write32(m + 0x70, r.next_u32() % 3);
        c.bus.write32(m + 0x74, (seed % 2) as u32);
        c.bus.write32(OC1577_VT + 0xa4, 0x0014_06c0);
        r
    }

    #[test]
    fn wscript_mission_v18_matches() {
        check(
            ee::s10::f_00307308__generated,
            wscript_mission_v18,
            OC1577_SEEDS,
            &|c, seed| {
                oc1577_mission(c, seed, 0x1577_0018);
            },
        );
    }

    #[test]
    fn wscript_mission_v19_matches() {
        check(
            ee::s10::f_003074c0__generated,
            wscript_mission_v19,
            OC1577_SEEDS,
            &|c, seed| {
                oc1577_mission(c, seed, 0x1577_0019);
            },
        );
    }

    #[test]
    fn wscript_mission_v20_matches() {
        check(
            ee::s10::f_00307568__generated,
            wscript_mission_v20,
            OC1577_SEEDS,
            &|c, seed| {
                oc1577_mission(c, seed, 0x1577_0020);
            },
        );
    }

    #[test]
    fn wscript_process_v04_matches() {
        check(
            ee::s10::f_00307c40__generated,
            wscript_process_v04,
            OC1577_SEEDS,
            &|c, seed| {
                elf_image(c);
                let mut r = machine_rt(c, seed, 0x1577_0004, 0);
                oc1577_gate(c);
                let p = OC1577_PROCESS;
                c.g[A0][0] = sx(p);
                c.bus
                    .write32(p + 0x1c, if seed % 2 == 0 { 0 } else { OC1577_ELEM });
                c.bus.write32(p + 0x28, r.next_u32() % 2);
                c.bus.write32(p + 0x10, OC1577_PROC);
                c.bus.write32(p + 0x4, 4);
                c.bus.write32(p + 0x8, if seed % 4 == 0 { 4 } else { r.next_u32() % 4 });
                c.bus.write32(p + 0x14, OC1577_TABLE_P);
                for k in 0..6u32 {
                    c.bus.write32(OC1577_TABLE_P + 0x44 * k + 0x40, OC1577_ELEM);
                }
                c.bus.write32(OC1577_ELEM + 0x20, 0);
                c.bus.write32(OC1577_ELEM + 0x24, 0x0014_06c0);
            },
        );
    }

    #[test]
    fn block_man_init_matches() {
        check(
            ee::s10::f_00316a48__generated,
            block_man_init,
            OC1577_SEEDS,
            &|c, seed| {
                elf_image(c);
                let mut r = machine_rt(c, seed, 0x1577_0005, 0);
                c.g[A0][0] = sx(OC1577_BM);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = sx(OC1577_LIST);
                c.g[A3][0] = sx(r.next_u32());
                for i in 0..4u32 {
                    c.bus.write32(OC1577_LIST + 8 * i, r.next_u32() % 16);
                    c.bus.write32(OC1577_LIST + 8 * i + 4, 1 + r.next_u32() % 200);
                }
            },
        );
    }

    #[test]
    fn app_dtor_matches() {
        check(
            ee::s10::f_00316d28__generated,
            app_dtor,
            OC1577_SEEDS,
            &|c, seed| {
                elf_image(c);
                machine_rt(c, seed, 0x1577_0006, 0);
                c.g[A0][0] = sx(OC1577_APP);
                c.g[A1][0] = (seed % 3) as u64;
            },
        );
    }

    #[test]
    fn app_set_exec_callback_matches() {
        check(
            ee::s10::f_00317308__generated,
            app_set_exec_callback,
            OC1577_SEEDS,
            &|c, seed| {
                elf_image(c);
                let mut r = machine_rt(c, seed, 0x1577_0007, 0);
                c.g[A0][0] = sx(OC1577_EXEC);
                c.bus.write32(OC1577_EXEC + 0x8, r.next_u32());
            },
        );
    }
}
