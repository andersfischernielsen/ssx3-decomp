//! Staging module oc338 (wave wave215): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc338`.

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
        0x0035_5e38,
        mod_obj_add_rail_modifier,
        "ModObj_AddRailModifier",
    ),
    (
        0x0036_a190,
        graphics_man_destroy_grid_mesh,
        "GraphicsMan_DestroyGridMesh",
    ),
    (
        0x0035_2f08,
        modifier_block_clear_transform_finished,
        "ModifierBlock_ClearTransformFinished",
    ),
    (0x0036_1b18, spline_modifier_v01, "SplineModifier_v01"),
    (
        0x002a_7408,
        crowd_audio_start_anticipate,
        "CrowdAudio_StartAnticipate",
    ),
    (
        0x003c_6580,
        snd_channel_find_max_voice,
        "SND_ChannelFindMaxVoice",
    ),
    (
        0x0035_5eb0,
        mod_obj_add_halo_modifier,
        "ModObj_AddHaloModifier",
    ),
    (
        0x0028_5f80,
        ssx_audio_system_stop_by_callback,
        "SSXAudioSystem_StopByCallback",
    ),
    (
        0x002a_1d48,
        ssx_audio_system_get_rider_voice_mask,
        "SSXAudioSystem_GetRiderVoiceMask",
    ),
    (0x003d_b3b8, spch_match_flags, "SPCH_MatchFlags"),
];

// ------------------------------------------------- ModObj_AddRailModifier

/// `cMemMan_alloc` (0x317d70, another shard: through the function table).
pub(crate) const OC338_MEM_MAN_ALLOC: u32 = 0x0031_7d70;
/// `operator delete` (0x317e50, another shard).
pub(crate) const OC338_OPERATOR_DELETE: u32 = 0x0031_7e50;

/// `ModObj::modifierBlock` (+0x1c): the object's modifier block (may be null).
pub(crate) const MOD_OBJ_ADD_RAIL_BLOCK: u32 = 0x1c;
/// `ModObj` +0x18: the owner passed to the modifier constructors.
pub(crate) const MOD_OBJ_ADD_RAIL_OWNER: u32 = 0x18;
/// `tAddRailData` +4: the rail id looked up in the block.
pub(crate) const MOD_OBJ_ADD_RAIL_ID: u32 = 0x4;

const MOD_OBJ_HAS_RAIL_MODIFIER: Callee = Callee {
    addr: 0x0035_3418,
    f: ee::s11::ModifierBlock_HasRailModifier,
};
const MOD_OBJ_RAIL_MODIFIER_CTOR: Callee = Callee {
    addr: 0x0035_b708,
    f: ee::s11::RailModifier_ctor,
};
const MOD_OBJ_SET_RAIL_MODIFIER: Callee = Callee {
    addr: 0x0035_56a8,
    f: ee::s11::ModObj_SetRailModifier,
};

/// `void ModObj_AddRailModifier(ModObj* o, const tAddRailData* desc)`
/// (0x00355e38, `cMoveNode::addRail`): unless the object's modifier block
/// already has a rail modifier for `desc->id`, allocate a 0x90-byte rail
/// modifier (`D_0048E9B8` tag), construct it from the descriptor and install
/// it on the object.
pub fn mod_obj_add_rail_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(MOD_OBJ_ADD_RAIL_BLOCK)).lw(c);
        c.g[S1][0] = c.g[A1][0];
        let mut present = false;
        if c.g[A0][0] != 0 {
            c.cycles += 4;
            c.g[A1][0] = Addr(r32(c, S1).wrapping_add(MOD_OBJ_ADD_RAIL_ID)).lw(c);
            MOD_OBJ_HAS_RAIL_MODIFIER.call(c, 0x0035_5e60)?;
            present = c.g[V0][0] != 0;
        }
        if !present {
            c.cycles += 14;
            c.g[A0][0] = 0x90;
            c.g[A1][0] = sx(0x0048_e9b8); // D_0048E9B8
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, OC338_MEM_MAN_ALLOC, 0x0035_5e80)?;
            c.g[A2][0] = Addr(r32(c, S0).wrapping_add(MOD_OBJ_ADD_RAIL_OWNER)).lw(c);
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            MOD_OBJ_RAIL_MODIFIER_CTOR.call(c, 0x0035_5e90)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            MOD_OBJ_SET_RAIL_MODIFIER.call(c, 0x0035_5e9c)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// -------------------------------------------- GraphicsMan_DestroyGridMesh

const GRAPHICS_MAN_DEFERRED_DELETE_PUSH: Callee = Callee {
    addr: 0x0036_a1c0,
    f: ee::s12::DeferredDelete_Push,
};

/// `void GraphicsMan_DestroyGridMesh(GraphicsMan* gm, GridMesh* mesh)`
/// (0x0036a190): hand the mesh to the deferred-delete queue.
pub fn graphics_man_destroy_grid_mesh(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        GRAPHICS_MAN_DEFERRED_DELETE_PUSH.call(c, 0x0036_a1a0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ----------------------------------- ModifierBlock_ClearTransformFinished

/// Vtable entry 7 (`{delta, fn}` at +0x38): the transform modifier's
/// "clear finished" method.
pub(crate) const MODIFIER_BLOCK_CLEAR_FINISHED_SLOT: u32 = 0x38;

/// `void ModifierBlock_ClearTransformFinished(tModifierBlock* b)`
/// (0x00352f08, `tModifierBlock::clearComplete`): if the block has a
/// transform modifier (+0), call its virtual "clear finished" method.
pub fn modifier_block_clear_transform_finished(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(r32(c, A0)).lw(c);
        frame.restore_ra(c, 0);
        if c.g[A1][0] != 0 {
            c.cycles += 6;
            let modifier = r32(c, A1);
            c.g[V0][0] = Addr(modifier).lw(c);
            let vt = r32(c, V0);
            c.g[A0][0] = Addr(vt.wrapping_add(MODIFIER_BLOCK_CLEAR_FINISHED_SLOT)).lh(c);
            c.g[V1][0] = Addr(vt.wrapping_add(MODIFIER_BLOCK_CLEAR_FINISHED_SLOT + 4)).lw(c);
            c.g[A0][0] = sx(modifier.wrapping_add(r32(c, A0)));
            let method = r32(c, V1);
            call_ptr(c, method, 0x0035_2f30)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------- SplineModifier_v01

/// `cSplineModifier`'s base vtable (`D_004913F8`).
pub(crate) const SPLINE_MODIFIER_V01_VTABLE: u32 = 0x0049_13f8;

/// `void SplineModifier_v01(cSplineModifier* m, int flags)` (0x00361b18,
/// `cSplineModifier::~cSplineModifier`): reset the vtable to the base one
/// and free the object when bit 0 of `flags` asks for it.
pub fn spline_modifier_v01(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[V0][0] = sx(SPLINE_MODIFIER_V01_VTABLE);
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] &= 1;
        Addr(r32(c, A0)).sw(c, c.g[V0][0]);
        if c.g[A1][0] != 0 {
            c.cycles += 2;
            call_ptr(c, OC338_OPERATOR_DELETE, 0x0036_1b3c)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// --------------------------------------------- CrowdAudio_StartAnticipate

/// `CrowdAudio` +0x18: the array of 0x190-byte crowd instances.
pub(crate) const CROWD_AUDIO_INSTANCES: u32 = 0x18;
/// Size of one crowd instance.
pub(crate) const CROWD_AUDIO_INSTANCE_SIZE: u64 = 0x190;

const CROWD_AUDIO_INSTANCE_START_ANTICIPATE: Callee = Callee {
    addr: 0x002a_67f0,
    f: ee::s08::CrowdAudio_InstanceStartAnticipate,
};

/// `void CrowdAudio_StartAnticipate(CrowdAudio* ca, int i)` (0x002a7408):
/// start the anticipation sound of crowd instance `i`, if the instances
/// exist.
pub fn crowd_audio_start_anticipate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(CROWD_AUDIO_INSTANCES)).lw(c);
        frame.restore_ra(c, 0);
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            c.g[A0][0] = CROWD_AUDIO_INSTANCE_SIZE;
            c.mult(0, V1, A1, A0);
            c.g[A0][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            CROWD_AUDIO_INSTANCE_START_ANTICIPATE.call(c, 0x002a_742c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------ SND_ChannelFindMaxVoice

/// `D_0050AAD8`: pointer to the channel table (0x8c bytes per channel; the
/// voice priorities are halfwords from +4).
pub(crate) const SND_CHANNEL_FIND_MAX_VOICE_TABLE: u32 = 0x0050_aad8;

/// `int SND_ChannelFindMaxVoice(int channel, int count)` (0x003c6580): the
/// index of the voice with the highest priority among the channel's first
/// `count` voices (the first on ties), or -1 if there are none.
pub fn snd_channel_find_max_voice(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = c.g[A0][0];
    c.g[T0][0] = u64::MAX; // best priority
    c.g[T1][0] = u64::MAX; // its voice
    c.g[A3][0] = 0; // voice index
    if (c.g[A1][0] as i64) > 0 {
        c.cycles += 7;
        c.g[V1][0] = 0x8c;
        c.g[A0][0] = 0x0051_0000;
        c.mult(0, V1, V0, V1);
        c.g[V0][0] = Addr(SND_CHANNEL_FIND_MAX_VOICE_TABLE).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
        // v1: the current priority halfword.
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        loop {
            c.cycles += 4;
            c.g[A2][0] = Addr(r32(c, V1)).lh(c);
            let higher = (c.g[T0][0] as i64) < (c.g[A2][0] as i64);
            c.g[V0][0] = higher as u64;
            if higher {
                c.cycles += 3;
                c.g[T0][0] = c.g[A2][0];
                c.g[T1][0] = c.g[A3][0];
            }
            c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
            c.cycles += 3;
            let more = (c.g[A3][0] as i64) < (c.g[A1][0] as i64);
            c.g[V0][0] = more as u64;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(2));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_65b0);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[T1][0];
    ret(c);
}

// ------------------------------------------------- ModObj_AddHaloModifier

const MOD_OBJ_HALO_MODIFIER_CTOR: Callee = Callee {
    addr: 0x0034_6120,
    f: ee::s11::HaloModifier_ctor,
};
const MOD_OBJ_LINK_PARTICLE_MODIFIER: Callee = Callee {
    addr: 0x0035_5658,
    f: ee::s11::ModObj_LinkParticleModifier,
};

/// `void ModObj_AddHaloModifier(ModObj* o, const tAddHaloData* desc)`
/// (0x00355eb0, `cMoveNode::addHalo`): allocate a 0x40-byte halo modifier
/// (`D_0048E9A8` tag), construct it from the descriptor (owner `o+0x18`)
/// and link it into the object's particle (effect) modifiers.
pub fn mod_obj_add_halo_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x10);
        c.g[A2][0] = 0x2000_0000;
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A1][0];
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x40;
        c.g[A1][0] = sx(0x0048_e9a8); // D_0048E9A8
        call_ptr(c, OC338_MEM_MAN_ALLOC, 0x0035_5ee0)?;
        c.g[A2][0] = Addr(r32(c, S0).wrapping_add(MOD_OBJ_ADD_RAIL_OWNER)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        MOD_OBJ_HALO_MODIFIER_CTOR.call(c, 0x0035_5ef0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        MOD_OBJ_LINK_PARTICLE_MODIFIER.call(c, 0x0035_5efc)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------ SSXAudioSystem_StopByCallback

/// `SSXAudioSystem` +0x118: pointer to the bank-monitor holder pointer.
pub(crate) const SSX_AUDIO_STOP_BY_CALLBACK_HOLDER: u32 = 0x118;
/// Offset of the bank monitor inside the holder's object.
pub(crate) const SSX_AUDIO_STOP_BY_CALLBACK_MONITOR: u32 = 0x1d8;

/// `void SSXAudioSystem_StopByCallback(SSXAudioSystem* s, Pair p)`
/// (0x00285f80): forward the (64-bit) callback pair to the bank monitor
/// `(**(s+0x118)) + 0x1d8` (0x2ad410), stopping the sounds it matches.
pub fn ssx_audio_system_stop_by_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        // The pair is passed through a stack slot (`sd a1; ld a1`).
        let pair = frame.at(c, 0);
        c.sd(pair, c.g[A1][0]);
        c.g[A1][0] = c.ld(pair);
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(SSX_AUDIO_STOP_BY_CALLBACK_HOLDER)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).lw(c);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(SSX_AUDIO_STOP_BY_CALLBACK_MONITOR));
        call_ptr(c, 0x002a_d410, 0x0028_5fa0)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// --------------------------------------- SSXAudioSystem_GetRiderVoiceMask

const SSX_AUDIO_CHARACTER_TO_VOICE_MASK: Callee = Callee {
    addr: 0x002a_1da0,
    f: ee::s08::SSXAudioSystem_CharacterToVoiceMask,
};

/// `u32 SSXAudioSystem_GetRiderVoiceMask(SSXAudioSystem* s, Rider* r)`
/// (0x002a1d48): the voice mask for the rider's character, from the two
/// rider queries 0x123128 and 0x123168.
pub fn ssx_audio_system_get_rider_voice_mask(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0012_3128, 0x002a_1d6c)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0012_3168, 0x002a_1d78)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[V0][0];
        SSX_AUDIO_CHARACTER_TO_VOICE_MASK.call(c, 0x002a_1d88)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// -------------------------------------------------------- SPCH_MatchFlags

/// `bool SPCH_MatchFlags(u8 want, u8 ignore, const u8* ev)` (0x003db3b8):
/// whether the event's flag byte `ev[3]` equals `want` on every bit of the
/// mask byte `ev[2]` that `ignore` does not clear.
pub fn spch_match_flags(c: &mut Cpu) {
    c.cycles += 10;
    c.g[A1][0] &= 0xff;
    let ev = r32(c, A2);
    c.g[V0][0] = Addr(ev.wrapping_add(3)).lbu(c);
    c.g[V1][0] = Addr(ev.wrapping_add(2)).lbu(c);
    c.g[A0][0] &= 0xff;
    c.g[A1][0] = !c.g[A1][0];
    c.g[V0][0] ^= c.g[A0][0];
    c.g[V1][0] &= c.g[A1][0];
    c.g[V0][0] &= c.g[V1][0];
    c.g[V0][0] = (c.g[V0][0] == 0) as u64;
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes::{
        covered, fake, fall_back, hook, machine, FAKE, LOG_RESULTS, LOG_UNWIND_AT,
    };
    use crate::difftest::{check, fill, scramble, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Magic word that turns this module's gated fakes on, in each test
    /// machine's own RAM (other tests reach the bodies the slots held).
    pub(crate) const OC338_CTRL: u32 = DATA + 0x0e_3380;
    pub(crate) const OC338_MAGIC: u32 = 0x4f43_3338;

    /// A gated logging fake for an override slot: logs the caller's
    /// registers and returns a pointer (or unwinds at the setup's index).
    macro_rules! oc338_fake {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC338_CTRL) != OC338_MAGIC {
                    return fall_back(c, &$prev, $gen);
                }
                let ra = c.g[RA][0] as u32;
                fake(c, $addr, ra);
            }
        };
    }

    oc338_fake!(
        oc338_fake_alloc,
        OC338_PREV_ALLOC,
        0x0031_7d70,
        ee::s10::f_00317d70__generated
    );
    oc338_fake!(
        oc338_fake_set_rail,
        OC338_PREV_SET_RAIL,
        0x0035_56a8,
        ee::s11::f_003556a8__generated
    );
    oc338_fake!(
        oc338_fake_deferred_push,
        OC338_PREV_DEFERRED_PUSH,
        0x0036_a1c0,
        ee::s12::f_0036a1c0__generated
    );
    oc338_fake!(
        oc338_fake_delete,
        OC338_PREV_DELETE,
        0x0031_7e50,
        ee::s10::f_00317e50__generated
    );
    oc338_fake!(
        oc338_fake_crowd_anticipate,
        OC338_PREV_CROWD_ANTICIPATE,
        0x002a_67f0,
        ee::s08::f_002a67f0__generated
    );
    oc338_fake!(
        oc338_fake_particle_base_ctor,
        OC338_PREV_PARTICLE_BASE_CTOR,
        0x0034_5798,
        ee::s11::f_00345798__generated
    );
    oc338_fake!(
        oc338_fake_link_particle,
        OC338_PREV_LINK_PARTICLE,
        0x0035_5658,
        ee::s11::f_00355658__generated
    );
    oc338_fake!(
        oc338_fake_bank_monitor_stop,
        OC338_PREV_BANK_MONITOR_STOP,
        0x002a_d410,
        ee::s08::f_002ad410__generated
    );
    oc338_fake!(
        oc338_fake_rider_query_a,
        OC338_PREV_RIDER_QUERY_A,
        0x0012_3128,
        ee::s00::f_00123128__generated
    );
    oc338_fake!(
        oc338_fake_rider_query_b,
        OC338_PREV_RIDER_QUERY_B,
        0x0012_3168,
        ee::s00::f_00123168__generated
    );
    oc338_fake!(
        oc338_fake_voice_mask,
        OC338_PREV_VOICE_MASK,
        0x002a_1da0,
        ee::s08::f_002a1da0__generated
    );

    static OC338_PREV_HAS_RAIL: AtomicUsize = AtomicUsize::new(0);

    /// `ModifierBlock_HasRailModifier`: logs, then answers the word at
    /// `OC338_CTRL + 4` (0 = no rail modifier yet).
    fn oc338_fake_has_rail(c: &mut Cpu) {
        if c.bus.read32(OC338_CTRL) != OC338_MAGIC {
            return fall_back(c, &OC338_PREV_HAS_RAIL, ee::s11::f_00353418__generated);
        }
        let ra = c.g[RA][0] as u32;
        fake(c, 0x0035_3418, ra);
        if c.pc == ra {
            c.g[V0][0] = c.bus.read32(OC338_CTRL + 4) as u64;
        }
    }

    /// Where the fake allocator's block goes (away from the fakes' hashed
    /// pointers, which stay below `DATA + 0x40000`).
    const OC338_BLOCK: u32 = DATA + 0x4_8000;

    /// `ModObj_AddRailModifier`: no block, a block that already has the
    /// rail (early exit), and one without it. The real rail constructor
    /// runs up to its first virtual call (a logging fake through a fake
    /// vtable), where the seed always unwinds (past it the constructor
    /// builds rail nodes); so `ModObj_SetRailModifier` is not reached.
    #[test]
    fn mod_obj_add_rail_modifier_matches() {
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_00353418),
            oc338_fake_has_rail,
            &OC338_PREV_HAS_RAIL,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc338_fake_alloc,
            &OC338_PREV_ALLOC,
        );
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_003556A8),
            oc338_fake_set_rail,
            &OC338_PREV_SET_RAIL,
        );
        const OBJ: u32 = DATA + 0x5_0000;
        const DESC: u32 = DATA + 0x5_1000;
        const OWNER: u32 = DATA + 0x5_2000;
        const OWNER_INNER: u32 = DATA + 0x5_3000;
        const OWNER_VT: u32 = DATA + 0x5_4000;
        covered(
            ee::s11::f_00355e38__generated,
            mod_obj_add_rail_modifier,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0035_5e38, 3);
                c.pc = 0x0035_5e38;
                c.bus.write32(OC338_CTRL, OC338_MAGIC);
                let has_block = seed % 3 != 0;
                let present = has_block && seed % 3 == 1 && r.next_u32() % 2 == 0;
                c.bus.write32(
                    OC338_CTRL + 4,
                    if present { 1 + r.next_u32() % 7 } else { 0 },
                );
                fill(c, &mut r, OBJ, 0x10);
                fill(c, &mut r, DESC, 0x10);
                c.bus.write32(
                    OBJ + MOD_OBJ_ADD_RAIL_BLOCK,
                    if has_block { DATA + 0x5_8000 } else { 0 },
                );
                // Owner chain read by the rail constructor before its
                // first virtual call: owner+0xc -> +0xc -> vtable+0x160.
                c.bus.write32(OBJ + MOD_OBJ_ADD_RAIL_OWNER, OWNER);
                c.bus.write32(OWNER + 0xc, OWNER_INNER);
                c.bus.write32(OWNER_INNER + 0xc, OWNER_VT);
                c.bus
                    .write16(OWNER_VT + 0x160, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(OWNER_VT + 0x164, FAKE);
                let alloc_index = has_block as u32;
                c.bus.write32(LOG_RESULTS + 4 * alloc_index, OC338_BLOCK);
                if !present {
                    // Always unwind by the constructor's virtual call.
                    let ctor_call = alloc_index + 1;
                    c.bus.write32(LOG_UNWIND_AT, r.next_u32() % (ctor_call + 1));
                }
                c.g[A0][0] = sx(OBJ);
                c.g[A1][0] = sx(DESC);
            },
        );
    }

    /// `GraphicsMan_DestroyGridMesh`: the mesh goes to a logging fake of
    /// `DeferredDelete_Push`.
    #[test]
    fn graphics_man_destroy_grid_mesh_matches() {
        hook(
            addr_of_mut!(ee::s12::OVERRIDE_0036A1C0),
            oc338_fake_deferred_push,
            &OC338_PREV_DEFERRED_PUSH,
        );
        covered(
            ee::s12::f_0036a190__generated,
            graphics_man_destroy_grid_mesh,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0036_a190, 1);
                c.pc = 0x0036_a190;
                c.bus.write32(OC338_CTRL, OC338_MAGIC);
                c.g[A0][0] = sx(DATA + 0x5_0000);
                c.g[A1][0] = sx(DATA + 0x5_1000 + (r.next_u32() & 0xff0));
            },
        );
    }

    /// `ModifierBlock_ClearTransformFinished`: no transform modifier, or one
    /// whose method (a fake through a fake vtable) is called on the
    /// adjusted `this`.
    #[test]
    fn modifier_block_clear_transform_finished_matches() {
        covered(
            ee::s11::f_00352f08__generated,
            modifier_block_clear_transform_finished,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0035_2f08, 1);
                c.pc = 0x0035_2f08;
                let block = DATA + 0x5_0000;
                let modifier = DATA + 0x5_1000;
                let vt = DATA + 0x5_2000;
                let delta = (r.next_u32() as i16 as i32 & !3) as u32;
                c.bus
                    .write16(vt + MODIFIER_BLOCK_CLEAR_FINISHED_SLOT, delta as u16);
                c.bus
                    .write32(vt + MODIFIER_BLOCK_CLEAR_FINISHED_SLOT + 4, FAKE);
                c.bus.write32(modifier, vt);
                c.bus
                    .write32(block, if seed % 4 == 0 { 0 } else { modifier });
                c.g[A0][0] = sx(block);
            },
        );
    }

    /// `SplineModifier_v01`: the vtable reset, with and without the delete
    /// flag (a logging fake of `operator delete`).
    #[test]
    fn spline_modifier_v01_matches() {
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc338_fake_delete,
            &OC338_PREV_DELETE,
        );
        covered(
            ee::s12::f_00361b18__generated,
            spline_modifier_v01,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0036_1b18, 1);
                c.pc = 0x0036_1b18;
                c.bus.write32(OC338_CTRL, OC338_MAGIC);
                let this = DATA + 0x5_0000 + (r.next_u32() & 0xff0);
                fill(c, &mut r, this, 4);
                c.g[A0][0] = sx(this);
                // Random flags (bit 0 set or clear, junk above).
            },
        );
    }

    /// `CrowdAudio_StartAnticipate`: no instances, or instance `i` passed
    /// to a logging fake of `CrowdAudio_InstanceStartAnticipate`.
    #[test]
    fn crowd_audio_start_anticipate_matches() {
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002A67F0),
            oc338_fake_crowd_anticipate,
            &OC338_PREV_CROWD_ANTICIPATE,
        );
        covered(
            ee::s08::f_002a7408__generated,
            crowd_audio_start_anticipate,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002a_7408, 1);
                c.pc = 0x002a_7408;
                c.bus.write32(OC338_CTRL, OC338_MAGIC);
                let this = DATA + 0x5_0000;
                let instances = if seed % 4 == 0 { 0 } else { DATA + 0x6_0000 };
                c.bus.write32(this + CROWD_AUDIO_INSTANCES, instances);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 5 == 0 {
                    r.next_u32() as u64 | 0xdead_0000_0000_0000
                } else {
                    (r.next_u32() % 16) as u64
                };
            },
        );
    }

    driven!(oc338_gen_find_max_voice, ee::s13::f_003c6580__generated);
    driven!(oc338_hand_find_max_voice, snd_channel_find_max_voice);

    /// `SND_ChannelFindMaxVoice`: counts <= 0 (no loop) and 1..40 voices of
    /// random priorities (ties, negatives), with events delivered and
    /// unwinding at the loop's back edge.
    #[test]
    fn snd_channel_find_max_voice_matches() {
        check_events(
            oc338_gen_find_max_voice,
            oc338_hand_find_max_voice,
            300,
            &|c, seed| {
                let mut r = Rng(0x003c_6580 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                let table = DATA + 0x5_0000;
                c.bus.write32(SND_CHANNEL_FIND_MAX_VOICE_TABLE, table);
                let channel = r.next_u32() % 4;
                for i in 0..48 {
                    let p = match r.next_u32() % 4 {
                        0 => 0xffff,
                        1 => 7,
                        _ => r.next_u32() & 0xffff,
                    };
                    c.bus.write16(table + 4 + channel * 0x8c + 2 * i, p as u16);
                }
                c.g[A0][0] = channel as u64;
                c.g[A1][0] = match seed % 7 {
                    0 => 0,
                    1 => sx(0x8000_0000 | r.next_u32()),
                    _ => (1 + r.next_u32() % 40) as u64,
                };
                schedule(c, &mut r, seed, 0x003c_6580)
            },
        );
    }

    /// `ModObj_AddHaloModifier`: the allocator, the particle base
    /// constructor and the particle-modifier link are logging fakes; the
    /// real halo constructor copies the descriptor and bumps the counter
    /// at `gp+0xf10`.
    #[test]
    fn mod_obj_add_halo_modifier_matches() {
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc338_fake_alloc,
            &OC338_PREV_ALLOC,
        );
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_00345798),
            oc338_fake_particle_base_ctor,
            &OC338_PREV_PARTICLE_BASE_CTOR,
        );
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_00355658),
            oc338_fake_link_particle,
            &OC338_PREV_LINK_PARTICLE,
        );
        const OBJ: u32 = DATA + 0x5_0000;
        const DESC: u32 = DATA + 0x5_1000;
        covered(
            ee::s11::f_00355eb0__generated,
            mod_obj_add_halo_modifier,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0035_5eb0, 3);
                c.pc = 0x0035_5eb0;
                c.bus.write32(OC338_CTRL, OC338_MAGIC);
                c.bus.write32(LOG_RESULTS, OC338_BLOCK);
                fill(c, &mut r, OBJ, 0x10);
                fill(c, &mut r, DESC, 0x10);
                c.bus.write32(GAME_GP + 0xf10, r.next_u32());
                c.g[A0][0] = sx(OBJ);
                c.g[A1][0] = sx(DESC);
            },
        );
    }

    /// `SSXAudioSystem_StopByCallback`: the bank monitor at
    /// `**(s+0x118) + 0x1d8` gets the 64-bit pair (a logging fake).
    #[test]
    fn ssx_audio_system_stop_by_callback_matches() {
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002AD410),
            oc338_fake_bank_monitor_stop,
            &OC338_PREV_BANK_MONITOR_STOP,
        );
        covered(
            ee::s07::f_00285f80__generated,
            ssx_audio_system_stop_by_callback,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0028_5f80, 1);
                c.pc = 0x0028_5f80;
                c.bus.write32(OC338_CTRL, OC338_MAGIC);
                let this = DATA + 0x5_0000;
                let holder = DATA + 0x5_1000;
                c.bus
                    .write32(this + SSX_AUDIO_STOP_BY_CALLBACK_HOLDER, holder);
                c.bus
                    .write32(holder, DATA + 0x5_2000 + (r.next_u32() & 0xff0));
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `SSXAudioSystem_GetRiderVoiceMask`: the two rider queries and the
    /// mask lookup are logging fakes (their results flow into the next
    /// call's arguments).
    #[test]
    fn ssx_audio_system_get_rider_voice_mask_matches() {
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_00123128),
            oc338_fake_rider_query_a,
            &OC338_PREV_RIDER_QUERY_A,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_00123168),
            oc338_fake_rider_query_b,
            &OC338_PREV_RIDER_QUERY_B,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002A1DA0),
            oc338_fake_voice_mask,
            &OC338_PREV_VOICE_MASK,
        );
        covered(
            ee::s08::f_002a1d48__generated,
            ssx_audio_system_get_rider_voice_mask,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x002a_1d48, 3);
                c.pc = 0x002a_1d48;
                c.bus.write32(OC338_CTRL, OC338_MAGIC);
                c.g[A0][0] = sx(DATA + 0x5_0000);
                c.g[A1][0] = sx(DATA + 0x5_1000);
            },
        );
    }

    /// `SPCH_MatchFlags`: random bytes (junk above the low byte of the
    /// arguments), with the matching case forced half the time.
    #[test]
    fn spch_match_flags_matches() {
        check(
            ee::s14::f_003db3b8__generated,
            spch_match_flags,
            300,
            &|c, seed| {
                let mut r = Rng(0x003d_b3b8 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.pc = 0x003d_b3b8;
                let ev = DATA + 0x100 + (r.next_u32() & 0xff);
                fill(c, &mut r, DATA + 0x100, 0x48);
                if seed % 2 == 0 {
                    // want == ev[3] on the masked bits.
                    let flags = c.bus.read8(ev + 3) as u64;
                    c.g[A0][0] = (c.g[A0][0] & !0xff) | flags;
                }
                if seed % 5 == 0 {
                    c.bus.write8(ev + 2, 0);
                }
                c.g[A2][0] = sx(ev);
            },
        );
    }
}
