//! Staging module oc706 (wave wave239): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc706`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0030_6300,
        wscript_fn_career_action,
        "WScriptFn_CareerAction",
    ),
    (0x0030_6438, wscript_fn_add_danger, "WScriptFn_AddDanger"),
    (
        0x0030_65e8,
        wscript_fn_remove_danger,
        "WScriptFn_RemoveDanger",
    ),
    (
        0x0030_6770,
        wscript_fn_acquire_danger_zone,
        "WScriptFn_AcquireDangerZone",
    ),
    (
        0x0030_6908,
        wscript_fn_release_danger_zone,
        "WScriptFn_ReleaseDangerZone",
    ),
    (
        0x0030_6e68,
        wscript_fn_check_rider_gate,
        "WScriptFn_CheckRiderGate",
    ),
    (
        0x0030_7020,
        wscript_fn_career_check,
        "WScriptFn_CareerCheck",
    ),
    (
        0x0032_db40,
        sphere_tree_sort_child_order,
        "SphereTree_SortChildOrder",
    ),
];

/// The 12/16-byte `luno_gLunoNil` struct the WScript bodies return through
/// their struct return pointer (`sret`): the generated tail, verbatim
/// (unaligned `ldl`/`ldr` through `a0`, then a word; the same stores to `sret`).
fn wscript_return_nil(c: &mut Cpu, sret: u64) {
    c.g[V1][0] = 0x4d0000;
    c.g[V0][0] = sret;
    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
    c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
    c.ldr(4, c.g[A3][0] as u32);
    c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
    c.sdl(4, (sret as u32).wrapping_add(0x7));
    c.sdr(4, sret as u32);
    c.sw((sret as u32).wrapping_add(0x8), c.g[A1][0]);
}

/// The `MEM_copy(vp, block, len)` call the WScript bodies open their param
/// block with: `vp` is `sp + 0x10` (kept in s0), `src` the block in `a1`.
fn wscript_copy_to_vp(c: &mut Cpu, cycles: u64, src: u64, len: u64, ret_addr: u32) -> Flow {
    c.cycles += cycles;
    c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A2][0] = len;
    c.g[A1][0] = src;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x003e_6574, ret_addr)
}

/// The `(params, n)` loop shared by the WScript bodies: each argument is
/// stored into the block at `vp` by its index, as a float when the kind
/// table says float and the argument's kind differs, else as an int.
/// `entry` is the cycle charge of the loop's entry block, `top` its back-edge
/// head, `table` the kind table. Leaves a0..a3, t0, v0, v1 as the loop does.
fn wscript_fill_block(
    c: &mut Cpu,
    vp: u64,
    count: u64,
    params: u64,
    table: u32,
    entry: u64,
    top: u32,
) {
    c.g[A2][0] = params;
    if (count as i64) <= 0 {
        return;
    }
    c.cycles += entry;
    c.g[A1][0] = count;
    c.g[T0][0] = sx(table);
    c.g[A3][0] = 2;
    loop {
        c.cycles += 8;
        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(vp as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = c.lw(c.g[V0][0] as u32);
        let mut int_store = true;
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
                int_store = false;
            }
        }
        if int_store {
            c.cycles += 1;
            c.sw(c.g[A0][0] as u32, c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
        let more = c.g[A1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, top);
        }
    }
}

/// The zone-tag lookup of the four danger wrappers, from `vp` (the x word).
/// Returns true when the tag lookup ran (its result is in v0); false when
/// x is -1 and v0 (or v1, when `to_v1`) takes `*(gp+0xce8)->0x290`.
fn wscript_zone_lookup(c: &mut Cpu, vp: u64, to_v1: bool) -> bool {
    let gp = c.g[GP][0] as u32;
    c.cycles += 4;
    c.g[A2][0] = c.lw(vp as u32);
    c.g[V0][0] = !(0u64 | c.g[A2][0]);
    let t = c.g[V0][0] != 0;
    c.g[A1][0] = c.lw(gp.wrapping_add(0x16c8));
    if !t {
        c.cycles += 3;
        c.g[V0][0] = c.lw(gp.wrapping_add(0xce8));
        if to_v1 {
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
        } else {
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
        }
        return false;
    }
    c.cycles += 8;
    c.g[V0][0] = c.g[A2][0] & 0xff;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
    let found = c.g[V1][0] != 0;
    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
    if found {
        c.cycles += 7;
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
        let nonzero = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        if nonzero {
            return true;
        }
    }
    c.cycles += 1;
    c.g[V0][0] = 0;
    true
}

/// `WScriptFn_CareerAction` (0x306300): fills the once-initialised career
/// block at `vp` from the argument list (int or float by the kind table),
/// calls the BE interface with `SSXApp->BE` and 7, and when no network game
/// runs, `func_001E3510(x, y)`. Returns `luno_gLunoNil` through the sret.
pub fn wscript_fn_career_action(c: &mut Cpu) {
    guest(c, career_action_body)
}

fn career_action_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    c.cycles += 12;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
    c.g[V0][0] = c.lw(gp.wrapping_add(0x279c));
    c.sq(17, sp(c).wrapping_add(0x50));
    c.sq(18, sp(c).wrapping_add(0x40));
    c.g[S1][0] = c.g[A1][0];
    c.sq(19, sp(c).wrapping_add(0x30));
    c.g[S2][0] = c.g[A2][0];
    c.sq(16, sp(c).wrapping_add(0x60));
    c.g[S3][0] = c.g[A0][0];
    c.sd(sp(c).wrapping_add(0x20), c.g[RA][0]);
    let initialised = c.g[V0][0] != 0;
    c.g[A1][0] = 0x500000;
    if !initialised {
        c.cycles += 8;
        c.sw((c.g[A1][0] as u32).wrapping_sub(0x40c8), 0);
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x40c8));
        c.g[V1][0] = 0x1;
        c.g[A0][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x8));
        c.sw(gp.wrapping_add(0x279c), c.g[V1][0]);
        c.sb((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0);
        c.sb(sp(c), c.g[A0][0]);
    }
    let src = sx((c.g[A1][0] as u32).wrapping_sub(0x40c8));
    wscript_copy_to_vp(c, 7, src, 0xc, 0x0030_6364)?;
    let vp = c.g[S0][0];
    let (count, params) = (c.g[S1][0], c.g[S2][0]);
    wscript_fill_block(c, vp, count, params, gp.wrapping_add(0xcc0), 3, 0x0030_6378);

    // The BE interface, then the network check.
    c.cycles += 10;
    c.g[V0][0] = c.lw(gp.wrapping_sub(0x848));
    c.g[A1][0] = 0x7;
    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
    c.g[S0][0] = c.lw(c.g[S0][0] as u32);
    call_ptr(c, 0x0014_dd58, 0x0030_63d8)?;
    c.g[V1][0] = 0x530000;
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4b30));
    let networked = c.g[V0][0] != 0;
    c.g[V1][0] = 0x4d0000;
    if !networked {
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x001e_3510, 0x0030_63f4)?;
        c.g[V1][0] = 0x4d0000;
    }

    c.cycles += 15;
    let sret = c.g[S3][0];
    wscript_return_nil(c, sret);
    c.g[V0][0] = c.g[S3][0];
    c.lq(16, sp(c).wrapping_add(0x60));
    c.lq(17, sp(c).wrapping_add(0x50));
    c.lq(18, sp(c).wrapping_add(0x40));
    c.lq(19, sp(c).wrapping_add(0x30));
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x20));
    c.g[SP][0] = sx(sp(c).wrapping_add(0x70));
    ret(c);
    Ok(())
}

/// `WScriptFn_AddDanger` (0x306438): stores the argument block as above
/// (a -1 head, 16 bytes), looks up the zone tag for x, and when it is set
/// calls the danger manager's add (0x101728) on the tagged object.
pub fn wscript_fn_add_danger(c: &mut Cpu) {
    guest(c, add_danger_body)
}

fn add_danger_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    c.cycles += 12;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
    c.g[V0][0] = c.lw(gp.wrapping_add(0x27a0));
    c.sq(18, sp(c).wrapping_add(0x50));
    c.sq(17, sp(c).wrapping_add(0x60));
    c.g[S2][0] = c.g[A2][0];
    c.sq(19, sp(c).wrapping_add(0x40));
    c.g[S1][0] = c.g[A1][0];
    c.sq(16, sp(c).wrapping_add(0x70));
    c.g[S3][0] = c.g[A0][0];
    c.sd(sp(c).wrapping_add(0x30), c.g[RA][0]);
    let initialised = c.g[V0][0] != 0;
    c.g[A2][0] = 0x500000;
    if !initialised {
        c.cycles += 13;
        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x40b8));
        c.g[V0][0] = u64::MAX;
        c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0xc));
        c.g[V0][0] |= 0xffff;
        c.g[AT][0] = 0x447a_0000;
        c.mtc1(1, 0);
        c.g[A0][0] = 0x1;
        c.sw((c.g[A2][0] as u32).wrapping_sub(0x40b8), c.g[V0][0]);
        c.sb((c.g[V1][0] as u32).wrapping_add(0xc), c.g[A1][0]);
        c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
        c.sw(gp.wrapping_add(0x27a0), c.g[A0][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0);
        c.sb(sp(c), c.g[A1][0]);
    }
    let src = sx((c.g[A2][0] as u32).wrapping_sub(0x40b8));
    c.g[V0][0] = u64::MAX;
    c.sw(sp(c).wrapping_add(0x10), c.g[V0][0]);
    wscript_copy_to_vp(c, 10, src, 0x10, 0x0030_64bc)?;
    let vp = c.g[S0][0];
    let (count, params) = (c.g[S1][0], c.g[S2][0]);
    wscript_fill_block(c, vp, count, params, 0x0044_6880, 5, 0x0030_64d8);

    wscript_zone_lookup(c, vp, false);
    // The tagged object's add.
    c.cycles += 2;
    let tagged = c.g[V0][0] != 0;
    c.g[A1][0] = sx(sp(c).wrapping_add(0x20));
    if tagged {
        c.cycles += 9;
        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
        c.lwc1(13, (c.g[S0][0] as u32).wrapping_add(0x8));
        c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x4));
        c.sw(sp(c).wrapping_add(0x20), c.g[V0][0]);
        c.g[A0][0] = c.lw(gp.wrapping_sub(0x848));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa8));
        call_ptr(c, 0x0010_1728, 0x0030_65a8)?;
    }

    c.cycles += 16;
    let sret = c.g[S3][0];
    wscript_return_nil(c, sret);
    c.g[V0][0] = c.g[S3][0];
    c.lq(16, sp(c).wrapping_add(0x70));
    c.lq(17, sp(c).wrapping_add(0x60));
    c.lq(18, sp(c).wrapping_add(0x50));
    c.lq(19, sp(c).wrapping_add(0x40));
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x30));
    c.g[SP][0] = sx(sp(c).wrapping_add(0x80));
    ret(c);
    Ok(())
}

/// `WScriptFn_RemoveDanger` (0x3065e8): the remove twin of the add body, with
/// the 8-byte block at `gp-0x...` and the manager's remove (0x101888).
pub fn wscript_fn_remove_danger(c: &mut Cpu) {
    guest(c, remove_danger_body)
}

fn remove_danger_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    c.cycles += 11;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
    c.g[V0][0] = c.lw(gp.wrapping_add(0x27b0));
    c.sq(17, sp(c).wrapping_add(0x60));
    c.sq(18, sp(c).wrapping_add(0x50));
    c.g[S1][0] = c.g[A1][0];
    c.sq(19, sp(c).wrapping_add(0x40));
    c.g[S2][0] = c.g[A2][0];
    c.sq(16, sp(c).wrapping_add(0x70));
    c.g[S3][0] = c.g[A0][0];
    let initialised = c.g[V0][0] != 0;
    c.sd(sp(c).wrapping_add(0x30), c.g[RA][0]);
    if !initialised {
        c.cycles += 7;
        c.g[V0][0] = c.lbu(gp.wrapping_add(0x27ac));
        c.g[V1][0] = u64::MAX;
        c.g[V1][0] |= 0xffff;
        c.g[A0][0] = 0x1;
        c.sw(gp.wrapping_add(0x27a8), c.g[V1][0]);
        c.sw(gp.wrapping_add(0x27b0), c.g[A0][0]);
        c.sb(sp(c), c.g[V0][0]);
    }
    c.g[V0][0] = u64::MAX;
    c.sw(sp(c).wrapping_add(0x10), c.g[V0][0]);
    let src = sx(gp.wrapping_add(0x27a8));
    wscript_copy_to_vp(c, 10, src, 0x8, 0x0030_6650)?;
    let vp = c.g[S0][0];
    let (count, params) = (c.g[S1][0], c.g[S2][0]);
    wscript_fill_block(c, vp, count, params, gp.wrapping_add(0xcc8), 4, 0x0030_6668);

    wscript_zone_lookup(c, vp, false);
    c.cycles += 2;
    let tagged = c.g[V0][0] != 0;
    c.g[A1][0] = sx(sp(c).wrapping_add(0x20));
    if tagged {
        c.cycles += 7;
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
        c.g[A0][0] = c.lw(gp.wrapping_sub(0x848));
        c.sw(sp(c).wrapping_add(0x20), c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa8));
        call_ptr(c, 0x0010_1888, 0x0030_6730)?;
    }

    c.cycles += 16;
    let sret = c.g[S3][0];
    wscript_return_nil(c, sret);
    c.g[V0][0] = c.g[S3][0];
    c.lq(16, sp(c).wrapping_add(0x70));
    c.lq(17, sp(c).wrapping_add(0x60));
    c.lq(18, sp(c).wrapping_add(0x50));
    c.lq(19, sp(c).wrapping_add(0x40));
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x30));
    c.g[SP][0] = sx(sp(c).wrapping_add(0x80));
    ret(c);
    Ok(())
}

/// `WScriptFn_AcquireDangerZone` (0x306770): the zone-acquire wrapper. The
/// tag lookup's result goes to v1, and a set tag calls the manager's acquire
/// (0x101900) with the argument at `vp + 4`.
pub fn wscript_fn_acquire_danger_zone(c: &mut Cpu) {
    guest(c, acquire_danger_zone_body)
}

fn acquire_danger_zone_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    c.cycles += 12;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
    c.g[V0][0] = c.lw(gp.wrapping_add(0x27b4));
    c.sq(18, sp(c).wrapping_add(0x40));
    c.sq(17, sp(c).wrapping_add(0x50));
    c.g[S2][0] = c.g[A2][0];
    c.sq(19, sp(c).wrapping_add(0x30));
    c.g[S1][0] = c.g[A1][0];
    c.sq(16, sp(c).wrapping_add(0x60));
    c.g[S3][0] = c.g[A0][0];
    c.sd(sp(c).wrapping_add(0x20), c.g[RA][0]);
    let initialised = c.g[V0][0] != 0;
    c.g[A2][0] = 0x500000;
    if !initialised {
        c.cycles += 10;
        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x40a8));
        c.g[V0][0] = u64::MAX;
        c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x8));
        c.sw((c.g[A2][0] as u32).wrapping_sub(0x40a8), c.g[V0][0]);
        c.g[A0][0] = 0x1;
        c.sb((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.sw(gp.wrapping_add(0x27b4), c.g[A0][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0);
        c.sb(sp(c), c.g[A1][0]);
    }
    let src = sx((c.g[A2][0] as u32).wrapping_sub(0x40a8));
    c.g[V0][0] = u64::MAX;
    c.sw(sp(c).wrapping_add(0x10), c.g[V0][0]);
    wscript_copy_to_vp(c, 10, src, 0xc, 0x0030_67e8)?;
    let vp = c.g[S0][0];
    let (count, params) = (c.g[S1][0], c.g[S2][0]);
    wscript_fill_block(c, vp, count, params, gp.wrapping_add(0xcd0), 4, 0x0030_6800);

    if wscript_zone_lookup(c, vp, true) {
        c.cycles += 1;
        c.g[V1][0] = c.g[V0][0];
    }
    // The tagged object's acquire.
    c.cycles += 2;
    let tagged = c.g[V1][0] != 0;
    c.g[V0][0] = c.lw(gp.wrapping_sub(0x848));
    if tagged {
        c.cycles += 6;
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x78));
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa8));
        call_ptr(c, 0x0010_1900, 0x0030_68c8)?;
    }

    c.cycles += 16;
    let sret = c.g[S3][0];
    wscript_return_nil(c, sret);
    c.g[V0][0] = c.g[S3][0];
    c.lq(16, sp(c).wrapping_add(0x60));
    c.lq(17, sp(c).wrapping_add(0x50));
    c.lq(18, sp(c).wrapping_add(0x40));
    c.lq(19, sp(c).wrapping_add(0x30));
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x20));
    c.g[SP][0] = sx(sp(c).wrapping_add(0x70));
    ret(c);
    Ok(())
}

/// `WScriptFn_ReleaseDangerZone` (0x306908): the zone-release twin of the
/// acquire body, calling the manager's release (0x101970).
pub fn wscript_fn_release_danger_zone(c: &mut Cpu) {
    guest(c, release_danger_zone_body)
}

fn release_danger_zone_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    c.cycles += 11;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
    c.g[V0][0] = c.lw(gp.wrapping_add(0x27c0));
    c.sq(17, sp(c).wrapping_add(0x50));
    c.sq(18, sp(c).wrapping_add(0x40));
    c.g[S1][0] = c.g[A1][0];
    c.sq(19, sp(c).wrapping_add(0x30));
    c.g[S2][0] = c.g[A2][0];
    c.sq(16, sp(c).wrapping_add(0x60));
    c.g[S3][0] = c.g[A0][0];
    let initialised = c.g[V0][0] != 0;
    c.sd(sp(c).wrapping_add(0x20), c.g[RA][0]);
    if !initialised {
        c.cycles += 7;
        c.g[V0][0] = c.lbu(gp.wrapping_add(0x27bc));
        c.g[V1][0] = u64::MAX;
        c.g[A0][0] = 0x1;
        c.sw(gp.wrapping_add(0x27b8), c.g[V1][0]);
        c.sw(gp.wrapping_add(0x27c0), c.g[A0][0]);
        c.sb(sp(c), c.g[V0][0]);
    }
    c.g[V0][0] = u64::MAX;
    c.sw(sp(c).wrapping_add(0x10), c.g[V0][0]);
    let src = sx(gp.wrapping_add(0x27b8));
    wscript_copy_to_vp(c, 10, src, 0x8, 0x0030_6970)?;
    let vp = c.g[S0][0];
    let (count, params) = (c.g[S1][0], c.g[S2][0]);
    wscript_fill_block(c, vp, count, params, gp.wrapping_add(0xcd8), 4, 0x0030_6988);

    if wscript_zone_lookup(c, vp, true) {
        c.cycles += 1;
        c.g[V1][0] = c.g[V0][0];
    }
    c.cycles += 2;
    let tagged = c.g[V1][0] != 0;
    c.g[V0][0] = c.lw(gp.wrapping_sub(0x848));
    if tagged {
        c.cycles += 5;
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x78));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xa8));
        call_ptr(c, 0x0010_1970, 0x0030_6a4c)?;
    }

    c.cycles += 16;
    let sret = c.g[S3][0];
    wscript_return_nil(c, sret);
    c.g[V0][0] = c.g[S3][0];
    c.lq(16, sp(c).wrapping_add(0x60));
    c.lq(17, sp(c).wrapping_add(0x50));
    c.lq(18, sp(c).wrapping_add(0x40));
    c.lq(19, sp(c).wrapping_add(0x30));
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x20));
    c.g[SP][0] = sx(sp(c).wrapping_add(0x70));
    ret(c);
    Ok(())
}

/// `WScriptFn_CheckRiderGate` (0x306e68): fills the block from the argument
/// list, asks `WScriptMan_CheckRiderGate` (0x30bfc0) for the gate; when the
/// gate is open and the tagged entry has a flag word, it calls the two
/// helpers (0x28b180, 0x2974a0). Then it reports the result to 0x226610.
pub fn wscript_fn_check_rider_gate(c: &mut Cpu) {
    guest(c, check_rider_gate_body)
}

fn check_rider_gate_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    c.cycles += 12;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
    c.g[V0][0] = c.lw(gp.wrapping_add(0x27cc));
    c.sq(19, sp(c).wrapping_add(0x30));
    c.sq(17, sp(c).wrapping_add(0x50));
    c.g[S3][0] = c.g[A2][0];
    c.sq(18, sp(c).wrapping_add(0x40));
    c.g[S1][0] = c.g[A1][0];
    c.sq(16, sp(c).wrapping_add(0x60));
    c.g[S2][0] = c.g[A0][0];
    c.sd(sp(c).wrapping_add(0x20), c.g[RA][0]);
    let initialised = c.g[V0][0] != 0;
    c.g[A2][0] = 0x500000;
    if !initialised {
        c.cycles += 11;
        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x3fa0));
        c.g[V0][0] = u64::MAX;
        c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0xc));
        c.sw((c.g[A2][0] as u32).wrapping_sub(0x3fa0), c.g[V0][0]);
        c.g[A0][0] = 0x1;
        c.sb((c.g[V1][0] as u32).wrapping_add(0xc), c.g[A1][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.sw(gp.wrapping_add(0x27cc), c.g[A0][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.sb(sp(c), c.g[A1][0]);
    }
    // The block copy: vp = sp + 0x10, 16 bytes, with the three -1 heads.
    c.g[V0][0] = u64::MAX;
    let src = sx((c.g[A2][0] as u32).wrapping_sub(0x3fa0));
    c.sw(sp(c).wrapping_add(0x18), c.g[V0][0]);
    c.sw(sp(c).wrapping_add(0x10), c.g[V0][0]);
    c.sw(sp(c).wrapping_add(0x14), c.g[V0][0]);
    wscript_copy_to_vp(c, 12, src, 0x10, 0x0030_6eec)?;
    let vp = c.g[S0][0];
    let (count, params) = (c.g[S1][0], c.g[S3][0]);
    wscript_fill_block(c, vp, count, params, 0x0044_6980, 5, 0x0030_6f08);

    // The gate check, then the tagged entry's flag.
    c.cycles += 9;
    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
    c.g[A0][0] = c.lw(gp.wrapping_add(0xce8));
    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
    Callee {
        addr: 0x0030_bfc0,
        f: ee::s10::f_0030bfc0,
    }
    .call(c, 0x0030_6f64)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[V0][0] = 0x1;
    let open = c.g[S1][0] == c.g[V0][0];
    c.g[A0][0] = c.g[S2][0];
    if open {
        let mut to_report = false;
        c.cycles += 10;
        c.g[V1][0] = c.lw(gp.wrapping_add(0x16c8));
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
        c.g[V0][0] = c.g[A1][0] & 0xff;
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        if c.g[V1][0] != 0 {
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
        } else {
            c.cycles += 2;
            c.g[S0][0] = 0;
            to_report = true;
        }
        if !to_report {
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
            let zero = c.g[V0][0] == 0;
            c.g[A1][0] = 0;
            if !zero {
                c.cycles += 1;
                c.g[A1][0] = sx((c.g[V0][0] as u32) << 2);
            }
            c.cycles += 1;
            c.g[S0][0] = c.g[A1][0];
        }
        // The flag word: call the two helpers when it is set.
        c.cycles += 2;
        let flagged = c.g[S0][0] != 0;
        c.g[V0][0] = 0x1;
        if flagged {
            c.cycles += 7;
            call_ptr(c, 0x0028_b180, 0x0030_6fd8)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = 0x5c;
            call_ptr(c, 0x0029_74a0, 0x0030_6fe8)?;
            c.g[V0][0] = 0x1;
        }
        c.cycles += 1;
        c.g[A0][0] = c.g[S2][0];
    }

    c.cycles += 11;
    c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    call_ptr(c, 0x0022_6610, 0x0030_6ff8)?;
    c.sw(c.g[V0][0] as u32, c.g[S1][0]);
    c.g[V0][0] = c.g[S2][0];
    c.lq(16, sp(c).wrapping_add(0x60));
    c.lq(17, sp(c).wrapping_add(0x50));
    c.lq(18, sp(c).wrapping_add(0x40));
    c.lq(19, sp(c).wrapping_add(0x30));
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x20));
    c.g[SP][0] = sx(sp(c).wrapping_add(0x70));
    ret(c);
    Ok(())
}

/// `WScriptFn_CareerCheck` (0x307020): fills the career-check block (the
/// same copy and argument loop), asks `func_001E3570` for the check result,
/// stores a 1 and the (result > 0) flag into the object, and returns it.
pub fn wscript_fn_career_check(c: &mut Cpu) {
    guest(c, career_check_body)
}

fn career_check_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    c.cycles += 12;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
    c.g[V0][0] = c.lw(gp.wrapping_add(0x27d0));
    c.sq(18, sp(c).wrapping_add(0x40));
    c.sq(17, sp(c).wrapping_add(0x50));
    c.g[S2][0] = c.g[A1][0];
    c.sq(19, sp(c).wrapping_add(0x30));
    c.g[S1][0] = c.g[A0][0];
    c.sq(16, sp(c).wrapping_add(0x60));
    c.g[S3][0] = c.g[A2][0];
    c.sd(sp(c).wrapping_add(0x20), c.g[RA][0]);
    let initialised = c.g[V0][0] != 0;
    c.g[A1][0] = 0x500000;
    if !initialised {
        c.cycles += 8;
        c.sw((c.g[A1][0] as u32).wrapping_sub(0x3f90), 0);
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3f90));
        c.g[V1][0] = 0x1;
        c.g[A0][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x8));
        c.sw(gp.wrapping_add(0x27d0), c.g[V1][0]);
        c.sb((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0);
        c.sb(sp(c), c.g[A0][0]);
    }
    let src = sx((c.g[A1][0] as u32).wrapping_sub(0x3f90));
    wscript_copy_to_vp(c, 7, src, 0xc, 0x0030_7084)?;
    let vp = c.g[S0][0];
    let (count, params) = (c.g[S2][0], c.g[S3][0]);
    wscript_fill_block(c, vp, count, params, gp.wrapping_add(0xce0), 3, 0x0030_7098);

    c.cycles += 17;
    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
    call_ptr(c, 0x001e_3570, 0x0030_70ec)?;
    c.g[V1][0] = 0x1;
    c.g[S0][0] = (0u64 < c.g[V0][0]) as u64;
    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x0022_6610, 0x0030_7100)?;
    c.sw(c.g[V0][0] as u32, c.g[S0][0]);
    c.g[V0][0] = c.g[S1][0];
    c.lq(16, sp(c).wrapping_add(0x60));
    c.lq(17, sp(c).wrapping_add(0x50));
    c.lq(18, sp(c).wrapping_add(0x40));
    c.lq(19, sp(c).wrapping_add(0x30));
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x20));
    c.g[SP][0] = sx(sp(c).wrapping_add(0x70));
    ret(c);
    Ok(())
}

/// `SphereTree_SortChildOrder` (0x32db40): builds the four child keys with
/// VU0 (`-1000` scaled by the sphere's centre), fills the order and distance
/// arrays for four children, then sorts the key/value pairs descending with
/// `ShellSortDesc_KeyVal` (0x32da40).
pub fn sphere_tree_sort_child_order(c: &mut Cpu) {
    guest(c, sort_child_order_body)
}

fn sort_child_order_body(c: &mut Cpu) -> Flow {
    c.cycles += 20;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
    c.g[AT][0] = 0xffff_ffff_c47a_0000;
    c.mtc1(1, 0);
    c.sd(sp(c).wrapping_add(0x40), c.g[RA][0]);
    c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x9c));
    c.mfc1(2, 0);
    c.lqc2(4, c.g[A1][0] as u32);
    c.qmtc2(2, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, sp(c).wrapping_add(0x30));
    c.g[T2][0] = 0;
    c.lq(2, sp(c).wrapping_add(0x30));
    c.g[A1][0] = c.g[A0][0];
    c.g[T4][0] = 0x7;
    c.g[T3][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb8));
    c.sq(2, sp(c).wrapping_add(0x20));
    c.g[T1][0] = c.g[A2][0];
    c.g[T0][0] = sx(sp(c).wrapping_add(0x1c));
    c.g[A3][0] = c.g[SP][0];
    loop {
        c.cycles += 25;
        c.lqc2(3, c.g[A1][0] as u32);
        c.lqc2(5, sp(c).wrapping_add(0x20));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(2, 4);
        c.mtc1(2, 0);
        c.g[V1][0] = sx((c.g[T4][0] as u32).wrapping_sub(c.g[T2][0] as u32));
        c.cvt_w_s(1, 0);
        c.mfc1(4, 1);
        c.sw(c.g[T1][0] as u32, c.g[T2][0]);
        c.sw(c.g[T3][0] as u32, c.g[V1][0]);
        c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
        c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x4));
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = sx(0u32.wrapping_sub(c.g[A0][0] as u32));
        c.sw(c.g[A3][0] as u32, c.g[A0][0]);
        c.sw(c.g[T0][0] as u32, c.g[V0][0]);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4));
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x4));
        c.g[V0][0] = ((c.g[T2][0] as i64) < 4i64) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0032_db90);
        }
    }
    c.cycles += 6;
    c.g[A0][0] = 0x8;
    c.g[A1][0] = c.g[SP][0];
    Callee {
        addr: 0x0032_da40,
        f: ee::s11::f_0032da40,
    }
    .call(c, 0x0032_dc00)?;
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x40));
    c.g[SP][0] = sx(sp(c).wrapping_add(0x50));
    ret(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, Rng, DATA, GAME_GP};

    // Fixture objects, all in DATA+0x200000 and up (clear of the fake log).
    const FX_WORLD: u32 = DATA + 0x20_0000;
    const FX_BE: u32 = DATA + 0x21_0000;
    const FX_OBJ: u32 = DATA + 0x22_0000;
    const FX_VT: u32 = DATA + 0x23_0000;
    const FX_MAN: u32 = DATA + 0x24_0000;
    const FX_PARAMS: u32 = DATA + 0x25_0000;
    const FX_SRET: u32 = DATA + 0x26_0000;
    const FX_TAGS: u32 = DATA + 0x27_0000;
    const FX_TAGS2: u32 = DATA + 0x27_1000;
    const FX_TAGTBL: u32 = DATA + 0x28_0000;
    const FX_ENTRY: u32 = DATA + 0x29_0000;
    const FX_ENTRY2: u32 = DATA + 0x2a_0000;
    const FX_PREV: u32 = DATA + 0x2b_0000;
    const FX_SORT: u32 = DATA + 0x30_0000;
    const FX_SORT_GATE: u32 = DATA + 0x31_0000;
    const FX_SORT_MAGIC: u32 = 0x5027_0706;

    // ShellSortDesc_KeyVal is bounded only by its input: the gated fake logs the
    // call and returns, the generated body runs otherwise.
    fn oc706_sort_fake(c: &mut Cpu) {
        if c.bus.read32(FX_SORT_GATE) != FX_SORT_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc706_sort_fake),
                ee::s11::f_0032da40__generated,
            );
        }
        fakes::fake_call(c, 0x0032_da40);
    }

    fn oc706_sort_install() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s11::OVERRIDE_0032DA40),
            oc706_sort_fake,
            fakes::prev_cell(oc706_sort_fake),
        );
    }

    /// The globals and objects the WScript bodies read, varied by `seed`:
    /// the BE singleton, the world and its object and manager chain, the
    /// zone-tag tables, the guard flag at `gp+guard`, the kind table at
    /// `table`, and four small parameter records.
    fn wscript_fixture(c: &mut Cpu, seed: u32, salt: u64, guard: u32, table: u32) {
        let mut r = machine(c, seed, salt, 4);
        let gp = GAME_GP;
        c.bus.write32(gp.wrapping_sub(0x1f04), FX_BE);
        c.bus.write32(gp.wrapping_sub(0x848), FX_WORLD);
        c.bus.write32(FX_WORLD + 0x78, FX_BE);
        c.bus.write32(FX_WORLD + 0x84, FX_OBJ);
        c.bus.write32(FX_OBJ + 0xc, FX_VT);
        c.bus.write32(FX_VT + 0xa8, FX_MAN);
        // bIsNetworkGame: odd seeds skip func_001E3510.
        c.bus.write32(0x0053_4b30, seed & 1);
        c.bus.write32(gp.wrapping_add(0x16c8), FX_TAGS);
        c.bus.write32(FX_TAGS, FX_TAGS2);
        c.bus.write32(FX_TAGS2 + 0x8, FX_TAGTBL);
        for i in 0..256u32 {
            let e = if r.next_u32() % 4 == 0 { 0 } else { FX_ENTRY };
            c.bus.write32(FX_TAGTBL + 4 * i, e);
        }
        c.bus.write32(FX_ENTRY + 0x1c, FX_ENTRY2);
        for i in 0..0x100u32 {
            c.bus.write32(FX_ENTRY2 + 4 * i, (r.next_u32() & 0xff) << 8);
        }
        c.bus.write32(
            FX_PREV + 0x290,
            if r.next_u32() % 3 == 0 { 0 } else { FX_OBJ },
        );
        c.bus.write32(gp.wrapping_add(0xce8), FX_PREV);
        c.bus.write32(gp.wrapping_add(guard), r.next_u32() & 1);
        for k in 0..4u32 {
            c.bus.write32(table + 4 * k, r.next_u32() % 3);
        }
        for i in 0..4u32 {
            let p = FX_PARAMS + 16 * i;
            c.bus.write32(p, r.next_u32() % 4);
            let v = if r.next_u32() % 5 == 0 {
                u32::MAX
            } else {
                r.next_u32() & 0xffff
            };
            c.bus.write32(p + 0x4, v);
            c.bus.write32(p + 0xc, r.next_u32() % 3);
        }
    }

    /// Arguments of the four-argument WScript bodies: sret, count (0..4 and
    /// -1), and the parameter array.
    fn wscript_args(c: &mut Cpu, seed: u32, sret: u32) {
        let count = [0u64, 1, 2, 3, 4, u64::MAX][(seed % 6) as usize];
        c.g[A0][0] = sx(sret);
        c.g[A1][0] = count;
        c.g[A2][0] = sx(FX_PARAMS);
    }

    #[test]
    fn wscript_fn_career_action_matches() {
        check(
            ee::s10::f_00306300__generated,
            wscript_fn_career_action,
            200,
            &|c, seed| {
                wscript_fixture(c, seed, 0x0030_6300, 0x279c, GAME_GP.wrapping_add(0xcc0));
                wscript_args(c, seed, FX_SRET);
                c.pc = 0x0030_6300;
            },
        );
    }

    #[test]
    fn wscript_fn_add_danger_matches() {
        check(
            ee::s10::f_00306438__generated,
            wscript_fn_add_danger,
            200,
            &|c, seed| {
                wscript_fixture(c, seed, 0x0030_6438, 0x27a0, 0x0044_6880);
                wscript_args(c, seed, FX_SRET);
                c.pc = 0x0030_6438;
            },
        );
    }

    #[test]
    fn wscript_fn_remove_danger_matches() {
        check(
            ee::s10::f_003065e8__generated,
            wscript_fn_remove_danger,
            200,
            &|c, seed| {
                wscript_fixture(c, seed, 0x0030_65e8, 0x27b0, GAME_GP.wrapping_add(0xcc8));
                wscript_args(c, seed, FX_SRET);
                c.pc = 0x0030_65e8;
            },
        );
    }

    #[test]
    fn wscript_fn_acquire_danger_zone_matches() {
        check(
            ee::s10::f_00306770__generated,
            wscript_fn_acquire_danger_zone,
            200,
            &|c, seed| {
                wscript_fixture(c, seed, 0x0030_6770, 0x27b4, GAME_GP.wrapping_add(0xcd0));
                wscript_args(c, seed, FX_SRET);
                c.pc = 0x0030_6770;
            },
        );
    }

    #[test]
    fn wscript_fn_release_danger_zone_matches() {
        check(
            ee::s10::f_00306908__generated,
            wscript_fn_release_danger_zone,
            200,
            &|c, seed| {
                wscript_fixture(c, seed, 0x0030_6908, 0x27c0, GAME_GP.wrapping_add(0xcd8));
                wscript_args(c, seed, FX_SRET);
                c.pc = 0x0030_6908;
            },
        );
    }

    #[test]
    fn wscript_fn_check_rider_gate_matches() {
        check(
            ee::s10::f_00306e68__generated,
            wscript_fn_check_rider_gate,
            200,
            &|c, seed| {
                wscript_fixture(c, seed, 0x0030_6e68, 0x27cc, 0x0044_6980);
                wscript_args(c, seed, FX_SRET);
                c.pc = 0x0030_6e68;
            },
        );
    }

    #[test]
    fn wscript_fn_career_check_matches() {
        check(
            ee::s10::f_00307020__generated,
            wscript_fn_career_check,
            200,
            &|c, seed| {
                wscript_fixture(c, seed, 0x0030_7020, 0x27d0, GAME_GP.wrapping_add(0xce0));
                wscript_args(c, seed, FX_SRET);
                c.pc = 0x0030_7020;
            },
        );
    }

    #[test]
    fn sphere_tree_sort_child_order_matches() {
        check(
            ee::s11::f_0032db40__generated,
            sphere_tree_sort_child_order,
            200,
            &|c, seed| {
                oc706_sort_install();
                c.bus.write32(FX_SORT_GATE, FX_SORT_MAGIC);
                let mut r = Rng(0x0032_db40 ^ seed as u64);
                for i in 0..0x60u32 {
                    c.bus.write32(FX_SORT + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(FX_SORT);
                c.g[A1][0] = sx(FX_SORT + 0x40);
                c.pc = 0x0032_db40;
            },
        );
    }
}
