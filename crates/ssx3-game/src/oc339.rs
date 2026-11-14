//! Staging module oc339 (wave wave215): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc339`.

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
        0x003c_a5e8,
        snd_dec_eaxa_free_stream,
        "SND_DecEaxa_FreeStream",
    ),
    (0x0036_14d0, debounce_v01, "Debounce_v01"),
    (0x0036_0ab8, restore_node_dtor, "RestoreNode_dtor"),
    (
        0x003d_a3f0,
        spch_has_pending_sentence,
        "SPCH_HasPendingSentence",
    ),
    (0x003c_cf18, snd_eaxa_free, "SND_EaxaFree"),
    (
        0x002a_b1a0,
        speech_system_clear_prebuffer_callback,
        "SpeechSystem_ClearPrebufferCallback",
    ),
    (
        0x002b_1248,
        speech_system_clear_prebuffer,
        "SpeechSystem_ClearPrebuffer",
    ),
    (
        0x003c_e0e0,
        snd_ee_voice_get_master_channel,
        "SND_EeVoiceGetMasterChannel",
    ),
    (0x0035_2208, mesh_anim_v01, "MeshAnim_v01"),
    (0x003c_cef0, snd_eaxa_alloc, "SND_EaxaAlloc"),
];

// ---------------------------------------------------------------------------
// SND_DecEaxa_FreeStream (0x3ca5e8)

/// The EAXA decoder stream's buffer pointer.
pub(crate) const SND_DEC_EAXA_STREAM_BUFFER: u32 = 0x1c;

const SND_EAXA_FREE: Callee = Callee {
    addr: 0x003c_cf18,
    f: ee::s14::SND_EaxaFree,
};

/// `void SND_DecEaxa_FreeStream(DecStream* s)` (0x3ca5e8): hand the stream's
/// buffer (`+0x1c`) back to the sound library's allocator when it has one.
pub fn snd_dec_eaxa_free_stream(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(r32(c, A0).wrapping_add(SND_DEC_EAXA_STREAM_BUFFER));
        frame.restore_ra(c, 0);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            SND_EAXA_FREE.call(c, 0x003c_a604)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Debounce_v01 (0x3614d0)

/// The `Debounce` vtable (`D_004906f0`).
pub(crate) const DEBOUNCE_VTABLE_ADDR: u32 = 0x0049_06f0;

/// Object vtable pointer offset (all `WorldObj`-derived classes).
pub(crate) const OC339_OBJ_VTABLE: u32 = 0xc;

/// `Debounce::~Debounce(Debounce* self, int flags)` (0x3614d0): restore the
/// `Debounce` vtable, then run the `ModObj` destructor (which takes the
/// caller's flags in `a1`).
pub fn debounce_v01(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x0049_0000;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx(DEBOUNCE_VTABLE_ADDR);
        c.sw(r32(c, V1).wrapping_add(OC339_OBJ_VTABLE), c.g[V0][0]);
        call_ptr(c, 0x0035_53c0, 0x0036_14ec)?; // ModObj_dtor
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RestoreNode_dtor (0x360ab8)

/// The `RestoreNode` vtable (`D_00491680`).
pub(crate) const RESTORE_NODE_VTABLE_ADDR: u32 = 0x0049_1680;

/// `RestoreNode::~RestoreNode(RestoreNode* self, int flags)` (0x360ab8):
/// restore the `RestoreNode` vtable, then run the `WorldObj` destructor.
pub fn restore_node_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x0049_0000;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx(RESTORE_NODE_VTABLE_ADDR);
        c.sw(r32(c, V1).wrapping_add(OC339_OBJ_VTABLE), c.g[V0][0]);
        call_ptr(c, 0x0034_fbf0, 0x0036_0ad4)?; // WorldObj_dtor
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SPCH_HasPendingSentence (0x3da3f0)

/// The speech library's current sentence: `{owner, pending count}`.
pub(crate) const SPCH_CURRENT_SENTENCE: u32 = 0x0045_0700;

/// `bool SPCH_HasPendingSentence(int owner)` (0x3da3f0): whether the current
/// sentence belongs to `owner` and still has a positive pending count.
pub fn spch_has_pending_sentence(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = 0x0045_0000;
    c.g[A1][0] = 0;
    c.g[V0][0] = c.lw(SPCH_CURRENT_SENTENCE);
    c.g[A2][0] = sx(SPCH_CURRENT_SENTENCE);
    if c.g[V0][0] == c.g[A0][0] {
        c.cycles += 2;
        c.g[V0][0] = c.lw(SPCH_CURRENT_SENTENCE + 4);
        c.g[A1][0] = (0 < c.g[V0][0] as i64) as u64;
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A1][0];
    ret(c);
}

// ---------------------------------------------------------------------------
// SND_EaxaFree (0x3ccf18) / SND_EaxaAlloc (0x3ccef0)

/// The sound library's allocator hook (`D_00517604`).
pub(crate) const SND_EAXA_ALLOC_HOOK: u32 = 0x0051_7604;
/// The sound library's free hook (`D_00517608`).
pub(crate) const SND_EAXA_FREE_HOOK: u32 = 0x0051_7608;

/// `void SND_EaxaFree(void* p)` (0x3ccf18): forward to the free hook
/// installed at `0x517608`, arguments untouched.
pub fn snd_eaxa_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[V1][0] = 0x0051_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw(SND_EAXA_FREE_HOOK);
        frame.save_ra(c, 0);
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, 0x003c_cf30)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* SND_EaxaAlloc(int size)` (0x3ccef0): forward to the allocator hook
/// installed at `0x517604`, arguments untouched.
pub fn snd_eaxa_alloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[V1][0] = 0x0051_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw(SND_EAXA_ALLOC_HOOK);
        frame.save_ra(c, 0);
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, 0x003c_cf08)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SpeechSystem_ClearPrebufferCallback (0x2ab1a0) / SpeechSystem_ClearPrebuffer (0x2b1248)

/// The speech stream object's stream handle.
pub(crate) const SPEECH_STREAM_HANDLE: u32 = 0x1c;

/// `int SpeechSystem_ClearPrebufferCallback(void* unused, SpeechStream* s)`
/// (0x2ab1a0): set the stream's prebuffer to 0
/// (`SNDSTRM_SetPrebuffer(s->handle, 0)`) and return 1.
pub fn speech_system_clear_prebuffer_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(r32(c, A1).wrapping_add(SPEECH_STREAM_HANDLE));
        c.g[A1][0] = 0;
        call_ptr(c, 0x003b_7d80, 0x002a_b1b4)?; // SNDSTRM_SetPrebuffer
        frame.restore_ra(c, 0);
        c.g[V0][0] = 1;
        frame.close(c);
        Ok(())
    })
}

const SPEECH_CLEAR_PREBUFFER_CALLBACK: Callee = Callee {
    addr: 0x002a_b1a0,
    f: ee::s08::SpeechSystem_ClearPrebufferCallback,
};

/// `void SpeechSystem_ClearPrebuffer(SpeechSystem* self)` (0x2b1248): run the
/// clear-prebuffer callback with `a0 = *self` (`a1` passes through).
pub fn speech_system_clear_prebuffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(r32(c, A0));
        SPEECH_CLEAR_PREBUFFER_CALLBACK.call(c, 0x002b_1258)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_EeVoiceGetMasterChannel (0x3ce0e0)

/// The sound library's EE voice state (`0x510000 - 0x5718`).
pub(crate) const SND_EE_VOICE_STATE: u32 = 0x0050_a8e8;
/// Voice-table pointer in that state.
pub(crate) const SND_EE_VOICE_TABLE: u32 = 0x1f0;
/// Size of one voice record.
pub(crate) const SND_EE_VOICE_SIZE: u32 = 0x8c;
/// A voice's master channel (`short`, -1 = the voice is its own master).
pub(crate) const SND_EE_VOICE_MASTER: u32 = 0x28;

/// `int SND_EeVoiceGetMasterChannel(int voice)` (0x3ce0e0): offset the voice
/// by the state's two base indices (bytes `+0x30`, `+0x2f`) and return its
/// master channel, or the offset voice itself when the master is -1.
pub fn snd_ee_voice_get_master_channel(c: &mut Cpu) {
    c.cycles += 14;
    let state = SND_EE_VOICE_STATE;
    c.g[V0][0] = sx(state);
    c.g[A1][0] = SND_EE_VOICE_SIZE as u64;
    c.g[A2][0] = c.lbu(state.wrapping_add(0x30));
    c.g[A3][0] = c.lbu(state.wrapping_add(0x2f));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A2)));
    c.g[V1][0] = c.lw(state.wrapping_add(SND_EE_VOICE_TABLE));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A3)));
    c.mult(0, A1, A0, A1);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
    c.g[V0][0] = c.lh(r32(c, V1).wrapping_add(SND_EE_VOICE_MASTER));
    c.g[A1][0] = !c.g[V0][0];
    if c.g[A1][0] == 0 {
        c.g[V0][0] = c.g[A0][0];
    }
    ret(c);
}

// ---------------------------------------------------------------------------
// MeshAnim_v01 (0x352208)

/// The `MeshAnim` vtable (`0x490000 - 0x928`).
pub(crate) const MESH_ANIM_VTABLE_ADDR: u32 = 0x0048_f6d8;

const MESH_ANIM_BASE_DTOR: Callee = Callee {
    addr: 0x0034_fbf0,
    f: ee::s11::WorldObj_dtor,
};

/// `MeshAnim::~MeshAnim(MeshAnim* self, int flags)` (0x352208): restore the
/// `MeshAnim` vtable, then run the `WorldObj` destructor.
pub fn mesh_anim_v01(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x0049_0000;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx(MESH_ANIM_VTABLE_ADDR);
        c.sw(r32(c, V1).wrapping_add(OC339_OBJ_VTABLE), c.g[V0][0]);
        MESH_ANIM_BASE_DTOR.call(c, 0x0035_2224)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{check, check_lohi, fill, scramble, Rng, DATA, GAME_GP};

    /// A `WorldObj` at `obj` with an instance at `inst` (random flags,
    /// pointing back or not) and process-list neighbours or not, as
    /// `world_obj_dtor_matches` builds it. Flags in `a1` without the delete
    /// bit, so nothing is freed.
    fn oc339_world_obj(c: &mut Cpu, r: &mut Rng, seed: u32, obj: u32) {
        let inst = obj + 0x100;
        fill(c, r, obj, 8);
        c.bus.write16(obj + 0x10, (r.next_u32() % 8) as u16);
        if seed % 3 == 0 {
            c.bus.write32(obj + 4, 0);
        } else {
            c.bus.write32(obj + 4, obj + 0x200);
            c.bus
                .write32(obj + 8, if seed % 3 == 1 { 0 } else { obj + 0x300 });
        }
        c.bus
            .write32(obj + 0x18, if seed % 7 == 0 { 0 } else { inst });
        c.bus.write32(inst + 0x8, r.next_u32());
        c.bus
            .write32(inst + 0xc, if seed % 2 == 0 { obj } else { r.next_u32() });
        c.g[A0][0] = obj as u64;
        c.g[A1][0] = (r.next_u32() & !1) as u64;
    }

    /// `SND_DecEaxa_FreeStream`: a stream with no buffer (no call) or one
    /// the real `SND_EaxaFree` hands to the fake free hook.
    #[test]
    fn snd_dec_eaxa_free_stream_matches() {
        covered(
            ee::s14::f_003ca5e8__generated,
            snd_dec_eaxa_free_stream,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c_a5e8, 1);
                c.pc = 0x003c_a5e8;
                c.bus.write32(SND_EAXA_FREE_HOOK, FAKE);
                let buf = if seed % 3 == 0 { 0 } else { r.next_u32() };
                c.bus.write32(DATA + SND_DEC_EAXA_STREAM_BUFFER, buf);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    /// `Debounce_v01`: a `ModObj` without a modifier block (the fixture of
    /// `mod_obj_dtor_matches`); the real `ModObj`/`WorldObj` destructors run.
    #[test]
    fn debounce_v01_matches() {
        check(
            ee::s12::f_003614d0__generated,
            debounce_v01,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x36_14d0, 0);
                c.pc = 0x0036_14d0;
                let obj = DATA + 0x1000;
                oc339_world_obj(c, &mut r, seed, obj);
                c.bus.write32(obj + 0x1c, 0); // no modifier block
            },
        );
    }

    /// `RestoreNode_dtor`: the `world_obj_dtor_matches` fixture; the real
    /// `WorldObj` destructor runs.
    #[test]
    fn restore_node_dtor_matches() {
        check(
            ee::s12::f_00360ab8__generated,
            restore_node_dtor,
            300,
            &|c, seed| {
                let mut r = Rng(0x36_0ab8 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0036_0ab8;
                c.g[GP][0] = GAME_GP as u64;
                oc339_world_obj(c, &mut r, seed, DATA);
            },
        );
    }

    /// `SPCH_HasPendingSentence`: the owner matches (sign-extended word) or
    /// not; the pending count is negative, zero or positive.
    #[test]
    fn spch_has_pending_sentence_matches() {
        check(
            ee::s14::f_003da3f0__generated,
            spch_has_pending_sentence,
            300,
            &|c, seed| {
                let mut r = Rng(0x3d_a3f0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x003d_a3f0;
                let owner = r.next_u32();
                c.bus.write32(SPCH_CURRENT_SENTENCE, owner);
                let count = match seed % 3 {
                    0 => 0,
                    1 => r.next_u32() | 0x8000_0000,
                    _ => (r.next_u32() >> 1).max(1),
                };
                c.bus.write32(SPCH_CURRENT_SENTENCE + 4, count);
                c.g[A0][0] = match seed % 4 {
                    0 => r.next_u32() as u64,
                    // Same low word, upper half not the sign extension.
                    1 => owner as u64 | 0x1_0000_0000,
                    _ => sx(owner),
                };
            },
        );
    }

    /// `SND_EaxaFree`: the free hook is a fake that must be called.
    #[test]
    fn snd_eaxa_free_matches() {
        covered(
            ee::s14::f_003ccf18__generated,
            snd_eaxa_free,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c_cf18, 1);
                c.pc = 0x003c_cf18;
                c.bus.write32(SND_EAXA_FREE_HOOK, FAKE + 0x10 * (seed % 4));
                c.g[A0][0] = r.next_u32() as u64;
            },
        );
    }

    /// `SND_EaxaAlloc`: the allocator hook is a fake that must be called.
    #[test]
    fn snd_eaxa_alloc_matches() {
        covered(
            ee::s14::f_003ccef0__generated,
            snd_eaxa_alloc,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c_cef0, 1);
                c.pc = 0x003c_cef0;
                c.bus.write32(SND_EAXA_ALLOC_HOOK, FAKE + 0x10 * (seed % 4));
                c.g[A0][0] = r.next_u32() as u64;
            },
        );
    }

    /// A speech stream at `DATA + 0x100` whose handle (0 or -1) has a
    /// request or not in `snd_world` (fake lock/unlock hooks).
    fn oc339_speech_stream(c: &mut Cpu, r: &mut Rng, seed: u32) -> u32 {
        use crate::game::lib::snd::tests::snd_world;
        let found = seed % 2 == 0;
        snd_world(c, r, found);
        let stream = DATA + 0x100;
        c.bus.write32(
            stream + SPEECH_STREAM_HANDLE,
            if found { 0 } else { 0xffff_ffff },
        );
        stream
    }

    /// `SpeechSystem_ClearPrebufferCallback`: the real
    /// `SNDSTRM_SetPrebuffer` with a request found or not.
    #[test]
    fn speech_system_clear_prebuffer_callback_matches() {
        covered(
            ee::s08::f_002ab1a0__generated,
            speech_system_clear_prebuffer_callback,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_b1a0, 2);
                c.pc = 0x002a_b1a0;
                let stream = oc339_speech_stream(c, &mut r, seed);
                c.g[A1][0] = stream as u64;
            },
        );
    }

    /// `SpeechSystem_ClearPrebuffer`: `*self` is random, `a1` the stream the
    /// callback reads.
    #[test]
    fn speech_system_clear_prebuffer_matches() {
        covered(
            ee::s08::f_002b1248__generated,
            speech_system_clear_prebuffer,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b_1248, 2);
                c.pc = 0x002b_1248;
                let stream = oc339_speech_stream(c, &mut r, seed);
                c.bus.write32(DATA + 0x200, r.next_u32());
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.g[A1][0] = stream as u64;
            },
        );
    }

    /// `SND_EeVoiceGetMasterChannel`: small base indices and voice; the
    /// master is -1 (returns the voice) or a random short.
    #[test]
    fn snd_ee_voice_get_master_channel_matches() {
        check_lohi(
            ee::s14::f_003ce0e0__generated,
            snd_ee_voice_get_master_channel,
            300,
            &|c, seed| {
                let mut r = Rng(0x3c_e0e0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x003c_e0e0;
                let b30 = r.next_u32() % 8;
                let b2f = r.next_u32() % 8;
                let voice = r.next_u32() % 16;
                c.bus.write8(SND_EE_VOICE_STATE + 0x30, b30 as u8);
                c.bus.write8(SND_EE_VOICE_STATE + 0x2f, b2f as u8);
                c.bus.write32(SND_EE_VOICE_STATE + SND_EE_VOICE_TABLE, DATA);
                let idx = voice + b30 + b2f;
                let master = if seed % 3 == 0 {
                    0xffff
                } else {
                    r.next_u32() as u16
                };
                c.bus
                    .write16(DATA + idx * SND_EE_VOICE_SIZE + SND_EE_VOICE_MASTER, master);
                // Upper half of a0 is junk: the adds are 32-bit.
                c.g[A0][0] = (c.g[A0][0] & !0xffff_ffff) | voice as u64;
            },
        );
    }

    /// `MeshAnim_v01`: the `world_obj_dtor_matches` fixture; the real
    /// `WorldObj` destructor runs.
    #[test]
    fn mesh_anim_v01_matches() {
        check(
            ee::s11::f_00352208__generated,
            mesh_anim_v01,
            300,
            &|c, seed| {
                let mut r = Rng(0x35_2208 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0035_2208;
                c.g[GP][0] = GAME_GP as u64;
                oc339_world_obj(c, &mut r, seed, DATA);
            },
        );
    }
}
