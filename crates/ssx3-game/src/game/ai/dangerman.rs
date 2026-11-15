//! `src/ai/dangerman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::ret;
use crate::call::sp;
use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0010_13a8, danger_man_update, "DangerMan_Update"),
    (
        0x0010_1a10,
        danger_man_get_rider_danger,
        "DangerMan_GetRiderDanger",
    ),
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
];

// ---------------------------------------------------------------------------
// Danger manager.

/// `int DangerMan_GetRiderDanger(DangerMan *man, int rider)` (0x101a10):
/// `man->riders[rider].danger` at `man + rider*12 + 0x710`.
pub fn danger_man_get_rider_danger(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = 0xc;
    let scaled = abi::mult(c, abi::arg(c, 1), 0xc);
    c.g[V1][0] = sx(scaled);
    let slot = abi::arg(c, 0).wrapping_add(scaled);
    c.g[A0][0] = sx(slot);
    c.g[V0][0] = Addr(slot).off(0x710).lw(c);
    abi::ret(c);
}

/// `DangerMan` fields.
pub(crate) mod danger {
    /// The race world: rider count at `+0x78`, rider pointers at `+0x28`.
    pub const WORLD: u32 = 0x0;
    /// 64 danger sources of 0x1c bytes: `+4` timer (s), `+8` state (0 live,
    /// 1 timed, 2 released), `+0xc..+0x14` position, `+0x18` object handle.
    pub const SOURCES: u32 = 0x4;
    pub const SOURCE_SIZE: u32 = 0x1c;
    pub const SOURCE_COUNT: i64 = 64;
    /// Live sources.
    pub const ACTIVE: u32 = 0x704;
    /// Six rider slots of 12 bytes: `+0` danger seen, `+4` danger level,
    /// `+8` hold (ticks).
    pub const RIDERS: u32 = 0x70c;
    pub const RIDER_SIZE: u32 = 0xc;
    /// The watch list (`+0` handle, `+4` danger, `+0xc` next) and the rider
    /// it is checked against.
    pub const WATCH_LIST: u32 = 0xb58;
    pub const WATCH_RIDER: u32 = 0xb5c;
    /// How long a seen danger is held, in 60 Hz frames (1 s).
    pub const HOLD_FRAMES: u32 = 0x3c;
    /// `gp + 0x16c8`: the object handle table (`**p + 8`: per-type tables).
    pub const GP_HANDLES: u32 = 0x16c8;
}

/// `DangerMan_ReleaseDanger(man, source)` (0x1441a0).
const RELEASE_DANGER: Callee = Callee {
    addr: 0x0014_41a0,
    f: ssx3_ee::s01::DangerMan_ReleaseDanger,
};
/// `int DangerMan_GetRiderDangerLevel(man, rider)` (0x1441b8).
const GET_RIDER_DANGER_LEVEL: Callee = Callee {
    addr: 0x0014_41b8,
    f: ssx3_ee::s01::DangerMan_GetRiderDangerLevel,
};

/// `void DangerMan_Update(DangerMan *man)` (0x1013a8), once per tick:
///
/// 1. With a watch list: ask the watched rider (`+0xb5c`, 0 if out of range)
///    whether each listed object is a danger to it (rider `+0x6c0` vtable
///    slot `+0x18`); the highest danger seen is stored in its slot with a
///    hold of 1 s, and its danger level is refreshed
///    (`DangerMan_GetRiderDangerLevel`).
/// 2. Count down every rider slot's hold, clearing the slot at zero.
/// 3. Walk the live danger sources: timed ones lose a step and are released
///    at zero; the others follow their object (released when it is gone or
///    its slot `+0x8c` says so, else its position is copied).
/// 4. An empty loop over the riders (cycles only).
///
/// Tick rate: the hold is `tick::ticks_from_frames60(60)` ticks (exact at
/// 60); the source timer steps by the `.lit4` step (`gp - 0x7f94`, already
/// 1/rate).
pub fn danger_man_update(c: &mut Cpu) {
    use danger::*;
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S2, 0x60);
        frame.save(c, S0, 0x80);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x70);
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save(c, S6, 0x20);
        frame.save_ra(c, 0x10);
        let man = c.g[S2][0] as u32;
        let gp = c.g[GP][0] as u32;
        c.g[V0][0] = c.lw(man.wrapping_add(WATCH_LIST));
        c.g[V1][0] = RIDER_SIZE as u64;

        // 1. The watch list against the watched rider.
        if c.g[V0][0] != 0 {
            c.cycles += 15;
            c.g[A0][0] = c.lw(man.wrapping_add(WORLD));
            c.g[S0][0] = c.g[V0][0];
            c.g[S3][0] = c.lw(man.wrapping_add(WATCH_RIDER));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x78));
            c.g[S1][0] = 0;
            c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.g[S3][0] = 0;
            }
            c.mult(0, V1, S3, V1);
            c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[S4][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x28));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(RIDERS));
            c.g[S6][0] = sx((c.g[S4][0] as u32).wrapping_add(0x6c0));
            c.g[S5][0] = sx(man.wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = c.lw(gp.wrapping_add(GP_HANDLES));
            loop {
                c.cycles += 13;
                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x6c0));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                let node = c.g[S0][0] as u32;
                c.g[A2][0] = c.lw(node);
                c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c));
                c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x18));
                // The listed object: `types[h & 0xff]->objects[h >> 8]`
                // (word >> 8 << 2), or 0.
                let handle = c.g[A2][0];
                c.g[V0][0] = handle & 0xff;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[A3][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.g[A1][0] = 0;
                if c.g[V1][0] != 0 {
                    c.cycles += 8;
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((handle as u32) >> 8);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                    if c.g[V0][0] != 0 {
                        c.g[A1][0] = sx((c.g[V0][0] as u32) << 2);
                    }
                }
                // Is this object a danger to the rider?
                c.cycles += 5;
                c.g[V0][0] = c.lw(c.g[T0][0] as u32);
                let method = c.g[V0][0] as u32;
                c.g[A0][0] = c.g[A3][0];
                call_ptr(c, method, 0x0010_1478)?;
                let node = c.g[S0][0] as u32;
                if c.g[V0][0] != 0 {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(node.wrapping_add(4));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[S1][0] = c.g[V1][0];
                    }
                }
                c.g[S0][0] = c.lw(node.wrapping_add(0xc));
                c.cycles += 2;
                let more = c.g[S0][0] != 0;
                c.g[V0][0] = c.lw(gp.wrapping_add(GP_HANDLES));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0010_1418);
                }
            }
            // The highest danger seen, held for 1 s.
            c.cycles += 2;
            let hold = crate::tick::ticks_from_frames60(c, HOLD_FRAMES) as u64;
            c.g[V0][0] = hold;
            if c.g[S1][0] != 0 {
                c.cycles += 2;
                let slot = c.g[S5][0] as u32;
                c.sw(slot, c.g[S1][0]);
                c.sw(slot.wrapping_add(8), hold);
            }
            c.cycles += 4;
            c.g[A1][0] = c.g[S3][0];
            c.g[A0][0] = c.g[S2][0];
            GET_RIDER_DANGER_LEVEL.call(c, 0x0010_14b4)?;
            c.sw((c.g[S5][0] as u32).wrapping_add(4), c.g[V0][0]);
        }

        // 2. Count down the six rider slots' holds.
        c.cycles += 2;
        c.g[V1][0] = sx(man.wrapping_add(RIDERS));
        c.g[A0][0] = 5;
        loop {
            c.cycles += 3;
            let slot = c.g[V1][0] as u32;
            c.g[V0][0] = c.lw(slot);
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[V0][0] = c.lw(slot.wrapping_add(8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                c.sw(slot.wrapping_add(8), c.g[V0][0]);
                if c.g[V0][0] == 0 {
                    c.cycles += 2;
                    c.sw(slot, 0);
                    c.sw(slot.wrapping_add(4), 0);
                }
                c.cycles += 1;
            }
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
            c.cycles += 2;
            let more = (c.g[A0][0] as i64) >= 0;
            c.g[V1][0] = sx(slot.wrapping_add(RIDER_SIZE));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0010_14c0);
            }
        }

        // 3. The live danger sources.
        c.cycles += 3;
        c.g[V0][0] = c.lw(man.wrapping_add(ACTIVE));
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            c.g[S3][0] = c.g[V0][0];
            c.g[S4][0] = 0;
            c.g[S5][0] = 1;
            c.g[V0][0] = SOURCE_SIZE as u64;
            loop {
                c.cycles += 7;
                c.g[V1][0] = 2;
                c.mult(0, V0, S4, V0);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(SOURCES));
                c.g[S1][0] = sx(man.wrapping_add(c.g[V0][0] as u32));
                let src = c.g[S1][0] as u32;
                c.g[V0][0] = c.lw(src.wrapping_add(8));
                if c.g[V0][0] != c.g[V1][0] {
                    c.cycles += 2;
                    let state = c.g[V0][0];
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(1));
                    let mut released = false;
                    if state == c.g[S5][0] {
                        // Timed: one step off its timer, released at zero.
                        c.cycles += 8;
                        c.lwc1(0, src.wrapping_add(4));
                        c.lwc1(1, gp.wrapping_sub(0x7f94));
                        c.mtc1(0, 2);
                        c.sub_s(0, 0, 1);
                        c.c_le_s(0, 2);
                        let expired = c.fcc();
                        c.swc1(0, src.wrapping_add(4));
                        if expired {
                            c.cycles += 5;
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A0][0] = c.g[S2][0];
                            RELEASE_DANGER.call(c, 0x0010_1560)?;
                            released = true;
                        }
                    }
                    if !released {
                        follow_source(c, src)?;
                    }
                }
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S4][0] as i64) < SOURCE_COUNT) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.cycles += 2;
                let more = c.g[S3][0] != 0;
                c.g[V0][0] = SOURCE_SIZE as u64;
                if !more {
                    c.cycles += 1;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0010_1510);
                }
            }
        }
        c.g[V0][0] = c.lw(man.wrapping_add(WORLD));

        // 4. An empty loop over the riders.
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            loop {
                c.cycles += 7;
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    break;
                }
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                if event_due(c) {
                    deliver_event_here(c, 0x0010_1640);
                }
            }
        }
        frame.restore(c, S0, 0x80);
        c.cycles += 9;
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// A live (or still running timed) danger source follows its object: none
/// (handle gone) or one whose slot `+0x8c` answers true releases it to state
/// 1 (`s5`); otherwise its position (slot `+0xc4`, `+0x30`) is copied.
fn follow_source(c: &mut Cpu, src: u32) -> crate::call::Flow {
    use danger::*;
    let gp = c.g[GP][0] as u32;
    c.cycles += 10;
    c.g[V1][0] = c.lw(gp.wrapping_add(GP_HANDLES));
    c.g[A1][0] = c.lw(src.wrapping_add(0x18));
    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
    c.g[V0][0] = c.g[A1][0] & 0xff;
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
    let no_type = c.g[V1][0] == 0;
    c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
    let mut found = false;
    if !no_type {
        c.cycles += 7;
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
        found = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    }
    if !found {
        c.cycles += 1;
        c.g[V0][0] = 0;
    }
    c.cycles += 3;
    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
    if c.g[S0][0] == 0 {
        c.sw(src.wrapping_add(8), c.g[S5][0]);
    } else {
        c.cycles += 7;
        let obj = c.g[S0][0] as u32;
        c.g[V1][0] = c.lw(obj.wrapping_add(0xc));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x88));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
        let method = c.g[V0][0] as u32;
        c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, method, 0x0010_15d0)?;
        let obj = c.g[S0][0] as u32;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.sw(src.wrapping_add(8), c.g[S5][0]);
        } else {
            c.g[V1][0] = c.lw(obj.wrapping_add(0xc));
            c.cycles += 12;
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
            let method = c.g[V0][0] as u32;
            c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, method, 0x0010_15f0)?;
            let sp = c.g[SP][0] as u32;
            c.lq(V1, (c.g[V0][0] as u32).wrapping_add(0x30));
            c.sq(V1, sp);
            c.lwc1(1, sp);
            c.lwc1(2, sp.wrapping_add(4));
            c.lwc1(0, sp.wrapping_add(8));
            c.swc1(1, src.wrapping_add(0xc));
            c.swc1(0, src.wrapping_add(0x14));
            c.swc1(2, src.wrapping_add(0x10));
        }
    }
    c.cycles += 1;
    Ok(())
}

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
pub(crate) const OC341_DANGER_MAN_RESET: Callee = Callee {
    addr: 0x0010_1688,
    f: ee::s00::DangerMan_Reset,
};

/// `DangerMan_RemoveDanger` (0x101888), through its slot.
pub(crate) const OC341_DANGER_MAN_REMOVE_DANGER: Callee = Callee {
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::fakes::{self, machine, FAKE};
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    #[test]
    fn danger_man_get_rider_danger_matches() {
        check(
            ssx3_ee::s00::f_00101a10__generated,
            danger_man_get_rider_danger,
            200,
            &|c, seed| {
                let mut r = Rng(0x2718_2818 ^ seed as u64);
                scramble(c, &mut r);
                let rider = seed % 5;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = rider as u64;
                c.bus.write32(DATA + rider * 12 + 0x710, r.next_u32());
            },
        );
    }

    // ------------------------------------------------------ DangerMan_Update

    const MAN: u32 = DATA;
    const WORLD: u32 = DATA + 0x2000;
    const RIDERS: u32 = DATA + 0x3000;
    const RIDER_VT: u32 = DATA + 0x6000;
    const NODES: u32 = DATA + 0x7000;
    const HANDLES: u32 = DATA + 0x8000;
    const TYPES: u32 = DATA + 0x8200;
    const TYPE_ENTRIES: u32 = DATA + 0x8800;
    const OBJECTS: u32 = DATA + 0x9000;
    const OBJ: u32 = DATA + 0xa000;
    const SUB: u32 = DATA + 0xb000;
    const SUB_VT: u32 = DATA + 0xc000;
    /// A fake method that answers 0 ([`zero_hook`]).
    const FAKE_ZERO: u32 = FAKE + 0x100;

    /// The shared fake, answering 0 for [`FAKE_ZERO`].
    fn zero_hook(c: &mut Cpu, target: u32) {
        let ret = c.g[RA][0] as u32;
        fakes::fake(c, target, ret);
        if c.pc == ret && target == FAKE_ZERO {
            c.g[V0][0] = 0;
        }
    }

    /// A handle `(object << 8) | type` for types 0..3 (type 3 has no table)
    /// and objects 0..3 (object 3 is gone).
    fn handle(r: &mut Rng) -> u32 {
        (r.next_u32() % 4) << 8 | r.next_u32() % 4
    }

    /// The danger manager, its race world (riders, their danger vtables),
    /// the object handle tables and objects, steered by `r`.
    fn danger_world(c: &mut Cpu, r: &mut Rng, watch: bool, active: u32) {
        c.hooks.miss = zero_hook;
        c.g[GP][0] = GAME_GP as u64;
        // The source timer's step (`.lit4`, gp - 0x7f94).
        c.bus.write32(GAME_GP - 0x7f94, crate::tick::STEP_60);
        let riders = 1 + r.next_u32() % 4;
        c.bus.write32(MAN, WORLD);
        c.bus.write32(WORLD + 0x78, riders);
        for i in 0..4 {
            let rd = RIDERS + 0x800 * i;
            c.bus.write32(WORLD + 0x28 + 4 * i, rd);
            c.bus.write32(rd + 0x6c0, RIDER_VT);
        }
        c.bus
            .write16(RIDER_VT + 0x18, (r.next_u32() % 4 * 4) as u16);
        c.bus.write32(
            RIDER_VT + 0x1c,
            if r.next_u32() % 3 == 0 {
                FAKE_ZERO
            } else {
                FAKE + 0x10
            },
        );
        c.bus
            .write16(RIDER_VT + 0x28, (r.next_u32() % 4 * 4) as u16);
        c.bus.write32(RIDER_VT + 0x2c, FAKE + 0x20);
        // Handles: gp + 0x16c8 -> p, *p + 8 -> per-type tables.
        c.bus.write32(GAME_GP + 0x16c8, HANDLES);
        c.bus.write32(HANDLES, HANDLES + 0x100);
        c.bus.write32(HANDLES + 0x108, TYPES);
        for t in 0..4u32 {
            let e = TYPE_ENTRIES + 0x20 * t;
            c.bus.write32(TYPES + 4 * t, if t == 3 { 0 } else { e });
            c.bus.write32(e + 0x1c, OBJECTS);
        }
        for o in 0..4u32 {
            let obj = OBJ + 0x100 * o;
            c.bus.write32(
                OBJECTS + 4 * o,
                if o == 3 {
                    r.next_u32() & 0xff
                } else {
                    (obj >> 2) << 8 | r.next_u32() & 0xff
                },
            );
            let sub = if r.next_u32() % 4 == 0 {
                0
            } else {
                SUB + 0x100 * o
            };
            c.bus.write32(obj + 0xc, sub);
            if sub != 0 {
                let vt = SUB_VT + 0x100 * (r.next_u32() % 2);
                c.bus.write32(sub + 0xc, vt);
            }
        }
        for (k, vt) in [SUB_VT, SUB_VT + 0x100].into_iter().enumerate() {
            c.bus.write16(vt + 0x88, (r.next_u32() % 4 * 4) as u16);
            c.bus
                .write32(vt + 0x8c, if k == 0 { FAKE_ZERO } else { FAKE + 0x30 });
            c.bus.write16(vt + 0xc0, (r.next_u32() % 4 * 4) as u16);
            c.bus.write32(vt + 0xc4, FAKE + 0x40);
        }
        // The watch list.
        let nodes = 1 + r.next_u32() % 3;
        c.bus.write32(MAN + 0xb58, if watch { NODES } else { 0 });
        for i in 0..nodes {
            let n = NODES + 0x10 * i;
            c.bus.write32(n, handle(r));
            c.bus.write32(
                n + 4,
                [0u32, 1, 2, 3, 0xffff_ffff][(r.next_u32() % 5) as usize],
            );
            c.bus
                .write32(n + 0xc, if i + 1 < nodes { n + 0x10 } else { 0 });
        }
        c.bus.write32(MAN + 0xb5c, r.next_u32() % 6);
        // Rider slots.
        for i in 0..6 {
            let s = MAN + 0x70c + 0xc * i;
            c.bus
                .write32(s, [0u32, 1, 2, 3][(r.next_u32() % 4) as usize]);
            c.bus.write32(s + 4, r.next_u32() % 4);
            c.bus.write32(
                s + 8,
                [0u32, 1, 2, 60, 0xffff_ffff][(r.next_u32() % 5) as usize],
            );
        }
        // Danger sources: the first eight random, the rest released.
        c.bus.write32(MAN + 0x704, active);
        c.bus.write32(MAN + 0x708, 0);
        for i in 0..64u32 {
            let src = MAN + 4 + 0x1c * i;
            let state = if i < 8 { r.next_u32() % 3 } else { 2 };
            c.bus.write32(src + 8, state);
            let t = [0.5f32, 0.01, -1.0, 1.0 / 60.0, 0.02][(r.next_u32() % 5) as usize];
            c.bus.write32(src + 4, t.to_bits());
            c.bus.write32(src + 0x18, handle(r));
        }
    }

    #[test]
    fn danger_man_update_matches() {
        check_lohi(
            ssx3_ee::s00::f_001013a8__generated,
            danger_man_update,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_13a8, 8);
                c.g[A0][0] = sx(MAN);
                let active = [0u32, 1, 2, 3, 9][(seed % 5) as usize];
                danger_world(c, &mut r, seed % 4 != 0, active);
            },
        );
    }

    /// Updates after a danger is seen (one update with the watch list, then
    /// none) until the rider's slot is cleared, at rate `hz`.
    fn danger_hold_updates(hz: u32) -> u32 {
        let mut c = crate::difftest::fresh();
        let c = &mut *c;
        let mut r = machine(c, 1, 0x0010_13a8, 0);
        crate::tick::set_for_test(c, hz);
        danger_world(c, &mut r, true, 0);
        // One node, a danger of 2 to rider 0, its slot empty.
        c.bus.write32(RIDER_VT + 0x1c, FAKE + 0x10);
        c.bus.write32(NODES + 4, 2);
        c.bus.write32(NODES + 0xc, 0);
        c.bus.write32(MAN + 0xb5c, 0);
        c.bus.write32(MAN + 0x70c, 0);
        let mut n = 0;
        loop {
            c.g[A0][0] = sx(MAN);
            c.g[SP][0] = sx(crate::difftest::SP);
            c.g[RA][0] = sx(crate::difftest::RA);
            danger_man_update(c);
            assert_eq!(c.pc, crate::difftest::RA, "update did not return");
            n += 1;
            c.bus.write32(MAN + 0xb58, 0);
            if c.bus.read32(MAN + 0x70c) == 0 {
                return n;
            }
            assert!(n < 10_000, "{hz} Hz: the danger is never cleared");
        }
    }

    /// A seen danger is held for 1 s at every rate (60 updates at 60 Hz).
    #[test]
    fn danger_man_update_hold_follows_the_rate() {
        assert_eq!(danger_hold_updates(60), 60);
        assert_eq!(danger_hold_updates(120), 120);
        assert_eq!(danger_hold_updates(240), 240);
    }

    use crate::difftest::fakes::covered;
    use crate::difftest::fill;

    /// The danger manager of the DangerMan tests.
    pub(crate) const OC341_MAN: u32 = DATA;

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
    pub(crate) fn oc341_danger_sources(c: &mut Cpu, r: &mut Rng, seed: u32, handle: u32) {
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

    pub(crate) const OC341_ADD_HANDLE: u32 = DATA + 0x1000;

    pub(crate) const OC341_ADD_HANDLES: u32 = DATA + 0x2000;

    pub(crate) const OC341_ADD_TYPES: u32 = DATA + 0x2200;

    pub(crate) const OC341_ADD_TYPE_ENTRIES: u32 = DATA + 0x2800;

    pub(crate) const OC341_ADD_OBJECTS: u32 = DATA + 0x3000;

    pub(crate) const OC341_ADD_OBJ: u32 = DATA + 0x4000;

    pub(crate) const OC341_ADD_PART: u32 = DATA + 0x5000;

    pub(crate) const OC341_ADD_PART_VT: u32 = DATA + 0x6000;

    /// A fake method answering 0 (the object is not gone).
    pub(crate) const OC341_FAKE_ZERO: u32 = FAKE + 0x100;

    /// The shared fake, answering 0 for [`OC341_FAKE_ZERO`].
    pub(crate) fn oc341_zero_hook(c: &mut Cpu, target: u32) {
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

    pub(crate) const OC341_ZONE_HANDLE: u32 = DATA + 0x1000;

    pub(crate) const OC341_ZONE_NODES: u32 = DATA + 0x1100;

    /// A zone list of 0..4 nodes (handles 0..5, references 0..2) and a free
    /// list of two nodes; `*handle` is 0..5.
    pub(crate) fn oc341_zone_lists(c: &mut Cpu, r: &mut Rng, seed: u32) {
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
}
