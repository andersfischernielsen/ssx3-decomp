//! Staging module oc332 (wave wave214): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc332`.

#![allow(unused_imports)]

use crate::abi::r32;
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
        0x0010_4b48,
        rider_anim_seq_end_type05,
        "RiderAnim_SeqEndType05",
    ),
    (
        0x002f_5af0,
        world_light_man_add_point_lights,
        "WorldLightMan_AddPointLights",
    ),
    (
        0x0010_4c18,
        rider_anim_seq_end_type04,
        "RiderAnim_SeqEndType04",
    ),
    (0x0038_5290, cps2_grid_mesh_dtor, "cPS2GridMesh_dtor"),
    (0x003d_83b8, spch_find_bank_data_ptr, "SPCH_FindBankDataPtr"),
    (0x0034_21a0, anim_teeter_ctor, "AnimTeeter_ctor"),
    (0x0036_1d50, particle_v01, "Particle_v01"),
    (0x0027_0b88, replay_begin_highlight, "Replay_BeginHighlight"),
    (
        0x0010_cbe0,
        computer_actor_rider_pick_random_trick,
        "ComputerActorRider_PickRandomTrick",
    ),
    (0x003c_8430, snd_ee_voice_finish, "SND_EeVoiceFinish"),
];

// ---------------------------------------------------------------------------
// RiderAnim sequence-end handlers (0x104b48, 0x104c18)

/// `RiderAnim_ClearFlagsAndPlayAnim` (0x312bd0): cross-shard, so the generated
/// code calls it through the function table.
pub(crate) const RIDER_ANIM_CLEAR_FLAGS_AND_PLAY_ANIM: u32 = 0x0031_2bd0;

/// The animation id used when the rider has an animation chain.
pub(crate) const SEQ_END_ANIM_CHAIN: u64 = 0x1b4;

/// The animation id used when the rider has no animation chain.
pub(crate) const SEQ_END_ANIM_NO_CHAIN: u64 = 0x11f;

/// `RiderAnim_SeqEndType05` (0x104b48): the sequence-end handler for state
/// type 05. It plays animation 0x1b4 when the rider's animation chain
/// (`RiderAnim+0x64`) is present, 0x11f otherwise, through
/// `RiderAnim_ClearFlagsAndPlayAnim(a, b, c, anim)`.
pub fn rider_anim_seq_end_type05(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x64)).lw(c);
        let has_chain = c.g[V0][0] != 0;
        c.g[A3][0] = SEQ_END_ANIM_CHAIN;
        if !has_chain {
            c.cycles += 1;
            c.g[A3][0] = SEQ_END_ANIM_NO_CHAIN;
        }
        c.cycles += 5;
        call_ptr(c, RIDER_ANIM_CLEAR_FLAGS_AND_PLAY_ANIM, 0x0010_4b68)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `RiderAnim_SeqEndType04` (0x104c18): the sequence-end handler for state
/// type 04; always plays animation 5.
pub fn rider_anim_seq_end_type04(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A3][0] = 0x5;
        call_ptr(c, RIDER_ANIM_CLEAR_FLAGS_AND_PLAY_ANIM, 0x0010_4c28)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WorldLightMan_AddPointLights (0x2f5af0)

/// `WorldLightMan_FindStrongestLights` (0x2f5b68), same shard.
pub(crate) const FIND_STRONGEST_LIGHTS: Callee = Callee {
    addr: 0x002f_5b68,
    f: ee::s09::WorldLightMan_FindStrongestLights,
};

/// The per-light apply step, cross-shard (`call_indirect` in the generated
/// body).
pub(crate) const LIGHT_APPLY: u32 = 0x0038_a6a8;

/// `void WorldLightMan_AddPointLights(WorldLightMan* self, void* a1, void* a2,
/// void* a3, int count)` (0x2f5af0): collect the `count` strongest point
/// lights into a stack buffer, then add each non-null one through
/// `func_0038A6A8(a3, a1, light)`.
pub fn world_light_man_add_point_lights(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S0, 0xc0);
        frame.save(c, S2, 0xa0);
        c.g[S0][0] = c.g[T0][0];
        frame.save(c, S3, 0x90);
        c.g[S2][0] = c.g[A1][0];
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S1, 0xb0);
        frame.save_ra(c, 0x80);
        // The light buffer is the 32-word scratch area at the frame base.
        c.g[A3][0] = c.g[SP][0];
        FIND_STRONGEST_LIGHTS.call(c, 0x002f_5b1c)?;
        let mut remaining = c.g[S0][0] as i64;
        c.g[S1][0] = c.g[S0][0];
        if remaining > 0 {
            c.cycles += 1;
            c.g[S0][0] = c.g[SP][0];
            loop {
                c.cycles += 3;
                c.g[A2][0] = Addr(r32(c, S0)).lw(c);
                let empty = c.g[A2][0] == 0;
                c.g[A0][0] = c.g[S3][0];
                if !empty {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S2][0];
                    call_ptr(c, LIGHT_APPLY, 0x002f_5b3c)?;
                }
                c.cycles += 3;
                c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
                remaining = c.g[S1][0] as i64;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
                if remaining == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_5b28);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cPS2GridMesh_dtor (0x385290)

/// `cMemMan_free` (0x317e98), cross-shard.
pub(crate) const CMEM_FREE: u32 = 0x0031_7e98;

/// `GridMeshBase_dtor` (0x385260), same shard.
pub(crate) const GRID_MESH_BASE_DTOR: Callee = Callee {
    addr: 0x0038_5260,
    f: ee::s12::GridMeshBase_dtor,
};

/// `void cPS2GridMesh::~cPS2GridMesh(this, int flags)` (0x385290): free the
/// DMA chain at +0x19c when present, then run the base destructor.
pub fn cps2_grid_mesh_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x490000;
        frame.save(c, 16, 0x20);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x3208));
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw(r32(c, S0).wrapping_add(0x4), c.g[V0][0]);
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x19c)).lw(c);
        let no_chain = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_chain {
            c.cycles += 2;
            call_ptr(c, CMEM_FREE, 0x0038_52c4)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        GRID_MESH_BASE_DTOR.call(c, 0x0038_52d0)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SPCH_FindBankDataPtr (0x3d83b8)

/// The bank descriptor table (`.data`): eight `{ void* p; }` entries.
#[cfg(test)]
pub(crate) const SPCH_BANK_TABLE: u32 = 0x0044_ffb8;

/// `int SPCH_FindBankDataPtr(u8* key, u8** data)` (0x3d83b8): find the bank
/// descriptor whose byte +9 matches `key[2]` and byte +8 matches `key[3]`
/// (scanning at most eight entries); when found, return 1 and write the
/// bank's data pointer plus the size derived from its u16 at +0x10.
pub fn spch_find_bank_data_ptr(c: &mut Cpu) {
    c.cycles += 8;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = 0x450000;
    c.g[T2][0] = c.g[A1][0];
    c.g[A2][0] = sx(r32(c, V0).wrapping_sub(0x48));
    c.g[T3][0] = 0;
    c.g[T1][0] = c.g[SP][0];
    c.g[T0][0] = 0;
    c.g[A3][0] = 0;
    loop {
        c.cycles += 3;
        let entry = Addr(r32(c, A2)).lw(c);
        c.g[A1][0] = entry;
        let mut found = false;
        if entry == 0 {
            c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
        } else {
            let e = entry as u32;
            c.cycles += 4;
            c.g[V1][0] = Addr(e.wrapping_add(9)).lbu(c);
            c.g[V0][0] = Addr(r32(c, A0).wrapping_add(2)).lbu(c);
            if c.g[V1][0] != c.g[V0][0] {
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
            } else {
                c.cycles += 4;
                c.g[V1][0] = Addr(e.wrapping_add(8)).lbu(c);
                c.g[V0][0] = Addr(r32(c, A0).wrapping_add(3)).lbu(c);
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                } else {
                    c.cycles += 3;
                    c.sw(r32(c, T1), c.g[A2][0]);
                    c.g[T0][0] = 1;
                    found = true;
                }
            }
        }
        if found {
            break;
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A3][0] as i64) < 8) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(8));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003d_83d8);
        }
    }
    c.cycles += 2;
    let not_found = c.g[T0][0] == 0;
    c.g[A1][0] = Addr(r32(c, SP)).lw(c);
    if !not_found {
        c.cycles += 11;
        // ~3 = 0xffff_ffff_ffff_fffc, the 4-byte alignment mask.
        c.g[A0][0] = 0xffff_ffff_ffff_0000;
        c.g[A0][0] |= 0xfffc;
        c.g[T3][0] = 1;
        c.g[V1][0] = Addr(r32(c, A1)).lw(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x10)).lhu(c);
        c.g[V0][0] = sx(r32(c, V0) << 1);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(3));
        c.g[V0][0] &= c.g[A0][0];
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x18));
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        c.sw(r32(c, T2), c.g[V1][0]);
    }
    c.cycles += 3;
    c.g[V0][0] = c.g[T3][0];
    frame.close(c);
}

// ---------------------------------------------------------------------------
// AnimTeeter_ctor (0x3421a0)

/// `AnimObject_BaseCtor` (0x34d9b0), same shard.
pub(crate) const ANIM_OBJECT_BASE_CTOR: Callee = Callee {
    addr: 0x0034_d9b0,
    f: ee::s11::AnimObject_BaseCtor,
};

/// `IntersectInfo_GetDuration` (0x351508), same shard.
pub(crate) const INTERSECT_INFO_GET_DURATION: Callee = Callee {
    addr: 0x0035_1508,
    f: ee::s11::IntersectInfo_GetDuration,
};

/// The pool constants the teeter's rates scale by (gp-relative): two 30 Hz
/// steps, the per-byte duration scale, and one more 30 Hz step.
pub(crate) const POOL_RATE: u32 = 0x2b6c;
pub(crate) const POOL_RATE_MAX: u32 = 0x2b68;
pub(crate) const POOL_PER_BYTE: u32 = 0x2b64;
pub(crate) const POOL_FRAME: u32 = 0x2b60;

/// `void* AnimTeeter_ctor(AnimTeeter* self, int a1, char* obj, char* def)`
/// (0x3421a0): set up the base animation object and copy the teeter's rate,
/// speed and fade parameters out of its definition record.
pub fn anim_teeter_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x40);
        c.g[T0][0] = 0;
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, 18, 0x10);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A3][0];
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(0x30));
        c.g[A3][0] = c.g[S0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = 0x4;
        ANIM_OBJECT_BASE_CTOR.call(c, 0x0034_21d4)?;
        c.g[V0][0] = 0x490000;
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x78)).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x8f8));
        c.mtc1(0, 2); // f2 = 0.0f (the "no instance" default)
        c.sw(r32(c, S1).wrapping_add(0x50), c.g[V0][0]);
        let no_instance = c.g[A0][0] == 0;
        c.g[S0][0] = Addr(r32(c, S0).wrapping_add(0x80)).lw(c);
        if !no_instance {
            c.cycles += 3;
            INTERSECT_INFO_GET_DURATION.call(c, 0x0034_21f8)?;
            c.mov_s(2, 0); // f2 = duration
        }
        // setRate(self->f10, def->f20, duration): negative rates keep the
        // duration, others are per-frame steps.
        c.cycles += 6;
        c.lwc1(1, r32(c, S2).wrapping_add(0x20));
        c.mtc1(0, 0);
        c.c_le_s(0, 1);
        c.g[V0][0] = sx(r32(c, S1).wrapping_add(0x10));
        if c.fcc() {
            c.cycles += 3;
            c.lwc1(0, r32(c, GP).wrapping_sub(POOL_RATE));
            c.mul_s(0, 1, 0);
        } else {
            c.cycles += 1;
            c.mov_s(0, 2);
        }
        // setRateMax(self->f14, def->f24, info): a negative rate takes the
        // info record's +0x14 instead.
        c.cycles += 7;
        c.swc1(0, r32(c, V0));
        c.mtc1(0, 0);
        c.lwc1(1, r32(c, S2).wrapping_add(0x24));
        c.c_lt_s(1, 0);
        c.g[V0][0] = sx(r32(c, S1).wrapping_add(0x14));
        if !c.fcc() {
            c.cycles += 2;
            c.lwc1(0, r32(c, GP).wrapping_sub(POOL_RATE_MAX));
            c.mul_s(0, 1, 0);
        } else {
            c.cycles += 2;
            c.lwc1(0, r32(c, S0).wrapping_add(0x14));
        }
        // The copied parameters.
        c.cycles += 37;
        c.swc1(0, r32(c, V0));
        c.g[V1][0] = Addr(r32(c, GP).wrapping_add(0x2a74)).lw(c);
        c.g[V0][0] = c.g[S1][0];
        c.lwc1(0, r32(c, S2).wrapping_add(0x8));
        c.lwc1(2, r32(c, GP).wrapping_sub(POOL_PER_BYTE));
        c.swc1(0, r32(c, S1).wrapping_add(0xc));
        c.lwc1(3, r32(c, GP).wrapping_sub(POOL_FRAME));
        c.lwc1(1, r32(c, S2).wrapping_add(0x4));
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x56)).lhu(c);
        c.mul_s(1, 1, 2);
        frame.restore(c, 16, 0x30);
        c.g[A0][0] |= 0x1;
        frame.restore_ra(c, 0);
        c.swc1(1, r32(c, S1).wrapping_add(0x8));
        c.lwc1(0, r32(c, S2).wrapping_add(0xc));
        c.lwc1(1, r32(c, V1).wrapping_add(0x10));
        c.cvt_s_w(1, 1);
        c.mul_s(0, 0, 3);
        c.div_s(0, 0, 1);
        c.swc1(0, r32(c, S1).wrapping_add(0x18));
        c.g[V1][0] = Addr(r32(c, S2).wrapping_add(0x1c)).lw(c);
        c.sw(r32(c, S1).wrapping_add(0x2c), c.g[V1][0]);
        c.lwc1(0, r32(c, S2).wrapping_add(0x14));
        c.mul_s(0, 0, 3);
        c.swc1(0, r32(c, S1).wrapping_add(0x24));
        c.lwc1(1, r32(c, S2).wrapping_add(0x18));
        c.swc1(1, r32(c, S1).wrapping_add(0x1c));
        c.lwc1(0, r32(c, S2).wrapping_add(0x10));
        c.sh(r32(c, S1).wrapping_add(0x56), c.g[A0][0]);
        c.swc1(0, r32(c, S1).wrapping_add(0x20));
        c.sw(r32(c, S1), 0);
        c.sw(r32(c, S1).wrapping_add(0x4), 0);
        c.sw(r32(c, S1).wrapping_add(0x28), 0);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Particle_v01 (0x361d50)

/// The particle node's base destructor (0x3553c0), cross-shard.
pub(crate) const PARTICLE_BASE_DTOR: u32 = 0x0035_53c0;

/// `cParticleNode::~cParticleNode(this)` (0x361d50): install the
/// `cParticleNode` vtable and run the base destructor.
pub fn particle_v01(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x490000;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x11a0));
        c.sw(r32(c, V1).wrapping_add(0xc), c.g[V0][0]);
        call_ptr(c, PARTICLE_BASE_DTOR, 0x0036_1d6c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay_BeginHighlight (0x270b88)

/// One replay highlight slot: 0xb4 bytes, `state` at +0xb0.
pub(crate) const REPLAY_SLOT_STRIDE: u64 = 0xb4;

/// The slot array base relative to the cache (+0x494); the busy word is at
/// `slot + 0xb0` and the rider at +0x3b4.
pub(crate) const REPLAY_SLOTS: u64 = 0x494;

/// The rider handle the highlight copies from.
pub(crate) const REPLAY_RIDER: u64 = 0x3b4;

/// `Replay_HighlightSlotBegin` (0x26e6e0), same shard.
pub(crate) const HIGHLIGHT_SLOT_BEGIN: Callee = Callee {
    addr: 0x0026_e6e0,
    f: ee::s07::Replay_HighlightSlotBegin,
};

/// `void Replay_BeginHighlight(ReplayCache* self, int slot, int id)`
/// (0x270b88): start the replay highlight in `slot` unless the cache is
/// already recording (`+0`) or the slot is still busy (+0xb0).
pub fn replay_begin_highlight(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, A3)).lw(c);
        let recording = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if !recording {
            c.cycles += 6;
            c.g[A0][0] = REPLAY_SLOT_STRIDE;
            c.mult(0, V1, A1, A0);
            c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, A3)));
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x544)).lw(c);
            let busy = c.g[V1][0] != 0;
            if !busy {
                c.cycles += 6;
                c.mult(0, A0, A1, A0);
                c.g[A1][0] = Addr(r32(c, A3).wrapping_add(REPLAY_RIDER as u32)).lw(c);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(REPLAY_SLOTS as u32));
                c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
                HIGHLIGHT_SLOT_BEGIN.call(c, 0x0027_0bcc)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ComputerActorRider_PickRandomTrick (0x10cbe0)

/// `AIrand` (0x317810), cross-shard.
pub(crate) const AI_RAND: u32 = 0x0031_7810;

/// `cBE_getBE` (0x14dc80), cross-shard.
pub(crate) const CBE_GET_BE: u32 = 0x0014_dc80;

/// `BELibrary_GetInterface` (0x14dd58), cross-shard.
pub(crate) const CBE_GET_INTERFACE: u32 = 0x0014_dd58;

/// `BEBAGTInt_GetTrickField0` (0x1500d8), cross-shard.
pub(crate) const BE_GET_TRICK_FIELD0: u32 = 0x0015_00d8;

/// `Rider_GetTrickSkillFactor` (0x120038), same shard.
pub(crate) const RIDER_GET_TRICK_SKILL_FACTOR: Callee = Callee {
    addr: 0x0012_0038,
    f: ee::s00::Rider_GetTrickSkillFactor,
};

/// The per-trick field lookup (0x312820), cross-shard.
pub(crate) const TRICK_FIELD: u32 = 0x0031_2820;

/// The trick-usage counters, one `int` per trick id (`self+0xe7c`).
pub(crate) const TRICK_USE_COUNTS: u32 = 0xe7c;

/// `int ComputerActorRider_PickRandomTrick(ComputerActorRider* self, float t)`
/// (0x10cbe0): pick one of the rider's 15 fitted tricks at random and return
/// its index, recording the remaining air time in +0xe34 and the trick's
/// duration in +0xe2c. Returns -1 while there is not enough air time.
pub fn computer_actor_rider_pick_random_trick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x60);
        frame.save(c, 17, 0x20);
        frame.save_fpr(c, 22, 0x50);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 16, 0x30);
        c.mov_s(22, 12); // t
        frame.save(c, 18, 0x10);
        frame.save_fpr(c, 21, 0x48);
        frame.save_fpr(c, 20, 0x40);
        frame.save_ra(c, 0);
        c.sw(r32(c, S1).wrapping_add(0xe24), 0);
        c.sw(r32(c, S1).wrapping_add(0xe28), 0);
        c.swc1(22, r32(c, S1).wrapping_add(0xe30));
        call_ptr(c, AI_RAND, 0x0010_cc18)?;
        c.g[V1][0] = 0xf;
        c.divu(0, V0, V1);
        c.cycles += 38;
        c.g[S2][0] = c.hi[0]; // idx = rand % 15
        call_ptr(c, CBE_GET_BE, 0x0010_cc34)?;
        c.g[A1][0] = 0x6;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, CBE_GET_INTERFACE, 0x0010_cc40)?;
        // The rider's anim object's virtual at vtable +0x38.
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x18)).lw(c);
        c.g[S0][0] = c.g[V0][0];
        c.g[V1][0] = Addr(r32(c, A0).wrapping_add(0x6c0)).lw(c);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6c0));
        c.g[A1][0] = Addr(r32(c, V1).wrapping_add(0x38)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x3c)).lw(c);
        let anim_get = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
        call_ptr(c, anim_get, 0x0010_cc60)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S2][0];
        call_ptr(c, BE_GET_TRICK_FIELD0, 0x0010_cc70)?;
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x18)).lw(c);
        c.g[S0][0] = c.g[V0][0]; // anim
        RIDER_GET_TRICK_SKILL_FACTOR.call(c, 0x0010_cc7c)?;
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x18)).lw(c);
        c.mov_s(20, 0); // spd = skill factor
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = 0x2;
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x784)).lw(c);
        call_ptr(c, TRICK_FIELD, 0x0010_cc94)?;
        c.div_s(21, 0, 20); // a = field2 / spd
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x18)).lw(c);
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = 0x1;
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x784)).lw(c);
        call_ptr(c, TRICK_FIELD, 0x0010_ccac)?;
        c.div_s(0, 0, 20); // b = field1 / spd
        c.c_lt_s(22, 21); // t < a?
        let too_early = c.fcc();
        c.g[V0][0] = u64::MAX;
        if !too_early {
            c.cycles += 5;
            c.sub_s(0, 21, 0); // a - b
            c.sub_s(1, 22, 21); // t - a
            c.swc1(0, r32(c, S1).wrapping_add(0xe2c));
            let unknown = c.g[S2][0] == c.g[V0][0];
            c.swc1(1, r32(c, S1).wrapping_add(0xe34));
            if !unknown {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S2][0] as i64) < 15) as u64;
                let out_of_range = c.g[V0][0] == 0;
                c.g[V0][0] = sx(r32(c, S1).wrapping_add(TRICK_USE_COUNTS));
                if !out_of_range {
                    c.cycles += 5;
                    c.g[A0][0] = sx(r32(c, S2) << 2);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
                    c.sw(r32(c, V0), c.g[V1][0]);
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S2][0];
        }
        // Too early: return -1 (v0 set before the branch).
        c.cycles += 9;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x50);
        frame.restore_fpr(c, 21, 0x48);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_EeVoiceFinish (0x3c8430)

/// `SND_EeVoiceStop` (0x3c8468), same shard.
pub(crate) const SND_EE_VOICE_STOP: Callee = Callee {
    addr: 0x003c_8468,
    f: ee::s14::SND_EeVoiceStop,
};

/// The sound library's per-voice handler (`D_00515B48`).
pub(crate) const SND_VOICE_HANDLER: u32 = 0x0051_5b48;

/// `void SND_EeVoiceFinish(int voice)` (0x3c8430): stop the voice, then let
/// the sound library's handler finish it.
pub fn snd_ee_voice_finish(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        SND_EE_VOICE_STOP.call(c, 0x003c_8444)?;
        c.g[V1][0] = 0x510000;
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x5b48)).lw(c);
        let handler = r32(c, V0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, handler, 0x003c_8454)?;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::fakes::{machine, vtable, FAKE};
    use crate::difftest::SP as STACK;
    use crate::difftest::{check, DATA, GAME_GP};
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// Magic word that turns the gated test fakes on. It lives in each test
    /// machine's own RAM (`DATA + 0xf0000`, below the fakes' call log), so the
    /// rest of the suite reaches the fakes' chained-to bodies instead.
    const CTRL: u32 = DATA + 0x0f_0000;
    const MAGIC: u32 = 0x4f43_3332;

    /// The bodies the slots held before a test fake was installed.
    static PREV: [AtomicUsize; 16] = [const { AtomicUsize::new(0) }; 16];

    /// Replace a generated slot with `fake` (once), remembering what it held.
    fn install(slot: *mut fn(&mut Cpu), fake: fn(&mut Cpu), idx: usize) {
        crate::difftest::fakes::hook(slot, fake, &PREV[idx]);
    }

    /// Run the body a slot held before our fake (if any).
    fn chain(c: &mut Cpu, idx: usize, mine: fn(&mut Cpu)) {
        let p = PREV[idx].load(Ordering::Relaxed);
        if p != 0 && p != mine as usize {
            let f: GuestFn = unsafe { std::mem::transmute(p) };
            f(c);
        }
    }

    fn active(c: &mut Cpu) -> bool {
        c.bus.read32(CTRL) == MAGIC
    }

    /// Return to the caller after `cycles`, with `v0`/`v1` clobbered.
    fn fake_ret(c: &mut Cpu, cycles: u64) {
        c.g[V0][0] = c.bus.read32(CTRL + 8) as u64;
        c.g[V1][0] = c.bus.read32(CTRL + 0xc) as u64;
        c.cycles += cycles;
        let ra = c.g[RA][0] as u32;
        c.pc = ra;
    }

    // -- slot 0: RiderAnim_ClearFlagsAndPlayAnim (0x312bd0) ------------------

    fn fake_clear_flags_and_play_anim(c: &mut Cpu) {
        if !active(c) {
            return chain(c, 0, fake_clear_flags_and_play_anim);
        }
        if c.bus.read32(CTRL + 4) != 0 {
            // Ask the caller to unwind.
            c.pc = 0xdead_0000;
            return;
        }
        fake_ret(c, 5);
    }

    fn install_seq_end_fakes() {
        install(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00312BD0),
            fake_clear_flags_and_play_anim,
            0,
        );
    }

    #[test]
    fn rider_anim_seq_end_type05_matches() {
        install_seq_end_fakes();
        check(
            ee::s00::f_00104b48__generated,
            rider_anim_seq_end_type05,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_4b48, 1);
                let anim = DATA + 0x1000;
                c.g[A0][0] = anim as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = (DATA + 0x3000) as u64;
                // The rider's animation chain, absent on every third seed.
                c.bus
                    .write32(anim + 0x64, if seed % 3 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(CTRL, MAGIC);
                c.bus.write32(CTRL + 4, (seed % 4 == 0) as u32);
                c.pc = 0x0010_4b48;
            },
        );
    }

    #[test]
    fn rider_anim_seq_end_type04_matches() {
        install_seq_end_fakes();
        check(
            ee::s00::f_00104c18__generated,
            rider_anim_seq_end_type04,
            200,
            &|c, seed| {
                machine(c, seed, 0x0010_4c18, 1);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = (DATA + 0x3000) as u64;
                c.bus.write32(CTRL, MAGIC);
                c.bus.write32(CTRL + 4, (seed % 4 == 0) as u32);
                c.pc = 0x0010_4c18;
            },
        );
    }

    // -- slot 1/2: WorldLightMan_FindStrongestLights (0x2f5b68) --------------

    fn fake_find_strongest_lights(c: &mut Cpu) {
        if !active(c) {
            return chain(c, 1, fake_find_strongest_lights);
        }
        c.g[V0][0] = 1;
        c.cycles += 6;
        let ra = c.g[RA][0] as u32;
        c.pc = ra;
    }

    fn fake_light_apply(c: &mut Cpu) {
        if !active(c) {
            return chain(c, 2, fake_light_apply);
        }
        fake_ret(c, 3);
    }

    #[test]
    fn world_light_man_add_point_lights_matches() {
        install(
            std::ptr::addr_of_mut!(ee::s09::OVERRIDE_002F5B68),
            fake_find_strongest_lights,
            1,
        );
        install(
            std::ptr::addr_of_mut!(ee::s12::OVERRIDE_0038A6A8),
            fake_light_apply,
            2,
        );
        check(
            ee::s09::f_002f5af0__generated,
            world_light_man_add_point_lights,
            200,
            &|c, seed| {
                machine(c, seed, 0x002f_5af0, 2);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = (DATA + 0x3000) as u64;
                c.g[A3][0] = (DATA + 0x4000) as u64;
                let count: i32 = match seed % 4 {
                    0 => 0,
                    1 => -1,
                    2 => 2,
                    _ => 4,
                };
                c.g[T0][0] = sx(count as u32);
                // The light buffer is the 32-word area at the frame base.
                let buf = STACK - 0xd0;
                for i in 0..32u32 {
                    let v = if (seed + i) % 3 == 0 {
                        0
                    } else {
                        (DATA + 0x5000 + (i << 4)) | 0x1000
                    };
                    c.bus.write32(buf + 4 * i, v);
                }
                c.bus.write32(CTRL, MAGIC);
                c.pc = 0x002f_5af0;
            },
        );
    }

    // -- slot 3: cMemMan_free (0x317e98) -------------------------------------

    fn fake_mem_free(c: &mut Cpu) {
        if !active(c) {
            return chain(c, 3, fake_mem_free);
        }
        fake_ret(c, 4);
    }

    #[test]
    fn cps2_grid_mesh_dtor_matches() {
        install(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            fake_mem_free,
            3,
        );
        check(ee::s12::f_00385290__generated, cps2_grid_mesh_dtor, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x0038_5290, 1);
            let mesh = DATA + 0x1000;
            c.g[A0][0] = mesh as u64;
            // Flags 0 keep the base destructor on its no-delete path.
            c.g[A1][0] = 0;
            c.bus.write32(
                mesh + 0x19c,
                if seed % 2 == 0 {
                    0
                } else {
                    r.next_u32() | 0x1000
                },
            );
            c.bus.write32(CTRL, MAGIC);
            c.pc = 0x0038_5290;
        });
    }

    // -- SPCH_FindBankDataPtr ------------------------------------------------

    #[test]
    fn spch_find_bank_data_ptr_matches() {
        check(
            ee::s14::f_003d83b8__generated,
            spch_find_bank_data_ptr,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003d_83b8, 0);
                let key = DATA + 0x1000;
                let out = DATA + 0x1400;
                c.g[A0][0] = key as u64;
                c.g[A1][0] = out as u64;
                let want2 = (seed % 251) as u8;
                let want3 = (seed / 7 % 251) as u8;
                c.bus.write8(key + 2, want2);
                c.bus.write8(key + 3, want3);
                // Where a matching entry sits, when it is not a null one.
                let match_at = seed % 9;
                for i in 0..8u32 {
                    let slot_addr = SPCH_BANK_TABLE + 8 * i;
                    let null = (seed + i) % 5 == 0;
                    if null {
                        c.bus.write32(slot_addr, 0);
                        continue;
                    }
                    let entry = DATA + 0x2000 + 0x40 * i;
                    c.bus.write32(slot_addr, entry);
                    c.bus
                        .write8(entry + 8, if i == match_at { want3 } else { want3 ^ 0xff });
                    c.bus
                        .write8(entry + 9, if i == match_at { want2 } else { want2 ^ 0x55 });
                    c.bus.write16(entry + 0x10, r.next_u32() as u16);
                }
                c.pc = 0x003d_83b8;
            },
        );
    }

    // -- AnimTeeter_ctor -----------------------------------------------------

    fn fake_anim_object_base_ctor(c: &mut Cpu) {
        if !active(c) {
            return chain(c, 4, fake_anim_object_base_ctor);
        }
        fake_ret(c, 9);
    }

    fn fake_intersect_info_get_duration(c: &mut Cpu) {
        if !active(c) {
            return chain(c, 5, fake_intersect_info_get_duration);
        }
        c.f[0] = c.bus.read32(CTRL + 8);
        c.g[V0][0] = c.bus.read32(CTRL + 0xc) as u64;
        c.cycles += 7;
        let ra = c.g[RA][0] as u32;
        c.pc = ra;
    }

    #[test]
    fn anim_teeter_ctor_matches() {
        install(
            std::ptr::addr_of_mut!(ee::s11::OVERRIDE_0034D9B0),
            fake_anim_object_base_ctor,
            4,
        );
        install(
            std::ptr::addr_of_mut!(ee::s11::OVERRIDE_00351508),
            fake_intersect_info_get_duration,
            5,
        );
        check(ee::s11::f_003421a0__generated, anim_teeter_ctor, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x0034_21a0, 2);
            let (self_, obj, def, info, inst, dur) = (
                DATA + 0x1000,
                DATA + 0x2000,
                DATA + 0x3000,
                DATA + 0x4000,
                DATA + 0x5000,
                DATA + 0x6000,
            );
            c.g[A0][0] = self_ as u64;
            c.g[A1][0] = (DATA + 0x900) as u64;
            c.g[A2][0] = obj as u64;
            c.g[A3][0] = def as u64;
            c.bus
                .write32(self_ + 0x78, if seed % 3 == 0 { 0 } else { inst });
            c.bus.write16(self_ + 0x56, r.next_u32() as u16);
            c.bus.write32(obj + 0x80, info);
            for off in [0x4u32, 0x8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24] {
                c.bus.write32(def + off, r.edge_f32());
            }
            c.bus.write32(info + 0x14, r.edge_f32());
            // The 30 Hz and 1e-6 pools and the pool-info global.
            c.bus
                .write32(GAME_GP.wrapping_sub(0x2b6c), 0.033_333_335f32.to_bits());
            c.bus
                .write32(GAME_GP.wrapping_sub(0x2b68), 0.033_333_335f32.to_bits());
            c.bus
                .write32(GAME_GP.wrapping_sub(0x2b60), 0.033_333_335f32.to_bits());
            c.bus
                .write32(GAME_GP.wrapping_sub(0x2b64), 1.0e-6f32.to_bits());
            c.bus.write32(GAME_GP.wrapping_add(0x2a74), dur);
            c.bus.write32(dur + 0x10, 1 + r.next_u32() % 30);
            c.bus.write32(CTRL, MAGIC);
            c.bus.write32(CTRL + 8, r.edge_f32());
            c.bus.write32(CTRL + 0xc, r.next_u32());
            c.pc = 0x0034_21a0;
        });
    }

    // -- Particle_v01 --------------------------------------------------------

    fn fake_particle_base_dtor(c: &mut Cpu) {
        if !active(c) {
            return chain(c, 6, fake_particle_base_dtor);
        }
        fake_ret(c, 8);
    }

    #[test]
    fn particle_v01_matches() {
        install(
            std::ptr::addr_of_mut!(ee::s11::OVERRIDE_003553C0),
            fake_particle_base_dtor,
            6,
        );
        check(ee::s12::f_00361d50__generated, particle_v01, 200, &|c, seed| {
            machine(c, seed, 0x0036_1d50, 1);
            c.g[A0][0] = (DATA + 0x1000) as u64;
            c.bus.write32(CTRL, MAGIC);
            c.pc = 0x0036_1d50;
        });
    }

    // -- Replay_BeginHighlight ----------------------------------------------

    fn fake_highlight_slot_begin(c: &mut Cpu) {
        if !active(c) {
            return chain(c, 7, fake_highlight_slot_begin);
        }
        fake_ret(c, 10);
    }

    #[test]
    fn replay_begin_highlight_matches() {
        install(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0026E6E0),
            fake_highlight_slot_begin,
            7,
        );
        check(
            ee::s07::f_00270b88__generated,
            replay_begin_highlight,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_0b88, 2);
                let cache = DATA + 0x1000;
                let slot = (seed % 4) as u64;
                c.g[A0][0] = cache as u64;
                c.g[A1][0] = slot;
                c.g[A2][0] = (DATA + 0x3000) as u64;
                c.bus
                    .write32(cache, if seed % 3 == 0 { r.next_u32() } else { 0 });
                let slot_addr = cache + (slot * REPLAY_SLOT_STRIDE) as u32;
                c.bus
                    .write32(slot_addr + 0x544, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(cache + REPLAY_RIDER as u32, DATA + 0x2000);
                c.bus.write32(CTRL, MAGIC);
                c.pc = 0x0027_0b88;
            },
        );
    }

    // -- ComputerActorRider_PickRandomTrick ----------------------------------
    //
    // slots: 8 AIrand, 9 cBE_getBE, 10 BELibrary_GetInterface,
    //        11 BEBAGTInt_GetTrickField0, 12 Rider_GetTrickSkillFactor,
    //        13 the per-trick field lookup.

    fn fake_ai_rand(c: &mut Cpu) {
        if !active(c) {
            return chain(c, 8, fake_ai_rand);
        }
        c.g[V0][0] = c.bus.read32(CTRL + 0x10) as u64;
        c.cycles += 4;
        let ra = c.g[RA][0] as u32;
        c.pc = ra;
    }

    fn fake_get_be(c: &mut Cpu) {
        if !active(c) {
            return chain(c, 9, fake_get_be);
        }
        c.g[V0][0] = c.bus.read32(CTRL + 0x14) as u64;
        c.cycles += 4;
        let ra = c.g[RA][0] as u32;
        c.pc = ra;
    }

    fn fake_get_interface(c: &mut Cpu) {
        if !active(c) {
            return chain(c, 10, fake_get_interface);
        }
        c.g[V0][0] = c.bus.read32(CTRL + 0x18) as u64;
        c.cycles += 4;
        let ra = c.g[RA][0] as u32;
        c.pc = ra;
    }

    fn fake_get_trick_field(c: &mut Cpu) {
        if !active(c) {
            return chain(c, 11, fake_get_trick_field);
        }
        c.g[V0][0] = c.bus.read32(CTRL + 0x1c) as u64;
        c.cycles += 4;
        let ra = c.g[RA][0] as u32;
        c.pc = ra;
    }

    fn fake_get_skill_factor(c: &mut Cpu) {
        if !active(c) {
            return chain(c, 12, fake_get_skill_factor);
        }
        c.f[0] = c.bus.read32(CTRL + 0x20);
        c.cycles += 4;
        let ra = c.g[RA][0] as u32;
        c.pc = ra;
    }

    fn fake_trick_field(c: &mut Cpu) {
        if !active(c) {
            return chain(c, 13, fake_trick_field);
        }
        // Field 2 and field 1 of the trick, by the a2 selector.
        c.f[0] = c.bus.read32(CTRL + 0x20 + 4 * (r32(c, A2) & 3));
        c.cycles += 4;
        let ra = c.g[RA][0] as u32;
        c.pc = ra;
    }

    fn install_pick_trick_fakes() {
        install(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317810),
            fake_ai_rand,
            8,
        );
        install(
            std::ptr::addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            fake_get_be,
            9,
        );
        install(
            std::ptr::addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            fake_get_interface,
            10,
        );
        install(
            std::ptr::addr_of_mut!(ee::s01::OVERRIDE_001500D8),
            fake_get_trick_field,
            11,
        );
        install(
            std::ptr::addr_of_mut!(ee::s00::OVERRIDE_00120038),
            fake_get_skill_factor,
            12,
        );
        install(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00312820),
            fake_trick_field,
            13,
        );
    }

    #[test]
    fn computer_actor_rider_pick_random_trick_matches() {
        install_pick_trick_fakes();
        check(
            ee::s00::f_0010cbe0__generated,
            computer_actor_rider_pick_random_trick,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_cbe0, 4);
                let (self_, rider, anim) = (DATA + 0x1000, DATA + 0x2000, DATA + 0x3000);
                c.g[A0][0] = self_ as u64;
                c.f[12] = r.edge_f32();
                c.bus.write32(self_ + 0x18, rider);
                // The rider's anim object virtual at vtable +0x38.
                let vt = vtable(c, &mut r, &[0x38]);
                c.bus.write32(rider + 0x6c0, vt);
                c.bus.write32(rider + 0x784, anim);
                // The six callees' answers.
                c.bus.write32(CTRL, MAGIC);
                c.bus.write32(CTRL + 0x10, r.next_u32());
                c.bus.write32(CTRL + 0x14, DATA + 0x4000);
                c.bus.write32(CTRL + 0x18, DATA + 0x4800);
                c.bus.write32(CTRL + 0x1c, r.next_u32() % 15);
                c.bus
                    .write32(CTRL + 0x20, (0.5 + (r.next_u32() % 4) as f32).to_bits());
                c.bus.write32(CTRL + 0x24, r.f32(4.0));
                c.bus.write32(CTRL + 0x28, r.f32(4.0));
                c.pc = 0x0010_cbe0;
            },
        );
    }

    // -- SND_EeVoiceFinish ---------------------------------------------------

    fn fake_voice_stop(c: &mut Cpu) {
        if !active(c) {
            return chain(c, 14, fake_voice_stop);
        }
        fake_ret(c, 11);
    }

    #[test]
    fn snd_ee_voice_finish_matches() {
        install(
            std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003C8468),
            fake_voice_stop,
            14,
        );
        check(ee::s14::f_003c8430__generated, snd_ee_voice_finish, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x003c_8430, 1);
            c.g[A0][0] = (DATA + 0x1000) as u64;
            c.bus.write32(SND_VOICE_HANDLER, FAKE);
            c.bus.write32(CTRL, MAGIC);
            c.bus.write32(CTRL + 8, r.next_u32());
            c.bus.write32(CTRL + 0xc, r.next_u32());
            c.pc = 0x003c_8430;
        });
    }
}
