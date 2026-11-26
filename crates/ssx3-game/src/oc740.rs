//! Staging module oc740 (wave wave242): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc740`.

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
        0x002f_b270,
        world_obj_factory_anim_combo,
        "WorldObjFactory_AnimCombo",
    ),
    (
        0x002f_b6b8,
        world_obj_factory_conveyor,
        "WorldObjFactory_Conveyor",
    ),
    (
        0x002f_bec8,
        world_obj_factory_boost,
        "WorldObjFactory_Boost",
    ),
    (0x002f_cc20, wscript_fn_modifier9, "WScriptFn_Modifier9"),
    (
        0x002f_d758,
        wscript_fn_add_position_modifier,
        "WScriptFn_AddPositionModifier",
    ),
    (
        0x002f_d9f8,
        wscript_fn_add_parent_modifier,
        "WScriptFn_AddParentModifier",
    ),
];

/// `gp` of the game (the world and script-manager globals are gp-relative).
fn wf740_gp(c: &Cpu) -> u32 {
    c.g[GP][0] as u32
}

/// A register as a 32-bit address.
fn wf740_addr(c: &Cpu, r: usize) -> u32 {
    c.g[r][0] as u32
}

/// The `memcpy` the world-object factories copy their template with.
const WF740_MEMCPY: u32 = 0x003e_6574;

/// `WorldObjFactory_AttachModifier` (0x2fae38), reached by a direct call.
const WF740_ATTACH: Callee = Callee {
    addr: 0x002f_ae38,
    f: ee::s10::WorldObjFactory_AttachModifier,
};

/// The parameter loop shared by the factories: each `(idx, val, type)` entry
/// of 16 bytes at `args` (count `cnt`) stores its value into `buf[idx]`, as a
/// float when the type table asks for one and the entry's type does not match.
struct WfFill {
    /// Register holding the stack buffer (`buf`).
    buf: usize,
    /// Register holding the parameter count.
    cnt: usize,
    /// Register holding the parameter list.
    args: usize,
    /// Cycles charged before the loop.
    pre: u64,
    /// Address of the parameter type table.
    table: u32,
    /// Event point of the loop's back edge.
    head: u32,
}

fn wf740_fill(c: &mut Cpu, f: &WfFill) {
    c.g[A2][0] = c.g[f.args][0];
    if (c.g[f.cnt][0] as i64) <= 0 {
        return;
    }
    c.cycles += f.pre;
    c.g[V0][0] = 0x0044_0000;
    c.g[A1][0] = c.g[f.cnt][0];
    c.g[T0][0] = sx(f.table);
    c.g[A3][0] = 2;
    loop {
        c.cycles += 8;
        let ent = wf740_addr(c, A2);
        c.g[V0][0] = Addr(ent).lw(c);
        c.g[V1][0] = Addr(ent.wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[f.buf][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
        let slot = wf740_addr(c, A0);
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
            deliver_event_here(c, f.head);
        }
    }
}

/// The instance lookup of the factories (`S1` = the buffer, `S0` = the result,
/// `tmp` the scratch register the result passes through): the id in `buf[0]`
/// is -1 for the script manager's instance, else a section / entry pair
/// resolved through the world's section table (0 when missing).
fn wf740_lookup_to_s0(c: &mut Cpu, tmp: usize) {
    c.cycles += 4;
    c.g[A2][0] = Addr(wf740_addr(c, S1)).lw(c);
    c.g[V0][0] = !c.g[A2][0];
    c.g[A1][0] = Addr(wf740_gp(c).wrapping_add(0x16c8)).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 3;
        c.g[V0][0] = Addr(wf740_gp(c).wrapping_add(0xce8)).lw(c);
        c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
        return;
    }
    c.cycles += 8;
    c.g[V0][0] = c.g[A2][0] & 0xff;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = Addr(wf740_addr(c, A1)).lw(c);
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(wf740_addr(c, V0)).lw(c);
    if c.g[V1][0] != 0 {
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
        c.cycles += 7;
        c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr(wf740_addr(c, V0)).lw(c);
        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
        let nonzero = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        if !nonzero {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 1;
        c.g[tmp][0] = c.g[V0][0];
    } else {
        c.cycles += 2;
        c.g[tmp][0] = 0;
    }
    c.cycles += 1;
    c.g[S0][0] = c.g[tmp][0];
}

/// The section / entry half of the lookup of the later builders: `A2` is the
/// non-(-1) id and `A1` the world; leaves the instance (0 when missing) in V0.
fn wf740_lookup_tail_v0(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = c.g[A2][0] & 0xff;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = Addr(wf740_addr(c, A1)).lw(c);
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(wf740_addr(c, V0)).lw(c);
    let missing = c.g[V1][0] == 0;
    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
    if !missing {
        c.cycles += 7;
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr(wf740_addr(c, V0)).lw(c);
        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
        let nonzero = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        if nonzero {
            return;
        }
    }
    c.cycles += 1;
    c.g[V0][0] = 0;
}

/// The instance lookup of the later builders, with the id at `buf[0]` (`buf`
/// holds the buffer): -1 is the script manager's instance. Result in V0.
fn wf740_lookup_v0(c: &mut Cpu, buf: usize) {
    c.cycles += 4;
    c.g[A2][0] = Addr(wf740_addr(c, buf)).lw(c);
    c.g[V0][0] = !c.g[A2][0];
    c.g[A1][0] = Addr(wf740_gp(c).wrapping_add(0x16c8)).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 3;
        c.g[V0][0] = Addr(wf740_gp(c).wrapping_add(0xce8)).lw(c);
        c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
        return;
    }
    wf740_lookup_tail_v0(c);
}

/// Epilogue copy of the 12-byte `luno` nil value (at 0x4c9098, from `imm`,
/// the `0x4d0000` register) into the hidden return slot `dst`; the pointer is
/// left in V0.
fn wf740_copy_nil(c: &mut Cpu, imm: usize, dst: usize) {
    c.cycles += 16;
    c.g[A1][0] = sx((c.g[imm][0] as u32).wrapping_sub(0x6f68));
    let src = wf740_addr(c, A1);
    c.ldl(2, src.wrapping_add(7));
    c.ldr(2, src);
    c.g[V1][0] = Addr(src.wrapping_add(8)).lw(c);
    let d = wf740_addr(c, dst);
    c.sdl(2, d.wrapping_add(7));
    c.sdr(2, d);
    let v1 = c.g[V1][0];
    Addr(d.wrapping_add(8)).sw(c, v1);
    c.g[V0][0] = c.g[dst][0];
}

/// The attach-or-allocate tail of a factory: with an instance (`S0`), its
/// active node (+0xc) is offered to `WorldObjFactory_AttachModifier` (`flag`
/// is the node type); when there is none or the attach finds nothing, a new
/// node is allocated (`EA_MemAllocNamed`) and built by its constructor.
struct WfNode {
    /// `A1` for the attach (node type).
    flag: u64,
    attach_ret: u32,
    /// Cycles charged before the allocation.
    alloc_cycles: u64,
    /// Node size and name string (vtable pointer) of the allocation.
    size: u64,
    name: u32,
    alloc_ret: u32,
    /// The node constructor and its return address.
    ctor: u32,
    ctor_ret: u32,
    /// `WorldObjFactory_Boost` hands the constructor `(this, 1, 8, inst, buf)`,
    /// the others `(this, 1, inst, buf)`.
    five_args: bool,
}

fn wf740_attach_or_alloc(c: &mut Cpu, n: &WfNode) -> Flow {
    c.cycles += 2;
    if c.g[S0][0] == 0 {
        return Ok(());
    }
    c.cycles += 3;
    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
    c.g[A1][0] = n.flag;
    if c.g[A0][0] != 0 {
        c.cycles += 4;
        c.g[A2][0] = c.g[S1][0];
        WF740_ATTACH.call(c, n.attach_ret)?;
        if c.g[V0][0] != 0 {
            return Ok(());
        }
    }
    c.cycles += n.alloc_cycles;
    c.g[A0][0] = n.size;
    c.g[A1][0] = sx(n.name);
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    // EA_MemAllocNamed(size, name, 0x20000000, 0).
    call_ptr(c, 0x0031_7d70, n.alloc_ret)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 1;
    if n.five_args {
        c.g[A3][0] = c.g[S0][0];
        c.g[T0][0] = c.g[S1][0];
        c.g[A2][0] = 8;
    } else {
        c.g[A2][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S1][0];
    }
    call_ptr(c, n.ctor, n.ctor_ret)?;
    Ok(())
}

/// What differs between the three `WorldObjFactory_*` builders that share one
/// shape (`AnimCombo`, `Conveyor`, `Boost`).
struct WfSite {
    /// Frame size; `S0` is saved at `frame - 0x10`, `S1` below it, ..., `ra`
    /// at `frame - 0x60`.
    frame: u32,
    /// `gp` offset of the one-time init flag of the static template.
    flag_gp: u32,
    /// The parameter-list register (`S3` or `S4`) and the `0x4d0000` register.
    args: usize,
    imm: usize,
    /// The static template and its size.
    tpl: u32,
    copy_len: u64,
    copy_ret: u32,
    /// The scratch register of the instance lookup.
    tmp: usize,
    fill: WfFill,
    node: WfNode,
}

/// The shared body: prologue, one-time template init (`init`), template copy
/// into the stack buffer, parameter fill, instance lookup, attach, nil value
/// epilogue. Returns the hidden return pointer.
fn wf740_factory(c: &mut Cpu, s: &WfSite, init: fn(&mut Cpu, &WfSite)) -> Flow {
    c.cycles += 13;
    let frame = Frame::open(c, s.frame);
    c.g[V0][0] = Addr(wf740_gp(c).wrapping_add(s.flag_gp)).lw(c);
    for (k, reg) in [S0, S1, S2, S3, S4].into_iter().enumerate() {
        frame.save(c, reg, s.frame - 0x10 - 0x10 * k as u32);
    }
    c.g[s.args][0] = c.g[A2][0];
    c.g[S0][0] = c.g[A1][0];
    c.g[S2][0] = c.g[A0][0];
    c.g[A2][0] = 0x0050_0000;
    let flag_clear = c.g[V0][0] == 0;
    frame.save_ra(c, s.frame - 0x60);
    if flag_clear {
        init(c, s);
    }

    // The id word of the buffer, then the template copy.
    c.cycles += 11;
    c.g[V0][0] = u64::MAX;
    c.g[S1][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A1][0] = sx(s.tpl);
    c.g[A2][0] = s.copy_len;
    let v0 = c.g[V0][0];
    Addr(sp(c).wrapping_add(0x10)).sw(c, v0);
    c.g[A0][0] = c.g[S1][0];
    c.g[s.imm][0] = 0x004d_0000;
    call_ptr(c, WF740_MEMCPY, s.copy_ret)?;

    wf740_fill(c, &s.fill);
    wf740_lookup_to_s0(c, s.tmp);
    wf740_attach_or_alloc(c, &s.node)?;
    wf740_copy_nil(c, s.imm, S2);
    for (k, reg) in [S0, S1, S2, S3, S4].into_iter().enumerate() {
        frame.restore(c, reg, s.frame - 0x10 - 0x10 * k as u32);
    }
    frame.restore_ra(c, s.frame - 0x60);
    frame.close(c);
    Ok(())
}

/// `WorldObjFactory_AnimCombo`'s constants (template at `0x4fb3c0`, 60 bytes).
const WF740_ANIM: WfSite = WfSite {
    frame: 0xb0,
    flag_gp: 0x24d4,
    args: S4,
    imm: S3,
    tpl: 0x004f_b3c0,
    copy_len: 0x3c,
    copy_ret: 0x002f_b338,
    tmp: A2,
    fill: WfFill {
        buf: S1,
        cnt: S0,
        args: S4,
        pre: 4,
        table: 0x0044_5e98,
        head: 0x002f_b350,
    },
    node: WfNode {
        flag: 3,
        attach_ret: 0x002f_b420,
        alloc_cycles: 11,
        size: 0x88,
        name: 0x0048_9668,
        alloc_ret: 0x002f_b440,
        ctor: 0x0035_8c80,
        ctor_ret: 0x002f_b454,
        five_args: false,
    },
};

/// `WorldObjFactory_Conveyor`'s constants (template at `0x4fb430`, 52 bytes).
const WF740_CONVEYOR: WfSite = WfSite {
    frame: 0xb0,
    flag_gp: 0x24dc,
    args: S3,
    imm: S4,
    tpl: 0x004f_b430,
    copy_len: 0x34,
    copy_ret: 0x002f_b76c,
    tmp: A2,
    fill: WfFill {
        buf: S1,
        cnt: S0,
        args: S3,
        pre: 5,
        table: 0x0044_5ef8,
        head: 0x002f_b788,
    },
    node: WfNode {
        flag: 0xf,
        attach_ret: 0x002f_b858,
        alloc_cycles: 11,
        size: 0x48,
        name: 0x0048_9688,
        alloc_ret: 0x002f_b878,
        ctor: 0x0034_3010,
        ctor_ret: 0x002f_b88c,
        five_args: false,
    },
};

/// `WorldObjFactory_Boost`'s constants (template at `0x4fb4c8`, 36 bytes).
const WF740_BOOST: WfSite = WfSite {
    frame: 0xa0,
    flag_gp: 0x24ec,
    args: S3,
    imm: S4,
    tpl: 0x004f_b4c8,
    copy_len: 0x24,
    copy_ret: 0x002f_bf6c,
    tmp: A3,
    fill: WfFill {
        buf: S1,
        cnt: S0,
        args: S3,
        pre: 5,
        table: 0x0044_5f70,
        head: 0x002f_bf88,
    },
    node: WfNode {
        flag: 8,
        attach_ret: 0x002f_c058,
        alloc_cycles: 12,
        size: 0x50,
        name: 0x004a_3bf0,
        alloc_ret: 0x002f_c078,
        ctor: 0x0034_1388,
        ctor_ret: 0x002f_c090,
        five_args: true,
    },
};

/// One-time init of the anim-combo template (flag clear): the id word, the
/// type/value fields (-1.0, 30.0, 31.0 and zeros) and the flag.
fn wf740_anim_init(c: &mut Cpu, s: &WfSite) {
    c.cycles += 28;
    let tpl = s.tpl;
    c.g[V0][0] = sx(tpl);
    c.g[V1][0] = 0xffff_ffff_ffff_0000;
    c.g[A1][0] = c.lbu(tpl.wrapping_add(0x38));
    c.g[V1][0] |= 0xffff;
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(AT, 2);
    c.g[A0][0] = 1;
    c.g[AT][0] = 0x41f0_0000;
    c.mtc1(AT, 0);
    c.g[AT][0] = 0x41f8_0000;
    c.mtc1(AT, 1);
    let v1 = c.g[V1][0];
    c.sw(tpl, v1);
    let a1 = c.g[A1][0];
    c.sb(tpl.wrapping_add(0x38), a1);
    c.swc1(1, tpl.wrapping_add(0x28));
    c.swc1(2, tpl.wrapping_add(0x2c));
    c.swc1(0, tpl.wrapping_add(0x30));
    let a0 = c.g[A0][0];
    c.sw(wf740_gp(c).wrapping_add(s.flag_gp), a0);
    c.sw(tpl.wrapping_add(4), a0);
    c.sw(tpl.wrapping_add(8), 0);
    c.swc1(2, tpl.wrapping_add(0xc));
    c.swc1(0, tpl.wrapping_add(0x10));
    c.swc1(0, tpl.wrapping_add(0x14));
    c.sw(tpl.wrapping_add(0x18), 0);
    c.swc1(2, tpl.wrapping_add(0x1c));
    c.sw(tpl.wrapping_add(0x20), 0);
    c.sw(tpl.wrapping_add(0x24), 0);
    c.sw(tpl.wrapping_add(0x34), 0);
    c.sb(sp(c), a1);
}

/// One-time init of the conveyor template: the id word, 100.0f at +4 and zeros
/// in the other fields, and the flag.
fn wf740_conveyor_init(c: &mut Cpu, s: &WfSite) {
    c.cycles += 23;
    let tpl = s.tpl;
    c.g[V0][0] = sx(tpl);
    c.g[V1][0] = 0xffff_ffff_ffff_0000;
    c.g[A1][0] = c.lbu(tpl.wrapping_add(0x30));
    c.g[V1][0] |= 0xffff;
    c.mtc1(0, 1);
    c.g[A0][0] = 1;
    c.g[AT][0] = 0x42c8_0000;
    c.mtc1(AT, 0);
    let v1 = c.g[V1][0];
    c.sw(tpl, v1);
    let a1 = c.g[A1][0];
    c.sb(tpl.wrapping_add(0x30), a1);
    c.swc1(0, tpl.wrapping_add(4));
    c.swc1(1, tpl.wrapping_add(0xc));
    let a0 = c.g[A0][0];
    c.sw(wf740_gp(c).wrapping_add(s.flag_gp), a0);
    c.swc1(1, tpl.wrapping_add(8));
    for off in [0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c] {
        c.sw(tpl.wrapping_add(off), 0);
    }
    c.sb(sp(c), a1);
}

/// One-time init of the boost template: the id word, 1.0f and 0.0f fields.
fn wf740_boost_init(c: &mut Cpu, s: &WfSite) {
    c.cycles += 19;
    let tpl = s.tpl;
    c.g[V0][0] = sx(tpl);
    c.g[V1][0] = 0xffff_ffff_ffff_0000;
    c.g[A1][0] = c.lbu(tpl.wrapping_add(0x20));
    c.g[V1][0] |= 0xffff;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 1);
    c.g[A0][0] = 1;
    c.mtc1(0, 0);
    let v1 = c.g[V1][0];
    c.sw(tpl, v1);
    let a1 = c.g[A1][0];
    c.sb(tpl.wrapping_add(0x20), a1);
    c.swc1(1, tpl.wrapping_add(0x18));
    c.swc1(0, tpl.wrapping_add(0x1c));
    let a0 = c.g[A0][0];
    c.sw(wf740_gp(c).wrapping_add(s.flag_gp), a0);
    c.sw(tpl.wrapping_add(4), 0);
    c.swc1(1, tpl.wrapping_add(8));
    c.swc1(0, tpl.wrapping_add(0xc));
    c.swc1(1, tpl.wrapping_add(0x10));
    c.swc1(0, tpl.wrapping_add(0x14));
    c.sb(sp(c), a1);
}

/// `WorldObjFactory_AnimCombo` (0x2fb270), the C++ `MakeAnimComboFunc`:
/// builds an anim-combo parameter record in a 60-byte stack buffer (the
/// static template, then the script's parameters), finds the target instance
/// by id and attaches an `AnimCombo` node to it (node type 3), allocating one
/// when the instance has no active node that takes it. Returns the `luno` nil
/// value through the hidden pointer in `a0`.
pub fn world_obj_factory_anim_combo(c: &mut Cpu) {
    guest(c, |c| wf740_factory(c, &WF740_ANIM, wf740_anim_init));
}

/// `WorldObjFactory_Conveyor` (0x2fb6b8), the C++ `MakeConveyorFunc`: the
/// anim-combo's shape with a 52-byte buffer (speed 100.0f) and a 0x48-byte
/// conveyor node (node type 15).
pub fn world_obj_factory_conveyor(c: &mut Cpu) {
    guest(c, |c| {
        wf740_factory(c, &WF740_CONVEYOR, wf740_conveyor_init)
    });
}

/// `WorldObjFactory_Boost` (0x2fbec8), the C++ `MakeBoostFunc`: the same
/// shape with a 36-byte buffer and a 0x50-byte boost node (node type 8); its
/// constructor takes the extra argument 8.
pub fn world_obj_factory_boost(c: &mut Cpu) {
    guest(c, |c| wf740_factory(c, &WF740_BOOST, wf740_boost_init));
}

/// `WScriptFn_Modifier9` (0x2fcc20), the C++ `MakeFenceFunc`: builds a 16-byte
/// parameter record (the template with 0.5f at +8, then the script's
/// parameters), finds the instance by id (-1: the script manager's) and, when
/// it has an active node, offers the record to `WorldObjFactory_AttachModifier`
/// (type 9). Returns the `luno` nil value through the hidden pointer in `a0`.
pub fn wscript_fn_modifier9(c: &mut Cpu) {
    guest(c, |c| {
        const TEMPLATE: u32 = 0x004f_b5b0;
        c.cycles += 13;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = Addr(wf740_gp(c).wrapping_add(0x2504)).lw(c);
        frame.save(c, S3, 0x40);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save(c, S0, 0x70);
        frame.save(c, S4, 0x30);
        c.g[S3][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        c.g[A2][0] = 0x0050_0000;
        let flag_clear = c.g[V0][0] == 0;
        frame.save_ra(c, 0x20);
        if flag_clear {
            // One-time init of the template and its flag.
            c.cycles += 13;
            c.g[V1][0] = sx(TEMPLATE);
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A1][0] = c.lbu(TEMPLATE.wrapping_add(0xc));
            c.g[V0][0] |= 0xffff;
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(AT, 0);
            c.g[A0][0] = 1;
            let v0 = c.g[V0][0];
            c.sw(TEMPLATE, v0);
            let a1 = c.g[A1][0];
            c.sb(TEMPLATE.wrapping_add(0xc), a1);
            c.swc1(0, TEMPLATE.wrapping_add(8));
            let a0 = c.g[A0][0];
            c.sw(wf740_gp(c).wrapping_add(0x2504), a0);
            c.sw(TEMPLATE.wrapping_add(4), 0);
            c.sb(sp(c), a1);
        }

        // The id word of the buffer, then the template copy.
        c.cycles += 11;
        c.g[V0][0] = u64::MAX;
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A1][0] = sx(TEMPLATE);
        c.g[A2][0] = 0x10;
        let v0 = c.g[V0][0];
        Addr(sp(c).wrapping_add(0x10)).sw(c, v0);
        c.g[A0][0] = c.g[S0][0];
        c.g[S4][0] = 0x004d_0000;
        call_ptr(c, WF740_MEMCPY, 0x002f_ccac)?;

        wf740_fill(
            c,
            &WfFill {
                buf: S0,
                cnt: S2,
                args: S3,
                pre: 5,
                table: 0x0044_6028,
                head: 0x002f_ccc8,
            },
        );
        wf740_lookup_v0(c, S0);

        // With an instance whose active node exists: attach (type 9).
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
            c.g[A2][0] = c.g[S0][0];
            if c.g[A0][0] != 0 {
                c.cycles += 2;
                c.g[A1][0] = 9;
                WF740_ATTACH.call(c, 0x002f_cd88)?;
            }
        }
        wf740_copy_nil(c, S4, S1);
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    });
}

/// `WScriptFn_AddPositionModifier` (0x2fd758), the C++ `AddRiderPositionFunc`:
/// builds a 24-byte parameter record, finds the instance by id and, when its
/// active node answers the type query, moves the node by the position of a
/// second object: the target object's two vtable queries, the rotated offset
/// (`VU0` matrix times the record's vector) added to the position, and
/// `ModObj_AddPositionModifier`. Returns the `luno` nil value through the
/// hidden pointer in `a0`.
pub fn wscript_fn_add_position_modifier(c: &mut Cpu) {
    guest(c, |c| {
        const TEMPLATE: u32 = 0x004f_b720;
        c.cycles += 12;
        let frame = Frame::open(c, 0xc0);
        c.g[V0][0] = Addr(wf740_gp(c).wrapping_add(0x2518)).lw(c);
        frame.save(c, S1, 0xa0);
        c.g[A3][0] = 0x0050_0000;
        frame.save(c, S2, 0x90);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x80);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0xb0);
        c.g[S3][0] = c.g[A0][0];
        let flag_clear = c.g[V0][0] == 0;
        frame.save_ra(c, 0x70);
        if flag_clear {
            // One-time init of the template and its flag.
            c.cycles += 15;
            c.g[V1][0] = sx(TEMPLATE);
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A2][0] = c.lbu(TEMPLATE.wrapping_add(0x14));
            c.g[V0][0] |= 0xffff;
            c.mtc1(0, 0);
            c.g[A0][0] = u64::MAX;
            let v0 = c.g[V0][0];
            c.sw(TEMPLATE, v0);
            c.g[A1][0] = 1;
            let a2 = c.g[A2][0];
            c.sb(TEMPLATE.wrapping_add(0x14), a2);
            let a0 = c.g[A0][0];
            c.sw(TEMPLATE.wrapping_add(4), a0);
            c.swc1(0, TEMPLATE.wrapping_add(0x10));
            let a1 = c.g[A1][0];
            c.sw(wf740_gp(c).wrapping_add(0x2518), a1);
            c.swc1(0, TEMPLATE.wrapping_add(8));
            c.swc1(0, TEMPLATE.wrapping_add(0xc));
            c.sb(sp(c), a2);
        }

        // The id word of the buffer, then the template copy.
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A2][0] = 0x18;
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x48e0));
        let v0 = c.g[V0][0];
        Addr(sp(c).wrapping_add(0x10)).sw(c, v0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, WF740_MEMCPY, 0x002f_d7e4)?;

        wf740_fill(
            c,
            &WfFill {
                buf: S0,
                cnt: S1,
                args: S2,
                pre: 5,
                table: 0x0044_6190,
                head: 0x002f_d800,
            },
        );
        wf740_lookup_v0(c, S0);

        // The instance's active node (S1), if any: ask it whether it is a
        // movable node, then place it relative to the target object.
        c.cycles += 3;
        c.g[S1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[V1][0] = 0x004d_0000;
        if c.g[S1][0] != 0 {
            c.cycles += 7;
            c.g[V1][0] = Addr(wf740_addr(c, S1).wrapping_add(0xc)).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x80)).lh(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
            let target = wf740_addr(c, V0);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x002f_d8c4)?;
            let is_node = c.g[V0][0] != 0;
            c.g[V0][0] = u64::MAX;
            if is_node {
                c.cycles += 3;
                c.g[A1][0] = Addr(wf740_addr(c, S0).wrapping_add(4)).lw(c);
                let same = c.g[A1][0] == c.g[V0][0];
                c.g[A0][0] = Addr(wf740_gp(c).wrapping_sub(0x848)).lw(c);
                if !same {
                    wf740_position_move(c)?;
                }
            }
            c.cycles += 1;
            c.g[V1][0] = 0x004d_0000;
        }

        // Epilogue: copy the nil value to the hidden return slot (S3).
        c.cycles += 15;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        let src = wf740_addr(c, A3);
        c.ldl(4, src.wrapping_add(7));
        c.ldr(4, src);
        c.g[A1][0] = Addr(src.wrapping_add(8)).lw(c);
        let dst = wf740_addr(c, S3);
        c.sdl(4, dst.wrapping_add(7));
        c.sdr(4, dst);
        let a1 = c.g[A1][0];
        Addr(dst.wrapping_add(8)).sw(c, a1);
        frame.restore(c, S0, 0xb0);
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore_ra(c, 0x70);
        frame.close(c);
        Ok(())
    });
}

/// The position part of `WScriptFn_AddPositionModifier` (the world is in
/// `A0`): the target object's two world-table queries (its object, then its
/// matrix), the `VU0` product of the matrix with the record's offset vector,
/// the sum with the object's position, and `ModObj_AddPositionModifier`.
fn wf740_position_move(c: &mut Cpu) -> Flow {
    c.cycles += 55;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[V1][0] = Addr(wf740_addr(c, A0).wrapping_add(0x84)).lw(c);
    c.g[V0][0] = Addr(wf740_addr(c, V1).wrapping_add(0xc)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V1][0] = Addr(wf740_addr(c, V0).wrapping_add(0x28)).lw(c);
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6c0));
    if c.g[V1][0] == 0 {
        c.g[A0][0] = 0;
    }
    c.g[A1][0] = Addr(wf740_addr(c, A0)).lw(c);
    c.g[V1][0] = Addr(wf740_addr(c, A1).wrapping_add(0x28)).lh(c);
    c.g[V0][0] = Addr(wf740_addr(c, A1).wrapping_add(0x2c)).lw(c);
    let target = wf740_addr(c, V0);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    call_ptr(c, target, 0x002f_d908)?;
    c.lq(3, wf740_addr(c, V0));
    c.g[A0][0] = Addr(wf740_gp(c).wrapping_sub(0x848)).lw(c);
    c.sq(3, sp(c).wrapping_add(0x30));
    c.g[A1][0] = Addr(wf740_addr(c, A0).wrapping_add(0x84)).lw(c);
    c.g[V0][0] = Addr(wf740_addr(c, S0).wrapping_add(4)).lw(c);
    c.g[V1][0] = Addr(wf740_addr(c, A1).wrapping_add(0xc)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A1][0] = Addr(wf740_addr(c, V1).wrapping_add(0x28)).lw(c);
    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c0));
    if c.g[A1][0] == 0 {
        c.g[A0][0] = 0;
    }
    c.g[V1][0] = Addr(wf740_addr(c, A0)).lw(c);
    c.g[A1][0] = Addr(wf740_addr(c, V1).wrapping_add(0x30)).lh(c);
    c.g[V0][0] = Addr(wf740_addr(c, V1).wrapping_add(0x34)).lw(c);
    let target = wf740_addr(c, V0);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    call_ptr(c, target, 0x002f_d948)?;
    let s0 = wf740_addr(c, S0);
    c.lwc1(2, s0.wrapping_add(0x10));
    c.g[A0][0] = c.g[S1][0];
    c.lwc1(1, s0.wrapping_add(8));
    c.g[A1][0] = sx(sp(c).wrapping_add(0x30));
    c.lwc1(0, s0.wrapping_add(0xc));
    c.swc1(1, sp(c).wrapping_add(0x50));
    c.swc1(0, sp(c).wrapping_add(0x54));
    c.swc1(2, sp(c).wrapping_add(0x58));
    c.sw(sp(c).wrapping_add(0x5c), 0);
    c.lqc2(8, sp(c).wrapping_add(0x50));
    let v0 = wf740_addr(c, V0);
    c.lqc2(4, v0);
    c.lqc2(5, v0.wrapping_add(0x10));
    c.lqc2(6, v0.wrapping_add(0x20));
    c.lqc2(7, v0.wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, sp(c).wrapping_add(0x60));
    c.lq(2, sp(c).wrapping_add(0x60));
    c.sq(2, sp(c).wrapping_add(0x40));
    c.lqc2(3, sp(c).wrapping_add(0x30));
    c.lqc2(4, sp(c).wrapping_add(0x40));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp(c).wrapping_add(0x30));
    // ModObj_AddPositionModifier(node, position).
    call_ptr(c, 0x0035_58b8, 0x002f_d9b4)?;
    Ok(())
}

/// `WScriptFn_AddParentModifier` (0x2fd9f8), the C++ `AddParentFunc`: builds a
/// 28-byte parameter record whose first two words are object ids, resolves
/// both (the child and the parent), asks the parent's active node whether it
/// takes the child and, when the child's node is a movable one, calls
/// `ModObj_AddParentModifier`. Returns the `luno` nil value through the
/// hidden pointer in `a0`.
pub fn wscript_fn_add_parent_modifier(c: &mut Cpu) {
    guest(c, |c| {
        const TEMPLATE: u32 = 0x004f_b738;
        c.cycles += 12;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = Addr(wf740_gp(c).wrapping_add(0x251c)).lw(c);
        frame.save(c, S2, 0x60);
        frame.save(c, S0, 0x80);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S3, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x70);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x40);
        let flag_clear = c.g[V0][0] == 0;
        c.g[A2][0] = 0x0050_0000;
        if flag_clear {
            // One-time init of the template and its flag.
            c.cycles += 15;
            c.g[V0][0] = sx(TEMPLATE);
            c.g[V1][0] = 0xffff_ffff_ffff_0000;
            c.g[A1][0] = c.lbu(TEMPLATE.wrapping_add(0x18));
            c.g[V1][0] |= 0xffff;
            c.mtc1(0, 0);
            c.g[A0][0] = 1;
            let v1 = c.g[V1][0];
            c.sw(TEMPLATE, v1);
            let a1 = c.g[A1][0];
            c.sb(TEMPLATE.wrapping_add(0x18), a1);
            c.sw(TEMPLATE.wrapping_add(4), v1);
            c.swc1(0, TEMPLATE.wrapping_add(0x14));
            let a0 = c.g[A0][0];
            c.sw(wf740_gp(c).wrapping_add(0x251c), a0);
            c.sw(TEMPLATE.wrapping_add(8), 0);
            c.swc1(0, TEMPLATE.wrapping_add(0xc));
            c.swc1(0, TEMPLATE.wrapping_add(0x10));
            c.sb(sp(c), a1);
        }

        // Both id words of the buffer, then the template copy.
        c.cycles += 11;
        c.g[V0][0] = u64::MAX;
        c.g[S1][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A1][0] = sx(TEMPLATE);
        c.g[A2][0] = 0x1c;
        let v0 = c.g[V0][0];
        Addr(sp(c).wrapping_add(0x14)).sw(c, v0);
        c.g[A0][0] = c.g[S1][0];
        Addr(sp(c).wrapping_add(0x10)).sw(c, v0);
        call_ptr(c, WF740_MEMCPY, 0x002f_da88)?;

        wf740_fill(
            c,
            &WfFill {
                buf: S1,
                cnt: S0,
                args: S2,
                pre: 4,
                table: 0x0044_61a8,
                head: 0x002f_daa0,
            },
        );

        // The child instance (id in buf[0]) into S0.
        c.cycles += 4;
        c.g[A2][0] = Addr(wf740_addr(c, S1)).lw(c);
        c.g[V0][0] = !c.g[A2][0];
        c.g[A1][0] = Addr(wf740_gp(c).wrapping_add(0x16c8)).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(wf740_gp(c).wrapping_add(0xce8)).lw(c);
            c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
        } else {
            wf740_lookup_tail_v0(c);
            c.cycles += 1;
            c.g[S0][0] = c.g[V0][0];
        }

        // The parent (id in buf[1]); with no parent the work is skipped.
        'done: {
            'exit: {
                'parent: {
                    c.cycles += 4;
                    c.g[A2][0] = Addr(wf740_addr(c, S1).wrapping_add(4)).lw(c);
                    c.g[V0][0] = !c.g[A2][0];
                    let no_parent = c.g[V0][0] == 0;
                    c.g[A1][0] = Addr(wf740_gp(c).wrapping_add(0x16c8)).lw(c);
                    if no_parent {
                        break 'parent;
                    }
                    wf740_lookup_tail_v0(c);
                    // The parent's active node (S2).
                    c.cycles += 2;
                    let missing = c.g[V0][0] == 0;
                    c.g[S2][0] = 0;
                    if !missing {
                        c.cycles += 1;
                        c.g[S2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
                    }
                    c.cycles += 2;
                    let no_node = c.g[S2][0] == 0;
                    c.g[A1][0] = 6;
                    if no_node {
                        break 'parent;
                    }
                    c.cycles += 7;
                    c.g[V1][0] = Addr(wf740_addr(c, S2).wrapping_add(0xc)).lw(c);
                    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x50)).lh(c);
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x54)).lw(c);
                    let target = wf740_addr(c, V0);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x002f_dbc0)?;
                    let takes = c.g[V0][0] != 0;
                    c.g[V1][0] = 0x004d_0000;
                    if takes {
                        break 'exit;
                    }
                    // The child's node (S0).
                    c.cycles += 3;
                    c.g[S0][0] = Addr(wf740_addr(c, S0).wrapping_add(0xc)).lw(c);
                    let no_child = c.g[S0][0] == 0;
                    c.g[V0][0] = c.g[S3][0];
                    if no_child {
                        break 'done;
                    }
                    c.cycles += 7;
                    c.g[V1][0] = Addr(wf740_addr(c, S0).wrapping_add(0xc)).lw(c);
                    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x80)).lh(c);
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
                    let target = wf740_addr(c, V0);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x002f_dbe8)?;
                    let not_movable = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S0][0];
                    if not_movable {
                        break 'parent;
                    }
                    c.cycles += 11;
                    let s1 = wf740_addr(c, S1);
                    c.lwc1(2, s1.wrapping_add(0xc));
                    c.lwc1(0, s1.wrapping_add(0x10));
                    c.g[A3][0] = sx(sp(c).wrapping_add(0x30));
                    c.lwc1(1, s1.wrapping_add(0x14));
                    c.g[A1][0] = Addr(wf740_addr(c, S2).wrapping_add(0x18)).lw(c);
                    c.swc1(2, sp(c).wrapping_add(0x30));
                    c.swc1(0, sp(c).wrapping_add(0x34));
                    c.swc1(1, sp(c).wrapping_add(0x38));
                    c.sw(sp(c).wrapping_add(0x3c), 0);
                    c.g[A2][0] = Addr(s1.wrapping_add(8)).lw(c);
                    // ModObj_AddParentModifier(child node, parent object, ..).
                    call_ptr(c, 0x0035_5978, 0x002f_dc1c)?;
                }
                c.cycles += 1;
                c.g[V1][0] = 0x004d_0000;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
        }

        // Epilogue: copy the nil value to the hidden return slot (S3).
        c.cycles += 14;
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        let src = wf740_addr(c, A3);
        c.ldl(4, src.wrapping_add(7));
        c.ldr(4, src);
        c.g[A1][0] = Addr(src.wrapping_add(8)).lw(c);
        let dst = wf740_addr(c, S3);
        c.sdl(4, dst.wrapping_add(7));
        c.sdr(4, dst);
        let a1 = c.g[A1][0];
        Addr(dst.wrapping_add(8)).sw(c, a1);
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    });
}

// @@FUNCS@@

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    const SEEDS: u32 = 240;

    /// Gate of the callee fakes: a magic word in the machine's RAM turns them
    /// on (both sides of a diff get their own machine); `CTRL + 4` is the
    /// result the attach fake returns.
    const OC740_MAGIC: u32 = 0x0740_0740;
    const OC740_CTRL: u32 = DATA + 0x20_0000;
    /// A real leaf that returns 0 (the vtable "is it a ..." queries).
    const OC740_LEAF_ZERO: u32 = 0x0014_06c0;

    macro_rules! oc740_fake {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC740_CTRL) != OC740_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $addr, ra);
            }
        };
    }

    static OC740_PREV_ATTACH: AtomicUsize = AtomicUsize::new(0);

    /// Gated fake of `WorldObjFactory_AttachModifier`: the logged call, then
    /// the result the fixture asked for (`CTRL + 4`: 0 or 1).
    fn oc740_fake_attach(c: &mut Cpu) {
        if c.bus.read32(OC740_CTRL) != OC740_MAGIC {
            return fakes::fall_back(c, &OC740_PREV_ATTACH, ee::s10::f_002fae38);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x002f_ae38, ra);
        if c.pc == ra {
            c.g[V0][0] = c.bus.read32(OC740_CTRL + 4) as u64;
        }
    }
    oc740_fake!(
        oc740_fake_alloc,
        OC740_PREV_ALLOC,
        0x0031_7d70,
        ee::s10::f_00317d70
    );
    oc740_fake!(
        oc740_fake_anim_ctor,
        OC740_PREV_ANIM_CTOR,
        0x0035_8c80,
        ee::s11::f_00358c80
    );
    oc740_fake!(
        oc740_fake_conv_ctor,
        OC740_PREV_CONV_CTOR,
        0x0034_3010,
        ee::s11::f_00343010
    );
    oc740_fake!(
        oc740_fake_boost_ctor,
        OC740_PREV_BOOST_CTOR,
        0x0034_1388,
        ee::s11::f_00341388
    );
    oc740_fake!(
        oc740_fake_position,
        OC740_PREV_POSITION,
        0x0035_58b8,
        ee::s11::f_003558b8
    );
    oc740_fake!(
        oc740_fake_parent,
        OC740_PREV_PARENT,
        0x0035_5978,
        ee::s11::f_00355978
    );

    fn oc740_hook() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_002FAE38),
            oc740_fake_attach,
            &OC740_PREV_ATTACH,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc740_fake_alloc,
            &OC740_PREV_ALLOC,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00358C80),
            oc740_fake_anim_ctor,
            &OC740_PREV_ANIM_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00343010),
            oc740_fake_conv_ctor,
            &OC740_PREV_CONV_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00341388),
            oc740_fake_boost_ctor,
            &OC740_PREV_BOOST_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_003558B8),
            oc740_fake_position,
            &OC740_PREV_POSITION,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00355978),
            oc740_fake_parent,
            &OC740_PREV_PARENT,
        );
    }

    /// An instance record at `inst` whose active node (+0xc) is `node` (0: none).
    /// A node's vtable (+0xc of the node) answers the type queries at the
    /// offsets of `fns` (`(adjust offset, fn offset)`): each calls `FAKE` (a
    /// fake returning a pointer) or the leaf that returns 0.
    fn oc740_instance(c: &mut Cpu, r: &mut Rng, inst: u32, node: u32, fns: &[(u32, u32)]) {
        c.bus.write32(inst + 0xc, node);
        if node == 0 {
            return;
        }
        let vt = node + 0x100;
        c.bus.write32(node + 0xc, vt);
        for (i, &(adj, f)) in fns.iter().enumerate() {
            c.bus.write16(vt + adj, (r.next_u32() % 8 * 4) as u16);
            let target = if r.next_u32() % 2 == 0 {
                OC740_LEAF_ZERO
            } else {
                fakes::FAKE + 0x10 * i as u32
            };
            c.bus.write32(vt + f, target);
        }
    }

    /// The world the factories resolve ids through: id 0 -> instance
    /// `DATA + 0x8000` (section 0 / entry 0), id 0x100 -> a missing entry,
    /// id 5 -> a missing section; the script manager's instance (id -1) at
    /// `DATA + 0xb000`. Both have an active node on `seed` bits.
    fn oc740_world(c: &mut Cpu, r: &mut Rng, seed: u32, fns: &[(u32, u32)]) {
        c.bus.write32(GAME_GP + 0x16c8, DATA + 0x3000);
        c.bus.write32(DATA + 0x3000, DATA + 0x4000);
        c.bus.write32(DATA + 0x4008, DATA + 0x5000);
        c.bus.write32(DATA + 0x5000, DATA + 0x6000);
        c.bus.write32(DATA + 0x601c, DATA + 0x6100);
        c.bus.write32(DATA + 0x6100, ((DATA + 0x8000) >> 2) << 8);
        let node = if (seed / 8) % 4 == 0 {
            0
        } else {
            DATA + 0x9000
        };
        oc740_instance(c, r, DATA + 0x8000, node, fns);
        c.bus.write32(GAME_GP + 0xce8, DATA + 0xa000);
        c.bus.write32(DATA + 0xa290, DATA + 0xb000);
        let node = if (seed / 32) % 4 == 0 {
            0
        } else {
            DATA + 0xb100
        };
        oc740_instance(c, r, DATA + 0xb000, node, fns);
    }

    /// The script's parameter list at `DATA + 0x1000` (16-byte entries
    /// `(idx, val, ?, type)`): first the `forced` `(idx, value)` entries
    /// (stored as integers: their type equals the table's), then `seed % 3`
    /// random ones; and the type table at `types`.
    fn oc740_params(
        c: &mut Cpu,
        r: &mut Rng,
        seed: u32,
        types: u32,
        maxidx: u32,
        forced: &[(u32, u32)],
    ) {
        let table: Vec<u32> = (0..=maxidx).map(|_| r.next_u32() % 3).collect();
        for (i, &t) in table.iter().enumerate() {
            c.bus.write32(types + 4 * i as u32, t);
        }
        let n = forced.len() as u32 + seed % 3;
        c.g[A1][0] = n as u64;
        c.g[A2][0] = sx(DATA + 0x1000);
        for i in 0..n {
            let e = DATA + 0x1000 + 16 * i;
            let (idx, val, ty) = if (i as usize) < forced.len() {
                let (idx, val) = forced[i as usize];
                (idx, val, table[idx as usize])
            } else {
                (
                    2 + r.next_u32() % (maxidx - 1),
                    r.next_u32(),
                    r.next_u32() % 3,
                )
            };
            c.bus.write32(e, idx);
            c.bus.write32(e + 4, val);
            c.bus.write32(e + 0xc, ty);
        }
    }

    /// The id of the target instance a seed asks for: `None` leaves the
    /// template's (-1: the script manager's), else a forced id parameter.
    fn oc740_id(seed: u32) -> Option<u32> {
        match seed % 5 {
            0 | 1 => None,
            2 => Some(0),
            3 => Some(0x100),
            _ => Some(5),
        }
    }

    /// The machine of the three factories: the template's id word and
    /// `flag_gp`, the parameters, the world, and the gated fakes.
    fn oc740_factory_machine(c: &mut Cpu, seed: u32, salt: u64, site: &WfSite) {
        oc740_hook();
        let mut r = machine(c, seed, salt, 3);
        c.bus.write32(OC740_CTRL, OC740_MAGIC);
        c.bus.write32(OC740_CTRL + 4, (seed / 16) % 2);
        c.g[A0][0] = sx(DATA + 0x7000);
        let forced: Vec<(u32, u32)> = oc740_id(seed).map(|v| (0, v)).into_iter().collect();
        oc740_params(c, &mut r, seed, site.fill.table, 5, &forced);
        c.bus
            .write32(site.tpl, if (seed / 4) % 2 == 0 { u32::MAX } else { 0 });
        c.bus.write32(GAME_GP + site.flag_gp, seed % 2);
        oc740_world(c, &mut r, seed, &[]);
    }

    /// Covers the one-time init (flag clear or set), the parameter loop (int
    /// and float stores), every instance lookup exit (-1, id 0, missing entry,
    /// missing section) and the attach / alloc / constructor tails with the
    /// fakes returning and unwinding.
    #[test]
    fn world_obj_factory_anim_combo_matches() {
        fakes::covered(
            ee::s10::f_002fb270__generated,
            world_obj_factory_anim_combo,
            SEEDS,
            &|c, seed| {
                oc740_factory_machine(c, seed, 0x2fb270, &WF740_ANIM);
            },
        );
    }

    #[test]
    fn world_obj_factory_conveyor_matches() {
        fakes::covered(
            ee::s10::f_002fb6b8__generated,
            world_obj_factory_conveyor,
            SEEDS,
            &|c, seed| {
                oc740_factory_machine(c, seed, 0x2fb6b8, &WF740_CONVEYOR);
            },
        );
    }

    #[test]
    fn world_obj_factory_boost_matches() {
        fakes::covered(
            ee::s10::f_002fbec8__generated,
            world_obj_factory_boost,
            SEEDS,
            &|c, seed| {
                oc740_factory_machine(c, seed, 0x2fbec8, &WF740_BOOST);
            },
        );
    }

    /// `WScriptFn_Modifier9`: the template, the 16-byte buffer, the lookup and
    /// the attach (type 9) when the instance has an active node.
    #[test]
    fn wscript_fn_modifier9_matches() {
        fakes::covered(
            ee::s10::f_002fcc20__generated,
            wscript_fn_modifier9,
            SEEDS,
            &|c, seed| {
                oc740_hook();
                let mut r = machine(c, seed, 0x2fcc20, 2);
                c.bus.write32(OC740_CTRL, OC740_MAGIC);
                c.bus.write32(OC740_CTRL + 4, (seed / 16) % 2);
                // a0: hidden return slot, a1: parameter count, a2: parameters.
                c.g[A0][0] = sx(DATA + 0x7000);
                let forced: Vec<(u32, u32)> = oc740_id(seed).map(|v| (0, v)).into_iter().collect();
                oc740_params(c, &mut r, seed, 0x0044_6028, 3, &forced);
                c.bus
                    .write32(0x004f_b5b0, if (seed / 4) % 2 == 0 { u32::MAX } else { 0 });
                c.bus.write32(GAME_GP + 0x2504, seed % 2);
                oc740_world(c, &mut r, seed, &[]);
            },
        );
    }

    /// `WScriptFn_AddPositionModifier`: lookup, the node's type query (a real
    /// leaf returning 0 or a fake), the target id (-1 or an object index),
    /// the object table, the two vtable queries, the `VU0` product and the
    /// position-modifier call.
    #[test]
    fn wscript_fn_add_position_modifier_matches() {
        fakes::covered(
            ee::s10::f_002fd758__generated,
            wscript_fn_add_position_modifier,
            SEEDS,
            &|c, seed| {
                oc740_hook();
                let mut r = machine(c, seed, 0x2fd758, 5);
                c.bus.write32(OC740_CTRL, OC740_MAGIC);
                c.g[A0][0] = sx(DATA + 0x7000);
                let forced: Vec<(u32, u32)> = match oc740_id(seed) {
                    Some(v) => vec![(0, v)],
                    None => vec![],
                };
                oc740_params(c, &mut r, seed, 0x0044_6190, 5, &forced);
                // The template's target id (word 1): -1 (none) or object 3.
                let tpl = 0x004f_b720;
                c.bus.write32(tpl, u32::MAX);
                c.bus
                    .write32(tpl + 4, if (seed / 4) % 3 == 0 { u32::MAX } else { 3 });
                c.bus.write32(GAME_GP + 0x2518, seed % 2);
                oc740_world(c, &mut r, seed, &[(0x80, 0x84)]);
                // The target object: gp-0x848 -> world; world+0x84 -> table whose
                // entry 3 (+0xc) has an object at +0x28 with a vtable.
                c.bus.write32(GAME_GP - 0x848, DATA + 0xc000);
                c.bus.write32(DATA + 0xc084, DATA + 0xc100);
                c.bus.write32(DATA + 0xc10c, DATA + 0xc200);
                c.bus.write32(DATA + 0xc200 + 12 + 0x28, DATA + 0xc300);
                let vt = DATA + 0xc400;
                c.bus.write32(DATA + 0xc300 + 0x6c0, vt);
                c.bus.write16(vt + 0x28, 4);
                c.bus.write32(vt + 0x2c, fakes::FAKE + 0x20);
                c.bus.write16(vt + 0x30, 8);
                c.bus.write32(vt + 0x34, fakes::FAKE + 0x30);
            },
        );
    }

    /// `WScriptFn_AddParentModifier`: both ids (child, parent), the parent's
    /// active node and its vtable query, the child's node and its query, and
    /// the parent-modifier call.
    #[test]
    fn wscript_fn_add_parent_modifier_matches() {
        fakes::covered(
            ee::s10::f_002fd9f8__generated,
            wscript_fn_add_parent_modifier,
            SEEDS,
            &|c, seed| {
                oc740_hook();
                let mut r = machine(c, seed, 0x2fd9f8, 4);
                c.bus.write32(OC740_CTRL, OC740_MAGIC);
                c.g[A0][0] = sx(DATA + 0x7000);
                // The child (id word 0) is -1 (manager) or 0; the parent (word 1)
                // is -1 (none), 0 or a missing entry.
                let mut forced = vec![];
                if seed % 2 == 0 {
                    forced.push((0, 0));
                }
                if seed % 3 != 0 {
                    forced.push((1, if seed % 7 == 0 { 0x100 } else { 0 }));
                }
                // forced entries keep their position: pad the list so that
                // (1, ..) is entry 1 when (0, ..) is absent.
                if forced.len() == 1 && forced[0].0 == 1 {
                    forced.insert(0, (0, u32::MAX));
                }
                oc740_params(c, &mut r, seed, 0x0044_61a8, 5, &forced);
                let tpl = 0x004f_b738;
                c.bus.write32(tpl, u32::MAX);
                c.bus.write32(tpl + 4, u32::MAX);
                c.bus.write32(GAME_GP + 0x251c, seed % 2);
                oc740_world(c, &mut r, seed, &[(0x50, 0x54), (0x80, 0x84)]);
            },
        );
    }
}
