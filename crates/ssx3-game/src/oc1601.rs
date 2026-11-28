//! Staging module oc1601 (wave wave280): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1601`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002f_d250,
        wscript_fn_add_roller_modifier,
        "WScriptFn_AddRollerModifier",
    ),
    (0x0031_6d60, app_run_main_loop, "App_Run_MainLoop"),
    (
        0x0042_4aa0,
        libkernl_intr_helper_thread,
        "libkernl_IntrHelperThread",
    ),
];

/// `luno_t WScriptFn_AddRollerModifier(buf_t *out, int n, Ent *params)`
/// (0x002fd250): the script call `AddRollerModifier(...)`. Builds the 5-word
/// parameter block (defaults from a guarded static: id -1, 1.0, 0.7, 0.5, one
/// byte), overwrites the slots named by the `n` `{idx, val, ?, type}` entries
/// (converting an int to a float when the keyword type table says Float),
/// finds the target instance (`id == -1`: the script manager's own instance,
/// else the world cache: `section[id & 0xff]`, tag `[id >> 8]`) and, when its
/// active node is a move node (`isMoveNode()` vtable call), calls
/// `cMoveNode_addRoller(node, buf, IntersectInfo, CollideInfo)`. Returns the
/// nil constant, copied into `*out` (12 bytes).
pub fn wscript_fn_add_roller_modifier(c: &mut Cpu) {
    guest(c, |c| {
        let gp = c.g[GP][0] as u32;
        c.cycles += 12;
        let fp = sp(c).wrapping_sub(0x80);
        c.g[SP][0] = sx(fp);
        c.g[V0][0] = c.lw(gp.wrapping_add(0x2510));
        c.sq(18, fp.wrapping_add(0x50));
        c.sq(16, fp.wrapping_add(0x70));
        c.g[S2][0] = c.g[A2][0];
        c.sq(19, fp.wrapping_add(0x40));
        c.g[S0][0] = c.g[A1][0];
        c.sq(17, fp.wrapping_add(0x60));
        c.g[S3][0] = c.g[A0][0];
        c.sd(fp.wrapping_add(0x30), c.g[RA][0]);
        let initialised = c.g[V0][0] != 0;
        c.g[A2][0] = 0x500000;
        if !initialised {
            // guarded static-local init of the default block at 0x4fb628
            c.cycles += 17;
            c.g[V1][0] = sx(0x4fb628);
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A0][0] = c.lbu(0x4fb628 + 0x10);
            c.g[V0][0] |= 0xffff;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 2);
            c.g[A1][0] = 0x1;
            c.lwc1(0, gp.wrapping_sub(0x3388));
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 1);
            c.sw(0x4fb628, c.g[V0][0]);
            c.sb(0x4fb628 + 0x10, c.g[A0][0]);
            c.swc1(2, 0x4fb628 + 0x4);
            c.swc1(0, 0x4fb628 + 0x8);
            c.swc1(1, 0x4fb628 + 0xc);
            c.sw(gp.wrapping_add(0x2510), c.g[A1][0]);
            c.sb(fp, c.g[A0][0]);
        }

        // buf = defaults (id = -1), copied with EA_MemCopy(buf, 0x4fb628, 20)
        c.cycles += 10;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        c.g[S1][0] = sx(fp.wrapping_add(0x10));
        c.g[A1][0] = sx(0x4fb628);
        c.g[A2][0] = 0x14;
        c.sw(fp.wrapping_add(0x10), c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x003e_6574, 0x002f_d2e4)?;
        let none = (c.g[S0][0] as i64) <= 0;
        c.g[A2][0] = c.g[S2][0];
        if !none {
            // for each entry: buf[idx] = val (float-converted when the
            // keyword type of idx is Float (2) and the entry's type differs)
            c.cycles += 5;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx(0x440000u32.wrapping_add(0x60a0));
            c.g[V0][0] = 0x440000;
            c.g[A3][0] = 0x2;
            loop {
                c.cycles += 8;
                c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                let mut store_int = true;
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[A3][0] {
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                    } else {
                        c.cycles += 4;
                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, c.g[A0][0] as u32);
                        store_int = false;
                    }
                }
                if store_int {
                    c.cycles += 1;
                    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x002f_d300);
                    }
                    continue;
                }
                break;
            }
        }

        // the target instance: obj in V0
        c.cycles += 4;
        c.g[A2][0] = c.lw(c.g[S1][0] as u32);
        c.g[V0][0] = !c.g[A2][0];
        let by_id = c.g[V0][0] != 0;
        c.g[A1][0] = c.lw(gp.wrapping_add(0x16c8));
        if by_id {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = c.lw(c.g[A1][0] as u32);
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            let no_section = c.g[V1][0] == 0;
            c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
            let mut found = false;
            if !no_section {
                c.cycles += 7;
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                found = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            }
            if !found {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw(gp.wrapping_add(0xce8));
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
        }

        // active node: if it is a move node, add the roller
        c.cycles += 3;
        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        let no_node = c.g[S0][0] == 0;
        c.g[V1][0] = 0x4d0000;
        if !no_node {
            c.cycles += 7;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x80));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x002f_d3c4)?;
            let not_move_node = c.g[V0][0] == 0;
            c.g[V0][0] = c.lw(gp.wrapping_add(0xce8));
            if !not_move_node {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x294));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x298));
                call_ptr(c, 0x0035_5db8, 0x002f_d3e0)?;
            }
            c.cycles += 1;
            c.g[V1][0] = 0x4d0000;
        }

        // return the nil constant (12 bytes copied to *out)
        c.cycles += 15;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx(0x4d0000u32.wrapping_sub(0x6f68));
        c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.ldr(4, c.g[A3][0] as u32);
        c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
        c.sdl(4, (c.g[S3][0] as u32).wrapping_add(0x7));
        c.sdr(4, c.g[S3][0] as u32);
        c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.lq(16, fp.wrapping_add(0x70));
        c.lq(17, fp.wrapping_add(0x60));
        c.lq(18, fp.wrapping_add(0x50));
        c.lq(19, fp.wrapping_add(0x40));
        c.g[RA][0] = c.ld(fp.wrapping_add(0x30));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(fp.wrapping_add(0x80));
        c.pc = ra;
        Ok(())
    })
}

/// `int App_Run_MainLoop(App *self, Exec *exec, a2, a3)` (0x00316d60): runs the
/// whole application: sets the frame time (1/60) and the tick count (60), calls
/// the exec's setup methods, loops the exec's frame methods (with
/// `EATimer_ServiceCallbacks(0)`, `App_InvokeExecCallback`) until its done
/// method says stop, runs the next module through `App_RunModuleLoop` and shuts
/// the exec down. A `match pc` body: every resume point of the generated
/// function is an arm, so a thread that unwound inside a callee continues here.
pub fn app_run_main_loop(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00316d60 => {
                    c.cycles += 20;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x2ef8));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = 0x3c;
                    c.g[S2][0] = c.g[A0][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[A0][0] = c.g[A1][0];
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[S1][0] = c.g[A2][0];
                    c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x14));
                    c.g[S0][0] = c.g[A3][0];
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                    c.g[A1][0] = 0x3c;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                    let t = c.g[A2][0] as u32;
                    c.g[RA][0] = sx(0x00316db0);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    call_ptr(c, t, 0x00316db0)?;
                    pc = 0x00316db0;
                    continue;
                }
                0x00316db0 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5c));
                    c.g[A2][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00316dcc);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00316dcc)?;
                    pc = 0x00316dcc;
                    continue;
                }
                0x00316dcc => {
                    c.cycles += 7;
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00316de8);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00316de8)?;
                    pc = 0x00316de8;
                    continue;
                }
                0x00316de8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x00316e48;
                        continue;
                    }
                    pc = 0x00316df0;
                    continue;
                }
                0x00316df0 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00316e04);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00316e04)?;
                    pc = 0x00316e04;
                    continue;
                }
                0x00316e04 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00316e0c);
                    c.g[A0][0] = 0u64;
                    call_ptr(c, 0x003e5928, 0x00316e0c)?;
                    pc = 0x00316e0c;
                    continue;
                }
                0x00316e0c => {
                    c.cycles += 6;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x60));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00316e24);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00316e24)?;
                    pc = 0x00316e24;
                    continue;
                }
                0x00316e24 => {
                    c.cycles += 6;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00316e3c);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00316e3c)?;
                    pc = 0x00316e3c;
                    continue;
                }
                0x00316e3c => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00316e44);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x00317328,
                        f: ee::s10::App_InvokeExecCallback,
                    }
                    .call(c, 0x00316e44)?;
                    pc = 0x00316e44;
                    continue;
                }
                0x00316e44 => {
                    c.cycles += 1;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    pc = 0x00316e48;
                    continue;
                }
                0x00316e48 => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00316e5c);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00316e5c)?;
                    pc = 0x00316e5c;
                    continue;
                }
                0x00316e5c => {
                    c.cycles += 2;

                    if c.g[V0][0] == 0u64 {
                        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                        if back_edge_unwinds(c, 0x00316df0) {
                            return Ok(());
                        }
                        pc = 0x00316df0;
                        continue;
                    }
                    pc = 0x00316e64;
                    continue;
                }
                0x00316e64 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x4), 0u64);
                    c.sw((c.g[S2][0] as u32), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                    c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
                    let t = c.g[A1][0] as u32;
                    c.g[RA][0] = sx(0x00316e84);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    call_ptr(c, t, 0x00316e84)?;
                    pc = 0x00316e84;
                    continue;
                }
                0x00316e84 => {
                    c.cycles += 3;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x58), 0u64);
                    c.g[RA][0] = sx(0x00316e90);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x00316f00,
                        f: ee::s10::App_RunModuleLoop,
                    }
                    .call(c, 0x00316e90)?;
                    pc = 0x00316e90;
                    continue;
                }
                0x00316e90 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5c));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00316ea4);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00316ea4)?;
                    pc = 0x00316ea4;
                    continue;
                }
                0x00316ea4 => {
                    c.cycles += 6;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00316ebc);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00316ebc)?;
                    pc = 0x00316ebc;
                    continue;
                }
                0x00316ebc => {
                    c.cycles += 3;
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0x3;
                    if t {
                        pc = 0x00316edc;
                        continue;
                    }
                    pc = 0x00316ec8;
                    continue;
                }
                0x00316ec8 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00316edc);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00316edc)?;
                    pc = 0x00316edc;
                    continue;
                }
                0x00316edc => {
                    c.cycles += 7;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[V0][0] = 0u64;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00316d60, 0x00316ef8);
                    return Ok(());
                }
            }
        }
    });
}

/// Internal arms of [`libkernl_intr_helper_thread`] (not guest addresses: they
/// never reach `c.pc`).
const ARM_WAKEUP: u32 = 0xfff0_0001;
const ARM_ROTATE: u32 = 0xfff0_0002;
const ARM_SUSPEND: u32 = 0xfff0_0003;
const ARM_PRINTF: u32 = 0xfff0_0004;

/// `void libkernl_IntrHelperThread(void *queue)` (0x00424aa0): the interrupt
/// helper thread's main. Forever: `WaitSema(sema)`, pop the next 2-byte record
/// of the queue (`queue[0] & 0x1ff` is the read index; type at `+8 + 2 * idx`,
/// argument at `+9 + 2 * idx`) and dispatch on its type: 0 `WakeupThread(arg)`,
/// 1 `RotateThreadReadyQueue(arg)`, 2 `SuspendThread(arg)`, anything else
/// `printf(fmt)`. It never returns. A `match pc` body so a thread that
/// unwound inside a callee (the parked `WaitSema`) resumes in the middle:
/// every return address of the generated body and the loop head are arms. The
/// generated body charges the whole receive block (12 cycles) before the
/// `WaitSema` call, so the resume arm after it charges nothing.
pub fn libkernl_intr_helper_thread(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0042_4aa0 => {
                    c.cycles += 16;
                    c.g[SP][0] = sx(sp(c).wrapping_sub(0x80));
                    c.sd(sp(c).wrapping_add(0x10), c.g[S1][0]);
                    c.g[S1][0] = c.g[A0][0];
                    c.sd(sp(c).wrapping_add(0x60), c.g[S6][0]);
                    c.sd(sp(c).wrapping_add(0x50), c.g[S5][0]);
                    c.g[S6][0] = 0x530000;
                    c.sd(sp(c).wrapping_add(0x40), c.g[S4][0]);
                    c.g[S5][0] = 0x4a0000;
                    c.sd(sp(c).wrapping_add(0x30), c.g[S3][0]);
                    c.g[S4][0] = 0x1;
                    c.sd(sp(c).wrapping_add(0x20), c.g[S2][0]);
                    c.g[S3][0] = 0x2;
                    c.sd(sp(c), c.g[S0][0]);
                    c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.sd(sp(c).wrapping_add(0x70), c.g[RA][0]);
                    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x9));
                    pc = 0x0042_4ae0;
                }
                // loop head: wait for the semaphore
                0x0042_4ae0 => {
                    c.cycles += 12;
                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_sub(0x4c60));
                    Callee {
                        addr: 0x0042_3de0,
                        f: ee::s15::WaitSema,
                    }
                    .call(c, 0x0042_4ae8)?;
                    pc = 0x0042_4ae8;
                }
                // after WaitSema: pop the record and dispatch on its type
                0x0042_4ae8 => {
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V1][0] &= 0x1ff;
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
                    c.sw(c.g[S1][0] as u32, c.g[A0][0]);
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lbu(c.g[V0][0] as u32);
                    let is_rotate = c.g[V0][0] == c.g[S4][0];
                    c.g[A1][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
                    if is_rotate {
                        pc = ARM_ROTATE;
                        continue;
                    }
                    c.cycles += 2;
                    let not_small = c.g[A1][0] == 0;
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x5520));
                    if not_small {
                        c.cycles += 2;
                        let is_suspend = c.g[V0][0] == c.g[S3][0];
                        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        if is_suspend {
                            pc = ARM_SUSPEND;
                        } else {
                            c.cycles += 2;
                            pc = ARM_PRINTF;
                        }
                    } else {
                        c.cycles += 2;
                        let is_wakeup = c.g[V0][0] == 0;
                        c.g[V1][0] = c.g[A2][0];
                        if is_wakeup {
                            pc = ARM_WAKEUP;
                        } else {
                            c.cycles += 2;
                            pc = ARM_PRINTF;
                        }
                    }
                }
                // type 0: WakeupThread(arg)
                ARM_WAKEUP => {
                    c.cycles += 4;
                    c.g[A0][0] = c.lbu(c.g[V1][0] as u32);
                    Callee {
                        addr: 0x0042_3cd0,
                        f: ee::s15::WakeupThread,
                    }
                    .call(c, 0x0042_4b40)?;
                    pc = 0x0042_4b40;
                }
                // type 1: RotateThreadReadyQueue(arg)
                ARM_ROTATE => {
                    c.cycles += 4;
                    c.g[A0][0] = c.lbu(c.g[A2][0] as u32);
                    Callee {
                        addr: 0x0042_3c50,
                        f: ee::s15::RotateThreadReadyQueue,
                    }
                    .call(c, 0x0042_4b50)?;
                    pc = 0x0042_4b50;
                }
                // type 2: SuspendThread(arg)
                ARM_SUSPEND => {
                    c.cycles += 4;
                    c.g[A0][0] = c.lbu(c.g[V1][0] as u32);
                    Callee {
                        addr: 0x0042_3d10,
                        f: ee::s15::SuspendThread,
                    }
                    .call(c, 0x0042_4b60)?;
                    pc = 0x0042_4b60;
                }
                // other types: printf(fmt)
                ARM_PRINTF => {
                    c.cycles += 4;
                    Callee {
                        addr: 0x0042_5c28,
                        f: ee::s15::libkernl_printf,
                    }
                    .call(c, 0x0042_4b70)?;
                    pc = 0x0042_4b70;
                }
                // back edge after any handler
                0x0042_4b40 | 0x0042_4b50 | 0x0042_4b60 | 0x0042_4b70 => {
                    if back_edge_unwinds(c, 0x0042_4ae0) {
                        return Ok(());
                    }
                    pc = 0x0042_4ae0;
                }
                _ => {
                    c.jump_out(pc, 0x0042_4aa0, 0x0042_4b78);
                    return Ok(());
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{
        compare, fakes, strict_ok, Rng, DATA, GAME_GP, RA as TEST_RA, SP as TEST_SP,
    };
    use std::ptr::addr_of_mut;

    /// Gate word of this module's callee fakes (a unique cell of the machine's
    /// RAM): the fakes answer only when a fixture sets it.
    const OC1601_GATE: u32 = DATA + 0x21_6010;
    const OC1601_MAGIC: u32 = 0x1601_f001;
    /// Target number of fake calls of the helper-thread resume test.
    const OC1601_TOTAL: u32 = DATA + 0x21_6020;

    /// A gated fake for the real callee at `$addr`: with the gate off it runs
    /// whatever the slot held before, with the gate on it logs the call and
    /// returns (or unwinds) like `fakes::fake_call`.
    macro_rules! oc1601_fake {
        ($name:ident, $gen:path, $addr:expr) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC1601_GATE) != OC1601_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                }
                fakes::fake_call(c, $addr);
            }
        };
    }

    oc1601_fake!(
        oc1601_fake_mem_copy,
        ee::s14::f_003e6574__generated,
        0x003e_6574
    );
    oc1601_fake!(
        oc1601_fake_add_roller,
        ee::s11::f_00355db8__generated,
        0x0035_5db8
    );
    oc1601_fake!(
        oc1601_fake_invoke_exec,
        ee::s10::f_00317328__generated,
        0x0031_7328
    );
    oc1601_fake!(
        oc1601_fake_module_loop,
        ee::s10::f_00316f00__generated,
        0x0031_6f00
    );
    oc1601_fake!(
        oc1601_fake_service_timers,
        ee::s14::f_003e5928__generated,
        0x003e_5928
    );
    oc1601_fake!(
        oc1601_fake_wait_sema,
        ee::s15::f_00423de0__generated,
        0x0042_3de0
    );
    oc1601_fake!(
        oc1601_fake_wakeup,
        ee::s15::f_00423cd0__generated,
        0x0042_3cd0
    );
    oc1601_fake!(
        oc1601_fake_rotate,
        ee::s15::f_00423c50__generated,
        0x0042_3c50
    );
    oc1601_fake!(
        oc1601_fake_suspend,
        ee::s15::f_00423d10__generated,
        0x0042_3d10
    );
    oc1601_fake!(
        oc1601_fake_printf,
        ee::s15::f_00425c28__generated,
        0x0042_5c28
    );

    /// Hook every fake of this module into its callee's slot (once per process).
    fn oc1601_install() {
        macro_rules! hook {
            ($slot:path, $fake:ident) => {
                fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake));
            };
        }
        hook!(ee::s14::OVERRIDE_003E6574, oc1601_fake_mem_copy);
        hook!(ee::s11::OVERRIDE_00355DB8, oc1601_fake_add_roller);
        hook!(ee::s10::OVERRIDE_00317328, oc1601_fake_invoke_exec);
        hook!(ee::s10::OVERRIDE_00316F00, oc1601_fake_module_loop);
        hook!(ee::s14::OVERRIDE_003E5928, oc1601_fake_service_timers);
        hook!(ee::s15::OVERRIDE_00423DE0, oc1601_fake_wait_sema);
        hook!(ee::s15::OVERRIDE_00423CD0, oc1601_fake_wakeup);
        hook!(ee::s15::OVERRIDE_00423C50, oc1601_fake_rotate);
        hook!(ee::s15::OVERRIDE_00423D10, oc1601_fake_suspend);
        hook!(ee::s15::OVERRIDE_00425C28, oc1601_fake_printf);
    }

    /// `fakes::machine` with this module's fake gate switched on.
    fn oc1601_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = fakes::machine(c, seed, salt, calls);
        c.bus.write32(OC1601_GATE, OC1601_MAGIC);
        r
    }

    /// A vtable slot `{delta: i16, fn}` at `vt + off` whose function is the
    /// `i`-th fake method.
    fn oc1601_slot(c: &mut Cpu, r: &mut Rng, vt: u32, off: u32, i: u32) {
        c.bus
            .write16(vt + off, (r.next_u32() as i16 as i32 & !3) as u16);
        c.bus.write32(vt + off + 4, fakes::FAKE + 0x10 * i);
    }

    /// `WScriptFn_AddRollerModifier`: the default block init (flag clear/set),
    /// the entry loop (equal type, different type with a Float keyword and
    /// not), the three instance lookups (script manager, world cache hit, miss
    /// by section and by tag), a missing active node, a node that is not a move
    /// node (real leaf returning 0) and one that is (`addRoller` called), with
    /// an unwind at each of the three calls.
    #[test]
    fn wscript_fn_add_roller_modifier_matches() {
        oc1601_install();
        fakes::check_reaching(
            ee::s10::f_002fd250__generated,
            wscript_fn_add_roller_modifier,
            300,
            &[0x002f_d2e4, 0x002f_d3c4, 0x002f_d3e0],
            &|c, seed| {
                let mut r = oc1601_machine(c, seed, 0x002f_d250, 3);
                let n = seed % 5;
                let params = DATA + 0x1000;
                let out = DATA + 0x1800;
                let (world, wcache, stab, sec, ctab) = (
                    DATA + 0x2000,
                    DATA + 0x2100,
                    DATA + 0x2200,
                    DATA + 0x2300,
                    DATA + 0x2400,
                );
                let (man, obj, node, vt) =
                    (DATA + 0x2800, DATA + 0x3000, DATA + 0x3400, DATA + 0x3800);
                // keyword types of the five slots
                let mut kw = [0u32; 8];
                for (i, k) in kw.iter_mut().enumerate() {
                    *k = r.next_u32() % 4;
                    c.bus.write32(0x0044_60a0 + 4 * i as u32, *k);
                }
                // the target id: -1 (script manager), a cache hit, a miss
                let kind = (seed / 5) % 3;
                let sidx = r.next_u32() & 3;
                let kidx = r.next_u32() & 3;
                let id = match kind {
                    0 => 0xffff_ffffu32,
                    _ => (kidx << 8) | sidx,
                };
                // entry 0 sets the id (a matching type: plain store; sometimes
                // a different type: float conversion when the keyword says so)
                let t0 = if seed % 7 == 0 { kw[0] ^ 2 } else { kw[0] };
                c.bus.write32(params, 0);
                c.bus.write32(params + 4, id);
                c.bus.write32(params + 0xc, t0);
                for i in 1..5u32 {
                    let e = params + 0x10 * i;
                    c.bus.write32(e, 1 + r.next_u32() % 4);
                    c.bus.write32(e + 4, r.next_u32() % 4000);
                    c.bus.write32(e + 8, r.next_u32());
                    c.bus.write32(e + 0xc, r.next_u32() % 4);
                }
                // world cache
                c.bus.write32(GAME_GP + 0x16c8, world);
                c.bus.write32(world, wcache);
                c.bus.write32(wcache + 8, stab);
                let no_section = kind == 2 && seed % 2 == 0;
                c.bus
                    .write32(stab + 4 * sidx, if no_section { 0 } else { sec });
                c.bus.write32(sec + 0x1c, ctab);
                let tag = if kind == 1 {
                    ((obj >> 2) << 8) | (r.next_u32() & 0xff)
                } else {
                    r.next_u32() & 0xff
                };
                c.bus.write32(ctab + 4 * kidx, tag);
                // script manager and the instance
                c.bus.write32(GAME_GP + 0xce8, man);
                c.bus.write32(man + 0x290, obj);
                c.bus.write32(man + 0x294, r.next_u32());
                c.bus.write32(man + 0x298, r.next_u32());
                c.bus
                    .write32(obj + 0xc, if seed % 4 == 0 { 0 } else { node });
                c.bus.write32(node + 0xc, vt);
                c.bus
                    .write16(vt + 0x80, (r.next_u32() as i16 as i32 & !3) as u16);
                let move_node = (seed / 3) % 2 == 0;
                // a real leaf returning 0 is "not a move node"
                c.bus
                    .write32(vt + 0x84, if move_node { fakes::FAKE } else { 0x0014_06c0 });
                // init flag and defaults
                c.bus.write32(GAME_GP + 0x2510, (seed / 2) % 2);
                let fl = r.f32(1.0);
                c.bus.write32(GAME_GP.wrapping_sub(0x3388), fl);
                for k in 0..6u32 {
                    c.bus.write32(0x004f_b628 + 4 * k, r.next_u32());
                    c.bus.write32(0x004c_9098 + 4 * k, r.next_u32());
                    c.bus.write32(out + 4 * k, r.next_u32());
                }
                c.g[A0][0] = out as u64;
                c.g[A1][0] = n as u64;
                c.g[A2][0] = params as u64;
            },
        );
    }

    /// Every resume point of `App_Run_MainLoop` (the start, the return address
    /// of each call and the loop head).
    const MAIN_LOOP_ENTRIES: [u32; 19] = [
        0x0031_6d60,
        0x0031_6db0,
        0x0031_6dcc,
        0x0031_6de8,
        0x0031_6df0,
        0x0031_6e04,
        0x0031_6e0c,
        0x0031_6e24,
        0x0031_6e3c,
        0x0031_6e44,
        0x0031_6e48,
        0x0031_6e5c,
        0x0031_6e64,
        0x0031_6e84,
        0x0031_6e90,
        0x0031_6ea4,
        0x0031_6ebc,
        0x0031_6ec8,
        0x0031_6edc,
    ];

    /// `App_Run_MainLoop` from the start and from every resume entry: the
    /// exec's vtable calls (fake methods), the gated `EATimer_ServiceCallbacks`,
    /// `App_InvokeExecCallback` and `App_RunModuleLoop`, the done-check loop
    /// (entered with `v0 == 0`), a null exec at the final shutdown, and unwinds.
    #[test]
    fn app_run_main_loop_matches() {
        oc1601_install();
        fakes::check_reaching(
            ee::s10::f_00316d60__generated,
            app_run_main_loop,
            380,
            &[
                0x0031_6db0,
                0x0031_6dcc,
                0x0031_6de8,
                0x0031_6e04,
                0x0031_6e0c,
                0x0031_6e24,
                0x0031_6e3c,
                0x0031_6e44,
                0x0031_6e5c,
                0x0031_6e84,
                0x0031_6e90,
                0x0031_6ea4,
                0x0031_6ebc,
                0x0031_6edc,
            ],
            &|c, seed| {
                let mut r = oc1601_machine(c, seed, 0x0031_6d60, 4);
                let entry = MAIN_LOOP_ENTRIES[(seed % 19) as usize];
                let (selfp, exec, vte, vt2, next, vtn) = (
                    DATA + 0x1000,
                    DATA + 0x2000,
                    DATA + 0x2400,
                    DATA + 0x2800,
                    DATA + 0x4000,
                    DATA + 0x4400,
                );
                c.bus.write32(selfp + 4, next);
                let null_exec = entry == 0x0031_6ebc && seed % 3 == 0;
                c.bus.write32(selfp + 8, if null_exec { 0 } else { exec });
                c.bus.write32(selfp + 0x5c, vt2);
                c.bus.write32(exec + 8, vte);
                c.bus.write32(next, vtn);
                for (i, off) in [0x10u32, 0x48, 0x58, 0x60, 0x20, 0x50, 0x18, 0x08]
                    .into_iter()
                    .enumerate()
                {
                    oc1601_slot(c, &mut r, vte, off, i as u32);
                }
                oc1601_slot(c, &mut r, vt2, 0x18, 8);
                oc1601_slot(c, &mut r, vt2, 0x20, 9);
                oc1601_slot(c, &mut r, vtn, 0x28, 10);
                let fl = r.f32(1.0);
                c.bus.write32(GAME_GP.wrapping_sub(0x2ef8), fl);
                c.g[A0][0] = selfp as u64;
                c.g[A1][0] = exec as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                if entry != 0x0031_6d60 {
                    c.g[S2][0] = selfp as u64;
                    c.g[S1][0] = r.next_u32() as u64;
                    c.g[S0][0] = r.next_u32() as u64;
                    c.g[V0][0] = if seed % 2 == 0 {
                        0
                    } else {
                        r.next_u32() as u64
                    };
                    if entry == 0x0031_6ec8 {
                        c.g[A2][0] = exec as u64;
                        c.g[A1][0] = 3;
                    }
                    c.bus.write64(TEST_SP, TEST_RA as u64);
                }
                c.pc = entry;
            },
        );
    }

    /// The generated helper thread, driven until the fake call number
    /// `OC1601_TOTAL` unwinds.
    fn oc1601_helper_generated(c: &mut Cpu) {
        let total = c.bus.read32(OC1601_TOTAL);
        c.bus.write32(fakes::LOG_UNWIND_AT, total);
        ee::s15::f_00424aa0__generated(c);
    }

    /// The port, driven until the fake call number `OC1601_TOTAL` unwinds.
    fn oc1601_helper_port(c: &mut Cpu) {
        let total = c.bus.read32(OC1601_TOTAL);
        c.bus.write32(fakes::LOG_UNWIND_AT, total);
        libkernl_intr_helper_thread(c);
    }

    /// The port driven the same way as a native thread would be: every fake
    /// call unwinds, and the thread is re-entered at the call's return address
    /// (the fake's results are the same as for a normal return), until call
    /// number `OC1601_TOTAL` unwinds.
    fn oc1601_helper_resumed(c: &mut Cpu) {
        let total = c.bus.read32(OC1601_TOTAL);
        c.bus.write32(fakes::LOG_UNWIND_AT, 0);
        libkernl_intr_helper_thread(c);
        while c.pc & 0xffff_0000 == 0xdead_0000 {
            let n = (c.pc & 0xffff) >> 4;
            if n >= total {
                break;
            }
            c.bus.write32(fakes::LOG_UNWIND_AT, n + 1);
            let ret = fakes::logged_returns(c)[n as usize];
            c.pc = ret;
            libkernl_intr_helper_thread(c);
        }
    }

    /// Queue and registers of the helper thread: the read index, 64 records of
    /// every type (wake, rotate, suspend, two printf kinds).
    fn oc1601_helper_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1601_machine(c, seed, 0x0042_4aa0, 0);
        let queue = DATA + 0x4000;
        c.bus.write32(queue, r.next_u32() % 20);
        for i in 0..64u32 {
            c.bus.write8(queue + 8 + 2 * i, (r.next_u32() % 5) as u8);
            c.bus.write8(queue + 9 + 2 * i, r.next_u32() as u8);
        }
        c.bus
            .write32(0x0053_0000u32.wrapping_sub(0x4c60), r.next_u32());
        c.bus.write32(OC1601_TOTAL, 1 + seed % 11);
        c.g[A0][0] = queue as u64;
        c.pc = 0x0042_4aa0;
    }

    /// `libkernl_IntrHelperThread` over many dispatched records: wake, rotate,
    /// suspend and printf arms, the loop back edge, ending at a chosen fake
    /// call's unwind (the thread never returns).
    #[test]
    fn libkernl_intr_helper_thread_matches() {
        oc1601_install();
        for seed in 0..60 {
            let r = compare(oc1601_helper_generated, oc1601_helper_port, &|c| {
                oc1601_helper_setup(c, seed)
            });
            assert!(r.hard.is_empty(), "seed {seed}: {:#?}", r.hard);
            assert!(strict_ok(&r.soft), "seed {seed} (scratch): {:#?}", r.soft);
        }
    }

    /// The resume shape: the port unwinding at every call and re-entered at the
    /// call's return address ends in exactly the state of the generated
    /// (uninterrupted) body.
    #[test]
    fn libkernl_intr_helper_thread_resumes() {
        oc1601_install();
        for seed in 0..60 {
            let r = compare(oc1601_helper_generated, oc1601_helper_resumed, &|c| {
                oc1601_helper_setup(c, seed)
            });
            assert!(r.hard.is_empty(), "seed {seed}: {:#?}", r.hard);
            assert!(strict_ok(&r.soft), "seed {seed} (scratch): {:#?}", r.soft);
        }
    }
}
