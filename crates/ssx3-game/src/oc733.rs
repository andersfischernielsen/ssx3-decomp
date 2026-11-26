//! Staging module oc733 (wave wave241): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc733`.

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
        0x0030_1d78,
        wscript_fn_has_rider_visited,
        "WScriptFn_HasRiderVisited",
    ),
    (0x0030_1f48, wscript_fn_nop_1f48, "WScriptFn_Nop_1f48"),
    (0x0030_2680, wscript_fn_is_game_mode, "WScriptFn_IsGameMode"),
    (0x0030_2870, wscript_fn_is_event, "WScriptFn_IsEvent"),
    (
        0x0030_2ac0,
        wscript_fn_process_command,
        "WScriptFn_ProcessCommand",
    ),
    (
        0x0030_2bd8,
        wscript_fn_mission_stop_process,
        "WScriptFn_MissionStopProcess",
    ),
    (
        0x0030_2ce8,
        wscript_fn_stop_process,
        "WScriptFn_StopProcess",
    ),
];

/// The per-function constants of one `gp`-flagged bind block. The block is
/// a flag word at `gp+flag` (set once, on the first call), the id word
/// `-1` at `gp+flag-8` (the copy source) and the kind byte at `gp+flag-4`
/// (copied into the frame): the first call sets them, then every call copies
/// the eight bytes to the argument block at `sp+0x10` and fills it from the
/// argument list through the kind table at `gp+table`.
struct Ws733Block {
    flag: u32,
    init_cycles: u64,
    copy_ret: u32,
    table: u32,
    entry: u64,
    top: u32,
}

/// A bind body that answers `self` (v0 = self): the block, then one
/// `matches(id)` call, then `castInt(self)` gets the result as its int.
struct Ws733Match {
    block: Ws733Block,
    matches: u32,
    matches_ret: u32,
    cast_ret: u32,
}

const NOP_1F48: Ws733Block = Ws733Block {
    flag: 0x2638,
    init_cycles: 6,
    copy_ret: 0x0030_1fa0,
    table: 0xbc8,
    entry: 4,
    top: 0x0030_1fb8,
};

const MISSION_STOP_PROCESS: Ws733Block = Ws733Block {
    flag: 0x2690,
    init_cycles: 7,
    copy_ret: 0x0030_2c34,
    table: 0xc08,
    entry: 3,
    top: 0x0030_2c48,
};

const STOP_PROCESS: Ws733Block = Ws733Block {
    flag: 0x26a0,
    init_cycles: 7,
    copy_ret: 0x0030_2d44,
    table: 0xc10,
    entry: 3,
    top: 0x0030_2d58,
};

const IS_GAME_MODE: Ws733Match = Ws733Match {
    block: Ws733Block {
        flag: 0x2650,
        init_cycles: 6,
        copy_ret: 0x0030_26d8,
        table: 0xbe0,
        entry: 4,
        top: 0x0030_26f0,
    },
    matches: 0x0030_3d90,
    matches_ret: 0x0030_2740,
    cast_ret: 0x0030_2754,
};

const IS_EVENT: Ws733Match = Ws733Match {
    block: Ws733Block {
        flag: 0x2670,
        init_cycles: 6,
        copy_ret: 0x0030_28c8,
        table: 0xbf0,
        entry: 4,
        top: 0x0030_28e0,
    },
    matches: 0x0030_3e60,
    matches_ret: 0x0030_2930,
    cast_ret: 0x0030_2944,
};

/// Opens the frame and saves s16..s19 and ra, as the generated prologue
/// does, and loads the flag word at `gp+flag` first. `s1` gets the count
/// (`a1`); `s2`/`s3` get `a0`/`a2` (self first) or `a2`/`a0` (sret first).
/// Returns whether the flag word was already set.
fn ws733_prologue(c: &mut Cpu, cycles: u64, flag: u32, self_first: bool) -> bool {
    let gp = c.g[GP][0] as u32;
    c.cycles += cycles;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x70));
    c.g[V0][0] = c.lw(gp.wrapping_add(flag));
    c.sq(17, sp(c).wrapping_add(0x50));
    c.sq(18, sp(c).wrapping_add(0x40));
    c.g[S1][0] = c.g[A1][0];
    c.sq(19, sp(c).wrapping_add(0x30));
    let (src2, src3) = if self_first { (A0, A2) } else { (A2, A0) };
    c.g[S2][0] = c.g[src2][0];
    c.sq(16, sp(c).wrapping_add(0x60));
    c.g[S3][0] = c.g[src3][0];
    let initialised = c.g[V0][0] != 0;
    c.sd(sp(c).wrapping_add(0x20), c.g[RA][0]);
    initialised
}

/// The first-call init of a bind block: the id word `-1` and the flag word
/// `1` in the globals, the kind byte into the frame.
fn ws733_gp_init(c: &mut Cpu, cycles: u64, flag: u32) {
    let gp = c.g[GP][0] as u32;
    c.cycles += cycles;
    c.g[V0][0] = c.lbu(gp.wrapping_add(flag - 4));
    c.g[V1][0] = u64::MAX;
    c.g[A0][0] = 0x1;
    c.sw(gp.wrapping_add(flag - 8), c.g[V1][0]);
    c.sw(gp.wrapping_add(flag), c.g[A0][0]);
    c.sb(sp(c), c.g[V0][0]);
}

/// `MEM_copy(dst, src, len)` with `dst` the argument block at `sp+0x10`
/// (kept in s0).
fn ws733_copy_block(c: &mut Cpu, cycles: u64, src: u64, len: u64, ret_addr: u32) -> Flow {
    c.cycles += cycles;
    c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A2][0] = len;
    c.g[A1][0] = src;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x003e_6574, ret_addr)
}

/// The `(params, count)` loop: each argument is stored into the block at `vp`
/// by its index, as a float when the kind table says float and the argument's
/// kind differs, else as an int. `entry` is the cycle charge of the loop's
/// entry block, `top` its back-edge head. Leaves a0..a3, t0, v0, v1 as the
/// generated loop does.
fn ws733_fill_block(
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

/// The block every `gp`-flagged bind body opens with: the first-call init
/// (unless the flag is set), the copy of the id word, and the fill.
fn ws733_gp_block(c: &mut Cpu, b: &Ws733Block, initialised: bool, count: u64, params: u64) -> Flow {
    let gp = c.g[GP][0] as u32;
    if !initialised {
        ws733_gp_init(c, b.init_cycles, b.flag);
    }
    ws733_copy_block(c, 7, sx(gp.wrapping_add(b.flag - 8)), 8, b.copy_ret)?;
    let vp = c.g[S0][0];
    ws733_fill_block(
        c,
        vp,
        count,
        params,
        gp.wrapping_add(b.table),
        b.entry,
        b.top,
    );
    Ok(())
}

/// `luno_gLunoNil` through the sret pointer (the 12-byte struct tail every
/// WScript body returns through).
fn ws733_nil_return(c: &mut Cpu, sret: u64) {
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

/// The epilogue: restore s16..s19 and ra, pop the frame, return to ra.
fn ws733_leave(c: &mut Cpu) {
    c.lq(16, sp(c).wrapping_add(0x60));
    c.lq(17, sp(c).wrapping_add(0x50));
    c.lq(18, sp(c).wrapping_add(0x40));
    c.lq(19, sp(c).wrapping_add(0x30));
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x20));
    c.g[SP][0] = sx(sp(c).wrapping_add(0x70));
    ret(c);
}

/// A bind body that returns `self` (v0 = self): the block, then the
/// `matches` call on the id word, then `castInt(self)` gets the result.
fn ws733_bind_match(c: &mut Cpu, m: &Ws733Match) -> Flow {
    let initialised = ws733_prologue(c, 11, m.block.flag, true);
    let (count, params) = (c.g[S1][0], c.g[S3][0]);
    ws733_gp_block(c, &m.block, initialised, count, params)?;
    c.cycles += 16;
    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
    call_ptr(c, m.matches, m.matches_ret)?;
    c.g[V1][0] = 0x1;
    c.g[S0][0] = c.g[V0][0];
    c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x0022_6610, m.cast_ret)?;
    c.sw(c.g[V0][0] as u32, c.g[S0][0]);
    c.g[V0][0] = c.g[S2][0];
    ws733_leave(c);
    Ok(())
}

/// `WScriptFn_HasRiderVisited` (0x301d78): the `(params, n)` argument list
/// fills the block at sp+0x10 (the id word first, -1). The id names the
/// instance: -1 is the wscript's own (`*(gp+0xce8)+0x290`), any other id
/// the world cache's section table entry. When that instance is intersected
/// by the being's rider (its virtual `isIntersecting` at `+0x1c`), `self`
/// gets type 1 and its int is set to 1 through `castInt`; otherwise type 1
/// and int 0. Returns `self`.
pub fn wscript_fn_has_rider_visited(c: &mut Cpu) {
    guest(c, has_rider_visited_body)
}

fn has_rider_visited_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    let initialised = ws733_prologue(c, 11, 0x2628, true);
    if !initialised {
        ws733_gp_init(c, 7, 0x2628);
    }
    // The copy of the id word: -1 into the block, then MEM_copy.
    c.g[V0][0] = u64::MAX;
    c.sw(sp(c).wrapping_add(0x10), c.g[V0][0]);
    ws733_copy_block(c, 10, sx(gp.wrapping_add(0x2620)), 8, 0x0030_1de0)?;
    let vp = c.g[S0][0];
    let (count, params) = (c.g[S1][0], c.g[S3][0]);
    ws733_fill_block(c, vp, count, params, gp.wrapping_add(0xbc0), 4, 0x0030_1df8);

    // The instance the id names.
    c.cycles += 4;
    c.g[A2][0] = c.lw(vp as u32);
    c.g[V0][0] = !(0u64 | c.g[A2][0]);
    let has_id = c.g[V0][0] != 0;
    c.g[A1][0] = c.lw(gp.wrapping_add(0x16c8));
    if has_id {
        c.cycles += 8;
        c.g[V0][0] = c.g[A2][0] & 0xff;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        if c.g[V1][0] != 0 {
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
            let nonzero = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            if !nonzero {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            c.g[A1][0] = c.g[V0][0];
        } else {
            c.cycles += 2;
            c.g[A1][0] = 0;
        }
    } else {
        c.cycles += 3;
        c.g[V0][0] = c.lw(gp.wrapping_add(0xce8));
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
    }

    // The rider test on the instance (a1).
    c.cycles += 2;
    let has_obj = c.g[A1][0] != 0;
    c.g[V1][0] = c.lw(gp.wrapping_sub(0x848));
    let mut hit = false;
    if has_obj {
        c.cycles += 20;
        c.g[S0][0] = 1;
        c.g[A2][0] = c.lw(gp.wrapping_add(0xce8));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
        if c.g[A0][0] == 0 {
            c.g[V0][0] = 0;
        }
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6c0));
        if c.g[V0][0] == 0 {
            c.g[A0][0] = 0;
        }
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
        c.g[A2][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        call_ptr(c, target, 0x0030_1ef8)?;
        hit = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
    } else {
        c.cycles += 1;
        c.g[V0][0] = 1;
    }

    if hit {
        c.cycles += 5;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[S0][0]);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0022_6610, 0x0030_1f0c)?;
        c.sw(c.g[V0][0] as u32, c.g[S0][0]);
    } else {
        c.cycles += 4;
        c.g[A0][0] = c.g[S2][0];
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0022_6610, 0x0030_1f24)?;
        c.sw(c.g[V0][0] as u32, 0);
    }
    c.cycles += 8;
    c.g[V0][0] = c.g[S2][0];
    ws733_leave(c);
    Ok(())
}

/// `WScriptFn_Nop_1f48` (0x301f48), the set-game-path body: the block, then
/// `WScriptFn_EmptyHelper(id)`, and the nil struct through the sret.
pub fn wscript_fn_nop_1f48(c: &mut Cpu) {
    guest(c, nop_1f48_body)
}

fn nop_1f48_body(c: &mut Cpu) -> Flow {
    let initialised = ws733_prologue(c, 11, NOP_1F48.flag, false);
    let (count, params) = (c.g[S1][0], c.g[S2][0]);
    ws733_gp_block(c, &NOP_1F48, initialised, count, params)?;
    c.cycles += 18;
    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
    call_ptr(c, 0x0030_3d88, 0x0030_2008)?;
    let sret = c.g[S3][0];
    ws733_nil_return(c, sret);
    ws733_leave(c);
    Ok(())
}

/// `WScriptFn_IsGameMode` (0x302680): the block, then `isGamePath(id)` as
/// the type-and-int of `self`; returns `self`.
pub fn wscript_fn_is_game_mode(c: &mut Cpu) {
    guest(c, is_game_mode_body)
}

fn is_game_mode_body(c: &mut Cpu) -> Flow {
    ws733_bind_match(c, &IS_GAME_MODE)
}

/// `WScriptFn_IsEvent` (0x302870): the block, then `isGameEvent(id)` as the
/// type-and-int of `self`; returns `self`.
pub fn wscript_fn_is_event(c: &mut Cpu) {
    guest(c, is_event_body)
}

fn is_event_body(c: &mut Cpu) -> Flow {
    ws733_bind_match(c, &IS_EVENT)
}

/// `WScriptFn_ProcessCommand` (0x302ac0): the block (its static is the
/// 12-byte record at 0x4fbc40, initialised on the first call), then
/// `WScriptMan_ProcessCommand(manager, rec[0], rec[1])` and the nil struct.
pub fn wscript_fn_process_command(c: &mut Cpu) {
    guest(c, process_command_body)
}

fn process_command_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    let initialised = ws733_prologue(c, 12, 0x2684, false);
    c.g[A2][0] = 0x500000;
    if !initialised {
        c.cycles += 10;
        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x43c0));
        c.g[V0][0] = u64::MAX;
        c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x8));
        c.sw((c.g[A2][0] as u32).wrapping_sub(0x43c0), c.g[V0][0]);
        c.g[A0][0] = 0x1;
        c.sb((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.sw(gp.wrapping_add(0x2684), c.g[A0][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0);
        c.sb(sp(c), c.g[A1][0]);
    }
    let src = sx((c.g[A2][0] as u32).wrapping_sub(0x43c0));
    ws733_copy_block(c, 7, src, 0xc, 0x0030_2b2c)?;
    let vp = c.g[S0][0];
    let (count, params) = (c.g[S1][0], c.g[S2][0]);
    ws733_fill_block(c, vp, count, params, gp.wrapping_add(0xc00), 3, 0x0030_2b40);
    c.cycles += 20;
    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
    c.g[A0][0] = c.lw(gp.wrapping_add(0xce8));
    call_ptr(c, 0x0030_a728, 0x0030_2b98)?;
    let sret = c.g[S3][0];
    ws733_nil_return(c, sret);
    ws733_leave(c);
    Ok(())
}

/// `WScriptFn_MissionStopProcess` (0x302bd8): the block, then
/// `WScriptMan_MissionCommand(manager, rec[0], 0, -1)` and the nil struct.
pub fn wscript_fn_mission_stop_process(c: &mut Cpu) {
    guest(c, mission_stop_process_body)
}

fn mission_stop_process_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    let initialised = ws733_prologue(c, 11, MISSION_STOP_PROCESS.flag, false);
    let (count, params) = (c.g[S1][0], c.g[S2][0]);
    ws733_gp_block(c, &MISSION_STOP_PROCESS, initialised, count, params)?;
    c.cycles += 21;
    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
    c.g[A2][0] = 0;
    c.g[A0][0] = c.lw(gp.wrapping_add(0xce8));
    c.g[A3][0] = u64::MAX;
    call_ptr(c, 0x0030_a868, 0x0030_2ca4)?;
    let sret = c.g[S3][0];
    ws733_nil_return(c, sret);
    ws733_leave(c);
    Ok(())
}

/// `WScriptFn_StopProcess` (0x302ce8): the block, then
/// `WScriptMan_ProcessCommand(manager, rec[0], 0)` and the nil struct.
pub fn wscript_fn_stop_process(c: &mut Cpu) {
    guest(c, stop_process_body)
}

fn stop_process_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    let initialised = ws733_prologue(c, 11, STOP_PROCESS.flag, false);
    let (count, params) = (c.g[S1][0], c.g[S2][0]);
    ws733_gp_block(c, &STOP_PROCESS, initialised, count, params)?;
    c.cycles += 20;
    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
    c.g[A2][0] = 0;
    c.g[A0][0] = c.lw(gp.wrapping_add(0xce8));
    call_ptr(c, 0x0030_a728, 0x0030_2db0)?;
    let sret = c.g[S3][0];
    ws733_nil_return(c, sret);
    ws733_leave(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    // Fixture objects, in DATA+0x200000 and up (clear of the fake log).
    const W_WORLD: u32 = DATA + 0x20_0000;
    const W_OBJ: u32 = DATA + 0x22_0000;
    const W_VT: u32 = DATA + 0x23_0000;
    const W_PARAMS: u32 = DATA + 0x25_0000;
    const W_SRET: u32 = DATA + 0x26_0000;
    const W_TAGS: u32 = DATA + 0x27_0000;
    const W_TAGS2: u32 = DATA + 0x27_1000;
    const W_TAGTBL: u32 = DATA + 0x28_0000;
    const W_ENTRY: u32 = DATA + 0x29_0000;
    const W_ENTRY2: u32 = DATA + 0x2a_0000;
    const W_PREV: u32 = DATA + 0x2b_0000;
    const W_RIDER: u32 = DATA + 0x2c_0000;
    const W_SUB: u32 = DATA + 0x2d_0000;
    /// The word the gated fakes look for (other tests' machines lack it).
    const W_GATE: u32 = DATA + 0x3f_0000;
    const W_MAGIC: u32 = 0x0733_0733;

    static OC733_PREV_PROCESS: AtomicUsize = AtomicUsize::new(0);
    static OC733_PREV_MISSION: AtomicUsize = AtomicUsize::new(0);

    fn oc733_gated(c: &mut Cpu) -> bool {
        c.bus.read32(W_GATE) == W_MAGIC
    }

    /// `WScriptMan_ProcessCommand` (0x30a728), gated: logs the call and returns.
    fn oc733_fake_process(c: &mut Cpu) {
        if !oc733_gated(c) {
            return fakes::fall_back(c, &OC733_PREV_PROCESS, ee::s10::f_0030a728__generated);
        }
        fakes::fake_call(c, 0x0030_a728);
    }

    /// `WScriptMan_MissionCommand` (0x30a868), gated: logs the call and returns.
    fn oc733_fake_mission(c: &mut Cpu) {
        if !oc733_gated(c) {
            return fakes::fall_back(c, &OC733_PREV_MISSION, ee::s10::f_0030a868__generated);
        }
        fakes::fake_call(c, 0x0030_a868);
    }

    fn oc733_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0030A728),
            oc733_fake_process,
            &OC733_PREV_PROCESS,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0030A868),
            oc733_fake_mission,
            &OC733_PREV_MISSION,
        );
    }

    /// The globals and objects the bind bodies read, varied by `seed`: the
    /// world and its object and manager chain, the zone tables, the flag word
    /// at `flag` (set or not), the id word at `block` (the copy source), the
    /// kind table at `table`, and four parameter records.
    fn oc733_fixture(c: &mut Cpu, seed: u32, salt: u64, flag: u32, block: u32, table: u32) {
        let mut r = machine(c, seed, salt, 2);
        let gp = GAME_GP;
        c.bus.write32(gp.wrapping_sub(0x848), W_WORLD);
        c.bus.write32(W_WORLD + 0x84, W_OBJ);
        c.bus.write32(W_OBJ + 0xc, W_VT);
        c.bus.write32(gp.wrapping_add(0x16c8), W_TAGS);
        c.bus.write32(W_TAGS, W_TAGS2);
        c.bus.write32(W_TAGS2 + 0x8, W_TAGTBL);
        for i in 0..256u32 {
            let e = if r.next_u32() % 4 == 0 { 0 } else { W_ENTRY };
            c.bus.write32(W_TAGTBL + 4 * i, e);
        }
        c.bus.write32(W_ENTRY + 0x1c, W_ENTRY2);
        for i in 0..0x100u32 {
            c.bus.write32(W_ENTRY2 + 4 * i, (r.next_u32() & 0xff) << 8);
        }
        c.bus.write32(
            W_PREV + 0x290,
            if r.next_u32() % 3 == 0 { 0 } else { W_OBJ },
        );
        c.bus.write32(gp.wrapping_add(0xce8), W_PREV);
        c.bus.write32(flag, r.next_u32() & 1);
        // The id word: -1 (the wscript's own instance) or a small section id.
        let id = if r.next_u32() % 3 == 0 {
            u32::MAX
        } else {
            r.next_u32() & 0xffff
        };
        c.bus.write32(block, id);
        for k in 0..4u32 {
            c.bus.write32(table + 4 * k, r.next_u32() % 3);
        }
        for i in 0..4u32 {
            let p = W_PARAMS + 16 * i;
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

    /// Arguments of the bind bodies: sret or self (`a0`), count (0..4 and
    /// -1), and the parameter array.
    fn oc733_args(c: &mut Cpu, seed: u32, sret: u32) {
        let count = [0u64, 1, 2, 3, 4, u64::MAX][(seed % 6) as usize];
        c.g[A0][0] = sx(sret);
        c.g[A1][0] = count;
        c.g[A2][0] = sx(W_PARAMS);
    }

    /// The rider of the being (`gp+0xce8`'s word names the entry index): its
    /// `+0x6c0` record holds the virtual `isIntersecting` at `+0x1c`. One half
    /// of the seeds answer through the miss fake (non-zero), the other half
    /// through a real leaf that returns 0.
    fn oc733_rider(c: &mut Cpu, seed: u32) {
        let k = seed % 4;
        c.bus.write32(W_PREV, k);
        c.bus.write32(W_VT + 4 * k + 0x28, W_RIDER);
        c.bus.write32(W_RIDER + 0x6c0, W_SUB);
        c.bus.write16(W_SUB + 0x18, 0);
        let target = if (seed / 4) % 2 == 0 {
            fakes::FAKE
        } else {
            0x0014_06c0
        };
        c.bus.write32(W_SUB + 0x1c, target);
    }

    #[test]
    fn wscript_fn_has_rider_visited_matches() {
        check(
            ee::s10::f_00301d78__generated,
            wscript_fn_has_rider_visited,
            200,
            &|c, seed| {
                oc733_fixture(
                    c,
                    seed,
                    0x0030_1d78,
                    GAME_GP.wrapping_add(0x2628),
                    GAME_GP.wrapping_add(0x2620),
                    GAME_GP.wrapping_add(0xbc0),
                );
                oc733_rider(c, seed);
                oc733_args(c, seed, W_SRET);
                c.pc = 0x0030_1d78;
            },
        );
    }

    #[test]
    fn wscript_fn_nop_1f48_matches() {
        check(
            ee::s10::f_00301f48__generated,
            wscript_fn_nop_1f48,
            200,
            &|c, seed| {
                oc733_fixture(
                    c,
                    seed,
                    0x0030_1f48,
                    GAME_GP.wrapping_add(0x2638),
                    GAME_GP.wrapping_add(0x2630),
                    GAME_GP.wrapping_add(0xbc8),
                );
                oc733_args(c, seed, W_SRET);
                c.pc = 0x0030_1f48;
            },
        );
    }

    #[test]
    fn wscript_fn_is_game_mode_matches() {
        check(
            ee::s10::f_00302680__generated,
            wscript_fn_is_game_mode,
            200,
            &|c, seed| {
                oc733_fixture(
                    c,
                    seed,
                    0x0030_2680,
                    GAME_GP.wrapping_add(0x2650),
                    GAME_GP.wrapping_add(0x2648),
                    GAME_GP.wrapping_add(0xbe0),
                );
                oc733_args(c, seed, W_SRET);
                c.pc = 0x0030_2680;
            },
        );
    }

    #[test]
    fn wscript_fn_is_event_matches() {
        check(
            ee::s10::f_00302870__generated,
            wscript_fn_is_event,
            200,
            &|c, seed| {
                oc733_fixture(
                    c,
                    seed,
                    0x0030_2870,
                    GAME_GP.wrapping_add(0x2670),
                    GAME_GP.wrapping_add(0x2668),
                    GAME_GP.wrapping_add(0xbf0),
                );
                oc733_args(c, seed, W_SRET);
                c.pc = 0x0030_2870;
            },
        );
    }

    #[test]
    fn wscript_fn_process_command_matches() {
        oc733_install_fakes();
        check(
            ee::s10::f_00302ac0__generated,
            wscript_fn_process_command,
            200,
            &|c, seed| {
                oc733_fixture(
                    c,
                    seed,
                    0x0030_2ac0,
                    GAME_GP.wrapping_add(0x2684),
                    0x004f_bc40,
                    GAME_GP.wrapping_add(0xc00),
                );
                oc733_args(c, seed, W_SRET);
                c.bus.write32(W_GATE, W_MAGIC);
                c.pc = 0x0030_2ac0;
            },
        );
    }

    #[test]
    fn wscript_fn_mission_stop_process_matches() {
        oc733_install_fakes();
        check(
            ee::s10::f_00302bd8__generated,
            wscript_fn_mission_stop_process,
            200,
            &|c, seed| {
                oc733_fixture(
                    c,
                    seed,
                    0x0030_2bd8,
                    GAME_GP.wrapping_add(0x2690),
                    GAME_GP.wrapping_add(0x2688),
                    GAME_GP.wrapping_add(0xc08),
                );
                oc733_args(c, seed, W_SRET);
                c.bus.write32(W_GATE, W_MAGIC);
                c.pc = 0x0030_2bd8;
            },
        );
    }

    #[test]
    fn wscript_fn_stop_process_matches() {
        oc733_install_fakes();
        check(
            ee::s10::f_00302ce8__generated,
            wscript_fn_stop_process,
            200,
            &|c, seed| {
                oc733_fixture(
                    c,
                    seed,
                    0x0030_2ce8,
                    GAME_GP.wrapping_add(0x26a0),
                    GAME_GP.wrapping_add(0x2698),
                    GAME_GP.wrapping_add(0xc10),
                );
                oc733_args(c, seed, W_SRET);
                c.bus.write32(W_GATE, W_MAGIC);
                c.pc = 0x0030_2ce8;
            },
        );
    }
}
