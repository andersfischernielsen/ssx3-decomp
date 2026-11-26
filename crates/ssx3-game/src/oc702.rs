//! Staging module oc702 (wave wave239): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc702`.

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
        0x002f_b8d0,
        world_obj_factory_counter,
        "WorldObjFactory_Counter",
    ),
    (
        0x002f_bab0,
        world_obj_factory_floating,
        "WorldObjFactory_Floating",
    ),
];

/// GP-relative pointer of the counter's section table (`gp+0x16c8`, the world).
fn counter_gp(c: &Cpu) -> u32 {
    c.g[GP][0] as u32
}

/// `WorldObjFactory_AttachModifier` (0x2fae38), reached by a direct call.
const COUNTER_ATTACH_NODE: Callee = Callee {
    addr: 0x002f_ae38,
    f: ee::s10::WorldObjFactory_AttachModifier,
};

/// Static 12-byte counter template (`D_B8D0`, `0x500000 - 0x4b98`).
const COUNTER_TEMPLATE: u32 = 0x004f_b468;

/// `WorldObjFactory_Counter` (0x2fb8d0), the C++ `MakeCounterFunc`. Builds a
/// counter record in a 12-byte stack buffer (the id word, then the parameter
/// values, stored as floats where the type table says so), finds the target
/// instance by id, and attaches a counter node to it. Returns the `luno` nil
/// value through the hidden pointer in `a0`. The first call copies the
/// static template; the template's own init runs once (the flag at gp+0x24e0).
pub fn world_obj_factory_counter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = Addr(counter_gp(c).wrapping_add(0x24e0)).lw(c);
        frame.save(c, S3, 0x40);
        frame.save(c, S0, 0x70);
        frame.save(c, S2, 0x50);
        frame.save(c, S1, 0x60);
        frame.save(c, S4, 0x30);
        c.g[S3][0] = c.g[A2][0];
        c.g[S0][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        c.g[A2][0] = 0x0050_0000;
        let flag_clear = c.g[V0][0] == 0;
        frame.save_ra(c, 0x20);
        if flag_clear {
            // One-time init of the static template and its flag.
            c.cycles += 11;
            let tpl = COUNTER_TEMPLATE;
            c.g[V1][0] = sx(tpl);
            c.g[V0][0] = u64::MAX;
            c.g[A1][0] = Addr(tpl + 8).lbu(c);
            Addr(tpl).sw(c, u64::MAX);
            c.g[A0][0] = 3;
            let a1 = c.g[A1][0];
            Addr(tpl + 8).sb(c, a1);
            c.g[V0][0] = 1;
            let a0 = c.g[A0][0];
            Addr(tpl + 4).sw(c, a0);
            let v0 = c.g[V0][0];
            let flag = counter_gp(c).wrapping_add(0x24e0);
            Addr(flag).sw(c, v0);
            let a1 = c.g[A1][0];
            Addr(sp(c)).sb(c, a1);
        }

        // Block 2: the buffer's id word, then the template copy.
        c.cycles += 11;
        c.g[V0][0] = u64::MAX;
        c.g[S1][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A1][0] = sx(COUNTER_TEMPLATE);
        c.g[A2][0] = 0xc;
        let v0 = c.g[V0][0];
        Addr(sp(c).wrapping_add(0x10)).sw(c, v0);
        c.g[A0][0] = c.g[S1][0];
        c.g[S4][0] = 0x004d_0000;
        // MEM_copy_B8D0(buf, template, 12).
        call_ptr(c, 0x003e_6574, 0x002f_b954)?;

        c.g[A2][0] = c.g[S3][0];
        if (c.g[S0][0] as i64) > 0 {
            c.cycles += 3;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx(counter_gp(c).wrapping_add(0xaf0));
            c.g[A3][0] = 2;
            loop {
                // One parameter: store it into buf[idx], as float if the
                // table asks for float and the type does not match.
                c.cycles += 8;
                let ent = c.g[A2][0] as u32;
                c.g[V0][0] = Addr(ent).lw(c);
                c.g[V1][0] = Addr(ent.wrapping_add(0xc)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
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
                    deliver_event_here(c, 0x002f_b968);
                }
            }
        }

        // Block 3: resolve the instance from the id in buf[0].
        c.cycles += 4;
        c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V0][0] = !c.g[A2][0];
        c.g[A1][0] = Addr(counter_gp(c).wrapping_add(0x16c8)).lw(c);
        if c.g[V0][0] == 0 {
            // Id is -1: the instance is the script manager's.
            c.cycles += 3;
            c.g[V0][0] = Addr(counter_gp(c).wrapping_add(0xce8)).lw(c);
            c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            if c.g[V1][0] != 0 {
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
                // Section entry: the instance's record from its id.
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
                c.g[A2][0] = c.g[V0][0];
            } else {
                c.cycles += 2;
                c.g[A2][0] = 0;
            }
            c.cycles += 1;
            c.g[S0][0] = c.g[A2][0];
        }

        // Block 4: attach a counter node unless the instance is missing or
        // already has an active node.
        c.cycles += 2;
        if c.g[S0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
            c.g[A1][0] = 7;
            let mut done = false;
            if c.g[A0][0] != 0 {
                c.cycles += 4;
                c.g[A2][0] = c.g[S1][0];
                COUNTER_ATTACH_NODE.call(c, 0x002f_ba38)?;
                done = c.g[V0][0] != 0;
            }
            if !done {
                c.cycles += 11;
                c.g[A1][0] = 0x004a_0000;
                c.g[A0][0] = 0x24;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3be8));
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                // EA_MemAllocNamed(36, D_004A3BE8, 0x20000000, 0).
                call_ptr(c, 0x0031_7d70, 0x002f_ba58)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 1;
                c.g[A2][0] = c.g[S0][0];
                c.g[A3][0] = c.g[S1][0];
                // Counter node constructor.
                call_ptr(c, 0x0034_3768, 0x002f_ba6c)?;
            }
        }

        // Epilogue: copy the nil value (12 bytes) to the return slot.
        c.cycles += 16;
        c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x6f68));
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

/// Call-site constants of a factory's node allocation: the check flag for
/// `checkActiveNode`, the attach return, and the `cMemMan_alloc` size,
/// vtable, return, node constructor and its return.
struct NodeCall {
    flag: u64,
    attach_ret: u32,
    size: u64,
    vtable: u32,
    alloc_ret: u32,
    ctor: u32,
    ctor_ret: u32,
}

/// The parameter loop of the counter-style factories (`MakeCounterFunc`,
/// `MakeFloatingFunc`): each parameter's value goes into `buf[idx]`, stored as
/// a float when the type table asks for one and the type does not match.
/// `pre` is the cycle charge before the loop, `table` the type table's
/// address, `head` the loop's event point. Runs only for a positive count.
fn wof_fill_params(c: &mut Cpu, pre: u64, table: u32, head: u32) {
    c.g[A2][0] = c.g[S3][0];
    if (c.g[S0][0] as i64) <= 0 {
        return;
    }
    c.cycles += pre;
    c.g[A1][0] = c.g[S0][0];
    c.g[T0][0] = sx(table);
    c.g[A3][0] = 2;
    loop {
        c.cycles += 8;
        let ent = c.g[A2][0] as u32;
        c.g[V0][0] = Addr(ent).lw(c);
        c.g[V1][0] = Addr(ent.wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
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
            deliver_event_here(c, head);
        }
    }
}

/// Block 3 of the counter-style factories: the instance for the id in
/// `buf[0]` (`S1`): the script manager for -1, else the section entry's
/// record; sets `S0` to it (0 when there is none).
fn wof_resolve_instance(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
    c.g[V0][0] = !c.g[A2][0];
    c.g[A1][0] = Addr(counter_gp(c).wrapping_add(0x16c8)).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 3;
        c.g[V0][0] = Addr(counter_gp(c).wrapping_add(0xce8)).lw(c);
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
        c.g[A2][0] = c.g[V0][0];
    } else {
        c.cycles += 2;
        c.g[A2][0] = 0;
    }
    c.cycles += 1;
    c.g[S0][0] = c.g[A2][0];
}

/// Block 4 of the counter-style factories: when there is an instance without
/// an active node, `checkActiveNode` (the attach) and, if that finds nothing,
/// a new node from `cMemMan_alloc` built by its constructor.
fn wof_attach_or_alloc(c: &mut Cpu, node: &NodeCall) -> Flow {
    c.cycles += 2;
    if c.g[S0][0] == 0 {
        return Ok(());
    }
    c.cycles += 3;
    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
    c.g[A1][0] = node.flag;
    if c.g[A0][0] != 0 {
        c.cycles += 4;
        c.g[A2][0] = c.g[S1][0];
        COUNTER_ATTACH_NODE.call(c, node.attach_ret)?;
        if c.g[V0][0] != 0 {
            return Ok(());
        }
    }
    c.cycles += 11;
    c.g[A0][0] = node.size;
    c.g[A1][0] = sx(node.vtable);
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7d70, node.alloc_ret)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 1;
    c.g[A2][0] = c.g[S0][0];
    c.g[A3][0] = c.g[S1][0];
    call_ptr(c, node.ctor, node.ctor_ret)?;
    Ok(())
}

/// Epilogue of the counter-style factories: copies the 12-byte `luno` nil
/// value (at 0x4c9098) to the hidden return slot `S2`, and leaves it in `v0`.
fn wof_copy_nil(c: &mut Cpu) {
    c.cycles += 16;
    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x6f68));
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
}

/// `WorldObjFactory_Floating` (0x2fbab0), the C++ `MakeFloatingFunc`: the
/// counter's shape with a 32-byte buffer (the float template for speed,
/// scale, ...) and a 0x7c-byte node. Same protocol as
/// [`world_obj_factory_counter`].
pub fn world_obj_factory_floating(c: &mut Cpu) {
    guest(c, |c| {
        const TEMPLATE: u32 = 0x004f_b478;
        c.cycles += 13;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = Addr(counter_gp(c).wrapping_add(0x24e4)).lw(c);
        frame.save(c, S3, 0x50);
        frame.save(c, S0, 0x80);
        frame.save(c, S2, 0x60);
        frame.save(c, S1, 0x70);
        frame.save(c, S4, 0x40);
        c.g[S3][0] = c.g[A2][0];
        c.g[S0][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        c.g[A2][0] = 0x0050_0000;
        let flag_clear = c.g[V0][0] == 0;
        frame.save_ra(c, 0x30);
        if flag_clear {
            // One-time init of the float template and its flag.
            c.cycles += 20;
            c.g[V0][0] = sx(TEMPLATE);
            c.g[V1][0] = u64::MAX;
            c.g[A1][0] = Addr(TEMPLATE + 0x1c).lbu(c);
            c.g[AT][0] = 0x4120_0000;
            c.mtc1(1, 0);
            c.g[V1][0] |= 0xffff;
            c.g[A0][0] = 1;
            c.g[AT][0] = 0x4220_0000;
            c.mtc1(1, 1);
            c.lwc1(2, counter_gp(c).wrapping_sub(0x3394));
            let v1 = c.g[V1][0];
            Addr(TEMPLATE).sw(c, v1);
            let a1 = c.g[A1][0];
            Addr(TEMPLATE + 0x1c).sb(c, a1);
            c.swc1(1, TEMPLATE + 0xc);
            c.swc1(2, TEMPLATE + 0x14);
            c.swc1(0, TEMPLATE + 0x18);
            let a0 = c.g[A0][0];
            Addr(counter_gp(c).wrapping_add(0x24e4)).sw(c, a0);
            c.swc1(0, TEMPLATE + 4);
            c.swc1(0, TEMPLATE + 8);
            c.swc1(0, TEMPLATE + 0x10);
            let a1 = c.g[A1][0];
            Addr(sp(c)).sb(c, a1);
        }

        // Block 2: the id word, then the 32-byte template copy.
        c.cycles += 11;
        c.g[V0][0] = u64::MAX;
        c.g[S1][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A1][0] = sx(TEMPLATE);
        c.g[A2][0] = 0x20;
        let v0 = c.g[V0][0];
        Addr(sp(c).wrapping_add(0x10)).sw(c, v0);
        c.g[A0][0] = c.g[S1][0];
        c.g[S4][0] = 0x004d_0000;
        // MEM_copy_BAB0(buf, template, 32).
        call_ptr(c, 0x003e_6574, 0x002f_bb58)?;

        wof_fill_params(c, 4, 0x0044_5f28, 0x002f_bb70);
        wof_resolve_instance(c);
        wof_attach_or_alloc(
            c,
            &NodeCall {
                flag: 0xe,
                attach_ret: 0x002f_bc40,
                size: 0x7c,
                vtable: 0x0048_9698,
                alloc_ret: 0x002f_bc60,
                ctor: 0x0034_ec58,
                ctor_ret: 0x002f_bc74,
            },
        )?;
        wof_copy_nil(c);
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    });
}

// @@FUNCS@@

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    /// The machine state the counter-style factories read: parameters at
    /// `DATA + 0x1000`, a type table at `table`, the flag at `flag`, the
    /// section table that resolves ids to no instance.
    fn counter_fixture(c: &mut Cpu, r: &mut Rng, seed: u32, table: u32, flag: u32) {
        c.g[GP][0] = sx(GAME_GP);
        c.g[A0][0] = sx(DATA + 0x7000);
        let n = seed % 4;
        c.g[A1][0] = n as u64;
        c.g[A2][0] = sx(DATA + 0x1000);
        for i in 0..n {
            let e = DATA + 0x1000 + 16 * i;
            c.bus.write32(e, r.next_u32() % 3);
            c.bus.write32(e + 4, r.next_u32());
            c.bus.write32(e + 0xc, r.next_u32() % 3);
        }
        for i in 0..3 {
            c.bus.write32(table + 4 * i, r.next_u32() % 3);
        }
        c.bus.write32(flag, seed % 2);
        c.bus.write32(GAME_GP + 0x16c8, DATA + 0x3000);
        c.bus.write32(DATA + 0x3000, DATA + 0x4000);
        c.bus.write32(DATA + 0x4008, DATA + 0x5000);
        for i in 0..3 {
            c.bus.write32(DATA + 0x5000 + 4 * i, 0);
        }
        c.bus.write32(GAME_GP + 0xce8, DATA + 0x6000);
        c.bus.write32(DATA + 0x6290, 0);
    }

    #[test]
    fn world_obj_factory_floating_matches() {
        check(
            ee::s10::f_002fbab0__generated,
            world_obj_factory_floating,
            300,
            &|c, seed| {
                let mut r = Rng(0x2fba_b0 ^ seed as u64);
                scramble(c, &mut r);
                counter_fixture(c, &mut r, seed, 0x0044_5f28, GAME_GP + 0x24e4);
            },
        );
    }

    /// Covers the one-time init (flag clear or set), the parameter loop
    /// (int and float stores), the no-instance exits (id -1 and a missing
    /// section entry). The attach and allocation tails need fakes and are
    /// not reached here.
    #[test]
    fn world_obj_factory_counter_matches() {
        check(
            ee::s10::f_002fb8d0__generated,
            world_obj_factory_counter,
            300,
            &|c, seed| {
                let mut r = Rng(0x2fb8_d0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.g[A0][0] = sx(DATA + 0x7000);
                let n = seed % 4;
                c.g[A1][0] = n as u64;
                c.g[A2][0] = sx(DATA + 0x1000);
                for i in 0..n {
                    let e = DATA + 0x1000 + 16 * i;
                    c.bus.write32(e, r.next_u32() % 3);
                    c.bus.write32(e + 4, r.next_u32());
                    c.bus.write32(e + 0xc, r.next_u32() % 3);
                }
                for i in 0..3 {
                    c.bus.write32(GAME_GP + 0xaf0 + 4 * i, r.next_u32() % 3);
                }
                c.bus.write32(GAME_GP + 0x24e0, seed % 2);
                c.bus.write32(GAME_GP + 0x16c8, DATA + 0x3000);
                c.bus.write32(DATA + 0x3000, DATA + 0x4000);
                c.bus.write32(DATA + 0x4008, DATA + 0x5000);
                for i in 0..3 {
                    c.bus.write32(DATA + 0x5000 + 4 * i, 0);
                }
                c.bus.write32(GAME_GP + 0xce8, DATA + 0x6000);
                c.bus.write32(DATA + 0x6290, 0);
            },
        );
    }
}
