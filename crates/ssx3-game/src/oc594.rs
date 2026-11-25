//! Staging module oc594 (wave wave233): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc594`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x003a_89c8,
        cworld_cache_evict_lowest_priority_hull,
        "cWorldCache_EvictLowestPriorityHull",
    ),
    (
        0x003a_9180,
        cworld_cache_streamer_dtor_stub,
        "cWorldCache_StreamerDtorStub",
    ),
    (
        0x003a_9958,
        cworld_cache_streamer_release_section,
        "cWorldCache_StreamerReleaseSection",
    ),
    (
        0x003a_9ac0,
        cworld_cache_streamer_are_box_hulls_resident,
        "cWorldCache_StreamerAreBoxHullsResident",
    ),
    (
        0x003a_9bb8,
        cworld_cache_streamer_check_node_box,
        "cWorldCache_StreamerCheckNodeBox",
    ),
    (
        0x003a_ace0,
        cps2_chunk_resolve_calc_percent,
        "cPS2ChunkResolve_CalcPercent",
    ),
    (
        0x003a_ae60,
        cps2_chunk_resolve_dtor_stub,
        "cPS2ChunkResolve_DtorStub",
    ),
    (0x003a_c8f0, cwscript_cache_dtor, "cWScriptCache_dtor"),
    (0x003a_c980, cwscript_cache_update, "cWScriptCache_Update"),
    (0x003a_c9e0, cwscript_cache_load, "cWScriptCache_Load"),
    (
        0x003a_cad0,
        twscript_cache_poll_load,
        "tWScriptCache_PollLoad",
    ),
    (0x003a_cbb8, twscript_cache_load, "tWScriptCache_Load"),
    (0x003a_d250, world_return_self, "World_ReturnSelf"),
    (0x003a_dda0, world_global_ctors, "World_GlobalCtors"),
];

/// `cWorldCache_FreeHullPage(self, page)` (0x3a8528).
const EVICT_FREE_HULL_PAGE: Callee = Callee {
    addr: 0x003a_8528,
    f: ee::s13::cWorldCache_FreeHullPage,
};
/// `cWorldCache_IsHullResident(cache, id)` (0x3a8618).
const IS_HULL_RESIDENT: Callee = Callee {
    addr: 0x003a_8618,
    f: ee::s13::cWorldCache_IsHullResident,
};
/// `cWorldCache_StreamerCheckNodeBox(view, node, box, ok)` (0x3a9bb8).
const CHECK_NODE_BOX: Callee = Callee {
    addr: 0x003a_9bb8,
    f: ee::s13::cWorldCache_StreamerCheckNodeBox,
};
/// `EAC_GetUnpackedSize(...)` (0x3b4818).
const EAC_GET_UNPACKED_SIZE: Callee = Callee {
    addr: 0x003b_4818,
    f: ee::s13::EAC_GetUnpackedSize,
};
/// `EAC_Unpack(...)` (0x3b47f8).
const EAC_UNPACK: Callee = Callee {
    addr: 0x003b_47f8,
    f: ee::s13::EAC_Unpack,
};
/// `tWScriptCacheEntry_dtor` (0x3aca70).
const WSCRIPT_ENTRY_DTOR: Callee = Callee {
    addr: 0x003a_ca70,
    f: ee::s13::tWScriptCache_dtor,
};
/// `cWScriptCacheEntry_update` (0x3acad0).
const WSCRIPT_POLL_LOAD: Callee = Callee {
    addr: 0x003a_cad0,
    f: ee::s13::tWScriptCache_PollLoad,
};
/// `cWScriptCacheEntry_load` (0x3acbb8).
const WSCRIPT_ENTRY_LOAD: Callee = Callee {
    addr: 0x003a_cbb8,
    f: ee::s13::tWScriptCache_Load,
};
/// `World_StaticInitDestroy(on, prio)` (0x3ad290).
const WORLD_STATIC_INIT_DESTROY: Callee = Callee {
    addr: 0x003a_d290,
    f: ee::s13::World_StaticInitDestroy,
};

/// `void cWorldCache_EvictLowestPriorityHull(cWorldCache* self, int)` (0x3a89c8).
///
/// Scans the `count` (at +0x3ec) 0x18-byte slots from +0x3f0. A slot is a
/// candidate when its state word is 3 or 4 and its lock word (+4) is zero;
/// the one with the lowest float priority (+0xc, below 1.0) wins. The winner
/// is marked state 4, its page freed through `cWorldCache_FreeHullPage`, and
/// its state word cleared.
pub fn cworld_cache_evict_lowest_priority_hull(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = u64::MAX;
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 1);
        c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0x3ec));
        let empty = c.g[V0][0] == 0;
        c.g[A3][0] = 0;
        if !empty {
            c.cycles += 3;
            c.g[T0][0] = c.g[V0][0];
            c.g[A2][0] = sx(r32(c, A0).wrapping_add(0x3fc));
            loop {
                c.cycles += 3;
                c.g[V1][0] = c.lw(r32(c, A2).wrapping_sub(0xc));
                'slot: {
                    if c.g[V1][0] == 0 {
                        c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                        break 'slot;
                    }
                    'cand: {
                        c.cycles += 2;
                        let nonpositive = (c.g[V1][0] as i64) <= 0;
                        c.g[V0][0] = ((c.g[V1][0] as i64) < 3) as u64;
                        if nonpositive {
                            break 'cand;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] != 0 {
                            c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                            break 'slot;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[V1][0] as i64) < 5) as u64;
                        if c.g[V0][0] == 0 {
                            c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                            break 'slot;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(r32(c, A2).wrapping_sub(0x8));
                        if c.g[V0][0] != 0 {
                            c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                            break 'slot;
                        }
                        c.cycles += 5;
                        c.lwc1(0, r32(c, A2));
                        c.c_lt_s(0, 1);
                        if !c.fcc() {
                            c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                            break 'slot;
                        }
                        c.cycles += 2;
                        c.mov_s(1, 0);
                        c.g[A1][0] = c.g[A3][0];
                    }
                    c.cycles += 1;
                    c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                }
                c.cycles += 3;
                c.g[V0][0] = (c.g[A3][0] < c.g[T0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x18));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_89f8);
                }
            }
        }
        c.cycles += 2;
        let none = (c.g[A1][0] as i64) < 0;
        c.g[V0][0] = 0x18;
        if !none {
            c.cycles += 7;
            c.g[S0][0] = sx(r32(c, A0).wrapping_add(0x3f0));
            c.mult(0, 2, 5, 2);
            c.g[V1][0] = 4;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
            c.g[RA][0] = sx(0x003a_8a78);
            c.sw(r32(c, S0), c.g[V1][0]);
            EVICT_FREE_HULL_PAGE.call(c, 0x003a_8a78)?;
            c.sw(r32(c, S0), 0);
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cWorldView_purge(cWorldView*)` (0x3a9180): an empty stub.
pub fn cworld_cache_streamer_dtor_stub(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `int cWorldView_unloadSection(cWorldView* view, int section)` (0x3a9958).
///
/// Looks up the section's state word in the table at +0x14 (8-byte entries).
/// A state below 7 selects a case through the function table at 0x494f30:
/// state 0 returns 0 at once, state 1 and 3 store a state word of 3 or 0 at
/// the entry, and any other target leaves through `jump_out`.
pub fn cworld_cache_streamer_release_section(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0x4));
    let t = c.g[V0][0] == 0;
    'end: {
        if t {
            break 'end;
        }
        c.cycles += 4;
        c.g[V0][0] = c.lw(r32(c, V0).wrapping_add(0x8));
        c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
        let t = c.g[V0][0] == 0;
        c.g[V0][0] = sx(r32(c, A1) << 3);
        if t {
            break 'end;
        }
        c.cycles += 8;
        c.g[V1][0] = sx(r32(c, A0).wrapping_add(0x14));
        c.g[A1][0] = c.g[V1][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        c.g[V1][0] = c.lw(r32(c, V1));
        c.g[V0][0] = (c.g[V1][0] < 0x7) as u64;
        let t = c.g[V0][0] == 0;
        c.g[V0][0] = 0x490000;
        if t {
            break 'end;
        }
        'case: {
            c.cycles += 6;
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x4f30));
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A0][0] = c.lw(r32(c, V1));
            let target = r32(c, A0);
            match target {
                0x003a_99ac => {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    ret(c);
                    return;
                }
                0x003a_99b4 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx(r32(c, A1).wrapping_add(r32(c, A2)));
                    c.g[V0][0] = 3;
                    c.sw(r32(c, V1), c.g[V0][0]);
                    break 'end;
                }
                0x003a_99c4 => break 'case,
                0x003a_99cc => break 'end,
                _ => {
                    c.jump_out(target, 0x003a_9958, 0x003a_99d4);
                    return;
                }
            }
        }
        c.cycles += 2;
        c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, A2)));
        c.sw(r32(c, V0), 0);
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    ret(c);
}

/// `bool cWorldView_isBoxValid(cWorldView* view, box)` (0x3a9ac0): the box
/// is valid when the global hull of the current section is resident, and
/// every section in state 6 passes the recursive node-box test. Returns the
/// `ok` flag (0 if the global hull is not resident).
pub fn cworld_cache_streamer_are_box_hulls_resident(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x80);
        c.g[V1][0] = 0x44;
        frame.save(c, 16, 0x70);
        frame.save(c, 20, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x60);
        frame.save(c, 18, 0x50);
        frame.save(c, 19, 0x40);
        frame.save(c, 21, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A2][0] = c.lw(r32(c, S0).wrapping_add(0x8));
        c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(0x10));
        c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x18));
        c.mult(0, 2, 2, 3);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
        c.g[V1][0] = c.lhu(r32(c, V0));
        let no_hull = c.g[V1][0] == 0;
        c.g[S4][0] = c.g[A1][0];
        'end: {
            if no_hull {
                c.cycles += 1;
                c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x4));
            } else {
                c.cycles += 5;
                c.g[A1][0] = c.lw(r32(c, A2).wrapping_add(0x18));
                c.g[A0][0] = c.lw(r32(c, S0));
                IS_HULL_RESIDENT.call(c, 0x003a_9b14)?;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x4));
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    break 'end;
                }
            }
            // Loop over the sections (`'b_3a9b8c`).
            c.cycles += 5;
            c.g[V1][0] = 0x1;
            c.sw(sp(c), c.g[V1][0]);
            c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x8));
            let no_sections = c.g[V1][0] == 0;
            c.g[S2][0] = 0;
            if !no_sections {
                c.cycles += 3;
                c.g[S5][0] = 0x6;
                c.g[S3][0] = 0;
                c.g[S1][0] = sx(r32(c, S0).wrapping_add(0x14));
                loop {
                    'item: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(r32(c, S1));
                        if c.g[V0][0] != c.g[S5][0] {
                            c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x4));
                            break 'item;
                        }
                        c.cycles += 8;
                        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x8));
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A2][0] = c.g[S4][0];
                        c.g[A3][0] = c.g[SP][0];
                        c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
                        c.g[A1][0] = c.lw(r32(c, V0).wrapping_add(0x1c));
                        CHECK_NODE_BOX.call(c, 0x003a_9b70)?;
                        c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x4));
                    }
                    c.cycles += 6;
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(0x1));
                    c.g[S3][0] = sx(r32(c, S3).wrapping_add(0x58));
                    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x8));
                    c.g[V0][0] = (c.g[S2][0] < c.g[V0][0]) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x8));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003a_9b48);
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw(sp(c));
        }
        c.cycles += 9;
        frame.restore(c, 16, 0x70);
        frame.restore(c, 17, 0x60);
        frame.restore(c, 18, 0x50);
        frame.restore(c, 19, 0x40);
        frame.restore(c, 20, 0x30);
        frame.restore(c, 21, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// Node-box overlap of the generated code (`'b_3a9c68`): the six axis tests
/// `box.min <= node.max` and `node.min <= box.max`, stopping at the first
/// failure. Leaves the last comparison's flag in fcc.
fn node_box_overlap(c: &mut Cpu, node: u32, bx: u32) -> bool {
    c.cycles += 6;
    c.lwc1(1, bx);
    c.lwc1(0, node.wrapping_add(0x10));
    c.c_le_s(1, 0);
    if !c.fcc() {
        return false;
    }
    c.cycles += 5;
    c.lwc1(1, bx.wrapping_add(0x10));
    c.lwc1(0, node);
    c.c_le_s(0, 1);
    if !c.fcc() {
        return false;
    }
    c.cycles += 5;
    c.lwc1(1, bx.wrapping_add(0x4));
    c.lwc1(0, node.wrapping_add(0x14));
    c.c_le_s(1, 0);
    if !c.fcc() {
        return false;
    }
    c.cycles += 5;
    c.lwc1(1, bx.wrapping_add(0x14));
    c.lwc1(0, node.wrapping_add(0x4));
    c.c_le_s(0, 1);
    if !c.fcc() {
        return false;
    }
    c.cycles += 5;
    c.lwc1(1, bx.wrapping_add(0x8));
    c.lwc1(0, node.wrapping_add(0x18));
    c.c_le_s(1, 0);
    if !c.fcc() {
        return false;
    }
    c.cycles += 6;
    c.lwc1(1, bx.wrapping_add(0x18));
    c.lwc1(0, node.wrapping_add(0x8));
    c.c_le_s(0, 1);
    c.fcc()
}

/// Split-plane test of the generated VU0 code (`'b_3a9cf0`): dot the plane
/// at `plane` (vf3) with the box corner at `point` (vf5) and return whether
/// the result is below zero (fcc of `f1 < f0`, with f0 = 0).
fn node_split_dot_negative(c: &mut Cpu, plane: u32, point: u32) -> bool {
    c.cycles += 14;
    c.mtc1(0, 0);
    c.lqc2(3, plane);
    c.lqc2(5, point);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(2, 4);
    c.mtc1(2, 1);
    c.c_lt_s(1, 0);
    c.fcc()
}

/// `void cWorldView_recursiveIsValid(cWorldView* view, node, box, bool* ok)`
/// (0x3a9bb8): clear `*ok` when the node's page is not resident, and descend
/// into the children whose split plane the box reaches.
pub fn cworld_cache_streamer_check_node_box(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, 18, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, 19, 0x10);
        c.g[S2][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(r32(c, S2));
        let ok_clear = c.g[V0][0] == 0;
        c.g[S3][0] = c.g[A0][0];
        // `plus_one`: whether the exit runs the `cycles += 1` tail (the
        // generated `'b_3a9d40` block); the early exits skip it.
        let plus_one = 'body: {
            if ok_clear {
                break 'body true;
            }
            let node = r32(c, S0);
            let bx = r32(c, S1);
            let overlap = node_box_overlap(c, node, bx);
            c.g[V0][0] = overlap as u64;
            c.cycles += 2;
            if !overlap {
                break 'body false;
            }
            // Hull of the node: resident, or clear `*ok`.
            c.cycles += 3;
            c.g[V0][0] = c.lw(node.wrapping_add(0x58));
            if c.g[V0][0] == 0 {
                c.g[A1][0] = c.lw(node.wrapping_add(0x50));
            } else {
                c.cycles += 5;
                c.g[A1][0] = c.lhu(r32(c, V0).wrapping_add(0x2));
                c.g[A0][0] = c.lw(r32(c, S3));
                IS_HULL_RESIDENT.call(c, 0x003a_9c88)?;
                if c.g[V0][0] != 0 {
                    c.g[A1][0] = c.lw(node.wrapping_add(0x50));
                } else {
                    c.cycles += 2;
                    c.sw(r32(c, S2), 0);
                    break 'body true;
                }
            }
            // In child (`'b_3a9cf4`).
            c.cycles += 2;
            if c.g[A1][0] == 0 {
                c.g[A1][0] = c.lw(node.wrapping_add(0x54));
            } else {
                let below = node_split_dot_negative(c, node.wrapping_add(0x20), bx);
                c.g[A0][0] = c.g[S3][0];
                if below {
                    c.cycles += 6;
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A3][0] = c.g[S2][0];
                    CHECK_NODE_BOX.call(c, 0x003a_9ce4)?;
                    c.g[V0][0] = c.lw(r32(c, S2));
                    if c.g[V0][0] == 0 {
                        // `*ok` cleared below: exit without the tail.
                        break 'body false;
                    }
                }
                c.cycles += 1;
                c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(0x54));
            }
            // Out child.
            c.cycles += 2;
            if c.g[A1][0] == 0 {
                break 'body false;
            }
            let node = r32(c, S0);
            let below = node_split_dot_negative(c, node.wrapping_add(0x30), bx.wrapping_add(0x10));
            c.g[A0][0] = c.g[S3][0];
            if !below {
                break 'body true;
            }
            c.cycles += 3;
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = c.g[S2][0];
            CHECK_NODE_BOX.call(c, 0x003a_9d40)?;
            true
        };
        if plus_one {
            c.cycles += 1;
        }
        frame.restore(c, 16, 0x40);
        c.cycles += 6;
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float cChunkResolve_calcMaxLightDistance(self, light, divisor)`
/// (0x3aace0): the light's max distance from `a * b / divisor` shaped by the
/// falloff kind byte at +0x64: 1 linear, 2 square root, 3 `exp(log(r) / 3)`,
/// otherwise 1e7. The linear, root and exponential forms are scaled by 100.
pub fn cps2_chunk_resolve_calc_percent(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 2;
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x10);
        let light = r32(c, A1);
        c.lwc1(0, light.wrapping_add(0x14));
        c.lwc1(1, light.wrapping_add(0x18));
        c.g[V1][0] = c.lb(light.wrapping_add(0x64));
        c.mul_s(0, 0, 1);
        c.div_s(12, 0, 12);
        let kind = c.g[V1][0];
        let is_root = kind == c.g[V0][0];
        c.g[V0][0] = ((kind as i64) < 3) as u64;
        enum Arm {
            Linear,
            Root,
            LogExp,
            Other,
        }
        let arm = if is_root {
            Arm::Root
        } else {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[V0][0] = 3;
                if kind == 3 {
                    Arm::LogExp
                } else {
                    c.cycles += 2;
                    Arm::Other
                }
            } else {
                c.g[V0][0] = 1;
                c.cycles += 2;
                if kind == 1 {
                    Arm::Linear
                } else {
                    c.cycles += 2;
                    Arm::Other
                }
            }
        };
        match arm {
            Arm::Linear => {
                c.cycles += 4;
                c.g[AT][0] = 0x42c8_0000;
                c.mtc1(1, 0);
                c.mul_s(0, 12, 0);
            }
            Arm::Root => {
                c.cycles += 5;
                c.g[AT][0] = 0x42c8_0000;
                c.mtc1(1, 1);
                c.sqrt_s(0, 12);
                c.mul_s(0, 0, 1);
            }
            Arm::LogExp => {
                c.cycles += 9;
                c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x23b4));
                call_ptr(c, 0x0040_da10, 0x003a_ad68)?;
                c.mul_s(12, 0, 20);
                call_ptr(c, 0x0040_d758, 0x003a_ad70)?;
                c.g[AT][0] = 0x42c8_0000;
                c.mtc1(1, 1);
                c.mul_s(0, 0, 1);
            }
            Arm::Other => {
                c.cycles += 1;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x23b0));
            }
        }
        c.cycles += 4;
        frame.restore_fpr(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2ChunkResolve_purge(void*)` (0x3aae60): an empty stub.
pub fn cps2_chunk_resolve_dtor_stub(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cWScriptCache::~cWScriptCache(this, flags)` (0x3ac8f0): destroy the
/// entries from the last to the first (`tWScriptCache_dtor`), free the array
/// (`operator delete[]`, the cookie 0x10 below it), and free `this` when
/// bit 0 of `flags` is set.
pub fn cwscript_cache_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 17, 0x20);
        frame.save(c, 18, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 16, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw(r32(c, S1).wrapping_add(0x4));
        let no_entries = c.g[A1][0] == 0;
        c.g[V0][0] = 0xc;
        if !no_entries {
            'arr: {
                c.cycles += 5;
                c.g[V1][0] = c.lw(r32(c, A1).wrapping_sub(0x10));
                c.mult(0, 4, 3, 2);
                c.g[S0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
                if c.g[A1][0] == c.g[S0][0] {
                    c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x4));
                    break 'arr;
                }
                c.cycles += 1;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0xc));
                loop {
                    c.cycles += 6;
                    c.g[A1][0] = 0;
                    c.g[A0][0] = c.g[S0][0];
                    WSCRIPT_ENTRY_DTOR.call(c, 0x003a_c93c)?;
                    c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x4));
                    let more = c.g[V0][0] != c.g[S0][0];
                    c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0xc));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003a_c930);
                    }
                }
                c.cycles += 1;
                c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x4));
            }
            c.cycles += 2;
            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x10));
            call_ptr(c, 0x0031_7e98, 0x003a_c954)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S2][0] & 0x1;
        let no_free = c.g[V0][0] == 0;
        c.sw((c.g[GP][0] as u32).wrapping_add(0x1720), 0);
        if !no_free {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x003a_c968)?;
        }
        c.cycles += 6;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cWScriptCache_update(cWScriptCache* self)` (0x3ac980): poll every
/// entry (`cWScriptCacheEntry_update`), 0xc bytes apart.
pub fn cwscript_cache_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        frame.save(c, 18, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x8));
        let none = c.g[V0][0] == 0;
        c.g[S0][0] = 0;
        if !none {
            c.cycles += 1;
            c.g[S2][0] = 0;
            loop {
                c.cycles += 8;
                c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x4));
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x1));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S2)));
                WSCRIPT_POLL_LOAD.call(c, 0x003a_c9b8)?;
                c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x8));
                c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(0xc));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_c9a8);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cWScriptCache_load(cWScriptCache* self, int index, int fileNum)`
/// (0x3ac9e0): load entry `index` (0xc bytes each) from file `fileNum`.
pub fn cwscript_cache_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xc;
        frame.save_ra(c, 0);
        c.mult(0, 3, 5, 2);
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = c.lw(r32(c, A0).wrapping_add(0x4));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        WSCRIPT_ENTRY_LOAD.call(c, 0x003a_ca00)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cWScriptCacheEntry_update(entry* e)` (0x3acad0): an entry in state
/// 1 polls its resource (`0x3df980`); once it is ready it probes the data
/// (`0x3df9d8`), unpacks a new buffer when the size is positive and stores
/// the entry's state word (`sh`) as 1 or 2.
pub fn twscript_cache_poll_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 1;
        frame.save(c, 17, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 16, 0x30);
        c.g[S0][0] = c.lh(r32(c, S1));
        // The generated exits that skip the store reload s0 only on the
        // not-ready paths; the unpack-size exit leaves s0 as it is.
        let mut reload_s0 = true;
        'body: {
            if c.g[S0][0] != c.g[V0][0] {
                break 'body;
            }
            c.cycles += 4;
            c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x4));
            call_ptr(c, 0x003d_f980, 0x003a_cafc)?;
            if c.g[V0][0] != c.g[S0][0] {
                break 'body;
            }
            c.cycles += 9;
            c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x4));
            c.g[A1][0] = c.g[SP][0];
            c.sw(sp(c), 0);
            c.g[A2][0] = c.g[SP][0] | 0x4;
            c.sw(sp(c).wrapping_add(0x4), 0);
            call_ptr(c, 0x003d_f9d8, 0x003a_cb1c)?;
            c.g[A0][0] = c.lw(sp(c));
            c.g[V0][0] = c.lw(sp(c).wrapping_add(0x4));
            if c.g[A0][0] != 0 {
                c.cycles += 2;
                let nonpositive = (c.g[V0][0] as i64) <= 0;
                c.g[V0][0] = 0x2;
                if nonpositive {
                    // Leaves through the epilogue with s0 unreloaded.
                    reload_s0 = false;
                    break 'body;
                }
                'b94: {
                    'b6c: {
                        c.cycles += 4;
                        EAC_GET_UNPACKED_SIZE.call(c, 0x003a_cb38)?;
                        let no_size = (c.g[V0][0] as i64) <= 0;
                        c.g[A1][0] = 0x490000;
                        if no_size {
                            break 'b6c;
                        }
                        c.cycles += 11;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x5080));
                        c.g[A2][0] = 0x2000_0000;
                        c.g[A3][0] = 0;
                        call_ptr(c, 0x0031_7e30, 0x003a_cb54)?;
                        c.g[A0][0] = c.lw(sp(c));
                        c.g[A1][0] = c.g[V0][0];
                        c.sw(r32(c, S1).wrapping_add(0x8), c.g[V0][0]);
                        EAC_UNPACK.call(c, 0x003a_cb64)?;
                        break 'b94;
                    }
                    c.cycles += 10;
                    c.g[A0][0] = c.lw(sp(c).wrapping_add(0x4));
                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x5080));
                    c.g[A2][0] = 0x2000_0000;
                    c.g[A3][0] = 0;
                    call_ptr(c, 0x0031_7e30, 0x003a_cb80)?;
                    c.g[A2][0] = c.lw(sp(c).wrapping_add(0x4));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.lw(sp(c));
                    c.sw(r32(c, S1).wrapping_add(0x8), c.g[V0][0]);
                    call_ptr(c, 0x003e_6574, 0x003a_cb94)?;
                }
                c.cycles += 2;
                c.g[A0][0] = c.lw(sp(c));
                call_ptr(c, 0x0025_23a8, 0x003a_cb9c)?;
            }
            // Store the state word (`'b_3acb9c` exit).
            c.cycles += 2;
            c.sh(r32(c, S1), c.g[V0][0]);
        }
        if reload_s0 {
            frame.restore(c, 16, 0x30);
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void cWScriptCacheEntry_load(entry* e, int fileNum)` (0x3acbb8): the
/// entry's file load: format the name, allocate 0x100 bytes for the data and
/// set the entry's state (`sh`) to 1 when the file is non-empty, else 2.
pub fn twscript_cache_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x4a_0000;
        frame.save(c, 17, 0x30);
        c.g[A2][0] = c.g[A1][0];
        frame.save(c, 16, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.sh(r32(c, S1).wrapping_add(0x2), 0);
        c.g[A1][0] = sx(r32(c, V0).wrapping_add(0x47c0));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0031_86d0, 0x003a_cbe8)?;
        c.g[A2][0] = 0x4a_0000;
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x4818));
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0031_8630, 0x003a_cbfc)?;
        c.g[A0][0] = c.lw(sp(c));
        c.g[A1][0] = 0x100;
        call_ptr(c, 0x003d_f690, 0x003a_cc08)?;
        c.sw(r32(c, S1).wrapping_add(0x4), c.g[V0][0]);
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x003a_cc18)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x003a_cc24)?;
        c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x4));
        let non_empty = (c.g[V0][0] as i64) > 0;
        c.g[V0][0] = 1;
        if !non_empty {
            c.cycles += 1;
            c.g[V0][0] = 2;
        }
        c.cycles += 6;
        c.sh(r32(c, S1), c.g[V0][0]);
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void World_ReturnSelf(...)` (0x3ad250, `func_003AD250`): spills its
/// arguments to its frame and returns its first argument.
pub fn world_return_self(c: &mut Cpu) {
    c.cycles += 15;
    let frame = Frame::open(c, 0x80);
    c.g[V0][0] = c.g[A0][0];
    let base = sp(c);
    c.sd(base.wrapping_add(0x48), c.g[A1][0]);
    c.sd(base.wrapping_add(0x50), c.g[A2][0]);
    c.sd(base.wrapping_add(0x58), c.g[A3][0]);
    c.sd(base.wrapping_add(0x60), c.g[T0][0]);
    c.sd(base.wrapping_add(0x68), c.g[T1][0]);
    c.sd(base.wrapping_add(0x70), c.g[T2][0]);
    c.sd(base.wrapping_add(0x78), c.g[T3][0]);
    c.swc1(12, base.wrapping_add(0x38));
    c.swc1(14, base.wrapping_add(0x3c));
    c.swc1(16, base.wrapping_add(0x40));
    c.swc1(18, base.wrapping_add(0x44));
    frame.close(c);
}

/// `void World_GlobalCtors(void)` (0x3adda0): runs the static init of the
/// world (`World_StaticInitDestroy(1, 0xffff)`).
pub fn world_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        WORLD_STATIC_INIT_DESTROY.call(c, 0x003a_ddb4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, Rng, DATA};

    const SEEDS: u32 = 200;

    /// Scrambled registers, the fake hooks and the entry `pc`.
    fn machine(c: &mut Cpu, seed: u32, salt: u64, entry: u32) -> Rng {
        let r = fakes::machine(c, seed, salt, 0);
        c.pc = entry;
        r
    }

    #[test]
    fn cworld_cache_evict_lowest_priority_hull_matches() {
        check(
            ee::s13::f_003a89c8__generated,
            cworld_cache_evict_lowest_priority_hull,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a89, 0x003a_89c8);
                let this = DATA + 0x100;
                let count = seed % 6;
                c.bus.write32(this + 0x3ec, count);
                for i in 0..count {
                    let rec = this + 0x3f0 + 0x18 * i;
                    let state = match r.next_u32() % 8 {
                        7 => u32::MAX,
                        s => s,
                    };
                    c.bus.write32(rec, state);
                    c.bus
                        .write32(rec + 4, if r.next_u32() % 3 == 0 { 1 } else { 0 });
                    // The page this slot holds (FreeHullPage reads it at +8).
                    let page = DATA + 0x2000 + 0x100 * i;
                    c.bus.write32(rec + 8, page);
                    c.bus.write32(page, 0);
                    c.bus.write32(page + 8, 0);
                    c.bus.write32(page + 0xc, DATA + 0x3000);
                    c.bus.write32(rec + 0xc, r.f32(1.5));
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = seed as u64;
            },
        );
    }

    #[test]
    fn cworld_cache_streamer_dtor_stub_matches() {
        check(
            ee::s13::f_003a9180__generated,
            cworld_cache_streamer_dtor_stub,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x3a91, 0x003a_9180);
            },
        );
    }

    #[test]
    fn cworld_cache_streamer_release_section_matches() {
        check(
            ee::s13::f_003a9958__generated,
            cworld_cache_streamer_release_section,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a99, 0x003a_9958);
                let view = DATA + 0x1000;
                let hdr = DATA + 0x2000;
                c.bus.write32(view + 4, hdr);
                c.bus.write32(hdr + 8, 1 + seed % 4);
                // The case targets the generated code reads from 0x494f30.
                let targets = [
                    0x003a_99ac,
                    0x003a_99b4,
                    0x003a_99c4,
                    0x003a_99cc,
                    0x003a_99b4,
                    0x003a_99c4,
                    0x0012_3450,
                ];
                for (i, t) in targets.iter().enumerate() {
                    c.bus.write32(0x0049_4f30 + 4 * i as u32, *t);
                }
                for i in 0..6 {
                    c.bus.write32(view + 0x14 + 8 * i, r.next_u32() % 8);
                }
                c.g[A0][0] = sx(view);
                c.g[A1][0] = (seed % 5) as u64;
            },
        );
    }

    #[test]
    fn cworld_cache_streamer_are_box_hulls_resident_matches() {
        check(
            ee::s13::f_003a9ac0__generated,
            cworld_cache_streamer_are_box_hulls_resident,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a9a, 0x003a_9ac0);
                let view = DATA + 0x1000;
                let hdr = DATA + 0x2000;
                let section = DATA + 0x3000;
                let hulls = DATA + 0x4000;
                c.bus.write32(view, DATA + 0x7000);
                c.bus.write32(view + 4, hdr);
                c.bus.write32(view + 8, section);
                c.bus.write32(view + 0x10, hulls);
                c.bus.write32(hdr + 8, seed % 3);
                c.bus.write32(section + 0x18, seed % 4);
                c.bus.write16(hulls + 0x44 * (seed % 4), (seed % 2) as u16);
                for i in 0..4 {
                    c.bus.write32(view + 0x14 + 8 * i, r.next_u32() % 8);
                }
                for w in 0..24 {
                    c.bus.write32(DATA + 0x8000 + 4 * w, r.f32(4.0));
                }
                c.g[A0][0] = sx(view);
                c.g[A1][0] = sx(DATA + 0x8000);
            },
        );
    }

    #[test]
    fn cworld_cache_streamer_check_node_box_matches() {
        check(
            ee::s13::f_003a9bb8__generated,
            cworld_cache_streamer_check_node_box,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a9b, 0x003a_9bb8);
                let world = DATA + 0x5000;
                let node = DATA + 0x4000;
                let child = DATA + 0x4100;
                let bx = DATA + 0x6000;
                let ok = DATA + 0x6100;
                c.bus.write32(world, DATA + 0x7000);
                c.bus.write32(ok, 1);
                for n in [node, child] {
                    for k in 0..8 {
                        c.bus.write32(n + 4 * k, r.f32(3.0));
                    }
                    for k in 0..8 {
                        c.bus.write32(n + 0x10 + 4 * k, r.f32(3.0));
                    }
                    c.bus.write32(n + 0x20, 0);
                }
                for k in 0..8 {
                    c.bus.write32(bx + 4 * k, r.f32(3.0));
                    c.bus.write32(node + 0x20 + 4 * k % 0x10, r.f32(1.0));
                }
                c.bus
                    .write32(node + 0x50, if seed % 2 == 0 { child } else { 0 });
                c.bus
                    .write32(node + 0x54, if seed % 3 == 0 { child } else { 0 });
                c.bus.write32(node + 0x58, 0);
                c.g[A0][0] = sx(world);
                c.g[A1][0] = sx(node);
                c.g[A2][0] = sx(bx);
                c.g[A3][0] = sx(ok);
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_calc_percent_matches() {
        check(
            ee::s13::f_003aace0__generated,
            cps2_chunk_resolve_calc_percent,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aac, 0x003a_ace0);
                let light = DATA + 0x100;
                c.bus.write32(light + 0x14, r.f32(4.0));
                c.bus.write32(light + 0x18, r.f32(4.0));
                c.bus.write8(light + 0x64, (seed % 6) as u8);
                c.f[12] = r.f32(3.0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(light);
                c.g[GP][0] = crate::difftest::GAME_GP as u64;
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_dtor_stub_matches() {
        check(
            ee::s13::f_003aae60__generated,
            cps2_chunk_resolve_dtor_stub,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x3aae, 0x003a_ae60);
            },
        );
    }

    #[test]
    fn cwscript_cache_dtor_matches() {
        check(
            ee::s13::f_003ac8f0__generated,
            cwscript_cache_dtor,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3ac8, 0x003a_c8f0);
                let cache = DATA + 0x100;
                // No array (the free paths are skipped) for the seeds whose
                // array would need the real allocator; the flag bit is set on
                // odd seeds.
                c.bus.write32(cache + 0x4, 0);
                c.bus.write32(cache + 0x8, r.next_u32());
                c.g[A0][0] = sx(cache);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn cwscript_cache_update_matches() {
        check(
            ee::s13::f_003ac980__generated,
            cwscript_cache_update,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x3ac9, 0x003a_c980);
                let cache = DATA + 0x100;
                let entries = DATA + 0x200;
                c.bus.write32(cache + 0x4, entries);
                let count = seed % 4;
                c.bus.write32(cache + 0x8, count);
                for i in 0..count {
                    // state 0: the poll returns at once
                    c.bus.write16(entries + 0xc * i, 0);
                }
                c.g[A0][0] = sx(cache);
            },
        );
    }

    #[test]
    fn cwscript_cache_load_matches() {
        check(
            ee::s13::f_003ac9e0__generated,
            cwscript_cache_load,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x3ac9, 0x003a_c9e0);
                let cache = DATA + 0x100;
                c.bus.write32(cache + 0x4, DATA + 0x200);
                c.g[A0][0] = sx(cache);
                c.g[A1][0] = (seed % 3) as u64;
                c.g[A2][0] = (seed % 5) as u64;
                c.g[GP][0] = crate::difftest::GAME_GP as u64;
            },
        );
    }

    #[test]
    fn twscript_cache_poll_load_matches() {
        check(
            ee::s13::f_003acad0__generated,
            twscript_cache_poll_load,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x3aca, 0x003a_cad0);
                let entry = DATA + 0x100;
                // Entry states 0..=2: the generated code only works on state 1.
                c.bus.write16(entry, (seed % 3) as u16);
                c.bus.write32(entry + 4, DATA + 0x200);
                c.g[A0][0] = sx(entry);
            },
        );
    }

    #[test]
    fn twscript_cache_load_matches() {
        check(
            ee::s13::f_003acbb8__generated,
            twscript_cache_load,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x3acb, 0x003a_cbb8);
                let entry = DATA + 0x100;
                c.g[A0][0] = sx(entry);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[GP][0] = crate::difftest::GAME_GP as u64;
            },
        );
    }

    #[test]
    fn world_return_self_matches() {
        check(
            ee::s13::f_003ad250__generated,
            world_return_self,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x3ad2, 0x003a_d250);
            },
        );
    }

    #[test]
    fn world_global_ctors_matches() {
        check(
            ee::s13::f_003adda0__generated,
            world_global_ctors,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x3add, 0x003a_dda0);
            },
        );
    }
}
