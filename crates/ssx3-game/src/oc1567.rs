//! Staging module oc1567 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1567`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::game::seg::seg_1fbe38::{wof_copy_nil, wof_fill_params};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002f_c5d8,
        world_obj_factory_dead_fade,
        "WorldObjFactory_DeadFade",
    ),
    (
        0x002f_cdc8,
        world_obj_factory_flex_bridge,
        "WorldObjFactory_FlexBridge",
    ),
    (
        0x0032_0550,
        eacore_static_init_destroy,
        "EACore_StaticInitDestroy",
    ),
    (
        0x002d_0908,
        menu_sys_point2d_item_handle_input,
        "MenuSysPoint2DItem_HandleInput",
    ),
    (0x002e_c060, ccircle_tobj_draw, "cCircle_TObj_Draw"),
];

// ---------------------------------------------------------------------------
// Shared tail of the two factories below (same shape as the counter-style
// factories, but each has its own constants and constructor call).

/// `MEM_copy` (0x3e6574; `call_indirect`).
const OC1567_MEM_COPY: u32 = 0x003e_6574;

/// `WorldObjFactory_AttachModifier` (0x2fae38, `checkActiveNode`).
const OC1567_CHECK_ACTIVE_NODE: Callee = Callee {
    addr: 0x002f_ae38,
    f: ee::s10::WorldObjFactory_AttachModifier,
};

/// `EA_MemAllocNamed` (0x317d70).
const OC1567_MEM_ALLOC: Callee = Callee {
    addr: 0x0031_7d70,
    f: ee::s10::EA_MemAllocNamed,
};

/// Call-site constants of one factory's node build.
struct Oc1567Node {
    /// Modifier type given to `checkActiveNode`.
    flag: u64,
    attach_ret: u32,
    /// Cycles charged before the allocation call.
    pre_alloc: u64,
    /// `cMemMan_alloc` size and name (`0x490000 - name_sub`).
    size: u64,
    name_sub: u32,
    alloc_ret: u32,
    ctor: u32,
    ctor_ret: u32,
    /// The constructor takes a fifth argument (`t0 = 0`).
    ctor_t0: bool,
}

/// Resolve the object the id at `buf[id_off]` (`S1 + id_off`) names into `S0`:
/// the script manager's object for -1, else the section table entry's record
/// (0 when there is none).
fn oc1567_resolve_object(c: &mut Cpu, id_off: u32) {
    c.cycles += 4;
    c.g[A2][0] = Addr((c.g[S1][0] as u32).wrapping_add(id_off)).lw(c);
    c.g[V0][0] = !(0u64 | c.g[A2][0]);
    let named = c.g[V0][0] != 0;
    let gp = r32(c, GP);
    c.g[A1][0] = Addr(gp.wrapping_add(0x16c8)).lw(c);
    if !named {
        c.cycles += 3;
        c.g[V0][0] = Addr(gp.wrapping_add(0xce8)).lw(c);
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

/// With an object in `S0`: `checkActiveNode` on its active node (when it has
/// one) and, if that finds nothing, a new node from `cMemMan_alloc` built by
/// the factory's constructor.
fn oc1567_attach_or_build(c: &mut Cpu, node: &Oc1567Node) -> Flow {
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
        OC1567_CHECK_ACTIVE_NODE.call(c, node.attach_ret)?;
        if c.g[V0][0] != 0 {
            return Ok(());
        }
    }
    c.cycles += node.pre_alloc;
    c.g[A1][0] = sx(0x0049_0000u32.wrapping_sub(node.name_sub));
    c.g[A0][0] = node.size;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    OC1567_MEM_ALLOC.call(c, node.alloc_ret)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 1;
    c.g[A2][0] = c.g[S0][0];
    c.g[A3][0] = c.g[S1][0];
    if node.ctor_t0 {
        c.g[T0][0] = 0;
    }
    call_ptr(c, node.ctor, node.ctor_ret)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// 0x002fc5d8 WorldObjFactory_DeadFade

/// The dead-fade factory's init flag (`D_004A55E8`, at `gp + 0x24f8`).
pub(crate) const OC1567_DEAD_FADE_INIT: u32 = 0x24f8;

/// The dead-fade static description (`D_004FB510`, 20 bytes: id, 0, 2.0f,
/// 25.0f, a trailing byte).
#[cfg(test)]
pub(crate) const OC1567_DEAD_FADE_DESC: u32 = 0x004f_b510;

/// The per-slot type table of the dead-fade arguments (5 words).
#[cfg(test)]
pub(crate) const OC1567_DEAD_FADE_TYPES: u32 = 0x0044_5fa0;

/// `cLunoVar WorldObjFactory_DeadFade(cLunoVar* ret, int n, Ent* params)`
/// (0x2fc5d8, the C++ `MakeDeadFadeFunc`): the Luno call that gives an object
/// a dead-fade node. Expands the argument block over the static description
/// (initialised on the first call), resolves the object it names (id -1: the
/// script manager's current object), and, unless its active node already
/// is a fade (`checkActiveNode` type 0x16), allocates the 0x3c-byte node and
/// builds it. The nil Luno value is copied into `ret`.
pub fn world_obj_factory_dead_fade(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x90);
        let gp = r32(c, GP);
        c.g[V0][0] = Addr(gp.wrapping_add(OC1567_DEAD_FADE_INIT)).lw(c);
        frame.save(c, S3, 0x50);
        frame.save(c, S0, 0x80);
        c.g[S3][0] = c.g[A2][0]; // params
        frame.save(c, S2, 0x60);
        c.g[S0][0] = c.g[A1][0]; // n
        frame.save(c, S1, 0x70);
        c.g[S2][0] = c.g[A0][0]; // ret
        frame.save(c, S4, 0x40);
        c.g[A2][0] = 0x0050_0000;
        let initialised = c.g[V0][0] != 0;
        frame.save_ra(c, 0x30);
        if !initialised {
            // Guarded static-local init of the description.
            c.cycles += 16;
            c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4af0));
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x10)).lbu(c);
            c.g[V0][0] |= 0xffff;
            c.g[AT][0] = 0x4000_0000;
            c.mtc1(AT, 1);
            c.g[A0][0] = 1;
            c.g[AT][0] = 0x41c8_0000;
            c.mtc1(AT, 0);
            c.sw((c.g[A2][0] as u32).wrapping_sub(0x4af0), c.g[V0][0]);
            c.sb((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A1][0]);
            c.swc1(1, (c.g[V1][0] as u32).wrapping_add(8));
            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0xc));
            c.sw(gp.wrapping_add(OC1567_DEAD_FADE_INIT), c.g[A0][0]);
            c.sw((c.g[V1][0] as u32).wrapping_add(4), 0);
            c.sb(frame.at(c, 0), c.g[A1][0]);
        }
        // Expand the argument block over a copy of the description.
        c.cycles += 11;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S1][0] = sx(frame.at(c, 0x10));
        c.g[V0][0] |= 0xffff;
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4af0));
        c.g[A2][0] = 0x14;
        c.sw(frame.at(c, 0x10), c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.g[S4][0] = 0x004d_0000;
        call_ptr(c, OC1567_MEM_COPY, 0x002f_c670)?;
        wof_fill_params(c, 4, 0x0044_5fa0, 0x002f_c688);
        oc1567_resolve_object(c, 0);
        oc1567_attach_or_build(
            c,
            &Oc1567Node {
                flag: 0x16,
                attach_ret: 0x002f_c758,
                pre_alloc: 12,
                size: 0x3c,
                name_sub: 0x6928,
                alloc_ret: 0x002f_c778,
                ctor: 0x0035_0750,
                ctor_ret: 0x002f_c790,
                ctor_t0: true,
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
    })
}

// ---------------------------------------------------------------------------
// 0x002fcdc8 WorldObjFactory_FlexBridge

/// The flex-bridge factory's init flag (`D_004A55F8`, at `gp + 0x2508`).
pub(crate) const OC1567_FLEX_BRIDGE_INIT: u32 = 0x2508;

/// The flex-bridge static description (`D_004FB5C0`, 40 bytes: nine words
/// and a trailing byte at +0x24).
#[cfg(test)]
pub(crate) const OC1567_FLEX_BRIDGE_DESC: u32 = 0x004f_b5c0;

/// The per-slot type table of the flex-bridge arguments (10 words).
#[cfg(test)]
pub(crate) const OC1567_FLEX_BRIDGE_TYPES: u32 = 0x0044_6038;

/// `cLunoVar WorldObjFactory_FlexBridge(cLunoVar* ret, int n, Ent* params)`
/// (0x2fcdc8, the C++ `MakeFlexBridgeFunc`): the dead-fade factory's shape
/// for a flex-bridge node (type 5, 0xb0 bytes, three-argument constructor at
/// 0x346350). The object id is the description's word at +0x20.
pub fn world_obj_factory_flex_bridge(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0xa0);
        let gp = r32(c, GP);
        c.g[V0][0] = Addr(gp.wrapping_add(OC1567_FLEX_BRIDGE_INIT)).lw(c);
        frame.save(c, S0, 0x90);
        c.g[A3][0] = 0x0050_0000;
        frame.save(c, S2, 0x70);
        c.g[S0][0] = c.g[A1][0]; // n
        frame.save(c, S3, 0x60);
        c.g[S2][0] = c.g[A0][0]; // ret
        frame.save(c, S1, 0x80);
        c.g[S3][0] = c.g[A2][0]; // params
        frame.save(c, S4, 0x50);
        let initialised = c.g[V0][0] != 0;
        frame.save_ra(c, 0x40);
        if !initialised {
            // Guarded static-local init of the description.
            c.cycles += 27;
            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x4a40));
            c.g[AT][0] = 0x43fa_0000;
            c.mtc1(AT, 3);
            c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x24)).lbu(c);
            c.g[V1][0] = 0xffff_ffff_ffff_0000;
            c.lwc1(4, gp.wrapping_sub(0x3390));
            c.g[A2][0] = 0x10;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 2);
            c.g[V1][0] |= 0xffff;
            c.g[AT][0] = 0x4516_0000;
            c.mtc1(AT, 0);
            c.g[A0][0] = 1;
            c.g[AT][0] = 0x4120_0000;
            c.mtc1(AT, 1);
            c.swc1(3, (c.g[A3][0] as u32).wrapping_sub(0x4a40));
            c.sb((c.g[V0][0] as u32).wrapping_add(0x24), c.g[A1][0]);
            c.swc1(2, (c.g[V0][0] as u32).wrapping_add(4));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(8));
            c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x10));
            c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A2][0]);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x20), c.g[V1][0]);
            c.sw(gp.wrapping_add(OC1567_FLEX_BRIDGE_INIT), c.g[A0][0]);
            c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0xc));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x18), 0);
            c.sb(frame.at(c, 0), c.g[A1][0]);
        }
        // Expand the argument block over a copy of the description.
        c.cycles += 11;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S1][0] = sx(frame.at(c, 0x10));
        c.g[V0][0] |= 0xffff;
        c.g[A2][0] = 0x28;
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x4a40));
        c.sw(frame.at(c, 0x30), c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.g[S4][0] = 0x004d_0000;
        call_ptr(c, OC1567_MEM_COPY, 0x002f_ce8c)?;
        wof_fill_params(c, 5, 0x0044_6038, 0x002f_cea8);
        oc1567_resolve_object(c, 0x20);
        oc1567_attach_or_build(
            c,
            &Oc1567Node {
                flag: 5,
                attach_ret: 0x002f_cf78,
                pre_alloc: 11,
                size: 0xb0,
                name_sub: 0x6908,
                alloc_ret: 0x002f_cf98,
                ctor: 0x0034_6350,
                ctor_ret: 0x002f_cfac,
                ctor_t0: false,
            },
        )?;
        wof_copy_nil(c);
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x00320550 EACore_StaticInitDestroy

/// `void EACore_StaticInitDestroy(int init, int priority)` (0x320550, the
/// compiler's `__sti__all_in_one_bx_cpp`): the translation unit's static
/// initialisation. Only for priority 0xffff: with `init` set, construct the
/// block manager (`BlockMan_ctor` at 0x4ff000), the two random generators
/// (`Rand_GetDefaultState`), the constant colour, 2D/3D/4D vector, matrix,
/// quaternion and pose globals (the flat float stores below, in the
/// generated order with every scratch register it leaves) and the dynamic
/// memory manager (`EACoreGlobal_ctor`); with `init` clear, destroy it
/// (`EACoreGlobal_dtor(&the manager, 2)`). Any other priority does nothing.
pub fn eacore_static_init_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = 0xffff;
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        let init_priority = c.g[A1][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        if init_priority {
            c.cycles += 2;
            if c.g[A0][0] != 0 {
                // Construct: the block manager and the two random states.
                c.cycles += 214;
                c.g[A0][0] = 0x0050_0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1000));
                OC1567_BLOCK_MAN_CTOR.call(c, 0x0032_0598)?;
                c.g[A0][0] = 0x0050_0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xfe8));
                OC1567_RAND_DEFAULT_STATE.call(c, 0x0032_05a4)?;
                c.g[A0][0] = 0x0050_0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xfd0));
                OC1567_RAND_DEFAULT_STATE.call(c, 0x0032_05b0)?;
                // The constant globals (flat stores of 0, 0.25, 0.5, 1).
                c.g[T5][0] = 0x500000;
                c.g[AT][0] = 0x3e800000;
                c.mtc1(1, 2);
                c.g[T1][0] = c.g[T5][0];
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 1);
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0xf38));
                c.mtc1(0, 0);
                c.swc1(2, (c.g[T1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = 0x500000;
                c.swc1(1, (c.g[V0][0] as u32).wrapping_sub(0xfb8));
                c.g[S6][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xfb8));
                c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = 0x500000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0xfa8));
                c.g[A0][0] = 0x500000;
                c.g[FP][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xfa8));
                c.g[V0][0] = c.g[A0][0];
                c.swc1(1, (c.g[FP][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = 0x500000;
                c.swc1(1, (c.g[A0][0] as u32).wrapping_sub(0xf98));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xf98));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.g[A2][0];
                c.swc1(1, (c.g[A2][0] as u32).wrapping_sub(0xf88));
                c.g[A3][0] = 0x500000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xf88));
                c.g[A2][0] = c.g[A3][0];
                c.swc1(1, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0xf78));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0xf78));
                c.g[T0][0] = 0x500000;
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_sub(0xf68));
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 3);
                c.g[T2][0] = 0x500000;
                c.swc1(1, (c.g[T0][0] as u32).wrapping_sub(0xf68));
                c.g[V0][0] = c.g[T2][0];
                c.swc1(3, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[T3][0] = 0x500000;
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xf58));
                c.swc1(0, (c.g[T2][0] as u32).wrapping_sub(0xf58));
                c.g[A0][0] = c.g[T3][0];
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xf48));
                c.swc1(1, (c.g[T3][0] as u32).wrapping_sub(0xf48));
                c.g[V0][0] = 0x500000;
                c.swc1(1, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[S3][0] = 0x4a0000;
                c.g[T7][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xf08));
                c.g[S4][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2868));
                c.g[A0][0] = 0x500000;
                c.g[V0][0] = c.g[S3][0];
                c.swc1(2, (c.g[T5][0] as u32).wrapping_sub(0xf38));
                c.g[S5][0] = 0x4a0000;
                c.sw((c.g[GP][0] as u32).wrapping_add(0xda4), c.g[S4][0]);
                c.g[T6][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xef8));
                c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x5960));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5960));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.g[S5][0];
                c.swc1(1, (c.g[S5][0] as u32).wrapping_add(0x5968));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5968));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[S1][0] = 0x500000;
                c.g[A0][0] = 0x4a0000;
                c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_sub(0xf28));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x5970));
                c.g[T4][0] = 0x500000;
                c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x500000;
                c.g[S6][0] = 0x500000;
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_sub(0xf98));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xf88));
                c.swc1(1, (c.g[S6][0] as u32).wrapping_add(0x4));
                c.g[S0][0] = sx((c.g[T4][0] as u32).wrapping_sub(0xf18));
                c.g[S6][0] = 0x500000;
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_sub(0xf78));
                c.g[T1][0] = 0x500000;
                c.swc1(1, (c.g[S6][0] as u32).wrapping_add(0x4));
                c.g[A2][0] = sx((c.g[T1][0] as u32).wrapping_sub(0xee0));
                c.g[S6][0] = 0x500000;
                c.g[S7][0] = 0x500000;
                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_sub(0xf58));
                c.g[A3][0] = sx((c.g[S7][0] as u32).wrapping_sub(0xec0));
                c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x4));
                c.g[T8][0] = 0x500000;
                c.g[S6][0] = 0x500000;
                c.g[T0][0] = sx((c.g[T8][0] as u32).wrapping_sub(0xeb0));
                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_sub(0xf48));
                c.g[T9][0] = 0x500000;
                c.swc1(1, (c.g[S6][0] as u32).wrapping_add(0x4));
                c.g[T1][0] = sx((c.g[T9][0] as u32).wrapping_sub(0xea0));
                c.g[S6][0] = 0x500000;
                c.g[T5][0] = 0x500000;
                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_sub(0xf38));
                c.g[V1][0] = sx((c.g[T5][0] as u32).wrapping_sub(0xe90));
                c.swc1(2, (c.g[S6][0] as u32).wrapping_add(0x4));
                c.g[S3][0] = 0x500000;
                c.g[S6][0] = 0x4a0000;
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0xe60));
                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x5970));
                c.g[S4][0] = 0x500000;
                c.swc1(1, (c.g[S6][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0xe20));
                c.swc1(0, (c.g[S1][0] as u32).wrapping_sub(0xf28));
                c.g[S6][0] = 0x500000;
                c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x8));
                c.g[S5][0] = 0x500000;
                c.swc1(1, (c.g[T4][0] as u32).wrapping_sub(0xf18));
                c.g[T2][0] = sx((c.g[S5][0] as u32).wrapping_sub(0xde0));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[T4][0] = 0x500000;
                c.swc1(0, (c.g[T4][0] as u32).wrapping_sub(0xf08));
                c.g[T3][0] = 0x500000;
                c.swc1(0, (c.g[S6][0] as u32).wrapping_sub(0xef8));
                c.g[T4][0] = 0x500000;
                c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x8));
                c.g[S6][0] = 0x500000;
                c.swc1(1, (c.g[T6][0] as u32).wrapping_add(0x8));
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0xdd0));
                c.swc1(0, (c.g[T4][0] as u32).wrapping_sub(0xee0));
                c.g[A0][0] = 0x500000;
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xdb0));
                c.swc1(0, (c.g[S6][0] as u32).wrapping_sub(0xed0));
                c.swc1(1, (c.g[S7][0] as u32).wrapping_sub(0xec0));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0xc));
                c.swc1(0, (c.g[T8][0] as u32).wrapping_sub(0xeb0));
                c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0xc));
                c.swc1(0, (c.g[T9][0] as u32).wrapping_sub(0xea0));
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0xc));
                c.swc1(1, (c.g[T5][0] as u32).wrapping_sub(0xe90));
                c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                c.swc1(1, (c.g[T7][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[T6][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = c.g[S6][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0xed0));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.swc1(1, (c.g[A2][0] as u32).wrapping_add(0xc));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.swc1(1, (c.g[T0][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x4));
                c.swc1(1, (c.g[T1][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0xc));
                c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x20));
                c.swc1(1, (c.g[S3][0] as u32).wrapping_sub(0xe60));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x3c));
                c.swc1(0, (c.g[S4][0] as u32).wrapping_sub(0xe20));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x18));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x10));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x1c));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x14));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x10));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x20));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x24));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x34));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x18));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x28));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x38));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x1c));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x2c));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x10));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x20));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x30));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x14));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x24));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x34));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x18));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x28));
                c.swc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
                c.swc1(0, (c.g[S5][0] as u32).wrapping_sub(0xde0));
                c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x4));
                c.lq(3, c.g[A2][0] as u32);
                c.lq(2, c.g[T2][0] as u32);
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x3c));
                c.sq(2, (c.g[T3][0] as u32).wrapping_add(0x10));
                c.sq(3, c.g[T3][0] as u32);
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x38));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0xc));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x1c));
                // The jal's delay slot, then `EACoreGlobal_ctor(&manager)`.
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x2c));
                OC1567_CORE_GLOBAL_CTOR.call(c, 0x0032_08dc)?;
                frame.restore(c, S0, 0x90);
                oc1567_static_init_epilogue(c, frame);
                return Ok(());
            }
            c.g[A0][0] = 0x0050_0000;
            // Destroy the dynamic memory manager.
            c.cycles += 3;
            c.g[A1][0] = 2;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xdb0));
            OC1567_CORE_GLOBAL_DTOR.call(c, 0x0032_08f0)?;
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x90);
        oc1567_static_init_epilogue(c, frame);
        Ok(())
    })
}

/// The common exit of [`eacore_static_init_destroy`]: restore s1..s7, fp and
/// ra, release the frame.
fn oc1567_static_init_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 11;
    frame.restore(c, S1, 0x80);
    frame.restore(c, S2, 0x70);
    frame.restore(c, S3, 0x60);
    frame.restore(c, S4, 0x50);
    frame.restore(c, S5, 0x40);
    frame.restore(c, S6, 0x30);
    frame.restore(c, S7, 0x20);
    frame.restore(c, FP, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `BlockMan_ctor` (0x316a00).
const OC1567_BLOCK_MAN_CTOR: Callee = Callee {
    addr: 0x0031_6a00,
    f: ee::s10::BlockMan_ctor,
};

/// `Rand_GetDefaultState` (0x317930).
const OC1567_RAND_DEFAULT_STATE: Callee = Callee {
    addr: 0x0031_7930,
    f: ee::s10::Rand_GetDefaultState,
};

/// `EACoreGlobal_ctor` (0x31ff20).
const OC1567_CORE_GLOBAL_CTOR: Callee = Callee {
    addr: 0x0031_ff20,
    f: ee::s10::EACoreGlobal_ctor,
};

/// `EACoreGlobal_dtor` (0x31ff38).
const OC1567_CORE_GLOBAL_DTOR: Callee = Callee {
    addr: 0x0031_ff38,
    f: ee::s10::EACoreGlobal_dtor,
};

// ---------------------------------------------------------------------------
// 0x002d0908 MenuSysPoint2DItem_HandleInput

/// `MenuSysItem_IsDisabled` (0x2ca3b0).
const OC1567_ITEM_IS_DISABLED: Callee = Callee {
    addr: 0x002c_a3b0,
    f: ee::s09::MenuSysItem_IsDisabled,
};

/// `MenuSysItem_GetInputCode` (0x2ca3e8): the menu's current input code.
const OC1567_ITEM_INPUT_CODE: Callee = Callee {
    addr: 0x002c_a3e8,
    f: ee::s09::MenuSysItem_GetInputCode,
};

/// `MenuSysItem_GetLeftAxis` (0x2ca408).
const OC1567_ITEM_LEFT_AXIS: Callee = Callee {
    addr: 0x002c_a408,
    f: ee::s09::MenuSysItem_GetLeftAxis,
};

/// `MenuSysItem_GetRightAxis` (0x2ca428).
const OC1567_ITEM_RIGHT_AXIS: Callee = Callee {
    addr: 0x002c_a428,
    f: ee::s09::MenuSysItem_GetRightAxis,
};

/// `MenuSysItem_GetUpAxis` (0x2ca448).
const OC1567_ITEM_UP_AXIS: Callee = Callee {
    addr: 0x002c_a448,
    f: ee::s09::MenuSysItem_GetUpAxis,
};

/// `MenuSysItem_GetDownAxis` (0x2ca468).
const OC1567_ITEM_DOWN_AXIS: Callee = Callee {
    addr: 0x002c_a468,
    f: ee::s09::MenuSysItem_GetDownAxis,
};

/// `MenuSysItem_HandleInput` (0x2ca2b0): the base item's handler.
const OC1567_ITEM_HANDLE_INPUT: Callee = Callee {
    addr: 0x002c_a2b0,
    f: ee::s09::MenuSysItem_HandleInput,
};

// Fields of the 2D point item (`cGraphMenuItem`), relative to `self`.
/// `void* menu` (its vtable pointer sits at +0x12c of the menu).
#[cfg(test)]
pub(crate) const OC1567_P2D_MENU: u32 = 0x00;
/// `int count`: the number of points.
#[cfg(test)]
pub(crate) const OC1567_P2D_COUNT: u32 = 0x18;
/// `u32 lockX`, `u32 lockY`: one bit per point.
#[cfg(test)]
pub(crate) const OC1567_P2D_LOCK_X: u32 = 0x1c;
#[cfg(test)]
pub(crate) const OC1567_P2D_LOCK_Y: u32 = 0x20;
/// `int cur`: the selected point.
#[cfg(test)]
pub(crate) const OC1567_P2D_CUR: u32 = 0x24;
/// `Vec2* pts` (8 bytes per point: x, y).
#[cfg(test)]
pub(crate) const OC1567_P2D_PTS: u32 = 0x28;
/// Bounds: `minX, minY, maxX, maxY` floats at +0x2c..+0x38.
#[cfg(test)]
pub(crate) const OC1567_P2D_BOUNDS: u32 = 0x2c;
/// `int order`: >0 keeps y ascending along the points, <0 descending.
#[cfg(test)]
pub(crate) const OC1567_P2D_ORDER: u32 = 0x3c;
/// `int editing`: 1 while a point is being dragged.
#[cfg(test)]
pub(crate) const OC1567_P2D_EDITING: u32 = 0x40;

/// `int MenuSysPoint2DItem_HandleInput(Point2DItem* self)` (0x2d0908, the
/// reference's `cGraphMenuItem::input`): the input handler of the curve
/// editor item. A disabled item returns 0. Input code 6 toggles `editing`;
/// 7 and 8 end it. While editing, left (4) and right (5) move the selection,
/// and the up/down and left/right axes drag the selected point: `y += dy *
/// (maxY - minY) / 120`, `x += dx * (maxX - minX) / 120` (unless that
/// point's bit is set in `lockY` / `lockX`), clamped to the bounds, kept in
/// order against its neighbours (y by `order`, x at least `step` apart,
/// `step = (maxX - minX) / max(2 * count, 50)`). When not editing the base
/// item's handler decides the result. A change notifies the menu through its
/// vtable (`menu->vt[4](menu, self)`).
pub fn menu_sys_point2d_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[S2][0] = 0;
        frame.save_ra(c, 0);
        frame.save_fpr(c, 21, 0x48);
        frame.save_fpr(c, 20, 0x40);
        OC1567_ITEM_IS_DISABLED.call(c, 0x002d_0930)?;
        let disabled = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[S2][0];
        if !disabled {
            // S1: whether a point changed; S2: the result.
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            c.g[S1][0] = 0;
            OC1567_ITEM_INPUT_CODE.call(c, 0x002d_0944)?;
            c.g[V1][0] = 6;
            let toggle = c.g[V0][0] == c.g[V1][0];
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x40)).lw(c);
            if toggle {
                c.cycles += 3;
                c.g[V0][0] ^= 1;
                let editing = c.g[V0][0];
                Addr((c.g[S0][0] as u32).wrapping_add(0x40)).sw(c, editing);
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x40)).lw(c);
            }
            'done: {
                'base: {
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        break 'base;
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    OC1567_ITEM_INPUT_CODE.call(c, 0x002d_096c)?;
                    c.g[V1][0] = 8;
                    if c.g[V0][0] == c.g[V1][0] {
                        // Code 8: stop editing.
                        Addr((c.g[S0][0] as u32).wrapping_add(0x40)).sw(c, 0);
                        break 'done;
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    OC1567_ITEM_INPUT_CODE.call(c, 0x002d_0980)?;
                    c.g[V1][0] = 7;
                    if c.g[V0][0] != c.g[V1][0] {
                        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x40)).lw(c);
                    } else {
                        // Code 7: stop editing.
                        c.cycles += 2;
                        Addr((c.g[S0][0] as u32).wrapping_add(0x40)).sw(c, 0);
                        break 'done;
                    }
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        break 'base;
                    }
                    oc1567_point2d_edit(c)?;
                    break 'done;
                }
                // Not editing: the base item decides.
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                OC1567_ITEM_HANDLE_INPUT.call(c, 0x002d_0d00)?;
                c.g[S2][0] = c.g[V0][0];
            }
            // A changed point tells the menu: `menu->vt[4](menu, self)`.
            c.cycles += 2;
            let changed = c.g[S1][0] != 0;
            c.g[A1][0] = c.g[S0][0];
            if changed {
                c.cycles += 6;
                c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x12c)).lw(c);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x20)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x24)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x002d_0d24)?;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S2][0];
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x48);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// The address of a field of the item (`S0 + off`).
fn oc1567_p2d(c: &Cpu, off: u32) -> u32 {
    (c.g[S0][0] as u32).wrapping_add(off)
}

/// The editing part of [`menu_sys_point2d_item_handle_input`] (`editing` is
/// set and the code is not 7/8): selection moves, then the y drag, then the
/// x drag.
fn oc1567_point2d_edit(c: &mut Cpu) -> Flow {
    'after_move: {
        'moved_next: {
            'store_cur: {
                'not_left: {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    OC1567_ITEM_INPUT_CODE.call(c, 0x002d_09a4)?;
                    c.g[V1][0] = 4;
                    if c.g[V0][0] != c.g[V1][0] {
                        break 'not_left;
                    }
                    // Left: select the previous point.
                    c.cycles += 3;
                    let a = oc1567_p2d(c, 0x24);
                    c.g[V0][0] = Addr(a).lw(c);
                    let at_first = (c.g[V0][0] as i64) <= 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    if at_first {
                        break 'moved_next;
                    }
                    c.cycles += 2;
                    c.g[S1][0] = 1;
                    break 'store_cur;
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                OC1567_ITEM_INPUT_CODE.call(c, 0x002d_09cc)?;
                c.g[V1][0] = 5;
                let not_right = c.g[V0][0] != c.g[V1][0];
                let a = oc1567_p2d(c, 0x18);
                c.g[V0][0] = Addr(a).lw(c);
                if not_right {
                    break 'after_move;
                }
                // Right: select the next point.
                c.cycles += 5;
                let a = oc1567_p2d(c, 0x24);
                c.g[V1][0] = Addr(a).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let at_last = c.g[V0][0] == 0;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                if at_last {
                    break 'moved_next;
                }
                c.cycles += 1;
                c.g[S1][0] = 1;
            }
            c.cycles += 1;
            let new_cur = c.g[V0][0];
            let a = oc1567_p2d(c, 0x24);
            Addr(a).sw(c, new_cur);
        }
        c.cycles += 1;
        let a = oc1567_p2d(c, 0x18);
        c.g[V0][0] = Addr(a).lw(c);
    }

    // The y drag. f21 = step = (maxX - minX) / max(2 * count, 50).
    c.cycles += 22;
    c.g[V1][0] = 0x31;
    c.lwc1(2, oc1567_p2d(c, 0x2c));
    c.g[A1][0] = 0x32;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
    c.lwc1(1, oc1567_p2d(c, 0x34));
    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    if c.g[V1][0] != 0 {
        c.g[A1][0] = c.g[V0][0];
    }
    c.sub_s(1, 1, 2);
    c.mtc1(A1, 0);
    c.cvt_s_w(0, 0);
    c.div_s(21, 1, 0);
    c.g[A0][0] = c.g[S0][0];
    OC1567_ITEM_UP_AXIS.call(c, 0x002d_0a2c)?;
    c.mov_s(20, 0);
    c.g[A0][0] = c.g[S0][0];
    OC1567_ITEM_DOWN_AXIS.call(c, 0x002d_0a38)?;
    c.sub_s(20, 20, 0);
    c.mtc1(0, 0);
    c.c_eq_s(20, 0);
    let dy_zero = c.fcc();
    c.g[V0][0] = 1;
    'x_drag: {
        'y_done: {
            if dy_zero {
                break 'y_done;
            }
            c.cycles += 6;
            let a = oc1567_p2d(c, 0x24);
            c.g[A0][0] = Addr(a).lw(c);
            let a = oc1567_p2d(c, 0x20);
            c.g[V1][0] = Addr(a).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[A0][0] as u32) & 31));
            c.g[V1][0] &= c.g[V0][0];
            let locked = c.g[V1][0] != 0;
            c.g[V0][0] = sx((c.g[A0][0] as u32) << 3);
            if locked {
                break 'y_done;
            }
            // y += dy * (maxY - minY) * (1 / 120), clamped to the bounds.
            c.cycles += 21;
            c.lwc1(1, oc1567_p2d(c, 0x30));
            c.lwc1(0, oc1567_p2d(c, 0x38));
            let a = oc1567_p2d(c, 0x28);
            c.g[V1][0] = Addr(a).lw(c);
            c.sub_s(0, 0, 1);
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x3fa0));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(4));
            c.mul_s(0, 20, 0);
            c.mul_s(0, 0, 2);
            c.add_s(1, 1, 0);
            c.swc1(1, (c.g[V0][0] as u32).wrapping_add(4));
            let a = oc1567_p2d(c, 0x24);
            c.g[V0][0] = Addr(a).lw(c);
            let a = oc1567_p2d(c, 0x28);
            c.g[V1][0] = Addr(a).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
            c.lwc1(1, oc1567_p2d(c, 0x38));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(4));
            c.c_lt_s(1, 0);
            let within_max = !c.fcc();
            c.g[S1][0] = 1;
            if !within_max {
                c.cycles += 1;
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(4));
            }
            c.cycles += 10;
            let a = oc1567_p2d(c, 0x24);
            c.g[V0][0] = Addr(a).lw(c);
            let a = oc1567_p2d(c, 0x28);
            c.g[V1][0] = Addr(a).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
            c.lwc1(1, oc1567_p2d(c, 0x30));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(4));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(4));
            }
            // Keep y in order against the next point (`order`).
            c.cycles += 6;
            let a = oc1567_p2d(c, 0x18);
            c.g[V0][0] = Addr(a).lw(c);
            let a = oc1567_p2d(c, 0x24);
            c.g[V1][0] = Addr(a).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            'order_next_done: {
                if c.g[V0][0] == 0 {
                    let a = oc1567_p2d(c, 0x24);
                    c.g[A1][0] = Addr(a).lw(c);
                    break 'order_next_done;
                }
                'next_fixed: {
                    'not_ascending: {
                        c.cycles += 3;
                        let a = oc1567_p2d(c, 0x3c);
                        c.g[A0][0] = Addr(a).lw(c);
                        let order_le0 = (c.g[A0][0] as i64) <= 0;
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
                        if order_le0 {
                            break 'not_ascending;
                        }
                        c.cycles += 8;
                        let a = oc1567_p2d(c, 0x28);
                        c.g[V0][0] = Addr(a).lw(c);
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0xc));
                        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(4));
                        c.c_lt_s(1, 0);
                        if c.fcc() {
                            c.swc1(1, (c.g[V0][0] as u32).wrapping_add(4));
                            break 'next_fixed;
                        }
                    }
                    c.cycles += 2;
                    if (c.g[A0][0] as i64) >= 0 {
                        let a = oc1567_p2d(c, 0x24);
                        c.g[A1][0] = Addr(a).lw(c);
                        break 'order_next_done;
                    }
                    c.cycles += 10;
                    let a = oc1567_p2d(c, 0x24);
                    c.g[V0][0] = Addr(a).lw(c);
                    let a = oc1567_p2d(c, 0x28);
                    c.g[V1][0] = Addr(a).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0xc));
                    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(4));
                    c.c_lt_s(0, 1);
                    if !c.fcc() {
                        let a = oc1567_p2d(c, 0x24);
                        c.g[A1][0] = Addr(a).lw(c);
                        break 'order_next_done;
                    }
                    c.cycles += 1;
                    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(4));
                }
                c.cycles += 1;
                let a = oc1567_p2d(c, 0x24);
                c.g[A1][0] = Addr(a).lw(c);
            }
            // ... and against the previous point.
            c.cycles += 2;
            if (c.g[A1][0] as i64) <= 0 {
                break 'y_done;
            }
            'not_ascending_prev: {
                c.cycles += 3;
                let a = oc1567_p2d(c, 0x3c);
                c.g[A0][0] = Addr(a).lw(c);
                let order_le0 = (c.g[A0][0] as i64) <= 0;
                c.g[V1][0] = sx((c.g[A1][0] as u32) << 3);
                if order_le0 {
                    break 'not_ascending_prev;
                }
                c.cycles += 8;
                let a = oc1567_p2d(c, 0x28);
                c.g[V0][0] = Addr(a).lw(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.lwc1(1, (c.g[V1][0] as u32).wrapping_sub(4));
                c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(4));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, (c.g[V1][0] as u32).wrapping_add(4));
                    break 'y_done;
                }
            }
            c.cycles += 2;
            let order_ge0 = (c.g[A0][0] as i64) >= 0;
            c.g[V1][0] = sx((c.g[A1][0] as u32) << 3);
            if order_ge0 {
                break 'y_done;
            }
            c.cycles += 8;
            let a = oc1567_p2d(c, 0x28);
            c.g[V0][0] = Addr(a).lw(c);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.lwc1(1, (c.g[V1][0] as u32).wrapping_sub(4));
            c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(4));
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.swc1(1, (c.g[V1][0] as u32).wrapping_add(4));
            }
        }
        // The x drag: x += dx * (maxX - minX) * (1 / 120), clamped, and at
        // least `step` away from its neighbours.
        c.cycles += 11;
        c.g[A0][0] = c.g[S0][0];
        OC1567_ITEM_RIGHT_AXIS.call(c, 0x002d_0bc8)?;
        c.mov_s(20, 0);
        c.g[A0][0] = c.g[S0][0];
        OC1567_ITEM_LEFT_AXIS.call(c, 0x002d_0bd4)?;
        c.sub_s(20, 20, 0);
        c.mtc1(0, 0);
        c.c_eq_s(20, 0);
        let dx_zero = c.fcc();
        c.g[V0][0] = 1;
        if dx_zero {
            break 'x_drag;
        }
        c.cycles += 6;
        let a = oc1567_p2d(c, 0x24);
        c.g[A0][0] = Addr(a).lw(c);
        let a = oc1567_p2d(c, 0x1c);
        c.g[V1][0] = Addr(a).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[A0][0] as u32) & 31));
        c.g[V1][0] &= c.g[V0][0];
        let locked = c.g[V1][0] != 0;
        c.g[V0][0] = sx((c.g[A0][0] as u32) << 3);
        if locked {
            break 'x_drag;
        }
        c.cycles += 21;
        c.lwc1(1, oc1567_p2d(c, 0x2c));
        c.lwc1(0, oc1567_p2d(c, 0x34));
        let a = oc1567_p2d(c, 0x28);
        c.g[V1][0] = Addr(a).lw(c);
        c.sub_s(0, 0, 1);
        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x3f9c));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.lwc1(1, c.g[V0][0] as u32);
        c.mul_s(0, 20, 0);
        c.mul_s(0, 0, 2);
        c.add_s(1, 1, 0);
        c.swc1(1, c.g[V0][0] as u32);
        let a = oc1567_p2d(c, 0x24);
        c.g[V0][0] = Addr(a).lw(c);
        let a = oc1567_p2d(c, 0x28);
        c.g[V1][0] = Addr(a).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
        c.lwc1(1, oc1567_p2d(c, 0x34));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.lwc1(0, c.g[V0][0] as u32);
        c.c_lt_s(1, 0);
        let within_max = !c.fcc();
        c.g[S1][0] = 1;
        if !within_max {
            c.cycles += 1;
            c.swc1(1, c.g[V0][0] as u32);
        }
        c.cycles += 10;
        let a = oc1567_p2d(c, 0x24);
        c.g[V0][0] = Addr(a).lw(c);
        let a = oc1567_p2d(c, 0x28);
        c.g[V1][0] = Addr(a).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
        c.lwc1(1, oc1567_p2d(c, 0x2c));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.lwc1(0, c.g[V0][0] as u32);
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, c.g[V0][0] as u32);
        }
        // Against the next point: x <= next.x - step.
        c.cycles += 6;
        let a = oc1567_p2d(c, 0x18);
        c.g[V0][0] = Addr(a).lw(c);
        let a = oc1567_p2d(c, 0x24);
        c.g[A0][0] = Addr(a).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let at_last = c.g[V0][0] == 0;
        c.g[V0][0] = sx((c.g[A0][0] as u32) << 3);
        if !at_last {
            c.cycles += 9;
            let a = oc1567_p2d(c, 0x28);
            c.g[V1][0] = Addr(a).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(8));
            c.lwc1(1, c.g[V0][0] as u32);
            c.sub_s(0, 0, 21);
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(0, c.g[V0][0] as u32);
            }
        }
        // Against the previous point: x >= prev.x + step.
        c.cycles += 3;
        let a = oc1567_p2d(c, 0x24);
        c.g[V0][0] = Addr(a).lw(c);
        let at_first = (c.g[V0][0] as i64) <= 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
        if at_first {
            break 'x_drag;
        }
        c.cycles += 9;
        let a = oc1567_p2d(c, 0x28);
        c.g[V1][0] = Addr(a).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_sub(8));
        c.lwc1(1, c.g[V0][0] as u32);
        c.add_s(0, 0, 21);
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(0, c.g[V0][0] as u32);
        } else {
            c.cycles += 2;
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// 0x002ec060 cCircle_TObj_Draw

/// `TransitionEffect_DrawRect` (0x2eb938): fills a rectangle of the screen.
const OC1567_DRAW_RECT: Callee = Callee {
    addr: 0x002e_b938,
    f: ee::s09::TransitionEffect_DrawRect,
};

/// The vertex packer the circle wipe calls through the function table
/// (0x2ebf48; `x`, `y` in f12/f13, vertex out in a0, colour source in a1).
const OC1567_PACK_VERTEX: u32 = 0x002e_bf48;

/// `void cCircle_TObj_Draw(CircleTransition* self, float t)` (0x2ec060, the
/// reference's `func_002EC060`; `t` is the wipe's progress in f12): the iris
/// wipe of a screen transition. `r = (1 - t) * 800` is the radius of the
/// open circle around the centre `self[0x14]` (the circle's y is `self[0x18]`).
/// The two side rectangles outside the circle are drawn with
/// `TransitionEffect_DrawRect` (the texture window `{gp+0xa40..}` scaled
/// across `640 - x`), then the arc is built into two 40-entry tables from the
/// sine table at 0x504fb8 (an x offset and a y offset per step), and two
/// triangle strips of 80 vertices (the area above and below the arc) are
/// packed with the vertex packer and submitted through the draw object's
/// vtable (`gp+0x2a90`, entry 71, command 0x50). The y values are clamped to
/// 0..480.
pub fn ccircle_tobj_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 94;
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x2040));
        c.g[AT][0] = 0x44480000;
        c.mtc1(1, 3);
        c.sub_s(0, 0, 12);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x2000));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x1ff0));
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x1f90));
        c.g[A1][0] = c.g[S0][0];
        c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x2038));
        c.g[S7][0] = c.g[A0][0];
        c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x2030));
        c.mul_s(25, 0, 3);
        c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x2028));
        c.g[S1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x4));
        c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x2020));
        c.g[A0][0] = c.g[SP][0];
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x2018));
        c.g[A2][0] = c.g[S1][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x1fe0));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x1fd0));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x1fc0));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x1fb0));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x1fa0));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x1f80));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x1f70), c.g[RA][0]);
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x2010));
        c.lwc1(20, (c.g[S7][0] as u32).wrapping_add(0x14));
        c.lwc1(22, (c.g[GP][0] as u32).wrapping_sub(0x3984));
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_add(0xa48));
        c.sub_s(4, 20, 25);
        c.lwc1(2, (c.g[GP][0] as u32).wrapping_add(0xa40));
        c.add_s(20, 20, 25);
        c.mul_s(1, 1, 22);
        c.sw(c.g[SP][0] as u32, 0u64);
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lwc1(21, c.g[SP][0] as u32);
        c.mul_s(1, 1, 4);
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0xa4c));
        c.lwc1(3, (c.g[GP][0] as u32).wrapping_add(0xa54));
        c.mov_s(12, 21);
        c.g[AT][0] = 0x43f00000;
        c.mtc1(1, 24);
        c.add_s(2, 2, 1);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x1c));
        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x18));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
        c.g[AT][0] = 0x44200000;
        c.mtc1(1, 23);
        c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0xc));
        OC1567_DRAW_RECT.call(c, 0x002ec13c)?;
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_add(0xa48));
        c.g[A1][0] = c.g[S0][0];
        c.sub_s(5, 23, 20);
        c.lwc1(2, (c.g[GP][0] as u32).wrapping_add(0xa40));
        c.mul_s(0, 1, 22);
        c.lwc1(3, (c.g[GP][0] as u32).wrapping_add(0xa4c));
        c.lwc1(4, (c.g[GP][0] as u32).wrapping_add(0xa54));
        c.g[A2][0] = c.g[S1][0];
        c.mul_s(1, 1, 5);
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x14));
        c.mul_s(0, 0, 20);
        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x1c));
        c.g[A0][0] = c.g[SP][0];
        c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.mul_s(1, 1, 22);
        c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0xc));
        c.add_s(0, 2, 0);
        c.swc1(20, c.g[SP][0] as u32);
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.mov_s(12, 21);
        c.add_s(2, 2, 1);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x18));
        OC1567_DRAW_RECT.call(c, 0x002ec19c)?;
        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1060));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x1f60), c.g[V0][0]);
        c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x1f64), c.g[V1][0]);
        c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.g[V0][0] = 0x500000;
        c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x3980));
        c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x397c));
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4fb8));
        c.lwc1(4, (c.g[S7][0] as u32).wrapping_add(0x14));
        c.mov_s(3, 21);
        c.g[A2][0] = c.g[T1][0];
        c.g[A1][0] = c.g[T0][0];
        c.g[A0][0] = 0x27;
        // The arc: 40 steps of an angle (f21) through the sine table.
        loop {
            c.cycles += 19;
            c.mul_s(0, 21, 5);
            c.cvt_w_s(1, 0);
            c.mfc1(2, 1);
            c.g[V0][0] = c.g[V0][0] & 0x1ff;
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x80));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
            c.lwc1(0, c.g[V0][0] as u32);
            c.lwc1(1, c.g[V1][0] as u32);
            c.mul_s(0, 25, 0);
            c.mul_s(1, 25, 1);
            c.swc1(0, c.g[A1][0] as u32);
            c.add_s(0, 4, 1);
            c.c_le_s(3, 0);
            let keep_low = !c.fcc();
            c.mov_s(2, 3);
            if !keep_low {
                c.cycles += 1;
                c.min_s(2, 0, 23);
            }
            c.cycles += 6;
            c.swc1(2, c.g[A2][0] as u32);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(4));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
            let more = (c.g[A0][0] as i64) >= 0;
            c.add_s(21, 21, 6);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002e_c1d8);
            }
        }
        c.cycles += 6;
        c.g[V0][0] = 0x4f;
        c.g[S2][0] = sx((c.g[S7][0] as u32).wrapping_add(4));
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        c.g[V0][0] = 0x4f;
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        // An empty countdown of 80 steps (the compiler's leftover loop).
        loop {
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            let more = c.g[V0][0] != c.g[V1][0];
            c.g[S1][0] = c.g[T0][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002e_c258);
            }
        }
        c.cycles += 11;
        c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x1f64)).lw(c);
        c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x1f60)).lw(c);
        c.g[S0][0] = c.g[T1][0];
        c.mtc1(0, 20);
        c.g[S6][0] = sx((c.g[V0][0] as u32).wrapping_add(0x30));
        c.g[AT][0] = 0x43f0_0000;
        c.mtc1(AT, 21);
        c.g[S5][0] = c.g[V0][0];
        c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
        c.g[S3][0] = c.g[V1][0];
        c.g[FP][0] = 0x27;
        // Per step, four vertices (top strip: 0 and the clamped upper edge;
        // bottom strip: the clamped lower edge and 480).
        loop {
            c.cycles += 12;
            c.lwc1(12, c.g[S0][0] as u32);
            c.mov_s(13, 20);
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S2][0];
            call_ptr(c, OC1567_PACK_VERTEX, 0x002e_c2b4)?;
            c.lwc1(1, (c.g[S7][0] as u32).wrapping_add(0x18));
            c.lwc1(0, c.g[S1][0] as u32);
            c.sub_s(1, 1, 0);
            c.c_le_s(20, 1);
            let keep_low = !c.fcc();
            c.mov_s(13, 20);
            if !keep_low {
                c.cycles += 1;
                c.min_s(13, 1, 21);
            }
            c.cycles += 11;
            c.lwc1(12, c.g[S0][0] as u32);
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S2][0];
            call_ptr(c, OC1567_PACK_VERTEX, 0x002e_c2e4)?;
            c.lwc1(1, (c.g[S7][0] as u32).wrapping_add(0x18));
            c.lwc1(0, c.g[S1][0] as u32);
            c.add_s(1, 1, 0);
            c.c_le_s(20, 1);
            let keep_low = !c.fcc();
            c.mov_s(13, 20);
            if !keep_low {
                c.cycles += 1;
                c.min_s(13, 1, 21);
            }
            c.cycles += 18;
            c.lwc1(12, c.g[S0][0] as u32);
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x60));
            call_ptr(c, OC1567_PACK_VERTEX, 0x002e_c318)?;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
            c.lwc1(12, c.g[S0][0] as u32);
            c.g[A0][0] = c.g[S6][0];
            c.g[AT][0] = 0x43f0_0000;
            c.mtc1(AT, 13);
            c.g[A1][0] = c.g[S2][0];
            c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x60));
            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x60));
            call_ptr(c, OC1567_PACK_VERTEX, 0x002e_c33c)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(1));
            let more = (c.g[FP][0] as i64) >= 0;
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x60));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002e_c2a0);
            }
        }
        // Submit both strips: `entry71.fn(obj + delta, 0x50, strip, 0)`.
        c.cycles += 36;
        for (strip, ret) in [(0x1f60u32, 0x002e_c370u32), (0x1f64, 0x002e_c394)] {
            c.g[T0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a90)).lw(c);
            c.g[A1][0] = 0x50;
            c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(strip)).lw(c);
            c.g[A3][0] = 0;
            c.g[V0][0] = Addr((c.g[T0][0] as u32).wrapping_add(0x10d8)).lw(c);
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x238)).lh(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x23c)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, ret)?;
        }
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2000));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x1ff0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x1fe0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x1fd0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x1fc0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x1fb0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x1fa0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x1f90));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x1f80));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x1f70));
        c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x2038));
        c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x2030));
        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x2028));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x2020));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x2018));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x2010));
        let target = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2040));
        c.pc = target;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, fake, fall_back, hook, machine};
    use crate::difftest::{Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    // ------------------------------------------------------------ gated fakes

    /// Control word of this module's gated fakes (in the machine's RAM).
    const OC1567_CTRL: u32 = DATA + 0x3d_0000;
    const OC1567_MAGIC: u32 = 0x0c15_6700;
    /// One forced result word per fake.
    const OC1567_RET: u32 = DATA + 0x3d_0010;
    const OC1567_RET_ATTACH: u32 = 0;
    const OC1567_RET_ALLOC: u32 = 1;
    const OC1567_RET_CTOR: u32 = 2;

    /// A logging fake when the control word is set, otherwise whatever was
    /// installed before (chained).
    fn oc1567_gated(
        c: &mut Cpu,
        addr: u32,
        ret_idx: u32,
        prev: &AtomicUsize,
        fallback: fn(&mut Cpu),
    ) {
        if c.bus.read32(OC1567_CTRL) == OC1567_MAGIC {
            let ra = c.g[RA][0] as u32;
            fake(c, addr, ra);
            if c.pc == ra {
                c.g[V0][0] = sx(c.bus.read32(OC1567_RET + 4 * ret_idx));
            }
            return;
        }
        fall_back(c, prev, fallback)
    }

    static OC1567_PREV_ATTACH: AtomicUsize = AtomicUsize::new(0);
    static OC1567_PREV_ALLOC: AtomicUsize = AtomicUsize::new(0);
    static OC1567_PREV_DEAD_CTOR: AtomicUsize = AtomicUsize::new(0);
    static OC1567_PREV_FLEX_CTOR: AtomicUsize = AtomicUsize::new(0);

    fn oc1567_fake_attach(c: &mut Cpu) {
        oc1567_gated(
            c,
            0x002f_ae38,
            OC1567_RET_ATTACH,
            &OC1567_PREV_ATTACH,
            ee::s10::f_002fae38__generated,
        );
    }

    fn oc1567_fake_alloc(c: &mut Cpu) {
        oc1567_gated(
            c,
            0x0031_7d70,
            OC1567_RET_ALLOC,
            &OC1567_PREV_ALLOC,
            ee::s10::f_00317d70__generated,
        );
    }

    fn oc1567_fake_dead_ctor(c: &mut Cpu) {
        oc1567_gated(
            c,
            0x0035_0750,
            OC1567_RET_CTOR,
            &OC1567_PREV_DEAD_CTOR,
            ee::s11::f_00350750__generated,
        );
    }

    fn oc1567_fake_flex_ctor(c: &mut Cpu) {
        oc1567_gated(
            c,
            0x0034_6350,
            OC1567_RET_CTOR,
            &OC1567_PREV_FLEX_CTOR,
            ee::s11::f_00346350__generated,
        );
    }

    fn oc1567_install_factory_fakes() {
        hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_002FAE38),
            oc1567_fake_attach,
            &OC1567_PREV_ATTACH,
        );
        hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc1567_fake_alloc,
            &OC1567_PREV_ALLOC,
        );
        hook(
            std::ptr::addr_of_mut!(ee::s11::OVERRIDE_00350750),
            oc1567_fake_dead_ctor,
            &OC1567_PREV_DEAD_CTOR,
        );
        hook(
            std::ptr::addr_of_mut!(ee::s11::OVERRIDE_00346350),
            oc1567_fake_flex_ctor,
            &OC1567_PREV_FLEX_CTOR,
        );
    }

    /// The shape of one factory's static data.
    struct Oc1567Factory {
        entry: u32,
        init: u32,
        desc: u32,
        /// Words of the description (the argument buffer).
        words: u32,
        types: u32,
        /// Index of the object id in the buffer.
        id_idx: u32,
    }

    /// A fixture for the factories: the init flag set or clear, the type
    /// table, up to three argument entries (one setting the object id), the
    /// instance tables the id resolves through (id -1: the script manager's
    /// object; a section entry; a null section entry; a slot of 0), and an
    /// object with and without an active node, with the attach, allocation
    /// and constructor calls faked.
    fn oc1567_factory_fixture(c: &mut Cpu, seed: u32, salt: u64, fx: &Oc1567Factory) {
        const RETBUF: u32 = DATA + 0x100;
        const ENTRIES: u32 = DATA + 0x200;
        const INST: u32 = DATA + 0x8000;
        const INST_HDR: u32 = DATA + 0x8100;
        const TABLE: u32 = DATA + 0x8200;
        const ENTRY: u32 = DATA + 0x8300;
        const SLOTS: u32 = DATA + 0x8400;
        const OBJECT: u32 = DATA + 0x8800;
        const SCRIPT_OBJECT: u32 = DATA + 0x8a00;
        const MANAGER: u32 = DATA + 0x8b00;
        const NODE: u32 = DATA + 0x8c00;
        let mut r = machine(c, seed, salt, 3);
        c.pc = fx.entry;
        let gp = GAME_GP;
        c.bus.write32(OC1567_CTRL, OC1567_MAGIC);
        c.bus
            .write32(OC1567_RET + 4 * OC1567_RET_ATTACH, (seed / 4) % 2);
        c.bus
            .write32(OC1567_RET + 4 * OC1567_RET_ALLOC, DATA + 0x8_0000);
        c.bus.write32(OC1567_RET + 4 * OC1567_RET_CTOR, 0);
        // The `.lit4` constant the flex-bridge init reads (0.1f).
        c.bus.write32(gp.wrapping_sub(0x3390), 0x3dcc_cccd);
        // The nil Luno value copied into the result.
        for i in 0..3u32 {
            c.bus.write32(0x004c_9098 + 4 * i, r.next_u32());
        }
        // Which object the id names.
        let mode = (seed / 3) % 4;
        let slot = (seed / 12) % 4;
        let table_idx = (seed / 48) % 4;
        let id = if mode == 0 {
            0xffff_ffffu32
        } else {
            (slot << 8) | table_idx
        };
        // Init flag and description.
        let inited = seed % 3 != 0;
        c.bus.write32(gp + fx.init, inited as u32);
        for i in 0..fx.words {
            c.bus.write32(fx.desc + 4 * i, r.next_u32());
        }
        c.bus.write32(fx.desc + 4 * fx.id_idx, id);
        // The per-slot types: the id slot is never a float, the others
        // mostly are or match.
        for i in 0..10u32 {
            let ty = if i == fx.id_idx {
                r.next_u32() % 2
            } else if (i + seed) % 3 == 0 {
                2
            } else {
                r.next_u32() % 4
            };
            c.bus.write32(fx.types + 4 * i, ty);
        }
        // The argument entries.
        let n = (seed % 4) as i32 - 1;
        for i in 0..3u32 {
            let e = ENTRIES + 0x10 * i;
            let idx = if i == 0 && seed % 2 == 0 {
                fx.id_idx
            } else {
                (seed + 2 * i) % fx.words
            };
            let ety = c.bus.read32(fx.types + 4 * idx);
            let val = if idx == fx.id_idx {
                id
            } else {
                r.next_u32() % 1000
            };
            c.bus.write32(e, idx);
            c.bus.write32(e + 4, val);
            c.bus.write32(e + 8, r.next_u32());
            c.bus.write32(
                e + 0xc,
                if (seed + i) % 3 == 0 {
                    ety
                } else {
                    r.next_u32() % 4
                },
            );
        }
        // The section tables the id resolves through.
        c.bus.write32(gp + 0x16c8, INST);
        c.bus.write32(INST, INST_HDR);
        c.bus.write32(INST_HDR + 8, TABLE);
        for i in 0..4u32 {
            c.bus
                .write32(TABLE + 4 * i, if mode == 3 { 0 } else { ENTRY });
        }
        c.bus.write32(ENTRY + 0x1c, SLOTS);
        for i in 0..4u32 {
            let word = if mode == 2 {
                0
            } else {
                ((OBJECT >> 2) << 8) | (r.next_u32() & 0xff)
            };
            c.bus.write32(SLOTS + 4 * i, word);
        }
        // The objects: with and without an active node.
        let node = if seed % 5 < 2 { 0 } else { NODE };
        c.bus.write32(OBJECT + 0xc, node);
        c.bus
            .write32(SCRIPT_OBJECT + 0xc, if seed % 7 < 3 { 0 } else { NODE });
        c.bus.write32(gp + 0xce8, MANAGER);
        c.bus.write32(
            MANAGER + 0x290,
            if seed % 11 == 0 { 0 } else { SCRIPT_OBJECT },
        );
        c.g[A0][0] = sx(RETBUF);
        c.g[A1][0] = n as i64 as u64;
        c.g[A2][0] = sx(ENTRIES);
    }

    /// `WorldObjFactory_DeadFade`: the first-call init (flag clear), the
    /// argument loop (raw and float stores), every id path, the attach check
    /// (answering 0 and 1) and the node build with its fifth constructor
    /// argument.
    #[test]
    fn world_obj_factory_dead_fade_matches() {
        oc1567_install_factory_fakes();
        let fx = Oc1567Factory {
            entry: 0x002f_c5d8,
            init: OC1567_DEAD_FADE_INIT,
            desc: OC1567_DEAD_FADE_DESC,
            words: 5,
            types: OC1567_DEAD_FADE_TYPES,
            id_idx: 0,
        };
        covered(
            ee::s10::f_002fc5d8__generated,
            world_obj_factory_dead_fade,
            240,
            &|c, seed| oc1567_factory_fixture(c, seed, 0x2f_c5d8, &fx),
        );
    }

    /// `WorldObjFactory_FlexBridge`: as the dead-fade test, for the flex
    /// bridge's 10-word buffer (object id at index 8) and its constructor.
    #[test]
    fn world_obj_factory_flex_bridge_matches() {
        oc1567_install_factory_fakes();
        let fx = Oc1567Factory {
            entry: 0x002f_cdc8,
            init: OC1567_FLEX_BRIDGE_INIT,
            desc: OC1567_FLEX_BRIDGE_DESC,
            words: 10,
            types: OC1567_FLEX_BRIDGE_TYPES,
            id_idx: 8,
        };
        covered(
            ee::s10::f_002fcdc8__generated,
            world_obj_factory_flex_bridge,
            240,
            &|c, seed| oc1567_factory_fixture(c, seed, 0x2f_cdc8, &fx),
        );
    }

    // -- MenuSysPoint2DItem_HandleInput -------------------------------------

    const OC1567_RET_DISABLED: u32 = 3;
    const OC1567_RET_CODE: u32 = 4;
    const OC1567_RET_UP: u32 = 5;
    const OC1567_RET_DOWN: u32 = 6;
    const OC1567_RET_LEFT: u32 = 7;
    const OC1567_RET_RIGHT: u32 = 8;
    const OC1567_RET_BASE: u32 = 9;

    /// [`oc1567_gated`], and for the axis getters the forced word also
    /// becomes the float result `f0`.
    fn oc1567_gated_axis(
        c: &mut Cpu,
        addr: u32,
        ret_idx: u32,
        float: bool,
        prev: &AtomicUsize,
        fallback: fn(&mut Cpu),
    ) {
        if c.bus.read32(OC1567_CTRL) == OC1567_MAGIC {
            let ra = c.g[RA][0] as u32;
            fake(c, addr, ra);
            if c.pc == ra {
                let word = c.bus.read32(OC1567_RET + 4 * ret_idx);
                c.g[V0][0] = sx(word);
                if float {
                    c.f[0] = word;
                }
            }
            return;
        }
        fall_back(c, prev, fallback)
    }

    macro_rules! oc1567_item_fake {
        ($fake:ident, $prev:ident, $addr:expr, $idx:expr, $float:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                oc1567_gated_axis(c, $addr, $idx, $float, &$prev, $generated);
            }
        };
    }

    oc1567_item_fake!(
        oc1567_fake_disabled,
        OC1567_PREV_DISABLED,
        0x002c_a3b0,
        OC1567_RET_DISABLED,
        false,
        ee::s09::f_002ca3b0__generated
    );
    oc1567_item_fake!(
        oc1567_fake_code,
        OC1567_PREV_CODE,
        0x002c_a3e8,
        OC1567_RET_CODE,
        false,
        ee::s09::f_002ca3e8__generated
    );
    oc1567_item_fake!(
        oc1567_fake_left,
        OC1567_PREV_LEFT,
        0x002c_a408,
        OC1567_RET_LEFT,
        true,
        ee::s09::f_002ca408__generated
    );
    oc1567_item_fake!(
        oc1567_fake_right,
        OC1567_PREV_RIGHT,
        0x002c_a428,
        OC1567_RET_RIGHT,
        true,
        ee::s09::f_002ca428__generated
    );
    oc1567_item_fake!(
        oc1567_fake_up,
        OC1567_PREV_UP,
        0x002c_a448,
        OC1567_RET_UP,
        true,
        ee::s09::f_002ca448__generated
    );
    oc1567_item_fake!(
        oc1567_fake_down,
        OC1567_PREV_DOWN,
        0x002c_a468,
        OC1567_RET_DOWN,
        true,
        ee::s09::f_002ca468__generated
    );
    oc1567_item_fake!(
        oc1567_fake_base,
        OC1567_PREV_BASE,
        0x002c_a2b0,
        OC1567_RET_BASE,
        false,
        ee::s09::f_002ca2b0__generated
    );

    fn oc1567_install_item_fakes() {
        hook(
            std::ptr::addr_of_mut!(ee::s09::OVERRIDE_002CA3B0),
            oc1567_fake_disabled,
            &OC1567_PREV_DISABLED,
        );
        hook(
            std::ptr::addr_of_mut!(ee::s09::OVERRIDE_002CA3E8),
            oc1567_fake_code,
            &OC1567_PREV_CODE,
        );
        hook(
            std::ptr::addr_of_mut!(ee::s09::OVERRIDE_002CA408),
            oc1567_fake_left,
            &OC1567_PREV_LEFT,
        );
        hook(
            std::ptr::addr_of_mut!(ee::s09::OVERRIDE_002CA428),
            oc1567_fake_right,
            &OC1567_PREV_RIGHT,
        );
        hook(
            std::ptr::addr_of_mut!(ee::s09::OVERRIDE_002CA448),
            oc1567_fake_up,
            &OC1567_PREV_UP,
        );
        hook(
            std::ptr::addr_of_mut!(ee::s09::OVERRIDE_002CA468),
            oc1567_fake_down,
            &OC1567_PREV_DOWN,
        );
        hook(
            std::ptr::addr_of_mut!(ee::s09::OVERRIDE_002CA2B0),
            oc1567_fake_base,
            &OC1567_PREV_BASE,
        );
    }

    /// `MenuSysPoint2DItem_HandleInput`: the input code (6 toggles, 7/8 stop
    /// editing, 4/5 move the selection, anything else drags), the editing
    /// flag, the item disabled or not, the up/down and left/right axes equal
    /// and different, locks set and clear, points inside and outside the
    /// bounds, `order` -1/0/1, counts below and above 25 (the step's
    /// `max(2 * count, 50)`), the selection at the first, last and middle
    /// points, and the menu notification (a faked vtable call). The seven
    /// callees are gated fakes with forced results.
    #[test]
    fn menu_sys_point2d_item_handle_input_matches() {
        use crate::difftest::fakes::FAKE;
        const ITEM: u32 = DATA + 0x1000;
        const MENU: u32 = DATA + 0x2000;
        const MENU_VT: u32 = DATA + 0x2400;
        const PTS: u32 = DATA + 0x3000;
        oc1567_install_item_fakes();
        covered(
            ee::s09::f_002d0908__generated,
            menu_sys_point2d_item_handle_input,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d_0908, 8);
                c.pc = 0x002d_0908;
                c.bus.write32(OC1567_CTRL, OC1567_MAGIC);
                let code = [6u32, 8, 7, 4, 5, 9, 6, 5, 4][(seed % 9) as usize];
                let editing = (seed / 9) % 2;
                c.bus.write32(
                    OC1567_RET + 4 * OC1567_RET_DISABLED,
                    if seed % 13 == 0 { 1 } else { 0 },
                );
                c.bus.write32(OC1567_RET + 4 * OC1567_RET_CODE, code);
                c.bus
                    .write32(OC1567_RET + 4 * OC1567_RET_BASE, r.next_u32());
                // Axes: equal pairs (no drag) and differing ones.
                let axis = |r: &mut Rng| -> u32 {
                    let v = [0.0f32, 1.0, -1.0, 0.5, 2.5, -3.0][(r.next_u32() % 6) as usize];
                    v.to_bits()
                };
                let up = axis(&mut r);
                let right = axis(&mut r);
                let down = if (seed / 18) % 3 == 0 {
                    up
                } else {
                    axis(&mut r)
                };
                let left = if (seed / 54) % 3 == 0 {
                    right
                } else {
                    axis(&mut r)
                };
                c.bus.write32(OC1567_RET + 4 * OC1567_RET_UP, up);
                c.bus.write32(OC1567_RET + 4 * OC1567_RET_DOWN, down);
                c.bus.write32(OC1567_RET + 4 * OC1567_RET_RIGHT, right);
                c.bus.write32(OC1567_RET + 4 * OC1567_RET_LEFT, left);
                // `.lit4`: 1 / 120 at gp - 0x3fa0 and gp - 0x3f9c.
                let gp = GAME_GP;
                c.bus.write32(gp.wrapping_sub(0x3fa0), 0x3c08_8889);
                c.bus.write32(gp.wrapping_sub(0x3f9c), 0x3c08_8889);
                // The item.
                let count = [1u32, 2, 3, 5, 30, 4][((seed / 7) % 6) as usize];
                let cur = match (seed / 3) % 4 {
                    0 => 0,
                    1 => count - 1,
                    _ => (seed / 5) % count,
                };
                let order = [0i32, 1, -1][((seed / 11) % 3) as usize];
                c.bus.write32(ITEM + OC1567_P2D_MENU, MENU);
                c.bus.write32(MENU + 0x12c, MENU_VT);
                c.bus.write32(MENU_VT + 0x20, 0x10 + seed % 3);
                c.bus.write32(MENU_VT + 0x24, FAKE + 0x80);
                c.bus.write32(ITEM + OC1567_P2D_COUNT, count);
                let lock = |r: &mut Rng, cur: u32, on: bool| -> u32 {
                    let mut m = r.next_u32();
                    if on {
                        m |= 1 << cur;
                    } else {
                        m &= !(1 << cur);
                    }
                    m
                };
                let lock_x = lock(&mut r, cur, seed % 17 == 0);
                let lock_y = lock(&mut r, cur, seed % 19 == 0);
                c.bus.write32(ITEM + OC1567_P2D_LOCK_X, lock_x);
                c.bus.write32(ITEM + OC1567_P2D_LOCK_Y, lock_y);
                c.bus.write32(ITEM + OC1567_P2D_CUR, cur);
                c.bus.write32(ITEM + OC1567_P2D_PTS, PTS);
                // Bounds (x -10..10, y 0..20) and points around them.
                let (min_x, max_x, min_y, max_y) = (-10.0f32, 10.0f32, 0.0f32, 20.0f32);
                for (i, v) in [min_x, min_y, max_x, max_y].into_iter().enumerate() {
                    c.bus
                        .write32(ITEM + OC1567_P2D_BOUNDS + 4 * i as u32, v.to_bits());
                }
                c.bus.write32(ITEM + OC1567_P2D_ORDER, order as u32);
                c.bus.write32(ITEM + OC1567_P2D_EDITING, editing);
                for i in 0..32u32 {
                    // Mostly inside the bounds, sometimes beyond one.
                    let x = (r.next_u32() % 2600) as f32 / 100.0 - 13.0;
                    let y = (r.next_u32() % 2600) as f32 / 100.0 - 3.0;
                    c.bus.write32(PTS + 8 * i, x.to_bits());
                    c.bus.write32(PTS + 8 * i + 4, y.to_bits());
                }
                c.g[A0][0] = sx(ITEM);
            },
        );
    }

    // -- cCircle_TObj_Draw --------------------------------------------------

    const OC1567_RET_DRAW: u32 = 10;
    static OC1567_PREV_DRAW_RECT: AtomicUsize = AtomicUsize::new(0);

    fn oc1567_fake_draw_rect(c: &mut Cpu) {
        oc1567_gated(
            c,
            0x002e_b938,
            OC1567_RET_DRAW,
            &OC1567_PREV_DRAW_RECT,
            ee::s09::f_002eb938__generated,
        );
    }

    /// `cCircle_TObj_Draw`: wipe progress `t` from 0 (a radius that clamps
    /// every vertex) to beyond 1 (negative radius), centres inside and
    /// outside the screen; `TransitionEffect_DrawRect` is a gated fake, the
    /// vertex packer (no generated function) and the two vtable submits are
    /// logged by the machine's miss hook.
    #[test]
    fn ccircle_tobj_draw_matches() {
        use crate::difftest::fakes::FAKE;
        const SELF: u32 = DATA + 0x1000;
        const DRAW_OBJ: u32 = DATA + 0x2000;
        const TABLE: u32 = DATA + 0x2400;
        const SINE: u32 = 0x0050_4fb8;
        hook(
            std::ptr::addr_of_mut!(ee::s09::OVERRIDE_002EB938),
            oc1567_fake_draw_rect,
            &OC1567_PREV_DRAW_RECT,
        );
        covered(
            ee::s09::f_002ec060__generated,
            ccircle_tobj_draw,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e_c060, 164);
                c.pc = 0x002e_c060;
                c.bus.write32(OC1567_CTRL, OC1567_MAGIC);
                let gp = GAME_GP;
                // `.lit4`: 1 / 640, the arc step (pi / 39), 512 / (2 pi).
                c.bus
                    .write32(gp.wrapping_sub(0x3984), 0.0015625f32.to_bits());
                c.bus
                    .write32(gp.wrapping_sub(0x3980), 0.080_553_66f32.to_bits());
                c.bus
                    .write32(gp.wrapping_sub(0x397c), 81.487_25f32.to_bits());
                // The texture window (floats at gp + 0xa40..0xa54).
                for i in 0..6u32 {
                    c.bus.write32(gp + 0xa40 + 4 * i, r.f32(2.0));
                }
                // The 640-entry sine table.
                for i in 0..640u32 {
                    let a = (i as f32) * (std::f32::consts::TAU / 512.0);
                    c.bus.write32(SINE + 4 * i, a.sin().to_bits());
                }
                // The transition: centre, colour block.
                let cx = [320.0f32, -40.0, 700.0, 100.0][(seed % 4) as usize];
                let cy = [240.0f32, 20.0, 500.0, 240.0][((seed / 4) % 4) as usize];
                c.bus.write32(SELF + 0x14, cx.to_bits());
                c.bus.write32(SELF + 0x18, cy.to_bits());
                for i in 0..3u32 {
                    c.bus.write32(SELF + 4 + 4 * i, r.next_u32());
                }
                // The draw object: `obj[0x10d8] -> table`, whose entry
                // (+0x238 delta, +0x23c function) is a call the machine fakes.
                c.bus.write32(gp + 0x2a90, DRAW_OBJ);
                c.bus.write32(DRAW_OBJ + 0x10d8, TABLE);
                c.bus.write16(TABLE + 0x238, (r.next_u32() % 0x40) as u16);
                c.bus.write32(TABLE + 0x23c, FAKE + 0x80);
                // Progress 0..1.2 (negative radius beyond 1).
                let t = [0.0f32, 0.25, 0.5, 0.9, 1.0, 1.2][((seed / 16) % 6) as usize];
                c.f[12] = t.to_bits();
                c.g[A0][0] = sx(SELF);
            },
        );
    }

    // -- EACore_StaticInitDestroy -------------------------------------------

    static OC1567_PREV_BLOCK_MAN: AtomicUsize = AtomicUsize::new(0);
    static OC1567_PREV_RAND: AtomicUsize = AtomicUsize::new(0);
    static OC1567_PREV_CORE_CTOR: AtomicUsize = AtomicUsize::new(0);
    static OC1567_PREV_CORE_DTOR: AtomicUsize = AtomicUsize::new(0);

    fn oc1567_fake_block_man(c: &mut Cpu) {
        oc1567_gated(
            c,
            0x0031_6a00,
            OC1567_RET_CTOR,
            &OC1567_PREV_BLOCK_MAN,
            ee::s10::f_00316a00__generated,
        );
    }

    fn oc1567_fake_rand(c: &mut Cpu) {
        oc1567_gated(
            c,
            0x0031_7930,
            OC1567_RET_CTOR,
            &OC1567_PREV_RAND,
            ee::s10::f_00317930__generated,
        );
    }

    fn oc1567_fake_core_ctor(c: &mut Cpu) {
        oc1567_gated(
            c,
            0x0031_ff20,
            OC1567_RET_CTOR,
            &OC1567_PREV_CORE_CTOR,
            ee::s10::f_0031ff20__generated,
        );
    }

    fn oc1567_fake_core_dtor(c: &mut Cpu) {
        oc1567_gated(
            c,
            0x0031_ff38,
            OC1567_RET_CTOR,
            &OC1567_PREV_CORE_DTOR,
            ee::s10::f_0031ff38__generated,
        );
    }

    /// `EACore_StaticInitDestroy`: another priority (nothing happens), the
    /// destroy path (`init` clear) and the construct path (the whole flat
    /// store block); the five callees are faked.
    #[test]
    fn eacore_static_init_destroy_matches() {
        hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00316A00),
            oc1567_fake_block_man,
            &OC1567_PREV_BLOCK_MAN,
        );
        hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317930),
            oc1567_fake_rand,
            &OC1567_PREV_RAND,
        );
        hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_0031FF20),
            oc1567_fake_core_ctor,
            &OC1567_PREV_CORE_CTOR,
        );
        hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_0031FF38),
            oc1567_fake_core_dtor,
            &OC1567_PREV_CORE_DTOR,
        );
        covered(
            ee::s10::f_00320550__generated,
            eacore_static_init_destroy,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32_0550, 4);
                c.pc = 0x0032_0550;
                c.bus.write32(OC1567_CTRL, OC1567_MAGIC);
                c.bus
                    .write32(OC1567_RET + 4 * OC1567_RET_CTOR, r.next_u32());
                // The globals the static init overwrites hold junk first.
                for i in 0..0x180u32 {
                    c.bus.write32(0x004f_f000 + 4 * i, r.next_u32());
                }
                let init = if seed % 4 == 0 { 0 } else { r.next_u32() | 1 };
                c.g[A0][0] = sx(init);
                c.g[A1][0] = match seed % 3 {
                    0 => 0xffff,
                    1 => 0xffff,
                    _ => {
                        if seed % 2 == 0 {
                            0x1_0000 | 0xffff
                        } else {
                            r.next_u32() as u64
                        }
                    }
                };
            },
        );
    }
}
