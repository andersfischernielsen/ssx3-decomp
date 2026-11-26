//! Staging module oc794 (wave wave246): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc794`.

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
    (
        0x002a_1e20,
        ssx_audio_system_track_to_category,
        "SSXAudioSystem_TrackToCategory",
    ),
    (
        0x0028_a1e8,
        snd_load_queue_alloc_async_buffer,
        "SndLoadQueue_AllocAsyncBuffer",
    ),
    (0x003d_6c60, spch_set_bank_table, "SPCH_SetBankTable"),
    (0x003b_80a0, sndstrm_set_lowpass, "SNDSTRM_SetLowpass"),
    (
        0x002b_4b08,
        world_trigger_manager_create_instance,
        "WorldTriggerManager_CreateInstance",
    ),
    (
        0x0029_b960,
        ssx_audio_system_empty_trigger_update,
        "SSXAudioSystem_EmptyTriggerUpdate",
    ),
];

/// `int SSXAudioSystem::TrackToCategory(SSXAudioSystem* this, int track)` (0x2a1e20).
///
/// The music category of a track number: `track` 0..=21 indexes a jump table
/// (0x483010) whose cases return 1, 2 or 4; any other track returns 0.
pub fn ssx_audio_system_track_to_category(c: &mut Cpu) {
    if c.pc != 0x002a_1e20 {
        return ee::s08::f_002a1e20__entries(c);
    }
    c.cycles += 3;
    c.g[V0][0] = (c.g[A1][0] < 0x16u64) as u64;
    let out_of_range = c.g[V0][0] == 0;
    c.g[V0][0] = 0x48_0000;
    if out_of_range {
        c.cycles += 2;
        c.g[V0][0] = 0;
        ret(c);
        return;
    }
    c.cycles += 6;
    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3010));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
    let target = c.g[A0][0] as u32;
    let category = match target {
        0x002a_1e44 => 1,
        0x002a_1e4c => 2,
        0x002a_1e54 => 4,
        _ => {
            c.jump_out(target, 0x002a_1e20, 0x002a_1e64);
            return;
        }
    };
    c.cycles += 2;
    c.g[V0][0] = category;
    ret(c);
}

/// `SndLoadQueue::AllocAsyncBuffer(this, size)` (0x28a1e8).
///
/// With a non-zero `size`, allocates the async load buffer with the named
/// allocator (`operator_new_named`, size, "...", 0, 0); the size goes to
/// `this+0x1d0` and the buffer to `this+0x1cc`. A zero size does nothing.
pub fn snd_load_queue_alloc_async_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let no_size = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        if !no_size {
            c.cycles += 7;
            c.g[A1][0] = 0x48_0000;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2988));
            let this = r32(c, S0);
            c.sw(this.wrapping_add(SNDLOADQUEUE_ASYNC_SIZE), c.g[V0][0]);
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7e30, 0x0028_a218)?;
            let this = r32(c, S0);
            c.sw(this.wrapping_add(SNDLOADQUEUE_ASYNC_BUFFER), c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SndLoadQueue`: the async buffer size and pointer.
pub(crate) const SNDLOADQUEUE_ASYNC_SIZE: u32 = 0x1d0;
pub(crate) const SNDLOADQUEUE_ASYNC_BUFFER: u32 = 0x1cc;

/// `void SPCH_SetBankTable(int count, int* table)` (0x3d6c60).
///
/// Remembers the speech bank table (`gp-relative` count at `+0x1738`, pointer
/// at `+0x1734`) and clears its `count` entries when there are any.
pub fn spch_set_bank_table(c: &mut Cpu) {
    c.cycles += 3;
    let gp = c.g[GP][0] as u32;
    c.sw(gp.wrapping_add(SPCH_BANK_COUNT), c.g[A0][0]);
    let no_table = c.g[A1][0] == 0;
    c.sw(gp.wrapping_add(SPCH_BANK_TABLE), c.g[A1][0]);
    'out: {
        if no_table {
            break 'out;
        }
        c.cycles += 2;
        let empty = (c.g[A0][0] as i64) <= 0;
        c.g[A1][0] = 0;
        if empty {
            break 'out;
        }
        c.cycles += 1;
        c.g[V0][0] = c.lw(gp.wrapping_add(SPCH_BANK_TABLE));
        loop {
            c.cycles += 8;
            c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw(c.g[V1][0] as u32, 0);
            c.g[V0][0] = c.lw(gp.wrapping_add(SPCH_BANK_COUNT));
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = c.lw(gp.wrapping_add(SPCH_BANK_TABLE));
            if !more {
                break 'out;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003d_6c78);
            }
        }
    }
    c.cycles += 2;
    ret(c);
}

/// gp offsets of the speech bank table count and pointer.
pub(crate) const SPCH_BANK_COUNT: u32 = 0x1738;
pub(crate) const SPCH_BANK_TABLE: u32 = 0x1734;

/// `int SNDSTRM_SetLowpass(int stream, int value)` (0x3b80a0).
///
/// Stores `value` (halfword) in the stream object's `+0x100` and applies it
/// to the stream's voice (`SND_SetLowpass(*(stream+4), value)`). Returns 0, or
/// -8 when the stream does not exist.
pub fn sndstrm_set_lowpass(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        SNDSTRM_SETLOWPASS_GET_STREAM.call(c, 0x003b_80b4)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0xffff_ffff_ffff_fff8;
        } else {
            c.cycles += 5;
            c.g[A0][0] = Addr(r32(c, V0)).off(SNDSTRM_VOICE).lw(c);
            c.g[A1][0] = c.g[S0][0];
            let stream = r32(c, V0);
            c.sh(stream.wrapping_add(SNDSTRM_LOWPASS), c.g[S0][0]);
            SNDSTRM_SETLOWPASS_SND_SET_LOWPASS.call(c, 0x003b_80cc)?;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Stream object: the voice handle (+4) and the low-pass value (+0x100).
pub(crate) const SNDSTRM_VOICE: u32 = 0x4;
pub(crate) const SNDSTRM_LOWPASS: u32 = 0x100;

pub(crate) const SNDSTRM_SETLOWPASS_GET_STREAM: Callee = Callee {
    addr: 0x003b_6a38,
    f: ee::s13::SNDSTRM_GetStream,
};

pub(crate) const SNDSTRM_SETLOWPASS_SND_SET_LOWPASS: Callee = Callee {
    addr: 0x003b_8978,
    f: ee::s13::SND_SetLowpass,
};

/// `WorldTriggerManager* WorldTriggerManager::CreateInstance()` (0x2b4b08).
///
/// The singleton (gp-0x... word at `gp+0x21e4`): when it does not exist yet,
/// allocates 0x344 bytes with the named allocator and constructs it.
pub fn world_trigger_manager_create_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let gp = c.g[GP][0] as u32;
        c.g[V0][0] = c.lw(gp.wrapping_add(WORLD_TRIGGER_MANAGER_INSTANCE));
        let exists = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if !exists {
            c.cycles += 9;
            c.g[A1][0] = 0x48_0000;
            c.g[A0][0] = 0x344;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3588));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x002b_4b30)?;
            c.g[A0][0] = c.g[V0][0];
            WORLD_TRIGGER_MANAGER_CTOR.call(c, 0x002b_4b38)?;
            let gp = c.g[GP][0] as u32;
            c.sw(gp.wrapping_add(WORLD_TRIGGER_MANAGER_INSTANCE), c.g[V0][0]);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// gp offset of the `WorldTriggerManager` singleton pointer.
pub(crate) const WORLD_TRIGGER_MANAGER_INSTANCE: u32 = 0x21e4;

pub(crate) const WORLD_TRIGGER_MANAGER_CTOR: Callee = Callee {
    addr: 0x002b_58d0,
    f: ee::s08::WorldTriggerManager_ctor,
};

/// `void SSXAudioSystem::EmptyTriggerUpdate()` (0x29b960): does nothing.
pub fn ssx_audio_system_empty_trigger_update(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, put_words, DATA};
    use std::sync::atomic::AtomicUsize;

    /// The word that arms oc794's callee fakes in a machine's RAM.
    pub(crate) const OC794_CTRL: u32 = DATA + 0x32_0000;
    pub(crate) const OC794_MAGIC: u32 = 0x0794_f4ce;

    static OC794_PREV_NEW: AtomicUsize = AtomicUsize::new(0);
    static OC794_PREV_ALLOC: AtomicUsize = AtomicUsize::new(0);
    static OC794_PREV_CTOR: AtomicUsize = AtomicUsize::new(0);

    fn oc794_fk_new(c: &mut Cpu) {
        if c.bus.read32(OC794_CTRL) != OC794_MAGIC {
            return fakes::fall_back(c, &OC794_PREV_NEW, ee::s10::f_00317e30__generated);
        }
        fakes::fake_call(c, 0x0031_7e30);
    }

    fn oc794_fk_alloc(c: &mut Cpu) {
        if c.bus.read32(OC794_CTRL) != OC794_MAGIC {
            return fakes::fall_back(c, &OC794_PREV_ALLOC, ee::s10::f_00317d70__generated);
        }
        fakes::fake_call(c, 0x0031_7d70);
    }

    fn oc794_fk_ctor(c: &mut Cpu) {
        if c.bus.read32(OC794_CTRL) != OC794_MAGIC {
            return fakes::fall_back(c, &OC794_PREV_CTOR, ee::s08::f_002b58d0__generated);
        }
        fakes::fake_call(c, 0x002b_58d0);
    }

    /// Hook the gated fakes (once per process) and arm them in this machine.
    fn oc794_arm(c: &mut Cpu) {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc794_fk_new,
            &OC794_PREV_NEW,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc794_fk_alloc,
            &OC794_PREV_ALLOC,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B58D0),
            oc794_fk_ctor,
            &OC794_PREV_CTOR,
        );
        c.bus.write32(OC794_CTRL, OC794_MAGIC);
    }

    /// `SSXAudioSystem_TrackToCategory`: tracks 0..=21 over a jump table of
    /// the three known cases and a corrupt entry (jump out), tracks out of
    /// range (also negative and 64-bit values).
    #[test]
    fn ssx_audio_system_track_to_category_matches() {
        check(
            ee::s08::f_002a1e20__generated,
            ssx_audio_system_track_to_category,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x794a, 0);
                c.pc = 0x002a_1e20;
                let cases = [0x002a_1e44u32, 0x002a_1e4c, 0x002a_1e54, 0x002a_1e5c];
                let mut table = [0u32; 22];
                for t in table.iter_mut() {
                    *t = cases[(r.next_u32() % 4) as usize];
                }
                put_words(c, 0x0048_3010, &table);
                c.g[A1][0] = match seed % 8 {
                    0 => sx(0x8000_0000 | r.next_u32()),
                    1 => 22 + (r.next_u32() % 5) as u64,
                    2 => 1u64 << 32 | (r.next_u32() % 22) as u64,
                    _ => (seed % 22) as u64,
                };
            },
        );
    }

    pub(crate) const OC794_THIS: u32 = DATA + 0x1_0000;

    /// `SndLoadQueue_AllocAsyncBuffer`: zero and non-zero sizes and the
    /// unwind of the allocator call.
    #[test]
    fn snd_load_queue_alloc_async_buffer_matches() {
        fakes::covered(
            ee::s07::f_0028a1e8__generated,
            snd_load_queue_alloc_async_buffer,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x794b, 1);
                oc794_arm(c);
                c.pc = 0x0028_a1e8;
                c.g[A0][0] = sx(OC794_THIS);
                c.g[A1][0] = if seed % 3 == 0 {
                    0
                } else {
                    sx(r.next_u32() & 0xf_fff0 | 0x10)
                };
            },
        );
    }

    /// `SPCH_SetBankTable`: no table, an empty or negative count, and tables
    /// of 1..=9 entries over random old contents.
    #[test]
    fn spch_set_bank_table_matches() {
        check(
            ee::s14::f_003d6c60__generated,
            spch_set_bank_table,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x794c, 0);
                c.pc = 0x003d_6c60;
                for i in 0..12u32 {
                    c.bus.write32(OC794_THIS + 4 * i, r.next_u32() | 1);
                }
                c.g[A1][0] = if seed % 7 == 0 { 0 } else { sx(OC794_THIS) };
                c.g[A0][0] = match seed % 5 {
                    0 => 0,
                    1 => sx(0u32.wrapping_sub(1 + seed % 4)),
                    _ => (1 + seed % 9) as u64,
                };
            },
        );
    }

    const OC794_STREAM_COUNT: u32 = 0x0050_a92e;
    const OC794_STREAMS: u32 = 0x0050_a0a0;

    /// `SNDSTRM_SetLowpass`: a missing stream (out of range or null slot) and
    /// streams whose voice handle is negative or names no channel (the real
    /// `SND_SetLowpass` then finds no channel).
    #[test]
    fn sndstrm_set_lowpass_matches() {
        check(
            ee::s13::f_003b80a0__generated,
            sndstrm_set_lowpass,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x794d, 0);
                c.pc = 0x003b_80a0;
                c.bus.write8(OC794_STREAM_COUNT, 4);
                for i in 0..4u32 {
                    let stream = OC794_THIS + 0x1000 * i;
                    c.bus
                        .write32(OC794_STREAMS + 4 * i, if i == 3 { 0 } else { stream });
                    let handle = if i == 1 {
                        0x8000_0000 | r.next_u32()
                    } else {
                        r.next_u32() & 0x7fff_ffff
                    };
                    c.bus.write32(stream + SNDSTRM_VOICE, handle);
                }
                c.g[A0][0] = match seed % 6 {
                    4 => 4 + (r.next_u32() % 5) as u64,
                    5 => sx(0x8000_0000 | r.next_u32()),
                    _ => (seed % 4) as u64 & 0xff,
                };
                c.g[A1][0] = r.next_u32() as u64 | (r.next_u32() as u64) << 32;
            },
        );
    }

    /// `WorldTriggerManager_CreateInstance`: the singleton already there, or
    /// created (allocator and constructor), with the unwinds of both calls.
    #[test]
    fn world_trigger_manager_create_instance_matches() {
        fakes::covered(
            ee::s08::f_002b4b08__generated,
            world_trigger_manager_create_instance,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x794e, 2);
                oc794_arm(c);
                c.pc = 0x002b_4b08;
                let gp = c.g[GP][0] as u32;
                let old = if seed % 4 == 0 { r.next_u32() | 1 } else { 0 };
                c.bus
                    .write32(gp.wrapping_add(WORLD_TRIGGER_MANAGER_INSTANCE), old);
            },
        );
    }

    /// `SSXAudioSystem_EmptyTriggerUpdate`: returns after two cycles.
    #[test]
    fn ssx_audio_system_empty_trigger_update_matches() {
        check(
            ee::s08::f_0029b960__generated,
            ssx_audio_system_empty_trigger_update,
            4,
            &|c, seed| {
                machine(c, seed, 0x794f, 0);
                c.pc = 0x0029_b960;
            },
        );
    }
}
