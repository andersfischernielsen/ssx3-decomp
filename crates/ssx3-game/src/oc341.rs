//! Staging module oc341 (wave wave215): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc341`.

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
    (0x0010_1310, danger_man_ctor, "DangerMan_ctor"),
    (0x0010_1688, danger_man_reset, "DangerMan_Reset"),
    (0x0010_1728, danger_man_add_danger, "DangerMan_AddDanger"),
    (
        0x0010_1888,
        danger_man_remove_danger,
        "DangerMan_RemoveDanger",
    ),
    (
        0x0010_1900,
        danger_man_acquire_zone,
        "DangerMan_AcquireZone",
    ),
    (
        0x0010_1970,
        danger_man_release_zone,
        "DangerMan_ReleaseZone",
    ),
    (
        0x0010_3308,
        focus_man_remove_section_objects,
        "FocusMan_RemoveSectionObjects",
    ),
    (0x0010_3358, focus_man_reset, "FocusMan_Reset"),
    (
        0x0010_33f8,
        focus_man_remove_focus_box,
        "FocusMan_RemoveFocusBox",
    ),
    (0x0010_3578, focus_man_load_state, "FocusMan_LoadState"),
];

// ---------------------------------------------------------------------------
// Danger manager (`cDangerMan`, all_in_one_ai.cpp).
//
// Layout: `+0` the AI/world, `+4` 64 danger sources of 0x1c bytes (`+8`
// state: 0 live, 1 timed, 2 free; `+0x18` object handle), `+0x704` live
// count, `+0x708`, `+0x70c` six rider slots of 12 bytes, `+0x754` a pool of
// 64 zone nodes of 0x10 bytes (`+0` handle, `+4` danger, `+8` references,
// `+0xc` next), `+0xb54` the free zone list, `+0xb58` the active zone list,
// `+0xb5c` the watched rider.

/// Danger sources: first at `+4`, 0x1c bytes each.
pub(crate) const DANGER_MAN_SOURCES: u32 = 0x4;
/// Live danger source count.
pub(crate) const DANGER_MAN_ACTIVE: u32 = 0x704;
/// Six rider slots of 12 bytes.
pub(crate) const DANGER_MAN_RIDERS: u32 = 0x70c;
/// The zone node pool (64 nodes of 0x10 bytes).
pub(crate) const DANGER_MAN_ZONE_POOL: u32 = 0x754;
/// Free zone nodes.
pub(crate) const DANGER_MAN_FREE_ZONES: u32 = 0xb54;
/// Zones in use.
pub(crate) const DANGER_MAN_ZONES: u32 = 0xb58;
/// The rider the zone list is checked against.
pub(crate) const DANGER_MAN_WATCH_RIDER: u32 = 0xb5c;

/// `DangerMan_Reset` (0x101688), through its slot.
const OC341_DANGER_MAN_RESET: Callee = Callee {
    addr: 0x0010_1688,
    f: ee::s00::DangerMan_Reset,
};

/// `DangerMan_RemoveDanger` (0x101888), through its slot.
const OC341_DANGER_MAN_REMOVE_DANGER: Callee = Callee {
    addr: 0x0010_1888,
    f: ee::s00::DangerMan_RemoveDanger,
};

/// `cDangerMan *cDangerMan::cDangerMan(cAI *ai)` (0x101310): every danger
/// source's handle (+0x18) and every zone node's handle = -1, `+0` = `ai`,
/// then `Reset()`; returns `this`.
pub fn danger_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = u64::MAX;
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        // Sources' handles.
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(DANGER_MAN_SOURCES));
        c.g[A0][0] = 0x3f;
        c.g[A2][0] = u64::MAX;
        loop {
            c.cycles += 7;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
            let more = c.g[A0][0] != c.g[A2][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1c));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0010_1338);
            }
        }
        // Zone nodes' handles.
        c.cycles += 5;
        c.g[V1][0] = u64::MAX;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(DANGER_MAN_ZONE_POOL));
        c.g[A0][0] = 0x3f;
        c.g[A2][0] = u64::MAX;
        loop {
            c.cycles += 7;
            c.sw(c.g[V0][0] as u32, c.g[V1][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
            let more = c.g[A0][0] != c.g[A2][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0010_1368);
            }
        }
        c.cycles += 8;
        c.sw(c.g[S0][0] as u32, c.g[A1][0]);
        c.g[A0][0] = c.g[S0][0];
        OC341_DANGER_MAN_RESET.call(c, 0x0010_1390)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void cDangerMan::Reset()` (0x101688): no live sources, every source free
/// (state 2), the six rider slots cleared, the 64 zone nodes chained into the
/// free list (`+0xb54`), no zones in use, watched rider 0.
pub fn danger_man_reset(c: &mut Cpu) {
    let man = c.g[A0][0] as u32;
    c.cycles += 4;
    c.sw(man.wrapping_add(DANGER_MAN_ACTIVE), 0);
    // Every source's state = 2 (free), from the last one down.
    c.g[A1][0] = 2;
    c.g[V1][0] = 0x3f;
    c.g[V0][0] = sx(man.wrapping_add(0x6f0));
    loop {
        c.cycles += 7;
        c.sw(c.g[V0][0] as u32, c.g[A1][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1c));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0010_1698);
        }
    }
    // Clear the rider slots' first two words.
    c.cycles += 5;
    c.g[A3][0] = sx(man.wrapping_add(0x760));
    c.g[T0][0] = sx(man.wrapping_add(DANGER_MAN_ZONE_POOL));
    c.g[V1][0] = sx(man.wrapping_add(0x710));
    c.g[A1][0] = sx(man.wrapping_add(DANGER_MAN_RIDERS));
    c.g[A2][0] = sx(man.wrapping_add(0x758));
    loop {
        c.cycles += 7;
        c.sw(c.g[A1][0] as u32, 0);
        c.sw(c.g[V1][0] as u32, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc));
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0010_16c8);
        }
    }
    // Chain the zone nodes: node[i].next = node[i + 1], from the top down.
    c.cycles += 3;
    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x3e0));
    c.g[A1][0] = 0x3e;
    c.g[V0][0] = sx(man.wrapping_add(0xb44));
    loop {
        c.cycles += 7;
        c.sw(c.g[V1][0] as u32, c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x10));
        let more = (c.g[A1][0] as i64) >= 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0010_16f0);
        }
    }
    c.cycles += 6;
    c.sw(man.wrapping_add(DANGER_MAN_FREE_ZONES), c.g[T0][0]);
    c.sw(man.wrapping_add(0x708), 0);
    c.sw(man.wrapping_add(0xb50), 0);
    c.sw(man.wrapping_add(DANGER_MAN_ZONES), 0);
    c.sw(man.wrapping_add(DANGER_MAN_WATCH_RIDER), 0);
    ret(c);
}

/// `void DangerMan_AddDanger(cDangerMan *man, u32 *handle, float f12, float
/// f13)` (0x101728): look the object up by handle (`gp+0x16c8` tables); if it
/// exists and is not gone (its `+0xc` part's vtable `+0x88` answers 0),
/// remove any source for the handle, then take the first free source: state
/// 0, `+0`/`+4` = f12/f13, `+0x18` = the handle, `+0xc..+0x14` = the
/// position (vtable `+0xc0` matrix, row 3), and count it live.
pub fn danger_man_add_danger(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x70);
        let gp = c.g[GP][0] as u32;
        c.g[V0][0] = c.lw(gp.wrapping_add(0x16c8));
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save_fpr(c, 21, 0x68);
        c.g[S3][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x60);
        c.mov_s(21, 13);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save_ra(c, 0x10);
        // The object: `types[h & 0xff]->objects[h >> 8]` (word >> 8 << 2).
        c.g[A0][0] = c.lw(c.g[V0][0] as u32);
        c.g[A1][0] = c.lw(c.g[S3][0] as u32);
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(8));
        c.g[V0][0] = c.g[A1][0] & 0xff;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.mov_s(20, 12);
        let mut found = false;
        if c.g[V1][0] != 0 {
            c.cycles += 8;
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
            c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
            let gone = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            if !gone {
                c.cycles += 2;
                c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                found = true;
            }
        }
        if !found {
            // A null object: its `+0xc` read from address 0xc.
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.g[S1][0] = c.lw(0xc);
        }
        c.cycles += 2;
        frame.restore(c, S0, 0x50);
        'out: {
            if c.g[S1][0] == 0 {
                break 'out;
            }
            // Is the object gone? (vtable +0x88)
            c.cycles += 7;
            let part = c.g[S1][0] as u32;
            c.g[V1][0] = c.lw(part.wrapping_add(0xc));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x88));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
            let method = c.g[V0][0] as u32;
            c.g[A0][0] = sx(part.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, method, 0x0010_1820)?;
            let gone = c.g[V0][0] != 0;
            frame.restore(c, S0, 0x50);
            if gone {
                break 'out;
            }
            c.cycles += 8;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S3][0];
            OC341_DANGER_MAN_REMOVE_DANGER.call(c, 0x0010_1834)?;
            // The first free source (state 2).
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(DANGER_MAN_SOURCES));
            c.g[A1][0] = 2;
            c.g[A0][0] = 0;
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
            loop {
                c.cycles += 3;
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                let free = c.g[V0][0] == c.g[A1][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1c));
                if free {
                    if event_due(c) {
                        deliver_event_here(c, 0x0010_17a4);
                    }
                    c.cycles += 22;
                    let src = c.g[S0][0] as u32;
                    c.sw(src.wrapping_add(8), 0);
                    c.g[V0][0] = c.lw(c.g[S3][0] as u32);
                    c.swc1(20, src);
                    c.sw(src.wrapping_add(0x18), c.g[V0][0]);
                    c.swc1(21, src.wrapping_add(4));
                    let part = c.g[S1][0] as u32;
                    c.g[V1][0] = c.lw(part.wrapping_add(0xc));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                    let method = c.g[V0][0] as u32;
                    c.g[A0][0] = sx(part.wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, method, 0x0010_17cc)?;
                    // Position: the matrix's row 3.
                    let stack = sp(c);
                    c.lq(V1, (c.g[V0][0] as u32).wrapping_add(0x30));
                    c.sq(V1, stack);
                    c.lwc1(1, stack);
                    c.lwc1(2, stack.wrapping_add(4));
                    c.lwc1(0, stack.wrapping_add(8));
                    let src = c.g[S0][0] as u32;
                    c.swc1(1, src.wrapping_add(0xc));
                    c.swc1(0, src.wrapping_add(0x14));
                    c.swc1(2, src.wrapping_add(0x10));
                    let man = c.g[S2][0] as u32;
                    c.g[V0][0] = c.lw(man.wrapping_add(DANGER_MAN_ACTIVE));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                    c.sw(man.wrapping_add(DANGER_MAN_ACTIVE), c.g[V0][0]);
                    break;
                }
                c.cycles += 4;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[A0][0] as i64) < 64) as u64;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0010_1848);
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void DangerMan_RemoveDanger(cDangerMan *man, u32 *handle)` (0x101888):
/// walk the sources (at most the live count of non-free ones) and release
/// (`DangerMan_ReleaseDanger`) the first whose handle (+0x18) is `*handle`.
pub fn danger_man_remove_danger(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let man = c.g[A0][0] as u32;
        c.g[V0][0] = c.lw(man.wrapping_add(DANGER_MAN_ACTIVE));
        c.g[T0][0] = c.g[A1][0];
        'not_found: {
            if c.g[V0][0] == 0 {
                break 'not_found;
            }
            c.cycles += 5;
            c.g[A2][0] = c.g[V0][0];
            c.g[A3][0] = 0;
            c.g[T1][0] = 2;
            c.g[A1][0] = sx(man.wrapping_add(DANGER_MAN_SOURCES));
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(8));
            loop {
                c.cycles += 2;
                if c.g[V0][0] == c.g[T1][0] {
                    // Free: skipped, not counted.
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
                    c.g[V0][0] = c.lw(c.g[T0][0] as u32);
                    let other = c.g[V1][0] != c.g[V0][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                    if !other {
                        c.cycles += 4;
                        call_ptr(c, 0x0014_41a0, 0x0010_18d0)?;
                        frame.restore_ra(c, 0);
                        c.cycles += 2;
                        frame.close(c);
                        return Ok(());
                    }
                    c.cycles += 1;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A3][0] as i64) < 64) as u64;
                let end = c.g[V0][0] == 0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1c));
                if end {
                    break 'not_found;
                }
                c.cycles += 2;
                if c.g[A2][0] == 0 {
                    break 'not_found;
                }
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(8));
                if event_due(c) {
                    deliver_event_here(c, 0x0010_18b0);
                }
            }
        }
        c.cycles += 1;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void DangerMan_AcquireZone(cDangerMan *man, u32 *handle, int danger)`
/// (0x101900): a zone already in use for `*handle` gets one more reference;
/// otherwise a node from the free list becomes `{*handle, danger, 1}` at the
/// head of the zones in use.
pub fn danger_man_acquire_zone(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A3][0] = c.g[A0][0];
    c.g[A0][0] = c.g[A1][0];
    let man = c.g[A3][0] as u32;
    c.g[A1][0] = c.lw(man.wrapping_add(DANGER_MAN_ZONES));
    if c.g[A1][0] != 0 {
        c.cycles += 1;
        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
            if c.g[V1][0] == c.g[V0][0] {
                // In use: one more reference.
                c.cycles += 4;
                let node = c.g[A1][0] as u32;
                c.g[V0][0] = c.lw(node.wrapping_add(8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                c.sw(node.wrapping_add(8), c.g[V0][0]);
                ret(c);
                return;
            }
            c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
            c.cycles += 2;
            if c.g[A1][0] == 0 {
                c.cycles += 1;
                break;
            }
            c.g[V1][0] = c.lw(c.g[A1][0] as u32);
            if event_due(c) {
                deliver_event_here(c, 0x0010_1918);
            }
        }
    }
    // A new zone from the free list.
    c.g[V1][0] = c.lw(man.wrapping_add(DANGER_MAN_FREE_ZONES));
    c.cycles += 11;
    c.g[A1][0] = 1;
    let node = c.g[V1][0] as u32;
    c.g[V0][0] = c.lw(node.wrapping_add(0xc));
    c.sw(man.wrapping_add(DANGER_MAN_FREE_ZONES), c.g[V0][0]);
    c.g[A0][0] = c.lw(c.g[A0][0] as u32);
    c.sw(node.wrapping_add(4), c.g[A2][0]);
    c.sw(node, c.g[A0][0]);
    c.sw(node.wrapping_add(8), c.g[A1][0]);
    c.g[V0][0] = c.lw(man.wrapping_add(DANGER_MAN_ZONES));
    c.sw(node.wrapping_add(0xc), c.g[V0][0]);
    c.sw(man.wrapping_add(DANGER_MAN_ZONES), node as u64);
    ret(c);
}

/// `void DangerMan_ReleaseZone(cDangerMan *man, u32 *handle)` (0x101970):
/// drop one reference of the zone in use for `*handle`; at zero the node goes
/// back to the free list, and when no zone is left in use the six rider
/// slots' first two words are cleared.
pub fn danger_man_release_zone(c: &mut Cpu) {
    c.cycles += 3;
    let man = c.g[A0][0] as u32;
    c.g[A2][0] = c.lw(man.wrapping_add(DANGER_MAN_ZONES));
    c.g[A3][0] = 0;
    'done: {
        if c.g[A2][0] == 0 {
            break 'done;
        }
        c.cycles += 3;
        c.g[T1][0] = sx(man.wrapping_add(DANGER_MAN_RIDERS));
        c.g[T0][0] = sx(man.wrapping_add(0x710));
        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(c.g[A1][0] as u32);
            if c.g[V1][0] != c.g[V0][0] {
                c.g[A3][0] = c.g[A2][0];
                c.cycles += 3;
                c.g[A2][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                if c.g[A2][0] == 0 {
                    break 'done;
                }
                c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                if event_due(c) {
                    deliver_event_here(c, 0x0010_1988);
                }
                continue;
            }
            c.cycles += 4;
            let node = c.g[A2][0] as u32;
            c.g[V0][0] = c.lw(node.wrapping_add(8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            c.sw(node.wrapping_add(8), c.g[V0][0]);
            if c.g[V0][0] != 0 {
                break 'done;
            }
            // Unlink it and put it on the free list.
            c.cycles += 2;
            c.g[V0][0] = c.lw(node.wrapping_add(0xc));
            if c.g[A3][0] != 0 {
                c.cycles += 2;
                c.sw((c.g[A3][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            } else {
                c.cycles += 1;
                c.sw(man.wrapping_add(DANGER_MAN_ZONES), c.g[V0][0]);
            }
            c.cycles += 6;
            c.g[V0][0] = c.lw(man.wrapping_add(DANGER_MAN_FREE_ZONES));
            c.sw(node.wrapping_add(0xc), c.g[V0][0]);
            c.sw(man.wrapping_add(DANGER_MAN_FREE_ZONES), c.g[A2][0]);
            c.g[V0][0] = c.lw(man.wrapping_add(DANGER_MAN_ZONES));
            c.g[V1][0] = c.g[T0][0];
            if c.g[V0][0] != 0 {
                break 'done;
            }
            // No zone left: clear the rider slots.
            c.cycles += 2;
            c.g[A0][0] = c.g[T1][0];
            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x48));
            loop {
                c.cycles += 7;
                c.sw(c.g[A0][0] as u32, 0);
                c.sw(c.g[V1][0] as u32, 0);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0010_19d8);
                }
            }
            break 'done;
        }
    }
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// Focus manager (`cFocusMan`).
//
// Layout: `+0` focus box count, box i at `+0x20 + 0x20*i` (0x10 bytes) with
// its owner at `+0x10 + 0x20*i`, `+0xd0` current focus (-1 none), `+0xd4`
// loaded flag, `+0xd8` object count, `+0xdc` object pointers.

/// Object count.
pub(crate) const FOCUS_MAN_OBJECT_COUNT: u32 = 0xd8;
/// Object pointers.
pub(crate) const FOCUS_MAN_OBJECTS: u32 = 0xdc;
/// The current focus (-1: none).
pub(crate) const FOCUS_MAN_CURRENT: u32 = 0xd0;

/// `void FocusMan_RemoveSectionObjects(cFocusMan *fm, int section)`
/// (0x103308): keep only the objects whose section byte (+0x78) is not
/// `section`, compacting the list in place.
pub fn focus_man_remove_section_objects(c: &mut Cpu) {
    let fm = c.g[A0][0] as u32;
    c.cycles += 4;
    c.g[V0][0] = c.lw(fm.wrapping_add(FOCUS_MAN_OBJECT_COUNT));
    c.g[T1][0] = 0;
    c.g[T0][0] = 0;
    if (c.g[V0][0] as i64) > 0 {
        c.cycles += 2;
        c.g[A2][0] = sx(fm.wrapping_add(FOCUS_MAN_OBJECTS));
        c.g[A3][0] = c.g[A2][0];
        loop {
            c.cycles += 4;
            c.g[V1][0] = c.lw(c.g[A2][0] as u32);
            c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x78));
            if c.g[V0][0] != c.g[A1][0] {
                // Kept.
                c.cycles += 4;
                c.sw(c.g[A3][0] as u32, c.g[V1][0]);
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(4));
            }
            c.g[V0][0] = c.lw(fm.wrapping_add(FOCUS_MAN_OBJECT_COUNT));
            c.cycles += 4;
            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(4));
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0010_3320);
            }
        }
    }
    c.cycles += 2;
    c.sw(fm.wrapping_add(FOCUS_MAN_OBJECT_COUNT), c.g[T0][0]);
    ret(c);
}

/// `void cFocusMan::reset()` (0x103358): clear flag 0x100 of every object
/// (+0x8), empty the object list and set the current focus to -1.
pub fn focus_man_reset(c: &mut Cpu) {
    c.cycles += 4;
    c.g[T0][0] = c.g[A0][0];
    let fm = c.g[T0][0] as u32;
    c.g[V0][0] = c.lw(fm.wrapping_add(FOCUS_MAN_OBJECT_COUNT));
    c.g[A2][0] = 0;
    if (c.g[V0][0] as i64) > 0 {
        c.cycles += 4;
        c.g[A3][0] = 0xffff_ffff_ffff_feff;
        c.g[A1][0] = sx(fm.wrapping_add(FOCUS_MAN_OBJECTS));
        loop {
            c.cycles += 9;
            c.g[V0][0] = c.lw(c.g[A1][0] as u32);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            let obj = c.g[V0][0] as u32;
            c.g[V1][0] = c.lw(obj.wrapping_add(8));
            c.g[V1][0] &= c.g[A3][0];
            c.sw(obj.wrapping_add(8), c.g[V1][0]);
            c.g[A0][0] = c.lw(fm.wrapping_add(FOCUS_MAN_OBJECT_COUNT));
            c.g[A0][0] = ((c.g[A2][0] as i64) < (c.g[A0][0] as i64)) as u64;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
            if c.g[A0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0010_3378);
            }
        }
    }
    c.cycles += 4;
    c.g[V0][0] = u64::MAX;
    c.sw(fm.wrapping_add(FOCUS_MAN_OBJECT_COUNT), 0);
    c.sw(fm.wrapping_add(FOCUS_MAN_CURRENT), c.g[V0][0]);
    ret(c);
}

/// `void FocusMan_RemoveFocusBox(cFocusMan *fm, void *owner)` (0x1033f8):
/// current focus = -1; remove the first focus box whose owner (+0x10 of its
/// 0x20-byte entry) is `owner`, moving the later ones down, and decrement the
/// count.
pub fn focus_man_remove_focus_box(c: &mut Cpu) {
    let fm = c.g[A0][0] as u32;
    c.cycles += 5;
    c.g[V0][0] = u64::MAX;
    c.g[A3][0] = c.lw(fm);
    c.sw(fm.wrapping_add(FOCUS_MAN_CURRENT), c.g[V0][0]);
    c.g[A2][0] = 0;
    if (c.g[A3][0] as i64) > 0 {
        c.cycles += 1;
        c.g[V1][0] = c.g[A0][0];
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
            if c.g[V0][0] == c.g[A1][0] {
                // Found at index a2: shift the rest down.
                c.cycles += 4;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[A3][0] as i64)) as u64;
                let last = c.g[V0][0] == 0;
                c.g[V0][0] = sx((c.g[A2][0] as u32) << 5);
                if !last {
                    c.cycles += 1;
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(fm));
                    loop {
                        c.cycles += 9;
                        let entry = c.g[A1][0] as u32;
                        c.lq(V0, entry.wrapping_add(0x20));
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                        c.lq(V1, entry.wrapping_add(0x10));
                        c.sq(V0, entry);
                        c.sq(V1, entry.wrapping_sub(0x10));
                        c.g[V0][0] = c.lw(fm);
                        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        c.g[A1][0] = sx(entry.wrapping_add(0x20));
                        if c.g[V0][0] == 0 {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0010_3430);
                        }
                    }
                }
                c.cycles += 4;
                c.g[V0][0] = c.lw(fm);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                c.sw(fm, c.g[V0][0]);
                ret(c);
                return;
            }
            c.g[A3][0] = c.lw(fm);
            c.cycles += 4;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[A3][0] as i64)) as u64;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0010_3410);
            }
        }
    }
    c.cycles += 2;
    ret(c);
}

/// One `cStream` virtual call of FocusMan_LoadState: the stream is in s2,
/// `this` adjusted by the halfword at `vt + delta_off`, the method at
/// `vt + delta_off + 4`; the vtable in `vt_reg`, the method in `fn_reg`.
fn focus_man_load_vcall(
    c: &mut Cpu,
    vt_reg: usize,
    fn_reg: usize,
    delta_off: u32,
    ret_to: u32,
) -> crate::call::Flow {
    let vt = c.g[vt_reg][0] as u32;
    c.g[A0][0] = c.lh(vt.wrapping_add(delta_off));
    c.g[fn_reg][0] = c.lw(vt.wrapping_add(delta_off + 4));
    let method = c.g[fn_reg][0] as u32;
    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, ret_to)
}

/// `void FocusMan_LoadState(cFocusMan *fm, cStream *s)` (0x103578): reads
/// the current focus (+0xd0), the boxes (0x10 bytes each, stride 0x20; the
/// count at +0 is already set), sets the loaded flag (+0xd4) to 1, reads the
/// object count (+0xd8) and then each object (`s->ReadObject()`), whose
/// flags (+0x8) get 0x100 set and 0x200 cleared.
pub fn focus_man_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x60);
        c.g[A2][0] = 4;
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x50);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S4, 0x10);
        c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(FOCUS_MAN_CURRENT));
        frame.save_ra(c, 0);
        // s->Read(&fm->current, 4)
        c.g[V0][0] = c.lw(c.g[S2][0] as u32);
        focus_man_load_vcall(c, V0, V1, 0x10, 0x0010_35bc)?;
        c.g[V0][0] = c.lw(c.g[S3][0] as u32);
        let any = (c.g[V0][0] as i64) > 0;
        c.g[V0][0] = 1;
        if any {
            c.cycles += 2;
            c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x20));
            c.g[V0][0] = c.lw(c.g[S2][0] as u32);
            loop {
                // s->Read(&fm->boxes[i], 0x10)
                c.cycles += 12;
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = 0x10;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20));
                focus_man_load_vcall(c, V0, V1, 0x10, 0x0010_35f0)?;
                c.g[V0][0] = c.lw(c.g[S3][0] as u32);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                    break;
                }
                c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                if event_due(c) {
                    deliver_event_here(c, 0x0010_35d0);
                }
            }
        }
        // fm->loaded = 1; s->Read(&fm->object_count, 4)
        c.cycles += 12;
        c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(FOCUS_MAN_OBJECT_COUNT));
        c.sw((c.g[S3][0] as u32).wrapping_add(0xd4), c.g[V0][0]);
        c.g[A2][0] = 4;
        c.g[S4][0] = 0;
        c.g[V0][0] = c.lw(c.g[S2][0] as u32);
        focus_man_load_vcall(c, V0, V1, 0x10, 0x0010_3628)?;
        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(FOCUS_MAN_OBJECT_COUNT));
        let any = (c.g[V0][0] as i64) > 0;
        c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(FOCUS_MAN_OBJECTS));
        if any {
            c.cycles += 3;
            c.g[S1][0] = 0xffff_ffff_ffff_fdff;
            loop {
                // fm->objects[j] = s->ReadObject(), flags |= 0x100, &= ~0x200
                c.cycles += 15;
                c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
                focus_man_load_vcall(c, V1, V0, 0x18, 0x0010_3658)?;
                let obj = c.g[V0][0] as u32;
                c.g[V1][0] = c.lw(obj.wrapping_add(8));
                c.g[V1][0] |= 0x100;
                c.g[V1][0] &= c.g[S1][0];
                c.sw(obj.wrapping_add(8), c.g[V1][0]);
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(FOCUS_MAN_OBJECT_COUNT));
                c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0010_3640);
                }
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine, FAKE};
    use crate::difftest::{check, fill, scramble, Rng, DATA, GAME_GP};

    /// The danger manager of the DangerMan tests.
    const OC341_MAN: u32 = DATA;

    #[test]
    fn danger_man_ctor_matches() {
        check(
            ee::s00::f_00101310__generated,
            danger_man_ctor,
            100,
            &|c, seed| {
                let mut r = Rng(0x0c34_1001 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0010_1310;
                fill(c, &mut r, OC341_MAN, 0xb60 / 4);
                c.g[A0][0] = sx(OC341_MAN);
                c.g[A1][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn danger_man_reset_matches() {
        check(
            ee::s00::f_00101688__generated,
            danger_man_reset,
            100,
            &|c, seed| {
                let mut r = Rng(0x0c34_1002 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0010_1688;
                fill(c, &mut r, OC341_MAN, 0xb60 / 4);
                c.g[A0][0] = sx(OC341_MAN);
            },
        );
    }

    /// Danger sources for RemoveDanger/AddDanger: states 0..2 (some seeds
    /// with no free source), handles that match `HANDLE` or not.
    fn oc341_danger_sources(c: &mut Cpu, r: &mut Rng, seed: u32, handle: u32) {
        let none_free = seed % 6 == 0;
        for i in 0..64u32 {
            let src = OC341_MAN + DANGER_MAN_SOURCES + 0x1c * i;
            let state = if none_free {
                r.next_u32() % 2
            } else if i < 10 {
                r.next_u32() % 3
            } else {
                1 + r.next_u32() % 2
            };
            c.bus.write32(src + 8, state);
            let h = if r.next_u32() % 4 == 0 {
                handle
            } else {
                handle ^ (1 + r.next_u32() % 7)
            };
            c.bus.write32(src + 0x18, h);
        }
        let active = [0u32, 1, 2, 5, 64][(r.next_u32() % 5) as usize];
        c.bus.write32(OC341_MAN + DANGER_MAN_ACTIVE, active);
    }

    #[test]
    fn danger_man_remove_danger_matches() {
        const HANDLE_AT: u32 = DATA + 0x1000;
        check(
            ee::s00::f_00101888__generated,
            danger_man_remove_danger,
            300,
            &|c, seed| {
                let mut r = Rng(0x0c34_1004 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0010_1888;
                let handle = r.next_u32();
                c.bus.write32(HANDLE_AT, handle);
                oc341_danger_sources(c, &mut r, seed, handle);
                c.g[A0][0] = sx(OC341_MAN);
                c.g[A1][0] = sx(HANDLE_AT);
            },
        );
    }

    // ------------------------------------------------- DangerMan_AddDanger

    const OC341_ADD_HANDLE: u32 = DATA + 0x1000;
    const OC341_ADD_HANDLES: u32 = DATA + 0x2000;
    const OC341_ADD_TYPES: u32 = DATA + 0x2200;
    const OC341_ADD_TYPE_ENTRIES: u32 = DATA + 0x2800;
    const OC341_ADD_OBJECTS: u32 = DATA + 0x3000;
    const OC341_ADD_OBJ: u32 = DATA + 0x4000;
    const OC341_ADD_PART: u32 = DATA + 0x5000;
    const OC341_ADD_PART_VT: u32 = DATA + 0x6000;
    /// A fake method answering 0 (the object is not gone).
    const OC341_FAKE_ZERO: u32 = FAKE + 0x100;

    /// The shared fake, answering 0 for [`OC341_FAKE_ZERO`].
    fn oc341_zero_hook(c: &mut Cpu, target: u32) {
        let ret_to = c.g[RA][0] as u32;
        fakes::fake(c, target, ret_to);
        if c.pc == ret_to && target == OC341_FAKE_ZERO {
            c.g[V0][0] = 0;
        }
    }

    /// `DangerMan_AddDanger`: handles of types 0..3 (type 3 has no table)
    /// and objects 0..3 (object 3 is gone), objects with or without a part,
    /// parts alive or gone, sources with or without a free one; fakes unwind
    /// at random calls.
    #[test]
    fn danger_man_add_danger_matches() {
        covered(
            ee::s00::f_00101728__generated,
            danger_man_add_danger,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_1728, 2);
                c.hooks.miss = oc341_zero_hook;
                c.pc = 0x0010_1728;
                // A null object's part is read from address 0xc.
                c.bus.write32(0xc, 0);
                let handle = (r.next_u32() % 4) << 8 | r.next_u32() % 4;
                c.bus.write32(OC341_ADD_HANDLE, handle);
                c.bus.write32(GAME_GP + 0x16c8, OC341_ADD_HANDLES);
                c.bus.write32(OC341_ADD_HANDLES, OC341_ADD_HANDLES + 0x100);
                c.bus.write32(OC341_ADD_HANDLES + 0x108, OC341_ADD_TYPES);
                for t in 0..4u32 {
                    let e = OC341_ADD_TYPE_ENTRIES + 0x20 * t;
                    c.bus
                        .write32(OC341_ADD_TYPES + 4 * t, if t == 3 { 0 } else { e });
                    c.bus.write32(e + 0x1c, OC341_ADD_OBJECTS);
                }
                for o in 0..4u32 {
                    let obj = OC341_ADD_OBJ + 0x100 * o;
                    c.bus.write32(
                        OC341_ADD_OBJECTS + 4 * o,
                        if o == 3 {
                            r.next_u32() & 0xff
                        } else {
                            (obj >> 2) << 8 | r.next_u32() & 0xff
                        },
                    );
                    let part = if r.next_u32() % 5 == 0 {
                        0
                    } else {
                        OC341_ADD_PART + 0x100 * o
                    };
                    c.bus.write32(obj + 0xc, part);
                    if part != 0 {
                        c.bus.write32(part + 0xc, OC341_ADD_PART_VT);
                    }
                }
                c.bus
                    .write16(OC341_ADD_PART_VT + 0x88, (r.next_u32() % 4 * 4) as u16);
                c.bus.write32(
                    OC341_ADD_PART_VT + 0x8c,
                    if r.next_u32() % 4 == 0 {
                        FAKE + 0x30
                    } else {
                        OC341_FAKE_ZERO
                    },
                );
                c.bus
                    .write16(OC341_ADD_PART_VT + 0xc0, (r.next_u32() % 4 * 4) as u16);
                c.bus.write32(OC341_ADD_PART_VT + 0xc4, FAKE + 0x40);
                oc341_danger_sources(c, &mut r, seed, handle);
                c.g[A0][0] = sx(OC341_MAN);
                c.g[A1][0] = sx(OC341_ADD_HANDLE);
                c.f[12] = r.edge_f32();
                c.f[13] = r.edge_f32();
            },
        );
    }

    // ---------------------------------------------------- zones in use

    const OC341_ZONE_HANDLE: u32 = DATA + 0x1000;
    const OC341_ZONE_NODES: u32 = DATA + 0x1100;

    /// A zone list of 0..4 nodes (handles 0..5, references 0..2) and a free
    /// list of two nodes; `*handle` is 0..5.
    fn oc341_zone_lists(c: &mut Cpu, r: &mut Rng, seed: u32) {
        fill(c, r, OC341_MAN + 0x700, 0x460 / 4);
        let used = seed % 5;
        for i in 0..6u32 {
            let n = OC341_ZONE_NODES + 0x10 * i;
            c.bus.write32(n, r.next_u32() % 6);
            c.bus.write32(n + 8, r.next_u32() % 3);
            // Used nodes 0..used chained; free nodes 4 -> 5.
            let next = if i + 1 < used || i == 4 { n + 0x10 } else { 0 };
            c.bus.write32(n + 0xc, next);
        }
        c.bus.write32(
            OC341_MAN + DANGER_MAN_ZONES,
            if used == 0 { 0 } else { OC341_ZONE_NODES },
        );
        c.bus
            .write32(OC341_MAN + DANGER_MAN_FREE_ZONES, OC341_ZONE_NODES + 0x40);
        c.bus.write32(OC341_ZONE_HANDLE, r.next_u32() % 6);
        c.g[A0][0] = sx(OC341_MAN);
        c.g[A1][0] = sx(OC341_ZONE_HANDLE);
    }

    #[test]
    fn danger_man_acquire_zone_matches() {
        check(
            ee::s00::f_00101900__generated,
            danger_man_acquire_zone,
            300,
            &|c, seed| {
                let mut r = Rng(0x0c34_1005 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0010_1900;
                oc341_zone_lists(c, &mut r, seed);
                c.g[A2][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn danger_man_release_zone_matches() {
        check(
            ee::s00::f_00101970__generated,
            danger_man_release_zone,
            300,
            &|c, seed| {
                let mut r = Rng(0x0c34_1006 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0010_1970;
                oc341_zone_lists(c, &mut r, seed);
            },
        );
    }

    // ---------------------------------------------------- FocusMan

    const OC341_FOCUS: u32 = DATA;
    const OC341_FOCUS_OBJS: u32 = DATA + 0x1000;

    /// A focus manager with 0, a negative or 1..6 objects of sections 0..2.
    fn oc341_focus_objects(c: &mut Cpu, r: &mut Rng, seed: u32) {
        fill(c, r, OC341_FOCUS, 0x100 / 4);
        let count = match seed % 6 {
            0 => 0,
            1 => 0xffff_fffe,
            _ => 1 + r.next_u32() % 6,
        };
        c.bus.write32(OC341_FOCUS + FOCUS_MAN_OBJECT_COUNT, count);
        for i in 0..6u32 {
            let obj = OC341_FOCUS_OBJS + 0x100 * i;
            c.bus.write32(OC341_FOCUS + FOCUS_MAN_OBJECTS + 4 * i, obj);
            c.bus.write8(obj + 0x78, (r.next_u32() % 3) as u8);
            c.bus.write32(obj + 8, r.next_u32());
        }
        c.g[A0][0] = sx(OC341_FOCUS);
    }

    #[test]
    fn focus_man_remove_section_objects_matches() {
        check(
            ee::s00::f_00103308__generated,
            focus_man_remove_section_objects,
            300,
            &|c, seed| {
                let mut r = Rng(0x0c34_1007 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0010_3308;
                oc341_focus_objects(c, &mut r, seed);
                c.g[A1][0] = if seed % 7 == 0 {
                    (r.next_u32() as u64) << 32 | (r.next_u32() % 3) as u64
                } else {
                    (r.next_u32() % 3) as u64
                };
            },
        );
    }

    #[test]
    fn focus_man_reset_matches() {
        check(
            ee::s00::f_00103358__generated,
            focus_man_reset,
            200,
            &|c, seed| {
                let mut r = Rng(0x0c34_1008 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0010_3358;
                oc341_focus_objects(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn focus_man_remove_focus_box_matches() {
        check(
            ee::s00::f_001033f8__generated,
            focus_man_remove_focus_box,
            300,
            &|c, seed| {
                let mut r = Rng(0x0c34_1009 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0010_33f8;
                fill(c, &mut r, OC341_FOCUS, 0x100 / 4);
                let count = match seed % 6 {
                    0 => 0,
                    1 => 0xffff_ffff,
                    _ => 1 + r.next_u32() % 5,
                };
                c.bus.write32(OC341_FOCUS, count);
                for i in 0..5u32 {
                    c.bus.write32(OC341_FOCUS + 0x10 + 0x20 * i, 0x100 + i);
                }
                c.g[A0][0] = sx(OC341_FOCUS);
                c.g[A1][0] = 0x100 + (r.next_u32() % 6) as u64;
            },
        );
    }

    // ------------------------------------------------- FocusMan_LoadState

    const OC341_LOAD_FOCUS: u32 = DATA + 0x5_0000;
    const OC341_LOAD_STREAM: u32 = DATA + 0x5_1000;
    const OC341_LOAD_STREAM_VT: u32 = DATA + 0x5_2000;

    /// `FocusMan_LoadState`: empty, negative and 1..4 counts for the boxes
    /// and the objects; fakes return objects in the data area and unwind at
    /// random calls.
    #[test]
    fn focus_man_load_state_matches() {
        covered(
            ee::s00::f_00103578__generated,
            focus_man_load_state,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_3578, 10);
                c.pc = 0x0010_3578;
                let boxes = match seed % 5 {
                    0 => 0,
                    1 => 0xffff_fffe,
                    _ => 1 + r.next_u32() % 4,
                };
                let objects = match seed % 7 {
                    0 => 0,
                    1 => 0x8000_0000,
                    _ => 1 + r.next_u32() % 4,
                };
                fill(c, &mut r, OC341_LOAD_FOCUS, 0x100 / 4);
                c.bus.write32(OC341_LOAD_FOCUS, boxes);
                c.bus
                    .write32(OC341_LOAD_FOCUS + FOCUS_MAN_OBJECT_COUNT, objects);
                c.bus.write32(OC341_LOAD_STREAM, OC341_LOAD_STREAM_VT);
                c.bus.write16(
                    OC341_LOAD_STREAM_VT + 0x10,
                    (r.next_u32() % 0x40) as u16 & !3,
                );
                c.bus.write32(OC341_LOAD_STREAM_VT + 0x14, FAKE);
                c.bus.write16(
                    OC341_LOAD_STREAM_VT + 0x18,
                    (r.next_u32() % 0x40) as u16 & !3,
                );
                c.bus.write32(OC341_LOAD_STREAM_VT + 0x1c, FAKE + 0x10);
                c.g[A0][0] = sx(OC341_LOAD_FOCUS);
                c.g[A1][0] = sx(OC341_LOAD_STREAM);
            },
        );
    }
}
