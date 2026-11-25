//! Staging module oc693 (wave wave237): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc693`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x003c_1b80, snd_iop_init, "SND_IopInit")];

/// Entry and end of `SND_IopInit`.
const SND_IOP_INIT_ENTRY: u32 = 0x003c_1b80;
const SND_IOP_INIT_END: u32 = 0x003c_2264;

/// `int SND_IopInit(void)` (0x3c1b80, SNDPLATFORM_init): start the sound
/// driver. Allocates the channel table and the IOP command buffer for the
/// configured sample rate, the encoder state and the ring buffer, creates the
/// driver semaphore, loads the IOP modules (first start only), allocates the
/// IOP heap block, starts the driver thread, creates the mixer, waits for the
/// IOP RPC server, picks the frame period and sends the init request.
/// Returns 0, or -1 when a module load or the RPC bind fails.
///
/// Globals: `sndgs` = 0x50a8e8 (settings: +0x28 voices, +0x2a sample rate,
/// +0x34 sync period, +0x3f load modules, +0x47/+0x48 buffer KiB, +0x49 flags,
/// +0x4a, +0x4c format count, +0x4e[] format flags, +0x64/+0x68 encoder
/// state, +0x6c module path, +0x1c4 frame period); `snd_drv` = 0x50ad00
/// (+0x1aa0 channels, +0x1aa4 semaphore, +0x1aa8 VBlank handler id, +0x1aad
/// started, +0x1aae encoder size, +0x1ab0 command size, +0x1ad4.. ring
/// buffer, +0x1af8/+0x1afc encoder state); `started` = 0x44dd40.
///
/// A resumable dispatcher: the generated body is a `match pc` over its
/// blocks, and the event checks at the loop back edges (0x3c1eec, 0x3c1ff8,
/// 0x3c2090, 0x3c20e8) can unwind and re-enter at those heads, so the port
/// keeps the block structure (cycle charges per block, delay slots and all
/// registers as the generated body leaves them).
pub fn snd_iop_init(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                // Prologue (frame 0x100, s0..s4, ra), clear the encoder size, enter the driver lock, allocate the channel table (voices * 4), pick the IOP command size from the sample rate.
                0x003c1b80 => {
                    c.cycles += 22;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x100));
                    c.g[V0][0] = 0x510000;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.g[S0][0] = 0x510000;
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5300));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.g[S3][0] = c.g[V0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.g[S4][0] = c.g[S0][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[RA][0]);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x5718));
                    c.sh((c.g[S1][0] as u32).wrapping_add(0x1aae), 0u64);
                    Callee {
                        addr: 0x003b58a0,
                        f: ee::s13::SND_Lock,
                    }
                    .call(c, 0x003c1bbc)?;
                    c.g[A0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x28));
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                    Callee {
                        addr: 0x003b5c60,
                        f: ee::s13::SND_MemAlloc,
                    }
                    .call(c, 0x003c1bc8)?;
                    c.g[A0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x2a));
                    c.g[V1][0] = 0x5dc0;
                    let t = c.g[A0][0] != c.g[V1][0];
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1aa0), c.g[V0][0]);
                    if t {
                        pc = 0x003c1be0;
                        continue;
                    }
                    pc = 0x003c1bd8;
                    continue;
                }
                // 24000 Hz: command size 0x100.
                0x003c1bd8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0x100;
                    if t {
                        pc = 0x003c1bf0;
                        continue;
                    }
                    pc = 0x003c1be0;
                    continue;
                }
                // Not 24000 Hz: 36000 Hz gives 0x180, anything else 0x200.
                0x003c1be0 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x8ca0;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[V0][0] = 0x200;
                    if t {
                        pc = 0x003c1bf0;
                        continue;
                    }
                    pc = 0x003c1bec;
                    continue;
                }
                0x003c1bec => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x180;
                    pc = 0x003c1bf0;
                    continue;
                }
                // Store the command size, SND_InitIopCmdBuffer, set the all-ones mask at +8 of the command block; no encoder (flags & 2 clear) skips to the SPU setup.
                0x003c1bf0 => {
                    c.cycles += 12;
                    c.sh((c.g[S1][0] as u32).wrapping_add(0x1ab0), c.g[V0][0]);
                    Callee {
                        addr: 0x003c1638,
                        f: ee::s13::SND_InitIopCmdBuffer,
                    }
                    .call(c, 0x003c1bf8)?;
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x5300));
                    c.g[V1][0] = 0xffffffffffffffff;
                    c.g[V1][0] = c.g[V1][0] >> 16;
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcb0));
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x5718));
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x49));
                    c.g[V0][0] = c.g[V0][0] & 0x2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x5;
                    if t {
                        pc = 0x003c1db4;
                        continue;
                    }
                    pc = 0x003c1c20;
                    continue;
                }
                // Encoder buffers: query the sizes (callback at 0x50c840, mode 2 for format 5, else 1).
                0x003c1c20 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x47));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V1][0] = 0x510000;
                    if t {
                        pc = 0x003c1c4c;
                        continue;
                    }
                    pc = 0x003c1c2c;
                    continue;
                }
                0x003c1c2c => {
                    c.cycles += 8;
                    c.g[A1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x4a));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x37c0));
                    c.g[A0][0] = 0x2;
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                    let t = c.g[V0][0] as u32;
                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x94));
                    call_ptr(c, t, 0x003c1c44)?;
                    let t = true;
                    c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x5718));
                    if t {
                        pc = 0x003c1c68;
                        continue;
                    }
                    pc = 0x003c1c4c;
                    continue;
                }
                0x003c1c4c => {
                    c.cycles += 7;
                    c.g[A1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x4a));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x37c0));
                    c.g[A0][0] = 0x1;
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                    let t = c.g[V0][0] as u32;
                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x94));
                    call_ptr(c, t, 0x003c1c64)?;
                    c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x5718));
                    pc = 0x003c1c68;
                    continue;
                }
                // Pick the encoder-state allocation: with a preset size (+0x64) and size1 below +0x68 a separate state of size0, else one block of size0 + size1.
                0x003c1c68 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x94));
                    if t {
                        pc = 0x003c1ca4;
                        continue;
                    }
                    pc = 0x003c1c74;
                    continue;
                }
                0x003c1c74 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x003c1ca8;
                        continue;
                    }
                    pc = 0x003c1c84;
                    continue;
                }
                0x003c1c84 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x90));
                    Callee {
                        addr: 0x003b5c60,
                        f: ee::s13::SND_MemAlloc,
                    }
                    .call(c, 0x003c1c8c)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64));
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x5300));
                    c.g[A1][0] = c.lhu((c.g[SP][0] as u32).wrapping_add(0x94));
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x1af8), c.g[V0][0]);
                    let t = true;
                    c.sh((c.g[A0][0] as u32).wrapping_add(0x1aae), c.g[A1][0]);
                    if t {
                        pc = 0x003c1cc0;
                        continue;
                    }
                    pc = 0x003c1ca4;
                    continue;
                }
                0x003c1ca4 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x90));
                    pc = 0x003c1ca8;
                    continue;
                }
                0x003c1ca8 => {
                    c.cycles += 6;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    Callee {
                        addr: 0x003b5c60,
                        f: ee::s13::SND_MemAlloc,
                    }
                    .call(c, 0x003c1cb0)?;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x5300));
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x1af8), c.g[V0][0]);
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    pc = 0x003c1cc0;
                    continue;
                }
                // Hand the encoder state to the init callback (0x50c844).
                0x003c1cc0 => {
                    c.cycles += 6;
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x1afc), c.g[V1][0]);
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x5718));
                    c.g[V1][0] = 0x5;
                    c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x47));
                    let t = c.g[V0][0] != c.g[V1][0];
                    c.g[V1][0] = 0x510000;
                    if t {
                        pc = 0x003c1cf8;
                        continue;
                    }
                    pc = 0x003c1cd8;
                    continue;
                }
                0x003c1cd8 => {
                    c.cycles += 8;
                    c.g[A3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x5300));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x37bc));
                    c.g[A0][0] = 0x2;
                    c.g[A1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x4a));
                    let t = c.g[V0][0] as u32;
                    c.g[A2][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1af8));
                    call_ptr(c, t, 0x003c1cf0)?;
                    let t = true;
                    c.g[S1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x5718));
                    if t {
                        pc = 0x003c1d14;
                        continue;
                    }
                    pc = 0x003c1cf8;
                    continue;
                }
                0x003c1cf8 => {
                    c.cycles += 7;
                    c.g[A3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x5300));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x37bc));
                    c.g[A0][0] = 0x1;
                    c.g[A1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x4a));
                    let t = c.g[V0][0] as u32;
                    c.g[A2][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1af8));
                    call_ptr(c, t, 0x003c1d10)?;
                    c.g[S1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x5718));
                    pc = 0x003c1d14;
                    continue;
                }
                // Allocate and clear the ring buffer ((fmt + chans) KiB + 0x40), align to 64.
                0x003c1d14 => {
                    c.cycles += 21;
                    c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x5300));
                    c.g[S0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x47));
                    c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x48));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[S0][0] = sx((c.g[S0][0] as u32) << 10);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x003b5c60,
                        f: ee::s13::SND_MemAlloc,
                    }
                    .call(c, 0x003c1d34)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = c.g[S0][0];
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x1ad4), c.g[V0][0]);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x00416210, 0x003c1d48)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1ad4));
                    c.g[A0][0] = 0x4;
                    c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x47));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3f));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) >> 6);
                    c.g[A3][0] = sx((c.g[V1][0] as u32) << 6);
                    let t = c.g[V0][0] != c.g[A0][0];
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x1ad8), c.g[A3][0]);
                    if t {
                        pc = 0x003c1d88;
                        continue;
                    }
                    pc = 0x003c1d68;
                    continue;
                }
                // Format 4 buffer layout.
                0x003c1d68 => {
                    c.cycles += 8;
                    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x400));
                    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0xc00));
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x800));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x1ae8), c.g[V0][0]);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x1adc), c.g[V1][0]);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x1ae4), c.g[A0][0]);
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x1ae0), c.g[A3][0]);
                    if t {
                        pc = 0x003c1db4;
                        continue;
                    }
                    pc = 0x003c1d88;
                    continue;
                }
                // Other formats buffer layout.
                0x003c1d88 => {
                    c.cycles += 11;
                    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1400));
                    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1000));
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x400));
                    c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(0xc00));
                    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x800));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x1af0), c.g[V0][0]);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x1ae0), c.g[V1][0]);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x1ae4), c.g[A0][0]);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x1ae8), c.g[A1][0]);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x1aec), c.g[A2][0]);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x1adc), c.g[A3][0]);
                    pc = 0x003c1db4;
                    continue;
                }
                // SPU FX volume, channel limit, unlock, create the driver semaphore; first start only: sceSifInitRpc and the IOP heap.
                0x003c1db4 => {
                    c.cycles += 25;
                    c.g[A0][0] = 0u64;
                    c.g[A1][0] = 0x8;
                    Callee {
                        addr: 0x003c36c8,
                        f: ee::s13::SND_SetSpuFxVolume,
                    }
                    .call(c, 0x003c1dc0)?;
                    c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x5300));
                    c.g[S1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x5718));
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcb0));
                    c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x41));
                    c.g[V1][0] = 0x141;
                    c.g[A0][0] = 0x8000;
                    c.sb((c.g[A1][0] as u32).wrapping_add(0x13), c.g[V0][0]);
                    c.sh((c.g[S0][0] as u32).wrapping_add(0x1a9a), c.g[V1][0]);
                    c.sh((c.g[S0][0] as u32).wrapping_add(0x1a9c), c.g[A0][0]);
                    Callee {
                        addr: 0x003b58d8,
                        f: ee::s13::SND_Unlock,
                    }
                    .call(c, 0x003c1de8)?;
                    c.g[V0][0] = 0x510000;
                    c.g[V1][0] = 0x1;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5350));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    c.g[A0][0] = c.g[V0][0];
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                    call_ptr(c, 0x00423da0, 0x003c1e04)?;
                    c.g[V1][0] = 0x450000;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1aa4), c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x22c0));
                    let t = c.g[V0][0] != 0u64;
                    c.g[S2][0] = c.g[V1][0];
                    if t {
                        pc = 0x003c1e28;
                        continue;
                    }
                    pc = 0x003c1e18;
                    continue;
                }
                0x003c1e18 => {
                    c.cycles += 4;
                    c.g[A0][0] = 0u64;
                    call_ptr(c, 0x00426408, 0x003c1e20)?;
                    call_ptr(c, 0x0042ab18, 0x003c1e28)?;
                    pc = 0x003c1e28;
                    continue;
                }
                // Optional IOP module loading (first start, load_modules set): try the configured path then the fallback for the two IRX modules; any failure returns -1.
                0x003c1e28 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x3f));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x22c0));
                    if t {
                        pc = 0x003c1f2c;
                        continue;
                    }
                    pc = 0x003c1e34;
                    continue;
                }
                0x003c1e34 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x510000;
                    if t {
                        pc = 0x003c1f30;
                        continue;
                    }
                    pc = 0x003c1e3c;
                    continue;
                }
                0x003c1e3c => {
                    c.cycles += 14;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
                    c.g[A0][0] = c.g[SP][0];
                    c.sb((c.g[SP][0] as u32), 0u64);
                    Callee {
                        addr: 0x003c15d8,
                        f: ee::s13::SND_StrCat,
                    }
                    .call(c, 0x003c1e4c)?;
                    c.g[A1][0] = 0x490000;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5830));
                    Callee {
                        addr: 0x003c15d8,
                        f: ee::s13::SND_StrCat,
                    }
                    .call(c, 0x003c1e5c)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x0042ba68, 0x003c1e6c)?;
                    let t = (c.g[V0][0] as i64) >= 0;
                    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x5718));
                    if t {
                        pc = 0x003c1e90;
                        continue;
                    }
                    pc = 0x003c1e74;
                    continue;
                }
                0x003c1e74 => {
                    c.cycles += 7;
                    c.g[A0][0] = 0x490000;
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5840));
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x0042ba68, 0x003c1e88)?;
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x5718));
                    if t {
                        pc = 0x003c1ee4;
                        continue;
                    }
                    pc = 0x003c1e90;
                    continue;
                }
                0x003c1e90 => {
                    c.cycles += 14;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x6c));
                    c.sb((c.g[SP][0] as u32), 0u64);
                    Callee {
                        addr: 0x003c15d8,
                        f: ee::s13::SND_StrCat,
                    }
                    .call(c, 0x003c1ea0)?;
                    c.g[A1][0] = 0x490000;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5870));
                    Callee {
                        addr: 0x003c15d8,
                        f: ee::s13::SND_StrCat,
                    }
                    .call(c, 0x003c1eb0)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x0042ba68, 0x003c1ec0)?;
                    let t = (c.g[V0][0] as i64) >= 0;
                    c.g[V0][0] = 0x510000;
                    if t {
                        pc = 0x003c1f30;
                        continue;
                    }
                    pc = 0x003c1ec8;
                    continue;
                }
                0x003c1ec8 => {
                    c.cycles += 7;
                    c.g[A0][0] = 0x490000;
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5880));
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x0042ba68, 0x003c1edc)?;
                    let t = (c.g[V0][0] as i64) >= 0;
                    c.g[V0][0] = 0x510000;
                    if t {
                        pc = 0x003c1f30;
                        continue;
                    }
                    pc = 0x003c1ee4;
                    continue;
                }
                0x003c1ee4 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        pc = 0x003c2244;
                        continue;
                    }
                    pc = 0x003c1eec;
                    continue;
                }
                // Mixer init for a format entry with the 0x4 flag (SND_EeMixerInit through the table).
                0x003c1eec => {
                    c.cycles += 16;
                    c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x47));
                    c.g[V1][0] = 0x3c0000;
                    c.g[A2][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xbe8));
                    c.g[A3][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2a));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[A1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x2e));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x85), c.g[V0][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x84), c.g[A1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x80), c.g[A3][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x88), c.g[V1][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x86), 0u64);
                    call_ptr(c, 0x003c7d00, 0x003c1f24)?;
                    let t = true;
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x5718));
                    if t {
                        pc = 0x003c2018;
                        continue;
                    }
                    pc = 0x003c1f2c;
                    continue;
                }
                0x003c1f2c => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x510000;
                    pc = 0x003c1f30;
                    continue;
                }
                // Allocate the IOP heap block (a loop that runs once).
                0x003c1f30 => {
                    c.cycles += 4;
                    c.g[S1][0] = 0u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5300));
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xe94));
                    pc = 0x003c1f40;
                    continue;
                }
                0x003c1f40 => {
                    c.cycles += 7;
                    c.g[A0][0] = 0x1000;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x0042aba0, 0x003c1f4c)?;
                    c.sw((c.g[S0][0] as u32).wrapping_sub(0x8), c.g[V0][0]);
                    c.sw((c.g[S0][0] as u32), 0u64);
                    let t = c.g[S1][0] == 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                    if t {
                        if back_edge_unwinds(c, 0x003c1f40) {
                            return Ok(());
                        }
                        pc = 0x003c1f40;
                        continue;
                    }
                    pc = 0x003c1f5c;
                    continue;
                }
                // Create and start the driver thread, priority 100, group effects 8 and 4, then the mixer-flag scan.
                0x003c1f5c => {
                    c.cycles += 37;
                    c.g[V0][0] = 0x510000;
                    c.g[A1][0] = 0x3c0000;
                    c.g[V1][0] = 0x510000;
                    c.g[A2][0] = 0x4a0000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5380));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x30f0));
                    c.g[A3][0] = 0x6000;
                    c.g[T0][0] = 0x6;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x19a8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x34e0));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[A3][0]);
                    c.g[A0][0] = c.g[V0][0];
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[T0][0]);
                    c.g[S1][0] = 0u64;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x20), 0u64);
                    call_ptr(c, 0x00423ba0, 0x003c1fa8)?;
                    c.g[V1][0] = 0x510000;
                    c.g[A1][0] = 0u64;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x2b20), c.g[V0][0]);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x00423bc0, 0x003c1fbc)?;
                    call_ptr(c, 0x00423c90, 0x003c1fc4)?;
                    c.g[A1][0] = 0x64;
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x00423c30, 0x003c1fd0)?;
                    c.g[A0][0] = 0x8;
                    Callee {
                        addr: 0x003c35f8,
                        f: ee::s13::SND_ApplyGroupEffect,
                    }
                    .call(c, 0x003c1fd8)?;
                    c.g[A0][0] = 0x4;
                    Callee {
                        addr: 0x003c35f8,
                        f: ee::s13::SND_ApplyGroupEffect,
                    }
                    .call(c, 0x003c1fe0)?;
                    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x5718));
                    c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[V1][0];
                    if t {
                        pc = 0x003c2014;
                        continue;
                    }
                    pc = 0x003c1ff0;
                    continue;
                }
                0x003c1ff0 => {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4e));
                    pc = 0x003c1ff8;
                    continue;
                }
                0x003c1ff8 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lhu((c.g[V1][0] as u32));
                    c.g[V0][0] = c.g[V0][0] & 0x4;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x003c1eec) {
                            return Ok(());
                        }
                        pc = 0x003c1eec;
                        continue;
                    }
                    pc = 0x003c2008;
                    continue;
                }
                0x003c2008 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S1][0] < c.g[A0][0]) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
                    if t {
                        if back_edge_unwinds(c, 0x003c1ff8) {
                            return Ok(());
                        }
                        pc = 0x003c1ff8;
                        continue;
                    }
                    pc = 0x003c2014;
                    continue;
                }
                0x003c2014 => {
                    c.cycles += 1;
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x5718));
                    pc = 0x003c2018;
                    continue;
                }
                // Raise the encoder size to the preset minus the shared word, allocate the +0x1b38 buffer.
                0x003c2018 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x64));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A3][0] = 0x510000;
                    if t {
                        pc = 0x003c2050;
                        continue;
                    }
                    pc = 0x003c2024;
                    continue;
                }
                0x003c2024 => {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x68));
                    c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x5d24));
                    c.g[A2][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x5300));
                    c.g[V1][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x1aae));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
                    c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V1][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x5d24));
                    if t {
                        pc = 0x003c2050;
                        continue;
                    }
                    pc = 0x003c2044;
                    continue;
                }
                0x003c2044 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x68));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                    c.sh((c.g[A2][0] as u32).wrapping_add(0x1aae), c.g[V0][0]);
                    pc = 0x003c2050;
                    continue;
                }
                0x003c2050 => {
                    c.cycles += 4;
                    c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x5300));
                    c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1aae));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x22c0));
                    if t {
                        pc = 0x003c2080;
                        continue;
                    }
                    pc = 0x003c2060;
                    continue;
                }
                0x003c2060 => {
                    c.cycles += 8;
                    Callee {
                        addr: 0x003b58a0,
                        f: ee::s13::SND_Lock,
                    }
                    .call(c, 0x003c2068)?;
                    c.g[A0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1aae));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                    Callee {
                        addr: 0x003b5c60,
                        f: ee::s13::SND_MemAlloc,
                    }
                    .call(c, 0x003c2074)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1b38), c.g[V0][0]);
                    Callee {
                        addr: 0x003b58d8,
                        f: ee::s13::SND_Unlock,
                    }
                    .call(c, 0x003c207c)?;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x22c0));
                    pc = 0x003c2080;
                    continue;
                }
                0x003c2080 => {
                    c.cycles += 2;

                    if c.g[V0][0] != 0u64 {
                        c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x5718));
                        pc = 0x003c2110;
                        continue;
                    }
                    pc = 0x003c2088;
                    continue;
                }
                0x003c2088 => {
                    c.cycles += 2;
                    c.g[V1][0] = 0x10000;
                    pc = 0x003c2090;
                    continue;
                }
                // First start: wait for interrupts off, set the SIF command buffer and the handler, re-enable, then bind the RPC server until the server answers.
                0x003c2090 => {
                    c.cycles += 7;
                    c.di();
                    c.mfc0(2, 12);
                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = 0x510000;
                    if t {
                        if back_edge_unwinds(c, 0x003c2090) {
                            return Ok(());
                        }
                        pc = 0x003c2090;
                        continue;
                    }
                    pc = 0x003c20ac;
                    continue;
                }
                0x003c20ac => {
                    c.cycles += 15;
                    c.g[A1][0] = 0x10;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3780));
                    call_ptr(c, 0x00425ff0, 0x003c20b8)?;
                    c.g[A1][0] = 0x3c0000;
                    c.g[A2][0] = 0x510000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1578));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x3700));
                    c.g[A0][0] = 0x1;
                    call_ptr(c, 0x00426020, 0x003c20d0)?;
                    c.ei();
                    c.g[V0][0] = 0x510000;
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4680));
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xc80));
                    c.g[A1][0] = 0x530000;
                    pc = 0x003c20e8;
                    continue;
                }
                0x003c20e8 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[A1][0] | 0x4e44;
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x00426b48, 0x003c20f8)?;
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        pc = 0x003c2244;
                        continue;
                    }
                    pc = 0x003c2100;
                    continue;
                }
                0x003c2100 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xca4));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = 0x530000;
                    if t {
                        if back_edge_unwinds(c, 0x003c20e8) {
                            return Ok(());
                        }
                        pc = 0x003c20e8;
                        continue;
                    }
                    pc = 0x003c210c;
                    continue;
                }
                0x003c210c => {
                    c.cycles += 1;
                    c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x5718));
                    pc = 0x003c2110;
                    continue;
                }
                // Frame period: external sync (-1.0) hooks the VBlank handler (INTC 2), else a fixed 10.67 ms period.
                0x003c2110 => {
                    c.cycles += 7;
                    c.g[AT][0] = 0xffffffffbf800000;
                    c.mtc1(1, 1);
                    c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x34));
                    c.c_eq_s(0, 1);
                    let t = !c.fcc();
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x5300));
                    if t {
                        pc = 0x003c2190;
                        continue;
                    }
                    pc = 0x003c212c;
                    continue;
                }
                0x003c212c => {
                    c.cycles += 8;
                    call_ptr(c, 0x003fdaa0, 0x003c2134)?;
                    c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x2));
                    c.g[V0][0] = 0x2;
                    c.g[AT][0] = 0x41a00000;
                    c.mtc1(1, 0);

                    if c.g[V1][0] != c.g[V0][0] {
                        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x1c4));
                        pc = 0x003c215c;
                        continue;
                    }
                    pc = 0x003c214c;
                    continue;
                }
                0x003c214c => {
                    c.cycles += 4;
                    c.g[AT][0] = 0x41850000;
                    c.g[AT][0] = c.g[AT][0] | 0x7780;
                    c.mtc1(1, 0);
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x1c4));
                    pc = 0x003c215c;
                    continue;
                }
                0x003c215c => {
                    c.cycles += 13;
                    c.g[A0][0] = 0x2;
                    call_ptr(c, 0x00424880, 0x003c2164)?;
                    c.g[A1][0] = 0x3c0000;
                    c.g[A0][0] = 0x2;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1980));
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x00423a80, 0x003c2178)?;
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x5300));
                    c.g[A0][0] = 0x2;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x1aa8), c.g[V0][0]);
                    call_ptr(c, 0x004248e8, 0x003c2188)?;
                    let t = true;
                    c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x5718));
                    if t {
                        pc = 0x003c21ac;
                        continue;
                    }
                    pc = 0x003c2190;
                    continue;
                }
                0x003c2190 => {
                    c.cycles += 7;
                    c.g[AT][0] = 0x412a0000;
                    c.g[AT][0] = c.g[AT][0] | 0xaaab;
                    c.mtc1(1, 0);
                    c.g[V0][0] = 0xffffffffffffffff;
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x1c4));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x1aa8), c.g[V0][0]);
                    c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x5718));
                    pc = 0x003c21ac;
                    continue;
                }
                // Build the IOP init request (0x80303) and send it with SND_IopCallRpc; mark started, SND_IopInitDefaults, return 0.
                0x003c21ac => {
                    c.cycles += 24;
                    c.g[A1][0] = 0x510000;
                    c.g[T3][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x47));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x48c0));
                    c.g[V0][0] = 0x80000;
                    c.g[T2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1098));
                    c.g[T0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = 0x510000;
                    c.g[T1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x2f));
                    c.g[V0][0] = c.g[V0][0] | 0x303;
                    c.g[A3][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x2a));
                    c.g[T4][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5338));
                    c.g[A2][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x2c));
                    c.g[A0][0] = c.g[T3][0];
                    c.sw((c.g[V1][0] as u32).wrapping_sub(0x5338), c.g[V0][0]);
                    c.g[T5][0] = c.g[V1][0];
                    c.g[A0][0] = (c.g[A0][0] < 0x4u64) as u64;
                    c.sw((c.g[T4][0] as u32).wrapping_add(0x8), c.g[T2][0]);
                    c.sb((c.g[T4][0] as u32).wrapping_add(0x11), c.g[T0][0]);
                    c.sb((c.g[T4][0] as u32).wrapping_add(0x12), c.g[T1][0]);
                    c.sh((c.g[T4][0] as u32).wrapping_add(0xc), c.g[A3][0]);
                    c.sh((c.g[T4][0] as u32).wrapping_add(0xe), c.g[A2][0]);
                    c.sw((c.g[T4][0] as u32).wrapping_add(0x4), c.g[A1][0]);
                    let t = c.g[A0][0] != 0u64;
                    c.sb((c.g[T4][0] as u32).wrapping_add(0x10), c.g[T3][0]);
                    if t {
                        pc = 0x003c2218;
                        continue;
                    }
                    pc = 0x003c220c;
                    continue;
                }
                0x003c220c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x43));
                    let t = true;
                    c.sb((c.g[T4][0] as u32).wrapping_add(0x13), c.g[V0][0]);
                    if t {
                        pc = 0x003c221c;
                        continue;
                    }
                    pc = 0x003c2218;
                    continue;
                }
                0x003c2218 => {
                    c.cycles += 1;
                    c.sb((c.g[T4][0] as u32).wrapping_add(0x13), 0u64);
                    pc = 0x003c221c;
                    continue;
                }
                0x003c221c => {
                    c.cycles += 10;
                    c.g[A1][0] = sx((c.g[T5][0] as u32).wrapping_sub(0x5338));
                    c.g[A0][0] = 0u64;
                    c.g[A2][0] = 0x14;
                    Callee {
                        addr: 0x003c0b10,
                        f: ee::s13::SND_IopCallRpc,
                    }
                    .call(c, 0x003c222c)?;
                    c.g[V0][0] = 0x1;
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x5300));
                    c.sb((c.g[V1][0] as u32).wrapping_add(0x1aad), c.g[V0][0]);
                    c.sw((c.g[S2][0] as u32).wrapping_sub(0x22c0), c.g[V0][0]);
                    Callee {
                        addr: 0x003c7480,
                        f: ee::s13::SND_IopInitDefaults,
                    }
                    .call(c, 0x003c2240)?;
                    c.g[V0][0] = 0u64;
                    pc = 0x003c2244;
                    continue;
                }
                // Epilogue.
                0x003c2244 => {
                    c.cycles += 8;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                    c.pc = t;
                    return Ok(());
                }
                pc => {
                    c.jump_out(pc, SND_IOP_INIT_ENTRY, SND_IOP_INIT_END);
                    return Ok(());
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, fake, machine, FAKE};
    use crate::difftest::{fill, DATA, SP as TEST_STACK_TOP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The sound settings block (`sndgs`).
    const SNDGS: u32 = 0x0050_a8e8;
    /// The IOP sound driver block.
    const SND_DRV: u32 = 0x0050_ad00;
    /// "Driver already started" flag.
    const SND_STARTED: u32 = 0x0044_dd40;
    /// Encoder buffer size/init callbacks (function pointers).
    const ENCODER_SIZE_CALLBACK: u32 = 0x0050_c840;
    const ENCODER_INIT_CALLBACK: u32 = 0x0050_c844;
    /// `sceSifInitRpc`'s "already initialised" word and the RPC packet pool
    /// length (0: every bind fails at once).
    const SIF_INIT_RPC_DONE: u32 = 0x0045_5180;
    const SIF_RPC_POOL_COUNT: u32 = 0x0052_d688;
    /// `sceGsGetGParam` returns this address (the mode half-word at +2).
    const GS_GPARAM: u32 = 0x0045_1550;

    /// Gate of the `sceSifLoadModule` fake: the machine's RAM holds the magic
    /// here (+4.. the results of its calls, +0x100 its call counter).
    const OC693_CTRL: u32 = DATA + 0x20_0000;
    const OC693_MAGIC: u32 = 0x0693_a001;
    static OC693_PREV_LOAD_MODULE: AtomicUsize = AtomicUsize::new(0);

    /// `sceSifLoadModule` with a scripted result per call (it needs a real
    /// IOP, and the real call fails with no RPC packets): negative or not.
    fn oc693_fake_load_module(c: &mut Cpu) {
        if c.bus.read32(OC693_CTRL) != OC693_MAGIC {
            return fakes::fall_back(c, &OC693_PREV_LOAD_MODULE, ee::s15::f_0042ba68__generated);
        }
        let ra = c.g[RA][0] as u32;
        let k = c.bus.read32(OC693_CTRL + 0x100);
        c.bus.write32(OC693_CTRL + 0x100, k + 1);
        fake(c, 0x0042_ba68, ra);
        if c.pc == ra {
            let v = c.bus.read32(OC693_CTRL + 4 + 4 * (k & 7));
            c.g[V0][0] = sx(v);
        }
    }

    /// `SND_IopInit`: every branch of the setup (sample rate, encoder
    /// buffers and their layouts, first start with the module loads, the
    /// mixer scan, the encoder size, the frame period, the init request)
    /// with the driver lock, allocator, IOP heap and thread calls running
    /// for real over an empty driver heap, the syscalls and the encoder
    /// callbacks logged by fakes (some unwinding), and `sceSifLoadModule`
    /// scripted. The first start always ends at the failing RPC bind (the
    /// packet pool is empty) or an earlier module failure.
    #[test]
    fn snd_iop_init_matches() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0042BA68),
            oc693_fake_load_module,
            &OC693_PREV_LOAD_MODULE,
        );
        covered(
            ee::s13::f_003c1b80__generated,
            snd_iop_init,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003c_1b80, 8);
                c.pc = SND_IOP_INIT_ENTRY;
                c.g[SP][0] = sx(TEST_STACK_TOP);
                c.bus.write32(OC693_CTRL, OC693_MAGIC);
                // sndgs: voices, sample rate, sync period, load flag, formats.
                c.bus.write16(SNDGS + 0x28, (1 + r.next_u32() % 8) as u16);
                let hz = [24000u32, 36000, 48000][(r.next_u32() % 3) as usize];
                c.bus.write16(SNDGS + 0x2a, hz as u16);
                c.bus.write16(SNDGS + 0x2c, r.next_u32() as u16);
                c.bus.write16(SNDGS + 0x2e, r.next_u32() as u16);
                c.bus.write8(SNDGS + 0x2f, r.next_u32() as u8);
                let sync = if r.next_u32() % 2 == 0 {
                    0xbf80_0000
                } else {
                    0x3f80_0000
                };
                c.bus.write32(SNDGS + 0x34, sync);
                c.bus.write8(SNDGS + 0x3f, (r.next_u32() % 2) as u8);
                c.bus.write8(SNDGS + 0x41, r.next_u32() as u8);
                c.bus.write8(SNDGS + 0x43, r.next_u32() as u8);
                let format = [4u8, 5, 3][(r.next_u32() % 3) as usize];
                c.bus.write8(SNDGS + 0x47, format);
                c.bus.write8(SNDGS + 0x48, (r.next_u32() % 4) as u8);
                let encoder = r.next_u32() % 3 != 0;
                c.bus.write8(SNDGS + 0x49, if encoder { 2 } else { 1 });
                c.bus.write8(SNDGS + 0x4a, r.next_u32() as u8);
                // Format entries: some with the 0x4 flag.
                let count = if r.next_u32() % 4 == 0 {
                    1 + r.next_u32() % 3
                } else {
                    0
                };
                c.bus.write8(SNDGS + 0x4c, count as u8);
                for i in 0..4 {
                    c.bus.write16(SNDGS + 0x4e + 2 * i, r.next_u32() as u16);
                }
                // The encoder state: a preset size (+0x64) with size1 below or
                // at/above +0x68.
                let size0 = r.next_u32() % 0x400;
                let size1 = r.next_u32() % 0x200;
                let sp_in = TEST_STACK_TOP - 0x100;
                c.bus.write32(sp_in + 0x90, size0);
                c.bus.write32(sp_in + 0x94, size1);
                let preset = r.next_u32() % 2;
                c.bus
                    .write32(SNDGS + 0x64, preset * (1 + r.next_u32() % 0x100));
                let limit = match r.next_u32() % 3 {
                    0 => size1 + 1 + r.next_u32() % 0x100,
                    1 => size1,
                    _ => r.next_u32() % 0x200,
                };
                c.bus.write32(SNDGS + 0x68, limit);
                c.bus.write32(0x0051_5d24, r.next_u32() % 0x100);
                c.bus.write32(ENCODER_SIZE_CALLBACK, FAKE);
                c.bus.write32(ENCODER_INIT_CALLBACK, FAKE + 0x10);
                // Module path (a short string) and a half-word at the GS mode.
                c.bus.write32(SNDGS + 0x6c, DATA + 0x1000);
                c.bus.write8(DATA + 0x1000, b'h');
                c.bus.write8(DATA + 0x1001, 0);
                let mode = if r.next_u32() % 2 == 0 { 2 } else { 1 };
                c.bus.write16(GS_GPARAM + 2, mode);
                // First start (about a third of the seeds) or started.
                let first = r.next_u32() % 3 == 0;
                c.bus.write32(SND_STARTED, if first { 0 } else { 1 });
                c.bus.write32(SIF_INIT_RPC_DONE, 1);
                c.bus.write32(SIF_RPC_POOL_COUNT, 0);
                // `sceSifLoadModule` results, negative or not.
                for k in 0..8 {
                    let v = if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        0xffff_ffff
                    };
                    c.bus.write32(OC693_CTRL + 4 + 4 * k, v);
                }
                fill(c, &mut r, DATA + 0x2000, 0x40);
            },
        );
    }
}
