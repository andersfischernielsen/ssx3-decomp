//! Staging module oc564 (wave wave232): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc564`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

/// The stack frame of `Nash_CollideObjects`.
const NASH_FRAME: u32 = 0x100;
/// The query box copied onto the stack (two 16-byte quads).
const NASH_BOX_OFF: u32 = 0x20;
/// The `int` result counter.
const NASH_COUNT_OFF: u32 = 0x40;
/// `SpatialKey_ClassifyAABB(node, box)` (0x328360). The generated branches
/// read its result as: 0 (test the cell's objects and children), 1 (nothing),
/// 2 (wholly inside: recurse into the whole subtree). Inferred from the
/// branches, not from the callee's own source.
const NASH_CLASSIFY: u32 = 0x0032_8360;

/// `Cell_CollideObjectEx(ctx, obj)` (0x336d40).
pub(crate) const NASH_COLLIDE_EX: Callee = Callee {
    addr: 0x0033_6d40,
    f: ee::s11::f_00336d40,
};

/// `Cell_CollideObjectsSubtree(cell, ctx)` (0x3410c0).
pub(crate) const NASH_COLLIDE_SUBTREE: Callee = Callee {
    addr: 0x0034_10c0,
    f: ee::s11::f_003410c0,
};

/// `Cell_CollideObjectsRecursive(out, cell, node, ctx, ctx, box)` (0x33dfd8).
pub(crate) const NASH_COLLIDE_RECURSIVE: Callee = Callee {
    addr: 0x0033_dfd8,
    f: ee::s11::f_0033dfd8,
};

/// One of the eight sub-nodes the function walks in order: node `k` lives at
/// `nash + node_off` and its cell pointer at +0x10. The phase's stack slots,
/// call return addresses and cycle charges are the generated body's own.
#[derive(Clone, Copy)]
struct NashPhase {
    /// Offset of the node inside the nash object.
    node_off: u32,
    /// Stack byte the recursion writes its flag through (`sp + out_off`).
    out_off: u32,
    /// The register that holds `out` (s0, or s3 for the second node).
    out_reg: usize,
    /// Cycles charged on entry, before the cell test.
    entry_cycles: u64,
    /// Cycles charged before the classify call.
    box_cycles: u64,
    /// Cycles charged when the cell has objects (on top of the 3 always).
    list_else: u64,
    /// Return address of the classify call.
    box_ra: u32,
    /// Return address of the object call (the list loop).
    list_ra: u32,
    /// Back-edge pc of the object loop (event delivery).
    list_head: u32,
    /// Return address of the first child call; child `i` returns at `+0x14 * i`.
    child_ra: u32,
    /// Return address of the recursion.
    rec_ra: u32,
    /// Offset of the next node (its cell is the exit value); `None` returns
    /// the object count instead.
    next_off: Option<u32>,
}

const NASH_PHASES: [NashPhase; 8] = [
    NashPhase {
        node_off: 0x00,
        out_off: 0x44,
        out_reg: S0,
        entry_cycles: 0,
        box_cycles: 7,
        list_else: 2,
        box_ra: 0x0033_6fa8,
        list_ra: 0x0033_6fd0,
        list_head: 0x0033_6fc8,
        child_ra: 0x0033_6ff0,
        rec_ra: 0x0033_70a8,
        next_off: Some(0x14),
    },
    NashPhase {
        node_off: 0x14,
        out_off: 0x45,
        out_reg: S3,
        entry_cycles: 10,
        box_cycles: 8,
        list_else: 2,
        box_ra: 0x0033_70e8,
        list_ra: 0x0033_7110,
        list_head: 0x0033_7108,
        child_ra: 0x0033_7130,
        rec_ra: 0x0033_71e8,
        next_off: Some(0x28),
    },
    NashPhase {
        node_off: 0x28,
        out_off: 0x46,
        out_reg: S0,
        entry_cycles: 2,
        box_cycles: 7,
        list_else: 1,
        box_ra: 0x0033_7204,
        list_ra: 0x0033_7228,
        list_head: 0x0033_7220,
        child_ra: 0x0033_7248,
        rec_ra: 0x0033_7300,
        next_off: Some(0x3c),
    },
    NashPhase {
        node_off: 0x3c,
        out_off: 0x47,
        out_reg: S0,
        entry_cycles: 2,
        box_cycles: 7,
        list_else: 1,
        box_ra: 0x0033_731c,
        list_ra: 0x0033_7340,
        list_head: 0x0033_7338,
        child_ra: 0x0033_7360,
        rec_ra: 0x0033_7418,
        next_off: Some(0x50),
    },
    NashPhase {
        node_off: 0x50,
        out_off: 0x48,
        out_reg: S0,
        entry_cycles: 2,
        box_cycles: 7,
        list_else: 1,
        box_ra: 0x0033_7434,
        list_ra: 0x0033_7458,
        list_head: 0x0033_7450,
        child_ra: 0x0033_7478,
        rec_ra: 0x0033_7530,
        next_off: Some(0x64),
    },
    NashPhase {
        node_off: 0x64,
        out_off: 0x49,
        out_reg: S0,
        entry_cycles: 2,
        box_cycles: 7,
        list_else: 1,
        box_ra: 0x0033_754c,
        list_ra: 0x0033_7570,
        list_head: 0x0033_7568,
        child_ra: 0x0033_7590,
        rec_ra: 0x0033_7648,
        next_off: Some(0x78),
    },
    NashPhase {
        node_off: 0x78,
        out_off: 0x4a,
        out_reg: S0,
        entry_cycles: 2,
        box_cycles: 8,
        list_else: 2,
        box_ra: 0x0033_7668,
        list_ra: 0x0033_7690,
        list_head: 0x0033_7688,
        child_ra: 0x0033_76b0,
        rec_ra: 0x0033_7768,
        next_off: Some(0x8c),
    },
    NashPhase {
        node_off: 0x8c,
        out_off: 0x4b,
        out_reg: S0,
        entry_cycles: 2,
        box_cycles: 8,
        list_else: 2,
        box_ra: 0x0033_7788,
        list_ra: 0x0033_77b0,
        list_head: 0x0033_77a8,
        child_ra: 0x0033_77d0,
        rec_ra: 0x0033_7888,
        next_off: None,
    },
];

/// The value a phase leaves in v0: the next node's cell, or the object
/// count after the last node.
fn nash_exit_value(c: &mut Cpu, nash: u32, sp_: u32, ph: &NashPhase) -> u64 {
    match ph.next_off {
        Some(off) => Addr(nash.wrapping_add(off).wrapping_add(0x10)).lw(c),
        None => Addr(sp_.wrapping_add(NASH_COUNT_OFF)).lw(c),
    }
}

/// One node's pass: classify the node's box, then either recurse into the
/// whole subtree (inside), or test the cell's objects and its eight children
/// (partly overlapping). `cell` is the node's cell pointer (already tested
/// for the entry in v0 or s2).
fn nash_collide_phase(c: &mut Cpu, nash: u32, ph: &NashPhase, cell: u64) -> Flow {
    let sp_ = sp(c);
    c.cycles += ph.entry_cycles;
    if cell == 0 {
        c.cycles += 1;
        c.g[V0][0] = nash_exit_value(c, nash, sp_, ph);
        return Ok(());
    }
    let node = nash.wrapping_add(ph.node_off);
    let out = sp_.wrapping_add(ph.out_off);
    let boxp = sp_.wrapping_add(NASH_BOX_OFF);
    c.cycles += ph.box_cycles;
    c.g[ph.out_reg][0] = sx(out);
    c.g[S2][0] = cell;
    c.g[A0][0] = sx(node);
    c.g[A1][0] = sx(boxp);
    call_ptr(c, NASH_CLASSIFY, ph.box_ra)?;
    c.g[V1][0] = c.g[V0][0];
    c.g[V0][0] = 2;
    if c.g[V1][0] != 0 {
        c.cycles += 2;
        if c.g[V1][0] != c.g[V0][0] {
            c.g[V0][0] = nash_exit_value(c, nash, sp_, ph);
            return Ok(());
        }
        // Wholly inside: the recursion takes the whole subtree.
        c.cycles += 7;
        c.g[A0][0] = sx(out);
        c.g[T0][0] = c.g[S2][0];
        c.g[T1][0] = sx(node);
        c.g[A1][0] = sx(sp_);
        c.g[A2][0] = sx(sp_);
        c.g[A3][0] = sx(boxp);
        NASH_COLLIDE_RECURSIVE.call(c, ph.rec_ra)?;
        c.cycles += 1;
        c.g[V0][0] = nash_exit_value(c, nash, sp_, ph);
        return Ok(());
    }

    // Partly overlapping: the cell's own objects, then its eight children.
    let cell32 = cell as u32;
    c.cycles += 3;
    c.g[S0][0] = Addr(cell32.wrapping_add(0x20)).lw(c);
    if c.g[S0][0] == 0 {
        c.g[A0][0] = Addr(cell32).lw(c);
    } else {
        c.cycles += ph.list_else;
        c.g[A1][0] = c.g[S0][0];
        loop {
            c.cycles += 5;
            c.g[A0][0] = sx(sp_);
            NASH_COLLIDE_EX.call(c, ph.list_ra)?;
            c.g[S0][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[A1][0] = c.g[S0][0];
            if c.g[S0][0] != 0 {
                if event_due(c) {
                    deliver_event_here(c, ph.list_head);
                }
                continue;
            }
            break;
        }
        c.cycles += 1;
        c.g[A0][0] = Addr(cell32).lw(c);
    }
    for i in 0..8u32 {
        c.cycles += 2;
        if c.g[A0][0] == 0 {
            if i == 7 {
                break;
            }
            c.g[A0][0] = Addr(cell32.wrapping_add(4 * (i + 1))).lw(c);
            continue;
        }
        c.cycles += if i == 7 { 4 } else { 3 };
        c.g[A1][0] = sx(sp_);
        NASH_COLLIDE_SUBTREE.call(c, ph.child_ra.wrapping_add(0x14 * i))?;
        if i == 7 {
            break;
        }
        c.g[A0][0] = Addr(cell32.wrapping_add(4 * (i + 1))).lw(c);
    }
    c.g[V0][0] = nash_exit_value(c, nash, sp_, ph);
    Ok(())
}

/// `Nash_CollideObjects` (0x336f30, 2444 bytes): collides a query box with
/// the nash's eight sub-nodes in turn (each with its cell's objects and
/// children, or its whole subtree when the box holds it), counting the hits
/// through the stack context. Returns the hit count in v0.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
/// Not installed: the diff leaves one RAM word different (see the wave notes).
#[allow(dead_code)]
pub fn nash_collide_objects(c: &mut Cpu) {
    guest(c, nash_collide_objects_body);
}

fn nash_collide_objects_body(c: &mut Cpu) -> Flow {
    c.cycles += 26;
    let frame = Frame::open(c, NASH_FRAME);
    frame.save(c, S1, 0xe0);
    c.g[V0][0] = sx(sp(c).wrapping_add(NASH_COUNT_OFF));
    frame.save(c, S4, 0xb0);
    c.g[S1][0] = c.g[SP][0];
    frame.save(c, S0, 0xf0);
    c.g[S4][0] = c.g[A0][0];
    frame.save(c, S2, 0xd0);
    frame.save(c, S3, 0xc0);
    frame.save(c, S5, 0xa0);
    frame.save(c, S6, 0x90);
    frame.save(c, S7, 0x80);
    frame.save(c, FP, 0x70);
    frame.save_ra(c, 0x60);

    // The query box, copied to the stack; the context fields.
    let query = c.g[A1][0] as u32;
    c.lq(A0, query.wrapping_add(0x20));
    c.lq(V1, query.wrapping_add(0x30));
    let nash = c.g[S4][0] as u32;
    c.g[S2][0] = Addr(nash.wrapping_add(0x10)).lw(c);
    let sp_ = sp(c);
    let a2 = c.g[A2][0];
    c.sw(sp_.wrapping_add(4), a2);
    let a3 = c.g[A3][0];
    c.sw(sp_.wrapping_add(8), a3);
    let v0 = c.g[V0][0];
    c.sw(sp_.wrapping_add(0xc), v0);
    let t0 = c.g[T0][0];
    c.sw(sp_.wrapping_add(0x10), t0);
    c.sq(V1, sp_.wrapping_add(NASH_BOX_OFF));
    c.sq(A0, sp_.wrapping_add(0x30));
    let a1 = c.g[A1][0];
    c.sw(sp_, a1);
    c.sw(sp_.wrapping_add(NASH_COUNT_OFF), 0);

    // Node 0: the nash itself.
    let cell = c.g[S2][0];
    nash_collide_phase(c, nash, &NASH_PHASES[0], cell)?;

    // Node 1 (its entry also stores the addresses of nodes 6 and 7 on the
    // stack and loads the node pointers the later phases use).
    let v1 = sx(nash.wrapping_add(0x78));
    c.g[V1][0] = v1;
    c.sw(sp_.wrapping_add(0x4c), v1);
    c.g[S0][0] = sx(nash.wrapping_add(0x14));
    let v1 = sx(nash.wrapping_add(0x8c));
    c.g[V1][0] = v1;
    c.g[S5][0] = sx(nash.wrapping_add(0x28));
    c.g[S6][0] = sx(nash.wrapping_add(0x3c));
    c.g[S7][0] = sx(nash.wrapping_add(0x50));
    c.g[FP][0] = sx(nash.wrapping_add(0x64));
    c.sw(sp_.wrapping_add(0x50), v1);
    let cell = c.g[V0][0];
    nash_collide_phase(c, nash, &NASH_PHASES[1], cell)?;

    // Nodes 2 to 5.
    let cell = c.g[V0][0];
    nash_collide_phase(c, nash, &NASH_PHASES[2], cell)?;
    let cell = c.g[V0][0];
    nash_collide_phase(c, nash, &NASH_PHASES[3], cell)?;
    let cell = c.g[V0][0];
    nash_collide_phase(c, nash, &NASH_PHASES[4], cell)?;
    let cell = c.g[V0][0];
    nash_collide_phase(c, nash, &NASH_PHASES[5], cell)?;

    // Node 6: its pointer comes from the stack slot.
    let cell = c.g[V0][0];
    let node7 = Addr(sp_.wrapping_add(0x4c)).lw(c);
    c.g[V0][0] = node7;
    nash_collide_phase(c, nash, &NASH_PHASES[6], cell)?;

    // Node 7: its pointer comes from the stack slot, in v1.
    let cell = c.g[V0][0];
    let node8 = Addr(sp_.wrapping_add(0x50)).lw(c);
    c.g[V1][0] = node8;
    nash_collide_phase(c, nash, &NASH_PHASES[7], cell)?;

    c.cycles += 12;
    frame.restore(c, S0, 0xf0);
    frame.restore(c, S1, 0xe0);
    frame.restore(c, S2, 0xd0);
    frame.restore(c, S3, 0xc0);
    frame.restore(c, S4, 0xb0);
    frame.restore(c, S5, 0xa0);
    frame.restore(c, S6, 0x90);
    frame.restore(c, S7, 0x80);
    frame.restore(c, FP, 0x70);
    frame.restore_ra(c, 0x60);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, fill, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;

    /// The gate word: the fakes do their scripted work only when it is set.
    const NASH_TEST_GATE: u32 = DATA + 0x3000;
    const NASH_TEST_MAGIC: u32 = 0x0564_0564;
    const NASH_TEST_NASH: u32 = DATA + 0x1000;
    const NASH_TEST_QUERY: u32 = DATA + 0x2000;
    const NASH_TEST_HITS: u32 = DATA + 0x2400;
    /// Cell k (for node k) at `CELLS + 0x100 * k`.
    const NASH_TEST_CELLS: u32 = DATA + 0x4000;
    /// A zeroed cell: an empty subtree for the real callees to walk.
    const NASH_TEST_EMPTY: u32 = DATA + 0x5000;
    /// Sixteen objects, linked through their first word.
    const NASH_TEST_OBJS: u32 = DATA + 0x6000;
    /// Node offsets inside the nash object.
    const NASH_TEST_NODES: [u32; 8] = [0x00, 0x14, 0x28, 0x3c, 0x50, 0x64, 0x78, 0x8c];

    fn nash_test_gated(c: &mut Cpu) -> bool {
        c.bus.read32(NASH_TEST_GATE) == NASH_TEST_MAGIC
    }

    /// Scripted classify: the node's first word is the result (0, 1 or 2).
    fn nash_test_fake_classify(c: &mut Cpu) {
        if !nash_test_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(nash_test_fake_classify),
                ee::s10::f_00328360__generated,
            );
        }
        let node = c.g[A0][0] as u32;
        let result = c.bus.read32(node) as u64;
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, NASH_CLASSIFY, ra);
        if c.pc == ra {
            c.g[V0][0] = result;
        }
    }

    /// The recursion into a wholly inside subtree is faked (it is a whole
    /// traversal of its own); the object and child calls run for real on
    /// the empty cells.
    fn nash_test_fake_recursive(c: &mut Cpu) {
        if !nash_test_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(nash_test_fake_recursive),
                ee::s11::f_0033dfd8__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, NASH_COLLIDE_RECURSIVE.addr, ra);
    }

    /// Puts the gated fakes into their override slots (once per process).
    fn nash_test_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00328360),
            nash_test_fake_classify,
            fakes::prev_cell(nash_test_fake_classify),
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_0033DFD8),
            nash_test_fake_recursive,
            fakes::prev_cell(nash_test_fake_recursive),
        );
    }

    /// Scrambled registers, the gate on, the query box and the nash object
    /// with random node results, cells, children and object lists.
    fn nash_test_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x0564_0000 ^ seed as u64);
        scramble(c, &mut r);
        c.g[SP][0] = sx(crate::difftest::SP);
        c.pc = 0x0033_6f30;
        c.bus.write32(NASH_TEST_GATE, NASH_TEST_MAGIC);
        let unwind = if seed % 9 == 0 {
            1 + seed % 5
        } else {
            u32::MAX
        };
        c.bus.write32(fakes::LOG_UNWIND_AT, unwind);
        c.g[A0][0] = sx(NASH_TEST_NASH);
        c.g[A1][0] = sx(NASH_TEST_QUERY);
        c.g[A2][0] = sx(NASH_TEST_HITS);
        c.g[A3][0] = r.next_u32() as u64;
        c.g[T0][0] = r.next_u32() as u64;
        fill(c, &mut r, NASH_TEST_QUERY, 16);
        for (k, off) in NASH_TEST_NODES.iter().enumerate() {
            let node = NASH_TEST_NASH + off;
            c.bus.write32(node, r.next_u32() % 3);
            let cell_base = NASH_TEST_CELLS + 0x100 * k as u32;
            let cell = if r.next_u32() % 4 == 0 { 0 } else { cell_base };
            c.bus.write32(node + 0x10, cell);
            for i in 0..8u32 {
                let child = if r.next_u32() % 3 == 0 {
                    0
                } else {
                    NASH_TEST_EMPTY
                };
                c.bus.write32(cell_base + 4 * i, child);
            }
            let head = if r.next_u32() % 3 == 0 {
                0
            } else {
                NASH_TEST_OBJS + 0x10 * (r.next_u32() % 16)
            };
            c.bus.write32(cell_base + 0x20, head);
        }
        for j in 0..16u32 {
            let next = if j < 15 {
                NASH_TEST_OBJS + 0x10 * (j + 1)
            } else {
                0
            };
            c.bus.write32(NASH_TEST_OBJS + 0x10 * j, next);
        }
    }
}
