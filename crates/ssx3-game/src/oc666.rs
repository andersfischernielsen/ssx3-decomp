//! Staging module oc666 (wave g46.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc666`.

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
    0x0036_6e30,
    vram_heap_free_texture_blocks_port,
    "VramHeap_FreeTextureBlocks",
)];

/// `void VramHeap_FreeTextureBlocks(VramHeap* heap, int index)` (0x366e30):
/// the texture block in slot `index` of the heap's table (the table pointer
/// at `heap+0x23e8`, 4 bytes per slot) gives up its two sub-blocks. Each id
/// stored at `+0x30` and `+0x34` that is non-negative is freed with
/// `VramHeap_FreeBlock(heap, id)`; a negative id is an empty sub-block.
pub fn vram_heap_free_texture_blocks_port(c: &mut Cpu) {
    const FREE_BLOCK: Callee = Callee {
        addr: 0x0036_63d8,
        f: ee::s12::f_003663d8,
    };
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let heap = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(heap.wrapping_add(0x23e8)).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[S1][0] = Addr(c.g[A1][0] as u32).lw(c);
        let blk = c.g[S1][0] as u32;
        c.g[A1][0] = Addr(blk.wrapping_add(0x30)).lw(c);
        if (c.g[A1][0] as i64) >= 0 {
            c.cycles += 3;
            FREE_BLOCK.call(c, 0x0036_6e68)?;
        }
        let blk = c.g[S1][0] as u32;
        c.g[A1][0] = Addr(blk.wrapping_add(0x34)).lw(c);
        c.cycles += 2;
        if (c.g[A1][0] as i64) >= 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            FREE_BLOCK.call(c, 0x0036_6e7c)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{fakes, Rng, DATA};
    use std::ptr::addr_of_mut;

    /// Fixture layout in the test data area: the slot table, the blocks, and
    /// the gate word that switches the `VramHeap_FreeBlock` fake on.
    const OC666_TABLE: u32 = DATA + 0x2_0000;
    const OC666_BLOCKS: u32 = DATA + 0x3_0000;
    const OC666_CTRL: u32 = DATA + 0x20_1000;
    const OC666_MAGIC: u32 = 0x0666_0666;

    /// `VramHeap_FreeBlock` fake, gated on [`OC666_CTRL`]: logs the call and
    /// returns to `ra`; with the gate off it runs the body the slot held.
    fn oc666_fake_free_block(c: &mut Cpu) {
        if c.bus.read32(OC666_CTRL) != OC666_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc666_fake_free_block),
                ee::s12::f_003663d8__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0036_63d8, ra);
    }

    fn oc666_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_003663D8),
            oc666_fake_free_block,
            fakes::prev_cell(oc666_fake_free_block),
        );
    }

    /// A sub-block id, negative (an empty sub-block) when `negative`.
    fn oc666_block_id(r: &mut Rng, negative: bool) -> u32 {
        let v = r.next_u32() & 0x7fff_ffff;
        if negative {
            v | 0x8000_0000
        } else {
            v
        }
    }

    /// `VramHeap_FreeTextureBlocks` against the generated body: a random
    /// slot, both sub-block ids negative or not (all four combinations),
    /// `VramHeap_FreeBlock` as the gated fake.
    #[test]
    fn vram_heap_free_texture_blocks_port_matches() {
        oc666_install_fakes();
        fakes::covered(
            ee::s12::f_00366e30__generated,
            vram_heap_free_texture_blocks_port,
            60,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0036_6e30, 2);
                c.bus.write32(OC666_CTRL, OC666_MAGIC);
                let heap = DATA;
                let idx = r.next_u32() % 8;
                let blk = OC666_BLOCKS + 0x100 * idx;
                c.bus.write32(heap + 0x23e8, OC666_TABLE);
                c.bus.write32(OC666_TABLE + 4 * idx, blk);
                let first = oc666_block_id(&mut r, (seed & 1) != 0);
                let second = oc666_block_id(&mut r, (seed & 2) != 0);
                c.bus.write32(blk + 0x30, first);
                c.bus.write32(blk + 0x34, second);
                c.g[A0][0] = sx(heap);
                c.g[A1][0] = sx(idx);
            },
        );
    }
}
