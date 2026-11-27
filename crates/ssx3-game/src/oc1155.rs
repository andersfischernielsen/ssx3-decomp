//! Staging module oc1155 (wave wave273): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1155`.

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
    (0x003c_7aa8, snd_ee_mixer_shutdown_stub, "SND_EeMixerShutdownStub"),
    (0x003c_8018, snd_ee_mixer_shutdown, "SND_EeMixerShutdown"),
    (0x003c_8a40, snd_ee_voice_get_position, "SND_EeVoiceGetPosition"),
    (0x003c_8e28, snd_ee_voice_add_dsp, "SND_EeVoiceAddDsp"),
    (0x003c_8ec0, snd_ee_voice_remove_dsp, "SND_EeVoiceRemoveDsp"),
    (0x003c_8fb0, snd_dsp_remove, "SND_DspRemove"),
    (0x003c_9010, snd_dsp_node_call_free, "SND_DspNode_CallFree"),
    (0x003c_9040, snd_dsp_node_destroy_tree, "SND_DspNode_DestroyTree"),
];

/// `SND_EeMixerShutdownStub` (0x3c7aa8): a leaf that returns.
pub(crate) const CALL_MIXER_SHUTDOWN_STUB: Callee = Callee {
    addr: 0x003c_7aa8,
    f: ee::s14::SND_EeMixerShutdownStub,
};

/// `SND_DspInsertSorted(list head, node)` (0x3c8f48).
pub(crate) const CALL_DSP_INSERT_SORTED: Callee = Callee {
    addr: 0x003c_8f48,
    f: ee::s14::SND_DspInsertSorted,
};

/// `SND_DspRemove(list head, node)` (0x3c8fb0).
pub(crate) const CALL_DSP_REMOVE: Callee = Callee {
    addr: 0x003c_8fb0,
    f: ee::s14::SND_DspRemove,
};

/// `SND_DspNode_DestroyTree(node)` (0x3c9040): the recursion goes through the
/// generated alias, which dispatches through the override slot.
pub(crate) const CALL_DSP_NODE_DESTROY_TREE: Callee = Callee {
    addr: 0x003c_9040,
    f: ee::s14::SND_DspNode_DestroyTree,
};

/// The word at `base + off`, as the compiled code addresses it (32-bit wrap).
#[inline]
fn reg_field(base: u64, off: u32) -> Addr {
    Addr((base as u32).wrapping_add(off))
}

/// `SND_EeMixerShutdownStub` (0x3c7aa8): nothing to do, return.
pub fn snd_ee_mixer_shutdown_stub(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void SND_EeMixerShutdown(void)` (0x3c8018): releases the mixer's sound
/// memory. Under the lock it frees the voice-state array (`smv`, cleared),
/// then each of the two temp buffers and the `outputchannels` mix buffers
/// (a flagged pointer is cleared without a free), unlocks, and stops the
/// mixer through the stub with the constant 2.0f.
pub fn snd_ee_mixer_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        call_ptr(c, 0x003b_58a0, 0x003c_8038)?;
        c.g[V0][0] = 0x51_0000;
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5b40));
        c.g[A0][0] = reg_field(c.g[S0][0], 0x1dc).lw(c);
        let no_smv = c.g[A0][0] == 0;
        c.g[S3][0] = c.g[V0][0];
        if !no_smv {
            c.cycles += 3;
            call_ptr(c, 0x003b_5e98, 0x003c_8054)?;
            reg_field(c.g[S0][0], 0x1dc).sw(c, 0);
        }
        // The two temp buffers (sndmix + 0xa8), then the mix buffers (+0xb8).
        c.cycles += 4;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa8));
        c.g[S2][0] = 0x7000_0000;
        c.g[S1][0] = 1;
        'l_3c8068: loop {
            'b_3c8090: {
                'b_3c808c: {
                    c.cycles += 4;
                    c.g[A0][0] = reg_field(c.g[S0][0], 0).lw(c);
                    c.g[V0][0] = c.g[A0][0] & c.g[S2][0];
                    if c.g[V0][0] == c.g[S2][0] {
                        reg_field(c.g[S0][0], 0).sw(c, 0);
                        break 'b_3c808c;
                    }
                    c.cycles += 2;
                    if c.g[A0][0] == 0 {
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
                        break 'b_3c8090;
                    }
                    c.cycles += 3;
                    call_ptr(c, 0x003b_5e98, 0x003c_8088)?;
                    reg_field(c.g[S0][0], 0).sw(c, 0);
                }
                c.cycles += 1;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            }
            c.cycles += 2;
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x003c_8068);
                }
                continue 'l_3c8068;
            }
            'b_3c80f0: {
                c.cycles += 4;
                c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5b40));
                c.g[V0][0] = reg_field(c.g[V1][0], 5).lbu(c);
                let no_channels = c.g[V0][0] == 0;
                c.g[S1][0] = 0;
                if no_channels {
                    break 'b_3c80f0;
                }
                c.cycles += 4;
                c.g[S3][0] = c.g[V1][0];
                c.g[S2][0] = 0x7000_0000;
                c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(0xb8));
                'l_3c80b8: loop {
                    'b_3c80e0: {
                        'b_3c80dc: {
                            c.cycles += 4;
                            c.g[A0][0] = reg_field(c.g[S0][0], 0).lw(c);
                            c.g[V0][0] = c.g[A0][0] & c.g[S2][0];
                            if c.g[V0][0] == c.g[S2][0] {
                                reg_field(c.g[S0][0], 0).sw(c, 0);
                                break 'b_3c80dc;
                            }
                            c.cycles += 2;
                            if c.g[A0][0] == 0 {
                                c.g[V0][0] = reg_field(c.g[S3][0], 5).lbu(c);
                                break 'b_3c80e0;
                            }
                            c.cycles += 3;
                            call_ptr(c, 0x003b_5e98, 0x003c_80d8)?;
                            reg_field(c.g[S0][0], 0).sw(c, 0);
                        }
                        c.cycles += 1;
                        c.g[V0][0] = reg_field(c.g[S3][0], 5).lbu(c);
                    }
                    c.cycles += 4;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more_mix = c.g[V0][0] != 0;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                    if !more_mix {
                        break 'l_3c80b8;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_80b8);
                    }
                    continue 'l_3c80b8;
                }
            }
            break 'l_3c8068;
        }
        // Stop the mixer (the stub gets 2.0f in f12), unlock, restore.
        c.cycles += 13;
        c.g[AT][0] = 0x4000_0000;
        c.mtc1(1, 12);
        CALL_MIXER_SHUTDOWN_STUB.call(c, 0x003c_8100)?;
        call_ptr(c, 0x003b_58d8, 0x003c_8108)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MIX_getframe`, `int SND_EeVoiceGetPosition(int voice)` (0x3c8a40): the
/// voice's position comes from its `unpackgetframe` callback, called with the
/// voice's `pups` field; -1 when the voice has no callback.
pub fn snd_ee_voice_get_position(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[V1][0] = 0x60;
        c.g[A1][0] = 0x51_0000;
        c.mult(0, 4, 4, 3);
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = reg_field(c.g[A1][0], 0x5d1c).lw(c);
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = reg_field(c.g[V0][0], 0x44).lw(c);
        if c.g[V1][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
        } else {
            c.cycles += 2;
            let callback = c.g[V1][0] as u32;
            c.g[A0][0] = reg_field(c.g[V0][0], 0x48).lw(c);
            call_ptr(c, callback, 0x003c_8a78)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SND_EeVoiceAddDsp(int voice, int channel, FilterDesc* desc)`
/// (0x3c8e28): allocates a filter node of `desc->statesize` bytes, runs the
/// descriptor's `filterinit(node, desc->id, channel)`, copies the filter
/// callbacks and priority into the node, clears its `requester` byte and
/// inserts it into the voice's filter list (+0x40) by priority.
pub fn snd_ee_voice_add_dsp(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        c.g[V0][0] = 0x60;
        let frame = Frame::open(c, 0x50);
        c.mult(0, 7, 4, 2);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[V1][0] = 0x51_0000;
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0];
        c.g[S2][0] = reg_field(c.g[V1][0], 0x5d1c).lw(c);
        c.g[A0][0] = reg_field(c.g[S1][0], 4).lw(c);
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A3][0] as u32));
        call_ptr(c, 0x003b_5c60, 0x003c_8e64)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = reg_field(c.g[S1][0], 0).lw(c);
        c.g[V0][0] = reg_field(c.g[S1][0], 0xc).lw(c);
        c.g[A2][0] = c.g[S3][0];
        let filterinit = c.g[V0][0] as u32;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, filterinit, 0x003c_8e7c)?;
        c.g[V1][0] = reg_field(c.g[S1][0], 8).lhu(c);
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = c.g[S0][0];
        let priority = c.g[V1][0];
        reg_field(c.g[S0][0], 0x18).sh(c, priority);
        c.g[V0][0] = reg_field(c.g[S1][0], 0x10).lw(c);
        let filter = c.g[V0][0];
        reg_field(c.g[S0][0], 0).sw(c, filter);
        c.g[V1][0] = reg_field(c.g[S1][0], 0x14).lw(c);
        reg_field(c.g[S0][0], 0x1a).sb(c, 0);
        let restore = c.g[V1][0];
        reg_field(c.g[S0][0], 4).sw(c, restore);
        CALL_DSP_INSERT_SORTED.call(c, 0x003c_8ea4)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SND_EeVoiceRemoveDsp(int voice, int filterOrder)` (0x3c8ec0): walks
/// the voice's filter list (+0x40) for the first node of priority
/// `filterOrder`, runs its restore callback, unlinks it and frees it. A walk
/// that finds nothing returns with the list untouched.
pub fn snd_ee_voice_remove_dsp(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[V0][0] = 0x60;
        c.g[V1][0] = 0x51_0000;
        c.mult(0, 4, 4, 2);
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = reg_field(c.g[V1][0], 0x5d1c).lw(c);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[S0][0] = reg_field(c.g[S1][0], 0x40).lw(c);
        c.g[V0][0] = reg_field(c.g[S0][0], 0x18).lhu(c);
        'l_3c8ef0: loop {
            {
                c.cycles += 2;
                if c.g[V0][0] != c.g[A1][0] {
                    c.g[S0][0] = reg_field(c.g[S0][0], 8).lw(c);
                } else {
                    'b_3c8f10: {
                        c.cycles += 3;
                        c.g[V0][0] = reg_field(c.g[S0][0], 4).lw(c);
                        let no_restore = c.g[V0][0] == 0;
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x40));
                        if no_restore {
                            break 'b_3c8f10;
                        }
                        c.cycles += 3;
                        let restore = c.g[V0][0] as u32;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, restore, 0x003c_8f0c)?;
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x40));
                    }
                    c.cycles += 6;
                    c.g[A1][0] = c.g[S0][0];
                    CALL_DSP_REMOVE.call(c, 0x003c_8f18)?;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003b_5e98, 0x003c_8f20)?;
                    frame.restore(c, S0, 0x20);
                    break 'l_3c8ef0;
                }
            }
            c.cycles += 2;
            if c.g[S0][0] != 0 {
                c.g[V0][0] = reg_field(c.g[S0][0], 0x18).lhu(c);
                if event_due(c) {
                    deliver_event_here(c, 0x003c_8ef0);
                }
                continue 'l_3c8ef0;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
            break 'l_3c8ef0;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SND_DspRemove(SND_DspNode** head, SND_DspNode* node)` (0x3c8fb0):
/// unlinks `node` from the singly linked list at `*head` (with `prev` links
/// kept for the successor). Nothing is freed here.
pub fn snd_dsp_remove(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A2][0] = reg_field(c.g[A0][0], 0).lw(c);
    let found_not_head = c.g[A1][0] != c.g[A2][0];
    c.g[V0][0] = reg_field(c.g[A2][0], 8).lw(c);
    if found_not_head {
        'l_3c8fcc: loop {
            'b_3c9004: {
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    break 'b_3c9004;
                }
                c.cycles += 2;
                if c.g[V0][0] != c.g[A1][0] {
                    c.g[A2][0] = c.g[V0][0];
                    c.cycles += 1;
                    c.g[V0][0] = reg_field(c.g[A2][0], 8).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_8fcc);
                    }
                    continue 'l_3c8fcc;
                }
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    break 'b_3c9004;
                }
                c.cycles += 2;
                if c.g[V0][0] != c.g[A1][0] {
                    break 'b_3c9004;
                }
                {
                    c.cycles += 3;
                    c.g[A1][0] = reg_field(c.g[A1][0], 8).lw(c);
                    if c.g[A1][0] != 0 {
                        let prev = c.g[A2][0];
                        reg_field(c.g[A1][0], 0x10).sw(c, prev);
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = reg_field(c.g[A2][0], 8).lw(c);
                c.g[V1][0] = reg_field(c.g[V0][0], 8).lw(c);
                let next = c.g[V1][0];
                reg_field(c.g[A2][0], 8).sw(c, next);
            }
            break 'l_3c8fcc;
        }
        c.cycles += 2;
        ret(c);
    } else {
        c.cycles += 2;
        let head_next = c.g[V0][0];
        reg_field(c.g[A0][0], 0).sw(c, head_next);
        ret(c);
    }
}

/// `int SND_DspNode_CallFree(SND_DspNode* node)` (0x3c9010): runs the node's
/// restore callback (+4) if it has one; returns 0.
pub fn snd_dsp_node_call_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = reg_field(c.g[A0][0], 4).lw(c);
        let no_restore = c.g[V0][0] == 0;
        frame.restore_ra(c, 0);
        if !no_restore {
            c.cycles += 3;
            let restore = c.g[V0][0] as u32;
            call_ptr(c, restore, 0x003c_902c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 3;
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    })
}

/// `int SND_DspNode_DestroyTree(SND_DspNode* node)` (0x3c9040): frees a
/// node's two children (+8, +0xc) first unless they are already clean (the
/// clean flag is the byte at +0x1b), runs the node's restore callback (+4),
/// then marks the node clean. Returns 1.
pub fn snd_dsp_node_destroy_tree(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = reg_field(c.g[S0][0], 8).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = reg_field(c.g[A0][0], 0x1b).lbu(c);
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                CALL_DSP_NODE_DESTROY_TREE.call(c, 0x003c_9070)?;
            }
        }
        c.g[A0][0] = reg_field(c.g[S0][0], 0xc).lw(c);
        c.cycles += 2;
        if c.g[A0][0] == 0 {
            c.g[V0][0] = reg_field(c.g[S0][0], 4).lw(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = reg_field(c.g[A0][0], 0x1b).lbu(c);
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                CALL_DSP_NODE_DESTROY_TREE.call(c, 0x003c_9090)?;
            }
            c.g[V0][0] = reg_field(c.g[S0][0], 4).lw(c);
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 1;
        } else {
            c.cycles += 3;
            let restore = c.g[V0][0] as u32;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, restore, 0x003c_90a4)?;
            c.g[V0][0] = 1;
        }
        c.cycles += 5;
        frame.restore_ra(c, 0);
        let clean = c.g[V0][0];
        reg_field(c.g[S0][0], 0x1b).sb(c, clean);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, fake, fall_back, hook, machine, FAKE};
    use crate::difftest::{check, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate magic: a gated fake answers only while its gate word holds this.
    const OC1155_MAGIC: u32 = 0x1155_0a0d;
    /// Gate words (one per fake, in the test machine's RAM).
    const GATE_LOCK: u32 = DATA + 0x1f_0000;
    const GATE_UNLOCK: u32 = DATA + 0x1f_0010;
    const GATE_MEM_FREE: u32 = DATA + 0x1f_0020;
    const GATE_MEM_ALLOC: u32 = DATA + 0x1f_0030;
    const GATE_INSERT: u32 = DATA + 0x1f_0040;

    /// A gated fake of a table callee: logs the call (returning to `ra`) while
    /// its gate is on, else runs what the slot held before the hook.
    macro_rules! oc1155_fake {
        ($name:ident, $prev:ident, $addr:expr, $gen:path, $gate:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32($gate) != OC1155_MAGIC {
                    return fall_back(c, &$prev, $gen);
                }
                let ra = c.g[RA][0] as u32;
                fake(c, $addr, ra);
            }
        };
    }

    oc1155_fake!(
        oc1155_fake_lock,
        OC1155_PREV_LOCK,
        0x003b_58a0,
        ee::s13::f_003b58a0__generated,
        GATE_LOCK
    );
    oc1155_fake!(
        oc1155_fake_unlock,
        OC1155_PREV_UNLOCK,
        0x003b_58d8,
        ee::s13::f_003b58d8__generated,
        GATE_UNLOCK
    );
    oc1155_fake!(
        oc1155_fake_mem_free,
        OC1155_PREV_MEM_FREE,
        0x003b_5e98,
        ee::s13::f_003b5e98__generated,
        GATE_MEM_FREE
    );
    oc1155_fake!(
        oc1155_fake_mem_alloc,
        OC1155_PREV_MEM_ALLOC,
        0x003b_5c60,
        ee::s13::f_003b5c60__generated,
        GATE_MEM_ALLOC
    );
    oc1155_fake!(
        oc1155_fake_insert,
        OC1155_PREV_INSERT,
        0x003c_8f48,
        ee::s14::f_003c8f48__generated,
        GATE_INSERT
    );

    /// Hook the gated fakes into their override slots (once per process).
    fn oc1155_install() {
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B58A0),
            oc1155_fake_lock,
            &OC1155_PREV_LOCK,
        );
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B58D8),
            oc1155_fake_unlock,
            &OC1155_PREV_UNLOCK,
        );
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B5E98),
            oc1155_fake_mem_free,
            &OC1155_PREV_MEM_FREE,
        );
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B5C60),
            oc1155_fake_mem_alloc,
            &OC1155_PREV_MEM_ALLOC,
        );
        hook(
            addr_of_mut!(ee::s14::OVERRIDE_003C8F48),
            oc1155_fake_insert,
            &OC1155_PREV_INSERT,
        );
    }

    /// Switch the given gated fakes on in this machine.
    fn oc1155_gate(c: &mut Cpu, gates: &[u32]) {
        for &g in gates {
            c.bus.write32(g, OC1155_MAGIC);
        }
    }

    /// A punalignedtempbuf / punalignedmixbuf entry: empty, flagged, or a block.
    fn oc1155_buffer(r: &mut Rng) -> u32 {
        match r.next_u32() % 3 {
            0 => 0,
            1 => 0x7000_1000,
            _ => DATA + 0x1100 + 0x40 * (r.next_u32() % 8),
        }
    }

    #[test]
    fn snd_ee_mixer_shutdown_stub_matches() {
        check(
            ee::s14::f_003c7aa8__generated,
            snd_ee_mixer_shutdown_stub,
            50,
            &|c, seed| {
                let mut r = machine(c, seed, 0x7aa8, 0);
                scramble(c, &mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    #[test]
    fn snd_ee_mixer_shutdown_matches() {
        oc1155_install();
        const SMV: u32 = 0x0051_5d1c;
        const TEMP: u32 = 0x0051_5be8;
        const MIX: u32 = 0x0051_5bf8;
        const CHANNELS: u32 = 0x0051_5b45;
        check(
            ee::s14::f_003c8018__generated,
            snd_ee_mixer_shutdown,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x8018, 0);
                oc1155_gate(c, &[GATE_LOCK, GATE_UNLOCK, GATE_MEM_FREE]);
                c.bus
                    .write32(SMV, if seed % 3 == 0 { 0 } else { DATA + 0x1000 });
                for i in 0..2 {
                    let v = oc1155_buffer(&mut r);
                    c.bus.write32(TEMP + 4 * i, v);
                }
                for i in 0..4 {
                    let v = oc1155_buffer(&mut r);
                    c.bus.write32(MIX + 4 * i, v);
                }
                c.bus.write8(CHANNELS, (seed % 5) as u8);
            },
        );
    }

    #[test]
    fn snd_ee_voice_get_position_matches() {
        const SMV: u32 = 0x0051_5d1c;
        covered(
            ee::s14::f_003c8a40__generated,
            snd_ee_voice_get_position,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x8a40, 1);
                let voice = seed % 4;
                c.g[A0][0] = voice as u64;
                c.bus.write32(SMV, DATA + 0x1000);
                let state = DATA + 0x1000 + 0x60 * voice;
                let callback = if seed % 3 == 0 {
                    0
                } else {
                    FAKE + 0x40 * (1 + seed % 2)
                };
                c.bus.write32(state + 0x44, callback);
                c.bus.write32(state + 0x48, r.next_u32());
            },
        );
    }

    #[test]
    fn snd_ee_voice_add_dsp_matches() {
        oc1155_install();
        const SMV: u32 = 0x0051_5d1c;
        check(
            ee::s14::f_003c8e28__generated,
            snd_ee_voice_add_dsp,
            300,
            &|c, seed| {
                machine(c, seed, 0x8e28, 2);
                oc1155_gate(c, &[GATE_MEM_ALLOC, GATE_INSERT]);
                let desc = DATA + 0x2000;
                c.g[A0][0] = (seed % 4) as u64;
                c.g[A1][0] = (seed / 4 % 3) as u64;
                c.g[A2][0] = desc as u64;
                c.bus.write32(SMV, DATA + 0x1000);
                c.bus.write32(desc, 0x1234 + seed);
                c.bus.write32(desc + 4, 0x40 * (1 + seed % 3));
                c.bus.write16(desc + 8, (seed % 7) as u16);
                c.bus.write32(desc + 0xc, FAKE + 0x80);
                c.bus.write32(desc + 0x10, FAKE + 0xc0);
                c.bus.write32(desc + 0x14, FAKE + 0x100 + (seed % 2) * 0x10);
            },
        );
    }

    #[test]
    fn snd_ee_voice_remove_dsp_matches() {
        oc1155_install();
        const SMV: u32 = 0x0051_5d1c;
        check(
            ee::s14::f_003c8ec0__generated,
            snd_ee_voice_remove_dsp,
            300,
            &|c, seed| {
                machine(c, seed, 0x8ec0, 0);
                oc1155_gate(c, &[GATE_MEM_FREE]);
                let voice = seed % 4;
                let count = seed / 4 % 4;
                let state = DATA + 0x1000 + 0x60 * voice;
                let node = |i: u32| DATA + 0x3000 + 0x40 * i;
                c.bus.write32(SMV, DATA + 0x1000);
                c.bus
                    .write32(state + 0x40, if count > 0 { node(0) } else { 0 });
                for i in 0..count {
                    let n = node(i);
                    let next = if i + 1 < count { node(i + 1) } else { 0 };
                    c.bus.write32(n + 8, next);
                    let restore = if (seed + i) % 3 == 0 { 0 } else { FAKE + 0x40 };
                    c.bus.write32(n + 4, restore);
                    c.bus.write16(n + 0x18, ((seed >> i) % 3) as u16);
                }
                c.g[A0][0] = voice as u64;
                c.g[A1][0] = if seed % 5 == 4 {
                    7
                } else {
                    (seed / 3 % 3) as u64
                };
            },
        );
    }

    #[test]
    fn snd_dsp_remove_matches() {
        check(
            ee::s14::f_003c8fb0__generated,
            snd_dsp_remove,
            300,
            &|c, seed| {
                machine(c, seed, 0x8fb0, 0);
                let head = DATA + 0x2000;
                let node = |i: u32| DATA + 0x3000 + 0x40 * i;
                let count = seed % 5;
                c.bus
                    .write32(head, if count > 0 { node(0) } else { 0 });
                for i in 0..count {
                    let next = if i + 1 < count { node(i + 1) } else { 0 };
                    c.bus.write32(node(i) + 8, next);
                    c.bus.write32(node(i) + 0x10, 0);
                }
                c.g[A0][0] = head as u64;
                let target = seed / 5 % 6;
                c.g[A1][0] = if target < count {
                    node(target) as u64
                } else {
                    (DATA + 0x3f00) as u64
                };
            },
        );
    }

    #[test]
    fn snd_dsp_node_call_free_matches() {
        covered(
            ee::s14::f_003c9010__generated,
            snd_dsp_node_call_free,
            200,
            &|c, seed| {
                machine(c, seed, 0x9010, 1);
                let node = DATA + 0x4000;
                c.g[A0][0] = node as u64;
                let restore = if seed % 3 == 0 { 0 } else { FAKE + 0x40 };
                c.bus.write32(node + 4, restore);
            },
        );
    }

    #[test]
    fn snd_dsp_node_destroy_tree_matches() {
        const ROOT: u32 = DATA + 0x5000;
        const KID_A: u32 = DATA + 0x5040;
        const KID_B: u32 = DATA + 0x5080;
        covered(
            ee::s14::f_003c9040__generated,
            snd_dsp_node_destroy_tree,
            200,
            &|c, seed| {
                machine(c, seed, 0x9040, 2);
                c.g[A0][0] = ROOT as u64;
                c.bus
                    .write32(ROOT + 8, if seed & 1 != 0 { KID_A } else { 0 });
                c.bus
                    .write32(ROOT + 0xc, if seed & 2 != 0 { KID_B } else { 0 });
                c.bus
                    .write32(ROOT + 4, if seed % 3 == 0 { 0 } else { FAKE + 0x40 });
                c.bus.write8(ROOT + 0x1b, ((seed >> 2) & 1) as u8);
                for (kid, bit) in [(KID_A, 3), (KID_B, 4)] {
                    c.bus.write32(kid + 4, FAKE + 0x80);
                    c.bus.write32(kid + 8, 0);
                    c.bus.write32(kid + 0xc, 0);
                    c.bus.write8(kid + 0x1b, ((seed >> bit) & 1) as u8);
                }
            },
        );
    }
}
