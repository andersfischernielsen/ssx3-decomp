//! `src/animation/animmodel.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call;
use crate::call::call_ptr;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use crate::call::guest;
use crate::call::Unwound;
use crate::ps2f::FpuF32;
use crate::abi::r32;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call::VSlot;
use crate::call::sp;
use ssx3_ee_rt::funcs::call_indirect;
pub(crate) use crate::game::ai::aianimeventmap::BUILTIN_DELETE;
pub(crate) use crate::game::visualfx::boardwakefx::p;
pub(crate) use crate::game::visualfx::boardwakefx::gp;
pub(crate) use crate::game::visualfx::boardwakefx::ldf_2 as ldf;
pub(crate) use crate::game::visualfx::boardwakefx::lw;
pub(crate) use crate::game::visualfx::boardwakefx::lh;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_2;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw_2;
pub(crate) use crate::game::wscript::wscriptman::sw;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0030_db90, rider_model_compare_int, "RiderModel_CompareInt"),
    (0x0030_dba0, rider_model_set_color_scale, "RiderModel_SetColorScale"),
    (0x0030_e9e0, rider_model_show_part, "RiderModel_ShowPart"),
    (0x0030_ea80, rider_model_hide_part, "RiderModel_HidePart"),
    (0x0030_eb60, rider_model_hide_all_parts, "RiderModel_HideAllParts"),
    (0x0030_ebc0, rider_model_is_part_visible, "RiderModel_IsPartVisible"),
    (0x0030_ec18, rider_model_show_part_variant, "RiderModel_ShowPartVariant"),
    (0x0030_eca0, rider_model_lock_part, "RiderModel_LockPart"),
    (0x0030_f2b0, rider_model_blend_anim_pose, "RiderModel_BlendAnimPose"),
    (0x0031_0640, rider_model_render, "RiderModel_Render"),
    (0x0031_0948, rider_model_render_shadow, "RiderModel_RenderShadow"),
    (0x0031_0ee0, rider_model_load_state, "RiderModel_LoadState"),
    (0x0031_1048, anim_data_base_ctor, "AnimDataBase_ctor"),
    (0x0031_1088, anim_data_base_dtor, "AnimDataBase_dtor"),
    (0x0031_10d0, anim_data_base_reset_chan_cache_table, "AnimDataBase_ResetChanCacheTable"),
    (0x0031_1290, adf_data_source_set_data, "AdfDataSource_SetData"),
    (0x0031_12c8, adf_data_source_dtor, "AdfDataSource_dtor"),
    (0x0031_1318, anim_data_base_sample_channel, "AnimDataBase_SampleChannel"),
];

// ---------------------------------------------------------------- AnimSequence

/// Direct-call targets of the animation functions (shard s10).
pub(crate) mod anim_callee {
    use super::Callee;
    use ssx3_ee::s10 as ee;


    pub const FIRE_TRIGGERS_AT_TIME: Callee =
        Callee { addr: 0x0031_3938, f: ee::AnimSequence_FireTriggersAtTime };

    pub const FIRE_END_TRIGGERS: Callee =
        Callee { addr: 0x0031_39a8, f: ee::AnimSequence_FireEndTriggers };

    pub const FIRE_TRIGGERS_IN_RANGE: Callee =
        Callee { addr: 0x0031_3868, f: ee::AnimSequence_FireTriggersInRange };

    pub const GET_TRACK_BLEND_WEIGHT: Callee =
        Callee { addr: 0x0031_3d70, f: ee::AnimSequence_GetTrackBlendWeight };

    pub const ALLOC_CHAN_CACHE: Callee =
        Callee { addr: 0x0031_0f18, f: ee::AnimDataBase_AllocChanCache };

    pub const DECODE_CHAN_CACHE: Callee =
        Callee { addr: 0x0031_30a8, f: ee::AnimDataBase_DecodeChanCache };

    pub const EVAL_CHAN_CURVES: Callee =
        Callee { addr: 0x0031_2c20, f: ee::AnimDataBase_EvalChanCurves };
}

// ---------------------------------------------------------------- AnimDataBase

/// `AnimDataBase` fields and the global channel caches.
pub(crate) mod adb {

    /// Clip table (0x14 bytes per clip: +4 first chunk, +0xa chunk count,
    /// +0xc frame count, +0xe frames per chunk).
    pub const CLIPS: u32 = 0x8;

    /// Chunk descriptors (8 bytes: +0 data offset, +4 cache bank, +5 cache
    /// slot or 0xff, +6 curve count).
    pub const CHUNKS: u32 = 0xc;

    /// Compressed channel data.
    pub const DATA: u32 = 0x14;

    /// Per cache bank: the entry pool and the LRU list head (entries of 0x24
    /// bytes: +0 owner descriptor, +4 decoded channel, +0x1c prev, +0x20
    /// next).
    pub const POOL: u32 = 0x004f_e7a0;

    pub const HEAD: u32 = 0x004f_e860;

    pub const ENTRY_SIZE: u32 = 0x24;

    pub const E_PREV: u32 = 0x1c;

    pub const E_NEXT: u32 = 0x20;
}

/// `AnimDataBase_EvalChanCurves` entry points and jump tables.
pub(crate) mod eval {

    pub const ENTRY: u32 = 0x0031_2c20;

    pub const END: u32 = 0x0031_30a4;

    /// Loop heads (resume points after an unwind): the interpolating loop
    /// and the loop for frames that fall on keys.
    pub const LERP_HEAD: u32 = 0x0031_2d28;

    pub const KEY_HEAD: u32 = 0x0031_2f18;

    /// The curve-kind jump tables (`.rodata`) and their targets (left in a0
    /// as by the original's `lw`/`jr`).
    pub const LERP_TABLE_AT: u32 = 0x0048_a530;

    pub const KEY_TABLE_AT: u32 = 0x0048_a560;

    pub const LERP_TABLE: [u32; 10] = [
        0x0031_2d4c, 0x0031_2d54, 0x0031_2d60, 0x0031_2d78, 0x0031_2d9c, 0x0031_2dcc, 0x0031_2e08,
        0x0031_2e4c, 0x0031_2e90, 0x0031_2ed8,
    ];

    pub const KEY_TABLE: [u32; 10] = [
        0x0031_2f3c, 0x0031_2f44, 0x0031_2f4c, 0x0031_2f60, 0x0031_2f80, 0x0031_2fac, 0x0031_2fec,
        0x0031_3014, 0x0031_303c, 0x0031_3068,
    ];

    /// Curve record (0x14 bytes): +0 kind, +4.. coefficients / scale, bias
    /// and key index.
    pub const CURVE_SIZE: u32 = 0x14;
}

/// The game's render-context singleton (`gp+0x2a90`, `D_004A5B80`): its
/// vtable (`+0x10d8`), the render-state stack pointer (`+0xe84`) and the
/// texture-id table (`+0xf50`).
#[allow(dead_code)]
pub(crate) mod ctx {
    pub(crate) use crate::game::visualfx::boardwakefx::ctx::VTABLE;
    pub(crate) use crate::game::visualfx::boardwakefx::ctx::TOP;
    pub(crate) use crate::game::visualfx::boardwakefx::ctx::TEX_IDS;
}

/// `RiderModel` (an `sAnimModel`): the two bone tables, the parts array and
/// the per-part LOD info.
#[allow(dead_code)]
pub(crate) mod model {
    pub(crate) use crate::game::ai::rider::model::NUM_PARTS;
    pub(crate) use crate::game::ai::rider::model::PARTS;
    pub(crate) use crate::game::ai::rider::model::NUM_BONES;
    pub(crate) use crate::game::ai::rider::model::HAS_PARTS;
    pub(crate) use crate::game::ai::rider::model::BONE_TABLE_A;
    pub(crate) use crate::game::ai::rider::model::BONE_TABLE_A_SHADOW;
    pub(crate) use crate::game::ai::rider::model::BONE_TABLE_B;
    pub(crate) use crate::game::ai::rider::model::LOD_INFO;
    pub(crate) use crate::game::ai::rider::model::SCALE_Y;
    pub(crate) use crate::game::ai::rider::model::PART_STRIDE;
    pub(crate) use crate::game::ai::rider::model::PART_LODS;
    pub(crate) use crate::game::ai::rider::model::PART_REF_INDEX;
}

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_STATE_GOTO_NEXT_2 as GAME_STATE_GOTO_NEXT;
    pub(crate) use crate::game::sound::bankmonitor::callee::PS2_MEMCARD_CREATE;
    pub(crate) use crate::game::sound::bankmonitor::callee::LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::sound::bankmonitor::callee::BUDDY_MAN_CONNECT;
    pub(crate) use crate::game::sound::bankmonitor::callee::STR_CASE_CMP;
    pub(crate) use crate::game::sound::bankmonitor::callee::SLOGIN_MC_GET_PORT_SLOT;
    pub(crate) use crate::game::sound::bankmonitor::callee::MEMCARD_REQUEST_GET_INFO_NOW;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_INT;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_VEC3;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_FLOAT_2 as PATH_DATA_READ_FLOAT;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_LIST_ADD_CHOICE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS;
    pub(crate) use crate::game::sound::bankmonitor::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SSX_AUDIO_SYSTEM_END_SCRIPT_MUSIC;
    pub(crate) use crate::game::sound::bankmonitor::callee::WORLD_PAINTER_SET_SCRIPT_SURFACE_MODE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SND_BANK_MAN_UNLOAD_MEMORY_BANK;
    pub(crate) use crate::game::sound::bankmonitor::callee::SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::sound::bankmonitor::callee::MUSIC_PLAYER_GET_SONG_TAG;
    pub(crate) use crate::game::sound::bankmonitor::callee::SPEECH_SYSTEM_STOP_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::BANK_MONITOR_TIMER_KILL;
    pub(crate) use crate::game::sound::bankmonitor::callee::SNDSTRM_SET_PREBUFFER;
    pub(crate) use crate::game::sound::bankmonitor::callee::AUDIO_RAND;
    pub(crate) use crate::game::sound::bankmonitor::callee::SONG_INSTANCE_GET_REQUEST_STATUS;
    pub(crate) use crate::game::sound::bankmonitor::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::sound::bankmonitor::callee::MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_MAN_CALL_TASK_FUNC;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_PROCESS_LIST_REMOVE;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_LIBRARY_GET_INSTANCE_3 as BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_MISSION_INT_GET_COLLECTED_COUNT;
    pub(crate) use crate::game::sound::bankmonitor::callee::ADF_DATA_SOURCE_SET_DATA;
    pub(crate) use crate::game::sound::bankmonitor::callee::EA_FILE_LOAD_DECOMPRESSED;
    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_ANIM_MAP_FIND_HASH;
    pub(crate) use crate::game::sound::bankmonitor::callee::MEMSET_2 as MEMSET;
    pub(crate) use crate::game::sound::bankmonitor::callee::CROWD_MAN3D_GROUP_CREATE_MESHES;
    pub(crate) use crate::game::sound::bankmonitor::callee::TEX_MAN_FREE_TEXTURE;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_LIST_COUNT;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_MENU_SET_NAV_BUTTONS;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_OBJECT_LOAD;
    pub(crate) use crate::game::sound::bankmonitor::callee::SECTION_ALLOCATOR_DTOR;
    pub(crate) use crate::game::sound::bankmonitor::callee::STREAM_FILE_RELEASE_CHUNK;
    pub(crate) use crate::game::sound::bankmonitor::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE;
    pub(crate) use crate::game::sound::bankmonitor::callee::RIDER_METRIX_SHOW_HUD_NUMBER;
    pub(crate) use crate::game::sound::bankmonitor::callee::RIDER_METRIX_CLEAR_PEAK_A4;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_GET_EVENT_RECORD_INDEX;
    pub(crate) use crate::game::sound::bankmonitor::callee::BOLTS_FREE_DB;
}


// ---------------------------------------------------------------------------
// Rider models.

/// `void RiderModel_SetColorScale(RiderModel *model, float r, float g)`
/// (0x30dba0): build the `(r, g, g, 1)` quad and store it at `model+0x140`.
pub fn rider_model_set_color_scale(c: &mut Cpu) {
    c.cycles += 11;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 0); // f0 = 1.0
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    let sp = c.g[SP][0] as u32;
    let this = abi::arg(c, 0);
    c.swc1(12, sp);
    c.swc1(13, sp.wrapping_add(0x8));
    c.swc1(0, sp.wrapping_add(0xc));
    c.swc1(13, sp.wrapping_add(0x4));
    c.lq(V0, sp);
    c.sq(V0, this.wrapping_add(0x140));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    abi::ret(c);
}


/// `int RiderModel_IsPartVisible(RiderModel *model, int part, int sub)`
/// (0x30ebc0): resolve `model->parts[part]` and return the visible entry for
/// the sub-part at `entry*0x58 + model->offset + sub*8` + 0x18, or 0 when the
/// part is out of range / unloaded / not visible.
pub fn rider_model_is_part_visible(c: &mut Cpu) {
    c.cycles += 5;
    let this = abi::arg(c, 0);
    c.g[V0][0] = Addr(this).off(0x14).lw(c);
    c.g[V1][0] = c.g[A1][0];
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[A1][0] = Addr(this).off(0x1c).lw(c);
    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
    if c.g[A1][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
    let visible = (c.g[V0][0] as i64) >= 0;
    c.g[V1][0] = 0x58;
    if !visible {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 7;
    c.g[A0][0] = Addr(this).off(0xc).lw(c);
    c.mult(0, V1, V0, V1);
    c.g[V0][0] = sx((c.g[A2][0] as u32) << 3);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x18)).lw(c);
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Container / animation helpers.

/// `void AdfDataSource_SetData(AdfDataSource *this, const u8 *data)`
/// (0x311290): publish the four variable-length sections of the data block.
pub fn adf_data_source_set_data(c: &mut Cpu) {
    c.cycles += 13;
    let this = abi::arg(c, 0);
    let src = c.g[A1][0] as u32;
    c.g[V1][0] = sx(src.wrapping_add(0x10));
    Addr(this).off(0x4).sw(c, c.g[A1][0]);
    Addr(this).off(0x8).sw(c, c.g[V1][0]);
    c.g[V0][0] = Addr(src).off(0x4).lw(c);
    c.g[V0][0] = sx(src.wrapping_add(c.g[V0][0] as u32));
    Addr(this).off(0xc).sw(c, c.g[V0][0]);
    c.g[V1][0] = Addr(src).off(0x8).lw(c);
    c.g[V1][0] = sx(src.wrapping_add(c.g[V1][0] as u32));
    Addr(this).off(0x10).sw(c, c.g[V1][0]);
    c.g[V0][0] = Addr(src).off(0xc).lw(c);
    c.g[A1][0] = sx(src.wrapping_add(c.g[V0][0] as u32));
    Addr(this).off(0x14).sw(c, c.g[A1][0]);
    abi::ret(c);
}

pub(crate) const ADF_DATA_SOURCE_SET_DATA: Callee = Callee { addr: 0x0031_1290, f: ee::s10::f_00311290 };

pub(crate) const MEMSET_ADDR: u32 = 0x0041_6210;

pub(crate) const ANIM_DATA_BASE_RESET: Callee = Callee { addr: 0x0031_10d0, f: ee::s10::f_003110d0 };

pub(crate) const ANIM_DATA_BASE_FREE: Callee = Callee { addr: 0x0031_1110, f: ee::s10::f_00311110 };

/// `EAFile_LoadDecompressedThunk`, cross-shard (through the function table).
pub(crate) const EA_FILE_LOAD_DECOMPRESSED_THUNK: u32 = 0x003e_22f0;

pub(crate) const EA_MEM_FREE_ADDR: u32 = 0x0031_7e98;


/// `void AnimDataBase_ResetChanCacheTable(void)` (0x3110d0): clear two
/// 48-entry tables at 0x4fe7a0 and 0x4fe860.
pub fn anim_data_base_reset_chan_cache_table(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = 0x50_0000;
    c.g[V0][0] = 0x50_0000;
    c.g[V0][0] = sx(0x50_0000u32.wrapping_sub(0x17a0));
    c.g[V1][0] = sx(0x50_0000u32.wrapping_sub(0x1860));
    c.g[A0][0] = 0x2f;
    loop {
        c.cycles += 7;
        c.sw(c.g[V1][0] as u32, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
        c.sw(c.g[V0][0] as u32, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
        let more = (c.g[A0][0] as i64) >= 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
        if !more {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0031_10e8);
        }
    }
}


// ---------------------------------------------------------------------------
// AnimDataBase.

/// `AnimDataBase *AnimDataBase_ctor(AnimDataBase *this)` (0x311048): clear the
/// 0x3fc-byte object, register it in the global at `gp + 0xd08`, and reset the
/// channel-cache table.
pub fn anim_data_base_ctor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0x3fc;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw((c.g[GP][0] as u32).wrapping_add(0xd08), c.g[S0][0]);
        call_ptr(c, MEMSET_ADDR, 0x0031_1068)?;
        c.g[A0][0] = c.g[S0][0];
        ANIM_DATA_BASE_RESET.call(c, 0x0031_1070)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void AnimDataBase_dtor(AnimDataBase *this, uint flags)` (0x311088): clear
/// the global at `gp + 0xd08`, free the channel caches and, when `flags & 1`,
/// delete `this`.
pub fn anim_data_base_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.sw((c.g[GP][0] as u32).wrapping_add(0xd08), 0);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        ANIM_DATA_BASE_FREE.call(c, 0x0031_10a8)?;
        c.g[S0][0] &= 1;
        let delete_this = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if delete_this {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            BUILTIN_DELETE.call(c, 0x0031_10bc)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// ADF data source and animation combo.

/// `AdfDataSource *AdfDataSource_ctor(AdfDataSource *this, name)` (0x311250):
/// load the file through `EAFile_LoadDecompressedThunk(name, 0)`, store it at
/// +0 and hand it to `AdfDataSource_SetData`.
pub fn adf_data_source_ctor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        c.g[A1][0] = 0;
        call_ptr(c, EA_FILE_LOAD_DECOMPRESSED_THUNK, 0x0031_126c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        ADF_DATA_SOURCE_SET_DATA.call(c, 0x0031_127c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `AdfDataSource_dtor(AdfDataSource *this, uint flags)` (0x3112c8): free the
/// loaded data (+0) and, when `flags & 1`, delete `this`.
pub fn adf_data_source_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        let no_data = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_data {
            c.cycles += 2;
            call_ptr(c, EA_MEM_FREE_ADDR, 0x0031_12f0)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            BUILTIN_DELETE.call(c, 0x0031_1304)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void AnimDataBase_SampleChannel(AnimDataBase* db, float* out,
/// AnimRef* anim, int channel, int firstCurve, int maxCurves)` (0x311318):
/// sample one channel of the anim (`anim+0`: clip index << 8, `anim+4`:
/// time in seconds) at 30 frames per second, clamped to the last frame.
/// The clip is stored in chunks; the chunk's channel is taken from the
/// decode cache (moving its entry to the most-recently-used end) or decoded
/// into the least-recently-used entry, then its curves `firstCurve..` (at
/// most `maxCurves` and the chunk's curve count) are evaluated into `out` by
/// `AnimDataBase_EvalChanCurves` at the frame within the chunk.
pub fn anim_data_base_sample_channel(c: &mut Cpu) {
    const FPS: FpuF32 = FpuF32(0x41f0_0000); // 30.0
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = 0x14;
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save(c, S5, 0x20);
        frame.save(c, S6, 0x10);
        frame.save_fpr(c, 20, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        frame.save(c, S4, 0x30);
        frame.save_ra(c, 0);
        // s2 = db, s3 = curve limit, s5 = first curve, s6 = out, s1 = chunk
        // descriptor, f20 = frame
        c.g[S2][0] = c.g[A0][0];
        c.g[S3][0] = c.g[T1][0];
        c.g[S5][0] = c.g[T0][0];
        c.g[S6][0] = c.g[A1][0];
        let db = Addr(abi::arg(c, 0));
        let anim = Addr(abi::arg(c, 2));
        let channel = abi::arg(c, 3);
        c.g[AT][0] = FPS.0 as u64;
        let anim_word = anim.lw(c) as u32;
        let clips = db.off(adb::CLIPS).lw(c) as u32;
        let time = anim.off(4).read_fpu(c);
        c.f[1] = time.0;
        let clip = Addr(clips.wrapping_add(abi::mult(c, anim_word >> 8, 0x14)));
        let mut frame_f = time * FPS;
        let last = (clip.off(0xc).lhu(c) as u32).wrapping_sub(1);
        let last_f = FpuF32::from_i32(last as i32);
        let past_end = last_f.le(frame_f);
        abi::set_fcc(c, past_end);
        // (chunk count, chunk, frames per chunk)
        let (chunks, chunk, chunk_len) = if !past_end {
            c.cycles += 8;
            let len = clip.off(0xe).lhu(c) as u32;
            let whole = frame_f.to_i32() as u32;
            let (q, _) = abi::div(c, whole, len.wrapping_sub(1));
            c.cycles += 3;
            (clip.off(0xa).lhu(c) as u32, q, len)
        } else {
            c.cycles += 6;
            let chunks = clip.off(0xa).lhu(c) as u32;
            frame_f = last_f;
            (chunks, chunks.wrapping_sub(1), clip.off(0xe).lhu(c) as u32)
        };
        c.cycles += 24;
        let row = abi::mult(c, channel, chunks);
        let first_chunk = clip.off(4).lw(c) as u32;
        let table = db.off(adb::CHUNKS).lw(c) as u32;
        let start = abi::mult(c, chunk, chunk_len.wrapping_sub(1));
        let desc = Addr(
            table
                .wrapping_add(first_chunk << 3)
                .wrapping_add(row << 3)
                .wrapping_add(chunk << 3),
        );
        c.g[S1][0] = sx(desc.0);
        let data = db.off(adb::DATA).lw(c) as u32;
        let start_f = FpuF32::from_i32(start as i32);
        c.f[0] = start_f.0;
        let src = data.wrapping_add(desc.lw(c) as u32);
        let slot = desc.off(5).lbu(c);
        let bank = desc.off(4).lbu(c) as u32;
        let curves = Addr(src).lbu(c);
        frame_f = frame_f - start_f;
        c.f[20] = frame_f.0;
        if (curves as i64) < (c.g[S3][0] as i64) {
            c.g[S3][0] = curves;
        }
        let head_slot = Addr(adb::HEAD.wrapping_add(bank << 2));
        if slot != 0xff {
            // cached: move the entry to the most-recently-used end (just
            // before the head), unless it is there already
            c.cycles += 14;
            let off = abi::mult(c, slot as u32, adb::ENTRY_SIZE);
            let pool = Addr(adb::POOL.wrapping_add(bank << 2)).lw(c) as u32;
            let head = head_slot.lw(c) as u32;
            let entry = Addr(pool.wrapping_add(off));
            if entry.0 == head {
                c.cycles += 3;
                let next = entry.off(adb::E_NEXT).lw(c);
                head_slot.sw(c, next);
                c.cycles += 1;
            } else {
                let tail = Addr(head).off(adb::E_PREV).lw(c) as u32;
                c.cycles += 2;
                if tail != entry.0 {
                    c.cycles += 16;
                    let prev = entry.off(adb::E_PREV).lw(c);
                    let next = entry.off(adb::E_NEXT).lw(c);
                    Addr(prev as u32).off(adb::E_NEXT).sw(c, next);
                    let prev = entry.off(adb::E_PREV).lw(c);
                    let next = entry.off(adb::E_NEXT).lw(c);
                    Addr(next as u32).off(adb::E_PREV).sw(c, prev);
                    let head = head_slot.lw(c);
                    let tail = Addr(head as u32).off(adb::E_PREV).lw(c);
                    entry.off(adb::E_PREV).sw(c, tail);
                    let head = head_slot.lw(c);
                    entry.off(adb::E_NEXT).sw(c, head);
                    let head = head_slot.lw(c);
                    let tail = Addr(head as u32).off(adb::E_PREV).lw(c);
                    Addr(tail as u32).off(adb::E_NEXT).sw(c, entry.0 as u64);
                    let head = head_slot.lw(c);
                    Addr(head as u32).off(adb::E_PREV).sw(c, entry.0 as u64);
                    c.cycles += 1;
                }
            }
            c.cycles += 7;
            abi::set_word(c, A0, entry.0.wrapping_add(4));
            eval_curves(c, 0x0031_14dc)?;
        } else {
            // not cached: take the least-recently-used entry (the head) and
            // decode the chunk into it
            c.cycles += 9;
            c.g[S4][0] = 0x50_0000;
            c.g[S0][0] = (bank << 2) as u64;
            let mut entry = head_slot.lw(c) as u32;
            let mut new_slot = None;
            if entry == 0 {
                c.cycles += 4;
                c.g[A0][0] = bank as u64;
                c.g[A1][0] = curves;
                anim_callee::ALLOC_CHAN_CACHE.call(c, 0x0031_1528)?;
                entry = c.g[V0][0] as u32;
            } else {
                c.cycles += 3;
                let owner = Addr(entry).lw(c) as u32;
                if owner != 0 {
                    // evict: the old owner loses its slot
                    c.cycles += 3;
                    new_slot = Some(Addr(owner).off(5).lbu(c));
                    Addr(owner).off(5).sb(c, slot);
                }
            }
            let new_slot = match new_slot {
                Some(s) => s,
                None => {
                    // (entry - pool) / 0x24 by the inverse of 9
                    c.cycles += 8;
                    let pool = Addr(adb::POOL.wrapping_add(bank << 2)).lw(c) as u32;
                    let scaled = abi::mult(c, entry.wrapping_sub(pool), 0x38e3_8e39);
                    sx(((scaled as i32) >> 2) as u32)
                }
            };
            c.cycles += 20;
            desc.off(5).sb(c, new_slot);
            let e = Addr(entry);
            e.sw(c, desc.0 as u64);
            let next = e.off(adb::E_NEXT).lw(c);
            head_slot.sw(c, next);
            c.g[S0][0] = sx(entry.wrapping_add(4));
            c.g[A0][0] = c.g[S0][0];
            c.g[A3][0] = db.off(adb::DATA).lw(c);
            let data = c.g[A3][0] as u32;
            let offset = desc.lw(c) as u32;
            c.g[A2][0] = desc.off(6).lhu(c);
            abi::set_word(c, A1, data.wrapping_add(offset));
            anim_callee::DECODE_CHAN_CACHE.call(c, 0x0031_1584)?;
            c.g[A0][0] = c.g[S0][0];
            eval_curves(c, 0x0031_159c)?;
        }
        frame.restore(c, S0, 0x70);
        c.cycles += 10;
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    });

    /// `AnimDataBase_EvalChanCurves(a0, firstCurve, limit, out, frame)`.
    fn eval_curves(c: &mut Cpu, ret: u32) -> Flow {
        c.f[12] = c.f[20];
        c.g[A1][0] = c.g[S5][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S6][0];
        anim_callee::EVAL_CHAN_CURVES.call(c, ret)
    }
}


/// `out = lhs * rhs` for two 4x4 matrices through VU0 (vf4..7 = lhs,
/// vf8..11 = rhs, result rows in vf12..15), as the model renderers issue it.
pub(crate) fn concat_matrix(c: &mut Cpu, lhs: u32, rhs: u32, out: u32) {
    c.lqc2(4, lhs);
    c.lqc2(5, p(lhs, 0x10));
    c.lqc2(6, p(lhs, 0x20));
    c.lqc2(7, p(lhs, 0x30));
    c.lqc2(8, rhs);
    c.lqc2(9, p(rhs, 0x10));
    c.lqc2(10, p(rhs, 0x20));
    c.lqc2(11, p(rhs, 0x30));
    let vu = c.vu0();
    vu.mula(0b1111, 4, Src::Bc(8, X));
    vu.madda(0b1111, 5, Src::Bc(8, Y));
    vu.madda(0b1111, 6, Src::Bc(8, Z));
    vu.madd(0b1111, 12, 7, Src::Bc(8, W));
    vu.mula(0b1111, 4, Src::Bc(9, X));
    vu.madda(0b1111, 5, Src::Bc(9, Y));
    vu.madda(0b1111, 6, Src::Bc(9, Z));
    vu.madd(0b1111, 13, 7, Src::Bc(9, W));
    vu.mula(0b1111, 4, Src::Bc(10, X));
    vu.madda(0b1111, 5, Src::Bc(10, Y));
    vu.madda(0b1111, 6, Src::Bc(10, Z));
    vu.madd(0b1111, 14, 7, Src::Bc(10, W));
    vu.mula(0b1111, 4, Src::Bc(11, X));
    vu.madda(0b1111, 5, Src::Bc(11, Y));
    vu.madda(0b1111, 6, Src::Bc(11, Z));
    vu.madd(0b1111, 15, 7, Src::Bc(11, W));
    c.sqc2(12, out);
    c.sqc2(13, p(out, 0x10));
    c.sqc2(14, p(out, 0x20));
    c.sqc2(15, p(out, 0x30));
}


// ---------------------------------------------------------------------------
// RiderModel_Render / RiderModel_RenderShadow.

/// One bone of the concatenation loop: `out = table_a[i] * table_b[i]`,
/// routed through `scratch` and then the stack exactly as the original does
/// (`sqc2` to `scratch`, copy to `stack`, copy to `out`).
pub(crate) fn bone_matrix(c: &mut Cpu, a: u32, b: u32, scratch: u32, stack: u32, out: u32) {
    concat_matrix(c, a, b, scratch);
    c.lqc2(1, scratch);
    c.lqc2(2, p(scratch, 0x10));
    c.lqc2(3, p(scratch, 0x20));
    c.lqc2(4, p(scratch, 0x30));
    c.sqc2(1, stack);
    c.sqc2(2, p(stack, 0x10));
    c.sqc2(3, p(stack, 0x20));
    c.sqc2(4, p(stack, 0x30));
    c.lqc2(1, stack);
    c.lqc2(2, p(stack, 0x10));
    c.lqc2(3, p(stack, 0x20));
    c.lqc2(4, p(stack, 0x30));
    c.sqc2(1, out);
    c.sqc2(2, p(out, 0x10));
    c.sqc2(3, p(out, 0x20));
    c.sqc2(4, p(out, 0x30));
}


/// `void RiderModel_Render(RiderModel* m, int lod, int x, int y)` (0x310640):
/// concatenate the per-bone rotation (`+0x34`) and translation (`+0x38`)
/// matrices into `D_004FC420`, then either draw every part with its own LOD
/// reference (`+0x18 != 0`: `refs[refIndex]`, blend x or y by part id,
/// `vt[111]` for the matrices) or draw the parts with the single LOD matrix
/// (`vt[111]` once, then `vt[99]` per part).
pub fn rider_model_render(c: &mut Cpu) {
    c.cycles += 17;
    let frame = Frame::open(c, 0x120);
    let once = lw(c, gp(c, 0x2858));
    c.g[V0][0] = once;
    frame.save(c, S1, 0x100);
    frame.save(c, S2, 0xf0);
    let model = r32(c, A0);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S6, 0xb0);
    frame.save(c, S7, 0xa0);
    let y = r32(c, A3);
    c.g[S6][0] = c.g[A3][0];
    frame.save(c, S0, 0x110);
    let x = r32(c, A2);
    c.g[S7][0] = c.g[A2][0];
    frame.save(c, S3, 0xe0);
    frame.save(c, S4, 0xd0);
    frame.save(c, S5, 0xc0);
    frame.save(c, FP, 0x90);
    frame.save_ra(c, 0x80);
    let t = once != 0;
    let context = lw(c, gp(c, 0x2a90));
    c.g[S2][0] = context;
    if !t {
        c.cycles += 5;
        c.g[V0][0] = sx(0x3f);
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        c.g[V0][0] = sx(1);
        Addr(gp(c, 0x2858)).sw(c, 1);
    }
    guest(c, |c| {
        'body: {
        // Bone matrices.
        'bones: {
            c.cycles += 3;
            let bones = lw(c, p(model, model::NUM_BONES));
            c.g[V0][0] = bones;
            let none = (bones as i64) <= 0;
            c.g[A2][0] = 0;
            if none {
                break 'bones;
            }
            c.cycles += 5;
            c.g[V0][0] = sx(0x50_0000);
            c.g[A3][0] = sx(frame.at(c, 0x40));
            c.g[T0][0] = sx(0x50_0000u32.wrapping_sub(0x3be0));
            c.g[T1][0] = c.g[SP][0];
            loop {
                c.cycles += 54;
                c.g[V1][0] = lw(c, p(model, model::BONE_TABLE_A));
                c.g[V0][0] = sx(r32(c, A2) << 6);
                c.g[A0][0] = lw(c, p(model, model::BONE_TABLE_B));
                c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                bone_matrix(
                    c,
                    r32(c, V1),
                    r32(c, V0),
                    r32(c, A3),
                    r32(c, T1),
                    r32(c, T0),
                );
                c.g[V0][0] = lw(c, p(model, model::NUM_BONES));
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[T0][0] = sx(r32(c, T0).wrapping_add(0x40));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_06b8);
                    }
                    continue;
                }
                break;
            }
        }
        // Parts.
        'parts: {
            c.cycles += 3;
            let has_parts = lw(c, p(model, model::HAS_PARTS));
            c.g[V0][0] = has_parts;
            let plain = has_parts == 0;
            c.g[V0][0] = sx(r32(c, A1) << 3);
            if plain {
                break 'parts;
            }
            c.cycles += 3;
            let count = lw(c, p(model, model::NUM_PARTS));
            c.g[V0][0] = count;
            let none = (count as i64) <= 0;
            c.g[S4][0] = 0;
            if none {
                c.cycles += 1;
                break 'body;
            }
            c.cycles += 4;
            c.g[S5][0] = sx(r32(c, A1) << 3);
            c.g[FP][0] = sx(0x50_0000);
            c.g[S3][0] = 0;
            loop {
                'part: {
                    c.cycles += 6;
                    c.g[V0][0] = lw(c, p(model, model::PARTS));
                    c.g[A0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
                    c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, S5)));
                    c.g[V0][0] = lw(c, p(r32(c, V1), model::PART_LODS));
                    let unused = c.g[V0][0] == 0;
                    c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, S5)));
                    if unused {
                        break 'part;
                    }
                    c.cycles += 7;
                    c.g[V1][0] = lw(c, p(r32(c, A0), model::PART_REF_INDEX));
                    c.g[A0][0] = lw(c, p(r32(c, V0), 0x1c));
                    c.g[V1][0] = sx(r32(c, V1) << 4);
                    let r = r32(c, A0).wrapping_add(r32(c, V1));
                    c.g[S0][0] = sx(r);
                    c.g[V0][0] = lw(c, p(r, 0));
                    let no_ref = c.g[V0][0] == 0;
                    c.g[A1][0] = sx(0x50_0000u32.wrapping_sub(0x3be0));
                    if no_ref {
                        break 'part;
                    }
                    c.cycles += 24;
                    // vt[111](ctx, mats, 0, ref->unk_0C, ref->unk_08, 0)
                    c.g[V0][0] = lw(c, p(context as u32, ctx::VTABLE));
                    c.g[A3][0] = lw(c, p(r, 0xc));
                    c.g[A2][0] = 0;
                    c.g[A0][0] = lh(c, p(r32(c, V0), 0x378));
                    c.g[T1][0] = 0;
                    c.g[V1][0] = lw(c, p(r32(c, V0), 0x37c));
                    c.g[T0][0] = lw(c, p(r, 8));
                    let f = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0031_0810);
                    c.g[A0][0] = sx((context as u32).wrapping_add(r32(c, A0)));
                    call_ptr(c, f, 0x0031_0810)?;
                    // vt[99](ctx, ref->part, ref->data, id == 2 ? x : y, 0, scale.y)
                    c.g[V0][0] = lw(c, p(model, model::PARTS));
                    c.g[A3][0] = c.g[S7][0];
                    c.g[A1][0] = lw(c, p(context as u32, ctx::VTABLE));
                    c.g[T0][0] = 0;
                    c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
                    c.g[A2][0] = lw(c, p(r, 4));
                    c.g[V1][0] = lw(c, p(r32(c, V0), 0));
                    c.g[A0][0] = lh(c, p(r32(c, A1), 0x318));
                    c.g[V0][0] = lw(c, p(r32(c, A1), 0x31c));
                    c.g[V1][0] = c.g[V1][0] ^ 0x2;
                    c.g[A1][0] = lw(c, p(r, 0));
                    c.g[A0][0] = sx((context as u32).wrapping_add(r32(c, A0)));
                    ldf(c, 12, p(model, model::SCALE_Y));
                    let f = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0031_084c);
                    if c.g[V1][0] != 0 {
                        c.g[A3][0] = c.g[S6][0];
                    }
                    call_ptr(c, f, 0x0031_084c)?;
                }
                c.cycles += 5;
                c.g[V0][0] = lw(c, p(model, model::NUM_PARTS));
                c.g[S4][0] = sx(r32(c, S4).wrapping_add(1));
                c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(0x58));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_07b8);
                    }
                    continue;
                }
                c.cycles += 2;
                break 'body;
            }
        }
        // No per-part references: one matrix set for the whole LOD.
        c.cycles += 17;
        c.g[V1][0] = lw(c, p(context as u32, ctx::VTABLE));
        c.g[A2][0] = sx(model.wrapping_add(r32(c, V0)));
        c.g[S4][0] = c.g[V0][0];
        c.g[V0][0] = c.g[A2][0];
        c.g[A0][0] = lh(c, p(r32(c, V1), 0x378));
        c.g[A3][0] = lw(c, p(r32(c, A2), 0x44));
        c.g[A1][0] = sx(0x50_0000);
        c.g[T0][0] = lw(c, p(r32(c, V0), 0x40));
        c.g[A0][0] = sx((context as u32).wrapping_add(r32(c, A0)));
        c.g[T2][0] = lw(c, p(r32(c, V1), 0x37c));
        c.g[A1][0] = sx(0x50_0000u32.wrapping_sub(0x3be0));
        c.g[A2][0] = 0;
        let f = c.g[T2][0] as u32;
        c.g[RA][0] = sx(0x0031_08a0);
        c.g[T1][0] = 0;
        call_ptr(c, f, 0x0031_08a0)?;
        let count = lw(c, p(model, model::NUM_PARTS));
        c.g[V0][0] = count;
        let none = (count as i64) <= 0;
        c.g[S0][0] = 0;
        if none {
            c.cycles += 1;
            break 'body;
        }
        c.cycles += 1;
        c.g[S3][0] = 0;
        loop {
            'part: {
                c.cycles += 6;
                c.g[V0][0] = lw(c, p(model, model::PARTS));
                c.g[A0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
                c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, S4)));
                c.g[V0][0] = lw(c, p(r32(c, V1), model::PART_LODS));
                let unused = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[V1][0];
                if unused {
                    break 'part;
                }
                c.cycles += 4;
                c.g[A2][0] = lw(c, p(r32(c, V0), 0x1c));
                c.g[A1][0] = lw(c, p(r32(c, A2), 0));
                let no_ref = c.g[A1][0] == 0;
                c.g[T0][0] = 0;
                if no_ref {
                    break 'part;
                }
                c.cycles += 11;
                c.g[V1][0] = lw(c, p(context as u32, ctx::VTABLE));
                c.g[A3][0] = c.g[S7][0];
                c.g[V0][0] = lw(c, p(r32(c, A0), 0));
                c.g[A0][0] = lh(c, p(r32(c, V1), 0x318));
                c.g[V0][0] = c.g[V0][0] ^ 0x2;
                c.g[T1][0] = lw(c, p(r32(c, V1), 0x31c));
                c.g[A2][0] = lw(c, p(r32(c, A2), 4));
                c.g[A0][0] = sx((context as u32).wrapping_add(r32(c, A0)));
                ldf(c, 12, p(model, model::SCALE_Y));
                let f = c.g[T1][0] as u32;
                c.g[RA][0] = sx(0x0031_0904);
                if c.g[V0][0] != 0 {
                    c.g[A3][0] = c.g[S6][0];
                }
                call_ptr(c, f, 0x0031_0904)?;
            }
            c.cycles += 5;
            c.g[V0][0] = lw(c, p(model, model::NUM_PARTS));
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S3][0] = sx(r32(c, S3).wrapping_add(0x58));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0031_08b0);
                }
                continue;
            }
            break;
        }
        c.cycles += 1;
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x110);
        frame.restore(c, S1, 0x100);
        frame.restore(c, S2, 0xf0);
        frame.restore(c, S3, 0xe0);
        frame.restore(c, S4, 0xd0);
        frame.restore(c, S5, 0xc0);
        frame.restore(c, S6, 0xb0);
        frame.restore(c, S7, 0xa0);
        frame.restore(c, FP, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}


/// `void RiderModel_RenderShadow(RiderModel* m, int lod, Vec4* a, Vec4* b)`
/// (0x310948): as [`rider_model_render`] but with the shadow bone rotation
/// table (`+0x30`), the shadow matrix array (`D_004FD420`) and the shadow
/// draw entry (slot 100, `id - 1 < 2` selects x else y).
pub fn rider_model_render_shadow(c: &mut Cpu) {
    c.cycles += 16;
    let frame = Frame::open(c, 0x120);
    let once = lw(c, gp(c, 0x285c));
    c.g[V0][0] = once;
    frame.save(c, S1, 0x100);
    frame.save(c, S2, 0xf0);
    let model = r32(c, A0);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S6, 0xb0);
    frame.save(c, S7, 0xa0);
    let y = r32(c, A3);
    c.g[S6][0] = c.g[A3][0];
    frame.save(c, S0, 0x110);
    let x = r32(c, A2);
    c.g[S7][0] = c.g[A2][0];
    frame.save(c, S3, 0xe0);
    frame.save(c, S4, 0xd0);
    frame.save(c, S5, 0xc0);
    frame.save(c, FP, 0x90);
    frame.save_ra(c, 0x80);
    let t = once != 0;
    let context = lw(c, gp(c, 0x2a90));
    c.g[S2][0] = context;
    if !t {
        c.cycles += 4;
        c.g[V0][0] = sx(0x3f);
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        c.g[V0][0] = sx(1);
        Addr(gp(c, 0x285c)).sw(c, 1);
    }
    guest(c, |c| {
        'body: {
        'bones: {
            c.cycles += 4;
            let bones = lw(c, p(model, model::NUM_BONES));
            c.g[V0][0] = bones;
            let none = (bones as i64) <= 0;
            c.g[A2][0] = 0;
            if none {
                break 'bones;
            }
            c.cycles += 4;
            c.g[V0][0] = sx(0x50_0000);
            c.g[A3][0] = sx(frame.at(c, 0x40));
            c.g[T0][0] = sx(0x50_0000u32.wrapping_sub(0x2be0));
            c.g[T1][0] = c.g[SP][0];
            loop {
                c.cycles += 54;
                c.g[V1][0] = lw(c, p(model, model::BONE_TABLE_A_SHADOW));
                c.g[V0][0] = sx(r32(c, A2) << 6);
                c.g[A0][0] = lw(c, p(model, model::BONE_TABLE_B));
                c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                bone_matrix(
                    c,
                    r32(c, V1),
                    r32(c, V0),
                    r32(c, A3),
                    r32(c, T1),
                    r32(c, T0),
                );
                c.g[V0][0] = lw(c, p(model, model::NUM_BONES));
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[T0][0] = sx(r32(c, T0).wrapping_add(0x40));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_09b8);
                    }
                    continue;
                }
                break;
            }
        }
        'parts: {
            c.cycles += 3;
            let has_parts = lw(c, p(model, model::HAS_PARTS));
            c.g[V0][0] = has_parts;
            let plain = has_parts == 0;
            c.g[V0][0] = sx(r32(c, A1) << 3);
            if plain {
                break 'parts;
            }
            c.cycles += 3;
            let count = lw(c, p(model, model::NUM_PARTS));
            c.g[V0][0] = count;
            let none = (count as i64) <= 0;
            c.g[S4][0] = 0;
            if none {
                c.cycles += 1;
                break 'body;
            }
            c.cycles += 4;
            c.g[S5][0] = sx(r32(c, A1) << 3);
            c.g[FP][0] = sx(0x50_0000);
            c.g[S3][0] = 0;
            loop {
                'part: {
                    c.cycles += 6;
                    c.g[V0][0] = lw(c, p(model, model::PARTS));
                    c.g[A0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
                    c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, S5)));
                    c.g[V0][0] = lw(c, p(r32(c, V1), model::PART_LODS));
                    let unused = c.g[V0][0] == 0;
                    c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, S5)));
                    if unused {
                        break 'part;
                    }
                    c.cycles += 7;
                    c.g[V1][0] = lw(c, p(r32(c, A0), model::PART_REF_INDEX));
                    c.g[A0][0] = lw(c, p(r32(c, V0), 0x1c));
                    c.g[V1][0] = sx(r32(c, V1) << 4);
                    let r = r32(c, A0).wrapping_add(r32(c, V1));
                    c.g[S0][0] = sx(r);
                    c.g[V0][0] = lw(c, p(r, 0));
                    let no_ref = c.g[V0][0] == 0;
                    c.g[A1][0] = sx(0x50_0000u32.wrapping_sub(0x2be0));
                    if no_ref {
                        break 'part;
                    }
                    c.cycles += 23;
                    c.g[V0][0] = lw(c, p(context as u32, ctx::VTABLE));
                    c.g[A3][0] = lw(c, p(r, 0xc));
                    c.g[A2][0] = 0;
                    c.g[A0][0] = lh(c, p(r32(c, V0), 0x378));
                    c.g[T1][0] = 0;
                    c.g[V1][0] = lw(c, p(r32(c, V0), 0x37c));
                    c.g[T0][0] = lw(c, p(r, 8));
                    let f = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0031_0b10);
                    c.g[A0][0] = sx((context as u32).wrapping_add(r32(c, A0)));
                    call_ptr(c, f, 0x0031_0b10)?;
                    // vt[100](ctx, ref->part, ref->data, m)
                    c.g[V0][0] = lw(c, p(model, model::PARTS));
                    c.g[A3][0] = c.g[S7][0];
                    c.g[A1][0] = lw(c, p(context as u32, ctx::VTABLE));
                    c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
                    c.g[A2][0] = lw(c, p(r, 4));
                    c.g[V1][0] = lw(c, p(r32(c, V0), 0));
                    c.g[A0][0] = lh(c, p(r32(c, A1), 0x320));
                    c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
                    c.g[V0][0] = lw(c, p(r32(c, A1), 0x324));
                    c.g[V1][0] = (c.g[V1][0] < 0x2u64) as u64;
                    c.g[A1][0] = lw(c, p(r, 0));
                    c.g[A0][0] = sx((context as u32).wrapping_add(r32(c, A0)));
                    let f = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0031_0b48);
                    if c.g[V1][0] == 0 {
                        c.g[A3][0] = c.g[S6][0];
                    }
                    call_ptr(c, f, 0x0031_0b48)?;
                }
                c.cycles += 5;
                c.g[V0][0] = lw(c, p(model, model::NUM_PARTS));
                c.g[S4][0] = sx(r32(c, S4).wrapping_add(1));
                c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(0x58));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_0ab8);
                    }
                    continue;
                }
                c.cycles += 2;
                break 'body;
            }
        }
        // No per-part references: one matrix set for the whole LOD.
        c.cycles += 17;
        c.g[V1][0] = lw(c, p(context as u32, ctx::VTABLE));
        c.g[A2][0] = sx(model.wrapping_add(r32(c, V0)));
        c.g[S4][0] = c.g[V0][0];
        c.g[V0][0] = c.g[A2][0];
        c.g[A0][0] = lh(c, p(r32(c, V1), 0x378));
        c.g[A3][0] = lw(c, p(r32(c, A2), 0x44));
        c.g[A1][0] = sx(0x50_0000);
        c.g[T0][0] = lw(c, p(r32(c, V0), 0x40));
        c.g[A0][0] = sx((context as u32).wrapping_add(r32(c, A0)));
        c.g[T2][0] = lw(c, p(r32(c, V1), 0x37c));
        c.g[A1][0] = sx(0x50_0000u32.wrapping_sub(0x2be0));
        c.g[A2][0] = 0;
        let f = c.g[T2][0] as u32;
        c.g[RA][0] = sx(0x0031_0b9c);
        c.g[T1][0] = 0;
        call_ptr(c, f, 0x0031_0b9c)?;
        let count = lw(c, p(model, model::NUM_PARTS));
        c.g[V0][0] = count;
        let none = (count as i64) <= 0;
        c.g[S0][0] = 0;
        if none {
            c.cycles += 1;
            break 'body;
        }
        c.cycles += 2;
        c.g[S3][0] = 0;
        loop {
            let mut no_ref_tail = false;
            'part: {
                c.cycles += 6;
                c.g[V0][0] = lw(c, p(model, model::PARTS));
                c.g[A0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
                c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, S4)));
                c.g[V0][0] = lw(c, p(r32(c, V1), model::PART_LODS));
                let unused = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[V1][0];
                if unused {
                    break 'part;
                }
                c.cycles += 4;
                c.g[V1][0] = lw(c, p(r32(c, V0), 0x1c));
                c.g[A1][0] = lw(c, p(r32(c, V1), 0));
                if c.g[A1][0] == 0 {
                    c.g[V0][0] = lw(c, p(model, model::NUM_PARTS));
                    no_ref_tail = true;
                    break 'part;
                }
                c.cycles += 11;
                c.g[A3][0] = lw(c, p(context as u32, ctx::VTABLE));
                c.g[V0][0] = lw(c, p(r32(c, A0), 0));
                c.g[A2][0] = lw(c, p(r32(c, V1), 4));
                c.g[A0][0] = lh(c, p(r32(c, A3), 0x320));
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                c.g[V1][0] = lw(c, p(r32(c, A3), 0x324));
                c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                c.g[A3][0] = c.g[S7][0];
                c.g[A0][0] = sx((context as u32).wrapping_add(r32(c, A0)));
                let f = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x0031_0c04);
                if c.g[V0][0] == 0 {
                    c.g[A3][0] = c.g[S6][0];
                }
                call_ptr(c, f, 0x0031_0c04)?;
            }
            if !no_ref_tail {
                c.cycles += 1;
                c.g[V0][0] = lw(c, p(model, model::NUM_PARTS));
            }
            c.cycles += 4;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S3][0] = sx(r32(c, S3).wrapping_add(0x58));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0031_0bb0);
                }
                continue;
            }
            break;
        }
        c.cycles += 1;
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x110);
        frame.restore(c, S1, 0x100);
        frame.restore(c, S2, 0xf0);
        frame.restore(c, S3, 0xe0);
        frame.restore(c, S4, 0xd0);
        frame.restore(c, S5, 0xc0);
        frame.restore(c, S6, 0xb0);
        frame.restore(c, S7, 0xa0);
        frame.restore(c, FP, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}


/// `void RiderModel_LockPart(RiderModel* model, int i)` (0x30eca0): OR the
/// mask of part `partIndex[i]` into the model's locked-part mask.
pub fn rider_model_lock_part(c: &mut Cpu) {
    c.cycles += 13;
    let model = abi::arg(c, 0);
    let i = r32(c, A1) << 2;
    c.g[V0][0] = Addr(model).off(0x1c).lw(c);
    c.g[V1][0] = 0x58;
    c.g[A2][0] = Addr(model).off(0xc).lw(c);
    c.g[A1][0] = sx(i.wrapping_add(r32(c, V0)));
    c.g[A3][0] = Addr(model).off(0x158).ld(c);
    c.g[V0][0] = Addr(r32(c, A1)).lw(c);
    c.mult(0, V0, V0, V1);
    c.g[A2][0] = sx(r32(c, A2).wrapping_add(r32(c, V0)));
    c.g[V1][0] = Addr(r32(c, A2)).off(0x10).ld(c);
    c.g[A3][0] = c.g[A3][0] | c.g[V1][0];
    c.sd(model.wrapping_add(0x158), c.g[A3][0]);
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// RiderModel_LoadState.

/// `void RiderModel_LoadState(RiderModel* self, cAmStream* stream)`
/// (0x310ee0): hand the bone array (`+0x2c`, `numBones << 5` bytes, count at
/// `+0x10`) to the stream's `write` slot.
pub fn rider_model_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        let stream = c.g[A1][0];
        c.g[T0][0] = stream;
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(stream as u32).lw(c);
        c.g[A2][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x10)).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x10).lh(c);
        c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x2c)).lw(c);
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 5);
        c.g[A3][0] = Addr(c.g[V0][0] as u32).off(0x14).lw(c);
        let target = c.g[A3][0] as u32;
        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, target, 0x0031_0f0c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `AdfDataSource* AdfDataSource_ctor(AdfDataSource* self, const char* src
/// /*a1*/)` (0x311250): load the compressed source and adopt the data.
pub fn adf_data_source_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        c.g[A1][0] = 0;
        callee::EA_FILE_LOAD_DECOMPRESSED.call(c, 0x0031_126c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        c.sw(r32(c, S0), c.g[V0][0]);
        callee::ADF_DATA_SOURCE_SET_DATA.call(c, 0x0031_127c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `int RiderModel_CompareInt(int** a, int** b)` (0x0030db90):
/// `*a - *b`.
pub fn rider_model_compare_int(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = Addr(abi::arg(c, 0)).lw(c);
    c.g[V0][0] = Addr(abi::arg(c, 1)).lw(c);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
    abi::ret(c);
}


/// `void RiderModel_HideAllParts(RiderModel* self)` (0x30eb60): clear the
/// `used` flag of every LOD of every part and the model's visibility mask.
pub fn rider_model_hide_all_parts(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A2][0] = c.g[A0][0];
    c.g[V0][0] = lw_2(c, p_2(c.g[A2][0], 8));
    let empty = (c.g[V0][0] as i64) <= 0;
    c.g[V1][0] = 0;
    if !empty {
        c.cycles += 2;
        c.g[T1][0] = 0x58;
        c.mult(0, 7, 3, 9);
        'outer: loop {
            c.cycles += 2;
            c.g[A1][0] = 0;
            c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            loop {
                c.cycles += 8;
                c.g[V0][0] = lw_2(c, p_2(c.g[A2][0], 0xc));
                c.g[A0][0] = sx((c.g[A1][0] as u32) << 3);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = ((c.g[A1][0] as i64) < 4i64) as u64;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                let more = c.g[V1][0] != 0;
                Addr(p_2(c.g[V0][0], 0x18)).sw(c, 0);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0030_eb80);
                    }
                    continue;
                }
                c.cycles += 5;
                c.g[V0][0] = lw_2(c, p_2(c.g[A2][0], 8));
                c.g[V1][0] = c.g[T0][0];
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.mult(0, 7, 3, 9);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0030_eb78);
                    }
                    continue 'outer;
                }
                break 'outer;
            }
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    Addr(p_2(c.g[A2][0], 0x150)).sd(c, 0);
    c.pc = ra;
}

pub fn rider_model_blend_anim_pose(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x4e00);
        'b_30f690: {
            c.cycles += 42;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xd08));
            frame.save(c, 21, 0x4d90);
            c.sw(frame.at(c, 0x4c84), c.g[A2][0]);
            c.g[S5][0] = c.g[A0][0];
            c.sw(frame.at(c, 0x4c80), c.g[A1][0]);
            c.g[A0][0] = sx(frame.at(c, 0x620));
            frame.save(c, 23, 0x4d70);
            c.g[A1][0] = 0u64;
            c.sw(frame.at(c, 0x4c8c), c.g[V0][0]);
            c.g[A2][0] = 0x100;
            frame.save(c, 16, 0x4de0);
            c.g[S7][0] = 0u64;
            frame.save(c, 17, 0x4dd0);
            frame.save(c, 18, 0x4dc0);
            frame.save(c, 19, 0x4db0);
            frame.save(c, 20, 0x4da0);
            frame.save(c, 22, 0x4d80);
            frame.save(c, 30, 0x4d60);
            frame.save_ra(c, 0x4d50);
            frame.save_fpr(c, 20, 0x4df0);
            c.g[RA][0] = sx(0x0030f30c);
            c.sw(frame.at(c, 0x4c88), c.g[A3][0]);
            call_ptr(c, 0x003e6448, 0x0030f30c)?;
            c.g[V1][0] = 0x1;
            c.g[A0][0] = sx(frame.at(c, 0x720));
            c.sw(frame.at(c, 0x4c90), c.g[V1][0]);
            c.g[A1][0] = 0u64;
            c.g[RA][0] = sx(0x0030f324);
            c.g[A2][0] = 0x100;
            call_ptr(c, 0x003e6448, 0x0030f324)?;
            c.g[A0][0] = sx(frame.at(c, 0x820));
            c.g[A2][0] = 0x20;
            c.g[RA][0] = sx(0x0030f334);
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x003e6448, 0x0030f334)?;
            c.g[V0][0] = 0xffffffffffffffff;
            c.sw(frame.at(c, 0x948), 0u64);
            c.sd(frame.at(c, 0x840), c.g[V0][0]);
            c.sw(frame.at(c, 0x94c), 0u64);
            c.g[A0][0] = c.lw(frame.at(c, 0x4c84));
            c.g[A2][0] = c.lw(frame.at(c, 0x4c80));
            c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
            let t = (c.g[A2][0] as i64) <= 0;
            c.sw(frame.at(c, 0x4c94), c.g[A0][0]);
            if t {
                break 'b_30f690;
            } else {
                c.cycles += 6;
                c.g[V0][0] = sx(frame.at(c, 0x848));
                c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x323c));
                c.sw(frame.at(c, 0x4ca8), c.g[V0][0]);
                c.g[T7][0] = 0x110;
                c.g[T6][0] = 0x110;
                c.g[V1][0] = c.lw(frame.at(c, 0x4c84));
                'l_30f370: loop {
                    'b_30f680: {
                        'b_30f67c: {
                            'b_30f3cc: {
                                c.cycles += 8;
                                c.g[V0][0] = 0x50;
                                c.lo[0] = c.g[V1][0];
                                c.madd(0, 11, 23, 2);
                                c.lwc1(0, (c.g[T3][0] as u32).wrapping_add(0xc));
                                c.c_lt_s(0, 20);
                                let t = !c.fcc();
                                c.g[A0][0] = c.lw(frame.at(c, 0x4c90));
                                if t {
                                    break 'b_30f3cc;
                                } else {
                                    c.cycles += 2;
                                    let t = (c.g[A0][0] as i64) <= 0;
                                    c.g[T9][0] = sx((c.g[S7][0] as u32).wrapping_add(0x1));
                                    if t {
                                        break 'b_30f67c;
                                    } else {
                                        c.cycles += 4;
                                        c.g[A2][0] = c.lw(frame.at(c, 0x4ca8));
                                        c.g[V0][0] = sx((c.g[S7][0] as u32) << 2);
                                        c.g[V1][0] = c.g[A0][0];
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[A2][0] as u32)));
                                        'l_30f3a8: loop {
                                            c.cycles += 7;
                                            c.sw((c.g[V0][0] as u32), 0u64);
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                            let t = c.g[V1][0] != 0u64;
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x110));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x0030f3a8);
                                                }
                                                continue 'l_30f3a8;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[V1][0] = c.lw(frame.at(c, 0x4c80));
                                                if t {
                                                    break 'b_30f680;
                                                } else {
                                                    break 'b_30f3cc;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            'b_30f438: {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x8));
                                c.g[V1][0] = c.lw(frame.at(c, 0x4c94));
                                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[T9][0] = sx((c.g[S7][0] as u32).wrapping_add(0x1));
                                if t {
                                    break 'b_30f438;
                                } else {
                                    'b_30f430: {
                                        c.cycles += 3;
                                        c.g[A0][0] = c.lw(frame.at(c, 0x4c90));
                                        let t = (c.g[A0][0] as i64) <= 0;
                                        c.g[V0][0] = sx(frame.at(c, 0x840));
                                        if t {
                                            break 'b_30f430;
                                        } else {
                                            c.cycles += 3;
                                            c.g[AT][0] = 0x3f800000;
                                            c.mtc1(1, 3);
                                            c.g[V1][0] = c.g[A0][0];
                                            'l_30f3f8: loop {
                                                'b_30f420: {
                                                    c.cycles += 9;
                                                    c.lwc1(
                                                        0,
                                                        (c.g[V0][0] as u32).wrapping_add(0x108),
                                                    );
                                                    c.lwc1(
                                                        2,
                                                        (c.g[V0][0] as u32).wrapping_add(0x10c),
                                                    );
                                                    c.sub_s(1, 3, 0);
                                                    c.mul_s(1, 1, 2);
                                                    c.add_s(0, 0, 1);
                                                    c.c_lt_s(3, 0);
                                                    let t = !c.fcc();
                                                    c.swc1(
                                                        0,
                                                        (c.g[V0][0] as u32).wrapping_add(0x108),
                                                    );
                                                    if t {
                                                        break 'b_30f420;
                                                    } else {
                                                        c.cycles += 1;
                                                        c.swc1(
                                                            3,
                                                            (c.g[V0][0] as u32).wrapping_add(0x108),
                                                        );
                                                        break 'b_30f420;
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.sw((c.g[V0][0] as u32).wrapping_add(0x10c), 0u64);
                                                c.g[V1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                                let t = c.g[V1][0] != 0u64;
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x110));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x0030f3f8);
                                                    }
                                                    continue 'l_30f3f8;
                                                } else {
                                                    break 'b_30f430;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    c.g[A2][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x8));
                                    c.sw(frame.at(c, 0x4c94), c.g[A2][0]);
                                    break 'b_30f438;
                                }
                            }
                            'b_30f4e4: {
                                c.cycles += 16;
                                c.g[A0][0] = c.lw((c.g[T3][0] as u32));
                                c.g[V0][0] = 0x14;
                                c.g[A2][0] = c.lw(frame.at(c, 0x4c8c));
                                c.g[T1][0] = 0u64;
                                c.g[V1][0] = c.g[A0][0] & 0xff;
                                c.g[T5][0] = c.ld((c.g[S5][0] as u32).wrapping_add(0x150));
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[A0][0] = sx((c.g[A0][0] as u32) >> 8);
                                c.g[V1][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                c.mult(0, 4, 4, 2);
                                c.g[V1][0] = c.lw((c.g[V1][0] as u32));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[A1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                                let t = c.g[A1][0] == 0u64;
                                c.g[T2][0] = c.ld((c.g[T3][0] as u32).wrapping_add(0x10));
                                if t {
                                    break 'b_30f4e4;
                                } else {
                                    c.cycles += 10;
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                                    c.g[A2][0] = c.g[A1][0];
                                    c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                    c.g[T4][0] = 0x58;
                                    c.g[A0][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0xa));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                                    c.g[T0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x14));
                                    c.g[A3][0] = sx((c.g[A0][0] as u32) << 3);
                                    c.g[A1][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    'l_30f4a0: loop {
                                        'b_30f4dc: {
                                            c.cycles += 4;
                                            c.g[A0][0] = c.lbu((c.g[A1][0] as u32));
                                            c.g[V0][0] =
                                                ((c.g[T0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[A2][0] =
                                                    sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                                                break 'b_30f4dc;
                                            } else {
                                                'b_30f4d8: {
                                                    c.cycles += 6;
                                                    c.g[V1][0] = c
                                                        .lw((c.g[S5][0] as u32).wrapping_add(0x1c));
                                                    c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add((c.g[V1][0] as u32)));
                                                    c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                                                    let t = (c.g[V0][0] as i64) < 0;
                                                    c.mult(0, 4, 2, 12);
                                                    if t {
                                                        break 'b_30f4d8;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S5][0] as u32)
                                                                .wrapping_add(0xc));
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add((c.g[A0][0] as u32)));
                                                        c.g[V1][0] = c
                                                            .ld((c.g[V0][0] as u32)
                                                                .wrapping_add(0x10));
                                                        c.g[T1][0] = c.g[T1][0] | c.g[V1][0];
                                                        break 'b_30f4d8;
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[A2][0] =
                                                    sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                                                break 'b_30f4dc;
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = c.g[A2][0] != 0u64;
                                        c.g[A1][0] =
                                            sx((c.g[A1][0] as u32)
                                                .wrapping_add((c.g[A3][0] as u32)));
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x0030f4a0);
                                            }
                                            continue 'l_30f4a0;
                                        } else {
                                            break 'b_30f4e4;
                                        }
                                    }
                                }
                            }
                            'b_30f4fc: {
                                c.cycles += 4;
                                c.g[T1][0] = c.g[T1][0] & c.g[T5][0];
                                c.g[V0][0] = c.lw(frame.at(c, 0x4c88));
                                let t = c.g[V0][0] == 0u64;
                                c.g[T1][0] = c.g[T1][0] & c.g[T2][0];
                                if t {
                                    break 'b_30f4fc;
                                } else {
                                    c.cycles += 2;
                                    c.g[V0][0] = c.ld((c.g[S5][0] as u32).wrapping_add(0x158));
                                    c.g[T1][0] = c.g[T1][0] & c.g[V0][0];
                                    break 'b_30f4fc;
                                }
                            }
                            c.cycles += 3;
                            c.g[V1][0] = c.lw(frame.at(c, 0x4c90));
                            let t = (c.g[V1][0] as i64) <= 0;
                            c.g[A0][0] = 0x110;
                            if t {
                                break 'b_30f67c;
                            } else {
                                c.cycles += 10;
                                c.g[FP][0] = sx(frame.at(c, 0x840));
                                c.mult(0, 7, 3, 4);
                                c.g[A2][0] = c.lw(frame.at(c, 0x4ca8));
                                c.g[S0][0] = c.g[FP][0];
                                c.g[T0][0] = sx(frame.at(c, 0x848));
                                c.g[T4][0] = c.g[T0][0];
                                c.g[T2][0] = 0u64;
                                c.g[FP][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add((c.g[T6][0] as u32)));
                                c.g[S1][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add((c.g[T7][0] as u32)));
                                c.g[S6][0] = c.g[V1][0];
                                'l_30f530: loop {
                                    'b_30f668: {
                                        'b_30f55c: {
                                            c.cycles += 5;
                                            c.g[V0][0] = c.ld((c.g[S0][0] as u32));
                                            c.g[V1][0] = !(0u64 | c.g[T1][0]);
                                            c.g[S2][0] = c.g[V0][0] & c.g[T1][0];
                                            let t = c.g[S2][0] != 0u64;
                                            c.g[V1][0] = c.g[V0][0] & c.g[V1][0];
                                            if t {
                                                break 'b_30f55c;
                                            } else {
                                                c.cycles += 6;
                                                c.g[V0][0] = sx((c.g[S7][0] as u32) << 2);
                                                c.sd((c.g[S0][0] as u32), c.g[V1][0]);
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add((c.g[T2][0] as u32)));
                                                c.g[V0][0] = sx((c.g[T0][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                let t = true;
                                                c.sw((c.g[V0][0] as u32), 0u64);
                                                if t {
                                                    break 'b_30f668;
                                                } else {
                                                    break 'b_30f55c;
                                                }
                                            }
                                        }
                                        'b_30f604: {
                                            c.cycles += 3;
                                            c.g[S4][0] = sx((c.g[S7][0] as u32) << 2);
                                            let t = c.g[V1][0] == 0u64;
                                            c.g[S3][0] = sx(frame.at(c, 0x848));
                                            if t {
                                                break 'b_30f604;
                                            } else {
                                                'b_30f5e4: {
                                                    c.cycles += 9;
                                                    c.sd((c.g[S1][0] as u32), c.g[V1][0]);
                                                    c.g[V0][0] = sx((c.g[S4][0] as u32)
                                                        .wrapping_add((c.g[A3][0] as u32)));
                                                    c.g[V0][0] = sx((c.g[T0][0] as u32)
                                                        .wrapping_add((c.g[V0][0] as u32)));
                                                    c.lwc1(
                                                        0,
                                                        (c.g[S0][0] as u32).wrapping_add(0x108),
                                                    );
                                                    c.swc1(
                                                        0,
                                                        (c.g[S1][0] as u32).wrapping_add(0x108),
                                                    );
                                                    c.lwc1(
                                                        1,
                                                        (c.g[S0][0] as u32).wrapping_add(0x10c),
                                                    );
                                                    c.swc1(
                                                        1,
                                                        (c.g[S1][0] as u32).wrapping_add(0x10c),
                                                    );
                                                    let t = c.g[S7][0] == 0u64;
                                                    c.sw((c.g[V0][0] as u32), 0u64);
                                                    if t {
                                                        break 'b_30f5e4;
                                                    } else {
                                                        c.cycles += 22;
                                                        c.g[A1][0] = c.g[T4][0];
                                                        frame.save(c, 7, 0x4cb0);
                                                        frame.save(c, 8, 0x4cc0);
                                                        c.g[A0][0] = c.g[FP][0];
                                                        frame.save(c, 9, 0x4cd0);
                                                        c.g[A2][0] = c.g[S4][0];
                                                        frame.save(c, 10, 0x4ce0);
                                                        frame.save(c, 11, 0x4cf0);
                                                        frame.save(c, 12, 0x4d00);
                                                        frame.save(c, 14, 0x4d20);
                                                        frame.save(c, 15, 0x4d30);
                                                        c.g[RA][0] = sx(0x0030f5c0);
                                                        frame.save(c, 25, 0x4d40);
                                                        call_ptr(c, 0x003e6574, 0x0030f5c0)?;
                                                        frame.restore(c, 25, 0x4d40);
                                                        frame.restore(c, 15, 0x4d30);
                                                        frame.restore(c, 14, 0x4d20);
                                                        frame.restore(c, 12, 0x4d00);
                                                        frame.restore(c, 11, 0x4cf0);
                                                        frame.restore(c, 10, 0x4ce0);
                                                        frame.restore(c, 9, 0x4cd0);
                                                        frame.restore(c, 8, 0x4cc0);
                                                        frame.restore(c, 7, 0x4cb0);
                                                        break 'b_30f5e4;
                                                    }
                                                }
                                                c.cycles += 8;
                                                c.g[V0][0] = c.lw(frame.at(c, 0x4c90));
                                                c.g[FP][0] =
                                                    sx((c.g[FP][0] as u32).wrapping_add(0x110));
                                                c.g[S1][0] =
                                                    sx((c.g[S1][0] as u32).wrapping_add(0x110));
                                                c.g[A3][0] =
                                                    sx((c.g[A3][0] as u32).wrapping_add(0x110));
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                c.g[T7][0] =
                                                    sx((c.g[T7][0] as u32).wrapping_add(0x110));
                                                c.sw(frame.at(c, 0x4c90), c.g[V0][0]);
                                                c.g[T6][0] =
                                                    sx((c.g[T6][0] as u32).wrapping_add(0x110));
                                                break 'b_30f604;
                                            }
                                        }
                                        'b_30f660: {
                                            'b_30f648: {
                                                c.cycles += 14;
                                                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x108));
                                                c.g[AT][0] = 0x3f800000;
                                                c.mtc1(1, 3);
                                                c.sd((c.g[S0][0] as u32), c.g[S2][0]);
                                                c.sub_s(0, 3, 0);
                                                c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x10c));
                                                c.lwc1(1, (c.g[T3][0] as u32).wrapping_add(0xc));
                                                c.add_s(2, 2, 1);
                                                c.mul_s(0, 0, 1);
                                                c.div_s(1, 0, 2);
                                                c.c_lt_s(1, 20);
                                                let t = !c.fcc();
                                                c.swc1(2, (c.g[S0][0] as u32).wrapping_add(0x10c));
                                                if t {
                                                    break 'b_30f648;
                                                } else {
                                                    c.cycles += 3;
                                                    c.mtc1(0, 1);
                                                    let t = true;
                                                    c.g[V0][0] = sx((c.g[S4][0] as u32)
                                                        .wrapping_add((c.g[T2][0] as u32)));
                                                    if t {
                                                        break 'b_30f660;
                                                    } else {
                                                        break 'b_30f648;
                                                    }
                                                }
                                            }
                                            'b_30f65c: {
                                                c.cycles += 5;
                                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3238));
                                                c.c_le_s(0, 1);
                                                if c.fcc() {
                                                    c.mov_s(1, 3);
                                                    break 'b_30f65c;
                                                } else {
                                                    break 'b_30f65c;
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[V0][0] = sx((c.g[S4][0] as u32)
                                                .wrapping_add((c.g[T2][0] as u32)));
                                            break 'b_30f660;
                                        }
                                        c.cycles += 2;
                                        c.g[V0][0] =
                                            sx((c.g[S3][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.swc1(1, (c.g[V0][0] as u32));
                                        break 'b_30f668;
                                    }
                                    c.cycles += 5;
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x110));
                                    c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x110));
                                    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x1));
                                    let t = c.g[S6][0] != 0u64;
                                    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x110));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0030f530);
                                        }
                                        continue 'l_30f530;
                                    } else {
                                        break 'b_30f67c;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V1][0] = c.lw(frame.at(c, 0x4c80));
                        break 'b_30f680;
                    }
                    c.cycles += 4;
                    c.g[S7][0] = c.g[T9][0];
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = c.lw(frame.at(c, 0x4c84));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0030f370);
                        }
                        continue 'l_30f370;
                    } else {
                        break 'b_30f690;
                    }
                }
            }
        }
        'b_30ffcc: {
            c.cycles += 3;
            c.g[A0][0] = c.lw(frame.at(c, 0x4c90));
            let t = (c.g[A0][0] as i64) <= 0;
            c.g[T9][0] = 0u64;
            if t {
                break 'b_30ffcc;
            } else {
                c.cycles += 1;
                c.g[A2][0] = sx((c.g[T9][0] as u32).wrapping_add(0x1));
                'l_30f6a0: loop {
                    'b_30ffb8: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw(frame.at(c, 0x4c80));
                        c.g[A1][0] = 0u64;
                        let t = (c.g[V0][0] as i64) <= 0;
                        c.sw(frame.at(c, 0x4ca0), c.g[A2][0]);
                        if t {
                            break 'b_30ffb8;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = 0x110;
                            c.mult(0, 3, 25, 2);
                            c.sw(frame.at(c, 0x4c98), c.g[V1][0]);
                            c.g[A0][0] = c.lw(frame.at(c, 0x4c98));
                            'l_30f6c0: loop {
                                'b_30ffa8: {
                                    c.cycles += 11;
                                    c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                                    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                    c.mtc1(0, 0);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    c.sw(frame.at(c, 0x4ca4), c.g[A2][0]);
                                    c.g[V1][0] = sx(sp(c).wrapping_add((c.g[V0][0] as u32)));
                                    c.lwc1(20, (c.g[V1][0] as u32).wrapping_add(0x848));
                                    c.c_eq_s(20, 0);
                                    if c.fcc() {
                                        c.g[A1][0] = c.lw(frame.at(c, 0x4ca4));
                                        break 'b_30ffa8;
                                    } else {
                                        'b_30ffa4: {
                                            c.cycles += 19;
                                            c.g[V1][0] = c.lw(frame.at(c, 0x4c84));
                                            c.g[V0][0] = 0x50;
                                            c.g[A0][0] = 0x14;
                                            c.g[S2][0] = 0u64;
                                            c.lo[0] = c.g[V1][0];
                                            c.madd(0, 19, 5, 2);
                                            c.g[V1][0] = c.lw((c.g[S3][0] as u32));
                                            c.g[V0][0] = c.g[V1][0] & 0xff;
                                            c.g[V1][0] = sx((c.g[V1][0] as u32) >> 8);
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                            c.mult(0, 3, 3, 4);
                                            c.g[A0][0] = c.lw(frame.at(c, 0x4c8c));
                                            c.g[V0][0] = sx((c.g[A0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                                            c.g[A1][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[A1][0] as u32)));
                                            c.g[V1][0] =
                                                c.lhu((c.g[V1][0] as u32).wrapping_add(0x8));
                                            let t = c.g[V1][0] == 0u64;
                                            c.sw(frame.at(c, 0x4c9c), c.g[V1][0]);
                                            if t {
                                                break 'b_30ffa4;
                                            } else {
                                                c.cycles += 2;
                                                c.g[A1][0] = c.lw((c.g[S3][0] as u32));
                                                'l_30f740: loop {
                                                    'b_30ff94: {
                                                        'b_30ff90: {
                                                            c.cycles += 22;
                                                            c.g[V1][0] = 0x14;
                                                            c.g[A2][0] = c.lw(frame.at(c, 0x4c8c));
                                                            c.g[V0][0] = c.g[A1][0] & 0xff;
                                                            c.g[A3][0] = c.lw((c.g[S5][0] as u32)
                                                                .wrapping_add(0x14));
                                                            c.g[A1][0] =
                                                                sx((c.g[A1][0] as u32) >> 8);
                                                            c.g[V0][0] =
                                                                sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[A2][0] as u32)
                                                                .wrapping_add((c.g[V0][0] as u32)));
                                                            c.mult(0, 5, 5, 3);
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                            c.g[A0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[A2][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0xc));
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add((c.g[A1][0] as u32)));
                                                            c.g[V1][0] = c.lhu(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0xa),
                                                            );
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.mult(0, 3, 18, 3);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add((c.g[V1][0] as u32)));
                                                            c.g[V0][0] =
                                                                sx((c.g[V0][0] as u32) << 3);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add((c.g[A2][0] as u32)));
                                                            c.g[V0][0] = c.lbu(
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.g[A3][0] = ((c.g[A3][0] as i64)
                                                                < (c.g[V0][0] as i64))
                                                                as u64;
                                                            let t = c.g[A3][0] != 0u64;
                                                            c.g[T5][0] = sx((c.g[S2][0] as u32)
                                                                .wrapping_add(0x1));
                                                            if t {
                                                                break 'b_30ff90;
                                                            } else {
                                                                c.cycles += 6;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[S5][0] as u32)
                                                                        .wrapping_add(0x1c));
                                                                c.g[V0][0] =
                                                                    sx((c.g[V0][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[V1][0] as u32),
                                                                    ));
                                                                c.g[V0][0] =
                                                                    c.lw((c.g[V0][0] as u32));
                                                                let t = (c.g[V0][0] as i64) < 0;
                                                                c.g[A0][0] = 0x58;
                                                                if t {
                                                                    break 'b_30ff90;
                                                                } else {
                                                                    c.cycles += 12;
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[S5][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    c.mult(0, 4, 2, 4);
                                                                    c.g[A2][0] =
                                                                        c.lw(frame.at(c, 0x4c98));
                                                                    c.g[V0][0] =
                                                                        sx(frame.at(c, 0x840));
                                                                    c.g[V1][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            (c.g[A2][0] as u32),
                                                                        ));
                                                                    c.g[FP][0] = c.g[V0][0];
                                                                    c.g[V1][0] =
                                                                        c.ld((c.g[V1][0] as u32));
                                                                    c.g[S6][0] = sx((c.g[A1][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            (c.g[A0][0] as u32),
                                                                        ));
                                                                    c.g[V0][0] = c
                                                                        .ld((c.g[S6][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    c.g[V0][0] =
                                                                        c.g[V0][0] & c.g[V1][0];
                                                                    if c.g[V0][0] == 0u64 {
                                                                        c.g[V1][0] =
                                                                            c.lw(
                                                                                frame.at(c, 0x4c9c)
                                                                            );
                                                                        break 'b_30ff94;
                                                                    } else {
                                                                        'b_30f984: {
                                                                            'b_30f980: {
                                                                                c.cycles += 3;
                                                                                c.g[S0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x4c));
                                                                                let t = c.g[S0][0]
                                                                                    == 0u64;
                                                                                c.g[S7][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xc));
                                                                                if t {
                                                                                    break 'b_30f980;
                                                                                } else {
                                                                                    c.cycles += 8;
                                                                                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x10));
                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S7][0] as u32)));
                                                                                    c.g[V0][0] = c
                                                                                        .g[V1][0]
                                                                                        >> (c.g
                                                                                            [V0]
                                                                                            [0]
                                                                                            & 63);
                                                                                    c.g[V0][0] = c
                                                                                        .g[V0][0]
                                                                                        & 0x1;
                                                                                    c.g[V0][0] = c
                                                                                        .g[V0][0]
                                                                                        << 32;
                                                                                    c.g[V0][0] = ((c
                                                                                        .g[V0][0]
                                                                                        as i64)
                                                                                        >> 32)
                                                                                        as u64;
                                                                                    if c.g[V0][0]
                                                                                        == 0u64
                                                                                    {
                                                                                        c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x44));
                                                                                        break 'b_30f984;
                                                                                    } else {
                                                                                        'b_30f848: {
                                                                                            'b_30f838: {
                                                                                                c.cycles += 6;
                                                                                                c.g[AT][0] = 0x3f800000;
                                                                                                c.mtc1(1, 0);
                                                                                                c.c_eq_s(20, 0);
                                                                                                let t = !c.fcc();
                                                                                                c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x8));
                                                                                                if t
                                                                                                {
                                                                                                    break 'b_30f838;
                                                                                                } else {
                                                                                                    c.cycles += 5;
                                                                                                    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x3c));
                                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                                                    c.g[S4][0] = c.g[V0][0];
                                                                                                    let t = true;
                                                                                                    c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                                    if t {
    break 'b_30f848;
    } else {
    break 'b_30f838;
    }
                                                                                                }
                                                                                            }
                                                                                            c.cycles += 4;
                                                                                            c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                                                                                            c.g[V0][0] = sx(frame.at(c, 0x20));
                                                                                            c.g[S4][0] = c.g[V1][0];
                                                                                            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                                            break 'b_30f848;
                                                                                        }
                                                                                        'b_30f914: {
                                                                                            'b_30f8d8: {
                                                                                                c.cycles += 3;
                                                                                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                                                                                let t = c.g[V0][0] == 0u64;
                                                                                                c.g[T5][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                                                                                if t
                                                                                                {
                                                                                                    break 'b_30f8d8;
                                                                                                } else {
                                                                                                    c.cycles += 17;
                                                                                                    c.g[V0][0] = c.lbu((c.g[S3][0] as u32));
                                                                                                    c.g[V1][0] = c.lw(frame.at(c, 0x4c8c));
                                                                                                    c.g[A1][0] = sx(frame.at(c, 0x220));
                                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                                                    c.g[A2][0] = c.g[S3][0];
                                                                                                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                                    c.g[A3][0] = c.g[S2][0];
                                                                                                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                                                                                                    c.g[T0][0] = 0u64;
                                                                                                    frame.save(c, 13, 0x4d10);
                                                                                                    c.g[T1][0] = c.g[S0][0];
                                                                                                    c.g[RA][0] = sx(0x0030f888);
                                                                                                    frame.save(c, 25, 0x4d40);
                                                                                                    call_ptr(c, 0x00311318, 0x0030f888)?;
                                                                                                    c.g[A1][0] = 0u64;
                                                                                                    frame.restore(c, 13, 0x4d10);
                                                                                                    let t = (c.g[S0][0] as i64) <= 0;
                                                                                                    frame.restore(c, 25, 0x4d40);
                                                                                                    if t {
    break 'b_30f914;
    } else {
    c.cycles += 2;
    c.g[A3][0] = sx(frame.at(c, 0x220));
    c.g[A2][0] = c.g[S1][0];
    'l_30f8a0: loop {
    c.cycles += 12;
    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x40));
    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[A0][0] = ((c.g[A1][0] as i64) < (c.g[S0][0] as i64)) as u64;
    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.lwc1(0, (c.g[V0][0] as u32));
    c.swc1(0, (c.g[A2][0] as u32));
    let t = c.g[A0][0] != 0u64;
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
    if t {
    if event_due(c) { deliver_event_here(c, 0x0030f8a0); } continue 'l_30f8a0;
    } else {
    c.cycles += 2;
    let t = true;

    if t {
    break 'b_30f914;
    } else {
    break 'b_30f8d8;
    }
    }
    }
    }
                                                                                                }
                                                                                            }
                                                                                            c.cycles += 15;
                                                                                            c.g[V0][0] = c.lbu((c.g[S3][0] as u32));
                                                                                            c.g[A0][0] = c.lw(frame.at(c, 0x4c8c));
                                                                                            c.g[A1][0] = c.g[S1][0];
                                                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                                            c.g[A2][0] = c.g[S3][0];
                                                                                            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                            c.g[A3][0] = c.g[S2][0];
                                                                                            c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                                                                                            c.g[T0][0] = 0u64;
                                                                                            frame.save(c, 13, 0x4d10);
                                                                                            c.g[T1][0] = c.g[S0][0];
                                                                                            c.g[RA][0] = sx(0x0030f90c);
                                                                                            frame.save(c, 25, 0x4d40);
                                                                                            call_ptr(c, 0x00311318, 0x0030f90c)?;
                                                                                            frame.restore(c, 25, 0x4d40);
                                                                                            frame.restore(c, 13, 0x4d10);
                                                                                            break 'b_30f914;
                                                                                        }
                                                                                        'b_30f940: {
                                                                                            c.cycles += 6;
                                                                                            c.g[AT][0] = 0x3f800000;
                                                                                            c.mtc1(
                                                                                                1,
                                                                                                1,
                                                                                            );
                                                                                            c.c_eq_s(20, 1);
                                                                                            let t = !c.fcc();
                                                                                            c.g[V1][0] = 0x1;
                                                                                            if t {
                                                                                                break 'b_30f940;
                                                                                            } else {
                                                                                                c.cycles += 5;
                                                                                                c.g[V0][0] = sx((c.g[S7][0] as u32) << 2);
                                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(sp(c)));
                                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x820));
                                                                                                let t = true;
                                                                                                c.sw((c.g[V0][0] as u32), c.g[V1][0]);
                                                                                                if t
                                                                                                {
                                                                                                    break 'b_30f980;
                                                                                                } else {
                                                                                                    break 'b_30f940;
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        c.cycles +=
                                                                                            4;
                                                                                        c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x3c));
                                                                                        c.sub_s(
                                                                                            2, 1,
                                                                                            20,
                                                                                        );
                                                                                        let t = (c.g
                                                                                            [S0][0]
                                                                                            as i64)
                                                                                            <= 0;
                                                                                        c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                        if t {
                                                                                            break 'b_30f980;
                                                                                        } else {
                                                                                            c.cycles += 2;
                                                                                            c.g[A1][0] = c.g[S1][0];
                                                                                            c.g[T1][0] = c.g[S0][0];
                                                                                            'l_30f958: loop {
                                                                                                c.cycles += 10;
                                                                                                c.lwc1(1, (c.g[V0][0] as u32));
                                                                                                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1));
                                                                                                c.lwc1(0, (c.g[A1][0] as u32));
                                                                                                c.mul_s(1, 2, 1);
                                                                                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                                                                                                c.mul_s(0, 20, 0);
                                                                                                c.add_s(1, 1, 0);
                                                                                                c.swc1(1, (c.g[V0][0] as u32));
                                                                                                let t = c.g[T1][0] != 0u64;
                                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                                                                                                if t
                                                                                                {
                                                                                                    if event_due(c) { deliver_event_here(c, 0x0030f958); }
                                                                                                    continue 'l_30f958;
                                                                                                } else {
                                                                                                    break 'b_30f980;
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            c.cycles += 1;
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [S6][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x44,
                                                                                ));
                                                                            break 'b_30f984;
                                                                        }
                                                                        'b_30f9c4: {
                                                                            c.cycles += 2;
                                                                            let t = (c.g[V0][0]
                                                                                as i64)
                                                                                <= 0;
                                                                            c.g[A3][0] = c.g[S2][0];
                                                                            if t {
                                                                                break 'b_30f9c4;
                                                                            } else {
                                                                                c.cycles += 14;
                                                                                c.g[V0][0] = c.lbu(
                                                                                    (c.g[S3][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[V1][0] =
                                                                                    c.lw(frame.at(
                                                                                        c, 0x4c8c,
                                                                                    ));
                                                                                c.g[A1][0] =
                                                                                    sx(frame.at(
                                                                                        c, 0x420,
                                                                                    ));
                                                                                c.g[V0][0] =
                                                                                    sx((c.g[V0][0]
                                                                                        as u32)
                                                                                        << 2);
                                                                                c.g[A2][0] =
                                                                                    c.g[S3][0];
                                                                                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                c.g[T0][0] = 0u64;
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[V0][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[T1][0] = 0x80;
                                                                                frame.save(
                                                                                    c, 13, 0x4d10,
                                                                                );
                                                                                c.g[RA][0] =
                                                                                    sx(0x0030f9bc);
                                                                                frame.save(
                                                                                    c, 25, 0x4d40,
                                                                                );
                                                                                call_ptr(
                                                                                    c, 0x00311318,
                                                                                    0x0030f9bc,
                                                                                )?;
                                                                                frame.restore(
                                                                                    c, 25, 0x4d40,
                                                                                );
                                                                                frame.restore(
                                                                                    c, 13, 0x4d10,
                                                                                );
                                                                                break 'b_30f9c4;
                                                                            }
                                                                        }
                                                                        c.cycles += 7;
                                                                        c.g[T2][0] = c
                                                                            .lw((c.g[S6][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        c.g[V0][0] = c.lw((c.g[S6]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x44));
                                                                        c.g[T0][0] = c.g[T2][0];
                                                                        c.g[V0][0] =
                                                                            sx((c.g[T0][0] as u32)
                                                                                .wrapping_add(
                                                                                    (c.g[V0][0]
                                                                                        as u32),
                                                                                ));
                                                                        c.g[V0][0] = ((c.g[T0][0]
                                                                            as i64)
                                                                            < (c.g[V0][0] as i64))
                                                                            as u64;
                                                                        let t = c.g[V0][0] == 0u64;
                                                                        c.g[A3][0] = c.lw((c.g[S6]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x38));
                                                                        if t {
                                                                            break 'b_30ff90;
                                                                        } else {
                                                                            c.cycles += 14;
                                                                            c.g[V0][0] = 0x110;
                                                                            c.g[V1][0] =
                                                                                sx((c.g[T0][0]
                                                                                    as u32)
                                                                                    << 2);
                                                                            c.mult(0, 4, 25, 2);
                                                                            c.g[A2][0] =
                                                                                sx((c.g[V1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        sp(c),
                                                                                    ));
                                                                            c.g[V1][0] = c.g[A2][0];
                                                                            c.g[AT][0] = 0x3f800000;
                                                                            c.mtc1(1, 6);
                                                                            c.g[T4][0] =
                                                                                sx((c.g[A2][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x720,
                                                                                    ));
                                                                            c.g[T1][0] =
                                                                                sx(frame
                                                                                    .at(c, 0x420));
                                                                            c.g[S0][0] =
                                                                                0xffffffffffffffff;
                                                                            c.g[S1][0] = 0x500000;
                                                                            c.g[T6][0] =
                                                                                sx((c.g[A0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        (c.g[FP][0]
                                                                                            as u32),
                                                                                    ));
                                                                            c.g[T7][0] = 0x1;
                                                                            c.g[T3][0] =
                                                                                sx((c.g[V1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x620,
                                                                                    ));
                                                                            'l_30fa18: loop {
                                                                                'b_30ff74: {
                                                                                    c.cycles += 5;
                                                                                    c.g[V0][0] = c
                                                                                        .ld(
                                                                                        (c.g[T6][0]
                                                                                            as u32),
                                                                                    );
                                                                                    c.g[V0][0] = c
                                                                                        .g[V0][0]
                                                                                        >> (c.g
                                                                                            [T0]
                                                                                            [0]
                                                                                            & 63);
                                                                                    c.g[V0][0] = c
                                                                                        .g[V0][0]
                                                                                        & 0x1;
                                                                                    if c.g[V0][0]
                                                                                        == 0u64
                                                                                    {
                                                                                        c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x44));
                                                                                        break 'b_30ff74;
                                                                                    } else {
                                                                                        'b_30fa5c: {
                                                                                            'b_30fa50: {
                                                                                                c.cycles += 3;
                                                                                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                                                                                let t = c.g[V0][0] == 0u64;
                                                                                                c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x20));
                                                                                                if t
                                                                                                {
                                                                                                    break 'b_30fa50;
                                                                                                } else {
                                                                                                    c.cycles += 6;
                                                                                                    c.g[V0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x16));
                                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T2][0] as u32)));
                                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                                                    c.g[A0][0] = c.g[V1][0];
                                                                                                    let t = true;
                                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                                                    if t {
    break 'b_30fa5c;
    } else {
    break 'b_30fa50;
    }
                                                                                                }
                                                                                            }
                                                                                            c.cycles += 3;
                                                                                            c.g[V0][0] = sx((c.g[T0][0] as u32) << 2);
                                                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                                            c.g[A0][0] = c.g[V1][0];
                                                                                            break 'b_30fa5c;
                                                                                        }
                                                                                        'b_30fc28: {
                                                                                            c.cycles += 8;
                                                                                            c.g[A1][0] = c.lw((c.g[V0][0] as u32));
                                                                                            c.g[V0][0] = sx((c.g[T2][0] as u32) << 2);
                                                                                            c.g[V1][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x14));
                                                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                                            c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                                                                                            c.g[V1][0] = c.g[V1][0] & 0x1;
                                                                                            let t = c.g[V1][0] == 0u64;
                                                                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
                                                                                            if t {
                                                                                                break 'b_30fc28;
                                                                                            } else {
                                                                                                'b_30fa9c: {
                                                                                                    'b_30fa98: {
                                                                                                        c.cycles += 4;
                                                                                                        c.c_eq_s(20, 6);
                                                                                                        let t = !c.fcc();
                                                                                                        c.g[V0][0] = sx((c.g[T0][0] as u32) << 4);
                                                                                                        if t {
    break 'b_30fa98;
    } else {
    c.cycles += 3;
    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x24));
    let t = true;
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    if t {
    break 'b_30fa9c;
    } else {
    break 'b_30fa98;
    }
    }
                                                                                                    }
                                                                                                    c.cycles += 1;
                                                                                                    c.g[A0][0] = c.g[SP][0];
                                                                                                    break 'b_30fa9c;
                                                                                                }
                                                                                                'b_30fb18: {
                                                                                                    c.cycles += 18;
                                                                                                    c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                                                                                                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                                                                                    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                                                    c.lwc1(5, (c.g[V0][0] as u32));
                                                                                                    c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                                                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2));
                                                                                                    c.swc1(5, (c.g[A0][0] as u32));
                                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                                                    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                                    c.lwc1(3, (c.g[V1][0] as u32));
                                                                                                    c.swc1(3, (c.g[A0][0] as u32).wrapping_add(0x4));
                                                                                                    c.lwc1(4, (c.g[V0][0] as u32));
                                                                                                    c.swc1(6, (c.g[A0][0] as u32).wrapping_add(0xc));
                                                                                                    c.swc1(4, (c.g[A0][0] as u32).wrapping_add(0x8));
                                                                                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                                                                                    let t = c.g[V0][0] == 0u64;
                                                                                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3));
                                                                                                    if t {
    break 'b_30fb18;
    } else {
    c.cycles += 13;
    c.lwc1(0, (c.g[A3][0] as u32).wrapping_add(0x40));
    c.lwc1(1, (c.g[A3][0] as u32).wrapping_add(0x44));
    c.lwc1(2, (c.g[A3][0] as u32).wrapping_add(0x48));
    c.mul_s(0, 5, 0);
    c.mul_s(1, 3, 1);
    c.mul_s(2, 4, 2);
    c.lwc1(3, (c.g[A3][0] as u32).wrapping_add(0x4c));
    frame.save_fpr(c, 0, 0x4c40);
    frame.save_fpr(c, 1, 0x4c44);
    frame.save_fpr(c, 2, 0x4c48);
    frame.save_fpr(c, 3, 0x4c4c);
    frame.restore(c, 2, 0x4c40);
    c.sq(2, (c.g[A0][0] as u32));
    break 'b_30fb18;
    }
                                                                                                }
                                                                                                'b_30fbe8: {
                                                                                                    c.cycles += 3;
                                                                                                    c.g[V0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x10));
                                                                                                    let t = c.g[V0][0] != c.g[S0][0];

                                                                                                    if t {
    break 'b_30fbe8;
    } else {
    'b_30fbdc: {
    'b_30fb74: {
    c.cycles += 5;
    c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0xc));
    c.mtc1(0, 0);
    c.c_eq_s(1, 0);
    let t = !c.fcc();

    if t {
    break 'b_30fb74;
    } else {
    c.cycles += 15;
    c.lqc2(4, (c.g[S3][0] as u32).wrapping_add(0x30));
    c.lqc2(5, (c.g[A0][0] as u32));
    c.vu0().sub(0b0001, 8, 8, Src::Vec(8));
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().opmula(4, 6);
    c.vu0().opmsub(7, 6, 4);
    c.vu0().mula(0b1110, 5, Src::Bc(0, W));
    c.vu0().madda(0b1110, 6, Src::Bc(4, W));
    c.vu0().madda(0b1110, 6, Src::Bc(4, W));
    c.vu0().madda(0b1110, 7, Src::Bc(0, W));
    c.vu0().madd(0b1110, 8, 7, Src::Bc(0, W));
    c.sqc2(8, frame.at(c, 0x4c50));
    let t = true;
    frame.restore(c, 2, 0x4c50);
    if t {
    break 'b_30fbdc;
    } else {
    break 'b_30fb74;
    }
    }
    }
    c.cycles += 26;
    c.lqc2(3, (c.g[A0][0] as u32));
    c.lqc2(4, (c.g[S1][0] as u32).wrapping_sub(0xed0));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, frame.at(c, 0x4c70));
    frame.restore(c, 2, 0x4c70);
    frame.save(c, 2, 0x4c60);
    c.lqc2(4, (c.g[S3][0] as u32).wrapping_add(0x30));
    c.lqc2(5, frame.at(c, 0x4c60));
    c.vu0().sub(0b0001, 8, 8, Src::Vec(8));
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().opmula(4, 6);
    c.vu0().opmsub(7, 6, 4);
    c.vu0().mula(0b1110, 5, Src::Bc(0, W));
    c.vu0().madda(0b1110, 6, Src::Bc(4, W));
    c.vu0().madda(0b1110, 6, Src::Bc(4, W));
    c.vu0().madda(0b1110, 7, Src::Bc(0, W));
    c.vu0().madd(0b1110, 8, 7, Src::Bc(0, W));
    c.sqc2(8, frame.at(c, 0x4c70));
    frame.restore(c, 2, 0x4c70);
    frame.save(c, 2, 0x4c50);
    c.lqc2(3, frame.at(c, 0x4c50));
    c.lqc2(4, (c.g[S3][0] as u32).wrapping_add(0x20));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, frame.at(c, 0x4c70));
    frame.restore(c, 2, 0x4c70);
    break 'b_30fbdc;
    }
    c.cycles += 3;
    frame.save(c, 2, 0x4c40);
    frame.restore(c, 2, 0x4c40);
    c.sq(2, (c.g[A0][0] as u32));
    break 'b_30fbe8;
    }
                                                                                                }
                                                                                                'b_30fc00: {
                                                                                                    c.cycles += 4;
                                                                                                    c.c_eq_s(20, 6);
                                                                                                    if !c.fcc() {
    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x24));
    break 'b_30fc00;
    } else {
    c.cycles += 2;
    let t = true;
    c.sw((c.g[T4][0] as u32), c.g[T7][0]);
    if t {
    break 'b_30fc28;
    } else {
    break 'b_30fc00;
    }
    }
                                                                                                }
                                                                                                c.cycles += 10;
                                                                                                c.g[V1][0] = sx((c.g[T0][0] as u32) << 4);
                                                                                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                                c.mfc1(2, 20);
                                                                                                c.lqc2(4, (c.g[V1][0] as u32));
                                                                                                c.qmtc2(2, 3);
                                                                                                c.lqc2(5, (c.g[A0][0] as u32));
                                                                                                c.vu0().mula(0b1111, 4, Src::Bc(0, W));
                                                                                                c.vu0().msuba(0b1111, 4, Src::Bc(3, X));
                                                                                                c.vu0().madd(0b1111, 4, 5, Src::Bc(3, X));
                                                                                                c.sqc2(4, (c.g[V1][0] as u32));
                                                                                                break 'b_30fc28;
                                                                                            }
                                                                                        }
                                                                                        c.cycles +=
                                                                                            4;
                                                                                        c.g[V0][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x14));
                                                                                        c.g[V0]
                                                                                            [0] = c
                                                                                            .g[V0]
                                                                                            [0]
                                                                                            & 0x2;
                                                                                        if c.g[V0]
                                                                                            [0]
                                                                                            == 0u64
                                                                                        {
                                                                                            c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x44));
                                                                                            break 'b_30ff74;
                                                                                        } else {
                                                                                            'b_30fc58: {
                                                                                                'b_30fc54: {
                                                                                                    c.cycles += 4;
                                                                                                    c.c_eq_s(20, 6);
                                                                                                    let t = !c.fcc();
                                                                                                    c.g[V0][0] = sx((c.g[T0][0] as u32) << 4);
                                                                                                    if t {
    break 'b_30fc54;
    } else {
    c.cycles += 3;
    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x28));
    let t = true;
    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    if t {
    break 'b_30fc58;
    } else {
    break 'b_30fc54;
    }
    }
                                                                                                }
                                                                                                c.cycles += 1;
                                                                                                c.g[A2][0] = sx(frame.at(c, 0x10));
                                                                                                break 'b_30fc58;
                                                                                            }
                                                                                            'b_30fe98: {
                                                                                                c.cycles += 85;
                                                                                                c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                                                                                                c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                                c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x8));
                                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                                                                                                c.lwc1(0, (c.g[V0][0] as u32));
                                                                                                frame.save_fpr(c, 1, 0x4c44);
                                                                                                frame.save_fpr(c, 2, 0x4c48);
                                                                                                frame.save_fpr(c, 0, 0x4c40);
                                                                                                c.sw(frame.at(c, 0x4c4c), 0u64);
                                                                                                c.lqc2(3, frame.at(c, 0x4c40));
                                                                                                c.vu0().add(0b1110, 4, 0, Src::Bc(0, W));
                                                                                                c.vu0().abs(0b1110, 6, 3);
                                                                                                c.vu0().ftoi(0, 0b1111, 8, 3);
                                                                                                c.vu0().add(0b1110, 7, 3, Src::Vec(6));
                                                                                                c.qmfc2(4, 8);
                                                                                                c.vu0().ftoi(0, 0b1110, 6, 4);
                                                                                                c.qmfc2(5, 7);
                                                                                                c.pceqw(5, 5, 0);
                                                                                                c.paddw(4, 4, 5);
                                                                                                c.psraw(5, 4, 1);
                                                                                                c.qmtc2(5, 7);
                                                                                                c.qmfc2(2, 6);
                                                                                                c.paddw(2, 2, 4);
                                                                                                c.vu0().itof(0, 0b1111, 7, 7);
                                                                                                c.psllw(4, 4, 15);
                                                                                                c.psllw(4, 4, 15);
                                                                                                c.pceqw(5, 4, 0);
                                                                                                c.vu0().suba(0b1110, 3, Src::Vec(7));
                                                                                                c.vu0().msub(0b1110, 3, 7, Src::Bc(0, W));
                                                                                                c.pcgtw(4, 4, 0);
                                                                                                c.por(4, 4, 5);
                                                                                                c.vu0().suba(0b1110, 3, Src::Bc(0, W));
                                                                                                c.vu0().msub(0b1110, 5, 4, Src::Bc(0, W));
                                                                                                c.psllw(2, 2, 15);
                                                                                                c.psllw(2, 2, 15);
                                                                                                c.vu0().mul(0b1110, 5, 5, Src::Vec(3));
                                                                                                c.vu0().sub(0b1110, 9, 0, Src::Vec(5));
                                                                                                c.vu0().mula(0b1110, 9, Src::Vec(5));
                                                                                                c.vu0().madd(0b1110, 10, 4, Src::Bc(0, W));
                                                                                                c.qmfc2(5, 9);
                                                                                                c.pand(5, 5, 4);
                                                                                                c.qmfc2(3, 5);
                                                                                                c.pnor(4, 4, 0);
                                                                                                c.pand(4, 4, 3);
                                                                                                c.por(5, 5, 4);
                                                                                                frame.save(c, 5, 0x4c50);
                                                                                                c.sqc2(10, frame.at(c, 0x4c60));
                                                                                                frame.restore_fpr(c, 0, 0x4c60);
                                                                                                c.sqrt_s(0, 0);
                                                                                                frame.save_fpr(c, 0, 0x4c60);
                                                                                                frame.restore_fpr(c, 0, 0x4c64);
                                                                                                c.sqrt_s(0, 0);
                                                                                                frame.save_fpr(c, 0, 0x4c64);
                                                                                                frame.restore_fpr(c, 0, 0x4c68);
                                                                                                c.sqrt_s(0, 0);
                                                                                                frame.save_fpr(c, 0, 0x4c68);
                                                                                                c.lqc2(3, frame.at(c, 0x4c60));
                                                                                                c.pceqw(5, 2, 0);
                                                                                                c.pcgtw(2, 2, 0);
                                                                                                c.por(2, 2, 5);
                                                                                                c.vu0().sub(0b1110, 4, 0, Src::Vec(3));
                                                                                                c.qmfc2(5, 3);
                                                                                                c.pand(5, 5, 2);
                                                                                                c.pnor(2, 2, 0);
                                                                                                c.qmfc2(3, 4);
                                                                                                c.pand(2, 2, 3);
                                                                                                c.por(5, 5, 2);
                                                                                                frame.save(c, 5, 0x4c60);
                                                                                                frame.restore_fpr(c, 4, 0x4c50);
                                                                                                frame.restore_fpr(c, 0, 0x4c54);
                                                                                                frame.restore_fpr(c, 1, 0x4c68);
                                                                                                c.mul_s(0, 4, 0);
                                                                                                frame.restore_fpr(c, 5, 0x4c58);
                                                                                                frame.restore_fpr(c, 2, 0x4c64);
                                                                                                frame.restore_fpr(c, 3, 0x4c60);
                                                                                                c.mul_s(1, 0, 1);
                                                                                                c.mul_s(4, 4, 2);
                                                                                                c.swc1(3, (c.g[A2][0] as u32));
                                                                                                c.mul_s(0, 0, 5);
                                                                                                c.swc1(1, (c.g[A2][0] as u32).wrapping_add(0x4));
                                                                                                c.swc1(4, (c.g[A2][0] as u32).wrapping_add(0x8));
                                                                                                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
                                                                                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                                                                                if c.g[V0][0] == 0u64 {
    c.g[V0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x10));
    break 'b_30fe98;
    } else {
    'b_30fde0: {
    'b_30fdcc: {
    c.cycles += 4;
    c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x1b));
    c.g[V0][0] = (c.g[V0][0] < 0x4u64) as u64;
    let t = c.g[V0][0] == 0u64;
    c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x1b));
    if t {
    break 'b_30fdcc;
    } else {
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    let t = true;
    c.lwc1(3, (c.g[V0][0] as u32));
    if t {
    break 'b_30fde0;
    } else {
    break 'b_30fdcc;
    }
    }
    }
    c.cycles += 5;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.lwc1(0, (c.g[V0][0] as u32));
    c.neg_s(3, 0);
    break 'b_30fde0;
    }
    'b_30fe14: {
    'b_30fe00: {
    c.cycles += 4;
    c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x18));
    c.g[V0][0] = (c.g[V0][0] < 0x4u64) as u64;
    let t = c.g[V0][0] == 0u64;
    c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x18));
    if t {
    break 'b_30fe00;
    } else {
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    let t = true;
    c.lwc1(2, (c.g[V0][0] as u32));
    if t {
    break 'b_30fe14;
    } else {
    break 'b_30fe00;
    }
    }
    }
    c.cycles += 5;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.lwc1(0, (c.g[V0][0] as u32));
    c.neg_s(2, 0);
    break 'b_30fe14;
    }
    'b_30fe48: {
    'b_30fe34: {
    c.cycles += 4;
    c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x19));
    c.g[V0][0] = (c.g[V0][0] < 0x4u64) as u64;
    let t = c.g[V0][0] == 0u64;
    c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x19));
    if t {
    break 'b_30fe34;
    } else {
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    let t = true;
    c.lwc1(1, (c.g[V0][0] as u32));
    if t {
    break 'b_30fe48;
    } else {
    break 'b_30fe34;
    }
    }
    }
    c.cycles += 5;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.lwc1(0, (c.g[V0][0] as u32));
    c.neg_s(1, 0);
    break 'b_30fe48;
    }
    'b_30fe7c: {
    'b_30fe68: {
    c.cycles += 4;
    c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x1a));
    c.g[V0][0] = (c.g[V0][0] < 0x4u64) as u64;
    let t = c.g[V0][0] == 0u64;
    c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x1a));
    if t {
    break 'b_30fe68;
    } else {
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    let t = true;
    c.lwc1(0, (c.g[V0][0] as u32));
    if t {
    break 'b_30fe7c;
    } else {
    break 'b_30fe68;
    }
    }
    }
    c.cycles += 5;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.lwc1(0, (c.g[V0][0] as u32));
    c.neg_s(0, 0);
    break 'b_30fe7c;
    }
    c.cycles += 7;
    frame.save_fpr(c, 3, 0x4c4c);
    frame.save_fpr(c, 2, 0x4c40);
    frame.save_fpr(c, 1, 0x4c44);
    frame.save_fpr(c, 0, 0x4c48);
    frame.restore(c, 2, 0x4c40);
    c.sq(2, (c.g[A2][0] as u32));
    c.g[V0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x10));
    break 'b_30fe98;
    }
                                                                                            }
                                                                                            'b_30fedc: {
                                                                                                c.cycles += 2;
                                                                                                let t = c.g[V0][0] != c.g[S0][0];

                                                                                                if t
                                                                                                {
                                                                                                    break 'b_30fedc;
                                                                                                } else {
                                                                                                    c.cycles += 15;
                                                                                                    c.lqc2(4, (c.g[S3][0] as u32).wrapping_add(0x30));
                                                                                                    c.lqc2(5, (c.g[A2][0] as u32));
                                                                                                    c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
                                                                                                    c.vu0().opmula(4, 5);
                                                                                                    c.vu0().opmsub(6, 5, 4);
                                                                                                    c.vu0().mula(0b1110, 4, Src::Bc(5, W));
                                                                                                    c.vu0().madda(0b1110, 5, Src::Bc(4, W));
                                                                                                    c.vu0().suba(0b0001, 7, Src::Bc(7, X));
                                                                                                    c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
                                                                                                    c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
                                                                                                    c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
                                                                                                    c.sqc2(8, frame.at(c, 0x4c50));
                                                                                                    frame.restore(c, 2, 0x4c50);
                                                                                                    frame.save(c, 2, 0x4c40);
                                                                                                    c.sq(2, (c.g[A2][0] as u32));
                                                                                                    break 'b_30fedc;
                                                                                                }
                                                                                            }
                                                                                            'b_30ff70: {
                                                                                                'b_30fef4: {
                                                                                                    c.cycles += 4;
                                                                                                    c.c_eq_s(20, 6);
                                                                                                    if !c.fcc() {
    c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x28));
    break 'b_30fef4;
    } else {
    c.cycles += 2;
    let t = true;
    c.sw((c.g[T3][0] as u32), c.g[T7][0]);
    if t {
    break 'b_30ff70;
    } else {
    break 'b_30fef4;
    }
    }
                                                                                                }
                                                                                                c.cycles += 31;
                                                                                                c.g[V0][0] = sx((c.g[T0][0] as u32) << 4);
                                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                                                                                c.mfc1(4, 20);
                                                                                                c.lqc2(4, (c.g[V0][0] as u32));
                                                                                                c.qmtc2(4, 3);
                                                                                                c.lqc2(5, (c.g[A2][0] as u32));
                                                                                                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                                                                c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
                                                                                                c.vu0().adda(0b1000, 7, Src::Bc(7, Y));
                                                                                                c.vu0().madda(0b1000, 6, Src::Bc(7, Z));
                                                                                                c.vu0().madd(0b1000, 7, 6, Src::Bc(7, W));
                                                                                                c.qmfc2(4, 7);
                                                                                                c.g[V1][0] = 0xffffffff80000000;
                                                                                                c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                                                                                                c.pextlw(3, 3, 3);
                                                                                                c.pextlw(3, 3, 3);
                                                                                                c.qmfc2(4, 5);
                                                                                                c.pxor(4, 4, 3);
                                                                                                c.qmtc2(4, 5);
                                                                                                c.vu0().mula(0b1111, 4, Src::Bc(0, W));
                                                                                                c.vu0().msuba(0b1111, 4, Src::Bc(3, X));
                                                                                                c.vu0().madd(0b1111, 3, 5, Src::Bc(3, X));
                                                                                                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                                                                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                                                                                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                                                                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                                                                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                                                                c.vu0().rsqrt(0, 3, 4, 0);
                                                                                                c.vu0().waitq();
                                                                                                c.vu0().mul(0b1111, 5, 3, Src::Q);
                                                                                                c.sqc2(5, (c.g[V0][0] as u32));
                                                                                                break 'b_30ff70;
                                                                                            }
                                                                                            c.cycles += 1;
                                                                                            c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x44));
                                                                                            break 'b_30ff74;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                c.cycles += 7;
                                                                                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                                                                                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x4));
                                                                                c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x4));
                                                                                c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                c.g[V0][0] = ((c.g
                                                                                    [T0][0]
                                                                                    as i64)
                                                                                    < (c.g[V0][0]
                                                                                        as i64))
                                                                                    as u64;
                                                                                let t = c.g[V0][0]
                                                                                    != 0u64;
                                                                                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x50));
                                                                                if t {
                                                                                    if event_due(c)
                                                                                    {
                                                                                        deliver_event_here(c, 0x0030fa18);
                                                                                    }
                                                                                    continue 'l_30fa18;
                                                                                } else {
                                                                                    break 'b_30ff90;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 1;
                                                        c.g[V1][0] = c.lw(frame.at(c, 0x4c9c));
                                                        break 'b_30ff94;
                                                    }
                                                    c.cycles += 4;
                                                    c.g[S2][0] = c.g[T5][0];
                                                    c.g[V0][0] = ((c.g[S2][0] as i64)
                                                        < (c.g[V1][0] as i64))
                                                        as u64;
                                                    if c.g[V0][0] != 0u64 {
                                                        c.g[A1][0] = c.lw((c.g[S3][0] as u32));
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x0030f740);
                                                        }
                                                        continue 'l_30f740;
                                                    } else {
                                                        break 'b_30ffa4;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[A1][0] = c.lw(frame.at(c, 0x4ca4));
                                        break 'b_30ffa8;
                                    }
                                }
                                c.cycles += 4;
                                c.g[A0][0] = c.lw(frame.at(c, 0x4c80));
                                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[A0][0] = c.lw(frame.at(c, 0x4c98));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0030f6c0);
                                    }
                                    continue 'l_30f6c0;
                                } else {
                                    break 'b_30ffb8;
                                }
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[T9][0] = c.lw(frame.at(c, 0x4ca0));
                    c.g[A2][0] = c.lw(frame.at(c, 0x4c90));
                    c.g[V0][0] = ((c.g[T9][0] as i64) < (c.g[A2][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A2][0] = sx((c.g[T9][0] as u32).wrapping_add(0x1));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0030f6a0);
                        }
                        continue 'l_30f6a0;
                    } else {
                        break 'b_30ffcc;
                    }
                }
            }
        }
        'b_3100ec: {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[T1][0] = 0u64;
            if t {
                break 'b_3100ec;
            } else {
                c.cycles += 2;
                c.g[S1][0] = 0x58;
                c.mult(0, 3, 9, 17);
                'l_30ffe0: loop {
                    'b_310090: {
                        c.cycles += 8;
                        c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xc));
                        c.g[T0][0] = 0u64;
                        c.g[S0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                        c.g[A2][0] = c.g[A1][0];
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                        let t = (c.g[V0][0] as i64) <= 0;
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x38));
                        if t {
                            break 'b_310090;
                        } else {
                            c.cycles += 6;
                            c.g[V0][0] = 0x58;
                            c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30));
                            c.mult(0, 10, 9, 2);
                            c.g[T4][0] = sx(frame.at(c, 0x720));
                            c.g[T3][0] = sx(frame.at(c, 0x620));
                            'l_310018: loop {
                                'b_310050: {
                                    c.cycles += 8;
                                    c.g[V0][0] =
                                        sx((c.g[T2][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                                    c.g[A2][0] =
                                        sx((c.g[T0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.g[A1][0] = sx((c.g[A2][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[T4][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                    let t = c.g[V1][0] != 0u64;
                                    c.g[V0][0] =
                                        sx((c.g[T3][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                    if t {
                                        break 'b_310050;
                                    } else {
                                        c.cycles += 6;
                                        c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x24));
                                        c.g[V1][0] = sx((c.g[A2][0] as u32) << 4);
                                        c.lq(4, (c.g[A3][0] as u32).wrapping_sub(0x10));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.sq(4, (c.g[V1][0] as u32));
                                        c.g[V0][0] =
                                            sx((c.g[T3][0] as u32)
                                                .wrapping_add((c.g[A1][0] as u32)));
                                        break 'b_310050;
                                    }
                                }
                                'b_310074: {
                                    c.cycles += 3;
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                    if c.g[V1][0] != 0u64 {
                                        c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xc));
                                        break 'b_310074;
                                    } else {
                                        c.cycles += 6;
                                        c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x28));
                                        c.g[V1][0] = sx((c.g[A2][0] as u32) << 4);
                                        c.lq(4, (c.g[A3][0] as u32));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.sq(4, (c.g[V1][0] as u32));
                                        c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xc));
                                        break 'b_310074;
                                    }
                                }
                                c.cycles += 7;
                                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x50));
                                c.g[V1][0] =
                                    sx((c.g[T2][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                                c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[A2][0] = c.g[A1][0];
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00310018);
                                    }
                                    continue 'l_310018;
                                } else {
                                    break 'b_310090;
                                }
                            }
                        }
                    }
                    'b_3100dc: {
                        'b_3100d8: {
                            c.cycles += 5;
                            c.mult(0, 2, 9, 17);
                            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                            let t = (c.g[V0][0] as i64) < 0;
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            if t {
                                break 'b_3100d8;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(sp(c)));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x820));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                if c.g[V1][0] != 0u64 {
                                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
                                    break 'b_3100dc;
                                } else {
                                    c.cycles += 8;
                                    c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A1][0] = 0u64;
                                    c.g[A2][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4c));
                                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x3c));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                                    c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
                                    c.g[RA][0] = sx(0x003100d8);
                                    c.g[A0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, 0x003e6448, 0x003100d8)?;
                                    break 'b_3100d8;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
                        break 'b_3100dc;
                    }
                    c.cycles += 4;
                    c.g[T1][0] = c.g[S0][0];
                    c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.mult(0, 3, 9, 17);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0030ffe0);
                        }
                        continue 'l_30ffe0;
                    } else {
                        break 'b_3100ec;
                    }
                }
            }
        }
        c.cycles += 13;
        frame.restore(c, 16, 0x4de0);
        frame.restore(c, 17, 0x4dd0);
        frame.restore(c, 18, 0x4dc0);
        frame.restore(c, 19, 0x4db0);
        frame.restore(c, 20, 0x4da0);
        frame.restore(c, 21, 0x4d90);
        frame.restore(c, 22, 0x4d80);
        frame.restore(c, 23, 0x4d70);
        frame.restore(c, 30, 0x4d60);
        frame.restore_ra(c, 0x4d50);
        frame.restore_fpr(c, 20, 0x4df0);
        frame.close(c);
        return Ok(());
        Ok(())
    })
}


/// `void RiderModel_ShowPartVariant(RiderModel* self, int part, void* value)`
/// (0x30ec18): store `value` in the part's variant slot, OR the part's mask
/// bit and mark the three variant words.
pub fn rider_model_show_part_variant(c: &mut Cpu) {
    c.cycles += 6;
    c.g[T0][0] = c.g[A0][0];
    c.g[V1][0] = c.g[A1][0];
    c.g[V0][0] = Addr(p(r32(c, T0), 0x14)).lw(c);
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[A1][0] = Addr(p(r32(c, T0), 0x1c)).lw(c);
    c.g[V0][0] = sx(r32(c, V1) << 2);
    if c.g[A1][0] == 0 {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    c.g[V0][0] = sx(p(r32(c, V0), r32(c, A1)));
    c.g[A1][0] = Addr(r32(c, V0)).lw(c);
    c.g[V0][0] = 0x58;
    if (c.g[A1][0] as i64) < 0 {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 11;
    c.g[V1][0] = Addr(p(r32(c, T0), 0xc)).lw(c);
    c.mult(0, 2, 5, 2);
    c.g[A3][0] = 0x3;
    c.g[A1][0] = 0x1;
    c.g[V1][0] = sx(p(r32(c, V1), r32(c, V0)));
    Addr(p(r32(c, V1), 0x50)).sw(c, c.g[A2][0]);
    c.g[A0][0] = Addr(p(r32(c, V1), 0x10)).ld(c);
    c.g[A2][0] = sx(p(r32(c, V1), 0x30));
    c.g[V0][0] = Addr(p(r32(c, T0), 0x150)).ld(c);
    c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
    Addr(p(r32(c, T0), 0x150)).sd(c, c.g[V0][0]);
    loop {
        c.cycles += 7;
        Addr(r32(c, A2)).sw(c, c.g[A1][0]);
        c.g[A3][0] = sx(r32(c, A3).wrapping_sub(1));
        let more = (c.g[A3][0] as i64) >= 0;
        c.g[A2][0] = sx(r32(c, A2).wrapping_sub(8));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0030_ec78);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Rider model.

/// `void RiderModel_ShowPart(RiderModel* model, int part, int slot)` (0x30e9e0):
/// reveal one part. Parts past the count at `+0x14` are ignored; a part whose
/// table (`+0x1c`) entry is negative is ignored. Otherwise set the part's
/// visibility word (`+0x18`) on the part block (`+0xc + 88*part`), or, when
/// `slot` is negative, all four (`+0x30..+0x18`), and OR its 64-bit mask at
/// `+0x150`.
pub fn rider_model_show_part(c: &mut Cpu) {
    c.cycles += 6;
    c.g[A3][0] = c.g[A0][0];
    c.g[V1][0] = c.g[A1][0];
    c.g[V0][0] = lw(c, p(c.g[A3][0] as u32, 0x14));
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    if c.g[V0][0] == 0 {
        c.cycles += 3;
        c.g[A1][0] = lw(c, p(c.g[A3][0] as u32, 0x1c));
        let no_table = c.g[A1][0] == 0;
        c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
        if !no_table {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V1][0] = lw(c, c.g[V0][0] as u32);
            let negative = (c.g[V1][0] as i64) < 0;
            c.g[V0][0] = 0x58;
            if !negative {
                c.cycles += 8;
                c.g[A1][0] = lw(c, p(c.g[A3][0] as u32, 0xc));
                c.mult(0, 2, 3, 2);
                c.g[V1][0] = c.ld(p(c.g[A3][0] as u32, 0x150));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = c.ld(p(c.g[A1][0] as u32, 0x10));
                c.g[V1][0] |= c.g[A0][0];
                let all = (c.g[A2][0] as i64) < 0;
                c.sd(p(c.g[A3][0] as u32, 0x150), c.g[V1][0]);
                if !all {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[A2][0] as u32) << 3);
                    c.g[V1][0] = 1;
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    sw(c, p(c.g[V0][0] as u32, 0x18), c.g[V1][0]);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x30));
                    c.g[A0][0] = 1;
                    c.g[V1][0] = 3;
                    loop {
                        c.cycles += 7;
                        sw(c, c.g[V0][0] as u32, c.g[A0][0]);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                        let again = (c.g[V1][0] as i64) >= 0;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(8));
                        if again {
                            if event_due(c) {
                                deliver_event_here(c, 0x0030_ea40);
                            }
                            continue;
                        }
                        break;
                    }
                }
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Rider model.

/// `void RiderModel_HidePart(self, int part, int channel)` (0x30ea80): clear
/// a part's channel LOD and recompute the visibility mask.
pub fn rider_model_hide_part(c: &mut Cpu) {
    c.cycles += 6;
    c.g[A3][0] = c.g[A0][0];
    c.g[V1][0] = c.g[A1][0];
    c.g[V0][0] = Addr(p(r32(c, A3), 0x14)).lw(c);
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        let ra = r32(c, RA);
        c.pc = ra;
        return;
    }
    'outer: {
        c.cycles += 3;
        c.g[A1][0] = Addr(p(r32(c, A3), 0x1c)).lw(c);
        let have_index = c.g[A1][0] != 0;
        c.g[V0][0] = sx(r32(c, V1) << 2);
        if !have_index {
            break 'outer;
        }
        c.cycles += 4;
        c.g[V0][0] = sx(p(r32(c, V0), r32(c, A1)));
        c.g[V0][0] = Addr(r32(c, V0)).lw(c);
        let negative = (c.g[V0][0] as i64) < 0;
        c.g[V1][0] = 0x58;
        if negative {
            break 'outer;
        }
        c.cycles += 9;
        c.g[A1][0] = Addr(p(r32(c, A3), 0xc)).lw(c);
        c.mult(0, 3, 2, 3);
        c.g[A0][0] = c.ld(p(r32(c, A3), 0x150));
        c.g[A1][0] = sx(p(r32(c, A1), r32(c, V1)));
        c.g[V0][0] = c.ld(p(r32(c, A1), 0x10));
        c.g[V0][0] = !(0u64 | c.g[V0][0]);
        c.g[A0][0] = c.g[A0][0] & c.g[V0][0];
        let channel_ok = (c.g[A2][0] as i64) >= 0;
        c.sd(p(r32(c, A3), 0x150), c.g[A0][0]);
        if channel_ok {
            'found: {
                c.cycles += 8;
                c.g[V0][0] = sx(r32(c, A2) << 3);
                c.g[V1][0] = sx(p(r32(c, A1), 0x18));
                c.g[A2][0] = c.g[V1][0];
                c.g[V1][0] = sx(p(r32(c, V1), r32(c, V0)));
                Addr(r32(c, V1)).sw(c, 0);
                c.g[V0][0] = Addr(p(r32(c, A1), 0x18)).lw(c);
                c.g[A0][0] = 0;
                if c.g[V0][0] != 0 {
                    break 'found;
                }
                c.cycles += 1;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                loop {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 4i64) as u64;
                    let stop = c.g[V0][0] == 0;
                    c.g[V0][0] = sx(r32(c, A0) << 3);
                    if stop {
                        break 'outer;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = sx(p(r32(c, A2), r32(c, V0)));
                    c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                    let empty = c.g[V1][0] == 0;
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                    if !empty {
                        break 'found;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0030_eb28);
                    }
                }
            }
            c.cycles += 4;
            c.g[V1][0] = c.ld(p(r32(c, A1), 0x10));
            c.g[V0][0] = c.ld(p(r32(c, A3), 0x150));
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.sd(p(r32(c, A3), 0x150), c.g[V0][0]);
        } else {
            c.cycles += 2;
            c.g[V0][0] = sx(p(r32(c, A1), 0x30));
            c.g[V1][0] = 3;
            loop {
                c.cycles += 7;
                Addr(r32(c, V0)).sw(c, 0);
                c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
                let more = (c.g[V1][0] as i64) >= 0;
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(8));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_eae0);
                }
            }
        }
    }
    c.cycles += 2;
    let ra = r32(c, RA);
    c.pc = ra;
}

/// `AnimDataBase_AllocChanCache` (0x00310f18): build the animation channel
/// cache of bank `bank`: allocate 20 `sAnimNode_310F18` (0x24 bytes each),
/// link them into a circular list (`next` at +0x14, `prev` at +0x1c), give
/// each node a freshly allocated array of `n` 0x14-byte `sAnimElem_310F18`
/// (+0x4), and publish the list in `D_004FE7A0[bank]`/`D_004FE860[bank]`.
pub fn anim_data_base_alloc_chan_cache(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = 0x50_0000;
        frame.save(c, S1, 0x70);
        c.g[A2][0] = 0x2000_0000;
        c.g[S1][0] = c.g[A1][0]; // n
        frame.save(c, S0, 0x80);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = sx((c.g[A0][0] as u32) << 2); // bank * 4
        frame.save(c, S5, 0x30);
        c.g[A1][0] = 0x49_0000;
        frame.save(c, S2, 0x20);
        c.g[S7][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1860)); // 0x004fe7a0
        c.g[S5][0] = c.g[S0][0];
        c.g[S6][0] = c.g[A1][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.save(c, S4, 0x40);
        c.g[A0][0] = 0x2d0; // 20 * 0x24
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x62b0)); // 0x00489d50
        c.g[A3][0] = 0u64;
        c.g[S2][0] = c.g[S0][0];
        call_ptr(c, 0x0031_7e30, 0x0031_0f7c)?;
        c.g[S3][0] = 0x13;
        c.g[V1][0] = 0x14;
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        c.mult(0, S4, S1, V1); // n * 0x14
        c.g[S1][0] = 0u64;
        loop {
            c.cycles += 22;
            c.g[T0][0] = c.lw(c.g[S2][0] as u32);
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x62b0));
            c.g[A2][0] = 0x2000_0000;
            c.g[T0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[T0][0] as u32));
            c.g[A3][0] = 0u64;
            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x24));
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(1));
            c.sw((c.g[T0][0] as u32).wrapping_add(0x20), c.g[V0][0]); // node[i].next
            c.g[V1][0] = c.lw(c.g[S2][0] as u32);
            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x24));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[T0][0]); // node[i].prev
            c.g[V0][0] = c.lw(c.g[S2][0] as u32);
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw(c.g[V0][0] as u32, 0u64); // node[i].used = 0
            c.g[S0][0] = c.lw(c.g[S2][0] as u32);
            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
            call_ptr(c, 0x0031_7e30, 0x0031_0fdc)?;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x24));
            let more = (c.g[S3][0] as i64) >= 0;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]); // node[i].data
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0031_0f90);
            }
        }
        c.cycles += 23;
        c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_sub(0x1860));
        c.g[A2][0] = 0x50_0000;
        c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x17a0));
        c.g[A0][0] = c.lw(c.g[A1][0] as u32);
        c.g[A2][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A2][0] as u32));
        frame.restore(c, S0, 0x80);
        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2ac)); // node[19]
        frame.restore(c, S1, 0x70);
        c.sw((c.g[A0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
        frame.restore(c, S2, 0x60);
        c.g[V0][0] = c.lw(c.g[A1][0] as u32);
        frame.restore(c, S3, 0x50);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x2cc), c.g[V0][0]);
        frame.restore(c, S4, 0x40);
        c.g[V0][0] = c.lw(c.g[A1][0] as u32);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        c.sw(c.g[A2][0] as u32, c.g[V0][0]);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;
    use crate::difftest::put_words;
    use ssx3_ee as ee;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::put_vec;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::SP as STACK_TOP;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::elf_image;
    use crate::difftest::SP;
    pub(crate) use crate::game::world::wscriptcache::tests::SEEDS;
    pub(crate) use crate::game::ai::aianimeventmap::tests::MEM_MUTEX;
    pub(crate) use crate::game::ai::aianimeventmap::tests::alloc_setup;
    pub(crate) use crate::game::visualfx::worldlightman::tests::f;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::SEEDS_3 as SEEDS_2;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::CTX;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::TOP;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::TEX;
    pub(crate) use crate::game::ai::rider::tests::MODEL_2 as MODEL;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::uf;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::ctx_at;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::put_ctx;
    pub(crate) use crate::game::world::worldview::tests::SEEDS_5 as SEEDS_3;
    pub(crate) use crate::game::world::worldview::tests::word;
    pub(crate) use crate::game::fe::feasyncfile::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::clear_4 as clear;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2 as w;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_5 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_4 as world;
    pub(crate) use crate::game::wscript::wscriptman::tests::w as w_3;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::BE_3 as BE;
    pub(crate) use crate::game::wscript::wscriptman::tests::WORLD_3 as WORLD;
    pub(crate) use crate::game::wscript::wscriptman::tests::GAME;
    pub(crate) use crate::game::wscript::wscriptman::tests::globals_3 as globals;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_2;
    pub(crate) use crate::game::sound::soundsys::tests::BE as BE_2;
    pub(crate) use crate::game::sound::soundsys::tests::WORLD as WORLD_2;
    pub(crate) use crate::game::sound::soundsys::tests::w as w_4;
    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear_3;
    pub(crate) use crate::game::sound::soundsys::tests::globals_7 as globals_2;
    pub(crate) use crate::game::sound::soundsys::tests::pool_7 as pool;
    pub(crate) use crate::game::sound::soundsys::tests::load_2 as load;
    pub(crate) use crate::game::sound::soundsys::tests::alloc_ready;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_3;


    #[test]
    fn rider_model_set_color_scale_matches() {
        check(ssx3_ee::s10::f_0030dba0, rider_model_set_color_scale, 200, &|c, seed| {
            let mut r = Rng(0x200a ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.f[12] = r.edge_f32();
            c.f[13] = r.edge_f32();
            c.bus.write32(DATA + 0x140, r.next_u32());
        });
    }


    #[test]
    fn rider_model_is_part_visible_matches() {
        check_lohi(ssx3_ee::s10::f_0030ebc0, rider_model_is_part_visible, 300, &|c, seed| {
            let mut r = Rng(0x200b ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let idx = seed % 4;
            c.g[A1][0] = idx as u64;
            c.g[A2][0] = (seed % 8) as u64;
            c.bus.write32(DATA + 0x14, if seed % 3 == 0 { idx } else { 0x10 });
            let arr = if seed % 5 == 0 { 0 } else { DATA + 0x200 };
            c.bus.write32(DATA + 0x1c, arr);
            c.bus.write32(DATA + 0xc, DATA + 0x300);
            let entry = match seed % 6 {
                0 => 0u32,
                1 => 0x8000_0000,
                _ => (seed % 64) as u32,
            };
            c.bus.write32(arr.wrapping_add(idx * 4), entry);
            for i in 0..8 {
                c.bus.write32(DATA + 0x300 + i * 4, r.next_u32());
            }
        });
    }


    #[test]
    fn adf_data_source_set_data_matches() {
        check(ssx3_ee::s10::f_00311290, adf_data_source_set_data, 200, &|c, seed| {
            let mut r = Rng(0x200c ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            for i in 0..4 {
                c.bus.write32(DATA + 0x100 + i * 4, r.next_u32());
            }
        });
    }


    #[test]
    fn anim_data_base_ctor_matches() {
        check(ee::s10::f_00311048, anim_data_base_ctor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x1048, 0);
            c.g[A0][0] = DATA as u64;
            crate::difftest::fill(c, &mut r, DATA, 0x3fc / 4);
        });
    }


    #[test]
    fn anim_data_base_dtor_matches() {
        check(ee::s10::f_00311088, anim_data_base_dtor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x1088, 0);
            alloc_setup(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 2) as u64;
        });
    }


    // -------------------------------- ADF data source / animation combo

    #[test]
    fn adf_data_source_dtor_matches() {
        check(ee::s10::f_003112c8, adf_data_source_dtor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x12c8, 0);
            alloc_setup(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 2) as u64;
            c.bus.write32(DATA, if seed % 3 == 0 { 0 } else { DATA + 0x100 });
        });
    }


    #[test]
    fn anim_data_base_reset_chan_cache_table_matches() {
        check(ee::s10::f_003110d0, anim_data_base_reset_chan_cache_table, SEEDS, &|c, seed| {
            let mut r = Rng(0x3110d0 ^ seed as u64);
            scramble(c, &mut r);
            c.bus.write32(0x4fe7a0, r.next_u32());
            c.bus.write32(0x4fe860, r.next_u32());
        });
    }


    // ------------------------------------------------------------ AnimDataBase

    /// The jump tables and the epsilon `AnimDataBase_EvalChanCurves` reads.
    pub(crate) fn eval_constants(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        put_words(c, eval::LERP_TABLE_AT, &eval::LERP_TABLE);
        put_words(c, eval::KEY_TABLE_AT, &eval::KEY_TABLE);
        c.bus.write32(GAME_GP - 0x3228, f(0.01));
    }


    /// Curve records at `recs` (kinds 0..10, 10 = none) and keys at `keys`.
    pub(crate) fn curves(c: &mut Cpu, r: &mut Rng, recs: u32, keys: u32, n: u32) {
        fill(c, r, keys, 0x100);
        for k in 0..n {
            let rec = recs + k * 0x14;
            let kind = r.next_u32() % 11;
            c.bus.write32(rec, kind);
            for i in 1..5 {
                c.bus.write32(rec + 4 * i, r.f32(4.0));
            }
            if kind >= 6 {
                c.bus.write32(rec + 0xc, r.next_u32() % 8);
            }
            if kind == 5 && r.next_u32() % 2 == 0 {
                c.bus.write32(rec + 0x10, f((r.next_u32() % 16) as f32));
            }
        }
    }


    /// A decoded channel at `ch`: records at `recs`, key tables in `keys`.
    pub(crate) fn channel(c: &mut Cpu, r: &mut Rng, ch: u32, recs: u32, keys: u32) {
        c.bus.write32(ch, recs);
        c.bus.write32(ch + 0x4, keys);
        c.bus.write32(ch + 0x8, keys + 0x100);
        c.bus.write32(ch + 0xc, keys + 0x200);
        c.bus.write32(ch + 0x10, keys + 0x300);
        for i in 0..4 {
            c.bus.write8(ch + 0x14 + i, (r.next_u32() % 4) as u8);
        }
        fill(c, r, keys, 0x100);
    }


    /// A clip, chunk descriptors, compressed chunk data and two cache banks
    /// (four entries each in a random LRU order) for
    /// `AnimDataBase_SampleChannel`. Case (seed % 5): 0 the cached entry is
    /// the head, 1 the tail, 2 in the middle; 3 a miss evicting the head's
    /// owner, 4 a miss on an unowned head.
    pub(crate) fn sample_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x5a3b_1e00 ^ (seed as u64) << 12);
        scramble(c, &mut r);
        eval_constants(c);
        let (db, anim, clips, table, data) = (DATA, DATA + 0x80, DATA + 0x100, DATA + 0x200, DATA + 0x2000);
        let (recs, keys, out) = (DATA + 0x6000, DATA + 0x7000, DATA + 0x9000);
        c.bus.write32(db + 0x8, clips);
        c.bus.write32(db + 0xc, table);
        c.bus.write32(db + 0x14, data);
        // clip 1: 20 frames in chunks of 8
        let clip = clips + 0x14;
        c.bus.write32(clip + 4, 1);
        c.bus.write16(clip + 0xa, 3);
        c.bus.write16(clip + 0xc, 20);
        c.bus.write16(clip + 0xe, 8);
        c.bus.write32(anim, 1 << 8 | (r.next_u32() & 0xff));
        let frame = (r.next_u32() % 2600) as f32 / 100.0;
        c.bus.write32(anim + 4, f(frame / 30.0));
        curves(c, &mut r, recs, keys + 0x400, 12);
        let case = seed % 5;
        let bank = r.next_u32() % 2;
        // the entries' order in each bank's LRU list
        let rot = r.next_u32() % 4;
        let order: [u32; 4] = if r.next_u32() % 2 == 0 {
            [rot, (rot + 1) % 4, (rot + 2) % 4, (rot + 3) % 4]
        } else {
            [rot, (rot + 3) % 4, (rot + 2) % 4, (rot + 1) % 4]
        };
        let old_owner = DATA + 0x300;
        c.bus.write32(old_owner + 4, r.next_u32());
        for b in 0..2 {
            let pool = DATA + 0x4000 + b * 0x200;
            c.bus.write32(adb::POOL + 4 * b, pool);
            c.bus.write32(adb::HEAD + 4 * b, pool + order[0] * 0x24);
            for i in 0..4 {
                let e = pool + order[i] * 0x24;
                c.bus.write32(e + 0x1c, pool + order[(i + 3) % 4] * 0x24);
                c.bus.write32(e + 0x20, pool + order[(i + 1) % 4] * 0x24);
                c.bus.write32(e, if case == 3 { old_owner } else { 0 });
                channel(c, &mut r, e + 4, recs, keys);
            }
        }
        let slot = match case {
            0 => order[0],
            1 => order[3],
            2 => order[1 + r.next_u32() as usize % 2],
            _ => 0xff,
        };
        for k in 0..8 {
            let d = table + k * 8;
            c.bus.write32(d, k * 0x40);
            c.bus.write8(d + 4, bank as u8);
            c.bus.write8(d + 5, slot as u8);
            c.bus.write16(d + 6, (r.next_u32() % 4) as u16);
            // compressed data: curve count, then kind nibbles (all 0)
            let src = data + k * 0x40;
            c.bus.write8(src, (1 + r.next_u32() % 3) as u8);
            c.bus.write8(src + 1, 0);
            c.bus.write8(src + 2, 0);
        }
        c.g[A0][0] = sx(db);
        c.g[A1][0] = sx(out);
        c.g[A2][0] = sx(anim);
        c.g[A3][0] = (r.next_u32() % 2) as u64;
        c.g[T0][0] = (r.next_u32() % 2) as u64;
        c.g[T1][0] = (1 + r.next_u32() % 6) as u64;
    }


    #[test]
    fn anim_data_base_sample_channel_matches() {
        check(ee::s10::f_00311318__generated, anim_data_base_sample_channel, 300, &sample_setup);
    }


    /// A `RiderModel` for the two model renderers: bone tables, optional parts
    /// with their LOD references, and the LOD info words.
    pub(crate) fn put_render_model(c: &mut Cpu, r: &mut Rng, base: u32, seed: u32, shadow: bool) {
        let bones = if seed % 3 == 0 { 0 } else { 2 };
        let parts = if seed % 4 == 0 { 0 } else { 2 };
        c.bus.write32(base + model::NUM_BONES, bones);
        c.bus.write32(base + model::NUM_PARTS, parts);
        c.bus.write32(base + model::HAS_PARTS, (seed % 5 != 0) as u32);
        c.bus.write32(base + model::PARTS, base + 0x8000);
        c.bus.write32(base + model::SCALE_Y, uf(r, 0.5, 2.0));
        for lod in 0..2u32 {
            c.bus.write32(base + model::LOD_INFO + 8 * lod, r.next_u32());
            c.bus.write32(base + model::LOD_INFO + 8 * lod + 4, r.next_u32());
        }
        let a = base + 0x1000;
        let b = base + 0x2000;
        c.bus.write32(
            base + if shadow { model::BONE_TABLE_A_SHADOW } else { model::BONE_TABLE_A },
            a,
        );
        c.bus.write32(base + model::BONE_TABLE_B, b);
        for i in 0..2u32 {
            for k in 0..16 {
                c.bus.write32(a + 0x40 * i + 4 * k, r.next_u32());
                c.bus.write32(b + 0x40 * i + 4 * k, r.next_u32());
            }
        }
        let lod = seed % 2;
        for i in 0..parts {
            let part = base + 0x8000 + model::PART_STRIDE * i;
            let used = ((seed + i) % 3 != 0) as u32;
            let refs = base + 0x9000 + 0x100 * i;
            c.bus.write32(part + model::PART_LODS + 8 * lod, used);
            c.bus.write32(part + model::PART_LODS + 8 * lod + 4, refs);
            c.bus.write32(part + model::PART_REF_INDEX, (seed + i) % 2);
            c.bus.write32(part, (seed + i) % 4); // part id
            for k in 0..2u32 {
                let rec = refs + 16 * k;
                c.bus.write32(rec, if (seed + k) % 3 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(rec + 4, r.next_u32());
                c.bus.write32(rec + 8, r.next_u32());
                c.bus.write32(rec + 0xc, r.next_u32());
            }
        }
    }


    #[test]
    fn rider_model_render_matches() {
        check(ee::s10::f_00310640__generated, rider_model_render, SEEDS_2, &|c, seed| {
            let mut r = machine(c, seed, 0x0640, 3);
            put_ctx(c, &mut r, &[0x378, 0x318]);
            c.bus.write32(GAME_GP + 0x2858, (seed % 6 == 0) as u32);
            let m = MODEL;
            put_render_model(c, &mut r, m, seed, false);
            c.g[A0][0] = sx(m);
            c.g[A1][0] = sx(seed % 2);
            c.g[A2][0] = sx(1 + seed % 3);
            c.g[A3][0] = sx(4 + seed % 5);
        });
    }


    #[test]
    fn rider_model_render_shadow_matches() {
        check(ee::s10::f_00310948__generated, rider_model_render_shadow, SEEDS_2, &|c, seed| {
            let mut r = machine(c, seed, 0x0948, 3);
            put_ctx(c, &mut r, &[0x378, 0x320]);
            c.bus.write32(GAME_GP + 0x285c, (seed % 6 == 0) as u32);
            let m = MODEL;
            put_render_model(c, &mut r, m, seed, true);
            c.g[A0][0] = sx(m);
            c.g[A1][0] = sx(seed % 2);
            c.g[A2][0] = sx(1 + seed % 3);
            c.g[A3][0] = sx(4 + seed % 5);
        });
    }


    #[test]
    fn rider_model_lock_part_matches() {
        check(ee::s10::f_0030eca0__generated, rider_model_lock_part, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x30ec_a0, 0);
            let model = DATA + 0x100;
            let parts = DATA + 0x1000;
            let indices = DATA + 0x2000;
            c.g[A0][0] = sx(model);
            c.g[A1][0] = sx(seed % 8);
            c.bus.write32(model + 0x1c, parts);
            c.bus.write32(model + 0xc, DATA + 0x3000);
            c.bus.write64(model + 0x158, r.next_u32() as u64);
            for i in 0..8u32 {
                c.bus.write32(indices + 4 * i, r.next_u32() % 6);
            }
            fill(c, &mut r, parts, 6 * 0x58 / 4);
        });
    }


    // -----------------------------------------------------------------------
    // RiderModel_LoadState.

    #[test]
    fn rider_model_load_state_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x310e_e0, 1);
            let this = DATA;
            let stream = DATA + 0x800;
            let vt = DATA + 0x900;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = stream as u64;
            c.bus.write32(stream, vt);
            c.bus.write16(vt + 0x10, (r.next_u32() & 7) as u16);
            c.bus.write32(vt + 0x14, FAKE);
            c.bus.write32(this + 0x10, r.next_u32() % 8);
            c.bus.write32(this + 0x2c, ptr(&mut r));
        };
        check(ssx3_ee::s10::f_00310ee0__generated, rider_model_load_state, 64, &setup);
        crate::difftest::fakes::covered(ssx3_ee::s10::f_00310ee0__generated, rider_model_load_state, 64, &setup);
    }


    #[test]
    fn rider_model_compare_int_matches() {
        check(ssx3_ee::s10::f_0030db90__generated, rider_model_compare_int, SEEDS_3, &|c, seed| {
            let mut r = machine(c, seed, 0x30db90, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 4);
            word(c, DATA, r.next_u32());
            word(c, DATA + 4, r.next_u32());
        });
    }


    pub(crate) const SEEDS_4: u32 = 60;


    #[test]
    fn rider_model_hide_all_parts_matches() {
        check_lohi(ee::s10::f_0030eb60__generated, rider_model_hide_all_parts, SEEDS_4, &|c, seed| {
            let _ = base(c, seed, 0x30eb_60);
            let s = DATA + 0x100;
            clear(c, s, 0x200);
            let np = (seed % 4) as u32;
            let parts = DATA + 0x400;
            clear(c, parts, 0x200);
            w(c, s + 8, np);
            w(c, s + 0xc, parts as u32);
            for j in 0..np {
                for k in 0..4 {
                    w(c, parts + 0x58 * j + 0x18 + 8 * k, 1);
                }
            }
            c.g[A0][0] = sx(s);
            c.g[A3][0] = if seed % 2 == 0 { 0 } else { 0x40 };
        });
    }


    // Fixture objects, all inside RAM below the test stack.
    pub(crate) const MODEL_2: u32 = DATA;
 // the rider model
    pub(crate) const STATES: u32 = DATA + 0x2000;
 // `sAnimState` per bone, 0x50 apart
    pub(crate) const BONES: u32 = DATA + 0x4000;
 // the model's bone list, 0x50 apart
    pub(crate) const CHANS: u32 = DATA + 0x5000;
 // global animation-channel table
    pub(crate) const WORDS: u32 = DATA + 0x6000;
 // model+0x1c
    pub(crate) const FLOATS: u32 = DATA + 0x6400;
 // model+0x20
    pub(crate) const VEC_A: u32 = DATA + 0x6800;
 // model+0x24
    pub(crate) const VEC_B: u32 = DATA + 0x6c00;
 // model+0x28
    pub(crate) const MATS: u32 = DATA + 0x7000;
 // model+0x3c
    pub(crate) const AUX: u32 = DATA + 0x7800;
 // *(bone+0x38)
    pub(crate) const IDX: u32 = DATA + 0x7c00;
 // *(bone+0x40)

    #[test]
    fn rider_model_blend_anim_pose_matches() {
        check(
            ssx3_ee::s10::f_0030f2b0__generated,
            rider_model_blend_anim_pose,
            250,
            &|c, seed| {
                let mut r = Rng(0x30f2_b0a5_1c3d ^ seed as u64);
                scramble(c, &mut r);

                c.g[GP][0] = GAME_GP as u64;
                // No animation channels are registered for this frame, so the
                // sampler's channel lookups take their short paths.
                c.bus.write32(GAME_GP + 0xd08, CHANS);
                for i in 0..16 {
                    c.bus.write32(CHANS + 4 * i, 0);
                }

                let bones = 1 + seed % 2;
                c.g[A0][0] = MODEL_2 as u64;
                c.g[A1][0] = bones as u64;
                c.g[A2][0] = STATES as u64;
                c.g[A3][0] = (seed % 2) as u64;

                // The model: bone list, per-bone arrays and the two blend masks.
                c.bus.write32(MODEL_2 + 0x8, bones);
                c.bus.write32(MODEL_2 + 0xc, BONES);
                c.bus.write32(MODEL_2 + 0x10, r.next_u32() % 32);
                c.bus.write32(MODEL_2 + 0x14, r.next_u32() % 4);
                c.bus.write32(MODEL_2 + 0x1c, WORDS);
                c.bus.write32(MODEL_2 + 0x20, FLOATS);
                c.bus.write32(MODEL_2 + 0x24, VEC_A);
                c.bus.write32(MODEL_2 + 0x28, VEC_B);
                c.bus.write32(MODEL_2 + 0x3c, MATS);
                let bits = if seed % 2 == 0 { 0u64 } else { u64::MAX };
                c.bus.write64(MODEL_2 + 0x150, bits);
                c.bus
                    .write64(MODEL_2 + 0x158, if seed % 3 == 0 { 0 } else { bits });
                for i in 0..16 {
                    c.bus.write32(WORDS + 4 * i, r.next_u32() % 8);
                    c.bus.write32(FLOATS + 4 * i, r.edge_f32());
                    put_vec(
                        c,
                        VEC_A + 16 * i,
                        [r.edge_f32(), r.edge_f32(), r.edge_f32(), 0x3f80_0000],
                    );
                    put_vec(
                        c,
                        VEC_B + 16 * i,
                        [r.edge_f32(), r.edge_f32(), r.edge_f32(), 0x3f80_0000],
                    );
                    put_vec(
                        c,
                        MATS + 16 * i,
                        [r.edge_f32(), r.edge_f32(), r.edge_f32(), 0x3f80_0000],
                    );
                }

                for b in 0..bones {
                    let bone = BONES + 0x50 * b;
                    c.bus.write32(bone + 0x4, r.next_u32() % 3);
                    c.bus.write32(bone + 0x8, r.next_u32() % 32);
                    c.bus.write32(bone + 0xc, r.next_u32() % 8);
                    c.bus.write32(bone + 0x10, r.next_u32() % 4);
                    c.bus.write32(bone + 0x38, AUX + 0x100 * b);
                    c.bus.write32(bone + 0x40, IDX + 0x40 * b);
                    // No channel samples for this bone: the sample path reaches
                    // `AnimDataBase_SampleChannel`, which needs a populated
                    // animation-cache/heap fixture (known runaway PC 0x31ef40).
                    c.bus.write32(bone + 0x44, 0);
                    c.bus.write32(bone + 0x4c, r.next_u32() % 3);
                    for k in 0..4 {
                        c.bus.write32(AUX + 0x100 * b + 4 * k, r.next_u32() % 4);
                        c.bus.write32(IDX + 0x40 * b + 4 * k, r.next_u32() % 8);
                    }

                    // The animation state this bone blends from.
                    let st = STATES + 0x50 * b;
                    for k in 0..20 {
                        c.bus.write32(st + 4 * k, r.next_u32() % 8);
                    }
                    // channel index (low byte) and byte offset (high 24 bits)
                    c.bus
                        .write32(st, (r.next_u32() & 0xff) | ((r.next_u32() % 0x40) << 8));
                    c.bus.write32(st + 0x8, b);
                    c.bus.write64(
                        st + 0x10,
                        if seed % 2 == 0 {
                            0
                        } else {
                            1u64 << (r.next_u32() % 64)
                        },
                    );
                    c.bus.write16(st + 0x14, (r.next_u32() % 4) as u16);
                    c.bus.write16(st + 0x16, (r.next_u32() % 4) as u16);
                    for k in 0x18..0x1c {
                        c.bus.write8(st + k, (r.next_u32() % 6) as u8);
                    }
                    for k in 0..4 {
                        c.bus.write32(st + 0x40 + 4 * k, r.edge_f32());
                    }
                }
            },
        );
    }


    #[test]
    fn rider_model_show_part_variant_matches() {
        check(ssx3_ee::s10::f_0030ec18__generated, rider_model_show_part_variant, SEEDS_5, &|c, seed| {
            let mut r = machine(c, seed, 0x30ec18, 0);
            be_library(c);
            world(c);
            let model = DATA + 0x100;
            c.g[A0][0] = model as u64;
            let part = (r.next_u32() % 4) as u32;
            c.g[A1][0] = part as u64;
            c.g[A2][0] = r.next_u32() as u64;
            w_2(c, model + 0x14, (r.next_u32() % 8) as u32);
            if seed % 5 == 0 {
                w_2(c, model + 0x1c, 0);
            } else {
                let arr = DATA + 0x200;
                w_2(c, model + 0x1c, arr);
                for i in 0..4 {
                    // A negative id is a "no variant" slot.
                    let id = if seed % 3 == 0 && i == 1 {
                        0xffff_ffffu32
                    } else {
                        i
                    };
                    w_2(c, arr + 4 * i, id);
                }
                w_2(c, model + 0xc, DATA + 0x300);
                c.bus.write64(model + 0x150, r.next_u32() as u64);
            }
        });
    }


    // ----------------------------------------------------------- rider model

    #[test]
    fn rider_model_show_part_matches() {
        check(ssx3_ee::s10::f_0030e9e0__generated, rider_model_show_part, 24, &|c, seed| {
            let mut r = base_2(c, seed, 0x0030_e9e0);
            let model = DATA + 0x100;
            let table = DATA + 0x2000;
            let parts = DATA + 0x3000;
            clear_2(c, model, 0x160);
            clear_2(c, table, 0x80);
            clear_2(c, parts, 0x400);
            let part = (seed % 8) as u32;
            w_3(c, model + 0x14, if seed % 6 == 0 { part } else { 8 });
            w_3(c, model + 0x1c, table);
            w_3(c, model + 0xc, parts);
            for i in 0..8 {
                w_3(c, table + 4 * i, if (seed as u32 + i) % 4 == 0 { 0xffff_ffff } else { i });
            }
            w_3(c, model + 0x150, r.next_u32());
            w_3(c, model + 0x154, r.next_u32());
            c.g[A0][0] = sx(model);
            c.g[A1][0] = part as u64;
            c.g[A2][0] = if seed % 3 == 0 { 0xffff_ffff_ffff_ffff } else { (seed % 4) as u64 };
        });
    }


    #[test]
    fn rider_model_hide_part_matches() {
        check_lohi(
            ee::s10::f_0030ea80__generated,
            rider_model_hide_part,
            12,
            &|c, seed| {
                let _ = base_3(c, seed, 0x30ea80, 0);
                let this = DATA;
                clear_3(c, this, 0x800);
                c.g[A0][0] = this as u64;
                let part = (seed % 4) as u64;
                c.g[A1][0] = part;
                c.g[A2][0] = if seed % 3 == 0 {
                    0xffff_ffff_ffff_ffff
                } else {
                    (seed % 4) as u64
                };
                w_4(c, this + 0x14, if seed % 5 == 0 { 0 } else { 3 });
                w_4(c, this + 0x1c, DATA + 0x200);
                w_4(
                    c,
                    DATA + 0x200 + 4 * (part as u32),
                    if seed % 6 == 0 { 0xffff_ffff } else { 0 },
                );
                w_4(c, this + 0xc, DATA + 0x400);
                let e = DATA + 0x400 + 0x58;
                w_4(
                    c,
                    e + 0x10,
                    if seed % 2 == 0 {
                        0
                    } else {
                        0x0000_0000_0000_00ff
                    },
                );
                w_4(c, e + 0x18, 0);
                w_4(c, e + 0x1c, 0);
            },
        );
    }
}
