//! Staging module oc1131 (wave wave271): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1131`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;


/// Addresses and offsets of `SND_SifDmaUpload` (0x3c4788).
pub(crate) const SND_DMA_STATE: u32 = 0x0050_ad00;
/// Count of queued DMA transfers (a signed byte) in the DMA state block.
pub(crate) const SND_DMA_QUEUED: u32 = 0x0cc0;

/// `int SND_SifDmaUpload(int playLoc /*a0*/, int src /*a1*/, int dst /*a2*/,
/// int bytes /*a3*/, int priority /*t0*/)` (0x3c4788): upload `bytes` bytes to
/// the sound IOP in chunks of at most 0x1000. Before each chunk, kick the
/// transfer queue until at least 5 of its 28 slots are free, then queue the
/// chunk (`SND_SifDmaQueue`). Returns the last transfer id (0 if nothing was
/// sent).
pub fn snd_sif_dma_upload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = 0;
        frame.save(c, S0, 0x90);
        frame.save(c, S4, 0x50);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S5, 0x40);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, S6, 0x30);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S7, 0x20);
        c.g[S6][0] = c.g[T0][0];
        frame.save(c, S1, 0x80);
        c.g[S7][0] = c.g[A0][0];
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, FP, 0x10);
        let done = (c.g[S0][0] as i64) <= 0;
        frame.save_ra(c, 0);
        if !done {
            c.cycles += 2;
            c.g[FP][0] = 0x1c;
            'chunks: loop {
                c.cycles += 12;
                c.g[V1][0] = 0x0051_0000;
                c.g[A0][0] = 0xfff;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5300));
                c.g[S1][0] = 0x1000;
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(SND_DMA_QUEUED));
                c.g[V1][0] = ((c.g[A0][0] as i64) < (c.g[S0][0] as i64)) as u64;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[V0][0] = ((c.g[V0][0] as i64) < 5) as u64;
                let enough_room = c.g[V0][0] == 0;
                if c.g[V1][0] == 0 {
                    c.g[S1][0] = c.g[S0][0];
                }
                if !enough_room {
                    c.cycles += 4;
                    c.g[V0][0] = 0x0051_0000;
                    c.g[S2][0] = 0x1c;
                    c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5300));
                    loop {
                        c.cycles += 9;
                        c.g[A0][0] = 0;
                        Callee { addr: 0x003c_45e8, f: ee::s13::f_003c45e8 }.call(c, 0x003c_4820)?;
                        c.g[V0][0] = c.lbu((c.g[S3][0] as u32).wrapping_add(SND_DMA_QUEUED));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                        c.g[V0][0] = ((c.g[V0][0] as i64) < 5) as u64;
                        if c.g[V0][0] == 0 {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003c_4818);
                        }
                    }
                }
                c.cycles += 10;
                c.g[A1][0] = c.g[S5][0];
                c.g[A2][0] = c.g[S4][0];
                c.g[A0][0] = c.g[S7][0];
                c.g[A3][0] = c.g[S1][0];
                c.g[T0][0] = c.g[S6][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                Callee { addr: 0x003c_4668, f: ee::s13::f_003c4668 }.call(c, 0x003c_4858)?;
                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                let more = (c.g[S0][0] as i64) > 0;
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S1][0] as u32));
                if !more {
                    break 'chunks;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_47d8);
                }
            }
        }
        c.cycles += 12;
        for (reg, off) in [(S0, 0x90), (S1, 0x80), (S2, 0x70), (S3, 0x60), (S4, 0x50), (S5, 0x40), (S6, 0x30), (S7, 0x20), (FP, 0x10)] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// Size of one `FEAsyncManager` file record (`self + 0x100 + idx * 0x11c`).
pub(crate) const FEASYNC_FILE_STRIDE: u32 = 0x11c;
/// Size of the buffer `FEAsyncManager_LoadSlot` allocates for a slot.
pub(crate) const FEASYNC_SLOT_BYTES: u32 = 0x109a0;

/// `void cFEAsyncManager::LoadSlot(this /*a0*/, int idx /*a1*/)` (0x1a3cd8):
/// set the slot's size to 0x109a0, allocate its buffer (`operator new`,
/// string at 0x461430, flags 0x25000000) into the record's first word, mark the
/// slot state 2, then either start an async load (record `+0x108` set: state
/// 2 again, handle at `+0x104` from `AsyncFile_LoadInto`) or load it at once
/// (`EAFile_LoadFileInto`) and mark the state 3.
pub fn feasync_manager_load_slot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = FEASYNC_FILE_STRIDE as u64;
        frame.save(c, S4, 0x10);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x40);
        c.g[S4][0] = c.g[A1][0];
        c.mult(0, S1, S4, V0);
        frame.save(c, S0, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S3, 0x20);
        c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10c));
        c.g[V0][0] = 0x10000;
        frame.save(c, S2, 0x30);
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x100));
        frame.save_ra(c, 0);
        // S3 -> the record's size field, S2 -> its buffer pointer.
        c.g[S3][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.g[V0][0] |= 0x9a0;
        c.g[A1][0] = 0x0046_0000;
        c.g[A0][0] = 0x10000;
        c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1430));
        c.g[A0][0] |= 0x9a0;
        c.g[A2][0] = 0x2500_0000;
        let size_at = c.g[S3][0] as u32;
        let size = c.g[V0][0];
        c.sw(size_at, size);
        call_ptr(c, 0x0031_7e30, 0x001a_3d3c)?;
        let buffer_at = c.g[S2][0] as u32;
        let buffer = c.g[V0][0];
        c.sw(buffer_at, buffer);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S4][0];
        c.g[A2][0] = 2;
        Callee { addr: 0x001a_39d8, f: ee::s03::f_001a39d8 }.call(c, 0x001a_3d50)?;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x108));
        let blocking = c.g[V1][0] == 0;
        c.g[A1][0] = c.g[S4][0];
        if !blocking {
            c.cycles += 10;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = 2;
            Callee { addr: 0x001a_39d8, f: ee::s03::f_001a39d8 }.call(c, 0x001a_3d6c)?;
            c.g[A1][0] = c.lw(c.g[S2][0] as u32);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S1][0] as u32));
            c.g[A2][0] = c.lw(c.g[S3][0] as u32);
            call_ptr(c, 0x003d_f748, 0x001a_3d7c)?;
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S1][0] as u32));
            let handle_at = (c.g[V1][0] as u32).wrapping_add(0x104);
            let handle = c.g[V0][0];
            c.sw(handle_at, handle);
        } else {
            c.cycles += 8;
            c.g[A1][0] = c.lw(c.g[S2][0] as u32);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S1][0] as u32));
            c.g[A2][0] = c.lw(c.g[S3][0] as u32);
            call_ptr(c, 0x003e_1bb8, 0x001a_3d98)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S4][0];
            c.g[A2][0] = 3;
            Callee { addr: 0x001a_39d8, f: ee::s03::f_001a39d8 }.call(c, 0x001a_3da8)?;
        }
        c.cycles += 8;
        for (reg, off) in [(S0, 0x50), (S1, 0x40), (S2, 0x30), (S3, 0x20), (S4, 0x10)] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// The "FX bank loaded" flag of `SND_EeFxFree`.
pub(crate) const SND_EEFX_LOADED: u32 = 0x0044_e898;
/// `SND_EeFxFree` state words in the sound block (`0x510000 + off`).
pub(crate) const SND_EEFX_BASE_OFF: u32 = 0x5d40;
pub(crate) const SND_EEFX_ENTRIES_OFF: u32 = 0x5d44;
pub(crate) const SND_EEFX_TABLE6_OFF: u32 = 0x5d48;
pub(crate) const SND_EEFX_INDEX_OFF: u32 = 0x5d54;

/// `void SND_EeFxFree(void)` (0x3cbec8): if the FX bank is loaded, read the
/// entry count (big endian) from `base + index*4 + 4`, call each entry's
/// release callback (`+4`) and free the entry, free the entry array, clear the
/// six-word table and free it, zero the bank's counters, clear the loaded flag
/// and `memset(0x515d80, 0, 0x800)`.
pub fn snd_ee_fx_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[V1][0] = 0x0045_0000;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x1768));
        for (reg, off) in [(S0, 0x90), (S1, 0x80), (S2, 0x70), (S3, 0x60), (S4, 0x50), (S5, 0x40), (S6, 0x30), (S7, 0x20), (FP, 0x10)] {
            frame.save(c, reg, off);
        }
        let loaded = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if loaded {
            c.cycles += 17;
            c.g[V0][0] = 0x0051_0000;
            c.g[V1][0] = 0x0051_0000;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(SND_EEFX_INDEX_OFF));
            c.g[A1][0] = 4;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(SND_EEFX_BASE_OFF));
            c.g[S3][0] = 0x0051_0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[S4][0] = 0x0051_0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
            c.g[FP][0] = 0x0051_0000;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[S7][0] = 0x0051_0000;
            call_ptr(c, 0x003c_72f8, 0x003c_bf34)?;
            c.g[S6][0] = 0x0051_0000;
            c.g[S2][0] = c.g[V0][0];
            let none = c.g[S2][0] == 0;
            c.g[S5][0] = 0x0051_0000;
            if !none {
                c.cycles += 2;
                let empty = (c.g[S2][0] as i64) <= 0;
                c.g[S0][0] = 0;
                if !empty {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(SND_EEFX_ENTRIES_OFF));
                    loop {
                        c.cycles += 6;
                        c.g[S1][0] = sx((c.g[S0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(4));
                        if c.g[V0][0] == 0 {
                            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(SND_EEFX_ENTRIES_OFF));
                        } else {
                            c.cycles += 7;
                            let callback = c.g[V0][0] as u32;
                            call_ptr(c, callback, 0x003c_bf70)?;
                            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(SND_EEFX_ENTRIES_OFF));
                            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                            c.sw((c.g[V1][0] as u32).wrapping_add(4), 0);
                            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(SND_EEFX_ENTRIES_OFF));
                        }
                        c.cycles += 4;
                        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                        let entry_empty = c.g[A0][0] == 0;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                        if !entry_empty {
                            c.cycles += 5;
                            call_ptr(c, 0x003b_5e98, 0x003c_bf9c)?;
                            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(SND_EEFX_ENTRIES_OFF));
                            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.sw(c.g[V0][0] as u32, 0);
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(SND_EEFX_ENTRIES_OFF));
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003c_bf50);
                        }
                    }
                }
            }
            // Free the entry array.
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(SND_EEFX_ENTRIES_OFF));
            let no_array = c.g[A0][0] == 0;
            c.g[S0][0] = 0;
            if !no_array {
                c.cycles += 3;
                call_ptr(c, 0x003b_5e98, 0x003c_bfc8)?;
                c.sw((c.g[S3][0] as u32).wrapping_add(SND_EEFX_ENTRIES_OFF), 0);
            }
            // Clear the six-word table.
            c.cycles += 3;
            c.g[A1][0] = 0x0051_0000;
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(SND_EEFX_TABLE6_OFF));
            loop {
                c.cycles += 5;
                c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                if c.g[A0][0] != 0 {
                    c.sw(c.g[V1][0] as u32, 0);
                }
                c.cycles += 4;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < 6) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(SND_EEFX_TABLE6_OFF));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_bfd8);
                }
            }
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(SND_EEFX_TABLE6_OFF));
            let no_table = c.g[A0][0] == 0;
            c.g[V0][0] = 0x0051_0000;
            if !no_table {
                c.cycles += 4;
                call_ptr(c, 0x003b_5e98, 0x003c_c010)?;
                c.sw((c.g[S4][0] as u32).wrapping_add(SND_EEFX_TABLE6_OFF), 0);
                c.g[V0][0] = 0x0051_0000;
            }
            c.cycles += 10;
            c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x5d80));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x75f8), 0);
            c.g[A1][0] = 0;
            c.g[V0][0] = 0x0045_0000;
            c.sw((c.g[FP][0] as u32).wrapping_add(0x75fc), 0);
            c.sw((c.g[S7][0] as u32).wrapping_add(0x5d50), 0);
            c.g[A2][0] = 0x800;
            c.sw((c.g[S6][0] as u32).wrapping_add(0x5d4c), 0);
            c.sw((c.g[V0][0] as u32).wrapping_sub(0x1768), 0);
            call_ptr(c, 0x0041_6210, 0x003c_c040)?;
        }
        c.cycles += 12;
        for (reg, off) in [(S0, 0x90), (S1, 0x80), (S2, 0x70), (S3, 0x60), (S4, 0x50), (S5, 0x40), (S6, 0x30), (S7, 0x20), (FP, 0x10)] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003c_4788, snd_sif_dma_upload, "SND_SifDmaUpload"),
    (0x001a_3cd8, feasync_manager_load_slot, "FEAsyncManager_LoadSlot"),
    (0x003c_bec8, snd_ee_fx_free, "SND_EeFxFree"),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{Rng, DATA};
    use std::ptr::addr_of_mut;

    /// Gate of this module's callee fakes: they answer only while this word
    /// holds `OC1131_MAGIC` (otherwise the bodies they replaced run).
    const OC1131_CTRL: u32 = DATA + 0x0079_0000;
    const OC1131_MAGIC: u32 = 0x1131_c0de;
    /// The entry count `SND_ReadBigEndian`'s fake returns.
    const OC1131_COUNT: u32 = OC1131_CTRL + 8;

    fn oc1131_gated(c: &mut Cpu) -> bool {
        c.bus.read32(OC1131_CTRL) == OC1131_MAGIC
    }

    /// A callee fake that logs the call and returns (gate on), or runs the
    /// body it replaced (gate off).
    macro_rules! oc1131_fake {
        ($name:ident, $target:expr, $generated:path) => {
            fn $name(c: &mut Cpu) {
                if !oc1131_gated(c) {
                    return fakes::fall_back(c, fakes::prev_cell($name), $generated);
                }
                fakes::fake_call(c, $target);
            }
        };
    }

    oc1131_fake!(oc1131_fake_new, 0x0031_7e30, ee::s10::f_00317e30__generated);
    oc1131_fake!(oc1131_fake_set_state, 0x001a_39d8, ee::s03::f_001a39d8__generated);
    oc1131_fake!(oc1131_fake_async_load, 0x003d_f748, ee::s14::f_003df748__generated);
    oc1131_fake!(oc1131_fake_file_load, 0x003e_1bb8, ee::s14::f_003e1bb8__generated);
    oc1131_fake!(oc1131_fake_mem_free, 0x003b_5e98, ee::s13::f_003b5e98__generated);
    oc1131_fake!(oc1131_fake_memset, 0x0041_6210, ee::s15::f_00416210__generated);

    /// SND_SifDmaKick: a transfer leaves the queue (count -= 2).
    fn oc1131_fake_kick(c: &mut Cpu) {
        if !oc1131_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1131_fake_kick),
                ee::s13::f_003c45e8__generated,
            );
        }
        fakes::fake_call(c, 0x003c_45e8);
        let n = c.bus.read8(SND_DMA_STATE + SND_DMA_QUEUED);
        c.bus.write8(SND_DMA_STATE + SND_DMA_QUEUED, n.wrapping_sub(2));
    }

    /// SND_SifDmaQueue: the chunk joins the queue (count += 1).
    fn oc1131_fake_queue(c: &mut Cpu) {
        if !oc1131_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1131_fake_queue),
                ee::s13::f_003c4668__generated,
            );
        }
        fakes::fake_call(c, 0x003c_4668);
        let n = c.bus.read8(SND_DMA_STATE + SND_DMA_QUEUED);
        c.bus.write8(SND_DMA_STATE + SND_DMA_QUEUED, n.wrapping_add(1));
    }

    /// SND_ReadBigEndian: returns the count the test put in `OC1131_COUNT`.
    fn oc1131_fake_read_be(c: &mut Cpu) {
        if !oc1131_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1131_fake_read_be),
                ee::s13::f_003c72f8__generated,
            );
        }
        let count = c.bus.read32(OC1131_COUNT);
        fakes::fake_call(c, 0x003c_72f8);
        c.g[V0][0] = sx(count);
    }

    fn oc1131_install_fakes() {
        fakes::hook(addr_of_mut!(ee::s10::OVERRIDE_00317E30), oc1131_fake_new, fakes::prev_cell(oc1131_fake_new));
        fakes::hook(addr_of_mut!(ee::s03::OVERRIDE_001A39D8), oc1131_fake_set_state, fakes::prev_cell(oc1131_fake_set_state));
        fakes::hook(addr_of_mut!(ee::s14::OVERRIDE_003DF748), oc1131_fake_async_load, fakes::prev_cell(oc1131_fake_async_load));
        fakes::hook(addr_of_mut!(ee::s14::OVERRIDE_003E1BB8), oc1131_fake_file_load, fakes::prev_cell(oc1131_fake_file_load));
        fakes::hook(addr_of_mut!(ee::s13::OVERRIDE_003B5E98), oc1131_fake_mem_free, fakes::prev_cell(oc1131_fake_mem_free));
        fakes::hook(addr_of_mut!(ee::s15::OVERRIDE_00416210), oc1131_fake_memset, fakes::prev_cell(oc1131_fake_memset));
        fakes::hook(addr_of_mut!(ee::s13::OVERRIDE_003C45E8), oc1131_fake_kick, fakes::prev_cell(oc1131_fake_kick));
        fakes::hook(addr_of_mut!(ee::s13::OVERRIDE_003C4668), oc1131_fake_queue, fakes::prev_cell(oc1131_fake_queue));
        fakes::hook(addr_of_mut!(ee::s13::OVERRIDE_003C72F8), oc1131_fake_read_be, fakes::prev_cell(oc1131_fake_read_be));
    }

    /// Uploads of 0, negative, one-chunk and several-chunk sizes, with the DMA
    /// queue empty, half full and full (so the kick loop runs 0-3 times).
    #[test]
    fn oc1131_snd_sif_dma_upload_matches() {
        oc1131_install_fakes();
        fakes::covered(
            ee::s13::f_003c4788__generated,
            snd_sif_dma_upload,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1131_0001, 6);
                c.bus.write32(OC1131_CTRL, OC1131_MAGIC);
                let bytes: u32 = match seed % 7 {
                    0 => 0,
                    1 => (r.next_u32() | 0x8000_0000) & 0xffff_fff0,
                    2 => 0x10 + (r.next_u32() & 0xff),
                    3 => 0x1000,
                    4 => 0xfff,
                    5 => 0x2fff,
                    _ => 0x3500,
                };
                let queued: u8 = match seed % 5 {
                    0 => 0,
                    1 => 0x1c,
                    2 => 0x18,
                    3 => 0x17,
                    _ => 0xf0,
                };
                c.bus.write8(SND_DMA_STATE + SND_DMA_QUEUED, queued);
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = sx(0x0010_0000 + (r.next_u32() & 0xf_fff0));
                c.g[A2][0] = sx(r.next_u32() & 0x1f_fff0);
                c.g[A3][0] = sx(bytes);
                c.g[T0][0] = r.next_u32() as u64;
            },
        );
    }

    /// Loads slots 0-2 of a manager, async (flag at `+0x108` set) and
    /// blocking.
    #[test]
    fn oc1131_feasync_manager_load_slot_matches() {
        oc1131_install_fakes();
        fakes::covered(
            ee::s03::f_001a3cd8__generated,
            feasync_manager_load_slot,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1131_0002, 4);
                c.bus.write32(OC1131_CTRL, OC1131_MAGIC);
                let this = DATA + 0x0079_1000;
                let idx = seed % 3;
                for k in 0..0x40 {
                    c.bus.write32(this + 4 * k, r.next_u32());
                }
                let record = this + 0x100 + idx * FEASYNC_FILE_STRIDE;
                c.bus.write32(record + 0x108, if seed % 2 == 0 { 0 } else { r.next_u32() | 1 });
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(idx);
            },
        );
    }

    const OC1131_ENTRIES: u32 = DATA + 0x0079_2000;
    const OC1131_OBJECTS: u32 = DATA + 0x0079_3000;
    const OC1131_TABLE6: u32 = DATA + 0x0079_4000;
    const OC1131_BASE: u32 = DATA + 0x0079_5000;

    /// Frees an FX bank: not loaded, no entries, entries with and without
    /// release callbacks, and with and without the entry array / table.
    #[test]
    fn oc1131_snd_ee_fx_free_matches() {
        oc1131_install_fakes();
        fakes::covered(
            ee::s14::f_003cbec8__generated,
            snd_ee_fx_free,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1131_0003, 10);
                c.bus.write32(OC1131_CTRL, OC1131_MAGIC);
                let count: u32 = match seed % 6 {
                    0 => 0,
                    1 => 0xffff_ffff,
                    2 => 1,
                    3 => 2,
                    4 => 3,
                    _ => 4,
                };
                c.bus.write32(OC1131_COUNT, count);
                c.bus.write32(SND_EEFX_LOADED, if seed % 9 == 0 { 0 } else { 1 });
                c.bus.write32(0x0051_5d54, r.next_u32() & 3);
                c.bus.write32(0x0051_5d40, OC1131_BASE);
                let entries = if seed % 11 == 5 { 0 } else { OC1131_ENTRIES };
                c.bus.write32(0x0051_5d44, entries);
                let table = if seed % 7 == 3 { 0 } else { OC1131_TABLE6 };
                c.bus.write32(0x0051_5d48, table);
                for i in 0..4u32 {
                    let obj = OC1131_OBJECTS + 0x40 * i;
                    let callback = if (seed >> i) & 1 == 0 { 0 } else { 0x4000 + 0x10 * i };
                    c.bus.write32(obj + 4, callback);
                    let present = (seed >> (i + 2)) % 5 != 0;
                    c.bus.write32(OC1131_ENTRIES + 4 * i, if present { obj } else { 0 });
                }
                for i in 0..6u32 {
                    let word = if r.next_u32() & 1 == 0 { 0 } else { r.next_u32() | 1 };
                    c.bus.write32(OC1131_TABLE6 + 4 * i, word);
                }
                for (off, v) in [(0x75f8, 7), (0x75fc, 8)] {
                    c.bus.write32(0x0051_0000 + off, v);
                }
                c.bus.write32(0x0051_5d4c, 9);
                c.bus.write32(0x0051_5d50, 10);
            },
        );
    }
}
