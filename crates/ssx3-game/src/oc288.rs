//! Staging wave 210 (oc288): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0026_b508,
        path_data_get_track_path_index,
        "PathData_GetTrackPathIndex",
    ),
    (
        0x0027_0730,
        replay_find_rider_index,
        "Replay_FindRiderIndex",
    ),
    (0x0012_b7f0, ai_save_state, "AI_SaveState"),
    (
        0x0034_60a0,
        dynamic_particle_modifier_save,
        "DynamicParticleModifier_Save",
    ),
    (
        0x0013_68e8,
        rider_uber_grind_sub_save_state,
        "RiderUberGrindSub_SaveState",
    ),
    (
        0x0012_c9c0,
        rider_finish_line_sub_save_state,
        "RiderFinishLineSub_SaveState",
    ),
    (
        0x0035_3300,
        modifier_block_free_rail_modifiers,
        "ModifierBlock_FreeRailModifiers",
    ),
    (
        0x0011_58b8,
        input_quantize_stick_direction,
        "Input_QuantizeStickDirection",
    ),
    (0x003b_76e8, sndstrm_queue_request, "SNDSTRM_QueueRequest"),
];

/// `PathData` track path table: entry count at +0x10, entries (0x3c bytes
/// each) at +0x14.
const PATH_DATA_TRACK_COUNT: u32 = 0x10;
const PATH_DATA_TRACK_ENTRIES: u32 = 0x14;
const PATH_DATA_TRACK_ENTRY_SIZE: u32 = 0x3c;

/// `int PathData_GetTrackPathIndex(PathData* tree, TrackPath* path)`
/// (0x26b508): the index of `path` in the tree's track path array, or -1
/// (also for a null path). The loop checks events at its head 0x26b528.
pub fn path_data_get_track_path_index(c: &mut Cpu) {
    c.cycles += 2;
    let path = c.g[A1][0];
    if path != 0 {
        c.cycles += 3;
        let tree = Addr(c.g[A0][0] as u32);
        let count = tree.off(PATH_DATA_TRACK_COUNT).lw(c);
        c.g[V0][0] = count;
        c.g[V1][0] = 0;
        if (count as i64) > 0 {
            c.cycles += 3;
            let mut entry = tree.off(PATH_DATA_TRACK_ENTRIES).lw(c);
            let mut index: u64 = 0;
            let mut count = count;
            c.g[A2][0] = count;
            loop {
                c.cycles += 2;
                if entry == path {
                    c.g[A0][0] = entry;
                    c.g[V1][0] = index;
                    c.cycles += 2;
                    c.g[V0][0] = index;
                    ret(c);
                    return;
                }
                index = sx((index as u32).wrapping_add(1));
                c.cycles += 3;
                let more = (index as i64) < (count as i64);
                entry = sx((entry as u32).wrapping_add(PATH_DATA_TRACK_ENTRY_SIZE));
                if !more {
                    c.g[A0][0] = entry;
                    c.g[V1][0] = index;
                    c.g[V0][0] = 0;
                    break;
                }
                if event_due(c) {
                    c.g[A0][0] = entry;
                    c.g[V1][0] = index;
                    c.g[V0][0] = 1;
                    deliver_event_here(c, 0x0026_b528);
                    entry = c.g[A0][0];
                    index = c.g[V1][0];
                    count = c.g[A2][0];
                }
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = u64::MAX;
    ret(c);
}

/// The world pointer (`gp-0x848`).
const REPLAY_FIND_WORLD_GP: u32 = 0x848;
/// World -> rider list (+0x84), list -> rider table (+0xc), table -> rider
/// count (+0x7c) and rider pointers (+0x40, one word each).
const REPLAY_FIND_WORLD_RIDERS: u32 = 0x84;
const REPLAY_FIND_LIST_TABLE: u32 = 0xc;
const REPLAY_FIND_TABLE_COUNT: u32 = 0x7c;
const REPLAY_FIND_TABLE_RIDERS: u32 = 0x40;
/// Rider -> object (+0x18); the replay id object is embedded at +0x6c0,
/// its id getter is vtable slot 0x38.
const REPLAY_FIND_RIDER_OBJ: u32 = 0x18;
const REPLAY_FIND_OBJ_ID: u32 = 0x6c0;
const REPLAY_FIND_ID_SLOT: u32 = 0x38;

/// `world` as `lw gp-0x848` leaves it.
fn replay_find_world(c: &mut Cpu) -> u64 {
    Addr((c.g[GP][0] as u32).wrapping_sub(REPLAY_FIND_WORLD_GP)).lw(c)
}

/// The world's rider count (`world->riders->table->count`), leaving the
/// table pointer in v1 as the original does.
fn replay_find_rider_count(c: &mut Cpu, world: u64) -> u64 {
    let list = Addr((world as u32).wrapping_add(REPLAY_FIND_WORLD_RIDERS)).lw(c);
    c.g[A0][0] = list;
    let table = Addr((list as u32).wrapping_add(REPLAY_FIND_LIST_TABLE)).lw(c);
    table
}

/// `int Replay_FindRiderIndex(int id)` (0x270730): the index of the world
/// rider whose replay id object (rider object +0x6c0, virtual getter at slot
/// 0x38) returns `id`, or -1. The loop checks events at its head 0x270768.
pub fn replay_find_rider_index(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        let world = replay_find_world(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[A1][0] = world;
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        let table = replay_find_rider_count(c, world);
        c.g[V0][0] = table;
        let count = Addr((table as u32).wrapping_add(REPLAY_FIND_TABLE_COUNT)).lw(c);
        c.g[V1][0] = count;
        c.g[S0][0] = 0;
        let mut found = false;
        if (count as i64) > 0 {
            c.cycles += 2;
            c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(REPLAY_FIND_WORLD_RIDERS)).lw(c);
            loop {
                c.cycles += 13;
                let index = c.g[S0][0] as u32;
                c.g[A0][0] = sx(index << 2);
                let table = Addr((c.g[V1][0] as u32).wrapping_add(REPLAY_FIND_LIST_TABLE)).lw(c);
                let slot = sx((table as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V0][0] = slot;
                let rider = Addr((slot as u32).wrapping_add(REPLAY_FIND_TABLE_RIDERS)).lw(c);
                c.g[V1][0] = rider;
                let obj = Addr((rider as u32).wrapping_add(REPLAY_FIND_RIDER_OBJ)).lw(c);
                let vt = Addr((obj as u32).wrapping_add(REPLAY_FIND_OBJ_ID)).lw(c);
                c.g[A1][0] = vt;
                let id_obj = sx((obj as u32).wrapping_add(REPLAY_FIND_OBJ_ID));
                let delta = Addr((vt as u32).wrapping_add(REPLAY_FIND_ID_SLOT)).lh(c);
                c.g[V1][0] = delta;
                let getter = Addr((vt as u32).wrapping_add(REPLAY_FIND_ID_SLOT + 4)).lw(c);
                c.g[V0][0] = getter;
                c.g[A0][0] = sx((id_obj as u32).wrapping_add(delta as u32));
                call_ptr(c, getter as u32, 0x0027_0794)?;
                let differs = c.g[V0][0] != c.g[S1][0];
                c.g[V0][0] = replay_find_world(c);
                if !differs {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S0][0];
                    found = true;
                    break;
                }
                c.cycles += 8;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                let world = c.g[V0][0];
                let table = replay_find_rider_count(c, world);
                c.g[A1][0] = world;
                c.g[V1][0] = table;
                let count = Addr((table as u32).wrapping_add(REPLAY_FIND_TABLE_COUNT)).lw(c);
                let more = (c.g[S0][0] as i64) < (count as i64);
                c.g[V0][0] = more as u64;
                if !more {
                    break;
                }
                c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(REPLAY_FIND_WORLD_RIDERS)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0027_0768);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = u64::MAX;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `AI` object lists: players (inline pointer array at +0x40, count at
/// +0x7c), mission riders (+0x48, +0x80) and actors (+0x5c, +0x84).
const AI_SAVE_PLAYERS: u32 = 0x40;
const AI_SAVE_MISSION: u32 = 0x48;
const AI_SAVE_ACTORS: u32 = 0x5c;
const AI_SAVE_NUM_PLAYERS: u32 = 0x7c;
const AI_SAVE_NUM_MISSION: u32 = 0x80;
const AI_SAVE_NUM_ACTORS: u32 = 0x84;
/// Replay frame vtable: `Write(frame, src, size)` at slot 0x8.
const AI_SAVE_FRAME_WRITE: u32 = 0x8;
/// Rider -> object (+0x18) -> replay object (+0x77c); its vtable (+0xde8)
/// has `Save(obj, frame)` at slot 0x10.
const AI_SAVE_RIDER_OBJ: u32 = 0x18;
const AI_SAVE_OBJ_REPLAY: u32 = 0x77c;
const AI_SAVE_REPLAY_VT: u32 = 0xde8;
const AI_SAVE_REPLAY_SAVE: u32 = 0x10;
/// `WorldPainter_SaveViewState(frame)`.
const AI_SAVE_VIEW_STATE: u32 = 0x002e_f248;

/// One object loop of `AI_SaveState`, entered with a positive count: for
/// each entry of the inline list at `self + list`, call its replay
/// object's `Save(frame)` (vtable slot 0x10). `self` is s2, the frame s3,
/// the index s1 and the list cursor s0; the back edge (loop head `head`)
/// checks events. Leaves v0 = 0 (the failed `i < count` test).
fn ai_save_state_objs(
    c: &mut Cpu,
    list: u32,
    count: u32,
    head: u32,
    back: u32,
) -> crate::call::Flow {
    c.cycles += 3;
    c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(list));
    c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
    loop {
        c.cycles += 14;
        c.g[A1][0] = c.g[S3][0];
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
        let rider = c.g[V1][0] as u32;
        let obj = Addr(rider.wrapping_add(AI_SAVE_RIDER_OBJ)).lw(c);
        c.g[V0][0] = obj;
        let replay = Addr((obj as u32).wrapping_add(AI_SAVE_OBJ_REPLAY)).lw(c);
        c.g[A3][0] = replay;
        let vt = Addr((replay as u32).wrapping_add(AI_SAVE_REPLAY_VT)).lw(c);
        c.g[V1][0] = vt;
        let delta = Addr((vt as u32).wrapping_add(AI_SAVE_REPLAY_SAVE)).lh(c);
        let save = Addr((vt as u32).wrapping_add(AI_SAVE_REPLAY_SAVE + 4)).lw(c);
        c.g[A2][0] = save;
        c.g[A0][0] = sx((replay as u32).wrapping_add(delta as u32));
        call_ptr(c, save as u32, back)?;
        let n = Addr((c.g[S2][0] as u32).wrapping_add(count)).lw(c);
        let more = (c.g[S1][0] as i64) < (n as i64);
        c.g[V0][0] = more as u64;
        if !more {
            return Ok(());
        }
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `void AI_SaveState(AI* self, cReplayFrame* frame)` (0x12b7f0): write
/// the AI block's first 0x28 bytes into the replay frame (its virtual
/// `Write`), let every player, mission rider and actor save its replay
/// state, then save the world painter's view state.
pub fn ai_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = 0x28;
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[S2][0];
        let replay_frame = c.g[S3][0] as u32;
        let vt = Addr(replay_frame).lw(c);
        c.g[V0][0] = vt;
        let delta = Addr((vt as u32).wrapping_add(AI_SAVE_FRAME_WRITE)).lh(c);
        let write = Addr((vt as u32).wrapping_add(AI_SAVE_FRAME_WRITE + 4)).lw(c);
        c.g[V1][0] = write;
        c.g[A0][0] = sx(replay_frame.wrapping_add(delta as u32));
        call_ptr(c, write as u32, 0x0012_b830)?;

        // Players.
        let n = Addr((c.g[S2][0] as u32).wrapping_add(AI_SAVE_NUM_PLAYERS)).lw(c);
        if (n as i64) > 0 {
            ai_save_state_objs(
                c,
                AI_SAVE_PLAYERS,
                AI_SAVE_NUM_PLAYERS,
                0x0012_b848,
                0x0012_b870,
            )?;
            c.cycles += 1;
        }
        c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(AI_SAVE_NUM_MISSION)).lw(c);

        // Mission riders.
        c.cycles += 2;
        c.g[S1][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            ai_save_state_objs(
                c,
                AI_SAVE_MISSION,
                AI_SAVE_NUM_MISSION,
                0x0012_b898,
                0x0012_b8c0,
            )?;
        }

        // Actors.
        c.cycles += 3;
        let n = Addr((c.g[S2][0] as u32).wrapping_add(AI_SAVE_NUM_ACTORS)).lw(c);
        c.g[V0][0] = n;
        c.g[S1][0] = 0;
        if (n as i64) > 0 {
            ai_save_state_objs(
                c,
                AI_SAVE_ACTORS,
                AI_SAVE_NUM_ACTORS,
                0x0012_b8e8,
                0x0012_b910,
            )?;
        }

        c.cycles += 9;
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, AI_SAVE_VIEW_STATE, 0x0012_b928)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Replay/save stream vtable: `Write(stream, src, size)` at slot 0x8.
const DPM_SAVE_STREAM_WRITE: u32 = 0x8;
/// `DynamicParticleModifier`: saved block at +0x10 (0x50 bytes), "static"
/// flag at +0x18, emitter state pointer at +0x260 (0xd8 bytes), embedded
/// `DynamicEmitter` at +0x60.
const DPM_SAVE_BLOCK: u32 = 0x10;
const DPM_SAVE_BLOCK_SIZE: u64 = 0x50;
const DPM_SAVE_STATIC: u32 = 0x18;
const DPM_SAVE_STATE: u32 = 0x260;
const DPM_SAVE_STATE_SIZE: u64 = 0xd8;
const DPM_SAVE_EMITTER: u32 = 0x60;

/// `stream->Write(src, size)` through the stream's vtable slot 0x8,
/// returning to `back` (v0 = vtable, v1 = method, as the original leaves
/// them).
fn dpm_save_stream_write(c: &mut Cpu, back: u32) -> crate::call::Flow {
    let stream = c.g[S0][0] as u32;
    let vt = Addr(stream).lw(c);
    c.g[V0][0] = vt;
    let delta = Addr((vt as u32).wrapping_add(DPM_SAVE_STREAM_WRITE)).lh(c);
    let write = Addr((vt as u32).wrapping_add(DPM_SAVE_STREAM_WRITE + 4)).lw(c);
    c.g[V1][0] = write;
    c.g[A0][0] = sx(stream.wrapping_add(delta as u32));
    call_ptr(c, write as u32, back)
}

/// `void DynamicParticleModifier_Save(DynamicParticleModifier* self,
/// Stream* stream)` (0x3460a0): the base modifier's save, then the 0x50
/// byte block at +0x10; a non-static modifier also writes its 0xd8 byte
/// state (+0x260) and saves its embedded emitter (`DynamicEmitter_Save`).
pub fn dynamic_particle_modifier_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        Callee {
            addr: 0x0034_5890,
            f: ee::s11::ParticleModifier_BaseSave,
        }
        .call(c, 0x0034_60bc)?;
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(DPM_SAVE_BLOCK));
        c.g[A2][0] = DPM_SAVE_BLOCK_SIZE;
        dpm_save_stream_write(c, 0x0034_60d8)?;
        let is_static = Addr((c.g[S1][0] as u32).wrapping_add(DPM_SAVE_STATIC)).lw(c);
        c.g[V0][0] = is_static;
        if is_static == 0 {
            c.cycles += 11;
            c.g[A2][0] = DPM_SAVE_STATE_SIZE;
            c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(DPM_SAVE_STATE)).lw(c);
            dpm_save_stream_write(c, 0x0034_6100)?;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(DPM_SAVE_EMITTER));
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0037_12b8, 0x0034_610c)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The replay stream interface's `Write(stream, src, size)` slot.
const SUB_SAVE_STATE_WRITE: u32 = 0x8;

/// The shared body of the rider sub-state `SaveState(self, stream)`
/// forwarders: `stream->Write(self, size)` through vtable slot 0x8 in a
/// 0x10 frame (the stream kept in a3, `self` passed as the source).
fn sub_save_state_write(c: &mut Cpu, size: u32, back: u32) -> crate::call::Flow {
    c.cycles += 13;
    let frame = Frame::open(c, 0x10);
    c.g[A3][0] = c.g[A1][0];
    let stream = c.g[A3][0];
    frame.save_ra(c, 0);
    c.g[A1][0] = c.g[A0][0];
    c.g[A2][0] = sx(size);
    let vt = Addr(stream as u32).lw(c);
    c.g[V1][0] = vt;
    let delta = Addr((vt as u32).wrapping_add(SUB_SAVE_STATE_WRITE)).lh(c);
    let write = Addr((vt as u32).wrapping_add(SUB_SAVE_STATE_WRITE + 4)).lw(c);
    c.g[V0][0] = write;
    c.g[A0][0] = sx((stream as u32).wrapping_add(delta as u32));
    call_ptr(c, write as u32, back)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void RiderUberGrindSub_SaveState(RiderUberGrindSub* self, Stream* s)`
/// (0x1368e8): write the sub-state's 8 bytes into the replay stream.
pub fn rider_uber_grind_sub_save_state(c: &mut Cpu) {
    guest(c, |c| sub_save_state_write(c, 0x8, 0x0013_6910))
}

/// `void RiderFinishLineSub_SaveState(RiderFinishLineSub* self, Stream* s)`
/// (0x12c9c0): write the sub-state's 4 bytes into the replay stream.
pub fn rider_finish_line_sub_save_state(c: &mut Cpu) {
    guest(c, |c| sub_save_state_write(c, 0x4, 0x0012_c9e8))
}

/// `ModifierBlock`: head of the rail modifier list at +0x1c. A list link:
/// next at +0, vtable at +0x8 (`Release(link, how)` at slot 0x8), id at
/// +0x30.
const MOD_BLOCK_RAIL_HEAD: u32 = 0x1c;
const MOD_BLOCK_LINK_NEXT: u32 = 0x0;
const MOD_BLOCK_LINK_VT: u32 = 0x8;
const MOD_BLOCK_LINK_ID: u32 = 0x30;
const MOD_BLOCK_RELEASE: u32 = 0x8;

/// `link->Release(3)` through the link's vtable (link in a2), returning to
/// `back`.
fn mod_block_release_link(c: &mut Cpu, back: u32) -> crate::call::Flow {
    let link = c.g[A2][0] as u32;
    let vt = Addr(link.wrapping_add(MOD_BLOCK_LINK_VT)).lw(c);
    c.g[V0][0] = vt;
    c.g[A1][0] = 3;
    let delta = Addr((vt as u32).wrapping_add(MOD_BLOCK_RELEASE)).lh(c);
    let release = Addr((vt as u32).wrapping_add(MOD_BLOCK_RELEASE + 4)).lw(c);
    c.g[V1][0] = release;
    c.g[A0][0] = sx(link.wrapping_add(delta as u32));
    call_ptr(c, release as u32, back)
}

/// `void ModifierBlock_FreeRailModifiers(ModifierBlock* self, u32 id)`
/// (0x353300): release (`Release(3)`) the rail modifier with `id`, or every
/// rail modifier when `id` is -1. Both list walks check events at their
/// heads (0x353320, 0x353358).
pub fn modifier_block_free_rail_modifiers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let head = Addr((c.g[A0][0] as u32).wrapping_add(MOD_BLOCK_RAIL_HEAD)).lw(c);
        c.g[A0][0] = head;
        let id = c.g[A1][0];
        c.g[V0][0] = !id;
        let mut free_all_done = head == 0;
        if head != 0 {
            c.cycles += 2;
            c.g[A2][0] = head;
            if !id != 0 {
                // Release the one modifier whose id matches.
                loop {
                    c.cycles += 2;
                    if c.g[A2][0] == 0 {
                        break;
                    }
                    c.cycles += 3;
                    let link_id = Addr((c.g[A2][0] as u32).wrapping_add(MOD_BLOCK_LINK_ID)).lw(c);
                    c.g[V0][0] = link_id;
                    if link_id == c.g[A1][0] {
                        c.cycles += 8;
                        mod_block_release_link(c, 0x0035_334c)?;
                        break;
                    }
                    c.g[A2][0] = Addr((c.g[A2][0] as u32).wrapping_add(MOD_BLOCK_LINK_NEXT)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0035_3320);
                    }
                }
            } else {
                // Release every modifier (the next link read first).
                loop {
                    c.cycles += 2;
                    let link = c.g[A2][0];
                    c.g[S0][0] = Addr((link as u32).wrapping_add(MOD_BLOCK_LINK_NEXT)).lw(c);
                    if link != 0 {
                        c.cycles += 6;
                        mod_block_release_link(c, 0x0035_3378)?;
                    }
                    c.cycles += 3;
                    c.g[A2][0] = c.g[S0][0];
                    if c.g[A2][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0035_3358);
                    }
                }
                free_all_done = true;
            }
        }
        if free_all_done {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `.lit4` constants of `Input_QuantizeStickDirection` (gp-relative):
/// the angles for a vertical stick (y < 0, y > 0), the atan quadrant
/// corrections (added for y > 0, subtracted otherwise) and the final
/// snap-to-zero threshold.
const QUANTIZE_STICK_DOWN_ANGLE: u32 = 0x7b68;
const QUANTIZE_STICK_UP_ANGLE: u32 = 0x7b64;
const QUANTIZE_STICK_ADD_PI: u32 = 0x7b60;
const QUANTIZE_STICK_SUB_PI: u32 = 0x7b5c;
const QUANTIZE_STICK_SNAP: u32 = 0x7b58;
/// `Math_Atan(f12)` and `Math_SinCos(sin*, cos*, f12)`.
const QUANTIZE_STICK_ATAN: u32 = 0x0031_c228;
const QUANTIZE_STICK_SINCOS: u32 = 0x0031_be50;

/// A `.lit4` float at `gp - off`.
fn quantize_stick_lit(c: &mut Cpu, off: u32) -> FpuF32 {
    Addr((c.g[GP][0] as u32).wrapping_sub(off)).read_fpu(c)
}

/// Zero `*p` when `|*p| <= eps` (`le`) or `|*p| < eps` (`!le`), leaving
/// `|*p|` in f0 and the compare in the FPU condition bit.
fn quantize_stick_snap(c: &mut Cpu, p: u32, eps: FpuF32, le: bool) {
    let v = Addr(p).read_fpu(c).abs();
    c.f[0] = v.0;
    let small = if le { v.le(eps) } else { v.lt(eps) };
    crate::abi::set_fcc(c, small);
    if small {
        Addr(p).sw(c, 0);
    }
}

/// `void Input_QuantizeStickDirection(Rider*, float* y, float* x, float
/// step, float eps)` (0x1158b8): snap the stick vector (x, y) to the
/// nearest multiple of the angle `step`, keeping its length (clamped to
/// [-1, 1] per axis); components within `eps` count as zero, and results
/// below the snap threshold become zero.
pub fn input_quantize_stick_direction(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A2][0];
        frame.save_fpr(c, 24, 0x60);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.f[24] = c.f[12];
        frame.save_fpr(c, 23, 0x58);
        frame.save_fpr(c, 22, 0x50);
        frame.save_fpr(c, 21, 0x48);
        frame.save_fpr(c, 20, 0x40);
        let x_ptr = c.g[S0][0] as u32;
        let y_ptr = c.g[S1][0] as u32;
        let eps = FpuF32(c.f[13]);
        quantize_stick_snap(c, x_ptr, eps, true);
        c.cycles += 6;
        quantize_stick_snap(c, y_ptr, eps, true);

        c.cycles += 11;
        let x = Addr(x_ptr).read_fpu(c);
        let y = Addr(y_ptr).read_fpu(c);
        c.f[21] = x.0;
        c.f[20] = y.0;
        let xx = x * x;
        let yy = y * y;
        c.f[1] = yy.0;
        let zero = FpuF32::ZERO;
        c.f[22] = zero.0;
        c.f[0] = (xx + yy).0;
        let len = (xx + yy).sqrt();
        c.f[23] = len.0;
        let still = len.eq_s(zero);
        crate::abi::set_fcc(c, still);
        if !still {
            // The stick angle (atan2(y, x)) in f1.
            c.cycles += 3;
            let vertical = x.eq_s(zero);
            crate::abi::set_fcc(c, vertical);
            let angle;
            if vertical {
                c.cycles += 4;
                let y_zero = y.eq_s(zero);
                crate::abi::set_fcc(c, y_zero);
                if y_zero {
                    angle = y;
                } else {
                    c.cycles += 4;
                    let up = zero.le(y);
                    crate::abi::set_fcc(c, up);
                    let down = quantize_stick_lit(c, QUANTIZE_STICK_DOWN_ANGLE);
                    if up {
                        c.cycles += 3;
                        angle = quantize_stick_lit(c, QUANTIZE_STICK_UP_ANGLE);
                    } else {
                        angle = down;
                    }
                }
            } else {
                c.cycles += 7;
                c.f[12] = y.div(x).0;
                call_ptr(c, QUANTIZE_STICK_ATAN, 0x0011_5988)?;
                let x = FpuF32(c.f[21]);
                let y = FpuF32(c.f[20]);
                let zero = FpuF32(c.f[22]);
                let left = x.lt(zero);
                crate::abi::set_fcc(c, left);
                let a = FpuF32(c.f[0]);
                if !left {
                    angle = a;
                } else {
                    c.cycles += 3;
                    let up = zero.lt(y);
                    crate::abi::set_fcc(c, up);
                    if up {
                        c.cycles += 3;
                        let k = quantize_stick_lit(c, QUANTIZE_STICK_ADD_PI);
                        c.f[0] = k.0;
                        angle = a + k;
                    } else {
                        c.cycles += 2;
                        let k = quantize_stick_lit(c, QUANTIZE_STICK_SUB_PI);
                        c.f[0] = k.0;
                        angle = a - k;
                    }
                }
            }
            c.f[1] = angle.0;

            // Round angle / step to the nearest integer (half away from 0).
            c.cycles += 7;
            let step = FpuF32(c.f[24]);
            let steps = angle.div(step);
            c.f[2] = steps.0;
            let negative = angle.lt(FpuF32::ZERO);
            crate::abi::set_fcc(c, negative);
            let half: u32 = if negative {
                c.cycles += 3;
                0xbf00_0000
            } else {
                0x3f00_0000
            };
            c.g[AT][0] = sx(half);
            let rounded = steps + FpuF32(half);
            c.f[0] = rounded.0;

            c.cycles += 18;
            let n = rounded.to_i32();
            c.f[1] = n as u32;
            c.g[V0][0] = sx(n as u32);
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = c.g[SP][0] | 0x4;
            c.f[12] = (FpuF32::from_i32(n) * step).0;
            call_ptr(c, QUANTIZE_STICK_SINCOS, 0x0011_5a00)?;

            // x = clamp(cos * len), y = clamp(sin * len).
            let len = FpuF32(c.f[23]);
            let cos = Addr(sp(c).wrapping_add(4)).read_fpu(c);
            let lo = FpuF32(0xbf80_0000);
            let hi = FpuF32(0x3f80_0000);
            c.f[1] = lo.0;
            let new_x = cos * len;
            c.f[2] = hi.0;
            c.g[AT][0] = sx(hi.0);
            let above = lo.le(new_x);
            crate::abi::set_fcc(c, above);
            let mut clamped_x = lo;
            if above {
                c.cycles += 1;
                clamped_x = new_x.min(hi);
            }
            c.f[4] = clamped_x.0;

            c.cycles += 7;
            let sin = Addr(sp(c)).read_fpu(c);
            let new_y = sin * len;
            c.f[0] = new_y.0;
            let above = lo.le(new_y);
            crate::abi::set_fcc(c, above);
            Addr(c.g[S0][0] as u32).write_fpu(c, clamped_x);
            let mut clamped_y = lo;
            if above {
                c.cycles += 1;
                clamped_y = new_y.min(hi);
            }
            c.f[3] = clamped_y.0;

            c.cycles += 8;
            Addr(c.g[S1][0] as u32).write_fpu(c, clamped_y);
            let snap = quantize_stick_lit(c, QUANTIZE_STICK_SNAP);
            c.f[1] = snap.0;
            let x_ptr = c.g[S0][0] as u32;
            let y_ptr = c.g[S1][0] as u32;
            quantize_stick_snap(c, x_ptr, snap, false);
            c.cycles += 6;
            quantize_stick_snap(c, y_ptr, snap, false);
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 9;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 24, 0x60);
        frame.restore_fpr(c, 23, 0x58);
        frame.restore_fpr(c, 22, 0x50);
        frame.restore_fpr(c, 21, 0x48);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `SNDSTRM` stream: file handle at +0, request id counter at +0xc,
/// queued requests list at +0x11c, free request list at +0x128 (its count
/// at +0x130). A request node: data at +0x8, id | flags at +0xc, user
/// word at +0x20 (0x28 bytes, cleared on reuse).
const SNDSTRM_QUEUE_HANDLE: u32 = 0x0;
const SNDSTRM_QUEUE_NEXT_ID: u32 = 0xc;
const SNDSTRM_QUEUE_QUEUED: u32 = 0x11c;
const SNDSTRM_QUEUE_FREE: u32 = 0x128;
const SNDSTRM_QUEUE_FREE_COUNT: u32 = 0x130;
const SNDSTRM_QUEUE_REQ_DATA: u32 = 0x8;
const SNDSTRM_QUEUE_REQ_ID: u32 = 0xc;
const SNDSTRM_QUEUE_REQ_USER: u32 = 0x20;
const SNDSTRM_QUEUE_REQ_SIZE: u64 = 0x28;
/// The stream-file owner tag `'lECS'` passed to the file queue.
const SNDSTRM_QUEUE_TAG: u64 = 0x6c45_4353;
/// `StreamFile_QueueRange`, `StreamFile_QueueFile`, `memset`.
const SNDSTRM_QUEUE_RANGE: u32 = 0x003e_0c88;
const SNDSTRM_QUEUE_FILE: u32 = 0x003e_0d80;
const SNDSTRM_QUEUE_MEMSET: u32 = 0x0041_6210;

/// `int SNDSTRM_QueueRequest(int stream, int user, int a2, int a3, int
/// mode)` (0x3b76e8): queue a request on sound stream `stream`. `mode` 0
/// queues the file range (a2, a3), mode 1 the whole file a2, any other
/// mode takes a3 as the already-queued file request. Takes a free request
/// node, links it into the queued list with the next request id (steps of
/// 0x100, back to 0 past 2^31, or'ed with `stream`) and returns that id;
/// -8 for no stream, -13 for no free node, -1 when the file queue failed.
pub fn sndstrm_queue_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S4, 0x20);
        frame.save(c, S1, 0x50);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[T0][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S5, 0x10);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S0, 0x60);
        frame.save_ra(c, 0);
        c.g[S5][0] = c.g[A1][0];
        Callee {
            addr: 0x003b_6a38,
            f: ee::s13::SNDSTRM_GetStream,
        }
        .call(c, 0x003b_7720)?;
        c.g[S0][0] = c.g[V0][0];
        'done: {
            c.g[V0][0] = (-8i64) as u64;
            if c.g[S0][0] == 0 {
                break 'done;
            }
            c.cycles += 3;
            let free = Addr((c.g[S0][0] as u32).wrapping_add(SNDSTRM_QUEUE_FREE_COUNT)).lw(c);
            c.g[V0][0] = (-13i64) as u64;
            if free == 0 {
                break 'done;
            }
            c.cycles += 2;
            c.g[V0][0] = 1;
            if c.g[S1][0] == 0 {
                c.cycles += 8;
                c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(SNDSTRM_QUEUE_HANDLE)).lw(c);
                c.g[A2][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S3][0];
                c.g[A3][0] = SNDSTRM_QUEUE_TAG;
                call_ptr(c, SNDSTRM_QUEUE_RANGE, 0x003b_7758)?;
                c.g[S2][0] = c.g[V0][0];
            } else {
                c.cycles += 2;
                c.g[A1][0] = c.g[S3][0];
                if c.g[S1][0] == c.g[V0][0] {
                    c.cycles += 6;
                    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(SNDSTRM_QUEUE_HANDLE)).lw(c);
                    c.g[A2][0] = 0;
                    c.g[A3][0] = SNDSTRM_QUEUE_TAG;
                    call_ptr(c, SNDSTRM_QUEUE_FILE, 0x003b_777c)?;
                    c.g[S2][0] = c.g[V0][0];
                }
            }
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
            if c.g[S2][0] == 0 {
                break 'done;
            }

            // Take a free node, clear it, queue it.
            c.cycles += 19;
            Callee {
                addr: 0x003b_58a0,
                f: ee::s13::SND_Lock,
            }
            .call(c, 0x003b_7790)?;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(SNDSTRM_QUEUE_FREE));
            Callee {
                addr: 0x003c_62e8,
                f: ee::s13::SND_ListPopFront,
            }
            .call(c, 0x003b_7798)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = SNDSTRM_QUEUE_REQ_SIZE;
            call_ptr(c, SNDSTRM_QUEUE_MEMSET, 0x003b_77ac)?;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(SNDSTRM_QUEUE_QUEUED));
            c.g[A1][0] = c.g[S1][0];
            Callee {
                addr: 0x003c_62b8,
                f: ee::s13::SND_ListPushBack,
            }
            .call(c, 0x003b_77b8)?;
            let stream = c.g[S0][0] as u32;
            let req = c.g[S1][0] as u32;
            let data = c.g[S2][0];
            Addr(req.wrapping_add(SNDSTRM_QUEUE_REQ_DATA)).sw(c, data);
            let next = Addr(stream.wrapping_add(SNDSTRM_QUEUE_NEXT_ID)).lw(c);
            let next = sx((next as u32).wrapping_add(0x100));
            c.g[V0][0] = next;
            Addr(stream.wrapping_add(SNDSTRM_QUEUE_NEXT_ID)).sw(c, next);
            let next = Addr(stream.wrapping_add(SNDSTRM_QUEUE_NEXT_ID)).lw(c);
            c.g[V1][0] = next;
            if (next as i64) < 0 {
                Addr(stream.wrapping_add(SNDSTRM_QUEUE_NEXT_ID)).sw(c, 0);
            }

            c.cycles += 8;
            let id = Addr(stream.wrapping_add(SNDSTRM_QUEUE_NEXT_ID)).lw(c) | c.g[S4][0];
            c.g[V0][0] = id;
            Addr(req.wrapping_add(SNDSTRM_QUEUE_REQ_ID)).sw(c, id);
            let user = c.g[S5][0];
            Addr(req.wrapping_add(SNDSTRM_QUEUE_REQ_USER)).sw(c, user);
            c.g[S0][0] = Addr(req.wrapping_add(SNDSTRM_QUEUE_REQ_ID)).lw(c);
            Callee {
                addr: 0x003b_58d8,
                f: ee::s13::SND_Unlock,
            }
            .call(c, 0x003b_77f0)?;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule};
    use crate::difftest::{check, scramble, Rng, DATA, RA as SENTINEL};

    /// A tree whose track path array holds 0..6 entries, asked for a null
    /// path, one of its entries, or a pointer that is not in the array.
    fn track_path_index_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) {
        scramble(c, r);
        let tree = DATA;
        let entries = DATA + 0x1000;
        let count = match seed % 7 {
            0 => 0,
            1 => (-(1 + (r.next_u32() % 3) as i32)) as u32,
            _ => 1 + r.next_u32() % 6,
        };
        c.bus.write32(tree + PATH_DATA_TRACK_COUNT, count);
        c.bus.write32(tree + PATH_DATA_TRACK_ENTRIES, entries);
        let n = if (count as i32) > 0 { count } else { 1 };
        let path = match seed % 5 {
            0 => 0,
            1 => entries + 0x3c * n + 4,
            2 => 0xdead_0000,
            _ => entries + 0x3c * (r.next_u32() % n),
        };
        c.g[A0][0] = sx(tree);
        c.g[A1][0] = sx(path);
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x0026_b508;
    }

    /// `PathData_GetTrackPathIndex`: found, not found, null, empty array.
    #[test]
    fn path_data_get_track_path_index_matches() {
        check(
            ee::s07::f_0026b508__generated,
            path_data_get_track_path_index,
            300,
            &|c, seed| {
                let mut r = Rng(0x26b5_0800 ^ seed as u64);
                track_path_index_fixture(c, &mut r, seed);
            },
        );
    }

    /// Event delivery at the search loop head (0x26b528).
    #[test]
    fn path_data_get_track_path_index_events_match() {
        check_events(
            ee::s07::f_0026b508__generated,
            path_data_get_track_path_index,
            150,
            &|c, seed| {
                let mut r = Rng(0x26b5_0801 ^ seed as u64);
                track_path_index_fixture(c, &mut r, 3 + seed % 4);
                // Long array, path near the end (or absent).
                c.bus.write32(DATA + PATH_DATA_TRACK_COUNT, 6);
                let path = if seed % 2 == 0 {
                    DATA + 0x1000 + 0x3c * 5
                } else {
                    4
                };
                c.g[A1][0] = sx(path);
                schedule(c, &mut r, seed, 0x0026_b508)
            },
        );
    }

    /// A world with 0..5 riders (or a negative count), each rider object's
    /// replay id getter a fake method; the fake for rider `hit` (if any)
    /// returns the id asked for, the others a pointer.
    fn replay_find_rider_fixture(c: &mut Cpu, seed: u32) {
        use crate::difftest::fakes::{machine, FAKE, LOG_RESULTS};
        use crate::difftest::GAME_GP;
        let count = match seed % 6 {
            0 => 0u32,
            1 => (-(1 + (seed / 6 % 3) as i32)) as u32,
            _ => 1 + seed / 6 % 5,
        };
        let calls = if (count as i32) > 0 { count } else { 0 };
        let mut r = machine(c, seed, 0x2707_3000, calls);
        let world = DATA;
        let list = DATA + 0x100;
        let table = DATA + 0x200;
        c.bus.write32(GAME_GP - REPLAY_FIND_WORLD_GP, world);
        c.bus.write32(world + REPLAY_FIND_WORLD_RIDERS, list);
        c.bus.write32(list + REPLAY_FIND_LIST_TABLE, table);
        c.bus.write32(table + REPLAY_FIND_TABLE_COUNT, count);
        for i in 0..calls {
            let rider = DATA + 0x1000 + i * 0x100;
            let obj = DATA + 0x4000 + i * 0x1000;
            let vt = DATA + 0x10000 + i * 0x100;
            c.bus
                .write32(table + REPLAY_FIND_TABLE_RIDERS + 4 * i, rider);
            c.bus.write32(rider + REPLAY_FIND_RIDER_OBJ, obj);
            c.bus.write32(obj + REPLAY_FIND_OBJ_ID, vt);
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(vt + REPLAY_FIND_ID_SLOT, delta as u16);
            c.bus.write32(vt + REPLAY_FIND_ID_SLOT + 4, FAKE + 0x10 * i);
        }
        let id = 1 + r.next_u32() % 0x1000;
        // Rider `hit` answers with the id (none for seed % 4 == 0).
        if calls > 0 && seed % 4 != 0 {
            let hit = r.next_u32() % calls;
            c.bus.write32(LOG_RESULTS + 4 * hit, id);
        }
        c.g[A0][0] = if seed % 7 == 3 {
            (r.next_u32() as u64) << 32 | id as u64
        } else {
            id as u64
        };
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x0027_0730;
    }

    /// `Replay_FindRiderIndex`: found at any index, not found, no riders,
    /// fakes unwinding.
    #[test]
    fn replay_find_rider_index_matches() {
        check(
            ee::s07::f_00270730__generated,
            replay_find_rider_index,
            300,
            &replay_find_rider_fixture,
        );
    }

    /// The world painter's six view states (globals at 0x4fa370, stride
    /// 0xf0, field +0x20 -> holder -> object with a vtable at +4 whose
    /// slot 0x200 is a fake): `WorldPainter_SaveViewState` makes 12 fake
    /// calls.
    fn ai_save_view_state_fixture(c: &mut Cpu) {
        use crate::difftest::fakes::FAKE;
        let holder = DATA + 0x30000;
        let obj = DATA + 0x30100;
        let vt = DATA + 0x30200;
        for k in 0..6 {
            c.bus.write32(0x004f_a370 + 0x20 + k * 0xf0, holder);
        }
        c.bus.write32(holder, obj);
        c.bus.write32(obj + 4, vt);
        c.bus.write16(vt + 0x200, 0);
        c.bus.write32(vt + 0x204, FAKE + 0x800);
    }

    /// An AI block with 0..2 players, 0..5 mission riders and 0..8 actors
    /// (or negative counts), each rider's replay object saving through a
    /// fake; a replay frame whose `Write` is a fake.
    fn ai_save_state_fixture(c: &mut Cpu, seed: u32) {
        use crate::difftest::fakes::{machine, FAKE};
        let mut r = Rng(0x12b7_f0ff ^ (seed as u64) << 4);
        let mut pick = |max: u32| -> u32 {
            match r.next_u32() % 5 {
                0 => 0,
                1 => (-(1 + (r.next_u32() % 2) as i32)) as u32,
                _ => 1 + r.next_u32() % max,
            }
        };
        let counts = [pick(2), pick(5), pick(8)];
        let calls: u32 = 1
            + 12
            + counts
                .iter()
                .map(|&n| if (n as i32) > 0 { n } else { 0 })
                .sum::<u32>();
        let mut r = machine(c, seed, 0x12b7_f000, calls);
        let ai = DATA;
        let frame = DATA + 0x200;
        let frame_vt = DATA + 0x300;
        c.bus.write32(frame, frame_vt);
        let delta = (r.next_u32() as i16 as i32 & !3) as u32;
        c.bus.write16(frame_vt + AI_SAVE_FRAME_WRITE, delta as u16);
        c.bus.write32(frame_vt + AI_SAVE_FRAME_WRITE + 4, FAKE);
        let lists = [AI_SAVE_PLAYERS, AI_SAVE_MISSION, AI_SAVE_ACTORS];
        let count_offs = [AI_SAVE_NUM_PLAYERS, AI_SAVE_NUM_MISSION, AI_SAVE_NUM_ACTORS];
        let mut k = 0;
        for l in 0..3 {
            c.bus.write32(ai + count_offs[l], counts[l]);
            let n = if (counts[l] as i32) > 0 { counts[l] } else { 0 };
            for i in 0..n {
                let rider = DATA + 0x1000 + k * 0x100;
                let obj = DATA + 0x4000 + k * 0x800;
                let replay = DATA + 0x10000 + k * 0x1000;
                let vt = DATA + 0x20000 + k * 0x40;
                c.bus.write32(ai + lists[l] + 4 * i, rider);
                c.bus.write32(rider + AI_SAVE_RIDER_OBJ, obj);
                c.bus.write32(obj + AI_SAVE_OBJ_REPLAY, replay);
                c.bus.write32(replay + AI_SAVE_REPLAY_VT, vt);
                let delta = (r.next_u32() as i16 as i32 & !3) as u32;
                c.bus.write16(vt + AI_SAVE_REPLAY_SAVE, delta as u16);
                c.bus
                    .write32(vt + AI_SAVE_REPLAY_SAVE + 4, FAKE + 0x10 * (k + 1));
                k += 1;
            }
        }
        ai_save_view_state_fixture(c);
        c.g[A0][0] = sx(ai);
        c.g[A1][0] = sx(frame);
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x0012_b7f0;
    }

    /// `AI_SaveState`: empty and filled lists, fakes unwinding at any call.
    #[test]
    fn ai_save_state_matches() {
        check(
            ee::s00::f_0012b7f0__generated,
            ai_save_state,
            300,
            &ai_save_state_fixture,
        );
    }

    /// A dynamic particle modifier (static or not) and a stream whose
    /// vtable slots 0x8 (`Write`) and 0x28 (`WriteInt`, the base save) are
    /// fakes; the embedded emitter holds random words.
    fn dpm_save_fixture(c: &mut Cpu, seed: u32) {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::fill;
        let is_static = seed % 3 == 0;
        let calls = if is_static { 2 } else { 4 };
        let mut r = machine(c, seed, 0x3460_a000, calls);
        let modifier = DATA;
        let stream = DATA + 0x1000;
        let vt = DATA + 0x1100;
        fill(c, &mut r, modifier, 0x110);
        c.bus.write32(
            modifier + DPM_SAVE_STATIC,
            if is_static { 1 + r.next_u32() % 9 } else { 0 },
        );
        c.bus.write32(modifier + DPM_SAVE_STATE, DATA + 0x2000);
        c.bus.write32(stream, vt);
        for (i, slot) in [DPM_SAVE_STREAM_WRITE, 0x28].into_iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(vt + slot, delta as u16);
            c.bus.write32(vt + slot + 4, FAKE + 0x10 * i as u32);
        }
        c.g[A0][0] = sx(modifier);
        c.g[A1][0] = sx(stream);
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x0034_60a0;
    }

    /// `DynamicParticleModifier_Save`: static and dynamic modifiers, fakes
    /// unwinding at any call.
    #[test]
    fn dynamic_particle_modifier_save_matches() {
        check(
            ee::s11::f_003460a0__generated,
            dynamic_particle_modifier_save,
            300,
            &dpm_save_fixture,
        );
    }

    /// A sub-state at DATA+0x400 and a stream whose `Write` slot (0x8) is a
    /// fake that may unwind.
    fn sub_save_state_fixture(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        use crate::difftest::fakes::{machine, vtable};
        let mut r = machine(c, seed, salt, 1);
        let stream = DATA + 0x800;
        let vt = vtable(c, &mut r, &[SUB_SAVE_STATE_WRITE]);
        c.bus.write32(stream, vt);
        c.g[A0][0] = sx(DATA + 0x400);
        c.g[A1][0] = sx(stream);
        c.g[RA][0] = SENTINEL as u64;
        c.pc = entry;
    }

    /// `RiderUberGrindSub_SaveState`: the stream write, returning or
    /// unwinding.
    #[test]
    fn rider_uber_grind_sub_save_state_matches() {
        check(
            ee::s01::f_001368e8__generated,
            rider_uber_grind_sub_save_state,
            200,
            &|c, seed| sub_save_state_fixture(c, seed, 0x1368_e800, 0x0013_68e8),
        );
    }

    /// `RiderFinishLineSub_SaveState`: the stream write, returning or
    /// unwinding.
    #[test]
    fn rider_finish_line_sub_save_state_matches() {
        check(
            ee::s00::f_0012c9c0__generated,
            rider_finish_line_sub_save_state,
            200,
            &|c, seed| sub_save_state_fixture(c, seed, 0x12c9_c000, 0x0012_c9c0),
        );
    }

    /// A modifier block whose rail list holds 0..5 links (each with a
    /// fake `Release`), asked to free one id (present or not) or all (-1).
    fn mod_block_free_rail_fixture(c: &mut Cpu, seed: u32) {
        use crate::difftest::fakes::{machine, FAKE};
        let n = seed / 3 % 6;
        let mut r = machine(c, seed, 0x3533_0000, n.max(1));
        let block = DATA;
        let links = DATA + 0x1000;
        let mut ids = Vec::new();
        for i in 0..n {
            let link = links + i * 0x100;
            let next = if i + 1 < n { link + 0x100 } else { 0 };
            let vt = DATA + 0x8000 + i * 0x40;
            c.bus.write32(link + MOD_BLOCK_LINK_NEXT, next);
            c.bus.write32(link + MOD_BLOCK_LINK_VT, vt);
            let id = r.next_u32() & 0x7fff_ffff;
            ids.push(id);
            c.bus.write32(link + MOD_BLOCK_LINK_ID, id);
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(vt + MOD_BLOCK_RELEASE, delta as u16);
            c.bus.write32(vt + MOD_BLOCK_RELEASE + 4, FAKE + 0x10 * i);
        }
        c.bus
            .write32(block + MOD_BLOCK_RAIL_HEAD, if n > 0 { links } else { 0 });
        // Word 0 (the free-all walk reads `next` of a null link there).
        c.bus.write32(0, r.next_u32());
        c.g[A1][0] = match seed % 3 {
            0 => u64::MAX,
            1 if n > 0 => ids[(r.next_u32() % n) as usize] as u64,
            _ => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
        };
        c.g[A0][0] = sx(block);
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x0035_3300;
    }

    /// `ModifierBlock_FreeRailModifiers`: empty list, one id found or not,
    /// free all; fakes unwinding.
    #[test]
    fn modifier_block_free_rail_modifiers_matches() {
        check(
            ee::s11::f_00353300__generated,
            modifier_block_free_rail_modifiers,
            300,
            &mod_block_free_rail_fixture,
        );
    }

    /// Event delivery at both walk heads.
    #[test]
    fn modifier_block_free_rail_modifiers_events_match() {
        check_events(
            ee::s11::f_00353300__generated,
            modifier_block_free_rail_modifiers,
            150,
            &|c, seed| {
                // Free all (seed even) or an absent id over five links.
                let base = 15 + if seed % 2 == 0 { 0 } else { 2 };
                mod_block_free_rail_fixture(c, base);
                c.bus
                    .write32(crate::difftest::fakes::LOG_UNWIND_AT, u32::MAX);
                let mut r = Rng(0x3533_0001 ^ seed as u64);
                schedule(c, &mut r, seed, 0x0035_3300)
            },
        );
    }

    /// A stick vector (x at DATA, y at DATA+4): zero, on an axis, tiny
    /// (within eps), or anywhere in [-1.5, 1.5]^2; steps of pi/4, pi/8 or
    /// random; the real `Math_Atan` / `Math_SinCos` and the game's `.lit4`.
    fn quantize_stick_fixture(c: &mut Cpu, seed: u32) {
        use crate::difftest::{elf_image, GAME_GP};
        let mut r = Rng(0x1158_b800 ^ (seed as u64) << 8);
        scramble(c, &mut r);
        elf_image(c);
        c.g[GP][0] = GAME_GP as u64;
        let x_ptr = DATA;
        let y_ptr = DATA + 4;
        let mut comp = |k: u32| -> u32 {
            match k % 6 {
                0 => 0,
                1 => 0x8000_0000,
                2 => r.f32(0.05),
                3 => r.edge_f32(),
                _ => r.f32(1.5),
            }
        };
        let (x, y) = match seed % 5 {
            0 => (0, comp(seed / 5)),
            1 => (comp(seed / 5), 0),
            _ => (comp(seed / 5), comp(seed / 7 + 3)),
        };
        c.bus.write32(x_ptr, x);
        c.bus.write32(y_ptr, y);
        c.f[12] = match seed % 4 {
            0 => std::f32::consts::FRAC_PI_4.to_bits(),
            1 => std::f32::consts::FRAC_PI_8.to_bits(),
            2 => (0.3f32).to_bits(),
            _ => r.f32(2.0) & 0x7fff_ffff | 0x3c00_0000,
        };
        c.f[13] = match seed % 3 {
            0 => (0.1f32).to_bits(),
            1 => 0,
            _ => r.f32(0.2) & 0x7fff_ffff,
        };
        c.g[A0][0] = sx(DATA + 0x100);
        c.g[A1][0] = sx(y_ptr);
        c.g[A2][0] = sx(x_ptr);
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x0011_58b8;
    }

    /// `Input_QuantizeStickDirection`: still stick, vertical (up/down),
    /// horizontal, all quadrants, clamping and the final snap to zero.
    #[test]
    fn input_quantize_stick_direction_matches() {
        check(
            ee::s00::f_001158b8__generated,
            input_quantize_stick_direction,
            200,
            &quantize_stick_fixture,
        );
    }

    /// Stream table at 0x50a0a0 (count byte at 0x50a92e), one stream with
    /// a free list of 0..2 nodes; the SND lock/unlock hooks (0x44c46c,
    /// 0x44c470) are fakes; the stream file handle is null or has a bad
    /// magic, so the real `StreamFile_Queue*` take their short path (0).
    fn sndstrm_queue_fixture(c: &mut Cpu, seed: u32) {
        use crate::difftest::fakes::{machine, FAKE};
        let mut r = machine(c, seed, 0x3b76_e800, 2);
        let streams = 0x0050_a0a0;
        let stream = DATA;
        c.bus.write8(0x0050_a92e, 4);
        let index = match seed % 9 {
            0 => 5,
            1 => 2,
            _ => 1,
        };
        for k in 0..4 {
            c.bus
                .write32(streams + 4 * k, if k == 1 { stream } else { 0 });
        }
        c.bus.write32(0x0044_c46c, FAKE);
        c.bus.write32(0x0044_c470, FAKE + 0x10);
        // File handle: null, or an object whose header lacks 'MRTS'.
        let handle = if seed % 2 == 0 { 0 } else { DATA + 0x800 };
        c.bus.write32(DATA + 0x800, DATA + 0x900);
        c.bus.write32(DATA + 0x900, r.next_u32());
        c.bus.write32(stream + SNDSTRM_QUEUE_HANDLE, handle);
        let id = match seed % 4 {
            0 => 0x7fff_ff80u32,
            _ => r.next_u32() & 0x7fff_ff00,
        };
        c.bus.write32(stream + SNDSTRM_QUEUE_NEXT_ID, id);
        // Free list: 0, 1 or 2 nodes.
        let nodes = if seed % 7 == 3 { 0 } else { 1 + seed % 2 };
        let n0 = DATA + 0x1000;
        let n1 = DATA + 0x1100;
        crate::difftest::fill(c, &mut r, n0, 0x80);
        c.bus
            .write32(stream + SNDSTRM_QUEUE_FREE, if nodes > 0 { n0 } else { 0 });
        c.bus.write32(
            stream + SNDSTRM_QUEUE_FREE + 4,
            if nodes == 2 {
                n1
            } else if nodes == 1 {
                n0
            } else {
                0
            },
        );
        c.bus.write32(stream + SNDSTRM_QUEUE_FREE_COUNT, nodes);
        c.bus.write32(n0, if nodes == 2 { n1 } else { 0 });
        c.bus.write32(n1, 0);
        // Queued list: empty or one node.
        let q = DATA + 0x1200;
        let queued = seed % 3 == 0;
        c.bus
            .write32(stream + SNDSTRM_QUEUE_QUEUED, if queued { q } else { 0 });
        c.bus.write32(
            stream + SNDSTRM_QUEUE_QUEUED + 4,
            if queued { q } else { 0 },
        );
        c.bus
            .write32(stream + SNDSTRM_QUEUE_QUEUED + 8, queued as u32);
        c.bus.write32(q, 0);
        c.g[A0][0] = index;
        c.g[A1][0] = r.next_u32() as u64;
        c.g[A2][0] = sx(r.next_u32());
        c.g[A3][0] = match seed % 5 {
            0 => 0,
            _ => sx(r.next_u32() | 1),
        };
        c.g[T0][0] = match seed % 6 {
            0 => 0,
            1 => 1,
            _ => (2 + r.next_u32() % 4) as u64,
        };
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x003b_76e8;
    }

    /// `SNDSTRM_QueueRequest`: no stream, no free node, file queue
    /// failure (modes 0 and 1), a queued request (id wrap), fakes
    /// unwinding at the lock or unlock.
    #[test]
    fn sndstrm_queue_request_matches() {
        check(
            ee::s13::f_003b76e8__generated,
            sndstrm_queue_request,
            300,
            &sndstrm_queue_fixture,
        );
    }
}
