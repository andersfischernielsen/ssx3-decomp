//! Staging module oc1052 (wave wave264): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1052`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002b_7410,
        world_sound_unlink_emitter,
        "cWorldSound_UnlinkEmitter",
    ),
    (
        0x002c_06c0,
        world_painter_query_dtor,
        "WorldPainterQuery_dtor",
    ),
    (
        0x002c_1688,
        world_painter_query_global_ctors,
        "WorldPainterQuery_GlobalCtors",
    ),
    (0x002c_17d0, quad_tree_alloc_node, "QuadTree_AllocNode"),
    (0x002c_1950, quad_tree_insert, "QuadTree_Insert"),
    (0x002c_2178, quad_tree_count_nodes, "QuadTree_CountNodes"),
    (
        0x002c_3fa0,
        game_module_unload_stub,
        "GameModule_UnloadStub",
    ),
    (0x002d_4a70, anger_fx_dtor, "cAngerFX_dtor"),
];

/// Entry of `cWorldSound_UnlinkEmitter` (the dispatcher's entry `pc`).
const UNLINK_EMITTER_ENTRY: u32 = 0x002b_7410;

/// `cWorldSound_ComputeRegionBounds` (0x2b6cf8).
const WS_COMPUTE_REGION_BOUNDS: Callee = Callee {
    addr: 0x002b_6cf8,
    f: ee::s08::cWorldSound_ComputeRegionBounds,
};
/// `cWorldSound_RemoveEmitter` (0x2b77c0).
const WS_REMOVE_EMITTER: Callee = Callee {
    addr: 0x002b_77c0,
    f: ee::s08::cWorldSound_RemoveEmitter,
};
/// `WorldPainterQuery_StaticInitDestroy` (0x2c0b70).
const WPQ_STATIC_INIT_DESTROY: Callee = Callee {
    addr: 0x002c_0b70,
    f: ee::s08::WorldPainterQuery_StaticInitDestroy,
};
/// `QuadTree_InsertRecursive` (0x2c1a00).
const QT_INSERT_RECURSIVE: Callee = Callee {
    addr: 0x002c_1a00,
    f: ee::s08::QuadTree_InsertRecursive,
};
/// `QuadTree_CountNodes` (0x2c2178), called on itself for the children.
const QT_COUNT_NODES: Callee = Callee {
    addr: 0x002c_2178,
    f: ee::s08::QuadTree_CountNodes,
};

/// The register `off` bytes above the value of `reg`, as an address.
#[inline(always)]
fn oc1052_at(c: &Cpu, reg: usize, off: u32) -> Addr {
    Addr((c.g[reg][0] as u32).wrapping_add(off))
}

/// `void cWorldSound_UnlinkEmitter(self, e)` (0x2b7410): takes emitter `e` out of
/// the world-sound grid cell (and frees the cell's list when `e` was its last
/// instance) and removes it from the overlap list. A null `e` does nothing.
/// Written as the generated dispatcher's normal entry; any other `pc` (a resume
/// after an unwound callee or event) runs the generated body.
pub fn world_sound_unlink_emitter(c: &mut Cpu) {
    if c.pc != UNLINK_EMITTER_ENTRY {
        return ee::s08::f_002b7410__generated(c);
    }
    guest(c, unlink_emitter_body);
}

fn unlink_emitter_body(c: &mut Cpu) -> Flow {
    c.cycles += 8;
    let f = Frame::open(c, 0x90);
    f.save(c, S1, 0x70);
    f.save(c, S2, 0x60);
    c.g[S1][0] = c.g[A1][0];
    f.save(c, S0, 0x80);
    c.g[S2][0] = c.g[A0][0];
    let null_e = c.g[S1][0] == 0;
    f.save_ra(c, 0x50);
    if null_e {
        return unlink_emitter_exit_full(c, f);
    }
    // 0x2b7430: the grid cell of the emitter.
    c.cycles += 3;
    let cell = oc1052_at(c, S1, 0xc).lw(c);
    c.g[V0][0] = cell;
    let no_cell = cell == 0;
    f.restore(c, S0, 0x80);
    if no_cell {
        return unlink_emitter_exit(c, f);
    }
    // 0x2b743c: the cell's first list entry.
    c.cycles += 4;
    let first = oc1052_at(c, S1, 0x10).lw(c);
    c.g[V0][0] = first;
    c.g[V1][0] = oc1052_at(c, V0, 4).lw(c);
    let no_entry = c.g[V1][0] == 0;
    c.g[A3][0] = sx(sp(c).wrapping_add(0x10));
    if no_entry {
        return unlink_emitter_exit(c, f);
    }
    // 0x2b744c: bounds of the emitter, then the audio and world calls.
    c.cycles += 50;
    c.g[A2][0] = c.g[SP][0];
    WS_COMPUTE_REGION_BOUNDS.call(c, 0x002b_7454)?;
    call_ptr(c, 0x0028_b1c8, 0x002b_745c)?;
    c.lwc1(6, sp(c));
    c.g[A0][0] = sx(sp(c).wrapping_add(0x40));
    c.lwc1(5, sp(c).wrapping_add(4));
    c.g[A1][0] = sx(sp(c).wrapping_add(0x20));
    c.lwc1(1, sp(c).wrapping_add(8));
    c.lwc1(2, sp(c).wrapping_add(0x10));
    c.lwc1(3, sp(c).wrapping_add(0x14));
    c.lwc1(4, sp(c).wrapping_add(0x18));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 0);
    c.g[S0][0] = oc1052_at(c, V0, 0x20).lw(c);
    c.swc1(6, sp(c).wrapping_add(0x20));
    c.swc1(5, sp(c).wrapping_add(0x24));
    c.swc1(1, sp(c).wrapping_add(0x28));
    c.swc1(2, sp(c).wrapping_add(0x30));
    c.swc1(3, sp(c).wrapping_add(0x34));
    c.swc1(4, sp(c).wrapping_add(0x38));
    c.swc1(0, sp(c).wrapping_add(0x3c));
    c.swc1(0, sp(c).wrapping_add(0x2c));
    let count = oc1052_at(c, S0, 0xa0).lw(c);
    let count = sx((count as u32).wrapping_add(1));
    c.g[V0][0] = count;
    oc1052_at(c, S0, 0xa0).sw(c, count);
    call_ptr(c, 0x0032_8f28, 0x002b_74b8)?;
    // 0x2b74b8: which of the two candidate cells the emitter belongs to.
    c.g[V0][0] = oc1052_at(c, SP, 0x44).lw(c);
    c.g[A3][0] = u64::MAX;
    c.g[A0][0] = oc1052_at(c, SP, 0x48).lw(c);
    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
    c.g[A1][0] = oc1052_at(c, SP, 0x4c).lw(c);
    if c.g[V0][0] != 0 {
        c.g[V1][0] = c.g[S0][0];
    }
    c.g[A0][0] = ((c.g[A3][0] as i64) < (c.g[A0][0] as i64)) as u64;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x28));
    c.g[A3][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
    if c.g[A0][0] != 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    c.g[A1][0] = 2;
    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
    c.g[A2][0] = c.g[S1][0];
    if c.g[A3][0] != 0 {
        c.g[S0][0] = c.g[V0][0];
    }
    c.g[A0][0] = oc1052_at(c, S0, 0x10).lw(c);
    c.g[A3][0] = sx(sp(c).wrapping_add(0x40));
    c.g[T0][0] = c.g[S0][0];
    call_ptr(c, 0x0032_84b8, 0x002b_7504)?;
    // 0x2b7504 .. 0x2b755c: is any of the cell's six slots in use?
    c.g[V1][0] = oc1052_at(c, S0, 0x10).lw(c);
    c.g[V0][0] = oc1052_at(c, V1, 0).lw(c);
    let mut busy = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if !busy {
        for off in [4u32, 8, 0xc, 0x10, 0x14, 0x18] {
            c.cycles += 3;
            c.g[V0][0] = oc1052_at(c, V1, off).lw(c);
            busy = c.g[V0][0] != 0;
            c.g[V0][0] = 0;
            if busy {
                break;
            }
        }
    }
    // Slot-scan tail: 0x2b755c (the last slot, then the walk over the rest).
    let mut walk = false;
    if !busy {
        c.cycles += 3;
        c.g[V0][0] = oc1052_at(c, V1, 0x1c).lw(c);
        let last_empty = c.g[V0][0] == 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
        if last_empty {
            walk = true;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
    }
    // 0x2b7570 .. 0x2b7590: walk the remaining slots, four at a time.
    if walk {
        c.cycles += 2;
        c.g[A0][0] = 0;
        loop {
            c.cycles += 3;
            c.g[V0][0] = oc1052_at(c, V1, 0).lw(c);
            let used = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
            if used {
                if event_due(c) && deliver_event(c, 0x002b_7568) {
                    return Ok(());
                }
                c.cycles += 2;
                c.g[V0][0] = 0;
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A0][0] as i64) < 3) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
            if more {
                if event_due(c) && deliver_event(c, 0x002b_7578) {
                    return Ok(());
                }
                continue;
            }
            c.cycles += 1;
            c.g[V0][0] = 1;
            break;
        }
    }
    // 0x2b7594: the cell is empty when V0 is 1: free its list.
    c.cycles += 2;
    let cell_empty = c.g[V0][0] != 0;
    c.g[A0][0] = c.g[S2][0];
    if cell_empty {
        c.cycles += 4;
        c.g[A0][0] = oc1052_at(c, S0, 0x10).lw(c);
        call_ptr(c, 0x0031_7e50, 0x002b_75a4)?;
        oc1052_at(c, S0, 0x10).sw(c, 0);
        c.g[A0][0] = c.g[S2][0];
    }
    // 0x2b75ac: unlink the emitter from the overlap list.
    c.cycles += 2;
    c.g[A1][0] = c.g[S1][0];
    WS_REMOVE_EMITTER.call(c, 0x002b_75b4)?;
    unlink_emitter_exit_full(c, f)
}

/// 0x2b75b4: restore `s0`, then the common epilogue.
fn unlink_emitter_exit_full(c: &mut Cpu, f: Frame) -> Flow {
    c.cycles += 1;
    f.restore(c, S0, 0x80);
    unlink_emitter_exit(c, f)
}

/// 0x2b75b8: restore `s1`, `s2` and `ra`, pop the frame.
fn unlink_emitter_exit(c: &mut Cpu, f: Frame) -> Flow {
    c.cycles += 5;
    f.restore(c, S1, 0x70);
    f.restore(c, S2, 0x60);
    f.restore_ra(c, 0x50);
    f.close(c);
    Ok(())
}

/// `WorldPainterQuery` destructor (0x2c06c0): frees the object's first field,
/// unlinks the object from the painter-query list at `gp+0x790` (or moves the
/// list head), and frees the object itself when `a1 & 1` is set.
pub fn world_painter_query_dtor(c: &mut Cpu) {
    guest(c, world_painter_query_dtor_body);
}

fn world_painter_query_dtor_body(c: &mut Cpu) -> Flow {
    c.cycles += 14;
    let f = Frame::open(c, 0x30);
    c.g[V0][0] = 0x48_0000;
    f.save(c, S0, 0x20);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3e00));
    f.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    f.save_ra(c, 0);
    c.g[S1][0] = c.g[A1][0];
    let vtable = c.g[V0][0];
    oc1052_at(c, S0, 4).sw(c, vtable);
    c.g[A0][0] = oc1052_at(c, S0, 0).lw(c);
    call_ptr(c, 0x0031_7e50, 0x002c_06ec)?;
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x790)).lw(c);
    if c.g[V1][0] == c.g[S0][0] {
        // The object is the list head: the head becomes its successor.
        c.cycles += 5;
        c.g[V0][0] = oc1052_at(c, S0, 0x14).lw(c);
        c.g[A1][0] = c.g[S1][0] & 1;
        c.g[A0][0] = 0x48_0000;
        let successor = c.g[V0][0];
        Addr((c.g[GP][0] as u32).wrapping_add(0x790)).sw(c, successor);
        c.cycles += 1;
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5d30));
    } else {
        c.g[V0][0] = oc1052_at(c, V1, 0x14).lw(c);
        // 0x2c073c: walk the list until the object's predecessor is found.
        c.cycles += 3;
        c.g[A1][0] = c.g[S1][0] & 1;
        let at_head_next = c.g[V0][0] == c.g[S0][0];
        c.g[A0][0] = 0x48_0000;
        if !at_head_next {
            c.cycles += 2;
            c.g[V1][0] = oc1052_at(c, V1, 0x14).lw(c);
            loop {
                c.cycles += 7;
                c.g[V0][0] = oc1052_at(c, V1, 0x14).lw(c);
                if c.g[V0][0] == c.g[S0][0] {
                    break;
                }
                c.g[V1][0] = oc1052_at(c, V1, 0x14).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x002c_0720);
                }
            }
        }
        c.cycles += 2;
        let found = c.g[V1][0] != 0;
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5d30));
        if found {
            c.cycles += 2;
            c.g[V0][0] = oc1052_at(c, S0, 0x14).lw(c);
            let successor = c.g[V0][0];
            oc1052_at(c, V1, 0x14).sw(c, successor);
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5d30));
        }
    }
    // 0x2c0760: store the vtable of the base class; free the object if asked.
    c.cycles += 2;
    let free_self = c.g[A1][0] != 0;
    let base_vtable = c.g[V0][0];
    oc1052_at(c, S0, 4).sw(c, base_vtable);
    if free_self {
        c.cycles += 2;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0031_7e50, 0x002c_0760)?;
    }
    c.cycles += 5;
    f.restore(c, S0, 0x20);
    f.restore(c, S1, 0x10);
    f.restore_ra(c, 0);
    f.close(c);
    Ok(())
}

/// Global constructor of the painter-query module (0x2c1688): the static init
/// `WorldPainterQuery_StaticInitDestroy(1, 0xffff)`.
pub fn world_painter_query_global_ctors(c: &mut Cpu) {
    guest(c, world_painter_query_global_ctors_body);
}

fn world_painter_query_global_ctors_body(c: &mut Cpu) -> Flow {
    c.cycles += 8;
    let f = Frame::open(c, 0x10);
    c.g[A0][0] = 1;
    f.save_ra(c, 0);
    c.g[A1][0] = 0xffff;
    WPQ_STATIC_INIT_DESTROY.call(c, 0x002c_169c)?;
    f.restore_ra(c, 0);
    f.close(c);
    Ok(())
}

/// `QuadTree_AllocNode(tree)` (0x2c17d0): hands out the next free node slot
/// of the tree. When the free list is empty it grows the node array (a fresh
/// allocation, the old contents copied over, then the old array freed) and
/// links the new nodes into the free list. Returns the node pointer.
pub fn quad_tree_alloc_node(c: &mut Cpu) {
    guest(c, quad_tree_alloc_node_body);
}

fn quad_tree_alloc_node_body(c: &mut Cpu) -> Flow {
    c.cycles += 9;
    let f = Frame::open(c, 0x30);
    f.save(c, S0, 0x20);
    f.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    f.save_ra(c, 0);
    c.g[V1][0] = oc1052_at(c, S0, 0x24).lw(c);
    c.g[V0][0] = oc1052_at(c, S0, 0x20).lw(c);
    if c.g[V1][0] != c.g[V0][0] {
        c.g[V0][0] = oc1052_at(c, S0, 0x24).lw(c);
    } else {
        // 0x2c1830: allocate the grown array (size = count*8 + 0x400 bytes, 8-byte units).
        c.cycles += 11;
        c.g[A0][0] = oc1052_at(c, S0, 0x10).lw(c);
        c.g[A1][0] = sx(0x48_0000u32.wrapping_add(0x5f98));
        c.g[A2][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x400));
        c.g[A3][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
        call_ptr(c, 0x0031_7e30, 0x002c_1814)?;
        c.g[A1][0] = oc1052_at(c, S0, 0x20).lw(c);
        let no_old = c.g[A1][0] == 0;
        c.g[S1][0] = c.g[V0][0];
        if !no_old {
            // 0x2c1830 (copy the old array over).
            c.cycles += 4;
            c.g[A2][0] = oc1052_at(c, S0, 0x10).lw(c);
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32) << 3);
            call_ptr(c, 0x0041_605c, 0x002c_1830)?;
        }
        // 0x2c186c: fill the new slots of the free list.
        c.cycles += 5;
        c.g[V1][0] = oc1052_at(c, S0, 0x10).lw(c);
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3ff));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
        let grow = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[V1][0] as u32) << 3);
        if grow {
            c.cycles += 3;
            c.g[A1][0] = c.g[A0][0];
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
            loop {
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                let link = c.g[V0][0];
                Addr(c.g[A0][0] as u32).sh(c, link);
                c.g[V1][0] = c.g[V0][0];
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(8));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x002c_1850);
                    }
                    continue;
                }
                break;
            }
        }
        // 0x2c189c: publish the new array and release the old one.
        c.cycles += 10;
        c.g[V0][0] = oc1052_at(c, S0, 0x10).lw(c);
        c.g[A1][0] = oc1052_at(c, S0, 0x20).lw(c);
        c.g[V1][0] = sx((c.g[V0][0] as u32) << 3);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x400));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S1][0] as u32));
        Addr((c.g[V1][0] as u32).wrapping_add(0x1ff8)).sh(c, 0);
        let first_free = c.g[A0][0];
        oc1052_at(c, S0, 0x24).sw(c, first_free);
        let old = c.g[A1][0];
        let grown_count = c.g[V0][0];
        oc1052_at(c, S0, 0x10).sw(c, grown_count);
        if old != 0 {
            c.cycles += 2;
            c.g[A0][0] = c.g[A1][0];
            call_ptr(c, 0x0031_7e98, 0x002c_189c)?;
        }
        c.cycles += 2;
        let s1 = c.g[S1][0];
        oc1052_at(c, S0, 0x20).sw(c, s1);
        c.g[V0][0] = oc1052_at(c, S0, 0x24).lw(c);
    }
    // 0x2c18a4 (epilogue of the allocation): hand out the node at the head.
    c.cycles += 16;
    c.g[A1][0] = oc1052_at(c, S0, 0x20).lw(c);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lhu(c);
    c.g[A0][0] = oc1052_at(c, S0, 0xc).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A1][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
    let used = c.g[A0][0];
    oc1052_at(c, S0, 0xc).sw(c, used);
    c.g[V0][0] &= 0xffff;
    let next_free = c.g[A1][0];
    oc1052_at(c, S0, 0x24).sw(c, next_free);
    f.restore(c, S0, 0x20);
    f.restore(c, S1, 0x10);
    f.restore_ra(c, 0);
    f.close(c);
    Ok(())
}

/// `QuadTree_Insert(node, x, y, ...)` (0x2c1950): descends the quad tree from
/// `node` towards the point `(x, y)` (the node's bounds are the three floats
/// at +0, +4, +8), and inserts into the child cell that holds it. Leaves
/// with a full cell are split through `QuadTree_InsertRecursive`.
pub fn quad_tree_insert(c: &mut Cpu) {
    guest(c, quad_tree_insert_body);
}

fn quad_tree_insert_body(c: &mut Cpu) -> Flow {
    c.cycles += 20;
    let f = Frame::open(c, 0x10);
    c.g[V1][0] = c.g[A0][0];
    f.save_ra(c, 0);
    c.g[T2][0] = c.g[A2][0];
    c.g[A0][0] = 0x7fff;
    let node = c.g[V1][0] as u32;
    c.lwc1(2, node.wrapping_add(4));
    c.lwc1(1, node.wrapping_add(8));
    c.sub_s(12, 12, 2);
    c.lwc1(0, node);
    c.sub_s(13, 13, 1);
    c.mul_s(14, 14, 0);
    c.mul_s(12, 12, 0);
    c.mul_s(13, 13, 0);
    c.cvt_w_s(0, 12);
    c.mfc1(6, 0);
    c.cvt_w_s(0, 13);
    c.mfc1(7, 0);
    c.g[V0][0] = (c.g[A0][0] < c.g[A2][0]) as u64;
    let beyond_limit = c.g[V0][0] != 0;
    c.g[T1][0] = c.g[A1][0];
    if !beyond_limit {
        c.cycles += 3;
        c.g[V0][0] = (c.g[A0][0] < c.g[A3][0]) as u64;
        let leaf_done = c.g[V0][0] != 0;
        f.restore_ra(c, 0);
        if leaf_done {
            return quad_tree_insert_end(c, f);
        }
        // 0x2c19d4: the split point (a 16-bit quotient of the halved limits).
        c.cycles += 10;
        c.cvt_w_s(0, 14);
        c.mfc1(2, 0);
        c.g[T0][0] = 0x8000;
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 1);
        c.g[A3][0] = sx((c.g[A3][0] as u32) << 1);
        c.g[A0][0] = c.g[V1][0];
        c.g[A2][0] = c.g[A2][0] & 0xfffe;
        c.div(0, T0, V0);
        c.cycles += 7;
        c.g[A1][0] = Addr(node.wrapping_add(0x14)).lhu(c);
        c.g[A3][0] &= 0xfffe;
        c.g[V0][0] = oc1052_at(c, V1, 0x20).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 3);
        c.g[T0][0] = c.lo[0];
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        QT_INSERT_RECURSIVE.call(c, 0x002c_19f0)?;
    }
    // 0x2c19f0: the node is handled; leave through the epilogue.
    c.cycles += 1;
    f.restore_ra(c, 0);
    quad_tree_insert_end(c, f)
}

/// 0x2c19f4 epilogue of `QuadTree_Insert`.
fn quad_tree_insert_end(c: &mut Cpu, f: Frame) -> Flow {
    c.cycles += 2;
    f.close(c);
    Ok(())
}

/// `int QuadTree_CountNodes(node)` (0x2c2178): the number of nodes under
/// `node`, counting the node itself (1 for a leaf). Recurses through the four
/// child slots of a node whose first halfword is odd.
pub fn quad_tree_count_nodes(c: &mut Cpu) {
    guest(c, quad_tree_count_nodes_body);
}

fn quad_tree_count_nodes_body(c: &mut Cpu) -> Flow {
    c.cycles += 9;
    let f = Frame::open(c, 0x50);
    f.save(c, S2, 0x20);
    f.save(c, S3, 0x10);
    c.g[S2][0] = c.g[A0][0];
    f.save(c, S0, 0x40);
    c.g[S3][0] = 1;
    f.save(c, S1, 0x30);
    let null_node = c.g[A1][0] == 0;
    f.save_ra(c, 0);
    if !null_node {
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lhu(c);
        c.g[V0][0] &= 1;
        let leaf = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[A1][0];
        if !leaf {
            c.cycles += 1;
            c.g[S1][0] = 3;
            loop {
                c.cycles += 11;
                c.g[A1][0] = Addr(c.g[S0][0] as u32).lhu(c);
                c.g[A0][0] = c.g[S2][0];
                c.g[V0][0] = oc1052_at(c, S2, 0x20).lw(c);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(2));
                c.g[A1][0] = sx((c.g[A1][0] as u32) >> 1);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
                c.g[A1][0] = sx((c.g[A1][0] as u32) << 3);
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                QT_COUNT_NODES.call(c, 0x002c_21d4)?;
                let more_children = (c.g[S1][0] as i64) >= 0;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                if more_children {
                    if event_due(c) {
                        deliver_event_here(c, 0x002c_21b0);
                    }
                    continue;
                }
                break;
            }
        }
    }
    // 0x2c21dc: return the count.
    c.cycles += 8;
    c.g[V0][0] = c.g[S3][0];
    f.restore(c, S0, 0x40);
    f.restore(c, S1, 0x30);
    f.restore(c, S2, 0x20);
    f.restore(c, S3, 0x10);
    f.restore_ra(c, 0);
    f.close(c);
    Ok(())
}

/// `GameModule_UnloadStub` (0x2c3fa0): an empty unload hook; returns at once.
pub fn game_module_unload_stub(c: &mut Cpu) {
    c.cycles += 2;
    c.pc = c.g[RA][0] as u32;
}

/// Destructor of `cAngerFX` (0x2d4a70): copies the effect's two static tables
/// onto the stack when `a1 == 0` (the object is not freed from its own
/// destructor), then releases the object's two buffers as its flags say.
pub fn anger_fx_dtor(c: &mut Cpu) {
    guest(c, anger_fx_dtor_body);
}

fn anger_fx_dtor_body(c: &mut Cpu) -> Flow {
    c.cycles += 21;
    let f = Frame::open(c, 0x130);
    c.g[V0][0] = 0x49_0000;
    f.save(c, S0, 0x120);
    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7540));
    f.save(c, S1, 0x110);
    c.g[S0][0] = c.g[A0][0];
    f.save_ra(c, 0x100);
    c.g[S1][0] = c.g[A1][0];
    c.g[A0][0] = 0x46_0000;
    c.g[A3][0] = c.g[V0][0];
    c.g[V1][0] = oc1052_at(c, S0, 0).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6470));
    let table_a = c.g[A2][0];
    oc1052_at(c, V1, 0x6e8).sw(c, table_a);
    c.g[V1][0] = 0x49_0000;
    c.g[A1][0] = oc1052_at(c, S0, 0).lw(c);
    c.g[T2][0] = c.g[V1][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x74f8));
    let name = c.g[A0][0];
    oc1052_at(c, A1, 0x6d0).sw(c, name);
    c.g[V0][0] = oc1052_at(c, S0, 0).lw(c);
    let has_flags = c.g[S1][0] != 0;
    let table_b = c.g[V1][0];
    oc1052_at(c, V0, 0x6c0).sw(c, table_b);
    'copy: {
        if has_flags {
            break 'copy;
        }
        // 0x2d4ac8: copy the two tables (two words each) to the stack.
        c.cycles += 4;
        c.g[A1][0] = c.g[A2][0];
        c.g[V1][0] = c.g[SP][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
        c.g[T0][0] = sx(sp(c).wrapping_add(0x50));
        loop {
            c.cycles += 11;
            c.g[A0][0] = Addr(c.g[A1][0] as u32).ld(c);
            c.g[A2][0] = Addr((c.g[A1][0] as u32).wrapping_add(8)).ld(c);
            c.g[T1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x10)).ld(c);
            c.g[T3][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x18)).ld(c);
            let v = c.g[A0][0];
            Addr(c.g[V1][0] as u32).sd(c, v);
            let v = c.g[A2][0];
            Addr((c.g[V1][0] as u32).wrapping_add(8)).sd(c, v);
            let v = c.g[T1][0];
            Addr((c.g[V1][0] as u32).wrapping_add(0x10)).sd(c, v);
            let v = c.g[T3][0];
            Addr((c.g[V1][0] as u32).wrapping_add(0x18)).sd(c, v);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
            let more = c.g[A1][0] != c.g[V0][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x002d_4ad4);
                }
                continue;
            }
            // 0x2d4b00: the second table, from the static block to the stack.
            c.cycles += 14;
            c.g[V0][0] = oc1052_at(c, S0, 0).lw(c);
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7540));
            c.g[A2][0] = Addr(c.g[A1][0] as u32).ld(c);
            let v = c.g[A2][0];
            Addr(c.g[V1][0] as u32).sd(c, v);
            let v = c.g[SP][0];
            oc1052_at(c, V0, 0x6e8).sw(c, v);
            c.g[A2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x74f8));
            c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(8)).lhu(c);
            c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
            c.g[V0][0] = oc1052_at(c, S0, 0).lw(c);
            c.g[A1][0] = c.g[T0][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
            c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
            let v = c.g[V1][0];
            oc1052_at(c, SP, 8).sh(c, v);
            loop {
                c.cycles += 11;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).ld(c);
                c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(8)).ld(c);
                c.g[A0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x10)).ld(c);
                c.g[T3][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x18)).ld(c);
                let v = c.g[V0][0];
                Addr(c.g[A1][0] as u32).sd(c, v);
                let v = c.g[V1][0];
                Addr((c.g[A1][0] as u32).wrapping_add(8)).sd(c, v);
                let v = c.g[A0][0];
                Addr((c.g[A1][0] as u32).wrapping_add(0x10)).sd(c, v);
                let v = c.g[T3][0];
                Addr((c.g[A1][0] as u32).wrapping_add(0x18)).sd(c, v);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                let more = c.g[A2][0] != c.g[T1][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x002d_4b38);
                    }
                    continue;
                }
                break;
            }
            c.cycles += 10;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).ld(c);
            c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(8)).ld(c);
            let v = c.g[V0][0];
            Addr(c.g[A1][0] as u32).sd(c, v);
            let v = c.g[V1][0];
            Addr((c.g[A1][0] as u32).wrapping_add(8)).sd(c, v);
            c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x74f8));
            c.g[A0][0] = oc1052_at(c, S0, 0).lw(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(8)).lhu(c);
            let v = c.g[T0][0];
            oc1052_at(c, A0, 0x6c0).sw(c, v);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
            let v = c.g[V1][0];
            oc1052_at(c, SP, 0x58).sh(c, v);
            break 'copy;
        }
    }
    // 0x2d4ba4: the first owned buffer (+4) goes back to the allocator.
    c.cycles += 3;
    c.g[A0][0] = oc1052_at(c, S0, 4).lw(c);
    let no_buffer = c.g[A0][0] == 0;
    c.g[V0][0] = c.g[S1][0] & 2;
    if !no_buffer {
        c.cycles += 3;
        call_ptr(c, 0x0031_7e98, 0x002d_4ba0)?;
        c.g[V0][0] = c.g[S1][0] & 2;
    }
    // 0x2d4bbc: the second owned block, when flagged.
    c.cycles += 2;
    let no_block = c.g[V0][0] == 0;
    c.g[V0][0] = c.g[S1][0] & 1;
    if !no_block {
        c.cycles += 4;
        c.g[A0][0] = oc1052_at(c, S0, 0).lw(c);
        c.g[A1][0] = 0;
        call_ptr(c, 0x0011_b978, 0x002d_4bb8)?;
        c.g[V0][0] = c.g[S1][0] & 1;
    }
    // 0x2d4bd0: the object itself, when the low flag says so.
    c.cycles += 2;
    if c.g[V0][0] == 0 {
        f.restore(c, S0, 0x120);
    } else {
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0031_7e50, 0x002d_4bcc)?;
        f.restore(c, S0, 0x120);
    }
    c.cycles += 4;
    f.restore(c, S1, 0x110);
    f.restore_ra(c, 0x100);
    f.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate word of this module's fakes (far above the fixtures' data).
    const OC1052_CTRL: u32 = DATA + 0x0020_0000;
    const OC1052_MAGIC: u32 = 0x1052_c0de;

    /// Machine with scrambled registers and the fakes' gate on.
    fn oc1052_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        oc1052_hooks();
        let r = fakes::machine(c, seed, salt, 0);
        c.bus.write32(OC1052_CTRL, OC1052_MAGIC);
        r
    }

    /// A callee fake: logs the call and returns (gate on), or runs the callee it
    /// replaced (gate off, another test in this binary).
    macro_rules! oc1052_fake {
        ($fake:ident, $prev:ident, $target:expr, $gen:path) => {
            static $prev: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC1052_CTRL) != OC1052_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $target);
            }
        };
    }

    oc1052_fake!(
        oc1052_fake_002b6cf8,
        OC1052_PREV_002B6CF8,
        0x002b_6cf8,
        ee::s08::f_002b6cf8__generated
    );
    oc1052_fake!(
        oc1052_fake_002b77c0,
        OC1052_PREV_002B77C0,
        0x002b_77c0,
        ee::s08::f_002b77c0__generated
    );
    oc1052_fake!(
        oc1052_fake_0028b1c8,
        OC1052_PREV_0028B1C8,
        0x0028_b1c8,
        ee::s07::f_0028b1c8__generated
    );
    oc1052_fake!(
        oc1052_fake_00328f28,
        OC1052_PREV_00328F28,
        0x0032_8f28,
        ee::s10::f_00328f28__generated
    );
    oc1052_fake!(
        oc1052_fake_003284b8,
        OC1052_PREV_003284B8,
        0x0032_84b8,
        ee::s10::f_003284b8__generated
    );
    oc1052_fake!(
        oc1052_fake_00317e50,
        OC1052_PREV_00317E50,
        0x0031_7e50,
        ee::s10::f_00317e50__generated
    );
    oc1052_fake!(
        oc1052_fake_00317e98,
        OC1052_PREV_00317E98,
        0x0031_7e98,
        ee::s10::f_00317e98__generated
    );
    oc1052_fake!(
        oc1052_fake_00317e30,
        OC1052_PREV_00317E30,
        0x0031_7e30,
        ee::s10::f_00317e30__generated
    );
    oc1052_fake!(
        oc1052_fake_0011b978,
        OC1052_PREV_0011B978,
        0x0011_b978,
        ee::s00::f_0011b978__generated
    );
    oc1052_fake!(
        oc1052_fake_002c1a00,
        OC1052_PREV_002C1A00,
        0x002c_1a00,
        ee::s08::f_002c1a00__generated
    );
    oc1052_fake!(
        oc1052_fake_002c0b70,
        OC1052_PREV_002C0B70,
        0x002c_0b70,
        ee::s08::f_002c0b70__generated
    );

    /// Installs the fakes above (once per process; each is gated).
    fn oc1052_hooks() {
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B6CF8),
            oc1052_fake_002b6cf8,
            &OC1052_PREV_002B6CF8,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B77C0),
            oc1052_fake_002b77c0,
            &OC1052_PREV_002B77C0,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0028B1C8),
            oc1052_fake_0028b1c8,
            &OC1052_PREV_0028B1C8,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00328F28),
            oc1052_fake_00328f28,
            &OC1052_PREV_00328F28,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003284B8),
            oc1052_fake_003284b8,
            &OC1052_PREV_003284B8,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc1052_fake_00317e50,
            &OC1052_PREV_00317E50,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc1052_fake_00317e98,
            &OC1052_PREV_00317E98,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc1052_fake_00317e30,
            &OC1052_PREV_00317E30,
        );
        fakes::hook(
            addr_of_mut!(ee::s00::OVERRIDE_0011B978),
            oc1052_fake_0011b978,
            &OC1052_PREV_0011B978,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C1A00),
            oc1052_fake_002c1a00,
            &OC1052_PREV_002C1A00,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C0B70),
            oc1052_fake_002c0b70,
            &OC1052_PREV_002C0B70,
        );
    }

    #[test]
    fn world_sound_unlink_emitter_matches() {
        check(
            ee::s08::f_002b7410__generated,
            world_sound_unlink_emitter,
            300,
            &|c, seed| {
                oc1052_machine(c, seed, 0x1052_0001);
                c.pc = UNLINK_EMITTER_ENTRY;
                let emitter = DATA + 0x1000;
                c.g[A0][0] = sx(DATA + 0x2000);
                // Null emitter, a missing cell, a cell with no first entry, and the full path.
                c.g[A1][0] = if seed % 7 == 0 { 0 } else { sx(emitter) };
                c.bus
                    .write32(emitter + 0xc, if seed % 3 == 1 { 0 } else { DATA + 0x1100 });
                c.bus.write32(emitter + 0x10, DATA + 0x1200);
                c.bus
                    .write32(DATA + 0x1204, if seed % 3 == 2 { 0 } else { DATA + 0x1300 });
                c.bus.write32(DATA + 0x1300, seed);
                c.bus.write32(DATA + 0x1320, seed.wrapping_mul(3));
            },
        );
    }

    #[test]
    fn world_painter_query_dtor_matches() {
        check(
            ee::s08::f_002c06c0__generated,
            world_painter_query_dtor,
            300,
            &|c, seed| {
                oc1052_machine(c, seed, 0x1052_0002);
                let this = DATA + 0x2000;
                c.g[A0][0] = sx(this);
                // Bit 0 of a1 decides whether the object itself is freed.
                c.g[A1][0] = seed as u64;
                c.bus.write32(this, DATA + 0x2100);
                c.bus.write32(this + 0x14, DATA + 0x2200);
                // The list at gp+0x790: the object is the head, or one of 1..3 links deep.
                let head = match seed % 4 {
                    0 => this,
                    1 => DATA + 0x3000,
                    2 => DATA + 0x3000,
                    _ => DATA + 0x3000,
                };
                c.bus.write32(GAME_GP.wrapping_add(0x790), head);
                if seed % 4 == 1 {
                    c.bus.write32(DATA + 0x3014, this);
                } else if seed % 4 == 2 {
                    c.bus.write32(DATA + 0x3014, DATA + 0x3100);
                    c.bus.write32(DATA + 0x3114, this);
                } else if seed % 4 == 3 {
                    c.bus.write32(DATA + 0x3014, DATA + 0x3100);
                    c.bus.write32(DATA + 0x3114, DATA + 0x3200);
                    c.bus.write32(DATA + 0x3214, this);
                }
            },
        );
    }

    #[test]
    fn world_painter_query_global_ctors_matches() {
        check(
            ee::s08::f_002c1688__generated,
            world_painter_query_global_ctors,
            200,
            &|c, seed| {
                oc1052_machine(c, seed, 0x1052_0003);
            },
        );
    }

    #[test]
    fn quad_tree_alloc_node_matches() {
        check(
            ee::s08::f_002c17d0__generated,
            quad_tree_alloc_node,
            200,
            &|c, seed| {
                oc1052_machine(c, seed, 0x1052_0004);
                let tree = DATA + 0x2000;
                c.g[A0][0] = sx(tree);
                c.bus.write32(tree + 0x10, seed % 5);
                c.bus.write32(tree + 0xc, seed);
                // Free list present (the first branch), or empty with an old array or none.
                let (first, cap) = match seed % 3 {
                    0 => (DATA + 0x3000, DATA + 0x3008),
                    1 => (DATA + 0x3000, DATA + 0x3000),
                    _ => (DATA + 0x3000, 0),
                };
                c.bus.write32(tree + 0x24, first);
                c.bus.write32(tree + 0x20, cap);
                Addr(DATA + 0x3000).sh(c, 0x0102);
            },
        );
    }

    #[test]
    fn quad_tree_insert_matches() {
        check(
            ee::s08::f_002c1950__generated,
            quad_tree_insert,
            200,
            &|c, seed| {
                let mut r = oc1052_machine(c, seed, 0x1052_0005);
                let node = DATA + 0x2000;
                c.g[A0][0] = sx(node);
                c.g[A1][0] = sx(DATA + 0x4000);
                // Limits chosen so the three branches of the descent are taken.
                let (a2, a3) = match seed % 3 {
                    0 => (0x8000u64, 0x7000u64),
                    1 => (0x7000, 0x8000),
                    _ => (0x7000, 0x7000),
                };
                c.g[A2][0] = a2;
                c.g[A3][0] = a3;
                c.bus.write32(node, r.f32(1000.0));
                c.bus.write32(node + 4, r.f32(1000.0));
                c.bus.write32(node + 8, r.f32(1000.0));
                c.bus.write32(node + 0x20, DATA + 0x5000);
                Addr(node + 0x14).sh(c, seed as u64 % 0x100);
                c.g[A3][0] = a3;
            },
        );
    }

    #[test]
    fn quad_tree_count_nodes_matches() {
        check(
            ee::s08::f_002c2178__generated,
            quad_tree_count_nodes,
            200,
            &|c, seed| {
                oc1052_machine(c, seed, 0x1052_0006);
                let tree = DATA + 0x1000;
                let base = DATA + 0x20000;
                c.g[A0][0] = sx(tree);
                c.bus.write32(tree + 0x20, base);
                // Node at DATA+0x30000: a leaf (even first halfword), or odd with
                // children whose halfwords (in the zeroed pool) make them leaves.
                let node = DATA + 0x30000;
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { sx(node) };
                let first = if seed % 4 == 1 { 0u16 } else { 1 };
                Addr(node).sh(c, first as u64);
                for k in 1..4u32 {
                    Addr(node + 2 * k).sh(c, ((seed % 7 + k) * 2 + 1) as u64);
                }
            },
        );
    }

    #[test]
    fn game_module_unload_stub_matches() {
        check(
            ee::s08::f_002c3fa0__generated,
            game_module_unload_stub,
            50,
            &|c, seed| {
                oc1052_machine(c, seed, 0x1052_0007);
            },
        );
    }

    #[test]
    fn anger_fx_dtor_matches() {
        check(
            ee::s09::f_002d4a70__generated,
            anger_fx_dtor,
            300,
            &|c, seed| {
                oc1052_machine(c, seed, 0x1052_0008);
                let fx = DATA + 0x2000;
                c.g[A0][0] = sx(fx);
                // a1 == 0 copies the tables; otherwise the flags pick the frees.
                c.g[A1][0] = (seed % 8) as u64;
                c.bus.write32(fx, DATA + 0x2100);
                c.bus
                    .write32(fx + 4, if seed % 2 == 1 { DATA + 0x4000 } else { 0 });
            },
        );
    }
}
