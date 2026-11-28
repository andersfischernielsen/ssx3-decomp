//! `src/path/pathsys.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::dirtysock::tagsunk::at_3 as at;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::p;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0026_b598, path_data_has_marker, "PathData_HasMarker"),
    (0x0026_b5e0, path_data_find_marker, "PathData_FindMarker"),
    (
        0x0026_b7d8,
        path_data_read_float_int_pairs,
        "PathData_ReadFloatIntPairs",
    ),
    (
        0x0026_b508,
        path_data_get_track_path_index,
        "PathData_GetTrackPathIndex",
    ),
    (
        0x0026_b550,
        path_data_get_ai_path_index,
        "PathData_GetAIPathIndex",
    ),
    (0x0026_b880, path_data_read_markers, "PathData_ReadMarkers"),
    (
        0x0026_b310,
        path_data_read_track_paths,
        "PathData_ReadTrackPaths",
    ),
    (0x0026_b410, path_data_read_ai_paths, "PathData_ReadAIPaths"),
    (
        0x0026_b680,
        path_data_find_nearest_marker,
        "PathData_FindNearestMarker",
    ),
    (
        0x0026_baa8,
        path_data_debug_printf_stub,
        "PathData_DebugPrintfStub",
    ),
    (0x0026_bae8, path_data_static_init, "PathData_StaticInit"),
    (0x0026_c438, path_data_global_ctors, "PathData_GlobalCtors"),
    (0x0026_c410, path_init_fields_0026c410, "Path_InitFields"),
    (0x0026_c418, path_read_field_0026c418, "Path_ReadField"),
];

/// `bool PathData_HasMarker(PathData* self, int a /*a1*/, int b /*a2*/)`
/// (0x26b598): true when one of the `count` markers (0x28 bytes each, a
/// 16-byte head at +4) matches `(b, a)`.
pub fn path_data_has_marker(c: &mut Cpu) {
    c.cycles += 3;
    let count = c.lw(r32(c, A0));
    c.g[A3][0] = count;
    let none = (count as i64) <= 0;
    c.g[V1][0] = 0;
    if !none {
        c.cycles += 1;
        c.g[A0][0] = c.lw(at(c, A0, 0x04));
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(at(c, A0, 0x04));
            let mismatch = c.g[V0][0] != c.g[A1][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            if mismatch {
                c.cycles += 1;
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A3][0] as i64)) as u64;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw(r32(c, A0));
                let mismatch = c.g[V0][0] != c.g[A2][0];
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A3][0] as i64)) as u64;
                if !mismatch {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    abi::ret(c);
                    return;
                }
            }
            c.cycles += 2;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x28));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0026_b5a8);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `PathMarker* PathData_FindMarker(PathData* self, int type, int id)`
/// (0x26b5e0): linear search of the 0x28-byte marker array; then the first
/// marker when there is one, else null.
pub fn path_data_find_marker(c: &mut Cpu) {
    c.cycles += 3;
    let list = c.g[A0][0];
    let ty = c.g[A1][0];
    let id = c.g[A2][0];
    c.g[V0][0] = lw(c, list as u32);
    let empty = (c.g[V0][0] as i64) <= 0;
    c.g[T0][0] = 0;
    let mut found: Option<u64> = None;
    if !empty {
        c.cycles += 3;
        let mut entry = lw(c, p(list as u32, 4));
        c.g[A3][0] = entry;
        let count = c.g[V0][0];
        c.g[T1][0] = count;
        loop {
            c.cycles += 4;
            c.g[V1][0] = entry;
            c.g[V0][0] = lw(c, p(entry as u32, 4));
            let next = c.g[V0][0] != ty;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
            if !next {
                c.cycles += 3;
                c.g[V0][0] = lw(c, entry as u32);
                let hit = c.g[V0][0] == id;
                if hit {
                    c.cycles += 2;
                    found = Some(entry);
                }
            }
            if found.is_some() {
                c.g[V0][0] = found.unwrap();
                abi::ret(c);
                return;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[T1][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            entry = sx((entry as u32).wrapping_add(0x28));
            c.g[A3][0] = entry;
            if !more {
                c.cycles += 1;
                c.g[V0][0] = lw(c, list as u32);
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0026_b5f8);
            }
        }
    }
    c.cycles += 2;
    let nonempty = (c.g[V0][0] as i64) > 0;
    c.g[V0][0] = 0;
    c.cycles += 2;
    if nonempty {
        c.g[V0][0] = lw(c, p(list as u32, 4));
    }
    abi::ret(c);
}

/// `int PathData_ReadFloatIntPairs(pairs, p)` (0x26b7d8): read a count and
/// then that many (float, int) pairs, returning the bytes consumed.
pub fn path_data_read_float_int_pairs(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S2, 0x40);
        c.g[A0][0] = c.g[SP][0];
        frame.save_ra(c, 0x10);
        c.g[S4][0] = c.g[S0][0];
        call_ptr(c, 0x0026_ba68, 0x0026b808)?;
        c.g[S0][0] = sx(p(r32(c, S0), 4));
        c.g[V0][0] = Addr(r32(c, SP)).lw(c);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = 0;
        if !none {
            c.cycles += 2;
            c.g[S2][0] = sx(p(r32(c, S1), 0x18));
            c.g[S1][0] = sx(p(r32(c, S1), 0x1c));
            loop {
                c.cycles += 14;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0026_ba88, 0x0026b82c)?;
                c.g[S0][0] = sx(p(r32(c, S0), 4));
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0026_ba68, 0x0026b83c)?;
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
                c.g[V0][0] = Addr(r32(c, SP)).lw(c);
                c.g[S0][0] = sx(p(r32(c, S0), 4));
                c.g[S1][0] = sx(p(r32(c, S1), 8));
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = sx(p(r32(c, S2), 8));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_b820);
                }
            }
        }
        c.cycles += 9;
        c.g[V0][0] = sx(r32(c, S0).wrapping_sub(r32(c, S4)));
        frame.restore(c, S1, 0x50);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `PathData` AI path table: entry count at +0x8, entries (0x40 bytes each)
/// at +0xc.
pub(crate) const PATH_DATA_AI_COUNT: u32 = 0x8;

pub(crate) const PATH_DATA_AI_ENTRIES: u32 = 0xc;

pub(crate) const PATH_DATA_AI_ENTRY_SIZE: u32 = 0x40;

/// `int PathData_GetAIPathIndex(PathData* tree, AIPath* path)` (0x26b550):
/// the index of `path` in the tree's AI path array, or -1 (also for a null
/// path). The loop checks events at its head 0x26b570.
pub fn path_data_get_ai_path_index(c: &mut Cpu) {
    c.cycles += 2;
    let path = c.g[A1][0];
    if path != 0 {
        c.cycles += 3;
        let tree = Addr(c.g[A0][0] as u32);
        let count = tree.off(PATH_DATA_AI_COUNT).lw(c);
        c.g[V0][0] = count;
        c.g[V1][0] = 0;
        if (count as i64) > 0 {
            c.cycles += 3;
            let mut entry = tree.off(PATH_DATA_AI_ENTRIES).lw(c);
            let mut index: u64 = 0;
            let mut count = count;
            c.g[A2][0] = count;
            loop {
                c.cycles += 2;
                if entry == path {
                    c.g[A0][0] = entry;
                    c.g[V1][0] = index;
                    c.cycles += 2;
                    c.g[V0][0] = index;
                    ret(c);
                    return;
                }
                index = sx((index as u32).wrapping_add(1));
                c.cycles += 3;
                let more = (index as i64) < (count as i64);
                entry = sx((entry as u32).wrapping_add(PATH_DATA_AI_ENTRY_SIZE));
                if !more {
                    c.g[A0][0] = entry;
                    c.g[V1][0] = index;
                    c.g[V0][0] = 0;
                    break;
                }
                if event_due(c) {
                    c.g[A0][0] = entry;
                    c.g[V1][0] = index;
                    c.g[V0][0] = 1;
                    deliver_event_here(c, 0x0026_b570);
                    entry = c.g[A0][0];
                    index = c.g[V1][0];
                    count = c.g[A2][0];
                }
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = u64::MAX;
    ret(c);
}

/// `PathData` track path table: entry count at +0x10, entries (0x3c bytes
/// each) at +0x14.
pub(crate) const PATH_DATA_TRACK_COUNT: u32 = 0x10;

pub(crate) const PATH_DATA_TRACK_ENTRIES: u32 = 0x14;

pub(crate) const PATH_DATA_TRACK_ENTRY_SIZE: u32 = 0x3c;

/// `int PathData_GetTrackPathIndex(PathData* tree, TrackPath* path)`
/// (0x26b508): the index of `path` in the tree's track path array, or -1
/// (also for a null path). The loop checks events at its head 0x26b528.
pub fn path_data_get_track_path_index(c: &mut Cpu) {
    c.cycles += 2;
    let path = c.g[A1][0];
    if path != 0 {
        c.cycles += 3;
        let tree = Addr(c.g[A0][0] as u32);
        let count = tree.off(PATH_DATA_TRACK_COUNT).lw(c);
        c.g[V0][0] = count;
        c.g[V1][0] = 0;
        if (count as i64) > 0 {
            c.cycles += 3;
            let mut entry = tree.off(PATH_DATA_TRACK_ENTRIES).lw(c);
            let mut index: u64 = 0;
            let mut count = count;
            c.g[A2][0] = count;
            loop {
                c.cycles += 2;
                if entry == path {
                    c.g[A0][0] = entry;
                    c.g[V1][0] = index;
                    c.cycles += 2;
                    c.g[V0][0] = index;
                    ret(c);
                    return;
                }
                index = sx((index as u32).wrapping_add(1));
                c.cycles += 3;
                let more = (index as i64) < (count as i64);
                entry = sx((entry as u32).wrapping_add(PATH_DATA_TRACK_ENTRY_SIZE));
                if !more {
                    c.g[A0][0] = entry;
                    c.g[V1][0] = index;
                    c.g[V0][0] = 0;
                    break;
                }
                if event_due(c) {
                    c.g[A0][0] = entry;
                    c.g[V1][0] = index;
                    c.g[V0][0] = 1;
                    deliver_event_here(c, 0x0026_b528);
                    entry = c.g[A0][0];
                    index = c.g[V1][0];
                    count = c.g[A2][0];
                }
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = u64::MAX;
    ret(c);
}

/// `int PathData_ReadMarkers(PathData* self, char* p)`: reads the marker table of a path file at `p` (count, then per node id, type key, position, direction and two indices) into `self` and returns the number of bytes consumed. The node type is the index of its key in the 3-entry table at 0x445428.
/// (`PathData_ReadMarkers`, 0x0026b880)
pub fn path_data_read_markers(c: &mut Cpu) {
    guest(c, |c| {
        'b_26ba18: {
            c.cycles += 21;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S0][0] = c.g[A1][0];
            c.g[S1][0] = c.g[A0][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[A0][0] = c.g[SP][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S7][0] = c.g[S0][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[S3][0] = 0u64;
            Callee {
                addr: 0x0026ba68,
                f: ee::s07::PathData_ReadInt,
            }
            .call(c, 0x0026b8c4)?;
            c.g[V0][0] = c.lw(c.g[SP][0] as u32);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
            let t = (c.g[V0][0] as i64) <= 0;
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            if t {
                break 'b_26ba18;
            } else {
                c.cycles += 5;
                c.g[S6][0] = 0x440000;
                c.g[S4][0] = 0x28;
                c.g[S5][0] = sx((c.g[S6][0] as u32).wrapping_add(0x5428));
                c.mult(0, 18, 19, 20);
                'l_26b8e8: loop {
                    'b_26b9c4: {
                        'b_26b990: {
                            c.cycles += 37;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                            Callee {
                                addr: 0x0026ba68,
                                f: ee::s07::PathData_ReadInt,
                            }
                            .call(c, 0x0026b8fc)?;
                            c.g[A0][0] = c.g[SP][0];
                            c.g[A1][0] = c.g[S0][0];
                            Callee {
                                addr: 0x0026ba68,
                                f: ee::s07::PathData_ReadInt,
                            }
                            .call(c, 0x0026b908)?;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
                            Callee {
                                addr: 0x0026ba48,
                                f: ee::s07::PathData_ReadVec3,
                            }
                            .call(c, 0x0026b924)?;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
                            Callee {
                                addr: 0x0026ba48,
                                f: ee::s07::PathData_ReadVec3,
                            }
                            .call(c, 0x0026b93c)?;
                            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4));
                            c.g[A1][0] = c.g[S0][0];
                            Callee {
                                addr: 0x0026ba68,
                                f: ee::s07::PathData_ReadInt,
                            }
                            .call(c, 0x0026b948)?;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x8));
                            Callee {
                                addr: 0x0026ba68,
                                f: ee::s07::PathData_ReadInt,
                            }
                            .call(c, 0x0026b958)?;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[T0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                            c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x5428));
                            c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[A1][0] = 0u64;
                            if t {
                                break 'b_26b990;
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] =
                                    sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                let t = true;
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                                if t {
                                    break 'b_26b9c4;
                                } else {
                                    c.cycles += 1;
                                    break 'b_26b990;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                        'l_26b994: loop {
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[A1][0] as i64) < 3i64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                            if t {
                                break 'b_26b9c4;
                            } else {
                                c.cycles += 5;
                                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                if c.g[V1][0] != c.g[A0][0] {
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0026b994);
                                    }
                                    continue 'l_26b994;
                                } else {
                                    c.cycles += 4;
                                    c.mult(0, 2, 19, 20);
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
                                    break 'b_26b9c4;
                                }
                            }
                        }
                    }
                    c.cycles += 21;
                    c.mult(0, 6, 19, 20);
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[A3][0] = 0x3c;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
                    c.g[S3][0] = c.g[T0][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.mult(0, 4, 4, 7);
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x20), c.g[V1][0]);
                    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.mult(0, 18, 19, 20);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0026b8e8);
                        }
                        continue 'l_26b8e8;
                    } else {
                        break 'b_26ba18;
                    }
                }
            }
        }
        c.cycles += 12;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[S7][0] as u32));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        Ok(())
    })
}

/// `int PathData_ReadTrackPaths(PathData* self, char* data)` (0x26b310): reads the
/// track path table from a path file: the count (kept at `+0x10`), a `count * 0x3c`
/// byte array ("TrackPaths", pointer at `+0x14`) whose entries get their vtable
/// (`+0x34`) and are then read with `Path_Read`. Returns the bytes consumed.
pub fn path_data_read_track_paths(c: &mut Cpu) {
    guest(c, |c| {
        'b_26b3ac: {
            c.cycles += 28;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S1][0] = c.g[A1][0];
            c.g[S3][0] = c.g[A0][0];
            c.g[A0][0] = c.g[SP][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x14));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[S4][0] = c.g[S1][0];
            Callee {
                addr: 0x0026ba68,
                f: ee::s07::PathData_ReadInt,
            }
            .call(c, 0x0026b344)?;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
            c.g[S0][0] = c.lw(c.g[SP][0] as u32);
            c.g[A0][0] = 0x3c;
            c.g[A1][0] = 0x480000;
            c.g[A2][0] = 0u64;
            c.mult(0, 0, 16, 4);
            c.sw((c.g[S3][0] as u32).wrapping_add(0x10), c.g[S0][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1438));
            c.g[A0][0] = c.lo[0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317e30, 0x0026b370)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            let t = c.g[S0][0] == 0u64;
            c.g[V1][0] = c.g[A1][0];
            if t {
                break 'b_26b3ac;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x480000;
                c.g[A2][0] = 0xffffffffffffffff;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1448));
                'l_26b390: loop {
                    c.cycles += 7;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                    let t = c.g[A0][0] != c.g[A2][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3c));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0026b390);
                        }
                        continue 'l_26b390;
                    } else {
                        break 'b_26b3ac;
                    }
                }
            }
        }
        'b_26b3e8: {
            c.cycles += 4;
            c.sw(c.g[S2][0] as u32, c.g[A1][0]);
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S0][0] = 0u64;
            if t {
                break 'b_26b3e8;
            } else {
                c.cycles += 1;
                c.g[S2][0] = 0u64;
                'l_26b3c0: loop {
                    c.cycles += 10;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    Callee {
                        addr: 0x0026a180,
                        f: ee::s07::Path_Read,
                    }
                    .call(c, 0x0026b3d4)?;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x3c));
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let t = c.g[V1][0] != 0u64;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0026b3c0);
                        }
                        continue 'l_26b3c0;
                    } else {
                        break 'b_26b3e8;
                    }
                }
            }
        }
        c.cycles += 9;
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[S4][0] as u32));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}

/// `int PathData_ReadAIPaths(PathData* self, char* data)` (0x26b410): the same for the
/// AI path table: the count (kept at `+0x8`), a `count * 0x40` byte array ("AIPaths",
/// pointer at `+0xc`), each entry with its vtable at `+0x34`, read with `Path_Read`.
/// Returns the bytes consumed.
pub fn path_data_read_ai_paths(c: &mut Cpu) {
    guest(c, |c| {
        'b_26b4a4: {
            c.cycles += 26;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S1][0] = c.g[A1][0];
            c.g[S2][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[A0][0] = c.g[SP][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[S4][0] = c.g[S1][0];
            Callee {
                addr: 0x0026ba68,
                f: ee::s07::PathData_ReadInt,
            }
            .call(c, 0x0026b444)?;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
            c.g[S0][0] = c.lw(c.g[SP][0] as u32);
            c.g[A1][0] = 0x4a0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3408));
            c.g[A2][0] = 0u64;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[S0][0]);
            c.g[A0][0] = sx((c.g[S0][0] as u32) << 6);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317e30, 0x0026b468)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            let t = c.g[S0][0] == 0u64;
            c.g[V1][0] = c.g[A1][0];
            if t {
                break 'b_26b4a4;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x480000;
                c.g[A2][0] = 0xffffffffffffffff;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1478));
                'l_26b488: loop {
                    c.cycles += 7;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                    let t = c.g[A0][0] != c.g[A2][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x40));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0026b488);
                        }
                        continue 'l_26b488;
                    } else {
                        break 'b_26b4a4;
                    }
                }
            }
        }
        'b_26b4e0: {
            c.cycles += 4;
            c.sw(c.g[S3][0] as u32, c.g[A1][0]);
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S0][0] = 0u64;
            if t {
                break 'b_26b4e0;
            } else {
                c.cycles += 1;
                'l_26b4b8: loop {
                    c.cycles += 10;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = sx((c.g[S0][0] as u32) << 6);
                    c.g[A1][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    Callee {
                        addr: 0x0026a180,
                        f: ee::s07::Path_Read,
                    }
                    .call(c, 0x0026b4d0)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let t = c.g[V1][0] != 0u64;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0026b4b8);
                        }
                        continue 'l_26b4b8;
                    } else {
                        break 'b_26b4e0;
                    }
                }
            }
        }
        c.cycles += 9;
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[S4][0] as u32));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}

/// `PathData_StaticInit` (0x0026bae8): pathsys.cpp's static constants. When `initialize` is
/// nonzero and `priority` is 0xFFFF, it writes the 56 four-float colour globals
/// (`D_004D2FB8`..`D_004D3328`), the three-float vectors (`D_004D3338`..`D_004D3368`), the
/// nine-float block `D_004D3378` and the key `D_004A5158` (both fields -1). The float
/// literals are read from the GP-relative pool, as the generated body reads them. The body is
/// a line-for-line transcription of the generated one, produced by script so the flat table
/// is not retyped by hand.
#[allow(unused_assignments, unused_variables, unused_mut)]
pub fn path_data_static_init(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x0026bae8);
    'b_26c3a0: {
        c.cycles += 12;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
        c.g[V0][0] = 0xffff;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[A1][0] != c.g[V0][0];
        c.sq(30, c.g[SP][0] as u32);
        if t {
            break 'b_26c3a0;
        } else {
            c.cycles += 2;
            let t = c.g[A0][0] == 0u64;
            c.g[T5][0] = 0x4d0000;
            if t {
                break 'b_26c3a0;
            } else {
                c.cycles += 544;
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x4a38));
                c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x4a34));
                c.g[T6][0] = 0x4d0000;
                c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x4a30));
                c.g[T7][0] = 0x4d0000;
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x4a2c));
                c.g[V0][0] = sx((c.g[T7][0] as u32).wrapping_add(0x2fe8));
                c.swc1(8, (c.g[T5][0] as u32).wrapping_add(0x2fc8));
                c.g[S1][0] = 0x4d0000;
                c.swc1(10, (c.g[T6][0] as u32).wrapping_add(0x2fd8));
                c.g[S2][0] = 0x4d0000;
                c.swc1(11, (c.g[T7][0] as u32).wrapping_add(0x2fe8));
                c.g[S3][0] = 0x4d0000;
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4a28));
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x3018));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4a24));
                c.g[S5][0] = 0x4d0000;
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4a20));
                c.g[S4][0] = 0x4d0000;
                c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[T3][0] = 0x4d0000;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4a1c));
                c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x2fb8));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.g[A3][0] = sx((c.g[T5][0] as u32).wrapping_add(0x2fc8));
                c.swc1(7, (c.g[S1][0] as u32).wrapping_add(0x2ff8));
                c.g[T1][0] = sx((c.g[T6][0] as u32).wrapping_add(0x2fd8));
                c.swc1(4, (c.g[S2][0] as u32).wrapping_add(0x3008));
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2ff8));
                c.swc1(5, (c.g[S3][0] as u32).wrapping_add(0x3018));
                c.g[T0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x3008));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4a18));
                c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x3028));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4a14));
                c.g[S2][0] = 0x4d0000;
                c.swc1(2, (c.g[A0][0] as u32).wrapping_add(0xc));
                c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x3098));
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 9);
                c.g[S3][0] = 0x4d0000;
                c.swc1(3, (c.g[S4][0] as u32).wrapping_add(0x3028));
                c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x30a8));
                c.swc1(1, (c.g[T3][0] as u32).wrapping_add(0x3038));
                c.g[FP][0] = 0x4d0000;
                c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x2fb8));
                c.g[T4][0] = sx((c.g[T3][0] as u32).wrapping_add(0x3038));
                c.swc1(9, (c.g[A1][0] as u32).wrapping_add(0xc));
                c.g[S3][0] = sx((c.g[FP][0] as u32).wrapping_add(0x30b8));
                c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
                c.g[T9][0] = 0x4d0000;
                c.lwc1(17, (c.g[GP][0] as u32).wrapping_sub(0x4a10));
                c.g[FP][0] = 0x4d0000;
                c.lwc1(18, (c.g[GP][0] as u32).wrapping_sub(0x4a0c));
                c.g[T2][0] = sx((c.g[T9][0] as u32).wrapping_add(0x3048));
                c.sw((c.g[T1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[S6][0] = 0x4d0000;
                c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x4a08));
                c.g[T7][0] = sx((c.g[S6][0] as u32).wrapping_add(0x3078));
                c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x4a04));
                c.g[T8][0] = 0x4d0000;
                c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[T5][0] = sx((c.g[T8][0] as u32).wrapping_add(0x3058));
                c.swc1(18, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[S7][0] = 0x4d0000;
                c.swc1(13, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = 0x4d0000;
                c.swc1(14, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x30c8));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[T6][0] = sx((c.g[S7][0] as u32).wrapping_add(0x3068));
                c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x4a00));
                c.g[V1][0] = 0x4d0000;
                c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x49fc));
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3108));
                c.sw((c.g[T0][0] as u32).wrapping_add(0xc), 0u64);
                c.g[V1][0] = 0x4d0000;
                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x49f8));
                c.g[T3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3138));
                c.swc1(15, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = 0x4d0000;
                c.swc1(16, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[S0][0] = 0x4d0000;
                c.swc1(12, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x4d0000;
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
                c.g[S4][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3118));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3128));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[FP][0] = 0x4d0000;
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3158));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[FP][0] = 0x4d0000;
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x4));
                c.g[T1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3188));
                c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x8));
                c.lwc1(19, (c.g[GP][0] as u32).wrapping_sub(0x49f4));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x49f0));
                c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3168));
                c.swc1(19, (c.g[T4][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = 0x4d0000;
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x49ec));
                c.g[FP][0] = sx((c.g[V1][0] as u32).wrapping_add(0x31a8));
                c.swc1(2, (c.g[T9][0] as u32).wrapping_add(0x3048));
                c.g[V1][0] = 0x4d0000;
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x49e8));
                c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x31b8));
                c.swc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = 0x4d0000;
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x49e4));
                c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3218));
                c.swc1(7, (c.g[T7][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = c.g[S0][0];
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x49e0));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3088));
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x49dc));
                c.swc1(3, (c.g[T4][0] as u32).wrapping_add(0x8));
                c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
                c.swc1(4, (c.g[T2][0] as u32).wrapping_add(0x4));
                c.swc1(5, (c.g[T2][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[T8][0] as u32).wrapping_add(0x3058));
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0xc));
                c.swc1(0, (c.g[S7][0] as u32).wrapping_add(0x3068));
                c.sw((c.g[T6][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x49d8));
                c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x3078));
                c.swc1(6, (c.g[T7][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x3088));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = 0x4d0000;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x49d4));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x3098));
                c.g[V1][0] = 0x4d0000;
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[T2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3248));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x49d0));
                c.g[V1][0] = 0x4d0000;
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x49cc));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x30a8));
                c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
                c.g[V1][0] = 0x4d0000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x30b8));
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = c.g[S0][0];
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3088));
                c.sw((c.g[T6][0] as u32).wrapping_add(0x4), 0u64);
                c.sw((c.g[T6][0] as u32).wrapping_add(0x8), 0u64);
                c.swc1(7, (c.g[T7][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = 0x4d0000;
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[T7][0] = sx((c.g[V1][0] as u32).wrapping_add(0x32c8));
                c.sw((c.g[S2][0] as u32).wrapping_add(0x4), 0u64);
                c.g[V1][0] = 0x4d0000;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
                c.g[T4][0] = sx((c.g[V1][0] as u32).wrapping_add(0x32e8));
                c.sw((c.g[S3][0] as u32).wrapping_add(0x4), 0u64);
                c.g[S2][0] = 0x4d0000;
                c.sw((c.g[S3][0] as u32).wrapping_add(0x8), 0u64);
                c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x32a8));
                c.sw((c.g[S3][0] as u32).wrapping_add(0xc), 0u64);
                c.g[S2][0] = 0x4d0000;
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[S3][0] = 0x4d0000;
                c.swc1(1, (c.g[S2][0] as u32).wrapping_add(0x3128));
                c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x30c8));
                c.g[S2][0] = 0x4d0000;
                c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0xc));
                c.g[S3][0] = 0x4d0000;
                c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x30d8));
                c.g[V1][0] = c.g[S2][0];
                c.g[T5][0] = sx((c.g[S3][0] as u32).wrapping_add(0x32f8));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30d8));
                c.g[S2][0] = 0x4d0000;
                c.swc1(9, (c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[T6][0] = sx((c.g[S2][0] as u32).wrapping_add(0x3308));
                c.g[S3][0] = 0x4d0000;
                c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x30e8));
                c.g[V1][0] = 0x4d0000;
                c.g[S2][0] = 0x4d0000;
                c.g[S3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3318));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x30e8));
                c.g[V1][0] = 0x4d0000;
                c.swc1(9, (c.g[S2][0] as u32).wrapping_add(0xc));
                c.g[S2][0] = c.g[V1][0];
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x30f8));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x30f8));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x49c8));
                c.swc1(9, (c.g[S2][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = 0x4d0000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x3108));
                c.g[S2][0] = 0x4d0000;
                c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x49c4));
                c.g[V0][0] = 0x4d0000;
                c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x3118));
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3378));
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x49c0));
                c.g[S2][0] = 0xffffffffffff0000;
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x49bc));
                c.g[S2][0] = c.g[S2][0] | 0xff00;
                c.swc1(5, (c.g[S4][0] as u32).wrapping_add(0x8));
                c.sw((c.g[S4][0] as u32).wrapping_add(0xc), 0u64);
                c.swc1(6, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.swc1(7, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.sw((c.g[A0][0] as u32).wrapping_add(0xc), 0u64);
                c.g[A0][0] = 0x4d0000;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x49b8));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x3138));
                c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = 0x4d0000;
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x3148));
                c.g[A0][0] = 0x4d0000;
                c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30d8));
                c.swc1(9, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = 0x4d0000;
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x49b4));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30e8));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x49b0));
                c.swc1(9, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = 0x4d0000;
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x49ac));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30f8));
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x49a8));
                c.swc1(9, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x4d0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3148));
                c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.swc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
                c.swc1(1, (c.g[T1][0] as u32).wrapping_add(0xc));
                c.swc1(3, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.sw((c.g[A0][0] as u32).wrapping_add(0xc), 0u64);
                c.g[A0][0] = 0x4d0000;
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x49a4));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x3158));
                c.swc1(4, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x4d0000;
                c.swc1(5, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.sw((c.g[A1][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x49a0));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x3168));
                c.swc1(6, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x4d0000;
                c.swc1(7, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x3178));
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x499c));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3178));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0xc));
                c.swc1(8, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x4998));
                c.g[A0][0] = 0x4d0000;
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x3188));
                c.swc1(10, (c.g[T1][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x4d0000;
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x3198));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3198));
                c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x4994));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = 0x4d0000;
                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x4990));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x31a8));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = 0x4d0000;
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x31b8));
                c.swc1(11, (c.g[A3][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x4d0000;
                c.swc1(12, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x31c8));
                c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x498c));
                c.g[A0][0] = 0x4d0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3178));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4988));
                c.swc1(13, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x4d0000;
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3198));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4984));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x4d0000;
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
                c.g[FP][0] = 0x4d0000;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4980));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x31c8));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x497c));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0xc));
                c.swc1(4, (c.g[A0][0] as u32).wrapping_add(0x3208));
                c.g[FP][0] = 0x4d0000;
                c.swc1(3, (c.g[T0][0] as u32).wrapping_add(0xc));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3228));
                c.swc1(2, (c.g[FP][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = 0x4d0000;
                c.swc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
                c.g[FP][0] = c.g[A0][0];
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4978));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x31d8));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x31d8));
                c.swc1(5, (c.g[FP][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x4d0000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
                c.g[FP][0] = c.g[A0][0];
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x31e8));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x31e8));
                c.g[A0][0] = 0x4d0000;
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0xc));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x31f8));
                c.g[FP][0] = c.g[A0][0];
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x31f8));
                c.g[A0][0] = 0x4d0000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3208));
                c.sw((c.g[A0][0] as u32).wrapping_add(0xc), 0u64);
                c.g[FP][0] = 0x4d0000;
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x4974));
                c.g[A0][0] = 0x4d0000;
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4970));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x3218));
                c.swc1(6, (c.g[T0][0] as u32).wrapping_add(0x4));
                c.g[FP][0] = c.g[A0][0];
                c.swc1(7, (c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3228));
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x496c));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x3228));
                c.swc1(8, (c.g[FP][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x4d0000;
                c.g[FP][0] = c.g[A0][0];
                c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x4968));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x3238));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3238));
                c.swc1(10, (c.g[FP][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = 0x4d0000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x4964));
                c.g[FP][0] = 0x4d0000;
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x3248));
                c.swc1(11, (c.g[T2][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x4d0000;
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x3258));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x31d8));
                c.g[FP][0] = 0x4d0000;
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x31e8));
                c.g[A0][0] = 0x4d0000;
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x31f8));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
                c.g[FP][0] = 0x4d0000;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x8), 0u64);
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3208));
                c.sw((c.g[FP][0] as u32).wrapping_add(0x4), 0u64);
                c.g[A0][0] = 0x4d0000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0x8), 0u64);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3228));
                c.g[FP][0] = 0x4d0000;
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3238));
                c.g[A0][0] = 0x4d0000;
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3258));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4960));
                c.g[FP][0] = 0x4d0000;
                c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x8));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x32b8));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x495c));
                c.swc1(2, (c.g[S1][0] as u32).wrapping_add(0xc));
                c.swc1(1, (c.g[FP][0] as u32).wrapping_add(0xc));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = 0x4d0000;
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4958));
                c.g[FP][0] = c.g[A0][0];
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x3268));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3268));
                c.g[A0][0] = 0x4d0000;
                c.swc1(13, (c.g[FP][0] as u32).wrapping_add(0xc));
                c.g[FP][0] = c.g[A0][0];
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x3278));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3278));
                c.g[A0][0] = 0x4d0000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
                c.g[FP][0] = c.g[A0][0];
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x3288));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3288));
                c.g[A0][0] = 0x4d0000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
                c.g[FP][0] = c.g[A0][0];
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x3298));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3298));
                c.g[A0][0] = 0x4d0000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x32a8));
                c.g[FP][0] = 0x4d0000;
                c.swc1(3, (c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.g[FP][0];
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4954));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x32b8));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x32b8));
                c.swc1(4, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[FP][0] = 0x4d0000;
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4950));
                c.g[A0][0] = 0x4d0000;
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x32c8));
                c.swc1(5, (c.g[T7][0] as u32).wrapping_add(0x8));
                c.g[FP][0] = c.g[A0][0];
                c.sw((c.g[T7][0] as u32).wrapping_add(0xc), 0u64);
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x32d8));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x32d8));
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x494c));
                c.g[A0][0] = 0x4d0000;
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4948));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3258));
                c.swc1(6, (c.g[FP][0] as u32).wrapping_add(0x4));
                c.swc1(7, (c.g[FP][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[FP][0] = 0x4d0000;
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3268));
                c.g[A0][0] = 0x4d0000;
                c.swc1(13, (c.g[FP][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3278));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[FP][0] = 0x4d0000;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x8), 0u64);
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3288));
                c.g[A0][0] = 0x4d0000;
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
                c.sw((c.g[FP][0] as u32).wrapping_add(0x8), 0u64);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3298));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[FP][0] = 0x4d0000;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x8), 0u64);
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x32b8));
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = 0x4d0000;
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x32d8));
                c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x4));
                c.g[FP][0] = 0x4d0000;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4944));
                c.sw((c.g[A0][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4940));
                c.g[A0][0] = 0x4d0000;
                c.swc1(2, (c.g[T5][0] as u32).wrapping_add(0xc));
                c.swc1(1, (c.g[T6][0] as u32).wrapping_add(0xc));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x493c));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4938));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x32e8));
                c.swc1(3, (c.g[T4][0] as u32).wrapping_add(0x4));
                c.g[FP][0] = 0x4d0000;
                c.swc1(4, (c.g[T4][0] as u32).wrapping_add(0x8));
                c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4934));
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x4930));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x32f8));
                c.swc1(5, (c.g[T5][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x4d0000;
                c.swc1(6, (c.g[T5][0] as u32).wrapping_add(0x8));
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x492c));
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x4928));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x3308));
                c.swc1(7, (c.g[T6][0] as u32).wrapping_add(0x4));
                c.g[FP][0] = 0x4d0000;
                c.swc1(8, (c.g[T6][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x3318));
                c.sw((c.g[S3][0] as u32).wrapping_add(0xc), 0u64);
                c.g[A0][0] = c.g[FP][0];
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x3328));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3328));
                c.sw((c.g[A0][0] as u32).wrapping_add(0xc), 0u64);
                c.g[FP][0] = 0x4d0000;
                c.g[A0][0] = c.g[FP][0];
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x3338));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3338));
                c.g[FP][0] = 0x4d0000;
                c.swc1(9, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.g[FP][0];
                c.swc1(9, (c.g[FP][0] as u32).wrapping_add(0x3348));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3348));
                c.g[FP][0] = 0x4d0000;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x8), 0u64);
                c.g[A0][0] = c.g[FP][0];
                c.sw((c.g[FP][0] as u32).wrapping_add(0x3358), 0u64);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3358));
                c.g[FP][0] = 0x4d0000;
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.g[FP][0];
                c.sw((c.g[FP][0] as u32).wrapping_add(0x3368), 0u64);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3368));
                c.g[FP][0] = 0x4d0000;
                c.swc1(9, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3338));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x3378));
                c.g[A0][0] = 0x4d0000;
                c.sw((c.g[S3][0] as u32).wrapping_add(0x4), 0u64);
                c.g[V0][0] = 0x4d0000;
                c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3348));
                c.g[S3][0] = 0x4d0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3358));
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x3328));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4924));
                c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x4));
                c.sw((c.g[S3][0] as u32).wrapping_add(0x8), 0u64);
                c.sw((c.g[FP][0] as u32).wrapping_add(0x4), 0u64);
                c.g[S3][0] = 0x4d0000;
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x3368));
                c.swc1(9, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[FP][0] = 0xffffffffffffffff;
                c.sw((c.g[S3][0] as u32).wrapping_add(0x4), 0u64);
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.sb((c.g[GP][0] as u32).wrapping_add(0x2068), c.g[FP][0]);
                c.swc1(13, (c.g[V1][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = c.g[FP][0] & 0xff;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.g[V0][0] | c.g[S2][0];
                c.swc1(9, (c.g[V1][0] as u32).wrapping_add(0x14));
                c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x1c));
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2068), c.g[V0][0]);
                c.swc1(9, (c.g[V1][0] as u32).wrapping_add(0xc));
                c.swc1(9, (c.g[V1][0] as u32).wrapping_add(0x10));
                c.swc1(13, (c.g[V1][0] as u32).wrapping_add(0x18));
                break 'b_26c3a0;
            }
        }
    }
    c.cycles += 11;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lq(30, c.g[SP][0] as u32);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
    ret(c);
}

/// Fetch FPU register `r` as the value the FPU holds.
#[inline(always)]
pub(crate) fn find_marker_fpr(c: &Cpu, r: usize) -> FpuF32 {
    FpuF32(c.f[r])
}

/// `int PathData_FindNearestMarker(PathTable* list, Vec3* pos)` (0x26b680): the
/// id of the nearest entry of type 2 with a non-zero id that
/// `BEMissionInt_SessionTypeAtLeast` (`cBEMissionInterface_isSessionPoint`)
/// accepts in the current section, or -1. Entries are 0x28 bytes: the id at
/// +0, the type at +4, the position at +8. A strictly nearer entry replaces the
/// best one, which starts at the game constant `gp-0x4a3c`.
pub fn path_data_find_nearest_marker(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save(c, S5, 0x20);
        frame.save_fpr(c, 20, 0x80);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S2][0] = 0;
        c.g[S3][0] = c.g[A1][0];
        let best = Addr((c.g[GP][0] as u32).wrapping_sub(0x4a3c)).read_fpu(c);
        abi::set_fpr(c, 20, best);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[S4][0] = 0xffff_ffff_ffff_ffff;
        'search: {
            if (c.g[V0][0] as i64) <= 0 {
                break 'search;
            }
            c.cycles += 2;
            c.g[S5][0] = 2;
            c.g[S1][0] = 0;
            loop {
                'entry: {
                    c.cycles += 5;
                    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lw(c);
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(4)).lw(c);
                    if c.g[V1][0] != c.g[S5][0] {
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                        break 'entry;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                        break 'entry;
                    }
                    c.cycles += 16;
                    call_ptr(c, 0x0014_dc80, 0x0026_b6f0)?;
                    c.g[A1][0] = 0xa;
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0014_dd58, 0x0026_b6fc)?;
                    c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lw(c);
                    c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x84)).lw(c);
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A3][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x78)).lw(c);
                    c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
                    c.g[A2][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x1bc)).lw(c);
                    call_ptr(c, 0x0015_4630, 0x0026_b720)?;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                        break 'entry;
                    }
                    c.cycles += 25;
                    c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lw(c);
                    let p3 = Addr(c.g[S3][0] as u32).read_fpu(c);
                    abi::set_fpr(c, 3, p3);
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    let p5 = Addr((c.g[S3][0] as u32).wrapping_add(4)).read_fpu(c);
                    abi::set_fpr(c, 5, p5);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(8));
                    let p4 = Addr((c.g[S3][0] as u32).wrapping_add(8)).read_fpu(c);
                    abi::set_fpr(c, 4, p4);
                    let pos = c.g[V0][0] as u32;
                    let e0 = Addr(pos).read_fpu(c);
                    abi::set_fpr(c, 0, e0);
                    let e1 = Addr(pos.wrapping_add(4)).read_fpu(c);
                    abi::set_fpr(c, 1, e1);
                    let e2 = Addr(pos.wrapping_add(8)).read_fpu(c);
                    abi::set_fpr(c, 2, e2);
                    // difference = entry position - *pos, then the squared length
                    let dx = find_marker_fpr(c, 0) - find_marker_fpr(c, 3);
                    let dy = find_marker_fpr(c, 1) - find_marker_fpr(c, 5);
                    let dz = find_marker_fpr(c, 2) - find_marker_fpr(c, 4);
                    abi::set_fpr(c, 0, dx);
                    abi::set_fpr(c, 1, dy);
                    abi::set_fpr(c, 2, dz);
                    abi::set_fpr(c, 3, dx * dx);
                    Addr(frame.at(c, 0)).write_fpu(c, dx);
                    abi::set_fpr(c, 4, dy * dy);
                    Addr(frame.at(c, 4)).write_fpu(c, dy);
                    abi::set_fpr(c, 0, dz * dz);
                    Addr(frame.at(c, 8)).write_fpu(c, dz);
                    let sum = find_marker_fpr(c, 3) + find_marker_fpr(c, 4);
                    let sum = sum + find_marker_fpr(c, 0);
                    abi::set_fpr(c, 3, sum);
                    let root = sum.sqrt();
                    abi::set_fpr(c, 0, root);
                    let best_so_far = find_marker_fpr(c, 20);
                    let nearer = abi::c_lt_s(c, root, best_so_far);
                    let entry = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = entry;
                    if nearer {
                        c.cycles += 2;
                        abi::set_fpr(c, 20, root);
                        c.g[S4][0] = Addr(entry as u32).lw(c);
                    }
                    c.cycles += 1;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                }
                c.cycles += 4;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x28));
                if !more {
                    break 'search;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_b6c8);
                }
            }
        }
        c.cycles += 11;
        c.g[V0][0] = c.g[S4][0];
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// `PathData_DebugPrintfStub` (0x26baa8): a debug printf that does nothing
/// but spill its argument registers (a1..a3, t0..t3 as 64-bit words, f12..f18
/// as 32-bit words) to its frame, then return with `v0 = a0`.
pub fn path_data_debug_printf_stub(c: &mut Cpu) {
    c.cycles += 15;
    let frame = Frame::open(c, 0x80);
    c.g[V0][0] = c.g[A0][0];
    for (i, reg) in [A1, A2, A3, T0, T1, T2, T3].into_iter().enumerate() {
        let slot = frame.at(c, 0x48 + 8 * i as u32);
        let v = c.g[reg][0];
        c.sd(slot, v);
    }
    for (i, f) in [12usize, 14, 16, 18].into_iter().enumerate() {
        frame.save_fpr(c, f, 0x38 + 4 * i as u32);
    }
    frame.close(c);
}

/// `PathData_GlobalCtors` (0x26c438): the static-constructor stub, which runs
/// `PathData_StaticInit(1, 0xffff)` (0x26bae8).
pub fn path_data_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        call_ptr(c, 0x0026_bae8, 0x0026_c44c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Path_InitFields` (0x0026_c410): straight-line leaf. Mechanical port of the generated body.
pub fn path_init_fields_0026c410(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `Path_ReadField` (0x0026_c418): straight-line leaf. Mechanical port of the generated body.
pub fn path_read_field_0026c418(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::machine;

    use crate::difftest::fill;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    pub(crate) use crate::game::be::belibrary::tests::SRC;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::be_library_2 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base;

    pub(crate) use crate::game::fe::feasyncfile::tests::w;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3 as OBJ;

    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;

    use ssx3_ee as ee;

    #[test]
    fn path_data_has_marker_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 0x26b598, 0);
            be_library(c);
            let path = DATA + 0x100;
            let count: u32 = match seed % 5 {
                0 => 0,
                1 => 0x8000_0000,
                _ => 5,
            };
            c.bus.write32(path, count);
            c.bus.write32(path + 4, DATA + 0x1000);
            for i in 0..5u32 {
                let e = DATA + 0x1000 + 0x28 * i;
                c.bus.write32(e, i);
                c.bus.write32(e + 4, 100 + i);
            }
            // Match the first marker, the third, or none.
            let (a, b): (u64, u64) = match seed % 3 {
                0 => (100, 0),
                1 => (102, 2),
                _ => (999, 9),
            };
            c.g[A0][0] = sx(path);
            c.g[A1][0] = a;
            c.g[A2][0] = b;
            let mode = schedule(c, &mut r, seed, 0x0026_b598);
            if mode != Events::Off {
                // The walk is at most five entries: make the event due at the
                // first back edge so the event path is really covered.
                c.next_event = c.cycles + (r.next_u32() % 4) as u64;
            }
            mode
        };
        check_events(
            ssx3_ee::s07::f_0026b598__generated,
            path_data_has_marker,
            80,
            &setup,
        );
    }

    #[test]
    fn path_data_find_marker_matches() {
        check(
            ssx3_ee::s07::f_0026b5e0__generated,
            path_data_find_marker,
            150,
            &|c, seed| {
                let mut r = setup(c, seed, 0x26b5e0);
                c.g[A0][0] = DATA as u64;
                let n = seed % 4;
                c.bus.write32(DATA, n);
                c.bus.write32(DATA + 4, DATA + 0x200);
                let ty: u32 = if seed % 2 == 0 { 7 } else { 9 };
                let id = r.next_u32();
                c.g[A1][0] = ty as u64;
                c.g[A2][0] = id as u64;
                for i in 0..n {
                    let e = DATA + 0x200 + i * 0x28;
                    let hit = seed % 3 == 0 && i == n - 1;
                    c.bus.write32(e, if hit { id } else { r.next_u32() });
                    c.bus.write32(e + 4, if hit { ty } else { r.next_u32() });
                    c.bus.write32(e + 8, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn path_data_read_float_int_pairs_matches() {
        check(
            ee::s07::f_0026b7d8__generated,
            path_data_read_float_int_pairs,
            6,
            &|c, seed| {
                let mut r = base(c, seed, 0x26b7d8, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = SRC as u64;
                w(c, SRC, (seed % 4) as u32);
                fill(c, &mut r, SRC + 4, 16);
                w(c, OBJ + 0x18, 0);
                w(c, OBJ + 0x1c, 0);
                w(c, OBJ + 0x20, 0);
                w(c, OBJ + 0x24, 0);
            },
        );
    }

    use crate::difftest::{scramble, RA as SENTINEL};

    /// A tree whose AI path array holds 0..6 entries, asked for a null path,
    /// one of its entries, or a pointer that is not in the array.
    pub(crate) fn path_index_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) {
        scramble(c, r);
        let tree = DATA;
        let entries = DATA + 0x1000;
        let count = match seed % 7 {
            0 => 0,
            1 => (-(1 + (r.next_u32() % 3) as i32)) as u32,
            _ => 1 + r.next_u32() % 6,
        };
        c.bus.write32(tree + PATH_DATA_AI_COUNT, count);
        c.bus.write32(tree + PATH_DATA_AI_ENTRIES, entries);
        let n = if (count as i32) > 0 { count } else { 1 };
        let path = match seed % 5 {
            0 => 0,
            1 => entries + 0x40 * n + 4,
            2 => 0xdead_0000,
            _ => entries + 0x40 * (r.next_u32() % n),
        };
        c.g[A0][0] = sx(tree);
        c.g[A1][0] = sx(path);
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x0026_b550;
    }

    /// `PathData_GetAIPathIndex`: found, not found, null, empty array.
    #[test]
    fn path_data_get_ai_path_index_matches() {
        check(
            ee::s07::f_0026b550__generated,
            path_data_get_ai_path_index,
            300,
            &|c, seed| {
                let mut r = Rng(0x26b5_5000 ^ seed as u64);
                path_index_fixture(c, &mut r, seed);
            },
        );
    }

    /// Event delivery at the search loop head (0x26b570).
    #[test]
    fn path_data_get_ai_path_index_events_match() {
        check_events(
            ee::s07::f_0026b550__generated,
            path_data_get_ai_path_index,
            150,
            &|c, seed| {
                let mut r = Rng(0x26b5_5001 ^ seed as u64);
                path_index_fixture(c, &mut r, 3 + seed % 4);
                // Long array, path near the end (or absent).
                c.bus.write32(DATA + PATH_DATA_AI_COUNT, 6);
                let path = if seed % 2 == 0 {
                    DATA + 0x1000 + 0x40 * 5
                } else {
                    4
                };
                c.g[A1][0] = sx(path);
                schedule(c, &mut r, seed, 0x0026_b550)
            },
        );
    }

    /// A tree whose track path array holds 0..6 entries, asked for a null
    /// path, one of its entries, or a pointer that is not in the array.
    pub(crate) fn track_path_index_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) {
        scramble(c, r);
        let tree = DATA;
        let entries = DATA + 0x1000;
        let count = match seed % 7 {
            0 => 0,
            1 => (-(1 + (r.next_u32() % 3) as i32)) as u32,
            _ => 1 + r.next_u32() % 6,
        };
        c.bus.write32(tree + PATH_DATA_TRACK_COUNT, count);
        c.bus.write32(tree + PATH_DATA_TRACK_ENTRIES, entries);
        let n = if (count as i32) > 0 { count } else { 1 };
        let path = match seed % 5 {
            0 => 0,
            1 => entries + 0x3c * n + 4,
            2 => 0xdead_0000,
            _ => entries + 0x3c * (r.next_u32() % n),
        };
        c.g[A0][0] = sx(tree);
        c.g[A1][0] = sx(path);
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x0026_b508;
    }

    /// `PathData_GetTrackPathIndex`: found, not found, null, empty array.
    #[test]
    fn path_data_get_track_path_index_matches() {
        check(
            ee::s07::f_0026b508__generated,
            path_data_get_track_path_index,
            300,
            &|c, seed| {
                let mut r = Rng(0x26b5_0800 ^ seed as u64);
                track_path_index_fixture(c, &mut r, seed);
            },
        );
    }

    /// Event delivery at the search loop head (0x26b528).
    #[test]
    fn path_data_get_track_path_index_events_match() {
        check_events(
            ee::s07::f_0026b508__generated,
            path_data_get_track_path_index,
            150,
            &|c, seed| {
                let mut r = Rng(0x26b5_0801 ^ seed as u64);
                track_path_index_fixture(c, &mut r, 3 + seed % 4);
                // Long array, path near the end (or absent).
                c.bus.write32(DATA + PATH_DATA_TRACK_COUNT, 6);
                let path = if seed % 2 == 0 {
                    DATA + 0x1000 + 0x3c * 5
                } else {
                    4
                };
                c.g[A1][0] = sx(path);
                schedule(c, &mut r, seed, 0x0026_b508)
            },
        );
    }

    pub(crate) use crate::game::be::belibrary::tests::OC431_BASE;

    /// Table of the three marker kinds the node type is looked up in.
    pub(crate) const OC431_PATH_TYPES: u32 = 0x0044_5428;

    /// `PathData_ReadMarkers`: 0 to 3 nodes read in place from a file image
    /// (count, then id, kind key, position, direction and two indices per
    /// node), the kind key matching table entry 0, 1, 2 or none. The real
    /// `PathData_ReadInt`/`ReadVec3` run.
    #[test]
    fn path_data_read_markers_matches() {
        check(
            ee::s07::f_0026b880__generated,
            path_data_read_markers,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x26b880, 0);
                c.pc = 0x0026_b880;
                let (this, file) = (OC431_BASE + 0x100, OC431_BASE + 0x1000);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = file as u64;
                fill(c, &mut r, this, 8);
                fill(c, &mut r, file, 0x200);
                let n = (seed % 5) as i32 - 1;
                c.bus.write32(file, n as u32);
                let keys = [11u32, 22, 33];
                for (j, k) in keys.iter().enumerate() {
                    c.bus.write32(OC431_PATH_TYPES + 4 * j as u32, *k);
                }
                for i in 0..4u32 {
                    let key = if r.next_u32() % 4 == 0 {
                        77
                    } else {
                        keys[(r.next_u32() % 3) as usize]
                    };
                    // node i starts at 4 + 40 * i: id, key, pos[3], dir[3], a, b
                    c.bus.write32(file + 4 + 40 * i + 4, key);
                    c.bus.write32(file + 4 + 40 * i + 32, r.next_u32() % 4);
                    c.bus.write32(file + 4 + 40 * i + 36, r.next_u32() % 4);
                }
            },
        );
    }

    use crate::difftest::fakes;

    pub(crate) use crate::game::ai::ai::tests::oc434_force;
    pub(crate) use crate::game::ai::ai::tests::oc434_setup;
    pub(crate) use crate::game::ai::ai::tests::oc434_w;

    /// Shared fixture of the two path table readers: a path file with `seed % 4`
    /// empty paths (every 4th..7th seed with one 8-byte item in the first).
    pub(crate) fn oc434_read_paths_setup(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        let _ = oc434_setup(c, seed, salt, 4, entry);
        let (this, data) = (DATA + 0x100, DATA + 0x1000);
        c.g[A0][0] = this as u64;
        c.g[A1][0] = data as u64;
        let count = seed % 4;
        oc434_w(c, data, count);
        if seed % 8 >= 4 && count > 0 {
            oc434_w(c, data + 4, 1);
            oc434_w(c, data + 8, 7);
            oc434_w(c, data + 12, 8);
        }
        oc434_force(c, 0, DATA + 0x4000);
    }

    /// `PathData_ReadTrackPaths`: 0..3 paths (allocation, vtable stores, `Path_Read`
    /// with its virtual calls and unwinds); the count read and `Path_Read` are real.
    #[test]
    fn path_data_read_track_paths_matches() {
        fakes::covered(
            ee::s07::f_0026b310__generated,
            path_data_read_track_paths,
            24,
            &|c, seed| oc434_read_paths_setup(c, seed, 0x26b310, 0x0026_b310),
        );
    }

    /// `PathData_ReadAIPaths`: the same for the AI path table.
    #[test]
    fn path_data_read_ai_paths_matches() {
        fakes::covered(
            ee::s07::f_0026b410__generated,
            path_data_read_ai_paths,
            24,
            &|c, seed| oc434_read_paths_setup(c, seed, 0x26b410, 0x0026_b410),
        );
    }

    use crate::difftest::GAME_GP;

    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    #[test]
    fn path_data_static_init_matches() {
        check(
            ee::s07::f_0026bae8__generated,
            path_data_static_init,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0026_bae8, 0);
                c.pc = 0x0026_bae8;
                // Literal pool words the body loads (gp-relative), random per seed.
                for off in [
                    0x4924, 0x4928, 0x492c, 0x4930, 0x4934, 0x4938, 0x493c, 0x4940, 0x4944, 0x4948,
                    0x494c, 0x4950, 0x4954, 0x4958, 0x495c, 0x4960, 0x4964, 0x4968, 0x496c, 0x4970,
                    0x4974, 0x4978, 0x497c, 0x4980, 0x4984, 0x4988, 0x498c, 0x4990, 0x4994, 0x4998,
                    0x499c, 0x49a0, 0x49a4, 0x49a8, 0x49ac, 0x49b0, 0x49b4, 0x49b8, 0x49bc, 0x49c0,
                    0x49c4, 0x49c8, 0x49cc, 0x49d0, 0x49d4, 0x49d8, 0x49dc, 0x49e0, 0x49e4, 0x49e8,
                    0x49ec, 0x49f0, 0x49f4, 0x49f8, 0x49fc, 0x4a00, 0x4a04, 0x4a08, 0x4a0c, 0x4a10,
                    0x4a14, 0x4a18, 0x4a1c, 0x4a20, 0x4a24, 0x4a28, 0x4a2c, 0x4a30, 0x4a34, 0x4a38,
                ] {
                    c.bus.write32(GAME_GP.wrapping_sub(off), r.next_u32());
                }
                // Branches: A1 must equal 0xFFFF, and A0 (the object) must be non-null.
                c.g[A1][0] = if seed % 4 == 0 { 0x1234 } else { 0xffff };
                c.g[A0][0] = if seed % 5 == 0 { 0 } else { sx(DATA) };
            },
        );
    }

    /// Gate word for the nearest-marker search's callee fakes.
    pub(crate) const PATH_FIND_NEAREST_GATE: u32 = DATA + 0x200000;

    /// The `PathTable` the search walks (count at +0, entries at +4).
    pub(crate) const PATH_FIND_NEAREST_LIST: u32 = DATA + 0x10000;

    pub(crate) static OC531_PREV_GET_BE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC531_PREV_GET_IF: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC531_PREV_SESSION: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn oc531_fake_get_be(c: &mut Cpu) {
        if c.bus.read32(PATH_FIND_NEAREST_GATE) != 0x0531_0002 {
            return fakes::fall_back(c, &OC531_PREV_GET_BE, ee::s01::f_0014dc80__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0014_dc80, ra);
    }

    pub(crate) fn oc531_fake_get_if(c: &mut Cpu) {
        if c.bus.read32(PATH_FIND_NEAREST_GATE) != 0x0531_0002 {
            return fakes::fall_back(c, &OC531_PREV_GET_IF, ee::s01::f_0014dd58__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0014_dd58, ra);
    }

    /// `isSessionPoint`: zero or non-zero from the entry id and a seed word.
    pub(crate) fn oc531_fake_session(c: &mut Cpu) {
        if c.bus.read32(PATH_FIND_NEAREST_GATE) != 0x0531_0002 {
            return fakes::fall_back(c, &OC531_PREV_SESSION, ee::s01::f_00154630__generated);
        }
        let id = c.g[A1][0] as u32;
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0015_4630, ra);
        let salt = c.bus.read32(PATH_FIND_NEAREST_GATE + 4);
        c.g[V0][0] = if (id ^ salt) % 3 == 0 { 0 } else { 1 };
    }

    #[test]
    fn path_data_find_nearest_marker_matches() {
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc531_fake_get_be,
            &OC531_PREV_GET_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc531_fake_get_if,
            &OC531_PREV_GET_IF,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00154630),
            oc531_fake_session,
            &OC531_PREV_SESSION,
        );
        check(
            ee::s07::f_0026b680__generated,
            path_data_find_nearest_marker,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0026_b680, 4);
                c.bus.write32(PATH_FIND_NEAREST_GATE, 0x0531_0002);
                c.bus.write32(PATH_FIND_NEAREST_GATE + 4, r.next_u32());
                // The world pointer chain the search reads: gp-0x848 -> +0x84
                // -> +0x78 -> +0x1bc.
                let world = DATA + 0x1000;
                let section = DATA + 0x2000;
                let mission = DATA + 0x3000;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, section);
                c.bus.write32(section + 0x78, mission);
                c.bus.write32(mission + 0x1bc, r.next_u32() % 9);
                let best = if seed % 4 == 0 {
                    f32::MAX.to_bits()
                } else {
                    r.f32(40.0)
                };
                c.bus.write32(GAME_GP - 0x4a3c, best);
                let list = PATH_FIND_NEAREST_LIST;
                let entries = list + 0x100;
                let count = seed % 5;
                c.bus.write32(list, count);
                c.bus.write32(list + 4, entries);
                for i in 0..count {
                    let e = entries + 0x28 * i;
                    let id = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 7
                    };
                    c.bus.write32(e, id);
                    c.bus
                        .write32(e + 4, if r.next_u32() % 4 == 0 { 1 } else { 2 });
                    for k in 0..3 {
                        c.bus.write32(e + 8 + 4 * k, r.f32(40.0));
                    }
                }
                let pos = DATA + 0x20000;
                for k in 0..3 {
                    c.bus.write32(pos + 4 * k, r.f32(40.0));
                }
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = sx(list);
                c.g[A1][0] = sx(pos);
                c.pc = 0x0026_b680;
            },
        );
    }

    #[test]
    fn path_data_debug_printf_stub_matches() {
        check(
            ee::s07::f_0026baa8__generated,
            path_data_debug_printf_stub,
            100,
            &|c, seed| {
                let mut r = Rng(0x0026_baa8 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0026_baa8;
            },
        );
    }

    #[test]
    fn path_data_global_ctors_matches() {
        check(
            ee::s07::f_0026c438__generated,
            path_data_global_ctors,
            20,
            &|c, seed| {
                let mut r = Rng(0x0026_c438 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0026_c438;
            },
        );
    }

    /// `Path_InitFields` (0x0026_c410) against the generated body.
    #[test]
    fn path_init_fields_0026c410_matches() {
        check(
            ee::s07::f_0026c410__generated,
            path_init_fields_0026c410,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x26c410, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Path_ReadField` (0x0026_c418) against the generated body.
    #[test]
    fn path_read_field_0026c418_matches() {
        check(
            ee::s07::f_0026c418__generated,
            path_read_field_0026c418,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x26c418, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
