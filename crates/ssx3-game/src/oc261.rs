//! Staging wave 208 (oc261): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0029_4678, ssx_audio_system_update_rider_air_state, "SSXAudioSystem_UpdateRiderAirState"),
    (0x0031_abd0, ps2_exec_man_vblank_signal, "PS2ExecMan_VBlankSignal"),
    (0x003d_54e8, path_update_tick42, "PATH_UpdateTick42"),
    (0x002a_77c8, crowd_audio_get_instance_reaction, "CrowdAudio_GetInstanceReaction"),
    (0x003a_8a90, c_world_cache_alloc_resource, "cWorldCache_AllocResource"),
    (0x0022_5be8, luno_vm_register_write, "LunoVMRegister_Write"),
    (0x003d_85c8, spch_match_conditions, "SPCH_MatchConditions"),
    (0x0032_8660, spatial_hash_insert_at_key, "SpatialHash_InsertAtKey"),
    (0x0016_6530, chase_camera_check_distance_reset_wrapper, "ChaseCameraAlgorithm_CheckDistanceReset_Wrapper"),
];

// -- 0x00294678 SSXAudioSystem_UpdateRiderAirState ---------------------------

/// Rider fields read by `UpdateRiderAirState`.
pub(crate) mod rider_air_state {
    /// Rider: the air-state object (+0x6c0, its vtable first; slot 0x38 =
    /// the rider's audio slot index).
    pub const RIDER_AIR_OBJ: u32 = 0x6c0;
    /// Rider: the animation/physics state (+0x788).
    pub const RIDER_ANIM: u32 = 0x788;
    /// Rider: player id (+0x870), human flags (+0x874, +0x87c).
    pub const RIDER_PLAYER_ID: u32 = 0x870;
    pub const RIDER_HUMAN: u32 = 0x874;
    pub const RIDER_HUMAN2: u32 = 0x87c;
    /// Anim state: air time (+0x98, float), air mode (+0xac: 1 or 3 = airborne).
    pub const ANIM_AIR_TIME: u32 = 0x98;
    pub const ANIM_AIR_MODE: u32 = 0xac;
    /// The air-state object's vtable slot giving the rider's audio slot.
    pub const VT_SLOT: u32 = 0x38;
}

/// `void SSXAudioSystem_UpdateRiderAirState(SSXAudioSystem* self, Rider*
/// rider)` (0x294678): when the rider is no longer airborne (air mode not 1
/// or 3), clear its airborne flag (`SetRiderAirFlag(self, slot, 1)`, the
/// slot from the rider's air-state virtual). Otherwise keep the flag set
/// (`SetRiderAirFlag(self, slot, 0)`), let the rider say line 0x2083, and
/// when the followed rider (`FindRider(self, idx)`, idx 0 in replays, the
/// player id of a human rider, else -1) is this rider and its air time is
/// over 2 s, start the crowd's anticipation for it
/// (`CrowdAudio_StartAnticipate(self, idx)`). Frame 0x50 (s0-s2, f20).
pub fn ssx_audio_system_update_rider_air_state(c: &mut Cpu) {
    use rider_air_state::*;
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x40);
        let rider = r32(c, S0);
        let anim = Addr(rider.wrapping_add(RIDER_ANIM)).lw(c);
        let mode = Addr((anim as u32).wrapping_add(ANIM_AIR_MODE)).lw(c);
        c.g[V1][0] = mode;
        let airborne = if mode == 1 {
            c.cycles += 1;
            true
        } else {
            c.cycles += 3;
            c.g[V0][0] = 3;
            if mode == 3 {
                c.cycles += 1;
            }
            mode == 3
        };
        c.g[A0][0] = airborne as u64;

        c.cycles += 2;
        let vt = Addr(rider.wrapping_add(RIDER_AIR_OBJ)).lw(c);
        c.g[V1][0] = vt;
        if !airborne {
            // Landed: clear the rider's airborne flag.
            c.cycles += 11;
            c.g[A1][0] = sx(rider.wrapping_add(RIDER_AIR_OBJ));
            let delta = Addr((vt as u32).wrapping_add(VT_SLOT)).lh(c);
            let method = Addr((vt as u32).wrapping_add(VT_SLOT + 4)).lw(c);
            c.g[V0][0] = method;
            c.g[A0][0] = abi::addu(c.g[A1][0], delta);
            call_ptr(c, method as u32, 0x0029_46d8)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = 1;
            Callee { addr: 0x0029_4880, f: ee::s07::f_00294880 }.call(c, 0x0029_46e8)?;
            frame.restore(c, S0, 0x30);
        } else {
            c.cycles += 19;
            c.g[A1][0] = sx(rider.wrapping_add(RIDER_AIR_OBJ));
            c.g[S2][0] = u64::MAX;
            let delta = Addr((vt as u32).wrapping_add(VT_SLOT)).lh(c);
            let method = Addr((vt as u32).wrapping_add(VT_SLOT + 4)).lw(c);
            c.g[V0][0] = method;
            c.g[A0][0] = abi::addu(c.g[A1][0], delta);
            call_ptr(c, method as u32, 0x0029_4708)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = 0;
            Callee { addr: 0x0029_4880, f: ee::s07::f_00294880 }.call(c, 0x0029_4718)?;
            // f20 = the rider's air time.
            let anim = Addr(r32(c, S0).wrapping_add(RIDER_ANIM)).lw(c);
            c.g[V0][0] = anim;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            let air_time = Addr((anim as u32).wrapping_add(ANIM_AIR_TIME)).read_fpu(c);
            abi::set_fpr(c, 20, air_time);
            // SSXAudioSystem_RiderSay2083(self, rider)
            call_ptr(c, 0x0029_fcc8, 0x0029_472c)?;
            c.g[A0][0] = c.g[S1][0];
            Callee { addr: 0x0028_8ae0, f: ee::s07::f_00288ae0 }.call(c, 0x0029_4734)?;
            if c.g[V0][0] != 0 {
                // Replay: the followed rider is rider 0.
                c.cycles += 2;
                c.g[S2][0] = 0;
            } else {
                let rider = r32(c, S0);
                let human = Addr(rider.wrapping_add(RIDER_HUMAN)).lw(c);
                c.g[V0][0] = human;
                c.cycles += 2;
                c.g[V1][0] = 0;
                if human != 0 {
                    c.cycles += 2;
                    let human2 = Addr(rider.wrapping_add(RIDER_HUMAN2)).lw(c);
                    c.g[V0][0] = human2;
                    c.g[V1][0] = (human2 != 0) as u64;
                }
                c.cycles += 2;
                if c.g[V1][0] != 0 {
                    c.g[S2][0] = Addr(rider.wrapping_add(RIDER_PLAYER_ID)).lw(c);
                }
            }
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S2][0];
            Callee { addr: 0x0028_5d98, f: ee::s07::f_00285d98 }.call(c, 0x0029_4768)?;
            let followed = c.g[V0][0] == c.g[S0][0];
            frame.restore(c, S0, 0x30);
            if followed {
                c.cycles += 6;
                c.g[AT][0] = 0x4000_0000;
                let two = FpuF32(0x4000_0000);
                abi::set_fpr(c, 0, two);
                let air_time = FpuF32(c.f[20]);
                let long_air = abi::c_lt_s(c, two, air_time);
                c.g[A0][0] = c.g[S1][0];
                if long_air {
                    c.cycles += 3;
                    c.g[A1][0] = c.g[S2][0];
                    // CrowdAudio_StartAnticipate(self, idx)
                    call_ptr(c, 0x002a_7408, 0x0029_4790)?;
                    frame.restore(c, S0, 0x30);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

// -- 0x0031abd0 PS2ExecMan_VBlankSignal --------------------------------------

/// Exec manager: "VSync thread busy" flag (+0x4038) and its VBlank
/// semaphore id (+0x4034).
pub(crate) const EXEC_VSYNC_BUSY: u32 = 0x4038;
pub(crate) const EXEC_VSYNC_SEMA: u32 = 0x4034;

/// `void PS2ExecMan_VBlankSignal(void)` (0x31abd0), the VBlank handler: when
/// the exec manager's VSync thread is idle (+0x4038 == 0), wake it with
/// `iSignalSema(sema)` (+0x4034). Frame 0x10 (ra).
pub fn ps2_exec_man_vblank_signal(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        // App_GetExecMan()
        Callee { addr: 0x0031_7520, f: ee::s10::f_00317520 }.call(c, 0x0031_abe0)?;
        let exec = c.g[V0][0];
        c.g[V1][0] = exec;
        let busy = Addr((exec as u32).wrapping_add(EXEC_VSYNC_BUSY)).lw(c);
        c.g[V0][0] = busy;
        frame.restore_ra(c, 0);
        if busy == 0 {
            c.cycles += 3;
            c.g[A0][0] = Addr((exec as u32).wrapping_add(EXEC_VSYNC_SEMA)).lw(c);
            // iSignalSema(sema)
            call_ptr(c, 0x0042_3dd0, 0x0031_abf8)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// -- 0x003d54e8 PATH_UpdateTick42 --------------------------------------------

/// `void PATH_UpdateTick42(void)` (0x3d54e8): the music-path timer tick,
/// `PATH_Update('B')`. Frame 0x10 (ra).
pub fn path_update_tick42(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x42;
        Callee { addr: 0x003d_5128, f: ee::s14::f_003d5128 }.call(c, 0x003d_54f8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// -- 0x002a77c8 CrowdAudio_GetInstanceReaction -------------------------------

/// Crowd audio: instance count (+0x14) and instance array (+0x18) of
/// 0x190-byte instances whose reaction is at +0x18c.
pub(crate) const CROWD_INSTANCE_COUNT: u32 = 0x14;
pub(crate) const CROWD_INSTANCES: u32 = 0x18;
pub(crate) const CROWD_INSTANCE_SIZE: u32 = 0x190;
pub(crate) const CROWD_INSTANCE_REACTION: u32 = 0x18c;

/// `int CrowdAudio_GetInstanceReaction(CrowdAudio* self, int i)` (0x2a77c8):
/// the reaction of crowd instance `i` (signed `i < count`), else 0. Leaf.
pub fn crowd_audio_get_instance_reaction(c: &mut Cpu) {
    c.cycles += 4;
    let crowd = abi::arg(c, 0);
    let count = Addr(crowd.wrapping_add(CROWD_INSTANCE_COUNT)).lw(c);
    let in_range = (c.g[A1][0] as i64) < (count as i64);
    c.g[V0][0] = in_range as u64;
    c.g[V1][0] = CROWD_INSTANCE_SIZE as u64;
    if in_range {
        c.cycles += 5;
        let base = Addr(crowd.wrapping_add(CROWD_INSTANCES)).lw(c);
        let i = abi::arg(c, 1);
        let off = abi::mult(c, i, CROWD_INSTANCE_SIZE);
        let inst = (base as u32).wrapping_add(off);
        c.g[V1][0] = sx(inst);
        c.g[V0][0] = Addr(inst.wrapping_add(CROWD_INSTANCE_REACTION)).lw(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0;
    }
    ret(c);
}

// -- 0x003a8a90 cWorldCache_AllocResource ------------------------------------

/// World cache layout used by `cWorldCache_AllocResource`.
pub(crate) mod world_cache_alloc {
    /// Cache: the global bank (+0x4), the bank table (+0x8), the bank
    /// section allocators (+0xc, each bank's allocator at +0x3c), and the
    /// per-slot load lists (slot * 0x18 + 0x3f8).
    pub const CACHE_GLOBAL_BANK: u32 = 0x4;
    pub const CACHE_BANKS: u32 = 0x8;
    pub const CACHE_SECTIONS: u32 = 0xc;
    pub const SECTION_ALLOCATOR: u32 = 0x3c;
    pub const CACHE_SLOT_LISTS: u32 = 0x3f8;
    pub const CACHE_SLOT_SIZE: u32 = 0x18;
    /// Bank: per resource type (+4 + type * 8) the reference-count bytes
    /// (one word per id).
    pub const BANK_TYPES: u32 = 0x4;
    /// Load list: count (+0x8) and items (+0xc) of `{u8 type, u8 bank, u16
    /// id}`.
    pub const LIST_COUNT: u32 = 0x8;
    pub const LIST_ITEMS: u32 = 0xc;
    /// The global bank's key (`key & 0xff`).
    pub const GLOBAL_BANK: u64 = 0xff;
    /// `switch (type - 1)` jump table (14 entries) and its targets.
    pub const TYPE_TABLE: u32 = 0x0049_4eb0;
    pub const CASE_STREAM: u32 = 0x003a_8bb8;
    pub const CASE_SHARED: u32 = 0x003a_8bec;
    pub const CASE_AIPATH: u32 = 0x003a_8c2c;
    pub const CASE_DEFAULT: u32 = 0x003a_8c3c;
    /// Allocation names: "BxStream SHAREDMEM", "BxStream AIPATH".
    pub const NAME_SHAREDMEM: u32 = 0x0049_4d40;
    pub const NAME_AIPATH: u32 = 0x0049_4e98;
}

/// How `cWorldCache_AllocResource` allocates a resource type.
#[derive(Clone, Copy)]
enum WorldCacheAllocKind {
    /// Bank section allocator (or shared memory for the global bank) with
    /// these flags; the case's extra cycles before the section call.
    Flags(u64, u64),
    /// `operator new(size, "BxStream AIPATH", 0, 0)`.
    AiPath,
    /// Bank section allocator with flags 0 (shared memory for the global
    /// bank).
    Default,
}

/// `void* cWorldCache_AllocResource(cWorldCache* cache, int slot, int type,
/// int key, int size)` (0x3a8a90): reference resource `type`/`key` (bank
/// `key & 0xff`, 0xff = global; id `key >> 8`). Already referenced: bump the
/// count, append `{type, bank, id}` to slot `slot`'s load list and return
/// 0. New: with `size` 0 return 0; else allocate `size` bytes (types 9/10
/// with flags 0x45000000, 1-3/11/12 with 0x22000000, 14 named AIPATH, others
/// 0) from the bank's section allocator, or for the global bank from shared
/// memory (`operator new`); on failure drop the reference again and return
/// -1. Frame 0x40 (s0-s2).
pub fn c_world_cache_alloc_resource(c: &mut Cpu) {
    use world_cache_alloc::*;
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        c.g[T2][0] = c.g[A3][0];
        c.g[T1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let key = c.g[T2][0];
        let bank = key & 0xff;
        c.g[A3][0] = bank;
        frame.save(c, S2, 0x10);
        let id = sx(key as u32 >> 8);
        c.g[A0][0] = id;
        frame.save_ra(c, 0);
        c.g[V0][0] = 0xff;
        c.g[T3][0] = c.g[A2][0];
        let cache = r32(c, T1);
        let bank_obj = if bank == GLOBAL_BANK {
            c.cycles += 2;
            Addr(cache.wrapping_add(CACHE_GLOBAL_BANK)).lw(c)
        } else {
            c.cycles += 4;
            let banks = Addr(cache.wrapping_add(CACHE_BANKS)).lw(c);
            c.g[V1][0] = banks;
            Addr(((bank as u32) << 2).wrapping_add(banks as u32)).lw(c)
        };

        // The reference count byte of this resource.
        c.cycles += 12;
        let types = sx((bank_obj as u32).wrapping_add(BANK_TYPES));
        let type_off = sx((c.g[T3][0] as u32) << 3);
        c.g[S1][0] = types;
        c.g[S2][0] = type_off;
        let refs = Addr((types as u32).wrapping_add(type_off as u32)).lw(c);
        let id_off = sx((id as u32) << 2);
        c.g[S0][0] = id_off;
        let ref_addr = (id_off as u32).wrapping_add(refs as u32);
        c.g[A0][0] = sx(ref_addr);
        let refcount = Addr(ref_addr).lbu(c);
        let referenced = refcount != 0;
        c.cycles += 2;
        c.g[V0][0] = sx((refcount as u32).wrapping_add(1));
        c.g[V1][0] = referenced as u64;
        c.cycles += 2;
        let bumped = c.g[V0][0];
        Addr(ref_addr).sb(c, bumped);

        let size = c.g[T0][0];
        if referenced {
            // Already loaded: record it in the slot's load list.
            c.cycles += 25;
            c.g[A0][0] = CACHE_SLOT_SIZE as u64;
            let id = sx(key as u32 >> 8);
            c.g[A3][0] = id;
            let slot = abi::arg(c, 1);
            let slot_off = abi::mult(c, slot, CACHE_SLOT_SIZE);
            c.g[V0][0] = sx(slot_off);
            let list = Addr(slot_off.wrapping_add(cache).wrapping_add(CACHE_SLOT_LISTS)).lw(c) as u32;
            c.g[A2][0] = sx(list);
            c.g[V0][0] = 0;
            let item = |c: &mut Cpu| {
                let n = Addr(list.wrapping_add(LIST_COUNT)).lw(c) as u32;
                let items = Addr(list.wrapping_add(LIST_ITEMS)).lw(c) as u32;
                (n, items, (n << 2).wrapping_add(items))
            };
            let (_, items, at) = item(c);
            c.g[A0][0] = sx(items);
            c.g[V1][0] = sx(at);
            let ty = c.g[T3][0];
            Addr(at).sb(c, ty);
            let (_, items, at) = item(c);
            c.g[V1][0] = sx(items);
            c.g[A0][0] = sx(at);
            Addr(at.wrapping_add(1)).sb(c, key);
            let (_, items, at) = item(c);
            c.g[V1][0] = sx(items);
            c.g[A1][0] = sx(at);
            Addr(at.wrapping_add(2)).sh(c, id);
            let n = Addr(list.wrapping_add(LIST_COUNT)).lw(c);
            let n = sx((n as u32).wrapping_add(1));
            c.g[V1][0] = n;
            Addr(list.wrapping_add(LIST_COUNT)).sw(c, n);
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        } else {
            c.cycles += 2;
            let case = sx((c.g[T3][0] as u32).wrapping_sub(1));
            c.g[V1][0] = case;
            if size == 0 {
                c.cycles += 2;
                c.g[V0][0] = 0;
                c.cycles += 1;
                frame.restore(c, S0, 0x30);
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x0049_0000;
                let kind = if case < 0xe {
                    c.cycles += 6;
                    c.g[V0][0] = sx(TYPE_TABLE);
                    let entry = ((case as u32) << 2).wrapping_add(TYPE_TABLE);
                    c.g[V1][0] = sx(entry);
                    let target = Addr(entry).lw(c);
                    c.g[A0][0] = target;
                    match target as u32 {
                        CASE_STREAM => WorldCacheAllocKind::Flags(5, 0x4500_0000),
                        CASE_SHARED => WorldCacheAllocKind::Flags(4, 0x2200_0000),
                        CASE_AIPATH => WorldCacheAllocKind::AiPath,
                        CASE_DEFAULT => WorldCacheAllocKind::Default,
                        t => {
                            c.jump_out(t, 0x003a_8a90, 0x003a_8cd0);
                            return Ok(());
                        }
                    }
                } else {
                    WorldCacheAllocKind::Default
                };
                // (Some(flags, extra cycles, return address)) for the bank
                // section allocator, None for operator new (a1/a2 set).
                let section = match kind {
                    WorldCacheAllocKind::Flags(extra, flags) => {
                        c.cycles += 3;
                        c.g[V0][0] = 0xff;
                        c.g[A0][0] = Addr(cache.wrapping_add(CACHE_SECTIONS)).lw(c);
                        if bank != GLOBAL_BANK {
                            Some((extra, flags, 0x003a_8c24, 4))
                        } else {
                            c.cycles += 5;
                            c.g[A1][0] = sx(NAME_SHAREDMEM);
                            c.g[A0][0] = size;
                            c.g[A2][0] = flags;
                            None
                        }
                    }
                    WorldCacheAllocKind::AiPath => {
                        c.cycles += 4;
                        c.g[A1][0] = sx(NAME_AIPATH);
                        c.g[A0][0] = size;
                        c.cycles += 1;
                        c.g[A2][0] = 0;
                        None
                    }
                    WorldCacheAllocKind::Default => {
                        c.cycles += 3;
                        c.g[V0][0] = 0xff;
                        c.g[A0][0] = Addr(cache.wrapping_add(CACHE_SECTIONS)).lw(c);
                        if bank != GLOBAL_BANK {
                            Some((6, 0, 0x003a_8c80, 0))
                        } else {
                            c.cycles += 3;
                            c.g[A1][0] = sx(NAME_SHAREDMEM);
                            c.g[A0][0] = size;
                            c.cycles += 1;
                            c.g[A2][0] = 0;
                            None
                        }
                    }
                };
                match section {
                    Some((extra, flags, ret_addr, call_cycles)) => {
                        // SectionAllocator_Alloc(sections[bank], size, flags)
                        c.cycles += extra;
                        let sections = c.g[A0][0];
                        let at = sx(((bank as u32) << 2).wrapping_add(sections as u32));
                        c.g[V0][0] = at;
                        c.g[A1][0] = size;
                        c.g[A2][0] = flags;
                        c.cycles += call_cycles;
                        c.g[A0][0] = Addr((at as u32).wrapping_add(SECTION_ALLOCATOR)).lw(c);
                        Callee { addr: 0x003a_7878, f: ee::s13::f_003a7878 }.call(c, ret_addr)?;
                    }
                    None => {
                        // operator new(size, name, flags, 0)
                        c.cycles += 4;
                        c.g[A3][0] = 0;
                        call_ptr(c, 0x0031_7e30, 0x003a_8c60)?;
                    }
                }
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x30);
                } else {
                    // Out of memory: drop the reference again.
                    c.cycles += 8;
                    let at = abi::addu(c.g[S1][0], c.g[S2][0]);
                    c.g[V0][0] = at;
                    let refs = Addr(at as u32).lw(c);
                    c.g[V1][0] = refs;
                    let ref_addr = abi::addu(c.g[S0][0], refs) as u32;
                    c.g[A0][0] = sx(ref_addr);
                    let n = Addr(ref_addr).lbu(c);
                    let n = sx((n as u32).wrapping_sub(1));
                    c.g[V0][0] = n;
                    c.g[V1][0] = n & 0xff;
                    Addr(ref_addr).sb(c, n);
                    if n & 0xff == 0 {
                        c.cycles += 2;
                        c.g[V0][0] = 0;
                        Addr(ref_addr).sw(c, 0);
                    }
                    c.cycles += 2;
                    c.g[V0][0] = u64::MAX;
                    c.cycles += 1;
                    frame.restore(c, S0, 0x30);
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// -- 0x00225be8 LunoVMRegister_Write -----------------------------------------

/// Luno value: the payload word (+0) and the type tag (+8; 4 = table,
/// 5 = pointer). Output stream: vtable slot 0x8 = `Write(this, data, n)`.
pub(crate) const LUNO_VALUE_TYPE: u32 = 0x8;
pub(crate) const LUNO_TYPE_TABLE: u64 = 4;
pub(crate) const LUNO_TYPE_PTR: u64 = 5;
pub(crate) const LUNO_STREAM_WRITE: u32 = 0x8;

/// `void LunoVMRegister_Write(cLunoValue* self, Stream* stream, int base)`
/// (0x225be8): save a Luno register: write its type tag (4 bytes), then a
/// pointer relative to `base` (`LunoVMRegister_WritePtr`), a table
/// (`cLunoTable_Write`) or the 4-byte payload. Frame 0x40 (s0-s2).
pub fn luno_vm_register_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        // stream->Write(&self->type, 4)
        c.g[A1][0] = sx(r32(c, S0).wrapping_add(LUNO_VALUE_TYPE));
        c.g[A2][0] = 4;
        let vt = Addr(r32(c, S1)).lw(c);
        c.g[V1][0] = vt;
        let delta = Addr((vt as u32).wrapping_add(LUNO_STREAM_WRITE)).lh(c);
        let write = Addr((vt as u32).wrapping_add(LUNO_STREAM_WRITE + 4)).lw(c);
        c.g[V0][0] = write;
        c.g[A0][0] = abi::addu(c.g[S1][0], delta);
        call_ptr(c, write as u32, 0x0022_5c24)?;
        let ty = Addr(r32(c, S0).wrapping_add(LUNO_VALUE_TYPE)).lw(c);
        c.g[V1][0] = ty;
        c.g[V0][0] = LUNO_TYPE_TABLE;
        if ty == LUNO_TYPE_PTR {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S2][0];
            Callee { addr: 0x0022_4c00, f: ee::s05::f_00224c00 }.call(c, 0x0022_5c44)?;
        } else {
            c.cycles += 2;
            if ty == LUNO_TYPE_TABLE {
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.g[S2][0];
                Callee { addr: 0x0022_57e0, f: ee::s05::f_002257e0 }.call(c, 0x0022_5c64)?;
            } else {
                // stream->Write(self, 4)
                let vt = Addr(r32(c, S1)).lw(c);
                c.g[V0][0] = vt;
                c.cycles += 7;
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = 4;
                let delta = Addr((vt as u32).wrapping_add(LUNO_STREAM_WRITE)).lh(c);
                let write = Addr((vt as u32).wrapping_add(LUNO_STREAM_WRITE + 4)).lw(c);
                c.g[V1][0] = write;
                c.g[A0][0] = abi::addu(c.g[S1][0], delta);
                call_ptr(c, write as u32, 0x0022_5c84)?;
            }
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// -- 0x003d85c8 SPCH_MatchConditions -----------------------------------------

/// Speech condition record (a1 of `SPCH_MatchConditions`): count (+4,
/// signed byte, at most 4), per condition its kind (+8 + i: 0/0xfe = always,
/// 0xff = a byte of the bank data or of the context, else an index into the
/// value table), its bank data offset (+0x10 + i, bit 7 = from the context)
/// and its accepted-value mask (+0x14 + 4 i).
pub(crate) mod spch_match {
    pub const COND_COUNT: u32 = 0x4;
    pub const COND_KIND: u32 = 0x8;
    pub const COND_OFFSET: u32 = 0x10;
    pub const COND_MASKS: u32 = 0x14;
    pub const KIND_ALWAYS: u64 = 0;
    pub const KIND_ALWAYS2: u64 = 0xfe;
    pub const KIND_DATA: u64 = 0xff;
    /// Context (a0): two size bytes (+4, +6) placing its value table.
    pub const CTX_SIZE_A: u32 = 0x4;
    pub const CTX_SIZE_B: u32 = 0x6;
    /// The loop head (event checks).
    pub const LOOP: u32 = 0x003d_8630;
}

/// `bool SPCH_MatchConditions(Ctx* ctx, Cond* cond, u32* values, u8*
/// actual)` (0x3d85c8): true when every condition `i < count` (count at
/// most 4, else false) accepts its actual value `actual[i]` (< 32, else
/// false): either the condition does not care (mask bit clear, kind 0 or
/// 0xfe), or bit `actual[i]` is set in the condition's value word
/// (`values[kind]`, or for kind 0xff a word of the context's table (offset
/// bit 7) or the bank data byte at the offset, looked up with
/// `SPCH_FindBankDataPtr(*values, &data)`, which must equal `actual[i]`).
/// Frame 0xe0 (s0-s7, fp, ra as quadwords; a2/t0 kept around the call).
pub fn spch_match_conditions(c: &mut Cpu) {
    use spch_match::*;
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, FP, 0xc0);
        frame.save(c, S7, 0xb0);
        c.g[FP][0] = c.g[A1][0];
        frame.save(c, S1, 0x50);
        c.g[S7][0] = c.g[A0][0];
        frame.save(c, RA, 0xd0);
        c.g[S1][0] = 1;
        frame.save(c, S6, 0xa0);
        frame.save(c, S5, 0x90);
        frame.save(c, S4, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S2, 0x60);
        frame.save(c, S0, 0x40);
        let count = Addr(r32(c, FP).wrapping_add(COND_COUNT)).lb(c);
        c.g[T0][0] = count;
        c.g[V0][0] = ((count as i64) < 5) as u64;
        let actual_at = frame.at(c, 0x14);
        let actual = c.g[A3][0];
        Addr(actual_at).sw(c, actual);
        if (count as i64) >= 5 {
            c.cycles += 2;
            c.g[S1][0] = 0;
        } else {
            c.cycles += 2;
            c.g[S3][0] = 0;
            if (count as i64) > 0 {
                c.cycles += 4;
                c.g[S6][0] = sx(0xffff_fffc);
                c.g[S5][0] = sx(r32(c, FP).wrapping_add(COND_MASKS));
                loop {
                    if !spch_match_condition(c, frame)? {
                        // Actual value out of range: fail at once.
                        break;
                    }
                    c.cycles += 2;
                    c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
                    if c.g[S1][0] == 0 {
                        break;
                    }
                    c.cycles += 3;
                    let more = (c.g[S3][0] as i64) < (c.g[T0][0] as i64);
                    c.g[V0][0] = more as u64;
                    c.g[S5][0] = sx(r32(c, S5).wrapping_add(4));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, LOOP);
                    }
                }
            }
        }
        c.cycles += 13;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, RA, 0xd0);
        frame.restore(c, FP, 0xc0);
        frame.restore(c, S7, 0xb0);
        frame.restore(c, S6, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore(c, S4, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S0, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// One condition of `SPCH_MatchConditions` (index s3, mask word at s5):
/// leaves s1 = 1 if it accepts the actual value, else 0, up to the loop
/// tail (block 0x3d8738). `false`: the actual value is out of range and the
/// function returns at once (s1 = 0, no tail).
fn spch_match_condition(c: &mut Cpu, frame: Frame) -> Result<bool, crate::call::Unwound> {
    use spch_match::*;
    c.cycles += 6;
    let actual = Addr(frame.at(c, 0x14)).lw(c);
    c.g[V0][0] = actual;
    let at = sx((actual as u32).wrapping_add(r32(c, S3)));
    c.g[V1][0] = at;
    let value = Addr(at as u32).lbu(c);
    c.g[S2][0] = value;
    c.g[V0][0] = (value < 0x20) as u64;
    c.g[S1][0] = 0;
    if value >= 0x20 {
        return Ok(false);
    }
    c.cycles += 7;
    c.g[S4][0] = 1;
    let mask = Addr(r32(c, S5)).lw(c);
    let bit = sx(1u32 << (value as u32 & 31));
    c.g[A1][0] = bit;
    let cond = sx(r32(c, FP).wrapping_add(r32(c, S3)));
    c.g[V0][0] = cond;
    c.g[V1][0] = bit & mask;
    let kind = Addr((cond as u32).wrapping_add(COND_KIND)).lbu(c);
    c.g[A0][0] = kind;
    if bit & mask == 0 {
        return Ok(true);
    }
    c.cycles += 2;
    c.g[V0][0] = KIND_ALWAYS2;
    let always = if kind == KIND_ALWAYS {
        true
    } else {
        c.cycles += 2;
        c.g[V0][0] = KIND_DATA;
        kind == KIND_ALWAYS2
    };
    if always {
        c.cycles += 2;
        c.g[S1][0] = 1;
        return Ok(true);
    }
    c.cycles += 2;
    let index_off = sx((kind as u32) << 2);
    c.g[V1][0] = index_off;
    let word = if kind != KIND_DATA {
        // values[kind]
        c.cycles += 3;
        c.g[S1][0] = c.g[S4][0];
        let at = sx((index_off as u32).wrapping_add(r32(c, A2)));
        c.g[V1][0] = at;
        Addr(at as u32).lw(c)
    } else {
        c.cycles += 5;
        let cond = sx(r32(c, FP).wrapping_add(r32(c, S3)));
        c.g[V0][0] = cond;
        let offset = Addr((cond as u32).wrapping_add(COND_OFFSET)).lbu(c);
        c.g[S0][0] = offset;
        c.g[V1][0] = offset & 0x80;
        c.g[A0][0] = c.g[SP][0];
        if offset & 0x80 == 0 {
            // The bank data byte at `offset` must equal the actual value.
            c.cycles += 11;
            let values = r32(c, A2);
            let sp_ = sp(c);
            c.lwl(V1, values.wrapping_add(3));
            c.lwr(V1, values);
            c.swl(V1, sp_.wrapping_add(3));
            c.swr(V1, sp_);
            c.g[A1][0] = sx(sp_.wrapping_add(0x10));
            frame.save(c, A2, 0x20);
            frame.save(c, T0, 0x30);
            // SPCH_FindBankDataPtr(&id, &data)
            Callee { addr: 0x003d_83b8, f: ee::s14::f_003d83b8 }.call(c, 0x003d_86fc)?;
            frame.restore(c, A2, 0x20);
            frame.restore(c, T0, 0x30);
            if c.g[V0][0] != 0 {
                c.cycles += 7;
                let data = Addr(frame.at(c, 0x10)).lw(c);
                c.g[S1][0] = c.g[S4][0];
                let at = sx((data as u32).wrapping_add(r32(c, S0)));
                c.g[V1][0] = at;
                let byte = Addr(at as u32).lbu(c);
                let diff = c.g[S2][0] ^ byte;
                c.g[V0][0] = diff;
                if diff != 0 {
                    c.g[S1][0] = 0;
                }
            }
            return Ok(true);
        }
        // A word of the context's value table.
        c.cycles += 17;
        let ctx = r32(c, S7);
        let size_a = Addr(ctx.wrapping_add(CTX_SIZE_A)).lbu(c);
        let offset = offset & 0x7f;
        c.g[S0][0] = offset;
        let size_b = Addr(ctx.wrapping_add(CTX_SIZE_B)).lbu(c);
        c.g[S1][0] = c.g[S4][0];
        let s6 = c.g[S6][0];
        let a = sx(((size_a as u32 >> 2).wrapping_add(3) as u64 & s6) as u32);
        let b = sx(((size_b as u32) << 2).wrapping_add(3) as u64 as u32) & s6;
        c.g[V0][0] = b;
        let table = sx(ctx.wrapping_add((a as u32).wrapping_add(8)).wrapping_add(b as u32));
        c.g[V1][0] = table;
        let at = sx(((offset as u32) << 2).wrapping_add(table as u32));
        c.g[A0][0] = at;
        Addr(at as u32).lw(c)
    };
    c.g[V0][0] = word;
    c.cycles += 2;
    let hit = c.g[A1][0] & word;
    c.g[V0][0] = hit;
    if hit == 0 {
        c.g[S1][0] = 0;
    }
    Ok(true)
}

// -- 0x00328660 SpatialHash_InsertAtKey --------------------------------------

/// Spatial hash octree node (0x2c bytes): eight children
/// `child[dx][dy][dz]` (+0, dx * 0x10 + dy * 8 + dz * 4) and three item
/// lists (+0x20 + 4 * list). A cell key is `{level, x, y, z}` (16 bytes).
pub(crate) mod spatial_insert {
    pub const NODE_LISTS: u32 = 0x20;
    pub const NODE_SIZE: u64 = 0x2c;
    /// Item: next (+0), prev (+4).
    pub const ITEM_PREV: u32 = 0x4;
    /// `EA_MemAllocNamed` name (0x4a3fc0) and flags for a new node.
    pub const NODE_NAME: u32 = 0x004a_3fc0;
    pub const NODE_FLAGS: u64 = 0x2000_0000;
    /// Loop heads (event checks).
    pub const DESCEND: u32 = 0x0032_86c0;
    pub const CLEAR: u32 = 0x0032_8778;
}

/// `void SpatialHash_InsertAtKey(Node* node, int list, Item* item, Cell*
/// target, Cell* at)` (0x328660): walk down the octree from `node` (whose
/// cell is `*at`) to the cell `target`, one level at a time, creating
/// missing child nodes (`EA_MemAllocNamed(0x2c, name, 0x20000000, 0)`,
/// lists cleared, children `memset` to 0), then push `item` onto the
/// target node's list `list`. Frame 0x80 (s0-s5; the walked cell at sp+0).
pub fn spatial_hash_insert_at_key(c: &mut Cpu) {
    use spatial_insert::*;
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S4, 0x30);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S0, 0x70);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S1, 0x60);
        frame.save(c, S5, 0x20);
        frame.save_ra(c, 0x10);
        // The walked cell: a copy of *at at sp+0.
        let at = r32(c, T0);
        let cell = sp(c);
        c.ldl(V0, at.wrapping_add(7));
        c.ldr(V0, at);
        c.ldl(V1, at.wrapping_add(0xf));
        c.ldr(V1, at.wrapping_add(8));
        c.sdl(V0, cell.wrapping_add(7));
        c.sdr(V0, cell);
        c.sdl(V1, cell.wrapping_add(0xf));
        c.sdr(V1, cell.wrapping_add(8));
        let target = r32(c, S2);
        c.g[V1][0] = Addr(target).lw(c);
        c.g[V0][0] = Addr(cell).lw(c);
        c.g[T2][0] = c.g[A0][0];
        if c.g[V0][0] != c.g[V1][0] {
            c.cycles += 1;
            c.g[S5][0] = 0x004a_0000;
            loop {
                spatial_hash_insert_descend(c)?;
                c.cycles += 3;
                let target_level = Addr(r32(c, S2)).lw(c);
                c.g[V0][0] = target_level;
                c.g[T2][0] = Addr(r32(c, S1)).lw(c);
                if c.g[V1][0] == target_level {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, DESCEND);
                }
            }
        }
        // Push the item onto the node's list.
        c.cycles += 6;
        let off = sx(r32(c, S4) << 2);
        c.g[V0][0] = off;
        let head_at = (off as u32).wrapping_add(r32(c, T2)).wrapping_add(NODE_LISTS);
        c.g[V1][0] = sx(head_at);
        let head = Addr(head_at).lw(c);
        c.g[V0][0] = head;
        let item = r32(c, S3);
        if head != 0 {
            Addr((head as u32).wrapping_add(ITEM_PREV)).sw(c, item as u64);
        }
        c.cycles += 13;
        let head = Addr(head_at).lw(c);
        c.g[V0][0] = head;
        Addr(item.wrapping_add(ITEM_PREV)).sw(c, 0);
        Addr(item).sw(c, head);
        Addr(head_at).sw(c, item as u64);
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// One level down (block 0x3286c0 up to 0x3287ac): step the walked cell
/// one level toward the target, select the child (s1 = its slot), create
/// it if missing. Leaves v1 = the new level.
fn spatial_hash_insert_descend(c: &mut Cpu) -> crate::call::Flow {
    use spatial_insert::*;
    c.cycles += 36;
    let cell = sp(c);
    let target = r32(c, S2);
    let level = Addr(cell).lw(c) as u32;
    let target_level = Addr(target).lw(c) as u32;
    c.g[T1][0] = sx(level.wrapping_sub(1));
    let sh = level.wrapping_sub(target_level).wrapping_sub(1);
    c.g[V1][0] = sx(sh);
    let tx = Addr(target.wrapping_add(4)).lw(c) as u32;
    let ty = Addr(target.wrapping_add(8)).lw(c) as u32;
    let tz = Addr(target.wrapping_add(0xc)).lw(c) as u32;
    let cx2 = (Addr(cell.wrapping_add(4)).lw(c) as u32) << 1;
    let cy2 = (Addr(cell.wrapping_add(8)).lw(c) as u32) << 1;
    let cz2 = (Addr(cell.wrapping_add(0xc)).lw(c) as u32) << 1;
    let shr = |v: u32| ((v as i32) >> (sh & 31)) as u32;
    let dx = shr(tx).wrapping_sub(cx2);
    let dy = shr(ty).wrapping_sub(cy2);
    let dz = shr(tz).wrapping_sub(cz2);
    let nx = cx2.wrapping_add(dx);
    let ny = cy2.wrapping_add(dy);
    let nz = cz2.wrapping_add(dz);
    c.g[A1][0] = sx(dx);
    c.g[T0][0] = sx(nx);
    c.g[A2][0] = sx(ny);
    c.g[A3][0] = sx(nz);
    let row = (dx << 4).wrapping_add(r32(c, T2)).wrapping_add(dy << 3);
    Addr(cell).sw(c, sx(level.wrapping_sub(1)));
    Addr(cell.wrapping_add(4)).sw(c, sx(nx));
    let dz4 = dz << 2;
    c.g[A0][0] = sx(dz4);
    Addr(cell.wrapping_add(8)).sw(c, sx(ny));
    let slot = row.wrapping_add(dz4);
    c.g[S1][0] = sx(slot);
    Addr(cell.wrapping_add(0xc)).sw(c, sx(nz));
    let child = Addr(slot).lw(c);
    c.g[V0][0] = child;
    c.g[V1][0] = Addr(cell).lw(c);
    if child == 0 {
        // A new node: lists cleared, children zeroed.
        c.cycles += 10;
        c.g[A0][0] = NODE_SIZE;
        c.g[A1][0] = sx(r32(c, S5).wrapping_add(NODE_NAME & 0xffff));
        c.g[A2][0] = NODE_FLAGS;
        c.g[A3][0] = 0;
        Callee { addr: 0x0031_7d70, f: ee::s10::f_00317d70 }.call(c, 0x0032_8764)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = u64::MAX;
        c.g[V0][0] = sx(r32(c, S0).wrapping_add(NODE_LISTS));
        c.g[V1][0] = 2;
        loop {
            c.cycles += 7;
            Addr(r32(c, V0)).sw(c, 0);
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
            let more = c.g[V1][0] != c.g[A0][0];
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, CLEAR);
            }
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x20;
        // memset(node, 0, 0x20)
        call_ptr(c, 0x003e_6448, 0x0032_87a4)?;
        let node = c.g[S0][0];
        Addr(r32(c, S1)).sw(c, node);
        c.g[V1][0] = Addr(sp(c)).lw(c);
    }
    Ok(())
}

// -- 0x00166530 ChaseCameraAlgorithm_CheckDistanceReset_Wrapper --------------

/// `void ChaseCameraAlgorithm_CheckDistanceReset_Wrapper(Chase* self)`
/// (0x166530): forwards to `ChaseCameraAlgorithm_CheckDistanceReset(self)`
/// (0x168150). Frame 0x10 (ra).
pub fn chase_camera_check_distance_reset_wrapper(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Direct(Callee { addr: 0x0016_8150, f: ee::s02::f_00168150 }),
        0x0016_6540,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- 0x00294678 SSXAudioSystem_UpdateRiderAirState ------------------------

    /// `SSXAudioSystem_UpdateRiderAirState` against the generated body: air
    /// mode 1/3 (airborne) or other (landed); the rider's slot virtual is a
    /// fake (some unwind, some forced to a slot index); the real
    /// `RiderSay2083` stops at once (speech channel volume 0); the real
    /// `IsReplayPlaying` (replay states none/0/3/12) and `FindRider` over up
    /// to three race riders (which may or may not be this rider); human flags
    /// and player ids; air time below or above 2.0; the real
    /// `CrowdAudio_StartAnticipate` with or without a crowd array.
    #[test]
    fn ssx_audio_system_update_rider_air_state_matches() {
        use super::rider_air_state::*;
        use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
        use crate::difftest::{DATA, GAME_GP};
        const THIS: u32 = DATA + 0x1_0000;
        const WORLD: u32 = DATA + 0x2_0000;
        const MODULE: u32 = DATA + 0x2_0100;
        const RACE_MAN: u32 = DATA + 0x2_0400;
        const REPLAY: u32 = DATA + 0x2_0800;
        const CROWD: u32 = DATA + 0x4_0000;
        const RIDERS: u32 = DATA + 0x5_0000;
        covered(ee::s07::f_00294678__generated, ssx_audio_system_update_rider_air_state, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x0029_4678, 1);
            c.pc = 0x0029_4678;
            let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
            w(c, GAME_GP - 0x848, WORLD);
            w(c, WORLD + 0x84, MODULE);
            w(c, MODULE + 0xc, RACE_MAN);
            w(c, MODULE + 0x28, if r.next_u32() % 3 == 0 { REPLAY } else { 0 });
            w(c, REPLAY, [0, 0, 3, 12][(r.next_u32() % 4) as usize]);
            w(c, RACE_MAN + 0x40, REPLAY);
            w(c, RACE_MAN + 0x7c, r.next_u32() % 4);
            w(c, RACE_MAN + 0x78, r.next_u32() % 4);
            for i in 0..3u32 {
                let rd = RIDERS + 0x1000 * i;
                let anim = rd + 0x800;
                w(c, RACE_MAN + 0x28 + 4 * i, rd);
                w(c, rd + RIDER_ANIM, anim);
                w(c, anim + ANIM_AIR_MODE, [1, 3, 1, 3, 0, 2][(r.next_u32() % 6) as usize]);
                w(c, anim + ANIM_AIR_TIME, r.f32(4.0));
                w(c, rd + RIDER_HUMAN, r.next_u32() % 2);
                w(c, rd + RIDER_HUMAN2, r.next_u32() % 2);
                w(c, rd + RIDER_PLAYER_ID, (r.next_u32() % 4).wrapping_sub(1));
                w(c, rd + 0xec, (r.next_u32() % 8).wrapping_sub(1));
                let vt = rd + 0xc00;
                c.bus.write16(vt + VT_SLOT, (r.next_u32() as i16 & !3) as u16);
                w(c, vt + VT_SLOT + 4, FAKE);
                w(c, rd + RIDER_AIR_OBJ, vt);
            }
            let k = r.next_u32() % 3;
            w(c, REPLAY + 0x18, RIDERS + 0x1000 * (r.next_u32() % 3));
            // Speech channel (4) volume 0: RiderSay2083 returns at once.
            w(c, THIS + 0x62bc + 0x40, 0);
            w(c, THIS + 0x18, if r.next_u32() % 4 == 0 { 0 } else { CROWD });
            // Fake slot results: a pointer (no flag) or a slot index 1..5.
            let v = if r.next_u32() % 2 == 0 { 0 } else { 1 + r.next_u32() % 5 };
            w(c, LOG_RESULTS, v);
            c.g[A0][0] = sx(THIS);
            c.g[A1][0] = sx(RIDERS + 0x1000 * k);
        });
    }

    // -- 0x0031abd0 PS2ExecMan_VBlankSignal ------------------------------------

    /// `PS2ExecMan_VBlankSignal` against the generated body: the VSync thread
    /// busy (no signal) or idle (the real `iSignalSema`, whose syscall is a
    /// fake: some unwind).
    #[test]
    fn ps2_exec_man_vblank_signal_matches() {
        use crate::difftest::fakes::{covered, machine};
        use crate::difftest::{DATA, GAME_GP};
        const APP: u32 = DATA + 0x1_0000;
        const EXEC: u32 = DATA + 0x2_0000;
        covered(ee::s10::f_0031abd0__generated, ps2_exec_man_vblank_signal, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x0031_abd0, 1);
            c.pc = 0x0031_abd0;
            c.bus.write32(GAME_GP + 0x2a74, APP);
            c.bus.write32(APP + 8, EXEC);
            let busy = if r.next_u32() % 3 == 0 { r.next_u32() | 1 } else { 0 };
            c.bus.write32(EXEC + EXEC_VSYNC_BUSY, busy);
            c.bus.write32(EXEC + EXEC_VSYNC_SEMA, r.next_u32() % 64);
        });
    }

    // -- 0x003d54e8 PATH_UpdateTick42 ------------------------------------------

    /// `PATH_UpdateTick42` against the generated body, with the real
    /// `PATH_Update` (fixture as in `path_update_matches`): locked /
    /// uninitialised / busy early outs, and the full tick over four players
    /// that are inactive, active without a handle, or active with a handle.
    #[test]
    fn path_update_tick42_matches() {
        use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
        check(ee::s14::f_003d54e8__generated, path_update_tick42, 120, &|c, seed| {
            let mut r = Rng(0x003d_54e8 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.pc = 0x003d_54e8;
            let gate = seed % 6;
            c.bus.write8(0x0044_f42d, (gate == 0) as u8 * (1 + r.next_u32() as u8 % 3));
            c.bus.write8(0x0044_f42c, (gate != 1) as u8);
            c.bus.write32(0x0044_f430, if gate == 2 { r.next_u32() | 1 } else { 0 });
            c.bus.write32(0x0045_0dcc, r.next_u32()); // timer count
            c.bus.write32(0x0045_0dc8, 1 + r.next_u32() % 0x1000); // timer period
            for i in 0..4u32 {
                let p = 0x0051_7610 + 0x928 * i;
                let kind = r.next_u32() % 3;
                c.bus.write32(p + 0x34, (kind != 0) as u32 * (1 + r.next_u32() % 5));
                c.bus.write32(p + 0x30, if kind == 2 { DATA + 0x100 * i } else { 0 });
            }
        });
    }

    // -- 0x002a77c8 CrowdAudio_GetInstanceReaction -----------------------------

    /// `CrowdAudio_GetInstanceReaction` against the generated body: indices
    /// below, at and above the count, negative indices (and upper bits in
    /// a1 that the 64-bit compare sees); LO/HI compared too.
    #[test]
    fn crowd_audio_get_instance_reaction_matches() {
        use crate::difftest::{check_lohi, fill, scramble, Rng, DATA};
        const CROWD: u32 = DATA;
        const INSTANCES: u32 = DATA + 0x1000;
        check_lohi(ee::s08::f_002a77c8__generated, crowd_audio_get_instance_reaction, 300, &|c, seed| {
            let mut r = Rng(0x002a_77c8 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.pc = 0x002a_77c8;
            let count = r.next_u32() % 8;
            c.bus.write32(CROWD + CROWD_INSTANCE_COUNT, count);
            c.bus.write32(CROWD + CROWD_INSTANCES, INSTANCES);
            for i in 0..8 {
                fill(c, &mut r, INSTANCES + CROWD_INSTANCE_SIZE * i + CROWD_INSTANCE_REACTION, 1);
            }
            c.g[A0][0] = sx(CROWD);
            c.g[A1][0] = match seed % 4 {
                0 | 1 => (r.next_u32() % 8) as u64,
                2 => sx((r.next_u32() % 4).wrapping_neg()),
                _ => (r.next_u32() % 8) as u64 | (r.next_u32() as u64) << 32,
            };
        });
    }

    // -- 0x003a8a90 cWorldCache_AllocResource ----------------------------------

    /// `cWorldCache_AllocResource` against the generated body: the global
    /// bank or banks 0-3, every resource type 0-15 (all jump-table cases and
    /// the out-of-table default), reference counts 0 (new), 1 or 0xff
    /// (wrapping), size 0 or not; the real `SectionAllocator_Alloc` from a
    /// block with room (base 0 and nothing used: returns 0, so the reference
    /// is dropped again and -1 returned) and the real `operator new` on a
    /// test heap (`DLHeap_InitArena`; its mutex syscalls are fakes, some
    /// unwind). The jump table is the game's (0x494eb0).
    #[test]
    fn c_world_cache_alloc_resource_matches() {
        use super::world_cache_alloc::*;
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, put_words, DATA};
        use crate::game::bx::bxstring::tests::alloc_ready;
        // (alloc_ready's arena: DATA+0x1_0000..DATA+0x5_0000.)
        const CACHE: u32 = DATA + 0x6_0000;
        const BANK_TABLE: u32 = DATA + 0x6_0800;
        const SECTIONS: u32 = DATA + 0x6_0900;
        const BANKS: u32 = DATA + 0x6_1000;
        const ALLOCATORS: u32 = DATA + 0x6_2000;
        const BLOCKS: u32 = DATA + 0x6_3000;
        const LISTS: u32 = DATA + 0x6_4000;
        const REFS: u32 = DATA + 0x6_8000;
        const BLOCK_BASE: u32 = DATA + 0x8_0000;
        check(ee::s13::f_003a8a90__generated, c_world_cache_alloc_resource, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x003a_8a90, 3);
            alloc_ready(c);
            c.pc = 0x003a_8a90;
            put_words(
                c,
                TYPE_TABLE,
                &[
                    CASE_SHARED, CASE_SHARED, CASE_SHARED, CASE_DEFAULT, CASE_DEFAULT,
                    CASE_DEFAULT, CASE_DEFAULT, CASE_DEFAULT, CASE_STREAM, CASE_STREAM,
                    CASE_SHARED, CASE_SHARED, CASE_DEFAULT, CASE_AIPATH,
                ],
            );
            let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
            // Banks 0-3 and the global bank (index 4): per type a refs array.
            w(c, CACHE + CACHE_GLOBAL_BANK, BANKS + 0x400);
            w(c, CACHE + CACHE_BANKS, BANK_TABLE);
            w(c, CACHE + CACHE_SECTIONS, SECTIONS);
            for k in 0..5u32 {
                let bank = BANKS + 0x100 * k;
                if k < 4 {
                    w(c, BANK_TABLE + 4 * k, bank);
                }
                for ty in 0..16u32 {
                    w(c, bank + BANK_TYPES + 8 * ty, REFS + (k * 16 + ty) * 0x40);
                }
            }
            let bank = r.next_u32() % 5;
            let ty = r.next_u32() % 16;
            let id = r.next_u32() % 16;
            let refs = REFS + (bank * 16 + ty) * 0x40;
            let count = [0, 0, 1, 0xff][(r.next_u32() % 4) as usize];
            w(c, refs + 4 * id, count | r.next_u32() & 0xffff_ff00);
            // The bank's section allocator: one block with room.
            let alloc = ALLOCATORS + 0x40 * bank;
            let block = BLOCKS + 0x40 * bank;
            if bank < 4 {
                w(c, SECTIONS + 4 * bank + SECTION_ALLOCATOR, alloc);
            }
            w(c, alloc + 8, 1 + r.next_u32() % 3);
            w(c, alloc + 0x10, block);
            let empty = r.next_u32() % 3 == 0;
            w(c, block, if empty { 0 } else { BLOCK_BASE });
            w(c, block + 4, 0x1_0000);
            w(c, block + 8, if empty { 0 } else { r.next_u32() % 0x400 });
            w(c, block + 0xc, r.next_u32() % 0x400);
            // The slot's load list.
            let slot = r.next_u32() % 4;
            let list = LISTS + 0x40 * slot;
            w(c, CACHE + slot * CACHE_SLOT_SIZE + CACHE_SLOT_LISTS, list);
            w(c, list + LIST_COUNT, r.next_u32() % 8);
            w(c, list + LIST_ITEMS, list + 0x100);
            let key = (id << 8) | if bank == 4 { 0xff } else { bank };
            c.g[A0][0] = sx(CACHE);
            c.g[A1][0] = slot as u64;
            c.g[A2][0] = ty as u64;
            c.g[A3][0] = key as u64;
            c.g[T0][0] = if r.next_u32() % 5 == 0 { 0 } else { 1 + (r.next_u32() % 0x200) as u64 };
        });
    }

    // -- 0x00225be8 LunoVMRegister_Write ---------------------------------------

    /// `LunoVMRegister_Write` against the generated body: type tags 5
    /// (the real `LunoVMRegister_WritePtr`), 4 (the real `cLunoTable_Write`
    /// over a table with no hash part and no entries) and others (payload
    /// written directly). The stream's Write virtual is a fake (some calls
    /// unwind).
    #[test]
    fn luno_vm_register_write_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        use crate::difftest::DATA;
        const VALUE: u32 = DATA + 0x20_0000;
        const STREAM: u32 = DATA + 0x20_0100;
        const TABLE: u32 = DATA + 0x20_0200;
        covered(ee::s05::f_00225be8__generated, luno_vm_register_write, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x0022_5be8, 3);
            c.pc = 0x0022_5be8;
            let vt = vtable(c, &mut r, &[LUNO_STREAM_WRITE]);
            c.bus.write32(STREAM, vt);
            let ty = [4, 5, 4, 5, 0, 1, 2, 3, 6][(r.next_u32() % 9) as usize];
            c.bus.write32(VALUE + LUNO_VALUE_TYPE, ty);
            c.bus.write32(VALUE, if ty == 4 { TABLE } else { r.next_u32() });
            c.bus.write32(VALUE + 4, r.next_u32());
            // Table: no hash part (+0xc = -1), so nothing to count or write.
            c.bus.write32(TABLE + 0xc, u32::MAX);
            c.bus.write32(TABLE + 0x10, TABLE + 0x40);
            c.g[A0][0] = sx(VALUE);
            c.g[A1][0] = sx(STREAM);
            c.g[A2][0] = r.next_u32() as u64;
        });
    }

    // -- 0x003d85c8 SPCH_MatchConditions ---------------------------------------

    /// `SPCH_MatchConditions` against the generated body: condition counts
    /// -1..5, actual values in and out of range, masks that accept or not,
    /// kinds 0/0xfe (always), value-table indices, and 0xff (a context table
    /// word, or a bank data byte via the real `SPCH_FindBankDataPtr` over
    /// the eight bank slots at 0x44ffb8, found or not).
    #[test]
    fn spch_match_conditions_matches() {
        use super::spch_match::*;
        use crate::difftest::{check, scramble, Rng, DATA};
        const CTX: u32 = DATA + 0x1000;
        const COND: u32 = DATA + 0x2000;
        const VALUES: u32 = DATA + 0x3000;
        const ACTUAL: u32 = DATA + 0x4000;
        const BANKS: u32 = DATA + 0x5000;
        const BANK_SLOTS: u32 = 0x0045_0000 - 0x48;
        check(ee::s14::f_003d85c8__generated, spch_match_conditions, 300, &|c, seed| {
            let mut r = Rng(0x003d_85c8 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.pc = 0x003d_85c8;
            let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
            // Context: two size bytes, then its value table.
            c.bus.write8(CTX + CTX_SIZE_A, (r.next_u32() % 0x40) as u8);
            c.bus.write8(CTX + CTX_SIZE_B, (r.next_u32() % 0x20) as u8);
            for i in 0..0x80 {
                let v = if r.next_u32() % 3 == 0 { r.next_u32() } else { 0xf };
                w(c, CTX + 8 + 4 * i, v);
            }
            let count = [1, 2, 3, 4, 4, 4, 0, 5, 0xff][(r.next_u32() % 9) as usize];
            c.bus.write8(COND + COND_COUNT, count);
            for i in 0..4u32 {
                let kind = [0, 0xfe, 0xff, 0xff, 0xff, r.next_u32() % 8][(r.next_u32() % 6) as usize];
                c.bus.write8(COND + COND_KIND + i, kind as u8);
                c.bus.write8(COND + COND_OFFSET + i, ((r.next_u32() % 2) << 7 | r.next_u32() % 8) as u8);
                let mask = if r.next_u32() % 5 == 0 { r.next_u32() } else { u32::MAX };
                w(c, COND + COND_MASKS + 4 * i, mask);
                let actual = if r.next_u32() % 10 == 0 { 0x20 + r.next_u32() % 0xe0 } else { r.next_u32() % 4 };
                c.bus.write8(ACTUAL + i, actual as u8);
            }
            // Values: the bank id (bytes 2, 3) overlapping values[0].
            for i in 0..8 {
                let v = if r.next_u32() % 3 == 0 { r.next_u32() } else { 0xf };
                w(c, VALUES + 4 * i, v);
            }
            let id = c.bus.read32(VALUES);
            for j in 0..8u32 {
                let bank = BANKS + 0x100 * j;
                w(c, BANK_SLOTS + 8 * j, if r.next_u32() % 3 == 0 { 0 } else { bank });
                let hit = r.next_u32() % 4 == 0;
                c.bus.write8(bank + 9, if hit { (id >> 16) as u8 } else { r.next_u32() as u8 });
                c.bus.write8(bank + 8, if hit { (id >> 24) as u8 } else { r.next_u32() as u8 });
                c.bus.write16(bank + 0x10, (r.next_u32() % 16) as u16);
                for k in 0..0x40 {
                    c.bus.write8(bank + 0x18 + k, (r.next_u32() % 4) as u8);
                }
            }
            c.g[A0][0] = sx(CTX);
            c.g[A1][0] = sx(COND);
            c.g[A2][0] = sx(VALUES);
            c.g[A3][0] = sx(ACTUAL);
        });
    }

    // -- 0x00328660 SpatialHash_InsertAtKey ------------------------------------

    /// `SpatialHash_InsertAtKey` against the generated body: the target cell
    /// 0-3 levels below the start cell (start cell copy aligned or not),
    /// child nodes present or created (the real `EA_MemAllocNamed` on a
    /// test heap, its mutex syscalls fakes that sometimes unwind; the real
    /// `memset`), lists empty or not.
    #[test]
    fn spatial_hash_insert_at_key_matches() {
        use super::spatial_insert::*;
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA};
        use crate::game::bx::bxstring::tests::alloc_ready;
        const TARGET: u32 = DATA + 0x20_0000;
        const ROOT: u32 = DATA + 0x20_1000;
        const PRE: u32 = DATA + 0x20_2000;
        const OLD: u32 = DATA + 0x20_3000;
        const ITEM: u32 = DATA + 0x20_4000;
        check(ee::s10::f_00328660__generated, spatial_hash_insert_at_key, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x0032_8660, 3);
            alloc_ready(c);
            c.pc = 0x0032_8660;
            let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
            let level = r.next_u32() % 3;
            let depth = r.next_u32() % 4;
            let at = TARGET + if r.next_u32() % 2 == 0 { 0x10 } else { 0x13 };
            w(c, TARGET, level);
            c.bus.write32(at - (at & 3), 0);
            for k in 0..3u32 {
                let v = r.next_u32() % 64;
                w(c, TARGET + 4 + 4 * k, v);
                for (b, byte) in (v >> depth).to_le_bytes().into_iter().enumerate() {
                    c.bus.write8(at + 4 + 4 * k + b as u32, byte);
                }
            }
            for (b, byte) in (level + depth).to_le_bytes().into_iter().enumerate() {
                c.bus.write8(at + b as u32, byte);
            }
            // Root and pre-made nodes: some children present, lists empty or not.
            for n in 0..9u32 {
                let node = if n == 0 { ROOT } else { PRE + 0x40 * (n - 1) };
                for k in 0..8u32 {
                    let child = if n == 0 && r.next_u32() % 2 == 0 { PRE + 0x40 * k } else { 0 };
                    w(c, node + 4 * k, child);
                }
                for l in 0..3u32 {
                    let head = if r.next_u32() % 2 == 0 { 0 } else { OLD + 0x10 * (n * 3 + l) };
                    w(c, node + NODE_LISTS + 4 * l, head);
                }
            }
            c.g[A0][0] = sx(ROOT);
            c.g[A1][0] = (r.next_u32() % 3) as u64;
            c.g[A2][0] = sx(ITEM);
            c.g[A3][0] = sx(TARGET);
            c.g[T0][0] = sx(at);
        });
    }

    // -- 0x00166530 ChaseCameraAlgorithm_CheckDistanceReset_Wrapper ------------

    /// `ChaseCameraAlgorithm_CheckDistanceReset_Wrapper` against the
    /// generated body, with the real `CheckDistanceReset`: camera-to-target
    /// distances below 20, between 20 and the limit (gp-0x6778), and above
    /// (the reset path, whose rider virtual is a fake that sometimes
    /// unwinds).
    #[test]
    fn chase_camera_check_distance_reset_wrapper_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        use crate::difftest::{put_vec, DATA, GAME_GP};
        const CAM: u32 = DATA + 0x20_0000;
        const RIDER: u32 = DATA + 0x20_1000;
        const BODY: u32 = DATA + 0x20_2000;
        covered(ee::s02::f_00166530__generated, chase_camera_check_distance_reset_wrapper, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x0016_6530, 1);
            c.pc = 0x0016_6530;
            c.bus.write32(GAME_GP - 0x6778, 100.0f32.to_bits());
            let span = [5.0f32, 40.0, 400.0][(r.next_u32() % 3) as usize];
            let p = [r.f32(span), r.f32(span), r.f32(span), r.f32(1.0)];
            put_vec(c, CAM + 0x20, p);
            put_vec(c, CAM + 0x40, [r.f32(span), r.f32(span), r.f32(span), r.f32(1.0)]);
            c.bus.write32(CAM + 0x30, RIDER);
            let vt = vtable(c, &mut r, &[0x8]);
            c.bus.write32(RIDER, vt);
            c.bus.write32(RIDER + 4, BODY);
            put_vec(c, BODY + 0x1b0, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
            put_vec(c, 0x0050_0000 - 0xea0, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
            c.g[A0][0] = sx(CAM);
        });
    }
}
