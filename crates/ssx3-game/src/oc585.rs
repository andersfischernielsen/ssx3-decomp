//! Staging module oc585 (wave wave233): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc585`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0034_4348, crowd_man3d_group_free, "CrowdMan3d_GroupFree"),
    (
        0x0034_4800,
        crowd_man3d_group_free_meshes,
        "CrowdMan3d_GroupFreeMeshes",
    ),
];

/// `CrowdMan3d_GroupFreeMeshes` (0x344800), the callee of `CrowdMan3d_GroupFree`.
pub(crate) const OC585_GROUP_FREE_MESHES: Callee = Callee {
    addr: 0x0034_4800,
    f: ee::s11::f_00344800,
};

/// `EA_MemFree` (0x317e98), called with the block to free in `a0`.
pub(crate) const OC585_MEM_FREE: u32 = 0x0031_7e98;

/// `void CrowdMan3d_GroupFree(CrowdGroup* g)` (0x344348): frees the group's
/// meshes (a tail call to `CrowdMan3d_GroupFreeMeshes`, which gets `g`
/// unchanged).
pub fn crowd_man3d_group_free(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(OC585_GROUP_FREE_MESHES),
        0x0034_4358,
    );
}

/// `void CrowdMan3d_GroupFreeMeshes(CrowdGroup* g)` (0x344800): frees a
/// group's mesh records and their block.
///
/// The `+0x1ec` word points at an array of 0xd0-byte records, and the word
/// before the array is the record count. Each record's `+0x84` word points
/// at a vtable whose `+0xc` word is a function called with the record
/// address plus the halfword at vtable `+8`. Then the `+0x1e8` block is
/// freed with `EA_MemFree`; both words are cleared.
pub fn crowd_man3d_group_free_meshes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x1ec)).lw(c);
        c.g[V0][0] = 0xd0;
        if c.g[A1][0] != 0 {
            c.cycles += 5;
            c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_sub(0x10)).lw(c);
            c.mult(0, 4, 3, 2);
            c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            if c.g[A1][0] == c.g[S0][0] {
                c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x1ec)).lw(c);
            } else {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xd0));
                loop {
                    c.cycles += 9;
                    c.g[A1][0] = 0;
                    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x84)).lw(c);
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lh(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0034_4850)?;
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x1ec)).lw(c);
                    let again = c.g[V0][0] != c.g[S0][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xd0));
                    if again {
                        if event_due(c) {
                            deliver_event_here(c, 0x0034_4838);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x1ec)).lw(c);
                    break;
                }
            }
            c.cycles += 2;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
            call_ptr(c, OC585_MEM_FREE, 0x0034_4868)?;
        }
        c.cycles += 3;
        c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x1e8)).lw(c);
        if c.g[A0][0] == 0 {
            Addr((c.g[S1][0] as u32).wrapping_add(0x1e8)).sw(c, 0);
        } else {
            c.cycles += 3;
            call_ptr(c, OC585_MEM_FREE, 0x0034_487c)?;
            Addr((c.g[S1][0] as u32).wrapping_add(0x1e8)).sw(c, 0);
        }
        c.cycles += 6;
        Addr((c.g[S1][0] as u32).wrapping_add(0x1ec)).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::fakes::{fake, fall_back, hook, machine};
    use crate::difftest::{check, DATA};
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's fake `EA_MemFree` (in the machine's RAM).
    pub(crate) const OC585_CTRL: u32 = DATA + 0x3f_0000;
    pub(crate) const OC585_MAGIC: u32 = 0x0585_f00d;
    /// Vtable of the test records: `+8` halfword and `+0xc` callee.
    pub(crate) const OC585_VTABLE: u32 = DATA + 0x28_0000;
    /// The record array and the `+0x1e8` block (kept out of the log area).
    pub(crate) const OC585_ARRAY: u32 = DATA + 0x20_0000;
    pub(crate) const OC585_BLOCK: u32 = DATA + 0x30_0000;

    static OC585_PREV_FREE: AtomicUsize = AtomicUsize::new(0);

    /// `EA_MemFree`: logs the call when the gate is set, otherwise runs the
    /// body the slot held before.
    fn oc585_fake_free(c: &mut Cpu) {
        if c.bus.read32(OC585_CTRL) == OC585_MAGIC {
            let ra = c.g[RA][0] as u32;
            fake(c, OC585_MEM_FREE, ra);
            return;
        }
        fall_back(c, &OC585_PREV_FREE, ee::s10::f_00317e98__generated);
    }

    fn oc585_install_free() {
        hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc585_fake_free,
            &OC585_PREV_FREE,
        );
    }

    /// A group at `DATA` with `count` records (count 0 and no array cover the
    /// short paths), a block at `+0x1e8`, and vtables for the record calls.
    fn oc585_group_setup(c: &mut Cpu, seed: u32) {
        let _r = machine(c, seed, 0x0034_4800, 0);
        c.bus.write32(OC585_CTRL, OC585_MAGIC);
        c.g[A0][0] = sx(DATA);
        let array = if seed % 5 == 0 { 0 } else { OC585_ARRAY };
        let count = seed % 4;
        c.bus.write32(DATA + 0x1ec, array);
        c.bus
            .write32(DATA + 0x1e8, if seed % 2 == 0 { 0 } else { OC585_BLOCK });
        if array != 0 {
            c.bus.write32(array - 0x10, count);
            for k in 0..count {
                c.bus.write32(array + 0xd0 * k + 0x84, OC585_VTABLE);
            }
            c.bus.write16(OC585_VTABLE + 8, 0x10);
            c.bus
                .write32(OC585_VTABLE + 0xc, crate::difftest::fakes::FAKE);
        }
    }

    #[test]
    fn crowd_man3d_group_free_matches() {
        oc585_install_free();
        check(
            ee::s11::f_00344348__generated,
            crowd_man3d_group_free,
            200,
            &|c, seed| {
                oc585_group_setup(c, seed);
                c.pc = 0x0034_4348;
            },
        );
    }

    #[test]
    fn crowd_man3d_group_free_meshes_matches() {
        oc585_install_free();
        check(
            ee::s11::f_00344800__generated,
            crowd_man3d_group_free_meshes,
            200,
            &|c, seed| {
                oc585_group_setup(c, seed);
                c.pc = 0x0034_4800;
            },
        );
    }
}
