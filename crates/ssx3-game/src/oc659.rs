//! Staging module oc659 (wave234): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc659`.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0034_0b18, cell_gather_type2_subtree, "Cell_GatherType2Subtree"),
    (0x0034_0dc0, cell_gather_all_subtree, "Cell_GatherAllSubtree"),
    (0x0034_4898, crowd_man3d_group_remove_section, "CrowdMan3d_GroupRemoveSection"),
];

/// The eight child slots of a cell (+0x0 .. +0x1c), each descended into when
/// non-null with the output in A1. `self_reg` holds the cell, `out_reg` the
/// output; `rets` are the return addresses of the eight call sites. The
/// slot loads, the per-slot cycle charges and the final `lq s0` match the
/// generated blocks one for one.
fn gather_children(
    c: &mut Cpu,
    frame: Frame,
    self_reg: usize,
    out_reg: usize,
    callee: Callee,
    rets: [u32; 8],
) -> Flow {
    for k in 0..8u32 {
        if k == 0 {
            c.cycles += 3;
            c.g[A0][0] = Addr(c.g[self_reg][0] as u32).lw(c);
        } else {
            c.cycles += 2;
        }
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = c.g[out_reg][0];
            callee.call(c, rets[k as usize])?;
        }
        if k < 7 {
            let next = (c.g[self_reg][0] as u32).wrapping_add(4 * (k + 1));
            c.g[A0][0] = Addr(next).lw(c);
        } else {
            frame.restore(c, 16, 0x20);
        }
    }
    Ok(())
}

/// Return addresses of the child call sites of `nash_Cell_descend1`
/// (0x340b18), from its `RA = sx(...)` lines.
const TYPE2_CHILD_RETS: [u32; 8] = [
    0x0034_0b7c,
    0x0034_0b90,
    0x0034_0ba4,
    0x0034_0bb8,
    0x0034_0bcc,
    0x0034_0be0,
    0x0034_0bf4,
    0x0034_0c08,
];

/// Return addresses of the child call sites of `nash_Cell_descend3`
/// (0x340dc0).
const ALL_CHILD_RETS: [u32; 8] = [
    0x0034_0efc,
    0x0034_0f10,
    0x0034_0f24,
    0x0034_0f38,
    0x0034_0f4c,
    0x0034_0f60,
    0x0034_0f74,
    0x0034_0f88,
];

/// `void nash_Cell_descend1(nash_Cell* self, sOctCollect* out)` (0x340b18):
/// appends the cell's own list (head at +0x28, each node's first word the
/// next node) to `out` (items array at +0, count word address at +4), then
/// descends into the eight child cells with the same `out`.
pub fn cell_gather_type2_subtree(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A2][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x28)).lw(c);
        let empty = c.g[A2][0] == 0;
        c.g[S0][0] = c.g[A1][0];
        if !empty {
            c.cycles += 2;
            c.g[A1][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lw(c);
            loop {
                c.cycles += 10;
                c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                let item = c.g[A2][0];
                Addr(c.g[V0][0] as u32).sw(c, item);
                let count = c.g[V1][0];
                Addr(c.g[A1][0] as u32).sw(c, count);
                c.g[A2][0] = Addr(c.g[A2][0] as u32).lw(c);
                if c.g[A2][0] == 0 {
                    break;
                }
                c.g[A1][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0034_0b40);
                }
            }
        }
        let callee = Callee {
            addr: 0x0034_0b18,
            f: ee::s11::f_00340b18,
        };
        gather_children(c, frame, S1, S0, callee, TYPE2_CHILD_RETS)?;
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `T box_test` of `nash_Cell_descend3` (the block at 0x340e78): six plane
/// tests of the cell's box at +0x158..+0x16c against the object's box at
/// `obj` (+0, +4, +8, +0x10, +0x14, +0x18). Each step loads two floats into
/// f1 then f0 (leaving them there), compares and latches the FCC bit; the
/// first failing step stops the chain. Returns true when all six passed.
fn gather_box_test(c: &mut Cpu, node: u32, obj: u32) -> bool {
    // (cycles, f1 at (is_obj, offset), f0 at (is_obj, offset), swapped compare)
    const STEPS: [(u64, (bool, u32), (bool, u32), bool); 6] = [
        (6, (false, 0x164), (true, 0x0), false),
        (5, (true, 0x10), (false, 0x158), false),
        (5, (true, 0x4), (false, 0x168), true),
        (5, (true, 0x14), (false, 0x15c), false),
        (5, (true, 0x8), (false, 0x16c), true),
        (6, (true, 0x18), (false, 0x160), false),
    ];
    for (cycles, f1, f0, swapped) in STEPS {
        c.cycles += cycles;
        let base1 = if f1.0 { obj } else { node };
        let v1 = Addr(base1.wrapping_add(f1.1)).read_fpu(c);
        abi::set_fpr(c, 1, v1);
        let base0 = if f0.0 { obj } else { node };
        let a0 = base0.wrapping_add(f0.1);
        let v0 = Addr(a0).read_fpu(c);
        abi::set_fpr(c, 0, v0);
        let pass = if swapped { v1.le(v0) } else { v0.le(v1) };
        abi::set_fcc(c, pass);
        if !pass {
            return false;
        }
    }
    true
}

/// `void nash_Cell_descend3(nash_Cell* self, OverlapEverythingCallback* ctx)`
/// (0x340dc0): hands the cell's own objects (the list at +0x24, filtered by
/// the box test, items at ctx+0x10) and its list at +0x28 (items at
/// ctx+0x20) to the callback collector, then descends into the children.
pub fn cell_gather_all_subtree(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        Callee {
            addr: 0x0033_09d8,
            f: ee::s11::f_003309d8,
        }
        .call(c, 0x0034_0de4)?;
        c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x24)).lw(c);
        let empty = c.g[A2][0] == 0;
        c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
        if !empty {
            c.cycles += 2;
            c.g[V0][0] = Addr((c.g[A3][0] as u32).wrapping_add(0xc)).lw(c);
            loop {
                let (node, obj) = (c.g[A2][0] as u32, c.g[V0][0] as u32);
                let pass = gather_box_test(c, node, obj);
                c.g[V1][0] = pass as u64;
                c.cycles += 2;
                if c.g[V1][0] == 0 {
                    c.g[A2][0] = Addr(c.g[A2][0] as u32).lw(c);
                } else {
                    c.cycles += 9;
                    c.g[A1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x4)).lw(c);
                    c.g[A0][0] = Addr(c.g[A3][0] as u32).lw(c);
                    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                    let item = c.g[A2][0];
                    Addr(c.g[V0][0] as u32).sw(c, item);
                    let count = c.g[V1][0];
                    Addr(c.g[A1][0] as u32).sw(c, count);
                    c.g[A2][0] = Addr(c.g[A2][0] as u32).lw(c);
                }
                c.cycles += 2;
                if c.g[A2][0] == 0 {
                    break;
                }
                c.g[V0][0] = Addr((c.g[A3][0] as u32).wrapping_add(0xc)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0034_0df8);
                }
            }
        }
        c.cycles += 3;
        c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x28)).lw(c);
        let empty2 = c.g[A2][0] == 0;
        c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x20));
        if !empty2 {
            c.cycles += 2;
            c.g[A1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x4)).lw(c);
            loop {
                c.cycles += 10;
                c.g[A0][0] = Addr(c.g[A3][0] as u32).lw(c);
                c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                let item = c.g[A2][0];
                Addr(c.g[V0][0] as u32).sw(c, item);
                let count = c.g[V1][0];
                Addr(c.g[A1][0] as u32).sw(c, count);
                c.g[A2][0] = Addr(c.g[A2][0] as u32).lw(c);
                if c.g[A2][0] == 0 {
                    break;
                }
                c.g[A1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x4)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0034_0ec0);
                }
            }
        }
        let callee = Callee {
            addr: 0x0034_0dc0,
            f: ee::s11::f_00340dc0,
        };
        gather_children(c, frame, S0, S1, callee, ALL_CHILD_RETS)?;
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The loop over the group's entries in `crowd_man3d_group_remove_section`
/// once an entry of the wanted section is found (the block at 0x3448b8 +
/// the inner search at 0x344910). Returns true when the generated body
/// leaves through `break 'b_3449b8` (one extra cycle), false when it leaves
/// through `break 'b_3449bc` (after the group reset).
fn remove_section_entries(c: &mut Cpu) -> Result<bool, Unwound> {
    c.g[S3][0] = 0xffff_ffff_ffff_0000;
    c.g[S4][0] = 0xffff_ffff_ffff_0000;
    c.g[S0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
    c.g[A0][0] = 0;
    c.g[S3][0] |= 0xfffb;
    c.g[S4][0] |= 0xffff;
    c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
    loop {
        c.cycles += 4;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
        let t = c.g[V0][0] != c.g[S0][0];
        c.g[V1][0] = ((c.g[A0][0] as i64) < 32) as u64;
        if !t {
            // 'b_344974: the lookup of this section's record.
            c.cycles += 10;
            call_ptr(c, 0x002d_1bd8, 0x0034_4928)?;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V0][0] = c.g[S0][0] & 0xff;
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
            let found = c.g[A0][0] != 0;
            c.g[V0][0] = sx((c.g[S0][0] as u32) >> 8);
            if found {
                // 'b_344970
                c.cycles += 7;
                c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x1c)).lw(c);
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
                c.g[V1][0] = c.g[V0][0];
            } else {
                c.cycles += 2;
                c.g[V1][0] = 0;
            }
            // The count of the group's entries, and the entry's flag word.
            c.cycles += 9;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x8)).lw(c);
            c.g[V0][0] &= c.g[S3][0];
            c.g[V0][0] |= 0x2;
            let flags = c.g[V0][0];
            Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, flags);
            let slot_value = c.g[S4][0];
            Addr(c.g[S1][0] as u32).sw(c, slot_value);
            c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            let left = (c.g[V0][0] as i64) > 0;
            let remaining = c.g[V0][0];
            Addr(c.g[S2][0] as u32).sw(c, remaining);
            if left {
                return Ok(true);
            }
            c.cycles += 6;
            c.g[A0][0] = c.g[S2][0];
            Callee {
                addr: 0x0034_4800,
                f: ee::s11::f_00344800,
            }
            .call(c, 0x0034_49a0)?;
            c.g[A0][0] = c.g[S2][0];
            Callee {
                addr: 0x0034_42e0,
                f: ee::s11::f_003442e0,
            }
            .call(c, 0x0034_49a8)?;
            return Ok(false);
        }
        // 'b_3449b0 tail: no match in this slot, keep searching while A0 < 32.
        c.cycles += 2;
        let more = c.g[V1][0] != 0;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(8));
        if !more {
            return Ok(true);
        }
        if event_due(c) {
            deliver_event_here(c, 0x0034_4910);
        }
    }
}

/// `void CrowdMan3d_GroupRemoveSection(CrowdGroup* g, u8 section)`
/// (0x344898): removes every entry of `section` from the group's entry
/// table (32 entries of 8 bytes, the word at +0xc of each is the entry's
/// key), dropping each one's flags in the shared record. When the count at
/// the group's +0 falls to zero, the group frees its meshes and resets.
pub fn crowd_man3d_group_remove_section(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x80);
        c.g[V1][0] = 0;
        frame.save(c, 18, 0x50);
        frame.save(c, 22, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 16, 0x70);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, 17, 0x60);
        frame.save(c, 19, 0x40);
        frame.save(c, 20, 0x30);
        frame.save(c, 21, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[V1][0] as u32) << 3);
        loop {
            c.cycles += 6;
            c.g[S5][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
            c.g[V0][0] = !c.g[V0][0];
            let free_slot = c.g[V0][0] == 0;
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            if free_slot {
                c.cycles += 1;
                c.g[V1][0] = c.g[S5][0];
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lbu(c);
                if c.g[V0][0] != c.g[S6][0] {
                    c.g[V1][0] = c.g[S5][0];
                } else {
                    c.cycles += 7;
                    let plus_one = remove_section_entries(c)?;
                    if plus_one {
                        c.cycles += 1;
                    }
                    c.g[V1][0] = c.g[S5][0];
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 32) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[A0][0] = sx((c.g[V1][0] as u32) << 3);
            if event_due(c) {
                deliver_event_here(c, 0x0034_48d0);
            }
        }
        c.cycles += 10;
        frame.restore(c, 16, 0x70);
        frame.restore(c, 17, 0x60);
        frame.restore(c, 18, 0x50);
        frame.restore(c, 19, 0x40);
        frame.restore(c, 20, 0x30);
        frame.restore(c, 21, 0x20);
        frame.restore(c, 22, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    // Gather fixtures.
    const OUT: u32 = DATA + 0x3_0000;
    const ITEMS_A: u32 = DATA + 0x3_1000;
    const ITEMS_B: u32 = DATA + 0x3_2000;
    const CNT_A: u32 = DATA + 0x3_3000;
    const CNT_B: u32 = DATA + 0x3_3010;
    const OBJ: u32 = DATA + 0x3_8000;
    const CELL: u32 = DATA + 0x4_0000;
    const LIST: u32 = DATA + 0x6_0000;
    const GATHER_CTRL: u32 = DATA + 0x3_4000;
    const GATHER_MAGIC: u32 = 0x4741_5448;
    static GATHER_PREV: AtomicUsize = AtomicUsize::new(0);

    /// Gate of the object gatherer fake: `Cell_GatherObjects` only records
    /// nothing while `GATHER_CTRL` holds the magic word.
    fn gather_fake_objects(c: &mut Cpu) {
        if c.bus.read32(GATHER_CTRL) != GATHER_MAGIC {
            return fakes::fall_back(c, &GATHER_PREV, ee::s11::f_003309d8__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    fn gather_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_003309D8),
            gather_fake_objects,
            &GATHER_PREV,
        );
    }

    /// Writes a list of `n` nodes at `base` (each node's first word is the
    /// next node). Returns the head.
    fn write_list(c: &mut Cpu, base: u32, n: u32) -> u32 {
        for j in 0..n {
            let next = if j + 1 < n { base + 0x40 * (j + 1) } else { 0 };
            c.bus.write32(base + 0x40 * j, next);
        }
        if n == 0 {
            0
        } else {
            base
        }
    }

    /// One cell: eight child words, two lists (+0x24, +0x28) and the six
    /// box floats at +0x158..+0x16c. With `seed % 3 == 0` the box of the
    /// object passes every plane test.
    fn write_cell(c: &mut Cpu, r: &mut Rng, seed: u32, addr: u32, kids: &[u32; 8], lists: u32) {
        for k in 0..8u32 {
            c.bus.write32(addr + 4 * k, kids[k as usize]);
        }
        let head1 = write_list(c, lists, (seed + addr) % 4);
        let head2 = write_list(c, lists + 0x200, (seed / 4) % 3);
        c.bus.write32(addr + 0x24, head1);
        c.bus.write32(addr + 0x28, head2);
        for k in 0..6u32 {
            let bits = if seed % 3 == 0 {
                [0, 0, 0, 0x42c8_0000, 0x42c8_0000, 0x42c8_0000][k as usize]
            } else {
                r.f32(4.0)
            };
            c.bus.write32(addr + 0x158 + 4 * k, bits);
        }
    }

    /// The object box at `OBJ` (passing values when `seed % 3 == 0`).
    fn write_object(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let offs = [0u32, 4, 8, 0x10, 0x14, 0x18];
        for (k, off) in offs.iter().enumerate() {
            let pass = [0, 0, 0, 0x42c8_0000u32, 0x42c8_0000, 0x42c8_0000];
            let bits = if seed % 3 == 0 { pass[k] } else { r.f32(4.0) };
            c.bus.write32(OBJ + off, bits);
        }
    }

    /// A cell tree below `CELL` (up to four children, each with an optional
    /// leaf below) and the output record `OUT`.
    fn gather_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let mut root = [0u32; 8];
        for k in 0..4u32 {
            if (seed >> k) & 1 == 1 {
                root[k as usize] = CELL + 0x200 * (k + 1);
            }
        }
        write_cell(c, r, seed, CELL, &root, LIST);
        for k in 0..4u32 {
            if root[k as usize] != 0 {
                let mut kids = [0u32; 8];
                if (seed >> (k + 4)) & 1 == 1 {
                    kids[0] = CELL + 0x2000;
                }
                write_cell(c, r, seed, root[k as usize], &kids, LIST + 0x400 * (k + 1));
            }
        }
        write_cell(c, r, seed, CELL + 0x2000, &[0; 8], LIST + 0x2000);
        write_object(c, r, seed);
        c.bus.write32(OUT, ITEMS_A);
        c.bus.write32(OUT + 4, CNT_A);
        c.bus.write32(OUT + 0x10, ITEMS_A);
        c.bus.write32(OUT + 0x14, CNT_A);
        c.bus.write32(OUT + 0x1c, OBJ);
        c.bus.write32(OUT + 0x20, ITEMS_B);
        c.bus.write32(OUT + 0x24, CNT_B);
        c.bus.write32(CNT_A, seed % 4);
        c.bus.write32(CNT_B, (seed / 4) % 3);
    }

    #[test]
    fn cell_gather_type2_subtree_matches() {
        check(ee::s11::f_00340b18__generated, cell_gather_type2_subtree, 200, &|c, seed| {
            let mut r = Rng(0x340b_1800 ^ seed as u64);
            scramble(c, &mut r);
            c.pc = 0x0034_0b18;
            gather_fixture(c, &mut r, seed);
            c.g[A0][0] = sx(CELL);
            c.g[A1][0] = sx(OUT);
        });
    }

    #[test]
    fn cell_gather_all_subtree_matches() {
        gather_install_fakes();
        check(ee::s11::f_00340dc0__generated, cell_gather_all_subtree, 200, &|c, seed| {
            let mut r = Rng(0x340d_c000 ^ seed as u64);
            scramble(c, &mut r);
            c.pc = 0x0034_0dc0;
            c.bus.write32(GATHER_CTRL, GATHER_MAGIC);
            gather_fixture(c, &mut r, seed);
            c.g[A0][0] = sx(CELL);
            c.g[A1][0] = sx(OUT);
        });
    }

    // Group-removal fixtures.
    const GROUP: u32 = DATA + 0x2_0000;
    const WORLD: u32 = DATA + 0x2_1000;
    const BE: u32 = DATA + 0x2_1100;
    const TBL: u32 = DATA + 0x2_2000;
    const ELEM: u32 = DATA + 0x2_3000;
    const TBL2: u32 = DATA + 0x2_4000;
    const FREE_CTRL: u32 = DATA + 0x2_5000;
    const RESET_CTRL: u32 = DATA + 0x2_5010;
    const GROUP_MAGIC: u32 = 0x4752_4f50;
    static FREE_PREV: AtomicUsize = AtomicUsize::new(0);
    static RESET_PREV: AtomicUsize = AtomicUsize::new(0);

    /// Gated fakes of the mesh free and the reset: they only return while
    /// their control words hold the magic, otherwise the real body runs.
    fn group_fake_free(c: &mut Cpu) {
        if c.bus.read32(FREE_CTRL) != GROUP_MAGIC {
            return fakes::fall_back(c, &FREE_PREV, ee::s11::f_00344800__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    fn group_fake_reset(c: &mut Cpu) {
        if c.bus.read32(RESET_CTRL) != GROUP_MAGIC {
            return fakes::fall_back(c, &RESET_PREV, ee::s11::f_003442e0__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    fn group_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00344800),
            group_fake_free,
            &FREE_PREV,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_003442E0),
            group_fake_reset,
            &RESET_PREV,
        );
    }

    #[test]
    fn crowd_man3d_group_remove_section_matches() {
        group_install_fakes();
        check(ee::s11::f_00344898__generated, crowd_man3d_group_remove_section, 200, &|c, seed| {
            let mut r = Rng(0x3448_9800 ^ seed as u64);
            scramble(c, &mut r);
            c.pc = 0x0034_4898;
            let sec = (seed % 3) as u32 + 1;
            c.bus.write32(FREE_CTRL, GROUP_MAGIC);
            c.bus.write32(RESET_CTRL, GROUP_MAGIC);
            // Group: count at +0, entries of 8 bytes with the key word at +0xc.
            c.bus.write32(GROUP, seed % 4);
            for i in 0..32u32 {
                let key = match r.next_u32() % 4 {
                    0 => 0xffff_ffff,
                    1 => sec,
                    _ => sec | ((r.next_u32() % 8) << 8),
                };
                c.bus.write32(GROUP + 8 * i + 0xc, key);
            }
            // World -> BE record -> section tables.
            c.bus.write32(GAME_GP.wrapping_sub(0x848), WORLD);
            c.bus.write32(WORLD, BE);
            c.bus.write32(BE + 8, TBL);
            for k in 0..256u32 {
                let elem = if (seed + k) % 3 == 0 { 0 } else { ELEM + 0x40 * (k % 4) };
                c.bus.write32(TBL + 4 * k, elem);
            }
            for e in 0..4u32 {
                c.bus.write32(ELEM + 0x40 * e + 0x1c, TBL2);
            }
            for j in 0..256u32 {
                let v = if (seed + j) % 2 == 0 { 0 } else { (j % 8) << 8 };
                c.bus.write32(TBL2 + 4 * j, v);
            }
            c.g[A0][0] = sx(GROUP);
            c.g[A1][0] = sx(sec);
        });
    }
}
