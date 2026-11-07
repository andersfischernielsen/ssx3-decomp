//! Staging wave 211 (oc298): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

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
        0x0034_b168,
        c_flag_manager_group_remove_flag,
        "cFlagManager_GroupRemoveFlag",
    ),
    (0x0034_dba8, anim_object_base_dtor, "AnimObject_BaseDtor"),
    (
        0x002f_ee98,
        world_obj_factory_add_dynamic_particle_data,
        "WorldObjFactory_AddDynamicParticleData",
    ),
    (
        0x002a_5cd8,
        crowd_audio_instance_set_source_param,
        "CrowdAudio_InstanceSetSourceParam",
    ),
    (0x003b_4740, refpack_unpack, "RefPack_Unpack"),
    (0x003c_dde8, snd_utk_set_input, "SND_UtkSetInput"),
    (
        0x0023_4990,
        game_state_pre_race_load_save_state,
        "GameStatePreRaceLoad_SaveState",
    ),
    (0x0040_d610, libm_log, "log"),
    (
        0x0035_3188,
        modifier_block_free_boost,
        "ModifierBlock_FreeBoost",
    ),
];

// ---------------------------------------------------------------------------
// cFlagManager_GroupRemoveFlag (0x34b168).

/// Flag group: `+0x58` the track it belongs to, `+0x5c` how many riders it
/// holds, `+0x94` the rider pointers.
const FLAG_GROUP_TRACK: u32 = 0x58;
const FLAG_GROUP_COUNT: u32 = 0x5c;
const FLAG_GROUP_RIDERS: u32 = 0x94;
/// Rider: `+0x78` its id, `+0x80` a pointer to its current track.
const FLAG_RIDER_ID: u32 = 0x78;
const FLAG_RIDER_TRACK_PTR: u32 = 0x80;

const FLAG_GROUP_FREE_MESH: Callee = Callee {
    addr: 0x0034_b7b8,
    f: ee::s11::cFlagManager_GroupFreeMesh,
};

/// `int cFlagManager_GroupRemoveFlag(Group* group, Rider* rider)` (0x34b168):
/// remove `rider` from the group's rider list (matched by id; the last entry
/// moves into its slot) when the group is on the rider's track; free the
/// group's mesh when it becomes empty. Returns 1 when removed, else 0.
pub fn c_flag_manager_group_remove_flag(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let group = Addr(r32(c, A3));
        let rider = Addr(r32(c, A1));
        c.g[A2][0] = group.off(FLAG_GROUP_COUNT).lw(c);
        c.g[V0][0] = 0;
        'out: {
            if c.g[A2][0] == 0 {
                break 'out;
            }
            c.cycles += 5;
            c.g[V0][0] = rider.off(FLAG_RIDER_TRACK_PTR).lw(c);
            c.g[A0][0] = group.off(FLAG_GROUP_TRACK).lw(c);
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            c.g[V0][0] = 0;
            if c.g[V1][0] != c.g[A0][0] {
                break 'out;
            }
            c.cycles += 2;
            c.g[T0][0] = 0;
            if (c.g[A2][0] as i64) <= 0 {
                break 'out;
            }
            c.cycles += 3;
            c.g[A2][0] = sx(group.0.wrapping_add(FLAG_GROUP_RIDERS));
            c.g[T1][0] = c.g[A2][0];
            loop {
                // (Reloaded from the registers: an event may have run.)
                let group = Addr(r32(c, A3));
                let rider = Addr(r32(c, A1));
                let riders = r32(c, T1);
                let slot = Addr(r32(c, A2));
                c.cycles += 5;
                c.g[V0][0] = slot.lw(c);
                c.g[A0][0] = rider.off(FLAG_RIDER_ID).lw(c);
                c.g[V1][0] = Addr(r32(c, V0)).off(FLAG_RIDER_ID).lw(c);
                c.g[V0][0] = group.off(FLAG_GROUP_COUNT).lw(c);
                if c.g[V1][0] == c.g[A0][0] {
                    // Found: move the last entry here, clear the last slot.
                    c.cycles += 14;
                    let last = Addr(riders.wrapping_add(r32(c, V0).wrapping_sub(1) << 2));
                    c.g[V0][0] = sx(last.0);
                    c.g[V1][0] = last.lw(c);
                    slot.sw(c, c.g[V1][0]);
                    let count = group.off(FLAG_GROUP_COUNT).lw(c) as u32;
                    let last = Addr(riders.wrapping_add(count.wrapping_sub(1) << 2));
                    c.g[V0][0] = sx(last.0);
                    last.sw(c, 0);
                    let count = (group.off(FLAG_GROUP_COUNT).lw(c) as u32).wrapping_sub(1);
                    c.g[V1][0] = sx(count);
                    group.off(FLAG_GROUP_COUNT).sw(c, sx(count));
                    if count == 0 {
                        c.cycles += 2;
                        c.g[A0][0] = c.g[A3][0];
                        FLAG_GROUP_FREE_MESH.call(c, 0x0034_b1fc)?;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    break 'out;
                }
                c.cycles += 4;
                c.g[T0][0] = sx(r32(c, T0).wrapping_add(1));
                let more = (c.g[T0][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = more as u64;
                c.g[A2][0] = sx(slot.0.wrapping_add(4));
                if !more {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    break 'out;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_b1a8);
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// AnimObject_BaseDtor (0x34dba8).

/// The base vtable this destructor installs (`+0x20`).
const ANIM_OBJ_BASE_VTABLE: u32 = 0x0049_0cc8;
const ANIM_OBJ_VTABLE: u32 = 0x20;
/// `+0x14` the embedded ModObj; `+0x44` a cache block, `+0x48` the item
/// array (`new[]`: count at `-0x10`, 0xd0-byte items with their vtable at
/// `+0x84`), `+0x4c` a buffer.
const ANIM_OBJ_MOD_OBJ: u32 = 0x14;
const ANIM_OBJ_CACHE: u32 = 0x44;
const ANIM_OBJ_ITEMS: u32 = 0x48;
const ANIM_OBJ_BUF: u32 = 0x4c;
const ANIM_ITEM_SIZE: u32 = 0xd0;
const ANIM_ITEM_VTABLE: u32 = 0x84;
/// `EA_MemFree(p)` and `operator delete(p)` (through the table).
const ANIM_EA_MEM_FREE: u32 = 0x0031_7e98;
const ANIM_OPERATOR_DELETE: u32 = 0x0031_7e50;
const ANIM_MOD_OBJ_DTOR: Callee = Callee {
    addr: 0x0035_53c0,
    f: ee::s11::ModObj_dtor,
};

/// `void AnimObject_BaseDtor(AnimObject* self, int flags)` (0x34dba8):
/// reset the vtable, free the cache, destroy the item array back to front
/// (virtual destructor slot 1 of each, flags 0) and free it, free the
/// buffer, destroy the embedded ModObj, and `delete self` when `flags & 1`.
pub fn anim_object_base_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = sx(ANIM_OBJ_BASE_VTABLE);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        let me = Addr(r32(c, S1));
        me.off(ANIM_OBJ_VTABLE).sw(c, c.g[V0][0]);
        c.g[A0][0] = me.off(ANIM_OBJ_CACHE).lw(c);
        c.g[S0][0] = c.g[A1][0];
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            call_ptr(c, ANIM_EA_MEM_FREE, 0x0034_dbe4)?;
        }

        c.cycles += 4;
        let me = Addr(r32(c, S1));
        c.g[A1][0] = me.off(ANIM_OBJ_ITEMS).lw(c);
        c.g[S3][0] = c.g[S0][0] & 1;
        c.g[S2][0] = sx(me.0.wrapping_add(ANIM_OBJ_MOD_OBJ));
        if c.g[A1][0] != 0 {
            c.cycles += 6;
            let items = r32(c, A1);
            c.g[V1][0] = Addr(items.wrapping_sub(0x10)).lw(c);
            c.g[V0][0] = ANIM_ITEM_SIZE as u64;
            c.mult(0, A0, V1, V0);
            c.g[S0][0] = sx(r32(c, A0).wrapping_add(items));
            if c.g[A1][0] == c.g[S0][0] {
                c.g[A0][0] = me.off(ANIM_OBJ_ITEMS).lw(c);
            } else {
                c.cycles += 1;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(ANIM_ITEM_SIZE));
                loop {
                    // Item s0's virtual destructor (slot 1), flags 0.
                    c.cycles += 9;
                    let item = r32(c, S0);
                    c.g[A1][0] = 0;
                    c.g[V0][0] = Addr(item.wrapping_add(ANIM_ITEM_VTABLE)).lw(c);
                    let vt = r32(c, V0);
                    c.g[A0][0] = Addr(vt.wrapping_add(8)).lh(c);
                    c.g[V1][0] = Addr(vt.wrapping_add(0xc)).lw(c);
                    c.g[A0][0] = sx(item.wrapping_add(r32(c, A0)));
                    let target = r32(c, V1);
                    call_ptr(c, target, 0x0034_dc28)?;
                    let items = Addr(r32(c, S1)).off(ANIM_OBJ_ITEMS).lw(c);
                    c.g[V0][0] = items;
                    if items != c.g[S0][0] {
                        c.g[S0][0] = sx(r32(c, S0).wrapping_sub(ANIM_ITEM_SIZE));
                        if event_due(c) {
                            deliver_event_here(c, 0x0034_dc10);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    c.g[A0][0] = Addr(r32(c, S1)).off(ANIM_OBJ_ITEMS).lw(c);
                    break;
                }
            }
            // Free the block (the array starts 0x10 bytes into it).
            c.cycles += 2;
            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x10));
            call_ptr(c, ANIM_EA_MEM_FREE, 0x0034_dc40)?;
        }

        c.cycles += 3;
        c.g[A0][0] = Addr(r32(c, S1)).off(ANIM_OBJ_BUF).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, ANIM_EA_MEM_FREE, 0x0034_dc54)?;
        }
        c.g[A0][0] = c.g[S2][0];

        c.cycles += 4;
        c.g[A1][0] = 0;
        ANIM_MOD_OBJ_DTOR.call(c, 0x0034_dc60)?;
        let delete = c.g[S3][0] != 0;
        frame.restore(c, S0, 0x40);
        if delete {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, ANIM_OPERATOR_DELETE, 0x0034_dc70)?;
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WorldObjFactory_AddDynamicParticleData (0x2fee98).

/// The default particle record (0xf0 bytes, a function-local static) and
/// its init guard (`gp+0x253c`).
const DPD_TEMPLATE: u32 = 0x004f_b938;
const DPD_GUARD: u32 = 0x253c;
/// Record fields: `+0x14`/`+0x1c` a range whose top is clamped to its
/// bottom, `+0xd8` the target object handle (-1: the world script
/// manager's default object), `+0xdc` the modifier slot, `+0xe0..` a vector.
const DPD_RANGE_LO: u32 = 0x14;
const DPD_RANGE_HI: u32 = 0x1c;
const DPD_HANDLE: u32 = 0xd8;
const DPD_SLOT: u32 = 0xdc;
const DPD_VEC: u32 = 0xe0;
/// Per-field value types (2: float) of the record (`0x446380`).
const DPD_FIELD_TYPES: u32 = 0x0044_6380;
/// The allocation name. (The returned 12-byte value is at `0x4c9098`,
/// addressed from `s5 = 0x4d0000`.)
const DPD_ALLOC_NAME: u32 = 0x0048_9750;
/// The object handle table (`gp+0x16c8`) and the world script manager
/// (`gp+0xce8`, default object at `+0x290`).
const DPD_HANDLES: u32 = 0x16c8;
const DPD_SCRIPT_MAN: u32 = 0xce8;
const DPD_ALLOC: Callee = Callee {
    addr: 0x0031_7d70,
    f: ee::s10::EA_MemAllocNamed,
};
const DPD_DELETE: Callee = Callee {
    addr: 0x0031_7e50,
    f: ee::s10::__builtin_delete,
};
/// `EA_MemCopy(dst, src, n)` and `ModObj_AddDynamicParticleModifier(obj,
/// record, slot, &vec)` (through the table).
const DPD_MEM_COPY: u32 = 0x003e_6574;
const DPD_ADD_MODIFIER: u32 = 0x0035_5d10;

/// The default record's initial words (offset, value); every other word up
/// to `+0xe8` is 0.0.
const DPD_DEFAULTS: &[(u32, u32)] = &[
    (0x00, 1),
    (0x08, 0xbf80_0000), // -1.0
    (0x0c, 0x3f80_0000), // 1.0
    (0x10, 0x4080_0000), // 4.0
    (0x84, 0x3f80_0000),
    (0x94, 0x3f80_0000),
    (0x98, 0x3f80_0000),
    (0x9c, 0x3f80_0000),
    (0xa0, 0x3f80_0000),
    (0xa4, 0x3f80_0000),
    (0xb4, 0x3f80_0000),
    (0xc4, 0x10),
    (0xd0, 1),
    (0xd4, 0x41a0_0000), // 20.0
    (0xd8, 0xffff_ffff),
];

/// `Result WorldObjFactory_AddDynamicParticleData(Result* out, int n,
/// Entry* entries)` (0x2fee98): build a dynamic particle record (0xf0
/// bytes, allocated) from the default record (initialised on first use)
/// and `n` entries `{field, value, _, type}` (an int stored into a float
/// field is converted); clamp its range; look up the target object (by
/// handle, or the default one) and, when the object's node accepts
/// particles (virtual `+0x80`), hand the record to
/// `ModObj_AddDynamicParticleModifier`, else free it. Returns the constant
/// at `0x4c9098` (12 bytes into `*out`).
pub fn world_obj_factory_add_dynamic_particle_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = Addr(r32(c, GP).wrapping_add(DPD_GUARD)).lw(c);
        frame.save(c, S0, 0x80);
        frame.save(c, S2, 0x60);
        c.g[S0][0] = 0x50_0000;
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S1, 0x70);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, S5, 0x30);
        frame.save_ra(c, 0x20);
        if c.g[V0][0] == 0 {
            // First use: fill the default record.
            c.cycles += 76;
            let t = Addr(DPD_TEMPLATE);
            for off in (0..=0xe8).step_by(4) {
                let v = DPD_DEFAULTS
                    .iter()
                    .find(|&&(o, _)| o == off)
                    .map_or(0, |&(_, v)| v);
                t.off(off).sw(c, v as u64);
            }
            c.g[A1][0] = t.off(0xec).lbu(c);
            Addr(r32(c, GP).wrapping_add(DPD_GUARD)).sw(c, 1);
            // (Its trailing byte copied through a temporary.)
            Addr(sp(c)).sb(c, c.g[A1][0]);
            c.g[V0][0] = sx(DPD_TEMPLATE);
            c.g[A0][0] = 1;
            c.g[V1][0] = u64::MAX;
            c.g[A2][0] = 0x10;
            c.g[AT][0] = 0x41a0_0000;
            c.f[0] = 0;
            c.f[1] = 0x41a0_0000;
            c.f[2] = 0x4080_0000;
            c.f[3] = 0x3f80_0000;
        }

        // The record: allocate, copy the defaults.
        c.cycles += 17;
        c.g[A0][0] = 0xf0;
        c.g[A1][0] = sx(DPD_ALLOC_NAME);
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[S5][0] = 0x4d_0000;
        DPD_ALLOC.call(c, 0x002f_f01c)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A2][0] = 0xf0;
        c.g[V0][0] = u64::MAX;
        c.g[A1][0] = sx(DPD_TEMPLATE);
        c.g[A0][0] = c.g[S1][0];
        Addr(r32(c, S1)).off(DPD_HANDLE).sw(c, u64::MAX);
        call_ptr(c, DPD_MEM_COPY, 0x002f_f03c)?;
        c.g[A2][0] = c.g[S4][0];
        if (c.g[S3][0] as i64) > 0 {
            c.cycles += 5;
            c.g[V0][0] = 0x44_0000;
            c.g[A1][0] = c.g[S3][0];
            c.g[T0][0] = sx(DPD_FIELD_TYPES);
            c.g[A3][0] = 2;
            loop {
                let entry = Addr(r32(c, A2));
                c.cycles += 8;
                let field = (entry.lw(c) as u32) << 2;
                c.g[V1][0] = entry.off(0xc).lw(c);
                c.g[A0][0] = sx(field.wrapping_add(r32(c, S1)));
                c.g[V0][0] = Addr(field.wrapping_add(r32(c, T0))).lw(c);
                let dst = r32(c, A0);
                let int_into_float = c.g[V1][0] != c.g[V0][0] && {
                    c.cycles += 2;
                    c.g[V0][0] == c.g[A3][0]
                };
                if int_into_float {
                    c.cycles += 4;
                    c.lwc1(0, entry.0.wrapping_add(4));
                    c.cvt_s_w(0, 0);
                    c.swc1(0, dst);
                } else {
                    c.g[V0][0] = entry.off(4).lw(c);
                    c.cycles += 1;
                    Addr(dst).sw(c, c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
                c.g[A2][0] = sx(entry.0.wrapping_add(0x10));
                if c.g[A1][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_f058);
                }
            }
        }

        // Clamp the range top to its bottom.
        c.cycles += 6;
        let rec = r32(c, S1);
        c.lwc1(0, rec.wrapping_add(DPD_RANGE_HI));
        c.lwc1(1, rec.wrapping_add(DPD_RANGE_LO));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, rec.wrapping_add(DPD_RANGE_HI));
        }

        // The target object.
        c.cycles += 4;
        c.g[A2][0] = Addr(rec).off(DPD_HANDLE).lw(c);
        c.g[V0][0] = !c.g[A2][0];
        c.g[A1][0] = Addr(r32(c, GP).wrapping_add(DPD_HANDLES)).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, GP).wrapping_add(DPD_SCRIPT_MAN)).lw(c);
            c.g[V0][0] = Addr(r32(c, V0)).off(0x290).lw(c);
        } else {
            c.cycles += 8;
            let handle = r32(c, A2);
            c.g[V0][0] = sx((handle & 0xff) << 2);
            c.g[V1][0] = Addr(r32(c, A1)).lw(c);
            c.g[A0][0] = Addr(r32(c, V1)).off(8).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            c.g[V0][0] = sx(handle >> 8);
            let mut found = false;
            if c.g[V1][0] != 0 {
                c.cycles += 7;
                c.g[A0][0] = Addr(r32(c, V1)).off(0x1c).lw(c);
                let at = (handle >> 8 << 2).wrapping_add(r32(c, A0));
                c.g[V0][0] = sx(at);
                c.g[V1][0] = Addr(at).lw(c);
                let index = r32(c, V1) >> 8;
                c.g[V0][0] = sx(index << 2);
                found = index != 0;
            }
            if !found {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }

        c.cycles += 3;
        c.g[S0][0] = Addr(r32(c, V0)).off(0xc).lw(c);
        let mut handed_over = false;
        if c.g[S0][0] != 0 {
            c.cycles += 7;
            let node = r32(c, S0);
            c.g[V1][0] = Addr(node).off(0xc).lw(c);
            let vt = r32(c, V1);
            c.g[A0][0] = Addr(vt).off(0x80).lh(c);
            c.g[V0][0] = Addr(vt).off(0x84).lw(c);
            c.g[A0][0] = sx(node.wrapping_add(r32(c, A0)));
            let accepts = r32(c, V0);
            call_ptr(c, accepts, 0x002f_f134)?;
            let ok = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S1][0];
            if ok {
                c.cycles += 15;
                let rec = r32(c, S1);
                c.lwc1(3, rec.wrapping_add(DPD_VEC));
                c.lwc1(2, rec.wrapping_add(DPD_VEC + 4));
                c.g[A0][0] = c.g[S0][0];
                c.lwc1(1, rec.wrapping_add(DPD_VEC + 8));
                let v = frame.at(c, 0x10);
                c.g[A3][0] = sx(v);
                c.g[AT][0] = 0x3f80_0000;
                c.f[0] = 0x3f80_0000;
                c.swc1(3, v);
                c.swc1(2, v + 4);
                c.swc1(1, v + 8);
                c.swc1(0, v + 0xc);
                c.g[A2][0] = Addr(rec).off(DPD_SLOT).lw(c);
                call_ptr(c, DPD_ADD_MODIFIER, 0x002f_f170)?;
                handed_over = true;
            }
        }
        if !handed_over {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            DPD_DELETE.call(c, 0x002f_f180)?;
        }

        // Return the 12-byte constant.
        c.cycles += 17;
        c.g[A1][0] = sx(r32(c, S5).wrapping_sub(0x6f68));
        let src = r32(c, A1);
        c.ldl(2, src.wrapping_add(7));
        c.ldr(2, src);
        c.g[V1][0] = Addr(src).off(8).lw(c);
        let out = r32(c, S2);
        c.sdl(2, out.wrapping_add(7));
        c.sdr(2, out);
        Addr(out).off(8).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// CrowdAudio_InstanceSetSourceParam (0x2a5cd8).

/// Crowd audio instance: 12-byte sources from `+0x20` (`+0` active, `+4`
/// the parameter).
const CROWD_SOURCE_SIZE: u32 = 0xc;
const CROWD_SOURCE_ACTIVE: u32 = 0x20;
const CROWD_SOURCE_PARAM: u32 = 0x24;

/// `void CrowdAudio_InstanceSetSourceParam(Instance* inst, int source, int
/// value)` (0x2a5cd8): set an active source's parameter (negative source
/// indices are ignored).
pub fn crowd_audio_instance_set_source_param(c: &mut Cpu) {
    c.cycles += 2;
    c.g[A3][0] = c.g[A0][0];
    if (c.g[A1][0] as i64) >= 0 {
        c.cycles += 6;
        c.g[V0][0] = CROWD_SOURCE_SIZE as u64;
        c.mult(0, A0, A1, V0);
        let source = Addr(r32(c, A3).wrapping_add(r32(c, A0)));
        c.g[V1][0] = sx(source.0);
        c.g[V0][0] = source.off(CROWD_SOURCE_ACTIVE).lw(c);
        let active = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[V1][0];
        if active {
            c.cycles += 1;
            source.off(CROWD_SOURCE_PARAM).sw(c, c.g[A2][0]);
        }
    }
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// RefPack_Unpack (0x3b4740).

const REFPACK_DECODE: Callee = Callee {
    addr: 0x003b_49d0,
    f: ee::s13::RefPack_Decode,
};

/// Where RefPack_Unpack's compare chain leaves: reload `ra` first (`E8`),
/// or with `ra` already reloaded (`EC`), both returning `a1`; or straight
/// out with `v0` already set.
enum RefPackExit {
    ReloadRa,
    ReturnA1,
    Out,
}

/// `int RefPack_Unpack(const u8* src, u8* dst)` (0x3b4740): decode a
/// RefPack stream (`xx fb` header) whose type byte (ignoring bit 0) is 0x10
/// or 0x90 with `RefPack_Decode`; any other stream yields 0. Returns the
/// decoded size.
pub fn refpack_unpack(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        c.g[V0][0] = 0xfb;
        let src = Addr(r32(c, A3));
        c.g[V1][0] = src.off(1).lbu(c);
        c.g[A1][0] = 0;
        let exit = 'chain: {
            if c.g[V1][0] != 0xfb {
                break 'chain RefPackExit::ReloadRa;
            }
            c.cycles += 5;
            c.g[V0][0] = src.lbu(c);
            c.g[V1][0] = 0x46;
            let kind = c.g[V0][0] & 0xfe;
            c.g[A2][0] = kind;
            c.g[V0][0] = (kind < 71) as u64;
            if kind == 0x46 {
                break 'chain RefPackExit::ReloadRa;
            }
            c.cycles += 2;
            c.g[V0][0] = 0x1c;
            if kind < 71 {
                c.cycles += 2;
                c.g[V0][0] = (kind < 29) as u64;
                if kind == 0x1c {
                    break 'chain RefPackExit::ReloadRa;
                }
                c.cycles += 2;
                c.g[V0][0] = 0x18;
                if kind >= 29 {
                    break 'chain RefPackExit::ReloadRa;
                }
                c.cycles += 2;
                c.g[V0][0] = (kind < 25) as u64;
                if kind == 0x18 {
                    break 'chain RefPackExit::ReloadRa;
                }
                c.cycles += 2;
                c.g[V0][0] = 0x10;
                if kind >= 25 {
                    break 'chain RefPackExit::ReloadRa;
                }
                c.cycles += 2;
                frame.restore_ra(c, 0);
                if kind != 0x10 {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[A1][0];
                    break 'chain RefPackExit::Out;
                }
            } else {
                c.cycles += 3;
                c.g[V0][0] = (kind < 159) as u64;
                if kind == 0x9e {
                    break 'chain RefPackExit::ReloadRa;
                }
                c.cycles += 2;
                c.g[V0][0] = 0x98;
                if kind >= 159 {
                    break 'chain RefPackExit::ReloadRa;
                }
                c.cycles += 2;
                c.g[V0][0] = (kind < 153) as u64;
                if kind == 0x98 {
                    break 'chain RefPackExit::ReloadRa;
                }
                c.cycles += 2;
                c.g[V0][0] = 0x90;
                if kind >= 153 {
                    break 'chain RefPackExit::ReloadRa;
                }
                c.cycles += 2;
                frame.restore_ra(c, 0);
                if kind != 0x90 {
                    break 'chain RefPackExit::ReturnA1;
                }
            }
            // RefPack_Decode(dst, src, NULL).
            c.cycles += 4;
            c.g[A1][0] = c.g[A3][0];
            c.g[A2][0] = 0;
            REFPACK_DECODE.call(c, 0x003b_47e4)?;
            c.g[A1][0] = c.g[V0][0];
            RefPackExit::ReloadRa
        };
        if let RefPackExit::ReloadRa = exit {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        }
        if !matches!(exit, RefPackExit::Out) {
            c.cycles += 1;
            c.g[V0][0] = c.g[A1][0];
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_UtkSetInput (0x3cdde8).

/// UTK decoder state: `+0xd44` bit position, `+0xd48` the input, `+0xd54`
/// the input's size (non-zero: busy), `+0xd58` the user word, `+0xd5c`
/// the header reset flag, `+0xd60` "starts with the 0xee marker", `+0xd64`
/// the stream format (1: marker byte first).
const UTK_BIT_POS: u32 = 0xd44;
const UTK_INPUT: u32 = 0xd48;
const UTK_INPUT_SIZE: u32 = 0xd54;
const UTK_USER: u32 = 0xd58;
const UTK_RESET: u32 = 0xd5c;
const UTK_MARKED: u32 = 0xd60;
const UTK_FORMAT: u32 = 0xd64;
const UTK_PARSE_HEADER: Callee = Callee {
    addr: 0x003c_d6f0,
    f: ee::s14::SND_UtkParseHeader,
};

/// `int SND_UtkSetInput(UtkState* st, u8* data, int user, int size)`
/// (0x3cdde8): attach a new input buffer to an idle decoder (-1 if `data`
/// is null or an input is still attached), note whether a format-1 stream
/// starts with the 0xee marker (skipped), and parse its header. Returns 0.
pub fn snd_utk_set_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = u64::MAX;
        'out: {
            if c.g[A1][0] == 0 {
                break 'out;
            }
            c.cycles += 3;
            let st = Addr(r32(c, A0));
            c.g[V0][0] = st.off(UTK_INPUT_SIZE).lw(c);
            let busy = c.g[V0][0] != 0;
            c.g[V0][0] = u64::MAX;
            if busy {
                break 'out;
            }
            c.cycles += 7;
            c.g[T0][0] = st.off(UTK_FORMAT).lw(c);
            c.g[V0][0] = 1;
            st.off(UTK_INPUT_SIZE).sw(c, c.g[A3][0]);
            st.off(UTK_USER).sw(c, c.g[A2][0]);
            st.off(UTK_BIT_POS).sw(c, 0);
            st.off(UTK_INPUT).sw(c, c.g[A1][0]);
            if c.g[T0][0] == 1 {
                c.cycles += 4;
                let data = r32(c, A1);
                c.g[V1][0] = Addr(data).lbu(c);
                c.g[V0][0] = 0xee;
                if c.g[V1][0] != 0xee {
                    st.off(UTK_MARKED).sw(c, 0);
                } else {
                    c.cycles += 1;
                    st.off(UTK_MARKED).sw(c, c.g[T0][0]);
                }
                c.cycles += 6;
                c.g[A3][0] = st.off(UTK_RESET).lw(c);
                c.g[A1][0] = sx(data.wrapping_add(1));
                c.g[A2][0] = c.g[A0][0];
                UTK_PARSE_HEADER.call(c, 0x003c_de44)?;
            } else {
                c.cycles += 4;
                c.g[A3][0] = st.off(UTK_RESET).lw(c);
                c.g[A2][0] = c.g[A0][0];
                UTK_PARSE_HEADER.call(c, 0x003c_de58)?;
            }
            c.g[V0][0] = 0;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// GameStatePreRaceLoad_SaveState (0x234990).

const PRE_RACE_BASE_SAVE_STATE: Callee = Callee {
    addr: 0x0023_3af0,
    f: ee::s06::GameState_SaveState,
};

/// `void GameStatePreRaceLoad_SaveState(GameStatePreRaceLoad* self, Stream*
/// s)` (0x234990): the base state's save, then the words at `+0x10`,
/// `+0x14`, `+0x1c` and `+0x8`, each written with the stream's virtual
/// `Write(s, p, 4)` (vtable slot `+8`).
pub fn game_state_pre_race_load_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        PRE_RACE_BASE_SAVE_STATE.call(c, 0x0023_49ac)?;
        for (field, ret) in [
            (0x10, 0x0023_49c8),
            (0x14, 0x0023_49e4),
            (0x1c, 0x0023_4a00),
            (0x08, 0x0023_4a1c),
        ] {
            let stream = r32(c, S0);
            c.g[V1][0] = Addr(stream).lw(c);
            c.g[A1][0] = sx(r32(c, S1).wrapping_add(field));
            c.g[A2][0] = 4;
            let vt = r32(c, V1);
            c.g[A0][0] = Addr(vt).off(8).lh(c);
            c.g[V0][0] = Addr(vt).off(0xc).lw(c);
            c.g[A0][0] = sx(stream.wrapping_add(r32(c, A0)));
            let write = r32(c, V0);
            call_ptr(c, write, ret)?;
        }
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// log (0x40d610), newlib's `w_log.c` wrapper (soft-float doubles in a0/v0).

/// `_LIB_VERSION` (-1 IEEE, 0 SVID, 1 XOPEN, 2 POSIX).
const LOG_LIB_VERSION: u32 = 0x0049_9688;
/// The exception's name ("log"), `-HUGE` (SVID) and `HUGE_VAL`.
const LOG_NAME: u32 = 0x0049_9070;
const LOG_MINUS_HUGE: u32 = 0x0049_9078;
const LOG_HUGE_VAL: u32 = 0x0049_9680;
/// `struct exception` on the stack: `+0` type, `+4` name, `+8`/`+0x10`
/// the arguments, `+0x18` the return value, `+0x20` err.
const LOG_EXC_TYPE: u32 = 0;
const LOG_EXC_NAME: u32 = 4;
const LOG_EXC_ARG1: u32 = 8;
const LOG_EXC_ARG2: u32 = 0x10;
const LOG_EXC_RETVAL: u32 = 0x18;
const LOG_EXC_ERR: u32 = 0x20;
const LOG_IEEE754_LOG: Callee = Callee {
    addr: 0x0040_db58,
    f: ee::s15::__ieee754_log,
};
const LOG_ISNAN: Callee = Callee {
    addr: 0x0040_f840,
    f: ee::s15::isnan,
};
const LOG_CMPDF2: Callee = Callee {
    addr: 0x0041_2500,
    f: ee::s15::__cmpdf2,
};
const LOG_SUBDF3: Callee = Callee {
    addr: 0x0041_1f70,
    f: ee::s15::__subdf3,
};
const LOG_GETREENT: Callee = Callee {
    addr: 0x0041_5fb8,
    f: ee::s15::__getreent,
};
const LOG_MATHERR: Callee = Callee {
    addr: 0x0040_f878,
    f: ee::s15::matherr,
};

/// The next step of `log`'s error report.
enum LogStep {
    /// Not POSIX: let `matherr` handle it.
    AskMatherr,
    /// errno = EDOM.
    Edom,
    /// Store `v1` into errno (`v0` the reentrancy struct).
    StoreErrno,
    /// `matherr` handled it.
    Handled,
}

/// `double log(double x)` (0x40d610): `__ieee754_log(x)`; outside IEEE
/// mode, for x <= 0 (not NaN) fill a `struct exception` (SING for 0, DOMAIN
/// for negatives; return -HUGE in SVID mode, else -HUGE_VAL), set errno
/// (ERANGE/EDOM) directly in POSIX mode, otherwise ask `matherr` and set
/// EDOM if it declines; a non-zero `exc.err` goes to errno too. Returns the
/// exception's value.
pub fn libm_log(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x90);
        let a = frame.at(c, 0x30);
        c.sd(a, c.g[S0][0]);
        c.g[S0][0] = c.g[A0][0];
        let a = frame.at(c, 0x60);
        c.sd(a, c.g[S3][0]);
        let a = frame.at(c, 0x50);
        c.sd(a, c.g[S2][0]);
        let a = frame.at(c, 0x40);
        c.sd(a, c.g[S1][0]);
        c.g[S3][0] = 0x4a_0000;
        frame.save_ra(c, 0x80);
        let a = frame.at(c, 0x70);
        c.sd(a, c.g[S4][0]);
        LOG_IEEE754_LOG.call(c, 0x0040_d638)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[S1][0] = Addr(LOG_LIB_VERSION).lw(c);
        c.g[V0][0] = u64::MAX;
        'done: {
            let ieee = c.g[S1][0] == u64::MAX;
            c.g[V0][0] = c.g[S2][0];
            if ieee {
                break 'done;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            LOG_ISNAN.call(c, 0x0040_d654)?;
            let nan = c.g[V0][0] != 0;
            c.g[V0][0] = c.g[S2][0];
            if nan {
                break 'done;
            }
            c.cycles += 6;
            c.g[S4][0] = 0;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S4][0];
            LOG_CMPDF2.call(c, 0x0040_d66c)?;
            let positive = (c.g[V0][0] as i64) > 0;
            c.g[V0][0] = c.g[S2][0];
            if positive {
                break 'done;
            }

            // x <= 0: the exception record.
            c.cycles += 7;
            Addr(frame.at(c, LOG_EXC_ERR)).sw(c, 0);
            c.g[V0][0] = sx(LOG_NAME);
            Addr(frame.at(c, LOG_EXC_ARG1)).sd(c, c.g[S0][0]);
            Addr(frame.at(c, LOG_EXC_NAME)).sw(c, c.g[V0][0]);
            Addr(frame.at(c, LOG_EXC_ARG2)).sd(c, c.g[S0][0]);
            if c.g[S1][0] == 0 {
                // SVID: -HUGE.
                c.cycles += 4;
                c.g[AT][0] = 0x4a_0000;
                c.g[V0][0] = Addr(LOG_MINUS_HUGE).ld(c);
            } else {
                // -HUGE_VAL.
                c.cycles += 5;
                c.g[V0][0] = 0x4a_0000;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = Addr(LOG_HUGE_VAL).ld(c);
                LOG_SUBDF3.call(c, 0x0040_d6b0)?;
            }
            Addr(frame.at(c, LOG_EXC_RETVAL)).sd(c, c.g[V0][0]);

            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            LOG_CMPDF2.call(c, 0x0040_d6c0)?;
            let zero = c.g[V0][0] == 0;
            c.g[V1][0] = 1;
            // POSIX sets errno directly; otherwise matherr may handle it.
            let posix_erange = if zero {
                // SING.
                c.cycles += 4;
                c.g[V0][0] = 2;
                c.g[V1][0] = Addr(LOG_LIB_VERSION).lw(c);
                Addr(frame.at(c, LOG_EXC_TYPE)).sw(c, c.g[V0][0]);
                c.g[V1][0] == 2
            } else {
                false
            };
            let step = if zero {
                if posix_erange {
                    c.cycles += 4;
                    LOG_GETREENT.call(c, 0x0040_d6e0)?;
                    c.g[V1][0] = 0x22; // ERANGE
                    LogStep::StoreErrno
                } else {
                    LogStep::AskMatherr
                }
            } else {
                // DOMAIN.
                c.cycles += 4;
                c.g[V0][0] = Addr(LOG_LIB_VERSION).lw(c);
                c.g[A0][0] = 2;
                Addr(frame.at(c, LOG_EXC_TYPE)).sw(c, c.g[V1][0]);
                if c.g[V0][0] == 2 {
                    LogStep::Edom
                } else {
                    LogStep::AskMatherr
                }
            };
            let step = if let LogStep::AskMatherr = step {
                c.cycles += 4;
                c.g[A0][0] = sx(sp(c));
                LOG_MATHERR.call(c, 0x0040_d700)?;
                let handled = c.g[V0][0] != 0;
                c.g[V0][0] = Addr(frame.at(c, LOG_EXC_ERR)).lw(c);
                if handled {
                    LogStep::Handled
                } else {
                    LogStep::Edom
                }
            } else {
                step
            };
            let step = if let LogStep::Edom = step {
                c.cycles += 3;
                LOG_GETREENT.call(c, 0x0040_d710)?;
                c.g[V1][0] = 0x21; // EDOM
                LogStep::StoreErrno
            } else {
                step
            };
            if let LogStep::StoreErrno = step {
                c.cycles += 2;
                Addr(r32(c, V0)).sw(c, c.g[V1][0]);
                c.g[V0][0] = Addr(frame.at(c, LOG_EXC_ERR)).lw(c);
            }
            // A non-zero exc.err becomes errno.
            c.cycles += 2;
            let exc_err = c.g[V0][0] != 0;
            c.g[V0][0] = Addr(frame.at(c, LOG_EXC_RETVAL)).ld(c);
            if exc_err {
                c.cycles += 5;
                LOG_GETREENT.call(c, 0x0040_d72c)?;
                c.g[V1][0] = Addr(frame.at(c, LOG_EXC_ERR)).lw(c);
                Addr(r32(c, V0)).sw(c, c.g[V1][0]);
                c.g[V0][0] = Addr(frame.at(c, LOG_EXC_RETVAL)).ld(c);
            }
        }
        c.cycles += 8;
        frame.restore_ra(c, 0x80);
        let a = frame.at(c, 0x70);
        c.g[S4][0] = c.ld(a);
        let a = frame.at(c, 0x60);
        c.g[S3][0] = c.ld(a);
        let a = frame.at(c, 0x50);
        c.g[S2][0] = c.ld(a);
        let a = frame.at(c, 0x40);
        c.g[S1][0] = c.ld(a);
        let a = frame.at(c, 0x30);
        c.g[S0][0] = c.ld(a);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ModifierBlock_FreeBoost (0x353188).

/// Modifier block: `+0xc` the boost object (vtable at `+4`, slot 1 its
/// deleting destructor).
const MOD_BLOCK_BOOST: u32 = 0xc;

/// `void ModifierBlock_FreeBoost(ModifierBlock* self)` (0x353188): delete
/// the boost object, if any (virtual destructor, flags 3), and clear it.
pub fn modifier_block_free_boost(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A2][0] = Addr(r32(c, S0)).off(MOD_BLOCK_BOOST).lw(c);
        c.g[A1][0] = 3;
        if c.g[A2][0] != 0 {
            c.cycles += 6;
            let boost = r32(c, A2);
            c.g[V0][0] = Addr(boost).off(4).lw(c);
            let vt = r32(c, V0);
            c.g[A0][0] = Addr(vt).off(8).lh(c);
            c.g[V1][0] = Addr(vt).off(0xc).lw(c);
            c.g[A0][0] = sx(boost.wrapping_add(r32(c, A0)));
            let dtor = r32(c, V1);
            call_ptr(c, dtor, 0x0035_31b8)?;
            Addr(r32(c, S0)).off(MOD_BLOCK_BOOST).sw(c, 0);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, fill, Rng, DATA};

    /// GroupRemoveFlag: a group of 0..6 riders (ids from a small set so
    /// matches and misses both happen), a rider on the group's track or not;
    /// the matched entry moved, the count reaching 0 calls the real
    /// `GroupFreeMesh` on its short path (no mesh, no buffer).
    #[test]
    fn c_flag_manager_group_remove_flag_matches() {
        const GROUP: u32 = DATA;
        const RIDER: u32 = DATA + 0x1000;
        const TRACK_CELL: u32 = DATA + 0x2000;
        const MEMBERS: u32 = DATA + 0x3000;
        check(
            ee::s11::f_0034b168__generated,
            c_flag_manager_group_remove_flag,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x34b168, 0);
                fill(c, &mut r, GROUP, 0x40);
                let count: i32 = match seed % 8 {
                    0 => 0,
                    1 => -1,
                    2 => 1,
                    _ => (r.next_u32() % 6 + 1) as i32,
                };
                let track = r.next_u32();
                c.bus.write32(GROUP + 0x58, track);
                c.bus.write32(GROUP + 0x5c, count as u32);
                // Mesh and buffer null: GroupFreeMesh takes its short path.
                c.bus.write32(GROUP + 0x60, 0);
                c.bus.write32(GROUP + 0x74, 0);
                c.bus.write32(RIDER + 0x80, TRACK_CELL);
                let same_track = seed % 5 != 3;
                c.bus
                    .write32(TRACK_CELL, if same_track { track } else { track ^ 1 });
                let id = r.next_u32() % 4;
                c.bus.write32(RIDER + 0x78, id);
                for i in 0..6u32 {
                    let m = MEMBERS + 0x100 * i;
                    c.bus.write32(GROUP + 0x94 + 4 * i, m);
                    c.bus.write32(m + 0x78, r.next_u32() % 4);
                }
                c.g[A0][0] = sx(GROUP);
                c.g[A1][0] = sx(RIDER);
                c.pc = 0x0034_b168;
            },
        );
    }

    /// AnimObject_BaseDtor: the object, its cache, buffer and item array
    /// (0..4 items of 0xd0 bytes behind the 0x10-byte `new[]` header) are
    /// allocated from a dlmalloc arena over test memory (pools off) and
    /// freed by the real EA_MemFree; the items' destructors are fakes (some
    /// unwind); the embedded ModObj has no modifier block and no node, so the
    /// real ModObj_dtor takes its short path; flags with and without the
    /// delete bit (the real operator delete frees the object).
    #[test]
    fn anim_object_base_dtor_matches() {
        use crate::difftest::fakes::{FAKE, LOG_COUNT, LOG_UNWIND_AT};
        use crate::difftest::GAME_GP;
        use crate::game::bx::cubicspline::tests::oc238_run;
        check(
            ee::s11::f_0034dba8__generated,
            anim_object_base_dtor,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x34dba8, 0);
                // The game heap over test memory, pools off.
                c.bus.write32(GAME_GP + 0xdd8, 0);
                oc238_run(
                    c,
                    0x0031_ff60,
                    ee::s10::f_0031ff60,
                    &[0x004f_f250, (DATA + 0x40000) as u64, 0x40000],
                );
                let alloc = |c: &mut Cpu, size: u64| -> u32 {
                    oc238_run(c, 0x0031_9e48, ee::s10::f_00319e48, &[size, 0]) as u32
                };
                let me = alloc(c, 0x60);
                fill(c, &mut r, me, 0x18);
                // ModObj at +0x14: no node (+4), no instance (+0x18), no block (+0x1c).
                let mod_obj = me + 0x14;
                c.bus.write32(mod_obj + 4, 0);
                c.bus.write16(mod_obj + 0x10, (r.next_u32() % 8) as u16);
                c.bus.write32(mod_obj + 0x18, 0);
                c.bus.write32(mod_obj + 0x1c, 0);
                let cache = if seed % 3 == 0 { 0 } else { alloc(c, 0x20) };
                let buf = if seed % 4 == 1 { 0 } else { alloc(c, 0x30) };
                let items = if seed % 5 == 2 {
                    0
                } else {
                    let n = r.next_u32() % 5;
                    let block = alloc(c, (0x10 + 0xd0 * n) as u64);
                    c.bus.write32(block, n);
                    let items = block + 0x10;
                    let vt = DATA + 0x200;
                    c.bus.write16(vt + 8, ((r.next_u32() % 0x10) & !3) as u16);
                    c.bus.write32(vt + 0xc, FAKE);
                    for i in 0..n {
                        c.bus.write32(items + 0xd0 * i + 0x84, vt);
                    }
                    items
                };
                c.bus.write32(me + 0x44, cache);
                c.bus.write32(me + 0x48, items);
                c.bus.write32(me + 0x4c, buf);
                // Unwind at one of the next calls (the setup made some).
                if seed % 3 == 1 {
                    let n = c.bus.read32(LOG_COUNT);
                    c.bus.write32(LOG_UNWIND_AT, n + r.next_u32() % 8);
                }
                c.g[A0][0] = sx(me);
                c.g[A1][0] = (r.next_u32() & 0xff) as u64;
                c.pc = 0x0034_dba8;
            },
        );
    }

    /// WorldObjFactory_AddDynamicParticleData: the default record fresh
    /// (guard 0) or already built (random words, the range in either
    /// order); 0..4 entries (and n <= 0) with field types random against
    /// the type table (int-into-float conversions), optionally setting the
    /// handle (-1, a resolving one, one whose table slot or index is empty)
    /// and the modifier slot; the target node's `+0x80` virtual is a fake
    /// (accepts: the real ModObj_AddDynamicParticleModifier, which frees the
    /// record on its short paths) or a leaf returning 0 (the record is
    /// deleted). The record comes from a dlmalloc arena over test memory.
    #[test]
    fn world_obj_factory_add_dynamic_particle_data_matches() {
        use crate::difftest::fakes::{FAKE, LOG_COUNT, LOG_UNWIND_AT};
        use crate::difftest::GAME_GP;
        use crate::game::bx::cubicspline::tests::oc238_run;
        const OUT: u32 = DATA + 0x500;
        const ENTRIES: u32 = DATA + 0x600;
        const SCRIPT_MAN: u32 = DATA + 0x2000;
        const HANDLES: u32 = DATA + 0x2100;
        const HANDLE_TABLE: u32 = DATA + 0x2200;
        const SETS: u32 = DATA + 0x2300;
        const SET: u32 = DATA + 0x2400;
        const SET_INDEX: u32 = DATA + 0x2500;
        const OBJ: u32 = DATA + 0x3000;
        const NODE: u32 = DATA + 0x3100;
        const NODE_VT: u32 = DATA + 0x3200;
        const MODS: u32 = DATA + 0x3300;
        const MOD_LIST: u32 = DATA + 0x3400;
        check(
            ee::s10::f_002fee98__generated,
            world_obj_factory_add_dynamic_particle_data,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x2fee98, 0);
                // The game heap over test memory, pools off.
                c.bus.write32(GAME_GP + 0xdd8, 0);
                oc238_run(
                    c,
                    0x0031_ff60,
                    ee::s10::f_0031ff60,
                    &[0x004f_f250, (DATA + 0x40000) as u64, 0x40000],
                );
                // The default record: fresh, or built (random words).
                if seed % 2 == 0 {
                    c.bus.write32(GAME_GP + DPD_GUARD, 0);
                } else {
                    c.bus.write32(GAME_GP + DPD_GUARD, 1);
                    fill(c, &mut r, DPD_TEMPLATE, 0x3c);
                    c.bus.write32(DPD_TEMPLATE + 0x14, r.f32(10.0));
                    c.bus.write32(DPD_TEMPLATE + 0x1c, r.f32(10.0));
                    c.bus.write32(DPD_TEMPLATE + 0xd8, u32::MAX);
                    c.bus.write32(DPD_TEMPLATE + 0xdc, u32::MAX);
                }
                fill(c, &mut r, DPD_TEMPLATE + 0xec, 1);
                // Field types: 0..2 (2: float); handle and slot plain ints.
                for i in 0..0x3c {
                    c.bus.write32(DPD_FIELD_TYPES + 4 * i, r.next_u32() % 3);
                }
                c.bus.write32(DPD_FIELD_TYPES + 4 * 54, 0);
                c.bus.write32(DPD_FIELD_TYPES + 4 * 55, 0);
                // Entries.
                let mut entries = Vec::new();
                for _ in 0..r.next_u32() % 4 {
                    let field = r.next_u32() % 54;
                    entries.push((field, r.next_u32() % 0x400, r.next_u32() % 3));
                }
                let handle = match seed % 5 {
                    0 => None,
                    1 => Some(u32::MAX),
                    2 => Some(0x0300 | 5), // empty set slot
                    3 => Some(0x0400 | 6), // empty index
                    _ => Some(0x0200 | 7), // resolves to OBJ
                };
                if let Some(h) = handle {
                    entries.push((54, h, 0));
                }
                if seed % 3 != 0 {
                    entries.push((55, if seed % 6 == 1 { u32::MAX } else { 3 }, 0));
                }
                for (i, &(field, value, ty)) in entries.iter().enumerate() {
                    let e = ENTRIES + 0x10 * i as u32;
                    c.bus.write32(e, field);
                    c.bus.write32(e + 4, value);
                    c.bus.write32(e + 0xc, ty);
                }
                let n = if seed % 13 == 0 {
                    u32::MAX
                } else {
                    entries.len() as u32
                };
                // Handle lookup tables.
                c.bus.write32(GAME_GP + DPD_SCRIPT_MAN, SCRIPT_MAN);
                c.bus.write32(SCRIPT_MAN + 0x290, OBJ);
                c.bus.write32(GAME_GP + DPD_HANDLES, HANDLES);
                c.bus.write32(HANDLES, HANDLE_TABLE);
                c.bus.write32(HANDLE_TABLE + 8, SETS);
                for j in 0..8 {
                    c.bus.write32(SETS + 4 * j, if j == 5 { 0 } else { SET });
                }
                for k in 0..8 {
                    let v = if k == 4 {
                        r.next_u32() & 0xff
                    } else {
                        (OBJ >> 2) << 8
                    };
                    c.bus.write32(SET + 0x1c, SET_INDEX);
                    c.bus.write32(SET_INDEX + 4 * k, v);
                }
                c.bus.write32(0xc, 0); // the null object's node
                c.bus
                    .write32(OBJ + 0xc, if seed % 7 == 3 { 0 } else { NODE });
                c.bus.write32(NODE + 0xc, NODE_VT);
                c.bus
                    .write16(NODE_VT + 0x80, ((r.next_u32() % 0x10) & !3) as u16);
                c.bus.write32(
                    NODE_VT + 0x84,
                    if seed % 4 == 1 { 0x0014_06c0 } else { FAKE },
                );
                // No modifier slots: AddDynamicParticleModifier frees the record.
                c.bus.write32(NODE + 0x18, MODS);
                c.bus.write32(MODS + 0x80, MOD_LIST);
                c.bus.write32(MOD_LIST + 4, 0);
                // The returned constant.
                fill(c, &mut r, 0x004c_9098, 3);
                if seed % 3 == 2 {
                    let k = c.bus.read32(LOG_COUNT);
                    c.bus.write32(LOG_UNWIND_AT, k + r.next_u32() % 6);
                }
                c.g[A0][0] = sx(OUT);
                c.g[A1][0] = sx(n);
                c.g[A2][0] = sx(ENTRIES);
                c.pc = 0x002f_ee98;
            },
        );
    }

    /// CrowdAudio_InstanceSetSourceParam: negative, zero and positive
    /// source indices; active and inactive sources.
    #[test]
    fn crowd_audio_instance_set_source_param_matches() {
        check(
            ee::s08::f_002a5cd8__generated,
            crowd_audio_instance_set_source_param,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x2a5cd8, 0);
                let inst = DATA + 0x100;
                fill(c, &mut r, inst, 0x40);
                let source: i32 = match seed % 4 {
                    0 => -((r.next_u32() % 5) as i32) - 1,
                    1 => 0,
                    _ => (r.next_u32() % 8) as i32,
                };
                if source >= 0 && seed % 3 == 0 {
                    c.bus.write32(inst + 0x20 + 0xc * source as u32, 0);
                }
                c.g[A0][0] = sx(inst);
                c.g[A1][0] = source as i64 as u64;
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.pc = 0x002a_5cd8;
            },
        );
    }

    /// RefPack_Unpack: a real RefPack stream (`game::misc` tests' generator)
    /// whose type byte is every value of interest (0x10/0x11/0x90/0x91
    /// decode; 0x46, 0x1c, 0x18, 0x9e, 0x98 and the values around and
    /// between them do not), or a header without the `fb` marker.
    #[test]
    fn refpack_unpack_matches() {
        use crate::game::misc::tests::refpack_stream;
        const KINDS: &[u8] = &[
            0x10, 0x11, 0x90, 0x91, 0x46, 0x47, 0x1c, 0x1d, 0x18, 0x19, 0x9e, 0x98, 0x99, 0x00,
            0x02, 0x12, 0x1a, 0x1e, 0x30, 0x48, 0x60, 0x80, 0x92, 0x9a, 0xa0, 0xfe, 0x10, 0x90,
        ];
        check(ee::s13::f_003b4740__generated, refpack_unpack, 300, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x3b4740, 0);
            let src = DATA + 0x1000;
            // The generator's header variant follows the type byte's bits
            // (0x80: 4-byte sizes, 0x01: packed size present).
            let kind = KINDS[seed as usize % KINDS.len()];
            let variant = (kind >> 7) as u32 | ((kind & 1) as u32) << 1;
            let mut stream = refpack_stream(&mut r, variant);
            stream[0] = kind;
            if seed % 11 == 5 {
                stream[1] = r.next_u32() as u8 & 0xfa;
            }
            for (i, b) in stream.into_iter().enumerate() {
                c.bus.write8(src + i as u32, b);
            }
            c.g[A0][0] = sx(src);
            c.g[A1][0] = sx(DATA + 0x4_0000);
            c.pc = 0x003b_4740;
        });
    }

    /// SND_UtkSetInput: null data, a busy decoder, format 1 with and
    /// without the 0xee marker, other formats; the real SND_UtkParseHeader
    /// (with or without its header read; its mask table from the ELF image)
    /// on random stream bytes.
    #[test]
    fn snd_utk_set_input_matches() {
        use crate::difftest::elf_image;
        check(ee::s14::f_003cdde8__generated, snd_utk_set_input, 300, &|c, seed| {
            elf_image(c);
            let mut r = fakes::machine(c, seed, 0x3cdde8, 0);
            let st = DATA + 0x1000;
            let data = DATA + 0x100 + seed % 4;
            fill(c, &mut r, DATA + 0x100, 8);
            fill(c, &mut r, st, 0xd70 / 4);
            c.bus.write32(
                st + UTK_INPUT_SIZE,
                if seed % 5 == 1 { r.next_u32() | 1 } else { 0 },
            );
            c.bus.write32(
                st + UTK_FORMAT,
                if seed % 3 == 0 { r.next_u32() % 3 } else { 1 },
            );
            if seed % 2 == 0 {
                c.bus.write8(data, 0xee);
            }
            let reset = match seed % 4 {
                0 => 0,
                1 => 1,
                _ => r.next_u32() | 1,
            };
            c.bus.write32(st + UTK_RESET, reset);
            c.g[A0][0] = sx(st);
            c.g[A1][0] = if seed % 7 == 3 { 0 } else { sx(data) };
            c.g[A2][0] = r.next_u32() as u64;
            c.g[A3][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.pc = 0x003c_dde8;
        });
    }

    /// GameStatePreRaceLoad_SaveState: a stream whose `Write` slot is a
    /// fake (logs every argument; one of the five calls may unwind); the
    /// real base GameState_SaveState makes the first call.
    #[test]
    fn game_state_pre_race_load_save_state_matches() {
        fakes::covered(
            ee::s06::f_00234990__generated,
            game_state_pre_race_load_save_state,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x234990, 5);
                let stream = DATA + 0x200;
                let vt = fakes::vtable(c, &mut r, &[8]);
                c.bus.write32(stream, vt);
                c.g[A0][0] = sx(DATA + 0x400 + (r.next_u32() & 0xfc));
                c.g[A1][0] = sx(stream);
                c.pc = 0x0023_4990;
            },
        );
    }

    /// log: every `_LIB_VERSION` mode (IEEE, SVID, XOPEN, POSIX) against
    /// positive, zero (both signs), negative, NaN and infinite arguments
    /// (the real soft-float callees, matherr and errno from the ELF image).
    #[test]
    fn libm_log_matches() {
        use crate::difftest::elf_image;
        check(ee::s15::f_0040d610__generated, libm_log, 300, &|c, seed| {
            elf_image(c);
            let mut r = fakes::machine(c, seed, 0x40d610, 0);
            let version = [u32::MAX, 0, 1, 2][(seed % 4) as usize];
            c.bus.write32(LOG_LIB_VERSION, version);
            let x: f64 = match (seed / 4) % 8 {
                0 => 0.0,
                1 => -0.0,
                2 => -(1.0 + (r.next_u32() % 1000) as f64),
                3 => f64::NAN,
                4 => f64::INFINITY,
                5 => f64::NEG_INFINITY,
                6 => (r.next_u32() % 100_000) as f64 / 7.0 + 1e-9,
                _ => f64::from_bits(r.next_u32() as u64), // denormal
            };
            c.g[A0][0] = x.to_bits();
            c.pc = 0x0040_d610;
        });
    }

    /// ModifierBlock_FreeBoost: no boost object, or one whose destructor
    /// slot is a fake (logs `this` and flags; some calls unwind).
    #[test]
    fn modifier_block_free_boost_matches() {
        fakes::covered(
            ee::s11::f_00353188__generated,
            modifier_block_free_boost,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x353188, 1);
                let block = DATA + 0x100;
                let boost = DATA + 0x200;
                fill(c, &mut r, block, 8);
                let vt = fakes::vtable(c, &mut r, &[8]);
                c.bus.write32(boost + 4, vt);
                c.bus
                    .write32(block + 0xc, if seed % 4 == 0 { 0 } else { boost });
                c.g[A0][0] = sx(block);
                c.pc = 0x0035_3188;
            },
        );
    }
}
