//! `src/ui/focusman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Target;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0010_1b60, focus_man_update, "FocusMan_Update"),
    (0x0010_32c0, focus_man_add_object, "FocusMan_AddObject"),
    (0x0010_33b0, focus_man_add_focus_box, "FocusMan_AddFocusBox"),
];

// ---------------------------------------------------------------------------
// Focus manager.

/// `void FocusMan_AddObject(FocusMan *self, FocusObject *obj)` (0x1032c0):
/// tag `obj->flags` (+0x8) with 0x100; store the new flags — OR 0x300 when
/// `self->locked` (+0xd4) is zero, else the tagged value with 0x200 cleared;
/// append `obj` to the list at +0xdc (count at +0xd8) and bump the count.
pub fn focus_man_add_object(c: &mut Cpu) {
    const LOCKED: u32 = 0xd4;
    const FLAGS: u32 = 0x8;
    const COUNT: u32 = 0xd8;
    const OBJECTS: u32 = 0xdc;
    c.cycles += 6;
    let this = abi::arg(c, 0);
    let obj = Addr(abi::arg(c, 1));
    let flags = obj.off(FLAGS).lw(c);
    c.g[V1][0] = flags;
    let tagged = flags | 0x100;
    c.g[A2][0] = tagged;
    obj.off(FLAGS).sw(c, tagged);
    let locked = Addr(this).off(LOCKED).lw(c);
    c.g[V0][0] = locked;
    let stored = if locked == 0 {
        flags | 0x300
    } else {
        c.cycles += 3;
        tagged & !0x200u64
    };
    c.g[V0][0] = stored;
    c.cycles += 8;
    obj.off(FLAGS).sw(c, stored);
    let count = Addr(this).off(COUNT).lw(c);
    c.g[V1][0] = count;
    c.g[V0][0] = sx((count as u32) << 2);
    c.g[V0][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = sx((count as u32).wrapping_add(1));
    Addr(c.g[V0][0] as u32).off(OBJECTS).sw(c, c.g[A1][0]);
    let ret = c.g[RA][0] as u32;
    Addr(this).off(COUNT).sw(c, c.g[V1][0]);
    c.pc = ret;
}

/// `void FocusMan_AddFocusBox(FocusMan *self, FocusBox *box)` (0x1033b0):
/// clear `self->active` (+0xd0) to -1, store `box` in slot `self->count`
/// (stride 0x20, +0x10), copy the box's quad to the next slot (+0x20) and to
/// the just-opened stack scratch, then bump the count.
pub fn focus_man_add_focus_box(c: &mut Cpu) {
    const COUNT: u32 = 0x0;
    const ACTIVE: u32 = 0xd0;
    const SLOT: u32 = 0x10;
    const QUAD: u32 = 0x20;
    c.cycles += 18;
    let this = abi::arg(c, 0);
    let box_ptr = abi::arg(c, 1);
    let count = Addr(this).off(COUNT).lw(c);
    c.g[V0][0] = count;
    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
    Addr(this).off(ACTIVE).sw(c, c.g[V1][0]);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[V0][0] = sx((count as u32) << 5);
    c.g[V0][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
    Addr(c.g[V0][0] as u32).off(SLOT).sw(c, c.g[A1][0]);
    let count = Addr(this).off(COUNT).lw(c);
    c.g[V1][0] = count;
    c.lq(6, box_ptr);
    c.g[V1][0] = sx((count as u32) << 5);
    c.g[V1][0] = sx(this.wrapping_add(c.g[V1][0] as u32));
    c.sq(6, c.g[SP][0] as u32);
    c.sq(6, (c.g[V1][0] as u32).wrapping_add(QUAD));
    let count = Addr(this).off(COUNT).lw(c);
    c.g[V0][0] = count;
    c.g[V0][0] = sx((count as u32).wrapping_add(1));
    Addr(this).off(COUNT).sw(c, c.g[V0][0]);
    let ret = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = ret;
}

// ---------------------------------------------------------------------------
// FocusMan_Update (0x101b60).
//
// The focus manager tracks which world objects are near any focus box (the
// riders, the camera...). Each update it measures how far each box has moved
// since the last rescan; at most every 20 60 Hz frames (or at once when a box
// moved 2000 units, or after a reset) it walks the world octree's eight level
// roots, collects the objects of every cell that overlaps a box, and then
// tells the script manager which objects entered and left the focus set.
//
// The original keeps its whole state in registers and in its 0x15c0-byte
// frame; the port keeps that shape (the callees and the event checks see the
// same registers) and splits it into one helper per block group.

/// `FocusMan` fields read by the update.
mod fm {
    /// `int` box count.
    pub const BOX_COUNT: u32 = 0x0;
    /// `FocusBox*` of box i at `+0x10 + 0x20 i` (its first quad is the
    /// position).
    pub const BOX: u32 = 0x10;
    /// The box position at the last rescan, quad at `+0x20 + 0x20 i`.
    pub const BOX_LAST: u32 = 0x20;
    /// `AI_GetFrameCounter()` at the last rescan (-1: rescan now).
    pub const RESCAN_STAMP: u32 = 0xd0;
    /// The rescan parity (toggled each rescan; object flag 0x200 marks the
    /// objects seen in the current one).
    pub const PHASE: u32 = 0xd4;
    /// The focus set: count and object pointers.
    pub const OBJ_COUNT: u32 = 0xd8;
    pub const OBJS: u32 = 0xdc;
    /// Object flags (`+0x8`): 0x100 in the focus set, 0x200 the phase mark.
    pub const OBJ_FLAGS: u32 = 0x8;
}

/// Offsets in the update's frame.
mod frm {
    /// The collected objects: count, then pointers.
    pub const LIST: u32 = 0x0;
    /// The boxes' extents (min quad, max quad; stride 0x20).
    pub const BOXES: u32 = 0x810;
    /// The octree walk's state `{list, boxes, box count}` passed to
    /// `FocusMan_CollectOctreeNode`; +0x0..+0x10 doubles as quad scratch.
    pub const SCAN: u32 = 0x8d0;
    /// Quad scratch, then the child key `{level, x, y, z}`.
    pub const KEY: u32 = 0x8e0;
    pub const SCRATCH: u32 = 0x8f0;
    /// Objects that entered / left the focus set, and the sort buffer.
    pub const ENTERED: u32 = 0x900;
    pub const LEFT: u32 = 0xd00;
    pub const SORTED: u32 = 0x1100;
    /// The two sorts' flag bytes.
    pub const SORT_FLAG: u32 = 0x1500;
    /// Spilled pointers: levels 6 and 7 of the octree, the focus set's
    /// array, the list's array.
    pub const LEVEL6: u32 = 0x1504;
    pub const LEVEL7: u32 = 0x1508;
    pub const OBJS_PTR: u32 = 0x150c;
    pub const LIST_PTR: u32 = 0x1510;
    /// Saved registers.
    pub const SAVE_S0: u32 = 0x15b0;
    pub const SAVE_S1: u32 = 0x15a0;
    pub const SAVE_S2: u32 = 0x1590;
    pub const SAVE_S3: u32 = 0x1580;
    pub const SAVE_S4: u32 = 0x1570;
    pub const SAVE_S5: u32 = 0x1560;
    pub const SAVE_S6: u32 = 0x1550;
    pub const SAVE_S7: u32 = 0x1540;
    pub const SAVE_FP: u32 = 0x1530;
    pub const SAVE_RA: u32 = 0x1520;
    pub const SIZE: u32 = 0x15c0;
}

/// The rescan interval in 60 Hz frames (`slti 0x14` at 0x101c90): three
/// rescans a second at any tick rate.
pub(crate) const RESCAN_FRAMES: u32 = 20;
/// A box that moved this far (2000.0) since the last rescan forces one.
const RESCAN_DISTANCE: u64 = 0x44fa_0000;
/// The focus box half-extent quad (`D_004A61E0`), addressed as
/// `0x4a0000 + 0x61e0`.
const BOX_EXTENT_HI: u64 = 0x004a_0000;
const BOX_EXTENT_LO: u32 = 0x61e0;
/// `World* g_world` (`gp-0x848`); `+0x84` its systems, whose `+0x20` is the
/// octree (eight level roots of 0x14 bytes: level, x, y, z, node).
const WORLD: u32 = 0x848;
/// `WScriptMan*` (`gp+0xce8`).
const SCRIPT_MAN: u32 = 0xce8;

const COLLECT_NODE_OBJECTS: Callee = Callee {
    addr: 0x0010_1a28,
    f: ssx3_ee::s00::f_00101a28,
};
const AI_GET_FRAME_COUNTER: Callee = Callee {
    addr: 0x0012_98c8,
    f: ssx3_ee::s00::f_001298c8,
};
/// `FocusMan_CollectOctreeNode(scan, node, key)` (through the table).
const COLLECT_OCTREE_NODE: u32 = 0x0014_4368;
/// `EA_MemCopy(dst, src, bytes)`.
const EA_MEM_COPY: u32 = 0x003e_6574;
/// `ObjPtrArray_Sort(begin, end, out, flag*)`.
const OBJ_PTR_ARRAY_SORT: u32 = 0x0013_fad8;
/// `WScriptMan_FocusLeave(man, obj)` / `WScriptMan_FocusEnter(man, obj)`.
const FOCUS_LEAVE: u32 = 0x0030_a460;
const FOCUS_ENTER: u32 = 0x0030_a3a0;

/// Where a level's root descriptor pointer lives during its walk.
#[derive(Clone, Copy, PartialEq)]
enum Desc {
    /// In a callee-saved register (levels 0-5).
    Reg(usize),
    /// Spilled at `LEVEL6` and reloaded into v0 at the loop head.
    Slot6,
    /// Spilled at `LEVEL7` and reloaded into a0 at the loop head.
    Slot7,
}

/// One octree level root walk of the unrolled original.
struct Level {
    /// The box loop's head (event checks; return addresses follow it).
    head: u32,
    desc: Desc,
    /// The register holding the root node.
    node: usize,
    /// The cell margin (`gp - margin`, `.lit4`).
    margin: u32,
    /// The next level's node (`octree + next`), loaded into v0 on the way
    /// out.
    next: u32,
}

const LEVELS: [Level; 8] = [
    Level {
        head: 0x0010_1ce8,
        desc: Desc::Reg(S5),
        node: S1,
        margin: 0x7f90,
        next: 0x24,
    },
    Level {
        head: 0x0010_1f80,
        desc: Desc::Reg(S1),
        node: S2,
        margin: 0x7f8c,
        next: 0x38,
    },
    Level {
        head: 0x0010_21f0,
        desc: Desc::Reg(S3),
        node: S1,
        margin: 0x7f88,
        next: 0x4c,
    },
    Level {
        head: 0x0010_2460,
        desc: Desc::Reg(S4),
        node: S1,
        margin: 0x7f84,
        next: 0x60,
    },
    Level {
        head: 0x0010_26d0,
        desc: Desc::Reg(S6),
        node: S1,
        margin: 0x7f80,
        next: 0x74,
    },
    Level {
        head: 0x0010_2940,
        desc: Desc::Reg(FP),
        node: S1,
        margin: 0x7f7c,
        next: 0x88,
    },
    Level {
        head: 0x0010_2bb8,
        desc: Desc::Slot6,
        node: S1,
        margin: 0x7f78,
        next: 0x9c,
    },
    Level {
        head: 0x0010_2e38,
        desc: Desc::Slot7,
        node: S1,
        margin: 0x7f74,
        next: 0,
    },
];

/// The children visited after a matching root, in the original's order:
/// (key field, step applied before the visit, child slot). The first child
/// (slot 0) is visited with the unmodified doubled key.
const CHILD_STEPS: [(u32, i32, u32); 7] = [
    (0x4, 1, 0x10),
    (0x8, 1, 0x18),
    (0x4, -1, 0x8),
    (0xc, 1, 0xc),
    (0x4, 1, 0x1c),
    (0x8, -1, 0x14),
    (0x4, -1, 0x4),
];

#[inline(always)]
fn sp_at(c: &Cpu, off: u32) -> u32 {
    (c.g[SP][0] as u32).wrapping_add(off)
}

#[inline(always)]
fn reg_at(c: &Cpu, r: usize, off: u32) -> u32 {
    (c.g[r][0] as u32).wrapping_add(off)
}

/// `void FocusMan_Update(FocusMan* self)` (0x101b60): see the section
/// comment. Tick rate: the rescan interval compares the AI frame counter
/// (ticks) with [`RESCAN_FRAMES`] 60 Hz frames converted to ticks.
pub fn focus_man_update(c: &mut Cpu) {
    use frm::*;
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, SIZE);
        c.g[T0][0] = 0;
        frame.save(c, S7, SAVE_S7);
        c.g[V0][0] = 0x5;
        frame.save(c, S0, SAVE_S0);
        c.g[S7][0] = c.g[A0][0];
        frame.save(c, S1, SAVE_S1);
        c.g[V1][0] = u64::MAX;
        frame.save(c, S2, SAVE_S2);
        frame.save(c, S3, SAVE_S3);
        frame.save(c, S4, SAVE_S4);
        frame.save(c, S5, SAVE_S5);
        frame.save(c, S6, SAVE_S6);
        frame.save(c, FP, SAVE_FP);
        frame.save_ra(c, SAVE_RA);
        Addr(sp_at(c, LIST)).sw(c, 0);
        c.g[V0][0] = Addr(c.g[S7][0] as u32).off(fm::BOX_COUNT).lw(c);
        c.g[A2][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            box_extents(c);
        }
        if rescan_due(c, frame)? {
            walk_octree(c)?;
            update_focus_set(c)?;
            c.cycles += 1;
            frame.restore(c, S0, SAVE_S0);
        }
        c.cycles += 11;
        frame.restore(c, S1, SAVE_S1);
        frame.restore(c, S2, SAVE_S2);
        frame.restore(c, S3, SAVE_S3);
        frame.restore(c, S4, SAVE_S4);
        frame.restore(c, S5, SAVE_S5);
        frame.restore(c, S6, SAVE_S6);
        frame.restore(c, S7, SAVE_S7);
        frame.restore(c, FP, SAVE_FP);
        frame.restore_ra(c, SAVE_RA);
        frame.close(c);
        Ok(())
    })
}

/// The box loop (0x101bc0): each box's extents (position -/+ the half
/// extent quad) into the frame, and t0 = 1 when a box moved at least 2000
/// since the last rescan (VU0 length of `last - position`).
fn box_extents(c: &mut Cpu) {
    use frm::*;
    c.cycles += 5;
    c.g[AT][0] = RESCAN_DISTANCE;
    c.mtc1(1, 1);
    c.g[A3][0] = BOX_EXTENT_HI;
    c.g[A1][0] = c.g[SP][0];
    c.g[A0][0] = c.g[S7][0];
    loop {
        c.cycles += 37;
        let extent = reg_at(c, A3, BOX_EXTENT_LO);
        c.g[V1][0] = Addr(reg_at(c, A0, fm::BOX)).lw(c);
        c.lq(V0, c.g[V1][0] as u32);
        c.sq(V0, sp_at(c, SCAN));
        // min = position - extent
        c.lqc2(3, sp_at(c, SCAN));
        c.lqc2(4, extent);
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp_at(c, SCRATCH));
        c.lq(V0, sp_at(c, SCRATCH));
        c.sq(V0, sp_at(c, KEY));
        c.sq(V0, reg_at(c, A1, BOXES));
        // max = position + extent
        c.lqc2(3, sp_at(c, SCAN));
        c.lqc2(4, extent);
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp_at(c, SCRATCH));
        c.lq(V0, sp_at(c, SCRATCH));
        c.sq(V0, sp_at(c, KEY));
        c.sq(V0, reg_at(c, A1, BOXES + 0x10));
        // |last - position|
        c.lqc2(3, reg_at(c, A0, fm::BOX_LAST));
        c.lqc2(4, sp_at(c, SCAN));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp_at(c, SCRATCH));
        c.lq(V0, sp_at(c, SCRATCH));
        c.sq(V0, sp_at(c, KEY));
        c.lqc2(3, sp_at(c, KEY));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(3, 22);
        c.mtc1(3, 0);
        c.c_le_s(1, 0);
        if c.fcc() {
            c.g[T0][0] = 0x1;
        }
        c.cycles += 6;
        c.g[V0][0] = Addr(c.g[S7][0] as u32).off(fm::BOX_COUNT).lw(c);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
        if c.g[V0][0] == 0 {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0010_1bc0);
        }
    }
}

/// Whether to rescan now: a box moved far, the stamp is negative (reset),
/// or the last rescan is at least [`RESCAN_FRAMES`] 60 Hz frames old. On
/// the skip path s0 is already reloaded (the original's delay slot); on the
/// rescan path a0 = the box count.
fn rescan_due(c: &mut Cpu, frame: Frame) -> Result<bool, call::Unwound> {
    c.cycles += 2;
    if c.g[T0][0] == 0 {
        c.cycles += 3;
        let this = Addr(c.g[S7][0] as u32);
        c.g[V0][0] = this.off(fm::RESCAN_STAMP).lw(c);
        if (c.g[V0][0] as i64) >= 0 {
            c.cycles += 7;
            AI_GET_FRAME_COUNTER.call(c, 0x0010_1c88)?;
            let this = Addr(c.g[S7][0] as u32);
            c.g[V1][0] = this.off(fm::RESCAN_STAMP).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            // Tick rate: the interval is in 60 Hz frames; the counter counts
            // ticks (exact at 60).
            let interval = crate::tick::ticks_from_frames60(c, RESCAN_FRAMES) as i64;
            c.g[V0][0] = ((c.g[V0][0] as i64) < interval) as u64;
            frame.restore(c, S0, frm::SAVE_S0);
            if c.g[V0][0] != 0 {
                return Ok(false);
            }
            c.cycles += 1;
        }
    }
    c.g[A0][0] = Addr(c.g[S7][0] as u32).off(fm::BOX_COUNT).lw(c);
    Ok(true)
}

/// The octree walk (0x101c9c..0x103094): the scan state in the frame, then
/// the eight level roots in turn. Each level's prologue tests the node
/// loaded into v0 by the previous level (level 0 tests its own).
fn walk_octree(c: &mut Cpu) -> Flow {
    use frm::*;
    c.cycles += 5;
    c.g[V0][0] = sx(sp_at(c, LIST + 4));
    c.g[V1][0] = sx(reg_at(c, S7, fm::OBJS));
    Addr(sp_at(c, LIST_PTR)).sw(c, c.g[V0][0]);
    Addr(sp_at(c, OBJS_PTR)).sw(c, c.g[V1][0]);
    if c.g[A0][0] == 0 {
        return Ok(());
    }
    // Level 0: the octree in s5.
    c.cycles += 10;
    let gp = c.g[GP][0] as u32;
    c.g[V1][0] = Addr(gp.wrapping_sub(WORLD)).lw(c);
    c.g[A1][0] = sx(sp_at(c, BOXES));
    c.g[V0][0] = Addr(reg_at(c, V1, 0x84)).lw(c);
    c.g[S5][0] = Addr(reg_at(c, V0, 0x20)).lw(c);
    Addr(sp_at(c, SCAN)).sw(c, c.g[SP][0]);
    Addr(sp_at(c, SCAN + 4)).sw(c, c.g[A1][0]);
    Addr(sp_at(c, SCAN + 8)).sw(c, c.g[A0][0]);
    c.g[S1][0] = Addr(reg_at(c, S5, 0x10)).lw(c);
    c.g[T1][0] = c.g[A0][0];
    if c.g[S1][0] == 0 {
        level_done(c, &LEVELS[0]);
    } else {
        c.cycles += 3;
        c.g[V1][0] = c.g[A1][0];
        c.g[A3][0] = 0;
        walk_level(c, &LEVELS[0])?;
    }
    // Level 1: the remaining roots into registers / spill slots.
    c.cycles += 10;
    c.g[A0][0] = sx(reg_at(c, S5, 0x78));
    c.g[V1][0] = sx(reg_at(c, S5, 0x8c));
    c.g[S1][0] = sx(reg_at(c, S5, 0x14));
    c.g[S3][0] = sx(reg_at(c, S5, 0x28));
    c.g[S4][0] = sx(reg_at(c, S5, 0x3c));
    c.g[S6][0] = sx(reg_at(c, S5, 0x50));
    c.g[FP][0] = sx(reg_at(c, S5, 0x64));
    Addr(sp_at(c, LEVEL6)).sw(c, c.g[A0][0]);
    Addr(sp_at(c, LEVEL7)).sw(c, c.g[V1][0]);
    if c.g[V0][0] == 0 {
        level_done(c, &LEVELS[1]);
    } else {
        c.cycles += 5;
        c.g[S2][0] = Addr(reg_at(c, S1, 0x10)).lw(c);
        c.g[A3][0] = 0;
        load_scan_state(c);
        walk_level(c, &LEVELS[1])?;
    }
    // Levels 2-5.
    for lv in &LEVELS[2..6] {
        let Desc::Reg(desc) = lv.desc else {
            unreachable!()
        };
        c.cycles += 2;
        let empty = c.g[V0][0] == 0;
        c.g[A3][0] = 0;
        if empty {
            level_done(c, lv);
        } else {
            c.cycles += 3;
            c.g[S1][0] = Addr(reg_at(c, desc, 0x10)).lw(c);
            load_scan_state(c);
            walk_level(c, lv)?;
        }
    }
    // Level 6 (spilled).
    c.cycles += 2;
    let empty = c.g[V0][0] == 0;
    c.g[A0][0] = Addr(sp_at(c, LEVEL6)).lw(c);
    if empty {
        level_done(c, &LEVELS[6]);
    } else {
        c.cycles += 5;
        c.g[A3][0] = 0;
        c.g[T1][0] = Addr(sp_at(c, SCAN + 8)).lw(c);
        c.g[S1][0] = Addr(reg_at(c, A0, 0x10)).lw(c);
        c.g[V1][0] = Addr(sp_at(c, SCAN + 4)).lw(c);
        walk_level(c, &LEVELS[6])?;
    }
    // Level 7 (spilled; its exits go straight to the stamp).
    c.cycles += 2;
    let empty = c.g[V0][0] == 0;
    c.g[V0][0] = Addr(sp_at(c, LEVEL7)).lw(c);
    if !empty {
        c.cycles += 5;
        c.g[A3][0] = 0;
        c.g[T1][0] = Addr(sp_at(c, SCAN + 8)).lw(c);
        c.g[S1][0] = Addr(reg_at(c, V0, 0x10)).lw(c);
        c.g[V1][0] = Addr(sp_at(c, SCAN + 4)).lw(c);
        walk_level(c, &LEVELS[7])?;
    }
    Ok(())
}

/// t1 = box count, v1 = the first box's extents (from the scan state).
#[inline(always)]
fn load_scan_state(c: &mut Cpu) {
    c.g[T1][0] = Addr(sp_at(c, frm::SCAN + 8)).lw(c);
    c.g[V1][0] = Addr(sp_at(c, frm::SCAN + 4)).lw(c);
}

/// v0 = the next level's root node.
#[inline(always)]
fn load_next(c: &mut Cpu, lv: &Level) {
    c.g[V0][0] = Addr(reg_at(c, S5, lv.next)).lw(c);
}

/// The common way out of a level (no early exit): one cycle, then the next
/// root node.
#[inline(always)]
fn level_done(c: &mut Cpu, lv: &Level) {
    c.cycles += 1;
    load_next(c, lv);
}

/// One level's box loop: the first box overlapping the root cell picks it
/// ([`visit_root`]); a3 = box index, t1 = box count, v1 = box extents.
fn walk_level(c: &mut Cpu, lv: &Level) -> Flow {
    loop {
        c.cycles += 2;
        let exhausted = c.g[A3][0] == c.g[T1][0];
        match lv.desc {
            Desc::Slot6 => c.g[V0][0] = Addr(sp_at(c, frm::LEVEL6)).lw(c),
            Desc::Slot7 => c.g[A0][0] = Addr(sp_at(c, frm::LEVEL7)).lw(c),
            Desc::Reg(_) => {}
        }
        if exhausted {
            match lv.desc {
                Desc::Reg(_) => load_next(c, lv),
                Desc::Slot6 => level_done(c, lv),
                Desc::Slot7 => {}
            }
            return Ok(());
        }
        cell_overlaps_box(c, lv);
        c.cycles += 2;
        let hit = c.g[T0][0] != 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
        if hit {
            return visit_root(c, lv);
        }
        c.cycles += 2;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
        if event_due(c) {
            deliver_event_here(c, lv.head);
        }
    }
}

/// t0 = 1 when the level root's cell `[(k - m) 2^level, (k + 1 + m) 2^level]`
/// (k = x, y, z; m = the level's margin) overlaps the box at v1 (min at
/// +0x0, max at +0x10). s0 = the root's level. FPU ops as the original
/// (f2 = 2^level from the exponent bits).
fn cell_overlaps_box(c: &mut Cpu, lv: &Level) {
    let margin = (c.g[GP][0] as u32).wrapping_sub(lv.margin);
    let ext = c.g[V1][0] as u32;
    c.cycles += 16;
    let desc = match lv.desc {
        Desc::Reg(r) => c.g[r][0] as u32,
        Desc::Slot6 => c.g[V0][0] as u32,
        Desc::Slot7 => c.g[A0][0] as u32,
    };
    c.lwc1(3, margin);
    c.g[A2][0] = Addr(desc).off(4).lw(c); // x
    c.g[V0][0] = Addr(desc).lw(c); // level
    c.mtc1(6, 0);
    c.cvt_s_w(0, 0);
    c.g[S0][0] = c.g[V0][0];
    c.lwc1(1, ext.wrapping_add(0x10));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7f));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 23);
    c.sub_s(0, 0, 3);
    c.mtc1(2, 2);
    c.mul_s(0, 0, 2);
    c.c_le_s(0, 1);
    c.g[T0][0] = 0;
    if !c.fcc() {
        return;
    }
    // y: (y - m) 2^level <= max.y
    match lv.desc {
        Desc::Reg(r) => {
            c.cycles += 9;
            c.g[A0][0] = Addr(reg_at(c, r, 8)).lw(c);
        }
        Desc::Slot6 => {
            c.cycles += 10;
            c.g[V0][0] = Addr(sp_at(c, frm::LEVEL6)).lw(c);
            c.g[A0][0] = Addr(reg_at(c, V0, 8)).lw(c);
        }
        Desc::Slot7 => {
            c.cycles += 10;
            c.g[A0][0] = Addr(reg_at(c, A0, 8)).lw(c);
        }
    }
    c.lwc1(1, ext.wrapping_add(0x14));
    c.mtc1(4, 0);
    c.cvt_s_w(0, 0);
    c.sub_s(0, 0, 3);
    c.mul_s(0, 0, 2);
    c.c_le_s(0, 1);
    if lv.desc == Desc::Slot7 {
        c.g[V0][0] = Addr(sp_at(c, frm::LEVEL7)).lw(c);
    }
    if !c.fcc() {
        return;
    }
    // z
    c.cycles += 10;
    let desc = match lv.desc {
        Desc::Reg(r) => c.g[r][0] as u32,
        Desc::Slot6 | Desc::Slot7 => c.g[V0][0] as u32,
    };
    c.g[A1][0] = Addr(desc).off(0xc).lw(c);
    c.lwc1(1, ext.wrapping_add(0x18));
    c.mtc1(5, 0);
    c.cvt_s_w(0, 0);
    c.sub_s(0, 0, 3);
    c.mul_s(0, 0, 2);
    c.c_le_s(0, 1);
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
    if !c.fcc() {
        return;
    }
    // min.x <= (x + 1 + m) 2^level, then y and z.
    c.cycles += 9;
    c.lwc1(1, ext);
    c.mtc1(2, 0);
    c.cvt_s_w(0, 0);
    c.add_s(0, 0, 3);
    c.mul_s(0, 0, 2);
    c.c_le_s(1, 0);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
    if !c.fcc() {
        return;
    }
    c.cycles += 9;
    c.lwc1(1, ext.wrapping_add(4));
    c.mtc1(2, 0);
    c.cvt_s_w(0, 0);
    c.add_s(0, 0, 3);
    c.mul_s(0, 0, 2);
    c.c_le_s(1, 0);
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
    if !c.fcc() {
        return;
    }
    c.cycles += 9;
    c.lwc1(1, ext.wrapping_add(8));
    c.mtc1(2, 0);
    c.cvt_s_w(0, 0);
    c.add_s(0, 0, 3);
    c.mul_s(0, 0, 2);
    c.c_le_s(1, 0);
    if c.fcc() {
        c.g[T0][0] = 0x1;
    }
}

/// A matching root: collect its objects (with all descendants below level
/// 14), else visit its eight children with the doubled key one level down
/// (none at level 11).
fn visit_root(c: &mut Cpu, lv: &Level) -> Flow {
    c.cycles += 7;
    c.g[S0][0] = ((c.g[S0][0] as i64) < 14i64) as u64;
    c.g[A0][0] = Addr(sp_at(c, frm::SCAN)).lw(c);
    c.g[A1][0] = c.g[lv.node][0];
    c.g[A2][0] = c.g[S0][0];
    let reg = matches!(lv.desc, Desc::Reg(_));
    COLLECT_NODE_OBJECTS.call(c, lv.head + if reg { 0x124 } else { 0x128 })?;
    let fine = c.g[S0][0] != 0;
    let desc = match lv.desc {
        Desc::Reg(r) => {
            if fine {
                load_next(c, lv);
                return Ok(());
            }
            c.cycles += 4;
            c.g[r][0] as u32
        }
        Desc::Slot6 => {
            if fine {
                load_next(c, lv);
                return Ok(());
            }
            c.cycles += 5;
            c.g[A0][0] = Addr(sp_at(c, frm::LEVEL6)).lw(c);
            c.g[A0][0] as u32
        }
        Desc::Slot7 => {
            c.g[A0][0] = Addr(sp_at(c, frm::LEVEL7)).lw(c);
            if fine {
                return Ok(());
            }
            c.cycles += 4;
            c.g[A0][0] as u32
        }
    };
    c.g[V0][0] = 0xb;
    c.g[V1][0] = Addr(desc).lw(c);
    let level_11 = c.g[V1][0] == c.g[V0][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
    if !level_11 {
        let first = lv.head
            + match lv.desc {
                Desc::Reg(_) => 0x178,
                Desc::Slot6 => 0x180,
                Desc::Slot7 => 0x17c,
            };
        visit_children(c, desc, lv.node, first)?;
    }
    if lv.desc != Desc::Slot7 {
        level_done(c, lv);
    }
    Ok(())
}

/// The child key `{level - 1 (in v1), 2x, 2y, 2z}` and the eight children
/// of the root node (in register `node`), each walked by
/// `FocusMan_CollectOctreeNode` with the key stepped to its cell.
fn visit_children(c: &mut Cpu, desc: u32, node: usize, first_ret: u32) -> Flow {
    use frm::*;
    c.cycles += 13;
    Addr(sp_at(c, KEY)).sw(c, c.g[V1][0]);
    c.g[V0][0] = sx((Addr(desc).off(4).lw(c) as u32) << 1);
    Addr(sp_at(c, KEY + 4)).sw(c, c.g[V0][0]);
    c.g[V1][0] = sx((Addr(desc).off(8).lw(c) as u32) << 1);
    Addr(sp_at(c, KEY + 8)).sw(c, c.g[V1][0]);
    c.g[V0][0] = sx((Addr(desc).off(0xc).lw(c) as u32) << 1);
    Addr(sp_at(c, KEY + 0xc)).sw(c, c.g[V0][0]);
    c.g[A1][0] = Addr(c.g[node][0] as u32).lw(c);
    c.g[A0][0] = sx(sp_at(c, SCAN));
    if c.g[A1][0] != 0 {
        c.cycles += 2;
        c.g[A2][0] = sx(sp_at(c, KEY));
        call_ptr(c, COLLECT_OCTREE_NODE, first_ret)?;
    }
    for (j, &(field, step, child)) in CHILD_STEPS.iter().enumerate() {
        c.cycles += 6;
        let key = sp_at(c, KEY + field);
        c.g[V0][0] = sx((Addr(key).lw(c) as u32).wrapping_add(step as u32));
        Addr(key).sw(c, c.g[V0][0]);
        c.g[A1][0] = Addr(reg_at(c, node, child)).lw(c);
        c.g[A0][0] = sx(sp_at(c, SCAN));
        if c.g[A1][0] != 0 {
            c.cycles += 2;
            c.g[A2][0] = sx(sp_at(c, KEY));
            call_ptr(c, COLLECT_OCTREE_NODE, first_ret + 0x20 * (j as u32 + 1))?;
        }
    }
    Ok(())
}

/// After a walk (0x103094..): stamp the rescan, remember the box positions,
/// flip the phase, split the collected list and the old focus set into the
/// objects that entered and left, store the new set, and report them to the
/// script manager (sorted).
fn update_focus_set(c: &mut Cpu) -> Flow {
    use frm::*;
    // Stamp and box positions.
    c.cycles += 6;
    AI_GET_FRAME_COUNTER.call(c, 0x0010_309c)?;
    let this = c.g[S7][0] as u32;
    c.g[V1][0] = Addr(this).off(fm::BOX_COUNT).lw(c);
    c.g[A1][0] = 0;
    Addr(this).off(fm::RESCAN_STAMP).sw(c, c.g[V0][0]);
    if (c.g[V1][0] as i64) > 0 {
        c.cycles += 1;
        c.g[A0][0] = c.g[S7][0];
        loop {
            c.cycles += 9;
            c.g[V0][0] = Addr(reg_at(c, A0, fm::BOX)).lw(c);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            c.lq(V1, c.g[V0][0] as u32);
            c.sq(V1, reg_at(c, A0, fm::BOX_LAST));
            c.sq(V1, sp_at(c, SCAN));
            c.g[V0][0] = Addr(this).off(fm::BOX_COUNT).lw(c);
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0010_30b0);
            }
        }
    }
    // Phase; the collected objects not yet in the set have entered (s2).
    c.cycles += 8;
    c.g[V0][0] = Addr(this).off(fm::PHASE).lw(c);
    c.g[S3][0] = 0;
    c.g[V1][0] = Addr(sp_at(c, LIST)).lw(c);
    c.g[A3][0] = 0;
    c.g[V0][0] ^= 1;
    c.g[S2][0] = 0;
    Addr(this).off(fm::PHASE).sw(c, c.g[V0][0]);
    if (c.g[V1][0] as i64) > 0 {
        c.cycles += 5;
        c.g[A2][0] = 0xffff_ffff_ffff_0000 | 0xfdff; // !0x200
        c.g[A1][0] = Addr(sp_at(c, LIST_PTR)).lw(c);
        c.g[A0][0] = sx(sp_at(c, ENTERED));
        loop {
            c.cycles += 5;
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            let flags = Addr(reg_at(c, V1, fm::OBJ_FLAGS));
            c.g[V0][0] = flags.lw(c) & 0x100;
            if c.g[V0][0] == 0 {
                c.cycles += 7;
                Addr(c.g[A0][0] as u32).sw(c, c.g[V1][0]);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                c.g[V0][0] = flags.lw(c) | 0x100;
                flags.sw(c, c.g[V0][0]);
            }
            c.g[V0][0] = Addr(this).off(fm::PHASE).lw(c);
            c.cycles += 2;
            let odd = c.g[V0][0] != 0;
            c.g[V0][0] = flags.lw(c);
            if odd {
                c.cycles += 2;
                c.g[V0][0] &= c.g[A2][0];
            } else {
                c.cycles += 1;
                c.g[V0][0] |= 0x200;
            }
            c.cycles += 6;
            flags.sw(c, c.g[V0][0]);
            c.g[V0][0] = Addr(sp_at(c, LIST)).lw(c);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0010_3108);
            }
        }
    }
    // The old set's objects not marked in this phase have left (s3).
    c.cycles += 3;
    c.g[V0][0] = Addr(this).off(fm::OBJ_COUNT).lw(c);
    c.g[A3][0] = 0;
    if (c.g[V0][0] as i64) > 0 {
        c.cycles += 6;
        c.g[T0][0] = 0xffff_ffff_ffff_0000 | 0xfeff; // !0x100
        c.g[V1][0] = sx(sp_at(c, LEFT));
        c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
        c.g[A1][0] = Addr(sp_at(c, OBJS_PTR)).lw(c);
        c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        loop {
            c.cycles += 8;
            c.g[A0][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[V1][0] = Addr(this).off(fm::PHASE).lw(c);
            let flags = Addr(reg_at(c, A0, fm::OBJ_FLAGS));
            c.g[V0][0] = flags.lw(c);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 9) as u32);
            c.g[V0][0] ^= 1;
            c.g[V0][0] &= 1;
            if c.g[V0][0] != c.g[V1][0] {
                c.cycles += 7;
                Addr(c.g[A2][0] as u32).sw(c, c.g[A0][0]);
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(4));
                c.g[V0][0] = flags.lw(c) & c.g[T0][0];
                flags.sw(c, c.g[V0][0]);
            }
            c.g[V0][0] = Addr(this).off(fm::OBJ_COUNT).lw(c);
            c.cycles += 4;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0010_3188);
            }
        }
    }
    // The new set = the collected list.
    c.cycles += 8;
    c.g[A2][0] = Addr(sp_at(c, LIST)).lw(c);
    c.g[A0][0] = Addr(sp_at(c, OBJS_PTR)).lw(c);
    c.g[A1][0] = Addr(sp_at(c, LIST_PTR)).lw(c);
    Addr(this).off(fm::OBJ_COUNT).sw(c, c.g[A2][0]);
    c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
    call_ptr(c, EA_MEM_COPY, 0x0010_31ec)?;
    // Report the departures, then the arrivals.
    c.g[A0][0] = sx(sp_at(c, LEFT));
    if c.g[S3][0] != 0 {
        c.cycles += 9;
        notify(
            c,
            S3,
            SORT_FLAG,
            0x0010_3210,
            FOCUS_LEAVE,
            0x0010_3238,
            0x0010_3228,
            false,
        )?;
    }
    c.cycles += 2;
    c.g[A0][0] = sx(sp_at(c, ENTERED));
    if c.g[S2][0] != 0 {
        c.cycles += 9;
        notify(
            c,
            S2,
            SORT_FLAG + 1,
            0x0010_3264,
            FOCUS_ENTER,
            0x0010_3288,
            0x0010_3278,
            true,
        )?;
    }
    Ok(())
}

/// Sort the `count` (register) objects at a0 into `SORTED` and call
/// `report(script_man, obj)` for each. The two copies differ only in when
/// s1 takes the sorted buffer (`early_s1`: before the count test, else in
/// the loop prologue, one cycle longer).
#[allow(clippy::too_many_arguments)]
fn notify(
    c: &mut Cpu,
    count: usize,
    flag: u32,
    sort_ret: u32,
    report: u32,
    report_ret: u32,
    head: u32,
    early_s1: bool,
) -> Flow {
    c.g[S0][0] = sx(sp_at(c, frm::SORTED));
    c.g[A1][0] = sx((c.g[count][0] as u32) << 2);
    Addr(sp_at(c, flag)).sb(c, 0);
    c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[A2][0] = c.g[S0][0];
    c.g[A3][0] = sx(sp_at(c, flag));
    call_ptr(c, OBJ_PTR_ARRAY_SORT, sort_ret)?;
    if early_s1 {
        c.g[S1][0] = c.g[S0][0];
    }
    if (c.g[count][0] as i64) <= 0 {
        return Ok(());
    }
    c.cycles += if early_s1 { 3 } else { 4 };
    if !early_s1 {
        c.g[S1][0] = c.g[S0][0];
    }
    c.g[S0][0] = c.g[count][0];
    c.g[A1][0] = Addr(c.g[S1][0] as u32).lw(c);
    loop {
        c.cycles += 6;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
        let gp = c.g[GP][0] as u32;
        c.g[A0][0] = Addr(gp.wrapping_add(SCRIPT_MAN)).lw(c);
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
        call_ptr(c, report, report_ret)?;
        if c.g[S0][0] == 0 {
            return Ok(());
        }
        c.g[A1][0] = Addr(c.g[S1][0] as u32).lw(c);
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    #[test]
    fn focus_man_add_object_matches() {
        check(
            ssx3_ee::s00::f_001032c0,
            focus_man_add_object,
            300,
            &|c, seed| {
                let mut r = Rng(0x1032_c000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x400) as u64;
                let count = (seed % 5) as u32;
                c.bus
                    .write32(DATA + 0xd4, if seed % 3 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(DATA + 0xd8, count);
                c.bus.write32(DATA + 0x400 + 0x8, r.next_u32());
                c.bus.write32(DATA + count * 4 + 0xdc, r.next_u32());
            },
        );
    }

    #[test]
    fn focus_man_add_focus_box_matches() {
        check(
            ssx3_ee::s00::f_001033b0,
            focus_man_add_focus_box,
            200,
            &|c, seed| {
                let mut r = Rng(0x1033_b000 ^ seed as u64);
                scramble(c, &mut r);
                let count = (seed % 4) as u32;
                c.g[A0][0] = DATA as u64;
                // Odd seeds use a misaligned box to cover `lq` masking.
                c.g[A1][0] = (DATA + 0x600 + (seed % 2) * 8) as u64;
                c.bus.write32(DATA, count);
                for i in 0..4 {
                    c.bus.write32(DATA + 0x600 + i * 4, r.next_u32());
                }
            },
        );
    }

    // FocusMan_Update fixtures.
    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{fresh, put_vec};

    const SELF: u32 = DATA;
    const BOXES: u32 = DATA + 0x1000;
    const WORLD_OBJ: u32 = DATA + 0x2000;
    const SYSTEMS: u32 = DATA + 0x2100;
    const AI_OBJ: u32 = DATA + 0x2180;
    const OCTREE: u32 = DATA + 0x2200;
    /// Octree nodes, 0x40 each: eight children, then the object list.
    const NODES: u32 = DATA + 0x3000;
    /// Focus objects, 0x20 each: next, flags (+0x8), owner (+0xc).
    const OBJECTS: u32 = DATA + 0x8000;
    const POOL: u32 = 96;
    const OWNER: u32 = DATA + 0x9000;
    const OWNER_VT: u32 = DATA + 0x9100;
    const SCRIPT_MAN_OBJ: u32 = DATA + 0xa000;

    /// The world, its octree pointer, the AI frame counter, the script
    /// manager, the box half-extent and the level margins.
    fn focus_world(c: &mut Cpu, counter: u32) {
        c.g[GP][0] = GAME_GP as u64;
        let gp = GAME_GP;
        c.bus.write32(gp - 0x848, WORLD_OBJ);
        c.bus.write32(WORLD_OBJ + 0x84, SYSTEMS);
        c.bus.write32(SYSTEMS + 0x20, OCTREE);
        c.bus.write32(SYSTEMS + 0xc, AI_OBJ);
        c.bus.write32(AI_OBJ + 8, counter);
        c.bus.write32(gp + 0xce8, SCRIPT_MAN_OBJ);
        let e = 100.0f32.to_bits();
        put_vec(c, 0x004a_61e0, [e, e, e, 0]);
        for k in 0..8 {
            c.bus.write32(gp - 0x7f90 + 4 * k, 0.25f32.to_bits());
        }
    }

    fn coord(r: &mut Rng) -> f32 {
        (r.next_u32() % 40000) as f32 - 4000.0
    }

    /// Up to three boxes (some moved >= 2000), every stamp case, an old
    /// focus set, scripted objects whose owners' vtables are fakes, and an
    /// octree whose eight roots are empty or nodes (levels 10..15, so both
    /// the fine collect and the child walk) with leaf children.
    fn focus_update_fixture(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x0010_1b60, 8);
        let counter = 1000 + r.next_u32() % 1000;
        focus_world(c, counter);
        c.pc = 0x0010_1b60;
        c.g[A0][0] = SELF as u64;
        let boxes = seed % 4;
        c.bus.write32(SELF, boxes);
        let one = 1.0f32.to_bits();
        for i in 0..boxes {
            let b = BOXES + 0x10 * i;
            let p = [coord(&mut r), coord(&mut r), coord(&mut r)];
            put_vec(c, b, [p[0].to_bits(), p[1].to_bits(), p[2].to_bits(), one]);
            c.bus.write32(SELF + 0x10 + 0x20 * i, b);
            let mut last = [0u32; 4];
            for a in 0..3 {
                last[a] = (p[a] + (r.next_u32() % 3000) as f32 - 1500.0).to_bits();
            }
            last[3] = one;
            put_vec(c, SELF + 0x20 + 0x20 * i, last);
        }
        let stamp = match (seed / 4) % 4 {
            0 => u32::MAX,
            1 => counter - r.next_u32() % 20,
            _ => counter - 20 - r.next_u32() % 40,
        };
        c.bus.write32(SELF + 0xd0, stamp);
        c.bus.write32(SELF + 0xd4, r.next_u32() & 1);
        for j in 0..POOL {
            let o = OBJECTS + 0x20 * j;
            c.bus.write32(o, 0);
            c.bus.write32(o + 8, r.next_u32() & 0xf0f);
            c.bus.write32(o + 0xc, OWNER);
        }
        c.bus.write32(OWNER + 0xc, OWNER_VT);
        for (k, off) in [0xa0u32, 0xa8, 0xb0].into_iter().enumerate() {
            c.bus.write16(OWNER_VT + off, 0);
            c.bus.write32(OWNER_VT + off + 4, FAKE + 0x10 * k as u32);
        }
        let old = r.next_u32() % 5;
        c.bus.write32(SELF + 0xd8, old);
        for k in 0..old {
            c.bus
                .write32(SELF + 0xdc + 4 * k, OBJECTS + 0x20 * (r.next_u32() % POOL));
        }
        let mut leaf = 8u32;
        for k in 0..8u32 {
            let d = OCTREE + 0x14 * k;
            c.bus
                .write32(d, [10u32, 12, 14, 15][(r.next_u32() % 4) as usize]);
            for a in 1..4 {
                c.bus.write32(d + 4 * a, (r.next_u32() % 3).wrapping_sub(1));
            }
            let node = if r.next_u32() % 5 == 0 {
                0
            } else {
                NODES + 0x40 * k
            };
            c.bus.write32(d + 0x10, node);
            if node == 0 {
                continue;
            }
            for s in 0..8 {
                let child = if r.next_u32() % 4 == 0 {
                    leaf += 1;
                    NODES + 0x40 * (leaf - 1)
                } else {
                    0
                };
                c.bus.write32(node + 4 * s, child);
                if child != 0 && r.next_u32() % 2 == 0 {
                    c.bus.write32(child + 0x20, OBJECTS + 0x20 * (leaf - 1));
                }
            }
            if r.next_u32() % 4 != 0 {
                c.bus.write32(node + 0x20, OBJECTS + 0x20 * k);
            }
        }
    }

    #[test]
    fn focus_man_update_matches() {
        covered(
            ssx3_ee::s00::f_00101b60__generated,
            focus_man_update,
            300,
            &focus_update_fixture,
        );
    }

    /// Rescans in one second of ticks at `hz` (one still box, an empty
    /// octree, the AI frame counter advancing once per tick).
    fn rescans_in_one_second(hz: u32) -> u32 {
        let mut c = fresh();
        crate::tick::set_for_test(&mut c, hz);
        focus_world(&mut c, 0);
        let one = 1.0f32.to_bits();
        put_vec(&mut c, BOXES, [0, 0, 0, one]);
        put_vec(&mut c, SELF + 0x20, [0, 0, 0, one]);
        c.bus.write32(SELF, 1);
        c.bus.write32(SELF + 0x10, BOXES);
        c.bus.write32(SELF + 0xd0, u32::MAX);
        let mut rescans = 0;
        for t in 0..hz {
            c.bus.write32(AI_OBJ + 8, 1000 + t);
            c.g[A0][0] = SELF as u64;
            c.g[RA][0] = crate::difftest::RA as u64;
            c.pc = 0x0010_1b60;
            focus_man_update(&mut c);
            assert_eq!(c.pc, crate::difftest::RA);
            assert_eq!(c.g[SP][0] as u32, crate::difftest::SP);
            if c.bus.read32(SELF + 0xd0) == 1000 + t {
                rescans += 1;
            }
        }
        rescans
    }

    #[test]
    fn focus_man_update_rescans_three_times_a_second() {
        for hz in [60, 120, 240] {
            assert_eq!(rescans_in_one_second(hz), 3, "{hz} Hz");
        }
    }
}
