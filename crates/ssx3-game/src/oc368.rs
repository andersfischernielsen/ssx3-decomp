//! Staging module oc368 (wave wave216): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc368`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x003a_ae68,
    c_ps2_chunk_resolve_resolve,
    "cPS2ChunkResolve_Resolve",
)];

// ---------------------------------------------------------------------------
// cPS2ChunkResolve_Resolve (0x3aae68)

pub(crate) const CRR_ENTRY: u32 = 0x003a_ae68;
pub(crate) const CRR_END: u32 = 0x003a_b494;
/// The message switch's jump table (`.rodata`, messages 0..22).
pub(crate) const CRR_JUMP_TABLE: u32 = 0x0049_4fb0;
pub(crate) const CRR_MSG_COUNT: u64 = 0x17;
/// Stream names handed to the world's stream handler (messages 9 and 10).
pub(crate) const CRR_STRM_TEX: u32 = 0x0049_4f88;
pub(crate) const CRR_STRM_LPG: u32 = 0x0049_4f98;

/// Resolver fields: its vtable, the spatial hash set (eight hashes of 0x14
/// bytes, picked by the signs of the key), the section's base index.
pub(crate) const CRR_VTABLE: u32 = 0x8;
pub(crate) const CRR_HASHES: u32 = 0xc;
pub(crate) const CRR_BASE_INDEX: u32 = 0x10;

/// Resolver vtable slots (`{delta, fn}` pairs).
pub(crate) const CRR_VT_REGISTER_B: u32 = 0x28; // message 2
pub(crate) const CRR_VT_REGISTER_A: u32 = 0x48; // message 0
pub(crate) const CRR_VT_INSTANCE: u32 = 0x68; // message 3
pub(crate) const CRR_VT_REGISTER_C: u32 = 0x70; // message 4
pub(crate) const CRR_VT_OCCLUDER: u32 = 0x88; // message 5
pub(crate) const CRR_VT_PATCH_LIST: u32 = 0x90; // message 8
pub(crate) const CRR_VT_GENERIC: u32 = 0x98; // message 12
pub(crate) const CRR_VT_VOLUME: u32 = 0xa0; // message 7
pub(crate) const CRR_VT_VISIBLE: u32 = 0xa8; // message 6

/// Chunk fields: the spatial kind (+8), a patch list's count and head
/// (+0x20/+0x24, linked through +0x64), and the bounding boxes per chunk
/// type (min xyz then max xyz, six floats).
pub(crate) const CRR_OBJ_KIND: u32 = 0x8;
pub(crate) const CRR_OBJ_COUNT: u32 = 0x20;
pub(crate) const CRR_OBJ_LIST: u32 = 0x24;
pub(crate) const CRR_NODE_NEXT: u32 = 0x64;
pub(crate) const CRR_NODE_BOX: u32 = 0x6c;
pub(crate) const CRR_MODEL_COUNTER: u32 = 0x1a2;
pub(crate) const CRR_MODEL_BOX: u32 = 0x158;
pub(crate) const CRR_INSTANCE_BOX: u32 = 0x60;
pub(crate) const CRR_VISIBLE_BOX: u32 = 0x44;
pub(crate) const CRR_VOLUME_BOX: u32 = 0x28;
pub(crate) const CRR_OCCLUDER_BOX: u32 = 0x68;

/// Globals: `gp+0x2a90` the world view (its vtable at +0x10d8, the stream
/// handler in slot 0x170), `gp+0x16c8` the script resources (`D_004A47B8`),
/// `gp-0x84c` the locator table owner, `gp-0x848` the world.
pub(crate) const CRR_GP_WORLD_VIEW: u32 = 0x2a90;
pub(crate) const CRR_GP_SCRIPT_RES: u32 = 0x16c8;
pub(crate) const CRR_GP_LOCATORS_NEG: u32 = 0x84c;
pub(crate) const CRR_GP_WORLD_NEG: u32 = 0x848;

const CRR_SPATIAL_KEY: u32 = 0x0032_8f28; // SpatialKey_FromAABB
const CRR_SPATIAL_INSERT: u32 = 0x0032_8c20; // SpatialHash_Insert
const CRR_AUDIO_GAME_MODULE: u32 = 0x0028_b1c8; // SSXAudioSystem_GetGameModule
const CRR_WORLD_SOUND_LOADED: u32 = 0x002b_6b08; // cWorldSound_OnSectionLoaded
const CRR_CAMERA_RELOAD: u32 = 0x0016_cec8; // CameraTriggerMan_ReloadIf_Global
const CRR_PAINTER_SET: u32 = 0x002b_b348; // cWorldPainterMan_SetFileData
const CRR_SCRIPT_TABLE: u32 = 0x0023_0430; // GameModule_CreateSectionScriptTable
const CRR_LOCATORS_SET: u32 = 0x0027_8a10; // ScriptEngine_SetLocatorTable
const CRR_AUDIO_INSTANCE: u32 = 0x0028_b180; // SSXAudioSystem_GetInstance
const CRR_AUDIO_LOAD_BANK: u32 = 0x0028_6ca8; // SSXAudioSystem_LoadMemoryBank
const CRR_GET_BE: u32 = 0x0014_dc80; // cBE_getBE
const CRR_BE_INTERFACE: u32 = 0x0014_dd58; // BELibrary_GetInterface
const CRR_MISSION_STUB: u32 = 0x0015_47d8; // BEMissionInt_EmptyStub_47d8
const CRR_RACE_PATH_SET: u32 = 0x0021_05d0; // RaceProgress_SetPathData
const CRR_ADD_OCCLUDER: u32 = 0x0024_2540; // OcclusionMan_AddOccluder
const CRR_TRACK_PATHS: u32 = 0x0012_a340; // AI_SetTrackPathsBuffer
const CRR_MEM_FREE: u32 = 0x0031_7e98; // EA_MemFree
const CRR_AVALANCHE_SET: u32 = 0x002d_9fb8; // Avalanche_SetHistoryData

const CRR_SCRIPT_CACHE_SET: Callee = Callee {
    addr: 0x003a_ca10,
    f: ee::s13::cWScriptCache_SetData,
};
const CRR_APP_GAME_MODULE: Callee = Callee {
    addr: 0x003a_d188,
    f: ee::s13::SSXApp_GetGameModule,
};
const CRR_OCCLUSION_MAN: Callee = Callee {
    addr: 0x003a_d198,
    f: ee::s13::GameModule_GetOcclusionMan,
};

/// How a message handler leaves: the shared tails of the original.
enum CrrExit {
    /// 0x3ab464: `v0 = obj`, reload s0, epilogue.
    Plain,
    /// 0x3ab468: `v0` already set, reload s0, epilogue.
    Done,
    /// 0x3ab46c: s0 already reloaded, epilogue.
    Epilogue,
    /// The jump table pointed at the entry: run the function again.
    Restart,
}

/// `char* cPS2ChunkResolve::resolve(eBxStreamDataType type /*a1*/,
/// tResourceID rid /*a2*/, char* data /*a3*/)` (0x3aae68): hand a streamed
/// chunk to whoever owns its type, and return `data`.
///
/// - 1 (model): bump the model's counter (+0x1a2) by the section's base
///   index and insert it into the spatial hash (kind 1).
/// - 0, 2, 4: register through a resolver virtual (+0x48, +0x28, +0x70).
/// - 3 (instance): resolver virtual +0x68, then insert (kind 0).
/// - 5, 6, 7: resolver virtual (+0x88, +0xa8 only inserting when it returns
///   non-zero, +0xa0), set the chunk's kind to 7, 6, 8 and insert it (kind 2).
/// - 8: resolver virtual +0x90, then mark every patch in the chunk's list
///   kind 1 and insert it (kind 2).
/// - 9, 10: the world view's stream handler with "strm_tex"/"strm_lpg" and
///   the texture index (`rid >> 8`, plus the base index for 10).
/// - 11-22: occluders, the generic resolver virtual, world sounds, track
///   paths (freeing the data when rejected), painter files, script tables,
///   camera triggers, locators, mission data, sound banks, race paths and
///   avalanche history.
///
/// Spatial inserts build the box at `sp` (w = 1), get its key into `sp+0x20`
/// (`SpatialKey_FromAABB`) and pick one of the eight hashes at
/// `resolver+0xc` by the signs of the key's x, y, z.
pub fn c_ps2_chunk_resolve_resolve(c: &mut Cpu) {
    if c.pc != CRR_ENTRY {
        return ee::s13::f_003aae68__entries(c);
    }
    c.cycles += 15;
    let frame = Frame::open(c, 0xc0);
    c.g[V0][0] = (c.g[A1][0] < CRR_MSG_COUNT) as u64;
    frame.save(c, S0, 0xa0);
    frame.save(c, S2, 0x80);
    c.g[S0][0] = c.g[A2][0];
    frame.save(c, S3, 0x70);
    c.g[S2][0] = c.g[A3][0];
    frame.save(c, S1, 0x90);
    c.g[S3][0] = c.g[A0][0];
    frame.save(c, S4, 0x60);
    frame.save(c, S5, 0x50);
    frame.save(c, S6, 0x40);
    frame.save_ra(c, 0x30);
    frame.save_fpr(c, 20, 0xb0);
    let exit = if c.g[V0][0] == 0 {
        Ok(CrrExit::Plain)
    } else {
        crr_dispatch(c)
    };
    let exit = match exit {
        Ok(CrrExit::Restart) => {
            c.pc = CRR_ENTRY;
            return c_ps2_chunk_resolve_resolve(c);
        }
        Ok(e) => e,
        Err(Unwound) => return,
    };
    if let CrrExit::Plain = exit {
        c.cycles += 1;
        c.g[V0][0] = c.g[S2][0];
    }
    if !matches!(exit, CrrExit::Epilogue) {
        c.cycles += 1;
        frame.restore(c, S0, 0xa0);
    }
    c.cycles += 10;
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore(c, S3, 0x70);
    frame.restore(c, S4, 0x60);
    frame.restore(c, S5, 0x50);
    frame.restore(c, S6, 0x40);
    frame.restore_ra(c, 0x30);
    frame.restore_fpr(c, 20, 0xb0);
    frame.close(c);
}

/// The jump table dispatch (0x3aaea4) and the handlers.
fn crr_dispatch(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 7;
    c.g[V0][0] = 0x0049_0000;
    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[V0][0] = sx(CRR_JUMP_TABLE);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(CRR_JUMP_TABLE));
    c.g[A0][0] = Addr(r32(c, V1)).lw(c);
    let target = c.g[A0][0] as u32;
    match target {
        CRR_ENTRY => {
            if event_due(c) {
                deliver_event_here(c, CRR_ENTRY);
            }
            Ok(CrrExit::Restart)
        }
        0x003a_aec0 => crr_model(c),
        0x003a_af5c => crr_stream(c, false),
        0x003a_af88 => crr_stream(c, true),
        0x003a_afc8 => crr_instance(c),
        0x003a_b06c => {
            c.cycles += 5;
            crr_vmethod(c, CRR_VT_REGISTER_B);
            crr_vcall_obj(c)
        }
        0x003a_b080 => {
            c.cycles += 5;
            crr_vmethod(c, CRR_VT_REGISTER_A);
            crr_vcall_obj(c)
        }
        0x003a_b094 => crr_visible(c),
        0x003a_b0dc => crr_volume(c),
        0x003a_b120 => crr_patch_list(c),
        0x003a_b200 => crr_occluder_box(c),
        0x003a_b2bc => {
            c.cycles += 4;
            crr_vmethod(c, CRR_VT_REGISTER_C);
            crr_vcall_obj(c)
        }
        0x003a_b2dc => crr_generic(c),
        0x003a_b300 => crr_world_sound(c),
        0x003a_b320 => crr_camera_triggers(c),
        0x003a_b334 => crr_painter(c),
        0x003a_b354 => crr_script_table(c),
        0x003a_b38c => crr_locators(c),
        0x003a_b3a4 => crr_sound_bank(c),
        0x003a_b3c4 => crr_mission(c),
        0x003a_b3f4 => {
            c.cycles += 4;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, CRR_RACE_PATH_SET, 0x003a_b3fc)?;
            c.g[V0][0] = c.g[S2][0];
            Ok(CrrExit::Done)
        }
        0x003a_b404 => crr_occluder(c),
        0x003a_b420 => crr_track_paths(c),
        0x003a_b45c => {
            c.cycles += 2;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, CRR_AVALANCHE_SET, 0x003a_b464)?;
            Ok(CrrExit::Plain)
        }
        _ => {
            c.jump_out(target, CRR_ENTRY, CRR_END);
            Err(Unwound)
        }
    }
}

/// Read resolver vtable slot `slot` (`v1 = vtable`, `a0 = delta`,
/// `v0 = fn`) with `a1 = data`.
fn crr_vmethod(c: &mut Cpu, slot: u32) {
    c.g[V1][0] = Addr(r32(c, S3).wrapping_add(CRR_VTABLE)).lw(c);
    c.g[A1][0] = c.g[S2][0];
    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(slot)).lh(c);
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(slot + 4)).lw(c);
}

/// `jalr v0` with `a0 = resolver + delta`, returning to `ret`.
fn crr_vcall(c: &mut Cpu, ret: u32) -> Flow {
    let f = c.g[V0][0] as u32;
    c.g[A0][0] = sx(r32(c, S3).wrapping_add(r32(c, A0)));
    call_ptr(c, f, ret)
}

/// 0x3ab2cc: the virtual alone, return `data`.
fn crr_vcall_obj(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 4;
    crr_vcall(c, 0x003a_b2d4)?;
    c.g[V0][0] = c.g[S2][0];
    Ok(CrrExit::Done)
}

/// Load the chunk's box at `data+off` into f5, f0..f4; `a0 = sp+0x20` (the
/// key), `a1 = sp` (the box).
fn crr_load_box(c: &mut Cpu, off: u32) {
    let obj = r32(c, S2).wrapping_add(off);
    c.lwc1(5, obj);
    c.g[A0][0] = sx(sp(c).wrapping_add(0x20));
    c.lwc1(0, obj.wrapping_add(0x4));
    c.g[A1][0] = c.g[SP][0];
    c.lwc1(1, obj.wrapping_add(0x8));
    c.lwc1(2, obj.wrapping_add(0xc));
    c.lwc1(3, obj.wrapping_add(0x10));
    c.lwc1(4, obj.wrapping_add(0x14));
}

/// Store the box at `sp` as `{min, 1}, {max, 1}`, `s0 = resolver->hashes`,
/// and get its key (`SpatialKey_FromAABB(sp+0x20, sp)`), returning to `ret`.
fn crr_box_key(c: &mut Cpu, ret: u32) -> Flow {
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 6);
    c.g[S0][0] = Addr(r32(c, S3).wrapping_add(CRR_HASHES)).lw(c);
    let s = sp(c);
    c.swc1(5, s);
    c.swc1(0, s.wrapping_add(0x4));
    c.swc1(1, s.wrapping_add(0x8));
    c.swc1(2, s.wrapping_add(0x10));
    c.swc1(3, s.wrapping_add(0x14));
    c.swc1(4, s.wrapping_add(0x18));
    c.swc1(6, s.wrapping_add(0x1c));
    c.swc1(6, s.wrapping_add(0xc));
    call_ptr(c, CRR_SPATIAL_KEY, ret)
}

/// Pick the hash by the key's signs (x >= 0: +0, else +0x50; y: +0 or
/// +0x28; z: +0 or +0x14) and insert the chunk with `kind`
/// (`SpatialHash_Insert(hash, kind, data, sp+0x20)`, 0x3ab2a8).
fn crr_insert(c: &mut Cpu, kind: u64) -> Result<CrrExit, Unwound> {
    let s = sp(c);
    c.g[V0][0] = Addr(s.wrapping_add(0x24)).lw(c);
    c.g[A3][0] = u64::MAX;
    c.g[V1][0] = sx(r32(c, S0).wrapping_add(0x50));
    c.g[A1][0] = Addr(s.wrapping_add(0x28)).lw(c);
    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
    c.g[A2][0] = Addr(s.wrapping_add(0x2c)).lw(c);
    if c.g[V0][0] != 0 {
        c.g[V1][0] = c.g[S0][0];
    }
    c.g[A1][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
    c.g[A0][0] = sx(r32(c, V1).wrapping_add(0x28));
    c.g[A3][0] = ((c.g[A3][0] as i64) < (c.g[A2][0] as i64)) as u64;
    if c.g[A1][0] != 0 {
        c.g[A0][0] = c.g[V1][0];
    }
    c.g[A2][0] = c.g[S2][0];
    c.g[V0][0] = sx(r32(c, A0).wrapping_add(0x14));
    c.g[A1][0] = kind;
    c.cycles += 5;
    if c.g[A3][0] == 0 {
        c.g[A0][0] = c.g[V0][0];
    }
    c.g[A3][0] = sx(s.wrapping_add(0x20));
    call_ptr(c, CRR_SPATIAL_INSERT, 0x003a_b2b4)?;
    c.g[V0][0] = c.g[S2][0];
    Ok(CrrExit::Done)
}

/// Message 1 (0x3aaec0): a model.
fn crr_model(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 39;
    c.g[V1][0] = Addr(r32(c, S3).wrapping_add(CRR_BASE_INDEX)).lhu(c);
    let counter = Addr(r32(c, S2).wrapping_add(CRR_MODEL_COUNTER));
    c.g[V0][0] = counter.lhu(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    let bumped = c.g[V0][0];
    counter.sh(c, bumped);
    crr_load_box(c, CRR_MODEL_BOX);
    crr_box_key(c, 0x003a_af20)?;
    crr_insert(c, 1)
}

/// Messages 9 and 10 (0x3aaf5c, 0x3aaf88): the world view's stream handler
/// `(view + delta, data, name, 0, 0, index)`.
fn crr_stream(c: &mut Cpu, lightmaps: bool) -> Result<CrrExit, Unwound> {
    let view_at = r32(c, GP).wrapping_add(CRR_GP_WORLD_VIEW);
    if lightmaps {
        c.cycles += 12;
        c.g[T2][0] = Addr(view_at).lw(c);
        c.g[V0][0] = sx(r32(c, S0) >> 8);
        c.g[T1][0] = Addr(r32(c, S3).wrapping_add(CRR_BASE_INDEX)).lw(c);
        c.g[V1][0] = Addr(r32(c, T2).wrapping_add(0x10d8)).lw(c);
        c.g[A2][0] = sx(CRR_STRM_LPG);
        c.g[T1][0] = sx(r32(c, V0).wrapping_add(r32(c, T1)));
    } else {
        c.cycles += 11;
        c.g[T2][0] = Addr(view_at).lw(c);
        c.g[A2][0] = sx(CRR_STRM_TEX);
        c.g[T1][0] = sx(r32(c, S0) >> 8);
        c.g[V1][0] = Addr(r32(c, T2).wrapping_add(0x10d8)).lw(c);
    }
    c.g[A1][0] = c.g[S2][0];
    c.g[A3][0] = 0;
    c.g[T0][0] = 0;
    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x170)).lh(c);
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x174)).lw(c);
    c.cycles += 4;
    let f = c.g[V0][0] as u32;
    c.g[A0][0] = sx(r32(c, T2).wrapping_add(r32(c, A0)));
    call_ptr(c, f, 0x003a_afc0)?;
    c.g[V0][0] = c.g[S2][0];
    Ok(CrrExit::Done)
}

/// Message 3 (0x3aafc8): an instance.
fn crr_instance(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 41;
    crr_vmethod(c, CRR_VT_INSTANCE);
    crr_vcall(c, 0x003a_afe0)?;
    crr_load_box(c, CRR_INSTANCE_BOX);
    crr_box_key(c, 0x003a_b030)?;
    crr_insert(c, 0)
}

/// 0x3ab240: key and insert with kind 2.
fn crr_insert_kind2(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 26;
    crr_box_key(c, 0x003a_b270)?;
    crr_insert(c, 2)
}

/// Message 6 (0x3ab094): inserted (kind 6) only when the virtual accepts it.
fn crr_visible(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 8;
    crr_vmethod(c, CRR_VT_VISIBLE);
    crr_vcall(c, 0x003a_b0ac)?;
    let accepted = c.g[V0][0] != 0;
    c.g[V0][0] = 6;
    if !accepted {
        return Ok(CrrExit::Plain);
    }
    c.cycles += 10;
    let kind = c.g[V0][0];
    Addr(r32(c, S2).wrapping_add(CRR_OBJ_KIND)).sw(c, kind);
    crr_load_box(c, CRR_VISIBLE_BOX);
    crr_insert_kind2(c)
}

/// Message 7 (0x3ab0dc): a volume (kind 8).
fn crr_volume(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 17;
    crr_vmethod(c, CRR_VT_VOLUME);
    crr_vcall(c, 0x003a_b0f4)?;
    c.g[V1][0] = 8;
    Addr(r32(c, S2).wrapping_add(CRR_OBJ_KIND)).sw(c, 8);
    crr_load_box(c, CRR_VOLUME_BOX);
    crr_insert_kind2(c)
}

/// Message 5 (0x3ab200): an occluder box (kind 7).
fn crr_occluder_box(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 16;
    crr_vmethod(c, CRR_VT_OCCLUDER);
    crr_vcall(c, 0x003a_b218)?;
    c.g[V1][0] = 7;
    Addr(r32(c, S2).wrapping_add(CRR_OBJ_KIND)).sw(c, 7);
    crr_load_box(c, CRR_OCCLUDER_BOX);
    crr_insert_kind2(c)
}

/// Message 8 (0x3ab120): a patch list; every patch gets kind 1 and is
/// inserted (s1 = patch, s4 = index, s5 = -1, s6 = 1, f20 = 1.0).
fn crr_patch_list(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 10;
    c.g[V1][0] = Addr(r32(c, S3).wrapping_add(CRR_VTABLE)).lw(c);
    c.g[A1][0] = c.g[S2][0];
    c.g[S4][0] = 0;
    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(CRR_VT_PATCH_LIST)).lh(c);
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(CRR_VT_PATCH_LIST + 4)).lw(c);
    crr_vcall(c, 0x003a_b13c)?;
    c.g[V1][0] = Addr(r32(c, S2).wrapping_add(CRR_OBJ_COUNT)).lw(c);
    c.g[S1][0] = Addr(r32(c, S2).wrapping_add(CRR_OBJ_LIST)).lw(c);
    if (c.g[V1][0] as i64) <= 0 {
        return Ok(CrrExit::Plain);
    }
    c.cycles += 4;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 20);
    c.g[S6][0] = 1;
    c.g[S5][0] = u64::MAX;
    loop {
        c.cycles += 40;
        let node = r32(c, S1);
        let s = sp(c);
        c.lwc1(0, node.wrapping_add(CRR_NODE_BOX));
        c.g[A0][0] = sx(s.wrapping_add(0x20));
        let kind = c.g[S6][0];
        Addr(node.wrapping_add(CRR_OBJ_KIND)).sw(c, kind);
        c.g[A1][0] = c.g[SP][0];
        c.swc1(0, s);
        c.g[S4][0] = sx(r32(c, S4).wrapping_add(1));
        c.lwc1(0, node.wrapping_add(CRR_NODE_BOX + 0x4));
        c.g[S0][0] = Addr(r32(c, S3).wrapping_add(CRR_HASHES)).lw(c);
        c.swc1(0, s.wrapping_add(0x4));
        c.lwc1(1, node.wrapping_add(CRR_NODE_BOX + 0x8));
        c.swc1(20, s.wrapping_add(0xc));
        c.swc1(1, s.wrapping_add(0x8));
        c.lwc1(0, node.wrapping_add(CRR_NODE_BOX + 0xc));
        c.swc1(0, s.wrapping_add(0x10));
        c.lwc1(1, node.wrapping_add(CRR_NODE_BOX + 0x10));
        c.swc1(1, s.wrapping_add(0x14));
        c.lwc1(0, node.wrapping_add(CRR_NODE_BOX + 0x14));
        c.swc1(20, s.wrapping_add(0x1c));
        c.swc1(0, s.wrapping_add(0x18));
        call_ptr(c, CRR_SPATIAL_KEY, 0x003a_b1a8)?;
        // Pick the hash (as `crr_insert`, with s5 = -1 and t0 for z).
        let s = sp(c);
        c.g[V0][0] = Addr(s.wrapping_add(0x24)).lw(c);
        c.g[V1][0] = sx(r32(c, S0).wrapping_add(0x50));
        c.g[A1][0] = Addr(s.wrapping_add(0x28)).lw(c);
        c.g[A2][0] = c.g[S1][0];
        c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[V0][0] as i64)) as u64;
        c.g[T0][0] = Addr(s.wrapping_add(0x2c)).lw(c);
        if c.g[V0][0] != 0 {
            c.g[V1][0] = c.g[S0][0];
        }
        c.g[A1][0] = ((c.g[S5][0] as i64) < (c.g[A1][0] as i64)) as u64;
        c.g[A0][0] = sx(r32(c, V1).wrapping_add(0x28));
        c.g[T0][0] = ((c.g[S5][0] as i64) < (c.g[T0][0] as i64)) as u64;
        if c.g[A1][0] != 0 {
            c.g[A0][0] = c.g[V1][0];
        }
        c.g[A3][0] = sx(s.wrapping_add(0x20));
        c.g[V0][0] = sx(r32(c, A0).wrapping_add(0x14));
        c.g[A1][0] = 2;
        if c.g[T0][0] == 0 {
            c.g[A0][0] = c.g[V0][0];
        }
        call_ptr(c, CRR_SPATIAL_INSERT, 0x003a_b1e8)?;
        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(CRR_OBJ_COUNT)).lw(c);
        c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S1][0] = Addr(r32(c, S1).wrapping_add(CRR_NODE_NEXT)).lw(c);
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003a_b158);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[S2][0];
    Ok(CrrExit::Done)
}

/// Message 12 (0x3ab2dc): resolver virtual +0x98 `(rid, data)`.
fn crr_generic(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 9;
    c.g[V1][0] = Addr(r32(c, S3).wrapping_add(CRR_VTABLE)).lw(c);
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = c.g[S2][0];
    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(CRR_VT_GENERIC)).lh(c);
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(CRR_VT_GENERIC + 4)).lw(c);
    crr_vcall(c, 0x003a_b2f8)?;
    c.g[V0][0] = c.g[S2][0];
    Ok(CrrExit::Done)
}

/// Message 13 (0x3ab300): `cWorldSound_OnSectionLoaded(gameModule->+0x4c,
/// data, rid)`.
fn crr_world_sound(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 8;
    call_ptr(c, CRR_AUDIO_GAME_MODULE, 0x003a_b308)?;
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x4c)).lw(c);
    c.g[A2][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S2][0];
    call_ptr(c, CRR_WORLD_SOUND_LOADED, 0x003a_b318)?;
    c.g[V0][0] = c.g[S2][0];
    Ok(CrrExit::Done)
}

/// Message 17 (0x3ab320): `CameraTriggerMan_ReloadIf_Global(rid, data)`.
fn crr_camera_triggers(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 5;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S2][0];
    call_ptr(c, CRR_CAMERA_RELOAD, 0x003a_b32c)?;
    c.g[V0][0] = c.g[S2][0];
    Ok(CrrExit::Done)
}

/// Message 15 (0x3ab334): `cWorldPainterMan_SetFileData(res[2], rid & 0xff,
/// data)` when the script resources exist.
fn crr_painter(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, GP).wrapping_add(CRR_GP_SCRIPT_RES)).lw(c);
    c.g[A1][0] = c.g[S0][0] & 0xff;
    if c.g[V0][0] == 0 {
        return Ok(CrrExit::Plain);
    }
    c.cycles += 5;
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x8)).lw(c);
    c.g[A2][0] = c.g[S2][0];
    call_ptr(c, CRR_PAINTER_SET, 0x003a_b34c)?;
    c.g[V0][0] = c.g[S2][0];
    Ok(CrrExit::Done)
}

/// Message 16 (0x3ab354): `cWScriptCache_SetData(res[1], rid & 0xff, data)`,
/// then `GameModule_CreateSectionScriptTable(SSXApp_GetGameModule(), rid &
/// 0xff)` when the script resources exist.
fn crr_script_table(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, GP).wrapping_add(CRR_GP_SCRIPT_RES)).lw(c);
    c.g[S0][0] &= 0xff;
    if c.g[V0][0] == 0 {
        return Ok(CrrExit::Plain);
    }
    c.cycles += 11;
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x4)).lw(c);
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = c.g[S2][0];
    CRR_SCRIPT_CACHE_SET.call(c, 0x003a_b370)?;
    CRR_APP_GAME_MODULE.call(c, 0x003a_b378)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0];
    call_ptr(c, CRR_SCRIPT_TABLE, 0x003a_b384)?;
    c.g[V0][0] = c.g[S2][0];
    Ok(CrrExit::Done)
}

/// Message 18 (0x3ab38c): `ScriptEngine_SetLocatorTable(*(gp-0x84c),
/// rid & 0xff, data)`.
fn crr_locators(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 6;
    c.g[A0][0] = Addr(r32(c, GP).wrapping_sub(CRR_GP_LOCATORS_NEG)).lw(c);
    c.g[A1][0] = c.g[S0][0] & 0xff;
    c.g[A2][0] = c.g[S2][0];
    call_ptr(c, CRR_LOCATORS_SET, 0x003a_b39c)?;
    c.g[V0][0] = c.g[S2][0];
    Ok(CrrExit::Done)
}

/// Message 20 (0x3ab3a4): `SSXAudioSystem_LoadMemoryBank(GetInstance(),
/// rid, data)`.
fn crr_sound_bank(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 8;
    call_ptr(c, CRR_AUDIO_INSTANCE, 0x003a_b3ac)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = c.g[S2][0];
    call_ptr(c, CRR_AUDIO_LOAD_BANK, 0x003a_b3bc)?;
    c.g[V0][0] = c.g[S2][0];
    Ok(CrrExit::Done)
}

/// Message 19 (0x3ab3c4): the mission interface (BE interface 10) gets
/// `(rid & 0xff, data + 4, *data)` (an empty stub in this build).
fn crr_mission(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 12;
    call_ptr(c, CRR_GET_BE, 0x003a_b3cc)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0xa;
    call_ptr(c, CRR_BE_INTERFACE, 0x003a_b3d8)?;
    c.g[A3][0] = Addr(r32(c, S2)).lw(c);
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0] & 0xff;
    c.g[A2][0] = sx(r32(c, S2).wrapping_add(0x4));
    call_ptr(c, CRR_MISSION_STUB, 0x003a_b3ec)?;
    c.g[V0][0] = c.g[S2][0];
    Ok(CrrExit::Done)
}

/// Message 11 (0x3ab404): `OcclusionMan_AddOccluder(GetOcclusionMan(), data)`.
fn crr_occluder(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 7;
    CRR_OCCLUSION_MAN.call(c, 0x003a_b40c)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S2][0];
    call_ptr(c, CRR_ADD_OCCLUDER, 0x003a_b418)?;
    c.g[V0][0] = c.g[S2][0];
    Ok(CrrExit::Done)
}

/// Message 14 (0x3ab420): `AI_SetTrackPathsBuffer(world->+0x84->+0xc,
/// rid & 0xff, rid >> 8, data)`; a rejected buffer is freed.
fn crr_track_paths(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 9;
    c.g[V1][0] = Addr(r32(c, GP).wrapping_sub(CRR_GP_WORLD_NEG)).lw(c);
    c.g[A2][0] = sx(r32(c, S0) >> 8);
    c.g[A1][0] = c.g[S0][0] & 0xff;
    c.g[A3][0] = c.g[S2][0];
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x84)).lw(c);
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0xc)).lw(c);
    call_ptr(c, CRR_TRACK_PATHS, 0x003a_b43c)?;
    let kept = c.g[V0][0] != 0;
    c.g[V0][0] = c.g[S2][0];
    if kept {
        return Ok(CrrExit::Done);
    }
    c.cycles += 2;
    let no_data = c.g[S2][0] == 0;
    let s0_at = sp(c).wrapping_add(0xa0);
    c.lq(S0, s0_at);
    if no_data {
        return Ok(CrrExit::Epilogue);
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, CRR_MEM_FREE, 0x003a_b454)?;
    c.g[V0][0] = c.g[S2][0];
    Ok(CrrExit::Done)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, FAKE, LOG_RESULTS};
    use crate::difftest::{fill, put_words, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// The gate of this module's callee fakes (outside the fakes' pointer
    /// range 0x800000..0x840000 and their log).
    const OC368_CTRL: u32 = 0x008f_0000;
    const OC368_MAGIC: u32 = 0x0c36_8a5e;

    /// The jump table's words (`.rodata` 0x494fb0, from the ELF).
    const OC368_CRR_TABLE: [u32; 23] = [
        0x003a_b080,
        0x003a_aec0,
        0x003a_b06c,
        0x003a_afc8,
        0x003a_b2bc,
        0x003a_b200,
        0x003a_b094,
        0x003a_b0dc,
        0x003a_b120,
        0x003a_af5c,
        0x003a_af88,
        0x003a_b404,
        0x003a_b2dc,
        0x003a_b300,
        0x003a_b420,
        0x003a_b334,
        0x003a_b354,
        0x003a_b320,
        0x003a_b38c,
        0x003a_b3c4,
        0x003a_b3a4,
        0x003a_b3f4,
        0x003a_b45c,
    ];

    /// Gated logging fakes in the callees' override slots: with the gate set
    /// a call is logged (`fakes::fake`: random pointer result, forced
    /// results, unwinds); otherwise the slot's previous body runs.
    macro_rules! oc368_gated_fakes {
        ($($name:ident: $slot:expr => $gen:expr, $addr:expr;)*) => {
            $(
                mod $name {
                    use super::{OC368_CTRL, OC368_MAGIC};
                    use crate::difftest::fakes;
                    use ssx3_ee as ee;
                    use ssx3_ee_rt::cpu::reg::RA;
                    use ssx3_ee_rt::Cpu;
                    use std::sync::atomic::AtomicUsize;
                    pub static PREV: AtomicUsize = AtomicUsize::new(0);
                    pub fn fake(c: &mut Cpu) {
                        if c.bus.read32(OC368_CTRL) != OC368_MAGIC {
                            return fakes::fall_back(c, &PREV, $gen);
                        }
                        let ra = c.g[RA][0] as u32;
                        fakes::fake(c, $addr, ra);
                    }
                }
            )*
            fn oc368_install_fakes() {
                $(
                    fakes::hook(std::ptr::addr_of_mut!($slot), $name::fake, &$name::PREV);
                )*
            }
        };
    }

    oc368_gated_fakes! {
        oc368_fk_spatial_key: ee::s10::OVERRIDE_00328F28 => ee::s10::f_00328f28__generated, 0x0032_8f28;
        oc368_fk_insert_at_key: ee::s10::OVERRIDE_00328660 => ee::s10::f_00328660__generated, 0x0032_8660;
        oc368_fk_alloc: ee::s10::OVERRIDE_00317D70 => ee::s10::f_00317d70__generated, 0x0031_7d70;
        oc368_fk_free: ee::s10::OVERRIDE_00317E98 => ee::s10::f_00317e98__generated, 0x0031_7e98;
        oc368_fk_memset: ee::s14::OVERRIDE_003E6448 => ee::s14::f_003e6448__generated, 0x003e_6448;
        oc368_fk_audio_module: ee::s07::OVERRIDE_0028B1C8 => ee::s07::f_0028b1c8__generated, 0x0028_b1c8;
        oc368_fk_locators: ee::s07::OVERRIDE_00278A10 => ee::s07::f_00278a10__generated, 0x0027_8a10;
        oc368_fk_audio_instance: ee::s07::OVERRIDE_0028B180 => ee::s07::f_0028b180__generated, 0x0028_b180;
        oc368_fk_load_bank: ee::s07::OVERRIDE_00286CA8 => ee::s07::f_00286ca8__generated, 0x0028_6ca8;
        oc368_fk_path_free: ee::s07::OVERRIDE_0026AF00 => ee::s07::f_0026af00__generated, 0x0026_af00;
        oc368_fk_path_set: ee::s07::OVERRIDE_0026AD70 => ee::s07::f_0026ad70__generated, 0x0026_ad70;
        oc368_fk_world_sound: ee::s08::OVERRIDE_002B6B08 => ee::s08::f_002b6b08__generated, 0x002b_6b08;
        oc368_fk_script_cache: ee::s13::OVERRIDE_003ACA10 => ee::s13::f_003aca10__generated, 0x003a_ca10;
        oc368_fk_app_module: ee::s13::OVERRIDE_003AD188 => ee::s13::f_003ad188__generated, 0x003a_d188;
        oc368_fk_occlusion_man: ee::s13::OVERRIDE_003AD198 => ee::s13::f_003ad198__generated, 0x003a_d198;
        oc368_fk_get_be: ee::s01::OVERRIDE_0014DC80 => ee::s01::f_0014dc80__generated, 0x0014_dc80;
        oc368_fk_be_interface: ee::s01::OVERRIDE_0014DD58 => ee::s01::f_0014dd58__generated, 0x0014_dd58;
        oc368_fk_race_path: ee::s05::OVERRIDE_002105D0 => ee::s05::f_002105d0__generated, 0x0021_05d0;
        oc368_fk_find_section: ee::s05::OVERRIDE_0022E0E0 => ee::s05::f_0022e0e0__generated, 0x0022_e0e0;
        oc368_fk_add_occluder: ee::s06::OVERRIDE_00242540 => ee::s06::f_00242540__generated, 0x0024_2540;
        oc368_fk_avalanche: ee::s09::OVERRIDE_002D9FB8 => ee::s09::f_002d9fb8__generated, 0x002d_9fb8;
    }

    /// Every message (0-22) and out-of-range ones, against fakes for the
    /// override-slot callees; the slotless callees run for real on short
    /// paths: SpatialHash_Insert (empty hash, then the same key again),
    /// WPFile_SetData (data not a relocatable file), the section script
    /// table (no scripts), the camera triggers (no data), the mission stub,
    /// AI_SetTrackPathsBuffer (forced section numbers 0x11..0x15 or a
    /// pointer; both results, freeing or not).
    #[test]
    fn c_ps2_chunk_resolve_resolve_matches() {
        oc368_install_fakes();
        fakes::covered(
            ee::s13::f_003aae68__generated,
            c_ps2_chunk_resolve_resolve,
            300,
            &|c, seed| {
                const BASE: u32 = 0x00a0_0000;
                const SELF: u32 = BASE;
                const VT: u32 = BASE + 0x100;
                const HASHES: u32 = BASE + 0x400;
                const OBJ: u32 = BASE + 0x1000;
                const NODES: u32 = BASE + 0x2000;
                const VIEW: u32 = BASE + 0x3000;
                const VIEW_VT: u32 = BASE + 0x3200;
                const RES: u32 = BASE + 0x4000;
                const RES_SCRIPTS: u32 = BASE + 0x4100;
                const RES_TABLE: u32 = BASE + 0x4200;
                const PAINTER: u32 = BASE + 0x5000;
                const PAINTER_FILES: u32 = BASE + 0x5100;
                const WORLD: u32 = BASE + 0x6000;
                const WORLD_AI: u32 = BASE + 0x6100;

                let mut r = fakes::machine(c, seed, 0x3aae68, 6);
                c.pc = CRR_ENTRY;
                c.bus.write32(OC368_CTRL, OC368_MAGIC);
                put_words(c, CRR_JUMP_TABLE, &OC368_CRR_TABLE);
                let msg = seed % 25;
                let round = seed / 25;
                c.g[A1][0] = match msg {
                    23 => 23 + (r.next_u32() & 0xff) as u64,
                    24 => sx(r.next_u32() | 0x8000_0000),
                    m => m as u64,
                };

                // The resolver and its vtable.
                c.g[A0][0] = sx(SELF);
                c.bus.write32(SELF + CRR_VTABLE, VT);
                c.bus.write32(SELF + CRR_HASHES, HASHES);
                c.bus.write32(SELF + CRR_BASE_INDEX, r.next_u32());
                let slots = [0x28, 0x48, 0x68, 0x70, 0x88, 0x90, 0x98, 0xa0, 0xa8];
                for (i, off) in slots.into_iter().enumerate() {
                    let delta = (r.next_u32() as i16 as i32 & !3) as u16;
                    c.bus.write16(VT + off, delta);
                    c.bus.write32(VT + off + 4, FAKE + 0x10 * i as u32);
                }
                if round % 2 == 0 {
                    // Visibility test rejects (a real leaf returning 0).
                    c.bus.write32(VT + CRR_VT_VISIBLE + 4, 0x0014_06c0);
                }

                // The chunk: random boxes, a patch list.
                fill(c, &mut r, OBJ, 0x70);
                c.bus.write32(OBJ, 0x20);
                let count = [0i32, 1, 2, 3, -2][(round % 5) as usize];
                c.bus.write32(OBJ + CRR_OBJ_COUNT, count as u32);
                c.bus.write32(OBJ + CRR_OBJ_LIST, NODES);
                for i in 0..3 {
                    let node = NODES + 0x100 * i;
                    fill(c, &mut r, node + CRR_NODE_BOX, 6);
                    c.bus.write32(node + CRR_NODE_NEXT, node + 0x100);
                }
                let obj = match msg {
                    17 => 0,
                    14 if round % 3 == 0 => 0,
                    _ => OBJ,
                };
                c.g[A3][0] = sx(obj);
                let hi = [0u32, 1, 2, 3, 5, r.next_u32() >> 8][(round % 6) as usize];
                c.g[A2][0] = sx((hi << 8) | (r.next_u32() & 0xff));

                // The world view's stream handler.
                c.bus.write32(GAME_GP + CRR_GP_WORLD_VIEW, VIEW);
                c.bus.write32(VIEW + 0x10d8, VIEW_VT);
                c.bus
                    .write16(VIEW_VT + 0x170, (r.next_u32() & 0xfffc) as u16);
                c.bus.write32(VIEW_VT + 0x174, FAKE + 0x200);

                // Script resources (or none), painter files, the world.
                let res = if round % 4 == 1 { 0 } else { RES };
                c.bus.write32(GAME_GP + CRR_GP_SCRIPT_RES, res);
                c.bus.write32(RES + 4, RES_SCRIPTS);
                c.bus.write32(RES_SCRIPTS + 4, RES_TABLE);
                c.bus.write32(RES + 8, PAINTER);
                c.bus.write32(PAINTER + 4, PAINTER_FILES);
                c.bus.write32(GAME_GP - CRR_GP_LOCATORS_NEG, r.next_u32());
                c.bus.write32(GAME_GP - CRR_GP_WORLD_NEG, WORLD);
                c.bus.write32(WORLD + 0x84, WORLD_AI);
                c.bus.write32(WORLD_AI + 0x78, r.next_u32());
                if msg == 14 && round % 2 == 1 {
                    // AI_SetTrackPathsBuffer's section number (first call).
                    c.bus.write32(LOG_RESULTS, 0x11 + r.next_u32() % 5);
                }
            },
        );
    }
}
