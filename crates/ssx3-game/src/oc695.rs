//! Staging module oc695 (wave wave237): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc695`.
//!
//! Not ported here: `SND_SpuMemAlloc` (0x3c4b70) is a resumable dispatcher
//! (`match pc` with loop-head re-entries); the orchestrator converts it with
//! `tools/port_dispatcher.py`.

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
    (0x003c_4898, snd_sif_dma_poll, "SND_SifDmaPoll"),
    (0x003c_4918, snd_sif_dma_complete, "SND_SifDmaComplete"),
    (0x003c_4d78, snd_spu_mem_free, "SND_SpuMemFree"),
    (
        0x003c_6018,
        snd_add_service_callback,
        "SND_AddServiceCallback",
    ),
    (0x003c_6500, snd_bank_find_free_slot, "SND_BankFindFreeSlot"),
    (0x003c_7480, snd_iop_init_defaults, "SND_IopInitDefaults"),
    (0x003c_7d00, snd_ee_mixer_init, "SND_EeMixerInit"),
    (
        0x003c_8128,
        snd_ee_mixer_init_mix_func,
        "SND_EeMixerInitMixFunc",
    ),
    (0x003c_8148, snd_ee_mixer_set_mode, "SND_EeMixerSetMode"),
    (
        0x003c_9268,
        snd_register_pcm16_decoders,
        "SND_RegisterPcm16Decoders",
    ),
    (
        0x003c_92b0,
        snd_register_eaxa_decoders,
        "SND_RegisterEaxaDecoders",
    ),
];

// ---------------------------------------------------------------------------
// SIF DMA queue.

/// The SND state block (`0x510000 - 0x5300`) that holds the SIF DMA queue.
pub(crate) const SIFDMA_STATE: u32 = 0x0050_ad00;
/// `u8`: number of queued transfers (signed; <= 0 means none).
pub(crate) const SIFDMA_QUEUED: u32 = 0xcc0;
/// `u8`: set while a transfer is in flight.
pub(crate) const SIFDMA_BUSY: u32 = 0xcc2;
/// `u32`: audio tick at which the current transfer started.
pub(crate) const SIFDMA_STARTED: u32 = 0xcc4;
/// 28 queue entries of 0x10 bytes, each starting with its key.
pub(crate) const SIFDMA_ENTRIES: u32 = 0xcc8;
/// Number of queue entries.
pub(crate) const SIFDMA_ENTRY_COUNT: u32 = 28;
/// The audio-system block (`0x510000 - 0x5718`); the tick is a word at +0x180.
pub(crate) const SND_AUDIO_STATE: u32 = 0x0050_a8e8;
/// `u32` audio tick in the audio-system block.
pub(crate) const SND_AUDIO_TICK: u32 = 0x180;

/// `SND_SifDmaStartNext(int key)` (0x3c4450) and `SND_SifDmaKick(int)`.
pub(crate) const SIFDMA_START_NEXT: Callee = Callee {
    addr: 0x003c_4450,
    f: ee::s13::f_003c4450,
};
pub(crate) const SIFDMA_KICK: Callee = Callee {
    addr: 0x003c_45e8,
    f: ee::s13::f_003c45e8,
};

/// `int SND_SifDmaPoll(int key)` (0x3c4898): 1 if no transfer is queued or
/// `key` is not in the queue; 0 if it is (still pending). A transfer pending
/// for more than 500 audio ticks restarts the queue (`SND_SifDmaStartNext(0)`).
pub fn snd_sif_dma_poll(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 0x51_0000;
        frame.save_ra(c, 0);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x5300));
        c.g[V0][0] = c.lbu(r32(c, V1).wrapping_add(SIFDMA_QUEUED));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[V0][0] = 0x51_0000;
        'done: {
            'not_found: {
                if empty {
                    break 'not_found;
                }
                c.cycles += 4;
                c.g[A2][0] = 0;
                c.g[A3][0] = sx(r32(c, V0).wrapping_sub(0x5718));
                c.g[A1][0] = sx(r32(c, V1).wrapping_add(SIFDMA_ENTRIES));
                loop {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(r32(c, A1));
                    if c.g[V0][0] != c.g[A0][0] {
                        c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                    } else {
                        c.cycles += 6;
                        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(SIFDMA_STARTED));
                        c.g[V1][0] = c.lw(r32(c, A3).wrapping_add(SND_AUDIO_TICK));
                        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x1f4));
                        c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                        let fresh = c.g[V0][0] == 0;
                        c.g[V0][0] = 0;
                        if fresh {
                            break 'done;
                        }
                        c.cycles += 4;
                        c.g[A0][0] = 0;
                        SIFDMA_START_NEXT.call(c, 0x003c_48f4)?;
                        c.g[V0][0] = 0;
                        break 'done;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A2][0] as i64) < SIFDMA_ENTRY_COUNT as i64) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x10));
                    if !more {
                        break 'not_found;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_48c8);
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SND_SifDmaComplete(int key)` (0x3c4918): a transfer finished. Finds
/// the queue entry with `key`, clears the busy flag, drops the queued count,
/// frees the entry and kicks the next transfer (`SND_SifDmaKick(1)`).
pub fn snd_sif_dma_complete(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x51_0000;
        c.g[A2][0] = sx(r32(c, V0).wrapping_sub(0x5300));
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        c.g[V1][0] = sx(r32(c, A2).wrapping_add(SIFDMA_ENTRIES));
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(r32(c, V1));
            let differs = c.g[V0][0] != c.g[A0][0];
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
            if !differs {
                c.cycles += 9;
                c.sb(r32(c, A2).wrapping_add(SIFDMA_BUSY), 0);
                c.g[A0][0] = 1;
                c.g[V0][0] = c.lbu(r32(c, A2).wrapping_add(SIFDMA_QUEUED));
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                c.sb(r32(c, A2).wrapping_add(SIFDMA_QUEUED), c.g[V0][0]);
                c.sw(r32(c, V1), 0);
                SIFDMA_KICK.call(c, 0x003c_4958)?;
                frame.restore_ra(c, 0);
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < SIFDMA_ENTRY_COUNT as i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x10));
            if !more {
                c.cycles += 1;
                frame.restore_ra(c, 0);
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_4930);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SPU memory.

/// SND state block field: `u16` number of allocated SPU-memory segments.
pub(crate) const SPUMEM_COUNT: u32 = 0x1a98;
/// SND state block field: pointer to the segment list (`{ u16 key; u16 size }`).
pub(crate) const SPUMEM_LIST: u32 = 0x1aa0;
/// `sceSifFreeSysMemory(void*)`: frees IOP memory (the `0x100` play location).
pub(crate) const SPUMEM_FREE_IOP: u32 = 0x0042_ad08;

/// `int SND_SpuMemFree(int playLoc, uint address)` (0x3c4d78): playLoc 0x100
/// is IOP memory (`sceSifFreeSysMemory(address)`); otherwise the segment
/// whose key is `address >> 6` is removed from the segment list (the later
/// entries shift down). 0 on success, -8 when no segment has that key.
pub fn snd_spu_mem_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x100;
        let not_iop = c.g[A0][0] != c.g[V0][0];
        frame.save_ra(c, 0);
        'end: {
            if !not_iop {
                c.cycles += 4;
                c.g[A0][0] = c.g[A1][0];
                call_ptr(c, SPUMEM_FREE_IOP, 0x003c_4d90)?;
                frame.restore_ra(c, 0);
                break 'end;
            }
            'result: {
                'not_found: {
                    c.cycles += 6;
                    c.g[A0][0] = 0x51_0000;
                    c.g[A1][0] = sx(r32(c, A1) >> 6);
                    c.g[V1][0] = sx(r32(c, A0).wrapping_sub(0x5300));
                    c.g[V0][0] = c.lhu(r32(c, V1).wrapping_add(SPUMEM_COUNT));
                    let empty = c.g[V0][0] == 0;
                    c.g[A2][0] = 0;
                    if empty {
                        break 'not_found;
                    }
                    c.cycles += 4;
                    c.g[A3][0] = c.g[V1][0];
                    c.g[T0][0] = c.g[A0][0];
                    c.g[V0][0] = c.lw(r32(c, A3).wrapping_add(SPUMEM_LIST));
                    loop {
                        c.cycles += 5;
                        c.g[V1][0] = sx(r32(c, A2) << 2);
                        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                        c.g[A0][0] = c.lhu(r32(c, V0));
                        if c.g[A0][0] != c.g[A1][0] {
                            c.g[V0][0] = c.lhu(r32(c, A3).wrapping_add(SPUMEM_COUNT));
                        } else {
                            'removed: {
                                c.cycles += 6;
                                c.g[V0][0] = c.lhu(r32(c, A3).wrapping_add(SPUMEM_COUNT));
                                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                                c.g[V1][0] = c.g[V0][0] & 0xffff;
                                c.g[V1][0] = ((c.g[A2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                let last = c.g[V1][0] == 0;
                                c.sh(r32(c, A3).wrapping_add(SPUMEM_COUNT), c.g[V0][0]);
                                if last {
                                    break 'removed;
                                }
                                c.cycles += 3;
                                c.g[A0][0] = sx(r32(c, T0).wrapping_sub(0x5300));
                                c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(SPUMEM_LIST));
                                // Shift the later segments down by one entry.
                                loop {
                                    c.cycles += 11;
                                    c.g[V1][0] = sx(r32(c, A2) << 2);
                                    c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                                    c.lwl(2, r32(c, V1).wrapping_add(0x7));
                                    c.lwr(2, r32(c, V1).wrapping_add(0x4));
                                    c.swl(2, r32(c, V1).wrapping_add(0x3));
                                    c.swr(2, r32(c, V1));
                                    c.g[V0][0] = c.lhu(r32(c, A0).wrapping_add(SPUMEM_COUNT));
                                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    if c.g[V0][0] == 0 {
                                        break 'removed;
                                    }
                                    c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(SPUMEM_LIST));
                                    if event_due(c) {
                                        deliver_event_here(c, 0x003c_4df8);
                                    }
                                }
                            }
                            c.cycles += 2;
                            c.g[V0][0] = 0;
                            break 'result;
                        }
                        c.cycles += 4;
                        c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        if c.g[V0][0] == 0 {
                            break 'not_found;
                        }
                        c.g[V0][0] = c.lw(r32(c, A3).wrapping_add(SPUMEM_LIST));
                        if event_due(c) {
                            deliver_event_here(c, 0x003c_4dc0);
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0xffff_ffff_ffff_fff8;
            }
            c.cycles += 1;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Service callbacks and banks.

/// Audio-system field: `i8` number of registered service callbacks.
pub(crate) const SERVICE_COUNT: u32 = 0x178;
/// Audio-system field: the service callback pointers.
pub(crate) const SERVICE_CALLBACKS: u32 = 0x19c;

/// `void SND_AddServiceCallback(void (*client)())` (0x3c6018): appends
/// `client` to the service callback list and counts it.
pub fn snd_add_service_callback(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V0][0] = 0x51_0000;
    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x5718));
    c.g[V1][0] = c.lb(r32(c, V0).wrapping_add(SERVICE_COUNT));
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    c.sw(r32(c, V1).wrapping_add(SERVICE_CALLBACKS), c.g[A0][0]);
    c.g[A0][0] = c.lbu(r32(c, V0).wrapping_add(SERVICE_COUNT));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
    c.sb(r32(c, V0).wrapping_add(SERVICE_COUNT), c.g[A0][0]);
    ret(c);
}

/// Audio-system field: `u16` maximum number of banks.
pub(crate) const BANK_MAX: u32 = 0x28;
/// Audio-system field: pointer to the bank list (0x10-byte entries whose
/// first word is the patch header, 0 when free).
pub(crate) const BANK_LIST: u32 = 0x1f4;

/// `int SND_BankFindFreeSlot()` (0x3c6500): index of the first bank whose
/// patch header is 0, or -9 when all `maxbanks` banks are in use.
pub fn snd_bank_find_free_slot(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = 0x51_0000;
    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x5718));
    c.g[A1][0] = c.lhu(r32(c, V0).wrapping_add(BANK_MAX));
    let none = c.g[A1][0] == 0;
    c.g[V1][0] = 0;
    if !none {
        c.cycles += 1;
        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(BANK_LIST));
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(r32(c, A0));
            if c.g[V0][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = c.g[V1][0];
                ret(c);
                return;
            }
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_6518);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0xffff_ffff_ffff_fff7;
    ret(c);
}

// ---------------------------------------------------------------------------
// IOP defaults.

/// `ldl`/`ldr` pair: load the (unaligned) doubleword at `addr` into `reg`.
fn iop_defaults_load(c: &mut Cpu, reg: usize, addr: u32) {
    c.ldl(reg, addr.wrapping_add(7));
    c.ldr(reg, addr);
}

/// `sdl`/`sdr` pair: store `reg` as an (unaligned) doubleword at `addr`.
fn iop_defaults_store(c: &mut Cpu, reg: usize, addr: u32) {
    c.sdl(reg, addr.wrapping_add(7));
    c.sdr(reg, addr);
}

/// `void SND_IopInitDefaults()` (0x3c7480): copies the default IOP-side
/// settings block (`0x44e1c8`.. in the game image, seven records of 0x18/0x10
/// bytes: 0x1e38, 0x1e20, 0x1e08, 0x1df0, 0x1dd8, 0x1dc0, 0x1db0 below
/// `0x450000`) to the settings block at `0x50ab20`. The scratch registers
/// hold the last addresses and doublewords the original used.
pub fn snd_iop_init_defaults(c: &mut Cpu) {
    c.cycles += 95;
    c.g[V1][0] = 0x51_0000;
    c.g[A0][0] = 0x45_0000;
    c.g[T1][0] = sx(r32(c, A0).wrapping_sub(0x1e38));
    c.g[T0][0] = sx(r32(c, V1).wrapping_sub(0x54e0));
    let (src, dst) = (r32(c, T1), r32(c, T0));
    iop_defaults_load(c, A2, src);
    iop_defaults_load(c, A3, src.wrapping_add(8));
    iop_defaults_store(c, A2, dst);
    iop_defaults_store(c, A3, dst.wrapping_add(8));
    iop_defaults_load(c, A2, src.wrapping_add(0x10));
    iop_defaults_store(c, A2, dst.wrapping_add(0x10));
    c.g[V0][0] = sx(r32(c, V1).wrapping_sub(0x54e0));
    c.g[A1][0] = 0x45_0000;
    c.g[A2][0] = 0x45_0000;
    c.g[T0][0] = sx(r32(c, A1).wrapping_sub(0x1e20));
    let (src, out) = (r32(c, T0), r32(c, V0));
    iop_defaults_load(c, V1, src);
    iop_defaults_load(c, A0, src.wrapping_add(8));
    iop_defaults_load(c, A3, src.wrapping_add(0x10));
    iop_defaults_store(c, V1, out.wrapping_add(0x18));
    iop_defaults_store(c, A0, out.wrapping_add(0x20));
    iop_defaults_store(c, A3, out.wrapping_add(0x28));
    c.g[A3][0] = sx(r32(c, A2).wrapping_sub(0x1e08));
    let src = r32(c, A3);
    iop_defaults_load(c, T1, src);
    iop_defaults_load(c, A0, src.wrapping_add(8));
    iop_defaults_load(c, A1, src.wrapping_add(0x10));
    iop_defaults_store(c, T1, out.wrapping_add(0x30));
    iop_defaults_store(c, A0, out.wrapping_add(0x38));
    iop_defaults_store(c, A1, out.wrapping_add(0x40));
    c.g[V1][0] = 0x45_0000;
    c.g[A2][0] = sx(r32(c, V1).wrapping_sub(0x1df0));
    let src = r32(c, A2);
    iop_defaults_load(c, T0, src);
    iop_defaults_load(c, T1, src.wrapping_add(8));
    iop_defaults_load(c, A1, src.wrapping_add(0x10));
    iop_defaults_store(c, T0, out.wrapping_add(0x48));
    iop_defaults_store(c, T1, out.wrapping_add(0x50));
    iop_defaults_store(c, A1, out.wrapping_add(0x58));
    c.g[A0][0] = 0x45_0000;
    c.g[V1][0] = sx(r32(c, A0).wrapping_sub(0x1dd8));
    let src = r32(c, V1);
    iop_defaults_load(c, A3, src);
    iop_defaults_load(c, T0, src.wrapping_add(8));
    iop_defaults_load(c, T1, src.wrapping_add(0x10));
    iop_defaults_store(c, A3, out.wrapping_add(0x60));
    iop_defaults_store(c, T0, out.wrapping_add(0x68));
    iop_defaults_store(c, T1, out.wrapping_add(0x70));
    c.g[A1][0] = 0x45_0000;
    c.g[V1][0] = 0x45_0000;
    c.g[T0][0] = sx(r32(c, A1).wrapping_sub(0x1dc0));
    let src = r32(c, T0);
    iop_defaults_load(c, A0, src);
    iop_defaults_load(c, A2, src.wrapping_add(8));
    iop_defaults_store(c, A0, out.wrapping_add(0x78));
    iop_defaults_store(c, A2, out.wrapping_add(0x80));
    c.g[A2][0] = sx(r32(c, V1).wrapping_sub(0x1db0));
    let src = r32(c, A2);
    iop_defaults_load(c, T1, src);
    iop_defaults_load(c, A0, src.wrapping_add(8));
    iop_defaults_store(c, T1, out.wrapping_add(0x90));
    iop_defaults_store(c, A0, out.wrapping_add(0x98));
    ret(c);
}

// ---------------------------------------------------------------------------
// EE mixer.

/// The EE mixer block (`0x510000 + 0x5b40`).
pub(crate) const MIXER_BASE: u32 = 0x0051_5b40;
/// Pool bytes left in the mixer's fast RAM (`0x510000 + 0x5d24`).
pub(crate) const MIXER_POOL_LEFT: u32 = 0x5d24;

/// `SND_DecSetAllocFunc` / `SND_DecSetFreeFunc`, the other decoder and DSP
/// setup calls of `SND_EeMixerInit`, and the allocator calls (`SND_Lock`,
/// `SND_MemAlloc`, `SND_Unlock`, `memset` go through the function table).
pub(crate) const MIXER_DEC_SET_ALLOC: Callee = Callee {
    addr: 0x003c_c918,
    f: ee::s14::f_003cc918,
};
pub(crate) const MIXER_DEC_SET_FREE: Callee = Callee {
    addr: 0x003c_c928,
    f: ee::s14::f_003cc928,
};
pub(crate) const MIXER_REGISTER_PCM16: Callee = Callee {
    addr: 0x003c_9268,
    f: ee::s14::f_003c9268,
};
pub(crate) const MIXER_REGISTER_EAXA: Callee = Callee {
    addr: 0x003c_92b0,
    f: ee::s14::f_003c92b0,
};
pub(crate) const MIXER_REGISTER_UTK: Callee = Callee {
    addr: 0x003c_92f8,
    f: ee::s14::f_003c92f8,
};
pub(crate) const MIXER_INIT_MIX_FUNC: Callee = Callee {
    addr: 0x003c_8128,
    f: ee::s14::f_003c8128,
};
pub(crate) const MIXER_CLIP16_INIT: Callee = Callee {
    addr: 0x003c_a7a0,
    f: ee::s14::f_003ca7a0,
};
pub(crate) const MIXER_INSERT_SORTED: Callee = Callee {
    addr: 0x003c_8f48,
    f: ee::s14::f_003c8f48,
};
pub(crate) const MIXER_LOCK: u32 = 0x003b_58a0;
pub(crate) const MIXER_UNLOCK: u32 = 0x003b_58d8;
pub(crate) const MIXER_MEM_ALLOC: u32 = 0x003b_5c60;
pub(crate) const MIXER_MEMSET: u32 = 0x0041_6210;

/// `void SND_EeMixerInit(const SndMixerConfig* cfg)` (0x3c7d00): installs the
/// decoder allocator, copies `cfg` (12 bytes: input channels at +4, output
/// channels at +5) into the mixer block, carves two 0x2058-byte output
/// buffers and one 0x840-byte buffer per output channel out of the fast RAM
/// pool (or `SND_MemAlloc` when the pool is short; each pointer is aligned
/// up to 64 bytes in its second array), allocates and clears the input
/// channel records (0x60 bytes each), registers the PCM16/EAXA/UTK decoders
/// and the mix function, and sets up each output channel's clip stage and
/// sorted DSP chain. The pool loops and the zeroing run under `SND_Lock`.
pub fn snd_ee_mixer_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S1, 0x70);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x80);
        c.g[A0][0] = 0x3c_0000;
        frame.save(c, S4, 0x40);
        frame.save(c, S6, 0x20);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x7cc0));
        frame.save(c, S2, 0x60);
        c.g[S6][0] = 0x51_0000;
        frame.save(c, S3, 0x50);
        c.g[S4][0] = 0x51_0000;
        frame.save(c, S5, 0x30);
        frame.save(c, S7, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = 0;
        MIXER_DEC_SET_ALLOC.call(c, 0x003c_7d44)?;
        c.g[A0][0] = 0x3c_0000;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x7ce0));
        MIXER_DEC_SET_FREE.call(c, 0x003c_7d50)?;
        // Clear the mixer's flag byte(s) (a one-iteration loop).
        c.g[V0][0] = sx(r32(c, S6).wrapping_add(0x5b40));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0xf));
        loop {
            c.cycles += 7;
            c.sb(r32(c, V0), 0);
            c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
            let more = (c.g[S0][0] as i64) >= 0;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_7d58);
            }
        }
        // Copy the configuration into the mixer block and take the lock.
        c.cycles += 19;
        c.g[A1][0] = sx(r32(c, S6).wrapping_add(0x5b40));
        c.ldl(2, r32(c, S1).wrapping_add(0x7));
        c.ldr(2, r32(c, S1));
        c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(0x8));
        c.sdl(2, r32(c, A1).wrapping_add(0x7));
        c.sdr(2, r32(c, A1));
        c.sw(r32(c, A1).wrapping_add(0x8), c.g[V1][0]);
        c.g[S0][0] = 0;
        call_ptr(c, MIXER_LOCK, 0x003c_7d98)?;
        // The pool: its address and size come from the audio-system block.
        c.g[S5][0] = 0x51_0000;
        c.g[V0][0] = 0x51_0000;
        c.g[V1][0] = 0x51_0000;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x5718));
        c.g[S3][0] = sx(r32(c, V1).wrapping_add(0x5be8));
        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0x68));
        c.g[S2][0] = sx(r32(c, S3).wrapping_add(0x8));
        c.g[S1][0] = c.lw(r32(c, V0).wrapping_add(0x64));
        c.sw(r32(c, S4).wrapping_add(MIXER_POOL_LEFT), c.g[A0][0]);
        c.g[A1][0] = c.lw(r32(c, S4).wrapping_add(MIXER_POOL_LEFT));
        // Two 0x2058-byte output buffers.
        loop {
            'placed: {
                'allocate: {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[A1][0] < 0x2058) as u64;
                    let short = c.g[V0][0] != 0;
                    c.g[V0][0] = sx(r32(c, S0) << 2);
                    if short {
                        break 'allocate;
                    }
                    c.cycles += 9;
                    c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x2058));
                    c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, S3)));
                    c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, S2)));
                    c.sw(r32(c, V1), c.g[S1][0]);
                    c.sw(r32(c, A0), c.g[S1][0]);
                    c.g[V1][0] = c.g[V0][0];
                    c.sw(r32(c, S4).wrapping_add(MIXER_POOL_LEFT), c.g[A1][0]);
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x2058));
                    break 'placed;
                }
                c.cycles += 7;
                c.g[A0][0] = 0x2058;
                call_ptr(c, MIXER_MEM_ALLOC, 0x003c_7df8)?;
                c.g[V1][0] = sx(r32(c, S0) << 2);
                c.g[A0][0] = sx(r32(c, V1).wrapping_add(r32(c, S3)));
                c.g[A1][0] = sx(r32(c, V1).wrapping_add(r32(c, S2)));
                c.sw(r32(c, A0), c.g[V0][0]);
                c.sw(r32(c, A1), c.g[V0][0]);
            }
            // Align the second pointer up to 64 bytes.
            'aligned: {
                c.cycles += 6;
                c.g[A0][0] = c.g[V1][0];
                c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, S2)));
                c.g[V0][0] = c.lw(r32(c, V1));
                c.g[V0][0] &= 0x3f;
                let ok = c.g[V0][0] == 0;
                c.g[A1][0] = sx(r32(c, S0).wrapping_add(1));
                if ok {
                    break 'aligned;
                }
                c.cycles += 3;
                c.g[A2][0] = sx(r32(c, S5).wrapping_add(0x5bf0));
                c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, A2)));
                c.g[V0][0] = c.lw(r32(c, V1));
                loop {
                    c.cycles += 7;
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
                    c.sw(r32(c, V1), c.g[V0][0]);
                    c.g[V0][0] &= 0x3f;
                    if c.g[V0][0] == 0 {
                        break 'aligned;
                    }
                    c.g[V0][0] = c.lw(r32(c, V1));
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_7e30);
                    }
                }
            }
            c.cycles += 4;
            c.g[S0][0] = c.g[A1][0];
            c.g[V0][0] = ((c.g[S0][0] as i64) < 2) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = c.lw(r32(c, S4).wrapping_add(MIXER_POOL_LEFT));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_7dc0);
            }
        }
        // One 0x840-byte buffer per output channel.
        'channels: {
            c.cycles += 4;
            c.g[V1][0] = sx(r32(c, S6).wrapping_add(0x5b40));
            c.g[V0][0] = c.lbu(r32(c, V1).wrapping_add(0x5));
            let no_channels = c.g[V0][0] == 0;
            c.g[S0][0] = 0;
            if no_channels {
                break 'channels;
            }
            c.cycles += 5;
            c.g[S5][0] = c.g[V1][0];
            c.g[S7][0] = 0x51_0000;
            c.g[S3][0] = sx(r32(c, S5).wrapping_add(0xb8));
            c.g[S2][0] = sx(r32(c, S5).wrapping_add(0xd0));
            c.g[A1][0] = c.lw(r32(c, S4).wrapping_add(MIXER_POOL_LEFT));
            loop {
                'placed: {
                    'allocate: {
                        c.cycles += 3;
                        c.g[V0][0] = (c.g[A1][0] < 0x840) as u64;
                        let short = c.g[V0][0] != 0;
                        c.g[V0][0] = sx(r32(c, S0) << 2);
                        if short {
                            break 'allocate;
                        }
                        c.cycles += 9;
                        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x840));
                        c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, S3)));
                        c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, S2)));
                        c.sw(r32(c, V1), c.g[S1][0]);
                        c.sw(r32(c, A0), c.g[S1][0]);
                        c.g[V1][0] = c.g[V0][0];
                        c.sw(r32(c, S4).wrapping_add(MIXER_POOL_LEFT), c.g[A1][0]);
                        c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x840));
                        break 'placed;
                    }
                    c.cycles += 7;
                    c.g[A0][0] = 0x840;
                    call_ptr(c, MIXER_MEM_ALLOC, 0x003c_7eb8)?;
                    c.g[V1][0] = sx(r32(c, S0) << 2);
                    c.g[A0][0] = sx(r32(c, V1).wrapping_add(r32(c, S3)));
                    c.g[A1][0] = sx(r32(c, V1).wrapping_add(r32(c, S2)));
                    c.sw(r32(c, A0), c.g[V0][0]);
                    c.sw(r32(c, A1), c.g[V0][0]);
                }
                'aligned: {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[V1][0];
                    c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, S2)));
                    c.g[V0][0] = c.lw(r32(c, V1));
                    c.g[V0][0] &= 0x3f;
                    let ok = c.g[V0][0] == 0;
                    c.g[A1][0] = sx(r32(c, S0).wrapping_add(1));
                    if ok {
                        break 'aligned;
                    }
                    c.cycles += 3;
                    c.g[A2][0] = sx(r32(c, S7).wrapping_add(0x5c10));
                    c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, A2)));
                    c.g[V0][0] = c.lw(r32(c, V1));
                    loop {
                        c.cycles += 7;
                        c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
                        c.sw(r32(c, V1), c.g[V0][0]);
                        c.g[V0][0] &= 0x3f;
                        if c.g[V0][0] == 0 {
                            break 'aligned;
                        }
                        c.g[V0][0] = c.lw(r32(c, V1));
                        if event_due(c) {
                            deliver_event_here(c, 0x003c_7ef0);
                        }
                    }
                }
                c.cycles += 5;
                c.g[V0][0] = c.lbu(r32(c, S5).wrapping_add(0x5));
                c.g[S0][0] = c.g[A1][0];
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = c.lw(r32(c, S4).wrapping_add(MIXER_POOL_LEFT));
                if !more {
                    break 'channels;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_7e80);
                }
            }
        }
        // The input channel records: 0x60 bytes each, allocated and cleared.
        'inputs: {
            c.cycles += 4;
            c.g[S1][0] = sx(r32(c, S6).wrapping_add(0x5b40));
            c.g[V0][0] = c.lbu(r32(c, S1).wrapping_add(0x4));
            let none = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[V0][0];
            if none {
                break 'inputs;
            }
            c.cycles += 10;
            c.g[S0][0] = 0x60;
            c.mult(0, A0, A0, S0);
            call_ptr(c, MIXER_MEM_ALLOC, 0x003c_7f3c)?;
            c.g[A2][0] = c.lbu(r32(c, S1).wrapping_add(0x4));
            c.g[A0][0] = c.g[V0][0];
            c.sw(r32(c, S1).wrapping_add(0x1dc), c.g[V0][0]);
            c.mult(0, 0, A2, S0);
            c.g[A2][0] = c.lo[0];
            c.g[A1][0] = 0;
            call_ptr(c, MIXER_MEMSET, 0x003c_7f58)?;
        }
        // Unlock, register the decoders and set up each channel's DSP chain.
        c.cycles += 13;
        c.g[S0][0] = 0;
        call_ptr(c, MIXER_UNLOCK, 0x003c_7f60)?;
        MIXER_REGISTER_PCM16.call(c, 0x003c_7f68)?;
        MIXER_REGISTER_EAXA.call(c, 0x003c_7f70)?;
        MIXER_REGISTER_UTK.call(c, 0x003c_7f78)?;
        MIXER_INIT_MIX_FUNC.call(c, 0x003c_7f80)?;
        c.g[V0][0] = c.lbu(r32(c, S1).wrapping_add(0x5));
        let none = c.g[V0][0] == 0;
        c.g[S4][0] = c.g[S1][0];
        if !none {
            c.cycles += 5;
            c.g[S3][0] = c.g[S4][0];
            c.g[S2][0] = sx(r32(c, S4).wrapping_add(0x134));
            c.g[S1][0] = sx(r32(c, S4).wrapping_add(0x104));
            c.g[A0][0] = c.g[S2][0];
            loop {
                c.cycles += 19;
                c.sw(r32(c, S1), 0);
                MIXER_CLIP16_INIT.call(c, 0x003c_7fa8)?;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S2][0];
                c.sh(r32(c, S3).wrapping_add(0x14c), 0);
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(0x1c));
                c.sb(r32(c, S3).wrapping_add(0x14e), 0);
                MIXER_INSERT_SORTED.call(c, 0x003c_7fc4)?;
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(0x1c));
                c.g[A0][0] = c.lw(r32(c, S1).wrapping_sub(0x34));
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x800;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                call_ptr(c, MIXER_MEMSET, 0x003c_7fdc)?;
                c.g[V1][0] = c.lbu(r32(c, S4).wrapping_add(0x5));
                c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let more = c.g[V1][0] != 0;
                c.g[A0][0] = c.g[S2][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_7fa0);
                }
            }
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SND_EeMixerInitMixFunc()` (0x3c8128): sets the mixer's mix routine
/// (`0x3cbb78`) and its block size (0x400).
pub fn snd_ee_mixer_init_mix_func(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x51_0000;
    c.g[V1][0] = 0x3d_0000;
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x5b40));
    c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x4488));
    c.g[A0][0] = 0x400;
    c.sw(r32(c, V0).wrapping_add(0x1e0), c.g[V1][0]);
    c.sh(r32(c, V0).wrapping_add(0xc), c.g[A0][0]);
    ret(c);
}

/// `void SND_EeMixerSetMode(int quality)` (0x3c8148): the resampling quality
/// byte (`sndmix.resamplequality`).
pub fn snd_ee_mixer_set_mode(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x51_0000;
    c.sb(r32(c, V0).wrapping_add(0x5b4e), c.g[A0][0]);
    ret(c);
}

/// `void SND_RegisterPcm16Decoders()` (0x3c9268): the PCM16 decoder entry
/// points (`0x3c9ac8`, `0x3c9bc8`, `0x3c9d28`) and sizes (0x2c, 0x30, 0x34)
/// in the mixer block.
pub fn snd_register_pcm16_decoders(c: &mut Cpu) {
    c.cycles += 18;
    c.g[V0][0] = 0x51_0000;
    c.g[A0][0] = 0x3d_0000;
    c.g[A1][0] = 0x3d_0000;
    c.g[A2][0] = 0x3d_0000;
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x5b40));
    c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x6538));
    c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x6438));
    c.g[A2][0] = sx(r32(c, A2).wrapping_sub(0x62d8));
    c.g[A3][0] = 0x2c;
    c.g[T0][0] = 0x30;
    c.g[V1][0] = 0x34;
    c.sw(r32(c, V0).wrapping_add(0x18), c.g[A0][0]);
    c.sw(r32(c, V0).wrapping_add(0x68), c.g[V1][0]);
    c.sw(r32(c, V0).wrapping_add(0x60), c.g[A3][0]);
    c.sw(r32(c, V0).wrapping_add(0x1c), c.g[A1][0]);
    c.sw(r32(c, V0).wrapping_add(0x64), c.g[T0][0]);
    c.sw(r32(c, V0).wrapping_add(0x20), c.g[A2][0]);
    ret(c);
}

/// `void SND_RegisterEaxaDecoders()` (0x3c92b0): the EA-XA decoder entry
/// points (`0x3ca028`, `0x3ca388`, `0x3ca610`) and sizes (0x2c, 0x54, 0x3c).
pub fn snd_register_eaxa_decoders(c: &mut Cpu) {
    c.cycles += 18;
    c.g[V0][0] = 0x51_0000;
    c.g[A0][0] = 0x3d_0000;
    c.g[A1][0] = 0x3d_0000;
    c.g[A2][0] = 0x3d_0000;
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x5b40));
    c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x5fd8));
    c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x5c78));
    c.g[A2][0] = sx(r32(c, A2).wrapping_sub(0x59f0));
    c.g[A3][0] = 0x2c;
    c.g[T0][0] = 0x54;
    c.g[V1][0] = 0x3c;
    c.sw(r32(c, V0).wrapping_add(0x24), c.g[A0][0]);
    c.sw(r32(c, V0).wrapping_add(0x74), c.g[V1][0]);
    c.sw(r32(c, V0).wrapping_add(0x6c), c.g[A3][0]);
    c.sw(r32(c, V0).wrapping_add(0x70), c.g[T0][0]);
    c.sw(r32(c, V0).wrapping_add(0x28), c.g[A1][0]);
    c.sw(r32(c, V0).wrapping_add(0x2c), c.g[A2][0]);
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule};
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{check, fill, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of this module's callee fakes: the magic word at `OC695_CTRL`.
    const OC695_CTRL: u32 = DATA + 0xf_c000;
    const OC695_MAGIC: u32 = 0x6950_0695;

    macro_rules! oc695_fakes {
        ($($name:ident $prev:ident $addr:literal $shard:ident $slot:ident $gen:ident;)*) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC695_CTRL) != OC695_MAGIC {
                        return fakes::fall_back(c, &$prev, ee::$shard::$gen);
                    }
                    fakes::fake_call(c, $addr)
                }
            )*
            fn oc695_install_fakes() {
                $(
                    fakes::hook(addr_of_mut!(ee::$shard::$slot), $name, &$prev);
                )*
            }
        };
    }

    oc695_fakes! {
        oc695_fake_snd_lock OC695_PREV_SND_LOCK 0x003b_58a0 s13 OVERRIDE_003B58A0 f_003b58a0__generated;
        oc695_fake_snd_unlock OC695_PREV_SND_UNLOCK 0x003b_58d8 s13 OVERRIDE_003B58D8 f_003b58d8__generated;
        oc695_fake_snd_mem_alloc OC695_PREV_SND_MEM_ALLOC 0x003b_5c60 s13 OVERRIDE_003B5C60 f_003b5c60__generated;
        oc695_fake_memset OC695_PREV_MEMSET 0x0041_6210 s15 OVERRIDE_00416210 f_00416210__generated;
        oc695_fake_sif_free_sys_memory OC695_PREV_SIF_FREE 0x0042_ad08 s15 OVERRIDE_0042AD08 f_0042ad08__generated;
    }

    /// A scrambled machine with the fakes installed and switched on.
    fn oc695_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32, entry: u32) -> Rng {
        oc695_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.pc = entry;
        c.bus.write32(OC695_CTRL, OC695_MAGIC);
        r
    }

    // -- SND_SifDmaPoll --------------------------------------------------

    /// The queue for `SND_SifDmaPoll`: empty (count 0 or negative), or 1..28
    /// queued with `key` at a random slot (late slots in `long` mode so the
    /// loop runs long enough for events) or absent. The pending time is
    /// always under 500 ticks, so the restart (`SND_SifDmaStartNext`, which
    /// runs the SIF command machinery for real) is never reached.
    fn oc695_poll_setup(c: &mut Cpu, seed: u32, long: bool) -> Rng {
        let mut r = oc695_machine(c, seed, 0x3c_4898, 0, 0x003c_4898);
        let count: u32 = match seed % 5 {
            0 => 0,
            1 => 0x80 | (r.next_u32() & 0x7f),
            _ => 1 + r.next_u32() % 28,
        };
        c.bus.write8(SIFDMA_STATE + SIFDMA_QUEUED, count as u8);
        let key = r.next_u32() | 1;
        let found = seed % 4 != 3;
        let at = if long {
            16 + r.next_u32() % 12
        } else {
            r.next_u32() % 28
        };
        for i in 0..SIFDMA_ENTRY_COUNT {
            let entry = SIFDMA_STATE + SIFDMA_ENTRIES + 0x10 * i;
            let k = if found && i == at {
                key
            } else if r.next_u32() % 3 == 0 {
                0
            } else {
                key ^ (1 << (r.next_u32() % 31 + 1))
            };
            c.bus.write32(entry, k);
        }
        let started = r.next_u32() % 0x1000_0000;
        c.bus.write32(SIFDMA_STATE + SIFDMA_STARTED, started);
        c.bus.write32(
            SND_AUDIO_STATE + SND_AUDIO_TICK,
            started + r.next_u32() % 501,
        );
        c.g[A0][0] = sx(key);
        r
    }

    /// SND_SifDmaPoll: nothing queued (zero or negative count), key absent,
    /// key present and still pending (restart not due). Not covered: the
    /// restart branch (more than 500 ticks pending), which calls the real
    /// `SND_SifDmaStartNext` -> `sceSifSendCmd` chain.
    #[test]
    fn snd_sif_dma_poll_matches() {
        check(
            ee::s13::f_003c4898__generated,
            snd_sif_dma_poll,
            300,
            &|c, seed| {
                oc695_poll_setup(c, seed, false);
            },
        );
    }

    /// SND_SifDmaPoll with events at the loop's back edge.
    #[test]
    fn snd_sif_dma_poll_events() {
        check_events(
            ee::s13::f_003c4898__generated,
            snd_sif_dma_poll,
            300,
            &|c, seed| {
                let mut r = oc695_poll_setup(c, seed, true);
                schedule(c, &mut r, seed, 0x003c_4898)
            },
        );
    }

    // -- SND_SifDmaComplete ----------------------------------------------

    /// The queue for `SND_SifDmaComplete`. A found key always leaves the
    /// queued count at 0 or negative after the decrement (counts 0, 1 and
    /// 0x81..0xff), so `SND_SifDmaKick(1)` does not start another transfer
    /// (that runs the real SIF command chain); an absent key can have any
    /// count.
    fn oc695_complete_setup(c: &mut Cpu, seed: u32, long: bool) -> Rng {
        let mut r = oc695_machine(c, seed, 0x3c_4918, 0, 0x003c_4918);
        let found = seed % 4 != 3;
        let count: u32 = if found {
            [0, 1, 0x81, 0xc0, 0xff][(seed / 4 % 5) as usize]
        } else {
            r.next_u32() & 0xff
        };
        c.bus.write8(SIFDMA_STATE + SIFDMA_QUEUED, count as u8);
        c.bus.write8(SIFDMA_STATE + SIFDMA_BUSY, r.next_u32() as u8);
        let key = r.next_u32() | 1;
        let at = if long {
            16 + r.next_u32() % 12
        } else {
            r.next_u32() % 28
        };
        for i in 0..SIFDMA_ENTRY_COUNT {
            let entry = SIFDMA_STATE + SIFDMA_ENTRIES + 0x10 * i;
            let k = if found && i == at {
                key
            } else if r.next_u32() % 3 == 0 {
                0
            } else {
                key ^ (1 << (r.next_u32() % 31 + 1))
            };
            c.bus.write32(entry, k);
        }
        c.g[A0][0] = sx(key);
        r
    }

    /// SND_SifDmaComplete: key found (the entry is cleared, busy flag
    /// cleared, count decremented, `SND_SifDmaKick(1)` with nothing left to
    /// start) and key absent (scan to the end). Not covered: a found key with
    /// further transfers queued (the real `SND_SifDmaStartNext` chain).
    #[test]
    fn snd_sif_dma_complete_matches() {
        check(
            ee::s13::f_003c4918__generated,
            snd_sif_dma_complete,
            300,
            &|c, seed| {
                oc695_complete_setup(c, seed, false);
            },
        );
    }

    /// SND_SifDmaComplete with events at the loop's back edge.
    #[test]
    fn snd_sif_dma_complete_events() {
        check_events(
            ee::s13::f_003c4918__generated,
            snd_sif_dma_complete,
            300,
            &|c, seed| {
                let mut r = oc695_complete_setup(c, seed, true);
                schedule(c, &mut r, seed, 0x003c_4918)
            },
        );
    }

    // -- SND_SpuMemFree --------------------------------------------------

    const OC695_SEGMENTS: u32 = DATA + 0x1000;

    /// The segment list for `SND_SpuMemFree`: 0..8 (or 20..40 in `long`
    /// mode) `{ key, size }` entries; the key (`address >> 6`) at a random
    /// position (first positions in `long` mode, so many entries shift) or
    /// absent. Play location 0x100 (IOP memory) for a third of the seeds
    /// unless `no_iop`.
    fn oc695_mem_free_setup(c: &mut Cpu, seed: u32, long: bool, no_iop: bool) -> Rng {
        let mut r = oc695_machine(c, seed, 0x3c_4d78, 1, 0x003c_4d78);
        let n = if seed % 7 == 0 {
            0
        } else if long {
            20 + r.next_u32() % 21
        } else {
            1 + r.next_u32() % 8
        };
        c.bus.write16(SIFDMA_STATE + SPUMEM_COUNT, n as u16);
        c.bus.write32(SIFDMA_STATE + SPUMEM_LIST, OC695_SEGMENTS);
        let key = r.next_u32() & 0xffff;
        let found = seed % 4 != 0 && n > 0;
        let at = if long {
            r.next_u32() % 3
        } else {
            r.next_u32() % n.max(1)
        };
        for i in 0..n.max(1) {
            let k = if found && i == at {
                key
            } else {
                (key ^ (1 + r.next_u32() % 0xfffe)) & 0xffff
            };
            c.bus.write16(OC695_SEGMENTS + 4 * i, k as u16);
            c.bus
                .write16(OC695_SEGMENTS + 4 * i + 2, r.next_u32() as u16);
        }
        // Entries past the end hold junk the shift must not disturb.
        for i in n..n + 4 {
            c.bus.write32(OC695_SEGMENTS + 4 * i, r.next_u32());
        }
        let address = (key << 6) | (r.next_u32() & 0x3f);
        c.g[A1][0] = sx(address);
        c.g[A0][0] = if !no_iop && seed % 3 == 0 {
            0x100
        } else {
            r.next_u32() as u64 & !0x100
        };
        r
    }

    /// SND_SpuMemFree: IOP memory (the free call, unwinding on some seeds),
    /// no segments, key absent, key at the last position (no shift) and key
    /// earlier (the following segments shift down).
    #[test]
    fn snd_spu_mem_free_matches() {
        oc695_install_fakes();
        covered(
            ee::s13::f_003c4d78__generated,
            snd_spu_mem_free,
            300,
            &|c, seed| {
                oc695_mem_free_setup(c, seed, false, false);
            },
        );
    }

    /// SND_SpuMemFree with events in the search and shift loops.
    #[test]
    fn snd_spu_mem_free_events() {
        check_events(
            ee::s13::f_003c4d78__generated,
            snd_spu_mem_free,
            300,
            &|c, seed| {
                let mut r = oc695_mem_free_setup(c, seed, true, true);
                schedule(c, &mut r, seed, 0x003c_4d78)
            },
        );
    }

    // -- SND_AddServiceCallback ------------------------------------------

    /// SND_AddServiceCallback: random counts (the count is a signed byte, so
    /// the slot address can sit below the list) and callback pointers.
    #[test]
    fn snd_add_service_callback_matches() {
        check(
            ee::s13::f_003c6018__generated,
            snd_add_service_callback,
            200,
            &|c, seed| {
                let mut r = Rng(0x6018 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003c_6018;
                let count = match seed % 4 {
                    0 => 0,
                    1 => 0xff,
                    2 => 0x7f,
                    _ => r.next_u32() as u8,
                };
                c.bus.write8(SND_AUDIO_STATE + SERVICE_COUNT, count);
            },
        );
    }

    // -- SND_BankFindFreeSlot --------------------------------------------

    const OC695_BANKS: u32 = DATA + 0x2000;

    /// The bank list for `SND_BankFindFreeSlot`: `maxbanks` 0..30 entries of
    /// 0x10 bytes, the first `used` of them taken (a first word of 0 is
    /// free). `long` fills most of the list so the loop runs long enough
    /// for events.
    fn oc695_bank_setup(c: &mut Cpu, seed: u32, long: bool) -> Rng {
        let mut r = oc695_machine(c, seed, 0x3c_6500, 0, 0x003c_6500);
        let max = if long {
            20 + r.next_u32() % 11
        } else if seed % 6 == 0 {
            0
        } else {
            r.next_u32() % 31
        };
        let used = if seed % 3 == 0 {
            max
        } else if long {
            12.min(max) + r.next_u32() % (max - 12.min(max) + 1)
        } else {
            r.next_u32() % (max + 1)
        };
        c.bus.write16(SND_AUDIO_STATE + BANK_MAX, max as u16);
        c.bus.write32(SND_AUDIO_STATE + BANK_LIST, OC695_BANKS);
        for i in 0..max + 2 {
            let first = if i < used { r.next_u32() | 1 } else { 0 };
            c.bus.write32(OC695_BANKS + 0x10 * i, first);
        }
        r
    }

    /// SND_BankFindFreeSlot: no banks, all taken (-9), a free slot at a
    /// random index (including 0).
    #[test]
    fn snd_bank_find_free_slot_matches() {
        check(
            ee::s13::f_003c6500__generated,
            snd_bank_find_free_slot,
            300,
            &|c, seed| {
                oc695_bank_setup(c, seed, false);
            },
        );
    }

    /// SND_BankFindFreeSlot with events at the loop's back edge.
    #[test]
    fn snd_bank_find_free_slot_events() {
        check_events(
            ee::s13::f_003c6500__generated,
            snd_bank_find_free_slot,
            300,
            &|c, seed| {
                let mut r = oc695_bank_setup(c, seed, true);
                schedule(c, &mut r, seed, 0x003c_6500)
            },
        );
    }

    // -- SND_IopInitDefaults ---------------------------------------------

    /// SND_IopInitDefaults: random bytes in the source records (the copy is
    /// unaligned doubleword by doubleword) and in the destination.
    #[test]
    fn snd_iop_init_defaults_matches() {
        check(
            ee::s13::f_003c7480__generated,
            snd_iop_init_defaults,
            100,
            &|c, seed| {
                let mut r = Rng(0x7480 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003c_7480;
                fill(c, &mut r, 0x0044_e1c0, 0x40);
                fill(c, &mut r, 0x0050_ab20, 0x30);
            },
        );
    }

    // -- SND_EeMixerInit -------------------------------------------------

    const OC695_CONFIG: u32 = DATA + 0x8_0000;
    const OC695_POOL: u32 = DATA + 0x9_0000;

    /// The inputs of `SND_EeMixerInit`: a 12-byte configuration (0..4 input
    /// channels, 0..6 output channels), and a fast-RAM pool that is empty,
    /// holds one or both output buffers, holds everything, or is huge (an
    /// unaligned start sometimes). A short pool makes the allocator fake
    /// supply the buffers (16-byte aligned results: the 64-byte alignment
    /// loops step the stored pointer).
    fn oc695_mixer_setup(c: &mut Cpu, seed: u32, many_channels: bool) -> Rng {
        let mut r = oc695_machine(c, seed, 0x3c_7d00, 20, 0x003c_7d00);
        let inputs = (r.next_u32() % 5) as u8;
        let outputs = if many_channels {
            4 + (r.next_u32() % 3) as u8
        } else {
            (r.next_u32() % 7) as u8
        };
        fill(c, &mut r, OC695_CONFIG, 3);
        c.bus.write8(OC695_CONFIG + 4, inputs);
        c.bus.write8(OC695_CONFIG + 5, outputs);
        let ch = outputs as u32;
        let size = match seed % 6 {
            0 => 0,
            1 => 8280,
            2 => 8280 * 2,
            3 => 8280 * 2 + 2112 * ch,
            4 => 0x10000,
            _ => 8280 * 2 + 2112 * (ch / 2),
        };
        c.bus.write32(SND_AUDIO_STATE + 0x68, size);
        let start = OC695_POOL
            + if seed % 5 == 0 {
                4 * (1 + r.next_u32() % 15)
            } else {
                0
            };
        c.bus.write32(SND_AUDIO_STATE + 0x64, start);
        c.g[A0][0] = OC695_CONFIG as u64;
        r
    }

    /// SND_EeMixerInit: pools of every size (the loops that carve from the
    /// pool and that call the allocator), 0..6 output channels, 0..4 input
    /// channels, and callees that unwind. The lock, unlock, allocator and
    /// `memset` are gated fakes; the decoder registration, clip init and
    /// `SND_DspInsertSorted` run for real.
    #[test]
    fn snd_ee_mixer_init_matches() {
        covered(
            ee::s14::f_003c7d00__generated,
            snd_ee_mixer_init,
            300,
            &|c, seed| {
                oc695_mixer_setup(c, seed, false);
            },
        );
    }

    /// SND_EeMixerInit with events at its loops' back edges.
    #[test]
    fn snd_ee_mixer_init_events() {
        check_events(
            ee::s14::f_003c7d00__generated,
            snd_ee_mixer_init,
            300,
            &|c, seed| {
                let mut r = oc695_mixer_setup(c, seed, true);
                schedule(c, &mut r, seed, 0x003c_7d00)
            },
        );
    }

    // -- leaves ----------------------------------------------------------

    /// SND_EeMixerInitMixFunc: stores the routine pointer and block size.
    #[test]
    fn snd_ee_mixer_init_mix_func_matches() {
        check(
            ee::s14::f_003c8128__generated,
            snd_ee_mixer_init_mix_func,
            20,
            &|c, seed| {
                let mut r = Rng(0x8128 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003c_8128;
            },
        );
    }

    /// SND_EeMixerSetMode: the quality byte, from random argument values.
    #[test]
    fn snd_ee_mixer_set_mode_matches() {
        check(
            ee::s14::f_003c8148__generated,
            snd_ee_mixer_set_mode,
            50,
            &|c, seed| {
                let mut r = Rng(0x8148 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003c_8148;
            },
        );
    }

    /// SND_RegisterPcm16Decoders: stores the decoder table entries.
    #[test]
    fn snd_register_pcm16_decoders_matches() {
        check(
            ee::s14::f_003c9268__generated,
            snd_register_pcm16_decoders,
            20,
            &|c, seed| {
                let mut r = Rng(0x9268 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003c_9268;
            },
        );
    }

    /// SND_RegisterEaxaDecoders: stores the decoder table entries.
    #[test]
    fn snd_register_eaxa_decoders_matches() {
        check(
            ee::s14::f_003c92b0__generated,
            snd_register_eaxa_decoders,
            20,
            &|c, seed| {
                let mut r = Rng(0x92b0 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003c_92b0;
            },
        );
    }
}
