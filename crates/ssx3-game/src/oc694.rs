//! Staging module oc694 (wave wave237): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc694`.

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
///
/// Not ported: `SND_SifDmaQueue` (0x3c4668) is a resumable dispatcher body
/// (`match pc` with back-edge event checks); the orchestrator converts it with
/// `tools/port_dispatcher.py`.
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003c_1980, snd_vblank_handler, "SND_VBlankHandler"),
    (0x003c_19a8, snd_ee_thread, "SND_EeThread"),
    (0x003c_3300, snd_mutex_init, "SND_MutexInit"),
    (0x003c_35f8, snd_apply_group_effect, "SND_ApplyGroupEffect"),
    (
        0x003c_3ea8,
        snd_relocate_patch_callback,
        "SND_RelocatePatchCallback",
    ),
    (0x003c_42d8, snd_iop_upload, "SND_IopUpload"),
    (0x003c_42f8, snd_iop_upload_wait, "SND_IopUploadWait"),
    (0x003c_43c0, snd_sif_set_dma, "SND_SifSetDma"),
    (0x003c_45e8, snd_sif_dma_kick, "SND_SifDmaKick"),
];

/// Sound library state (`0x50a8e8`, `0x510000 - 0x5718`).
#[cfg(test)]
const OC694_STATE: u32 = 0x0050_a8e8;
/// Sound library state block 2 (`0x50ad00`, `0x510000 - 0x5300`): SIF queue,
/// thread flags and the mixer buffers.
#[cfg(test)]
const OC694_BASE: u32 = 0x0050_ad00;

/// Callees of the sound library (all in shard s13).
mod oc694_callee {
    use crate::call::Callee;
    use ssx3_ee as ee;

    pub(super) const LOCK: Callee = Callee {
        addr: 0x003b_58a0,
        f: ee::s13::f_003b58a0,
    };
    pub(super) const UNLOCK: Callee = Callee {
        addr: 0x003b_58d8,
        f: ee::s13::f_003b58d8,
    };
    pub(super) const SET_SPU_REVERB_MODE: Callee = Callee {
        addr: 0x003c_3500,
        f: ee::s13::f_003c3500,
    };
    pub(super) const IOP_SET_EFFECT: Callee = Callee {
        addr: 0x003c_3450,
        f: ee::s13::f_003c3450,
    };
    pub(super) const SIF_DMA_UPLOAD: Callee = Callee {
        addr: 0x003c_4788,
        f: ee::s13::f_003c4788,
    };
    pub(super) const SIF_DMA_POLL: Callee = Callee {
        addr: 0x003c_4898,
        f: ee::s13::f_003c4898,
    };
    pub(super) const SIF_SET_DMA: Callee = Callee {
        addr: 0x003c_43c0,
        f: ee::s13::f_003c43c0__generated,
    };
    pub(super) const SIF_DMA_START_NEXT: Callee = Callee {
        addr: 0x003c_4450,
        f: ee::s13::f_003c4450,
    };
    pub(super) const SIF_DMA_KICK: Callee = Callee {
        addr: 0x003c_45e8,
        f: ee::s13::f_003c45e8__generated,
    };
    pub(super) const SIF_DMA_QUEUE: Callee = Callee {
        addr: 0x003c_4668,
        f: ee::s13::f_003c4668,
    };
    pub(super) const SERVICE: Callee = Callee {
        addr: 0x003b_5540,
        f: ee::s13::f_003b5540,
    };
    pub(super) const RUN_CHANGE_CALLBACKS: Callee = Callee {
        addr: 0x003b_54e0,
        f: ee::s13::f_003b54e0,
    };
    pub(super) const MEM_COPY: Callee = Callee {
        addr: 0x003c_51e8,
        f: ee::s13::f_003c51e8,
    };
    pub(super) const RELEASE_FINISHED_VOICES: Callee = Callee {
        addr: 0x003c_1298,
        f: ee::s13::f_003c1298,
    };
    pub(super) const SAVE_VU0_CONTEXT: Callee = Callee {
        addr: 0x003c_0858,
        f: ee::s13::f_003c0858,
    };
    pub(super) const SAVE_SPR_DMA: Callee = Callee {
        addr: 0x003c_09b8,
        f: ee::s13::f_003c09b8,
    };
    pub(super) const RESTORE_SPR_DMA: Callee = Callee {
        addr: 0x003c_0a60,
        f: ee::s13::f_003c0a60,
    };
    pub(super) const RESTORE_VU0_CONTEXT: Callee = Callee {
        addr: 0x003c_08f8,
        f: ee::s13::f_003c08f8,
    };
    pub(super) const RUN_STREAM_CALLBACKS: Callee = Callee {
        addr: 0x003c_7080,
        f: ee::s13::f_003c7080,
    };
}

/// `int SND_VBlankHandler(void)` (0x3c1980): the sound library's VBlank
/// interrupt handler. Counts one more VBlank (`state + 0x1c0`), re-enables
/// interrupts and returns 0.
pub fn snd_vblank_handler(c: &mut Cpu) {
    const VBLANKS: u32 = 0x1c0;

    c.cycles += 9;
    c.g[V1][0] = 0x510000;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5718));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(VBLANKS));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
    c.sw((c.g[V1][0] as u32).wrapping_add(VBLANKS), c.g[V0][0]);
    c.ei();
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = 0u64;
    c.pc = t;
}

/// `void SND_EeThread(void)` (0x3c19a8): the EE-side sound mixer thread. It
/// never returns: each round waits on the thread semaphore (`state2 + 0x1aa4`),
/// and if the mix flag (`+0x1aad`) is set runs the pre-mix hook and the
/// service hook, services the sound queue (`SND_Service(0x5e)`), runs the
/// change callbacks, snapshots the voice block, releases finished voices,
/// saves the VU0 context (and the scratchpad DMA state when `+0x1aae` is set),
/// mixes one block (`SND_EeMix`), hands the block to the output hook or copies
/// it, restores the contexts, runs the stream callbacks, swaps the two
/// buffer pointers, flushes the cache, and finally calls the post-mix hooks
/// with interrupts off (`di` loop) and the interrupt-disable counter
/// restored.
pub fn snd_ee_thread(c: &mut Cpu) {
    use oc694_callee::*;

    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = 0x510000;
        frame.save(c, S4, 0x30);
        frame.save(c, S1, 0x60);
        c.g[S4][0] = 0x510000;
        frame.save(c, S3, 0x40);
        c.g[S1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x5300));
        frame.save(c, S5, 0x20);
        c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5718));
        frame.save(c, S6, 0x10);
        c.g[S5][0] = 0x510000;
        frame.save(c, S0, 0x70);
        c.g[S6][0] = 0x510000;
        frame.save(c, S2, 0x50);
        frame.save_ra(c, 0);
        'round: loop {
            // Wait for the mixer semaphore.
            c.cycles += 5;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1aa4));
            call_ptr(c, 0x00423de0, 0x003c_19f0)?;
            c.g[V1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x1aad));
            let idle = c.g[V1][0] == 0u64;
            c.g[V0][0] = 0x450000;
            if idle {
                if c.cycles >= c.next_event {
                    c.pc = 0x003c19e8;
                    c.event();
                }
                continue 'round;
            }
            // Pre-mix hook, then the optional second hook.
            c.cycles += 8;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x3b94));
            let hook = c.g[V1][0] as u32;
            call_ptr(c, hook, 0x003c_1a08)?;
            c.g[V0][0] = 0x510000;
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5718));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x328));
            if c.g[V0][0] != 0u64 {
                c.cycles += 2;
                let hook = c.g[V0][0] as u32;
                call_ptr(c, hook, 0x003c_1a24)?;
            }
            // Service the queue; count a missed frame when the service flag
            // (`+0x1aa8`) is negative.
            c.cycles += 5;
            c.g[A0][0] = 0x5e;
            SERVICE.call(c, 0x003c_1a2c)?;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1aa8));
            if (c.g[V0][0] as i64) < 0 {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c0), c.g[V0][0]);
            }
            c.cycles += 13;
            c.g[S2][0] = 0x510000;
            RUN_CHANGE_CALLBACKS.call(c, 0x003c_1a4c)?;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x800));
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xa40));
            c.g[A2][0] = 0x240;
            MEM_COPY.call(c, 0x003c_1a5c)?;
            RELEASE_FINISHED_VOICES.call(c, 0x003c_1a64)?;
            c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x36f0));
            SAVE_VU0_CONTEXT.call(c, 0x003c_1a6c)?;
            c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x1aae));
            if c.g[V0][0] != 0u64 {
                c.cycles += 3;
                SAVE_SPR_DMA.call(c, 0x003c_1a80)?;
            }
            c.g[A1][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x1ab0));
            // Mix one block.
            c.cycles += 6;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1abc));
            call_ptr(c, 0x003c8968, 0x003c_1a8c)?;
            c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x49));
            c.g[V0][0] &= 0x2;
            let no_output_hook = c.g[V0][0] == 0u64;
            c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_sub(0x37b8));
            if !no_output_hook {
                // Output hook(block, size, buffer).
                c.cycles += 6;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1adc));
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1af4));
                let hook = c.g[V0][0] as u32;
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1afc));
                call_ptr(c, hook, 0x003c_1aac)?;
                c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x5300));
            } else {
                c.cycles += 4;
                c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x47));
                c.g[V0][0] = 0x1;
                let copy = c.g[V1][0] == c.g[V0][0];
                c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x5300));
                if copy {
                    // Copy the mixed block (`size * 2` bytes) to the output.
                    c.cycles += 6;
                    c.g[A2][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x1ab0));
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1ac0));
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1abc));
                    c.g[A2][0] = sx((c.g[A2][0] as u32) << 1);
                    MEM_COPY.call(c, 0x003c_1ad8)?;
                    c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x5300));
                }
            }
            // Restore the scratchpad DMA state and the VU0 context.
            c.cycles += 3;
            c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1aae));
            if c.g[V0][0] != 0u64 {
                c.cycles += 2;
                RESTORE_SPR_DMA.call(c, 0x003c_1af0)?;
            }
            c.cycles += 14;
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x36f0));
            RESTORE_VU0_CONTEXT.call(c, 0x003c_1af8)?;
            c.g[S2][0] = 0x450000;
            RUN_STREAM_CALLBACKS.call(c, 0x003c_1b00)?;
            // Swap the two buffer pointers (`+0xcb4` and `+0xcb8`) and flush.
            c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x330));
            c.g[A0][0] = 0u64;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcb4));
            c.sw(c.g[V0][0] as u32, c.g[A1][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcb8));
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x330));
            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
            call_ptr(c, 0x00424020, 0x003c_1b20)?;
            c.g[A0][0] = 0x510000;
            c.g[V1][0] = 0x10000;
            // Disable interrupts (until Status.EIE reads back clear).
            loop {
                c.cycles += 7;
                c.di();
                c.mfc0(2, 12);
                c.g[V0][0] &= c.g[V1][0];
                if c.g[V0][0] == 0u64 {
                    break;
                }
                if c.cycles >= c.next_event {
                    c.pc = 0x003c1b28;
                    c.event();
                }
            }
            c.cycles += 7;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3800));
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x00424160, 0x003c_1b50)?;
            c.ei();
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x32c));
            if c.g[V0][0] != 0u64 {
                c.cycles += 3;
                let hook = c.g[V0][0] as u32;
                call_ptr(c, hook, 0x003c_1b68)?;
            }
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x3b90));
            c.cycles += 4;
            let hook = c.g[V0][0] as u32;
            call_ptr(c, hook, 0x003c_1b74)?;
            if c.cycles >= c.next_event {
                c.pc = 0x003c19e8;
                c.event();
            }
        }
    })
}

/// `void SND_MutexInit(void)` (0x3c3300): the sound library's lock. Clears the
/// owner (`state2 + 0x1b14 = -1`, `+0x1b18 = 0`), fills the `SemaParam`
/// (`max_count = 0x14`, `init_count = 1`) at `0x50acE0` and stores the
/// `CreateSema` result as the semaphore id (`+0x1b10`).
pub fn snd_mutex_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 0xffffffffffffffff;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = 0x510000;
        c.g[S0][0] = 0x510000;
        frame.save_ra(c, 0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x5300));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5320));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1b14), c.g[V1][0]);
        c.g[A0][0] = 0x14;
        c.g[V1][0] = 0x1;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A0][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A0][0] = c.g[V0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1b18), 0u64);
        call_ptr(c, 0x00423da0, 0x003c_3344)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1b10), c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SND_ApplyGroupEffect(int effect)` (0x3c35f8): under the sound lock,
/// apply the effect a sound group asks for: 8 sets the SPU reverb mode from
/// the saved setting (`0x50ab08`), 0x100 sends the saved IOP effect
/// (`0x50aaf4`) to the IOP, 4 (with the effect state 1 and no IOP: `+0x1f8 ==
/// 1`, `+0x42 == 0`) rebuilds the EE effect from `state + 0x1fc`, any other
/// state frees the EE effect (`SND_EeFxDisable`), then returns the unlock.
pub fn snd_apply_group_effect(c: &mut Cpu) {
    use oc694_callee::*;

    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        LOCK.call(c, 0x003c_360c)?;
        c.g[V0][0] = 0x8;
        'apply: {
            if c.g[S0][0] == c.g[V0][0] {
                c.cycles += 5;
                c.g[V0][0] = 0x510000;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_sub(0x54f8));
                SET_SPU_REVERB_MODE.call(c, 0x003c_3624)?;
                break 'apply;
            }
            c.g[V0][0] = 0x100;
            c.cycles += 2;
            if c.g[S0][0] == c.g[V0][0] {
                c.cycles += 5;
                c.g[V0][0] = 0x510000;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_sub(0x550c));
                IOP_SET_EFFECT.call(c, 0x003c_3640)?;
                break 'apply;
            }
            c.g[V0][0] = 0x4;
            c.cycles += 2;
            let other = c.g[S0][0] != c.g[V0][0];
            c.g[V0][0] = 0x510000;
            if other {
                break 'apply;
            }
            'rebuild: {
                'disable: {
                    c.cycles += 5;
                    c.g[A0][0] = 0x1;
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5718));
                    c.g[V1][0] = c.lh((c.g[A2][0] as u32).wrapping_add(0x1f8));
                    let not_one = c.g[V1][0] != c.g[A0][0];
                    c.g[V0][0] = c.lbu((c.g[A2][0] as u32).wrapping_add(0x42));
                    if !not_one {
                        c.cycles += 2;
                        if c.g[V0][0] == 0u64 {
                            c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1fc));
                            break 'disable;
                        }
                    }
                    // Disable the EE effect (clearing the active flag first
                    // when the IOP effect is off).
                    c.cycles += 2;
                    let iop = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x510000;
                    if !iop {
                        c.cycles += 1;
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x5b50), 0u64);
                    }
                    c.cycles += 4;
                    call_ptr(c, 0x003cc848, 0x003c_3680)?;
                    break 'apply;
                }
                // Rebuild the EE effect from `state + 0x1fc`.
                c.cycles += 2;
                c.g[A0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x2a));
                call_ptr(c, 0x003cc630, 0x003c_3690)?;
                break 'rebuild;
            }
        }
        c.cycles += 6;
        UNLOCK.call(c, 0x003c_3698)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SND_RelocatePatchCallback(void* header, int new_base, int* old_base)`
/// (0x3c3ea8): relocation callback for patch loading. Remembers the first
/// base address seen (`*old_base = header[0x68]` when it is still 0) and
/// rewrites `header[0x64] = new_base + header[0x68] - *old_base`. Returns 0.
pub fn snd_relocate_patch_callback(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
    c.g[A3][0] = c.g[A0][0];
    let seen = c.g[V0][0] != 0u64;
    c.g[T0][0] = c.g[A1][0];
    if !seen {
        c.cycles += 2;
        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x68));
        c.sw(c.g[A2][0] as u32, c.g[V0][0]);
    }
    c.cycles += 8;
    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x68));
    c.g[V0][0] = 0u64;
    c.g[A0][0] = c.lw(c.g[A2][0] as u32);
    c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x64));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
    c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    let t = c.g[RA][0] as u32;
    c.sw(c.g[A1][0] as u32, c.g[V1][0]);
    c.pc = t;
}

/// `int SND_IopUpload(a0, a1, a2, a3)` (0x3c42d8): upload to the IOP, a
/// `SND_SifDmaUpload` with queue id 10.
pub fn snd_iop_upload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[T0][0] = 0xa;
        oc694_callee::SIF_DMA_UPLOAD.call(c, 0x003c_42e8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SND_IopUploadWait(void)` (0x3c42f8): wait for the queued uploads
/// (`SND_SifDmaPoll`).
pub fn snd_iop_upload_wait(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(oc694_callee::SIF_DMA_POLL),
        0x003c_4308,
    );
}

/// `int SND_SifSetDma(void* src, void* dst, int size, int nonblocking)`
/// (0x3c43c0): start one SIF DMA transfer `{src, dst, size, attr 0}` built
/// on the stack. With `nonblocking != 0` it disables interrupts first
/// (`DIntr`), spins on `sceSifDmaStat` of the previous id (`0x44e178`) until
/// it is negative, and starts the transfer; otherwise it flushes the cache,
/// spins the same way and starts the transfer. The new DMA id replaces
/// `0x44e178`.
pub fn snd_sif_set_dma(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        let at = frame.at(c, 0x10);
        c.sd(at, c.g[RA][0]);
        let at = frame.at(c, 0);
        c.sw(at, c.g[A0][0]);
        let at = frame.at(c, 8);
        c.sw(at, c.g[A2][0]);
        let at = frame.at(c, 4);
        c.sw(at, c.g[A1][0]);
        let nonblocking = c.g[A3][0] != 0u64;
        let at = frame.at(c, 0xc);
        c.sw(at, 0u64);
        'done: {
            if nonblocking {
                c.cycles += 4;
                c.g[A0][0] = 0u64;
                call_ptr(c, 0x00424050, 0x003c_43e8)?;
                c.g[S0][0] = 0x450000;
                loop {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x1e88));
                    call_ptr(c, 0x00424150, 0x003c_43f8)?;
                    let busy = (c.g[V0][0] as i64) >= 0;
                    c.g[A0][0] = c.g[SP][0];
                    if busy {
                        if c.cycles >= c.next_event {
                            c.pc = 0x003c43f0;
                            c.event();
                        }
                        continue;
                    }
                    c.cycles += 4;
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x00424170, 0x003c_4408)?;
                    c.sw((c.g[S0][0] as u32).wrapping_sub(0x1e88), c.g[V0][0]);
                    break 'done;
                }
            }
            c.cycles += 4;
            c.g[A0][0] = 0u64;
            call_ptr(c, 0x00424020, 0x003c_4418)?;
            c.g[S0][0] = 0x450000;
            loop {
                c.cycles += 4;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x1e88));
                call_ptr(c, 0x00424140, 0x003c_4428)?;
                let busy = (c.g[V0][0] as i64) >= 0;
                c.g[A0][0] = c.g[SP][0];
                if busy {
                    if c.cycles >= c.next_event {
                        c.pc = 0x003c4420;
                        c.event();
                    }
                    continue;
                }
                c.cycles += 3;
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x00424160, 0x003c_4438)?;
                c.sw((c.g[S0][0] as u32).wrapping_sub(0x1e88), c.g[V0][0]);
                break 'done;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void SND_SifDmaKick(int nested)` (0x3c45e8): start the next queued DMA if
/// none is running. With `nested == 0` it first disables interrupts (`di`
/// until Status.EIE reads back clear); then, when the busy flag (`+0xcc2`) is
/// clear and the queue is not empty (`+0xcc0 > 0`), calls
/// `SND_SifDmaStartNext(nested)`; with `nested == 0` interrupts are enabled
/// again at the end.
pub fn snd_sif_dma_kick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let nested = c.g[S0][0] != 0u64;
        frame.save_ra(c, 0);
        if !nested {
            c.cycles += 1;
            c.g[V1][0] = 0x10000;
            loop {
                c.cycles += 7;
                c.di();
                c.mfc0(2, 12);
                c.g[V0][0] &= c.g[V1][0];
                if c.g[V0][0] == 0u64 {
                    break;
                }
                if c.cycles >= c.next_event {
                    c.pc = 0x003c4600;
                    c.event();
                }
            }
        }
        'idle: {
            c.cycles += 5;
            c.g[V0][0] = 0x510000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5300));
            c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0xcc2));
            if c.g[V1][0] != 0u64 {
                break 'idle;
            }
            c.cycles += 4;
            c.g[V0][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0xcc0));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
            if (c.g[V0][0] as i64) <= 0 {
                break 'idle;
            }
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            oc694_callee::SIF_DMA_START_NEXT.call(c, 0x003c_4648)?;
        }
        c.cycles += 2;
        let nested = c.g[S0][0] != 0u64;
        frame.restore(c, S0, 0x10);
        if !nested {
            c.cycles += 2;
            c.ei();
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule, Events};
    use crate::difftest::fakes::{self, covered, machine, FAKE, LOG_COUNT, LOG_UNWIND_AT};
    use crate::difftest::{check, fill, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;

    /// Gated slot fakes (the suite runs tests in parallel): a fake runs only
    /// while its magic is at this address of the machine's RAM, else the body
    /// the slot held. `+4`: how many syscalls return a positive value
    /// before the status polls turn negative; `+8`: non-zero makes every
    /// syscall drain the SIF queue count (a completed DMA).
    const OC694_CTRL: u32 = DATA + 0x21_6940;
    const OC694_MAGIC: u32 = 0x0694_ca11;

    /// A gated logging fake for the slot of the function at `$addr`.
    macro_rules! oc694_fake {
        ($name:ident, $addr:expr, $gen:path) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC694_CTRL) != OC694_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $addr, ra);
            }
        };
    }

    oc694_fake!(oc694_fake_lock, 0x003b_58a0, ee::s13::f_003b58a0__generated);
    oc694_fake!(
        oc694_fake_unlock,
        0x003b_58d8,
        ee::s13::f_003b58d8__generated
    );
    oc694_fake!(
        oc694_fake_reverb,
        0x003c_3500,
        ee::s13::f_003c3500__generated
    );
    oc694_fake!(
        oc694_fake_service,
        0x003b_5540,
        ee::s13::f_003b5540__generated
    );
    oc694_fake!(
        oc694_fake_change_callbacks,
        0x003b_54e0,
        ee::s13::f_003b54e0__generated
    );
    oc694_fake!(
        oc694_fake_mem_copy,
        0x003c_51e8,
        ee::s13::f_003c51e8__generated
    );
    oc694_fake!(
        oc694_fake_release_voices,
        0x003c_1298,
        ee::s13::f_003c1298__generated
    );
    oc694_fake!(
        oc694_fake_save_vu0,
        0x003c_0858,
        ee::s13::f_003c0858__generated
    );
    oc694_fake!(
        oc694_fake_restore_vu0,
        0x003c_08f8,
        ee::s13::f_003c08f8__generated
    );
    oc694_fake!(
        oc694_fake_stream_callbacks,
        0x003c_7080,
        ee::s13::f_003c7080__generated
    );
    oc694_fake!(
        oc694_fake_ee_mix,
        0x003c_8968,
        ee::s14::f_003c8968__generated
    );

    /// Install the gated fakes (once per process).
    fn oc694_install() {
        let hook = |slot: *mut fn(&mut Cpu), fake: fn(&mut Cpu)| {
            fakes::hook(slot, fake, fakes::prev_cell(fake))
        };
        hook(addr_of_mut!(ee::s13::OVERRIDE_003B58A0), oc694_fake_lock);
        hook(addr_of_mut!(ee::s13::OVERRIDE_003B58D8), oc694_fake_unlock);
        hook(addr_of_mut!(ee::s13::OVERRIDE_003C3500), oc694_fake_reverb);
        hook(addr_of_mut!(ee::s13::OVERRIDE_003B5540), oc694_fake_service);
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B54E0),
            oc694_fake_change_callbacks,
        );
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_003C51E8),
            oc694_fake_mem_copy,
        );
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_003C1298),
            oc694_fake_release_voices,
        );
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_003C0858),
            oc694_fake_save_vu0,
        );
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_003C08F8),
            oc694_fake_restore_vu0,
        );
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_003C7080),
            oc694_fake_stream_callbacks,
        );
        hook(addr_of_mut!(ee::s14::OVERRIDE_003C8968), oc694_fake_ee_mix);
    }

    /// The syscall hook of these tests: the logging fake, then (after the
    /// first `+4` calls) a negative result, which ends the `sceSifDmaStat`
    /// style polls of the SIF code; optionally the DMA queue drains by two
    /// records per call.
    fn oc694_syscall(c: &mut Cpu, at: u32) {
        fakes::fake_syscall(c, at);
        if c.pc >> 16 == 0xdead {
            return;
        }
        let n = c.bus.read32(LOG_COUNT);
        if n > c.bus.read32(OC694_CTRL + 4) {
            c.g[V0][0] = u64::MAX;
        }
        if c.bus.read32(OC694_CTRL + 8) != 0 {
            let count = c.bus.read8(OC694_BASE + 0xcc0) as i8;
            if count > 0 {
                c.bus
                    .write8(OC694_BASE + 0xcc0, count.saturating_sub(2).max(0) as u8);
            }
        }
    }

    /// [`machine`] with the syscall hook above.
    fn oc694_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        c.hooks.syscall = oc694_syscall;
        c.bus.write32(OC694_CTRL + 4, r.next_u32() % 4);
        c.bus.write32(OC694_CTRL + 8, 0);
        r
    }

    /// The SIF DMA queue: `count` queued records (signed byte at `+0xcc0`),
    /// 28 records of 16 bytes at `+0xcc8` (about a third free; the others with
    /// small first words so priorities and first words tie), the destination
    /// at `+0xe8c`.
    fn oc694_queue(c: &mut Cpu, r: &mut Rng, count: i8) {
        c.bus.write8(OC694_BASE + 0xcc0, count as u8);
        c.bus.write8(OC694_BASE + 0xcc1, r.next_u32() as u8);
        c.bus.write8(OC694_BASE + 0xcc2, 0);
        for i in 0..28u32 {
            let rec = OC694_BASE + 0xcc8 + 0x10 * i;
            let used = r.next_u32() % 3 != 0;
            let first = if used {
                0x100 * (1 + r.next_u32() % 6)
            } else {
                0
            };
            c.bus.write32(rec, first);
            c.bus.write32(rec + 4, DATA + 0x4000 + 0x100 * i);
            c.bus.write32(rec + 8, r.next_u32());
            c.bus.write16(rec + 0xc, (r.next_u32() % 0x1000) as u16);
            c.bus.write8(rec + 0xe, (r.next_u32() % 3) as u8);
        }
        c.bus.write32(OC694_BASE + 0xe8c, r.next_u32());
    }

    /// `SND_VBlankHandler`: counts the VBlank (also wrapping), enables
    /// interrupts.
    #[test]
    fn snd_vblank_handler_matches() {
        check(
            ee::s13::f_003c1980__generated,
            snd_vblank_handler,
            60,
            &|c, seed| {
                let mut r = Rng(0x003c_1980 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x003c_1980;
                c.cop0[12] = r.next_u32();
                let count = if seed % 4 == 0 {
                    u32::MAX
                } else {
                    r.next_u32()
                };
                c.bus.write32(OC694_STATE + 0x1c0, count);
            },
        );
    }

    /// `SND_MutexInit`: the `CreateSema` syscall is a fake (unwinding for some
    /// seeds), the old lock state is random.
    #[test]
    fn snd_mutex_init_matches() {
        covered(
            ee::s13::f_003c3300__generated,
            snd_mutex_init,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003c_3300, 1);
                c.pc = 0x003c_3300;
                fill(c, &mut r, OC694_BASE + 0x1b10, 4);
                fill(c, &mut r, 0x0050_acd0, 0x10);
            },
        );
    }

    /// `SND_ApplyGroupEffect`: effects 8 (reverb), 4 (EE effect: disabled, with
    /// the active flag cleared or not) and others; the sound lock and the reverb
    /// call are logging fakes. The rebuild branch (`+0x1f8 == 1` with the IOP
    /// effect off) calls the real `SND_EeFxBuild` machinery on test data and
    /// is not reached here.
    #[test]
    fn snd_apply_group_effect_matches() {
        oc694_install();
        covered(
            ee::s13::f_003c35f8__generated,
            snd_apply_group_effect,
            200,
            &|c, seed| {
                let mut r = oc694_machine(c, seed, 0x003c_35f8, 3);
                c.bus.write32(OC694_CTRL, OC694_MAGIC);
                c.pc = 0x003c_35f8;
                c.g[A0][0] = [8u64, 4, 4, 0x20, 4, 0, 8][(seed % 7) as usize];
                let mode = [0u16, 2, 1, 0, 5][r.next_u32() as usize % 5];
                // Mode 1 with the IOP effect off would take the rebuild branch.
                let iop = if mode == 1 {
                    1
                } else {
                    (r.next_u32() % 2) as u8
                };
                c.bus.write16(OC694_STATE + 0x1f8, mode);
                c.bus.write8(OC694_STATE + 0x42, iop);
                c.bus.write32(OC694_STATE + 0x1fc, r.next_u32());
                c.bus.write16(OC694_STATE + 0x2a, r.next_u32() as u16);
                c.bus.write16(0x0050_ab08, r.next_u32() as u16);
                c.bus.write16(0x0050_aaf4, r.next_u32() as u16);
                c.bus.write32(0x0051_5b50, r.next_u32());
            },
        );
    }

    /// `SND_ApplyGroupEffect(0x100)`: sends the saved IOP effect through the
    /// real `SND_IopSetEffect` (and `SND_IopCallRpc`), whose paths depend on
    /// the SIF RPC state; its own test so it can be dropped alone if the
    /// real callees do not stay bounded on test data.
    #[test]
    fn snd_apply_group_effect_iop_effect_matches() {
        oc694_install();
        check(
            ee::s13::f_003c35f8__generated,
            snd_apply_group_effect,
            60,
            &|c, seed| {
                let mut r = oc694_machine(c, seed, 0x003c_35f9, 8);
                c.bus.write32(OC694_CTRL, OC694_MAGIC);
                c.pc = 0x003c_35f8;
                c.g[A0][0] = 0x100;
                c.bus.write8(OC694_STATE + 0x42, (seed % 2) as u8);
                c.bus
                    .write16(0x0050_aaf4, [1u16, 0, 5][(seed % 3) as usize]);
                c.bus.write32(0x0044_dd74, r.next_u32() & 0xf_fff0);
            },
        );
    }

    /// `SND_RelocatePatchCallback`: the base already known or not.
    #[test]
    fn snd_relocate_patch_callback_matches() {
        check(
            ee::s13::f_003c3ea8__generated,
            snd_relocate_patch_callback,
            100,
            &|c, seed| {
                let mut r = Rng(0x003c_3ea8 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x003c_3ea8;
                let (header, base) = (DATA + 0x100, DATA + 0x200);
                fill(c, &mut r, header, 0x40);
                c.bus
                    .write32(base, if seed % 2 == 0 { 0 } else { r.next_u32() });
                c.g[A0][0] = header as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = base as u64;
            },
        );
    }

    /// Queue state and arguments of an upload: size 0..0x2800 (also
    /// negative), an empty or half-full queue (kept below the kick threshold
    /// for some seeds, above it for others, with the drain hook).
    fn oc694_upload_args(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let high = seed % 5 == 4;
        let count = if high {
            24 + (r.next_u32() % 4) as i8
        } else {
            (r.next_u32() % 4) as i8
        };
        oc694_queue(c, r, count);
        // An empty queue (all records free), so the new record has a slot.
        for i in 0..28u32 {
            c.bus.write32(OC694_BASE + 0xcc8 + 0x10 * i, 0);
        }
        c.bus.write32(OC694_CTRL + 8, high as u32);
        c.bus.write32(OC694_BASE + 0xcbc, r.next_u32() % 3);
        c.bus.write32(OC694_STATE + 0x180, r.next_u32());
    }

    const OC694_UPLOAD_SIZES: [i32; 8] = [-5, 0, 0x10, 0xfff, 0x1000, 0x1001, 0x1800, 0x2800];

    /// `SND_IopUpload`: the real `SND_SifDmaUpload`/`SND_SifDmaQueue` run; the
    /// SIF syscalls are fakes.
    #[test]
    fn snd_iop_upload_matches() {
        check(
            ee::s13::f_003c42d8__generated,
            snd_iop_upload,
            120,
            &|c, seed| {
                let mut r = oc694_machine(c, seed, 0x003c_42d8, 6);
                c.pc = 0x003c_42d8;
                oc694_upload_args(c, &mut r, seed);
                c.g[A0][0] = [0x100u64, 5][(seed % 2) as usize];
                c.g[A1][0] = (DATA + 0x1000) as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = sx(OC694_UPLOAD_SIZES[(seed / 2 % 8) as usize] as u32);
            },
        );
    }

    /// `SND_IopUploadWait`: the real `SND_SifDmaPoll` over a random queue (its
    /// id looked up among the records, the pending DMA possibly restarted).
    #[test]
    fn snd_iop_upload_wait_matches() {
        check(
            ee::s13::f_003c42f8__generated,
            snd_iop_upload_wait,
            150,
            &|c, seed| {
                let mut r = oc694_machine(c, seed, 0x003c_42f8, 4);
                c.pc = 0x003c_42f8;
                let count = (r.next_u32() % 6) as i8 - 1;
                oc694_queue(c, &mut r, count);
                c.bus.write32(OC694_BASE + 0xcc4, r.next_u32() % 0x400);
                c.bus.write32(OC694_STATE + 0x180, r.next_u32() % 0x800);
                c.g[A0][0] =
                    [0x100u64, 0x200, 0x300, 0x400, r.next_u32() as u64][(seed % 5) as usize];
            },
        );
    }

    /// `SND_SifSetDma`: both arms (interrupt-disabling and flushing), the status
    /// polls spinning a few times; syscalls are fakes (unwinding for some
    /// seeds), events delivered in the polls.
    #[test]
    fn snd_sif_set_dma_matches() {
        check_events(
            ee::s13::f_003c43c0__generated,
            snd_sif_set_dma,
            300,
            &|c, seed| {
                let mut r = oc694_machine(c, seed, 0x003c_43c0, 3);
                c.pc = 0x003c_43c0;
                c.bus.write32(0x0044_e178, r.next_u32());
                let args = [DATA + 0x100, r.next_u32(), r.next_u32() & 0xffff];
                let flag = if seed % 2 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 3
                };
                let ev = schedule(c, &mut r, seed, 0x003c_43c0);
                for (i, v) in args.into_iter().enumerate() {
                    c.g[[A0, A1, A2][i]][0] = sx(v);
                }
                c.g[A3][0] = flag as u64;
                ev
            },
        );
    }

    /// `SND_SifDmaStartNext`: queues of 28 records (free, used, with equal
    /// priorities and first words) and counts -1..8, then the real
    /// `SND_SifSetDma` and the IOP command (both fakes at the syscall level).

    /// `SND_SifDmaKick`: nested or not, busy or idle, empty or non-empty
    /// queue; the real `SND_SifDmaStartNext` runs.
    #[test]
    fn snd_sif_dma_kick_matches() {
        check(
            ee::s13::f_003c45e8__generated,
            snd_sif_dma_kick,
            200,
            &|c, seed| {
                let mut r = oc694_machine(c, seed, 0x003c_45e8, 6);
                c.pc = 0x003c_45e8;
                c.cop0[12] = r.next_u32();
                let count = (r.next_u32() % 5) as i8 - 1;
                oc694_queue(c, &mut r, count);
                c.bus
                    .write8(OC694_BASE + 0xcc2, (r.next_u32() % 3 == 0) as u8);
                c.bus.write32(0x0044_e178, r.next_u32());
                c.bus.write32(OC694_STATE + 0x180, r.next_u32());
                c.g[A0][0] = (seed % 2) as u64 * (1 + r.next_u32() as u64 % 3);
            },
        );
    }

    /// `SND_SifDmaUpload`: sizes from negative to several chunks, the queue
    /// kicked while it holds 24 or more records (the drain hook completes
    /// them), the real `SND_SifDmaQueue` and `SND_SifDmaKick` run.

    /// `SND_EeThread`: never returns; every callee is a fake (the sound
    /// hooks through function pointers, the library calls through gated slot
    /// fakes, the syscalls) and one of the first calls unwinds. Event seeds
    /// idle (the mix flag clear) so the loop's back edge delivers the event;
    /// the others run whole rounds with the optional hooks and the scratchpad
    /// DMA branch set at random.
    #[test]
    fn snd_ee_thread_matches() {
        oc694_install();
        check_events(
            ee::s13::f_003c19a8__generated,
            snd_ee_thread,
            450,
            &|c, seed| {
                let mut r = oc694_machine(c, seed, 0x003c_19a8, 1);
                c.bus.write32(OC694_CTRL, OC694_MAGIC);
                c.pc = 0x003c_19a8;
                c.bus.write32(LOG_UNWIND_AT, r.next_u32() % 14);
                c.cop0[12] = r.next_u32();
                let idle = seed % 3 != 0 || seed % 7 == 0;
                let optional = |r: &mut Rng| if r.next_u32() % 2 == 0 { 0 } else { FAKE };
                c.bus.write32(OC694_BASE + 0x1aa4, r.next_u32());
                c.bus.write8(
                    OC694_BASE + 0x1aad,
                    if idle { 0 } else { 1 + r.next_u32() as u8 % 3 },
                );
                c.bus.write32(0x0044_c46c, FAKE);
                c.bus.write32(OC694_STATE + 0x328, optional(&mut r));
                c.bus.write32(OC694_BASE + 0x1aa8, r.next_u32());
                c.bus.write32(OC694_STATE + 0x1c0, r.next_u32());
                let spr = if seed % 6 == 0 {
                    1 + r.next_u32() as u16 % 0x100
                } else {
                    0
                };
                c.bus.write16(OC694_BASE + 0x1aae, spr);
                c.bus.write16(OC694_BASE + 0x1ab0, r.next_u32() as u16);
                for off in [0x1abc, 0x1ac0, 0x1adc, 0x1af4, 0x1afc] {
                    c.bus.write32(OC694_BASE + off, r.next_u32());
                }
                c.bus.write8(OC694_STATE + 0x49, r.next_u32() as u8);
                c.bus.write8(OC694_STATE + 0x47, (r.next_u32() % 3) as u8);
                c.bus.write32(0x0050_c848, FAKE);
                c.bus.write32(OC694_BASE + 0xcb4, DATA + 0x3000);
                c.bus.write32(OC694_BASE + 0xcb8, DATA + 0x3010);
                c.bus.write32(OC694_STATE + 0x330, r.next_u32());
                c.bus.write32(OC694_STATE + 0x32c, optional(&mut r));
                c.bus.write32(0x0044_c470, FAKE);
                schedule(c, &mut r, seed, 0x003c_19a8)
            },
        );
    }
}
