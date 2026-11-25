//! Staging module oc656 (wave g36.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc656`.

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
    (0x0031_1250, adf_data_source_ctor, "AdfDataSource_ctor"),
    (0x0032_8170, sphere_tree_walk_levels, "SphereTree_WalkLevels"),
];

/// `AdfDataSource_ctor(self, src)` (0x311250): loads the ADF pack `src`
/// through `FILE_loadpack` (`EAFile_LoadDecompressedThunk`, 0x3e22f0), stores
/// the pack pointer at `*self`, and fills the data source from it with
/// `AdfDataSource_SetData` (0x311290). Returns `self`.
pub fn adf_data_source_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x003e_22f0, 0x0031_126c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        let pack = c.g[V0][0];
        Addr(c.g[S0][0] as u32).sw(c, pack);
        Callee {
            addr: 0x0031_1290,
            f: ee::s10::f_00311290,
        }
        .call(c, 0x0031_127c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `SphereTree_WalkLevels(octree)` (0x328170; the reference calls it
/// `bxSphereTree_Draw`): for a compressed sphere tree, allocates the bit-tree
/// buffer (`SphereTreeCache_GetMasks`, 0x32df28) into `treeInfo->bitTree`
/// (+0x28), then walks the levels from the tree's depth (+0xc) with
/// `SphereTree_WalkLevelsRecursive` (0x3281d0). `treeInfo` is at +0x98.
pub fn sphere_tree_walk_levels(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x98).lw(c);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x8).lw(c);
        if c.g[V0][0] == 0 {
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x98).lw(c);
        } else {
            c.cycles += 5;
            call_ptr(c, 0x0032_df28, 0x0032_8198)?;
            c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x98).lw(c);
            let masks = c.g[V0][0];
            Addr(c.g[V1][0] as u32).off(0x28).sw(c, masks);
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x98).lw(c);
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x80));
        c.g[A3][0] = 0u64;
        c.g[A1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
        c.g[T0][0] = 0u64;
        Callee {
            addr: 0x0032_81d0,
            f: ee::s10::f_003281d0,
        }
        .call(c, 0x0032_81bc)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of the ADF fake: `FILE_loadpack` returns `ADF_PACK` only while
    /// this word holds `ADF_MAGIC` (the real loader would read a disc).
    const ADF_CTRL: u32 = DATA + 0xe_0000;
    const ADF_MAGIC: u32 = 0x4144_4631;
    const ADF_PACK: u32 = DATA + 0xe_1000;
    const ADF_SELF: u32 = DATA + 0x3000;
    const ADF_SRC: u32 = DATA + 0x3800;
    static ADF_PREV_LOAD: AtomicUsize = AtomicUsize::new(0);

    fn adf_fake_load(c: &mut Cpu) {
        if c.bus.read32(ADF_CTRL) != ADF_MAGIC {
            return fakes::fall_back(c, &ADF_PREV_LOAD, ee::s14::f_003e22f0__generated);
        }
        c.g[V0][0] = sx(ADF_PACK);
        c.pc = c.g[RA][0] as u32;
    }

    fn adf_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E22F0),
            adf_fake_load,
            &ADF_PREV_LOAD,
        );
    }

    #[test]
    fn adf_data_source_ctor_matches() {
        adf_install_fakes();
        check(ee::s10::f_00311250__generated, adf_data_source_ctor, 200, &|c, seed| {
            let mut r = Rng(0x0adf_0001 ^ seed as u64);
            scramble(c, &mut r);
            c.pc = 0x0031_1250;
            c.g[A0][0] = sx(ADF_SELF);
            c.g[A1][0] = sx(ADF_SRC);
            c.bus.write32(ADF_CTRL, ADF_MAGIC);
            // SetData reads the pack's word at +4 and adds it to the pack.
            c.bus.write32(ADF_PACK + 4, r.next_u32() & 0xffff);
        });
    }

    /// Gate of the bit-tree fake: `SphereTreeCache_GetMasks` returns the word
    /// at `SPH_CTRL + 4` only while `SPH_CTRL` holds `SPH_MAGIC`.
    const SPH_OCTREE: u32 = DATA + 0x3000;
    const SPH_INFO: u32 = DATA + 0x3100;
    const SPH_CTRL: u32 = DATA + 0xe_2000;
    const SPH_MAGIC: u32 = 0x5350_4831;
    static SPH_PREV_MASKS: AtomicUsize = AtomicUsize::new(0);

    fn sph_fake_masks(c: &mut Cpu) {
        if c.bus.read32(SPH_CTRL) != SPH_MAGIC {
            return fakes::fall_back(c, &SPH_PREV_MASKS, ee::s11::f_0032df28__generated);
        }
        c.g[V0][0] = sx(c.bus.read32(SPH_CTRL + 4));
        c.pc = c.g[RA][0] as u32;
    }

    fn sph_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_0032DF28),
            sph_fake_masks,
            &SPH_PREV_MASKS,
        );
    }

    #[test]
    fn sphere_tree_walk_levels_matches() {
        sph_install_fakes();
        check(ee::s10::f_00328170__generated, sphere_tree_walk_levels, 300, &|c, seed| {
            let mut r = Rng(0x5350_4800 ^ seed as u64);
            scramble(c, &mut r);
            c.pc = 0x0032_8170;
            c.g[A0][0] = sx(SPH_OCTREE);
            c.bus.write32(SPH_OCTREE + 0x98, SPH_INFO);
            // isCompressed: 0 skips the buffer, anything else allocates it.
            let compressed = if seed % 2 == 0 { 0 } else { r.next_u32() | 1 };
            c.bus.write32(SPH_INFO + 0x8, compressed);
            // Depth <= 0: the recursive walk returns at once.
            let depth = [0u32, 0xffff_ffff, 0xffff_fffb][(seed / 2 % 3) as usize];
            c.bus.write32(SPH_INFO + 0xc, depth);
            c.bus.write32(SPH_INFO + 0x28, r.next_u32());
            c.bus.write32(SPH_CTRL, SPH_MAGIC);
            c.bus.write32(SPH_CTRL + 4, r.next_u32());
        });
    }
}
