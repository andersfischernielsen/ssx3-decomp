//! Staging module oc741 (wave wave242): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc741`.

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
/// `WScriptFn_AddTexFlipModifier` (0x2fe4a8) and `WScriptFn_AddBoostModifier`
/// (0x2fe668) are on this wave's list but were already ported in oc731.
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002f_f390,
        world_obj_factory_float_rail,
        "WorldObjFactory_FloatRail",
    ),
    (
        0x002f_f5e8,
        world_obj_factory_spring_rail,
        "WorldObjFactory_SpringRail",
    ),
    (0x002f_fd58, wscript_fn_unhide, "WScriptFn_Unhide"),
    (
        0x0030_0418,
        wscript_fn_kill_restorable,
        "WScriptFn_KillRestorable",
    ),
];

/// `gp` of the game (the world and script-manager globals are gp-relative).
fn wsc_gp(c: &Cpu) -> u32 {
    c.g[GP][0] as u32
}

/// `EA_MemCopy`, which copies the static template into the stack buffer.
const WSC_MEMCPY: u32 = 0x003e_6574;

/// `WorldObjFactory_AttachModifier` (0x2fae38), reached by a direct call.
const WSC_ATTACH: Callee = Callee {
    addr: 0x002f_ae38,
    f: ee::s10::f_002fae38,
};

/// `EA_MemAllocNamed` (0x317d70), reached by a direct call.
const WSC_ALLOC: Callee = Callee {
    addr: 0x0031_7d70,
    f: ee::s10::f_00317d70,
};

/// The parameter loop shared by the script functions: each `(idx, val, type)`
/// entry (16 bytes, params in `A2`, count in the `count` register) stores its
/// value into `buf[idx]` (`buf` register), as a float when the type table
/// (`types`, kept in `T0`) wants a float and the entry's type differs.
struct WscFill {
    /// Register holding the parameter count (also copied to `A1`).
    count: usize,
    /// Register holding the parameter list.
    params: usize,
    /// Register holding the stack buffer.
    buf: usize,
    /// Cycles charged before the loop.
    pre: u64,
    /// `V0 = 0x440000` is set before the loop (the rails' table is `0x44xxxx`).
    set_v0: bool,
    /// Value loaded in `T0` (the type table's address).
    types: u32,
    /// Event point of the loop's back edge.
    head: u32,
}

fn wsc_fill(c: &mut Cpu, s: &WscFill) {
    c.g[A2][0] = c.g[s.params][0];
    if (c.g[s.count][0] as i64) <= 0 {
        return;
    }
    c.cycles += s.pre;
    if s.set_v0 {
        c.g[V0][0] = 0x0044_0000;
    }
    c.g[A1][0] = c.g[s.count][0];
    c.g[T0][0] = sx(s.types);
    c.g[A3][0] = 2;
    loop {
        c.cycles += 8;
        let ent = c.g[A2][0] as u32;
        c.g[V0][0] = Addr(ent).lw(c);
        c.g[V1][0] = Addr(ent.wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[s.buf][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
        let slot = c.g[A0][0] as u32;
        let mut store = true;
        if c.g[V1][0] == c.g[V0][0] {
            c.g[V0][0] = Addr(ent.wrapping_add(4)).lw(c);
        } else {
            c.cycles += 2;
            if c.g[V0][0] != c.g[A3][0] {
                c.g[V0][0] = Addr(ent.wrapping_add(4)).lw(c);
            } else {
                c.cycles += 4;
                c.lwc1(0, ent.wrapping_add(4));
                c.cvt_s_w(0, 0);
                c.swc1(0, slot);
                store = false;
            }
        }
        if store {
            c.cycles += 1;
            let v0 = c.g[V0][0];
            Addr(slot).sw(c, v0);
        }
        c.cycles += 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        let more = c.g[A1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, s.head);
        }
    }
}

/// The instance for the id in `buf[0]` (`buf` register): the script
/// manager's for -1, else the section entry's record (`0` when there is
/// none). Leaves it in V0 (the shape of `Unhide` and `KillRestorable`).
fn wsc_resolve(c: &mut Cpu, buf: usize) {
    c.cycles += 4;
    c.g[A2][0] = Addr(c.g[buf][0] as u32).lw(c);
    c.g[V0][0] = !c.g[A2][0];
    c.g[A1][0] = Addr(wsc_gp(c).wrapping_add(0x16c8)).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 3;
        c.g[V0][0] = Addr(wsc_gp(c).wrapping_add(0xce8)).lw(c);
        c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
        return;
    }
    c.cycles += 8;
    c.g[V0][0] = c.g[A2][0] & 0xff;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
    if c.g[V1][0] == 0 {
        c.cycles += 1;
        c.g[V0][0] = 0;
        return;
    }
    c.cycles += 7;
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
    if c.g[V0][0] != 0 {
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        return;
    }
    c.cycles += 1;
    c.g[V0][0] = 0;
}

/// The rails' instance lookup: like [`wsc_resolve`] but the result goes to
/// `S0` (and `A1`) and the instruction counts differ.
fn wsc_rail_resolve(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
    c.g[V0][0] = !c.g[A2][0];
    c.g[A1][0] = Addr(wsc_gp(c).wrapping_add(0x16c8)).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 3;
        c.g[V0][0] = Addr(wsc_gp(c).wrapping_add(0xce8)).lw(c);
        c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
        return;
    }
    c.cycles += 8;
    c.g[V0][0] = c.g[A2][0] & 0xff;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    if c.g[V1][0] != 0 {
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
        c.cycles += 7;
        c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
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
    c.cycles += 1;
    c.g[S0][0] = c.g[A1][0];
}

/// Per-rail constants of `WorldObjFactory_FloatRail` / `_SpringRail`: the two
/// factories share one shape (a 88-byte parameter record with a static
/// template) and differ in these addresses and in the node construction.
struct WscRail {
    /// `gp` offset of the one-time init flag.
    flag_gp: u32,
    /// The static record template (`0x500000 - off`).
    tpl: u32,
    /// The parameter type table.
    types: u32,
    /// Return address of the template copy and event point of the fill loop.
    copy_ret: u32,
    fill_head: u32,
    /// The `checkActiveNode` type (`A1` of the attach call) and its return.
    attach_type: u64,
    attach_ret: u32,
    /// Builds and constructs the rail node (the alloc tail).
    node: fn(&mut Cpu) -> Flow,
}

/// The floating-rail node: `cMemMan_alloc(0x80, name, 0x20000000, 0)` and its
/// constructor `(node, instance, buf)`.
fn wsc_float_rail_node(c: &mut Cpu) -> Flow {
    c.cycles += 10;
    c.g[A1][0] = 0x0049_0000;
    c.g[A0][0] = 0x80;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6898));
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    WSC_ALLOC.call(c, 0x002f_f598)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = c.g[S1][0];
    call_ptr(c, 0x0034_8d98, 0x002f_f5a8)
}

/// The spring-rail node: scales the record's float at `+0x50` by a constant
/// from gp, then allocates and constructs the node.
fn wsc_spring_rail_node(c: &mut Cpu) -> Flow {
    c.cycles += 14;
    let buf = c.g[S1][0] as u32;
    c.lwc1(0, buf.wrapping_add(0x50));
    c.g[A1][0] = 0x0049_0000;
    c.lwc1(1, wsc_gp(c).wrapping_sub(0x336c));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6888));
    c.g[A0][0] = 0x80;
    c.g[A2][0] = 0x2000_0000;
    c.mul_s(0, 0, 1);
    c.g[A3][0] = 0;
    c.swc1(0, buf.wrapping_add(0x50));
    WSC_ALLOC.call(c, 0x002f_f800)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = c.g[S1][0];
    call_ptr(c, 0x0034_9b48, 0x002f_f810)
}

const WSC_FLOAT_RAIL: WscRail = WscRail {
    flag_gp: 0x2544,
    tpl: 0x004f_ba38,
    types: 0x0044_6480,
    copy_ret: 0x002f_f490,
    fill_head: 0x002f_f4a8,
    attach_type: 0x14,
    attach_ret: 0x002f_f578,
    node: wsc_float_rail_node,
};

const WSC_SPRING_RAIL: WscRail = WscRail {
    flag_gp: 0x2548,
    tpl: 0x004f_ba90,
    types: 0x0044_64d8,
    copy_ret: 0x002f_f6e8,
    fill_head: 0x002f_f700,
    attach_type: 0x15,
    attach_ret: 0x002f_f7d0,
    node: wsc_spring_rail_node,
};

/// The shared body of the two rail factories `(ret slot, n, params)`: builds
/// the 22-word parameter record on the stack (template copy plus the
/// parameter list), finds the instance for its id, and unless the instance
/// already has a matching node attaches a new rail node to it. Returns the
/// script nil value through the hidden return slot (`S2`).
fn wsc_rail_factory(c: &mut Cpu, s: &WscRail) -> Flow {
    c.cycles += 13;
    let frame = Frame::open(c, 0xd0);
    c.g[V0][0] = Addr(wsc_gp(c).wrapping_add(s.flag_gp)).lw(c);
    frame.save(c, S0, 0xc0);
    c.g[A3][0] = 0x0050_0000;
    frame.save(c, S2, 0xa0);
    c.g[S0][0] = c.g[A1][0];
    frame.save(c, S4, 0x80);
    c.g[S2][0] = c.g[A0][0];
    frame.save(c, S1, 0xb0);
    c.g[S4][0] = c.g[A2][0];
    frame.save(c, S3, 0x90);
    let flag_clear = c.g[V0][0] == 0;
    frame.save_ra(c, 0x70);
    if flag_clear {
        // One-time init of the template: all ids -1, 0.5f and 1.0f scale
        // floats, the flag set.
        c.cycles += 34;
        c.g[V0][0] = sx(s.tpl);
        c.g[V1][0] = u64::MAX;
        c.g[A2][0] = Addr(s.tpl.wrapping_add(0x54)).lbu(c);
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 1);
        c.g[A0][0] = u64::MAX;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[A1][0] = 1;
        let (v1, a0, a1, a2) = (c.g[V1][0], c.g[A0][0], c.g[A1][0], c.g[A2][0]);
        Addr(s.tpl).sw(c, v1);
        Addr(s.tpl + 0x54).sb(c, a2);
        Addr(s.tpl + 0x20).sw(c, v1);
        Addr(s.tpl + 0x40).sw(c, a0);
        c.swc1(1, s.tpl + 0x4c);
        c.swc1(0, s.tpl + 0x50);
        Addr(wsc_gp(c).wrapping_add(s.flag_gp)).sw(c, a1);
        for off in [0x4, 0x8, 0xc, 0x10, 0x14, 0x18, 0x1c] {
            Addr(s.tpl + off).sw(c, v1);
        }
        for off in [0x24, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c] {
            Addr(s.tpl + off).sw(c, a0);
        }
        Addr(s.tpl + 0x44).sw(c, a1);
        c.swc1(1, s.tpl + 0x48);
        let a2 = c.g[A2][0];
        Addr(sp(c)).sb(c, a2);
    }

    // The record's first nine words are -1, then the template copy.
    c.cycles += 19;
    c.g[V0][0] = u64::MAX;
    c.g[S1][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A2][0] = 0x58;
    c.g[A1][0] = sx(s.tpl);
    let buf = sp(c).wrapping_add(0x10);
    for i in 0..9 {
        Addr(buf.wrapping_add(4 * i)).sw(c, u64::MAX);
    }
    c.g[A0][0] = c.g[S1][0];
    c.g[S3][0] = 0x004d_0000;
    call_ptr(c, WSC_MEMCPY, s.copy_ret)?;

    wsc_fill(
        c,
        &WscFill {
            count: S0,
            params: S4,
            buf: S1,
            pre: 4,
            set_v0: true,
            types: s.types,
            head: s.fill_head,
        },
    );
    wsc_rail_resolve(c);

    // Attach a rail node unless the instance is missing or its active node
    // already took the modifier.
    c.cycles += 2;
    if c.g[S0][0] != 0 {
        c.cycles += 3;
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[A1][0] = s.attach_type;
        let mut done = false;
        if c.g[A0][0] != 0 {
            c.cycles += 4;
            c.g[A2][0] = c.g[S1][0];
            WSC_ATTACH.call(c, s.attach_ret)?;
            done = c.g[V0][0] != 0;
        }
        if !done {
            (s.node)(c)?;
        }
    }

    // Epilogue: copy the nil value (12 bytes) to the return slot.
    c.cycles += 16;
    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6f68));
    let src = c.g[A1][0] as u32;
    c.ldl(2, src.wrapping_add(7));
    c.ldr(2, src);
    c.g[V1][0] = Addr(src.wrapping_add(8)).lw(c);
    let dst = c.g[S2][0] as u32;
    c.sdl(2, dst.wrapping_add(7));
    c.sdr(2, dst);
    let v1 = c.g[V1][0];
    Addr(dst.wrapping_add(8)).sw(c, v1);
    c.g[V0][0] = c.g[S2][0];
    frame.restore(c, S0, 0xc0);
    frame.restore(c, S1, 0xb0);
    frame.restore(c, S2, 0xa0);
    frame.restore(c, S3, 0x90);
    frame.restore(c, S4, 0x80);
    frame.restore_ra(c, 0x70);
    frame.close(c);
    Ok(())
}

/// `WorldObjFactory_FloatRail` (0x2ff390), the C++ `MakeFloatRailFunc`: the
/// script call that gives an instance a floating-rail node, built from the
/// script's parameter list.
pub fn world_obj_factory_float_rail(c: &mut Cpu) {
    guest(c, |c| wsc_rail_factory(c, &WSC_FLOAT_RAIL));
}

/// `WorldObjFactory_SpringRail` (0x2ff5e8): the same with a spring-rail node
/// (its `+0x50` float is scaled by a gp constant before the node is built).
pub fn world_obj_factory_spring_rail(c: &mut Cpu) {
    guest(c, |c| wsc_rail_factory(c, &WSC_SPRING_RAIL));
}

/// The epilogue of `Unhide` and `KillRestorable`: copies the 12-byte nil value
/// (at `V1 - 0x6f68`) to the return slot (`S3`), leaves that pointer in V0,
/// restores the saved registers and returns.
fn wsc_nil_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 15;
    c.g[V0][0] = c.g[S3][0];
    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
    let src = c.g[A3][0] as u32;
    c.ldl(4, src.wrapping_add(7));
    c.ldr(4, src);
    c.g[A1][0] = Addr(src.wrapping_add(8)).lw(c);
    let dst = c.g[S3][0] as u32;
    c.sdl(4, dst.wrapping_add(7));
    c.sdr(4, dst);
    let a1 = c.g[A1][0];
    Addr(dst.wrapping_add(8)).sw(c, a1);
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore_ra(c, 0x20);
    frame.close(c);
}

/// The prologue of `Unhide` and `KillRestorable` (frame 0x70, `S1` = count,
/// `S2` = params, `S3` = return slot): returns whether the init flag at
/// `gp + flag_gp` was clear. `kill` selects the order of the two register
/// saves (the compiled prologues differ).
fn wsc_script_prologue(c: &mut Cpu, flag_gp: u32, cycles: u64, kill: bool) -> (Frame, bool) {
    c.cycles += cycles;
    let frame = Frame::open(c, 0x70);
    c.g[V0][0] = Addr(wsc_gp(c).wrapping_add(flag_gp)).lw(c);
    if kill {
        frame.save(c, S2, 0x40);
        frame.save(c, S1, 0x50);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S3, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x60);
    } else {
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x60);
    }
    c.g[S3][0] = c.g[A0][0];
    let clear = c.g[V0][0] == 0;
    frame.save_ra(c, 0x20);
    (frame, clear)
}

/// `WScriptFn_Unhide` (0x2ffd58), the C++ `UnhideFunc`: unhides the instance
/// named by the first parameter: when its active node answers `isType(0x10)`
/// the node's `delete(3)` runs.
pub fn wscript_fn_unhide(c: &mut Cpu) {
    guest(c, |c| {
        let (frame, clear) = wsc_script_prologue(c, 0x2570, 11, false);
        if clear {
            c.cycles += 7;
            c.g[V0][0] = Addr(wsc_gp(c).wrapping_add(0x256c)).lbu(c);
            c.g[V1][0] = u64::MAX;
            c.g[A0][0] = 1;
            let (v0, v1, a0) = (c.g[V0][0], c.g[V1][0], c.g[A0][0]);
            Addr(wsc_gp(c).wrapping_add(0x2568)).sw(c, v1);
            Addr(wsc_gp(c).wrapping_add(0x2570)).sw(c, a0);
            Addr(sp(c)).sb(c, v0);
        }
        // The two-word record (id -1, template word), then the parameters.
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A2][0] = 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(wsc_gp(c).wrapping_add(0x2568));
        let v0 = c.g[V0][0];
        Addr(sp(c).wrapping_add(0x10)).sw(c, v0);
        call_ptr(c, WSC_MEMCPY, 0x002f_fdc0)?;
        wsc_fill(
            c,
            &WscFill {
                count: S1,
                params: S2,
                buf: S0,
                pre: 4,
                set_v0: false,
                types: wsc_gp(c).wrapping_add(0xb30),
                head: 0x002f_fdd8,
            },
        );
        wsc_resolve(c, S0);

        c.cycles += 2;
        c.g[V1][0] = 0x004d_0000;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
            c.g[A1][0] = 0x10;
            if c.g[S0][0] != 0 {
                // node->isType(0x10), then node->delete(3).
                c.cycles += 7;
                c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x50)).lh(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x54)).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x002f_fea4)?;
                c.g[A1][0] = 3;
                if c.g[V0][0] != 0 {
                    c.cycles += 5;
                    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(8)).lh(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x002f_fec0)?;
                }
                c.cycles += 1;
                c.g[V1][0] = 0x004d_0000;
            }
        }
        wsc_nil_epilogue(c, frame);
        Ok(())
    });
}

/// `WScriptFn_KillRestorable` (0x300418), the C++ `KillRestorableFunc`: when
/// the instance named by the first parameter has an active node, calls its
/// `expire(0)` or `expire(1)` as the second parameter says (nothing for other
/// values).
pub fn wscript_fn_kill_restorable(c: &mut Cpu) {
    guest(c, |c| {
        let (frame, clear) = wsc_script_prologue(c, 0x2580, 12, true);
        c.g[A2][0] = 0x0050_0000;
        if clear {
            c.cycles += 10;
            c.g[V1][0] = sx(0x004f_bb38);
            c.g[V0][0] = u64::MAX;
            c.g[A1][0] = Addr(0x004f_bb38u32.wrapping_add(8)).lbu(c);
            let (v0, a1) = (c.g[V0][0], c.g[A1][0]);
            Addr(0x004f_bb38).sw(c, v0);
            c.g[A0][0] = 1;
            Addr(0x004f_bb38u32.wrapping_add(8)).sb(c, a1);
            let a0 = c.g[A0][0];
            Addr(wsc_gp(c).wrapping_add(0x2580)).sw(c, a0);
            Addr(0x004f_bb38u32.wrapping_add(4)).sw(c, 0);
            Addr(sp(c)).sb(c, a1);
        }
        // The three-word record (id -1, template words), then the parameters.
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A1][0] = sx(0x004f_bb38);
        c.g[A2][0] = 0xc;
        let v0 = c.g[V0][0];
        Addr(sp(c).wrapping_add(0x10)).sw(c, v0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, WSC_MEMCPY, 0x0030_0490)?;
        wsc_fill(
            c,
            &WscFill {
                count: S1,
                params: S2,
                buf: S0,
                pre: 4,
                set_v0: false,
                types: wsc_gp(c).wrapping_add(0xb40),
                head: 0x0030_04a8,
            },
        );
        wsc_resolve(c, S0);

        c.cycles += 3;
        c.g[A2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[V1][0] = 0x004d_0000;
        if c.g[A2][0] != 0 {
            // node->expire(buf[1]) for the values 0 and 1.
            c.cycles += 3;
            c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lw(c);
            c.g[V0][0] = 1;
            if c.g[A0][0] == 0 {
                c.cycles += 8;
                c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0xc)).lw(c);
                c.g[A1][0] = 0;
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x110)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x114)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0030_057c)?;
                c.g[V1][0] = 0x004d_0000;
            } else {
                c.cycles += 2;
                c.g[V1][0] = 0x004d_0000;
                if c.g[A0][0] == c.g[V0][0] {
                    c.cycles += 7;
                    c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0xc)).lw(c);
                    c.g[A1][0] = 1;
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x110)).lh(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x114)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0030_05a4)?;
                    c.g[V1][0] = 0x004d_0000;
                }
            }
        }
        wsc_nil_epilogue(c, frame);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Control words of the gated fakes (clear of the fake call log).
    const WSC_CTRL: u32 = DATA + 0x20_0000;
    const WSC_MAGIC: u32 = 0x4f43_3734;
    /// Non-zero makes the `AttachModifier` fake report "not attached".
    const WSC_ATTACH_FAILS: u32 = WSC_CTRL + 4;

    macro_rules! wsc_fake {
        ($fake:ident, $prev:ident, $gen:path, $addr:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(WSC_CTRL) != WSC_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $addr);
            }
        };
    }

    wsc_fake!(
        oc741_fake_alloc,
        OC741_PREV_ALLOC,
        ee::s10::f_00317d70__generated,
        0x0031_7d70
    );
    wsc_fake!(
        oc741_fake_float_ctor,
        OC741_PREV_FLOAT_CTOR,
        ee::s11::f_00348d98__generated,
        0x0034_8d98
    );
    wsc_fake!(
        oc741_fake_spring_ctor,
        OC741_PREV_SPRING_CTOR,
        ee::s11::f_00349b48__generated,
        0x0034_9b48
    );

    static OC741_PREV_ATTACH: AtomicUsize = AtomicUsize::new(0);
    /// `WorldObjFactory_AttachModifier` fake: a pointer result, or 0 when the
    /// fixture asks for "not attached".
    fn oc741_fake_attach(c: &mut Cpu) {
        if c.bus.read32(WSC_CTRL) != WSC_MAGIC {
            return fakes::fall_back(c, &OC741_PREV_ATTACH, ee::s10::f_002fae38__generated);
        }
        fakes::fake_call(c, 0x002f_ae38);
        if c.pc == c.g[RA][0] as u32 && c.bus.read32(WSC_ATTACH_FAILS) != 0 {
            c.g[V0][0] = 0;
        }
    }

    fn oc741_install_fakes() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc741_fake_alloc,
            &OC741_PREV_ALLOC,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00348D98),
            oc741_fake_float_ctor,
            &OC741_PREV_FLOAT_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00349B48),
            oc741_fake_spring_ctor,
            &OC741_PREV_SPRING_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_002FAE38),
            oc741_fake_attach,
            &OC741_PREV_ATTACH,
        );
    }

    /// The machine state of the four script functions `(ret slot, n, params)`:
    /// `seed % 4` parameters (entry 0 names instance id 0 on `seed % 3 == 0`,
    /// else the id stays -1), the type table, the init flag, and the
    /// world -> section -> instance chain. The instance's node (when there is
    /// one) has a vtable at `DATA + 0x9100` for the caller to fill.
    fn wsc_fixture(c: &mut Cpu, r: &mut Rng, seed: u32, flag_gp: u32, types: u32, max_idx: u32) {
        c.bus.write32(WSC_CTRL, WSC_MAGIC);
        c.bus.write32(WSC_ATTACH_FAILS, (seed / 7) % 2);
        c.g[GP][0] = sx(GAME_GP);
        c.g[A0][0] = sx(DATA + 0x7000);
        let n = seed % 4;
        c.g[A1][0] = n as u64;
        c.g[A2][0] = sx(DATA + 0x1000);
        for i in 0..n {
            let e = DATA + 0x1000 + 16 * i;
            let idx = if i == 0 && seed % 3 == 0 {
                0
            } else {
                1 + r.next_u32() % max_idx
            };
            let val = if idx == 0 { 0 } else { r.next_u32() % 3 };
            c.bus.write32(e, idx);
            c.bus.write32(e + 4, val);
            c.bus.write32(e + 0xc, r.next_u32() % 3);
        }
        for i in 0..5 {
            c.bus.write32(types + 4 * i, r.next_u32() % 3);
        }
        c.bus.write32(GAME_GP + flag_gp, seed % 2);
        c.bus.write32(GAME_GP + 0x16c8, DATA + 0x3000);
        c.bus.write32(DATA + 0x3000, DATA + 0x4000);
        c.bus.write32(DATA + 0x4008, DATA + 0x5000);
        let section = if (seed / 3) % 5 == 2 {
            0
        } else {
            DATA + 0x6000
        };
        c.bus.write32(DATA + 0x5000, section);
        c.bus.write32(DATA + 0x601c, DATA + 0x6100);
        let entry = if (seed / 5) % 6 == 4 {
            0
        } else {
            ((DATA + 0x8000) >> 2) << 8
        };
        c.bus.write32(DATA + 0x6100, entry);
        // The script manager's instance (the id -1 path) is the same record.
        c.bus.write32(GAME_GP + 0xce8, DATA + 0xa000);
        let rec = if (seed / 11) % 7 == 3 {
            0
        } else {
            DATA + 0x8000
        };
        c.bus.write32(DATA + 0xa290, rec);
        c.bus
            .write32(DATA + 0x800c, if seed % 5 == 1 { 0 } else { DATA + 0x9000 });
        c.bus.write32(DATA + 0x900c, DATA + 0x9100);
    }

    /// `WorldObjFactory_FloatRail`: the one-time template init (flag clear
    /// or set), the parameter loop (int and float stores), the id -1 and
    /// section lookups (with missing section, entry and record), and the node
    /// tail: attach succeeds, attach fails (alloc + constructor), no node.
    #[test]
    fn world_obj_factory_float_rail_matches() {
        oc741_install_fakes();
        fakes::covered(
            ee::s10::f_002ff390__generated,
            world_obj_factory_float_rail,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ff3_90, 3);
                wsc_fixture(c, &mut r, seed, 0x2544, 0x0044_6480, 3);
                c.pc = 0x002f_f390;
            },
        );
    }

    /// `WorldObjFactory_SpringRail`: as the float rail, with the spring
    /// node's scaled float.
    #[test]
    fn world_obj_factory_spring_rail_matches() {
        oc741_install_fakes();
        fakes::covered(
            ee::s10::f_002ff5e8__generated,
            world_obj_factory_spring_rail,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ff5_e8, 3);
                wsc_fixture(c, &mut r, seed, 0x2548, 0x0044_64d8, 3);
                // The record's float at +0x50 gets scaled: give it a value.
                c.bus.write32(0x004f_ba90 + 0x50, 0x3f80_0000 ^ (seed << 8));
                c.pc = 0x002f_f5e8;
            },
        );
    }

    /// `WScriptFn_Unhide`: init flag clear or set, parameters, both lookups;
    /// the node's `isType(0x10)` is a leaf returning 0 or a fake (true), then
    /// the `delete(3)` fake.
    #[test]
    fn wscript_fn_unhide_matches() {
        oc741_install_fakes();
        fakes::covered(
            ee::s10::f_002ffd58__generated,
            wscript_fn_unhide,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ff_fd58, 2);
                wsc_fixture(c, &mut r, seed, 0x2570, GAME_GP + 0xb30, 1);
                let vt = DATA + 0x9100;
                c.bus.write16(vt + 0x50, ((seed % 3) * 4) as u16);
                c.bus.write32(
                    vt + 0x54,
                    if seed % 4 == 3 {
                        0x0014_06c0
                    } else {
                        fakes::FAKE
                    },
                );
                c.bus.write16(vt + 8, (((seed / 3) % 3) * 4) as u16);
                c.bus.write32(vt + 0xc, fakes::FAKE + 0x10);
                c.pc = 0x002f_fd58;
            },
        );
    }

    /// `WScriptFn_KillRestorable`: init flag, parameters (the expire type is
    /// parameter index 1: 0, 1 or 2), both lookups, the `expire` fakes.
    #[test]
    fn wscript_fn_kill_restorable_matches() {
        oc741_install_fakes();
        fakes::covered(
            ee::s10::f_00300418__generated,
            wscript_fn_kill_restorable,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x300_418, 2);
                wsc_fixture(c, &mut r, seed, 0x2580, GAME_GP + 0xb40, 2);
                let vt = DATA + 0x9100;
                c.bus.write16(vt + 0x110, ((seed % 3) * 4) as u16);
                c.bus.write32(vt + 0x114, fakes::FAKE);
                c.pc = 0x0030_0418;
            },
        );
    }
}
