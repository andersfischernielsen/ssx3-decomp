//! Staging module oc732 (wave wave241): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc732`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Flow};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0030_0948,
        wscript_fn_place_rider_at_object,
        "WScriptFn_PlaceRiderAtObject",
    ),
    (
        0x0030_0b20,
        wscript_fn_set_rail_enabled,
        "WScriptFn_SetRailEnabled",
    ),
    (
        0x0030_0c78,
        wscript_fn_play_script_list,
        "WScriptFn_PlayScriptList",
    ),
    (0x0030_1440, wscript_fn_nop_1440, "WScriptFn_Nop_1440"),
    (
        0x0030_1560,
        wscript_fn_get_collected_count,
        "WScriptFn_GetCollectedCount",
    ),
    (
        0x0030_1830,
        wscript_fn_get_object_slot31,
        "WScriptFn_GetObjectSlot31",
    ),
    (0x0030_1b88, wscript_fn_jump_to_task, "WScriptFn_JumpToTask"),
    (0x0030_1c80, wscript_fn_goto_task, "WScriptFn_GotoTask"),
];

/// The 0x70-byte WScript frame and prologue shared by these eight bodies:
/// saves s0..s3/ra, moves the three arguments into s1/s2/s3 (`regs`: the
/// argument register each of s1, s2, s3 takes; the bodies differ in order),
/// loads the once-init guard at `gp+guard`, and (`set_a2`) puts 0x500000 in
/// a2 as the static-block base. Returns whether the guard was already set.
fn oc732_prologue(c: &mut Cpu, cycles: u64, guard: u32, regs: [usize; 3], set_a2: bool) -> bool {
    let gp = c.g[GP][0] as u32;
    c.cycles += cycles;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
    c.g[V0][0] = c.lw(gp.wrapping_add(guard));
    c.sq(17, sp(c).wrapping_add(0x50));
    c.sq(18, sp(c).wrapping_add(0x40));
    c.g[S1][0] = c.g[regs[0]][0];
    c.sq(19, sp(c).wrapping_add(0x30));
    c.g[S2][0] = c.g[regs[1]][0];
    c.sq(16, sp(c).wrapping_add(0x60));
    c.g[S3][0] = c.g[regs[2]][0];
    let initialised = c.g[V0][0] != 0;
    c.sd(sp(c).wrapping_add(0x20), c.g[RA][0]);
    if set_a2 {
        c.g[A2][0] = 0x500000;
    }
    initialised
}

/// `MEM_copy(vp, src, len)` with `vp = sp + 0x10` (s0), then the
/// `(params, count)` fill loop of the block. `prefill` is the variant that
/// first stores -1 at `vp` (v0 = -1).
#[allow(clippy::too_many_arguments)]
fn oc732_copy_fill(
    c: &mut Cpu,
    cycles: u64,
    prefill: bool,
    src: u64,
    len: u64,
    ret_addr: u32,
    regs: (usize, usize),
    table: u32,
    entry: u64,
    top: u32,
) -> Flow {
    c.cycles += cycles;
    if prefill {
        c.g[V0][0] = u64::MAX;
    }
    c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A2][0] = len;
    c.g[A1][0] = src;
    c.g[A0][0] = c.g[S0][0];
    if prefill {
        c.sw(sp(c).wrapping_add(0x10), c.g[V0][0]);
    }
    call_ptr(c, 0x003e_6574, ret_addr)?;
    let (vp, count, params) = (c.g[S0][0], c.g[regs.0][0], c.g[regs.1][0]);
    oc732_fill_block(c, vp, count, params, table, entry, top);
    Ok(())
}

/// The `(params, n)` loop of the WScript bodies: each argument is stored into
/// the block at `vp` by its index, as a float when the kind table says float
/// and the argument's kind differs, else as an int. `entry` is the cycle
/// charge of the loop's entry block, `top` its back-edge head.
fn oc732_fill_block(
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

/// The zone-tag lookup of the danger/slot wrappers, from `vp` (the x word):
/// the tag table at `gp+0x16c8` maps x (low byte, then the next bits) to a
/// small index in v0; with x = -1, v0 is `*(gp+0xce8)->0x290`. Leaves a1, a2,
/// v0, v1, a0 as the generated block does. `x_cycles` is the block charge of
/// the x = -1 path (4 in 0948, 3 in the slot getter).
fn oc732_zone_lookup(c: &mut Cpu, vp: u64, x_cycles: u64) -> bool {
    let gp = c.g[GP][0] as u32;
    c.cycles += 4;
    c.g[A2][0] = c.lw(vp as u32);
    c.g[V0][0] = !(0u64 | c.g[A2][0]);
    let tagged = c.g[V0][0] != 0;
    c.g[A1][0] = c.lw(gp.wrapping_add(0x16c8));
    if !tagged {
        c.cycles += x_cycles;
        c.g[V0][0] = c.lw(gp.wrapping_add(0xce8));
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

/// The `luno_gLunoNil` struct return (12 bytes through `sret`), verbatim.
fn oc732_return_nil(c: &mut Cpu, sret: u64) {
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

/// Restore s0..s3, ra and the 0x70 frame, return.
fn oc732_epilogue(c: &mut Cpu) {
    c.lq(16, sp(c).wrapping_add(0x60));
    c.lq(17, sp(c).wrapping_add(0x50));
    c.lq(18, sp(c).wrapping_add(0x40));
    c.lq(19, sp(c).wrapping_add(0x30));
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x20));
    c.g[SP][0] = sx(sp(c).wrapping_add(0x70));
    ret(c);
}

/// `WScriptFn_PlaceRiderAtObject` (0x300948): fills an 8-byte block (x), looks
/// up the tagged object for x (x = -1: the current one, `*(gp+0xce8)->0x290`),
/// and when the rider index at `*(gp+0xce8)` is not negative calls the
/// object's vtable method and the world's rider (`world->0x84->0xc[idx]`)
/// vtable method with it. Returns `luno_gLunoNil` through sret (a0).
pub fn wscript_fn_place_rider_at_object(c: &mut Cpu) {
    guest(c, place_rider_body)
}

fn place_rider_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    let initialised = oc732_prologue(c, 11, 0x25b0, [A1, A2, A0], false);
    if !initialised {
        c.cycles += 7;
        c.g[V0][0] = c.lbu(gp.wrapping_add(0x25ac));
        c.g[V1][0] = u64::MAX;
        c.g[A0][0] = 0x1;
        c.sw(gp.wrapping_add(0x25a8), c.g[V1][0]);
        c.sw(gp.wrapping_add(0x25b0), c.g[A0][0]);
        c.sb(sp(c), c.g[V0][0]);
    }
    let src = sx(gp.wrapping_add(0x25a8));
    oc732_copy_fill(
        c,
        10,
        true,
        src,
        0x8,
        0x0030_09b0,
        (S1, S2),
        gp.wrapping_add(0xb58),
        4,
        0x0030_09c8,
    )?;

    let vp = c.g[S0][0];
    let tagged = oc732_zone_lookup(c, vp, 4);
    if !tagged {
        // x == -1: a2 = v0 = *(gp+0xce8); v0 = a2->0x290.
        c.g[A2][0] = c.g[V0][0];
        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
    } else {
        c.cycles += 1;
        c.g[A2][0] = c.lw(gp.wrapping_add(0xce8));
    }

    c.cycles += 3;
    c.g[S0][0] = c.lw(c.g[A2][0] as u32);
    let negative = (c.g[S0][0] as i64) < 0;
    c.g[V1][0] = 0x4d0000;
    let mut skip_move = false;
    if !negative {
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = c.g[S3][0];
            skip_move = true;
        } else {
            c.cycles += 3;
            c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            let no_obj = c.g[A2][0] == 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
            if !no_obj {
                c.cycles += 5;
                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0030_0aa8)?;
            }
            c.cycles += 14;
            c.g[A0][0] = c.lw(gp.wrapping_sub(0x848));
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = sx((c.g[S0][0] as u32) << 2);
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x60));
            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x64));
            let target = c.g[A2][0] as u32;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            call_ptr(c, target, 0x0030_0adc)?;
            c.g[V1][0] = 0x4d0000;
        }
    }
    if !skip_move {
        c.cycles += 1;
        c.g[V0][0] = c.g[S3][0];
    }

    c.cycles += 14;
    let sret = c.g[S3][0];
    oc732_return_nil(c, sret);
    oc732_epilogue(c);
    Ok(())
}

/// `WScriptFn_SetRailEnabled` (0x300b20): fills a 12-byte block (x, mode) and,
/// when x is not -1, `RailMan_SetRailEnabled(x, 1)` for mode 1, or
/// `(x, 0)` for mode 0 (other modes do nothing). Returns `luno_gLunoNil`.
pub fn wscript_fn_set_rail_enabled(c: &mut Cpu) {
    guest(c, set_rail_enabled_body)
}

fn set_rail_enabled_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    let initialised = oc732_prologue(c, 12, 0x25b4, [A1, A2, A0], true);
    if !initialised {
        c.cycles += 10;
        c.g[V1][0] = sx(0x0050_0000u32.wrapping_sub(0x44b8));
        c.g[V0][0] = u64::MAX;
        c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x8));
        c.sw(0x0050_0000u32.wrapping_sub(0x44b8), c.g[V0][0]);
        c.g[A0][0] = 0x1;
        c.sb((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.sw(gp.wrapping_add(0x25b4), c.g[A0][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A0][0]);
        c.sb(sp(c), c.g[A1][0]);
    }
    let src = sx(0x0050_0000u32.wrapping_sub(0x44b8));
    oc732_copy_fill(
        c,
        10,
        true,
        src,
        0xc,
        0x0030_0b98,
        (S1, S2),
        gp.wrapping_add(0xb60),
        4,
        0x0030_0bb0,
    )?;

    c.cycles += 4;
    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
    c.g[V0][0] = !(0u64 | c.g[A0][0]);
    let rail = c.g[V0][0] == 0;
    c.g[V0][0] = 0x1;
    if rail {
        // x == -1: nothing to set.
        c.cycles += 1;
        c.g[V1][0] = 0x4d0000;
    } else {
        c.cycles += 3;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        if c.g[V1][0] == c.g[V0][0] {
            c.cycles += 4;
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0035_8b28, 0x0030_0c1c)?;
            c.g[V1][0] = 0x4d0000;
        } else {
            c.cycles += 2;
            let other = c.g[V1][0] != 0;
            c.g[V1][0] = 0x4d0000;
            if !other {
                c.cycles += 2;
                c.g[A1][0] = 0;
                call_ptr(c, 0x0035_8b28, 0x0030_0c34)?;
                c.cycles += 1;
                c.g[V1][0] = 0x4d0000;
            }
        }
    }

    c.cycles += 15;
    c.g[V0][0] = c.g[S3][0];
    let sret = c.g[S3][0];
    oc732_return_nil(c, sret);
    oc732_epilogue(c);
    Ok(())
}

/// `WScriptFn_PlayScriptList` (0x300c78): fills a 12-byte block (x, mode) and
/// drives the script engine's list (`gp-0x84c`): mode 0 adds a movie
/// (`ListAddMovieById`) and prepares, mode 1 adds a choice
/// (`ListAddChoice`) and prepares, mode 2 starts the list; other modes do
/// nothing. Returns `luno_gLunoNil`.
pub fn wscript_fn_play_script_list(c: &mut Cpu) {
    guest(c, play_script_list_body)
}

fn play_script_list_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    let initialised = oc732_prologue(c, 12, 0x25b8, [A1, A2, A0], true);
    if !initialised {
        c.cycles += 10;
        c.g[V1][0] = sx(0x0050_0000u32.wrapping_sub(0x44a8));
        c.g[V0][0] = u64::MAX;
        c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x8));
        c.sw(0x0050_0000u32.wrapping_sub(0x44a8), c.g[V0][0]);
        c.g[A0][0] = 0x1;
        c.sb((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.sw(gp.wrapping_add(0x25b8), c.g[A0][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0);
        c.sb(sp(c), c.g[A1][0]);
    }
    let src = sx(0x0050_0000u32.wrapping_sub(0x44a8));
    oc732_copy_fill(
        c,
        7,
        false,
        src,
        0xc,
        0x0030_0ce4,
        (S1, S2),
        gp.wrapping_add(0xb68),
        3,
        0x0030_0cf8,
    )?;

    c.cycles += 4;
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = 0x1;
    let mode1 = c.g[V1][0] == c.g[V0][0];
    c.g[A2][0] = c.lw(c.g[S0][0] as u32);
    // 0: add movie + prepare; 1: add choice + prepare; 2: start; 3: nothing.
    let step;
    if mode1 {
        step = 1;
    } else {
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
        let ge2 = c.g[V0][0] == 0;
        c.g[V0][0] = 0x2;
        if !ge2 {
            c.cycles += 2;
            let zero = c.g[V1][0] == 0;
            c.g[V1][0] = 0x4d0000;
            if zero {
                step = 0;
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.g[S3][0];
                step = 3;
            }
        } else {
            c.cycles += 2;
            let two = c.g[V1][0] == c.g[V0][0];
            c.g[V1][0] = 0x4d0000;
            if two {
                step = 2;
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.g[S3][0];
                step = 3;
            }
        }
    }
    match step {
        0 | 1 => {
            c.cycles += 9;
            c.g[A0][0] = c.lw(gp.wrapping_sub(0x84c));
            c.g[A1][0] = 0;
            c.g[A3][0] = 0x3;
            if step == 0 {
                c.g[T0][0] = u64::MAX;
                c.g[T1][0] = u64::MAX;
                c.g[T2][0] = u64::MAX;
                call_ptr(c, 0x0027_8ed0, 0x0030_0d98)?;
            } else {
                c.g[T0][0] = 0;
                c.g[T1][0] = u64::MAX;
                c.g[T2][0] = u64::MAX;
                c.g[T3][0] = u64::MAX;
                call_ptr(c, 0x0027_8e50, 0x0030_0dc0)?;
            }
            c.g[A0][0] = c.lw(gp.wrapping_sub(0x84c));
            c.cycles += 4;
            c.g[A1][0] = 0;
            call_ptr(c, 0x0027_8f38, 0x0030_0dcc)?;
            c.g[V1][0] = 0x4d0000;
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
        }
        2 => {
            c.cycles += 6;
            c.g[A0][0] = c.lw(gp.wrapping_sub(0x84c));
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x1;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0027_8f68, 0x0030_0de8)?;
            c.g[V1][0] = 0x4d0000;
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
        }
        _ => {}
    }

    c.cycles += 14;
    let sret = c.g[S3][0];
    oc732_return_nil(c, sret);
    oc732_epilogue(c);
    Ok(())
}

/// `WScriptFn_Nop_1440` (0x301440): fills a 12-byte block (x, a flag) from the
/// argument list and does nothing with it (a retired script command). Returns
/// `luno_gLunoNil`.
pub fn wscript_fn_nop_1440(c: &mut Cpu) {
    guest(c, nop_1440_body)
}

fn nop_1440_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    let initialised = oc732_prologue(c, 12, 0x25cc, [A1, A2, A0], true);
    if !initialised {
        c.cycles += 10;
        c.g[V1][0] = sx(0x0050_0000u32.wrapping_sub(0x4410));
        c.g[V0][0] = u64::MAX;
        c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x8));
        c.sw(0x0050_0000u32.wrapping_sub(0x4410), c.g[V0][0]);
        c.g[A0][0] = 0x1;
        c.sb((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.sw(gp.wrapping_add(0x25cc), c.g[A0][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A0][0]);
        c.sb(sp(c), c.g[A1][0]);
    }
    let src = sx(0x0050_0000u32.wrapping_sub(0x4410));
    oc732_copy_fill(
        c,
        10,
        true,
        src,
        0xc,
        0x0030_14b8,
        (S1, S2),
        gp.wrapping_add(0xb88),
        4,
        0x0030_14d0,
    )?;

    c.cycles += 18;
    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
    c.g[V0][0] = !(0u64 | c.g[A0][0]);
    let sret = c.g[S3][0];
    oc732_return_nil(c, sret);
    oc732_epilogue(c);
    Ok(())
}

/// `WScriptFn_GetCollectedCount` (0x301560): fills an 8-byte block (id) and
/// returns, as an int `LunoVMRegister` in a0, the manager's collected count
/// for the id (`WScriptMan_GetCollectedCount(gp+0xce8, id)`), or 0 for id -1.
pub fn wscript_fn_get_collected_count(c: &mut Cpu) {
    guest(c, get_collected_count_body)
}

fn get_collected_count_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    let initialised = oc732_prologue(c, 11, 0x25d8, [A0, A1, A2], false);
    if !initialised {
        c.cycles += 7;
        c.g[V0][0] = c.lbu(gp.wrapping_add(0x25d4));
        c.g[V1][0] = u64::MAX;
        c.g[A0][0] = 0x1;
        c.sw(gp.wrapping_add(0x25d0), c.g[V1][0]);
        c.sw(gp.wrapping_add(0x25d8), c.g[A0][0]);
        c.sb(sp(c), c.g[V0][0]);
    }
    let src = sx(gp.wrapping_add(0x25d0));
    oc732_copy_fill(
        c,
        7,
        false,
        src,
        0x8,
        0x0030_15bc,
        (S2, S3),
        gp.wrapping_add(0xb90),
        3,
        0x0030_15d0,
    )?;

    c.cycles += 5;
    c.g[V0][0] = u64::MAX;
    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
    let no_id = c.g[A1][0] == c.g[V0][0];
    c.g[V0][0] = 0x1;
    if no_id {
        c.cycles += 4;
        c.g[A0][0] = c.g[S1][0];
        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0022_6610, 0x0030_165c)?;
        c.sw(c.g[V0][0] as u32, 0);
    } else {
        c.cycles += 9;
        c.g[A0][0] = c.lw(gp.wrapping_add(0xce8));
        call_ptr(c, 0x0030_ba80, 0x0030_1634)?;
        c.g[V1][0] = 0x1;
        c.g[S0][0] = c.g[V0][0];
        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0022_6610, 0x0030_1648)?;
        c.sw(c.g[V0][0] as u32, c.g[S0][0]);
    }
    c.cycles += 8;
    c.g[V0][0] = c.g[S1][0];
    oc732_epilogue(c);
    Ok(())
}

/// `WScriptFn_GetObjectSlot31` (0x301830): fills an 8-byte block (x), looks up
/// the tagged object for x (x = -1: `*(gp+0xce8)->0x290`) and returns, as an
/// int `LunoVMRegister` in a0, the result of the object's vtable method at
/// `+0xf8` (0 when the object has no sub-object), or 1 when there is no
/// object at all.
pub fn wscript_fn_get_object_slot31(c: &mut Cpu) {
    guest(c, get_object_slot31_body)
}

fn get_object_slot31_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    let initialised = oc732_prologue(c, 11, 0x25f8, [A1, A0, A2], false);
    if !initialised {
        c.cycles += 7;
        c.g[V0][0] = c.lbu(gp.wrapping_add(0x25f4));
        c.g[V1][0] = u64::MAX;
        c.g[A0][0] = 0x1;
        c.sw(gp.wrapping_add(0x25f0), c.g[V1][0]);
        c.sw(gp.wrapping_add(0x25f8), c.g[A0][0]);
        c.sb(sp(c), c.g[V0][0]);
    }
    let src = sx(gp.wrapping_add(0x25f0));
    oc732_copy_fill(
        c,
        10,
        true,
        src,
        0x8,
        0x0030_1898,
        (S1, S3),
        gp.wrapping_add(0xba0),
        4,
        0x0030_18b0,
    )?;

    let vp = c.g[S0][0];
    let tagged = oc732_zone_lookup(c, vp, 3);
    if !tagged {
        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
    }

    c.cycles += 2;
    if c.g[V0][0] == 0 {
        // No object: the result is 1... (the type word stays set to v0 = 1).
        c.g[V0][0] = 0x1;
        c.cycles += 4;
        c.g[A0][0] = c.g[S2][0];
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0022_6610, 0x0030_19a4)?;
        c.sw(c.g[V0][0] as u32, 0);
    } else {
        c.cycles += 3;
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        let no_sub = c.g[A1][0] == 0;
        c.g[S0][0] = 0;
        if !no_sub {
            c.cycles += 6;
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xf8));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xfc));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0030_197c)?;
            c.g[S0][0] = c.g[V0][0];
        }
        c.cycles += 6;
        c.g[V0][0] = 0x1;
        c.g[A0][0] = c.g[S2][0];
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0022_6610, 0x0030_1990)?;
        c.sw(c.g[V0][0] as u32, c.g[S0][0]);
    }
    c.cycles += 8;
    c.g[V0][0] = c.g[S2][0];
    oc732_epilogue(c);
    Ok(())
}

/// Shared body of `JumpToTask` and `GotoTask`: an 8-byte block (task id) at
/// `gp+blk`, then `WScriptMan_<JumpTo|Goto>Task(gp+0xce8, id)` and an int
/// result register (0) in a0. All addresses are parameters.
struct Oc732Task {
    guard: u32,
    block: u32,
    table: u32,
    copy_ret: u32,
    top: u32,
    man_fn: u32,
    man_ret: u32,
    as_int_ret: u32,
}

fn oc732_task_body(c: &mut Cpu, p: Oc732Task) -> Flow {
    let gp = c.g[GP][0] as u32;
    let initialised = oc732_prologue(c, 11, p.guard, [A0, A1, A2], false);
    if !initialised {
        c.cycles += 7;
        c.g[V0][0] = c.lbu(gp.wrapping_add(p.block + 4));
        c.g[V1][0] = u64::MAX;
        c.g[A0][0] = 0x1;
        c.sw(gp.wrapping_add(p.block), c.g[V1][0]);
        c.sw(gp.wrapping_add(p.guard), c.g[A0][0]);
        c.sb(sp(c), c.g[V0][0]);
    }
    let src = sx(gp.wrapping_add(p.block));
    oc732_copy_fill(
        c,
        7,
        false,
        src,
        0x8,
        p.copy_ret,
        (S2, S3),
        gp.wrapping_add(p.table),
        3,
        p.top,
    )?;

    c.cycles += 16;
    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
    c.g[A0][0] = c.lw(gp.wrapping_add(0xce8));
    call_ptr(c, p.man_fn, p.man_ret)?;
    c.g[V0][0] = 0x1;
    c.g[A0][0] = c.g[S1][0];
    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    call_ptr(c, 0x0022_6610, p.as_int_ret)?;
    c.sw(c.g[V0][0] as u32, 0);
    c.g[V0][0] = c.g[S1][0];
    oc732_epilogue(c);
    Ok(())
}

/// `WScriptFn_JumpToTask` (0x301b88): fills an 8-byte block (task id) and
/// calls `WScriptMan_JumpToTask(gp+0xce8, id)`; returns an int register (0).
pub fn wscript_fn_jump_to_task(c: &mut Cpu) {
    guest(c, |c| {
        oc732_task_body(
            c,
            Oc732Task {
                guard: 0x2608,
                block: 0x2600,
                table: 0xbb0,
                copy_ret: 0x0030_1be4,
                top: 0x0030_1bf8,
                man_fn: 0x0030_a6d8,
                man_ret: 0x0030_1c4c,
                as_int_ret: 0x0030_1c5c,
            },
        )
    })
}

/// `WScriptFn_GotoTask` (0x301c80): the `GotoTask` twin of the above
/// (`WScriptMan_GotoTask`).
pub fn wscript_fn_goto_task(c: &mut Cpu) {
    guest(c, |c| {
        oc732_task_body(
            c,
            Oc732Task {
                guard: 0x2618,
                block: 0x2610,
                table: 0xbb8,
                copy_ret: 0x0030_1cdc,
                top: 0x0030_1cf0,
                man_fn: 0x0030_a700,
                man_ret: 0x0030_1d44,
                as_int_ret: 0x0030_1d54,
            },
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;
    use crate::difftest::{check, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    // Fixture objects, all in DATA+0x200000 and up (clear of the fake log).
    const FX_CTRL: u32 = DATA + 0x20_0000;
    const FX_MAGIC: u32 = 0x4f43_3733;
    const FX_WORLD: u32 = DATA + 0x21_0000;
    const FX_OBJ: u32 = DATA + 0x22_0000;
    const FX_ARR: u32 = DATA + 0x23_0000;
    const FX_RIDER: u32 = DATA + 0x24_0000;
    const FX_VT2: u32 = DATA + 0x25_0000;
    const FX_PARAMS: u32 = DATA + 0x26_0000;
    const FX_SRET: u32 = DATA + 0x27_0000;
    const FX_TAGS: u32 = DATA + 0x28_0000;
    const FX_TAGS2: u32 = DATA + 0x28_1000;
    const FX_TAGTBL: u32 = DATA + 0x29_0000;
    const FX_ENTRY: u32 = DATA + 0x2a_0000;
    const FX_ENTRY2: u32 = DATA + 0x2b_0000;
    const FX_PREV: u32 = DATA + 0x2c_0000;
    const FX_OBJ2: u32 = DATA + 0x2d_0000;
    const FX_EOBJ: u32 = DATA + 0x2e_0000;
    const FX_EVT: u32 = DATA + 0x2f_0000;
    const FX_ENGINE: u32 = DATA + 0x30_0000;

    macro_rules! oc732_fake {
        ($fake:ident, $prev:ident, $gen:path, $addr:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(FX_CTRL) != FX_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $addr);
            }
        };
    }

    oc732_fake!(
        oc732_fake_as_int,
        OC732_PREV_AS_INT,
        ee::s05::f_00226610__generated,
        0x0022_6610
    );
    oc732_fake!(
        oc732_fake_rail,
        OC732_PREV_RAIL,
        ee::s11::f_00358b28__generated,
        0x0035_8b28
    );
    oc732_fake!(
        oc732_fake_movie,
        OC732_PREV_MOVIE,
        ee::s07::f_00278ed0__generated,
        0x0027_8ed0
    );
    oc732_fake!(
        oc732_fake_choice,
        OC732_PREV_CHOICE,
        ee::s07::f_00278e50__generated,
        0x0027_8e50
    );
    oc732_fake!(
        oc732_fake_prepare,
        OC732_PREV_PREPARE,
        ee::s07::f_00278f38__generated,
        0x0027_8f38
    );
    oc732_fake!(
        oc732_fake_start,
        OC732_PREV_START,
        ee::s07::f_00278f68__generated,
        0x0027_8f68
    );
    oc732_fake!(
        oc732_fake_collected,
        OC732_PREV_COLLECTED,
        ee::s10::f_0030ba80__generated,
        0x0030_ba80
    );
    oc732_fake!(
        oc732_fake_jump,
        OC732_PREV_JUMP,
        ee::s10::f_0030a6d8__generated,
        0x0030_a6d8
    );
    oc732_fake!(
        oc732_fake_goto,
        OC732_PREV_GOTO,
        ee::s10::f_0030a700__generated,
        0x0030_a700
    );

    fn oc732_install_fakes() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_00226610),
            oc732_fake_as_int,
            &OC732_PREV_AS_INT,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00358B28),
            oc732_fake_rail,
            &OC732_PREV_RAIL,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00278ED0),
            oc732_fake_movie,
            &OC732_PREV_MOVIE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00278E50),
            oc732_fake_choice,
            &OC732_PREV_CHOICE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00278F38),
            oc732_fake_prepare,
            &OC732_PREV_PREPARE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00278F68),
            oc732_fake_start,
            &OC732_PREV_START,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0030BA80),
            oc732_fake_collected,
            &OC732_PREV_COLLECTED,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0030A6D8),
            oc732_fake_jump,
            &OC732_PREV_JUMP,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0030A700),
            oc732_fake_goto,
            &OC732_PREV_GOTO,
        );
    }

    /// The globals and objects the WScript bodies read, varied by `seed`: the
    /// world and its rider array, the zone-tag tables, the current-object
    /// record at `gp+0xce8` (rider index, tagged object and its vtable chain),
    /// the script engine pointer, the once-init guard at `gp+guard`, the kind
    /// table at `table`, and four small argument records.
    fn oc732_fixture(c: &mut Cpu, seed: u32, salt: u64, guard: u32, table: u32) {
        let mut r = machine(c, seed, salt, 4);
        let gp = GAME_GP;
        c.bus.write32(FX_CTRL, FX_MAGIC);
        c.bus.write32(gp.wrapping_sub(0x848), FX_WORLD);
        c.bus.write32(gp.wrapping_sub(0x84c), FX_ENGINE);
        c.bus.write32(FX_WORLD + 0x84, FX_OBJ);
        c.bus.write32(FX_OBJ + 0xc, FX_ARR);
        for i in 0..8u32 {
            c.bus.write32(FX_ARR + 0x28 + 4 * i, FX_RIDER);
        }
        c.bus.write32(FX_RIDER + 0x6c0, FX_VT2);
        c.bus
            .write16(FX_VT2 + 0x60, (r.next_u32() as i16 as i32 & !3) as u16);
        c.bus.write32(FX_VT2 + 0x64, fakes::FAKE + 0x10);
        // The tag tables.
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
        // The current-object record: rider index, tagged object chain.
        c.bus.write32(gp.wrapping_add(0xce8), FX_PREV);
        let idx = [u32::MAX, 0, 1, 2, 3, 5][(r.next_u32() % 6) as usize];
        c.bus.write32(FX_PREV, idx);
        c.bus.write32(
            FX_PREV + 0x290,
            if r.next_u32() % 4 == 0 { 0 } else { FX_OBJ2 },
        );
        c.bus.write32(
            FX_OBJ2 + 0xc,
            if r.next_u32() % 4 == 0 { 0 } else { FX_EOBJ },
        );
        c.bus.write32(FX_EOBJ + 0xc, FX_EVT);
        for (k, off) in [0xc0u32, 0xf8].into_iter().enumerate() {
            c.bus
                .write16(FX_EVT + off, (r.next_u32() as i16 as i32 & !3) as u16);
            c.bus
                .write32(FX_EVT + off + 4, fakes::FAKE + 0x20 + 0x10 * k as u32);
        }
        c.bus.write32(gp.wrapping_add(guard), r.next_u32() & 1);
        for k in 0..4u32 {
            c.bus.write32(table + 4 * k, r.next_u32() % 3);
        }
        for i in 0..4u32 {
            let p = FX_PARAMS + 16 * i;
            c.bus.write32(p, r.next_u32() % 4);
            let v = match r.next_u32() % 6 {
                0 => u32::MAX,
                1 => 0,
                2 => 1,
                3 => 2,
                4 => 3,
                _ => r.next_u32() & 0xffff,
            };
            c.bus.write32(p + 0x4, v);
            c.bus.write32(p + 0xc, r.next_u32() % 3);
        }
    }

    /// Arguments of all eight bodies: a0 the struct return / result register,
    /// a1 the argument count (0..4 and -1), a2 the argument array.
    fn oc732_args(c: &mut Cpu, seed: u32) {
        let count = [0u64, 1, 2, 3, 4, u64::MAX][(seed % 6) as usize];
        c.g[A0][0] = sx(FX_SRET);
        c.g[A1][0] = count;
        c.g[A2][0] = sx(FX_PARAMS);
    }

    /// `WScriptFn_PlaceRiderAtObject`: block guard `gp+0x25b0`.
    #[test]
    fn wscript_fn_place_rider_at_object_matches() {
        oc732_install_fakes();
        check(
            ee::s10::f_00300948__generated,
            wscript_fn_place_rider_at_object,
            300,
            &|c, seed| {
                oc732_fixture(c, seed, 0x0030_0948, 0x25b0, GAME_GP.wrapping_add(0xb58));
                oc732_args(c, seed);
                c.pc = 0x0030_0948;
            },
        );
    }

    /// `WScriptFn_SetRailEnabled`: block guard `gp+0x25b4`.
    #[test]
    fn wscript_fn_set_rail_enabled_matches() {
        oc732_install_fakes();
        check(
            ee::s10::f_00300b20__generated,
            wscript_fn_set_rail_enabled,
            300,
            &|c, seed| {
                oc732_fixture(c, seed, 0x0030_0b20, 0x25b4, GAME_GP.wrapping_add(0xb60));
                oc732_args(c, seed);
                c.pc = 0x0030_0b20;
            },
        );
    }

    /// `WScriptFn_PlayScriptList`: block guard `gp+0x25b8`.
    #[test]
    fn wscript_fn_play_script_list_matches() {
        oc732_install_fakes();
        check(
            ee::s10::f_00300c78__generated,
            wscript_fn_play_script_list,
            300,
            &|c, seed| {
                oc732_fixture(c, seed, 0x0030_0c78, 0x25b8, GAME_GP.wrapping_add(0xb68));
                oc732_args(c, seed);
                c.pc = 0x0030_0c78;
            },
        );
    }

    /// `WScriptFn_Nop_1440`: block guard `gp+0x25cc`.
    #[test]
    fn wscript_fn_nop_1440_matches() {
        oc732_install_fakes();
        check(
            ee::s10::f_00301440__generated,
            wscript_fn_nop_1440,
            200,
            &|c, seed| {
                oc732_fixture(c, seed, 0x0030_1440, 0x25cc, GAME_GP.wrapping_add(0xb88));
                oc732_args(c, seed);
                c.pc = 0x0030_1440;
            },
        );
    }

    /// `WScriptFn_GetCollectedCount`: block guard `gp+0x25d8`.
    #[test]
    fn wscript_fn_get_collected_count_matches() {
        oc732_install_fakes();
        fakes::covered(
            ee::s10::f_00301560__generated,
            wscript_fn_get_collected_count,
            200,
            &|c, seed| {
                oc732_fixture(c, seed, 0x0030_1560, 0x25d8, GAME_GP.wrapping_add(0xb90));
                oc732_args(c, seed);
                c.pc = 0x0030_1560;
            },
        );
    }

    /// `WScriptFn_GetObjectSlot31`: block guard `gp+0x25f8`.
    #[test]
    fn wscript_fn_get_object_slot31_matches() {
        oc732_install_fakes();
        fakes::covered(
            ee::s10::f_00301830__generated,
            wscript_fn_get_object_slot31,
            300,
            &|c, seed| {
                oc732_fixture(c, seed, 0x0030_1830, 0x25f8, GAME_GP.wrapping_add(0xba0));
                oc732_args(c, seed);
                c.pc = 0x0030_1830;
            },
        );
    }

    /// `WScriptFn_JumpToTask`: block guard `gp+0x2608`.
    #[test]
    fn wscript_fn_jump_to_task_matches() {
        oc732_install_fakes();
        fakes::covered(
            ee::s10::f_00301b88__generated,
            wscript_fn_jump_to_task,
            200,
            &|c, seed| {
                oc732_fixture(c, seed, 0x0030_1b88, 0x2608, GAME_GP.wrapping_add(0xbb0));
                oc732_args(c, seed);
                c.pc = 0x0030_1b88;
            },
        );
    }

    /// `WScriptFn_GotoTask`: block guard `gp+0x2618`.
    #[test]
    fn wscript_fn_goto_task_matches() {
        oc732_install_fakes();
        fakes::covered(
            ee::s10::f_00301c80__generated,
            wscript_fn_goto_task,
            200,
            &|c, seed| {
                oc732_fixture(c, seed, 0x0030_1c80, 0x2618, GAME_GP.wrapping_add(0xbb8));
                oc732_args(c, seed);
                c.pc = 0x0030_1c80;
            },
        );
    }
}
