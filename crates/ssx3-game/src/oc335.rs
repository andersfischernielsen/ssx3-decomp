//! Staging module oc335 (wave wave214): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc335`.

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
    (0x003d_b9a0, spch_get_time, "SPCH_GetTime"),
    (
        0x0035_2c38,
        modifier_block_set_transform_radius,
        "ModifierBlock_SetTransformRadius",
    ),
    (0x003b_7d80, sndstrm_set_prebuffer, "SNDSTRM_SetPrebuffer"),
    (0x0034_2fa8, debounce_save, "Debounce_Save"),
    (0x002a_ad78, stream_array_queue, "StreamArray_Queue"),
    (
        0x002a_9f38,
        stream_buffer_set_pitch,
        "StreamBuffer_SetPitch",
    ),
    (0x003c_7cc0, snd_dec_alloc_thunk, "SND_DecAllocThunk"),
    (0x0038_5260, grid_mesh_base_dtor, "GridMeshBase_dtor"),
    (
        0x002a_bcf8,
        sound_request_set_stream,
        "SoundRequest_SetStream",
    ),
    (
        0x0011_9bb0,
        rider_metrix_get_recovery_boost,
        "RiderMetrix_GetRecoveryBoost",
    ),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::{ee, Callee};

    pub const SND_LOCK: Callee = Callee {
        addr: 0x003b_58a0,
        f: ee::s13::SND_Lock,
    };

    pub const SND_UNLOCK: Callee = Callee {
        addr: 0x003b_58d8,
        f: ee::s13::SND_Unlock,
    };

    pub const SNDSTRM_FIND_REQUEST: Callee = Callee {
        addr: 0x003b_7a70,
        f: ee::s13::SNDSTRM_FindRequest,
    };

    pub const MOD_OBJ_SAVE: Callee = Callee {
        addr: 0x0035_6b30,
        f: ee::s11::ModObj_Save,
    };

    pub const STREAM_BUFFER_QUEUE: Callee = Callee {
        addr: 0x002a_a320,
        f: ee::s08::StreamBuffer_Queue,
    };

    pub const STRCPY: Callee = Callee {
        addr: 0x0041_66f4,
        f: ee::s15::strcpy,
    };

    pub const RIDER_METRIX_SHOW_HUD_NUMBER: Callee = Callee {
        addr: 0x0011_7b88,
        f: ee::s00::RiderMetrix_ShowHUDNumber,
    };
}

/// `SND_Lock` / `SND_Unlock` call these hooks.
pub(crate) const SND_LOCK_HOOK: u32 = 0x0044_c46c;
pub(crate) const SND_UNLOCK_HOOK: u32 = 0x0044_c470;
/// The sound state block, its lock nesting byte, its stream table and the
/// stream count (`SNDSTRM_GetStream`).
pub(crate) const SND_STATE: u32 = 0x0050_a8e8;
pub(crate) const SND_LOCK_DEPTH: u32 = SND_STATE + 0x176;
pub(crate) const SND_STREAMS: u32 = 0x0050_a0a0;
pub(crate) const SND_STREAM_COUNT: u32 = 0x0050_a92e;
/// The SPCH time-provider hook (`0x450000 - 0x50`).
pub(crate) const SPCH_TIME_HOOK: u32 = 0x0044_ffb0;
/// `__builtin_delete`.
pub(crate) const BUILTIN_DELETE: u32 = 0x0031_7e50;

// ---------------------------------------------------------------------------
// Ports.

/// `int SPCH_GetTime(void)` (0x3db9a0): call the time-provider hook when one
/// is installed, else return 0.
pub fn spch_get_time(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x45_0000;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = Addr(SPCH_TIME_HOOK).lw(c);
    c.g[V1][0] = 0;
    frame.save(c, RA, 0);
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        let target = r32(c, V0);
        if call_ptr(c, target, 0x003d_b9c0).is_err() {
            return;
        }
        c.g[V1][0] = c.g[V0][0];
    }
    c.cycles += 4;
    frame.restore(c, RA, 0);
    c.g[V0][0] = c.g[V1][0];
    frame.close(c);
}

/// `void tModifierBlock_setRadius(tModifierBlock* self, ...)` (0x352c38): call
/// the handler at slot `+0x7c` of the vtable held by `*(*self)` with the
/// adjusted `this` (`*(*self) + slot +0x78`).
pub fn modifier_block_set_transform_radius(c: &mut Cpu) {
    c.cycles += 5;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[A1][0] = Addr(r32(c, A0)).lw(c);
    frame.restore_ra(c, 0);
    if c.g[A1][0] != 0 {
        c.cycles += 6;
        c.g[V0][0] = Addr(r32(c, A1)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x78).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x7c).lw(c);
        let target = r32(c, V1);
        c.g[RA][0] = sx(0x0035_2c60);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        if call_ptr(c, target, 0x0035_2c60).is_err() {
            return;
        }
        frame.restore_ra(c, 0);
    }
    c.cycles += 2;
    frame.close(c);
}

/// `int SNDSTRM_SetPrebuffer(int handle, int blocks)` (0x3b7d80): under the
/// sound lock, set the request's prebuffer size (`+0x20`, a `short`) when the
/// request exists; -8 otherwise.
pub fn sndstrm_set_prebuffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = 0xffff_ffff_ffff_fff8;
        callee::SND_LOCK.call(c, 0x003b_7da4)?;
        c.g[A0][0] = c.g[S0][0];
        callee::SNDSTRM_FIND_REQUEST.call(c, 0x003b_7dac)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[S1][0] as u32) << 16);
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            c.g[S2][0] = 0;
            c.g[V0][0] = sx((r32(c, V0) as i32 >> 16) as u32);
            Addr(r32(c, V1)).off(0x20).sw(c, c.g[V0][0]);
        }
        c.cycles += 9;
        callee::SND_UNLOCK.call(c, 0x003b_7dcc)?;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Debounce_Save(void* self, void* stream)` (0x342fa8): write the base
/// object, then the 0x2c and 0x30 fields through the stream's 4-byte writer.
pub fn debounce_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        callee::MOD_OBJ_SAVE.call(c, 0x0034_2fc4)?;
        c.g[V1][0] = Addr(r32(c, S0)).lw(c);
        c.g[A1][0] = sx(r32(c, S1).wrapping_add(0x2c));
        c.g[A2][0] = 4;
        c.g[A0][0] = Addr(r32(c, V1)).off(0x8).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0xc).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(0x0034_2fe0);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
        if call_ptr(c, target, 0x0034_2fe0).is_err() {
            return Ok(());
        }
        c.g[V1][0] = Addr(r32(c, S0)).lw(c);
        c.g[A1][0] = sx(r32(c, S1).wrapping_add(0x30));
        c.g[A2][0] = 4;
        c.g[A0][0] = Addr(r32(c, V1)).off(0x8).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0xc).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(0x0034_2ffc);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0034_2ffc)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int StreamArray_Queue(void* self, int i, int a, int b, int c)` (0x2aad78):
/// queue the `i`-th entry of the array at `self+8`; -1 when it is null.
pub fn stream_array_queue(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A3][0];
        c.g[V0][0] = Addr(r32(c, A0)).off(0x8).lw(c);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
        c.g[A0][0] = Addr(r32(c, A1)).lw(c);
        let entry = c.g[A0][0];
        c.g[A3][0] = c.g[T0][0];
        if entry != 0 {
            c.cycles += 3;
            c.g[A1][0] = c.g[A2][0];
            c.g[A2][0] = c.g[V1][0];
            callee::STREAM_BUFFER_QUEUE.call(c, 0x002a_adb0)?;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int StreamBuffer_SetPitch(void* self, int pitch)` (0x2a9f38): set the
/// stream's pitch under the sound lock.
pub fn stream_buffer_set_pitch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x003b_58a0, 0x002a_9f54)?;
        c.g[A0][0] = Addr(r32(c, S0)).off(0x8).lw(c);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x003b_7fb8, 0x002a_9f60)?;
        call_ptr(c, 0x003b_58d8, 0x002a_9f68)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SND_DecAllocThunk(int size)` (0x3c7cc0): hand `size` to the driver's
/// allocator (its result is left in `v0`).
pub fn snd_dec_alloc_thunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x003b_5c60, 0x003c_7cd0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* GridMeshBase_dtor(GridMeshBase* self, int flags)` (0x385260):
/// install the class vtable and, when `flags & 1`, delete the object.
pub fn grid_mesh_base_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x49_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x49_0000u32.wrapping_add(0x3938));
        c.g[A1][0] &= 1;
        Addr(r32(c, A0)).off(0x4).sw(c, c.g[V0][0]);
        if c.g[A1][0] != 0 {
            c.cycles += 2;
            c.g[RA][0] = sx(0x0038_5284);
            call_ptr(c, BUILTIN_DELETE, 0x0038_5284)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SoundRequest_SetStream(SoundRequest* self, int a1, char* name,
/// int a3)` (0x2abcf8): store the id, copy the stream name to `+0x10` and the
/// last argument to `+0x50`.
pub fn sound_request_set_stream(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A3][0];
        Addr(r32(c, S0)).off(0xc).sw(c, c.g[A1][0]);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x10));
        c.g[A1][0] = c.g[A2][0];
        callee::STRCPY.call(c, 0x002a_bd20)?;
        Addr(r32(c, S0)).off(0x50).sw(c, c.g[S1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float RiderMetrix_GetRecoveryBoost(RiderMetrix* self, int show)`
/// (0x119bb0): when `show`, flash HUD number 0x21 at 1.5 and return the boost
/// constant at `gp-0x7a08`; 0.0 otherwise.
pub fn rider_metrix_get_recovery_boost(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        if c.g[A1][0] != 0 {
            c.cycles += 9;
            c.g[AT][0] = 0x3fc0_0000;
            c.mtc1(AT, 12);
            c.g[A1][0] = 0x21;
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            callee::RIDER_METRIX_SHOW_HUD_NUMBER.call(c, 0x0011_9bd4)?;
            let boost = (c.g[GP][0] as u32).wrapping_sub(0x7a08);
            c.f[0] = Addr(boost).read_u32(c);
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 2;
            c.mtc1(ZERO, 0);
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{check, fill, scramble, Rng, DATA, GAME_GP};

    /// A fake SND lock/unlock hook and stream-table state: a random nesting
    /// depth; `streams_ready` installs one stream whose request list holds one
    /// node matching handle 0.
    fn snd_world(c: &mut Cpu, r: &mut Rng, streams_ready: bool) {
        c.bus.write32(SND_LOCK_HOOK, FAKE);
        c.bus.write32(SND_UNLOCK_HOOK, FAKE + 0x10);
        c.bus.write8(SND_LOCK_DEPTH, (r.next_u32() & 0xff) as u8);
        let count = if streams_ready { 1 } else { 0 };
        c.bus.write8(SND_STREAM_COUNT, count);
        let stream = DATA + 0x4000;
        let node = DATA + 0x5000;
        c.bus.write32(SND_STREAMS, stream);
        c.bus
            .write32(stream + 0x11c, if streams_ready { node } else { 0 });
        c.bus.write32(node, 0); // next
        c.bus.write32(node + 0xc, 0); // the handle it matches
    }

    // ------------------------------------------------------------ SPCH_GetTime

    #[test]
    fn spch_get_time_matches() {
        covered(ee::s14::f_003db9a0__generated, spch_get_time, 300, &|c, seed| {
            let _r = machine(c, seed, 0x3db9, 1);
            c.pc = 0x003d_b9a0;
            // No hook (returns 0), or a fake the function has to call.
            c.bus
                .write32(SPCH_TIME_HOOK, if seed % 4 < 2 { 0 } else { FAKE });
        });
    }

    // ------------------------- ModifierBlock_SetTransformRadius

    #[test]
    fn modifier_block_set_transform_radius_matches() {
        covered(
            ee::s11::f_00352c38__generated,
            modifier_block_set_transform_radius,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x352c, 1);
                c.pc = 0x0035_2c38;
                let obj = DATA;
                let holder = DATA + 0x100;
                let vt = DATA + 0x200;
                let wired = seed % 4 != 0;
                c.bus.write32(obj, if wired { holder } else { 0 });
                c.bus.write32(holder, vt);
                c.bus
                    .write16(vt + 0x78, ((r.next_u32() as i16) & !3) as u16);
                c.bus.write32(vt + 0x7c, FAKE);
                c.g[A0][0] = sx(obj);
            },
        );
    }

    // ------------------------------------------------- SNDSTRM_SetPrebuffer

    #[test]
    fn sndstrm_set_prebuffer_matches() {
        covered(
            ee::s13::f_003b7d80__generated,
            sndstrm_set_prebuffer,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3b7d, 1);
                c.pc = 0x003b_7d80;
                let found = seed % 2 == 0;
                snd_world(c, &mut r, found);
                // Handle 0 with a matching request, or a negative handle (no
                // request at all).
                c.g[A0][0] = if found { 0 } else { 0xffff_ffff_ffff_ffff };
                c.g[A1][0] = if seed % 5 == 0 {
                    r.edge_f32() as u64
                } else {
                    r.next_u32() as u64
                };
            },
        );
    }

    // --------------------------------------------------------- Debounce_Save

    #[test]
    fn debounce_save_matches() {
        covered(ee::s11::f_00342fa8__generated, debounce_save, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x342f, 8);
            c.pc = 0x0034_2fa8;
            let obj = DATA;
            let stream = DATA + 0x1000;
            let table = DATA + 0x2000;
            // The stream's first word points at the writer table; entry 1
            // is the writer the whole Save chain calls.
            c.bus.write32(stream, table);
            c.bus
                .write16(table + 8, ((r.next_u32() as i16) & !3) as u16);
            c.bus.write32(table + 0xc, FAKE);
            // No modifier block: skip ModifierBlock_Save.
            c.bus.write32(obj + 0x1c, 0);
            fill(c, &mut r, obj + 0x20, 4);
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = sx(stream);
        });
    }

    // ------------------------------------------------------ StreamArray_Queue

    #[test]
    fn stream_array_queue_matches() {
        check(ee::s08::f_002aad78__generated, stream_array_queue, 300, &|c, seed| {
            let mut r = Rng(0x002a_ad78 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.pc = 0x002a_ad78;
            let this = DATA;
            let array = DATA + 0x1000;
            let entry = DATA + 0x2000;
            let i = seed % 4;
            c.bus.write32(this + 8, array);
            c.bus
                .write32(array + 4 * i, if seed % 2 == 0 { 0 } else { entry });
            // Keep the queued buffer on StreamBuffer_Queue's short path.
            c.bus.write32(entry + 0x74, 0);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = i as u64;
            c.g[A2][0] = sx(DATA + 0x3000);
            c.g[A3][0] = sx(DATA + 0x4000);
            c.g[T0][0] = sx(DATA + 0x5000);
        });
    }

    // ------------------------------------------------ StreamBuffer_SetPitch

    #[test]
    fn stream_buffer_set_pitch_matches() {
        covered(
            ee::s08::f_002a9f38__generated,
            stream_buffer_set_pitch,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a9f, 1);
                c.pc = 0x002a_9f38;
                // No streams: the handle resolves to no stream and the pitch
                // setter takes its early return.
                snd_world(c, &mut r, false);
                let this = DATA;
                c.bus.write32(this + 8, 0xffff_ffff);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 5 == 0 {
                    r.edge_f32() as u64
                } else {
                    r.next_u32() as u64
                };
            },
        );
    }

    // ---------------------------------------------------- SND_DecAllocThunk

    #[test]
    fn snd_dec_alloc_thunk_matches() {
        check(ee::s14::f_003c7cc0__generated, snd_dec_alloc_thunk, 300, &|c, seed| {
            let mut r = Rng(0x003c_7cc0 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.pc = 0x003c_7cc0;
            // Zeroed heap: SND_MemAlloc clamps against a zero limit and
            // returns 0 without touching the arena.
            c.bus.write32(SND_STATE + 0x234, 0);
            c.g[A0][0] = (1 + r.next_u32() % 0xff) as u64;
        });
    }

    // ----------------------------------------------------- GridMeshBase_dtor

    #[test]
    fn grid_mesh_base_dtor_matches() {
        covered(ee::s12::f_00385260__generated, grid_mesh_base_dtor, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x3852, 8);
            c.pc = 0x0038_5260;
            // A small address counts as a pool block in the test machine
            // (every pool base is 0), so the delete takes the pool-free
            // path instead of walking an empty heap.
            let obj = 0x100 + 8 * (r.next_u32() % 0x80);
            let flags = if seed % 3 == 0 {
                r.next_u32() & !1
            } else {
                r.next_u32() | 1
            };
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = flags as u64;
        });
    }

    // ------------------------------------------------ SoundRequest_SetStream

    #[test]
    fn sound_request_set_stream_matches() {
        check(
            ee::s08::f_002abcf8__generated,
            sound_request_set_stream,
            300,
            &|c, seed| {
                let mut r = Rng(0x002a_bcf8 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x002a_bcf8;
                let req = DATA;
                let src = DATA + 0x1000;
                let n = 1 + r.next_u32() % 24;
                for k in 0..n {
                    c.bus.write8(src + k, 1 + (r.next_u32() % 0x7e) as u8);
                }
                for k in n..n + 48 {
                    c.bus.write8(src + k, 0);
                }
                c.g[A0][0] = sx(req);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = sx(src);
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    // -------------------------------------- RiderMetrix_GetRecoveryBoost

    #[test]
    fn rider_metrix_get_recovery_boost_matches() {
        check(
            ee::s00::f_00119bb0__generated,
            rider_metrix_get_recovery_boost,
            300,
            &|c, seed| {
                let mut r = Rng(0x0011_9bb0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0011_9bb0;
                let this = DATA;
                // No HUD table: ShowHUDNumber returns -1 without calling.
                c.bus.write32(this + 0x1b0, 0);
                c.bus.write32(
                    GAME_GP - 0x7a08,
                    if seed % 5 == 0 {
                        r.edge_f32()
                    } else {
                        r.next_u32()
                    },
                );
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
            },
        );
    }
}
