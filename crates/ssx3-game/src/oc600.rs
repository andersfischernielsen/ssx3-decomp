//! Staging module oc600 (wave l585a.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc600`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0034_3f38,
    crowd_man3d_remove_instance,
    "CrowdMan3d_RemoveInstance",
)];

/// `CrowdMan3d_GroupFreeMeshes(group)` (0x344800), generated.
const OC600_GROUP_FREE_MESHES: Callee = Callee {
    addr: 0x0034_4800,
    f: ee::s11::f_00344800,
};

/// `CrowdMan3d_GroupReset(group)` (0x3442e0), generated or replaced.
const OC600_GROUP_RESET: Callee = Callee {
    addr: 0x0034_42e0,
    f: ee::s11::f_003442e0,
};

/// `void CrowdMan3d_RemoveInstance(CrowdMan3d* self, int id)` (0x343f38).
///
/// Looks up `id` through the world's model table: the entry's group word
/// (`+0x1c`, then the word indexed by `id >> 8`) gives a block, whose
/// pointer chain (`+0x80`) holds the key that the five groups of `self` are
/// matched against (group `i` key at `self+8+i*0x1f4`). In the matching group
/// the 32-slot id list (from `self+i*0x1f4+0xc`, 8 bytes a slot) is searched
/// for `id`; the slot's id word becomes -1 and the block's flag word loses its
/// high bits. The group count (word at `self+i*0x1f4`) drops by one, and at
/// zero the group's meshes are freed and the group reset.
pub fn crowd_man3d_remove_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S4, 0x10);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[S4][0] = c.g[A1][0];

        // World -> model table -> entry for `id`.
        call_ptr(c, 0x002d_1bd8, 0x0034_3f60)?;
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        c.g[V0][0] = c.g[S4][0] & 0xff;
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x8)).lw(c);
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
        c.g[V0][0] = Addr(r32(c, V0)).lw(c);
        let entry_missing = c.g[V0][0] == 0;
        c.g[V1][0] = sx(r32(c, S4) >> 8);
        let mut group_found = false;
        if !entry_missing {
            c.cycles += 7;
            c.g[V0][0] = Addr(r32(c, V0).wrapping_add(0x1c)).lw(c);
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A0][0] = Addr(r32(c, V1)).lw(c);
            c.g[V0][0] = sx(r32(c, A0) >> 8);
            group_found = c.g[V0][0] != 0;
            c.g[V0][0] = sx(r32(c, V0) << 2);
        }
        if !group_found {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }

        // The five groups: the first whose key matches holds the id.
        c.cycles += 4;
        c.g[T0][0] = Addr(r32(c, V0).wrapping_add(0x80)).lw(c);
        c.g[A2][0] = 0;
        c.g[A3][0] = sx(r32(c, S0).wrapping_add(0x8));
        c.g[V0][0] = 0x1f4;
        loop {
            c.cycles += 7;
            c.g[A0][0] = Addr(r32(c, T0)).lw(c);
            c.mult(0, A1, A2, V0);
            c.g[A0][0] = sx(r32(c, A0) >> 8);
            c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, A1)));
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            if c.g[A0][0] == c.g[V1][0] {
                oc600_remove_from_group(c, frame)?;
                return Ok(());
            }
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A2][0] as i64) < 5) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = 0x1f4;
            if !more {
                c.cycles += 1;
                frame.restore(c, S0, 0x50);
                oc600_remove_epilogue(c, frame);
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_3fb0);
            }
        }
    })
}

/// The matching group's part of `CrowdMan3d_RemoveInstance`: scans its 32
/// slots for `id`, removes it, and frees the group at count zero. Always
/// ends the call (the epilogue runs on every path, as generated).
fn oc600_remove_from_group(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 7;
    c.g[S0][0] = sx(r32(c, A1).wrapping_add(r32(c, S0)));
    c.g[S2][0] = 0xffff_ffff_ffff_fffb;
    c.g[S3][0] = u64::MAX;
    c.g[V1][0] = 0;
    c.g[S1][0] = sx(r32(c, S0).wrapping_add(0xc));
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        let differs = c.g[V0][0] != c.g[S4][0];
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
        if differs {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 32) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x8));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0034_3fe8);
                }
                continue;
            }
            c.cycles += 2;
            frame.restore(c, S0, 0x50);
            oc600_remove_epilogue(c, frame);
            return Ok(());
        }

        // The slot's id matched: the block through the world table again.
        c.cycles += 10;
        call_ptr(c, 0x002d_1bd8, 0x0034_3ffc)?;
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        c.g[V0][0] = c.g[S4][0] & 0xff;
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x8)).lw(c);
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
        c.g[A0][0] = Addr(r32(c, V0)).lw(c);
        let entry_present = c.g[A0][0] != 0;
        c.g[V0][0] = sx(r32(c, S4) >> 8);
        if entry_present {
            c.cycles += 7;
            c.g[A0][0] = Addr(r32(c, A0).wrapping_add(0x1c)).lw(c);
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            c.g[V0][0] = sx(r32(c, V1) >> 8);
            let nonzero = c.g[V0][0] != 0;
            c.g[V0][0] = sx(r32(c, V0) << 2);
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

        // Clear the block's flag bits, free the slot, count the group down.
        c.cycles += 9;
        let flag_addr = r32(c, V1).wrapping_add(0x8);
        c.g[V0][0] = Addr(flag_addr).lw(c);
        c.g[V0][0] &= c.g[S2][0];
        c.g[V0][0] |= 0x2;
        let flags = c.g[V0][0];
        Addr(flag_addr).sw(c, flags);
        let slot_value = c.g[S3][0];
        Addr(r32(c, S1)).sw(c, slot_value);
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        let count = c.g[V0][0];
        Addr(r32(c, S0)).sw(c, count);
        if (count as i64) > 0 {
            c.cycles += 1;
            frame.restore(c, S0, 0x50);
            oc600_remove_epilogue(c, frame);
            return Ok(());
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        OC600_GROUP_FREE_MESHES.call(c, 0x0034_4074)?;
        c.g[A0][0] = c.g[S0][0];
        OC600_GROUP_RESET.call(c, 0x0034_407c)?;
        frame.restore(c, S0, 0x50);
        oc600_remove_epilogue(c, frame);
        return Ok(());
    }
}

/// The epilogue of `CrowdMan3d_RemoveInstance`: restores s1..s4 and ra,
/// pops the 0x60 frame and returns.
fn oc600_remove_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 7;
    frame.restore(c, S1, 0x40);
    frame.restore(c, S2, 0x30);
    frame.restore(c, S3, 0x20);
    frame.restore(c, S4, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    /// Fixture for `CrowdMan3d_RemoveInstance`: `self` at DATA+0x1000 with five
    /// groups (count 3, a key word, 32 id slots), the world at GP+0x16c8, the
    /// model table at DATA+0x2000 (entries zero on every fourth seed), the
    /// group word at DATA+0x2800 and the block chain at DATA+0x3ff8+0x80.
    /// The id sits in the matching group, so the removal path runs; the
    /// count-zero path (which calls the group's free and reset) is not
    /// reached here.
    fn oc600_remove_instance_fixture(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x0343_f38 ^ seed as u64);
        scramble(c, &mut r);
        c.pc = 0x0034_3f38;
        c.g[GP][0] = sx(GAME_GP);
        let me = DATA + 0x1000;
        let hit_group = (seed / 4) % 5;
        let hit_slot = seed % 32;
        let id = (r.next_u32() & 0xff) | ((r.next_u32() % 16) << 8);
        c.g[A0][0] = sx(me);
        c.g[A1][0] = sx(id);

        let world = DATA + 0x1100;
        let model = DATA + 0x1200;
        let table = DATA + 0x2000;
        let entry = DATA + 0x3000;
        let group_words = DATA + 0x2800;
        let chain = DATA + 0x3ff8;
        let key_ptr = DATA + 0x4100;
        c.bus.write32(GAME_GP + 0x16c8, world);
        c.bus.write32(world, model);
        c.bus.write32(model + 8, table);
        let have_entry = seed % 4 != 0;
        for idx in 0..256u32 {
            c.bus
                .write32(table + idx * 4, if have_entry { entry } else { 0 });
        }
        c.bus.write32(entry + 0x1c, group_words);
        let have_group = seed % 5 != 0;
        let group = if have_group {
            ((chain >> 2) << 8) | (r.next_u32() & 0xff)
        } else {
            r.next_u32() & 0xff
        };
        for k in 0..16u32 {
            c.bus.write32(group_words + k * 4, group);
        }
        c.bus.write32(chain + 0x80, key_ptr);
        let key = r.next_u32() & 0x00ff_ffff;
        c.bus.write32(key_ptr, (key << 8) | (r.next_u32() & 0xff));
        for i in 0..5u32 {
            let g = me + i * 0x1f4;
            c.bus.write32(g, 3);
            c.bus
                .write32(g + 8, if i == hit_group { key } else { r.next_u32() });
            for j in 0..32u32 {
                let v = if i == hit_group && j == hit_slot {
                    id
                } else {
                    r.next_u32()
                };
                c.bus.write32(g + 0xc + j * 8, v);
            }
        }
    }

    #[test]
    fn crowd_man3d_remove_instance_matches() {
        check(
            ssx3_ee::s11::f_00343f38__generated,
            crowd_man3d_remove_instance,
            200,
            &|c, seed| oc600_remove_instance_fixture(c, seed),
        );
    }
}
