//! Staging module oc336 (wave wave214): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc336`.

#![allow(unused_imports)]

use crate::abi;
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
    (0x0031_ffd8, dlheap_more_core, "DLHeap_MoreCore"),
    (
        0x002b_5758,
        world_trigger_instance_stop,
        "WorldTriggerInstance_Stop",
    ),
    (
        0x0011_a168,
        rider_metrix_announce_trick,
        "RiderMetrix_AnnounceTrick",
    ),
    (
        0x0035_5ad0,
        mod_obj_add_spline_modifier,
        "ModObj_AddSplineModifier",
    ),
    (
        0x0035_5918,
        mod_obj_add_position_modifier_from_matrix,
        "ModObj_AddPositionModifierFromMatrix",
    ),
    (0x003c_6ce8, sndstrm_stop_slot, "SNDSTRM_StopSlot"),
    (
        0x0035_6020,
        mod_obj_take_particle_modifiers,
        "ModObj_TakeParticleModifiers",
    ),
    (
        0x0011_a0c0,
        rider_metrix_return_zero_a0c0,
        "RiderMetrix_ReturnZero_a0c0",
    ),
    (
        0x0010_f280,
        rider_add_wipeout_recovery_boost,
        "Rider_AddWipeoutRecoveryBoost",
    ),
    (
        0x0035_6f10,
        position_modifier_ctor_2,
        "PositionModifier_ctor_2",
    ),
];

/// `SSXAudioSystem_GetInstance` (0x0028b180): the audio singleton.
const SSX_AUDIO_GET_INSTANCE: Callee = Callee {
    addr: 0x0028_b180,
    f: ee::s07::SSXAudioSystem_GetInstance,
};

/// `CrowdAudio_RemoveSource(crowd, sourceId, fadeSeconds)` (0x002a72d8).
const CROWD_AUDIO_REMOVE_SOURCE: Callee = Callee {
    addr: 0x002a_72d8,
    f: ee::s08::CrowdAudio_RemoveSource,
};

/// `BankMonitor_StopHandle(monitor, handle, kind, fadeSeconds)` (0x002ad5f0).
const BANK_MONITOR_STOP_HANDLE: Callee = Callee {
    addr: 0x002a_d5f0,
    f: ee::s08::BankMonitor_StopHandle,
};

/// World-trigger instance: the sound handle at +0x30.
pub(crate) const WORLD_TRIGGER_HANDLE: u32 = 0x30;
/// World-trigger instance: non-zero when the trigger owns a crowd source.
pub(crate) const WORLD_TRIGGER_CROWD: u32 = 0x38;
/// World-trigger instance: the "active" byte.
pub(crate) const WORLD_TRIGGER_ACTIVE: u32 = 0x40;
/// World-trigger instance: the sound instance/request words.
pub(crate) const WORLD_TRIGGER_SND: u32 = 0x60;
/// World-trigger instance: the fade value.
pub(crate) const WORLD_TRIGGER_FADE: u32 = 0x68;

/// `void WorldTriggerInstance_Stop(sWorldTrigger70* t)` (0x002b5758): stop
/// the trigger's crowd-audio source or sound instance (fading it out over
/// 0.25 s), then reset the handle, the active flag and the sound words.
pub fn world_trigger_instance_stop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(this.wrapping_add(WORLD_TRIGGER_CROWD)).lw(c);
        let crowd = c.g[V0][0] != 0;
        c.g[V0][0] = sx(0xffff_d8f1); // -9999: "no handle"
        let mut done = false;
        if crowd {
            c.cycles += 3;
            c.g[V1][0] = Addr(this.wrapping_add(WORLD_TRIGGER_HANDLE)).lw(c);
            let no_handle = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = u64::MAX;
            if no_handle {
                done = true;
            } else {
                c.cycles += 9;
                SSX_AUDIO_GET_INSTANCE.call(c, 0x002b_5788)?;
                c.g[A1][0] = Addr(this.wrapping_add(WORLD_TRIGGER_HANDLE)).lw(c);
                c.g[AT][0] = 0x3e80_0000; // 0.25f
                c.mtc1(1, 12);
                c.g[A0][0] = c.g[V0][0];
                CROWD_AUDIO_REMOVE_SOURCE.call(c, 0x002b_579c)?;
                c.g[V0][0] = u64::MAX;
                done = true;
            }
        }
        if !done {
            c.cycles += 3;
            c.g[V1][0] = Addr(this.wrapping_add(WORLD_TRIGGER_HANDLE)).lw(c);
            let no_handle = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = u64::MAX;
            if !no_handle {
                c.cycles += 11;
                SSX_AUDIO_GET_INSTANCE.call(c, 0x002b_57b8)?;
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x118)).lw(c);
                c.g[A2][0] = 0x1;
                c.g[A1][0] = Addr(this.wrapping_add(WORLD_TRIGGER_HANDLE)).lw(c);
                c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                c.g[AT][0] = 0x3e80_0000; // 0.25f
                c.mtc1(1, 12);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                BANK_MONITOR_STOP_HANDLE.call(c, 0x002b_57d8)?;
                c.g[V0][0] = u64::MAX;
            }
        }
        // Epilogue: clear the handle, deactivate, reset fade and sound words.
        c.cycles += 14;
        c.g[V1][0] = 0x4a_0000;
        Addr(this.wrapping_add(WORLD_TRIGGER_HANDLE)).sw(c, c.g[V0][0]);
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 0); // f0 = -1.0f
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x35a8)).lbu(c);
        c.swc1(0, this.wrapping_add(WORLD_TRIGGER_FADE));
        Addr(this.wrapping_add(WORLD_TRIGGER_ACTIVE)).sb(c, c.g[V0][0]);
        Addr(this.wrapping_add(WORLD_TRIGGER_SND + 0xc)).sw(c, 0);
        Addr(this.wrapping_add(WORLD_TRIGGER_SND)).sw(c, 0);
        Addr(this.wrapping_add(WORLD_TRIGGER_SND + 0x4)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `RiderMetrix_BuildTrickDesc(self, id, stance, alternate, style, flag)`
/// (0x0011a8c8).
const RIDER_METRIX_BUILD_TRICK_DESC: Callee = Callee {
    addr: 0x0011_a8c8,
    f: ee::s00::RiderMetrix_BuildTrickDesc,
};

/// `int Trick_LookupBonusTrick(RiderMetrix* self, sTrickId* id)` (0x0011b1a8).
const TRICK_LOOKUP_BONUS_TRICK: Callee = Callee {
    addr: 0x0011_b1a8,
    f: ee::s00::Trick_LookupBonusTrick,
};

/// `RiderMetrix_ShowHUDNumber(self, kind, value, arg, duration)` (0x00117b88).
const RIDER_METRIX_SHOW_HUD_NUMBER: Callee = Callee {
    addr: 0x0011_7b88,
    f: ee::s00::RiderMetrix_ShowHUDNumber,
};

/// `int RiderMetrix_CountTrickRepeats(RiderMetrix* self, sTrickId* id)`
/// (0x001190f0).
const RIDER_METRIX_COUNT_TRICK_REPEATS: Callee = Callee {
    addr: 0x0011_90f0,
    f: ee::s00::RiderMetrix_CountTrickRepeats,
};

/// `void RiderMetrix_ShowTrickName(RiderMetrix* self, sTrickId* id, int
/// repeat)` (0x00118ff8).
const RIDER_METRIX_SHOW_TRICK_NAME: Callee = Callee {
    addr: 0x0011_8ff8,
    f: ee::s00::RiderMetrix_ShowTrickName,
};

/// Rider metrics: the score float at +0x14.
pub(crate) const RIDER_METRIX_SCORE: u32 = 0x14;
/// Rider metrics: the rider object pointer at +0x1ac.
pub(crate) const RIDER_METRIX_RIDER: u32 = 0x1ac;
/// Rider metrics: the trick-class word at +0x70 (set while a special trick is
/// in progress; `BuildTrickDesc` then short-circuits).
pub(crate) const RIDER_METRIX_TRICK_STATE: u32 = 0x70;

/// `void RiderMetrix_AnnounceTrick(RiderMetrix* self, int stance, int
/// alternate, int style, int flag)` (0x0011a168): build the 8-byte trick id,
/// look up its bonus points; if it has any, award `points * 1e-4` to the
/// score and show the HUD number (plus the bonus-trick sound), then show the
/// trick name with its repeat count.
pub fn rider_metrix_announce_trick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = c.g[A2][0];
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.g[V1][0] = c.g[A3][0];
        c.g[T1][0] = c.g[T0][0];
        c.g[S0][0] = c.g[A0][0];
        c.g[A2][0] = c.g[A1][0];
        c.sd(sp(c).wrapping_add(0x10), c.g[RA][0]);
        c.g[A3][0] = c.g[V0][0];
        c.g[T0][0] = c.g[V1][0];
        c.g[A1][0] = c.g[SP][0];
        RIDER_METRIX_BUILD_TRICK_DESC.call(c, 0x0011_a198)?;
        let no_trick = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[S0][0];
        if !no_trick {
            c.cycles += 4;
            c.g[A1][0] = c.g[SP][0];
            TRICK_LOOKUP_BONUS_TRICK.call(c, 0x0011_a1a8)?;
            let points = c.g[V0][0];
            let no_points = (points as i64) <= 0;
            c.g[A0][0] = c.g[S0][0];
            if !no_points {
                c.cycles += 18;
                // score += points * 1e-4f
                c.mtc1(V0, 0);
                c.cvt_s_w(0, 0);
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x79f8));
                c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(RIDER_METRIX_SCORE));
                c.g[AT][0] = 0x3fc0_0000; // 1.5f
                c.mtc1(1, 12);
                c.g[A1][0] = 0x32;
                c.mul_s(0, 0, 2);
                c.g[A2][0] = c.g[V0][0];
                c.g[A3][0] = 0;
                c.add_s(1, 1, 0);
                c.swc1(1, (c.g[S0][0] as u32).wrapping_add(RIDER_METRIX_SCORE));
                RIDER_METRIX_SHOW_HUD_NUMBER.call(c, 0x0011_a1e4)?;
                call_ptr(c, 0x0028_b180, 0x0011_a1ec)?;
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(RIDER_METRIX_RIDER).lw(c);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0029_b7e0, 0x0011_a1f8)?;
            }
            c.cycles += 7;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[SP][0];
            RIDER_METRIX_COUNT_TRICK_REPEATS.call(c, 0x0011_a204)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = ((0u64 as i64) < (c.g[V0][0] as i64)) as u64;
            c.g[A1][0] = c.g[SP][0];
            RIDER_METRIX_SHOW_TRICK_NAME.call(c, 0x0011_a214)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `ModObj_SetTransformModifier(ModObj* o, Modifier* m)` (0x003554b0).
const MOD_OBJ_SET_TRANSFORM_MODIFIER: Callee = Callee {
    addr: 0x0035_54b0,
    f: ee::s11::ModObj_SetTransformModifier,
};

/// `SplineModifier` constructor (0x00359460).
const SPLINE_MODIFIER_CTOR: Callee = Callee {
    addr: 0x0035_9460,
    f: ee::s11::SplineModifier_ctor,
};

/// `cMemMan_alloc(size, name, flags, arg)` (0x00317d70), a pool allocator
/// reached through the function table.
const MEM_MAN_ALLOC: u32 = 0x0031_7d70;

/// `void ModObj_AddSplineModifier(ModObj* o, const tAddSplineData* desc)`
/// (0x00355ad0): allocate a 0xf0-byte spline modifier, construct it from the
/// descriptor and install it as the object's transform modifier.
pub fn mod_obj_add_spline_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0x2000_0000;
        frame.save(c, S0, 0x20);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = 0x49_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = 0xf0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1730)); // D_0048E8D0
        call_ptr(c, MEM_MAN_ALLOC, 0x0035_5b00)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        SPLINE_MODIFIER_CTOR.call(c, 0x0035_5b0c)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        MOD_OBJ_SET_TRANSFORM_MODIFIER.call(c, 0x0035_5b18)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ModObj_AddPositionModifierFromMatrix(ModObj* o, const Matrix* m)`
/// (0x00355918): allocate a 0x90-byte position modifier, construct it from
/// the matrix descriptor and install it as the object's transform modifier.
pub fn mod_obj_add_position_modifier_from_matrix(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0x2000_0000;
        frame.save(c, S0, 0x20);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = 0x49_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x90;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1710)); // D_0048E8F0
        call_ptr(c, MEM_MAN_ALLOC, 0x0035_5948)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        POSITION_MODIFIER_CTOR_2.call(c, 0x0035_5954)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        MOD_OBJ_SET_TRANSFORM_MODIFIER.call(c, 0x0035_5960)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `PositionModifier* PositionModifier_ctor_2(self, const Matrix* m)`
/// (0x00356f10); no override slot, so this is its generated entry.
const POSITION_MODIFIER_CTOR_2: Callee = Callee {
    addr: 0x0035_6f10,
    f: ee::s11::f_00356f10,
};

/// `void ModObj_TakeParticleModifiers(ModObj* o, ModObj* from)` (0x00356020):
/// take the particle list of `from`'s modifier block into `o`'s (creating
/// `o`'s block when it has none).
pub fn mod_obj_take_particle_modifiers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x1c)).lw(c);
        let none = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        if !none {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x1c)).lw(c);
            if c.g[V0][0] != 0 {
                c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x1c)).lw(c);
            } else {
                c.cycles += 3;
                MOD_OBJ_CREATE_MODIFIER_BLOCK.call(c, 0x0035_6054)?;
                c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x1c)).lw(c);
            }
            c.cycles += 3;
            c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x1c)).lw(c);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
            MODIFIER_BLOCK_TAKE_PARTICLE_LIST.call(c, 0x0035_6064)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ModObj_CreateModifierBlock(ModObj* o)` (0x00355468).
const MOD_OBJ_CREATE_MODIFIER_BLOCK: Callee = Callee {
    addr: 0x0035_5468,
    f: ee::s11::ModObj_CreateModifierBlock,
};

/// `ModifierBlock_TakeParticleList(block, list)` (0x00353398); no override
/// slot, so this is its generated entry.
const MODIFIER_BLOCK_TAKE_PARTICLE_LIST: Callee = Callee {
    addr: 0x0035_3398,
    f: ee::s11::f_00353398,
};

/// `SND_Stop(handle)` (0x003b89e0).
const SND_STOP: Callee = Callee {
    addr: 0x003b_89e0,
    f: ee::s13::SND_Stop,
};

/// `SNDSTRM_RunCallbacks()` (0x003c7080).
const SNDSTRM_RUN_CALLBACKS: Callee = Callee {
    addr: 0x003c_7080,
    f: ee::s13::SNDSTRM_RunCallbacks,
};

/// `SNDSTRM_StopSlotUnsupported(slot, arg, handle)` (0x003c6ce0).
const SNDSTRM_STOP_SLOT_UNSUPPORTED: Callee = Callee {
    addr: 0x003c_6ce0,
    f: ee::s13::SNDSTRM_StopSlotUnsupported,
};

/// `SND_MemFree(ptr)` (0x003b5e98).
const SND_MEM_FREE: Callee = Callee {
    addr: 0x003b_5e98,
    f: ee::s13::SND_MemFree,
};

/// The stream-slot table base (`D_005157B8`); a slot is at `base + n*4 + 0x304`.
pub(crate) const SNDSTRM_SLOT_BASE: u32 = 0x0051_57b8;
/// Offset of the slot pointer array inside the table.
pub(crate) const SNDSTRM_SLOT_ARRAY: u32 = 0x304;
/// Stream slot: the stream name/handle word.
pub(crate) const SNDSTRM_SLOT_NAME: u32 = 0x0;
/// Stream slot: the buffer count byte.
pub(crate) const SNDSTRM_SLOT_COUNT: u32 = 0x62;
/// Stream slot: the 4-byte buffer pointer array.
pub(crate) const SNDSTRM_SLOT_BUFFERS: u32 = 0x64;

/// `int SNDSTRM_StopSlot(int index)` (0x003c6ce8): stop the stream in slot
/// `index`, stop that slot's handles, clear the slot's name and free every
/// buffer it still holds.
pub fn sndstrm_stop_slot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x51_0000;
        frame.save(c, S0, 0x30);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x57b8));
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
        frame.save(c, S1, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[S1][0] = Addr((c.g[V0][0] as u32).wrapping_add(SNDSTRM_SLOT_ARRAY)).lw(c);
        c.g[S2][0] = 0;
        c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
        SND_STOP.call(c, 0x003c_6d24)?;
        SNDSTRM_RUN_CALLBACKS.call(c, 0x003c_6d2c)?;
        c.g[A2][0] = 0x7fff_0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = c.g[A2][0] | 0xffff;
        SNDSTRM_STOP_SLOT_UNSUPPORTED.call(c, 0x003c_6d40)?;
        c.g[V1][0] = u64::MAX;
        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(SNDSTRM_SLOT_COUNT)).lbu(c);
        Addr(c.g[S1][0] as u32).sw(c, c.g[V1][0]);
        let empty = c.g[V0][0] == 0;
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(SNDSTRM_SLOT_BUFFERS));
        if !empty {
            c.cycles += 1;
            loop {
                c.cycles += 3;
                c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                if c.g[A0][0] != 0 {
                    c.cycles += 3;
                    SND_MEM_FREE.call(c, 0x003c_6d6c)?;
                }
                c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(SNDSTRM_SLOT_COUNT)).lbu(c);
                c.cycles += 4;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_6d58);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Rider_AddBoost(Rider* r, int kind, float amount)` (0x0010e098).
const RIDER_ADD_BOOST: Callee = Callee {
    addr: 0x0010_e098,
    f: ee::s00::Rider_AddBoost,
};

/// Rider: the rider-metrics pointer at +0x790.
pub(crate) const RIDER_METRIX_PTR: u32 = 0x790;

/// `void Rider_AddWipeoutRecoveryBoost(Rider* r)` (0x0010f280): convert the
/// recovery boost the rider earned into a boost.
pub fn rider_add_wipeout_recovery_boost(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(RIDER_METRIX_PTR)).lw(c);
        // `RiderMetrix_GetRecoveryBoost` has no override slot: call its
        // generated entry.
        Callee {
            addr: 0x0011_9bb0,
            f: ee::s00::f_00119bb0,
        }
        .call(c, 0x0010_f298)?;
        c.g[A0][0] = c.g[S0][0];
        c.mov_s(12, 0);
        c.g[A1][0] = 0x1;
        RIDER_ADD_BOOST.call(c, 0x0010_f2a8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `unsigned int DLHeap_MoreCore(DLHeapList* list, long incr)` (0x0031ffd8):
/// dlmalloc's morecore hook. A negative `incr` moves the list's current offset
/// back by it and returns the new offset; otherwise returns the old offset
/// (storing `old + incr`) or -1 when the list has no room left.
pub fn dlheap_more_core(c: &mut Cpu) {
    c.cycles += 2;
    let list = abi::arg(c, 0);
    c.g[A2][0] = c.g[A0][0];
    let incr = c.g[A1][0] as i64;
    let base = list; // a2 == a0
    if incr < 0 {
        c.cycles += 6;
        let cur = Addr(base.wrapping_add(0x8));
        c.g[V0][0] = cur.lw(c);
        c.g[V1][0] = sx(c.g[A1][0] as u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        let ra = c.g[RA][0] as u32;
        cur.sw(c, c.g[V0][0]);
        c.pc = ra;
    } else {
        c.cycles += 10;
        c.g[A3][0] = Addr(base.wrapping_add(0x8)).lw(c);
        c.g[V0][0] = sx(c.g[A1][0] as u32);
        c.g[V1][0] = Addr(base.wrapping_add(0x4)).lw(c);
        c.g[A0][0] = Addr(base.wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
        if c.g[V1][0] != 0 {
            // Past the end of the list.
            c.cycles += 3;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            abi::ret(c);
        } else {
            c.cycles += 3;
            let ra = c.g[RA][0] as u32;
            Addr(base.wrapping_add(0x8)).sw(c, c.g[V0][0]);
            c.g[V0][0] = c.g[A3][0];
            c.pc = ra;
        }
    }
}

/// `float RiderMetrix_ReturnZero_a0c0(void)` (0x0011a0c0): return 0.0f (the
/// FPU result is the only one the caller reads; v0 is left alone).
pub fn rider_metrix_return_zero_a0c0(c: &mut Cpu) {
    c.cycles += 3;
    c.mtc1(0, 0);
    abi::ret(c);
}

/// `PositionModifier* PositionModifier_ctor_2(PositionModifier* self,
/// const Matrix* m)` (0x00356f10): install the position-modifier vtable, copy
/// the 4x4 matrix at `m` into the modifier's fields at +0x50 (VU0) and its
/// first row to the stack slot, and set the "position set" flag at +0x44.
pub fn position_modifier_ctor_2(c: &mut Cpu) {
    c.cycles += 21;
    c.g[V0][0] = 0x490000;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x10));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xa10));
    c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x50));
    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
    c.lq(2, (c.g[A1][0] as u32).wrapping_add(0x30));
    c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x30));
    c.sq(2, sp(c));
    c.lqc2(1, c.g[A1][0] as u32);
    c.lqc2(2, (c.g[A1][0] as u32).wrapping_add(0x10));
    c.lqc2(3, (c.g[A1][0] as u32).wrapping_add(0x20));
    c.lqc2(4, (c.g[A1][0] as u32).wrapping_add(0x30));
    c.sqc2(1, c.g[A2][0] as u32);
    c.sqc2(2, (c.g[A2][0] as u32).wrapping_add(0x10));
    c.sqc2(3, (c.g[A2][0] as u32).wrapping_add(0x20));
    c.sqc2(4, (c.g[A2][0] as u32).wrapping_add(0x30));
    c.g[V1][0] = 0x1;
    c.g[V0][0] = c.g[A0][0];
    c.sw((c.g[A0][0] as u32).wrapping_add(0x44), c.g[V1][0]);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
    c.pc = ra;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fill, put_vec, scramble, Rng, DATA, GAME_GP};

    use crate::difftest::fakes::fake;
    use crate::difftest::fakes::{covered, machine};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Once;

    /// Where this module's tests keep the forced callee results: +0 the
    /// magic, then per-test words.
    pub(crate) const OC336_CTRL: u32 = DATA + 0x7_0000;

    pub(crate) const OC336_MAGIC: u32 = 0x11a1_68f0;

    fn oc336_fall_through(c: &mut Cpu, prev: &AtomicUsize, gen: fn(&mut Cpu)) {
        let p = prev.load(Ordering::Relaxed);
        if p == 0 {
            return gen(c);
        }
        // SAFETY: `p` was read from the same slot (a `fn(&mut Cpu)`).
        let f = unsafe { std::mem::transmute::<usize, fn(&mut Cpu)>(p) };
        f(c)
    }

    /// A gated logging fake: `fake` logs every register the caller set up and
    /// returns to `ra` (or unwinds at the setup's call index). When the
    /// machine's magic word is absent, it falls through to whatever the slot
    /// held (or the generated body), so other tests are not affected.
    macro_rules! oc336_fake {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC336_CTRL) != OC336_MAGIC {
                    return oc336_fall_through(c, &$prev, $gen);
                }
                let ra = c.g[RA][0] as u32;
                fake(c, $addr, ra);
            }
        };
    }

    /// Install gated fakes in the given override slots, once per test.
    macro_rules! oc336_install {
        ($($slot:expr => $f:path => $prev:ident),* $(,)?) => {{
            static ONCE: Once = Once::new();
            ONCE.call_once(|| unsafe {
                let hook = |slot: *mut fn(&mut Cpu), f: fn(&mut Cpu), prev: &AtomicUsize| {
                    let cur = *slot;
                    if cur as usize != f as usize {
                        prev.store(cur as usize, Ordering::Relaxed);
                        *slot = f;
                    }
                };
                $(hook(addr_of_mut!($slot), $f, &$prev);)*
            });
        }};
    }

    static ANNOUNCE_PREV_LOOKUP: AtomicUsize = AtomicUsize::new(0);
    static ANNOUNCE_PREV_REPEATS: AtomicUsize = AtomicUsize::new(0);

    /// `Trick_LookupBonusTrick`: log the call, then return the setup's points
    /// (0, -1 or a positive count) so both branches of the caller are
    /// reached.
    fn announce_fake_lookup_bonus(c: &mut Cpu) {
        if c.bus.read32(OC336_CTRL) != OC336_MAGIC {
            return oc336_fall_through(c, &ANNOUNCE_PREV_LOOKUP, ee::s00::f_0011b1a8__generated);
        }
        let ra = c.g[RA][0] as u32;
        let points = c.bus.read32(OC336_CTRL + 4) as i32;
        fake(c, 0x0011_b1a8, ra);
        if c.pc == ra {
            c.g[V0][0] = sx(points as u32);
        }
    }

    /// `RiderMetrix_CountTrickRepeats`: log the call, then return the setup's
    /// repeat count (zero or non-zero).
    fn announce_fake_count_repeats(c: &mut Cpu) {
        if c.bus.read32(OC336_CTRL) != OC336_MAGIC {
            return oc336_fall_through(c, &ANNOUNCE_PREV_REPEATS, ee::s00::f_001190f0__generated);
        }
        let ra = c.g[RA][0] as u32;
        let repeats = c.bus.read32(OC336_CTRL + 8);
        fake(c, 0x0011_90f0, ra);
        if c.pc == ra {
            c.g[V0][0] = sx(repeats);
        }
    }

    oc336_fake!(
        announce_fake_show_hud,
        ANNOUNCE_PREV_HUD,
        0x0011_7b88,
        ee::s00::f_00117b88__generated
    );

    oc336_fake!(
        announce_fake_bonus_trick,
        ANNOUNCE_PREV_BONUS_TRICK,
        0x0029_b7e0,
        ee::s08::f_0029b7e0__generated
    );

    oc336_fake!(
        announce_fake_show_name,
        ANNOUNCE_PREV_NAME,
        0x0011_8ff8,
        ee::s00::f_00118ff8__generated
    );

    fn rider_metrix_announce_trick_install() {
        oc336_install!(
            ee::s00::OVERRIDE_0011B1A8 => announce_fake_lookup_bonus => ANNOUNCE_PREV_LOOKUP,
            ee::s00::OVERRIDE_001190F0 => announce_fake_count_repeats => ANNOUNCE_PREV_REPEATS,
            ee::s00::OVERRIDE_00117B88 => announce_fake_show_hud => ANNOUNCE_PREV_HUD,
            ee::s08::OVERRIDE_0029B7E0 => announce_fake_bonus_trick => ANNOUNCE_PREV_BONUS_TRICK,
            ee::s00::OVERRIDE_00118FF8 => announce_fake_show_name => ANNOUNCE_PREV_NAME,
        );
    }

    /// `RiderMetrix_AnnounceTrick`: no trick (the descriptor builder returns
    /// 0), a trick with no bonus points, and one with points (the score, the
    /// HUD number and the bonus-trick sound); the repeat count feeds the
    /// trick-name display. The descriptor builder is real (`self+0x70` makes
    /// it short-circuit), the other callees are logging fakes.
    #[test]
    fn rider_metrix_announce_trick_matches() {
        rider_metrix_announce_trick_install();
        const SELF: u32 = DATA + 0x100;
        const AUDIO: u32 = DATA + 0x1000;
        covered(
            ee::s00::f_0011a168__generated,
            rider_metrix_announce_trick,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0011_a168, 5);
                c.pc = 0x0011_a168;
                c.bus.write32(OC336_CTRL, OC336_MAGIC);
                c.bus.write32(
                    OC336_CTRL + 4,
                    match seed % 3 {
                        0 => 0,
                        1 => 0xffff_ffff,
                        _ => 1 + r.next_u32() % 0x1000,
                    },
                );
                c.bus.write32(OC336_CTRL + 8, (seed % 2) as u32);
                c.bus.write32(GAME_GP.wrapping_add(0x410), AUDIO);
                // The real .lit4 constants the descriptor builder and the
                // score use: degrees -> radians, and 1e-4f.
                c.bus.write32(GAME_GP.wrapping_sub(0x79e4), 0x4265_2ee0);
                c.bus.write32(GAME_GP.wrapping_sub(0x79f8), 0x38d1_b717);
                c.bus.write32(SELF + 0x14, r.edge_f32());
                c.bus.write32(SELF + 0x1ac, AUDIO + 0x100);
                // The descriptor builder short-circuits on +0x70; with 0 it
                // walks the trick bits: no rotation at all (no trick), or a
                // small angle (a spin, so the trick path runs).
                let state = if seed % 4 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 0x10
                };
                c.bus.write32(SELF + RIDER_METRIX_TRICK_STATE, state);
                c.bus.write32(SELF + 0xc, r.next_u32() % 8);
                let no_rotation = state == 0 && seed % 8 == 0;
                c.bus
                    .write32(SELF + 0x34, if no_rotation { 0 } else { r.f32(700.0) });
                c.bus
                    .write32(SELF + 0x38, if no_rotation { 0 } else { r.f32(1400.0) });
                // Fields that would take the descriptor builder's other
                // (state-machine) paths stay clear.
                for off in [
                    0x0u32, 0x4, 0x8, 0x10, 0x20, 0x24, 0x28, 0x2c, 0x60, 0x64, 0x68, 0x7c,
                ] {
                    c.bus.write32(SELF + off, 0);
                }
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = (seed % 2) as u64; // stance
                c.g[A2][0] = ((seed / 2) % 2) as u64; // alternate
                c.g[A3][0] = ((seed / 4) % 2) as u64; // style
                c.g[T0][0] = (seed % 3) as u64; // flag
            },
        );
    }

    /// `WorldTriggerInstance_Stop`: a crowd source (stopped through
    /// `CrowdAudio_RemoveSource`), a sound instance (stopped through
    /// `BankMonitor_StopHandle`) and neither (no call), each with and without
    /// a valid handle.
    #[test]
    fn world_trigger_instance_stop_matches() {
        const SELF: u32 = DATA + 0x100;
        const AUDIO: u32 = DATA + 0x1000;
        const STREAMS: u32 = DATA + 0x2000;
        const REQUEST: u32 = DATA + 0x3000;
        check(
            ee::s08::f_002b5758__generated,
            world_trigger_instance_stop,
            120,
            &|c, seed| {
                let mut r = Rng(0x002b_5758 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                // The audio singleton and its stream list.
                c.bus.write32(GAME_GP.wrapping_add(0x410), AUDIO);
                c.bus.write32(AUDIO + 0x18, 0); // no crowd source: short path
                c.bus.write32(AUDIO + 0x118, STREAMS);
                c.bus.write32(STREAMS, REQUEST);
                // No bank-monitor instances in the request's monitor.
                c.bus.write32(REQUEST + 0x1d8 + 0x8ec, 0);
                // The trigger instance.
                c.bus.write32(
                    SELF + WORLD_TRIGGER_HANDLE,
                    match seed % 3 {
                        0 => 0xffff_d8f1, // -9999: nothing to stop
                        _ => r.next_u32() % 0x100,
                    },
                );
                c.bus.write32(SELF + WORLD_TRIGGER_CROWD, (seed % 2) as u32);
                c.bus.write32(SELF + WORLD_TRIGGER_ACTIVE, r.next_u32());
                c.bus.write32(SELF + 0x60, r.next_u32());
                c.bus.write32(SELF + 0x64, r.next_u32());
                c.bus.write32(SELF + 0x68, r.edge_f32());
                c.bus.write32(SELF + 0x6c, r.next_u32());
                c.g[A0][0] = sx(SELF);
            },
        );
    }

    /// `DLHeap_MoreCore`: a negative increment moves the offset back, a
    /// positive one returns the old offset unless the list is full (-1).
    #[test]
    fn dlheap_more_core_matches() {
        check(ee::s10::f_0031ffd8__generated, dlheap_more_core, 200, &|c, seed| {
            let mut r = Rng(0x31ff_d800 ^ seed as u64);
            let list = DATA;
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = sx(list);
            let incr = match seed % 3 {
                0 => -(1 + (r.next_u32() % 0x1000) as i64),
                1 => (r.next_u32() % 0x1000) as i64,
                _ => (r.next_u32() % 0x10000) as i64 - 0x8000,
            };
            c.g[A1][0] = sx(incr as u32);
            c.bus.write32(list, r.next_u32());
            c.bus.write32(list + 4, r.next_u32());
            c.bus.write32(list + 8, r.next_u32());
            c.bus.write32(list + 0xc, r.next_u32());
            // Half the positive seeds use a list with no room left.
            if seed % 2 == 0 {
                c.bus.write32(list + 4, (r.next_u32() % 0x100) as u32);
                c.bus
                    .write32(list + 8, 0x100 - (r.next_u32() % 0x10) as u32);
                c.bus.write32(list + 0xc, r.next_u32() % 0x20);
            }
            c.pc = 0x0031_ffd8;
        });
    }

    /// `RiderMetrix_ReturnZero_a0c0`: the FPU result is 0.0f.
    #[test]
    fn rider_metrix_return_zero_a0c0_matches() {
        check(
            ee::s00::f_0011a0c0__generated,
            rider_metrix_return_zero_a0c0,
            50,
            &|c, seed| {
                let mut r = Rng(0x0011_a0c0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
            },
        );
    }

    /// `PositionModifier_ctor_2`: vtable, the matrix copy through VU0 and the
    /// flag at +0x44.
    #[test]
    fn position_modifier_ctor_2_matches() {
        check(
            ee::s11::f_00356f10__generated,
            position_modifier_ctor_2,
            50,
            &|c, seed| {
                let mut r = Rng(0x0035_6f10 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let this = DATA + 0x100;
                let m = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(m);
                fill(c, &mut r, this, 0x30);
                for i in 0..4 {
                    put_vec(
                        c,
                        m + 0x10 * i,
                        [r.f32(1e4), r.f32(1e4), r.f32(1e4), r.f32(2.0)],
                    );
                }
            },
        );
    }

    // ------------------------------------------------------- ModObj modifiers

    oc336_fake!(
        modifier_fake_alloc,
        MODIFIER_PREV_ALLOC,
        0x0031_7d70,
        ee::s10::f_00317d70__generated
    );

    oc336_fake!(
        modifier_fake_spline_ctor,
        MODIFIER_PREV_SPLINE_CTOR,
        0x0035_9460,
        ee::s11::f_00359460__generated
    );

    oc336_fake!(
        modifier_fake_set_transform,
        MODIFIER_PREV_SET_TRANSFORM,
        0x0035_54b0,
        ee::s11::f_003554b0__generated
    );

    oc336_fake!(
        modifier_fake_create_block,
        MODIFIER_PREV_CREATE_BLOCK,
        0x0035_5468,
        ee::s11::f_00355468__generated
    );

    fn mod_obj_add_spline_modifier_install() {
        oc336_install!(
            ee::s10::OVERRIDE_00317D70 => modifier_fake_alloc => MODIFIER_PREV_ALLOC,
            ee::s11::OVERRIDE_00359460 => modifier_fake_spline_ctor => MODIFIER_PREV_SPLINE_CTOR,
            ee::s11::OVERRIDE_003554B0 => modifier_fake_set_transform => MODIFIER_PREV_SET_TRANSFORM,
        );
    }

    /// `ModObj_AddSplineModifier`: the allocator, the spline-modifier
    /// constructor and `ModObj_SetTransformModifier` are logging fakes, so
    /// every argument the original passes is compared.
    #[test]
    fn mod_obj_add_spline_modifier_matches() {
        mod_obj_add_spline_modifier_install();
        const THIS: u32 = DATA + 0x100;
        const DESC: u32 = DATA + 0x200;
        covered(
            ee::s11::f_00355ad0__generated,
            mod_obj_add_spline_modifier,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0035_5ad0, 3);
                c.bus.write32(OC336_CTRL, OC336_MAGIC);
                fill(c, &mut r, THIS, 0x40);
                fill(c, &mut r, DESC, 0x40);
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = sx(DESC);
            },
        );
    }

    // `ModObj_AddPositionModifierFromMatrix`: same three calls, but the
    // position-modifier constructor has no override slot (it is generated or
    // the port itself), so only the allocator and the transform setter are
    // faked.
    fn mod_obj_add_position_modifier_from_matrix_install() {
        oc336_install!(
            ee::s10::OVERRIDE_00317D70 => modifier_fake_alloc => MODIFIER_PREV_ALLOC,
            ee::s11::OVERRIDE_003554B0 => modifier_fake_set_transform => MODIFIER_PREV_SET_TRANSFORM,
        );
    }

    /// `ModObj_AddPositionModifierFromMatrix`: the allocator and
    /// `ModObj_SetTransformModifier` are logging fakes; the real position
    /// constructor copies the matrix from the descriptor.
    #[test]
    fn mod_obj_add_position_modifier_from_matrix_matches() {
        mod_obj_add_position_modifier_from_matrix_install();
        const THIS: u32 = DATA + 0x100;
        const MATRIX: u32 = DATA + 0x400;
        covered(
            ee::s11::f_00355918__generated,
            mod_obj_add_position_modifier_from_matrix,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0035_5918, 2);
                c.bus.write32(OC336_CTRL, OC336_MAGIC);
                fill(c, &mut r, THIS, 0x40);
                for i in 0..4 {
                    put_vec(
                        c,
                        MATRIX + 0x10 * i,
                        [r.f32(1e3), r.f32(1e3), r.f32(1e3), r.f32(1.0)],
                    );
                }
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = sx(MATRIX);
            },
        );
    }

    /// `ModObj_TakeParticleModifiers`: no block on the source (no call), no
    /// block on the target (created, a fake) and a block with an empty
    /// particle list (the real `ModifierBlock_TakeParticleList`).
    #[test]
    fn mod_obj_take_particle_modifiers_matches() {
        oc336_install!(
            ee::s11::OVERRIDE_00355468 => modifier_fake_create_block => MODIFIER_PREV_CREATE_BLOCK,
        );
        const TARGET: u32 = DATA + 0x100;
        const SOURCE: u32 = DATA + 0x200;
        const SRC_BLOCK: u32 = DATA + 0x1000;
        const DST_BLOCK: u32 = DATA + 0x2000;
        covered(
            ee::s11::f_00356020__generated,
            mod_obj_take_particle_modifiers,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0035_6020, 1);
                c.bus.write32(OC336_CTRL, OC336_MAGIC);
                let has_source = seed % 4 != 0;
                let has_target = seed % 2 == 0;
                c.bus
                    .write32(SOURCE + 0x1c, if has_source { SRC_BLOCK } else { 0 });
                c.bus
                    .write32(TARGET + 0x1c, if has_target { DST_BLOCK } else { 0 });
                // Empty particle lists on both blocks.
                c.bus.write32(SRC_BLOCK + 0x10, 0);
                c.bus.write32(DST_BLOCK + 0x10, 0);
                fill(c, &mut r, SRC_BLOCK + 0x20, 0x10);
                fill(c, &mut r, DST_BLOCK + 0x20, 0x10);
                c.g[A0][0] = sx(TARGET);
                c.g[A1][0] = sx(SOURCE);
            },
        );
    }

    // ------------------------------------------------------------ SNDSTRM

    oc336_fake!(
        sndstrm_fake_stop,
        SNDSTRM_PREV_STOP,
        0x003b_89e0,
        ee::s13::f_003b89e0__generated
    );

    oc336_fake!(
        sndstrm_fake_callbacks,
        SNDSTRM_PREV_CALLBACKS,
        0x003c_7080,
        ee::s13::f_003c7080__generated
    );

    oc336_fake!(
        sndstrm_fake_stop_unsupported,
        SNDSTRM_PREV_UNSUPPORTED,
        0x003c_6ce0,
        ee::s13::f_003c6ce0__generated
    );

    oc336_fake!(
        sndstrm_fake_mem_free,
        SNDSTRM_PREV_MEM_FREE,
        0x003b_5e98,
        ee::s13::f_003b5e98__generated
    );

    fn sndstrm_stop_slot_install() {
        oc336_install!(
            ee::s13::OVERRIDE_003B89E0 => sndstrm_fake_stop => SNDSTRM_PREV_STOP,
            ee::s13::OVERRIDE_003C7080 => sndstrm_fake_callbacks => SNDSTRM_PREV_CALLBACKS,
            ee::s13::OVERRIDE_003C6CE0 => sndstrm_fake_stop_unsupported => SNDSTRM_PREV_UNSUPPORTED,
            ee::s13::OVERRIDE_003B5E98 => sndstrm_fake_mem_free => SNDSTRM_PREV_MEM_FREE,
        );
    }

    /// `SNDSTRM_StopSlot`: an empty slot, a slot with only its name (no
    /// buffers) and a slot with 1..5 buffers (some NULL, so some are freed);
    /// `SND_Stop`, `SNDSTRM_RunCallbacks`,
    /// `SNDSTRM_StopSlotUnsupported` and `SND_MemFree` are logging fakes.
    #[test]
    fn sndstrm_stop_slot_matches() {
        sndstrm_stop_slot_install();
        const SLOT: u32 = DATA + 0x1000;
        covered(ee::s13::f_003c6ce8__generated, sndstrm_stop_slot, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x003c_6ce8, 4);
            c.bus.write32(OC336_CTRL, OC336_MAGIC);
            let index = seed % 4;
            c.bus
                .write32(SNDSTRM_SLOT_BASE + 4 * index + SNDSTRM_SLOT_ARRAY, SLOT);
            let count = (seed % 6) as u8;
            c.bus.write32(SLOT + SNDSTRM_SLOT_NAME, r.next_u32());
            c.bus.write8(SLOT + SNDSTRM_SLOT_COUNT, count);
            for i in 0..count as u32 {
                let buf = if (seed + i as u32) % 3 == 0 {
                    0
                } else {
                    DATA + 0x2000 + 0x10 * i
                };
                c.bus.write32(SLOT + SNDSTRM_SLOT_BUFFERS + 4 * i, buf);
            }
            for i in count as u32..8 {
                c.bus
                    .write32(SLOT + SNDSTRM_SLOT_BUFFERS + 4 * i, r.next_u32());
            }
            c.g[A0][0] = sx(index as u32);
        });
    }

    // ------------------------------------------------------------- rider

    oc336_fake!(
        rider_fake_add_boost,
        RIDER_PREV_ADD_BOOST,
        0x0010_e098,
        ee::s00::f_0010e098__generated
    );

    fn rider_add_wipeout_recovery_boost_install() {
        oc336_install!(
            ee::s00::OVERRIDE_0010E098 => rider_fake_add_boost => RIDER_PREV_ADD_BOOST,
        );
    }

    /// `Rider_AddWipeoutRecoveryBoost`: the real
    /// `RiderMetrix_GetRecoveryBoost` on a metrics block (a1 = 0 keeps it on
    /// its short path) returns the boost, which goes to `Rider_AddBoost` (a
    /// logging fake).
    #[test]
    fn rider_add_wipeout_recovery_boost_matches() {
        rider_add_wipeout_recovery_boost_install();
        const RIDER: u32 = DATA + 0x100;
        const METRIX: u32 = DATA + 0x1000;
        covered(
            ee::s00::f_0010f280__generated,
            rider_add_wipeout_recovery_boost,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_f280, 1);
                c.bus.write32(OC336_CTRL, OC336_MAGIC);
                c.bus.write32(RIDER + RIDER_METRIX_PTR, METRIX);
                fill(c, &mut r, METRIX, 0x80);
                c.g[A0][0] = sx(RIDER);
                c.g[A1][0] = 0;
            },
        );
    }
}
