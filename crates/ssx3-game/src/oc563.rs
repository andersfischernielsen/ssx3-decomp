//! Staging module oc563 (wave wave232): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc563`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0033_0ba8,
    overlap_list_gather_objects,
    "OverlapList_GatherObjects",
)];

/// Stack frame of `OverlapList_GatherObjects`.
const FRAME: u32 = 0xa0;
/// Saved-register slots (`sq` of s0..s6, `sd` of ra), offsets from `sp`.
const SAVE_S0: u32 = 0x90;
const SAVE_S1: u32 = 0x80;
const SAVE_S2: u32 = 0x70;
const SAVE_S3: u32 = 0x60;
const SAVE_S4: u32 = 0x50;
const SAVE_S5: u32 = 0x40;
const SAVE_S6: u32 = 0x30;
const SAVE_RA: u32 = 0x20;

/// The `OverlapInstancesCallback` record on the frame: instance array pointer,
/// count pointer, maximum (0x40) and the query box.
const CB_INSTANCES: u32 = 0x0;
const CB_NUMBER: u32 = 0x4;
const CB_MAXIMUM: u32 = 0x8;
const CB_BOX: u32 = 0xc;
/// `char flags[8]` on the frame; cell `k` gets `FLAGS + k`.
const FLAGS: u32 = 0x10;

/// The overlap list (`self`): the Nash grid, the delete count, the instance
/// count and array, the patch count (-1 when not collected) and the typed count.
const LIST_NASH: u32 = 0x00;
const LIST_DELETE_COUNT: u32 = 0x04;
const LIST_NUM_INSTANCES: u32 = 0x08;
const LIST_INSTANCES: u32 = 0x0c;
const LIST_NUM_PATCHES: u32 = 0x10c;
const LIST_NUM_TYPED: u32 = 0x210;

/// The grid (`sGrid`): eight octant cells of 0x14 bytes (level, x, y, z, tree),
/// then the delete count at +0xa0.
const CELL_SIZE: u32 = 0x14;
const CELL_TREE: u32 = 0x10;
const GRID_DELETE_COUNT: u32 = 0xa0;

/// Return addresses of the calls. Each cell's code is 0x104 bytes after the
/// previous one's; a cell's children are 0x14 bytes apart.
const PROX_RET: u32 = 0x0033_0c1c;
const COLLECT_RET: u32 = 0x0033_0c34;
const CHILD_RET: u32 = 0x0033_0c48;
const REC_RET: u32 = 0x0033_0d00;
const CELL_STRIDE_RET: u32 = 0x104;
const CHILD_STRIDE_RET: u32 = 0x14;
/// Cell 7's recursive call returns to the epilogue block, not the stride.
const REC_LAST_RET: u32 = 0x0033_1418;

/// The cell proximity test (0 inside, 1 outside, 2 crossing).
const PROX_FN: u32 = 0x0032_8360;

const CELL_GATHER_OBJECTS: Callee = Callee {
    addr: 0x0033_09d8,
    f: ee::s11::f_003309d8,
};
const CELL_GATHER_OBJECTS_SUBTREE: Callee = Callee {
    addr: 0x0034_0a40,
    f: ee::s11::f_00340a40,
};
const CELL_GATHER_OBJECTS_RECURSIVE: Callee = Callee {
    addr: 0x0033_8920,
    f: ee::s11::f_00338920,
};

fn cell_ret(base: u32, k: u32) -> u32 {
    base + CELL_STRIDE_RET * k
}

fn child_ret(k: u32, j: u32) -> u32 {
    cell_ret(CHILD_RET, k) + CHILD_STRIDE_RET * j
}

fn rec_ret(k: u32) -> u32 {
    cell_ret(REC_RET, k)
}

/// `void cOverlapList_getinstances(cOverlapList* self, tBoundBox* box)`
/// (0x330ba8, `OverlapList_GatherObjects`): collects into `self`'s instance
/// array the instances whose bounds overlap `box`, walking the Nash grid's eight
/// octant roots. The delete count is copied from the grid, the instance count is
/// cleared, and the patch and typed counts end at -1 (not collected).
///
/// Each root with a tree is tested against the box (`0x328360`). Inside (0): the
/// root goes to the callback and each of its children to the subtree walker.
/// Crossing (2): the cell goes to the recursive collector. Outside (1): skipped.
pub fn overlap_list_gather_objects(c: &mut Cpu) {
    guest(c, gather_objects);
}

fn gather_objects(c: &mut Cpu) -> Flow {
    c.cycles += 25;
    let frame = Frame::open(c, FRAME);
    frame.save(c, S0, SAVE_S0);
    frame.save(c, S5, SAVE_S5);
    frame.save(c, S6, SAVE_S6);
    c.g[S5][0] = c.g[A1][0];
    frame.save(c, S1, SAVE_S1);
    c.g[S6][0] = c.g[A0][0];
    frame.save(c, S2, SAVE_S2);
    let list = c.g[S6][0] as u32;
    c.g[V1][0] = sx(list.wrapping_add(LIST_INSTANCES));
    frame.save(c, S3, SAVE_S3);
    c.g[A0][0] = sx(list.wrapping_add(LIST_NUM_INSTANCES));
    frame.save_ra(c, SAVE_RA);
    c.g[A1][0] = 0x40;
    frame.save(c, S4, SAVE_S4);
    c.g[S4][0] = Addr(list.wrapping_add(LIST_NASH)).lw(c);
    let grid = c.g[S4][0] as u32;
    c.g[V0][0] = Addr(grid.wrapping_add(GRID_DELETE_COUNT)).lw(c);
    let instances = c.g[V1][0];
    Addr(sp(c).wrapping_add(CB_INSTANCES)).sw(c, instances);
    let deletes = c.g[V0][0];
    Addr(list.wrapping_add(LIST_DELETE_COUNT)).sw(c, deletes);
    let number = c.g[A0][0];
    Addr(sp(c).wrapping_add(CB_NUMBER)).sw(c, number);
    let maximum = c.g[A1][0];
    Addr(sp(c).wrapping_add(CB_MAXIMUM)).sw(c, maximum);
    let bbox = c.g[S5][0];
    Addr(sp(c).wrapping_add(CB_BOX)).sw(c, bbox);
    Addr(list.wrapping_add(LIST_NUM_INSTANCES)).sw(c, 0);
    c.g[S1][0] = Addr(grid.wrapping_add(CELL_TREE)).lw(c);
    c.g[S0][0] = c.g[SP][0];

    gather_cell0(c)?;
    for k in 1..=6 {
        gather_cell(c, k)?;
    }
    gather_last_cell(c)?;

    // Epilogue: the patch and typed counts become -1.
    c.cycles += 14;
    c.g[V0][0] = 0xffff_ffff_ffff_0000;
    frame.restore(c, S0, SAVE_S0);
    c.g[V0][0] |= 0xffff;
    frame.restore(c, S1, SAVE_S1);
    let minus_one = c.g[V0][0];
    let list = c.g[S6][0] as u32;
    Addr(list.wrapping_add(LIST_NUM_TYPED)).sw(c, minus_one);
    Addr(list.wrapping_add(LIST_NUM_PATCHES)).sw(c, minus_one);
    frame.restore(c, S2, SAVE_S2);
    frame.restore(c, S3, SAVE_S3);
    frame.restore(c, S4, SAVE_S4);
    frame.restore(c, S5, SAVE_S5);
    frame.restore(c, S6, SAVE_S6);
    frame.restore_ra(c, SAVE_RA);
    frame.close(c);
    Ok(())
}

/// Sets `V0` to the tree of the cell after `k`.
fn load_next_tree(c: &mut Cpu, k: u32) {
    let grid = c.g[S4][0] as u32;
    c.g[V0][0] = Addr(grid.wrapping_add(CELL_SIZE * (k + 1) + CELL_TREE)).lw(c);
}

/// A cell with no tree: one more cycle, then on to the next cell.
fn skip_cell(c: &mut Cpu, k: u32) {
    c.cycles += 1;
    load_next_tree(c, k);
}

/// Collects one octant root whose box holds the whole subtree: the root goes to
/// the callback (`Cell_GatherObjects`), then its eight children are walked with
/// `Cell_GatherObjectsSubtree`. A child is tested in `A0`, and the next child's
/// pointer is loaded after each call.
fn gather_node(c: &mut Cpu, k: u32) -> Flow {
    c.cycles += 6;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    CELL_GATHER_OBJECTS.call(c, cell_ret(COLLECT_RET, k))?;
    c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
    for j in 0..7u32 {
        if j > 0 {
            c.cycles += 2;
        }
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = c.g[S0][0];
            CELL_GATHER_OBJECTS_SUBTREE.call(c, child_ret(k, j))?;
        }
        let slot = (c.g[S1][0] as u32).wrapping_add(4 * (j + 1));
        c.g[A0][0] = Addr(slot).lw(c);
    }
    c.cycles += 2;
    if c.g[A0][0] != 0 {
        c.cycles += 4;
        c.g[A1][0] = c.g[S0][0];
        CELL_GATHER_OBJECTS_SUBTREE.call(c, child_ret(k, 7))?;
    }
    Ok(())
}

/// Octant 0: the cell pointer is the grid itself, its flag is `flags[0]` in s2.
/// Entered after the prologue, with `s1` = the tree.
fn gather_cell0(c: &mut Cpu) -> Flow {
    if c.g[S1][0] == 0 {
        skip_cell(c, 0);
        return Ok(());
    }
    c.cycles += 7;
    c.g[S2][0] = sx(sp(c).wrapping_add(FLAGS));
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = c.g[S5][0];
    call_ptr(c, PROX_FN, cell_ret(PROX_RET, 0))?;
    c.g[V1][0] = c.g[V0][0];
    let hit = c.g[V1][0] != 0;
    c.g[V0][0] = 2;
    if hit {
        c.cycles += 2;
        if c.g[V1][0] != c.g[V0][0] {
            load_next_tree(c, 0);
            return Ok(());
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S2][0];
        c.g[T0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S5][0];
        c.g[T1][0] = c.g[S4][0];
        CELL_GATHER_OBJECTS_RECURSIVE.call(c, rec_ret(0))?;
        skip_cell(c, 0);
        return Ok(());
    }
    gather_node(c, 0)?;
    load_next_tree(c, 0);
    Ok(())
}

/// Octant `k` (1..=6): entered with `V0` = the cell's tree, leaves `V0` = the
/// next cell's tree. Cell pointer in s2, flag in s3.
fn gather_cell(c: &mut Cpu, k: u32) -> Flow {
    c.cycles += 2;
    let empty = c.g[V0][0] == 0;
    let cell = (c.g[S4][0] as u32).wrapping_add(CELL_SIZE * k);
    c.g[S2][0] = sx(cell);
    if empty {
        skip_cell(c, k);
        return Ok(());
    }
    c.cycles += 8;
    c.g[S1][0] = Addr(cell.wrapping_add(CELL_TREE)).lw(c);
    c.g[S3][0] = sx(sp(c).wrapping_add(FLAGS + k));
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S5][0];
    call_ptr(c, PROX_FN, cell_ret(PROX_RET, k))?;
    c.g[V1][0] = c.g[V0][0];
    let hit = c.g[V1][0] != 0;
    c.g[V0][0] = 2;
    if hit {
        c.cycles += 2;
        if c.g[V1][0] != c.g[V0][0] {
            load_next_tree(c, k);
            return Ok(());
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S3][0];
        c.g[T0][0] = c.g[S1][0];
        c.g[T1][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S5][0];
        CELL_GATHER_OBJECTS_RECURSIVE.call(c, rec_ret(k))?;
        skip_cell(c, k);
        return Ok(());
    }
    gather_node(c, k)?;
    load_next_tree(c, k);
    Ok(())
}

/// Octant 7: like [`gather_cell`], but every exit goes to the epilogue (no
/// next cell, no skip charge), and its crossing branch clears `a1` first and
/// charges 6 cycles.
fn gather_last_cell(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    let empty = c.g[V0][0] == 0;
    let cell = (c.g[S4][0] as u32).wrapping_add(CELL_SIZE * 7);
    c.g[S2][0] = sx(cell);
    if empty {
        return Ok(());
    }
    c.cycles += 8;
    c.g[S1][0] = Addr(cell.wrapping_add(CELL_TREE)).lw(c);
    c.g[S3][0] = sx(sp(c).wrapping_add(FLAGS + 7));
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S5][0];
    call_ptr(c, PROX_FN, cell_ret(PROX_RET, 7))?;
    c.g[V1][0] = c.g[V0][0];
    let hit = c.g[V1][0] != 0;
    c.g[V0][0] = 2;
    if !hit {
        return gather_node(c, 7);
    }
    c.cycles += 2;
    c.g[A1][0] = c.g[S0][0];
    if c.g[V1][0] != c.g[V0][0] {
        return Ok(());
    }
    c.cycles += 6;
    c.g[A0][0] = c.g[S3][0];
    c.g[A3][0] = c.g[S5][0];
    c.g[T0][0] = c.g[S1][0];
    c.g[T1][0] = c.g[S2][0];
    c.g[A2][0] = c.g[A1][0];
    CELL_GATHER_OBJECTS_RECURSIVE.call(c, REC_LAST_RET)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{fakes, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word in the machine's RAM: the fakes forward to the real bodies
    /// unless it holds `OC563_MAGIC`.
    const OC563_GATE: u32 = DATA + 0x8_0000;
    const OC563_MAGIC: u32 = 0x5633_0ba8;
    /// The proximity answer for each of the eight cells (one word each).
    const OC563_PROX: u32 = DATA + 0x8_0100;
    const OC563_SELF: u32 = DATA + 0x8_1000;
    const OC563_GRID: u32 = DATA + 0x8_2000;
    /// Node `k` is cell `k`'s tree; its children are leaves below `OC563_LEAVES`.
    const OC563_NODES: u32 = DATA + 0x8_4000;
    const OC563_LEAVES: u32 = DATA + 0x8_8000;
    const OC563_BOX: u32 = DATA + 0x8_c000;

    static OC563_PREV_PROX: AtomicUsize = AtomicUsize::new(0);
    static OC563_PREV_CGO: AtomicUsize = AtomicUsize::new(0);
    static OC563_PREV_REC: AtomicUsize = AtomicUsize::new(0);

    /// The proximity test, gated: answers the fixture's word for the cell
    /// (the cell pointer is `a0`), through the logging fake.
    fn oc563_prox_fake(c: &mut Cpu) {
        if c.bus.read32(OC563_GATE) != OC563_MAGIC {
            return fakes::fall_back(c, &OC563_PREV_PROX, ee::s10::f_00328360__generated);
        }
        let k = (c.g[A0][0] as u32).wrapping_sub(OC563_GRID) / 0x14;
        let r = c.bus.read32(OC563_PROX + 4 * (k & 7));
        let ret = c.g[RA][0] as u32;
        fakes::fake(c, 0x0032_8360, ret);
        if c.pc == ret {
            c.g[V0][0] = r as u64;
        }
    }

    /// The callback (`Cell_GatherObjects`), gated, logged.
    fn oc563_cgo_fake(c: &mut Cpu) {
        if c.bus.read32(OC563_GATE) != OC563_MAGIC {
            return fakes::fall_back(c, &OC563_PREV_CGO, ee::s11::f_003309d8__generated);
        }
        fakes::fake_call(c, 0x0033_09d8);
    }

    /// The recursive collector, gated, logged.
    fn oc563_rec_fake(c: &mut Cpu) {
        if c.bus.read32(OC563_GATE) != OC563_MAGIC {
            return fakes::fall_back(c, &OC563_PREV_REC, ee::s11::f_00338920__generated);
        }
        fakes::fake_call(c, 0x0033_8920);
    }

    fn oc563_install() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00328360),
            oc563_prox_fake,
            &OC563_PREV_PROX,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_003309D8),
            oc563_cgo_fake,
            &OC563_PREV_CGO,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00338920),
            oc563_rec_fake,
            &OC563_PREV_REC,
        );
    }

    #[test]
    fn overlap_list_gather_objects_matches() {
        oc563_install();
        fakes::covered(
            ee::s11::f_00330ba8__generated,
            overlap_list_gather_objects,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0563_0ba8, 24);
                c.bus.write32(OC563_GATE, OC563_MAGIC);
                c.bus.write32(OC563_SELF, OC563_GRID);
                c.bus.write32(OC563_SELF + 8, r.next_u32());
                c.bus.write32(OC563_SELF + 0x10c, r.next_u32());
                c.bus.write32(OC563_SELF + 0x210, r.next_u32());
                c.bus.write32(OC563_GRID + 0xa0, r.next_u32());
                for k in 0..8u32 {
                    let cell = OC563_GRID + 0x14 * k;
                    let node = OC563_NODES + 0x100 * k;
                    let has_tree = r.next_u32() % 4 != 0;
                    c.bus.write32(cell + 0x10, if has_tree { node } else { 0 });
                    c.bus.write32(OC563_PROX + 4 * k, r.next_u32() % 3);
                    for j in 0..8u32 {
                        let child = if r.next_u32() % 3 != 0 {
                            OC563_LEAVES + 0x100 * (8 * k + j)
                        } else {
                            0
                        };
                        c.bus.write32(node + 4 * j, child);
                    }
                }
                c.g[A0][0] = sx(OC563_SELF);
                c.g[A1][0] = sx(OC563_BOX);
                c.pc = 0x0033_0ba8;
            },
        );
    }
}
