//! Staging module oc588 (wave wave233): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc588`.

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
        0x0035_3f00,
        flex_bridge_alloc_spring_nodes,
        "FlexBridge_AllocSpringNodes",
    ),
    (0x0035_4688, game_process_ctor_2, "GameProcess_ctor_2"),
    (0x0035_49d0, process_list_ctor, "ProcessList_ctor"),
    (
        0x0035_4e78,
        process_list_remove_and_unlink,
        "ProcessList_RemoveAndUnlink",
    ),
    (
        0x0035_5878,
        mod_obj_set_field24_from_arg,
        "ModObj_SetField24FromArg",
    ),
    (
        0x0035_5888,
        mod_obj_call_v41_with_arg,
        "ModObj_CallV41WithArg",
    ),
    (
        0x0035_6f68,
        position_modifier_ctor_3,
        "PositionModifier_ctor_3",
    ),
    (
        0x0035_7798,
        ava_spline_modifier_ctor_2,
        "AvaSplineModifier_ctor_2",
    ),
    (
        0x0035_8468,
        boost_modifier_base_ctor_2,
        "BoostModifier_BaseCtor_2",
    ),
    (
        0x0035_cfe8,
        roller_modifier_member_ctor_stub,
        "RollerModifier_MemberCtorStub",
    ),
];

/// `ProcessList_Add(list, process, kind)` (0x354c08), called by the
/// `GameProcess` constructors.
const OC588_PROCESS_LIST_ADD: Callee = Callee {
    addr: 0x0035_4c08,
    f: ee::s11::ProcessList_Add,
};

/// `ProcessList_Remove(list, process)` (0x354e48).
const OC588_PROCESS_LIST_REMOVE: Callee = Callee {
    addr: 0x0035_4e48,
    f: ee::s11::ProcessList_Remove,
};

/// `void cSpring_setupNodes(cSpring* self)` (0x353f00; the FlexBridge spring
/// setup). `self+0` holds the node count. The function allocates
/// `count * 0x50` bytes (`operator new`, the block stored at `self+0x24`) and
/// copies a 0x30-byte template into each node, clearing its words at
/// +0x20, +0x24 and +0x40. A zero count leaves the block unused.
pub fn flex_bridge_alloc_spring_nodes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = 0x0049_0000;
        frame.save(c, S1, 0x20);
        c.g[A2][0] = 0x2000_0000;
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x50;
        frame.save(c, S0, 0x30);
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x1638));
        c.g[S0][0] = Addr(r32(c, S1)).lw(c);
        c.g[A3][0] = 0;
        c.mult(0, 0, S0, A0);
        c.g[A0][0] = c.lo[0];
        c.g[S2][0] = sx(r32(c, S1).wrapping_add(0x24));
        call_ptr(c, 0x0031_7e30, 0x0035_3f40)?;
        let empty = c.g[S0][0] == 0;
        c.g[V1][0] = sx(r32(c, S0).wrapping_sub(1));
        if !empty {
            c.cycles += 2;
            c.g[A0][0] = u64::MAX;
        }
        // Block 2: store the block, then fill the nodes.
        c.cycles += 5;
        let block = c.g[V0][0];
        Addr(r32(c, S2)).sw(c, block);
        c.g[A1][0] = 0;
        c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        let no_nodes = (c.g[V0][0] as i64) <= 0;
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x24)).lw(c);
        if !no_nodes {
            c.cycles += 3;
            c.g[V0][0] = 0x0050_0000;
            c.g[A2][0] = sx(r32(c, V0).wrapping_sub(0xee0));
            loop {
                c.cycles += 14;
                c.lq(V1, r32(c, A2));
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                c.sq(V1, r32(c, A0));
                c.lq(V0, r32(c, A2));
                c.sq(V0, r32(c, A0).wrapping_add(0x10));
                c.lq(V1, r32(c, A2));
                c.sw(r32(c, A0).wrapping_add(0x24), 0);
                c.sq(V1, r32(c, A0).wrapping_add(0x30));
                c.sw(r32(c, A0).wrapping_add(0x20), 0);
                c.sw(r32(c, A0).wrapping_add(0x40), 0);
                c.g[V0][0] = Addr(r32(c, S1)).lw(c);
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x50));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_3f70);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `GameProcess* GameProcess_ctor_2(GameProcess* self, int kind)` (0x354688):
/// installs the `GameProcess` vtable at `+0xc` and adds the process to the
/// global process list; returns `self`.
pub fn game_process_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x1f00));
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A1][0];
        let vtable = c.g[V0][0];
        Addr(r32(c, S0).wrapping_add(0xc)).sw(c, vtable);
        c.g[A0][0] = sx(r32(c, GP).wrapping_add(0x2898));
        c.g[A1][0] = c.g[S0][0];
        OC588_PROCESS_LIST_ADD.call(c, 0x0035_46b4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void ProcessList_ctor(ProcessList* self)` (0x3549d0): an empty list
/// (both head words cleared); returns `self`.
pub fn process_list_ctor(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = c.g[A0][0];
    let head = r32(c, A0);
    Addr(head.wrapping_add(0x4)).sw(c, 0);
    Addr(head).sw(c, 0);
    ret(c);
}

/// `void ProcessList_RemoveAndUnlink(ProcessList* list, GameProcess* p)`
/// (0x354e78): removes `p` from `list`, then clears its prev/next links.
pub fn process_list_remove_and_unlink(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        OC588_PROCESS_LIST_REMOVE.call(c, 0x0035_4e8c)?;
        let p = r32(c, S0);
        Addr(p.wrapping_add(0x8)).sw(c, 0);
        Addr(p.wrapping_add(0x4)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ModObj_ctor_2(ModObj* self, int, WorldObj* owner)` (0x3552d8): the
/// modifier object's base construction. Builds the world-object base, installs
/// the `ModObj` vtable (0x491028), calls the owner's four virtual hooks with
/// `self+0x20/0x24/0x28` and the frame's scratch word, then creates the
/// modifier block when that word is set and sets bit 0 of `self+0x12`.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn mod_obj_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 42;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0034_fb90, 0x0035_52f4)?;
        c.g[V1][0] = 0x0049_0000;
        Addr(r32(c, S0).wrapping_add(0x1c)).sw(c, 0);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x1028));
        c.g[A1][0] = sx(r32(c, S0).wrapping_add(0x20));
        let vtable = c.g[V1][0];
        Addr(r32(c, S0).wrapping_add(0xc)).sw(c, vtable);
        c.g[A2][0] = 4;
        c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x10)).lh(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x14)).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0035_5320)?;
        c.g[A3][0] = Addr(r32(c, S1)).lw(c);
        c.g[A1][0] = sx(r32(c, S0).wrapping_add(0x24));
        c.g[A2][0] = 4;
        c.g[A0][0] = Addr(r32(c, A3).wrapping_add(0x10)).lh(c);
        c.g[V0][0] = Addr(r32(c, A3).wrapping_add(0x14)).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0035_533c)?;
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A1][0] = sx(r32(c, S0).wrapping_add(0x28));
        c.g[A2][0] = 4;
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x10)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x14)).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0035_5358)?;
        c.g[A3][0] = Addr(r32(c, S1)).lw(c);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 4;
        c.g[A0][0] = Addr(r32(c, A3).wrapping_add(0x10)).lh(c);
        c.g[V1][0] = Addr(r32(c, A3).wrapping_add(0x14)).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0035_5374)?;
        c.g[V0][0] = Addr(sp(c)).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0035_5468, 0x0035_5388)?;
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x1c)).lw(c);
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0035_36d8, 0x0035_5394)?;
        }
        c.cycles += 11;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0035_6780, 0x0035_539c)?;
        c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0x12)).lhu(c);
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] |= 0x1;
        let flags = c.g[V1][0];
        Addr(r32(c, S0).wrapping_add(0x12)).sh(c, flags);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void ModObj_SetField24FromArg(ModObj* self, const Vec* v)` (0x355878):
/// copies the float at `v+4` to `self+0x24`.
pub fn mod_obj_set_field24_from_arg(c: &mut Cpu) {
    c.cycles += 3;
    c.lwc1(0, r32(c, A1).wrapping_add(0x4));
    c.swc1(0, r32(c, A0).wrapping_add(0x24));
    ret(c);
}

/// `void ModObj_CallV41WithArg(ModObj* self, const Vec* v)` (0x355888): calls
/// the virtual hook at `sub+0x14c` of the sub-object at `self+0xc` (its
/// receiver offset is the halfword at `sub+0x148`), passing `v+4` as f12.
pub fn mod_obj_call_v41_with_arg(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0xc)).lw(c);
        c.lwc1(12, r32(c, A1).wrapping_add(0x4));
        let sub = r32(c, V0);
        c.g[V1][0] = Addr(sub.wrapping_add(0x148)).lh(c);
        c.g[A1][0] = Addr(sub.wrapping_add(0x14c)).lw(c);
        let target = r32(c, A1);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        call_ptr(c, target, 0x0035_58a8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `PositionModifier* PositionModifier_ctor_3(PositionModifier* self, Obj* o)`
/// (0x356f68): calls the source object's virtual hook (receiver adjusted by
/// the halfword at its vtable +0x10), copies a 0x40-byte block from the
/// template at 0x4ff1a0 into `self+0x50`, and sets `self+0x44` to 1.
pub fn position_modifier_ctor_3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xa10));
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let vtable = c.g[V0][0];
        Addr(r32(c, S0)).sw(c, vtable);
        c.g[A0][0] = c.g[A1][0];
        c.g[A1][0] = sx(r32(c, S0).wrapping_add(0x10));
        c.g[A2][0] = 0x40;
        c.g[V0][0] = Addr(r32(c, A0)).lw(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x10)).lh(c);
        c.g[A3][0] = Addr(r32(c, V0).wrapping_add(0x14)).lw(c);
        let target = r32(c, A3);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        call_ptr(c, target, 0x0035_6fa4)?;
        c.g[V0][0] = 0x0050_0000;
        c.g[V1][0] = sx(r32(c, S0).wrapping_add(0x50));
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xe60));
        c.lqc2(1, r32(c, V0));
        c.lqc2(2, r32(c, V0).wrapping_add(0x10));
        c.lqc2(3, r32(c, V0).wrapping_add(0x20));
        c.lqc2(4, r32(c, V0).wrapping_add(0x30));
        c.sqc2(1, r32(c, V1));
        c.sqc2(2, r32(c, V1).wrapping_add(0x10));
        c.sqc2(3, r32(c, V1).wrapping_add(0x20));
        c.sqc2(4, r32(c, V1).wrapping_add(0x30));
        c.g[A0][0] = 0x1;
        c.g[V0][0] = c.g[S0][0];
        Addr(r32(c, S0).wrapping_add(0x44)).sw(c, 1);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `AvaSplineModifier* AvaSplineModifier_ctor_2(self, Obj* o)` (0x357798):
/// the base vtable (0x48f338) and the two hooks of `o` (the first returns the
/// value kept at `self+0x80`, the second fills `self+0x8c..`), then the
/// spline-setup call on `self+0x80` with `self+0x40`.
pub fn ava_spline_modifier_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S1, 0x20);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xcc8));
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(r32(c, S1).wrapping_add(0x10));
        let vtable = c.g[V0][0];
        Addr(r32(c, S1)).sw(c, vtable);
        c.g[V1][0] = Addr(r32(c, S0)).lw(c);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x18)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x1c)).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0035_77d8)?;
        let first = c.g[V0][0];
        Addr(r32(c, S1).wrapping_add(0x80)).sw(c, first);
        c.g[A2][0] = 0x30;
        c.g[A1][0] = c.g[S2][0];
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x10)).lh(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x14)).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0035_77f8)?;
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x80)).lw(c);
        c.g[A1][0] = sx(r32(c, S1).wrapping_add(0x40));
        call_ptr(c, 0x002d_1cf0, 0x0035_7804)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BoostModifier* BoostModifier_BaseCtor_2(BoostModifier* self, Obj* o)`
/// (0x358468): installs the base vtable (0x4913b8) at `self+4`, then calls
/// the source object's hook (receiver adjusted by the halfword at its
/// vtable +0x10) with `self` as the argument; returns `self`.
pub fn boost_modifier_base_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x13b8));
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let vtable = c.g[V0][0];
        Addr(r32(c, S0).wrapping_add(0x4)).sw(c, vtable);
        c.g[A0][0] = c.g[A1][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = 0x4;
        c.g[V0][0] = Addr(r32(c, A0)).lw(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x10)).lh(c);
        c.g[A3][0] = Addr(r32(c, V0).wrapping_add(0x14)).lw(c);
        let target = r32(c, A3);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        call_ptr(c, target, 0x0035_84a4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `AnimDelta* AnimDelta_ctor_2(AnimDelta* self, int, AnimCombo* combo?)`
/// (0x359270): builds the `AnimObject` base (0x341c80), installs the
/// `AnimDelta` vtable (0x48ea70) at `self+0x3c` and calls the second
/// argument's hook (receiver adjusted by the halfword at its vtable +0x10)
/// with `self+0x6c` as the argument; returns `self`.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn anim_delta_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0034_1c80, 0x0035_928c)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[A1][0] = sx(r32(c, S0).wrapping_add(0x6c));
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x1590));
        c.g[A2][0] = 4;
        let vtable = c.g[V1][0];
        Addr(r32(c, S0).wrapping_add(0x3c)).sw(c, vtable);
        c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x10)).lh(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x14)).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0035_92b4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The member-constructor stub of `RollerModifier` (0x35cfe8): returns its
/// first argument unchanged.
pub fn roller_modifier_member_ctor_stub(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL;
    use crate::game::object::spring::tests::oc300_process_list;
    use crate::game::worldpainter::quadtree::tests::{heap_init, pool_allocator};

    /// An object whose vtable (at `obj+0`) holds the receiver offset (a
    /// halfword at +0x10) and the function (at +0x14) for one virtual call.
    fn vt_obj(c: &mut Cpu, obj: u32, vt: u32, delta: u16, f: u32) {
        c.bus.write32(obj, vt);
        c.bus.write16(vt + 0x10, delta);
        c.bus.write32(vt + 0x14, f);
    }

    #[test]
    fn flex_bridge_alloc_spring_nodes_matches() {
        check(
            ee::s11::f_00353f00__generated,
            flex_bridge_alloc_spring_nodes,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x353f00, 1);
                pool_allocator(c);
                heap_init(c);
                let this = DATA + 0x100;
                c.bus.write32(this, seed % 5);
                c.bus.write32(this + 0x24, r.next_u32());
                c.g[A0][0] = sx(this);
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0035_3f00;
            },
        );
    }

    #[test]
    fn game_process_ctor_2_matches() {
        check(
            ee::s11::f_00354688__generated,
            game_process_ctor_2,
            300,
            &|c, seed| {
                let mut r = Rng(0x3546_8800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                oc300_process_list(c, &mut r);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = (r.next_u32() % 4) as u64;
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0035_4688;
            },
        );
    }

    #[test]
    fn process_list_ctor_matches() {
        check(
            ee::s11::f_003549d0__generated,
            process_list_ctor,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3549d0, 0);
                let list = DATA + 0x100;
                c.bus.write32(list, r.next_u32());
                c.bus.write32(list + 4, r.next_u32());
                c.g[A0][0] = sx(list);
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0035_49d0;
            },
        );
    }

    #[test]
    fn process_list_remove_and_unlink_matches() {
        use crate::game::object::objnode::tests::{oc286_heap_process, oc286_process_list};
        check(
            ee::s11::f_00354e78__generated,
            process_list_remove_and_unlink,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0x354e78, 0);
                oc286_process_list(c, &mut r);
                let this = oc286_heap_process(c, &mut r, seed);
                c.g[A0][0] = sx(GAME_GP + 0x2898);
                c.g[A1][0] = sx(this);
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0035_4e78;
            },
        );
    }

    #[test]
    fn mod_obj_set_field24_from_arg_matches() {
        check(
            ee::s11::f_00355878__generated,
            mod_obj_set_field24_from_arg,
            20,
            &|c, seed| {
                let mut r = machine(c, seed, 0x355878, 0);
                let this = DATA + 0x100;
                let v = DATA + 0x200;
                c.bus.write32(v + 4, r.next_u32());
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(v);
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0035_5878;
            },
        );
    }

    #[test]
    fn mod_obj_call_v41_with_arg_matches() {
        check(
            ee::s11::f_00355888__generated,
            mod_obj_call_v41_with_arg,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x355888, 1);
                let this = DATA + 0x100;
                let sub = DATA + 0x800;
                let v = DATA + 0x200;
                c.bus.write32(this + 0xc, sub);
                c.bus.write16(sub + 0x148, (r.next_u32() & 0xff) as u16);
                c.bus.write32(sub + 0x14c, FAKE + 0x40);
                c.bus.write32(v + 4, r.next_u32());
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(v);
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0035_5888;
            },
        );
    }

    #[test]
    fn position_modifier_ctor_3_matches() {
        check(
            ee::s11::f_00356f68__generated,
            position_modifier_ctor_3,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x356f68, 1);
                let this = DATA + 0x100;
                let o = DATA + 0x200;
                let vt = DATA + 0x400;
                vt_obj(c, o, vt, (seed % 3) as u16 * 0x10, FAKE + 0x40);
                c.bus.write32(this + 0x44, r.next_u32());
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(o);
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0035_6f68;
            },
        );
    }

    #[test]
    fn ava_spline_modifier_ctor_2_matches() {
        check(
            ee::s11::f_00357798__generated,
            ava_spline_modifier_ctor_2,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x357798, 2);
                let this = DATA + 0x100;
                let o = DATA + 0x200;
                let vt = DATA + 0x400;
                vt_obj(c, o, vt, 0x10, FAKE + 0x40);
                c.bus.write16(vt + 0x18, 0x8);
                c.bus.write32(vt + 0x1c, FAKE + 0x48);
                c.bus.write32(this + 0x80, r.next_u32());
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(o);
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0035_7798;
            },
        );
    }

    #[test]
    fn boost_modifier_base_ctor_2_matches() {
        check(
            ee::s11::f_00358468__generated,
            boost_modifier_base_ctor_2,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x358468, 1);
                let this = DATA + 0x100;
                let o = DATA + 0x200;
                let vt = DATA + 0x400;
                vt_obj(c, o, vt, (seed % 3) as u16 * 0x10, FAKE + 0x40);
                c.bus.write32(this + 0x4, r.next_u32());
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(o);
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0035_8468;
            },
        );
    }

    #[test]
    fn roller_modifier_member_ctor_stub_matches() {
        check(
            ee::s11::f_0035cfe8__generated,
            roller_modifier_member_ctor_stub,
            20,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35cfe8, 0);
                c.g[A0][0] = sx(DATA + 0x100 + (r.next_u32() & 0xf0));
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0035_cfe8;
                let _ = seed;
            },
        );
    }
}
